#!/usr/bin/env python3
"""Confine every native launch of the desktop application to a throwaway home, then check it.

Protection is first by confinement, built in: the helper makes the throwaway home under the
system's own per-user temporary folder and starts the application itself, with an environment it
builds from an allowlist (both home variables, TMPDIR inside the home, the strict chooser variable,
locale, a fixed PATH and the kit's own named variables) and a working directory inside the home.
It never copies the caller's environment. Observation of the consequences comes second: the real
Library and the system's per-user cache folder are compared before and after, the throwaway home
is listed, four home folders' times are read by a single stat each, and nothing else on the disk
is watched. It launches on macOS only and refuses on any other system.

Why both home variables: the system web view does not honour HOME alone. With only HOME set it
keeps its website data under the account's real Library, in folders named for the product's bundle
identifier or its executable's name. With CFFIXED_USER_HOME set to the same throwaway home it keeps
it under the throwaway home's Library, and removing the home removes it. See native_launch_guard.md.

Command line (a Node runner calls it with `/usr/bin/python3 -I`):

  native_launch_guard.py check-launch [--guard-dir DIR]
  native_launch_guard.py make-home --evidence DIR [--label home]
  native_launch_guard.py spec --record FILE [--set NAME=VALUE]...
  native_launch_guard.py snapshot --name <bundle id> --name <executable name> --out <before.json>
  native_launch_guard.py compare --before <before.json> --record FILE [--out <report.json>]
  native_launch_guard.py remove-home --record FILE --evidence DIR
      [--started PID]... [--group PGID]... [--private PATH]... [--processes-not-gone]

Exit status, stable: 0 clean; 1 a product entry changed; 2 refused or unusable input; 3 no web view
data in the throwaway home; 4 the home was not removed; 5 the comparison failed; 6 the web view's
helpers wrote their cache folders under the system's per-user cache folder. The application
itself ends with 86 when a chooser is asked with no answer queued (the strict chooser mode).
"""
from __future__ import annotations

# Its directory scans read the account's Library, the system's per-user cache and temporary
# folders and a throwaway home, never repository files (tools/ci/python_test_reads.py).
CI_READS = ()

import argparse
import json
import os
from pathlib import Path
import plistlib
import pwd
import re
import shutil
import stat
import subprocess
import sys
import tempfile
import time

WATCHED_TOPS = ("WebKit", "Caches", "HTTPStorages", "Application Support", "Preferences", "Saved Application State")
WEB_VIEW_TOPS = ("WebKit", "Caches")
HOME_FOLDERS = ("", "Desktop", "Documents", "Downloads")  # times by a single stat each; never listed
NAMED_OTHERS = (("Preferences",), ("Preferences", "ByHost"))
# The system's per-user cache folder, named by what it is and never by its path on a machine. The
# system gives it independently of the home, so neither home variable moves it, and the web view's
# helper processes, which the system starts, keep folders there named for the application.
CACHE_TOP = "<system per-user cache folder>"
HELPER_CACHE = re.compile(r"^com\.apple\.WebKit\.(?:Networking|WebContent(?:\.EnhancedSecurity)?|GPU)\+(.+)$")
# Whether a run whose only product-named changes are the web view helpers' cache folders counts as
# clean. The SWBPIPE Agent 0 decides; until then the strict default stands and such a run stops.
HELPER_CACHE_COUNTS_AS_CLEAN = False
PLATFORM = sys.platform  # launches are made on macOS only
HOME_LISTING_KEPT, HOME_WALK_LIMIT = 2000, 100000
PATH = "/usr/bin:/bin"
LOCALE = {"LANG": "en_US.UTF-8"}
# The strict chooser mode: set on every launch; a chooser asked with nothing queued ends the
# application with STRICT_CHOOSER_EXIT before any system dialog. The Rust side keeps its own copy.
STRICT_CHOOSER_ENV, STRICT_CHOOSER_EXIT = "SWBPIPE_NATIVE_SMOKE_STRICT_CHOOSER", 86
# The kit's own named variables a caller may pass, and nothing else.
KIT_VARIABLES = ("SWBPIPE_NATIVE_SMOKE_DRIVER", "SWBPIPE_NATIVE_SMOKE_PLAN", "SWBPIPE_NATIVE_SMOKE_REPORT",
                 "SWBPIPE_NATIVE_SMOKE_AGENT_GATE", "SWBPIPE_NATIVE_SMOKE_AGENT_GATE_MODE",
                 "SWBPIPE_NATIVE_SMOKE_PROVIDER_KEY")
