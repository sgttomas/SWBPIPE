"""Live CAEPIPE round-trip harness for the correlation pilot (DEL-17-05).

The harness is disabled unless the user configures it: `SWBPIPE_CAEPIPE_EXCHANGE`
names a private exchange folder outside every Git work tree, holding a
`.gitignore` of `*` and a `harness_config.json` with the virtual machine name,
the guest paths of the user's own CAEPIPE executable, of the shared exchange
folder and of a guest-local work folder, and
`license_and_environment_responsibility_acknowledged: true`. Without that, every
live step skips with a stated reason (PH-004).

The harness never searches for, bundles or installs CAEPIPE, never stops a
process on the host other than the `prlctl` client it started itself (on its
own timeout), and runs one case at a time. A round trip starts the program only
through one guest operation per attempt token, which claims the token by exclusive
create before it starts anything; a reply that is lost or out of form is never
repeated but reconciled by read-only queries into not started, completed, failed
and ended, or unknown (`run_caepipe_round_trip`). The guest never writes into the
exchange folder: the host reads a completed attempt's files and stores them in
`caepipe_out/<token>/`. Its export step, which would write a stored solved model
back to MBF (`caepipe_out/exported.mbf`) for the DEL-17-04 import regression, is
held: the guest script refuses it. Everything it writes under
`caepipe_out/` is CAEPIPE output or derived from it and stays private.
Its records are private validation evidence (docs/PRD.md §22.5), never professional
acceptance or code compliance.

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
tool that could be changing its settings at the same time.
"""

from __future__ import annotations

import dataclasses
from dataclasses import dataclass, field
from datetime import datetime, timezone
import base64
import contextlib
import csv
import hashlib
import io
import json
import math
import os
from pathlib import Path
import re
import secrets
import shutil
import signal
import stat
import subprocess
import threading
import time
from types import SimpleNamespace
from typing import Any, Callable, Mapping

# Reads its directory scans stand for, for CI selection (tools/ci/python_test_reads.py). It lists only the
# CAEPIPE exchange folder's caepipe_out, outside the project.
CI_READS = ()


ENV_VAR = "SWBPIPE_CAEPIPE_EXCHANGE"
CONFIG_NAME = "harness_config.json"
ACKNOWLEDGEMENT_KEY = "license_and_environment_responsibility_acknowledged"
REQUIRED_KEYS = ("vm_name", "guest_executable", "guest_exchange_root", "guest_work_root")
CASE_ID_PATTERN = re.compile(r"^[a-z0-9][a-z0-9-]{0,62}\Z")
GUEST_SCRIPT = Path(__file__).resolve().parent / "guest" / "Invoke-CaepipeRoundTrip.ps1"
GUEST_SCRIPT_REPOSITORY_PATH = "core/handoff/caepipe_external/guest/Invoke-CaepipeRoundTrip.ps1"
# The read-only queries' script (`_read_command`): installed and called like the guest script, by a short -File command.
READ_SCRIPT = Path(__file__).resolve().parent / "guest" / "Read-SwbpipeGuest.ps1"
READ_SCRIPT_REPOSITORY_PATH = "core/handoff/caepipe_external/guest/Read-SwbpipeGuest.ps1"
# The changing operations' script (`_write_command`): the host's fence, the registry API writes and the exercise key,
# each confined by pattern, installed and called the same way.
WRITE_SCRIPT = Path(__file__).resolve().parent / "guest" / "Write-SwbpipeGuest.ps1"
WRITE_SCRIPT_REPOSITORY_PATH = "core/handoff/caepipe_external/guest/Write-SwbpipeGuest.ps1"
HARNESS_DIR = "_harness"
RUN_RECORD_KIND = "openpipestress.caepipe_harness_run_record"
BATCH_COMMAND_SHAPE = '<exe> "<mbf>";batchmode=yes'
BOUNDARY_NOTE = "private validation evidence; not compatibility proof, code compliance or professional acceptance"
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
# The one value name a restore may address outside REG_VALUE_NAME_PATTERN, and only in the recent-files subkey: the
# folder of the configured executable with one trailing backslash, a value the tool writes there (observed format
# behaviour). `reg` cannot carry a quoted argument that ends in a backslash, so this value alone is written through
# the write script's registry API operations and read back through the read script's `regread`, its key, name and
# data passed as encoded data.
LITERAL_VALUE_FOLDER = re.compile(r"^[A-Za-z]:(\\[A-Za-z0-9 ._()-]+)+\Z")
# Every guest command the harness builds is at most GUEST_COMMAND_LIMIT characters, measured as the prlctl call's
# argv joined with single spaces, the program named `prlctl` (`_guest_command_length`): a margin well inside the
# lengths established on the route. It is enforced in one place (`_checked_command`), which every send passes through
# (`_guest_argv`) and which the builders also call, so a plan is refused before anything is sent.
GUEST_COMMAND_LIMIT = 850
# A write-script command carrying a REG_BINARY value's data that would come within 50 characters of the limit does
# not rely on that margin: the data travels in a content-addressed data file instead (`_write_command`). Every other
# command is refused at the limit.
DATA_FILE_THRESHOLD = 800
# The controlled exercise's own names (`exercise_names`): its folder below C:\oracle and its registry key, dated.
EXERCISE_PREFIX = "SWBPIPE-harness-exercise-"
EXERCISE_PARENT = "C:\\oracle"
# Every PowerShell script the harness sends begins with POWERSHELL_QUIET, so Windows PowerShell writes no progress
# record (a module's first-use preparation included), which it would otherwise serialise to the redirected stderr.
# Nothing else is silenced: a warning or an error stays visible on stderr, and every reply judged fails closed on
# any non-blank stderr (`_answered`).
POWERSHELL_QUIET = "$ProgressPreference = 'SilentlyContinue'\n"
# How every PowerShell command is started (-File shows no banner).
POWERSHELL_COMMAND = "powershell -NoProfile -NonInteractive -ExecutionPolicy Bypass"
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
# The prlctl client's two transport lines, exactly as it prints them: it failed to fetch a guest job's result or its
# return code. Such an answer allows that the guest command was dispatched, and possibly that it completed
# (`transport_line`).
TRANSPORT_LINES = ("PrlJob_GetResult: Invalid argument. An invalid argument was passed.",
                   "PrlJob_GetRetCode: Invalid argument. An invalid argument was passed.")
# The guest's answer to `ver`: the one line a Windows guest prints (liveness).
VER_LINE = re.compile(r"^Microsoft Windows \[Version [0-9]+(?:\.[0-9]+)+\]\Z")
# A live run's opening liveness (`await_guest_alive`): at most this many checks in all, this far apart. A guest
# resumed from a long suspend can let the first short call run out its time.
SESSION_LIVENESS_CHECKS = 4
SESSION_LIVENESS_WAIT_S = 30.0
# A drive mapping line of `net use` (`parse_net_use`): an optional status word, the drive letter and colon, then the
# rest of the line, which is read only against the one remote wanted, never by a general pattern for a share name.
NET_USE_DRIVE = re.compile(r"^\s*(?:(?P<status>\S+)\s+)?(?P<letter>[A-Za-z]):\s+(?P<rest>\S.*)\Z")
# A connection line of `net use` with no drive letter (an optional status word, then the remote): a row of its own,
# never a continuation of the line above it.
NET_USE_DEVICELESS = re.compile(r"^\s*(?:\S+\s+)?\\\\\S")
# The network column `net use` prints for the host's shared folders; it follows the remote after white space, or sits
# alone on the next line when a long remote wraps the line.
NET_USE_NETWORK = "Parallels Shared Folders"
# The English form of a `net use` listing (`exchange_share_letter`): its last line, and its column header when it
# lists any connection.
NET_USE_DONE = "The command completed successfully."
NET_USE_EMPTY = "There are no entries in the list."
NET_USE_HEADER = re.compile(r"^Status\s+Local\s+Remote\s+Network\Z")
# A mapping's status that leaves the share usable: OK, or none printed. `net use` prints a status for connections of
# the Windows network provider; a mapping through another provider, as the host's shared folders are, has a blank
# Status column, so a blank status is this share's ordinary state, not a failure.
NET_USE_USABLE = ("OK", "")
# One attempt per token: a fresh, unguessable token for each dispatch of the attempt operation (lower-case hex).
ATTEMPT_TOKEN_PATTERN = re.compile(r"^[0-9a-f]{32}\Z")
# At most this many dispatches of the attempt operation per round trip; the second only after an established
# "not started" of the first.
ATTEMPT_DISPATCHES = 2
# The one deadline of an attempt's reconciliation, from the moment its dispatch returns, through every nested read;
# each read's own timeout, and the pause between the reads that poll for quiescence.
RECONCILE_DEADLINE_S = 180.0
RECONCILE_CALL_TIMEOUT_S = 30.0
RECONCILE_POLL_S = 5.0
# Collection of a completed attempt's files: its own deadline, each read's timeout, the largest chunk and file.
COLLECT_DEADLINE_S = 300.0
COLLECT_CALL_TIMEOUT_S = 60.0
COLLECT_CHUNK_BYTES = 1 << 20
COLLECT_MAX_FILE_BYTES = 1 << 30
# The print file names the host collects from an attempt folder besides OUTPUT_NAMES (the guest reports which).
PRINT_NAME_PATTERN = re.compile(r"^model[A-Za-z0-9_]*\.(?:csv|txt)\Z", re.IGNORECASE)
GUEST_REPORT_SCHEMA = "openpipestress.caepipe_guest_round_trip.v1"
# What `copy` prints on stdout when it copied the one file it was given (the held export's prepare and collect).
COPY_ONE_FILE = re.compile(r"^[ \t]*1 file\(s\) copied\.[ \t]*\Z")
# What `type` prints, exit 1, for a file (or its folder) that does not exist.
TYPE_NOT_FOUND = ("The system cannot find the file specified.", "The system cannot find the path specified.")


class ContainmentError(RuntimeError):
    """A path the pilot would write or delete is a symbolic link or lies outside the exchange folder."""


# The attempt's disposition. The invocation-owned live record of a round trip holds it in ONE place, `live["start"]`,
# written only by `_set_start` and read only through `_classify` (and, outside the harness, through `disposition` on
# the state an error or a record carries outward). Absent or unknown is unresolved.
START_STATES = ("not_dispatched", "unresolved", "not_started", "completed", "partial", "contradicted")
# Restoration of the saved settings is permitted only for a start that cannot still be writing.
RESTORE_PERMITTED = frozenset({"not_dispatched", "not_started", "completed", "partial", "contradicted"})
# A retry is permitted only when the program was never started; completed, partial and contradicted have run it.
RETRY_PERMITTED = frozenset({"not_dispatched", "not_started"})
# The client's answer when it cannot open a session in the guest. Like a transport line, it leaves the terminal report
# to decide; it is never a contradiction of it.
SESSION_UNAVAILABLE_LINE = ("PrlVmGuest_RunProgram: Unable to open new session in this virtual machine. Make sure your "
                            "virtual machine has finished booting, runs the latest version of Parallels Tools, and is "
                            "not isolated from the host OS.")


def disposition(start: Any) -> dict[str, Any]:
    """The one rule table for an attempt's start: the state (anything absent or not a known state is "unresolved")
    and whether restoration and retry are permitted."""
    state = start if start in START_STATES else "unresolved"
    return {"start": state, "restore": state in RESTORE_PERMITTED, "retry": state in RETRY_PERMITTED}


def _set_start(live: dict[str, Any], state: str) -> None:
    live["start"] = state


def _classify(live: Mapping[str, Any] | None) -> dict[str, Any]:
    """The one classifier over a round trip's live record: its start's `disposition`, and whether its run record
    was saved (evidence only: a recording failure never changes the start)."""
    live = live or {}
    return disposition(live.get("start")) | {"evidence_saved": bool(live.get("record_saved"))}


def _carry_disposition(error: BaseException, live: Mapping[str, Any] | None) -> None:
    """Carry the live record's disposition outward on `error` (`disposition`, and `start_unresolved` derived from
    it). The attributes are a copy for the caller, never a fact kept on their own."""
    classified = _classify(live)
    with contextlib.suppress(Exception):
        error.disposition = classified
        error.start_unresolved = classified["start"] == "unresolved"


class StartUnresolved(RuntimeError):
    """The round trip's attempt could not be established as not started, completed, or failed and quiescent within
    its deadline (disposition 4, unknown). Nothing was collected and no restore write was issued. The run record,
    with route `start_unresolved`, is in `record` and the guard result in `settings_guard`; the caller stops."""

    def __init__(self, reason: str, record: dict[str, Any] | None = None):
        super().__init__(reason)
        self.record = record


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
    # Set only by `open_live_session`: the guest's own letter for the exchange share beside the configured root, which
    # every run record states (`configuration_surface.exchange_share`). None: the configured root, unexamined.
    exchange_share: "Mapping[str, Any] | None" = field(default=None, hash=False)


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


def _literal_recent_value(config: HarnessConfig) -> tuple[str, str] | None:
    """(recent-files key id, value name) of the one literal value a restore may address (LITERAL_VALUE_FOLDER): the
    configured executable's folder with one trailing backslash, inside the configured recent-files subkey. None
    without a recent-files subkey, or when the executable is not in a named folder below a drive."""

    guard = config.settings_guard
    if guard is None or guard.recent_files_key is None or not valid_guest_path(config.guest_executable,
                                                                               directory=False):
        return None
    folder = config.guest_executable.rsplit("\\", 1)[0]
    if not LITERAL_VALUE_FOLDER.match(folder):
        return None
    return _registry_key_id(guard.recent_files_key), folder + "\\"


def _literal_write_command(config: HarnessConfig, printed_key: str, name: str, action: str,
                           data: str | None = None) -> str:
    """The guest command for one `set` (REG_BINARY `data`, upper-case hex) or `delete` of a value through the write
    script's registry API (`regset`, `regdelete`), the key, name and data passed as encoded data, never as quoted
    command words. It prints what `reg` prints: `REG_WRITE_OK`, or `REG_NOT_FOUND_LINE` with exit 1. Data too long
    for one command travels in a content-addressed data file (`_write_command`)."""

    if not REG_PRINTED_KEY_PATTERN.match(printed_key) or action not in ("set", "delete") \
            or (action == "set") != (data is not None) or (data is not None and not re.fullmatch(r"(?:[0-9A-F]{2})*",
                                                                                                data)):
        raise ValueError(f"{printed_key}\\{name} cannot be written back exactly")
    payload = {"key": printed_key[len("HKEY_CURRENT_USER\\"):], "name": name} | ({"data": data} if data is not None
                                                                                  else {})
    try:
        return _write_command(config, "regset" if action == "set" else "regdelete", payload, spill=("data",))
    except ValueError as error:
        raise ValueError(f"{printed_key}\\{name} cannot be written back exactly ({error})") from error


