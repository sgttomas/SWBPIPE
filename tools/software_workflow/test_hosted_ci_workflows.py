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
    # The merge queue runs the same aggregate on its temporary merge commit.
    assert root['on']['merge_group'] == {'types': ['checks_requested']}
    assert set(root['on']) == {'pull_request', 'merge_group', 'workflow_dispatch'}
    assert 'schedule' not in root['on']
    assert 'push' not in root['on']  # no duplicate post-merge product run
    jobs = root['jobs']
    assert jobs['harness']['if'] == 'always()'
    assert set(jobs['harness']['needs']) == {'changes', 'repository-checks', 'piping'}
    for name in ('repository-checks', 'piping'):
        assert jobs[name]['needs'] == 'changes'
        assert jobs[name]['if'] == "needs.changes.outputs.mode == 'product'"
    for job, name in [('piping', 'piping-desktop-e2e.yml')]:
        config = workflow(name)
        assert 'workflow_call' in config['on']
        assert 'workflow_dispatch' in config['on']
        assert 'pull_request' not in config['on']
        assert 'merge_group' not in config['on']  # reached only through the caller
        assert 'schedule' not in config['on']
        assert jobs[job]['uses'] == f'./.github/workflows/{name}'
        assert 'if' not in config['jobs']['selection']


HEAD_REF = '${{ github.event.pull_request.head.sha || github.event.merge_group.head_sha || github.sha }}'
BASE_REF = '${{ github.event.pull_request.base.sha || github.event.merge_group.base_sha }}'


def test_merge_group_candidate_is_checked_out_and_selected_as_a_pull_request():
    """Every checkout and selection names the queue's merge commit and base
    where it names a pull request's head and base."""
    for name in ('governance-harness.yml', 'piping-desktop-e2e.yml'):
        text = (ROOT / '.github/workflows' / name).read_text()
        for pr_only in ('${{ github.event.pull_request.head.sha || github.sha }}',
                        'BASE: ${{ github.event.pull_request.base.sha }}\n',
                        '${{ github.event.pull_request.base.sha || github.event.before }}'):
            assert pr_only not in text, (name, pr_only)
        selections = 0
        for job in workflow(name)['jobs'].values():
            for step in job.get('steps', []):
                if str(step.get('uses', '')).startswith('actions/checkout@'):
                    assert step['with']['ref'] == HEAD_REF, (name, step)
                if '--event "$EVENT"' in step.get('run', ''):
                    selections += 1
                    assert step['env'] == {'EVENT': '${{ github.event_name }}', 'BASE': BASE_REF, 'HEAD': HEAD_REF}
        assert selections == {'governance-harness.yml': 2, 'piping-desktop-e2e.yml': 1}[name]
    for step in workflow('governance-harness.yml')['jobs']['repository-checks']['steps']:
        if 'BASE' in step.get('env', {}) and '--event' not in step['run']:
            assert step['env']['BASE'] == '${{ github.event.pull_request.base.sha || github.event.merge_group.base_sha || github.event.before }}'


def test_merge_group_runs_neither_cancel_nor_are_cancelled():
    for name, prefix in [('governance-harness.yml', 'governance-harness'), ('piping-desktop-e2e.yml', 'piping-desktop-e2e')]:
        concurrency = workflow(name)['concurrency']
        assert concurrency['group'] == prefix + '-${{ github.event.pull_request.number || github.event.merge_group.head_ref || github.ref }}'
        assert concurrency['cancel-in-progress'] == "${{ github.event_name == 'pull_request' }}"


def concurrency_group(expression, github):
    """A group as GitHub evaluates it, for expressions of context names, quoted
    strings, `==`, `&&`, `||` and parentheses. As in Python, `&&` and `||`
    return an operand, and null, '' and false are false."""
    import re
    assert all("'" not in str(value) and '}' not in str(value) for value in github.values())

    def value(match):
        assert re.fullmatch(r"[A-Za-z_.'=&|() -]*", match.group(1)), match.group(1)
        names = re.sub(r"github((?:\.\w+)+)", lambda name: f"lookup({name.group(1)[1:]!r})", match.group(1))
        return str(eval(names.replace('&&', ' and ').replace('||', ' or '), {'__builtins__': {}},
                        {'lookup': lambda dotted: github.get(dotted)}))
    return re.sub(r"\$\{\{(.*?)\}\}", value, expression)


