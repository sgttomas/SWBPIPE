#!/usr/bin/env python3
"""K6b: the W1 smoke run, the per-run W1 figures and the projection (T3 K6b plan sections 4, 5; Q1, Q5).

Standard library only; it reuses K6's runner (``k6_runner``) for launching, the watchdog, the
records and the parsers, and adds only what W1 needs:

- ``w1_figures``: from one ``w1a`` process's JSONL, the outcome, the charged work, the
  ``w1_solve`` median and minimum, seconds per limb-multiply equivalent (the median
  ``w1_solve`` time divided by the meter's charged work), the heap peaks of the call and of
  each prefix, the prefix increments, and the stop rule's increment (the call's peak minus
  the peak through the last verification pass) with its tracker-entry equivalent;
- ``smoke``: the ``w1a`` and ``sparse`` runs at 10 and 100 members and the DEC-053 nine,
  under a 512 MiB heap cap (checkpoint A2);
- ``project``: W1-T3's and W1-T4's rows from the 100-member runs: the projected call time and
  process time, the projected work, ρ and the admission it gives, and the tracker term.

Observation only: nothing here asserts a time or memory bound. Every projection is labelled
as one; it is not a claim.
"""
import argparse
import json
import math
import os
import sys

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import k6_runner as r  # noqa: E402

# One ExtremeTracker entry (K4R/adaptive.rs:540-544): two ExactWideSum values of 2,144 bytes
# and a u64, 4,304 bytes with alignment (H/src/k6/w1/counts.rs, TRACKER_ENTRY_BYTES).
TRACKER_ENTRY_BYTES = 2 * 2144 + 16
SMOKE_REPEATS = 3


def objects_of(record_dir, run_id):
    with open(os.path.join(record_dir, run_id + '.jsonl')) as fh:
        return r.jsonl(fh)


def w1_figures(objects):
    """The W1 figures of one w1a process (module documentation). None if it has no outcome."""
    counts = r.first_of(objects, 'counts') or {}
    outcomes = [o for o in objects if o.get('kind') == 'outcome']
    if not outcomes:
        return None
    first = outcomes[0]
    solves = [o for o in objects if o.get('kind') == 'stage' and o.get('stage') == 'w1_solve']
    elapsed = [o['elapsed_ns'] for o in solves]
    peaks = [o['heap_peak'] - o['heap_current_begin'] for o in solves]
    prefixes = sorted((o for o in objects if o.get('kind') == 'stage'
                       and o.get('stage', '').startswith('w1_prefix_')),
                      key=lambda o: int(o['stage'].rsplit('_', 1)[1]))
    prefix_lines = {o['prefix']: o for o in objects if o.get('kind') == 'prefix'}
    charged = first.get('meter_charged')
    median_ns = r.median(elapsed) if elapsed else None
    call_peak = max(peaks) if peaks else None
    out = {
        'model': counts.get('model'), 'class': first.get('class'),
        'selected_precision': first.get('selected_precision'), 'rows': first.get('rows'),
        'attempts': first.get('attempts'), 'meter_charged': charged,
        'solve_samples': len(elapsed), 'solve_median_ns': median_ns,
        'solve_min_ns': min(elapsed) if elapsed else None,
        's_per_lme_median': (median_ns / 1e9 / charged) if (median_ns and charged) else None,
        's_per_lme_min': (min(elapsed) / 1e9 / charged) if (elapsed and charged) else None,
        'call_heap_peak': call_peak,
        'estimate_adm_bytes_w1a': counts.get('estimate_adm_bytes_w1a'),
        'estimate_w1_sel128_bytes': counts.get('estimate_w1_sel128_bytes'),
        'estimate_w1_decide_bytes': counts.get('estimate_w1_decide_bytes'),
        'prefixes': [],
    }
    previous = 0
    for o in prefixes:
        j = int(o['stage'].rsplit('_', 1)[1])
        peak = o['heap_peak'] - o['heap_current_begin']
        line = prefix_lines.get(j, {})
        out['prefixes'].append({'prefix': j, 'through_segment': line.get('through_segment'),
                                'case_limit': line.get('case_limit'), 'elapsed_ns': o['elapsed_ns'],
                                'heap_peak': peak, 'increment': peak - previous})
        previous = max(previous, peak)
    if call_peak is not None and out['prefixes']:
        increment = call_peak - max(p['heap_peak'] for p in out['prefixes'])
        out['decide_increment'] = increment
        out['tracker_entries_equivalent'] = increment / TRACKER_ENTRY_BYTES
    if call_peak and out['estimate_adm_bytes_w1a']:
        out['heap_over_e_max'] = call_peak / out['estimate_adm_bytes_w1a']
    if call_peak and out['estimate_w1_sel128_bytes']:
        out['heap_over_e_sel128'] = call_peak / out['estimate_w1_sel128_bytes']
    return out


