#!/usr/bin/env python3
"""Independent reference for curved-bend transverse shear (RT-K1 / RT-A1 candidate).

Standard library only. It imports, reads and runs no SWBPIPE product code.

Formulation (EDF Code_Aster R3.08.01 rev. 13337, 9 July 2015, section 8,
pp. 45-49): the end flexibility of a constant-radius, constant-section
circular arc is C = R * integral_0^phi J(theta)^T D J(theta) dtheta, where
J transports the end actions to the section by equilibrium alone and
D = diag(1/ES, 1/(k1 S G), 1/(k2 S G), 1/(G C), 1/(E I1), 1/(E I2)).
Here k1 S = k2 S = A_s, the shear area, an input (SWBPIPE's adopted
energy factor kappa_E is formed elsewhere and is not re-derived here).
The bending flexibility factors k_in and k_out scale only the two bending
terms, as R3.08.01 p. 52 divides only I_y and I_z by them.

The arc, its frame, the section actions and the integrals are formed here
by a construction different from the product's: an explicit centre, the
arc points p(theta), free-body actions of the tip-side segment (nested
Gauss-Legendre quadrature for the distributed load), Gauss-Legendre
quadrature of the energy, and the closed forms derived in the Work note.
All arithmetic is decimal at PREC digits with pi, sin, cos and atan
evaluated by series written here. Binary64 inputs are lifted exactly.

Usage (from this directory's parent):
    python3 -I generators/curved_bend_shear_reference.py [--check]
writes element_cases.json, expected.json and SHA256SUMS next to README.md;
--check regenerates in memory and compares bytes instead of writing.
"""
from decimal import Decimal, getcontext
import hashlib
import json
import os
import sys

PREC = 70
getcontext().prec = PREC
GL_NODES = 40
OUT_DIGITS = 34

ZERO = Decimal(0)
ONE = Decimal(1)
TWO = Decimal(2)
HALF = ONE / TWO


# ---------------------------------------------------------------------------
# Scalar functions (series written here; no math module)
# ---------------------------------------------------------------------------

def D(x):
    """Exact lift of a binary64 (or int / exact decimal string)."""
    if isinstance(x, float):
        return Decimal(x)
    return Decimal(x)


def atan_small(x):
    # Reduce |x| by atan(x) = 2 atan(x / (1 + sqrt(1 + x^2))) until tiny.
    k = 0
    while abs(x) > Decimal("1e-3"):
        x = x / (ONE + (ONE + x * x).sqrt())
        k += 1
    eps = Decimal(10) ** (-(PREC + 5))
    term, total, n, x2 = x, x, 1, x * x
    while True:
        term = -term * x2
        add = term / (2 * n + 1)
        if abs(add) < eps:
            break
        total += add
        n += 1
    return total * (2 ** k)


def compute_pi():
    # Machin: pi = 16 atan(1/5) - 4 atan(1/239)
    return 16 * atan_small(ONE / 5) - 4 * atan_small(ONE / 239)


PI = compute_pi()


def sin_cos(x):
    """(sin x, cos x) by Taylor series after halving, then double-angle."""
    k = 0
    y = x
    while abs(y) > Decimal("1e-2"):
        y = y / 2
        k += 1
    eps = Decimal(10) ** (-(PREC + 10))
    s, c = y, ONE
    ts, tc, n = y, ONE, 1
    y2 = y * y
    while True:
        ts = -ts * y2 / ((2 * n) * (2 * n + 1))
        tc = -tc * y2 / ((2 * n - 1) * (2 * n))
        if abs(ts) < eps and abs(tc) < eps:
            break
        s += ts
        c += tc
        n += 1
    for _ in range(k):
        s, c = 2 * s * c, c * c - s * s
    return s, c


def atan2_pos(y, x):
    """atan2 for y > 0 (result in (0, pi))."""
    assert y > 0
    if x > 0:
        if y <= x:
            return atan_small(y / x)
        return PI / 2 - atan_small(x / y)
    if x == 0:
        return PI / 2
    if y <= -x:
        return PI - atan_small(y / -x)
    return PI / 2 + atan_small(-x / y)


# ---------------------------------------------------------------------------
# Vectors and small matrices
# ---------------------------------------------------------------------------

def add(a, b):
    return [p + q for p, q in zip(a, b)]


def sub(a, b):
    return [p - q for p, q in zip(a, b)]


def scale(a, s):
    return [p * s for p in a]


