#!/usr/bin/env python3
"""Conservative source-mode CI routing. No shell execution or external packages."""
import argparse
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
    return (record(path)
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
                base=None, head=head, baseline=None, changed_paths=[], baseline_delta=[],
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
        if valid and all(c['path'].startswith(E2E + 'ui-foundation/') for c in active):
            plan.update(mode='instruments', selected_specs=sorted({FAST} | {s for s in specs if s.startswith('e2e/ui-foundation/')}),
                        reasons=['Complete PR diff contains instrument inputs and optional records only; no benchmarks'])
        elif valid and all(c['path'].startswith(E2E) and c['path'][len(DESKTOP):] in specs and
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
    # Recompute instead of trusting downloaded command/filter data.
    expected = make_plan(root, plan.get('event'), plan.get('base') or '', plan.get('head', ''), plan.get('pr', ''))
    if plan != expected:
        raise ValueError('Plan differs from checkout and conservative routing policy')
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


def aggregate(mode, selection, barrier, remainder):
    return (mode in {'full', 'changed-specs', 'instruments', 'pr825-repair'}
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
    run.add_argument('--list', action='store_true')
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
    else:
        plan = json.loads(Path(args.plan).read_text())
        validate(root, plan)
        for command in commands(plan, args.stage, args.shard, args.list):
            print(json.dumps(command), flush=True)
            subprocess.run(command, cwd=root / DESKTOP, check=True)


if __name__ == '__main__':
    main()
