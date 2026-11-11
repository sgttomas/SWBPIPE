"""Original history identities must select the recorded project tree, offline."""
from pathlib import Path
import os
import subprocess
import pytest
from tools import repository_history
from tools.repository_history import resolve_revision
ROOT = Path(__file__).resolve().parents[1]
BASIS = '2243be380ba6f9b45ef62ea25d2ee0dd968b2a25'
BASIS_EXTRACTED = '51eb3bcb31279d1e4e99dcd08312e9455773665e'


def test_required_source_basis_resolves_to_recorded_import():
    assert resolve_revision(ROOT, 'source:' + BASIS) == BASIS_EXTRACTED
    assert resolve_revision(ROOT, BASIS[:10]) == BASIS_EXTRACTED


def test_every_recorded_identity_resolves_to_its_recorded_tree():
    for original, (extracted, _) in repository_history.EXTRACTED.items():
        assert resolve_revision(ROOT, original) == extracted


def test_wrong_existing_target_cannot_select_another_tree(monkeypatch):
    table = dict(repository_history.EXTRACTED)
    other = next(commit for original, (commit, _) in table.items() if original != BASIS)
    table[BASIS] = (other, table[BASIS][1])
    monkeypatch.setattr(repository_history, 'EXTRACTED', table)
    with pytest.raises(ValueError, match='tree differs'):
        resolve_revision(ROOT, 'source:' + BASIS)


def test_missing_and_ambiguous_identity_refuse(monkeypatch):
    with pytest.raises(ValueError, match='missing or ambiguous'):
        resolve_revision(ROOT, 'source:' + '0' * 40)
    table = dict(repository_history.EXTRACTED)
    table[BASIS[:39] + ('0' if BASIS[39] != '0' else '1')] = table[BASIS]
    monkeypatch.setattr(repository_history, 'EXTRACTED', table)
    with pytest.raises(ValueError, match='missing or ambiguous'):
        resolve_revision(ROOT, 'source:' + BASIS[:12])


# A partial (blob:none) clone whose promisor remote is a trap: any lazy fetch runs the trap, which records the attempt
# and fails. Local file transport only; no network and no credentials.
GIT_ENV = {'GIT_CONFIG_GLOBAL': os.devnull, 'GIT_CONFIG_NOSYSTEM': '1', 'GIT_AUTHOR_NAME': 't', 'GIT_AUTHOR_EMAIL': 't@t',
           'GIT_COMMITTER_NAME': 't', 'GIT_COMMITTER_EMAIL': 't@t'}
ABSENT = '0123456789abcdef0123456789abcdef01234567'
ORIGINAL = 'fedcba9876543210fedcba9876543210fedcba98'


def _git(*args, cwd=None):
    env = {k: v for k, v in os.environ.items() if not k.startswith('GIT_')} | GIT_ENV
    return subprocess.run(['git', *args], cwd=cwd, env=env, check=True, capture_output=True, text=True).stdout.strip()


@pytest.fixture
def promisor(tmp_path, monkeypatch):
    """(clone, trap log, ordinary commit, mapped target commit). ORIGINAL maps to the target, whose tree is recorded,
    in a table that replaces the repository's for the test.
    The test runs in a git environment without inherited GIT_* variables (GIT_NO_LAZY_FETCH among them), so only the
    resolver's own command line can keep a lookup local."""
    for name in [name for name in os.environ if name.startswith('GIT_')]:
        monkeypatch.delenv(name)
    for name, value in GIT_ENV.items():
        monkeypatch.setenv(name, value)
    src = tmp_path / 'src'
    _git('init', '-q', str(src))
    _git('config', 'uploadpack.allowFilter', 'true', cwd=src)
    (src / 'f').write_text('ordinary\n')
    _git('add', 'f', cwd=src)
    _git('commit', '-qm', 'ordinary', cwd=src)
    ordinary = _git('rev-parse', 'HEAD', cwd=src)
    (src / 'g').write_text('target\n')
    _git('add', 'g', cwd=src)
    _git('commit', '-qm', 'target', cwd=src)
    target, tree = _git('rev-parse', 'HEAD', cwd=src), _git('rev-parse', 'HEAD^{tree}', cwd=src)
    monkeypatch.setattr(repository_history, 'EXTRACTED', {ORIGINAL: (target, tree)})
    clone = tmp_path / 'clone'
    _git('clone', '-q', '--filter=blob:none', (src.as_uri()), str(clone))
    assert _git('config', 'remote.origin.promisor', cwd=clone) == 'true'
    log = tmp_path / 'trap.log'
    trap = tmp_path / 'trap.sh'
    trap.write_text('#!/bin/sh\necho fetch >> "%s"\nexit 1\n' % log)
    trap.chmod(0o755)
    _git('config', 'remote.origin.uploadpack', str(trap), cwd=clone)
    return clone, log, ordinary, target


def test_the_trap_detects_a_lazy_fetch(promisor):
    """Control: without --no-lazy-fetch, git asks the promisor remote for the absent object."""
    clone, log, _, _ = promisor
    result = subprocess.run(['git', '-C', str(clone), 'rev-parse', '--verify', ABSENT + '^{commit}'],
                            capture_output=True, text=True)
    assert result.returncode == 128 and log.is_file()


def test_absent_revision_is_refused_without_a_fetch(promisor):
    clone, log, _, _ = promisor
    with pytest.raises(ValueError, match='missing or ambiguous'):
        resolve_revision(clone, ABSENT)
    with pytest.raises(ValueError, match='missing or ambiguous'):
        resolve_revision(clone, ABSENT, env=dict(os.environ) | GIT_ENV, git_options=('--no-replace-objects',))
    assert not log.exists()


def test_present_revision_resolves_without_a_fetch(promisor):
    clone, log, ordinary, _ = promisor
    assert resolve_revision(clone, ordinary) == ordinary
    assert resolve_revision(clone, ordinary[:12]) == ordinary
    assert not log.exists()


def test_mapped_original_resolves_to_its_recorded_tree_without_a_fetch(promisor):
    clone, log, _, target = promisor
    assert resolve_revision(clone, ORIGINAL) == target
    assert resolve_revision(clone, 'source:' + ORIGINAL[:12]) == target
    assert not log.exists()


def test_a_failed_offline_lookup_is_refused_not_mapped(promisor, monkeypatch):
    """A git that rejects --no-lazy-fetch (exit 129) or fails otherwise is refused, even for a mapped original."""
    clone, log, _, _ = promisor
    real = subprocess.run
    def rejecting(command, *args, **kwargs):
        if '--no-lazy-fetch' in command:
            return subprocess.CompletedProcess(command, 129, '', 'unknown option: --no-lazy-fetch')
        return real(command, *args, **kwargs)
    monkeypatch.setattr(subprocess, 'run', rejecting)
    with pytest.raises(ValueError, match='Offline revision lookup failed'):
        resolve_revision(clone, ORIGINAL)
    assert not log.exists()
