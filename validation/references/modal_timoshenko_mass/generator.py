#!/usr/bin/env python3
"""Exact consistent mass of a two-node Timoshenko straight frame member.

Source-blind derivation from PROBLEM.md (SHA-256
ab4a949c2533603d91d27cdeff561207d0339997dbe1e53bbc783ca8cc00784d).

Standard library only. Every value is an exact rational (fractions.Fraction)
from the binary64 operands to the final output; floating point is never used.

Default mode rebuilds the output in memory and compares it byte for byte with
the written file (exit 0 when identical, 1 when not). --write writes it.
Every self-check runs in both modes; any failure raises and exits non-zero.
"""

import argparse
import json
import os
import sys
from fractions import Fraction
from math import isqrt

PROBLEM_SHA256 = "ab4a949c2533603d91d27cdeff561207d0339997dbe1e53bbc783ca8cc00784d"
OUTPUT_NAME = "timoshenko_consistent_mass.json"
FORMAT_ID = "swbpipe-source-blind-timoshenko-consistent-mass/1"
ADDENDUM1_SHA256 = "40b5954acac00205fe05db8fa167251f82b5c8a21850eb86e8df74e614cc1d91"
ADDENDUM1_OUTPUT_NAME = "timoshenko_consistent_mass_addendum1.json"
ADDENDUM1_FORMAT_ID = "swbpipe-source-blind-timoshenko-consistent-mass-addendum1/1"

ZERO = Fraction(0)
ONE = Fraction(1)


class CheckFailure(Exception):
    pass


def require(cond, what):
    if not cond:
        raise CheckFailure(what)


# --------------------------------------------------------------------------
# binary64 <-> exact rational
# --------------------------------------------------------------------------

def bits_to_fraction(hexstr):
    """Exact rational value of a 16-hex-digit IEEE 754 binary64 bit pattern."""
    if len(hexstr) != 16:
        raise ValueError("bit pattern must have 16 hex digits: %r" % hexstr)
    b = int(hexstr, 16)
    sign = -1 if (b >> 63) else 1
    e = (b >> 52) & 0x7FF
    m = b & ((1 << 52) - 1)
    if e == 0x7FF:
        raise ValueError("non-finite operand %s" % hexstr)
    if e == 0:
        v = Fraction(m, 1 << 1074)
    else:
        v = Fraction((1 << 52) | m) * pow2(e - 1075)
    return sign * v


def pow2(k):
    return Fraction(1 << k) if k >= 0 else Fraction(1, 1 << (-k))


def round_half_even(x):
    """Nearest integer to a non-negative rational, ties to even."""
    require(x >= 0, "round_half_even expects a non-negative value")
    q, r = divmod(x.numerator, x.denominator)
    twice = 2 * r
    if twice > x.denominator or (twice == x.denominator and q % 2 == 1):
        q += 1
    return q


def floor_log2(a):
    """Largest integer e with 2**e <= a, for rational a > 0."""
    e = a.numerator.bit_length() - a.denominator.bit_length()
    while pow2(e) > a:
        e -= 1
    while pow2(e + 1) <= a:
        e += 1
    return e


def to_binary64(x, signed_zero_underflow=False):
    """Round a rational to binary64, nearest, ties to even.

    Returns (bit pattern as int, exact rational value of the rounded result).
    Raises on overflow. A nonzero value that rounds to zero raises, unless
    signed_zero_underflow is set: then it gives the zero carrying x's sign.
    """
    if x == 0:
        return 0, ZERO
    sign = 1 if x < 0 else 0
    a = -x if sign else x
    e = floor_log2(a)
    qexp = max(e - 52, -1074)  # exponent of one unit in the last place
    n = round_half_even(a / pow2(qexp))
    if n == 0:
        if signed_zero_underflow:
            return sign << 63, ZERO
        raise CheckFailure("nonzero value underflows to zero in binary64")
    if n >= (1 << 53):  # rounding carried into the next binade
        n >>= 1  # n was exactly 2**53
        qexp += 1
    if n < (1 << 52):  # subnormal
        biased, mant = 0, n
    else:
        biased, mant = qexp + 1075, n - (1 << 52)
    if biased >= 0x7FF:
        raise CheckFailure("value overflows binary64")
    bits = (sign << 63) | (biased << 52) | mant
    value = Fraction(n) * pow2(qexp)
    return bits, (-value if sign else value)


def hex64(bits):
    return "%016x" % bits