def test_full_runs_of_main_share_one_group_and_candidate_groups_are_unchanged():
    """Runs of main go one at a time, so a scheduled skip is only ever decided
    from a finished run (nightly_ci.py relies on this)."""
    a = 'a' * 40
    main = {'ref': 'refs/heads/main'}
    pull = {'event_name': 'pull_request', 'ref': 'refs/pull/7/merge', 'sha': a, 'event.pull_request.number': 7}
    queue = {'event_name': 'merge_group', 'ref': 'refs/heads/gh-readonly-queue/main/pr-7-x', 'sha': a,
             'event.merge_group.head_ref': 'refs/heads/gh-readonly-queue/main/pr-7-x'}
    product = workflow('piping-desktop-e2e.yml')['concurrency']['group']
    nightly = workflow('nightly-full-ci.yml')['concurrency']
    assert nightly['cancel-in-progress'] == 'false'
    report = workflow('nightly-full-ci.yml')['jobs']['report']['concurrency']
    assert report['cancel-in-progress'] == 'false'
    groups = set()
    for event in ('schedule', 'workflow_dispatch'):
        scheduled = {**main, 'event_name': event, 'sha': a}
        groups.add(concurrency_group(product, scheduled))
        assert concurrency_group(product, scheduled) == 'piping-desktop-e2e-refs/heads/main'
        assert concurrency_group(nightly['group'], scheduled) == 'nightly-full-ci-refs/heads/main'
    # A pull request and a merge-queue group keep the groups they had, in both candidate workflows,
    # and a pull-request run is still the only kind that cancels its predecessor.
    harness = workflow('governance-harness.yml')['concurrency']
    assert concurrency_group(product, pull) == 'piping-desktop-e2e-7'
    assert concurrency_group(harness['group'], pull) == 'governance-harness-7'
    assert concurrency_group(product, queue) == 'piping-desktop-e2e-refs/heads/gh-readonly-queue/main/pr-7-x'
    assert concurrency_group(harness['group'], queue) == 'governance-harness-refs/heads/gh-readonly-queue/main/pr-7-x'
    assert not {concurrency_group(product, pull), concurrency_group(product, queue)} & groups
    cancel = workflow('piping-desktop-e2e.yml')['concurrency']['cancel-in-progress']
    assert [concurrency_group(cancel, {'event_name': event}) for event in ('pull_request', 'merge_group', 'schedule')] == [
        'True', 'False', 'False']


def test_merge_group_runs_exactly_the_pull_request_jobs():
    # The broad browser matrix is a dispatch-only run; a queued candidate runs what its pull request ran.
    matrix = workflow('piping-desktop-e2e.yml')['jobs']['browser-matrix']
    assert matrix['if'] == "github.event_name != 'pull_request' && github.event_name != 'merge_group' && needs.selection.outputs.mode == 'full'"


