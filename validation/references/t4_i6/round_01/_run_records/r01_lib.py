"""T4-I6 repair round 01: shared library (standard library only).

Three parts, each labelled where it is used:
  1. EXACT  - references. The B1 element of round 00 (curved_ref.py, imported unchanged from
              the round-00 copy in rerun00/) at high decimal precision; model assembly and exact
              solves; the kd5_tests.rs actual-error measure; equilibrated condition numbers.
  2. BINP   - a binary p-bit arithmetic (round to nearest, ties to even) on Fractions, written
              for this round (no code from RV131's rv_wide.py), and K-D5's curved re-formation
              of the B1 element at p in formation_check.rs's operation order with B1's (R, y)
              inputs, in two spellings of 1 - cos phi ('closed': 1 - fl(1 - 2 s^2); 'stable':
              2 s^2 rounded once).
  3. EMU64  - labelled binary64 emulations (NOT product code): FK's straight frame in FK's
              operation order; plausible T4-U1 curved elements (correctly rounded F, a stable
              closed form, a naive closed form, 16-point Gauss-Legendre); FK's preparation
              (radix scaling) and its two factorizations (dense Cholesky; LDL^T in a given order);
              K-D5's rule (rho exact, w from the exact intended reduced matrix, factor 2).
"""
import math
import os
import sys
from decimal import Decimal as D, getcontext, localcontext
from fractions import Fraction as Fr

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, "rerun00"))
import curved_ref as C  # noqa: E402  (round-00 library, bytes unchanged)

F122_SECTION = dict(E=200000000000.0, G=80000000000.0, A=0.005969026041820614,
                    I=2.700984283923829e-05, J=5.401968567847658e-05)


# =============================================================================== 1. EXACT
def dec(v):
    return C.dec(v)


def exact_element(xi, xj, R, y, sec=F122_SECTION, kin=1.0, kout=1.0, chord_local=None):
    """Global 12x12 of the B1 element (Decimal), round-00 closed form; chord_local overrides H."""
    return C.curved_element(xi, xj, R, y, sec["E"], sec["G"], sec["A"], sec["I"], sec["J"], kin, kout,
                            chord=chord_local)


def assemble(n_nodes, elems, springs, add_springs="exact"):
    """elems: [(i, j, K12)]. Springs added exactly (Decimal) or in binary64 (add_springs='b64',
    only meaningful when the element entries are binary64 floats)."""
    n = 6 * n_nodes
    K = [[D(0)] * n for _ in range(n)]
    for (i, j, Ke) in elems:
        dof = [6 * i + r for r in range(6)] + [6 * j + r for r in range(6)]
        for a in range(12):
            for b in range(12):
                K[dof[a]][dof[b]] += dec(Ke[a][b])
    for dd, v in springs:
        if add_springs == "b64":
            K[dd][dd] = dec(float(K[dd][dd]) + v)
        else:
            K[dd][dd] += dec(v)
    return K


def reduce(K, free):
    return [[K[a][b] for b in free] for a in free]


def solve_exact(K, f, free):
    Kf = reduce(K, free)
    u = C.solve(Kf, [dec(f.get(a, 0.0)) for a in free])
    return dict(zip(free, u))


def extent_diag(nodes):
    ext = []
    for k in range(3):
        vals = [dec(p[k]) for p in nodes]
        ext.append(max(vals) - min(vals))
    return C.norm(ext)


def actual_ratio(nodes, uref, u):
    """kd5_tests.rs actual_ratio: max over free rows |u - u_ref| / (1e-9 max(|u_ref|, S*_kind(u_ref)))."""
    st = max((abs(v) for d_, v in uref.items() if d_ % 6 < 3), default=D(0))
    sr = max((abs(v) for d_, v in uref.items() if d_ % 6 >= 3), default=D(0))
    lb = extent_diag(nodes)
    tr, ro = max(st, lb * sr), max(sr, st / lb)
    worst, at = D(0), None
    for d_, v in uref.items():
        sc = max(abs(v), tr if d_ % 6 < 3 else ro)
        r = abs(dec(u[d_]) - v) / (D("1e-9") * sc)
        if r > worst:
            worst, at = r, d_
    return worst, at


def cond1(A):
    n = len(A)
    Ai = C.inverse(A)
    n1 = max(sum(abs(A[i][j]) for i in range(n)) for j in range(n))
    return n1 * max(sum(abs(Ai[i][j]) for i in range(n)) for j in range(n))


def cond1_sqrt_equilibrated(Kf):
    s = [abs(Kf[i][i]).sqrt() for i in range(len(Kf))]
    return cond1([[Kf[i][j] / (s[i] * s[j]) for j in range(len(Kf))] for i in range(len(Kf))])


def binexp(x):
    """binary exponent of a nonzero binary64 value (FK's binary_exponent for normal values)."""
    m, e = math.frexp(abs(x))
    return e - 1


