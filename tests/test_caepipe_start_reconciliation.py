#!/usr/bin/env python3
"""The run phase of the CAEPIPE round trip (DEL-17-05): one attempt per token, reconciled and never repeated.

Every case runs against the fake guest of tests/caepipe_fake_guest.py: files with exclusive create, the guest
script's attempt as resumable steps, processes with creation times, named jobs with inherited membership, and lost
replies. Nothing here is a capture or CAEPIPE output; every case is synthetic.
"""

from __future__ import annotations

import dataclasses
import hashlib
import json
import re
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
    HarnessConfig,
    SettingsGuard,
    StartUnresolved,
    check_guest_alive,
    run_caepipe_round_trip,
)
from tests.caepipe_fake_guest import (  # noqa: E402
    RUN_PHASE_KINDS,
    SIGN_IN_TEXT,
    TRANSPORT,
    FakeGuestModel,
    Program,
    classify,
    done,
)
from tools.caepipe_correlation import run_pilot  # noqa: E402

MBF_TEXT = "INVENTED\r\nMBF\r\n"
MBF_SHA = hashlib.sha256(MBF_TEXT.encode("utf-8")).hexdigest()
CASE = "cx-s"
KEY = "HKEY_CURRENT_USER\\Software\\Invented Vendor\\Tool\\1.0"
GUARD = SettingsGuard(key="HKCU\\Software\\Invented Vendor\\Tool\\1.0", recent_files_subkey="Recent list",
                      restore_scope="key")


@pytest.fixture
def clock(monkeypatch):
    """A fake monotonic clock: every guest call takes one second, and every pause its own length."""
    state = {"t": 0.0}
    monkeypatch.setattr(harness, "time", SimpleNamespace(monotonic=lambda: state["t"]))
    monkeypatch.setattr(harness, "_backoff", lambda seconds: state.__setitem__("t", state["t"] + seconds))
    return state


def config_for(tmp_path: Path, guard: SettingsGuard | None = None) -> HarnessConfig:
    root = tmp_path / "exchange"
    root.mkdir(parents=True, exist_ok=True)
    (root / ".gitignore").write_text("*\n", encoding="utf-8")
    return HarnessConfig(exchange_root=root, vm_name="invented-vm", guest_executable="C:\\Invented\\tool.exe",
                         guest_exchange_root="Z:\\invented", guest_work_root="C:\\invented-work",
                         prlctl="/invented/prlctl", acknowledged=True, trial_first_launch_date=None,
                         settings_guard=guard)


def guest_for(config: HarnessConfig, clock_state=None) -> FakeGuestModel:
    guest = FakeGuestModel(config.exchange_root)
    if clock_state is not None:
        runner = guest.runner

        def ticking(argv, **kwargs):
            clock_state["t"] += 1.0
            return runner(argv, **kwargs)
        guest.runner = ticking
    return guest


class Registry:
    """An invented saved-settings key for guarded runs: `reg query /s` before and after, tasklist, and every write
    recorded (none may be issued while the start is unresolved)."""

    def __init__(self, guest: FakeGuestModel, *, change_on_launch: bool = True):
        self.values = {(KEY + "\\Recent list", "1"): ("REG_BINARY", "01")}
        self.writes: list[str] = []
        self.guest = guest
        self.change_on_launch = change_on_launch
        self.changed = False
        guest.extra = self.answer

    def answer(self, argv, command, kind):
        if self.change_on_launch and self.guest.launches and not self.changed:
            self.changed = True
            self.values[(KEY + "\\Recent list", "1")] = ("REG_BINARY", "02")  # the program changed its settings
        if kind == "reg_query":
            lines = ["", KEY + "\\Recent list"] + [f"    {name}    {t}    {d}" for (k, name), (t, d) in
                                                  sorted(self.values.items())] + [""]
            return done(argv, 0, "\r\n".join(lines), "")
        if kind in ("reg_write", "literal"):
            self.writes.append(command)
            add = re.fullmatch(r'reg add "([^"]+)" /v "([^"]+)" /t (REG_\w+) /d "([^"]*)" /f', command)
            if add:
                self.values[(add[1], add[2])] = (add[3], add[4])
            return done(argv, 0, "The operation completed successfully.\r\n", "")
        if kind == "tasklist":
            return done(argv, 0, '"other.exe","9999","Console","2","61,744 K"\r\n', "")  # no tool process listed
        return None


def run(config, guest, **kwargs):
    return run_caepipe_round_trip(config, CASE, MBF_TEXT, runner=guest.runner, **kwargs)


def assert_run_phase_only(guest: FakeGuestModel) -> None:
    """No guest command but the attempt, the fence and read-only queries: nothing prepares, stages or copies."""
    assert set(guest.kinds) <= RUN_PHASE_KINDS, sorted(set(guest.kinds) - RUN_PHASE_KINDS)


# --------------------------------------------------------------------------------------------------------------
# Section 3: quiescence
# --------------------------------------------------------------------------------------------------------------
def test_a_claim_won_before_the_fence_is_waited_for_and_completes_with_exactly_one_launch(tmp_path, clock):
    config = config_for(tmp_path)
    guest = guest_for(config, clock)
    # The script reaches its work folder and pauses; the reply is lost. The host reads no claim and fences, but the
    # script claims first; the fence fails (exists); the host waits; the script resumes, makes its job and launches.
    guest.plan = {1: {"run": "work", "reply": "transport"}}
    guest.hooks[("fence", 1)] = lambda: guest.attempts[0].advance("claimed")
    guest.hooks[("cim", 3)] = lambda: guest.attempts[0].advance()
    record = run(config, guest)
    assert record["route"] == "csv_from_batch" and record["start"]["state"] == "completed"
    assert guest.launches == [record["attempt_token"]] and guest.kinds.count("attempt") == 1
    fence = next(step for step in record["steps"] if step["name"].startswith("fence"))
    assert fence["answer"] == "exists"
    assert guest.kinds[:6] == ["attempt", "claim_read", "fence", "claim_read", "cim", "cim"]
    assert_run_phase_only(guest)


def test_a_child_in_the_job_keeps_the_attempt_from_quiescence_even_without_a_persisted_pid(tmp_path, clock):
    config = config_for(tmp_path)
    guest = guest_for(config, clock)
    guest.plan = {1: {"kill_at": "launched", "program_lives": True, "reply": "transport"}}
    guest.hooks[("cim", 1)] = lambda: [guest.files.pop(path) for path in list(guest.files)
                                       if path.endswith("owned_pids.txt")]
    with pytest.raises(StartUnresolved) as raised:
        run(config, guest)
    record = raised.value.record
    assert record["route"] == "start_unresolved" and record["start"]["state"] == "unresolved"
    assert "job member" in record["start"]["reason"]
    assert record["outputs_collected"] == [] and "chunk" not in guest.kinds  # nothing collected
    assert guest.kinds.count("attempt") == 1  # never redispatched
    assert clock["t"] <= 600 + harness.RECONCILE_DEADLINE_S + 60


def test_pid_reuse_after_the_launcher_ends_reads_as_ended_by_creation_time(tmp_path, clock):
    config = config_for(tmp_path)
    guest = guest_for(config, clock)
    guest.reuse_launcher_pid = True
    guest.plan = {1: {"reply": "transport"}}
    record = run(config, guest)
    claim_pid = int(record["start"]["claim"]["pid"])
    assert guest.processes[claim_pid].alive  # another process holds the launcher's pid
    assert record["start"]["state"] == "completed" and record["route"] == "csv_from_batch"


