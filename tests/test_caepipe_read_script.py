"""The read script and its short commands (DEL-17-05): the echo, the data files, the one length rule, and the script's
read-only construction. Against the fake guest only: no prlctl, no virtual machine; everything here is invented."""

from __future__ import annotations

import base64
import json
import dataclasses
import re
import sys
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.handoff.caepipe_external import harness  # noqa: E402
from core.handoff.caepipe_external.harness import StartUnresolved  # noqa: E402
from tests.caepipe_fake_guest import FakeGuestModel, done, read_call  # noqa: E402
from tests.test_caepipe_start_reconciliation import clock, config_for, guest_for, run  # noqa: E402

__all__ = ["clock"]  # the fixture, used by name

READ_SOURCE = harness.READ_SCRIPT.read_text(encoding="utf-8")
ECHO = f"SWBPIPE-READ v1 script={harness.READ_SCRIPT_SHA256} data=none\r\n"


# --------------------------------------------------------------------------------------------------------------
# Read-only by construction: a guard on the script's source (an independent reading checks the property itself).
# --------------------------------------------------------------------------------------------------------------
# The deny-list: each entry is (what it names, a pattern over the source with comments removed).
DENY_LIST = [
    ("Set-", r"\bSet-"), ("New-", r"\bNew-"), ("Remove-", r"\bRemove-"), ("Move-", r"\bMove-"),
    ("Copy-", r"\bCopy-"), ("Rename-", r"\bRename-"), ("Out-File", r"\bOut-File\b"),
    ("Start-Process", r"\bStart-Process\b"), ("Invoke-Expression", r"\bInvoke-Expression\b"), ("iex", r"\biex\b"),
    ("Add-Type", r"\bAdd-Type\b"),
    ("[IO.File]::Open with a write mode", r"\[(?:System\.)?IO\.File\]::Open\([^)]*(?:Write|Append|Create|Truncate)"),
    ("CreateNew", r"CreateNew"), ("WriteAll", r"WriteAll"),
    ("OpenSubKey with true", r"OpenSubKey\([^)]*\$true"), ("SetValue", r"SetValue"), ("DeleteValue", r"DeleteValue"),
    ("CreateDirectory", r"CreateDirectory"), ("Process]::Start", r"Process\]::Start"), ("cmd", r"\bcmd\b"),
]
LOCAL_FUNCTIONS = set(re.findall(r"^function ([A-Za-z]+)", READ_SOURCE, re.MULTILINE))


def _code(source: str) -> str:
    """The script without its comments (the header block and `#` comments), so the list is matched against code."""
    source = re.sub(r"<#.*?#>", "", source, flags=re.DOTALL)
    return "\n".join(line.split(" #", 1)[0] if not line.lstrip().startswith("#") else "" for line in
                     source.splitlines())


@pytest.mark.parametrize("name, pattern", DENY_LIST, ids=[name for name, _ in DENY_LIST])
def test_the_read_script_holds_no_member_of_the_deny_list(name, pattern):
    assert re.search(pattern, _code(READ_SOURCE), re.IGNORECASE) is None, name


def test_the_read_script_invokes_nothing_but_its_own_functions_and_compiles_nothing():
    code = _code(READ_SOURCE)
    for invoked in re.findall(r"&\s*([^\s(]+)", code):
        assert invoked in LOCAL_FUNCTIONS, invoked  # `&` only on a function the script defines (there is none now)
    assert "&" not in code
    # Native calls through Reflection.Emit in memory: a dynamic assembly with Run access only, never saved.
    assert "DefineDynamicAssembly(" in code and "'Run')" in code and "RunAndSave" not in code and ".Save(" not in code


def test_the_deny_list_catches_what_it_names():
    for line in ("Set-ItemProperty -Path x", "New-Item x", "Add-Type 'x'", "[IO.File]::Open($p, 'Create')",
                 "$k.OpenSubKey($n, $true)", "$k.SetValue('a', 1)", "cmd /c del x", "iex $x",
                 "[Diagnostics.Process]::Start('x')", "[IO.File]::WriteAllBytes($p, $b)"):
        assert any(re.search(pattern, line, re.IGNORECASE) for _, pattern in DENY_LIST), line
    for line in ("[IO.File]::Open($path, [IO.FileMode]::Open, [IO.FileAccess]::Read, [IO.FileShare]::Read)",
                 "$key = [Microsoft.Win32.Registry]::CurrentUser.OpenSubKey([string]$p.key)"):
        assert not any(re.search(pattern, line, re.IGNORECASE) for _, pattern in DENY_LIST), line


