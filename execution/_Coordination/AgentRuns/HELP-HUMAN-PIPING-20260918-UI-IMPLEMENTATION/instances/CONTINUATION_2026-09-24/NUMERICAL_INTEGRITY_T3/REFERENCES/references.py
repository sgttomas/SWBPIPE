"""T3 independent references (R1): VP-ORACLES and VP-ROBUST candidate references.

Theory: small-displacement, linear-elastic Euler-Bernoulli space frame (no shear deformation),
circular annular sections, rigid restraints on global DOFs, grounded linear springs (global or
stated direction), nodal forces and moments.  Everything is original synthetic input; no material,
component, catalogue or code-rule data.

Independence: standard library only.  Nothing here imports, calls or reads product code, product
tests or product fixtures.  Expected values come from closed forms of the stated theory:
  * tree integration: statics of each cut plus Euler-Bernoulli compatibility integrals along each
    member (determinate structures, root with rigid restraints and springs);
  * the force (flexibility) method on top of tree integration (indeterminate structures);
  * pure-torsion and single-member pinned closed forms (derived in README.md);
  * the three-moment (Clapeyron) equation for continuous beams.
No direct-stiffness assembly produces any expected value.  (The author's separate self-check,
_run_records/selfcheck_stiffness.py, re-solves the small cases by exact direct stiffness.)

Arithmetic: exact rationals (fractions.Fraction).  pi is replaced by PI_Q, the exact rational value
of a 190-digit Machin evaluation (|PI_Q - pi| < 1e-189).  Published decimals are rounded from the
exact rationals to 40 significant digits.  One case family (represented inputs with irrational
member lengths) uses decimal arithmetic at 110 digits, stated per case.

Run:   python3 references.py            -> writes references.json, prints the run summary
       python3 references.py --full ID  -> prints every node/member value of one case (JSON lines)
The published JSON, not this script, is the candidate frozen expectation; the script is the
reproducible derivation.  Comparisons use the unchanged form
       |observed - expected| <= 1e-9 * max(|expected|, scale)
with the per-class scales published with each case.  No tolerance is proposed here.
"""
from __future__ import annotations

import json
import sys
from decimal import Decimal as D, localcontext
from fractions import Fraction as Fr
from math import isqrt
from pathlib import Path

SIG_EXPECTED = 40
SIG_AUX = 25
CRITERION = Fr(1, 10 ** 9)

# =====================================================================================
# arithmetic


def machin_pi(digits):
    """pi by Machin's identity 16 atan(1/5) - 4 atan(1/239); omitted terms < 10**-(digits+10)."""
    with localcontext() as c:
        c.prec = digits + 15
        eps = D(10) ** (-(digits + 10))

        def atan_inv(n):
            x = D(1) / n
            term = x
            total = x
            i = 1
            while abs(term) > eps:
                term *= -x * x
                total += term / (2 * i + 1)
                i += 1
            return total
        return +(16 * atan_inv(5) - 4 * atan_inv(239))


PI_DIGITS = 190
PI_DEC = machin_pi(PI_DIGITS)
PI_Q = Fr(PI_DEC)


def qsqrt(q):
    n, d = q.numerator, q.denominator
    rn, rd = isqrt(n), isqrt(d)
    if rn * rn != n or rd * rd != d:
        raise ValueError('irrational length in exact mode')
    return Fr(rn, rd)


class Exact:
    name = 'exact'

    def __init__(self, pi=PI_Q):
        self.pi = pi

    def num(self, x):
        return Fr(x)

    def sqrt(self, x):
        return qsqrt(Fr(x))


class Dec:
    name = 'decimal'

    def __init__(self, prec=110):
        self.prec = prec
        with localcontext() as c:
            c.prec = prec
            self.pi = +machin_pi(prec + 10)

    def num(self, x):
        x = Fr(x)
        with localcontext() as c:
            c.prec = self.prec
            return D(x.numerator) / D(x.denominator)

    def sqrt(self, x):
        with localcontext() as c:
            c.prec = self.prec
            return x.sqrt()


def rep(x):
    """binary64 value of an intended input: correctly rounded, then decoded exactly (Decimal.from_float)."""
    return Fr(D.from_float(float(Fr(x))))


def to_dec(v, prec=60):
    with localcontext() as c:
        c.prec = prec
        if isinstance(v, tuple):  # ('sqrt', q)
            return to_dec(v[1], prec + 10).sqrt()
        if isinstance(v, Fr):
            return D(v.numerator) / D(v.denominator)
        if isinstance(v, D):
            return +v
        if isinstance(v, int):
            return D(v)
        raise TypeError('unexpected value type %r' % type(v))


def fmt(v, sig=SIG_EXPECTED):
    if isinstance(v, tuple):
        if v[1] == 0:
            return '0'
    elif v == 0:
        return '0'
    d = to_dec(v, sig + 20)
    return format(d, '.%de' % (sig - 1))


def _fmt_input_rev1(q):
    """Revision-1 formatter, kept only so that every string it printed exactly stays byte-identical.
    (Its decimal branch normalized in the default 28-digit context and could round; fmt_input checks.)"""
    q = Fr(q)
    if q == 0:
        return '0'
    num, den = q.numerator, q.denominator
    d2 = den
    for p in (2, 5):
        while d2 % p == 0:
            d2 //= p
    if d2 == 1:
        with localcontext() as c:
            c.prec = 60
            s = D(num) / D(den)
        t = format(s.normalize(), 'f')
        if len(t.replace('-', '').replace('.', '').strip('0')) <= 40 and len(t) <= 60:
            return t
    k = 0
    while num % 2 == 0:
        num //= 2
        k += 1
    while den % 2 == 0:
        den //= 2
        k -= 1
    r = Fr(num, den)
    if k != 0 and r != q:
        rs = _fmt_input_rev1(r)
        if '/' not in rs:
            return '%s*2^%d' % (rs, k)
    return '%d/%d' % (q.numerator, q.denominator)


def exact_decimal(q):
    """(sign, digit string, exponent) with q = sign * int(digits) * 10**exponent exactly, or None when the
    rational has no finite decimal expansion."""
    q = Fr(q)
    num, den = abs(q.numerator), q.denominator
    a = b = 0
    d = den
    while d % 2 == 0:
        d //= 2
        a += 1
    while d % 5 == 0:
        d //= 5
        b += 1
    if d != 1:
        return None
    m = max(a, b)
    n = num * 10 ** m // den
    e = -m
    while n and n % 10 == 0:
        n //= 10
        e += 1
    return (-1 if q < 0 else 1), str(n), e


def _dec_string(sign, digits, e):
    plain_len = len(digits) + max(e, 0) + (max(-e - len(digits), 0) + 2 if e < 0 else 0)
    if plain_len <= 60:
        if e >= 0:
            t = digits + '0' * e
        elif -e < len(digits):
            t = digits[:e] + '.' + digits[e:]
        else:
            t = '0.' + '0' * (-e - len(digits)) + digits
    else:
        exp10 = e + len(digits) - 1
        t = digits[0] + ('.' + digits[1:] if len(digits) > 1 else '') + 'e%+d' % exp10
    return ('-' if sign < 0 else '') + t


def _fmt_input_exact(q):
    """Exact spelling: a short decimal (<= 40 significant digits), else r*2^k with r a short decimal, else the
    full decimal, else p/q.  Every form parses back to exactly q (parse_input)."""
    q = Fr(q)
    if q == 0:
        return '0'
    ed = exact_decimal(q)
    if ed and len(ed[1]) <= 40:
        return _dec_string(*ed)
    num, den, k = q.numerator, q.denominator, 0
    while num % 2 == 0:
        num //= 2
        k += 1
    while den % 2 == 0:
        den //= 2
        k -= 1
    r = Fr(num, den)
    er = exact_decimal(r)
    if k != 0 and er and len(er[1]) <= 40:
        return '%s*2^%d' % (_dec_string(*er), k)
    if ed:
        return _dec_string(*ed)
    return '%d/%d' % (q.numerator, q.denominator)


def parse_input(t):
    """Exact rational of a printed model input (decimal or scientific, p/q, or d*2^k)."""
    if '*2^' in t:
        a, k = t.split('*2^')
        return parse_input(a) * Fr(2) ** int(k)
    if '/' in t:
        p, q = t.split('/')
        return Fr(int(p), int(q))
    return Fr(D(t))


def fmt_input(q):
    """Exact printed form of an input.  Revision 2 (V2 finding F1): the revision-1 spelling is kept wherever it
    was already exact, and replaced by an exact spelling wherever it had been rounded."""
    q = Fr(q)
    t = _fmt_input_rev1(q)
    if parse_input(t) != q:
        t = _fmt_input_exact(q)
    if parse_input(t) != q:
        raise AssertionError('inexact input spelling for %r' % q)
    return t


# =====================================================================================
# vectors

def vadd(*vs_):
    return tuple(sum(c) for c in zip(*vs_))


def vsub(a, b):
    return (a[0] - b[0], a[1] - b[1], a[2] - b[2])


def vs(s, a):
    return (s * a[0], s * a[1], s * a[2])


def dot(a, b):
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]


def cross(a, b):
    return (a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])


def mv(Q, v):
    return tuple(sum(Q[r][c] * v[c] for c in range(3)) for r in range(3))


Z3 = (0, 0, 0)
AXES = {'X': (1, 0, 0), 'Y': (0, 1, 0), 'Z': (0, 0, 1)}
DOFS = ('UX', 'UY', 'UZ', 'RX', 'RY', 'RZ')


def dof_axis(dof):
    return AXES[dof[1]]


# =====================================================================================
# model realization

def section_props(ar, E, G, OD, ID):
    A = ar.pi * (OD * OD - ID * ID) / 4
    I = ar.pi * (OD ** 4 - ID ** 4) / 64
    return {'EA': E * A, 'EI': E * I, 'GJ': G * 2 * I}


class Model:
    """Numeric realization of a case definition in one arithmetic.

    decode maps each intended input (rational) to the value actually used: identity for the
    intended basis, rep() for the binary64-represented basis.  Spring directions are exact
    integer (or rational) vectors and are not decoded; a product normalizes them itself.
    """

    def __init__(self, defn, ar, decode=None):
        dec = decode or (lambda x: Fr(x))

        def n(x):
            return ar.num(dec(x))
        self.ar = ar
        self.nodes = {k: tuple(n(c) for c in v) for k, v in defn['nodes'].items()}
        self.sections = {}
        for sid, s in defn['sections'].items():
            E, OD, ID = n(s['E']), n(s['OD']), n(s['ID'])
            G = n(s['G']) if 'G' in s else E / (2 * (1 + n(s['nu'])))
            self.sections[sid] = section_props(ar, E, G, OD, ID)
        self.members = [dict(id=m[0], i=m[1], j=m[2], sec=m[3]) for m in defn['members']]
        self.supports = {}
        for node, s in defn.get('supports', {}).items():
            springs = [(kind, tuple(ar.num(Fr(c)) for c in d), n(k)) for (kind, d, k) in s.get('springs', [])]
            self.supports[node] = {'rigid': list(s.get('rigid', [])), 'springs': springs}
        self.loads = {}
        for node, ld in defn.get('loads', {}).items():
            F = tuple(n(c) for c in ld.get('F', (0, 0, 0)))
            M = tuple(n(c) for c in ld.get('M', (0, 0, 0)))
            self.loads[node] = (F, M)
        # authored load contributions (node, DOF, value), each decoded separately, summed exactly
        for (node, dof, val) in defn.get('load_contributions', []):
            F, M = self.loads.get(node, ((0, 0, 0), (0, 0, 0)))
            F, M = list(F), list(M)
            (F if dof[0] == 'U' else M)[DOF_INDEX_[dof]] += n(val)
            self.loads[node] = (tuple(F), tuple(M))
        self.member_loads = {mid: tuple(n(c) for c in q) for mid, q in defn.get('member_loads', {}).items()}


DOF_INDEX_ = {'UX': 0, 'UY': 1, 'UZ': 2, 'RX': 0, 'RY': 1, 'RZ': 2}


def spring_dirs(sup, kind):
    out = [(dof_axis(d), None) for d in sup['rigid'] if (d[0] == 'U') == (kind == 't')]
    out += [(d, k) for (kk, d, k) in sup['springs'] if kk == kind]
    return out


def check_root(sup):
    for kind in 'tr':
        dirs = [d for d, _ in spring_dirs(sup, kind)]
        if len(dirs) != 3:
            raise ValueError('a tree root must be supported in exactly three %s directions' % kind)
        for a in range(3):
            for b in range(a + 1, 3):
                if dot(dirs[a], dirs[b]) != 0:
                    raise ValueError('tree-root support directions must be mutually orthogonal')


# =====================================================================================
# tree integration (determinate root) and the force method

def build_tree(model, root):
    adj = {k: [] for k in model.nodes}
    for m in model.members:
        adj[m['i']].append((m['j'], m))
        adj[m['j']].append((m['i'], m))
    parent = {root: None}
    order = [root]
    k = 0
    while k < len(order):
        n = order[k]
        k += 1
        for (c, m) in adj[n]:
            if c not in parent:
                parent[c] = (n, m)
                order.append(c)
    if len(order) != len(model.nodes) or len(model.members) != len(model.nodes) - 1:
        raise ValueError('structure is not a connected tree from %s' % root)
    children = {k: [] for k in model.nodes}
    for c in order[1:]:
        children[parent[c][0]].append(c)
    return order, parent, children


def tree_solve(model, root, tree=None, extra=None, with_loads=True, cframe=None):
    """Statics plus Euler-Bernoulli compatibility along a tree.

    For member p->c (e = unit p->c, length L) carrying the subtree resultant F and moment M_c about c,
    the section moment at distance x from p is M(x) = M_c + (L-x) e x F, curvature C M(x) with
    C = e e^T/GJ + (I - e e^T)/EI, so
        dtheta = C (L M_c + L^2/2 e x F)
        du     = L^2/2 (C M_c) x e + L^3/3 (C (e x F)) x e + L (e.F)/EA e
        theta_c = theta_p + dtheta,  u_c = u_p + theta_p x (x_c - x_p) + du.
    cframe (negative controls only) replaces e inside C for named members.
    """
    ar = model.ar
    order, parent, children = tree or build_tree(model, root)
    X = model.nodes
    Fs, Ms = {}, {}
    for n in reversed(order):
        F, M = model.loads.get(n, (Z3, Z3)) if with_loads else (Z3, Z3)
        if extra and n in extra:
            F = vadd(F, extra[n][0])
            M = vadd(M, extra[n][1])
        for c in children[n]:
            F = vadd(F, Fs[c])
            M = vadd(M, Ms[c], cross(vsub(X[c], X[n]), Fs[c]))
        Fs[n], Ms[n] = F, M
    RF = vs(-1, Fs[root])
    RM = vs(-1, Ms[root])
    sup = model.supports[root]
    check_root(sup)
    u0, t0 = Z3, Z3
    for (d, k) in spring_dirs(sup, 't'):
        if k is not None:
            u0 = vadd(u0, vs(-dot(RF, d) / (k * dot(d, d)), d))
    for (d, k) in spring_dirs(sup, 'r'):
        if k is not None:
            t0 = vadd(t0, vs(-dot(RM, d) / (k * dot(d, d)), d))
    u, th = {root: u0}, {root: t0}
    mem = {}
    for n in order[1:]:
        p, m = parent[n]
        s = model.sections[m['sec']]
        EA, EI, GJ = s['EA'], s['EI'], s['GJ']
        dx = vsub(X[n], X[p])
        L2 = dot(dx, dx)
        L = ar.sqrt(L2)
        e = vs(1 / L, dx)
        ec = e
        if cframe and m['id'] in cframe:
            ec = cframe[m['id']] if m['i'] == p else vs(-1, cframe[m['id']])
        Fc, Mc = Fs[n], Ms[n]

        def C(v, ec=ec, GJ=GJ, EI=EI):
            ev = dot(ec, v)
            return vadd(vs(ev / GJ, ec), vs(1 / EI, vsub(v, vs(ev, ec))))
        eF = cross(e, Fc)
        dth = C(vadd(vs(L, Mc), vs(L2 / 2, eF)))
        du = vadd(vs(L2 / 2, cross(C(Mc), e)), vs(L * L2 / 3, cross(C(eF), e)), vs(L * dot(ec, Fc) / EA, ec))
        th[n] = vadd(th[p], dth)
        u[n] = vadd(u[p], cross(th[p], dx), du)

        def b2(Mx, dx=dx, L2=L2):
            dm = dot(dx, Mx)
            return dot(Mx, Mx) - dm * dm / L2
        M_par = vadd(Mc, vs(L, eF))
        M_mid = vadd(Mc, vs(L / 2, eF))
        rec = {'N': dot(dx, Fc) / L, 'T': dot(dx, Mc) / L, 'tw': dot(dx, Mc) / GJ, 'ext': dot(dx, Fc) / EA,
               'b2_mid': b2(M_mid)}
        if cframe and m['id'] in cframe:  # defective constitutive frame: publish what that frame implies
            rec['tw'] = dot(dth, e)
            rec['ext'] = dot(du, e)
        if m['i'] == p:
            rec['b2_i'], rec['b2_j'] = b2(M_par), b2(Mc)
        else:
            rec['b2_i'], rec['b2_j'] = b2(Mc), b2(M_par)
        mem[m['id']] = rec
    return {'u': u, 'th': th, 'RF': RF, 'RM': RM, 'mem': mem}


def split_root_reaction(model, root, RF, RM):
    sup = model.supports[root]
    rig = {}
    for d in sup['rigid']:
        rig[d] = dot(RF if d[0] == 'U' else RM, dof_axis(d))
    spr = []
    for (kind, d, k) in sup['springs']:
        R = RF if kind == 't' else RM
        spr.append(vs(dot(R, d) / dot(d, d), d))
    return rig, spr


def lin_solve(A, b, exact=True):
    n = len(b)
    A = [row[:] + [b[i]] for i, row in enumerate(A)]
    for c in range(n):
        if exact:
            piv = next(r for r in range(c, n) if A[r][c] != 0)
        else:
            piv = max(range(c, n), key=lambda r: abs(A[r][c]))
        A[c], A[piv] = A[piv], A[c]
        pv = A[c][c]
        for r in range(c + 1, n):
            f = A[r][c] / pv
            if f != 0:
                for k in range(c, n + 1):
                    A[r][k] -= f * A[c][k]
    x = [None] * n
    for r in range(n - 1, -1, -1):
        acc = A[r][n]
        for k in range(r + 1, n):
            acc -= A[r][k] * x[k]
        x[r] = acc / A[r][r]
    return x


def solve_force(model, root, cframe=None):
    """Force method: tree from a determinate root; every other rigid restraint or spring is a redundant.

    For redundant i (direction d_i, action X_i d_i on the structure):
        d_i.u_i(loads) + sum_j X_j d_i.u_i(unit j) + X_i |d_i|^2 / k_i = 0   (k_i = inf for rigid).
    """
    tree = build_tree(model, root)
    reds = []
    for node, sup in model.supports.items():
        if node == root:
            continue
        for d in sup['rigid']:
            reds.append((node, 't' if d[0] == 'U' else 'r', dof_axis(d), None, ('rigid', d)))
        for idx, (kind, d, k) in enumerate(sup['springs']):
            reds.append((node, kind, d, k, ('spring', idx)))
    base = tree_solve(model, root, tree, cframe=cframe)
    X = []
    sol = base
    if reds:
        def comp(s, r):
            return dot(r[2], s['u' if r[1] == 't' else 'th'][r[0]])
        units = []
        for (node, kind, d, k, _) in reds:
            ex = {node: (d, Z3) if kind == 't' else (Z3, d)}
            units.append(tree_solve(model, root, tree, ex, with_loads=False, cframe=cframe))
        A, b = [], []
        for i, r in enumerate(reds):
            row = [comp(units[j], r) for j in range(len(reds))]
            if r[3] is not None:
                row[i] += dot(r[2], r[2]) / r[3]
            A.append(row)
            b.append(-comp(base, r))
        X = lin_solve(A, b, exact=(model.ar.name == 'exact'))
        ex = {}
        for x, (node, kind, d, k, _) in zip(X, reds):
            F, M = ex.get(node, (Z3, Z3))
            if kind == 't':
                F = vadd(F, vs(x, d))
            else:
                M = vadd(M, vs(x, d))
            ex[node] = (F, M)
        sol = tree_solve(model, root, tree, ex, cframe=cframe)
    rig, spr = split_root_reaction(model, root, sol['RF'], sol['RM'])
    reac = {root: rig}
    springs = {root: spr}
    for x, (node, kind, d, k, tag) in zip(X, reds):
        if tag[0] == 'rigid':
            reac.setdefault(node, {})[tag[1]] = x
        else:
            springs.setdefault(node, [None] * len(model.supports[node]['springs']))[tag[1]] = vs(x, d)
    return {'u': sol['u'], 'th': sol['th'], 'reac': reac, 'springs': springs, 'mem': sol['mem']}


# =====================================================================================
# closed forms for translation-pinned members

