"""The read script's job query on hosted Windows (DEL-17-05): the exact maintained script, by digest, invoked as the
harness invokes it, through `cmd /c` and Windows PowerShell 5.1, against named jobs this test creates and closes.

It runs on Windows (`.github/workflows/piping-windows-read-script.yml`, on windows-11-arm and windows-2025); its
Windows tests are skipped elsewhere. It shows, on the runner, that the job query answers `absent` for a name that does
not exist (which needs the canary to read CloseHandle(NULL)'s 6), live while a separately created job is held, and
`absent` again once every handle is closed. Then, and only then, each control is evaluated against ITS exact answers
(`CONTROL_ANSWERS`); before the maintained script has passed on the runner a control is not evaluated.

The command line reaches cmd verbatim, as one string: `cmd /c <command>`. (A list would be joined by `list2cmdline`,
which escapes the command's own double quotes as `\\"`; cmd keeps those, and PowerShell would read `-File` as a path
with quote characters in it.) Every installed script is read back and hashed before it runs.

What it prints is public: the test's own outcomes, one line per script variant, and on an unresolved answer the exit
code and the bounded repr of stdout and stderr. The runner's versions are printed by the workflow's own step.
"""

from __future__ import annotations

import ctypes
import hashlib
import re
import secrets
import subprocess
import sys
from types import SimpleNamespace
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.handoff.caepipe_external import harness  # noqa: E402
from core.handoff.caepipe_external.harness import HarnessConfig  # noqa: E402

windows_only = pytest.mark.skipif(sys.platform != "win32", reason="the read script's job query runs on Windows only")

SCRIPT_RELATIVE = "core/handoff/caepipe_external/guest/Read-SwbpipeGuest.ps1"
ERROR_ALREADY_EXISTS = 183


# --------------------------------------------------------------------------------------------------------------
# The variants: the maintained script, and the controls, each made from it by one exact replacement.
# --------------------------------------------------------------------------------------------------------------
def _replace_once(text: str, old: str, new: str) -> str:
    assert text.count(old) == 1, old[:80]
    return text.replace(old, new)


def _always_absent(text: str) -> str:
    return _replace_once(text, "  $job = $n::OpenJobChecked(", '  return "SWBPIPE-JOB v1 absent`r`n"\n'
                                                              "  $job = $n::OpenJobChecked(")


def _stale_error(text: str) -> str:
    # The helper reads the last error before OpenJobObject instead of after it: what it reads is the canary's.
    return _replace_once(text, '''  $il.Emit($o::Ldarg_1)
  $il.Emit($o::Ldc_I4, [int]4)
  $il.Emit($o::Ldc_I4_0)
  $il.Emit($o::Ldarg_0)
  $il.Emit($o::Call, [Reflection.MethodInfo]$native['OpenJobObject'])
  $il.Emit($o::Stloc, $handle)
  $il.Emit($o::Call, $lastError)
  $il.Emit($o::Stind_I4)
''', '''  $il.Emit($o::Ldarg_1)
  $il.Emit($o::Call, $lastError)
  $il.Emit($o::Stind_I4)
  $il.Emit($o::Ldc_I4, [int]4)
  $il.Emit($o::Ldc_I4_0)
  $il.Emit($o::Ldarg_0)
  $il.Emit($o::Call, [Reflection.MethodInfo]$native['OpenJobObject'])
  $il.Emit($o::Stloc, $handle)
''')


def _previous_definition_form(text: str) -> str:
    return _replace_once(text, "    $m = $tb.DefineMethod($d[0], 'Public,Static,PinvokeImpl', $d[1], [Type[]]$d[2])\n",
                         "    $m = $tb.DefinePInvokeMethod($d[0], 'kernel32.dll', 'Public,Static,PinvokeImpl', "
                         "'Standard', $d[1],\n                                 [Type[]]$d[2], 'Winapi', 'Unicode')\n")


VARIANTS = {"maintained": None, "control: always absent": _always_absent,
            "control: stale error": _stale_error, "control: previous definition form": _previous_definition_form}
PRESENT = r"present active=(?:[0-9]+|unknown)"
CANARY_NOT_6 = r"error=canary-(?!6\Z)-?[0-9]+"
# Each control's exact answers (fresh name, held job, after close), each a full-match pattern. Its reason:
# - always absent: it answers absent even for the job the test holds;
# - stale error: its helper stores the last error before the open, which is the canary's 6, so a job it cannot open
#   reads error=6 (never absent), and a job it can open is still present (a handle is presence);
# - previous definition form: DefinePInvokeMethod writes the P/Invoke map without the last-error flag, so the canary
#   does not read 6 and a job it cannot open reads error=canary-<n>, n not 6; a job it can open is still present.
CONTROL_ANSWERS = {"control: always absent": ("absent", "absent", "absent"),
                   "control: stale error": ("error=6", PRESENT, "error=6"),
                   "control: previous definition form": (CANARY_NOT_6, PRESENT, CANARY_NOT_6)}