def test_the_read_script_confines_every_path_it_opens():
    code = _code(READ_SOURCE)
    for op in ("OpChunk", "OpStorage", "OpHash"):
        body = code[code.index(f"function {op}"):]
        body = body[:body.index("\nfunction ", 1)] if "\nfunction " in body[1:] else body
        assert "Confined (" in body, op
    assert "Confined ($root" in code  # the data file too


# --------------------------------------------------------------------------------------------------------------
# The short command and the one length rule.
# --------------------------------------------------------------------------------------------------------------
@pytest.mark.parametrize("payload", [{"pid": 4294967295}, {"job": "swbpipe-" + "0" * 32},
                                     {"path": "C:\\a b\\c (1)\\d.csv", "offset": 0, "length": 1 << 20},
                                     {"key": "Software\\Invented\\Recent list", "name": "C:\\Program Files (x86)\\x\\"},
                                     {"folders": ["C:\\x", "Z:\\y"]}, {"text": "é ü \u4e2d \\ \" '"}])
def test_payload_decoding_is_the_exact_inverse_of_the_builders_encoding(tmp_path, payload):
    config = config_for(tmp_path)
    encoded = harness._encode_payload(payload)
    assert harness._decode_payload(encoded) == payload
    assert re.fullmatch(r"[A-Za-z0-9+/]+={0,2}", encoded)  # no character cmd or PowerShell would read
    command = harness._read_command(config, "hash", payload)
    form = harness.READ_COMMAND_FORM.match(command)
    assert form and harness._decode_payload(form["payload"]) == payload and form["op"] == "hash"
    assert read_call(command)["payload"] == payload  # and the fake guest decodes it the same way
    with pytest.raises(ValueError):
        harness._decode_payload(base64.b64encode(b"[1]").decode("ascii"))


def _command_of_length(config, length: int) -> str:
    """A changing-free guest command whose measured length is exactly `length`."""
    base = harness._guest_command_length(config.vm_name, "ver ")
    return "ver " + "x" * (length - base)


@pytest.mark.parametrize("send", ["read", "write"])
def test_a_command_one_character_over_the_limit_is_refused_before_any_send(tmp_path, send):
    config = config_for(tmp_path)
    sent: list = []

    def runner(argv, **kwargs):
        sent.append(argv)
        return done(argv, 0, "The operation completed successfully.", "")
    fits, over = _command_of_length(config, harness.GUEST_COMMAND_LIMIT), \
        _command_of_length(config, harness.GUEST_COMMAND_LIMIT + 1)
    assert harness._guest_command_length(config.vm_name, over) == harness.GUEST_COMMAND_LIMIT + 1
    steps: list = []
    call = (lambda command: harness._guest_step(config, runner, 5.0, steps, "s", command, attempts=1)) \
        if send == "read" else (lambda command: harness._guest_write(config, runner, 5.0, steps, "w", command))
    call(fits)
    assert len(sent) == 1
    with pytest.raises(ValueError, match="exceeds GUEST_COMMAND_LIMIT"):
        call(over)
    assert len(sent) == 1 and len(steps) == 1  # nothing sent, nothing recorded as sent
    with pytest.raises(ValueError, match="exceeds GUEST_COMMAND_LIMIT"):
        harness._read_command(config, "hash", {"path": "C:\\" + "x" * 900})


def test_every_send_builds_its_argv_in_the_one_place():
    source = harness.__loader__.get_source(harness.__name__)
    assert source.count('[config.prlctl, "exec"') == 1  # `_guest_argv`, which applies `_checked_command`
    assert source.count("argv = _guest_argv(config, command)") == 2  # `_guest_step` and `_guest_write`
    assert len(re.findall(r"\brunner\(argv", source)) == 2  # the only two calls of the runner
    # The module's one direct process call is the host's own Git check, never a guest command.
    assert source.count("subprocess.run(") == source.count('subprocess.run(["git", ') == 1


def test_an_attempt_command_over_the_limit_refuses_before_its_start_becomes_unresolved(tmp_path, clock):
    config = config_for(tmp_path)
    config = dataclasses.replace(config, guest_work_root="C:\\" + "w" * 700)
    guest = guest_for(config, clock)
    with pytest.raises(ValueError, match="exceeds GUEST_COMMAND_LIMIT") as raised:
        run(config, guest)
    assert guest.kinds == [] and raised.value.disposition["start"] == "not_dispatched"


# --------------------------------------------------------------------------------------------------------------
# The echo: a reply whose echo is missing, malformed or not the expected one is unresolved, never an answer.
# --------------------------------------------------------------------------------------------------------------
def _reply(config, command: str, stdout: str, code: int = 0, stderr: str = "") -> dict:
    steps: list = []
    return harness._guest_step(config, lambda argv, **k: done(argv, code, stdout, stderr), 5.0, steps, "r", command,
                               attempts=1)


