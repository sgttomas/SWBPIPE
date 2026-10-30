"""Hosted routing must receive PR changes and retain an honest result gate."""
from pathlib import Path
import json

import yaml
from select_affected_checks import select_checks

ROOT = Path(__file__).resolve().parents[2]


def workflow(name):
    return yaml.load((ROOT / ".github/workflows" / name).read_text(), Loader=yaml.BaseLoader)


def test_changes_to_hosted_workflows_select_their_structural_tests():
    profile = json.loads((ROOT / "tools/tools-test-routing.json").read_text())
    for name in ["shared-build-cache.yml", "piping-desktop-e2e.yml"]:
        selection = select_checks(profile, [f".github/workflows/{name}"])
        assert "software_workflow" in selection["checks"]


def test_every_pr_reaches_single_required_aggregate_and_reusable_products():
    root = workflow('governance-harness.yml')
    assert 'pull_request' in root['on']
    assert not root['on']['pull_request']
    assert 'schedule' not in root['on']
    assert 'push' not in root['on']  # no duplicate post-merge product run
    jobs = root['jobs']
    assert jobs['harness']['if'] == 'always()'
    assert set(jobs['harness']['needs']) == {'repository-checks', 'piping'}
    for job, name in [('piping', 'piping-desktop-e2e.yml')]:
        config = workflow(name)
        assert 'workflow_call' in config['on']
        assert 'workflow_dispatch' in config['on']
        assert 'pull_request' not in config['on']
        assert 'schedule' not in config['on']
        assert jobs[job]['uses'] == f'./.github/workflows/{name}'
        assert 'if' not in config['jobs']['selection']


def test_single_required_result_fails_for_any_unsuccessful_child(tmp_path):
    import os
    import subprocess
    step = workflow('governance-harness.yml')['jobs']['harness']['steps'][0]
    for state in ['success', 'failure', 'cancelled', 'skipped']:
        results = {name: {'result': 'success'} for name in ['repository-checks', 'piping']}
        results['piping']['result'] = state
        process = subprocess.run(['bash', '-c', step['run']], env={**os.environ, 'RESULTS': json.dumps(results)}, capture_output=True)
        assert (process.returncode == 0) == (state == 'success')


def test_detached_and_retired_products_have_no_workflows():
    for name in ('app-v4.yml', 'harness-premerge.yml', 'pec-tests.yml', 'desktop-release-template.yml'):
        assert not (ROOT / '.github/workflows' / name).exists()


def test_runner_context_is_not_evaluated_in_job_level_environment():
    # Runner context is available in step env, but not in jobs.<id>.env.
    # The old whole-file substring ban rejected valid step-level usage too.
    for name in ["shared-build-cache.yml", "piping-desktop-e2e.yml"]:
        for job in workflow(name)["jobs"].values():
            assert all("${{ runner." not in str(value) for value in job.get("env", {}).values())


def test_python_gate_and_result_are_independent_of_cargo():
    jobs = workflow('piping-desktop-e2e.yml')['jobs']
    assert jobs['numerical']['if'] == "needs.selection.outputs.numerical_mode == 'full'"
    assert jobs['python-product']['if'] == "needs.selection.outputs.python_mode == 'full'"
    assert jobs['selection']['outputs']['python_mode'] == '${{ steps.numeric.outputs.python_mode }}'
    aggregate = jobs['desktop-e2e']['steps'][-1]
    assert aggregate['env']['PYTHON_MODE'] == '${{ needs.selection.outputs.python_mode }}'
    assert 'aggregate --suite piping-python --mode "$PYTHON_MODE"' in aggregate['run']
    # The sharded desktop job joins the aggregate (desktop suite sharding, 2026-10-22).
    assert set(jobs['desktop-e2e']['needs']) == {'selection', 'product', 'desktop', 'desktop-rust', 'numerical', 'python-product'}


