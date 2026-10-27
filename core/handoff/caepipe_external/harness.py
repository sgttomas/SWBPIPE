"""Live CAEPIPE round-trip harness for the correlation pilot (DEL-17-05, Design §3-§5).

The harness is disabled unless the user configures it: `SWBPIPE_CAEPIPE_EXCHANGE`
names a private exchange folder outside every Git work tree, holding a
`.gitignore` of `*` and a `harness_config.json` with the virtual machine name,
the guest paths of the user's own CAEPIPE executable, of the shared exchange
folder and of a guest-local work folder, and
`license_and_environment_responsibility_acknowledged: true`. Without that, every
live step skips with a stated reason (PH-004).

The harness never searches for, bundles or installs CAEPIPE, never stops a
process on the host other than the `prlctl` client it started itself (on its
own timeout), and runs one case at a time. Everything it writes under
`caepipe_out/` is CAEPIPE output or derived from it and stays private (§8).
Its records are regression and handoff evidence only.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import stat
import subprocess
from typing import Any, Callable, Mapping


ENV_VAR = "SWBPIPE_CAEPIPE_EXCHANGE"
CONFIG_NAME = "harness_config.json"
ACKNOWLEDGEMENT_KEY = "license_and_environment_responsibility_acknowledged"
REQUIRED_KEYS = ("vm_name", "guest_executable", "guest_exchange_root", "guest_work_root")
CASE_ID_PATTERN = re.compile(r"^[a-z0-9][a-z0-9-]{0,62}\Z")
GUEST_SCRIPT = Path(__file__).resolve().parent / "guest" / "Invoke-CaepipeRoundTrip.ps1"
GUEST_SCRIPT_REPOSITORY_PATH = "core/handoff/caepipe_external/guest/Invoke-CaepipeRoundTrip.ps1"
HARNESS_DIR = "_harness"
RUN_RECORD_KIND = "openpipestress.caepipe_harness_run_record"
BATCH_COMMAND_SHAPE = '<exe> "<mbf>";batchmode=yes'
BOUNDARY_NOTE = "regression/handoff evidence only; not validation, compatibility proof or professional acceptance"
CASE_LAYERS = ("swbpipe", "caepipe_in", "caepipe_out", "parsed", "comparison")
OUTPUT_NAMES = ("model.res", "model.mod")
PRLCTL_ATTEMPTS = 3
CSV_ROUTES = ("csv_from_batch", "csv_from_gui_print")
# Guest paths enter `cmd /c` command strings, so they are held to a strict allow-list: an absolute Windows path
# (drive letter, colon, backslash) of letters, digits, space and \ . - _ ( ) only. Quotes, %, ^, &, |, <, >, !
# and control characters can never reach the guest shell.
GUEST_PATH_KEYS = ("guest_executable", "guest_exchange_root", "guest_work_root")
GUEST_PATH_PATTERN = re.compile(r"^[A-Za-z]:\\[A-Za-z0-9 \\._()-]*\Z")


class ContainmentError(RuntimeError):
    """A path the pilot would write or delete is a symbolic link or lies outside the exchange folder."""


@dataclass(frozen=True)
class HarnessSkip:
    """The harness is not configured; `reason` says why, `checked` what was checked before stopping."""

    reason: str
    checked: tuple[str, ...] = ()


@dataclass(frozen=True)
class HarnessConfig:
    exchange_root: Path
    vm_name: str
    guest_executable: str
    guest_exchange_root: str
    guest_work_root: str
    prlctl: str | None
    acknowledged: bool
    trial_first_launch_date: str | None
    checked: tuple[str, ...] = ()
    warnings: tuple[str, ...] = field(default_factory=tuple)


def resolve_harness_config(environ: Mapping[str, str] = os.environ, *,
                           require_vm_tools: bool = True) -> HarnessConfig | HarnessSkip:
    """Resolve the private harness configuration, or say exactly why the harness is skipped.

    `require_vm_tools=False` (offline re-comparison of stored outputs) omits the
    `prlctl` check; every data-boundary check still applies.
    """

    checked: list[str] = []

    def skip(reason: str) -> HarnessSkip:
        return HarnessSkip(reason=reason, checked=tuple(checked))

    raw = environ.get(ENV_VAR, "")
    checked.append(f"{ENV_VAR} set")
    if not raw.strip():
        return skip(f"{ENV_VAR} is unset or empty; the CAEPIPE harness is disabled")
    root = Path(raw).expanduser()
    checked.append("exchange folder exists")
    if not root.exists():
        return skip(f"{ENV_VAR} names a folder that does not exist")
    checked.append("exchange folder is a directory")
    if not root.is_dir():
        return skip(f"{ENV_VAR} does not name a directory")
    root = root.resolve()
    checked.append(".gitignore of '*'")
    ignore = root / ".gitignore"
    try:
        content = ignore.read_text(encoding="utf-8") if ignore.is_file() else None
    except (OSError, UnicodeDecodeError):
        content = None
    if content not in ("*", "*\n", "*\r\n"):
        return skip("the exchange folder lacks a .gitignore whose content is exactly '*'")
    checked.append("outside every Git work tree")
    inside = _inside_git(root, environ)
    if inside is None:
        return skip("git is not available to confirm the exchange folder is outside every Git work tree")
    if inside:
        return skip("the exchange folder lies inside a Git work tree or Git directory; CAEPIPE outputs must stay "
                    "outside every repository")
    checked.append(f"{CONFIG_NAME} present")
    path = root / CONFIG_NAME
    if not path.is_file():
        return skip(f"the exchange folder has no {CONFIG_NAME}")
    try:
        config = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, UnicodeDecodeError, json.JSONDecodeError):
        return skip(f"{CONFIG_NAME} is not readable JSON")
    if not isinstance(config, dict):
        return skip(f"{CONFIG_NAME} is not a JSON object")
    checked.append(f"{ACKNOWLEDGEMENT_KEY} is true")
    if config.get(ACKNOWLEDGEMENT_KEY) is not True:
        return skip(f"{CONFIG_NAME} does not set {ACKNOWLEDGEMENT_KEY} to true")
    checked.append("required keys " + ", ".join(REQUIRED_KEYS))
    missing = [key for key in REQUIRED_KEYS if not isinstance(config.get(key), str) or not config[key].strip()]
    if missing:
        return skip(f"{CONFIG_NAME} lacks {', '.join(missing)}")
    checked.append("guest paths are allow-listed Windows absolute paths")
    for key in GUEST_PATH_KEYS:
        if not valid_guest_path(config[key], directory=key != "guest_executable"):
            return skip(f"{CONFIG_NAME} {key} is not an absolute Windows path (X:\\...) of letters, digits, space and "
                        "\\ . - _ ( ) only")
    prlctl = None
    if require_vm_tools:
        checked.append("prlctl on PATH")
        prlctl = shutil.which("prlctl", path=environ.get("PATH"))
        if prlctl is None:
            return skip("prlctl is not on PATH; the virtual machine cannot be reached")
    warnings = []
    mode = stat.S_IMODE(root.stat().st_mode)
    if mode & 0o077:
        warnings.append(f"exchange folder mode is {mode:o}; Design §5 asks for 700")
    first_launch = config.get("trial_first_launch_date")
    return HarnessConfig(
        exchange_root=root,
        vm_name=config["vm_name"],
        guest_executable=config["guest_executable"],
        guest_exchange_root=config["guest_exchange_root"].rstrip("\\"),
        guest_work_root=config["guest_work_root"].rstrip("\\"),
        prlctl=prlctl,
        acknowledged=True,
        trial_first_launch_date=first_launch if isinstance(first_launch, str) else None,
        checked=tuple(checked),
        warnings=tuple(warnings),
    )


def valid_guest_path(value: Any, *, directory: bool) -> bool:
    """True for an allow-listed absolute Windows path without `.`/`..` segments; a file may not end in `\\`."""

    if not isinstance(value, str) or not GUEST_PATH_PATTERN.match(value):
        return False
    stripped = value.rstrip("\\")
    if not directory and stripped != value:
        return False
    return all(part not in (".", "..") for part in stripped.split("\\")[1:])


def _inside_git(path: Path, environ: Mapping[str, str]) -> bool | None:
    env = {key: value for key, value in os.environ.items() if not key.startswith("GIT_")}
    if environ.get("PATH"):
        env["PATH"] = environ["PATH"]
    try:
        completed = subprocess.run(["git", "-C", str(path), "rev-parse", "--is-inside-work-tree"],
                                   capture_output=True, text=True, env=env, check=False, timeout=30)
    except (FileNotFoundError, subprocess.TimeoutExpired):
        return None
    # Exit 0 prints "true" inside a work tree and "false" inside a .git directory: both are refused.
    return completed.returncode == 0


def contained(root: Path, path: Path) -> Path:
    """Return `path` when no component below the resolved exchange root is a symbolic link and it stays inside.

    Applied to every directory and file the pilot creates, writes or deletes, so an existing link in the exchange
    folder can never redirect a write or a cleanup elsewhere (for example into a Git work tree).
    """

    base = Path(root).resolve()
    try:
        parts = Path(path).relative_to(base).parts
    except ValueError:
        raise ContainmentError(f"{path} is not inside the exchange folder") from None
    current = base
    for part in parts:
        if part in (".", ".."):
            raise ContainmentError(f"{path} contains a {part!r} segment")
        current = current / part
        if current.is_symlink():
            raise ContainmentError(f"{current.relative_to(base)} in the exchange folder is a symbolic link; refused")
    if current.exists() and not current.resolve().is_relative_to(base):
        raise ContainmentError(f"{current.relative_to(base)} resolves outside the exchange folder")
    return current


def contained_dir(root: Path, path: Path) -> Path:
    """`contained(root, path)`, created as a directory (parents checked one by one) when absent."""

    base = Path(root).resolve()
    current = base
    for part in contained(base, path).relative_to(base).parts:
        current = contained(base, current / part)
        if not current.exists():
            current.mkdir()
        elif not current.is_dir():
            raise ContainmentError(f"{current.relative_to(base)} in the exchange folder is not a directory")
    return current


def write_private(root: Path, path: Path, data: bytes | str) -> None:
    """Write `data` to `path` inside the exchange folder after the containment check."""

    target = contained(root, path)
    if isinstance(data, str):
        target.write_text(data, encoding="utf-8")
    else:
        target.write_bytes(data)


def case_paths(config: HarnessConfig | Path, case_id: str) -> dict[str, Path]:
    """The Design §5 run directory of one case, created on demand inside the exchange folder."""

    if not isinstance(case_id, str) or not CASE_ID_PATTERN.match(case_id):
        raise ValueError(f"case id {case_id!r} must match {CASE_ID_PATTERN.pattern}")
    root = (config.exchange_root if isinstance(config, HarnessConfig) else Path(config)).resolve()
    case_root = root / "cases" / case_id
    paths = {"root": case_root, "manifest": contained(root, case_root / "case_manifest.json")}
    contained_dir(root, case_root)
    for layer in CASE_LAYERS:
        paths[layer] = contained_dir(root, case_root / layer)
    return paths


def _now() -> str:
    return datetime.now(timezone.utc).isoformat(timespec="seconds")


def _sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def last_json_object(text: str) -> dict[str, Any] | None:
    """The last top-level JSON object in `text`, which may be preceded or followed by other output."""

    decoder = json.JSONDecoder()
    found = None
    index = 0
    while True:
        start = text.find("{", index)
        if start < 0:
            return found
        try:
            value, end = decoder.raw_decode(text, start)
        except json.JSONDecodeError:
            index = start + 1
            continue
        if isinstance(value, dict):
            found = value
        index = end


def _quote(path: str) -> str:
    # Only allow-listed guest paths (GUEST_PATH_PATTERN, CASE_ID_PATTERN and fixed names) are ever quoted here.
    if not re.fullmatch(r"[A-Za-z0-9 \\._()*:-]*", path):
        raise ValueError("a guest path outside the allow-list would enter a guest command")
    return f'"{path}"'


def _win(*parts: str) -> str:
    """Join guest (Windows) path parts with a backslash."""
    return "\\".join(parts)


def run_caepipe_round_trip(config: HarnessConfig, case_id: str, mbf_text: str, *, timeout_s: float = 600,
                           runner: Callable[..., subprocess.CompletedProcess] = subprocess.run) -> dict[str, Any]:
    """Solve one MBF with the user's CAEPIPE in the configured virtual machine and record what happened.

    The guest work is done in three `prlctl exec <vm> --current-user cmd /c <command>`
    steps (prepare, solve, collect) so each step's exit status is recorded on its
    own. A failed prepare step stops before solving (`prepare_failed`). Outputs are
    collected, even when the solve step fails, only when the guest reports solving
    an MBF whose SHA-256 equals the host's (otherwise `input_hash_mismatch`).
    """

    if config.prlctl is None:
        raise ValueError("the harness configuration was resolved without prlctl; live runs need it")
    for key in GUEST_PATH_KEYS:
        directory = key != "guest_executable"
        # Resolution strips a directory's trailing backslash (a drive root `X:\` is held as `X:`).
        value = getattr(config, key) + ("\\" if directory else "")
        if not valid_guest_path(value, directory=directory):
            raise ValueError(f"{key} is not an allow-listed absolute Windows path")
    paths = case_paths(config, case_id)
    harness_dir = contained_dir(config.exchange_root, config.exchange_root.resolve() / HARNESS_DIR)
    lock = contained(config.exchange_root, harness_dir / "round_trip.lock")
    try:
        handle = os.open(lock, os.O_CREAT | os.O_EXCL | os.O_WRONLY, 0o600)
    except FileExistsError as error:
        raise RuntimeError(f"another CAEPIPE round trip holds {lock}; runs are never parallel (remove a stale lock "
                           "only when no run is active)") from error
    try:
        os.write(handle, f"{os.getpid()} {_now()} {case_id}\n".encode("ascii"))
        os.close(handle)
        return _round_trip(config, case_id, mbf_text, paths, harness_dir, timeout_s, runner)
    finally:
        lock.unlink(missing_ok=True)


_CLEARED_OUTPUTS = {name.lower() for name in (*OUTPUT_NAMES, "guest_stdout.txt", "guest_stderr.txt", "guest_log.json",
                                               "run_record.json")}


def _refuse_links(root: Path, out: Path) -> None:
    """Refuse any symbolic link inside `caepipe_out`, whatever its letter case.

    The exchange folder may sit on a case-insensitive file system, where a link named `MODEL_RESULTS.CSV` would
    receive the guest's copy of `model_results.csv`; nothing in the layer is ever a link of ours.
    """

    for entry in contained(root, out).iterdir():
        if entry.is_symlink():
            raise ContainmentError(f"{entry.relative_to(Path(root).resolve())} in the exchange folder is a symbolic "
                                   "link; refused")
        # The layer only ever holds regular files; a directory at an output name would make the guest's `copy`
        # write inside it (and through any link it contains).
        if not entry.is_file():
            raise ContainmentError(f"{entry.relative_to(Path(root).resolve())} in the exchange folder is not a "
                                   "regular file; refused")


def _clear_outputs(root: Path, out: Path) -> None:
    """Delete the previous run's outputs from `caepipe_out`, matching names without regard to letter case."""

    _refuse_links(root, out)
    for entry in out.iterdir():
        name = entry.name.lower()
        if (name in _CLEARED_OUTPUTS or (name.startswith("model") and name.endswith(".csv"))) and entry.is_file():
            contained(root, entry).unlink()


