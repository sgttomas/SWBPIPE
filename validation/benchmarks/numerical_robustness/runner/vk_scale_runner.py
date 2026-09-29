#!/usr/bin/env python3
"""V-K checkpoint B: the scale runs (plan §13; brief Scope 12; ROOT's Q5 ruling).

Standard library only. It imports K6's runner by path
(`P/core/solver/performance_harness/runner/k6_runner.py`) and uses:
- `launch()`: the `/usr/bin/time` wrapper, the 100 ms RSS watchdog on the
  binary's pid, the process-group SIGKILL, the load and `memorystatus`;
- `wait_for_quiet_host()` and `metadata()`;
- `admission()`: the ascent rule with ρ measured, with ROOT's footprint and
  projected-RSS amendments.
It runs `vk_scale` (release, from a `git archive` of the exact commit) once
per model, with no repeats. V-K makes no timing claim; timing is K6b's.

The tiers follow ROOT's Q5 ruling, 100 → 1,000 → 10,000:
- V1: the six RF-LARGE frames at 100 members (their models are committed),
  which measure ρ;
- V2: the six at 1,000 members;
- V3: the six at 10,000 members, only with `--approve-10000`, as ROOT
  approves.
Every run uses C = 8 GiB, a heap cap of C − 512 MiB and a 1,800 s timeout.

**Admission** is K6's rule on V-K's estimate (`vk_scale --counts-only`, whose
`estimate_max_bytes` is K6b's E_max, ported). The binary's own backstop (an
estimate above half the heap cap) is checked here first. A row the binary
would refuse is deferred by name (K6b's C ruling), so every admitted row is
one the binary also admits.

**Stops,** after which the tier ends:
- a failing comparison on a covered row;
- a not-covered set different from the committed list, or a nonempty C9 set
  from S_full;
- a case not selected (no RF-LARGE case is on the expected-unresolved list);
- a class mismatch, or a discriminating control that passes;
- an unaccounted report, RCM inequality, or counts that differ from K4's
  storage counts;
- a watchdog kill, heap-cap abort or timeout on an admitted run, the binary
  refusing an admitted run, or an error;
- a memory-guard kill during the tier.

**One named exception** (ROOT's ruling for V-K's B, matching K6b's): the KF3
availability finding. A 10,000-member case that ends `Unresolved(ExactSumSpan)`
is recorded with its attempts and work, and its rows are counted as
`unresolved_availability`, never as passes. The tier then continues. The
exception applies only when all of these hold:
- the model has 10,000 members;
- the outcome is `Unresolved ExactSumSpan`;
- nothing was published (no pass, absolute-range pass, not-covered or
  structural-zero row, and every row counted as failed);
- the only failure is that case's "not selected" line.
Any other unresolved reason, a failure on a covered row, or any other stop
condition still stops. V3's final figures are re-run after KF3 merges.
`runner/test_vk_scale_runner.py` tests the exception's edges.

Usage (from VR's directory):
  python3 runner/vk_scale_runner.py --counts --binary B --models DIR --out FILE [--tiers V1,V2,V3]
  python3 runner/vk_scale_runner.py --plan --counts-file FILE [--records DIR] [--approve-10000]
  python3 runner/vk_scale_runner.py --run V1|V2|V3 --binary B --models DIR --counts-file FILE
      --records DIR --source-commit C --source-tree T [--approve-10000] [--memguard-log F]
"""
import argparse
import importlib.util
import json
import os
import sys

sys.dont_write_bytecode = True

HERE = os.path.dirname(os.path.abspath(__file__))
P_ROOT = os.path.normpath(os.path.join(HERE, '..', '..', '..', '..'))
K6_RUNNER = os.path.join(P_ROOT, 'core/solver/performance_harness/runner/k6_runner.py')


def load_k6():
    spec = importlib.util.spec_from_file_location('k6_runner', K6_RUNNER)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


k6 = load_k6()

MODE = 'w1a'
FRAMES = ('CHAIN-%s-AX', 'CHAIN-%s-ROT', 'TREE-%s-AX', 'TREE-%s-ROT', 'CONT-%s-AX', 'CONT-%s-ROT')
TIERS = (('V1', 100), ('V2', 1000), ('V3', 10000))
CONDITIONAL_TIER = 'V3'
TIMEOUT_S = 1800
COUNTS_CAP_BYTES = 512 * k6.MIB
VK_HEAP_CAP_MARKER = 'vk_scale: heap cap refused '
# ROOT's ruling for V-K's B (the KF3 availability finding); see the module text.
KF3_AVAILABILITY = 'kf3_availability_exact_sum_span'
KF3_MEMBERS = 10000
KF3_OUTCOME_PREFIX = 'Unresolved ExactSumSpan '


