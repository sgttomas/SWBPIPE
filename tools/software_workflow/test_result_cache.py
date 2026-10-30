"""Successful verification reuse follows inputs, command selection and environment."""
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys

import yaml

spec = importlib.util.spec_from_file_location('result_cache', Path(__file__).with_name('result_cache.py'))
cache = importlib.util.module_from_spec(spec)
spec.loader.exec_module(cache)
ROOT = Path(__file__).resolve().parents[2]


def test_input_key_changes_for_code_tests_fixtures_commands_and_image(tmp_path):
    def git(*args):
        subprocess.run(['git', '-C', str(tmp_path), *args], check=True, capture_output=True)
    git('init', '-q'); git('config', 'user.email', 'test@example.invalid'); git('config', 'user.name', 'Fixture')
    def commit(path, content):
        p = tmp_path / path; p.parent.mkdir(parents=True, exist_ok=True); p.write_text(content)
        git('add', '.'); git('commit', '-qm', 'fixture')
    def key(commands=None, environment=None):
        return cache.fingerprint(tmp_path, 'numerical', commands or ['cargo test'], environment or {'image': 'v1'})
    commit('core/lib.rs', 'one')
    first = key()
    commit('execution/PKG-01/DEL-01-01/Design/note.md', 'prose')
    assert key() == first
    for path in ['core/lib.rs', 'tests/test_one.py',
                 'fixtures/handoff/one.json', '.github/workflows/piping-desktop-e2e.yml']:
        before = key(); commit(path, 'new'); assert key() != before
    assert key(['cargo test --lib']) != key()
    assert key(environment={'image': 'v2'}) != key()
    git('rm', 'core/lib.rs'); git('commit', '-qm', 'delete')
    assert key() != first


def test_reuse_rejects_missing_wrong_or_failed_record(tmp_path):
    record = tmp_path / 'result.json'
    env = {**os.environ, 'RESULT_KEY': 'expected', 'GITHUB_STEP_SUMMARY': str(tmp_path / 'summary')}
    command = [sys.executable, str(Path(cache.__file__)), 'reuse', '--suite', 'numerical', '--record', str(record)]
    for data in [None, {'key': 'wrong', 'conclusion': 'success'}, {'key': 'expected', 'conclusion': 'failure'}]:
        if data is not None: record.write_text(json.dumps(data))
        assert subprocess.run(command, env=env, capture_output=True).returncode != 0
    record.write_text(json.dumps({'key': 'expected', 'conclusion': 'success', 'head': 'original', 'run': 'https://github.com/example/repo/actions/runs/1'}))
    result = subprocess.run(command, env=env, capture_output=True, text=True)
    assert result.returncode == 0
    assert 'No tests executed' in result.stdout


def test_workflow_saves_only_success_and_never_restores_partial_keys():
    workflow = yaml.load((ROOT / '.github/workflows/piping-desktop-e2e.yml').read_text(), Loader=yaml.BaseLoader)
    jobs = workflow['jobs']
    # Browser verification is recorded by its own job after the product job and
    # the desktop shards it vouches for (desktop suite sharding, 2026-10-22).
    for job, saver, names in [('numerical', 'numerical', ['Locked Cargo numerical suite']),
                              ('product', 'browser-record', ['Production build', 'Core browser journey and save/reopen']),
                              ('desktop', 'browser-record', ['Desktop behaviour (including persistence and undo)'])]:
        steps = jobs[job]['steps']
        restore = next(s for s in steps if s.get('uses') == 'actions/cache/restore@v4')
        assert restore['if'] == "github.event_name == 'pull_request'"
        assert 'restore-keys' not in restore['with']
        saver_steps = jobs[saver]['steps']
        save = next(s for s in saver_steps if s.get('uses') == 'actions/cache/save@v4')
        assert 'success()' in save['if']
        assert save['with']['key'] == restore['with']['key']
        saver_restore = next(s for s in saver_steps if s.get('uses') == 'actions/cache/restore@v4')
        assert 'restore-keys' not in saver_restore['with']
        for name in names:
            step = next(s for s in steps if s.get('name') == name)
            assert step['if'] == "steps.verified.outputs.cache-hit != 'true'"


def test_selection_identity_changes_reuse_key_even_with_identical_commands(tmp_path):
    for argv in [('init', '-q'), ('config', 'user.email', 'test@example.invalid'), ('config', 'user.name', 'Fixture')]:
        subprocess.run(['git', '-C', str(tmp_path), *argv], check=True, capture_output=True)
    (tmp_path / 'input').write_text('fixture')
    subprocess.run(['git', '-C', str(tmp_path), 'add', '.'], check=True, capture_output=True)
    subprocess.run(['git', '-C', str(tmp_path), 'commit', '-qm', 'fixture'], check=True, capture_output=True)
    selection = {'classification': 'reachability', 'manifests': ['core/a/Cargo.toml'], 'reasons': [{'kind': 'cargo-owner'}]}
    def key(value):
        return cache.fingerprint(tmp_path, 'numerical', ['same command'], {'image': 'v1'}, value)
    first = key(selection)
    for field, value in [('classification', 'full-fallback'), ('manifests', ['core/b/Cargo.toml']), ('reasons', [{'kind': 'exact-file-reader'}])]:
        assert key({**selection, field: value}) != first


def test_record_stands_only_for_a_key_every_verifier_ran_with(tmp_path):
    def publish(name, key):
        (tmp_path / ('browser-key-' + name)).mkdir(exist_ok=True)
        (tmp_path / ('browser-key-' + name) / 'key').write_text(key)
    key = 'verified-v2-browser-' + 'a' * 64
    publish('product', key)
    assert cache.agreed_key(tmp_path, ['product']) == key
    # A selected shard that published nothing cannot be vouched for.
    assert cache.agreed_key(tmp_path, ['product', 'desktop-1', 'desktop-2']) is None
    publish('desktop-1', key); publish('desktop-2', key)
    assert cache.agreed_key(tmp_path, ['product', 'desktop-1', 'desktop-2']) == key
    # A verifier on another runner image computed another key: save nothing.
    publish('desktop-2', 'verified-v2-browser-' + 'b' * 64)
    assert cache.agreed_key(tmp_path, ['product', 'desktop-1', 'desktop-2']) is None
    publish('product', 'verified-v2-numerical-' + 'a' * 64)
    assert cache.agreed_key(tmp_path, ['product']) is None