def radix_exponents(Kf_b64_diag):
    """FK prepare: exponent_r = -floor(binary_exponent(K_rr) / 2)."""
    return [-(binexp(d) // 2) for d in Kf_b64_diag]


def cond1_radix(Kf):
    """1-norm condition of the matrix FK's gate reads: K scaled by 2^(e_r + e_c)."""
    e = radix_exponents([float(Kf[i][i]) for i in range(len(Kf))])
    return cond1([[Kf[i][j] * D(2) ** (e[i] + e[j]) for j in range(len(Kf))] for i in range(len(Kf))])


def sig20(v):
    return C.sci(dec(v), 20)


def round_matrix(M):
    return [[float(v) for v in row] for row in M]


# =============================================================================== 2. BINP
class BinP:
    """Binary p-bit floating point, round to nearest, ties to even (unbounded exponent)."""

    def __init__(self, p):
        self.p = p
        self.u = Fr(1, 2 ** p)  # unit roundoff 2^-p

    def rnd(self, x):
        x = Fr(x)
        if x == 0:
            return Fr(0)
        neg = x < 0
        a = -x if neg else x
        num, den = a.numerator, a.denominator
        # e with 2^e <= a < 2^(e+1)
        e = num.bit_length() - den.bit_length()
        if (num << max(0, -e)) < (den << max(0, e)):
            e -= 1
        k = self.p - 1 - e  # scale so the integer part has p bits
        if k >= 0:
            q, r = divmod(num << k, den)
            dd = den
        else:
            q, r = divmod(num, den << (-k))
            dd = den << (-k)
        if 2 * r > dd or (2 * r == dd and (q & 1)):
            q += 1
        v = Fr(q) / (Fr(2) ** k) if k >= 0 else Fr(q) * (Fr(2) ** (-k))
        return -v if neg else v

    def add(self, a, b):
        return self.rnd(Fr(a) + Fr(b))

    def sub(self, a, b):
        return self.rnd(Fr(a) - Fr(b))

    def mul(self, a, b):
        return self.rnd(Fr(a) * Fr(b))

    def div(self, a, b):
        return self.rnd(Fr(a) / Fr(b))

    def sqrt(self, a):
        """Correctly rounded square root (integer square root with a sticky remainder)."""
        a = Fr(a)
        if a == 0:
            return Fr(0)
        num, den = a.numerator, a.denominator
        # want floor(sqrt(a) * 2^K) with >= p + 2 significant bits, then round with sticky
        K = self.p + 4 - (num.bit_length() - den.bit_length()) // 2
        if K >= 0:
            N = (num << (2 * K)) // den
            exact = (num << (2 * K)) % den == 0
        else:
            N = num // (den << (-2 * K))
            exact = num % (den << (-2 * K)) == 0
        s = math.isqrt(N)
        sticky = not (exact and s * s == N)
        # s/2^K <= sqrt(a) < (s+1)/2^K; add half an lsb of the extra precision when inexact
        v = Fr(2 * s + (1 if sticky else 0), 2) / (Fr(2) ** K) if K >= 0 else Fr(2 * s + (1 if sticky else 0), 2) * (Fr(2) ** (-K))
        return self.rnd(v)

    def dot3(self, a, b):
        p0, p1, p2 = self.mul(a[0], b[0]), self.mul(a[1], b[1]), self.mul(a[2], b[2])
        return self.add(self.add(p0, p1), p2)

    def cross3(self, a, b):
        def c(i, j):
            return self.sub(self.mul(a[i], b[j]), self.mul(a[j], b[i]))
        return [c(1, 2), c(2, 0), c(0, 1)]

    def atan2_q1(self, y, x):
        """atan2 for y, x >= 0, evaluated in Decimal at 90 digits and rounded once."""
        with localcontext() as cx:
            cx.prec = 90
            yd = D(y.numerator) / D(y.denominator)
            xd = D(x.numerator) / D(x.denominator)
            v = C.atan2_q1(yd, xd)
        return self.rnd(Fr(v))


def fr_dec(x):
    return D(x.numerator) / D(x.denominator)


def kd5_b1_at_p(ar, xi, xj, R, y, sec=F122_SECTION, kin=1.0, kout=1.0, variant="stable",
                chord_mode="actual", chord_override=None):
    """K-D5's curved re-formation with B1's inputs (x_i, x_j, R, y) at precision ar.p.
    Operation order: formation_check.rs curved_matrix steps 2-6, with the geometry of B1
    (s = L/2R, c = sqrt((2R-L)(2R+L))/2R, phi = 2 atan2(s, c), axes from d and y).
    variant 'closed': cos = fl(1 - 2 s^2), one_minus_cos = fl(1 - cos) (the cancelling form);
            'stable': one_minus_cos = 2 fl(s^2) (exact doubling), cos = fl(1 - one_minus_cos).
    chord_mode 'actual': H from A_p . d_p (the definition); 'formula_p': (R(cos - 1), R sin, 0) at p
            (M31b0, cos - 1 formed as fl(cos - 1)); 'override': the supplied local chord (Fractions).
    Returns (K 12x12 Fractions, info dict)."""
    r = ar
    xi = [Fr(v) for v in xi]
    xj = [Fr(v) for v in xj]
    y = [Fr(v) for v in y]
    R = Fr(R)
    d = [r.sub(xj[k], xi[k]) for k in range(3)]
    L = r.sqrt(r.dot3(d, d))
    twoR = 2 * R
    s = r.div(L, twoR)
    c = r.div(r.sqrt(r.mul(r.sub(twoR, L), r.add(twoR, L))), twoR)
    phi = 2 * r.atan2_q1(s, c)
    ss2 = 2 * r.mul(s, s)
    if variant == "closed":
        cos = r.sub(1, ss2)
        omc = r.sub(1, cos)
    elif variant == "stable":
        omc = ss2
        cos = r.sub(1, omc)
    else:
        raise ValueError(variant)
    sin = 2 * r.mul(s, c)
    sin2 = 2 * r.mul(sin, cos)
    half_ss = r.mul(sin, sin) / 2
    half_phi = phi / 2
    q_sin2 = sin2 / 4
    gram = [[phi, sin, omc],
            [sin, r.add(half_phi, q_sin2), half_ss],
            [omc, half_ss, r.sub(half_phi, q_sin2)]]
    rs, rc = r.mul(R, sin), r.mul(R, cos)
    z = Fr(0)
    z3 = [z, z, z]
    one = Fr(1)
    # actions per load: [M_ip, M_op, T, N] coefficient triples on (1, cos t, sin t)
    cases = [
        [[-rs, z, R], z3, z3, [z, z, -one]],
        [[rc, -R, z], z3, z3, [z, one, z]],
        [z3, [z, rs, -rc], [R, -rc, -rs], z3],
        [z3, [z, one, z], [z, z, -one], z3],
        [z3, [z, z, one], [z, one, z], z3],
        [[one, z, z], z3, z3, z3],
    ]

    def quad(lft, rgt):
        acc = Fr(0)
        for i in range(3):
            if lft[i] == 0:
                continue
            for j in range(3):
                if rgt[j] == 0 or gram[i][j] == 0:
                    continue
                acc = r.add(acc, r.mul(r.mul(lft[i], gram[i][j]), rgt[j]))
        return acc

    E, G, A, I, J = (Fr(sec[k]) for k in ("E", "G", "A", "I", "J"))
    fin, fout = Fr(kin), Fr(kout)
    bending, torsion, axial = r.mul(E, I), r.mul(G, J), r.mul(E, A)
    F = [[Fr(0)] * 6 for _ in range(6)]
    for a in range(6):
        for b in range(a, 6):
            t_in = r.div(r.mul(fin, quad(cases[a][0], cases[b][0])), bending)
            t_out = r.div(r.mul(fout, quad(cases[a][1], cases[b][1])), bending)
            t_t = r.div(quad(cases[a][2], cases[b][2]), torsion)
            t_a = r.div(quad(cases[a][3], cases[b][3]), axial)
            v = r.mul(R, r.add(r.add(r.add(t_in, t_out), t_t), t_a))
            F[a][b] = F[b][a] = v
    tip = _invert6_p(r, F)
    dh = [r.div(d[k], L) for k in range(3)]
    proj = r.dot3(y, dh)
    yc = [r.sub(y[k], r.mul(proj, dh[k])) for k in range(3)]
    ym = r.sqrt(r.dot3(yc, yc))
    n = [r.div(yc[k], ym) for k in range(3)]
    ex = [r.add(-r.mul(s, dh[k]), r.mul(c, n[k])) for k in range(3)]
    ey = [r.add(r.mul(c, dh[k]), r.mul(s, n[k])) for k in range(3)]
    ez = r.cross3(n, dh)
    axes = [ex, ey, ez]
    if chord_mode == "actual":
        chord = [r.dot3(axes[k], d) for k in range(3)]
    elif chord_mode == "formula_p":
        chord = [r.mul(R, r.sub(cos, 1)), r.mul(R, sin), Fr(0)]
    elif chord_mode == "override":
        chord = [Fr(v) for v in chord_override]
    else:
        raise ValueError(chord_mode)
    K = _assemble_from_tip_p(r, tip, chord, axes)
    return K, dict(d=d, L=L, s=s, c=c, phi=phi, cos=cos, sin=sin, omc=omc, axes=axes, chord=chord,
                   F=F, tip=tip, n=n, ym=ym)


def _invert6_p(r, f):
    m = [row[:] for row in f]
    inv = [[Fr(int(i == j)) for j in range(6)] for i in range(6)]
    for col in range(6):
        piv = max(range(col, 6), key=lambda q: abs(m[q][col]))
        m[col], m[piv] = m[piv], m[col]
        inv[col], inv[piv] = inv[piv], inv[col]
        dd = m[col][col]
        m[col] = [r.div(v, dd) for v in m[col]]
        inv[col] = [r.div(v, dd) for v in inv[col]]
        for q in range(6):
            if q == col or m[q][col] == 0:
                continue
            fac = m[q][col]
            m[q] = [r.sub(m[q][cc], r.mul(fac, m[col][cc])) for cc in range(6)]
            inv[q] = [r.sub(inv[q][cc], r.mul(fac, inv[col][cc])) for cc in range(6)]
    out = [row[:] for row in inv]
    for i in range(6):
        for j in range(i + 1, 6):
            v = r.add(inv[i][j], inv[j][i]) / 2
            out[i][j] = out[j][i] = v
    return out


def _mul6_p(r, x, yv, transpose_right):
    out = [[Fr(0)] * 6 for _ in range(6)]
    for i in range(6):
        for j in range(6):
            acc = Fr(0)
            for k in range(6):
                b = yv[j][k] if transpose_right else yv[k][j]
                if x[i][k] == 0 or b == 0:
                    continue
                acc = r.add(acc, r.mul(x[i][k], b))
            out[i][j] = acc
    return out


def _assemble_from_tip_p(r, tip, chord, axes):
    h = [[Fr(int(i == j)) for j in range(6)] for i in range(6)]
    h[3][1], h[3][2], h[4][0], h[4][2], h[5][0], h[5][1] = -chord[2], chord[1], chord[2], -chord[0], -chord[1], chord[0]
    coupled = _mul6_p(r, h, tip, False)
    anchored = _mul6_p(r, coupled, h, True)
    k = [[Fr(0)] * 12 for _ in range(12)]
    for i in range(6):
        for j in range(6):
            k[i][j] = anchored[i][j]
            k[i][j + 6] = -coupled[i][j]
            k[i + 6][j] = -coupled[j][i]
            k[i + 6][j + 6] = tip[i][j]
    out = [[Fr(0)] * 12 for _ in range(12)]
    for bi in range(4):
        for bj in range(4):
            if all(k[3 * bi + p][3 * bj + q] == 0 for p in range(3) for q in range(3)):
                continue
            tmp = [[Fr(0)] * 3 for _ in range(3)]
            for p in range(3):
                for cc in range(3):
                    acc = Fr(0)
                    for q in range(3):
                        kv = k[3 * bi + p][3 * bj + q]
                        if kv == 0 or axes[q][cc] == 0:
                            continue
                        acc = r.add(acc, r.mul(kv, axes[q][cc]))
                    tmp[p][cc] = acc
            for rr in range(3):
                for cc in range(3):
                    acc = Fr(0)
                    for p in range(3):
                        if axes[p][rr] == 0 or tmp[p][cc] == 0:
                            continue
                        acc = r.add(acc, r.mul(axes[p][rr], tmp[p][cc]))
                    out[3 * bi + rr][3 * bj + cc] = acc
    return out


def chords_at_p(ar, xi, xj, R, y):
    """(actual chord A_p.d_p, formula chord with the cancelling fl(cos) - 1, formula chord with
    the benign -2 s^2, phi_p, |y_perp|_p, L_p) at precision ar.p, B1's operation order."""
    r = ar
    xi = [Fr(v) for v in xi]
    xj = [Fr(v) for v in xj]
    y = [Fr(v) for v in y]
    R = Fr(R)
    d = [r.sub(xj[k], xi[k]) for k in range(3)]
    L = r.sqrt(r.dot3(d, d))
    twoR = 2 * R
    s = r.div(L, twoR)
    c = r.div(r.sqrt(r.mul(r.sub(twoR, L), r.add(twoR, L))), twoR)
    phi = 2 * r.atan2_q1(s, c)
    dh = [r.div(d[k], L) for k in range(3)]
    proj = r.dot3(y, dh)
    yc = [r.sub(y[k], r.mul(proj, dh[k])) for k in range(3)]
    ym = r.sqrt(r.dot3(yc, yc))
    n = [r.div(yc[k], ym) for k in range(3)]
    ex = [r.add(-r.mul(s, dh[k]), r.mul(c, n[k])) for k in range(3)]
    ey = [r.add(r.mul(c, dh[k]), r.mul(s, n[k])) for k in range(3)]
    ez = r.cross3(n, dh)
    act = [r.dot3(ex, d), r.dot3(ey, d), r.dot3(ez, d)]
    ss2 = 2 * r.mul(s, s)
    cos = r.sub(1, ss2)
    sin = 2 * r.mul(s, c)
    f_cancel = [r.mul(R, r.sub(cos, 1)), r.mul(R, sin), Fr(0)]
    f_benign = [r.mul(R, -ss2), r.mul(R, sin), Fr(0)]
    nd = r.dot3(n, dh)
    return dict(act=act, f_cancel=f_cancel, f_benign=f_benign, phi=phi, ym=ym, L=L, s=s, c=c, n_dot_dh=nd)


# =============================================================================== 3. EMU64
def fk_frame_b64(xi, xj, y, sec=F122_SECTION):
    """FK's FrameElement::global_stiffness in FK's binary64 operation order (lib.rs at HEAD)."""
    def dot(a, b):
        return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]

    def norm(v):
        return math.sqrt(dot(v, v))

    def scale(v, s):
        return [v[0] * s, v[1] * s, v[2] * s]

    def sub(a, b):
        return [a[0] - b[0], a[1] - b[1], a[2] - b[2]]

    def cross(a, b):
        return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]

    def normalize(v):
        return scale(v, 1.0 / norm(v))
    delta = sub(xj, xi)
    length = norm(delta)
    xa = normalize(delta)
    proj = dot(y, xa)
    ya = normalize(sub(y, scale(xa, proj)))
    za = normalize(cross(xa, ya))
    e, g, area, iy, iz, j = sec["E"], sec["G"], sec["A"], sec["I"], sec["I"], sec["J"]
    l2 = length * length
    l3 = l2 * length
    axial = (e * area) / length
    torsion = (g * j) / length
    t12, t6, t4, t2 = 12.0 * e, 6.0 * e, 4.0 * e, 2.0 * e
    by12, by6, by4, by2 = (t12 * iy) / l3, (t6 * iy) / l2, (t4 * iy) / length, (t2 * iy) / length
    bz12, bz6, bz4, bz2 = (t12 * iz) / l3, (t6 * iz) / l2, (t4 * iz) / length, (t2 * iz) / length
    k = [[0.0] * 12 for _ in range(12)]
    k[0][6] = k[6][0] = -axial
    k[0][0] = axial
    k[6][6] = axial
    k[3][9] = k[9][3] = -torsion
    k[3][3] = torsion
    k[9][9] = torsion

    def add_terms(idx, terms):
        for rr in range(4):
            for cc in range(4):
                k[idx[rr]][idx[cc]] += terms[rr][cc]
    add_terms([1, 5, 7, 11], [[bz12, bz6, -bz12, bz6], [bz6, bz4, -bz6, bz2], [-bz12, -bz6, bz12, -bz6], [bz6, bz2, -bz6, bz4]])
    add_terms([2, 4, 8, 10], [[by12, -by6, -by12, -by6], [-by6, by4, by6, by2], [-by12, by6, by12, by6], [-by6, by2, by6, by4]])
    axes = [xa, ya, za]
    T = [[0.0] * 12 for _ in range(12)]
    for blk in range(4):
        for rr in range(3):
            for cc in range(3):
                T[3 * blk + rr][3 * blk + cc] = axes[rr][cc]
    temp = [[0.0] * 12 for _ in range(12)]
    for rr in range(12):
        for cc in range(12):
            acc = 0.0
            for inner in range(12):
                acc += k[rr][inner] * T[inner][cc]
            temp[rr][cc] = acc
    res = [[0.0] * 12 for _ in range(12)]
    for rr in range(12):
        for cc in range(12):
            acc = 0.0
            for inner in range(12):
                acc += T[inner][rr] * temp[inner][cc]
            res[rr][cc] = acc
    return res