def _literal_readback_command(config: HarnessConfig, printed_key: str, name: str) -> str:
    """The read-back of the literal value: the read script's `regread`, which prints the value as `reg query` does
    (REG_BINARY only), or `REG_NOT_FOUND_LINE` on stderr with exit 1."""
    if not REG_PRINTED_KEY_PATTERN.match(printed_key):
        raise ValueError(f"{printed_key}\\{name} cannot be read back exactly")
    return _read_command(config, "regread", {"key": printed_key[len("HKEY_CURRENT_USER\\"):], "name": name})


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


# ---------------------------------------------------------------------------------------------------------------
# The attempt: one guest operation per token, its reconciliation by read-only queries, and host-pulled collection.
# ---------------------------------------------------------------------------------------------------------------
# Every read-only query is an operation of the read script (`_read_command`), and every changing operation of the
# run phase an operation of the write script (`_write_command`). Each prints exactly one answer in a fixed form
# beginning with its own sentinel, after the echo of its script's digest, and each answer is matched whole
# (`_answered`, `_body` and the patterns below); anything else is unresolved.
# The host's fence (the write script's `claim`): created, exists (ERROR_FILE_EXISTS), or held (a sharing violation).
CLAIM_ANSWER = re.compile(r"^SWBPIPE-CLAIM v1 (?P<result>created|exists|held)\r?\n?\Z")
CIM_ANSWER = re.compile(r"^SWBPIPE-CIM v1 (?:none|pid=(?P<pid>[0-9]{1,10}) created=(?P<created>[0-9]{14}\.[0-9]{6}))"
                        r"\r?\n?\Z")
# `error=canary-<n>`: the job query's canary (CloseHandle(NULL)'s last error, which must be 6) was not 6, so no last
# error it read can be trusted; never absent.
JOB_ANSWER = re.compile(r"^SWBPIPE-JOB v1 (?P<state>absent|present active=(?:[0-9]{1,10}|unknown)"
                        r"|error=(?:canary-)?-?[0-9]{1,10})"
                        r"\r?\n?\Z")
CHUNK_ANSWER = re.compile(r"^SWBPIPE-CHUNK v1 (?:absent\r?\n?|size=(?P<size>[0-9]{1,12}) "
                          r"sha256=(?P<sha256>[0-9a-f]{64}) "
                          r"offset=(?P<offset>[0-9]{1,12}) length=(?P<length>[0-9]{1,8}) "
                          r"chunk_sha256=(?P<chunk_sha256>[0-9a-f]{64})\r?\n(?P<data>[A-Za-z0-9+/]*={0,2})\r?\n?)\Z")
# The claim, as the guest script writes it (owner "script") or as the host's fence writes it (owner "host").
CLAIM_SCRIPT_FORM = re.compile(r"^swbpipe-claim v1\r?\ntoken=(?P<token>[0-9a-f]{32})\r?\nowner=script\r?\n"
                               r"pid=(?P<pid>[0-9]{1,10})\r?\ncreated=(?P<created>[0-9]{14}\.[0-9]{6})\r?\n"
                               r"nonce=(?P<nonce>[0-9a-f]{32})\r?\n?\Z")
CLAIM_HOST_FORM = re.compile(r"^swbpipe-claim v1\r?\ntoken=(?P<token>[0-9a-f]{32})\r?\nowner=host\r?\n"
                             r"nonce=(?P<nonce>[0-9a-f]{32})\r?\n?\Z")
FINISHED_FORM = re.compile(r"^swbpipe-finished v1\r?\ntoken=(?P<token>[0-9a-f]{32})\r?\n"
                           r"report=(?P<report>\{[\x20-\x7e]*\})\r?\n?\Z")


STORAGE_ANSWER = re.compile(r"^SWBPIPE-STORAGE v1 (?P<json>\[[\x20-\x7e]*\])\r?\n?\Z")
HASH_ANSWER = re.compile(r"^SWBPIPE-HASH v1 (?:absent|size=(?P<size>[0-9]{1,12}) sha256=(?P<sha256>[0-9a-f]{64}))"
                         r"\r?\n?\Z")


# ---------------------------------------------------------------------------------------------------------------
# The read script: every read-only query is a short -File call (`_read_command`), and every reply's first line is the
# echo of the script's and the data file's computed digests, which the host checks against what it installed.
# ---------------------------------------------------------------------------------------------------------------
READ_OPS = ("cim", "job", "chunk", "regread", "storage", "hash")
READ_SCRIPT_SHA256 = _sha256(READ_SCRIPT.read_bytes())
DATA_DIR = "data"  # data files: `_harness/data/<sha256>.json`
READ_ECHO = re.compile(r"^SWBPIPE-READ v1 script=(?P<script>[0-9a-f]{64}) data=(?P<data>[0-9a-f]{64}|none)\r?\n")
READ_COMMAND_FORM = re.compile(
    "^" + re.escape(POWERSHELL_COMMAND) + r' -File "(?P<path>[A-Za-z]:[^"]*\\' + re.escape(HARNESS_DIR)
    + r'\\(?P<script>[0-9a-f]{64})\\' + re.escape(READ_SCRIPT.name) + r')" -Op (?P<op>[a-z]+) '
    r"-Payload (?P<payload>[A-Za-z0-9+/]+={0,2})(?: -Data (?P<data>[0-9a-f]{64}))?\Z")
# The write script: the same short form, its own echo sentinel.
WRITE_OPS = ("claim", "regset", "regdelete", "keycreate", "keydelete")
WRITE_SCRIPT_SHA256 = _sha256(WRITE_SCRIPT.read_bytes())
WRITE_ECHO = re.compile(r"^SWBPIPE-WRITE v1 script=(?P<script>[0-9a-f]{64}) data=(?P<data>[0-9a-f]{64}|none)\r?\n")
WRITE_COMMAND_FORM = re.compile(
    "^" + re.escape(POWERSHELL_COMMAND) + r' -File "(?P<path>[A-Za-z]:[^"]*\\' + re.escape(HARNESS_DIR)
    + r'\\(?P<script>[0-9a-f]{64})\\' + re.escape(WRITE_SCRIPT.name) + r')" -Op (?P<op>[a-z]+) '
    r"-Payload (?P<payload>[A-Za-z0-9+/]+={0,2})(?: -Data (?P<data>[0-9a-f]{64}))?\Z")
# (command form, echo, the digest the host installed) of each script whose replies carry an echo.
_ECHOED_SCRIPTS = ((READ_COMMAND_FORM, READ_ECHO, READ_SCRIPT_SHA256),
                   (WRITE_COMMAND_FORM, WRITE_ECHO, WRITE_SCRIPT_SHA256))


def _guest_command_length(vm_name: str, command: str) -> int:
    """A guest command's length as the route measures it: the prlctl call's argv joined with single spaces, the
    program named `prlctl` (the host's path to it never reaches the guest)."""
    return len(" ".join(["prlctl", "exec", vm_name, "--current-user", "cmd", "/c", command]))


def _checked_command(config: HarnessConfig, command: str) -> str:
    """The one length rule (`GUEST_COMMAND_LIMIT`), for every guest command without exception: `command`, or
    ValueError before anything is sent."""
    length = _guest_command_length(config.vm_name, command)
    if length > GUEST_COMMAND_LIMIT:
        raise ValueError(f"a guest command of {length} characters exceeds GUEST_COMMAND_LIMIT "
                         f"({GUEST_COMMAND_LIMIT}); nothing was sent")
    return command


def _guest_argv(config: HarnessConfig, command: str) -> list[str]:
    """The prlctl call for `command`: every guest command the harness sends is built here, through the one length
    rule (`_checked_command`)."""
    return [config.prlctl, "exec", config.vm_name, "--current-user", "cmd", "/c", _checked_command(config, command)]


def _encode_payload(payload: Mapping[str, Any]) -> str:
    """A read operation's parameters as `-Payload`: base64 of compact, key-sorted UTF-8 JSON (`_decode_payload`)."""
    text = json.dumps(dict(payload), sort_keys=True, separators=(",", ":"), ensure_ascii=False)
    return base64.b64encode(text.encode("utf-8")).decode("ascii")


def _decode_payload(encoded: str) -> dict[str, Any]:
    """The exact inverse of `_encode_payload`."""
    value = json.loads(base64.b64decode(encoded, validate=True).decode("utf-8"))
    if not isinstance(value, dict):
        raise ValueError("a read payload is a JSON object")
    return value


def _read_command(config: HarnessConfig, op: str, payload: Mapping[str, Any], data: str | None = None) -> str:
    """The short guest command for one read operation: the installed read script by -File, the operation, its
    parameters as `-Payload`, and with `data` the SHA-256 of an installed data file (`_install_data_file`) whose
    object the script merges into them. Checked against the one length rule as it is built."""
    if op not in READ_OPS or (data is not None and not re.fullmatch(r"[0-9a-f]{64}", data)):
        raise ValueError(f"no read operation {op!r} with data {data!r}")
    script = _win(config.guest_exchange_root, HARNESS_DIR, READ_SCRIPT_SHA256, READ_SCRIPT.name)
    command = (f"{POWERSHELL_COMMAND} -File {_quote(script)} -Op {op} -Payload {_encode_payload(payload)}"
               + (f" -Data {data}" if data else ""))
    return _checked_command(config, command)


def _write_command(config: HarnessConfig, op: str, payload: Mapping[str, Any], *, spill: tuple[str, ...] = ()) -> str:
    """The short guest command for one changing operation of the write script, in `_read_command`'s form. When the
    command would exceed `DATA_FILE_THRESHOLD`, the payload members named in `spill` travel instead in a
    content-addressed data file installed in the exchange folder (`_install_data_file`), whose digest the command
    names. Checked against the one length rule as it is built."""
    if op not in WRITE_OPS:
        raise ValueError(f"no write operation {op!r}")
    script = _quote(_win(config.guest_exchange_root, HARNESS_DIR, WRITE_SCRIPT_SHA256, WRITE_SCRIPT.name))
    command = f"{POWERSHELL_COMMAND} -File {script} -Op {op} -Payload {_encode_payload(payload)}"
    if _guest_command_length(config.vm_name, command) <= DATA_FILE_THRESHOLD or not any(key in payload for key in spill):
        return _checked_command(config, command)
    inline = {key: value for key, value in payload.items() if key not in spill}
    root = config.exchange_root
    digest = _install_data_file(root, contained_dir(root, root.resolve() / HARNESS_DIR),
                                {key: payload[key] for key in spill if key in payload})
    return _checked_command(config, f"{POWERSHELL_COMMAND} -File {script} -Op {op} -Payload "
                                    f"{_encode_payload(inline)} -Data {digest}")


def _settle_echo(record: dict[str, Any]) -> None:
    """For a read-script or write-script reply: `echo` (what it named, whether it is the expected one) and
    `reply_body` (the reply after the echo line). The echo must be the reply's first line, exactly
    `SWBPIPE-READ v1 script=<sha256> data=<sha256|none>` (`SWBPIPE-WRITE v1 ...` for the write script), naming the
    script the host installed (its digest, which is also the digest in the command's path) and the data file the
    command named (or none). Anything else leaves the reply unresolved (`_answered`, `_not_found`), never an answer.
    Other commands are left as they are."""
    command = record.get("guest_command") or ""
    for command_form, echo_form, installed in _ECHOED_SCRIPTS:
        form = command_form.match(command)
        if form is not None:
            break
    else:
        return
    expected = (installed, form["data"] or "none")
    stdout = record.get("stdout") or ""
    seen = echo_form.match(stdout) if isinstance(stdout, str) else None
    verified = bool(seen) and form["script"] == installed and (seen["script"], seen["data"]) == expected
    record["echo"] = {"expected": f"script={expected[0]} data={expected[1]}",
                      "seen": seen[0].rstrip("\r\n") if seen else None, "verified": verified}
    record["reply_body"] = stdout[seen.end():] if verified else ""


def _body(record: Mapping[str, Any]) -> str:
    """The text a reply's form is matched against: after the verified echo for a read-script reply (empty when the
    echo is not verified), else the whole stdout."""
    if "echo" in record:
        return record.get("reply_body") or ""
    return record.get("stdout") or ""


def storage_meets(entry: Mapping[str, Any]) -> bool:
    """The storage predicate for one folder of a `storage` answer: present; its final path on C: (`\\\\?\\C:\\...`);
    its volume path exactly `C:\\`; NTFS; Fixed; and no path component a reparse point (each a directory)."""
    final = entry.get("final")
    components = entry.get("components")
    return (entry.get("absent") is False and isinstance(final, str) and final.upper().startswith("\\\\?\\C:\\")
            and entry.get("volume") == "C:\\" and entry.get("format") == "NTFS" and entry.get("type") == "Fixed"
            and isinstance(components, list) and bool(components)
            and all(isinstance(item, dict) and item.get("reparse") is False and item.get("directory") is True
                    for item in components))


def parse_storage(body: str, folders: list[str]) -> list[dict[str, Any]] | None:
    """A `storage` answer whole: one entry per folder asked, in order, each `absent` or with every member; with
    `meets` (`storage_meets`) added. None for anything out of form."""
    match = STORAGE_ANSWER.match(body)
    if match is None:
        return None
    try:
        entries = json.loads(match["json"])
    except json.JSONDecodeError:
        return None
    if not isinstance(entries, list) or len(entries) != len(folders):
        return None
    keys = {"folder", "absent", "final", "volume", "format", "type", "components"}
    for entry, folder in zip(entries, folders):
        if not isinstance(entry, dict) or not isinstance(entry.get("folder"), str) \
                or entry["folder"].lower() != folder.lower() or not isinstance(entry.get("absent"), bool) \
                or set(entry) != ({"folder", "absent"} if entry["absent"] else keys):
            return None
        entry["meets"] = storage_meets(entry)
    return entries


def _type_answer(record: Mapping[str, Any]) -> tuple[str, str]:
    """A `type` read: ("text", stdout) for exit 0 with nothing on stderr; ("absent", "") for exit 1, nothing on
    stdout and stderr exactly a not-found line (`TYPE_NOT_FOUND`); ("unresolved", "") for anything else, a sharing
    violation (the writer still holds the file) and a transport answer included."""
    if _answered(record):
        return "text", record["stdout"]
    if record["exit_status"] == 1 and not (record.get("stdout") or "").strip() \
            and (record.get("stderr") or "").strip() in TYPE_NOT_FOUND:
        return "absent", ""
    return "unresolved", ""


def _parse_claim(text: str, token: str) -> dict[str, Any] | None:
    """A complete claim on `token`, as written by the guest script or by the host's fence, else None."""
    for owner, form in (("script", CLAIM_SCRIPT_FORM), ("host", CLAIM_HOST_FORM)):
        match = form.match(text)
        if match and match["token"] == token:
            return {"owner": owner, **{key: value for key, value in match.groupdict().items() if key != "token"}}
    return None


