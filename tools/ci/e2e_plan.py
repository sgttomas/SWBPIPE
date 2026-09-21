#!/usr/bin/env python3
"""Candidate-bound lean/affected/full source CI; all omissions are explicit."""
import argparse
from collections import Counter
import hashlib
from functools import lru_cache
import math
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
LEAN_TITLES = {
    'e2e/b3a-session-status.spec.ts': ['canonical edit/save/Undo/Redo/reopen marker follows the persisted snapshot'],
    'e2e/workspace-layout.spec.ts': ['workspace preserves the visible model during discovery, routing, property editing and analysis navigation'],
    'e2e/linear-authoring.spec.ts': ['compact blank-to-straight authoring keeps the canvas and exact Add/Apply review'],
    'e2e/r2-smoke.spec.ts': ['R2 desktop preview smoke covers solve, results, report, and viewport overlay'],
    'e2e/result-compatibility.spec.ts': ['fresh Current exports strict 0.2 result and stress-neutral evidence'],
    'e2e/gui-workflow-validation.spec.ts': [
        'shared drawer menu overlap preserves ordinary Close and active menu priority: issues-home',
        'shared drawer menu overlap preserves ordinary Close and active menu priority: audit-boundary-drawer'],
    'e2e/ui-foundation.spec.ts': ['[preflight] maintained fixture supports typed selection and a keyboard-authored measurement'],
}
LAYOUT_TITLES = [
    'keyboard splitters stay named and bounded; narrow drawers restore opener focus and hide inactive controls',
    'decorative viewport overlays pass real canvas gestures while view controls stay interactive',
    'workspace Escape event ownership body idle', 'workspace Escape event ownership body captured',
    'workspace Escape event ownership consumed palette', 'workspace Escape event ownership consumed drawer',
]
# Only these reviewed module inputs use an actual import-consumer traversal.
# Fixtures, config, unknown helpers and dynamic resource inputs remain full.
INSTRUMENT_MODULES = {
    'benchmark-harness.ts', 'fresh-demo-policy.mjs', 'performance-targets.ts',
    'full-cohort-controller.ts', 'causal-method-contract.ts',
    'causal-presentation-extractor.mjs', 'chromium-compositor-trace.ts', 'causal-phase-journal.ts',
}


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


def irrelevant(path):
    """Known non-inputs only; unknown root/shared inputs still run full."""
    if path.startswith('projects/') and not path.startswith(PROJECT):
        return True  # Runtime is not adopted as a Piping product dependency.
    if path.startswith(PROJECT):
        return any(path.startswith(PROJECT + p) for p in
                   ('execution/', 'docs/', 'plans/', 'governance/', 'provenance/', 'loop/', 'validation/evidence/'))
    return path.startswith(('docs/', 'agents/')) or path in {'README.md', 'AGENTS.md', 'CLAUDE.md'}


def instrument_consumers(root, path, specs):
    """Transitive literal import ownership, only for reviewed module entrypoints."""
    if not path.startswith(E2E + 'ui-foundation/') or path[len(E2E + 'ui-foundation/'):] not in INSTRUMENT_MODULES:
        return None
    directory = Path(root) / DESKTOP
    graph = {}
    # Include test helpers and benchmarks as intermediate importers, never execute benchmarks.
    for file in (directory / 'e2e').rglob('*'):
        if not file.is_file() or file.suffix not in {'.ts', '.tsx', '.mjs', '.js'}:
            continue
        imports = re.findall(r'''(?:from\s*|import\s*\(\s*|require\s*\(\s*|import\s*)["'](\.[^"']+)["']''', file.read_text())
        for name in imports:
            candidate = file.parent / name
            choices = [candidate] + [Path(str(candidate) + ext) for ext in ('.ts', '.tsx', '.mjs', '.js', '/index.ts')]
            resolved = next((c.resolve() for c in choices if c.is_file()), None)
            if resolved:
                graph.setdefault(resolved, set()).add(file.resolve())
    seen, pending = set(), [(Path(root) / path).resolve()]
    while pending:
        file = pending.pop()
        if file in seen:
            continue
        seen.add(file)
        pending.extend(graph.get(file, ()))
    found = sorted(s for s in specs if (directory / s).resolve() in seen)
    return found or None


