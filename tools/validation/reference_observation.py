"""Record selected events during a reference generator's run.

This is a limited additional check. An empty observation means no prohibited
event was detected within its coverage; it establishes neither independence
nor the absence of a product dependency. Independence must be examined on the
actual generator, its helpers, admitted reference modules and inputs. A change
to generator logic, imports or input resolution re-examines independence in
that change's reference review; a green observation does not discharge it.

**Coverage.** In an interpreter that installs the startup hook:

- Audit events named by `_PATHS` select one argument at the listed index. A
  text or bytes path is made absolute relative to the current working
  directory. Its lowercase spelling is compared with watched prefixes; it
  and its lexical parents are stat'ed and compared by device and file number
  with the watched roots. A match is recorded, including a missing path
  under a watched prefix. This covers direct paths and root aliases that
  this particular walk recognizes, not every resolution of a filesystem path.
- Audit events named by `_STARTS` are recorded, including `ctypes.dlopen`.
  Calls through the wrapped `_posixsubprocess.fork_exec` also report their
  returned child ID, arguments and whether the hook environment was kept.
  The judge treats a same-executable invocation matching a listed
  multiprocessing entry and its option/environment checks as an expected
  worker; another reported invocation is recorded as a program start.
- Fork events named by `_FORKS` are counted. Missing hook reports for expected
  workers, or fewer child-fork reports than recorded forks, are findings.
  These checks cover starts that reached those hooks, not unobserved starts.

**Reports.** An installed hook writes events to a named pipe as they occur;
the caller reads concurrently. Catching an access exception or a worker dying
after a report does not remove that report. The startup module reports its
interpreter, and the registered after-fork callback reports a forked child.
An interpreter that bypasses startup can escape these mechanisms.

**Known gaps.**

- An ordinary native-extension load outside watched roots can go unrecorded.
  Its native code can start a program or an unhooked interpreter without a
  record. Watching `ctypes.dlopen` does not cover all native-code loading.
- A directory-descriptor-relative open can read product code undetected:
  the audit path is resolved here against the working directory, not `dir_fd`.
- A symlink to a product file or subdirectory can escape the ancestry check:
  `stat` follows the link but the walk ascends lexical parents, not the target's
  resolved ancestors.

These are ordinary mechanisms, not evidence of intent to evade the check.
Unexecuted branches and scripts that no observed command runs are also outside
its evidence. POSIX only: this uses a named pipe and `_posixsubprocess`.
"""
import ast
from collections import namedtuple
import os
from pathlib import Path
import subprocess
import threading

Observation = namedtuple("Observation", "returncode stdout stderr records interpreters")

# The startup module, imported on Python startup when site initialization and the supplied
# module path are used. An interpreter bypassing that startup is not necessarily observed.
HOOK_SOURCE = '''\
import os
import sys

_channel = os.environ.get("REFERENCE_OBSERVATION_CHANNEL")
_KEPT = ("REFERENCE_OBSERVATION_CHANNEL", "REFERENCE_OBSERVATION_ROOTS", "REFERENCE_OBSERVATION_PREFIXES", "PYTHONPATH")
_watched = set()
for _root in os.environ.get("REFERENCE_OBSERVATION_ROOTS", "").split(os.pathsep):
    try:
        _status = os.stat(_root)
        _watched.add((_status.st_dev, _status.st_ino))
    except OSError:
        pass
_prefixes = [p.lower().rstrip(os.sep) for p in os.environ.get("REFERENCE_OBSERVATION_PREFIXES", "").split(os.pathsep) if p]
_STARTS = {
    "subprocess.Popen", "os.system", "os.exec", "os.posix_spawn", "os.spawn", "os.startfile", "pty.spawn", "ctypes.dlopen",
}
_FORKS = {"os.fork", "os.forkpty"}
# The position of the path among each event's arguments.
_PATHS = {
    "open": 0, "os.listdir": 0, "os.scandir": 0, "os.walk": 0, "os.fwalk": 0, "glob.glob": 0, "os.chdir": 0,
    "import": 1, "shutil.copyfile": 0, "shutil.copytree": 0, "os.rename": 0, "os.remove": 0, "os.mkdir": 0,
}
_busy = False


def _send(kind, event, detail):
    """One report, written whole at once: a line shorter than the pipe's atomic write size."""
    global _busy
    was, _busy = _busy, True
    try:
        line = "\\t".join((kind, str(os.getpid()), event, detail.replace("\\n", " ").replace("\\t", " ")))
        data = line.encode("utf-8", "backslashreplace")[:480] + b"\\n"
        descriptor = os.open(_channel, os.O_WRONLY)
        try:
            os.write(descriptor, data)
        finally:
            os.close(descriptor)
    finally:
        _busy = was


def _inside(path):
    if isinstance(path, bytes):
        path = os.fsdecode(path)
    if not isinstance(path, str) or not path:
        return False
    path = os.path.abspath(path)
    lowered = path.lower()
    if any(lowered == prefix or lowered.startswith(prefix + os.sep) for prefix in _prefixes):
        return True
    while True:
        try:
            status = os.stat(path)
            if (status.st_dev, status.st_ino) in _watched:
                return True
        except OSError:
            pass
        parent = os.path.dirname(path)
        if parent == path:
            return False
        path = parent


def _hook(event, args):
    if _busy:
        return
    if event in _STARTS:
        _send("start", event, repr(args[:2]))
    elif event in _FORKS:
        _send("fork", event, "")
    elif event in _PATHS and len(args) > _PATHS[event]:
        target = args[_PATHS[event]]
        if isinstance(target, (str, bytes)) and _inside(target):
            _send("path", event, repr(target))


def _environment_kept(environment):
    """Whether a started program receives the hook's environment unchanged."""
    if environment is None:
        return True
    given = {}
    for item in environment:
        name, _, value = bytes(item).partition(b"=")
        given[os.fsdecode(name)] = os.fsdecode(value)
    return all(given.get(name) == os.environ.get(name) for name in _KEPT)


def _watch_program_starts():
    import _posixsubprocess

    real = _posixsubprocess.fork_exec

    def fork_exec(*args, **kwargs):
        arguments = [os.fsdecode(item) for item in args[0]]
        kept = _environment_kept(args[5] if len(args) > 5 else kwargs.get("env"))
        child = real(*args, **kwargs)
        _send("exec", str(child), repr((kept, arguments)))
        return child

    _posixsubprocess.fork_exec = fork_exec


if _channel:
    _send("interpreter", "start", sys.executable)
    _watch_program_starts()
    os.register_at_fork(after_in_child=lambda: _send("interpreter", "fork", ""))
    sys.addaudithook(_hook)
'''

