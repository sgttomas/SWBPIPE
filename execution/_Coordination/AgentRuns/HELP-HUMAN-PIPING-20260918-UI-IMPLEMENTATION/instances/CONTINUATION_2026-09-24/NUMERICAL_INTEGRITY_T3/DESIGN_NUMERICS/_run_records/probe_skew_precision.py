#!/usr/bin/env python3
"""T3 D1 design probe (standard library only; no product code is imported or read).

Question: for a skewed 3D Euler-Bernoulli frame with an N05-class soft restraint,
does (a) an exact solve of the represented binary64 global element contributions
(what a generalized exact-block method would compute), (b) promoting the rounded
binary64 matrix to 128 bits, or (c) rebuilding each element from its binary64
primitives in basic-deformation form at emulated p-bit precision, meet the
unchanged relative criterion |obs-exp| <= 1e-9*max(|exp|, scale)?

It also exercises a proposed adaptive stop rule: accept precision p when every
published quantity agrees between p and 2p within 2^-64 of max(|q_2p|, S_class),
where S_class is the largest magnitude of the same kind (translation, rotation,
force, moment) in the same connected body at 2p, coupled to its dimensional partner
kind through the body extent (translation<->rotation, force<->moment).

Every case and value is invented. The class scale used as the zero scale here is a
probe convenience for design evidence, not a frozen reference; R1 owns references.
Arithmetic emulation: every operation of method (c) is rounded to p significand
bits, ties-to-even, unbounded exponent (Fraction based), as in the precision
refutation probe. This isolates significand precision; it is not a Rust or MPFR run.
"""
from fractions import Fraction as Fr
import json
import math
import sys

G_SCREEN = Fr(1, 2 ** 64)
CRITERION = Fr(1, 10 ** 9)


# ---------------------------------------------------------------- arithmetic
def rnd(x, p):
    """Round Fraction x to p significand bits, ties to even; p=None is exact."""
    if p is None or x == 0:
        return Fr(x)
    x = Fr(x)
    sign = -1 if x < 0 else 1
    a = abs(x)
    n, d = a.numerator, a.denominator
    e = n.bit_length() - d.bit_length()
    if (n << max(0, -e)) < (d << max(0, e)):
        e -= 1
    shift = p - 1 - e  # scale so that the integer part has p bits
    if shift >= 0:
        q, r = divmod(n << shift, d)
    else:
        q, r = divmod(n, d << (-shift))
    twice = 2 * r
    if twice > d or (twice == d and (q & 1)):
        q += 1
    return sign * (Fr(q) / (Fr(2) ** shift) if shift >= 0 else Fr(q) * (Fr(2) ** (-shift)))


def rsqrt(x, p):
    """Correctly rounded square root of a nonnegative Fraction at p bits (p=None exact only when x is a perfect square)."""
    x = Fr(x)
    if x == 0:
        return Fr(0)
    if p is None:
        n, d = x.numerator, x.denominator
        rn, rd = math.isqrt(n), math.isqrt(d)
        if rn * rn != n or rd * rd != d:
            raise ValueError("inexact exact sqrt")
        return Fr(rn, rd)
    n, d = x.numerator, x.denominator
    # choose k so that sqrt(x)*2^k has about p+3 integer bits
    k = (p + 4) - (n.bit_length() - d.bit_length()) // 2
    num = n * 4 ** k if k >= 0 else n
    den = d if k >= 0 else d * 4 ** (-k)
    t = num // den
    s = math.isqrt(t)
    exact = (s * s == t) and (t * den == num)
    # s = floor(sqrt(x)*2^k); add a sticky half-unit when inexact
    val = Fr(2 * s + (0 if exact else 1), 2) / (Fr(2) ** k)
    return rnd(val, p)