def solve_pinned_torsion(model, root, tip):
    """Collinear members; every node translation-pinned; one rotational spring along the line at the
    root; a torque along the line at the tip.  Pure torsion: every member carries the tip torque T,
    theta_n = (T/k + sum_{members root..n} T L/GJ) e, no translation, force or bending anywhere.
    (Nodal moment equilibrium holds at every node; pinned translations and zero curvature are
    compatible; the structure is stable, so this field is the solution.)"""
    X = model.nodes
    order, parent, children = build_tree(model, root)
    Tvec = model.loads[tip][1]
    line = vsub(X[tip], X[root])
    if any(c != 0 for c in model.loads[tip][0]) or len(model.loads) != 1:
        raise ValueError('closed form needs a single tip torque')
    if cross(line, Tvec) != (0, 0, 0):
        raise ValueError('torque must lie along the line')
    for n in X:
        if cross(vsub(X[n], X[root]), line) != (0, 0, 0):
            raise ValueError('nodes must be collinear')
        sup = model.supports[n]
        if sorted(sup['rigid']) != ['UX', 'UY', 'UZ']:
            raise ValueError('every node must be translation-pinned only')
        if n != root and sup['springs']:
            raise ValueError('only the root carries a spring')
    rs = model.supports[root]['springs']
    if len(rs) != 1 or rs[0][0] != 'r' or cross(rs[0][1], line) != (0, 0, 0):
        raise ValueError('root needs one rotational spring along the line')
    d, k = rs[0][1], rs[0][2]
    th = {root: vs(dot(Tvec, d) / (k * dot(d, d)), d)}
    u = {n: Z3 for n in X}
    mem = {}
    zero = 0 * k
    for n in order[1:]:
        p, m = parent[n]
        s = model.sections[m['sec']]
        seg = vsub(X[n], X[p])
        L = model.ar.sqrt(dot(seg, seg))
        th[n] = vadd(th[p], vs(dot(seg, Tvec) / (s['GJ'] * L), seg))
        mem[m['id']] = {'N': zero, 'T': dot(seg, Tvec) / L, 'tw': dot(seg, Tvec) / s['GJ'], 'ext': zero,
                        'b2_i': zero, 'b2_j': zero, 'b2_mid': zero}
    reac = {n: {'UX': zero, 'UY': zero, 'UZ': zero} for n in X}
    springs = {n: [] for n in X}
    springs[root] = [vs(-dot(Tvec, d) / dot(d, d), d)]
    return {'u': u, 'th': th, 'reac': reac, 'springs': springs, 'mem': mem}


def solve_pin_off(model, root, tip):
    """One member, both ends translation-pinned; at the root one rotational spring k (or one rigid global
    rotation restraint) with unit axis d at cosine c = d.e to the member (c != 0); a tip moment
    M1 = M1e e + M1p (M1p perpendicular to e).  With w = d - c e and s = d.theta_root:
        s = M1e/(c k)                      (s = 0 for a rigid restraint)
        theta_root_perp = -L/(3 EI) (M1p/2 + (M1e/c) w)
        theta_root.e    = (s - w.theta_root_perp)/c
        theta_tip_perp  = (M1p L/EI - 2 theta_root_perp)/4 ;   theta_tip.e = theta_root.e + M1e L/GJ
        restraint moment on the structure -(M1e/c) d
        translation reactions R_tip = e x (M_0 + M_1)/L = -R_root, with member end moments
        M_0 = -(M1e) e + EI/L (4 theta_root_perp + 2 theta_tip_perp) and M_1 = M1.
    For M1p = 0 (tip torque T e): theta_root = [T/(c^2 k) + (1-c^2) T L/(3 c^2 EI)] e - T L/(3 c EI) w,
    theta_tip = [theta_root.e + T L/GJ] e + T L/(6 c EI) w, R_tip = T (d x e)/(c L), root bending |T/c| sqrt(1-c^2).
    """
    X = model.nodes
    m = model.members[0]
    s_ = model.sections[m['sec']]
    EI, GJ = s_['EI'], s_['GJ']
    dx = vsub(X[tip], X[root])
    L2 = dot(dx, dx)
    L = model.ar.sqrt(L2)
    e = vs(1 / L, dx)
    if any(c != 0 for c in model.loads[tip][0]) or len(model.loads) != 1:
        raise ValueError('closed form needs a single tip moment')
    M1 = model.loads[tip][1]
    M1e = dot(M1, e)
    M1p = vsub(M1, vs(M1e, e))
    sup = model.supports[root]
    rot_rigid = [r for r in sup['rigid'] if r[0] == 'R']
    if sup['springs']:
        (kind, d, k), = sup['springs']
        if rot_rigid or kind != 'r':
            raise ValueError('one rotational spring or one rigid rotation restraint at the root')
        dh = d  # unnormalized direction: every formula below is homogeneous in d except s
    else:
        (r,) = rot_rigid
        k = None
        dh = dof_axis(r)
    c = dot(dh, e)
    w = vsub(dh, vs(c, e))
    s = M1e * dot(dh, dh) / (c * k) if k is not None else 0 * L
    t0p = vs(-L / (3 * EI), vadd(vs(Fr(1, 2) if model.ar.name == 'exact' else model.ar.num(Fr(1, 2)), M1p), vs(M1e / c, w)))
    t0e = (s - dot(w, t0p)) / c
    t1p = vs(Fr(1, 4) if model.ar.name == 'exact' else model.ar.num(Fr(1, 4)), vsub(vs(L / EI, M1p), vs(2, t0p)))
    t1e = t0e + M1e * L / GJ
    th_root = vadd(vs(t0e, e), t0p)
    th_tip = vadd(vs(t1e, e), t1p)
    M0 = vadd(vs(-M1e, e), vs(EI / L, vadd(vs(4, t0p), vs(2, t1p))))
    Rt = vs(1 / L, cross(e, vadd(M0, M1)))
    Mres = vs(-M1e / c, dh)
    reac = {root: {'UX': -Rt[0], 'UY': -Rt[1], 'UZ': -Rt[2]}, tip: {'UX': Rt[0], 'UY': Rt[1], 'UZ': Rt[2]}}
    springs = {root: [], tip: []}
    if k is None:
        reac[root][rot_rigid[0]] = dot(Mres, dh)
    else:
        springs[root] = [Mres]

    def b2(Mx):
        dm = dot(dx, Mx)
        return dot(Mx, Mx) - dm * dm / L2
    zero = 0 * L
    rec = {'N': zero, 'T': M1e, 'tw': M1e * L / GJ, 'ext': zero, 'b2_mid': b2(vs(Fr(1, 2) if model.ar.name == 'exact' else model.ar.num(Fr(1, 2)), vsub(M1, M0)))}
    if m['i'] == root:
        rec['b2_i'], rec['b2_j'] = b2(M0), b2(M1)
    else:
        rec['b2_i'], rec['b2_j'] = b2(M1), b2(M0)
    return {'u': {root: (zero, zero, zero), tip: (zero, zero, zero)}, 'th': {root: th_root, tip: th_tip},
            'reac': reac, 'springs': springs, 'mem': {m['id']: rec}}


# =====================================================================================
# continuous beam: three-moment equation

def three_moment_plane(p, ell):
    """Support moments m_0..m_s (m = EI v'', m'' = q) for a beam fixed at support 0 and pinned at 1..s,
    with point loads p_j (+ local transverse) at the midspans of equal spans ell:
        2 m_0 + m_1 = (3 ell/8) p_1 ;  m_{i-1} + 4 m_i + m_{i+1} = (3 ell/8)(p_i + p_{i+1}) ;  m_s = 0."""
    s = len(p) - 1
    one = ell / ell
    if s == 1:
        return [3 * ell * p[1] / 16, 0 * ell]
    diag = [2 * one] + [4 * one] * (s - 1)
    r = [3 * ell * p[1] / 8] + [3 * ell * (p[i] + p[i + 1]) / 8 for i in range(1, s)]
    cp = [None] * s
    dp = [None] * s
    cp[0] = one / diag[0]
    dp[0] = r[0] / diag[0]
    for i in range(1, s):
        den = diag[i] - cp[i - 1]
        cp[i] = one / den
        dp[i] = (r[i] - dp[i - 1]) / den
    m = [None] * (s + 1)
    m[s - 1] = dp[s - 1]
    for i in range(s - 2, -1, -1):
        m[i] = dp[i] - cp[i] * m[i + 1]
    m[s] = 0 * ell
    return m


def three_moment_response(p, ell, EI, m=None):
    """Span-wise integration of EI v'' = m with v = 0 at the supports (see README.md, RF-LARGE)."""
    if m is None:
        m = three_moment_plane(p, ell)
    s = len(p) - 1
    vmid = [None] + [-(ell * ell) * (m[j - 1] + m[j]) / (16 * EI) + p[j] * ell ** 3 / (48 * EI) for j in range(1, s + 1)]
    slope_sup = []
    for i in range(s + 1):
        if i < s:
            slope_sup.append(-(ell / EI) * (m[i] / 3 + m[i + 1] / 6) + p[i + 1] * ell * ell / (16 * EI))
        else:
            slope_sup.append((ell / EI) * (m[s - 1] / 6 + m[s] / 3) - p[s] * ell * ell / (16 * EI))
    slope_mid = [None] + [(ell / (24 * EI)) * (m[j - 1] - m[j]) for j in range(1, s + 1)]
    R = []
    for i in range(s + 1):
        left = (m[i] - m[i - 1]) / ell + p[i] / 2 if i >= 1 else 0 * ell
        right = (m[i + 1] - m[i]) / ell - p[i + 1] / 2 if i < s else 0 * ell
        R.append(right - left)
    mmid = [None] + [(m[j - 1] + m[j]) / 2 - p[j] * ell / 4 for j in range(1, s + 1)]
    return {'m': m, 'vmid': vmid, 'slope_sup': slope_sup, 'slope_mid': slope_mid, 'R': R, 'mmid': mmid}


def solve_continuous(model, spec, simple_spans=False):
    """spec: spans, ell, Q (global = Q local), origin.  Loads read back from the model's midspan nodes."""
    s, Q = spec['spans'], spec['Q']
    EI = model.sections[model.members[0]['sec']]['EI']
    ar = model.ar
    ell = ar.sqrt(dot(vsub(model.nodes['S1'], model.nodes['S0']), vsub(model.nodes['S1'], model.nodes['S0'])))

    def to_local(v):
        return tuple(sum(Q[r][c] * v[r] for r in range(3)) for c in range(3))

    def to_global(v):
        return tuple(sum(Q[r][c] * v[c] for c in range(3)) for r in range(3))
    Z = 0 * ell
    if ar.name == 'exact':
        for i in range(s + 1):
            if vsub(model.nodes['S%d' % i], model.nodes['S0']) != vs(i, vsub(model.nodes['S1'], model.nodes['S0'])):
                raise ValueError('supports must be equally spaced on a line')
        for j in range(1, s + 1):
            if vs(2, model.nodes['C%d' % j]) != vadd(model.nodes['S%d' % (j - 1)], model.nodes['S%d' % j]):
                raise ValueError('midspan nodes must bisect their spans')
    if any(n[0] != 'C' for n in model.loads):
        raise ValueError('loads only at midspan nodes')
    py = [Z] * (s + 1)
    pz = [Z] * (s + 1)
    for j in range(1, s + 1):
        F = model.loads.get('C%d' % j, ((Z, Z, Z), (Z, Z, Z)))[0]
        lf = to_local(F)
        if lf[0] != 0 or any(c != 0 for c in model.loads.get('C%d' % j, ((Z, Z, Z), (Z, Z, Z)))[1]):
            raise ValueError('closed form admits transverse midspan forces only')
        py[j], pz[j] = lf[1], lf[2]
    zeros = [Z] * (s + 1)
    rv = three_moment_response(py, ell, EI, zeros if simple_spans else None)
    rw = three_moment_response(pz, ell, EI, zeros if simple_spans else None)
    u, th, reac, springs, mem = {}, {}, {}, {}, {}
    for i in range(s + 1):
        u['S%d' % i] = (Z, Z, Z)
        th['S%d' % i] = to_global((Z, -rw['slope_sup'][i], rv['slope_sup'][i]))
        springs['S%d' % i] = []
        reac['S%d' % i] = dict(zip(('UX', 'UY', 'UZ'), to_global((Z, rv['R'][i], rw['R'][i]))))
    for j in range(1, s + 1):
        u['C%d' % j] = to_global((Z, rv['vmid'][j], rw['vmid'][j]))
        th['C%d' % j] = to_global((Z, -rw['slope_mid'][j], rv['slope_mid'][j]))
    X = model.nodes
    Mtot = (Z, Z, Z)
    for node, (F, Mo) in model.loads.items():
        Mtot = vadd(Mtot, Mo, cross(vsub(X[node], X['S0']), F))
    for i in range(1, s + 1):
        Rg = to_global((Z, rv['R'][i], rw['R'][i]))
        Mtot = vadd(Mtot, cross(vsub(X['S%d' % i], X['S0']), Rg))
    reac['S0'].update(dict(zip(('RX', 'RY', 'RZ'), vs(-1, Mtot))))
    for j in range(1, s + 1):
        A = (rv['m'][j - 1], rw['m'][j - 1])
        Cc = (rv['mmid'][j], rw['mmid'][j])
        B = (rv['m'][j], rw['m'][j])
        for mid, P1, P2 in (('A%d' % j, A, Cc), ('B%d' % j, Cc, B)):
            mem[mid] = {'N': Z, 'T': Z, 'tw': Z, 'ext': Z,
                        'b2_i': P1[0] ** 2 + P1[1] ** 2, 'b2_j': P2[0] ** 2 + P2[1] ** 2,
                        'b2_mid': ((P1[0] + P2[0]) / 2) ** 2 + ((P1[1] + P2[1]) / 2) ** 2}
    return {'u': u, 'th': th, 'reac': reac, 'springs': springs, 'mem': mem}


def solve_udl2(model):
    """Nodes S0 (x = 0), S1 (x = L), S2 (x = 2L) on the global x axis; S0 and S2 fixed, S1 translation-pinned;
    span A (S0-S1) and span B (S1-S2) carry uniform transverse loads w_A, w_B (N/m, global +y); S1 carries a
    nodal moment Mz.  With m = EI v'' (internal moment about z, m'' = w), each span is the fixed-ended solution
    plus the effect of the shared-node rotation theta:
        theta = (Mz + (w_B - w_A) L^2/12) / (8 EI/L)
        m_A(x) = w_A (x^2/2 - L x/2 + L^2/12) + EI theta (-2/L + 6x/L^2)
        m_B(x) = w_B (x^2/2 - L x/2 + L^2/12) + EI theta (-4/L + 6x/L^2)
    Reactions (support on structure): S0: FY = -w_A L/2 + 6 EI theta/L^2, MZ = -m_A(0);
    S1: FY = -(w_A + w_B) L/2;  S2: FY = -w_B L/2 - 6 EI theta/L^2, MZ = m_B(L).
    The fixed-end moments of the two spans enter the shared node with opposite signs, so equal loads cancel there."""
    X = model.nodes
    L = X['S1'][0] - X['S0'][0]
    zero = 0 * L
    for n, x in (('S0', 0), ('S1', 1), ('S2', 2)):
        if X[n] != (x * L, zero, zero) and X[n] != (x * L, 0, 0):
            raise ValueError('udl2 closed form: nodes on the x axis at 0, L, 2L')
    wA = model.member_loads.get('A', (zero, zero, zero))
    wB = model.member_loads.get('B', (zero, zero, zero))
    if wA[0] != 0 or wA[2] != 0 or wB[0] != 0 or wB[2] != 0:
        raise ValueError('udl2 closed form: transverse y loads only')
    F1, M1 = model.loads.get('S1', ((zero, zero, zero), (zero, zero, zero)))
    if any(c != 0 for c in F1) or M1[0] != 0 or M1[1] != 0 or any(n != 'S1' for n in model.loads):
        raise ValueError('udl2 closed form: a single nodal moment about z at S1')
    wA, wB, Mz = wA[1], wB[1], M1[2]
    EI = model.sections[model.members[0]['sec']]['EI']
    th = (Mz + (wB - wA) * L * L / 12) / (8 * EI / L)

    def mA(x):
        return wA * (x * x / 2 - L * x / 2 + L * L / 12) + EI * th * (-2 / L + 6 * x / (L * L))

    def mB(x):
        return wB * (x * x / 2 - L * x / 2 + L * L / 12) + EI * th * (-4 / L + 6 * x / (L * L))
    Z = (zero, zero, zero)
    u = {'S0': Z, 'S1': Z, 'S2': Z}
    thv = {'S0': Z, 'S1': (zero, zero, th), 'S2': Z}
    reac = {'S0': {'UX': zero, 'UY': -wA * L / 2 + 6 * EI * th / (L * L), 'UZ': zero, 'RX': zero, 'RY': zero, 'RZ': -mA(zero)},
            'S1': {'UX': zero, 'UY': -(wA + wB) * L / 2, 'UZ': zero},
            'S2': {'UX': zero, 'UY': -wB * L / 2 - 6 * EI * th / (L * L), 'UZ': zero, 'RX': zero, 'RY': zero, 'RZ': mB(L)}}
    mem = {}
    for mid, f in (('A', mA), ('B', mB)):
        m = next(x for x in model.members if x['id'] == mid)
        ends = (f(zero), f(L / 2), f(L))
        rec = {'N': zero, 'T': zero, 'tw': zero, 'ext': zero, 'b2_mid': ends[1] ** 2}
        if (m['i'], m['j']) in (('S0', 'S1'), ('S1', 'S2')):
            rec['b2_i'], rec['b2_j'] = ends[0] ** 2, ends[2] ** 2
        else:
            rec['b2_i'], rec['b2_j'] = ends[2] ** 2, ends[0] ** 2
        mem[mid] = rec
    return {'u': u, 'th': thv, 'reac': reac, 'springs': {'S0': [], 'S1': [], 'S2': []}, 'mem': mem}


# =====================================================================================
# solve dispatch

def solve(defn, method, ar=None, decode=None, **kw):
    ar = ar or Exact()
    model = Model(defn, ar, decode)
    kind = method[0]
    if kind in ('tree', 'force'):
        return solve_force(model, method[1], **kw)
    if kind == 'pintor':
        return solve_pinned_torsion(model, method[1], method[2])
    if kind == 'pinoff':
        return solve_pin_off(model, method[1], method[2])
    if kind == 'cont':
        return solve_continuous(model, method[1], **kw)
    if kind == 'udl2':
        return solve_udl2(model)
    raise ValueError(kind)


METHOD_TEXT = {
    'tree': 'tree integration (cut statics + Euler-Bernoulli compatibility integrals) from the determinate root %s',
    'force': 'force method on tree integration from root %s; other supports are redundant reactions',
    'pintor': 'pure-torsion closed form (translation-pinned collinear nodes, root spring %s, tip %s)',
    'pinoff': 'single-member pinned closed form (root %s, tip %s)',
    'cont': 'three-moment equation (fixed support S0, pinned supports S1..Ss, midspan point loads)',
    'udl2': 'two fixed-ended spans with uniform transverse loads meeting at a translation-pinned node: fixed-end moments plus the rotation of the shared node',
}


def method_text(method):
    t = METHOD_TEXT[method[0]]
    if method[0] in ('tree', 'force'):
        return t % method[1]
    if method[0] in ('pintor', 'pinoff'):
        return t % (method[1], method[2])
    return t


# =====================================================================================
# published quantities, classes and scales
#
# Keys:  u.<node>.UX|UY|UZ (m)            th.<node>.RX|RY|RZ (rad)
#        R.<node>.<DOF>  rigid-restraint action on the structure (N or N*m)
#        S.<node>.<i>.FX|FY|FZ or MX|MY|MZ  global components of spring i's action on the structure
#        N.<member> axial force, tension positive (N)      T.<member> torque = GJ * twist / L (N*m)
#        Mb.<member>.i|mid|j  bending-moment magnitude hypot(My, Mz) at authored end i, midpoint, end j
#        tw.<member> = (theta_j - theta_i).e_ij (rad)      ext.<member> = (u_j - u_i).e_ij (m)
# N, T, tw and ext do not depend on which end is called i.

CLASSES = ('translation', 'rotation', 'force', 'moment', 'twist', 'extension')


def region_maps(defn, regions):
    rn, rm = {}, {}
    for name, r in (regions or {}).items():
        for n in r.get('nodes', []):
            rn[n] = name
        for m in r.get('members', []):
            rm[m] = name
    return rn, rm


def cls_name(base, region):
    return base if region in (None, 'main') else '%s@%s' % (base, region)


def flatten(sol, defn, nodes=None, members=None, regions=None):
    rn, rm = region_maps(defn, regions)
    out = []
    nl = [n for n in defn['nodes'] if nodes is None or n in nodes]
    for n in nl:
        r = rn.get(n)
        for ax, c in enumerate('XYZ'):
            out.append(('u.%s.U%s' % (n, c), sol['u'][n][ax], cls_name('translation', r)))
        for ax, c in enumerate('XYZ'):
            out.append(('th.%s.R%s' % (n, c), sol['th'][n][ax], cls_name('rotation', r)))
    sups = defn.get('supports', {})
    for n in nl:
        if n not in sups:
            continue
        r = rn.get(n)
        for d in sups[n].get('rigid', []):
            out.append(('R.%s.%s' % (n, d), sol['reac'][n][d], cls_name('force' if d[0] == 'U' else 'moment', r)))
        for i, (kind, dvec, k) in enumerate(sups[n].get('springs', [])):
            vec = sol['springs'][n][i]
            for ax, c in enumerate('XYZ'):
                out.append(('S.%s.%d.%s%s' % (n, i, 'F' if kind == 't' else 'M', c), vec[ax],
                            cls_name('force' if kind == 't' else 'moment', r)))
    for m in defn['members']:
        mid = m[0]
        if members is not None and mid not in members:
            continue
        r = rm.get(mid)
        rec = sol['mem'][mid]
        out.append(('N.%s' % mid, rec['N'], cls_name('force', r)))
        out.append(('T.%s' % mid, rec['T'], cls_name('moment', r)))
        for st in ('i', 'mid', 'j'):
            out.append(('Mb.%s.%s' % (mid, st), ('sqrt', rec['b2_' + st]), cls_name('moment', r)))
        out.append(('tw.%s' % mid, rec['tw'], cls_name('twist', r)))
        out.append(('ext.%s' % mid, rec['ext'], cls_name('extension', r)))
    return out


def absval(v):
    if isinstance(v, tuple):
        return to_dec(v)
    return abs(to_dec(v))


