"""Private-fixture tests for ac004_supervisor.py.

Run directly: `python3 tools/ac004_supervisor_test.py` (standard-library unittest; no pytest,
no Cargo).  Every test builds its own temporary guard directory, lock files and fake memory
guard, and uses only lightweight fake payloads that limit their own lifetime.  Nothing here
reads or touches the host's real guard directory, locks or pending marker.
"""
import json
import errno
import io
import os
import shutil
import signal
import subprocess
import sys
import tempfile
import textwrap
import time
import unittest
from unittest import mock
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import ac004_supervisor as ac  # noqa: E402

TOOL = str(HERE / "ac004_supervisor.py")
LOCKF = "/usr/bin/lockf" if os.path.exists("/usr/bin/lockf") else None
FLOCK = shutil.which("flock")
PY = sys.executable
RUN_LIMIT = 25.0   # each supervisor run in this suite is killed after this


def holder_cmd(path, seconds):
    """A real external lock holder: BSD lockf(1) on macOS, util-linux flock(1) or Python otherwise."""
    if LOCKF:
        return [LOCKF, "-k", path, "sleep", str(seconds)]
    if FLOCK:
        return [FLOCK, path, "sleep", str(seconds)]
    return [PY, "-c", "import fcntl,os,sys,time;fd=os.open(sys.argv[1],os.O_RDONLY);"
            "fcntl.flock(fd,fcntl.LOCK_EX);time.sleep(float(sys.argv[2]))", path, str(seconds)]


def probe_busy(path):
    """True if an external utility cannot take the lock now."""
    if LOCKF:
        rc = subprocess.run([LOCKF, "-s", "-k", "-t", "0", path, "true"], timeout=10).returncode
        assert rc in (0, 75), rc
        return rc == 75
    if FLOCK:
        rc = subprocess.run([FLOCK, "-n", path, "true"], timeout=10).returncode
        return rc != 0
    rc = subprocess.run([PY, "-c", "import fcntl,os,sys;fd=os.open(sys.argv[1],os.O_RDONLY);"
                         "fcntl.flock(fd,fcntl.LOCK_EX|fcntl.LOCK_NB)", path], timeout=10,
                        stderr=subprocess.DEVNULL).returncode
    return rc != 0


def wait_for(pred, limit=10.0):
    end = time.monotonic() + limit
    while time.monotonic() < end:
        if pred():
            return True
        time.sleep(0.02)
    return False


def process_identity(pid):
    """Kernel birth identity, session and group for an exact private fixture PID."""
    try:
        if sys.platform == "darwin":
            # proc_bsdinfo, PROC_PIDTBSDINFO: the final two uint64s are start timeval.
            buf = ac.ctypes.create_string_buffer(136)
            if ac.ctypes.CDLL(None).proc_pidinfo(pid, 3, 0, buf, 136) != 136:
                return None
            birth = buf.raw[120:136].hex()
        else:
            data = Path(f"/proc/{pid}/stat").read_bytes()
            birth = data[data.rindex(b")") + 2:].split()[19].decode()
        return [pid, os.getsid(pid), os.getpgid(pid), birth]
    except (OSError, ValueError, IndexError):
        return None