def test_full_runs_of_main_reuse_the_product_workflow_and_are_advisory():
    nightly = workflow('nightly-full-ci.yml')
    # Nightly and by hand; never on a candidate and never on a push to main.
    assert nightly['on'] == {'schedule': [{'cron': '0 9 * * *'}], 'workflow_dispatch': ''}
    assert nightly['permissions'] == {'contents': 'read'}
    jobs = nightly['jobs']
    assert set(jobs) == {'changed', 'full-suite', 'report'}
    # Neither the job ids nor the check names can be taken for the required `harness` context.
    for job_id, job in jobs.items():
        assert 'harness' not in job_id.lower() and 'harness' not in job['name'].lower()
        assert '${{' not in job['name']
    assert jobs['changed']['permissions'] == {'contents': 'read', 'actions': 'read'}
    [gate] = [s for s in jobs['changed']['steps'] if s.get('id') == 'decide']
    assert gate['env'] == {'GH_TOKEN': '${{ github.token }}', 'EVENT': '${{ github.event_name }}', 'SHA': '${{ github.sha }}'}
    assert gate['run'] == ('python3 tools/software_workflow/nightly_ci.py --event "$EVENT" --sha "$SHA" '
                           '--workflow nightly-full-ci.yml')
    assert jobs['changed']['outputs']['run'] == '${{ steps.decide.outputs.run }}'
    full = jobs['full-suite']
    assert full['needs'] == 'changed'
    assert full['if'] == "needs.changed.outputs.run == 'true'"
    # Reuse, not copies: the product workflow with full selection, keeping the routing suite in its Python shards.
    assert full['uses'] == './.github/workflows/piping-desktop-e2e.yml'
    assert full['with'] == {'repository-checks-run': 'false'}
    assert 'steps' not in full and 'permissions' not in full and 'secrets' not in full
    # The required harness gains no trigger and calls no nightly work.
    harness = workflow('governance-harness.yml')
    assert set(harness['on']) == {'pull_request', 'merge_group', 'workflow_dispatch'}
    assert 'nightly' not in (ROOT / '.github/workflows/governance-harness.yml').read_text()
    assert [name for name in harness['jobs'] if 'uses' in harness['jobs'][name]] == ['piping']
    # The broad matrix runs on the full run's events and not on candidates.
    matrix = workflow('piping-desktop-e2e.yml')['jobs']['browser-matrix']['if']
    for event in ('schedule', 'workflow_dispatch'):
        assert f"'{event}'" not in matrix
    # The product workflow selects from the caller's event.
    [select] = [s for s in workflow('piping-desktop-e2e.yml')['jobs']['selection']['steps'] if s.get('id') == 'plan']
    assert select['env']['EVENT'] == '${{ github.event_name }}' and '--event "$EVENT"' in select['run']


def test_only_the_report_job_may_write_issues_and_it_interpolates_nothing_into_its_command():
    text = (ROOT / '.github/workflows/nightly-full-ci.yml').read_text()
    nightly = workflow('nightly-full-ci.yml')
    report = nightly['jobs']['report']
    # It runs whatever became of the run, and decides from the job results in Python: a job stopped
    # by its time limit is reported as cancelled while the run is not, which failure() and cancelled() both miss.
    assert report['if'] == 'always()'
    assert set(report['needs']) == {'changed', 'full-suite'}
    assert report['permissions'] == {'contents': 'read', 'actions': 'read', 'checks': 'read', 'pull-requests': 'read',
                                     'issues': 'write'}
    for name in ('nightly-full-ci.yml', 'governance-harness.yml', 'piping-desktop-e2e.yml'):
        config = workflow(name)
        holders = [config.get('permissions', {})] + [job.get('permissions', {}) for job_id, job in config['jobs'].items()
                                                     if (name, job_id) != ('nightly-full-ci.yml', 'report')]
        assert all(set(held.values()) <= {'read'} for held in holders), name
    assert text.count('write') == 1 and 'secrets.' not in text and 'write-all' not in text
    [step] = [s for s in report['steps'] if 'run' in s]
    assert step['env'] == {'GH_TOKEN': '${{ github.token }}', 'RESULTS': '${{ toJSON(needs) }}'}
    assert step['run'] == 'python3 tools/software_workflow/nightly_report.py'
    # No expression reaches a shell, and no event text (a title, branch or message) is named anywhere in the workflow.
    for job in nightly['jobs'].values():
        for entry in job.get('steps', []):
            assert '${{' not in entry.get('run', '')
            if str(entry.get('uses', '')).startswith('actions/checkout@'):
                assert entry['with']['ref'] == '${{ github.sha }}'
    assert 'github.event.' not in text and 'github.head_ref' not in text and 'github.ref_name' not in text