def _round_trip(config: HarnessConfig, case_id: str, mbf_text: str, paths: dict[str, Path], harness_dir: Path,
                timeout_s: float, runner: Callable[..., subprocess.CompletedProcess]) -> dict[str, Any]:
    started = _now()
    root = config.exchange_root
    mbf_bytes = mbf_text.encode("utf-8")
    mbf_sha256 = _sha256(mbf_bytes)
    write_private(root, paths["caepipe_in"] / "model.mbf", mbf_bytes)
    script_bytes = GUEST_SCRIPT.read_bytes()
    write_private(root, harness_dir / GUEST_SCRIPT.name, script_bytes)
    out = paths["caepipe_out"]
    _clear_outputs(root, out)

    guest_case = _win(config.guest_exchange_root, "cases", case_id)
    guest_in, guest_out = _win(guest_case, "caepipe_in"), _win(guest_case, "caepipe_out")
    work = _win(config.guest_work_root, case_id)
    guest_mbf = _win(work, "model.mbf")
    share_script = _win(config.guest_exchange_root, HARNESS_DIR, GUEST_SCRIPT.name)
    work_script = _win(work, GUEST_SCRIPT.name)

    def solve_command(script: str) -> str:
        return (f"powershell -NoProfile -ExecutionPolicy Bypass -File {_quote(script)} "
                f"-Exe {_quote(config.guest_executable)} -Mbf {_quote(guest_mbf)}")

    steps: list[dict[str, Any]] = []

    def step(name: str, command: str) -> dict[str, Any]:
        argv = [config.prlctl, "exec", config.vm_name, "--current-user", "cmd", "/c", command]
        record: dict[str, Any] = {"name": name, "guest_command": command, "started": _now(), "attempts": 0}
        # The prlctl client intermittently fails a call before or after the guest command with exit 255 and a
        # "PrlJob_..." message. Every step is idempotent (prepare and solve clear old outputs first), so retry.
        while True:
            record["attempts"] += 1
            try:
                completed = runner(argv, capture_output=True, text=True, check=False, timeout=timeout_s)
                record.update(exit_status=completed.returncode, stdout=completed.stdout or "", stderr=completed.stderr or "")
            except subprocess.TimeoutExpired as error:
                record.update(exit_status="timeout", stdout=_text(error.stdout), stderr=_text(error.stderr))
            except OSError as error:
                record.update(exit_status="launch_failed", stdout="", stderr=str(error))
            if not (record["exit_status"] == 255 and "PrlJob_" in record["stderr"]) or record["attempts"] >= PRLCTL_ATTEMPTS:
                break
        record["finished"] = _now()
        steps.append(record)
        return record

    # The guest work copy of the MBF is deleted before the copy, so a failed copy can never leave an older MBF
    # to be solved; a failed prepare step stops the round trip before solving.
    stale_guest = " ".join(_quote(_win(work, name)) for name in ("model*.csv", "model.mbf", *OUTPUT_NAMES))
    prepare = step("prepare", f"if not exist {_quote(work)} mkdir {_quote(work)} & del /Q {stale_guest} 2>nul & "
                              f"copy /Y {_quote(_win(guest_in, 'model.mbf'))} {_quote(guest_mbf)}")
    script_location = "exchange_share"
    solve: dict[str, Any] = {"exit_status": None, "stdout": "", "stderr": ""}
    guest_log = None
    if prepare["exit_status"] == 0:
        solve = step("solve", solve_command(share_script))
        guest_log = last_json_object(solve["stdout"])
        if guest_log is None and solve["exit_status"] != "timeout":
            script_location = "guest_work_dir"
            step("stage_script", f"copy /Y {_quote(share_script)} {_quote(work_script)}")
            solve = step("solve_from_work_dir", solve_command(work_script))
            guest_log = last_json_object(solve["stdout"])
    # The guest script reports the SHA-256 of the MBF it actually solved; outputs are attributed to this MBF only
    # when that hash equals the host's.
    guest_mbf_sha256 = guest_log.get("mbf_sha256") if isinstance(guest_log, dict) else None
    guest_mbf_sha256 = guest_mbf_sha256.lower() if isinstance(guest_mbf_sha256, str) else None
    input_verified = guest_mbf_sha256 == mbf_sha256
    if guest_log is not None and input_verified:
        _refuse_links(root, out)  # again just before the guest writes into the layer
        step("collect", " & ".join([
            f"copy /Y {_quote(_win(work, 'model*.csv'))} {_quote(guest_out)}",
            *(f"copy /Y {_quote(_win(work, name))} {_quote(_win(guest_out, name))}" for name in OUTPUT_NAMES),
        ]))

    write_private(root, out / "guest_stdout.txt", solve["stdout"])
    write_private(root, out / "guest_stderr.txt", solve["stderr"])
    if guest_log is not None:
        write_private(root, out / "guest_log.json", json.dumps(guest_log, indent=2, sort_keys=True) + "\n")

    guest_csv = guest_log.get("csv_path") if isinstance(guest_log, dict) else None
    observed = None
    csv_name = guest_csv.replace("/", "\\").split("\\")[-1] if isinstance(guest_csv, str) else ""
    if csv_name not in ("", ".", "..") and input_verified:
        candidate = contained(root, out / csv_name)
        if candidate.is_file():
            observed = candidate
    status = guest_log.get("status") if isinstance(guest_log, dict) else None
    if prepare["exit_status"] != 0:
        route = "prepare_failed"
    elif solve["exit_status"] == "timeout":
        route = "timeout"
    elif guest_log is None:
        route = "no_guest_json"
    elif not input_verified:
        route = "input_hash_mismatch"
    elif status in CSV_ROUTES and observed is not None:
        route = status
    elif status in CSV_ROUTES:
        route = "csv_not_collected"
    else:
        route = str(status or "unknown_guest_status")
    gui_used = bool(isinstance(guest_log, dict) and isinstance(guest_log.get("gui"), dict) and guest_log["gui"].get("used"))

    record = {
        "document_kind": RUN_RECORD_KIND,
        "schema_version": "1",
        "deliverable": "DEL-17-05",
        "case_id": case_id,
        "configuration_surface": {
            "environment_variable": ENV_VAR,
            "config_file": CONFIG_NAME,
            "checked": list(config.checked),
            "warnings": list(config.warnings),
        },
        "executable_path_as_configured": config.guest_executable,
        ACKNOWLEDGEMENT_KEY: config.acknowledged,
        "mbf": {"path": "caepipe_in/model.mbf", "guest_path": guest_mbf, "sha256": mbf_sha256,
                "bytes": len(mbf_bytes)},
        "guest_mbf_sha256": guest_mbf_sha256,
        "input_hash_verified": input_verified,
        "guest_script": {"repository_path": GUEST_SCRIPT_REPOSITORY_PATH,
                         "exchange_copy": f"{HARNESS_DIR}/{GUEST_SCRIPT.name}",
                         "sha256": _sha256(script_bytes), "run_from": script_location},
        "command_shape": {
            "batch": BATCH_COMMAND_SHAPE,
            "gui_print_fallback": {
                "used": gui_used,
                "label": "print_to_file_fallback",
                "description": "fallback when the batch route yields no result file: the guest script stops the "
                               "batch process it started, opens the solved .res in a CAEPIPE instance it starts "
                               "itself and prints the results to CSV via Print > To File",
            },
            "host_invocation": "prlctl exec <vm_name> --current-user cmd /c <guest command>",
        },
        "steps": [{key: value for key, value in item.items() if key not in ("stdout", "stderr")}
                  | {"stdout_chars": len(item["stdout"]), "stderr_chars": len(item["stderr"])} for item in steps],
        "working_directory": work,
        "expected_batch_csv_path": _win(work, "model.csv"),
        "observed_csv_path": {"guest": guest_csv, "host": f"caepipe_out/{observed.name}" if observed else None},
        "route": route,
        "exit_status": solve["exit_status"],
        "stdout_file": "caepipe_out/guest_stdout.txt",
        "stderr_file": "caepipe_out/guest_stderr.txt",
        "guest_log": guest_log,
        "outputs_collected": sorted(path.name for path in out.iterdir()
                                    if path.name.startswith("model") and path.is_file() and not path.is_symlink()),
        "timestamps": {"started": started, "finished": _now()},
        "boundary_note": BOUNDARY_NOTE,
    }
    write_private(root, out / "run_record.json", json.dumps(record, indent=2, sort_keys=True) + "\n")
    return record


def _text(value: Any) -> str:
    if value is None:
        return ""
    if isinstance(value, bytes):
        return value.decode("utf-8", "replace")
    return str(value)