def model_id(frame, members):
    return 'RF-LARGE-' + frame % ('n%05d' % members)


def schedule():
    runs = []
    for tier, members in TIERS:
        for frame in FRAMES:
            mid = model_id(frame, members)
            run = {'order': len(runs) + 1, 'tier': tier, 'model': mid, 'mode': MODE,
                   'family': k6.family_of(mid), 'members': members,
                   'rss_cap_bytes': k6.C_DEFAULT, 'heap_cap_bytes': k6.C_DEFAULT - k6.HEAP_MARGIN,
                   'timeout_s': TIMEOUT_S, 'conditional': tier == CONDITIONAL_TIER}
            run['run_id'] = '%02d_%s_%s' % (run['order'], mid, MODE)
            runs.append(run)
    return runs


def model_file(models_dir, mid, members):
    """The generator's model file for 1,000 and 10,000 members; None where the case line
    carries the model (100 members)."""
    return None if members <= 100 else os.path.join(models_dir, mid + '.json')


def binary_argv(binary, run, models_dir, counts_only=False, heap_cap=None):
    argv = [binary, '--case', run['model'], '--heap-cap-bytes', str(heap_cap or run['heap_cap_bytes'])]
    path = model_file(models_dir, run['model'], run['members'])
    if path:
        argv += ['--model-file', path]
    if counts_only:
        argv.append('--counts-only')
    return argv


def objects_of(record_dir, run_id):
    path = os.path.join(record_dir, run_id + '.jsonl')
    if not os.path.exists(path):
        return []
    with open(path) as fh:
        return k6.jsonl(fh)


def by_kind(objects, kind):
    return next((o for o in objects if o.get('kind') == kind), None)


def reclassify(record, record_dir):
    """K6's `classify` knows K6's own heap-cap marker. V-K's allocator writes its own."""
    path = os.path.join(record_dir, record['run_id'] + '.stderr.txt')
    if record['classification'] == 'error' and os.path.exists(path):
        with open(path) as fh:
            if VK_HEAP_CAP_MARKER in fh.read():
                record = dict(record, classification='heap_cap_abort', classification_detail='vk_scale marker')
    return record


# --------------------------------------------------------------- counts


def run_counts(binary, models_dir, out_path, tiers=None, log=print):
    """Counts-only runs (no solve), one process per model, under a 512 MiB heap cap."""
    record_dir = os.path.join(os.path.dirname(os.path.abspath(out_path)), 'counts_runs')
    lines = []
    for run in [r for r in schedule() if tiers is None or r['tier'] in tiers]:
        rec = k6.launch(binary_argv(binary, run, models_dir, counts_only=True, heap_cap=COUNTS_CAP_BYTES),
                        rss_cap_bytes=k6.C_DEFAULT, timeout_s=600, record_dir=record_dir,
                        run_id='counts_' + run['model'])
        objects = objects_of(record_dir, 'counts_' + run['model'])
        counts = by_kind(objects, 'counts')
        line = {'kind': 'counts', 'model': run['model'], 'members': run['members'],
                'classification': rec['classification'], 'wall_s': rec['wall_s'],
                'heap_peak': (by_kind(objects, 'summary') or {}).get('repeats_heap_peak')}
        if counts:
            line.update({k: v for k, v in counts.items() if k not in ('kind', 'schema')})
            line[k6.estimate_key(MODE)] = counts['estimate_max_bytes']
        lines.append(line)
        log('%s: %s estimate %s B' % (run['model'], rec['classification'], line.get('estimate_max_bytes')))
    with open(out_path, 'w') as fh:
        for line in lines:
            fh.write(json.dumps(line, sort_keys=True) + '\n')
    return lines


# --------------------------------------------------------------- admission