class Fixture(unittest.TestCase):
    def setUp(self):
        self.tmp = Path(tempfile.mkdtemp(prefix="ac004-test-"))
        self.guard = self.tmp / "guard"
        self.guard.mkdir()
        for name in ac.LOCK_NAMES:
            (self.guard / name).touch()
        self.inodes = {n: (self.guard / n).stat().st_ino for n in ac.LOCK_NAMES}
        self.out = self.tmp / "out"
        self.out.mkdir()
        self.procs = []
        self.payload_records = []
        self.descendant_records = self.tmp / "descendant-identities"
        self.descendant_records.mkdir()
        self.memguard = self.spawn([PY, "-c", "import time; time.sleep(120)", str(self.guard / ac.MEMGUARD_NAME)])
        # an unrelated same-user process and a foreign file that must survive every run
        self.sentinel = self.spawn(["sleep", "120"])
        (self.guard / "foreign.txt").write_text("not ours\n")
        self.addCleanup(self.cleanup)

    def spawn(self, argv, **kw):
        p = subprocess.Popen(argv, start_new_session=True, **kw)
        self.procs.append(p)
        return p

    def cleanup(self):
        # independent, bounded cleanup of everything this test started (own private sessions only)
        self.cleanup_payloads()
        for p in reversed(self.procs):
            if p.poll() is None:
                try:
                    os.killpg(p.pid, signal.SIGKILL)
                except (ProcessLookupError, PermissionError):
                    pass
            try:
                p.wait(timeout=5)
            except subprocess.TimeoutExpired:
                pass
        shutil.rmtree(self.tmp, ignore_errors=True)

    def cleanup_payloads(self):
        # Registration comes from private launches/forks, not a success trace.
        # Check kernel birth/session/group identities before signalling, never a name match.
        for record in self.payload_records + sorted(self.descendant_records.glob("*.json")):
            if not record.exists():
                continue
            identities = [json.loads(line) for line in record.read_text().splitlines()]
            owned = next((i for i in identities if i and process_identity(i[0]) == i), None)
            if owned is None:
                continue
            pgid = owned[2]
            try:
                os.killpg(pgid, signal.SIGKILL)
            except ProcessLookupError:
                pass
            if not wait_for(lambda: not [m for m in ac.members(pgid) if not m[1]], 5):
                raise AssertionError("private fixture group survived independent cleanup")

    def script(self, name, body):
        path = self.tmp / name
        # Every maintained fake descendant is forked here. Register in the child before
        # returning to either branch, so the parent cannot disappear before registration.
        # This instrumentation changes no production process/descriptor behavior.
        boot = f"""
import os, sys, time, signal, fcntl, json, select
sys.path.insert(0, {str(HERE)!r})
from ac004_supervisor_test import process_identity
OUT = {str(self.out)!r}
original_fork = os.fork
def registered_fork():
    read_fd, write_fd = os.pipe()
    pid = original_fork()
    if pid == 0:
        os.close(read_fd)
        try:
            identity = process_identity(os.getpid())
            if identity is None:
                os._exit(70)
            with open(os.path.join({str(self.descendant_records)!r}, str(os.getpid()) + '.json'), 'w') as f:
                f.write(json.dumps(identity) + '\\n')
            os.write(write_fd, b'1')
        except BaseException:
            os._exit(70)
        finally:
            os.close(write_fd)
    else:
        os.close(write_fd)
        try:
            if not select.select([read_fd], [], [], 3)[0] or os.read(read_fd, 1) != b'1':
                os.kill(pid, signal.SIGKILL)
                os.waitpid(pid, 0)
                raise RuntimeError('fake descendant registration failed')
        finally:
            os.close(read_fd)
    return pid
os.fork = registered_fork
"""
        path.write_text(boot + textwrap.dedent(body))
        return [PY, str(path)]

    def start(self, payload, budget="2", reserve="0.6", grace="0.2", extra=(), patch=""):
        record = self.tmp / f"payload-group-{len(self.payload_records)}"
        self.payload_records.append(record)
        # Test-only instrumentation records exact owned identities before running payloads.
        # The production tool and its payload descriptor policy are unchanged.
        runner = f"""
import sys, os, json
sys.path.insert(0, {str(HERE)!r})
import ac004_supervisor as ac
from ac004_supervisor_test import process_identity
original_sentry = ac._sentry_main
def tracked_sentry(*args):
    os.setpgid(0, 0)
    def record(pid):
        with open({str(record)!r}, 'a') as f:
            f.write(json.dumps(process_identity(pid)) + '\\n')
    record(os.getpid())
    original_popen = ac.subprocess.Popen
    def tracked_popen(*args, **kwargs):
        child = original_popen(*args, **kwargs)
        record(child.pid)
        return child
    ac.subprocess.Popen = tracked_popen
    original_sentry(*args)
ac._sentry_main = tracked_sentry
{patch}
sys.exit(ac.main(sys.argv[1:]))
"""
        argv = [PY, "-c", runner, "--guard-dir", str(self.guard), "--budget", budget, "--reserve", reserve,
                "--term-grace", grace, *extra, "--", *payload]
        return self.spawn(argv, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)

    def finish(self, proc):
        try:
            out, err = proc.communicate(timeout=RUN_LIMIT)
        except subprocess.TimeoutExpired:
            self.cleanup_payloads()
            os.killpg(proc.pid, signal.SIGKILL)
            proc.communicate(timeout=5)
            self.fail("supervisor run did not end within the suite limit")
        trace = json.loads(err.strip().splitlines()[-1])
        self.assertEqual(trace["exit_code"], proc.returncode)
        return proc.returncode, trace, out

    def run_sup(self, payload, **kw):
        return self.finish(self.start(payload, **kw))

    def assert_released_and_intact(self, trace=None):
        for name in ac.LOCK_NAMES:
            path = self.guard / name
            self.assertEqual(path.stat().st_ino, self.inodes[name], f"{name} inode kept")
            self.assertFalse(probe_busy(str(path)), f"{name} free for a contender after return")
        self.assertFalse((self.guard / ac.PENDING_NAME).exists())
        self.assertIsNone(self.sentinel.poll(), "unrelated process untouched")
        self.assertEqual((self.guard / "foreign.txt").read_text(), "not ours\n")
        if trace and trace["payload"]["pgid"]:
            self.assertEqual([m for m in ac.members(trace["payload"]["pgid"]) if not m[1]], [],
                             "no live member of the owned group")


