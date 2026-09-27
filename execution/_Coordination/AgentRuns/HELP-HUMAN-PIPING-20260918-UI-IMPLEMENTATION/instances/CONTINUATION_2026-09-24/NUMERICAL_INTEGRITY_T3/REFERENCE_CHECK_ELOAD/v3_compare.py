"""V3 step 2: compare V3's derivation with the RF-ELOAD package (run after v3_derive.py).

Usage: python3 v3_compare.py <references_eload.json> <v3_values.json> <out v3_compare.json>
Checks: every expected value (on the case's basis), every represented value, finite_input, class
scales (V3's own implementation of README section 4), zero-valued keys and their scales,
nonzero_below_class_scale, the per-quantity represented-vs-intended table, generated intensities,
and local-to-global intensities.
"""
import json
import math
import struct
import sys
from decimal import Decimal, getcontext
from fractions import Fraction as Fr
import v3_core as V

getcontext().prec = 80
D = Decimal


def dq(q):
    return D(q.numerator) / D(q.denominator)


def fam(cid):
    return cid.split('-')[2]


def cls_of(key):
    p = key.split('.')[0]
    if p == 'u':
        return 'translation'
    if p == 'th':
        return 'rotation'
    if p in ('N',):
        return 'force'
    if p in ('T', 'Mb'):
        return 'moment'
    if p == 'tw':
        return 'twist'
    if p == 'ext':
        return 'extension'
    last = key.split('.')[-1]
    return 'force' if last[0] in 'UF' else 'moment'


def class_scales(case, vals, pb, springs_with_R=True):
    """V3 implementation of README section 4 from V3's own values. Returns {class: (scale, derivation)}."""
    mags = {c: D(0) for c in ('translation', 'rotation', 'force', 'moment', 'twist', 'extension')}
    nodevec = {}
    for k, v in vals.items():
        p = k.split('.')
        c = cls_of(k)
        if p[0] in ('u', 'th', 'R', 'S'):
            grp = ('RS' if springs_with_R else p[0]) if p[0] in ('R', 'S') else p[0]
            nodevec.setdefault((grp, p[1], c), []).append(v)
        else:
            mags[c] = max(mags[c], abs(v))
    for (grp, n, c), vs in nodevec.items():
        mags[c] = max(mags[c], D(sum(x * x for x in vs)).sqrt())
    Lc = max(m['L'] for m in pb.members)
    Lc_d = dq(Lc)
    LEA = max(dq(m['L'] / m['sec'].EA) for m in pb.members)
    LGJ = max(dq(m['L'] / m['sec'].GJ) for m in pb.members)
    L2EI = max(dq(m['L'] ** 2 / (2 * m['sec'].EI)) for m in pb.members)
    # references
    nf = {}
    nm = {}
    for n, dof, v, _ in pb.nodal:
        tgt = nf if dof[0] == 'U' else nm
        tgt.setdefault(n, [Fr(0)] * 3)[V.DOFS.index(dof) % 3] += v
    for n, dof, v in pb.efforts:
        nf.setdefault(n, [Fr(0)] * 3)[V.DOFS.index(dof) % 3] += v
    Fref = D(0)
    for n, vec in nf.items():
        Fref = max(Fref, D(sum(dq(x) ** 2 for x in vec)).sqrt())
    for m, Fp in pb.thrust.items():
        Fref = max(Fref, dq(Fp))
    for el in pb.eloads:
        L = pb.mem[el['member']]['L']
        Fref = max(Fref, D(dq(V.vdot(el['q'], el['q']))).sqrt() * dq((el['b'] - el['a']) * L))
    for m, eps in pb.eigen.items():
        Fref = max(Fref, abs(dq(pb.mem[m]['sec'].EA * eps)))
    Mref = D(0)
    for n, vec in nm.items():
        Mref = max(Mref, D(sum(dq(x) ** 2 for x in vec)).sqrt())
    out = {}
    fz, mz = mags['force'] == 0, mags['moment'] == 0
    if not fz:
        out['force'] = (mags['force'], 'nonempty')
    elif not mz:
        out['force'] = (mags['moment'] / Lc_d, 'moment/Lc')
    else:
        out['force'] = (max(Fref, Mref / Lc_d), 'max(Fref, Mref/Lc)')
    if not mz:
        out['moment'] = (mags['moment'], 'nonempty')
    elif not fz:
        out['moment'] = (mags['force'] * Lc_d, 'force*Lc')
    else:
        out['moment'] = (max(Mref, Fref * Lc_d), 'max(Mref, Fref*Lc)')
    tz, rz = mags['translation'] == 0, mags['rotation'] == 0
    if not tz:
        out['translation'] = (mags['translation'], 'nonempty')
    elif not rz:
        out['translation'] = (mags['rotation'] * Lc_d, 'rotation*Lc')
    if not rz:
        out['rotation'] = (mags['rotation'], 'nonempty')
    elif not tz:
        out['rotation'] = (mags['translation'] / Lc_d, 'translation/Lc')
    if tz and rz:
        pt = D(0)
        pr = D(0)
        for n, dd in pb.restraints.items():
            vt = [dq(dd.get(d, Fr(0))) for d in ('UX', 'UY', 'UZ')]
            vr = [dq(dd.get(d, Fr(0))) for d in ('RX', 'RY', 'RZ')]
            pt = max(pt, D(sum(x * x for x in vt)).sqrt())
            pr = max(pr, D(sum(x * x for x in vr)).sqrt())
        el = max([abs(dq(eps * pb.mem[m]['L'])) for m, eps in pb.eigen.items()] + [D(0)])
        cands = [out['force'][0] * LEA, pt, pr * Lc_d, el]
        if not mz:
            cands.append(out['moment'][0] * L2EI)
        t = max(cands)
        out['translation'] = (t, 'both zero: max(F L/EA, presc t, presc r Lc, eps L [, M L^2/2EI])')
        out['rotation'] = (t / Lc_d, 'both zero: translation/Lc')
    out['twist'] = (mags['twist'], 'nonempty') if mags['twist'] != 0 else (out['moment'][0] * LGJ, 'moment*L/GJ')
    out['extension'] = (mags['extension'], 'nonempty') if mags['extension'] != 0 else (out['force'][0] * LEA, 'force*L/EA')
    return out


