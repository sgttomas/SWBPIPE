"""The write script and the changing operations' short forms (DEL-17-05): the fence and the registry API writes, every
failure state the reconciliation names, the read-only state report, the session's settings record, the controlled
exercise on harmless resources, and the retry classification. Against the fake guest only: no prlctl, no virtual
machine; everything here is invented."""

from __future__ import annotations

import dataclasses
import re
import subprocess
import sys
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.handoff.caepipe_external import harness  # noqa: E402
from core.handoff.caepipe_external.harness import StartUnresolved  # noqa: E402
from tests.caepipe_fake_guest import (  # noqa: E402
    SIGN_IN_TEXT, TRANSPORT, FakeGuestModel, classify, done, script_call, write_call)
from tests.test_caepipe_start_reconciliation import (  # noqa: E402
    GUARD, KEY, Registry, clock, config_for, guest_for, run)

__all__ = ["clock"]  # the fixture, used by name

WRITE_SOURCE = harness.WRITE_SCRIPT.read_text(encoding="utf-8")
WRITE_ECHO = f"SWBPIPE-WRITE v1 script={harness.WRITE_SCRIPT_SHA256} data=none\r\n"
DATE = "20261121"
NAMES = harness.exercise_names(DATE)
OK = "The operation completed successfully."
NOT_FOUND = harness.REG_NOT_FOUND_LINE


def _code(source: str) -> str:
    source = re.sub(r"<#.*?#>", "", source, flags=re.DOTALL)
    return "\n".join("" if line.lstrip().startswith("#") else line for line in source.splitlines())


def _function(code: str, name: str) -> str:
    body = code[code.index(f"function {name}"):]
    following = body.find("\nfunction ", 1)
    return body if following < 0 else body[:following]


# --------------------------------------------------------------------------------------------------------------
# The write script changes only what it names, each change inside its own confined operation.
# --------------------------------------------------------------------------------------------------------------
NEVER = [r"\bSet-", r"\bNew-", r"\bRemove-", r"\bMove-", r"\bCopy-", r"\bRename-", r"\bOut-File\b",
         r"\bStart-Process\b", r"\bInvoke-Expression\b", r"\biex\b", r"\bAdd-Type\b", r"Process\]::Start",
         r"\bcmd\b", r"WriteAll", r"&\s*\S", r"Stop-Process"]
# Each changing call, and the one operation it may appear in.
CHANGES = {"CreateNew": "OpClaim", "CreateDirectory": "OpClaim", "SetValue": "OpRegValue",
           "DeleteValue": "OpRegValue", "OpenSubKey($keyName, $true)": "OpRegValue", "CreateSubKey": "OpKey",
           "DeleteSubKeyTree": "OpKey"}


@pytest.mark.parametrize("pattern", NEVER)
def test_the_write_script_starts_nothing_and_holds_no_cmdlet_that_changes(pattern):
    assert re.search(pattern, _code(WRITE_SOURCE), re.IGNORECASE) is None, pattern


@pytest.mark.parametrize("call, op", list(CHANGES.items()))
def test_each_change_of_the_write_script_lies_inside_its_own_confined_operation(call, op):
    code = _code(WRITE_SOURCE)
    assert code.count(call) == _function(code, op).count(call) >= 1, call


def test_the_write_scripts_operations_are_confined_by_pattern():
    code = _code(WRITE_SOURCE)
    claim, value, key = _function(code, "OpClaim"), _function(code, "OpRegValue"), _function(code, "OpKey")
    assert "OnC (" in claim and "'^(SWBPIPE-harness-exercise-)?[0-9a-f]{32}\\.claim\\z'" in claim
    assert "$exerciseFolder" in claim and "swbpipe-claim v1" in claim
    assert "RegKey (" in value and "RegistryValueKind]::Binary" in value
    assert "$exerciseKey" in key and "-cnotmatch $exerciseKey" in key
    assert "$exerciseKey = '^Software\\\\SWBPIPE-harness-exercise-[0-9]{8}\\z'" in code


# --------------------------------------------------------------------------------------------------------------
# C3: every failure state of the changing operations the reconciliation names, in the short forms.
# The fence (`claim`): its reply is recorded and checked against its form; the claim read decides.
# --------------------------------------------------------------------------------------------------------------
FENCE_REPLIES = {
    "created": None,                                                     # the fence's own answer, in form
    "exists": (0, "{echo}SWBPIPE-CLAIM v1 exists\r\n", ""),               # the name existed (a late earlier fence)
    "held": (0, "{echo}SWBPIPE-CLAIM v1 held\r\n", ""),                   # a sharing violation on the name
    "sign-in text, exit 0": (0, SIGN_IN_TEXT, ""),
    "serialised record on stderr": (0, "{echo}SWBPIPE-CLAIM v1 created\r\n", "#< CLIXML\r\n<Objs/>"),
    "transport line (the effect happened)": (255, "", TRANSPORT[0]),
    "session-unavailable line": (1, "", harness.SESSION_UNAVAILABLE_LINE),
    "echo missing": (0, "SWBPIPE-CLAIM v1 created\r\n", ""),
    "echo of another script": (0, "SWBPIPE-WRITE v1 script=" + "0" * 64 + " data=none\r\nSWBPIPE-CLAIM v1 created\r\n", ""),
    "error exit": (1, "{echo}", "Access to the path is denied."),
}


