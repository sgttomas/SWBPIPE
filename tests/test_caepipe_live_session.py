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
import os
import subprocess
import sys
from pathlib import Path
from types import SimpleNamespace

import pytest

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.handoff.caepipe_external import harness  # noqa: E402
from core.handoff.caepipe_external.harness import (  # noqa: E402
    CONFIG_NAME,
    HarnessConfig,
    await_guest_alive,
    exchange_share_letter,
    net_use_form,
    open_live_session,
    parse_net_use,
    run_caepipe_round_trip,
)
from tests.caepipe_fake_guest import TRANSPORT, FakeGuestModel, done  # noqa: E402
from tests.test_caepipe_correlation import INVENTED_GUARD, authored_dir  # noqa: E402
from tools.caepipe_correlation import run_pilot  # noqa: E402

SHARE = "invented-exchange"
MANIFEST = ROOT / "validation" / "caepipe_benchmark" / "manifest.json"
NETWORK = "Parallels Shared Folders"
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
    """The stated commitment, in literals: four checks in all, 30 s apart."""
    config = config_for(tmp_path)
    answers = [{"status": "unreachable", "reason": "timeout"}, {"status": "unreachable", "reason": "timeout"},
               {"status": "alive"}]
    sleeps: list[float] = []
    result = await_guest_alive(config, liveness=lambda c: answers.pop(0), sleep=sleeps.append)
    assert result["status"] == "alive" and [c["check"] for c in result["checks"]] == [1, 2, 3]
    assert sleeps == [30.0, 30.0]

    asked: list[int] = []
    sleeps.clear()
    result = await_guest_alive(config, liveness=lambda c: asked.append(1) or {"status": "unreachable",
                                                                                "reason": "no answer"},
                               sleep=sleeps.append)
    assert result["status"] == "guest_unreachable" and len(asked) == 4
    assert sleeps == [30.0, 30.0, 30.0]
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
    assert len(calls) == 4 * harness.PRLCTL_ATTEMPTS  # each check retries a transport answer
    assert sorted(p.name for p in config.exchange_root.iterdir()) == [".gitignore"]  # nothing staged


# --------------------------------------------------------------------------------------------------------------
# The exchange share by name: the guest's own letter, used in memory, stated in every run record
# --------------------------------------------------------------------------------------------------------------
def test_net_use_lines_are_read_for_the_named_share_only():
    text = net_use(f"OK           Y:        \\\\Mac\\{SHARE}     Parallels Shared Folders",
                   "             Z:        \\\\Mac\\Home              Parallels Shared Folders",
                   f"OK           W:        \\\\Mac\\{SHARE}-other",
                   "OK           V:        \\\\fileserver\\" + SHARE)
    assert parse_net_use(text, SHARE) == [{"status": "OK", "letter": "Y:", "remote": f"\\\\Mac\\{SHARE}",
                                           "ambiguous": False}]
    assert parse_net_use(text, SHARE.upper())[0]["letter"] == "Y:"  # the guest compares share names without case
    assert parse_net_use(text, "home") == [{"status": "", "letter": "Z:", "remote": "\\\\Mac\\Home",
                                            "ambiguous": False}]
    # A long remote name pushes the network column to the next line: the mapping line ends at the share name.
    assert parse_net_use(net_use(f"OK           y:        \\\\Mac\\{SHARE}", "                    Parallels Shared "
                                                                             "Folders"), SHARE)[0]["letter"] == "Y:"


@pytest.mark.parametrize("line", [
    # The wanted name followed by two spaces and more text: another share, or a path beneath it, never this share.
    f"OK           X:        \\\\Mac\\{SHARE}  copy     {NETWORK}",
    f"OK           X:        \\\\Mac\\{SHARE}  old\\sub     {NETWORK}",
    f"OK           X:        \\\\Mac\\{SHARE}  \\sub",
    f"OK           X:        \\\\Mac\\{SHARE}\\sub",
    f"OK           X:        \\\\Mac\\{SHARE}\\",
    f"OK           X:        \\\\Mac\\{SHARE}     Another Network",
    f"OK           X:        \\\\Mac\\invented     {NETWORK}",
    f"OK                     \\\\Mac\\{SHARE}     {NETWORK}",
])
def test_a_line_that_is_not_exactly_the_wanted_remote_gives_no_letter(line):
    assert parse_net_use(net_use(line), SHARE) == []


