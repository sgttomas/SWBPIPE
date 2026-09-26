"""T3 RF-ELOAD independent references (R1 addendum R1_ADDENDUM_ELOAD.md). Standard library only; deterministic.

Usage: python3 references_eload.py            regenerates references_eload.json next to this file
       python3 references_eload.py --case ID... runs the named cases only (no JSON written)

No product code is read, imported or called. Every value is exact rational arithmetic with
pi -> PI_Q (the rational of the 190-digit Machin value). Route A (tree integration plus force
method) derives every value; route B (direct stiffness with consistent loads) must agree exactly.
"""
import sys
from fractions import Fraction as Fr

DOFS = ('UX', 'UY', 'UZ', 'RX', 'RY', 'RZ')
Z3 = (Fr(0), Fr(0), Fr(0))


def machin_pi_scaled(digits):
    guard = 12
    scale = 10 ** (digits + guard)

    def arctan_inv(x):
        total = term = scale // x
        x2 = x * x
        n = 1
        sign = -1
        while term:
            term //= x2
            total += sign * (term // (2 * n + 1))
            sign = -sign
            n += 1
        return total
    pi = 4 * (4 * arctan_inv(5) - arctan_inv(239))
    return pi // 10 ** guard


PI_DIGITS = 190
PI_Q = Fr(machin_pi_scaled(PI_DIGITS), 10 ** PI_DIGITS)


def add(a, b):
    return (a[0] + b[0], a[1] + b[1], a[2] + b[2])


def sub(a, b):
    return (a[0] - b[0], a[1] - b[1], a[2] - b[2])


def mul(s, a):
    return (s * a[0], s * a[1], s * a[2])


def dot(a, b):
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]


def cross(a, b):
    return (a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])


def norm2(a):
    return dot(a, a)


def unit_vec(dof):
    k = DOFS.index(dof) % 3
    return tuple(Fr(1) if m == k else Fr(0) for m in range(3))


def isqrt_frac_exact(q):
    """Exact square root of a rational square, or None."""
    from math import isqrt
    if q < 0:
        return None
    n, d = q.numerator, q.denominator
    rn, rd = isqrt(n), isqrt(d)
    if rn * rn == n and rd * rd == d:
        return Fr(rn, rd)
    return None


def member_geometry(model, m):
    xi = model['nodes'][m['i']]
    xj = model['nodes'][m['j']]
    d = sub(xj, xi)
    L = isqrt_frac_exact(norm2(d))
    if L is None:
        raise ValueError('member %s has irrational length' % m['name'])
    return L, mul(1 / L, d)


def solve_linear(A, b):
    """Exact Gaussian elimination. A: list of rows (Fractions)."""
    n = len(A)
    M = [list(A[r]) + [b[r]] for r in range(n)]
    for c in range(n):
        p = next((r for r in range(c, n) if M[r][c] != 0), None)
        if p is None:
            raise ZeroDivisionError('singular system')
        if p != c:
            M[c], M[p] = M[p], M[c]
        piv = M[c][c]
        rowc = M[c]
        for r in range(c + 1, n):
            f = M[r][c]
            if f:
                f = f / piv
                rowr = M[r]
                for k in range(c, n + 1):
                    if rowc[k]:
                        rowr[k] -= f * rowc[k]
    x = [Fr(0)] * n
    for r in range(n - 1, -1, -1):
        s = M[r][n]
        for k in range(r + 1, n):
            if M[r][k]:
                s -= M[r][k] * x[k]
        x[r] = s / M[r][r]
    return x


class Mag:
    """A bending magnitude carried exactly as its square."""
    __slots__ = ('sq',)

    def __init__(self, sq):
        self.sq = sq

    def __eq__(self, other):
        return isinstance(other, Mag) and self.sq == other.sq

    def __repr__(self):
        return 'Mag(%s)' % float(self.sq) ** 0.5


STATIONS = (('i', Fr(0)), ('q1', Fr(1, 4)), ('mid', Fr(1, 2)), ('q3', Fr(3, 4)), ('j', Fr(1)))


def total_nodal(model, include_caps=True):
    """Nodal forces/moments: authored nodal loads, constant efforts, thrust caps."""
    nod = {}

    def put(node, dof, val):
        F, M = nod.get(node, (Z3, Z3))
        v = mul(val, unit_vec(dof))
        if dof[0] == 'U':
            F = add(F, v)
        else:
            M = add(M, v)
        nod[node] = (F, M)
    for c in model.get('nloads', []):
        put(c['node'], c['dof'], c['value'])
    for c in model.get('efforts', []):
        put(c['node'], c['dof'], c['value'])
    if include_caps:
        for m in model['members']:
            Fp = model.get('thrust', {}).get(m['name'])
            if Fp:
                L, e = member_geometry(model, m)
                for node, s in ((m['i'], -1), (m['j'], 1)):
                    F, M = nod.get(node, (Z3, Z3))
                    nod[node] = (add(F, mul(s * Fp, e)), M)
    return nod


# ---------------------------------------------------------------------------
# Route A: exact Euler-Bernoulli tree integration plus force method.
# ---------------------------------------------------------------------------

def _tree(model):
    root = model['root']
    adj = {}
    for idx, m in enumerate(model['members']):
        adj.setdefault(m['i'], []).append(idx)
        adj.setdefault(m['j'], []).append(idx)
    order = [root]
    parent = {root: None}
    k = 0
    while k < len(order):
        p = order[k]
        k += 1
        for idx in adj.get(p, []):
            m = model['members'][idx]
            c = m['j'] if m['i'] == p else m['i']
            if c in parent:
                if parent[p] is None or parent[p][0] != idx:
                    raise ValueError('member graph is not a tree')
                continue
            parent[c] = (idx, p)
            order.append(c)
    if len(order) != len(model['nodes']):
        raise ValueError('member graph is disconnected')
    return order, parent


def _simpson(f, t0, t1):
    h = t1 - t0
    return mul(h / 6, add(add(f(t0), mul(4, f((t0 + t1) / 2))), f(t1)))


def _simpson_s(f, t0, t1):
    h = t1 - t0
    return h / 6 * (f(t0) + 4 * f((t0 + t1) / 2) + f(t1))


def tree_solve(model, nodal, use_loads, use_eigen, root_motion):
    """Solve the determinate tree (root fully fixed at root_motion).

    nodal: {node: (F, M)} point actions (including redundant actions).
    Returns (u, th, member_eval, root_reaction(F, M)).
    """
    nodes = model['nodes']
    order, parent = _tree(model)
    members = model['members']
    geo = {}
    loads = {}
    for idx, m in enumerate(members):
        L, e_auth = member_geometry(model, m)
        geo[idx] = (L, e_auth)
        loads[idx] = []
    if use_loads:
        byname = {m['name']: idx for idx, m in enumerate(members)}
        for ml in model.get('mloads', []):
            loads[byname[ml['member']]].append(ml)
    children = {n: [] for n in nodes}
    for c in order[1:]:
        idx, p = parent[c]
        children[p].append((idx, c))

    def oriented(idx, p):
        m = members[idx]
        L, e_auth = geo[idx]
        if m['i'] == p:
            e = e_auth
            segs = [(ml['q'], ml['a'] * L, ml['b'] * L) for ml in loads[idx]]
        else:
            e = mul(-1, e_auth)
            segs = [(ml['q'], (1 - ml['b']) * L, (1 - ml['a']) * L) for ml in loads[idx]]
        return L, e, segs

    F = {}
    M = {}
    for c in reversed(order):
        Fc, Mc = nodal.get(c, (Z3, Z3))
        for idx, d in children[c]:
            L, e, segs = oriented(idx, c)
            Fc = add(Fc, F[d])
            Mc = add(Mc, add(M[d], cross(sub(nodes[d], nodes[c]), F[d])))
            for q, A, B in segs:
                Fc = add(Fc, mul(B - A, q))
                Mc = add(Mc, mul((B * B - A * A) / 2, cross(e, q)))
        F[c] = Fc
        M[c] = Mc
    u = {}
    th = {}
    root = model['root']
    u[root], th[root] = root_motion
    evals = {}
    for c in order[1:]:
        idx, p = parent[c]
        m = members[idx]
        L, e, segs = oriented(idx, p)
        EA, EI, GJ = m['EA'], m['EI'], m['GJ']
        Fc, Mc = F[c], M[c]

        def Fout(t, Fc=Fc, segs=segs):
            r = Fc
            for q, A, B in segs:
                lo = max(t, A)
                if B > lo:
                    r = add(r, mul(B - lo, q))
            return r

        def Mout(t, Fc=Fc, Mc=Mc, segs=segs, L=L, e=e):
            r = add(Mc, mul(L - t, cross(e, Fc)))
            acc = Z3
            for q, A, B in segs:
                lo = max(t, A)
                if B > lo:
                    acc = add(acc, mul(((B - t) ** 2 - (lo - t) ** 2) / 2, q))
            return add(r, cross(e, acc))

        def C(v, e=e, EI=EI, GJ=GJ):
            a = dot(v, e)
            return add(mul(a / GJ, e), mul(1 / EI, sub(v, mul(a, e))))
        bps = sorted(set([Fr(0), L] + [x for q, A, B in segs for x in (A, B) if 0 < x < L]))
        IM1 = Z3
        IM2 = Z3
        IN = Fr(0)
        for t0, t1 in zip(bps, bps[1:]):
            IM1 = add(IM1, _simpson(Mout, t0, t1))
            IM2 = add(IM2, _simpson(lambda t: mul(L - t, Mout(t)), t0, t1))
            IN += _simpson_s(lambda t: dot(Fout(t), e), t0, t1)
        eps = model.get('eigen', {}).get(m['name'], Fr(0)) if use_eigen else Fr(0)
        th[c] = add(th[p], C(IM1))
        u[c] = add(add(add(u[p], cross(th[p], mul(L, e))), cross(C(IM2), e)),
                   mul(IN / EA + eps * L, e))
        evals[m['name']] = (L, e, m['i'] == p, Fout, Mout)
    return u, th, evals, (mul(-1, F[root]), mul(-1, M[root]))


def _readout(u, th, node, dof):
    k = DOFS.index(dof)
    return u[node][k] if k < 3 else th[node][k - 3]


def solve_A(model):
    root = model['root']
    rs = model['restraints']
    assert set(rs[root]) == set(DOFS), 'root must be fully restrained'
    cons = []
    for node in model['nodes']:
        if node == root or node not in rs:
            continue
        for dof in DOFS:
            if dof in rs[node]:
                cons.append((node, dof, None, rs[node][dof]))
    for s in model.get('springs', []):
        assert s['node'] != root and s['dof'] not in rs.get(s['node'], {})
        cons.append((s['node'], s['dof'], s['k'], Fr(0)))
    r0 = rs[root]
    root_motion = ((r0['UX'], r0['UY'], r0['UZ']), (r0['RX'], r0['RY'], r0['RZ']))
    base = total_nodal(model)
    u0, th0, _, _ = tree_solve(model, base, True, True, root_motion)
    n = len(cons)
    flex = [[Fr(0)] * n for _ in range(n)]
    for l, (node, dof, k, val) in enumerate(cons):
        v = unit_vec(dof)
        act = {node: (v, Z3) if dof[0] == 'U' else (Z3, v)}
        ul, thl, _, _ = tree_solve(model, act, False, False, (Z3, Z3))
        for kk, (n2, d2, _, _) in enumerate(cons):
            flex[kk][l] = _readout(ul, thl, n2, d2)
    A = [row[:] for row in flex]
    b = []
    for kk, (node, dof, k, val) in enumerate(cons):
        if k is not None:
            A[kk][kk] += 1 / k
        b.append(val - _readout(u0, th0, node, dof))
    X = solve_linear(A, b) if n else []
    nod = {nd: (F, Mm) for nd, (F, Mm) in base.items()}
    for (node, dof, k, val), x in zip(cons, X):
        F, Mm = nod.get(node, (Z3, Z3))
        v = mul(x, unit_vec(dof))
        nod[node] = (add(F, v), Mm) if dof[0] == 'U' else (F, add(Mm, v))
    u, th, evals, (RF, RM) = tree_solve(model, nod, True, True, root_motion)
    reac = {}
    for k3, dof in enumerate(DOFS):
        reac[(root, dof)] = RF[k3] if k3 < 3 else RM[k3 - 3]
    springs = {}
    for (node, dof, k, val), x in zip(cons, X):
        if k is None:
            reac[(node, dof)] = x
        else:
            springs[(node, dof)] = x
    thrust = model.get('thrust', {})
    conv = model.get('thrust_convention', 'effective')
    mem = {}
    for m in model['members']:
        L, e, parent_is_i, Fout, Mout = evals[m['name']]
        Fp = thrust.get(m['name'], Fr(0)) if conv == 'effective' else Fr(0)
        vals = {}
        for sname, sig in STATIONS:
            t = sig * L if parent_is_i else (1 - sig) * L
            Fv, Mv = Fout(t), Mout(t)
            N = dot(Fv, e) - Fp
            T = dot(Mv, e)
            vals[sname] = (N, T, norm2(sub(Mv, mul(T, e))))
        mem[m['name']] = vals
    return {'u': u, 'th': th, 'reac': reac, 'springs': springs, 'mem': mem}


# ---------------------------------------------------------------------------
# Route B: exact direct stiffness with consistent loads and fixed-end
# correction (independent cross-check of route A).
# ---------------------------------------------------------------------------

def _mat_S(e):
    return ((Fr(0), -e[2], e[1]), (e[2], Fr(0), -e[0]), (-e[1], e[0], Fr(0)))


def _mat_lin(terms):
    out = [[Fr(0)] * 3 for _ in range(3)]
    for s, Mx in terms:
        for r in range(3):
            for c in range(3):
                out[r][c] += s * Mx[r][c]
    return out


def element_stiffness(L, e, EA, EI, GJ):
    Pa = tuple(tuple(e[r] * e[c] for c in range(3)) for r in range(3))
    I3 = tuple(tuple(Fr(1) if r == c else Fr(0) for c in range(3)) for r in range(3))
    Pt = tuple(tuple(I3[r][c] - Pa[r][c] for c in range(3)) for r in range(3))
    S = _mat_S(e)
    a, b, c = EA / L, 12 * EI / L ** 3, 6 * EI / L ** 2
    ei, gj = EI / L, GJ / L
    AB = _mat_lin([(a, Pa), (b, Pt)])
    nAB = _mat_lin([(-a, Pa), (-b, Pt)])
    cS = _mat_lin([(c, S)])
    ncS = _mat_lin([(-c, S)])
    R4 = _mat_lin([(4 * ei, Pt), (gj, Pa)])
    R2 = _mat_lin([(2 * ei, Pt), (-gj, Pa)])
    blocks = [[AB, ncS, nAB, ncS],
              [cS, R4, ncS, R2],
              [nAB, cS, AB, cS],
              [cS, R2, ncS, R4]]
    K = [[Fr(0)] * 12 for _ in range(12)]
    for bi in range(4):
        for bj in range(4):
            for r in range(3):
                for cc in range(3):
                    K[3 * bi + r][3 * bj + cc] = blocks[bi][bj][r][cc]
    return K