def owned_area(path):
    if path == DESKTOP + 'src/features/workspace/shell/DisabledReason.tsx':
        return 'lean'
    if path == DESKTOP + 'src/styles.css' or path.startswith(DESKTOP + 'src/features/workspace/shell/') or path in {
        DESKTOP + 'src/features/workspace/shellLayout.ts', DESKTOP + 'src/features/workspace/uiPreferences.ts',
        DESKTOP + 'src/features/workspace/workspaceCanvasBudget.ts'}:
        return 'layout'
    if any(path.startswith(DESKTOP + 'src/features/' + p) for p in
           ('viewport/', 'model-tree/', 'geometry-tools/', 'rich-authoring/')) or path == DESKTOP + 'src/features/workspace/selectionState.ts':
        return 'authoring'
    if any(path.startswith(DESKTOP + 'src/features/' + p) for p in
           ('results/', 'report/', 'stress-neutral/', 'project-storage/', 'redaction-controls/')) or path == DESKTOP + 'src/features/workspace/projectPersistenceIntegrity.ts':
        return 'results'
    return None


def make_plan(root, event, base='', head='HEAD', pr=''):
    specs = inventory(root)
    head = git(root, 'rev-parse', '--verify', head + '^{commit}').strip()
    plan = dict(version=3, mode='full', coverage_full=True, event=event, pr=str(pr),
                base=None, target_base=base, head=head, changed_paths=[],
                projects=PROJECTS, inventory=specs, selected_specs=specs, selected_titles={},
                appearance=False, ownership={}, reasons=['Full source coverage'])
    if event == 'pull_request':
        try:
            merge_base = git(root, 'merge-base', base, head).strip()
            delta = changes(root, merge_base, head)
            plan.update(base=merge_base, changed_paths=delta)
        except (subprocess.CalledProcessError, UnicodeError, IndexError):
            plan['reasons'] = ['Unavailable PR diff; full coverage, target validation still required']
        else:
            active = [c for c in delta if not irrelevant(c['path'])]
            if not active:
                plan.update(mode='not-applicable', selected_specs=[], reasons=['No relevant Piping source/build/CI inputs changed'])
            elif all(c['status'] in {'A', 'M'} and c['path'].startswith(E2E) and
                     c['path'][len(DESKTOP):] in specs and c['path'].endswith('.spec.ts') for c in active):
                plan.update(mode='changed-specs', selected_specs=sorted({FAST} | {c['path'][len(DESKTOP):] for c in active}),
                            reasons=['Complete PR diff changes source specs only'])
            elif all(c['status'] in {'A', 'M'} for c in active):
                areas, consumers = set(), set()
                for change in active:
                    path = change['path']
                    area = owned_area(path)
                    found = instrument_consumers(root, path, specs) if area is None else None
                    if area is None and found is None:
                        break
                    areas.add(area or 'instruments')
                    consumers.update(found or [])
                    plan['ownership'][path] = found or [area]
                else:
                    selected = {FAST} | consumers
                    titles = {f: list(ts) for f, ts in LEAN_TITLES.items()}
                    if 'layout' in areas:
                        selected.update(['e2e/workspace-layout.spec.ts', 'e2e/c3-viewport-visibility.spec.ts',
                                         'e2e/b4-table-editing.spec.ts'])
                        titles['e2e/ui-foundation.spec.ts'] += LAYOUT_TITLES
                    if 'authoring' in areas:
                        selected.update(['e2e/ui-foundation.spec.ts', 'e2e/linear-authoring.spec.ts',
                                         'e2e/c3-viewport-visibility.spec.ts', 'e2e/b4-table-editing.spec.ts'])
                    if 'results' in areas:
                        selected.update(['e2e/result-compatibility.spec.ts', 'e2e/gui-workflow-validation.spec.ts',
                                         'e2e/b3a-session-status.spec.ts', 'e2e/r2-smoke.spec.ts',
                                         'e2e/b3b-project-persistence.spec.ts'])
                    plan.update(mode='lean' if areas == {'lean'} else 'lean-affected', selected_specs=sorted(selected),
                                selected_titles=titles, appearance='layout' in areas,
                                reasons=['Lean ordinary journeys plus reviewed affected ownership: ' + ', '.join(sorted(areas))])
    else:
        plan['reasons'] = ['Deliberate manual full integration milestone or unknown event']
        if base:
            try:
                plan['base'] = git(root, 'merge-base', base, head).strip()
            except subprocess.CalledProcessError:
                pass  # Validation below blocks unresolved or unintegrated targets.
    plan['coverage_full'] = plan['mode'] == 'full'
    plan['coverage_note'] = ('Full deduplicated source coverage requires barrier and four exact partitions to succeed.'
        if plan['coverage_full'] else 'Not applicable: no tests selected or claimed passed.' if plan['mode'] == 'not-applicable'
        else 'Partial source coverage; omitted tests are not passed and this is not DEC093 full surface4 evidence.')
    if plan['mode'] != 'not-applicable' and FAST not in specs:
        raise ValueError('Required accessibility barrier is missing')
    return plan