@pytest.mark.parametrize("line, share", [
    (f"OK           Y:        \\\\Mac\\a     {NETWORK}", "a"),                       # a one-character share name
    (f"OK           Y:        \\\\Mac\\{SHARE}-12 {NETWORK}", f"{SHARE}-12"),        # one space before the column
    (f"OK           Y:        \\\\Mac\\my  exchange     {NETWORK}", "my  exchange"),  # two spaces in the name
])
def test_a_valid_line_for_the_wanted_remote_is_read(line, share):
    assert [(item["letter"], item["ambiguous"]) for item in parse_net_use(net_use(line), share)] == [("Y:", False)]


def test_a_line_the_column_layout_leaves_open_fails_closed(tmp_path):
    """The network column after the name on the line and alone on the next line as well: the line could be the share
    with its column, or a share whose name ends in the column's text with the column wrapped. It is not read."""
    text = net_use(f"OK           Y:        \\\\Mac\\{SHARE}     {NETWORK}", f"                    {NETWORK}")
    assert parse_net_use(text, SHARE)[0]["ambiguous"] is True
    session, record = open_live_session(config_for(tmp_path), runner=answering(text), sleep=lambda s: None)
    assert session is None and record["status"] == "share_unknown" and "more than one way" in record["reason"]


@pytest.mark.parametrize("wrapped", ["Microsoft Windows Network", NETWORK, "   Another Network"])
def test_a_row_with_its_network_column_and_a_wrapped_one_after_it_fails_closed(tmp_path, wrapped):
    """A row that carries its network column and is continued on the next line: the wrapped text could be the row's
    provider (the inline text then part of the remote), so the share is not read either way."""
    text = net_use(f"OK           Y:        \\\\Mac\\{SHARE}     {NETWORK}", f"                    {wrapped}")
    assert parse_net_use(text, SHARE)[0]["ambiguous"] is True
    session, record = open_live_session(config_for(tmp_path), runner=answering(text), sleep=lambda s: None)
    assert session is None and record["status"] == "share_unknown"


def test_a_row_ending_at_the_name_is_continued_only_by_the_network_column(tmp_path):
    """Ending at the name, a row may be continued by the network column alone (a long remote wraps it); any other
    continuation leaves the provider open, and the share is not read."""
    wrapped = net_use(f"OK           Y:        \\\\Mac\\{SHARE}", "                    Microsoft Windows Network")
    assert parse_net_use(wrapped, SHARE)[0]["ambiguous"] is True
    # A connection with no drive letter is a row of its own, not a continuation of the share's row.
    deviceless = net_use(f"OK           Y:        \\\\Mac\\{SHARE}",
                         "OK                     \\\\fileserver\\data     Microsoft Windows Network")
    assert [(m["letter"], m["ambiguous"]) for m in parse_net_use(deviceless, SHARE)] == [("Y:", False)]


@pytest.mark.parametrize("spelling, letters", [("Straße", ["Y:"]), ("STRAßE", ["Y:"]), ("Strasse", [])])
def test_the_share_name_is_matched_letter_by_letter_in_its_exact_spelling(spelling, letters):
    """The share is named by the exchange folder `Straße`: the same letters in any case are the share; `Strasse`,
    which folds to the same text, is another name. With the network column on the line as well as without it."""
    row = f"OK           Y:        \\\\Mac\\{spelling}"
    for line in (f"{row}     {NETWORK}", row):
        assert [m["letter"] for m in parse_net_use(net_use(line), "Straße")] == letters