def consistent_load(L, e, q, a, b):
    qa = mul(dot(q, e), e)
    qt = sub(q, qa)
    d1 = b - a
    d2 = (b ** 2 - a ** 2)
    d3 = (b ** 3 - a ** 3)
    d4 = (b ** 4 - a ** 4)
    I1a = d1 - d2 / 2
    I1b = d2 / 2
    IN1 = d1 - d3 + d4 / 2
    IN2 = d2 / 2 - 2 * d3 / 3 + d4 / 4
    IN3 = d3 - d4 / 2
    IN4 = -d3 / 3 + d4 / 4
    ext = cross(e, qt)
    fi = mul(L, add(mul(I1a, qa), mul(IN1, qt)))
    mi = mul(L * L * IN2, ext)
    fj = mul(L, add(mul(I1b, qa), mul(IN3, qt)))
    mj = mul(L * L * IN4, ext)
    return list(fi) + list(mi) + list(fj) + list(mj)


def solve_B(model):
    names = list(model['nodes'])
    pos = {nm: 6 * k for k, nm in enumerate(names)}
    n = 6 * len(names)
    K = [[Fr(0)] * n for _ in range(n)]
    f = [Fr(0)] * n
    thrust = model.get('thrust', {})
    conv = model.get('thrust_convention', 'effective')
    eig = model.get('eigen', {})
    mloads = {}
    for ml in model.get('mloads', []):
        mloads.setdefault(ml['member'], []).append(ml)
    elem = {}
    for m in model['members']:
        L, e = member_geometry(model, m)
        Ke = element_stiffness(L, e, m['EA'], m['EI'], m['GJ'])
        feq = [Fr(0)] * 12
        for ml in mloads.get(m['name'], []):
            cl = consistent_load(L, e, ml['q'], ml['a'], ml['b'])
            feq = [x + y for x, y in zip(feq, cl)]
        eps = eig.get(m['name'], Fr(0))
        if eps:
            P = m['EA'] * eps
            for k3 in range(3):
                feq[k3] -= P * e[k3]
                feq[6 + k3] += P * e[k3]
        cap = [Fr(0)] * 12
        Fp = thrust.get(m['name'], Fr(0))
        if Fp:
            for k3 in range(3):
                cap[k3] -= Fp * e[k3]
                cap[6 + k3] += Fp * e[k3]
        dofs = [pos[m['i']] + k for k in range(6)] + [pos[m['j']] + k for k in range(6)]
        for r in range(12):
            f[dofs[r]] += feq[r] + cap[r]
            for c in range(12):
                K[dofs[r]][dofs[c]] += Ke[r][c]
        defects = model.get('defects', set())
        rec = feq[:] if conv == 'true' else [x + y for x, y in zip(feq, cap)]
        if 'no_fec' in defects:
            for ml in mloads.get(m['name'], []):
                cl = consistent_load(L, e, ml['q'], ml['a'], ml['b'])
                rec = [x - y for x, y in zip(rec, cl)]
        if 'no_eigen_recovery' in defects and eps:
            P = m['EA'] * eps
            for k3 in range(3):
                rec[k3] += P * e[k3]
                rec[6 + k3] -= P * e[k3]
        elem[m['name']] = (L, e, Ke, dofs, rec, mloads.get(m['name'], []))
    for c in model.get('nloads', []) + model.get('efforts', []):
        f[pos[c['node']] + DOFS.index(c['dof'])] += c['value']
    for s in model.get('springs', []):
        g = pos[s['node']] + DOFS.index(s['dof'])
        K[g][g] += s['k']
    presc = {}
    for node, dd in model['restraints'].items():
        for dof, val in dd.items():
            presc[pos[node] + DOFS.index(dof)] = val
    free = [g for g in range(n) if g not in presc]
    uvec = [Fr(0)] * n
    for g, v in presc.items():
        uvec[g] = v
    A = [[K[r][c] for c in free] for r in free]
    rhs = [f[r] - sum((K[r][g] * v for g, v in presc.items() if v), Fr(0)) for r in free]
    sol = solve_linear(A, rhs)
    for g, v in zip(free, sol):
        uvec[g] = v
    Ku = [sum((K[r][c] * uvec[c] for c in range(n) if uvec[c] and K[r][c]), Fr(0)) for r in range(n)]
    reac = {}
    for node, dd in model['restraints'].items():
        for dof in dd:
            g = pos[node] + DOFS.index(dof)
            reac[(node, dof)] = Ku[g] - f[g]
    springs = {}
    for s in model.get('springs', []):
        g = pos[s['node']] + DOFS.index(s['dof'])
        springs[(s['node'], s['dof'])] = -s['k'] * uvec[g]
    u = {nm: tuple(uvec[pos[nm] + k] for k in range(3)) for nm in names}
    th = {nm: tuple(uvec[pos[nm] + 3 + k] for k in range(3)) for nm in names}
    mem = {}
    for m in model['members']:
        L, e, Ke, dofs, rec, mls = elem[m['name']]
        ue = [uvec[d] for d in dofs]
        fe = [sum((Ke[r][c] * ue[c] for c in range(12)), Fr(0)) - rec[r] for r in range(12)]
        Fi = tuple(fe[0:3])
        Mi = tuple(fe[3:6])
        vals = {}
        for sname, sig in STATIONS:
            s = sig * L
            Fs = mul(-1, Fi)
            Ms = add(mul(-1, Mi), mul(s, cross(e, Fi)))
            acc = Z3
            for ml in mls:
                A0, B0 = ml['a'] * L, ml['b'] * L
                hi = min(s, B0)
                if hi > A0:
                    Fs = sub(Fs, mul(hi - A0, ml['q']))
                    acc = add(acc, mul(((s - A0) ** 2 - (s - hi) ** 2) / 2, ml['q']))
            Ms = add(Ms, cross(e, acc))
            N = dot(Fs, e)
            T = dot(Ms, e)
            vals[sname] = (N, T, norm2(sub(Ms, mul(T, e))))
        mem[m['name']] = vals
    return {'u': u, 'th': th, 'reac': reac, 'springs': springs, 'mem': mem}


def publish(model, sol):
    """Flatten a solution into convention-free published quantities."""
    out = {}
    for nm in model['nodes']:
        for k, d in enumerate(('UX', 'UY', 'UZ')):
            out['u.%s.%s' % (nm, d)] = sol['u'][nm][k]
        for k, d in enumerate(('RX', 'RY', 'RZ')):
            out['th.%s.%s' % (nm, d)] = sol['th'][nm][k]
    for (node, dof), v in sorted(sol['reac'].items(), key=lambda kv: (list(model['nodes']).index(kv[0][0]), DOFS.index(kv[0][1]))):
        out['R.%s.%s' % (node, dof)] = v
    for (node, dof), v in sol['springs'].items():
        out['S.%s.%s' % (node, ('F' if dof[0] == 'U' else 'M') + dof[1].lower())] = v
    for m in model['members']:
        L, e = member_geometry(model, m)
        vals = sol['mem'][m['name']]
        for sname in ('i', 'mid', 'j'):
            out['N.%s.%s' % (m['name'], sname)] = vals[sname][0]
        for sname in ('i', 'mid', 'j'):
            out['T.%s.%s' % (m['name'], sname)] = vals[sname][1]
        for sname, _ in STATIONS:
            out['Mb.%s.%s' % (m['name'], sname)] = Mag(vals[sname][2])
        out['ext.%s' % m['name']] = dot(sub(sol['u'][m['j']], sol['u'][m['i']]), e)
        out['tw.%s' % m['name']] = dot(sub(sol['th'][m['j']], sol['th'][m['i']]), e)
    return out


# ---------------------------------------------------------------------------
# Exact input handling, formatting and the model builder.
# ---------------------------------------------------------------------------
import math
from decimal import Decimal, localcontext, getcontext

getcontext().prec = 60   # every Decimal operation outside an explicit context runs at 60 digits

CRIT = Decimal('1e-9')


def parse_input(text):
    """Exact rational of an authored input string (decimal, scientific or p/q)."""
    text = text.strip()
    if '/' in text:
        p, q = text.split('/')
        return Fr(int(p), int(q))
    return Fr(Decimal(text))


def dec_str(q, prec=40):
    """Round an exact rational once to prec significant digits."""
    if q == 0:
        return '0'
    with localcontext() as c:
        c.prec = prec
        return str(Decimal(q.numerator) / Decimal(q.denominator))


def to_dec(v, prec=60):
    with localcontext() as c:
        c.prec = prec
        if isinstance(v, Mag):
            if v.sq == 0:
                return Decimal(0)
            c.prec = prec + 10
            x = (Decimal(v.sq.numerator) / Decimal(v.sq.denominator)).sqrt()
            c.prec = prec
            return +x
        return Decimal(v.numerator) / Decimal(v.denominator)


def value_str(v):
    if isinstance(v, Mag):
        if v.sq == 0:
            return '0'
        with localcontext() as c:
            c.prec = 80
            x = (Decimal(v.sq.numerator) / Decimal(v.sq.denominator)).sqrt()
            c.prec = 40
            return str(+x)
    return dec_str(v)


def frac_str(q):
    """Exact printable form of a rational: plain decimal when finite, else p/q."""
    d = q.denominator
    k2 = k5 = 0
    while d % 2 == 0:
        d //= 2
        k2 += 1
    while d % 5 == 0:
        d //= 5
        k5 += 1
    if d != 1:
        return '%d/%d' % (q.numerator, q.denominator)
    k = max(k2, k5)
    n = q.numerator * 10 ** k // q.denominator
    sign = '-' if n < 0 else ''
    digits = str(abs(n))
    if k:
        digits = digits.rjust(k + 1, '0')
        s = digits[:-k] + '.' + digits[-k:]
    else:
        s = digits
    s = sign + s
    assert parse_input(s) == q
    return s


class Inp:
    """Records every authored input string. In represented mode each input is
    decoded exactly from its binary64 value (Decimal.from_float(float(x)))."""

    def __init__(self, represented=False):
        self.represented = represented
        self.log = {}

    def __call__(self, name, text):
        x = parse_input(text)
        if name in self.log:
            assert self.log[name] == text, name
        self.log[name] = text
        if self.represented:
            return Fr(Decimal.from_float(float(x)))
        return x

    def f(self, name, text):
        """The binary64 value of an input (for the binary64 defect models)."""
        self(name, text)
        return float(parse_input(text))


def rel_frame(e, yref):
    y = sub(yref, mul(dot(yref, e), e))
    n = isqrt_frac_exact(norm2(y))
    assert n is not None and n != 0, 'y_reference must give a rational unit y'
    y = mul(1 / n, y)
    z = cross(e, y)
    return e, y, z


class Builder:
    def __init__(self, inp, pi, flags):
        self.inp = inp
        self.pi = pi
        self.flags = set(flags)
        self.secs = {}
        self.m = {'nodes': {}, 'members': [], 'restraints': {}, 'springs': [], 'nloads': [],
                  'efforts': [], 'mloads': [], 'eigen': {}, 'thrust': {}, 'root': None,
                  'thrust_convention': 'effective'}
        self.info = {'loads': [], 'eigen': [], 'thrust': [], 'generated': []}

    # geometry and section -------------------------------------------------
    def node(self, name, x, y, z):
        self.m['nodes'][name] = (self.inp('X.%s' % name, x), self.inp('Y.%s' % name, y),
                                 self.inp('Z.%s' % name, z))
        if self.m['root'] is None:
            self.m['root'] = name

    def section(self, key, E='200e9', G='80e9', OD='0.2', t='0.01', mill=None, nominal=False):
        inp = self.inp
        Ev = inp('sec.%s.E' % key, E)
        Gv = inp('sec.%s.G' % key, G)
        ODv = inp('sec.%s.OD' % key, OD)
        tv = inp('sec.%s.wall' % key, t)
        mv = inp('sec.%s.mill_tolerance' % key, mill) if mill is not None else Fr(0)
        nominal = nominal or 'nominal_wall_all' in self.flags
        teff = tv if nominal else tv - mv
        ID = ODv - 2 * teff
        A = self.pi * (ODv ** 2 - ID ** 2) / 4
        I = self.pi * (ODv ** 4 - ID ** 4) / 64
        J = 2 * I
        sec = {'E': Ev, 'G': Gv, 'OD': ODv, 't': tv, 'mill': mv, 't_eff': teff, 'ID': ID,
               'A': A, 'I': I, 'J': J, 'EA': Ev * A, 'EI': Ev * I, 'GJ': Gv * J}
        self.secs[key] = sec
        return sec

    def member(self, name, i, j, sec):
        s = self.secs[sec]
        m = {'name': name, 'i': i, 'j': j, 'EA': s['EA'], 'EI': s['EI'], 'GJ': s['GJ'], 'sec': sec, 'yref': None}
        self.m['members'].append(m)
        return m

    def geom(self, name):
        m = next(x for x in self.m['members'] if x['name'] == name)
        return member_geometry(self.m, m)

    # supports ---------------------------------------------------------------
    def fix(self, node, dofs=DOFS, presc=None):
        presc = presc or {}
        d = self.m['restraints'].setdefault(node, {})
        for dof in dofs:
            if dof in presc:
                d[dof] = self.inp('presc.%s.%s' % (node, dof), presc[dof])
            else:
                d[dof] = Fr(0)

    def spring(self, node, dof, k):
        self.m['springs'].append({'node': node, 'dof': dof,
                                  'k': self.inp('k.%s.%s' % (node, dof), k)})

    # loads ------------------------------------------------------------------
    def nload(self, node, dof, text, tag=None):
        n = sum(1 for c in self.m['nloads'] if c['node'] == node and c['dof'] == dof)
        name = 'P.%s.%s' % (node, dof) + ('#%d' % (n + 1) if n else '')
        self.m['nloads'].append({'node': node, 'dof': dof, 'value': self.inp(name, text),
                                 'tag': tag or name})

    def effort(self, node, dof, text):
        assert dof in ('UX', 'UY', 'UZ')
        self.m['efforts'].append({'node': node, 'dof': dof,
                                  'value': self.inp('CE.%s.%s' % (node, dof), text)})

    def uload(self, member, q, a='0', b='1', local=None, tag=None, texts=None, source='authored'):
        """q: exact global vector (generated) or, with texts, authored strings.
        local: y_reference (exact integer vector) when q is in local axes."""
        n = sum(1 for c in self.m['mloads'] if c['member'] == member)
        key = 'w.%s#%d' % (member, n + 1)
        if texts is not None:
            q = tuple(self.inp('%s.%s' % (key, comp), t) for comp, t in zip(('x', 'y', 'z'), texts))
        av = self.inp('%s.a' % key, a) if a != '0' else Fr(0)
        bv = self.inp('%s.b' % key, b) if b != '1' else Fr(1)
        L, e = self.geom(member)
        rec = {'member': member, 'a': av, 'b': bv, 'tag': tag or key}
        if local is not None:
            mem = next(x for x in self.m['members'] if x['name'] == member)
            yr = tuple(Fr(c) for c in local)
            assert mem['yref'] in (None, yr), 'one y_reference per member'
            mem['yref'] = yr
            ex, ey, ez = rel_frame(e, yr)
            rec['local'] = (q, (ex, ey, ez))
            qg = add(add(mul(q[0], ex), mul(q[1], ey)), mul(q[2], ez))
        else:
            qg = q
        rec['q'] = qg
        self.m['mloads'].append(rec)
        self.info['loads'].append({'member': member, 'key': key, 'source': source,
                                   'frame': 'global' if local is None else 'local',
                                   'y_reference': None if local is None else [str(c) for c in local],
                                   'authored': (list(texts) if texts is not None else
                                                ['generated (see generated_intensities)'] if source.startswith('generated') else [frac_str(c) for c in q]),
                                   'global_intensity': ([frac_str(c) for c in qg] if not source.startswith('generated')
                                                              else [dec_str(c) for c in qg]),
                                   'extent': [frac_str(av), frac_str(bv)]})
        return rec

    def eigen(self, member, eps, how):
        self.m['eigen'][member] = self.m['eigen'].get(member, Fr(0)) + eps
        self.info['eigen'].append({'member': member, 'definition': how, 'eps_exact': frac_str(eps)})

    def thrust(self, member, p):
        sec = self.secs[next(x for x in self.m['members'] if x['name'] == member)['sec']]
        pv = self.inp('p.%s' % member, p)
        if 'thrust_steel_area' in self.flags:
            Fp = pv * sec['A']
        elif 'thrust_od_area' in self.flags:
            Fp = pv * self.pi * sec['OD'] ** 2 / 4
        else:
            Fp = pv * self.pi * sec['ID'] ** 2 / 4
        self.m['thrust'][member] = Fp
        self.info['thrust'].append({'member': member, 'pressure': p,
                                    'thrust_area': 'pi/4*ID^2, ID = OD - 2*t_eff',
                                    'F_p_over_pi': frac_str(Fp / self.pi)})
        return Fp

    def done(self):
        for mem in self.m['members']:
            if mem['yref'] is None:
                L, e = member_geometry(self.m, mem)
                for c in ((0, 0, 1), (0, 1, 0), (1, 0, 0)):
                    c = tuple(Fr(x) for x in c)
                    if norm2(cross(e, c)) != 0:
                        mem['yref'] = c
                        break
        self.m['_info'] = self.info
        return self.m


