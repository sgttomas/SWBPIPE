"""Private-fixture tests for ac004_supervisor.py.

Run directly: `python3 tools/ac004_supervisor_test.py` (standard-library unittest; no pytest,
no Cargo).  Every test builds its own temporary guard directory, lock files and fake memory
guard, and uses only lightweight fake payloads that limit their own lifetime.  Nothing here
reads or touches the host's real guard directory, locks or pending marker.
"""
import json
import os
import shutil
import signal
import subprocess
import sys
import tempfile
import textwrap
import time
import unittest
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

    def script(self, name, body):
        path = self.tmp / name
        path.write_text("import os, sys, time, signal, fcntl\nOUT = %r\n" % str(self.out) + textwrap.dedent(body))
        return [PY, str(path)]

    def start(self, payload, budget="2", reserve="0.6", grace="0.2", extra=()):
        argv = [PY, TOOL, "--guard-dir", str(self.guard), "--budget", budget, "--reserve", reserve,
                "--term-grace", grace, *extra, "--", *payload]
        return self.spawn(argv, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)

    def finish(self, proc):
        try:
            out, err = proc.communicate(timeout=RUN_LIMIT)
        except subprocess.TimeoutExpired:
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
    def test_members_sees_own_group(self):
        self.assertIn((os.getpid(), False), ac.members(os.getpgrp()))

    def test_production_policy_constants(self):
        self.assertEqual(ac.CEILING_S, 60.0)
        self.assertLess(ac.DEFAULT_TERM_GRACE_S, ac.DEFAULT_RESERVE_S)
        self.assertLessEqual(ac.DEFAULT_BUDGET_S, ac.CEILING_S)


if __name__ == "__main__":
    unittest.main()
