"""Observe what a reference generator's run reaches.

A frozen reference must regenerate from a generator that reaches no product
code. `run_observed` runs a generator with a hook in every Python interpreter
the run starts and returns what the hooks reported. It shows what a run
reached. It does not show that a reference was derived independently: that is
each package's own refutation record.

**What is a record.**

- A file opened, a directory listed or entered, or a module loaded, whose
  location is inside a watched root. Locations are compared by device and file
  number, so a case variant of a watched root, or a link to it, is that root.
  A location under a watched prefix is a record too, whether or not it exists.
- A program started, with one exception. A generator may start an interpreter
  of the same executable that runs `multiprocessing`'s own worker entry with
  the hook's environment intact: a worker of itself. The hook sees every
  program start at the one function through which Python starts a program
  (`_posixsubprocess.fork_exec`) and tells a worker from any other start by its
  executable, its arguments and its environment. Every other start is a record.
- A started worker that never reported its hook, and a fork whose child never
  reported. Silence is not counted as no access.

**How a report is made.** Each hook writes its report to a named pipe at the
moment of the event, and the caller reads the pipe while the run goes on. A
report made stays made: the generator can catch nothing, a worker that dies
afterwards changes nothing, and nothing the run does later can take a report
back. Each interpreter reports that its hook is installed before any other
code of the run executes, because Python imports the startup module first. A
forked child is the same program going on under the same hook and reports
itself as it starts.

**What this does not observe.**

- Anything the observed command does not execute.
- A script that no observed command runs.
- A program started without Python's own functions for it, by machine code the
  run loads. Loading such code is itself a record.

POSIX only: it relies on a named pipe and on `_posixsubprocess`.
"""
import ast
from collections import namedtuple
import os
from pathlib import Path
import subprocess
import threading

Observation = namedtuple("Observation", "returncode stdout stderr records interpreters")

# The startup module. Python imports `sitecustomize` when it starts, from the module path, before
# the script and before any module the script imports.
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