# ---------------------------------------------------------------------------
# Generic defect models applied to a built model.
# ---------------------------------------------------------------------------
import copy


def apply_defects(model, defects):
    m = copy.deepcopy(model)
    route = set()
    for d in defects:
        if d in ('lumped5050', 'lever'):
            new = []
            for ml in m['mloads']:
                L, e = member_geometry(m, next(x for x in m['members'] if x['name'] == ml['member']))
                mem = next(x for x in m['members'] if x['name'] == ml['member'])
                W = mul((ml['b'] - ml['a']) * L, ml['q'])
                c = (ml['a'] + ml['b']) / 2 if d == 'lever' else Fr(1, 2)
                for node, share in ((mem['i'], 1 - c), (mem['j'], c)):
                    for k3, dof in enumerate(('UX', 'UY', 'UZ')):
                        if W[k3]:
                            new.append({'node': node, 'dof': dof, 'value': share * W[k3], 'tag': 'lumped'})
            m['mloads'] = []
            m['nloads'] = m['nloads'] + new
        elif d == 'partial_as_full':
            for ml in m['mloads']:
                ml['a'], ml['b'] = Fr(0), Fr(1)
        elif d == 'local_as_global':
            for ml in m['mloads']:
                if 'local' in ml:
                    ml['q'] = ml['local'][0]
        elif d == 'global_as_local':
            for ml in m['mloads']:
                if 'local' not in ml:
                    mem = next(x for x in m['members'] if x['name'] == ml['member'])
                    L, e = member_geometry(m, mem)
                    ex, ey, ez = rel_frame(e, mem['yref'])
                    q = ml['q']
                    ml['q'] = add(add(mul(q[0], ex), mul(q[1], ey)), mul(q[2], ez))
        elif d == 'transposed_frame':
            for ml in m['mloads']:
                if 'local' in ml:
                    ql, (ex, ey, ez) = ml['local']
                    ml['q'] = (dot(ex, ql), dot(ey, ql), dot(ez, ql))
        elif d == 'eigen_sign':
            m['eigen'] = {k: -v for k, v in m['eigen'].items()}
        elif d == 'eigen_omit':
            m['eigen'] = {}
        elif d == 'pm_omit':
            for node, dd in m['restraints'].items():
                for dof in dd:
                    dd[dof] = Fr(0)
        elif d == 'pm_sign':
            for node, dd in m['restraints'].items():
                for dof in dd:
                    dd[dof] = -dd[dof]
        elif d == 'ce_sign':
            for c in m['efforts']:
                c['value'] = -c['value']
        elif d == 'ce_omit':
            m['efforts'] = []
        elif d == 'ce_restraint':
            for c in m['efforts']:
                m['restraints'].setdefault(c['node'], {})[c['dof']] = Fr(0)
            m['efforts'] = []
        elif d == 'thrust_sign':
            m['thrust'] = {k: -v for k, v in m['thrust'].items()}
        elif d == 'thrust_omit':
            m['thrust'] = {}
        elif d == 'thrust_no_fec':
            m['thrust_convention'] = 'true'
        elif d in ('no_fec', 'no_eigen_recovery'):
            route.add(d)
        elif d == 'nodal_dropped_small':
            pass
        else:
            raise KeyError(d)
    if route:
        m['defects'] = route
    return m


def solve(model):
    """Route A normally; route B when a route-level defect is modelled."""
    if model.get('defects'):
        return publish(model, solve_B(model))
    return publish(model, solve_A(model))


def equilibrium_residual(model, sol):
    """Sum of applied actions and support actions, force and moment about the origin."""
    Ft, Mt = Z3, Z3
    nodes = model['nodes']
    for node, (F, M) in total_nodal(model).items():
        Ft = add(Ft, F)
        Mt = add(Mt, add(M, cross(nodes[node], F)))
    for ml in model['mloads']:
        mem = next(x for x in model['members'] if x['name'] == ml['member'])
        L, e = member_geometry(model, mem)
        W = mul((ml['b'] - ml['a']) * L, ml['q'])
        P = add(nodes[mem['i']], mul((ml['a'] + ml['b']) * L / 2, e))
        Ft = add(Ft, W)
        Mt = add(Mt, cross(P, W))
    acts = list(sol['reac'].items()) + list(sol['springs'].items())
    for (node, dof), v in acts:
        vec = mul(v, unit_vec(dof))
        if dof[0] == 'U':
            Ft = add(Ft, vec)
            Mt = add(Mt, cross(nodes[node], vec))
        else:
            Mt = add(Mt, vec)
    return Ft, Mt


# ---------------------------------------------------------------------------
# Case catalogue.
# ---------------------------------------------------------------------------
CASES = []

Q3 = ((Fr(1, 3), Fr(2, 3), Fr(2, 3)), (Fr(2, 3), Fr(1, 3), Fr(-2, 3)), (Fr(-2, 3), Fr(2, 3), Fr(-1, 3)))
Q9 = ((Fr(1, 9), Fr(8, 9), Fr(4, 9)), (Fr(8, 9), Fr(1, 9), Fr(-4, 9)), (Fr(-4, 9), Fr(4, 9), Fr(-7, 9)))


def matvec(Q, v):
    return tuple(sum(Q[r][c] * v[c] for c in range(3)) for r in range(3))


def rot_texts(Q, texts):
    if Q is None:
        return tuple(texts)
    v = matvec(Q, tuple(parse_input(t) for t in texts))
    out = tuple(frac_str(c) for c in v)
    assert all('/' not in s for s in out), 'rotated input not a finite decimal'
    return out


def case(**kw):
    kw.setdefault('ncs', [])
    kw.setdefault('closed_forms', [])
    kw.setdefault('cancel', None)
    kw.setdefault('notes', [])
    kw.setdefault('generated', False)
    CASES.append(kw)
    return kw


NC_TEXT = {
    'lumped5050': 'NC-LUMPED-5050: each element load replaced by half its resultant at each end node, no fixed-end moments (the primitive-load 50/50 cross-check used as the solve); member actions then follow from the lumped nodal loads.',
    'lever': 'NC-LEVER-RULE: each partial-span load replaced by its lever-rule end forces W(1-c), Wc, no fixed-end moments (the sub-span preview tier used as the solve).',
    'no_fec': 'NC-NO-FEC: the correct nodal solution, but member end actions recovered as K_e u_e without subtracting the element-load fixed-end forces; station values then follow by statics from those uncorrected i-end actions and the span load.',
    'partial_as_full': 'NC-PARTIAL-AS-FULL: each partial-span load integrated over the full span.',
    'local_as_global': 'NC-LOCAL-AS-GLOBAL: the local load components applied as global components.',
    'global_as_local': 'NC-GLOBAL-AS-LOCAL: the global load components applied in the member local axes (x = e, y from the member y_reference, z = x cross y).',
    'transposed_frame': 'NC-TRANSPOSED-FRAME: local-to-global transform applied as its transpose (global = R^T q_local).',
    'eigen_sign': 'NC-EIGEN-SIGN: the thermal eigenstrain with its sign reversed.',
    'eigen_omit': 'NC-EIGEN-OMITTED: the thermal equivalent axial pair omitted entirely.',
    'no_eigen_recovery': 'NC-EIGEN-RECOVERY-OMITTED: the correct nodal solution, but N recovered as EA*ext/L without the -EA*eps* correction.',
    'pm_omit': 'NC-PM-OMITTED: every prescribed support value replaced by zero (the prescribed-motion term omitted).',
    'pm_sign': 'NC-PM-SIGN: every prescribed support value with its sign reversed.',
    'ce_sign': 'NC-CE-SIGN: the constant effort applied along the negative axis.',
    'ce_omit': 'NC-CE-OMITTED: the constant effort omitted.',
    'ce_restraint': 'NC-CE-AS-RESTRAINT: the constant-effort DOF modelled as a rigid restraint (a stiffness row) and the effort dropped.',
    'thrust_sign': 'NC-THRUST-SIGN: the cap pair with its sign reversed.',
    'thrust_omit': 'NC-THRUST-OMITTED: the cap pair omitted.',
    'thrust_no_fec': 'NC-THRUST-NO-FEC: the correct nodal solution, but N recovered without subtracting the cap pair (the true wall force N = EA*ext/L, not the effective force of the implemented theory).',
}


def nc(defect, text=None):
    return {'id': (text or NC_TEXT[defect]).split(':')[0], 'text': text or NC_TEXT[defect],
            'kind': 'defect', 'arg': [defect]}


def ncf(flag, text):
    return {'id': text.split(':')[0], 'text': text, 'kind': 'flags', 'arg': [flag]}


# ----- family 1: uniform element loads --------------------------------------

def b_cant(inp, pi, flags, end, loads, sec='N', mill=None, prop=False, extra=None):
    b = Builder(inp, pi, flags)
    b.section(sec, mill=mill)
    b.node('N0', '0', '0', '0')
    b.node('N1', *end)
    b.member('M1', 'N0', 'N1', sec)
    b.fix('N0')
    if prop:
        b.fix('N1', ('UX', 'UY', 'UZ'))
    for ld in loads:
        b.uload('M1', None, texts=ld[0], a=ld[1], b=ld[2], local=ld[3] if len(ld) > 3 else None)
    if extra:
        extra(b)
    return b.done()


STD_UDL_NC = [nc('lumped5050'), nc('no_fec')]
PART_NC = [nc('lumped5050'), nc('lever'), nc('partial_as_full'), nc('no_fec')]
LOCAL_NC = [nc('local_as_global'), nc('transposed_frame')]

case(id='RF-ELOAD-UDL-CANT-FULL-GLOB-AX', family='RF-ELOAD-UDL', item=1,
     purpose='Cantilever along X (L = 4 m), full-span uniform global load q = (300, -2000, 1200) N/m: axial, and bending in both planes.',
     build=lambda inp, pi, fl: b_cant(inp, pi, fl, ('4', '0', '0'), [(('300', '-2000', '1200'), '0', '1')]),
     ncs=STD_UDL_NC,
     closed_forms=[('u.N1.UY', 'q_y L^4/(8EI)', lambda m: Fr(-2000) * 4 ** 4 / (8 * m['members'][0]['EI'])),
                   ('th.N1.RZ', 'q_y L^3/(6EI)', lambda m: Fr(-2000) * 4 ** 3 / (6 * m['members'][0]['EI'])),
                   ('u.N1.UX', 'q_x L^2/(2EA)', lambda m: Fr(300) * 16 / (2 * m['members'][0]['EA'])),
                   ('N.M1.i', 'q_x L (tension)', lambda m: Fr(1200))])

case(id='RF-ELOAD-UDL-CANT-PART-GLOB-AX', family='RF-ELOAD-UDL', item=1,
     purpose='The same cantilever, the same load on the partial span [0.2, 0.7] (fractions not exact in binary64).',
     build=lambda inp, pi, fl: b_cant(inp, pi, fl, ('4', '0', '0'), [(('300', '-2000', '1200'), '0.2', '0.7')]),
     ncs=[nc('global_as_local')] + PART_NC)

case(id='RF-ELOAD-UDL-CANT-FULL-LOC-Q3', family='RF-ELOAD-UDL', item=1,
     purpose='Cantilever along Q3 e_x = (1,2,-2)/3 (L = 3 m) with a full-span LOCAL load (300, -2000, 1200) N/m, y_reference (2,1,2): local axes are the columns of Q3, so it equals the X-axis cantilever of length 3 m rotated by Q3 (asserted).',
     build=lambda inp, pi, fl: b_cant(inp, pi, fl, ('1', '2', '-2'), [(('300', '-2000', '1200'), '0', '1', (2, 1, 2))]),
     ncs=LOCAL_NC + STD_UDL_NC,
     invariance={'rotation': 'Q3', 'base': lambda inp, pi, fl: b_cant(inp, pi, fl, ('3', '0', '0'), [(('300', '-2000', '1200'), '0', '1')])})

case(id='RF-ELOAD-UDL-CANT-PART-LOC-122', family='RF-ELOAD-UDL', item=1,
     purpose='Cantilever N0 -> (2,4,4) (e = (1,2,2)/3, L = 6 m), local load (-150, 900, -2500) N/m on [0.25, 1], y_reference (2,1,-2), z = (-2,2,-1)/3.',
     build=lambda inp, pi, fl: b_cant(inp, pi, fl, ('2', '4', '4'), [(('-150', '900', '-2500'), '0.25', '1', (2, 1, -2))]),
     ncs=LOCAL_NC + PART_NC)

case(id='RF-ELOAD-UDL-CANT-PART-GLOB-345', family='RF-ELOAD-UDL', item=1,
     purpose='Cantilever N0 -> (3,4,0) (L = 5 m), global load (0, 500, -1800) N/m on [0.1, 0.6]: the Y component has axial and transverse parts on the skew member.',
     build=lambda inp, pi, fl: b_cant(inp, pi, fl, ('3', '4', '0'), [(('0', '500', '-1800'), '0.1', '0.6')]),
     ncs=[nc('global_as_local')] + PART_NC)

case(id='RF-ELOAD-UDL-PROP-FULL-AX', family='RF-ELOAD-UDL', item=1,
     purpose='Propped cantilever (N0 fixed, N1 = (6,0,0) translation-pinned), full-span q = (400, -3000, 1000) N/m. Statically indeterminate in both bending planes and axially.',
     build=lambda inp, pi, fl: b_cant(inp, pi, fl, ('6', '0', '0'), [(('400', '-3000', '1000'), '0', '1')], prop=True),
     ncs=STD_UDL_NC,
     closed_forms=[('R.N1.UY', '-3 q_y L/8', lambda m: -3 * Fr(-3000) * 6 / 8),
                   ('R.N0.RZ', '-q_y L^2/8 (support moment on the structure)', lambda m: -Fr(-3000) * 36 / 8),
                   ('R.N1.UX', '-q_x L/2 (axial, equal stiffness halves)', lambda m: -Fr(400) * 6 / 2),
                   ('th.N1.RZ', '-q_y L^3/(48EI)', lambda m: Fr(3000) * 216 / (48 * m['members'][0]['EI']))])

