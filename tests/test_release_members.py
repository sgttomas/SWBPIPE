"""Retained member policy after retiring the sanitized-export mechanism."""
from pathlib import Path
import hashlib
import importlib.util
import json
import subprocess
import pytest
ROOT = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location('release_member_policy', ROOT / 'tools/release/release_members.py')
policy = importlib.util.module_from_spec(spec); spec.loader.exec_module(policy)


def test_exact_reviewed_binary_binding_and_unknown_binary_refusal():
    data = b'\x00\xffinvented binary'; name = 'invented/image.png'
    reviewed = {name: {'size_bytes': len(data), 'sha256': hashlib.sha256(data).hexdigest(), 'reason': 'invented test'}}
    assert policy.binary_admission(name, data, reviewed) is None
    assert policy.binary_admission(name, data + b'changed', reviewed) == 'reviewed_binary_hash_mismatch'
    assert policy.binary_admission('another.png', data, reviewed) == 'unreviewed_binary'


def test_binary_source_policy_covers_every_tracked_nontext_member():
    listing = ROOT / policy.REVIEWED_BINARIES_PATH
    reviewed = policy.load_reviewed_binaries(listing)
    assert len(reviewed) == len(json.loads(listing.read_text())['binaries'])
    for name in filter(None, subprocess.check_output(['git', '-C', str(ROOT), 'ls-files', '-z']).decode().split('\0')):
        path = ROOT / name
        if not path.is_file() or policy.is_text_member(path):
            continue
        assert policy.binary_admission(name, path.read_bytes(), reviewed) is None, name
    for name, entry in reviewed.items():
        data = (ROOT / name).read_bytes()
        assert len(data) == entry['size_bytes'] and hashlib.sha256(data).hexdigest() == entry['sha256']


def test_utf8_content_defines_lint_surface_instead_of_extension(tmp_path):
    text = tmp_path / 'invented.PNG'; text.write_text('invented UTF-8 text\n')
    binary = tmp_path / 'invented.json'; binary.write_bytes(b'\xff\xfe\x00')
    assert policy.is_text_member(text)
    assert not policy.is_text_member(binary)


def test_partition_preserves_order_and_refuses_an_oversized_windows_path(monkeypatch):
    paths = [Path(f'invented-{i}.txt') for i in range(1001)]
    monkeypatch.setattr(policy.platform, 'system', lambda: 'Linux')
    assert [p for batch in policy.lint_path_batches(paths, ['lint']) for p in batch] == paths
    monkeypatch.setattr(policy.platform, 'system', lambda: 'Windows')
    assert [p for batch in policy.lint_path_batches(paths, ['lint']) for p in batch] == paths
    with pytest.raises(OSError):
        policy.lint_path_batches([Path('a' * 31000)], ['lint'])