def test_an_unavailable_process_query_is_unresolved_never_ended(tmp_path, clock):
    config = config_for(tmp_path)
    guest = guest_for(config, clock)
    guest.cim_unavailable = True
    with pytest.raises(StartUnresolved) as raised:
        run(config, guest)
    assert "launcher" in raised.value.record["start"]["reason"]
    assert "job" not in guest.kinds and "finished_read" not in guest.kinds


def test_a_job_held_open_by_a_lingering_handle_is_not_quiescent_then_unknown_at_the_deadline(tmp_path, clock):
    config = config_for(tmp_path)
    guest = guest_for(config, clock)
    guest.hooks[("job", 1)] = lambda: guest.jobs.__setitem__(f"swbpipe-{guest.attempts[0].token}", 1)
    with pytest.raises(StartUnresolved) as raised:
        run(config, guest)
    job_answers = [step["answer"] for step in raised.value.record["steps"] if step["name"].startswith("job_query")]
    assert job_answers and set(job_answers) == {"present active=0"}


def test_one_deadline_bounds_every_nested_read_and_no_read_restarts_it(tmp_path, clock):
    config = config_for(tmp_path)
    guest = guest_for(config, clock)
    guest.plan = {1: {"run": "launched", "reply": "transport"}}  # the program runs on and never finishes
    guest.programs[1] = Program(lingers=True)
    timeouts = []
    runner = guest.runner
    guest.runner = lambda argv, **kwargs: timeouts.append((clock["t"], kwargs["timeout"])) or runner(argv, **kwargs)
    with pytest.raises(StartUnresolved):
        run(config, guest)
    reply_at = timeouts[0][0] + 1.0
    deadline = reply_at + harness.RECONCILE_DEADLINE_S
    reads = timeouts[1:]
    assert reads and all(at + timeout <= deadline + 1e-9 for at, timeout in reads)
    assert all(timeout <= harness.RECONCILE_CALL_TIMEOUT_S for _, timeout in reads)
    assert reads[-1][0] < deadline


# --------------------------------------------------------------------------------------------------------------
# Section 9: controlled cases
# --------------------------------------------------------------------------------------------------------------
def test_a_failure_before_dispatch_is_fenced_as_not_started_and_redispatched_once(tmp_path, clock):
    config = config_for(tmp_path)
    guest = guest_for(config, clock)
    guest.plan = {1: {"reply": "undelivered"}}
    record = run(config, guest)
    assert [entry["disposition"] for entry in record["attempts"]] == ["not_started", "completed"]
    first, second = (entry["attempt_token"] for entry in record["attempts"])
    assert first != second and record["attempt_token"] == second
    assert guest.files[f"C:\\invented-work\\{CASE}\\{first}.claim"].decode().startswith(
        f"swbpipe-claim v1\r\ntoken={first}\r\nowner=host\r\n")
    assert guest.launches == [second] and guest.kinds.count("attempt") == 2
    assert record["route"] == "csv_from_batch"


def test_a_lost_reply_after_the_effect_is_collected_read_only_and_never_relaunched(tmp_path, clock):
    config = config_for(tmp_path)
    guest = guest_for(config, clock)
    guest.plan = {1: {"reply": "transport"}}
    record = run(config, guest)
    token = record["attempt_token"]
    assert guest.kinds.count("attempt") == 1 and guest.launches == [token]
    out = config.exchange_root / "cases" / CASE / "caepipe_out"
    assert sorted(path.name for path in (out / token).iterdir()) == [
        "guest_log.json", "guest_stderr.txt", "guest_stdout.txt", "model.csv", "model.mod", "model.res"]
    assert (out / token / "model.mod").read_bytes() == b"INVENTED SOLVED MODEL\r\n"
    assert record["observed_csv_path"]["host"] == f"caepipe_out/{token}/model.csv"
    assert record["outputs_sha256"]["model.res"] == hashlib.sha256(b"RES").hexdigest()
    assert not any(path.is_file() and path.name.startswith("model") for path in out.iterdir())  # nothing flat
    dispatch = next(step for step in record["steps"] if step["name"] == "attempt_1")
    assert dispatch["attempt_log"] == [{"attempt": 1, "exit_status": 255, "transport_line": TRANSPORT[1]}]
    # The binding the pilot reads still holds under the caepipe_out/ prefix.
    paths = harness.case_paths(config, CASE)
    csv_path, route, reason = run_pilot.bound_csv(config.exchange_root.resolve(), paths, record, MBF_SHA)
    assert csv_path == (out / token / "model.csv").resolve() and route == "csv_from_batch" and reason == ""
    assert_run_phase_only(guest)


def test_partial_execution_is_start_partial_and_never_retried(tmp_path, clock):
    config = config_for(tmp_path)
    guest = guest_for(config, clock)
    guest.plan = {1: {"kill_at": "launched", "reply": "transport"}}
    record = run(config, guest)
    assert record["route"] == "start_partial" and record["start"]["state"] == "partial"
    assert guest.kinds.count("attempt") == 1 and len(guest.launches) == 1
    assert record["outputs_collected"] == [] and "chunk" not in guest.kinds


def test_delayed_completion_within_the_deadline_is_completed_and_past_it_unknown(tmp_path, clock):
    config = config_for(tmp_path)
    guest = guest_for(config, clock)
    guest.plan = {1: {"run": "launched", "reply": "transport", "resume": ("cim", 4)}}
    record = run(config, guest)
    assert record["start"]["state"] == "completed" and record["route"] == "csv_from_batch"

    late = config_for(tmp_path / "late")
    guest = guest_for(late, clock)
    guest.plan = {1: {"run": "launched", "reply": "transport", "resume": ("cim", 10_000)}}
    with pytest.raises(StartUnresolved) as raised:
        run(late, guest)
    assert raised.value.record["route"] == "start_unresolved"
    assert guest.kinds.count("attempt") == 1


def test_an_exhausted_claim_read_is_unknown_and_makes_no_restore_write(tmp_path, clock):
    config = config_for(tmp_path, GUARD)
    guest = guest_for(config, clock)
    registry = Registry(guest)
    guest.plan = {1: {"reply": "transport"}}
    for number in range(1, 500):
        guest.answers[("claim_read", number)] = (1, "", "The process cannot access the file because it is being "
                                                        "used by another process.")
    with pytest.raises(StartUnresolved) as raised:
        run(config, guest)
    assert raised.value.settings_guard["status"] == "start_unresolved"
    assert "restoration inhibited" in raised.value.settings_guard["reason"]
    assert registry.writes == [] and "fence" not in guest.kinds  # a sharing violation is never "absent"


def test_an_unknown_start_with_empty_process_lists_and_changed_settings_makes_zero_restore_writes(tmp_path, clock):
    config = config_for(tmp_path, GUARD)
    guest = guest_for(config, clock)
    registry = Registry(guest)
    guest.plan = {1: {"run": "launched", "reply": "transport"}}
    guest.programs[1] = Program(lingers=True)
    with pytest.raises(StartUnresolved) as raised:
        run(config, guest)
    guard = raised.value.settings_guard
    assert guard["status"] == "start_unresolved" and guard["changed_values"]  # the settings did change
    assert registry.writes == [] and not any(kind in ("reg_write", "literal") for kind in guest.kinds)
    # The record and the guard result are written; the record names the route the wrapper stops on.
    out = config.exchange_root / "cases" / CASE / "caepipe_out"
    assert json.loads((out / "run_record.json").read_text())["route"] == "start_unresolved"
    assert json.loads((out / "settings_guard.json").read_text())["status"] == "start_unresolved"


