#!/usr/bin/env python3
"""Confine every native launch of the desktop application to a throwaway home, then check it.

One object, `Run`, owns a launch from end to end, and it is the only way the kit starts the
application:

- `begin` admits the run (macOS, the console session, the slot wrapper, the executable's strict
  chooser lead, the inventory of left homes), takes the baseline listing and refuses an incomplete
  one, and makes the throwaway home inside the kit's own folder under the system's per-user
  temporary folder, entering it in the kit's registry before the folder exists;
- `start` starts the application with an environment and working directory the helper builds
  (never the caller's), with only the standard streams the helper allows and every other
  descriptor closed, and records the identity of what it started (process identifier and start
  time together);
- `finish` waits for the web view's helpers to settle, makes and persists the comparison, lists
  the home, checks for processes that could still write to it, removes it through directory
  descriptors that follow nothing, and looks again some seconds later that the path has not
  reappeared.

The application is confined as to where it reads and writes its own files, and as to what
environment it is handed; nothing more. Observation of the consequences comes second: the real
Library and the system's per-user cache folder are compared before and after, the throwaway home
is listed, four home folders' times are read by a single stat each, and nothing else on the disk is
watched. It launches on macOS only and refuses on any other system. See native_launch_guard.md.

Command line (a Node runner calls it with `/usr/bin/python3 -I`):

  native_launch_guard.py check-launch [--guard-dir DIR]
  native_launch_guard.py left-homes
  native_launch_guard.py remove-left-home --record <the run's record.json>

Exit status, stable: 0 clean; 1 a product entry changed; 2 refused or unusable input; 3 no web view
data in the throwaway home; 4 the home was not removed; 5 the comparison failed; 6 the web view's
helpers wrote their cache folders under the system's per-user cache folder; 7 the home's path
reappeared after removal.
"""
from __future__ import annotations

# Its directory scans read the account's Library, the system's per-user cache and temporary
# folders and a throwaway home, never repository files (tools/ci/python_test_reads.py).
CI_READS = ()

import argparse
import errno
import hashlib
import json
import os
from pathlib import Path
import plistlib
import pwd
import re
import secrets
import signal
import stat
import subprocess
import sys
import tempfile
import threading
import time

PLATFORM = sys.platform  # launches are made on macOS only
WATCHED_TOPS = ("WebKit", "Caches", "HTTPStorages", "Application Support", "Preferences", "Saved Application State", "Logs")
WEB_VIEW_TOPS = ("WebKit", "Caches")
HOME_FOLDERS = ("", "Desktop", "Documents", "Downloads")  # times by a single stat each; never listed
PROTECTED_FOLDERS = ("Desktop", "Documents", "Downloads", "Library")  # no evidence folder under these
NAMED_OTHERS = (("Preferences",), ("Preferences", "ByHost"))
# The system's per-user cache folder, named by what it is and never by its path on a machine.
CACHE_TOP = "<system per-user cache folder>"
HELPER_CACHE = re.compile(r"^com\.apple\.WebKit\.(?:Networking|WebContent(?:\.EnhancedSecurity)?|GPU)\+(.+)$")
HOME_LISTING_KEPT = 2000
MAX_DEPTH = 64
SETTLE_SECONDS = 20  # the web view's helpers may write after the application exits
SECOND_LOOK_SECONDS = 5  # the look, after removal, that the path has not reappeared
PATH = "/usr/bin:/bin"  # the fixed PATH handed to the application
LOCALE = {"LANG": "en_US.UTF-8"}
# The helper's own utility programs: named by absolute path, given this environment, never the caller's.
PS, LSOF, IOREG, GETCONF = "/bin/ps", "/usr/sbin/lsof", "/usr/sbin/ioreg", "/usr/bin/getconf"
UTIL_ENV = {"PATH": "/usr/bin:/bin:/usr/sbin:/sbin", "LANG": "C", "LC_ALL": "C"}
CHECK_TIMEOUT = 60
# The strict chooser mode: the helper sets this variable on every launch. Once the product change
# lands, a chooser asked with nothing queued ends the application with STRICT_CHOOSER_EXIT before
# any system dialog. The product keeps its own copy of both.
STRICT_CHOOSER_ENV, STRICT_CHOOSER_EXIT = "SWBPIPE_NATIVE_SMOKE_STRICT_CHOOSER", 86
DRIVER, PLAN, REPORT = "SWBPIPE_NATIVE_SMOKE_DRIVER", "SWBPIPE_NATIVE_SMOKE_PLAN", "SWBPIPE_NATIVE_SMOKE_REPORT"
GATE, GATE_MODE = "SWBPIPE_NATIVE_SMOKE_AGENT_GATE", "SWBPIPE_NATIVE_SMOKE_AGENT_GATE_MODE"
PROVIDER_KEY = "SWBPIPE_NATIVE_SMOKE_PROVIDER_KEY"
KIT_VARIABLES = (DRIVER, PLAN, REPORT, GATE, GATE_MODE, PROVIDER_KEY)
DRIVER_ON_STDIN, PROVIDER_KEY_MARKER, RUN_OWNED, GATE_MODES = "-", "stdin", "run-owned", ("handshake",)
REFUSED_NAMES = ("SWBPIPE_NATIVE_SMOKE_INJECT_RESULT", "SWBPIPE_NATIVE_SMOKE_INJECT_ROOT")
REFUSED_PREFIXES = ("DYLD_", "LD_", "XDG_")
PROXY = re.compile(r"(?i)^(http|https|ftp|all|no)_proxy$")
CREDENTIAL = re.compile(r"(?i)(KEY|TOKEN|SECRET|PASSWORD|PASSWD|CREDENTIAL|AUTH)")
# The plan's closed shape: no field is a path, and none may look like one.
PLAN_SHAPE = {"provider": {"endpoint": str, "model": str, "contextWindow": int}, "turns": [str], "resultProbe": str,
              "treeTypes": [str], "timeoutsMs": {"ui": int, "start": int, "turn": int}}
ENDPOINT = re.compile(r"^https?://[A-Za-z0-9.\-\[\]:]+(/[^\s]*)?$")
STREAM_OPTIONS = ("stdin", "stdout", "stderr")
STREAMS = (subprocess.PIPE, subprocess.DEVNULL)
SLOT_LOCK = re.compile(r"^cargo_job(\.slot\d+)?\.lock$")
LOCK_PROGRAM = "/usr/bin/lockf"
KIT_DIR = "swbpipe-launch-kit"
ENTRY_KEYS = {"id", "label", "home", "state", "made", "maker", "token", "tempRoot", "kitRoot", "identity", "started",
              "groups", "executable", "updated"}
STATES = ("creating", "open", "withheld", "failed", "refused", "reappeared")
EXIT = {"clean": 0, "product": 1, "refused": 2, "no web view data": 3, "not removed": 4, "comparison failed": 5,
        "web view helper cache": 6, "reappeared": 7}
O_DIR = os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW | os.O_CLOEXEC
WATCH_STATEMENT = ("Nothing outside the real Library's watched folders (WebKit, Caches, HTTPStorages, Application "
                   "Support, Preferences, Saved Application State and Logs, walked in full; every other top folder "
                   "one level deep, by name), the system's per-user cache folder and the throwaway home was watched; "
                   "the times of the home, Desktop, Documents and Downloads folders were read by a single stat each, "
                   "and nothing inside them was listed or opened.")
OTHERS_STATEMENT = ("Records written during the window by the operating system's own services or by other "
                    "software exist under the account's real Library. They are counted per folder, named only "
                    "directly under Preferences, never opened, edited or removed, and they do not fail the run.")
WINDOW_STATEMENT = ("The comparison covers the window named here. Two launches whose windows overlap cannot be told "
                    "apart; the journey controller takes a host-wide lock, and other runners are not serialised by "
                    "this helper.")
LIMITS_STATEMENT = ("A clean comparison establishes neither that nothing was read nor that nothing was written "
                    "elsewhere on the disk.")
CONFINEMENT_STATEMENT = ("The application is confined as to where it reads and writes its own files, and as to what "
                         "environment it is handed.")
