"""V3 step 3: cancellation cases (CE-CANCEL, CANCEL-FEM, CANCEL-SEIS).

Usage: python3 v3_cancel.py <references_eload.json> <out v3_cancel.json>
* Recomputes the recommended (net-governed) scale, the gross scale and governed_by for every row,
  from V3's own net-alone, other-loads and gross-alone solves.
* Rebuilds every float-sum, dropped and binary64-product control with V3's own defect models and
  evaluates them under the recommended, class and gross scales.
* Sterbenz and order analysis of the binary64 sums.
* D-14 analysis for CANCEL-SEIS: represented vs intended nets, rounded-once generation, a sweep of the
  authored D at each ratio to see how often each defect discriminates.
"""
import json
import math
import sys
from decimal import Decimal, getcontext
from fractions import Fraction as Fr
import v3_core as V

getcontext().prec = 80
D = Decimal
CRIT = D('1e-9')


def dq(q):
    return D(q.numerator) / D(q.denominator)


def pub_dec(out):
    return {k: V.to_dec(v) for k, v in out.items()}


def mbvec(out):
    return {k: v[2] for k, v in out.items() if v[0] == 'sq'}


def vals_exact(out):
    """key -> exact Fraction (scalar) or exact 3-vector (bending moment vector for Mb)."""
    return {k: (v[1] if v[0] == 'v' else v[2]) for k, v in out.items()}


def mag(x):
    if isinstance(x, tuple):
        return D(dq(V.vdot(x, x))).sqrt()
    return abs(dq(x))


def diff(a, b):
    if isinstance(a, tuple):
        return V.vsub(a, b)
    return a - b


def is0(x):
    return all(c == 0 for c in x) if isinstance(x, tuple) else x == 0


def ratio(obs, exp, scale):
    return abs(obs - exp) / (CRIT * max(abs(exp), scale)) if max(abs(exp), scale) else D(0)


def worst(obsd, expd, scales):
    w, wk = D(0), None
    nf = 0
    for k, e in expd.items():
        r = ratio(obsd[k], e, scales[k])
        if r > 1:
            nf += 1
        if r > w:
            w, wk = r, k
    return w, wk, nf


def f64(x):
    return Fr(float(x))


def seis_bin64(inp, gf_axis='Z'):
    OD = float(inp['sec.G.OD'])
    t = float(inp['sec.G.wall']) - float(inp['sec.G.mill_tolerance'])
    ID = OD - 2 * t
    Dins = OD + 2 * float(inp['gen.t_insulation'])
    Am = math.pi / 4 * (OD * OD - ID * ID)
    Ac = math.pi / 4 * (ID * ID)
    Ai = math.pi / 4 * (Dins * Dins - OD * OD)
    m = Am * float(inp['gen.rho_metal']) + Ac * float(inp['gen.rho_contents']) + Ai * float(inp['gen.rho_insulation'])
    return m * float(inp['gen.g_factor.' + gf_axis]) * float(inp['gen.g'])