def decimal40(x):
    """Forty significant digits, scientific notation, nearest ties to even."""
    if x == 0:
        return "0"
    neg = x < 0
    a = -x if neg else x
    k = len(str(a.numerator)) - len(str(a.denominator))
    while Fraction(10) ** k > a:
        k -= 1
    while Fraction(10) ** (k + 1) <= a:
        k += 1
    n = round_half_even(a * Fraction(10) ** (39 - k))
    if n == 10 ** 40:
        n = 10 ** 39
        k += 1
    d = str(n)
    require(len(d) == 40, "decimal40 digit count")
    exp = ("e+%02d" % k) if k >= 0 else ("e-%02d" % -k)
    return ("-" if neg else "") + d[0] + "." + d[1:] + exp


def encode(x):
    hi_bits, hi = to_binary64(x)
    rem = x - hi
    lo_bits, _ = to_binary64(rem)
    return {"decimal": decimal40(x), "hi": hex64(hi_bits), "lo": hex64(lo_bits)}


MAX_FINITE = Fraction((1 << 53) - 1) * pow2(971)   # largest finite binary64
MIN_SUBNORMAL = pow2(-1074)                        # smallest positive subnormal


def encode_phi(x):
    """ADDENDUM-1 rule: a shear parameter outside the binary64 range (above the
    largest finite value or below the smallest positive subnormal) is given as
    its decimal and its exact rational in lowest terms, with no (hi, lo) pair."""
    a = -x if x < 0 else x
    if x != 0 and (a > MAX_FINITE or a < MIN_SUBNORMAL):
        return {"decimal": decimal40(x),
                "numerator": str(x.numerator),       # Fraction: lowest terms,
                "denominator": str(x.denominator)}   # positive denominator
    # In range: hi is x rounded to nearest-even; lo is the exact residual
    # rounded to nearest-even. A nonzero residual that rounds to zero gives the
    # zero with the residual's sign; an exactly zero residual gives +0.
    hi_bits, hi = to_binary64(x)
    rem = x - hi
    lo_bits = 0 if rem == 0 else to_binary64(rem, signed_zero_underflow=True)[0]
    return {"decimal": decimal40(x), "hi": hex64(hi_bits), "lo": hex64(lo_bits)}


# --------------------------------------------------------------------------
# exact square root, vectors, matrices
# --------------------------------------------------------------------------

def exact_sqrt(x, what):
    require(x >= 0, "negative radicand for %s" % what)
    n, d = x.numerator, x.denominator  # Fraction keeps these coprime
    rn, rd = isqrt(n), isqrt(d)
    if rn * rn != n or rd * rd != d:
        raise CheckFailure("square root of %s is not rational" % what)
    return Fraction(rn, rd)


def dot(u, v):
    return sum((a * b for a, b in zip(u, v)), ZERO)


def cross(u, v):
    return [u[1] * v[2] - u[2] * v[1],
            u[2] * v[0] - u[0] * v[2],
            u[0] * v[1] - u[1] * v[0]]


def zeros(r, c):
    return [[ZERO] * c for _ in range(r)]


def matmul(A, B):
    n, k, m = len(A), len(B), len(B[0])
    C = zeros(n, m)
    for i in range(n):
        Ai = A[i]
        for p in range(k):
            a = Ai[p]
            if a:
                Bp = B[p]
                Ci = C[i]
                for j in range(m):
                    Ci[j] += a * Bp[j]
    return C


def matvec(A, x):
    return [dot(row, x) for row in A]


def transpose(A):
    return [list(r) for r in zip(*A)]


def is_symmetric(A):
    n = len(A)
    return all(A[i][j] == A[j][i] for i in range(n) for j in range(n))


def solve(A, b):
    """Exact Gauss-Jordan solve of a square rational system."""
    n = len(A)
    M = [list(A[i]) + [b[i]] for i in range(n)]
    for col in range(n):
        piv = next((r for r in range(col, n) if M[r][col] != 0), None)
        require(piv is not None, "singular linear system")
        M[col], M[piv] = M[piv], M[col]
        p = M[col][col]
        M[col] = [v / p for v in M[col]]
        for r in range(n):
            if r != col and M[r][col] != 0:
                f = M[r][col]
                M[r] = [vr - f * vc for vr, vc in zip(M[r], M[col])]
    return [M[i][n] for i in range(n)]


def ldlt_pivots(A):
    """Pivots of the exact symmetric elimination A = L D L^T (no pivoting)."""
    n = len(A)
    M = [list(r) for r in A]
    pivots = []
    for k in range(n):
        p = M[k][k]
        pivots.append(p)
        if p <= 0:
            return pivots  # not positive definite; caller checks
        for i in range(k + 1, n):
            f = M[i][k] / p
            if f:
                for j in range(k + 1, n):
                    M[i][j] -= f * M[k][j]
    return pivots


# --------------------------------------------------------------------------
# exact polynomials in one variable: coefficient lists, lowest power first
# --------------------------------------------------------------------------

