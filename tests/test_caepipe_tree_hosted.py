"""The read script's process-tree observer (`tree`, DEL-17-05), proved on hosted Windows runners.

Run by `.github/workflows/caepipe-tree-observer-windows.yml` on `windows-11-arm` (the deployed guest's architecture) and
`windows-2025` (x64); skipped elsewhere. It runs the exact maintained read script, as committed (LF line ends), at its
content-addressed path and checked by its echo against `harness.READ_SCRIPT_SHA256`, under Windows PowerShell 5.1,
through `cmd /c`, with the command the harness builds (`harness._tree_command`).

The tree it observes is the test's own: copies of the system's `ping.exe` under test names in a temporary folder,
started by this file's helper (CreateProcess with PROC_THREAD_ATTRIBUTE_PARENT_PROCESS), not by the script under test:
a launcher L, its child C, and C's child G. Then C ends, so G is orphaned and its ParentProcessId chain is broken.

- With the whole tree alive, descent alone reads L, C and G; with a recorded creation time later than every member's,
  the creation-time guard admits none of L's children.
- Once G is orphaned: with G's path listed the op reads L and G; with G's path NOT listed G is not found. This is the
  observer's stated weakness, asserted so that it is shown, not hidden.
- Once the test has ended every member, the op reads `none`.

Controls, by mutant: an op that always answers `none`, and an op without the creation-time guard, are each run
against the live tree and must fail the same checks; each failure is printed. The output names only the runner's
versions and the test's outcomes: no path, and nothing of any guest.
"""

from __future__ import annotations

import base64
from datetime import datetime, timedelta, timezone
import hashlib
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import time

import pytest

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.handoff.caepipe_external import harness  # noqa: E402

# The controls: (anchor in the read script, its replacement). Each anchor occurs exactly once (`mutant_source`).
MUTANTS = {
    "always-none": ("function OpTree($p) {\n", "function OpTree($p) {\n  return \"SWBPIPE-TREE v1 none`r`n\"\n"),
    "no-creation-time-guard": ("      if ([string]::CompareOrdinal($r.created, $node.created) -lt 0) { continue }"
                               "  # created before its parent: not its child\n", ""),
}
MEMBERS = ("tree-launcher.exe", "tree-child.exe", "tree-grandchild.exe")
EXPECTED_ARCH = "SWBPIPE_TREE_EXPECTED_ARCH"  # Arm64 or X64, set by the workflow's job
CALL_TIMEOUT_S = 180


def mutant_source(name: str) -> bytes:
    """The read script with one control's mutation applied; its anchor must occur exactly once."""
    anchor, replacement = MUTANTS[name]
    source = harness.READ_SCRIPT.read_bytes()
    assert source.count(anchor.encode("ascii")) == 1, f"the {name} mutation's anchor is not in the read script once"
    return source.replace(anchor.encode("ascii"), replacement.encode("ascii"))


