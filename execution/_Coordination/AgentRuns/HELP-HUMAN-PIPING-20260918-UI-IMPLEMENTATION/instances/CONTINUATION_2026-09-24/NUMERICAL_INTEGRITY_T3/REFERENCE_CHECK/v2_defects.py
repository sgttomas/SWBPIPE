"""V2: plausible implementation defects that R1 did not list, and whether some reference catches each.

Usage: python3 v2_defects.py <references.json> <v2_values.json> <out.json>
A defect is caught by a case when at least one published value fails
|obs - exp| <= 1e-9 max(|exp|, scale) (R1's expected values and class scales), or when the defect
produces a refusal, a non-finite value or a missing value for a case that must be solved.
Defects:
  X-CHOP      every published |value| < 1e-12 (case units) written as 0 (output clean-up)   [all families]
  X-BINARY64  plain binary64 assembly and elimination, load contributions summed left to right
              in binary64 in authored order                                              [all families]
  X-J-EQ-I    polar constant taken as I instead of 2I (GJ halved)                          [RF-CHAIN]
  X-UNNORM    spring direction used without normalization (k scaled by |d|^2)            [RF-SKEW]
  X-GJC0      coupling member torsion dropped (G of the coupling section set to 0)       [RF-WEAK]
  X-PIN-AS-FIX translation-pinned supports also restrain rotation                         [RF-LARGE CONT]
  X-ONE-LOAD  one load record per node: the moment replaces the force at a node with both [RF-LARGE CHAIN]
  X-COORD32   node coordinates stored in binary32                               [RF-INVARIANCE, RF-FINITE]
  X-SEC64     EA, EI, GJ formed as binary64 products (IEEE overflow/underflow)            [RF-RANGE]
  X-NUMRANK   mechanism test: a pivot below 1e-12 of the largest stiffness diagonal is taken as
              a zero pivot and the model is refused                                        [all stable cases]
  X-OVERWRITE load contributions on one DOF assigned instead of accumulated (last wins);
              for the spans, span B's equivalent moment overwrites span A's at S1          [RF-CANCEL]
"""
import json
import math
import sys
from decimal import Decimal
from fractions import Fraction as Fr

import v2_frame as vf
from v2_models import load_cases

D = Decimal
RD = vf.Num('dec')
RF64 = vf.Num('float')


def judge(exp, cls, scales, vals):
    n, worst = 0, D(0)
    for k, e in exp.items():
        if k not in vals:
            return {'caught': True, 'why': 'value missing'}
        v = vals[k]
        if isinstance(v, float) and not math.isfinite(v):
            return {'caught': True, 'why': 'non-finite value'}
        s = max(abs(e), scales[cls[k]])
        r = abs(D(v) - e) / s
        if r > D('1e-9'):
            n += 1
        worst = max(worst, r)
    return {'caught': n > 0, 'violations': n, 'max_ratio_x1e9': format(worst * D('1e9'), '.4e')}


def solve_or_refuse(model, R, **kw):
    try:
        out = vf.solve(model, R, **kw)
        return {k: v for k, v in out.items() if not k.startswith('_')}
    except (vf.Singular, ZeroDivisionError, OverflowError, ValueError, ArithmeticError) as ex:
        return {'__refused__': type(ex).__name__ + ': ' + str(ex)[:80]}