class A:
    """Arithmetic context: every result rounded to p bits (None = exact)."""

    def __init__(self, p):
        self.p = p
        self.ops = 0

    def add(self, a, b):
        self.ops += 1
        return rnd(a + b, self.p)

    def sub(self, a, b):
        self.ops += 1
        return rnd(a - b, self.p)

    def mul(self, a, b):
        self.ops += 1
        return rnd(a * b, self.p)

    def div(self, a, b):
        self.ops += 1
        return rnd(Fr(a) / Fr(b), self.p)

    def sqrt(self, a):
        self.ops += 1
        if self.p is None:
            # Exact when the argument is a rational square (all frame axes in these
            # cases); otherwise (bending magnitudes only) correctly rounded at 400 bits.
            try:
                return rsqrt(a, None)
            except ValueError:
                self.inexact_sqrt = getattr(self, "inexact_sqrt", 0) + 1
                return rsqrt(a, 400)
        return rsqrt(a, self.p)

    def dot(self, xs, ys):
        s = Fr(0)
        for x, y in zip(xs, ys):
            if x == 0 or y == 0:
                continue
            s = self.add(s, self.mul(x, y))
        return s


def f64(x):
    return float(Fr(x))


def F(x):
    return Fr(float(x))  # exact decoding of a binary64 value


# ---------------------------------------------------------------- model inputs
PI = math.pi  # binary64 pi: the represented input basis, not the decimal pi
E, Gm = 200e9, 80e9
OD, ID = 0.2, 0.18
AREA = PI * (OD * OD - ID * ID) / 4.0
IY = PI * (OD ** 4 - ID ** 4) / 64.0
IZ = IY
JT = 2.0 * IY
SECTION = dict(E=F(E), G=F(Gm), A=F(AREA), Iy=F(IY), Iz=F(IZ), J=F(JT))


def cross(a, b, ar):
    return [ar.sub(ar.mul(a[1], b[2]), ar.mul(a[2], b[1])),
            ar.sub(ar.mul(a[2], b[0]), ar.mul(a[0], b[2])),
            ar.sub(ar.mul(a[0], b[1]), ar.mul(a[1], b[0]))]


def normalize(v, ar):
    n2 = ar.add(ar.add(ar.mul(v[0], v[0]), ar.mul(v[1], v[1])), ar.mul(v[2], v[2]))
    n = ar.sqrt(n2)
    return [ar.div(c, n) for c in v], n


def element_frame(xi, xj, yref, ar):
    d = [ar.sub(F(xj[k]), F(xi[k])) for k in range(3)]
    ex, L = normalize(d, ar)
    proj = ar.dot([F(c) for c in yref], ex)
    yc = [ar.sub(F(yref[k]), ar.mul(proj, ex[k])) for k in range(3)]
    ey, _ = normalize(yc, ar)
    ez, _ = normalize(cross(ex, ey, ar), ar)
    return [ex, ey, ez], L


def transform12(R):
    T = [[Fr(0)] * 12 for _ in range(12)]
    for blk in range(4):
        for r in range(3):
            for c in range(3):
                T[3 * blk + r][3 * blk + c] = R[r][c]
    return T


def basic_B_local(L, ar):
    """6x12 basic deformation map in local axes: axial, twist, (thz_i, thz_j) relative
    to chord in x-y, (thy_i, thy_j) relative to chord in x-z."""
    B = [[Fr(0)] * 12 for _ in range(6)]
    invL = ar.div(Fr(1), L)
    B[0][0], B[0][6] = Fr(-1), Fr(1)
    B[1][3], B[1][9] = Fr(-1), Fr(1)
    # chord rotation in x-y: (v_j - v_i)/L ; e = th_z - psi_z
    for row, th in ((2, 5), (3, 11)):
        B[row][th] = Fr(1)
        B[row][1] = invL
        B[row][7] = -invL
    # chord rotation in x-z: psi_y = -(w_j - w_i)/L ; e = th_y - psi_y
    for row, th in ((4, 4), (5, 10)):
        B[row][th] = Fr(1)
        B[row][2] = -invL
        B[row][8] = invL
    return B