def validate(root, plan):
    if not isinstance(plan, dict) or plan.get('version') != 3:
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
        elif plan['event'] == 'workflow_dispatch':
            event = json.loads(Path(os.environ['GITHUB_EVENT_PATH']).read_text())
            target = event.get('inputs', {}).get('target_base') or ''
            if plan.get('target_base') != target or plan.get('head') != os.environ['GITHUB_SHA']:
                raise ValueError('Plan candidate/target differs from the manual full event')
    # Recompute instead of trusting downloaded command/filter data.
    expected = make_plan(root, plan.get('event'), plan.get('target_base') or '', plan.get('head', ''), plan.get('pr', ''))
    if plan != expected:
        raise ValueError('Plan differs from checkout and conservative routing policy')
    if plan['event'] == 'pull_request' or plan['target_base']:
        try:
            if not plan['target_base']:
                raise ValueError('Missing event target base')
            if plan['event'] == 'workflow_dispatch' and not re.fullmatch(r'[0-9a-fA-F]{40}', plan['target_base']):
                raise ValueError('Manual target must be an immutable commit SHA')
            target = git(root, 'rev-parse', '--verify', plan['target_base'] + '^{commit}').strip()
            git(root, 'merge-base', '--is-ancestor', target, plan['head'])
        except (subprocess.CalledProcessError, ValueError):
            raise ValueError('Update the PR base: event target base is missing, unavailable or not integrated into head') from None
    if git(root, 'rev-parse', 'HEAD').strip() != plan['head']:
        raise ValueError('Plan head differs from checkout')


def collected_tests(report):
    if not isinstance(report, dict) or report.get('errors'):
        raise ValueError('Invalid Playwright collection')
    rows = []
    def visit(suite, titles):
        for spec in suite.get('specs', []):
            for test in spec['tests']:
                rows.append(dict(id=spec['id'], file='e2e/' + spec['file'], title=spec['title'],
                                 title_path=titles + [spec['title']], tags=['@' + tag.lstrip('@') for tag in spec.get('tags', [])],
                                 project=test['projectName'], line=spec['line']))
        for child in suite.get('suites', []):
            visit(child, titles + [child['title']] if child.get('line', 0) else titles)
    visit(report, [])
    if not rows:
        raise ValueError('Empty Playwright collection')
    return rows


def test_key(row):
    return row['id'], row['project'], row['file'], tuple(row['title_path'])