case(id='RF-ELOAD-UDL-PROP-PART-AX', family='RF-ELOAD-UDL', item=1,
     purpose='The propped cantilever with (0, -3000, 0) N/m on [0, 0.5] and (0, 1200, 800) N/m on [0.75, 1].',
     build=lambda inp, pi, fl: b_cant(inp, pi, fl, ('6', '0', '0'), [(('0', '-3000', '0'), '0', '0.5'), (('0', '1200', '800'), '0.75', '1')], prop=True),
     ncs=PART_NC)


def b_cont2(inp, pi, fl, Q=None, loads=True, nodal=None, gen=None, presc=None, extra_loads=None, spans=('9', '27')):
    b = Builder(inp, pi, fl)
    b.section('G' if gen else 'N', mill='0.00125' if gen else None)
    s = 'G' if gen else 'N'
    b.node('N0', '0', '0', '0')
    b.node('N1', *rot_texts(Q, (spans[0], '0', '0')))
    b.node('N2', *rot_texts(Q, (spans[1], '0', '0')))
    b.member('M1', 'N0', 'N1', s)
    b.member('M2', 'N1', 'N2', s)
    b.fix('N0', presc=(presc or {}).get('N0'))
    b.fix('N1', ('UX', 'UY', 'UZ'), presc=(presc or {}).get('N1'))
    b.fix('N2', ('UX', 'UY', 'UZ'), presc=(presc or {}).get('N2'))
    if loads:
        b.uload('M1', None, texts=rot_texts(Q, ('0', '-2700', '0')))
        b.uload('M2', None, texts=rot_texts(Q, ('0', '-1800', '900')), a='0.25', b='0.75')
    for ld in (extra_loads or []):
        b.uload(ld[0], None, texts=rot_texts(Q, ld[1]), a=ld[2], b=ld[3])
    if nodal:
        for node, dof, t in nodal:
            b.nload(node, dof, t)
    if gen:
        gen(b)
    return b.done()


case(id='RF-ELOAD-UDL-CONT2-AX', family='RF-ELOAD-UDL', item=1,
     purpose='Two-span continuous beam along X: N0 fixed, N1 = (9,0,0) and N2 = (27,0,0) translation-pinned. Span 1 carries (0, -2700, 0) N/m; span 2 carries (0, -1800, 900) N/m on [0.25, 0.75].',
     build=lambda inp, pi, fl: b_cont2(inp, pi, fl), ncs=PART_NC)

case(id='RF-ELOAD-UDL-CONT2-Q9', family='RF-ELOAD-UDL', item=1,
     purpose='RF-ELOAD-UDL-CONT2-AX rotated by Q9 (nodes (1,8,-4) and (3,24,-12); global loads rotated exactly). The expectations equal the rotated base solution (asserted).',
     build=lambda inp, pi, fl: b_cont2(inp, pi, fl, Q=Q9), ncs=PART_NC,
     invariance={'rotation': 'Q9', 'base': lambda inp, pi, fl: b_cont2(inp, pi, fl)})


def b_lframe(inp, pi, fl, far_fixed=False, sec='N', mill=None, extra=None, sec2=None,
             n1=('4', '0', '0'), n2=('4', '3', '0')):
    b = Builder(inp, pi, fl)
    b.section(sec, mill=mill)
    s2 = sec
    if sec2:
        b.section(sec2[0], **sec2[1])
        s2 = sec2[0]
    b.node('N0', '0', '0', '0')
    b.node('N1', *n1)
    b.node('N2', *n2)
    b.member('M1', 'N0', 'N1', sec)
    b.member('M2', 'N1', 'N2', s2)
    b.fix('N0')
    if far_fixed:
        b.fix('N2')
    if extra:
        extra(b)
    return b.done()


def _lf_udl(b):
    b.uload('M1', None, texts=('0', '0', '-1500'))
    b.uload('M2', None, texts=('0', '0', '-1500'), a='0', b='0.5')
    b.uload('M2', None, texts=('800', '0', '0'))


case(id='RF-ELOAD-UDL-LFRAME-3D', family='RF-ELOAD-UDL', item=1,
     purpose='L-frame N0 (fixed) -> N1 (4,0,0) -> N2 (4,3,0) (free). M1: (0,0,-1500) N/m full; M2: (0,0,-1500) N/m on [0, 0.5] plus (800,0,0) N/m full. M2 out-of-plane loads twist M1.',
     build=lambda inp, pi, fl: b_lframe(inp, pi, fl, extra=_lf_udl), ncs=PART_NC)


def b_weight(inp, pi, fl):
    b = Builder(inp, pi, fl)
    b.section('N')
    b.node('N0', '0', '0', '0')
    b.node('N1', '6', '-3', '6')
    b.node('N2', '10', '-3', '6')
    b.member('M1', 'N0', 'N1', 'N')
    b.member('M2', 'N1', 'N2', 'N')
    b.fix('N0')
    b.fix('N1', ('UX', 'UY', 'UZ'))
    b.uload('M1', None, texts=('0', '0', '-512.5'), tag='weight')
    b.uload('M2', None, texts=('0', '0', '-512.5'), tag='weight')
    return b.done()


case(id='RF-ELOAD-UDL-WEIGHT-SKEW', family='RF-ELOAD-UDL', item=1,
     purpose='Weight entered as a user-given intensity (0, 0, -512.5) N/m, per unit member length, on a skew member N0 -> N1 (6,-3,6) (L = 9 m, N1 translation-pinned) and an overhang N1 -> N2 (10,-3,6). This also covers generated self-weight, which is an authoring-time stored intensity (manager Q2, ROOT D-14 scope).',
     build=b_weight, ncs=STD_UDL_NC)


# ----- family 3: thermal eigen axial load ------------------------------------

def eps_legacy(b, key, alpha, dT):
    a = b.inp('th.%s.alpha' % key, alpha)
    d = b.inp('th.%s.DeltaT' % key, dT)
    return a * d, 'legacy: eps* = alpha*DeltaT = %s*%s' % (alpha, dT)


def eps_resolved(b, key, Tm, Ti, T, a_i, a_T, fit=None, L=None):
    """0.4.0 engineering-secant datum ratio, optionally composed with a fit."""
    Tm_, Ti_, T_ = (b.inp('th.%s.%s' % (key, n), v) for n, v in (('T_datum', Tm), ('T_install', Ti), ('T_operating', T)))
    ai = b.inp('th.%s.alpha_sec(T_install)' % key, a_i)
    aT = b.inp('th.%s.alpha_sec(T_operating)' % key, a_T)
    lam_i = 1 + ai * (Ti_ - Tm_)
    lam_T = 1 + aT * (T_ - Tm_)
    lam_th = lam_T / lam_i
    lam_fit = Fr(1)
    how = ('0.4.0 resolved (engineering secant): lambda(T) = 1 + alpha_sec(T)(T - T_datum); '
           'lambda_thermal = lambda(%s)/lambda(%s) with T_datum %s, alpha_sec %s and %s' % (T, Ti, Tm, a_T, a_i))
    if 'legacy_form' in b.flags:
        return aT * (T_ - Ti_), how + ' [defect: alpha_sec(T)*(T - T_install)]'
    if 'subtract_dilations' in b.flags:
        return lam_T - lam_i, how + ' [defect: lambda(T) - lambda(T_install)]'
    if fit is not None:
        dL = b.inp('fit.%s.delta_L' % key, fit)
        lam_fit = 1 + dL / L
        how += '; lambda_fit = 1 + delta_L_fit/L with delta_L_fit = %s m' % fit
        if 'fit_additive' in b.flags:
            return (lam_fit - 1) + (lam_th - 1), how + ' [defect: additive strains]'
        if 'fit_omit' in b.flags:
            lam_fit = Fr(1)
    return lam_fit * lam_th - 1, how + '; eps* = lambda_fit*lambda_thermal - 1'


def b_axial(inp, pi, fl, end, eps_fn, spring=None, far='fixed', thrust=None, sec='N', mill=None):
    b = Builder(inp, pi, fl)
    b.section(sec, mill=mill)
    b.node('N0', '0', '0', '0')
    b.node('N1', *end)
    b.member('M1', 'N0', 'N1', sec)
    b.fix('N0')
    if far == 'fixed':
        b.fix('N1')
    elif far == 'spring':
        b.fix('N1', ('UY', 'UZ', 'RX', 'RY', 'RZ'))
        b.spring('N1', 'UX', spring)
    if eps_fn:
        eps, how = eps_fn(b)
        b.eigen('M1', eps, how)
    if thrust:
        b.thrust('M1', thrust)
    return b.done()


TH_NC = [nc('eigen_sign'), nc('eigen_omit'), nc('no_eigen_recovery')]
RES_NC = [ncf('legacy_form', 'NC-ALPHA-TIMES-INTERVAL: eps* = alpha_sec(T)*(T - T_install), ignoring the datum ratio.'),
          ncf('subtract_dilations', 'NC-SUBTRACT-DILATIONS: eps* = lambda(T) - lambda(T_install) instead of their ratio minus one.')]


def lost_soft(model):
    """NC-LOST-SOFT: the spring after one rounded binary64 addition into EA/L."""
    m = copy.deepcopy(model)
    mem = m['members'][0]
    L, e = member_geometry(m, mem)
    a = float(mem['EA'] / L)
    for s in m['springs']:
        k = float(s['k'])
        s['k'] = Fr(a + k) - Fr(a)
    return m


def ncc(fn, text):
    return {'id': text.split(':')[0], 'text': text, 'kind': 'custom', 'arg': fn}


LOST_SOFT = ncc(lost_soft, 'NC-LOST-SOFT: the axial spring k after one rounded binary64 addition into the member axial stiffness EA/L (k_eff = fl(fl(EA/L) + fl(k)) - fl(EA/L)).')

case(id='RF-ELOAD-TH-FF-LEG-AX', family='RF-ELOAD-TH', item=3,
     purpose='Fixed-fixed member along X (L = 6 m), legacy thermal alpha = 1.2e-5 1/K, DeltaT = 75 K. Every displacement is zero, so the translation and rotation classes are all-zero with derived scales.',
     build=lambda inp, pi, fl: b_axial(inp, pi, fl, ('6', '0', '0'), lambda b: eps_legacy(b, 'M1', '1.2e-5', '75')),
     ncs=TH_NC,
     closed_forms=[('N.M1.mid', '-E A alpha DeltaT', lambda m: -m['members'][0]['EA'] * Fr(9, 10000)),
                   ('R.N0.UX', '+E A alpha DeltaT', lambda m: m['members'][0]['EA'] * Fr(9, 10000))])

case(id='RF-ELOAD-TH-FF-RES-122', family='RF-ELOAD-TH', item=3,
     purpose='Fixed-fixed skew member N0 -> (2,4,4) (L = 6 m), 0.4.0 resolved thermal: datum 20, installation 30, operating 230 (degC), alpha_sec 11.5e-6 at 30 and 14e-6 at 230 (1/K).',
     build=lambda inp, pi, fl: b_axial(inp, pi, fl, ('2', '4', '4'), lambda b: eps_resolved(b, 'M1', '20', '30', '230', '11.5e-6', '14e-6')),
     ncs=TH_NC + RES_NC,
     closed_forms=[('N.M1.i', '-E A ((1 + 14e-6*210)/(1 + 11.5e-6*10) - 1)',
                    lambda m: -m['members'][0]['EA'] * (Fr(100294, 100000) / Fr(1000115, 1000000) - 1))])

for tag, k, note in (('r1e-06', '200', 'k/(EA/L) = 1.0e-6'), ('r1e-12', '0.0002', 'k/(EA/L) = 1.0e-12')):
    case(id='RF-ELOAD-TH-SPRING-LEG-%s' % tag, family='RF-ELOAD-TH', item=3,
         purpose='Member along X (L = 6 m), N0 fixed, N1 on a soft axial spring k = %s N/m (%s), other N1 DOFs fixed; legacy thermal alpha = 1.2e-5, DeltaT = 75. N = -k u is proportional to the soft spring.' % (k, note),
         build=(lambda k: lambda inp, pi, fl: b_axial(inp, pi, fl, ('6', '0', '0'), lambda b: eps_legacy(b, 'M1', '1.2e-5', '75'), spring=k, far='spring'))(k),
         ncs=TH_NC + [LOST_SOFT],
         closed_forms=[('u.N1.UX', 'EA eps/(EA/L + k)', (lambda k: lambda m: (m['members'][0]['EA'] * Fr(9, 10000)) / (m['members'][0]['EA'] / 6 + parse_input(k)))(k)),
                       ('N.M1.mid', '-k u', (lambda k: lambda m: -parse_input(k) * (m['members'][0]['EA'] * Fr(9, 10000)) / (m['members'][0]['EA'] / 6 + parse_input(k)))(k))])

case(id='RF-ELOAD-TH-SPRING-RES-r1e-08', family='RF-ELOAD-TH', item=3,
     purpose='The soft-spring member with k = 0.02 N/m (k/(EA/L) = 1.0e-8) and 0.4.0 resolved thermal (datum 20, installation 30, operating 230 degC; alpha_sec 11.5e-6 and 14e-6).',
     build=lambda inp, pi, fl: b_axial(inp, pi, fl, ('6', '0', '0'), lambda b: eps_resolved(b, 'M1', '20', '30', '230', '11.5e-6', '14e-6'), spring='0.02', far='spring'),
     ncs=TH_NC + RES_NC + [LOST_SOFT])


def b_serial(inp, pi, fl):
    b = Builder(inp, pi, fl)
    b.section('N')
    b.section('B', OD='0.25', t='0.012')
    b.node('N0', '0', '0', '0')
    b.node('N1', '4', '0', '0')
    b.node('N2', '10', '0', '0')
    b.member('M1', 'N0', 'N1', 'N')
    b.member('M2', 'N1', 'N2', 'B')
    b.fix('N0')
    b.fix('N2')
    eps, how = eps_resolved(b, 'M1', '20', '30', '230', '11.5e-6', '14e-6', fit='-0.002', L=Fr(4))
    b.eigen('M1', eps, how)
    return b.done()


case(id='RF-ELOAD-TH-SERIAL-RES-FIT', family='RF-ELOAD-TH', item=3,
     purpose='Two members in series, N0 (fixed) -> N1 (4,0,0) free -> N2 (10,0,0) fixed; M1 (N section) resolved thermal composed with a fit delta_L = -0.002 m (cut short); M2 (OD 0.25, wall 0.012) cold. The restraint of M1 comes from M2.',
     build=b_serial,
     ncs=TH_NC + RES_NC + [ncf('fit_additive', 'NC-FIT-ADDITIVE: eps* = eps_fit + eps_thermal instead of lambda_fit*lambda_thermal - 1.'),
                           ncf('fit_omit', 'NC-FIT-OMITTED: the fit length change dropped.')],
     closed_forms=[('N.M1.mid', '-(eps* L1)/(L1/EA1 + L2/EA2)', lambda m: -(m['eigen']['M1'] * 4) / (Fr(4) / m['members'][0]['EA'] + Fr(6) / m['members'][1]['EA']))])


def _lf_th(kind):
    def extra(b):
        if kind == 'leg':
            eps, how = eps_legacy(b, 'M1', '1.2e-5', '150')
            b.eigen('M1', eps, how)
        else:
            eps, how = eps_resolved(b, 'M1', '20', '30', '230', '11.5e-6', '14e-6')
            b.eigen('M1', eps, how)
            eps, how = eps_resolved(b, 'M2', '20', '30', '180', '11.5e-6', '13.4e-6')
            b.eigen('M2', eps, how)
    return extra