def test_a_cancellation_while_the_start_is_unresolved_makes_no_restore_write(tmp_path, clock):
    config = config_for(tmp_path, GUARD)
    guest = guest_for(config, clock)
    registry = Registry(guest)
    guest.plan = {1: {"reply": "transport"}}

    def cancel():
        raise KeyboardInterrupt("invented cancellation")
    guest.hooks[("cim", 1)] = cancel
    with pytest.raises(KeyboardInterrupt) as raised:
        run(config, guest)
    assert raised.value.settings_guard["status"] == "start_unresolved"
    assert getattr(raised.value, "start_unresolved", False) is True
    assert registry.writes == []


def test_a_resolved_partial_start_lets_the_guard_restore_as_before(tmp_path, clock):
    config = config_for(tmp_path, GUARD)
    guest = guest_for(config, clock)
    registry = Registry(guest)
    guest.plan = {1: {"kill_at": "launched", "reply": "transport"}}
    record = run(config, guest)
    assert record["route"] == "start_partial"
    assert record["settings_guard"]["status"] == "settings_restored" and registry.writes


def test_no_guest_command_but_the_attempt_and_read_only_queries_is_issued_in_the_run_phase(tmp_path, clock):
    plans = [{}, {1: {"reply": "transport"}}, {1: {"reply": "undelivered"}},
             {1: {"kill_at": "launched", "reply": "transport"}}, {1: {"reply": "timeout"}}]
    for number, plan in enumerate(plans):
        config = config_for(tmp_path / str(number), GUARD)
        guest = guest_for(config, clock)
        Registry(guest, change_on_launch=False)
        guest.plan = plan
        run(config, guest)
        assert_run_phase_only(guest)
        attempts = [command for command in guest.commands if classify(command) == "attempt"]
        assert all(" -NoGuiFallback" not in command for command in attempts)  # gui_fallback defaults on
        assert not any(command.startswith(("copy", "del", "(if", "mkdir")) for command in guest.commands)


def test_a_repeated_chunk_read_changes_nothing(tmp_path, clock):
    config = config_for(tmp_path)
    guest = guest_for(config, clock)
    guest.lost |= {("chunk", 1), ("chunk", 2)}
    guest.answers[("chunk", 4)] = (0, SIGN_IN_TEXT, "")
    before: list[dict] = []
    guest.hooks[("chunk", 1)] = lambda: before.append(dict(guest.files))  # the guest's files as collection begins
    record = run(config, guest)
    token = record["attempt_token"]
    out = config.exchange_root / "cases" / CASE / "caepipe_out" / token
    assert (out / "model.csv").read_bytes() == b"Invented,print\r\n"
    assert before and guest.files == before[0]  # every read, the repeated ones included, changed nothing
    assert guest.kinds.count("chunk") == 3 + 3
    collect = [step for step in record["steps"] if step["name"].startswith("collect_")]
    assert [step.get("answer") for step in collect][:4] == ["unresolved", "unresolved", "chunk", "unresolved"]


def test_two_not_started_in_a_row_is_not_started_distinct_from_partial(tmp_path, clock):
    config = config_for(tmp_path)
    guest = guest_for(config, clock)
    guest.plan = {1: {"reply": "undelivered"}, 2: {"reply": "undelivered"}}
    record = run(config, guest)
    assert record["route"] == "not_started" and record["start"]["state"] == "not_started"
    assert guest.kinds.count("attempt") == 2 and guest.launches == []
    assert [entry["disposition"] for entry in record["attempts"]] == ["not_started", "not_started"]


def test_a_script_that_cannot_run_from_the_share_never_claims_and_is_fenced_not_staged(tmp_path, clock):
    config = config_for(tmp_path)
    guest = guest_for(config, clock)
    runner = guest.runner

    def no_script(argv, **kwargs):
        if classify(argv[6]) == "attempt":
            guest.kinds.append("attempt")
            return done(argv, 1, "", "The argument is not recognized as the name of a script file.")
        return runner(argv, **kwargs)
    guest.runner = no_script
    record = run(config, guest)
    assert record["route"] == "not_started" and guest.launches == []
    assert_run_phase_only(guest)


def test_input_mismatch_is_a_terminal_report_and_nothing_is_collected(tmp_path, clock):
    config = config_for(tmp_path)
    guest = guest_for(config, clock)
    guest.corrupt_copy = True
    record = run(config, guest)
    assert record["route"] == "input_hash_mismatch" and record["input_hash_verified"] is False
    assert guest.launches == [] and "chunk" not in guest.kinds


def test_the_gui_fallback_flag_reaches_the_attempt_only_when_refused(tmp_path, clock):
    config = config_for(tmp_path)
    guest = guest_for(config, clock)
    run(config, guest, gui_fallback=False)
    attempt = next(command for command in guest.commands if classify(command) == "attempt")
    assert attempt.endswith(" -NoGuiFallback")
    assert f"-ExpectedMbfSha256 {MBF_SHA} -WorkRoot \"C:\\invented-work\\{CASE}\"" in attempt
    assert f'-Mbf "Z:\\invented\\cases\\{CASE}\\caepipe_in\\model.mbf"' in attempt
    sha = hashlib.sha256(harness.GUEST_SCRIPT.read_bytes()).hexdigest()
    assert f'-File "Z:\\invented\\_harness\\{sha}\\Invoke-CaepipeRoundTrip.ps1"' in attempt


# --------------------------------------------------------------------------------------------------------------
# The guest script: every launch needs a won claim; the export and the unclaimed solve refuse
# --------------------------------------------------------------------------------------------------------------
SCRIPT = harness.GUEST_SCRIPT.read_text(encoding="utf-8")
READ_SCRIPT = harness.READ_SCRIPT.read_text(encoding="utf-8")
# The echo the installed read script prints first, which an injected answer in form carries.
ECHO = f"SWBPIPE-READ v1 script={harness.READ_SCRIPT_SHA256} data=none\r\n"


def test_the_export_and_an_unclaimed_solve_refuse_before_anything_can_start():
    export_refusal = SCRIPT.index("if ($ExportMod) {")
    claim_refusal = SCRIPT.index("if (-not $AttemptToken) {")
    claim = SCRIPT.index("New-ExclusiveFile $claimPath")
    job = SCRIPT.index("[CaepipeJob]::CreateAndJoin(")
    launches = [index for index in range(len(SCRIPT)) if SCRIPT.startswith("Start-Process", index)]
    assert export_refusal < claim_refusal < claim < job
    refusal = SCRIPT[export_refusal:claim_refusal]
    assert 'status = "export_held"' in refusal and "exit 6" in refusal
    assert 'status = "claim_required"' in SCRIPT[claim_refusal:claim]
    # Every launch outside the held export function comes after the claim and the job: the program's batch run,
    # its GUI fallback, and the controlled exercise's harmless process.
    start = SCRIPT.index("function Export-SolvedModel {")
    held = (start, SCRIPT.index("\n}\n", start))
    live = [index for index in launches if not held[0] < index < held[1]]
    assert len(live) == 3 and all(index > job for index in live)
    assert "Export-SolvedModel" not in SCRIPT[held[1]:]  # the held export is never called