def sparse_figures(objects):
    """K6's sparse run: the median staged total per repeat and the median entry_checked time."""
    stages = r.aggregate_stages(objects)
    staged = sum(v['median_ns'] for k, v in stages.items() if k not in r.ENTRY_STAGES)
    entry = stages.get('entry_checked', {}).get('median_ns')
    return {'staged_median_ns': staged, 'entry_checked_median_ns': entry}


def smoke_ids():
    return r.rf_ids(10) + r.rf_ids(100) + ['DEC053:' + r.dec053_fixture_id(s) for s in r.DEC053]


def smoke(binary, record_dir, counts_path, log=print):
    """--smoke (A2): per model, one w1a process (3 repeats, prefixes, rows dump) and one sparse
    process (3 repeats), every run under a 512 MiB heap cap and a 1 GiB RSS watchdog."""
    os.makedirs(record_dir, exist_ok=True)
    results = []
    base = r.baseline_run(binary, record_dir, 'w1smoke')
    row = {'run_id': 'baseline_w1smoke', 'classification': base['classification'],
           'peak_rss_bytes': base.get('peak_rss_bytes'), 'rss': base.get('rss')}
    results.append(row)
    r.append_record(record_dir, row)
    for model_id in smoke_ids():
        for mode in ('w1a', 'sparse'):
            run_id = 'w1smoke_%s_%s' % (model_id.replace(':', '_'), mode)
            argv = [binary, '--model', model_id, '--mode', mode, '--heap-cap-bytes', str(512 * r.MIB),
                    '--repeats', str(SMOKE_REPEATS), '--counts-file', counts_path]
            if mode == 'w1a':
                argv += ['--w1-prefixes', '--dump-published', os.path.join(record_dir, run_id + '.rows')]
            rec = r.launch(argv, rss_cap_bytes=r.GIB, timeout_s=r.TIMEOUT_SMALL, record_dir=record_dir,
                           run_id=run_id)
            objects = objects_of(record_dir, run_id)
            row = {'run_id': run_id, 'model': model_id, 'mode': mode,
                   'classification': rec['classification'], 'wall_s': rec['wall_s'],
                   'peak_rss_bytes': rec.get('peak_rss_bytes'),
                   'peak_footprint_bytes': rec['rss'].get('time_peak_footprint_bytes'),
                   'parity_failures': len(r.parity_failures(objects))}
            if mode == 'w1a':
                row['w1'] = w1_figures(objects)
            else:
                row['sparse'] = sparse_figures(objects)
            results.append(row)
            r.append_record(record_dir, row)
            log(json.dumps({k: row[k] for k in ('run_id', 'classification', 'wall_s', 'parity_failures')}))
    return results


def loglog_slope(x0, y0, x1, y1):
    return math.log(y1 / y0) / math.log(x1 / x0)


