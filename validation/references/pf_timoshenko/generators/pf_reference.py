#!/usr/bin/env python3
"""PF Timoshenko independent references: opt-in Timoshenko straight members on the
pressure-free route (DEL-05-03 Design pf-timoshenko.md sections 1-3 and 5).

Standard-library Python 3 only. This generator imports, reads and runs no product
code. Besides the outputs it compares in check mode it reads only the README (whose
kappa_E figures it checks) and runs one piece of reference code, t4_u4a's arc engine
(validation/references/t4_u4a/u4a_engine.py), to cross-check its arcs. Every expected
value is derived here in exact arithmetic over Q(pi) (rational functions of pi with
rational coefficients; a realized arc's quarter stations add sqrt(2), cos(pi/8) and
sin(pi/8)), from:
  - the element of section-bases-and-timoshenko.md section 3.3: per bending plane
    phi = 12EI/(G A_s L^2), k_vv = 12EI/(L^3(1+phi)), k_vtheta = 6EI/(L^2(1+phi)),
    k_thetatheta = (4+phi)EI/(L(1+phi)), k_thetatheta' = (2-phi)EI/(L(1+phi)); axial
    EA/L and torsion GJ/L unchanged;
  - the shear area A_s = kappa_E A, kappa_E the energy-matched annular factor of
    section 3.1, with nu = E/(2G) - 1 from the authored (unscaled) E and G
    (pf-timoshenko.md section 2) on PF's single annulus r_o = OD/2,
    r_i = OD/2 - (t - c_m);
  - the full-span uniform load vector [wL/2, wL^2/12, wL/2, -wL^2/12] (section 3.4);
  - partitioned elimination through the assembled K for supports, springs and
    imposed support displacements, and DEC-067's support law (DEL-04-01 Design
    kd5-nonlinear-active-set.md section 0) solved by enumerating every support state;
  - for a realized arc (a bend consumed as curved_bend_macro_element), t4_u4a's
    Euler-Bernoulli arc treatment condensed exactly to a 12x12 element (see arc_element).
The frame assembly and solve are this generator's own. The Q(pi) arithmetic and the
row vocabulary follow validation/references/t4_u8/generators/u8_reference.py (reference
code, not product code).

Modes:
  python3 -I pf_reference.py           check: rebuild every output in memory and compare
                                        byte for byte with the committed files
  python3 -I pf_reference.py --write   write the outputs (intentional changes only)
"""
import argparse
import hashlib
import itertools
import json
import struct
import sys
from decimal import Decimal, getcontext, localcontext
from fractions import Fraction as F
from math import isqrt
from pathlib import Path

HERE = Path(__file__).resolve().parent
OUT = HERE.parent

getcontext().prec = 140

# ===========================================================================
# Exact numbers: Q(pi) as normalized quotients of polynomials in pi (after t4_u8).
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
    """Exact text: 'r', 'r*pi', 'r/pi', 'r*pi^k', 'r/pi^k', or '(n0 + n1*pi + ...)/(d0 + d1*pi + ...)'."""
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


def dstr(d):
    return "0" if d == 0 else format(d, ".40g")


def b64(d):
    """The binary64 nearest to the decimal d (an 80-digit intermediate; the residual
    double-rounding risk is far below the 1e-9 criterion)."""
    return 0.0 if d == 0 else float(format(d, ".80e"))


def f64hex(x):
    return struct.pack(">d", x).hex()


class Sqrt:
    """sqrt(radicand), radicand > 0 in Q(pi) and not a square there."""

    def __init__(self, radicand):
        self.radicand = Q.of(radicand)

    def dec(self):
        return self.radicand.dec().sqrt()


class Expr:
    """A positive real outside Q(pi) and not a plain square root (only mutants' wrong
    maxima): an exact text and its decimal."""

    def __init__(self, text, d):
        self.text, self.d = text, d

    def dec(self):
        return self.d


SQRT2_DEC = Decimal(2).sqrt()
C8_DEC = (2 + SQRT2_DEC).sqrt() / 2  # cos(pi/8)
S8_DEC = (2 - SQRT2_DEC).sqrt() / 2  # sin(pi/8)
ALG_NAMES = ("", "sqrt(2)", "cos(pi/8)", "sin(pi/8)")


class Alg:
    """a + b sqrt(2) + c cos(pi/8) + d sin(pi/8) with a, b, c, d in Q(pi): the values of a
    quarter-circle arc's section actions at its stations psi = k pi/8. Only the linear
    operations the rows need are defined."""

    __slots__ = ("v",)

    def __init__(self, v):
        self.v = tuple(Q.of(x) for x in v)

    def __add__(self, o):
        o = o if isinstance(o, Alg) else Alg((o, 0, 0, 0))
        return Alg(tuple(a + b for a, b in zip(self.v, o.v)))

    __radd__ = __add__

    def __neg__(self):
        return Alg(tuple(-a for a in self.v))

    def __sub__(self, o):
        return self + (-(o if isinstance(o, Alg) else Alg((o, 0, 0, 0))))

    def __mul__(self, o):
        return Alg(tuple(a * o for a in self.v))

    __rmul__ = __mul__

    def __truediv__(self, o):
        return Alg(tuple(a / o for a in self.v))

    def dec(self):
        a, b, c, d = (x.dec() for x in self.v)
        return a + b * SQRT2_DEC + c * C8_DEC + d * S8_DEC

    def text(self):
        terms = []
        for x, name in zip(self.v, ALG_NAMES):
            if not x.is_zero():
                terms.append(f"({qexpr(x)})" + (f"*{name}" if name else ""))
        return " + ".join(terms)


def alg_simplify(a):
    """An Alg with no irrational part is returned as its Q(pi) element."""
    return a.v[0] if all(x.is_zero() for x in a.v[1:]) else a


def vdec(v):
    return v.dec() if isinstance(v, (Sqrt, Expr, Alg)) else Q.of(v).dec()


def vzero(v):
    return (not isinstance(v, (Sqrt, Expr, Alg))) and Q.of(v).is_zero()


def vexpr(v):
    if isinstance(v, Expr):
        return v.text
    if isinstance(v, Alg):
        return v.text()
    return f"sqrt({qexpr(v.radicand)})" if isinstance(v, Sqrt) else qexpr(v)


def vabs(v):
    if isinstance(v, (Sqrt, Expr)):
        return v
    if isinstance(v, Alg):
        return -v if v.dec() < 0 else v
    v = Q.of(v)
    return -v if v.sign() < 0 else v


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


def num(v, unit=None):
    """A reference number: exact text, 40-digit decimal and binary64 value."""
    d = vdec(v)
    out = {} if unit is None else {"unit": unit}
    out.update({"exact": vexpr(v), "decimal": dstr(d), "value": b64(d)})
    return out


def rnum(x, unit=None):
    return num(Q.of(F(x)), unit)


# ===========================================================================
# The energy-matched annular shear factor (section-bases-and-timoshenko.md 3.1) and
# the factors the discriminators substitute for it.
# ===========================================================================


def kappa_energy(m, nu):
    """kappa_E, cancellation-free denominator: 6(1+nu)^2(1+m^2)^2 /
    [(7+14nu+8nu^2)(1-m^2)^2 + 48 m^2 (1+nu)^2]."""
    m, nu = F(m), F(nu)
    n1 = 1 + nu
    c = (8 * nu + 14) * nu + 7
    den = c * (1 - m * m) ** 2 + 48 * m * m * n1 * n1
    return 6 * n1 * n1 * (1 + m * m) ** 2 / den


def kappa_energy_contract_form(m, nu):
    """The same factor in SHEAR_REFERENCE CONTRACT's form (algebraically equal)."""
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


KAPPA_MUTANTS = {
    "cowper_factor": lambda m, nu: kappa_cowper(m, nu),
    "five_sixths_factor": lambda m, nu: F(5, 6),
    "unit_shear_factor": lambda m, nu: F(1),
}


def derived_nu(E, G):
    """PF's Poisson ratio: nu = E/(2G) - 1 from the authored, unscaled moduli (exact)."""
    return F(E) / (2 * F(G)) - 1


def derived_nu_binary64(E, G):
    """The same in binary64, E/(2G) - 1. For every material here E/(2G), (E/2)/G and
    (E/G)/2 are the same exact quotient scaled by powers of two, so any of those operation
    orders rounds to this value."""
    return float(E) / (2.0 * float(G)) - 1.0


# ===========================================================================
# Linear algebra.
# ===========================================================================


def gauss_solve(a, b):
    """Exact Gaussian elimination with back substitution (first nonzero pivot)."""
    n = len(b)
    m = [[Q.of(x) for x in row] + [Q.of(b[i])] for i, row in enumerate(a)]
    for col in range(n):
        piv = next((r for r in range(col, n) if not m[r][col].is_zero()), None)
        if piv is None:
            return None
        m[col], m[piv] = m[piv], m[col]
        pv = m[col][col]
        prow = m[col]
        nz = [k for k in range(col, n + 1) if not prow[k].is_zero()]
        for r in range(col + 1, n):
            if m[r][col].is_zero():
                continue
            f = m[r][col] / pv
            row = m[r]
            for k in nz:
                row[k] = row[k] - f * prow[k]
    x = [ZERO] * n
    for i in range(n - 1, -1, -1):
        s = m[i][n]
        for k in range(i + 1, n):
            if not m[i][k].is_zero() and not x[k].is_zero():
                s = s - m[i][k] * x[k]
        x[i] = s / m[i][i]
    return x


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
# The 12-DOF straight member (DOF order per node: ux, uy, uz, rx, ry, rz).
# Plane map (design section 3.3): v = uy, theta = rz (Iz); v = uz, theta = -ry (Iy).
# ===========================================================================

XY = (1, 5, 7, 11)
XZ = (2, 4, 8, 10)
SIGN_XZ = (1, -1, 1, -1)


def plane_k(EI, Ks, L):
    """4x4 bending matrix on (v_i, theta_i, v_j, theta_j); Ks None is Euler-Bernoulli."""
    L = F(L)
    phi = ZERO if Ks is None else 12 * EI / (Ks * L * L)
    c = EI / (L ** 3 * (1 + phi))
    return [[12 * c, 6 * L * c, -12 * c, 6 * L * c],
            [6 * L * c, (4 + phi) * L * L * c, -6 * L * c, (2 - phi) * L * L * c],
            [-12 * c, -6 * L * c, 12 * c, -6 * L * c],
            [6 * L * c, (2 - phi) * L * L * c, -6 * L * c, (4 + phi) * L * L * c]]


def frame_local_k(t, L, mut=frozenset(), euler_bernoulli=False):
    """Local 12x12 of a member from its terms t (euler_bernoulli drops the shear term).
    Mutants: shear_area_in_EA, kappa_on_GJ, swapped_plane_signs."""
    E, G, A, I, J = t["Es"], t["Gs"], t["A"], t["I"], t["J"]
    Ks = None if euler_bernoulli else t["Ks"]
    kappa = t["kappa"]
    ea = E * A * (Q.of(kappa) if ("shear_area_in_EA" in mut and kappa is not None) else ONE)
    gj = G * J * (Q.of(kappa) if ("kappa_on_GJ" in mut and kappa is not None) else ONE)
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
    """Local axes: x from i to j; y = y_reference less its x part, normalized; z = x cross y."""
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
DOFS = ("UX", "UY", "UZ", "RX", "RY", "RZ")
STATIONS = (("end_i", F(0)), ("quarter_1", F(1, 4)), ("midspan", F(1, 2)), ("quarter_3", F(3, 4)), ("end_j", F(1)))


def annulus(ro, ri):
    ro, ri = F(ro), F(ri)
    A = Q.of(ro * ro - ri * ri) * PI
    I = Q.of((ro ** 4 - ri ** 4) / 4) * PI
    return {"ro": ro, "ri": ri, "A": A, "I": I, "J": 2 * I, "Z": I / ro}


def pf_section(mem):
    """PF's single section (pf-timoshenko.md 2): r_o = OD/2, r_i = OD/2 - (t - c_m)."""
    ro = F(mem["od"]) / 2
    return annulus(ro, ro - (F(mem["wall"]) - F(mem["mill"])))


def nominal_section(mem):
    ro = F(mem["od"]) / 2
    return annulus(ro, ro - F(mem["wall"]))


def member_scale(spec, mem):
    for c in spec.get("components", []):
        if c["pipe"] == mem["id"]:
            return F(c["scale"])
    return F(1)


def member_terms(spec, mem, theory, mut):
    """Moduli, section and shear terms of one member, honouring the mutants."""
    mat = spec["materials"][mem["material"]]
    E, G = F(mat["E"]), F(mat["G"])
    nu = derived_nu(E, G)
    s = member_scale(spec, mem)
    sec = nominal_section(mem) if "stiffness_on_nominal_wall" in mut else pf_section(mem)
    shear = (theory == TIMO) != ("euler_bernoulli_despite_timoshenko" in mut
                                 or "timoshenko_despite_euler_bernoulli" in mut)
    t = {"E": E, "G": G, "nu": nu, "s": s, "Es": s * E, "Gs": s * G, "sec": sec, "A": sec["A"], "I": sec["I"],
         "J": sec["J"], "kappa": None, "As": None, "Ks": None}
    if not shear:
        return t
    ksec = nominal_section(mem) if "kappa_on_nominal_wall" in mut else sec
    nu_k = nu
    if "authored_nu_for_kappa" in mut:
        nu_k = F(mat["nu_authored"])
    m = ksec["ri"] / ksec["ro"]
    kappa = kappa_energy(m, nu_k)
    for name, fn in KAPPA_MUTANTS.items():
        if name in mut:
            kappa = fn(m, nu_k)
    As = Q.of(kappa) * ksec["A"]
    Gk = G if "shear_stiffness_unscaled" in mut else s * G
    t.update({"kappa": kappa, "As": As, "Ks": Q.of(Gk) * As, "m_kappa": m, "nu_kappa": nu_k})
    return t


# ===========================================================================
# Realized arcs (curved_bend_macro_element), in exact form. The treatment is t4_u4a's
# (validation/references/t4_u4a/u4a_engine.py): the arc rule of Member.__init__ (the arc bows
# toward +y_reference projected normal to the chord, centre c = (x_i + x_j)/2 - sqrt(R^2 -
# (L/2)^2) n_hat), the tangent frame of Member.axes (x toward j, y toward the centre, z the
# bend-plane normal), uniform loads as a constant global force per unit arc length
# (Member.uniform_resultant), and Line._integrand's complementary energy N n/(EA) + T t/(GJ) +
# k (M.m - T t)/(EI) + n eps0, Euler-Bernoulli with the user's k on both bending planes. Here it
# is condensed to a 12x12 stiffness: the flexibility of end j over end i (clamped) and the
# member-load tip displacement are integrated exactly over a quarter circle, where every
# integral lies in Q(pi). check_arcs_against_u4a_engine runs that engine on the same lines.
# ===========================================================================

PHI_ARC = Q((0, F(1, 2)))  # pi/2: the quarter circle, the only arc angle used here


def _cos_table(m):
    """cos(m pi/8) as coefficients on (1, sqrt(2), cos(pi/8), sin(pi/8))."""
    m %= 16
    h = F(1, 2)
    table = {0: (1, 0, 0, 0), 1: (0, 0, 1, 0), 2: (0, h, 0, 0), 3: (0, 0, 0, 1), 4: (0, 0, 0, 0)}
    if m <= 4:
        return table[m]
    if m <= 8:
        return tuple(-x for x in table[8 - m])
    return _cos_table(16 - m)


def _sin_table(m):
    return _cos_table(4 - m)


class Tri:
    """A function of the arc angle psi: sum of coef * psi^e * cos(n psi) or sin(n psi), coef in Q(pi).
    Keys (e, 'c' | 's', n), n >= 0."""

    __slots__ = ("t",)

    def __init__(self, t=None):
        self.t = {}
        for k, v in (t or {}).items():
            v = Q.of(v)
            if not v.is_zero():
                self.t[k] = v

    @staticmethod
    def of(x):
        return x if isinstance(x, Tri) else Tri({(0, "c", 0): Q.of(x)})

    def __add__(self, o):
        o = Tri.of(o)
        out = dict(self.t)
        for k, v in o.t.items():
            out[k] = out[k] + v if k in out else v
        return Tri(out)

    __radd__ = __add__

    def __neg__(self):
        return Tri({k: -v for k, v in self.t.items()})

    def __sub__(self, o):
        return self + (-Tri.of(o))

    def __rsub__(self, o):
        return Tri.of(o) - self

    def __mul__(self, o):
        if not isinstance(o, Tri):
            o = Q.of(o)
            return Tri({k: v * o for k, v in self.t.items()})
        out = {}

        def put(e, kind, n, v):
            if n < 0:
                n = -n
                if kind == "s":
                    v = -v
            if kind == "s" and n == 0:
                return
            k = (e, kind, n)
            out[k] = out[k] + v if k in out else v

        for (e1, k1, n1), a in self.t.items():
            for (e2, k2, n2), b in o.t.items():
                h = a * b * F(1, 2)
                e = e1 + e2
                if k1 == "c" and k2 == "c":
                    put(e, "c", n1 - n2, h), put(e, "c", n1 + n2, h)
                elif k1 == "s" and k2 == "s":
                    put(e, "c", n1 - n2, h), put(e, "c", n1 + n2, -h)
                elif k1 == "c":
                    put(e, "s", n1 + n2, h), put(e, "s", n2 - n1, h)
                else:
                    put(e, "s", n1 + n2, h), put(e, "s", n1 - n2, h)
        return Tri(out)

    __rmul__ = __mul__

    def integral(self):
        """Exact integral over [0, pi/2]."""
        ph = PHI_ARC
        total = ZERO
        for (e, kind, n), v in self.t.items():
            if n == 0:
                val = [ph, ph * ph / 2, ph * ph * ph / 3][e]
            else:
                sn, cn = [0, 1, 0, -1][n % 4], [1, 0, -1, 0][n % 4]
                n = F(n)
                if kind == "c":
                    val = [Q.of(sn / n), ph * (sn / n) + (cn - 1) / (n * n),
                           ph * ph * (sn / n) + ph * (2 * cn / (n * n)) - 2 * sn / n ** 3][e]
                else:
                    val = [Q.of((1 - cn) / n), ph * (-cn / n) + sn / (n * n),
                           ph * ph * (-cn / n) + ph * (2 * sn / (n * n)) + 2 * (cn - 1) / n ** 3][e]
            total = total + v * val
        return total

    def at(self, k):
        """The value at psi = k pi/8, a Q(pi) element or an Alg."""
        acc = [ZERO] * 4
        for (e, kind, n), v in self.t.items():
            trig = _cos_table(n * k) if kind == "c" else _sin_table(n * k)
            pe = Q((0,) * e + (F(k, 8) ** e,)) if k else (ONE if e == 0 else ZERO)
            for a in range(4):
                if trig[a]:
                    acc[a] = acc[a] + v * pe * trig[a]
        return alg_simplify(Alg(acc))