_WORKER_ENTRIES = (
    "from multiprocessing.spawn import spawn_main",
    "from multiprocessing.resource_tracker import main",
    "from multiprocessing.forkserver import main",
)
# Interpreter options that would stop the startup module from being imported.
_UNHOOKED_OPTIONS = {"-I", "-E", "-S"}


def _is_own_worker(interpreter, kept, arguments):
    if not kept or not arguments or "-c" not in arguments:
        return False
    try:
        same = os.path.samefile(arguments[0], interpreter)
    except OSError:
        same = False
    at = arguments.index("-c")
    options = arguments[1:at]
    if any(option in _UNHOOKED_OPTIONS or (option.startswith("-") and not option.startswith("--") and set(option[1:]) & set("IES")) for option in options):
        return False
    return same and at + 1 < len(arguments) and arguments[at + 1].startswith(_WORKER_ENTRIES)


def _judge(lines, interpreter):
    """The records of a run and the number of interpreters that reported their hook."""
    records = []
    reported = set()
    forked_reports = 0
    forks = 0
    workers = []
    for line in lines:
        parts = line.split("\t", 3)
        if len(parts) != 4:
            records.append(("unreadable", "report", line))
            continue
        kind, pid, event, detail = parts
        if kind == "interpreter":
            reported.add(pid)
            forked_reports += event == "fork"
        elif kind == "fork":
            forks += 1
        elif kind == "exec":
            try:
                kept, arguments = ast.literal_eval(detail)
            except (ValueError, SyntaxError):
                kept, arguments = False, []
            if _is_own_worker(interpreter, kept, arguments):
                workers.append(event)
            else:
                records.append(("start", "exec", detail))
        else:
            records.append((kind, event, detail))
    for child in workers:
        if child not in reported:
            records.append(("unobserved", "worker", child))
    if forked_reports < forks:
        records.append(("unobserved", "fork", f"{forks} forks, {forked_reports} children reported"))
    return records, len(reported)


def run_observed(command, cwd, workdir, roots, timeout, prefixes=()):
    """Run `command` under the hook and return an `Observation`: the exit status and output, the
    records as (kind, event, detail), and the number of interpreters whose hook reported.

    `command` is the full argument list, its first item the Python interpreter, which is run with
    `-s -B`: no user site directory and no bytecode written. The environment holds the search path
    for programs, the module path of the startup module and the hook's variables, and nothing else.
    `roots` are the watched directories and `prefixes` further watched locations that need not
    exist. `workdir` is an empty directory the caller owns.
    """
    workdir = Path(workdir)
    (workdir / "sitecustomize.py").write_text(HOOK_SOURCE, encoding="utf-8")
    channel = workdir / "reports"
    os.mkfifo(channel)
    environment = {
        "PATH": os.environ.get("PATH", ""),
        "PYTHONPATH": str(workdir),
        "REFERENCE_OBSERVATION_CHANNEL": str(channel),
        "REFERENCE_OBSERVATION_ROOTS": os.pathsep.join(str(root) for root in roots),
        "REFERENCE_OBSERVATION_PREFIXES": os.pathsep.join(str(prefix) for prefix in prefixes),
    }
    # Held open for reading and writing, so that a writer never waits and the pipe never reports its end.
    descriptor = os.open(channel, os.O_RDWR | os.O_NONBLOCK)
    received = bytearray()
    finished = threading.Event()

    def read():
        while True:
            try:
                received.extend(os.read(descriptor, 65536))
            except BlockingIOError:
                if finished.is_set():
                    return
                finished.wait(0.01)

    reader = threading.Thread(target=read, daemon=True)
    reader.start()
    try:
        done = subprocess.run(
            [command[0], "-s", "-B", *command[1:]], cwd=cwd, env=environment, timeout=timeout, capture_output=True, text=True
        )
    finally:
        finished.set()
        reader.join()
        os.close(descriptor)
    lines = received.decode("utf-8", "backslashreplace").splitlines()
    records, interpreters = _judge(lines, command[0])
    return Observation(done.returncode, done.stdout, done.stderr, records, interpreters)
