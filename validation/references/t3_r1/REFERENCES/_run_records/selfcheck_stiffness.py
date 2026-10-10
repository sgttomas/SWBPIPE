"""Author self-check for T3 R1 references (NOT the independent refutation V2).

Re-solves every non-mechanism case with at most a few hundred members by an exact direct-stiffness
assembly written from Euler-Bernoulli theory (circular-section 12x12 element blocks in frame-invariant
form: e e^T, P = I - e e^T, S = [e]x), at two rational stand-ins for pi (22/7 and 333/106), and
compares every published quantity with the closed forms of references.py evaluated at the same pi.
The closed forms are rational functions of pi, so exact agreement at two points is a strong check of
the algebra and code; it is not a check of pi itself.  For every RF-MECH case it counts the exact zero
pivots of the unrestrained stiffness (the nullity) and compares it with the stated null motions.
For every case whose basis is the represented input, it repeats the comparison on represented inputs.

Standard library only; imports references.py for the case inputs and the closed forms.
Run from anywhere:  python3 _run_records/selfcheck_stiffness.py
"""
import sys
import time
from fractions import Fraction as Fr
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent))
import references as R  # noqa: E402

DOF_INDEX = {'UX': 0, 'UY': 1, 'UZ': 2, 'RX': 3, 'RY': 4, 'RZ': 5}
I3 = [[1, 0, 0], [0, 1, 0], [0, 0, 1]]


def outer(a, b):
    return [[a[r] * b[c] for c in range(3)] for r in range(3)]


def madd(*ms):
    return [[sum(m[r][c] for m in ms) for c in range(3)] for r in range(3)]


def msc(s, m):
    return [[s * m[r][c] for c in range(3)] for r in range(3)]


def skew(e):
    return [[0, -e[2], e[1]], [e[2], 0, -e[0]], [-e[1], e[0], 0]]


def element_blocks(dx, L, EA, EI, GJ):
    e = tuple(c / L for c in dx)
    ee = outer(e, e)
    P = madd(I3, msc(-1, ee))
    S = skew(e)
    Kuu = madd(msc(EA / L, ee), msc(12 * EI / L ** 3, P))
    c = 6 * EI / L ** 2
    Kii = madd(msc(GJ / L, ee), msc(4 * EI / L, P))
    Kij = madd(msc(-GJ / L, ee), msc(2 * EI / L, P))
    nK = msc(-1, Kuu)
    cS, mcS = msc(c, S), msc(-c, S)
    # block rows Fi, Mi, Fj, Mj ; block columns ui, thi, uj, thj
    return [[Kuu, mcS, nK, mcS], [cS, Kii, mcS, Kij], [nK, cS, Kuu, cS], [cS, Kij, mcS, Kii]], e


def bfs_order(model):
    adj = {n: [] for n in model.nodes}
    for m in model.members:
        adj[m['i']].append(m['j'])
        adj[m['j']].append(m['i'])
    seen, order = set(), []
    for start in model.nodes:
        if start in seen:
            continue
        seen.add(start)
        q = [start]
        k = 0
        while k < len(q):
            n = q[k]
            k += 1
            order.append(n)
            for c in adj[n]:
                if c not in seen:
                    seen.add(c)
                    q.append(c)
    return order