def test_selected_desktop_rust_tests_run_on_macos_and_gate_the_aggregate(tmp_path):
    import os
    import subprocess
    jobs = workflow('piping-desktop-e2e.yml')['jobs']
    assert jobs['selection']['outputs']['desktop_rust_mode'] == '${{ steps.numeric.outputs.desktop_rust_mode }}'
    read = next(s for s in jobs['selection']['steps'] if s.get('id') == 'numeric')
    assert "('piping-desktop-rust', 'desktop_rust_mode')" in read['run']
    job = jobs['desktop-rust']
    assert job['needs'] == 'selection'
    assert job['if'] == "needs.selection.outputs.desktop_rust_mode == 'full'"
    # The product platform (DEC-057); the job never reuses a result.
    assert job['runs-on'] == 'macos-26'
    run = next(s for s in job['steps'] if s.get('name') == 'Locked desktop shell Cargo tests')
    assert run['run'] == 'cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml'
    assert run['working-directory'] == '.'
    assert 'if' not in run
    assert not any(s.get('uses', '').startswith('actions/cache') for s in job['steps'])
    aggregate = jobs['desktop-e2e']['steps'][-1]
    assert aggregate['env']['DESKTOP_RUST_MODE'] == '${{ needs.selection.outputs.desktop_rust_mode }}'
    assert aggregate['env']['DESKTOP_RUST'] == '${{ needs.desktop-rust.result }}'
    assert 'aggregate --suite piping-desktop-rust --mode "$DESKTOP_RUST_MODE" --selection "$SELECTION" --product "$DESKTOP_RUST"' in aggregate['run']
    # Executed as the workflow runs it: selected and failed, cancelled or skipped
    # fails; unselected passes only when the job was skipped.
    script = '\n'.join(line for line in aggregate['run'].splitlines() if 'piping-desktop-rust' in line)
    for mode, result, passed in [('full', 'success', True), ('full', 'failure', False), ('full', 'cancelled', False),
                                 ('full', 'skipped', False), ('not-applicable', 'skipped', True),
                                 ('not-applicable', 'success', False)]:
        env = {**os.environ, 'DESKTOP_RUST_MODE': mode, 'SELECTION': 'success', 'DESKTOP_RUST': result}
        process = subprocess.run(['bash', '-e', '-c', script], cwd=ROOT, env=env, capture_output=True)
        assert (process.returncode == 0) == passed, (mode, result)


