"""The Repository checks list (tools/ci/repository_checks.py): its order, its base and its failure."""
import importlib.util
import json
import subprocess
from pathlib import Path
from types import SimpleNamespace

import pytest

ROOT = Path(__file__).resolve().parents[2]
spec = importlib.util.spec_from_file_location('repository_checks', ROOT / 'tools/ci/repository_checks.py')
checks = importlib.util.module_from_spec(spec)
spec.loader.exec_module(checks)

PY = 'python3'
SHA = 'a' * 40


def git(root, *args):
    return subprocess.run(['git', '-C', str(root), '-c', 'user.name=T', '-c', 'user.email=t@example.invalid',
                           '-c', 'commit.gpgsign=false', *args], check=True, capture_output=True, text=True).stdout.strip()


@pytest.fixture
def repo(tmp_path):
    """Two commits, no remote; nothing here reaches the network."""
    git(tmp_path, 'init', '-q')
    for text in ('one\n', 'two\n'):
        (tmp_path / 'f.txt').write_text(text)
        git(tmp_path, 'add', 'f.txt')
        git(tmp_path, 'commit', '-qm', text.strip())
    (tmp_path / checks.PLAN).write_text(json.dumps({'deselect': []}))
    return tmp_path


class Recorder:
    """A runner that records each command and fails the one whose step is named."""

    def __init__(self, root, fail=None, deselect=()):
        self.root, self.fail, self.deselect, self.commands = root, fail, list(deselect), []

    def __call__(self, command, cwd, stdout=None):
        assert Path(cwd) == self.root
        self.commands.append((command, stdout.name if stdout else None))
        if command[1:3] == ['tools/software_workflow/hosted_ci.py', 'repository']:
            (self.root / checks.PLAN).write_text(json.dumps({'deselect': self.deselect}))
        return SimpleNamespace(returncode=1 if self.fail and self.fail in command else 0)


def expected(event, base, documents_base, head='HEAD', deselect=()):
    return [
        ([PY, 'tools/software_workflow/hosted_ci.py', 'repository', '--event', event, '--base', base, '--head', head,
          '--output', 'repository-ci-plan.json'], None),
        ([PY, '-m', 'pytest', '-q', '-n', 'auto', *[f'--deselect={t}' for t in deselect],
          'tools/software_workflow', 'tools/deliverables', 'tools/validation', 'tests/test_ci_numerical.py',
          'tests/test_ci_apt_sources.py', 'tests/test_release_members.py', 'tests/test_release_candidate_scan.py',
          'tests/test_repository_history.py'], None),
        ([PY, '-m', 'pytest', '-q',
          'tests/test_ci_python.py::test_every_maintained_test_runs_when_a_path_it_reads_changes',
          'tests/test_ci_python.py::test_a_specification_a_test_only_names_is_never_its_probe'], None),
        ([PY, '-m', 'tools.deliverables', '--project', '.', 'check'], 'deliverable-check.json'),
        ([PY, 'tools/software_workflow/documentation_ci.py', 'documents', '--base', documents_base], None),
        ([PY, 'tools/validation/validate_conflict_markers.py', '--base', documents_base, '--head', 'HEAD'], None),
        ([PY, 'tools/validation/validate_run_record_leaks.py', '--base', documents_base, '--head', 'HEAD'], None),
    ]


def commands(recorder):
    return [(command, None if output is None else Path(output).name) for command, output in recorder.commands]


def test_the_steps_run_in_the_job_s_order_with_its_names_and_output_files(repo, capsys, monkeypatch):
    monkeypatch.delenv('GITHUB_ACTIONS', raising=False)
    base = git(repo, 'rev-parse', 'HEAD^1')
    plan = ['tests/test_ci_numerical.py::A::one', 'tests/test_ci_numerical.py::B::two']
    recorder = Recorder(repo, deselect=plan)
    assert checks.run(repo, 'pull_request', base, 'HEAD', PY, recorder) == 0
    # The pytest step passes exactly the plan's omissions, written by the selection step before it.
    assert commands(recorder) == expected('pull_request', base, base, deselect=plan)
    shown = [line[3:] for line in capsys.readouterr().out.splitlines() if line.startswith('== ')]
    assert shown == list(checks.STEP_NAMES)
    assert (repo / 'deliverable-check.json').exists()


def test_the_first_failing_step_ends_the_run_with_its_status(repo, capsys, monkeypatch):
    monkeypatch.delenv('GITHUB_ACTIONS', raising=False)  # the local form; the hosted form is the next test's
    base = git(repo, 'rev-parse', 'HEAD^1')
    recorder = Recorder(repo, fail='tools.deliverables')
    assert checks.run(repo, 'pull_request', base, 'HEAD', PY, recorder) == 1
    assert commands(recorder) == expected('pull_request', base, base)[:4]
    assert 'Repository checks failed at: Deliverable dependency structure' in capsys.readouterr().err