def test_single_required_result_fails_for_any_unsuccessful_child(tmp_path):
    import os
    import subprocess
    step = workflow('governance-harness.yml')['jobs']['harness']['steps'][-1]
    for state in ['success', 'failure', 'cancelled', 'skipped']:
        results = {name: {'result': 'success'} for name in ['repository-checks', 'piping']}
        results['changes'] = {'result': 'success', 'outputs': {'mode': 'product'}}
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
    aggregate = next(s for s in jobs['desktop-e2e']['steps'] if s.get('name') == 'Report selected checks without treating skipped work as passed')
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
    aggregate = next(s for s in jobs['desktop-e2e']['steps'] if s.get('name') == 'Report selected checks without treating skipped work as passed')
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
    aggregate = next(s for s in jobs['desktop-e2e']['steps'] if s.get('name') == 'Report selected checks without treating skipped work as passed')
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


def test_only_the_harness_lets_the_product_job_skip_the_repository_routing_suite():
    harness = workflow('governance-harness.yml')['jobs']
    assert harness['piping']['with']['repository-checks-run'] == 'true'
    assert 'tests/test_ci_numerical.py' in harness['repository-checks']['steps'][
        [s.get('name') for s in harness['repository-checks']['steps']].index('Bounded shared-tool and routing regressions')]['run']
    product = workflow('piping-desktop-e2e.yml')
    assert product['on']['workflow_call']['inputs']['repository-checks-run']['default'] == 'false'
    assert 'inputs' not in (product['on'].get('workflow_dispatch') or {})
    [step] = [s for s in product['jobs']['python-product']['steps']
              if s.get('name') == 'Selected Python product contracts and qualification oracles']
    assert step['env']['REPOSITORY_CHECKS_RUN'] == '${{ inputs.repository-checks-run }}'
    assert '--repository-checks-run' in step['run']


def test_exact_wasm_cache_wiring_and_main_build_only_warmer():
    action = yaml.load((ROOT / '.github/actions/setup-piping-e2e/action.yml').read_text(), Loader=yaml.BaseLoader)
    steps = action['runs']['steps']
    by_id = {s['id']: s for s in steps if 'id' in s}
    assert by_id['engine-key']['run'] == 'python3 tools/software_workflow/wasm_cache.py key'
    assert all(by_id[name]['continue-on-error'] == 'true' for name in ('engine-key', 'engine-verified', 'engine-record'))
    restore = by_id['engine-cache']
    assert restore['uses'] == 'actions/cache/restore@v4'
    assert restore['with']['key'] == '${{ steps.engine-key.outputs.key }}'
    assert 'restore-keys' not in restore['with']
    assert restore['continue-on-error'] == 'true'
    build = next(s for s in steps if s.get('name') == 'Build desktop WASM engine')
    assert build['if'] == "steps.engine-verified.outputs.valid != 'true'"
    assert 'rm -rf apps/desktop/public/wasm-engine apps/desktop/public/self-weight-engine apps/desktop/public/mbf-engine' in build['run']
    assert 'npm run build:wasm:desktop' in build['run']
    assert 'wasm_cache.py verify --key "$ENGINE_KEY"' in by_id['engine-verified']['run']
    save = next(s for s in steps if s.get('uses') == 'actions/cache/save@v4')
    assert "steps.engine-record.outputs.valid == 'true'" in save['if']
    assert save['with'] == restore['with']
    assert steps.index(restore) < steps.index(by_id['engine-verified']) < steps.index(build) < steps.index(by_id['engine-record']) < steps.index(save)
    browser = steps[steps.index(save) + 1:]
    assert all("inputs.build-only != 'true'" in s['if'] for s in browser)
    warmer = workflow('shared-build-cache.yml')['jobs']['piping-desktop-wasm']
    assert "github.ref == 'refs/heads/main'" in warmer['if']
    # The key includes the runner image: the warmer must share every consumer's.
    consumers = {job['runs-on'] for job in workflow('piping-desktop-e2e.yml')['jobs'].values()
                 if any(step.get('uses') == './.github/actions/setup-piping-e2e' for step in job.get('steps', []))}
    assert consumers and consumers == {warmer['runs-on']}
    assert warmer['steps'][-1] == {'uses': './.github/actions/setup-piping-e2e', 'with': {'build-only': 'true'}}


REGRESSION_TARGETS = ("tools/software_workflow tools/deliverables tools/validation tests/test_ci_numerical.py "
                      "tests/test_ci_apt_sources.py tests/test_release_members.py tests/test_release_candidate_scan.py "
                      "tests/test_repository_history.py")


