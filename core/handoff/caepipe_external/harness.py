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
own timeout), and runs one case at a time. Its export step writes a stored solved
model back to MBF (`caepipe_out/exported.mbf`) for the DEL-17-04 import
regression. Everything it writes under
`caepipe_out/` is CAEPIPE output or derived from it and stays private (§8).
Its records are regression and handoff evidence only.

An optional `settings_guard` in the configuration names the guest registry key under
which the user's tool keeps its saved settings. When it is set, no round trip starts
while any instance of the tool is running in the guest; every round trip reads the
whole key before the guest work (no reading, no run) and again afterwards, after
confirming from the guest's process list that every process the guest script
started has ended and that no other instance appeared. Any difference, and any
uncertain process state, is recorded in the run record for the caller to stop on.
Only when the configuration enables restoring are differences written back: with
`restore_recent_files` only a difference lying wholly in the recent-files subkey's values,
with `restore_scope` `"key"` any difference strictly inside the configured key (values,
and subkeys created or deleted, never the key itself or anything outside it). The key is
then read a third time and must equal the first reading; nothing else is ever written.
The restore is safe only because the precondition admits no other instance of the
tool that could be changing its settings at the same time (Design §5).
"""

from __future__ import annotations

from dataclasses import dataclass, field
from datetime import datetime, timezone
import contextlib
import csv
import hashlib
import io
import json
import math
import os
from pathlib import Path
import re
import shutil
import signal
import stat
import subprocess
import threading
import time
from types import SimpleNamespace
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
# The batch print written with a .txt extension (the tool's saved print-file extension); accepted only when its
# content parses as the documented print-CSV layout (`_print_layout_problem`).
PRINT_TXT_NAME = "model.txt"
PRLCTL_ATTEMPTS = 3
CSV_ROUTES = ("csv_from_batch", "csv_from_gui_print")
EXPORTED_MBF = "exported.mbf"
EXPORT_OUTPUTS = (EXPORTED_MBF, "export_guest_stdout.txt", "export_guest_stderr.txt", "export_guest_log.json")
EXPORT_ROUTE = "mbf_exported"
# Guest paths enter `cmd /c` command strings, so they are held to a strict allow-list: an absolute Windows path
# (drive letter, colon, backslash) of letters, digits, space and \ . - _ ( ) only. Quotes, %, ^, &, |, <, >, !
# and control characters can never reach the guest shell.
GUEST_PATH_KEYS = ("guest_executable", "guest_exchange_root", "guest_work_root")
GUEST_PATH_PATTERN = re.compile(r"^[A-Za-z]:\\[A-Za-z0-9 \\._()-]*\Z")
SETTINGS_KEY_PATTERN = re.compile(r"^HKCU(\\[A-Za-z0-9 ._()-]+)+\Z")
SETTINGS_SUBKEY_PATTERN = re.compile(r"^[A-Za-z0-9 ._()-]+\Z")
# A value line of `reg query`: four spaces, name, four spaces, type, then four spaces and the data (absent when empty).
REG_VALUE_LINE = re.compile(r"^ {4}(?P<name>.+?) {4}(?P<type>REG_[A-Z_]+)(?: {4}(?P<data>.*))?\Z")
SETTINGS_KEEP_GOING = ("unchanged", "recent_files_restored", "settings_restored")
# What a restore may cover: the recent-files subkey's values, or everything strictly inside the configured key.
RESTORE_SCOPES = ("recent_files", "key")
# Names a restore writes into a guest command: printed keys and value names within this character set only.
REG_PRINTED_KEY_PATTERN = re.compile(r"^HKEY_CURRENT_USER(\\[A-Za-z0-9 ._()-]+)+\Z")
REG_VALUE_NAME_PATTERN = re.compile(r"^[A-Za-z0-9 ._()-]+\Z")
# Data a restore can write back exactly through `reg add` (REG_SZ only within the guest-command allow-list).
RESTORABLE_DATA = {"REG_BINARY": re.compile(r"^[0-9A-Fa-f]*\Z"), "REG_DWORD": re.compile(r"^0x[0-9A-Fa-f]{1,8}\Z"),
                   "REG_SZ": re.compile(r"^[A-Za-z0-9 \\._()*:-]*\Z")}
# Written by the guest script in the case's work folder: one id per process it starts (guarded runs only).
OWNED_PIDS_FILE = "owned_pids.txt"
# The guard's result, with its write log, persisted on its own before the run record (guarded runs only).
GUARD_RESULT_FILE = "settings_guard.json"
# Guard call limits (configurable in settings_guard): each guard call's timeout, and the total time the restore's
# registry-mutation section may take. The solve keeps its own timeout.
GUARD_CALL_TIMEOUT_S = 20.0
GUARD_SECTION_BUDGET_S = 120.0
# Guard reads (process lists, registry readings, read-backs) have no side effect, so a transport failure of the
# prlctl client is retried more patiently than other steps: up to this many attempts, this far apart, all within
# the call's timeout (and the section budget inside the restore). Writes keep their single settled retry.
GUARD_READ_ATTEMPTS = 6
GUARD_READ_BACKOFF_S = 2.0


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
    settings_guard: "SettingsGuard | None" = None


@dataclass(frozen=True)
class SettingsGuard:
    """The guest registry key of the tool's saved settings, read whole, and its recent-files subkey.

    `restore_recent_files` (off unless configured) lets a round trip write back the recent-files subkey alone when
    nothing else changed; `restore_scope` `"key"` lets it write back any difference strictly inside `key` (and
    enables restoring by itself). Either is safe only because the precondition admits no instance of the tool but
    our own.
    """

    key: str
    recent_files_subkey: str | None = None
    restore_recent_files: bool = False
    call_timeout_s: float = GUARD_CALL_TIMEOUT_S
    section_budget_s: float = GUARD_SECTION_BUDGET_S
    restore_scope: str = "recent_files"

    @property
    def recent_files_key(self) -> str | None:
        return f"{self.key}\\{self.recent_files_subkey}" if self.recent_files_subkey else None

    @property
    def restores_whole_key(self) -> bool:
        return self.restore_scope == "key"


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
    guard = None
    if "settings_guard" in config:
        checked.append("settings_guard names an HKCU key")
        raw_guard = config["settings_guard"]
        key = raw_guard.get("key") if isinstance(raw_guard, dict) else None
        subkey = raw_guard.get("recent_files_subkey") if isinstance(raw_guard, dict) else None
        restore = raw_guard.get("restore_recent_files", False) if isinstance(raw_guard, dict) else None
        limits = {name: (raw_guard.get(name, default) if isinstance(raw_guard, dict) else None)
                  for name, default in (("call_timeout_s", GUARD_CALL_TIMEOUT_S),
                                        ("section_budget_s", GUARD_SECTION_BUDGET_S))}
        scope = raw_guard.get("restore_scope", "recent_files") if isinstance(raw_guard, dict) else None
        limits_valid = all(isinstance(value, (int, float)) and not isinstance(value, bool) and math.isfinite(value)
                           and value > 0 for value in limits.values())
        if not (isinstance(key, str) and SETTINGS_KEY_PATTERN.match(key)
                and (subkey is None or isinstance(subkey, str) and SETTINGS_SUBKEY_PATTERN.match(subkey))
                and isinstance(restore, bool) and (subkey is not None or not restore) and limits_valid
                and scope in RESTORE_SCOPES):
            return skip(f"{CONFIG_NAME} settings_guard must give key (HKCU\\..., letters, digits, space and "
                        ". _ ( ) - only), optionally recent_files_subkey (the same characters), "
                        "restore_recent_files (true or false; true needs recent_files_subkey), restore_scope "
                        "(\"recent_files\" or \"key\") and positive call_timeout_s and section_budget_s (seconds)")
        guard = SettingsGuard(key=key, recent_files_subkey=subkey, restore_recent_files=restore,
                              call_timeout_s=float(limits["call_timeout_s"]),
                              section_budget_s=float(limits["section_budget_s"]), restore_scope=scope)
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
        settings_guard=guard,
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


def _ensure_dir(path: str) -> str:
    """A guest command creating `path` when absent. Bracketed: in `cmd`, an unbracketed `if` takes the rest of the
    line, `& ...` included, as its body, so the commands after it would run only when the folder was absent."""
    return f"(if not exist {_quote(path)} mkdir {_quote(path)})"


def _win(*parts: str) -> str:
    """Join guest (Windows) path parts with a backslash."""
    return "\\".join(parts)


def run_caepipe_round_trip(config: HarnessConfig, case_id: str, mbf_text: str, *, timeout_s: float = 600,
                           runner: Callable[..., subprocess.CompletedProcess] = subprocess.run,
                           gui_fallback: bool = True) -> dict[str, Any]:
    """Solve one MBF with the user's CAEPIPE in the configured virtual machine and record what happened.

    The guest work is done in three `prlctl exec <vm> --current-user cmd /c <command>`
    steps (prepare, solve, collect) so each step's exit status is recorded on its
    own. A failed prepare step stops before solving (`prepare_failed`). Outputs are
    collected, even when the solve step fails, only when the guest reports solving
    an MBF whose SHA-256 equals the host's (otherwise `input_hash_mismatch`).

    `gui_fallback=False` tells the guest script not to fall back to printing through the GUI when the batch route
    leaves no CSV (route `batch_no_csv`). With `settings_guard` configured, the run record's `settings_guard`
    holds the detect-only check; an exception raised after the guest work started carries it as `settings_guard`.
    """

    return _locked(config, case_id, lambda paths, harness_dir: _round_trip(
        config, case_id, mbf_text, paths, harness_dir, timeout_s, runner, gui_fallback))


def _locked(config: HarnessConfig, case_id: str, work: Callable[[dict[str, Path], Path], dict[str, Any]]) -> dict[str, Any]:
    """Check the guest paths, then run `work` for one case under the exchange folder's single-run lock."""

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
        return work(paths, harness_dir)
    finally:
        lock.unlink(missing_ok=True)


