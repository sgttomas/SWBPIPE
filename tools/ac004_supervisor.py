#!/usr/bin/env python3
"""AC-004 exclusive-lane deadline supervisor (DEL-04-08 qualification tooling).

Takes the T3 host's four lock slots in the exclusive-job order, runs one payload
in its own process group under a monotonic deadline that is armed no later than
the final lock's acquisition, contains that group, and releases the locks within
the budget.  See ac004_supervisor.md beside this file for the protocol, the
bounded region and the limits.  Standard library only; macOS and Linux.
"""
import ctypes
import errno
import fcntl
import hashlib
import json
import math
import os
import select
import shutil
import signal
import stat
import struct
import subprocess
import sys
import time

CEILING_S = 60.0           # production ceiling, teardown and release included
DEFAULT_BUDGET_S = 60.0
DEFAULT_RESERVE_S = 3.0    # payload is stopped at budget - reserve
DEFAULT_TERM_GRACE_S = 1.0  # SIGTERM -> SIGKILL, inside the reserve
LOCK_NAMES = ("cargo_job.lock", "cargo_job.slot2.lock", "cargo_job.slot3.lock", "cargo_job.slot4.lock")
PENDING_NAME = "exclusive.pending"
MEMGUARD_NAME = "memguard.sh"
FINAL_POLL_S = 0.05
TICK_S = 0.01

EX_OK, EX_USAGE, EX_PENDING, EX_CONFIG, EX_MEMGUARD = 0, 64, 75, 78, 97
EX_STRAYS, EX_TIMEOUT, EX_SUPERVISOR, EX_SPAWN = 123, 124, 125, 126


class Refusal(Exception):
    def __init__(self, code, reason):
        super().__init__(reason)
        self.code, self.reason = code, reason


class Cancelled(BaseException):
    def __init__(self, signum):
        super().__init__(signum)
        self.signum = signum


# --- signals: raise only around untimed blocking waits; otherwise just record ---
class _Sig:
    signum = 0
    raise_ok = False


def _on_signal(signum, _frame):
    if not _Sig.signum:
        _Sig.signum = signum
    if _Sig.raise_ok:
        _Sig.raise_ok = False
        raise Cancelled(signum)


def _interruptible(fn, *args):
    _Sig.raise_ok = True
    try:
        if _Sig.signum:
            raise Cancelled(_Sig.signum)
        return fn(*args)
    finally:
        _Sig.raise_ok = False


# --- process-group membership from the kernel (no ps/pgrep matching) ---
_KINFO_SIZE, _P_STAT, _P_PID, _SZOMB = 648, 36, 40, 5  # Darwin struct kinfo_proc / extern_proc


_LIBC = []


def _members_darwin(pgid):
    if not _LIBC:
        lib = ctypes.CDLL(None, use_errno=True)
        lib.sysctl.argtypes = [ctypes.POINTER(ctypes.c_int), ctypes.c_uint, ctypes.c_void_p,
                               ctypes.POINTER(ctypes.c_size_t), ctypes.c_void_p, ctypes.c_size_t]
        _LIBC.append(lib)
    libc = _LIBC[0]
    mib = (ctypes.c_int * 4)(1, 14, 2, pgid)  # CTL_KERN, KERN_PROC, KERN_PROC_PGRP
    for _ in range(8):
        size = ctypes.c_size_t(0)
        if libc.sysctl(mib, 4, None, ctypes.byref(size), None, 0) != 0:
            raise OSError(ctypes.get_errno(), "sysctl kern.proc.pgrp size")
        cap = size.value + 16 * _KINFO_SIZE
        buf = ctypes.create_string_buffer(cap)
        size = ctypes.c_size_t(cap)
        if libc.sysctl(mib, 4, buf, ctypes.byref(size), None, 0) == 0:
            break
        if ctypes.get_errno() != errno.ENOMEM:
            raise OSError(ctypes.get_errno(), "sysctl kern.proc.pgrp")
    else:
        raise OSError(errno.ENOMEM, "sysctl kern.proc.pgrp kept growing")
    raw = buf.raw[:size.value]
    if len(raw) % _KINFO_SIZE:
        raise OSError(errno.EPROTO, "unexpected kinfo_proc size")
    return [(struct.unpack_from("i", raw, o + _P_PID)[0], raw[o + _P_STAT] == _SZOMB)
            for o in range(0, len(raw), _KINFO_SIZE)]