def validate_source(source):
    if set(t['project'] for t in source) != set(PROJECTS):
        raise ValueError('Both source projects are required')
    if any(n != 1 for n in Counter(test_key(t) for t in source).values()):
        raise ValueError('Duplicate source identity')
    if any(t['project'] == 'chromium-compact' and '@explicit-viewport' in t['tags'] for t in source):
        raise ValueError('Explicit-viewport duplicate reintroduced by a filter override')
    # The twelve matrix combinations are intentional distinct scenarios.
    appearance = [t for t in source if t['file'] == 'e2e/ui-foundation.spec.ts'
                  and t['title'].startswith('task and analysis dock preserve usable canvas ')]
    expected = {f'task and analysis dock preserve usable canvas {theme} {density} {width}x{height}'
                for theme in ('light', 'dark') for density in ('comfortable', 'compact')
                for width, height in ((1024, 768), (1280, 800), (1440, 920))}
    if {t['title'] for t in appearance} != expected or len(appearance) != 12:
        raise ValueError('Appearance matrix must retain all twelve distinct setups exactly once')
    for file in {t['file'] for t in source}:
        for profile in PROJECTS:
            if not any(t['file'] == file and t['project'] == profile for t in source):
                raise ValueError('Missing file/profile in full source collection: ' + file + ' / ' + profile)


def select_tests(plan, source):
    validate_source(source)
    if plan['mode'] == 'not-applicable':
        raise ValueError('N/A must not run browser collection')
    for file in plan['selected_specs']:
        if not any(t['file'] == file for t in source):
            raise ValueError('Empty selected file: ' + file)
    for file, titles in plan['selected_titles'].items():
        for title in titles:
            for profile in PROJECTS:
                if sum(t['file'] == file and t['title'] == title and t['project'] == profile for t in source) != 1:
                    raise ValueError('Required title/profile missing or duplicated: ' + title + ' / ' + profile)
    selected = [t for t in source if plan['mode'] == 'full' or t['file'] in plan['selected_specs']
                or t['title'] in plan['selected_titles'].get(t['file'], [])
                or (plan['appearance'] and t['file'] == 'e2e/ui-foundation.spec.ts' and '@explicit-viewport' in t['tags'])]
    if not selected or not any(t['file'] == FAST for t in selected):
        raise ValueError('Missing accessibility barrier')
    return selected


# Scheduling hints only; never acceptance limits. Filled from retained observed
# source job logs; new/unobserved identities get a conservative 30 second weight.
DURATION_HINTS = Path(__file__).with_name('e2e_duration_hints.json')


@lru_cache(maxsize=1)
def duration_hints():
    hints = json.loads(DURATION_HINTS.read_text())
    if hints.get('version') != 1 or any(not isinstance(n, (float, int)) or not math.isfinite(n) or n <= 0
                                      for n in [hints['unknown_seconds'], *hints['seconds'].values()]):
        raise ValueError('Invalid scheduling duration hints')
    return hints


def duration_weight(row):
    hints = duration_hints()
    key = ' › '.join([row['project'], row['file'], *row['title_path']])
    return hints['seconds'].get(key, hints['unknown_seconds'])


def assign_partitions(plan, source, root):
    selected = select_tests(plan, source)
    barrier = [t for t in selected if t['file'] == FAST]
    rest = [t for t in selected if t['file'] != FAST]
    if plan['mode'] != 'full':
        return {'barrier': barrier, **({'selected': rest} if rest else {})}
    groups = {}
    bodies = {file: (Path(root) / DESKTOP / file).read_text() for file in {t['file'] for t in rest}}
    for row in rest:
        # Keep named describe groups together. Conservatively preserve any file
        # with a serial declaration rather than splitting its serial state.
        body = bodies[row['file']]
        serial = re.search(r'describe\.serial|mode\s*:\s*["\']serial["\']', body)
        independent = row['file'] in {'e2e/ui-foundation.spec.ts', 'e2e/workspace-layout.spec.ts'}
        key = (row['file'], row['project'], () if serial or not independent else tuple(row['title_path'][:-1]) or (row['title'],))
        groups.setdefault(key, []).append(row)
    for rows in groups.values():
        rows.sort(key=test_key)
    bins = [[] for _ in range(4)]
    loads = [0.0] * 4
    ordered = sorted(groups.items(), key=lambda pair: (-sum(duration_weight(t) for t in pair[1]), pair[0]))
    for _, rows in ordered:
        target = min(range(4), key=lambda i: (loads[i], i))
        bins[target].extend(rows)
        loads[target] += sum(duration_weight(t) for t in rows)
    if any(not rows for rows in bins):
        raise ValueError('Full selection needs four nonempty partitions')
    return {'barrier': barrier, **{f'shard-{i+1}': rows for i, rows in enumerate(bins)}}