COS, SIN, PSI = Tri({(0, "c", 1): 1}), Tri({(0, "s", 1): 1}), Tri({(1, "c", 0): 1})


def tdot(a, b):
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]


def arc_geometry(xi, xj, yref, R):
    """Centre and bend-plane axes of a quarter-circle arc by t4_u4a's rule (Member.__init__),
    exactly: every quantity is rational for the arcs used here."""
    xi, xj, y = [F(c) for c in xi], [F(c) for c in xj], [F(c) for c in yref]
    R = F(R)
    d = [b - a for a, b in zip(xi, xj)]
    dd = sum(c * c for c in d)
    yd = sum(a * b for a, b in zip(y, d)) / dd
    yp = [a - yd * b for a, b in zip(y, d)]
    h = fsqrt((R * R - dd / 4) / sum(c * c for c in yp))
    if h is None:
        raise ValueError("arc centre is not rational")
    centre = [(a + b) / 2 - h * c for a, b, c in zip(xi, xj, yp)]
    ri_, rj_ = [a - c for a, c in zip(xi, centre)], [b - c for b, c in zip(xj, centre)]
    if not (sum(c * c for c in ri_) == R * R == sum(c * c for c in rj_) and sum(a * b for a, b in zip(ri_, rj_)) == 0):
        raise ValueError("not a quarter circle")
    ex = [c / R for c in ri_]
    normal = [c / (R * R) for c in cross(ri_, rj_)]
    ey = cross(normal, ex)
    assert ey == [c / R for c in rj_]
    return {"centre": centre, "ex": ex, "ey": ey, "normal": normal, "R": R, "xi": xi, "xj": xj}


def arc_fields(g):
    """Point and tangent frame (t4_u4a Member.point and Member.axes) as functions of psi."""
    c, ex, ey, n, R = g["centre"], g["ex"], g["ey"], g["normal"], g["R"]
    p = [c[k] + (ex[k] * COS + ey[k] * SIN) * R for k in range(3)]
    tx = [ex[k] * (-SIN) + ey[k] * COS for k in range(3)]
    ty = [-(ex[k] * COS + ey[k] * SIN) for k in range(3)]
    tz = [Tri.of(n[k]) for k in range(3)]
    return p, (tx, ty, tz)


def arc_components(Fv, Mv, axes):
    return [tdot(Fv, a) for a in axes] + [tdot(Mv, a) for a in axes]


def arc_load_actions(g, w, p):
    """Section action at psi (j-side on i-side) of a uniform global load w per unit arc length on
    (psi, pi/2], and the resultant of the whole load about x_i."""
    R, c, ex, ey = g["R"], g["centre"], g["ex"], g["ey"]
    rest = Tri.of(PHI_ARC) - PSI  # the angle beyond the cut
    force = [rest * (w[k] * R) for k in range(3)]
    lever = [(rest * c[k] + (ex[k] * (1 - SIN) + ey[k] * COS) * R - rest * p[k]) * R for k in range(3)]
    moment = cross(lever, [Tri.of(x) for x in w])
    xi = g["xi"]
    f_res = [Q.of(x) * R * PHI_ARC for x in w]
    lev0 = [(PHI_ARC * (c[k] - xi[k]) + (ex[k] + ey[k]) * R) * R for k in range(3)]
    m_res = cross(lev0, [Q.of(x) for x in w])
    return force, moment, f_res, m_res


def arc_element(spec, mem, lc, mut):
    """The arc's global 12x12 stiffness, equivalent loads and recovery data. Euler-Bernoulli
    (t4_u4a's integrand); the control mutant timoshenko_arc adds (V.v)/(kappa_E G A)."""
    pos = dict(spec["nodes"])
    t = member_terms(spec, mem, TIMO, mut)
    g = arc_geometry(pos[mem["i"]], pos[mem["j"]], mem["y_ref"], mem["arc"]["R"])
    R = g["R"]
    k_flex = F(1) if "arc_flexibility_factor_ignored" in mut else F(mem["arc"]["k"])
    E, G, A, I, J = Q.of(t["Es"]), Q.of(t["Gs"]), t["A"], t["I"], t["J"]
    shear = "timoshenko_arc" in mut
    comp = [1 / (E * A), (1 / (G * A * t["kappa"])) if shear else ZERO, (1 / (G * A * t["kappa"])) if shear else ZERO,
            1 / (G * J), k_flex / (E * I), k_flex / (E * I)]
    p, axes = arc_fields(g)
    xj = g["xj"]
    arm = [Tri.of(xj[k]) - p[k] for k in range(3)]
    B = []
    for q in range(6):
        e = [Tri.of(1 if r == q % 3 else 0) for r in range(3)]
        if q < 3:
            B.append(arc_components(e, cross(arm, e), axes))
        else:
            B.append(arc_components([Tri()] * 3, e, axes))
    Fm = [[sum(((B[q][s] * B[r][s]).integral() * comp[s] for s in range(6) if not comp[s].is_zero()), ZERO) * R
           for r in range(6)] for q in range(6)]
    w = [ZERO, ZERO, ZERO]
    for (pid, direction, val, _cat) in lc.get("uniform", []):
        if pid == mem["id"]:
            w[DIRS[direction]] = w[DIRS[direction]] + Q.of(val)
    eps0 = ZERO
    for (pid, dT) in lc.get("thermal", []):
        if pid == mem["id"]:
            eps0 = eps0 + Q.of(F(spec["materials"][mem["material"]]["alpha"]) * F(dT))
    S0F, S0M, f_res, m_res = arc_load_actions(g, w, p)
    S0 = arc_components(S0F, S0M, axes)
    delta0 = []
    for q in range(6):
        val = sum(((B[q][s] * S0[s]).integral() * comp[s] for s in range(6) if not comp[s].is_zero()), ZERO)
        if not eps0.is_zero():
            val = val + B[q][0].integral() * eps0
        delta0.append(val * R)
    cols = [gauss_solve(Fm, [ONE if r == c else ZERO for r in range(6)]) for c in range(6)]
    Kjj = [[cols[c][r] for c in range(6)] for r in range(6)]
    d = [b - a for a, b in zip(g["xi"], xj)]
    Gam = [[ZERO] * 6 for _ in range(6)]
    for r in range(6):
        Gam[r][r] = ONE
    dx = [[0, -d[2], d[1]], [d[2], 0, -d[0]], [-d[1], d[0], 0]]
    for r in range(3):
        for c in range(3):
            Gam[r][3 + c] = Q.of(-dx[r][c])

    def mm(a, b):
        return [[sum((a[r][s] * b[s][c] for s in range(6)), ZERO) for c in range(6)] for r in range(6)]

    GamT = [[Gam[c][r] for c in range(6)] for r in range(6)]
    KG = mm(Kjj, Gam)
    Kii = mm(GamT, KG)
    Kij = [[-x for x in row] for row in mm(GamT, Kjj)]
    Kji = [[-x for x in row] for row in KG]
    kg = [Kii[r] + Kij[r] for r in range(6)] + [Kji[r] + Kjj[r] for r in range(6)]
    kd0 = matvec(Kjj, delta0)
    gkd0 = matvec(GamT, kd0)
    feq = [-gkd0[r] + (f_res + m_res)[r] for r in range(6)] + kd0
    return {"kg": kg, "feq": feq, "Kjj": Kjj, "Gam": Gam, "GamT": GamT, "delta0": delta0, "S0F": S0F, "S0M": S0M,
            "f_res": f_res, "m_res": m_res, "geom": g, "p": p, "axes": axes, "arm": arm, "t": t, "k_flex": k_flex,
            "arc_shear": shear}


ARC_STATIONS = (("end_i", 0), ("quarter_1", 1), ("midspan", 2), ("quarter_3", 3), ("end_j", 4))


def arc_recover(ae, dg):
    """Node-on-element end actions from the solved displacements, and the tangent-frame section
    action (j-side on i-side) at the five stations psi = 0, pi/8, pi/4, 3pi/8, pi/2."""
    ui, uj = dg[0:6], dg[6:12]
    gui = matvec(ae["Gam"], ui)
    rel = [uj[r] - gui[r] - ae["delta0"][r] for r in range(6)]
    fj = matvec(ae["Kjj"], rel)
    gfj = matvec(ae["GamT"], fj)
    fi = [-gfj[r] - (ae["f_res"] + ae["m_res"])[r] for r in range(6)]
    SF = [ae["S0F"][k] + fj[k] for k in range(3)]
    fjF = [Tri.of(x) for x in fj[0:3]]
    SM = [ae["S0M"][k] + fj[3 + k] for k in range(3)]
    SM = [SM[k] + v for k, v in enumerate(cross(ae["arm"], fjF))]
    S = arc_components(SF, SM, ae["axes"])
    faces = {name: [s.at(k) for s in S] for name, k in ARC_STATIONS}
    f = [-v for v in faces["end_i"]] + list(faces["end_j"])
    # f_i and f_j projected on the end frames must equal -S(0) and +S(pi/2)
    ends_ok = True
    for (fv, k, sign) in ((fi, 0, -1), (fj, 4, 1)):
        ax = [[c.at(k) for c in axis] for axis in ae["axes"]]
        loc = [sum((Q.of(fv[a]) * ax[r][a] for a in range(3)), ZERO) for r in range(3)] + \
              [sum((Q.of(fv[3 + a]) * ax[r][a] for a in range(3)), ZERO) for r in range(3)]
        ends_ok &= all(x == sign * y for x, y in zip(loc, faces["end_i" if k == 0 else "end_j"]))
    return {"f": f, "faces": faces, "f_global_i": fi, "f_global_j": fj, "ends_ok": ends_ok}


# ===========================================================================
# Assembly and solve.
# ===========================================================================


def assemble(spec, lc, theory, mut):
    nodes = spec["nodes"]
    nidx = {nid: k for k, (nid, _) in enumerate(nodes)}
    pos = dict(nodes)
    ndof = 6 * len(nodes)
    K = [[ZERO] * ndof for _ in range(ndof)]
    Fv = [ZERO] * ndof
    mres = {}
    for mem in spec["members"]:
        dofs = [6 * nidx[mem["i"]] + k for k in range(6)] + [6 * nidx[mem["j"]] + k for k in range(6)]
        if "arc" in mem:
            ae = arc_element(spec, mem, lc, mut)
            for a in range(12):
                Fv[dofs[a]] = Fv[dofs[a]] + ae["feq"][a]
                for b in range(12):
                    if not ae["kg"][a][b].is_zero():
                        K[dofs[a]][dofs[b]] = K[dofs[a]][dofs[b]] + ae["kg"][a][b]
            mres[mem["id"]] = dict(ae["t"], mem=mem, L=Q.of(ae["geom"]["R"]) * PHI_ARC, R=None, kl=None, feq=ae["feq"],
                                   w_loc=None, dofs=dofs, arc=ae, Ks=None)
            continue
        t = member_terms(spec, mem, theory, mut)
        L, R = member_frame(pos[mem["i"]], pos[mem["j"]], mem["y_ref"])
        kl = frame_local_k(t, L, mut)
        w_loc = [ZERO, ZERO, ZERO]
        for (pid, direction, w, _cat) in lc.get("uniform", []):
            if pid == mem["id"]:
                wg = [ZERO, ZERO, ZERO]
                wg[DIRS[direction]] = Q.of(w)
                w_loc = [a + b for a, b in zip(w_loc, to_local(R, wg))]
        feq = [ZERO] * 12
        for plane, comp, signs in ((XY, 1, (1, 1, 1, 1)), (XZ, 2, SIGN_XZ)):
            w = w_loc[comp]
            if w.is_zero():
                continue
            fp = [w * L / 2, w * L * L / 12, w * L / 2, -w * L * L / 12]
            if "swapped_plane_signs" in mut:
                signs = (1, 1, 1, 1)
            for a in range(4):
                feq[plane[a]] = feq[plane[a]] + signs[a] * fp[a]
        feq[0] = feq[0] + w_loc[0] * L / 2
        feq[6] = feq[6] + w_loc[0] * L / 2
        eig = ZERO  # held thermal end force s E A alpha dT (W-15 2.1: the free strain keeps its meaning)
        for (pid, dT) in lc.get("thermal", []):
            if pid == mem["id"]:
                eig = eig + Q.of(t["Es"]) * t["A"] * F(spec["materials"][mem["material"]]["alpha"]) * F(dT)
        feq[0] = feq[0] - eig
        feq[6] = feq[6] + eig
        kg = rotate_k(kl, R)
        fg = []
        for b in range(4):
            fg += to_global(R, feq[3 * b:3 * b + 3])
        for a in range(12):
            Fv[dofs[a]] = Fv[dofs[a]] + fg[a]
            for b in range(12):
                if not kg[a][b].is_zero():
                    K[dofs[a]][dofs[b]] = K[dofs[a]][dofs[b]] + kg[a][b]
        mres[mem["id"]] = dict(t, mem=mem, L=L, R=R, kl=kl, feq=feq, w_loc=w_loc, dofs=dofs)
    for (nid, direction, val) in lc.get("nodal", []):
        g = 6 * nidx[nid] + DIRS[direction]
        Fv[g] = Fv[g] + Q.of(val)
    fixed, springs, nonlinear = {}, [], []
    imposed = {(sid, dof): F(v) for (sid, dof, v) in lc.get("imposed", [])}
    for sup in spec["supports"]:
        if sup["family"] == "spring":
            dof, k = sup["spring"]
            g = 6 * nidx[sup["node"]] + DOFS.index(dof)
            K[g][g] = K[g][g] + Q.of(k)
            springs.append((sup["id"], g, Q.of(k)))
        elif sup["family"] == "nonlinear":
            nonlinear.append(dict(sup["nonlinear"], id=sup["id"], g=6 * nidx[sup["node"]] + DOFS.index(sup["nonlinear"]["dof"])))
        else:
            for r in sup["restraints"]:
                fixed[6 * nidx[sup["node"]] + DOFS.index(r)] = (sup["id"], imposed.pop((sup["id"], r), F(0)))
    if imposed:
        raise ValueError(f"imposed motion on an unrestrained DOF: {imposed}")
    return {"K": K, "F": Fv, "members": mres, "fixed": fixed, "springs": springs, "nonlinear": nonlinear,
            "nidx": nidx, "ndof": ndof}


def solve_state(sysm, states):
    """Solve one support state. states: per nonlinear support (state, direction) with
    state in active, inactive, sticking, sliding. Returns u, r (= K u - f) or None."""
    K, Fv, n = sysm["K"], sysm["F"], sysm["ndof"]
    pres = {g: Q.of(v) for g, (_, v) in sysm["fixed"].items()}
    extra = {}
    for sup, (state, direction) in zip(sysm["nonlinear"], states):
        g = sup["g"]
        if state in ("active", "sticking"):
            if sup["behavior"] == "gap":
                gap = F(sup["gap"])
                pres[g] = Q.of(-gap if sup["closes_when"] == "negative_displacement" else gap)
            else:
                pres[g] = ZERO
        elif state == "sliding":
            extra[g] = Q.of(-direction * F(sup["mu"]) * abs(F(sup["normal"])))
    free = [g for g in range(n) if g not in pres]
    rhs = []
    for i in free:
        s = Fv[i] + extra.get(i, ZERO)
        for j, v in pres.items():
            if not K[i][j].is_zero() and not v.is_zero():
                s = s - K[i][j] * v
        rhs.append(s)
    x = gauss_solve([[K[r][c] for c in free] for r in free], rhs)
    if x is None:
        return None
    u = [ZERO] * n
    for g, v in pres.items():
        u[g] = v
    for g, v in zip(free, x):
        u[g] = v
    Ku = matvec(K, u)
    r = [Ku[i] - Fv[i] for i in range(n)]
    return u, r, extra


def support_states(sup):
    if sup["behavior"] in ("one_way", "gap"):
        return [("active", 0), ("inactive", 0)]
    return [("inactive", 0), ("sticking", 0), ("sliding", 1), ("sliding", -1)]


def classify(sysm, states, u, r):
    """'strict', 'weak' or None against DEC-067 (kd5-nonlinear-active-set.md section 0)."""
    weak = False
    for sup, (state, direction) in zip(sysm["nonlinear"], states):
        g = sup["g"]
        if sup["behavior"] == "one_way":
            s = 1 if sup["active_when"] == "positive_reaction" else -1
            value = (s * r[g] if state == "active" else s * u[g]).sign()
        elif sup["behavior"] == "gap":
            gap = F(sup["gap"])
            neg = sup["closes_when"] == "negative_displacement"
            if state == "active":
                value = (r[g] if neg else -r[g]).sign()
            else:
                value = ((u[g] + gap) if neg else (gap - u[g])).sign()
        else:
            normal = abs(F(sup["normal"]))
            if state == "inactive":
                value = -1 if normal > 0 else 1
            elif state == "sticking":
                value = (Q.of(F(sup["mu"]) * normal) - vabs(r[g])).sign()
            else:
                value = (direction * u[g]).sign()
        if value < 0:
            return None
        weak |= value == 0
    return "weak" if weak else "strict"