def basic_D(sec, L, ar):
    ea = ar.div(ar.mul(sec["E"], sec["A"]), L)
    gj = ar.div(ar.mul(sec["G"], sec["J"]), L)
    eiz = ar.div(ar.mul(sec["E"], sec["Iz"]), L)
    eiy = ar.div(ar.mul(sec["E"], sec["Iy"]), L)
    D = [[Fr(0)] * 6 for _ in range(6)]
    D[0][0], D[1][1] = ea, gj
    for (a, b, ei) in ((2, 3, eiz), (4, 5, eiy)):
        D[a][a] = ar.mul(Fr(4), ei)
        D[b][b] = ar.mul(Fr(4), ei)
        D[a][b] = ar.mul(Fr(2), ei)
        D[b][a] = ar.mul(Fr(2), ei)
    return D


def matmul(Am, Bm, ar):
    n, m, q = len(Am), len(Bm), len(Bm[0])
    return [[ar.dot(Am[i], [Bm[k][j] for k in range(m)]) for j in range(q)] for i in range(n)]


def transpose(M):
    return [list(r) for r in zip(*M)]


# ---------------------------------------------------------------- model
def build_case(members, springs, restrained, loads, nnodes):
    return dict(members=members, springs=springs, restrained=restrained, loads=loads, n=6 * nnodes)


# f64 product-like formation: local K, f64 frame from f64 normalize, two-stage T^T(KT)
def f64_local_K(L):
    e, g, a, iy, iz, j = E, Gm, AREA, IY, IZ, JT
    L2, L3 = L * L, L * L * L
    K = [[0.0] * 12 for _ in range(12)]

    def sym(r, c, v):
        K[r][c] = v
        K[c][r] = v
    ax, tor = e * a / L, g * j / L
    K[0][0] = K[6][6] = ax
    sym(0, 6, -ax)
    K[3][3] = K[9][9] = tor
    sym(3, 9, -tor)
    for (v, th, ei, s) in ((1, 5, iz, 1.0), (2, 4, iy, -1.0)):
        k12, k6, k4, k2 = 12 * e * ei / L3, 6 * e * ei / L2, 4 * e * ei / L, 2 * e * ei / L
        K[v][v] = K[v + 6][v + 6] = k12
        sym(v, v + 6, -k12)
        sym(v, th, s * k6)
        sym(v, th + 6, s * k6)
        sym(v + 6, th, -s * k6)
        sym(v + 6, th + 6, -s * k6)
        K[th][th] = K[th + 6][th + 6] = k4
        sym(th, th + 6, k2)
    return K


def f64_frame(xi, xj, yref):
    d = [xj[k] - xi[k] for k in range(3)]
    n = math.sqrt(d[0] * d[0] + d[1] * d[1] + d[2] * d[2])
    ex = [c / n for c in d]
    pr = yref[0] * ex[0] + yref[1] * ex[1] + yref[2] * ex[2]
    yc = [yref[k] - pr * ex[k] for k in range(3)]
    m = math.sqrt(sum(c * c for c in yc))
    ey = [c / m for c in yc]
    z = [ex[1] * ey[2] - ex[2] * ey[1], ex[2] * ey[0] - ex[0] * ey[2], ex[0] * ey[1] - ex[1] * ey[0]]
    q = math.sqrt(sum(c * c for c in z))
    ez = [c / q for c in z]
    return [ex, ey, ez], n


def f64_global_K(K, R):
    T = [[0.0] * 12 for _ in range(12)]
    for blk in range(4):
        for r in range(3):
            for c in range(3):
                T[3 * blk + r][3 * blk + c] = R[r][c]
    KT = [[sum(K[i][k] * T[k][j] for k in range(12)) for j in range(12)] for i in range(12)]
    return [[sum(T[k][i] * KT[k][j] for k in range(12)) for j in range(12)] for i in range(12)], T


def dofmap(i, j):
    return [6 * i + a for a in range(6)] + [6 * j + a for a in range(6)]


# ---------------------------------------------------------------- solvers
PIVOT_EVIDENCE = {}


def gamma_p(m, p):
    u = Fr(1, 2 ** p)
    return m * u / (1 - m * u)