# A new solve also clears the previous solve's MBF export, which no longer describes the solved model.
_CLEARED_OUTPUTS = {name.lower() for name in (*OUTPUT_NAMES, PRINT_TXT_NAME, "guest_stdout.txt", "guest_stderr.txt", "guest_log.json",
                                               GUARD_RESULT_FILE,
                                               "run_record.json", *EXPORT_OUTPUTS)}


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


def _guest_read(config: HarnessConfig, runner: Callable[..., subprocess.CompletedProcess], timeout_s: float,
                steps: list[dict[str, Any]], name: str, command: str, deadline: float | None = None) -> dict[str, Any]:
    """A guard read (`_guest_step`): up to `GUARD_READ_ATTEMPTS` attempts `GUARD_READ_BACKOFF_S` apart, all within
    `timeout_s` and `deadline`."""

    return _guest_step(config, runner, timeout_s, steps, name, command, deadline, attempts=GUARD_READ_ATTEMPTS,
                       backoff_s=GUARD_READ_BACKOFF_S, within_timeout=True)


def _backoff(seconds: float) -> None:
    time.sleep(seconds)


def _guest_step(config: HarnessConfig, runner: Callable[..., subprocess.CompletedProcess], timeout_s: float,
                steps: list[dict[str, Any]], name: str, command: str, deadline: float | None = None, *,
                attempts: int = PRLCTL_ATTEMPTS, backoff_s: float = 0.0,
                within_timeout: bool = False) -> dict[str, Any]:
    """Run one guest command through `prlctl exec`, append its record to `steps` and return it.

    A transport failure of the prlctl client (exit 255, `PrlJob_`) is retried, up to `attempts` in all, `backoff_s`
    apart. With `deadline` (a `time.monotonic()` value), and with `within_timeout` (all attempts together within
    `timeout_s`), no attempt runs past the limit: each attempt's timeout is cut to what remains, and no further
    attempt starts when the limit would be reached before it does; the last attempt's result is then returned.
    """

    argv = [config.prlctl, "exec", config.vm_name, "--current-user", "cmd", "/c", command]
    record: dict[str, Any] = {"name": name, "guest_command": command, "started": _now(), "attempts": 0}
    limit = deadline
    if within_timeout:
        call_end = time.monotonic() + timeout_s
        limit = call_end if limit is None else min(limit, call_end)
    # The prlctl client intermittently fails a call before or after the guest command with exit 255 and a
    # "PrlJob_..." message. Every step is idempotent (each clears its old outputs first), so retry.
    while True:
        record["attempts"] += 1
        attempt_timeout = timeout_s if limit is None else max(0.001, min(timeout_s, limit - time.monotonic()))
        try:
            completed = runner(argv, capture_output=True, text=True, check=False, timeout=attempt_timeout)
            record.update(exit_status=completed.returncode, stdout=completed.stdout or "", stderr=completed.stderr or "")
        except subprocess.TimeoutExpired as error:
            record.update(exit_status="timeout", stdout=_text(error.stdout), stderr=_text(error.stderr))
        except OSError as error:
            record.update(exit_status="launch_failed", stdout="", stderr=str(error))
        if not (record["exit_status"] == 255 and "PrlJob_" in record["stderr"]) or record["attempts"] >= attempts:
            break
        if limit is not None and time.monotonic() + backoff_s >= limit:
            record["attempts_stopped_by"] = "deadline"
            break
        if backoff_s:
            _backoff(backoff_s)
    record["finished"] = _now()
    steps.append(record)
    return record


class TerminationRequested(KeyboardInterrupt):
    """SIGTERM (or SIGBREAK) received during a guarded round trip, raised as a cancellation like Ctrl-C."""


class SectionBudgetExceeded(RuntimeError):
    """The restore's registry-mutation section used up its time budget (`section_budget_s`)."""


@contextlib.contextmanager
def _catchable_termination(active: bool, live: dict[str, Any] | None = None):
    """For a guarded round trip, make SIGTERM (and SIGBREAK where it exists) raise TerminationRequested instead of
    ending the process unrecorded; inside the mutation section `_deferred_signals` defers it as before. Each
    TerminationRequested carries the guard result as it stands when the signal arrives (`_live_guard_result`), so
    a termination request at any statement, a second one included, never leaves an exception without the write
    log. The previous handlers are restored on exit. Main thread only (elsewhere nothing changes)."""

    if not active or threading.current_thread() is not threading.main_thread():
        yield
        return
    previous: dict[int, Any] = {}

    def terminate(signum, frame):  # noqa: ARG001
        error = TerminationRequested(f"signal {signal.Signals(signum).name}")
        if live is not None and live.get("guard_state") is not None:
            try:
                _attach_guard(error, _live_guard_result(live))
            except Exception:  # the request is raised whatever happens; the round trip's recovery attaches again
                pass
        raise error

    try:
        for name in ("SIGTERM", "SIGBREAK"):
            number = getattr(signal, name, None)
            if number is not None:
                previous[number] = signal.signal(number, terminate)
        yield
    finally:
        for number, handler in previous.items():
            signal.signal(number, handler)


@contextlib.contextmanager
def _deferred_signals(active: bool):
    """Defer SIGINT, SIGTERM and (where it exists) SIGBREAK across the restore's registry-mutation section.

    While active, each such signal is recorded in `received` instead of interrupting, so neither a Ctrl-C nor a
    termination request can cut between a check and its write or leave a write half-logged; the previous handlers
    are restored on exit and the caller acts on what was received. The section is made only of guest calls, each
    with its own timeout, so the deferral is bounded. Python lets handlers be installed only on the main thread:
    elsewhere `deferral` says so and nothing is deferred.
    """

    state = SimpleNamespace(received=[], deferral="not needed")
    if not active:
        yield state
        return
    if threading.current_thread() is not threading.main_thread():
        state.deferral = "unavailable off the main thread"
        yield state
        return
    previous: dict[int, Any] = {}

    def record(signum, frame):  # noqa: ARG001 - the frame is not needed
        state.received.append(signum)

    try:
        for name in ("SIGINT", "SIGTERM", "SIGBREAK"):
            number = getattr(signal, name, None)
            if number is not None:
                previous[number] = signal.signal(number, record)
        state.deferral = "active"
        yield state
    finally:
        for number, handler in previous.items():
            signal.signal(number, handler)


def _secondary(settings: dict[str, Any] | None, where: str, error: BaseException) -> None:
    """Keep a later error beside the primary one in the guard result (never in place of it)."""
    if settings is not None:
        settings.setdefault("secondary_errors", []).append(f"{where}: {type(error).__name__}: {error}")


# What `reg add` and `reg delete` print on success; a write counts as completed only with exit 0 and this text.
REG_WRITE_OK = "The operation completed successfully"


class RestoreWriteFailed(RuntimeError):
    """A registry write of the restore did not complete: `outcome` is `failed` (the command reported failure) or
    `unknown` (the transport failed, or the output was not the expected one; the write may or may not have taken)."""

    def __init__(self, name: str, outcome: str, exit_status: Any):
        super().__init__(f"{name}: write {outcome} (exit status {exit_status})")
        self.outcome = outcome
        self.exit_status = exit_status


def _guest_write(config: HarnessConfig, runner: Callable[..., subprocess.CompletedProcess], timeout_s: float,
                 steps: list[dict[str, Any]], name: str, command: str,
                 state: dict[str, Any] | None = None, deadline: float | None = None) -> dict[str, Any]:
    """Run one mutating guest command exactly once: this call never retries, whatever the transport reports (the
    restore may issue a value set once more, only after settling its outcome; `_restore_recent_files`).

    The step is recorded before anything is raised. Exit 0 with `REG_WRITE_OK` in its output is the only success.
    Exit 255 with a `PrlJob_` message (the client's transport failure) or exit 0 with other output is `unknown`; any
    other exit is `failed`; both raise RestoreWriteFailed. An exception from the runner (a timeout, a launch
    failure, cancellation) is stored as `state["primary_error"]` before anything else (unless one is already
    there), then recorded under a guard whose own failure is kept as secondary, and re-raised unchanged.
    """

    argv = [config.prlctl, "exec", config.vm_name, "--current-user", "cmd", "/c", command]
    # Complete from the start, so the step stays recordable however its own logging ends.
    record: dict[str, Any] = {"name": name, "guest_command": command, "started": _now(), "attempts": 1,
                              "mutating": True, "exit_status": "not_returned", "stdout": "", "stderr": ""}
    if deadline is not None:
        timeout_s = max(0.001, min(timeout_s, deadline - time.monotonic()))
    try:
        completed = runner(argv, capture_output=True, text=True, check=False, timeout=timeout_s)
    except BaseException as error:
        if state is not None:
            state.setdefault("primary_error", error)
        try:
            record.update(exit_status=type(error).__name__, stdout=_text(getattr(error, "stdout", None)),
                          stderr=_text(getattr(error, "stderr", None)) or str(error), finished=_now())
        except BaseException as late:
            if state is not None:
                state.setdefault("secondary_errors", []).append(late)
        finally:
            steps.append(record)
        raise error
    record.update(exit_status=completed.returncode, stdout=completed.stdout or "", stderr=completed.stderr or "",
                  finished=_now())
    steps.append(record)
    if completed.returncode == 0 and REG_WRITE_OK.lower() in record["stdout"].lower():
        record["outcome"] = "completed"
        return record
    transport = completed.returncode == 255 and "PrlJob_" in record["stderr"]
    record["outcome"] = "unknown" if transport or completed.returncode == 0 else "failed"
    raise RestoreWriteFailed(name, record["outcome"], completed.returncode)


def _print_layout_problem(path: Path) -> str | None:
    """Why a print file is not the documented print-CSV layout, or None when it is: the strict print parser must
    recognise its header, parse at least one section and report no error."""

    from .print_csv import read_caepipe_print_csv  # the package imports this module; resolved at call time

    try:
        parsed = read_caepipe_print_csv(path)
    except Exception as error:  # an unreadable file is not the layout
        return f"unreadable: {type(error).__name__}"
    codes = [item["code"] for item in parsed["diagnostics"]
             if item["severity"] == "error" or item["code"] == "CAEPIPE-PRINT-HEADER-UNRECOGNIZED"]
    if codes:
        return "not the print-CSV layout: " + ", ".join(sorted(set(codes)))
    if not parsed["sections_parsed"]:
        return "not the print-CSV layout: no section recognised"
    return None