def _parse_finished(text: str, token: str) -> dict[str, Any] | None:
    """The terminal report of `token`: the guest's report object naming the same token, else None."""
    match = FINISHED_FORM.match(text)
    if not match or match["token"] != token:
        return None
    try:
        report = json.loads(match["report"])
    except json.JSONDecodeError:
        return None
    if not isinstance(report, dict) or report.get("schema") != GUEST_REPORT_SCHEMA \
            or report.get("attempt_token") != token:
        return None
    return report


def _install_add_only(root: Path, folder: Path, name: str, data: bytes, shown: str) -> None:
    """Put `data` at `folder/name` in the exchange folder: created exclusively when absent, verified byte for byte
    when present, refused when it differs; never overwritten."""
    target = contained(root, folder / name)
    if os.path.lexists(target):
        if target.is_symlink() or not target.is_file() or target.read_bytes() != data:
            raise ContainmentError(f"{shown} in the exchange folder differs from the content of that digest; refused "
                                   "(it is never overwritten)")
        return
    contained_dir(root, folder)
    # O_BINARY (Windows only; 0 elsewhere): without it a Windows host would open in text mode and write LF as CRLF,
    # so the installed bytes, which the guest script hashes for its echo, would differ from `data`.
    handle = os.open(target, os.O_CREAT | os.O_EXCL | os.O_WRONLY | getattr(os, "O_BINARY", 0), 0o600)
    with os.fdopen(handle, "wb") as stream:
        stream.write(data)


def _install_guest_script(root: Path, harness_dir: Path, script: Path = GUEST_SCRIPT) -> dict[str, str]:
    """Put a guest script (the round-trip script, or the read script) at its content-addressed, add-only path
    `_harness/<sha256>/<name>` in the exchange folder (`_install_add_only`). The earlier flat copy `_harness/<name>`
    is left as it is."""

    data = script.read_bytes()
    digest = _sha256(data)
    shown = f"{HARNESS_DIR}/{digest}/{script.name}"
    _install_add_only(root, contained(root, harness_dir / digest), script.name, data, shown)
    return {"sha256": digest, "exchange_copy": shown}


def _install_data_file(root: Path, harness_dir: Path, value: Mapping[str, Any]) -> str:
    """Put a read operation's data (a JSON object) at `_harness/data/<sha256>.json` by the same add-only rules, and
    return its SHA-256 for `_read_command`. Used only where the payload would take a command over the limit."""
    data = (json.dumps(dict(value), sort_keys=True, separators=(",", ":")) + "\n").encode("utf-8")
    digest = _sha256(data)
    _install_add_only(root, contained(root, harness_dir / DATA_DIR), f"{digest}.json", data,
                      f"{HARNESS_DIR}/{DATA_DIR}/{digest}.json")
    return digest


class _Reconciler:
    """The reconciliation of one attempt token T: read-only queries, and the host's fence, under one deadline.

    Dispositions (`run`): `not_started` (the fence created T's claim); `completed` (the script's terminal report is
    complete, and the attempt quiescent); `partial` (the script claimed, the attempt is quiescent, and there is no
    terminal report); `unknown` (anything not established before the deadline). A missing file or an absent process
    is never taken as more than its reading shows."""

    def __init__(self, config: HarnessConfig, runner: Callable[..., subprocess.CompletedProcess],
                 steps: list[dict[str, Any]], work: str, token: str, deadline: float, prefix: str = ""):
        self.config, self.runner, self.steps, self.work, self.token = config, runner, steps, work, token
        if prefix not in ("", EXERCISE_PREFIX):
            raise ValueError("an attempt's names carry no prefix but the exercise's")
        self.prefix = prefix  # the exercise's names only (EXERCISE_PREFIX); empty for every run
        self.deadline = deadline
        self.host_nonce = secrets.token_hex(16)  # constant for T: every fence of T writes the same bytes
        self.evidence: list[str] = []
        self.counts: dict[str, int] = {}

    def _read(self, kind: str, command: str) -> dict[str, Any] | None:
        """One read (no retry inside it: the caller re-reads), or None when the deadline has passed."""
        if time.monotonic() >= self.deadline:
            return None
        self.counts[kind] = self.counts.get(kind, 0) + 1
        return _guest_step(self.config, self.runner, RECONCILE_CALL_TIMEOUT_S, self.steps,
                           f"{kind}_{self.counts[kind]}", command, self.deadline, attempts=1)

    def _pause(self) -> bool:
        """Wait before the next poll; False when the deadline would pass first."""
        if time.monotonic() + RECONCILE_POLL_S >= self.deadline:
            return False
        _backoff(RECONCILE_POLL_S)
        return True

    def _unknown(self, reason: str) -> dict[str, Any]:
        return {"disposition": "unknown", "reason": reason, "evidence": self.evidence}

    def path(self, suffix: str) -> str:
        return _win(self.work, self.prefix + self.token + suffix)

    def run(self) -> dict[str, Any]:
        claim = self._claim()
        if claim is None or claim.get("disposition"):
            return claim or self._unknown("the claim could not be read in its form before the deadline")
        launcher = self._launcher_ended(claim)
        if launcher is not None:
            return launcher
        job = self._job_absent()
        if job is not None:
            return job
        # (c): any later invocation under T fails its claim (T.claim exists); every earlier token of this round
        # trip was fenced before T was dispatched; the host dispatches nothing else that starts the program.
        self.evidence.append("pending: none (T is claimed; every earlier token of this round trip is fenced)")
        return self._finished(claim)

    def _claim(self) -> dict[str, Any] | None:
        """R1, re-read until it is in its form: the script's claim, or the fence's (not started)."""
        while True:
            record = self._read("claim_read", f"type {_quote(self.path('.claim'))}")
            if record is None:
                return None
            state, text = _type_answer(record)
            claim = _parse_claim(text, self.token) if state == "text" else None
            if claim is not None and claim["owner"] == "script":
                self.evidence.append(f"claimed by the script (pid {claim['pid']}, created {claim['created']})")
                return claim
            if claim is not None:  # the host's
                if claim["nonce"] == self.host_nonce:
                    self.evidence.append("the fence created the claim: not started")
                    return {"disposition": "not_started", "reason": "the host's fence created the claim; the attempt "
                                                                    "never claimed and can never start",
                            "evidence": self.evidence}
                return self._unknown("the claim is a host claim with another nonce")
            if state == "absent":
                self._fence()
                continue  # the claim is read again whatever the fence answered
            if not self._pause():
                return None

    def _fence(self) -> None:
        """The host's exclusive create of T's claim. Its answer is recorded and checked against its form; the
        decision is always the claim read that follows. Re-issuing it writes the same bytes and can never
        overwrite, so a late earlier fence changes nothing."""
        content = f"swbpipe-claim v1\r\ntoken={self.token}\r\nowner=host\r\nnonce={self.host_nonce}\r\n"
        record = self._read("fence", _write_command(self.config, "claim", {
            "folder": self.work, "name": self.prefix + self.token + ".claim", "content": content}))
        if record is None:
            return
        match = CLAIM_ANSWER.match(_body(record)) if _answered(record) else None
        record["answer"] = match["result"] if match else "unresolved"
        self.evidence.append(f"fence: {record['answer']}")
        if match is None:
            self._pause()  # a fence out of form is not repeated at once; the claim read decides

    def _launcher_ended(self, claim: Mapping[str, Any]) -> dict[str, Any] | None:
        """(a): the CIM query for the claim's pid shows no process, or one created at another time (pid reuse).
        Re-polled while the launcher runs or the answer is not in its form; None once ended."""
        while True:
            record = self._read("launcher_query", _read_command(self.config, "cim", {"pid": int(claim["pid"])}))
            if record is None:
                return self._unknown("the launcher was not established as ended before the deadline")
            match = CIM_ANSWER.match(_body(record)) if _answered(record) else None
            if match and (match["pid"] is None or match["created"] != claim["created"]):
                self.evidence.append("launcher ended" + ("" if match["pid"] is None else " (its pid now names "
                                                                                           "another process)"))
                return None
            record["answer"] = "running" if match else "unresolved"
            if not self._pause():
                return self._unknown("the launcher was not established as ended before the deadline")

    def _job_absent(self) -> dict[str, Any] | None:
        """(b): opening the job swbpipe-T by name fails with ERROR_FILE_NOT_FOUND. Any other answer (a member, an
        open handle, another error, an answer out of form) is re-polled; None once absent."""
        while True:
            record = self._read("job_query", _read_command(self.config, "job", {"job": f"swbpipe-{self.token}"}))
            if record is None:
                return self._unknown("no remaining job member was not established before the deadline")
            match = JOB_ANSWER.match(_body(record)) if _answered(record) else None
            if match and match["state"] == "absent":
                self.evidence.append("job: no member, no handle")
                return None
            record["answer"] = match["state"] if match else "unresolved"
            if not self._pause():
                return self._unknown("no remaining job member was not established before the deadline")

    def _finished(self, claim: Mapping[str, Any]) -> dict[str, Any]:
        """After quiescence T.finished can no longer change: complete is `completed`, absent is `partial`."""
        while True:
            record = self._read("finished_read", f"type {_quote(self.path('.finished'))}")
            if record is None:
                return self._unknown("the terminal report could not be read in its form before the deadline")
            state, text = _type_answer(record)
            report = _parse_finished(text, self.token) if state == "text" else None
            if report is not None:
                self.evidence.append("terminal report complete")
                return {"disposition": "completed", "reason": "the script's terminal report is complete and the "
                                                              "attempt is quiescent", "report": report,
                        "claim": dict(claim), "evidence": self.evidence}
            if state == "absent":
                self.evidence.append("no terminal report")
                return {"disposition": "partial", "reason": "the script claimed, the attempt is quiescent and it left "
                                                            "no terminal report", "claim": dict(claim),
                        "evidence": self.evidence}
            if not self._pause():
                return self._unknown("the terminal report could not be read in its form before the deadline")


def _collect_attempt(config: HarnessConfig, runner: Callable[..., subprocess.CompletedProcess],
                     steps: list[dict[str, Any]], attempt_dir: str,
                     names: list[str]) -> tuple[dict[str, bytes], str | None]:
    """Read each named file of a quiescent attempt folder by read-only chunk queries ({name: bytes} for those
    present) and verify every chunk and the whole file by SHA-256. A chunk read that is lost or out of form is read
    again: it changes nothing. (files, None), or (what was read, why collection failed) at COLLECT_DEADLINE_S."""

    deadline = time.monotonic() + COLLECT_DEADLINE_S
    files: dict[str, bytes] = {}
    for name in names:
        parts: list[bytes] = []
        whole = size = None
        offset = 0
        reads = 0
        while True:
            if time.monotonic() >= deadline:
                return files, f"{name} could not be read in full before the collection deadline"
            reads += 1
            record = _guest_step(config, runner, COLLECT_CALL_TIMEOUT_S, steps, f"collect_{name}_{offset}_{reads}",
                                 _read_command(config, "chunk", {"path": _win(attempt_dir, name), "offset": offset,
                                                                 "length": COLLECT_CHUNK_BYTES}),
                                 deadline, attempts=1)
            match = CHUNK_ANSWER.match(_body(record)) if _answered(record) else None
            chunk = None
            if match and match["size"] is None:
                if offset == 0:
                    record["answer"] = "absent"
                    break
                return files, f"{name} vanished while it was read"
            if match:
                length = int(match["length"])
                try:
                    chunk = base64.b64decode(match["data"], validate=True)
                except ValueError:
                    chunk = None
                consistent = (int(match["offset"]) == offset and (size is None or (int(match["size"]), match["sha256"])
                                                                   == (size, whole)))
                if chunk is None or len(chunk) != length or _sha256(chunk) != match["chunk_sha256"] \
                        or not consistent or int(match["size"]) > COLLECT_MAX_FILE_BYTES \
                        or (length == 0 and offset < int(match["size"])):
                    chunk = None
            if chunk is None:
                record["answer"] = "unresolved"
                if time.monotonic() + RECONCILE_POLL_S >= deadline:
                    return files, f"{name} could not be read in full before the collection deadline"
                _backoff(RECONCILE_POLL_S)
                continue
            record["answer"] = "chunk"
            size, whole = int(match["size"]), match["sha256"]
            parts.append(chunk)
            offset += len(chunk)
            if offset >= size:
                data = b"".join(parts)
                if len(data) != size or _sha256(data) != whole:
                    return files, f"{name} does not match the size and SHA-256 its reads reported"
                files[name] = data
                break
    return files, None


def run_caepipe_round_trip(config: HarnessConfig, case_id: str, mbf_text: str, *, timeout_s: float = 600,
                           runner: Callable[..., subprocess.CompletedProcess] = subprocess.run,
                           gui_fallback: bool = True) -> dict[str, Any]:
    """Solve one MBF with the user's CAEPIPE in the configured virtual machine and record what happened.

    Every guest call is `prlctl exec <vm> --current-user cmd /c <command>`. The host stages the MBF in the exchange
    folder (`caepipe_in/model.mbf`) and the guest script at its content-addressed, add-only path, then makes these
    calls, each only under the condition stated:
    - guarded only: the precondition process list, then (only when it is clear) the settings reading;
    - only when the guard, if any, admitted the run: the attempt, one invocation of the guest script under a fresh
      token T (claim, job, attempt folder, verified copy, launch, terminal report; never retried);
    - after every attempt, whatever its reply: its reconciliation under one deadline (`_Reconciler`): claim reads,
      the host's fence (only while the claim reads absent), the launcher and job queries (only once the script's
      claim is read), the terminal-report read (only once quiescent);
    - only when the first attempt is established as not started: a second attempt under a new token, reconciled
      likewise; never a third;
    - only when the attempt completed and the guest's report names the host's MBF hash: the read-only chunk reads
      of the print the report names, model.res and model.mod, written by the host into `caepipe_out/<T>/`;
    - guarded only, whatever happened: the owned-process record and process list (only when an attempt may have
      started the program), the settings reading again, and a restore only when the guard's conditions hold and the
      start is resolved (`_guard_check`).
    No other guest command is issued. The host never prepares, stages or copies inside the guest.

    Routes for the start: `not_started` (two attempts, each established as not started); `start_partial` (the
    script claimed, the attempt is quiescent, no terminal report; never retried); `report_reply_contradiction`
    (a success terminal report beside a nonzero dispatch reply that is neither a transport line nor the
    session-unavailable line: quiescent, with a report, never retried, never compared; both kept whole);
    `start_unresolved` (not established within the deadline: StartUnresolved is raised after the records, with
    nothing collected and no restore write). The disposition is kept only in the live record and carried outward on
    every exit (`_classify`, `_carry_disposition`). A completed attempt's report then decides the route as before;
    outputs are collected only when the guest reports solving an MBF whose SHA-256 equals the host's (otherwise
    `input_hash_mismatch`).

    `gui_fallback=False` tells the guest script not to fall back to printing through the GUI when the batch route
    leaves no CSV (route `batch_no_csv`); the fallback, when allowed, runs inside the same claim and job. With
    `settings_guard` configured, the run record's `settings_guard` holds the check; an exception raised after the
    guest work started carries it as `settings_guard`.
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

    def check(entry: Path, *, folders: bool) -> None:
        if entry.is_symlink():
            raise ContainmentError(f"{entry.relative_to(Path(root).resolve())} in the exchange folder is a symbolic "
                                   "link; refused")
        # The layer holds regular files and, one level down, the attempt folders (`caepipe_out/<T>/`) of regular
        # files; nothing else, so no write of ours can go through a planted directory or link.
        if folders and entry.is_dir() and ATTEMPT_TOKEN_PATTERN.match(entry.name):
            for inner in entry.iterdir():
                check(inner, folders=False)
            return
        if not entry.is_file():
            raise ContainmentError(f"{entry.relative_to(Path(root).resolve())} in the exchange folder is not a "
                                   "regular file; refused")

    for entry in contained(root, out).iterdir():
        check(entry, folders=True)


def _clear_outputs(root: Path, out: Path) -> None:
    """Delete the previous run's outputs from `caepipe_out`, matching names without regard to letter case."""

    _refuse_links(root, out)
    for entry in out.iterdir():
        name = entry.name.lower()
        # Only flat files; an attempt folder (`caepipe_out/<T>/`) is never rewritten or removed.
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