def _members_linux(pgid):
    out = []
    for name in os.listdir("/proc"):
        if not name.isdigit():
            continue
        try:
            with open(f"/proc/{name}/stat", "rb") as f:
                data = f.read()
        except OSError:
            continue
        rest = data[data.rindex(b")") + 2:].split()
        if int(rest[2]) == pgid:
            out.append((int(name), rest[0] in (b"Z", b"X")))
    return out


def members(pgid):
    """[(pid, is_zombie)] for every process whose process group is pgid."""
    if sys.platform == "darwin":
        return _members_darwin(pgid)
    if sys.platform.startswith("linux"):
        return _members_linux(pgid)
    raise OSError(errno.ENOSYS, "unsupported platform")


def _live(pgid, exclude=()):
    return [p for p, zombie in members(pgid) if not zombie and p not in exclude]


# --- inputs ---
def _seconds(name, text):
    try:
        value = float(text)
    except (TypeError, ValueError):
        raise Refusal(EX_USAGE, f"{name} is not a number")
    if not math.isfinite(value) or value <= 0:
        raise Refusal(EX_USAGE, f"{name} must be finite and positive")
    return value


def parse_args(argv):
    if "--" not in argv:
        raise Refusal(EX_USAGE, "usage: ac004_supervisor.py --guard-dir DIR [--budget S] [--reserve S] "
                                "[--term-grace S] [--trace-file PATH] -- PAYLOAD...")
    split = argv.index("--")
    opts, payload = argv[:split], argv[split + 1:]
    known = {"--guard-dir": None, "--budget": str(DEFAULT_BUDGET_S), "--reserve": str(DEFAULT_RESERVE_S),
             "--term-grace": str(DEFAULT_TERM_GRACE_S), "--trace-file": None}
    seen = set()
    if len(opts) % 2:
        raise Refusal(EX_USAGE, "every option takes one value")
    for key, value in zip(opts[::2], opts[1::2]):
        if key not in known or key in seen:
            raise Refusal(EX_USAGE, f"unknown or repeated option {key}")
        seen.add(key)
        known[key] = value
    if not known["--guard-dir"]:
        raise Refusal(EX_USAGE, "--guard-dir is required")
    if not payload:
        raise Refusal(EX_USAGE, "no payload command")
    budget = _seconds("--budget", known["--budget"])
    reserve = _seconds("--reserve", known["--reserve"])
    grace = _seconds("--term-grace", known["--term-grace"])
    if budget > CEILING_S:
        raise Refusal(EX_USAGE, f"--budget exceeds the {CEILING_S:g} s ceiling")
    if not grace < reserve < budget:
        raise Refusal(EX_USAGE, "need term-grace < reserve < budget")
    return {"guard": os.path.abspath(known["--guard-dir"]), "budget": budget, "reserve": reserve,
            "grace": grace, "trace_file": known["--trace-file"], "payload": payload}


def preflight(opts):
    guard = opts["guard"]
    if not os.path.isdir(guard):
        raise Refusal(EX_CONFIG, "guard directory does not exist")
    for name in LOCK_NAMES:
        try:
            st = os.lstat(os.path.join(guard, name))
        except FileNotFoundError:
            raise Refusal(EX_CONFIG, f"lock file {name} is missing")
        if not stat.S_ISREG(st.st_mode):
            raise Refusal(EX_CONFIG, f"lock file {name} is not a regular file")
    if os.path.lexists(os.path.join(guard, PENDING_NAME)):
        raise Refusal(EX_PENDING, "an exclusive.pending marker already exists; not touching it")
    trace = opts["trace_file"]
    if trace and (os.path.lexists(trace) or not os.path.isdir(os.path.dirname(os.path.abspath(trace)))):
        raise Refusal(EX_USAGE, "--trace-file exists or its directory does not")
    if shutil.which(opts["payload"][0]) is None:
        raise Refusal(EX_USAGE, "payload command is not an executable")
    try:
        if (os.getpid(), False) not in members(os.getpgrp()):
            raise OSError(errno.EPROTO, "own process not found in own group")
    except OSError as e:
        raise Refusal(EX_CONFIG, f"process-group inspection unavailable: {e}")
    try:
        found = subprocess.run(["pgrep", "-f", os.path.join(guard, MEMGUARD_NAME)],
                               stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, timeout=10).returncode == 0
    except (OSError, subprocess.TimeoutExpired):
        found = False
    if not found:
        raise Refusal(EX_MEMGUARD, "memory guard not running; refusing to start")