def dot(a, b):
    return sum((p * q for p, q in zip(a, b)), ZERO)


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def norm(a):
    return dot(a, a).sqrt()


def unit(a):
    return scale(a, ONE / norm(a))


def matmul(a, b):
    n, m, p = len(a), len(b), len(b[0])
    return [[sum((a[i][k] * b[k][j] for k in range(m)), ZERO) for j in range(p)] for i in range(n)]


def transpose(a):
    return [list(r) for r in zip(*a)]


def inverse(a):
    n = len(a)
    m = [list(row) + [ONE if i == j else ZERO for j in range(n)] for i, row in enumerate(a)]
    for col in range(n):
        piv = max(range(col, n), key=lambda r: abs(m[r][col]))
        m[col], m[piv] = m[piv], m[col]
        p = m[col][col]
        m[col] = [v / p for v in m[col]]
        for r in range(n):
            if r != col and m[r][col] != 0:
                f = m[r][col]
                m[r] = [v - f * w for v, w in zip(m[r], m[col])]
    return [row[n:] for row in m]


# ---------------------------------------------------------------------------
# Gauss-Legendre rule on [-1, 1] (Newton on the Legendre recurrence)
# ---------------------------------------------------------------------------

def legendre(n, x):
    p0, p1 = ONE, x
    for k in range(2, n + 1):
        p0, p1 = p1, ((2 * k - 1) * x * p1 - (k - 1) * p0) / k
    dp = n * (x * p1 - p0) / (x * x - ONE)
    return p1, dp


def gauss_legendre(n):
    nodes, weights = [], []
    eps = Decimal(10) ** (-(PREC - 5))
    for i in range(1, n + 1):
        _, x = sin_cos(PI * (Decimal(i) - Decimal("0.25")) / (Decimal(n) + HALF))
        for _ in range(100):
            p, dp = legendre(n, x)
            dx = p / dp
            x -= dx
            if abs(dx) < eps:
                break
        _, dp = legendre(n, x)
        nodes.append(x)
        weights.append(TWO / ((ONE - x * x) * dp * dp))
    return nodes, weights


GL = gauss_legendre(GL_NODES)


def quad(f, a, b):
    """integral_a^b f(t) dt; f returns a list of Decimals."""
    half = (b - a) / 2
    mid = (a + b) / 2
    total = None
    for x, w in zip(*GL):
        v = f(mid + half * x)
        total = [w * e for e in v] if total is None else [t + w * e for t, e in zip(total, v)]
    return [t * half for t in total]


# ---------------------------------------------------------------------------
# The arc, built from (x_i, x_j, R, y_reference) through an explicit centre
# ---------------------------------------------------------------------------

class Arc:
    def __init__(self, xi, xj, radius, yref):
        self.xi = [D(v) for v in xi]
        self.xj = [D(v) for v in xj]
        self.R = D(radius)
        y = [D(v) for v in yref]
        d = sub(self.xj, self.xi)
        L = norm(d)
        dh = scale(d, ONE / L)
        bow = unit(sub(y, scale(dh, dot(y, dh))))
        h = (self.R * self.R - L * L / 4).sqrt()
        mid = scale(add(self.xi, self.xj), HALF)
        self.centre = sub(mid, scale(bow, h))
        ri = scale(sub(self.xi, self.centre), ONE / self.R)
        rj = scale(sub(self.xj, self.centre), ONE / self.R)
        self.b = unit(cross(ri, d))
        self.ri = ri
        self.ti = cross(self.b, ri)
        self.phi = atan2_pos(norm(cross(ri, rj)), dot(ri, rj))
        self.chord = d
        # Local frame rows: radial at i, tangent at i, binormal.
        self.axes = [self.ri, self.ti, self.b]

    def frame(self, theta):
        s, c = sin_cos(theta)
        r = add(scale(self.ri, c), scale(self.ti, s))
        t = add(scale(self.ri, -s), scale(self.ti, c))
        p = add(self.centre, scale(r, self.R))
        return p, r, t, self.b