def solve_model(spec, lc_id, theory, mut=frozenset()):
    lc = next(c for c in spec["load_cases"] if c["id"] == lc_id)
    sysm = assemble(spec, lc, theory, mut)
    combos = list(itertools.product(*[support_states(s) for s in sysm["nonlinear"]]))
    strict, weak = [], []
    for combo in combos:
        solved = solve_state(sysm, combo)
        if solved is None:
            continue
        kind = classify(sysm, combo, solved[0], solved[1])
        if kind == "strict":
            strict.append((combo, solved))
        elif kind == "weak":
            weak.append(combo)
    if len(strict) != 1 or weak:
        raise ValueError(f"{spec['case_id']} {lc_id} {sorted(mut)}: {len(strict)} strict and {len(weak)} weak states")
    combo, (u, r, extra) = strict[0]
    n = sysm["ndof"]
    reactions = {sup["id"]: [ZERO] * 6 for sup in spec["supports"]}
    for g, (sid, _) in sysm["fixed"].items():
        reactions[sid][g % 6] = r[g]
    for sid, g, k in sysm["springs"]:
        reactions[sid][g % 6] = -k * u[g]
    states = {}
    for sup, (state, direction) in zip(sysm["nonlinear"], combo):
        g = sup["g"]
        # the support-on-pipe action at its DOF: the constraint force when engaged, the
        # applied friction force when sliding (= r at that free row), zero when released
        reactions[sup["id"]][g % 6] = r[g] if state in ("active", "sticking", "sliding") else ZERO
        states[sup["id"]] = state if state != "sliding" else f"sliding_{'positive' if direction > 0 else 'negative'}"
    members = {}
    for mid, mr in sysm["members"].items():
        dg = [u[g] for g in mr["dofs"]]
        if "arc" in mr:
            rec = arc_recover(mr["arc"], dg)
            if not mut:
                agree("arc end actions", f"{spec['case_id']} {lc_id} {mid}: f_i and f_j on the end tangent frames "
                      "equal -S(0) and +S(pi/2)", rec["ends_ok"])
            members[mid] = dict(mr, f=rec["f"], faces=rec["faces"], f_global=(rec["f_global_i"], rec["f_global_j"]))
            continue
        dl = []
        for b in range(4):
            dl += to_local(mr["R"], dg[3 * b:3 * b + 3])
        kl = mr["kl"]
        if "euler_bernoulli_recovery" in mut:
            kl = frame_local_k(mr, mr["L"], euler_bernoulli=True)
        f = [s - q for s, q in zip(matvec(kl, dl), mr["feq"])]
        members[mid] = dict(mr, d_local=dl, f=f,
                            faces={name: face_action(f, mr["w_loc"], frac * mr["L"]) for name, frac in STATIONS})
    sol = {"u": {nid: u[6 * k:6 * k + 6] for k, (nid, _) in enumerate(spec["nodes"])}, "members": members,
           "reactions": reactions, "states": states, "spec": spec, "lc": lc, "mutant": bool(mut), "r": r,
           "extra": extra, "sys": sysm}
    if not mut:
        equilibrium_check(sol)
        Km = sysm["K"]
        agree("symmetry", f"{spec['case_id']} {lc_id}: assembled K equals its transpose",
              all(Km[i][j] == Km[j][i] for i in range(n) for j in range(i + 1, n)))
    return sol


def face_action(f, w_loc, x):
    """Section-cut action on the +x face of the i-side segment at local x, from the
    node-on-element i-end action f[0:6] and the uniform member load (statics)."""
    x = F(x)
    Fi, Mi = f[0:3], f[3:6]
    return ([-Fi[k] - w_loc[k] * x for k in range(3)]
            + [-Mi[0], -Mi[1] - x * Fi[2] - x * x * w_loc[2] / 2, -Mi[2] + x * Fi[1] + x * x * w_loc[1] / 2])


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


CROSS = []


def agree(source, quantity, ok, **extra):
    CROSS.append(dict({"source": source, "quantity": quantity, "agrees": bool(ok)}, **extra))
    return ok


def equilibrium_check(sol):
    """Global force and moment balance about the origin: supports, springs and nonlinear
    supports against nodal loads and member-load resultants (exact)."""
    spec, lc = sol["spec"], sol["lc"]
    pos = dict(spec["nodes"])
    fsum, msum = [ZERO] * 3, [ZERO] * 3

    def add(force, moment, at):
        nonlocal fsum, msum
        p = [Q.of(F(c)) for c in at]
        fsum = [a + b for a, b in zip(fsum, force)]
        msum = [a + b + c for a, b, c in zip(msum, moment, cross(p, force))]

    for sup in spec["supports"]:
        r = sol["reactions"][sup["id"]]
        add(r[0:3], r[3:6], pos[sup["node"]])
    for (nid, direction, val) in lc.get("nodal", []):
        v = [ZERO] * 6
        v[DIRS[direction]] = Q.of(val)
        add(v[0:3], v[3:6], pos[nid])
    for mr in sol["members"].values():
        mem = mr["mem"]
        if "arc" in mr:
            add(mr["arc"]["f_res"], mr["arc"]["m_res"], pos[mem["i"]])
            continue
        for (pid, direction, w, _cat) in lc.get("uniform", []):
            if pid == mem["id"]:
                v = [ZERO] * 3
                v[DIRS[direction]] = Q.of(w) * mr["L"]
                mid = [(F(a) + F(b)) / 2 for a, b in zip(pos[mem["i"]], pos[mem["j"]])]
                add(v, [ZERO] * 3, mid)
    ok = all(x.is_zero() for x in fsum + msum)
    agree("statics", f"{spec['case_id']} {lc['id']}: global force and moment balance", ok)
    if not ok:
        raise AssertionError(f"equilibrium fails: {spec['case_id']} {lc['id']}")


def slope_rotations(sol):
    """The theta = v' mutant: the published rotation replaced by the centreline slope
    v' = theta + V/Ks at every unrestrained node where the shear is single-valued."""
    spec = sol["spec"]
    restrained = {s["node"] for s in spec["supports"] if s["family"] not in ("spring",)}
    out = {}
    for nid, _ in spec["nodes"]:
        if nid in restrained:
            continue
        ends = []
        for mr in sol["members"].values():
            if mr["Ks"] is None:
                continue
            if mr["mem"]["j"] == nid:
                face = mr["faces"]["end_j"]
            elif mr["mem"]["i"] == nid:
                face = mr["faces"]["end_i"]
            else:
                continue
            ends.append((mr, face))
        if len(ends) != 1:
            continue
        mr, face = ends[0]
        rl = to_local(mr["R"], sol["u"][nid][3:6])
        rl = [rl[0], rl[1] - face[2] / mr["Ks"], rl[2] + face[1] / mr["Ks"]]
        out[nid] = to_global(mr["R"], rl)
    return out


# ===========================================================================
# Rows (the PF preview-physics row vocabulary; see the README for its source).
# ===========================================================================

REL = 1e-9
SEP = Decimal("1e-7")
FORCE_KINDS = (("element_local_axial_force", "axial_force", 0, "N"),
               ("element_local_shear_force_y", "shear_force_y", 1, "N"),
               ("element_local_shear_force_z", "shear_force_z", 2, "N"),
               ("element_local_torsional_moment", "torsional_moment", 3, "N*m"),
               ("element_local_bending_moment_y", "bending_moment_y", 4, "N*m"),
               ("element_local_bending_moment_z", "bending_moment_z", 5, "N*m"))
UNIT_FACTOR = {("m", "mm"): F(1000), ("Pa", "MPa"): F(1, 1000000)}
NONLINEAR_KINDS = ("support_reaction_force_magnitude_v2", "support_reaction_moment_magnitude_v2",
                   "pipe_elastic_normal_stress_maximum_v2")


def make_row(kind, entity, component, location, ref_unit, row_unit, csys, basis, value, family):
    row = {"kind": kind, "entity_ref": entity, "component": component, "location": location}
    if ref_unit != row_unit and (ref_unit, row_unit) not in UNIT_FACTOR:
        raise ValueError((ref_unit, row_unit))
    row.update({"unit": row_unit, "coordinate_system": csys, "basis_ref": dict(basis),
                "reference_unit": ref_unit, "factor": fstr(UNIT_FACTOR.get((ref_unit, row_unit), F(1))),
                "_value": value, "_family": family})
    return row


def member_maximum(mr, s):
    """max over the member of |N/A| + hypot(M_y, M_z)/Z, when the five-station maximum is
    provably the member maximum; else None (the row is then not frozen):
      - no transverse member load: N and the moment vector are affine in x, so both terms
        are convex and the maximum is at an end;
      - one loaded plane with no axial load: N is constant, the other plane's moment is zero
        along the member, and the loaded plane's quadratic moment has no vertex strictly
        inside other than at a station, so |M| peaks at an end or at that station."""
    w, L, f = mr["w_loc"], mr["L"], mr["f"]
    if not (w[1].is_zero() and w[2].is_zero()):
        if not w[0].is_zero() or not (w[1].is_zero() or w[2].is_zero()):
            return None
        comp = 5 if not w[1].is_zero() else 4
        other = 4 if comp == 5 else 5
        if any(not face_action(f, w, x)[other].is_zero() for x in (0, L / 2, L)):
            return None
        a0, am, a1 = (face_action(f, w, x)[comp] for x in (0, L / 2, L))
        c = (a0 + a1 - 2 * am) * 2 / (L * L)
        b = (a1 - a0) / L - c * L
        if not c.is_zero():
            xv = -b / (2 * c)
            if 0 < xv.dec() < L and (xv / L).rational() not in (F(1, 4), F(1, 2), F(3, 4)):
                return None
    best, bestd = None, None
    for name, frac in STATIONS:
        face = face_action(f, w, frac * L)
        mag = vmag([face[4], face[5]])
        nterm = vabs(face[0] / s["A"])
        if isinstance(mag, Sqrt):
            val = Expr(f"{qexpr(nterm)} + sqrt({qexpr(mag.radicand)})/({qexpr(s['Z'])})",
                       vdec(nterm) + mag.dec() / s["Z"].dec())
        else:
            val = nterm + mag / s["Z"]
        d = vdec(val)
        if bestd is None or d > bestd:
            best, bestd = val, d
    return best


def member_rows(sol, mid, basis, rows, omitted):
    mr = sol["members"][mid]
    s, faces, f = mr["sec"], mr["faces"], mr["f"]
    for kind, comp, k, unit in FORCE_KINDS:
        for loc, _ in STATIONS:
            v = f[k] if loc == "end_i" else (f[6 + k] if loc == "end_j" else faces[loc][k])
            rows.append(make_row(kind, mid, comp, loc, unit, unit, "element_local", basis, v,
                                 "member_force" if unit == "N" else "member_moment"))
    for loc, _ in STATIONS:
        face = faces[loc]
        for kind, comp, val in (
                ("element_local_axial_normal_stress", "axial_normal_stress", face[0] / s["A"]),
                ("element_local_bending_normal_stress_y", "bending_normal_stress_y", face[4] / s["Z"]),
                ("element_local_bending_normal_stress_z", "bending_normal_stress_z", face[5] / s["Z"]),
                ("element_local_torsional_shear_stress", "torsional_shear_stress", face[3] * s["ro"] / s["J"])):
            rows.append(make_row(kind, mid, comp, loc, "Pa", "MPa", "element_local", basis, val, "member_stress"))
    mx = None if "arc" in mr else member_maximum(mr, s)
    if mx is None and "arc" in mr:
        omitted.append({"kind": "pipe_elastic_normal_stress_maximum_v2", "entity_ref": mid,
                        "reason": "a realized arc's maximum is an extremum over the arc angle (T4-U4b), not a "
                                  "station value; not frozen here"})
    elif mx is None:
        omitted.append({"kind": "pipe_elastic_normal_stress_maximum_v2", "entity_ref": mid,
                        "reason": "the five-station maximum is not provably the member maximum here "
                                  "(two loaded planes or an axial member load)"})
    else:
        rows.append(make_row("pipe_elastic_normal_stress_maximum_v2", mid, "maximum_absolute_normal_stress",
                             "governing_station", "Pa", "Pa", "pipe_section", basis, mx, "maximum"))


def nodal_rows(sol, basis, rows, rotations=None):
    names = ("x", "y", "z")
    for nid, _ in sol["spec"]["nodes"]:
        u = sol["u"][nid]
        rot = rotations.get(nid, u[3:6]) if rotations is not None else u[3:6]
        for k in range(3):
            rows.append(make_row(f"global_nodal_displacement_{names[k]}", nid, f"nodal_displacement_{names[k]}",
                                 "node", "m", "mm", "global", basis, u[k], "displacement"))
        for k in range(3):
            rows.append(make_row(f"global_nodal_rotation_{names[k]}", nid, f"nodal_rotation_{names[k]}", "node",
                                 "rad", "rad", "global", basis, rot[k], "rotation"))


def reaction_rows(sol, basis, rows):
    names = ("Fx", "Fy", "Fz", "Mx", "My", "Mz")
    for sup in sol["spec"]["supports"]:
        r = sol["reactions"][sup["id"]]
        for k in range(6):
            unit = "N" if k < 3 else "N*m"
            rows.append(make_row("support_reaction_component_v2", sup["id"], names[k], "node", unit, unit, "global",
                                 basis, r[k], "reaction_force" if k < 3 else "reaction_moment"))
        rows.append(make_row("support_reaction_force_magnitude_v2", sup["id"], "force_magnitude", "node", "N", "N",
                             "global", basis, vmag(r[0:3]), "reaction_force"))
        rows.append(make_row("support_reaction_moment_magnitude_v2", sup["id"], "moment_magnitude", "node", "N*m",
                             "N*m", "global", basis, vmag(r[3:6]), "reaction_moment"))


def all_rows(sol, basis, rotations=None):
    rows, omitted = [], []
    for mem in sol["spec"]["members"]:
        member_rows(sol, mem["id"], basis, rows, omitted)
    nodal_rows(sol, basis, rows, rotations)
    reaction_rows(sol, basis, rows)
    return rows, omitted


def row_key(r):
    return (r["kind"], r["entity_ref"], r["component"], r["location"])


FAMILY_GROUP = {"member_force": "force", "reaction_force": "force", "member_moment": "moment",
                "reaction_moment": "moment", "member_stress": "stress", "maximum": "stress",
                "displacement": "length", "rotation": "rotation"}
FAMILY_SCALE_DEFINITIONS = {
    "scale_force": "max |expected| over this load case's nonzero force rows (N)",
    "scale_moment": "max |expected| over this load case's nonzero moment rows (N*m)",
    "scale_length": "max |expected| over this load case's nonzero displacement rows (m)",
    "scale_rotation": "max |expected| over this load case's nonzero rotation rows (rad)",
    "scale_stress": "max |expected| over this load case's nonzero stress rows (Pa)",
    "fallback": "a family with no nonzero row takes a neighbouring family's scale: force = moment / Lmax, "
                "moment = force * Lmax, length = rotation * Lmax, rotation = length / Lmax (Lmax the longest member)",
}


def member_length(spec, mem):
    """A straight member's length, or a quarter-circle arc's length R pi/2 (Q(pi))."""
    if "arc" in mem:
        return Q.of(F(mem["arc"]["R"])) * PHI_ARC
    pos = dict(spec["nodes"])
    return Q.of(member_frame(pos[mem["i"]], pos[mem["j"]], mem["y_ref"])[0])


def family_tag(r):
    return "scale_" + FAMILY_GROUP[r["_family"]]


def family_scales(spec, rows):
    best = {}
    for r in rows:
        if vzero(r["_value"]):
            continue
        tag = family_tag(r)
        d = abs(vdec(r["_value"]))
        if tag not in best or d > best[tag][1]:
            best[tag] = (vabs(r["_value"]), d)
    out = {tag: v for tag, (v, _) in best.items()}
    lmax = max((member_length(spec, m) for m in spec["members"]), key=lambda v: v.dec())
    for tag, src, op in (("scale_force", "scale_moment", lambda v: v / lmax),
                         ("scale_moment", "scale_force", lambda v: v * lmax),
                         ("scale_length", "scale_rotation", lambda v: v * lmax),
                         ("scale_rotation", "scale_length", lambda v: v / lmax)):
        if tag not in out and src in out and not isinstance(out[src], (Sqrt, Expr)):
            out[tag] = op(out[src])
    return dict(sorted(out.items()))


def finish_rows(rows, scales, status_of=None):
    out = []
    for r in rows:
        r = dict(r)
        v = r.pop("_value")
        tag = family_tag(r)
        r.pop("_family")
        factor = F(r["factor"])
        fd = Decimal(factor.numerator) / Decimal(factor.denominator)
        r["exact"] = vexpr(v)
        r["decimal"] = dstr(vdec(v))
        sc = scales.get(tag)
        tol = None if sc is None else Decimal(REL) * abs(vdec(sc)) * fd
        if vzero(v):
            r["expected"] = 0.0
            r["criterion"] = {"kind": "zero_scale", "scale": tag, "absolute_tolerance": b64(tol)}
        elif abs(vdec(v)) * fd < tol:
            # an exact near-cancellation (phi within ulps of 2): the row's own magnitude is below its
            # family's zero tolerance, so it is judged, like a zero row, on that absolute tolerance
            r["expected"] = b64(vdec(v) * fd)
            r["criterion"] = {"kind": "cancellation_zero_scale", "scale": tag, "absolute_tolerance": b64(tol)}
        else:
            r["expected"] = b64(vdec(v) * fd)
            r["criterion"] = {"kind": "relative", "relative_tolerance": REL}
        if status_of is not None:
            r.update(status_of(row_key(r)))
        out.append(r)
    return out


def separation(a, b):
    da, db = vdec(a), vdec(b)
    if da == db:
        return Decimal(0)
    return abs(da - db) / max(abs(da), abs(db))


WRONG_EXACT_MAX = 160  # longer exact texts of wrong values (rational functions of pi) are given as decimals


def discriminate(expected_rows, wrong_rows):
    exp = {row_key(r): r for r in expected_rows}
    out = []
    for r in wrong_rows:
        e = exp.get(row_key(r))
        if e is None:
            continue
        sep = separation(e["_value"], r["_value"])
        if sep <= SEP:
            continue
        factor = F(r["factor"])
        fd = Decimal(factor.numerator) / Decimal(factor.denominator)
        entry = {"kind": r["kind"], "entity_ref": r["entity_ref"], "component": r["component"],
                 "location": r["location"], "unit": r["unit"]}
        text = vexpr(r["_value"])
        if len(text) <= WRONG_EXACT_MAX:
            entry["wrong_exact"] = text
        entry.update({"wrong_decimal": dstr(vdec(r["_value"])), "wrong": b64(vdec(r["_value"]) * fd),
                      "expected": b64(vdec(e["_value"]) * fd), "relative_separation": float(format(sep, ".6e"))})
        out.append(entry)
    return out