def _transport_failure(error: BaseException) -> bool:
    """A write whose outcome the transport left unknown: the prlctl client's exit 255 (`PrlJob_`), or a timeout."""
    return (isinstance(error, RestoreWriteFailed) and error.outcome == "unknown" and error.exit_status == 255) \
        or isinstance(error, subprocess.TimeoutExpired)


# What `reg query` reports, exit 1, for a key or value that does not exist; any other failure is unreadable.
REG_NOT_FOUND = "unable to find the specified registry key or value"


def _not_found(record: Mapping[str, Any]) -> bool:
    return record["exit_status"] == 1 and REG_NOT_FOUND in (record["stdout"] + record["stderr"]).lower()


def _read_value(step: Callable[..., dict[str, Any]], name: str, key: str,
                value_name: str) -> tuple[str, tuple[str, str] | None]:
    """One value read back alone (`reg query <key> /v <name>`): ("present", (type, data)), ("absent", None) when
    reg reports that it does not exist, or ("unreadable", None) when the read fails otherwise, does not parse
    whole, or does not show exactly that one value."""

    record = step(name, f"reg query {_quote(key)} /v {_quote(value_name)}")
    if _not_found(record):
        return "absent", None
    parsed = parse_reg_query(record["stdout"]) if record["exit_status"] == 0 else None
    if parsed is None or list(parsed["values"]) != [(_registry_key_id(key), value_name)]:
        return "unreadable", None
    found = parsed["values"][(_registry_key_id(key), value_name)]
    return "present", (found[1], found[2])


def _read_subtree(step: Callable[..., dict[str, Any]], name: str,
                  key: str) -> tuple[str, dict[tuple[str, str], tuple[str, str]] | None]:
    """A subkey read back whole (`reg query <key> /s`): ("present", {(key id, value name): (type, data)}) for
    every value in it and below, ("absent", None) when reg reports that it does not exist, or ("unreadable",
    None) otherwise (a failed read, output that does not parse whole, or a key printed outside it). A present
    subkey without values prints nothing."""

    record = step(name, f"reg query {_quote(key)} /s")
    if _not_found(record):
        return "absent", None
    if record["exit_status"] != 0:
        return "unreadable", None
    if not record["stdout"].strip():
        return "present", {}
    parsed = parse_reg_query(record["stdout"])
    key_id = _registry_key_id(key)
    if parsed is None or not all(found == key_id or found.startswith(key_id + "\\") for found in parsed["keys"]):
        return "unreadable", None
    return "present", {item: value[1:] for item, value in parsed["values"].items()}


def _write_outcome(error: BaseException) -> str:
    """The write log's outcome for a write that raised: a launch failure never reached the guest."""
    if isinstance(error, RestoreWriteFailed):
        return error.outcome
    if isinstance(error, OSError):
        return "failed"
    return "unknown"


def _registry_key_id(key: str) -> str:
    """A registry key for comparison: lower-cased, with `HKEY_CURRENT_USER` (as `reg query` prints it) as `HKCU`."""

    key = key.strip().lower()
    return "hkcu" + key[len("hkey_current_user"):] if key.startswith("hkey_current_user\\") else key


def parse_reg_query(text: str) -> dict[str, dict] | None:
    """`reg query <key> /s` output as a whole snapshot: {"keys": {key id: key as printed}, "values": {(key id,
    value name): (key as printed, type, data)}} (`_registry_key_id`). Every key is kept, empty ones included.

    None when the output names no key, or has a line that is neither blank, a key nor a value line: a reading
    that is not understood whole is no reading.
    """

    keys: dict[str, str] = {}
    values: dict[tuple[str, str], tuple[str, str, str]] = {}
    current = None
    for line in text.splitlines():
        line = line.rstrip("\r")
        if not line.strip():
            continue
        if line.startswith("HKEY_"):
            current = line.strip()
            keys[_registry_key_id(current)] = current
            continue
        match = REG_VALUE_LINE.match(line)
        if not match or current is None:
            return None
        values[(_registry_key_id(current), match["name"])] = (current, match["type"], match["data"] or "")
    return {"keys": keys, "values": values} if keys else None


def _settings_snapshot(config: HarnessConfig, step: Callable[[str, str], dict[str, Any]],
                       name: str) -> dict[str, dict] | None:
    """Read the whole configured settings key, every subkey included, through one guest step.

    None unless the step succeeded, its output parses whole and every key it prints is the configured key or lies
    under it. `reg query /s` prints no header for the queried key when that key holds no values of its own (only
    its subkeys); a successful query that prints subkeys of it shows that it exists, so it is then recorded as
    printed in `HKEY_CURRENT_USER` form. An output that prints no key at all is no reading.
    """

    record = step(name, f"reg query {_quote(config.settings_guard.key)} /s")
    snapshot = parse_reg_query(record["stdout"]) if record["exit_status"] == 0 else None
    root = _registry_key_id(config.settings_guard.key)
    if snapshot is None or not all(key == root or key.startswith(root + "\\") for key in snapshot["keys"]):
        return None
    snapshot["keys"].setdefault(root, "HKEY_CURRENT_USER" + config.settings_guard.key[len("HKCU"):])
    return snapshot


def settings_difference(before: Mapping[str, Mapping], after: Mapping[str, Mapping]) -> list[dict[str, Any]]:
    """Every difference between two whole snapshots: a key added or removed (empty ones included), and a value
    added, removed or changed in type or data."""

    changed: list[dict[str, Any]] = []
    for key in sorted(set(before["keys"]) | set(after["keys"])):
        if (key in before["keys"]) != (key in after["keys"]):
            changed.append({"key": (after["keys"].get(key) or before["keys"][key]),
                            "before": "present" if key in before["keys"] else None,
                            "after": "present" if key in after["keys"] else None})
    for item in sorted(set(before["values"]) | set(after["values"])):
        old, new = before["values"].get(item), after["values"].get(item)
        if (old or (None, None, None))[1:] != (new or (None, None, None))[1:]:
            changed.append({"value": f"{(new or old)[0]}\\{item[1]}",
                            "before": {"type": old[1], "data": old[2]} if old else None,
                            "after": {"type": new[1], "data": new[2]} if new else None})
    return changed


def _unknown(reason: str, **extra: Any) -> dict[str, Any]:
    return {"status": "unknown", "reason": reason, **extra}


def _tool_processes(config: HarnessConfig, step: Callable[[str, str], dict[str, Any]], name: str) -> list[int] | None:
    """The ids of every process of the tool's image in the guest's process list, or None when the list cannot be
    read or is not valid `tasklist /FO CSV /NH` output (no row, or a row that is not five fields with a decimal id)."""

    listing = step(name, "tasklist /FO CSV /NH")
    if listing["exit_status"] != 0:
        return None
    rows = [line for line in listing["stdout"].splitlines() if line.strip()]
    parsed = [next(csv.reader([line])) for line in rows]
    if not parsed or not all(len(row) == 5 and re.fullmatch(r"[0-9]{1,10}", row[1].strip()) for row in parsed):
        return None
    image = config.guest_executable.split("\\")[-1].lower()
    return sorted(int(row[1]) for row in parsed if row[0].strip().lower() == image)


def _precondition(config: HarnessConfig, step: Callable[[str, str], dict[str, Any]], name: str) -> dict[str, Any]:
    """Before anything starts the tool: no instance of it may be running (none is ours yet). Read only."""

    try:
        found = _tool_processes(config, step, name)
    except Exception as failure:
        return {"status": "precondition_unknown", "reason": f"the process check failed: {type(failure).__name__}"}
    if found is None:
        return {"status": "precondition_unknown", "reason": "the guest's process list could not be read or is "
                                                            "malformed"}
    if found:
        return {"status": "foreign_instance_running", "foreign_pids": found,
                "reason": "an instance of the tool this run did not start is running; nothing ran"}
    return {"status": "clear"}


def check_no_foreign_instance(config: HarnessConfig, *, timeout_s: float | None = None,
                              runner: Callable[..., subprocess.CompletedProcess] = subprocess.run) -> dict[str, Any]:
    """Session precondition: `clear` when no instance of the tool is running in the guest. Starts and stops nothing.
    Each attempt has the guard's call timeout (`call_timeout_s`, default `GUARD_CALL_TIMEOUT_S`)."""

    if timeout_s is None:
        timeout_s = config.settings_guard.call_timeout_s if config.settings_guard else GUARD_CALL_TIMEOUT_S
    steps: list[dict[str, Any]] = []
    return _precondition(config, lambda name, command: _guest_read(config, runner, timeout_s, steps, name, command),
                         "session_process_list")


def check_guest_alive(config: HarnessConfig, *, timeout_s: float = GUARD_CALL_TIMEOUT_S,
                      runner: Callable[..., subprocess.CompletedProcess] = subprocess.run) -> dict[str, Any]:
    """Liveness check: `alive` when the guest answers `cmd /c ver` with exit status 0 and some output. Starts and
    stops nothing; the guest's output is not kept."""

    steps: list[dict[str, Any]] = []
    try:
        record = _guest_step(config, runner, timeout_s, steps, "liveness", "ver")
    except Exception as failure:
        return {"status": "unreachable", "reason": f"the liveness call failed: {type(failure).__name__}"}
    if record["exit_status"] != 0 or not record["stdout"].strip():
        return {"status": "unreachable",
                "reason": f"the guest did not answer the liveness check (exit status {record['exit_status']})"}
    return {"status": "alive"}