@pytest.mark.parametrize("text", [
    # A mapping line, then the empty listing's last line: the reply contradicts itself.
    f"New connections will be remembered.\r\n\r\nOK           Y:        \\\\Mac\\{SHARE}     {NETWORK}\r\n"
    "There are no entries in the list.\r\n",
    # A mapping line before the column header: outside the table body.
    f"OK           Y:        \\\\Mac\\{SHARE}     {NETWORK}\r\n" + net_use(),
    # The header twice, or a closing line inside the body.
    net_use(f"OK           Y:        \\\\Mac\\{SHARE}", "Status       Local     Remote                    Network"),
    net_use(f"OK           Y:        \\\\Mac\\{SHARE}", "There are no entries in the list."),
])
def test_a_contradictory_listing_is_unknown_and_its_lines_are_not_read(tmp_path, text):
    assert net_use_form(text) is False and parse_net_use(text, SHARE) == []
    session, record = open_live_session(config_for(tmp_path), runner=answering(text), sleep=lambda s: None)
    assert session is None and record["status"] == "share_unknown"


def test_the_share_letter_read_from_the_guest_replaces_the_configured_one_in_memory_only(tmp_path):
    config = config_for(tmp_path)
    (config.exchange_root / CONFIG_NAME).write_text(json.dumps({"guest_exchange_root": "Z:\\invented"}) + "\n",
                                                    encoding="utf-8")
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


@pytest.mark.parametrize("status", ["Unavailable", "Disconnected", "Reconnecting"])
def test_a_mapping_whose_status_is_not_ok_stops_the_run_by_name(tmp_path, status):
    config = config_for(tmp_path)
    session, record = open_live_session(
        config, runner=answering(net_use(f"{status:<13}Y:        \\\\Mac\\{SHARE}     {NETWORK}")),
        sleep=lambda s: None)
    assert session is None and record["status"] == "share_unavailable"
    assert record["exchange_share"]["mapping_status"] == status and status in record["reason"]
    assert sorted(p.name for p in config.exchange_root.iterdir()) == [".gitignore"]


def test_a_blank_status_is_the_shares_ordinary_state(tmp_path):
    """`net use` prints a status for connections of the Windows network provider only; the host's shared folders are
    mapped through another provider, so their Status column is blank. Blank is accepted as OK is; any other status
    stops the run (previous test)."""
    session, record = open_live_session(
        config_for(tmp_path), runner=answering(net_use(f"             Y:        \\\\Mac\\{SHARE}     {NETWORK}")),
        sleep=lambda s: None)
    assert record["status"] == "ready" and session.exchange_share["mapping_status"] == ""


@pytest.mark.parametrize("exit_status, stdout, stderr", [
    (2, None, ""), (0, None, "System error 53 has occurred."), (255, None, TRANSPORT[0]),
    (0, "Please sign in to continue.\r\n", ""), (0, "", ""),
    (0, f"OK           Y:        \\\\Mac\\{SHARE}\r\nThe command completed successfully.\r\n", ""),  # no header
])
def test_an_unreadable_mapping_is_unknown_and_stops_the_run(tmp_path, exit_status, stdout, stderr):
    """A reply that is not a `net use` listing in its English form is unknown, never unmapped."""
    config = config_for(tmp_path)
    text = net_use(f"OK           Y:        \\\\Mac\\{SHARE}") if stdout is None else stdout
    session, record = open_live_session(config, runner=answering(text, exit_status=exit_status, stderr=stderr),
                                        sleep=lambda s: None)
    assert session is None and record["status"] == "share_unknown"


def test_an_empty_listing_in_its_form_is_unmapped(tmp_path):
    text = "There are no entries in the list.\r\n\r\n"
    session, record = open_live_session(config_for(tmp_path), runner=answering(text), sleep=lambda s: None)
    assert session is None and record["status"] == "share_unmapped"


