"""A fake Windows guest for the CAEPIPE harness's attempt protocol (DEL-17-05). No real prlctl, no virtual machine.

It models what the harness's run phase relies on: files with exclusive create, the guest script's attempt (claim,
job, attempt folder, verified copy, launch, terminal report) as resumable steps, processes with creation times and
pid reuse, named job objects whose membership is inherited, and replies that are lost after their effect. Every
answer it gives is in the form a real guest would print; tests inject anything else (a transport line, a sign-in
text with exit 0, a sharing violation) through `answers`.

Everything here is invented: no capture, no session history, no program output.
"""

from __future__ import annotations

import base64
import hashlib
import json
import re
import subprocess
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any, Callable, Iterator

from core.handoff.caepipe_external import harness

GUEST_SCHEMA = harness.GUEST_REPORT_SCHEMA
SIGN_IN_TEXT = "Please sign in to continue. Your licence could not be verified."
TRANSPORT = harness.TRANSPORT_LINES  # the two transport lines, as the client prints them
STEPS = ("work", "claimed", "job", "folder", "copied", "launched", "program_ended", "finished")


def done(argv, code: int = 0, stdout: str = "", stderr: str = "") -> subprocess.CompletedProcess:
    return subprocess.CompletedProcess(argv, code, stdout, stderr)


# The read script's short form, as the harness builds it (`harness._read_command`): -File naming the installed copy,
# the operation, its parameters as base64 JSON, and optionally a data file's SHA-256.
READ_CALL = re.compile(r'^powershell -NoProfile -NonInteractive -ExecutionPolicy Bypass -File "(?P<script>[^"]+)" '
                       r"-Op (?P<op>[a-z]+) -Payload (?P<payload>[A-Za-z0-9+/=]+)(?: -Data (?P<data>[0-9a-f]{64}))?\Z")
READ_KINDS = {"cim", "job", "chunk", "regread", "storage", "hash"}  # each read op is its own kind
# The write script's ops, by the kind each is counted under: the fence, the registry API writes, the exercise key.
WRITE_KINDS = {"claim": "fence", "regset": "literal", "regdelete": "literal", "keycreate": "keywrite",
               "keydelete": "keywrite"}
SCRIPT_SENTINELS = {"Read-SwbpipeGuest.ps1": "SWBPIPE-READ", "Write-SwbpipeGuest.ps1": "SWBPIPE-WRITE"}


def script_call(command: str) -> dict | None:
    """A read- or write-script call: {script_name, op, payload (decoded), data, script (the guest path)}, else None."""
    match = READ_CALL.match(command)
    if match is None:
        return None
    name = match["script"].rsplit("\\", 1)[-1]
    if name not in SCRIPT_SENTINELS:
        return None
    return {"script_name": name, "op": match["op"], "data": match["data"], "script": match["script"],
            "payload": json.loads(base64.b64decode(match["payload"]).decode("utf-8"))}


def read_call(command: str) -> dict | None:
    """The read script's call: {op, payload (decoded), data, script (the guest path)}, else None."""
    call = script_call(command)
    return call if call is not None and call["script_name"] == "Read-SwbpipeGuest.ps1" else None


def write_call(command: str) -> dict | None:
    """The write script's call, like `read_call`, else None."""
    call = script_call(command)
    return call if call is not None and call["script_name"] == "Write-SwbpipeGuest.ps1" else None


def host_path(exchange_root: Path, guest_exchange_root: str, guest_path: str) -> Path | None:
    """The host file behind a guest path inside the exchange folder, else None."""
    if not guest_path.lower().startswith(guest_exchange_root.lower() + "\\"):
        return None
    return Path(exchange_root) / guest_path[len(guest_exchange_root) + 1:].replace("\\", "/")