def main():
    ref = json.load(open(sys.argv[1]))
    res = {}
    for case in ref['cases']:
        cid = case['id']
        if 'cancellation' not in case:
            continue
        basis = 'int' if case['basis'] == 'intended' else 'rep'
        pb, sol, out = V.run(case, basis)
        tot = vals_exact(out)
        totd = pub_dec(out)
        cls = {r[0]: r[2] for r in case['expected']}
        cscale = {k: D(case['classes'][cls[k]]['scale']) for k in cls}
        rows = {r[0]: r for r in case['expected']}
        inp = case['inputs']
        rec = {}
        # ------- net-alone and gross-alone models
        if '-CE-CANCEL-' in cid:
            def m_net(p):
                p.nodal = [x for x in p.nodal if x[3].endswith('#2')]
                p.efforts = []

            def m_gross(p):
                p.nodal = [x for x in p.nodal if not x[3].endswith('#2')]
                p.efforts = []
            exact_net = sum(x[2] for x in pb.nodal) + sum(x[2] for x in pb.efforts)
        elif '-CANCEL-FEM-' in cid:
            # FEM contributions at S1 (equivalent nodal moment RZ), from V3's fixed-state solution
            fem = {}
            for m in pb.members:
                fs = V.FixedState(m['L'], m['e'], m['sec'].EA, m['sec'].EI)
                for el in pb.eloads:
                    if el['member'] == m['name']:
                        fs.add_span_load(el['q'], el['a'], el['b'])
                P = fs.end_reactions()
                fem[m['name']] = -P[11] if m['j'] == 'S1' else -P[5]
            Mz = [x for x in pb.nodal if x[0] == 'S1' and x[1] == 'RZ'][0][2]
            exact_net = fem['M1'] + fem['M2'] + Mz
            rec['fem_contributions'] = {k: str(v) for k, v in fem.items()}
            rec['exact_net'] = str(exact_net)

            def m_net(p):
                p.eloads = []
                p.nodal = [('S1', 'RZ', exact_net, 'net')]

            def m_gross(p):
                p.eloads = [el for el in p.eloads if el['member'] == 'M1']
                p.nodal = []
        else:
            ws = [el for el in pb.eloads if el['kind'] == 'seismic'][0]['q'][2]   # on the case's basis
            ws_int = [el for el in V.build(case, 'int').eloads if el['kind'] == 'seismic'][0]['q'][2]
            Dv = [el for el in pb.eloads if el['kind'] == 'authored'][0]['q'][2]
            exact_net = ws + Dv
            rec['w_s'] = format(dq(ws), '.40e')
            rec['D'] = format(dq(Dv), '.40e')
            rec['exact_net'] = format(dq(exact_net), '.40e')

            def m_net(p):
                p.eloads = [dict(p.eloads[0], q=(Fr(0), Fr(0), exact_net), kind='net')]

            def m_gross(p):
                p.eloads = [el for el in p.eloads if el['kind'] == 'seismic']
        _, _, onet = V.run(case, basis, m_net)
        _, _, ogross = V.run(case, basis, m_gross)
        net, gross = vals_exact(onet), vals_exact(ogross)
        # ------- columns
        colbad = []
        recsc = {}
        gov_counts = {}
        for k in cls:
            v, n = tot[k], net[k]
            other = diff(v, n)
            if is0(v):
                gov, rs = 'zero', cscale[k]
            elif is0(n):
                gov, rs = 'other loads only', cscale[k]
            elif is0(other):
                gov, rs = 'net', mag(n)
            else:
                gov, rs = 'mixed', mag(n)
            gs = mag(gross[k]) if not is0(gross[k]) else cscale[k]
            recsc[k] = rs
            gov_counts[gov] = gov_counts.get(gov, 0) + 1
            r = rows[k]
            ok = (abs(rs - D(r[3])) <= D('1e-24') * rs) and (gov == r[5])
            okg = abs(gs - D(r[4])) <= D('1e-24') * max(gs, D('1e-300'))
            if not ok or not okg:
                colbad.append((k, r[3], format(rs, '.25e'), r[4], format(gs, '.25e'), r[5], gov))
            assert rs <= cscale[k] * (1 + D('1e-24')), (cid, k)
        gscale = {k: (mag(gross[k]) if not is0(gross[k]) else cscale[k]) for k in cls}
        rec['columns_mismatch'] = colbad
        rec['governed_by_counts'] = gov_counts
        rec['recommended_below_abs_exp'] = sum(1 for k in cls if recsc[k] < abs(totd[k]))
        # ------- controls (V3 models)
        ctrls = {}

        def evalc(name, mod):
            _, _, o = V.run(case, basis, mod)
            od = pub_dec(o)
            e = {k: totd[k] for k in cls}
            out = {}
            for sname, sc in (('recommended', recsc), ('class', cscale), ('gross', gscale)):
                w, wk, nf = worst(od, e, sc)
                out[sname] = (format(w, '.5e'), wk, nf)
            ctrls[name] = out

        def net_replaced(newnet):
            if '-CE-CANCEL-' in cid:
                def m(p):
                    p.nodal = [('N2', 'UY', newnet, 'defect')]
                    p.efforts = []
            elif '-CANCEL-FEM-' in cid:
                def m(p):
                    p.nodal = [('S1', 'RZ', newnet - fem['M1'] - fem['M2'], 'defect')]
            else:
                def m(p):
                    p.eloads = [dict(p.eloads[0], q=(Fr(0), Fr(0), newnet), kind='defect')]
            return m

        sums = {}
        if '-CE-CANCEL-' in cid:
            G = float(inp['CE.N2.UY'])
            n = float(inp['P.N2.UY#2'])
            parts = {'Gm': -G, 'n': n, 'Gp': G}
            for order in (('Gm', 'n', 'Gp'), ('Gm', 'Gp', 'n'), ('n', 'Gm', 'Gp')):
                s = 0.0
                for o in order:
                    s = s + parts[o]
                sums['NC-FLOAT-SUM-' + ''.join(order)] = s
            for name, s in sums.items():
                evalc(name, net_replaced(Fr(s)))
            evalc('NC-SMALL-DROPPED', net_replaced(Fr(0)))
            evalc('X-OVERWRITE (last authored contribution assigned, G+)', net_replaced(Fr(G)))
        elif '-CANCEL-FEM-' in cid:
            wA = float(inp['w.M1#1.y'])
            wB = float(inp['w.M2#1.y'])
            A = -(wA * 2.0 * 2.0 / 12.0)
            B = (wB * 3.0 * 3.0 / 12.0)
            nz = float(inp['P.S1.RZ'])
            assert Fr(A) == fem['M1'] and Fr(B) == fem['M2'], (A, B, fem)
            parts = {'A': A, 'B': B, 'n': nz}
            for order in (('A', 'B', 'n'), ('A', 'n', 'B'), ('n', 'A', 'B')):
                s = 0.0
                for o in order:
                    s = s + parts[o]
                sums['NC-FLOAT-SUM-' + ''.join(order)] = s
            for name, s in sums.items():
                evalc(name, net_replaced(Fr(s)))
            evalc('NC-SMALL-DROPPED', net_replaced(fem['M1'] + fem['M2']))
            evalc('X-OVERWRITE (last contribution n assigned)', net_replaced(Fr(nz)))
            rec['fem_float_exact'] = True
        else:
            wb = seis_bin64(inp)
            gi = case['generated_intensities'][0]
            rec['bin64_product_equals_package'] = (repr(wb) == gi['binary64_product'], repr(wb), gi['binary64_product'])
            Dd = f64(inp['w.M1#2.z'])
            Dbasis = Dv
            rec['sterbenz_two_term'] = {}
            sums['NC-BIN64-PRODUCT'] = Fr(wb) + Dbasis
            sums['NC-BIN64-PRODUCT(D decoded)'] = Fr(wb) + Dd
            sums['NC-FLOAT-SUM-BIN64'] = Fr(wb + float(Dd))
            # package reading: the correctly rounded generated intensity ON THE CASE'S BASIS (w_rep on the represented basis)
            sums['NC-FLOAT-SUM'] = Fr(float(dq(ws)) + float(Dd))
            sums['NC-FLOAT-SUM (reading: fl of the INTENDED w_s)'] = Fr(float(dq(ws_int)) + float(Dd))
            # exactness of the two-term binary64 sums (Sterbenz: y/2 <= x <= 2y)
            rec['fl_w_int_equals_fl_w_rep'] = float(dq(ws_int)) == float(dq(V.build(case, 'rep').eloads[0]['q'][2]))
            for nm, a in (('bin64 product', wb), ('rounded w_s', float(dq(ws_int)))):
                s = a + float(Dd)
                rec['sterbenz_two_term'][nm] = (Fr(s) == Fr(a) + Dd, abs(a) / 2 <= abs(float(Dd)) <= 2 * abs(a))
            for name, s in sums.items():
                evalc(name, net_replaced(s))
            evalc('NC-NET-DROPPED', net_replaced(Fr(0)))
            evalc('X-OVERWRITE (authored load assigned over the generated one)', net_replaced(Dd))
            # D-14: rounded-once generation variants against the basis
            wrep = V.build(case, 'rep').eloads[0]['q'][2]
            extra = {
                'X-ROUNDED-ONCE-INTENDED: fl(exact intended w_s) + fl(D), summed exactly': Fr(float(dq(ws_int))) + Dd,
                'X-ROUNDED-ONCE-REPRESENTED: fl(exact product of decoded inputs) + fl(D), summed exactly': Fr(float(dq(wrep))) + Dd,
                'X-REPRESENTED-GENERATION: exact product of decoded inputs + fl(D) (the represented solution)': wrep + Dd,
                'X-INTENDED-GENERATION-DECODED-D: exact intended w_s + fl(D)': ws_int + Dd,
            }
            rec['d14_variants'] = {}
            for name, s in extra.items():
                evalc(name, net_replaced(s))
                rec['d14_variants'][name] = ctrls[name]
            # worst-case bound: half an ulp of |w_s| relative to the net
            ulp = math.ulp(float(dq(ws_int)))
            rec['fl_w_rep_minus_w_rep_in_ulps'] = format(dq(Fr(float(dq(wrep))) - wrep) / D(ulp), '.4f')
            rec['half_ulp_w_over_net'] = format(D(ulp) / 2 / abs(dq(exact_net)), '.4e')
        rec['float_sums'] = {k: str(v) if isinstance(v, Fr) else repr(v) for k, v in sums.items()}
        rec['controls_v3'] = ctrls
        # compare with the package's flags
        pk = {}
        for c in case['negative_controls']:
            mine = ctrls.get(c['id'])
            if mine is None:
                continue
            pk[c['id']] = {'package': (c['discriminates'], c['worst_ratio_to_criterion'], c.get('discriminates_under_class_scale'),
                                       c.get('discriminates_under_gross_scale')),
                           'v3': (D(mine['recommended'][0]) > 1, mine['recommended'][0], D(mine['class'][0]) > 1,
                                  D(mine['gross'][0]) > 1)}
        rec['control_comparison'] = pk
        res[cid] = rec
        print(cid, 'columns mismatch:', len(colbad), 'gov', gov_counts, 'rec<|exp|:', rec['recommended_below_abs_exp'])
        for k, v in ctrls.items():
            if k.startswith('X-OVERWRITE'):
                print('    ', k, v['recommended'])
        for k, v in pk.items():
            print('   ', k, 'pkg', v['package'], 'v3', v['v3'])
        if 'd14_variants' in rec:
            print('    bin64 product equals package:', rec['bin64_product_equals_package'][0], ' sterbenz:', rec['sterbenz_two_term'],
                  ' half-ulp(w)/net:', rec['half_ulp_w_over_net'])
            for k, v in rec['d14_variants'].items():
                print('    ', k[:40], v['recommended'])
            print('    NC-BIN64-PRODUCT(D decoded)', ctrls['NC-BIN64-PRODUCT(D decoded)']['recommended'])
    # --------------- sweep of the authored D for CANCEL-SEIS (how often each defect discriminates)
    sweep = {}
    seis = [c for c in ref['cases'] if '-CANCEL-SEIS-G1e5' in c['id']][0]
    pbI = V.build(seis, 'int')
    pbR = V.build(seis, 'rep')
    ws = pbI.eloads[0]['q'][2]
    wrep = pbR.eloads[0]['q'][2]
    wb = Fr(seis_bin64(seis['inputs']))
    for r in (10 ** 5, 10 ** 6, 10 ** 7, 10 ** 8):
        counts = {'rounded_once_intended>1': 0, 'rounded_once_represented>1': 0, 'bin64_product>1': 0,
                  'input_rounding>1e-9 (represented basis needed)': 0}
        N = 400
        maxes = {k: D(0) for k in counts}
        for t in range(N):
            # D printed to 20 significant digits near -w_s (1 - 1/r), perturbed so that the rounding position varies
            target = -dq(ws) * (1 - D(1) / r) * (1 + D(t) * D('1e-13'))
            Ds = format(target, '.19e')
            Dint, Ddec = Fr(Ds), Fr(float(Ds))
            net_int = ws + Dint
            net_rep = wrep + Ddec
            vals = {
                'rounded_once_intended>1': (Fr(float(dq(ws))) + Ddec - net_rep) / net_rep / Fr(1, 10 ** 9),
                'rounded_once_represented>1': (Fr(float(dq(wrep))) + Ddec - net_rep) / net_rep / Fr(1, 10 ** 9),
                'bin64_product>1': (wb + Ddec - net_rep) / net_rep / Fr(1, 10 ** 9),
                'input_rounding>1e-9 (represented basis needed)': (net_rep - net_int) / net_int / Fr(1, 10 ** 9),
            }
            for k, v in vals.items():
                a = abs(dq(v))
                maxes[k] = max(maxes[k], a)
                if a > 1:
                    counts[k] += 1
        sweep['r=%g' % r] = {'N': N, 'fraction_failing': {k: counts[k] / N for k in counts},
                             'max_ratio': {k: format(maxes[k], '.3e') for k in maxes}}
        print('sweep r=%g' % r, sweep['r=%g' % r])
    # --------------- rounding position of the generated intensity: g-factor search at r = 1e8
    gsearch = {}
    import copy
    for k in range(0, 21):
        gf = format(D('-0.2') - D(k) * D('0.005'), 'f')
        c2 = copy.deepcopy(seis)
        c2['inputs']['gen.g_factor.Z'] = gf
        wr = V.build(c2, 'rep').eloads[0]['q'][2]
        pos = dq(Fr(float(dq(wr))) - wr) / D(math.ulp(float(dq(wr))))
        # ratio of the rounded-once (represented) defect at r = 1e8: |fl(w) - w| / (1e-9 |w| / 1e8)
        rr = abs(dq(Fr(float(dq(wr))) - wr)) / (D('1e-9') * abs(dq(wr)) / D(10 ** 8))
        gsearch[gf] = (format(pos, '.4f'), format(rr, '.3f'))
    print('g-factor search (rounding position in ulps, rounded-once ratio at r=1e8):', gsearch)
    # --------------- proposal check: CANCEL-SEIS-G1e8 with g_factor.Z = -0.23 (full solve, all controls)
    prop = {}
    for gf in ('-0.2', '-0.23'):
        c2 = copy.deepcopy([c for c in ref['cases'] if c['id'].endswith('CANCEL-SEIS-G1e8')][0])
        c2['inputs']['gen.g_factor.Z'] = gf
        wi = V.build(c2, 'int').eloads[0]['q'][2]
        Ds = format(-dq(wi) * (1 - D(1) / 10 ** 8), '.19e')
        c2['inputs']['w.M1#2.z'] = Ds
        c2['model']['element_loads'][1]['authored'] = ['0', '0', Ds]
        c2['model']['element_loads'][1]['global_intensity'] = ['0', '0', Ds]
        c2['model']['element_loads'][0]['global_intensity'] = None
        pI, sI, oI = V.run(c2, 'int')
        pR, sR, oR = V.run(c2, 'rep')
        eI, eR = pub_dec(oI), pub_dec(oR)
        wr = pR.eloads[0]['q'][2]
        Dd = pR.eloads[1]['q'][2]
        fin = max(abs(eR[k] - eI[k]) / max(abs(eI[k]), D('1e-300')) for k in eI if eI[k] != 0)
        variants = {'NC-BIN64-PRODUCT': Fr(seis_bin64(c2['inputs'])) + Dd,
                    'NC-FLOAT-SUM (fl of the represented w)': Fr(float(dq(wr))) + Dd,
                    'fl of the intended w': Fr(float(dq(wi))) + Dd,
                    'decimal-exact generation, D decoded': wi + Dd}
        rr = {}
        for nm, newnet in variants.items():
            def mfun(p, newnet=newnet):
                p.eloads = [dict(p.eloads[0], q=(Fr(0), Fr(0), newnet), kind='defect')]
            _, _, o = V.run(c2, 'rep', mfun)
            od = pub_dec(o)
            rr[nm] = format(max(abs(od[k] - eR[k]) / (CRIT * abs(eR[k])) for k in eR if eR[k] != 0), '.4g')
        prop[gf] = {'D': Ds, 'finite_input (relative, nonzero values)': format(fin, '.4e'), 'ratios_vs_represented_basis': rr}
    print('proposal check G1e8:', json.dumps(prop, indent=1))
    json.dump({'cases': res, 'seis_D_sweep': sweep, 'gfactor_rounding_search_r1e8': gsearch, 'proposal_check_G1e8': prop}, open(sys.argv[2], 'w'), indent=1, default=str)


if __name__ == '__main__':
    main()