def transport_line(exit_status: Any, stdout: str | None, stderr: str | None) -> str | None:
    """The listed transport line when a prlctl answer is exactly one (`TRANSPORT_LINES`): exit 255 and the whole
    answer, both streams together, one listed line (surrounding blank space aside). Otherwise None.

    Such an answer allows that the guest command was dispatched, and possibly that it completed; it never shows
    that the command did not start. Any other answer is not a transport answer, whatever its exit."""

    if exit_status != 255:
        return None
    lines = [line.strip() for line in f"{stdout or ''}\n{stderr or ''}".splitlines() if line.strip()]
    return lines[0] if len(lines) == 1 and lines[0] in TRANSPORT_LINES else None


# The route of a success terminal report contradicted by its own dispatch reply (`_report_contradicted`).
REPORT_REPLY_CONTRADICTION = "report_reply_contradiction"


def reply_leaves_report_to_decide(exit_status: Any, stdout: str | None, stderr: str | None) -> bool:
    """Whether a dispatch reply leaves the terminal report to decide: exit 0; no reply at all (a host timeout, or a
    client that could not be started); a transport answer (`transport_line`); or exactly the client's
    session-unavailable line (`SESSION_UNAVAILABLE_LINE`), with any exit. Any other nonzero reply contradicts a
    success report."""
    if not isinstance(exit_status, int) or isinstance(exit_status, bool) or exit_status == 0:
        return True
    if transport_line(exit_status, stdout, stderr) is not None:
        return True
    lines = [line.strip() for line in f"{stdout or ''}\n{stderr or ''}".splitlines() if line.strip()]
    return len(lines) == 1 and lines[0] == SESSION_UNAVAILABLE_LINE


def _report_contradicted(report: Any, dispatch: Mapping[str, Any]) -> bool:
    """A success terminal report (a CSV status) beside a dispatch reply that does not leave the report to decide.
    Total: it reads two values and raises nothing, so it may run before the reconciled start is set."""
    status = report.get("status") if isinstance(report, dict) else None
    return status in CSV_ROUTES and not reply_leaves_report_to_decide(
        dispatch.get("exit_status"), dispatch.get("stdout"), dispatch.get("stderr"))


def _guest_step(config: HarnessConfig, runner: Callable[..., subprocess.CompletedProcess], timeout_s: float,
                steps: list[dict[str, Any]], name: str, command: str, deadline: float | None = None, *,
                attempts: int = PRLCTL_ATTEMPTS, backoff_s: float = 0.0,
                within_timeout: bool = False) -> dict[str, Any]:
    """Run one guest command through `prlctl exec`, append its record to `steps` and return it.

    A transport answer (`transport_line`) is retried, up to `attempts` in all, `backoff_s` apart; no other answer
    is retried here. Callers pass `attempts=1` for any command that changes something, and retry only read-only
    commands whose source is stable. With `deadline` (a `time.monotonic()` value), and with `within_timeout` (all
    attempts together within `timeout_s`), no attempt runs past the limit: each attempt's timeout is cut to what
    remains, and no further attempt starts when the limit would be reached before it does; the last attempt's
    result is then returned. `attempt_log` lists every attempt's exit and, for a transport answer, its line.
    """

    argv = _guest_argv(config, command)  # the one length rule, before anything is sent
    record: dict[str, Any] = {"name": name, "guest_command": command, "started": _now(), "attempts": 0,
                              "attempt_log": []}
    limit = deadline
    if within_timeout:
        call_end = time.monotonic() + timeout_s
        limit = call_end if limit is None else min(limit, call_end)
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
        line = transport_line(record["exit_status"], record["stdout"], record["stderr"])
        record["attempt_log"].append({"attempt": record["attempts"], "exit_status": record["exit_status"]}
                                     | ({"transport_line": line} if line else {}))
        if line is None or record["attempts"] >= attempts:
            break
        if limit is not None and time.monotonic() + backoff_s >= limit:
            record["attempts_stopped_by"] = "deadline"
            break
        if backoff_s:
            _backoff(backoff_s)
    record["finished"] = _now()
    _settle_echo(record)
    steps.append(record)
    return record


def _answered(record: Mapping[str, Any]) -> bool:
    """Exit 0 with nothing but blank space on stderr: the precondition of every judged reply's form, reads and
    changing commands alike (each then matches its stdout, or for a read-script reply the text after its echo
    (`_body`), against its own exact form). Anything on stderr, a PowerShell warning or a serialised record included,
    fails it, and so does a read-script reply whose echo is missing, malformed or not the expected one."""
    return record["exit_status"] == 0 and not (record.get("stderr") or "").strip() \
        and record.get("echo", {}).get("verified", True)


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
        if live is not None:
            _carry_disposition(error, live)
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


# What `reg add` and `reg delete` print on success; a write counts as completed only with exit 0, nothing on stderr
# and stdout exactly one of these lines (`reg` ends it with a full stop; the registry-API script prints the same).
REG_WRITE_OK = "The operation completed successfully"
REG_WRITE_OK_LINES = (REG_WRITE_OK + ".",)


# The stderr text a failed or unknown write keeps in the records, at most this many UTF-8 bytes (diagnostics only).
FAILED_WRITE_STDERR_CAP_BYTES = 2048


def _failed_write_stderr(item: Mapping[str, Any]) -> dict[str, Any]:
    """The capped stderr text of a mutating step that did not complete, for the records; empty for any other step.

    Only a write (`mutating`) whose outcome is not `completed` keeps text: no successful step's output and no
    stdout. The text is cut to `FAILED_WRITE_STDERR_CAP_BYTES` UTF-8 bytes at a character boundary;
    `stderr_truncated` says whether anything was cut. Nothing here affects any check, attempt or decision."""

    if not item.get("mutating") or item.get("outcome") == "completed":
        return {}
    raw = item.get("stderr") or ""
    data = (raw if isinstance(raw, str) else _text(raw)).encode("utf-8", errors="replace")
    return {"stderr_text": data[:FAILED_WRITE_STDERR_CAP_BYTES].decode("utf-8", errors="ignore"),
            "stderr_truncated": len(data) > FAILED_WRITE_STDERR_CAP_BYTES}


def _note_failed_write(state: dict[str, Any] | None, record: dict[str, Any]) -> None:
    """Keep a failed or unknown write's capped stderr in `state["failed_write_stderr"]`, which every guard result
    built from `state` carries (success, failure path, live result for an interruption)."""

    if state is not None:
        diagnostic = _failed_write_stderr(record)
        if diagnostic:
            state.setdefault("failed_write_stderr", []).append(
                {"step": record["name"], "exit_status": record["exit_status"],
                 "outcome": record.get("outcome", "not_returned"),
                 "stderr_chars": len(record.get("stderr") or "")} | diagnostic)


class RestoreWriteFailed(RuntimeError):
    """A registry write of the restore did not complete: `outcome` is `failed` (the command reported failure) or
    `unknown` (the transport failed, or the output was not the expected one; the write may or may not have taken)."""

    def __init__(self, name: str, outcome: str, exit_status: Any, transport: bool = False):
        super().__init__(f"{name}: write {outcome} (exit status {exit_status})")
        self.outcome = outcome
        self.exit_status = exit_status
        self.transport = transport  # the answer was a transport answer (`transport_line`)


def _guest_write(config: HarnessConfig, runner: Callable[..., subprocess.CompletedProcess], timeout_s: float,
                 steps: list[dict[str, Any]], name: str, command: str,
                 state: dict[str, Any] | None = None, deadline: float | None = None) -> dict[str, Any]:
    """Run one mutating guest command exactly once: this call never retries, whatever the transport reports (the
    restore may issue a value set once more, only after settling its outcome; `_restore_recent_files`).

    The step is recorded before anything is raised. Its answer form: exit 0, nothing on stderr, and stdout exactly
    one line of `REG_WRITE_OK_LINES`; that is the only success. A transport answer (`transport_line`) or exit 0 with
    any other answer is `unknown`, and so is exit 255 with any other text; any other exit is `failed`; both raise
    RestoreWriteFailed. Only the transport answer is settled and may be issued again
    (`_transport_failure`). An exception from the runner (a timeout, a launch
    failure, cancellation) is stored as `state["primary_error"]` before anything else (unless one is already
    there), then recorded under a guard whose own failure is kept as secondary, and re-raised unchanged.
    """

    argv = _guest_argv(config, command)  # the one length rule, before anything is sent
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
            _note_failed_write(state, record)
        except BaseException as late:
            if state is not None:
                state.setdefault("secondary_errors", []).append(late)
        finally:
            steps.append(record)
        raise error
    record.update(exit_status=completed.returncode, stdout=completed.stdout or "", stderr=completed.stderr or "",
                  finished=_now())
    _settle_echo(record)
    line = transport_line(completed.returncode, record["stdout"], record["stderr"])
    record["attempt_log"] = [{"attempt": 1, "exit_status": completed.returncode}
                             | ({"transport_line": line} if line else {})]
    steps.append(record)
    if _answered(record) and _body(record).strip() in REG_WRITE_OK_LINES:
        record["outcome"] = "completed"
        return record
    # Exit 255 is the client's own failure: unknown whatever its text, but only a transport answer is settled.
    record["outcome"] = "unknown" if line is not None or completed.returncode in (0, 255) else "failed"
    _note_failed_write(state, record)
    raise RestoreWriteFailed(name, record["outcome"], completed.returncode, transport=line is not None)


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
    """A write whose outcome the transport left unknown: a transport answer (exit 255, `transport_line`), or a
    timeout."""
    return (isinstance(error, RestoreWriteFailed) and error.outcome == "unknown" and error.transport) \
        or isinstance(error, subprocess.TimeoutExpired)


# What `reg query` reports, exit 1, for a key or value that does not exist; any other failure is unreadable.
REG_NOT_FOUND = "unable to find the specified registry key or value"
REG_NOT_FOUND_LINE = "ERROR: The system was unable to find the specified registry key or value."


def _copied_one_file(record: Mapping[str, Any]) -> bool:
    """A `copy` of one file answered in its form: exit 0, blank stderr, and stdout exactly one non-blank line,
    `1 file(s) copied.` (`COPY_ONE_FILE`). Anything else, a sign-in text with exit 0 included, is not a copy."""
    lines = [line for line in (record.get("stdout") or "").splitlines() if line.strip()]
    return _answered(record) and len(lines) == 1 and bool(COPY_ONE_FILE.match(lines[0]))


def _single_json_object(text: str) -> dict[str, Any] | None:
    """`text`, blank space aside, as exactly one JSON object and nothing else; else None."""
    try:
        value = json.loads(text.strip())
    except json.JSONDecodeError:
        return None
    return value if isinstance(value, dict) else None


def _not_found(record: Mapping[str, Any]) -> bool:
    """`reg`'s not-found answer exactly: exit 1, nothing on stdout (for a read-script reply: its verified echo and
    nothing after it), stderr exactly `REG_NOT_FOUND_LINE`."""
    return record["exit_status"] == 1 and record.get("echo", {}).get("verified", True) \
        and not _body(record).strip() and (record.get("stderr") or "").strip() == REG_NOT_FOUND_LINE


def _read_value(step: Callable[..., dict[str, Any]], name: str, key: str,
                value_name: str, command: str | None = None) -> tuple[str, tuple[str, str] | None]:
    """One value read back alone (`reg query <key> /v <name>`, or `command` for the literal value, which prints the
    same): ("present", (type, data)), ("absent", None) when reg reports that it does not exist, or ("unreadable",
    None) when the read fails otherwise, does not parse whole, or does not show exactly that one value."""

    record = step(name, command or f"reg query {_quote(key)} /v {_quote(value_name)}")
    if _not_found(record):
        return "absent", None
    parsed = parse_reg_query(_body(record)) if _answered(record) else None
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
    if not _answered(record):
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
    snapshot = parse_reg_query(record["stdout"]) if _answered(record) else None
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
    read or is not valid `tasklist /FO CSV /NH` output: exit 0, nothing on stderr, and every non-blank line a CSV
    row of five fields whose second is a decimal id (no row at all is not a list)."""

    listing = step(name, "tasklist /FO CSV /NH")
    if not _answered(listing):
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
    """Liveness check: `alive` only when the guest answers `cmd /c ver` in its form: exit 0, nothing on stderr, and
    stdout exactly one non-blank line, the Windows version line (`VER_LINE`). Any other answer, a sign-in or
    licence text with exit 0 included, is `unreachable`. Starts and stops nothing; the guest's output is not kept."""

    steps: list[dict[str, Any]] = []
    try:
        record = _guest_step(config, runner, timeout_s, steps, "liveness", "ver")
    except Exception as failure:
        return {"status": "unreachable", "reason": f"the liveness call failed: {type(failure).__name__}"}
    lines = [line.strip() for line in record["stdout"].splitlines() if line.strip()]
    if not (_answered(record) and len(lines) == 1 and VER_LINE.match(lines[0])):
        return {"status": "unreachable",
                "reason": f"the guest did not answer the liveness check in its form (exit status "
                          f"{record['exit_status']})"}
    return {"status": "alive"}


