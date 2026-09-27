"""V2: independent recomputation of R1's negative controls whose outcome depends on a defect model.

Usage: python3 v2_nc.py <references.json> <v2_values.json> <out.json>
Recomputed from V2's own solutions and defect models (R1's control values are not used as inputs):
  NC-LOST-SOFT (RF-CHAIN, RF-SKEW, RF-WEAK), NC-SUBTRACT-ROUNDED (every family that lists it),
  NC-STORED-ASSEMBLY (RF-CHAIN), NC-WRONG-TRANSFORM (RF-SKEW, RF-WEAK), NC-ORIGIN-MOMENTS and
  NC-UNSWAPPED-ENDS where R1 marks them non-discriminating, NC-ROTATION-SCALED-AS-TRANSLATION (pl = 0).
Each control is judged with R1's expected values and class scales under
|obs - exp| <= 1e-9 max(|exp|, scale), and the outcome is compared with R1's `discriminates` flag.
"""
import json
import sys
from decimal import Decimal, getcontext
from fractions import Fraction as Fr

import v2_frame as vf
from v2_models import load_cases

D = Decimal
R = vf.Num('dec')


def f64(q):
    return Fr(float(Fr(q)))


def f64add(a, b):
    """binary64 a + b of two binary64 values, as an exact Fraction."""
    return Fr(float(a) + float(b))


def judge(exp, cls, scales, vals):
    n, worst, at = 0, D(0), None
    for k, v in vals.items():
        if k not in exp:
            continue
        e = exp[k]
        s = max(abs(e), scales[cls[k]])
        r = abs(D(v) - e) / s
        if r > D('1e-9'):
            n += 1
        if r > worst:
            worst, at = r, k
    return {'discriminates': n > 0, 'violations': n, 'max_ratio_x1e9': format(worst * D('1e9'), '.5e'), 'at': at}


def fr_of(d):
    """Decimal (from the solve) -> Fraction, exactly."""
    return Fr(d)


def solve_vals(model):
    return {k: v for k, v in vf.solve(model, R).items() if not k.startswith('_')}


def member_geom(model):
    props = {k: vf.section_props(s, R) for k, s in model['sections'].items()}
    out = {}
    for mem in model['members']:
        name, a, b, sec = mem
        xa, xb = model['nodes'][a], model['nodes'][b]
        d = tuple(xb[i] - xa[i] for i in range(3))
        L2 = sum(c * c for c in d)
        out[name] = (a, b, d, L2, props[sec])
    return out


def nc_subtract_rounded(model, v2):
    """tw, ext from binary64-rounded global values (exact values rounded once), T = GJ tw / L, N = EA ext / L."""
    vals = {}
    for name, (a, b, d, L2, (EA, EI, GJ)) in member_geom(model).items():
        L = R.sqrt(R.conv(L2))
        e = tuple(R.conv(c) / L for c in d)
        th = lambda n: [R.conv(f64(Fr(v2['th.%s.%s' % (n, x)]))) for x in ('RX', 'RY', 'RZ')]
        uu = lambda n: [R.conv(f64(Fr(v2['u.%s.%s' % (n, x)]))) for x in ('UX', 'UY', 'UZ')]
        tw = sum((th(b)[i] - th(a)[i]) * e[i] for i in range(3))
        ext = sum((uu(b)[i] - uu(a)[i]) * e[i] for i in range(3))
        vals['tw.%s' % name] = tw
        vals['ext.%s' % name] = ext
        vals['T.%s' % name] = GJ * tw / L
        vals['N.%s' % name] = EA * ext / L
    return vals


def soft_spring(model):
    """(node, index, kind, direction, k) of the soft spring (the smallest k)."""
    best = None
    for n, s in model['supports'].items():
        for i, (kind, d, k) in enumerate(s['springs']):
            if best is None or k < best[4]:
                best = (n, i, kind, d, k)
    return best


def kdd_in_direction(model, node, kind, d):
    """Diagonal stiffness of the member at `node` in the spring direction d (R1's defect model)."""
    for name, (a, b, dv, L2, (EA, EI, GJ)) in member_geom(model).items():
        if node in (a, b):
            L = R.sqrt(R.conv(L2))
            dd = sum(c * c for c in d)
            c2 = R.conv(sum(dv[i] * d[i] for i in range(3)) ** 2 / (L2 * dd))
            if kind == 'rotation':
                return GJ / L * c2 + 4 * EI / L * (1 - c2)
            return EA / L * c2 + 12 * EI / L ** 3 * (1 - c2)


