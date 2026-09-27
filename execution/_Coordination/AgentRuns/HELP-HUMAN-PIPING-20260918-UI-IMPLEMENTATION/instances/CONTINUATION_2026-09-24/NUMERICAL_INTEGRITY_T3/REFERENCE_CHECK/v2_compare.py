"""V2 step 2: compare V2's independent values with R1's published references (run after v2_derive.py).

Usage: python3 v2_compare.py <references.json> <v2_values.json> <out.json>
Checks, per case:
  A. value agreement: m = |v2 - r1| / max(|r1|, class scale) (agreement when m <= 1e-30);
  B. key sets (R1's keys must be V2 keys; complete cases must match exactly);
  C. class scales re-derived from V2's values by R1's stated rule (README section 4);
  D. finite-input metric and basis decision recomputed from V2's represented-input solve;
     represented-basis expectations compared with V2's represented values;
  E. D1 floor (V1-S8): S* per D1 section 4.1.6 from the reference values; flags every quantity whose
     comparison scale max(|exp|, scale) is below R*S*, and every nonzero |exp| below R*S*;
  F. every negative control that publishes values: re-evaluated against the criterion with R1's
     expected values and scales (for RF-CANCEL also with the recommended and gross scales).
RF-CANCEL scales, governed_by and negative controls are handled in v2_cancel.py.
"""
import json
import re
import sys
from decimal import Decimal, getcontext

getcontext().prec = 80
getcontext().Emax = 999999
getcontext().Emin = -999999
D = Decimal
TOL = D('1e-30')
R_FLOOR = D(2) ** -64 / D('1e-9')
KIND = {'translation': 'translation', 'extension': 'translation', 'rotation': 'rotation', 'twist': 'rotation',
        'force': 'force', 'moment': 'moment'}


def dd(s):
    return D(s)


def vec_groups(vals):
    """R1's non-empty class rule: nodal translations, rotations and support actions as vector norms,
    member values as absolute values."""
    groups = {}
    for k, v in vals.items():
        p = k.split('.')
        if p[0] in ('u', 'th'):
            g = (p[0], p[1])
        elif p[0] == 'R':
            g = ('R', p[1], 'F' if p[2][0] == 'U' else 'M')
        elif p[0] == 'S':
            g = ('S', p[1], p[2], p[3][0])
        else:
            g = (k,)
        groups.setdefault(g, []).append(v)
    return {g: sum(x * x for x in vs).sqrt() for g, vs in groups.items()}


def node_extent(model):
    xs = [[D(eval_num(c)) for c in xyz] for xyz in model['nodes_m'].values()]
    lo = [min(x[i] for x in xs) for i in range(3)]
    hi = [max(x[i] for x in xs) for i in range(3)]
    return sum((hi[i] - lo[i]) ** 2 for i in range(3)).sqrt()


def eval_num(s):
    from fractions import Fraction
    m = re.fullmatch(r'(.+)\*2\^(-?\d+)', s)
    if m:
        q = Fraction(m.group(1)) * Fraction(2) ** int(m.group(2))
    else:
        q = Fraction(s)
    return D(q.numerator) / D(q.denominator)