def await_guest_alive(config: HarnessConfig, *, checks: int = SESSION_LIVENESS_CHECKS,
                      wait_s: float = SESSION_LIVENESS_WAIT_S,
                      liveness: Callable[[HarnessConfig], dict[str, Any]] | None = None,
                      sleep: Callable[[float], None] | None = None) -> dict[str, Any]:
    """The run's opening liveness: `check_guest_alive` up to `checks` times, `wait_s` apart, until the guest answers.
    A guest resumed from a long suspend can let the first short call run out its time; it is asked again, never more
    than `checks` times in all. `alive` with each check's outcome, or `guest_unreachable` with the last one's reason.
    Starts and stops nothing."""

    liveness = liveness or check_guest_alive
    sleep = sleep or _backoff
    outcomes: list[dict[str, Any]] = []
    for number in range(1, checks + 1):
        if number > 1:
            sleep(wait_s)
        outcome = liveness(config)
        outcomes.append({"check": number, "status": outcome.get("status")}
                        | ({"reason": outcome["reason"]} if outcome.get("reason") else {}))
        if outcome.get("status") == "alive":
            return {"status": "alive", "checks": outcomes}
    return {"status": "guest_unreachable", "checks": outcomes,
            "reason": f"the guest did not answer {checks} liveness checks {wait_s:g} s apart "
                      f"(last: {outcomes[-1].get('reason', outcomes[-1]['status'])}); nothing was staged"}


def _same_share_name(given: str, wanted: str) -> bool:
    """Whether two share remotes are one name, letter by letter without regard to case (each letter compared alone,
    so a letter never stands for two, and `Strasse` is not `Straße`)."""
    return len(given) == len(wanted) and all(a == b or a.lower() == b.lower() for a, b in zip(given, wanted))


def _net_use_row(line: str) -> bool:
    return bool(NET_USE_DRIVE.match(line.rstrip()) or NET_USE_DEVICELESS.match(line))


def net_use_rows(text: str) -> list[str] | None:
    """The table body of a `net use` listing in its English form, or None when the reply is not one.

    The form is either the column header `NET_USE_HEADER` (once), with the body after it (past a dashed separator,
    when present) up to the last line `NET_USE_DONE`; or the last line `NET_USE_EMPTY` with no header and no body.
    A connection line outside the body (before the header, or in an empty listing) makes the reply contradictory, so
    not in form. Blank lines are dropped."""
    lines = [line.rstrip() for line in text.splitlines() if line.strip()]
    if not lines:
        return None
    last, earlier = lines[-1].strip(), lines[:-1]
    if any(line.strip() in (NET_USE_DONE, NET_USE_EMPTY) for line in earlier):
        return None
    headers = [index for index, line in enumerate(earlier) if NET_USE_HEADER.match(line.strip())]
    if last == NET_USE_EMPTY:
        return [] if not headers and not any(_net_use_row(line) for line in earlier) else None
    if last != NET_USE_DONE or len(headers) != 1 or any(_net_use_row(line) for line in earlier[:headers[0]]):
        return None
    body = earlier[headers[0] + 1:]
    if body and set(body[0].strip()) == {"-"}:
        body = body[1:]
    return body


def parse_net_use(text: str, share_name: str) -> list[dict[str, Any]]:
    """The guest's mappings of the host share `share_name` in the table body of `net use` output (`net_use_rows`; none
    when the reply is not a listing in its English form).

    A line maps the share when, after an optional status word and the drive letter and colon, the rest of the line is
    exactly `\\\\Mac\\<share name>` (`_same_share_name`), followed by the end of the line, or by white space and
    then exactly the network column `NET_USE_NETWORK`. Anything else after the name (a further path, another word) is
    not the share. The lines after a row, up to the next row, continue it. A mapping whose reading the layout leaves
    open is returned with `ambiguous` set: a row with its network column on the line and a continuation too (the name
    could end in the column's text, or the provider be the wrapped one); a row ending at the name whose continuation is
    anything but the network column alone; or a share name that itself contains the network column's text. A drive
    letter is returned upper-case with its colon."""

    body = net_use_rows(text) or []
    wanted = f"\\\\Mac\\{share_name}"
    found = []
    for index, line in enumerate(body):
        match = NET_USE_DRIVE.match(line)
        if not match:
            continue
        rest = match.group("rest")
        if _same_share_name(rest, wanted):
            remote, inline = rest, False
        elif (rest.endswith(NET_USE_NETWORK) and rest[:-len(NET_USE_NETWORK)][-1:].isspace()
              and _same_share_name(rest[:-len(NET_USE_NETWORK)].rstrip(), wanted)):
            remote, inline = rest[:-len(NET_USE_NETWORK)].rstrip(), True
        else:
            continue
        following = []
        for other in body[index + 1:]:
            if _net_use_row(other):
                break
            following.append(other.strip())
        ambiguous = (NET_USE_NETWORK.lower() in share_name.lower()
                     or (following != [] if inline else following not in ([], [NET_USE_NETWORK])))
        found.append({"status": match.group("status") or "", "letter": match.group("letter").upper() + ":",
                      "remote": remote, "ambiguous": ambiguous})
    return found


def net_use_form(text: str) -> bool:
    """Whether a reply is a `net use` listing in its English form (`net_use_rows`)."""
    return net_use_rows(text) is not None


def exchange_share_letter(config: HarnessConfig, *, timeout_s: float | None = None,
                          runner: Callable[..., subprocess.CompletedProcess] = subprocess.run) -> dict[str, Any]:
    """The guest's own drive letter for the exchange folder's share, read from `net use` (read only, retried like a
    guard read on a transport answer). The share is named by the host exchange folder's name. `mapped` with the
    letter and the configured root, or `share_unmapped` (no letter), `share_ambiguous` (more than one letter),
    `share_unavailable` (the one mapping's status is neither OK nor blank) or `share_unknown` (the reply is not a
    `net use` listing in its English form, or a line of it cannot be read one way) with the reason. Starts and stops
    nothing."""

    if timeout_s is None:
        timeout_s = config.settings_guard.call_timeout_s if config.settings_guard else GUARD_CALL_TIMEOUT_S
    share = config.exchange_root.name
    base = {"share_name": share, "remote": f"\\\\Mac\\{share}", "configured_root": config.guest_exchange_root}
    steps: list[dict[str, Any]] = []
    try:
        record = _guest_read(config, runner, timeout_s, steps, "exchange_share", "net use")
    except Exception as failure:
        return base | {"status": "share_unknown", "reason": f"the share mapping call failed: {type(failure).__name__}"}
    if not (_answered(record) and net_use_form(record["stdout"])):
        return base | {"status": "share_unknown",
                       "reason": f"the guest did not answer `net use` in its form (exit status "
                                 f"{record['exit_status']}); nothing was staged"}
    mappings = parse_net_use(record["stdout"], share)
    if any(mapping["ambiguous"] for mapping in mappings):
        return base | {"status": "share_unknown",
                       "reason": f"a `net use` line for {base['remote']} can be read more than one way in its column "
                                 "layout; nothing was staged"}
    letters = sorted({mapping["letter"] for mapping in mappings})
    if not letters:
        return base | {"status": "share_unmapped",
                       "reason": f"the guest maps no drive letter to {base['remote']}; nothing was staged"}
    if len(letters) > 1:
        return base | {"status": "share_ambiguous", "letters": letters,
                       "reason": f"the guest maps more than one drive letter to {base['remote']} "
                                 f"({', '.join(letters)}); nothing was staged"}
    unusable = sorted({mapping["status"] for mapping in mappings if mapping["status"] not in NET_USE_USABLE})
    if unusable:
        return base | {"status": "share_unavailable", "letter": letters[0], "mapping_status": unusable[0],
                       "reason": f"the guest's mapping of {base['remote']} at {letters[0]} has the status "
                                 f"{', '.join(unusable)}, not OK; nothing was staged"}
    return base | {"status": "mapped", "letter": letters[0], "mapping_status": mappings[0]["status"],
                   "root": letters[0], "differs_from_configured": letters[0].casefold()
                   != config.guest_exchange_root.casefold()}