case(id='RF-ELOAD-TH-LFRAME-LEG', family='RF-ELOAD-TH', item=3,
     purpose='L-frame N0 (fixed) -> N1 (6,0,0) -> N2 (6,4,0) (fixed). M1 heated (legacy alpha = 1.2e-5, DeltaT = 150 K), M2 cold: the restraint of M1 comes from the bending of M2 (and M1).',
     build=lambda inp, pi, fl: b_lframe(inp, pi, fl, far_fixed=True, extra=_lf_th('leg'), n1=('6', '0', '0'), n2=('6', '4', '0')),
     ncs=TH_NC)

case(id='RF-ELOAD-TH-LFRAME-RES', family='RF-ELOAD-TH', item=3,
     purpose='The same L-frame with both members resolved (M1: operating 230 degC, alpha_sec 14e-6; M2: operating 180 degC, alpha_sec 13.4e-6; both installed at 30 degC, alpha_sec 11.5e-6, datum 20 degC). M2 has its own state modulus E = 180 GPa (G = 72 GPa).',
     build=lambda inp, pi, fl: b_lframe(inp, pi, fl, far_fixed=True, extra=_lf_th('res'), n1=('6', '0', '0'), n2=('6', '4', '0'),
                                        sec2=('H', {'E': '180e9', 'G': '72e9'})),
     ncs=TH_NC + RES_NC)


# ----- family 4: pressure thrust ---------------------------------------------

PT_NC = [nc('thrust_no_fec'), nc('thrust_sign'), nc('thrust_omit'),
         ncf('thrust_steel_area', 'NC-THRUST-STEEL-AREA: F_p = p * A_steel instead of p * pi/4 * ID^2.'),
         ncf('thrust_od_area', 'NC-THRUST-OD-AREA: F_p = p * pi/4 * OD^2.')]

case(id='RF-ELOAD-PT-FREE-AX', family='RF-ELOAD-PT', item=4,
     purpose='Pressure thrust p = 2.5e6 Pa on a cantilever along X (L = 6 m, N section, ID 0.18 m), free end. F_p = p*pi/4*ID^2 = 20250*pi N. Effective N = 0 everywhere; the tip moves F_p L/EA; the anchor carries nothing.',
     build=lambda inp, pi, fl: b_axial(inp, pi, fl, ('6', '0', '0'), None, far='free', thrust='2.5e6'),
     ncs=PT_NC,
     closed_forms=[('u.N1.UX', 'F_p L/EA', lambda m: m['thrust']['M1'] * 6 / m['members'][0]['EA'])])

case(id='RF-ELOAD-PT-FF-AX', family='RF-ELOAD-PT', item=4,
     purpose='The same thrust on a fixed-fixed member: zero displacement, effective N = -F_p, reactions -F_p at N0 and +F_p at N1 along X.',
     build=lambda inp, pi, fl: b_axial(inp, pi, fl, ('6', '0', '0'), None, far='fixed', thrust='2.5e6'),
     ncs=PT_NC,
     closed_forms=[('N.M1.mid', '-F_p', lambda m: -m['thrust']['M1']), ('R.N0.UX', '+F_p', lambda m: m['thrust']['M1'])])

case(id='RF-ELOAD-PT-FREE-122-MILL', family='RF-ELOAD-PT', item=4,
     purpose='Thrust p = 2.5e6 Pa on a free-ended skew cantilever N0 -> (2,4,4) with wall 0.01 m and mill tolerance 0.00125 m: effective wall 0.00875 m for the stiffness and the thrust area (ID = 0.1825 m).',
     build=lambda inp, pi, fl: b_axial(inp, pi, fl, ('2', '4', '4'), None, far='free', thrust='2.5e6', sec='M', mill='0.00125'),
     ncs=PT_NC + [ncf('nominal_wall_all', 'NC-NOMINAL-WALL: nominal wall 0.01 m instead of the effective wall, for the section and the thrust area.')])


def _pt_lf(b):
    b.thrust('M1', '2.5e6')
    b.thrust('M2', '2.5e6')


case(id='RF-ELOAD-PT-LFRAME-FF', family='RF-ELOAD-PT', item=4,
     purpose='Thrust p = 2.5e6 Pa on both legs of the fixed-fixed L-frame N0 -> N1 (6,0,0) -> N2 (6,4,0). The caps meeting at the corner N1 leave F_p (1,-1,0) unbalanced, which bends both legs.',
     build=lambda inp, pi, fl: b_lframe(inp, pi, fl, far_fixed=True, extra=_pt_lf, n1=('6', '0', '0'), n2=('6', '4', '0')),
     ncs=PT_NC)


def _pt_th_spring(inp, pi, fl):
    return b_axial(inp, pi, fl, ('6', '0', '0'), lambda b: eps_legacy(b, 'M1', '1.2e-5', '75'), spring='2e5', far='spring', thrust='2.5e6')


case(id='RF-ELOAD-PT-TH-SPRING', family='RF-ELOAD-PT', item=4,
     purpose='Thrust (2.5e6 Pa) and legacy thermal (1.2e-5, 75 K) together on the member with a soft axial spring k = 2e5 N/m at N1: N = EA(ext/L - eps*) - F_p = -k u.',
     build=_pt_th_spring, ncs=PT_NC[:3] + TH_NC)


# ----- family 5: constant-effort supports ------------------------------------

CE_NC = [nc('ce_sign'), nc('ce_omit'), nc('ce_restraint')]


def b_ce(inp, pi, fl, which):
    b = Builder(inp, pi, fl)
    b.section('N')
    b.node('N0', '0', '0', '0')
    if which == 'alone':
        b.node('N1', '5', '0', '0')
        b.member('M1', 'N0', 'N1', 'N')
        b.fix('N0')
        b.effort('N1', 'UY', '4500')
        return b.done()
    b.node('N1', '4', '0', '0')
    b.node('N2', '8', '0', '0')
    b.member('M1', 'N0', 'N1', 'N')
    b.member('M2', 'N1', 'N2', 'N')
    b.fix('N0')
    b.nload('N1', 'UY', '-1000')
    b.nload('N1', 'RZ', '500')
    b.nload('N2', 'UZ', '-2000')
    b.nload('N2', 'UX', '700')
    b.effort('N2', 'UZ', '3000')
    b.effort('N1', 'UY', '250')
    return b.done()


case(id='RF-ELOAD-CE-ALONE', family='RF-ELOAD-CE', item=5,
     purpose='Constant-effort support alone: cantilever N0 -> N1 (5,0,0), effort 4500 N along +UY at N1 (no stiffness, no restraint row).',
     build=lambda inp, pi, fl: b_ce(inp, pi, fl, 'alone'), ncs=CE_NC,
     closed_forms=[('u.N1.UY', 'F L^3/(3EI)', lambda m: Fr(4500) * 125 / (3 * m['members'][0]['EI']))])

case(id='RF-ELOAD-CE-NODAL', family='RF-ELOAD-CE', item=5,
     purpose='Constant efforts with nodal loads: chain N0 -> N1 (4,0,0) -> N2 (8,0,0); nodal loads N1: UY -1000 N, RZ 500 N*m; N2: UZ -2000 N, UX 700 N; efforts +UZ 3000 N at N2 and +UY 250 N at N1.',
     build=lambda inp, pi, fl: b_ce(inp, pi, fl, 'nodal'), ncs=CE_NC)


def _ce_cancel(inp, pi, fl, G):
    b = Builder(inp, pi, fl)
    b.section('N')
    b.node('N0', '0', '0', '0')
    b.node('N1', '1', '0', '0')
    b.node('N2', '2', '0', '0')
    b.member('M1', 'N0', 'N1', 'N')
    b.member('M2', 'N1', 'N2', 'N')
    b.fix('N0')
    Gv = parse_input(G)
    Gm, n = '-' + G, '0.3'
    fl = b.flags
    if 'net_state' in fl:
        b.nload('N2', 'UY', n)
        return b.done()
    if 'gross_state' in fl:
        b.nload('N2', 'UY', Gm)
        return b.done()
    fo = [f for f in fl if f.startswith('float:')]
    if fo:
        vals = {'G-': b.inp.f('P.N2.UY', Gm), 'n': b.inp.f('P.N2.UY#2', n), 'G+': b.inp.f('CE.N2.UY', G)}
        acc = None
        for tok in fo[0][6:].split(','):
            acc = vals[tok] if acc is None else acc + vals[tok]
        b.m['nloads'].append({'node': 'N2', 'dof': 'UY', 'value': Fr(acc), 'tag': 'float-summed net'})
        return b.done()
    b.nload('N2', 'UY', Gm, tag='G-')
    if 'drop_small' not in fl:
        b.nload('N2', 'UY', n, tag='n')
    b.effort('N2', 'UY', G)
    return b.done()


def cancel_ncs(orders, authored, extra):
    out = []
    for o in orders:
        out.append(ncf('float:' + o, 'NC-FLOAT-SUM-%s: the net contribution obtained by summing the binary64 contributions left to right in the order (%s)%s.'
                       % (o.replace(',', '').replace('+', 'p').replace('-', 'm'), o, ' (the authored order)' if o == authored else '')))
    out.append(ncf('drop_small', 'NC-SMALL-DROPPED: the small contribution dropped.'))
    return out + extra


for G, tag in (('1e5', 'G1e5'), ('1e8', 'G1e8')):
    case(id='RF-ELOAD-CE-CANCEL-%s' % tag, family='RF-ELOAD-CE', item=5,
         purpose='Constant effort nearly cancelling a nodal load at the same node: cantilever N0 -> N1 (1,0,0) -> N2 (2,0,0); at N2 UY the nodal load -%s N, the nodal load 0.3 N and the effort +%s N (authored in that order). The exact net is 0.3 N.' % (G, G),
         build=(lambda G: lambda inp, pi, fl: _ce_cancel(inp, pi, fl, G))(G),
         ncs=cancel_ncs(['G-,n,G+', 'G-,G+,n', 'n,G-,G+'], 'G-,n,G+', CE_NC),
         cancel={'net': 'the 0.3 N nodal load alone', 'gross': 'the nodal load -G alone', 'gross_over_net': dec_str(parse_input(G) / Fr(3, 10), 12)})


# ----- family 6: generated equivalent-static loads (D-14) --------------------

GEN_INPUTS = (('gen.rho_metal', '7000'), ('gen.rho_contents', '900'), ('gen.rho_insulation', '150'),
              ('gen.t_insulation', '0.03'), ('gen.g', '9.80665'), ('gen.g_factor.X', '0.3'),
              ('gen.g_factor.Y', '-0.15'), ('gen.g_factor.Z', '-0.2'), ('gen.wind_pressure', '520'),
              ('gen.shape_factor', '0.65'))
GEN_TEXT = dict(GEN_INPUTS)


def gin(b, name):
    return b.inp(name, GEN_TEXT[name])


def gfl(b, name):
    return b.inp.f(name, GEN_TEXT[name])


def mass_per_length(b, sec):
    s = b.secs[sec]
    rm, rc, ri, ti = gin(b, 'gen.rho_metal'), gin(b, 'gen.rho_contents'), gin(b, 'gen.rho_insulation'), gin(b, 'gen.t_insulation')
    OD = s['OD']
    ID = OD - 2 * s['t'] if 'nominal_wall_mass' in b.flags else s['ID']
    pi = b.pi
    m = rm * pi / 4 * (OD ** 2 - ID ** 2) + rc * pi / 4 * ID ** 2
    if 'no_ins_mass' not in b.flags:
        m += ri * pi / 4 * ((OD + 2 * ti) ** 2 - OD ** 2)
    return m


def mass_per_length_bin64(b, sec):
    """The D-14 hazard: binary64 left-to-right evaluation of the record's formula."""
    log = b.inp.log
    OD = float(parse_input(log['sec.%s.OD' % sec]))
    t = float(parse_input(log['sec.%s.wall' % sec]))
    mill = float(parse_input(log['sec.%s.mill_tolerance' % sec])) if 'sec.%s.mill_tolerance' % sec in log else 0.0
    ID = OD - 2 * (t - mill)
    rm, rc, ri, ti = gfl(b, 'gen.rho_metal'), gfl(b, 'gen.rho_contents'), gfl(b, 'gen.rho_insulation'), gfl(b, 'gen.t_insulation')
    Dins = OD + 2 * ti
    A_metal = math.pi / 4 * (OD * OD - ID * ID)
    A_cont = math.pi / 4 * (ID * ID)
    A_ins = math.pi / 4 * (Dins * Dins - OD * OD)
    return A_metal * rm + A_cont * rc + A_ins * ri


BIN64_TEXT = ('NC-BIN64-PRODUCT: the generated intensity taken as the binary64 left-to-right product of the binary64 inputs '
              '(the D-14 hazard): seismic m\' = A_metal*rho_m + A_contents*rho_c + A_ins*rho_i with each A = math.pi/4*(D1*D1 - D2*D2), '
              'ID = OD - 2*(t - m), then w = m\' * g_factor * g; wind w = p * Cs * (OD + 2*t_ins); the rest of the model exact.')


def seismic(b, member, sec, axes='XYZ', a='0', bb='1'):
    q = []
    fq = []
    mp = mass_per_length(b, sec)
    g = gin(b, 'gen.g')
    bin64 = 'bin64' in b.flags
    if bin64:
        mf = mass_per_length_bin64(b, sec)
        gf_ = gfl(b, 'gen.g')
    for ax in 'XYZ':
        if ax in axes:
            q.append(mp * gin(b, 'gen.g_factor.%s' % ax) * g)
            if bin64:
                fq.append(Fr(mf * gfl(b, 'gen.g_factor.%s' % ax) * gf_))
        else:
            q.append(Fr(0))
            if bin64:
                fq.append(Fr(0))
    qq = tuple(fq) if 'bin64' in b.flags else tuple(q)
    rec = b.uload(member, qq, a=a, b=bb, source='generated seismic', tag='seismic')
    b.info['generated'].append({'member': member, 'kind': 'seismic', 'axes': axes,
                                'exact': [c for c in q], 'mass_per_length': mp,
                                'bin64_product': [float(c) for c in fq] if fq else None})
    return rec


def wind(b, member, sec, axis='Y', extents=(('0', '1'),)):
    s = b.secs[sec]
    p, Cs, ti = gin(b, 'gen.wind_pressure'), gin(b, 'gen.shape_factor'), gin(b, 'gen.t_insulation')
    D = s['OD'] if 'no_ins_diam' in b.flags else s['OD'] + 2 * ti
    w = p * Cs * D
    if 'bin64' in b.flags:
        log = b.inp.log
        ODf = float(parse_input(log['sec.%s.OD' % sec]))
        wf = gfl(b, 'gen.wind_pressure') * gfl(b, 'gen.shape_factor') * (ODf + 2 * gfl(b, 'gen.t_insulation'))
        wv = Fr(wf)
    else:
        wv = w
    d = unit_vec('U' + axis)
    if 'projected_wind' in b.flags:
        L, e = b.geom(member)
        c = dot(e, d)
        fac = isqrt_frac_exact(1 - c * c)
        assert fac is not None
        wv = wv * fac
    for a, bb in extents:
        b.uload(member, mul(wv, d), a=a, b=bb, source='generated wind', tag='wind')
    b.info['generated'].append({'member': member, 'kind': 'wind', 'axes': axis, 'exact': [w * c for c in d],
                                'extents': [list(x) for x in extents],
                                'bin64_product': [float(wv) * float(c) for c in d] if 'bin64' in b.flags else None})


GEN_NC_S = [ncf('bin64', BIN64_TEXT),
            ncf('no_ins_mass', 'NC-NO-INSULATION-MASS: insulation omitted from the mass per length.'),
            ncf('nominal_wall_mass', 'NC-NOMINAL-WALL-MASS: the mass per length over the nominal wall instead of the mill-tolerance effective wall (stiffness kept effective).'),
            ncf('nominal_wall_all', 'NC-NOMINAL-WALL: the nominal wall instead of the effective wall everywhere (section and mass).')]