def test_the_script_claims_by_exclusive_create_and_never_requests_breakaway():
    assert "[IO.FileMode]::CreateNew, [IO.FileAccess]::Write, [IO.FileShare]::None" in SCRIPT
    assert "BREAKAWAY" not in SCRIPT.upper().replace("NO BREAKAWAY", "")
    for marker in ('"swbpipe-claim v1", "token=$AttemptToken", "owner=script", "pid=$PID"',
                   '"swbpipe-job v1", "token=$AttemptToken"', '"swbpipe-finished v1", "token=$AttemptToken"'):
        assert marker in SCRIPT
    assert "$WorkRoot + '\\' + $prefix + $AttemptToken + '.claim'" in SCRIPT  # the host's join (prefix: exercise)


# --------------------------------------------------------------------------------------------------------------
# The script's content-addressed, add-only copy
# --------------------------------------------------------------------------------------------------------------
def test_the_guest_script_is_added_once_verified_after_and_never_overwritten(tmp_path, clock):
    config = config_for(tmp_path)
    harness_dir = config.exchange_root / "_harness"
    harness_dir.mkdir()
    old = harness_dir / "Invoke-CaepipeRoundTrip.ps1"
    old.write_bytes(b"an earlier sitting's copy\r\n")
    run(config, guest_for(config, clock))
    sha = hashlib.sha256(harness.GUEST_SCRIPT.read_bytes()).hexdigest()
    copy = harness_dir / sha / "Invoke-CaepipeRoundTrip.ps1"
    assert copy.read_bytes() == harness.GUEST_SCRIPT.read_bytes()
    assert old.read_bytes() == b"an earlier sitting's copy\r\n"
    run(config, guest_for(config, clock))  # present and identical: verified, not rewritten
    copy.write_bytes(b"tampered")
    guest = guest_for(config, clock)
    with pytest.raises(harness.ContainmentError, match="never overwritten"):
        run(config, guest)
    assert copy.read_bytes() == b"tampered" and guest.commands == []


# --------------------------------------------------------------------------------------------------------------
# The transport predicate and attempt_log
# --------------------------------------------------------------------------------------------------------------
# The client's two lines, written out here independently of the harness's constant.
RESULT_LINE = "PrlJob_GetResult: Invalid argument. An invalid argument was passed."
RETCODE_LINE = "PrlJob_GetRetCode: Invalid argument. An invalid argument was passed."


def test_the_transport_lines_are_the_clients_whole_lines():
    assert harness.TRANSPORT_LINES == (RESULT_LINE, RETCODE_LINE)


@pytest.mark.parametrize("line", [RESULT_LINE, RETCODE_LINE])
@pytest.mark.parametrize("stream", ["stdout", "stderr"])
@pytest.mark.parametrize("space", [("", ""), ("\r\n", "\r\n"), ("  ", " \t"), ("\n\n", "\r\n\r\n")])
def test_each_exact_line_with_exit_255_is_a_transport_answer_on_either_stream(line, stream, space):
    text = space[0] + line + space[1]
    answer = (255, text, "") if stream == "stdout" else (255, "", text)
    assert harness.transport_line(*answer) == line


@pytest.mark.parametrize("line, other", [(RESULT_LINE, RETCODE_LINE), (RETCODE_LINE, RESULT_LINE)])
def test_nothing_but_an_exact_line_with_exit_255_is_a_transport_answer(line, other):
    prefix = line.split(":", 1)[0]
    not_transport = [
        (255, "", prefix), (255, prefix, ""), (255, "", prefix + ":"),                  # the bare prefix
        (255, "", "Failed to exec: " + line), (255, "", line + " Retrying."),           # text before or after
        (255, "", "x" + line), (255, "", line + "x"),
        (255, "", line + "\r\n" + other), (255, line, other),                          # the two lines together
        (255, "", line + "\r\nAnother line."), (255, "Another line.", line),           # another non-blank line
        (0, "", line), (1, "", line), (0, line, ""), (1, line, ""),                     # an exact line, exit 0 or 1
        (255, "", line.lower()), (255, "", line[:-1]),                                 # not the same line
    ]
    for answer in not_transport:
        assert harness.transport_line(*answer) is None, answer


def test_a_read_retries_only_an_exact_transport_answer_and_logs_every_attempt(tmp_path, monkeypatch):
    config = config_for(tmp_path)
    monkeypatch.setattr(harness, "_backoff", lambda seconds: None)
    answers = [(255, "", TRANSPORT[1]), (255, "", "Failed to exec: PrlJob_invented"), (0, "never reached", "")]
    calls = []

    def runner(argv, **kwargs):
        calls.append(argv)
        return done(argv, *answers[len(calls) - 1])
    steps: list = []
    record = harness._guest_read(config, runner, 20.0, steps, "invented_read", "tasklist /FO CSV /NH")
    assert len(calls) == 2 and record["exit_status"] == 255
    assert record["attempt_log"] == [{"attempt": 1, "exit_status": 255, "transport_line": TRANSPORT[1]},
                                     {"attempt": 2, "exit_status": 255}]


# --------------------------------------------------------------------------------------------------------------
# Answer forms: a sign-in or licence text arriving with exit 0 is never an answer, on any read
# --------------------------------------------------------------------------------------------------------------
def test_liveness_needs_the_exact_windows_version_line(tmp_path):
    config = config_for(tmp_path)
    for answer, status in [((0, "\r\nMicrosoft Windows [Version 10.0.19045.5011]\r\n", ""), "alive"),
                           ((0, SIGN_IN_TEXT, ""), "unreachable"),
                           ((0, "\r\nMicrosoft Windows [Version 10.0.19045.5011]\r\n", "warning"), "unreachable"),
                           ((0, "Microsoft Windows [Version 10.0.19045.5011]\r\n" + SIGN_IN_TEXT, ""), "unreachable"),
                           ((0, "Microsoft Windows [Version ten]", ""), "unreachable")]:
        result = check_guest_alive(config, runner=lambda argv, **k: done(argv, *answer))
        assert result["status"] == status, answer


@pytest.mark.parametrize("kind", ["claim_read", "cim", "job", "finished_read"])
def test_a_sign_in_text_with_exit_0_on_a_reconciling_read_is_unresolved_never_an_answer(tmp_path, clock, kind):
    config = config_for(tmp_path, GUARD)
    guest = guest_for(config, clock)
    registry = Registry(guest)
    guest.plan = {1: {"reply": "transport"}}
    for number in range(1, 500):
        guest.answers[(kind, number)] = (0, SIGN_IN_TEXT, "")
    with pytest.raises(StartUnresolved) as raised:
        run(config, guest)
    assert raised.value.record["route"] == "start_unresolved" and registry.writes == []
    if kind == "claim_read":
        assert "fence" not in guest.kinds  # text that is not a claim is never "absent" either


def test_a_sign_in_text_once_on_each_reconciling_read_is_read_again(tmp_path, clock):
    config = config_for(tmp_path)
    guest = guest_for(config, clock)
    guest.plan = {1: {"reply": "transport"}}
    for kind in ("claim_read", "cim", "job", "finished_read", "chunk"):
        guest.answers[(kind, 1)] = (0, SIGN_IN_TEXT, "")
    record = run(config, guest)
    assert record["route"] == "csv_from_batch"
    answers = {step["name"]: step.get("answer") for step in record["steps"]}
    assert answers["launcher_query_1"] == "unresolved" and answers["job_query_1"] == "unresolved"


