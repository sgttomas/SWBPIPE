"""D4 monitor: injected telemetry/clock only; no native process, sleep or network."""
import json
from pathlib import Path
from types import SimpleNamespace

import pytest

from tools import agent_journey_fixture as journey
from tools import agent_journey_memory_monitor as monitor


def private(tmp_path):
    root = tmp_path / "monitor"
    root.mkdir(mode=0o700)
    return root


def test_the_metric_is_exactly_free_plus_inactive_plus_speculative_pages():
    text = ("Mach Virtual Memory Statistics: (page size of 16384 bytes)\n"
            "Pages free: 100.\nPages active: 999.\nPages inactive: 200.\nPages speculative: 300.\n")
    assert monitor.parse_free_bytes(text) == 600 * 16384
    for bad in (None, "", text.replace("16384", "0"), text.replace("100.", "-100."),
                text.replace("300.", "300.5"), text + "Pages free: 100.\n"):
        with pytest.raises(monitor.MonitorRefusal):
            monitor.parse_free_bytes(bad)


def test_healthy_producer_publishes_every_two_seconds_until_done(tmp_path):
    root, times, intervals = private(tmp_path), [10.0], []
    def wait(seconds):
        intervals.append(seconds)
        monitor.check_health(root / "heartbeat.json", root / "cancel", clock=lambda: times[0])
        times[0] += seconds
        if len(intervals) == 2:
            (root / "done").touch()
    assert monitor.monitor(root, telemetry=lambda: monitor.MIN_FREE_BYTES, clock=lambda: times[0], wait=wait) == 0
    assert intervals == [2.0, 2.0]
    assert json.loads((root / "heartbeat.json").read_text())["sampledAt"] == 12.0
    assert not (root / "cancel").exists()


@pytest.mark.parametrize("free", [monitor.MIN_FREE_BYTES - 1, None, True, float("nan")])
def test_low_or_invalid_telemetry_creates_the_existing_cancel_file(tmp_path, free):
    root = private(tmp_path)
    assert monitor.monitor(root, telemetry=lambda: free, clock=lambda: 10) == 1
    assert (root / "cancel").exists() and not (root / "heartbeat.json").exists()


@pytest.mark.parametrize("error", [OSError(), monitor.MonitorRefusal("telemetry_invalid"), RuntimeError()])
def test_unavailable_telemetry_and_unexpected_producer_failure_cancel(tmp_path, error):
    root = private(tmp_path)
    def unavailable():
        raise error
    assert monitor.monitor(root, telemetry=unavailable, clock=lambda: 10) == 1
    assert (root / "cancel").exists()


def test_failed_heartbeat_publication_cancels(tmp_path, monkeypatch):
    root = private(tmp_path)
    def broken(*args):
        raise OSError("invented write failure")
    monkeypatch.setattr(monitor, "publish", broken)
    assert monitor.monitor(root, telemetry=lambda: monitor.MIN_FREE_BYTES, clock=lambda: 10) == 1
    assert (root / "cancel").exists()


def test_abrupt_producer_loss_expires_in_consumer_without_a_producer_exception(tmp_path):
    root = private(tmp_path)
    heartbeat, cancellation = root / "heartbeat.json", root / "cancel"
    monitor.publish(heartbeat, monitor.MIN_FREE_BYTES, 10)
    monitor.check_health(heartbeat, cancellation, clock=lambda: 14.999)
    # No producer running/catch block: stale bytes alone must cancel by five seconds.
    with pytest.raises(monitor.MonitorRefusal, match="health_expired"):
        monitor.check_health(heartbeat, cancellation, clock=lambda: 15)
    assert cancellation.exists()


@pytest.mark.parametrize("data", [None, b"not json", b"x" * 1025,
                                    b'{"version":1,"sampledAt":NaN,"freeBytes":999999999999}',
                                    b'{"version":1,"sampledAt":100,"freeBytes":999999999999}'])
def test_missing_malformed_and_future_heartbeats_cancel(tmp_path, data):
    root = private(tmp_path)
    heartbeat = root / "heartbeat.json"
    if data is not None:
        heartbeat.write_bytes(data)
    with pytest.raises(monitor.MonitorRefusal):
        monitor.check_health(heartbeat, root / "cancel", clock=lambda: 10)
    assert (root / "cancel").exists()


def test_lost_telemetry_and_cancel_write_failure_still_refuse_the_owning_consumer(tmp_path, monkeypatch):
    root = private(tmp_path)
    def failed_write(path):
        raise monitor.MonitorRefusal("cancel_delivery_failed")
    monkeypatch.setattr(monitor, "cancel", failed_write)
    with pytest.raises(monitor.MonitorRefusal):
        monitor.check_health(root / "heartbeat.json", root / "cancel", clock=lambda: 10)