@pytest.mark.parametrize("reply", list(FENCE_REPLIES))
def test_every_fence_reply_is_recorded_and_the_claim_read_decides(tmp_path, clock, reply):
    config = config_for(tmp_path)
    guest = guest_for(config, clock)
    guest.plan = {1: {"reply": "undelivered"}}  # the first dispatch never arrives: the host fences it
    answer = FENCE_REPLIES[reply]
    if answer is not None:
        guest.answers[("fence", 1)] = (answer[0], answer[1].format(echo=WRITE_ECHO), answer[2])
    record = run(config, guest)
    fence = next(step for step in record["steps"] if step["name"] == "fence_1")
    expected = {"created": "created", "exists": "exists", "held": "held"}.get(reply, "unresolved")
    assert fence["answer"] == expected
    # Whatever the fence answered, its effect (the host's claim with this round trip's nonce) is read back.
    assert record["attempts"][0]["disposition"] == "not_started" and record["route"] == "csv_from_batch"
    assert guest.launches == [record["attempt_token"]]


def test_a_fence_refused_without_effect_leaves_the_start_unresolved(tmp_path, clock):
    config = config_for(tmp_path, GUARD)
    guest = guest_for(config, clock)
    registry = Registry(guest)
    guest.plan = {1: {"reply": "undelivered"}}
    guest.fence_denied = True  # exit 1, access denied, nothing created
    with pytest.raises(StartUnresolved) as raised:
        run(config, guest)
    fences = [step for step in raised.value.record["steps"] if step["name"].startswith("fence")]
    assert fences and {step["answer"] for step in fences} == {"unresolved"}
    assert raised.value.record["settings_guard"]["status"] == "start_unresolved" and registry.writes == []
    assert raised.value.disposition == {"start": "unresolved", "restore": False, "retry": False,
                                        "evidence_saved": True}


def test_a_held_claim_read_is_unresolved_never_absent(tmp_path, clock):
    config = config_for(tmp_path)
    guest = guest_for(config, clock)
    guest.plan = {1: {"run": "claimed", "reply": "transport", "resume": ("claim_read", 3)}}

    def hold():
        guest.locked.add(guest.attempts[0].path(".claim"))  # the writer still holds the name
    guest.hooks[("claim_read", 1)] = hold
    guest.hooks[("claim_read", 3)] = lambda: guest.locked.clear()
    record = run(config, guest)
    assert [step["name"] for step in record["steps"] if step["name"].startswith("fence")] == []  # never fenced
    assert record["attempts"][0]["disposition"] == "completed"


# The registry writes, in the short forms: each reply of a write is classified by `_guest_write`, the transport
# answer alone settled by reading back.
WRITE_REPLIES = {
    "completed": ((0, "{echo}" + OK, ""), "completed"),
    "failed: access denied": ((1, "{echo}", "ERROR: Access is denied."), "failed"),
    "failed: not found": ((1, "{echo}", NOT_FOUND), "failed"),
    "unknown: exit 0, another text": ((0, "{echo}" + SIGN_IN_TEXT, ""), "unknown"),
    "unknown: exit 0, echo missing": ((0, OK, ""), "unknown"),
    "unknown: exit 0, serialised record": ((0, "{echo}" + OK, "#< CLIXML\r\n<Objs/>"), "unknown"),
    "unknown: exit 255, other text": ((255, "", "prlctl: an invented client failure"), "unknown"),
    "unknown, settled: transport line": ((255, "", TRANSPORT[1]), "unknown"),
    "failed: session-unavailable line": ((1, "", harness.SESSION_UNAVAILABLE_LINE), "failed"),
}


def _write_commands(config) -> dict[str, str]:
    return {"regset": harness._literal_write_command(config, KEY + "\\Recent list", "C:\\Invented\\", "set", "0A"),
            "regdelete": harness._literal_write_command(config, KEY + "\\Recent list", "C:\\Invented\\", "delete"),
            "keycreate": harness.exercise_registry_command(config, DATE, "keycreate"),
            "keydelete": harness.exercise_registry_command(config, DATE, "keydelete"),
            "reg add": f'reg add "{KEY}\\Recent list" /v "1" /t REG_BINARY /d "0A" /f'}