def test_a_sign_in_text_answering_the_fence_is_recorded_and_the_claim_read_decides(tmp_path, clock):
    config = config_for(tmp_path)
    guest = guest_for(config, clock)
    guest.plan = {1: {"reply": "undelivered"}}
    guest.answers[("fence", 1)] = (0, SIGN_IN_TEXT, "")  # the fence took effect; its answer is out of form
    record = run(config, guest)
    fence = next(step for step in record["steps"] if step["name"] == "fence_1")
    assert fence["answer"] == "unresolved"
    assert record["attempts"][0]["disposition"] == "not_started"  # read from the claim, not from the answer


def test_a_sign_in_text_on_every_chunk_read_fails_collection_without_admitting_anything(tmp_path, clock):
    config = config_for(tmp_path)
    guest = guest_for(config, clock)
    for number in range(1, 500):
        guest.answers[("chunk", number)] = (0, SIGN_IN_TEXT, "")
    record = run(config, guest)
    assert record["route"] == "collection_failed" and record["observed_csv_path"]["host"] is None
    assert record["outputs_collected"] == []


def test_a_sign_in_text_answering_the_attempt_is_not_a_report_and_reconciliation_decides(tmp_path, clock):
    config = config_for(tmp_path)
    guest = guest_for(config, clock)
    runner = guest.runner

    def answer(argv, **kwargs):
        result = runner(argv, **kwargs)
        return done(argv, 0, SIGN_IN_TEXT, "") if classify(argv[6]) == "attempt" else result
    guest.runner = answer
    record = run(config, guest)
    assert record["route"] == "csv_from_batch" and record["guest_log"]["attempt_token"] == record["attempt_token"]


@pytest.mark.parametrize("read", ["tasklist", "reg_query", "owned_pids"])
def test_a_sign_in_text_with_exit_0_on_a_guard_read_is_never_its_answer(tmp_path, clock, read):
    config = config_for(tmp_path, GUARD)
    guest = guest_for(config, clock)
    registry = Registry(guest)
    for number in range(1, 50):
        guest.answers[(read, number)] = (0, SIGN_IN_TEXT, "")
    record = run(config, guest)
    status = record["settings_guard"]["status"]
    assert status in {"tasklist": ("precondition_unknown",), "reg_query": ("settings_check_failed",),
                      "owned_pids": ("owned_process_state_unknown",)}[read], status
    assert registry.writes == []


def test_a_write_answered_with_a_sign_in_text_and_exit_0_is_unknown_not_completed(tmp_path):
    config = config_for(tmp_path, GUARD)
    steps: list = []
    with pytest.raises(harness.RestoreWriteFailed) as raised:
        harness._guest_write(config, lambda argv, **k: done(argv, 0, SIGN_IN_TEXT, ""), 20.0, steps, "w",
                             'reg add "HKEY_CURRENT_USER\\Software\\Invented Vendor\\Tool\\1.0" /f')
    assert raised.value.outcome == "unknown"
    # The exact success form completes; text around it does not.
    record = harness._guest_write(config, lambda argv, **k: done(argv, 0, "The operation completed successfully.\r\n",
                                                                 ""), 20.0, steps, "w", "reg add x")
    assert record["outcome"] == "completed"
    with pytest.raises(harness.RestoreWriteFailed):
        harness._guest_write(config, lambda argv, **k: done(argv, 0, SIGN_IN_TEXT + "\r\nThe operation completed "
                                                                                     "successfully.", ""),
                             20.0, steps, "w", "reg add x")


def test_registry_read_backs_need_their_exact_forms():
    for record in ({"exit_status": 0, "stdout": SIGN_IN_TEXT, "stderr": ""},
                   {"exit_status": 1, "stdout": "", "stderr": SIGN_IN_TEXT + " unable to find the specified registry "
                                                                             "key or value"}):
        state, _ = harness._read_value(lambda name, command: record, "r", KEY, "N")
        assert state == "unreadable"
        state, _ = harness._read_subtree(lambda name, command: record, "r", KEY)
        assert state == "unreadable"
    absent = {"exit_status": 1, "stdout": "", "stderr": harness.REG_NOT_FOUND_LINE + "\r\n"}
    assert harness._read_value(lambda name, command: absent, "r", KEY, "N") == ("absent", None)


# --------------------------------------------------------------------------------------------------------------
# The pilot: PARTIAL, not_started and StartUnresolved
# --------------------------------------------------------------------------------------------------------------
def test_the_pilot_names_partial_beside_the_other_outcomes():
    assert run_pilot.PARTIAL == "partial"
    assert {run_pilot.RECORDED, run_pilot.NOT_RUN, run_pilot.RETRY, run_pilot.PARTIAL} == {
        "recorded", "not_run", "retry", "partial"}
    assert "start_unresolved" not in harness.SETTINGS_KEEP_GOING


# --------------------------------------------------------------------------------------------------------------
# PowerShell's serialised records on stderr. Windows PowerShell 5.1 can write a progress record (for instance while it
# prepares a module for first use) to a redirected stderr as CLIXML. Prevention: every PowerShell script the harness
# sends, and the guest script, sets $ProgressPreference = 'SilentlyContinue' as its first statement. The rule stays
# strict and deliberate: any non-blank stderr fails a judged reply closed, a progress record included, so a record
# that still arrived is never read past. The records below are synthetic, in PowerShell's published form.
# --------------------------------------------------------------------------------------------------------------
def _clixml(*kinds: str) -> str:
    objects = "".join(
        f'<Obj S="{kind}" RefId="{number}"><TN RefId="{number}"><T>System.Management.Automation.PSCustomObject</T>'
        f'<T>System.Object</T></TN><MS><I64 N="SourceId">1</I64><PR N="Record"><AV>Invented activity.</AV>'
        f'<AI>0</AI><Nil /><PI>-1</PI><PC>-1</PC><T>Completed</T><SR>-1</SR><SD> </SD></PR></MS></Obj>'
        for number, kind in enumerate(kinds))
    return ('#< CLIXML\r\n<Objs Version="1.1.0.1" xmlns="http://schemas.microsoft.com/powershell/2004/04">'
            + objects + "</Objs>")


PROGRESS = _clixml("progress")


def test_every_powershell_script_turns_progress_off_as_its_first_statement_and_nothing_else(tmp_path):
    write_script = harness.WRITE_SCRIPT.read_text(encoding="utf-8")
    for text in (SCRIPT, READ_SCRIPT, write_script):  # every -File script: the first statement after its parameters
        after_params = text[text.index("\n)\n", text.index("param(")) + 3:]
        statements = [line for line in after_params.splitlines()
                      if line.strip() and not line.lstrip().startswith("#")]
        assert statements[:2] == ["$ProgressPreference = 'SilentlyContinue'", "$ErrorActionPreference = 'Stop'"] \
            or (text is SCRIPT and statements[0] == "$ProgressPreference = 'SilentlyContinue'")
        assert "WarningPreference" not in text and "Write-Host" not in text
    assert harness.POWERSHELL_COMMAND == "powershell -NoProfile -NonInteractive -ExecutionPolicy Bypass"
    config = config_for(tmp_path)
    for command in (harness._read_command(config, "cim", {"pid": 1}),
                    harness._literal_readback_command(config, KEY + "\\Recent list", "C:\\Invented\\"),
                    harness._write_command(config, "claim", {"folder": "C:\\w", "name": "0" * 32 + ".claim",
                                                             "content": "swbpipe-claim v1\r\nx\r\n"}),
                    harness._literal_write_command(config, KEY + "\\Recent list", "C:\\Invented\\", "set", "0A")):
        assert command.startswith(harness.POWERSHELL_COMMAND + " -File ")