FD_REPORT = """
def held_lock_fds():
    locks = {(os.stat(os.path.join(GUARD, n)).st_dev, os.stat(os.path.join(GUARD, n)).st_ino) for n in NAMES}
    held = []
    for fd in range(3, 256):
        try:
            st = os.fstat(fd)
        except OSError:
            continue
        if (st.st_dev, st.st_ino) in locks:
            held.append(fd)
    return held
"""


class Outcomes(Fixture):
    def test_success_releases_everything(self):
        rc, t, out = self.run_sup(["sh", "-c", "echo payload-ran"])
        self.assertEqual((rc, t["outcome"]), (0, "success"))
        self.assertIn("payload-ran", out)
        self.assertTrue(t["containment"]["verified"])
        self.assertTrue(t["release_within_budget"])
        self.assert_released_and_intact(t)

    def test_nonzero_and_signaled_payloads_are_preserved(self):
        rc, t, _ = self.run_sup(["sh", "-c", "exit 7"])
        self.assertEqual((rc, t["outcome"], t["payload"]["exit_code"]), (7, "payload_failed", 7))
        rc, t, _ = self.run_sup(["sh", "-c", "kill -KILL $$"])
        self.assertEqual((rc, t["outcome"], t["payload"]["signal"]), (128 + 9, "payload_failed", 9))
        self.assert_released_and_intact(t)

    def test_payload_sees_all_four_locks_held_and_inherits_none(self):
        names = list(ac.LOCK_NAMES)
        payload = self.script("fds.py", f"GUARD = {str(self.guard)!r}\nNAMES = {names!r}\n" + FD_REPORT + """
pid = os.fork()
if pid == 0:
    open(os.path.join(OUT, "grandchild"), "w").write(repr(held_lock_fds()))
    os._exit(0)
os.waitpid(pid, 0)
busy = []
for n in NAMES:
    fd = os.open(os.path.join(GUARD, n), os.O_RDONLY)
    try:
        fcntl.flock(fd, fcntl.LOCK_EX | fcntl.LOCK_NB); busy.append(False)
    except BlockingIOError:
        busy.append(True)
    os.close(fd)
open(os.path.join(OUT, "payload"), "w").write(repr((held_lock_fds(), busy)))
""")
        rc, t, _ = self.run_sup(payload)
        self.assertEqual(rc, 0, t)
        self.assertEqual((self.out / "payload").read_text(), repr(([], [True] * 4)))
        self.assertEqual((self.out / "grandchild").read_text(), "[]")
        self.assert_released_and_intact(t)