def test_selected_desktop_suite_runs_in_the_required_product_job_and_keys_its_reuse(tmp_path):
    import os
    import subprocess
    import sys
    jobs = workflow('piping-desktop-e2e.yml')['jobs']
    assert jobs['selection']['outputs']['desktop_mode'] == '${{ steps.numeric.outputs.desktop_mode }}'
    assert not any(s.get('name') == 'Desktop behaviour (including persistence and undo)' for s in jobs['product']['steps'])
    desktop_job = jobs['desktop']
    assert desktop_job['if'] == "needs.selection.outputs.desktop_mode == 'full'"
    assert desktop_job['strategy']['matrix']['shard'] == ['1', '2']
    assert desktop_job['strategy']['fail-fast'] == 'false'
    desktop = next(s for s in desktop_job['steps'] if s.get('name') == 'Desktop behaviour (including persistence and undo)')
    assert desktop['run'] == 'npm test --workspace apps/desktop -- --shard=${{ matrix.shard }}/2'
    assert desktop['if'] == "steps.verified.outputs.cache-hit != 'true'"
    # Every verifying job computes the same mode-aware key and publishes the one it ran with.
    for job, artifact in (('product', 'browser-key-product'), ('desktop', 'browser-key-desktop-${{ matrix.shard }}')):
        key = next(s for s in jobs[job]['steps'] if s.get('id') == 'result-key')
        assert key['env']['DESKTOP_MODE'] == '${{ needs.selection.outputs.desktop_mode }}'
        assert '--desktop-mode "$DESKTOP_MODE"' in key['run']
        upload = next(s for s in jobs[job]['steps'] if s.get('uses') == 'actions/upload-artifact@v4' and s['with']['name'] == artifact)
        assert upload['if'] == 'success()'
    # The recorder saves only under the key every expected verifier published.
    agree = next(s for s in jobs['browser-record']['steps'] if s.get('id') == 'result-key')
    assert 'result_cache.py agree --suite browser' in agree['run']
    assert '--expect product' in agree['run'] and '--expect desktop-1 --expect desktop-2' in agree['run']
    assert not any('result_cache.py key' in s.get('run', '') for s in jobs['browser-record']['steps'])
    for step in jobs['browser-record']['steps']:
        if step.get('uses') in ('actions/cache/restore@v4', 'actions/cache/save@v4') or 'result_cache.py save' in step.get('run', ''):
            assert "steps.result-key.outputs.key != ''" in step['if']
    # The record follows every job it vouches for.
    record = jobs['browser-record']
    assert set(record['needs']) == {'selection', 'product', 'desktop'}
    assert "needs.product.result == 'success'" in record['if']
    assert "needs.desktop.result == 'success' || (needs.desktop.result == 'skipped' && needs.selection.outputs.desktop_mode != 'full')" in record['if']
    assert not any(s.get('uses') == 'actions/cache/save@v4' for s in jobs['product']['steps'] + desktop_job['steps'])
    aggregate = jobs['desktop-e2e']['steps'][-1]
    assert aggregate['env']['DESKTOP_MODE'] == '${{ needs.selection.outputs.desktop_mode }}'
    assert aggregate['env']['DESKTOP'] == '${{ needs.desktop.result }}'
    assert 'aggregate --suite piping-desktop --mode "$DESKTOP_MODE" --selection "$SELECTION" --product "$DESKTOP"' in aggregate['run']
    # A browser pass recorded without the desktop suite cannot be reused for a run that needs it.
    env = {**os.environ, 'GITHUB_OUTPUT': str(tmp_path / 'out'), 'ImageOS': 'ubuntu24', 'ImageVersion': '1',
           'RUNNER_OS': 'Linux', 'RUNNER_ARCH': 'X64'}
    command = [sys.executable, str(ROOT / 'tools/software_workflow/result_cache.py'), 'key', '--suite', 'browser',
               '--record', str(tmp_path / 'record.json'), '--browser-project', 'chromium-desktop', '--browser-specs', 'r2-smoke.spec.ts']
    keys = {}
    for mode in ('full', 'not-applicable'):
        keys[mode] = subprocess.run(command + ['--desktop-mode', mode], env=env, capture_output=True, text=True, check=True).stdout
    assert keys['full'] != keys['not-applicable']
    assert subprocess.run(command, env=env, capture_output=True).returncode != 0