def with_spring_k(model, node, idx, k):
    m = dict(model)
    sups = {n: {'rigid': s['rigid'], 'springs': list(s['springs'])} for n, s in model['supports'].items()}
    kind, d, _ = sups[node]['springs'][idx]
    sups[node]['springs'][idx] = (kind, d, k)
    m['supports'] = sups
    return m


def nc_lost_soft(cid, model, ref_case):
    fam = ref_case['family']
    if fam == 'RF-WEAK' and ('W-AX' in cid or 'W-3D' in cid):
        # coupling axial stiffness added once into the adjacent diagonal; retention applied to E and G of C
        geo = member_geom(model)
        a, b, d, L2, (EA, EI, GJ) = geo['C']
        kc = EA / R.sqrt(R.conv(L2))
        if 'W-AX' in cid:
            a2 = geo['A2']
            kdd = a2[4][0] / R.sqrt(R.conv(a2[3]))
        else:
            a12 = geo['A12']
            kdd = 12 * a12[4][1] / R.sqrt(R.conv(a12[3])) ** 3
        K, k = Fr(float(kdd)), Fr(float(kc))
        r = (f64add(K, k) - K) / k
        m = dict(model)
        secs = dict(model['sections'])
        secs['C'] = dict(secs['C'])
        secs['C']['E'] = secs['C']['E'] * r
        secs['C']['G'] = secs['C']['G'] * r
        m['sections'] = secs
        return solve_vals(m), {'retention_factor': format(D(r.numerator) / D(r.denominator), '.11e'),
                               'kdd': format(kdd, '.10e'), 'kc': format(kc, '.10e')}
    node, idx, kind, d, k = soft_spring(model)
    kdd = kdd_in_direction(model, node, kind, d)
    K = Fr(float(kdd))
    keff = f64add(K, f64(k)) - K
    return solve_vals(with_spring_k(model, node, idx, keff)), {
        'kdd': format(kdd, '.16e'), 'k_eff': format(D(keff.numerator) / D(keff.denominator), '.16e'),
        'relative_change_of_k': format((D(keff.numerator) / D(keff.denominator) - D(k.numerator) / D(k.denominator)) /
                                       (D(k.numerator) / D(k.denominator)), '.5e')}


def nc_stored_chain(model):
    """RF-CHAIN: exact solution of the binary64-stored tridiagonal (torsion or axial) chain matrix."""
    node, idx, kind, d, k = soft_spring(model)
    geo = member_geom(model)
    names = [m[0] for m in model['members']]
    nodes = [model['members'][0][1]] + [m[2] for m in model['members']]
    coef = []
    for nm in names:
        a, b, dv, L2, (EA, EI, GJ) = geo[nm]
        L = R.sqrt(R.conv(L2))
        coef.append(Fr(float((GJ if kind == 'rotation' else EA) / L)))
    n = len(nodes)
    diag = [Fr(0)] * n
    diag[0] = f64add(coef[0], f64(k))
    for i in range(1, n - 1):
        diag[i] = f64add(coef[i - 1], coef[i])
    diag[n - 1] = coef[n - 2]
    tip = nodes[-1]
    F, M = model['loads'][tip]
    load = f64(M[0] if kind == 'rotation' else F[0])
    # tridiagonal solve (exact)
    b = [Fr(0)] * n
    b[-1] = load
    c = [-x for x in coef]      # off-diagonal (i, i+1)
    dd = list(diag)
    bb = list(b)
    for i in range(1, n):
        w = c[i - 1] / dd[i - 1]
        dd[i] -= w * c[i - 1]
        bb[i] -= w * bb[i - 1]
    x = [Fr(0)] * n
    x[-1] = bb[-1] / dd[-1]
    for i in range(n - 2, -1, -1):
        x[i] = (bb[i] - c[i] * x[i + 1]) / dd[i]
    vals = {}
    lab = 'th' if kind == 'rotation' else 'u'
    comp = 'RX' if kind == 'rotation' else 'UX'
    for nd, xv in zip(nodes, x):
        vals['%s.%s.%s' % (lab, nd, comp)] = R.conv(xv)
    for i, nm in enumerate(names):
        a, b2, dv, L2, (EA, EI, GJ) = geo[nm]
        L = R.sqrt(R.conv(L2))
        rel = R.conv(x[i + 1] - x[i])
        if kind == 'rotation':
            vals['tw.%s' % nm] = rel
            vals['T.%s' % nm] = GJ * rel / L
        else:
            vals['ext.%s' % nm] = rel
            vals['N.%s' % nm] = EA * rel / L
    vals['S.%s.%d.%s' % (node, idx, 'MX' if kind == 'rotation' else 'FX')] = -R.conv(f64(k) * x[0])
    return vals