@pytest.mark.parametrize("failure", ["exit", "stderr", "timeout"])
def test_the_share_read_is_not_issued_again_on_an_answer_that_is_not_a_transport_answer(tmp_path, monkeypatch,
                                                                                       failure):
    monkeypatch.setattr(harness, "_backoff", lambda seconds: None)
    seen: list[str] = []

    def runner(argv, **kwargs):
        seen.append(argv[-1])
        if failure == "timeout":
            raise subprocess.TimeoutExpired(argv, kwargs.get("timeout"))
        return done(argv, 2 if failure == "exit" else 0, "", "" if failure == "exit" else "System error 53.")
    assert exchange_share_letter(config_for(tmp_path), runner=runner)["status"] == "share_unknown"
    assert seen == ["net use"]


def test_a_root_that_fails_the_path_allow_list_stops_the_run(tmp_path):
    def share(config):
        return {"status": "mapped", "share_name": SHARE, "remote": f"\\\\Mac\\{SHARE}",
                "configured_root": config.guest_exchange_root, "letter": "Y:", "mapping_status": "OK",
                "root": "Y:\\..\\elsewhere", "differs_from_configured": True}
    session, record = open_live_session(config_for(tmp_path), liveness=lambda c: {"status": "alive"}, share=share,
                                        sleep=lambda s: None)
    assert session is None and record["status"] == "share_root_refused" and "allow-listed" in record["reason"]


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
def command_config(tmp_path: Path, monkeypatch, configured: str = "Z:\\invented") -> HarnessConfig:
    config = dataclasses.replace(config_for(tmp_path, configured), settings_guard=INVENTED_GUARD)
    monkeypatch.setattr(run_pilot, "resolve_harness_config", lambda environ, require_vm_tools: config)
    monkeypatch.setattr(harness, "_backoff", lambda seconds: None)
    return config


def guest(monkeypatch, *, alive: bool, calls: list | None = None, text: str | None = None) -> None:
    """The command's guest: liveness and the share read, each call recorded."""
    calls = [] if calls is None else calls
    monkeypatch.setattr(run_pilot, "check_guest_alive", lambda c: calls.append("ver") or (
        {"status": "alive"} if alive else {"status": "unreachable", "reason": "invented"}))
    answer = answering(text or net_use(f"OK           Y:        \\\\Mac\\{SHARE}     {NETWORK}"))
    monkeypatch.setattr(run_pilot, "exchange_share_letter", lambda c: calls.append("net use") or exchange_share_letter(
        c, runner=answer))


def benchmark_argv(tmp_path: Path, *extra: str) -> list[str]:
    runner = tmp_path / "runner"
    runner.write_text("invented", encoding="utf-8")
    return ["--benchmark", str(MANIFEST), "--case", "cpb-g1-01", "--runner", str(runner),
            "--freeze-file", str(tmp_path / "freeze"), "--exclusive-file", str(tmp_path / "exclusive"), *extra]


def tree(path: Path) -> dict[str, bytes]:
    return {str(p.relative_to(path)): p.read_bytes() if p.is_file() else b"<dir>" for p in sorted(path.rglob("*"))}


def test_every_live_route_of_the_command_stops_on_an_unopened_session(tmp_path, monkeypatch, capsys):
    config = command_config(tmp_path, monkeypatch)
    guest(monkeypatch, alive=False)
    for name in ("run_cases", "export_case", "run_authored_case", "run_benchmark", "check_no_foreign_instance"):
        monkeypatch.setattr(run_pilot, name, lambda *a, _n=name, **k: pytest.fail(f"{_n} ran"))
    runner = tmp_path / "runner"
    runner.write_text("invented", encoding="utf-8")
    directory = authored_dir(config.exchange_root, ("cx-01-invented",))
    before = tree(config.exchange_root)
    assert run_pilot.main(["--export-only", "--case", "cpc-03-anchored-weight"], environ={}) == 1
    assert run_pilot.main(["--authored", "--case", "cpa-12-guide-gap"], environ={}) == 1
    # With --authored-dir the session opens before the foreign-instance check, which would otherwise run first.
    assert run_pilot.main(["--authored", "--authored-dir", str(directory)], environ={}) == 1
    assert run_pilot.main(["--case", "cpc-03-anchored-weight", "--runner", str(runner)], environ={}) == 1
    # The benchmark route (its call is examined here, not behind the pass-through of tests/test_caepipe_benchmark.py).
    assert run_pilot.main(benchmark_argv(tmp_path), environ={}) == 1
    out = capsys.readouterr().out
    assert out.count("stopped before the first guest step: guest_unreachable") == 5
    assert tree(config.exchange_root) == before


