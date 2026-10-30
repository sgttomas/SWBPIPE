#!/usr/bin/env python3
"""T4-U8 independent references: the Timoshenko straight member (SHEAR_REFERENCE).

Standard-library Python 3 only. This generator imports, reads and runs no
product code. Besides the files it writes, it reads two vetted inputs, each
bound by SHA-256 (the T4-I8 blocks by a canonical-JSON digest):
  - ../../t4_i8/rebuilt_reference_cases.json: T4-I8's frozen case-2 and twin-A
    documents and values, re-authored and cross-checked here;
  - core/product_physics/tests/fixtures/pressure_reference/SOURCE_ODWALL_EXPECTATIONS.json:
    twin A's binary64 inputs and its frozen Euler-Bernoulli values.

Physics source: SHEAR_REFERENCE at f2211ecef6 (CONTRACT.md sections 1-5,
INDEPENDENT_REFUTATION.md, reference_arithmetic.py and its stdout) and the
T4-U6/T4-U8 design section 3. Every expected value is derived here in exact
arithmetic over Q(pi): rational functions of pi with rational coefficients,
normalized so that equality is exact. The element is CONTRACT section 3's exact
static Timoshenko element; its load equivalents come from section 4's
integrated solution; the frame assembly and solve are this generator's own.

Modes:
  python3 -I u8_reference.py           check: rebuild every output in memory and
                                        compare byte for byte with the committed
                                        files, SHA256SUMS included
  python3 -I u8_reference.py --write   write the outputs (intentional changes only)
"""
import argparse
import copy
import hashlib
import json
import struct
import sys
from decimal import Decimal, getcontext
from fractions import Fraction as F
from math import isqrt
from pathlib import Path

HERE = Path(__file__).resolve().parent
OUT = HERE.parent
REFS = OUT.parent
PROJECT = REFS.parent.parent
T4_I8_PATH = REFS / "t4_i8" / "rebuilt_reference_cases.json"
ODWALL_PATH = (PROJECT / "core" / "product_physics" / "tests" / "fixtures" / "pressure_reference"
               / "SOURCE_ODWALL_EXPECTATIONS.json")
ODWALL_SHA256 = "7e51ecb3c7eca573194ce60222f2e2e7422b817be7aacad9a2a5e79102801ab1"
# Canonical JSON (sorted keys, compact) digests of the two T4-I8 case blocks consumed
# here (repair round 01, 5b07d85cf3; tracked copy at b3f4da7661).
I8_FILE_SHA256 = "838578ab059f310536afaac115c6e84e40ac53842800ef79030800432220724b"
I8_CASE2_SHA256 = "b801f427c9268eda6eb8bb6c9699e3f63f77a19bed1e141c202d1e6233ac2c68"
I8_TWIN_SHA256 = "45c65f4cf2184dfcd0ded2fd7d58650611ecf2553e5459ff1e77d653b9d96a34"

getcontext().prec = 140

# ===========================================================================
# Exact numbers: Q(pi) as normalized quotients of polynomials in pi.
# Polynomials are tuples of Fractions, lowest degree first.
# ===========================================================================


def ptrim(p):
    p = list(p)
    while p and p[-1] == 0:
        p.pop()
    return tuple(p)


def padd(a, b):
    n = max(len(a), len(b))
    return ptrim([(a[i] if i < len(a) else 0) + (b[i] if i < len(b) else 0) for i in range(n)])


def pmul(a, b):
    if not a or not b:
        return ()
    out = [F(0)] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        if x == 0:
            continue
        for j, y in enumerate(b):
            if y != 0:
                out[i + j] += x * y
    return ptrim(out)


def pdivmod(a, b):
    a = list(ptrim(a))
    b = ptrim(b)
    if not b:
        raise ZeroDivisionError("polynomial division by zero")
    q = [F(0)] * max(len(a) - len(b) + 1, 1)
    while a and len(a) >= len(b):
        k = len(a) - len(b)
        c = a[-1] / b[-1]
        q[k] = c
        for i, y in enumerate(b):
            a[i + k] -= c * y
        a = list(ptrim(a))
    return ptrim(q), ptrim(a)


def pgcd(a, b):
    a, b = ptrim(a), ptrim(b)
    while b:
        a, b = b, pdivmod(a, b)[1]
    return tuple(x / a[-1] for x in a)


def pmono(p):
    """(power, coefficient) when p has exactly one nonzero coefficient."""
    hits = [(i, c) for i, c in enumerate(p) if c != 0]
    return hits[0] if len(hits) == 1 else None


class Q:
    """An exact element of Q(pi), held with a monic denominator in lowest terms."""

    __slots__ = ("n", "d")

    def __init__(self, n, d=(F(1),)):
        n = ptrim(F(x) for x in n)
        d = ptrim(F(x) for x in d)
        if not d:
            raise ZeroDivisionError("Q(pi) division by zero")
        if not n:
            self.n, self.d = (), (F(1),)
            return
        mn, md = pmono(n), pmono(d)
        if mn and md:
            (a, ca), (b, cb) = mn, md
            k = min(a, b)
            self.n = (F(0),) * (a - k) + (ca / cb,)
            self.d = (F(0),) * (b - k) + (F(1),)
            return
        g = pgcd(n, d)
        if len(g) > 1:
            n, d = pdivmod(n, g)[0], pdivmod(d, g)[0]
        lead = d[-1]
        self.n = tuple(x / lead for x in n)
        self.d = tuple(x / lead for x in d)

    @staticmethod
    def of(x):
        if isinstance(x, Q):
            return x
        return Q((F(x),))

    def __add__(self, o):
        o = Q.of(o)
        if not o.n:
            return self
        if not self.n:
            return o
        if self.d == o.d:
            return Q(padd(self.n, o.n), self.d)
        return Q(padd(pmul(self.n, o.d), pmul(o.n, self.d)), pmul(self.d, o.d))

    __radd__ = __add__

    def __neg__(self):
        r = Q.__new__(Q)
        r.n, r.d = tuple(-x for x in self.n), self.d
        return r

    def __sub__(self, o):
        return self + (-Q.of(o))

    def __rsub__(self, o):
        return Q.of(o) - self

    def __mul__(self, o):
        o = Q.of(o)
        if not self.n or not o.n:
            return ZERO
        return Q(pmul(self.n, o.n), pmul(self.d, o.d))

    __rmul__ = __mul__

    def __truediv__(self, o):
        o = Q.of(o)
        if not o.n:
            raise ZeroDivisionError("Q(pi) division by zero")
        return Q(pmul(self.n, o.d), pmul(self.d, o.n))

    def __rtruediv__(self, o):
        return Q.of(o) / self

    def __eq__(self, o):
        if isinstance(o, (int, F)):
            o = Q.of(o)
        if not isinstance(o, Q):
            return NotImplemented
        return self.n == o.n and self.d == o.d

    def __ne__(self, o):
        return not self == o

    def __hash__(self):
        return hash((self.n, self.d))

    def is_zero(self):
        return not self.n

    def rational(self):
        if not self.n:
            return F(0)
        if len(self.n) == 1 and len(self.d) == 1:
            return self.n[0] / self.d[0]
        return None

    def dec(self):
        def ev(p):
            total, power = Decimal(0), Decimal(1)
            for c in p:
                if c != 0:
                    total += Decimal(c.numerator) / Decimal(c.denominator) * power
                power *= PI_DEC
            return total

        if not self.n:
            return Decimal(0)
        return ev(self.n) / ev(self.d)

    def sign(self):
        if not self.n:
            return 0
        return 1 if self.dec() > 0 else -1


def _atan_inv(x):
    x = Decimal(x)
    total, term, n, sign = Decimal(0), 1 / x, 1, 1
    x2 = x * x
    eps = Decimal(10) ** -(getcontext().prec - 5)
    while True:
        delta = term / n
        if delta < eps:
            break
        total += sign * delta
        term /= x2
        n += 2
        sign = -sign
    return total


PI_DEC = 16 * _atan_inv(5) - 4 * _atan_inv(239)  # Machin
ZERO = Q(())
ONE = Q((1,))
PI = Q((0, 1))


def fsqrt(x):
    """Exact square root of a nonnegative rational, or None."""
    x = F(x)
    if x < 0:
        return None
    a, b = isqrt(x.numerator), isqrt(x.denominator)
    if a * a == x.numerator and b * b == x.denominator:
        return F(a, b)
    return None


def qsqrt(x):
    """Exact square root in Q(pi) of c*pi^(2k), c a rational square; else None."""
    x = Q.of(x)
    if x.is_zero():
        return ZERO
    mn, md = pmono(x.n), pmono(x.d)
    if not (mn and md):
        return None
    (a, ca), (b, cb) = mn, md
    if (a - b) % 2:
        return None
    r = fsqrt(ca / cb)
    if r is None:
        return None
    k = (a - b) // 2
    return Q((0,) * k + (r,)) if k >= 0 else Q((r,), (0,) * (-k) + (1,))


def fstr(c):
    c = F(c)
    return str(c.numerator) if c.denominator == 1 else f"{c.numerator}/{c.denominator}"


def qexpr(x):
    """Exact text: 'r', 'r*pi', 'r/pi', 'r*pi^k', 'r/pi^k', or '(n0 + n1*pi + ...)/(d0 + d1*pi + ...)'
    with rational coefficients r = 'p' or 'p/q'."""
    x = Q.of(x)
    if x.is_zero():
        return "0"
    mn, md = pmono(x.n), pmono(x.d)
    if mn and md:
        (a, ca), (b, cb) = mn, md
        k, c = a - b, ca / cb
        if k == 0:
            return fstr(c)
        if k == 1:
            return f"{fstr(c)}*pi"
        if k == -1:
            return f"{fstr(c)}/pi"
        return f"{fstr(c)}*pi^{k}" if k > 0 else f"{fstr(c)}/pi^{-k}"

    def poly(p):
        terms = []
        for i, c in enumerate(p):
            if c == 0:
                continue
            power = "" if i == 0 else ("pi" if i == 1 else f"pi^{i}")
            if i == 0:
                terms.append(fstr(c))
            elif c == 1:
                terms.append(power)
            elif c == -1:
                terms.append("-" + power)
            else:
                terms.append(f"{fstr(c)}*{power}")
        return "(" + " + ".join(terms) + ")"

    if x.d == (F(1),):
        return poly(x.n)
    return poly(x.n) + "/" + poly(x.d)


def qform(x):
    x = Q.of(x)
    if x.is_zero():
        return {"kind": "rational", "rational": "0"}
    mn, md = pmono(x.n), pmono(x.d)
    if mn and md:
        (a, ca), (b, cb) = mn, md
        k, c = a - b, ca / cb
        kinds = {0: "rational", 1: "rational_times_pi", -1: "rational_over_pi"}
        if k in kinds:
            return {"kind": kinds[k], "rational": fstr(c)}
        return {"kind": "rational_times_pi_power", "rational": fstr(c), "power": k}
    return {"kind": "rational_function_of_pi", "numerator_ascending": [fstr(c) for c in x.n],
            "denominator_ascending": [fstr(c) for c in x.d], "expression": qexpr(x)}


def dstr(d):
    return "0" if d == 0 else format(d, ".40g")


def b64(d):
    return 0.0 if d == 0 else float(format(d, ".80e"))


class Sqrt:
    """sqrt(radicand), radicand > 0 in Q(pi) and not a square there."""

    def __init__(self, radicand):
        self.radicand = Q.of(radicand)

    def dec(self):
        return self.radicand.dec().sqrt()


class Expr:
    """A positive real outside Q(pi) and not a plain square root (only wrong values of
    mutants): an exact text and its decimal."""

    def __init__(self, text, d):
        self.text, self.d = text, d

    def dec(self):
        return self.d


def vdec(v):
    return v.dec() if isinstance(v, (Sqrt, Expr)) else Q.of(v).dec()


def vzero(v):
    return (not isinstance(v, (Sqrt, Expr))) and Q.of(v).is_zero()


def vexpr(v):
    if isinstance(v, Expr):
        return v.text
    return f"sqrt({qexpr(v.radicand)})" if isinstance(v, Sqrt) else qexpr(v)


def vabs(v):
    if isinstance(v, (Sqrt, Expr)):
        return v
    v = Q.of(v)
    return -v if v.sign() < 0 else v


def num(v, unit=None):
    """A reference number: exact form, 40-digit decimal and binary64 value."""
    d = vdec(v)
    out = {} if unit is None else {"unit": unit}
    if isinstance(v, Expr):
        out["exact"] = {"kind": "expression", "text": v.text}
    else:
        out["exact"] = {"kind": "sqrt", "radicand": qform(v.radicand)} if isinstance(v, Sqrt) else qform(v)
    out["decimal"] = dstr(d)
    out["value"] = b64(d)
    return out


def vmag(components):
    nz = [Q.of(c) for c in components if not Q.of(c).is_zero()]
    if not nz:
        return ZERO
    if len(nz) == 1:
        return vabs(nz[0])
    s = ZERO
    for c in nz:
        s = s + c * c
    r = qsqrt(s)
    return r if r is not None else Sqrt(s)


# ===========================================================================
# Section shear rigidity (CONTRACT section 1; design section 3.1).
# ===========================================================================


def kappa_energy(m, nu):
    """kappa_E with the cancellation-free denominator (CONTRACT section 1)."""
    m, nu = F(m), F(nu)
    n1 = 1 + nu
    c = (8 * nu + 14) * nu + 7
    den = c * (1 - m * m) ** 2 + 48 * m * m * n1 * n1
    return 6 * n1 * n1 * (1 + m * m) ** 2 / den


def kappa_energy_contract_form(m, nu):
    m, nu = F(m), F(nu)
    den = (7 + 14 * nu + 8 * nu * nu) * (1 + m * m) ** 2 + 4 * m * m * (5 + 10 * nu + 4 * nu * nu)
    return 6 * (1 + nu) ** 2 * (1 + m * m) ** 2 / den


def kappa_cowper(m, nu):
    m, nu = F(m), F(nu)
    return 6 * (1 + nu) * (1 + m * m) ** 2 / ((7 + 6 * nu) * (1 + m * m) ** 2 + (20 + 12 * nu) * m * m)


def kappa_binary64(r_o, r_i, nu):
    """Design section 3.1's fixed binary64 operation order (Python floats; no FMA)."""
    m = r_i / r_o
    m2 = m * m
    a = 1.0 + m2
    b = 1.0 - m2
    n1 = 1.0 + nu
    c = (8.0 * nu + 14.0) * nu + 7.0
    d = c * (b * b) + 48.0 * (m2 * (n1 * n1))
    return (6.0 * (n1 * n1)) * (a * a) / d


def laurent_integral(terms, lo, hi):
    """Integral over [lo, hi] of sum c r^k for integer k != -1."""
    total = F(0)
    for k, c in terms.items():
        if k == -1:
            raise ValueError("logarithmic term")
        total += c * (F(hi) ** (k + 1) - F(lo) ** (k + 1)) / (k + 1)
    return total


def laurent_mul(a, b):
    out = {}
    for ka, ca in a.items():
        for kb, cb in b.items():
            out[ka + kb] = out.get(ka + kb, F(0)) + ca * cb
    return {k: c for k, c in out.items() if c != 0}


def laurent_add(*ps):
    out = {}
    for p in ps:
        for k, c in p.items():
            out[k] = out.get(k, F(0)) + c
    return {k: c for k, c in out.items() if c != 0}


def saint_venant_check(a, b, nu):
    """INDEPENDENT_REFUTATION section 2's flexure field, integrated here term by term
    (a outer, b inner radius). Returns (kappa from the shear energy, int tau_X dA / V,
    traction residuals (B0 + B2)(r) at the free surfaces)."""
    a, b, nu = F(a), F(b), F(nu)
    s, p = a * a + b * b, a * a * b * b
    c, e = (3 + 2 * nu) / 4, (1 + 2 * nu) / 4
    B0 = {0: -c * s, 2: F(1, 2)}
    B2 = laurent_add({-2: c * p} if p else {}, {2: e})
    inertia_pi = (a ** 4 - b ** 4) / 4  # I / pi
    area_pi = a * a - b * b  # A / pi
    radial = laurent_integral({k + 1: v for k, v in laurent_add(laurent_mul(B0, B0), laurent_mul(B2, B2)).items()},
                              b, a)
    # int (tau^2/V^2) dA = pi/(2(1+nu)^2 I^2) * radial = radial / (2 (1+nu)^2 pi inertia_pi^2)
    energy_pi = radial / (2 * (1 + nu) ** 2 * inertia_pi ** 2)  # pi * int(tau^2/V^2 dA)
    kappa = 1 / (area_pi * energy_pi)
    # int tau_X dA / V = -(1/(2(1+nu) I)) * 2 pi * int B0 r dr
    resultant = -laurent_integral({k + 1: v for k, v in B0.items()}, b, a) / ((1 + nu) * inertia_pi)
    residuals = [sum(cc * F(r) ** k for k, cc in laurent_add(B0, B2).items()) for r in ((a, b) if b else (a,))]
    return kappa, resultant, residuals


def cowper_displacement_check(a, b, nu):
    """kappa_C = -2(1+nu) I^2 / (A Jchi), Jchi = -A[(7+6nu)s^2 + (20+12nu)p]/48."""
    a, b, nu = F(a), F(b), F(nu)
    s, p = a * a + b * b, a * a * b * b
    area, inertia = a * a - b * b, (a ** 4 - b ** 4) / 4
    jchi = -area * ((7 + 6 * nu) * s * s + (20 + 12 * nu) * p) / 48
    return -2 * (1 + nu) * inertia ** 2 / (area * jchi)


# ===========================================================================
# The exact static plane element (CONTRACT section 3) and its loads (section 4).
# d = [v_i, theta_i, v_j, theta_j]; Ks None is the Euler-Bernoulli limit.
# ===========================================================================


def plane_k(EI, Ks, L):
    L = F(L)
    phi = ZERO if Ks is None else Q.of(12) * Q.of(EI) / (Q.of(Ks) * L * L)
    c = Q.of(EI) / (L ** 3 * (ONE + phi))
    return [[12 * c, 6 * L * c, -12 * c, 6 * L * c],
            [6 * L * c, (4 + phi) * L * L * c, -6 * L * c, (2 - phi) * L * L * c],
            [-12 * c, -6 * L * c, 12 * c, -6 * L * c],
            [6 * L * c, (2 - phi) * L * L * c, -6 * L * c, (4 + phi) * L * L * c]]


def plane_k_linear(EI, Ks, L):
    """The fully integrated equal-order linear v/theta element (the locking control),
    formed from U = 1/2 int EI theta'^2 + Ks (v' - theta)^2 dx with t = x/L."""
    L = F(L)
    dth = [ZERO, Q.of(F(-1) / L), ZERO, Q.of(F(1) / L)]
    g0 = [Q.of(F(-1) / L), Q.of(-1), Q.of(F(1) / L), ZERO]
    g1 = [ZERO, ONE, ZERO, Q.of(-1)]
    K = [[ZERO] * 4 for _ in range(4)]
    for a in range(4):
        for b in range(4):
            gi = g0[a] * g0[b] + (g0[a] * g1[b] + g1[a] * g0[b]) / 2 + g1[a] * g1[b] / 3
            K[a][b] = Q.of(EI) * L * dth[a] * dth[b] + Q.of(Ks) * L * gi
    return K


def gp_add(a, b):
    n = max(len(a), len(b))
    return [(a[i] if i < len(a) else ZERO) + (b[i] if i < len(b) else ZERO) for i in range(n)]


def gp_scale(a, s):
    return [Q.of(c) * s for c in a]


def gp_int(a):
    return [ZERO] + [Q.of(c) / (k + 1) for k, c in enumerate(a)]


def gp_eval(a, t):
    total = ZERO
    for c in reversed(a):
        total = total * t + c
    return total


def gp_mul(a, b):
    out = [ZERO] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        for j, y in enumerate(b):
            out[i + j] = out[i + j] + Q.of(x) * y
    return out


def integrate_span(EI, Ks, L, loads, V0, M0, th0, v0):
    """CONTRACT section 4: V' = -w; M' = -V; theta' = M/EI; v' = theta + V/Ks.
    loads: (s1, s2, w) uniform transverse loads. Pieces carry polynomials in t = x - x0."""
    L = F(L)
    cuts = sorted({F(0), L} | {F(s) for s1, s2, _ in loads for s in (s1, s2)})
    pieces = []
    V, M, th, v = Q.of(V0), Q.of(M0), Q.of(th0), Q.of(v0)
    for x0, x1 in zip(cuts, cuts[1:]):
        w = ZERO
        for s1, s2, ww in loads:
            if F(s1) <= x0 and x1 <= F(s2):
                w = w + ww
        Vp = [V, -w]
        Mp = gp_add([M], gp_scale(gp_int(Vp), Q.of(-1)))
        thp = gp_add([th], gp_scale(gp_int(Mp), ONE / Q.of(EI)))
        integrand = thp if Ks is None else gp_add(thp, gp_scale(Vp, ONE / Q.of(Ks)))
        vp = gp_add([v], gp_int(integrand))
        pieces.append((x0, x1, Vp, Mp, thp, vp))
        V, M, th, v = (gp_eval(p, x1 - x0) for p in (Vp, Mp, thp, vp))
    return pieces


def span_end(pieces):
    x0, x1, Vp, Mp, thp, vp = pieces[-1]
    return tuple(gp_eval(p, x1 - x0) for p in (Vp, Mp, thp, vp))