@pytest.mark.parametrize("kind", ["cim", "job"])
@pytest.mark.parametrize("record", ["progress", "warning", "Error"])
def test_a_serialised_record_beside_a_clean_reconciling_reply_fails_closed(tmp_path, clock, kind, record):
    config = config_for(tmp_path)
    guest = guest_for(config, clock)
    guest.stderr_added[kind] = _clixml(record)
    with pytest.raises(StartUnresolved) as raised:
        run(config, guest)
    answers = {step.get("answer") for step in raised.value.record["steps"]
               if classify(step["guest_command"]) == kind}
    assert answers == {"unresolved"}


@pytest.mark.parametrize("kind", ["claim_read", "finished_read"])
def test_a_serialised_record_on_a_report_or_claim_read_fails_closed(tmp_path, clock, kind):
    config = config_for(tmp_path)
    guest = guest_for(config, clock)
    guest.plan = {1: {"reply": "transport"}}
    guest.stderr_added[kind] = PROGRESS
    with pytest.raises(StartUnresolved):
        run(config, guest)


def test_a_serialised_record_beside_a_chunk_or_fence_reply_fails_closed(tmp_path, clock):
    config = config_for(tmp_path)
    guest = guest_for(config, clock)
    guest.stderr_added["chunk"] = PROGRESS
    record = run(config, guest)
    assert record["route"] == "collection_failed" and record["outputs_collected"] == []

    config = config_for(tmp_path / "fence")
    guest = guest_for(config, clock)
    guest.plan = {1: {"reply": "undelivered"}}
    guest.stderr_added["fence"] = PROGRESS  # the fence took effect; its reply is out of form
    record = run(config, guest)
    assert [step["answer"] for step in record["steps"] if step["name"].startswith("fence")][0] == "unresolved"
    assert record["attempts"][0]["disposition"] == "not_started"  # the claim read decides, never the fence's reply


def test_a_serialised_record_beside_the_attempt_reply_changes_nothing(tmp_path, clock):
    # The attempt's own reply is never judged: its reconciliation decides.
    config = config_for(tmp_path)
    guest = guest_for(config, clock)
    guest.stderr_added["attempt"] = PROGRESS
    assert run(config, guest)["route"] == "csv_from_batch"


@pytest.mark.parametrize("read", ["tasklist", "reg_query", "owned_pids"])
def test_a_serialised_record_on_a_guard_read_fails_closed(tmp_path, clock, read):
    config = config_for(tmp_path, GUARD)
    guest = guest_for(config, clock)
    registry = Registry(guest)
    guest.stderr_added[read] = PROGRESS
    record = run(config, guest)
    assert record["settings_guard"]["status"] in ("precondition_unknown", "settings_check_failed",
                                                  "owned_process_state_unknown")
    assert registry.writes == []
    assert check_guest_alive(config, runner=lambda argv, **k: done(
        argv, 0, "\r\nMicrosoft Windows [Version 10.0.19045.5011]\r\n", PROGRESS))["status"] == "unreachable"


def test_the_registry_api_read_and_write_fail_closed_on_any_serialised_record(tmp_path):
    config = config_for(tmp_path, GUARD)
    key = KEY + "\\Recent list"
    read = harness._literal_readback_command(config, key, "C:\\Invented\\")
    write = harness._literal_write_command(config, key, "C:\\Invented\\", "set", "0A")
    shown = f"{ECHO}\r\n{key}\r\n    C:\\Invented\\    REG_BINARY    0A\r\n"

    def step_answering(stdout, stderr, code=0):
        def step(name, command):
            record = {"name": name, "guest_command": command, "exit_status": code, "stdout": stdout, "stderr": stderr}
            harness._settle_echo(record)  # as `_guest_step` does for every reply
            return record
        return step
    assert harness._read_value(step_answering(shown, ""), "r", key, "C:\\Invented\\", read) == (
        "present", ("REG_BINARY", "0A"))
    assert harness._read_value(step_answering(shown, PROGRESS), "r", key, "C:\\Invented\\", read)[0] == "unreadable"
    assert harness._read_value(step_answering(ECHO, PROGRESS + harness.REG_NOT_FOUND_LINE, 1), "r", key,
                               "C:\\Invented\\", read)[0] == "unreadable"  # never read as "absent"
    assert harness._read_value(step_answering(ECHO, harness.REG_NOT_FOUND_LINE, 1), "r", key,
                               "C:\\Invented\\", read)[0] == "absent"
    # Without the echo, the same answers are unresolved: never present, never absent.
    for stdout, stderr, code in ((shown[len(ECHO):], "", 0), ("", harness.REG_NOT_FOUND_LINE, 1)):
        assert harness._read_value(step_answering(stdout, stderr, code), "r", key, "C:\\Invented\\",
                                   read)[0] == "unreadable"
    steps: list = []
    for command in (write, 'reg add "x" /f'):
        with pytest.raises(harness.RestoreWriteFailed) as raised:
            harness._guest_write(config, lambda argv, **k: done(argv, 0, "The operation completed successfully.",
                                                                PROGRESS), 20.0, steps, "w", command)
        assert raised.value.outcome == "unknown" and not raised.value.transport  # stopped, never reissued


# --------------------------------------------------------------------------------------------------------------
# The held export's three replies (prepare, report, collect) and the subtree read-back: each counts only in its form.
# --------------------------------------------------------------------------------------------------------------
def _export_site(command: str) -> str:
    return {"(if": "export_prepare", "powershell": "export", "copy": "export_collect"}[command.split(" ")[0]]


@pytest.mark.parametrize("site, route", [("export_prepare", "prepare_failed"), ("export", "no_guest_json"),
                                         ("export_collect", "mbf_not_collected")])
@pytest.mark.parametrize("reply", ["sign-in text, exit 0", "serialised progress record on stderr"])
def test_each_held_export_reply_counts_only_in_its_form(tmp_path, site, route, reply):
    from tests.test_caepipe_correlation import export_runner, solved_case
    config, out = solved_case(tmp_path, "cpc-e")
    base = export_runner(out, [])
    seen = []

    def runner(argv, **kwargs):
        result = base(argv, **kwargs)
        seen.append(_export_site(argv[6]))
        if _export_site(argv[6]) == site:
            result = done(argv, 0, SIGN_IN_TEXT, "") if reply.startswith("sign-in") else \
                done(argv, result.returncode, result.stdout, PROGRESS)
        return result
    entry = harness.run_caepipe_mbf_export(config, "cpc-e", MBF_SHA, runner=runner)
    assert entry["route"] == route and entry["exported_mbf"]["sha256"] is None
    assert not (out / "exported.mbf").exists()  # nothing out of form is left where the import regression reads
    assert seen.count(site) == 1  # never retried: not a transport answer


