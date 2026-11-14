#!/usr/bin/env python3
"""Confine native launches of the desktop application to a throwaway home, record custody, finish.

One object, `Run`, owns a launch from admission to its finish; the module starts nothing else but the
four fixed utilities in their fixed invocation shapes. The helper REMOVES NOTHING: it admits,
confines, records custody (held across the death of a parent), compares, lists the home and marks
the run finished in the kit's inventory. Every home a run leaves stays in place, inventoried by
name and state, until the owning lane removes it with the separate removal command; "run finished"
never means "safe to remove".

The application is confined as to where it reads and writes its own files, and as to what
environment it is handed; nothing more. With no operating system boundary around the application
itself, sockets elsewhere in the account stay reachable to it, and home redirection cannot change
that. It launches on macOS only. See native_launch_guard.md, which states the adversary model.

Command line (a Node runner calls it with `/usr/bin/python3 -I`):

  native_launch_guard.py check-launch [--guard-dir DIR]
  native_launch_guard.py homes

Exit status, stable: 0 clean; 1 a product entry changed; 2 refused or unusable input; 3 no web view
data in the throwaway home; 5 the comparison failed; 6 the web view's helpers wrote their cache
folders under the system's per-user cache folder.
"""
from __future__ import annotations

# Its directory scans read the account's Library, the system's per-user cache and temporary
# folders, a throwaway home and one driver file named by its caller, never other repository files
# (tools/ci/python_test_reads.py).
CI_READS = ()

import argparse
import atexit
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
REPOSITORY = Path(__file__).resolve().parents[1]
WATCHED_TOPS = ("WebKit", "Caches", "HTTPStorages", "Application Support", "Preferences", "Saved Application State", "Logs")
WEB_VIEW_TOPS = ("WebKit", "Caches")
HOME_FOLDERS = ("", "Desktop", "Documents", "Downloads")  # times by a single stat each; never listed
PROTECTED_FOLDERS = ("Desktop", "Documents", "Downloads", "Library")  # no caller location under these
# The system's per-user cache folder, named by what it is and never by its path on a machine.
CACHE_TOP = "<system per-user cache folder>"
HELPER_CACHE = re.compile(r"^com\.apple\.WebKit\.(?:Networking|WebContent(?:\.EnhancedSecurity)?|GPU)\+(.+)$")
HOME_LISTING_KEPT = 2000
MAX_DEPTH = 64
SETTLE_SECONDS = 20  # the web view's helpers may write after the application exits
SAMPLE_SECONDS = 1.0  # descendants are sampled this often while a run is open
PATH = "/usr/bin:/bin"  # the fixed PATH handed to the application
LOCALE = {"LANG": "en_US.UTF-8"}
# The helper's own utility programs: named by absolute path, in fixed invocation shapes, given this
# environment, never the caller's. Nothing else can be started through `_read`.
PS, LSOF, IOREG, GETCONF = "/bin/ps", "/usr/sbin/lsof", "/usr/sbin/ioreg", "/usr/bin/getconf"
NUMBER, ABSOLUTE = object(), object()
UTILITY_SHAPES = {
    PS: [("-axww", "-o", "pid=,ppid=,pgid=,uid=,lstart=,comm=")],
    LSOF: [("-n", "-P", "-F", "n", "-p", NUMBER), ("-n", "-P", "-F", "p", "+D", ABSOLUTE)],
    IOREG: [("-n", "Root", "-d1", "-a")],
    GETCONF: [("DARWIN_USER_TEMP_DIR",), ("DARWIN_USER_CACHE_DIR",)],
}
UTIL_ENV = {"PATH": "/usr/bin:/bin:/usr/sbin:/sbin", "LANG": "C", "LC_ALL": "C"}
CHECK_TIMEOUT = 60
# The strict chooser mode: the helper sets this variable on every launch. Once the product change
# (pull request 283) lands, a chooser asked with nothing queued ends the application with
# STRICT_CHOOSER_EXIT before any system dialog, and a queued answer outside the home is refused.
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
FIXTURE_MODES = ("preflight", "idle", "active", "pending")
STREAM_OPTIONS = ("stdin", "stdout", "stderr")
STREAMS = (subprocess.PIPE, subprocess.DEVNULL)
SLOT_LOCK = re.compile(r"^cargo_job(\.slot\d+)?\.lock$")
LOCK_PROGRAM = "/usr/bin/lockf"
KIT_DIR = "swbpipe-launch-kit"
ENTRY_KEYS = {"id", "label", "home", "state", "made", "maker", "token", "tempRoot", "kitRoot", "identity", "started",
              "groups", "private", "samples", "findings", "executable", "updated"}
# A home's state in the inventory. None of them means "safe to remove"; removal is the separate
# command's judgment, made when it runs.
STATES = ("creating", "open", "finished", "left")
EXIT = {"clean": 0, "product": 1, "refused": 2, "no web view data": 3, "comparison failed": 5,
        "web view helper cache": 6}
O_DIR = os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW | os.O_CLOEXEC
WATCH_STATEMENT = ("Nothing outside the real Library's watched folders (WebKit, Caches, HTTPStorages, Application "
                   "Support, Preferences, Saved Application State and Logs, walked in full; every other top folder "
                   "one level deep, by name), the system's per-user cache folder and the throwaway home was watched; "
                   "the times of the home, Desktop, Documents and Downloads folders were read by a single stat each, "
                   "and nothing inside them was listed or opened.")