def test_hosted_logs_group_each_step_and_annotate_the_failure(repo, capsys, monkeypatch):
    monkeypatch.setenv('GITHUB_ACTIONS', 'true')
    base = git(repo, 'rev-parse', 'HEAD^1')
    assert checks.run(repo, 'pull_request', base, 'HEAD', PY, Recorder(repo, fail='-n')) == 1
    out = capsys.readouterr().out
    assert '::group::Select repository regressions' in out
    assert '::error::Bounded shared-tool and routing regressions:' in out
    assert '::group::Every maintained Python test follows the paths it reads' not in out


@pytest.mark.parametrize('event, base', [('pull_request', 'missing'), ('pull_request', SHA), ('workflow_dispatch', ''),
                                         ('pull_request', '')])
def test_a_base_that_names_no_commit_falls_back_to_the_head_s_parent(repo, event, base):
    """As the job's shell: `git rev-parse --verify --quiet "${BASE}^{commit}"` failing gives HEAD^1, which is
    workflow dispatch's case (it has no base), while the selection keeps the base as given."""
    recorder = Recorder(repo)
    assert checks.run(repo, event, base, 'HEAD', PY, recorder) == 0
    assert commands(recorder) == expected(event, base, 'HEAD^1')


def test_a_merge_queue_base_that_names_no_commit_fails_closed(repo, capsys, monkeypatch):
    """A queue group can hold several pull requests, so HEAD^1 would omit earlier ones."""
    monkeypatch.delenv('GITHUB_ACTIONS', raising=False)
    recorder = Recorder(repo)
    assert checks.run(repo, 'merge_group', SHA, 'HEAD', PY, recorder) == 1
    assert commands(recorder) == expected('merge_group', SHA, 'unused')[:4]
    assert f'Execution boundary and changed specification structure: merge_group base {SHA} is unavailable' in (
        capsys.readouterr().err)


def test_a_named_base_is_used_as_given(repo):
    git(repo, 'branch', 'main-copy', 'HEAD^1')
    recorder = Recorder(repo)
    assert checks.run(repo, 'merge_group', 'main-copy', 'HEAD', PY, recorder) == 0
    assert commands(recorder) == expected('merge_group', 'main-copy', 'main-copy')


def test_the_list_validates_and_never_refreshes(repo):
    recorder = Recorder(repo)
    checks.run(repo, 'pull_request', 'HEAD^1', 'HEAD', PY, recorder)
    text = (ROOT / 'tools/ci/repository_checks.py').read_text()
    for command, _ in recorder.commands:
        assert not any(word in ' '.join(command) for word in ('refresh', 'regenerate', '--write', '--update'))
    assert 'refresh-audit' not in text and 'write_text' not in text


def test_the_registry_names_the_documented_command_for_every_change():
    """The local selection names the Repository checks for every change, whatever its route: the route also depends
    on modes, additions, encodings and the reader audit, which paths cannot show (docs/CI_STRATEGY.md)."""
    import sys
    sys.path.insert(0, str(ROOT / 'tools/software_workflow'))
    from select_affected_checks import select_checks
    profile = json.loads((ROOT / 'software-workflow.json').read_text())
    assert profile['checks']['repository-checks'] == {
        'cwd': '.', 'command': ['python3', 'tools/ci/repository_checks.py', '--event', 'pull_request',
                                '--base', 'origin/main', '--head', 'HEAD']}
    assert 'repository-checks' in profile['always_checks']
    # Selected with no path at all, so for every change, including the declarative records (whose paths the
    # execution boundary keeps out of maintained code).
    assert select_checks(profile, [])['checks'] == ['repository-checks']
    for paths in (['docs/CI_STRATEGY.md'], ['README.md'], ['AGENTS.md'],  # documentation route only when audited
                  ['tools/ci/repository_checks.py'], ['core/units/adapter.py'], ['.gitignore'],
                  ['.github/workflows/governance-harness.yml'], ['docs/CI_STRATEGY.md', 'tools/REGISTRY.md']):
        assert 'repository-checks' in select_checks(profile, paths)['checks'], paths


def test_a_local_run_leaves_its_output_files_ignored():
    ignored = subprocess.run(['git', '-C', str(ROOT), 'check-ignore', '--no-index', checks.PLAN, checks.DELIVERABLE_CHECK],
                             capture_output=True, text=True)
    assert ignored.returncode == 0 and ignored.stdout.split() == [checks.PLAN, checks.DELIVERABLE_CHECK]