class QueueAndDeadline(Fixture):
    def test_delayed_controller_skips_grace_to_preserve_teardown(self):
        payload = self.script("delayed.py", "signal.signal(signal.SIGTERM, signal.SIG_IGN)\ntime.sleep(30)\n")
        patch = """
original_read = ac.Supervisor.read_status
def delayed_read(self, timeout):
    if not hasattr(self, '_delayed'):
        self._delayed = True
        ac.time.sleep(max(0, self.armed + 2.15 - ac.time.monotonic()))
    return original_read(self, timeout)
ac.Supervisor.read_status = delayed_read
"""
        rc, t, _ = self.run_sup(payload, budget="2.5", reserve="1", grace="0.7", patch=patch)
        self.assertEqual((rc, t["outcome"]), (ac.EX_TIMEOUT, "timeout"))
        marks = t["marks_from_armed_s"]
        self.assertGreater(marks["term_sent"], 2.1)
        self.assertLess(marks["kill_sent"] - marks["term_sent"], 0.3)
        self.assertLessEqual(t["armed_to_release_s"], 2.5)
        self.assert_released_and_intact(t)

    def test_delayed_setup_does_not_launch_payload(self):
        marker = self.out / "ran"
        patch = """
original_verify = ac.Supervisor.verify_inodes
def delayed_verify(self):
    original_verify(self)
    ac.time.sleep(max(0, self.armed + 1.1 - ac.time.monotonic()))
ac.Supervisor.verify_inodes = delayed_verify
"""
        rc, t, _ = self.run_sup(["sh", "-c", f": > {marker}"], budget="1.5", reserve="0.6", patch=patch)
        self.assertEqual((rc, t["outcome"], t["payload"]["status"]), (ac.EX_TIMEOUT, "timeout", "not_started"))
        self.assertFalse(marker.exists())
        self.assertLessEqual(t["armed_to_release_s"], 1.5)
        self.assert_released_and_intact(t)

    def test_queue_is_untimed_and_budget_starts_at_final_lock(self):
        # slot 4 is held longer than the whole budget; the run still succeeds
        hold = self.spawn(holder_cmd(str(self.guard / ac.LOCK_NAMES[3]), 3.0))
        self.assertTrue(wait_for(lambda: probe_busy(str(self.guard / ac.LOCK_NAMES[3]))))
        marker = self.out / "ran"
        sup = self.start(["sh", "-c", f": > {marker}"], budget="1.5", reserve="0.5")
        pending = self.guard / ac.PENDING_NAME
        self.assertTrue(wait_for(pending.exists))
        self.assertEqual(pending.read_text(), f"{sup.pid}\n", "pending names the live controller")
        for name in ac.LOCK_NAMES[:3]:
            self.assertTrue(probe_busy(str(self.guard / name)), f"{name} held while queued")
        time.sleep(1.0)
        self.assertFalse(marker.exists(), "no payload before all four locks")
        rc, t, _ = self.finish(sup)
        hold.wait(timeout=10)
        self.assertEqual((rc, t["outcome"]), (0, "success"))
        self.assertTrue(marker.exists())
        self.assertGreater(t["queue_wait_s"], 1.5, "queue wait exceeded the budget without a timeout")
        self.assertLessEqual(t["armed_to_release_s"], 1.5)
        self.assertLessEqual(t["marks_from_armed_s"]["armed"], 0.0)
        self.assertGreaterEqual(t["marks_from_armed_s"]["cargo_job.slot4.lock_acquired"], 0.0)
        self.assert_released_and_intact(t)

    def test_hang_is_bounded_through_release_and_contender_reacquires(self):
        started = self.out / "started"
        sup = self.start(["sh", "-c", f": > {started}; exec sleep 30"], budget="1.5", reserve="0.6", grace="0.2")
        self.assertTrue(wait_for(started.exists))
        # real contender, blocked on slot 1, records when it actually gets the lock
        stamp = self.out / "contender"
        contender = self.spawn([PY, "-c", "import fcntl,os,sys,time;fd=os.open(sys.argv[1],os.O_RDONLY);"
                                "fcntl.flock(fd,fcntl.LOCK_EX);open(sys.argv[2],'w').write(repr(time.monotonic()))",
                                str(self.guard / ac.LOCK_NAMES[0]), str(stamp)])
        rc, t, _ = self.finish(sup)
        contender.wait(timeout=10)
        self.assertEqual((rc, t["outcome"], t["stop_reason"]), (ac.EX_TIMEOUT, "timeout", "timeout"))
        self.assertLessEqual(t["armed_to_release_s"], 1.5)
        acquired = float(stamp.read_text())
        self.assertGreaterEqual(acquired, t["released_monotonic"])
        self.assertLessEqual(acquired - t["armed_monotonic"], 1.5 + 0.5, "contender in within the budget")
        self.assert_released_and_intact(t)

    def test_term_resistant_child_and_grandchild_are_contained(self):
        payload = self.script("stubborn.py", """
signal.signal(signal.SIGTERM, signal.SIG_IGN)
if os.fork() == 0:
    open(os.path.join(OUT, "gc"), "w").write(str(os.getpid()))
    time.sleep(30); os._exit(0)
open(os.path.join(OUT, "main"), "w").write(str(os.getpid()))
time.sleep(30)
""")
        rc, t, _ = self.run_sup(payload, budget="2", reserve="0.8", grace="0.3")
        self.assertEqual((rc, t["outcome"]), (ac.EX_TIMEOUT, "timeout"))
        self.assertTrue(t["containment"]["verified"])
        self.assertLessEqual(t["armed_to_release_s"], 2.0)
        self.assert_released_and_intact(t)

    def test_main_exiting_first_leaves_no_descendant_and_no_success(self):
        payload = self.script("orphan.py", """
if os.fork() == 0:
    signal.signal(signal.SIGTERM, signal.SIG_IGN)
    time.sleep(30); os._exit(0)
""")
        rc, t, _ = self.run_sup(payload)
        self.assertEqual((rc, t["outcome"], t["payload"]["exit_code"]), (ac.EX_STRAYS, "strays_contained", 0))
        self.assertGreaterEqual(t["payload"]["strays_at_exit"], 1)
        self.assert_released_and_intact(t)