class Section:
    def __init__(self, E, G, A, I, J, k_in, k_out, shear_area):
        self.EA = D(E) * D(A)
        self.GJ = D(G) * D(J)
        self.EI = D(E) * D(I)
        self.k_in = D(k_in)
        self.k_out = D(k_out)
        self.GAs = None if shear_area is None else D(G) * D(shear_area)

    def energy(self, a, b, families=("axial", "shear", "torsion", "bending")):
        """Energy-density bilinear form of two action sets (N, Vr, Vb, T, Mr, Mb)."""
        e = ZERO
        if "axial" in families:
            e += a[0] * b[0] / self.EA
        if "shear" in families and self.GAs is not None:
            e += (a[1] * b[1] + a[2] * b[2]) / self.GAs
        if "torsion" in families:
            e += a[3] * b[3] / self.GJ
        if "bending" in families:
            e += self.k_out * a[4] * b[4] / self.EI + self.k_in * a[5] * b[5] / self.EI
        return e


def resolve(F, M, r, t, b):
    """Section actions (N, Vr, Vb, T, Mr, Mb) from force and moment vectors."""
    return [dot(F, t), dot(F, r), dot(F, b), dot(M, t), dot(M, r), dot(M, b)]


def unit_tip_actions(arc, theta, ptip, k):
    """Actions at theta of unit global load k (0-2 force, 3-5 moment) at point ptip."""
    p, r, t, b = arc.frame(theta)
    e = [ZERO] * 3
    if k < 3:
        e[k] = ONE
        F, M = e, cross(sub(ptip, p), e)
    else:
        e[k - 3] = ONE
        F, M = [ZERO] * 3, e
    return resolve(F, M, r, t, b)


def udl_actions(arc, theta, w):
    """Actions at theta of the tip-side segment's uniform load w (per arc length)."""
    p, r, t, b = arc.frame(theta)
    if arc.phi - theta == 0:
        return [ZERO] * 6

    def seg(tp):
        q, _, _, _ = arc.frame(tp)
        return list(w) + cross(sub(q, p), w)

    v = quad(seg, theta, arc.phi)
    F = scale(v[:3], arc.R)
    M = scale(v[3:], arc.R)
    return resolve(F, M, r, t, b)


def flexibility_global(arc, sec, families):
    ptip = arc.frame(arc.phi)[0]

    def integrand(theta):
        acts = [unit_tip_actions(arc, theta, ptip, k) for k in range(6)]
        return [sec.energy(acts[i], acts[j], families) for i in range(6) for j in range(6)]

    v = quad(integrand, ZERO, arc.phi)
    return [[v[6 * i + j] * arc.R for j in range(6)] for i in range(6)]


def to_local6(arc, m):
    a = [[ZERO] * 6 for _ in range(6)]
    for blk in (0, 3):
        for i in range(3):
            for j in range(3):
                a[blk + i][blk + j] = arc.axes[i][j]
    return matmul(matmul(a, m), transpose(a))


def vec_to_local6(arc, v):
    return [dot(arc.axes[i], v[:3]) for i in range(3)] + [dot(arc.axes[i], v[3:]) for i in range(3)]


def udl_tip_deflection_global(arc, sec, w, families):
    ptip = arc.frame(arc.phi)[0]

    def integrand(theta):
        load = udl_actions(arc, theta, w)
        return [sec.energy(unit_tip_actions(arc, theta, ptip, k), load, families) for k in range(6)]

    return scale(quad(integrand, ZERO, arc.phi), arc.R)


def transfer(d):
    """H: node-j end action moved to node i (force unchanged, moment + d x f)."""
    h = [[ONE if i == j else ZERO for j in range(6)] for i in range(6)]
    h[3][1], h[3][2] = -d[2], d[1]
    h[4][0], h[4][2] = d[2], -d[0]
    h[5][0], h[5][1] = -d[1], d[0]
    return h


def stiffness12(arc, Fg):
    Kg = inverse(Fg)
    H = transfer(arc.chord)
    HK = matmul(H, Kg)
    Kii = matmul(HK, transpose(H))
    Kji = matmul(Kg, transpose(H))
    K = [[ZERO] * 12 for _ in range(12)]
    for i in range(6):
        for j in range(6):
            K[i][j] = Kii[i][j]
            K[i][6 + j] = -HK[i][j]
            K[6 + i][j] = -Kji[i][j]
            K[6 + i][6 + j] = Kg[i][j]
    return K, Kg


def consistent_loads(arc, Kg, delta0, w):
    X = [-sum((Kg[i][k] * delta0[k] for k in range(6)), ZERO) for i in range(6)]
    H = transfer(arc.chord)

    def res(theta):
        q = arc.frame(theta)[0]
        return list(w) + cross(sub(q, arc.xi), w)

    W = scale(quad(res, ZERO, arc.phi), arc.R)
    HX = [sum((H[i][k] * X[k] for k in range(6)), ZERO) for i in range(6)]
    return [HX[i] + W[i] for i in range(6)] + [-x for x in X]


