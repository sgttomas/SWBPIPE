"""Continuum beam engine: Euler-Bernoulli and Rayleigh beams in one bending plane.

Nondimensional form, x in [0, 1] (x = X/L):
  W'''' + r2 Om^2 W'' - Om^2 W = 0,  Om^2 = w^2 mbar L^4/EI,  r2 = ibar/(mbar L^2)
(r2 = 0 is Euler-Bernoulli). With W = e^{sx}: s^4 + r2 Om^2 s^2 - Om^2 = 0, so
s^2 = a^2 or -b^2 with
  a^2 = (-r2 Om^2 + sqrt(r2^2 Om^4 + 4 Om^2))/2,  b^2 = (r2 Om^2 + sqrt(...))/2,
and W = c1 cosh(ax) + c2 sinh(ax) + c3 cos(bx) + c4 sin(bx).

Boundary conditions (from the variational form, see README):
  clamped  W = 0, W' = 0
  pinned   W = 0, W'' = 0
  free     W'' = 0, W''' + r2 Om^2 W' = 0
  spring   W'' = 0, W''' + r2 Om^2 W' - kappa W = 0   (tip spring, kappa = k L^3/EI; x = 1 only)
Frequencies are roots of the 4x4 determinant of these rows (rows at x = 1
divided by cosh a, which does not move a root); the shape is the null vector
from the cofactors of the best-conditioned 3x4 submatrix.

Every integral used for participation is evaluated in closed form from the
antiderivatives of products of cosh, sinh, cos, sin; Gauss-Legendre
quadrature recomputes them as a cross-check.
"""

import math
from decimal import Decimal
from fractions import Fraction

from numerics import (bisect, dcos, dcosh, dec, det, dsin, dsinh, dsqrt, integrate, pi,
                      scan_brackets)


# --------------------------------------------------------------------------
# characteristic exponents


def ab(Om, r2, lib):
    if lib == "f":
        d = math.sqrt(r2 * r2 * Om ** 4 + 4 * Om * Om)
        return math.sqrt((-r2 * Om * Om + d) / 2), math.sqrt((r2 * Om * Om + d) / 2)
    d = dsqrt(r2 * r2 * Om ** 4 + 4 * Om * Om)
    return dsqrt((-r2 * Om * Om + d) / 2), dsqrt((r2 * Om * Om + d) / 2)


def basis(x, a, b, k, lib):
    """k-th derivative of [cosh ax, sinh ax, cos bx, sin bx] at x."""
    if lib == "f":
        ch, sh, c, s = math.cosh(a * x), math.sinh(a * x), math.cos(b * x), math.sin(b * x)
    else:
        ax, bx = a * x, b * x
        ch, sh, c, s = dcosh(ax), dsinh(ax), dcos(bx), dsin(bx)
    hyp = [(ch, sh), (sh, ch)][k % 2]
    trig = [(c, s), (-s, c), (-c, -s), (s, -c)][k % 4]
    return [a ** k * hyp[0], a ** k * hyp[1], b ** k * trig[0], b ** k * trig[1]]


def bc_rows(kind, x, Om, r2, kappa, a, b, lib):
    B = [basis(x, a, b, k, lib) for k in range(4)]
    if kind == "clamped":
        rows = [B[0], B[1]]
    elif kind == "pinned":
        rows = [B[0], B[2]]
    elif kind == "free":
        rows = [B[2], [B[3][i] + r2 * Om * Om * B[1][i] for i in range(4)]]
    elif kind == "spring":
        rows = [B[2], [B[3][i] + r2 * Om * Om * B[1][i] - kappa * B[0][i] for i in range(4)]]
    else:
        raise ValueError(kind)
    if x != 0:
        ch = math.cosh(a) if lib == "f" else dcosh(a)
        rows = [[v / ch for v in r] for r in rows]
    return rows


def bc_matrix(left, right, Om, r2, kappa, lib):
    a, b = ab(Om, r2, lib)
    zero, one = (0.0, 1.0) if lib == "f" else (Decimal(0), Decimal(1))
    return bc_rows(left, zero, Om, r2, kappa, a, b, lib) + bc_rows(right, one, Om, r2, kappa, a, b, lib)


def char_det(left, right, Om, r2, kappa, lib):
    return det(bc_matrix(left, right, Om, r2, kappa, lib))