# ---------------------------------------------------------------------------------------------------------------
# The helper that starts the test's own tree: CreateProcessW, with a chosen parent for C and G. Windows only.
# ---------------------------------------------------------------------------------------------------------------
if sys.platform == "win32":
    import ctypes
    from ctypes import wintypes

    KERNEL32 = ctypes.WinDLL("kernel32", use_last_error=True)
    DETACHED_PROCESS = 0x00000008  # no console, so no console host joins the tree
    EXTENDED_STARTUPINFO_PRESENT = 0x00080000
    PROC_THREAD_ATTRIBUTE_PARENT_PROCESS = 0x00020000

    class STARTUPINFOW(ctypes.Structure):
        _fields_ = [("cb", wintypes.DWORD), ("lpReserved", wintypes.LPWSTR), ("lpDesktop", wintypes.LPWSTR),
                    ("lpTitle", wintypes.LPWSTR), ("dwX", wintypes.DWORD), ("dwY", wintypes.DWORD),
                    ("dwXSize", wintypes.DWORD), ("dwYSize", wintypes.DWORD), ("dwXCountChars", wintypes.DWORD),
                    ("dwYCountChars", wintypes.DWORD), ("dwFillAttribute", wintypes.DWORD),
                    ("dwFlags", wintypes.DWORD), ("wShowWindow", wintypes.WORD), ("cbReserved2", wintypes.WORD),
                    ("lpReserved2", ctypes.c_void_p), ("hStdInput", wintypes.HANDLE),
                    ("hStdOutput", wintypes.HANDLE), ("hStdError", wintypes.HANDLE)]

    class STARTUPINFOEXW(ctypes.Structure):
        _fields_ = [("StartupInfo", STARTUPINFOW), ("lpAttributeList", ctypes.c_void_p)]

    class PROCESS_INFORMATION(ctypes.Structure):
        _fields_ = [("hProcess", wintypes.HANDLE), ("hThread", wintypes.HANDLE), ("dwProcessId", wintypes.DWORD),
                    ("dwThreadId", wintypes.DWORD)]

    KERNEL32.InitializeProcThreadAttributeList.argtypes = [ctypes.c_void_p, wintypes.DWORD, wintypes.DWORD,
                                                           ctypes.POINTER(ctypes.c_size_t)]
    KERNEL32.InitializeProcThreadAttributeList.restype = wintypes.BOOL
    KERNEL32.UpdateProcThreadAttribute.argtypes = [ctypes.c_void_p, wintypes.DWORD, ctypes.c_size_t, ctypes.c_void_p,
                                                   ctypes.c_size_t, ctypes.c_void_p, ctypes.c_void_p]
    KERNEL32.UpdateProcThreadAttribute.restype = wintypes.BOOL
    KERNEL32.DeleteProcThreadAttributeList.argtypes = [ctypes.c_void_p]
    KERNEL32.DeleteProcThreadAttributeList.restype = None
    KERNEL32.CreateProcessW.argtypes = [wintypes.LPCWSTR, wintypes.LPWSTR, ctypes.c_void_p, ctypes.c_void_p,
                                        wintypes.BOOL, wintypes.DWORD, ctypes.c_void_p, wintypes.LPCWSTR,
                                        ctypes.c_void_p, ctypes.POINTER(PROCESS_INFORMATION)]
    KERNEL32.CreateProcessW.restype = wintypes.BOOL
    KERNEL32.TerminateProcess.argtypes = [wintypes.HANDLE, wintypes.UINT]
    KERNEL32.TerminateProcess.restype = wintypes.BOOL
    KERNEL32.WaitForSingleObject.argtypes = [wintypes.HANDLE, wintypes.DWORD]
    KERNEL32.WaitForSingleObject.restype = wintypes.DWORD
    KERNEL32.CloseHandle.argtypes = [wintypes.HANDLE]
    KERNEL32.CloseHandle.restype = wintypes.BOOL