@pytest.mark.parametrize("op", ["regset", "regdelete", "keycreate", "keydelete", "reg add"])
@pytest.mark.parametrize("reply", list(WRITE_REPLIES))
def test_every_write_reply_is_classified_in_the_short_forms(tmp_path, op, reply):
    config = config_for(tmp_path, GUARD)
    command = _write_commands(config)[op]
    (code, stdout, stderr), outcome = WRITE_REPLIES[reply]
    echo = WRITE_ECHO if op != "reg add" else ""
    steps: list = []
    state: dict = {}
    runner = lambda argv, **kwargs: done(argv, code, stdout.format(echo=echo), stderr)  # noqa: E731
    if outcome == "completed":
        assert harness._guest_write(config, runner, 5.0, steps, "w", command, state)["outcome"] == "completed"
        return
    if op == "reg add" and reply == "unknown: exit 0, echo missing":
        assert harness._guest_write(config, runner, 5.0, steps, "w", command, state)["outcome"] == "completed"
        return  # reg.exe carries no echo: its success line alone is the answer
    with pytest.raises(harness.RestoreWriteFailed) as raised:
        harness._guest_write(config, runner, 5.0, steps, "w", command, state)
    assert raised.value.outcome == outcome
    assert raised.value.transport == reply.endswith("transport line")  # only that one is settled and reissued
    assert steps[-1]["outcome"] == outcome and steps[-1]["mutating"]


@pytest.mark.parametrize("raised", [subprocess.TimeoutExpired(["x"], 5), OSError("invented launch failure")])
def test_a_write_whose_call_raises_is_recorded_and_raised_unchanged(tmp_path, raised):
    config = config_for(tmp_path, GUARD)
    steps: list = []
    state: dict = {}

    def runner(argv, **kwargs):
        raise raised
    with pytest.raises(type(raised)):
        harness._guest_write(config, runner, 5.0, steps, "w", _write_commands(config)["regset"], state)
    assert state["primary_error"] is raised and steps[-1]["exit_status"] == type(raised).__name__
    assert harness._write_outcome(raised) == ("failed" if isinstance(raised, OSError) else "unknown")


def test_long_registry_data_travels_in_a_data_file_and_every_command_fits(tmp_path):
    config = config_for(tmp_path, GUARD)
    data = "AB" * 3000
    command = harness._literal_write_command(config, KEY + "\\Recent list", "C:\\Invented\\", "set", data)
    call = write_call(command)
    assert call["op"] == "regset" and call["data"] and "data" not in call["payload"]
    assert harness._guest_command_length(config.vm_name, command) <= harness.GUEST_COMMAND_LIMIT
    stored = config.exchange_root / harness.HARNESS_DIR / harness.DATA_DIR / f"{call['data']}.json"
    assert harness.json.loads(stored.read_text(encoding="utf-8")) == {"data": data}
    # A REG_BINARY value whose reg add would not fit goes the same way; a short one stays reg add.
    root = harness._registry_key_id(GUARD.key)
    recent = KEY + "\\Recent list"
    before = {"keys": {root: KEY, root + "\\recent list": recent},
              "values": {(root + "\\recent list", "1"): (recent, "REG_BINARY", data),
                         (root + "\\recent list", "2"): (recent, "REG_BINARY", "0A")}}
    after = {"keys": before["keys"], "values": {(root + "\\recent list", "1"): (recent, "REG_BINARY", "0B"),
                                                (root + "\\recent list", "2"): (recent, "REG_BINARY", "0B")}}
    plan = harness._restore_plan(before, after, root, whole_key=True, config=config)
    assert write_call(plan[0]["command"])["op"] == "regset" and plan[1]["command"].startswith("reg add ")
    assert all(harness._guest_command_length(config.vm_name, entry["command"]) <= 850 for entry in plan)


def test_an_unknown_registry_api_write_is_settled_by_its_read_back_in_the_short_forms(tmp_path, clock):
    from tests.test_caepipe_correlation import FakeGuest, KEY_GUARD, _created, _literal_solve, guarded
    guest = FakeGuest()
    before = dict(guest.values)
    guest.write_results[1] = (255, "", TRANSPORT[0])
    guest.applied_anyway.add(1)
    record = guarded(tmp_path, guest, _literal_solve(_created), guard=KEY_GUARD)
    assert record["settings_guard"]["status"] == "settings_restored" and guest.values == before
    sent = [classify(command) for command in guest.commands]
    assert "literal" in sent and "regread" in sent
    assert not any(" -EncodedCommand " in command for command in guest.commands)