PROVIDER_KEY_MARKER = "stdin"  # the only value: the key itself comes on stdin, never in the environment
REFUSED_NAMES = ("SWBPIPE_NATIVE_SMOKE_INJECT_RESULT", "SWBPIPE_NATIVE_SMOKE_INJECT_ROOT")
REFUSED_PREFIXES = ("DYLD_", "LD_", "XDG_")
PROXY = re.compile(r"(?i)^(http|https|ftp|all|no)_proxy$")
CREDENTIAL = re.compile(r"(?i)(KEY|TOKEN|SECRET|PASSWORD|PASSWD|CREDENTIAL|AUTH)")
SLOT_LOCK = re.compile(r"^cargo_job(\.slot\d+)?\.lock$")
LOCK_PROGRAM = "/usr/bin/lockf"
PS, LSOF, IOREG, GETCONF = "/bin/ps", "/usr/sbin/lsof", "/usr/sbin/ioreg", "/usr/bin/getconf"
CHECK_TIMEOUT = 60
EXIT = {"clean": 0, "product": 1, "refused": 2, "no web view data": 3, "not removed": 4, "comparison failed": 5,
        "web view helper cache": 6}
WATCH_STATEMENT = ("Nothing outside the real Library's six folders, the system's per-user cache folder and the "
                   "throwaway home was watched; the times of the home, Desktop, Documents and Downloads folders "
                   "were read by a single stat each, and nothing inside them was listed or opened.")
HELPER_CACHE_STATEMENT = ("The web view's helper processes, which the system starts, keep cache folders named for the "
                          "executable under the system's per-user cache folder. The system gives that folder "
                          "independently of the home, and whether both home variables redirect the helpers' folders "
                          "is not established; where they do not, it is a write outside the throwaway home that "
                          "confinement does not reach, observed by the comparison, not prevented. Nothing there is "
                          "opened, edited or removed.")
OTHERS_STATEMENT = ("Records written during the window by the operating system's own services or by other "
                    "software exist under the account's real Library. They are counted per folder, named only "
                    "directly under Preferences, never opened, edited or removed, and they do not fail the run.")
WINDOW_STATEMENT = ("The comparison covers the window named here. Two launches whose windows overlap cannot be "
                    "told apart, so launches through this kit are serialised.")
LIMITS_STATEMENT = ("A clean comparison establishes neither that nothing was read nor that nothing was written "
                    "elsewhere on the disk.")
SIGNS_NOT_USED = {
    "screenSaver": "not used: which process runs while the screen saver is active could not be established by a harmless read",
    "displaySleep": "not used: the display's power value while asleep could not be established by a harmless read",
}
# Held at import, so a caller replacing subprocess.Popen replaces neither the kit's starts nor its
# read-only system commands; kept apart so a test that stands in for the start keeps the reads real.
_POPEN = _READ_POPEN = subprocess.Popen


class LaunchRefused(Exception):
    """A launch, home or removal the kit will not make, named."""


def real_home():
    return Path(pwd.getpwuid(os.getuid()).pw_dir)


def real_library():
    return real_home() / "Library"


def stamp(t):
    return time.strftime("%Y-%m-%dT%H:%M:%S", time.gmtime(t)) + f".{int((t % 1) * 1000):03d}Z"