def admission(run, counts, records, baseline_rss, baseline_footprint, approve_10000):
    line = counts.get(run['model'])
    if line is None or line.get('estimate_max_bytes') is None:
        return {'decision': 'deferred', 'reason': 'deferred:no_counts_line'}
    if line['estimate_max_bytes'] > run['heap_cap_bytes'] // 2:
        return {'decision': 'deferred', 'reason': "deferred:the binary's backstop would refuse "
                '(estimate %d B > half the heap cap %d B)' % (line['estimate_max_bytes'],
                                                               run['heap_cap_bytes'] // 2)}
    if run['conditional'] and not approve_10000:
        return {'decision': 'deferred', 'reason': 'deferred:10000_members_needs_root_approval'}
    return k6.admission(run, counts, records, baseline_rss, require_ascent=run['members'] > 100,
                        baseline_footprint_bytes=baseline_footprint)


def plan(counts_path, record_dir, approve_10000):
    counts = k6.read_counts(counts_path)
    records = k6.read_records(record_dir) if record_dir else []
    rows = []
    for run in schedule():
        d = admission(run, counts, records, None, None, approve_10000)
        line = counts.get(run['model'], {})
        rows.append((run['run_id'], run['tier'], line.get('estimate_max_bytes'), d['decision'], d['reason']))
    for r in rows:
        print('%-34s %-3s %14s  %-9s %s' % r)
    return rows


# --------------------------------------------------------------- run


def memguard_kills(path):
    if not path or not os.path.exists(path):
        return None
    with open(path) as fh:
        return sum(1 for line in fh if ' KILLED ' in line)


def availability_exception(run, objects):
    """The KF3 availability exception's name when it applies to this run, else None."""
    w1, report = by_kind(objects, 'w1'), by_kind(objects, 'report')
    if run.get('members') != KF3_MEMBERS or w1 is None or report is None:
        return None
    outcome = str(w1.get('outcome', ''))
    if not outcome.startswith(KF3_OUTCOME_PREFIX):
        return None
    published = (report['pass'], report['pass_absolute_range'], report['not_covered'], report['structural_zero'])
    if any(published) or report['fail'] != report['rows']:
        return None
    heads = report.get('failures_head') or []
    if report['failures'] != 1 or len(heads) != 1 or (': not selected: ' + KF3_OUTCOME_PREFIX) not in heads[0]:
        return None
    return KF3_AVAILABILITY


def stop_reasons(record, objects, admitted, run=None):
    reasons = k6.stop_reasons(record, admitted)
    if record['classification'] == 'timed_out' and admitted:
        reasons.append('timed out on an admitted run')
    if record['classification'] != 'ok':
        return reasons
    w1, report, rcm = by_kind(objects, 'w1'), by_kind(objects, 'report'), by_kind(objects, 'rcm')
    if w1 is None or report is None or rcm is None:
        return reasons + ['a phase line is missing']
    exception = availability_exception(run or {}, objects)
    if exception is None:
        if not str(w1.get('outcome', '')).startswith('Selected'):
            reasons.append('not selected: %s' % w1.get('outcome'))
        if report['fail'] or report['failures']:
            reasons.append('%d failure(s): %s' % (report['failures'], report['failures_head'][:3]))
    if not w1.get('counts_match_storage'):
        reasons.append("V-K's counts differ from K4's storage counts")
    if not report['accounted']:
        reasons.append('the report does not account for every row')
    if not report['not_covered_equal']:
        reasons.append('not-covered set %s, committed %s' % (report['not_covered_rows'],
                                                             report['not_covered_committed']))
    if report.get('not_covered_rows_s_full'):
        reasons.append('C9: not covered with S_full: %s' % report['not_covered_rows_s_full'])
    if report['class_mismatches']:
        reasons.append('class mismatches: %s' % report['class_mismatches'][:3])
    if report['controls_undiscriminated'] or report['controls_unexpectedly_failing']:
        reasons.append('controls: %s %s' % (report['controls_undiscriminated'],
                                            report['controls_unexpectedly_failing']))
    if not rcm['equal']:
        reasons.append("K4's RCM differs from SD's")
    return reasons


def run_tier(tier, binary, models_dir, counts_path, record_dir, source_commit, source_tree,
             approve_10000=False, memguard_log=None, log=print):
    counts = k6.read_counts(counts_path)
    os.makedirs(record_dir, exist_ok=True)
    md_path = os.path.join(record_dir, 'metadata.json')
    if not os.path.exists(md_path):
        with open(md_path, 'w') as fh:
            json.dump(k6.metadata(binary, source_commit, source_tree), fh, indent=1, sort_keys=True)
            fh.write('\n')
    base = k6.launch([binary, '--noop', '--heap-cap-bytes', str(k6.C_DEFAULT - k6.HEAP_MARGIN)],
                     rss_cap_bytes=k6.C_DEFAULT, timeout_s=60, record_dir=record_dir, run_id='baseline_' + tier)
    baseline_rss = base.get('peak_rss_bytes')
    baseline_footprint = base.get('rss', {}).get('time_peak_footprint_bytes')
    kills_before = memguard_kills(memguard_log)
    log('baseline (no-op) peak RSS %s bytes, footprint %s bytes; memory-guard kills so far %s'
        % (baseline_rss, baseline_footprint, kills_before))
    stop = []
    for run in [r for r in schedule() if r['tier'] == tier]:
        records = k6.read_records(record_dir)
        if any(x.get('run_id') == run['run_id'] and x.get('classification') != 'not_run'
               for x in k6.run_entries(records)):
            log('%s: already recorded; not repeated' % run['run_id'])
            continue
        decision = admission(run, counts, records, baseline_rss, baseline_footprint, approve_10000)
        entry = dict(run, admission=decision, baseline_rss_bytes=baseline_rss,
                     baseline_footprint_bytes=baseline_footprint, source_commit=source_commit,
                     source_tree=source_tree, estimate_adm_bytes=counts.get(run['model'], {}).get(
                         'estimate_max_bytes'))
        if decision['decision'] != 'admitted':
            entry.update(classification='not_run')
            k6.append_record(record_dir, entry)
            log('%s: %s' % (run['run_id'], decision['reason']))
            continue
        quiet = k6.wait_for_quiet_host(log)
        record = k6.launch(binary_argv(binary, run, models_dir), rss_cap_bytes=run['rss_cap_bytes'],
                           timeout_s=run['timeout_s'], record_dir=record_dir, run_id=run['run_id'])
        record = reclassify(record, record_dir)
        objects = objects_of(record_dir, run['run_id'])
        entry.update({k: v for k, v in record.items() if k != 'watchdog_samples'})
        entry['quiet_host'] = quiet
        entry['peak_footprint_bytes'] = record['rss'].get('time_peak_footprint_bytes')
        entry['phases'] = {o['kind']: {k: o.get(k) for k in ('elapsed_ns', 'heap_peak', 'heap_peak_move')}
                           for o in objects if 'elapsed_ns' in o}
        w1 = by_kind(objects, 'w1') or {}
        entry['outcome'] = w1.get('outcome')
        entry['selected_precision'] = w1.get('selected_precision')
        entry['invocation_charged'] = w1.get('invocation_charged')
        reasons = stop_reasons(record, objects, True, run)
        exception = availability_exception(run, objects) if record['classification'] == 'ok' else None
        if exception:
            report = by_kind(objects, 'report')
            entry['availability_exception'] = exception
            entry['unresolved_availability'] = report['rows']
            log('%s: %s (%d rows counted as unresolved_availability, never as passes)'
                % (run['run_id'], exception, report['rows']))
        k6.append_record(record_dir, entry)
        log('%s: %s %.1fs rss=%s outcome=%s' % (run['run_id'], record['classification'], record['wall_s'],
                                                 record.get('peak_rss_bytes'), entry['outcome']))
        if reasons:
            stop = reasons
            log('STOP: ' + '; '.join(reasons))
            break
    kills_after = memguard_kills(memguard_log)
    if kills_before is not None and kills_after != kills_before:
        stop = stop + ['the memory guard killed %d process(es) during the tier' % (kills_after - kills_before)]
        log('STOP: ' + stop[-1])
    return {'stop': stop}


def main(argv=None):
    ap = argparse.ArgumentParser()
    ap.add_argument('--counts', action='store_true')
    ap.add_argument('--plan', action='store_true')
    ap.add_argument('--run', choices=[t for t, _ in TIERS])
    ap.add_argument('--binary')
    ap.add_argument('--models')
    ap.add_argument('--out')
    ap.add_argument('--counts-file')
    ap.add_argument('--records')
    ap.add_argument('--source-commit')
    ap.add_argument('--source-tree')
    ap.add_argument('--approve-10000', action='store_true')
    ap.add_argument('--memguard-log')
    ap.add_argument('--tiers', help='--counts only: the tiers to count (default all)')
    args = ap.parse_args(argv)
    if args.counts:
        run_counts(args.binary, args.models, args.out,
                   tiers=set(args.tiers.split(',')) if args.tiers else None)
        return 0
    if args.plan:
        plan(args.counts_file, args.records, args.approve_10000)
        return 0
    if args.run:
        result = run_tier(args.run, args.binary, args.models, args.counts_file, args.records,
                          args.source_commit, args.source_tree, approve_10000=args.approve_10000,
                          memguard_log=args.memguard_log)
        return 1 if result['stop'] else 0
    ap.error('one of --counts, --plan or --run')


if __name__ == '__main__':
    sys.exit(main())
