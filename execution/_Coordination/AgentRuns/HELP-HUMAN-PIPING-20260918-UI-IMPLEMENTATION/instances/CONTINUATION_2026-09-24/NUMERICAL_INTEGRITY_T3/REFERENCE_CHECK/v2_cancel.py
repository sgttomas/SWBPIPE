"""V2 RF-CANCEL check: scales, governed_by, the net-governed recommendation and the negative controls.

Usage: python3 v2_cancel.py <references.json> <v2_values.json> <out.json>
Uses V2's own responses (v2_derive.py): full, net contribution alone, gross contribution alone, and the
binary64 left-to-right sums / dropped-small responses.  For each row it re-derives:
  recommended (net-governed) scale = |net-alone response|, or the class scale where that is zero or the
  value is zero; gross scale = |gross-alone response| (class scale where zero); governed_by.
It then evaluates every negative control under the recommended, class and gross scales, and states
whether the recommended scale ever falls below |exp| and where max(|exp|, rec) sits against D1's floor.
"""
import json
import sys
from decimal import Decimal, getcontext

getcontext().prec = 80
getcontext().Emax = 999999
getcontext().Emin = -999999
D = Decimal
R_FLOOR = D(2) ** -64 / D('1e-9')
ZERO_REL = D('1e-60')
KIND = {'translation': 'translation', 'extension': 'translation', 'rotation': 'rotation', 'twist': 'rotation',
        'force': 'force', 'moment': 'moment'}


def exact_zero_sets(mj):
    """Keys whose net-alone / gross-alone responses are exactly zero (exact Fraction solve, pi rational)."""
    from fractions import Fraction
    from v2_frame import Num, load_model, solve
    import v2_derive
    R = Num('frac')
    base = load_model(mj)
    out = {}
    if 'load_contributions_in_authored_order' in mj:
        contrib = mj['load_contributions_in_authored_order']
        node, dof = contrib[0][0], contrib[0][1]
        vals = [v2_derive.parse(x[2]) for x in contrib]
        ordm = load_model({k: v for k, v in mj.items() if k != 'load_contributions_in_authored_order'})
        for tag, val in (('net', sum(vals)), ('gro', max(abs(v) for v in vals))):
            o = solve(v2_derive.with_nodal(v2_derive.zero_loads(ordm), node, dof, val), R)
            out[tag] = {k for k, v in o.items() if v == 0}
    else:
        L = Fraction(2)
        wA = v2_derive.parse(mj['member_uniform_loads_N_per_m_global']['A'][1])
        wB = v2_derive.parse(mj['member_uniform_loads_N_per_m_global']['B'][1])
        Mz = v2_derive.parse(mj['loads']['S1']['M'][2])
        o = solve(v2_derive.with_nodal(v2_derive.zero_loads(base), 'S1', 'RZ', Mz + (wB - wA) * L * L / 12), R)
        out['net'] = {k for k, v in o.items() if v == 0}
        mA = v2_derive.zero_loads(base)
        mA['udl'] = {'A': base['udl']['A']}
        o = solve(mA, R)
        out['gro'] = {k for k, v in o.items() if v == 0}
    return out