# ---------------------------------------------------------------------------
# Closed forms (Work note, "Derivation")
# ---------------------------------------------------------------------------

def closed_shear_flex(phi):
    """Delta F * (G A_s) / R in the local frame, translational 3x3."""
    S, C = sin_cos(phi)
    return [[(phi + S * C) / 2, S * S / 2, ZERO], [S * S / 2, (phi - S * C) / 2, ZERO], [ZERO, ZERO, phi]]


def closed_shear_udl(phi):
    """Shear free-tip translation * (G A_s) / R^2 per unit local w, 3x3."""
    S, C = sin_cos(phi)
    return [[(phi * phi + S * S) / 4, (phi - S * C) / 4, ZERO],
            [(phi - S * C) / 4, (phi * phi - S * S) / 4, ZERO],
            [ZERO, ZERO, phi * phi / 2]]


# ---------------------------------------------------------------------------
# Output helpers
# ---------------------------------------------------------------------------

def fmt_s(x):
    with_ctx = getcontext().copy()
    with_ctx.prec = OUT_DIGITS
    v = with_ctx.plus(x)
    if v == 0:
        return "0"
    return format(v, "." + str(OUT_DIGITS - 1) + "E")


def fv(v):
    return [fmt_s(x) for x in v]


def fm(m):
    return [fv(r) for r in m]


def max_rel(a, b, floor):
    worst = ZERO
    for x, y in zip(a, b):
        worst = max(worst, abs(x - y) / max(abs(y), floor))
    return worst


def flat(m):
    return [x for r in m for x in r]


# ---------------------------------------------------------------------------
# Cases (invented inputs; binary64-exact where a placement test needs it)
# ---------------------------------------------------------------------------

E, G = 2.0e11, 8.0e10
A, I, J, AS = 0.0078125, 3.0517578125e-05, 6.103515625e-05, 0.00390625
W_GLOBAL = [300.0, -1000.0, 250.0]

STIFF_CASES = [
    {"id": "Q1", "note": "quarter arc in the global XY plane; phi = pi/2 exactly",
     "x_i": [1.5, 0.0, 0.0], "x_j": [0.0, 1.5, 0.0], "radius": 1.5, "y_reference": [1.0, 1.0, 0.0],
     "k_in": 1.0, "k_out": 1.0},
    {"id": "Q1K", "note": "Q1 with bending flexibility factors; the shear block must not move",
     "x_i": [1.5, 0.0, 0.0], "x_j": [0.0, 1.5, 0.0], "radius": 1.5, "y_reference": [1.0, 1.0, 0.0],
     "k_in": 2.5, "k_out": 1.75},
    {"id": "S1", "note": "skew arc, phi about 1.72 rad, all coordinates dyadic",
     "x_i": [0.375, -1.25, 2.5], "x_j": [1.125, 0.375, 1.875], "radius": 1.25, "y_reference": [0.25, 0.875, 0.5],
     "k_in": 1.0, "k_out": 1.0},
    {"id": "S1T", "note": "S1 translated by (1024, -2048, 512), exact in binary64; K and the load vector equal S1's",
     "x_i": [1024.375, -2049.25, 514.5], "x_j": [1025.125, -2047.625, 513.875], "radius": 1.25,
     "y_reference": [0.25, 0.875, 0.5], "k_in": 1.0, "k_out": 1.0},
    {"id": "S1P", "note": "S1 under the cyclic axis permutation (x, y, z) -> (y, z, x) of points, y_reference and w",
     "x_i": [-1.25, 2.5, 0.375], "x_j": [0.375, 1.875, 1.125], "radius": 1.25, "y_reference": [0.875, 0.5, 0.25],
     "k_in": 1.0, "k_out": 1.0, "w_permuted": True},
]

