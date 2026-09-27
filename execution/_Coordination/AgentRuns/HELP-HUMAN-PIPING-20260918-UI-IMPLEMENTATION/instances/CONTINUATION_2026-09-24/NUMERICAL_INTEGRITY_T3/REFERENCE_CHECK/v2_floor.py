"""V2: D1 stop-rule floor (V1-S8) against the references.

Usage: python3 v2_floor.py <references.json> <v2_values.json> <v2_compare.json> <out.json>
R = 2^-64 / 1e-9.  S* per D1 section 4.1.6 (one connected body per case; kinds translation, rotation,
force, moment; L_b = diagonal of the node bounding box), computed from the reference values (V2's
complete solution, which equals R1's published values to 5e-40).
Variant A: twist counted as rotation and extension as translation (D1's four kinds).
Variant B: twist and extension are their own kinds with S* = their own largest magnitude.
Reports every nonzero expected value with |exp| < R*S*, and every value whose comparison scale
max(|exp|, scale) is below R*S* (published class scale; for RF-CANCEL also the recommended scale).
"""
import json
import sys
from decimal import Decimal, getcontext
getcontext().prec = 60
getcontext().Emin = -999999
D = Decimal
RF = D(2) ** -64 / D('1e-9')
KA = {'translation': 'translation', 'extension': 'translation', 'rotation': 'rotation', 'twist': 'rotation',
      'force': 'force', 'moment': 'moment'}
KB = {'translation': 'translation', 'extension': 'extension', 'rotation': 'rotation', 'twist': 'twist',
      'force': 'force', 'moment': 'moment'}


def main():
    ref = json.load(open(sys.argv[1]))['cases']
    mine = json.load(open(sys.argv[2]))
    cmpr = json.load(open(sys.argv[3]))['cases']
    out = {'by_family': {}, 'cases': {}}
    for cid, c in ref.items():
        if not c.get('expected'):
            continue
        fam = c['family']
        Lb = D(cmpr[cid]['L_b'])
        cls = {r[0]: r[2] for r in c['expected']}
        exp = {r[0]: D(r[1]) for r in c['expected']}
        rec = {r[0]: D(r[3]) for r in c['expected']} if fam == 'RF-CANCEL' else None
        scales = {k: D(v['value']) for k, v in c['scales'].items()}
        # kinds of every V2 value (complete solution; large cases infer the class from the key)
        def kcls(k):
            if k in cls:
                return cls[k].split('@')[0]
            p = k.split('.')[0]
            return {'u': 'translation', 'th': 'rotation', 'N': 'force', 'T': 'moment', 'Mb': 'moment', 'tw': 'twist',
                    'ext': 'extension'}.get(p) or ('force' if k.split('.')[-1][0] in 'UF' else 'moment')
        S = {}
        for k, v in mine[cid]['int'].items():
            b = kcls(k)
            S[b] = max(S.get(b, D(0)), abs(D(v)))
        g = lambda b: S.get(b, D(0))
        SA = {'translation': max(g('translation'), g('extension'), Lb * max(g('rotation'), g('twist'))),
              'rotation': max(g('rotation'), g('twist'), max(g('translation'), g('extension')) / Lb),
              'force': max(g('force'), g('moment') / Lb), 'moment': max(g('moment'), Lb * g('force'))}
        SB = {'translation': max(g('translation'), Lb * g('rotation')), 'rotation': max(g('rotation'), g('translation') / Lb),
              'force': SA['force'], 'moment': SA['moment'], 'twist': g('twist'), 'extension': g('extension')}
        r = {'A_nonzero_below': [], 'B_nonzero_below': [], 'A_scale_below': [], 'B_scale_below': [], 'cancel_rec_scale_below_A': []}
        for k, e in exp.items():
            b = cls[k].split('@')[0]
            for var, K, SS in (('A', KA, SA), ('B', KB, SB)):
                thr = RF * SS[K[b]]
                if e != 0 and abs(e) < thr:
                    r[var + '_nonzero_below'].append((k, cls[k], format(abs(e) / SS[K[b]], '.2e')))
                if max(abs(e), scales[cls[k]]) < thr:
                    r[var + '_scale_below'].append((k, cls[k], format(max(abs(e), scales[cls[k]]) / SS[K[b]], '.2e')))
            if rec is not None and max(abs(e), rec[k]) < RF * SA[KA[b]]:
                r['cancel_rec_scale_below_A'].append((k, format(max(abs(e), rec[k]) / SA[KA[b]], '.2e')))
        out['cases'][cid] = r
        F = out['by_family'].setdefault(fam, {x: 0 for x in r})
        for x in r:
            F[x] += len(r[x])
    json.dump(out, open(sys.argv[4], 'w'), indent=1)
    for f, F in out['by_family'].items():
        print(f, F)


if __name__ == '__main__':
    main()