def span_at(pieces, x):
    x = F(x)
    for x0, x1, Vp, Mp, thp, vp in pieces:
        if x0 <= x <= x1:
            return tuple(gp_eval(p, x - x0) for p in (Vp, Mp, thp, vp))
    raise ValueError(x)


def solve_2x2(a, b):
    det = a[0][0] * a[1][1] - a[0][1] * a[1][0]
    return [(b[0] * a[1][1] - b[1] * a[0][1]) / det, (a[0][0] * b[1] - a[1][0] * b[0]) / det]


def span_solution(EI, Ks, L, loads, d):
    """V0, M0 from compatibility v(L) = v_j and theta(L) = theta_j."""
    vi, thi, vj, thj = (Q.of(x) for x in d)
    base = span_end(integrate_span(EI, Ks, L, loads, 0, 0, thi, vi))
    eV = span_end(integrate_span(EI, Ks, L, [], 1, 0, 0, 0))
    eM = span_end(integrate_span(EI, Ks, L, [], 0, 1, 0, 0))
    V0, M0 = solve_2x2([[eV[2], eM[2]], [eV[3], eM[3]]], [thj - base[2], vj - base[3]])
    return V0, M0, integrate_span(EI, Ks, L, loads, V0, M0, thi, vi)


def end_action(EI, Ks, L, loads, d):
    """f_e = [-V(0), -M(0), V(L), M(L)] (CONTRACT section 4) = K d - feq."""
    V0, M0, pieces = span_solution(EI, Ks, L, loads, d)
    VL, ML, _, _ = span_end(pieces)
    return [-V0, -M0, VL, ML]


def feq_span(EI, Ks, L, loads):
    return [-x for x in end_action(EI, Ks, L, loads, [0, 0, 0, 0])]


def span_energy(EI, Ks, pieces):
    total = ZERO
    for x0, x1, Vp, Mp, thp, vp in pieces:
        h = x1 - x0
        total = total + gp_eval(gp_int(gp_mul(Mp, Mp)), h) / (2 * Q.of(EI))
        if Ks is not None:
            total = total + gp_eval(gp_int(gp_mul(Vp, Vp)), h) / (2 * Q.of(Ks))
    return total


def span_load_work(pieces, loads):
    """1/2 int w(x) v(x) dx."""
    total = ZERO
    for x0, x1, Vp, Mp, thp, vp in pieces:
        w = ZERO
        for s1, s2, ww in loads:
            if F(s1) <= x0 and x1 <= F(s2):
                w = w + ww
        total = total + w * gp_eval(gp_int(vp), x1 - x0) / 2
    return total


def distal_half_contract(q, L, phi):
    q, L, phi = Q.of(q), F(L), Q.of(phi)
    return [q * L * (3 + 4 * phi) / (32 * (1 + phi)), q * L * L * (5 + 8 * phi) / (192 * (1 + phi)),
            q * L * (13 + 12 * phi) / (32 * (1 + phi)), -q * L * L * (11 + 8 * phi) / (192 * (1 + phi))]


def partial_span_design(EI, Ks, L, loads):
    """Design section 3.4: feq_T = feq_EB + dV0 [1, L/2, -1, L/2], dV0 = phi/(1+phi)(R_ss - V0_EB)."""
    L = F(L)
    phi = Q.of(12) * Q.of(EI) / (Q.of(Ks) * L * L)
    feb = feq_span(EI, None, L, loads)
    rss = ZERO
    for s1, s2, w in loads:
        s1, s2 = F(s1), F(s2)
        rss = rss + Q.of(w) * ((L * s2 - s2 * s2 / 2) - (L * s1 - s1 * s1 / 2)) / L
    dv0 = phi / (1 + phi) * (rss - feb[0])
    return [feb[0] + dv0, feb[1] + dv0 * L / 2, feb[2] - dv0, feb[3] + dv0 * L / 2]


# ===========================================================================
# Small dense linear algebra.
# ===========================================================================


def gauss_solve(a, b):
    n = len(b)
    m = [[Q.of(x) for x in row] + [Q.of(b[i])] for i, row in enumerate(a)]
    for col in range(n):
        piv = next((r for r in range(col, n) if not m[r][col].is_zero()), None)
        if piv is None:
            raise ArithmeticError("singular system")
        m[col], m[piv] = m[piv], m[col]
        pv = m[col][col]
        prow = m[col]
        for r in range(n):
            if r == col or m[r][col].is_zero():
                continue
            f = m[r][col] / pv
            m[r] = [x if y.is_zero() else x - f * y for x, y in zip(m[r], prow)]
    return [m[i][n] / m[i][i] for i in range(n)]


def rank_rational(a):
    m = [[Q.of(x).rational() for x in row] for row in a]
    rows, cols, r = len(m), len(m[0]), 0
    for c in range(cols):
        piv = next((i for i in range(r, rows) if m[i][c] != 0), None)
        if piv is None:
            continue
        m[r], m[piv] = m[piv], m[r]
        for i in range(rows):
            if i != r and m[i][c] != 0:
                f = m[i][c] / m[r][c]
                m[i] = [x - f * y for x, y in zip(m[i], m[r])]
        r += 1
    return r


def matvec(a, x):
    out = []
    for row in a:
        s = ZERO
        for p, q in zip(row, x):
            p, q = Q.of(p), Q.of(q)
            if not p.is_zero() and not q.is_zero():
                s = s + p * q
        out.append(s)
    return out


# ===========================================================================
# The 12-DOF straight member and a small frame model.
# DOF order per node: ux, uy, uz, rx, ry, rz; member DOFs node i then node j.
# Plane map (design section 3.3): v = uy, theta = rz (Iz); v = uz, theta = -ry (Iy).
# ===========================================================================

XY = (1, 5, 7, 11)
XZ = (2, 4, 8, 10)
SIGN_XZ = (1, -1, 1, -1)


def frame_local_k(E, G, A, I, J, kappa, Ks, L, mut=frozenset()):
    """kappa and Ks are None for Euler-Bernoulli. Mutants: shear_area_in_EA,
    kappa_on_GJ, swapped_plane_signs, linear_element."""
    L = F(L)
    E, G, A, I, J = (Q.of(x) for x in (E, G, A, I, J))
    ea = E * A * (Q.of(kappa) if "shear_area_in_EA" in mut else ONE)
    gj = G * J * (Q.of(kappa) if "kappa_on_GJ" in mut else ONE)
    if "linear_element" in mut:
        plane = plane_k_linear(E * I, Ks, L)
    else:
        plane = plane_k(E * I, Ks, L)
    K = [[ZERO] * 12 for _ in range(12)]
    for (p, q, val) in ((0, 0, ea / L), (0, 6, -ea / L), (6, 0, -ea / L), (6, 6, ea / L),
                        (3, 3, gj / L), (3, 9, -gj / L), (9, 3, -gj / L), (9, 9, gj / L)):
        K[p][q] = val
    sxz = (1, 1, 1, 1) if "swapped_plane_signs" in mut else SIGN_XZ
    for a in range(4):
        for b in range(4):
            K[XY[a]][XY[b]] = K[XY[a]][XY[b]] + plane[a][b]
            K[XZ[a]][XZ[b]] = K[XZ[a]][XZ[b]] + sxz[a] * sxz[b] * plane[a][b]
    return K


def member_frame(xi, xj, yref):
    dx = [F(b) - F(a) for a, b in zip(xi, xj)]
    L = fsqrt(sum(c * c for c in dx))
    if L is None:
        raise ValueError("member length is not rational")
    ex = [c / L for c in dx]
    y = [F(c) for c in yref]
    yd = sum(a * b for a, b in zip(y, ex))
    y = [a - yd * b for a, b in zip(y, ex)]
    ny = fsqrt(sum(c * c for c in y))
    if ny is None:
        raise ValueError("local y is not of rational length")
    ey = [c / ny for c in y]
    ez = [ex[1] * ey[2] - ex[2] * ey[1], ex[2] * ey[0] - ex[0] * ey[2], ex[0] * ey[1] - ex[1] * ey[0]]
    return L, [ex, ey, ez]


def rotate_k(kl, R):
    """K_g = T^T K_l T with T = blockdiag(R, R, R, R); R rows are the local axes."""
    kg = [[ZERO] * 12 for _ in range(12)]
    for bi in range(4):
        for bj in range(4):
            blk = [[kl[3 * bi + r][3 * bj + c] for c in range(3)] for r in range(3)]
            if all(x.is_zero() for row in blk for x in row):
                continue
            tmp = [[sum((blk[r][k] * R[k][c] for k in range(3)), ZERO) for c in range(3)] for r in range(3)]
            for r in range(3):
                for c in range(3):
                    kg[3 * bi + r][3 * bj + c] = sum((R[k][r] * tmp[k][c] for k in range(3)), ZERO)
    return kg


def to_local(R, v):
    return [sum((Q.of(R[r][k]) * v[k] for k in range(3)), ZERO) for r in range(3)]


def to_global(R, v):
    return [sum((Q.of(R[k][r]) * v[k] for k in range(3)), ZERO) for r in range(3)]


DIRS = {"global_x": 0, "global_y": 1, "global_z": 2, "rotation_x": 3, "rotation_y": 4, "rotation_z": 5}
RESTRAINT_DOF = {"UX": 0, "UY": 1, "UZ": 2, "RX": 3, "RY": 4, "RZ": 5}
STATIONS = (("end_i", F(0)), ("quarter_1", F(1, 4)), ("midspan", F(1, 2)), ("quarter_3", F(3, 4)), ("end_j", F(1)))


def annulus(ro, ri):
    ro, ri = F(ro), F(ri)
    A = Q.of(ro * ro - ri * ri) * PI
    I = Q.of((ro ** 4 - ri ** 4) / 4) * PI
    return {"ro": ro, "ri": ri, "wall": ro - ri, "A": A, "I": I, "J": 2 * I, "Z": I / ro}


def section_bases(mem):
    """D-5 (design section 1): nominal section, reduced section and pressure bore."""
    od, wall, mill, ca = (F(mem[k]) for k in ("od", "wall", "mill", "ca"))
    ro = od / 2
    t_r = (wall - mill) - ca
    return {"nominal_section": annulus(ro, ro - wall), "reduced_section": annulus(ro, ro - t_r),
            "pre_d5_wall_minus_mill": annulus(ro, ro - (wall - mill)),
            "pressure_bore": {"ri": ro - t_r, "Ai": Q.of((ro - t_r) ** 2) * PI}}


KAPPA_MUTANTS = {
    "cowper_factor": lambda m, nu: kappa_cowper(m, nu),
    "solid_circle_cowper_factor": lambda m, nu: kappa_cowper(0, nu),
    "solid_circle_energy_factor": lambda m, nu: kappa_energy(0, nu),
    "five_sixths_factor": lambda m, nu: F(5, 6),
    "unit_shear_factor": lambda m, nu: F(1),
}


def member_terms(spec, lc, mem, theory, mut):
    """E, nu, G (stiffness and torsion) and the shear terms kappa, A_s, Ks of one member
    in one load case, honouring the mutants."""
    mat = spec["materials"][mem["material"]]
    E, nu = F(mat["E"]), F(mat["nu"])
    point = lc.get("points", {}).get(mem["id"])
    if point is not None:
        E, nu = F(mat["points"][point]["E"]), F(mat["points"][point]["nu"])
    G = E / (2 * (1 + nu))
    secs = section_bases(mem)
    if "stiffness_pre_d5" in mut:
        st = secs["pre_d5_wall_minus_mill"]
    elif "stiffness_on_reduced_section" in mut:
        st = secs["reduced_section"]
    else:
        st = secs["nominal_section"]
    shear = (theory == "timoshenko_annulus_energy_v1") != ("euler_bernoulli_despite_timoshenko" in mut
                                                          or "timoshenko_despite_euler_bernoulli" in mut)
    if not shear:
        return {"E": E, "nu": nu, "G": G, "secs": secs, "st": st, "kappa": None, "As": None, "Ks": None}
    sh = st
    if "kappa_on_reduced_section" in mut:
        sh = secs["reduced_section"]
    nu_k, G_k = nu, G
    if "base_nu_for_kappa" in mut:
        nu_k = F(mat["nu"])
    if "base_nu_for_kappa_and_G" in mut:
        nu_k = F(mat["nu"])
        G_k = E / (2 * (1 + nu_k))
    if "first_case_shear_reused" in mut:
        p0 = spec["load_cases"][0]["points"][mem["id"]]
        E0, nu0 = F(mat["points"][p0]["E"]), F(mat["points"][p0]["nu"])
        nu_k, G_k = nu0, E0 / (2 * (1 + nu0))
    m = sh["ri"] / sh["ro"]
    kappa = kappa_energy(m, nu_k)
    for name, fn in KAPPA_MUTANTS.items():
        if name in mut:
            kappa = fn(m, nu_k)
    As = Q.of(kappa) * sh["A"]
    return {"E": E, "nu": nu, "G": G, "secs": secs, "st": st, "kappa": kappa, "As": As, "Ks": Q.of(G_k) * As}


def face_action(f, w_loc, x):
    """Section-cut action on the +x face of the i-side segment at local x (the j-side
    cut), from the node-on-element i-end action f[0:6] and the uniform member load."""
    x = F(x)
    Fi, Mi = f[0:3], f[3:6]
    return ([-Fi[k] - w_loc[k] * x for k in range(3)]
            + [-Mi[0], -Mi[1] - x * Fi[2] - x * x * w_loc[2] / 2, -Mi[2] + x * Fi[1] + x * x * w_loc[1] / 2])


def solve_model(spec, lc_id, theory, mut=frozenset()):
    """Assemble and solve one load case exactly; recover member actions and reactions."""
    lc = next(c for c in spec["load_cases"] if c["id"] == lc_id)
    nodes = spec["nodes"]
    nidx = {nid: k for k, (nid, _) in enumerate(nodes)}
    pos = dict(nodes)
    ndof = 6 * len(nodes)
    K = [[ZERO] * ndof for _ in range(ndof)]
    Fv = [ZERO] * ndof
    pressurized = {}
    for reg in lc.get("pressure_regions", []):
        for mid in reg["members"]:
            pressurized[mid] = reg
    mres = {}
    for mem in spec["members"]:
        t = member_terms(spec, lc, mem, theory, mut)
        st = t["st"]
        L, R = member_frame(pos[mem["i"]], pos[mem["j"]], mem["y_ref"])
        kl = frame_local_k(t["E"], t["G"], st["A"], st["I"], st["J"], t["kappa"], t["Ks"], L, mut)
        EI = Q.of(t["E"]) * st["I"]
        w_loc = [ZERO, ZERO, ZERO]
        for (mid, direction, w) in lc.get("member_uniform", []):
            if mid == mem["id"]:
                wg = [ZERO, ZERO, ZERO]
                wg[DIRS[direction]] = Q.of(w)
                w_loc = [a + b for a, b in zip(w_loc, to_local(R, wg))]
        feq = [ZERO] * 12
        for plane, comp, signs in ((XY, 1, (1, 1, 1, 1)), (XZ, 2, SIGN_XZ)):
            w = w_loc[comp]
            if w.is_zero():
                continue
            if "linear_element" in mut:
                fp = [w * L / 2, ZERO, w * L / 2, ZERO]
            else:
                fp = feq_span(EI, t["Ks"], L, [(0, L, w)])
            if "swapped_plane_signs" in mut:
                signs = (1, 1, 1, 1)
            for a in range(4):
                feq[plane[a]] = feq[plane[a]] + signs[a] * fp[a]
        feq[0] = feq[0] + w_loc[0] * L / 2
        feq[6] = feq[6] + w_loc[0] * L / 2
        eig = ZERO  # E A eps0: thermal on the nominal area, plus the pressure Poisson term E A eps_p = -2 nu P
        for (mid, dT) in lc.get("thermal", []):
            if mid == mem["id"]:
                eig = eig + Q.of(t["E"]) * t["secs"]["nominal_section"]["A"] * F(
                    spec["materials"][mem["material"]]["alpha"]) * F(dT)
        P = ZERO
        if mem["id"] in pressurized:
            P = Q.of(pressurized[mem["id"]]["p"]) * t["secs"]["pressure_bore"]["Ai"]
            eig = eig - 2 * Q.of(t["nu"]) * P
        feq[0] = feq[0] - eig
        feq[6] = feq[6] + eig
        kg = rotate_k(kl, R)
        fg = []
        for b in range(4):
            fg += to_global(R, feq[3 * b:3 * b + 3])
        dofs = [6 * nidx[mem["i"]] + k for k in range(6)] + [6 * nidx[mem["j"]] + k for k in range(6)]
        for a in range(12):
            Fv[dofs[a]] = Fv[dofs[a]] + fg[a]
            for b in range(12):
                if not kg[a][b].is_zero():
                    K[dofs[a]][dofs[b]] = K[dofs[a]][dofs[b]] + kg[a][b]
        mres[mem["id"]] = dict(t, mem=mem, L=L, R=R, kl=kl, feq=feq, w_loc=w_loc, dofs=dofs, EI=EI, P=P,
                               pressurized=mem["id"] in pressurized,
                               p=pressurized[mem["id"]]["p"] if mem["id"] in pressurized else None)
    Fnode = [ZERO] * ndof  # loads applied directly at nodes: nodal loads and transferred caps
    for (nid, direction, val) in lc.get("nodal", []):
        g = 6 * nidx[nid] + DIRS[direction]
        Fnode[g] = Fnode[g] + Q.of(val)
    for reg in lc.get("pressure_regions", []):
        for (nid, closure) in reg["terminals"]:
            if closure != "transfers_to_wall":
                continue
            for mid in reg["members"]:
                mr = mres[mid]
                if nid in (mr["mem"]["i"], mr["mem"]["j"]):
                    outward = -1 if mr["mem"]["i"] == nid else 1
                    for k in range(3):
                        g = 6 * nidx[nid] + k
                        Fnode[g] = Fnode[g] + outward * mr["P"] * mr["R"][0][k]
    Fv = [a + b for a, b in zip(Fv, Fnode)]
    fixed, springs = {}, []
    for sup in spec["supports"]:
        if sup["family"] == "spring":
            dof, k = sup["stiffness"]
            g = 6 * nidx[sup["node"]] + RESTRAINT_DOF[dof]
            K[g][g] = K[g][g] + Q.of(k)
            springs.append((sup["id"], g, Q.of(k)))
        else:
            for r in sup["restraints"]:
                fixed[6 * nidx[sup["node"]] + RESTRAINT_DOF[r]] = sup["id"]
    free = [g for g in range(ndof) if g not in fixed]
    x = gauss_solve([[K[r][c] for c in free] for r in free], [Fv[r] for r in free])
    u = [ZERO] * ndof
    for g, val in zip(free, x):
        u[g] = val
    Ku = matvec(K, u)
    reactions = {sup["id"]: [ZERO] * 6 for sup in spec["supports"]}
    for g, sid in fixed.items():
        reactions[sid][g % 6] = Ku[g] - Fv[g]
    for sid, g, k in springs:
        reactions[sid][g % 6] = -k * u[g]
    members = {}
    for mid, mr in mres.items():
        dg = [u[g] for g in mr["dofs"]]
        dl = []
        for b in range(4):
            dl += to_local(mr["R"], dg[3 * b:3 * b + 3])
        kl = mr["kl"]
        if "euler_bernoulli_recovery" in mut:
            st = mr["st"]
            kl = frame_local_k(mr["E"], mr["G"], st["A"], st["I"], st["J"], None, None, mr["L"])
        f = [s - q for s, q in zip(matvec(kl, dl), mr["feq"])]
        members[mid] = dict(mr, d_local=dl, f=f,
                            faces={name: face_action(f, mr["w_loc"], frac * mr["L"]) for name, frac in STATIONS})
    if "euler_bernoulli_recovery" in mut:
        # the mutant's reactions, if a product recovered them from member end actions
        for sup in spec["supports"]:
            if sup["family"] == "spring":
                continue
            comp = [ZERO] * 6
            for mid, mr in members.items():
                if mr["mem"]["i"] == sup["node"]:
                    part = mr["f"][0:6]
                elif mr["mem"]["j"] == sup["node"]:
                    part = mr["f"][6:12]
                else:
                    continue
                comp = [c + v for c, v in zip(comp, to_global(mr["R"], part[0:3]) + to_global(mr["R"], part[3:6]))]
            g0 = 6 * nidx[sup["node"]]
            reactions[sup["id"]] = [comp[k] - Fnode[g0 + k] if (g0 + k) in fixed else ZERO for k in range(6)]
    return {"u": {nid: u[6 * k:6 * k + 6] for k, (nid, _) in enumerate(nodes)}, "members": members,
            "reactions": reactions, "spec": spec, "lc": lc, "mutant": bool(mut)}


def slope_rotations(sol):
    """The theta = v' mutant: the published rotation replaced by the centreline slope
    v' = theta + V/Ks at every unrestrained node where the shear is single-valued."""
    spec = sol["spec"]
    restrained = {s["node"] for s in spec["supports"] if s["family"] != "spring"}
    out = {}
    for nid, _ in spec["nodes"]:
        if nid in restrained:
            continue
        ends = []
        for mid, mr in sol["members"].items():
            if mr["Ks"] is None:
                continue
            if mr["mem"]["j"] == nid:
                face = mr["faces"]["end_j"]
            elif mr["mem"]["i"] == nid:
                face = mr["faces"]["end_i"]
            else:
                continue
            ends.append((mr, face))
        if not ends:
            continue
        shears = {(vexpr(f[1]), vexpr(f[2])) for _, f in ends}
        if len(shears) != 1:
            continue  # shear jumps at this node (spring or point load): slope is two-valued
        mr, face = ends[0]
        rl = to_local(mr["R"], sol["u"][nid][3:6])
        rl = [rl[0], rl[1] - face[2] / mr["Ks"], rl[2] + face[1] / mr["Ks"]]
        out[nid] = to_global(mr["R"], rl)
    return out