def _read(argv, timeout=CHECK_TIMEOUT):
    """Run one read-only system command; (status, stdout bytes, stderr text)."""
    with _READ_POPEN(argv, stdin=subprocess.DEVNULL, stdout=subprocess.PIPE, stderr=subprocess.PIPE, cwd="/") as process:
        try:
            out, err = process.communicate(timeout=timeout)
        except subprocess.TimeoutExpired:
            process.kill()
            process.communicate()
            raise
        return process.returncode, out, err.decode("utf-8", "replace")


# ---------------------------------------------------------------------------
# Before any launch: the screen and the slot.

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
    """None when the console session entry for this account is found, is on the console and carries
    no locked key; otherwise the reason. Anything unparsed refuses."""
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
    entry = mine[0]
    if entry.get("kCGSSessionOnConsoleKey") is not True:
        return "screen_not_on_console"
    if "CGSSessionScreenIsLocked" in entry and entry.get("CGSSessionScreenIsLocked") is not False:
        return "screen_locked"
    return None


def _process_table():
    """pid -> (ppid, pgid, uid, executable), read only."""
    status, out, _ = _read([PS, "-axww", "-o", "pid=,ppid=,pgid=,uid=,comm="])
    if status != 0:
        raise OSError(f"ps exit {status}")
    table = {}
    for line in out.decode("utf-8", "replace").splitlines():
        fields = line.split(None, 4)
        if len(fields) == 5:
            table[int(fields[0])] = (int(fields[1]), int(fields[2]), int(fields[3]), fields[4])
    return table


def _open_names(pid):
    status, out, _ = _read([LSOF, "-n", "-P", "-w", "-F", "n", "-p", str(pid)])
    if status not in (0, 1):
        raise OSError(f"lsof exit {status}")
    return [line[1:] for line in out.decode("utf-8", "replace").splitlines() if line.startswith("n")]


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
            ppid, _, uid, exe = table[pid]
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
    """Refuse a launch on any system but macOS, outside the slot wrapper, or without an unlocked
    console session."""
    if PLATFORM != "darwin":
        raise LaunchRefused("platform_not_macos")
    for reason in (screen_refusal(), slot_refusal(guard_dir)):
        if reason:
            raise LaunchRefused(reason)
    return {"screen": "console session for this account, on console, not locked", "slot": "inside the slot wrapper",
            "guardDirGiven": guard_dir is not None, "furtherSigns": SIGNS_NOT_USED}


# ---------------------------------------------------------------------------
# The throwaway home, made by the kit under the system's own temporary folder.

def temp_root():
    """The system's own per-user temporary folder (macOS), else the standard temporary folder."""
    if sys.platform == "darwin":
        status, out, _ = _read([GETCONF, "DARWIN_USER_TEMP_DIR"])
        text = out.decode().strip()
        if status != 0 or not text.startswith("/"):
            raise LaunchRefused("temporary_folder_unknown")
        return Path(text).resolve()
    return Path(tempfile.gettempdir()).resolve()


def user_cache_dir():
    """The system's own per-user cache folder (macOS), or None where the system gives none."""
    if PLATFORM != "darwin":
        return None
    status, out, _ = _read([GETCONF, "DARWIN_USER_CACHE_DIR"])
    text = out.decode().strip()
    return Path(text).resolve() if status == 0 and text.startswith("/") else None


def _identity(path):
    st = os.lstat(path)
    if not stat.S_ISDIR(st.st_mode):
        raise LaunchRefused(f"not_a_real_directory:{path}")
    return [st.st_dev, st.st_ino]