def test_a_reply_in_form_with_its_echo_is_an_answer(tmp_path):
    config = config_for(tmp_path)
    command = harness._read_command(config, "cim", {"pid": 1})
    record = _reply(config, command, ECHO + "SWBPIPE-CIM v1 none\r\n")
    assert record["echo"]["verified"] and harness._answered(record)
    assert harness.CIM_ANSWER.match(harness._body(record))


@pytest.mark.parametrize("stdout", [
    "SWBPIPE-CIM v1 none\r\n",                                                          # no echo
    "SWBPIPE-READ v1 script=" + "0" * 64 + " data=none\r\nSWBPIPE-CIM v1 none\r\n",      # another script
    f"SWBPIPE-READ v1 script={harness.READ_SCRIPT_SHA256} data={'1' * 64}\r\nSWBPIPE-CIM v1 none\r\n",  # a data file
    f"SWBPIPE-READ v1 script={harness.READ_SCRIPT_SHA256.upper()} data=none\r\nSWBPIPE-CIM v1 none\r\n",  # malformed
    f" SWBPIPE-READ v1 script={harness.READ_SCRIPT_SHA256} data=none\r\nSWBPIPE-CIM v1 none\r\n",  # not first
    f"SWBPIPE-READ v1 script={harness.READ_SCRIPT_SHA256} data=none",                    # no line end, no answer
], ids=["missing", "wrong script digest", "unexpected data digest", "malformed", "not the first line", "echo only"])
def test_a_reply_whose_echo_is_not_the_expected_one_is_unresolved(tmp_path, stdout):
    config = config_for(tmp_path)
    record = _reply(config, harness._read_command(config, "cim", {"pid": 1}), stdout)
    assert not harness._answered(record) and harness._body(record) == ""
    assert record["echo"]["verified"] is False


def test_a_wrong_data_digest_and_changed_data_bytes_are_refused(tmp_path):
    config = config_for(tmp_path)
    harness_dir = config.exchange_root / harness.HARNESS_DIR
    harness._install_guest_script(config.exchange_root, harness_dir, harness.READ_SCRIPT)
    digest = harness._install_data_file(config.exchange_root, harness_dir, {"path": "C:\\Invented\\a.bin"})
    command = harness._read_command(config, "hash", {}, data=digest)
    guest = FakeGuestModel(config.exchange_root)
    guest.files["C:\\Invented\\a.bin"] = b"INVENTED"
    steps: list = []

    def ask() -> dict:
        return harness._guest_step(config, guest.runner, 5.0, steps, "h", command, attempts=1)
    good = ask()
    assert harness._answered(good) and harness.HASH_ANSWER.match(harness._body(good))["size"] == "8"
    # A reply naming another data digest than the command's.
    wrong = _reply(config, command, f"SWBPIPE-READ v1 script={harness.READ_SCRIPT_SHA256} data={'2' * 64}\r\n"
                                    "SWBPIPE-HASH v1 absent\r\n")
    assert not harness._answered(wrong) and harness._body(wrong) == ""
    # The data file's bytes changed after it was installed: the script refuses before using it (exit 1), its echo
    # names the computed digest, and the host refuses the reply.
    path = harness_dir / harness.DATA_DIR / f"{digest}.json"
    path.write_bytes(path.read_bytes().replace(b"a.bin", b"b.bin"))
    changed = ask()
    assert changed["exit_status"] == 1 and not harness._answered(changed) and not harness._not_found(changed)
    assert changed["echo"]["verified"] is False
    # Installing it again refuses: a data file is never overwritten.
    with pytest.raises(harness.ContainmentError):
        harness._install_data_file(config.exchange_root, harness_dir, {"path": "C:\\Invented\\a.bin"})


def test_a_missing_echo_on_every_reconciling_read_leaves_the_start_unresolved(tmp_path, clock):
    config = config_for(tmp_path)
    guest = guest_for(config, clock)
    base = guest.runner

    def runner(argv, **kwargs):
        result = base(argv, **kwargs)
        if read_call(argv[6]) is not None and result.stdout.startswith("SWBPIPE-READ v1 "):
            result = done(argv, result.returncode, result.stdout.split("\r\n", 1)[1], result.stderr)
        return result
    guest.runner = runner
    with pytest.raises(StartUnresolved) as raised:
        run(config, guest)
    reads = [step for step in raised.value.record["steps"] if step["name"].startswith("launcher_query")]
    assert reads and all(step["echo"]["verified"] is False and step["answer"] == "unresolved" for step in reads)


