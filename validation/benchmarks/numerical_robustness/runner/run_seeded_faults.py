#!/usr/bin/env python3
"""V-K A2: the seeded-fault kill matrix (T3 D1 §4.10 and §7.3; plan §10.5).

Standard library only. It builds VR's tests once with `--features
seeded-faults`, then runs them once per fault, with `FK_SEEDED_FAULT` naming
the fault:
- NONE first, which must pass every test;
- every fault of plan §10.4 (VK-F05 added by ROOT's ruling), each of which must
  fail at least one test: a fault no test fails is a survivor and a stop;
- an unknown id, which must make the tests that reach a site panic, naming it.

It writes `kill_matrix.jsonl` (one line per run) and each run's log into the
output directory, and exits nonzero on a survivor or a failing NONE.

Usage (from VR's directory, with the host's cargo environment set):
  python3 runner/run_seeded_faults.py --out <dir> [--only VK-F01,VK-F02] [--timeout 1800]
"""
import argparse
import functools
import json
import os
import re
import subprocess
import sys
import time

# (id, D1 §7.3 or R7 item, expected kill kind, expected to fail) — plan §10.4.
FAULTS = [
    ('VK-F01', '§7.3-1', 'value or outcome', 'RF-CHAIN, RF-SKEW, RF-WEAK soft rows (NC-STORED-ASSEMBLY)'),
    ('VK-F02', '§7.3-2', 'value or outcome', 'RF-CHAIN-T/A r1e-10/12 T, N, tw, ext (NC-SUBTRACT-ROUNDED)'),
    ('VK-F03', '§7.3-4', 'value or outcome', 'soft roots of RF-CHAIN and RF-SKEW (NC-LOST-SOFT)'),
    ('VK-F04', '§7.3-4', 'value', 'RF-SKEW, RF-INVARIANCE-ROT, RF-LARGE-ROT, RF-WEAK-W-L (NC-WRONG-TRANSFORM)'),
    ('VK-F05', '§7.3-5', 'evidence', "THIN-A and THIN-B's attempts against the records"),
    ('VK-F06', '§7.3-6', 'evidence or outcome', 'THIN-A and THIN-B selected at 128 (leave the expected-unresolved list)'),
    ('VK-F07', '§7.3-7', 'evidence', "RF-MECH's refusals against the records"),
    ('VK-F08', '§7.3-8', 'parity', 'bitwise K, sparse against dense'),
    ('VK-F10', '§7.3-10', 'bit', 'the canonical bytes and the list-permutation checks'),
    ('VK-F13', '§7.3-13', 'value', 'RF-CANCEL G1e80 net-governed rows (NC-FLOAT-SUM)'),
    ('VK-F17', '§7.3-17', 'class', "the 51 not-covered rows' class"),
    ('VK-R02', 'R7-M2', 'outcome or evidence', 'RF-LARGE-TREE-n00100-AX'),
    ('VK-R28', 'R7-M28', 'evidence', 'RF-LARGE-CHAIN-n00100-AX and TREE-n00100-AX selected higher'),
    ('VK-S1', 'directional', 'value', 'the directional-spring cases'),
    ('VK-S2', 'NC-SIGN', 'value', 'every nonzero reaction'),
]

TEST_LINE = re.compile(r'^test (\S+) \.\.\. (ok|FAILED|ignored)')


BIT_TESTS = ('every_ci_models_canonical_bytes_equal_the_generators_from_references_py_model',
             'permuting_every_list_changes_no_canonical_byte')
PARITY_TESTS = ('rf_large_at_10_and_100_members_has_bitwise_k_class_parity_and_the_dec053_basis',
                'rf_mech_is_refused_in_both_modes_with_bitwise_k')


def kinds_of(text, failing):
    """The kill kinds a run's output shows: value (a predicate failure),
    outcome (a case not selected, leaving the expected-unresolved list, or an
    outcome pin), evidence (the per-case records), class (the class
    correspondence), floor (the not-covered set), bit (the canonical bytes),
    parity (the binary64 parity tests); panic for the unknown id."""
    if 'FK_SEEDED_FAULT: unknown fault id' in text:
        return ['panic']
    kinds = set()
    for line in text.splitlines():
        if 'FAILURE' in line:
            if 'predicate' in line:
                kinds.add('value')
            if any(k in line for k in ('not selected', 'expected-unresolved', 'mechanism was published',
                                       'not published', 'overflowed')):
                kinds.add('outcome')
            if 'K4SRC' in line:
                kinds.add('bit')
            if 'CLASS' in line:
                kinds.add('class')
            if 'FLOOR' in line:
                kinds.add('floor')
        if 'per-case records differ' in line:
            kinds.add('evidence')
        if 'failed: RF-' in line and ': outcomes' in line:
            kinds.add('outcome')
    for t in failing:
        if t in BIT_TESTS:
            kinds.add('bit')
        if t in PARITY_TESTS:
            kinds.add('parity')
    return sorted(kinds)


@functools.lru_cache(maxsize=1)
def machine_roots():
    top = subprocess.run(['git', 'rev-parse', '--show-toplevel'], capture_output=True,
                         text=True).stdout.strip()
    return (os.path.dirname(top) if top else None), os.path.expanduser('~')


def sanitize(line):
    """No machine path in a committed record: the worktrees' parent becomes
    <wt> and the home directory <home>."""
    wt, home = machine_roots()
    if wt:
        line = line.replace(wt, '<wt>')
    return line.replace(home, '<home>')