def p_trim(p):
    p = list(p)
    while p and p[-1] == 0:
        p.pop()
    return p


def p_add(p, q):
    n = max(len(p), len(q))
    return p_trim([(p[i] if i < len(p) else ZERO) + (q[i] if i < len(q) else ZERO)
                   for i in range(n)])


def p_scale(p, c):
    return p_trim([c * a for a in p])


def p_sub(p, q):
    return p_add(p, p_scale(q, -ONE))


def p_mul(p, q):
    if not p or not q:
        return []
    r = [ZERO] * (len(p) + len(q) - 1)
    for i, a in enumerate(p):
        for j, b in enumerate(q):
            r[i + j] += a * b
    return p_trim(r)


def p_deriv(p):
    return p_trim([i * p[i] for i in range(1, len(p))])


def p_antideriv(p):
    """Antiderivative vanishing at 0."""
    return p_trim([ZERO] + [p[i] / (i + 1) for i in range(len(p))])


def p_eval(p, x):
    r = ZERO
    for a in reversed(p):
        r = r * x + a
    return r


def p_integral(p, a, b):
    P = p_antideriv(p)
    return p_eval(P, b) - p_eval(P, a)


def p_is_zero(p):
    return not p_trim(p)


# --------------------------------------------------------------------------
# Timoshenko plane: shapes from the homogeneous equations
# --------------------------------------------------------------------------
#
# Equilibrium with no distributed load: Q = G A_s (w' - theta) is constant and
# EI theta'' + Q = 0.  Hence theta'' is constant: theta = c1 + c2 s + c3 s^2,
# Q = -2 EI c3, and w' = theta + Q / (G A_s) = theta - 2 beta c3 with
# beta = EI / (G A_s) = Phi L^2 / 12.  Integrating,
#   w = c0 + c1 s + c2 s^2 / 2 + c3 (s^3 / 3 - 2 beta s).
# The four basis solutions (w, theta) below span the homogeneous solution;
# the shapes follow by imposing the four nodal values.

def timoshenko_basis(beta):
    third = Fraction(1, 3)
    half = Fraction(1, 2)
    return [
        ([ONE], []),                          # c0
        ([ZERO, ONE], [ONE]),                 # c1
        ([ZERO, ZERO, half], [ZERO, ONE]),    # c2
        ([ZERO, -2 * beta, ZERO, third], [ZERO, ZERO, ONE]),  # c3
    ]


def nodal_values(w, th, L):
    return [p_eval(w, ZERO), p_eval(th, ZERO), p_eval(w, L), p_eval(th, L)]


def timoshenko_shapes(L, beta):
    """Shape functions (N_w,k, N_theta,k), k = 0..3, on (w_i, th_i, w_j, th_j)."""
    basis = timoshenko_basis(beta)
    # A[r][c]: nodal value r of basis solution c.
    cols = [nodal_values(w, th, L) for (w, th) in basis]
    A = [[cols[c][r] for c in range(4)] for r in range(4)]
    shapes = []
    for k in range(4):
        e = [ONE if r == k else ZERO for r in range(4)]
        c = solve(A, e)
        w, th = [], []
        for ci, (bw, bth) in zip(c, basis):
            w = p_add(w, p_scale(bw, ci))
            th = p_add(th, p_scale(bth, ci))
        shapes.append((w, th))
    return shapes


def plane_mass(shapes, L, mbar, ibar):
    M = zeros(4, 4)
    for a in range(4):
        for b in range(4):
            wa, ta = shapes[a]
            wb, tb = shapes[b]
            M[a][b] = (mbar * p_integral(p_mul(wa, wb), ZERO, L)
                       + ibar * p_integral(p_mul(ta, tb), ZERO, L))
    return M


def plane_stiffness(shapes, L, EI, GA):
    """Stiffness from (1/2) int EI theta'^2 + (1/2) int G A_s (w' - theta)^2."""
    K = zeros(4, 4)
    curv = [p_deriv(th) for (w, th) in shapes]
    shear = [p_sub(p_deriv(w), th) for (w, th) in shapes]
    for a in range(4):
        for b in range(4):
            K[a][b] = (EI * p_integral(p_mul(curv[a], curv[b]), ZERO, L)
                       + GA * p_integral(p_mul(shear[a], shear[b]), ZERO, L))
    return K


def check_plane_shapes(shapes, L, EI, GA, label):
    for k, (w, th) in enumerate(shapes):
        Q = p_scale(p_sub(p_deriv(w), th), GA)
        eq = p_add(p_scale(p_deriv(p_deriv(th)), EI), Q)
        require(p_is_zero(eq), "%s shape %d: EI th'' + G A_s (w' - th) != 0" % (label, k))
        require(p_is_zero(p_deriv(Q)), "%s shape %d: shear force not constant" % (label, k))
        want = [ONE if r == k else ZERO for r in range(4)]
        require(nodal_values(w, th, L) == want, "%s shape %d: nodal conditions" % (label, k))