def test_selected_browser_specs_drive_the_product_job_and_key_its_reuse(tmp_path):
    """W-12: the routing file's browser specs reach Playwright through the plan,
    and a pass without a selected spec is never reused for a run needing it."""
    import os
    import re
    import subprocess
    import sys
    from hosted_ci import browser_plan, route_paths

    def select_paths(paths, profile):
        # The plan fields the step reads, from the real path rules and browser
        # planner without the full plan's source scans (test_hosted_ci.py checks
        # the plan carries exactly this selection).
        selection = route_paths(paths, profile)
        return {'modes': selection['modes'], 'piping_browser_selection': browser_plan(paths, selection, profile)}
    profile = json.loads((ROOT / 'tools/hosted-ci-routing.json').read_text())
    jobs = workflow('piping-desktop-e2e.yml')['jobs']
    selection = jobs['selection']
    assert selection['outputs']['browser_project'] == '${{ steps.numeric.outputs.browser_project }}'
    assert selection['outputs']['browser_specs'] == '${{ steps.numeric.outputs.browser_specs }}'
    # The product job runs the selected specs, never a list of its own.
    step = next(s for s in jobs['product']['steps'] if s.get('name') == 'Core browser journey and save/reopen')
    assert step['env'] == {'BROWSER_PROJECT': '${{ needs.selection.outputs.browser_project }}',
                           'BROWSER_SPECS': '${{ needs.selection.outputs.browser_specs }}'}
    assert 'npm run test:e2e:desktop -- --project="$BROWSER_PROJECT" $BROWSER_SPECS' in step['run']
    assert not re.search(r'\w+\.spec\.ts', step['run'])
    # Every verifier keys the same selected specs.
    for job in ('product', 'desktop'):
        key = next(s for s in jobs[job]['steps'] if s.get('id') == 'result-key')
        assert key['env']['BROWSER_SPECS'] == '${{ needs.selection.outputs.browser_specs }}'
        assert '--browser-project "$BROWSER_PROJECT" --browser-specs "$BROWSER_SPECS"' in key['run']
    # The selection job's own step turns plans into those outputs, as the workflow runs it.
    read = next(s for s in selection['steps'] if s.get('id') == 'numeric')['run']
    script = read.split("<<'PYTHON'\n", 1)[1].rsplit('PYTHON', 1)[0]
    project = 'apps/desktop/'
    outputs = {}
    for name, paths in [('replace', [project + 'src/features/project-storage/unsavedChangesGuard.ts']),
                        ('unrelated', [project + 'src/App.tsx'])]:
        (tmp_path / 'piping-ci-plan.json').write_text(json.dumps(select_paths(paths, profile)))
        out = tmp_path / ('out-' + name)
        subprocess.run([sys.executable, '-c', script], cwd=tmp_path, env={**os.environ, 'GITHUB_OUTPUT': str(out)}, check=True,
                       capture_output=True)
        outputs[name] = dict(line.split('=', 1) for line in out.read_text().splitlines())
        assert outputs[name]['browser_project'] == 'chromium-desktop'
    assert outputs['unrelated']['browser_specs'] == 'r2-smoke.spec.ts b3b-project-persistence.spec.ts rotation-glyph.spec.ts result-overlays.spec.ts'
    assert outputs['replace']['browser_specs'] == (outputs['unrelated']['browser_specs'] + ' ui-foundation.spec.ts c3-viewport-visibility.spec.ts'
                                                   ' gui-workflow-validation.spec.ts linear-authoring.spec.ts')
    # A plan whose spec could inject into the command line is refused.
    plan = select_paths([project + 'src/App.tsx'], profile)
    plan['piping_browser_selection']['specs'].append('x.spec.ts;true')
    (tmp_path / 'piping-ci-plan.json').write_text(json.dumps(plan))
    assert subprocess.run([sys.executable, '-c', script], cwd=tmp_path, env={**os.environ, 'GITHUB_OUTPUT': str(tmp_path / 'bad')},
                          capture_output=True).returncode != 0
    # The reuse key differs between the two selections, and cannot be computed without one.
    env = {**os.environ, 'GITHUB_OUTPUT': str(tmp_path / 'out'), 'ImageOS': 'ubuntu24', 'ImageVersion': '1',
           'RUNNER_OS': 'Linux', 'RUNNER_ARCH': 'X64'}
    command = [sys.executable, str(ROOT / 'tools/software_workflow/result_cache.py'), 'key', '--suite', 'browser',
               '--record', str(tmp_path / 'record.json'), '--desktop-mode', 'full']
    keys = {name: subprocess.run(command + ['--browser-project', value['browser_project'], '--browser-specs', value['browser_specs']],
                                 env=env, capture_output=True, text=True, check=True).stdout for name, value in outputs.items()}
    assert keys['replace'] != keys['unrelated']
    for missing in ([], ['--browser-project', 'chromium-desktop'], ['--browser-project', 'chromium-desktop', '--browser-specs', ''],
                    ['--browser-project', 'chromium-desktop', '--browser-specs', 'x.spec.ts;true']):
        assert subprocess.run(command + missing, env=env, capture_output=True).returncode != 0


def test_repository_checks_run_claims_language_on_its_selection():
    steps = workflow('governance-harness.yml')['jobs']['repository-checks']['steps']
    [step] = [s for s in steps if s.get('run') == 'python3 tools/validation/validate_claims_language.py']
    assert 'if' not in step  # the standalone repository always checks product claims