# --- the sentry: group leader that reports the payload and guards controller death ---
def _sentry_main(payload, life_r, status_w):
    os.setpgid(0, 0)  # the parent also does this; whichever runs first creates the group

    def say(text):
        try:
            os.write(status_w, text.encode())
        except OSError:
            pass

    for s in (signal.SIGINT, signal.SIGTERM, signal.SIGHUP):
        signal.signal(s, lambda *_: None)  # caught (not ignored), so exec restores the default
    try:
        child = subprocess.Popen(payload, stdin=subprocess.DEVNULL, close_fds=True)
    except OSError as e:
        say(f"E {e.errno or 0}\n")
        os._exit(0)
    say(f"S {child.pid}\n")
    reported = False
    while True:
        if not reported:
            pid, status = os.waitpid(child.pid, os.WNOHANG)
            if pid:
                say(f"X {status}\n")
                reported = True
        if select.select([life_r], [], [], TICK_S)[0] and not os.read(life_r, 64):
            break  # controller gone: its lifeline closed
    # The controller died without containing us.  The inherited lock descriptors are still
    # open here, so the slots stay held until this one kill(2) fatally signals the group.
    os.killpg(os.getpgrp(), signal.SIGKILL)
    os._exit(1)


class Supervisor:
    def __init__(self, opts):
        self.o = opts
        self.guard = opts["guard"]
        self.pending = os.path.join(self.guard, PENDING_NAME) if self.guard else None
        self.fds = []
        self.marks = {}
        self.armed = None
        self.sentry = None      # pid == pgid
        self.life_w = self.status_r = None
        self.status = {}
        self.buf = b""
        self.reaped = False
        self.contain = {"verified": False, "kill_rounds": 0, "overrun": False, "error": None}
        self.reason = None      # timeout | cancelled
        self.strays = None

    def mark(self, key):
        self.marks[key] = time.monotonic()

    # pending marker: create only while holding slot 1; remove only our own
    def publish_pending(self):
        if os.path.lexists(self.pending):
            raise Refusal(EX_PENDING, "an exclusive.pending marker already exists; not touching it")
        fd = os.open(self.pending, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o644)
        try:
            os.write(fd, f"{os.getpid()}\n".encode())
        finally:
            os.close(fd)
        self.mark("pending_published")

    def remove_own_pending(self):
        try:
            with open(self.pending, "rb") as f:
                ours = f.read() == f"{os.getpid()}\n".encode()
        except FileNotFoundError:
            return
        if ours:
            os.unlink(self.pending)
            self.mark("pending_removed")

    def open_lock(self, name):
        fd = os.open(os.path.join(self.guard, name), os.O_RDONLY | os.O_NOFOLLOW | os.O_CLOEXEC)
        self.fds.append(fd)
        return fd

    def acquire(self):
        for name in LOCK_NAMES[:3]:
            _interruptible(fcntl.flock, self.open_lock(name), fcntl.LOCK_EX)  # untimed queue
            self.mark(f"{name}_acquired")
            if name == LOCK_NAMES[0]:
                self.publish_pending()
        fd = self.open_lock(LOCK_NAMES[3])
        while True:
            attempt = time.monotonic()
            try:
                fcntl.flock(fd, fcntl.LOCK_EX | fcntl.LOCK_NB)
                break
            except BlockingIOError:
                _interruptible(time.sleep, FINAL_POLL_S)
        self.armed = self.marks["armed"] = attempt  # taken before the acquisition that succeeded
        self.mark(f"{LOCK_NAMES[3]}_acquired")

    def verify_inodes(self):
        for name, fd in zip(LOCK_NAMES, self.fds):
            a, b = os.fstat(fd), os.stat(os.path.join(self.guard, name))
            if (a.st_dev, a.st_ino) != (b.st_dev, b.st_ino):
                raise Refusal(EX_CONFIG, f"lock file {name} was replaced while queued")

    def spawn(self):
        life_r, self.life_w = os.pipe()
        self.status_r, status_w = os.pipe()
        pid = os.fork()
        if pid == 0:
            try:
                os.close(self.life_w)
                os.close(self.status_r)
                _sentry_main(self.o["payload"], life_r, status_w)
            finally:
                os._exit(70)
        self.sentry = pid
        try:
            os.setpgid(pid, pid)  # the group exists before we ever signal or inspect it
        except OSError as e:
            if e.errno not in (errno.EACCES, errno.ESRCH):  # already done by the child / child gone
                raise
        os.close(life_r)
        os.close(status_w)
        os.set_blocking(self.status_r, False)
        self.mark("sentry_forked")

    def read_status(self, timeout):
        if "eof" in self.status:
            time.sleep(timeout)
            return
        if select.select([self.status_r], [], [], timeout)[0]:
            try:
                chunk = os.read(self.status_r, 4096)
            except BlockingIOError:
                return
            if not chunk:
                self.status["eof"] = True
            self.buf += chunk
            *lines, self.buf = self.buf.split(b"\n")
            for line in lines:
                kind, _, value = line.decode().partition(" ")
                if kind == "S":
                    self.status["pid"] = int(value)
                    self.mark("payload_spawned")
                elif kind == "X":
                    self.status["wait"] = int(value)
                    self.mark("payload_ended")
                elif kind == "E":
                    self.status["spawn_errno"] = int(value)

    def killpg(self, sig):
        try:
            os.killpg(self.sentry, sig)  # safe: the sentry (pid == pgid) is not reaped yet
        except (ProcessLookupError, PermissionError):
            pass  # Darwin gives EPERM for a group of zombies; membership is checked separately

    def supervise(self):
        deadline = self.armed + self.o["budget"] - self.o["reserve"]
        while True:
            self.read_status(TICK_S)
            done = "wait" in self.status or "spawn_errno" in self.status or "eof" in self.status
            if done:
                self.strays = len(_live(self.sentry, exclude=(self.sentry,)))
                return
            if _Sig.signum or time.monotonic() >= deadline:
                self.reason = "cancelled" if _Sig.signum else "timeout"
                break
        self.killpg(signal.SIGTERM)
        self.killpg(signal.SIGCONT)  # a stopped member must see its TERM
        self.mark("term_sent")
        grace_end = time.monotonic() + self.o["grace"]
        while time.monotonic() < grace_end:
            self.read_status(TICK_S)
            if ("wait" in self.status or "eof" in self.status) and not _live(self.sentry, exclude=(self.sentry,)):
                break

    def contain_group(self):
        """SIGKILL the owned group until no live member remains, then reap the sentry."""
        if self.sentry is None or self.reaped:
            return
        hard = self.armed + self.o["budget"]
        try:
            self.mark("kill_sent")
            while True:
                self.killpg(signal.SIGKILL)
                self.contain["kill_rounds"] += 1
                time.sleep(0.002)
                if not _live(self.sentry):
                    break
                if time.monotonic() > hard:
                    self.contain["overrun"] = True  # keep the locks while any owned process lives
            self.contain["verified"] = True   # sentry dead and no live member: nothing can fork into the group
            self.mark("contained")
        except OSError as e:
            self.contain["error"] = f"{type(e).__name__}: {e}"
            for _ in range(3):
                self.killpg(signal.SIGKILL)
                time.sleep(0.01)
        while "eof" not in self.status:
            self.read_status(0)
            if not select.select([self.status_r], [], [], 0)[0]:
                break
        os.waitpid(self.sentry, 0)
        self.reaped = True
        self.mark("sentry_reaped")
        for fd in (self.life_w, self.status_r):
            os.close(fd)

    def release(self):
        while self.fds:
            os.close(self.fds.pop())
        self.mark("released")