def _owned_processes(config: HarnessConfig, step: Callable[[str, str], dict[str, Any]], work: str,
                     guest_log: Any) -> dict[str, Any]:
    """Whether every process the guest script started has exited, and no other instance of the tool appeared.

    The guest script appends the id of each process it starts to `OWNED_PIDS_FILE` in the work folder (and lists
    them in its report as `owned_pids`). The record must be complete and valid: readable, at least one id (every
    round trip that reaches the guest script starts the tool), one decimal id per line, and equal to the report's
    list when there is a report. The process list must be valid `tasklist /FO CSV /NH` output. No process is
    stopped or signalled here. `exited` when no instance of the tool is listed; `running` when one of ours is;
    `foreign` when one that is not ours is (the precondition checked again); `unknown` whenever the record or the
    list falls short.
    """

    ids = step("owned_pids", f"type {_quote(_win(work, OWNED_PIDS_FILE))}")
    if ids["exit_status"] != 0:
        return _unknown("the record of the processes the guest started could not be read")
    lines = [line.strip() for line in ids["stdout"].splitlines() if line.strip()]
    if not lines or not all(re.fullmatch(r"[0-9]{1,10}", line) for line in lines):
        return _unknown("the record of the processes the guest started is empty or malformed")
    pids = sorted({int(line) for line in lines})
    if isinstance(guest_log, dict):
        reported = guest_log.get("owned_pids")
        reported = [reported] if isinstance(reported, int) else reported
        if not (isinstance(reported, list) and all(isinstance(pid, int) for pid in reported)
                and sorted(set(reported)) == pids):
            return _unknown("the guest's report and its record of the processes it started disagree", owned_pids=pids)
    found = _tool_processes(config, step, "process_list")
    if found is None:
        return _unknown("the guest's process list could not be read or is malformed", owned_pids=pids)
    running = [pid for pid in found if pid in pids]
    foreign = [pid for pid in found if pid not in pids]
    if running:
        return {"status": "running", "owned_pids": pids, "running": running, "foreign_pids": foreign}
    if foreign:
        return {"status": "foreign", "owned_pids": pids, "foreign_pids": foreign}
    return {"status": "exited", "owned_pids": pids}


def _restore_plan(before: Mapping[str, Mapping], after: Mapping[str, Mapping], scope_id: str, *,
                  whole_key: bool = False) -> list[dict[str, Any]]:
    """The writes that take the restore scope back to `before`.

    The scope is the key `scope_id`: its own values alone, or (`whole_key`) its values and everything strictly
    inside it. Subkeys that were removed are created first (shallowest first); then values that changed or were
    removed are set to their first-reading type and data, in the first reading's order, and values that appeared
    are deleted (except inside a subkey that appeared); last, subkeys that appeared are deleted (deepest first).
    `scope_id` itself is never created or deleted.

    ValueError (nothing is written) when any difference lies outside the scope, when a key or value name falls
    outside the allowed characters, or when a value cannot be written back exactly.
    """

    def inside(key_id: str, *, strictly: bool = False) -> bool:
        if not whole_key:
            return key_id == scope_id and not strictly
        return key_id.startswith(scope_id + "\\") or (key_id == scope_id and not strictly)

    def printed_key(printed: str, key_id: str) -> str:
        if not REG_PRINTED_KEY_PATTERN.match(printed) or _registry_key_id(printed) != key_id:
            raise ValueError(f"{printed} cannot be named in a guest command")
        return printed

    differing_values = [item for item in set(before["values"]) | set(after["values"])
                        if before["values"].get(item, (None,))[1:] != after["values"].get(item, (None,))[1:]]
    differing_keys = set(before["keys"]) ^ set(after["keys"])
    # Containment first: any difference outside the scope refuses the whole restore before any write.
    for key_id in differing_keys:
        if not inside(key_id, strictly=True):
            raise ValueError(f"{(before['keys'].get(key_id) or after['keys'].get(key_id))} lies outside the restore "
                             "scope")
    for item in differing_values:
        if not inside(item[0]):
            raise ValueError(f"{(before['values'].get(item) or after['values'][item])[0]}\\{item[1]} lies outside "
                             "the restore scope")
    added = sorted((key for key in after["keys"] if key not in before["keys"]), key=lambda key: -key.count("\\"))
    removed = sorted((key for key in before["keys"] if key not in after["keys"]), key=lambda key: key.count("\\"))

    plan = []
    for key_id in removed:
        printed = printed_key(before["keys"][key_id], key_id)
        plan.append({"key": printed, "action": "create_key", "restores": "present", "replaces": None,
                     "command": f"reg add {_quote(printed)} /f"})
    for item, (printed, kind, data) in before["values"].items():
        if item in differing_values:
            if kind not in RESTORABLE_DATA or not RESTORABLE_DATA[kind].match(data) or item[1] == "(Default)" \
                    or not REG_VALUE_NAME_PATTERN.match(item[1]):
                raise ValueError(f"{printed}\\{item[1]} ({kind}) cannot be written back exactly")
            printed = printed_key(printed, item[0])
            found = after["values"].get(item)
            plan.append({"value": f"{printed}\\{item[1]}", "action": "set", "target": (printed, item[1]),
                         "restores": {"type": kind, "data": data},
                         "replaces": {"type": found[1], "data": found[2]} if found else None,
                         "command": f"reg add {_quote(printed)} /v {_quote(item[1])} /t {kind} /d {_quote(data)} /f"})
    for item, (printed, kind, data) in after["values"].items():
        if item not in before["values"] and item[0] not in added:
            if item[1] == "(Default)" or not REG_VALUE_NAME_PATTERN.match(item[1]):
                raise ValueError(f"{printed}\\{item[1]} cannot be deleted exactly")
            printed = printed_key(printed, item[0])
            plan.append({"value": f"{printed}\\{item[1]}", "action": "delete", "target": (printed, item[1]),
                         "restores": None,
                         "replaces": {"type": kind, "data": data},
                         "command": f"reg delete {_quote(printed)} /v {_quote(item[1])} /f"})
    for key_id in added:
        printed = printed_key(after["keys"][key_id], key_id)
        subtree = {item: value[1:] for item, value in after["values"].items()
                   if item[0] == key_id or item[0].startswith(key_id + "\\")}
        plan.append({"key": printed, "action": "delete_key", "subtree": subtree, "restores": None,
                     "replaces": "present",
                     "command": f"reg delete {_quote(printed)} /f"})
    return plan


class _RestoreStopped(Exception):
    """A check inside the restore found another instance of the tool, or could not tell."""

    def __init__(self, check: dict[str, Any]):
        super().__init__(check["status"])
        self.check = check


def _restore_recent_files(config: HarnessConfig, step: Callable[[str, str], dict[str, Any]],
                          write: Callable[[str, str], dict[str, Any]], settings_before, plan: list[dict[str, Any]],
                          state: dict[str, Any]) -> tuple[dict[str, Any], BaseException | None]:
    """Write the restore scope (`_restore_plan`) back to the first reading, as one routine with one failure path.

    Steps: the precondition before each write; the write, logged in `state["write_log"]` (value, action, the value
    it restores and the one it replaces, attempts, outcome) before it is issued, and issued exactly once through
    `write`; the precondition after the last write; the third reading and its exact comparison with the first; the
    precondition once more before success is declared. `recent_files_restored` only when every step completed and
    the comparison found no difference. Any other outcome takes the one failure path: a check that is not clear
    (no exception), or any exception or cancellation at any step. The first exception is kept in
    `state["primary_error"]` from the moment it is caught, so nothing later can replace it.
    """

    state["restore_entered"] = True
    log: list[dict[str, Any]] = state.setdefault("write_log", [])
    reason = None
    guard = config.settings_guard
    deadline = None  # set when the mutation section starts

    def within(name: str) -> None:
        if deadline is not None and time.monotonic() >= deadline:
            state["section_budget_exceeded"] = name
            raise SectionBudgetExceeded(f"the section budget of {guard.section_budget_s:g} s ran out before {name}")

    def budgeted_step(name: str, command: str) -> dict[str, Any]:
        within(name)
        return step(name, command, deadline=deadline)

    def budgeted_write(name: str, command: str) -> dict[str, Any]:
        within(name)
        return write(name, command, deadline=deadline)

    def check(name: str, caller: Callable[..., dict[str, Any]] | None = None) -> None:
        # Not `_precondition`: an exception here must reach the failure path unchanged, not become a finding.
        found = _tool_processes(config, caller or step, name)
        if found is None:
            raise _RestoreStopped({"status": "precondition_unknown",
                                   "reason": "the guest's process list could not be read or is malformed"})
        if found:
            raise _RestoreStopped({"status": "foreign_instance_running", "foreign_pids": found,
                                   "reason": "an instance of the tool this run did not start is running"})

    def settled_write(number: int, entry: dict[str, Any], logged: dict[str, Any]) -> None:
        """Issue one write. A value set, value delete or subkey delete whose outcome the transport left unknown
        (exit 255, timeout) is settled before anything is issued again: the foreign-instance check, then the value
        or subkey read back. Already as intended (the value set to its target; the value or subkey absent):
        completed. Still as the run left it (the value it replaces; for a subkey, every value in it as read after
        the run): issued once more (attempt 2). Anything else (another value or content, an unreadable read-back,
        another instance): the first error stops the restore. At most two attempts; every attempt and the
        read-back are logged. Creating a subkey is not retried."""

        name = f"settings_restore_{number}"
        try:
            budgeted_write(name, entry["command"])
            return
        except BaseException as error:
            if entry["action"] not in ("set", "delete", "delete_key") or not _transport_failure(error):
                raise
            first = error
        if state.get("primary_error") is first:  # a timeout the write recorded: not yet the restore's error
            del state["primary_error"]
        logged["attempt_log"] = [{"attempt": 1, "outcome": "unknown", "error": f"{type(first).__name__}: {first}"}]
        check(f"restore_check_before_retry_{number}", budgeted_step)
        within(f"settings_readback_{number}")
        if entry["action"] == "delete_key":
            state_found, found = _read_subtree(budgeted_step, f"settings_readback_{number}", entry["key"])
            intended = state_found == "absent"
            as_left = state_found == "present" and found == entry["subtree"]
        else:
            state_found, found = _read_value(budgeted_step, f"settings_readback_{number}", *entry["target"])
            target = (entry["restores"]["type"], entry["restores"]["data"]) if entry["restores"] else None
            left = (entry["replaces"]["type"], entry["replaces"]["data"]) if entry["replaces"] else None
            intended = (state_found == "present" and found == target) or (state_found == "absent" and target is None)
            as_left = (state_found == "present" and found == left) or (state_found == "absent" and left is None)
        if intended:
            logged["readback"] = "target"
            logged["attempt_log"][0]["outcome"] = "completed (read back)"
            return
        if not as_left:
            logged["readback"] = "unreadable" if state_found == "unreadable" else "third_value"
            raise first
        logged["readback"] = "unchanged"
        logged["attempts"] = 2
        logged["attempt_log"].append({"attempt": 2, "outcome": "issued"})
        try:
            budgeted_write(f"{name}_attempt_2", entry["command"])
        except BaseException as error:
            logged["attempt_log"][1]["outcome"] = _write_outcome(error)
            raise
        logged["attempt_log"][1]["outcome"] = "completed"

    try:
        # The registry-mutation section: the checks, the writes, the check after them and the verification
        # reading, with signals deferred. Its bound is the sum of these guest calls' timeouts.
        signals = None
        try:
            with _deferred_signals(True) as signals:
                deadline = time.monotonic() + guard.section_budget_s
                state["section_budget_s"] = guard.section_budget_s
                for number, entry in enumerate(plan, 1):
                    check(f"restore_check_before_write_{number}", budgeted_step)
                    logged = {key: entry[key] for key in ("value", "key", "action", "restores", "replaces")
                              if key in entry} | {
                        "attempts": 1, "outcome": "issued", "completed": False}
                    log.append(logged)
                    try:
                        settled_write(number, entry, logged)
                    except _RestoreStopped:
                        logged["outcome"] = "unknown"  # the write's outcome was never settled
                        raise
                    except BaseException as error:
                        state.setdefault("primary_error", error)
                        logged["outcome"] = _write_outcome(error)
                        raise
                    logged["outcome"], logged["completed"] = "completed", True
                check("restore_check_after_writes", budgeted_step)
                within("settings_verify")
                third = _settings_snapshot(config, budgeted_step, "settings_verify")
        finally:
            # The section has closed (handlers restored): what it deferred is reported now, before any record.
            if signals is not None:
                state["signal_deferral"] = signals.deferral
                if signals.received:
                    state["deferred_signals"] = [signal.Signals(number).name for number in signals.received]
        if third is None:
            reason = "the key could not be read after the restore"
        else:
            remaining = settings_difference(settings_before, third)
            if remaining:
                reason = "the reading after the restore does not equal the reading before the run"
                state["remaining_differences"] = remaining
        if reason is None:
            check("restore_check_before_success")
            done = {"status": "settings_restored" if guard.restores_whole_key else "recent_files_restored",
                    "write_log": log, "restored_values": len(plan)}
            for key in ("signal_deferral", "deferred_signals"):
                if key in state:
                    done[key] = state[key]
            return done, None
    except _RestoreStopped as stopped:
        reason = f"{stopped.check['status']}: {stopped.check.get('reason', '')}"
        state["stopped_by"] = stopped.check
    except BaseException as caught:  # cancellation included
        state.setdefault("primary_error", caught)
        reason = f"the restore was interrupted ({type(caught).__name__}: {caught})"
    state["reason"] = reason
    return _failed_restore(config, step, settings_before, state)