def compute_scales(sol, defn, model, regions=None):
    """Per-class scale for every published value.  A non-empty class uses the largest magnitude of that
    class in the case (vector norms for nodal translations, rotations and support actions, so the
    scale is invariant under rotation).  An all-zero class uses the load and geometry fallbacks:
        force  <- moment/L_c, or max(F_ref, M_ref/L_c) if both classes vanish
        moment <- force*L_c,  or max(M_ref, F_ref*L_c)
        translation <- rotation*L_c ;  rotation <- translation/L_c
        twist <- moment * max(L/GJ) ;  extension <- force * max(L/EA)
    L_c: longest member of the region; F_ref, M_ref: largest applied nodal force and moment norms."""
    rn, rm = region_maps(defn, regions)
    names = sorted(set(['main'] + list((regions or {}).keys())))
    acc = {r: {c: 0 for c in CLASSES} for r in names}   # squared maxima (exact or decimal)

    def put(r, c, v2):
        if v2 > acc[r][c]:
            acc[r][c] = v2
    for n in defn['nodes']:
        r = rn.get(n, 'main')
        put(r, 'translation', dot(sol['u'][n], sol['u'][n]))
        put(r, 'rotation', dot(sol['th'][n], sol['th'][n]))
    for n, sup in defn.get('supports', {}).items():
        r = rn.get(n, 'main')
        F, M = Z3, Z3
        for d in sup.get('rigid', []):
            if d[0] == 'U':
                F = vadd(F, vs(sol['reac'][n][d], dof_axis(d)))
            else:
                M = vadd(M, vs(sol['reac'][n][d], dof_axis(d)))
        for i, (kind, dvec, k) in enumerate(sup.get('springs', [])):
            if kind == 't':
                F = vadd(F, sol['springs'][n][i])
            else:
                M = vadd(M, sol['springs'][n][i])
        put(r, 'force', dot(F, F))
        put(r, 'moment', dot(M, M))
    for m in defn['members']:
        r = rm.get(m[0], 'main')
        rec = sol['mem'][m[0]]
        put(r, 'force', rec['N'] ** 2)
        put(r, 'moment', rec['T'] ** 2)
        for st in ('i', 'mid', 'j'):
            put(r, 'moment', rec['b2_' + st])
        put(r, 'twist', rec['tw'] ** 2)
        put(r, 'extension', rec['ext'] ** 2)
    scales = {}
    for r in names:
        mem_r = [m for m in defn['members'] if rm.get(m[0], 'main') == r] or list(defn['members'])
        lens, lgj, lea = [], [], []
        for m in mem_r:
            dx = vsub(model.nodes[m[2]], model.nodes[m[1]])
            L = to_dec(dot(dx, dx)).sqrt()
            s = model.sections[m[3]]
            lens.append(L)
            lgj.append(L / to_dec(s['GJ']))
            lea.append(L / to_dec(s['EA']))
        Lc, mLGJ, mLEA = max(lens), max(lgj), max(lea)
        nodes_r = [n for n in defn['nodes'] if rn.get(n, 'main') == r]
        Fref, Mref = D(0), D(0)
        for n, ld in defn.get('loads', {}).items():
            if n in nodes_r or not any(x in defn.get('loads', {}) for x in nodes_r):
                F = [to_dec(Fr(c)) for c in ld.get('F', (0, 0, 0))]
                M = [to_dec(Fr(c)) for c in ld.get('M', (0, 0, 0))]
                Fref = max(Fref, sum(c * c for c in F).sqrt())
                Mref = max(Mref, sum(c * c for c in M).sqrt())
        v = {c: to_dec(acc[r][c]).sqrt() for c in CLASSES}
        how = {c: 'largest %s magnitude in %s' % (c, 'the case' if r == 'main' else 'region ' + r) for c in CLASSES}
        if v['force'] == 0 and v['moment'] == 0:
            v['force'] = max(Fref, Mref / Lc)
            v['moment'] = max(Mref, Fref * Lc)
            how['force'] = 'max(F_ref, M_ref/L_c) (no force or moment in class)'
            how['moment'] = 'max(M_ref, F_ref*L_c) (no force or moment in class)'
        elif v['force'] == 0:
            v['force'] = v['moment'] / Lc
            how['force'] = 'moment scale / L_c, L_c = %s m (class all zero)' % fmt(Lc, 12)
        elif v['moment'] == 0:
            v['moment'] = v['force'] * Lc
            how['moment'] = 'force scale * L_c, L_c = %s m (class all zero)' % fmt(Lc, 12)
        if not nodes_r:
            how['translation'] = how['rotation'] = 'region has no nodes (class unused)'
        elif v['translation'] == 0 and v['rotation'] == 0:
            raise ValueError('no motion in region %s' % r)
        elif v['translation'] == 0:
            v['translation'] = v['rotation'] * Lc
            how['translation'] = 'rotation scale * L_c, L_c = %s m (class all zero)' % fmt(Lc, 12)
        elif v['rotation'] == 0:
            v['rotation'] = v['translation'] / Lc
            how['rotation'] = 'translation scale / L_c, L_c = %s m (class all zero)' % fmt(Lc, 12)
        if v['twist'] == 0:
            v['twist'] = v['moment'] * mLGJ
            how['twist'] = 'moment scale * max(L/GJ) (class all zero)'
        if v['extension'] == 0:
            v['extension'] = v['force'] * mLEA
            how['extension'] = 'force scale * max(L/EA) (class all zero)'
        for c in CLASSES:
            scales[cls_name(c, r)] = (v[c], how[c])
    return scales


def normalized_diff(a, e, scale):
    ea = absval(e)
    den = CRIT_D * max(ea, scale)
    da = to_dec(a) if not isinstance(a, tuple) else to_dec(a)
    de = to_dec(e) if not isinstance(e, tuple) else to_dec(e)
    return abs(da - de) / den


CRIT_D = D(1) / D(10 ** 9)


def compare(values, exp_map, scales, top=24):
    """values: {key: value} from a defective answer.  Returns the violation summary under the criterion."""
    viol = []
    with localcontext() as c:
        c.prec = 60
        for k, v in values.items():
            e, cl = exp_map[k]
            r = normalized_diff(v, e, scales[cl][0])
            if r > 1:
                viol.append((r, k, v))
    viol.sort(key=lambda t: (-t[0], t[1]))
    return {'discriminates': bool(viol), 'violating_values': len(viol),
            'max_violation_ratio': fmt(viol[0][0], 6) if viol else '0',
            'at': viol[0][1] if viol else None,
            'values': {k: fmt(v, SIG_AUX) for (_, k, v) in viol[:top]}}


# =====================================================================================
# negative controls (the specific wrong answers of named defects)

class Ctx:
    def __init__(self, case, sol, flat, exp_map, scales, model):
        self.case, self.sol, self.flat, self.exp_map, self.scales, self.model = case, sol, flat, exp_map, scales, model
        self.defn, self.method = case['defn'], case['method']

    def resolve(self, defn2, method=None, **kw):
        s = solve(defn2, method or self.method, **kw)
        f = flatten(s, defn2, *self.case.get('sample', (None, None)), regions=self.case.get('regions'))
        return {k: v for (k, v, c) in f if k in self.exp_map}


def nc_result(nid, description, ctx, values=None, outcome=None, extra=None):
    out = {'id': nid, 'description': description}
    if outcome:
        out['defective_outcome'] = outcome
    if values is not None:
        out.update(compare(values, ctx.exp_map, ctx.scales))
    else:
        out['discriminates'] = True
    if extra:
        out.update(extra)
    return out


def _deep_copy(x):
    if isinstance(x, dict):
        return {k: _deep_copy(v) for k, v in x.items()}
    if isinstance(x, list):
        return [_deep_copy(v) for v in x]
    if isinstance(x, tuple):
        return tuple(_deep_copy(v) for v in x)
    return x


def f64_sum_increment(K, k):
    """Effective value of k after one rounded binary64 addition into a diagonal holding K."""
    Kf, kf = float(Fr(K)), float(Fr(k))
    return Fr(Kf + kf) - Fr(Kf)


def nc_lost_soft(node, idx, K_dd, where):
    def f(ctx):
        k = Fr(ctx.defn['supports'][node]['springs'][idx][2])
        keff = f64_sum_increment(K_dd(ctx.model), k)
        desc = ('lost soft contribution: spring %d at %s after one rounded binary64 addition into the %s '
                'diagonal (K_dd = %s); effective k = %s instead of %s' %
                (idx, node, where, fmt(to_dec(K_dd(ctx.model)), 17), fmt(keff, 17), fmt(k, 17)))
        if keff == 0:
            return nc_result('NC-LOST-SOFT', desc, ctx, outcome='the soft spring vanishes entirely: the represented '
                             'assembly is singular and a defective solver reports a mechanism instead of solving')
        d2 = _deep_copy(ctx.defn)
        sp = list(d2['supports'][node]['springs'])
        kind, dv, _ = sp[idx]
        sp[idx] = (kind, dv, keff)
        d2['supports'][node]['springs'] = sp
        return nc_result('NC-LOST-SOFT', desc, ctx, ctx.resolve(d2),
                         extra={'relative_change_of_k': fmt((keff - k) / k, 6)})
    return f


def nc_scaled_section(sec, K_dd, k_of, where):
    """Soft coupling member: its whole section stiffness scaled by the one-addition retention factor."""
    def f(ctx):
        k = k_of(ctx.model)
        keff = f64_sum_increment(K_dd(ctx.model), k)
        fac = keff / k
        desc = ('lost soft coupling: coupling stiffness %s added once in binary64 to the %s diagonal %s; '
                'retention factor %s applied to the coupling section (E and G)' %
                (fmt(k, 12), where, fmt(K_dd(ctx.model), 12), fmt(fac, 12)))
        d2 = _deep_copy(ctx.defn)
        if fac == 0:
            members = [m for m in d2['members'] if m[3] != sec]
            d2['members'] = members
            return nc_result('NC-LOST-SOFT', desc, ctx, outcome='the coupling vanishes: identical to NC-DROPPED-COUPLING')
        s = dict(d2['sections'][sec])
        s['E'] = Fr(s['E']) * fac
        s['G'] = Fr(s['G']) * fac
        d2['sections'][sec] = s
        return nc_result('NC-LOST-SOFT', desc, ctx, ctx.resolve(d2), extra={'retention_factor': fmt(fac, 12)})
    return f


def nc_subtract_rounded():
    def f(ctx):
        vals = {}
        m = ctx.model
        for mem in ctx.defn['members']:
            mid, i, j, sec = mem
            if 'tw.%s' % mid not in ctx.exp_map:
                continue
            dx = vsub(m.nodes[j], m.nodes[i])
            L = m.ar.sqrt(dot(dx, dx))
            e = vs(1 / L, dx)
            s = m.sections[sec]
            dth = tuple(rep(a) - rep(b) for a, b in zip(ctx.sol['th'][j], ctx.sol['th'][i]))
            du = tuple(rep(a) - rep(b) for a, b in zip(ctx.sol['u'][j], ctx.sol['u'][i]))
            tw, ex = dot(dth, e), dot(du, e)
            vals['tw.%s' % mid] = tw
            vals['T.%s' % mid] = s['GJ'] * tw / L
            vals['ext.%s' % mid] = ex
            vals['N.%s' % mid] = s['EA'] * ex / L
        return nc_result('NC-SUBTRACT-ROUNDED', 'relative motion recovered by subtracting binary64-rounded global '
                         'rotations and translations (exact values rounded once), then T = GJ tw/L, N = EA ext/L', ctx, vals)
    return f


def nc_sign():
    def f(ctx):
        vals = {k: -v for (k, v, c) in ctx.flat if (k.startswith('R.') or k.startswith('S.')) and v != 0}
        return nc_result('NC-SIGN', 'support actions published with the opposite sign (structure-on-support '
                         'instead of support-on-structure)', ctx, vals)
    return f


def nc_wrong_transform(member_axes, form_note=None):
    def f(ctx):
        s = ctx.resolve(ctx.defn, cframe={k: v for k, v in member_axes.items()})
        desc = ('direction-cosine indexing error: the constitutive frame of %s uses the member axis with its first '
                'two global components swapped' % ', '.join(member_axes))
        extra = None
        if form_note:
            desc += form_note[0]
            extra = form_note[1]
        return nc_result('NC-WRONG-TRANSFORM', desc, ctx, s, extra=extra)
    return f


def nc_resolved(nid, description, modify, method=None):
    def f(ctx):
        d2 = modify(_deep_copy(ctx.defn))
        return nc_result(nid, description, ctx, ctx.resolve(d2, method))
    return f


def nc_values(nid, description, fn):
    def f(ctx):
        return nc_result(nid, description, ctx, fn(ctx))
    return f


def nc_round6():
    def f(ctx):
        vals = {}
        for (k, v, c) in ctx.flat:
            x = to_dec(v)
            q = x.quantize(D('1e-6'))
            vals[k] = Fr(q) if not isinstance(v, tuple) else ('sqrt', Fr(q) ** 2)
        return nc_result('NC-ROUND6', 'historical publication quantization: every value rounded to 1e-6 in its unit', ctx, vals)
    return f


# =====================================================================================
# finite-input basis (RF-FINITE applied to every case)

def finite_compare(case, sol_int, flat_int, exp_map, scales):
    sample = case.get('sample', (None, None))
    try:
        ar = Exact()
        sol_rep = solve(case['defn'], case['method'], ar, rep)
        mode = 'exact rational on represented inputs'
    except ValueError as err:
        if 'irrational' not in str(err):
            raise
        ar = Dec(110)
        sol_rep = solve(case['defn'], case['method'], ar, rep)
        mode = 'decimal 110 digits on represented inputs (irrational member lengths)'
    flat_rep = flatten(sol_rep, case['defn'], *sample, regions=case.get('regions'))
    worst = (D(0), None)
    identical = True
    per = []
    with localcontext() as c:
        c.prec = 60
        for (k, v, cl), (k2, v2, cl2) in zip(flat_int, flat_rep):
            assert k == k2
            if v2 != v:
                identical = False
            r = normalized_diff(v2, v, scales[cl][0])
            per.append((r * CRIT_D, k))
            if r > worst[0]:
                worst = (r, k)
    nonrep = count_nonrepresentable(case['defn'])
    out = {'nonrepresentable_inputs': nonrep, 'method': mode,
           'max_normalized_difference': fmt(worst[0] * CRIT_D, 6), 'at': worst[1],
           'exceeds_1e-9': worst[0] > 1}
    if identical:
        out['max_normalized_difference'] = '0'
        out['at'] = None
    elif case['family'] == 'RF-FINITE' or worst[0] * CRIT_D >= D('1e-12'):
        keep = [(d, k) for (d, k) in per if case['family'] == 'RF-FINITE' or d >= D('1e-12')]
        out['per_quantity_normalized_difference'] = {k: fmt(d, 6) for (d, k) in keep}
        out['per_quantity_note'] = ('|represented - intended| / max(|intended|, scale) for %s' %
                                    ('every published value' if case['family'] == 'RF-FINITE' else 'values at or above 1e-12'))
    return out, flat_rep


def count_nonrepresentable(defn):
    vals = []
    for v in defn['nodes'].values():
        vals += list(v)
    for s in defn['sections'].values():
        vals += [s[k] for k in s]
    for s in defn.get('supports', {}).values():
        vals += [k for (_, _, k) in s.get('springs', [])]
    for ld in defn.get('loads', {}).values():
        vals += list(ld.get('F', ())) + list(ld.get('M', ()))
    vals += [v for (_, _, v) in defn.get('load_contributions', [])]
    for q in defn.get('member_loads', {}).values():
        vals += list(q)
    return sum(1 for x in vals if rep(x) != Fr(x))


# =====================================================================================
# common inputs

SEC_N = {'E': Fr(200 * 10 ** 9), 'G': Fr(80 * 10 ** 9), 'OD': Fr('0.2'), 'ID': Fr('0.18')}
ALL6 = list(DOFS)
PIN3 = ['UX', 'UY', 'UZ']
ROT3 = ['RX', 'RY', 'RZ']
THETA0 = Fr(1, 10 ** 4)      # target soft-mode rotation (rad) / translation (m) of the soft cases

Q3 = [[Fr(1, 3), Fr(2, 3), Fr(2, 3)], [Fr(2, 3), Fr(1, 3), Fr(-2, 3)], [Fr(-2, 3), Fr(2, 3), Fr(-1, 3)]]
Q9 = [[Fr(1, 9), Fr(8, 9), Fr(4, 9)], [Fr(8, 9), Fr(1, 9), Fr(-4, 9)], [Fr(-4, 9), Fr(4, 9), Fr(-7, 9)]]
I3 = [[Fr(1), Fr(0), Fr(0)], [Fr(0), Fr(1), Fr(0)], [Fr(0), Fr(0), Fr(1)]]
ROTATIONS = {'Q3': (Q3, 3, 'quaternion (1,1,1,0)/sqrt(3)'), 'Q9': (Q9, 9, 'quaternion (1,2,2,0)/3')}


def check_rotation(Q):
    for r in range(3):
        for c in range(3):
            assert sum(Q[k][r] * Q[k][c] for k in range(3)) == (1 if r == c else 0)
    det = (Q[0][0] * (Q[1][1] * Q[2][2] - Q[1][2] * Q[2][1]) - Q[0][1] * (Q[1][0] * Q[2][2] - Q[1][2] * Q[2][0])
           + Q[0][2] * (Q[1][0] * Q[2][1] - Q[1][1] * Q[2][0]))
    assert det == 1


for _q in (Q3, Q9):
    check_rotation(_q)

CASES = []
FAMILY_TEXT = {}


def add_case(**kw):
    kw.setdefault('ncs', [])
    kw.setdefault('notes', [])
    kw.setdefault('flags', {})
    kw.setdefault('finite', True)
    CASES.append(kw)
    return kw


def needs_directional(defn):
    for s in defn.get('supports', {}).values():
        for (kind, d, k) in s.get('springs', []):
            if sum(1 for c in d if c != 0) != 1:
                return True
    return False


def K_rot_dir(member, node_dir):
    """Rotational stiffness of a member at its end in unit direction d: GJ/L c^2 + 4EI/L (1 - c^2)."""
    def f(model):
        m = next(x for x in model.members if x['id'] == member)
        s = model.sections[m['sec']]
        dx = vsub(model.nodes[m['j']], model.nodes[m['i']])
        L = model.ar.sqrt(dot(dx, dx))
        d = node_dir
        c2 = dot(dx, d) ** 2 / (dot(dx, dx) * dot(d, d))
        return s['GJ'] / L * c2 + 4 * s['EI'] / L * (1 - c2)
    return f


def K_trans_dir(member, node_dir):
    """Translational stiffness of a member at its end in unit direction d: EA/L c^2 + 12EI/L^3 (1 - c^2)."""
    def f(model):
        m = next(x for x in model.members if x['id'] == member)
        s = model.sections[m['sec']]
        dx = vsub(model.nodes[m['j']], model.nodes[m['i']])
        L = model.ar.sqrt(dot(dx, dx))
        d = node_dir
        c2 = dot(dx, d) ** 2 / (dot(dx, dx) * dot(d, d))
        return s['EA'] / L * c2 + 12 * s['EI'] / L ** 3 * (1 - c2)
    return f


def fr(s):
    return Fr(s)


# =====================================================================================
# RF-CHAIN

FAMILY_TEXT['RF-CHAIN'] = ('Series torsion and axial chains of 3, 5 and 10 members with a soft grounding spring at the root '
                           '(k/a from about 1e-4 to 1e-12, a = stiffness of the first member), generalizing N05.  Closed form '
                           'by series compliance: root motion = load/k, each member adds load*L/(GJ or EA).')
CHAIN_LENGTHS = ['2', '1', '3', '1.5', '2.5', '0.5', '4', '1.25', '3.5', '0.75']
CHAIN_RATIOS = [('1e-04', '216', '59700'), ('1e-06', '2.16', '597'), ('1e-08', '0.0216', '5.97'),
                ('1e-10', '0.000216', '0.0597'), ('1e-12', '0.00000216', '0.000597')]


def chain_defn(n, kind, k, load):
    nodes = {'N0': (Fr(0), Fr(0), Fr(0))}
    x = Fr(0)
    for i in range(n):
        x += Fr(CHAIN_LENGTHS[i])
        nodes['N%d' % (i + 1)] = (x, Fr(0), Fr(0))
    members = [('M%d' % (i + 1), 'N%d' % i, 'N%d' % (i + 1), 'N') for i in range(n)]
    if kind == 'T':
        sup = {'N0': {'rigid': ['UX', 'UY', 'UZ', 'RY', 'RZ'], 'springs': [('r', (1, 0, 0), Fr(k))]}}
        loads = {'N%d' % n: {'M': (Fr(load), Fr(0), Fr(0))}}
    else:
        sup = {'N0': {'rigid': ['UY', 'UZ', 'RX', 'RY', 'RZ'], 'springs': [('t', (1, 0, 0), Fr(k))]}}
        loads = {'N%d' % n: {'F': (Fr(load), Fr(0), Fr(0))}}
    return {'nodes': nodes, 'sections': {'N': SEC_N}, 'members': members, 'supports': sup, 'loads': loads}


def nc_stored_chain(kind):
    """Exact solution of the binary64-stored 1-D chain matrix (NP-A generalized): correctly rounded member
    coefficients a_i = GJ/L_i (EA/L_i), one rounded addition per assembled diagonal, rounded k and load."""
    def f(ctx):
        m = ctx.model
        mems = ctx.defn['members']
        n = len(mems)
        a = []
        for (mid, i, j, sec) in mems:
            dx = vsub(m.nodes[j], m.nodes[i])
            L = m.ar.sqrt(dot(dx, dx))
            a.append((m.sections[sec]['GJ'] if kind == 'T' else m.sections[sec]['EA']) / L)
        af = [float(x) for x in a]
        k = Fr(ctx.defn['supports']['N0']['springs'][0][2])
        kf = float(k)
        ld = ctx.defn['loads']['N%d' % n]
        load = Fr(ld['M'][0] if kind == 'T' else ld['F'][0])
        diag = [kf + af[0]] + [af[i - 1] + af[i] for i in range(1, n)] + [af[n - 1]]
        A = [[Fr(0)] * (n + 1) for _ in range(n + 1)]
        for i in range(n + 1):
            A[i][i] = Fr(diag[i])
        for i in range(n):
            A[i][i + 1] = A[i + 1][i] = -Fr(af[i])
        b = [Fr(0)] * n + [Fr(float(load))]
        desc = ('exact solution of the binary64-stored chain matrix: correctly rounded member coefficients '
                '%s/L_i, one rounded addition per assembled diagonal, rounded k and load (NP-A generalized)' %
                ('GJ' if kind == 'T' else 'EA'))
        if diag[0] == af[0]:
            return nc_result('NC-STORED-ASSEMBLY', desc, ctx, outcome='fl(a_1 + k) = a_1: the stored matrix has lost k '
                             'and is singular or indefinite; a defective solver refuses or returns an arbitrary state')
        x = lin_solve(A, b)
        vals = {}
        dof = 'th.N%d.RX' if kind == 'T' else 'u.N%d.UX'
        for i in range(n + 1):
            vals[dof % i] = x[i]
        for i in range(n):
            rel = x[i + 1] - x[i]
            vals[('tw.M%d' if kind == 'T' else 'ext.M%d') % (i + 1)] = rel
            vals[('T.M%d' if kind == 'T' else 'N.M%d') % (i + 1)] = Fr(af[i]) * rel
        vals['S.N0.0.MX' if kind == 'T' else 'S.N0.0.FX'] = -Fr(kf) * x[0]
        return nc_result('NC-STORED-ASSEMBLY', desc, ctx, vals)
    return f