def test_the_held_exports_report_counts_only_alone_on_stdout(tmp_path):
    from tests.test_caepipe_correlation import EXPORTED_SHA, MOD_SHA, export_runner, solved_case
    config, out = solved_case(tmp_path, "cpc-e")
    report = json.dumps({"status": "mbf_exported", "mod_sha256": MOD_SHA, "exported_mbf_sha256": EXPORTED_SHA})
    base = export_runner(out, [])

    def runner(argv, **kwargs):
        result = base(argv, **kwargs)
        return done(argv, 0, SIGN_IN_TEXT + "\r\n" + report, "") if _export_site(argv[6]) == "export" else result
    assert harness.run_caepipe_mbf_export(config, "cpc-e", MBF_SHA, runner=runner)["route"] == "no_guest_json"
    assert harness.run_caepipe_mbf_export(config, "cpc-e", MBF_SHA, runner=base)["route"] == "mbf_exported"


def test_a_serialised_record_on_a_subtree_read_back_fails_closed():
    key = KEY + "\\Recent list"
    shown = f"\r\n{key}\r\n    1    REG_BINARY    01\r\n"

    def step_answering(stdout, stderr, code=0):
        return lambda name, command: {"name": name, "guest_command": command, "exit_status": code,
                                      "stdout": stdout, "stderr": stderr}
    assert harness._read_subtree(step_answering(shown, ""), "r", key) == (
        "present", {(harness._registry_key_id(key), "1"): ("REG_BINARY", "01")})
    assert harness._read_subtree(step_answering("", ""), "r", key) == ("present", {})
    for stdout, stderr, code in ((shown, PROGRESS, 0), ("", PROGRESS, 0),
                                 ("", PROGRESS + harness.REG_NOT_FOUND_LINE, 1)):
        assert harness._read_subtree(step_answering(stdout, stderr, code), "r", key) == ("unreadable", None)


# --------------------------------------------------------------------------------------------------------------
# A report with no input hash: nothing was compared, so no route says the input hash mismatched.
# --------------------------------------------------------------------------------------------------------------
@pytest.mark.parametrize("status, route", [
    ("attempt_folder_exists", "attempt_folder_exists"),
    ("attempt_failed", "attempt_failed"),
    ("csv_from_batch", "input_hash_unreported"),  # a success status without its hash is not a success
    (None, "input_hash_unreported"),
])
def test_a_terminal_report_without_an_input_hash_never_reads_as_a_mismatch(tmp_path, clock, status, route):
    config = config_for(tmp_path)
    guest = guest_for(config, clock)
    guest.plan = {1: {"kill_at": "folder", "reply": "transport"}}  # claimed, job made, ended before any copy

    def report_without_hash():
        token = guest.attempts[0].token
        report = {"schema": harness.GUEST_REPORT_SCHEMA, "attempt_token": token, "mbf_sha256": None,
                  **({"status": status} if status else {})}
        text = json.dumps(report, separators=(",", ":"))
        guest.answers[("finished_read", 1)] = (0, f"swbpipe-finished v1\r\ntoken={token}\r\nreport={text}\r\n", "")
    guest.hooks[("finished_read", 1)] = report_without_hash
    record = run(config, guest)
    assert record["start"]["state"] == "completed" and record["route"] == route
    assert record["guest_mbf_sha256"] is None and "chunk" not in guest.kinds  # nothing compared, nothing collected
    paths = harness.case_paths(config, CASE)
    _, bound_route, reason = run_pilot.bound_csv(config.exchange_root.resolve(), paths, record, MBF_SHA)
    assert bound_route == "unbound_output" and "absent or different" in reason


@pytest.mark.parametrize("report, code, route", [
    ({"status": "export_held"}, 6, "export_held"),                    # the guest script's real refusal
    ({"status": "export_failed"}, 4, "export_failed"),                # an older failure report without the hash
    ({"status": "mbf_not_written"}, 4, "mbf_not_written"),
    ({"status": "mbf_exported", "exported_mbf_sha256": "1" * 64}, 0, "input_hash_unreported"),
    ({}, 4, "input_hash_unreported"),
    ({"status": "mbf_exported", "mod_sha256": "0" * 64, "exported_mbf_sha256": "1" * 64}, 0, "input_hash_mismatch"),
])
def test_an_export_report_without_the_models_hash_never_reads_as_a_mismatch(tmp_path, report, code, route):
    from tests.test_caepipe_correlation import export_runner, solved_case
    config, out = solved_case(tmp_path, "cpc-e")
    commands: list[str] = []
    base = export_runner(out, commands, guest=report)

    def runner(argv, **kwargs):
        result = base(argv, **kwargs)
        return done(argv, code, result.stdout, "") if argv[6].startswith("powershell") else result
    entry = harness.run_caepipe_mbf_export(config, "cpc-e", MBF_SHA, runner=runner)
    assert entry["route"] == route
    assert not any(command.startswith("copy") for command in commands) and not (out / "exported.mbf").exists()


# --------------------------------------------------------------------------------------------------------------
# The first independent reading's own passing cases, kept: section 3a's late and fenced invocations, lost fence
# replies, a collection over one chunk, malformed and wrong-token claims, a denied fence, a missing creation time,
# a denied job query, and a record failure while the start is unresolved.
# --------------------------------------------------------------------------------------------------------------
def test_a_late_first_invocation_cannot_launch_after_its_token_was_fenced(tmp_path, clock):
    config = config_for(tmp_path)
    guest = guest_for(config, clock)
    guest.plan = {1: {"run": "work", "reply": "transport"}}
    guest.hooks[("attempt", 2)] = lambda: guest.attempts[0].advance()  # the first arrives after the redispatch
    record = run(config, guest)
    assert [entry["disposition"] for entry in record["attempts"]] == ["not_started", "completed"]
    assert guest.launches == [record["attempt_token"]] and guest.attempts[0].exit == 20  # its claim lost


def test_a_permission_failure_on_the_fence_is_unknown_without_a_second_dispatch(tmp_path, clock):
    config = config_for(tmp_path, GUARD)
    guest = guest_for(config, clock)
    registry = Registry(guest)
    guest.plan = {1: {"reply": "undelivered"}}
    base = guest.runner

    def deny(argv, **kwargs):
        if classify(argv[6]) == "fence":
            clock["t"] += 1
            return done(argv, 1, "", "Access is denied.")
        return base(argv, **kwargs)
    guest.runner = deny
    with pytest.raises(StartUnresolved):
        run(config, guest)
    assert guest.kinds.count("attempt") == 1 and registry.writes == []


@pytest.mark.parametrize("fence_effect", ["ran", "undelivered", "delayed"])
def test_a_lost_fence_reply_is_settled_by_its_claim_even_when_the_fence_arrives_late(tmp_path, clock, fence_effect):
    config = config_for(tmp_path)
    guest = guest_for(config, clock)
    guest.plan = {1: {"run": "work", "reply": "transport"}}
    base = guest.runner
    fences = []

    def runner(argv, **kwargs):
        if classify(argv[6]) == "fence":
            fences.append((argv, kwargs))
            if len(fences) == 1:
                if fence_effect == "ran":
                    base(argv, **kwargs)
                clock["t"] += 1
                return done(argv, 255, "", TRANSPORT[0])
            if fence_effect == "delayed" and len(fences) == 2:
                base(fences[0][0], **fences[0][1])  # the first fence arrives late, just before the second
        return base(argv, **kwargs)
    guest.runner = runner
    guest.hooks[("attempt", 2)] = lambda: guest.attempts[0].advance()
    record = run(config, guest)
    assert [entry["disposition"] for entry in record["attempts"]] == ["not_started", "completed"]
    assert guest.launches == [record["attempt_token"]]
    assert len(fences) == (1 if fence_effect == "ran" else 2)