def _restore_log_summary(result: dict[str, Any], state: dict[str, Any]) -> None:
    log = state.get("write_log", [])
    result["write_log"] = log
    result["writes_completed"] = sum(1 for entry in log if entry["completed"])
    for key in ("stopped_by", "remaining_differences", "signal_deferral", "deferred_signals", "section_budget_s",
                "section_budget_exceeded"):
        if key in state:
            result[key] = state[key]


def _restore_reason(result: dict[str, Any], state: dict[str, Any]) -> None:
    log = state.get("write_log", [])
    done = sum(1 for entry in log if entry["completed"])
    reason = state.get("reason") or "the restore did not complete"
    result["reason"] = f"{reason}; {done} of {len(log)} attempted writes completed" if log else \
        f"{reason}; nothing was written"


def _failed_restore(config: HarnessConfig, step: Callable[[str, str], dict[str, Any]], settings_before,
                    state: dict[str, Any]) -> tuple[dict[str, Any], BaseException | None]:
    """The restore's one failure path. `settings_restore_failed` and the write log are in `state["result"]` before
    anything else runs;
    the bookkeeping, the final reading, its comparison and the reason then run each under its own guard, so an
    exception or cancellation in one is recorded as secondary and neither skips the others nor replaces the primary
    error. Returns the result and the error to raise: the primary one, else the first secondary one."""

    result = state.setdefault("result", {})
    result["status"] = "settings_restore_failed"
    result["write_log"] = state.setdefault("write_log", [])  # what was written, before anything else can fail
    secondary: list[BaseException] = state.setdefault("secondary_errors", [])

    def guarded(work: Callable[[], None]) -> None:
        try:
            work()
        except BaseException as caught:
            secondary.append(caught)

    def final_reading() -> None:
        result["final_reading"] = "interrupted"
        state["final"] = _settings_snapshot(config, step, "settings_after_failed_restore")
        result["final_reading"] = "read" if state["final"] is not None else "could not be read"

    def final_difference() -> None:
        if state.get("final") is not None:
            result["remaining_differences"] = settings_difference(settings_before, state["final"])

    guarded(lambda: _restore_log_summary(result, state))
    guarded(final_reading)
    guarded(final_difference)
    guarded(lambda: _restore_reason(result, state))
    if secondary:
        result["secondary_errors"] = [f"{type(error).__name__}: {error}" for error in secondary]
    primary = state.get("primary_error")
    return result, primary if primary is not None else (secondary[0] if secondary else None)


def _attach_guard(error: BaseException, settings: dict[str, Any]) -> None:
    error.settings_guard = settings


def _persist_guard_result(root: Path, out: Path, settings: dict[str, Any]) -> BaseException | None:
    """Write the guard result, with its write log, to `GUARD_RESULT_FILE` before the run record (interruptible).

    A cancellation during the write is retried once, so a termination request arriving then does not lose the
    log; it is returned for the caller to raise (or keep beside a primary error). An I/O error is not retried; it
    is named in the result and also returned.
    """

    interrupted: BaseException | None = None
    for _ in range(2):
        try:
            write_private(root, out / GUARD_RESULT_FILE,
                          json.dumps(settings, indent=2, sort_keys=True, default=str) + "\n")
            if interrupted is not None:
                settings["guard_result_file"] = f"written after {type(interrupted).__name__}"
            return interrupted
        except BaseException as caught:
            interrupted = interrupted or caught
            if isinstance(caught, Exception):
                break
    settings["guard_result_file"] = f"not written: {type(interrupted).__name__}: {interrupted}"
    return interrupted


def _live_guard_result(live: dict[str, Any]) -> dict[str, Any]:
    """The guard result as it stands, without guest or file I/O: the round trip's own result once it has one; else
    the restore's failure-path result; else one built from the write log (`settings_restore_failed` when the
    restore was entered, else `settings_check_failed` with nothing written). The write log is always included."""

    state = live.get("guard_state") or {}
    log = state.get("write_log") or []
    result = live.get("settings")
    if result is None:
        result = state.get("result")
    if result is None:
        done = sum(1 for entry in log if entry["completed"])
        if log or state.get("restore_entered"):
            result = {"status": "settings_restore_failed",
                      "reason": f"interrupted before the guard result was complete; {done} of {len(log)} attempted "
                                "writes completed"}
        else:
            result = {"status": "settings_check_failed",
                      "reason": "interrupted before the guard result was complete; nothing was written"}
    if log or "write_log" in result or state.get("restore_entered"):
        result.setdefault("write_log", log)
        result.setdefault("writes_completed", sum(1 for entry in result["write_log"] if entry["completed"]))
    return result


def _recover_guard_result(error: BaseException, live: dict[str, Any], root: Path, out: Path) -> None:
    """For an exception leaving a guarded round trip before its reporting finished: attach the guard result, with
    its write log, to the exception before anything else, then write it to `GUARD_RESULT_FILE` once more, best
    effort and interruptible. A second cancellation during the write is kept beside the first, which is the one
    raised. After the reporting finished only a missing attachment is supplied."""

    def attach(result: dict[str, Any]) -> None:
        try:
            _attach_guard(error, result)
        except BaseException as caught:  # the error is raised all the same; the result names the failure
            _secondary(result, "attachment", caught)

    if live.get("reported"):
        if getattr(error, "settings_guard", None) is None:
            attach(_live_guard_result(live))
        return
    result = live.get("settings")
    if result is None:
        result = getattr(error, "settings_guard", None)
    if result is None:
        result = _live_guard_result(live)
    attach(result)
    result.setdefault("round_trip_error", f"{type(error).__name__}: {error}")
    again = _persist_guard_result(root, out, result)
    if again is not None and again is not error:
        _secondary(result, "guard result file", again)


def _attach_after_record_failure(error: BaseException, settings: dict[str, Any] | None) -> None:
    """A record failure that becomes the error (there was none before) still carries the guard result, with its
    write log, and the result names the failure."""
    if settings is not None:
        settings["run_record_error"] = f"{type(error).__name__}: {error}"
        try:
            _attach_guard(error, settings)
        except BaseException as caught:
            _secondary(settings, "attachment", caught)


