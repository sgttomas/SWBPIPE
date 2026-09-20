#!/usr/bin/env python3
"""Conservative source-mode CI routing. No shell execution or external packages."""
import argparse
from collections import Counter
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess

PROJECT = 'projects/chirality-piping/'
DESKTOP = PROJECT + 'apps/desktop/'
E2E = DESKTOP + 'e2e/'
FAST = 'e2e/b3-accessibility.spec.ts'
PROJECTS = ['chromium-desktop', 'chromium-compact']
BASELINE = '002dff0f244976f98b36517d920b3761f6f88704'
FOCUSED = [FAST, 'e2e/workspace-layout.spec.ts', 'e2e/gui-workflow-validation.spec.ts']
FOCUSED_TITLES = [
    'keyboard splitters stay named and bounded; narrow drawers restore opener focus and hide inactive controls',
    'decorative viewport overlays pass real canvas gestures while view controls stay interactive',
    'workspace Escape event ownership consumed palette',
    'workspace Escape event ownership consumed drawer',
]
# Owner-authorized PR825 exception only; further product paths require ROOT review.
REPAIR_PATHS = {DESKTOP + 'src/styles.css', E2E + 'b3-accessibility.spec.ts',
                DESKTOP + 'src/features/workspace/shell/DisabledReason.tsx'}
STRATEGY_PATHS = {'.github/workflows/piping-desktop-e2e.yml',
                  '.github/actions/setup-piping-e2e/action.yml',
                  PROJECT + 'tests/test_ci_e2e_plan.py',
                  'docs/governance_harness/tranche_manifests/PIPING-CI-STRATEGY-20260920.yaml'}
RUN = PROJECT + 'execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/'
REPAIR_EVIDENCE = [RUN + p for p in (
    'instances/ROOT/CONTINUATION_2026-09-19_CODEX/_run_records/PR825_CI_FAILURE/',
    'instances/ROOT/CONTINUATION_2026-09-19_CODEX/_run_records/final-repo-checks/',
    'instances/ROOT/CONTINUATION_2026-09-19_CODEX/_run_records/final-sweep/',
    'instances/B3-CODEX/ci-tooltip-repair/', 'instances/CI-STRATEGY-CODEX/',
)]


def git(root, *args):
    return subprocess.check_output(['git', '-C', str(root), *args], stderr=subprocess.PIPE).decode('utf-8')


def changes(root, base, head):
    fields = git(root, 'diff', '--name-status', '--no-renames', '-z', base, head, '--').split('\0')
    return [{'status': fields[i], 'path': fields[i + 1]} for i in range(0, len(fields) - 1, 2)]


def inventory(root):
    desktop = Path(root) / DESKTOP
    return sorted(p.relative_to(desktop).as_posix() for p in (desktop / 'e2e').rglob('*')
                  if p.is_file() and re.search(r'\.(spec|test)\.[cm]?[jt]sx?$', p.name)
                  and not p.name.endswith('-dist.spec.ts'))


def record(path):
    # Only prose and record data in explicitly non-runtime directories qualify.
    prefixes = [PROJECT + x for x in ('execution/', 'docs/', 'plans/', 'governance/', 'provenance/', 'loop/')]
    return (any(path.startswith(x) for x in prefixes)
            and Path(path).suffix in {'.md', '.json', '.csv', '.txt', '.log', '.sha256', '.jsonl'})


def repair_record(path):
    reproduction_records = {RUN + 'instances/B3-CODEX/ci-tooltip-repair/' + suffix for suffix in (
        '_run_records/implementer/final-source.diff',
        '_run_records/implementer/geometry-only.diff',
        '_run_records/implementer/run-focused.sh', '_run_records/run-manager-check.sh',
    )}
    return (path in reproduction_records or record(path)
            or (any(path.startswith(x) for x in REPAIR_EVIDENCE)
                and Path(path).suffix in {'.gz', '.png', '.zip'})
            or (path.startswith(PROJECT + 'validation/evidence/sweeps/')
                and Path(path).suffix == '.json'))