# --------------------------------------------------------------------------------------------------------------
# The read-only state report, its success predicate, and the session's settings record.
# --------------------------------------------------------------------------------------------------------------
def test_the_state_report_after_a_completed_run_meets_the_success_predicate(tmp_path, clock):
    config = config_for(tmp_path, GUARD)
    guest = guest_for(config, clock)
    Registry(guest, change_on_launch=False)
    before = harness.read_settings_record(config, runner=guest.runner)
    record = run(config, guest)
    commands = len(guest.commands)
    report = harness.read_state_report(config, runner=guest.runner, case_id="cx-s", token=record["attempt_token"],
                                       settings_before=before, storage_folders=("C:\\invented-work\\cx-s",))
    attempt = report["attempt"]
    assert attempt == attempt | {"claim": "script", "launcher": "ended", "job": "absent", "terminal": "complete",
                                 "classified": "completed", "success": True, "terminal_status": "csv_from_batch"}
    assert report["settings"] == {"read": True, "differences": [], "unchanged": True}
    assert report["storage"]["meets"] is True
    sent = guest.kinds[commands:]
    assert set(sent) <= {"chunk", "cim", "job", "reg_query", "storage"}  # reads only
    assert "fence" not in sent and "attempt" not in sent


@pytest.mark.parametrize("how, expected", [
    ("no terminal report", {"claim": "script", "terminal": "absent", "classified": "partial", "success": False}),
    ("job still has a member", {"job": "present", "classified": "unknown", "success": False}),
    ("launcher still running", {"launcher": "running", "classified": "unknown", "success": False}),
    ("fenced", {"claim": "host", "classified": "fenced", "success": False}),
    ("another token's claim", {"claim": "other", "classified": "unknown", "success": False}),
    ("claim held", {"claim": "unreadable", "classified": "unknown", "success": False}),
    ("nothing there", {"claim": "absent", "terminal": "absent", "classified": "unknown", "success": False}),
])
def test_the_state_report_says_what_the_reads_show_and_nothing_more(tmp_path, clock, how, expected):
    config = config_for(tmp_path)
    guest = guest_for(config, clock)
    token = "a" * 32
    work = "C:\\invented-work\\cx-s"
    guest.files[f"{work}\\{token}.claim"] = (
        f"swbpipe-claim v1\r\ntoken={token}\r\nowner=script\r\npid=3004\r\ncreated=20261114000001.000000\r\n"
        f"nonce={'b' * 32}\r\n").encode("ascii")
    report_text = harness.json.dumps({"schema": harness.GUEST_REPORT_SCHEMA, "attempt_token": token,
                                      "status": "csv_from_batch"})
    guest.files[f"{work}\\{token}.finished"] = f"swbpipe-finished v1\r\ntoken={token}\r\nreport={report_text}\r\n".encode()
    if how == "no terminal report":
        del guest.files[f"{work}\\{token}.finished"]
    elif how == "job still has a member":
        guest.jobs[f"swbpipe-{token}"] = 1
    elif how == "launcher still running":
        guest.processes[3004] = harness_process = type("P", (), {"pid": 3004, "created": "20261114000001.000000",
                                                                "alive": True, "job": None})()
        assert harness_process.alive
    elif how == "fenced":
        guest.files[f"{work}\\{token}.claim"] = (f"swbpipe-claim v1\r\ntoken={token}\r\nowner=host\r\n"
                                                 f"nonce={'c' * 32}\r\n").encode("ascii")
    elif how == "another token's claim":
        guest.files[f"{work}\\{token}.claim"] = guest.files[f"{work}\\{token}.claim"].replace(b"a" * 32, b"d" * 32)
    elif how == "claim held":
        guest.locked.add(f"{work}\\{token}.claim")
        base = guest._answer

        def answer(argv, command, kind, payload):  # the chunk read meets the writer's sharing violation
            if kind == "chunk" and payload["path"] in guest.locked:
                return done(argv, 1, "", "The process cannot access the file because it is being used by another "
                                         "process.")
            return base(argv, command, kind, payload)
        guest._answer = answer
    elif how == "nothing there":
        guest.files.clear()
    harness.install_guest_scripts(config)
    state = harness.read_state_report(config, runner=guest.runner, case_id="cx-s", token=token)["attempt"]
    assert state == state | expected, state