# Angle sweep: chord along global x, R = 1 unless stated; phi = 2 asin(L / (2R)).
SWEEP_CASES = [
    {"id": "A_2e-8", "chord": 2e-8, "radius": 1.0},
    {"id": "A_2e-4", "chord": 2e-4, "radius": 1.0},
    {"id": "A_0.02", "chord": 0.02, "radius": 1.0},
    {"id": "A_0.5", "chord": 0.5, "radius": 1.0},
    {"id": "A_switch_below", "chord": 0.9999999999999999, "radius": 1.0},
    {"id": "A_switch", "chord": 1.0, "radius": 1.0},
    {"id": "A_switch_above", "chord": 1.0000000000000002, "radius": 1.0},
    {"id": "A_quarter_near", "chord": 1.4142135623730951, "radius": 1.0},
    {"id": "A_1.9", "chord": 1.9, "radius": 1.0},
    {"id": "A_near_pi", "chord": 1.9999999, "radius": 1.0},
    {"id": "L_R50", "chord": 1.0, "radius": 50.0, "straight_limit": True},
    {"id": "L_R5e3", "chord": 1.0, "radius": 5000.0, "straight_limit": True},
    {"id": "L_R5e5", "chord": 1.0, "radius": 500000.0, "straight_limit": True},
    {"id": "L_R5e7", "chord": 1.0, "radius": 50000000.0, "straight_limit": True},
]


def sweep_case(c):
    arc = Arc([0.0, 0.0, 0.0], [c["chord"], 0.0, 0.0], c["radius"], [0.0, 1.0, 0.0])
    phi = arc.phi
    cf = closed_shear_flex(phi)
    cu = closed_shear_udl(phi)
    # Quadrature from the action fields with G A_s = 1, R = 1 normalisation.
    unit_sec = Section(1, 1, 1, 1, 1, 1, 1, None)
    unit_sec.GAs = ONE
    qf = to_local6(arc, flexibility_global(arc, unit_sec, ("shear",)))
    qf = [[qf[i][j] / arc.R for j in range(3)] for i in range(3)]
    qu, rot = [], []
    for k in range(3):
        wl = [ONE if m == k else ZERO for m in range(3)]
        wg = [sum((arc.axes[a][g] * wl[a] for a in range(3)), ZERO) for g in range(3)]
        dl = vec_to_local6(arc, udl_tip_deflection_global(arc, unit_sec, wg, ("shear",)))
        qu.append([dl[m] / (arc.R * arc.R) for m in range(3)])
        rot.append(dl[3:])
    qu = transpose(qu)  # rows: deflection component, cols: w component
    out = {
        "id": c["id"],
        "inputs": {"x_i": [0.0, 0.0, 0.0], "x_j": [c["chord"], 0.0, 0.0], "radius": c["radius"],
                   "y_reference": [0.0, 1.0, 0.0]},
        "phi": fmt_s(phi),
        "half_sine": fmt_s(D(c["chord"]) / (2 * D(c["radius"]))),
        "phi_minus_sin_cos": fmt_s(phi - sin_cos(phi)[0] * sin_cos(phi)[1]),
        "phi2_minus_sin2": fmt_s(phi * phi - sin_cos(phi)[0] ** 2),
        "shear_flexibility_over_R_per_GAs": fm(cf),
        "shear_udl_translation_over_R2_per_GAs": fm(cu),
        "quadrature_max_relative_difference": {
            "flexibility": fmt_s(max_rel(flat(qf), flat(cf), max(abs(x) for x in flat(cf)) * Decimal("1e-30"))),
            "udl": fmt_s(max_rel(flat(qu), flat(cu), max(abs(x) for x in flat(cu)) * Decimal("1e-30"))),
            "udl_rotation_abs": fmt_s(max(abs(x) for r in rot for x in r)),
        },
    }
    if c.get("straight_limit"):
        L = arc.R * phi
        out["arc_length"] = fmt_s(L)
        # Straight Timoshenko cantilever: tip shear flexibility L/(G A_s) transverse, 0 axial;
        # UDL shear translation w L^2 / (2 G A_s) transverse.
        out["straight_limit_relative_deviation"] = {
            "flex_xx": fmt_s((arc.R * cf[0][0] - L) / L),
            "flex_zz": fmt_s((arc.R * cf[2][2] - L) / L),
            "flex_yy_over_L": fmt_s(arc.R * cf[1][1] / L),
            "udl_xx": fmt_s((arc.R ** 2 * cu[0][0] - L * L / 2) / (L * L / 2)),
            "udl_zz": fmt_s((arc.R ** 2 * cu[2][2] - L * L / 2) / (L * L / 2)),
            "udl_yy_over_half_L2": fmt_s(arc.R ** 2 * cu[1][1] / (L * L / 2)),
        }
    return out, max(D(out["quadrature_max_relative_difference"]["flexibility"]),
                    D(out["quadrature_max_relative_difference"]["udl"]))