def test_repository_regressions_follow_their_own_plan_and_keep_every_target(tmp_path):
    import os
    import subprocess
    steps = workflow('governance-harness.yml')['jobs']['repository-checks']['steps']
    names = [s.get('name') for s in steps]
    plan_step = steps[names.index('Select repository regressions')]
    tests = steps[names.index('Bounded shared-tool and routing regressions')]
    assert names.index('Select repository regressions') < names.index('Bounded shared-tool and routing regressions')
    assert 'hosted_ci.py repository' in plan_step['run'] and '--output repository-ci-plan.json' in plan_step['run']
    assert 'if' not in plan_step and 'if' not in tests
    assert REGRESSION_TARGETS in tests['run']
    # The step passes exactly the plan's omissions to pytest, and none when
    # the plan lists none.
    stub = tmp_path / 'bin'
    stub.mkdir()
    (stub / 'pytest-args').write_text('')
    python = stub / 'python3'
    python.write_text('#!/bin/bash\nif [ "$1" = "-m" ]; then printf "%s\\n" "$@" > "$ARGS_OUT"; exit 0; fi\n'
                      f'exec {__import__("sys").executable} "$@"\n')
    python.chmod(0o755)
    for deselect in ([], ['a.py::T::one', 'b.py::T::two']):
        (tmp_path / 'repository-ci-plan.json').write_text(json.dumps({'deselect': deselect}))
        out = tmp_path / 'args.txt'
        subprocess.run(['bash', '-e', '-c', tests['run']], cwd=tmp_path, check=True,
                       env={**os.environ, 'PATH': f'{stub}:{os.environ["PATH"]}', 'ARGS_OUT': str(out)})
        args = out.read_text().split('\n')
        assert [a for a in args if a.startswith('--deselect')] == [f'--deselect={t}' for t in deselect]
        assert '-n' in args and 'auto' in args


def test_numerical_matrix_keeps_all_shards_and_gates_selected_coverage():
    import os
    import subprocess
    import importlib.util
    spec = importlib.util.spec_from_file_location('workflow_numerical', ROOT / 'tools/ci/numerical_ci.py')
    numerical = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(numerical)
    jobs = workflow('piping-desktop-e2e.yml')['jobs']
    job = jobs['numerical']
    assert job['strategy']['matrix']['shard'] == list(numerical.SHARDS)
    assert job['strategy']['fail-fast'] == 'false'
    assert job['if'] == "needs.selection.outputs.numerical_mode == 'full'"
    for step in job['steps']:
        run = step.get('run', '')
        if 'result_cache.py' in run or 'numerical_ci.py' in run:
            assert '--shard "${{ matrix.shard }}"' in run
        if 'verified-numerical' in str(step):
            assert 'verified-numerical-${{ matrix.shard }}' in str(step)
    evidence = next(s for s in job['steps'] if s.get('uses') == 'actions/upload-artifact@v4')
    assert evidence['with']['name'] == 'piping-numerical-evidence-${{ matrix.shard }}'
    assert 'always()' in evidence['if']
    restore = next(s for s in job['steps'] if s.get('uses') == './.github/actions/restore-build-cache')
    assert restore['with']['suite'] == 'piping-numerical'
    # The unsharded main warmer still compiles the entire plan's profiles.
    warm = workflow('shared-build-cache.yml')['jobs']['piping-numerical']
    build = next(s['run'] for s in warm['steps'] if 'numerical_ci.py' in s.get('run', ''))
    assert '--build-only' in build and '--shard' not in build
    aggregate = jobs['desktop-e2e']
    assert aggregate['if'] == 'always()' and 'numerical' in aggregate['needs']
    step = next(s for s in aggregate['steps'] if s.get('name') == 'Report selected checks without treating skipped work as passed')
    assert step['env']['NUMERICAL'] == '${{ needs.numerical.result }}'
    script = '\n'.join(line for line in step['run'].splitlines() if '--suite piping-numerical ' in line)
    assert script
    # GitHub reduces the matrix children into needs.numerical.result; only
    # success passes when selected. No shard-level condition can skip a member.
    assert not any('matrix.shard' in step.get('if', '') for step in job['steps'])
    for mode, state, passed in [('full', 'success', True), ('full', 'failure', False),
                                ('full', 'cancelled', False), ('full', 'skipped', False),
                                ('not-applicable', 'skipped', True), ('not-applicable', 'success', False)]:
        env = {**os.environ, 'NUMERICAL_MODE': mode, 'SELECTION': 'success', 'NUMERICAL': state}
        process = subprocess.run(['bash', '-e', '-c', script], cwd=ROOT, env=env, capture_output=True)
        assert (process.returncode == 0) == passed, (mode, state)