class Member:
    """One process of the test's tree: its pid and its process handle (held until it is ended)."""

    def __init__(self, exe: str, parent: "Member | None" = None):
        self.exe = exe
        info = PROCESS_INFORMATION()
        startup = STARTUPINFOEXW()
        flags = DETACHED_PROCESS
        attributes = None
        parent_handle = None
        if parent is None:
            startup.StartupInfo.cb = ctypes.sizeof(STARTUPINFOW)
        else:
            startup.StartupInfo.cb = ctypes.sizeof(STARTUPINFOEXW)
            size = ctypes.c_size_t(0)
            KERNEL32.InitializeProcThreadAttributeList(None, 1, 0, ctypes.byref(size))  # asks the size
            attributes = (ctypes.c_byte * size.value)()
            if not KERNEL32.InitializeProcThreadAttributeList(attributes, 1, 0, ctypes.byref(size)):
                raise ctypes.WinError(ctypes.get_last_error())
            parent_handle = wintypes.HANDLE(parent.handle)
            if not KERNEL32.UpdateProcThreadAttribute(attributes, 0, PROC_THREAD_ATTRIBUTE_PARENT_PROCESS,
                                                      ctypes.byref(parent_handle), ctypes.sizeof(parent_handle),
                                                      None, None):
                raise ctypes.WinError(ctypes.get_last_error())
            startup.lpAttributeList = ctypes.cast(attributes, ctypes.c_void_p)
            flags |= EXTENDED_STARTUPINFO_PRESENT
        command = ctypes.create_unicode_buffer(f'"{exe}" -n 3600 -w 1000 127.0.0.1')
        try:
            if not KERNEL32.CreateProcessW(exe, command, None, None, False, flags, None, None, ctypes.byref(startup),
                                           ctypes.byref(info)):
                raise ctypes.WinError(ctypes.get_last_error())
        finally:
            if attributes is not None:
                KERNEL32.DeleteProcThreadAttributeList(attributes)
        KERNEL32.CloseHandle(info.hThread)
        self.pid, self.handle = int(info.dwProcessId), info.hProcess

    def end(self) -> None:
        """Terminate it, wait for it, and close the handle, so that no snapshot lists it any more."""
        if self.handle:
            KERNEL32.TerminateProcess(self.handle, 1)
            KERNEL32.WaitForSingleObject(self.handle, 30000)
            KERNEL32.CloseHandle(self.handle)
            self.handle = None


# ---------------------------------------------------------------------------------------------------------------
# Running the read script as the harness builds its command, under `cmd /c`.
# ---------------------------------------------------------------------------------------------------------------
def _cmd(command: str) -> subprocess.CompletedProcess:
    cmd = os.path.join(os.environ["SystemRoot"], "System32", "cmd.exe")
    return subprocess.run("cmd /c " + command, executable=cmd, capture_output=True, timeout=CALL_TIMEOUT_S)


def _record(command: str, digest: str) -> dict:
    """One read-script call: the harness's record of it, its echo checked against `digest` (the script under test)."""
    result = _cmd(command)
    stdout, stderr = result.stdout.decode("utf-8"), result.stderr.decode("utf-8", "replace")
    echo = f"SWBPIPE-READ v1 script={digest} data=none\r\n"
    assert stdout.startswith(echo), "the reply's echo does not name the script under test"
    record = {"guest_command": command, "stdout": stdout, "stderr": stderr, "exit_status": result.returncode}
    if digest == harness.READ_SCRIPT_SHA256:
        harness._settle_echo(record)  # the host's own echo check
        assert record["echo"]["verified"], "the host does not verify the maintained script's echo"
    else:  # a control: its digest is not the maintained one, so its echo is checked here
        record["echo"], record["reply_body"] = {"verified": True}, stdout[len(echo):]
    return record


class Observer:
    """The read script under test (the maintained one, or a control) at its content-addressed path."""

    def __init__(self, config: harness.HarnessConfig, source: bytes | None = None):
        self.config = config
        if source is None:  # the maintained script, as committed: LF line ends (`.gitattributes`: core/** -text)
            source = harness.READ_SCRIPT.read_bytes()
            assert b"\r" not in source and hashlib.sha256(source).hexdigest() == harness.READ_SCRIPT_SHA256, \
                "the checkout's read script is not the committed bytes"
        self.digest = hashlib.sha256(source).hexdigest()
        folder = config.exchange_root / harness.HARNESS_DIR / self.digest
        folder.mkdir(parents=True, exist_ok=True)
        (folder / harness.READ_SCRIPT.name).write_bytes(source)  # binary: the bytes exactly
        installed = (config.exchange_root / harness.HARNESS_DIR / self.digest / harness.READ_SCRIPT.name).read_bytes()
        assert hashlib.sha256(installed).hexdigest() == self.digest

    def _command(self, command: str) -> str:
        assert harness.READ_SCRIPT_SHA256 in command and " -Data " not in command
        return command.replace(harness.READ_SCRIPT_SHA256, self.digest)

    def tree(self, pid: int, created: str, paths: list[str]) -> dict:
        command = self._command(harness._tree_command(self.config, pid, created, paths))
        return harness.read_tree(_record(command, self.digest))

    def created(self, pid: int) -> str | None:
        """The `cim` op's creation time for `pid` (the form the claim records), None when no such process lives."""
        record = _record(self._command(harness._read_command(self.config, "cim", {"pid": pid})), self.digest)
        match = harness.CIM_ANSWER.match(harness._body(record)) if harness._answered(record) else None
        assert match is not None, "the cim op did not answer in its form"
        return match["created"]