def _geom_b64(d, R, y, near_pi_exact=False):
    Lsq = d[0] * d[0] + d[1] * d[1] + d[2] * d[2]
    L = math.sqrt(Lsq)
    s = L / (2.0 * R)
    if near_pi_exact:
        # O7: 4R^2 - |d|^2 formed exactly from binary64 R and d, rounded once
        q = Fr(4) * Fr(R) * Fr(R) - sum(Fr(v) * Fr(v) for v in d)
        c = math.sqrt(float(q)) / (2.0 * R)
    else:
        c = math.sqrt((2.0 * R - L) * (2.0 * R + L)) / (2.0 * R)
    phi = 2.0 * math.atan2(s, c)
    dh = [v / L for v in d]
    yd = y[0] * dh[0] + y[1] * dh[1] + y[2] * dh[2]
    nr = [y[k] - yd * dh[k] for k in range(3)]
    nn = math.sqrt(nr[0] * nr[0] + nr[1] * nr[1] + nr[2] * nr[2])
    n = [v / nn for v in nr]
    ex = [-s * dh[k] + c * n[k] for k in range(3)]
    ey = [c * dh[k] + s * n[k] for k in range(3)]
    ez = [n[1] * dh[2] - n[2] * dh[1], n[2] * dh[0] - n[0] * dh[2], n[0] * dh[1] - n[1] * dh[0]]
    return dict(L=L, s=s, c=c, phi=phi, axes=[ex, ey, ez])