def test_a_wrong_script_digest_in_the_echo_leaves_the_start_unresolved(tmp_path, clock):
    config = config_for(tmp_path)
    guest = guest_for(config, clock)
    run(config, guest)  # installs the read script, and completes
    script = config.exchange_root / harness.HARNESS_DIR / harness.READ_SCRIPT_SHA256 / harness.READ_SCRIPT.name
    script.write_bytes(script.read_bytes() + b"# changed\n")  # the guest would now run other bytes under that name
    guest = guest_for(config, clock)
    with pytest.raises(harness.ContainmentError):  # the host refuses to run with a copy that differs
        run(config, guest)


def test_the_round_trip_installs_the_read_script_add_only_and_records_it(tmp_path, clock):
    config = config_for(tmp_path)
    record = run(config, guest_for(config, clock))
    copy = config.exchange_root / harness.HARNESS_DIR / harness.READ_SCRIPT_SHA256 / harness.READ_SCRIPT.name
    assert copy.read_bytes() == harness.READ_SCRIPT.read_bytes()
    assert record["read_script"]["sha256"] == harness.READ_SCRIPT_SHA256
    reads = [step for step in record["steps"] if harness.READ_COMMAND_FORM.match(step["guest_command"])]
    assert reads and all(step["echo"]["verified"] for step in reads)
    assert not any(" -EncodedCommand " in step["guest_command"] for step in record["steps"])  # no fence was needed


# --------------------------------------------------------------------------------------------------------------
# The storage and hash operations.
# --------------------------------------------------------------------------------------------------------------
def _storage(entry_overrides: dict | None = None, folder: str = "C:\\oracle\\work") -> dict:
    entry = {"folder": folder, "absent": False, "final": "\\\\?\\" + folder, "volume": "C:\\", "format": "NTFS",
             "type": "Fixed", "components": [{"path": "C:\\oracle", "reparse": False, "directory": True},
                                             {"path": folder, "reparse": False, "directory": True}]}
    return entry | (entry_overrides or {})


@pytest.mark.parametrize("override, meets", [
    ({}, True), ({"final": "\\\\?\\D:\\oracle\\work"}, False), ({"final": "\\\\?\\UNC\\host\\share"}, False),
    ({"volume": "C:\\oracle\\"}, False), ({"format": "ReFS"}, False), ({"type": "Network"}, False),
    ({"components": [{"path": "C:\\oracle", "reparse": True, "directory": True}]}, False),
    ({"components": []}, False), ({"final": None}, False),
])
def test_the_storage_predicate(override, meets):
    assert harness.storage_meets(_storage(override)) is meets


def test_a_storage_answer_is_parsed_whole_or_not_at_all(tmp_path):
    folders = ["C:\\oracle\\work", "C:\\oracle\\absent"]
    entries = [_storage(), {"folder": folders[1], "absent": True}]
    parsed = harness.parse_storage("SWBPIPE-STORAGE v1 " + json.dumps(entries) + "\r\n", folders)
    assert [entry["meets"] for entry in parsed] == [True, False]
    for broken in ("SWBPIPE-STORAGE v1 []\r\n", "SWBPIPE-STORAGE v1 " + json.dumps(entries[:1]) + "\r\n",
                   "SWBPIPE-STORAGE v1 " + json.dumps([entries[1], entries[0]]) + "\r\n",
                   "SWBPIPE-STORAGE v1 " + json.dumps([_storage() | {"extra": 1}, entries[1]]) + "\r\n",
                   "SWBPIPE-STORAGE v1 {}\r\n"):
        assert harness.parse_storage(broken, folders) is None, broken
    # Through the fake guest, with the echo.
    config = config_for(tmp_path)
    harness._install_guest_script(config.exchange_root, config.exchange_root / harness.HARNESS_DIR, harness.READ_SCRIPT)
    guest = FakeGuestModel(config.exchange_root)
    guest.dirs.add(folders[0])
    record = harness._guest_step(config, guest.runner, 5.0, [], "storage",
                                 harness._read_command(config, "storage", {"folders": folders}), attempts=1)
    assert harness._answered(record)
    assert [entry["meets"] for entry in harness.parse_storage(harness._body(record), folders)] == [True, False]


def test_the_fake_guest_runs_only_the_installed_read_script(tmp_path):
    config = config_for(tmp_path)
    guest = FakeGuestModel(config.exchange_root)
    record = harness._guest_step(config, guest.runner, 5.0, [], "cim", harness._read_command(config, "cim", {"pid": 1}),
                                 attempts=1)
    assert record["exit_status"] == 1 and not harness._answered(record)  # nothing installed: nothing runs