def strategy(path):
    return path in STRATEGY_PATHS or path.startswith(PROJECT + 'tools/ci/')


def make_plan(root, event, base='', head='HEAD', pr=''):
    specs = inventory(root)
    if FAST not in specs:
        raise ValueError('Required accessibility barrier is missing')
    head = git(root, 'rev-parse', '--verify', head + '^{commit}').strip()
    plan = dict(version=1, mode='full', coverage_full=True, event=event, pr=str(pr),
                base=None, target_base=base, head=head, baseline=None, changed_paths=[], baseline_delta=[],
                projects=PROJECTS, inventory=specs, selected_specs=specs,
                focused_spec=None, focused_titles=[], reasons=[], coverage_note='Full source coverage requires barrier and all four shards to succeed.')
    if event != 'pull_request':
        plan['reasons'] = ['Manual or unknown event: full source suite']
        return plan
    try:
        merge_base = git(root, 'merge-base', base, head).strip()
        delta = changes(root, merge_base, head)
        plan.update(base=merge_base, changed_paths=delta)
    except (subprocess.CalledProcessError, UnicodeError, IndexError):
        plan['reasons'] = ['Unavailable PR merge-base/diff: full source suite']
        return plan
    if str(pr) == '825':
        try:
            git(root, 'merge-base', '--is-ancestor', BASELINE, head)
            repair = changes(root, BASELINE, head)
            plan.update(baseline=BASELINE, baseline_delta=repair)
            if repair and all(c['status'] in {'A', 'M'} and
                              (c['path'] in REPAIR_PATHS or strategy(c['path']) or repair_record(c['path'])) for c in repair):
                if not set(FOCUSED + ['e2e/ui-foundation.spec.ts']).issubset(specs):
                    raise ValueError('Required repair selection file is missing')
                plan.update(mode='pr825-repair', selected_specs=FOCUSED,
                            focused_spec='e2e/ui-foundation.spec.ts', focused_titles=FOCUSED_TITLES,
                            reasons=['Owner-authorized PR825 repair delta from prior tested head'])
        except (subprocess.CalledProcessError, UnicodeError, IndexError):
            plan['reasons'].append('PR825 baseline unavailable or not an ancestor')
    if plan['mode'] == 'full':
        active = [c for c in delta if not record(c['path'])]
        valid = active and all(c['status'] in {'A', 'M'} for c in active)
        if valid and all(c['path'].startswith(E2E) and c['path'][len(DESKTOP):] in specs and
                         c['path'].endswith('.spec.ts') for c in active):
            plan.update(mode='changed-specs', selected_specs=sorted({FAST} | {c['path'][len(DESKTOP):] for c in active}),
                        reasons=['Complete PR diff contains source specs and optional records only'])
        else:
            plan['reasons'].append('Broad, empty, deleted, renamed or unclassified input: full source suite')
    if plan['mode'] != 'full':
        plan.update(coverage_full=False, coverage_note='Partial coverage on this revision; not DEC093 full surface4 evidence.')
    return plan


def validate(root, plan):
    if not isinstance(plan, dict) or plan.get('version') != 1:
        raise ValueError('Invalid plan schema')
    # Bind downloaded metadata to the hosted event, not only to its own fields.
    if os.getenv('GITHUB_EVENT_NAME'):
        if plan.get('event') != os.environ['GITHUB_EVENT_NAME']:
            raise ValueError('Plan event differs from the hosted event')
        if plan['event'] == 'pull_request':
            event = json.loads(Path(os.environ['GITHUB_EVENT_PATH']).read_text())
            request = event['pull_request']
            if (plan.get('target_base'), plan.get('head'), plan.get('pr')) != (
                    request['base']['sha'], request['head']['sha'], str(event['number'])):
                raise ValueError('Plan candidate/base differs from the hosted PR event')
    # Recompute instead of trusting downloaded command/filter data.
    expected = make_plan(root, plan.get('event'), plan.get('target_base') or '', plan.get('head', ''), plan.get('pr', ''))
    if plan != expected:
        raise ValueError('Plan differs from checkout and conservative routing policy')
    if plan['event'] == 'pull_request':
        try:
            if not plan['target_base']:
                raise ValueError('Missing event target base')
            target = git(root, 'rev-parse', '--verify', plan['target_base'] + '^{commit}').strip()
            git(root, 'merge-base', '--is-ancestor', target, plan['head'])
        except (subprocess.CalledProcessError, ValueError):
            raise ValueError('Update the PR base: event target base is missing, unavailable or not integrated into head') from None
    if git(root, 'rev-parse', 'HEAD').strip() != plan['head']:
        raise ValueError('Plan head differs from checkout')