def stiff_case(c):
    arc = Arc(c["x_i"], c["x_j"], c["radius"], c["y_reference"])
    w = W_GLOBAL if not c.get("w_permuted") else [W_GLOBAL[1], W_GLOBAL[2], W_GLOBAL[0]]
    wg = [D(v) for v in w]
    v2 = Section(E, G, A, I, J, c["k_in"], c["k_out"], AS)
    eb = Section(E, G, A, I, J, c["k_in"], c["k_out"], None)
    all_f = ("axial", "shear", "torsion", "bending")
    Fg_v2 = flexibility_global(arc, v2, all_f)
    Fg_eb = flexibility_global(arc, eb, all_f)
    Fl_v2, Fl_eb = to_local6(arc, Fg_v2), to_local6(arc, Fg_eb)
    dF = [[Fl_v2[i][j] - Fl_eb[i][j] for j in range(6)] for i in range(6)]
    cf = closed_shear_flex(arc.phi)
    gas = D(G) * D(AS)
    closed_dF = [[ZERO] * 6 for _ in range(6)]
    for i in range(3):
        for j in range(3):
            closed_dF[i][j] = arc.R * cf[i][j] / gas
    scale_f = max(abs(x) for x in flat(Fl_v2))
    dF_err = max(abs(x - y) for x, y in zip(flat(dF), flat(closed_dF))) / scale_f
    K_v2, Kg_v2 = stiffness12(arc, Fg_v2)
    K_eb, Kg_eb = stiffness12(arc, Fg_eb)
    d0_v2 = udl_tip_deflection_global(arc, v2, wg, all_f)
    d0_eb = udl_tip_deflection_global(arc, eb, wg, all_f)
    p_v2 = consistent_loads(arc, Kg_v2, d0_v2, wg)
    p_eb = consistent_loads(arc, Kg_eb, d0_eb, wg)
    # Shear part of the free-tip deflection: quadrature vs closed form (local).
    d0_sh = vec_to_local6(arc, udl_tip_deflection_global(arc, v2, wg, ("shear",)))
    wl = [dot(arc.axes[a], wg) for a in range(3)]
    cu = closed_shear_udl(arc.phi)
    closed_sh = [arc.R * arc.R * sum((cu[i][k] * wl[k] for k in range(3)), ZERO) / gas for i in range(3)]
    sh_err = max(abs(d0_sh[i] - closed_sh[i]) for i in range(3)) / max(abs(x) for x in closed_sh)
    rot_sh = max(abs(x) for x in d0_sh[3:])
    # Equilibrium transport: flexibility of node i with node j clamped, formed
    # by integrating from the other end, must equal H Kg H^T's inverse.
    p_i = arc.frame(ZERO)[0]

    def rev_integrand(theta):
        # Actions of a unit load at node i carried by the segment [0, theta];
        # sign of the section actions flips with the free-body side, which the
        # quadratic energy does not see.
        acts = [unit_tip_actions(arc, theta, p_i, k) for k in range(6)]
        return [v2.energy(acts[i], acts[j], all_f) for i in range(6) for j in range(6)]

    rv = quad(rev_integrand, ZERO, arc.phi)
    Fi = [[rv[6 * i + j] * arc.R for j in range(6)] for i in range(6)]
    Kii_rev = inverse(Fi)
    Kii = [[K_v2[i][j] for j in range(6)] for i in range(6)]
    kscale = max(abs(x) for x in flat(K_v2))
    transport_err = max(abs(x - y) for x, y in zip(flat(Kii_rev), flat(Kii))) / kscale
    # Rigid motions: K * rigid = 0.
    rigid_err = ZERO
    for mode in range(6):
        u = [ZERO] * 12
        if mode < 3:
            u[mode] = u[6 + mode] = ONE
        else:
            om = [ZERO] * 3
            om[mode - 3] = ONE
            ui = cross(om, arc.xi)
            uj = cross(om, arc.xj)
            u[0:3], u[3:6], u[6:9], u[9:12] = ui, om, uj, om
        f = [sum((K_v2[r][k] * u[k] for k in range(12)), ZERO) for r in range(12)]
        rigid_err = max(rigid_err, max(abs(x) for x in f) / (kscale * max(ONE, norm(arc.xi), norm(arc.xj))))
    # Betti: tip translation under the UDL equals the work of w through the
    # interior displacement field of each unit tip force (double quadrature).
    ptip = arc.frame(arc.phi)[0]

    def betti(psi, sec, fams):
        q = arc.frame(psi)[0]

        def inner(theta):
            out = []
            for k in range(3):
                tip = unit_tip_actions(arc, theta, ptip, k)
                out.append(sum((wg[m] * sec.energy(unit_tip_actions(arc, theta, q, m), tip, fams)
                                for m in range(3)), ZERO))
            return out

        return scale(quad(inner, ZERO, psi), arc.R) if psi > 0 else [ZERO] * 3

    betti_v2 = scale(quad(lambda s: betti(s, v2, all_f), ZERO, arc.phi), arc.R)
    betti_sh = scale(quad(lambda s: betti(s, v2, ("shear",)), ZERO, arc.phi), arc.R)
    d0_sh_g = udl_tip_deflection_global(arc, v2, wg, ("shear",))
    betti_err = max(abs(betti_v2[k] - d0_v2[k]) for k in range(3)) / max(abs(x) for x in d0_v2[:3])
    betti_sh_err = max(abs(betti_sh[k] - d0_sh_g[k]) for k in range(3)) / max(abs(x) for x in d0_sh_g[:3])
    # No double counting: N^2 + Vr^2 + Vb^2 = |F|^2 at every node of the rule.
    split_err = ZERO
    for x in GL[0]:
        theta = arc.phi * (x + 1) / 2
        _, r, t, b = arc.frame(theta)
        for k in range(3):
            a = unit_tip_actions(arc, theta, ptip, k)
            split_err = max(split_err, abs(a[0] ** 2 + a[1] ** 2 + a[2] ** 2 - ONE))
    out = {
        "id": c["id"],
        "phi": fmt_s(arc.phi),
        "local_axes_rows": fm(arc.axes),
        "w_global": [float(v) for v in w],
        "w_local": fv(wl),
        "end_flexibility_local_v2": fm(Fl_v2),
        "end_flexibility_local_euler_bernoulli": fm(Fl_eb),
        "shear_flexibility_local_closed_form": fm(closed_dF),
        "global_stiffness_v2": fm(K_v2),
        "global_stiffness_euler_bernoulli": fm(K_eb),
        "free_tip_deflection_local_v2": fv(vec_to_local6(arc, d0_v2)),
        "free_tip_deflection_local_shear_closed_form": fv(closed_sh),
        "consistent_uniform_load_global_v2": fv(p_v2),
        "consistent_uniform_load_global_euler_bernoulli": fv(p_eb),
        "checks": {
            "shear_block_quadrature_vs_closed_form_rel": fmt_s(dF_err),
            "shear_udl_quadrature_vs_closed_form_rel": fmt_s(sh_err),
            "shear_udl_rotation_abs": fmt_s(rot_sh),
            "equilibrium_transport_Kii_rel": fmt_s(transport_err),
            "rigid_motion_residual_rel": fmt_s(rigid_err),
            "betti_total_rel": fmt_s(betti_err),
            "betti_shear_rel": fmt_s(betti_sh_err),
            "force_split_identity_abs": fmt_s(split_err),
        },
    }
    worst = max(dF_err, sh_err, transport_err, rigid_err, betti_err, betti_sh_err, split_err)
    return out, worst, (K_v2, p_v2)