def test_the_session_settings_record_compares_on_the_host(tmp_path, clock):
    config = config_for(tmp_path, GUARD)
    guest = guest_for(config, clock)
    registry = Registry(guest, change_on_launch=False)
    first = harness.read_settings_record(config, runner=guest.runner)
    registry.values[(KEY + "\\Recent list", "1")] = ("REG_BINARY", "07")
    second = harness.read_settings_record(config, runner=guest.runner)
    [difference] = harness.settings_difference(first, second)
    assert difference["value"].endswith("\\Recent list\\1") and difference["after"] == {"type": "REG_BINARY",
                                                                                         "data": "07"}
    assert set(guest.kinds) == {"reg_query"} and registry.writes == []
    with pytest.raises(ValueError):
        harness.read_settings_record(config_for(tmp_path / "unguarded"), runner=guest.runner)


def test_the_run_record_shows_the_retry_classification(tmp_path, clock):
    config = config_for(tmp_path)
    record = run(config, guest_for(config, clock))
    assert record["start"] | {"reason": None, "claim": None} == {
        "state": "completed", "restore_permitted": True, "retry_permitted": False, "reason": None, "claim": None}
    guest = guest_for(config_for(tmp_path / "twice"), clock)
    guest.plan = {1: {"reply": "undelivered"}, 2: {"reply": "undelivered"}}
    record = run(config_for(tmp_path / "twice"), guest)
    assert (record["start"]["state"], record["start"]["retry_permitted"]) == ("not_started", True)
    assert [entry["disposition"] for entry in record["attempts"]] == ["not_started", "not_started"]


# --------------------------------------------------------------------------------------------------------------
# The controlled exercise: the final operations on harmless resources, confined to the exercise's own names.
# --------------------------------------------------------------------------------------------------------------
# What `dir` prints on stderr for a folder with nothing in it, in a few display languages (invented renderings of the
# kind each language shows; the classification must never depend on any of them).
DIR_EMPTY_MESSAGES = {"English": "File Not Found", "Deutsch": "Datei nicht gefunden",
                      "Francais": "Fichier introuvable", "Espanol": "No se encuentra el archivo"}


class ExerciseRegistry:
    """The current user's hive as far as the exercise touches it, and cmd running the key-presence and folder-listing
    forms: a redirection `2>nul` discards what the command writes on stderr, in the guest's display language."""

    def __init__(self, guest: FakeGuestModel, *, denied: bool = False, language: str = "English"):
        self.keys: dict[str, dict[str, str]] = {}
        self.guest, self.denied, self.language = guest, denied, language
        self.changes: list[str] = []
        guest.extra = self.answer

    def answer(self, argv, command, kind, payload=None):
        call = script_call(command)
        if kind == "keywrite":
            if self.denied:
                return done(argv, 1, "", "Requested registry access is not allowed.")
            self.changes.append(call["op"])
            if call["op"] == "keycreate":
                self.keys.setdefault(call["payload"]["key"], {})
                return done(argv, 0, OK, "")
            if call["payload"]["key"] not in self.keys:
                return done(argv, 1, "", NOT_FOUND)
            del self.keys[call["payload"]["key"]]
            return done(argv, 0, OK, "")
        if kind == "literal":
            values = self.keys.get(call["payload"]["key"])
            if values is None or self.denied:
                return done(argv, 1, "", NOT_FOUND if values is None else "Requested registry access is not allowed.")
            self.changes.append(call["op"])
            if call["op"] == "regset":
                values[call["payload"]["name"]] = call["payload"]["data"]
            elif call["payload"]["name"] in values:
                del values[call["payload"]["name"]]
            else:
                return done(argv, 1, "", NOT_FOUND)
            return done(argv, 0, OK, "")
        if kind == "regread":
            values = self.keys.get(call["payload"]["key"], {})
            name = call["payload"]["name"]
            if name not in values:
                return done(argv, 1, "", NOT_FOUND)
            return done(argv, 0, f"\r\nHKEY_CURRENT_USER\\{call['payload']['key']}\r\n    {name}    REG_BINARY    "
                                 f"{values[name]}\r\n", "")
        presence = re.fullmatch(r'reg query "HKCU\\([^"]+)" >nul 2>nul & if errorlevel 1 \(echo SWBPIPE-KEY v1 absent\) '
                                r'else \(echo SWBPIPE-KEY v1 present\)', command)
        if presence:  # reg's own output and message discarded; its exit status picks the sentinel
            present = presence[1] in self.keys
            return done(argv, 0 if present else 1, f"SWBPIPE-KEY v1 {'present' if present else 'absent'}\r\n", "")
        listing = re.fullmatch(r'if exist "([^"]+)\\" \(dir /b /a "([^"]+)"( 2>nul)?\) else '
                               r'\(echo SWBPIPE-LIST v1 absent\)', command)
        if listing:
            folder = listing[2]
            assert listing[1] == folder
            if folder not in self.guest.dirs:
                return done(argv, 0, "SWBPIPE-LIST v1 absent\r\n", "")
            names = sorted({path.rsplit("\\", 1)[1] for path in (*self.guest.files, *self.guest.dirs)
                            if path.rsplit("\\", 1)[0] == folder})
            if names:
                return done(argv, 0, "".join(f"{name}\r\n" for name in names), "")
            # dir found nothing: exit 1, its message on stderr in the display language, unless discarded
            return done(argv, 1, "", "" if listing[3] else DIR_EMPTY_MESSAGES[self.language])
        return None