def _gram_closed_b64(phi, s, c, stable):
    sinp = 2.0 * s * c
    if stable:
        omc = 2.0 * s * s
        cosp = 1.0 - omc
    else:
        cosp = math.cos(phi)
        omc = 1.0 - cosp
        sinp = math.sin(phi)
    sin2 = 2.0 * sinp * cosp
    g22 = phi / 2.0 - sin2 / 4.0
    if stable and phi < 0.1:
        # series for phi/2 - sin(2 phi)/4 = sum_{k>=1} (-1)^(k+1) (2 phi)^(2k+1) / (4 (2k+1)!)
        t, acc, k = (2.0 * phi) ** 3 / 6.0, 0.0, 1
        while True:
            term = t / 4.0
            acc += term if k % 2 == 1 else -term
            t *= (2.0 * phi) ** 2 / ((2 * k + 2) * (2 * k + 3))
            k += 1
            if abs(t) < 1e-40 * abs(acc) or k > 30:
                break
        g22 = acc
    return [[phi, sinp, omc], [sinp, phi / 2.0 + sin2 / 4.0, sinp * sinp / 2.0], [omc, sinp * sinp / 2.0, g22]], sinp, cosp


def local_F_closed_b64(R, geo, sec, kin, kout, stable):
    """B1's closed form evaluated in binary64 ('stable': half-angle trig and a series for G33;
    the action quadratic forms are still summed term by term, so the in-plane couplings keep
    their cancellation - a plausible but imperfect product)."""
    G, sinp, cosp = _gram_closed_b64(geo["phi"], geo["s"], geo["c"], stable)
    rs, rc = R * sinp, R * cosp
    z3 = (0.0, 0.0, 0.0)
    cases = [[(-rs, 0.0, R), z3, z3, (0.0, 0.0, -1.0)], [(rc, -R, 0.0), z3, z3, (0.0, 1.0, 0.0)],
             [z3, (0.0, rs, -rc), (R, -rc, -rs), z3], [z3, (0.0, 1.0, 0.0), (0.0, 0.0, -1.0), z3],
             [z3, (0.0, 0.0, 1.0), (0.0, 1.0, 0.0), z3], [(1.0, 0.0, 0.0), z3, z3, z3]]

    def q(u, v):
        acc = 0.0
        for i in range(3):
            for j in range(3):
                acc += u[i] * G[i][j] * v[j]
        return acc
    EI, GJ, EA = sec["E"] * sec["I"], sec["G"] * sec["J"], sec["E"] * sec["A"]
    F = [[0.0] * 6 for _ in range(6)]
    for a in range(6):
        for b in range(a, 6):
            v = R * (kin * q(cases[a][0], cases[b][0]) / EI + kout * q(cases[a][1], cases[b][1]) / EI
                     + q(cases[a][2], cases[b][2]) / GJ + q(cases[a][3], cases[b][3]) / EA)
            F[a][b] = F[b][a] = v
    return F


