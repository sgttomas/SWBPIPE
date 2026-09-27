"""V3 step 5: D1's stop-rule floor (V1-S8) against RF-ELOAD.

Usage: python3 v3_floor.py <references_eload.json> <out v3_floor.json>
S* follows D1 DESIGN.md section 4.1.6 / 4.1.6.1 (sha256 13c1a7d5..., the worktree head): one connected
body per case; rows at restrained or prescribed DOFs are input_derived (rule 2a) and carry no threshold;
kinds translation, rotation, force, moment coupled through L_b = the node bounding-box diagonal; twist and
extension per member: S*_tw = S*(moment) L/GJ, S*_ext = S*(force) L/EA.
Two row sets bracket what the product publishes:
  A: the package's published quantities only (u, th at free DOFs, R and S components, N, T, Mb, tw, ext);
  B: A plus the element-local end shear forces and end bending-moment components (local frame from the
     member's y_reference), which the product also publishes and which enter S(force) and S(moment).
Thresholds: R = 2^-34 (D1 revision 3+, binding) and R = 2^-64/1e-9 (the brief's "about 5.4e-11").
Reports every nonzero expected value with |q| < R S*, and every comparison whose scale max(|exp|, scale)
is below R S* (for the cancellation cases, with the binding recommended scale).
"""
import json
import sys
from decimal import Decimal, getcontext
from fractions import Fraction as Fr
import v3_core as V

getcontext().prec = 80
D = Decimal
RS = {'2^-34': D(2) ** -34, '2^-64/1e-9': D(2) ** -64 / D('1e-9')}


def dq(q):
    return D(q.numerator) / D(q.denominator)