OTHERS_STATEMENT = ("Records written during the window by the operating system's own services or by other software "
                    "exist under the account's real Library. They are counted by class and by top folder, never "
                    "named, opened, edited or removed, and they do not fail the run.")
WINDOW_STATEMENT = ("The comparison covers the window named here. Two launches whose windows overlap cannot be told "
                    "apart; the journey controller takes a host-wide lock, and other runners are not serialised by "
                    "this helper.")
LIMITS_STATEMENT = ("A clean comparison establishes neither that nothing was read nor that nothing was written "
                    "elsewhere on the disk.")
CONFINEMENT_STATEMENT = ("The application is confined as to where it reads and writes its own files, and as to what "
                         "environment it is handed.")
SOCKETS_STATEMENT = ("With no operating system boundary around the application itself, sockets elsewhere in the "
                     "account stay reachable to it, and home redirection cannot change that.")
FINISHED_STATEMENT = ("The helper removes nothing. A finished home may still be written to by anything of the launch "
                      "that is alive; it stays in place, in the kit's inventory, until its lane removes it with the "
                      "separate removal command.")
OUTSIDE_STATEMENT = ("The claim is bounded, not absolute protection against every process of the account. Outside "
                     "the adversary model, recorded and not repaired: an unrelated, deliberately adversarial process "
                     "of the same account that forges or races the kit's own folders, registry or records. What the "
                     "helper, which removes nothing, adds to such a process's reach: it can make admission refuse, or "
                     "make the inventory misreport the kit's own homes; it cannot make the helper remove, open or "
                     "write anything outside the kit's own folder and the run's evidence folder.")
HELPER_CACHE_STATEMENT = ("The web view's helper processes, which the system starts, keep cache folders named for the "
                          "executable under the system's per-user cache folder. The system gives that folder "
                          "independently of the home, and whether both home variables redirect the helpers' folders "
                          "is not established; where they do not, it is a write outside the throwaway home that "
                          "confinement does not reach, observed by the comparison, not prevented. The run stops on "
                          "it. Nothing there is opened, edited or removed.")
POSITIVE_LIMIT = ("Any file under the throwaway home's Library/WebKit or Library/Caches satisfies this check; it "
                  "does not identify the web view's own data.")
STRICT_CHOOSER_STATEMENT = ("The executable contains the strict chooser variable's name: a lead, not a proof. The tie "
                            "is the comparison test against the product's constants; the protection itself is the "
                            "product's, in pull request 283, and nothing of it is claimed until that is on main.")
SIGNS_NOT_USED = {
    "screenSaver": "not used: which process runs while the screen saver is active could not be established by a harmless read",
    "displaySleep": "not used: the display's power value while asleep could not be established by a harmless read",
}
# Held at import, so a caller replacing subprocess.Popen replaces neither the kit's starts nor its
# read-only utility commands; kept apart so a test that stands in for the start keeps the reads real.
_POPEN = _READ_POPEN = subprocess.Popen


class LaunchRefused(Exception):
    """A launch, location or removal the kit will not make, named."""


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


def _shape(args, shape):
    return len(args) == len(shape) and all(
        (re.fullmatch(r"\d+", a) if s is NUMBER else os.path.isabs(a) if s is ABSOLUTE else a == s)
        for a, s in zip(args, shape))


def _read(argv, timeout=CHECK_TIMEOUT):
    """Run one of the four fixed utilities in one of its fixed invocation shapes, with the helper's own
    minimal environment. Anything else is refused: this is not a way to start a program."""
    argv = [str(a) for a in argv]
    if not argv or not any(_shape(argv[1:], shape) for shape in UTILITY_SHAPES.get(argv[0], ())):
        raise LaunchRefused("utility_not_allowed")
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
# Admission: the system, the screen, the slot.

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
    """None when the console session entries for this account are found, agree, are on the console and
    carry no locked key (any value of it refuses); otherwise the reason. Anything unparsed refuses."""
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
    """The open-files lister's field output. Its diagnostics are kept: anything on standard error, or a
    status other than 0 or 1 (1 is also its answer when it found holders but not the folder itself
    open), is a failed listing."""
    status, out, err = _read(argv)
    if status not in (0, 1) or err.strip():
        raise OSError(f"lsof exit {status}: {err.strip()[:200]}")
    return out.decode("utf-8", "replace").splitlines()


def _open_names(pid):
    names = []
    for line in _lsof_lines([LSOF, "-n", "-P", "-F", "n", "-p", str(pid)]):
        if line.startswith("n"):
            names.append(line[1:])
        elif not re.fullmatch(r"[pf]\S*", line):
            raise OSError("open files listing unparsed")
    return names


def _open_under(path):
    """pids of this account's processes with an open file or working directory under `path`; an
    unparsed listing or any diagnostic raises."""
    pids = set()
    for line in _lsof_lines([LSOF, "-n", "-P", "-F", "p", "+D", str(path)]):
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


# ---------------------------------------------------------------------------
# Locations: every location a caller supplies is checked before anything is created or read.

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


def _protected(path):
    real = Path(os.path.realpath(real_home()))
    return next((f for f in PROTECTED_FOLDERS if path == real / f or (real / f) in path.parents), None)