def test_python_matrix_requires_all_successful_shards(tmp_path):
    import os
    import subprocess
    jobs = workflow('piping-desktop-e2e.yml')['jobs']
    job = jobs['python-product']
    assert job['strategy']['matrix']['shard'] == ['1', '2', '3', '4']
    assert job['strategy']['fail-fast'] == 'false'
    assert all('continue-on-error' not in step and 'matrix.shard' not in step.get('if', '') for step in job['steps'])
    runner = next(s for s in job['steps'] if 'python_ci.py' in s.get('run', ''))
    assert '--shard-index "${{ matrix.shard }}" --shard-count 4' in runner['run']
    upload = next(s for s in job['steps'] if s.get('uses') == 'actions/upload-artifact@v4')
    assert upload['with']['name'] == 'python-shard-${{ matrix.shard }}'
    assert 'if' not in upload
    aggregate = jobs['desktop-e2e']
    report = next(s for s in aggregate['steps'] if 'aggregate --suite piping-python' in s.get('run', ''))
    script = next(line for line in report['run'].splitlines() if '--suite piping-python ' in line)
    for mode, state, passed in [('full', 'success', True), ('full', 'failure', False),
                                ('full', 'cancelled', False), ('full', 'skipped', False),
                                ('not-applicable', 'skipped', True), ('not-applicable', 'success', False)]:
        env = {**os.environ, 'PYTHON_MODE': mode, 'SELECTION': 'success', 'PYTHON_PRODUCT_RESULT': state}
        result = subprocess.run(['bash', '-e', '-c', script], cwd=ROOT, env=env, capture_output=True)
        assert (result.returncode == 0) == passed
    download = next(s for s in aggregate['steps'] if s.get('name') == 'Download successful Python shard markers')
    verify = next(s for s in aggregate['steps'] if s.get('name') == 'Require all four Python shards')
    assert download['if'] == verify['if'] == "needs.selection.outputs.python_mode == 'full'"
    assert download['with']['pattern'] == 'python-shard-*'
    check = verify['run'].replace('${{ runner.temp }}', str(tmp_path))
    for shard in range(1, 5):
        path = tmp_path / 'python-shards' / f'python-shard-{shard}' / 'success'
        path.parent.mkdir(parents=True)
        path.write_text(f'{shard}\n')
    def succeeds():
        return subprocess.run(['bash', '-c', check], capture_output=True).returncode == 0
    assert succeeds()
    for shard in range(1, 5):
        path = tmp_path / 'python-shards' / f'python-shard-{shard}' / 'success'
        path.unlink()
        assert not succeeds()
        path.write_text('wrong\n')
        assert not succeeds()
        path.write_text(f'{shard}\n')


def test_merge_group_missing_base_fails_closed():
    """A queue group can hold several PRs; a missing base must not fall back to HEAD^1."""
    text = (ROOT / ".github/workflows/governance-harness.yml").read_text()
    guard = 'if [[ "$EVENT" == "merge_group" ]]; then echo "merge_group base ${BASE} is unavailable" >&2; exit 1; fi'
    blocks = text.split('if ! git rev-parse --verify --quiet "${BASE}^{commit}" >/dev/null; then')[1:]
    assert len(blocks) == 2
    for block in blocks:
        assert block.index(guard) < block.index("BASE=HEAD^1")
    assert text.count("EVENT: ${{ github.event_name }}") >= 2