def check_plane_stiffness(shapes, L, EI, GA, label):
    K = plane_stiffness(shapes, L, EI, GA)
    require(is_symmetric(K), "%s stiffness not symmetric" % label)
    translation = [ONE, ZERO, ONE, ZERO]          # w = 1, theta = 0
    rotation = [ZERO, ONE, L, ONE]                # w = s, theta = 1
    require(all(v == 0 for v in matvec(K, translation)),
            "%s stiffness does not annihilate translation" % label)
    require(all(v == 0 for v in matvec(K, rotation)),
            "%s stiffness does not annihilate rigid rotation" % label)


# --------------------------------------------------------------------------
# Independent Euler-Bernoulli plane mass (cubic interpolation, theta = w')
# --------------------------------------------------------------------------
#
# A separate path: natural coordinate xi = s / L on [0, 1], monomial cubic
# basis, theta = dw/ds = (1/L) dw/dxi, ds = L dxi.

def euler_bernoulli_plane_mass(L, mbar, ibar):
    def nodal(coeffs):
        w = coeffs
        dw = p_deriv(w)
        return [p_eval(w, ZERO), p_eval(dw, ZERO) / L, p_eval(w, ONE), p_eval(dw, ONE) / L]
    monomials = [[ONE if i == j else ZERO for i in range(j + 1)] for j in range(4)]
    cols = [nodal(m) for m in monomials]
    A = [[cols[c][r] for c in range(4)] for r in range(4)]
    H = []
    for k in range(4):
        e = [ONE if r == k else ZERO for r in range(4)]
        c = solve(A, e)
        H.append(p_trim(c))
    for k in range(4):
        require(nodal(H[k]) == [ONE if r == k else ZERO for r in range(4)],
                "Euler-Bernoulli Hermite nodal conditions")
    M = zeros(4, 4)
    for a in range(4):
        for b in range(4):
            ww = p_integral(p_mul(H[a], H[b]), ZERO, ONE) * L
            tt = p_integral(p_mul(p_deriv(H[a]), p_deriv(H[b])), ZERO, ONE) / L
            M[a][b] = mbar * ww + ibar * tt
    return M


# --------------------------------------------------------------------------
# Independent consistent load vector of a uniform transverse load
# --------------------------------------------------------------------------
#
# Clamped Timoshenko beam (w = theta = 0 at both ends) under uniform load q:
#   (G A_s (w' - theta))' = -q,   EI theta'' + G A_s (w' - theta) = 0.
# So Q(s) = Q0 - q s, theta = t1 s - Q0 s^2/(2 EI) + q s^3/(6 EI) (theta(0)=0),
# w = int_0^s (theta + Q / (G A_s)) (w(0)=0); theta(L) = w(L) = 0 fix t1, Q0.
# Integrating the energy bilinear form by parts on both arguments, with the
# clamped solution vanishing at the nodes and each exact shape homogeneous,
# gives int q N_w,a ds = [Q(0), EI theta'(0), -Q(L), -EI theta'(L)]_a.
# This uses no shape function.

def clamped_load_vector(L, EI, GA, q):
    def fields(t1, Q0):
        Q = [Q0, -q]
        th = [ZERO, t1, -Q0 / (2 * EI), q / (6 * EI)]
        w = p_antideriv(p_add(th, p_scale(Q, ONE / GA)))
        return p_trim(Q), p_trim(th), w
    # theta(L) and w(L) are linear in (t1, Q0); build the 2x2 system exactly.
    _, th00, w00 = fields(ZERO, ZERO)
    _, th10, w10 = fields(ONE, ZERO)
    _, th01, w01 = fields(ZERO, ONE)
    r0 = [-p_eval(th00, L), -p_eval(w00, L)]
    A = [[p_eval(th10, L) - p_eval(th00, L), p_eval(th01, L) - p_eval(th00, L)],
         [p_eval(w10, L) - p_eval(w00, L), p_eval(w01, L) - p_eval(w00, L)]]
    t1, Q0 = solve(A, r0)
    Q, th, w = fields(t1, Q0)
    # Self-consistency of the clamped solution.
    require(p_is_zero(p_add(p_deriv(p_scale(p_sub(p_deriv(w), th), GA)), [q])),
            "clamped solution: shear equilibrium")
    require(p_is_zero(p_add(p_scale(p_deriv(p_deriv(th)), EI),
                            p_scale(p_sub(p_deriv(w), th), GA))),
            "clamped solution: moment equilibrium")
    require(nodal_values(w, th, L) == [ZERO] * 4, "clamped solution: end conditions")
    dth = p_deriv(th)
    return [p_eval(Q, ZERO), EI * p_eval(dth, ZERO), -p_eval(Q, L), -EI * p_eval(dth, L)]