class LockfInterop(Fixture):
    def setUp(self):
        super().setUp()
        if not (LOCKF or FLOCK):
            self.skipTest("no lockf(1) or flock(1) utility")

    def test_utility_holder_blocks_supervisor(self):
        for name in ac.LOCK_NAMES:
            with self.subTest(lock=name):
                hold = self.spawn(holder_cmd(str(self.guard / name), 1.2))
                self.assertTrue(wait_for(lambda: probe_busy(str(self.guard / name))))
                rc, t, _ = self.run_sup(["true"])
                hold.wait(timeout=10)
                self.assertEqual(rc, 0)
                self.assertGreater(t["queue_wait_s"], 0.8, f"{name} held by the utility made us wait")

    def test_supervisor_blocks_utility_then_releases(self):
        started = self.out / "started"
        sup = self.start(["sh", "-c", f": > {started}; sleep 1"], budget="3", reserve="0.6")
        self.assertTrue(wait_for(started.exists))
        self.assertEqual([probe_busy(str(self.guard / n)) for n in ac.LOCK_NAMES], [True] * 4)
        rc, t, _ = self.finish(sup)
        self.assertEqual(rc, 0)
        self.assert_released_and_intact(t)


class FailuresAndCancellation(Fixture):
    def test_membership_inspection_failure_cannot_pass(self):
        patch = """
def unavailable(*args, **kwargs):
    raise OSError(5, 'injected process inspection error')
ac._live = unavailable
"""
        rc, t, _ = self.run_sup(["true"], patch=patch)
        self.assertEqual(rc, ac.EX_SUPERVISOR)
        self.assertFalse(t["containment"]["verified"])
        self.assertIn("injected process inspection error", t["containment"]["error"])
        self.assert_released_and_intact(t)

    def test_short_pending_write_fails_and_removes_owned_partial_marker(self):
        patch = """
import resource
resource.setrlimit(resource.RLIMIT_FSIZE, (1, 1))
"""
        rc, t, _ = self.run_sup(["true"], patch=patch)
        self.assertEqual((rc, t["outcome"]), (ac.EX_SUPERVISOR, "internal_error"))
        self.assertIn("incomplete pending-marker", t["error"])
        self.assertIn("pending_removed", t["marks_from_queue_start_s"])
        self.assert_released_and_intact(t)

    def test_foreign_fifo_during_queue_is_never_opened(self):
        marker = self.out / "ran"
        patch = """
original_acquire = ac.Supervisor.acquire
def changed_acquire(self):
    os.mkfifo(self.pending)
    return original_acquire(self)
ac.Supervisor.acquire = changed_acquire
"""
        rc, t, _ = self.run_sup(["sh", "-c", f": > {marker}"], patch=patch)
        self.assertEqual((rc, t["outcome"]), (ac.EX_PENDING, "refused"))
        self.assertFalse(marker.exists())
        self.assertTrue(ac.stat.S_ISFIFO((self.guard / ac.PENDING_NAME).lstat().st_mode))
        (self.guard / ac.PENDING_NAME).unlink()  # this test created the foreign fixture
        self.assert_released_and_intact(t)

    def test_independent_cleanup_survives_disabled_sentry_death_kill(self):
        marker = self.out / "started"
        payload = self.script("death-fault.py", f"""
signal.signal(signal.SIGTERM, signal.SIG_IGN)
if os.fork() == 0:
    time.sleep(30); os._exit(0)
open({str(marker)!r}, 'w').write(str(os.getpgrp()))
time.sleep(30)
""")
        # Disable only the sentry's kill when its controller dies, not fixture cleanup.
        patch = """
recorded_sentry = ac._sentry_main
def broken_sentry(*args):
    ac.os.killpg = lambda *args: None
    return recorded_sentry(*args)
ac._sentry_main = broken_sentry
"""
        sup = self.start(payload, budget="10", reserve="1", patch=patch)
        self.assertTrue(wait_for(lambda: marker.exists() and marker.read_text()))
        pgid = int(marker.read_text())
        try:
            sup.kill()
            sup.wait(timeout=5)
            self.assertTrue(wait_for(lambda: len([m for m in ac.members(pgid) if not m[1]]) == 2, 3))
        finally:
            self.cleanup_payloads()
        sup.communicate(timeout=5)  # inherited pipes no longer keep this blocked
        self.assertEqual([m for m in ac.members(pgid) if not m[1]], [])
        self.assert_released_and_intact()

    def test_independent_cleanup_retains_orphan_after_both_recorded_processes_die(self):
        marker = self.out / "orphan-identity"
        payload = self.script("orphan-death-fault.py", f"""
if os.fork() == 0:
    with open({str(marker)!r}, 'w') as f:
        f.write(json.dumps(process_identity(os.getpid())))
    time.sleep(30)
    os._exit(0)
""")
        patch = """
recorded_sentry = ac._sentry_main
def broken_sentry(*args):
    ac.os.killpg = lambda *args: None
    return recorded_sentry(*args)
ac._sentry_main = broken_sentry
ac.Supervisor.supervise = lambda self: ac.time.sleep(10)
"""
        sup = self.start(payload, budget="20", reserve="1", patch=patch)
        child_identity = None
        try:
            self.assertTrue(wait_for(lambda: marker.exists() and marker.read_text()))
            child_identity = json.loads(marker.read_text())
            pgid = child_identity[2]
            # The main exited; leave only sentry and orphan before killing the controller.
            self.assertTrue(wait_for(lambda: len([m for m in ac.members(pgid) if not m[1]]) == 2, 3))
            sup.kill()
            sup.wait(timeout=5)
            original_records = [json.loads(line) for line in self.payload_records[-1].read_text().splitlines()]
            self.assertTrue(wait_for(lambda: all(not i or process_identity(i[0]) != i for i in original_records), 3))
            self.assertEqual(process_identity(child_identity[0]), child_identity)
            self.cleanup_payloads()
            self.assertTrue(wait_for(lambda: not [m for m in ac.members(pgid) if not m[1]], 3))
            sup.communicate(timeout=3)
            self.assert_released_and_intact()
        finally:
            # Also contain a negative control that deliberately disables fixture tracking.
            # This identity was written by this exact private grandchild, not inferred by name.
            if child_identity and process_identity(child_identity[0]) == child_identity:
                os.killpg(child_identity[2], signal.SIGKILL)
                self.assertTrue(wait_for(lambda: not [m for m in ac.members(child_identity[2]) if not m[1]], 3))

    def test_spawn_failure_cleans_up(self):
        bad = self.tmp / "bad-interp"
        bad.write_text("#!/nonexistent/interpreter\n")
        bad.chmod(0o755)
        rc, t, _ = self.run_sup([str(bad)])
        self.assertEqual((rc, t["outcome"], t["payload"]["status"]), (ac.EX_SPAWN, "spawn_failed", "spawn_failed"))
        self.assert_released_and_intact(t)

    def test_sigterm_while_queued(self):
        hold = self.spawn(holder_cmd(str(self.guard / ac.LOCK_NAMES[1]), 5))
        self.assertTrue(wait_for(lambda: probe_busy(str(self.guard / ac.LOCK_NAMES[1]))))
        marker = self.out / "ran"
        sup = self.start(["sh", "-c", f": > {marker}"])
        self.assertTrue(wait_for((self.guard / ac.PENDING_NAME).exists))
        sup.send_signal(signal.SIGTERM)
        rc, t, _ = self.finish(sup)
        self.assertEqual((rc, t["outcome"]), (128 + signal.SIGTERM, "cancelled"))
        self.assertFalse(marker.exists())
        self.assertFalse((self.guard / ac.PENDING_NAME).exists())
        self.assertFalse(probe_busy(str(self.guard / ac.LOCK_NAMES[0])), "slot 1 released after cancel")
        os.killpg(hold.pid, signal.SIGKILL)
        hold.wait(timeout=5)
        self.assert_released_and_intact()

    def test_sigint_during_payload(self):
        started = self.out / "started"
        payload = self.script("hang.py", f"""
signal.signal(signal.SIGTERM, signal.SIG_IGN)
open({str(started)!r}, "w").close()
time.sleep(30)
""")
        sup = self.start(payload, budget="5", reserve="1", grace="0.3")
        self.assertTrue(wait_for(started.exists))
        sup.send_signal(signal.SIGINT)
        rc, t, _ = self.finish(sup)
        self.assertEqual((rc, t["outcome"], t["stop_reason"]), (128 + signal.SIGINT, "cancelled", "cancelled"))
        self.assert_released_and_intact(t)

    def test_controller_killed_abruptly_takes_its_group_down(self):
        pids = self.out / "pids"
        payload = self.script("tree.py", f"""
signal.signal(signal.SIGTERM, signal.SIG_IGN)
if os.fork() == 0:
    time.sleep(30); os._exit(0)
open({str(pids)!r}, "w").write(str(os.getpgrp()))
time.sleep(30)
""")
        sup = self.start(payload, budget="10", reserve="1")
        self.assertTrue(wait_for(lambda: pids.exists() and pids.read_text()))
        pgid = int(pids.read_text())
        self.assertEqual(len([m for m in ac.members(pgid) if not m[1]]), 3)  # sentry, payload, grandchild
        sup.kill()
        sup.communicate(timeout=10)
        self.assertTrue(wait_for(lambda: not [m for m in ac.members(pgid) if not m[1]], limit=5),
                        "owned group gone after controller death")
        self.assertTrue(wait_for(lambda: not any(probe_busy(str(self.guard / n)) for n in ac.LOCK_NAMES), 5))
        self.assert_released_and_intact()

    def test_replaced_lock_file_is_refused_after_queue(self):
        hold = self.spawn(holder_cmd(str(self.guard / ac.LOCK_NAMES[1]), 1.0))
        self.assertTrue(wait_for(lambda: probe_busy(str(self.guard / ac.LOCK_NAMES[1]))))
        marker = self.out / "ran"
        sup = self.start(["sh", "-c", f": > {marker}"])
        self.assertTrue(wait_for((self.guard / ac.PENDING_NAME).exists))
        fresh = self.guard / "fresh"
        fresh.touch()
        os.replace(fresh, self.guard / ac.LOCK_NAMES[0])
        rc, t, _ = self.finish(sup)
        hold.wait(timeout=5)
        self.assertEqual((rc, t["outcome"]), (ac.EX_CONFIG, "refused"))
        self.assertFalse(marker.exists())
        self.assertFalse((self.guard / ac.PENDING_NAME).exists())