def commands(plan, stage, shard=None, list_only=False):
    common = ['../../node_modules/.bin/playwright', 'test', '--project=chromium-desktop',
              '--project=chromium-compact', '--workers=1', '--timeout=180000', '--reporter=list,html']
    if list_only:
        common[-1] = '--reporter=json'
        common += ['--list']
    def selected(specs, extra=()):
        if not specs:
            raise ValueError('Empty source selection is forbidden')
        return common + list(extra) + [re.escape(s) + '$' for s in specs]
    if stage == 'remainder':
        if plan['mode'] != 'full' or shard not in range(1, 5):
            raise ValueError('Remainder requires full mode and shard 1..4')
        return [selected([s for s in plan['inventory'] if s != FAST], [f'--shard={shard}/4'])]
    if stage != 'barrier':
        raise ValueError('Unknown execution stage')
    result = [selected([FAST], ['--max-failures=1'])]
    if plan['mode'] != 'full':
        rest = [s for s in plan['selected_specs'] if s != FAST]
        if rest:
            result.append(selected(rest, ['--max-failures=1']))
        if plan['focused_titles']:
            grep = '(?:' + '|'.join(re.escape(t) for t in plan['focused_titles']) + ')$'
            result.append(selected(['e2e/ui-foundation.spec.ts'], ['--max-failures=1', '--grep', grep]))
    return result


def collected_tests(report):
    """Normalize real Playwright list JSON without treating list status as a pass."""
    if not isinstance(report, dict) or report.get('errors'):
        raise ValueError('Playwright collection is invalid or contains errors')
    rows = []
    def visit(suite):
        for spec in suite.get('specs', []):
            for test in spec['tests']:
                row = dict(id=spec['id'], file='e2e/' + spec['file'], title=spec['title'],
                           project=test['projectName'], line=spec['line'])
                if not all(isinstance(row[k], str) and row[k] for k in ('id', 'file', 'title', 'project')):
                    raise ValueError('Malformed collected test identity')
                rows.append(row)
        for child in suite.get('suites', []):
            visit(child)
    visit(report)
    if not rows:
        raise ValueError('Empty Playwright collection')
    return rows


def test_key(row):
    return row['id'], row['project'], row['file'], row['title']


def validate_collections(plan, collections):
    """Prove requested case/file/profile coverage and exact full partition."""
    full = collections['source']
    if not full or set(t['project'] for t in full) != set(PROJECTS):
        raise ValueError('Source collection must contain both profiles')
    full_ids = Counter(test_key(t) for t in full)
    if any(n != 1 for n in full_ids.values()):
        raise ValueError('Duplicate source test identities')
    for file in plan['selected_specs']:
        for profile in PROJECTS:
            if not any(t['file'] == file and t['project'] == profile for t in full):
                raise ValueError(f'Empty selected file/profile: {file} / {profile}')
    for title in plan['focused_titles']:
        for profile in PROJECTS:
            found = [t for t in full if t['file'] == plan['focused_spec']
                     and t['title'] == title and t['project'] == profile]
            if len(found) != 1:
                raise ValueError(f'Focused title must occur exactly once: {title} / {profile}')
    if plan['mode'] == 'full':
        names = ['barrier-0'] + [f'shard-{n}' for n in range(1, 5)]
        expected = full
    else:
        names = [f'barrier-{n}' for n in range(len(commands(plan, 'barrier')))]
        expected = [t for t in full if t['file'] in plan['selected_specs'] or
                    (t['file'] == plan['focused_spec'] and t['title'] in plan['focused_titles'])]
    selected = []
    for name in names:
        if not collections.get(name):
            raise ValueError('Missing or empty required collection: ' + name)
        selected.extend(collections[name])
    if Counter(test_key(t) for t in selected) != Counter(test_key(t) for t in expected):
        raise ValueError('Collected partition has missing, duplicate or unexpected test identities')
    barrier_expected = Counter(test_key(t) for t in full if t['file'] == FAST)
    if Counter(test_key(t) for t in collections['barrier-0']) != barrier_expected:
        raise ValueError('Accessibility barrier collection differs from source inventory')
    selected_keys = {test_key(t) for t in selected}
    omitted = [dict(t, reason='Outside explicitly partial ' + plan['mode'] + ' selection')
               for t in full if test_key(t) not in selected_keys]
    return selected, omitted