SOCKETS_STATEMENT = ("With no operating system boundary around the application itself, sockets elsewhere in the "
                     "account stay reachable to it, and home redirection cannot change that.")
RESIDUAL_STATEMENT = ("Before the throwaway home is removed, no process the kit started or can attribute to the "
                      "launch is alive. A system started web view helper that holds the home's path only in memory, "
                      "with no open file and no working directory there and an executable that is not ours, cannot "
                      "be seen; if one wrote after removal it would recreate a folder under the system's temporary "
                      "location; nothing of the account's own data is touched; the look some seconds after removal "
                      "and the left home inventory at the next admission find it.")
HELPER_CACHE_STATEMENT = ("The web view's helper processes, which the system starts, keep cache folders named for the "
                          "executable under the system's per-user cache folder. The system gives that folder "
                          "independently of the home, and whether both home variables redirect the helpers' folders "
                          "is not established; where they do not, it is a write outside the throwaway home that "
                          "confinement does not reach, observed by the comparison, not prevented. The run stops on "
                          "it. Nothing there is opened, edited or removed.")
POSITIVE_LIMIT = ("Any file under the throwaway home's Library/WebKit or Library/Caches satisfies this check; it "
                  "does not identify the web view's own data.")
STRICT_CHOOSER_STATEMENT = ("The executable contains the strict chooser variable's name: a lead, not a proof. The tie "
                            "is the comparison test against the product's constants.")
SIGNS_NOT_USED = {
    "screenSaver": "not used: which process runs while the screen saver is active could not be established by a harmless read",
    "displaySleep": "not used: the display's power value while asleep could not be established by a harmless read",
}
# Held at import, so a caller replacing subprocess.Popen replaces neither the kit's starts nor its
# read-only utility commands; kept apart so a test that stands in for the start keeps the reads real.
_POPEN = _READ_POPEN = subprocess.Popen


class LaunchRefused(Exception):
    """A launch, home or removal the kit will not make, named."""


class Interrupted(BaseException):
    """A catchable termination signal, raised so that the run's finish runs."""

    def __init__(self, signum):
        super().__init__(f"signal {signum}")
        self.signum = signum


class _Custody(Exception):
    """The folder to be removed is not, or is no longer, the one the kit made."""


def real_home():
    return Path(pwd.getpwuid(os.getuid()).pw_dir)


def real_library():
    return real_home() / "Library"


def stamp(t):
    return time.strftime("%Y-%m-%dT%H:%M:%S", time.gmtime(t)) + f".{int((t % 1) * 1000):03d}Z"


def _read(argv, timeout=CHECK_TIMEOUT):
    """Run one read-only utility, named by absolute path, with the helper's own minimal environment."""
    if not os.path.isabs(argv[0]):
        raise LaunchRefused("utility_not_absolute")
    with _READ_POPEN(argv, stdin=subprocess.DEVNULL, stdout=subprocess.PIPE, stderr=subprocess.PIPE, cwd="/",
                     env=dict(UTIL_ENV), close_fds=True) as process:
        try:
            out, err = process.communicate(timeout=timeout)
        except subprocess.TimeoutExpired:
            process.kill()
            process.communicate()
            raise
        return process.returncode, out, err.decode("utf-8", "replace")


# ---------------------------------------------------------------------------
# Admission: the system, the screen, the slot, the executable.

def console_users(answer):
    """The IOConsoleUsers array from the system registry's root, given as plist bytes."""
    def find(node):
        if isinstance(node, dict):
            if "IOConsoleUsers" in node:
                return node["IOConsoleUsers"]
            for value in node.values():
                found = find(value)
                if found is not None:
                    return found
        elif isinstance(node, list):
            for value in node:
                found = find(value)
                if found is not None:
                    return found
        return None
    return find(plistlib.loads(answer))


def screen_refusal(answer=None, uid=None):
    """None when the console session entry for this account is found, unambiguous, on the console and
    carries no locked key (any value of it refuses); otherwise the reason. Anything unparsed refuses."""
    uid = os.getuid() if uid is None else uid
    try:
        if answer is None:
            status, answer, _ = _read([IOREG, "-n", "Root", "-d1", "-a"])
            if status != 0:
                return "screen_unreadable"
        users = console_users(answer)
    except Exception:  # noqa: BLE001
        return "screen_unparsed"
    if not isinstance(users, list):
        return "screen_unparsed"
    mine = [u for u in users if isinstance(u, dict) and u.get("kCGSSessionUserIDKey") == uid]
    if not mine:
        return "screen_no_session_for_this_account"
    shapes = {(u.get("kCGSSessionOnConsoleKey") is True, "CGSSessionScreenIsLocked" in u,
               repr(u.get("CGSSessionScreenIsLocked"))) for u in mine}
    if len(shapes) > 1:
        return "screen_ambiguous"
    entry = mine[0]
    if entry.get("kCGSSessionOnConsoleKey") is not True:
        return "screen_not_on_console"
    if "CGSSessionScreenIsLocked" in entry:
        return "screen_locked"
    return None


def _start_text(fields):
    """A process's start time from ps's five lstart fields, in canonical form; raises when unparsed."""
    return time.strftime("%Y-%m-%dT%H:%M:%S", time.strptime(" ".join(fields), "%a %b %d %H:%M:%S %Y"))


def _process_table():
    """pid -> (ppid, pgid, uid, executable, start), read only. An unparsed row, or a listing that
    does not hold this process, raises: an incomplete listing is a refusal, never an empty result."""
    status, out, _ = _read([PS, "-axww", "-o", "pid=,ppid=,pgid=,uid=,lstart=,comm="])
    if status != 0:
        raise OSError(f"ps exit {status}")
    table = {}
    for line in out.decode("utf-8", "strict").splitlines():
        if not line.strip():
            continue
        fields = line.split(None, 9)
        try:
            if len(fields) != 10:
                raise ValueError(line)
            table[int(fields[0])] = (int(fields[1]), int(fields[2]), int(fields[3]), fields[9], _start_text(fields[4:9]))
        except ValueError:
            raise OSError("process listing row unparsed") from None
    if os.getpid() not in table:
        raise OSError("process listing incomplete")
    return table


def _lsof_lines(argv):
    status, out, err = _read(argv)
    # lsof exits 1 also when it found holders but not the named folder itself open; anything on
    # standard error (warnings are suppressed with -w) or any other status is a failed listing.
    if status not in (0, 1) or err.strip():
        raise OSError(f"lsof exit {status}")
    return out.decode("utf-8", "replace").splitlines()


def _open_names(pid):
    names = []
    for line in _lsof_lines([LSOF, "-n", "-P", "-w", "-F", "n", "-p", str(pid)]):
        if line.startswith("n"):
            names.append(line[1:])
        elif not re.fullmatch(r"[pf]\S*", line):
            raise OSError("open files listing unparsed")
    return names


def _open_under(home):
    """pids of this account's processes with an open file or working directory under `home`;
    an unparsed listing raises."""
    pids = set()
    for line in _lsof_lines([LSOF, "-n", "-P", "-w", "-F", "p", "+D", str(home)]):
        if re.fullmatch(r"p\d+", line):
            pids.add(int(line[1:]))
        elif not re.fullmatch(r"f\S+", line):
            raise OSError("open files listing unparsed")
    return pids


def slot_refusal(guard_dir=None, table=None, open_names=None, start=None):
    """None when an ancestor of this process is the slot wrapper's lock program, alive, with a slot
    lock file open: the lock program runs its command only while it holds that lock. With
    `guard_dir`, the lock file must be inside it. The kit takes no lock and acts on nothing of the
    wrapper's; it reads the process table and that ancestor's open files."""
    try:
        table = _process_table() if table is None else table
        open_names = _open_names if open_names is None else open_names
        pid, seen = (os.getppid() if start is None else start), set()
        while pid > 1 and pid in table and pid not in seen:
            seen.add(pid)
            ppid, _, uid, exe, _ = table[pid]
            if exe == LOCK_PROGRAM and uid == os.getuid():
                locks = [n for n in open_names(pid) if SLOT_LOCK.match(Path(n).name)]
                if guard_dir is not None:
                    locks = [n for n in locks if Path(n).resolve().parent == Path(guard_dir).resolve()]
                if locks:
                    return None
            pid = ppid
    except Exception:  # noqa: BLE001
        return "slot_unreadable"
    return "not_inside_the_slot_wrapper" if guard_dir is None else "not_inside_the_slot_wrapper_for_that_guard_folder"