class Refusals(Fixture):
    def refused(self, argv_opts, code, payload=None):
        marker = self.out / "ran"
        payload = payload or ["sh", "-c", f": > {marker}"]
        proc = self.spawn([PY, TOOL, *argv_opts, "--", *payload], stdout=subprocess.PIPE,
                          stderr=subprocess.PIPE, text=True)
        rc, t, _ = self.finish(proc)
        self.assertEqual((rc, t["outcome"]), (code, "refused"), t)
        self.assertFalse(marker.exists())
        return t

    def test_malformed_budget_inputs(self):
        g = ["--guard-dir", str(self.guard)]
        for opts in (["--budget", "nan"], ["--budget", "inf"], ["--budget", "-1"], ["--budget", "0"],
                     ["--budget", "60.5"], ["--budget", "abc"], ["--budget", "2", "--reserve", "2"],
                     ["--budget", "2", "--reserve", "0.5", "--term-grace", "0.5"], ["--reserve", "1e309"],
                     ["--bogus", "1"], ["--budget", "2", "--budget", "3"], ["--budget"]):
            with self.subTest(opts=opts):
                self.refused(g + opts, ac.EX_USAGE)
        self.refused([], ac.EX_USAGE)
        proc = subprocess.run([PY, TOOL, "--guard-dir", str(self.guard), "true"], capture_output=True, timeout=20)
        self.assertEqual(proc.returncode, ac.EX_USAGE)
        self.assert_released_and_intact()

    def test_bad_config(self):
        self.refused(["--guard-dir", str(self.tmp / "missing")], ac.EX_CONFIG)
        (self.guard / ac.LOCK_NAMES[2]).unlink()
        self.refused(["--guard-dir", str(self.guard)], ac.EX_CONFIG)
        (self.guard / ac.LOCK_NAMES[2]).mkdir()
        self.refused(["--guard-dir", str(self.guard)], ac.EX_CONFIG)
        (self.guard / ac.LOCK_NAMES[2]).rmdir()
        os.symlink(self.guard / "foreign.txt", self.guard / ac.LOCK_NAMES[2])
        self.refused(["--guard-dir", str(self.guard)], ac.EX_CONFIG)

    def test_missing_memguard_and_bad_payload_and_trace(self):
        self.refused(["--guard-dir", str(self.guard)], ac.EX_USAGE, payload=[str(self.tmp / "nope")])
        (self.tmp / "exists").touch()
        self.refused(["--guard-dir", str(self.guard), "--trace-file", str(self.tmp / "exists")], ac.EX_USAGE)
        os.killpg(self.memguard.pid, signal.SIGKILL)
        self.memguard.wait(timeout=5)
        self.refused(["--guard-dir", str(self.guard)], ac.EX_MEMGUARD)
        self.assert_released_and_intact()

    def test_foreign_pending_is_left_alone(self):
        pending = self.guard / ac.PENDING_NAME
        pending.write_text(f"{self.sentinel.pid}\n")
        self.refused(["--guard-dir", str(self.guard)], ac.EX_PENDING)
        self.assertEqual(pending.read_text(), f"{self.sentinel.pid}\n")
        pending.write_text("garbage")
        self.refused(["--guard-dir", str(self.guard)], ac.EX_PENDING)
        self.assertEqual(pending.read_text(), "garbage")
        for name in ac.LOCK_NAMES:
            self.assertFalse(probe_busy(str(self.guard / name)))

    def test_trace_file_written_once(self):
        trace = self.tmp / "trace.json"
        rc, t, _ = self.run_sup(["sh", "-c", "exit 0 # SECRET-xyzzy"], extra=("--trace-file", str(trace)),)
        self.assertEqual(rc, 0)
        self.assertEqual(json.loads(trace.read_text())["outcome"], "success")
        self.assertNotIn("xyzzy", trace.read_text(), "argv is hashed, not dumped")


