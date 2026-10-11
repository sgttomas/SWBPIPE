"""Reuse successful pure Rust checked-JSON responses during fixture assembly only."""
from collections import OrderedDict
from contextlib import contextmanager
import json
import os
from pathlib import Path
import subprocess

from core.serialization.canonical_json.adapter import PROFILE, PROTOCOL_VERSION, checked_executable


@contextmanager
def checked_json_fixture_cache():
    """The real authority handles every distinct exact request; no errors are reused.

    Direct adapter, invocation and security checks run outside this context.
    Responses contain immutable text, and every return gets a new process object.
    """
    executable = checked_executable()
    original = subprocess.run
    cache = OrderedDict()
    program_bytes = None
    retained_bytes = 0
    active = True
    max_bytes = 8 * 1024 * 1024
    max_entries = 512

    def run(command, *args, **options):
        nonlocal program_bytes, retained_bytes
        if not active:
            return original(command, *args, **options)
        allowed = {'input', 'text', 'encoding', 'capture_output', 'check', 'shell'}
        if (args or not isinstance(command, (list, tuple)) or len(command) != 1
                or Path(command[0]) != executable or set(options) != allowed
                or type(options['input']) is not str or options['text'] is not True
                or options['encoding'] != 'utf-8' or options['capture_output'] is not True
                or options['check'] is not False or options['shell'] is not False):
            return original(command, *args, **options)
        try:
            request = json.loads(options['input'])
        except ValueError:
            return original(command, *args, **options)
        if (type(request) is not dict or set(request) != {'protocol_version', 'profile', 'items'}
                or request['protocol_version'] != PROTOCOL_VERSION or request['profile'] != PROFILE
                or type(request['items']) is not list):
            return original(command, *args, **options)
        # Content is read on every call, including same-path, same-size, same-mtime
        # replacement. Equal byte strings share one stored identity in the keys.
        if not os.access(executable, os.X_OK):
            return original(command, *args, **options)
        current = executable.read_bytes()
        if current != program_bytes:
            cache.clear()
            retained_bytes = 0
            program_bytes = current
        key = (tuple(command), tuple(sorted(options.items())), program_bytes)
        if key in cache:
            stdout, stderr, _ = cache[key]
            cache.move_to_end(key)
            return subprocess.CompletedProcess(list(command), 0, stdout, stderr)
        completed = original(command, *args, **options)
        successful = False
        if (type(completed) is subprocess.CompletedProcess and completed.returncode == 0
                and type(completed.stdout) is str and type(completed.stderr) is str):
            try:
                response = json.loads(completed.stdout)
                successful = (type(response) is dict and response.get('protocol_version') == PROTOCOL_VERSION
                              and response.get('profile') == PROFILE and type(response.get('items')) is list
                              and [row.get('id') for row in response['items']] == [row.get('id') for row in request['items']]
                              and all(type(row.get('canonical_json')) is str for row in response['items']))
            except (ValueError, AttributeError, TypeError):
                pass
        if successful:
            size = len(options['input'].encode('utf-8')) + len(completed.stdout.encode('utf-8')) + len(completed.stderr.encode('utf-8'))
            if size <= max_bytes:
                while cache and (len(cache) >= max_entries or retained_bytes + size > max_bytes):
                    _, (_, _, removed) = cache.popitem(last=False)
                    retained_bytes -= removed
                cache[key] = (completed.stdout, completed.stderr, size)
                retained_bytes += size
        return completed

    subprocess.run = run
    try:
        yield
    finally:
        active = False
        cache.clear()
        subprocess.run = original
