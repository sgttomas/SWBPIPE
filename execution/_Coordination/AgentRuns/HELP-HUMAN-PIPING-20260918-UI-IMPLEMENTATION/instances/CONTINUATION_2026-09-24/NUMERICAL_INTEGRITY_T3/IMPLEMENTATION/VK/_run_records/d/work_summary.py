#!/usr/bin/env python3
"""V-K D: the per-family summary of the committed kernel-lane records, for
RETURN's "Interface for ROOT's W1 limits, V-P and F2a" (Scope 11).

Standard library only; it reads `VR/observations/kernel_lane/rf_*.json` (the
committed per-case records, `vk-case-record-v1`) and prints, per family:
- the outcomes and the selected precisions;
- the case's charged work (`invocation_charged`, limb-multiply equivalents):
  minimum, median, maximum and the case at the maximum;
- the work by precision and stage: for each attempt precision p, a stage's
  work is the attempt's own `stages` plus its `shared_stages` where the
  attempt built the shared state (`shared_built_here`); per (p, stage), the
  maximum over the family's cases and the case at the maximum.
It also writes the same as JSON (`work_summary.json`) next to itself.

Usage: python3 work_summary.py <VR dir>
"""
import json
import os
import statistics
import sys
from collections import Counter, defaultdict

FAMILIES = ['rf_chain', 'rf_skew', 'rf_weak', 'rf_large', 'rf_invariance', 'rf_range', 'rf_zero',
            'rf_finite', 'rf_mech', 'rf_cancel']


def case_stage_work(rec):
    out = defaultdict(lambda: defaultdict(int))
    for a in rec.get('attempts', []):
        p = a['precision']
        for k, v in a['stages'].items():
            out[p][k] += v
        if a.get('shared_built_here'):
            for k, v in a['shared_stages'].items():
                out[p][k] += v
    return out


def main(vr):
    summary = {}
    lines = []
    for fam in FAMILIES:
        recs = json.load(open(os.path.join(vr, 'observations/kernel_lane', fam + '.json')))
        outcomes = Counter(r['outcome'].split(' [')[0].split(' {')[0] for r in recs)
        selected = Counter(r.get('selected_precision') for r in recs if r.get('selected_precision'))
        charged = sorted((r.get('invocation_charged', 0), r['id']) for r in recs)
        stage_max = {}
        for r in recs:
            for p, stages in case_stage_work(r).items():
                for k, v in stages.items():
                    key = '%d/%s' % (p, k)
                    if v and (key not in stage_max or v > stage_max[key][0]):
                        stage_max[key] = (v, r['id'])
        per_p_total = defaultdict(list)
        for r in recs:
            for p, stages in case_stage_work(r).items():
                per_p_total[p].append((sum(stages.values()), r['id']))
        fam_out = {
            'cases': len(recs),
            'outcomes': dict(sorted(outcomes.items())),
            'selected_precision': {str(k): v for k, v in sorted(selected.items())},
            'charged': {'min': charged[0][0], 'median': statistics.median(c for c, _ in charged),
                        'max': charged[-1][0], 'at_max': charged[-1][1]},
            'work_by_precision': {str(p): {'cases': len(v), 'max': max(v)[0], 'at_max': max(v)[1],
                                           'median': statistics.median(x for x, _ in v)}
                                  for p, v in sorted(per_p_total.items())},
            'stage_max': {k: {'max': v[0], 'at_max': v[1]} for k, v in sorted(
                stage_max.items(), key=lambda kv: (int(kv[0].split('/')[0]), -kv[1][0]))},
        }
        summary[fam] = fam_out
        lines.append('== %s: %d cases; outcomes %s; selected %s' % (
            fam, len(recs), fam_out['outcomes'], fam_out['selected_precision']))
        c = fam_out['charged']
        lines.append('   charged work: min %d, median %s, max %d (%s)' % (c['min'], c['median'], c['max'], c['at_max']))
        for p, w in fam_out['work_by_precision'].items():
            top = [(k.split('/')[1], v['max'], v['at_max']) for k, v in fam_out['stage_max'].items()
                   if k.startswith(p + '/')][:4]
            lines.append('   p=%s: %d cases, work median %s, max %d (%s); largest stages %s' % (
                p, w['cases'], w['median'], w['max'], w['at_max'],
                ', '.join('%s %d (%s)' % t for t in top)))
    here = os.path.dirname(os.path.abspath(__file__))
    with open(os.path.join(here, 'work_summary.json'), 'w') as fh:
        json.dump(summary, fh, indent=1, sort_keys=True)
        fh.write('\n')
    print('\n'.join(lines))


if __name__ == '__main__':
    main(sys.argv[1])