def _guard_check(config: HarnessConfig, step: Callable[[str, str], dict[str, Any]], settings_before, after,
                 processes, state: dict[str, Any] | None = None,
                 write: Callable[[str, str], dict[str, Any]] | None = None) -> tuple[dict[str, Any], BaseException | None]:
    """The run record's `settings_guard`, and an exception (cancellation included) deferred from the restore.

    `unchanged` when the whole snapshot is equal and no instance of the tool is left. A difference confined to the
    values of the recent-files subkey (or, with `restore_scope` `"key"`, any difference strictly inside the
    configured key) is written back only when the configuration enables it and the process
    checks are clean (no instance of ours or anyone else's running), through `_restore_recent_files`; `state`
    carries its write log, so a caller that is itself interrupted can still report what was written. Any other
    difference is `settings_changed` and nothing is written.
    """

    state = {} if state is None else state
    guard = config.settings_guard
    if settings_before is None:
        return {"status": "settings_check_failed", "reason": "the settings could not be read before the run; "
                                                             "nothing ran"}, None
    result: dict[str, Any] = {"owned_processes": processes}
    if after is None:
        result.update(status="settings_check_failed", reason="the settings could not be read after the run")
        return result, None
    changed = settings_difference(settings_before, after)
    result["changed_values"] = changed
    if processes["status"] == "running":
        result.update(status="owned_process_running", reason="a process the guest started is still running; "
                                                             "nothing was written")
        return result, None
    if processes["status"] == "unknown":
        result.update(status="owned_process_state_unknown", reason=processes["reason"])
        return result, None
    if processes["status"] == "foreign":
        result.update(status="foreign_instance_running", reason="an instance of the tool this run did not start "
                                                                "appeared; nothing was written")
        return result, None
    if not changed:
        result["status"] = "unchanged"
        return result, None
    if guard.restores_whole_key:
        scope_id, enabled = _registry_key_id(guard.key), True
    else:
        scope_id = _registry_key_id(guard.recent_files_key) if guard.recent_files_key else None
        keys_equal = set(settings_before["keys"]) == set(after["keys"])
        outside = [item for item in set(settings_before["values"]) | set(after["values"])
                   if item[0] != scope_id
                   and settings_before["values"].get(item, (None,))[1:] != after["values"].get(item, (None,))[1:]]
        enabled = guard.restore_recent_files and scope_id and keys_equal and not outside
    if not enabled:
        result.update(status="settings_changed", reason="the saved settings differ after the run"
                      + ("" if guard.restore_recent_files else " (restoring is not enabled)") + "; nothing was written")
        return result, None
    try:
        plan = _restore_plan(settings_before, after, scope_id, whole_key=guard.restores_whole_key)
    except ValueError as error:
        result.update(status="settings_restore_failed", reason=f"{error}; nothing was written")
        return result, None
    state["restore_entered"] = True  # also set inside: recovery is attempted from here on, log or no log
    restored, failure = _restore_recent_files(config, step, write, settings_before, plan, state)
    result.update(restored)
    return result, failure


def restore_recorded_settings(config: HarnessConfig, changed_values: list[Mapping[str, Any]], *,
                              runner: Callable[..., subprocess.CompletedProcess] = subprocess.run) -> dict[str, Any]:
    """Write back the value differences a round trip recorded (`changed_values` of its guard result) to their
    `before` values, through the round trip's own restore routine (`_restore_plan`, `_restore_recent_files`).

    Needs `restore_scope` `"key"`. Nothing is written unless: no instance of the tool is running; the key reads
    whole; every recorded entry is a value (not a subkey) inside the key; and every recorded value still has its
    recorded `after` type and data (absent where `after` is None). Otherwise the status says why
    (`changed_since_record` lists the values that differ). The intended reading is the current one with each
    recorded value at its `before`; the routine checks before each write and after the last, reads a third time
    and compares exactly. Each recorded value's status is listed in `values`. An exception or cancellation from
    the routine is raised with the result attached.
    """

    guard = config.settings_guard
    if guard is None or not guard.restores_whole_key:
        return {"status": "not_enabled", "reason": "restoring recorded values needs restore_scope \"key\""}
    steps: list[dict[str, Any]] = []
    state: dict[str, Any] = {}

    def step(name: str, command: str, deadline: float | None = None) -> dict[str, Any]:
        return _guest_read(config, runner, guard.call_timeout_s, steps, name, command, deadline)

    def write(name: str, command: str, deadline: float | None = None) -> dict[str, Any]:
        return _guest_write(config, runner, guard.call_timeout_s, steps, name, command, state, deadline)

    def finish(result: dict[str, Any]) -> dict[str, Any]:
        result["steps"] = [{key: value for key, value in item.items() if key not in ("stdout", "stderr")}
                           for item in steps]
        return result

    live: dict[str, Any] = {"guard_state": state}
    with _catchable_termination(True, live):
        pre = _precondition(config, step, "precondition_process_list")
        if pre["status"] != "clear":
            return finish(pre | {"write_log": []})
        current = _settings_snapshot(config, step, "settings_current")
        if current is None:
            return finish({"status": "settings_check_failed", "reason": "the key could not be read; nothing was "
                                                                        "written", "write_log": []})
        root = _registry_key_id(guard.key)
        printed_keys = sorted(current["keys"].values(), key=len, reverse=True)
        intended = {"keys": dict(current["keys"]), "values": dict(current["values"])}
        located, differs = [], []
        for entry in changed_values:
            value = entry.get("value") if isinstance(entry, Mapping) else None
            printed = next((key for key in printed_keys if isinstance(value, str) and value.startswith(key + "\\")),
                           None)
            if printed is None or not (_registry_key_id(printed) == root
                                       or _registry_key_id(printed).startswith(root + "\\")):
                return finish({"status": "not_restorable", "reason": f"a recorded entry is not a value inside the "
                                                                     f"key: {value or entry}", "write_log": []})
            item = (_registry_key_id(printed), value[len(printed) + 1:])
            found = current["values"].get(item)
            after, before = entry.get("after"), entry.get("before")
            if (None if found is None else {"type": found[1], "data": found[2]}) != after:
                differs.append(value)
            if before is None:
                intended["values"].pop(item, None)
            else:
                intended["values"][item] = (printed, before["type"], before["data"])
            located.append((value, item))
        if differs:
            return finish({"status": "changed_since_record", "reason": "these recorded values no longer have their "
                                                                       "recorded after-state; nothing was written",
                           "values": [{"value": value, "status": "changed_since_record"} for value in differs],
                           "write_log": []})
        try:
            plan = _restore_plan(intended, current, root, whole_key=True)
        except ValueError as error:
            return finish({"status": "settings_restore_failed", "reason": f"{error}; nothing was written",
                           "write_log": []})
        state["restore_entered"] = True
        result, failure = _restore_recent_files(config, step, write, intended, plan, state)
        written = {entry["value"]: entry["outcome"] for entry in result.get("write_log", []) if "value" in entry}
        verified = result["status"] == "settings_restored"
        result["values"] = [{"value": value, "status": "restored_and_verified" if verified else
                             written.get(value, "not_written")} for value, _ in located]
        finish(result)
        if failure is not None:
            _attach_guard(failure, result)
            raise failure
        return result


def _round_trip(config: HarnessConfig, case_id: str, mbf_text: str, paths: dict[str, Path], harness_dir: Path,
                timeout_s: float, runner: Callable[..., subprocess.CompletedProcess],
                gui_fallback: bool = True) -> dict[str, Any]:
    # A guarded round trip turns SIGTERM into a catchable cancellation for its whole lifetime, so a termination
    # request is recorded like a Ctrl-C instead of ending the process with the guard's log unwritten.
    # The same holds for an exception at any reporting or persistence step: it leaves with the guard result attached
    # and written once more (`_recover_guard_result`).
    guarded = config.settings_guard is not None
    live: dict[str, Any] = {}
    with _catchable_termination(guarded, live):
        try:
            return _round_trip_body(config, case_id, mbf_text, paths, harness_dir, timeout_s, runner, gui_fallback,
                                    live)
        except BaseException as error:
            if guarded and live.get("guard_state") is not None:
                _recover_guard_result(error, live, config.exchange_root, paths["caepipe_out"])
            raise