def check_launch(guard_dir=None):
    """Refuse a launch on any system but macOS, outside the slot wrapper, or without an unlocked,
    unambiguous console session. Returns what was established, for the run's record."""
    if PLATFORM != "darwin":
        raise LaunchRefused("platform_not_macos")
    for reason in (screen_refusal(), slot_refusal(guard_dir)):
        if reason:
            raise LaunchRefused(reason)
    return {"screen": "one console session for this account, on console, no locked key", "slot": "inside the slot wrapper",
            "guardDirGiven": guard_dir is not None, "furtherSigns": SIGNS_NOT_USED}


def strict_chooser_lead(executable):
    """Whether the executable's bytes contain the strict chooser variable's name. Read, never run."""
    needle = STRICT_CHOOSER_ENV.encode()
    fd = os.open(executable, os.O_RDONLY | os.O_NOFOLLOW | os.O_CLOEXEC)
    try:
        if not stat.S_ISREG(os.fstat(fd).st_mode):
            return False
        tail = b""
        while chunk := os.read(fd, 1 << 20):
            if needle in tail + chunk:
                return True
            tail = (tail + chunk)[-(len(needle) - 1):]
        return False
    finally:
        os.close(fd)


# ---------------------------------------------------------------------------
# Folders: the system's, the kit's, the run's evidence.

def temp_root():
    """The system's own per-user temporary folder (macOS), else the standard temporary folder."""
    if PLATFORM == "darwin":
        status, out, _ = _read([GETCONF, "DARWIN_USER_TEMP_DIR"])
        text = out.decode().strip()
        if status != 0 or not text.startswith("/"):
            raise LaunchRefused("temporary_folder_unknown")
        return Path(os.path.realpath(text))
    return Path(os.path.realpath(tempfile.gettempdir()))


def user_cache_dir():
    """The system's own per-user cache folder (macOS), or None where the system gives none."""
    if PLATFORM != "darwin":
        return None
    status, out, _ = _read([GETCONF, "DARWIN_USER_CACHE_DIR"])
    text = out.decode().strip()
    return Path(os.path.realpath(text)) if status == 0 and text.startswith("/") else None


def _fid(fd):
    st = os.fstat(fd)
    return [st.st_dev, st.st_ino]


def _private_dir_at(parent_fd, name, create):
    """A folder of the kit's, opened without following a link, owned by this account and private."""
    if create:
        try:
            os.mkdir(name, 0o700, dir_fd=parent_fd)
        except FileExistsError:
            pass
    try:
        fd = os.open(name, O_DIR, dir_fd=parent_fd)
    except OSError as error:
        raise LaunchRefused(f"kit_folder_unopenable:{name}:{errno.errorcode.get(error.errno, error.errno)}") from None
    st = os.fstat(fd)
    if st.st_uid != os.geteuid() or stat.S_IMODE(st.st_mode) & 0o077:
        os.close(fd)
        raise LaunchRefused(f"kit_folder_not_private:{name}")
    return fd


class Kit:
    """The kit's folder under the system's temporary folder, with its registry and its homes, held
    open by descriptor: what is removed is found through these descriptors, never by a path resolved
    again, so a folder substituted by a link elsewhere is never reached."""

    def __init__(self, create=True):
        self.temp = temp_root()
        self.temp_fd = os.open(self.temp, os.O_RDONLY | os.O_DIRECTORY | os.O_CLOEXEC)
        self.fds = [self.temp_fd]
        try:
            self.kit_fd = _private_dir_at(self.temp_fd, KIT_DIR, create)
            self.fds.append(self.kit_fd)
            self.registry_fd = _private_dir_at(self.kit_fd, "registry", create)
            self.fds.append(self.registry_fd)
            self.homes_fd = _private_dir_at(self.kit_fd, "homes", create)
            self.fds.append(self.homes_fd)
        except BaseException:
            self.close()
            raise
        self.ids = {"tempRoot": _fid(self.temp_fd), "kitRoot": _fid(self.kit_fd)}
        self.homes = self.temp / KIT_DIR / "homes"

    def close(self):
        for fd in self.fds:
            try:
                os.close(fd)
            except OSError:
                pass
        self.fds = []

    def write_entry(self, entry):
        """Atomically replace the registry entry, written through the registry's descriptor."""
        entry = {**entry, "updated": stamp(time.time())}
        if set(entry) != ENTRY_KEYS:
            raise LaunchRefused("registry_entry_shape")
        name, scratch = f"{entry['id']}.json", f".{entry['id']}.tmp"
        try:
            os.unlink(scratch, dir_fd=self.registry_fd)
        except FileNotFoundError:
            pass
        fd = os.open(scratch, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW | os.O_CLOEXEC, 0o600,
                     dir_fd=self.registry_fd)
        with os.fdopen(fd, "w") as handle:
            handle.write(json.dumps(entry, indent=1) + "\n")
            handle.flush()
            os.fsync(handle.fileno())
        os.replace(scratch, name, src_dir_fd=self.registry_fd, dst_dir_fd=self.registry_fd)

    def drop_entry(self, entry_id):
        os.unlink(f"{entry_id}.json", dir_fd=self.registry_fd)

    def entries(self):
        """[(file name, entry or None when it cannot be read as a complete entry)]."""
        found = []
        for name in sorted(os.listdir(self.registry_fd)):
            if not name.endswith(".json"):
                continue
            entry = None
            try:
                fd = os.open(name, os.O_RDONLY | os.O_NOFOLLOW | os.O_CLOEXEC, dir_fd=self.registry_fd)
                with os.fdopen(fd, "rb") as handle:
                    if stat.S_ISREG(os.fstat(handle.fileno()).st_mode):
                        data = json.loads(handle.read(1 << 20))
                        if isinstance(data, dict) and set(data) == ENTRY_KEYS and data["state"] in STATES \
                                and f"{data['id']}.json" == name:
                            entry = data
            except (OSError, ValueError):
                entry = None
            found.append((name, entry))
        return found

    def exists(self, name):
        try:
            os.stat(name, dir_fd=self.homes_fd, follow_symlinks=False)
            return True
        except FileNotFoundError:
            return False


def check_evidence(evidence):
    """The run's evidence folder: absolute, a real folder with no link on its path, and neither
    under the account's Desktop, Documents, Downloads or Library nor under this repository."""
    if not Path(evidence).is_absolute():
        raise LaunchRefused("evidence_not_absolute")
    path = Path(os.path.normpath(evidence))
    real = Path(os.path.realpath(real_home()))
    for folder in PROTECTED_FOLDERS:
        if path == real / folder or (real / folder) in path.parents:
            raise LaunchRefused(f"evidence_under_{folder.lower()}")
    repository = Path(os.path.realpath(Path(__file__).resolve().parents[1]))
    if path == repository or repository in path.parents:
        raise LaunchRefused("evidence_under_the_repository")
    try:
        st = os.lstat(path)
    except OSError:
        raise LaunchRefused("evidence_missing") from None
    if not stat.S_ISDIR(st.st_mode):
        raise LaunchRefused("evidence_not_a_real_folder")
    if os.path.realpath(path) != str(path):
        raise LaunchRefused("evidence_path_has_a_link")
    return path


def _write_new(dir_fd, name, data):
    """A new file in the evidence folder, never through a link and never over another file."""
    fd = os.open(name, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW | os.O_CLOEXEC, 0o600, dir_fd=dir_fd)
    with os.fdopen(fd, "w") as handle:
        handle.write(json.dumps(data, indent=1) + "\n")


# ---------------------------------------------------------------------------
# The environment.