def samples(text, limit=10):
    keep = [line for line in text.splitlines()
            if 'FAILURE' in line or 'panicked at' in line or 'records differ' in line
            or line.startswith('assertion') or 'left:' in line]
    return [sanitize(line) for line in keep[:limit]]


def run(fault, out, timeout):
    env = dict(os.environ)
    env['FK_SEEDED_FAULT'] = fault
    env.setdefault('RUST_TEST_THREADS', '2')
    argv = ['cargo', 'test', '--offline', '--locked', '-j', '4', '--features', 'seeded-faults',
            '--no-fail-fast', '--', '--nocapture']
    t0 = time.monotonic()
    try:
        proc = subprocess.run(argv, env=env, capture_output=True, text=True, timeout=timeout)
        text, code, timed_out = proc.stdout + proc.stderr, proc.returncode, False
    except subprocess.TimeoutExpired as e:
        text = (e.stdout or b'').decode(errors='replace') if isinstance(e.stdout, bytes) else (e.stdout or '')
        code, timed_out = None, True
    seconds = round(time.monotonic() - t0, 1)
    with open(os.path.join(out, 'logs', '%s.log' % fault), 'w') as fh:
        fh.write(text)
    tests = [(m.group(1), m.group(2)) for m in (TEST_LINE.match(l) for l in text.splitlines()) if m]
    failing = sorted({name for name, status in tests if status == 'FAILED'})
    return {'tests_run': len(tests), 'failing_tests': failing, 'exit_code': code,
            'timed_out': timed_out, 'seconds': seconds, 'kinds': kinds_of(text, failing),
            'samples': samples(text)}, text


def verdict_of(fault, result, text):
    if fault == 'NONE':
        return 'pass' if not result['failing_tests'] and result['exit_code'] == 0 else 'NONE FAILS'
    if fault == 'VK-UNKNOWN':
        return 'panics as required' if result['failing_tests'] and 'unknown fault id' in text else 'NO PANIC'
    if result['failing_tests']:
        return 'killed'
    return 'TIMEOUT' if result['timed_out'] else 'SURVIVED'


def rebuild(out):
    plan = [('NONE', '-', 'none', 'every test passes')] + FAULTS
    plan += [('VK-UNKNOWN', '-', 'panic', 'an unknown id panics at the first site')]
    rows, status = [], 0
    for fault, item, kind, expected in plan:
        with open(os.path.join(out, 'logs', '%s.log' % fault)) as fh:
            text = fh.read()
        tests = [(m.group(1), m.group(2)) for m in (TEST_LINE.match(l) for l in text.splitlines()) if m]
        failing = sorted({name for name, st in tests if st == 'FAILED'})
        old = {}
        path = os.path.join(out, 'kill_matrix.jsonl')
        if os.path.exists(path):
            for line in open(path):
                r = json.loads(line)
                old[r['fault']] = r
        result = {'tests_run': len(tests), 'failing_tests': failing,
                  'exit_code': old.get(fault, {}).get('exit_code', 0 if not failing else 101),
                  'timed_out': False, 'seconds': old.get(fault, {}).get('seconds'),
                  'kinds': kinds_of(text, failing), 'samples': samples(text)}
        row = {'fault': fault, 'maps_to': item, 'expected_kind': kind, 'expected': expected}
        row.update(result)
        row['verdict'] = verdict_of(fault, result, text)
        status |= row['verdict'] in ('NONE FAILS', 'NO PANIC', 'SURVIVED', 'TIMEOUT')
        rows.append(row)
    with open(os.path.join(out, 'kill_matrix.jsonl'), 'w') as fh:
        for row in rows:
            fh.write(json.dumps(row, sort_keys=True) + '\n')
    for row in rows:
        print('%-10s %-18s %s' % (row['fault'], row['verdict'], row['kinds']))
    return int(status)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--out', required=True)
    ap.add_argument('--only')
    ap.add_argument('--timeout', type=int, default=1800)
    ap.add_argument('--from-logs', action='store_true',
                    help='rebuild kill_matrix.jsonl from the logs of an earlier run, running nothing')
    args = ap.parse_args()
    os.makedirs(os.path.join(args.out, 'logs'), exist_ok=True)
    if args.from_logs:
        return rebuild(args.out)
    build = subprocess.run(['cargo', 'test', '--offline', '--locked', '-j', '4', '--features',
                            'seeded-faults', '--no-run'], capture_output=True, text=True)
    if build.returncode:
        print(build.stdout + build.stderr)
        return 2
    only = set(args.only.split(',')) if args.only else None
    rows, status = [], 0
    plan = [('NONE', '-', 'none', 'every test passes')]
    plan += [f for f in FAULTS if only is None or f[0] in only]
    plan += [('VK-UNKNOWN', '-', 'panic', 'an unknown id panics at the first site')]
    for fault, item, kind, expected in plan:
        result, text = run(fault, args.out, args.timeout)
        row = {'fault': fault, 'maps_to': item, 'expected_kind': kind, 'expected': expected}
        row.update(result)
        row['verdict'] = verdict_of(fault, result, text)
        if row['verdict'] in ('NONE FAILS', 'NO PANIC', 'SURVIVED', 'TIMEOUT'):
            status = 1
        rows.append(row)
        print('%-10s %-10s %-24s %6.1fs  failing %s  kinds %s' % (
            fault, row['verdict'], item, result['seconds'], result['failing_tests'], result['kinds']),
            flush=True)
    with open(os.path.join(args.out, 'kill_matrix.jsonl'), 'w') as fh:
        for row in rows:
            fh.write(json.dumps(row, sort_keys=True) + '\n')
    return status


if __name__ == '__main__':
    sys.exit(main())