def _exercise(tmp_path, clock, **registry):
    config = config_for(tmp_path)
    guest = guest_for(config, clock)
    registry_ = ExerciseRegistry(guest, **registry)
    harness.install_guest_scripts(config)
    steps: list = []

    def send(name: str, command: str) -> dict:
        return harness._guest_step(config, guest.runner, 20.0, steps, name, command, attempts=1)

    def write(name: str, command: str) -> dict:
        return harness._guest_write(config, guest.runner, 20.0, steps, name, command)
    return config, guest, registry_, send, write


def test_the_exercise_names_are_its_own_and_dated():
    assert NAMES == {"folder": "C:\\oracle\\SWBPIPE-harness-exercise-20261121",
                     "key": "HKCU\\Software\\SWBPIPE-harness-exercise-20261121",
                     "printed_key": "HKEY_CURRENT_USER\\Software\\SWBPIPE-harness-exercise-20261121",
                     "registry_key": "Software\\SWBPIPE-harness-exercise-20261121"}
    for date in ("2026-11-21", "20261321", "2026112", "", "20261121\\..\\x"):
        with pytest.raises(ValueError):
            harness.exercise_names(date)


def test_the_exercise_refuses_any_other_name_key_or_value(tmp_path):
    config = config_for(tmp_path)
    for name in ("", "a b", "x\\..\\y", "a" * 41, "C:"):
        with pytest.raises(ValueError):
            harness.exercise_claim_command(config, DATE, name)
    for action, value, data in (("keycreate", "v", None), ("set", "C:\\Invented\\", "0A"), ("set", None, "0A"),
                                ("rename", "v", None), ("read", "a\\b", None)):
        with pytest.raises(ValueError):
            harness.exercise_registry_command(config, DATE, action, value, data)
    # Every key operation it can build names the dated key and nothing else.
    for action in ("keycreate", "keydelete"):
        assert write_call(harness.exercise_registry_command(config, DATE, action))["payload"] == {
            "key": NAMES["registry_key"]}
    assert write_call(harness.exercise_registry_command(config, DATE, "set", "v", "0A"))["payload"]["key"] == \
        NAMES["registry_key"]


def test_two_competing_exercise_claims_on_one_name_and_exactly_one_wins(tmp_path, clock):
    config, guest, _, send, _ = _exercise(tmp_path, clock)
    command = harness.exercise_claim_command(config, DATE, "competition")
    answers = []
    for name in ("first", "second"):
        record = send(name, command)
        answers.append(harness.CLAIM_ANSWER.match(harness._body(record))["result"] if harness._answered(record) else None)
    assert answers == ["created", "exists"]
    path = NAMES["folder"] + "\\SWBPIPE-harness-exercise-competition"
    assert guest.files[path].startswith(b"swbpipe-claim v1\r\nexercise=SWBPIPE-harness-exercise-competition\r\n")
    guest.locked.add(NAMES["folder"] + "\\SWBPIPE-harness-exercise-held")  # a name another holds
    held = send("held", harness.exercise_claim_command(config, DATE, "held"))
    assert harness.CLAIM_ANSWER.match(harness._body(held))["result"] == "held"


def test_the_exercise_start_uses_the_runs_own_start_path_and_is_seen_live_then_quiescent(tmp_path, clock):
    config, guest, _, send, _ = _exercise(tmp_path, clock)
    token = "e" * 32
    guest.exercise_lingers = True  # alive when first seen from a separate call
    seen: list = []

    def job_from_a_separate_call():
        if not seen:
            record = send("job_live", harness._read_command(config, "job", {"job": f"swbpipe-{token}"}))
            seen.append(harness.JOB_ANSWER.match(harness._body(record))["state"])
            for process in guest.processes.values():  # then the harmless process ends
                if process.job == f"swbpipe-{token}":
                    process.alive = False
    guest.hooks[("cim", 1)] = job_from_a_separate_call
    result = harness.run_exercise_start(config, DATE, runner=guest.runner, token=token)
    assert seen == ["present active=1"]
    assert result["disposition"] == {"start": "completed", "restore": True, "retry": False}
    assert result["report_status"] == "exercise_ended" and guest.exercise_starts == [token] and guest.launches == []
    attempt = next(command for command in guest.commands if classify(command) == "attempt")
    assert " -Exercise " in attempt and " -Exe " not in attempt and " -Mbf " not in attempt
    assert f'-WorkRoot "{NAMES["folder"]}"' in attempt
    report = harness.read_state_report(config, runner=guest.runner, token=token, exercise_date=DATE)
    assert report["attempt"]["success"] and report["attempt"]["terminal_status"] == "exercise_ended"
    assert report["exercise"]["folder_entries"] == sorted(
        [f"SWBPIPE-harness-exercise-{token}.claim", f"SWBPIPE-harness-exercise-{token}.finished",
         f"SWBPIPE-harness-exercise-{token}.job", f"SWBPIPE-harness-exercise-{token}"])