# ===========================================================================
# Rows.
# ===========================================================================

REL = 1e-9
FORCE_KINDS = (("element_local_axial_force", "axial_force", 0, "N"),
               ("element_local_shear_force_y", "shear_force_y", 1, "N"),
               ("element_local_shear_force_z", "shear_force_z", 2, "N"),
               ("element_local_torsional_moment", "torsional_moment", 3, "N*m"),
               ("element_local_bending_moment_y", "bending_moment_y", 4, "N*m"),
               ("element_local_bending_moment_z", "bending_moment_z", 5, "N*m"))
BASES = ("nominal_section", "reduced_section")
UNIT_FACTOR = {("m", "mm"): F(1000), ("Pa", "MPa"): F(1, 1000000)}


def make_row(kind, entity, component, location, ref_unit, row_unit, csys, lc_id, value, family, basis=None):
    row = {"kind": kind, "entity_ref": entity, "component": component, "location": location}
    if basis is not None:
        row["section_basis"] = basis
    if ref_unit != row_unit and (ref_unit, row_unit) not in UNIT_FACTOR:
        raise ValueError((ref_unit, row_unit))
    row.update({"unit": row_unit, "coordinate_system": csys, "basis_ref": {"ref_type": "load_case", "ref_id": lc_id},
                "reference_unit": ref_unit, "factor": fstr(UNIT_FACTOR.get((ref_unit, row_unit), F(1))),
                "_value": value, "_family": family})
    return row


def lame(secs, p):
    out = []
    for comp in ("lame_inner_radial_stress", "lame_outer_radial_stress", "lame_inner_hoop_stress",
                 "lame_outer_hoop_stress"):
        vals = {}
        for basis in BASES:
            s = secs[basis]
            hoop_outer = Q.of(2 * F(p) * s["ri"] ** 2) * PI / s["A"]
            vals[basis] = {"lame_inner_radial_stress": Q.of(-F(p)), "lame_outer_radial_stress": ZERO,
                           "lame_inner_hoop_stress": hoop_outer + F(p), "lame_outer_hoop_stress": hoop_outer}[comp]
        out.append((comp, vals))
    return out


def member_maximum(mr, s, allow_irrational=False):
    """max over the member of |N/A_b| + hypot(My, Mz)/Z_b (N is constant: no axial
    member load here). Unloaded members have affine moments, so the norm is convex
    and peaks at an end; a loaded member here bends in one plane and its quadratic
    moment has no interior vertex (checked). So the five-station maximum is the
    member maximum. A mutant's norm may be irrational: then the value is an Expr."""
    w, L, f = mr["w_loc"], mr["L"], mr["f"]
    if not (w[1].is_zero() and w[2].is_zero()):
        if not (w[1].is_zero() or w[2].is_zero()):
            raise ValueError("two loaded planes: maximum argument not covered")
        comp = 5 if not w[1].is_zero() else 4
        other = 4 if comp == 5 else 5
        a0, am, a1 = (face_action(f, w, x)[comp] for x in (0, L / 2, L))
        for x in (0, L / 2, L):
            if not face_action(f, w, x)[other].is_zero():
                raise ValueError("loaded member bends in two planes")
        c = (a0 + a1 - 2 * am) * 2 / (L * L)
        b = (a1 - a0) / L - c * L
        if not c.is_zero():
            xv = -b / (2 * c)
            if 0 < xv.dec() < L:
                raise ValueError("moment vertex inside the span")
    best, bestd = None, None
    for name, frac in STATIONS:
        face = face_action(f, w, frac * L)
        mag = vmag([face[4], face[5]])
        nterm = vabs(face[0] / s["A"])
        if isinstance(mag, Sqrt):
            if not allow_irrational:
                raise ValueError("irrational moment norm in a maximum")
            val = Expr(f"{qexpr(nterm)} + sqrt({qexpr(mag.radicand)})/({qexpr(s['Z'])})",
                       vdec(nterm) + mag.dec() / s["Z"].dec())
        else:
            val = nterm + mag / s["Z"]
        d = vdec(val)
        if bestd is None or d > bestd:
            best, bestd = val, d
    return best


def member_rows(sol, mid, lc_id, rows):
    mr = sol["members"][mid]
    secs, faces, f = mr["secs"], mr["faces"], mr["f"]

    def at(loc, comp):
        if loc == "end_i":
            return f[comp]
        if loc == "end_j":
            return f[6 + comp]
        return faces[loc][comp]

    if mr["pressurized"]:
        for loc, _ in STATIONS:
            nw = faces[loc][0]
            rows.append(make_row("pipe_wall_axial_force_v2", mid, "wall_axial_force", loc, "N", "N",
                                 "element_local", lc_id, nw, "wall_force"))
            rows.append(make_row("pipe_effective_axial_force_v2", mid, "effective_axial_force", loc, "N", "N",
                                 "element_local", lc_id, nw - mr["P"], "wall_force"))
            for basis in BASES:
                rows.append(make_row("pipe_axial_membrane_stress_v2", mid, "axial_membrane_stress", loc, "Pa", "Pa",
                                     "pipe_section", lc_id, nw / secs[basis]["A"], "pressure_stress", basis))
            for comp, vals in lame(secs, mr["p"]):
                kind = "pipe_lame_radial_stress_v2" if "radial" in comp else "pipe_lame_hoop_stress_v2"
                for basis in BASES:
                    rows.append(make_row(kind, mid, comp, loc, "Pa", "Pa", "pipe_section", lc_id, vals[basis],
                                         "pressure_stress", basis))
        for loc, k in (("end_i", 0), ("end_j", 6)):
            rows.append(make_row("pipe_wall_endpoint_action_v2", mid, "wall_axial_end_action", loc, "N", "N",
                                 "element_local", lc_id, f[k], "wall_force"))
    for kind, comp, k, unit in FORCE_KINDS:
        if mr["pressurized"] and k == 0:
            continue
        for loc, _ in STATIONS:
            rows.append(make_row(kind, mid, comp, loc, unit, unit, "element_local", lc_id, at(loc, k),
                                 "member_force" if unit == "N" else "member_moment"))
    for loc, _ in STATIONS:
        face = faces[loc]
        for kind, comp, fn in (
                ("element_local_axial_normal_stress", "axial_normal_stress", lambda s: face[0] / s["A"]),
                ("element_local_bending_normal_stress_y", "bending_normal_stress_y", lambda s: face[4] / s["Z"]),
                ("element_local_bending_normal_stress_z", "bending_normal_stress_z", lambda s: face[5] / s["Z"]),
                ("element_local_torsional_shear_stress", "torsional_shear_stress",
                 lambda s: face[3] * s["ro"] / s["J"])):
            if mr["pressurized"] and comp == "axial_normal_stress":
                continue
            for basis in BASES:
                rows.append(make_row(kind, mid, comp, loc, "Pa", "MPa", "element_local", lc_id, fn(secs[basis]),
                                     "member_stress", basis))
    for basis in BASES:
        rows.append(make_row("pipe_elastic_normal_stress_maximum_v2", mid, "maximum_absolute_normal_stress",
                             "governing_station", "Pa", "Pa", "pipe_section", lc_id,
                             member_maximum(mr, secs[basis], allow_irrational=sol["mutant"]), "maximum", basis))


def nodal_rows(sol, lc_id, rows, rotations=None):
    names = ("x", "y", "z")
    for nid, _ in sol["spec"]["nodes"]:
        u = sol["u"][nid]
        rot = rotations.get(nid, u[3:6]) if rotations is not None else u[3:6]
        for k in range(3):
            rows.append(make_row(f"global_nodal_displacement_{names[k]}", nid, f"nodal_displacement_{names[k]}",
                                 "node", "m", "mm", "global", lc_id, u[k], "displacement_" + names[k]))
        for k in range(3):
            rows.append(make_row(f"global_nodal_rotation_{names[k]}", nid, f"nodal_rotation_{names[k]}", "node",
                                 "rad", "rad", "global", lc_id, rot[k], "rotation"))


def reaction_rows(sol, lc_id, rows):
    names = ("Fx", "Fy", "Fz", "Mx", "My", "Mz")
    for sup in sol["spec"]["supports"]:
        r = sol["reactions"][sup["id"]]
        for k in range(6):
            unit = "N" if k < 3 else "N*m"
            rows.append(make_row("support_reaction_component_v2", sup["id"], names[k], "node", unit, unit, "global",
                                 lc_id, r[k], "reaction_force" if k < 3 else "reaction_moment"))
        rows.append(make_row("support_reaction_force_magnitude_v2", sup["id"], "force_magnitude", "node", "N", "N",
                             "global", lc_id, vmag(r[0:3]), "reaction_force_magnitude"))
        rows.append(make_row("support_reaction_moment_magnitude_v2", sup["id"], "moment_magnitude", "node", "N*m",
                             "N*m", "global", lc_id, vmag(r[3:6]), "reaction_moment_magnitude"))


def all_rows(sol, lc_id, rotations=None):
    rows = []
    for mem in sol["spec"]["members"]:
        member_rows(sol, mem["id"], lc_id, rows)
    nodal_rows(sol, lc_id, rows, rotations)
    reaction_rows(sol, lc_id, rows)
    return rows


def row_key(r):
    return (r["kind"], r["entity_ref"], r["component"], r["location"], r.get("section_basis"))


# Zero scales for the new cases: the largest magnitude among the load case's nonzero
# rows of the same physical family (forces, moments, lengths, rotations, stresses).
FAMILY_GROUP = {"wall_force": "force", "member_force": "force", "reaction_force": "force",
                "reaction_force_magnitude": "force", "member_moment": "moment", "reaction_moment": "moment",
                "reaction_moment_magnitude": "moment", "pressure_stress": "stress", "member_stress": "stress",
                "maximum": "stress", "displacement_x": "length", "displacement_y": "length",
                "displacement_z": "length", "rotation": "rotation"}
FAMILY_SCALE_DEFINITIONS = {
    "scale_force": "max |expected| over this load case's nonzero force rows (N)",
    "scale_moment": "max |expected| over this load case's nonzero moment rows (N*m)",
    "scale_length": "max |expected| over this load case's nonzero displacement rows (m)",
    "scale_rotation": "max |expected| over this load case's nonzero rotation rows (rad)",
    "scale_stress": "max |expected| over this load case's nonzero stress rows (Pa)",
}


def family_tag(r):
    return "scale_" + FAMILY_GROUP[r["_family"]]


def family_scales(sol, rows):
    best = {}
    for r in rows:
        if vzero(r["_value"]):
            continue
        tag = family_tag(r)
        d = abs(vdec(r["_value"]))
        if tag not in best or d > best[tag][1]:
            best[tag] = (vabs(r["_value"]), d)
    out = {tag: v for tag, (v, _) in best.items()}
    if "scale_force" not in out:
        # a pure-couple case: every force row is zero; scale by the moment over the longest member
        longest = max(mr["L"] for mr in sol["members"].values())
        out["scale_force"] = out["scale_moment"] / longest
    return dict(sorted(out.items()))


FAMILY_SCALE_DEFINITIONS["scale_force_pure_couple"] = ("when every force row is zero: scale_moment divided by the "
                                                       "longest member length (N)")


def finish_rows(rows, scales, tag_of, status_of=None):
    out = []
    for r in rows:
        r = dict(r)
        v = r["_value"]
        tag = tag_of(r)
        r.pop("_value")
        r.pop("_family")
        factor = F(r["factor"])
        fd = Decimal(factor.numerator) / Decimal(factor.denominator)
        r["exact"] = vexpr(v)
        r["decimal"] = dstr(vdec(v))
        if vzero(v):
            tol = Decimal(REL) * abs(vdec(scales[tag])) * fd
            r["expected"] = 0.0
            r["criterion"] = {"kind": "zero_scale", "scale": tag, "absolute_tolerance": b64(tol)}
        else:
            r["expected"] = b64(vdec(v) * fd)
            r["criterion"] = {"kind": "relative", "relative_tolerance": REL}
        if status_of is not None:
            r["status"] = status_of(r)
        out.append(r)
    return out


def discriminate(expected_rows, wrong_rows, min_sep=Decimal("1e-7")):
    exp = {row_key(r): r for r in expected_rows}
    out = []
    for r in wrong_rows:
        e = exp.get(row_key(r))
        if e is None:
            raise KeyError(row_key(r))
        de, dw = vdec(e["_value"]), vdec(r["_value"])
        if de == dw:
            continue
        sep = abs(de - dw) / max(abs(de), abs(dw))
        if sep < min_sep:
            continue
        factor = F(r["factor"])
        fd = Decimal(factor.numerator) / Decimal(factor.denominator)
        entry = {"kind": r["kind"], "entity_ref": r["entity_ref"], "component": r["component"],
                 "location": r["location"]}
        if r.get("section_basis"):
            entry["section_basis"] = r["section_basis"]
        entry.update({"unit": r["unit"], "wrong_exact": vexpr(r["_value"]), "wrong": b64(dw * fd),
                      "expected": b64(de * fd), "relative_separation": float(format(sep, ".6e"))})
        out.append(entry)
    return out


# ===========================================================================
# Fixtures: invented SI controls (not catalog, manufacturer, library or code-rule data).
# ===========================================================================

PROV = "invented T4-U8 reference fixture (SHEAR_REFERENCE values); not library, catalog, manufacturer or code-rule data"
UNITS = {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}
STATUS = {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing",
          "professional_acceptance": "not_provided"}
TIMO = "timoshenko_annulus_energy_v1"
EB = "euler_bernoulli"
SB_OD, SB_WALL, SB_E, SB_NU, SB_F = F(1, 5), F(1, 100), F(200000000000), F(3, 10), F(100)
# A proper rational rotation (quaternion (1, 1, 2, 3)/sqrt(15)); its rows are the rotated
# member's local axes in global coordinates. Every entry is nonzero.
ROT = [[F(-11, 15), F(-2, 15), F(10, 15)], [F(10, 15), F(-5, 15), F(10, 15)], [F(2, 15), F(14, 15), F(5, 15)]]
ROT_ROOT = (F(2), F(-1), F(3))
ROT_YREF = (F(2), F(-1), F(2))
ROT_FORCE = (F(26), F(107), F(230))  # local (120, 135, 180) N; |F| = 255 N; transverse 225 N
ROT_COUPLE = (F(-94), F(17), F(170))  # local (180, 45, 60) N*m; |C| = 195 N*m; bending 75 N*m
MILL_DUAL, CA_DUAL = F(1, 800), F(1, 500)  # 0.00125 m and 0.002 m
LS_BASE = {"E": F(190000000000), "nu": F(9, 20)}
LS_POINTS = {"point:cold": {"T": F(20), "E": F(200000000000), "nu": F(3, 10)},
             "point:hot": {"T": F(300), "E": F(170000000000), "nu": F(1, 4)}}


def cantilever_spec(case_id, theory, L, load_cases, mill=F(0), ca=F(0), rotated=False, materials=None,
                    material_id="material:steel", load_state=False):
    if rotated:
        root = ROT_ROOT
        tip = tuple(r + F(L) * e for r, e in zip(root, ROT[0]))
        yref = ROT_YREF
    else:
        root, tip, yref = (F(0), F(0), F(0)), (F(L), F(0), F(0)), (F(0), F(1), F(0))
    return {"case_id": case_id, "shear_model": theory, "project_id": "project:t4-u8-" + case_id.lower(),
            "nodes": [("node:root", root), ("node:tip", tip)],
            "members": [{"id": "pipe:span", "i": "node:root", "j": "node:tip", "od": SB_OD, "wall": SB_WALL,
                         "mill": mill, "ca": ca, "material": material_id, "y_ref": yref}],
            "materials": materials or {"material:steel": {"E": SB_E, "nu": SB_NU}},
            "supports": [{"id": "support:root", "node": "node:root", "family": "anchor",
                          "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"]}],
            "load_cases": load_cases, "load_state": load_state}


def sb_load_cases():
    return [{"id": "case:tip-fy", "nodal": [("node:tip", "global_y", SB_F)], "label": "tip force +100 N global Y"},
            {"id": "case:tip-fy-reversed", "nodal": [("node:tip", "global_y", -SB_F)],
             "label": "reversed tip force -100 N global Y"},
            {"id": "case:tip-fz", "nodal": [("node:tip", "global_z", SB_F)],
             "label": "tip force +100 N global Z (the x-z bending plane)"}]


def rotated_load_cases():
    return [{"id": "case:tip-force",
             "nodal": [("node:tip", d, v) for d, v in zip(("global_x", "global_y", "global_z"), ROT_FORCE)],
             "label": "tip force (26, 107, 230) N global = (120, 135, 180) N local"},
            {"id": "case:tip-force-reversed",
             "nodal": [("node:tip", d, -v) for d, v in zip(("global_x", "global_y", "global_z"), ROT_FORCE)],
             "label": "reversed tip force (-26, -107, -230) N global"},
            {"id": "case:tip-couple",
             "nodal": [("node:tip", d, v) for d, v in zip(("rotation_x", "rotation_y", "rotation_z"), ROT_COUPLE)],
             "label": "tip couple (-94, 17, 170) N*m global = (180, 45, 60) N*m local"}]


def spring_spec(case_id, theory):
    mem = {"od": SB_OD, "wall": SB_WALL, "mill": F(0), "ca": F(0), "material": "material:steel",
           "y_ref": (F(0), F(1), F(0))}
    return {"case_id": case_id, "shear_model": theory, "project_id": "project:t4-u8-" + case_id.lower(),
            "nodes": [("node:root", (F(0), F(0), F(0))), ("node:spring", (F(1), F(0), F(0))),
                      ("node:tip", (F(2), F(0), F(0)))],
            "members": [dict(mem, id="pipe:root-spring", i="node:root", j="node:spring"),
                        dict(mem, id="pipe:spring-tip", i="node:spring", j="node:tip")],
            "materials": {"material:steel": {"E": SB_E, "nu": SB_NU}},
            "supports": [{"id": "support:root", "node": "node:root", "family": "anchor",
                          "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"]},
                         {"id": "support:spring", "node": "node:spring", "family": "spring", "restraints": ["UY"],
                          "stiffness": ("UY", F(1000000))}],
            "load_cases": [{"id": "case:tip-fy", "nodal": [("node:tip", "global_y", SB_F)],
                            "label": "tip force +100 N global Y; ground spring 1e6 N/m in UY at x = 1 m"}],
            "load_state": False}


def load_state_spec(case_id, theory):
    mats = {"material:shared": {"E": LS_BASE["E"], "nu": LS_BASE["nu"],
                                "points": {k: dict(v) for k, v in LS_POINTS.items()}}}
    lcs = [{"id": "case:cold", "nodal": [("node:tip", "global_y", SB_F)], "points": {"pipe:span": "point:cold"},
            "label": "tip force +100 N global Y; member resolves point:cold (E 200 GPa, nu 0.3)"},
           {"id": "case:hot", "nodal": [("node:tip", "global_y", SB_F)], "points": {"pipe:span": "point:hot"},
            "label": "tip force +100 N global Y; member resolves point:hot (E 170 GPa, nu 0.25)"}]
    return cantilever_spec(case_id, theory, 1, lcs, materials=mats, material_id="material:shared", load_state=True)


def i8_spec(theory):
    xs = (("node:A", F(0)), ("node:B", F(3, 2)), ("node:C", F(9, 2)), ("node:D", F(6)))
    pipes = [{"id": f"pipe:{a}-{b}", "i": f"node:{a}", "j": f"node:{b}", "od": F(1, 5), "wall": F(1, 100),
              "mill": F(0), "ca": F(0), "material": "material:tp", "y_ref": (F(0), F(1), F(0))}
             for a, b in (("A", "B"), ("B", "C"), ("C", "D"))]
    region = {"id": "region:tp", "members": [p["id"] for p in pipes], "p": F(2000000),
              "terminals": [("node:A", "transfers_to_wall"), ("node:D", "transfers_to_wall")]}
    return {"case_id": "I8-CASE2-TIMOSHENKO", "shear_model": theory,
            "nodes": [(k, (v, F(0), F(0))) for k, v in xs], "members": pipes,
            "materials": {"material:tp": {"E": F(200000000000), "nu": F(3, 10), "alpha": F(3, 250000)}},
            "supports": [{"id": "support:A", "node": "node:A", "family": "anchor",
                          "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"]},
                         {"id": "support:D", "node": "node:D", "family": "line_stop", "restraints": ["UX"]}],
            "load_cases": [{"id": "case:pressure-half", "pressure_regions": [region]},
                           {"id": "case:combined", "pressure_regions": [region],
                            "thermal": [(p["id"], F(5)) for p in pipes],
                            "member_uniform": [("pipe:B-C", "global_y", F(-300))]}],
            "load_state": False}