def _report(sup, opts, code, outcome, extra, queue_start, queue_utc):
    rel = lambda t: None if t is None else round(t - sup.armed, 6) if sup.armed is not None else None
    queue_marks = {k: round(v - queue_start, 6) for k, v in sup.marks.items()}
    status = sup.status
    payload = {"argc": len(opts["payload"]) if opts else None,
               "argv_sha256": hashlib.sha256("\0".join(opts["payload"]).encode()).hexdigest() if opts else None,
               "pid": status.get("pid"), "pgid": sup.sentry, "strays_at_exit": sup.strays}
    if "wait" in status:
        rc = os.waitstatus_to_exitcode(status["wait"])
        payload.update({"status": "exited", "exit_code": rc} if rc >= 0 else {"status": "signaled", "signal": -rc})
    elif "spawn_errno" in status:
        payload.update({"status": "spawn_failed", "errno": status["spawn_errno"]})
    else:
        payload["status"] = "not_started" if sup.sentry is None else "unknown"
    armed_to_release = rel(sup.marks.get("released"))
    return {
        "tool": "ac004_supervisor", "outcome": outcome, "exit_code": code, "controller_pid": os.getpid(),
        "guard_dir": opts["guard"] if opts else None,
        "budget_s": opts and opts["budget"], "reserve_s": opts and opts["reserve"],
        "term_grace_s": opts and opts["grace"], "ceiling_s": CEILING_S,
        "queue_started_utc": queue_utc, "marks_from_queue_start_s": queue_marks,
        "queue_wait_s": None if sup.armed is None else round(sup.armed - queue_start, 6),
        "marks_from_armed_s": {k: rel(v) for k, v in sup.marks.items()} if sup.armed is not None else {},
        "armed_to_release_s": armed_to_release,
        "armed_monotonic": sup.armed, "released_monotonic": sup.marks.get("released"),
        "release_within_budget": None if armed_to_release is None else armed_to_release <= opts["budget"],
        "payload": payload, "stop_reason": sup.reason, "containment": sup.contain,
        "cancel_signal": _Sig.signum or None, **extra,
    }