_GL = {}


def _gl(n):
    if n not in _GL:
        with localcontext() as cx:
            cx.prec = 50
            xs, ws = C.gauss_legendre(n)
        _GL[n] = ([float(x) for x in xs], [float(w) for w in ws])
    return _GL[n]


def local_F_gl_b64(R, geo, sec, kin, kout, npts=16):
    """n-point Gauss-Legendre in binary64 with cancellation-free arm differences
    P(phi) - P(t) = 2R sin((phi - t)/2) (-sin((phi + t)/2), cos((phi + t)/2))."""
    xs, ws = _gl(npts)
    phi = geo["phi"]
    EI, GJ, EA = sec["E"] * sec["I"], sec["G"] * sec["J"], sec["E"] * sec["A"]
    F = [[0.0] * 6 for _ in range(6)]
    for x, w in zip(xs, ws):
        th = phi * (x + 1.0) * 0.5
        st, ct = math.sin(th), math.cos(th)
        hm = math.sin((phi - th) * 0.5)
        sp, cp = math.sin((phi + th) * 0.5), math.cos((phi + th) * 0.5)
        ax_, ay_ = -2.0 * R * hm * sp, 2.0 * R * hm * cp
        acts = []
        for a in range(6):
            Fv = [0.0, 0.0, 0.0]
            Mv = [0.0, 0.0, 0.0]
            if a < 3:
                Fv[a] = 1.0
                Mv = [ay_ * Fv[2], -ax_ * Fv[2], ax_ * Fv[1] - ay_ * Fv[0]]
            else:
                Mv[a - 3] = 1.0
            acts.append((-st * Fv[0] + ct * Fv[1], -st * Mv[0] + ct * Mv[1], Mv[2], ct * Mv[0] + st * Mv[1]))
        wt = w * phi * 0.5 * R
        for a in range(6):
            Na, Ta, Ia, Oa = acts[a]
            for b in range(a, 6):
                Nb, Tb, Ib, Ob = acts[b]
                F[a][b] += wt * (kin * Ia * Ib / EI + kout * Oa * Ob / EI + Ta * Tb / GJ + Na * Nb / EA)
    for a in range(6):
        for b in range(a):
            F[a][b] = F[b][a]
    return F