def solve_stiffness(model):
    ar = model.ar
    names = bfs_order(model)
    idx = {n: k for k, n in enumerate(names)}
    N = 6 * len(names)
    K = [dict() for _ in range(N)]

    def add(r, c, v):
        if v != 0:
            K[r][c] = K[r].get(c, 0) + v
    elems = {}
    for m in model.members:
        s = model.sections[m['sec']]
        dx = tuple(b - a for a, b in zip(model.nodes[m['i']], model.nodes[m['j']]))
        L = ar.sqrt(sum(c * c for c in dx))
        B, e = element_blocks(dx, L, s['EA'], s['EI'], s['GJ'])
        elems[m['id']] = (B, e, m)
        base = [6 * idx[m['i']], 6 * idx[m['i']] + 3, 6 * idx[m['j']], 6 * idx[m['j']] + 3]
        for br in range(4):
            for bc in range(4):
                blk = B[br][bc]
                for r in range(3):
                    for c in range(3):
                        add(base[br] + r, base[bc] + c, blk[r][c])
    f = [0] * N
    for node, (F, M) in model.loads.items():
        for k in range(3):
            f[6 * idx[node] + k] += F[k]
            f[6 * idx[node] + 3 + k] += M[k]
    # uniform member loads q (global, per length): consistent equivalent nodal loads
    #   forces q L/2 at both ends, moments +(L^2/12) e x q at i and -(L^2/12) e x q at j
    f_nodal = list(f)
    feq = {}
    for m in model.members:
        q = getattr(model, 'member_loads', {}).get(m['id'])
        if not q:
            continue
        B, e, _ = elems[m['id']]
        dx = tuple(b - a for a, b in zip(model.nodes[m['i']], model.nodes[m['j']]))
        L = ar.sqrt(sum(c * c for c in dx))
        exq = (e[1] * q[2] - e[2] * q[1], e[2] * q[0] - e[0] * q[2], e[0] * q[1] - e[1] * q[0])
        v = [c * L / 2 for c in q] + [c * L * L / 12 for c in exq] + [c * L / 2 for c in q] + [-c * L * L / 12 for c in exq]
        feq[m['id']] = (v, q, L)
        for k in range(6):
            f[6 * idx[m['i']] + k] += v[k]
            f[6 * idx[m['j']] + k] += v[6 + k]
    fixed = set()
    for node, sup in model.supports.items():
        for d in sup['rigid']:
            fixed.add(6 * idx[node] + DOF_INDEX[d])
        for (kind, d, k) in sup['springs']:
            off = 6 * idx[node] + (0 if kind == 't' else 3)
            dd = sum(c * c for c in d)
            for r in range(3):
                for c in range(3):
                    add(off + r, off + c, k * d[r] * d[c] / dd)
    free = [i for i in range(N) if i not in fixed]
    fpos = {g: p for p, g in enumerate(free)}
    rows = [{fpos[c]: v for c, v in K[g].items() if c in fpos} for g in free]
    rhs = [f[g] for g in free]
    n = len(free)
    zero_pivots = 0
    for c in range(n):
        if rows[c].get(c, 0) == 0:
            if any(v != 0 for cc, v in rows[c].items() if cc >= c):
                raise AssertionError('zero pivot with a nonzero row: matrix is not positive semidefinite')
            zero_pivots += 1
            continue
        pv = rows[c][c]
        rc = [(cc, v) for cc, v in rows[c].items() if cc > c]
        for r in [cc for cc, v in rc]:
            fct = rows[r].get(c, 0) / pv
            if fct == 0:
                continue
            rr = rows[r]
            rr.pop(c, None)
            for cc, v in rc:
                nv = rr.get(cc, 0) - fct * v
                if nv == 0:
                    rr.pop(cc, None)
                else:
                    rr[cc] = nv
            rhs[r] -= fct * rhs[c]
    if zero_pivots:
        return {'singular': True, 'zero_pivots': zero_pivots}
    x = [0] * n
    for c in range(n - 1, -1, -1):
        x[c] = (rhs[c] - sum(v * x[cc] for cc, v in rows[c].items() if cc > c)) / rows[c][c]
    U = [0] * N
    for p, g in enumerate(free):
        U[g] = x[p]
    u = {nm: tuple(U[6 * idx[nm] + k] for k in range(3)) for nm in names}
    th = {nm: tuple(U[6 * idx[nm] + 3 + k] for k in range(3)) for nm in names}
    springs = {}
    for node, sup in model.supports.items():
        sl = []
        for (kind, d, k) in sup['springs']:
            v = u[node] if kind == 't' else th[node]
            dd = sum(c * c for c in d)
            s = -k * sum(a * b for a, b in zip(d, v)) / dd
            sl.append(tuple(s * c for c in d))
        springs[node] = sl
    elem_force = [0] * N
    mem = {}
    for mid, (B, e, m) in elems.items():
        ue = list(u[m['i']]) + list(th[m['i']]) + list(u[m['j']]) + list(th[m['j']])
        fe = []
        for br in range(4):
            for r in range(3):
                fe.append(sum(B[br][bc][r][c] * ue[3 * bc + c] for bc in range(4) for c in range(3)))
        qmid = (0, 0, 0)
        if mid in feq:
            v, q, Lm = feq[mid]
            fe = [a - b for a, b in zip(fe, v)]
            exq = (e[1] * q[2] - e[2] * q[1], e[2] * q[0] - e[0] * q[2], e[0] * q[1] - e[1] * q[0])
            qmid = tuple(c * Lm * Lm / 8 for c in exq)
        for k in range(6):
            elem_force[6 * idx[m['i']] + k] += fe[k]
            elem_force[6 * idx[m['j']] + k] += fe[6 + k]
        fi, mi, fj, mj = fe[0:3], fe[3:6], fe[6:9], fe[9:12]
        Lm = ar.sqrt(sum((b - a) ** 2 for a, b in zip(model.nodes[m['i']], model.nodes[m['j']])))
        exf = (e[1] * fi[2] - e[2] * fi[1], e[2] * fi[0] - e[0] * fi[2], e[0] * fi[1] - e[1] * fi[0])
        # internal moment at the midpoint: -m_i + (L/2) e x f_i + (L^2/8) e x q
        mid_m = [-a + Lm / 2 * b + c for a, b, c in zip(mi, exf, qmid)]

        def perp2(v, e=e):
            ev = sum(a * b for a, b in zip(e, v))
            return sum(c * c for c in v) - ev * ev
        dth = [a - b for a, b in zip(th[m['j']], th[m['i']])]
        du = [a - b for a, b in zip(u[m['j']], u[m['i']])]
        mem[mid] = {'N': sum(a * b for a, b in zip(fj, e)), 'T': sum(a * b for a, b in zip(mj, e)),
                    'tw': sum(a * b for a, b in zip(dth, e)), 'ext': sum(a * b for a, b in zip(du, e)),
                    'b2_i': perp2(mi), 'b2_j': perp2(mj), 'b2_mid': perp2(mid_m)}
    reac = {}
    for node, sup in model.supports.items():
        for d in sup['rigid']:
            g = 6 * idx[node] + DOF_INDEX[d]
            ax = [0, 0, 0]
            ax[DOF_INDEX[d] % 3] = 1
            val = elem_force[g] - f_nodal[g]
            kind = 't' if d[0] == 'U' else 'r'
            for (kk, dv, k), sv in zip(sup['springs'], springs[node]):
                if kk == kind:
                    val -= sum(a * b for a, b in zip(ax, sv))
            reac.setdefault(node, {})[d] = val
    return {'singular': False, 'u': u, 'th': th, 'reac': reac, 'springs': springs, 'mem': mem}