def model_f64_loads(mj):
    """Load model with contributions summed left to right in binary64 (authored order)."""
    m = vf.load_model({k: v for k, v in mj.items() if k != 'load_contributions_in_authored_order'})
    acc = {}
    for (n, dof, v) in mj.get('load_contributions_in_authored_order', []):
        key = (n, dof)
        acc[key] = float(vf.parse(v)) if key not in acc else acc[key] + float(vf.parse(v))
    for (n, dof), s in acc.items():
        FM = m['loads'].setdefault(n, [(Fr(0),) * 3, (Fr(0),) * 3])
        k = vf.DOFS.index(dof)
        vec = list(FM[k // 3])
        vec[k % 3] += Fr(s)
        FM[k // 3] = tuple(vec)
    return m


def main():
    ref = json.load(open(sys.argv[1]))['cases']
    mine = json.load(open(sys.argv[2]))
    cases = load_cases(sys.argv[1])
    out = {}
    for cid, c in ref.items():
        if 'expected' not in c or c['expected'] is None:
            continue
        fam = c['family']
        mj = cases[cid]['model']
        exp = {r[0]: D(r[1]) for r in c['expected']}
        cls = {r[0]: r[2] for r in c['expected']}
        scales = {k: D(v['value']) for k, v in c['scales'].items()}
        v2 = {k: D(v) for k, v in mine[cid]['int'].items()}
        res = {}

        def run(tag, vals):
            if '__refused__' in vals:
                res[tag] = {'caught': True, 'why': 'refused: ' + vals['__refused__']}
            else:
                res[tag] = judge(exp, cls, scales, vals)

        # X-CHOP
        run('X-CHOP', {k: (D(0) if abs(v) < D('1e-12') else v) for k, v in v2.items()})
        # X-BINARY64
        run('X-BINARY64', solve_or_refuse(model_f64_loads(mj), RF64))
        base = vf.load_model(mj)
        if fam == 'RF-CHAIN':
            m = dict(base)
            m['sections'] = {k: dict(s, G=s['G'] / 2) for k, s in base['sections'].items()}
            run('X-J-EQ-I', solve_or_refuse(m, RD))
        if fam == 'RF-SKEW':
            m = dict(base)
            m['supports'] = {n: {'rigid': s['rigid'], 'springs': [(kd, d, k * sum(x * x for x in d)) for (kd, d, k) in s['springs']]}
                             for n, s in base['supports'].items()}
            run('X-UNNORM', solve_or_refuse(m, RD))
        if fam == 'RF-WEAK' and 'C' in base['sections']:
            m = dict(base)
            m['sections'] = dict(base['sections'])
            m['sections']['C'] = dict(base['sections']['C'], G=Fr(0))
            run('X-GJC0', solve_or_refuse(m, RD))
        if fam == 'RF-LARGE' and '-CONT-' in cid and len(base['members']) <= 1000:
            m = dict(base)
            m['supports'] = {n: {'rigid': sorted(set(s['rigid']) | {'RX', 'RY', 'RZ'}, key=vf.DOFS.index),
                                 'springs': s['springs']} for n, s in base['supports'].items()}
            vals = solve_or_refuse(m, RD)
            # the defective model publishes reactions at the extra restraints; judge the common keys only
            run('X-PIN-AS-FIX', {k: v for k, v in vals.items() if k in exp or k.startswith('__')})
        if fam == 'RF-LARGE' and '-CHAIN-' in cid and len(base['members']) <= 1000:
            m = dict(base)
            m['loads'] = {n: ([(Fr(0),) * 3, M] if any(M) else [F, M]) for n, (F, M) in base['loads'].items()}
            run('X-ONE-LOAD', solve_or_refuse(m, RD))
        if fam in ('RF-INVARIANCE', 'RF-FINITE') and len(base['members']) <= 1000:
            import struct
            f32 = lambda q: Fr(struct.unpack('f', struct.pack('f', float(q)))[0])
            m = dict(base)
            m['nodes'] = {n: tuple(f32(x) for x in xyz) for n, xyz in base['nodes'].items()}
            run('X-COORD32', solve_or_refuse(m, RD))
        if fam == 'RF-RANGE':
            bad = []
            for sname, s in base['sections'].items():
                E = float(s['E'])
                G = float(s['G']) if 'G' in s else E / (2 * (1 + float(s['nu'])))
                OD, ID = float(s['OD']), float(s['ID'])
                A = math.pi / 4 * (OD * OD - ID * ID)
                I = math.pi / 64 * (OD ** 4 - ID ** 4)
                for lab, v in (('EA', E * A), ('EI', E * I), ('GJ', G * 2 * I)):
                    if v == 0 or not math.isfinite(v) or abs(v) < 2.2250738585072014e-308:
                        bad.append('%s.%s=%r' % (sname, lab, v))
            res['X-SEC64'] = {'caught': bool(bad), 'why': bad} if bad else {
                'caught': False, 'why': 'every section product is a normal binary64 number (error ~1e-16, not a 1e-9 defect)'}
        # X-NUMRANK: would a numerical-rank mechanism test refuse this stable model?
        if len(base['members']) <= 1000:
            try:
                vf.solve(base, RD, singular_global=D('1e-12'))
                res['X-NUMRANK'] = {'caught': False, 'why': 'no pivot below 1e-12 of the largest diagonal'}
            except vf.Singular as ex:
                res['X-NUMRANK'] = {'caught': True, 'why': 'stable model refused: ' + str(ex)}
        if fam == 'RF-CANCEL':
            if 'load_contributions_in_authored_order' in mj:
                m = vf.load_model({k: v for k, v in mj.items() if k != 'load_contributions_in_authored_order'})
                last = mj['load_contributions_in_authored_order'][-1]
                n, dof, v = last[0], last[1], vf.parse(last[2])
                FM = m['loads'].setdefault(n, [(Fr(0),) * 3, (Fr(0),) * 3])
                k = vf.DOFS.index(dof)
                vec = list(FM[k // 3])
                vec[k % 3] += v
                FM[k // 3] = tuple(vec)
                run('X-OVERWRITE', solve_or_refuse(m, RD))
            else:
                L = Fr(2)
                wA = vf.parse(mj['member_uniform_loads_N_per_m_global']['A'][1])
                # S1 RZ receives span B's equivalent moment only: remove span A's -wA L^2/12 contribution
                m = dict(base)
                loads = {nn: [tuple(F), tuple(M)] for nn, (F, M) in base['loads'].items()}
                F, M = loads['S1']
                loads['S1'] = [F, (M[0], M[1], M[2] + wA * L * L / 12)]
                m['loads'] = loads
                run('X-OVERWRITE', solve_or_refuse(m, RD))
        out[cid] = {'family': fam, 'defects': res}
        print(cid, ' '.join('%s:%s' % (k, 'C' if v['caught'] else '-') for k, v in res.items()), flush=True)
    # summary per family and defect
    summ = {}
    for cid, r in out.items():
        for dname, v in r['defects'].items():
            s = summ.setdefault(r['family'], {}).setdefault(dname, {'cases': 0, 'caught': 0})
            s['cases'] += 1
            s['caught'] += bool(v['caught'])
    out['_summary'] = summ
    json.dump(out, open(sys.argv[3], 'w'), indent=1, default=str)
    print(json.dumps(summ, indent=1))


if __name__ == '__main__':
    main()