def make_home(evidence, label="home"):
    """Make the throwaway home, with its tmp and work folders, under the system's temporary folder,
    and write its record (paths with device and inode, up to that folder) into `evidence`, outside
    the home. A caller cannot name the home. Returns the record."""
    if not re.fullmatch(r"[a-z0-9][a-z0-9._-]{0,40}", label):
        raise LaunchRefused("home_label")
    root = temp_root()
    home = Path(tempfile.mkdtemp(prefix=f"swbpipe-{label}-", dir=root)).resolve()
    for name in ("tmp", "work"):
        (home / name).mkdir(mode=0o700)
    chain, path = {}, home
    while True:
        chain[str(path)] = _identity(path)
        if path == root or path == path.parent:
            break
        path = path.parent
    for name in ("tmp", "work"):
        chain[str(home / name)] = _identity(home / name)
    record = {"home": str(home), "tmp": str(home / "tmp"), "work": str(home / "work"), "tempRoot": str(root),
              "identity": chain, "pid": os.getpid(), "made": stamp(time.time()), "label": label}
    Path(evidence, f"{label}-record.json").write_text(json.dumps(record, indent=1) + "\n")
    return record


def check_record(record):
    """The record's home, tmp, work and every ancestor up to the temporary folder: the same device
    and inode, real directories, and the temporary folder still the system's own answer."""
    root = temp_root()
    if record.get("tempRoot") != str(root):
        raise LaunchRefused("temporary_folder_changed")
    home = Path(record["home"])
    if home.parent != root or not home.name.startswith("swbpipe-"):
        raise LaunchRefused("home_not_made_by_the_kit")
    if str(home) not in record["identity"] or str(root) not in record["identity"]:
        raise LaunchRefused("home_not_made_by_the_kit")
    for path, identity in record["identity"].items():
        if not os.path.lexists(path) or _identity(path) != identity:
            raise LaunchRefused(f"identity_changed:{path}")
    return home


def check_environment(env, record):
    """Refuse, by name, an environment that is not exactly the kit's for this home."""
    home = record["home"]
    for name in ("HOME", "CFFIXED_USER_HOME"):
        if not env.get(name):
            raise LaunchRefused(f"{name.lower()}_missing")
    if env["HOME"] != env["CFFIXED_USER_HOME"]:
        raise LaunchRefused("homes_differ")
    if not Path(env["HOME"]).is_absolute():
        raise LaunchRefused("home_not_absolute")
    resolved, real = Path(env["HOME"]).resolve(), real_home().resolve()
    if resolved == real or real.is_relative_to(resolved):
        raise LaunchRefused("home_is_the_real_home")
    if str(resolved) != home:
        raise LaunchRefused("home_not_the_kits")
    if env.get("TMPDIR") != record["tmp"] or env.get(STRICT_CHOOSER_ENV) != "1" or env.get("PATH") != PATH:
        raise LaunchRefused("environment_not_the_kits")
    return resolved


def allowed(name, value):
    """Refuse, by name, a variable a caller may not pass."""
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
    if name == "SWBPIPE_NATIVE_SMOKE_PROVIDER_KEY" and value != PROVIDER_KEY_MARKER:
        raise LaunchRefused("provider_key_marker_only")
    if not isinstance(value, str):
        raise LaunchRefused(f"variable_not_text:{name}")


def spec(record, kit=None):
    """The launch specification: the whole environment, built from the allowlist and never from the
    caller's own, and the working directory inside the home. Checked before it is handed back."""
    check_record(record)
    env = {}
    for name, value in (kit or {}).items():
        allowed(name, value)
        env[name] = value
    env.update(LOCALE)
    env.update({"PATH": PATH, "TMPDIR": record["tmp"], STRICT_CHOOSER_ENV: "1",
                "HOME": record["home"], "CFFIXED_USER_HOME": record["home"]})
    check_environment(env, record)
    return {"env": env, "cwd": record["work"]}