def test_the_benchmark_route_opens_the_session_and_runs_on_the_guests_letter(tmp_path, monkeypatch, capsys):
    command_config(tmp_path, monkeypatch)
    calls: list[str] = []
    guest(monkeypatch, alive=True, calls=calls)
    used = []
    monkeypatch.setattr(run_pilot, "run_benchmark", lambda models, config_, **k: used.append(
        (config_.guest_exchange_root, config_.exchange_share["configured_root"])) or 0)
    assert run_pilot.main(benchmark_argv(tmp_path), environ={}) == 0
    assert calls == ["ver", "net use"] and used == [("Y:", "Z:\\invented")]
    assert "this run uses Y: (the configuration file is unchanged)" in capsys.readouterr().out


def test_an_offline_benchmark_opens_no_session(tmp_path, monkeypatch):
    command_config(tmp_path, monkeypatch)
    monkeypatch.setattr(run_pilot, "open_live_session", lambda *a, **k: pytest.fail("a session was opened"))
    monkeypatch.setattr(run_pilot, "check_guest_alive", lambda *a, **k: pytest.fail("the guest was called"))
    monkeypatch.setattr(run_pilot, "run_benchmark", lambda models, config_, **k: 0)
    runner = tmp_path / "runner"
    runner.write_text("invented", encoding="utf-8")
    assert run_pilot.main(["--benchmark", str(MANIFEST), "--case", "cpb-g1-01", "--offline", "--runner", str(runner)],
                          environ={}) == 0


@pytest.mark.parametrize("stop, words, code", [
    ("freeze", "the freeze file is present", 1), ("exclusive", "an exclusive job was still pending", 1),
    ("batch", "--batch 9 is not one of the 1 batches", 2)])
def test_the_benchmark_route_checks_its_stops_and_batch_before_any_guest_call(tmp_path, monkeypatch, capsys, stop,
                                                                             words, code):
    command_config(tmp_path, monkeypatch)
    monkeypatch.setattr(run_pilot, "open_live_session", lambda *a, **k: pytest.fail("a session was opened"))
    monkeypatch.setattr(run_pilot, "check_guest_alive", lambda *a, **k: pytest.fail("the guest was called"))
    monkeypatch.setattr(run_pilot, "run_benchmark", lambda *a, **k: pytest.fail("the benchmark ran"))
    monkeypatch.setattr(run_pilot, "BENCHMARK_EXCLUSIVE_WAIT_S", 0.0)
    extra: tuple[str, ...] = ()
    if stop == "freeze":
        (tmp_path / "freeze").write_text("", encoding="utf-8")
    elif stop == "exclusive":
        (tmp_path / "exclusive").write_text(str(os.getpid()), encoding="ascii")  # a running process holds the host
    else:
        extra = ("--batch", "9")
    assert run_pilot.main(benchmark_argv(tmp_path, *extra), environ={}) == code
    assert words in capsys.readouterr().out