def allowed(name, value):
    """Refuse, by name, a variable a caller may not pass, and a value the kit does not own."""
    if name in REFUSED_NAMES:
        raise LaunchRefused(f"injection_variable:{name}")
    if name.startswith(REFUSED_PREFIXES):
        raise LaunchRefused(f"loader_or_xdg_variable:{name}")
    if PROXY.match(name):
        raise LaunchRefused(f"proxy_variable:{name}")
    if name not in KIT_VARIABLES:
        if CREDENTIAL.search(name):
            raise LaunchRefused(f"credential_like_variable:{name}")
        raise LaunchRefused(f"variable_not_on_the_list:{name}")
    if not isinstance(value, str):
        raise LaunchRefused(f"variable_not_text:{name}")
    if name == DRIVER and value != DRIVER_ON_STDIN:
        raise LaunchRefused("driver_on_stdin_only")
    if name == PROVIDER_KEY and value != PROVIDER_KEY_MARKER:
        raise LaunchRefused("provider_key_marker_only")
    if name in (REPORT, GATE) and value != RUN_OWNED:
        raise LaunchRefused("report_and_gate_are_the_runs")
    if name == GATE_MODE and value not in GATE_MODES:
        raise LaunchRefused("gate_mode_not_known")


def _pathlike(text):
    return text.startswith(("/", "~", "./", "../", "file:", "unix:")) or "\x00" in text


def validate_plan(plan, shape=PLAN_SHAPE, where="plan"):
    """The plan against its closed shape: no key outside it, every value of its type, no string that
    looks like a path, an http(s) endpoint only."""
    if isinstance(shape, dict):
        if not isinstance(plan, dict) or set(plan) - set(shape):
            raise LaunchRefused(f"plan_shape:{where}")
        for key, value in plan.items():
            validate_plan(value, shape[key], f"{where}.{key}")
    elif isinstance(shape, list):
        if not isinstance(plan, list) or len(plan) > 64:
            raise LaunchRefused(f"plan_shape:{where}")
        for value in plan:
            validate_plan(value, shape[0], where)
    elif shape is int:
        if not isinstance(plan, int) or isinstance(plan, bool) or not 0 < plan < 10 ** 9:
            raise LaunchRefused(f"plan_shape:{where}")
    elif not isinstance(plan, str) or len(plan) > 20000 or _pathlike(plan):
        raise LaunchRefused(f"plan_shape:{where}")
    elif where == "plan.provider.endpoint" and not ENDPOINT.match(plan):
        raise LaunchRefused("plan_endpoint")
    return plan


# ---------------------------------------------------------------------------
# Observation: names, sizes and times only, through descriptors that follow nothing.

def _forms(names):
    """The watched names and each in its underscore form, the form the web view's helpers use for the
    executable's name."""
    return sorted(set(names) | {name.replace("-", "_") for name in names})


def _named(text, name):
    return text == name or text.startswith(name + ".")


def _after_plus(segment, name):
    return _named(segment, name) or segment.startswith(name + "-")


def watched(relative, names):
    """A path component equal to a watched name (or its underscore form) or beginning with one
    followed by a dot, or, after a plus sign (the web view helpers' folder names), such a name or one
    followed by a hyphen."""
    forms = _forms(names)
    return any(_named(part, name) or any(_after_plus(seg, name) for seg in part.split("+")[1:])
               for part in Path(relative).parts for name in forms)


def helper_cache_entry(relative, names):
    """Whether a product-named path lies inside a web view helper's cache folder directly under the
    system's per-user cache folder: com.apple.WebKit.<Networking|WebContent|GPU>+<a watched name>."""
    parts = Path(relative).parts
    if len(parts) < 2 or parts[0] != CACHE_TOP:
        return False
    match = HELPER_CACHE.match(parts[1])
    return bool(match) and any(_after_plus(match.group(1), name) for name in _forms(names))


def facts(st):
    kind = ("dir" if stat.S_ISDIR(st.st_mode) else "link" if stat.S_ISLNK(st.st_mode)
            else "file" if stat.S_ISREG(st.st_mode) else "other")
    return {"kind": kind, "size": st.st_size, "mtime": st.st_mtime, "birth": getattr(st, "st_birthtime", st.st_ctime)}


class _Sink:
    """What a walk found: watched entries, others in the window, and what could not be listed."""

    def __init__(self, names, since=None, until=None):
        self.names, self.since, self.until = names, since, until
        self.entries, self.unreadable_product, self.unreadable_other = {}, [], 0
        self.changed, self.too_deep, self.failed = 0, 0, []
        self.others = {"total": 0, "byFolder": {}, "named": []}

    def entry(self, relative, found, top):
        if watched(relative, self.names):
            self.entries[relative] = found
        elif self.since is not None and (self.since <= found["mtime"] <= self.until or self.since <= found["birth"] <= self.until):
            self.others["total"] += 1
            self.others["byFolder"][top] = self.others["byFolder"].get(top, 0) + 1
            if Path(relative).parts[:-1] in NAMED_OTHERS:
                self.others["named"].append({"path": relative, "size": found["size"], "mtime": stamp(found["mtime"])})

    def unreadable(self, relative):
        if watched(relative, self.names):
            self.unreadable_product.append(relative)
        else:
            self.unreadable_other += 1


def _open_child(parent_fd, name, seen):
    """Open a folder seen by lstat, without following a link, and only if it is still that folder."""
    fd = os.open(name, O_DIR, dir_fd=parent_fd)
    opened = os.fstat(fd)
    if (opened.st_dev, opened.st_ino) != (seen.st_dev, seen.st_ino):
        os.close(fd)
        raise _Custody("changed")
    return fd


def _deep(fd, relative, top, sink, depth=0):
    """Every entry under an open folder, by lstat at the folder's descriptor; each folder entered
    is opened without following a link and checked to be the one seen."""
    try:
        names = sorted(os.listdir(fd))
    except OSError:
        sink.unreadable(relative)
        return
    for name in names:
        child = f"{relative}/{name}"
        try:
            seen = os.stat(name, dir_fd=fd, follow_symlinks=False)
        except FileNotFoundError:
            continue
        except OSError:
            sink.unreadable(child)
            continue
        sink.entry(child, facts(seen), top)
        if not stat.S_ISDIR(seen.st_mode):
            continue
        if depth + 1 > MAX_DEPTH:
            sink.too_deep += 1
            continue
        try:
            child_fd = _open_child(fd, name, seen)
        except FileNotFoundError:
            continue
        except _Custody:
            sink.changed += 1
            continue
        except OSError as error:
            if error.errno in (errno.ELOOP, errno.ENOTDIR):
                sink.changed += 1
            else:
                sink.unreadable(child)
            continue
        try:
            _deep(child_fd, child, top, sink, depth + 1)
        finally:
            os.close(child_fd)


def list_library(names, library, since=None, until=None, cache=None):
    """One walk: the watched Library folders and the system's per-user cache folder in full; every
    other top folder of the Library one level deep, by name only. An unreadable watched top folder,
    a link where one was expected, or a cache folder the system does not give, fails closed."""
    sink = _Sink(names, since, until)
    try:
        library_fd = os.open(library, O_DIR)
    except OSError:
        sink.failed.append("Library")
        return _listing(sink)
    try:
        try:
            tops = sorted(os.listdir(library_fd))
        except OSError:
            sink.failed.append("Library")
            tops = []
        for top in tops:
            try:
                seen = os.stat(top, dir_fd=library_fd, follow_symlinks=False)
            except FileNotFoundError:
                continue
            except OSError:
                (sink.failed.append(top) if top in WATCHED_TOPS else sink.unreadable(top))
                continue
            sink.entry(top, facts(seen), top)
            if not stat.S_ISDIR(seen.st_mode):
                if top in WATCHED_TOPS:
                    sink.failed.append(top)  # a watched top that is a link or not a folder
                continue
            try:
                top_fd = _open_child(library_fd, top, seen)
            except (OSError, _Custody):
                (sink.failed.append(top) if top in WATCHED_TOPS else sink.unreadable(top))
                continue
            try:
                if top in WATCHED_TOPS:
                    _deep(top_fd, top, top, sink)
                else:
                    _one_level(top_fd, top, sink)
            finally:
                os.close(top_fd)
    finally:
        os.close(library_fd)
    if cache is None:
        sink.failed.append(CACHE_TOP)
    else:
        try:
            cache_fd = os.open(cache, O_DIR)
        except OSError:
            sink.failed.append(CACHE_TOP)
        else:
            try:
                _deep(cache_fd, CACHE_TOP, CACHE_TOP, sink)
            finally:
                os.close(cache_fd)
    return _listing(sink)