def roots(left, right, r2, kappa, count, start=0.5, step=0.05):
    r2f, kf = float(r2), float(kappa)
    br = scan_brackets(lambda Om: char_det(left, right, Om, r2f, kf, "f"), start, 2000.0, step, count)
    r2d, kd = dec(r2), dec(kappa)
    return [bisect(lambda Om: char_det(left, right, Om, r2d, kd, "d"), dec(lo), dec(hi)) for lo, hi in br]


def minor3(A, drop_row, drop_col):
    return det([[A[i][j] for j in range(4) if j != drop_col] for i in range(4) if i != drop_row])


def null_vector(A):
    best = None
    for r in range(4):
        v = [(-1) ** j * minor3(A, r, j) for j in range(4)]
        n = max(abs(x) for x in v)
        if best is None or n > best[0]:
            best = (n, v)
    return best[1]


# --------------------------------------------------------------------------
# closed-form integrals over [0, 1]


def _prod_integrals(a, b):
    """I[i][j] = int_0^1 B_i B_j for B = [cosh ax, sinh ax, cos bx, sin bx]."""
    ch, sh = dcosh(a), dsinh(a)
    c, s = dcos(b), dsin(b)
    ch2, sh2 = dcosh(2 * a), dsinh(2 * a)
    c2, s2 = dcos(2 * b), dsin(2 * b)
    q = a * a + b * b
    I = [[None] * 4 for _ in range(4)]
    I[0][0] = sh2 / (4 * a) + Decimal(1) / 2
    I[1][1] = sh2 / (4 * a) - Decimal(1) / 2
    I[0][1] = I[1][0] = (ch2 - 1) / (4 * a)
    I[2][2] = Decimal(1) / 2 + s2 / (4 * b)
    I[3][3] = Decimal(1) / 2 - s2 / (4 * b)
    I[2][3] = I[3][2] = (1 - c2) / (4 * b)
    # hyperbolic x trigonometric, from the antiderivatives (README)
    I[0][2] = I[2][0] = (a * sh * c + b * ch * s) / q
    I[0][3] = I[3][0] = (a * sh * s - b * ch * c + b) / q
    I[1][2] = I[2][1] = (a * ch * c + b * sh * s - a) / q
    I[1][3] = I[3][1] = (a * ch * s - b * sh * c) / q
    return I


def _single_integrals(a, b):
    return [dsinh(a) / a, (dcosh(a) - 1) / a, dsin(b) / b, (1 - dcos(b)) / b]


def deriv_coef(c, a, b):
    """Coefficients of W' on the same basis."""
    return [a * c[1], a * c[0], b * c[3], -b * c[2]]


# --------------------------------------------------------------------------
# modes


class Mode:
    pass


def W(md, x, k=0):
    B = basis(dec(x), md.a, md.b, k, "d")
    return sum((ci * bi for ci, bi in zip(md.c, B)), Decimal(0))


def _max_abs_point(md):
    """x* in [0, 1] where |W| is largest, by a binary64 scan of W' and decimal bisection."""
    cf = [float(v) for v in md.c]
    af, bf = float(md.a), float(md.b)

    def wp(x):
        B = basis(x, af, bf, 1, "f")
        return sum(ci * bi for ci, bi in zip(cf, B))

    cands = [Decimal(0), Decimal(1)]
    n = 3989  # prime, so no grid point falls on a symmetric span's exact midpoint or quarter point
    for i in range(n):
        x0, x1 = i / n, (i + 1) / n
        if (wp(x0) < 0) != (wp(x1) < 0):
            try:
                cands.append(bisect(lambda x: W(md, x, 1), dec(x0), dec(x1),
                                    rel_tol=Decimal(10) ** -60))
            except ValueError:
                # binary64 noise at a clamped end, where W' = 0 exactly; the endpoint is a candidate already
                pass
    vals = [(abs(W(md, x)), x) for x in cands]
    big = max(v for v, _ in vals)
    near = sorted(x for v, x in vals if (big - v) <= big * Decimal(10) ** -40)
    # equal maxima (to 40 digits): the smallest x is taken as the normalizing point
    return near[0], near[1:]