GEN_NC_W = [ncf('bin64', BIN64_TEXT),
            ncf('no_ins_diam', 'NC-NO-INSULATION-DIAMETER: exposed diameter taken as OD, insulation omitted.')]
WIND_UNMARKED = ncf('wind_unmarked', 'NC-WIND-UNMARKED: wind also applied, full span, to the unmarked span(s).')


def b_gen_cant(inp, pi, fl):
    return b_cant(inp, pi, fl, ('4', '0', '0'), [], sec='G', mill='0.00125', extra=lambda b: seismic(b, 'M1', 'G'))


case(id='RF-ELOAD-GEN-SEIS-CANT-AX', family='RF-ELOAD-GEN', item=6, generated=True,
     purpose='Generated seismic load on a cantilever along X (L = 4 m). Section OD 0.2, wall 0.01, mill tolerance 0.00125 (effective wall 0.00875, ID 0.1825); densities 7000 / 900 / 150 kg/m^3, insulation 0.03 m, g = 9.80665 m/s^2, g-factors (0.3, -0.15, -0.2). Intensity per axis = g_factor * g * m\'.',
     build=b_gen_cant, ncs=GEN_NC_S + STD_UDL_NC)


def _gen_cont_seis(b):
    seismic(b, 'M1', 'G')
    seismic(b, 'M2', 'G')


case(id='RF-ELOAD-GEN-SEIS-CONT2-Q9', family='RF-ELOAD-GEN', item=6, generated=True,
     purpose='Generated seismic load (all three axes) on both spans of the Q9-rotated two-span beam (nodes (0,0,0), (1,8,-4), (3,24,-12); N0 fixed, N1 and N2 translation-pinned), effective-wall section G. Global-axis intensities on skew members, per unit member length.',
     build=lambda inp, pi, fl: b_cont2(inp, pi, fl, Q=Q9, loads=False, gen=_gen_cont_seis),
     ncs=GEN_NC_S + STD_UDL_NC)


def _gen_lf(b):
    seismic(b, 'M1', 'G')
    seismic(b, 'M2', 'G')
    wind(b, 'M1', 'G', 'Y')
    if 'wind_unmarked' in b.flags:
        wind(b, 'M2', 'G', 'Y')


case(id='RF-ELOAD-GEN-SEIS-WIND-LFRAME-FF', family='RF-ELOAD-GEN', item=6, generated=True,
     purpose='Generated seismic (three axes, both legs) and wind (+Y, pressure 520 Pa, shape factor 0.65, exposed diameter OD + 2 t_ins = 0.26 m, marked on M1 only) on the fixed-fixed L-frame N0 -> N1 (6,0,0) -> N2 (6,4,0), section G.',
     build=lambda inp, pi, fl: b_lframe(inp, pi, fl, far_fixed=True, sec='G', mill='0.00125', extra=_gen_lf, n1=('6', '0', '0'), n2=('6', '4', '0')),
     ncs=GEN_NC_S + GEN_NC_W[1:] + [WIND_UNMARKED, nc('lumped5050'), nc('no_fec')])


def _gen_wind_marked(b):
    wind(b, 'M1', 'G', 'Y')
    if 'wind_unmarked' in b.flags:
        wind(b, 'M2', 'G', 'Y')


def _gen_wind_sub(b):
    wind(b, 'M2', 'G', 'Y', extents=(('0.2', '0.7'), ('0.8', '1')))
    if 'wind_unmarked' in b.flags:
        wind(b, 'M1', 'G', 'Y')


case(id='RF-ELOAD-GEN-WIND-MARKED-CONT2', family='RF-ELOAD-GEN', item=6, generated=True,
     purpose='Generated wind (+Y) on the marked span M1 only of a two-span beam along X: N0 fixed, N1 (5,0,0) and N2 (8,0,0) translation-pinned; M2 unmarked.',
     build=lambda inp, pi, fl: b_cont2(inp, pi, fl, loads=False, gen=_gen_wind_marked, spans=('5', '8')),
     ncs=GEN_NC_W + [WIND_UNMARKED] + STD_UDL_NC)

case(id='RF-ELOAD-GEN-WIND-SUBSPAN', family='RF-ELOAD-GEN', item=6, generated=True,
     purpose='Sub-span wind exposure: on the same two-span beam, M2 is exposed on the disjoint extents [0.2, 0.7] and [0.8, 1.0]; M1 unmarked. Consistent partial-span loads with fixed-end moments.',
     build=lambda inp, pi, fl: b_cont2(inp, pi, fl, loads=False, gen=_gen_wind_sub, spans=('5', '8')),
     ncs=GEN_NC_W + [WIND_UNMARKED] + PART_NC)


def b_gen_wind_skew(inp, pi, fl):
    return b_cant(inp, pi, fl, ('0', '3', '4'), [], sec='G', mill='0.00125', prop=True, extra=lambda b: wind(b, 'M1', 'G', 'Y'))


case(id='RF-ELOAD-GEN-WIND-SKEW-PROP', family='RF-ELOAD-GEN', item=6, generated=True,
     purpose='Generated wind (+Y) on a skew propped member N0 (fixed) -> N1 (0,3,4) (translation-pinned), e = (0,3,4)/5 at cosine 3/5 to the wind: intensity p*Cs*D_exp per unit member length, no projection (manager Q4).',
     build=b_gen_wind_skew,
     ncs=GEN_NC_W + [ncf('projected_wind', 'NC-PROJECTED-WIND: intensity multiplied by the projected-length factor sqrt(1 - (e.d)^2) = 4/5.')] + STD_UDL_NC)


# ----- family 7: prescribed support motion (0.4.0) ---------------------------

PM_NC = [nc('pm_omit'), nc('pm_sign')]

case(id='RF-ELOAD-PM-SINGLE-FF', family='RF-ELOAD-PM', item=7,
     purpose='Fixed-fixed member along X (L = 5 m); N1 settles UY = -0.012 m (prescribed). V = 12EI delta/L^3, M = 6EI delta/L^2.',
     build=lambda inp, pi, fl: (lambda b: (b.section('N'), b.node('N0', '0', '0', '0'), b.node('N1', '5', '0', '0'),
                                           b.member('M1', 'N0', 'N1', 'N'), b.fix('N0'), b.fix('N1', presc={'UY': '-0.012'}), b.done())[-1])(Builder(inp, pi, fl)),
     ncs=PM_NC,
     closed_forms=[('R.N1.UY', '12 EI delta/L^3', lambda m: 12 * m['members'][0]['EI'] * Fr(-12, 1000) / 125),
                   ('R.N0.RZ', '-6 EI delta/L^2', lambda m: -6 * m['members'][0]['EI'] * Fr(-12, 1000) / 25)])

case(id='RF-ELOAD-PM-ROT-PROP', family='RF-ELOAD-PM', item=7,
     purpose='Prescribed rotations at the fixed end of a propped cantilever: N0 RZ = 0.002 rad and RY = -0.001 rad; N1 (6,0,0) translation-pinned. The prop reaction is 3EI theta/L^2 in each plane.',
     build=lambda inp, pi, fl: (lambda b: (b.section('N'), b.node('N0', '0', '0', '0'), b.node('N1', '6', '0', '0'),
                                           b.member('M1', 'N0', 'N1', 'N'), b.fix('N0', presc={'RZ': '0.002', 'RY': '-0.001'}),
                                           b.fix('N1', ('UX', 'UY', 'UZ')), b.done())[-1])(Builder(inp, pi, fl)),
     ncs=PM_NC,
     closed_forms=[('R.N1.UY', '-3 EI theta_z/L^2', lambda m: -3 * m['members'][0]['EI'] * Fr(2, 1000) / 36)])

case(id='RF-ELOAD-PM-DIFF-CONT2', family='RF-ELOAD-PM', item=7,
     purpose='Differential settlement of a two-span beam: N0 fixed, N1 (5,0,0) and N2 (8,0,0) translation-pinned, with N1 UY = -0.01 m, N2 UY = -0.004 m and N2 UZ = 0.003 m prescribed.',
     build=lambda inp, pi, fl: b_cont2(inp, pi, fl, loads=False, spans=('5', '8'),
                                       presc={'N1': {'UY': '-0.01'}, 'N2': {'UY': '-0.004', 'UZ': '0.003'}}),
     ncs=PM_NC)

case(id='RF-ELOAD-PM-ELOAD-CONT2', family='RF-ELOAD-PM', item=7,
     purpose='The differential settlement combined with element loads in the same state: (0, -2500, 0) N/m on M1 and (0, -1500, 800) N/m on [0.25, 0.75] of M2.',
     build=lambda inp, pi, fl: b_cont2(inp, pi, fl, loads=False, spans=('5', '8'),
                                       presc={'N1': {'UY': '-0.01'}, 'N2': {'UY': '-0.004', 'UZ': '0.003'}},
                                       extra_loads=[('M1', ('0', '-2500', '0'), '0', '1'), ('M2', ('0', '-1500', '800'), '0.25', '0.75')]),
     ncs=PM_NC + PART_NC)

case(id='RF-ELOAD-PM-SKEW-122-FF', family='RF-ELOAD-PM', item=7,
     purpose='Fixed-fixed skew member N0 -> N1 (2,4,4); N1 prescribed UX = 0.001, UY = -0.002, UZ = 0.0005 m and RX = 0.0003 rad (axial, transverse and torsional motion at once).',
     build=lambda inp, pi, fl: (lambda b: (b.section('N'), b.node('N0', '0', '0', '0'), b.node('N1', '2', '4', '4'),
                                           b.member('M1', 'N0', 'N1', 'N'), b.fix('N0'),
                                           b.fix('N1', presc={'UX': '0.001', 'UY': '-0.002', 'UZ': '0.0005', 'RX': '0.0003'}), b.done())[-1])(Builder(inp, pi, fl)),
     ncs=PM_NC)


# ----- family 8: cancellation across element loads (S11-F) -------------------

def _femc(inp, pi, fl, s):
    b = Builder(inp, pi, fl)
    b.section('N')
    b.node('S0', '0', '0', '0')
    b.node('S1', '2', '0', '0')
    b.node('S2', '5', '0', '0')
    b.member('M1', 'S0', 'S1', 'N')
    b.member('M2', 'S1', 'S2', 'N')
    b.fix('S0')
    b.fix('S1', ('UX', 'UY', 'UZ'))
    b.fix('S2')
    wA, wB = frac_str(9 * s), frac_str(4 * s + Fr(1, 4))
    Mz = '-0.2'
    fl = b.flags
    wAv = b.inp('w.M1#1.y', '-' + wA)
    wBv = b.inp('w.M2#1.y', '-' + wB)
    Mzv = b.inp('P.S1.RZ', Mz)
    fem = -wAv * 4 / 12 + wBv * 9 / 12   # z-moment at S1: +w_A L_A^2/12 - w_B L_B^2/12 with w = -q_y
    if 'net_state' in fl:
        b.m['nloads'].append({'node': 'S1', 'dof': 'RZ', 'value': fem + Mzv, 'tag': 'net'})
        return b.done()
    if 'gross_state' in fl:
        b.uload('M1', (Fr(0), wAv, Fr(0)))
        return b.done()
    b.uload('M1', (Fr(0), wAv, Fr(0)), tag='A')
    b.uload('M2', (Fr(0), wBv, Fr(0)), tag='B')
    b.info['loads'][0]['authored'] = ['0', '-' + wA, '0']
    b.info['loads'][1]['authored'] = ['0', '-' + wB, '0']
    fo = [f for f in fl if f.startswith('float:')]
    if fo:
        fA = -b.inp.f('w.M1#1.y', '-' + wA) * 2.0 * 2.0 / 12.0
        fB = b.inp.f('w.M2#1.y', '-' + wB) * 3.0 * 3.0 / 12.0
        fn = b.inp.f('P.S1.RZ', Mz)
        vals = {'A': fA, 'B': fB, 'n': fn}
        acc = None
        for tok in fo[0][6:].split(','):
            acc = vals[tok] if acc is None else acc + vals[tok]
        b.m['nloads'].append({'node': 'S1', 'dof': 'RZ', 'value': Fr(acc) - fem, 'tag': 'float-summed net'})
        return b.done()
    if 'drop_small' not in fl:
        b.m['nloads'].append({'node': 'S1', 'dof': 'RZ', 'value': Mzv, 'tag': 'n'})
    return b.done()


for s, tag in ((Fr(12500), 'G1e5'), (Fr(1250000), 'G1e7'), (Fr(12500000), 'G1e8')):
    ratio = 3 * s / Fr(3875, 10000)
    case(id='RF-ELOAD-CANCEL-FEM-%s' % tag, family='RF-ELOAD-CANCEL', item=8,
         purpose=('Opposing fixed-end moments at a shared node: S0 (0,0,0) and S2 (5,0,0) fixed, S1 (2,0,0) translation-pinned; spans L_A = 2 m and L_B = 3 m '
                  'carry w_A = %s and w_B = %s N/m, both along -Y, and S1 carries M_z = -0.2 N*m. The fixed-end moments at S1, +w_A L_A^2/12 = %s and '
                  '-w_B L_B^2/12 = -(%s + 0.1875), nearly cancel: net = -0.1875 - 0.2 = -0.3875 N*m. Gross/net = %s.')
         % (frac_str(9 * s), frac_str(4 * s + Fr(1, 4)), frac_str(3 * s), frac_str(3 * s), dec_str(ratio, 8)),
         build=(lambda s: lambda inp, pi, fl: _femc(inp, pi, fl, s))(s),
         ncs=cancel_ncs(['A,B,n', 'A,n,B', 'n,A,B'], 'A,B,n', [nc('lumped5050'), nc('no_fec')]),
         cancel={'net': 'the net nodal moment at S1 (both fixed-end moments plus M_z) on unloaded spans',
                 'gross': 'the span-A load alone', 'gross_over_net': dec_str(ratio, 12),
                 'float_model': 'fixed-end moments in binary64 as w*L*L/12.0 (w_A*2.0*2.0/12.0 and w_B*3.0*3.0/12.0) plus M_z, summed in the stated order'},
         closed_forms=[('th.S1.RZ', 'net/(4EI/L_A + 4EI/L_B)', lambda m: Fr(-3875, 10000) / (4 * m['members'][0]['EI'] / 2 + 4 * m['members'][0]['EI'] / 3))])


def _seis_ws(pi):
    b = Builder(Inp(), pi, ())
    b.section('G', mill='0.00125')
    return mass_per_length(b, 'G') * gin(b, 'gen.g_factor.Z') * gin(b, 'gen.g')


def _seisc(inp, pi, fl, D):
    b = Builder(inp, pi, fl)
    b.section('G', mill='0.00125')
    b.node('N0', '0', '0', '0')
    b.node('N1', '4', '0', '0')
    b.member('M1', 'N0', 'N1', 'G')
    b.fix('N0')
    fl = b.flags
    ws = mass_per_length(b, 'G') * gin(b, 'gen.g_factor.Z') * gin(b, 'gen.g')
    Dv = b.inp('w.M1#2.z', D)
    if 'net_state' in fl:
        b.uload('M1', (Fr(0), Fr(0), ws + Dv), tag='net')
        return b.done()
    if 'gross_state' in fl:
        seismic(b, 'M1', 'G', axes='Z')
        return b.done()
    if 'drop_small' in fl:
        return b.done()
    if 'float_sum_bin64' in fl or 'float_sum_exactgen' in fl:
        if 'float_sum_bin64' in fl:
            wsf = mass_per_length_bin64(b, 'G') * gfl(b, 'gen.g_factor.Z') * gfl(b, 'gen.g')
        else:
            wsf = float(ws)
        b.uload('M1', (Fr(0), Fr(0), Fr(wsf + b.inp.f('w.M1#2.z', D))), tag='float-summed net')
        return b.done()
    seismic(b, 'M1', 'G', axes='Z')
    b.uload('M1', None, texts=('0', '0', D), tag='authored')
    return b.done()