def _pids(reading: dict) -> dict[int, list[str]]:
    assert reading["state"] in ("absent", "present"), f"the tree op's answer is {reading['state']}"
    return {item["pid"]: item["match"] for item in reading.get("processes", [])}


def whole_tree_checks(observer: Observer, members: dict[str, Member], created: dict[str, str]) -> None:
    """With L, C and G alive: descent alone reads all three; a recorded creation time later than every member's
    admits none of L's children (the guard against pid reuse)."""
    launcher, child, grandchild = members["L"], members["C"], members["G"]
    seen = _pids(observer.tree(launcher.pid, created["L"], []))
    assert seen == {launcher.pid: ["launcher"], child.pid: ["descendant"], grandchild.pid: ["descendant"]}, \
        f"descent read {seen}"
    later = (datetime.now(timezone.utc) + timedelta(days=1)).strftime("%Y%m%d%H%M%S.%f")
    seen = _pids(observer.tree(launcher.pid, later, []))
    assert seen == {}, f"with a recorded creation time later than every child's, the op read {seen}"


def orphan_checks(observer: Observer, members: dict[str, Member], created: dict[str, str]) -> None:
    """With C ended and G orphaned: G is found only by its listed path."""
    launcher, grandchild = members["L"], members["G"]
    paths = {name: members[name].exe for name in members}
    seen = _pids(observer.tree(launcher.pid, created["L"], [paths["L"], paths["C"], paths["G"]]))
    assert seen == {launcher.pid: ["launcher", "path"], grandchild.pid: ["path"]}, f"with G's path listed: {seen}"
    seen = _pids(observer.tree(launcher.pid, created["L"], [paths["L"], paths["C"]]))
    assert seen == {launcher.pid: ["launcher", "path"]}, f"with G's path not listed: {seen}"
    assert grandchild.pid not in seen  # the weakness, shown: an orphan of an unlisted executable is not found


def _runner_identity() -> str:
    """The runner's Windows build, OS and process architecture, PowerShell version and .NET Framework release, as the
    `powershell` that `cmd /c` starts reports them."""
    script = (harness.POWERSHELL_QUIET + "$v = [Environment]::OSVersion.Version\n"
              "$nt = Get-ItemProperty 'HKLM:\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion'\n"
              "$nf = Get-ItemProperty 'HKLM:\\SOFTWARE\\Microsoft\\NET Framework Setup\\NDP\\v4\\Full'\n"
              "$os = try { [Runtime.InteropServices.RuntimeInformation]::OSArchitecture } catch { 'unavailable' }\n"
              "$pa = try { [Runtime.InteropServices.RuntimeInformation]::ProcessArchitecture } catch { 'unavailable' }\n"
              "[Console]::Out.Write('windows_build=' + $v.Major + '.' + $v.Minor + '.' + $nt.CurrentBuild + '.' + $nt.UBR"
              " + ' os_architecture=' + $os + ' process_architecture=' + $pa"
              " + ' processor_architecture=' + $env:PROCESSOR_ARCHITECTURE"
              " + ' powershell=' + $PSVersionTable.PSVersion + ' edition=' + $PSVersionTable.PSEdition"
              " + ' dotnet_framework_release=' + $nf.Release)\n")
    encoded = base64.b64encode(script.encode("utf-16-le")).decode("ascii")
    result = _cmd(f"{harness.POWERSHELL_COMMAND} -EncodedCommand {encoded}")
    assert result.returncode == 0 and not result.stderr.strip(), "the runner's identity could not be read"
    return result.stdout.decode("ascii").strip()