def inv6_b64(F):
    n = 6
    M = [F[i][:] + [1.0 if i == j else 0.0 for j in range(n)] for i in range(n)]
    for col in range(n):
        p = max(range(col, n), key=lambda q: abs(M[q][col]))
        M[col], M[p] = M[p], M[col]
        pv = M[col][col]
        M[col] = [v / pv for v in M[col]]
        for q in range(n):
            if q != col and M[q][col] != 0.0:
                f = M[q][col]
                M[q] = [M[q][k] - f * M[col][k] for k in range(2 * n)]
    K = [row[n:] for row in M]
    for i in range(6):
        for j in range(i + 1, 6):
            m = (K[i][j] + K[j][i]) * 0.5
            K[i][j] = K[j][i] = m
    return K


def curved_b64(d, R, y, sec=F122_SECTION, kin=1.0, kout=1.0, F_mode="closed_stable", chord="actual",
               near_pi_exact=False, F_exact_local=None):
    """A plausible binary64 T4-U1 element (labelled emulation). F_mode: 'closed_stable',
    'closed_naive', 'gl12', 'gl16', 'gl20', 'cr' (exact local F rounded once; F_exact_local must
    be given), 'kt_cr' (the exact local tip stiffness F^-1 rounded once)."""
    g = _geom_b64(d, R, y, near_pi_exact)
    if F_mode == "kt_cr":
        Kt = [[float(v) for v in row] for row in C.inverse(F_exact_local)]
    else:
        if F_mode == "cr":
            F = [[float(v) for v in row] for row in F_exact_local]
        elif F_mode.startswith("gl"):
            F = local_F_gl_b64(R, g, sec, kin, kout, int(F_mode[2:]))
        else:
            F = local_F_closed_b64(R, g, sec, kin, kout, stable=(F_mode == "closed_stable"))
        Kt = inv6_b64(F)
    if chord == "actual":
        c = [-g["s"] * g["L"], g["c"] * g["L"], 0.0]
    elif chord == "formula":
        c = [R * (math.cos(g["phi"]) - 1.0), R * math.sin(g["phi"]), 0.0]
    else:
        c = chord
    H = [[1.0 if i == j else 0.0 for j in range(6)] for i in range(6)]
    H[3][1], H[3][2], H[4][0], H[4][2], H[5][0], H[5][1] = -c[2], c[1], c[2], -c[0], -c[1], c[0]

    def mm(P, Q):
        return [[sum(P[i][t] * Q[t][j] for t in range(len(Q))) for j in range(len(Q[0]))] for i in range(len(P))]
    HK = mm(H, Kt)
    HKHt = mm(HK, [list(rw) for rw in zip(*H)])
    Kl = [[0.0] * 12 for _ in range(12)]
    for i in range(6):
        for j in range(6):
            Kl[i][j] = HKHt[i][j]
            Kl[i][j + 6] = -HK[i][j]
            Kl[i + 6][j] = -HK[j][i]
            Kl[i + 6][j + 6] = Kt[i][j]
    Ax = g["axes"]
    K = [[0.0] * 12 for _ in range(12)]
    for bi in range(4):
        for bj in range(4):
            blk = [[Kl[3 * bi + p][3 * bj + q] for q in range(3)] for p in range(3)]
            tmp = [[sum(blk[p][q] * Ax[q][cc] for q in range(3)) for cc in range(3)] for p in range(3)]
            for rr in range(3):
                for cc in range(3):
                    K[3 * bi + rr][3 * bj + cc] = sum(Ax[p][rr] * tmp[p][cc] for p in range(3))
    return K, g