def check_location(path, kind):
    """A caller-supplied location, before anything is created or read: absolute, canonical (no link
    on it), and not under the account's Desktop, Documents, Downloads or Library. `kind` "evidence"
    (it may not exist yet; its parent must, and it may not be under this repository) or "resource"
    (an existing regular file, not a link)."""
    path = Path(path)
    if not path.is_absolute():
        raise LaunchRefused(f"{kind}_not_absolute")
    path = Path(os.path.normpath(path))
    folder = _protected(path)
    if folder:
        raise LaunchRefused(f"{kind}_under_{folder.lower()}")
    if kind == "evidence":
        repository = Path(os.path.realpath(REPOSITORY))
        if path == repository or repository in path.parents:
            raise LaunchRefused("evidence_under_the_repository")
        anchor = path if os.path.lexists(path) else path.parent
    else:
        anchor = path
    if os.path.realpath(anchor) != str(anchor):
        raise LaunchRefused(f"{kind}_path_has_a_link")
    if kind == "resource":
        try:
            st = os.lstat(path)
        except OSError:
            raise LaunchRefused("resource_missing") from None
        if not stat.S_ISREG(st.st_mode):
            raise LaunchRefused("resource_not_a_regular_file")
    return path


def open_folder(path, create=False):
    """Open (or make, then open) a folder by walking from the root through folder descriptors, each
    component opened without following a link: a location checked earlier cannot be swapped for a
    link on the way. Returns the descriptor."""
    path = check_location(path, "evidence")
    fd = os.open("/", os.O_RDONLY | os.O_DIRECTORY | os.O_CLOEXEC)
    parts = path.parts[1:]
    try:
        for index, part in enumerate(parts):
            if create and index == len(parts) - 1:
                try:
                    os.mkdir(part, 0o700, dir_fd=fd)
                except FileExistsError:
                    raise LaunchRefused("evidence_exists") from None
            try:
                child = os.open(part, O_DIR, dir_fd=fd)
            except OSError:
                raise LaunchRefused("evidence_not_a_real_folder") from None
            os.close(fd)
            fd = child
        return fd
    except BaseException:
        os.close(fd)
        raise


def open_file(path):
    """Open a regular file by walking from the root through folder descriptors, every component,
    the file included, opened without following a link. Returns the file's descriptor."""
    path = Path(os.path.normpath(path))
    if not path.is_absolute():
        raise LaunchRefused("file_not_absolute")
    fd = os.open("/", os.O_RDONLY | os.O_DIRECTORY | os.O_CLOEXEC)
    try:
        for part in path.parts[1:-1]:
            try:
                child = os.open(part, O_DIR, dir_fd=fd)
            except OSError:
                raise LaunchRefused("file_path_not_real_folders") from None
            os.close(fd)
            fd = child
        try:
            handle = os.open(path.name, os.O_RDONLY | os.O_NOFOLLOW | os.O_CLOEXEC, dir_fd=fd)
        except OSError:
            raise LaunchRefused("file_not_openable_without_a_link") from None
        if not stat.S_ISREG(os.fstat(handle).st_mode):
            os.close(handle)
            raise LaunchRefused("file_not_regular")
        return handle
    finally:
        os.close(fd)


def make_evidence(path):
    """The tools' output folder: checked, then made through a verified descriptor walk. Returns its path."""
    os.close(open_folder(path, create=True))
    return Path(os.path.normpath(path))


def _write_new(dir_fd, name, data):
    """A new file in the evidence folder, never through a link and never over another file."""
    fd = os.open(name, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW | os.O_CLOEXEC, 0o600, dir_fd=dir_fd)
    with os.fdopen(fd, "w") as handle:
        handle.write(json.dumps(data, indent=1) + "\n")


# ---------------------------------------------------------------------------
# The product's identity, from its executable and bundle.

def product_identity(executable, extra=()):
    """The executable, canonical and absolute, a regular file inside a bundle whose Info.plist names it,
    and the names the comparison must watch: the bundle identifier and the executable's name (and,
    by matching, its underscore form). Failure to establish them refuses. Callers may add names."""
    executable = Path(executable)
    if not executable.is_absolute() or os.path.realpath(executable) != str(executable):
        raise LaunchRefused("executable_not_canonical")
    check_location(executable, "resource")
    if executable.parent.name != "MacOS" or executable.parent.parent.name != "Contents":
        raise LaunchRefused("executable_not_in_a_bundle")
    try:
        fd = os.open(executable.parents[1] / "Info.plist", os.O_RDONLY | os.O_NOFOLLOW | os.O_CLOEXEC)
        with os.fdopen(fd, "rb") as handle:
            info = plistlib.load(handle)
        bundle = info["CFBundleIdentifier"]
        named = info["CFBundleExecutable"]
    except (OSError, ValueError, KeyError, plistlib.InvalidFileException):
        raise LaunchRefused("bundle_identity_unestablished") from None
    if named != executable.name or not isinstance(bundle, str) or not re.fullmatch(r"[A-Za-z0-9.\-]{3,200}", bundle):
        raise LaunchRefused("bundle_does_not_name_the_executable")
    names = {bundle, executable.name}
    for name in extra:
        if not isinstance(name, str) or len(name) < 4 or "/" in name or not re.fullmatch(r"[A-Za-z0-9._\-]+", name):
            raise LaunchRefused("added_name_not_allowed")
        names.add(name)
    st = os.lstat(executable)
    return {"executable": str(executable), "bundle": bundle, "names": sorted(names),
            "identity": [st.st_dev, st.st_ino, st.st_size, st.st_mtime_ns]}


def private_copy(executable):
    """The private copy the executable belongs to: its bundle."""
    return Path(executable).parents[2]