@pytest.fixture
def exchange():
    """A temporary exchange folder on the runner, by its full long path (the form ExecutablePath reports)."""
    base = os.environ.get("RUNNER_TEMP") or tempfile.gettempdir()
    root = Path(os.path.realpath(tempfile.mkdtemp(prefix="swbpipe-tree-", dir=base)))
    try:
        yield root
    finally:
        shutil.rmtree(root, ignore_errors=True)


@pytest.mark.skipif(sys.platform != "win32", reason="the tree op's proof runs on hosted Windows runners")
def test_the_tree_op_observes_the_tests_own_tree_and_each_control_fails(exchange):
    identity = _runner_identity()
    print(f"\nrunner: {identity}")
    fields = dict(item.split("=", 1) for item in identity.split(" "))
    assert fields["powershell"].startswith("5.1.") and fields["edition"] == "Desktop", "not Windows PowerShell 5.1"
    expected = os.environ.get(EXPECTED_ARCH)
    if expected:
        assert fields["os_architecture"] == fields["process_architecture"] == expected, "not the expected architecture"
    config = harness.HarnessConfig(exchange_root=exchange, vm_name="hosted-runner", guest_executable="C:\\x.exe",
                                   guest_exchange_root=str(exchange), guest_work_root=str(exchange), prlctl=None,
                                   acknowledged=True, trial_first_launch_date=None)
    # The test's own harmless executables: copies of ping.exe (and its English message resources) under test names.
    system32 = Path(os.environ["SystemRoot"]) / "System32"
    bin_dir = exchange / "bin"
    (bin_dir / "en-US").mkdir(parents=True)
    for name in MEMBERS:
        shutil.copyfile(system32 / "PING.EXE", bin_dir / name)
        if (system32 / "en-US" / "PING.EXE.mui").is_file():
            shutil.copyfile(system32 / "en-US" / "PING.EXE.mui", bin_dir / "en-US" / f"{name}.mui")
        assert harness.TREE_PATH_PATTERN.match(str(bin_dir / name)), "the runner's folder is outside the allow-list"
    observer = Observer(config)
    controls = {name: Observer(config, mutant_source(name)) for name in MUTANTS}
    members: dict[str, Member] = {}
    try:
        members["L"] = Member(str(bin_dir / MEMBERS[0]))
        members["C"] = Member(str(bin_dir / MEMBERS[1]), parent=members["L"])
        members["G"] = Member(str(bin_dir / MEMBERS[2]), parent=members["C"])
        created = {name: observer.created(member.pid) for name, member in members.items()}
        assert all(created.values()), "a member of the tree is not alive"

        whole_tree_checks(observer, members, created)
        print("tree op, whole tree: descent reads L, C and G; a later recorded creation time admits no child: passed")
        for name, control in controls.items():
            with pytest.raises(AssertionError) as failed:
                whole_tree_checks(control, members, created)
            print(f"control {name}: failed the whole-tree checks, as required: {str(failed.value).splitlines()[0]}")

        members["C"].end()
        for _ in range(30):  # until no snapshot lists C
            if observer.created(members["C"].pid) != created["C"]:
                break
            time.sleep(1)
        else:
            pytest.fail("the ended child is still listed")
        orphan_checks(observer, members, created)
        print("tree op, orphaned grandchild: found with its path listed, NOT found without it (the stated weakness): "
              "passed")
    finally:
        for member in members.values():
            member.end()
    paths = [member.exe for member in members.values()]
    for _ in range(30):
        reading = observer.tree(members["L"].pid, created["L"], paths)
        if reading["state"] == "absent":
            break
        time.sleep(1)
    assert reading == {"state": "absent"}, f"after every member ended the op read {reading}"
    print("tree op, after the test ended every member: none: passed")