def linear_load_vector(L, q):
    """int N q ds for linear interpolation N1 = 1 - s/L, N2 = s/L."""
    N1 = [ONE, -ONE / L]
    N2 = [ZERO, ONE / L]
    return [q * p_integral(N1, ZERO, L), q * p_integral(N2, ZERO, L)]


def linear_mass(L, rho):
    N = [[ONE, -ONE / L], [ZERO, ONE / L]]
    return [[rho * p_integral(p_mul(N[a], N[b]), ZERO, L) for b in range(2)] for a in range(2)]


# --------------------------------------------------------------------------
# The member
# --------------------------------------------------------------------------

DOF_AXIAL = (0, 6)
DOF_TORSION = (3, 9)
DOF_XY = (1, 5, 7, 11)   # u_y,i  r_z,i  u_y,j  r_z,j
DOF_XZ = (2, 4, 8, 10)   # u_z,i  r_y,i  u_z,j  r_y,j
SIGN_XZ = (1, -1, 1, -1)


def member_axes(xi, xj, v):
    d = [b - a for a, b in zip(xi, xj)]
    L = exact_sqrt(dot(d, d), "L^2")
    require(L > 0, "zero-length member")
    xhat = [c / L for c in d]
    vn = [a - dot(v, xhat) * b for a, b in zip(v, xhat)]
    nv = exact_sqrt(dot(vn, vn), "|v normal to x|^2")
    require(nv > 0, "reference vector parallel to member")
    yhat = [c / nv for c in vn]
    zhat = cross(xhat, yhat)
    R = [xhat, yhat, zhat]
    RRt = matmul(R, transpose(R))
    require(RRt == [[ONE if i == j else ZERO for j in range(3)] for i in range(3)],
            "member axes not orthonormal")
    return L, R