def combine_rows(parts, basis):
    """A mechanics combination: sum of factor x row over the terms, for the linear row kinds.
    Magnitudes and the stress maximum are not linear in the terms and are not frozen."""
    acc, order, proto = {}, [], {}
    for factor, rows in parts:
        for r in rows:
            if r["kind"] in NONLINEAR_KINDS:
                continue
            k = row_key(r)
            if k not in acc:
                acc[k] = ZERO
                order.append(k)
                proto[k] = r
            acc[k] = acc[k] + Q.of(F(factor)) * Q.of(r["_value"])
    out = []
    for k in order:
        r = dict(proto[k])
        r["basis_ref"] = dict(basis)
        r["_value"] = acc[k]
        out.append(r)
    return out


# ===========================================================================
# Fixtures: invented SI controls (not catalog, manufacturer, library or code-rule data).
# ===========================================================================

PROV = "invented PF Timoshenko reference fixture; not library, catalog, manufacturer or code-rule data"
UNITS = {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}
STATUS = {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing",
          "professional_acceptance": "not_provided"}
TIMO = "timoshenko_annulus_energy_v1"
EB = "euler_bernoulli"

E_A, G_A = F(200000000000), F(77000000000)  # nu = E/(2G) - 1 = 23/77 exactly
ALPHA = F(12, 1000000)
MAT_A = {"mat:A": {"E": E_A, "G": G_A, "nu_authored": None, "alpha": None}}
MAT_A_THERMAL = {"mat:A": {"E": E_A, "G": G_A, "nu_authored": None, "alpha": ALPHA}}
SEC = {"od": F(1, 5), "wall": F(1, 100), "mill": F(0)}          # r_o 0.1, r_i 0.09
SEC_CM = {"od": F(1, 5), "wall": F(12, 1000), "mill": F(2, 1000)}  # the same r_i 0.09 from t - c_m
G_ACC = F(980665, 100000)
RHO_METAL, RHO_CONTENTS = F(7850), F(1000)


def anchor(sid, node, restraints=DOFS):
    """A rigid restraint in the anchor family: all six DOFs, or a partial set that includes a
    rotation (the product refuses a rotation in the guide family: "Rx is not valid for Guide")."""
    return {"id": sid, "node": node, "family": "anchor", "restraints": list(restraints)}


def guide(sid, node, restraints):
    return {"id": sid, "node": node, "family": "guide", "restraints": list(restraints)}


def spring(sid, node, dof, k):
    return {"id": sid, "node": node, "family": "spring", "restraints": [dof], "spring": (dof, F(k))}


def pipe(pid, i, j, sec, yref, material="mat:A"):
    return dict(sec, id=pid, i=i, j=j, material=material, y_ref=tuple(F(c) for c in yref))


def cantilever_spec(case_id, L, sec, load_cases, materials=MAT_A):
    return {"case_id": case_id, "materials": materials,
            "nodes": [("node:A", (F(0), F(0), F(0))), ("node:B", (F(L), F(0), F(0)))],
            "members": [pipe("pipe:AB", "node:A", "node:B", sec, (0, 1, 0))],
            "supports": [anchor("support:A", "node:A")], "load_cases": load_cases}


def sb_load_cases():
    return [{"id": "case:tip_y", "nodal": [("node:B", "global_y", F(100))]},
            {"id": "case:tip_y_reversed", "nodal": [("node:B", "global_y", F(-100))]},
            {"id": "case:tip_z", "nodal": [("node:B", "global_z", F(100))]},
            {"id": "case:tip_couple_z", "nodal": [("node:B", "rotation_z", F(100))]}]


def ss_mid_spec(case_id):
    return {"case_id": case_id, "materials": MAT_A,
            "nodes": [("node:A", (F(0), F(0), F(0))), ("node:M", (F(1), F(0), F(0))), ("node:B", (F(2), F(0), F(0)))],
            "members": [pipe("pipe:AM", "node:A", "node:M", SEC, (0, 1, 0)),
                        pipe("pipe:MB", "node:M", "node:B", SEC, (0, 1, 0))],
            "supports": [anchor("support:A", "node:A", ("UX", "UY", "UZ", "RX")),
                         guide("support:B", "node:B", ("UY", "UZ"))],
            "load_cases": [{"id": "case:w_y", "uniform": [("pipe:AM", "global_y", F(-1000), "distributed_force"),
                                                          ("pipe:MB", "global_y", F(-1000), "distributed_force")]},
                           {"id": "case:w_z", "uniform": [("pipe:AM", "global_z", F(-1000), "distributed_force"),
                                                          ("pipe:MB", "global_z", F(-1000), "distributed_force")]}]}


# phi = 2: OD 0.25 m and t 1/64 m (r_o 1/8, r_i 7/64, m 7/8), both binary64-exact. With
# r = E/G, phi = 2 r_o^2 [(2r^2 - r + 1)(1-m^2)^2 + 12 m^2 r^2] / (r (1+m^2) L^2), so phi = 2
# at L*^2 = r_o^2 X with X = [(2r^2 - r + 1)(1-m^2)^2 + 12 m^2 r^2]/(r(1+m^2)); L* is irrational
# here (no small rational point on that genus-1 condition was found), so the cases author the
# binary64 nearest L* and its neighbours two ulps away, and freeze phi exactly for each.
SEC_PHI2 = {"od": F(1, 4), "wall": F(1, 64), "mill": F(0)}


def phi2_lengths():
    ro, ri = F(1, 8), F(7, 64)
    m = ri / ro
    r = E_A / G_A
    a, b = 1 + m * m, 1 - m * m
    X = ((2 * r * r - r + 1) * b * b + 12 * m * m * r * r) / (r * a)
    L2 = ro * ro * X
    f = float(Decimal(L2.numerator).sqrt() / Decimal(L2.denominator).sqrt())
    # the binary64 nearest L*: compare squared midpoints exactly
    import math
    while True:
        up, dn = math.nextafter(f, math.inf), math.nextafter(f, -math.inf)
        if ((F(f) + F(up)) / 2) ** 2 < L2:
            f = up
        elif ((F(f) + F(dn)) / 2) ** 2 > L2:
            f = dn
        else:
            break
    near = f
    plus = math.nextafter(math.nextafter(near, math.inf), math.inf)
    minus = math.nextafter(math.nextafter(near, -math.inf), -math.inf)
    return L2, {"NEAREST": near, "PLUS2ULP": plus, "MINUS2ULP": minus}


def phi2_spec(case_id, L, sec=None, materials=None):
    """A simply supported member (torsion held at A) with end couples at A in both planes and a
    uniform load in y: the far-end rotation carries k_thetatheta' = (2 - phi)EI/(L(1+phi)), which
    vanishes at phi = 2, so theta_j under the couple at A is zero there in exact arithmetic."""
    return {"case_id": case_id, "materials": materials or MAT_A,
            "nodes": [("node:A", (F(0), F(0), F(0))), ("node:B", (F(L), F(0), F(0)))],
            "members": [pipe("pipe:AB", "node:A", "node:B", sec or SEC_PHI2, (0, 1, 0))],
            "supports": [anchor("support:A", "node:A", ("UX", "UY", "UZ", "RX")),
                         guide("support:B", "node:B", ("UY", "UZ"))],
            "load_cases": [{"id": "case:couple_a_z", "nodal": [("node:A", "rotation_z", F(100))]},
                           {"id": "case:couple_a_y", "nodal": [("node:A", "rotation_y", F(100))]},
                           {"id": "case:w_y", "uniform": [("pipe:AB", "global_y", F(-1000), "distributed_force")]}]}


def skew_spec(case_id):
    """Four skew spans with rational direction cosines and dyadic coordinates: (2,3,6)/7,
    (1,4,8)/9, (2,-1,2)/3, (6,2,3)/7. Anchors at both ends, a pinned support at N2, an
    interior translational spring at N1 and a rotational spring at N3."""
    n0 = (F(0), F(0), F(0))
    n1 = (F(1, 2), F(3, 4), F(3, 2))
    n2 = (F(5, 8), F(5, 4), F(5, 2))
    n3 = (F(13, 8), F(3, 4), F(7, 2))
    n4 = (F(25, 8), F(5, 4), F(17, 4))
    return {"case_id": case_id, "materials": MAT_A,
            "nodes": [("node:N0", n0), ("node:N1", n1), ("node:N2", n2), ("node:N3", n3), ("node:N4", n4)],
            "members": [pipe("pipe:P1", "node:N0", "node:N1", SEC, (3, -6, 2)),
                        pipe("pipe:P2", "node:N1", "node:N2", SEC, (4, 7, -4)),
                        pipe("pipe:P3", "node:N2", "node:N3", SEC, (2, 2, -1)),
                        pipe("pipe:P4", "node:N3", "node:N4", SEC, (2, 3, -6))],
            "supports": [anchor("support:N0", "node:N0"), spring("spring:N1", "node:N1", "UZ", 2000000),
                         guide("support:N2", "node:N2", ("UX", "UY", "UZ")),
                         spring("spring:N3", "node:N3", "RX", 500000), anchor("support:N4", "node:N4")],
            "load_cases": [{"id": "case:nodal", "nodal": [("node:N3", "global_x", F(300)), ("node:N3", "global_y", F(-200)),
                                                          ("node:N3", "global_z", F(-500)),
                                                          ("node:N1", "rotation_z", F(150))]},
                           {"id": "case:uniform_z", "uniform": [(p, "global_z", F(-800), "distributed_force")
                                                                for p in ("pipe:P1", "pipe:P2", "pipe:P3", "pipe:P4")]}]}


def imposed_spec(case_id):
    """W-16's L frame scaled down: A(0,0,0) anchor, corner C(3/2,0,0), D(3/2,1,0) anchor; D
    moves UX 3 mm, UY -5 mm, UZ 8 mm and RY 0.002 rad (RX, RZ held); a uniform temperature
    rise of 80 degC; and the mechanics combination MOTION + 0.5 THERMAL."""
    return {"case_id": case_id, "materials": MAT_A_THERMAL,
            "nodes": [("node:A", (F(0), F(0), F(0))), ("node:C", (F(3, 2), F(0), F(0))),
                      ("node:D", (F(3, 2), F(1), F(0)))],
            "members": [pipe("pipe:AC", "node:A", "node:C", SEC, (0, 1, 0)),
                        pipe("pipe:CD", "node:C", "node:D", SEC, (-1, 0, 0))],
            "supports": [anchor("support:a", "node:A"), anchor("support:d", "node:D")],
            "load_cases": [{"id": "case:motion", "imposed": [("support:d", "UX", F(3, 1000)),
                                                             ("support:d", "UY", F(-5, 1000)),
                                                             ("support:d", "UZ", F(8, 1000)),
                                                             ("support:d", "RY", F(2, 1000))]},
                           {"id": "case:thermal", "thermal": [("pipe:AC", F(80)), ("pipe:CD", F(80))]}],
            "combinations": [{"id": "combination:motion_plus_half_thermal",
                              "terms": [("case:motion", F(1)), ("case:thermal", F(1, 2))]}]}


SEC_WT = {"od": F(1, 5), "wall": F(1, 100), "mill": F(1, 1000)}  # r_i = 0.1 - 0.009 = 0.091


def weight_intensities():
    """PF's mass method (section-bases-and-timoshenko.md section 1: the preview branch keeps
    source_od_effective_wall_areas/v1, one section of wall t - c_m for every use): metal on
    pi (r_o^2 - r_i^2), contents on the bore pi r_i^2, r_i = OD/2 - (t - c_m)."""
    ro = SEC_WT["od"] / 2
    ri = ro - (SEC_WT["wall"] - SEC_WT["mill"])
    return (Q.of(G_ACC * RHO_METAL * (ro * ro - ri * ri)) * PI, Q.of(G_ACC * RHO_CONTENTS * ri * ri) * PI)


def weight_spec(case_id):
    wm, wc = weight_intensities()
    loads = []
    for p in ("pipe:W1", "pipe:W2", "pipe:W3"):
        loads.append((p, "global_z", -wm, "weight"))
        loads.append((p, "global_z", -wc, "weight"))
    return {"case_id": case_id, "materials": MAT_A,
            "nodes": [("node:W0", (F(0), F(0), F(0))), ("node:W1", (F(2), F(0), F(0))),
                      ("node:W2", (F(7, 2), F(0), F(2))), ("node:W3", (F(7, 2), F(3, 2), F(2)))],
            "members": [pipe("pipe:W1", "node:W0", "node:W1", SEC_WT, (0, 1, 0)),
                        pipe("pipe:W2", "node:W1", "node:W2", SEC_WT, (0, 1, 0)),
                        pipe("pipe:W3", "node:W2", "node:W3", SEC_WT, (-1, 0, 0))],
            "supports": [anchor("support:W0", "node:W0"), guide("support:W1", "node:W1", ("UZ",)),
                         anchor("support:W3", "node:W3")],
            "load_cases": [{"id": "case:self_weight_with_contents", "uniform": loads}]}


def w15_spec(case_id):
    """A cantilever A(0) -> B(1) -> C(3/2) along +X whose last span is the body of a valve at C
    with stiffness scale s = 4 (DEC-045, rigid-component-stiffness-scaling.md 2.1)."""
    return {"case_id": case_id, "materials": MAT_A,
            "nodes": [("node:A", (F(0), F(0), F(0))), ("node:B", (F(1), F(0), F(0))), ("node:C", (F(3, 2), F(0), F(0)))],
            "members": [pipe("pipe:AB", "node:A", "node:B", SEC, (0, 1, 0)),
                        pipe("body:BC", "node:B", "node:C", SEC, (0, 1, 0))],
            "components": [{"id": "component:valve-C", "kind": "valve", "node": "node:C", "pipe": "body:BC",
                            "scale": F(4)}],
            "supports": [anchor("support:A", "node:A")],
            "load_cases": [{"id": "case:tip", "nodal": [("node:C", "global_y", F(100)), ("node:C", "global_z", F(-60)),
                                                        ("node:C", "rotation_x", F(20))]}]}


SEC_THICK = {"od": F(1, 5), "wall": F(4, 100), "mill": F(0)}  # r_i 0.06, m = 3/5


def authnu_spec(case_id):
    mats = {"mat:A": {"E": E_A, "G": G_A, "nu_authored": F(3, 10), "alpha": None}}
    return {"case_id": case_id, "materials": mats,
            "nodes": [("node:A", (F(0), F(0), F(0))), ("node:B", (F(1, 2), F(0), F(0)))],
            "members": [pipe("pipe:AB", "node:A", "node:B", SEC_THICK, (0, 1, 0))],
            "supports": [anchor("support:A", "node:A")],
            "load_cases": [{"id": "case:tip_y", "nodal": [("node:B", "global_y", F(100))]},
                           {"id": "case:tip_z", "nodal": [("node:B", "global_z", F(-100))]}]}


# Exact phi = 2 (a supplementary auxetic control): OD 9/32 m, t 3/32 m (r_o 9/64, r_i 3/64, m = 1/3),
# E 8 GPa, G 9 GPa (nu = E/(2G) - 1 = -5/9, inside kappa_E's domain nu > -1), L 7/32 m: kappa_E = 30/49
# and phi = 12 E I/(G kappa_E A L^2) = 2 exactly. Every input is binary64-exact.
SEC_AUX = {"od": F(9, 32), "wall": F(3, 32), "mill": F(0)}
E_AUX, G_AUX, L_AUX = F(8000000000), F(9000000000), F(7, 32)
MAT_AUX = {"mat:A": {"E": E_AUX, "G": G_AUX, "nu_authored": None, "alpha": None}}

# The realized-arc L-line: A(0,0,0) -> B(1,0,0) straight, B -> C(11/8,3/8,0) a quarter-circle arc of
# R = 3/8 m (a bend component consumed as curved_bend_macro_element, k = 2), C -> D(11/8,11/8,0)
# straight; dyadic coordinates, as t4_u4a's L-LEFT line.
ARC_R, ARC_K = F(3, 8), F(2)


def arc_pipe(pid, i, j, sec, yref, R, k):
    return dict(pipe(pid, i, j, sec, yref), arc={"R": F(R), "k": F(k)})


def arc_line_spec(case_id, anchored_d, load_cases):
    supports = [anchor("support:A", "node:A")] + ([anchor("support:D", "node:D")] if anchored_d else [])
    return {"case_id": case_id, "materials": MAT_A_THERMAL,
            "nodes": [("node:A", (F(0), F(0), F(0))), ("node:B", (F(1), F(0), F(0))),
                      ("node:C", (F(11, 8), F(3, 8), F(0))), ("node:D", (F(11, 8), F(11, 8), F(0)))],
            "members": [pipe("pipe:AB", "node:A", "node:B", SEC, (0, 1, 0)),
                        arc_pipe("pipe:BC", "node:B", "node:C", SEC, (1, -1, 0), ARC_R, ARC_K),
                        pipe("pipe:CD", "node:C", "node:D", SEC, (-1, 0, 0))],
            "supports": supports, "load_cases": load_cases}


def arc_anch_spec(case_id):
    allp = ("pipe:AB", "pipe:BC", "pipe:CD")
    return arc_line_spec(case_id, True, [
        {"id": "case:nodal", "nodal": [("node:C", "global_x", F(400)), ("node:C", "global_y", F(-300)),
                                       ("node:C", "global_z", F(600)), ("node:B", "rotation_z", F(150))]},
        {"id": "case:thermal", "thermal": [(p, F(80)) for p in allp]},
        {"id": "case:uniform_z", "uniform": [(p, "global_z", F(-800), "distributed_force") for p in allp]},
        {"id": "case:arc_in_plane", "uniform": [("pipe:BC", "global_x", F(500), "distributed_force"),
                                                ("pipe:BC", "global_y", F(-300), "distributed_force")]}])


def arc_tip_spec(case_id):
    return arc_line_spec(case_id, False, [
        {"id": "case:tip", "nodal": [("node:D", "global_x", F(300)), ("node:D", "global_y", F(-200)),
                                     ("node:D", "global_z", F(500)), ("node:D", "rotation_y", F(80))]},
        {"id": "case:uniform_z", "uniform": [(p, "global_z", F(-800), "distributed_force")
                                             for p in ("pipe:AB", "pipe:BC", "pipe:CD")]}])


def nl_frame(case_id, supports, load_cases):
    return {"case_id": case_id, "materials": MAT_A,
            "nodes": [("node:A", (F(0), F(0), F(0))), ("node:B", (F(1), F(0), F(0))), ("node:C", (F(2), F(0), F(0)))],
            "members": [pipe("pipe:AB", "node:A", "node:B", SEC, (0, 1, 0)),
                        pipe("pipe:BC", "node:B", "node:C", SEC, (0, 1, 0))],
            "supports": [anchor("support:A", "node:A")] + supports, "load_cases": load_cases}