def start(argv, record, kit=None, guard_dir=None, **popen):
    """Start the application with the kit's specification and return the handle: the only way the
    kit's Python callers start it. The environment and working directory are not theirs to give."""
    if "env" in popen or "cwd" in popen:
        raise LaunchRefused("env_and_cwd_are_the_kits")
    check_launch(guard_dir)
    launch = spec(record, kit)
    return _POPEN(argv, env=launch["env"], cwd=launch["cwd"], **popen)


# ---------------------------------------------------------------------------
# Observation: names, sizes and times only.

def facts(path):
    st = os.lstat(path)
    kind = ("dir" if stat.S_ISDIR(st.st_mode) else "link" if stat.S_ISLNK(st.st_mode)
            else "file" if stat.S_ISREG(st.st_mode) else "other")
    return {"kind": kind, "size": st.st_size, "mtime": st.st_mtime, "birth": getattr(st, "st_birthtime", st.st_ctime)}


def walk(root, unreadable, limit=None):
    """(path, facts) for everything under `root`, root excluded; links are listed, not followed."""
    stack, seen = [root], 0
    while stack:
        directory = stack.pop()
        try:
            with os.scandir(directory) as entries:
                paths = sorted(entry.path for entry in entries)
        except OSError:
            unreadable.append(str(directory))
            continue
        for path in paths:
            if limit is not None and seen >= limit:
                return
            try:
                found = facts(path)
            except OSError:
                unreadable.append(path)
                continue
            seen += 1
            yield path, found
            if found["kind"] == "dir":
                stack.append(path)


def _named(text, name):
    return text == name or text.startswith(name + ".")


def _forms(names):
    """The watched names and each in its underscore form, the form the web view's helpers use for the
    executable's name."""
    return sorted(set(names) | {name.replace("-", "_") for name in names})


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


def list_library(names, library, since=None, until=None, cache=None):
    """One walk of the six Library folders and the system's per-user cache folder: every watched
    entry; given a window, other entries created or modified in it (counted, named only under
    Preferences); and what could not be listed. A cache folder the system does not give fails closed."""
    entries, failed, relatives = {}, [], []
    others = {"total": 0, "byFolder": {}, "named": []}
    roots = [(top, library / top, lambda p: os.path.relpath(p, library)) for top in WATCHED_TOPS]
    if cache is None:
        failed.append(CACHE_TOP)
    else:
        roots.append((CACHE_TOP, Path(cache), lambda p, c=Path(cache): str(Path(CACHE_TOP, os.path.relpath(p, c)))))
    for top, base, relative_to in roots:
        if top != CACHE_TOP and not os.path.lexists(base):
            continue
        try:
            with os.scandir(base):
                pass
        except OSError:
            failed.append(top)
            continue
        unreadable = []
        for path, found in walk(base, unreadable):
            relative = relative_to(path)
            if watched(relative, names):
                entries[relative] = found
            elif since is not None and (since <= found["mtime"] <= until or since <= found["birth"] <= until):
                others["total"] += 1
                others["byFolder"][top] = others["byFolder"].get(top, 0) + 1
                if Path(relative).parts[:-1] in NAMED_OTHERS:
                    others["named"].append({"path": relative, "size": found["size"], "mtime": stamp(found["mtime"])})
        relatives += [relative_to(p) for p in unreadable]
    return {"entries": entries, "others": others, "failedTops": failed,
            "unreadableWatched": sorted(r for r in relatives if watched(r, names)),
            "unreadableOther": sorted(r for r in relatives if not watched(r, names))}


def snapshot(names, library=None, home=None, cache=None):
    """Before the first launch: watched entries only, what could not be listed, and the four times."""
    names = sorted(set(names))
    if not names or any(not isinstance(n, str) or not n or "/" in n for n in names):
        raise ValueError("names to watch are required")
    library = Path(library) if library is not None else real_library()
    cache = Path(cache) if cache is not None else user_cache_dir()
    taken = time.time()
    listing = list_library(names, library, cache=cache)
    return {"taken": taken, "names": names, "entries": listing["entries"], "failedTops": listing["failedTops"],
            "unreadableWatched": listing["unreadableWatched"], "unreadableOther": listing["unreadableOther"],
            "folderTimes": folder_times(home)}


