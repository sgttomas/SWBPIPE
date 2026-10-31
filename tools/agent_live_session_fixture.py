#!/usr/bin/env python3
"""Bounded retained-bundle native qualification; never signals a recorded PID.

Own the actual app and anchor Popen objects. The original app holds its SDK
unreaped at the startup gate while the controller arms EVFILT_PROC and joins an
owned anchor to its group. That original anchor reserves group cleanup authority
through the last observation, even if the app is subsequently SIGKILLed. No
snapshot ancestry or recursive Darwin process tracking is used. Inherited
no-fork enforcement (C self-spawn included) covers supplier startup before arm.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import queue
import select
import subprocess
import threading
import time


class Refusal(RuntimeError):
    pass


class Records:
    """A reader is complete only after normal EOF; a queue snapshot is insufficient."""
    def __init__(self, pipe):
        self.pipe = pipe
        self.queue = queue.Queue(maxsize=512)
        self.records = []
        self.error = None
        self.eof = False
        self.total = 0
        self.reader = threading.Thread(target=self._read, daemon=True)
        self.reader.start()

    def _read(self):
        try:
            while True:
                line = self.pipe.readline(16385)
                if not line:
                    self.eof = True
                    return
                self.total += len(line)
                if len(line) > 16384 or not line.endswith(b"\n") or self.total > 1024 * 1024:
                    raise Refusal("control_output_limit_or_truncation")
                value = json.loads(line)
                if not isinstance(value, dict) or set(value) != {"record", "elapsedMs", "data"}:
                    raise Refusal("control_output_shape")
                self.queue.put_nowait(value)
        except Exception:
            self.error = "control_output_invalid_or_incomplete"

    def drain(self):
        while True:
            try:
                self.records.append(self.queue.get_nowait())
            except queue.Empty:
                break
        if self.error:
            raise Refusal(self.error)
        if any(r["record"] in {"failed", "startup_failure", "session_failure", "custody_retained"} for r in self.records):
            raise Refusal("fixture_reported_failure")

    def finish(self, deadline):
        self.reader.join(max(0, deadline - time.monotonic()))
        if self.reader.is_alive() or not self.eof:
            raise Refusal("control_reader_incomplete")
        self.drain()  # stable: normal pipe EOF and completed reader, then ALL records.
        self.pipe.close()

    def has(self, kind):
        self.drain()
        return any(r["record"] == kind for r in self.records)

    def last(self, kind):
        self.drain()
        return next((r for r in reversed(self.records) if r["record"] == kind), None)


class Scope:
    """Observe original SDK only. Any observation error/fork latches forever."""
    def __init__(self, pid):
        self.pid = pid
        self.invalid = None
        self.exited = False
        self.status = None
        self.kq = select.kqueue()
        flags = select.KQ_NOTE_EXIT | 0x04000000 | select.KQ_NOTE_FORK  # SDK NOTE_EXITSTATUS; omitted by Python.
        event = select.kevent(pid, filter=select.KQ_FILTER_PROC,
                              flags=select.KQ_EV_ADD | select.KQ_EV_ENABLE | select.KQ_EV_CLEAR,
                              fflags=flags)
        try:
            self.kq.control([event], 0, 0)
            self.sample()
        except Exception:
            self.invalid = "observation"
            self.kq.close()
            raise Refusal("observer_arm")

    def sample(self):
        try:
            for event in self.kq.control(None, 16, 0):
                if event.ident != self.pid:
                    self.invalid = self.invalid or "identity"
                if event.flags & select.KQ_EV_ERROR:
                    self.invalid = self.invalid or "observation"
                if event.fflags & select.KQ_NOTE_FORK:
                    self.invalid = self.invalid or "fork"
                if event.fflags & select.KQ_NOTE_EXIT:
                    self.exited = True
                    self.status = event.data
        except Exception:
            self.invalid = self.invalid or "observation"
        if self.invalid:
            raise Refusal("scope_" + self.invalid)
        return self.exited

    def close(self):
        self.kq.close()


def pid_live(pid):
    try:
        os.kill(pid, 0)  # liveness only; never signal authority.
        return True
    except ProcessLookupError:
        return False
    except PermissionError:
        return True


def bounded_wait(predicate, deadline, *, check=lambda: None):
    while True:
        check()
        if predicate():
            check()  # cancellation/safety can arrive DURING the final sample.
            if time.monotonic() >= deadline:
                raise Refusal("deadline")
            return
        check()
        if time.monotonic() >= deadline:
            raise Refusal("deadline")
        time.sleep(.005)


def send(app, cmd):
    app.stdin.write(json.dumps({"cmd": cmd}).encode() + b"\n")
    app.stdin.flush()


def anchor_start(probe, sdk, app):
    # SDK id is still reserved by its original app-held Child. Join only while
    # that app is alive, before opening its protocol gate. Anchor retains group.
    if app.poll() is not None:
        raise Refusal("app_lost_before_anchor")
    anchor = subprocess.Popen([str(probe), "--anchor"], process_group=sdk,
                              stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                              stderr=subprocess.DEVNULL, env={})
    ready, _, _ = select.select([anchor.stdout], [], [], 5)
    if not ready or anchor.stdout.readline(128) != f"anchor_group={sdk}\n".encode():
        anchor.kill()
        anchor.wait(timeout=5)
        raise Refusal("anchor_identity")
    if app.poll() is not None or os.getpgid(anchor.pid) != sdk:
        anchor.kill()
        anchor.wait(timeout=5)
        raise Refusal("anchor_custody")
    return anchor


def cleanup_anchor(anchor):
    # This original unreaped Child is the only rescue authority. No killpg(PID).
    if anchor.returncode is None:
        try:
            anchor.stdin.write(b"KILL\n")
            anchor.stdin.flush()
        except BrokenPipeError:
            pass
        anchor.stdin.close()
        anchor.wait(timeout=5)
    anchor.stdout.close()


def validate_terminal(records, mode, trigger):
    kinds = [r["record"] for r in records]
    if any(k in {"failed", "session_failure", "startup_failure"} for k in kinds):
        raise Refusal("terminal_failure_evidence")
    if trigger == "crash":
        if "crash_armed" not in kinds or "finished" in kinds:
            raise Refusal("crash_causal_evidence")
        if mode != "idle" and "turn_ended" in kinds:
            raise Refusal("turn_ended_before_crash")
    if trigger == "roundtrip":
        ended = [r for r in records if r["record"] == "turn_ended"]
        if "tool_completed" not in kinds or len(ended) != 1 or ended[0]["data"]["status"] != "completed":
            raise Refusal("roundtrip_not_completed")


def run_case(args, name, mode, trigger, root, key=None):
    state = root / name
    state.mkdir(mode=0o700)
    app = None
    anchor = None
    observer = None
    records = None
    result = {"case": name, "mode": mode, "trigger": trigger, "passed": False,
              "app": None, "anchor": None, "supplier": None, "cleanup": "unknown"}
    cancelled = args.cancel_file

    def safety():
        if cancelled and cancelled.exists():
            raise Refusal("cancelled")
        if observer:
            observer.sample()
        if records:
            records.drain()

    try:
        app = subprocess.Popen([str(args.app), "--native-agent-session-fixture"],
                               stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                               stderr=subprocess.DEVNULL, env={"PATH": "/usr/bin:/bin"})
        result["app"] = {"pid": app.pid}
        records = Records(app.stdout)
        config = {"state": str(state), "model": args.model, "contextWindow": args.context_window, "endpoint": args.endpoint,
                  "key": key, "mode": mode, "processProbe": str(args.process_probe),
                  "boundaryProbe": str(args.boundary_probe)}
        app.stdin.write(json.dumps(config).encode() + b"\n")
        app.stdin.flush()
        bounded_wait(lambda: records.has("ready"), time.monotonic() + 40, check=safety)
        ready = records.last("ready")["data"]
        sdk = ready["sdkPid"]
        if ready["appPid"] != app.pid or app.poll() is not None:
            raise Refusal("app_identity")
        result["supplier"] = {"pid": sdk, "scope": ready["scope"]}
        observer = Scope(sdk)
        if observer.exited:
            raise Refusal("supplier_exited_before_gate")
        anchor = anchor_start(args.process_probe, sdk, app)
        result["anchor"] = {"pid": anchor.pid, "group": sdk}
        send(app, "GO")
        bounded_wait(lambda: records.has("thread_ready"), time.monotonic() + 30, check=safety)
        if mode != "idle" or trigger == "roundtrip":
            send(app, "TEXT")
            stage = "turn_ended" if trigger == "roundtrip" else "tool_pending" if mode == "pending" else "turn_started"
            bounded_wait(lambda: records.has(stage), time.monotonic() + 90, check=safety)
        if trigger == "crash":
            send(app, "ARM_CRASH")
            bounded_wait(lambda: records.has("crash_armed"), time.monotonic() + 3, check=safety)
            if app.poll() is not None or observer.exited:
                raise Refusal("terminal_before_crash")
            begin = time.monotonic()
            app.kill()  # original owned app only.
            app.wait(timeout=5)
            if app.returncode != -9:
                raise Refusal("app_not_sigkill")
            records.finish(begin + 5)  # EOF BEFORE validating already-written terminal data.
            validate_terminal(records.records, mode, trigger)
            bounded_wait(observer.sample, begin + 5, check=safety)
            # This callback retains the original anchor; group rescue follows
            # the final supplier observation, never a saved-PID signal.
            cleanup_anchor(anchor)
            bounded_wait(lambda: not pid_live(sdk), begin + 7, check=safety)
            result["cleanup"] = "qualified_external_owned_group"
        else:
            if trigger == "eof":
                send(app, "EOF")
                bounded_wait(lambda: records.has("supplier_exit"), time.monotonic() + 5, check=safety)
                if not observer.sample():
                    raise Refusal("supplier_exit_not_observed")
            # The anchor has no supplier writer. Retire it via its own Child,
            # not its group (the app still owns the original SDK/group).
            anchor.kill()
            anchor.wait(timeout=5)
            anchor.stdin.close()
            anchor.stdout.close()
            send(app, "FINISH")
            app.stdin.close()
            bounded_wait(lambda: app.poll() is not None, time.monotonic() + 10, check=safety)
            records.finish(time.monotonic() + 3)
            validate_terminal(records.records, mode, trigger)
            if app.returncode != 0 or records.last("finished")["data"]["cleanup"] != "confirmed":
                raise Refusal("app_cleanup_unconfirmed")
            if not observer.sample() or pid_live(sdk):
                raise Refusal("supplier_liveness_unconfirmed")
            result["cleanup"] = "qualified_native_owned_group"
        safety()
        if not observer.exited or observer.status is None or (trigger != "roundtrip" and (not os.WIFEXITED(observer.status) or os.WEXITSTATUS(observer.status) != 0)):
            raise Refusal("supplier_nonzero_or_signal")
        # Marker removal is separate from process proof. Native FINISH removed
        # its own scope; crash orphan files are removable only after this proof.
        if trigger == "crash":
            import shutil
            scope = result["supplier"]["scope"]
            marker = state / "embedded-agent.scope"
            runtime = Path("/private/tmp") / ("swbpipe-agent-" + scope)
            if marker.read_text() != scope or len(scope) != 48 or any(c not in "0123456789abcdef" for c in scope):
                raise Refusal("marker_identity")
            shutil.rmtree(runtime)
            marker.unlink()
        if list(state.iterdir()):
            raise Refusal("retained_state")
        safety()
        result["passed"] = True
    except Exception as error:
        result["failure"] = str(error) if isinstance(error, Refusal) else type(error).__name__
    finally:
        # Invalid observation permanently forbids scope certification/removal.
        # Existing original-child/group rescue authority remains available.
        if app and app.returncode is None:
            if anchor is None:
                # A failed setup is not permission to kill the sole SDK owner.
                # Ask it to close; recover a held original identity if cleanup
                # is unknown and the native fixture deliberately retains it.
                try:
                    send(app, "FINISH")
                    app.stdin.close()
                except (BrokenPipeError, ValueError):
                    pass
                until=time.monotonic()+40
                while app.poll() is None and time.monotonic()<until:
                    if records:
                        try:
                            records.drain()
                        except Refusal:
                            pass
                        held=next((r["data"] for r in reversed(records.records)
                                   if r["record"] in {"ready","custody_retained"}),None)
                        if held and held["appPid"]==app.pid:
                            try:
                                anchor=anchor_start(args.process_probe,held["sdkPid"],app)
                                result["anchor"]={"pid":anchor.pid,"group":held["sdkPid"]}
                                break
                            except Exception:
                                # Never convert failed rescue setup to PID signals.
                                pass
                    time.sleep(.01)
            if anchor is not None and app.poll() is None:
                app.kill()
                app.wait(timeout=5)
            elif app.poll() is None:
                result["rescue"]="retained_original_app_custody"
                result["passed"]=False
        if anchor:
            try:
                cleanup_anchor(anchor)
            except Exception:
                result["rescue"] = "unconfirmed"
                result["passed"] = False
        if app:
            if app.stdin and not app.stdin.closed:
                app.stdin.close()
            result["app"]["status"] = app.returncode
        if records:
            try:
                if records.reader.is_alive():
                    records.finish(time.monotonic() + 3)
            except Exception:
                result["output"] = "incomplete"
                result["passed"] = False
            result["records"] = records.records
        if observer:
            result["supplier"].update(exitObserved=observer.exited, statusRaw=observer.status,
                                      invalid=observer.invalid)
            try:
                safety()
            except Exception:
                result["passed"] = False
                result["failure"] = "final_safety_unconfirmed"
            observer.close()
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--app", type=Path, required=True)
    parser.add_argument("--process-probe", type=Path, required=True)
    parser.add_argument("--boundary-probe", type=Path, required=True)
    parser.add_argument("--endpoint", required=True)
    parser.add_argument("--model", required=True)
    parser.add_argument("--context-window", type=int, required=True,
                        help="the model's context window in tokens, as the person enters it (8192..131072)")
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--cancel-file", type=Path)
    args = parser.parse_args()
    args.output.mkdir(mode=0o700, parents=False, exist_ok=False)
    root = args.output.resolve()
    for key in ("app", "process_probe", "boundary_probe"):
        setattr(args, key, getattr(args, key).resolve(strict=True))
    cases = [("roundtrip", "idle", "roundtrip")]
    cases += [(f"{trigger}-{mode}", mode, trigger)
              for trigger in ("eof", "crash") for mode in ("idle", "active", "pending")]
    report = {"passed": False, "appSha256": hashlib.sha256(args.app.read_bytes()).hexdigest(), "cases": []}
    for name, mode, trigger in cases:
        result = run_case(args, name, mode, trigger, root)
        if args.cancel_file and args.cancel_file.exists():
            result["passed"] = False
            result["failure"] = "cancelled"
        report["cases"].append(result)
        report["passed"] = len(report["cases"]) == len(cases) and all(c["passed"] for c in report["cases"])
        (root / "result.json").write_text(json.dumps(report, indent=2) + "\n")
        if not result["passed"]:
            break
    return 0 if report["passed"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