def main():
    ref = json.load(open(sys.argv[1]))
    mine = json.load(open(sys.argv[2]))
    cmp_ = json.load(open(sys.argv[4])) if len(sys.argv) > 4 else None
    out = {}
    tot = {'rows': 0, 'rec_agree': 0, 'gross_agree': 0, 'gov_agree': 0, 'rec_below_exp': 0,
           'nc': 0, 'nc_flags_agree': 0, 'nc_values_agree': 0}
    for cid, c in ref['cases'].items():
        if c['family'] != 'RF-CANCEL':
            continue
        m = mine[cid]
        scales = {k: D(v['value']) for k, v in c['scales'].items()}
        full = {k: D(v) for k, v in m['int'].items()}
        net = {k: D(v) for k, v in m['cancel']['net_alone'].items()}
        gro = {k: D(v) for k, v in m['cancel']['gross_alone'].items()}
        rec = {'rows': [], 'mismatch': [], 'rec_below_exp': [], 'floor': []}
        exact_zero = exact_zero_sets(c['model'])
        clsmax = {}
        for row in c['expected']:
            for tag, src in (('net', net), ('gro', gro)):
                clsmax[(tag, row[2])] = max(clsmax.get((tag, row[2]), D(0)), abs(src[row[0]]))
        Sstar = {k: D(v) for k, v in cmp_['cases'][cid]['S_star'].items()} if cmp_ else None
        exp, cls, recs, gross = {}, {}, {}, {}
        for row in c['expected']:
            k, e, cl, rs, gs, gov = row[0], D(row[1]), row[2], D(row[3]), D(row[4]), row[5]
            exp[k], cls[k], recs[k], gross[k] = e, cl, rs, gs
            sc = scales[cl]
            # a response is zero when it is below 1e-60 of the largest response of its class in the same
            # response set (V2's decimal solve leaves ~1e-100 relative noise where the exact value is 0)
            nz = lambda v, tag: (k not in exact_zero[tag]) and abs(v) > ZERO_REL * clsmax[(tag, cl)]
            my_rec = sc if (e == 0 or not nz(net[k], 'net')) else abs(net[k])
            my_gro = sc if not nz(gro[k], 'gro') else abs(gro[k])
            if e == 0:
                my_gov = 'zero'
            elif not nz(net[k], 'net'):
                my_gov = 'other loads only'
            elif abs(full[k] - net[k]) <= D('1e-35') * abs(full[k]):
                my_gov = 'net'
            else:
                my_gov = 'mixed'
            # the scale columns are published at 25 significant digits
            ok_r = abs(my_rec - rs) <= D('1e-23') * rs
            ok_g = abs(my_gro - gs) <= D('1e-23') * gs
            ok_v = my_gov == gov
            tot['rows'] += 1
            tot['rec_agree'] += ok_r
            tot['gross_agree'] += ok_g
            tot['gov_agree'] += ok_v
            if not (ok_r and ok_g and ok_v):
                rec['mismatch'].append((k, str(rs), format(my_rec, '.6e'), str(gs), format(my_gro, '.6e'), gov, my_gov))
            if rs < abs(e) * (1 - D('1e-20')):
                rec['rec_below_exp'].append((k, gov, format(rs / abs(e), '.4f')))
                tot['rec_below_exp'] += 1
            if Sstar is not None:
                thr = R_FLOOR * Sstar[KIND[cl]]
                if max(abs(e), rs) < thr:
                    rec['floor'].append((k, gov, format(max(abs(e), rs) / Sstar[KIND[cl]], '.3e')))
        # negative controls
        ncs = []
        for nc in c['negative_controls']:
            nid = nc['id']
            key = nid if nid in m['cancel'] else None
            if key is None:
                ncs.append((nid, 'not recomputed'))
                continue
            vals = {k: D(v) for k, v in m['cancel'][key]['values'].items()}

            def disc(scale_of):
                worst, n = D(0), 0
                for k, e in exp.items():
                    s = max(abs(e), scale_of(k))
                    r = abs(vals[k] - e) / s
                    if r > D('1e-9'):
                        n += 1
                    worst = max(worst, r)
                return n > 0, n, worst
            d_rec = disc(lambda k: recs[k])
            d_cls = disc(lambda k: scales[cls[k]])
            d_gro = disc(lambda k: gross[k])
            r1 = (bool(nc.get('discriminates')), bool(nc.get('discriminates_under_class_scale')),
                  bool(nc.get('discriminates_under_gross_scale')))
            mine_flags = (d_rec[0], d_cls[0], d_gro[0])
            tot['nc'] += 1
            tot['nc_flags_agree'] += r1 == mine_flags
            # R1's listed violating values against V2's control responses
            listed = json.loads(nc['values']) if isinstance(nc.get('values'), str) else (nc.get('values') or {})
            lv_ok = all(abs(D(v) - vals[k]) <= D('1e-24') * max(abs(vals[k]), scales[cls[k]]) for k, v in listed.items())
            tot['nc_values_agree'] += lv_ok
            net_ok = None
            if 'net_used' in nc and 'net_f64' in m['cancel'][key]:
                from fractions import Fraction
                q = Fraction(float(m['cancel'][key]['net_f64']))
                exact = D(q.numerator) / D(q.denominator)
                net_ok = abs(D(nc['net_used']) - exact) <= D('1e-23') * abs(exact) if exact != 0 else D(nc['net_used']) == 0
            ncs.append({'id': nid, 'r1_flags(rec,class,gross)': r1, 'v2_flags': mine_flags,
                        'v2_max_ratio_rec_x1e9': format(d_rec[2] * D('1e9'), '.4e'), 'v2_violations_rec': d_rec[1],
                        'r1_max_violation_ratio': nc.get('max_violation_ratio'), 'r1_violating_values': nc.get('violating_values'),
                        'listed_values_match_v2': lv_ok, 'net_used_r1': nc.get('net_used'),
                        'net_f64_v2': m['cancel'][key].get('net_f64'), 'net_matches': net_ok})
        rec['negative_controls'] = ncs
        out[cid] = rec
    out['_totals'] = tot
    json.dump(out, open(sys.argv[3], 'w'), indent=1, default=str)
    print(json.dumps(tot))
    for cid, r in out.items():
        if cid.startswith('_'):
            continue
        bad = [n for n in r['negative_controls'] if isinstance(n, dict) and
               (n['r1_flags(rec,class,gross)'] != n['v2_flags'] or not n['listed_values_match_v2'] or n['net_matches'] is False)]
        print('%-34s mism=%d below=%d floor=%d badNC=%d %s' % (cid, len(r['mismatch']), len(r['rec_below_exp']),
                                                          len(r['floor']), len(bad), r['rec_below_exp'][:3]))


if __name__ == '__main__':
    main()