def collect_candidate(root, plan, stage, shard, evidence_dir):
    """Validate cheap collection before any browser execution, on every runner."""
    evidence_dir = Path(evidence_dir)
    evidence_dir.mkdir(parents=True, exist_ok=True)
    evidence = dict(kind='collection-only', status='incomplete', head=plan.get('head'),
                    merge_base=plan.get('base'), target_base=plan.get('target_base'),
                    mode=plan.get('mode'), stage=stage, shard=shard, commands={},
                    selection_reasons=plan.get('reasons'),
                    plan_sha256=hashlib.sha256(json.dumps(plan, sort_keys=True).encode()).hexdigest(),
                    selected=[], omitted=[], execution_tests=[])
    try:
        validate(root, plan)
        evidence['execution_commands'] = commands(plan, stage, shard)
        config = Path(root) / DESKTOP / 'playwright.config.ts'
        registry = Path(root) / PROJECT / 'node_modules/playwright-core/browsers.json'
        evidence['identity'] = dict(config_sha256=hashlib.sha256(config.read_bytes()).hexdigest(),
            selector_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
            config_path=DESKTOP + 'playwright.config.ts', browser_registry=json.loads(registry.read_text()),
            configured_chromium_executable=os.getenv('PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH'),
            mac_chrome_fallback_present=Path('/Applications/Google Chrome.app/Contents/MacOS/Google Chrome').exists(),
            browser_launched=False, runtime_browser_version=None)
        # Unfiltered collection detects specs the filesystem discovery missed.
        source_command = commands(plan, 'barrier', list_only=True)[0]
        source_command = [arg for arg in source_command if arg != re.escape(FAST) + '$' and arg != '--max-failures=1']
        requests = {'source': source_command}
        for n, command in enumerate(commands(plan, 'barrier', list_only=True)):
            requests[f'barrier-{n}'] = command
        if plan['mode'] == 'full':
            for n in range(1, 5):
                requests[f'shard-{n}'] = commands(plan, 'remainder', n, list_only=True)[0]
        collections = {}
        for name, command in requests.items():
            result = subprocess.run(command, cwd=Path(root) / DESKTOP, capture_output=True, text=True, timeout=180)
            evidence['commands'][name] = dict(argv=command, returncode=result.returncode)
            (evidence_dir / (name + '.stdout.json')).write_text(result.stdout)
            (evidence_dir / (name + '.stderr.txt')).write_text(result.stderr)
            if result.returncode:
                raise ValueError(f'Collection {name} failed with exit {result.returncode}')
            report = json.loads(result.stdout)
            evidence['commands'][name]['config'] = report.get('config')
            collections[name] = collected_tests(report)
        selected, omitted = validate_collections(plan, collections)
        evidence['identity']['source_file_sha256'] = {file: hashlib.sha256(
            (Path(root) / DESKTOP / file).read_bytes()).hexdigest()
            for file in sorted({t['file'] for t in collections['source']})}
        selected = [dict(t, reason=('Full source coverage' if plan['mode'] == 'full'
                    else 'Selected complete source file' if t['file'] in plan['selected_specs']
                    else 'Required exact focused title/profile')) for t in selected]
        execution_names = ([f'shard-{shard}'] if stage == 'remainder'
                           else [name for name in requests if name.startswith('barrier-')])
        # Validate stage parameters even when collection is invoked without execution.
        commands(plan, stage, shard)
        evidence.update(status='validated', selected=selected, omitted=omitted,
                        partition={name: rows for name, rows in collections.items() if name != 'source'},
                        execution_tests=[row for name in execution_names for row in collections[name]])
    except Exception as exc:
        evidence.update(status='failed', error=f'{type(exc).__name__}: {exc}')
        raise
    finally:
        (evidence_dir / 'collection.json').write_text(json.dumps(evidence, indent=2) + '\n')
        summary = (f"## Source collection: {stage} {shard or ''}\n\n"
                   f"Status: {evidence['status']}; head: `{evidence['head']}`; "
                   f"target base: `{evidence['target_base']}`; merge base: `{evidence['merge_base']}`.\n\n"
                   f"Selected {len(evidence['selected'])}; omitted {len(evidence['omitted'])}; "
                   f"this stage {len(evidence['execution_tests'])}. Collection is not a test pass.\n")
        if 'error' in evidence:
            summary += '\n' + evidence['error'] + '\n'
        (evidence_dir / 'summary.md').write_text(summary)
        if os.getenv('GITHUB_STEP_SUMMARY'):
            with open(os.environ['GITHUB_STEP_SUMMARY'], 'a') as out:
                out.write(summary)
    return evidence


