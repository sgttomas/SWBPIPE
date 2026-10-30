"""Offline regressions for native qualification's evidence boundaries."""
import importlib.util
import io
from pathlib import Path
import time
import pytest

spec = importlib.util.spec_from_file_location("live_fixture", Path(__file__).with_name("agent_live_session_fixture.py"))
fixture = importlib.util.module_from_spec(spec)
spec.loader.exec_module(fixture)


def test_crash_cannot_hide_delayed_terminal_evidence():
    # finish must await the producer through EOF, not inspect a momentary queue.
    import os
    read, write = os.pipe()
    records = fixture.Records(os.fdopen(read, "rb"))
    with os.fdopen(write, "wb") as output:
        output.write(b'{"record":"crash_armed","elapsedMs":1,"data":{}}\n')
        output.flush()
        output.write(b'{"record":"turn_ended","elapsedMs":2,"data":{"status":"completed"}}\n')
    records.finish(time.monotonic() + 1)
    with pytest.raises(fixture.Refusal, match="turn_ended_before_crash"):
        fixture.validate_terminal(records.records, "active", "crash")


def test_reader_must_reach_normal_eof_and_reject_truncation():
    records = fixture.Records(io.BytesIO(b'{"record":"finished"'))
    with pytest.raises(fixture.Refusal):
        records.finish(time.monotonic() + 1)


def test_reader_drain_keeps_static_failure_record():
    records = fixture.Records(io.BytesIO(b'{"record":"failed","elapsedMs":1,"data":{"reason":"observation"}}\n'))
    with pytest.raises(fixture.Refusal, match="fixture_reported_failure"):
        records.finish(time.monotonic() + 1)
    assert records.records[0]["data"]["reason"] == "observation"


def test_cancel_during_last_successful_sample_is_checked():
    cancelled = False
    def check():
        if cancelled:
            raise fixture.Refusal("cancelled")
    def sample():
        nonlocal cancelled
        cancelled = True
        return True
    with pytest.raises(fixture.Refusal, match="cancelled"):
        fixture.bounded_wait(sample, time.monotonic() + 1, check=check)


def test_slow_success_cannot_escape_deadline():
    with pytest.raises(fixture.Refusal, match="deadline"):
        fixture.bounded_wait(lambda: True, time.monotonic() - 1)


def test_policy_invalidation_latches_after_following_exit(monkeypatch):
    for name, value in [("KQ_EV_ERROR", 1), ("KQ_NOTE_FORK", 2), ("KQ_NOTE_EXIT", 4)]:
        monkeypatch.setattr(fixture.select, name, value, raising=False)
    class Event:
        ident = 11
        flags = 0
        fflags = 2
        data = 0
    class Queue:
        event = Event()
        def control(self, *_):
            return [self.event]
    observer = object.__new__(fixture.Scope)
    observer.pid = 11
    observer.invalid = None
    observer.exited = False
    observer.status = None
    observer.kq = Queue()
    with pytest.raises(fixture.Refusal, match="scope_fork"):
        observer.sample()
    observer.kq.event.fflags = 4
    with pytest.raises(fixture.Refusal, match="scope_fork"):
        observer.sample()
    assert observer.exited
    assert observer.invalid == "fork"