def strict_chooser_lead(fd):
    """Whether the executable's bytes, read through the descriptor the run holds, contain the strict
    chooser variable's name. Read, never run."""
    needle = STRICT_CHOOSER_ENV.encode()
    os.lseek(fd, 0, os.SEEK_SET)
    tail = b""
    while chunk := os.read(fd, 1 << 20):
        if needle in tail + chunk:
            return True
        tail = (tail + chunk)[-(len(needle) - 1):]
    return False


# ---------------------------------------------------------------------------
# The kit's folder and registry, held by descriptor.

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
    again."""

    def __init__(self, create=True):
        self.temp = temp_root()
        self.temp_fd = os.open(self.temp, os.O_RDONLY | os.O_DIRECTORY | os.O_CLOEXEC)
        self.fds = [self.temp_fd]
        self.lock = threading.Lock()
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
        with self.lock:
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
                        data = json.loads(handle.read(1 << 22))
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


# ---------------------------------------------------------------------------
# Custody: what is recorded of a launch, and the one holder check made from it. The helper uses the
# check for admission's inventory and for the state a finished run is given; it removes nothing and
# never judges whether a home may be removed.

def _open_child(parent_fd, name, seen):
    """Open a folder seen by lstat, without following a link, and only if it is still that folder."""
    fd = os.open(name, O_DIR, dir_fd=parent_fd)
    opened = os.fstat(fd)
    if (opened.st_dev, opened.st_ino) != (seen.st_dev, seen.st_ino):
        os.close(fd)
        raise _Custody("changed")
    return fd


def unreadable_inside(fd):
    """Every folder inside an open home that cannot be opened and listed, by place within the home.
    The application leaving one is itself a finding about the application."""
    places = []

    def walk(dir_fd, relative, depth):
        try:
            names = os.listdir(dir_fd)
        except OSError:
            places.append(relative or ".")
            return
        for name in sorted(names):
            try:
                seen = os.stat(name, dir_fd=dir_fd, follow_symlinks=False)
            except FileNotFoundError:
                continue
            if not stat.S_ISDIR(seen.st_mode):
                continue
            child = f"{relative}/{name}" if relative else name
            if depth + 1 > MAX_DEPTH:
                places.append(child)
                continue
            try:
                child_fd = _open_child(dir_fd, name, seen)
            except (OSError, _Custody):
                places.append(child)
                continue
            try:
                walk(child_fd, child, depth + 1)
            finally:
                os.close(child_fd)
    walk(fd, "", 0)
    return places


def _generation_alive(generation, table):
    """'alive', 'unestablished' or None for a recorded (identifier, start time) generation."""
    row = table.get(generation["pid"])
    if row is None:
        return None
    if generation.get("start") is None:
        return "unestablished"
    return "alive" if row[4] == generation["start"] else None


def holders(kit, entry, children=(), expected=True):
    """The one holder check, from what is recorded for a home: 'absent', 'none found', 'alive' or
    'unestablished', with why. The home is opened through the kit's held descriptor without following
    a link; its identity must be the recorded one (when `expected`) and its path must still identify
    that same folder, else the result is unestablished. Then, from one process listing and one
    open-files listing of the home's path: a process started and not seen to end; a recorded
    generation still alive or whose start time could not be read; a descendant of a living
    generation; a member of a recorded group; a process whose executable lies under the home or the
    private copy; a holder of a file or working directory under the home. An incomplete listing, or
    a folder inside the home that cannot be listed, is unestablished: never 'none found'."""
    name = entry["home"]
    path = kit.homes / name
    result = {"home": name}
    try:
        fd = os.open(name, O_DIR, dir_fd=kit.homes_fd)
    except FileNotFoundError:
        return {**result, "holders": "absent"}
    except OSError as error:
        return {**result, "holders": "unestablished", "reason": f"unopenable: {errno.errorcode.get(error.errno, error.errno)}"}
    try:
        held = _fid(fd)
        if expected and held != list(entry["identity"].get("home", [])):
            return {**result, "holders": "unestablished", "reason": "the folder is not the one the kit made"}
        try:
            by_path = os.stat(path)
        except OSError:
            by_path = None
        if by_path is None or [by_path.st_dev, by_path.st_ino] != held:
            return {**result, "holders": "unestablished", "reason": "the home's path no longer identifies the held folder"}
        unreadable = unreadable_inside(fd)
    finally:
        os.close(fd)
    if unreadable:
        result["unreadableInside"] = {"count": len(unreadable), "places": unreadable[:50]}
    try:
        if PLATFORM != "darwin":
            raise OSError("the holder check is made on macOS only")
        table = _process_table()
        opened = _open_under(path)
    except BaseException as error:  # noqa: BLE001  (a check that cannot run is never 'none found')
        return {**result, "holders": "unestablished", "reason": f"the process check could not run ({type(error).__name__}: {error})"[:400]}
    roots = [str(path), *([entry["private"]] if entry.get("private") else [])]
    groups = set(entry.get("groups") or [])
    found = {}
    for child in children:
        if child.poll() is None:
            found.setdefault(child.pid, []).append("started by the run and not seen to end")
    living = []
    for generation in entry.get("started") or []:
        state = _generation_alive(generation, table)
        if state:
            found.setdefault(generation["pid"], []).append(f"recorded generation {state} ({generation.get('role')})")
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
            found.setdefault(pid, []).append("a descendant of a living generation")
        if pgid in groups:
            found.setdefault(pid, []).append("in a group of the run")
    found.pop(os.getpid(), None)
    coverage = {"processRows": len(table), "openFilesListed": True, "childrenChecked": len(list(children)),
                "generationsChecked": len(entry.get("started") or []), "groupsChecked": sorted(groups),
                "samples": entry.get("samples")}
    if unreadable:
        return {**result, "holders": "unestablished", "coverage": coverage,
                "reason": "folders inside the home cannot be listed, so a holder there cannot be excluded"}
    if found:
        writers = [{"pid": pid, "executable": table.get(pid, (0, 0, 0, "", ""))[3], "why": sorted(set(why))}
                   for pid, why in sorted(found.items())]
        # Only generations whose start time could not be read: whether they are alive is not known.
        unknown = all(all("unestablished" in w for w in writer["why"]) for writer in writers)
        return {**result, "holders": "unestablished" if unknown else "alive", "writers": writers, "coverage": coverage}
    return {**result, "holders": "none found", "coverage": coverage}


def inventory(kit, exclude=None):
    """Every home of this kit on disk, by name and state, with a count: finished and not removed;
    left because a holder may be alive; open; creating; unknown custody (a folder the registry does
    not hold). A launch is refused while any of them is found held (alive) or cannot be established
    free of holders, and for an unreadable registry entry; a finished home with no holder found is
    reported and does not by itself refuse."""
    report, blocking, known = [], [], set()
    for name, entry in kit.entries():
        if entry is None:
            report.append({"entry": name, "state": "unreadable entry"})
            blocking.append(f"{name}: unreadable entry")
            continue
        known.add(entry["home"])
        if entry["id"] == exclude:
            continue
        found = holders(kit, entry, expected=entry["state"] != "creating")
        if found["holders"] == "absent":
            report.append({"id": entry["id"], "home": entry["home"], "state": f"{entry['state']}, folder absent"})
            continue
        state = "left" if found["holders"] in ("alive", "unestablished") else entry["state"]
        report.append({"id": entry["id"], "home": entry["home"], "label": entry["label"], "state": state,
                       "recordedState": entry["state"], "holders": found["holders"], "reason": found.get("reason"),
                       "writers": found.get("writers"), "unreadableInside": found.get("unreadableInside")})
        if found["holders"] in ("alive", "unestablished"):
            blocking.append(f"{entry['home']}: {found['holders']}")
    for name in sorted(os.listdir(kit.homes_fd)):
        if name in known:
            continue
        unknown = {"home": name, "identity": {}, "started": [], "groups": [], "private": ""}
        found = holders(kit, unknown, expected=False)
        report.append({"home": name, "state": "unknown custody", "holders": found["holders"], "reason": found.get("reason")})
        if found["holders"] in ("alive", "unestablished"):
            blocking.append(f"{name}: unknown custody, {found['holders']}")
    counts = {}
    for item in report:
        counts[item["state"]] = counts.get(item["state"], 0) + 1
    return {"homes": report, "count": len(report), "byState": counts, "refuse": bool(blocking), "reasons": blocking,
            "statement": "The homes hold test data only. Each lane reports this count in its state; none outlives "
                         "the undertaking."}


# ---------------------------------------------------------------------------
# The environment, the plan and the standard input frames.

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


def _send(stream, data, digest=None):
    """Write every byte to the application's standard input and flush it, hashing exactly what each
    write took; returns the count."""
    view, total = memoryview(data), 0
    while total < len(data):
        count = stream.write(view[total:])
        count = len(data) - total if count is None else count
        if digest is not None:
            digest.update(view[total:total + count])
        total += count
    stream.flush()
    return total


class Frame:
    """Bytes for the application's standard input that the helper built from validated parts; the
    record says what they were, never a key."""

    def __init__(self, data, record):
        self.data, self.record = data, record


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
    """What a walk found: watched entries, counts of others in the window, and what could not be listed."""

    def __init__(self, names, since=None, until=None):
        self.names, self.since, self.until = names, since, until
        self.entries, self.unreadable_product, self.unreadable_other = {}, [], 0
        self.changed, self.too_deep, self.failed = 0, 0, []
        self.others = {"total": 0, "byFolder": {}}

    def entry(self, relative, found, top):
        if watched(relative, self.names):
            self.entries[relative] = found
        elif self.since is not None and (self.since <= found["mtime"] <= self.until or self.since <= found["birth"] <= self.until):
            self.others["total"] += 1
            self.others["byFolder"][top] = self.others["byFolder"].get(top, 0) + 1

    def unreadable(self, relative):
        if watched(relative, self.names):
            self.unreadable_product.append(relative)
        else:
            self.unreadable_other += 1


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


def _readable_at(fd, name, seen):
    """Whether a product-named folder found one level down can be opened and listed; nothing deeper."""
    try:
        child = _open_child(fd, name, seen)
    except (OSError, _Custody):
        return False
    try:
        os.listdir(child)
        return True
    except OSError:
        return False
    finally:
        os.close(child)


def _one_level(fd, top, sink):
    """A top folder outside the watched set: its immediate children by name and lstat only; a
    product-named child folder is checked to be readable, without walking it."""
    try:
        names = sorted(os.listdir(fd))
    except OSError:
        sink.unreadable(top)
        return
    for name in names:
        relative = f"{top}/{name}"
        try:
            seen = os.stat(name, dir_fd=fd, follow_symlinks=False)
        except FileNotFoundError:
            continue
        except OSError:
            sink.unreadable(relative)
            continue
        sink.entry(relative, facts(seen), top)
        if stat.S_ISDIR(seen.st_mode) and watched(relative, sink.names) and not _readable_at(fd, name, seen):
            sink.unreadable(relative)


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
# The run.

_OPEN_RUNS = []  # runs begun in this process and not yet finished: finished at exit if forgotten


def _finish_forgotten():
    for run in list(_OPEN_RUNS):
        try:
            run.finish(forgotten=True)
        except BaseException:  # noqa: BLE001
            pass


atexit.register(_finish_forgotten)


class Run:
    """One launch, from admission to its finish: the only way the kit starts the application. It
    removes nothing. A run begun and never finished is finished at the interpreter's exit; one killed
    outright stays in the inventory, open, and the holder check at the next admission decides whether
    it blocks."""

    library = None  # the real Library; offline tests alone set another
    cache = None  # the system's per-user cache folder; offline tests alone set another
    real = None  # the account's home folder for the four folder times; offline tests alone set another

    def __init__(self, executable, evidence, label="home", extra_names=(), guard_dir=None):
        if not re.fullmatch(r"[a-z0-9][a-z0-9._-]{0,40}", label):
            raise LaunchRefused("run_label")
        self.product = product_identity(executable, extra_names)
        self.executable = Path(self.product["executable"])
        self.evidence = check_location(evidence, "evidence")
        self.label = label
        self.names = self.product["names"]
        self.guard_dir = guard_dir
        self.private = private_copy(self.executable)
        self.kit = self.entry = self.before = self.evidence_fd = self.executable_fd = None
        self.children, self.roles, self.admissions, self.frames = [], [], [], []
        self.token = None
        self.state = "new"
        self.outcome = None
        self._finishing = False
        self._pending = None
        self._previous = {}
        self._lock = threading.Lock()
        self._stop = threading.Event()
        self._sampler = None

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
    def _executable_unchanged(self):
        st = os.lstat(self.executable)
        held = os.fstat(self.executable_fd)
        now = [st.st_dev, st.st_ino, st.st_size, st.st_mtime_ns]
        if now != self.product["identity"] or [held.st_dev, held.st_ino] != now[:2]:
            raise LaunchRefused("executable_changed")

    def _admit(self):
        admission = {"launch": check_launch(self.guard_dir)}
        self._executable_unchanged()
        if not strict_chooser_lead(self.executable_fd):
            raise LaunchRefused("strict_chooser_absent_from_executable")
        admission["strictChooser"] = STRICT_CHOOSER_STATEMENT
        admission["product"] = {"bundle": self.product["bundle"], "names": self.names}
        homes = inventory(self.kit, exclude=self.entry["id"] if self.entry else None)
        admission["homes"] = {"count": homes["count"], "byState": homes["byState"], "named": homes["homes"]}
        if homes["refuse"]:
            raise LaunchRefused("a_home_of_this_kit_may_be_held:" + "; ".join(homes["reasons"])[:400])
        admission["at"] = stamp(time.time())
        self.admissions.append(admission)
        return admission

    def begin(self):
        """Admit, take and check the baseline, then make the home, entered in the registry first."""
        if self.state != "new":
            raise LaunchRefused("run_already_begun")
        self.state = "beginning"
        _OPEN_RUNS.append(self)
        self.evidence_fd = open_folder(self.evidence)
        self.executable_fd = os.open(self.executable, os.O_RDONLY | os.O_NOFOLLOW | os.O_CLOEXEC)
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
                      "started": [], "groups": [], "private": str(self.private),
                      "samples": {"taken": 0, "missed": []}, "findings": {}, "executable": str(self.executable),
                      "updated": ""}
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

    # -- the environment and the frames
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

    def fixture_frame(self, *, model, context_window, endpoint, key, mode, process_probe, boundary_probe):
        """The fixture's first frame, built here: its state folder inside the run's home, the probe
        programs regular files that are not links and not under the account's protected folders."""
        if self.state != "open":
            raise LaunchRefused("run_not_open")
        probes = []
        for probe in (process_probe, boundary_probe):
            checked = check_location(Path(probe), "resource")
            os.close(open_file(checked))  # a real path to a regular file at the moment it is framed
            probes.append(str(checked))
        if mode not in FIXTURE_MODES:
            raise LaunchRefused("fixture_mode_not_known")
        if not isinstance(model, str) or _pathlike(model) or not isinstance(context_window, int) \
                or not isinstance(endpoint, str) or not ENDPOINT.match(endpoint) or not (key is None or isinstance(key, str)):
            raise LaunchRefused("fixture_frame_shape")
        state = self.work / ("preflight" if mode == "preflight" else "state")
        home_fd = os.open(self.entry["home"], O_DIR, dir_fd=self.kit.homes_fd)
        try:
            if _fid(home_fd) != list(self.entry["identity"]["home"]):
                raise LaunchRefused("home_changed")
            work_fd = os.open("work", O_DIR, dir_fd=home_fd)
            try:
                if _fid(work_fd) != list(self.entry["identity"]["work"]):
                    raise LaunchRefused("home_changed")
                os.mkdir(state.name, 0o700, dir_fd=work_fd)
            finally:
                os.close(work_fd)
        finally:
            os.close(home_fd)
        data = {"state": str(state), "model": model, "contextWindow": context_window, "endpoint": endpoint, "key": key,
                "mode": mode, "processProbe": probes[0], "boundaryProbe": probes[1]}
        return Frame(json.dumps(data).encode() + b"\n",
                     {"kind": "fixture", "state": f"work/{state.name}", "mode": mode, "probes": probes})

    def driver_frame(self, relative):
        """The native-smoke driver, bound to a file inside this repository and read through a verified
        descriptor walk; the hash of the bytes actually written to the application's standard input goes
        in the run's record."""
        path = Path(os.path.normpath(REPOSITORY / relative))
        if REPOSITORY not in path.parents:
            raise LaunchRefused("driver_not_in_the_repository")
        with os.fdopen(open_file(path), "rb") as handle:
            data = handle.read()
        return Frame(data, {"kind": "driver", "file": str(Path(relative))})

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

    def start(self, role, argv, kit=None, web_view=False, frame=None, key_line=None, close_stdin=False, **streams):
        """Start the run's executable with the helper's environment, working directory and streams, in a
        process group of its own, every other descriptor closed; record its identity at once; write the
        helper's frame (and the provider key line before it) to its standard input. Returns the handle."""
        if self.state != "open":
            raise LaunchRefused("run_not_open")
        if [str(a) for a in argv[:1]] != [str(self.executable)]:
            raise LaunchRefused("executable_not_the_runs")
        for name, value in streams.items():
            if name not in STREAM_OPTIONS:
                raise LaunchRefused(f"process_option_not_allowed:{name}")
            if value not in STREAMS:
                raise LaunchRefused(f"stream_not_allowed:{name}")
        if frame is not None and (not isinstance(frame, Frame) or streams.get("stdin") != subprocess.PIPE):
            raise LaunchRefused("frame_not_the_helpers")
        if key_line is not None and frame is None:
            raise LaunchRefused("key_line_needs_a_frame")
        self._admit()
        env = self.environment(kit)
        self._verify_paths()
        self._executable_unchanged()
        process = _POPEN([str(self.executable), *[str(a) for a in argv[1:]]], env=env, cwd=str(self.work),
                         close_fds=True, pass_fds=(), restore_signals=True, process_group=0,
                         **{name: streams.get(name, subprocess.DEVNULL) for name in STREAM_OPTIONS})
        self.children.append(process)
        generation = {"pid": process.pid, "start": None, "role": role}
        with self._lock:
            self.entry["started"] = [*self.entry["started"], generation]
            self.entry["groups"] = sorted(set(self.entry["groups"]) | {process.pid})
            self.roles.append({"role": role, "webView": bool(web_view)})
        try:
            generation["start"] = _process_table().get(process.pid, (0, 0, 0, "", None))[4]
        except Exception:  # noqa: BLE001  (unestablished: withholds removal)
            generation["start"] = None
        self.kit.write_entry(self.entry)
        self._start_sampler()
        if frame is not None:
            if key_line is not None:
                _send(process.stdin, key_line.encode() + b"\n")
            sent = hashlib.sha256()
            written = _send(process.stdin, frame.data, sent)
            # What the application was actually handed: the hash of the frame's bytes as written to the
            # pipe (a key line before it is written and never recorded).
            self.frames.append({"role": role, **frame.record, "bytesSent": written, "sha256Sent": sent.hexdigest()})
            if close_stdin:
                process.stdin.close()
        return process

    def attribute(self, group=None):
        """Add a process group to what the run answers for; it can only widen the check."""
        if isinstance(group, int) and group > 1 and self.entry:
            with self._lock:
                self.entry["groups"] = sorted(set(self.entry["groups"]) | {group})
            try:
                self.kit.write_entry(self.entry)
            except Exception:  # noqa: BLE001
                pass

    # -- custody as it goes
    def sample(self):
        """One sample: every living process descended from a recorded generation, or in a group of the
        run, is recorded as a generation of its own (identifier and start time together), so a survivor
        is known by its own record after its parent is gone. A sample that cannot be taken is recorded
        as missed."""
        if self.entry is None or self.state != "open":
            return
        try:
            table = _process_table()
        except Exception as error:  # noqa: BLE001
            with self._lock:
                self.entry["samples"]["missed"].append({"at": stamp(time.time()), "why": type(error).__name__})
            return
        with self._lock:
            known = {(g["pid"], g["start"]) for g in self.entry["started"]}
            living = {g["pid"] for g in self.entry["started"] if _generation_alive(g, table) == "alive"}
            groups = set(self.entry["groups"])
            children_of = {}
            for pid, row in table.items():
                children_of.setdefault(row[0], []).append(pid)
            stack, lineage = list(living), set()
            while stack:
                pid = stack.pop()
                if pid not in lineage:
                    lineage.add(pid)
                    stack.extend(children_of.get(pid, []))
            added = False
            for pid, (ppid, pgid, uid, exe, start) in table.items():
                if pid == os.getpid() or uid != os.getuid():
                    continue
                if (pid in lineage or pgid in groups) and (pid, start) not in known:
                    self.entry["started"].append({"pid": pid, "start": start, "role": "descendant"})
                    added = True
                if pid in lineage and pgid == pid and pgid not in groups:
                    # A descendant that made a group of its own (the supplier and the solve worker
                    # do): the group is held in the record, so its members are found after
                    # their parent is gone.
                    self.entry["groups"] = sorted(set(self.entry["groups"]) | {pgid})
                    groups.add(pgid)
                    added = True
            self.entry["samples"]["taken"] += 1
        if added:
            try:
                self.kit.write_entry(self.entry)
            except Exception:  # noqa: BLE001
                with self._lock:
                    self.entry["samples"]["missed"].append({"at": stamp(time.time()), "why": "not persisted"})

    def _start_sampler(self):
        if self._sampler is not None:
            return
        self.sample()

        def loop():
            last = time.monotonic()
            while not self._stop.wait(SAMPLE_SECONDS):
                now = time.monotonic()
                if now - last > 3 * SAMPLE_SECONDS:
                    with self._lock:
                        self.entry["samples"]["missed"].append({"at": stamp(time.time()), "why": "interval exceeded"})
                last = now
                self.sample()
        self._sampler = threading.Thread(target=loop, daemon=True)
        self._sampler.start()

    def _stop_sampler(self):
        self._stop.set()
        if self._sampler is not None:
            self._sampler.join(timeout=10)
        if self.entry is not None and self.state == "open":
            self.sample()  # a last sample before the judgment

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

    # -- the end of the run
    def finish(self, retain=False, forgotten=False):
        """Comparison made and persisted first, then the home listed, then the run marked in the
        inventory (finished, or left when a holder may be alive), then the run's record; whatever raises
        on the way, the later steps still run. Nothing is removed. A termination signal received
        meanwhile is re-raised at the end."""
        if self.outcome is not None:
            return self.outcome
        self._finishing = True
        outcome = {"confinement": CONFINEMENT_STATEMENT, "sockets": SOCKETS_STATEMENT, "finished": FINISHED_STATEMENT,
                   "outside": OUTSIDE_STATEMENT, "admissions": self.admissions, "frames": self.frames,
                   "launches": [r["role"] for r in self.roles], "forgotten": forgotten}
        try:
            try:
                self._stop_sampler()
                if self.state != "open":
                    raise LaunchRefused(f"run_{self.state}")
                if self.children:
                    try:
                        time.sleep(SETTLE_SECONDS)
                    except BaseException as error:  # noqa: BLE001  (an interrupt during the wait skips nothing)
                        outcome.setdefault("interruptions", []).append(f"settle wait: {type(error).__name__}")
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
                outcome["throwawayHome"] = self._mark(retain)
            except BaseException as error:  # noqa: BLE001
                outcome["throwawayHome"] = {"state": "left", "home": str(self.home), "removed": False,
                                            "reason": f"marking the run raised ({type(error).__name__})"}
            outcome["samples"] = (self.entry or {}).get("samples")
            try:
                if self.evidence_fd is not None:
                    _write_new(self.evidence_fd, f"{self.label}-run.json", outcome)
            except BaseException:  # noqa: BLE001
                outcome["recordPersisted"] = False
            self._restore_signals()
            self.outcome = outcome
            self.state = "finished"
            self._finishing = False
            for fd in (self.evidence_fd, self.executable_fd):
                if fd is not None:
                    os.close(fd)
            self.evidence_fd = self.executable_fd = None
            if self.kit:
                self.kit.close()
            if self in _OPEN_RUNS:
                _OPEN_RUNS.remove(self)
        if self._pending is not None:
            raise Interrupted(self._pending)
        return outcome

    def _mark(self, retain):
        """The home listed into the evidence folder and the run marked in the inventory: 'finished', or
        'left' when the caller could not establish its processes ended or the holder check finds one
        alive or cannot exclude one. Nothing is removed; 'finished' does not mean safe to remove."""
        if self.entry is None or self.kit is None:
            return {"state": "not created", "removed": False}
        name = self.entry["home"]
        result = {"home": str(self.home), "removed": False, "statement": FINISHED_STATEMENT}
        if self.entry["state"] == "creating":
            return {**result, "state": "creating" if self.kit.exists(name) else "not created",
                    "reason": "the home's record was not completed; it stays in the inventory"}
        try:
            listing = home_listing((self.kit.homes_fd, name))
            _write_new(self.evidence_fd, f"{self.label}-listing.json", {"home": str(self.home), **listing})
            result["listing"] = f"{self.label}-listing.json"
        except BaseException as error:  # noqa: BLE001
            result["listingError"] = type(error).__name__
        found = holders(self.kit, self.entry, self.children)
        result["holders"] = {k: found.get(k) for k in ("holders", "reason", "writers", "coverage")}
        if found.get("unreadableInside"):
            result["unreadableInside"] = found["unreadableInside"]
            self.entry["findings"] = {**self.entry["findings"], "unreadableInside": found["unreadableInside"]}
        state = "left" if retain or found["holders"] in ("alive", "unestablished") else "finished"
        self.entry["state"] = state
        try:
            self.kit.write_entry(self.entry)
        except Exception as error:  # noqa: BLE001
            result["registryError"] = type(error).__name__
        result["state"] = state
        if retain:
            result["reason"] = "the caller could not establish that the processes the run started ended"
        return result

    def __enter__(self):
        return self.begin()

    def __exit__(self, *exc):
        self.finish()
        return False


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    commands = parser.add_subparsers(dest="command", required=True)
    launch = commands.add_parser("check-launch")
    launch.add_argument("--guard-dir")
    commands.add_parser("homes", help="every home of this kit on disk, by name and state, with a count")
    args = parser.parse_args(argv)
    try:
        if args.command == "check-launch":
            print(json.dumps(check_launch(args.guard_dir)))
            return EXIT["clean"]
        kit = Kit(create=False)
        try:
            homes = inventory(kit)
        finally:
            kit.close()
        print(json.dumps(homes))
        return EXIT["clean"]
    except LaunchRefused as refusal:
        print(f"refused: {refusal}", file=sys.stderr)
        return EXIT["refused"]
    except (OSError, ValueError, KeyError) as error:
        print(f"unusable input: {type(error).__name__}: {error}", file=sys.stderr)
        return EXIT["refused"]


if __name__ == "__main__":
    raise SystemExit(main())