def nl_oneway_spec(case_id):
    sup = {"id": "support:NL-B", "node": "node:B", "family": "nonlinear", "restraints": [],
           "nonlinear": {"behavior": "one_way", "dof": "UY", "initial_state": "inactive",
                         "active_when": "positive_reaction"}}
    return nl_frame(case_id, [sup], [{"id": "case:pressed", "nodal": [("node:C", "global_y", F(-1000))]},
                                     {"id": "case:lifted", "nodal": [("node:C", "global_y", F(1000))]}])


def nl_gap_spec(case_id):
    sup = {"id": "support:GAP-C", "node": "node:C", "family": "nonlinear", "restraints": [],
           "nonlinear": {"behavior": "gap", "dof": "UY", "initial_state": "inactive",
                         "closes_when": "negative_displacement", "gap": F(1, 2000)}}
    return nl_frame(case_id, [sup], [{"id": "case:closed", "nodal": [("node:C", "global_y", F(-2000))]},
                                     {"id": "case:open", "nodal": [("node:C", "global_y", F(-500))]}])


def nl_friction_spec(case_id):
    sup = {"id": "support:FR-B", "node": "node:B", "family": "nonlinear", "restraints": [],
           "nonlinear": {"behavior": "friction", "dof": "UZ", "initial_state": "sticking",
                         "mu": F(3, 10), "normal": F(1000)}}
    return nl_frame(case_id, [sup], [{"id": "case:stick", "nodal": [("node:C", "global_z", F(80))]},
                                     {"id": "case:slide", "nodal": [("node:C", "global_z", F(200))]}])


# ===========================================================================
# Requests: model document 0.8.0 PF requests (no pressure_contract).
# ===========================================================================


def qty(v, unit):
    return {"value": float(F(v)) if not isinstance(v, Q) else b64(v.dec()), "unit": unit}


def vec(v):
    return {"x": float(F(v[0])), "y": float(F(v[1])), "z": float(F(v[2]))}


def request_doc(spec, shear_member):
    """shear_member: None (member absent: Euler-Bernoulli), EB or TIMO."""
    nodes = [{"id": nid, "position": vec(p), "provenance": PROV} for nid, p in spec["nodes"]]
    pipes = []
    for m in spec["members"]:
        section = {"outside_diameter": qty(m["od"], "m"), "wall_thickness": qty(m["wall"], "m")}
        if m["mill"] != 0:
            section["mill_tolerance"] = qty(m["mill"], "m")
        pipes.append({"id": m["id"], "from": m["i"], "to": m["j"], "material": m["material"],
                      "y_reference": vec(m["y_ref"]), "section": section, "provenance": PROV})
    materials = []
    for mid, mat in spec["materials"].items():
        entry = {"id": mid, "elastic_modulus": qty(mat["E"], "Pa"), "shear_modulus": qty(mat["G"], "Pa")}
        if mat["nu_authored"] is not None:
            entry["poisson_ratio"] = qty(mat["nu_authored"], "1")
        if mat["alpha"] is not None:
            entry["thermal_expansion_coefficient"] = qty(mat["alpha"], "1/degC")
        entry["provenance"] = PROV
        materials.append(entry)
    supports = []
    for s in spec["supports"]:
        entry = {"id": s["id"], "node": s["node"], "family": s["family"], "restraints": list(s["restraints"])}
        if s["family"] == "spring":
            dof, k = s["spring"]
            entry["stiffness"] = {"dof": dof, "value": qty(k, "N/m" if dof.startswith("U") else "N*m/rad")}
        if s["family"] == "nonlinear":
            nl = s["nonlinear"]
            # initial_state is only the active-set seed: each case has exactly one consistent state
            # (support_states), so the converged state and the rows do not depend on it
            out = {"behavior": nl["behavior"], "dof": nl["dof"], "initial_state": nl["initial_state"]}
            if nl["behavior"] == "one_way":
                out["active_when"] = nl["active_when"]
            elif nl["behavior"] == "gap":
                out["closes_when"] = nl["closes_when"]
                out["gap"] = qty(nl["gap"], "m")
            else:
                out["friction_coefficient"] = qty(nl["mu"], "none")
                out["normal_reaction"] = qty(nl["normal"], "N")
            entry["nonlinear"] = out
        entry["provenance"] = PROV
        supports.append(entry)
    components = []
    for m in spec["members"]:
        if "arc" in m:
            components.append({"id": "component:bend-" + m["id"].split(":", 1)[1], "kind": "bend", "node": m["i"],
                               "geometry": {"bend_pipe_ref": m["id"], "bend_radius": qty(m["arc"]["R"], "m"),
                                            "bend_angle": {"value": b64(PHI_ARC.dec()), "unit": "rad"},
                                            "bend_plane_orientation": "plane of the span chord and the pipe "
                                                                      "y_reference; the arc bows toward +y_reference"},
                               "modifiers": {"flexibility_factor_user_value": qty(m["arc"]["k"], "none"),
                                             "source_reference": "invented test value"},
                               "mechanics_interface": {"solver_consumption": "curved_bend_macro_element"},
                               "provenance": PROV})
    for c in spec.get("components", []):
        components.append({"id": c["id"], "kind": c["kind"], "node": c["node"],
                           "geometry": {"rigid_pipe_ref": c["pipe"]},
                           "modifiers": {"stiffness_scaling_user_value": qty(c["scale"], "none"),
                                         "source_reference": "invented_user_entered_stiffness_scale_no_catalog"},
                           "mechanics_interface": {"solver_consumption": "mechanics_geometry_only"},
                           "provenance": PROV})
    cases = []
    for lc in spec["load_cases"]:
        tag = lc["id"].split(":", 1)[1]
        loads = []
        for (nid, direction, val) in lc.get("nodal", []):
            moment = direction.startswith("rotation")
            loads.append({"id": f"load:{tag}:{len(loads)}", "category": "concentrated_moment" if moment
                          else "concentrated_force", "target": {"type": "node", "node": nid}, "direction": direction,
                          "magnitude": qty(val, "N*m" if moment else "N"), "dimension": "moment" if moment else "force",
                          "provenance": PROV})
        for (pid, direction, w, cat) in lc.get("uniform", []):
            loads.append({"id": f"load:{tag}:{len(loads)}", "category": cat, "target": {"type": "element", "pipe": pid},
                          "direction": direction, "magnitude": qty(w, "N/m"), "dimension": "force_per_length",
                          "provenance": PROV})
        for (pid, dT) in lc.get("thermal", []):
            loads.append({"id": f"load:{tag}:{len(loads)}", "category": "thermal",
                          "target": {"type": "element", "pipe": pid}, "direction": "global_z",
                          "magnitude": qty(dT, "degC"), "dimension": "temperature_interval", "provenance": PROV})
        for (sid, dof, val) in lc.get("imposed", []):
            rot = dof.startswith("R")
            loads.append({"id": f"load:{tag}:{len(loads)}", "category": "imposed_displacement",
                          "target": {"type": "support", "support": sid, "dof": dof}, "direction": dof,
                          "magnitude": qty(val, "rad" if rot else "m"),
                          "dimension": "rotation" if rot else "displacement", "provenance": PROV})
        cases.append({"id": lc["id"], "kind": "primitive_user_load", "primitive_loads": loads, "provenance": PROV})
    combinations = [{"id": c["id"], "basis": "mechanics",
                     "terms": [{"load_case": lc, "factor": float(f)} for lc, f in c["terms"]], "provenance": PROV}
                    for c in spec.get("combinations", [])]
    model = {"schema_version": "0.8.0", "document_kind": "openpipestress.product_preview.model",
             "analysis_status": STATUS, "project": {"id": "invented:pf-timoshenko:" + spec["case_id"], "units": UNITS}}
    if shear_member is not None:
        model["model_settings"] = {"shear_model": shear_member}
    model.update({"nodes": nodes, "pipe_segments": pipes, "materials": materials, "supports": supports,
                  "components": components, "load_cases": cases, "combinations": combinations})
    return {"model": model, "materials": []}


def check_request(doc, spec, shear_member):
    """Read the request back and confirm that it encodes the spec: the binary64 inputs are the
    nearest binary64 values of the exact inputs used here."""
    m = doc["model"]
    assert "pressure_contract" not in m
    assert m.get("model_settings", {}).get("shear_model") == shear_member
    for s in m["supports"]:
        assert s["family"] != "guide" or all(d.startswith("U") for d in s["restraints"]), s["id"]
        assert s["family"] != "nonlinear" or s["nonlinear"]["initial_state"] in ("inactive", "sticking"), s["id"]
    for (nid, p), node in zip(spec["nodes"], m["nodes"]):
        assert node["id"] == nid and all(node["position"][a] == float(F(c)) for a, c in zip("xyz", p))
    for mem, pj in zip(spec["members"], m["pipe_segments"]):
        assert pj["section"]["outside_diameter"]["value"] == float(mem["od"])
        assert pj["section"]["wall_thickness"]["value"] == float(mem["wall"])
        assert pj["section"].get("mill_tolerance", {"value": 0.0})["value"] == float(mem["mill"])
    for (mid, mat), mj in zip(spec["materials"].items(), m["materials"]):
        assert F(mj["elastic_modulus"]["value"]) == mat["E"] and F(mj["shear_modulus"]["value"]) == mat["G"]
    return True


# ===========================================================================
# Cases.
# ===========================================================================


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
        if not o or all(not isinstance(x, (dict, list)) for x in o):
            return _inline(o)
        return "[\n" + ",\n".join(f"{pad} {_compact(x, level + 1)}" for x in o) + "\n" + pad + "]"
    return _inline(o)


def dumps_compact(obj):
    """Indented JSON with every row and every number record on one line."""
    return _compact(obj, 0) + "\n"


MUTANT_CATALOGUE = {
    "euler_bernoulli_despite_timoshenko": "the selection ignored: every member Euler-Bernoulli. Its values are the "
                                          "Euler-Bernoulli twin's expected rows, so it is not listed again; the rows "
                                          "it moves are those marked discriminating",
    "timoshenko_despite_euler_bernoulli": "shear added to an Euler-Bernoulli document: its values are the Timoshenko "
                                          "twin's expected rows (the discriminating marks on the twin)",
    "cowper_factor": "Cowper's annular factor 6(1+nu)(1+m^2)^2/[(7+6nu)(1+m^2)^2+(20+12nu)m^2] in place of kappa_E",
    "five_sixths_factor": "kappa = 5/6",
    "unit_shear_factor": "kappa = 1 (A_s = A)",
    "kappa_on_nominal_wall": "kappa_E and A_s on the nominal wall t (r_i = OD/2 - t) while A, I, J use t - c_m",
    "stiffness_on_nominal_wall": "the whole section, kappa_E included, on the nominal wall t (mill tolerance ignored)",
    "authored_nu_for_kappa": "kappa_E from the authored poisson_ratio instead of E/(2G) - 1",
    "swapped_plane_signs": "the x-z plane placed with the x-y plane's signs (theta = +ry)",
    "kappa_on_GJ": "kappa applied to the torsional stiffness GJ",
    "shear_area_in_EA": "A_s in place of A in the axial stiffness",
    "theta_equals_slope": "published rotations replaced by the centreline slope v' = theta + V/Ks at free member ends",
    "euler_bernoulli_recovery": "member end actions recovered with the Euler-Bernoulli K from the Timoshenko "
                                "displacements (reactions from R = K u - f with the Timoshenko K are unaffected)",
    "shear_stiffness_unscaled": "a W-15 body's shear stiffness kappa G A left unscaled while E and G are scaled by s",
    "timoshenko_arc": "the realized arc made Timoshenko too: shear compliance 1/(kappa_E G A) on both transverse "
                      "directions along the arc (the ARC-TIMOSHENKO-CONTROL case's rows)",
    "arc_flexibility_factor_ignored": "the bend's flexibility_factor_user_value ignored (k = 1 on the arc)",
}


def mutant_rows(spec, lc_id, theory, name, rows_basis):
    if name == "theta_equals_slope":
        sol = solve_model(spec, lc_id, theory)
        return all_rows(sol, rows_basis, rotations=slope_rotations(sol))[0]
    return all_rows(solve_model(spec, lc_id, theory, frozenset([name])), rows_basis)[0]


def member_table(spec, theory):
    pos = dict(spec["nodes"])
    out = []
    for mem in spec["members"]:
        t = member_terms(spec, mem, TIMO, frozenset())
        sec = t["sec"]
        if "arc" in mem:
            R = F(mem["arc"]["R"])
            out.append({"id": mem["id"], "kind": "realized_arc (curved_bend_macro_element)", "radius_m": rnum(R),
                        "included_angle_rad": num(PHI_ARC), "arc_length_m": num(Q.of(R) * PHI_ARC),
                        "flexibility_factor_k": rnum(F(mem["arc"]["k"])), "r_o_m": rnum(sec["ro"]),
                        "r_i_m": rnum(sec["ri"]), "m": rnum(sec["ri"] / sec["ro"]), "E_Pa": rnum(t["E"]),
                        "G_Pa": rnum(t["G"]), "nu_derived": rnum(t["nu"]), "A_m2": num(sec["A"]), "I_m4": num(sec["I"]),
                        "applies": "euler_bernoulli in every document (realized arc); the arc-Timoshenko control "
                                   "adds the shear compliance 1/(kappa_E G A) on the arc",
                        "control_kappa_E": rnum(t["kappa"]), "control_A_s_m2": num(t["As"])})
            continue
        L, _ = member_frame(pos[mem["i"]], pos[mem["j"]], mem["y_ref"])
        entry = {"id": mem["id"], "length_m": rnum(L), "r_o_m": rnum(sec["ro"]), "r_i_m": rnum(sec["ri"]),
                 "m": rnum(sec["ri"] / sec["ro"]), "E_Pa": rnum(t["E"]), "G_Pa": rnum(t["G"]),
                 "nu_derived": rnum(t["nu"]), "kappa_E": rnum(t["kappa"]), "A_m2": num(sec["A"]),
                 "I_m4": num(sec["I"]), "A_s_m2": num(t["As"])}
        if t["s"] != 1:
            entry["stiffness_scale"] = rnum(t["s"])
        phi = 12 * (Q.of(t["Es"]) * sec["I"]) / (t["Ks"] * L * L)
        entry["phi"] = num(phi)
        entry["applies"] = "timoshenko" if theory == TIMO else "euler_bernoulli (phi shown for the twin)"
        nu_b = derived_nu_binary64(t["E"], t["G"])
        kb = kappa_binary64(float(sec["ro"]), float(sec["ro"]) - (float(mem["wall"]) - float(mem["mill"])), nu_b)
        entry["binary64"] = {"nu": nu_b, "nu_hex": f64hex(nu_b), "kappa_E_design_order": kb,
                             "kappa_E_design_order_hex": f64hex(kb),
                             "kappa_E_relative_to_exact": float(format(abs(Decimal(kb) - Decimal(t["kappa"].numerator)
                                                                           / Decimal(t["kappa"].denominator))
                                                                       / (Decimal(t["kappa"].numerator)
                                                                          / Decimal(t["kappa"].denominator)), ".3e"))}
        out.append(entry)
    return out


ALT_MARKS = {"authored_nu_for_kappa": "authored_nu_discriminating", "timoshenko_arc": "arc_timoshenko_discriminating"}


