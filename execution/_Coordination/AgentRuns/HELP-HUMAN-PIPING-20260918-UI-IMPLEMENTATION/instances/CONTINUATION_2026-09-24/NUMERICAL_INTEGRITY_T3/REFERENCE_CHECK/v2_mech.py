"""V2 RF-MECH check: nullity by exact elimination, and the stated null motions.

Usage: python3 v2_mech.py <references.json>
1. Nullity of the free-DOF stiffness, per connected component, by exact symmetric elimination in
   Fractions.  The stiffness is a sum of B^T D B with D positive definite, so its null space does not
   depend on the (positive) values of EA, EI, GJ; pi is set to 3 and to 355/113 (both give the same
   nullity).  For a PSD matrix, a zero pivot in symmetric elimination implies a zero remaining row, so
   the count of zero pivots is the nullity.  The 1000-member chain component of LINE-IN-CHAIN1000
   is checked in 110-digit decimal (smallest pivot ratio reported).
2. Each stated null motion (read from references.json): K v = 0 on every DOF (zero strain energy,
   springs included), every rigid restraint satisfied, motions linearly independent, work of the load.
"""
import json
import sys
from fractions import Fraction as Fr

from v2_frame import Num, load_model, solve, DOFS, dot
from v2_models import load_cases


def components(model):
    adj = {n: set() for n in model['nodes']}
    for (_, a, b, _) in model['members']:
        adj[a].add(b)
        adj[b].add(a)
    seen, comps = set(), []
    for n in model['nodes']:
        if n in seen:
            continue
        st, comp = [n], []
        seen.add(n)
        while st:
            v = st.pop()
            comp.append(v)
            for w in adj[v]:
                if w not in seen:
                    seen.add(w)
                    st.append(w)
        comps.append(comp)
    return comps


def submodel(model, nodes):
    ns = set(nodes)
    return {'nodes': {n: model['nodes'][n] for n in nodes if n in ns},
            'sections': model['sections'],
            'members': [m for m in model['members'] if m[1] in ns],
            'supports': {n: s for n, s in model['supports'].items() if n in ns},
            'loads': {n: l for n, l in model['loads'].items() if n in ns}, 'udl': {}}


def nullity(model, R, rel=None):
    A, b, free, pos, order = solve(model, R, return_system=True)
    n = len(free)
    zero = 0
    minratio = None
    for i in range(n):
        row = A[i]
        piv = row.get(i, R.zero)
        rowmax = max((abs(v) for v in row.values()), default=R.zero)
        if rel is None:
            is_zero = piv == 0
            if is_zero and any(v != 0 for v in row.values()):
                raise AssertionError('zero pivot with nonzero row: not PSD?')
        else:
            is_zero = rowmax == 0 or abs(piv) <= rel * rowmax
            if rowmax != 0:
                r = abs(piv) / rowmax
                minratio = r if minratio is None or r < minratio else minratio
        if is_zero:
            zero += 1
            continue
        items = [(j, v) for j, v in row.items() if j > i and v != 0]
        for (j, vj) in items:
            f = vj / piv
            Aj = A[j]
            for (k, vk) in items:
                if k >= j:
                    Aj[k] = Aj.get(k, R.zero) - f * vk
    return zero, n, minratio


def full_K_times(model, R, v):
    """(K v) on every DOF of every node, K = members + springs (restraints not applied)."""
    from v2_frame import element, section_props, spring_matrix
    props = {k: section_props(s, R) for k, s in model['sections'].items()}
    out = {n: [R.zero] * 6 for n in model['nodes']}
    for mem in model['members']:
        el = element(model, mem, R, props)
        ve = list(v[el['a']]) + list(v[el['b']])
        for r in range(12):
            s = sum((el['K'][r][c] * ve[c] for c in range(12)), R.zero)
            (out[el['a']] if r < 6 else out[el['b']])[r % 6] += s
    for n, s in model['supports'].items():
        for (kind, d, k) in s['springs']:
            Ks, _, _, _ = spring_matrix(d, k, R)
            off = 0 if kind in ('translation', 't') else 3
            for r in range(3):
                out[n][off + r] += sum(Ks[r][c] * v[n][off + c] for c in range(3))
    return out