def home_listing(home):
    """The throwaway home's own files by lstat (bounded, sorted by path), and whether the web view's
    data is under its Library/WebKit or Library/Caches. A failure is recorded, not raised."""
    try:
        rows, unreadable, complete = [], [], True
        for path, found in walk(Path(home), unreadable, HOME_WALK_LIMIT + 1):
            if len(rows) >= HOME_WALK_LIMIT:
                complete = False
                break
            rows.append({"path": os.path.relpath(path, home), **found})
        rows.sort(key=lambda r: r["path"])
        web_view = any(r["kind"] == "file" and Path(r["path"]).parts[:1] == ("Library",) and len(Path(r["path"]).parts) > 2
                       and Path(r["path"]).parts[1] in WEB_VIEW_TOPS for r in rows)
        return {"by": "lstat", "total": len(rows), "totalComplete": complete, "truncated": not complete or len(rows) > HOME_LISTING_KEPT,
                "entries": rows[:HOME_LISTING_KEPT], "webViewData": web_view,
                "unreadable": [os.path.relpath(p, home) for p in unreadable][:100]}
    except Exception as error:  # noqa: BLE001
        return {"by": "lstat", "error": f"{type(error).__name__}: {error}"[:1000], "webViewData": False}


def compare(before, record, until=None, library=None, home=None, cache=None, helper_cache_clean=None):
    """After the run. `verdict`: "product", "comparison failed", "web view helper cache", "no web view
    data" or "clean"; `class`: "product", "web view helper cache", "others" or "nothing". A change
    inside a web view helper's cache folder under the system's per-user cache folder is its own
    class; any other product-named change there is a product change like one under the Library."""
    library = Path(library) if library is not None else real_library()
    cache = Path(cache) if cache is not None else user_cache_dir()
    clean_helpers = HELPER_CACHE_COUNTS_AS_CLEAN if helper_cache_clean is None else helper_cache_clean
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
    blind = sorted(set(after["unreadableWatched"]) - set(before["unreadableWatched"]))
    failed = sorted(set(before["failedTops"]) | set(after["failedTops"]))
    newly_unreadable = sorted(set(after["unreadableOther"]) - set(before["unreadableOther"]))
    times_after = folder_times(home)
    positive = home_listing(record["home"])
    product = bool(changes or blind)
    stop_helpers = bool(helpers) and not clean_helpers
    verdict = ("product" if product else "comparison failed" if failed or newly_unreadable
               else "web view helper cache" if stop_helpers
               else "clean" if positive["webViewData"] else "no web view data")
    return {"compared": True, "verdict": verdict,
            "class": ("product" if product else "web view helper cache" if helpers
                      else "others" if after["others"]["total"] else "nothing"),
            "names": names, "window": {"since": stamp(since), "until": stamp(until), "statement": WINDOW_STATEMENT},
            "product": {"changes": changes, "unreadableWatched": blind},
            "helperCache": {"statement": HELPER_CACHE_STATEMENT, "changes": helpers, "countsAsClean": clean_helpers},
            "failure": {"foldersNotListed": failed, "newlyUnreadableCount": len(newly_unreadable)},
            "limit": {"unreadableBeforeAndAfterCount": len(set(before["unreadableOther"]) & set(after["unreadableOther"])),
                      "statement": "a product-named entry inside a folder that could not be listed before and after would not be seen"},
            "others": {"statement": OTHERS_STATEMENT, **after["others"]},
            "folderTimes": {"before": before["folderTimes"], "after": times_after,
                            "changed": sorted(k for k in times_after if times_after[k] != before["folderTimes"].get(k)),
                            "statement": "reported with the run, not concluded from"},
            "throwawayHome": positive, "watched": WATCH_STATEMENT, "limits": LIMITS_STATEMENT}