def _one_level(fd, top, sink):
    """A top folder outside the watched set: its immediate children by name and lstat only."""
    try:
        names = sorted(os.listdir(fd))
    except OSError:
        sink.unreadable(top)
        return
    for name in names:
        try:
            seen = os.stat(name, dir_fd=fd, follow_symlinks=False)
        except FileNotFoundError:
            continue
        except OSError:
            sink.unreadable(f"{top}/{name}")
            continue
        sink.entry(f"{top}/{name}", facts(seen), top)


def _listing(sink):
    return {"entries": sink.entries, "others": sink.others, "failedTops": sorted(set(sink.failed)),
            "unreadableProduct": sorted(sink.unreadable_product), "unreadableOther": sink.unreadable_other,
            "changedDuringWalk": sink.changed, "tooDeep": sink.too_deep}


def folder_times(home=None):
    """The modification time of the real home folder and its Desktop, Documents and Downloads, by a
    single stat of each; nothing inside them is listed, opened or read."""
    home = real_home() if home is None else Path(home)
    times = {}
    for name in HOME_FOLDERS:
        path = home / name if name else home
        try:
            times[name or "~"] = os.lstat(path).st_mtime
        except OSError as error:
            times[name or "~"] = f"unreadable: {type(error).__name__}"
    return times


def snapshot(names, library=None, home=None, cache=None):
    """Before the first launch: watched entries only, what could not be listed, and the four times."""
    names = sorted(set(names))
    if not names or any(not isinstance(n, str) or not n or "/" in n for n in names):
        raise ValueError("names to watch are required")
    library = Path(library) if library is not None else real_library()
    cache = Path(cache) if cache is not None else user_cache_dir()
    taken = time.time()
    listing = list_library(names, library, cache=cache)
    return {"taken": taken, "names": names, **{k: listing[k] for k in listing if k != "others"},
            "folderTimes": folder_times(home)}


def baseline_refusal(before):
    """An incomplete baseline refuses the launch: an unreadable watched top folder, an unreadable
    product-named folder, a folder changed during the walk, a walk too deep."""
    if before["failedTops"]:
        return "baseline_folder_not_listed:" + ",".join(before["failedTops"])
    if before["unreadableProduct"]:
        return "baseline_product_folder_unreadable"
    if before["changedDuringWalk"] or before["tooDeep"]:
        return "baseline_walk_incomplete"
    return None


def home_listing(home):
    """The throwaway home's own files by lstat through descriptors (bounded), and whether a file lies
    under its Library/WebKit or Library/Caches. `home` is a path, or (folder descriptor, name) so
    that the run lists the folder it holds. A failure is recorded, not raised."""
    try:
        rows = []

        class Rows(_Sink):
            def entry(self, relative, found, top):
                if len(rows) < HOME_LISTING_KEPT * 10:
                    rows.append({"path": relative.split("/", 1)[1], **found})

        sink = Rows(["\x00"])
        fd = os.open(home[1], O_DIR, dir_fd=home[0]) if isinstance(home, tuple) else os.open(home, O_DIR)
        try:
            _deep(fd, ".", ".", sink)
        finally:
            os.close(fd)
        rows.sort(key=lambda r: r["path"])
        web_view = any(r["kind"] == "file" and Path(r["path"]).parts[:1] == ("Library",) and len(Path(r["path"]).parts) > 2
                       and Path(r["path"]).parts[1] in WEB_VIEW_TOPS for r in rows)
        return {"by": "lstat through descriptors", "total": len(rows), "truncated": len(rows) > HOME_LISTING_KEPT,
                "entries": rows[:HOME_LISTING_KEPT], "webViewData": web_view, "webViewDataLimit": POSITIVE_LIMIT,
                "unreadableCount": sink.unreadable_other + len(sink.unreadable_product)}
    except Exception as error:  # noqa: BLE001
        return {"by": "lstat through descriptors", "error": f"{type(error).__name__}: {error}"[:1000], "webViewData": False}


def compare(before, throwaway, until=None, library=None, home=None, cache=None):
    """After the run. `verdict`: "product", "comparison failed", "web view helper cache", "no web view
    data" or "clean"; `class`: "product", "web view helper cache", "others" or "nothing". A change
    inside a web view helper's cache folder under the system's per-user cache folder is its own
    class and stops the run; any other product-named change there is a product change."""
    library = Path(library) if library is not None else real_library()
    cache = Path(cache) if cache is not None else user_cache_dir()
    names, since = before["names"], before["taken"]
    until = until if until is not None else time.time()
    after = list_library(names, library, since, until, cache)
    changes = []
    for relative in sorted(set(before["entries"]) | set(after["entries"])):
        old, new = before["entries"].get(relative), after["entries"].get(relative)
        if old == new:
            continue
        kind = "created" if old is None else "removed" if new is None else "modified"
        changes.append({"change": kind, "path": relative, "before": old, "after": new})
    helpers = [c for c in changes if helper_cache_entry(c["path"], names)]
    changes = [c for c in changes if not helper_cache_entry(c["path"], names)]
    blind = sorted(set(after["unreadableProduct"]) | set(before["unreadableProduct"]))
    failed = sorted(set(before["failedTops"]) | set(after["failedTops"]))
    count_changed = after["unreadableOther"] != before["unreadableOther"]
    walk_incomplete = bool(after["changedDuringWalk"] or after["tooDeep"])
    times_after = folder_times(home)
    positive = home_listing(throwaway)
    product = bool(changes or blind)
    verdict = ("product" if product else "comparison failed" if failed or count_changed or walk_incomplete
               else "web view helper cache" if helpers
               else "clean" if positive["webViewData"] else "no web view data")
    return {"compared": True, "verdict": verdict,
            "class": ("product" if product else "web view helper cache" if helpers
                      else "others" if after["others"]["total"] else "nothing"),
            "names": names, "window": {"since": stamp(since), "until": stamp(until), "statement": WINDOW_STATEMENT},
            "product": {"changes": changes, "unreadable": blind},
            "helperCache": {"statement": HELPER_CACHE_STATEMENT, "changes": helpers},
            "failure": {"foldersNotListed": failed, "unreadableCountBefore": before["unreadableOther"],
                        "unreadableCountAfter": after["unreadableOther"], "walkIncomplete": walk_incomplete},
            "limit": {"statement": f"{after['unreadableOther']} folders could not be listed and were not watched"},
            "others": {"statement": OTHERS_STATEMENT, **after["others"]},
            "folderTimes": {"before": before["folderTimes"], "after": times_after,
                            "changed": sorted(k for k in times_after if times_after[k] != before["folderTimes"].get(k)),
                            "statement": "reported with the run, not concluded from"},
            "throwawayHome": positive, "watched": WATCH_STATEMENT, "limits": LIMITS_STATEMENT}


# ---------------------------------------------------------------------------
# Removal through descriptors.

def _empty(fd, dev, depth=0):
    """Empty an open folder without following a link at any depth; a folder is entered only when,
    opened without following links, it is the one lstat saw, on the home's device."""
    if depth > MAX_DEPTH:
        raise _Custody("depth")
    for _ in range(1000):
        names = os.listdir(fd)
        if not names:
            return
        for name in names:
            try:
                seen = os.stat(name, dir_fd=fd, follow_symlinks=False)
            except FileNotFoundError:
                continue
            if not stat.S_ISDIR(seen.st_mode):
                os.unlink(name, dir_fd=fd)
                continue
            if seen.st_dev != dev:
                raise _Custody("other_volume_inside")
            if seen.st_uid != os.geteuid():
                raise _Custody("other_account_inside")
            child = _open_child(fd, name, seen)
            try:
                if stat.S_IMODE(os.fstat(child).st_mode) & 0o700 != 0o700:
                    os.fchmod(child, 0o700)
                _empty(child, dev, depth + 1)
            finally:
                os.close(child)
            os.rmdir(name, dir_fd=fd)
    raise _Custody("not_emptied")