def build_case(case):
    ops = {k: (list(map(bits_to_fraction, val)) if isinstance(val, list)
               else bits_to_fraction(val))
           for k, val in case["operands"].items()}
    E, G, As = ops["E"], ops["G"], ops["A_s"]
    Iy, Iz = ops["I_y"], ops["I_z"]
    mbar, ibar, jbar = ops["mbar"], ops["ibar"], ops["jbar"]
    for name in ("E", "G", "A_s", "I_y", "I_z"):
        require(ops[name] > 0, "%s must be positive" % name)
    for name in ("mbar", "ibar", "jbar"):
        require(ops[name] > 0, "%s must be positive" % name)

    L, R = member_axes(ops["x_i"], ops["x_j"], ops["v"])
    GA = G * As
    Phi_y = 12 * E * Iy / (GA * L * L)
    Phi_z = 12 * E * Iz / (GA * L * L)
    checks = []

    planes = {}
    for label, EI, Phi in (("x-y plane", E * Iz, Phi_z), ("x-z plane", E * Iy, Phi_y)):
        beta = EI / GA
        require(beta == Phi * L * L / 12, "beta = Phi L^2 / 12")
        shapes = timoshenko_shapes(L, beta)
        check_plane_shapes(shapes, L, EI, GA, label)
        check_plane_stiffness(shapes, L, EI, GA, label)
        Mp = plane_mass(shapes, L, mbar, ibar)
        require(is_symmetric(Mp), "%s mass not symmetric" % label)
        planes[label] = (Mp, EI)
    checks.append("shapes satisfy equilibrium and nodal conditions (both planes)")
    checks.append("strain-energy stiffness symmetric, annihilates translation and rigid rotation (both planes)")

    # Euler-Bernoulli limit: the Timoshenko construction at Phi = 0 against an
    # independent cubic Hermite derivation.
    shapes0 = timoshenko_shapes(L, ZERO)
    for k, (w, th) in enumerate(shapes0):
        require(p_deriv(w) == th, "Phi = 0 shape %d: theta != w'" % k)
    M0 = plane_mass(shapes0, L, mbar, ibar)
    Meb = euler_bernoulli_plane_mass(L, mbar, ibar)
    require(M0 == Meb, "Phi = 0 plane mass differs from Euler-Bernoulli cubic mass")
    checks.append("plane mass at Phi = 0 equals independent Euler-Bernoulli cubic mass")

    # Local 12x12.
    Ml = zeros(12, 12)
    for dofs, blk in ((DOF_AXIAL, linear_mass(L, mbar)), (DOF_TORSION, linear_mass(L, jbar))):
        for a in range(2):
            for b in range(2):
                Ml[dofs[a]][dofs[b]] = blk[a][b]
    Mxy = planes["x-y plane"][0]
    Mxz = planes["x-z plane"][0]
    for a in range(4):
        for b in range(4):
            Ml[DOF_XY[a]][DOF_XY[b]] = Mxy[a][b]
            Ml[DOF_XZ[a]][DOF_XZ[b]] = SIGN_XZ[a] * SIGN_XZ[b] * Mxz[a][b]

    # Global: T^T M T, T = diag(R, R, R, R).
    T = zeros(12, 12)
    for blk in range(4):
        for i in range(3):
            for j in range(3):
                T[3 * blk + i][3 * blk + j] = R[i][j]
    Tt = transpose(T)
    Mg = matmul(matmul(Tt, Ml), T)

    # Rigid translations against independently derived uniform-load vectors.
    for k in range(3):
        qloc = [R[0][k], R[1][k], R[2][k]]  # local components of global e_k
        F = [ZERO] * 12
        fa = linear_load_vector(L, qloc[0])
        F[DOF_AXIAL[0]], F[DOF_AXIAL[1]] = fa
        fxy = clamped_load_vector(L, planes["x-y plane"][1], GA, qloc[1])
        fxz = clamped_load_vector(L, planes["x-z plane"][1], GA, qloc[2])
        for a in range(4):
            F[DOF_XY[a]] += fxy[a]
            F[DOF_XZ[a]] += SIGN_XZ[a] * fxz[a]
        Fg = matvec(Tt, F)
        r = [ZERO] * 12
        r[k] = ONE
        r[6 + k] = ONE
        Mr = matvec(Mg, r)
        require(Mr == [mbar * f for f in Fg],
                "M r_%s != mbar * consistent uniform-load vector" % "xyz"[k])
        require(dot(r, Mr) == mbar * L, "r_%s^T M r_%s != mbar L" % ("xyz"[k], "xyz"[k]))
    checks.append("M r_k equals mbar times independent uniform-load vector, r_k^T M r_k = mbar L (k = x, y, z)")

    require(is_symmetric(Mg), "global mass not symmetric")
    piv = ldlt_pivots(Mg)
    require(len(piv) == 12 and all(p > 0 for p in piv), "global mass not positive definite")
    checks.append("global mass symmetric and positive definite (all 12 exact LDL^T pivots > 0)")

    def plane_entries(M):
        return [dict(a=a, b=b, **encode(M[a][b])) for a in range(4) for b in range(4)]

    upper = []
    for r_ in range(12):
        for c in range(r_, 12):
            if Mg[r_][c] != 0:
                upper.append(dict(r=r_, c=c, **encode(Mg[r_][c])))

    return {
        "id": case["id"],
        "operands": case["operands"],
        "L": encode(L),
        "Phi_y": encode_phi(Phi_y),
        "Phi_z": encode_phi(Phi_z),
        "plane_xy": plane_entries(Mxy),
        "plane_xz": plane_entries(Mxz),
        "global_upper_nonzero": upper,
        "self_checks_passed": checks,
    }