class Units(unittest.TestCase):
    def test_linux_reader_ignores_only_process_disappearance(self):
        for number in (errno.ENOENT, errno.ESRCH, errno.EIO, errno.EACCES):
            with self.subTest(errno=number), mock.patch.object(ac.os, "listdir", return_value=["123"]), \
                    mock.patch("builtins.open", side_effect=OSError(number, "injected")):
                if number in (errno.ENOENT, errno.ESRCH):
                    self.assertEqual(ac._members_linux(123), [])
                else:
                    with self.assertRaises(OSError):
                        ac._members_linux(123)
        with mock.patch.object(ac.os, "listdir", return_value=["123"]), \
                mock.patch("builtins.open", return_value=io.BytesIO(b"123 (fake) malformed")):
            with self.assertRaises(OSError):
                ac._members_linux(123)

    def test_members_sees_own_group(self):
        self.assertIn((os.getpid(), False), ac.members(os.getpgrp()))

    def test_production_policy_constants(self):
        self.assertEqual(ac.CEILING_S, 60.0)
        self.assertLess(ac.DEFAULT_TERM_GRACE_S, ac.DEFAULT_RESERVE_S)
        self.assertLessEqual(ac.DEFAULT_BUDGET_S, ac.CEILING_S)


if __name__ == "__main__":
    unittest.main()