def remove_at(parent_fd, name, identity):
    """Remove the folder `name` in the open folder `parent_fd`, only if, opened without following
    a link, it has the recorded device and inode; the emptied folder is checked again before it is
    removed. Returns False when there is nothing there."""
    try:
        fd = os.open(name, O_DIR, dir_fd=parent_fd)
    except FileNotFoundError:
        return False
    except OSError as error:
        raise _Custody(f"unopenable:{errno.errorcode.get(error.errno, error.errno)}") from None
    try:
        if _fid(fd) != list(identity):
            raise _Custody("identity_changed")
        _empty(fd, identity[0])
    finally:
        os.close(fd)
    named = os.stat(name, dir_fd=parent_fd, follow_symlinks=False)
    if [named.st_dev, named.st_ino] != list(identity):
        raise _Custody("changed")
    os.rmdir(name, dir_fd=parent_fd)
    return True


# ---------------------------------------------------------------------------
# Processes that could still write to a home.

def _generation_alive(generation, table):
    """'alive', 'unestablished' or None for a recorded (identifier, start time) generation."""
    row = table.get(generation["pid"])
    if row is None:
        return None
    if generation.get("start") is None:
        return "unestablished"
    return "alive" if row[4] == generation["start"] else None


def possible_writers(home, children=(), generations=(), groups=(), private=()):
    """Every process of this account that could still write to `home`, with why, and what coverage
    was achieved. A process the kit started and has not seen end; a recorded generation still alive,
    or whose start time could not be read; a descendant of one; a process in a group attributed to
    the run; one whose executable lies under the home or the private copy; one with a file or its
    working directory open under the home. macOS only; an incomplete listing raises."""
    if PLATFORM != "darwin":
        raise OSError("the check for possible writers is made on macOS only")
    home = str(home)
    roots = [home, *(str(p) for p in private)]
    table = _process_table()
    opened = _open_under(home)
    found = {}
    for child in children:
        if child.poll() is None:
            found.setdefault(child.pid, []).append("started by the run and not seen to end")
    living = []
    for generation in generations:
        state = _generation_alive(generation, table)
        if state:
            found.setdefault(generation["pid"], []).append(f"recorded generation {state}")
            living.append(generation["pid"])
    children_of = {}
    for pid, row in table.items():
        children_of.setdefault(row[0], []).append(pid)
    lineage, stack = set(), list(living)
    while stack:
        pid = stack.pop()
        if pid not in lineage:
            lineage.add(pid)
            stack.extend(children_of.get(pid, []))
    for pid in opened:
        found.setdefault(pid, []).append("open under the home")
    for pid, (ppid, pgid, uid, exe, _) in table.items():
        if uid != os.getuid():
            continue
        if exe and any(exe == r or exe.startswith(r.rstrip("/") + "/") for r in roots):
            found.setdefault(pid, []).append("executable under the home or the private copy")
        if pid in lineage and pid not in living:
            found.setdefault(pid, []).append("a descendant of a recorded generation")
        if pgid in set(groups):
            found.setdefault(pid, []).append("in a group attributed to the run")
    found.pop(os.getpid(), None)
    writers = [{"pid": pid, "executable": table.get(pid, (0, 0, 0, "", ""))[3], "why": sorted(set(why))}
               for pid, why in sorted(found.items())]
    coverage = {"processRows": len(table), "openFilesListed": True, "startedChecked": len(list(children)),
                "generationsChecked": len(list(generations)), "groupsChecked": sorted(set(groups)),
                "statement": RESIDUAL_STATEMENT}
    return writers, coverage


def left_homes(kit, exclude=None):
    """The inventory of unresolved homes: every registry entry not removed, by name, and every folder
    under the kit's homes the registry does not hold. A launch is refused only where the kit cannot
    establish that no process started from a left home is alive."""
    report, blocking = [], []
    table = None
    known = set()
    for name, entry in kit.entries():
        if entry is None:
            report.append({"entry": name, "state": "unreadable entry"})
            blocking.append(f"{name}: unreadable entry")
            continue
        known.add(entry["home"])
        if entry["id"] == exclude:
            continue
        item = {"id": entry["id"], "home": entry["home"], "label": entry["label"], "state": entry["state"],
                "exists": kit.exists(entry["home"])}
        try:
            table = _process_table() if table is None else table
            reasons = [s for g in entry["started"] if (s := _generation_alive(g, table))]
            if item["exists"] and _open_under(kit.homes / entry["home"]):
                reasons.append("open under the home")
        except Exception as error:  # noqa: BLE001
            reasons = [f"the check could not run ({type(error).__name__})"]
        item["processes"] = reasons
        if reasons:
            blocking.append(f"{entry['home']}: {', '.join(reasons)}")
        report.append(item)
    for name in sorted(os.listdir(kit.homes_fd)):
        if name not in known:
            report.append({"home": name, "state": "unregistered", "exists": True})
    return {"leftHomes": report, "refuse": bool(blocking), "reasons": blocking}


# ---------------------------------------------------------------------------
# The run.

def product_names(executable, extra=()):
    """The names the web view files data under: the executable's name, the built bundle's
    identifier (when the executable sits in a bundle), and any the caller adds."""
    executable = Path(executable)
    names = {executable.name, *extra}
    try:
        with open(executable.parents[1] / "Info.plist", "rb") as handle:
            names.add(plistlib.load(handle)["CFBundleIdentifier"])
    except (OSError, KeyError, ValueError, IndexError):
        pass
    return sorted(n for n in names if n)


def private_copy(executable):
    """The private copy the executable belongs to: its bundle when it sits in one, else its folder."""
    executable = Path(executable)
    if executable.parent.name == "MacOS" and executable.parent.parent.name == "Contents":
        return executable.parents[2]
    return executable.parent