CASES = [
    {"id": "T-AXIS-SLENDER", "operands": {
        "x_i": ["0000000000000000", "0000000000000000", "0000000000000000"],
        "x_j": ["4008000000000000", "0000000000000000", "0000000000000000"],
        "v": ["0000000000000000", "3ff0000000000000", "0000000000000000"],
        "E": "4247a1df17000000", "G": "423229298c000000", "A_s": "3f76b11c6d1e108c",
        "I_y": "3f20d6bce5a40934", "I_z": "3f20d6bce5a40934",
        "mbar": "4054600000000000", "ibar": "3ff08f5c28f5c28f", "jbar": "40008f5c28f5c28f"}},
    {"id": "T-SKEW-STUBBY", "operands": {
        "x_i": ["3ff0000000000000", "4000000000000000", "4008000000000000"],
        "x_j": ["3ff3000000000000", "4002000000000000", "400e000000000000"],
        "v": ["4050c00000000000", "c033000000000000", "404e000000000000"],
        "E": "4247a1df17000000", "G": "423229298c000000", "A_s": "3f76b11c6d1e108c",
        "I_y": "3f20d6bce5a40934", "I_z": "3f20d6bce5a40934",
        "mbar": "4054600000000000", "ibar": "3ff08f5c28f5c28f", "jbar": "40008f5c28f5c28f"}},
    {"id": "T-SKEW-UNEQUAL", "operands": {
        "x_i": ["0000000000000000", "0000000000000000", "0000000000000000"],
        "x_j": ["3fd0000000000000", "3fd8000000000000", "3fe8000000000000"],
        "v": ["4018000000000000", "4000000000000000", "c008000000000000"],
        "E": "42474876e8000000", "G": "4232a05f20000000", "A_s": "3f3a36e2eb1c432d",
        "I_y": "3f04f8b588e368f1", "I_z": "3ee4f8b588e368f1",
        "mbar": "4049000000000000", "ibar": "3fd0000000000000", "jbar": "3fe0000000000000"}},
    {"id": "T-AXIS-NEAR-EB", "operands": {
        "x_i": ["0000000000000000", "0000000000000000", "0000000000000000"],
        "x_j": ["0000000000000000", "0000000000000000", "4000000000000000"],
        "v": ["3ff0000000000000", "0000000000000000", "0000000000000000"],
        "E": "4247a1df17000000", "G": "423229298c000000", "A_s": "41f4a35b10000000",
        "I_y": "3f20d6bce5a40934", "I_z": "3f20d6bce5a40934",
        "mbar": "4054600000000000", "ibar": "3ff08f5c28f5c28f", "jbar": "40008f5c28f5c28f"}},
    {"id": "T-RANGE-TINY-MASS", "operands": {
        "x_i": ["0000000000000000", "0000000000000000", "0000000000000000"],
        "x_j": ["3ff0000000000000", "0000000000000000", "0000000000000000"],
        "v": ["0000000000000000", "3ff0000000000000", "0000000000000000"],
        "E": "3ff0000000000000", "G": "3ff0000000000000", "A_s": "2728000000000000",
        "I_y": "3ff0000000000000", "I_z": "3ff0000000000000",
        "mbar": "1430000000000000", "ibar": "1430000000000000", "jbar": "1430000000000000"}},
]

ADDENDUM1_CASES = [
    {"id": "T-RANGE-SUBFLOOR", "operands": {
        "x_i": ["0000000000000000", "0000000000000000", "0000000000000000"],
        "x_j": ["3ff0000000000000", "0000000000000000", "0000000000000000"],
        "v": ["0000000000000000", "3ff0000000000000", "0000000000000000"],
        "E": "3ff0000000000000", "G": "3ff0000000000000", "A_s": "4028000000000000",
        "I_y": "3ff0000000000000", "I_z": "3ff0000000000000",
        "mbar": "022a400000000000", "ibar": "01de000000000000", "jbar": "01de000000000000"}},
    {"id": "T-RANGE-PHI-HUGE", "operands": {
        "x_i": ["0000000000000000", "0000000000000000", "0000000000000000"],
        "x_j": ["3ff0000000000000", "0000000000000000", "0000000000000000"],
        "v": ["0000000000000000", "3ff0000000000000", "0000000000000000"],
        "E": "7e70000000000000", "G": "3ff0000000000000", "A_s": "4028000000000000",
        "I_y": "7e70000000000000", "I_z": "7e70000000000000",
        "mbar": "3ff0000000000000", "ibar": "3ff0000000000000", "jbar": "3ff0000000000000"}},
    {"id": "T-RANGE-PHI-TINY", "operands": {
        "x_i": ["0000000000000000", "0000000000000000", "0000000000000000"],
        "x_j": ["3ff0000000000000", "0000000000000000", "0000000000000000"],
        "v": ["0000000000000000", "3ff0000000000000", "0000000000000000"],
        "E": "0170000000000000", "G": "3ff0000000000000", "A_s": "4028000000000000",
        "I_y": "0170000000000000", "I_z": "0170000000000000",
        "mbar": "3ff0000000000000", "ibar": "3ff0000000000000", "jbar": "3ff0000000000000"}},
]