def main():
    ref = json.load(open(sys.argv[1]))
    mine = json.load(open(sys.argv[2]))
    import v2_models
    models = v2_models.load_cases(sys.argv[1])
    out = {'cases': {}, 'family': {}}
    for cid, c in ref['cases'].items():
        if 'expected' not in c or c['expected'] is None:
            continue
        fam = c['family']
        F = out['family'].setdefault(fam, {'cases': 0, 'values': 0, 'agree': 0, 'max_m': D(0), 'key_issues': 0,
                                           'scale_mismatch': 0, 'finite_basis_disagree': 0,
                                           'rep_values': 0, 'rep_agree': 0, 'floor_scale_flags': 0,
                                           'floor_nonzero_flags': 0, 'nc_checked': 0, 'nc_claim_mismatch': 0})
        F['cases'] += 1
        rec = {}
        scales = {k: dd(v['value']) for k, v in c['scales'].items()}
        v2 = {k: dd(v) for k, v in mine[cid]['int'].items()}
        exp = {}
        cls = {}
        for row in c['expected']:
            exp[row[0]] = dd(row[1])
            cls[row[0]] = row[2]
        # A. values
        worst = (D(0), None)
        missing = [k for k in exp if k not in v2]
        n_agree = 0
        for k, e in exp.items():
            if k not in v2:
                continue
            m = abs(v2[k] - e) / max(abs(e), scales[cls[k]])
            if m <= TOL:
                n_agree += 1
            if m > worst[0]:
                worst = (m, k)
        F['values'] += len(exp)
        F['agree'] += n_agree
        F['max_m'] = max(F['max_m'], worst[0])
        rec['values'] = len(exp)
        rec['agree'] = n_agree
        rec['max_m'] = format(worst[0], '.3e')
        rec['max_m_at'] = worst[1]
        # B. key sets
        subset = c.get('published_subset') is not None
        extra = [k for k in v2 if k not in exp] if not subset else []
        rec['missing_in_v2'] = missing
        rec['unpublished_v2_keys'] = extra[:20]
        if missing or extra:
            F['key_issues'] += 1
        # C. class scales from V2's values (complete solution, including unpublished large-case values)
        byclass = {}
        # classify V2 keys with R1's class labels (all keys of a complete case are published)
        keycls = dict(cls)
        if subset:
            # large cases: infer class from key type (no regions in RF-LARGE)
            for k in v2:
                if k not in keycls:
                    p = k.split('.')[0]
                    keycls[k] = {'u': 'translation', 'th': 'rotation', 'N': 'force', 'T': 'moment', 'Mb': 'moment',
                                 'tw': 'twist', 'ext': 'extension'}.get(p) or (
                        'force' if (p in ('R', 'S') and (k.split('.')[-1][0] in 'UF')) else 'moment')
        sumsq = {}
        for k, v in v2.items():
            cl = keycls.get(k)
            if cl is None:
                continue
            p = k.split('.')
            if p[0] in ('u', 'th'):
                g = (p[0], p[1])
            elif p[0] == 'R':
                g = ('SUP', p[1], 'F' if p[2][0] == 'U' else 'M', p[2][1])
            elif p[0] == 'S':
                g = ('SUP', p[1], p[3][0], p[3][1])
            else:
                g = (k,)
            if g[0] == 'SUP':
                # support action per node: rigid-restraint and spring actions summed as one vector (R1's rule
                # read as the total action of the node's supports), component-wise first
                comp = sumsq.setdefault((('SUPV', g[1], g[2]), cl), {})
                comp[g[3]] = comp.get(g[3], D(0)) + v
                continue
            sumsq[(g, cl)] = sumsq.get((g, cl), D(0)) + v * v
        for (g, cl), s2 in sumsq.items():
            if isinstance(s2, dict):
                s2 = sum(x * x for x in s2.values())
            byclass[cl] = max(byclass.get(cl, D(0)), s2.sqrt())
        scale_rows = []
        for cl, sv in scales.items():
            nz = byclass.get(cl, D(0))
            nonzero_in_r1 = any(e != 0 for k, e in exp.items() if cls[k] == cl)
            if nonzero_in_r1:
                ok = abs(nz - sv) <= D('1e-20') * sv
                scale_rows.append((cl, 'largest magnitude', format(sv, '.6e'), format(nz, '.6e'), ok))
                if not ok:
                    F['scale_mismatch'] += 1
            else:
                scale_rows.append((cl, 'all zero (derived)', format(sv, '.6e'), format(nz, '.3e'), nz <= D('1e-40') * sv))
        rec['scales'] = scale_rows
        # D. finite input
        if 'rep' in mine[cid]:
            rep = {k: dd(v) for k, v in mine[cid]['rep'].items()}
            fm = (D(0), None)
            for k, e in exp.items():
                if k in rep and k in v2:
                    m = abs(rep[k] - v2[k]) / max(abs(v2[k]), scales[cls[k]])
                    if m > fm[0]:
                        fm = (m, k)
            r1m = dd(c['finite_input']['max_normalized_difference']) if c.get('finite_input') and \
                c['finite_input'].get('max_normalized_difference') not in (None, '') else None
            rec['finite_v2'] = (format(fm[0], '.5e'), fm[1])
            rec['finite_r1'] = (c['finite_input'].get('max_normalized_difference'), c['finite_input'].get('at')) \
                if c.get('finite_input') else None
            v2_basis = 'represented' if fm[0] > D('1e-9') else 'intended'
            rec['basis_v2'] = v2_basis
            rec['basis_r1'] = c['basis']
            if v2_basis != c['basis']:
                F['finite_basis_disagree'] += 1
            if 'expected_represented' in c:
                wr = (D(0), None)
                nr = 0
                for row in c['expected_represented']:
                    k, e = row[0], dd(row[1])
                    m = abs(rep[k] - e) / max(abs(e), scales[row[2]])
                    nr += m <= TOL
                    if m > wr[0]:
                        wr = (m, k)
                F['rep_values'] += len(c['expected_represented'])
                F['rep_agree'] += nr
                rec['represented_check'] = {'values': len(c['expected_represented']), 'agree': nr,
                                            'max_m': format(wr[0], '.3e'), 'at': wr[1]}
        # E. D1 floor: S* per D1 4.1.6 from the reference values (V2's complete solution)
        Lb = node_extent(models[cid]['model'])
        Sk = {'translation': D(0), 'rotation': D(0), 'force': D(0), 'moment': D(0)}
        for k, v in v2.items():
            cl = keycls.get(k)
            if cl is None:
                continue
            kind = KIND[cl.split('@')[0]]
            Sk[kind] = max(Sk[kind], abs(v))
        Sstar = {'translation': max(Sk['translation'], Lb * Sk['rotation']),
                 'rotation': max(Sk['rotation'], Sk['translation'] / Lb if Lb else D(0)),
                 'force': max(Sk['force'], Sk['moment'] / Lb if Lb else D(0)),
                 'moment': max(Sk['moment'], Lb * Sk['force'])}
        fl_scale, fl_nz = [], []
        for k, e in exp.items():
            kind = KIND[cls[k].split('@')[0]]
            thr = R_FLOOR * Sstar[kind]
            if max(abs(e), scales[cls[k]]) < thr:
                fl_scale.append((k, format(max(abs(e), scales[cls[k]]) / Sstar[kind], '.3e')))
            if e != 0 and abs(e) < thr:
                fl_nz.append((k, format((abs(e) / Sstar[kind]), '.3e'), cls[k]))
        rec['S_star'] = {k: format(v, '.4e') for k, v in Sstar.items()}
        rec['L_b'] = format(Lb, '.4e')
        rec['floor_scale_flags'] = fl_scale
        rec['floor_nonzero_flags'] = fl_nz
        F['floor_scale_flags'] += len(fl_scale)
        F['floor_nonzero_flags'] += len(fl_nz)
        # F. negative controls that publish values
        ncs = []
        for nc in c['negative_controls']:
            if 'values' not in nc:
                continue
            vals = json.loads(nc['values']) if isinstance(nc['values'], str) else nc['values']
            viol = []
            for k, s in vals.items():
                e = exp[k]
                lim = D('1e-9') * max(abs(e), scales[cls[k]])
                r = abs(dd(s) - e) / max(abs(e), scales[cls[k]])
                if abs(dd(s) - e) > lim:
                    viol.append((k, r))
            claimed = bool(nc.get('discriminates'))
            actual = len(viol) > 0 if fam != 'RF-CANCEL' else None
            mx = max((r for _, r in viol), default=D(0))
            entry = {'id': nc['id'], 'claimed': claimed, 'listed_values': len(vals), 'violating_under_class_scale': len(viol),
                     'max_ratio_x1e9': format((mx * D('1e9')), '.4e') if viol else '0',
                     'r1_max_violation_ratio': nc.get('max_violation_ratio'), 'r1_violating_values': nc.get('violating_values')}
            if fam != 'RF-CANCEL':
                F['nc_checked'] += 1
                # R1 lists at most 24 violating values; every listed value must violate, the listed maximum
                # must reproduce R1's max_violation_ratio, and a non-discriminating control lists none
                r1max = D(nc['max_violation_ratio']) if nc.get('max_violation_ratio') not in (None, '0') else D(0)
                bad = (claimed and (len(viol) != len(vals) or len(vals) == 0 or
                                    abs(mx * D('1e9') - r1max) > D('1e-5') * r1max)) or (not claimed and len(vals) > 0)
                if bad:
                    F['nc_claim_mismatch'] += 1
                    entry['MISMATCH'] = True
            ncs.append(entry)
        rec['negative_controls'] = ncs
        out['cases'][cid] = rec
    for f, F in out['family'].items():
        F['max_m'] = format(F['max_m'], '.3e')
    json.dump(out, open(sys.argv[3], 'w'), indent=1, default=str)
    print('%-14s %5s %7s %7s %10s %4s %5s %5s %9s %6s %6s %6s' % ('family', 'cases', 'values', 'agree', 'max_m', 'keys',
                                                            'scale', 'basis', 'rep', 'flS', 'flNZ', 'ncMis'))
    for f, F in out['family'].items():
        print('%-14s %5d %7d %7d %10s %4d %5d %5d %4d/%-4d %6d %6d %3d/%d' % (
            f, F['cases'], F['values'], F['agree'], F['max_m'], F['key_issues'], F['scale_mismatch'],
            F['finite_basis_disagree'], F['rep_agree'], F['rep_values'], F['floor_scale_flags'],
            F['floor_nonzero_flags'], F['nc_claim_mismatch'], F['nc_checked']))


if __name__ == '__main__':
    main()