def test_a_missing_creation_time_never_establishes_the_launchers_end(tmp_path, clock):
    config = config_for(tmp_path)
    guest = guest_for(config, clock)
    for number in range(1, 500):
        guest.answers[("cim", number)] = (0, ECHO + "SWBPIPE-CIM v1 pid=3004 created=\r\n", "")
    with pytest.raises(StartUnresolved):
        run(config, guest)
    assert "job" not in guest.kinds


@pytest.mark.parametrize("claim", ["", "swbpipe-claim v1\r\n", SIGN_IN_TEXT,
                                   "swbpipe-claim v1\r\ntoken=" + "0" * 32 + "\r\nowner=host\r\nnonce=" + "a" * 32
                                   + "\r\n"], ids=["empty", "truncated", "sign-in text", "another token"])
def test_an_incomplete_or_wrong_token_claim_stays_unknown(tmp_path, clock, claim):
    config = config_for(tmp_path, GUARD)
    guest = guest_for(config, clock)
    registry = Registry(guest)
    for number in range(1, 500):
        guest.answers[("claim_read", number)] = (0, claim, "")
    with pytest.raises(StartUnresolved):
        run(config, guest)
    assert guest.kinds.count("attempt") == 1 and "fence" not in guest.kinds and registry.writes == []


def test_a_denied_job_query_is_unknown_never_absent(tmp_path, clock):
    config = config_for(tmp_path)
    guest = guest_for(config, clock)
    for number in range(1, 500):
        guest.answers[("job", number)] = (0, ECHO + "SWBPIPE-JOB v1 error=5\r\n", "")
    with pytest.raises(StartUnresolved):
        run(config, guest)
    assert "chunk" not in guest.kinds


def test_a_collection_over_one_chunk_changes_no_guest_file_and_checks_every_hash(tmp_path, clock):
    config = config_for(tmp_path)
    guest = guest_for(config, clock)
    payload = bytes(range(256)) * ((harness.COLLECT_CHUNK_BYTES + 17) // 256 + 1)
    guest.programs[1] = Program(files={"model.csv": payload}, print_name="model.csv")
    guest.lost.add(("chunk", 1))
    before: list[dict] = []
    guest.hooks[("chunk", 1)] = lambda: before.append(dict(guest.files))
    record = run(config, guest)
    assert guest.files == before[0]
    assert (config.exchange_root / "cases" / CASE / record["observed_csv_path"]["host"]).read_bytes() == payload
    reads = [step for step in record["steps"] if step["name"].startswith("collect_model.csv")]
    assert [step["answer"] for step in reads] == ["unresolved", "chunk", "chunk"]  # two chunks, one read again


def test_a_record_failure_while_the_start_is_unresolved_keeps_restoration_inhibited(tmp_path, monkeypatch, clock):
    config = config_for(tmp_path, GUARD)
    guest = guest_for(config, clock)
    registry = Registry(guest)
    guest.cim_unavailable = True
    real_write = harness.write_private

    def failing(root, path, data):
        if Path(path).name == "run_record.json":
            raise OSError("invented failure writing the run record")
        return real_write(root, path, data)
    monkeypatch.setattr(harness, "write_private", failing)
    with pytest.raises(OSError) as raised:
        run(config, guest)
    assert raised.value.start_unresolved is True and raised.value.settings_guard["status"] == "start_unresolved"
    assert registry.writes == []


# --------------------------------------------------------------------------------------------------------------
# A success terminal report contradicted by its own dispatch reply: recorded apart from a run that left no report,
# restoration permitted (quiescent, with a report), nothing collected.
# --------------------------------------------------------------------------------------------------------------
def _replying(guest, code, stdout=None, stderr=""):
    base = guest.runner

    def runner(argv, **kwargs):
        result = base(argv, **kwargs)
        if classify(argv[6]) != "attempt":
            return result
        return done(argv, code, result.stdout if stdout is None else stdout, stderr)
    guest.runner = runner


def test_a_contradicted_report_is_recorded_apart_from_a_run_that_left_no_report(tmp_path, clock):
    contradicted_config = config_for(tmp_path / "contradicted")
    guest = guest_for(contradicted_config, clock)
    _replying(guest, 3)
    contradicted = run(contradicted_config, guest)
    partial_config = config_for(tmp_path / "partial")
    guest = guest_for(partial_config, clock)
    guest.plan = {1: {"kill_at": "launched", "reply": "transport"}}
    partial = run(partial_config, guest)
    assert (contradicted["route"], contradicted["start"]["state"]) == ("report_reply_contradiction", "contradicted")
    assert (partial["route"], partial["start"]["state"]) == ("start_partial", "partial")
    assert contradicted["guest_log"]["status"] == "csv_from_batch" and partial["guest_log"] is None
    assert contradicted["outputs_collected"] == [] and contradicted["observed_csv_path"]["host"] is None


def test_a_contradicted_report_permits_restoration(tmp_path, clock):
    config = config_for(tmp_path, GUARD)
    guest = guest_for(config, clock)
    registry = Registry(guest)  # the program changed the saved settings
    _replying(guest, 1)
    record = run(config, guest)
    assert record["route"] == "report_reply_contradiction"
    assert record["settings_guard"]["status"] == "settings_restored" and registry.writes


@pytest.mark.parametrize("answer, decides", [
    ((0, "x", ""), True), ((255, "", TRANSPORT[0]), True), ((255, "", TRANSPORT[1]), True),
    ((255, "", harness.SESSION_UNAVAILABLE_LINE), True), ((7, harness.SESSION_UNAVAILABLE_LINE, ""), True),
    (("timeout", "", ""), True), (("launch_failed", "", "no prlctl"), True),
    ((1, "", ""), False), ((3, "{}", ""), False), ((-9, "", ""), False), ((1, "", TRANSPORT[0]), False),
    ((255, "", "PrlJob_GetResult"), False), ((255, "", harness.SESSION_UNAVAILABLE_LINE + " extra"), False),
])
def test_only_exit_0_no_reply_a_transport_answer_or_the_session_line_leave_the_report_to_decide(answer, decides):
    assert harness.reply_leaves_report_to_decide(*answer) is decides


def test_the_one_classifier_reads_only_the_live_record_and_absent_is_unresolved():
    assert harness._classify({}) == {"start": "unresolved", "restore": False, "retry": False, "evidence_saved": False}
    assert harness._classify({"start": "invented"})["start"] == "unresolved"
    expected = {"not_dispatched": (True, True), "not_started": (True, True), "completed": (True, False),
                "partial": (True, False), "contradicted": (True, False), "unresolved": (False, False)}
    for start, (restore, retry) in expected.items():
        assert harness.disposition(start) == {"start": start, "restore": restore, "retry": retry}
    source = Path(harness.__file__).read_text(encoding="utf-8")
    assert "start_state = " not in source  # no local copy of the disposition is kept anywhere
    assert source.count('live["start"] = ') == 1  # written in one place