def open_live_session(config: HarnessConfig, *, runner: Callable[..., subprocess.CompletedProcess] = subprocess.run,
                      liveness: Callable[[HarnessConfig], dict[str, Any]] | None = None,
                      share: Callable[[HarnessConfig], dict[str, Any]] | None = None,
                      sleep: Callable[[float], None] | None = None) -> tuple[HarnessConfig | None, dict[str, Any]]:
    """Before the first guest step of a live run, and before anything is staged: the guest's liveness
    (`await_guest_alive`), then the exchange share's drive letter (`exchange_share_letter`). Returns the configuration
    the run uses, in memory only, with `guest_exchange_root` the guest's own letter for the share and `exchange_share`
    stating it beside the configured root (every run record carries it), and the session record; or None and the
    record of why the run stops (`guest_unreachable`, `share_unmapped`, `share_ambiguous`, `share_unavailable`,
    `share_unknown`, `share_root_refused`). The configuration file is never read again or written."""

    alive = await_guest_alive(config, liveness=liveness or (lambda c: check_guest_alive(c, runner=runner)),
                              sleep=sleep)
    record: dict[str, Any] = {"liveness": alive}
    if alive["status"] != "alive":
        return None, record | {"status": alive["status"], "reason": alive["reason"]}
    mapped = (share or (lambda c: exchange_share_letter(c, runner=runner)))(config)
    record["exchange_share"] = mapped
    if mapped.get("status") != "mapped":
        return None, record | {"status": mapped.get("status", "share_unknown"),
                               "reason": mapped.get("reason", "the share mapping was not established")}
    root = mapped["root"]
    if not isinstance(root, str) or not valid_guest_path(root + "\\", directory=True):
        return None, record | {"status": "share_root_refused", "reason": f"the guest's root for the share ({root!r}) "
                                                                        "is not an allow-listed guest path; nothing "
                                                                        "was staged"}
    stated = {key: mapped[key] for key in ("share_name", "remote", "configured_root", "letter", "mapping_status",
                                           "differs_from_configured")}
    session = dataclasses.replace(config, guest_exchange_root=root, exchange_share=stated)
    return session, record | {"status": "ready"}


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
    if not _answered(ids):
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
                  whole_key: bool = False, literal: tuple[str, str] | None = None,
                  config: HarnessConfig | None = None) -> list[dict[str, Any]]:
    """The writes that take the restore scope back to `before`.

    The scope is the key `scope_id`: its own values alone, or (`whole_key`) its values and everything strictly
    inside it. Subkeys that were removed are created first (shallowest first); then values that changed or were
    removed are set to their first-reading type and data, in the first reading's order, and values that appeared
    are deleted (except inside a subkey that appeared); last, subkeys that appeared are deleted (deepest first).
    `scope_id` itself is never created or deleted.

    ValueError (nothing is written) when any difference lies outside the scope, when a key or value name falls
    outside the allowed characters, or when a value cannot be written back exactly. The one exception is `literal`
    (`_literal_recent_value`): that exact (key id, value name), as REG_BINARY, is set or deleted through the
    write script's registry API (`_literal_write_command`), and its entry carries the command that reads it back (the
    read script's `regread`); both need `config`. With `config`, a REG_BINARY value whose `reg add` would exceed
    `DATA_FILE_THRESHOLD` is set through the same registry API, its data in a data file. With `config`, every command of the plan is checked against the one length
    rule (`_checked_command`) while the plan is made: a command over it refuses the whole restore before any write.
    """

    def inside(key_id: str, *, strictly: bool = False) -> bool:
        if not whole_key:
            return key_id == scope_id and not strictly
        return key_id.startswith(scope_id + "\\") or (key_id == scope_id and not strictly)

    def printed_key(printed: str, key_id: str) -> str:
        if not REG_PRINTED_KEY_PATTERN.match(printed) or _registry_key_id(printed) != key_id:
            raise ValueError(f"{printed} cannot be named in a guest command")
        return printed

    def _literal_readback(printed: str, name: str) -> str:
        if config is None:
            raise ValueError(f"{printed}\\{name} cannot be read back without the harness configuration")
        return _literal_readback_command(config, printed, name)

    def _literal_write(printed: str, name: str, action: str, data: str | None = None) -> str:
        if config is None:
            raise ValueError(f"{printed}\\{name} cannot be written back exactly without the harness configuration")
        return _literal_write_command(config, printed, name, action, data)

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
    # The literal value, wherever it differs (a subkey that appeared included), must be REG_BINARY in every reading
    # that has it; otherwise the whole restore is refused before any write.
    if literal in differing_values:
        for reading in (before, after):
            found = reading["values"].get(literal)
            if found is not None and found[1] != "REG_BINARY":
                raise ValueError(f"{found[0]}\\{literal[1]} ({found[1]}) cannot be written back exactly")
    added = sorted((key for key in after["keys"] if key not in before["keys"]), key=lambda key: -key.count("\\"))
    removed = sorted((key for key in before["keys"] if key not in after["keys"]), key=lambda key: key.count("\\"))

    plan = []
    for key_id in removed:
        printed = printed_key(before["keys"][key_id], key_id)
        plan.append({"key": printed, "action": "create_key", "restores": "present", "replaces": None,
                     "command": f"reg add {_quote(printed)} /f"})
    for item, (printed, kind, data) in before["values"].items():
        if item in differing_values:
            found = after["values"].get(item)
            # Both readings of the literal value must be REG_BINARY where present; any other type keeps the refusal.
            exact = item == literal and kind == "REG_BINARY" and (found is None or found[1] == "REG_BINARY")
            if kind not in RESTORABLE_DATA or not RESTORABLE_DATA[kind].match(data) or item[1] == "(Default)" \
                    or not (REG_VALUE_NAME_PATTERN.match(item[1]) or exact):
                raise ValueError(f"{printed}\\{item[1]} ({kind}) cannot be written back exactly")
            printed = printed_key(printed, item[0])
            command = f"reg add {_quote(printed)} /v {_quote(item[1])} /t {kind} /d {_quote(data)} /f"
            if exact or (config is not None and kind == "REG_BINARY"
                         and _guest_command_length(config.vm_name, command) > DATA_FILE_THRESHOLD):
                # Through the registry API, its data in a data file when it would not fit one command.
                command = _literal_write(printed, item[1], "set", data.upper())
            entry = {"value": f"{printed}\\{item[1]}", "action": "set", "target": (printed, item[1]),
                     "restores": {"type": kind, "data": data},
                     "replaces": {"type": found[1], "data": found[2]} if found else None, "command": command}
            if exact:
                entry["readback_command"] = _literal_readback(printed, item[1])
            plan.append(entry)
    for item, (printed, kind, data) in after["values"].items():
        if item not in before["values"] and item[0] not in added:
            exact = item == literal and kind == "REG_BINARY"
            if item[1] == "(Default)" or not (REG_VALUE_NAME_PATTERN.match(item[1]) or exact):
                raise ValueError(f"{printed}\\{item[1]} cannot be deleted exactly")
            printed = printed_key(printed, item[0])
            entry = {"value": f"{printed}\\{item[1]}", "action": "delete", "target": (printed, item[1]),
                     "restores": None,
                     "replaces": {"type": kind, "data": data},
                     "command": f"reg delete {_quote(printed)} /v {_quote(item[1])} /f"
                     if not exact else _literal_write(printed, item[1], "delete")}
            if exact:
                entry["readback_command"] = _literal_readback(printed, item[1])
            plan.append(entry)
    for key_id in added:
        printed = printed_key(after["keys"][key_id], key_id)
        subtree = {item: value[1:] for item, value in after["values"].items()
                   if item[0] == key_id or item[0].startswith(key_id + "\\")}
        plan.append({"key": printed, "action": "delete_key", "subtree": subtree, "restores": None,
                     "replaces": "present",
                     "command": f"reg delete {_quote(printed)} /f"})
    if config is not None:
        for entry in plan:
            for key in ("command", "readback_command"):
                if key in entry:
                    _checked_command(config, entry[key])
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

    def refuse_foreign_instance(name: str, caller: Callable[..., dict[str, Any]] | None = None) -> None:
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
        refuse_foreign_instance(f"restore_check_before_retry_{number}", budgeted_step)
        within(f"settings_readback_{number}")
        if entry["action"] == "delete_key":
            state_found, found = _read_subtree(budgeted_step, f"settings_readback_{number}", entry["key"])
            intended = state_found == "absent"
            as_left = state_found == "present" and found == entry["subtree"]
        else:
            state_found, found = _read_value(budgeted_step, f"settings_readback_{number}", *entry["target"],
                                             command=entry.get("readback_command"))
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
                    refuse_foreign_instance(f"restore_check_before_write_{number}", budgeted_step)
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
                refuse_foreign_instance("restore_check_after_writes", budgeted_step)
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
            refuse_foreign_instance("restore_check_before_success")
            done = {"status": "settings_restored" if guard.restores_whole_key else "recent_files_restored",
                    "write_log": log, "restored_values": len(plan)}
            for key in ("signal_deferral", "deferred_signals", "failed_write_stderr"):
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
                "section_budget_exceeded", "failed_write_stderr"):
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
    restore was entered, else `settings_check_failed` with nothing written). The write log is always included.
    While the live record's start does not permit restoration (unresolved), the status is `start_unresolved`."""

    classified = _classify(live)
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
    if state.get("failed_write_stderr"):
        result.setdefault("failed_write_stderr", state["failed_write_stderr"])
    if not classified["restore"]:
        result = dict(result) | {"status": "start_unresolved",
                                 "reason": "restoration inhibited: start unresolved; nothing was written"}
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
    if result is None or not _classify(live)["restore"]:  # an unresolved start takes precedence over anything else
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
                 write: Callable[[str, str], dict[str, Any]] | None = None,
                 start_state: str | None = None) -> tuple[dict[str, Any], BaseException | None]:
    """The run record's `settings_guard`, and an exception (cancellation included) deferred from the restore.

    A `start_state` whose `disposition` does not permit restoration (unresolved: the attempt is not established
    as not started, completed or failed and quiescent, a cancellation during it included) inhibits every restore:
    the status is `start_unresolved`, nothing is written, and the reading after the run and the process check are
    recorded for diagnosis only.

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
    if start_state is not None and not disposition(start_state)["restore"]:
        if after is not None:
            result["changed_values"] = settings_difference(settings_before, after)
        result.update(status="start_unresolved", reason="restoration inhibited: start unresolved (the attempt was not "
                                                        "established as quiescent); nothing was written")
        return result, None
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
        plan = _restore_plan(settings_before, after, scope_id, whole_key=guard.restores_whole_key,
                             literal=_literal_recent_value(config), config=config)
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
                           | _failed_write_stderr(item) for item in steps]
        return result

    # No attempt is made in this call, so nothing of it can still be running: its guard result is never
    # "start_unresolved", and a termination carries no unresolved start.
    live: dict[str, Any] = {"start": "not_dispatched", "guard_state": state}
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
            plan = _restore_plan(intended, current, root, whole_key=True, literal=_literal_recent_value(config),
                                 config=config)
            exchange = config.exchange_root  # the read and write scripts in place before anything is sent
            for guest_script in (READ_SCRIPT, WRITE_SCRIPT):
                _install_guest_script(exchange, contained_dir(exchange, exchange.resolve() / HARNESS_DIR), guest_script)
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



# ---------------------------------------------------------------------------------------------------------------
# For an outer caller: the session's settings record, the read-only state report, and the controlled exercise's
# operations on harmless resources. All go through the same builders, the one length rule and the echo.
# ---------------------------------------------------------------------------------------------------------------
def read_settings_record(config: HarnessConfig, *, runner: Callable[..., subprocess.CompletedProcess] = subprocess.run,
                         steps: list[dict[str, Any]] | None = None) -> dict[str, dict] | None:
    """One read-only reading of the whole configured settings key, by the guard's own reader (`reg query <key> /s`,
    `_settings_snapshot`), for a record an outer caller takes at the start of a session and again before it
    suspends, and compares on the host (`settings_difference`). None when the key cannot be read whole."""
    guard = config.settings_guard
    if guard is None:
        raise ValueError("no settings key is configured")
    steps = [] if steps is None else steps
    return _settings_snapshot(config, lambda name, command: _guest_read(config, runner, guard.call_timeout_s, steps,
                                                                       name, command), "settings_record")


# The small files the state report reads whole through the read script's `chunk`: a claim or a terminal report.
STATE_FILE_MAX_BYTES = 1 << 16


def _read_small_file(config: HarnessConfig, runner: Callable[..., subprocess.CompletedProcess],
                     steps: list[dict[str, Any]], name: str, path: str) -> tuple[str, str]:
    """("text", its ASCII text), ("absent", ""), or ("unreadable", "") for anything else (a sharing violation, a file
    larger than one read, an answer out of form, a digest that does not match), through one `chunk` read."""
    record = _guest_step(config, runner, RECONCILE_CALL_TIMEOUT_S, steps, name,
                         _read_command(config, "chunk", {"path": path, "offset": 0, "length": STATE_FILE_MAX_BYTES}))
    match = CHUNK_ANSWER.match(_body(record)) if _answered(record) else None
    if match is None:
        return "unreadable", ""
    if match["size"] is None:
        return "absent", ""
    try:
        data = base64.b64decode(match["data"], validate=True)
        text = data.decode("ascii")
    except (ValueError, UnicodeDecodeError):
        return "unreadable", ""
    whole = int(match["size"]) == len(data) == int(match["length"]) and int(match["offset"]) == 0
    if not whole or _sha256(data) != match["sha256"] or _sha256(data) != match["chunk_sha256"]:
        return "unreadable", ""
    return "text", text


def read_attempt_state(config: HarnessConfig, work: str, token: str, *,
                       runner: Callable[..., subprocess.CompletedProcess] = subprocess.run,
                       steps: list[dict[str, Any]] | None = None, prefix: str = "") -> dict[str, Any]:
    """The attempt's state after a run, from the read script alone, in one pass and without the fence: independent
    of what the attempt or any changing script reported about itself.

    `claim` (the claim read whole): `script` (this token's claim in the script's form: owner, process id, creation
    time and nonce), `host` (this token's claim as the host's fence writes it), `other` (present, not this token's
    claim in either form), `absent` or `unreadable`. `launcher` (only for a script claim): `ended` (no process with
    the claim's id, or one created at another time), `running`, or `unknown`. `job`: `absent` (no member, no
    handle), `present`, or `unknown`. `terminal` (the terminal report read whole): `complete` (this token's report in
    its form), `other`, `absent` or `unreadable`.

    `classified`, in the reconciliation's own states: `completed` when the claim is the script's, the launcher has
    ended, the job is absent and the terminal report is complete; `partial` when the same holds with no terminal
    report; `fenced` when the claim is the host's (the reconciliation calls that not_started only for its own
    fence's nonce); `unknown` for anything else. `success` is `classified == "completed"`."""
    if prefix not in ("", EXERCISE_PREFIX) or not ATTEMPT_TOKEN_PATTERN.match(token):
        raise ValueError("an attempt is named by a 32-hex token, with no prefix but the exercise's")
    steps = [] if steps is None else steps
    path = lambda suffix: _win(work, prefix + token + suffix)  # noqa: E731
    state: dict[str, Any] = {"token": token, "work": work, "launcher": "not_applicable"}
    found, text = _read_small_file(config, runner, steps, "state_claim", path(".claim"))
    claim = _parse_claim(text, token) if found == "text" else None
    state["claim"] = claim["owner"] if claim else ("other" if found == "text" else found)
    if claim and claim["owner"] == "script":
        state["claim_identity"] = {"pid": int(claim["pid"]), "created": claim["created"]}
        record = _guest_step(config, runner, RECONCILE_CALL_TIMEOUT_S, steps, "state_launcher",
                             _read_command(config, "cim", {"pid": int(claim["pid"])}))
        match = CIM_ANSWER.match(_body(record)) if _answered(record) else None
        state["launcher"] = "unknown" if match is None else (
            "ended" if match["pid"] is None or match["created"] != claim["created"] else "running")
    record = _guest_step(config, runner, RECONCILE_CALL_TIMEOUT_S, steps, "state_job",
                         _read_command(config, "job", {"job": f"swbpipe-{token}"}))
    match = JOB_ANSWER.match(_body(record)) if _answered(record) else None
    state["job"] = "unknown" if match is None else ("absent" if match["state"] == "absent" else
                                                   "present" if match["state"].startswith("present") else "unknown")
    found, text = _read_small_file(config, runner, steps, "state_terminal", path(".finished"))
    report = _parse_finished(text, token) if found == "text" else None
    state["terminal"] = "complete" if report is not None else ("other" if found == "text" else found)
    if report is not None:
        state["terminal_status"] = report.get("status")
    quiescent = state["claim"] == "script" and state["launcher"] == "ended" and state["job"] == "absent"
    state["classified"] = ("completed" if quiescent and state["terminal"] == "complete" else
                           "partial" if quiescent and state["terminal"] == "absent" else
                           "fenced" if state["claim"] == "host" else "unknown")
    state["success"] = state["classified"] == "completed"
    return state


def exercise_names(date: str) -> dict[str, str]:
    """The exercise's own folder and registry key for the session's UTC date (YYYYMMDD): the folder
    C:\\oracle\\SWBPIPE-harness-exercise-<date>, and HKCU\\Software\\SWBPIPE-harness-exercise-<date>."""
    if not re.fullmatch(r"20[0-9]{2}(0[1-9]|1[0-2])(0[1-9]|[12][0-9]|3[01])", date or ""):
        raise ValueError("the exercise is named by the session's UTC date, YYYYMMDD")
    return {"folder": f"{EXERCISE_PARENT}\\{EXERCISE_PREFIX}{date}",
            "key": f"HKCU\\Software\\{EXERCISE_PREFIX}{date}",
            "printed_key": f"HKEY_CURRENT_USER\\Software\\{EXERCISE_PREFIX}{date}",
            "registry_key": f"Software\\{EXERCISE_PREFIX}{date}"}


def exercise_claim_command(config: HarnessConfig, date: str, name: str) -> str:
    """The exclusive create of one exercise claim, SWBPIPE-harness-exercise-<name>, in the exercise folder, through
    the write script's `claim` (the host's fence's own operation). Its content says whose it is and what for."""
    if not re.fullmatch(r"[A-Za-z0-9-]{1,40}", name or ""):
        raise ValueError("an exercise claim is named by letters, digits and hyphens")
    folder = exercise_names(date)["folder"]
    content = (f"swbpipe-claim v1\r\nexercise={EXERCISE_PREFIX}{name}\r\n"
               f"owner=SWBPIPE harness controlled exercise {date}\r\n")
    return _write_command(config, "claim", {"folder": folder, "name": EXERCISE_PREFIX + name, "content": content})


def exercise_registry_command(config: HarnessConfig, date: str, action: str, value_name: str | None = None,
                              data: str | None = None) -> str:
    """One registry operation on the exercise's own key, and on no other: `keycreate`, `keydelete`, and `set` (a
    REG_BINARY value, upper-case hex), `delete` and `read` of one value in it, through the same operations the
    restore uses (`_literal_write_command`, `regread`). The key comes from the date alone, so no other key, the
    program's included, can be named."""
    names = exercise_names(date)
    if action in ("keycreate", "keydelete"):
        if value_name is not None or data is not None:
            raise ValueError("a key operation names no value")
        return _write_command(config, action, {"key": names["registry_key"]})
    if not REG_VALUE_NAME_PATTERN.match(value_name or "") or action not in ("set", "delete", "read"):
        raise ValueError(f"no exercise registry operation {action!r} on {value_name!r}")
    if action == "read":
        return _read_command(config, "regread", {"key": names["registry_key"], "name": value_name})
    return _literal_write_command(config, names["printed_key"], value_name, action, data)


def exercise_start_command(config: HarnessConfig, date: str, token: str) -> str:
    """The exercise's start: the round-trip script in its exercise mode, its claim, job and folder named with the
    exercise prefix in the exercise folder, starting the fixed harmless process in place of the program."""
    if not ATTEMPT_TOKEN_PATTERN.match(token or ""):
        raise ValueError("an attempt is named by a 32-hex token")
    script = _win(config.guest_exchange_root, HARNESS_DIR, _sha256(GUEST_SCRIPT.read_bytes()), GUEST_SCRIPT.name)
    return _checked_command(config, f"{POWERSHELL_COMMAND} -File {_quote(script)} -Exercise -AttemptToken {token} "
                                    f"-WorkRoot {_quote(exercise_names(date)['folder'])}")


def install_guest_scripts(config: HarnessConfig) -> dict[str, str]:
    """Put the round-trip, read and write scripts at their content-addressed paths (add-only); their digests."""
    root = config.exchange_root
    harness_dir = contained_dir(root, root.resolve() / HARNESS_DIR)
    return {script.name: _install_guest_script(root, harness_dir, script)["sha256"]
            for script in (GUEST_SCRIPT, READ_SCRIPT, WRITE_SCRIPT)}