def modes(left, right, r2, kappa, count, closed_pp=False):
    """Continuum modes, each normalized so the largest |W| is +1."""
    r2, kappa = dec(r2), dec(kappa)
    if closed_pp:
        P = pi()
        Oms = [dsqrt((n * P) ** 4 / (1 + r2 * (n * P) ** 2)) for n in range(1, count + 1)]
    else:
        Oms = roots(left, right, r2, kappa, count)
    out = []
    for n, Om in enumerate(Oms, start=1):
        md = Mode()
        md.n, md.Om = n, Om
        md.a, md.b = ab(Om, r2, "d")
        if closed_pp:
            md.c = [Decimal(0), Decimal(0), Decimal(0), Decimal(1)]
        else:
            md.c = null_vector(bc_matrix(left, right, Om, r2, kappa, "d"))
        xs, ties = _max_abs_point(md)
        scale = W(md, xs)
        md.c = [v / scale for v in md.c]
        md.xstar = xs
        md.ties = [t for t in ties if (W(md, t) < 0)]  # opposite-sign ties only
        I = _prod_integrals(md.a, md.b)
        S1 = _single_integrals(md.a, md.b)
        dc = deriv_coef(md.c, md.a, md.b)
        md.I1 = sum((ci * si for ci, si in zip(md.c, S1)), Decimal(0))
        md.I2 = sum((md.c[i] * md.c[j] * I[i][j] for i in range(4) for j in range(4)), Decimal(0))
        md.I3 = sum((dc[i] * dc[j] * I[i][j] for i in range(4) for j in range(4)), Decimal(0))
        md.r2, md.kappa = r2, kappa
        out.append(md)
    return out


def quad_check(md):
    """(I1, I2, I3, J1 = int x W) by Gauss-Legendre quadrature."""
    q1 = integrate(lambda x: W(md, x), 0, 1)
    q2 = integrate(lambda x: W(md, x) ** 2, 0, 1)
    q3 = integrate(lambda x: W(md, x, 1) ** 2, 0, 1)
    j1 = integrate(lambda x: x * W(md, x), 0, 1)
    return q1, q2, q3, j1


# --------------------------------------------------------------------------
# Euler-Bernoulli textbook frequency equations (independent route for r2 = 0)


def eb_equation_roots(kind, count, kappa=0):
    """beta L from the textbook equations, divided by cosh to stay bounded."""
    def fl(x):
        ch = math.cosh(x)
        if kind == "cantilever":
            return math.cos(x) + 1 / ch
        if kind == "fixed_fixed":
            return math.cos(x) - 1 / ch
        if kind == "propped":
            return math.sin(x) - math.cos(x) * math.tanh(x)
        if kind == "tip_spring":
            k = float(kappa)
            return x ** 3 * (1 / ch + math.cos(x)) - k * (math.cos(x) * math.tanh(x) - math.sin(x))

    def dl(x):
        ch = dcosh(x)
        if kind == "cantilever":
            return dcos(x) + 1 / ch
        if kind == "fixed_fixed":
            return dcos(x) - 1 / ch
        if kind == "propped":
            return dsin(x) - dcos(x) * dsinh(x) / ch
        if kind == "tip_spring":
            k = dec(kappa)
            return x ** 3 * (1 / ch + dcos(x)) - k * (dcos(x) * dsinh(x) / ch - dsin(x))

    br = scan_brackets(fl, 0.3, 60.0, 0.01, count)
    return [bisect(dl, dec(lo), dec(hi)) for lo, hi in br]


# --------------------------------------------------------------------------
# static uniform load (Euler-Bernoulli statics; rotary inertia has no static effect)


def static_uniform(left, right):
    """Exact polynomial w(x) with w'''' = 1 and the end conditions; coefficients c0..c4."""
    # w = x^4/24 + c3 x^3 + c2 x^2 + c1 x + c0
    def row(kind, x, order):
        # derivative of the unknown monomials [1, x, x^2, x^3] and of x^4/24
        mons = [[1, x, x ** 2, x ** 3], [0, 1, 2 * x, 3 * x ** 2], [0, 0, 2, 6 * x], [0, 0, 0, 6]][order]
        known = [Fraction(x ** 4, 24), Fraction(x ** 3, 6), Fraction(x ** 2, 2), Fraction(x)][order]
        return [Fraction(m) for m in mons], -known

    orders = {"clamped": (0, 1), "pinned": (0, 2), "free": (2, 3)}
    A, rhs = [], []
    for kind, x in ((left, 0), (right, 1)):
        for o in orders[kind]:
            r, k = row(kind, x, o)
            A.append(r)
            rhs.append(k)
    from numerics import solve
    c = solve(A, rhs)
    return c + [Fraction(1, 24)]


def poly_eval(c, x, order=0):
    x = Fraction(x)
    coefs = list(c)
    for _ in range(order):
        coefs = [i * coefs[i] for i in range(1, len(coefs))]
    return sum((ci * x ** i for i, ci in enumerate(coefs)), Fraction(0))