def read_echo(exchange_root: Path, guest_exchange_root: str, command: str) -> tuple[str, dict | None]:
    """What the read or write script does before its operation: (its echo line, the data file's object or None).
    Like the real script it hashes its own file (absent: no echo at all, as when PowerShell cannot run it) and the
    data file it was named; ("", None) when there is no script to run."""
    call = script_call(command)
    script = host_path(exchange_root, guest_exchange_root, call["script"])
    if script is None or not script.is_file():
        return "", None
    data_sha, data = "none", None
    if call["data"]:
        path = Path(exchange_root) / harness.HARNESS_DIR / harness.DATA_DIR / f"{call['data']}.json"
        if path.is_file():
            raw = path.read_bytes()
            data_sha = hashlib.sha256(raw).hexdigest()
            data = json.loads(raw.decode("utf-8")) if data_sha == call["data"] else None
    sentinel = SCRIPT_SENTINELS[call["script_name"]]
    return f"{sentinel} v1 script={hashlib.sha256(script.read_bytes()).hexdigest()} data={data_sha}\r\n", data


def query_payload(command: str) -> tuple[str, dict] | None:
    """(kind, payload) of a scripted call: a read operation (its kind the op's name) or a write operation (the
    fence, "literal" for the registry API's set and delete, "keywrite" for the exercise key); else None."""
    call = script_call(command)
    if call is None:
        return None
    if call["script_name"] == "Read-SwbpipeGuest.ps1":
        return call["op"], call["payload"]
    return WRITE_KINDS.get(call["op"], "write_other"), call["payload"]


def classify(command: str) -> str:
    """The kind of a guest command the harness issues."""
    query = query_payload(command)
    if query:
        return query[0]
    if command.startswith("powershell ") and " -AttemptToken " in command:
        return "attempt"
    if command.startswith("powershell ") and " -ExportMod " in command:
        return "export"
    if command.startswith("type ") and command.endswith('.claim"'):
        return "claim_read"
    if command.startswith("type ") and command.endswith('.finished"'):
        return "finished_read"
    if command.startswith("type ") and command.endswith('owned_pids.txt"'):
        return "owned_pids"
    if command == "tasklist /FO CSV /NH":
        return "tasklist"
    if command.startswith("reg query "):
        return "reg_query"
    if command.startswith(("reg add", "reg delete")):
        return "reg_write"
    if command == "ver":
        return "ver"
    return "other"


# What the run phase may issue: the attempt, the host's fence (an exclusive create that never overwrites), and
# read-only queries. Nothing that prepares, stages or copies in the guest.
RUN_PHASE_KINDS = {"attempt", "fence", "claim_read", "finished_read", "cim", "job", "chunk", "owned_pids", "tasklist",
                   "reg_query", "regread", "storage", "hash"}
SCRIPTED_KINDS = READ_KINDS | {"fence", "literal", "keywrite"}


@dataclass
class Process:
    pid: int
    created: str
    job: str | None = None
    alive: bool = True


@dataclass
class Program:
    """What the program does when the attempt launches it: the report fields and the files it leaves."""

    status: str = "csv_from_batch"
    files: dict[str, bytes] = field(default_factory=lambda: {"model.csv": b"Invented,print\r\n", "model.res": b"RES",
                                                             "model.mod": b"INVENTED SOLVED MODEL\r\n"})
    print_name: str | None = "model.csv"
    lingers: bool = False  # the program keeps running after the script stops polling (a member of the job)
    pid: int | None = None