MAINTAINED_ANSWERS = ("absent", PRESENT, "absent")


# --------------------------------------------------------------------------------------------------------------
# A job held by a separately established helper: Python's ctypes, with the last error kept per call.
# --------------------------------------------------------------------------------------------------------------
def _kernel32():
    kernel32 = ctypes.WinDLL("kernel32", use_last_error=True)
    kernel32.CreateJobObjectW.restype = ctypes.c_void_p
    kernel32.CreateJobObjectW.argtypes = [ctypes.c_void_p, ctypes.c_wchar_p]
    kernel32.CloseHandle.restype = ctypes.c_int
    kernel32.CloseHandle.argtypes = [ctypes.c_void_p]
    return kernel32


def _hold_job(name: str) -> int:
    kernel32 = _kernel32()
    handle = kernel32.CreateJobObjectW(None, name)
    error = ctypes.get_last_error()
    assert handle, f"CreateJobObjectW failed for a fresh name (error {error})"
    assert error != ERROR_ALREADY_EXISTS, "the fresh name already named a job"
    return handle


def _close(handle: int) -> None:
    assert _kernel32().CloseHandle(handle), "CloseHandle failed"


# --------------------------------------------------------------------------------------------------------------
# The invocation: the command the harness builds, through cmd /c, as the guest receives it.
# --------------------------------------------------------------------------------------------------------------
def _repository_bytes() -> bytes:
    return subprocess.run(["git", "cat-file", "blob", f"HEAD:{SCRIPT_RELATIVE}"], cwd=ROOT, capture_output=True,
                          check=True).stdout


def _install(root: Path, text: bytes | None) -> str:
    """Put the maintained script (or a variant's bytes) at its content-addressed path, then read the installed file
    back and hash it: its digest, the one the script will echo."""
    harness_dir = root / harness.HARNESS_DIR
    intended = harness.READ_SCRIPT.read_bytes() if text is None else text
    if text is None:
        digest = harness._install_guest_script(root, harness_dir, harness.READ_SCRIPT)["sha256"]
    else:
        digest = hashlib.sha256(text).hexdigest()
        harness._install_add_only(root, harness_dir / digest, harness.READ_SCRIPT.name, text, digest)
    on_disk = (harness_dir / digest / harness.READ_SCRIPT.name).read_bytes()
    assert on_disk == intended and hashlib.sha256(on_disk).hexdigest() == digest, "the installed bytes differ"
    return digest


def invocation(command: str) -> str:
    """The command line as the guest's cmd receives it: `cmd /c <command>`, one string, passed verbatim."""
    return "cmd /c " + command


def _bounded(text: str | None) -> str:
    return repr((text or "")[:400])


def _job_state(config: HarnessConfig, digest: str, name: str) -> str:
    command = harness._read_command(config, "job", {"job": name}).replace(harness.READ_SCRIPT_SHA256, digest)
    completed = subprocess.run(invocation(command), capture_output=True, text=True, timeout=120)
    record = {"guest_command": command, "exit_status": completed.returncode, "stdout": completed.stdout,
              "stderr": completed.stderr}
    detail = (f"exit {completed.returncode}, stdout {_bounded(completed.stdout)}, "
              f"stderr {_bounded(completed.stderr)}")
    echo = harness.READ_ECHO.match(completed.stdout or "")
    if echo is None or echo["script"] != digest or echo["data"] != "none":
        return f"unresolved (echo; {detail})"
    body = completed.stdout[echo.end():]
    if digest == harness.READ_SCRIPT_SHA256:
        harness._settle_echo(record)  # the harness's own reading, for the maintained script
        assert record["echo"]["verified"]
        body = harness._body(record)
    match = harness.JOB_ANSWER.match(body) if completed.returncode == 0 and not completed.stderr.strip() else None
    return match["state"] if match else f"unresolved ({detail})"