def main():
    ref = json.load(open(sys.argv[1]))
    out = {'cases': {}, 'summary': {}}
    closest, closest_cmp = {}, {}
    for case in ref['cases']:
        cid = case['id']
        basis = 'int' if case['basis'] == 'intended' else 'rep'
        pb, sol, pubd = V.run(case, basis)
        # connectivity: one body
        adj = {n: set() for n in pb.nodes}
        for m in pb.members:
            adj[m['i']].add(m['j'])
            adj[m['j']].add(m['i'])
        seen, stack = set(), [next(iter(pb.nodes))]
        while stack:
            n = stack.pop()
            if n not in seen:
                seen.add(n)
                stack.extend(adj[n])
        assert seen == set(pb.nodes), cid
        restrained = {(n, d) for n, dd in pb.restraints.items() for d in dd}
        exp = {r[0]: (D(r[1]), r[2]) for r in case['expected']}
        rec_scale = {r[0]: D(r[3]) for r in case['expected']} if 'cancellation' in case else None
        cls_scale = {k: D(v['scale']) for k, v in case['classes'].items()}
        rows = {}   # kind -> max |q|
        idrows = set()

        def put(kind, v):
            rows[kind] = max(rows.get(kind, D(0)), abs(v))
        for k, (e, c) in exp.items():
            p = k.split('.')
            if p[0] in ('u', 'th'):
                dof = p[2]
                if (p[1], dof) in restrained:
                    idrows.add(k)
                    continue
            if c in ('translation', 'rotation', 'force', 'moment'):
                put(c, e)
        rowsB = dict(rows)
        for m in pb.members:
            e = [dq(x) for x in m['e']]
            yr = [dq(x) for x in m['yref']]
            ye = sum(a * b for a, b in zip(yr, e))
            y = [a - ye * b for a, b in zip(yr, e)]
            ny = D(sum(a * a for a in y)).sqrt()
            y = [a / ny for a in y]
            z = [e[1] * y[2] - e[2] * y[1], e[2] * y[0] - e[0] * y[2], e[0] * y[1] - e[1] * y[0]]
            P = [dq(x) for x in sol.members[m['name']]['Pend']]
            dot = lambda a, b: sum(x * w for x, w in zip(a, b))
            for off in (0, 6):
                F, M = P[off:off + 3], P[off + 3:off + 6]
                for v in (dot(F, y), dot(F, z)):
                    rowsB['force'] = max(rowsB.get('force', D(0)), abs(v))
                for v in (dot(M, y), dot(M, z)):
                    rowsB['moment'] = max(rowsB.get('moment', D(0)), abs(v))
        xs = [c for c in pb.nodes.values()]
        d = [max(dq(c[a]) for c in xs) - min(dq(c[a]) for c in xs) for a in range(3)]
        Lb = D(sum(x * x for x in d)).sqrt()
        res = {'L_b': format(Lb, '.6g'), 'input_derived_rows': len(idrows)}
        for var, R_ in (('A', rows), ('B', rowsB)):
            g = lambda k: R_.get(k, D(0))
            S = {'translation': max(g('translation'), Lb * g('rotation')),
                 'rotation': max(g('rotation'), g('translation') / Lb),
                 'force': max(g('force'), g('moment') / Lb),
                 'moment': max(g('moment'), Lb * g('force'))}
            for Rn, R in RS.items():
                nz, nc = [], []
                for k, (e, c) in exp.items():
                    if k in idrows:
                        continue
                    if c in S:
                        s = S[c]
                    else:
                        m = pb.mem[k.split('.')[1]]
                        s = S['moment'] * dq(m['L'] / m['sec'].GJ) if c == 'twist' else S['force'] * dq(m['L'] / m['sec'].EA)
                    thr = R * s
                    if e != 0 and Rn == "2^-34" and s != 0:
                        r_ = abs(e) / s
                        if var not in closest or r_ < closest[var][0]:
                            closest[var] = (r_, cid, k)
                    if Rn == "2^-34" and s != 0:
                        sc_ = rec_scale[k] if rec_scale else cls_scale[c]
                        r2 = max(abs(e), sc_) / s
                        if var not in closest_cmp or r2 < closest_cmp[var][0]:
                            closest_cmp[var] = (r2, cid, k)
                    if e != 0 and abs(e) < thr:
                        nz.append((k, format(abs(e) / s, '.3e')))
                    sc = rec_scale[k] if rec_scale else cls_scale[c]
                    if max(abs(e), sc) < thr:
                        nc.append((k, format(max(abs(e), sc) / s, '.3e')))
                res['%s|%s' % (var, Rn)] = {'S*': {k: format(v, '.6e') for k, v in S.items()},
                                           'nonzero_below_floor': nz, 'comparison_not_covered': nc}
        out['cases'][cid] = res
        for key in [x for x in res if '|' in x]:
            sm = out['summary'].setdefault(key, {'nonzero_below_floor': 0, 'comparison_not_covered': 0, 'cases': []})
            sm['nonzero_below_floor'] += len(res[key]['nonzero_below_floor'])
            sm['comparison_not_covered'] += len(res[key]['comparison_not_covered'])
            if res[key]['nonzero_below_floor'] or res[key]['comparison_not_covered']:
                sm['cases'].append((cid, res[key]['nonzero_below_floor'], res[key]['comparison_not_covered']))
    out['closest_nonzero_to_Sstar'] = {k: (format(v[0], '.3e'), v[1], v[2]) for k, v in closest.items()}
    out['closest_comparison_scale_to_Sstar'] = {k: (format(v[0], '.3e'), v[1], v[2]) for k, v in closest_cmp.items()}
    print('closest nonzero |q|/S*:', out['closest_nonzero_to_Sstar'])
    print('closest comparison scale/S*:', out['closest_comparison_scale_to_Sstar'])
    json.dump(out, open(sys.argv[2], 'w'), indent=1)
    for k, v in out['summary'].items():
        print(k, 'nonzero below floor:', v['nonzero_below_floor'], ' comparisons not covered:', v['comparison_not_covered'])
        for c in v['cases']:
            print('   ', c)


if __name__ == '__main__':
    main()