def exact_list(rows):
    if not rows:
        raise ValueError('Empty exact test list')
    lines = []
    for row in rows:
        tokens = [f"[{row['project']}]", row['file'].removeprefix('e2e/'), *row['title_path']]
        if any(any(char in token for char in ('\n', '\r', '›')) or token.strip() != token for token in tokens):
            raise ValueError('Test identity cannot be represented exactly in Playwright --test-list')
        lines.append(' › '.join(tokens))
    return '\n'.join(lines) + '\n'


def command(test_list=None, list_only=False, fast=False):
    args = ['../../node_modules/.bin/playwright', 'test', '--project=chromium-desktop',
            '--project=chromium-compact', '--workers=1', '--timeout=180000',
            '--reporter=json' if list_only else '--reporter=list,html']
    if list_only:
        args.append('--list')
    if test_list is not None:
        args.extend(['--test-list', str(Path(test_list).resolve())])
    if fast:
        args.append('--max-failures=1')
    return args


def assert_partition(expected, actual):
    if not actual or Counter(test_key(t) for t in expected) != Counter(test_key(t) for t in actual):
        raise ValueError('Collected exact list has missing, duplicate or unexpected identities')


def collect_candidate(root, plan, stage, shard, evidence_dir):
    directory = Path(evidence_dir).resolve()
    directory.mkdir(parents=True, exist_ok=True)
    evidence = dict(kind='collection-only', status='incomplete', head=plan.get('head'),
                    merge_base=plan.get('base'), target_base=plan.get('target_base'), mode=plan.get('mode'),
                    stage=stage, shard=shard, selected=[], omitted=[], execution_tests=[], commands={},
                    selection_reasons=plan.get('reasons'),
                    plan_sha256=hashlib.sha256(json.dumps(plan, sort_keys=True).encode()).hexdigest())
    try:
        validate(root, plan)
        if plan['mode'] == 'not-applicable':
            raise ValueError('N/A mode must not provision or execute Piping browser jobs')
        registry = Path(root) / PROJECT / 'node_modules/playwright-core/browsers.json'
        evidence['identity'] = dict(config_sha256=hashlib.sha256((Path(root) / DESKTOP / 'playwright.config.ts').read_bytes()).hexdigest(),
            selector_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
            duration_hints_sha256=hashlib.sha256(DURATION_HINTS.read_bytes()).hexdigest(), browser_registry=json.loads(registry.read_text()),
            configured_chromium_executable=os.getenv('PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH'),
            mac_chrome_fallback_present=Path('/Applications/Google Chrome.app/Contents/MacOS/Google Chrome').exists(),
            browser_launched=False, runtime_browser_version=None)
        def collect(name, args):
            result = subprocess.run(args, cwd=Path(root) / DESKTOP, capture_output=True, text=True, timeout=180)
            evidence['commands'][name] = dict(argv=args, returncode=result.returncode)
            (directory / (name + '.stdout.json')).write_text(result.stdout)
            (directory / (name + '.stderr.txt')).write_text(result.stderr)
            if result.returncode:
                raise ValueError(f'Collection {name} failed: {result.returncode}')
            report = json.loads(result.stdout)
            evidence['commands'][name]['config'] = report.get('config')
            return collected_tests(report)
        source = collect('source', command(list_only=True))
        selected = select_tests(plan, source)
        partitions = assign_partitions(plan, source, root)
        actual = {}
        for name, rows in partitions.items():
            file = directory / (name + '.test-list.txt')
            file.write_text(exact_list(rows))
            actual[name] = collect(name, command(file, list_only=True))
            assert_partition(rows, actual[name])
        assert_partition(selected, [t for rows in actual.values() for t in rows])
        names = ([f'shard-{shard}'] if stage == 'remainder' and plan['mode'] == 'full' and shard in range(1, 5)
                 else ['barrier'] + (['selected'] if 'selected' in partitions else []) if stage == 'barrier' else [])
        if not names or any(name not in actual for name in names):
            raise ValueError('Invalid execution stage or shard')
        keys = {test_key(t) for t in selected}
        evidence.update(status='validated', selected=[dict(t, reason='Selected by ' + plan['mode']) for t in selected],
            omitted=[dict(t, reason='Outside partial ' + plan['mode'] + ' coverage') for t in source if test_key(t) not in keys],
            partition=actual, estimated_seconds={name: round(sum(duration_weight(t) for t in rows), 2) for name, rows in partitions.items()},
            execution_tests=[t for name in names for t in actual[name]],
            execution_commands=[command(directory / (name + '.test-list.txt'), fast=name == 'barrier') for name in names])
        evidence['identity']['source_file_sha256'] = {file: hashlib.sha256((Path(root) / DESKTOP / file).read_bytes()).hexdigest()
            for file in sorted({t['file'] for t in source})}
    except Exception as exc:
        evidence.update(status='failed', error=f'{type(exc).__name__}: {exc}')
        raise
    finally:
        (directory / 'collection.json').write_text(json.dumps(evidence, indent=2) + '\n')
        summary = (f"## Piping collection: {stage} {shard or ''}\n\nStatus: {evidence['status']}; "
                   f"head `{evidence['head']}`, target `{evidence['target_base']}`.\n\n"
                   f"Selected {len(evidence['selected'])}; omitted {len(evidence['omitted'])}. Collection is not a test pass.\n")
        if 'error' in evidence:
            summary += '\n' + evidence['error'] + '\n'
        (directory / 'summary.md').write_text(summary)
        if os.getenv('GITHUB_STEP_SUMMARY'):
            with open(os.environ['GITHUB_STEP_SUMMARY'], 'a') as out:
                out.write(summary)
    return evidence