# ---------------------------------------------------------------------------
# Possible writers, and the end of every run.

def _under(path, roots):
    return any(path == str(r) or path.startswith(str(r).rstrip("/") + "/") for r in roots)


def _open_under(home):
    """pids of this account's processes with an open file or working directory under `home`."""
    status, out, err = _read([LSOF, "-n", "-P", "-w", "-F", "p", "+D", str(home)])
    if status not in (0, 1) or (status == 1 and err.strip()):
        raise OSError(f"lsof exit {status}")
    return {int(line[1:]) for line in out.decode("utf-8", "replace").splitlines() if line.startswith("p")}


def possible_writers(home, started=(), groups=(), private=()):
    """Every process of this account that could still write to `home`, with why: its executable is
    under the home or a private copy; it descends from a process the run started; it is in a group
    the run started; it has a file or its working directory open under the home (the web view's
    helpers, which the system starts outside the application's group and environment). macOS only:
    another process's environment cannot be read there, so no test rests on it. Raises when it
    cannot run, on any other system included."""
    home = str(Path(home).resolve())
    roots = [home, *(str(Path(p).resolve()) for p in private)]
    me, mine = os.getpid(), os.getuid()
    if PLATFORM != "darwin":
        raise OSError("the check for possible writers is made on macOS only")
    found = {}
    table = _process_table()
    for pid in _open_under(home):
        found.setdefault(pid, []).append("open under the home")
    children = {}
    for pid, (ppid, _, _, _) in table.items():
        children.setdefault(ppid, []).append(pid)
    lineage, stack = set(), [p for p in started if p in table and p > 1]
    while stack:
        pid = stack.pop()
        if pid not in lineage:
            lineage.add(pid)
            stack.extend(children.get(pid, []))
    for pid, (ppid, pgid, uid, exe) in table.items():
        if uid != mine:
            continue
        if exe and _under(exe, roots):
            found.setdefault(pid, []).append("executable under the home or the private copy")
        if pid in lineage:
            found.setdefault(pid, []).append("started by the run, or a descendant")
        if pgid in set(groups):
            found.setdefault(pid, []).append("in a group the run started")
    found.pop(me, None)
    return [{"pid": pid, "executable": table.get(pid, (0, 0, 0, ""))[3], "why": sorted(set(why))} for pid, why in sorted(found.items())]


