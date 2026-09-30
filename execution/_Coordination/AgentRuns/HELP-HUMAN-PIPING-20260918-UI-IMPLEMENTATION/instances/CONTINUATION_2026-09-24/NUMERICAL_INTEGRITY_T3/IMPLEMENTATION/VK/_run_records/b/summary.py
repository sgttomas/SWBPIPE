#!/usr/bin/env python3
"""V-K B: the per-run summary of `runs/` (records.jsonl and each run's JSONL).

Standard library only. Usage: python3 summary.py (from this directory).
"""
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
RUNS = os.path.join(HERE, 'runs')
MIB = 2 ** 20


def objects(run_id):
    path = os.path.join(RUNS, run_id + '.jsonl')
    if not os.path.exists(path):
        return {}
    return {o['kind']: o for o in (json.loads(l) for l in open(path) if l.startswith('{'))}


def main():
    records = [json.loads(l) for l in open(os.path.join(RUNS, 'records.jsonl')) if l.strip()]
    runs = [r for r in records if 'order' in r]
    print('run | class | outcome | exception | rows pass abs nc fail unresolved_availability | C9 (S_full) not covered | '
          'RCM | counts=storage | binary64 sparse')
    for r in runs:
        o = objects(r['run_id'])
        w1, rep, rcm, b64 = o.get('w1', {}), o.get('report', {}), o.get('rcm', {}), o.get('binary64', {})
        print('%s | %s | %s | %s | %s %s %s %s %s %s | %s | %s | %s | %s' % (
            r['model'], r['classification'], w1.get('outcome'), r.get('availability_exception', '-'),
            rep.get('rows'), rep.get('pass'), rep.get('pass_absolute_range'), rep.get('not_covered'), rep.get('fail'),
            r.get('unresolved_availability', 0), rep.get('not_covered_rows_s_full'), rcm.get('equal'),
            w1.get('counts_match_storage'), b64.get('sparse', b64.get('error'))))
    print()
    print('run | selected | charged (M LME) | W1 phase s | W1 heap MiB (in-place, move) | process heap move MiB | '
          'peak RSS MiB | peak footprint MiB | estimate MiB | rho (footprint) | wall s | load before | load after')
    for r in runs:
        o = objects(r['run_id'])
        w1 = o.get('w1', {})
        fp = (r.get('rss') or {}).get('time_peak_footprint_bytes')
        est = r.get('estimate_adm_bytes') or 0
        base = r.get('baseline_footprint_bytes') or 0
        heap = r.get('repeats_heap_peak_move') or 0
        rho = max((fp or 0) - base, heap) / est if est else None
        print('%s | %s | %.1f | %.2f | %.1f, %.1f | %.1f | %.1f | %.1f | %.0f | %.3f | %.1f | %s | %s' % (
            r['model'], w1.get('selected_precision'), (w1.get('invocation_charged') or 0) / 1e6,
            (w1.get('elapsed_ns') or 0) / 1e9, (w1.get('heap_peak') or 0) / MIB, (w1.get('heap_peak_move') or 0) / MIB,
            heap / MIB, (r.get('peak_rss_bytes') or 0) / MIB, (fp or 0) / MIB, est / MIB, rho or 0,
            r.get('wall_s') or 0, [round(x, 2) for x in r.get('load_before', [])],
            [round(x, 2) for x in r.get('load_after', [])]))
    print()
    print('attempts (precision role outcome; own / shared work, M LME; the shared stages the attempt built)')
    for r in runs:
        rec = objects(r['run_id']).get('record', {}).get('record', {})
        if r['members'] < 10000:
            continue
        print('%s: %s, charged %.1f M' % (r['model'], rec.get('outcome'), rec.get('invocation_charged', 0) / 1e6))
        for a in rec.get('attempts', []):
            shared = {k: round(v / 1e6, 1) for k, v in a['shared_stages'].items() if v} if a['shared_built_here'] else {}
            print('   %s %s %s; own %.1f, shared %.1f, verification shared %.1f; shared stages %s' % (
                a['precision'], a['role'], a['outcome'], a['own_work'] / 1e6, a['shared_work'] / 1e6,
                a['verification_shared_work'] / 1e6, shared))


if __name__ == '__main__':
    main()