def ldl_solve(K, f, ar):
    n = len(f)
    L = [[Fr(0)] * n for _ in range(n)]
    D = [Fr(0)] * n
    worst = None
    for i in range(n):
        for j in range(i):
            s = K[i][j]
            for k in range(j):
                if L[i][k] != 0 and L[j][k] != 0:
                    s = ar.sub(s, ar.mul(ar.mul(L[i][k], D[k]), L[j][k]))
            L[i][j] = ar.div(s, D[j]) if s != 0 else Fr(0)
        s = K[i][i]
        c = abs(K[i][i])
        for k in range(i):
            if L[i][k] != 0:
                t = ar.mul(ar.mul(L[i][k], D[k]), L[i][k])
                s = ar.sub(s, t)
                c += abs(t)
        if ar.p is not None and c != 0:
            # M03-style local cancellation screen at the working precision p:
            # pass requires d_i > 64*gamma_p(2*s_i+2)*c_i (margin > 1).
            margin = (s / c) / (64 * gamma_p(2 * i + 2, ar.p))
            worst = margin if worst is None else min(worst, margin)
        if s <= 0:
            PIVOT_EVIDENCE["last"] = None if worst is None else float(worst)
            return None, "nonpositive pivot"
        D[i] = s
        L[i][i] = Fr(1)
    PIVOT_EVIDENCE["last"] = None if worst is None else float(worst)
    y = list(f)
    for i in range(n):
        for k in range(i):
            if L[i][k] != 0:
                y[i] = ar.sub(y[i], ar.mul(L[i][k], y[k]))
    x = [ar.div(y[i], D[i]) for i in range(n)]
    for i in reversed(range(n)):
        for k in range(i + 1, n):
            if L[k][i] != 0:
                x[i] = ar.sub(x[i], ar.mul(L[k][i], x[k]))
    return x, None


def f64_ldl_solve(K, f):
    n = len(f)
    L = [[0.0] * n for _ in range(n)]
    D = [0.0] * n
    for i in range(n):
        for j in range(i):
            s = K[i][j] - sum(L[i][k] * D[k] * L[j][k] for k in range(j))
            L[i][j] = s / D[j]
        s = K[i][i] - sum(L[i][k] * D[k] * L[i][k] for k in range(i))
        if not s > 0:
            return None, "nonpositive pivot"
        D[i] = s
    y = list(f)
    for i in range(n):
        y[i] -= sum(L[i][k] * y[k] for k in range(i))
    x = [y[i] / D[i] for i in range(n)]
    for i in reversed(range(n)):
        x[i] -= sum(L[k][i] * x[k] for k in range(i + 1, n))
    return x, None


