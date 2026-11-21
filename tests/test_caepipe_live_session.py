#!/usr/bin/env python3
"""A live run's opening (DEL-17-05): the guest's liveness, asked again a stated number of times, then the guest's own
drive letter for the exchange share, before the first guest step and before anything is staged.

Every guest answer here is synthetic: an injected runner or the fake guest of tests/caepipe_fake_guest.py. No
virtual machine is started and no prlctl is called.
"""

from __future__ import annotations

import dataclasses
import hashlib
import json
import sys
from pathlib import Path
from types import SimpleNamespace

import pytest

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.handoff.caepipe_external import harness  # noqa: E402
from core.handoff.caepipe_external.harness import (  # noqa: E402
    SESSION_LIVENESS_CHECKS,
    SESSION_LIVENESS_WAIT_S,
    HarnessConfig,
    await_guest_alive,
    exchange_share_letter,
    open_live_session,
    parse_net_use,
    run_caepipe_round_trip,
)
from tests.caepipe_fake_guest import TRANSPORT, FakeGuestModel, done  # noqa: E402
from tools.caepipe_correlation import run_pilot  # noqa: E402

SHARE = "invented-exchange"
VER = "\r\nMicrosoft Windows [Version 10.0.19045.5011]\r\n"


def net_use(*rows: str) -> str:
    return "\r\n".join(["New connections will be remembered.", "", "",
                        "Status       Local     Remote                    Network", "",
                        "-" * 79, *rows, "The command completed successfully.", ""])


def config_for(tmp_path: Path, configured: str = "Z:\\invented") -> HarnessConfig:
    root = tmp_path / SHARE
    root.mkdir(parents=True, exist_ok=True)
    (root / ".gitignore").write_text("*\n", encoding="utf-8")
    return HarnessConfig(exchange_root=root, vm_name="invented-vm", guest_executable="C:\\Invented\\tool.exe",
                         guest_exchange_root=configured, guest_work_root="C:\\invented-work",
                         prlctl="/invented/prlctl", acknowledged=True, trial_first_launch_date=None)


def answering(net_use_text: str, calls: list | None = None, *, exit_status: int = 0, stderr: str = ""):
    def runner(argv, **kwargs):
        command = argv[-1]
        if calls is not None:
            calls.append(command)
        if command == "ver":
            return done(argv, 0, VER, "")
        if command == "net use":
            return done(argv, exit_status, net_use_text, stderr)
        raise AssertionError(f"unexpected guest command {command!r}")
    return runner


# --------------------------------------------------------------------------------------------------------------
# Liveness: asked again a stated number of times, then the run stops before anything is staged
# --------------------------------------------------------------------------------------------------------------
def test_liveness_is_asked_again_until_the_guest_answers_and_never_more_than_the_stated_number(tmp_path):
    config = config_for(tmp_path)
    answers = [{"status": "unreachable", "reason": "timeout"}, {"status": "unreachable", "reason": "timeout"},
               {"status": "alive"}]
    sleeps: list[float] = []
    result = await_guest_alive(config, liveness=lambda c: answers.pop(0), sleep=sleeps.append)
    assert result["status"] == "alive" and [c["check"] for c in result["checks"]] == [1, 2, 3]
    assert sleeps == [SESSION_LIVENESS_WAIT_S] * 2

    asked: list[int] = []
    sleeps.clear()
    result = await_guest_alive(config, liveness=lambda c: asked.append(1) or {"status": "unreachable",
                                                                                "reason": "no answer"},
                               sleep=sleeps.append)
    assert result["status"] == "guest_unreachable" and len(asked) == SESSION_LIVENESS_CHECKS
    assert sleeps == [SESSION_LIVENESS_WAIT_S] * (SESSION_LIVENESS_CHECKS - 1)
    assert "nothing was staged" in result["reason"] and "no answer" in result["reason"]


def test_a_guest_that_never_answers_stops_the_run_before_any_share_read_or_staging(tmp_path):
    config = config_for(tmp_path)
    calls: list[str] = []

    def runner(argv, **kwargs):
        calls.append(argv[-1])
        return done(argv, 255, "", TRANSPORT[0])  # a transport answer every time
    session, record = open_live_session(config, runner=runner, sleep=lambda s: None)
    assert session is None and record["status"] == "guest_unreachable"
    assert set(calls) == {"ver"}  # nothing but the liveness call reached the guest
    assert len(calls) == SESSION_LIVENESS_CHECKS * harness.PRLCTL_ATTEMPTS  # each check retries a transport answer
    assert sorted(p.name for p in config.exchange_root.iterdir()) == [".gitignore"]  # nothing staged