def compare(case, ar, decode=None):
    closed = R.solve(case['defn'], case['method'], ar, decode)
    stiff = solve_stiffness(R.Model(case['defn'], ar, decode))
    if stiff['singular']:
        return 'stiffness singular', 0
    a = R.flatten(closed, case['defn'], regions=case.get('regions'))
    b = R.flatten(stiff, case['defn'], regions=case.get('regions'))
    bad = [k for (k, v, c), (k2, v2, c2) in zip(a, b) if k != k2 or v != v2]
    return ('%d mismatches (first %s)' % (len(bad), bad[0])) if bad else 'exact', len(a)


def main():
    t0 = time.time()
    R.LARGE_N = (10, 100)
    for b in R.BUILD_ORDER:
        b()
    lines = []
    fails = 0
    n_q = 0
    n_cases = 0
    for case in R.CASES:
        if case.get('mechanism'):
            for pi in (Fr(22, 7),):
                st = solve_stiffness(R.Model(case['defn'], R.Exact(pi)))
                ok = st['singular'] and st['zero_pivots'] == len(case['null_modes'])
                fails += 0 if ok else 1
                lines.append('%-44s nullity stiffness=%s stated=%d %s' % (case['id'], st.get('zero_pivots', 0),
                                                                         len(case['null_modes']), 'OK' if ok else 'FAIL'))
            continue
        res = []
        for pi in (Fr(22, 7), Fr(333, 106)):
            try:
                msg, nq = compare(case, R.Exact(pi))
            except ValueError as err:
                msg, nq = 'skipped (%s)' % err, 0
            res.append(msg)
            n_q += nq
            if msg != 'exact' and not msg.startswith('skipped'):
                fails += 1
        n_cases += 1
        lines.append('%-44s pi=22/7: %s | pi=333/106: %s' % (case['id'], res[0], res[1]))
    # represented-input bases: every case whose finite comparison is decisive is re-checked on represented inputs
    rep_ids = ['RF-CANCEL-F-G1e8-GnG-INPLANE', 'RF-CANCEL-UDL-W1e8', 'RF-SKEW-A-CANT-AX-122-r1e-12', 'RF-SKEW-A-CANT-AX-345-r1e-12', 'RF-FINITE-THIRTIETHS-O1e6',
               'RF-FINITE-TENTHS-O1e6', 'RF-FINITE-THIRDS-O1e6', 'RF-CHAIN-T-n10-r1e-12', 'RF-WEAK-W-3D-rho1e-12']
    for cid in rep_ids:
        case = next(c for c in R.CASES if c['id'] == cid)
        msg, nq = compare(case, R.Exact(Fr(22, 7)), R.rep)
        n_q += nq
        if msg != 'exact':
            fails += 1
        lines.append('%-44s represented inputs, pi=22/7: %s' % (cid, msg))
    for ln in lines:
        print(ln)
    print('cases compared: %d; quantities compared: %d; failures: %d; seconds: %.0f' % (n_cases, n_q, fails, time.time() - t0))
    return fails


if __name__ == '__main__':
    sys.exit(1 if main() else 0)