def exact_local_F(xi, xj, R, y, sec=F122_SECTION, kin=1.0, kout=1.0):
    geo = C.geometry([dec(v) for v in xi], [dec(v) for v in xj], R, [dec(v) for v in y])
    return C.flexibility(geo, *(dec(sec[k]) for k in ("E", "G", "A", "I", "J")), dec(kin), dec(kout))


# ---- FK preparation and factorizations (binary64)
def fk_prepare(Kfull, f, free, springs_b64=()):
    """Reduced binary64 system with springs added in binary64 (SA order: elements, then springs),
    radix-scaled as FK prepare does. Returns (a, rhs, exponents)."""
    n = len(free)
    Kf = [[Kfull[a][b] for b in free] for a in free]
    e = [-(binexp(Kf[r][r]) // 2) for r in range(n)]
    a = [[math.ldexp(Kf[r][c], e[r] + e[c]) for c in range(n)] for r in range(n)]
    for i in range(n):
        for j in range(i):
            if a[i][j] != a[j][i]:
                avg = a[i][j] / 2.0 + a[j][i] / 2.0
                a[i][j] = a[j][i] = avg
    rhs = [math.ldexp(f.get(free[r], 0.0), e[r]) for r in range(n)]
    return a, rhs, e


def chol_solve(a, b):
    n = len(a)
    Lm = [[0.0] * n for _ in range(n)]
    for i in range(n):
        for j in range(i + 1):
            s = a[i][j]
            for k in range(j):
                s = s - Lm[i][k] * Lm[j][k]
            if i == j:
                Lm[i][j] = math.sqrt(s)
            else:
                Lm[i][j] = s / Lm[j][j]
    x = b[:]
    for i in range(n):
        for j in range(i):
            x[i] = x[i] - Lm[i][j] * x[j]
        x[i] = x[i] / Lm[i][i]
    for i in reversed(range(n)):
        for j in range(i + 1, n):
            x[i] = x[i] - Lm[j][i] * x[j]
        x[i] = x[i] / Lm[i][i]
    return x


def ldl_solve(a, b, order):
    """FK's skyline LDL (full profile) of the matrix permuted by `order`."""
    n = len(a)
    P = [[a[order[i]][order[j]] for j in range(n)] for i in range(n)]
    Lw = [row[:] for row in P]
    work = [0.0] * n
    for i in range(n):
        for j in range(i):
            s = Lw[i][j]
            for k in range(j):
                s = s - work[k] * Lw[j][k]
            work[j] = s
            Lw[i][j] = s / Lw[j][j]
        piv = Lw[i][i]
        for k in range(i):
            piv = piv - work[k] * Lw[i][k]
        Lw[i][i] = piv
    x = [b[order[i]] for i in range(n)]
    for i in range(n):
        for j in range(i):
            x[i] = x[i] - Lw[i][j] * x[j]
    for i in range(n):
        x[i] = x[i] / Lw[i][i]
    for i in reversed(range(n)):
        v = x[i]
        for j in range(i):
            x[j] = x[j] - Lw[i][j] * v
    res = [0.0] * n
    for i, o in enumerate(order):
        res[o] = x[i]
    return res


def product_solve(Kfull_b64, f, free, mode, order=None):
    a, rhs, e = fk_prepare(Kfull_b64, f, free)
    if mode == "dense":
        y = chol_solve(a, rhs)
    else:
        y = ldl_solve(a, rhs, order if order is not None else list(range(len(free))))
    return {free[r]: math.ldexp(y[r], e[r]) for r in range(len(free))}


def kd5_trigger(K_int, springs, f, free, u, nodes, K_solve=None):
    """K-D5's rule: rho = f - K_int u - springs u (exact), w = K_solve,free^-1 rho solved exactly
    (K_solve defaults to K_int; K-D5 uses the attempt's factor of the system matrix, which differs
    at second order), trigger = 2|w_i| / (1e-9 max(|u_i|, S*_kind)) with S* from the binary64 u
    and the body extent (one body)."""
    Kd = [[dec(v) if not isinstance(v, Fr) else fr_dec(v) for v in row] for row in K_int]
    Ks = Kd if K_solve is None else [[dec(v) if not isinstance(v, Fr) else fr_dec(v) for v in row] for row in K_solve]
    n = len(Kd)
    sp = dict(springs)
    rho = []
    for a in free:
        acc = dec(f.get(a, 0.0))
        for b in range(n):
            ub = u.get(b, 0.0)
            if ub != 0.0:
                acc -= Kd[a][b] * dec(ub)
        if a in sp:
            acc -= dec(sp[a]) * dec(u.get(a, 0.0))
        rho.append(acc)
    Kf = reduce(Ks, free)
    for i, a in enumerate(free):
        if a in sp:
            Kf[i][i] += dec(sp[a])
    w = C.solve(Kf, rho)
    st = max(abs(u[a]) for a in free if a % 6 < 3)
    sr = max(abs(u[a]) for a in free if a % 6 >= 3)
    ext = [max(p[k] for p in nodes) - min(p[k] for p in nodes) for k in range(3)]
    lb = math.sqrt((ext[0] * ext[0] + ext[1] * ext[1]) + ext[2] * ext[2])
    tr, ro = max(st, lb * sr), max(sr, st / lb)
    worst, at = 0.0, None
    for i, a in enumerate(free):
        sc = max(abs(u[a]), ro if a % 6 >= 3 else tr)
        t = float(2 * abs(w[i]) / (D("1e-9") * dec(sc)))
        if t > worst:
            worst, at = t, a
    return worst, at


def rcm_order(a):
    """sparse_direct's deterministic RCM on the exact nonzero pattern of the prepared matrix."""
    n = len(a)
    nb = [set() for _ in range(n)]
    for r in range(n):
        for c in range(r):
            if a[r][c] != 0.0 or a[c][r] != 0.0:
                nb[r].add(c)
                nb[c].add(r)
    deg = [len(s) for s in nb]
    nbl = [sorted(s, key=lambda q: (deg[q], q)) for s in nb]

    def reach(seed):
        mk = [False] * n
        mk[seed] = True
        out, qu = [seed], [seed]
        while qu:
            x = qu.pop(0)
            for y in nbl[x]:
                if not mk[y]:
                    mk[y] = True
                    out.append(y)
                    qu.append(y)
        return out

    def ecc(st):
        mk = [False] * n
        mk[st] = True
        e, cur, last = 0, [st], [st]
        while True:
            nxt = []
            for x in cur:
                for y in nbl[x]:
                    if not mk[y]:
                        mk[y] = True
                        nxt.append(y)
            if not nxt:
                break
            e += 1
            last = nxt[:]
            cur = nxt
        return e, last

    def mindeg(nodes):
        return min(nodes, key=lambda q: (deg[q], q)) if nodes else None
    visited = [False] * n
    order = []
    for seed in range(n):
        if visited[seed]:
            continue
        cand = mindeg(reach(seed))
        ce, last = ecc(cand)
        while True:
            nx = mindeg(last)
            if nx is None:
                break
            ne, nl = ecc(nx)
            if ne > ce:
                cand, ce, last = nx, ne, nl
            else:
                break
        visited[cand] = True
        qu = [cand]
        while qu:
            x = qu.pop(0)
            order.append(x)
            for y in nbl[x]:
                if not visited[y]:
                    visited[y] = True
                    qu.append(y)
    order.reverse()
    return order


def product_solve_fk(Kfull_b64, f, free, mode):
    """FK dense (Cholesky) or sparse_direct (RCM + skyline LDL^T) on FK's prepared system."""
    a, rhs, e = fk_prepare(Kfull_b64, f, free)
    if mode == "dense":
        y = chol_solve(a, rhs)
    else:
        y = ldl_solve(a, rhs, rcm_order(a))
    return {free[r]: math.ldexp(y[r], e[r]) for r in range(len(free))}