def run_exercise_start(config: HarnessConfig, date: str, *,
                       runner: Callable[..., subprocess.CompletedProcess] = subprocess.run,
                       token: str | None = None, timeout_s: float = 120.0) -> dict[str, Any]:
    """Dispatch the exercise's start once (never retried) and reconcile it exactly as a run's attempt is reconciled,
    in the exercise folder with the exercise prefix. Returns the token, the dispatch step, the disposition
    (`disposition`, the reconciliation's state), the terminal report's status and every step. An outer caller that
    wants to see the live job from a separate call passes its own `token` and queries `swbpipe-<token>` with the read
    script's `job` while this call runs."""
    install_guest_scripts(config)
    token = token or secrets.token_hex(16)
    folder = exercise_names(date)["folder"]
    command = exercise_start_command(config, date, token)
    steps: list[dict[str, Any]] = []
    dispatch = _guest_step(config, runner, timeout_s, steps, "exercise_attempt", command, attempts=1)
    resolution = _Reconciler(config, runner, steps, folder, token, time.monotonic() + RECONCILE_DEADLINE_S,
                             prefix=EXERCISE_PREFIX).run()
    settled = resolution["disposition"]
    if settled == "completed" and _report_contradicted(resolution["report"], dispatch):
        settled = "contradicted"
    start = settled if settled != "unknown" else "unresolved"
    return {"token": token, "dispatch": {key: dispatch.get(key) for key in ("exit_status", "stdout", "stderr")},
            "disposition": disposition(start), "reason": resolution["reason"],
            "report_status": (resolution.get("report") or {}).get("status"), "steps": steps}



# The exercise key's presence and the exercise folder's listing, decided without any message text: what Windows prints
# on stderr is in its display language, so each command discards it (`2>nul`) and prints, on stdout, either fixed ASCII
# sentinels of its own or bare names. Only the exit status and the shape of stdout are judged.
KEY_PRESENCE_ANSWER = re.compile(r"^SWBPIPE-KEY v1 (?P<state>present|absent)\r?\n?\Z")
FOLDER_ABSENT_ANSWER = re.compile(r"^SWBPIPE-LIST v1 absent\r?\n?\Z")


def exercise_key_presence_command(key: str) -> str:
    """`reg query` of the key with both streams discarded, then a fixed sentinel chosen by reg's exit status (0 when
    the key was read, 1 otherwise), which `if errorlevel` tests when it runs."""
    return (f"reg query {_quote(key)} >nul 2>nul & if errorlevel 1 (echo SWBPIPE-KEY v1 absent) "
            f"else (echo SWBPIPE-KEY v1 present)")


def exercise_folder_listing_command(folder: str) -> str:
    """The folder's entries by bare name (`dir /b /a`, its message discarded), or a fixed sentinel when the folder
    does not exist (`if exist "<folder>\\"`, a test of the file system, true only for a folder)."""
    return (f"if exist {_quote(folder + chr(92))} (dir /b /a {_quote(folder)} 2>nul) "
            f"else (echo SWBPIPE-LIST v1 absent)")


def _key_presence(record: Mapping[str, Any]) -> str:
    """`present` or `absent` from the sentinel alone (blank stderr); `unreadable` for anything else. `present` needs
    exit 0; `absent` is taken with exit 0 or 1, since `echo` leaves reg's errorlevel 1 and `cmd /c` may end with it."""
    if (record.get("stderr") or "").strip() or record.get("echo"):
        return "unreadable"
    match = KEY_PRESENCE_ANSWER.match((record.get("stdout") or "").strip() + "\n")
    if match is None or record.get("exit_status") not in ((0,) if match["state"] == "present" else (0, 1)):
        return "unreadable"
    return match["state"]


def _folder_listing(record: Mapping[str, Any]) -> list[str] | str:
    """The listing's answer: `absent` (exit 0, stdout the sentinel alone); the entries (exit 0, every line a bare
    name); `[]` (exit 1, stdout empty: `dir` found nothing in a folder that exists); `unreadable` for anything else,
    any text on stderr included. No message text is read."""
    stdout = record.get("stdout") or ""
    lines = [line.strip() for line in stdout.splitlines() if line.strip()]
    if (record.get("stderr") or "").strip() or record.get("echo"):
        return "unreadable"
    if record["exit_status"] == 0 and FOLDER_ABSENT_ANSWER.match(stdout.strip() + "\n"):
        return "absent"
    if record["exit_status"] == 0 and lines and all(re.fullmatch(r"[A-Za-z0-9 ._()-]+", line) for line in lines) \
            and not any(line.startswith("SWBPIPE-LIST") for line in lines):
        return lines
    if record["exit_status"] == 1 and not lines:
        return []
    return "unreadable"


def read_state_report(config: HarnessConfig, *, runner: Callable[..., subprocess.CompletedProcess] = subprocess.run,
                      case_id: str | None = None, token: str | None = None, exercise_date: str | None = None,
                      settings_before: Mapping[str, Mapping] | None = None,
                      storage_folders: tuple[str, ...] = ()) -> dict[str, Any]:
    """The read-only state report after a run, independent of what any changing script reported about itself, for an
    outer caller that needs no attempt to call it. Each part is present when asked for:

    - `attempt` (with `token`; the case's work folder, or with `exercise_date` the exercise folder and prefix):
      `read_attempt_state`, its `success` the predicate a run must meet;
    - `settings` (with `settings_before`): the key read again whole and compared with that record;
    - `storage` (with `storage_folders`): the read script's storage answer for each folder and the predicate;
    - `exercise` (with `exercise_date`): the exercise key `absent` or `present` (or `unreadable`), and the exercise
      folder's entries by name (`absent` when the folder is).

    Reads only: the read script, `reg query` and `dir /b /a`; the key's presence and the folder's listing are decided
    from exit status and the shape of stdout, never from message text (`exercise_key_presence_command`,
    `exercise_folder_listing_command`)."""
    steps: list[dict[str, Any]] = []
    report: dict[str, Any] = {"steps": steps}

    def read(name: str, command: str) -> dict[str, Any]:
        return _guest_step(config, runner, RECONCILE_CALL_TIMEOUT_S, steps, name, command)

    if token is not None:
        if exercise_date is not None:
            work, prefix = exercise_names(exercise_date)["folder"], EXERCISE_PREFIX
        elif case_id is not None and CASE_ID_PATTERN.match(case_id):
            work, prefix = _win(config.guest_work_root, case_id), ""
        else:
            raise ValueError("an attempt's state is read for a case id or for the exercise")
        report["attempt"] = read_attempt_state(config, work, token, runner=runner, steps=steps, prefix=prefix)
    if settings_before is not None:
        after = read_settings_record(config, runner=runner, steps=steps)
        differences = settings_difference(settings_before, after) if after is not None else None
        report["settings"] = {"read": after is not None, "differences": differences,
                              "unchanged": differences == []}
    if storage_folders:
        folders = list(storage_folders)
        record = read("state_storage", _read_command(config, "storage", {"folders": folders}))
        entries = parse_storage(_body(record), folders) if _answered(record) else None
        report["storage"] = {"entries": entries,
                             "meets": entries is not None and all(item["meets"] for item in entries)}
    if exercise_date is not None:
        names = exercise_names(exercise_date)
        report["exercise"] = {
            "key": _key_presence(read("state_exercise_key", exercise_key_presence_command(names["key"]))),
            "folder_entries": _folder_listing(read("state_exercise_folder",
                                                   exercise_folder_listing_command(names["folder"])))}
    return report


def _round_trip(config: HarnessConfig, case_id: str, mbf_text: str, paths: dict[str, Path], harness_dir: Path,
                timeout_s: float, runner: Callable[..., subprocess.CompletedProcess],
                gui_fallback: bool = True) -> dict[str, Any]:
    # A guarded round trip turns SIGTERM into a catchable cancellation for its whole lifetime, so a termination
    # request is recorded like a Ctrl-C instead of ending the process with the guard's log unwritten.
    # The same holds for an exception at any reporting or persistence step: it leaves with the guard result attached
    # and written once more (`_recover_guard_result`).
    guarded = config.settings_guard is not None
    live: dict[str, Any] = {"start": "not_dispatched"}  # the invocation-owned live record
    with _catchable_termination(guarded, live):
        try:
            return _round_trip_body(config, case_id, mbf_text, paths, harness_dir, timeout_s, runner, gui_fallback,
                                    live)
        except BaseException as error:
            _carry_disposition(error, live)  # on every exit, from the one source
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
    # The inputs are staged, and immutable, before any dispatch: the MBF once per round trip, the script add-only.
    write_private(root, paths["caepipe_in"] / "model.mbf", mbf_bytes)
    script = _install_guest_script(root, harness_dir)
    read_script = _install_guest_script(root, harness_dir, READ_SCRIPT)  # every read-only query runs it
    write_script = _install_guest_script(root, harness_dir, WRITE_SCRIPT)  # the fence and the registry API writes
    out = paths["caepipe_out"]
    _clear_outputs(root, out)

    guest_case = _win(config.guest_exchange_root, "cases", case_id)
    staged_mbf = _win(guest_case, "caepipe_in", "model.mbf")
    work = _win(config.guest_work_root, case_id)
    share_script = _win(config.guest_exchange_root, HARNESS_DIR, script["sha256"], GUEST_SCRIPT.name)
    guarded = config.settings_guard is not None

    def attempt_command(token: str) -> str:
        return _checked_command(config, (
            f"{POWERSHELL_COMMAND} -File {_quote(share_script)} "
            f"-Exe {_quote(config.guest_executable)} -Mbf {_quote(staged_mbf)} -AttemptToken {token} "
            f"-ExpectedMbfSha256 {mbf_sha256} -WorkRoot {_quote(work)}"
            + (f" -PidFile {_quote(_win(work, token, OWNED_PIDS_FILE))}" if guarded else "")
            + ("" if gui_fallback else " -NoGuiFallback")))

    steps: list[dict[str, Any]] = []
    guard_state: dict[str, Any] = {}
    live = {"start": "not_dispatched"} if live is None else live
    live["guard_state"] = guard_state  # what the outer recovery and the termination handler report from
    # Every guard call (registry reads and writes, process queries) has the guard's own short timeout; the attempt
    # keeps `timeout_s`.
    guard_timeout = config.settings_guard.call_timeout_s if config.settings_guard else timeout_s

    def guard_step(name: str, command: str, deadline: float | None = None) -> dict[str, Any]:
        return _guest_read(config, runner, guard_timeout, steps, name, command, deadline)

    def write(name: str, command: str, deadline: float | None = None) -> dict[str, Any]:
        return _guest_write(config, runner, guard_timeout, steps, name, command, guard_state, deadline)

    # Precondition: no instance of the tool may be running before ours starts; then the saved settings are read.
    # Without both, nothing runs.
    blocked = None
    if guarded:
        pre = _precondition(config, guard_step, "precondition_process_list")
        blocked = None if pre["status"] == "clear" else pre
    settings_before = _settings_snapshot(config, guard_step, "settings_before") if guarded and blocked is None else None
    # The start's disposition lives only in `live["start"]` (`_classify`): "not_dispatched" until an attempt is
    # dispatched, "unresolved" from just before the dispatch until reconciliation establishes a disposition, which
    # replaces it before any fallible line. Anything that interrupts in between leaves it unresolved, which inhibits
    # every restore (`_guard_check`).
    attempts: list[dict[str, Any]] = []
    dispatches: list[tuple[Path, dict[str, Any]]] = []  # each attempt's folder and dispatch step, for the records
    resolution: dict[str, Any] | None = None
    token: str | None = None
    dispatch: dict[str, Any] = {"exit_status": None, "stdout": "", "stderr": ""}
    guest_log = None
    guest_mbf_sha256 = None
    input_verified = False
    collection_problem = None
    attempt_out: Path | None = None
    settings = None
    error: BaseException | None = None
    try:
        if not guarded or (blocked is None and settings_before is not None):
            for number in range(1, ATTEMPT_DISPATCHES + 1):
                token = secrets.token_hex(16)
                attempt_out = contained(root, out / token)
                attempt_out.mkdir()  # fresh: a token is never reused, and nothing already stored is rewritten
                entry: dict[str, Any] = {"attempt_token": token, "dispatch": number}
                attempts.append(entry)
                command = attempt_command(token)  # built and checked while nothing of this attempt is sent
                _set_start(live, "unresolved")  # before the dispatch can have any effect
                # The attempt changes guest state, so it is dispatched once and never retried, whatever it answers.
                dispatch = _guest_step(config, runner, timeout_s, steps, f"attempt_{number}", command, attempts=1)
                # The one deadline, from the moment the reply arrived (or failed), through every nested read.
                resolution = _Reconciler(config, runner, steps, work, token,
                                         time.monotonic() + RECONCILE_DEADLINE_S).run()
                settled = resolution["disposition"]
                if settled == "completed" and _report_contradicted(resolution["report"], dispatch):
                    settled = "contradicted"
                if settled != "unknown":
                    _set_start(live, settled)  # the reconciled disposition replaces unresolved before anything else
                entry["dispatch_exit_status"] = dispatch["exit_status"]
                dispatches.append((attempt_out, dispatch))
                entry.update(disposition=settled, reason=resolution["reason"], evidence=list(resolution["evidence"]))
                if settled != "not_started":
                    break  # unknown leaves the start unresolved
            if _classify(live)["start"] in ("completed", "contradicted"):
                guest_log = resolution["report"]  # retained whole, the contradicted one included
                write_private(root, attempt_out / "guest_log.json", json.dumps(guest_log, indent=2, sort_keys=True)
                              + "\n")
            if _classify(live)["start"] == "completed":
                # The guest script reports the SHA-256 of the MBF it actually handed the program; outputs are
                # attributed to this MBF only when that hash equals the host's.
                reported = guest_log.get("mbf_sha256")
                guest_mbf_sha256 = reported.lower() if isinstance(reported, str) else None
                input_verified = guest_mbf_sha256 == mbf_sha256
                if input_verified:
                    reported_print = guest_log.get("csv_path")
                    print_name = (reported_print.replace("/", "\\").split("\\")[-1]
                                  if isinstance(reported_print, str) else "")
                    names = ([print_name] if PRINT_NAME_PATTERN.match(print_name) else []) + list(OUTPUT_NAMES)
                    files, collection_problem = _collect_attempt(config, runner, steps, _win(work, token), names)
                    _refuse_links(root, out)  # again just before the host writes into the layer
                    for name, data in files.items():
                        write_private(root, attempt_out / name, data)
    except BaseException as caught:  # re-raised below, after the checks
        error = caught
    # The guard's post-run work runs in normal signal context: only the restore's registry-mutation section defers
    # signals. Callee exceptions here are kept (the first one raised), and the guard result is attached to the
    # error before any record I/O, so a signal or a failure during the record cannot lose it.
    # Whatever happened above: confirm that what the guest started has ended, then compare the whole settings
    # key with its reading before the run. Each check is guarded on its own, so a failure of one never skips the
    # other or the result. While the start is unresolved these are read-only diagnosis: no restore.
    if guarded:
        deferred = None
        post_run_errors: list[BaseException] = []
        try:
            try:
                # The owned-process check only when an attempt may have started the program.
                processes = (_owned_processes(config, guard_step, _win(work, token), guest_log)
                             if token is not None
                             and _classify(live)["start"] in ("unresolved", "completed", "partial", "contradicted")
                             else {"status": "not_started"})
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
                                                  write, start_state=_classify(live)["start"])
        except BaseException as caught:  # cancellation during the checks or escaping the restore's failure path
            primary = guard_state.get("primary_error")
            deferred = primary if primary is not None else caught
            log = guard_state.get("write_log") or []
            settings = dict(guard_state.get("result") or {})
            if log or settings or guard_state.get("restore_entered"):
                settings.setdefault("write_log", log)
                settings.setdefault("writes_completed", sum(1 for entry in log if entry["completed"]))
                if guard_state.get("failed_write_stderr"):
                    settings.setdefault("failed_write_stderr", guard_state["failed_write_stderr"])
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
            elif not _classify(live)["restore"]:
                settings = {"status": "start_unresolved",
                            "reason": f"restoration inhibited: start unresolved; the checks after the run were "
                                      f"interrupted ({type(caught).__name__}); nothing was written"}
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
        for folder, sent in dispatches:  # each attempt's own output, in its own folder
            write_private(root, folder / "guest_stdout.txt", sent["stdout"])
            write_private(root, folder / "guest_stderr.txt", sent["stderr"])
        relative = f"caepipe_out/{token}" if token is not None and attempt_out is not None else None
        guest_csv = guest_log.get("csv_path") if isinstance(guest_log, dict) else None
        observed = None
        csv_name = guest_csv.replace("/", "\\").split("\\")[-1] if isinstance(guest_csv, str) else ""
        print_problem = None
        if PRINT_NAME_PATTERN.match(csv_name) and input_verified and error is None and attempt_out is not None:
            candidate = contained(root, attempt_out / csv_name)
            if candidate.is_file():
                observed = candidate
                if csv_name.lower() == PRINT_TXT_NAME:
                    print_problem = _print_layout_problem(candidate)
                    observed = None if print_problem else observed
        status = guest_log.get("status") if isinstance(guest_log, dict) else None
        start = _classify(live)["start"]
        if error is not None:
            route = "harness_error"
        elif blocked is not None:
            route = blocked["status"]
        elif guarded and settings_before is None:
            route = "settings_check_failed"
        elif start == "unresolved":
            route = "start_unresolved"
        elif start == "not_started":
            route = "not_started"
        elif start == "partial":
            route = "start_partial"
        elif start == "contradicted":
            route = REPORT_REPLY_CONTRADICTION
        elif guest_mbf_sha256 is None:  # the guest reported no hash: nothing was compared, so nothing "mismatched"
            route = str(status) if status and status not in CSV_ROUTES else "input_hash_unreported"
        elif not input_verified:
            route = "input_hash_mismatch"
        elif collection_problem is not None:
            route = "collection_failed"
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

        collected = sorted(path.name for path in attempt_out.iterdir()
                           if path.name.startswith("model") and path.is_file() and not path.is_symlink()) \
            if attempt_out is not None else []
        attempt_dir = _win(work, token) if token is not None else None
        record = {
            "document_kind": RUN_RECORD_KIND,
            "schema_version": "2",
            "deliverable": "DEL-17-05",
            "case_id": case_id,
            "configuration_surface": {
                "environment_variable": ENV_VAR,
                "config_file": CONFIG_NAME,
                "checked": list(config.checked),
                "warnings": list(config.warnings),
                # The guest's own letter for the exchange share, used for this run in place of the configured root
                # where they differ; the configuration file is not changed (`open_live_session`).
                **({"exchange_share": dict(config.exchange_share)} if config.exchange_share is not None else {}),
            },
            "executable_path_as_configured": config.guest_executable,
            ACKNOWLEDGEMENT_KEY: config.acknowledged,
            "mbf": {"path": "caepipe_in/model.mbf", "guest_staged_path": staged_mbf,
                    "guest_path": _win(attempt_dir, "model.mbf") if attempt_dir else None, "sha256": mbf_sha256,
                    "bytes": len(mbf_bytes)},
            "guest_mbf_sha256": guest_mbf_sha256,
            "input_hash_verified": input_verified,
            "guest_script": {"repository_path": GUEST_SCRIPT_REPOSITORY_PATH,
                             "exchange_copy": script["exchange_copy"], "sha256": script["sha256"],
                             "run_from": "exchange_share"},
            "read_script": {"repository_path": READ_SCRIPT_REPOSITORY_PATH,
                            "exchange_copy": read_script["exchange_copy"], "sha256": read_script["sha256"],
                            "run_from": "exchange_share"},
            "write_script": {"repository_path": WRITE_SCRIPT_REPOSITORY_PATH,
                             "exchange_copy": write_script["exchange_copy"], "sha256": write_script["sha256"],
                             "run_from": "exchange_share"},
            "command_shape": {
                "batch": BATCH_COMMAND_SHAPE,
                "gui_print_fallback": {
                    "allowed": gui_fallback,
                    "used": gui_used,
                    "label": "print_to_file_fallback",
                    "description": "fallback when the batch route yields no result file: the guest script stops "
                                   "the batch process it started, opens the solved .res in a CAEPIPE instance it "
                                   "starts itself, inside the same claim and job, and prints the results to CSV via "
                                   "Print > To File",
                },
                "host_invocation": "prlctl exec <vm_name> --current-user cmd /c <guest command>",
            },
            "attempt_token": token,
            "attempts": attempts,
            # The classification an outer caller shows: a further attempt needs `retry_permitted`.
            "start": {"state": start, "restore_permitted": disposition(start)["restore"],
                      "retry_permitted": disposition(start)["retry"], "reason": (resolution or {}).get("reason"),
                      "claim": (resolution or {}).get("claim")},
            "steps": [{key: value for key, value in item.items() if key not in ("stdout", "stderr")}
                      | {"stdout_chars": len(item.get("stdout") or ""), "stderr_chars": len(item.get("stderr") or "")}
                      | _failed_write_stderr(item)
                      for item in steps],
            "working_directory": attempt_dir,
            "expected_batch_csv_path": _win(attempt_dir, "model.csv") if attempt_dir else None,
            "observed_csv_path": {"guest": guest_csv,
                                  "host": f"{relative}/{observed.name}" if observed is not None else None},
            "print_file": {"collected": observed.name if observed else None,
                           "layout": ("print_csv" if observed is not None and csv_name.lower() == PRINT_TXT_NAME
                                      else print_problem or ("as_named" if observed is not None else None))},
            "route": route,
            "exit_status": dispatch["exit_status"],
            "stdout_file": f"{relative}/guest_stdout.txt" if relative else None,
            "stderr_file": f"{relative}/guest_stderr.txt" if relative else None,
            "guest_log": guest_log,
            "outputs_folder": relative,
            "outputs_collected": collected,
            "outputs_sha256": {name: _sha256((attempt_out / name).read_bytes()) for name in collected},
            "timestamps": {"started": started, "finished": _now()},
            "boundary_note": BOUNDARY_NOTE,
        }
        if collection_problem is not None:
            record["collection_problem"] = collection_problem
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
            live["record_saved"] = True  # evidence only: the start is not touched
        except BaseException as caught:
            if error is None:
                error = caught
                _attach_after_record_failure(error, settings)
            _secondary(settings, "run record", caught)
    live["reported"] = True  # the records are done: the outer recovery has nothing left to write
    if error is not None:
        _carry_disposition(error, live)  # whatever the start, from the one source
        raise error
    if _classify(live)["start"] == "unresolved":
        unresolved = StartUnresolved(f"start unresolved: {(resolution or {}).get('reason')}", record)
        _carry_disposition(unresolved, live)
        if settings is not None:
            _attach_guard(unresolved, settings)
        raise unresolved
    return record


