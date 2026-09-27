"""V2 step 1: derive every non-mechanism case by direct stiffness, before looking at any R1 value.

Usage: python3 v2_derive.py <references.json> <out.json> [case-id-prefix ...]
Reads only model inputs (see v2_models.load_cases).  Writes, per case, the intended-input values, the
represented-input values (binary64-decoded inputs; skipped for n = 10000), and for RF-CANCEL the
net-alone, gross-alone and negative-control responses.  Values are written with 45 significant digits.
"""
import json
import sys
import time
from fractions import Fraction as Fr

from v2_frame import Num, load_model, solve, equilibrium_residual, parse, represented, DOFS
from v2_models import load_cases

R = Num('dec')


def fmt(v):
    return format(v, '.45e') if v != 0 else '0'


def run(model):
    out = solve(model, R)
    F, M = equilibrium_residual(model, R, out)
    tchk = max(abs(v) for k, v in out.items() if k.startswith('_Tchk')) if any(k.startswith('_Tchk') for k in out) else 0
    vals = {k: fmt(v) for k, v in out.items() if not k.startswith('_')}
    return vals, {'eq_force': fmt(max(abs(x) for x in F)), 'eq_moment': fmt(max(abs(x) for x in M)), 'torque_check': fmt(tchk)}


def zero_loads(m):
    m = dict(m)
    m['loads'] = {}
    m['udl'] = {}
    return m


def with_nodal(m, node, dof, value):
    m = dict(m)
    loads = {n: [tuple(F), tuple(Mv)] for n, (F, Mv) in m['loads'].items()}
    FM = loads.setdefault(node, [(Fr(0),) * 3, (Fr(0),) * 3])
    k = DOFS.index(dof)
    v = list(FM[k // 3])
    v[k % 3] += value
    FM[k // 3] = tuple(v)
    m['loads'] = loads
    return m


def cancel_extras(cid, mj):
    """Responses used to judge RF-CANCEL scales and negative controls (independently of R1)."""
    ex = {}
    base = load_model(mj)
    if 'load_contributions_in_authored_order' in mj:
        contrib = mj['load_contributions_in_authored_order']
        node, dof = contrib[0][0], contrib[0][1]
        vals = [parse(c[2]) for c in contrib]
        net = sum(vals)
        G = max(abs(v) for v in vals)
        n_small = min(vals, key=abs)
        m0 = dict(base)
        m0['loads'] = {n: l for n, l in base['loads'].items()}
        # model with ordinary loads only (contributions removed)
        mj_ord = dict(mj)
        mj_ord = {k: v for k, v in mj.items() if k != 'load_contributions_in_authored_order'}
        ordm = load_model(mj_ord)
        ex['net_alone'] = run(with_nodal(zero_loads(ordm), node, dof, net))[0]
        ex['gross_alone'] = run(with_nodal(zero_loads(ordm), node, dof, G))[0]
        ex['ordinary_alone'] = run(ordm)[0] if ordm['loads'] else None
        # binary64 left-to-right sums in each order of (G, n, -G)
        g, n = float(G), float(n_small)
        orders = {'GnG': (g, n, -g), 'GGn': (g, -g, n), 'nGG': (n, g, -g)}
        for lab, seq in orders.items():
            acc = 0.0
            first = True
            for x in seq:
                acc = x if first else acc + x
                first = False
            ex['NC-FLOAT-SUM-' + lab] = {'net_f64': repr(acc), 'values': run(with_nodal(ordm, node, dof, Fr(acc)))[0]}
        ex['NC-SMALL-DROPPED'] = {'values': run(ordm)[0]}
        ex['exact_net'] = str(net)
    else:
        # two spans with uniform loads, nodal moment at S1
        L = Fr(2)
        wA = parse(mj['member_uniform_loads_N_per_m_global']['A'][1])
        wB = parse(mj['member_uniform_loads_N_per_m_global']['B'][1])
        Mz = parse(mj['loads']['S1']['M'][2])
        net = Mz + (wB - wA) * L * L / 12
        ex['exact_net'] = str(net)
        ex['net_alone'] = run(with_nodal(zero_loads(base), 'S1', 'RZ', net))[0]
        mA = zero_loads(base)
        mA['udl'] = {'A': base['udl']['A']}
        ex['gross_alone'] = run(mA)[0]
        fa, fb, fz, fl = float(wA), float(wB), float(Mz), 2.0
        mAf = -fa * fl * fl / 12.0
        mBf = fb * fl * fl / 12.0
        for lab, s in (('A-node-B', (mAf + fz) + mBf), ('A-B-node', (mAf + mBf) + fz)):
            # exact model with the shared-node equivalent moment replaced by the binary64 sum
            ex['NC-FLOAT-SUM-' + lab] = {'net_f64': repr(s), 'values': run(with_nodal(base, 'S1', 'RZ', Fr(s) - net))[0]}
        m_drop = with_nodal(base, 'S1', 'RZ', -Mz)
        ex['NC-SMALL-DROPPED'] = {'values': run(m_drop)[0]}
    return ex


def main():
    src, dst = sys.argv[1], sys.argv[2]
    only = sys.argv[3:]
    cases = load_cases(src)
    res = {}
    t_all = time.time()
    for cid, c in cases.items():
        if only and not any(cid.startswith(p) for p in only):
            continue
        if c['family'] == 'RF-MECH' and cid != 'RF-MECH-LINE345-RX-COMPANION':
            continue
        t0 = time.time()
        mj = c['model']
        rec = {}
        rec['int'], rec['checks'] = run(load_model(mj))
        nmem = len(mj['members'])
        if nmem <= 1000:
            rec['rep'], _ = run(load_model(mj, rep=True))
        if c['family'] == 'RF-CANCEL':
            rec['cancel'] = cancel_extras(cid, mj)
        rec['seconds'] = round(time.time() - t0, 2)
        res[cid] = rec
        print('%-46s %6d values  %.1fs' % (cid, len(rec['int']), rec['seconds']), flush=True)
    json.dump(res, open(dst, 'w'), indent=0, sort_keys=True)
    print('total %.0fs' % (time.time() - t_all))


if __name__ == '__main__':
    main()