def build_case(case_id, family, description, spec_fn, mutants, requests, explicit_eb_twin=False,
               key_values=None, diagnostics=None, extra=None, arc_control=False):
    """A Timoshenko document and its Euler-Bernoulli twin (member absent), optionally also the
    explicit euler_bernoulli twin and the arc-Timoshenko control (rows the product must not
    meet; no document). Returns the case records."""
    t_id, e_id, x_id = f"{case_id}-TIMOSHENKO", f"{case_id}-EULER-BERNOULLI", f"{case_id}-EXPLICIT-EULER-BERNOULLI"
    spec_t, spec_e = spec_fn(t_id), spec_fn(e_id)
    docs = {t_id: (request_doc(spec_t, TIMO), spec_t, TIMO), e_id: (request_doc(spec_e, None), spec_e, None)}
    if explicit_eb_twin:
        spec_x = spec_fn(x_id)
        docs[x_id] = (request_doc(spec_x, EB), spec_x, EB)
    req = {}
    for cid, (doc, spec, member) in docs.items():
        check_request(doc, spec, member)
        data = dumps(doc).encode()
        path = f"requests/{cid}.request.json"
        requests[path] = data
        req[cid] = {"path": path, "sha256": hashlib.sha256(data).hexdigest()}
    lc_t, lc_e = [], []
    sols = {}
    for lc in spec_t["load_cases"]:
        basis = {"ref_type": "load_case", "ref_id": lc["id"]}
        st, se = solve_model(spec_t, lc["id"], TIMO), solve_model(spec_e, lc["id"], None)
        sols[lc["id"]] = (st, se)
        rt, om_t = all_rows(st, basis)
        re_, om_e = all_rows(se, basis)
        lc_t.append((lc["id"], basis, rt, om_t, st))
        lc_e.append((lc["id"], basis, re_, om_e, se))
    for comb in spec_t.get("combinations", []):
        basis = {"ref_type": "combination", "ref_id": comb["id"]}
        ct = combine_rows([(f, next(x[2] for x in lc_t if x[0] == lc)) for lc, f in comb["terms"]], basis)
        ce = combine_rows([(f, next(x[2] for x in lc_e if x[0] == lc)) for lc, f in comb["terms"]], basis)
        lc_t.append((comb["id"], basis, ct, [{"kind": k, "reason": "not linear in the terms"} for k in NONLINEAR_KINDS],
                     None))
        lc_e.append((comb["id"], basis, ce, [{"kind": k, "reason": "not linear in the terms"} for k in NONLINEAR_KINDS],
                     None))
    out_t, out_e, out_c = [], [], []
    for (lid, basis, rt, om_t, st), (_, _, re_, om_e, se) in zip(lc_t, lc_e):
        emap = {row_key(r): r for r in re_}
        tmap = {row_key(r): r for r in rt}
        assert set(emap) == set(tmap), (case_id, lid)
        alt, alt_rows = {}, {}
        for name, mark in ALT_MARKS.items():
            if name in mutants and st is not None:
                sol = solve_model(spec_t, lid, TIMO, frozenset([name]))
                equilibrium_check(sol)
                alt_rows[mark] = all_rows(sol, basis)
                alt[mark] = {row_key(r): r for r in alt_rows[mark][0]}

        def status(key):
            d = {"discriminating": bool(separation(tmap[key]["_value"], emap[key]["_value"]) > SEP)}
            for mark, amap in alt.items():
                d[mark] = bool(separation(tmap[key]["_value"], amap[key]["_value"]) > SEP)
            return d

        scales_t, scales_e = family_scales(spec_t, rt), family_scales(spec_e, re_)
        disc = []
        if st is not None:
            for name in mutants:
                rows = discriminate(rt, mutant_rows(spec_t, lid, TIMO, name, basis))
                disc.append({"mutant": name, "rows_moved": len(rows), "rows": rows})
        else:
            for name in mutants:
                parts = []
                for lc, f in next(c for c in spec_t["combinations"] if c["id"] == lid)["terms"]:
                    parts.append((f, mutant_rows(spec_t, lc, TIMO, name, {"ref_type": "load_case", "ref_id": lc})))
                rows = discriminate(rt, combine_rows(parts, basis))
                disc.append({"mutant": name, "rows_moved": len(rows), "rows": rows})
        rec_t = {"id": lid, "basis_ref": basis}
        rec_e = {"id": lid, "basis_ref": basis}
        if st is not None and st["states"]:
            rec_t["support_states"] = st["states"]
            rec_e["support_states"] = se["states"]
        if key_values:
            kv = key_values(lid, st, se)
            if kv:
                rec_t["key_values"], rec_e["key_values"] = kv
        frt = finish_rows(rt, scales_t, status)
        nd = sum(1 for r in frt if r["discriminating"])
        rec_t.update({"zero_scales": {k: num(v) for k, v in scales_t.items()}, "rows": frt,
                      "discriminating_rows": nd, "omitted_rows": om_t, "discriminators": disc})
        for mark in alt:
            rec_t[f"{mark}_rows"] = sum(1 for r in frt if r[mark])
        fre = finish_rows(re_, scales_e, lambda key: {"discriminating": bool(
            separation(tmap[key]["_value"], emap[key]["_value"]) > SEP)})
        rec_e.update({"zero_scales": {k: num(v) for k, v in scales_e.items()}, "rows": fre,
                      "discriminating_rows": nd, "omitted_rows": om_e})
        out_t.append(rec_t)
        out_e.append(rec_e)
        if arc_control:
            mark = ALT_MARKS["timoshenko_arc"]
            rc, om_c = alt_rows[mark]
            scales_c = family_scales(spec_t, rc)
            frc = finish_rows(rc, scales_c, lambda key: {mark: bool(
                separation(tmap[key]["_value"], alt[mark][key]["_value"]) > SEP)})
            out_c.append({"id": lid, "basis_ref": basis, "zero_scales": {k: num(v) for k, v in scales_c.items()},
                          "rows": frc, f"{mark}_rows": rec_t[f"{mark}_rows"], "omitted_rows": om_c})
    base = {"family": family, "description": description}
    if extra:
        base.update(extra)
    cases = [dict(base, id=t_id, shear_model=TIMO, twin=e_id, request=req[t_id],
                  members=member_table(spec_t, TIMO), expected_diagnostics=(diagnostics or {}).get("timoshenko", []),
                  load_cases=out_t),
             dict(base, id=e_id, shear_model="absent (Euler-Bernoulli)", twin=t_id, request=req[e_id],
                  members=member_table(spec_e, None),
                  expected_diagnostics=(diagnostics or {}).get("euler_bernoulli", []), load_cases=out_e)]
    if explicit_eb_twin:
        cases.append(dict(base, id=x_id, shear_model=EB, twin=t_id, request=req[x_id], same_rows_as=e_id,
                          note="model_settings.shear_model = euler_bernoulli solves exactly as absent "
                               "(pf-timoshenko.md 1): every row equals the absent twin's row"))
    if arc_control:
        cases.append(dict(base, id=f"{case_id}-ARC-TIMOSHENKO-CONTROL",
                          shear_model="control (no document): the TIMOSHENKO document solved with the realized arc "
                                      "also Timoshenko (shear compliance 1/(kappa_E G A) on the arc)",
                          twin=t_id, request=None, must_not_match=True,
                          note="a product given the TIMOSHENKO request must breach these rows wherever "
                               "arc_timoshenko_discriminating is true: PF keeps realized arcs Euler-Bernoulli "
                               "(pf-timoshenko.md 1)",
                          members=member_table(spec_t, TIMO), load_cases=out_c))
    return cases, sols


def eiks(sol, mid):
    mr = sol["members"][mid]
    return Q.of(mr["Es"]) * mr["I"], mr["Ks"], mr["L"]


def kv(sol, nid, k, unit, label):
    return {label: num(sol["u"][nid][k], unit)}


def sb_cases(requests):
    out = []
    lengths = ((F(1, 4), "L0P25", (SEC,)), (F(1), "L1", (SEC, SEC_CM)), (F(2), "L2", (SEC, SEC_CM)),
               (F(10), "L10", (SEC, SEC_CM)), (F(50), "L50", (SEC,)))
    for L, tag, secs in lengths:
        for sec in secs:
            cm = "CM0" if sec["mill"] == 0 else "CM2MM"
            cid = f"PFT-SB-{tag}-{cm}"
            muts = ["cowper_factor", "five_sixths_factor", "unit_shear_factor", "theta_equals_slope",
                    "euler_bernoulli_recovery"]
            if sec["mill"] != 0:
                muts += ["kappa_on_nominal_wall", "stiffness_on_nominal_wall"]

            def keys(lid, st, se, L=L):
                res = []
                for sol in (st, se):
                    EI, Ks, _ = eiks(sol, "pipe:AB")
                    P = F(100) if lid != "case:tip_y_reversed" else F(-100)
                    if lid in ("case:tip_y", "case:tip_y_reversed"):
                        exp_v = P * L ** 3 / (3 * EI) + (ZERO if Ks is None else P * L / Ks)
                        exp_t = P * L * L / (2 * EI)
                        got_v, got_t = sol["u"]["node:B"][1], sol["u"]["node:B"][5]
                    elif lid == "case:tip_z":
                        exp_v = P * L ** 3 / (3 * EI) + (ZERO if Ks is None else P * L / Ks)
                        exp_t = -P * L * L / (2 * EI)
                        got_v, got_t = sol["u"]["node:B"][2], sol["u"]["node:B"][4]
                    else:
                        exp_v, exp_t = P * L * L / (2 * EI), P * L / EI
                        got_v, got_t = sol["u"]["node:B"][1], sol["u"]["node:B"][5]
                    agree("closed form (cantilever: PL^3/3EI + PL/(kappa G A), PL^2/2EI; couple ML^2/2EI, ML/EI)",
                          f"{sol['spec']['case_id']} {lid} tip", got_v == exp_v and got_t == exp_t)
                    res.append({"tip_transverse_m": num(got_v, "m"), "tip_rotation_rad": num(got_t, "rad")})
                return res

            desc = (f"t4_u8's S-B cantilever re-authored on PF: L = {fstr(L)} m along +X, anchored at A, "
                    f"OD 0.2 m, wall {fstr(sec['wall'])} m, mill tolerance {fstr(sec['mill'])} m "
                    f"(r_i = 0.09 m either way), E 200 GPa, G 77 GPa (nu = 23/77); tip force +100 N Y, its reversal, "
                    f"+100 N Z and a +100 N*m couple about Z")
            cases, _ = build_case(cid, "S-B cantilever", desc, lambda c, L=L, sec=sec: cantilever_spec(
                c, L, sec, sb_load_cases()), muts, requests, explicit_eb_twin=(tag == "L1" and cm == "CM0"),
                key_values=keys)
            if cm == "CM2MM":
                # t - c_m = 0.01 m: the same annulus as CM0, so every expected row must be identical;
                # the rows are stored once (on CM0) and this case keeps its own discriminators
                for case in cases:
                    twin = next(c for c in out if c["id"] == case["id"].replace("CM2MM", "CM0"))
                    same = all(a["rows"] == b["rows"] and a["zero_scales"] == b["zero_scales"]
                               for a, b in zip(case["load_cases"], twin["load_cases"]))
                    agree("mill tolerance applied", f"{case['id']}: every row equals {twin['id']}'s", same)
                    case["same_rows_as"] = twin["id"]
                    for lc in case["load_cases"]:
                        del lc["rows"]
                        del lc["zero_scales"]
            out += cases
    return out


def ss_cases(requests):
    def keys(lid, st, se):
        res = []
        for sol in (st, se):
            EI, Ks, _ = eiks(sol, "pipe:AM")
            w, L = F(1000), F(2)
            vmid = 5 * w * L ** 4 / (384 * EI) + (ZERO if Ks is None else w * L * L / (8 * Ks))
            th = w * L ** 3 / (24 * EI)
            k, kr, sgn = (1, 5, -1) if lid == "case:w_y" else (2, 4, 1)
            got, ta, tb = sol["u"]["node:M"][k], sol["u"]["node:A"][kr], sol["u"]["node:B"][kr]
            ok = got == -vmid and ta == sgn * th and tb == -sgn * th
            agree("closed form (simply supported span: v_mid = 5wL^4/384EI + wL^2/(8 kappa G A), "
                  "theta_end = wL^3/24EI)", f"{sol['spec']['case_id']} {lid}", ok)
            res.append({"midspan_deflection_m": num(got, "m"), "closed_form_midspan_m": num(-vmid, "m"),
                        "shear_term_wL2_over_8kGA_m": num(ZERO if Ks is None else -w * L * L / (8 * Ks), "m"),
                        "end_rotation_A_rad": num(ta, "rad")})
        return res

    desc = ("simply supported span L = 2 m along +X with a midpoint node (two 1 m members); A restrains UX UY UZ RX, "
            "B restrains UY UZ; full-span uniform load -1000 N/m in global Y, then in global Z")
    cases, _ = build_case("PFT-SS-MID", "simply supported span, midpoint node", desc, ss_mid_spec,
                          ["cowper_factor", "five_sixths_factor", "unit_shear_factor", "euler_bernoulli_recovery"],
                          requests, key_values=keys)
    return cases


def phi2_cases(requests):
    L2, lengths = phi2_lengths()
    out = []
    for tag, Lf in lengths.items():
        L = F(Lf)

        def keys(lid, st, se, L=L):
            res = []
            for sol in (st, se):
                EI, Ks, _ = eiks(sol, "pipe:AB")
                M = F(100)
                rec = {}
                if Ks is not None:
                    phi = 12 * EI / (Ks * L * L)
                    rec["phi"] = num(phi)
                    rec["two_minus_phi"] = num(2 - phi)
                    rec["phi_exact_relation"] = "phi = 2 L*^2/L^2 with L*^2 = " + fstr(L2) + " m^2"
                    agree("phi = 2 L*^2 / L^2", f"{sol['spec']['case_id']}", phi == Q.of(2 * L2 / (L * L)))
                if lid in ("case:couple_a_z", "case:couple_a_y"):
                    sh = ZERO if Ks is None else M / (Ks * L)
                    ta, tb = M * L / (3 * EI) + sh, -M * L / (6 * EI) + sh
                    k = 5 if lid == "case:couple_a_z" else 4
                    ga, gb = sol["u"]["node:A"][k], sol["u"]["node:B"][k]
                    agree("closed form (simply supported end couple: theta_A = ML/3EI + M/(Ks L), theta_B = "
                          "-ML/6EI + M/(Ks L))", f"{sol['spec']['case_id']} {lid}", ga == ta and gb == tb)
                    rec.update({"theta_A_rad": num(ga, "rad"), "theta_B_rad": num(gb, "rad")})
                res.append(rec)
            return res

        desc = (f"phi near 2: a simply supported member (A: UX UY UZ RX; B: UY UZ), OD 0.25 m, wall 1/64 m "
                f"(m = 7/8), E 200 GPa, G 77 GPa, L = {Lf!r} m ({tag}: the binary64 nearest L* = sqrt({fstr(L2)}) m "
                f"for NEAREST, two ulps above or below it otherwise); a 100 N*m couple at A about Z, then about Y, "
                f"then -1000 N/m in Y. Under the couple theta_B = -ML/6EI + M/(Ks L) vanishes at phi = 2")
        cases, _ = build_case(f"PFT-PHI2-{tag}", "phi = 2", desc, lambda c, L=L: phi2_spec(c, L),
                              ["cowper_factor", "five_sixths_factor", "euler_bernoulli_recovery"], requests,
                              key_values=keys, extra={"length_binary64": Lf, "length_hex": f64hex(Lf)})
        out += cases
    return out


def phi2_exact_cases(requests):
    """phi = 2 exactly (the supplementary auxetic control): theta_B under a couple at A is exactly
    zero, the (2 - phi) term of k_thetatheta' vanishing in exact arithmetic."""
    nu = derived_nu(E_AUX, G_AUX)
    agree("auxetic control inputs", "nu = E/(2G) - 1 = -5/9 > -1 and kappa_E(1/3, -5/9) = 30/49",
          nu == F(-5, 9) and kappa_energy(F(1, 3), nu) == F(30, 49))

    def keys(lid, st, se):
        res = []
        for sol in (st, se):
            EI, Ks, L = eiks(sol, "pipe:AB")
            M = F(100)
            rec = {}
            if Ks is not None:
                phi = 12 * EI / (Ks * L * L)
                agree("phi = 2 exactly", f"{sol['spec']['case_id']}", phi == 2)
                rec["phi"] = num(phi)
            if lid in ("case:couple_a_z", "case:couple_a_y"):
                sh = ZERO if Ks is None else M / (Ks * L)
                ta, tb = M * L / (3 * EI) + sh, -M * L / (6 * EI) + sh
                k = 5 if lid == "case:couple_a_z" else 4
                ga, gb = sol["u"]["node:A"][k], sol["u"]["node:B"][k]
                ok = ga == ta and gb == tb and (Ks is None or (gb.is_zero() and ga == M * L / (2 * EI)))
                agree("closed form (simply supported end couple at phi = 2: theta_A = ML/2EI, theta_B = 0)",
                      f"{sol['spec']['case_id']} {lid}", ok)
                rec.update({"theta_A_rad": num(ga, "rad"), "theta_B_rad": num(gb, "rad")})
            res.append(rec)
        return res

    desc = ("phi = 2 exactly, a supplementary auxetic control beside the near-2 steel cases: a simply supported "
            "member (A: UX UY UZ RX; B: UY UZ), OD 9/32 m, wall 3/32 m (m = 1/3), E 8 GPa, G 9 GPa "
            "(nu = E/(2G) - 1 = -5/9: physically unusual, inside kappa_E's domain nu > -1; kappa_E = 30/49), "
            "L = 7/32 m, every input binary64-exact; a 100 N*m couple at A about Z, then about Y, then -1000 N/m in "
            "Y. Under the couple theta_B = -ML/6EI + M/(Ks L) = 0 exactly. Admission is decided by the product's "
            "factor-range check: if the product refuses this nu by range, the consumer records that refusal "
            "instead of these rows")
    cases, _ = build_case("PFT-PHI2-EXACT-AUXETIC", "phi = 2 (exact, auxetic control)", desc,
                          lambda c: phi2_spec(c, L_AUX, SEC_AUX, MAT_AUX),
                          ["cowper_factor", "five_sixths_factor", "euler_bernoulli_recovery"], requests,
                          key_values=keys, extra={"length_binary64": float(L_AUX), "length_hex": f64hex(float(L_AUX))})
    return cases


def rigid_modes_check(spec):
    """Each member's global stiffness annihilates the six rigid motions of its two nodes."""
    pos = dict(spec["nodes"])
    for mem in spec["members"]:
        t = member_terms(spec, mem, TIMO, frozenset())
        L, R = member_frame(pos[mem["i"]], pos[mem["j"]], mem["y_ref"])
        kg = rotate_k(frame_local_k(t, L), R)
        xi, xj = [Q.of(F(c)) for c in pos[mem["i"]]], [Q.of(F(c)) for c in pos[mem["j"]]]
        ok = True
        for k in range(6):
            a = [ZERO] * 3
            if k < 3:
                tr = [ONE if q == k else ZERO for q in range(3)]
                d = tr + [ZERO] * 3 + tr + [ZERO] * 3
            else:
                a[k - 3] = ONE
                d = cross(a, xi) + a + cross(a, xj) + a
            ok &= all(x.is_zero() for x in matvec(kg, d))
        agree("six rigid modes", f"{spec['case_id']} {mem['id']}: K_g annihilates rigid translations and rotations",
              ok)


def skew_cases(requests):
    def keys(lid, st, se):
        for sol in (st, se):
            r = sol["reactions"]["spring:N1"][2]
            agree("spring reaction", f"{sol['spec']['case_id']} {lid}: -k u_z(N1)",
                  r == -2000000 * sol["u"]["node:N1"][2])
        return [{"u_z_N1_m": num(st["u"]["node:N1"][2], "m"), "r_x_N3_rad": num(st["u"]["node:N3"][3], "rad")},
                {"u_z_N1_m": num(se["u"]["node:N1"][2], "m"), "r_x_N3_rad": num(se["u"]["node:N3"][3], "rad")}]

    rigid_modes_check(skew_spec("PFT-SKEW-SPRING-TIMOSHENKO"))
    desc = ("skewed four-span run N0-N4 (directions (2,3,6)/7, (1,4,8)/9, (2,-1,2)/3, (6,2,3)/7; lengths 7/4, 9/8, "
            "3/2, 7/4 m; dyadic coordinates), anchors at N0 and N4, a pinned support (UX UY UZ) at N2, an interior "
            "spring 2e6 N/m in UZ at N1 and a rotational spring 5e5 N*m/rad in RX at N3; nodal forces at N3 and a "
            "couple at N1, then -800 N/m in global Z on every span")
    cases, _ = build_case("PFT-SKEW-SPRING", "skewed multi-span run with an interior spring", desc, skew_spec,
                          ["cowper_factor", "five_sixths_factor", "swapped_plane_signs", "kappa_on_GJ",
                           "shear_area_in_EA", "euler_bernoulli_recovery"], requests, explicit_eb_twin=True,
                          key_values=keys)
    return cases