def stored_run_record(root: Path, out: Path) -> dict[str, Any] | None:
    """The harness run record stored in caepipe_out/, or None when absent or not a run record."""

    path = contained(root, out / "run_record.json")
    try:
        record = json.loads(path.read_text(encoding="utf-8")) if path.is_file() else None
    except (OSError, UnicodeDecodeError, json.JSONDecodeError):
        return None
    return record if isinstance(record, dict) and record.get("document_kind") == RUN_RECORD_KIND else None


def _solved_model_folder(record: Mapping[str, Any] | None) -> str | None:
    """The attempt folder (`<T>`, below caepipe_out) of a stored solve, or None for a record written before attempt
    folders, whose outputs lie in caepipe_out itself."""
    token = (record or {}).get("attempt_token")
    return token if isinstance(token, str) and ATTEMPT_TOKEN_PATTERN.match(token) else None


def solved_model_binding(record: Mapping[str, Any] | None, mbf_sha256: str, mod: Path) -> str:
    """Why the stored `model.mod` is not the solve of the MBF with `mbf_sha256`, or "" when the run record binds it."""

    if record is None:
        return "no valid caepipe_out/run_record.json binds a solved model to this MBF"
    if (record.get("mbf") or {}).get("sha256") != mbf_sha256 or record.get("guest_mbf_sha256") != mbf_sha256:
        return "the stored solve is not of the current MBF (host or guest MBF hash absent or different)"
    if "model.mod" not in (record.get("outputs_collected") or []) or not mod.is_file():
        return "the run record names no collected caepipe_out/model.mod"
    hashes = record.get("outputs_sha256")
    # Records written before outputs were hashed bind model.mod by name only.
    if isinstance(hashes, dict) and hashes.get("model.mod") != _sha256(mod.read_bytes()):
        return "caepipe_out/model.mod differs from the solved model the run record hashed"
    return ""


def run_caepipe_mbf_export(config: HarnessConfig, case_id: str, mbf_sha256: str, *, timeout_s: float = 600,
                           runner: Callable[..., subprocess.CompletedProcess] = subprocess.run) -> dict[str, Any]:
    """Export one case's stored solved model back to MBF with the user's CAEPIPE.

    Held (DEL-17-05, Scope of work): the guest script refuses the export (`export_held`) and starts nothing, because
    the export has not adopted the attempt's claim and job; until it does, a live call ends with that route. The
    host side below is kept for that change and runs the script from its content-addressed path, never staged.

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
    folder = _solved_model_folder(run_record)
    mod = contained(root, (out / folder if folder else out) / "model.mod")
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
    script = _install_guest_script(root, harness_dir)

    guest_out = _win(config.guest_exchange_root, "cases", case_id, "caepipe_out")
    guest_solved = _win(guest_out, folder) if folder else guest_out
    work = _win(config.guest_work_root, case_id, "export")
    guest_mod, guest_mbf = _win(work, "model.mod"), _win(work, EXPORTED_MBF)
    share_script = _win(config.guest_exchange_root, HARNESS_DIR, script["sha256"], GUEST_SCRIPT.name)

    def export_command(script: str) -> str:
        return (f"{POWERSHELL_COMMAND} -File {_quote(script)} "
                f"-Exe {_quote(config.guest_executable)} -ExportMod {_quote(guest_mod)} -ExportName {EXPORTED_MBF}")

    steps: list[dict[str, Any]] = []

    def step(name: str, command: str) -> dict[str, Any]:
        return _guest_step(config, runner, timeout_s, steps, name, command)

    # As for the solve: the guest copies are deleted first, so a failed copy can never leave an older model open.
    # Its source, the stored model in the exchange folder, is not changed by anything this run does. Only when
    # it succeeded does the export run; there is no staging fallback.
    prepare = step("export_prepare", f"{_ensure_dir(work)} & del /Q {_quote(guest_mod)} "
                                     f"{_quote(guest_mbf)} 2>nul & copy /Y {_quote(_win(guest_solved, 'model.mod'))} "
                                     f"{_quote(guest_mod)}")
    export: dict[str, Any] = {"exit_status": None, "stdout": "", "stderr": ""}
    guest_log = None
    # Each reply counts only in its form: prepare and collect a one-file copy each (`_copied_one_file`), the export
    # a single JSON object on stdout with nothing on stderr.
    if _copied_one_file(prepare):
        export = step("export", export_command(share_script))
        guest_log = _single_json_object(export["stdout"]) if not (export["stderr"] or "").strip() else None

    def guest_hash(key: str) -> str | None:
        value = guest_log.get(key) if isinstance(guest_log, dict) else None
        return value.lower() if isinstance(value, str) else None

    guest_mod_sha256, guest_exported_sha256 = guest_hash("mod_sha256"), guest_hash("exported_mbf_sha256")
    collect = None
    mod_verified = guest_mod_sha256 == mod_sha256
    status = guest_log.get("status") if isinstance(guest_log, dict) else None
    # Collected only when the report says exported, names the stored model's hash, and its own exit is 0.
    if status == EXPORT_ROUTE and mod_verified and export["exit_status"] == 0:
        _refuse_links(root, out)  # again just before the guest writes into the layer
        collect = step("export_collect", f"copy /Y {_quote(guest_mbf)} {_quote(_win(guest_out, EXPORTED_MBF))}")

    write_private(root, out / "export_guest_stdout.txt", export["stdout"])
    write_private(root, out / "export_guest_stderr.txt", export["stderr"])
    if guest_log is not None:
        write_private(root, out / "export_guest_log.json", json.dumps(guest_log, indent=2, sort_keys=True) + "\n")

    exported = contained(root, out / EXPORTED_MBF)
    exported_sha256 = exported_bytes = None
    if exported.is_file() and not exported.is_symlink():
        data = exported.read_bytes()
        exported_sha256, exported_bytes = _sha256(data), len(data)
    if not _copied_one_file(prepare):
        route = "prepare_failed"
    elif export["exit_status"] == "timeout":
        route = "timeout"
    elif guest_log is None:
        route = "no_guest_json"
    elif status == "export_held":
        route = "export_held"
    elif guest_mod_sha256 is None:  # the guest reported no hash: nothing was compared, so nothing "mismatched"
        route = str(status) if status and status != EXPORT_ROUTE else "input_hash_unreported"
    elif not mod_verified:
        route = "input_hash_mismatch"
    elif status != EXPORT_ROUTE:
        route = str(status or "unknown_guest_status")
    elif export["exit_status"] != 0:  # a report of success whose own exit says otherwise is not a success
        route = "exported_with_nonzero_exit"
    elif exported_sha256 is None or not (collect is not None and _copied_one_file(collect)):
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
        "mod": {"path": f"caepipe_out/{folder}/model.mod" if folder else "caepipe_out/model.mod",
                "guest_path": guest_mod, "sha256": mod_sha256},
        "guest_mod_sha256": guest_mod_sha256,
        "mod_hash_verified": mod_verified,
        "exported_mbf": {"path": f"caepipe_out/{EXPORTED_MBF}" if exported_sha256 else None,
                         "sha256": exported_sha256, "bytes": exported_bytes},
        "guest_exported_mbf_sha256": guest_exported_sha256,
        "guest_script": {"repository_path": GUEST_SCRIPT_REPOSITORY_PATH,
                         "exchange_copy": script["exchange_copy"],
                         "sha256": script["sha256"], "run_from": "exchange_share"},
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