def test_the_exercise_registry_steps_answer_as_required_and_leave_the_key_absent_again(tmp_path, clock):
    config, guest, registry, send, write = _exercise(tmp_path, clock)
    report = lambda: harness.read_state_report(config, runner=guest.runner, exercise_date=DATE)["exercise"]  # noqa: E731
    assert report()["key"] == "absent"
    assert write("create", harness.exercise_registry_command(config, DATE, "keycreate"))["outcome"] == "completed"
    assert report()["key"] == "present"
    assert write("set", harness.exercise_registry_command(config, DATE, "set", "value", "0102"))["outcome"] == \
        "completed"
    read = lambda: harness._read_value(send, "read", NAMES["printed_key"], "value",  # noqa: E731
                                       harness.exercise_registry_command(config, DATE, "read", "value"))
    assert read() == ("present", ("REG_BINARY", "0102"))
    assert write("restore", harness.exercise_registry_command(config, DATE, "delete", "value"))["outcome"] == \
        "completed"
    assert read() == ("absent", None)
    assert write("remove", harness.exercise_registry_command(config, DATE, "keydelete"))["outcome"] == "completed"
    assert report()["key"] == "absent" and registry.keys == {}
    assert registry.changes == ["keycreate", "regset", "regdelete", "keydelete"]


def test_an_exercise_key_needing_elevation_is_a_stop_with_no_fallback(tmp_path, clock):
    config, guest, registry, _, write = _exercise(tmp_path, clock, denied=True)
    with pytest.raises(harness.RestoreWriteFailed) as raised:
        write("create", harness.exercise_registry_command(config, DATE, "keycreate"))
    assert raised.value.outcome == "failed" and registry.keys == {} and registry.changes == []
    assert [classify(command) for command in guest.commands] == ["keywrite"]  # nothing else was tried


def test_the_exercise_folder_listing_reports_what_was_left(tmp_path, clock):
    config, guest, _, send, _ = _exercise(tmp_path, clock)
    assert harness.read_state_report(config, runner=guest.runner, exercise_date=DATE)["exercise"][
        "folder_entries"] == "absent"
    send("claim", harness.exercise_claim_command(config, DATE, "left"))
    assert harness.read_state_report(config, runner=guest.runner, exercise_date=DATE)["exercise"][
        "folder_entries"] == ["SWBPIPE-harness-exercise-left"]


def test_every_command_of_the_exercise_and_the_report_is_within_the_limit(tmp_path):
    config = dataclasses.replace(config_for(tmp_path), guest_exchange_root="X:")
    commands = [harness.exercise_claim_command(config, DATE, "x" * 40),
                harness.exercise_start_command(config, DATE, "f" * 32),
                *(harness.exercise_registry_command(config, DATE, action) for action in ("keycreate", "keydelete")),
                harness.exercise_registry_command(config, DATE, "set", "v" * 60, "AB" * 200),
                harness.exercise_registry_command(config, DATE, "read", "v" * 60),
                harness._read_command(config, "storage", {"folders": [NAMES["folder"], "C:\\oracle\\work"]})]
    assert all(harness._guest_command_length(config.vm_name, command) <= harness.GUEST_COMMAND_LIMIT
               for command in commands)


# --------------------------------------------------------------------------------------------------------------
# Amendment 6: the exercise folder's listing and the exercise key's presence read no message text, in any language.
# --------------------------------------------------------------------------------------------------------------
@pytest.mark.parametrize("language", list(DIR_EMPTY_MESSAGES))
def test_the_folder_listing_is_absent_empty_or_its_entries_in_any_display_language(tmp_path, clock, language):
    config, guest, _, send, _ = _exercise(tmp_path, clock, language=language)
    listing = lambda: harness.read_state_report(config, runner=guest.runner, exercise_date=DATE)[  # noqa: E731
        "exercise"]["folder_entries"]
    assert listing() == "absent"
    guest.dirs.add(NAMES["folder"])  # present, with nothing in it
    assert listing() == []
    send("claim", harness.exercise_claim_command(config, DATE, "left"))
    assert listing() == ["SWBPIPE-harness-exercise-left"]