# --------------------------------------------------------------------------------------------------------------
# The exchange share by name: the guest's own letter, used in memory, stated in every run record
# --------------------------------------------------------------------------------------------------------------
def test_net_use_lines_are_read_for_the_named_share_only():
    text = net_use(f"OK           Y:        \\\\Mac\\{SHARE}     Parallels Shared Folders",
                   "             Z:        \\\\Mac\\Home              Parallels Shared Folders",
                   f"OK           W:        \\\\Mac\\{SHARE}-other",
                   "OK           V:        \\\\fileserver\\" + SHARE)
    assert parse_net_use(text, SHARE) == [{"status": "OK", "letter": "Y:", "remote": f"\\\\Mac\\{SHARE}"}]
    assert parse_net_use(text, SHARE.upper())[0]["letter"] == "Y:"  # the guest compares share names without case
    assert parse_net_use(text, "home") == [{"status": "", "letter": "Z:", "remote": "\\\\Mac\\Home"}]
    # A long remote name pushes the network column to the next line: the mapping line ends at the share name.
    assert parse_net_use(net_use(f"OK           y:        \\\\Mac\\{SHARE}", "                    Parallels Shared "
                                                                             "Folders"), SHARE)[0]["letter"] == "Y:"


def test_the_share_letter_read_from_the_guest_replaces_the_configured_one_in_memory_only(tmp_path):
    config = config_for(tmp_path)
    file_before = {p.name: p.read_bytes() for p in config.exchange_root.iterdir()}
    calls: list[str] = []
    session, record = open_live_session(
        config, runner=answering(net_use(f"OK           Y:        \\\\Mac\\{SHARE}     Parallels Shared Folders"),
                                 calls), sleep=lambda s: None)
    assert record["status"] == "ready" and calls == ["ver", "net use"]
    assert session.guest_exchange_root == "Y:" and config.guest_exchange_root == "Z:\\invented"
    assert session.exchange_share == {"share_name": SHARE, "remote": f"\\\\Mac\\{SHARE}",
                                      "configured_root": "Z:\\invented", "letter": "Y:", "mapping_status": "OK",
                                      "differs_from_configured": True}
    assert {p.name: p.read_bytes() for p in config.exchange_root.iterdir()} == file_before  # nothing written


@pytest.mark.parametrize("rows, status", [
    ((), "share_unmapped"),
    (("OK           Y:        \\\\Mac\\other-share",), "share_unmapped"),
    ((f"OK           Y:        \\\\Mac\\{SHARE}", f"Unavailable  X:        \\\\Mac\\{SHARE}"), "share_ambiguous"),
])
def test_no_letter_or_more_than_one_for_the_share_stops_the_run_before_anything_is_staged(tmp_path, rows, status):
    config = config_for(tmp_path)
    session, record = open_live_session(config, runner=answering(net_use(*rows)), sleep=lambda s: None)
    assert session is None and record["status"] == status and "nothing was staged" in record["reason"]
    assert sorted(p.name for p in config.exchange_root.iterdir()) == [".gitignore"]


@pytest.mark.parametrize("exit_status, stderr", [(2, ""), (0, "System error 53 has occurred."), (255, TRANSPORT[0])])
def test_an_unreadable_mapping_is_unknown_and_stops_the_run(tmp_path, exit_status, stderr):
    config = config_for(tmp_path)
    text = net_use(f"OK           Y:        \\\\Mac\\{SHARE}")
    session, record = open_live_session(config, runner=answering(text, exit_status=exit_status, stderr=stderr),
                                        sleep=lambda s: None)
    assert session is None and record["status"] == "share_unknown"


def test_the_share_read_is_a_read_retried_only_on_a_transport_answer(tmp_path, monkeypatch):
    monkeypatch.setattr(harness, "_backoff", lambda seconds: None)
    config = config_for(tmp_path)
    replies = [(255, "", TRANSPORT[1]), (0, net_use(f"OK           Y:        \\\\Mac\\{SHARE}"), "")]
    seen: list[str] = []

    def runner(argv, **kwargs):
        seen.append(argv[-1])
        return done(argv, *replies.pop(0))
    assert exchange_share_letter(config, runner=runner)["letter"] == "Y:" and seen == ["net use", "net use"]