def rank(vectors):
    rows = [list(v) for v in vectors]
    rk, col = 0, 0
    ncol = len(rows[0]) if rows else 0
    for col in range(ncol):
        p = next((i for i in range(rk, len(rows)) if rows[i][col] != 0), None)
        if p is None:
            continue
        rows[rk], rows[p] = rows[p], rows[rk]
        for i in range(len(rows)):
            if i != rk and rows[i][col] != 0:
                f = rows[i][col] / rows[rk][col]
                rows[i] = [a - f * b for a, b in zip(rows[i], rows[rk])]
        rk += 1
    return rk


def main():
    path = sys.argv[1]
    cases = load_cases(path)
    raw = json.load(open(path))['cases']
    report = {}
    for cid, c in cases.items():
        if c['family'] != 'RF-MECH' or cid == 'RF-MECH-LINE345-RX-COMPANION':
            continue
        model = load_model(c['model'])
        rec = {'components': []}
        total = {}
        for pi in (Fr(3), Fr(355, 113)):
            R = Num('frac', pi=pi)
            tot = 0
            for comp in components(model):
                sm = submodel(model, comp)
                if len(sm['members']) > 200:
                    Rd = Num('dec')
                    z, n, mr = nullity(sm, Rd, rel=Rd.conv(Fr(1, 10 ** 50)))
                    if pi == 3:
                        rec['components'].append({'members': len(sm['members']), 'free_dofs': n, 'nullity': z,
                                                  'method': 'decimal-110', 'min_pivot_ratio': format(mr, '.3e')})
                else:
                    z, n, _ = nullity(sm, R)
                    if pi == 3:
                        rec['components'].append({'members': len(sm['members']), 'free_dofs': n, 'nullity': z,
                                                  'method': 'exact'})
                tot += z
            total[str(pi)] = tot
        rec['nullity_v2'] = total
        rec['nullity_r1'] = raw[cid]['nullity']
        # stated null motions
        motions = raw[cid]['null_motions']
        vecs = []
        checks = []
        for mo in motions:
            v = {n: [Fr(0)] * 6 for n in model['nodes']}
            for n, uv in mo['nodes'].items():
                v[n] = [Fr(x) for x in uv['u']] + [Fr(x) for x in uv['theta']]
            ok_restraint = all(v[n][DOFS.index(d)] == 0 for n, s in model['supports'].items() for d in s['rigid'])
            kv_zero = True
            for pi in (Fr(3), Fr(355, 113)):
                R = Num('frac', pi=pi)
                kv = full_K_times(model, R, v)
                kv_zero &= all(x == 0 for n in kv for x in kv[n])
            work = sum(dot(F, v[n][0:3]) + dot(M, v[n][3:6]) for n, (F, M) in model['loads'].items())
            checks.append({'description': mo['description'], 'restraints_satisfied': ok_restraint,
                           'Kv_zero_all_dofs': kv_zero, 'load_work': str(work)})
            vecs.append([x for n in sorted(v) for x in v[n]])
        rec['null_motion_checks'] = checks
        rec['motions_independent'] = rank(vecs) == len(vecs)
        rec['load_work_r1'] = raw[cid]['load_work_on_null_motions']
        report[cid] = rec
        print(cid, json.dumps({k: rec[k] for k in ('nullity_v2', 'nullity_r1', 'motions_independent')}),
              [(x['restraints_satisfied'], x['Kv_zero_all_dofs'], x['load_work']) for x in checks], flush=True)
    return report


if __name__ == '__main__':
    rep = main()
    if len(sys.argv) > 2:
        json.dump(rep, open(sys.argv[2], 'w'), indent=1)