def _round_trip_body(config: HarnessConfig, case_id: str, mbf_text: str, paths: dict[str, Path], harness_dir: Path,
                     timeout_s: float, runner: Callable[..., subprocess.CompletedProcess],
                     gui_fallback: bool = True, live: dict[str, Any] | None = None) -> dict[str, Any]:
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
    guarded = config.settings_guard is not None

    def solve_command(script: str) -> str:
        return (f"powershell -NoProfile -ExecutionPolicy Bypass -File {_quote(script)} "
                f"-Exe {_quote(config.guest_executable)} -Mbf {_quote(guest_mbf)}"
                + (f" -PidFile {_quote(_win(work, OWNED_PIDS_FILE))}" if guarded else "")
                + ("" if gui_fallback else " -NoGuiFallback"))

    steps: list[dict[str, Any]] = []

    def step(name: str, command: str) -> dict[str, Any]:
        return _guest_step(config, runner, timeout_s, steps, name, command)

    guard_state: dict[str, Any] = {}
    live = {} if live is None else live
    live["guard_state"] = guard_state  # what the outer recovery and the termination handler report from
    # Every guard call (registry reads and writes, process queries) has the guard's own short timeout; the solve
    # keeps `timeout_s`.
    guard_timeout = config.settings_guard.call_timeout_s if config.settings_guard else timeout_s

    def guard_step(name: str, command: str, deadline: float | None = None) -> dict[str, Any]:
        return _guest_read(config, runner, guard_timeout, steps, name, command, deadline)

    def write(name: str, command: str, deadline: float | None = None) -> dict[str, Any]:
        return _guest_write(config, runner, guard_timeout, steps, name, command, guard_state, deadline)

    # The guest work copy of the MBF is deleted before the copy, so a failed copy can never leave an older MBF
    # to be solved; a failed prepare step stops the round trip before solving.
    stale_guest = " ".join(_quote(_win(work, name)) for name in ("model*.csv", "model.mbf", PRINT_TXT_NAME,
                                                                  *OUTPUT_NAMES, OWNED_PIDS_FILE))
    # The saved settings are read before anything starts the tool; without that reading nothing runs.
    # Precondition: no instance of the tool may be running before ours starts; then the saved settings are read.
    # Without both, nothing runs.
    blocked = None
    if guarded:
        pre = _precondition(config, guard_step, "precondition_process_list")
        blocked = None if pre["status"] == "clear" else pre
    settings_before = _settings_snapshot(config, guard_step, "settings_before") if guarded and blocked is None else None
    prepare: dict[str, Any] = {"exit_status": "not_run", "stdout": "", "stderr": ""}
    solve: dict[str, Any] = {"exit_status": None, "stdout": "", "stderr": ""}
    script_location = "exchange_share"
    guest_log = None
    guest_mbf_sha256 = None
    input_verified = False
    settings = None
    error: BaseException | None = None
    try:
        if not guarded or (blocked is None and settings_before is not None):
            prepare = step("prepare", f"{_ensure_dir(work)} & del /Q {stale_guest} 2>nul & "
                                      f"copy /Y {_quote(_win(guest_in, 'model.mbf'))} {_quote(guest_mbf)}")
        if prepare["exit_status"] == 0:
            solve = step("solve", solve_command(share_script))
            guest_log = last_json_object(solve["stdout"])
            if guest_log is None and solve["exit_status"] != "timeout":
                script_location = "guest_work_dir"
                step("stage_script", f"copy /Y {_quote(share_script)} {_quote(work_script)}")
                solve = step("solve_from_work_dir", solve_command(work_script))
                guest_log = last_json_object(solve["stdout"])
        # The guest script reports the SHA-256 of the MBF it actually solved; outputs are attributed to this MBF
        # only when that hash equals the host's.
        guest_mbf_sha256 = guest_log.get("mbf_sha256") if isinstance(guest_log, dict) else None
        guest_mbf_sha256 = guest_mbf_sha256.lower() if isinstance(guest_mbf_sha256, str) else None
        input_verified = guest_mbf_sha256 == mbf_sha256
        if guest_log is not None and input_verified:
            _refuse_links(root, out)  # again just before the guest writes into the layer
            step("collect", " & ".join([
                f"copy /Y {_quote(_win(work, 'model*.csv'))} {_quote(guest_out)}",
                f"copy /Y {_quote(_win(work, PRINT_TXT_NAME))} {_quote(_win(guest_out, PRINT_TXT_NAME))}",
                *(f"copy /Y {_quote(_win(work, name))} {_quote(_win(guest_out, name))}" for name in OUTPUT_NAMES),
            ]))
    except BaseException as caught:  # re-raised below, after the checks
        error = caught
    # The guard's post-run work runs in normal signal context: only the restore's registry-mutation section defers
    # signals. Callee exceptions here are kept (the first one raised), and the guard result is attached to the
    # error before any record I/O, so a signal or a failure during the record cannot lose it.
    # Whatever happened above: confirm that what the guest started has ended, then compare the whole settings
    # key with its reading before the run. Each check is guarded on its own, so a failure of one never skips the
    # other or the result.
    if guarded:
        deferred = None
        post_run_errors: list[BaseException] = []
        try:
            try:
                processes = (_owned_processes(config, guard_step, work, guest_log)
                             if prepare["exit_status"] == 0 else {"status": "not_started"})
            except BaseException as failure:  # a callee's cancellation included: the reading still runs
                if not isinstance(failure, Exception):
                    post_run_errors.append(failure)  # a cancellation is kept and raised; an error stops as unknown
                processes = _unknown(f"the process check failed: {type(failure).__name__}: {failure}")
            after = None
            if settings_before is not None:
                try:
                    after = _settings_snapshot(config, guard_step, "settings_after")
                except BaseException as failure:
                    if not isinstance(failure, Exception):
                        post_run_errors.append(failure)
                    processes = processes | {"settings_read_error": f"{type(failure).__name__}: {failure}"}
            if blocked is not None:
                settings = blocked
            else:
                settings, deferred = _guard_check(config, guard_step, settings_before, after, processes, guard_state,
                                                  write)
        except BaseException as caught:  # cancellation during the checks or escaping the restore's failure path
            primary = guard_state.get("primary_error")
            deferred = primary if primary is not None else caught
            log = guard_state.get("write_log") or []
            settings = dict(guard_state.get("result") or {})
            if log or settings or guard_state.get("restore_entered"):
                settings.setdefault("write_log", log)
                settings.setdefault("writes_completed", sum(1 for entry in log if entry["completed"]))
                settings["status"] = "settings_restore_failed"
                settings.setdefault("reason", f"interrupted ({type(caught).__name__}) during the restore; "
                                              f"{settings['writes_completed']} of {len(log)} attempted writes "
                                              "completed")
                if "final_reading" not in settings:
                    try:
                        final = _settings_snapshot(config, guard_step, "settings_after_failed_restore")
                        settings["final_reading"] = "read" if final is not None else "could not be read"
                    except BaseException as late:
                        settings["final_reading"] = f"interrupted ({type(late).__name__})"
            else:
                settings = {"status": "settings_check_failed",
                            "reason": f"the checks after the run were interrupted ({type(caught).__name__}); "
                                      "nothing was written"}
            if caught is not deferred:
                settings["secondary_errors"] = list(settings.get("secondary_errors", [])) + [
                    f"{type(caught).__name__}: {caught}"]
        live["settings"] = settings
        if deferred is not None:
            if error is None:
                error = deferred
            elif deferred is not error:
                settings["guard_error"] = f"{type(deferred).__name__}: {deferred}"
        for failure in post_run_errors:  # the first error stays primary; later ones are kept beside it
            if error is None:
                error = failure
            elif failure is not error:
                _secondary(settings, "post-run check", failure)
        # A signal the restore's mutation section deferred stops the run now, after the restore is recorded in memory.
        if guard_state.get("deferred_signals"):
            interrupt = KeyboardInterrupt(f"deferred signal {guard_state['deferred_signals'][0]}")
            if error is None:
                error = interrupt
            else:
                _secondary(settings, "deferred signal", interrupt)
    # The guard result is attached to the error before any record I/O, so no record failure can lose it, and then
    # persisted on its own (with its write log) before the run record, so a cancellation during the record does not
    # lose it either.
    if error is not None and settings is not None:
        try:
            _attach_guard(error, settings)
        except BaseException as caught:
            _secondary(settings, "attachment", caught)
    if guarded and settings is not None:
        interrupted = _persist_guard_result(root, out, settings)
        if interrupted is not None:
            if error is None:
                error = interrupted
                try:
                    _attach_guard(error, settings)
                except BaseException as caught:
                    _secondary(settings, "attachment", caught)
            elif interrupted is not error:
                _secondary(settings, "guard result file", interrupted)

    try:
        write_private(root, out / "guest_stdout.txt", solve["stdout"])
        write_private(root, out / "guest_stderr.txt", solve["stderr"])
        if guest_log is not None:
            write_private(root, out / "guest_log.json", json.dumps(guest_log, indent=2, sort_keys=True) + "\n")

        guest_csv = guest_log.get("csv_path") if isinstance(guest_log, dict) else None
        observed = None
        csv_name = guest_csv.replace("/", "\\").split("\\")[-1] if isinstance(guest_csv, str) else ""
        print_problem = None
        if csv_name not in ("", ".", "..") and input_verified and error is None:
            candidate = contained(root, out / csv_name)
            if candidate.is_file():
                observed = candidate
                if csv_name.lower() == PRINT_TXT_NAME:
                    print_problem = _print_layout_problem(candidate)
                    observed = None if print_problem else observed
        status = guest_log.get("status") if isinstance(guest_log, dict) else None
        if error is not None:
            route = "harness_error"
        elif blocked is not None:
            route = blocked["status"]
        elif guarded and settings_before is None:
            route = "settings_check_failed"
        elif prepare["exit_status"] != 0:
            route = "prepare_failed"
        elif solve["exit_status"] == "timeout":
            route = "timeout"
        elif guest_log is None:
            route = "no_guest_json"
        elif not input_verified:
            route = "input_hash_mismatch"
        elif print_problem is not None:
            route = "print_file_unrecognised"
        elif status in CSV_ROUTES and observed is not None:
            route = status
        elif status in CSV_ROUTES:
            route = "csv_not_collected"
        else:
            route = str(status or "unknown_guest_status")
        gui_used = bool(isinstance(guest_log, dict) and isinstance(guest_log.get("gui"), dict)
                        and guest_log["gui"].get("used"))

        collected = [path.name for path in out.iterdir()
                     if path.name.startswith("model") and path.is_file() and not path.is_symlink()]
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
                    "allowed": gui_fallback,
                    "used": gui_used,
                    "label": "print_to_file_fallback",
                    "description": "fallback when the batch route yields no result file: the guest script stops "
                                   "the batch process it started, opens the solved .res in a CAEPIPE instance it "
                                   "starts itself and prints the results to CSV via Print > To File",
                },
                "host_invocation": "prlctl exec <vm_name> --current-user cmd /c <guest command>",
            },
            "steps": [{key: value for key, value in item.items() if key not in ("stdout", "stderr")}
                      | {"stdout_chars": len(item.get("stdout") or ""), "stderr_chars": len(item.get("stderr") or "")}
                  for item in steps],
            "working_directory": work,
            "expected_batch_csv_path": _win(work, "model.csv"),
            "observed_csv_path": {"guest": guest_csv, "host": f"caepipe_out/{observed.name}" if observed else None},
            "print_file": {"collected": observed.name if observed else None,
                           "layout": ("print_csv" if observed is not None and csv_name.lower() == PRINT_TXT_NAME
                                      else print_problem or ("as_named" if observed is not None else None))},
            "route": route,
            "exit_status": solve["exit_status"],
            "stdout_file": "caepipe_out/guest_stdout.txt",
            "stderr_file": "caepipe_out/guest_stderr.txt",
            "guest_log": guest_log,
            "outputs_collected": sorted(collected),
            "outputs_sha256": {name: _sha256((out / name).read_bytes()) for name in sorted(collected)},
            "timestamps": {"started": started, "finished": _now()},
            "boundary_note": BOUNDARY_NOTE,
        }
        if error is not None:
            record["harness_error"] = f"{type(error).__name__}: {error}"
        if settings is not None:
            record["settings_guard"] = settings
    except BaseException as caught:
        record = None
        if error is None:
            error = caught
            _attach_after_record_failure(error, settings)
        else:
            _secondary(settings, "run record", caught)
    if record is not None:
        try:
            write_private(root, out / "run_record.json", json.dumps(record, indent=2, sort_keys=True) + "\n")
        except BaseException as caught:
            if error is None:
                error = caught
                _attach_after_record_failure(error, settings)
            _secondary(settings, "run record", caught)
    live["reported"] = True  # the records are done: the outer recovery has nothing left to write
    if error is not None:
        raise error
    return record