def build():
    cases_doc = {
        "schema": "swbpipe.validation.curved_bend_shear.cases/1",
        "status": "candidate; invented inputs; no acceptance",
        "units": "SI (m, N, Pa); w is force per unit arc length in global components",
        "section": {"elastic_modulus": E, "shear_modulus": G, "area": A, "second_moment": I,
                    "torsion_constant": J, "shear_area": AS},
        "uniform_load_global": W_GLOBAL,
        "stiffness_cases": STIFF_CASES,
        "sweep_cases": [{**c, "x_i": [0.0, 0.0, 0.0], "x_j": [c["chord"], 0.0, 0.0],
                         "y_reference": [0.0, 1.0, 0.0]} for c in SWEEP_CASES],
    }
    expected = {
        "schema": "swbpipe.validation.curved_bend_shear.expected/1",
        "status": "candidate; independent of product code; no acceptance",
        "arithmetic": {"decimal_precision": PREC, "gauss_legendre_nodes": GL_NODES,
                       "output_significant_digits": OUT_DIGITS},
        "conventions": (
            "Local frame rows: x radial outward at node i, y tangent at node i toward j, z = x cross y. "
            "Matrices are row-major. Global DOF order [ux, uy, uz, rx, ry, rz] at node i then node j. "
            "Flexibility is node j's with node i clamped. Consistent loads are equivalent nodal loads "
            "p (system load += p; true end forces K d - p)."),
        "stiffness_cases": [],
        "sweep_cases": [],
        "invariance": {},
    }
    worst = ZERO
    results = {}
    for c in STIFF_CASES:
        out, err, kp = stiff_case(c)
        expected["stiffness_cases"].append(out)
        results[c["id"]] = kp
        worst = max(worst, err)
    for c in SWEEP_CASES:
        out, err = sweep_case(c)
        expected["sweep_cases"].append(out)
        worst = max(worst, err)
    # Placement invariance (exact in the reals; reported in decimal).
    k1, p1 = results["S1"]
    kt, pt = results["S1T"]
    kp, pp = results["S1P"]
    perm = [1, 2, 0]  # new axis a takes old axis perm[a]

    def P12(m):
        idx = [3 * blk + perm[a] for blk in range(4) for a in range(3)]
        return [[m[idx[i]][idx[j]] for j in range(12)] for i in range(12)]

    def p12(v):
        idx = [3 * blk + perm[a] for blk in range(4) for a in range(3)]
        return [v[idx[i]] for i in range(12)]

    ks = max(abs(x) for x in flat(k1))
    ps = max(abs(x) for x in p1)
    inv = {
        "translation_K_rel": max(abs(x - y) for x, y in zip(flat(kt), flat(k1))) / ks,
        "translation_load_rel": max(abs(x - y) for x, y in zip(pt, p1)) / ps,
        "permutation_K_rel": max(abs(x - y) for x, y in zip(flat(kp), flat(P12(k1)))) / ks,
        "permutation_load_rel": max(abs(x - y) for x, y in zip(pp, p12(p1))) / ps,
    }
    q = {c["id"]: c for c in expected["stiffness_cases"]}
    qa, qk = q["Q1"], q["Q1K"]
    inv["k_separation_shear_block_rel"] = max(
        abs(D(x) - D(y)) for x, y in zip(flat(qa["shear_flexibility_local_closed_form"]),
                                        flat(qk["shear_flexibility_local_closed_form"])))
    v2k = flat([[D(x) for x in r] for r in qk["end_flexibility_local_v2"]])
    ebk = flat([[D(x) for x in r] for r in qk["end_flexibility_local_euler_bernoulli"]])
    shk = flat([[D(x) for x in r] for r in qk["shear_flexibility_local_closed_form"]])
    inv["k_separation_v2_minus_eb_vs_shear_rel"] = max(
        abs(a - b - s) for a, b, s in zip(v2k, ebk, shk)) / max(abs(x) for x in v2k)
    for v in inv.values():
        worst = max(worst, v)
    expected["invariance"] = {k: fmt_s(v) for k, v in inv.items()}
    expected["worst_internal_discrepancy"] = fmt_s(worst)
    return cases_doc, expected, worst