SEIS_WS = None
for ratio, tag in ((10 ** 5, 'G1e5'), (10 ** 7, 'G1e7'), (10 ** 8, 'G1e8')):
    if SEIS_WS is None:
        SEIS_WS = _seis_ws(PI_Q)
    D = dec_str(-SEIS_WS * (1 - Fr(1, ratio)), 20)
    case(id='RF-ELOAD-CANCEL-SEIS-%s' % tag, family='RF-ELOAD-CANCEL', item=8, generated=True,
         purpose=('A generated seismic load cancelled by an authored uniform load of opposite sign: cantilever along X (L = 4 m), section G; seismic g-factor Z = -0.2 '
                  '(w_s = -0.2 * g * m\', about -%s N/m) plus an authored (0, 0, %s) N/m. Gross/net about 1e%d; every value is proportional to the net intensity.')
         % (dec_str(-SEIS_WS, 8), D, int(round(math.log10(ratio)))),
         build=(lambda D: lambda inp, pi, fl: _seisc(inp, pi, fl, D))(D),
         ncs=[ncf('bin64', BIN64_TEXT + ' The authored load is kept exact.'),
              ncf('float_sum_bin64', 'NC-FLOAT-SUM-BIN64: the net intensity as fl(w_s_bin64 + fl(D)), the binary64 product summed left to right with the binary64 authored load.'),
              ncf('float_sum_exactgen', 'NC-FLOAT-SUM: the net intensity as fl(fl(w_s) + fl(D)), the correctly rounded generated intensity summed in binary64 with the authored load.'),
              ncf('drop_small', 'NC-NET-DROPPED: the net intensity lost (zero response).')],
         cancel={'net': 'the net uniform intensity w_s + D alone', 'gross': 'the generated seismic load alone',
                 'gross_over_net': dec_str(SEIS_WS / (SEIS_WS + parse_input(D)), 12)})


# ----- family 9: one combination ---------------------------------------------

COMB_B = [('N1', 'RX', '800'), ('N1', 'RY', '-1200'), ('N1', 'RZ', '-900'), ('N2', 'RZ', '2500'), ('N2', 'RY', '700')]

case(id='RF-ELOAD-COMB-B-NODAL', family='RF-ELOAD-COMB', item=9,
     purpose='Nodal-load case B on the RF-ELOAD-UDL-CONT2-AX structure: N1 moments (800, -1200, -900) N*m, N2 moments (0, 700, 2500) N*m.',
     build=lambda inp, pi, fl: b_cont2(inp, pi, fl, loads=False, nodal=COMB_B))


def _neg(t):
    return t[1:] if t.startswith('-') else '-' + t


case(id='RF-ELOAD-COMB-SUM', family='RF-ELOAD-COMB', item=9,
     purpose='Combination A + B, with A = RF-ELOAD-UDL-CONT2-AX (element loads) and B = RF-ELOAD-COMB-B-NODAL. Expectations are the exact sum of the component solutions (asserted), with Mb the magnitude of the combined moment vector.',
     build=lambda inp, pi, fl: b_cont2(inp, pi, fl, loads=True, nodal=COMB_B),
     combination=('RF-ELOAD-UDL-CONT2-AX', 'RF-ELOAD-COMB-B-NODAL', 1))

case(id='RF-ELOAD-COMB-DIFF', family='RF-ELOAD-COMB', item=9,
     purpose='Combination A - B of the same component cases.',
     build=lambda inp, pi, fl: b_cont2(inp, pi, fl, loads=True, nodal=[(n, d, _neg(t)) for n, d, t in COMB_B]),
     combination=('RF-ELOAD-UDL-CONT2-AX', 'RF-ELOAD-COMB-B-NODAL', -1))


# ---------------------------------------------------------------------------
# Scales, comparison, negative controls and the run.
# ---------------------------------------------------------------------------

def D60(q):
    return to_dec(q, 60)


def class_of(key):
    p = key.split('.')
    if p[0] == 'u':
        return 'translation'
    if p[0] == 'th':
        return 'rotation'
    if p[0] == 'R':
        return 'force' if p[2][0] == 'U' else 'moment'
    if p[0] == 'S':
        return 'force' if p[2][0] == 'F' else 'moment'
    if p[0] == 'N':
        return 'force'
    if p[0] in ('T', 'Mb'):
        return 'moment'
    if p[0] == 'tw':
        return 'twist'
    if p[0] == 'ext':
        return 'extension'
    raise KeyError(key)


def magnitudes(out):
    """Per-class magnitudes: nodal vectors as norms, member values as absolutes."""
    groups = {}
    for k, v in out.items():
        p = k.split('.')
        c = class_of(k)
        if p[0] in ('u', 'th') or p[0] == 'R':
            g = (c, p[0], p[1])
        else:
            g = (c, k)
        groups.setdefault(g, []).append(v)
    mags = {}
    for g, vs in groups.items():
        s = sum((v.sq if isinstance(v, Mag) else v * v for v in vs), Fr(0))
        mags.setdefault(g[0], []).append(to_dec(Mag(s), 60))
    return mags


def model_refs(model):
    Ls, LEA, LGJ, L3EI, L2EI = [], [], [], [], []
    for m in model['members']:
        L, e = member_geometry(model, m)
        Ls.append(L)
        LEA.append(L / m['EA'])
        LGJ.append(L / m['GJ'])
        L3EI.append(L ** 3 / (3 * m['EI']))
        L2EI.append(L ** 2 / (2 * m['EI']))
    Fref, Mref = [Fr(0)], [Fr(0)]
    for node, (F, M) in total_nodal(model).items():
        Fref.append(Mag(norm2(F)))
        Mref.append(Mag(norm2(M)))
    for ml in model['mloads']:
        mem = next(x for x in model['members'] if x['name'] == ml['member'])
        L, e = member_geometry(model, mem)
        Fref.append(Mag(norm2(ml['q']) * ((ml['b'] - ml['a']) * L) ** 2))
    epsL = [Fr(0)]
    for m in model['members']:
        eps = model['eigen'].get(m['name'], Fr(0))
        if eps:
            L, e = member_geometry(model, m)
            Fref.append(abs(m['EA'] * eps))
            epsL.append(abs(eps) * L)
    pt, pr = [Fr(0)], [Fr(0)]
    for node, dd in model['restraints'].items():
        pt.append(Mag(sum((dd.get(d, Fr(0)) ** 2 for d in ('UX', 'UY', 'UZ')), Fr(0))))
        pr.append(Mag(sum((dd.get(d, Fr(0)) ** 2 for d in ('RX', 'RY', 'RZ')), Fr(0))))

    def mx(xs):
        return max(to_dec(x, 60) if isinstance(x, Mag) else abs(D60(x)) for x in xs)
    return {'L_c': mx(Ls), 'F_ref': mx(Fref), 'M_ref': mx(Mref), 'L/EA': mx(LEA), 'L/GJ': mx(LGJ),
            'L^3/3EI': mx(L3EI), 'L^2/2EI': mx(L2EI), 'presc_t': mx(pt), 'presc_r': mx(pr), 'eps_L': mx(epsL)}


def g25(x):
    with localcontext() as c:
        c.prec = 25
        return str(+x)


def class_scales(out, model):
    mags = magnitudes(out)
    r = model_refs(model)
    sc, how = {}, {}
    for c, vs in mags.items():
        m = max(vs)
        if m > 0:
            sc[c] = m
            how[c] = 'largest magnitude in the class (nodal vectors as norms, member values as absolutes)'
    Lc = r['L_c']
    present = set(mags)
    if 'force' in present and 'force' not in sc:
        if 'moment' in sc:
            sc['force'], how['force'] = sc['moment'] / Lc, 'all zero: moment scale / L_c (L_c = %s m)' % g25(Lc)
        else:
            sc['force'] = max(r['F_ref'], r['M_ref'] / Lc)
            how['force'] = 'all zero, moments all zero: max(F_ref, M_ref/L_c) with F_ref = %s N, M_ref = %s N*m, L_c = %s m' % (g25(r['F_ref']), g25(r['M_ref']), g25(Lc))
    if 'moment' in present and 'moment' not in sc:
        if 'force' in mags and max(mags['force']) > 0:
            sc['moment'], how['moment'] = sc['force'] * Lc, 'all zero: force scale * L_c (L_c = %s m)' % g25(Lc)
        else:
            sc['moment'] = max(r['M_ref'], r['F_ref'] * Lc)
            how['moment'] = 'all zero, forces all zero: max(M_ref, F_ref*L_c) with F_ref = %s N, M_ref = %s N*m, L_c = %s m' % (g25(r['F_ref']), g25(r['M_ref']), g25(Lc))
    if 'translation' not in sc and 'rotation' in sc:
        sc['translation'], how['translation'] = sc['rotation'] * Lc, 'all zero: rotation scale * L_c (L_c = %s m)' % g25(Lc)
    elif 'rotation' not in sc and 'translation' in sc:
        sc['rotation'], how['rotation'] = sc['translation'] / Lc, 'all zero: translation scale / L_c (L_c = %s m)' % g25(Lc)
    elif 'rotation' not in sc and 'translation' not in sc:
        moment_nonzero = 'moment' in mags and max(mags['moment']) > 0
        cands = [('force scale * max(L/EA)', sc['force'] * r['L/EA']), ('max prescribed translation', r['presc_t']),
                 ('max prescribed rotation * L_c', r['presc_r'] * Lc), ('max |eps*| L', r['eps_L'])]
        if moment_nonzero:
            cands.append(('moment scale * max(L^2/(2EI))', sc['moment'] * r['L^2/2EI']))
        nm, v = max(cands, key=lambda t: t[1])
        sc['translation'] = v
        how['translation'] = ('translations and rotations all zero: the largest of force scale*max(L/EA) (axial compliance), '
                              'max prescribed translation, max prescribed rotation*L_c, max |eps*|L and, when moments are nonzero, '
                              'moment scale*max(L^2/(2EI)); governed by %s' % nm)
        sc['rotation'] = v / Lc
        how['rotation'] = 'translations and rotations all zero: translation scale / L_c (L_c = %s m)' % g25(Lc)
    if 'twist' not in sc:
        sc['twist'], how['twist'] = sc['moment'] * r['L/GJ'], 'all zero: moment scale * max(L/GJ)'
    if 'extension' not in sc:
        sc['extension'], how['extension'] = sc['force'] * r['L/EA'], 'all zero: force scale * max(L/EA)'
    return sc, how


def absd(v):
    return abs(to_dec(v, 60))


def ratio_of(exp, obs, scale):
    """|obs - exp| / (1e-9 max(|exp|, scale)); > 1 fails the criterion."""
    e, o = to_dec(exp, 60), to_dec(obs, 60)
    den = CRIT * max(abs(e), scale)
    d = abs(o - e)
    if den == 0:
        return Decimal(0) if d == 0 else Decimal('Infinity')
    return d / den


def compare(exp, obs, scales):
    worst, wk, nfail = Decimal(0), None, 0
    for k, v in exp.items():
        o = obs.get(k, Fr(0) if not isinstance(v, Mag) else Mag(Fr(0)))
        r = ratio_of(v, o, scales[k])
        if r > 1:
            nfail += 1
        if r > worst:
            worst, wk = r, k
    return {'discriminates': nfail > 0, 'values_failing': nfail, 'worst_key': wk,
            'worst_ratio_to_criterion': g6(worst),
            'worst_observed': value_str(obs.get(wk, Fr(0))) if wk else None,
            'worst_expected': value_str(exp[wk]) if wk else None}


def g6(x):
    if x == Decimal('Infinity'):
        return 'inf'
    if x == 0:
        return '0'
    with localcontext() as c:
        c.prec = 6
        return str(+x)


def rot_check(base_out, out, Q, model):
    """Assert out equals base_out rotated by Q (vectors rotate, invariants equal)."""
    for nm in model['nodes']:
        for pref, dofs in (('u', ('UX', 'UY', 'UZ')), ('th', ('RX', 'RY', 'RZ')), ('R', ('UX', 'UY', 'UZ')), ('R', ('RX', 'RY', 'RZ'))):
            keys = ['%s.%s.%s' % (pref, nm, d) for d in dofs]
            if not all(k in out for k in keys):
                assert not any(k in out for k in keys) or pref != 'R', 'partial restraint set is not rotation invariant'
                continue
            vb = tuple(base_out[k] for k in keys)
            vr = tuple(out[k] for k in keys)
            assert matvec(Q, vb) == vr, ('rotation mismatch', nm, pref)
    for k, v in out.items():
        if k.split('.')[0] in ('N', 'T', 'Mb', 'tw', 'ext', 'S'):
            assert base_out[k] == v, ('invariant mismatch', k)


def solve_both(model):
    sa = solve_A(model)
    sb = solve_B(model)
    oa, ob = publish(model, sa), publish(model, sb)
    bad = [k for k in oa if oa[k] != ob[k]]
    assert not bad, ('route A and route B disagree', bad[:5])
    Ft, Mt = equilibrium_residual(model, sa)
    assert Ft == Z3 and Mt == Z3, 'equilibrium residual not zero'
    return oa


def model_json(model, inp):
    info = model['_info']
    mem = []
    for m in model['members']:
        L, e = member_geometry(model, m)
        mem.append({'name': m['name'], 'i': m['i'], 'j': m['j'], 'section': m['sec'], 'L_m': frac_str(L),
                    'y_reference': [frac_str(c) for c in m['yref']],
                    'EA_over_pi': frac_str(m['EA'] / PI_Q) if (m['EA'] / PI_Q).denominator < 10 ** 60 else None,
                    'EI_over_pi': frac_str(m['EI'] / PI_Q) if (m['EI'] / PI_Q).denominator < 10 ** 60 else None,
                    'GJ_over_pi': frac_str(m['GJ'] / PI_Q) if (m['GJ'] / PI_Q).denominator < 10 ** 60 else None})
    return {'nodes': {n: [frac_str(c) for c in v] for n, v in model['nodes'].items()},
            'members': mem,
            'restraints': {n: {d: frac_str(v) for d, v in dd.items()} for n, dd in model['restraints'].items()},
            'springs': [{'node': s['node'], 'dof': s['dof'], 'k': frac_str(s['k'])} for s in model['springs']],
            'nodal_loads_in_authored_order': [{'node': c['node'], 'dof': c['dof'], 'value': frac_str(c['value'])} for c in model['nloads']],
            'constant_efforts': [{'node': c['node'], 'dof': c['dof'], 'value': frac_str(c['value'])} for c in model['efforts']],
            'element_loads': info['loads'], 'eigen': info['eigen'], 'thrust': info['thrust'],
            'root_for_route_A': model['root']}