def stored_run_record(root: Path, out: Path) -> dict[str, Any] | None:
    """The harness run record stored in caepipe_out/, or None when absent or not a run record."""

    path = contained(root, out / "run_record.json")
    try:
        record = json.loads(path.read_text(encoding="utf-8")) if path.is_file() else None
    except (OSError, UnicodeDecodeError, json.JSONDecodeError):
        return None
    return record if isinstance(record, dict) and record.get("document_kind") == RUN_RECORD_KIND else None


def solved_model_binding(record: Mapping[str, Any] | None, mbf_sha256: str, mod: Path) -> str:
    """Why the stored `model.mod` is not the solve of the MBF with `mbf_sha256`, or "" when the run record binds it."""

    if record is None:
        return "no valid caepipe_out/run_record.json binds a solved model to this MBF"
    if (record.get("mbf") or {}).get("sha256") != mbf_sha256 or record.get("guest_mbf_sha256") != mbf_sha256:
        return "the stored solve is not of the current MBF (host or guest MBF hash differs)"
    if "model.mod" not in (record.get("outputs_collected") or []) or not mod.is_file():
        return "the run record names no collected caepipe_out/model.mod"
    hashes = record.get("outputs_sha256")
    # Records written before outputs were hashed bind model.mod by name only.
    if isinstance(hashes, dict) and hashes.get("model.mod") != _sha256(mod.read_bytes()):
        return "caepipe_out/model.mod differs from the solved model the run record hashed"
    return ""


def run_caepipe_mbf_export(config: HarnessConfig, case_id: str, mbf_sha256: str, *, timeout_s: float = 600,
                           runner: Callable[..., subprocess.CompletedProcess] = subprocess.run) -> dict[str, Any]:
    """Export one case's stored solved model back to MBF with the user's CAEPIPE (Design §3, §5).

    The stored solve must be bound to the MBF with `mbf_sha256` by its run record; otherwise the route is
    `not_bound`, nothing is run, any earlier export outputs are removed and the run record's export entry
    states why. The guest opens a copy of `caepipe_out/model.mod` in a CAEPIPE
    instance it starts, exports it with File > Export to MBF... and stops that instance. The harness collects
    `caepipe_out/exported.mbf` only when the guest reports opening a model whose SHA-256 equals the stored one,
    keeps it only when its SHA-256 equals the one the guest reported, and records the export in the run record's
    `mbf_export` entry, which binds it to the solve by both hashes.
    """

    return _locked(config, case_id, lambda paths, harness_dir: _mbf_export(
        config, case_id, mbf_sha256, paths, harness_dir, timeout_s, runner))


def _mbf_export(config: HarnessConfig, case_id: str, mbf_sha256: str, paths: dict[str, Path], harness_dir: Path,
                timeout_s: float, runner: Callable[..., subprocess.CompletedProcess]) -> dict[str, Any]:
    started = _now()
    root = config.exchange_root
    out = paths["caepipe_out"]
    _refuse_links(root, out)
    run_record = stored_run_record(root, out)
    mod = contained(root, out / "model.mod")
    reason = solved_model_binding(run_record, mbf_sha256, mod)
    if reason:
        # An earlier export is no longer bound to the current MBF: it is removed, and the run record's export
        # entry says why, so no consumer can read a stale file as the current model's export.
        for entry in out.iterdir():
            if entry.name.lower() in EXPORT_OUTPUTS and entry.is_file():
                contained(root, entry).unlink()
        refused = {"route": "not_bound", "reason": reason, "solved_mbf_sha256": mbf_sha256,
                   "timestamps": {"started": started, "finished": _now()}}
        if run_record is not None:
            write_private(root, out / "run_record.json",
                          json.dumps({**run_record, "mbf_export": refused}, indent=2, sort_keys=True) + "\n")
        return {"route": "not_bound", "reason": reason}
    mod_sha256 = _sha256(mod.read_bytes())
    for entry in out.iterdir():
        if entry.name.lower() in EXPORT_OUTPUTS and entry.is_file():
            contained(root, entry).unlink()
    script_bytes = GUEST_SCRIPT.read_bytes()
    write_private(root, harness_dir / GUEST_SCRIPT.name, script_bytes)

    guest_out = _win(config.guest_exchange_root, "cases", case_id, "caepipe_out")
    work = _win(config.guest_work_root, case_id, "export")
    guest_mod, guest_mbf = _win(work, "model.mod"), _win(work, EXPORTED_MBF)
    share_script = _win(config.guest_exchange_root, HARNESS_DIR, GUEST_SCRIPT.name)
    work_script = _win(work, GUEST_SCRIPT.name)

    def export_command(script: str) -> str:
        return (f"powershell -NoProfile -ExecutionPolicy Bypass -File {_quote(script)} "
                f"-Exe {_quote(config.guest_executable)} -ExportMod {_quote(guest_mod)} -ExportName {EXPORTED_MBF}")

    steps: list[dict[str, Any]] = []

    def step(name: str, command: str) -> dict[str, Any]:
        return _guest_step(config, runner, timeout_s, steps, name, command)

    # As for the solve: the guest copies are deleted first, so a failed copy can never leave an older model open.
    prepare = step("export_prepare", f"{_ensure_dir(work)} & del /Q {_quote(guest_mod)} "
                                     f"{_quote(guest_mbf)} 2>nul & copy /Y {_quote(_win(guest_out, 'model.mod'))} "
                                     f"{_quote(guest_mod)}")
    script_location = "exchange_share"
    export: dict[str, Any] = {"exit_status": None, "stdout": "", "stderr": ""}
    guest_log = None
    if prepare["exit_status"] == 0:
        export = step("export", export_command(share_script))
        guest_log = last_json_object(export["stdout"])
        if guest_log is None and export["exit_status"] != "timeout":
            script_location = "guest_work_dir"
            step("export_stage_script", f"copy /Y {_quote(share_script)} {_quote(work_script)}")
            export = step("export_from_work_dir", export_command(work_script))
            guest_log = last_json_object(export["stdout"])

    def guest_hash(key: str) -> str | None:
        value = guest_log.get(key) if isinstance(guest_log, dict) else None
        return value.lower() if isinstance(value, str) else None

    guest_mod_sha256, guest_exported_sha256 = guest_hash("mod_sha256"), guest_hash("exported_mbf_sha256")
    mod_verified = guest_mod_sha256 == mod_sha256
    status = guest_log.get("status") if isinstance(guest_log, dict) else None
    if status == EXPORT_ROUTE and mod_verified:
        _refuse_links(root, out)  # again just before the guest writes into the layer
        step("export_collect", f"copy /Y {_quote(guest_mbf)} {_quote(_win(guest_out, EXPORTED_MBF))}")

    write_private(root, out / "export_guest_stdout.txt", export["stdout"])
    write_private(root, out / "export_guest_stderr.txt", export["stderr"])
    if guest_log is not None:
        write_private(root, out / "export_guest_log.json", json.dumps(guest_log, indent=2, sort_keys=True) + "\n")

    exported = contained(root, out / EXPORTED_MBF)
    exported_sha256 = exported_bytes = None
    if exported.is_file() and not exported.is_symlink():
        data = exported.read_bytes()
        exported_sha256, exported_bytes = _sha256(data), len(data)
    if prepare["exit_status"] != 0:
        route = "prepare_failed"
    elif export["exit_status"] == "timeout":
        route = "timeout"
    elif guest_log is None:
        route = "no_guest_json"
    elif not mod_verified:
        route = "input_hash_mismatch"
    elif status != EXPORT_ROUTE:
        route = str(status or "unknown_guest_status")
    elif exported_sha256 is None:
        route = "mbf_not_collected"
    elif exported_sha256 != guest_exported_sha256:
        route = "export_hash_mismatch"
    else:
        route = EXPORT_ROUTE
    if route != EXPORT_ROUTE and exported.is_file():
        exported.unlink()  # never left where the import regression would read an unbound file
        exported_sha256 = exported_bytes = None

    entry = {
        "route": route,
        "solved_mbf_sha256": mbf_sha256,
        "mod": {"path": "caepipe_out/model.mod", "guest_path": guest_mod, "sha256": mod_sha256},
        "guest_mod_sha256": guest_mod_sha256,
        "mod_hash_verified": mod_verified,
        "exported_mbf": {"path": f"caepipe_out/{EXPORTED_MBF}" if exported_sha256 else None,
                         "sha256": exported_sha256, "bytes": exported_bytes},
        "guest_exported_mbf_sha256": guest_exported_sha256,
        "guest_script": {"repository_path": GUEST_SCRIPT_REPOSITORY_PATH,
                         "exchange_copy": f"{HARNESS_DIR}/{GUEST_SCRIPT.name}",
                         "sha256": _sha256(script_bytes), "run_from": script_location},
        "command_shape": {
            "description": "the guest script opens a copy of the solved model in a CAEPIPE instance it starts "
                           "(File > Open...), writes it with File > Export to MBF... and stops that instance",
            "host_invocation": "prlctl exec <vm_name> --current-user cmd /c <guest command>",
        },
        "steps": [{key: value for key, value in item.items() if key not in ("stdout", "stderr")}
                  | {"stdout_chars": len(item["stdout"]), "stderr_chars": len(item["stderr"])} for item in steps],
        "working_directory": work,
        "exit_status": export["exit_status"],
        "stdout_file": "caepipe_out/export_guest_stdout.txt",
        "stderr_file": "caepipe_out/export_guest_stderr.txt",
        "guest_log": guest_log,
        "timestamps": {"started": started, "finished": _now()},
    }
    write_private(root, out / "run_record.json",
                  json.dumps({**run_record, "mbf_export": entry}, indent=2, sort_keys=True) + "\n")
    return entry


def _text(value: Any) -> str:
    if value is None:
        return ""
    if isinstance(value, bytes):
        return value.decode("utf-8", "replace")
    return str(value)