class Run:
    """One launch, from admission to removal: the only way the kit starts the application."""

    library = None  # the real Library; offline tests alone set another
    cache = None  # the system's per-user cache folder; offline tests alone set another
    real = None  # the account's home folder for the four folder times; offline tests alone set another

    def __init__(self, executable, evidence, label="home", names=None, guard_dir=None):
        if not re.fullmatch(r"[a-z0-9][a-z0-9._-]{0,40}", label):
            raise LaunchRefused("run_label")
        self.executable = Path(executable)
        self.evidence = Path(evidence)
        self.label = label
        self.names = sorted(set(names or product_names(executable)))
        self.guard_dir = guard_dir
        self.private = [private_copy(executable)]
        self.kit = self.entry = self.before = self.evidence_fd = None
        self.children, self.generations, self.groups, self.roles, self.admissions = [], [], [], [], []
        self.token = None
        self.registry_errors = []
        self.state = "new"
        self.outcome = None
        self._finishing = False
        self._pending = None
        self._previous = {}

    # -- paths
    @property
    def home(self):
        return self.kit.homes / self.entry["home"] if self.entry else None

    @property
    def tmp(self):
        return self.home / "tmp"

    @property
    def work(self):
        return self.home / "work"

    @property
    def report_path(self):
        return self.work / "report.json"

    @property
    def gate_path(self):
        return self.work / "gate"

    # -- admission
    def _admit(self):
        admission = {"launch": check_launch(self.guard_dir)}
        if not strict_chooser_lead(self.executable):
            raise LaunchRefused("strict_chooser_absent_from_executable")
        admission["strictChooser"] = STRICT_CHOOSER_STATEMENT
        inventory = left_homes(self.kit, exclude=self.entry["id"] if self.entry else None)
        admission["leftHomes"] = inventory["leftHomes"]
        if inventory["refuse"]:
            raise LaunchRefused("left_home_processes_not_established_gone:" + "; ".join(inventory["reasons"])[:400])
        admission["at"] = stamp(time.time())
        self.admissions.append(admission)
        return admission

    def begin(self):
        """Admit, take and check the baseline, then make the home, entered in the registry first."""
        if self.state != "new":
            raise LaunchRefused("run_already_begun")
        self.state = "beginning"
        check_evidence(self.evidence)
        self.evidence_fd = os.open(self.evidence, O_DIR)
        self.kit = Kit(create=True)
        self._admit()
        self.before = snapshot(self.names, self.library, self.real, self.cache)
        refusal = baseline_refusal(self.before)
        if refusal:
            raise LaunchRefused(refusal)
        self._install_signals()
        self.token = secrets.token_hex(32)
        entry_id = secrets.token_hex(8)
        maker = {"pid": os.getpid(), "start": None}
        try:
            maker["start"] = _process_table()[os.getpid()][4] if PLATFORM == "darwin" else None
        except Exception:  # noqa: BLE001
            pass
        self.entry = {"id": entry_id, "label": self.label, "home": f"swbpipe-{self.label}-{entry_id}", "state": "creating",
                      "made": stamp(time.time()), "maker": maker, "token": hashlib.sha256(self.token.encode()).hexdigest(),
                      "tempRoot": self.kit.ids["tempRoot"], "kitRoot": self.kit.ids["kitRoot"], "identity": {},
                      "started": [], "groups": [], "executable": str(self.executable), "updated": ""}
        # The run's own record first: an evidence folder another run already wrote to refuses here.
        _write_new(self.evidence_fd, f"{self.label}-record.json",
                   {"id": entry_id, "token": self.token, "home": str(self.home), "label": self.label,
                    "statement": "the run that made this home holds this token; only it may remove the home"})
        self.kit.write_entry(self.entry)  # before the folder exists: a failure after it is made is still held
        os.mkdir(self.entry["home"], 0o700, dir_fd=self.kit.homes_fd)
        home_fd = os.open(self.entry["home"], O_DIR, dir_fd=self.kit.homes_fd)
        try:
            identity = {"home": _fid(home_fd)}
            for name in ("tmp", "work"):
                os.mkdir(name, 0o700, dir_fd=home_fd)
                child = os.open(name, O_DIR, dir_fd=home_fd)
                identity[name] = _fid(child)
                os.close(child)
        finally:
            os.close(home_fd)
        opened = {**self.entry, "identity": identity, "state": "open"}
        self.kit.write_entry(opened)
        self.entry = opened
        self.state = "open"
        return self

    # -- the environment
    def environment(self, kit=None):
        env = {}
        for name, value in (kit or {}).items():
            allowed(name, value)
            if name == PLAN:
                try:
                    value = json.dumps(validate_plan(json.loads(value)))
                except ValueError:
                    raise LaunchRefused("plan_not_json") from None
            elif name == REPORT:
                value = str(self.report_path)
            elif name == GATE:
                value = str(self.gate_path)
            env[name] = value
        env.update(LOCALE)
        env.update({"PATH": PATH, "TMPDIR": str(self.tmp), STRICT_CHOOSER_ENV: "1",
                    "HOME": str(self.home), "CFFIXED_USER_HOME": str(self.home)})
        return env

    def _verify_paths(self):
        """The home's path still leads, without a link, to the folders the kit made."""
        fd = os.open(self.kit.temp, os.O_RDONLY | os.O_DIRECTORY | os.O_CLOEXEC)
        fds = [fd]
        try:
            if _fid(fd) != self.kit.ids["tempRoot"]:
                raise LaunchRefused("temporary_folder_changed")
            for name, expected in ((KIT_DIR, self.kit.ids["kitRoot"]), ("homes", _fid(self.kit.homes_fd)),
                                   (self.entry["home"], self.entry["identity"]["home"])):
                try:
                    fd = os.open(name, O_DIR, dir_fd=fd)
                except OSError:
                    raise LaunchRefused(f"home_path_changed:{name}") from None
                fds.append(fd)
                if _fid(fd) != list(expected):
                    raise LaunchRefused(f"home_path_changed:{name}")
            for name in ("tmp", "work"):
                child = os.open(name, O_DIR, dir_fd=fd)
                fds.append(child)
                if _fid(child) != list(self.entry["identity"][name]):
                    raise LaunchRefused(f"home_path_changed:{name}")
        finally:
            for each in fds:
                os.close(each)

    def start(self, role, argv, kit=None, web_view=False, **streams):
        """Start the run's executable with the helper's environment, working directory and streams,
        every other descriptor closed; record its identity at once. Returns the process handle."""
        if self.state != "open":
            raise LaunchRefused("run_not_open")
        if not argv or argv[0] != str(self.executable):
            raise LaunchRefused("executable_not_the_runs")
        for name, value in streams.items():
            if name not in STREAM_OPTIONS:
                raise LaunchRefused(f"process_option_not_allowed:{name}")
            if value not in STREAMS:
                raise LaunchRefused(f"stream_not_allowed:{name}")
        self._admit()
        env = self.environment(kit)
        self._verify_paths()
        process = _POPEN(list(argv), env=env, cwd=str(self.work), close_fds=True, pass_fds=(), restore_signals=True,
                         **{name: streams.get(name, subprocess.DEVNULL) for name in STREAM_OPTIONS})
        self.children.append(process)
        generation = {"pid": process.pid, "start": None, "role": role}
        self.generations.append(generation)
        self.roles.append({"role": role, "webView": bool(web_view)})
        try:
            generation["start"] = _process_table().get(process.pid, (0, 0, 0, "", None))[4]
        except Exception:  # noqa: BLE001  (unestablished: withholds removal)
            generation["start"] = None
        self.entry["started"] = [{"pid": g["pid"], "start": g["start"], "role": g["role"]} for g in self.generations]
        self.kit.write_entry(self.entry)
        return process

    def attribute(self, group=None):
        """Add a process group to what the run answers for; it can only widen the check."""
        if isinstance(group, int) and group > 1 and group not in self.groups:
            self.groups.append(group)
            if self.entry:
                self.entry["groups"] = list(self.groups)
                try:
                    self.kit.write_entry(self.entry)
                except Exception:  # noqa: BLE001
                    pass

    # -- signals
    def _install_signals(self):
        if threading.current_thread() is not threading.main_thread():
            return
        for signum in (signal.SIGTERM, signal.SIGINT, signal.SIGHUP):
            self._previous[signum] = signal.signal(signum, self._on_signal)

    def _on_signal(self, signum, frame):
        if self._finishing:
            self._pending = signum
            return
        raise Interrupted(signum)

    def _restore_signals(self):
        for signum, previous in self._previous.items():
            try:
                signal.signal(signum, previous)
            except (ValueError, TypeError):
                pass
        self._previous = {}

    def _pause(self, seconds, note, outcome):
        try:
            time.sleep(seconds)
        except BaseException as error:  # noqa: BLE001  (an interrupt during a wait skips nothing)
            outcome.setdefault("interruptions", []).append(f"{note}: {type(error).__name__}")

    # -- the end of the run
    def finish(self, retain=False):
        """Comparison made and persisted first, then the home's removal, then the second look, then the
        run's record; whatever raises on the way, the later steps still run."""
        if self.outcome is not None:
            return self.outcome
        self._finishing = True
        outcome = {"confinement": CONFINEMENT_STATEMENT, "sockets": SOCKETS_STATEMENT, "residual": RESIDUAL_STATEMENT,
                   "admissions": self.admissions, "launches": [r["role"] for r in self.roles]}
        try:
            try:
                if self.state != "open":
                    raise LaunchRefused(f"run_{self.state}")
                if self.children:
                    self._pause(SETTLE_SECONDS, "settle wait", outcome)
                report = compare(self.before, (self.kit.homes_fd, self.entry["home"]), library=self.library,
                                 home=self.real, cache=self.cache)
                if report["verdict"] == "no web view data" and not any(r["webView"] for r in self.roles):
                    report["verdict"] = "clean"  # only a window can leave web view data
                report["launches"] = outcome["launches"]
            except BaseException as error:  # noqa: BLE001
                report = {"compared": False, "verdict": "not compared", "error": type(error).__name__}
            outcome["comparison"] = report
            try:
                _write_new(self.evidence_fd, f"{self.label}-comparison.json", report)
                outcome["comparisonPersisted"] = True
            except BaseException as error:  # noqa: BLE001
                outcome["comparisonPersisted"] = False
                report["verdict"] = "not compared"
                report["persistError"] = type(error).__name__
        finally:
            try:
                outcome["throwawayHome"] = self._remove(retain, outcome)
            except BaseException as error:  # noqa: BLE001
                outcome["throwawayHome"] = {"cleanup": "failed", "home": str(self.home), "removed": False,
                                            "reason": type(error).__name__}
            try:
                if self.evidence_fd is not None:
                    _write_new(self.evidence_fd, f"{self.label}-run.json", outcome)
            except BaseException:  # noqa: BLE001
                outcome["recordPersisted"] = False
            self._restore_signals()
            self.outcome = outcome
            self.state = "finished"
            self._finishing = False
            if self.kit:
                self.kit.close()
            if self.evidence_fd is not None:
                os.close(self.evidence_fd)
                self.evidence_fd = None
        if self._pending is not None:
            raise Interrupted(self._pending)
        return outcome

    def _set_state(self, state):
        """Record the home's state in the registry; a failed write is itself recorded, never raised."""
        self.entry["state"] = state
        try:
            self.kit.write_entry(self.entry)
        except Exception as error:  # noqa: BLE001
            self.registry_errors.append(f"{state}: {type(error).__name__}")

    def _drop(self):
        try:
            self.kit.drop_entry(self.entry["id"])
        except FileNotFoundError:
            pass
        except Exception as error:  # noqa: BLE001
            self.registry_errors.append(f"drop: {type(error).__name__}")

    def _remove(self, retain, outcome):
        if self.entry is None or self.kit is None:
            return {"cleanup": "not created", "removed": False}
        name = self.entry["home"]
        result = {"home": str(self.home), "statement": RESIDUAL_STATEMENT}
        if not self.kit.exists(name):
            self._drop()
            return {**result, "cleanup": "not created", "removed": False}
        if self.entry["state"] == "creating":
            # Made, but its record was not completed: it stays in the inventory, never removed blind.
            return {**result, "cleanup": "withheld", "removed": False,
                    "reason": "the home was made but its record was not completed; it is left for the inventory"}
        listing = home_listing((self.kit.homes_fd, name))
        try:
            _write_new(self.evidence_fd, f"{self.label}-listing.json", {"home": str(self.home), **listing})
            result["listing"] = f"{self.label}-listing.json"
        except BaseException as error:  # noqa: BLE001
            result["listingError"] = type(error).__name__
        if retain:
            self._set_state("withheld")
            return {**result, "cleanup": "withheld", "removed": False,
                    "reason": "the caller could not verify the processes the run started are gone; the home is left at this path"}
        try:
            writers, coverage = possible_writers(self.home, self.children, self.generations, self.groups, self.private)
            result["coverage"] = coverage
        except BaseException as error:  # noqa: BLE001  (any failure of the check withholds; none removes)
            self._set_state("withheld")
            return {**result, "cleanup": "withheld", "removed": False,
                    "reason": f"the check for processes that could still write here could not run ({type(error).__name__}); the home is left at this path"}
        if writers:
            self._set_state("withheld")
            return {**result, "cleanup": "withheld", "removed": False, "writers": writers,
                    "reason": "processes that could still write here are alive; the home is left at this path"}
        try:
            remove_at(self.kit.homes_fd, name, self.entry["identity"]["home"])
        except _Custody as error:
            self._set_state("refused")
            return {**result, "cleanup": "refused", "removed": False, "reason": str(error)}
        except OSError as error:
            self._set_state("failed")
            return {**result, "cleanup": "failed", "removed": False, "reason": f"{type(error).__name__}: {error}"[:500]}
        self._pause(SECOND_LOOK_SECONDS, "second look wait", outcome)
        if self.kit.exists(name):
            self._set_state("reappeared")
            result["registryErrors"] = self.registry_errors
            return {**result, "cleanup": "reappeared", "removed": True, "secondLook": "reappeared",
                    "reason": "the home's path reappeared after removal; it is a left home and the SWBPIPE Agent 0 must be told"}
        self._drop()
        return {**result, "cleanup": "removed", "removed": True, "secondLook": "not reappeared",
                "registryErrors": self.registry_errors}

    def __enter__(self):
        return self.begin()

    def __exit__(self, *exc):
        self.finish()
        return False