def imposed_cases(requests):
    def keys(lid, st, se):
        if lid != "case:motion":
            return None
        for sol in (st, se):
            d = sol["u"]["node:D"]
            agree("prescribed motion", f"{sol['spec']['case_id']}: D moves by the imposed values",
                  d == [Q.of(F(3, 1000)), Q.of(F(-5, 1000)), Q.of(F(8, 1000)), ZERO, Q.of(F(2, 1000)), ZERO])
        return [{"reaction_d_Fz_N": num(st["reactions"]["support:d"][2], "N")},
                {"reaction_d_Fz_N": num(se["reactions"]["support:d"][2], "N")}]

    desc = ("L frame A(0,0,0) anchor -> C(3/2,0,0) -> D(3/2,1,0) anchor; case:motion moves D by UX 3 mm, UY -5 mm, "
            "UZ 8 mm and RY 0.002 rad (support-targeted imposed_displacement primitives; RX, RZ held); case:thermal "
            "is a uniform 80 degC rise with alpha 1.2e-5 /degC; the combination is motion + 0.5 thermal")
    cases, _ = build_case("PFT-IMPOSED", "imposed support displacement and a combination", desc, imposed_spec,
                          ["cowper_factor", "five_sixths_factor", "swapped_plane_signs", "kappa_on_GJ",
                           "euler_bernoulli_recovery"], requests, key_values=keys)
    return cases


def weight_cases(requests):
    wm, wc = weight_intensities()

    def keys(lid, st, se):
        res = []
        for sol in (st, se):
            total = (wm + wc) * (F(2) + F(5, 2) + F(3, 2))
            rz = sum((sol["reactions"][s][2] for s in ("support:W0", "support:W1", "support:W3")), ZERO)
            agree("statics", f"{sol['spec']['case_id']}: vertical reactions carry (w_m + w_c) x total length",
                  rz == total)
            res.append({"vertical_reaction_sum_N": num(rz, "N")})
        return res

    desc = ("self-weight with contents: W0(0,0,0) anchor -> W1(2,0,0) rest (UZ) -> W2(7/2,0,2) (a 5/2 m span "
            "rising along (3,0,4)/5) -> W3(7/2,3/2,2) anchor; OD 0.2 m, wall 0.01 m, mill tolerance 0.001 m; on every "
            "span two weight primitives in -Z: metal w_m = g rho_m pi (r_o^2 - r_i^2) and contents "
            "w_c = g rho_c pi r_i^2 with r_i = OD/2 - (t - c_m) = 0.091 m, g = 9.80665 m/s^2, rho_m = 7850 kg/m^3, "
            "rho_c = 1000 kg/m^3")
    extra = {"weight_intensities": {
        "formula": "w = g (rho_metal pi (r_o^2 - r_i^2) + rho_contents pi r_i^2), r_i = OD/2 - (t - c_m): PF's single "
                   "effective-wall section for every use, including mass (section-bases-and-timoshenko.md section 1: "
                   "the preview branch keeps source_od_effective_wall_areas/v1); insulation none",
        "metal_N_per_m": num(wm, "N/m"), "contents_N_per_m": num(wc, "N/m"),
        "authored": "each request authors the binary64 nearest each exact intensity, as a weight primitive "
                    "(force_per_length, global_z, negative); a test that generates self-weight from densities "
                    "instead should compare its intensities with these exact values"}}
    cases, _ = build_case("PFT-WEIGHT", "self-weight with contents", desc, weight_spec,
                          ["cowper_factor", "five_sixths_factor", "kappa_on_nominal_wall", "stiffness_on_nominal_wall",
                           "swapped_plane_signs", "euler_bernoulli_recovery"], requests, key_values=keys, extra=extra)
    return cases


def w15_cases(requests):
    def keys(lid, st, se):
        res = []
        for sol in (st, se):
            EI, Ks, _ = eiks(sol, "pipe:AB")
            s = F(4)
            # unit-load closed form: tip force over A-B (EI, Ks) and B-C (sEI, sKs)
            fy = F(100)
            v = fy * ((F(3, 2) ** 3 - F(1, 2) ** 3) / (3 * EI) + (F(1, 2) ** 3) / (3 * s * EI))
            if Ks is not None:
                v = v + fy * (1 / Ks + F(1, 2) / (s * Ks))
            agree("closed form (stepped cantilever by unit load: int (L-x)^2/(sEI) dx + int dx/(s kappa G A))",
                  f"{sol['spec']['case_id']} tip u_y", sol["u"]["node:C"][1] == v)
            rec = {"tip_u_y_m": num(sol["u"]["node:C"][1], "m")}
            if Ks is not None:
                bc = sol["members"]["body:BC"]
                phi_s = 12 * Q.of(bc["Es"]) * bc["I"] / (bc["Ks"] * bc["L"] ** 2)
                phi_1 = 12 * Q.of(bc["E"]) * bc["I"] / (Q.of(bc["G"]) * bc["As"] * bc["L"] ** 2)
                agree("phi invariant under s E, s G (exact)", f"{sol['spec']['case_id']} body:BC", phi_s == phi_1)
                rec["body_phi"] = num(phi_s)
            res.append(rec)
        return res

    desc = ("W-15 rigid-family component: cantilever A(0) anchor -> B(1,0,0) -> C(3/2,0,0); the span B-C is the body "
            "of a valve at C (mechanics_geometry_only) with stiffness scale s = 4, so its element is 4K, Timoshenko "
            "shear included (rigid-component-stiffness-scaling.md 2.1), and nu comes from the unscaled E and G; tip "
            "loads at C: +100 N Y, -60 N Z and a 20 N*m couple about X")
    cases, _ = build_case("PFT-W15", "W-15 rigid-family component", desc, w15_spec,
                          ["cowper_factor", "shear_stiffness_unscaled", "euler_bernoulli_recovery"], requests,
                          key_values=keys)
    return cases


def authnu_cases(requests):
    nu_d = derived_nu(E_A, G_A)

    def keys(lid, st, se):
        alt = solve_model(st["spec"], lid, TIMO, frozenset(["authored_nu_for_kappa"]))
        k = 1 if lid == "case:tip_y" else 2
        return [{"tip_transverse_m": num(st["u"]["node:B"][k], "m"),
                 "authored_nu_tip_transverse_m": num(alt["u"]["node:B"][k], "m"),
                 "relative_separation": float(format(separation(st["u"]["node:B"][k], alt["u"]["node:B"][k]), ".6e"))},
                {"tip_transverse_m": num(se["u"]["node:B"][k], "m")}]

    m = F(6, 100) / F(1, 10)
    diag = {"timoshenko": [{"code": "PF_TIMOSHENKO_AUTHORED_POISSON_RATIO_NOT_USED", "severity": "warning",
                            "material": "mat:A", "authored_poisson_ratio": rnum(F(3, 10)),
                            "derived_poisson_ratio": rnum(nu_d), "case_basis": "the material's base E and G (no "
                            "temperature selection), for every load case",
                            "trigger": f"|nu_a - nu_d| = {fstr(abs(F(3, 10) - nu_d))} > 1e-6 max(|nu_a|, |nu_d|) "
                                       "(pf-timoshenko.md 2)"}],
            "euler_bernoulli": [{"code": "none new", "note": "an Euler-Bernoulli document emits nothing new "
                                                            "(pf-timoshenko.md 2)"}]}
    extra = {"poisson": {"authored": rnum(F(3, 10)), "derived": rnum(nu_d),
                         "kappa_E_derived": rnum(kappa_energy(m, nu_d)), "kappa_E_authored": rnum(kappa_energy(m, F(3, 10)))}}
    desc = ("authored poisson_ratio 0.3 with E 200 GPa and G 77 GPa (E/(2G) - 1 = 23/77): a thick cantilever "
            "(OD 0.2 m, wall 0.04 m, m = 3/5) of L = 1/2 m; tip +100 N Y, then -100 N Z. Expected rows use the "
            "derived nu; the authored-nu rows are the discriminator")
    cases, _ = build_case("PFT-AUTHNU", "authored poisson_ratio not used", desc, authnu_spec,
                          ["authored_nu_for_kappa", "cowper_factor"], requests, key_values=keys, diagnostics=diag,
                          extra=extra)
    return cases


def nl_cases(requests):
    out = []

    def prop_flex(sol, a, b):
        """Cantilever flexibility v(a) under a unit load at b (a <= b), Timoshenko."""
        EI, Ks, _ = eiks(sol, "pipe:AB")
        f = F(a) * F(a) * (3 * F(b) - F(a)) / (6 * EI)
        return f + (ZERO if Ks is None else F(a) / Ks)

    def keys_oneway(lid, st, se):
        res = []
        for sol in (st, se):
            r = sol["reactions"]["support:NL-B"][1]
            if lid == "case:pressed":
                exp = F(1000) * prop_flex(sol, 1, 2) / prop_flex(sol, 1, 1)
                agree("compatibility (prop at B: R = -P delta_BC/delta_BB)", f"{sol['spec']['case_id']} {lid}", r == exp)
            res.append({"state": sol["states"]["support:NL-B"], "reaction_Fy_N": num(r, "N"),
                        "u_y_B_m": num(sol["u"]["node:B"][1], "m")})
        return res

    def keys_gap(lid, st, se):
        res = []
        for sol in (st, se):
            r = sol["reactions"]["support:GAP-C"][1]
            if lid == "case:closed":
                exp = (F(-1, 2000) - F(-2000) * prop_flex(sol, 2, 2)) / prop_flex(sol, 2, 2)
                agree("compatibility (closed gap at C: u_C = -g)", f"{sol['spec']['case_id']} {lid}", r == exp)
            res.append({"state": sol["states"]["support:GAP-C"], "reaction_Fy_N": num(r, "N"),
                        "u_y_C_m": num(sol["u"]["node:C"][1], "m")})
        return res

    def keys_fric(lid, st, se):
        res = []
        for sol in (st, se):
            r = sol["reactions"]["support:FR-B"][2]
            P = F(80) if lid == "case:stick" else F(200)
            if lid == "case:stick":
                ok = r == -P * prop_flex(sol, 1, 2) / prop_flex(sol, 1, 1)
            else:
                ok = r == F(-300) and sol["u"]["node:B"][2] == P * prop_flex(sol, 1, 2) - 300 * prop_flex(sol, 1, 1)
            agree("compatibility (friction: stick R = -P delta_BC/delta_BB; slide F = -mu N sign(u))",
                  f"{sol['spec']['case_id']} {lid}", ok)
            res.append({"state": sol["states"]["support:FR-B"], "reaction_Fz_N": num(r, "N"),
                        "u_z_B_m": num(sol["u"]["node:B"][2], "m")})
        return res

    frame = ("cantilever A(0,0,0) anchor -> B(1,0,0) -> C(2,0,0), OD 0.2 m, wall 0.01 m, E 200 GPa, G 77 GPa; ")
    law = {"support_law": "DEC-067 as stated in DEL-04-01 Design kd5-nonlinear-active-set.md section 0; every "
                          "combination of support states solved exactly, exactly one satisfies every condition "
                          "strictly and none weakly"}
    for cid, fam, desc, fn, keys in (
            ("PFT-NL-ONEWAY", "one-way support that lifts off", frame + "a one-way support at B in UY, active when "
             "the reaction is positive; case:pressed -1000 N Y at C (active), case:lifted +1000 N Y at C (lifts off)",
             nl_oneway_spec, keys_oneway),
            ("PFT-NL-GAP", "gap that closes", frame + "a gap at C in UY closing in negative displacement with clearance "
             "0.5 mm; case:closed -2000 N Y at C (closes; u_C = -g), case:open -500 N Y at C (stays open)",
             nl_gap_spec, keys_gap),
            ("PFT-NL-FRICTION", "friction stick and slide", frame + "a friction support at B in UZ, mu = 0.3 with an "
             "explicit normal reaction 1000 N (capacity 300 N); case:stick +80 N Z at C (sticks), case:slide +200 N Z "
             "at C (slides in +Z with F = -300 N)", nl_friction_spec, keys_fric)):
        cases, _ = build_case(cid, fam, desc, fn, ["cowper_factor", "five_sixths_factor", "euler_bernoulli_recovery"],
                              requests, key_values=keys, extra=law)
        out += cases
    return out


U4A_ENGINE = OUT.parent / "t4_u4a" / "u4a_engine.py"
_U4A = []


def u4a_engine():
    """t4_u4a's engine (reference code), loaded from its file without writing bytecode there."""
    if not _U4A:
        import importlib.util
        sys.dont_write_bytecode = True
        sp = importlib.util.spec_from_file_location("u4a_engine", U4A_ENGINE)
        mod = importlib.util.module_from_spec(sp)
        sp.loader.exec_module(mod)
        _U4A.append(mod)
    return _U4A[0]


def fdecstr(x):
    """The exact decimal text of a rational with a terminating expansion."""
    x = F(x)
    d = Decimal(x.numerator) / Decimal(x.denominator)
    assert F(d) == x
    return str(d)