def twin_spec(theory, inputs):
    return {"case_id": "TWIN-A-TIMOSHENKO", "shear_model": theory,
            "nodes": [("node:section-a", (F(0), F(0), F(0))), ("node:section-b", (F(inputs["L_m"]), F(0), F(0)))],
            "members": [{"id": "pipe:source-section", "i": "node:section-a", "j": "node:section-b",
                         "od": F(inputs["OD_m"]), "wall": F(inputs["wall_m"]), "mill": F(0), "ca": F(0),
                         "material": "material:section", "y_ref": (F(0), F(1), F(0))}],
            "materials": {"material:section": {"E": F(inputs["E_Pa"]), "nu": F(inputs["nu"])}},
            "supports": [{"id": "support:section-a", "node": "node:section-a", "family": "anchor",
                          "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"]}],
            "load_cases": [{"id": "case:source-section",
                            "nodal": [("node:section-b", "global_y", F(inputs["tip_Fy_N"])),
                                      ("node:section-b", "rotation_x", F(inputs["tip_Mx_Nm"]))],
                            "pressure_regions": [{"id": "region:source-section", "members": ["pipe:source-section"],
                                                  "p": F(inputs["p_Pa"]),
                                                  "terminals": [("node:section-a", "transfers_to_wall"),
                                                                ("node:section-b", "transfers_to_wall")]}]}],
            "load_state": False}


# ===========================================================================
# Request documents: model 0.5.0 under 3.0.0/exact_pressure_v3.
# ===========================================================================


def qty(v, unit):
    return {"value": float(v), "unit": unit}