class Attempt:
    """One invocation of the guest script, as resumable steps (`STEPS`)."""

    def __init__(self, guest: "FakeGuestModel", command: str):
        self.guest, self.command = guest, command
        def get(flag: str) -> str:
            return (re.search(flag + r' "([^"]*)"', command) or re.search(flag + r" (\S+)", command))[1]
        self.exercise = " -Exercise " in command
        self.prefix = "SWBPIPE-harness-exercise-" if self.exercise else ""
        self.token, self.work = get("-AttemptToken"), get("-WorkRoot")
        self.mbf = None if self.exercise else get("-Mbf")
        self.expected = None if self.exercise else get("-ExpectedMbfSha256")
        self.pid_file = (re.search(r'-PidFile "([^"]*)"', command) or [None, None])[1]
        self.gui_fallback = " -NoGuiFallback" not in command
        self.launcher = guest.spawn()
        self.reached: list[str] = []
        self.report: dict[str, Any] | None = None
        self.stdout = ""
        self.exit = None
        self._steps = self._run()

    def path(self, suffix: str) -> str:
        return f"{self.work}\\{self.prefix}{self.token}{suffix}"

    def advance(self, until: str | None = None) -> None:
        """Run the script's next steps, through `until` (all of them when None)."""
        for reached in self._steps:
            self.reached.append(reached)
            if reached == until:
                return

    @property
    def ended(self) -> bool:
        return not self.launcher.alive

    def kill(self, program_too: bool = True) -> None:
        """The launcher is killed; its program too unless `program_too` is False (it then lives on in the job)."""
        self.launcher.alive = False
        self._steps = iter(())
        if program_too and self.launcher.job:
            for pid in self.guest.job_members(self.launcher.job):
                self.guest.processes[pid].alive = False
        self.guest.release(self.launcher)

    def _end(self, code: int, report: dict[str, Any] | None = None, stdout: str | None = None) -> None:
        self.exit = code
        if report is not None:
            self.stdout = json.dumps(report)
        if stdout is not None:
            self.stdout = stdout
        self.launcher.alive = False
        self.guest.release(self.launcher)

    def _run(self) -> Iterator[str]:
        guest = self.guest
        guest.dirs.add(self.work)
        yield "work"
        claim = (f"swbpipe-claim v1\r\ntoken={self.token}\r\nowner=script\r\npid={self.launcher.pid}\r\n"
                 f"created={self.launcher.created}\r\nnonce={'ab' * 16}\r\n")
        if not guest.create_new(self.path(".claim"), claim.encode("ascii")):
            self._end(20, {"schema": GUEST_SCHEMA, "attempt_token": self.token, "status": "claim_not_won"})
            return
        guest.claims_won.append(self.token)
        yield "claimed"
        job = f"swbpipe-{self.token}"
        guest.jobs.setdefault(job, 0)
        self.launcher.job = job
        guest.create_new(self.path(".job"), f"swbpipe-job v1\r\ntoken={self.token}\r\njob={job}\r\n".encode("ascii"))
        yield "job"
        folder = f"{self.work}\\{self.prefix}{self.token}"
        assert folder not in guest.dirs, "a fresh attempt folder already exists"
        guest.dirs.add(folder)
        yield "folder"
        if self.exercise:  # the fixed harmless process in place of the program, in the same job
            process = guest.spawn(job=job)
            guest.exercise_starts.append(self.token)
            yield "launched"
            if not guest.exercise_lingers:
                process.alive = False
            report = {"schema": GUEST_SCHEMA, "attempt_token": self.token, "mbf": None, "mbf_sha256": None,
                      "batch": {}, "gui": {"used": False}, "csv_path": None, "owned_pids": [process.pid],
                      "status": "exercise_ended", "diagnostics": [],
                      "claim": {"pid": self.launcher.pid, "created": self.launcher.created},
                      "exercise": {"exit_code": 0}}
            yield "program_ended"
            text = json.dumps(report, separators=(",", ":"), sort_keys=True)
            guest.create_new(self.path(".finished"),
                             f"swbpipe-finished v1\r\ntoken={self.token}\r\nreport={text}\r\n".encode("ascii"))
            self.report = report
            yield "finished"
            self._end(0, stdout=text)
            return
        data = guest.read_share(self.mbf)
        if guest.corrupt_copy:
            data = data + b"corrupted"
        guest.files[f"{folder}\\model.mbf"] = data
        report: dict[str, Any] = {"schema": GUEST_SCHEMA, "attempt_token": self.token, "mbf": f"{folder}\\model.mbf",
                                  "mbf_sha256": hashlib.sha256(data).hexdigest(), "batch": {},
                                  "gui": {"used": False}, "csv_path": None, "owned_pids": [], "status": "started",
                                  "diagnostics": [], "claim": {"pid": self.launcher.pid,
                                                               "created": self.launcher.created}}
        yield "copied"
        if report["mbf_sha256"] != self.expected:
            report["status"] = "input_mismatch"
        else:
            program = guest.program_for(self)
            process = guest.spawn(job=job, pid=program.pid)
            guest.launches.append(self.token)
            report["owned_pids"] = [process.pid]
            if self.pid_file:
                guest.files[self.pid_file] = f"{process.pid}\r\n".encode("ascii")
            yield "launched"
            for name, content in program.files.items():
                guest.files[f"{folder}\\{name}"] = content
            if not program.lingers:
                process.alive = False
            report["status"] = program.status
            if program.print_name:
                report["csv_path"] = f"{folder}\\{program.print_name}"
            report.update(guest.report_extra)
            yield "program_ended"
        text = json.dumps(report, separators=(",", ":"), sort_keys=True)
        guest.create_new(self.path(".finished"),
                         f"swbpipe-finished v1\r\ntoken={self.token}\r\nreport={text}\r\n".encode("ascii"))
        self.report = report
        yield "finished"
        self._end(0 if report.get("csv_path") else 3, stdout=text)