def test_a_round_trip_on_the_session_uses_the_guests_letter_and_its_record_states_both(tmp_path, monkeypatch):
    monkeypatch.setattr(harness, "time", SimpleNamespace(monotonic=lambda: 0.0))
    monkeypatch.setattr(harness, "_backoff", lambda seconds: None)
    config = config_for(tmp_path)
    session, _ = open_live_session(config, runner=answering(net_use(f"OK           Y:        \\\\Mac\\{SHARE}")),
                                   sleep=lambda s: None)
    guest = FakeGuestModel(config.exchange_root, "Y:")  # the guest's share is at Y:, not the configured Z:\invented
    mbf = "INVENTED\r\nMBF\r\n"
    record = run_caepipe_round_trip(session, "cx-share", mbf, runner=guest.runner)
    assert record["route"] == "csv_from_batch" and record["input_hash_verified"] is True
    assert record["guest_mbf_sha256"] == hashlib.sha256(mbf.encode("utf-8")).hexdigest()
    assert record["mbf"]["guest_staged_path"].startswith("Y:\\cases\\cx-share\\")
    assert record["configuration_surface"]["exchange_share"]["configured_root"] == "Z:\\invented"
    assert record["configuration_surface"]["exchange_share"]["letter"] == "Y:"
    stored = json.loads((config.exchange_root / "cases" / "cx-share" / "caepipe_out" / "run_record.json").read_text())
    assert stored["configuration_surface"]["exchange_share"]["differs_from_configured"] is True


# --------------------------------------------------------------------------------------------------------------
# The command: every live route opens the session first; a stopped session stages nothing and runs nothing
# --------------------------------------------------------------------------------------------------------------
def test_every_live_route_of_the_command_stops_on_an_unopened_session(tmp_path, monkeypatch, capsys):
    config = config_for(tmp_path)
    monkeypatch.setattr(run_pilot, "resolve_harness_config", lambda environ, require_vm_tools: config)
    monkeypatch.setattr(run_pilot, "check_guest_alive", lambda c: {"status": "unreachable", "reason": "invented"})
    monkeypatch.setattr(harness, "_backoff", lambda seconds: None)
    for name in ("run_cases", "export_case", "run_authored_case", "run_benchmark", "check_no_foreign_instance"):
        monkeypatch.setattr(run_pilot, name, lambda *a, _n=name, **k: pytest.fail(f"{_n} ran"))
    runner = tmp_path / "runner"
    runner.write_text("invented", encoding="utf-8")
    assert run_pilot.main(["--export-only", "--case", "cpc-03-anchored-weight"], environ={}) == 1
    assert run_pilot.main(["--authored", "--case", "cpa-12-guide-gap"], environ={}) == 1
    assert run_pilot.main(["--case", "cpc-03-anchored-weight", "--runner", str(runner)], environ={}) == 1
    out = capsys.readouterr().out
    assert out.count("stopped before the first guest step: guest_unreachable") == 3
    assert sorted(p.name for p in config.exchange_root.iterdir()) == [".gitignore"]


def test_the_command_states_a_letter_that_differs_and_runs_on_it(tmp_path, monkeypatch, capsys):
    config = config_for(tmp_path)
    monkeypatch.setattr(run_pilot, "resolve_harness_config", lambda environ, require_vm_tools: config)
    monkeypatch.setattr(run_pilot, "check_guest_alive", lambda c: {"status": "alive"})
    monkeypatch.setattr(run_pilot, "exchange_share_letter", lambda c: exchange_share_letter(
        c, runner=answering(net_use(f"OK           Y:        \\\\Mac\\{SHARE}"))))
    used = []
    monkeypatch.setattr(run_pilot, "export_case", lambda case, config_: used.append(config_.guest_exchange_root)
                        or True)
    assert run_pilot.main(["--export-only", "--case", "cpc-03-anchored-weight"], environ={}) == 0
    assert used == ["Y:"]
    assert "this run uses Y: (the configuration file is unchanged)" in capsys.readouterr().out


def test_an_offline_run_opens_no_session(tmp_path, monkeypatch):
    config = dataclasses.replace(config_for(tmp_path), prlctl=None)
    monkeypatch.setattr(run_pilot, "resolve_harness_config", lambda environ, require_vm_tools: config)
    monkeypatch.setattr(run_pilot, "open_live_session", lambda *a, **k: pytest.fail("a session was opened"))
    monkeypatch.setattr(run_pilot, "run_cases", lambda *a, **k: {"cpc-03-anchored-weight": True})
    runner = tmp_path / "runner"
    runner.write_text("invented", encoding="utf-8")
    assert run_pilot.main(["--offline", "--case", "cpc-03-anchored-weight", "--runner", str(runner)],
                          environ={}) == 0
