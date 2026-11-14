"""The read script's job query on hosted Windows (DEL-17-05): the exact maintained script, by digest, invoked as the
harness invokes it, through `cmd /c` and Windows PowerShell 5.1, against named jobs this test creates and closes.

It runs only on Windows (`.github/workflows/piping-windows-read-script.yml`, on windows-11-arm and windows-2025) and is
skipped elsewhere. It shows, on the runner, that the job query answers `absent` for a name that does not exist (which
needs the canary to read CloseHandle(NULL)'s 6), live while a separately created job is held, and `absent` again once
every handle is closed; and that each control fails: a script that always answers absent, a script whose helper reads
the last error before the open, and the previous definition form (DefinePInvokeMethod, then the attribute).

What it prints is public: the test's own outcomes, one line per script variant. The runner's versions are printed by
the workflow's own step.
"""

from __future__ import annotations

import ctypes
import hashlib
import re
import secrets
import subprocess
import sys
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.handoff.caepipe_external import harness  # noqa: E402
from core.handoff.caepipe_external.harness import HarnessConfig  # noqa: E402

pytestmark = pytest.mark.skipif(sys.platform != "win32", reason="the read script's job query runs on Windows only")

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
    """Put the maintained script (or a variant's bytes) at its content-addressed path; its digest."""
    harness_dir = root / harness.HARNESS_DIR
    if text is None:
        return harness._install_guest_script(root, harness_dir, harness.READ_SCRIPT)["sha256"]
    digest = hashlib.sha256(text).hexdigest()
    harness._install_add_only(root, harness_dir / digest, harness.READ_SCRIPT.name, text, digest)
    return digest


def _job_state(config: HarnessConfig, digest: str, name: str) -> str:
    command = harness._read_command(config, "job", {"job": name}).replace(harness.READ_SCRIPT_SHA256, digest)
    completed = subprocess.run(["cmd", "/c", command], capture_output=True, text=True, timeout=120)
    record = {"guest_command": command, "exit_status": completed.returncode, "stdout": completed.stdout,
              "stderr": completed.stderr}
    echo = harness.READ_ECHO.match(completed.stdout or "")
    if echo is None or echo["script"] != digest or echo["data"] != "none":
        return "unresolved (echo)"
    body = completed.stdout[echo.end():]
    if digest == harness.READ_SCRIPT_SHA256:
        harness._settle_echo(record)  # the harness's own reading, for the maintained script
        assert record["echo"]["verified"]
        body = harness._body(record)
    match = harness.JOB_ANSWER.match(body) if completed.returncode == 0 and not completed.stderr.strip() else None
    return match["state"] if match else f"unresolved (exit {completed.returncode})"


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


def _passes(answers: tuple[str, str, str]) -> bool:
    first, held, released = answers
    return first == "absent" and re.fullmatch(r"present active=(?:[0-9]+|unknown)", held) is not None \
        and released == "absent"


@pytest.fixture(scope="module")
def config(tmp_path_factory) -> HarnessConfig:
    root = tmp_path_factory.mktemp("exchange")
    return HarnessConfig(exchange_root=root, vm_name="hosted-runner", guest_executable="C:\\Invented\\tool.exe",
                         guest_exchange_root=str(root), guest_work_root="C:\\invented-work", prlctl=None,
                         acknowledged=True, trial_first_launch_date=None)


def test_the_script_run_is_the_maintained_script_byte_for_byte():
    data = harness.READ_SCRIPT.read_bytes()
    assert data == _repository_bytes(), "the checkout changed the read script's bytes"
    assert hashlib.sha256(data).hexdigest() == harness.READ_SCRIPT_SHA256


@pytest.mark.parametrize("variant", list(VARIANTS))
def test_the_job_query_on_this_runner(config, variant):
    mutate = VARIANTS[variant]
    text = None if mutate is None else mutate(_repository_bytes().decode("utf-8")).encode("utf-8")
    digest = _install(config.exchange_root, text)
    if mutate is None:
        assert digest == harness.READ_SCRIPT_SHA256
    answers = _three_answers(config, digest)
    outcome = "passes" if _passes(answers) else "fails"
    print(f"\n{variant}: fresh name -> {answers[0]}; held job -> {answers[1]}; after close -> {answers[2]}: {outcome}")
    if mutate is None:
        assert outcome == "passes", answers
    else:
        assert outcome == "fails", f"the control did not fail: {answers}"


def test_the_controls_differ_from_the_maintained_script():
    text = _repository_bytes().decode("utf-8")
    for name, mutate in VARIANTS.items():
        if mutate is not None:
            assert mutate(text) != text, name