def nd(a, b, s):
    return abs(a - b) / max(abs(b), s) if max(abs(b), s) != 0 else abs(a - b)


def f64_bits_product_generated(pb_rec):
    return None


def main():
    ref = json.load(open(sys.argv[1]))
    mine = json.load(open(sys.argv[2]))['cases']
    res = {'by_family': {}, 'cases': {}}
    for case in ref['cases']:
        cid = case['id']
        f = fam(cid)
        pb_int = V.build(case, 'int')
        pb_rep = V.build(case, 'rep')
        vi = {k: D(v) for k, v in mine[cid]['int'].items()}
        vr = {k: D(v) for k, v in mine[cid]['rep'].items()}
        basis = case['basis']
        vb = vi if basis == 'intended' else vr
        exp = {r[0]: r for r in case['expected']}
        cs = class_scales(case, vb, pb_int if basis == 'intended' else pb_rep)
        cs_sep = class_scales(case, vb, pb_int if basis == 'intended' else pb_rep, springs_with_R=False)
        # values
        worst = D(0)
        wk = None
        bad = []
        for k, r in exp.items():
            e = D(r[1])
            s = D(case['classes'][r[2]]['scale'])
            d = nd(vb[k], e, s)
            if d > worst:
                worst, wk = d, k
            if d > D('5e-40'):
                bad.append((k, str(vb[k]), r[1], format(d, '.3e')))
            assert r[2] == cls_of(k), (cid, k)
        cr = {'n': len(exp), 'keys_equal': set(exp) == set(vb), 'worst': format(worst, '.3e'), 'worst_key': wk,
              'over_5e-40': bad}
        # class scales
        sc_bad = []
        for c, rec in case['classes'].items():
            mineS = cs[c][0]
            if abs(mineS - D(rec['scale'])) > D('1e-24') * max(mineS, D('1e-300')):
                alt = cs_sep[c][0]
                sc_bad.append((c, rec['scale'], format(mineS, '.25e'), cs[c][1], rec['derivation'],
                               'separate R/S norms: ' + format(alt, '.25e')))
        cr['class_scale_mismatch'] = sc_bad
        cr['class_derivations'] = {c: cs[c][1] for c in cs}
        # zero-valued keys
        zk = {z[0]: z for z in case['zero_valued']}
        myz = {k for k, v in vb.items() if v == 0}
        cr['zero_keys_equal'] = set(zk) == myz
        cr['zero_scale_mismatch'] = [z for z in case['zero_valued'] if D(z[2]) != D(case['classes'][z[1]]['scale'])]
        nzb = sum(1 for k, v in vb.items() if v != 0 and abs(v) < cs[cls_of(k)][0])
        cr['nonzero_below_class_scale'] = (nzb, case['nonzero_below_class_scale'])
        # finite input
        fi = D(0)
        fk = None
        for k in vi:
            s = D(case['classes'][cls_of(k)]['scale'])
            d = nd(vr[k], vi[k], s)
            if d > fi:
                fi, fk = d, k
        cr['finite_input'] = (format(fi, '.5e'), fk, case['finite_input']['max_normalized_difference'], case['finite_input']['worst_key'])
        cr['basis_rule'] = ('represented' if fi > D('1e-9') else 'intended', basis)
        # represented values
        if 'expected_represented' in case:
            er = case['expected_represented']
            rows = er if isinstance(er, list) else er.get('values', er)
            w = D(0)
            n = 0
            for r in rows:
                k, e = r[0], D(r[1])
                s = D(case['classes'][cls_of(k)]['scale'])
                w = max(w, nd(vr[k], e, s))
                n += 1
            cr['represented'] = (n, format(w, '.3e'))
        if 'represented_vs_intended_per_quantity' in case:
            t = case['represented_vs_intended_per_quantity']
            rows = t if isinstance(t, list) else t.get('rows', t)
            w = D(0)
            mism = []
            for r in rows:
                k = r[0]
                s = D(case['classes'][cls_of(k)]['scale'])
                mineq = nd(vr[k], vi[k], s)
                theirs = D(r[-1]) if not isinstance(r[-1], (list, dict)) else None
                if theirs is not None and abs(mineq - theirs) > D('1e-5') * max(theirs, D('1e-30')) + D('1e-21'):
                    mism.append((k, format(mineq, '.5e'), r[-1]))
            cr['rep_vs_int_table_mismatch'] = mism[:10]
            cr['rep_vs_int_table_mismatch_count'] = len(mism)
        # generated intensities
        if 'generated_intensities' in case:
            gm = []
            for g in case['generated_intensities']:
                el = [x for x in pb_int.eloads if x['member'] == g['member'] and x['kind'] == g['kind']][0]
                elr = [x for x in pb_rep.eloads if x['member'] == g['member'] and x['kind'] == g['kind']][0]
                ax = 'XYZ'.index(g['axis'])
                wi, wr = el['q'][ax], elr['q'][ax]
                ok_i = abs(dq(wi) - D(g['intended_exact'])) <= D('1e-38') * abs(dq(wi))
                ok_r = abs(dq(wr) - D(g['represented_exact_product'])) <= D('1e-38') * abs(dq(wr))
                ok_pi = g['intended_over_pi'] is None or wi / V.PI == Fr(g['intended_over_pi'])
                gm.append((g['member'], g['kind'], g['axis'], ok_i, ok_r, ok_pi))
            cr['generated_intensities_ok'] = all(all(x[3:]) for x in gm)
        # local loads: global intensity published in the model
        li = []
        for el, mel in zip(pb_int.eloads, case['model']['element_loads']):
            if mel['source'] == 'authored':
                li.append(tuple(el['q']) == tuple(Fr(x) for x in mel['global_intensity']))
        cr['authored_global_intensity_ok'] = all(li)
        res['cases'][cid] = cr
        F = res['by_family'].setdefault(f, {'cases': 0, 'values': 0, 'le_5e-40': 0, 'max_nd': '0', 'represented': 0,
                                            'scale_mismatch': 0, 'zero_key_mismatch': 0})
        F['cases'] += 1
        F['values'] += len(exp)
        F['le_5e-40'] += len(exp) - len(bad)
        F['max_nd'] = format(max(D(F['max_nd']), worst), '.3e')
        F['represented'] += cr.get('represented', (0,))[0]
        F['scale_mismatch'] += len(sc_bad)
        F['zero_key_mismatch'] += 0 if cr['zero_keys_equal'] else 1
    json.dump(res, open(sys.argv[3], 'w'), indent=1, default=str)
    for f, F in res['by_family'].items():
        print(f, F)
    for cid, cr in res['cases'].items():
        flags = []
        if cr['over_5e-40']:
            flags.append('VALUES %d' % len(cr['over_5e-40']))
        if cr['class_scale_mismatch']:
            flags.append('SCALES %s' % cr['class_scale_mismatch'])
        if not cr['zero_keys_equal'] or cr['zero_scale_mismatch']:
            flags.append('ZEROS')
        if cr['nonzero_below_class_scale'][0] != cr['nonzero_below_class_scale'][1]:
            flags.append('NZB %s' % (cr['nonzero_below_class_scale'],))
        if cr['basis_rule'][0] != cr['basis_rule'][1]:
            flags.append('BASIS %s' % (cr['basis_rule'],))
        if cr.get('rep_vs_int_table_mismatch_count'):
            flags.append('REPTAB %d %s' % (cr['rep_vs_int_table_mismatch_count'], cr['rep_vs_int_table_mismatch'][:3]))
        if 'generated_intensities_ok' in cr and not cr['generated_intensities_ok']:
            flags.append('GENINT')
        if not cr['authored_global_intensity_ok']:
            flags.append('LOCAL')
        print(cid, cr['worst'], 'fi', cr['finite_input'], cr.get('represented', ''), ' '.join(flags))


if __name__ == '__main__':
    main()