class FakeGuestModel:
    """The guest's state, and a `runner` that answers the harness's run-phase commands from it.

    `plan[n]` says what the n-th dispatch of the attempt does: `run` (the step it runs to before replying; None runs
    to the end), `reply` ("normal", "transport": the reply is lost after the effect, "timeout": the host gives up,
    "undelivered": the command never reaches the guest, with a transport reply) and `resume` ((kind, number) of the
    host read before whose answer the rest of the attempt runs, to `resume_until`), `kill_at` (the step after which
    the launcher is killed). `answers[(kind, number)]` replaces that read's answer with (exit, stdout, stderr), after
    the read ran; `lost` holds reads whose reply is lost after they ran (a transport line). `hooks[(kind, number)]` is
    called before that read is answered.
    """

    def __init__(self, exchange_root: Path, guest_exchange_root: str = "Z:\\invented"):
        self.exchange_root, self.guest_exchange_root = Path(exchange_root), guest_exchange_root
        self.files: dict[str, bytes] = {}
        self.dirs: set[str] = set()
        self.locked: set[str] = set()   # paths whose writer still holds them (a reader gets a sharing violation)
        self.processes: dict[int, Process] = {}
        self.jobs: dict[str, int] = {}  # job name -> lingering handles held by others
        self.attempts: list[Attempt] = []
        self.launches: list[str] = []
        self.claims_won: list[str] = []
        self.plan: dict[int, dict[str, Any]] = {}
        self.programs: dict[int, Program] = {}
        self.answers: dict[tuple[str, int], tuple[int, str, str]] = {}
        self.lost: set[tuple[str, int]] = set()
        self.hooks: dict[tuple[str, int], Callable[[], None]] = {}
        self.counts: dict[str, int] = {}
        self.commands: list[str] = []
        self.kinds: list[str] = []
        self.next_pid = 3000
        self.clock = 0
        self.reuse_launcher_pid = False  # another process takes a launcher's pid as soon as it ends
        self.corrupt_copy = False
        self.cim_unavailable = False
        self.report_extra: dict[str, Any] = {}
        self.resume_at: list[tuple[tuple[str, int], Attempt, str | None]] = []
        self.extra: Callable[..., subprocess.CompletedProcess | None] | None = None  # other commands (registry, ...)
        self.dispatch_raise: BaseException | None = None
        self.stderr_added: dict[str, str] = {}  # kind -> text appended to stderr of every answer of that kind
        self.program_hook: Callable[[Attempt], Program] | None = None  # decides the program at launch when set
        self.storage: dict[str, dict[str, Any]] = {}  # folder -> its storage entry (default: a plain NTFS C: folder)
        self.exercise_starts: list[str] = []  # tokens whose exercise process was started
        self.exercise_lingers = False  # the exercise's harmless process outlives the script's poll
        self.fence_denied = False  # the claim's create is refused (access denied): no effect, exit 1
        self.reparse: set[str] = set()  # paths that are reparse points (a junction, a symbolic link)
        # The job query's helper (OpenJobChecked): the canary its CloseHandle(NULL) reads (6 when the saved last error
        # is captured), and the last error OpenJobObject leaves for an absent job (2, ERROR_FILE_NOT_FOUND).
        self.job_canary = 6
        self.job_absent_error = 2

    # -- state ---------------------------------------------------------------------------------------------------
    def spawn(self, job: str | None = None, pid: int | None = None) -> Process:
        self.clock += 1
        if pid is None:
            self.next_pid += 4
            pid = self.next_pid
        process = Process(pid=pid, created=f"20261114{self.clock:06d}.000000", job=job)
        self.processes[pid] = process
        return process

    def release(self, launcher: Process) -> None:
        """A launcher has ended; with `reuse_launcher_pid` another process takes its pid at once (another creation
        time, in no job)."""
        if self.reuse_launcher_pid:
            self.clock += 1
            self.processes[launcher.pid] = Process(pid=launcher.pid, created=f"20261114{self.clock:06d}.999999")

    def create_new(self, path: str, data: bytes) -> bool:
        """CreateNew: False when the name exists; never overwrites."""
        if path in self.files:
            return False
        self.files[path] = data
        return True

    def read_share(self, path: str) -> bytes:
        relative = path[len(self.guest_exchange_root) + 1:].replace("\\", "/")
        return (self.exchange_root / relative).read_bytes()

    def job_members(self, name: str) -> list[int]:
        return [p.pid for p in self.processes.values() if p.job == name and p.alive]

    def program_for(self, attempt: Attempt) -> Program:
        if self.program_hook is not None:
            return self.program_hook(attempt)
        return self.programs.get(len(self.launches) + 1, Program())

    # -- the runner ----------------------------------------------------------------------------------------------
    def runner(self, argv, **kwargs) -> subprocess.CompletedProcess:
        assert argv[:6] == ["/invented/prlctl", "exec", "invented-vm", "--current-user", "cmd", "/c"], argv
        command = argv[6]
        kind = classify(command)
        self.commands.append(command)
        self.kinds.append(kind)
        self.counts[kind] = self.counts.get(kind, 0) + 1
        number = self.counts[kind]
        if (kind, number) in self.hooks:
            self.hooks[(kind, number)]()
        for entry in list(self.resume_at):
            if entry[0] == (kind, number):
                self.resume_at.remove(entry)
                entry[1].advance(entry[2])
        if kind == "attempt":
            return self._with_stderr(kind, self._dispatch(argv, command, number))
        if (kind, number) in self.answers and kind not in ("fence",):
            code, out, err = self.answers[(kind, number)]
            return done(argv, code, out, err)
        if kind in SCRIPTED_KINDS:
            result = self._read(argv, command, kind)
        else:
            result = self._answer(argv, command, kind, {})
        if (kind, number) in self.answers:  # a fence whose effect happened, answered otherwise
            code, out, err = self.answers[(kind, number)]
            return done(argv, code, out, err)
        if (kind, number) in self.lost:
            return done(argv, 255, "", TRANSPORT[0])
        return self._with_stderr(kind, result)

    def _with_stderr(self, kind: str, result: subprocess.CompletedProcess) -> subprocess.CompletedProcess:
        if kind in self.stderr_added:
            result.stderr = (result.stderr or "") + self.stderr_added[kind]
        return result

    def _dispatch(self, argv, command: str, number: int) -> subprocess.CompletedProcess:
        plan = self.plan.get(number, {})
        if plan.get("reply") == "undelivered":
            return done(argv, 255, "", TRANSPORT[1])
        attempt = Attempt(self, command)
        self.attempts.append(attempt)
        attempt.advance(plan.get("run") or plan.get("kill_at"))
        if plan.get("kill_at"):
            if plan["kill_at"] not in attempt.reached:
                attempt.advance(plan["kill_at"])
            attempt.kill(program_too=not plan.get("program_lives"))
        if plan.get("resume"):
            self.resume_at.append((plan["resume"], attempt, plan.get("resume_until")))
        if self.dispatch_raise is not None:  # what the host's call itself raised (a host timeout, ...)
            error, self.dispatch_raise = self.dispatch_raise, None
            raise error
        if plan.get("reply") == "transport":
            return done(argv, 255, "", TRANSPORT[number % 2])
        if plan.get("reply") == "timeout":
            raise subprocess.TimeoutExpired(argv, 600)
        if not attempt.ended:  # replied while the script is still running: not a real case; treat as lost
            return done(argv, 255, "", TRANSPORT[0])
        return done(argv, attempt.exit, attempt.stdout, "")

    def echo(self, command: str) -> str:
        """The echo line the read script prints first for `command` (for a test that injects an answer in form)."""
        return read_echo(self.exchange_root, self.guest_exchange_root, command)[0]

    def through_reparse(self, path: str) -> str | None:
        """The first component of `path`, from the drive's first folder on, that is a reparse point, else None."""
        parts = path.rstrip("\\").split("\\")
        for i in range(2, len(parts) + 1):
            if "\\".join(parts[:i]) in self.reparse:
                return "\\".join(parts[:i])
        return None

    def _read(self, argv, command: str, kind: str) -> subprocess.CompletedProcess:
        """A read- or write-script call: its echo first, then the operation's answer, as the real script prints
        them."""
        echo, data = read_echo(self.exchange_root, self.guest_exchange_root, command)
        if not echo:
            return done(argv, 1, "", "The term is not recognized as the name of a script file.")
        call = script_call(command)
        if call["data"] and data is None:
            return done(argv, 1, echo, "the data file differs from its digest")
        payload = dict(call["payload"]) | (data or {})
        result = self._answer(argv, command, kind, payload)
        return done(argv, result.returncode, echo + (result.stdout or ""), result.stderr or "")

    def _answer(self, argv, command: str, kind: str, payload: dict) -> subprocess.CompletedProcess:
        if kind in ("claim_read", "finished_read", "owned_pids"):
            path = re.fullmatch(r'type "([^"]+)"', command)[1]
            if path in self.locked:
                return done(argv, 1, "", "The process cannot access the file because it is being used by another "
                                         "process.")
            if path not in self.files:
                return done(argv, 1, "", "The system cannot find the file specified.")
            return done(argv, 0, self.files[path].decode("ascii"), "")
        if kind == "fence":  # the write script's claim: the folder, then the exclusive create
            if self.fence_denied:
                return done(argv, 1, "", "Access to the path is denied.")
            self.dirs.add(payload["folder"])
            path = payload["folder"] + "\\" + payload["name"]
            if path in self.locked:
                return done(argv, 0, "SWBPIPE-CLAIM v1 held\r\n", "")
            created = self.create_new(path, payload["content"].encode("ascii"))
            return done(argv, 0, f"SWBPIPE-CLAIM v1 {'created' if created else 'exists'}\r\n", "")
        if kind == "cim":
            if self.cim_unavailable:
                return done(argv, 1, "", "The RPC server is unavailable.")
            process = self.processes.get(payload["pid"])
            if process is None or not process.alive:
                return done(argv, 0, "SWBPIPE-CIM v1 none\r\n", "")
            return done(argv, 0, f"SWBPIPE-CIM v1 pid={process.pid} created={process.created}\r\n", "")
        if kind == "job":  # the read script's order: a handle is presence whatever the canary; then the canary
            name = payload["job"]
            members = self.job_members(name)
            if name not in self.jobs or (not members and not self.jobs[name]):
                if self.job_canary != 6:  # the canary failed: no failure code it read is trusted, never absent
                    return done(argv, 0, f"SWBPIPE-JOB v1 error=canary-{self.job_canary}\r\n", "")
                if self.job_absent_error != 2:
                    return done(argv, 0, f"SWBPIPE-JOB v1 error={self.job_absent_error}\r\n", "")
                return done(argv, 0, "SWBPIPE-JOB v1 absent\r\n", "")
            return done(argv, 0, f"SWBPIPE-JOB v1 present active={len(members)}\r\n", "")
        if kind in ("chunk", "hash") and self.through_reparse(payload["path"]):  # `Unreparsed`: refused, not opened
            return done(argv, 1, "", f"path through a reparse point refused: {self.through_reparse(payload['path'])}")
        if kind == "chunk":
            data = self.files.get(payload["path"])
            if data is None:
                return done(argv, 0, "SWBPIPE-CHUNK v1 absent\r\n", "")
            part = data[payload["offset"]:payload["offset"] + payload["length"]]
            return done(argv, 0, f"SWBPIPE-CHUNK v1 size={len(data)} sha256={hashlib.sha256(data).hexdigest()} "
                                 f"offset={payload['offset']} length={len(part)} "
                                 f"chunk_sha256={hashlib.sha256(part).hexdigest()}\r\n"
                                 f"{base64.b64encode(part).decode('ascii')}\r\n", "")
        if kind == "hash":
            data = self.files.get(payload["path"])
            if data is None:
                return done(argv, 0, "SWBPIPE-HASH v1 absent\r\n", "")
            return done(argv, 0, f"SWBPIPE-HASH v1 size={len(data)} sha256={hashlib.sha256(data).hexdigest()}\r\n", "")
        if kind == "storage":
            entries = []
            for folder in payload["folders"]:
                default = {"folder": folder, "absent": False, "final": "\\\\?\\" + folder, "volume": "C:\\",
                           "format": "NTFS", "type": "Fixed",
                           "components": [{"path": "\\".join(folder.split("\\")[:i]), "reparse": False,
                                           "directory": True} for i in range(2, len(folder.split("\\")) + 1)]}
                through = self.through_reparse(folder)
                if through is not None:  # the walk stops at the reparse component and opens nothing through it
                    components = [item for item in default["components"] if len(item["path"]) <= len(through)]
                    components[-1]["reparse"] = True
                    default = {"folder": folder, "absent": False, "final": None, "volume": None, "format": None,
                               "type": None, "components": components}
                entries.append(self.storage.get(folder, default if folder in self.dirs else
                                                {"folder": folder, "absent": True}))
            return done(argv, 0, "SWBPIPE-STORAGE v1 " + json.dumps(entries, separators=(",", ":")) + "\r\n", "")
        if self.extra is not None:
            result = self.extra(argv, command, kind)
            if result is not None:
                return result
        if kind == "tasklist":
            rows = ['"other.exe","9999","Console","2","61,744 K"']
            return done(argv, 0, "\r\n".join(rows) + "\r\n", "")
        if kind == "ver":
            return done(argv, 0, "\r\nMicrosoft Windows [Version 10.0.19045.5011]\r\n", "")
        raise AssertionError(f"unexpected guest command: {command[:120]}")

    # -- what a test asserts ---------------------------------------------------------------------------------------
    def writes_outside_reads(self) -> list[str]:
        return [kind for kind in self.kinds if kind not in RUN_PHASE_KINDS]
