#!/usr/bin/env python3
"""Refresh e2e_duration_hints.json from one successful full Piping Desktop E2E run.

Scheduling data only: launches no browser or test, and never changes coverage,
selection, timeouts or acceptance. Reads the run's retained collection artifact
and its barrier/shard job logs through the authenticated `gh` CLI, maps each
Playwright list-reporter line to its canonical collected identity (including
separately printed tags), and updates only the observed identities. Unobserved
identities keep their prior hints; new ones keep the conservative fallback.

    python3 projects/chirality-piping/tools/ci/refresh_duration_hints.py --run <id> [--apply]
"""
import argparse
import copy
import hashlib
import json
import math
from pathlib import Path
import re
import subprocess
import tempfile

import e2e_plan

REPO = 'sgttomas/chirality'
PASSED = re.compile(r'✓\s+\d+\s+\[([^\]]+)\]\s+›\s+(e2e/[^:]+):\d+:\d+\s+›\s+(.+?)\s+\(([\d.]+)(ms|s|m)\)\s*$')
SKIPPED = re.compile(r'-\s+\d+\s+\[([^\]]+)\]\s+›\s+(e2e/[^:]+):\d+:\d+\s+›\s+(.+?)\s*$')
SKIP_WEIGHT, ZERO_WEIGHT = .05, .001


def gh(*args):
    return subprocess.check_output(['gh', *args])


def key(row):
    return ' › '.join([row['project'], row['file'], *row['title_path']])


def parse_logs(logs, source):
    reporter = {}
    for row in source:
        # Tags declared in details are printed after the title; tags written
        # inline in the title are printed once, as part of it.
        for label in {key(row), key(row) + (' ' + ' '.join(row['tags']) if row['tags'] else '')}:
            if reporter.get(label, key(row)) != key(row):
                raise ValueError('Ambiguous reporter label: ' + label)
            reporter[label] = key(row)
    durations, skips = {}, set()
    for line in logs.splitlines():
        match = PASSED.search(line)
        if match:
            project, file, title, value, unit = match.groups()
            canonical = reporter[' › '.join([project, file, title])]
            if canonical in durations:
                raise ValueError('Duplicate duration: ' + canonical)
            durations[canonical] = float(value) * {'ms': .001, 's': 1, 'm': 60}[unit]
        elif (match := SKIPPED.search(line)):
            skips.add(reporter[' › '.join(match.groups())])
    return durations, skips


def refresh(old, durations, skips, basis):
    new = copy.deepcopy(old)
    new['seconds'].update({k: v if v > 0 else old['seconds'].get(k, ZERO_WEIGHT) for k, v in durations.items()})
    new['seconds'].update({k: SKIP_WEIGHT for k in skips})
    new['seconds'] = dict(sorted(new['seconds'].items()))
    new['basis'] = {**basis, 'prior_duration_basis': old['basis']}
    if any(type(v) not in (int, float) or not math.isfinite(v) or v <= 0
           for v in [new['unknown_seconds'], *new['seconds'].values()]):
        raise ValueError('Non-positive scheduling weight')
    return new


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument('--run', required=True, type=int)
    parser.add_argument('--apply', action='store_true')
    args = parser.parse_args()
    run = json.loads(gh('run', 'view', str(args.run), '-R', REPO, '--json', 'headSha,conclusion,jobs,attempt'))
    if run['conclusion'] != 'success':
        raise SystemExit('Run did not succeed; failed or cancelled runs carry no reliable durations')
    with tempfile.TemporaryDirectory() as tmp:
        gh('run', 'download', str(args.run), '-R', REPO, '-n', 'piping-e2e-selection', '-D', tmp + '/plan')
        plan = json.loads(Path(tmp, 'plan/piping-e2e-plan.json').read_text())
        if plan['mode'] != 'full' or plan['head'] != run['headSha']:
            raise SystemExit('Need a successful full-mode run bound to its own head')
        shard = next(j for j in run['jobs'] if j['name'].startswith('Source remainder'))['name']
        artifact = f"piping-e2e-collection-shard-{shard.split('(')[1].split('/')[0]}-{args.run}-{run['attempt']}"
        gh('run', 'download', str(args.run), '-R', REPO, '-n', artifact, '-D', tmp + '/collection')
        source_bytes = Path(tmp, 'collection/source.stdout.json').read_bytes()
    source = e2e_plan.collected_tests(json.loads(source_bytes))
    logs = ''.join(gh('api', f"repos/{REPO}/actions/jobs/{job['databaseId']}/logs").decode()
                   for job in run['jobs'] if job['name'].startswith(('Source remainder', 'Accessibility barrier')))
    durations, skips = parse_logs(logs, source)
    selected = {key(t) for t in e2e_plan.select_tests(plan, source)}
    if not selected <= set(durations) | skips:
        raise SystemExit(f'Log covers {len(selected & (set(durations) | skips))} of {len(selected)} selected identities')
    old = json.loads(e2e_plan.DURATION_HINTS.read_text())
    new = refresh(old, durations, skips, {
        'repository': REPO, 'pr': plan['pr'], 'run': args.run, 'head': run['headSha'],
        'log_sha256': hashlib.sha256(logs.encode()).hexdigest(),
        'collection_sha256': hashlib.sha256(source_bytes).hexdigest(),
        'meaning': ('Observed successful source durations, resolved to canonical collection identities including '
                    'separately reported tags. Skips and passes rounded to 0ms use positive scheduling floors, not '
                    'measured durations. Scheduling hints only, never acceptance limits or coverage selection; '
                    'unobserved identities keep prior hints and new ones the conservative fallback.'),
        'passed_duration_count': len(durations), 'observed_skip_count': len(skips),
        'skipped_scheduling_weight_seconds': SKIP_WEIGHT,
        'retained_prior_only_entries': len(set(old['seconds']) - set(durations) - skips)})
    if args.apply:
        e2e_plan.DURATION_HINTS.write_text(json.dumps(new, ensure_ascii=False, indent=2) + '\n')
    print(json.dumps({'applied': args.apply, 'run': args.run, 'measured': len(durations), 'skips': len(skips),
                      'newly_measured': len(set(durations) - set(old['seconds'])),
                      'fallback_before': len(selected - set(old['seconds'])),
                      'fallback_after': len(selected - set(new['seconds']))}, indent=2))


if __name__ == '__main__':
    main()