def test_slow_sample_is_not_stamped_fresh_after_the_read(tmp_path):
    root, now = private(tmp_path), [10.0]
    def slow():
        now[0] = 15
        return monitor.MIN_FREE_BYTES
    assert monitor.monitor(root, telemetry=slow, clock=lambda: now[0]) == 1
    assert (root / "cancel").exists() and not (root / "heartbeat.json").exists()


def test_monitor_refusal_helper_and_release_helper_preserve_unknown_custody(tmp_path, monkeypatch):
    from tools.agent_journey_fixture_test import FakeApp
    root = private(tmp_path)
    args = SimpleNamespace(mode="live", memory_heartbeat=root / "heartbeat.json", cancel_file=root / "cancel")
    monkeypatch.setattr(journey, "RETAINED", root / "retained")
    monkeypatch.setattr(journey.memory_monitor, "check_health", lambda *a: (_ for _ in ()).throw(monitor.MonitorRefusal("health_expired")))
    app, result = FakeApp(), {"passed": False}
    try:
        journey.check_memory_monitor(args)
        pytest.fail("unhealthy monitoring must refuse")
    except journey.Refusal as error:
        assert str(error) == "memory_monitor_health_expired"
    finally:
        # Same helper the controller's finally uses; no PID/native signal probe.
        journey.release_launch(app, None, None, None, root / "gate", None, False, root, 0, result)
    assert result["release"] == "retained_original_app_custody" and not app.killed
    assert (root / "retained").exists()


def test_native_telemetry_reader_is_bounded_without_executing_it(monkeypatch):
    calls = []
    def fake_run(command, **kwargs):
        calls.append((command, kwargs))
        raise monitor.subprocess.TimeoutExpired(command, 1)
    monkeypatch.setattr(monitor.subprocess, "run", fake_run)
    with pytest.raises(monitor.MonitorRefusal, match="telemetry_unavailable"):
        monitor.read_free_bytes()
    assert calls == [(["/usr/bin/vm_stat"], {"capture_output": True, "text": True, "check": True, "timeout": 1})]


def test_live_admission_requires_health_before_reading_the_single_key(tmp_path, monkeypatch, capsys):
    root = private(tmp_path)
    monkeypatch.setattr(journey, "RETAINED", root / "retained")
    monkeypatch.setattr(journey, "read_key", lambda *args: pytest.fail("key must not be consumed"))
    monkeypatch.setattr(journey.fcntl, "flock", lambda *args: None)
    monkeypatch.setattr(journey.sys, "argv", ["fixture", "--mode", "live", "--app", "unused",
        "--process-probe", "unused", "--boundary-probe", "unused", "--endpoint", "unused", "--key-stdin",
        "--output", str(root / "run"), "--memory-heartbeat", str(root / "heartbeat.json"),
        "--cancel-file", str(root / "cancel"), "--release-grace", "0"])
    # Only private temporary file I/O; don't use the real host lock path.
    real_open = open
    monkeypatch.setattr(journey, "open", lambda path, *a, **kw: real_open(root / "lock", *a, **kw), raising=False)
    assert journey.main() == 2
    assert "memory_monitor_" in capsys.readouterr().err
    assert not (root / "run").exists() and (root / "cancel").exists()


def test_producer_error_and_failed_cancel_write_still_expires_the_consumer_lease(tmp_path, monkeypatch):
    root, now = private(tmp_path), [10.0]
    def failed_cancel(path):
        raise monitor.MonitorRefusal("cancel_delivery_failed")
    def failed_wait(seconds):
        now[0] += monitor.LEASE
        raise RuntimeError("invented producer failure after publication")
    monkeypatch.setattr(monitor, "cancel", failed_cancel)
    with pytest.raises(monitor.MonitorRefusal):
        monitor.monitor(root, telemetry=lambda: monitor.MIN_FREE_BYTES, clock=lambda: now[0], wait=failed_wait)
    with pytest.raises(monitor.MonitorRefusal, match="health_expired"):
        monitor.check_health(root / "heartbeat.json", root / "cancel", clock=lambda: now[0])


def test_unadmitted_directory_is_not_written(tmp_path):
    root = tmp_path / "public"
    root.mkdir(mode=0o755)
    assert monitor.monitor(root, telemetry=lambda: pytest.fail("no telemetry")) == 1
    assert not list(root.iterdir())


@pytest.mark.parametrize("changes", [
    {"freeBytes": monitor.MIN_FREE_BYTES - 1}, {"freeBytes": True}, {"version": 2},
    {"version": True}, {"extra": 1}, {"sampledAt": 10 ** 400},
])
def test_consumer_refuses_low_wrong_version_and_malformed_record_values(tmp_path, changes):
    root = private(tmp_path)
    record = {"version": 1, "sampledAt": 10, "freeBytes": monitor.MIN_FREE_BYTES}
    record.update(changes)
    (root / "heartbeat.json").write_text(json.dumps(record))
    with pytest.raises(monitor.MonitorRefusal):
        monitor.check_health(root / "heartbeat.json", root / "cancel", clock=lambda: 10.0)
    assert (root / "cancel").exists()