def remove_left_home(record):
    """Removal of a left home by the run that made it: its token, from its own record, must match the
    registry's; the processes recorded for it must be established gone; then the same removal."""
    kit = Kit(create=False)
    try:
        if PLATFORM != "darwin":
            raise LaunchRefused("platform_not_macos")
        entry = next((e for _, e in kit.entries() if e and e["id"] == record.get("id")), None)
        if entry is None:
            raise LaunchRefused("not_in_the_registry")
        if hashlib.sha256(str(record.get("token", "")).encode()).hexdigest() != entry["token"]:
            raise LaunchRefused("not_the_run_that_made_it")
        if kit.ids != {"tempRoot": entry["tempRoot"], "kitRoot": entry["kitRoot"]}:
            raise LaunchRefused("kit_folder_changed")
        try:
            table = _process_table()
            reasons = [s for g in entry["started"] if (s := _generation_alive(g, table))]
            if kit.exists(entry["home"]) and _open_under(kit.homes / entry["home"]):
                reasons.append("open under the home")
        except Exception as error:  # noqa: BLE001
            reasons = [f"the check could not run ({type(error).__name__})"]
        if reasons:
            return {"cleanup": "withheld", "home": entry["home"], "removed": False, "processes": reasons}
        try:
            removed = remove_at(kit.homes_fd, entry["home"], entry["identity"].get("home", [0, 0]))
        except _Custody as error:
            entry["state"] = "refused"
            kit.write_entry(entry)
            return {"cleanup": "refused", "home": entry["home"], "removed": False, "reason": str(error)}
        time.sleep(SECOND_LOOK_SECONDS)
        if kit.exists(entry["home"]):
            entry["state"] = "reappeared"
            kit.write_entry(entry)
            return {"cleanup": "reappeared", "home": entry["home"], "removed": True}
        kit.drop_entry(entry["id"])
        return {"cleanup": "removed" if removed else "not created", "home": entry["home"], "removed": removed}
    finally:
        kit.close()


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    commands = parser.add_subparsers(dest="command", required=True)
    launch = commands.add_parser("check-launch")
    launch.add_argument("--guard-dir")
    commands.add_parser("left-homes")
    remove = commands.add_parser("remove-left-home")
    remove.add_argument("--record", type=Path, required=True)
    args = parser.parse_args(argv)
    try:
        if args.command == "check-launch":
            print(json.dumps(check_launch(args.guard_dir)))
            return EXIT["clean"]
        if args.command == "left-homes":
            kit = Kit(create=False)
            try:
                inventory = left_homes(kit)
            finally:
                kit.close()
            print(json.dumps(inventory))
            return EXIT["refused"] if inventory["refuse"] else EXIT["clean"]
        outcome = remove_left_home(json.loads(args.record.read_text()))
        print(json.dumps(outcome))
        return {"removed": 0, "not created": 0, "refused": 2, "reappeared": 7}.get(outcome["cleanup"], EXIT["not removed"])
    except LaunchRefused as refusal:
        print(f"refused: {refusal}", file=sys.stderr)
        return EXIT["refused"]
    except (OSError, ValueError, KeyError) as error:
        print(f"unusable input: {type(error).__name__}: {error}", file=sys.stderr)
        return EXIT["refused"]


if __name__ == "__main__":
    raise SystemExit(main())