def aggregate(mode, selection, barrier, remainder):
    return (mode in {'full', 'changed-specs', 'pr825-repair'}
            and selection == barrier == 'success'
            and remainder == ('success' if mode == 'full' else 'skipped'))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest='action', required=True)
    build = sub.add_parser('plan')
    build.add_argument('--event', required=True)
    build.add_argument('--base', default='')
    build.add_argument('--head', default='HEAD')
    build.add_argument('--pr', default='')
    build.add_argument('--output', required=True)
    run = sub.add_parser('run')
    run.add_argument('--plan', required=True)
    run.add_argument('--stage', choices=['barrier', 'remainder'], required=True)
    run.add_argument('--shard', type=int)
    run.add_argument('--list', action='store_true', help='Validate collection only; do not execute tests')
    run.add_argument('--evidence-dir', required=True)
    gate = sub.add_parser('aggregate')
    for key in ('mode', 'selection', 'barrier', 'remainder'):
        gate.add_argument('--' + key, required=True)
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[4]
    if args.action == 'aggregate':
        ok = aggregate(args.mode, args.selection, args.barrier, args.remainder)
        print('Required source CI jobs succeeded' if ok else 'Required source CI job failed, cancelled, absent or invalid')
        raise SystemExit(0 if ok else 1)
    if args.action == 'plan':
        plan = make_plan(root, args.event, args.base, args.head, args.pr)
        Path(args.output).write_text(json.dumps(plan, indent=2) + '\n')
        if os.getenv('GITHUB_OUTPUT'):
            with open(os.environ['GITHUB_OUTPUT'], 'a') as out:
                out.write('mode=' + plan['mode'] + '\n')
        summary = '## Piping source CI selection\n\n' + plan['coverage_note'] + '\n\n```json\n' + json.dumps(plan, indent=2) + '\n```\n'
        if os.getenv('GITHUB_STEP_SUMMARY'):
            with open(os.environ['GITHUB_STEP_SUMMARY'], 'a') as out:
                out.write(summary)
        print(summary)
        validate(root, plan)
    else:
        plan = json.loads(Path(args.plan).read_text())
        collect_candidate(root, plan, args.stage, args.shard, args.evidence_dir)
        if args.list:
            return
        for command in commands(plan, args.stage, args.shard):
            print(json.dumps(command), flush=True)
            subprocess.run(command, cwd=root / DESKTOP, check=True)


if __name__ == '__main__':
    main()