def self_test_primitives():
    """Exact checks of the encoders on values whose answers are known by construction."""
    for h in ("3ff0000000000000", "c033000000000000", "0000000000000001",
              "000fffffffffffff", "7fefffffffffffff", "1430000000000000", "3fd8000000000000"):
        require(hex64(to_binary64(bits_to_fraction(h))[0]) == h, "binary64 round trip %s" % h)
    # 1 + 2^-53 is a tie between 1 and 1 + 2^-52: even mantissa wins (1).
    require(to_binary64(1 + pow2(-53))[0] == 0x3FF0000000000000, "tie to even (down)")
    require(to_binary64(1 + 3 * pow2(-53))[0] == 0x3FF0000000000002, "tie to even (up)")
    require(decimal40(Fraction(1, 3)) == "3.333333333333333333333333333333333333333e-01", "decimal 1/3")
    require(decimal40(Fraction(-2, 3)) == "-6.666666666666666666666666666666666666667e-01", "decimal -2/3")
    require(decimal40(Fraction(10) ** 40 - Fraction(1, 2)) == "1.000000000000000000000000000000000000000e+40",
            "decimal carry")
    # Exactly halfway at the 40th digit: ...25 -> ...2 (even), ...35 -> ...4.
    require(decimal40(Fraction(10 ** 40 + 25, 10)) == "1.000000000000000000000000000000000000002e+39",
            "decimal tie to even (down)")
    require(decimal40(Fraction(10 ** 40 + 35, 10)) == "1.000000000000000000000000000000000000004e+39",
            "decimal tie to even (up)")
    # ADDENDUM-1 range rule: the boundaries themselves are in range.
    require("hi" in encode_phi(MAX_FINITE) and "hi" in encode_phi(MIN_SUBNORMAL), "Phi range bounds")
    require("numerator" in encode_phi(MAX_FINITE + MIN_SUBNORMAL), "Phi above largest finite")
    require("numerator" in encode_phi(MIN_SUBNORMAL / 2), "Phi below smallest subnormal")
    # In-range values whose nonzero residual rounds to a (signed) zero.
    mu = MIN_SUBNORMAL
    for x, hi_want, lo_want in (
            (mu + pow2(-2000), "0000000000000001", "0000000000000000"),
            (5 * mu / 4, "0000000000000001", "0000000000000000"),
            (3 * mu / 2, "0000000000000002", "8000000000000000"),
            (MAX_FINITE - mu / 4, "7fefffffffffffff", "8000000000000000")):
        got = encode_phi(x)
        require((got.get("hi"), got.get("lo")) == (hi_want, lo_want),
                "Phi pair for in-range value %s: %r" % (decimal40(x), got))
    # The same 5 mu / 4 through the member interface from finite operands.
    E_bits, E = to_binary64(pow2(-500))
    I_bits, I = to_binary64(Fraction(5, 4) * pow2(-574))
    require(hex64(E_bits) == "20b0000000000000" and E == pow2(-500), "member E operand")
    require(hex64(I_bits) == "1c14000000000000" and I == Fraction(5, 4) * pow2(-574), "member I operand")
    member = build_case({"id": "SELF-TEST-PHI-5MU/4", "operands": {
        "x_i": ["0000000000000000", "0000000000000000", "0000000000000000"],
        "x_j": ["3ff0000000000000", "0000000000000000", "0000000000000000"],
        "v": ["0000000000000000", "3ff0000000000000", "0000000000000000"],
        "E": hex64(E_bits), "G": "3ff0000000000000", "A_s": "4028000000000000",
        "I_y": hex64(I_bits), "I_z": hex64(I_bits),
        "mbar": "3ff0000000000000", "ibar": "3ff0000000000000", "jbar": "3ff0000000000000"}})
    for key in ("Phi_y", "Phi_z"):
        require((member[key].get("hi"), member[key].get("lo")) == ("0000000000000001", "0000000000000000"),
                "member-case %s pair: %r" % (key, member[key]))


def build():
    self_test_primitives()
    doc = {
        "format": FORMAT_ID,
        "problem_sha256": PROBLEM_SHA256,
        "cases": [build_case(c) for c in CASES],
    }
    return (json.dumps(doc, indent=1, sort_keys=False, ensure_ascii=True) + "\n").encode("ascii")


def build_addendum1():
    doc = {
        "format": ADDENDUM1_FORMAT_ID,
        "problem_sha256": PROBLEM_SHA256,
        "addendum_sha256": ADDENDUM1_SHA256,
        "cases": [build_case(c) for c in ADDENDUM1_CASES],
    }
    return (json.dumps(doc, indent=1, sort_keys=False, ensure_ascii=True) + "\n").encode("ascii")


def main(argv=None):
    here = os.path.dirname(os.path.abspath(__file__))
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--write", action="store_true", help="write the output files")
    ap.add_argument("--output", default=os.path.join(here, OUTPUT_NAME))
    ap.add_argument("--output-addendum1", default=os.path.join(here, ADDENDUM1_OUTPUT_NAME))
    args = ap.parse_args(argv)
    outputs = [(args.output, build()), (args.output_addendum1, build_addendum1())]
    if args.write:
        for path, data in outputs:
            with open(path, "wb") as f:
                f.write(data)
            print("wrote %s (%d bytes)" % (path, len(data)))
        return 0
    rc = 0
    for path, data in outputs:
        try:
            with open(path, "rb") as f:
                existing = f.read()
        except FileNotFoundError:
            print("missing %s" % path, file=sys.stderr)
            rc = 1
            continue
        if existing != data:
            print("MISMATCH: %s differs from the rebuilt output" % path, file=sys.stderr)
            rc = 1
            continue
        print("ok: %s matches the rebuilt output (%d bytes)" % (path, len(data)))
    return rc


if __name__ == "__main__":
    sys.exit(main())