def dumps(doc):
    return (json.dumps(doc, indent=1, sort_keys=True, ensure_ascii=True) + "\n").encode()


def main():
    here = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    cases_doc, expected, worst = build()
    files = {"element_cases.json": dumps(cases_doc), "expected.json": dumps(expected)}
    gen_rel = "generators/curved_bend_shear_reference.py"
    with open(os.path.join(here, gen_rel), "rb") as fh:
        gen_bytes = fh.read()
    readme = os.path.join(here, "README.md")
    sums_src = dict(files)
    sums_src[gen_rel] = gen_bytes
    if os.path.exists(readme):
        with open(readme, "rb") as fh:
            sums_src["README.md"] = fh.read()
    sums = "".join(f"{hashlib.sha256(sums_src[n]).hexdigest()}  {n}\n" for n in sorted(sums_src)).encode()
    files["SHA256SUMS"] = sums
    if worst > Decimal("1e-28"):
        print(f"internal discrepancy {worst} exceeds 1e-28", file=sys.stderr)
        return 1
    if "--check" in sys.argv:
        bad = []
        for name, data in files.items():
            with open(os.path.join(here, name), "rb") as fh:
                if fh.read() != data:
                    bad.append(name)
        print("check: " + ("OK" if not bad else "DIFFERS: " + ", ".join(bad)))
        return 1 if bad else 0
    for name, data in files.items():
        with open(os.path.join(here, name), "wb") as fh:
            fh.write(data)
    print(f"wrote {', '.join(sorted(files))}; worst internal discrepancy {fmt_s(worst)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
