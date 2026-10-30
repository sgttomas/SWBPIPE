"""Fixture response reuse preserves authority identity, failures and isolation."""
import json
import os
import subprocess

import pytest

import checked_json_fixture_cache as cache


def request(value=1):
    return json.dumps({'protocol_version': cache.PROTOCOL_VERSION, 'profile': cache.PROFILE,
                       'items': [{'id': 'value', 'json_text': json.dumps(value)}]})


def options(value=1):
    return dict(input=request(value), text=True, encoding='utf-8', capture_output=True, check=False, shell=False)


def setup_authority(monkeypatch, tmp_path, *, code=0, invalid=False):
    binary = tmp_path / 'openpipestress_jcs_ijson'
    binary.write_bytes(b'authority-one')
    binary.chmod(0o755)
    calls = []
    def run(command, **kwargs):
        calls.append((list(command), dict(kwargs)))
        text = 'invalid' if invalid else json.dumps({'protocol_version': cache.PROTOCOL_VERSION, 'profile': cache.PROFILE,
                  'items': [{'id': 'value', 'canonical_json': binary.read_text()}]})
        return subprocess.CompletedProcess(command, code, text, '')
    monkeypatch.setattr(cache, 'checked_executable', lambda: binary)
    monkeypatch.setattr(subprocess, 'run', run)
    return binary, calls, run


def test_successes_are_reused_with_fresh_objects_and_exact_requests(monkeypatch, tmp_path):
    binary, calls, _ = setup_authority(monkeypatch, tmp_path)
    with cache.checked_json_fixture_cache():
        first = subprocess.run([str(binary)], **options())
        original = first.stdout
        first.stdout = 'caller mutation'
        second = subprocess.run([str(binary)], **options())
        assert first is not second and first.args is not second.args
        assert second.stdout == original
        subprocess.run([str(binary)], **options(1.0))
    assert len(calls) == 2


def test_binary_content_replacement_with_identical_metadata_misses(monkeypatch, tmp_path):
    binary, calls, _ = setup_authority(monkeypatch, tmp_path)
    with cache.checked_json_fixture_cache():
        first = subprocess.run([str(binary)], **options())
        prior = binary.stat()
        replacement = tmp_path / 'replacement'
        replacement.write_bytes(b'authority-two')
        replacement.chmod(0o755)
        os.utime(replacement, ns=(prior.st_atime_ns, prior.st_mtime_ns))
        replacement.replace(binary)
        assert binary.stat().st_size == prior.st_size and binary.stat().st_mtime_ns == prior.st_mtime_ns
        second = subprocess.run([str(binary)], **options())
        assert first.stdout != second.stdout
        binary.write_bytes(b'authority-six')
        os.utime(binary, ns=(prior.st_atime_ns, prior.st_mtime_ns))
        third = subprocess.run([str(binary)], **options())
        assert third.stdout != second.stdout
    assert len(calls) == 3


@pytest.mark.parametrize('code,invalid', [(1, False), (0, True)])
def test_process_and_protocol_errors_are_never_reused(monkeypatch, tmp_path, code, invalid):
    binary, calls, _ = setup_authority(monkeypatch, tmp_path, code=code, invalid=invalid)
    with cache.checked_json_fixture_cache():
        for _ in range(2):
            subprocess.run([str(binary)], **options())
    assert len(calls) == 2


def test_context_boundaries_and_unmatched_options_do_not_reuse(monkeypatch, tmp_path):
    binary, calls, original = setup_authority(monkeypatch, tmp_path)
    with cache.checked_json_fixture_cache():
        subprocess.run([str(binary)], **options())
        subprocess.run([str(binary)], **dict(options(), check=True))
        subprocess.run([str(binary), '--unexpected'], **options())
        saved_context_callable = subprocess.run
    assert subprocess.run is original
    saved_context_callable([str(binary)], **options())
    subprocess.run([str(binary)], **options())
    with cache.checked_json_fixture_cache():
        subprocess.run([str(binary)], **options())
    assert len(calls) == 6


def test_exception_restores_original_callable(monkeypatch, tmp_path):
    _, _, original = setup_authority(monkeypatch, tmp_path)
    with pytest.raises(RuntimeError):
        with cache.checked_json_fixture_cache():
            raise RuntimeError('fixture failed')
    assert subprocess.run is original