def u4a_model(spec, lc):
    """The same line as t4_u4a engine input (traversal = node order; anchored at the first node and,
    when supported there, at the last)."""
    order = [nid for nid, _ in spec["nodes"]]
    members = []
    for m in spec["members"]:
        e = {"pipe": m["id"], "kind": "arc" if "arc" in m else "straight", "authored_from": m["i"],
             "authored_to": m["j"], "y_reference": [float(c) for c in m["y_ref"]]}
        if "arc" in m:
            e.update(R=fdecstr(m["arc"]["R"]), k=fdecstr(m["arc"]["k"]))
        members.append(e)
    mem0 = spec["members"][0]
    mat = spec["materials"][mem0["material"]]
    uniform = {}
    for (pid, direction, w, _cat) in lc.get("uniform", []):
        v = ["0", "0", "0"]
        v[DIRS[direction]] = fdecstr(w)
        uniform.setdefault(pid, []).append(v)
    nodal = {}
    for (nid, direction, val) in lc.get("nodal", []):
        fm = nodal.setdefault(nid, [["0", "0", "0"], ["0", "0", "0"]])
        k = DIRS[direction]
        fm[k // 3][k % 3] = fdecstr(val)
    return {"nodes": {nid: [float(c) for c in p] for nid, p in spec["nodes"]}, "traversal_order": order,
            "members": members,
            "section": {"outside_diameter": fdecstr(mem0["od"]), "wall_thickness": fdecstr(mem0["wall"] - mem0["mill"])},
            "material": {"E": fdecstr(mat["E"]), "G": fdecstr(mat["G"]), "alpha": fdecstr(mat["alpha"] or 0)},
            "loads": {"uniform": uniform, "nodal": nodal, "delta_T": {pid: fdecstr(dT) for pid, dT in lc.get("thermal", [])}},
            "anchor_last": any(s["node"] == order[-1] for s in spec["supports"])}


def check_arcs_against_u4a_engine(spec, lc_id, sol, shear_members, label):
    """Run t4_u4a's unit-load engine (90 digits, 48-point Gauss-Legendre per member) on the same line,
    with the shear complementary energy (F.f - N n)/(kappa_E G A) added on the members in
    shear_members, and compare every nodal displacement, support A's action and the five tangent-frame
    cuts of every member with this generator's exact values."""
    lc = next(c for c in spec["load_cases"] if c["id"] == lc_id)
    with localcontext() as ctx:
        ctx.prec = 90
        E = u4a_engine()
        shear = {}
        for mem in spec["members"]:
            if mem["id"] in shear_members:
                t = member_terms(spec, mem, TIMO, frozenset())
                shear[mem["id"]] = (1 / (Q.of(t["G"]) * t["As"])).dec()

        class ShearLine(E.Line):
            def _integrand(self, m, u, actual, unit_load, with_strain):
                value = super()._integrand(m, u, actual, unit_load, with_strain)
                c = shear.get(m.id)
                if c is not None:
                    (Fa, _), (fu, _) = actual, unit_load
                    tg = m.traversal_tangent(u)
                    value += (E.dot(Fa, fu) - E.dot(Fa, tg) * E.dot(fu, tg)) * c
                return value

        line = ShearLine(u4a_model(spec, lc)).solve()
        got = {"displacement": [], "reaction": [], "cut": []}
        for index, (nid, _) in enumerate(spec["nodes"]):
            if index == 0 or (line.anchor_d and index == len(spec["nodes"]) - 1):
                continue
            got["displacement"].append((line.virtual_work(index), sol["u"][nid]))
        fa, ma = line.support_a()
        got["reaction"].append((list(fa) + list(ma), sol["reactions"]["support:A"]))
        for k, m in enumerate(line.members):
            faces = sol["members"][m.id]["faces"]
            for (loc, u), (name, _) in zip(E.FRACTIONS, STATIONS):
                Fc, Mc = line.cut(k, u)
                got["cut"].append((E.components(Fc, Mc, m.axes(u)), faces[name]))
    worst = Decimal(0)
    for fam, pairs in got.items():
        for half in (slice(0, 3), slice(3, 6)):
            scale = max(abs(vdec(v)) for _, mine in pairs for v in mine[half])
            if scale == 0:
                continue
            for theirs, mine in pairs:
                for a, b in zip(theirs[half], mine[half]):
                    worst = max(worst, abs(a - vdec(b)) / scale)
    agree("t4_u4a engine (validation/references/t4_u4a/u4a_engine.py: Line, Member, unit-load method)",
          f"{spec['case_id']} {lc_id} {label}: nodal displacements, support A and the five cuts of every member agree "
          f"within 1e-60 of their family scale", worst < Decimal("1e-60"),
          max_relative_difference=float(format(worst, ".3e")))


def arc_cases(requests):
    out = []
    mutants = ["cowper_factor", "five_sixths_factor", "euler_bernoulli_recovery", "theta_equals_slope",
               "arc_flexibility_factor_ignored", "timoshenko_arc"]
    diag = {"timoshenko": [{"code": "PF_TIMOSHENKO_REALIZED_ARCS_EULER_BERNOULLI", "severity": "nonblocking",
                            "emission": "once per result, model-wide", "entity_refs": ["pipe:BC"],
                            "message_counts": {"straight_members_with_shear_deformation": 2,
                                               "realized_arcs_without_shear_deformation": 1},
                            "source": "pf-timoshenko.md section 4, named arc disclosure"}],
            "euler_bernoulli": [{"code": "none new", "note": "no arc disclosure without the Timoshenko selection "
                                                            "(pf-timoshenko.md section 4)"}]}
    premise = {"arc_premise": "manager's premise (a product code fact, not derived here): on PF a bend component "
                              "consumed as curved_bend_macro_element (DEC-070) replaces its pipe's straight chord by "
                              "the curved-bend macro-element, the element the exact route builds with the same "
                              "function; under the Timoshenko selection it stays Euler-Bernoulli and the straight "
                              "tangents are Timoshenko",
               "arc_treatment": "t4_u4a's (u4a_engine.py Member, Line._integrand): arc rule, tangent frame, k on "
                                "both bending planes, constant global uniform load per unit arc length, alpha dT "
                                "along the arc; arc rows in the tangent frame at their own section, end rows "
                                "node-on-element (DEL-04-07 arc-frames-and-signs.md section 2); PF's single section "
                                "(c_m = 0 here)",
               "station_values": "the arc's quarter stations are psi = pi/8 and 3pi/8, so their exact values lie in "
                                 "Q(pi)(sqrt(2), cos(pi/8), sin(pi/8)); midspan in Q(pi)(sqrt(2))"}
    line = ("L-line A(0,0,0) -> B(1,0,0) -> quarter-circle arc R = 3/8 m (bend component, curved_bend_macro_element, "
            "k = 2, bowing toward +y_reference (1,-1,0)) -> C(11/8,3/8,0) -> D(11/8,11/8,0); OD 0.2 m, wall 0.01 m, "
            "E 200 GPa, G 77 GPa, alpha 1.2e-5 /degC; ")
    specs = (("PFT-ARC-ANCH", "realized arc between Timoshenko tangents, both ends anchored", line + "anchors at A "
              "and D; case:nodal (400, -300, 600) N at C and 150 N*m about Z at B; case:thermal 80 degC on every "
              "member; case:uniform_z -800 N/m in Z on every member, the arc included; case:arc_in_plane (500, -300, "
              "0) N/m on the arc only", arc_anch_spec),
             ("PFT-ARC-TIP", "realized arc between Timoshenko tangents, tip-loaded", line + "anchor at A, D free; "
              "case:tip (300, -200, 500) N and 80 N*m about Y at D; case:uniform_z -800 N/m in Z on every member",
              arc_tip_spec))
    for cid, fam, desc, fn in specs:
        def keys(lid, st, se):
            res = []
            for sol, label, shear in ((st, "timoshenko", ("pipe:AB", "pipe:CD")), (se, "euler_bernoulli", ())):
                check_arcs_against_u4a_engine(sol["spec"], lid, sol, shear, label)
                rec = {"u_C_m": [num(x, "m") for x in sol["u"]["node:C"][0:3]]}
                if sol["spec"]["case_id"].startswith("PFT-ARC-TIP"):
                    rec["u_D_m"] = [num(x, "m") for x in sol["u"]["node:D"][0:3]]
                else:
                    rec["reaction_D_N"] = [num(x, "N") for x in sol["reactions"]["support:D"][0:3]]
                res.append(rec)
            ctl = solve_model(st["spec"], lid, TIMO, frozenset(["timoshenko_arc"]))
            check_arcs_against_u4a_engine(ctl["spec"], lid, ctl, ("pipe:AB", "pipe:BC", "pipe:CD"), "arc-Timoshenko control")
            return res

        cases, _ = build_case(cid, fam, desc, fn, mutants, requests, key_values=keys, diagnostics=diag,
                              extra=premise, arc_control=True)
        out += cases
    return out


def crosscheck_t4_u8():
    """Agreement with t4_u8's frozen S-B values (its README): kappa_E(m = 9/10, nu = 3/10) =
    16609827/33068756 and the L = 1 Timoshenko tip u_y 125917189/(6068975250000 pi) m under
    +100 N, here solved on a PF document whose G = E/2.6 gives nu = 3/10 exactly."""
    agree("t4_u8 README (S-B section)", "kappa_E(9/10, 3/10)", kappa_energy(F(9, 10), F(3, 10)) == F(16609827, 33068756))
    agree("SHEAR_REFERENCE CONTRACT form", "kappa_E cancellation-free form equals the contract form",
          all(kappa_energy(m, nu) == kappa_energy_contract_form(m, nu)
              for m in (F(0), F(3, 5), F(7, 8), F(9, 10), F(91, 100)) for nu in (F(0), F(23, 77), F(3, 10), F(1, 4))))
    mats = {"mat:A": {"E": E_A, "G": E_A * F(10, 26), "nu_authored": None, "alpha": None}}
    spec = cantilever_spec("t4_u8-crosscheck", F(1), SEC, [{"id": "case:tip_y", "nodal": [("node:B", "global_y", F(100))]}],
                           materials=mats)
    sol = solve_model(spec, "case:tip_y", TIMO)
    agree("t4_u8 README (S-B L = 1 tip u_y)", "Timoshenko tip u_y at nu = 3/10",
          sol["u"]["node:B"][1] == Q((F(125917189, 6068975250000),), (0, 1)))
    # the thermal eigenload's sign: a free cantilever grows by alpha dT L with no axial force
    spec = cantilever_spec("thermal-crosscheck", F(3, 2), SEC, [{"id": "case:t", "thermal": [("pipe:AB", F(80))]}],
                           materials=MAT_A_THERMAL)
    sol = solve_model(spec, "case:t", TIMO)
    agree("free thermal growth", "cantilever tip u_x = alpha dT L and N = 0",
          sol["u"]["node:B"][0] == Q.of(ALPHA * 80 * F(3, 2))
          and all(face[0].is_zero() for face in sol["members"]["pipe:AB"]["faces"].values()))


OMITTED = [
    {"case": "geometry-only bend (mechanics_geometry_only, the default consumption)",
     "reason": "no separate case needed, on the manager's premise (a product code fact, not derived here): such a "
               "bend is analysed as an ordinary straight chord member, and a DEC-045 multiplier applies only to a "
               "stress-review row, never to stiffness, so its rows are a straight member's. Consumer test: a "
               "geometry-only bend document gives the same rows as its plain straight-member twin"},
    {"case": "constant-effort hangers",
     "reason": "not asked in this step; the support law of a constant-effort gate is not frozen here"},
    {"case": "friction with a derived normal reaction (normal_reaction_source)",
     "reason": "the explicit-normal law is used for stick and slide; the derived-normal coupling is the K-D5 "
               "nonlinear reference's (kd5_nonlinear) and is not repeated"},
    {"case": "both solver modes",
     "reason": "a consumer property: both modes are judged against the same rows"},
    {"case": "M3-T judged set (certificate soundness)",
     "reason": "a separate later step (design section 5)"},
]


def product_cases(requests):
    crosscheck_t4_u8()
    return (sb_cases(requests) + ss_cases(requests) + phi2_cases(requests) + phi2_exact_cases(requests)
            + skew_cases(requests) + imposed_cases(requests) + weight_cases(requests) + w15_cases(requests)
            + authnu_cases(requests) + nl_cases(requests) + arc_cases(requests))


def product_document(cases):
    return {
        "schema_version": "independent.pf_timoshenko_product_cases/1.0.0",
        "unit": "PF opt-in Timoshenko straight members (model_settings.shear_model = timoshenko_annulus_energy_v1) "
                "and their Euler-Bernoulli twins",
        "status": "frozen independent reference before any product comparison; refuted in round 1 by a second TASK "
                  "(every row and mutant recomputed; its findings are repaired here), with the realized-arc cases and "
                  "the exact phi = 2 control added after that round and not yet refuted; not product output, not a "
                  "qualification or acceptance claim",
        "independence": "generators/pf_reference.py imports, reads and runs no product code; expected values are this "
                        "generator's exact arithmetic, never product observations; one product fact, PF's realized "
                        "arc, is the manager's premise (see the PFT-ARC cases)",
        "sources": {
            "design": "execution/PKG-05/DEL-05-03/Design/pf-timoshenko.md sections 1-3 and 5; "
                      "section-bases-and-timoshenko.md sections 1, 3.1, 3.3-3.5 and 7",
            "support_law": "execution/PKG-04/DEL-04-01/Design/kd5-nonlinear-active-set.md section 0 (DEC-067)",
            "w15_scaling": "execution/PKG-03/DEL-03-05/Design/rigid-component-stiffness-scaling.md sections 1-2.3",
            "imposed_motion": "validation/references/w16_imposed_motion/README.md (PF authoring of support-targeted "
                              "imposed_displacement)",
            "arithmetic_and_rows": "validation/references/t4_u8/generators/u8_reference.py (reference code)",
            "realized_arc": "validation/references/t4_u4a/u4a_engine.py (Member, Line; reference code, run here as a "
                            "cross-check) and DEL-04-07 Design arc-frames-and-signs.md section 2 (arc row frames and "
                            "signs); pf-timoshenko.md section 1 (realized arcs stay Euler-Bernoulli) and section 4 "
                            "(the named arc disclosure)"},
        "numeric_representation": {
            "exact": "authoritative; Q(pi) text forms 'p', 'p/q', 'r*pi', 'r/pi', '(n0 + n1*pi + ...)/(d0 + d1*pi + "
                     "...)', 'sqrt(...)' for a norm outside Q(pi), 'a + sqrt(b)/(c)' for a maximum; on a realized "
                     "arc's midspan and quarter stations '(a) + (b)*sqrt(2) + (c)*cos(pi/8) + (d)*sin(pi/8)' with a, "
                     "b, c, d in Q(pi) (zero terms omitted)",
            "decimal": "40 significant digits from the exact form (pi by Machin, 140-digit context)",
            "expected": "binary64 rounding of the exact value converted into the row unit; never a product value",
            "inputs": "decimal inputs are exact rationals (r_i = 9/100 m, and so on); the requests author the nearest "
                      "binary64 values (relative effects of order 1e-16). E = 200e9 Pa and G = 77e9 Pa are exact "
                      "binary64 integers, so nu = E/(2G) - 1 = 23/77 is exact for the authored values; the phi = 2 "
                      "cases author dyadic OD, wall and L, exact in binary64, and the exact phi = 2 control also E = "
                      "8e9 Pa and G = 9e9 Pa (nu = -5/9); the arc lines author dyadic coordinates and R = 0.375 m, "
                      "with the bend angle the binary64 nearest pi/2"},
        "criteria": {
            "modes": ["sparse_interactive", "dense_scrutiny"], "same_reference_both_modes": True,
            "nonzero": "|observed - expected| <= 1e-9 |expected| (criterion.kind relative)",
            "zero": "|observed| <= criterion.absolute_tolerance = 1e-9 |zero_scales[criterion.scale]| in the row unit",
            "negative": "for every discriminator row: |observed - wrong| > 1e-9 max(|observed|, |wrong|); every listed "
                        "wrong value is separated from the expected value by more than 1e-7 relative",
            "no_new_threshold": "the existing 1e-9 relative criterion only (t4_u8's policy)"},
        "row_conventions": {
            "key": "(kind, entity_ref, component, location); basis_ref is the load case, or for a combination "
                   "{ref_type: combination, ref_id}",
            "end_rows": "element_local force rows at end_i/end_j: node-on-element end actions, element-local DOF sense",
            "station_rows": "quarter_1/midspan/quarter_3: the section-cut action on the +x face of the i-side segment",
            "stress": "element_local stress rows (MPa) at all five locations from the +x-face section action: axial "
                      "N/A, bending M_y/Z and M_z/Z, torsional T r_o/J, on PF's single section",
            "maximum": "pipe_elastic_normal_stress_maximum_v2 (Pa, governing_station): max over the member of |N/A| + "
                       "hypot(M_y, M_z)/Z, frozen only where the five-station maximum is provably the member maximum",
            "displacements": "global, mm; rotations global, rad: on a Timoshenko member the section rotation theta, "
                             "not the slope",
            "arcs": "on a realized arc every element_local row is in the tangent frame at its own section (x toward "
                    "authored j, y toward the arc centre, z the bend-plane normal); end rows node-on-element, station "
                    "and stress rows the j-side section action at psi = 0, pi/8, pi/4, 3pi/8, pi/2 "
                    "(arc-frames-and-signs.md section 2); stresses by the straight formulas on the arc resultants; "
                    "the arc's pipe_elastic_normal_stress_maximum_v2 is not frozen",
            "reactions": "support_reaction_component_v2: support-on-pipe, global, R = K u - f; a spring's reaction is "
                         "-k u in its DOF; a nonlinear support's slot carries its constraint force when engaged, the "
                         "applied friction force when sliding and zero when released; magnitudes are the force and "
                         "moment norms",
            "discriminating": "on every row of both twins: true where the Timoshenko and Euler-Bernoulli expectations "
                              "lie more than 1e-7 relative apart (a zero against a nonzero counts); "
                              "authored_nu_discriminating likewise against the authored-nu solve (PFT-AUTHNU), and "
                              "arc_timoshenko_discriminating against the arc-Timoshenko control (PFT-ARC-*)",
            "not_frozen": "element_global_end_force_v1, element_global_end_moment_v1, "
                          "element_local_normal_stress_extreme, element_local_transverse_shear_stress_maximum, "
                          "displacement_magnitude and linear_solver_mode_basis rows, whose formulas are not stated in a "
                          "design read here; for combinations also the magnitude and maximum rows"},
        "zero_scale_definitions": FAMILY_SCALE_DEFINITIONS,
        "controls": {
            "euler_bernoulli_twins": "each EULER-BERNOULLI case is the Euler-Bernoulli expectation of its "
                                     "TIMOSHENKO twin's document without model_settings.shear_model; an "
                                     "EXPLICIT-EULER-BERNOULLI case has the same rows",
            "breach": "a Timoshenko product judged against the Euler-Bernoulli twin's rows must breach on every "
                      "row marked discriminating; on PFT-AUTHNU a product judged against the authored-nu rows must "
                      "breach on every row marked authored_nu_discriminating; on PFT-ARC-* a product given the "
                      "TIMOSHENKO request and judged against the ARC-TIMOSHENKO-CONTROL rows must breach on every row "
                      "marked arc_timoshenko_discriminating"},
        "discriminator_catalogue": MUTANT_CATALOGUE,
        "omitted_cases": OMITTED,
        "crosscheck": list(CROSS),
        "cases": cases,
    }


def readme_kappa_figures():
    """(section name, exact kappa_E, its 17-digit decimal): the README's kappa_E table, from the rationals."""
    nu = derived_nu(E_A, G_A)
    out = []
    for name, sec, n in (("SEC (m = 9/10)", SEC, nu), ("SEC_WT (m = 91/100)", SEC_WT, nu),
                         ("SEC_PHI2 (m = 7/8)", SEC_PHI2, nu), ("SEC_THICK (m = 3/5)", SEC_THICK, nu),
                         ("SEC_THICK, authored nu 3/10", SEC_THICK, F(3, 10)),
                         ("SEC_AUX (m = 1/3, nu = -5/9)", SEC_AUX, derived_nu(E_AUX, G_AUX))):
        s = pf_section(dict(sec))
        k = kappa_energy(s["ri"] / s["ro"], n)
        out.append((name, fstr(k), format(Decimal(k.numerator) / Decimal(k.denominator), ".17g")))
    return out


def readme_figures(cases):
    """The figures the README quotes, recomputed (printed in check mode)."""
    nu = derived_nu(E_A, G_A)
    out = [f"nu = E/(2G) - 1 = {fstr(nu)}; binary64 {derived_nu_binary64(E_A, G_A)!r}"]
    for name, exact, dec in readme_kappa_figures():
        out.append(f"kappa_E {name} = {exact} = {dec}")
    byid = {c["id"]: c for c in cases}
    for c in cases:
        if c["shear_model"] != TIMO:
            continue
        phis = ", ".join(f"{m['id']} {float(Decimal(m['phi']['decimal'])):.17g}" for m in c["members"] if "phi" in m)
        src = byid[c.get("same_rows_as", c["id"])]
        nd = sum(lc["discriminating_rows"] for lc in src["load_cases"])
        nr = sum(len(lc["rows"]) for lc in src["load_cases"])
        same = f" (rows of {src['id']})" if src is not c else ""
        out.append(f"{c['id']}: phi {phis}; {nd}/{nr} rows discriminating{same}")
    return out


def sha256sums(files):
    lines = [f"{hashlib.sha256(data).hexdigest()}  {path}" for path, data in sorted(files.items())]
    return ("\n".join(lines) + "\n").encode()


def build():
    CROSS.clear()
    requests = {}
    cases = product_cases(requests)
    files = dict(requests)
    doc = product_document(cases)
    files["product_cases.json"] = dumps_compact(doc).encode()
    assert json.loads(files["product_cases.json"]) == json.loads(dumps(doc))
    for fixed in ("README.md", "generators/pf_reference.py"):
        p = OUT / fixed
        if p.exists():
            files[fixed] = p.read_bytes()
    files["SHA256SUMS"] = sha256sums({k: v for k, v in files.items() if k != "SHA256SUMS"})
    return files, cases


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--write", action="store_true")
    parser.add_argument("--figures", action="store_true", help="print the README's figures")
    args = parser.parse_args()
    files, cases = build()
    bad = [c for c in CROSS if not c["agrees"]]
    rows = sum(len(lc.get("rows", [])) for c in cases for lc in c.get("load_cases", []))
    nd = sum(lc.get("discriminating_rows", 0) for c in cases if c["shear_model"] == TIMO and "same_rows_as" not in c
             for lc in c["load_cases"])
    print(f"{len(cases)} cases, {sum(len(c.get('load_cases', [])) for c in cases)} load cases and combinations, "
          f"{rows} rows ({nd} Timoshenko rows discriminating); "
          f"{len([k for k in files if k.startswith('requests/')])} requests")
    print(f"cross-checks: {len(CROSS)} recorded, {len(bad)} disagreeing")
    for c in bad:
        print("DISAGREES:", json.dumps(c))
    if args.figures:
        for line in readme_figures(cases):
            print(line)
    if args.write:
        for path, data in files.items():
            if path in ("README.md", "generators/pf_reference.py"):
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
    readme = (OUT / "README.md").read_text(encoding="utf-8")
    figures = readme_kappa_figures()
    missing = [f"README kappa_E {name} {exact} = {dec}" for name, exact, dec in figures
               if f"| {exact} | {dec} |" not in readme]
    if failures or extra or bad or missing:
        print("MISMATCH: " + ", ".join(failures + [f"unexpected {e}" for e in extra] + missing))
        return 1
    print(f"README kappa_E figures: {len(figures)} rows equal their rationals' 17-digit decimals")
    print(f"verified {len(files)} files byte for byte (SHA256SUMS included)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