def build_chain():
    for kind in 'TA':
        for n in (3, 5, 10):
            for (rlab, kt, ka) in CHAIN_RATIOS:
                k = Fr(kt if kind == 'T' else ka)
                load = k * THETA0
                defn = chain_defn(n, kind, k, load)
                cid = 'RF-CHAIN-%s-n%02d-r%s' % (kind, n, rlab)
                what = 'torsion' if kind == 'T' else 'axial'
                K_dd = (lambda mdl: mdl.sections['N']['GJ'] / Fr(CHAIN_LENGTHS[0])) if kind == 'T' else \
                    (lambda mdl: mdl.sections['N']['EA'] / Fr(CHAIN_LENGTHS[0]))
                add_case(id=cid, family='RF-CHAIN',
                         purpose='%s chain of %d members (lengths %s m) along x; root spring k = %s %s, k/a_1 ~ %s; '
                                 'tip %s %s so the root %s is 1e-4.' % (
                                     what, n, ', '.join(CHAIN_LENGTHS[:n]), fmt_input(k),
                                     'N*m/rad' if kind == 'T' else 'N/m', rlab,
                                     'torque' if kind == 'T' else 'force', fmt_input(load),
                                     'rotation (rad)' if kind == 'T' else 'translation (m)'),
                         defn=defn, method=('tree', 'N0'),
                         ncs=[nc_lost_soft('N0', 0, K_dd, 'root %s (member M1)' % ('RX' if kind == 'T' else 'UX')),
                              nc_stored_chain(kind), nc_subtract_rounded(), nc_sign()],
                         notes=['relative motions (%s) are published per member and carry their own class scale' %
                                ('element twists tw' if kind == 'T' else 'element extensions ext')])


# =====================================================================================
# RF-SKEW

FAMILY_TEXT['RF-SKEW'] = ('N05-class soft restraints on single members along (3,4,0)/5 (L = 5 m) and (1,2,2)/3 (L = 3 m), '
                          'integer node coordinates, with the soft spring along the member axis and off-axis (global X).  '
                          'Torsion (rotational spring) and axial (translational spring) variants, cantilevered '
                          '(determinate: tree integration) and translation-pinned at both ends (closed form).')
SKEW_DIRS = {
    '345': {'e': (3, 4, 0), 'L': 5, 'n': (-4, 3, 0), 'm': (0, 0, 1), 'wrong': (Fr(4, 5), Fr(3, 5), Fr(0)),
            'Pn': (-8, 6, 0), 'P': 10, 'rot': {'1e-04': '86.4', '1e-08': '0.00864', '1e-12': '0.000000864'},
            'tra': {'1e-04': '23900', '1e-08': '2.39', '1e-12': '0.000239'}},
    '122': {'e': (1, 2, 2), 'L': 3, 'n': (2, 1, -2), 'm': (-2, 2, -1), 'wrong': (Fr(2, 3), Fr(1, 3), Fr(2, 3)),
            'Pn': (6, 3, -6), 'P': 9, 'rot': {'1e-04': '144', '1e-08': '0.0144', '1e-12': '0.00000144'},
            'tra': {'1e-04': '39800', '1e-08': '3.98', '1e-12': '0.000398'}},
}
K_ORD_ROT = Fr(10 ** 6)
K_ORD_TRA = Fr(10 ** 8)


def nc_pin_projection():
    def f(ctx):
        m = ctx.model
        n0, n1 = 'N0', 'N1'
        dx = vsub(m.nodes[n1], m.nodes[n0])
        L = m.ar.sqrt(dot(dx, dx))
        e = vs(1 / L, dx)
        (kind, d, k), = m.supports[n0]['springs']
        dh = vs(1 / m.ar.sqrt(dot(d, d)), d)
        c = dot(dh, e)
        T = dot(m.loads[n1][1], e)
        s = m.sections['N']
        th0 = vs(T / (c * c * k), e)
        th1 = vs(T / (c * c * k) + T * L / s['GJ'], e)
        vals = {}
        for ax, cc in enumerate('XYZ'):
            vals['th.N0.R%s' % cc] = th0[ax]
            vals['th.N1.R%s' % cc] = th1[ax]
            vals['R.N0.U%s' % cc] = Fr(0)
            vals['R.N1.U%s' % cc] = Fr(0)
        for st in ('i', 'mid', 'j'):
            vals['Mb.M1.%s' % st] = ('sqrt', Fr(0))
        return nc_result('NC-DROPPED-COUPLING', 'off-axis spring treated as a torsional spring k c^2 about the member '
                         'axis, dropping the bending it induces (no perpendicular rotations, no translation reactions, '
                         'no bending moment)', ctx, vals)
    return f


def skew_defn(D, variant, k):
    e, L = D['e'], D['L']
    T = k * THETA0
    nodes = {'N0': (Fr(0), Fr(0), Fr(0)), 'N1': tuple(Fr(c) for c in e)}
    members = [('M1', 'N0', 'N1', 'N')]
    axis_load = tuple(T * c / L for c in e)
    if variant == 'T-PIN-AX':
        sup = {'N0': {'rigid': PIN3, 'springs': [('r', e, k)]}, 'N1': {'rigid': PIN3}}
        loads = {'N1': {'M': axis_load}}
    elif variant == 'T-PIN-OFF':
        sup = {'N0': {'rigid': PIN3, 'springs': [('r', (1, 0, 0), k)]}, 'N1': {'rigid': PIN3}}
        loads = {'N1': {'M': axis_load}}
    elif variant == 'T-CANT-AX':
        sup = {'N0': {'rigid': PIN3, 'springs': [('r', e, k), ('r', D['n'], K_ORD_ROT), ('r', D['m'], K_ORD_ROT)]}}
        loads = {'N1': {'M': axis_load, 'F': tuple(Fr(c) for c in D['Pn'])}}
    elif variant == 'T-CANT-OFF':
        sup = {'N0': {'rigid': PIN3, 'springs': [('r', (1, 0, 0), k), ('r', (0, 1, 0), K_ORD_ROT), ('r', (0, 0, 1), K_ORD_ROT)]}}
        loads = {'N1': {'M': axis_load}}
    elif variant == 'A-CANT-AX':
        sup = {'N0': {'rigid': ROT3, 'springs': [('t', e, k), ('t', D['n'], K_ORD_TRA), ('t', D['m'], K_ORD_TRA)]}}
        loads = {'N1': {'F': vadd(axis_load, tuple(Fr(c) for c in D['Pn']))}}
    elif variant == 'A-CANT-OFF':
        sup = {'N0': {'rigid': ROT3, 'springs': [('t', (1, 0, 0), k), ('t', (0, 1, 0), K_ORD_TRA), ('t', (0, 0, 1), K_ORD_TRA)]}}
        loads = {'N1': {'F': axis_load}}
    return {'nodes': nodes, 'sections': {'N': SEC_N}, 'members': members, 'supports': sup, 'loads': loads}


SKEW_TEXT = {
    'T-PIN-AX': 'both nodes translation-pinned; root rotational spring k along the member axis; tip torque T e',
    'T-PIN-OFF': 'both nodes translation-pinned; root rotational spring k along global X (off-axis); tip torque T e',
    'T-CANT-AX': 'root translations fixed; root rotational springs k along e and 1e6 N*m/rad along the two perpendicular '
                 'axes; tip torque T e plus a perpendicular tip force',
    'T-CANT-OFF': 'root translations fixed; root rotational springs k along global X and 1e6 N*m/rad along Y and Z; '
                  'tip torque T e',
    'A-CANT-AX': 'root rotations fixed; root translational springs k along e and 1e8 N/m along the two perpendicular '
                 'axes; tip axial force F e plus a perpendicular tip force',
    'A-CANT-OFF': 'root rotations fixed; root translational springs k along global X and 1e8 N/m along Y and Z; '
                  'tip axial force F e',
}


def build_skew():
    for variant in ('T-PIN-AX', 'T-PIN-OFF', 'T-CANT-AX', 'T-CANT-OFF', 'A-CANT-AX', 'A-CANT-OFF'):
        for dname, D in SKEW_DIRS.items():
            for rlab in ('1e-04', '1e-08', '1e-12'):
                k = Fr(D['rot' if variant[0] == 'T' else 'tra'][rlab])
                defn = skew_defn(D, variant, k)
                if variant in ('T-PIN-AX', 'T-PIN-OFF'):
                    method = ('pinoff', 'N0', 'N1')
                else:
                    method = ('tree', 'N0')
                spring_dir = D['e'] if variant.endswith('AX') else (1, 0, 0)
                if variant[0] == 'T':
                    Kdd = K_rot_dir('M1', spring_dir)
                    where = 'root rotational (member M1 in the spring direction)'
                else:
                    Kdd = K_trans_dir('M1', spring_dir)
                    where = 'root translational (member M1 in the spring direction)'
                ncs = [nc_lost_soft('N0', 0, Kdd, where), nc_subtract_rounded(), nc_sign()]
                if method[0] == 'tree':
                    ncs.append(nc_wrong_transform({'M1': D['wrong']}))
                if variant == 'T-PIN-OFF':
                    ncs.append(nc_pin_projection())
                add_case(id='RF-SKEW-%s-%s-r%s' % (variant, dname, rlab), family='RF-SKEW',
                         purpose='member (0,0,0)->%s, L = %d m: %s; k = %s (k/a ~ %s against %s/L), soft-mode motion 1e-4.' % (
                             D['e'], D['L'], SKEW_TEXT[variant], fmt_input(k), rlab,
                             'GJ' if variant[0] == 'T' else 'EA'),
                         defn=defn, method=method, ncs=ncs)


# =====================================================================================
# RF-WEAK

FAMILY_TEXT['RF-WEAK'] = ('Weak coupling.  W-AX: two stiff collinear subsystems joined by a soft member (E and G scaled by '
                          'rho), loaded on one side, both far ends fixed; the far response is proportional to rho.  W-L: '
                          'L-shaped two-member frame with a soft root torsional spring (bending-torsion coupling).  W-3D: '
                          'two fixed L-frames joined by a soft member normal to both planes, loaded on one side.  '
                          'Force method (6 redundants) or tree integration.')
WEAK_RHO = ['1e-04', '1e-08', '1e-12']
WL_TRANSFORM_NOTE = (
    ' (flexibility form: only the compliance of M2 in tree integration uses the swapped axis, so the rigid-body '
    'kinematics stay exact and the soft rigid mode is unchanged; the discriminates flag refers to this form only)',
    {'defect_form': 'flexibility (constitutive frame of M2 in tree integration); rigid-body kinematics exact',
     'other_forms': 'V2 (REFERENCE_CHECK/RETURN.md, finding F4) applied the same axis swap inside a direct-stiffness '
                    'element, where it breaks rigid-body invariance and stiffens the soft mode; that form fails by '
                    'about 1e9 at th.N2.RX.  Not computed by this script.'})


def soft_section(rho):
    return {'E': SEC_N['E'] * Fr(rho), 'G': SEC_N['G'] * Fr(rho), 'OD': SEC_N['OD'], 'ID': SEC_N['ID']}


def build_weak():
    for rho in WEAK_RHO:
        nodes = {'N%d' % i: (Fr(i), Fr(0), Fr(0)) for i in range(6)}
        members = [('A1', 'N0', 'N1', 'N'), ('A2', 'N1', 'N2', 'N'), ('C', 'N2', 'N3', 'C'), ('B1', 'N3', 'N4', 'N'),
                   ('B2', 'N4', 'N5', 'N')]
        defn = {'nodes': nodes, 'sections': {'N': SEC_N, 'C': soft_section(rho)}, 'members': members,
                'supports': {'N0': {'rigid': ALL6}, 'N5': {'rigid': ALL6}},
                'loads': {'N2': {'F': (Fr(1000), Fr(100), Fr(0)), 'M': (Fr(50), Fr(0), Fr(0))}}}
        regions = {'coupling': {'members': ['C']}, 'far': {'nodes': ['N3', 'N4', 'N5'], 'members': ['B1', 'B2']}}

        def drop(d):
            d['members'] = [m for m in d['members'] if m[0] != 'C']
            return d
        add_case(id='RF-WEAK-W-AX-rho%s' % rho, family='RF-WEAK',
                 purpose='nodes x = 0..5 m; A1, A2 stiff (N section), C soft (E, G x %s), B1, B2 stiff; N0 and N5 fixed; '
                         'load at N2: F = (1000, 100, 0) N, M = (50, 0, 0) N*m.  Far-side response is proportional to the '
                         'coupling.' % rho,
                 defn=defn, method=('force', 'N0'), regions=regions,
                 ncs=[nc_scaled_section('C', lambda mdl: mdl.sections['N']['EA'] / 1,
                                        lambda mdl: mdl.sections['C']['EA'] / 1, 'N2 UX (member A2)'),
                      nc_values('NC-DROPPED-COUPLING', 'coupling member omitted: far side unloaded, near side a cantilever',
                                lambda ctx: _dropped_values(ctx, drop, ('N3', 'N4', 'N5'), ('C', 'B1', 'B2'))),
                      nc_subtract_rounded(), nc_sign()],
                 notes=['regions: near (N0-N2, A1, A2), coupling (member C), far (N3-N5, B1, B2); each region has its own '
                        'class scales, so a lost coupling cannot hide under a near-side scale'])
    for rlab, k in (('1e-04', '216'), ('1e-08', '0.0216'), ('1e-12', '0.00000216')):
        k = Fr(k)
        Fz = k * THETA0 / 2
        defn = {'nodes': {'N0': (Fr(0), Fr(0), Fr(0)), 'N1': (Fr(2), Fr(0), Fr(0)), 'N2': (Fr(2), Fr(2), Fr(0))},
                'sections': {'N': SEC_N}, 'members': [('M1', 'N0', 'N1', 'N'), ('M2', 'N1', 'N2', 'N')],
                'supports': {'N0': {'rigid': ['UX', 'UY', 'UZ', 'RY', 'RZ'], 'springs': [('r', (1, 0, 0), k)]}},
                'loads': {'N2': {'F': (Fr(0), Fr(0), Fz)}}}
        add_case(id='RF-WEAK-W-L-r%s' % rlab, family='RF-WEAK',
                 purpose='L-frame N0 (0,0,0) -> N1 (2,0,0) -> N2 (2,2,0); root fixed except RX, which has spring k = %s '
                         'N*m/rad (k/(GJ/2) ~ %s); tip force Fz = %s N so the root rotation is 1e-4.  The bending of M2 '
                         'becomes torsion of M1, carried by the soft spring.' % (fmt_input(k), rlab, fmt_input(Fz)),
                 defn=defn, method=('tree', 'N0'),
                 ncs=[nc_lost_soft('N0', 0, lambda mdl: mdl.sections['N']['GJ'] / 2, 'root RX (member M1)'),
                      nc_subtract_rounded(), nc_sign(),
                      nc_wrong_transform({'M2': (Fr(1), Fr(0), Fr(0))}, WL_TRANSFORM_NOTE)])
    for rho in WEAK_RHO:
        nodes = {'A0': (0, 0, 0), 'A1': (2, 0, 0), 'A2': (2, 2, 0), 'B2': (2, 2, 2), 'B1': (4, 2, 2), 'B0': (4, 4, 2)}
        nodes = {k: tuple(Fr(c) for c in v) for k, v in nodes.items()}
        members = [('A01', 'A0', 'A1', 'N'), ('A12', 'A1', 'A2', 'N'), ('C', 'A2', 'B2', 'C'), ('B21', 'B2', 'B1', 'N'),
                   ('B10', 'B1', 'B0', 'N')]
        defn = {'nodes': nodes, 'sections': {'N': SEC_N, 'C': soft_section(rho)}, 'members': members,
                'supports': {'A0': {'rigid': ALL6}, 'B0': {'rigid': ALL6}},
                'loads': {'A2': {'F': (Fr(0), Fr(0), Fr(10)), 'M': (Fr(5), Fr(0), Fr(0))}}}
        regions = {'coupling': {'members': ['C']}, 'far': {'nodes': ['B2', 'B1', 'B0'], 'members': ['B21', 'B10']}}

        def drop3(d):
            d['members'] = [m for m in d['members'] if m[0] != 'C']
            return d
        add_case(id='RF-WEAK-W-3D-rho%s' % rho, family='RF-WEAK',
                 purpose='L-frame A (A0 fixed, x then y) and L-frame B (B0 fixed, -y then -x) joined by soft member C '
                         '(A2 -> B2 along z, E and G x %s); load at A2: F = (0,0,10) N, M = (5,0,0) N*m.  Coupling carries '
                         'axial force, torsion and bending into B.' % rho,
                 defn=defn, method=('force', 'A0'), regions=regions,
                 ncs=[nc_scaled_section('C', lambda mdl: 12 * mdl.sections['N']['EI'] / 8,
                                        lambda mdl: mdl.sections['C']['EA'] / 2, 'A2 UZ (member A12, 12EI/L^3)'),
                      nc_values('NC-DROPPED-COUPLING', 'coupling member omitted: frame B unloaded',
                                lambda ctx: _dropped_values(ctx, drop3, ('B2', 'B1', 'B0'), ('C', 'B21', 'B10'))),
                      nc_subtract_rounded(), nc_sign()])


def _dropped_values(ctx, modify, far_nodes, far_members):
    d2 = modify(_deep_copy(ctx.defn))
    near_nodes = [n for n in d2['nodes'] if n not in far_nodes]
    dn = {'nodes': {n: d2['nodes'][n] for n in near_nodes}, 'sections': d2['sections'],
          'members': [m for m in d2['members'] if m[1] in near_nodes and m[2] in near_nodes],
          'supports': {n: s for n, s in d2['supports'].items() if n in near_nodes},
          'loads': {n: l for n, l in d2['loads'].items() if n in near_nodes}}
    root = ctx.method[1]
    s = solve(dn, ('force', root))
    vals = {k: v for (k, v, c) in flatten(s, dn) if k in ctx.exp_map}
    for (k, v, c) in ctx.flat:
        parts = k.split('.')
        if parts[1] in far_nodes or parts[1] in far_members:
            vals[k] = ('sqrt', Fr(0)) if isinstance(v, tuple) else Fr(0)
    return vals


# =====================================================================================
# RF-LARGE

FAMILY_TEXT['RF-LARGE'] = ('Scale families at n = 10, 100, 1000 and 10000 members, axis-aligned and rotated by the exact '
                           'rational rotation Q3 (integer coordinates in both).  Determinate: a cantilever chain (member '
                           'length 3 m, tip force and moment) and a comb tree (spine plus one branch per spine node, '
                           'loads at every branch tip), solved by tree integration.  Indeterminate: a continuous beam '
                           '(n/2 spans of 6 m, fixed at S0, translation-pinned at every other support, loads at every '
                           'midspan), solved by the three-moment equation.  For n >= 1000 a stated subset of nodes and '
                           'members is published; every other value follows from the same closed form (--full).')
LARGE_N = (10, 100, 1000, 10000)


