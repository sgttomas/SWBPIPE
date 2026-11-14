"""The numerical job's reporting-runner shard writes the headless artifacts at the tested head and reads them.

tests/test_pressure_headless_artifacts.py and tests/test_load_reference_headless_artifacts.py skip without the
artifacts the headless runner's Rust tests write. The owner-approved step (2026-11-13) runs inside the existing
numerical job, acts only on the reporting-runner shard when its plan selects the headless runner, and fails on
any skip (docs/CI_STRATEGY.md).
"""
import json
import os
from pathlib import Path
import re
import subprocess

import yaml

ROOT = Path(__file__).resolve().parents[2]
STEP = 'Write the headless artifacts at the tested head and read them'
HEADLESS = 'core/runner/headless/Cargo.toml'
READERS = ('tests/test_pressure_headless_artifacts.py', 'tests/test_load_reference_headless_artifacts.py')
VARIABLES = ('HEADLESS_PRESSURE_OUTPUT_DIR', 'HEADLESS_LOAD_REFERENCE_OUTPUT_DIR', 'HEADLESS_LOAD_REFERENCE_SOURCE_OUTPUT_DIR')


def numerical_steps():
    config = yaml.load((ROOT / '.github/workflows/piping-desktop-e2e.yml').read_text(), Loader=yaml.BaseLoader)
    return config['jobs']['numerical']['steps']


def the_step():
    steps = numerical_steps()
    [index] = [i for i, step in enumerate(steps) if step.get('name') == STEP]
    return steps, index, steps[index]


def test_step_follows_the_suite_inside_the_numerical_job_and_its_result_reuse():
    steps, index, step = the_step()
    names = [s.get('name') for s in steps]
    # After the suite has built the headless runner's tests; before the verified result is recorded, so a reused
    # pass covers this step too, and gated like the suite on that reuse.
    assert names.index('Locked Cargo numerical suite') < index < names.index('Record successful numerical verification')
    assert step['if'] == "steps.verified.outputs.cache-hit != 'true'"
    assert 'matrix.shard' not in step['if']
    assert step['env']['SHARD'] == '${{ matrix.shard }}'
    assert step['shell'] == 'bash'


def test_step_writes_fresh_artifacts_at_the_head_then_reads_them_with_skips_refused():
    _, _, step = the_step()
    run = step['run']
    assert run.startswith('set -euo pipefail\n')
    assert 'python3 -m pip install -r requirements-dev.txt' in run
    # The suite's own target directory (numerical_ci.py --evidence-dir piping-numerical-evidence), so the
    # producers' test binaries are reused rather than rebuilt.
    suite = next(s for s in numerical_steps() if s.get('name') == 'Locked Cargo numerical suite')
    assert '--evidence-dir piping-numerical-evidence ' in suite['run']
    assert 'export CARGO_TARGET_DIR="$GITHUB_WORKSPACE/piping-numerical-evidence/target"' in run
    for variable in VARIABLES:
        assert re.search(rf'^export {variable}="\$out/[a-z-]+"$', run, re.M), variable
    assert 'rm -rf "$out"' in run and run.index('rm -rf "$out"') < run.index('mkdir -p')
    pressure = run.index('cargo test --offline --locked --manifest-path core/runner/headless/Cargo.toml --test pressure_export --test load_reference_cli')
    route = run.index('cargo test --offline --locked --manifest-path core/runner/headless/Cargo.toml --lib load_reference_route_tests')
    readers = run.index('HEADLESS_ARTIFACTS_REQUIRED=1 python3 -m pytest')
    assert max(pressure, route) < readers
    for reader in READERS:
        assert reader in run[readers:]
        assert (ROOT / reader).is_file()
    assert '--junitxml="$out/readers.xml"' in run


def gate(tmp_path, shard, plan):
    """Run the step's script up to its selection gate."""
    _, _, step = the_step()
    lines = step['run'].splitlines()
    end = next(i for i, line in enumerate(lines) if line.startswith('if [ "$selected" != yes ]'))
    (tmp_path / 'piping-ci-plan.json').write_text(json.dumps(plan))
    script = '\n'.join(lines[:end + 1] + ['echo SELECTED'])
    return subprocess.run(['bash', '-c', script], cwd=tmp_path, env={**os.environ, 'SHARD': shard},
                          capture_output=True, text=True)


def test_step_acts_only_on_the_reporting_runner_shard_when_the_headless_runner_is_selected(tmp_path):
    for shard, manifests, acts in [('reporting-runner', [HEADLESS], True),
                                   ('reporting-runner', ['core/reporting/result_export/Cargo.toml', HEADLESS], True),
                                   ('reporting-runner', ['core/reporting/result_export/Cargo.toml'], False),
                                   ('reporting-runner', [], False),
                                   ('physics', [HEADLESS], False), ('other', [HEADLESS], False)]:
        result = gate(tmp_path, shard, {'piping_numerical_selection': {'manifests': manifests}})
        assert result.returncode == 0, result.stderr
        assert ('SELECTED' in result.stdout) == acts, (shard, manifests)
    # An unreadable plan fails the step; it is never taken for "not selected".
    result = gate(tmp_path, 'reporting-runner', {})
    assert result.returncode != 0 and 'SELECTED' not in result.stdout


def test_reader_counts_fail_on_any_skip_failure_error_or_empty_run(tmp_path):
    _, _, step = the_step()
    match = re.search(r"<<'PYTHON'\n(.*?)\nPYTHON", step['run'], re.S)
    assert match
    check = tmp_path / 'check.py'
    check.write_text(match.group(1))
    report = tmp_path / 'readers.xml'
    for counts, passes in [({'tests': 34}, True), ({'tests': 34, 'skipped': 1}, False), ({'tests': 34, 'failures': 1}, False),
                           ({'tests': 34, 'errors': 1}, False), ({'tests': 0}, False)]:
        attributes = ' '.join(f'{key}="{value}"' for key, value in counts.items())
        report.write_text(f'<testsuites><testsuite name="pytest" {attributes}></testsuite></testsuites>')
        result = subprocess.run(['python3', str(check), str(report)], capture_output=True, text=True)
        assert (result.returncode == 0) == passes, (counts, result.stderr)
        assert 'headless artifact readers:' in result.stdout


def test_declared_artifact_inputs_are_the_dependency_lists_the_step_installs():
    """Each dependency list the step installs is a declared input of the artifact-reader rule, and only those."""
    _, _, step = the_step()
    installed = set(re.findall(r'pip install -r (\S+)', step['run']))
    assert installed == {'requirements-dev.txt'}
    profile = json.loads((ROOT / 'tools/hosted-ci-routing.json').read_text())
    assert set(profile['piping_artifact_readers']['inputs']) == installed