def aggregate(mode, selection, barrier, remainder):
    if selection != 'success':
        return False
    if mode == 'not-applicable':
        return barrier == remainder == 'skipped'
    return mode in {'full', 'changed-specs', 'lean', 'lean-affected'} and barrier == 'success' and remainder == ('success' if mode == 'full' else 'skipped')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest='action', required=True)
    build = sub.add_parser('plan')
    for name, default in [('event', None), ('base', ''), ('head', 'HEAD'), ('pr', '')]:
        build.add_argument('--' + name, default=default, required=name == 'event')
    build.add_argument('--output', required=True)
    run = sub.add_parser('run')
    run.add_argument('--plan', required=True)
    run.add_argument('--stage', choices=['barrier', 'remainder'], required=True)
    run.add_argument('--shard', type=int)
    run.add_argument('--list', action='store_true')
    run.add_argument('--evidence-dir', required=True)
    gate = sub.add_parser('aggregate')
    for key in ('mode', 'selection', 'barrier', 'remainder'):
        gate.add_argument('--' + key, required=True)
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[4]
    if args.action == 'aggregate':
        ok = aggregate(args.mode, args.selection, args.barrier, args.remainder)
        print('Validated not-applicable: no Piping tests selected or passed' if ok and args.mode == 'not-applicable'
              else 'Required selected coverage succeeded' if ok else 'Required CI failed, cancelled, missing or invalid')
        raise SystemExit(0 if ok else 1)
    if args.action == 'plan':
        plan = make_plan(root, args.event, args.base, args.head, args.pr)
        Path(args.output).write_text(json.dumps(plan, indent=2) + '\n')
        summary = '## Piping source selection\n\n' + plan['coverage_note'] + '\n\n```json\n' + json.dumps(plan, indent=2) + '\n```\n'
        if os.getenv('GITHUB_STEP_SUMMARY'):
            with open(os.environ['GITHUB_STEP_SUMMARY'], 'a') as out:
                out.write(summary)
        print(summary)
        validate(root, plan)
        if os.getenv('GITHUB_OUTPUT'):
            with open(os.environ['GITHUB_OUTPUT'], 'a') as out:
                out.write('mode=' + plan['mode'] + '\n')
    else:
        plan = json.loads(Path(args.plan).read_text())
        evidence = collect_candidate(root, plan, args.stage, args.shard, args.evidence_dir)
        if not args.list:
            for args in evidence['execution_commands']:
                subprocess.run(args, cwd=root / DESKTOP, check=True)


if __name__ == '__main__':
    main()