# ---------------------------------------------------------------- methods
def published(case, u, element_F, reactions):
    """Convention-free published quantities: key -> (group, kind, value).
    Kinds pair dimensionally through a length: translation<->rotation and
    force<->moment (see scales())."""
    out = {}
    for node in range(case["n"] // 6):
        for a in range(6):
            out[f"u:{node}:{a}"] = ("body", "trans" if a < 3 else "rot", u[6 * node + a])
    for idx, Fl in enumerate(element_F):
        g = "body"  # one connected body in every probe case
        out[f"N:{idx}"] = (g, "force", Fl["N"])
        out[f"T:{idx}"] = (g, "moment", Fl["T"])
        out[f"Mi:{idx}"] = (g, "moment", Fl["Mi"])
        out[f"Mj:{idx}"] = (g, "moment", Fl["Mj"])
    for dof, v in reactions.items():
        out[f"R:{dof}"] = ("body", "force" if dof % 6 < 3 else "moment", v)
    return out


PARTNER = {"trans": "rot", "rot": "trans", "force": "moment", "moment": "force"}


def lengths(case):
    """Characteristic length of the (single) connected body: its node extent."""
    pts = [m[2] for m in case["members"]] + [m[3] for m in case["members"]]
    o = pts[0]
    ext = max(math.sqrt(sum((p[k] - o[k]) ** 2 for k in range(3))) for p in pts)
    return {"body": Fr(ext)}


def scales(pub, case):
    """Per-quantity scale: max(|group,kind max|, partner max coupled by the group length)."""
    L = lengths(case)
    gk = {}
    for key, (g, kind, v) in pub.items():
        gk[(g, kind)] = max(gk.get((g, kind), Fr(0)), abs(Fr(v)))
    out = {}
    for key, (g, kind, v) in pub.items():
        partner = gk.get((g, PARTNER[kind]), Fr(0))
        coupled = partner * L[g] if kind in ("trans", "moment") else partner / L[g]
        out[key] = max(gk[(g, kind)], coupled)
    return out


def method_primitive(case, p):
    """(c) rebuild from binary64 primitives in basic-deformation form at p bits."""
    ar = A(p)
    n = case["n"]
    K = [[Fr(0)] * n for _ in range(n)]
    elems = []
    for (i, j, xi, xj, yref) in case["members"]:
        R, L = element_frame(xi, xj, yref, ar)
        T = transform12(R)
        Bl = basic_B_local(L, ar)
        Bg = matmul(Bl, T, ar)
        D = basic_D(SECTION, L, ar)
        DB = matmul(D, Bg, ar)
        Ke = matmul(transpose(Bg), DB, ar)
        m = dofmap(i, j)
        for a in range(12):
            for b in range(12):
                if Ke[a][b] != 0:
                    K[m[a]][m[b]] = ar.add(K[m[a]][m[b]], Ke[a][b])
        elems.append((m, T, Bl, D))
    for dof, k in case["springs"]:
        K[dof][dof] = ar.add(K[dof][dof], F(k))
    f = [Fr(0)] * n
    for dof, v in case["loads"]:
        f[dof] = ar.add(f[dof], F(v))
    free = [d for d in range(n) if d not in case["restrained"]]
    Kf = [[K[a][b] for b in free] for a in free]
    ff = [f[a] for a in free]
    x, err = ldl_solve(Kf, ff, ar)
    if err:
        return None, err, ar.ops
    u = [Fr(0)] * n
    for a, v in zip(free, x):
        u[a] = v
    element_F = []
    for (m, T, Bl, D) in elems:
        ue = [u[d] for d in m]
        dl = [ar.dot(T[r], ue) for r in range(12)]
        e = [ar.dot(Bl[r], dl) for r in range(6)]
        Q = [ar.dot(D[r], e) for r in range(6)]
        mi = ar.sqrt(ar.add(ar.mul(Q[2], Q[2]), ar.mul(Q[4], Q[4])))
        mj = ar.sqrt(ar.add(ar.mul(Q[3], Q[3]), ar.mul(Q[5], Q[5])))
        element_F.append(dict(N=Q[0], T=Q[1], Mi=mi, Mj=mj))
    reactions = {}
    for dof, k in case["springs"]:
        reactions[dof] = ar.mul(-F(k), u[dof])
    for dof in case["restrained"]:
        reactions[dof] = ar.sub(ar.dot(K[dof], u), f[dof])
    return published(case, u, element_F, reactions), None, ar.ops


def represented_f64(case):
    n = case["n"]
    K = [[0.0] * n for _ in range(n)]
    elems = []
    for (i, j, xi, xj, yref) in case["members"]:
        R, L = f64_frame(xi, xj, yref)
        Kl = f64_local_K(L)
        Kg, T = f64_global_K(Kl, R)
        m = dofmap(i, j)
        for a in range(12):
            for b in range(12):
                K[m[a]][m[b]] += Kg[a][b]
        elems.append((m, T, Kl))
    for dof, k in case["springs"]:
        K[dof][dof] += k
    f = [0.0] * n
    for dof, v in case["loads"]:
        f[dof] += v
    return K, f, elems


def recover_represented(case, K, f, elems, u, exact):
    conv = (lambda x: Fr(x)) if exact else (lambda x: x)
    element_F = []
    for (m, T, Kl) in elems:
        ue = [u[d] for d in m]
        dl = [sum(conv(T[r][c]) * ue[c] for c in range(12)) for r in range(12)]
        Fl = [sum(conv(Kl[r][c]) * dl[c] for c in range(12)) for r in range(12)]
        if exact:
            hyp = lambda a, b: Fr(math.hypot(float(a), float(b)))  # rounded once for publication
        else:
            hyp = math.hypot
        element_F.append(dict(N=Fl[6], T=Fl[9], Mi=hyp(Fl[5], Fl[4]), Mj=hyp(Fl[11], Fl[10])))
    reactions = {}
    for dof, k in case["springs"]:
        reactions[dof] = -conv(k) * u[dof]
    for dof in case["restrained"]:
        reactions[dof] = sum(conv(K[dof][c]) * u[c] for c in range(case["n"])) - conv(f[dof])
    return published(case, u, element_F, reactions)


def method_represented_exact(case):
    """(a) exact rational solve of the represented binary64 contributions."""
    K, f, elems = represented_f64(case)
    n = case["n"]
    free = [d for d in range(n) if d not in case["restrained"]]
    Kf = [[Fr(K[a][b]) for b in free] for a in free]
    ff = [Fr(f[a]) for a in free]
    x, err = ldl_solve(Kf, ff, A(None))
    if err:
        return None, err
    u = [Fr(0)] * n
    for a, v in zip(free, x):
        u[a] = v
    return recover_represented(case, K, f, elems, u, True), None


def method_promoted(case, p):
    """(b) promote the rounded binary64 global matrix to p bits, then solve at p."""
    K, f, elems = represented_f64(case)
    n = case["n"]
    free = [d for d in range(n) if d not in case["restrained"]]
    ar = A(p)
    x, err = ldl_solve([[Fr(K[a][b]) for b in free] for a in free], [Fr(f[a]) for a in free], ar)
    if err:
        return None, err
    u = [Fr(0)] * n
    for a, v in zip(free, x):
        u[a] = v
    return recover_represented(case, K, f, elems, u, True), None


def method_binary64(case):
    """(d) ordinary binary64 LDL on the represented matrix, binary64 recovery."""
    K, f, elems = represented_f64(case)
    n = case["n"]
    free = [d for d in range(n) if d not in case["restrained"]]
    x, err = f64_ldl_solve([[K[a][b] for b in free] for a in free], [f[a] for a in free])
    if err:
        return None, err
    u = [0.0] * n
    for a, v in zip(free, x):
        u[a] = v
    return recover_represented(case, K, f, elems, u, False), None


def method_exact_reference(case):
    """Reference: exact rational arithmetic from exact coordinates and binary64-decoded section/load inputs."""
    return method_primitive(case, None)


# ---------------------------------------------------------------- comparisons
def compare(pub, ref, case):
    sc = scales(ref, case)
    worst, worst_key, fails = Fr(0), None, 0
    for key, (g, kind, e) in ref.items():
        o = pub[key][2]
        e_pub = Fr(float(Fr(e)))  # the reference is compared at its binary64 publication
        o_pub = Fr(float(Fr(o)))
        scale = max(abs(e_pub), sc[key])
        ratio = (Fr(0) if o_pub == e_pub else Fr(10 ** 30)) if scale == 0 else abs(o_pub - e_pub) / scale
        if ratio > CRITERION:
            fails += 1
        if ratio > worst:
            worst, worst_key = ratio, key
    return dict(max_scaled_error=float(worst), at=worst_key, quantities=len(ref), failing=fails)


def agreement(pub_p, pub_2p, case):
    sc = scales(pub_2p, case)
    worst, worst_key = Fr(0), None
    for key, (g, kind, v2) in pub_2p.items():
        v1 = pub_p[key][2]
        denom = max(abs(Fr(v2)), sc[key])
        r = (Fr(0) if v1 == v2 else Fr(10 ** 30)) if denom == 0 else abs(Fr(v1) - Fr(v2)) / denom
        if r > worst:
            worst, worst_key = r, key
    return dict(max_scaled_disagreement=float(worst), at=worst_key,
                accepted_under_2_pow_minus_64=worst <= G_SCREEN)


# ---------------------------------------------------------------- cases
def pin_case(direction, k, moment, members=1, yref=(0.0, 0.0, 1.0)):
    """Node 0 translations fixed; node 0 rotations held only by three global
    rotational springs k; a straight run of `members` equal members along
    `direction` (integer coordinates, so every input coordinate is exact);
    a moment vector at the tip. Positive-definite for any k > 0."""
    nodes = [tuple(float(c * s) for c in direction) for s in range(members + 1)]
    mem = [(s, s + 1, nodes[s], nodes[s + 1], yref) for s in range(members)]
    springs = [(3, k), (4, k), (5, k)]
    restrained = [0, 1, 2]
    tip = 6 * members
    loads = [(tip + 3, moment[0]), (tip + 4, moment[1]), (tip + 5, moment[2])]
    return build_case(mem, springs, restrained, loads, members + 1)


def run(name, case, precisions, include_exact_represented=True):
    rec = {"case": name, "dofs": case["n"], "members": len(case["members"])}
    ref, err, _ = method_exact_reference(case)
    assert err is None, err
    if include_exact_represented:
        pub, err = method_represented_exact(case)
        rec["represented_binary64_exact_solve"] = err or compare(pub, ref, case)
        pub, err = method_promoted(case, 128)
        rec["promoted_rounded_matrix_p128"] = err or compare(pub, ref, case)
    pub, err = method_binary64(case)
    rec["ordinary_binary64"] = err or compare(pub, ref, case)
    prim = {}
    for p in precisions:
        pub, err, ops = method_primitive(case, p)
        prim[p] = pub
        rec[f"primitive_basic_p{p}"] = (err or compare(pub, ref, case))
        if not err:
            rec[f"primitive_basic_p{p}"]["emulated_operations"] = ops
            rec[f"primitive_basic_p{p}"]["min_pivot_screen_margin"] = PIVOT_EVIDENCE.get("last")
    for p in precisions:
        if 2 * p in prim and prim[p] is not None and prim[2 * p] is not None:
            rec[f"agreement_p{p}_vs_p{2 * p}"] = agreement(prim[p], prim[2 * p], case)
    return rec


def main():
    results = {"python": sys.version.split()[0], "criterion": "1e-9*max(|exp|,coupled class scale)",
               "stop_rule_probe": "2^-64*max(|q_2p|,coupled class scale)", "cases": []}
    skew = (3, 4, 0)
    axis = (5, 0, 0)
    oblique = (2, 3, 6)  # frame (2,3,6),(6,2,-3),(-3,6,-2) over 7: exact rational axes
    mom = lambda k: (1e-8 * k / 1e-4, 2e-8 * k / 1e-4, 0.0)  # root rotations ~1e-4 rad
    results["cases"].append(run("AXIS k=1e-4 (N05-class, axis-aligned control)", pin_case(axis, 1e-4, mom(1e-4)), [128, 256]))
    results["cases"].append(run("SKEW(3,4,0) k=100 (ordinary companion)", pin_case(skew, 100.0, mom(100.0)), [128, 256]))
    results["cases"].append(run("SKEW(3,4,0) k=1e-4 (N05-class)", pin_case(skew, 1e-4, mom(1e-4)), [128, 256]))
    results["cases"].append(run("SKEW(3,4,0) k=1e-12 (N06-class)", pin_case(skew, 1e-12, mom(1e-12)), [128, 256]))
    results["cases"].append(run("OBLIQUE(2,3,6) y_ref=(6,2,-3) k=1e-4", pin_case(oblique, 1e-4, mom(1e-4), yref=(6.0, 2.0, -3.0)), [128, 256]))
    results["cases"].append(run("SKEW(3,4,0) k=1e-28 (extreme arithmetic stress)", pin_case(skew, 1e-28, mom(1e-28)), [128, 256, 512]))
    results["cases"].append(run("SKEW(3,4,0) 6-member run k=1e-12 (order 39 block)", pin_case(skew, 1e-12, mom(1e-12), members=6), [128, 256], include_exact_represented=True))
    mech = {"case": "SKEW(3,4,0) k=0 (genuine mechanism: root rotations unrestrained)"}
    for p in (128, 256, 512):
        pub, err, ops = method_primitive(pin_case(skew, 0.0, (1e-8, 2e-8, 0.0)), p)
        mech[f"primitive_basic_p{p}"] = {"outcome": err or "factor completed",
                                          "min_pivot_screen_margin": PIVOT_EVIDENCE.get("last")}
    results["mechanism_control"] = mech
    print(json.dumps(results, indent=1))


if __name__ == "__main__":
    main()