def _three_answers(config: HarnessConfig, digest: str) -> tuple[str, str, str]:
    fresh = f"swbpipe-{secrets.token_hex(16)}"
    first = _job_state(config, digest, fresh)
    held_name = f"swbpipe-{secrets.token_hex(16)}"
    handle = _hold_job(held_name)
    try:
        held = _job_state(config, digest, held_name)
    finally:
        _close(handle)  # every handle this test holds
    released = _job_state(config, digest, held_name)
    return first, held, released


def _matches(answers: tuple[str, str, str], expected: tuple[str, str, str]) -> bool:
    return all(re.fullmatch(pattern, answer) is not None for answer, pattern in zip(answers, expected))


def _report(variant: str, answers: tuple[str, str, str], verdict: str) -> None:
    print(f"\n{variant}: fresh name -> {answers[0]}; held job -> {answers[1]}; after close -> {answers[2]}: {verdict}")


@pytest.fixture(scope="module")
def config(tmp_path_factory) -> HarnessConfig:
    root = tmp_path_factory.mktemp("exchange")
    return HarnessConfig(exchange_root=root, vm_name="hosted-runner", guest_executable="C:\\Invented\\tool.exe",
                         guest_exchange_root=str(root), guest_work_root="C:\\invented-work", prlctl=None,
                         acknowledged=True, trial_first_launch_date=None)


@pytest.fixture(scope="module")
def maintained(config) -> tuple[str, str, str]:
    """The maintained script's three answers on this runner, taken once."""
    digest = _install(config.exchange_root, None)
    assert digest == harness.READ_SCRIPT_SHA256
    return _three_answers(config, digest)


@windows_only
def test_the_script_run_is_the_maintained_script_byte_for_byte():
    data = harness.READ_SCRIPT.read_bytes()
    assert data == _repository_bytes(), "the checkout changed the read script's bytes"
    assert hashlib.sha256(data).hexdigest() == harness.READ_SCRIPT_SHA256


@windows_only
def test_the_maintained_job_query_on_this_runner(maintained):
    passed = _matches(maintained, MAINTAINED_ANSWERS)
    _report("maintained", maintained, "passes" if passed else "FAILS")
    assert passed, maintained


@windows_only
@pytest.mark.parametrize("variant", list(CONTROL_ANSWERS))
def test_each_control_gives_its_own_answers_on_this_runner(config, maintained, variant):
    if not _matches(maintained, MAINTAINED_ANSWERS):
        print(f"\n{variant}: not evaluated (the maintained script has not passed on this runner)")
        pytest.skip("not evaluated: the maintained script has not passed on this runner")
    text = VARIANTS[variant](_repository_bytes().decode("utf-8")).encode("utf-8")
    answers = _three_answers(config, _install(config.exchange_root, text))
    if _matches(answers, CONTROL_ANSWERS[variant]):
        _report(variant, answers, "killed for its reason")
        return
    if variant == "control: previous definition form" and _matches(answers, MAINTAINED_ANSWERS):
        _report(variant, answers, "FINDING: the previous definition form saved the last error on this runner")
        pytest.fail("FINDING: the previous definition form saved the last error on this runner")
    _report(variant, answers, "not killed for its reason")
    pytest.fail(f"{variant} did not give its own answers {CONTROL_ANSWERS[variant]}: {answers}")


def test_the_runner_passes_the_command_line_verbatim():
    command = harness._read_command(SimpleNamespace(vm_name="hosted-runner", guest_exchange_root="D:\\a\\x"), "job",
                                    {"job": "swbpipe-" + "0" * 32})
    line = invocation(command)
    assert isinstance(line, str) and line == "cmd /c " + command and '\\"' not in line
    # What a list would have become: the command's own quotes escaped, which cmd keeps and PowerShell misreads.
    assert '\\"' in subprocess.list2cmdline(["cmd", "/c", command])
    # And every query this test sends goes through that one string, never a list.
    import inspect
    source = inspect.getsource(_job_state)
    assert "subprocess.run(invocation(command)," in source and '["cmd"' not in source


def test_each_control_names_answers_that_differ_from_the_maintained_script():
    for variant, expected in CONTROL_ANSWERS.items():
        assert expected != MAINTAINED_ANSWERS, variant
    assert re.fullmatch(CANARY_NOT_6, "error=canary-0") and not re.fullmatch(CANARY_NOT_6, "error=canary-6")
    assert re.fullmatch(CANARY_NOT_6, "error=canary-60") and not re.fullmatch(CANARY_NOT_6, "error=6")


def test_the_controls_differ_from_the_maintained_script():
    text = harness.READ_SCRIPT.read_text(encoding="utf-8")
    for name, mutate in VARIANTS.items():
        if mutate is not None:
            assert mutate(text) != text, name