def gen_json(mI, mR, mF):
    rows = []
    for gi, gr, gf in zip(mI['_info']['generated'], mR['_info']['generated'], mF['_info']['generated'] if mF else [None] * 99):
        for k3, ax in enumerate('XYZ'):
            x = gi['exact'][k3]
            if x == 0:
                continue
            xr = gr['exact'][k3]
            row = {'member': gi['member'], 'kind': gi['kind'], 'axis': ax,
                   'intended_exact': dec_str(x), 'intended_over_pi': frac_str(x / PI_Q) if gi['kind'] == 'seismic' else None,
                   'represented_exact_product': dec_str(xr),
                   'represented_rel_diff': g6(abs(D60(xr - x) / D60(x)))}
            if gf is not None and gf.get('bin64_product'):
                fv = Fr(gf['bin64_product'][k3])
                row['binary64_product'] = repr(gf['bin64_product'][k3])
                row['binary64_product_rel_diff'] = g6(abs(D60(fv - x) / D60(x)))
            if gi['kind'] == 'seismic':
                row['mass_per_length_intended'] = dec_str(gi['mass_per_length'])
                row['mass_per_length_over_pi'] = frac_str(gi['mass_per_length'] / PI_Q)
            else:
                row['extents'] = gi['extents']
            rows.append(row)
    return rows


def run_case(cs, results):
    t0 = time.time()
    inpI = Inp(False)
    mI = cs['build'](inpI, PI_Q, ())
    expI = solve_both(mI)
    for key, text, fn in cs['closed_forms']:
        if fn is not None:
            assert expI[key] == fn(mI), ('closed form', cs['id'], key)
    if cs.get('invariance'):
        inv = cs['invariance']
        mb = inv['base'](Inp(False), PI_Q, ())
        ob = solve_both(mb)
        rot_check(ob, expI, {'Q3': Q3, 'Q9': Q9}[inv['rotation']], mI)
    comb = cs.get('combination')
    if comb:
        oa, obb = results[comb[0]]['_exp'], results[comb[1]]['_exp']
        for k, v in expI.items():
            if not isinstance(v, Mag):
                assert v == oa[k] + comb[2] * obb[k], ('combination not linear', k)
    sc, how = class_scales(expI, mI)
    scales = {k: sc[class_of(k)] for k in expI}
    # represented inputs
    inpR = Inp(True)
    mR = cs['build'](inpR, PI_Q, ())
    expR = solve_both(mR)
    diffs = {k: ratio_of(expI[k], expR[k], scales[k]) * CRIT for k in expI}
    fin = max(diffs.values())
    wk = max(diffs, key=lambda k: diffs[k])
    basis = 'represented' if fin > CRIT else 'intended'
    exp = expR if basis == 'represented' else expI
    mB = mR if basis == 'represented' else mI
    if basis == 'represented':
        sc, how = class_scales(expR, mR)
        scales = {k: sc[class_of(k)] for k in expR}
    rec = {'id': cs['id'], 'family': cs['family'], 'brief_item': cs['item'], 'purpose': cs['purpose'],
           'inputs': dict(sorted(inpI.log.items())), 'model': model_json(mI, inpI),
           'method': ('Route A (derivation): exact Euler-Bernoulli tree integration from the fully fixed root %s, element loads, eigenstrains and '
                      'prescribed values carried exactly, redundant restraints and springs by the force method. Route B (cross-check): exact direct '
                      'stiffness with consistent (Hermite) equivalent loads and fixed-end-corrected recovery. The two agree exactly for every value; '
                      'global equilibrium residual exactly zero.') % mI['root'],
           'closed_form_checks': [{'key': k, 'expression': t} for k, t, f in cs['closed_forms'] if f is not None],
           'reference_accuracy': 'exact rationals with pi -> PI_Q (|PI_Q - pi| < 1e-189); each nonzero value rounded once to 40 significant digits; bending magnitudes are square roots of exact rationals evaluated at 80 digits; zeros exact.',
           'basis': basis}
    cancel = cs.get('cancel')
    if cancel:
        mN = cs['build'](Inp(basis == 'represented'), PI_Q, ('net_state',))
        mG = cs['build'](Inp(basis == 'represented'), PI_Q, ('gross_state',))
        oN, oG = solve_both(mN), solve_both(mG)
        recsc, gsc, gov = {}, {}, {}
        for k, v in exp.items():
            n = oN[k]
            if v == 0 or (isinstance(v, Mag) and v.sq == 0):
                gov[k] = 'zero'
            elif n == v:
                gov[k] = 'net'
            elif n == 0 or (isinstance(n, Mag) and n.sq == 0):
                gov[k] = 'other loads only'
            else:
                gov[k] = 'mixed'
            an = absd(n)
            recsc[k] = an if an > 0 else scales[k]
            ag = absd(oG[k])
            gsc[k] = ag if ag > 0 else scales[k]
            assert recsc[k] <= scales[k], (k, recsc[k], scales[k], value_str(v), value_str(n))
        rec['expected_columns'] = ['key', 'expected', 'class', 'recommended_scale (binding, net-governed)', 'gross_scale', 'governed_by']
        rec['expected'] = [[k, value_str(v), class_of(k), g25(recsc[k]), g25(gsc[k]), gov[k]] for k, v in exp.items()]
        rec['cancellation'] = dict(cancel)
        rec['cancellation']['recommended_scale'] = ('binding (as ROOT ruled for RF-CANCEL): the magnitude of the value\'s response to the net contribution alone, '
                                                   'or the class scale where the net does not affect the value or the value is zero')
        rec['cancellation']['governed_by_counts'] = {g: sum(1 for x in gov.values() if x == g) for g in ('net', 'mixed', 'other loads only', 'zero')}
        cmp_scales = recsc
    else:
        rec['expected_columns'] = ['key', 'expected', 'class']
        rec['expected'] = [[k, value_str(v), class_of(k)] for k, v in exp.items()]
        cmp_scales = scales
    rec['classes'] = {c: {'scale': g25(sc[c]), 'derivation': how[c]} for c in sorted(sc)}
    rec['zero_valued'] = [[k, class_of(k), g25(scales[k])] for k, v in exp.items() if absd(v) == 0]
    rec['nonzero_below_class_scale'] = sum(1 for k, v in exp.items() if 0 < absd(v) < scales[k])
    rec['finite_input'] = {'max_normalized_difference': g6(fin), 'worst_key': wk,
                           'rule': 'max |rep - int| / max(|int|, scale); above 1e-9 the represented-input solution is the basis (R1 RF-FINITE)'}
    if basis == 'represented' or cs['generated']:
        rec['expected_represented'] = [[k, value_str(v)] for k, v in expR.items()]
        rec['represented_vs_intended_per_quantity'] = [
            [k, g6(abs(D60(expR[k]) - D60(expI[k])) / absd(expI[k])) if absd(expI[k]) > 0 else None, g6(diffs[k])]
            for k in expI] if cs['generated'] else None
    if cs['generated']:
        mF = cs['build'](Inp(False), PI_Q, ('bin64',)) if not cancel or True else None
        rec['generated_intensities'] = gen_json(mI, mR, mF)
    # negative controls
    ncres = []
    for ncd in cs['ncs'] + comb_ncs(cs, results):
        inpN = Inp(basis == 'represented')
        if ncd['kind'] == 'defect':
            obs = solve(apply_defects(mB, ncd['arg']))
        elif ncd['kind'] == 'flags':
            obs = solve(cs['build'](inpN, PI_Q, tuple(ncd['arg'])))
        elif ncd['kind'] == 'custom':
            obs = solve(ncd['arg'](mB))
        else:
            obs = ncd['arg'](exp)
        r = {'id': ncd['id'], 'defect': ncd['text']}
        r.update(compare(exp, obs, cmp_scales))
        if cancel:
            r['discriminates_under_class_scale'] = compare(exp, obs, scales)['discriminates']
            r['discriminates_under_gross_scale'] = compare(exp, obs, gsc)['discriminates']
        ncres.append(r)
    rec['negative_controls'] = ncres
    rec['notes'] = cs['notes']
    rec['_exp'] = expI
    rec['_seconds'] = round(time.time() - t0, 2)
    return rec


def comb_ncs(cs, results):
    comb = cs.get('combination')
    if not comb:
        return []
    A, B, sgn = results[comb[0]]['_exp'], results[comb[1]]['_exp'], comb[2]

    def magsum(exp):
        o = dict(exp)
        for k in exp:
            if isinstance(exp[k], Mag):
                a, b = to_dec(A[k], 70), to_dec(B[k], 70)
                v = a + b if sgn > 0 else abs(a - b)
                o[k] = Mag(Fr(v * v))
        return o

    def reversed_(exp):
        o = {}
        for k in exp:
            o[k] = exp[k] if isinstance(exp[k], Mag) else (B[k] - A[k] if sgn < 0 else A[k] - B[k])
        return o

    def aonly(exp):
        return dict(A)
    txt = ('NC-MAG-SUM: combined bending magnitudes taken as %s of the component magnitudes; other values correct.'
           % ('the sum' if sgn > 0 else 'the absolute difference'))
    return [{'id': 'NC-MAG-SUM', 'text': txt, 'kind': 'out', 'arg': magsum},
            {'id': 'NC-WRONG-DIFFERENCE', 'text': 'NC-WRONG-DIFFERENCE: components combined as %s (bending magnitudes left correct).' % ('B - A' if sgn < 0 else 'A - B'),
             'kind': 'out', 'arg': reversed_},
            {'id': 'NC-B-DROPPED', 'text': 'NC-B-DROPPED: the nodal-load case omitted from the combination.', 'kind': 'out', 'arg': aonly}]


import time


def main(argv):
    only = None
    if len(argv) > 1 and argv[1] == '--case':
        only = set(argv[2:])
    results = {}
    t0 = time.time()
    for cs in CASES:
        if only and cs['id'] not in only and not any(cs['id'] == c.get('combination', ('',))[0] or cs['id'] == c.get('combination', ('', ''))[1] for c in CASES if c['id'] in only):
            continue
        rec = run_case(cs, results)
        results[cs['id']] = rec
        nd = sum(1 for r in rec['negative_controls'] if r['discriminates'])
        print('%-40s basis=%-11s values=%4d zeros=%3d finite=%-9s NC %d/%d' % (
            cs['id'], rec['basis'], len(rec['expected']), len(rec['zero_valued']),
            rec['finite_input']['max_normalized_difference'], nd, len(rec['negative_controls'])))
        for r in rec['negative_controls']:
            print('    %-34s %-5s worst %s at %s' % (r['id'], 'FAIL' if r['discriminates'] else 'pass', r['worst_ratio_to_criterion'], r['worst_key']))
    return results, time.time() - t0


def write_json(results, path):
    cases = []
    for cs in CASES:
        r = dict(results[cs['id']])
        r.pop('_exp')
        r.pop('_seconds')
        cases.append(r)
    nvals = sum(len(c['expected']) for c in cases)
    nrep = sum(len(c.get('expected_represented') or []) for c in cases)
    ncs = sum(len(c['negative_controls']) for c in cases)
    ndisc = sum(1 for c in cases for n in c['negative_controls'] if n['discriminates'])
    doc = {
        'package': 'T3 RF-ELOAD: independent references for element, eigen, support-effort, prescribed and generated loads (R1 addendum R1_ADDENDUM_ELOAD.md)',
        'status': 'candidate; frozen only when ROOT selects it after a V2-style refutation',
        'criterion': '|observed - expected| <= 1e-9 * max(|expected|, scale); scale = the class scale of the value, except in RF-ELOAD-CANCEL and RF-ELOAD-CE-CANCEL cases, where the per-value recommended (net-governed) scale is binding, as ROOT ruled for RF-CANCEL. No tolerance is proposed.',
        'theory': ('Small-displacement, linear-elastic Euler-Bernoulli space frame, circular annular sections (Iy = Iz = I, J = 2I), rigid and prescribed '
                   'restraints on global DOFs, grounded linear springs on global DOFs. Uniform and partial-span uniform element loads (per unit member length, '
                   'global or local axes) as consistent equivalent nodal loads with fixed-end moments, member actions with the fixed-end correction; this is the exact '
                   'Euler-Bernoulli solution at nodes and stations. Eigen axial strain eps* (legacy alpha*DeltaT; 0.4.0 lambda_fit*lambda_thermal - 1) with '
                   'N = EA(ext/L - eps*). Pressure thrust: cap pair {-F_p e at i, +F_p e at j}, F_p = p*pi/4*ID^2, effective N = EA*ext/L - F_p. '
                   'Constant effort: a positive nodal force along the + axis of the declared translational DOF, no stiffness. Seismic w = g_factor*g*m\' per global '
                   'axis, wind w = p*Cs*(OD + 2 t_ins) along a global axis on marked extents, both computed exactly from the inputs (D-14). Curved bends excluded.'),
        'published_quantities': {
            'u.<node>.UX|UY|UZ, th.<node>.RX|RY|RZ': 'global nodal translations (m) and rotations (rad)',
            'R.<node>.<DOF>': 'rigid or prescribed restraint action on the structure, R = K d - f (N, N*m)',
            'S.<node>.F*|M*': 'spring action on the structure, -k u',
            'N.<m>.i|mid|j': 'axial force at authored end i, midpoint, end j, tension positive, fixed-end corrected (N)',
            'T.<m>.i|mid|j': 'torque, sign of (theta_j - theta_i).e (N*m)',
            'Mb.<m>.i|q1|mid|q3|j': 'bending-moment magnitude hypot(My, Mz) at fractions 0, 1/4, 1/2, 3/4, 1 from authored end i (N*m)',
            'tw.<m>, ext.<m>': '(theta_j - theta_i).e (rad) and (u_j - u_i).e (m)'},
        'pi': {'PI_Q': 'rational of the 190-digit Machin value', 'accuracy': '|PI_Q - pi| < 1e-189'},
        'units': 'm, N, Pa, N*m, rad, kg/m^3, m/s^2, 1/K, degC (differences only)',
        'summary': {'cases': len(cases), 'expected_values': nvals, 'represented_values': nrep,
                    'negative_controls': ncs, 'negative_controls_discriminating': ndisc,
                    'represented_basis_cases': [c['id'] for c in cases if c['basis'] == 'represented']},
        'cases': cases}
    text = json.dumps(doc, indent=1, sort_keys=False) + '\n'
    with open(path, 'w') as f:
        f.write(text)
    return doc['summary']


import json
import os


def verify_pi():
    """Cross-check PI_Q (Machin) against Gauss-Legendre at 230 digits."""
    with localcontext() as c:
        c.prec = 230
        a, b, t, p = Decimal(1), 1 / Decimal(2).sqrt(), Decimal(1) / 4, Decimal(1)
        for _ in range(12):
            an = (a + b) / 2
            b = (a * b).sqrt()
            t -= p * (a - an) ** 2
            a = an
            p *= 2
        gl = (a + b) ** 2 / (4 * t)
        err = abs(Decimal(PI_Q.numerator) / Decimal(PI_Q.denominator) - gl)
        assert err < Decimal('1e-189')
        c.prec = 3
        return str(+err)


def entry(argv):
    print('PI_Q check: |PI_Q - pi(Gauss-Legendre, 230 digits)| = %s < 1e-189' % verify_pi())
    results, secs = main(argv)
    if len(argv) > 1:
        print('partial run; JSON not written')
        return
    here = os.path.dirname(os.path.abspath(__file__))
    summ = write_json(results, os.path.join(here, 'references_eload.json'))
    print('summary: %d cases, %d expected values, %d represented values, %d negative controls (%d discriminate)' % (
        summ['cases'], summ['expected_values'], summ['represented_values'], summ['negative_controls'], summ['negative_controls_discriminating']))
    print('represented basis: %s' % ', '.join(summ['represented_basis_cases']))
    print('checks: route A == route B exactly and equilibrium residual zero for every intended, represented, net-state and gross-state model; closed forms, rotations and combination linearity asserted')


if __name__ == "__main__":
    entry(sys.argv)