def test_a_freeze_that_arrives_while_an_exclusive_job_is_waited_for_stops_before_any_guest_call(tmp_path,
                                                                                                monkeypatch, capsys):
    """While the command waits on an exclusive job, the job ends and the freeze file appears: both are checked again
    after the wait, so the run stops on the freeze, with no guest call."""
    command_config(tmp_path, monkeypatch)
    monkeypatch.setattr(run_pilot, "open_live_session", lambda *a, **k: pytest.fail("a session was opened"))
    monkeypatch.setattr(run_pilot, "check_guest_alive", lambda *a, **k: pytest.fail("the guest was called"))
    monkeypatch.setattr(run_pilot, "exchange_share_letter", lambda *a, **k: pytest.fail("the guest was called"))
    monkeypatch.setattr(run_pilot, "run_benchmark", lambda *a, **k: pytest.fail("the benchmark ran"))
    exclusive, freeze = tmp_path / "exclusive", tmp_path / "freeze"
    exclusive.write_text(str(os.getpid()), encoding="ascii")  # a running process holds the host
    waits: list[float] = []

    def sleep(seconds):
        waits.append(seconds)
        exclusive.unlink()  # the job ends ...
        freeze.write_text("", encoding="utf-8")  # ... and a freeze is set while the command waits
    monkeypatch.setattr(run_pilot.time, "sleep", sleep)
    assert run_pilot.main(benchmark_argv(tmp_path), environ={}) == 1
    out = capsys.readouterr().out
    assert len(waits) == 1 and "stopped before the first guest step: the freeze file is present" in out


def test_the_command_states_a_letter_that_differs_and_runs_on_it(tmp_path, monkeypatch, capsys):
    config = command_config(tmp_path, monkeypatch)
    stored = config.exchange_root / CONFIG_NAME
    stored.write_text(json.dumps({"vm_name": "invented-vm", "guest_exchange_root": "Z:\\invented"}) + "\n",
                      encoding="utf-8")
    before = stored.read_bytes()
    guest(monkeypatch, alive=True)
    used = []
    monkeypatch.setattr(run_pilot, "export_case", lambda case, config_: used.append(config_.guest_exchange_root)
                        or True)
    assert run_pilot.main(["--export-only", "--case", "cpc-03-anchored-weight"], environ={}) == 0
    assert used == ["Y:"]
    assert "this run uses Y: (the configuration file is unchanged)" in capsys.readouterr().out
    assert stored.read_bytes() == before  # the override is in memory: the file on disk is byte-unchanged


def test_a_letter_equal_to_the_configured_one_is_not_reported_and_the_file_is_unchanged(tmp_path, monkeypatch,
                                                                                        capsys):
    config = command_config(tmp_path, monkeypatch, configured="Y:")
    stored = config.exchange_root / CONFIG_NAME
    stored.write_text(json.dumps({"vm_name": "invented-vm", "guest_exchange_root": "Y:"}) + "\n", encoding="utf-8")
    before = stored.read_bytes()
    guest(monkeypatch, alive=True)
    used = []
    monkeypatch.setattr(run_pilot, "export_case", lambda case, config_: used.append(config_.exchange_share) or True)
    assert run_pilot.main(["--export-only", "--case", "cpc-03-anchored-weight"], environ={}) == 0
    assert used[0]["differs_from_configured"] is False and used[0]["letter"] == "Y:"
    assert "this run uses" not in capsys.readouterr().out
    assert stored.read_bytes() == before  # the configuration file on disk is never written


def test_an_offline_run_opens_no_session(tmp_path, monkeypatch):
    config = dataclasses.replace(config_for(tmp_path), prlctl=None)
    monkeypatch.setattr(run_pilot, "resolve_harness_config", lambda environ, require_vm_tools: config)
    monkeypatch.setattr(run_pilot, "open_live_session", lambda *a, **k: pytest.fail("a session was opened"))
    monkeypatch.setattr(run_pilot, "run_cases", lambda *a, **k: {"cpc-03-anchored-weight": True})
    runner = tmp_path / "runner"
    runner.write_text("invented", encoding="utf-8")
    assert run_pilot.main(["--offline", "--case", "cpc-03-anchored-weight", "--runner", str(runner)],
                          environ={}) == 0