def test_consumer_refuses_nonregular_health_and_producer_refuses_nonfresh_directory(tmp_path):
    root = private(tmp_path)
    (root / "heartbeat.json").mkdir()
    with pytest.raises(monitor.MonitorRefusal):
        monitor.check_health(root / "heartbeat.json", root / "cancel", clock=lambda: 10)
    assert (root / "cancel").exists()
    assert monitor.monitor(root, telemetry=lambda: pytest.fail("no telemetry for a reused directory")) == 1


def test_producer_stops_when_the_consumer_has_already_cancelled(tmp_path):
    root = private(tmp_path)
    def wait(seconds):
        (root / "cancel").touch()
    assert monitor.monitor(root, telemetry=lambda: monitor.MIN_FREE_BYTES, clock=lambda: 10, wait=wait) == 1
    assert (root / "cancel").exists()


@pytest.mark.parametrize("failure_at", [1, 2, 3, 4, 5, 8])
def test_run_monitor_checks_at_admission_ready_exit_and_success_use_owned_cleanup(tmp_path, monkeypatch, failure_at):
    """Actual run(), all OS/process/telemetry effects replaced by deterministic objects."""
    import io
    from unittest.mock import Mock
    from tools.agent_journey_fixture_test import FakeApp
    root = private(tmp_path)
    bundle = root / "Copy.app" / "Contents"
    (bundle / "MacOS").mkdir(parents=True)
    (bundle / "Resources" / "agent-supplier").mkdir(parents=True)
    (bundle / "Resources" / "agent-supplier" / "codex").touch()
    run_root = root / "run"
    run_root.mkdir()
    args = SimpleNamespace(mode="live", app=bundle / "MacOS" / "app", model="invented",
        context_window=32768, endpoint="unused", turn_timeout=1, deadline=10, release_grace=0,
        process_probe=root / "unused-probe", cancel_file=root / "cancel", memory_heartbeat=root / "heartbeat.json")
    app = FakeApp()
    app.stdin, app.stderr = io.BytesIO(), None
    scope = {"appPid": app.pid, "sdkPid": 9876, "scope": "invented"}
    lines = SimpleNamespace(error=None, scope=lambda: scope, scopes=lambda: [scope], lines=[], thread=Mock())
    observer = SimpleNamespace(exited=False, status=0, invalid=None, sample=lambda: not app.alive, close=Mock())
    anchor = SimpleNamespace(pid=5432, kill=Mock(), wait=Mock(), stdin=io.BytesIO(), stdout=io.BytesIO())
    events, checks = [], []
    launch = Mock(return_value=app)
    monkeypatch.setattr(journey, "sha256", lambda p: json.loads(journey.PIN.read_text())["codex"]["binarySha256"])
    monkeypatch.setattr(journey, "preflight", lambda *a: {"passed": True})
    monkeypatch.setattr(journey.subprocess, "Popen", launch)
    monkeypatch.setattr(journey, "Lines", lambda *a: lines)
    monkeypatch.setattr(journey, "Scope", lambda *a: observer)
    monkeypatch.setattr(journey, "anchor_start", lambda *a: anchor)
    monkeypatch.setattr(journey, "cleanup_anchor", lambda value: events.append(("rescue", value)))
    monkeypatch.setattr(journey, "answer_gate", lambda p, value: events.append(("gate", value)) or True)
    monkeypatch.setattr(journey, "residents", lambda *a: [])
    monkeypatch.setattr(journey, "pid_live", lambda *a: False)
    monkeypatch.setattr(journey, "judge", lambda *a: [])
    (run_root / "report.json").write_text("{}")
    monkeypatch.setattr(journey, "RETAINED", root / "retained")
    monkeypatch.setattr(journey.time, "sleep", lambda *a: None)
    def health(*a):
        checks.append(len(checks) + 1)
        if len(checks) == failure_at:
            raise monitor.MonitorRefusal("health_expired")
        if len(checks) == 4 and failure_at >= 5:
            app.alive = False  # normal simulated app exit, so the next checks are exit/success
    monkeypatch.setattr(journey.memory_monitor, "check_health", health)
    result = journey.run(args, None, run_root)
    assert result["failure"] == "memory_monitor_health_expired" and not result["passed"]
    assert len(checks) == failure_at
    if failure_at == 1:
        launch.assert_not_called()
    elif failure_at == 2:
        assert not app.killed and (root / "retained").exists()
    elif failure_at in (3, 4):
        assert app.killed and ("rescue", anchor) in events
        assert ("gate", b"REFUSE\n" if failure_at == 3 else b"READY\n") in events
        if failure_at == 3:
            assert ("gate", b"READY\n") not in events
    else:
        assert not app.killed
        if failure_at == 8:
            anchor.kill.assert_called_once()  # only the owned anchor's ordinary retirement
        else:
            assert ("rescue", anchor) in events