def main(argv):
    queue_start, queue_utc = time.monotonic(), time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())
    opts, sup, extra = None, None, {}
    for s in (signal.SIGINT, signal.SIGTERM, signal.SIGHUP):
        signal.signal(s, _on_signal)
    signal.signal(signal.SIGCHLD, signal.SIG_DFL)  # never auto-reap: the sentry must stay unreaped
    try:
        opts = parse_args(argv)
        preflight(opts)
    except Refusal as r:
        sup = Supervisor(opts or {"guard": None, "payload": []})
        trace = _report(sup, opts, r.code, "refused", {"refusal": r.reason}, queue_start, queue_utc)
        print(json.dumps(trace, sort_keys=True), file=sys.stderr)
        return r.code
    sup = Supervisor(opts)
    code, outcome = None, None
    try:
        sup.acquire()
        # ---- bounded region: armed at the final lock attempt; signals are only recorded ----
        if not _Sig.signum:
            sup.verify_inodes()
            sup.remove_own_pending()
            sup.spawn()
            sup.supervise()
        sup.contain_group()
    except Refusal as r:
        code, outcome, extra["refusal"] = r.code, "refused", r.reason
    except Cancelled:
        pass
    except Exception as e:  # noqa: BLE001 - any failure must still contain and release
        code, outcome, extra["error"] = EX_SUPERVISOR, "internal_error", f"{type(e).__name__}: {e}"
    finally:
        _Sig.raise_ok = False
        try:
            sup.contain_group()
        except Exception as e:  # noqa: BLE001
            sup.contain["error"] = sup.contain["error"] or f"{type(e).__name__}: {e}"
        try:
            sup.remove_own_pending()
        finally:
            sup.release()
    # ---- after release: classification and reporting are outside the bound ----
    if code is None:
        st = sup.status
        rc = os.waitstatus_to_exitcode(st["wait"]) if "wait" in st else None
        elapsed = sup.marks["released"] - sup.armed if sup.armed is not None else None
        if sup.sentry is not None and not sup.contain["verified"]:
            code, outcome = EX_SUPERVISOR, "containment_unverified"
        elif _Sig.signum:
            code, outcome = 128 + _Sig.signum, "cancelled"
        elif "spawn_errno" in st:
            code, outcome = EX_SPAWN, "spawn_failed"
        elif sup.reason == "timeout":
            code, outcome = EX_TIMEOUT, "timeout"
        elif rc is None:
            code, outcome = EX_SUPERVISOR, "payload_status_unknown"
        elif elapsed is None or elapsed > opts["budget"]:
            code, outcome = EX_TIMEOUT, "deadline_exceeded"
        elif rc != 0:
            code, outcome = (rc if rc > 0 else 128 - rc), "payload_failed"
        elif sup.strays:
            code, outcome = EX_STRAYS, "strays_contained"
        else:
            code, outcome = EX_OK, "success"
    trace = _report(sup, opts, code, outcome, extra, queue_start, queue_utc)
    text = json.dumps(trace, sort_keys=True)
    print(text, file=sys.stderr)
    if opts["trace_file"]:
        try:
            fd = os.open(opts["trace_file"], os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o600)
            with os.fdopen(fd, "w") as f:
                f.write(text + "\n")
        except OSError as e:
            print(f"ac004_supervisor: trace file not written: {e}", file=sys.stderr)
            return code or EX_SUPERVISOR  # a requested receipt that is missing is not a success
    return code


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