def remove_home(record, evidence, processes_gone=True, started=(), groups=(), private=()):
    """End of every run, whatever its outcome. The home's listing goes into `evidence`, outside the
    home, first (a failed listing is recorded and the home still removed). The home is removed only
    after its record is rechecked and nothing that could still write to it is found alive; links
    inside are unlinked, not followed; the path is verified gone. Otherwise it is withheld, listed
    and named. `cleanup`: removed, withheld, failed, refused, not created."""
    home = Path(record["home"])
    label = record.get("label", "home")
    if not os.path.lexists(home):
        return {"cleanup": "not created", "home": str(home), "removed": False}
    try:
        check_record(record)
    except (LaunchRefused, OSError, KeyError) as refusal:
        return {"cleanup": "refused", "home": str(home), "removed": False, "reason": str(refusal)}
    listing = home_listing(home)
    try:
        listing_path = Path(evidence, f"{label}-listing.json")
        listing_path.write_text(json.dumps({"home": str(home), "listedAt": stamp(time.time()), **listing}, indent=1) + "\n")
        written = str(listing_path)
    except OSError as error:
        written = None
        listing = {**listing, "error": f"listing not written: {type(error).__name__}"}
    outcome = {"home": str(home), "listing": written, "listingError": listing.get("error")}
    if not processes_gone:
        return {**outcome, "cleanup": "withheld", "removed": False,
                "reason": "the caller could not verify the processes the run started are gone; the home is left at this path"}
    try:
        writers = possible_writers(home, started, groups, private)
    except Exception as error:  # noqa: BLE001  (any failure of the check withholds; none removes)
        return {**outcome, "cleanup": "withheld", "removed": False,
                "reason": f"the check for processes that could still write here could not run ({type(error).__name__}); the home is left at this path"}
    if writers:
        return {**outcome, "cleanup": "withheld", "removed": False, "writers": writers,
                "reason": "processes that could still write here are alive; the home is left at this path"}
    try:
        check_record(record)  # immediately before removal
        shutil.rmtree(home)  # unlinks links inside the tree; never follows them
    except (LaunchRefused, OSError) as error:
        return {**outcome, "cleanup": "failed", "removed": False, "reason": f"{type(error).__name__}: {error}"[:500]}
    if os.path.lexists(home):
        return {**outcome, "cleanup": "failed", "removed": False, "reason": "the path is still present after removal"}
    return {**outcome, "cleanup": "removed", "removed": True}


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    commands = parser.add_subparsers(dest="command", required=True)
    launch = commands.add_parser("check-launch")
    launch.add_argument("--guard-dir")
    make = commands.add_parser("make-home")
    make.add_argument("--evidence", required=True)
    make.add_argument("--label", default="home")
    specify = commands.add_parser("spec")
    specify.add_argument("--record", type=Path, required=True)
    specify.add_argument("--set", action="append", default=[])
    snap = commands.add_parser("snapshot")
    snap.add_argument("--name", action="append", required=True)
    snap.add_argument("--out", type=Path, required=True)
    comp = commands.add_parser("compare")
    comp.add_argument("--before", type=Path, required=True)
    comp.add_argument("--record", type=Path, required=True)
    comp.add_argument("--out", type=Path)
    remove = commands.add_parser("remove-home")
    remove.add_argument("--record", type=Path, required=True)
    remove.add_argument("--evidence", required=True)
    remove.add_argument("--processes-not-gone", action="store_true")
    remove.add_argument("--started", type=int, action="append", default=[])
    remove.add_argument("--group", type=int, action="append", default=[])
    remove.add_argument("--private", action="append", default=[])
    args = parser.parse_args(argv)
    try:
        if args.command == "check-launch":
            print(json.dumps(check_launch(args.guard_dir)))
            return EXIT["clean"]
        if args.command == "make-home":
            print(json.dumps(make_home(args.evidence, args.label)))
            return EXIT["clean"]
        if args.command == "spec":
            pairs = dict(item.split("=", 1) for item in args.set)
            print(json.dumps(spec(json.loads(args.record.read_text()), pairs)))
            return EXIT["clean"]
        if args.command == "snapshot":
            args.out.write_text(json.dumps(snapshot(args.name)) + "\n")
            return EXIT["clean"]
        if args.command == "remove-home":
            outcome = remove_home(json.loads(args.record.read_text()), args.evidence, not args.processes_not_gone,
                                  args.started, args.group, args.private)
            print(json.dumps(outcome))
            return {"removed": 0, "not created": 0, "refused": 2}.get(outcome["cleanup"], EXIT["not removed"])
        report = compare(json.loads(args.before.read_text()), json.loads(args.record.read_text()))
    except LaunchRefused as refusal:
        print(f"refused: {refusal}", file=sys.stderr)
        return EXIT["refused"]
    except (OSError, ValueError, KeyError) as error:
        print(f"unusable input: {type(error).__name__}: {error}", file=sys.stderr)
        return EXIT["refused"]
    if args.out:
        args.out.write_text(json.dumps(report, indent=1) + "\n")
    print(json.dumps({"verdict": report["verdict"], "class": report["class"], "window": report["window"],
                      "productChanges": report["product"]["changes"], "othersTotal": report["others"]["total"]}))
    return EXIT[report["verdict"]]


if __name__ == "__main__":
    raise SystemExit(main())