def contract(theory):
    return {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": theory}


def request_doc(spec):
    nodes = [{"id": nid, "position": {"x": float(p[0]), "y": float(p[1]), "z": float(p[2])}, "provenance": PROV}
             for nid, p in spec["nodes"]]
    pipes = [{"id": m["id"], "from": m["i"], "to": m["j"],
              "section": {"outside_diameter": qty(m["od"], "m"), "wall_thickness": qty(m["wall"], "m"),
                          "mill_tolerance": qty(m["mill"], "m"), "corrosion_allowance": qty(m["ca"], "m")},
              "material": m["material"],
              "y_reference": {"x": float(m["y_ref"][0]), "y": float(m["y_ref"][1]), "z": float(m["y_ref"][2])},
              "provenance": PROV} for m in spec["members"]]
    supports = []
    for s in spec["supports"]:
        entry = {"id": s["id"], "node": s["node"], "family": s["family"], "restraints": list(s["restraints"])}
        if s["family"] == "spring":
            dof, k = s["stiffness"]
            entry["stiffness"] = {"dof": dof, "value": qty(k, "N/m")}
        entry["provenance"] = PROV
        supports.append(entry)
    materials = []
    for mid, mat in spec["materials"].items():
        entry = {"id": mid, "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
                 "elastic_modulus": qty(mat["E"], "Pa"), "poisson_ratio": qty(mat["nu"], "1")}
        if "points" in mat:
            entry["provenance"] = PROV + "; the base E/nu is selected by no member: every load case selects a point"
            entry["temperature_points"] = [
                {"id": pid, "temperature": qty(pt["T"], "degC"), "elastic_modulus": qty(pt["E"], "Pa"),
                 "poisson_ratio": qty(pt["nu"], "1"), "provenance": PROV} for pid, pt in mat["points"].items()]
        else:
            entry["provenance"] = PROV
        materials.append(entry)
    cases = []
    for lc in spec["load_cases"]:
        loads = []
        for k, (nid, direction, val) in enumerate(lc.get("nodal", [])):
            moment = direction.startswith("rotation")
            loads.append({"id": f"load:{lc['id'].split(':', 1)[1]}:{k}",
                          "category": "concentrated_moment" if moment else "concentrated_force",
                          "target": {"type": "node", "node": nid}, "direction": direction,
                          "dimension": "moment" if moment else "force",
                          "magnitude": qty(val, "N*m" if moment else "N"), "provenance": PROV})
        entry = {"id": lc["id"], "primitive_loads": loads, "pressure_regions": [], "provenance": PROV}
        if spec["load_state"]:
            entry["analysis_state"] = {
                "contract": "openpipestress.load_reference_state/1.0.0",
                "reference_configuration_ref": "reference:installed",
                "element_states": [{"pipe_ref": m["id"],
                                    "material_selection": {"kind": "exact_point", "material_ref": m["material"],
                                                           "point_ref": lc["points"][m["id"]]},
                                    "thermal_state": {"kind": "unchanged_reference", "provenance": PROV}}
                                   for m in spec["members"]],
                "support_states": [{"support_ref": s["id"], "participation": {"kind": "active_model_device"}}
                                   for s in spec["supports"]],
                "load_sources": [{"source_ref": ld["id"], "factor": 1.0} for ld in loads],
                "history": {"kind": "independent_equilibrium"},
                "provenance": PROV}
        cases.append(entry)
    model = {"schema_version": "0.5.0", "document_kind": "openpipestress.product_preview.model",
             "pressure_contract": contract(spec["shear_model"]),
             "project": {"id": spec["project_id"], "units": UNITS}, "analysis_status": STATUS,
             "nodes": nodes, "pipe_segments": pipes, "supports": supports, "components": [],
             "materials": materials, "load_cases": cases, "combinations": []}
    if spec["load_state"]:
        model["reference_configurations"] = [{
            "id": "reference:installed", "label": "Installed reference",
            "geometry_ref": {"kind": "authored_model_geometry"},
            "member_references": [{"pipe_ref": m["id"], "basis": {"kind": "direct_strain_reference"},
                                   "fit": {"kind": "none"}, "provenance": PROV} for m in spec["members"]],
            "provenance": PROV}]
    return {"model": model, "materials": []}


def reauthor(doc, theory, project_id):
    """Re-author a T4-I8 document sketch as a model 0.5.0 exact-contract document:
    version 0.5.0, the 3.0.0/exact_pressure_v3 contract with its shear model, and
    explicit zero mill tolerance and corrosion allowance on every section."""
    doc = copy.deepcopy(doc)
    m = doc["model"]
    m["schema_version"] = "0.5.0"
    m["pressure_contract"] = contract(theory)
    m["project"]["id"] = project_id
    for p in m["pipe_segments"]:
        sec = p["section"]
        if set(sec) != {"outside_diameter", "wall_thickness"}:
            raise ValueError("unexpected section fields in the T4-I8 sketch")
        p["section"] = {"outside_diameter": sec["outside_diameter"], "wall_thickness": sec["wall_thickness"],
                        "mill_tolerance": {"value": 0.0, "unit": "m"},
                        "corrosion_allowance": {"value": 0.0, "unit": "m"}}
    return doc


def check_doc_matches_spec(doc, spec, form):
    """The re-authored document carries exactly the model this generator solves."""
    m = doc["model"]
    pos = {n["id"]: n["position"] for n in m["nodes"]}
    assert set(pos) == {nid for nid, _ in spec["nodes"]}
    for nid, p in spec["nodes"]:
        assert (pos[nid]["x"], pos[nid]["y"], pos[nid]["z"]) == tuple(float(c) for c in p), nid
        assert all(F(pos[nid][k]) == F(c) for k, c in zip("xyz", p)), nid
    pipes = {p["id"]: p for p in m["pipe_segments"]}
    assert set(pipes) == {mem["id"] for mem in spec["members"]}
    for mem in spec["members"]:
        p = pipes[mem["id"]]
        assert (p["from"], p["to"], p["material"]) == (mem["i"], mem["j"], mem["material"])
        s = p["section"]
        assert s["outside_diameter"]["unit"] == "m" and F(s["outside_diameter"]["value"]) == F(float(mem["od"]))
        assert s["wall_thickness"]["unit"] == "m" and F(s["wall_thickness"]["value"]) == F(float(mem["wall"]))
        assert s["mill_tolerance"] == {"value": 0.0, "unit": "m"}
        assert s["corrosion_allowance"] == {"value": 0.0, "unit": "m"}
        yr = p["y_reference"]
        assert (yr["x"], yr["y"], yr["z"]) == tuple(float(c) for c in mem["y_ref"])
    mats = {x["id"]: x for x in m["materials"]}
    assert set(mats) == set(spec["materials"])
    for mid, mat in spec["materials"].items():
        assert mats[mid]["elastic_modulus"] == {"value": float(mat["E"]), "unit": "Pa"}
        assert mats[mid]["poisson_ratio"] == {"value": float(mat["nu"]), "unit": "1"}
    sups = {s["id"]: s for s in m["supports"]}
    assert set(sups) == {s["id"] for s in spec["supports"]}
    for s in spec["supports"]:
        assert (sups[s["id"]]["node"], sups[s["id"]]["family"], sups[s["id"]]["restraints"]) == (
            s["node"], s["family"], s["restraints"])
    cases = {c["id"]: c for c in m["load_cases"]}
    assert set(cases) == {lc["id"] for lc in spec["load_cases"]}
    for lc in spec["load_cases"]:
        c = cases[lc["id"]]
        regs = c["pressure_regions"]
        assert len(regs) == len(lc.get("pressure_regions", []))
        for reg, sreg in zip(regs, lc.get("pressure_regions", [])):
            assert reg["member_pipe_ids"] == sreg["members"]
            assert reg["pressure"] == {"value": float(sreg["p"]), "unit": "Pa"}
            assert reg["pressure_basis"] == "internal_differential_zero_external_v1"
            assert [(t["node_ref"], t["closure_transfer"]) for t in reg["terminals"]] == sreg["terminals"]
        nodal, uniform, thermal = [], [], []
        if form == "load_state":
            st = c["analysis_state"]
            assert st["contract"] == "openpipestress.load_reference_state/1.0.0"
            assert {s["source_ref"] for s in st["load_sources"]} == {ld["id"] for ld in c["primitive_loads"]}
            assert all(s["factor"] == 1.0 for s in st["load_sources"])
            assert {e["pipe_ref"] for e in st["element_states"]} == set(pipes)
            for es in st["element_states"]:
                assert es["material_selection"]["kind"] == "explicit_base_properties"
                ts = es["thermal_state"]
                if ts["kind"] == "constant_alpha_interval":
                    mat = spec["materials"][pipes[es["pipe_ref"]]["material"]]
                    assert ts["coefficient"] == {"value": float(mat["alpha"]), "unit": "1/degC"}
                    assert ts["temperature_change"]["unit"] == "degC"
                    thermal.append((es["pipe_ref"], ts["temperature_change"]["value"]))
                else:
                    assert ts["kind"] == "unchanged_reference"
        for ld in c["primitive_loads"]:
            if ld["category"] in ("concentrated_force", "concentrated_moment"):
                assert ld["magnitude"]["unit"] == ("N" if ld["category"] == "concentrated_force" else "N*m")
                nodal.append((ld["target"]["node"], ld["direction"], ld["magnitude"]["value"]))
            elif ld["category"] == "weight":
                assert ld["magnitude"]["unit"] == "N/m" and ld["dimension"] == "force_per_length"
                uniform.append((ld["target"]["pipe"], ld["direction"], ld["magnitude"]["value"]))
            elif ld["category"] == "thermal":
                assert ld["magnitude"]["unit"] == "degC" and ld["dimension"] == "temperature_interval"
                thermal.append((ld["target"]["pipe"], ld["magnitude"]["value"]))
                mat = spec["materials"][pipes[ld["target"]["pipe"]]["material"]]
                assert mats[pipes[ld["target"]["pipe"]]["material"]]["thermal_expansion_coefficient"] == {
                    "value": float(mat["alpha"]), "unit": "1/degC"}
            else:
                raise ValueError(ld["category"])
        assert sorted(nodal) == sorted((n, d, float(v)) for n, d, v in lc.get("nodal", []))
        assert sorted(uniform) == sorted((p, d, float(v)) for p, d, v in lc.get("member_uniform", []))
        assert sorted(thermal) == sorted((p, float(v)) for p, v in lc.get("thermal", []))


def dumps(obj):
    return json.dumps(obj, indent=1, ensure_ascii=False) + "\n"


def _inline(o):
    return json.dumps(o, ensure_ascii=False, separators=(", ", ": "))


def _compact(o, level):
    pad = " " * level
    if isinstance(o, dict):
        if not o or ("kind" in o and "entity_ref" in o) or ("exact" in o and set(o) <= {"unit", "exact", "decimal",
                                                                                        "value"}):
            return _inline(o)
        body = ",\n".join(f"{pad} {json.dumps(k)}: {_compact(v, level + 1)}" for k, v in o.items())
        return "{\n" + body + "\n" + pad + "}"
    if isinstance(o, list):
        if not o or all(not isinstance(x, (dict, list)) for x in o) or all(
                isinstance(x, list) and all(not isinstance(y, (dict, list)) for y in x) for x in o):
            if all(isinstance(x, list) for x in o) and o:
                return "[\n" + ",\n".join(f"{pad} {_inline(x)}" for x in o) + "\n" + pad + "]"
            return _inline(o)
        return "[\n" + ",\n".join(f"{pad} {_compact(x, level + 1)}" for x in o) + "\n" + pad + "]"
    return _inline(o)


def dumps_compact(obj):
    """Indented JSON with every row and every number record on one line."""
    return _compact(obj, 0) + "\n"


def canon_sha(obj):
    return hashlib.sha256(json.dumps(obj, sort_keys=True, separators=(",", ":")).encode()).hexdigest()


# ===========================================================================
# Cross-check bookkeeping.
# ===========================================================================

CROSS = []


def agree(source, quantity, source_value, this_value, ok):
    CROSS.append({"source": source, "quantity": quantity, "source_value": str(source_value),
                  "this_value": str(this_value), "agrees": bool(ok)})
    return ok


def rel_close(a, b, tol):
    a, b = Decimal(a), Decimal(b)
    if a == b:
        return True
    return abs(a - b) <= Decimal(tol) * max(abs(a), abs(b))


# ===========================================================================
# Discriminators (mutants). Each is a well-defined wrong system or wrong reading;
# its rows are the expected rows it moves by more than 1e-7 relative.
# ===========================================================================

MUTANT_CATALOGUE = {
    "euler_bernoulli_despite_timoshenko": "the Timoshenko selection ignored: the Euler-Bernoulli element (shear omitted)",
    "timoshenko_despite_euler_bernoulli": "shear deformation applied to a document that selects euler_bernoulli",
    "cowper_factor": "Cowper's displacement-average annulus factor kappa_C in place of the energy-matched kappa_E",
    "solid_circle_cowper_factor": "the solid-circle Cowper factor 6(1+nu)/(7+6nu) (39/44 at nu = 0.3)",
    "solid_circle_energy_factor": "the solid-circle energy factor 6(1+nu)^2/(7+14nu+8nu^2) (kappa_E at m = 0)",
    "five_sixths_factor": "kappa = 5/6",
    "unit_shear_factor": "kappa = 1 (shear area equal to the metal area, as Code_Aster's SSLL106)",
    "theta_equals_slope": "the published nodal rotation is the centreline slope v' = theta + V/Ks instead of the "
                          "section rotation theta (rows at unrestrained nodes where V is single-valued)",
    "linear_element": "the fully integrated equal-order linear v/theta element (shear locking; its consistent "
                      "transverse load vector [wL/2, 0, wL/2, 0])",
    "swapped_plane_signs": "the x-z plane mapped with theta = +ry (the x-y plane's sign pattern) instead of "
                           "theta = -ry",
    "kappa_on_GJ": "the shear factor applied to the torsional rigidity: GJ -> kappa G J",
    "shear_area_in_EA": "the shear area in the axial rigidity: EA -> E kappa A (axial stiffness only; eigenloads "
                        "keep the metal area)",
    "euler_bernoulli_recovery": "stiffness-only replacement: Timoshenko K assembled and solved, member end actions "
                                "recovered with the Euler-Bernoulli K (f = K_EB d - feq); support reaction rows "
                                "shown as a product would recover them from those end actions",
    "kappa_on_reduced_section": "kappa and A_s formed on the reduced section (wall - mill - allowance) instead of "
                                "the stiffness (nominal) section",
    "stiffness_on_reduced_section": "stiffness (A, I, J) and the shear area on the reduced section",
    "stiffness_pre_d5": "stiffness and the shear area on wall - mill tolerance (the pre-D-5 single section)",
    "reduced_rows_copied_from_nominal": "one basis for both stress families (each reduced row equals its nominal "
                                        "twin)",
    "base_nu_for_kappa": "kappa_E from the material's base nu instead of the load case's resolved nu (G resolved)",
    "base_nu_for_kappa_and_G": "kappa_E and the G of Ks from the base nu (E resolved per case)",
    "first_case_shear_reused": "the first load case's kappa_E and G (hence Ks) reused for every load case",
}
STALE_VECTOR_NOTE = ("stale_euler_bernoulli_load_vector (the distal-half EB vector with Timoshenko K: tip v "
                     "54/384 instead of 53/384) is an element-level discriminator (element_controls.json S-A): "
                     "the exact route has no partial-span member load (design section 3.4), and for the only "
                     "admitted member load, full-span uniform, the exact Timoshenko vector equals the "
                     "Euler-Bernoulli vector, so no product-route row can expose it")

TIMO_MUTANTS = ("euler_bernoulli_despite_timoshenko", "cowper_factor", "solid_circle_cowper_factor",
                "solid_circle_energy_factor", "five_sixths_factor", "unit_shear_factor", "theta_equals_slope",
                "linear_element", "swapped_plane_signs", "kappa_on_GJ", "shear_area_in_EA",
                "euler_bernoulli_recovery")
EB_MUTANTS = ("timoshenko_despite_euler_bernoulli", "swapped_plane_signs")
DUAL_MUTANTS_TIMO = ("kappa_on_reduced_section", "stiffness_on_reduced_section", "stiffness_pre_d5",
                     "reduced_rows_copied_from_nominal")
DUAL_MUTANTS_EB = ("stiffness_on_reduced_section", "stiffness_pre_d5", "reduced_rows_copied_from_nominal")
LS_MUTANTS = ("base_nu_for_kappa", "base_nu_for_kappa_and_G", "first_case_shear_reused")


def mutant_rows(spec, lc_id, theory, name, sol, rows):
    if name == "theta_equals_slope":
        return all_rows(sol, lc_id, rotations=slope_rotations(sol))
    if name == "reduced_rows_copied_from_nominal":
        nominal = {row_key(r)[:4]: r["_value"] for r in rows if r.get("section_basis") == "nominal_section"}
        out = []
        for r in rows:
            r = dict(r)
            if r.get("section_basis") == "reduced_section":
                r["_value"] = nominal[row_key(r)[:4]]
            out.append(r)
        return out
    return all_rows(solve_model(spec, lc_id, theory, frozenset([name])), lc_id)


# ===========================================================================
# Product cases.
# ===========================================================================


def section_block(s):
    return {"wall_m": num(s["wall"]), "ri_m": num(s["ri"]), "As_m2": num(s["A"]), "I_m4": num(s["I"]),
            "J_m4": num(s["J"]), "Z_m3": num(s["Z"])}


def derived_block(spec, lc, theory):
    out = {}
    pos = dict(spec["nodes"])
    for mem in spec["members"]:
        t = member_terms(spec, lc, mem, theory, frozenset())
        secs = t["secs"]
        L, _ = member_frame(pos[mem["i"]], pos[mem["j"]], mem["y_ref"])
        nom = secs["nominal_section"]
        EI = Q.of(t["E"]) * nom["I"]
        entry = {"length_m": num(L), "E_Pa": num(t["E"]), "nu": num(t["nu"]), "G_Pa": num(t["G"]),
                 "ro_m": num(nom["ro"]), "nominal_section": section_block(nom),
                 "reduced_section": section_block(secs["reduced_section"]),
                 "pressure_bore": {"ri_m": num(secs["pressure_bore"]["ri"]), "Ai_m2": num(secs["pressure_bore"]["Ai"])},
                 "EI_N_m2": num(EI), "EA_N": num(Q.of(t["E"]) * nom["A"]), "GJ_N_m2": num(Q.of(t["G"]) * nom["J"])}
        m = nom["ri"] / nom["ro"]
        ro64 = float(mem["od"]) / 2.0
        ri64 = ro64 - float(mem["wall"])
        k64 = kappa_binary64(ro64, ri64, float(t["nu"]))
        if t["kappa"] is not None:
            entry["beam_theory"] = {
                "model": TIMO, "shear_factor": num(t["kappa"]),
                "shear_factor_binary64_design_order": {"value": k64, "hex": k64.hex(),
                                                       "leaves": {"r_o": ro64, "r_i": ri64, "nu": float(t["nu"])},
                                                       "relative_difference_from_exact": float(format(
                                                           (Decimal(k64) - Decimal(t["kappa"].numerator)
                                                            / Decimal(t["kappa"].denominator))
                                                           / (Decimal(t["kappa"].numerator)
                                                              / Decimal(t["kappa"].denominator)), ".3e"))},
                "shear_area_m2": num(t["As"]), "Ks_N": num(t["Ks"]),
                "phi": num(Q.of(12) * EI / (t["Ks"] * L * L))}
            entry["not_selected_factors"] = {"kappa_cowper": num(kappa_cowper(m, t["nu"])),
                                             "solid_circle_cowper": num(kappa_cowper(0, t["nu"])),
                                             "solid_circle_energy": num(kappa_energy(0, t["nu"])),
                                             "five_sixths": num(F(5, 6))}
        else:
            entry["beam_theory"] = {"model": EB}
        out[mem["id"]] = entry
    return out


def shear_status(spec, lc_id, theory, rows):
    """Per row: whether the value depends on the shear model (compared with the
    document's Euler-Bernoulli twin)."""
    if theory != TIMO:
        return {}
    eb = {row_key(r): r["_value"] for r in all_rows(solve_model(spec, lc_id, EB), lc_id)}
    return {row_key(r): ("shear_dependent" if vdec(r["_value"]) != vdec(eb[row_key(r)]) else "same_as_euler_bernoulli")
            for r in rows}


def build_load_case(spec, lc, theory, mutants, scales_fn=None, tag_of=None, extra_status=None, defs=None):
    sol = solve_model(spec, lc["id"], theory)
    rows = all_rows(sol, lc["id"])
    scales = (scales_fn or family_scales)(sol, rows)
    if defs is None:
        defs = {k: FAMILY_SCALE_DEFINITIONS[k] for k in scales}
        if not any(family_tag(r) == "scale_force" and not vzero(r["_value"]) for r in rows):
            defs["scale_force"] = FAMILY_SCALE_DEFINITIONS["scale_force_pure_couple"]
    shear = shear_status(spec, lc["id"], theory, rows)

    def status_of(r):
        key = row_key(r)
        parts = {}
        if key in shear:
            parts["shear_model"] = shear[key]
        if extra_status:
            parts.update(extra_status(r))
        return parts

    discs, quiet = [], []
    for name in mutants:
        moved = discriminate(rows, mutant_rows(spec, lc["id"], theory, name, sol, rows))
        if moved:
            discs.append({"id": name, "rows": moved})
        else:
            quiet.append(name)
    finished = finish_rows(rows, scales, tag_of or family_tag, status_of if (shear or extra_status) else None)
    return sol, {"id": lc["id"], "label": lc.get("label", lc["id"]),
                 "zero_scales": {k: num(v, None) for k, v in scales.items()},
                 "zero_scale_definitions": defs,
                 "rows": finished, "discriminators": discs, "not_discriminated_here": quiet}


def request_entry(case_id, doc, requests, suffix=""):
    path = f"requests/{case_id}{suffix}.request.json"
    data = dumps(doc).encode()
    requests[path] = data
    return {"path": path, "sha256": hashlib.sha256(data).hexdigest()}


def simple_case(case_id, family, description, spec, theory, mutants, requests, notes=()):
    doc = request_doc(spec)
    lcs, derived = [], {}
    for lc in spec["load_cases"]:
        _, block = build_load_case(spec, lc, theory, mutants)
        lcs.append(block)
        derived[lc["id"]] = derived_block(spec, lc, theory)
    if all(d == derived[spec["load_cases"][0]["id"]] for d in derived.values()):
        derived = {"all_load_cases": derived[spec["load_cases"][0]["id"]]}
    return {"case_id": case_id, "family": family, "description": description, "shear_model": theory,
            "request": request_entry(case_id, doc, requests), "derived": derived, "load_cases": lcs,
            "notes": list(notes)}


def sb_cases(requests):
    cases = []
    for L in (1, 2, 10):
        for theory, tag, muts in ((TIMO, "TIMOSHENKO", TIMO_MUTANTS), (EB, "EULER-BERNOULLI", EB_MUTANTS)):
            cid = f"S-B-L{L}-{tag}"
            spec = cantilever_spec(cid, theory, L, sb_load_cases())
            cases.append(simple_case(
                cid, "S-B", f"SHEAR_REFERENCE S-B physical cantilever, L = {L} m, anchored root, S-B section "
                f"(ro 0.1 m, ri 0.09 m, E 200 GPa, nu 0.3), {theory}; tip force +100 N and its reversal in global "
                "Y (x-y plane) and +100 N in global Z (x-z plane)", spec, theory, muts, requests))
    for theory, tag, muts in ((TIMO, "TIMOSHENKO", TIMO_MUTANTS), (EB, "EULER-BERNOULLI", EB_MUTANTS)):
        cid = f"S-B-L1-ROTATED-{tag}"
        spec = cantilever_spec(cid, theory, 1, rotated_load_cases(), rotated=True)
        cases.append(simple_case(
            cid, "S-B", "S-B L = 1 m cantilever rotated rigidly in space (local axes the rows of the rational "
            "rotation (1/15)[[-11,-2,10],[10,-5,10],[2,14,5]], root at (2, -1, 3) m, y_reference (2, -1, 2)); "
            "skew tip force with axial, both transverse components and its reversal, and a skew tip couple with "
            f"torsion and both bending components; {theory}", spec, theory, muts, requests,
            notes=["node coordinates and directions are rational; the request authors their nearest binary64 "
                   "values (relative differences of order 1e-16, inside the criterion)"]))
    for theory, tag, muts in ((TIMO, "TIMOSHENKO", TIMO_MUTANTS + DUAL_MUTANTS_TIMO),
                              (EB, "EULER-BERNOULLI", EB_MUTANTS + DUAL_MUTANTS_EB)):
        cid = f"S-B-L1-DUALBASIS-{tag}"
        spec = cantilever_spec(cid, theory, 1, sb_load_cases()[:1], mill=MILL_DUAL, ca=CA_DUAL)
        cases.append(simple_case(
            cid, "S-B", "S-B L = 1 m cantilever with mill tolerance 0.00125 m and corrosion allowance 0.002 m "
            "(reduced wall 0.00675 m, reduced ri 0.09325 m): stiffness and kappa_E on the nominal section, so "
            f"every displacement equals the zero-allowance case; nominal and reduced stress families differ; "
            f"{theory}", spec, theory, muts, requests,
            notes=["corroborating control for kappa_on_reduced_section; T4-U6's references remain the designated "
                   "killers of the D-5 mutants (design section 8)"]))
    return cases


def se_cases(requests):
    cases = []
    for theory, tag, muts in ((TIMO, "TIMOSHENKO", TIMO_MUTANTS), (EB, "EULER-BERNOULLI", EB_MUTANTS)):
        cid = f"S-E-SPRING-{tag}"
        spec = spring_spec(cid, theory)
        case = simple_case(
            cid, "S-E", "SHEAR_REFERENCE S-E physical companion: S-B section, anchored root, L = 2 m in two "
            "members, linear ground spring k = 1e6 N/m in UY at the interior node a = 1 m, tip force +100 N global Y; "
            f"{theory}. Computed exactly here (the reference script computed it in binary64)", spec, theory, muts,
            requests)
        sol = solve_model(spec, "case:tip-fy", theory)
        ua, ul = sol["u"]["node:spring"][1], sol["u"]["node:tip"][1]
        thl = sol["u"]["node:tip"][5]
        R = sol["reactions"]["support:spring"][1]
        root = sol["reactions"]["support:root"]
        case["closed_form_check"] = spring_closed_form_check(theory, ua, ul, thl, R, root)
        cases.append(case)
    return cases


def spring_closed_form_check(theory, ua, ul, thl, R, root):
    """CONTRACT section 5 S-E flexibility relations against the assembled solve."""
    EI = Q.of(SB_E) * annulus(F(1, 10), F(9, 100))["I"]
    A = annulus(F(1, 10), F(9, 100))["A"]
    G = SB_E / (2 * (1 + SB_NU))
    inv_ks = ZERO if theory == EB else ONE / (Q.of(G) * Q.of(kappa_energy(F(9, 10), SB_NU)) * A)
    L, a, Fv, k = F(2), F(1), SB_F, F(1000000)
    c_al = Q.of(a * a * (3 * L - a) / 6) / EI + a * inv_ks
    c_aa = Q.of(a ** 3 / 3) / EI + a * inv_ks
    c_ll = Q.of(L ** 3 / 3) / EI + L * inv_ks
    u_a = Fv * c_al / (1 + k * c_aa)
    Rr = -k * u_a
    u_l = Fv * c_ll + Rr * c_al
    th_l = (Fv * L * L + Rr * a * a) / (2 * EI)
    ok = [u_a == ua, Rr == R, u_l == ul, th_l == thl, root[1] == -(Fv + Rr), root[5] == -(Fv * L + Rr * a)]
    assert all(ok), ok
    return {"relations": "c_aL = a^2(3L - a)/(6EI) + a/Ks; c_aa = a^3/(3EI) + a/Ks; c_LL = L^3/(3EI) + L/Ks; "
                         "u(a) = F c_aL/(1 + k c_aa); R = -k u(a); u(L) = F c_LL + R c_aL; "
                         "theta(L) = (F L^2 + R a^2)/(2EI); root force -(F + R); root moment -(F L + R a)",
            "assembled_solve_equals_closed_form_exactly": True,
            "R_N": num(Rr), "u_a_m": num(u_a), "u_L_m": num(u_l), "theta_L_rad": num(th_l),
            "root_force_N": num(-(Fv + Rr)), "root_moment_N_m": num(-(Fv * L + Rr * a))}


def load_state_cases(requests):
    cases = []
    for theory, tag, muts in ((TIMO, "TIMOSHENKO", TIMO_MUTANTS + LS_MUTANTS), (EB, "EULER-BERNOULLI", EB_MUTANTS)):
        cid = f"LOAD-STATE-PER-CASE-NU-{tag}"
        spec = load_state_spec(cid, theory)
        cases.append(simple_case(
            cid, "LOAD-STATE", "S-B L = 1 m cantilever in a load/reference-state document whose two load cases "
            "resolve different E/nu from one material's temperature points (exact_point): case:cold E 200 GPa, "
            "nu 0.3; case:hot E 170 GPa, nu 0.25; base E 190 GPa, nu 0.45 is selected by no member. kappa_E, G and "
            f"Ks therefore differ per case; {theory}", spec, theory, muts, requests,
            notes=["the input format supports per-case material resolution: analysis_state.element_states[]."
                   "material_selection = {kind: exact_point, material_ref, point_ref} against the material's "
                   "temperature_points (load/reference-state group, model 0.5.0 carrying 0.4.0's rules); case:cold "
                   "equals S-B-L1's case:tip-fy"]))
    return cases


# --- T4-I8 case 2 and twin A -------------------------------------------------


def i8_value(rec):
    ex = rec["exact"]
    if ex["kind"] == "rational":
        return Q.of(F(ex["rational"]))
    if ex["kind"] == "rational_times_pi":
        return Q.of(F(ex["rational"])) * PI
    if ex["kind"] == "rational_over_pi":
        return Q.of(F(ex["rational"])) / PI
    return Decimal(rec["decimal"])


def same_value(mine, theirs):
    if isinstance(theirs, Decimal):
        return rel_close(vdec(mine), theirs, "1e-75")
    return (not isinstance(mine, Sqrt)) and Q.of(mine) == theirs


def i8_tag_of(r):
    k, c = r["kind"], r["component"]
    if k == "global_nodal_displacement_x":
        return "scale_axial_length"
    if k.startswith("global_nodal_displacement"):
        return "scale_transverse_length"
    if k.startswith("global_nodal_rotation"):
        return "scale_rotation"
    if k in ("element_local_shear_force_y", "element_local_shear_force_z", "support_reaction_force_magnitude_v2"):
        return "scale_transverse_force"
    if k in ("element_local_torsional_moment", "element_local_bending_moment_y", "element_local_bending_moment_z",
             "support_reaction_moment_magnitude_v2"):
        return "scale_transverse_moment"
    if k == "support_reaction_component_v2":
        return "scale_transverse_force" if c.startswith("F") else "scale_transverse_moment"
    if k in ("element_local_bending_normal_stress_y", "element_local_bending_normal_stress_z",
             "element_local_torsional_shear_stress"):
        return "scale_bending_stress"
    if k in ("pipe_wall_axial_force_v2", "pipe_effective_axial_force_v2", "pipe_wall_endpoint_action_v2"):
        return "scale_force"
    return "scale_stress"


I8_SCALE_DEFS = {
    "case:pressure-half": {
        "scale_force": "P = p Ai (axial-family zeros)", "scale_stress": "P/As",
        "scale_axial_length": "free closed-line extension L (eps_th + (1-2nu) P/(E As))",
        "scale_transverse_force": "P (no transverse load in this case)", "scale_transverse_moment": "P L",
        "scale_transverse_length": "free closed-line extension", "scale_rotation": "free extension / L",
        "scale_bending_stress": "P/As (no transverse load in this case)"},
    "case:combined": {
        "scale_force": "P = p Ai (axial-family zeros)", "scale_stress": "P/As",
        "scale_axial_length": "free closed-line extension L (eps_th + (1-2nu) P/(E As))",
        "scale_transverse_force": "|q| (b - a) = total transverse load", "scale_transverse_moment": "|M(0)| root moment",
        "scale_transverse_length": "|uy(D)| (here the Timoshenko value, by this definition)",
        "scale_rotation": "|rz(D)|", "scale_bending_stress": "|M(0)|/Z root bending stress"},
}


def i8_scales(sol, rows):
    spec, lc = sol["spec"], sol["lc"]
    mem = sol["members"]["pipe:A-B"]
    nom = mem["secs"]["nominal_section"]
    P, As, E, nu = mem["P"], nom["A"], mem["E"], mem["nu"]
    L = F(6)
    eps_th = F(3, 250000) * 5 if lc.get("thermal") else F(0)
    ext = L * (eps_th + (1 - 2 * nu) * P / (Q.of(E) * As))
    if lc["id"] == "case:pressure-half":
        return {"scale_force": P, "scale_stress": P / As, "scale_axial_length": ext, "scale_transverse_force": P,
                "scale_transverse_moment": P * L, "scale_transverse_length": ext, "scale_rotation": ext / L,
                "scale_bending_stress": P / As}
    m0 = vabs(sol["reactions"]["support:A"][5])
    return {"scale_force": P, "scale_stress": P / As, "scale_axial_length": ext, "scale_transverse_force": Q.of(900),
            "scale_transverse_moment": m0, "scale_transverse_length": vabs(sol["u"]["node:D"][1]),
            "scale_rotation": vabs(sol["u"]["node:D"][5]), "scale_bending_stress": m0 / nom["Z"]}


def i8_case(t4i8, requests):
    blk = t4i8["cases"]["tp_phys_pressure_halves"]
    if canon_sha(blk) != I8_CASE2_SHA256:
        raise SystemExit("T4-I8 case-2 block changed; re-derive before regenerating")
    spec = i8_spec(TIMO)
    doc = reauthor(blk["documents"]["v2_model_0.3.0"], TIMO, "project:t4-u8-i8-case2-timoshenko")
    doc_ls = reauthor(blk["documents"]["v2_model_0.4.0"], TIMO, "project:t4-u8-i8-case2-timoshenko-load-state")
    check_doc_matches_spec(doc, spec, "primitive")
    check_doc_matches_spec(doc_ls, spec, "load_state")
    marked = {("global_nodal_displacement_y", f"node:{n}", "nodal_displacement_y", "node") for n in "BCD"}
    lcs, derived = [], {}
    for lc in spec["load_cases"]:
        lid = lc["id"]
        i8 = blk["load_cases"][lid]
        i8_rows = {(r["kind"], r["entity_ref"], r["component"], r["location"]): r for r in i8["rows"]}
        tags_i8 = {k: r["reference_origin"]["zero_scale"]["tag"] for k, r in i8_rows.items()
                   if "zero_scale" in r["reference_origin"]}
        for theory in (EB, TIMO):
            sol = solve_model(spec, lid, theory)
            rows = all_rows(sol, lid)
            keys = {row_key(r)[:4] for r in rows}
            assert keys == set(i8_rows), (lid, theory, keys ^ set(i8_rows))
            for r in rows:
                k4 = row_key(r)[:4]
                theirs = i8_value(i8["expected"][i8_rows[k4]["expected"]])
                equal = same_value(r["_value"], theirs)
                if theory == EB or k4 not in marked or lid != "case:combined":
                    assert equal, (lid, theory, k4, vexpr(r["_value"]), theirs)
                else:
                    assert not equal, (lid, k4)
                assert r["unit"] == i8_rows[k4]["unit"] and r["coordinate_system"] == i8_rows[k4]["coordinate_system"]
                if vzero(r["_value"]):
                    assert i8_tag_of(r) == tags_i8[k4], (k4, i8_tag_of(r), tags_i8[k4])
            scales = i8_scales(sol, rows)
            for tag, val in scales.items():
                theirs = i8_value(i8["zero_scales"][tag])
                if theory == EB or tag != "scale_transverse_length" or lid != "case:combined":
                    assert same_value(val, theirs), (lid, theory, tag)
            assert I8_SCALE_DEFS[lid].keys() == i8["zero_scale_definitions"].keys()
        agree("T4-I8 rebuilt_reference_cases.json case 2 " + lid, "every row (EB solve): value, unit, zero-scale tag",
              f"{len(i8_rows)} rows", "all equal exactly (stress rows on both bases)", True)

        def extra_status(r, lid=lid):
            k4 = row_key(r)[:4]
            s = {"t4_i8": "refrozen_t4_u8" if (lid == "case:combined" and k4 in marked) else "unchanged_from_t4_i8",
                 "t4_i8_expected_key": i8_rows[k4]["expected"]}
            return s

        _, block = build_load_case(spec, lc, TIMO, TIMO_MUTANTS, scales_fn=i8_scales, tag_of=i8_tag_of,
                                   extra_status=extra_status, defs=I8_SCALE_DEFS[lid])
        if lid == "case:combined":
            block["discriminators"].append(full_span_discriminator(spec))
        lcs.append(block)
        derived[lid] = derived_block(spec, lc, TIMO)
    shear = shear_additions(spec)
    return {"case_id": "I8-CASE2-TIMOSHENKO", "family": "I8",
            "description": "T4-I8 case EXACT-PRESSURE-THERMAL-TRANSVERSE-MIXED-001 (both load cases) under "
                           "timoshenko_annulus_energy_v1: the three marked global_nodal_displacement_y rows of "
                           "case:combined (node:B, node:C, node:D) are re-frozen; every other row equals T4-I8's "
                           "frozen value (checked exactly against this generator's own Euler-Bernoulli and "
                           "Timoshenko solves); stress rows on both section bases",
            "shear_model": TIMO, "request": request_entry("I8-CASE2-TIMOSHENKO", doc, requests),
            "alternate_requests": [request_entry("I8-CASE2-TIMOSHENKO", doc_ls, requests, "-LOAD-STATE")],
            "t4_i8_binding": {"file": "validation/references/t4_i8/rebuilt_reference_cases.json",
                              "file_sha256_when_frozen": I8_FILE_SHA256,
                              "block": "/cases/tp_phys_pressure_halves", "canonical_json_sha256": I8_CASE2_SHA256,
                              "documents_reauthored": ["v2_model_0.3.0 (primary)", "v2_model_0.4.0 (alternate)"],
                              "reauthoring": "schema_version 0.5.0; pressure_contract {3.0.0, exact_pressure_v3, "
                                             "timoshenko_annulus_energy_v1}; explicit zero mill_tolerance and "
                                             "corrosion_allowance on every section; project id; nothing else"},
            "shear_additions": shear, "derived": derived, "load_cases": lcs,
            "notes": ["rz rows are the section rotation theta = int M/EI with theta(0) = 0: unchanged for this "
                      "statically determinate cantilever (V, M, reactions and stresses are statics)",
                      "the zero rows tagged scale_transverse_length take |uy(D)| of this Timoshenko solve, per "
                      "T4-I8's own scale definition (0.297 % larger than the Euler-Bernoulli scale)",
                      "T4-I8's other wrong_result_discriminators (legacy thrust, Poisson sign, caps, S sign, "
                      "station sign) concern unchanged rows and remain applicable unchanged; its full_span_load is "
                      "recomputed here under Timoshenko"]}


def shear_additions(spec):
    out = {}
    eb = solve_model(spec, "case:combined", EB)
    tm = solve_model(spec, "case:combined", TIMO)
    ks = tm["members"]["pipe:A-B"]["Ks"]
    for n in "BCD":
        add = tm["u"][f"node:{n}"][1] - eb["u"][f"node:{n}"][1]
        out[f"node:{n}"] = {"uy_euler_bernoulli_m": num(eb["u"][f"node:{n}"][1]),
                            "uy_timoshenko_m": num(tm["u"][f"node:{n}"][1]), "shear_addition_m": num(add),
                            "shear_addition_times_Ks_N_m": num(add * ks),
                            "sign": "negative (same sense as the downward load and the bending deflection)"}
        assert add.sign() < 0
    # closed form: v_s(x) = (1/Ks) int_0^x V ds, V(s) = q (b - max(s, a)) for s < b
    q, a, b = F(-300), F(3, 2), F(9, 2)
    assert out["node:B"]["shear_addition_times_Ks_N_m"] == num(Q.of(q * (b - a) * a))
    assert out["node:C"]["shear_addition_times_Ks_N_m"] == num(Q.of(q * ((b - a) * a + (b - a) ** 2 / 2)))
    assert out["node:D"]["shear_addition_times_Ks_N_m"] == out["node:C"]["shear_addition_times_Ks_N_m"]
    out["closed_form"] = ("v_s(x) = (1/Ks) int_0^x V(s) ds with V(s) = q (b - max(s, a)) for s < b, q = -300 N/m, "
                          "a = 1.5 m, b = 4.5 m: Ks v_s = -1350 N*m at B, -2700 N*m at C and at D (no shear beyond "
                          "b); Ks = kappa_E G A = (606897525000000/8267189) pi N")
    out["relative_to_euler_bernoulli_at_D"] = float(format(
        vdec(tm["u"]["node:D"][1] / eb["u"]["node:D"][1]) - 1, ".6e"))
    return out


def full_span_discriminator(spec):
    """T4-I8's full_span_load under Timoshenko: the same total load (-900 N) spread over 0..6 m."""
    wrong_spec = copy.deepcopy(spec)
    lc = next(c for c in wrong_spec["load_cases"] if c["id"] == "case:combined")
    lc["member_uniform"] = [(m["id"], "global_y", F(-150)) for m in wrong_spec["members"]]
    exp_rows = all_rows(solve_model(spec, "case:combined", TIMO), "case:combined")
    wrong = all_rows(solve_model(wrong_spec, "case:combined", TIMO), "case:combined")
    moved = [r for r in discriminate(exp_rows, wrong) if r["kind"] == "global_nodal_displacement_y"
             and r["entity_ref"] == "node:D"]
    return {"id": "full_span_load_timoshenko", "rows": moved,
            "why": "T4-I8's full_span_load recomputed under Timoshenko: same root Fy and Mz and the same shear "
                   "part at D (the root moment over Ks), different bending deflection"}


def twin_case(t4i8, odwall, requests):
    blk = t4i8["cases"]["v3_straight_twin"]
    if canon_sha(blk) != I8_TWIN_SHA256:
        raise SystemExit("T4-I8 twin block changed; re-derive before regenerating")
    fx = odwall["cases"][0]
    assert fx["id"] == "ordinary"
    inputs = fx["inputs"]
    spec = twin_spec(TIMO, inputs)
    doc = reauthor(blk["documents"]["v2_model_0.3.0"], TIMO, "project:t4-u8-twin-a-timoshenko")
    check_doc_matches_spec(doc, spec, "primitive")
    lid = "case:source-section"
    ro64 = inputs["OD_m"] / 2.0
    ri64 = ro64 - inputs["wall_m"]
    agree("SOURCE_ODWALL_EXPECTATIONS ordinary", "ro, ri binary64 bits (design section 1 order D/2 - t)",
          f"{fx['ro_bits_hex']}, {fx['ri_bits_hex']}",
          f"{struct.pack('>d', ro64).hex()}, {struct.pack('>d', ri64).hex()}",
          (struct.pack('>d', ro64).hex(), struct.pack('>d', ri64).hex()) == (fx["ro_bits_hex"], fx["ri_bits_hex"]))
    sols = {th: solve_model(spec, lid, th) for th in (EB, TIMO)}
    for th, sol in sols.items():
        mr = sol["members"]["pipe:source-section"]
        nom = mr["secs"]["nominal_section"]
        tip, root = sol["u"]["node:section-b"], sol["reactions"]["support:section-a"]
        mine = {"A_m2": nom["A"], "Ai_m2": mr["secs"]["pressure_bore"]["Ai"], "I_m4": nom["I"], "J_m4": nom["J"],
                "Z_m3": nom["Z"], "P_N": mr["P"], "G_Pa": Q.of(mr["G"]), "free_pressure_extension_m": tip[0],
                "tip_bending_y_m": tip[1], "tip_rotation_z_rad": tip[5], "tip_rotation_x_rad": tip[3],
                "axial_membrane_Pa": mr["faces"]["midspan"][0] / nom["A"],
                "maximum_absolute_normal_stress_Pa": member_maximum(mr, nom),
                "torsional_surface_shear_Pa": mr["faces"]["midspan"][3] * nom["ro"] / nom["J"],
                "root_force_Fy_N": root[1], "root_moment_Mz_Nm": root[5], "root_moment_Mx_Nm": root[3]}
        lame_vals = dict(lame(mr["secs"], mr["p"]))
        for key, comp in (("lame_inner_radial_Pa", "lame_inner_radial_stress"),
                          ("lame_outer_radial_Pa", "lame_outer_radial_stress"),
                          ("lame_inner_hoop_Pa", "lame_inner_hoop_stress"),
                          ("lame_outer_hoop_Pa", "lame_outer_hoop_stress")):
            mine[key] = lame_vals[comp]["nominal_section"]
        for key, val in mine.items():
            theirs = Decimal(fx[key]["decimal"])
            ok = rel_close(vdec(val), theirs, "1e-70") if theirs != 0 else vzero(val)
            if th == TIMO and key == "tip_bending_y_m":
                assert not ok
                continue
            assert ok, (th, key, vdec(val), theirs)
        agree("SOURCE_ODWALL_EXPECTATIONS ordinary", f"{len(mine)} frozen values ({th} solve)",
              "fixture decimals", "equal to 1e-70 relative" + (" except tip_bending_y_m" if th == TIMO else ""), True)
    tm, eb = sols[TIMO], sols[EB]
    add = tm["u"]["node:section-b"][1] - eb["u"]["node:section-b"][1]
    mr = tm["members"]["pipe:source-section"]
    assert add == SB_F * F(inputs["L_m"]) / mr["Ks"]

    def extra_status(r):
        k = row_key(r)
        if k == ("global_nodal_displacement_y", "node:section-b", "nodal_displacement_y", "node", None):
            return {"source_odwall": "refrozen_t4_u8 (tip_bending_y_m)"}
        return {}

    _, block = build_load_case(spec, spec["load_cases"][0], TIMO, TIMO_MUTANTS, extra_status=extra_status)
    block["label"] = "tip force 100 N global Y and tip torque 1 N*m, internal pressure 2 MPa, closures transferring"
    return {"case_id": "TWIN-A-TIMOSHENKO", "family": "TWIN-A",
            "description": "T4-I8 twin A (SOURCE_ODWALL_EXPECTATIONS.json case 0 'ordinary', OD 0.12 m, wall "
                           "0.01 m, L 6 m) under timoshenko_annulus_energy_v1, from the fixture's exact binary64 "
                           "inputs as the fixture itself uses them; tip_bending_y_m is re-frozen; every other "
                           "fixture value is unchanged (checked)",
            "shear_model": TIMO, "request": request_entry("TWIN-A-TIMOSHENKO", doc, requests),
            "source_binding": {"fixture": "core/product_physics/tests/fixtures/pressure_reference/"
                                          "SOURCE_ODWALL_EXPECTATIONS.json", "sha256": ODWALL_SHA256,
                               "case": "ordinary", "t4_i8_block": "/cases/v3_straight_twin",
                               "t4_i8_canonical_json_sha256": I8_TWIN_SHA256},
            "refrozen": {"fixture_key": "tip_bending_y_m",
                         "row": {"kind": "global_nodal_displacement_y", "entity_ref": "node:section-b",
                                 "component": "nodal_displacement_y", "location": "node", "unit": "mm"},
                         "timoshenko_m": num(tm["u"]["node:section-b"][1]),
                         "euler_bernoulli_m (the fixture's value)": num(eb["u"]["node:section-b"][1]),
                         "shear_addition_m": num(add),
                         "shear_addition_formula": "F L / Ks, Ks = kappa_E G A on the fixture's binary64 annulus",
                         "relative_to_euler_bernoulli": float(format(vdec(add / eb["u"]["node:section-b"][1]), ".6e"))},
            "unchanged_fixture_keys": [k for k in fx if k not in ("id", "inputs", "geometry_basis",
                                                                  "earlier_declared_radius_comparison",
                                                                  "tip_bending_y_m")],
            "derived": {lid: derived_block(spec, spec["load_cases"][0], TIMO)}, "load_cases": [block],
            "notes": ["inputs are the fixture's exact binary64 values (OD, wall, E, nu = binary64(0.3), p, L, loads), "
                      "as SOURCE_ODWALL_EXPECTATIONS defines them; ri = OD/2 - wall matches the fixture's ri bits"]}


# ===========================================================================
# Layer A: element-level controls.
# ===========================================================================


def mat_text(K):
    return [[qexpr(x) for x in row] for row in K]


def cantilever_plane(EI, Ks, L, tip_force=0, tip_couple=0, loads=()):
    L = F(L)
    K = plane_k(EI, Ks, L)
    feq = feq_span(EI, Ks, L, list(loads)) if loads else [ZERO] * 4
    kff = [[K[2][2], K[2][3]], [K[3][2], K[3][3]]]
    v, th = solve_2x2(kff, [Q.of(tip_force) + feq[2], Q.of(tip_couple) + feq[3]])
    d = [ZERO, ZERO, v, th]
    fe = [k - q for k, q in zip(matvec(K, d), feq)]
    V0, M0, pieces = span_solution(EI, Ks, L, list(loads), d)
    VL, ML, thL, vL = span_end(pieces)
    assert [-V0, -M0, VL, ML] == fe and VL == Q.of(tip_force) and ML == Q.of(tip_couple) and vL == v and thL == th
    slope = th if Ks is None else th + VL / Q.of(Ks)
    energy = span_energy(EI, Ks, pieces)
    work = (Q.of(tip_force) * v + Q.of(tip_couple) * th) / 2 + span_load_work(pieces, list(loads))
    assert energy == work
    return {"tip_v": v, "tip_theta": th, "tip_slope": slope, "root_Fy": fe[0], "root_Mz": fe[1], "feq": feq,
            "f_e": fe, "energy": energy, "pieces": pieces}


def qd(x):
    return {"exact": qexpr(x), "decimal": dstr(vdec(x))}


def s_a():
    EI, Ks, L = F(1), F(12), F(1)
    K = plane_k(EI, Ks, L)
    assert K == [[Q.of(v) for v in row] for row in
                 [[6, 3, -6, 3], [3, F(5, 2), -3, F(1, 2)], [-6, -3, 6, -3], [3, F(1, 2), -3, F(5, 2)]]]
    assert all(K[i][j] == K[j][i] for i in range(4) for j in range(4))
    for mode in ([1, 0, 1, 0], [0, 1, L, 1]):
        assert all(x.is_zero() for x in matvec(K, mode))
    assert rank_rational(K) == 2
    out = {"fixture": "EI = 1 N*m^2, Ks = 12 N, L = 1 m, phi = 12EI/(Ks L^2) = 1 (an abstract element fixture, not "
                      "a pipe material); d = [v_i, theta_i, v_j, theta_j] in the x-y plane (v = uy, theta = rz)",
           "K": mat_text(K), "K_euler_bernoulli_same_EI_L": mat_text(plane_k(EI, None, L)),
           "rigid_modes": {"translation": [1, 0, 1, 0], "rotation": ["0", "1", "L", "1"], "K_times_mode": "0",
                           "rank": 2, "symmetric": True}}
    cases = {}
    c = cantilever_plane(EI, Ks, L, tip_force=1)
    assert (c["tip_v"], c["tip_theta"], c["tip_slope"], c["root_Fy"], c["root_Mz"]) == (
        Q.of(F(5, 12)), Q.of(F(1, 2)), Q.of(F(7, 12)), Q.of(-1), Q.of(-1))
    cases["tip_force"] = {"load": "+1 N at the free end", "tip_v_m": qd(c["tip_v"]), "tip_theta_rad": qd(c["tip_theta"]),
                          "tip_slope": qd(c["tip_slope"]), "tip_gamma": qd(c["tip_slope"] - c["tip_theta"]),
                          "root_Fy_N": qd(c["root_Fy"]), "root_Mz_N_m": qd(c["root_Mz"]),
                          "end_action_f_e": [qexpr(x) for x in c["f_e"]], "strain_energy_J": qd(c["energy"]),
                          "discriminator_theta_equals_slope": {"wrong_tip_theta": qd(c["tip_slope"])},
                          "discriminator_locked_linear_element": linear_cantilever(EI, Ks, L)}
    c = cantilever_plane(EI, Ks, L, tip_couple=1)
    assert (c["tip_v"], c["tip_theta"]) == (Q.of(F(1, 2)), ONE)
    indep = {}
    for ks in (F(1), F(1000), None):
        cc = cantilever_plane(EI, ks, L, tip_couple=1)
        assert (cc["tip_v"], cc["tip_theta"]) == (c["tip_v"], c["tip_theta"])
        indep["euler_bernoulli" if ks is None else f"Ks={fstr(ks)}"] = [qexpr(cc["tip_v"]), qexpr(cc["tip_theta"])]
    cases["tip_couple"] = {"load": "+1 N*m at the free end", "tip_v_m": qd(c["tip_v"]),
                           "tip_theta_rad": qd(c["tip_theta"]), "shear_V": "0 everywhere",
                           "root_Fy_N": qd(c["root_Fy"]), "root_Mz_N_m": qd(c["root_Mz"]),
                           "independent_of_Ks": indep}
    full = [(0, 1, Q.of(1))]
    c = cantilever_plane(EI, Ks, L, loads=full)
    assert c["feq"] == [Q.of(F(1, 2)), Q.of(F(1, 12)), Q.of(F(1, 2)), Q.of(F(-1, 12))]
    assert (c["tip_v"], c["tip_theta"], c["root_Fy"], c["root_Mz"]) == (
        Q.of(F(1, 6)), Q.of(F(1, 6)), Q.of(-1), Q.of(F(-1, 2)))
    assert c["tip_v"] == Q.of(F(1, 8) + F(1, 2) / 12)
    cases["full_span_uniform"] = {"load": "+1 N/m over [0, L]", "feq": [qexpr(x) for x in c["feq"]],
                                  "feq_equals_euler_bernoulli": feq_span(EI, None, L, full) == c["feq"],
                                  "tip_v_m": qd(c["tip_v"]), "tip_theta_rad": qd(c["tip_theta"]),
                                  "closed_form": "v = qL^4/(8EI) + qL^2/(2Ks); theta = qL^3/(6EI)",
                                  "root_Fy_N": qd(c["root_Fy"]), "root_Mz_N_m": qd(c["root_Mz"]),
                                  "strain_energy_J": qd(c["energy"])}
    half = [(F(1, 2), 1, Q.of(1))]
    c = cantilever_plane(EI, Ks, L, loads=half)
    good = [Q.of(F(7, 64)), Q.of(F(13, 384)), Q.of(F(25, 64)), Q.of(F(-19, 384))]
    assert c["feq"] == good == distal_half_contract(1, 1, 1) == partial_span_design(EI, Ks, L, half)
    assert (c["tip_v"], c["tip_theta"], c["root_Fy"], c["root_Mz"]) == (
        Q.of(F(53, 384)), Q.of(F(7, 48)), Q.of(F(-1, 2)), Q.of(F(-3, 8)))
    kernel = Q.of(laurent_integral({2: F(3, 6), 3: F(-1, 6)}, F(1, 2), 1)) + Q.of(
        laurent_integral({1: F(1, 12)}, F(1, 2), 1))
    assert kernel == c["tip_v"]
    stale = feq_span(EI, None, L, half)
    assert stale == [Q.of(F(3, 32)), Q.of(F(5, 192)), Q.of(F(13, 32)), Q.of(F(-11, 192))]
    Kp = plane_k(EI, Ks, L)
    wv, wth = solve_2x2([[Kp[2][2], Kp[2][3]], [Kp[3][2], Kp[3][3]]], [stale[2], stale[3]])
    assert (wv, wth) == (Q.of(F(9, 64)), Q.of(F(7, 48))) and wv == Q.of(F(54, 384))
    for vec in (good, stale):
        assert vec[0] + vec[2] == Q.of(F(1, 2)) and vec[1] + vec[2] + vec[3] == Q.of(F(3, 8))
    cases["distal_half_uniform"] = {
        "load": "+1 N/m over [L/2, L]", "feq_exact_timoshenko": [qexpr(x) for x in good],
        "feq_from": "the integrated solution of CONTRACT section 4 (fixed-fixed compatibility); equal to CONTRACT "
                    "section 4's closed form and to design section 3.4's dV0 relation",
        "tip_v_m": qd(c["tip_v"]), "tip_theta_rad": qd(c["tip_theta"]), "root_Fy_N": qd(c["root_Fy"]),
        "root_Mz_N_m": qd(c["root_Mz"]), "tip_v_by_point_force_kernels": qd(kernel),
        "closed_form": "v = 41qL^4/(384EI) + 3qL^2/(8Ks); theta = 7qL^3/(48EI)",
        "both_vectors_balance": "force qL/2 and moment 3qL^2/8 about the root",
        "discriminator_stale_euler_bernoulli_vector": {
            "feq": [qexpr(x) for x in stale], "wrong_tip_v_m": qd(wv), "wrong_tip_v_over_384": "54/384",
            "wrong_tip_theta_rad": qd(wth), "note": "rotation and global balance alone cannot see it"}}
    d = [ZERO, ZERO, ONE, ZERO]
    f = matvec(K, d)
    assert f == [Q.of(-6), Q.of(-3), Q.of(6), Q.of(-3)]
    assert f[2] == Q.of(12) * EI / (L ** 3 * 2) and f[1] + f[3] + L * f[2] == ZERO and f[0] + f[2] == ZERO
    cases["fixed_guided"] = {"motion": "v_j = delta = 1 m, v_i = theta_i = theta_j = 0",
                             "end_actions": [qexpr(x) for x in f],
                             "force": "12 EI delta/(L^3 (1 + phi)) = 6 N", "end_moments": "-3 N*m at i and at j",
                             "balance": "sum F = 0; M_i + M_j + L F_j = 0"}
    out["cases"] = cases
    table = []
    for phi in (F(0), F(1, 4), F(1, 2), F(1), F(2), F(4), F(10)):
        ks = None if phi == 0 else 12 * EI / (phi * L * L)
        integ = feq_span(EI, ks, L, half)
        assert integ == distal_half_contract(1, 1, phi)
        if ks is not None:
            assert integ == partial_span_design(EI, ks, L, half)
        assert feq_span(EI, ks, L, full) == [Q.of(F(1, 2)), Q.of(F(1, 12)), Q.of(F(1, 2)), Q.of(F(-1, 12))]
        cc = cantilever_plane(EI, ks, L, loads=half)
        assert cc["tip_v"] == Q.of(F(41, 384)) + (ZERO if ks is None else Q.of(F(3, 8)) / ks)
        table.append({"phi": fstr(phi), "Ks_N": "inf" if ks is None else fstr(ks),
                      "distal_half_feq": [qexpr(x) for x in integ], "distal_half_tip_v_m": qexpr(cc["tip_v"]),
                      "distal_half_tip_theta_rad": qexpr(cc["tip_theta"])})
    out["distal_half_over_phi"] = {"EI": "1", "L": "1", "q": "1", "rows": table,
                                   "checks": "integrated solution = CONTRACT closed form = design dV0 relation; "
                                             "full-span vector = [1/2, 1/12, 1/2, -1/12] for every phi"}
    out["frame_12x12"] = frame_12x12()
    return out


def linear_cantilever(EI, Ks, L):
    Kl = plane_k_linear(EI, Ks, L)
    v, th = solve_2x2([[Kl[2][2], Kl[2][3]], [Kl[3][2], Kl[3][3]]], [ONE, ZERO])
    return {"K_linear": mat_text(Kl), "wrong_tip_v_m": qexpr(v), "wrong_tip_theta_rad": qexpr(th),
            "ratio_to_exact_tip_v": qexpr(v / Q.of(F(5, 12)))}


def frame_12x12():
    E, G, A, I, J, kap = F(2), F(1), F(24), F(1, 2), F(1), F(1, 2)
    As = kap * A
    Ks = G * As
    L = F(1)
    kl = frame_local_k(E, G, A, I, J, kap, Ks, L)
    assert kl[0][0] == Q.of(48) and kl[3][3] == ONE and kl[1][1] == Q.of(6) and kl[1][5] == Q.of(3)
    assert kl[2][2] == Q.of(6) and kl[2][4] == Q.of(-3) and kl[4][4] == Q.of(F(5, 2)) and kl[4][10] == Q.of(F(1, 2))
    wrong = {}
    for name, (r, c) in (("shear_area_in_EA", (0, 0)), ("kappa_on_GJ", (3, 3)), ("swapped_plane_signs", (2, 4))):
        km = frame_local_k(E, G, A, I, J, kap, Ks, L, frozenset([name]))
        wrong[name] = {"entry": [r, c], "expected": qexpr(kl[r][c]), "wrong": qexpr(km[r][c])}
    return {"fixture": "E = 2, G = 1 (nu = 0), A = 24, Iy = Iz = 1/2, J = 1, kappa = 1/2, A_s = 12, Ks = 12, L = 1 "
                       "(abstract; EI = 1 and Ks = 12 as S-A)",
            "dof_order": "[ux, uy, uz, rx, ry, rz] at node i, then node j (element-local)",
            "plane_map": "x-y: (uy, rz) carry the S-A block; x-z: (uz, -ry) carry it, so the (uz, ry) entries "
                         "change sign where exactly one index is a rotation",
            "K_local": mat_text(kl),
            "axial_EA_over_L": qexpr(kl[0][0]), "torsion_GJ_over_L": qexpr(kl[3][3]),
            "discriminators": wrong}


def plane_chain(EI, Ks, L, n, tip_force=0, tip_couple=0, loads=(), springs=()):
    """n equal exact elements, node 0 fixed; loads (s1, s2, w) in member coordinates."""
    L = F(L)
    h = L / n
    nd = 2 * (n + 1)
    K = [[ZERO] * nd for _ in range(nd)]
    Fv = [ZERO] * nd
    elems = []
    for e in range(n):
        x0, x1 = e * h, (e + 1) * h
        le = [(max(F(s1), x0) - x0, min(F(s2), x1) - x0, w) for s1, s2, w in loads if F(s1) < x1 and F(s2) > x0]
        ke = plane_k(EI, Ks, h)
        fe = feq_span(EI, Ks, h, le) if le else [ZERO] * 4
        dofs = [2 * e, 2 * e + 1, 2 * e + 2, 2 * e + 3]
        for a in range(4):
            Fv[dofs[a]] = Fv[dofs[a]] + fe[a]
            for b in range(4):
                K[dofs[a]][dofs[b]] = K[dofs[a]][dofs[b]] + ke[a][b]
        elems.append((ke, fe, dofs, le))
    Fv[2 * n] = Fv[2 * n] + tip_force
    Fv[2 * n + 1] = Fv[2 * n + 1] + tip_couple
    for node, k in springs:
        K[2 * node][2 * node] = K[2 * node][2 * node] + k
    free = list(range(2, nd))
    x = gauss_solve([[K[r][c] for c in free] for r in free], [Fv[r] for r in free])
    d = [ZERO, ZERO] + x
    Kd = matvec(K, d)
    root = [Kd[0] - Fv[0], Kd[1] - Fv[1]]
    ends = [[a - b for a, b in zip(matvec(ke, [d[g] for g in dofs]), fe)] for ke, fe, dofs, le in elems]
    return {"d": d, "root": root, "ends": ends, "elems": elems, "h": h}


def s_d():
    out = {}
    EI, L = F(1), F(1)
    slender = []
    for phi in (F(1), F(1, 100), F(1, 10 ** 6), F(1, 10 ** 12)):
        ks = 12 * EI / (phi * L * L)
        t = cantilever_plane(EI, ks, L, tip_force=1)
        eb = cantilever_plane(EI, None, L, tip_force=1)
        ratio = t["tip_v"] / eb["tip_v"]
        assert ratio == Q.of(1 + phi / 4)
        Kl = plane_k_linear(EI, ks, L)
        vl, _ = solve_2x2([[Kl[2][2], Kl[2][3]], [Kl[3][2], Kl[3][3]]], [ONE, ZERO])
        locked = vl / t["tip_v"]
        assert locked == Q.of(phi / (1 + phi))
        KT, KE = plane_k(EI, ks, L), plane_k(EI, None, L)
        dmax = max(abs(vdec(KT[i][j] - KE[i][j])) for i in range(4) for j in range(4))
        slender.append({"phi": fstr(phi), "Ks_N": fstr(ks), "compliance_ratio_timoshenko_over_eb": qd(ratio),
                        "equals_1_plus_phi_over_4": True, "locked_linear_over_exact": qd(locked),
                        "equals_phi_over_1_plus_phi": True, "max_abs_K_T_minus_K_EB": dstr(dmax)})
    out["slender_limit"] = {"fixture": "EI = 1, L = 1, tip force 1, Ks = 12/phi", "rows": slender,
                            "statement": "the exact element approaches Euler-Bernoulli smoothly (K_T - K_EB = O(phi)); "
                                         "the fully integrated linear element locks (ratio phi/(1+phi) -> 0)"}
    Ks = F(12)
    sub = {}
    loads_set = {"tip_force": dict(tip_force=Q.of(1)), "tip_couple": dict(tip_couple=Q.of(1)),
                 "full_span_uniform": dict(loads=[(0, 1, Q.of(1))]),
                 "distal_half_uniform": dict(loads=[(F(1, 2), 1, Q.of(1))])}
    for name, kw in loads_set.items():
        cont = cantilever_plane(EI, Ks, L, tip_force=kw.get("tip_force", 0), tip_couple=kw.get("tip_couple", 0),
                                loads=kw.get("loads", ()))
        per_n = {}
        for n in (1, 2, 4, 8):
            ch = plane_chain(EI, Ks, L, n, **kw)
            vals = {}
            for x in (F(1, 2), F(1)):
                k = x / ch["h"]
                if k.denominator != 1:
                    continue
                k = int(k)
                V, M, th, v = span_at(cont["pieces"], x)
                assert ch["d"][2 * k] == v and ch["d"][2 * k + 1] == th
                vals[fstr(x)] = [qexpr(ch["d"][2 * k]), qexpr(ch["d"][2 * k + 1])]
            assert ch["root"] == [cont["root_Fy"], cont["root_Mz"]]
            per_n[str(n)] = {"retained_nodes_v_theta": vals, "root_Fy_Mz": [qexpr(x) for x in ch["root"]]}
        sub[name] = per_n
    out["subdivision_abstract"] = {"fixture": "EI = 1, Ks = 12, L = 1 meshed into 1, 2, 4, 8 exact elements; the "
                                              "distal half is element-aligned for n >= 2 and a partial span for n = 1",
                                   "cases": sub,
                                   "result": "retained-node v and theta, root reactions identical for every n and "
                                             "equal to the continuous solution, exactly"}
    sbI = annulus(F(1, 10), F(9, 100))
    EIp = Q.of(SB_E) * sbI["I"]
    Ksp = Q.of(SB_E / (2 * (1 + SB_NU))) * Q.of(kappa_energy(F(9, 10), SB_NU)) * sbI["A"]
    phys = {}
    for n in (1, 2, 4, 8):
        ch = plane_chain(EIp, Ksp, 2, n, tip_force=Q.of(100))
        phys[str(n)] = [qexpr(ch["d"][2 * n]), qexpr(ch["d"][2 * n + 1])]
    assert len(set(map(tuple, phys.values()))) == 1
    out["subdivision_physical"] = {"fixture": "S-B section, L = 2 m, tip force 100 N, n = 1, 2, 4, 8",
                                   "tip_v_theta": phys, "result": "identical for every n"}
    out["rigid_modes"] = rigid_modes()
    patch = {}
    for ks in (F(1), F(12), F(1000), None):
        Kp = plane_k(EI, ks, L)
        d = [ZERO, ZERO, Q.of(F(1, 2)), ONE]
        f = matvec(Kp, d)
        assert f == [ZERO, Q.of(-1), ZERO, ONE]
        V0, M0, pieces = span_solution(EI, ks, L, [], d)
        assert V0 == ZERO and M0 == ONE
        u = sum((a * b for a, b in zip(d, f)), ZERO) / 2
        assert u == Q.of(F(1, 2)) == span_energy(EI, ks, pieces)
        patch["euler_bernoulli" if ks is None else f"Ks={fstr(ks)}"] = {"end_actions": [qexpr(x) for x in f],
                                                                        "energy": qexpr(u)}
    out["pure_curvature_patch"] = {"motion": "d = [0, 0, c L^2/2, c L], c = 1, EI = 1, L = 1",
                                   "result": "V0 = 0, M = EI c, end actions [0, -EI c, 0, EI c], energy EI c^2 L/2, "
                                             "independent of Ks", "by_Ks": patch}
    energy = {}
    d = [Q.of(F(1, 3)), Q.of(F(-2, 5)), Q.of(F(3, 7)), Q.of(F(5, 11))]
    Kp = plane_k(EI, Ks, L)
    half_dkd = sum((a * b for a, b in zip(d, matvec(Kp, d))), ZERO) / 2
    V0, M0, pieces = span_solution(EI, Ks, L, [], d)
    assert half_dkd == span_energy(EI, Ks, pieces)
    energy["homogeneous_d"] = {"d": [qexpr(x) for x in d], "half_dT_K_d": qexpr(half_dkd),
                               "int_M2_over_2EI_plus_V2_over_2Ks": qexpr(span_energy(EI, Ks, pieces))}
    for name, kw in loads_set.items():
        c = cantilever_plane(EI, Ks, L, tip_force=kw.get("tip_force", 0), tip_couple=kw.get("tip_couple", 0),
                             loads=kw.get("loads", ()))
        energy[name] = {"external_work_half": qexpr(c["energy"]), "internal_energy": qexpr(c["energy"])}
    out["energy_identity"] = {"fixture": "EI = 1, Ks = 12, L = 1", "statement": "U = 1/2 d^T K d = "
                              "int M^2/(2EI) + V^2/(2Ks) dx; loaded cases: 1/2 (F v + C theta + int w v dx) = U",
                              "cases": energy}
    return out


def rigid_modes():
    E, G, A, I, J, L = F(2), F(1), F(24), F(1, 2), F(1), F(1)
    xi = (F(0), F(0), F(0))
    xj = tuple(F(L) * e for e in ROT[0])
    out = {}
    for label, kap in (("phi=1", F(1, 2)), ("phi=2 ((2 - phi) = 0)", F(1, 4))):
        As = kap * A
        Ks = G * As
        Lm, R = member_frame(xi, xj, ROT_YREF)
        assert Lm == L and R == ROT
        kg = rotate_k(frame_local_k(E, G, A, I, J, kap, Ks, L), R)
        modes = []
        for k in range(3):
            e = [F(0)] * 3
            e[k] = F(1)
            modes.append(e + [F(0)] * 3 + e + [F(0)] * 3)
        for k in range(3):
            w = [F(0)] * 3
            w[k] = F(1)
            ui = [w[1] * xi[2] - w[2] * xi[1], w[2] * xi[0] - w[0] * xi[2], w[0] * xi[1] - w[1] * xi[0]]
            uj = [w[1] * xj[2] - w[2] * xj[1], w[2] * xj[0] - w[0] * xj[2], w[0] * xj[1] - w[1] * xj[0]]
            modes.append(ui + w + uj + w)
        for mode in modes:
            assert all(x.is_zero() for x in matvec(kg, mode))
        rank = rank_rational(kg)
        assert rank == 6
        out[label] = {"rank": rank, "nullity": 12 - rank, "six_rigid_modes_in_null_space": True}
    return {"fixture": "the abstract 12-DOF element (E 2, G 1, A 24, I 1/2, J 1, L 1) rotated by ROT (every axis "
                       "skew), at phi = 1 and phi = 2", "results": out,
            "result": "exactly six zero-energy modes (the rigid modes); no extra mode, also where (2 - phi) = 0"}


def s_c():
    out = {}
    grid = []
    for m in (F(0), F(1, 4), F(1, 2), F(9, 10), F(99, 100)):
        for nu in (F(-1, 2), F(0), F(1, 4), F(3, 10), F(49, 100)):
            ke = kappa_energy(m, nu)
            assert ke == kappa_energy_contract_form(m, nu)
            kap, res, resid = saint_venant_check(1, m, nu)
            assert kap == ke and res == 1 and all(x == 0 for x in resid)
            kc = kappa_cowper(m, nu)
            assert kc == cowper_displacement_check(1, m, nu)
            t = m * m / (1 + m * m) ** 2
            assert 1 / ke == (7 + 20 * t + (1 - 4 * t) * (nu / (1 + nu)) ** 2) / 6
            assert 0 < ke <= 1
            if nu == 0:
                assert ke == kc
            grid.append({"m": fstr(m), "nu": fstr(nu), "kappa_E": fstr(ke), "kappa_C": fstr(kc),
                         "kappa_E_decimal": dstr(vdec(ke)), "kappa_C_decimal": dstr(vdec(kc))})
    out["coefficient_grid"] = {
        "checks": "for every (m, nu): kappa_E (cancellation-free form) = CONTRACT's expanded form = 1/(A int "
                  "tau^2/V^2 dA) from INDEPENDENT_REFUTATION's Saint-Venant field integrated term by term here; the "
                  "field's resultant is V and its lateral traction vanishes at both surfaces; kappa_C = -2(1+nu) "
                  "I^2/(A Jchi); 1/kappa_E = [7 + 20t + (1-4t)(nu/(1+nu))^2]/6 with t = m^2/(1+m^2)^2; "
                  "0 < kappa_E <= 1; kappa_E = kappa_C when nu = 0", "rows": grid}
    lim = {"m_to_0": [], "m_to_1": [], "approach_to_1_at_nu_0.3": []}
    for nu in (F(0), F(1, 4), F(3, 10), F(49, 100)):
        assert kappa_energy(0, nu) == 6 * (1 + nu) ** 2 / (7 + 14 * nu + 8 * nu * nu)
        assert kappa_cowper(0, nu) == 6 * (1 + nu) / (7 + 6 * nu)
        assert kappa_energy(1, nu) == F(1, 2)
        assert kappa_cowper(1, nu) == 2 * (1 + nu) / (4 + 3 * nu)
        lim["m_to_0"].append({"nu": fstr(nu), "kappa_E": fstr(kappa_energy(0, nu)), "kappa_C": fstr(kappa_cowper(0, nu))})
        lim["m_to_1"].append({"nu": fstr(nu), "kappa_E": fstr(kappa_energy(1, nu)), "kappa_C": fstr(kappa_cowper(1, nu))})
    for k in range(1, 7):
        m = 1 - F(1, 10 ** k)
        lim["approach_to_1_at_nu_0.3"].append({"m": fstr(m), "kappa_E_minus_half": dstr(vdec(kappa_energy(m, F(3, 10))
                                                                                              - F(1, 2)))})
    lim["formulas"] = {"m_to_0": "kappa_E = 6(1+nu)^2/(7+14nu+8nu^2); kappa_C = 6(1+nu)/(7+6nu)",
                       "m_to_1": "kappa_E = 1/2; kappa_C = 2(1+nu)/(4+3nu)"}
    out["limits"] = lim
    m, nu = F(9, 10), F(3, 10)
    ke = kappa_energy(m, nu)
    assert ke == F(16609827, 33068756) == F(33219654, 66137512)
    factors = {"kappa_E (selected)": ke, "kappa_C": kappa_cowper(m, nu), "solid_circle_cowper": kappa_cowper(0, nu),
               "solid_circle_energy": kappa_energy(0, nu), "five_sixths": F(5, 6), "unit": F(1)}
    assert factors["solid_circle_cowper"] == F(39, 44)
    sb = annulus(F(1, 10), F(9, 100))
    EIp = Q.of(SB_E) * sb["I"]
    G = SB_E / (2 * (1 + SB_NU))
    vb = Q.of(100) / (3 * EIp)
    ftab = {}
    for name, kap in factors.items():
        vs = Q.of(100) / (Q.of(G) * Q.of(kap) * sb["A"])
        ftab[name] = {"kappa": fstr(kap), "kappa_decimal": dstr(vdec(kap)), "S_B_L1_tip_v_m": qd(vb + vs),
                      "ratio_to_selected": dstr(vdec((vb + vs) / (vb + Q.of(100) / (Q.of(G) * Q.of(ke) * sb["A"]))))}
    out["hollow_m_0.9_nu_0.3"] = {"factors": ftab, "note": "nu = 0 alone cannot separate kappa_E from kappa_C; m = 0.9 "
                                                           "rejects a solid-circle substitution"}
    # unit scaling: the same physical S-B L = 1 m cantilever in three coherent systems
    systems = {}
    for label, lf, ff in (("m_N_Pa", F(1), F(1)), ("mm_N_MPa", F(1000), F(1)), ("m_kN_kPa", F(1), F(1, 1000))):
        sf = ff / (lf * lf)
        s = annulus(F(1, 10) * lf, F(9, 100) * lf)
        E = SB_E * sf
        Gs = E / (2 * (1 + SB_NU))
        Ksx = Q.of(Gs) * Q.of(kappa_energy(F(9, 10), SB_NU)) * s["A"]
        c = cantilever_plane(Q.of(E) * s["I"], Ksx, lf, tip_force=Q.of(100 * ff))
        systems[label] = {"Ks": qexpr(Ksx), "tip_v": qexpr(c["tip_v"]), "tip_theta": qexpr(c["tip_theta"]),
                          "root_Mz": qexpr(c["root_Mz"]), "Ks_in_N": qexpr(Ksx / ff), "tip_v_in_m": qexpr(c["tip_v"] / lf),
                          "root_Mz_in_N_m": qexpr(c["root_Mz"] / (ff * lf))}
    base = systems["m_N_Pa"]
    for v in systems.values():
        assert (v["Ks_in_N"], v["tip_v_in_m"], v["tip_theta"], v["root_Mz_in_N_m"]) == (
            base["Ks_in_N"], base["tip_v_in_m"], base["tip_theta"], base["root_Mz_in_N_m"])
    out["unit_scaling"] = {"systems": systems, "result": "after normalization the same physical Ks, displacement, "
                                                         "rotation and reaction in every system, exactly"}
    c1 = cantilever_plane(EIp, Q.of(G) * Q.of(ke) * sb["A"], 1, tip_force=100)
    c2 = cantilever_plane(2 * EIp, Q.of(2 * G) * Q.of(ke) * sb["A"], 1, tip_force=100)
    assert c2["tip_v"] == c1["tip_v"] / 2 and c2["tip_theta"] == c1["tip_theta"] / 2
    assert (c2["root_Fy"], c2["root_Mz"]) == (c1["root_Fy"], c1["root_Mz"])
    s1 = spring_case_rational(F(1), F(12), F(12))
    s2 = spring_case_rational(F(2), F(24), F(12))
    assert s2["R"] != s1["R"]
    out["material_scaling"] = {
        "S_B_L1_E_doubled": {"tip_v_ratio": qexpr(c2["tip_v"] / c1["tip_v"]), "reactions_unchanged": True},
        "S_E_counterexample_beam_E_doubled_k_fixed": {"R_before": qexpr(s1["R"]), "R_after": qexpr(s2["R"]),
                                                      "root_force_after": qexpr(s2["root_F"]),
                                                      "note": "a fixed spring does not scale with the beam: the "
                                                              "reaction redistributes"}}
    nu2 = F(1, 4)
    G2 = SB_E / (2 * (1 + nu2))
    k2 = kappa_energy(F(9, 10), nu2)
    out["nu_coherence"] = {"nu=0.3": {"G_Pa": fstr(G), "kappa_E": fstr(ke), "Ks_N": qexpr(Q.of(G * ke) * sb["A"])},
                           "nu=0.25": {"G_Pa": fstr(G2), "kappa_E": fstr(k2), "Ks_N": qexpr(Q.of(G2 * k2) * sb["A"])},
                           "statement": "changing nu changes both G and kappa_E; Ks = kappa_E G A moves by both"}
    return out


def spring_case_rational(EI, Ks, k, L=F(1), a=F(1, 2), Fv=F(1)):
    ch = plane_chain(EI, Ks, L, 2, tip_force=Q.of(Fv), springs=[(1, Q.of(k))])
    ua, tha, ul, thl = ch["d"][2], ch["d"][3], ch["d"][4], ch["d"][5]
    return {"R": -Q.of(k) * ua, "u_a": ua, "theta_a": tha, "u_L": ul, "theta_L": thl, "root_F": ch["root"][0],
            "root_M": ch["root"][1], "chain": ch}


def s_e():
    out = {}
    for label, Ks in (("timoshenko", F(12)), ("euler_bernoulli", None)):
        EI, L, a, Fv, k = F(1), F(1), F(1, 2), F(1), F(12)
        s = spring_case_rational(EI, Ks, k)
        inv = F(0) if Ks is None else 1 / Ks
        c_al = a * a * (3 * L - a) / (6 * EI) + a * inv
        c_aa = a ** 3 / (3 * EI) + a * inv
        c_ll = L ** 3 / (3 * EI) + L * inv
        ua = Fv * c_al / (1 + k * c_aa)
        R = -k * ua
        ul = Fv * c_ll + R * c_al
        thl = (Fv * L * L + R * a * a) / (2 * EI)
        closed = [R, ua, ul, thl, -(Fv + R), -(Fv * L + R * a)]
        mine = [s["R"], s["u_a"], s["u_L"], s["theta_L"], s["root_F"], s["root_M"]]
        assert [Q.of(x) for x in closed] == mine
        beam_energy = ZERO
        for (ke, fe, dofs, le), x0 in zip(s["chain"]["elems"], (F(0), a)):
            d = [s["chain"]["d"][g] for g in dofs]
            beam_energy = beam_energy + span_energy(EI, Ks, span_solution(EI, Ks, L / 2, le, d)[2])
        spring_energy = Q.of(k) * s["u_a"] * s["u_a"] / 2
        work = Q.of(Fv) * s["u_L"] / 2
        assert beam_energy + spring_energy == work
        entry = {"order": "[R, u(a), u(L), theta(L), root force, root moment]",
                 "values": [qexpr(x) for x in mine], "theta_a": qexpr(s["theta_a"]),
                 "assembly_equals_flexibility_closed_form": True, "energy_identity": "1/2 F u(L) = sum int (M^2/2EI + "
                 "V^2/2Ks) + 1/2 k u(a)^2, exactly", "work_half_F_uL": qexpr(work),
                 "beam_strain_energy": qexpr(beam_energy), "spring_energy_half_k_ua2": qexpr(spring_energy)}
        if Ks is not None:
            ke, fe, dofs, le = s["chain"]["elems"][0]
            d = [s["chain"]["d"][g] for g in dofs]
            kb = plane_k(EI, None, L / 2)
            wrong = [x - y for x, y in zip(matvec(kb, d), fe)]
            entry["discriminator_euler_bernoulli_recovery"] = {
                "root_member_end_actions_expected": [qexpr(x) for x in s["chain"]["ends"][0]],
                "root_member_end_actions_wrong": [qexpr(x) for x in wrong]}
        out[label] = entry
    assert out["timoshenko"]["values"] == ["-7/8", "7/96", "37/128", "25/64", "-1/8", "-9/16"]
    assert out["euler_bernoulli"]["values"] == ["-5/6", "5/72", "71/288", "19/48", "-1/6", "-7/12"]
    assert (out["timoshenko"]["beam_strain_energy"], out["euler_bernoulli"]["beam_strain_energy"]) == (
        "173/1536", "163/1728")
    out["fixture"] = ("fixed-root cantilever EI = 1, L = 1, linear ground spring k = 12 at a = 1/2, tip force F = 1; "
                      "Timoshenko Ks = 12; two exact elements assembled and solved here")
    return out


def element_controls():
    return {
        "schema_version": "independent.t4_u8_timoshenko_element_controls/1.0.0",
        "unit": "T4-U8 element-level controls (frame kernel / straight_pipe tests)",
        "status": "frozen independent reference; refuted independently on 2026-10-16 with three MINOR findings, two "
                  "repaired in 5e84d520a7 without a value change; an independent refuter confirmed both repairs on "
                  "2026-10-16 and showed that the third finding (an unlisted moved row, TWIN-A Fx) was not a defect: "
                  "the euler_bernoulli_recovery mutant does not move it, since EA/L is the same in both theories; "
                  "not product output",
        "independence": "generators/u8_reference.py imports, reads and runs no product code",
        "basis": "exact rationals (abstract fixtures) and exact Q(pi) (physical companions); every expected value "
                 "derived by this generator; text forms: 'p/q', 'r*pi', 'r/pi', '(n0 + n1*pi)/(d0 + d1*pi)'",
        "conventions": {
            "plane": "d = [v_i, theta_i, v_j, theta_j]; v = uy, theta = rz in x-y; v = uz, theta = -ry in x-z",
            "theta": "section rotation, not centreline slope; gamma = v' - theta; V = Ks gamma; M = EI theta'",
            "end_action": "f_e = [-V(0), -M(0), V(L), M(L)] = K d - feq (node-on-element); feq = -f_e(d = 0, loads)",
            "root_reaction": "support-on-element at the fixed end = f_e at i"},
        "S-A": s_a(), "S-C": s_c(), "S-D": s_d(), "S-E": s_e(),
    }


# ===========================================================================
# Cross-check against SHEAR_REFERENCE CONTRACT section 5 and reference_arithmetic.stdout.
# ===========================================================================

STDOUT_SB = {  # (phi, v_b, v_s, v_tip, theta, slope, v_s/v_b) for F = 100 pi N, as printed (exact)
    1: ("8267189/29412500", "1/51585", "8267189/6068975250000", "125917189/6068975250000", "1/34390",
        "184742189/6068975250000", "8267189/117650000"),
    2: ("8267189/117650000", "8/51585", "8267189/3034487625000", "478867189/3034487625000", "2/17195",
        "714167189/6068975250000", "8267189/470600000"),
    10: ("8267189/2941250000", "200/10317", "8267189/606897525000", "11773267189/606897525000", "10/3439",
         "17655767189/6068975250000", "8267189/11765000000"),
}
STDOUT_REALISTIC = {  # F = 100 N, binary64 printouts: v_b, v_s, v_tip, theta, slope
    1: ("6.170590020040529e-06", "4.336033483823956e-07", "6.604193368422925e-06", "9.255885030060794e-06",
        "9.689488378443189e-06"),
    2: ("4.936472016032423e-05", "8.672066967647911e-07", "5.023192685708902e-05", "3.702354012024318e-05",
        "3.745714346862557e-05"),
    10: ("0.006170590020040529", "4.336033483823956e-06", "0.006174926053524352", "0.0009255885030060794",
         "0.0009260221063544617"),
}
STDOUT_WRONG = {1: ("122334823/125917189", "8267189/37679689"), 2: ("475284823/478867189", "8267189/125917189"),
                10: ("11769684823/11773267189", "8267189/2949517189")}
STDOUT_SPRING_PHYSICAL = ("-14.877537081183531", "1.4877537081183531e-05", "4.7872337812249824e-05",
                          "3.564649239270416e-05", "-85.12246291881647", "-185.12246291881647")
CONTRACT_SPRING_PHYSICAL = ("-14.8775370812", "1.48775370812e-5", "4.78723378122e-5", "3.56464923927e-5",
                            "-85.1224629188", "-185.122462919")


def contract_crosscheck(controls, product):
    src = "CONTRACT section 5 S-A / reference_arithmetic.stdout"
    sa = controls["S-A"]
    agree(src, "K rows", "[[6,3,-6,3],[3,5/2,-3,1/2],[-6,-3,6,-3],[3,1/2,-3,5/2]]", sa["K"],
          sa["K"] == [["6", "3", "-6", "3"], ["3", "5/2", "-3", "1/2"], ["-6", "-3", "6", "-3"],
                      ["3", "1/2", "-3", "5/2"]])
    c = sa["cases"]
    for q, want, got in (("tip force: v, theta, slope, root Fy, Mz", "5/12, 1/2, 7/12, -1, -1",
                          (c["tip_force"]["tip_v_m"], c["tip_force"]["tip_theta_rad"], c["tip_force"]["tip_slope"],
                           c["tip_force"]["root_Fy_N"], c["tip_force"]["root_Mz_N_m"])),
                         ("tip couple: v, theta", "1/2, 1", (c["tip_couple"]["tip_v_m"], c["tip_couple"]["tip_theta_rad"])),
                         ("full span: v, theta, root Fy, Mz", "1/6, 1/6, -1, -1/2",
                          (c["full_span_uniform"]["tip_v_m"], c["full_span_uniform"]["tip_theta_rad"],
                           c["full_span_uniform"]["root_Fy_N"], c["full_span_uniform"]["root_Mz_N_m"])),
                         ("distal half: v, theta, root Fy, Mz", "53/384, 7/48, -1/2, -3/8",
                          (c["distal_half_uniform"]["tip_v_m"], c["distal_half_uniform"]["tip_theta_rad"],
                           c["distal_half_uniform"]["root_Fy_N"], c["distal_half_uniform"]["root_Mz_N_m"]))):
        text = ", ".join(x["exact"] for x in got)
        agree(src, q, want, text, text == want)
    agree(src, "distal-half feq", "[7/64, 13/384, 25/64, -19/384]", c["distal_half_uniform"]["feq_exact_timoshenko"],
          c["distal_half_uniform"]["feq_exact_timoshenko"] == ["7/64", "13/384", "25/64", "-19/384"])
    st = c["distal_half_uniform"]["discriminator_stale_euler_bernoulli_vector"]
    agree(src, "stale EB vector and its tip v, theta", "[3/32, 5/192, 13/32, -11/192] -> 9/64 (54/384), 7/48",
          f"{st['feq']} -> {st['wrong_tip_v_m']['exact']}, {st['wrong_tip_theta_rad']['exact']}",
          st["feq"] == ["3/32", "5/192", "13/32", "-11/192"] and st["wrong_tip_v_m"]["exact"] == "9/64"
          and st["wrong_tip_theta_rad"]["exact"] == "7/48")
    agree(src, "fixed-guided force", "6", c["fixed_guided"]["end_actions"][2], c["fixed_guided"]["end_actions"][2] == "6")
    se = controls["S-E"]
    agree("CONTRACT section 5 S-E", "Timoshenko [R, u(a), u(L), theta(L), root F, root M]",
          "[-7/8, 7/96, 37/128, 25/64, -1/8, -9/16]", se["timoshenko"]["values"],
          se["timoshenko"]["values"] == ["-7/8", "7/96", "37/128", "25/64", "-1/8", "-9/16"])
    eb = se["euler_bernoulli"]["values"]
    agree("CONTRACT section 5 S-E", "Euler-Bernoulli [R, u(L), theta(L), root F, root M]",
          "[-5/6, 71/288, 19/48, -1/6, -7/12]", [eb[0]] + eb[2:], [eb[0]] + eb[2:] == ["-5/6", "71/288", "19/48",
                                                                                     "-1/6", "-7/12"])
    agree("reference_arithmetic.stdout S-E", "Euler-Bernoulli u(a)", "5/72", eb[1], eb[1] == "5/72")
    # S-B section and responses
    m, nu = F(9, 10), F(3, 10)
    G = SB_E / (2 * (1 + nu))
    ke, kc = kappa_energy(m, nu), kappa_cowper(m, nu)
    sb = annulus(F(1, 10), F(9, 100))
    A_pi, I_pi = F(19, 10000), F(3439, 400000000)
    for q, want, got in (("A/pi", "19/10000", fstr(sb["A"].n[1])), ("I/pi", "3439/400000000", fstr(sb["I"].n[1])),
                         ("G", "1000000000000/13", fstr(G)), ("kappa_C", "1277679/2397284", fstr(kc)),
                         ("kappa_E", "16609827/33068756 (= 33.219654/66.137512)", fstr(ke)),
                         ("EI/pi", "1719500", fstr(SB_E * I_pi)),
                         ("Ks/pi", "606897525000000/8267189", fstr(ke * G * A_pi)),
                         ("kappa_E/kappa_C", "7791173/8267189", fstr(ke / kc)),
                         ("shear compliance C relative to E", "-476016/8267189", fstr(ke / kc - 1))):
        agree("reference_arithmetic.stdout S-B", q, want, got, want.split(" ")[0] == got)
    EIp, Ksp = SB_E * I_pi, ke * G * A_pi
    for L, row in STDOUT_SB.items():
        fpi = F(100)
        phi = 12 * EIp / (Ksp * L * L)
        vb, vs = fpi * L ** 3 / (3 * EIp), fpi * L / Ksp
        th, ga = fpi * L * L / (2 * EIp), fpi / Ksp
        got = tuple(fstr(x) for x in (phi, vb, vs, vb + vs, th, th + ga, vs / vb))
        agree("reference_arithmetic.stdout S-B", f"L={L}: phi, v_b, v_s, v_tip, theta, slope, v_s/v_b (F = 100 pi)",
              row, got, got == row)
        case = next(x for x in product if x["case_id"] == f"S-B-L{L}-TIMOSHENKO")
        lc = next(x for x in case["load_cases"] if x["id"] == "case:tip-fy")
        rows = {(r["kind"], r["entity_ref"], r.get("section_basis")): r for r in lc["rows"]}
        v = Decimal(rows[("global_nodal_displacement_y", "node:tip", None)]["decimal"])
        t = Decimal(rows[("global_nodal_rotation_z", "node:tip", None)]["decimal"])
        real = STDOUT_REALISTIC[L]
        mine = [vdec(Q.of(vb) / PI), vdec(Q.of(vs) / PI), v, t, vdec(Q.of(th + ga) / PI)]
        worst = max(abs(a - Decimal(b)) / abs(a) for a, b in zip(mine, real))
        assert rel_close(v, vdec(Q.of(vb + vs) / PI), "1e-38") and rel_close(t, vdec(Q.of(th) / PI), "1e-38")
        agree("reference_arithmetic.stdout S-B realistic F = 100 N (binary64 printouts)",
              f"L={L}: v_b, v_s, v_tip, theta, slope (v_tip and theta from S-B-L{L}-TIMOSHENKO case:tip-fy rows)",
              ", ".join(real), f"exact; worst relative difference {format(worst, '.3e')}", worst < Decimal("1e-15"))
        wrong_solid = (vb + fpi * L / (kappa_cowper(0, nu) * G * A_pi)) / (vb + vs)
        locked = phi / (1 + phi)
        got = (fstr(wrong_solid), fstr(locked))
        agree("reference_arithmetic.stdout S-B", f"L={L}: solid-Cowper ratio, locked linear ratio", STDOUT_WRONG[L],
              got, got == STDOUT_WRONG[L])
    sd = controls["S-D"]["slender_limit"]["rows"]
    want = {"1": ("5/4", "1/2"), "1/100": ("401/400", "1/101"), "1/1000000": ("4000001/4000000", "1/1000001"),
            "1/1000000000000": ("4000000000001/4000000000000", "1/1000000000001")}
    for r in sd:
        got = (r["compliance_ratio_timoshenko_over_eb"]["exact"], r["locked_linear_over_exact"]["exact"])
        agree("reference_arithmetic.stdout S-D", f"phi={r['phi']}: 1 + phi/4, phi/(1 + phi)", want[r["phi"]], got,
              got == want[r["phi"]])
    case = next(x for x in product if x["case_id"] == "S-E-SPRING-TIMOSHENKO")
    cf = case["closed_form_check"]
    mine = [Decimal(cf[k]["decimal"]) for k in ("R_N", "u_a_m", "u_L_m", "theta_L_rad", "root_force_N",
                                                 "root_moment_N_m")]
    worst = max(abs(a - Decimal(b)) / abs(a) for a, b in zip(mine, STDOUT_SPRING_PHYSICAL))
    agree("reference_arithmetic.stdout S-E physical (binary64)", "[R, u(a), u(L), theta(L), root F, root M]",
          list(STDOUT_SPRING_PHYSICAL), f"exact; worst relative difference {format(worst, '.3e')}", worst < Decimal("1e-13"))
    digits_ok = all(Decimal(format(a, "." + str(len(b.replace("-", "").split("e")[0].replace(".", "").lstrip("0")))
                                   + "g")) == Decimal(b) for a, b in zip(mine, CONTRACT_SPRING_PHYSICAL))
    agree("CONTRACT section 5 S-E physical (presentation decimals)", "[R, u(a), u(L), theta(L), root F, root M]",
          list(CONTRACT_SPRING_PHYSICAL), "exact values rounded to the same significant digits", digits_ok)
    i8 = next(x for x in product if x["case_id"] == "I8-CASE2-TIMOSHENKO")
    dval = Decimal(i8["shear_additions"]["node:D"]["shear_addition_m"]["decimal"])
    agree("T4-RV4 REVIEW S-2", "uy(D) shear addition 'about -1.2e-5 m, or 0.3%'", "-1.2e-5 m; 0.3 %",
          f"{dstr(dval)} m; {i8['shear_additions']['relative_to_euler_bernoulli_at_D']}",
          abs(dval + Decimal("1.2e-5")) < Decimal("0.05e-5"))


# ===========================================================================
# Outputs.
# ===========================================================================


def product_cases(t4i8, odwall, requests):
    cases = sb_cases(requests) + se_cases(requests) + [i8_case(t4i8, requests), twin_case(t4i8, odwall, requests)] \
        + load_state_cases(requests)
    return cases


def product_document(cases):
    return {
        "schema_version": "independent.t4_u8_timoshenko_product_cases/1.0.0",
        "unit": "T4-U8 product exact-route references (Timoshenko straight member) and the T4-I8 marked re-freezes",
        "status": "frozen independent reference before any product comparison; refuted independently on 2026-10-16 "
                  "with three MINOR findings, two repaired in 5e84d520a7 without a value change; an independent "
                  "refuter confirmed both repairs on 2026-10-16 and showed that the third finding (an unlisted moved "
                  "row, TWIN-A Fx) was not a defect: the euler_bernoulli_recovery mutant does not move it, since EA/L "
                  "is the same in both theories; not product output, not a qualification or acceptance claim",
        "independence": "generators/u8_reference.py imports, reads and runs no product code; expected values are "
                        "this generator's exact arithmetic, never product observations",
        "sources": {
            "physics": "SHEAR_REFERENCE at f2211ecef6 (CONTRACT.md sections 1-5, INDEPENDENT_REFUTATION.md, "
                       "_run_records/shear_refutation/reference_arithmetic.py and stdout)",
            "design": "execution/PKG-05/DEL-05-03/Design/section-bases-and-timoshenko.md sections 1-3, 2.5, 7, 8",
            "t4_i8": "repaired T4-I8 freeze at 5b07d85cf3 (tracked copy validation/references/t4_i8/), with "
                     "T4-RV4's S-2 re-freeze marks",
            "twin_a_inputs": "core/product_physics/tests/fixtures/pressure_reference/SOURCE_ODWALL_EXPECTATIONS.json "
                             f"(sha256 {ODWALL_SHA256})"},
        "numeric_representation": {
            "exact": "authoritative; Q(pi) text forms 'p', 'p/q', 'r*pi', 'r/pi', 'r*pi^k', "
                     "'(n0 + n1*pi + ...)/(d0 + d1*pi + ...)', and 'sqrt(...)' for a norm outside Q(pi)",
            "decimal": "40 significant digits from the exact form (pi by Machin, 140-digit context)",
            "expected": "binary64 rounding of the exact value converted into the row unit; never a product value",
            "inputs": "S-B, S-E, rotated, dual-basis and load-state cases use their decimal inputs as exact rationals "
                      "(ri = 9/100 m); the requests author the nearest binary64 values (relative effects of order "
                      "1e-16). Twin A uses the fixture's exact binary64 inputs, as the fixture does"},
        "criteria": {
            "modes": ["sparse_interactive", "dense_scrutiny"], "same_reference_both_modes": True,
            "nonzero": "|observed - expected| <= 1e-9 |expected| (criterion.kind relative)",
            "zero": "|observed| <= criterion.absolute_tolerance = 1e-9 |zero_scales[criterion.scale]| in the row unit",
            "negative": "for every discriminator row: |observed - wrong| > 1e-9 max(|observed|, |wrong|); every "
                        "listed wrong value is separated from the expected value by more than 1e-7 relative",
            "no_new_threshold": "the existing 1e-9 relative criterion only"},
        "row_conventions": {
            "key": "(kind, entity_ref, location, component) and section_basis on stress rows; basis_ref is the load case",
            "stress_rows": "published once per section basis (nominal_section then reduced_section); with zero mill "
                           "tolerance and allowance both carry the same value",
            "end_rows": "element_local force rows at end_i/end_j: node-on-element end actions, element-local DOF "
                        "sense (T4-I8 section 1)",
            "station_rows": "quarter_1/midspan/quarter_3: the section-cut action on the +x face of the i-side segment",
            "stress": "element_local stress rows (MPa) at all five locations from the +x-face section action: "
                      "axial N/A_b, bending M_y/Z_b and M_z/Z_b, torsional T r_o/J_b",
            "maximum": "pipe_elastic_normal_stress_maximum_v2 (Pa, governing_station): max over the member of "
                       "|N/A_b| (|Nw/A_b| when pressurized) + hypot(M_y, M_z)/Z_b",
            "pressure_rows": "pressurized members: pipe_wall_axial_force_v2 = Nw, pipe_effective_axial_force_v2 = "
                             "Nw - pAi, membrane Nw/A_b and Lame rows per basis, pipe_wall_endpoint_action_v2 "
                             "-Nw/+Nw; no element_local_axial_* rows (T4-I8 section 1)",
            "displacements": "global, mm; rotations global, rad, the Timoshenko section rotation theta",
            "reactions": "support_reaction_component_v2: support-on-pipe, global, R = K u - f; a spring's reaction "
                         "is -k u in its DOF; magnitudes are the force and moment norms",
            "status": "on Timoshenko cases each row states shear_model: shear_dependent or same_as_euler_bernoulli "
                      "(against the document's own Euler-Bernoulli solve); I8 and twin-A rows also state their "
                      "T4-I8 / fixture standing",
            "inferred": "element_local_axial_force and element_local_axial_normal_stress (unpressurized members) "
                        "follow the pattern of the other element_local kinds (component = kind without the "
                        "element_local_ prefix; N and MPa; end rows node-on-element). T4-I8 records no row of these "
                        "two kinds; they are nonzero only in the rotated cases"},
        "discriminator_catalogue": MUTANT_CATALOGUE,
        "stale_euler_bernoulli_load_vector": STALE_VECTOR_NOTE,
        "cases": cases,
    }


def sha256sums(files):
    lines = [f"{hashlib.sha256(data).hexdigest()}  {path}" for path, data in sorted(files.items())]
    return ("\n".join(lines) + "\n").encode()


def build():
    CROSS.clear()
    t4i8_bytes = T4_I8_PATH.read_bytes()
    t4i8 = json.loads(t4i8_bytes)
    odwall_bytes = ODWALL_PATH.read_bytes()
    if hashlib.sha256(odwall_bytes).hexdigest() != ODWALL_SHA256:
        raise SystemExit("SOURCE_ODWALL_EXPECTATIONS.json changed")
    odwall = json.loads(odwall_bytes)
    requests = {}
    cases = product_cases(t4i8, odwall, requests)
    controls = element_controls()
    contract_crosscheck(controls, cases)
    controls["crosscheck"] = list(CROSS)
    files = dict(requests)
    files["product_cases.json"] = dumps_compact(product_document(cases)).encode()
    files["element_controls.json"] = dumps_compact(controls).encode()
    for name in ("product_cases.json", "element_controls.json"):
        assert json.loads(files[name]) == json.loads(dumps(product_document(cases) if name == "product_cases.json"
                                                           else controls))
    for fixed in ("README.md", "generators/u8_reference.py"):
        p = OUT / fixed
        if p.exists():
            files[fixed] = p.read_bytes()
    files["SHA256SUMS"] = sha256sums({k: v for k, v in files.items() if k != "SHA256SUMS"})
    return files, cases


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--write", action="store_true")
    args = parser.parse_args()
    files, cases = build()
    bad = [c for c in CROSS if not c["agrees"]]
    rows = sum(len(lc["rows"]) for c in cases for lc in c["load_cases"])
    discs = sum(len(d["rows"]) for c in cases for lc in c["load_cases"] for d in lc["discriminators"])
    print(f"{len(cases)} product cases, {sum(len(c['load_cases']) for c in cases)} load cases, {rows} rows, "
          f"{discs} discriminator rows; {len([k for k in files if k.startswith('requests/')])} requests")
    print(f"cross-checks: {len(CROSS)} recorded, {len(bad)} disagreeing")
    for c in bad:
        print("DISAGREES:", json.dumps(c))
    if args.write:
        for path, data in files.items():
            if path in ("README.md", "generators/u8_reference.py"):
                continue
            target = OUT / path
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(data)
        print(f"wrote {len(files) - 2} files")
        return 0 if not bad else 1
    failures = []
    for path, data in files.items():
        target = OUT / path
        if not target.exists() or target.read_bytes() != data:
            failures.append(path)
    committed = {str(p.relative_to(OUT)) for p in OUT.rglob("*") if p.is_file() and "__pycache__" not in p.parts}
    extra = sorted(committed - set(files))
    if failures or extra or bad:
        print("MISMATCH: " + ", ".join(failures + [f"unexpected {e}" for e in extra]))
        return 1
    print(f"verified {len(files)} files byte for byte (SHA256SUMS included)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