def project(record_dir, counts_path, tiers=('W1-T3', 'W1-T4')):
    """W1-T3's and W1-T4's rows, projected from the smoke's 10- and 100-member runs.

    - Work: charged(n) = charged(100) × (n/100)^s, s the log-log slope of charged work from 10
      to 100 members of the same model family and orientation.
    - Call time: the 100-member run's seconds per LME × the projected work.
    - Process time (a W1 process of 5 repeats; pass 1 also runs its prefixes, about 2.1 calls
      at 100 members, measured): repeats × call + prefixes × call.
    - ρ: the 100-member run's max(footprint net of the no-op baseline, heap move) / E_max; the
      admission: E_max(n) × ρ ≤ C/2, and projected RSS ≤ 0.8 C, as ruled.
    - The tracker: E_decide(n) (the worst case) and, as the expectation, the 100-member stop
      rule's entry equivalent scaled by the rows.
    """
    counts = r.read_counts(counts_path)
    rows_by_id = {row.get('run_id'): row for row in r.read_records(record_dir) if 'run_id' in row}
    base = rows_by_id.get('baseline_w1smoke') or {}
    out, total = [], 0.0
    for run in [x for x in r.schedule() if x['tier'] in tiers]:
        n = run['members']
        small = run['model'].replace('-n%05d-' % n, '-n00100-')
        tiny = run['model'].replace('-n%05d-' % n, '-n00010-')
        mode = run['mode']
        rid = lambda m: 'w1smoke_%s_%s' % (m, mode)
        s100, s10 = rows_by_id.get(rid(small)), rows_by_id.get(rid(tiny))
        line = counts.get(run['model'], {})
        row = {'order': run['order'], 'tier': run['tier'], 'model': run['model'], 'mode': mode,
               'pass': run['pass'], 'estimate_adm_bytes': line.get(r.estimate_key(mode))}
        if not s100 or not s10:
            row['basis'] = 'no smoke record at 10 or 100 members'
            out.append(row)
            continue
        if mode == 'w1a':
            f100, f10 = s100['w1'], s10['w1']
            slope = loglog_slope(10, f10['meter_charged'], 100, f100['meter_charged'])
            work = f100['meter_charged'] * (n / 100) ** slope
            call_s = f100['s_per_lme_median'] * work
            prefix_calls = sum(p['elapsed_ns'] for p in f100['prefixes']) / f100['solve_median_ns']
            process_s = run['repeats'] * call_s + (prefix_calls * call_s if run['pass'] == 1 else 0.0)
            small_counts = counts.get(small, {})
            rows_ratio = line.get('w1_rows', 0) / small_counts['w1_rows'] if small_counts.get('w1_rows') else None
            kept = f100.get('tracker_entries_equivalent')
            row.update({'work_slope_10_100': round(slope, 3), 'projected_work_lme': int(work),
                        'projected_call_s': round(call_s, 2), 'prefix_calls_at_100': round(prefix_calls, 2),
                        'projected_process_s': round(process_s, 1),
                        'tracker_worst_bytes': line.get('estimate_w1_decide_bytes'),
                        'tracker_entries_at_100': round(kept, 1) if kept is not None else None,
                        'tracker_entries_expected': round(kept * rows_ratio, 1)
                        if (kept is not None and rows_ratio) else None,
                        'tracker_bytes_expected': int(kept * rows_ratio * TRACKER_ENTRY_BYTES)
                        if (kept is not None and rows_ratio) else None})
            heap_move = f100['call_heap_peak']
        else:
            sp = s100['sparse']
            per_repeat = (sp['staged_median_ns'] + (sp['entry_checked_median_ns'] or 0)) / 1e9 * (n / 100)
            process_s = run['repeats'] * per_repeat
            row.update({'projected_process_s': round(process_s, 1),
                        'basis': 'sparse: 100-member staged + entry_checked median x members ratio'})
            summary = r.first_of(objects_of(record_dir, s100['run_id']), 'summary') or {}
            heap_move = summary.get('repeats_heap_peak_move')
        small_estimate = counts.get(small, {}).get(r.estimate_key(mode))
        footprint_net = max(0, (s100.get('peak_footprint_bytes') or 0)
                            - ((base.get('rss') or {}).get('time_peak_footprint_bytes') or 0))
        rho = max(footprint_net, heap_move or 0) / small_estimate if small_estimate else None
        row['rho_from_100'] = round(rho, 4) if rho else None
        est = row['estimate_adm_bytes']
        if est and rho:
            footprint = est * rho
            rss = footprint * r.RSS_TO_FOOTPRINT_DEFAULT
            row.update({'footprint_projected_bytes': int(footprint),
                        'rss_projected_bytes': int(rss),
                        'admission_projected': 'admitted' if (footprint <= r.C_DEFAULT // 2
                                                              and rss <= r.PROJECTED_RSS_FRACTION * r.C_DEFAULT)
                        else 'deferred'})
        total += row.get('projected_process_s') or 0.0
        out.append(row)
    return {'observation_only': True, 'rows': out, 'total_s_by_tier': {
        t: round(sum(x.get('projected_process_s') or 0.0 for x in out if x['tier'] == t), 1) for t in tiers}}


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument('--smoke', action='store_true')
    ap.add_argument('--project', action='store_true')
    ap.add_argument('--figures', help='print the W1 figures of one w1a JSONL file')
    ap.add_argument('--binary')
    ap.add_argument('--records')
    ap.add_argument('--counts')
    args = ap.parse_args(argv)
    if args.figures:
        with open(args.figures) as fh:
            print(json.dumps(w1_figures(r.jsonl(fh)), indent=1))
        return 0
    if args.smoke:
        smoke(args.binary, args.records, args.counts)
        return 0
    if args.project:
        print(json.dumps(project(args.records, args.counts), indent=1))
        return 0
    ap.print_help()
    return 2


if __name__ == '__main__':
    sys.exit(main())