def nc_wrong_transform(model, member):
    geo = member_geom(model)
    a, b, dv, L2, _ = geo[member]
    L = R.sqrt(R.conv(L2))
    e = tuple(R.conv(c) / L for c in dv)
    m = dict(model)
    m['axis_override'] = {member: (e[1], e[0], e[2])}
    return solve_vals(m)


def main():
    ref = json.load(open(sys.argv[1]))['cases']
    mine = json.load(open(sys.argv[2]))
    cases = load_cases(sys.argv[1])
    out = {}
    agree = disagree = 0
    for cid, c in ref.items():
        if 'expected' not in c or c['expected'] is None:
            continue
        exp = {r[0]: D(r[1]) for r in c['expected']}
        cls = {r[0]: r[2] for r in c['expected']}
        scales = {k: D(v['value']) for k, v in c['scales'].items()}
        model = vf.load_model(cases[cid]['model'])
        v2 = mine[cid]['int']
        res = []
        for nc in c['negative_controls']:
            nid = nc['id']
            info = {}
            if nid == 'NC-SUBTRACT-ROUNDED':
                vals = nc_subtract_rounded(model, v2)
            elif nid == 'NC-LOST-SOFT':
                vals, info = nc_lost_soft(cid, model, c)
            elif nid == 'NC-STORED-ASSEMBLY':
                vals = nc_stored_chain(model)
            elif nid == 'NC-WRONG-TRANSFORM':
                member = 'M2' if 'constitutive frame of M2' in nc['description'] else 'M1'
                vals = nc_wrong_transform(model, member)
                nodal = {k: v for k, v in vals.items() if k.split('.')[0] in ('u', 'th', 'R', 'S')}
                info['nodal_only'] = judge(exp, cls, scales, nodal)
            elif nid == 'NC-ORIGIN-MOMENTS' and not nc.get('discriminates'):
                # support moments about the origin differ from those about the node only by x x R_force
                forces = [abs(D(v2[k])) for k in v2 if k.startswith('R.') and k.split('.')[2][0] == 'U']
                vals = None
                info = {'max_translation_reaction': format(max(forces, default=D(0)), '.3e'),
                        'note': 'non-discriminating is truthful iff every restraint force is zero'}
                j = {'discriminates': max(forces, default=D(0)) > D('1e-60') * scales.get('force', D(1))}
            elif nid == 'NC-UNSWAPPED-ENDS' and not nc.get('discriminates'):
                mb = [abs(D(v2[k])) for k in v2 if k.startswith('Mb.')]
                vals = None
                info = {'max_bending': format(max(mb), '.3e')}
                j = {'discriminates': max(mb) > D('1e-60') * scales['moment']}
            elif nid == 'NC-ROTATION-SCALED-AS-TRANSLATION' and not nc.get('discriminates'):
                pl = c['range'].get('pl') if isinstance(c.get('range'), dict) else None
                vals = None
                info = {'pl': pl, 'note': 'rotation and translation factors coincide iff pl = 0'}
                j = {'discriminates': pl not in (0, '0', None)}
            else:
                continue
            if vals is not None:
                j = judge(exp, cls, scales, vals)
            same = j['discriminates'] == bool(nc.get('discriminates'))
            agree += same
            disagree += not same
            res.append({'id': nid, 'r1_discriminates': bool(nc.get('discriminates')),
                        'r1_max_violation_ratio': nc.get('max_violation_ratio'), 'v2': j, 'info': info,
                        'agree': same})
        if res:
            out[cid] = res
    out['_totals'] = {'agree': agree, 'disagree': disagree}
    json.dump(out, open(sys.argv[3], 'w'), indent=1, default=str)
    print(json.dumps(out['_totals']))
    for cid, rs in out.items():
        if cid.startswith('_'):
            continue
        for r in rs:
            if not r['agree']:
                print('DISAGREE', cid, json.dumps(r, default=str)[:400])


if __name__ == '__main__':
    main()
