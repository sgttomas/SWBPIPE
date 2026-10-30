"""Bounded foreground subprocess capture for the qualification development gate.

No shell, build, lookup on PATH, network operation, or implicit output directory.
Records bytes delivered to stdin and retained stdout/stderr, not an attestation
of the executable's internal use of those bytes or its numerical correctness.
"""
from __future__ import annotations
import hashlib
import os
from pathlib import Path
import selectors
import signal
import subprocess
import time


def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def file_sha256(path: Path, max_bytes: int | None = None) -> str:
    total = 0
    digest = hashlib.sha256()
    with path.open('rb') as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b''):
            total += len(chunk)
            if max_bytes is not None and total > max_bytes:
                raise ValueError('file exceeds bounded hash scope')
            digest.update(chunk)
    return digest.hexdigest()


def capture(executable: Path, arguments: list[str], input_bytes: bytes, directory: Path,
            timeout_seconds: float, output_limit_bytes: int, *, expected_executable_sha256: str | None = None) -> dict:
    """Capture one owned process with simultaneous bounded stdin/output I/O."""
    if not executable.is_absolute() or not executable.is_file():
        raise ValueError('explicit existing absolute executable required')
    if not 0 < timeout_seconds <= 3600 or not 0 < output_limit_bytes <= 64 * 1024 * 1024:
        raise ValueError('invalid supervision limits')
    directory.mkdir(parents=True, exist_ok=False)
    archive = directory / 'stdin.bin'
    archive.write_bytes(input_bytes)
    executable_before = file_sha256(executable)
    if expected_executable_sha256 is not None and executable_before != expected_executable_sha256:
        raise ValueError('executable digest changed before process launch')
    start = time.monotonic()
    outcome, process, return_code = 'error', None, None
    files = {name: (directory / (name + '.bin')).open('xb') for name in ('stdout', 'stderr')}
    counts = dict.fromkeys(files, 0)
    delivered_output = {name: hashlib.sha256() for name in files}
    sent = 0
    command = [str(executable), *arguments]
    detail = None
    try:
        # The archive is never the process's input source: a pipe receives the
        # exact immutable bytes, multiplexed with output to avoid pipe deadlock.
        process = subprocess.Popen(command, stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                   stderr=subprocess.PIPE, cwd=directory, start_new_session=True)
        selector = selectors.DefaultSelector()
        try:
            for name, stream in (('stdout', process.stdout), ('stderr', process.stderr)):
                os.set_blocking(stream.fileno(), False)
                selector.register(stream, selectors.EVENT_READ, name)
            if input_bytes:
                os.set_blocking(process.stdin.fileno(), False)
                selector.register(process.stdin, selectors.EVENT_WRITE, 'stdin')
            else:
                process.stdin.close()
            while selector.get_map():
                remaining = timeout_seconds - (time.monotonic() - start)
                if remaining <= 0:
                    outcome, detail = 'timeout', 'process time budget exceeded'
                    break
                for key, _ in selector.select(min(0.1, remaining)):
                    if key.data == 'stdin':
                        try:
                            written = os.write(key.fd, input_bytes[sent:sent + 65536])
                        except BrokenPipeError:
                            outcome, detail = 'input_closed', 'stdin closed before all selected bytes were delivered'
                            break
                        except BlockingIOError:
                            continue
                        sent += written
                        if sent == len(input_bytes):
                            selector.unregister(key.fileobj)
                            key.fileobj.close()
                        continue
                    try:
                        chunk = os.read(key.fd, 65536)
                    except BlockingIOError:
                        continue
                    if not chunk:
                        selector.unregister(key.fileobj)
                        key.fileobj.close()
                        continue
                    name = key.data
                    available = output_limit_bytes - counts[name]
                    files[name].write(chunk[:available])
                    delivered_output[name].update(chunk[:available])
                    counts[name] += min(len(chunk), available)
                    if len(chunk) > available:
                        outcome, detail = 'output_limit', f'{name} exceeds retained byte limit; remainder not retained'
                        break
                if detail:
                    break
            if detail is None:
                remaining = max(0.001, timeout_seconds - (time.monotonic() - start))
                try:
                    return_code = process.wait(timeout=remaining)
                    outcome = 'completed' if return_code == 0 else 'nonzero_exit'
                except subprocess.TimeoutExpired:
                    outcome, detail = 'timeout', 'process time budget exceeded after pipe EOF'
        finally:
            selector.close()
    except KeyboardInterrupt:
        outcome, detail = 'interrupted', 'supervisor interrupted; required ledger remains nonpassing'
    except OSError as exc:
        outcome, detail = 'spawn_error', f'{type(exc).__name__}: {exc}'
    finally:
        if process is not None:
            if process.poll() is None or detail is not None:
                try:
                    os.killpg(process.pid, signal.SIGKILL)
                except ProcessLookupError:
                    pass
            return_code = process.wait()
            for stream in (process.stdin, process.stdout, process.stderr):
                if stream is not None:
                    stream.close()
        for stream in files.values():
            stream.close()
    after = file_sha256(executable) if executable.is_file() else None
    if after != executable_before:
        outcome, detail = 'executable_changed', 'executable identity changed during process'
    def retained_hash(path: Path, limit: int) -> str | None:
        try:
            return file_sha256(path, max_bytes=limit)
        except (OSError, ValueError):
            return None
    retained_input = retained_hash(archive, len(input_bytes))
    selected_input = sha256_bytes(input_bytes)
    if retained_input != selected_input:
        outcome, detail = 'input_archive_changed', 'retained stdin artifact changed during process'
    retained_outputs = {name: retained_hash(directory / (name + '.bin'), output_limit_bytes) for name in files}
    if any(retained_outputs[name] != delivered_output[name].hexdigest() for name in files):
        outcome, detail = 'output_archive_changed', 'retained output differs from actual captured stream'
    return {'outcome': outcome, 'detail': detail, 'command': command, 'return_code': return_code,
            'elapsed_seconds': time.monotonic() - start, 'executable_sha256_before': executable_before,
            'executable_sha256_after': after, 'stdin_sha256': selected_input,
            'stdin_bytes_delivered': sent, 'stdin_delivered_sha256': sha256_bytes(input_bytes[:sent]),
            'stdin_delivery_complete': sent == len(input_bytes),
            'retained_stdin_sha256': retained_input,
            'stdout_sha256': delivered_output['stdout'].hexdigest(),
            'stderr_sha256': delivered_output['stderr'].hexdigest(),
            'retained_stdout_sha256': retained_outputs['stdout'], 'retained_stderr_sha256': retained_outputs['stderr'],
            'stdout_bytes': counts['stdout'], 'stderr_bytes': counts['stderr'],
            'timeout_seconds': timeout_seconds, 'output_limit_bytes_per_stream': output_limit_bytes,
            'raw_capture_complete': detail is None,
            'environment': {key: os.environ.get(key) for key in ('OMP_NUM_THREADS', 'OPENBLAS_NUM_THREADS', 'MKL_NUM_THREADS', 'RAYON_NUM_THREADS')},
            'scope': 'actual local process byte delivery/capture; no internal-use, build-source or solver attestation'}