def sample_idx(n):
    return sorted(set([0, 1, 2, n // 4, n // 2, (3 * n) // 4, n - 1, n]))


def rot_defn(defn, Q, den):
    """Apply an exact rational rotation to coordinates, loads and spring directions (restraint sets must be
    rotation-invariant: full translation or rotation triples)."""
    d = _deep_copy(defn)
    d['nodes'] = {k: mv(Q, v) for k, v in defn['nodes'].items()}
    sups = {}
    for n, s in defn.get('supports', {}).items():
        rig = s.get('rigid', [])
        for trip in (PIN3, ROT3):
            if any(r in rig for r in trip) and not all(r in rig for r in trip):
                raise ValueError('restraints at %s are not rotation-invariant' % n)
        sups[n] = {'rigid': list(rig),
                   'springs': [(kind, tuple(int(c * den) if (c * den).denominator == 1 else c * den for c in mv(Q, dv)), k)
                               for (kind, dv, k) in s.get('springs', [])]}
    d['supports'] = sups
    d['loads'] = {n: {kk: mv(Q, vv) for kk, vv in ld.items()} for n, ld in defn.get('loads', {}).items()}
    return d


def rot_solution(sol, Q, defn):
    """Rotate every vector of a solution (support actions regrouped into vectors per node)."""
    out = {'u': {k: mv(Q, v) for k, v in sol['u'].items()}, 'th': {k: mv(Q, v) for k, v in sol['th'].items()},
           'springs': {k: [mv(Q, v) for v in vv] for k, vv in sol['springs'].items()}, 'mem': sol['mem'], 'reac': {}}
    for n, r in sol['reac'].items():
        new = {}
        for trip in (PIN3, ROT3):
            if trip[0] in r:
                v = mv(Q, tuple(r[t] for t in trip))
                for t, c in zip(trip, v):
                    new[t] = c
        out['reac'][n] = new
    return out


def scale_solution(sol, f):
    return {'u': {k: vs(f, v) for k, v in sol['u'].items()}, 'th': {k: vs(f, v) for k, v in sol['th'].items()},
            'reac': {n: {d: f * x for d, x in r.items()} for n, r in sol['reac'].items()},
            'springs': {n: [vs(f, v) for v in vv] for n, vv in sol['springs'].items()},
            'mem': {m: {k: (f * f * v if k.startswith('b2') else f * v) for k, v in rec.items()} for m, rec in sol['mem'].items()}}


def scale_loads(defn, f):
    d = _deep_copy(defn)
    d['loads'] = {n: {kk: vs(f, vv) for kk, vv in ld.items()} for n, ld in defn['loads'].items()}
    return d


def max_rotation(sol):
    return max(to_dec(dot(v, v)).sqrt() for v in sol['th'].values())


def lg_chain_defn(n):
    import math
    s = round(math.log2(1.264 / n ** 2))
    s2 = round(math.log2(24.5 / n))
    nodes = {'N%d' % i: (Fr(3 * i), Fr(0), Fr(0)) for i in range(n + 1)}
    members = [('M%d' % (i + 1), 'N%d' % i, 'N%d' % (i + 1), 'N') for i in range(n)]
    loads = {'N%d' % n: {'F': vs(Fr(2) ** s, (Fr(6), Fr(-3), Fr(9))), 'M': vs(Fr(2) ** s2, (Fr(3), Fr(6), Fr(-3)))}}
    return {'nodes': nodes, 'sections': {'N': SEC_N}, 'members': members, 'supports': {'N0': {'rigid': ALL6}},
            'loads': loads}


def lg_tree_defn(n):
    h = n // 2
    nodes = {'P0': (Fr(0), Fr(0), Fr(0))}
    members = []
    loads = {}
    for k in range(1, h + 1):
        nodes['P%d' % k] = (Fr(3 * k), Fr(0), Fr(0))
        if k % 2:
            nodes['B%d' % k] = (Fr(3 * k), Fr(3), Fr(0))
            F = (Fr(0), Fr(0), Fr(3 * ((k % 7) - 3)))
        else:
            nodes['B%d' % k] = (Fr(3 * k), Fr(0), Fr(3))
            F = (Fr(0), Fr(3 * ((k % 7) - 3)), Fr(0))
        members.append(('S%d' % k, 'P%d' % (k - 1), 'P%d' % k, 'N'))
        members.append(('Q%d' % k, 'P%d' % k, 'B%d' % k, 'N'))
        if any(c != 0 for c in F):
            loads['B%d' % k] = {'F': F}
    return {'nodes': nodes, 'sections': {'N': SEC_N}, 'members': members, 'supports': {'P0': {'rigid': ALL6}},
            'loads': loads}


def continuous_defn(spans, ell, Q, origin, py, pz, sec):
    """Nodes S0..Ss (supports) and C1..Cs (midspans); members A_j: S_{j-1}->C_j and B_j: C_j->S_j.
    S0 fixed in all six DOFs; S1..Ss translation-pinned; transverse loads (py_j, pz_j) in the local frame."""
    def g(xl):
        return tuple(origin[r] + sum(Q[r][c] * xl[c] for c in range(3)) for r in range(3))
    nodes = {}
    members = []
    for i in range(spans + 1):
        nodes['S%d' % i] = g((i * ell, 0, 0))
    for j in range(1, spans + 1):
        nodes['C%d' % j] = g(((2 * j - 1) * ell / 2, 0, 0))
        members.append(('A%d' % j, 'S%d' % (j - 1), 'C%d' % j, 'S'))
        members.append(('B%d' % j, 'C%d' % j, 'S%d' % j, 'S'))
    supports = {'S0': {'rigid': ALL6}}
    for i in range(1, spans + 1):
        supports['S%d' % i] = {'rigid': list(PIN3)}
    loads = {}
    for j in range(1, spans + 1):
        F = mv(Q, (Fr(0), py[j], pz[j]))
        if any(c != 0 for c in F):
            loads['C%d' % j] = {'F': F}
    return {'nodes': nodes, 'sections': {'S': sec}, 'members': members, 'supports': supports, 'loads': loads}


def lg_cont_defn(n, Q=I3, origin=(Fr(0), Fr(0), Fr(0)), ell=Fr(6), unit=Fr(32)):
    s = n // 2
    py = [Fr(0)] + [unit * 3 * ((j % 5) - 2) for j in range(1, s + 1)]
    pz = [Fr(0)] + [unit * 3 * ((j % 3) - 1) for j in range(1, s + 1)]
    return continuous_defn(s, ell, Q, origin, py, pz, SEC_N)


def _nc_offbyone(prefix_nodes):
    def fn(ctx):
        vals = {}
        full = ctx.sol
        for (k, v, c) in ctx.flat:
            parts = k.split('.')
            if parts[0] in ('u', 'th'):
                node = parts[1]
                pre = node.rstrip('0123456789')
                idx = int(node[len(pre):])
                if pre in prefix_nodes and idx >= 1 and '%s%d' % (pre, idx - 1) in full['u']:
                    src = full[parts[0]]['%s%d' % (pre, idx - 1)]
                    vals[k] = src['XYZ'.index(parts[2][1])]
        return vals
    return nc_values('NC-OFF-BY-ONE', 'node numbering shifted by one: each published node carries the value of '
                     'its predecessor', fn)


def _nc_drop_last_load():
    def modify(d):
        last = list(d['loads'])[-1]
        del d['loads'][last]
        return d
    return nc_resolved('NC-DROPPED-LOAD', 'the last authored nodal load is lost (for example a truncated load table)', modify)


def _nc_unrotated(base_sol, base_defn, sample):
    def fn(ctx):
        f = flatten(base_sol, base_defn, *sample, regions=None)
        return {k: v for (k, v, c) in f if k in ctx.exp_map}
    return nc_values('NC-UNROTATED', 'rotated model published with the axis-aligned components (global transform '
                     'not applied to results)', fn)


def _nc_simple_spans():
    def f(ctx):
        s = solve(ctx.defn, ctx.method, simple_spans=True)
        fl = flatten(s, ctx.defn, *ctx.case.get('sample', (None, None)))
        return nc_result('NC-SIMPLE-SPANS', 'continuity over the supports dropped (every span simply supported: '
                         'support moments zero, the fixed end ignored)', ctx, {k: v for (k, v, c) in fl if k in ctx.exp_map})
    return f


def build_large():
    for n in LARGE_N:
        big = n >= 1000
        # chain
        base = lg_chain_defn(n)
        smp = sample_idx(n)
        sample = ({'N%d' % i for i in smp}, {'M%d' % i for i in smp if i >= 1}) if big else (None, None)
        sol_ax = solve(base, ('tree', 'N0'))
        add_case(id='RF-LARGE-CHAIN-n%05d-AX' % n, family='RF-LARGE',
                 purpose='cantilever chain of %d members of 3 m along x (N0 fixed); tip force %s N and moment %s N*m '
                         '(powers of two chosen for a tip rotation near 1e-4).' % (
                             n, [fmt_input(c) for c in base['loads']['N%d' % n]['F']],
                             [fmt_input(c) for c in base['loads']['N%d' % n]['M']]),
                 defn=base, method=('tree', 'N0'), sol=sol_ax, sample=sample,
                 ncs=[_nc_offbyone(('N',)), _nc_drop_last_load(), nc_sign()],
                 finite=True, finite_reason='inputs differ from the n = 1000 case only by count; the represented '
                 'section is the only rounded input (coordinates and loads are exact binary64)')
        rdef = rot_defn(base, Q3, 3)
        sol_r = rot_solution(sol_ax, Q3, base)
        add_case(id='RF-LARGE-CHAIN-n%05d-ROT' % n, family='RF-LARGE',
                 purpose='the AX chain rotated by Q3 (members along (1,2,-2), integer coordinates); loads rotated exactly.',
                 defn=rdef, method=('tree', 'N0'), sol=sol_r, sample=sample,
                 checks=[_check_direct(rdef, ('tree', 'N0'), sol_r)],
                 ncs=[_nc_unrotated(sol_ax, base, sample), _nc_offbyone(('N',)), nc_sign()],
                 finite=True, finite_reason='as the AX case')
        # comb tree
        tb = lg_tree_defn(n)
        s0 = solve(tb, ('tree', 'P0'))
        import math
        sc = round(math.log2(1e-4 / float(max_rotation(s0))))
        tdef = scale_loads(tb, Fr(2) ** sc)
        tsol = scale_solution(s0, Fr(2) ** sc)
        h = n // 2
        ts = sample_idx(h)
        tsample = ({'P%d' % i for i in ts} | {'B%d' % i for i in ts if i >= 1},
                   {'S%d' % i for i in ts if i >= 1} | {'Q%d' % i for i in ts if i >= 1}) if big else (None, None)
        add_case(id='RF-LARGE-TREE-n%05d-AX' % n, family='RF-LARGE',
                 purpose='comb tree: spine P0..P%d along x (3 m members, P0 fixed), branch of 3 m at every spine node '
                         '(+y at odd k, +z at even k); branch-tip force 3*((k mod 7)-3)*2^%d N normal to the branch and '
                         'spine.' % (h, sc),
                 defn=tdef, method=('tree', 'P0'), sol=tsol, sample=tsample,
                 checks=[_check_direct(tdef, ('tree', 'P0'), tsol)],
                 ncs=[_nc_offbyone(('P', 'B')), _nc_drop_last_load(), nc_sign()],
                 finite=True, finite_reason='as the chain')
        trdef = rot_defn(tdef, Q3, 3)
        trsol = rot_solution(tsol, Q3, tdef)
        add_case(id='RF-LARGE-TREE-n%05d-ROT' % n, family='RF-LARGE',
                 purpose='the AX comb tree rotated by Q3.', defn=trdef, method=('tree', 'P0'), sol=trsol,
                 sample=tsample, checks=[_check_direct(trdef, ('tree', 'P0'), trsol)],
                 ncs=[_nc_unrotated(tsol, tdef, tsample), _nc_offbyone(('P', 'B')), nc_sign()],
                 finite=True, finite_reason='as the chain')
        # continuous beam
        s = n // 2
        cdef = lg_cont_defn(n)
        spec = {'spans': s, 'Q': I3}
        csol = solve(cdef, ('cont', spec))
        cs = sample_idx(s)
        csample = ({'S%d' % i for i in cs} | {'C%d' % i for i in cs if i >= 1},
                   {'A%d' % i for i in cs if i >= 1} | {'B%d' % i for i in cs if i >= 1}) if big else (None, None)
        add_case(id='RF-LARGE-CONT-n%05d-AX' % n, family='RF-LARGE',
                 purpose='continuous beam of %d spans of 6 m along x (%d members); S0 fixed, S1..S%d translation-pinned; '
                         'midspan forces 96*((j mod 5)-2) N along y and 96*((j mod 3)-1) N along z.' % (s, n, s),
                 defn=cdef, method=('cont', spec), sol=csol, sample=csample,
                 checks=[_check_direct(cdef, ('force', 'S0'), csol)] if n <= 10 else [],
                 ncs=[_nc_simple_spans(), _nc_offbyone(('C',)), _nc_drop_last_load(), nc_sign()],
                 finite=True, finite_reason='as the chain')
        crdef = lg_cont_defn(n, Q=Q3)
        rspec = {'spans': s, 'Q': Q3}
        crsol = rot_solution(csol, Q3, cdef)
        add_case(id='RF-LARGE-CONT-n%05d-ROT' % n, family='RF-LARGE',
                 purpose='the AX continuous beam rotated by Q3.', defn=crdef, method=('cont', rspec), sol=crsol,
                 sample=csample, checks=[_check_direct(crdef, ('cont', rspec), crsol)],
                 ncs=[_nc_unrotated(csol, cdef, csample), _nc_simple_spans(), nc_sign()],
                 finite=True, finite_reason='as the chain')


def _check_direct(defn, method, sol):
    """Assert that an independently transformed expectation equals a direct solve of the transformed model."""
    def chk(_):
        s2 = solve(defn, method)
        a = flatten(sol, defn)
        b = flatten(s2, defn)
        for (k, v, c), (k2, v2, c2) in zip(a, b):
            if k != k2 or v != v2:
                raise AssertionError('transformed expectation differs from direct solve at %s' % k)
    return chk


# =====================================================================================
# RF-INVARIANCE

FAMILY_TEXT['RF-INVARIANCE'] = ('Exactly transformed expectations of four base cases whose restraint sets are rotation-invariant '
                                '(full translation or rotation triples, springs as directions): exact rational rotations Q3 and '
                                'Q9 (base coordinates and loads are multiples of 9, so rotated inputs are exact binary64 '
                                'integers or dyadic values), origin offsets (1e3 and 1e6 m), node and member relabelling '
                                'with reversed member directions, and the same physics in mm/N/MPa (moments N*mm).  Each '
                                'transformed expectation is computed by transforming the base solution and is asserted '
                                'identical to a direct solve of the transformed model.')
INV_UNITS_MM = {'length': 'mm', 'force': 'N', 'moment': 'N*mm', 'rotation': 'rad', 'modulus': 'MPa'}


def inv_bases():
    k1 = Fr(90000, 2 ** 20)
    xs = [0, 18, 27, 54, 63, 81]
    b1 = {'nodes': {'N%d' % i: (Fr(x), Fr(0), Fr(0)) for i, x in enumerate(xs)}, 'sections': {'N': SEC_N},
          'members': [('M%d' % i, 'N%d' % (i - 1), 'N%d' % i, 'N') for i in range(1, 6)],
          'supports': dict([('N0', {'rigid': list(PIN3), 'springs': [('r', (1, 0, 0), k1)]})] +
                           [('N%d' % i, {'rigid': list(PIN3)}) for i in range(1, 6)]),
          'loads': {'N5': {'M': (Fr(9, 2 ** 20), Fr(0), Fr(0))}}}
    b2 = {'nodes': {'A0': (Fr(0), Fr(0), Fr(0)), 'A1': (Fr(9), Fr(0), Fr(0)), 'A2': (Fr(9), Fr(9), Fr(0))},
          'sections': {'N': SEC_N}, 'members': [('M1', 'A0', 'A1', 'N'), ('M2', 'A1', 'A2', 'N')],
          'supports': {'A0': {'rigid': list(PIN3), 'springs': [('r', (1, 0, 0), Fr(810000, 2 ** 16)),
                                                               ('r', (0, 1, 0), K_ORD_ROT), ('r', (0, 0, 1), K_ORD_ROT)]}},
          'loads': {'A2': {'F': (Fr(0), Fr(0), Fr(9, 2 ** 16)), 'M': (Fr(0), Fr(9, 2 ** 10), Fr(0))}}}
    b3 = continuous_defn(4, Fr(18), I3, (Fr(0), Fr(0), Fr(0)), [0, Fr(36), Fr(-72), Fr(0), Fr(108)],
                         [0, Fr(-36), Fr(0), Fr(72), Fr(36)], SEC_N)
    tb = lg_tree_defn(100)
    import math
    s0 = solve(tb, ('tree', 'P0'))
    sc = round(math.log2(1e-4 / float(max_rotation(s0))))
    b4 = scale_loads(tb, Fr(2) ** sc)
    return [('PINTOR', b1, ('pintor', 'N0', 'N5'), 'pinned torsion chain (lengths 18, 9, 27, 9, 18 m along x, every node '
             'translation-pinned, root rotational spring k = 90000*2^-20 N*m/rad along the chain, tip torque 9*2^-20 N*m): '
             'pure torsion with a soft rigid mode'),
            ('LFRAME', b2, ('tree', 'A0'), 'L-frame A0 (0,0,0) -> A1 (9,0,0) -> A2 (9,9,0); root translation-pinned with '
             'rotational springs 810000*2^-16 N*m/rad (X) and 1e6 N*m/rad (Y, Z); tip force (0,0,9*2^-16) N and moment '
             '(0,9*2^-10,0) N*m'),
            ('CONT4', b3, ('cont', {'spans': 4, 'Q': I3}), 'continuous beam, 4 spans of 18 m along x, S0 fixed, others '
             'translation-pinned, midspan forces y (36,-72,0,108) N and z (-36,0,72,36) N'),
            ('TREE100', b4, ('tree', 'P0'), 'the RF-LARGE comb tree with 100 members')]


def offset_defn(defn, O):
    d = _deep_copy(defn)
    d['nodes'] = {k: vadd(v, O) for k, v in defn['nodes'].items()}
    return d


def relabel_maps(defn):
    names = list(defn['nodes'])
    nmap = {n: 'R%d' % (len(names) - 1 - i) for i, n in enumerate(names)}
    mems = list(defn['members'])
    mmap = {m[0]: 'E%d' % (len(mems) - i) for i, m in enumerate(mems)}
    return nmap, mmap


def relabel_defn(defn):
    nmap, mmap = relabel_maps(defn)
    d = {'nodes': {}, 'sections': defn['sections'], 'members': [], 'supports': {}, 'loads': {}}
    for n in sorted(defn['nodes'], key=lambda x: int(nmap[x][1:])):
        d['nodes'][nmap[n]] = defn['nodes'][n]
    for m in reversed(defn['members']):
        d['members'].append((mmap[m[0]], nmap[m[2]], nmap[m[1]], m[3]))
    for n, s in defn.get('supports', {}).items():
        d['supports'][nmap[n]] = s
    for n, l in defn.get('loads', {}).items():
        d['loads'][nmap[n]] = l
    return d


def relabel_solution(sol, defn):
    nmap, mmap = relabel_maps(defn)
    out = {'u': {nmap[k]: v for k, v in sol['u'].items()}, 'th': {nmap[k]: v for k, v in sol['th'].items()},
           'reac': {nmap[k]: v for k, v in sol['reac'].items()},
           'springs': {nmap[k]: v for k, v in sol['springs'].items()}, 'mem': {}}
    for mid, rec in sol['mem'].items():
        r = dict(rec)
        r['b2_i'], r['b2_j'] = rec['b2_j'], rec['b2_i']   # every member is reversed
        out['mem'][mmap[mid]] = r
    return out


def units_defn(defn):
    d = {'nodes': {k: vs(1000, v) for k, v in defn['nodes'].items()}, 'members': defn['members'], 'sections': {},
         'supports': {}, 'loads': {}}
    for sid, s in defn['sections'].items():
        t = dict(s)
        for kk in ('E', 'G'):
            if kk in t:
                t[kk] = Fr(t[kk]) / 10 ** 6
        t['OD'] = Fr(t['OD']) * 1000
        t['ID'] = Fr(t['ID']) * 1000
        d['sections'][sid] = t
    for n, s in defn.get('supports', {}).items():
        d['supports'][n] = {'rigid': list(s.get('rigid', [])),
                            'springs': [(kind, dv, Fr(k) / 1000 if kind == 't' else Fr(k) * 1000)
                                        for (kind, dv, k) in s.get('springs', [])]}
    for n, l in defn.get('loads', {}).items():
        d['loads'][n] = {kk: (vv if kk == 'F' else vs(1000, vv)) for kk, vv in l.items()}
    return d


def units_solution(sol):
    return {'u': {k: vs(1000, v) for k, v in sol['u'].items()}, 'th': sol['th'],
            'reac': {n: {d: (x if d[0] == 'U' else 1000 * x) for d, x in r.items()} for n, r in sol['reac'].items()},
            'springs': {n: vv for n, vv in sol['springs'].items()},  # rescaled below per kind
            'mem': {m: {'N': r['N'], 'T': 1000 * r['T'], 'tw': r['tw'], 'ext': 1000 * r['ext'],
                        'b2_i': 10 ** 6 * r['b2_i'], 'b2_mid': 10 ** 6 * r['b2_mid'], 'b2_j': 10 ** 6 * r['b2_j']}
                    for m, r in sol['mem'].items()}}


def units_solution_full(sol, defn):
    out = units_solution(sol)
    sp = {}
    for n, vv in sol['springs'].items():
        kinds = [kind for (kind, _, _) in defn['supports'][n].get('springs', [])]
        sp[n] = [v if kind == 't' else vs(1000, v) for v, kind in zip(vv, kinds)]
    out['springs'] = sp
    return out


def _nc_origin_moments():
    def fn(ctx):
        vals = {}
        X = ctx.model.nodes
        for n, sup in ctx.defn.get('supports', {}).items():
            Fv = Z3
            for d in sup.get('rigid', []):
                if d[0] == 'U':
                    Fv = vadd(Fv, vs(ctx.sol['reac'][n][d], dof_axis(d)))
            for i, (kind, dv, k) in enumerate(sup.get('springs', [])):
                if kind == 't':
                    Fv = vadd(Fv, ctx.sol['springs'][n][i])
            extra = cross(X[n], Fv)
            for d in sup.get('rigid', []):
                if d[0] == 'R':
                    vals['R.%s.%s' % (n, d)] = ctx.sol['reac'][n][d] + dot(extra, dof_axis(d))
            rot_springs = [i for i, (kind, dv, k) in enumerate(sup.get('springs', [])) if kind == 'r']
            for i in rot_springs:
                dv = sup['springs'][i][1]
                part = vs(dot(extra, dv) / dot(dv, dv), dv)
                v = vadd(ctx.sol['springs'][n][i], part)
                for ax, c in enumerate('XYZ'):
                    vals['S.%s.%d.M%s' % (n, i, c)] = v[ax]
        return {k: v for k, v in vals.items() if k in ctx.exp_map}
    return nc_values('NC-ORIGIN-MOMENTS', 'support moments reported about the global origin instead of about the '
                     'supported node (large origin offsets make this obvious)', fn)


def _nc_unconverted(base_sol, base_defn):
    def fn(ctx):
        f = flatten(base_sol, base_defn)
        return {k: v for (k, v, c) in f if k in ctx.exp_map}
    return nc_values('NC-WRONG-UNIT', 'mm/N/MPa model published with the m/N/Pa values (translations and moments '
                     'not converted)', fn)


def _nc_old_numbering(base_sol, base_defn):
    def fn(ctx):
        nmap, mmap = relabel_maps(base_defn)
        inv = {v: k for k, v in nmap.items()}
        names = list(base_defn['nodes'])
        vals = {}
        for (k, v, c) in ctx.flat:
            p = k.split('.')
            if p[0] in ('u', 'th'):
                idx = int(p[1][1:])
                old = names[idx]  # value of the base node that had the same position in the list
                vals[k] = base_sol[p[0]][old]['XYZ'.index(p[2][1])]
        return vals
    return nc_values('NC-OLD-NUMBERING', 'relabelled model published against the original node order', fn)


def _nc_unswapped_ends():
    def fn(ctx):
        vals = {}
        for m in ctx.defn['members']:
            ki, kj = 'Mb.%s.i' % m[0], 'Mb.%s.j' % m[0]
            if ki in ctx.exp_map:
                vals[ki], vals[kj] = ctx.exp_map[kj][0], ctx.exp_map[ki][0]
        return vals
    return nc_values('NC-UNSWAPPED-ENDS', 'end stations not remapped after member reversal (end-i and end-j bending '
                     'magnitudes exchanged)', fn)


def build_invariance():
    for (bname, bdefn, bmethod, btext) in inv_bases():
        bsol = solve(bdefn, bmethod)
        add_case(id='RF-INVARIANCE-%s-BASE' % bname, family='RF-INVARIANCE', purpose='base: ' + btext + '.',
                 defn=bdefn, method=bmethod, sol=bsol, ncs=[nc_sign(), nc_subtract_rounded()])
        if bname != 'TREE100':
            for qn, (Q, den, qtext) in ROTATIONS.items():
                rdef = rot_defn(bdefn, Q, den)
                rmethod = ('cont', {'spans': bmethod[1]['spans'], 'Q': Q}) if bmethod[0] == 'cont' else bmethod
                rsol = rot_solution(bsol, Q, bdefn)
                add_case(id='RF-INVARIANCE-%s-ROT-%s' % (bname, qn), family='RF-INVARIANCE',
                         purpose='base %s rotated by %s = %s (%s); coordinates, loads and spring directions rotated '
                                 'exactly.' % (bname, qn, [[fmt_input(c) for c in row] for row in Q], qtext),
                         defn=rdef, method=rmethod, sol=rsol, checks=[_check_direct(rdef, rmethod, rsol)],
                         ncs=[_nc_unrotated(bsol, bdefn, (None, None)), nc_sign()])
        for olab, O in (('1e3', (Fr(1000), Fr(-1000), Fr(1000))), ('1e6', (Fr(10 ** 6), Fr(-10 ** 6), Fr(10 ** 6)))):
            odef = offset_defn(bdefn, O)
            add_case(id='RF-INVARIANCE-%s-OFF-%s' % (bname, olab), family='RF-INVARIANCE',
                     purpose='base %s translated by %s m; every expectation equals the base (reactions are per node).' % (
                         bname, [fmt_input(c) for c in O]),
                     defn=odef, method=bmethod, sol=bsol, checks=[_check_direct(odef, bmethod, bsol)],
                     ncs=[_nc_origin_moments(), nc_sign()])
        ldef = relabel_defn(bdefn)
        nmap, mmap = relabel_maps(bdefn)
        if bmethod[0] == 'cont':
            lmethod = ('force', nmap['S0'])
        elif bmethod[0] == 'pintor':
            lmethod = ('pintor', nmap[bmethod[1]], nmap[bmethod[2]])
        else:
            lmethod = (bmethod[0], nmap[bmethod[1]])
        lsol = relabel_solution(bsol, bdefn)
        add_case(id='RF-INVARIANCE-%s-RELABEL' % bname, family='RF-INVARIANCE',
                 purpose='base %s with nodes renumbered in reverse (R0 = last base node), members renumbered in reverse '
                         'and every member direction reversed (i and j exchanged).' % bname,
                 defn=ldef, method=lmethod, sol=lsol, checks=[_check_direct(ldef, lmethod, lsol)],
                 ncs=[_nc_old_numbering(bsol, bdefn), _nc_unswapped_ends(), nc_sign()],
                 notes=['node map: %s' % ', '.join('%s->%s' % kv for kv in list(nmap.items())[:12]) +
                        (' ...' if len(nmap) > 12 else ''),
                        'member map: %s (all reversed)' % ', '.join('%s->%s' % kv for kv in list(mmap.items())[:12]) +
                        (' ...' if len(mmap) > 12 else '')])
        if bname != 'TREE100':
            udef = units_defn(bdefn)
            usol = units_solution_full(bsol, bdefn)
            add_case(id='RF-INVARIANCE-%s-UNITS-mm' % bname, family='RF-INVARIANCE',
                     purpose='base %s authored in mm/N/MPa (moments N*mm, rotational springs N*mm/rad, translational '
                             'springs N/mm); expectations are the base values converted exactly.' % bname,
                     defn=udef, method=bmethod, sol=usol, units=INV_UNITS_MM,
                     checks=[_check_direct(udef, bmethod, usol)],
                     ncs=[_nc_unconverted(bsol, bdefn), nc_sign()])


# =====================================================================================
# RF-RANGE

FAMILY_TEXT['RF-RANGE'] = ('Exact power-of-two scaling of three base problems toward both ends of the binary64 normal range: '
                           'lengths (coordinates and OD/ID) by 2^pl, moduli by 2^pm, loads by 2^pf (moments by 2^(pf+pl)), '
                           'translational springs by 2^(pm+pl), rotational springs by 2^(pm+3pl).  Then translations and '
                           'extensions scale by 2^(pf-pm-pl), rotations and twists by 2^(pf-pm-2pl), forces by 2^pf and '
                           'moments by 2^(pf+pl), exactly.  Every published input and expected value is a normal binary64 '
                           'number; the derived-quantity envelope (A, I, EI, element coefficients) is reported.  Plus two '
                           'loaded cantilevers at the PHYS-R4 geometry (OD 4e-77 m, wall 1e-77 m, L 1 m).')
MIN_NORMAL = Fr(1, 2 ** 1022)
MAX_FINITE = Fr((2 ** 53 - 1) * 2 ** 971)


def scale_defn(defn, pl, pm, pf):
    L, Mu, Ph = Fr(2) ** pl, Fr(2) ** pm, Fr(2) ** pf
    d = {'nodes': {k: vs(L, v) for k, v in defn['nodes'].items()}, 'members': defn['members'], 'sections': {},
         'supports': {}, 'loads': {}}
    for sid, s in defn['sections'].items():
        t = dict(s)
        for kk in ('E', 'G'):
            if kk in t:
                t[kk] = Fr(t[kk]) * Mu
        t['OD'], t['ID'] = Fr(t['OD']) * L, Fr(t['ID']) * L
        d['sections'][sid] = t
    for n, s in defn.get('supports', {}).items():
        d['supports'][n] = {'rigid': list(s.get('rigid', [])),
                            'springs': [(kind, dv, Fr(k) * Mu * (L if kind == 't' else L ** 3))
                                        for (kind, dv, k) in s.get('springs', [])]}
    for n, l in defn.get('loads', {}).items():
        d['loads'][n] = {kk: vs(Ph if kk == 'F' else Ph * L, vv) for kk, vv in l.items()}
    return d


def scale_range_solution(sol, pl, pm, pf):
    t = Fr(2) ** (pf - pm - pl)
    r = Fr(2) ** (pf - pm - 2 * pl)
    f = Fr(2) ** pf
    m = Fr(2) ** (pf + pl)
    return {'u': {k: vs(t, v) for k, v in sol['u'].items()}, 'th': {k: vs(r, v) for k, v in sol['th'].items()},
            'reac': {n: {d: (f if d[0] == 'U' else m) * x for d, x in rr.items()} for n, rr in sol['reac'].items()},
            'springs': {n: vv for n, vv in sol['springs'].items()},
            'mem': {k: {'N': f * v['N'], 'T': m * v['T'], 'tw': r * v['tw'], 'ext': t * v['ext'],
                        'b2_i': m * m * v['b2_i'], 'b2_mid': m * m * v['b2_mid'], 'b2_j': m * m * v['b2_j']}
                    for k, v in sol['mem'].items()}}


def scale_range_solution_full(sol, defn, pl, pm, pf):
    out = scale_range_solution(sol, pl, pm, pf)
    f, m = Fr(2) ** pf, Fr(2) ** (pf + pl)
    sp = {}
    for n, vv in sol['springs'].items():
        kinds = [kind for (kind, _, _) in defn['supports'][n].get('springs', [])]
        sp[n] = [vs(f if kind == 't' else m, v) for v, kind in zip(vv, kinds)]
    out['springs'] = sp
    return out


def log2_abs(q):
    q = abs(Fr(q))
    e = q.numerator.bit_length() - q.denominator.bit_length()
    if Fr(2) ** e > q:
        e -= 1
    return e


def range_envelope(defn, flat):
    """log2 envelope of inputs, derived section and element quantities (reported) and published values (checked)."""
    ar = Exact()
    inputs = []
    for v in defn['nodes'].values():
        inputs += [c for c in v if c != 0]
    for s in defn['sections'].values():
        inputs += [Fr(x) for x in s.values()]
    for s in defn.get('supports', {}).values():
        inputs += [Fr(k) for (_, _, k) in s.get('springs', [])]
    for l in defn.get('loads', {}).values():
        for vv in l.values():
            inputs += [Fr(c) for c in vv if c != 0]
    derived = []
    for s in defn['sections'].values():
        E, OD, ID = Fr(s['E']), Fr(s['OD']), Fr(s['ID'])
        G = Fr(s['G']) if 'G' in s else E / (2 * (1 + Fr(s['nu'])))
        A = PI_Q * (OD * OD - ID * ID) / 4
        I = PI_Q * (OD ** 4 - ID ** 4) / 64
        derived += [('OD^4', OD ** 4), ('ID^4', ID ** 4), ('A', A), ('I', I), ('EA', E * A), ('EI', E * I), ('GJ', 2 * G * I)]
    model = Model(defn, ar)
    for m in model.members:
        dx = vsub(model.nodes[m['j']], model.nodes[m['i']])
        L = ar.sqrt(dot(dx, dx))
        s = model.sections[m['sec']]
        derived += [('EA/L', s['EA'] / L), ('12EI/L^3', 12 * s['EI'] / L ** 3), ('6EI/L^2', 6 * s['EI'] / L ** 2),
                    ('4EI/L', 4 * s['EI'] / L), ('GJ/L', s['GJ'] / L)]
    outs = [(v[1] if isinstance(v, tuple) else v) for (k, v, c) in flat]
    outs = [x for x in outs if x != 0]
    # published bending magnitudes are square roots: check the magnitude, not its square
    mags = []
    for (k, v, c) in flat:
        if isinstance(v, tuple):
            if v[1] != 0:
                mags.append(v[1])
        elif v != 0:
            mags.append(v * v)
    def normal(q):
        return MIN_NORMAL <= abs(q) <= MAX_FINITE
    in_ok = all(normal(x) for x in inputs)
    out_ok = all(MIN_NORMAL ** 2 <= q <= MAX_FINITE ** 2 for q in mags)
    dmin = min(derived, key=lambda t: abs(t[1]))
    dmax = max(derived, key=lambda t: abs(t[1]))
    return {'inputs_log2_range': [min(log2_abs(x) for x in inputs), max(log2_abs(x) for x in inputs)],
            'published_values_log2_range': [min(log2_abs(q) for q in mags) // 2, max(log2_abs(q) for q in mags) // 2],
            'all_inputs_normal': in_ok, 'all_published_values_normal': out_ok,
            'derived_quantities_log2_range': [[dmin[0], log2_abs(dmin[1])], [dmax[0], log2_abs(dmax[1])]],
            'derived_quantities_all_normal': all(normal(q) for _, q in derived)}


RANGE_VECTORS = [('L-240', -240, 0, 0), ('L+240', 240, 0, 0), ('E-1000', 0, -1000, 0), ('E+960', 0, 960, 0),
                 ('F-960', 0, 0, -960), ('F+960', 0, 0, 960), ('LEF-small', -200, -300, -600),
                 ('LEF-large', 200, 300, 600), ('SIM-a', -120, 500, 260), ('SIM-b', 120, -500, -260)]


def build_range():
    bases = [('CHAIN', chain_defn(5, 'T', Fr('0.0216'), Fr('0.0216') * THETA0), ('tree', 'N0'), 'RF-CHAIN-T-n05-r1e-08'),
             ('SKEW', skew_defn(SKEW_DIRS['122'], 'T-CANT-OFF', Fr('0.0144')), ('tree', 'N0'), 'RF-SKEW-T-CANT-OFF-122-r1e-08'),
             ('CONT', lg_cont_defn(10), ('cont', {'spans': 5, 'Q': I3}), 'RF-LARGE-CONT-n00010-AX')]
    for (bname, bdefn, bmethod, bid) in bases:
        bsol = solve(bdefn, bmethod)
        for (vlab, pl, pm, pf) in RANGE_VECTORS:
            sdef = scale_defn(bdefn, pl, pm, pf)
            ssol = scale_range_solution_full(bsol, bdefn, pl, pm, pf)
            fac_rot = Fr(2) ** (pf - pm - 2 * pl)
            fac_tr = Fr(2) ** (pf - pm - pl)

            def rot_as_trans(ctx, bsol=bsol, bdefn=bdefn, pl=pl, pm=pm, pf=pf, fr_=fac_rot, ft=fac_tr):
                vals = {}
                for (k, v, c) in ctx.flat:
                    if k.startswith('th.') or k.startswith('tw.'):
                        vals[k] = v / fr_ * ft
                return vals

            def unscaled(ctx, bsol=bsol, bdefn=bdefn):
                f = flatten(bsol, bdefn)
                return {k: v for (k, v, c) in f if k in ctx.exp_map}
            add_case(id='RF-RANGE-%s-%s' % (bname, vlab), family='RF-RANGE',
                     purpose='%s scaled by pl = %d, pm = %d, pf = %d (base %s).  A correct solve publishes these exactly '
                             'scaled values; a named range refusal is not a wrong number but does not pass this reference; '
                             'a zero, infinite, NaN or unscaled value is a failure.' % (bid, pl, pm, pf, bid),
                     defn=sdef, method=bmethod, sol=ssol, checks=[_check_direct(sdef, bmethod, ssol)],
                     ncs=[nc_values('NC-UNSCALED', 'base-case values published for the scaled model', unscaled),
                          nc_values('NC-ROTATION-SCALED-AS-TRANSLATION', 'rotations and twists scaled with the translation '
                                    'factor 2^(pf-pm-pl) instead of 2^(pf-pm-2pl)', rot_as_trans), nc_sign()],
                     range_info={'pl': pl, 'pm': pm, 'pf': pf, 'base': bid})
    # PHYS-R4-comparable loaded cantilevers
    for lab, E, loads, text in (
            ('THIN-A', Fr(2) ** 200, {'F': (Fr(0), Fr('3.8e-251'), Fr('-1.9e-251')), 'M': (Fr('1.7e-251'), Fr(0), Fr(0))},
             'E = 2^200 Pa so that small-rotation loads (~1e-251 N) are normal numbers; rotations ~1e-4 rad'),
            ('THIN-B', Fr(1), {'F': (Fr(0), Fr('1e-300'), Fr(0)), 'M': (Fr('1e-300'), Fr(0), Fr(0))},
             'E = 1 Pa as in PHYS-R4; loads 1e-300 give rotations of order 1e6 rad: linear-theory arithmetic only, far '
             'outside small-rotation validity')):
        defn = {'nodes': {'N0': (Fr(0), Fr(0), Fr(0)), 'N1': (Fr(1), Fr(0), Fr(0))},
                'sections': {'T': {'E': E, 'nu': Fr('0.1'), 'OD': Fr('4e-77'), 'ID': Fr('2e-77')}},
                'members': [('M1', 'N0', 'N1', 'T')], 'supports': {'N0': {'rigid': ALL6}}, 'loads': {'N1': loads}}
        add_case(id='RF-RANGE-%s' % lab, family='RF-RANGE',
                 purpose='cantilever L = 1 m at the PHYS-R4 geometry (OD 4e-77 m, ID 2e-77 m, nu 0.1, G = E/(2(1+nu))); '
                         '%s.  PHYS-R4 itself (pressure-only stress) keeps its frozen reference and is not re-derived.  A '
                         'correct solve publishes these values.' % text,
                 defn=defn, method=('tree', 'N0'), ncs=[nc_sign()], range_info={'phys_r4_geometry': True})


# =====================================================================================
# RF-ZERO

FAMILY_TEXT['RF-ZERO'] = ('Structural zeros with derived zero scales: symmetry (fixed-fixed beam, centre load), pure torsion '
                          'of a skew cantilever (zero translation, force and bending), a planar frame loaded in its plane '
                          '(zero out-of-plane response), and a tiny nonzero case whose values (1.7e-12 m, 4.6e-10 rad) '
                          'sit below a naive absolute 1e-9 floor but equal their own class scales.')


def _nc_spurious(keys_prefix, factor=Fr(1, 10 ** 8)):
    def fn(ctx):
        vals = {}
        for (k, v, c) in ctx.flat:
            if v == 0 and any(k.startswith(p) for p in keys_prefix):
                vals[k] = factor * Fr(ctx.scales[c][0])
        return vals
    return nc_values('NC-SPURIOUS-NONZERO', 'each structural zero of the listed kinds published as 1e-8 times its class '
                     'scale (for example symmetry or plane leakage from a wrong transform)', fn)


def _nc_zeroed():
    def f(ctx):
        vals = {k: (('sqrt', Fr(0)) if isinstance(v, tuple) else Fr(0)) for (k, v, c) in ctx.flat
                if (v[1] if isinstance(v, tuple) else v) != 0 and not (k.startswith('R.') or k.startswith('S.'))}
        naive = all(absval(ctx.exp_map[k][0]) <= CRIT_D for k in vals)
        return nc_result('NC-ZEROED', 'every nonzero displacement and member value published as zero (lost '
                         'sub-quantum result)', ctx, vals,
                         extra={'naive_absolute_1e-9_floor_accepts_this_defect': naive})
    return f


def build_zero():
    nodes = {'N%d' % i: (Fr(i), Fr(0), Fr(0)) for i in range(5)}
    add_case(id='RF-ZERO-SYM', family='RF-ZERO',
             purpose='fixed-fixed beam, nodes x = 0..4 m, N0 and N4 fixed, centre load F = (0, 1000, 500) N at N2: '
                     'rotations at N2, all axial and torsional quantities and the axial/torsional reactions are exact '
                     'zeros; symmetric pairs are equal.',
             defn={'nodes': nodes, 'sections': {'N': SEC_N}, 'members': [('M%d' % i, 'N%d' % (i - 1), 'N%d' % i, 'N') for i in range(1, 5)],
                   'supports': {'N0': {'rigid': ALL6}, 'N4': {'rigid': ALL6}}, 'loads': {'N2': {'F': (Fr(0), Fr(1000), Fr(500))}}},
             method=('force', 'N0'), ncs=[_nc_spurious(('th.N2', 'u.', 'T.', 'N.')), nc_sign()])
    add_case(id='RF-ZERO-TORSION', family='RF-ZERO',
             purpose='cantilever N0 (0,0,0) fixed -> N1 (1,2,2), tip moment (1,2,2) N*m (torque 3 N*m along the member): '
                     'pure torsion, so every translation, force and bending moment is an exact zero.',
             defn={'nodes': {'N0': (Fr(0), Fr(0), Fr(0)), 'N1': (Fr(1), Fr(2), Fr(2))}, 'sections': {'N': SEC_N},
                   'members': [('M1', 'N0', 'N1', 'N')], 'supports': {'N0': {'rigid': ALL6}},
                   'loads': {'N1': {'M': (Fr(1), Fr(2), Fr(2))}}},
             method=('tree', 'N0'), ncs=[_nc_spurious(('u.', 'Mb.', 'N.', 'R.N0.U')), nc_sign()])
    add_case(id='RF-ZERO-PLANAR', family='RF-ZERO',
             purpose='L-frame N0 (0,0,0) fixed -> N1 (2,0,0) -> N2 (2,2,0); in-plane loads F = (100, -200, 0) N at N2 and '
                     'M = (0, 0, 50) N*m at N1: out-of-plane translation UZ, rotations RX and RY, torques and the '
                     'reactions UZ, RX, RY are exact zeros.',
             defn={'nodes': {'N0': (Fr(0), Fr(0), Fr(0)), 'N1': (Fr(2), Fr(0), Fr(0)), 'N2': (Fr(2), Fr(2), Fr(0))},
                   'sections': {'N': SEC_N}, 'members': [('M1', 'N0', 'N1', 'N'), ('M2', 'N1', 'N2', 'N')],
                   'supports': {'N0': {'rigid': ALL6}},
                   'loads': {'N2': {'F': (Fr(100), Fr(-200), Fr(0))}, 'N1': {'M': (Fr(0), Fr(0), Fr(50))}}},
             method=('tree', 'N0'), ncs=[_nc_spurious(('u.N1.UZ', 'u.N2.UZ', 'th.N1.RX', 'th.N1.RY', 'th.N2.RX', 'th.N2.RY', 'T.')),
                                         nc_sign()])
    add_case(id='RF-ZERO-TINY', family='RF-ZERO',
             purpose='cantilever N0 fixed -> N1 (2,0,0), tip F = (1e-3, 0, 0) N and M = (1e-3, 0, 0) N*m: u_x = 2e-3/EA '
                     '(1.7e-12 m) and theta_x = 2e-3/GJ (4.6e-10 rad) are below a naive absolute 1e-9 floor but are the '
                     'class scales themselves, so the criterion compares them relatively; every other displacement is an '
                     'exact zero at those scales.',
             defn={'nodes': {'N0': (Fr(0), Fr(0), Fr(0)), 'N1': (Fr(2), Fr(0), Fr(0))}, 'sections': {'N': SEC_N},
                   'members': [('M1', 'N0', 'N1', 'N')], 'supports': {'N0': {'rigid': ALL6}},
                   'loads': {'N1': {'F': (Fr('0.001'), Fr(0), Fr(0)), 'M': (Fr('0.001'), Fr(0), Fr(0))}}},
             method=('tree', 'N0'), ncs=[nc_round6(), _nc_zeroed(), nc_sign()])


# =====================================================================================
# RF-FINITE (dedicated cases; every other case also carries its finite_input comparison)

FAMILY_TEXT['RF-FINITE'] = ('Finite-accuracy budgets.  Every case publishes the exact solution for the binary64-rounded '
                            'inputs (Decimal.from_float decoding) next to the intended one, and the largest per-quantity '
                            'difference normalized by the criterion.  Dedicated cases: chains with node coordinates at '
                            'multiples of 1/3 m (at the origin and offset by 1e3 and 1e6 m) and of 1/10 and 1/30 m (offset 1e6 m), and an L-frame '
                            'rotated by Q9 without multiple-of-9 coordinates (irrational represented lengths).')


def build_finite():
    for lab, O, step in (('THIRDS-O0', Fr(0), Fr(1, 3)), ('THIRDS-O1e3', Fr(1000), Fr(1, 3)),
                         ('THIRDS-O1e6', Fr(10 ** 6), Fr(1, 3)), ('TENTHS-O1e6', Fr(10 ** 6), Fr(1, 10)),
                         ('THIRTIETHS-O1e6', Fr(10 ** 6), Fr(1, 30))):
        nodes = {'N%d' % i: (O + i * step, Fr(0), Fr(0)) for i in range(11)}
        defn = {'nodes': nodes, 'sections': {'N': SEC_N},
                'members': [('M%d' % i, 'N%d' % (i - 1), 'N%d' % i, 'N') for i in range(1, 11)],
                'supports': {'N0': {'rigid': ALL6}}, 'loads': {'N10': {'F': (Fr(10), Fr(5), Fr(-3)), 'M': (Fr(2), Fr(0), Fr(0))}}}
        add_case(id='RF-FINITE-%s' % lab, family='RF-FINITE',
                 purpose='cantilever chain of 10 members along x, nodes at x = %s + i*%s m (i = 0..10), N0 fixed, tip '
                         'F = (10, 5, -3) N and M = (2, 0, 0) N*m.  Intended coordinates are not binary64 numbers; the '
                         'rounded coordinates change member lengths by up to half an ulp of the offset.' % (
                             fmt_input(O), fmt_input(step)),
                 defn=defn, method=('tree', 'N0'), ncs=[nc_sign(), nc_subtract_rounded()])
    x1 = mv(Q9, (Fr(2), Fr(0), Fr(0)))
    x2 = mv(Q9, (Fr(2), Fr(0), Fr(2)))
    defn = {'nodes': {'N0': (Fr(0), Fr(0), Fr(0)), 'N1': x1, 'N2': x2}, 'sections': {'N': SEC_N},
            'members': [('M1', 'N0', 'N1', 'N'), ('M2', 'N1', 'N2', 'N')], 'supports': {'N0': {'rigid': ALL6}},
            'loads': {'N2': {'F': mv(Q9, (Fr(0), Fr(10), Fr(0))), 'M': mv(Q9, (Fr(3), Fr(0), Fr(0)))}}}
    add_case(id='RF-FINITE-ROT-Q9-NONMULT', family='RF-FINITE',
             purpose='L-frame (2 m along x then 2 m along z, N0 fixed, load (0,10,0) N and (3,0,0) N*m at the tip) '
                     'rotated by Q9 without multiple-of-9 coordinates: node coordinates and loads are ninths, so the '
                     'represented member lengths are irrational (evaluated in 110-digit decimal).',
             defn=defn, method=('tree', 'N0'), ncs=[nc_sign(), nc_subtract_rounded()])


# =====================================================================================
# RF-MECH

FAMILY_TEXT['RF-MECH'] = ('Genuine mechanisms that must be refused, in larger and skewed systems: translation-pinned nodes '
                          'along a skew line (rotation about the line free), a count trap (many rotation restraints none '
                          'of which intersect the null axis), a zero-stiffness spring, and disconnected sub-assemblies '
                          'inside large stable models.  Each null motion is stated and verified to be a zero-energy '
                          'admissible motion.  One stable companion (a rotation restraint that does intersect the axis) '
                          'must be solved, not refused.')


def rigid_mode(defn, nodes, t=Z3, w=Z3, about=None):
    about = about or defn['nodes'][nodes[0]]
    return {n: (vadd(t, cross(w, vsub(defn['nodes'][n], about))), w) for n in nodes}


def verify_null(defn, mode):
    """Zero element deformation for every member and zero motion in every restrained or sprung direction."""
    X = defn['nodes']

    def get(n):
        return mode.get(n, (Z3, Z3))
    for (mid, i, j, sec) in defn['members']:
        ui, ti = get(i)
        uj, tj = get(j)
        if tuple(tj) != tuple(ti) or vsub(uj, ui) != cross(ti, vsub(X[j], X[i])):
            return False
    for n, s in defn.get('supports', {}).items():
        u, t = get(n)
        for d in s.get('rigid', []):
            if dot(u if d[0] == 'U' else t, dof_axis(d)) != 0:
                return False
        for (kind, dv, k) in s.get('springs', []):
            if Fr(k) != 0 and dot(u if kind == 't' else t, dv) != 0:
                return False
    return True


def independent(modes, keys):
    rows = []
    for m in modes:
        row = []
        for n in keys:
            u, t = m.get(n, (Z3, Z3))
            row += [Fr(c) for c in u] + [Fr(c) for c in t]
        rows.append(row)
    rank = 0
    ncol = len(rows[0]) if rows else 0
    for c in range(ncol):
        piv = next((r for r in range(rank, len(rows)) if rows[r][c] != 0), None)
        if piv is None:
            continue
        rows[rank], rows[piv] = rows[piv], rows[rank]
        for r in range(len(rows)):
            if r != rank and rows[r][c] != 0:
                f = rows[r][c] / rows[rank][c]
                rows[r] = [a - f * b for a, b in zip(rows[r], rows[rank])]
        rank += 1
    return rank == len(modes)


def line_defn(n, step, origin=(Fr(0), Fr(0), Fr(0)), prefix='N', mprefix='M', extra_rigid=()):
    nodes = {'%s%d' % (prefix, i): vadd(origin, vs(i, tuple(Fr(c) for c in step))) for i in range(n + 1)}
    members = [('%s%d' % (mprefix, i), '%s%d' % (prefix, i - 1), '%s%d' % (prefix, i), 'N') for i in range(1, n + 1)]
    sups = {k: {'rigid': list(PIN3) + list(extra_rigid)} for k in nodes}
    return nodes, members, sups


def build_mech():
    e122 = (Fr(1), Fr(2), Fr(2))
    for lab, loads, text in (('TORQUE', {'N10': {'M': vs(Fr('0.001'), e122)}}, 'torque 0.001*(1,2,2) N*m at the tip (excites the null motion)'),
                             ('PERP', {'N5': {'M': (Fr('0.002'), Fr('0.001'), Fr('-0.002'))}}, 'moment 0.001*(2,1,-2) N*m at N5, orthogonal to the null motion (consistent but non-unique)'),
                             ('UNLOADED', {}, 'no load (u = 0 satisfies equilibrium with zero residual, yet the state is not unique)')):
        nodes, members, sups = line_defn(10, (1, 2, 2))
        defn = {'nodes': nodes, 'sections': {'N': SEC_N}, 'members': members, 'supports': sups, 'loads': loads}
        mode = {n: (Z3, e122) for n in nodes}
        add_case(id='RF-MECH-LINE122-%s' % lab, family='RF-MECH', mechanism=True, defn=defn, method=None,
                 purpose='10 members along (1,2,2) (integer coordinates), every node translation-pinned, no rotational '
                         'restraint; %s.' % text,
                 null_modes=[('rigid rotation of every node about the line through the nodes, axis (1,2,2)/3, zero '
                              'translation', mode)])
    nodes, members, sups = line_defn(10, (3, 4, 0), extra_rigid=('RZ',))
    defn = {'nodes': nodes, 'sections': {'N': SEC_N}, 'members': members, 'supports': sups,
            'loads': {'N10': {'M': (Fr('0.003'), Fr('0.004'), Fr(0))}}}
    add_case(id='RF-MECH-LINE345-RZ', family='RF-MECH', mechanism=True, defn=defn, method=None,
             purpose='10 members along (3,4,0), every node translation-pinned and restrained in global RZ (44 restrained '
                     'DOFs), tip torque 0.001*(3,4,0) N*m.  RZ is normal to the null axis, so no restraint stops it: a '
                     'restraint-count heuristic passes this mechanism.',
             null_modes=[('rigid rotation about the line, axis (3,4,0)/5 (no RZ component)', {n: (Z3, (Fr(3), Fr(4), Fr(0))) for n in nodes})])
    cdef = chain_defn(10, 'T', Fr(0), Fr('1e-8'))
    add_case(id='RF-MECH-K0', family='RF-MECH', mechanism=True, defn=cdef, method=None,
             purpose='RF-CHAIN torsion chain of 10 members with the root spring stiffness set to exactly 0 and tip torque '
                     '1e-8 N*m (contrast: k = 2.16e-6 in RF-CHAIN-T-n10-r1e-12 must be solved).  If a product rejects a '
                     'zero spring as invalid input instead, that is also a refusal.',
             null_modes=[('rigid rotation of the whole chain about the x axis', {n: (Z3, (Fr(1), Fr(0), Fr(0))) for n in cdef['nodes']})])
    for n_big, sub, lab in ((100, 'DISC', 'RF-MECH-DISC-CHAIN100'), (100, 'DISC-SPRING', 'RF-MECH-DISC-CHAIN100-SPRING'),
                            (1000, 'LINE', 'RF-MECH-LINE-IN-CHAIN1000')):
        base = lg_chain_defn(n_big)
        defn = _deep_copy(base)
        if sub.startswith('DISC'):
            dn = {'D0': (Fr(0), Fr(30), Fr(0)), 'D1': (Fr(3), Fr(30), Fr(0)), 'D2': (Fr(3), Fr(33), Fr(0)), 'D3': (Fr(3), Fr(33), Fr(3))}
            defn['nodes'].update(dn)
            defn['members'] += [('DM1', 'D0', 'D1', 'N'), ('DM2', 'D1', 'D2', 'N'), ('DM3', 'D2', 'D3', 'N')]
            keys = list(dn)
            modes = []
            for ax in ('X', 'Y', 'Z'):
                modes.append(('translation of the sub-assembly along %s' % ax, rigid_mode(defn, keys, t=AXES[ax])))
            for ax in ('X', 'Y', 'Z'):
                modes.append(('rotation of the sub-assembly about the %s axis through D0' % ax, rigid_mode(defn, keys, w=AXES[ax])))
            if sub == 'DISC-SPRING':
                defn['supports']['D0'] = {'springs': [('t', (1, 0, 0), K_ORD_TRA)]}
                modes = [m for m in modes if 'along X' not in m[0]]
                text = ('a separate 3-member sub-assembly D0-D3 held only by one translational spring (1e8 N/m along X '
                        'at D0): five rigid modes remain')
            else:
                text = 'a separate, unsupported and unloaded 3-member sub-assembly D0 (0,30,0) - D3 (3,33,3): six rigid modes'
        else:
            ln, lm, ls = line_defn(5, (1, 2, 2), origin=(Fr(0), Fr(100), Fr(0)), prefix='L', mprefix='LM')
            defn['nodes'].update(ln)
            defn['members'] += lm
            defn['supports'].update(ls)
            defn['loads']['L5'] = {'M': vs(Fr('0.001'), e122)}
            keys = list(ln)
            modes = [('rigid rotation of the line sub-assembly about its own axis (1,2,2)/3', {n: (Z3, e122) for n in keys})]
            text = ('a separate 5-member line along (1,2,2) starting at (0,100,0), every node translation-pinned, torque '
                    '0.001*(1,2,2) N*m at L5: rotation about that line is free')
        trap = None
        if sub.startswith('DISC'):
            trap = ('accepting because the global restraint count is at least six: only the global form of the count '
                    'is defeated here; a per-body count finds %s on the sub-assembly and refuses correctly' %
                    ('one restraint (the spring)' if sub == 'DISC-SPRING' else 'no restraint'))
        add_case(id=lab, family='RF-MECH', mechanism=True, defn=defn, method=None, count_trap=trap,
                 purpose='RF-LARGE chain of %d members (stable on its own, tip loads as RF-LARGE-CHAIN-n%05d-AX) plus %s.  The '
                         'global restraint count is ample; the model must still be refused.' % (n_big, n_big, text),
                 null_modes=modes)
    # stable companion: a restraint that does intersect the null axis
    defn = {'nodes': {'N0': (Fr(0), Fr(0), Fr(0)), 'N1': (Fr(3), Fr(4), Fr(0))}, 'sections': {'N': SEC_N},
            'members': [('M1', 'N0', 'N1', 'N')],
            'supports': {'N0': {'rigid': ['UX', 'UY', 'UZ', 'RX']}, 'N1': {'rigid': list(PIN3)}},
            'loads': {'N1': {'M': (Fr('0.003'), Fr('0.004'), Fr(0))}}}

    def refuse(ctx):
        return nc_result('NC-FALSE-MECHANISM', 'the stable companion refused as a mechanism (a count or DOF-class '
                         'heuristic instead of the geometric test)', ctx, outcome='refusal of a stable model')
    add_case(id='RF-MECH-LINE345-RX-COMPANION', family='RF-MECH', defn=defn, method=('pinoff', 'N0', 'N1'),
             purpose='stable companion: one member (0,0,0)->(3,4,0), both nodes translation-pinned, root restrained in '
                     'global RX (cosine 3/5 to the axis), tip torque 0.001*(3,4,0) N*m.  RX intersects the null axis, so '
                     'this model must be solved (closed form: the rigid-restraint limit of RF-SKEW-T-PIN-OFF).',
             ncs=[refuse, nc_sign()])


def mech_record(case):
    defn = case['defn']
    modes = case['null_modes']
    ok = all(verify_null(defn, m) for _, m in modes)
    keys = sorted({n for _, m in modes for n in m})
    indep = independent([m for _, m in modes], keys)
    if not (ok and indep):
        raise AssertionError('%s: null motions fail verification' % case['id'])
    works = []
    for desc, m in modes:
        wk = Fr(0)
        for n, ld in defn.get('loads', {}).items():
            u, t = m.get(n, (Z3, Z3))
            wk += dot(tuple(Fr(c) for c in ld.get('F', (0, 0, 0))), u) + dot(tuple(Fr(c) for c in ld.get('M', (0, 0, 0))), t)
        works.append(fmt(wk, SIG_AUX))
    return {'family': 'RF-MECH', 'purpose': case['purpose'], 'model': model_json(defn),
            'flags': {'needs_directional_spring': needs_directional(defn)},
            'expected_outcome': ('refuse: mechanism.  The stiffness on the unrestrained DOFs is singular with nullity %d; '
                                 'no displacement, reaction or member value may be published as a solution.  A correct '
                                 'diagnostic identifies the listed null motion(s).' % len(modes)),
            'nullity': len(modes),
            'null_motions': [{'description': desc,
                              'nodes': {n: {'u': [fmt_input(c) for c in m[n][0]], 'theta': [fmt_input(c) for c in m[n][1]]}
                                        for n in keys if n in m},
                              'unlisted_nodes': 'zero motion'} for desc, m in modes],
            'load_work_on_null_motions': works,
            'verification': ('each null motion is a rigid motion of every member (theta_j = theta_i, u_j - u_i = theta_i x '
                             '(x_j - x_i)), satisfies every rigid restraint and every nonzero spring, and the motions are '
                             'linearly independent (checked exactly by this script).  Completeness of the nullity is checked '
                             'by the author self-check (exact elimination) for every RF-MECH case.'),
            'negative_controls': [
                {'id': 'NC-REGULARIZED', 'description': 'a solver that adds a small diagonal penalty, drops the zero pivot '
                 'or falls back to a least-squares or pseudo-inverse solve publishes finite values: any published '
                 'solution fails this reference', 'discriminates': True},
                {'id': 'NC-ZERO-RESIDUAL', 'description': 'accepting because the residual of some returned state is zero '
                 '(true for u = 0 when the load does no work on the null motion)', 'discriminates': True},
                {'id': 'NC-RESTRAINT-COUNT', 'description': case.get('count_trap') or ('accepting because the restraint '
                 'count is at least six, counted per connected body or globally: both forms of the count are defeated '
                 'here'), 'discriminates': True}],
            'reference_accuracy': 'exact rational null motions; nullity exact'}


# =====================================================================================
# RF-CANCEL (addendum R1_ADDENDUM_CANCEL.md)

FAMILY_TEXT['RF-CANCEL'] = ('Cancelling load contributions.  A cantilever tip DOF (force UY or moment RZ) carries three '
                            'authored contributions (G, n, -G) whose exact net is n = 0.3 (G = 1e5..1e8) or 1e-8 '
                            '(G = 1e80), in the orders (G, n, -G), (G, -G, n) and (n, G, -G); the same with an ordinary '
                            'load on an orthogonal DOF or in the same plane; and two uniformly loaded spans whose fixed-end '
                            'moments cancel at a shared node next to a small nodal moment.  Exact responses come from the '
                            'exact net load.  Every value carries a recommended (net-governed) scale and the gross '
                            'alternative; negative controls are the left-to-right binary64 sums in each order and the '
                            'response with the small contribution dropped.')
CANCEL_G = [('1e5', '100000', '0.3'), ('1e6', '1000000', '0.3'), ('1e7', '10000000', '0.3'),
            ('1e8', '100000000', '0.3'), ('1e80', '1e80', '1e-8')]
CANCEL_ORDERS = {'GnG': ('G', 'n', '-G'), 'GGn': ('G', '-G', 'n'), 'nGG': ('n', 'G', '-G')}


def float_sum(values):
    s = float(values[0])
    for v in values[1:]:
        s = s + float(v)
    return Fr(s)


def cancel_defn(kind, contributions, ordinary=None):
    nodes = {'N0': (Fr(0), Fr(0), Fr(0)), 'N1': (Fr(1), Fr(0), Fr(0)), 'N2': (Fr(2), Fr(0), Fr(0))}
    loads = {}
    if ordinary == 'ORTHO':
        loads = {'N2': {'F': (Fr(0), Fr(0), Fr(100))}} if kind == 'F' else {'N2': {'M': (Fr(0), Fr(50), Fr(0))}}
    elif ordinary == 'INPLANE':
        loads = {'N1': {'F': (Fr(0), Fr(50), Fr(0))}} if kind == 'F' else {'N1': {'M': (Fr(0), Fr(0), Fr(20))}}
    return {'nodes': nodes, 'sections': {'N': SEC_N}, 'members': [('M1', 'N0', 'N1', 'N'), ('M2', 'N1', 'N2', 'N')],
            'supports': {'N0': {'rigid': ALL6}}, 'loads': loads, 'load_contributions': contributions}


def build_cancel():
    ORD_TEXT = {None: '', 'ORTHO': '; ordinary load %s', 'INPLANE': '; ordinary load %s'}
    variants = [(o, None) for o in CANCEL_ORDERS] + [('GnG', 'ORTHO'), ('GnG', 'INPLANE')]
    for kind in 'FM':
        dof = 'UY' if kind == 'F' else 'RZ'
        unit = 'N' if kind == 'F' else 'N*m'
        for (glab, G, n) in CANCEL_G:
            G, n = Fr(G), Fr(n)
            vals = {'G': G, 'n': n, '-G': -G}
            for (olab, ordinary) in variants:
                if ordinary and glab not in ('1e8', '1e80'):
                    continue
                contrib = [('N2', dof, vals[o]) for o in CANCEL_ORDERS[olab]]
                defn = cancel_defn(kind, contrib, ordinary)
                ord_desc = ''
                if ordinary:
                    ld = defn['loads']
                    node = list(ld)[0]
                    kk = list(ld[node])[0]
                    ord_desc = '; ordinary load %s = (%s) %s at %s (%s)' % (
                        kk, ', '.join(fmt_input(c) for c in ld[node][kk]), unit, node,
                        'orthogonal DOF: the cancelled DOF response stays a pure net response' if ordinary == 'ORTHO'
                        else 'same bending plane: every response mixes the net and the ordinary load')
                net_only = cancel_defn(kind, [('N2', dof, n)])
                gross_only = cancel_defn(kind, [('N2', dof, G)])
                ncs = []
                for o2, seq in CANCEL_ORDERS.items():
                    fvals = [vals[o] for o in seq]
                    nf = float_sum(fvals)
                    ncs.append(('NC-FLOAT-SUM-%s' % o2,
                                'net obtained by summing the binary64 contributions left to right in the order (%s)%s: '
                                'net = %s instead of %s' % (', '.join(seq), ' (the authored order)' if o2 == olab else '',
                                                            fmt(nf, 17) if nf else '0', fmt_input(n)),
                                cancel_defn(kind, [('N2', dof, nf)], ordinary), nf))
                ncs.append(('NC-SMALL-DROPPED', 'the small contribution n dropped (net 0)',
                            cancel_defn(kind, [('N2', dof, G), ('N2', dof, -G)], ordinary), Fr(0)))
                add_case(id='RF-CANCEL-%s-G%s-%s%s' % (kind, glab, olab, ('-' + ordinary) if ordinary else ''),
                         family='RF-CANCEL',
                         purpose='cantilever N0 (fixed) -> N1 (1,0,0) -> N2 (2,0,0), N section; tip %s carries the '
                                 'contributions (%s) %s in this authored order, exact net %s %s%s.' % (
                                     dof, ', '.join(fmt_input(c[2]) for c in contrib), unit, fmt_input(n), unit, ord_desc),
                         defn=defn, method=('tree', 'N0'),
                         cancel={'net_only': net_only, 'gross_only': gross_only, 'ncs': ncs, 'gross': G, 'net': n,
                                 'gross_text': 'the gross contribution G = %s %s applied alone at %s' % (fmt_input(G), unit, dof),
                                 'net_text': 'the net contribution n = %s %s applied alone at %s' % (fmt_input(n), unit, dof)})
    L = Fr(2)
    for (wlab, W, d, m) in (('W1e5', Fr(100000), Fr('0.875'), Fr('0.2')), ('W1e8', Fr(10 ** 8), Fr('0.875'), Fr('0.2')),
                            ('W1e80', Fr('1e80'), Fr(0), Fr('1e-8'))):
        def udl(wA, wB, mz):
            return {'nodes': {'S0': (Fr(0), Fr(0), Fr(0)), 'S1': (L, Fr(0), Fr(0)), 'S2': (2 * L, Fr(0), Fr(0))},
                    'sections': {'N': SEC_N}, 'members': [('A', 'S0', 'S1', 'N'), ('B', 'S1', 'S2', 'N')],
                    'supports': {'S0': {'rigid': ALL6}, 'S1': {'rigid': list(PIN3)}, 'S2': {'rigid': ALL6}},
                    'loads': {'S1': {'M': (Fr(0), Fr(0), mz)}} if mz else {},
                    'member_loads': {k: (Fr(0), w, Fr(0)) for k, w in (('A', wA), ('B', wB)) if w}}
        defn = udl(W, W + d, m)
        net = m + d * L * L / 12
        ncs = []
        wAf, wBf, mf, Lf = float(W), float(W + d), float(m), float(L)
        cA = -(wAf * Lf * Lf / 12.0)
        cB = wBf * Lf * Lf / 12.0
        for lab, seq, text in (('A-node-B', (cA, mf, cB), 'span A, nodal moment, span B'),
                               ('A-B-node', (cA, cB, mf), 'span A, span B, nodal moment')):
            s_ = seq[0]
            for v in seq[1:]:
                s_ = s_ + v
            nf = Fr(s_)
            mz = nf - d * L * L / 12
            ncs.append(('NC-FLOAT-SUM-%s' % lab, 'net moment at S1 assembled in binary64 from the equivalent end moments '
                        '-w_A L^2/12, +w_B L^2/12 (each evaluated as w*L*L/12.0) and the nodal moment, summed in the order '
                        '(%s): net = %s instead of %s' % (text, fmt(nf, 17) if nf else '0', fmt(net, 17)),
                        udl(W, W + d, mz), nf))
        ncs.append(('NC-SMALL-DROPPED', 'the small nodal moment dropped (net = (w_B - w_A) L^2/12)', udl(W, W + d, Fr(0)),
                    d * L * L / 12))
        add_case(id='RF-CANCEL-UDL-%s' % wlab, family='RF-CANCEL',
                 purpose='two 2 m spans S0 (fixed) - S1 (translation-pinned) - S2 (fixed) along x, N section; uniform '
                         'loads w_A = %s N/m and w_B = %s N/m, both along +y, so their fixed-end moments enter S1 with '
                         'opposite signs (-w_A L^2/12 and +w_B L^2/12); nodal moment %s N*m about z at S1; exact net '
                         'moment at S1 = %s N*m.  Reactions and end moments are governed by the gross loads, the S1 '
                         'rotation by the net moment.' % (fmt_input(W), fmt_input(W + d), fmt_input(m), fmt_input(net)),
                 defn=defn, method=('udl2',),
                 cancel={'net_only': udl(Fr(0), Fr(0), net), 'gross_only': udl(W, Fr(0), Fr(0)), 'ncs': ncs,
                         'gross': W * L * L / 12, 'net': net,
                         'gross_text': 'span A load w_A alone (its fixed-end moment w_A L^2/12 = %s N*m)' % fmt_input(W * L * L / 12),
                         'net_text': 'the net moment %s N*m applied alone at S1' % fmt_input(net)})


# =====================================================================================
# processing and output

def check_exact(flat):
    for (k, v, c) in flat:
        x = v[1] if isinstance(v, tuple) else v
        if not isinstance(x, (Fr, int)) or isinstance(x, bool):
            raise TypeError('non-exact value at %s: %r' % (k, type(x)))


def equilibrium_residual(sol, defn):
    """Global force and moment balance (about the origin) of loads and support actions; exact zero expected."""
    X = defn['nodes']
    F, M = Z3, Z3
    loads = [(n, tuple(Fr(c) for c in ld.get('F', (0, 0, 0))), tuple(Fr(c) for c in ld.get('M', (0, 0, 0))))
             for n, ld in defn.get('loads', {}).items()]
    for (n, dof, val) in defn.get('load_contributions', []):
        v = [Fr(0)] * 3
        v[DOF_INDEX_[dof]] = Fr(val)
        loads.append((n, tuple(v), Z3) if dof[0] == 'U' else (n, Z3, tuple(v)))
    for (mid, i, j, sec) in defn['members']:
        if mid in defn.get('member_loads', {}):
            xi, xj = tuple(Fr(c) for c in X[i]), tuple(Fr(c) for c in X[j])
            L = qsqrt(dot(vsub(xj, xi), vsub(xj, xi)))
            loads.append((None, vs(L, tuple(Fr(c) for c in defn['member_loads'][mid])), Z3, vs(Fr(1, 2), vadd(xi, xj))))
    for item in loads:
        n, f, mo = item[0], item[1], item[2]
        x = item[3] if len(item) > 3 else tuple(Fr(c) for c in X[n])
        F = vadd(F, f)
        M = vadd(M, mo, cross(x, f))
    for n, sup in defn.get('supports', {}).items():
        x = tuple(Fr(c) for c in X[n])
        for d in sup.get('rigid', []):
            v = vs(sol['reac'][n][d], dof_axis(d))
            if d[0] == 'U':
                F = vadd(F, v)
                M = vadd(M, cross(x, v))
            else:
                M = vadd(M, v)
        for i, (kind, dv, k) in enumerate(sup.get('springs', [])):
            v = sol['springs'][n][i]
            if kind == 't':
                F = vadd(F, v)
                M = vadd(M, cross(x, v))
            else:
                M = vadd(M, v)
    return F, M


def model_json(defn, full=False):
    if not full and len(defn['members']) > 1000:
        return {'generator': 'this case has %d nodes and %d members; python3 references.py --model <case id> prints the '
                             'complete model in the same form as the other cases' % (len(defn['nodes']), len(defn['members'])),
                'sections': {k: {kk: fmt_input(vv) for kk, vv in s.items()} for k, s in defn['sections'].items()},
                'supports': {n: {'rigid': list(s.get('rigid', []))} for n, s in defn.get('supports', {}).items()},
                'load_count': len(defn.get('loads', {}))}
    out = {'nodes_m': {k: [fmt_input(c) for c in v] for k, v in defn['nodes'].items()},
           'sections': {k: {kk: fmt_input(vv) for kk, vv in s.items()} for k, s in defn['sections'].items()},
           'members': [list(m) for m in defn['members']],
           'supports': {}, 'loads': {}}
    for n, s in defn.get('supports', {}).items():
        out['supports'][n] = {'rigid': list(s.get('rigid', [])),
                              'springs': [{'kind': 'translation' if kind == 't' else 'rotation',
                                           'direction': [fmt_input(c) for c in d], 'k': fmt_input(k)}
                                          for (kind, d, k) in s.get('springs', [])]}
    for n, ld in defn.get('loads', {}).items():
        out['loads'][n] = {kk: [fmt_input(c) for c in vv] for kk, vv in ld.items()}
    if defn.get('load_contributions'):
        out['load_contributions_in_authored_order'] = [[n, d, fmt_input(v)] for (n, d, v) in defn['load_contributions']]
    if defn.get('member_loads'):
        out['member_uniform_loads_N_per_m_global'] = {m: [fmt_input(c) for c in q] for m, q in defn['member_loads'].items()}
    return out


REFERENCE_ACCURACY = ('exact rational evaluation of the stated closed form with pi replaced by PI_Q (|PI_Q - pi| < 1e-189); '
                      'each published nonzero value is the exact rational rounded to 40 significant digits (relative '
                      'error < 5e-40); bending magnitudes are square roots of exact rationals evaluated at 70 digits; '
                      'zeros are exact.')


def process(case):
    defn, method = case['defn'], case['method']
    sample = case.get('sample', (None, None))
    regions = case.get('regions')
    sol = case['sol'] if 'sol' in case else solve(defn, method)
    flat_all = flatten(sol, defn, regions=regions)
    check_exact(flat_all)
    F, M = equilibrium_residual(sol, defn)
    if F != (0, 0, 0) or M != (0, 0, 0):
        raise AssertionError('%s: global equilibrium residual %s %s' % (case['id'], F, M))
    for chk in case.get('checks', []):
        chk(sol)
    flat = flatten(sol, defn, *sample, regions=regions) if sample != (None, None) else flat_all
    model = Model(defn, Exact())
    scales = compute_scales(sol, defn, model, regions)
    exp_map = {k: (v, c) for (k, v, c) in flat}
    ctx = Ctx(case, sol, flat, exp_map, scales, model)
    ncs = [f(ctx) for f in case['ncs']]
    zeros = sum(1 for (k, v, c) in flat if (v[1] if isinstance(v, tuple) else v) == 0)
    below = sum(1 for (k, v, c) in flat if (v[1] if isinstance(v, tuple) else v) != 0 and absval(v) < scales[c][0])
    rec = {'family': case['family'], 'purpose': case['purpose'], 'method': method_text(method),
           'units': case.get('units', {'length': 'm', 'force': 'N', 'moment': 'N*m', 'rotation': 'rad', 'modulus': 'Pa'}),
           'flags': dict(case['flags'], needs_directional_spring=needs_directional(defn)),
           'model': model_json(defn)}
    if case.get('regions'):
        rec['regions'] = case['regions']
    if sample != (None, None):
        rec['published_subset'] = {'nodes': [n for n in defn['nodes'] if n in sample[0]],
                                   'members': [m[0] for m in defn['members'] if m[0] in sample[1]],
                                   'note': 'every other node/member value follows from the same closed form; '
                                           'python3 references.py --full %s prints all of them' % case['id']}
    rec['reference_accuracy'] = case.get('accuracy', REFERENCE_ACCURACY)
    rec['scales'] = {c: {'value': fmt(v, SIG_AUX), 'derivation': how} for c, (v, how) in sorted(scales.items())}
    rec['expected'] = [[k, fmt(v), c] for (k, v, c) in flat]
    rec['counts'] = {'values': len(flat), 'zeros': zeros, 'nonzero_below_class_scale': below}
    rec['negative_controls'] = ncs
    rec['basis'] = 'intended'
    if case['finite']:
        fin, flat_rep = finite_compare(case, sol, flat, exp_map, scales)
        rec['finite_input'] = fin
        if fin['exceeds_1e-9']:
            rec['basis'] = 'represented'
            rec['finite_input']['note'] = ('binary64 rounding of the inputs moves at least one value by more than the 1e-9 '
                                           'criterion: this case cannot discriminate at 1e-9 on intended inputs.  The '
                                           'represented-input values below are the reference basis (as NP-A).')
            rec['expected_represented'] = [[k, fmt(v), c] for (k, v, c) in flat_rep]
    else:
        rec['finite_input'] = {'computed': False, 'reason': case.get('finite_reason', '')}
    if case.get('cancel'):
        cancel_augment(case, rec, flat, scales)
    if 'range_info' in case:
        env = range_envelope(defn, flat)
        sc_ok = all(MIN_NORMAL <= Fr(v) <= MAX_FINITE for (v, how) in scales.values())
        env['all_scales_normal'] = sc_ok
        rec['range'] = dict(case['range_info'], **env)
        if not (env['all_inputs_normal'] and env['all_published_values_normal'] and sc_ok):
            raise AssertionError('%s: a published input, value or scale is outside the binary64 normal range' % case['id'])
    if case['notes']:
        rec['notes'] = case['notes']
    return rec


FAMILY_BUILDER = {'RF-CHAIN': 'build_chain', 'RF-SKEW': 'build_skew', 'RF-WEAK': 'build_weak', 'RF-LARGE': 'build_large',
                  'RF-INVARIANCE': 'build_invariance', 'RF-RANGE': 'build_range', 'RF-ZERO': 'build_zero',
                  'RF-FINITE': 'build_finite', 'RF-MECH': 'build_mech', 'RF-CANCEL': 'build_cancel'}


def compare_scaled(values, exp, smap, top=12):
    viol = []
    with localcontext() as c:
        c.prec = 60
        for k, v in values.items():
            e = exp[k]
            r = abs(to_dec(v) - to_dec(e)) / (CRIT_D * max(absval(e), smap[k]))
            if r > 1:
                viol.append((r, k, v))
    viol.sort(key=lambda t: (-t[0], t[1]))
    return viol


def cancel_augment(case, rec, flat, scales):
    c = case['cancel']
    fn = {k: v for (k, v, cl) in flatten(solve(c['net_only'], case['method']), c['net_only'])}
    fg = {k: v for (k, v, cl) in flatten(solve(c['gross_only'], case['method']), c['gross_only'])}
    exp = {k: v for (k, v, cl) in flat}
    s_rec, s_gro, s_cls, rows = {}, {}, {}, []
    counts = {}
    for (k, v, cl) in flat:
        sc = scales[cl][0]
        qn, qg = fn.get(k, Fr(0)), fg.get(k, Fr(0))
        an, ag = absval(qn), absval(qg)
        s_rec[k] = an if an != 0 else sc
        s_gro[k] = ag if ag != 0 else sc
        s_cls[k] = sc
        z = (v[1] if isinstance(v, tuple) else v) == 0
        if z:
            gov = 'zero'
        elif v == qn:
            gov = 'net'
        elif an == 0:
            gov = 'other loads only'
        else:
            gov = 'mixed'
        counts[gov] = counts.get(gov, 0) + 1
        rows.append([k, fmt(v), cl, fmt(s_rec[k], SIG_AUX), fmt(s_gro[k], SIG_AUX), gov])
    rec['expected'] = rows
    rec['expected_columns'] = ['key', 'expected', 'class', 'recommended_scale (net-governed)',
                               'gross_scale (alternative)', 'governed_by']
    ncs = []
    for (nid, desc, defn2, net_used) in c['ncs']:
        vals = {k: v for (k, v, cl) in flatten(solve(defn2, case['method']), defn2) if k in exp}
        vr = compare_scaled(vals, exp, s_rec)
        vc = compare_scaled(vals, exp, s_cls)
        vg = compare_scaled(vals, exp, s_gro)
        ncs.append({'id': nid, 'description': desc, 'net_used': fmt(net_used, SIG_AUX),
                    'discriminates': bool(vr), 'violating_values': len(vr),
                    'max_violation_ratio': fmt(vr[0][0], 6) if vr else '0', 'at': vr[0][1] if vr else None,
                    'discriminates_under_class_scale': bool(vc), 'discriminates_under_gross_scale': bool(vg),
                    'values': {k: fmt(v, SIG_AUX) for (_, k, v) in vr[:12]}})
    rec['negative_controls'] = ncs
    rec['cancellation'] = {
        'exact_net': fmt(c['net'], SIG_AUX), 'gross': fmt(c['gross'], SIG_AUX),
        'recommended_scale': ('BINDING scale for this case (ROOT ruling on V2, ROOT_RULINGS_V2.md item 1).  '
                              'Net-governed: the magnitude of the value\'s response to %s; for a value that the net '
                              'contribution does not affect, or an exact zero, the case class scale.  It never exceeds '
                              'the class scale.  It can fall below |exp| (in mixed rows); there the comparison '
                              '|obs - exp| <= 1e-9 max(|exp|, scale) is exactly relative.  The effective comparison '
                              'scale max(|exp|, scale) is never below |exp|.' % c['net_text']),
        'gross_scale': ('alternative, not recommended: the magnitude of the value\'s response to %s (or the class '
                        'scale where that is zero).  It would accept a lost or mis-summed net contribution.' % c['gross_text']),
        'class_scale': 'the family-wide class rule (published under scales); shown for comparison in the negative controls',
        'governed_by_counts': counts}


def main(argv):
    if len(argv) > 2 and argv[1] in ('--full', '--model'):
        fam = next(f for f in FAMILY_BUILDER if argv[2].startswith(f + '-'))
        globals()[FAMILY_BUILDER[fam]]()
    else:
        for b in BUILD_ORDER:
            b()
    if len(argv) > 2 and argv[1] == '--model':
        case = next(c for c in CASES if c['id'] == argv[2])
        print(json.dumps(model_json(case['defn'], full=True), indent=1))
        return
    if len(argv) > 2 and argv[1] == '--full':
        case = next(c for c in CASES if c['id'] == argv[2])
        sol = case['sol'] if 'sol' in case else solve(case['defn'], case['method'])
        for (k, v, c) in flatten(sol, case['defn'], regions=case.get('regions')):
            print(json.dumps([k, fmt(v), c]))
        return
    only = argv[1:] if len(argv) > 1 else None
    out_cases = {}
    for case in CASES:
        if only and not any(case['id'].startswith(p) for p in only):
            continue
        if case['family'] == 'RF-MECH' and case.get('mechanism'):
            out_cases[case['id']] = mech_record(case)
        else:
            out_cases[case['id']] = process(case)
        r = out_cases[case['id']]
        if 'counts' in r:
            print('%-46s %6d values  basis=%s  negative controls discriminating %d/%d' % (
                case['id'], r['counts']['values'], r['basis'],
                sum(1 for nc in r['negative_controls'] if nc.get('discriminates')), len(r['negative_controls'])), flush=True)
        else:
            print('%-46s refuse: mechanism, nullity %d' % (case['id'], r['nullity']), flush=True)
    return out_cases


BUILD_ORDER = [build_chain, build_skew, build_weak, build_large, build_invariance, build_range, build_zero, build_finite,
               build_mech, build_cancel]


def header():
    sec = section_props(Exact(Fr(1)), SEC_N['E'], SEC_N['G'], SEC_N['OD'], SEC_N['ID'])  # pi = 1: coefficients of pi
    assert sec['EA'] == 380000000 and sec['EI'] == 1719500 and sec['GJ'] == 1375600
    return {
        'schema': 't3-independent-references-v1',
        'status': 'candidate, revision 2 (narrow revision after V2: findings F1, F3, F4, F5 and F9); frozen only '
                  'when ROOT selects it after the V2 backcheck',
        'criterion': '|observed - expected| <= 1e-9 * max(|expected|, scale), scale = the published class scale of the '
                     'value, except in RF-CANCEL, where the binding scale is the recommended (net-governed) column of '
                     'each value row (ROOT ruling on V2, ROOT_RULINGS_V2.md item 1); unchanged form, no tolerance is '
                     'proposed here',
        'theory': 'small-displacement linear-elastic Euler-Bernoulli space frame, no shear deformation, circular annular '
                  'sections (Iy = Iz = I, J = 2I), rigid restraints on global DOFs, grounded linear springs (global or '
                  'stated direction), nodal forces and moments only',
        'arithmetic': 'exact rationals with pi replaced by the exact rational of a %d-digit Machin value; decimal '
                      'strings rounded to %d significant digits (auxiliary values %d)' % (PI_DIGITS, SIG_EXPECTED, SIG_AUX),
        'pi_used': format(PI_DEC, '.60e') + ' (first 61 digits shown)',
        'section_N': {'E_Pa': '2e11', 'G_Pa': '8e10', 'OD_m': '0.2', 'ID_m': '0.18', 'A': '0.0019*pi m^2',
                      'I': '0.0000085975*pi m^4', 'J': '0.000017195*pi m^4', 'EA': '380000000*pi N',
                      'EI': '1719500*pi N*m^2', 'GJ': '1375600*pi N*m^2'},
        'conventions': {
            'frame': 'right-handed global X, Y, Z; DOFs UX, UY, UZ, RX, RY, RZ per node',
            'u, th': 'global nodal translations (m) and rotations (rad)',
            'R.<node>.<DOF>': 'action of a rigid restraint on the structure (N or N*m); sum of loads and support actions is zero',
            'S.<node>.<i>.F*|M*': 'global components of spring i action on the structure, -k (d.u) d/|d|^2',
            'N.<m>': 'axial force, tension positive = EA * ext / L',
            'T.<m>': 'torque = GJ * tw / L',
            'Mb.<m>.i|mid|j': 'bending-moment magnitude hypot(My, Mz) at the authored end i, the midpoint and end j',
            'tw.<m>': '(theta_j - theta_i) . e_ij, rad',
            'ext.<m>': '(u_j - u_i) . e_ij, m (N, T, tw and ext do not depend on the end order)',
            'classes': 'translation, rotation, force, moment, twist, extension; a suffix @region marks a region-specific '
                       'class (RF-WEAK).  Every published value is compared with the scale of its class.',
            'finite_input': 'max over published values of |represented - intended| / max(|intended|, scale); basis = '
                            'represented where that exceeds 1e-9',
            'model inputs': 'decimal strings are exact intended values; p/q is an exact rational; d*2^k is d times a power '
                            'of two; spring directions are exact vectors (normalize in the product)'},
        'families': FAMILY_TEXT,
    }


def summarize(cases):
    fam = {}
    nvals = 0
    for cid, r in cases.items():
        f = fam.setdefault(r['family'], {'cases': 0, 'values': 0, 'refusal_cases': 0, 'negative_controls': 0,
                                         'discriminating_negative_controls': 0, 'represented_basis_cases': []})
        f['cases'] += 1
        if 'counts' in r:
            f['values'] += r['counts']['values']
            nvals += r['counts']['values']
        else:
            f['refusal_cases'] += 1
        f['negative_controls'] += len(r['negative_controls'])
        f['discriminating_negative_controls'] += sum(1 for nc in r['negative_controls'] if nc.get('discriminates'))
        if r.get('basis') == 'represented':
            f['represented_basis_cases'].append(cid)
    return {'cases': len(cases), 'published_expected_values': nvals, 'by_family': fam}


if __name__ == '__main__':
    result = main(sys.argv)
    if result is not None and len(sys.argv) == 1:
        out = header()
        out['summary'] = summarize(result)
        out['cases'] = result
        text = json.dumps(out, indent=1, ensure_ascii=True) + '\n'
        Path(__file__).with_name('references.json').write_text(text)
        print(json.dumps(out['summary'], indent=1))