@pytest.mark.parametrize("language", list(DIR_EMPTY_MESSAGES))
def test_the_exercise_key_presence_reads_no_message_text(tmp_path, clock, language):
    config, guest, registry, _, write = _exercise(tmp_path, clock, language=language)
    presence = lambda: harness.read_state_report(config, runner=guest.runner, exercise_date=DATE)[  # noqa: E731
        "exercise"]["key"]
    assert presence() == "absent"
    write("create", harness.exercise_registry_command(config, DATE, "keycreate"))
    assert presence() == "present"


@pytest.mark.parametrize("answer, expected", [
    ((0, "SWBPIPE-LIST v1 absent\r\n", ""), "absent"),
    ((0, "a.claim\r\nSWBPIPE-harness-exercise-x\r\n", ""), ["a.claim", "SWBPIPE-harness-exercise-x"]),
    ((1, "", ""), []),
    ((1, "", "File Not Found"), "unreadable"),          # any text on stderr, in any wording: never read as empty
    ((1, "", "Datei nicht gefunden"), "unreadable"),
    ((0, "", ""), "unreadable"),                         # exit 0 with nothing: neither the sentinel nor names
    ((0, "SWBPIPE-LIST v1 absent\r\nx\r\n", ""), "unreadable"),
    ((255, "", TRANSPORT[0]), "unreadable"),
    ((2, "", ""), "unreadable"),
])
def test_the_folder_listing_is_classified_by_exit_status_and_the_shape_of_stdout(answer, expected):
    code, stdout, stderr = answer
    assert harness._folder_listing({"exit_status": code, "stdout": stdout, "stderr": stderr}) == expected


@pytest.mark.parametrize("answer, expected", [
    ((0, "SWBPIPE-KEY v1 present\r\n", ""), "present"), ((0, "SWBPIPE-KEY v1 absent\r\n", ""), "absent"),
    ((1, "SWBPIPE-KEY v1 absent\r\n", ""), "absent"), ((1, "SWBPIPE-KEY v1 present\r\n", ""), "unreadable"),
    ((1, "", NOT_FOUND), "unreadable"), ((1, "", "FEHLER: Der angegebene Registrierungsschluessel wurde nicht gefunden."),
                                         "unreadable"),
    ((0, "SWBPIPE-KEY v1 absent\r\n", "#< CLIXML"), "unreadable"), ((255, "", TRANSPORT[1]), "unreadable"),
])
def test_the_key_presence_is_classified_by_its_sentinel_alone(answer, expected):
    code, stdout, stderr = answer
    assert harness._key_presence({"exit_status": code, "stdout": stdout, "stderr": stderr}) == expected


def test_no_listing_or_presence_classification_matches_message_text():
    import inspect
    for function in (harness._folder_listing, harness._key_presence, harness.read_state_report,
                     harness.exercise_folder_listing_command, harness.exercise_key_presence_command):
        source = inspect.getsource(function)
        assert "File Not Found" not in source and "_not_found(" not in source and "REG_NOT_FOUND" not in source
    assert " 2>nul" in harness.exercise_folder_listing_command(NAMES["folder"])
    assert ">nul 2>nul" in harness.exercise_key_presence_command(NAMES["key"])


# --------------------------------------------------------------------------------------------------------------
# The second reading's remedies F2 and F3 (reviews/harness-second-reading/reader/review.md), as the reader wrote them.
# --------------------------------------------------------------------------------------------------------------
def test_regdelete_deletes_only_a_reg_binary_value():
    source = harness.WRITE_SCRIPT.read_text(encoding="utf-8")
    branch = source[source.index("    } else {\n      if (-not ($key.GetValueNames()"):source.index("$key.DeleteValue(")]
    assert "GetValueKind" in branch and "RegistryValueKind]::Binary" in branch


def test_every_confining_pattern_ends_at_the_end_of_the_string():
    """.NET's `$` also matches before a final line feed; `\\z` does not. A pattern that confines a changing call must
    end with `\\z` (or the string be refused for any control character first)."""
    WRITE_SCRIPT = harness.WRITE_SCRIPT.read_text(encoding="utf-8")  # noqa: N806 - the reader's name
    patterns = re.findall(r"'(\^[^']*)'", WRITE_SCRIPT)
    loose = [pattern for pattern in patterns if pattern.endswith('$')]
    assert not loose, loose


def test_regset_is_described_as_replacing_a_value_of_another_type():
    header = WRITE_SOURCE[:WRITE_SOURCE.index("#>")]
    assert "a value of another type is\n             replaced" in header
    assert "set to REG_BINARY data, or deleted\n    when it is REG_BINARY;" in header
