#!/usr/bin/env python3
"""W-16t independent references: partial-span wind on Timoshenko members on the exact route.

Standard-library Python 3 only. This generator imports, reads and runs no
product code and reads no file other than the ones it writes. Every expected
value is derived here in exact arithmetic from linear-elastic small-displacement
beam theory (DEL-05-03 Design `section-bases-and-timoshenko.md` §3.4 and §4
item 4; DEL-05-01 Design `exact-equivalent-static.md` §3 and §5).

Timoshenko beam (one bending plane; v transverse displacement, theta section
rotation, M = EI theta', Q = K_s (v' - theta), K_s = G A_s = G kappa_E A_n):

  Q' = -w,  M' = -Q,  phi = 12 EI / (K_s L^2).

Three routes, required to agree exactly (the generator aborts otherwise):
  1. continuum: the force method on the whole line (cantilever or fixed-fixed)
     with Macaulay load integrals W_n(x) = int_0^x (x - s)^n / n! w(s) ds, and
     interior values by integration from the i end; no element formula;
  2. element: the exact Timoshenko element (stiffness 12EI/(L^3(1+phi)),
     6EI/(L^2(1+phi)), (4+phi)EI/(L(1+phi)), (2-phi)EI/(L(1+phi))) and the
     fixed-end vector feq_T = feq_EB + dV0 [1, L/2, -1, L/2],
     dV0 = phi/(1+phi) (wL/2) [g(a) - g(b)], g(x) = x^2 (1 - x)^2, assembled
     and solved in rationals;
  3. closure: each element's interior integrated from its i-end state and
     recovered i-end actions returns the j-end motion exactly.
Cantilever tips are also checked by the unit-load method.

Every component value is q * pi**k with q rational (forces and moments k = 0
for wind, 1 for seismic; motions and stresses one power lower, I and A_s being
rational multiples of pi). Magnitudes and elastic maxima need square roots:
they are sums c * sqrt(s) * pi**k with rational c and s, evaluated in rigorous
rational intervals (pi between two 90-digit rational bounds, square roots by
integer square roots at 10^-100), and their binary64 value is the nearest
double, decided by both interval ends.

Modes:
  python3 -I w16t_reference.py           check: rebuild every output in memory and
                                          compare byte for byte with the committed
                                          files, SHA256SUMS included
  python3 -I w16t_reference.py --write   write the outputs (intentional changes only)
"""
import argparse
import hashlib
import json
import math
import sys
from decimal import ROUND_CEILING, ROUND_FLOOR, Decimal, getcontext, localcontext
from fractions import Fraction as F
from pathlib import Path

HERE = Path(__file__).resolve().parent
OUT = HERE.parent
FIXED_FILES = ("README.md", "generators/w16t_reference.py")

# ----------------------------------------------------------------------------- pi and exact reals
getcontext().prec = 120


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


PI_DEC = 16 * _atan_inv(5) - 4 * _atan_inv(239)  # Machin, ~115 correct digits
_PI_90 = F(str(PI_DEC)[:92])  # "3." + 90 digits, truncated
PI_LO, PI_HI = _PI_90, _PI_90 + F(1, 10 ** 90)
assert PI_LO < F(314159265358979323846, 10 ** 20) + F(1, 10 ** 20) and PI_HI > F(314159265358979323846, 10 ** 20)
SQRT_SCALE = 10 ** 100


def sqrt_bounds(s):
    """Rational lo <= sqrt(s) <= hi for rational s >= 0 (width about 10^-100 relative)."""
    s = F(s)
    assert s >= 0
    if s == 0:
        return F(0), F(0)
    n, d = s.numerator, s.denominator
    # sqrt(n/d) = sqrt(n d) / d.
    root = math.isqrt(n * d * SQRT_SCALE * SQRT_SCALE)
    lo = F(root, SQRT_SCALE * d)
    hi = F(root + 1, SQRT_SCALE * d)
    if root * root == n * d * SQRT_SCALE * SQRT_SCALE:
        hi = lo
    return lo, hi


class Real:
    """A finite sum of c * sqrt(s) * pi**k (c, s rational; s = 1 for rational terms)."""

    def __init__(self, terms=None):
        self.t = {}
        for (s, k), c in (terms or {}).items():
            if c != 0:
                self.t[(F(s), k)] = self.t.get((F(s), k), F(0)) + F(c)
        self.t = {key: c for key, c in self.t.items() if c != 0}

    @staticmethod
    def of(q, k=0):
        return Real({(1, k): F(q)})

    @staticmethod
    def sqrt_of(s, coef=1, k=0):
        s = F(s)
        if s == 0:
            return Real()
        r = math.isqrt(s.numerator), math.isqrt(s.denominator)
        if r[0] * r[0] == s.numerator and r[1] * r[1] == s.denominator:
            return Real({(1, k): F(coef) * F(r[0], r[1])})
        return Real({(s, k): F(coef)})

    def __add__(self, other):
        out = dict(self.t)
        for key, c in other.t.items():
            out[key] = out.get(key, F(0)) + c
        return Real(out)

    def scale(self, f):
        return Real({key: c * F(f) for key, c in self.t.items()})

    def is_zero(self):
        return not self.t

    def interval(self):
        lo = hi = F(0)
        for (s, k), c in self.t.items():
            slo, shi = sqrt_bounds(s)
            plo, phi = {0: (F(1), F(1)), 1: (PI_LO, PI_HI), -1: (1 / PI_HI, 1 / PI_LO)}[k]
            a, b = slo * plo, shi * phi
            if c >= 0:
                lo, hi = lo + c * a, hi + c * b
            else:
                lo, hi = lo + c * b, hi + c * a
        return lo, hi

    def binary64(self):
        lo, hi = self.interval()
        flo, fhi = float(lo), float(hi)
        if flo != fhi:
            raise SystemExit(f"binary64 rounding of {self.text()} is not decided by its interval")
        return flo

    def decimal40(self):
        if self.is_zero():
            return "0"
        lo, hi = self.interval()
        mid = (lo + hi) / 2
        v = Decimal(mid.numerator) / Decimal(mid.denominator)
        return format(v.normalize().__format__(".39e"))

    def text(self):
        if self.is_zero():
            return "0"
        parts = []
        for (s, k), c in sorted(self.t.items(), key=lambda kv: (kv[0][1], kv[0][0])):
            piece = str(c)
            if s != 1:
                piece += f"*sqrt({s})"
            piece += {0: "", 1: "*pi", -1: "/pi"}[k]
            parts.append(piece)
        return " + ".join(parts)

    def rational_k(self):
        """(q, k) when the value is q * pi**k (one rational term), else None."""
        if self.is_zero():
            return F(0), 0
        if len(self.t) == 1:
            (s, k), c = next(iter(self.t.items()))
            if s == 1:
                return c, k
        return None


def record(v):
    v = v if isinstance(v, Real) else Real.of(v)
    rec = {"exact": v.text(), "decimal": v.decimal40(), "value": v.binary64()}
    if v.rational_k() is None:
        lo, hi = v.interval()
        rec["interval"] = [endpoint(lo, ROUND_FLOOR), endpoint(hi, ROUND_CEILING)]
        # Self-check, exact: the serialized endpoints enclose the internal rational interval.
        assert F(rec["interval"][0]) <= lo and hi <= F(rec["interval"][1]), (v.text(), rec["interval"])
    return rec


def endpoint(q, rounding):
    """q to 45 significant digits, rounded toward -inf (ROUND_FLOOR) or +inf (ROUND_CEILING)."""
    with localcontext() as ctx:
        ctx.rounding = rounding
        return format(Decimal(q.numerator) / Decimal(q.denominator), ".45e")


CROSS = []


def cross_check(name, a, b):
    ok = a == b
    CROSS.append({"check": name, "agrees": ok})
    if not ok:
        print(f"CROSS-CHECK FAILED: {name}: {a} != {b}", file=sys.stderr)
    return ok


# ----------------------------------------------------------------------------- inputs (invented; W16C's model)
D = F(1, 5)  # m, outside diameter
R = D / 2
T = F(1, 100)  # m, nominal wall
C_M = F(1, 800)  # m, mill tolerance
C_A = F(1, 500)  # m, corrosion allowance
T_I = F(1, 20)  # m, insulation thickness
RHO_M, RHO_C, RHO_I = F(7850), F(1000), F(150)  # kg/m^3: metal, contents, insulation
E = F(200 * 10 ** 9)  # Pa
NU = F(3, 10)
G = E / (2 * (1 + NU))
GRAVITY = F(981, 100)  # m/s^2 (the combination's seismic case)
K_Y = F(1, 4)  # g-factor along global Y (the combination's seismic case)
WIND_P = F(1200)  # Pa
WIND_C = F(7, 10)
COMBINATION_FACTORS = (F(1), F(1, 2))  # wind case, seismic case
STATIONS = (F(0), F(1, 4), F(1, 2), F(3, 4), F(1))
STATION_NAMES = ("0", "1/4", "1/2", "3/4", "1")
COMPONENTS_F = ("Fx", "Fy", "Fz", "Mx", "My", "Mz")
COMPONENTS_U = ("ux", "uy", "uz", "rx", "ry", "rz")
SECTION_COMPONENTS = ("axial_force", "shear_force_y", "shear_force_z",
                      "torsional_moment", "bending_moment_y", "bending_moment_z")
NEAR_FULL = 1 - F(1, 2 ** 40)

T_R = (T - C_M) - C_A
R_N = R - T
R_P = R - T_R


def annulus_p(r_o, r_i):
    return r_o * r_o - r_i * r_i


def inertia_p(r_i):
    return (R ** 4 - r_i ** 4) / 4


def kappa_energy(m, nu):
    n1 = 1 + nu
    c = (8 * nu + 14) * nu + 7
    den = c * (1 - m * m) ** 2 + 48 * m * m * n1 * n1
    return 6 * n1 * n1 * (1 + m * m) ** 2 / den


def kappa_energy_contract_form(m, nu):
    den = (7 + 14 * nu + 8 * nu * nu) * (1 + m * m) ** 2 + 4 * m * m * (5 + 10 * nu + 4 * nu * nu)
    return 6 * (1 + nu) ** 2 * (1 + m * m) ** 2 / den


A_N_P = annulus_p(R, R_N)  # nominal area / pi (stiffness)
A_R_P = annulus_p(R, R - T_R)  # reduced area / pi
I_N_P = inertia_p(R_N)
I_R_P = inertia_p(R - T_R)
KAPPA = kappa_energy(R_N / R, NU)
A_S_P = KAPPA * A_N_P
EI_P, KS_P, EA_P = E * I_N_P, G * A_S_P, E * A_N_P  # each / pi
MU_EX_P = RHO_M * annulus_p(R, R_N) + RHO_C * R_P * R_P + RHO_I * annulus_p(R + T_I, R)
Q_WIND = WIND_P * WIND_C * (D + 2 * T_I)  # N/m, rational
Q_SEISMIC_P = MU_EX_P * K_Y * GRAVITY  # N/m / pi
BASES = (("nominal_section", A_N_P, I_N_P), ("reduced_section", A_R_P, I_R_P))


def phi_of(length, timoshenko):
    return 12 * EI_P / (KS_P * length * length) if timoshenko else F(0)


# ----------------------------------------------------------------------------- the element (route 2)
def g(x):
    return x * x * (1 - x) ** 2


def feq_eb(w, length, a, b):
    def span(p):
        return p(b) - p(a)
    return [w * length * span(lambda x: x - x ** 3 + x ** 4 / 2),
            w * length * length * span(lambda x: x ** 2 / 2 - F(2, 3) * x ** 3 + x ** 4 / 4),
            w * length * span(lambda x: x ** 3 - x ** 4 / 2),
            w * length * length * span(lambda x: -x ** 3 / 3 + x ** 4 / 4)]


def feq_t(w, length, a, b, phi):
    e = feq_eb(w, length, a, b)
    dv = phi / (1 + phi) * w * length / 2 * (g(a) - g(b))
    return [e[0] + dv, e[1] + dv * length / 2, e[2] - dv, e[3] + dv * length / 2]


def k_t(ei, length, phi):
    c = ei / (1 + phi)
    L1, L2, L3 = length, length ** 2, length ** 3
    return [[12 * c / L3, 6 * c / L2, -12 * c / L3, 6 * c / L2],
            [6 * c / L2, (4 + phi) * c / L1, -6 * c / L2, (2 - phi) * c / L1],
            [-12 * c / L3, -6 * c / L2, 12 * c / L3, -6 * c / L2],
            [6 * c / L2, (2 - phi) * c / L1, -6 * c / L2, (4 + phi) * c / L1]]


def solve(A, r):
    n = len(A)
    M = [list(row) + [r[i]] for i, row in enumerate(A)]
    for i in range(n):
        p = next(k for k in range(i, n) if M[k][i] != 0)
        M[i], M[p] = M[p], M[i]
        for k in range(n):
            if k != i and M[k][i] != 0:
                f = M[k][i] / M[i][i]
                M[k] = [x - f * y for x, y in zip(M[k], M[i])]
    return [M[i][n] / M[i][i] for i in range(n)]


# ----------------------------------------------------------------------------- the continuum (route 1)
def pos(t):
    return t if t > 0 else F(0)


class Plane:
    """One bending plane (or the axial line when `axial`) of a straight line [0, Lt] under
    uniform segments (s_a, s_b, w), with supports 'cantilever' (fixed at 0) or 'fixed-fixed'."""

    def __init__(self, segments, total, supports, ei, ks, axial=False, ea=None):
        self.seg, self.Lt, self.ei, self.ks, self.axial, self.ea = segments, total, ei, ks, axial, ea
        if axial:
            if supports == "cantilever":
                self.n0 = self.W(0, total)
            else:
                self.n0 = self.W(1, total) / total
            return
        if supports == "cantilever":
            self.q0 = self.W(0, total)
            self.m0 = self.q0 * total - self.W(1, total)
        else:
            Lt = total
            flex = (ei / ks) if ks is not None else F(0)
            # theta(Lt) = 0: M0 Lt - Q0 Lt^2/2 + W2 = 0
            # v(Lt) = 0:     M0 Lt^2/2 - Q0 Lt^3/6 + W3 + flex (Q0 Lt - W1) = 0
            a11, a12, r1 = Lt, -Lt * Lt / 2, -self.W(2, Lt)
            a21, a22, r2 = Lt * Lt / 2, -Lt ** 3 / 6 + flex * Lt, -self.W(3, Lt) + flex * self.W(1, Lt)
            det = a11 * a22 - a12 * a21
            self.m0 = (r1 * a22 - a12 * r2) / det
            self.q0 = (a11 * r2 - a21 * r1) / det

    def W(self, n, x):
        fact = math.factorial(n + 1)
        return sum((w * (pos(x - sa) ** (n + 1) - pos(x - sb) ** (n + 1)) / fact for sa, sb, w in self.seg), F(0))

    def Q(self, x):
        return self.q0 - self.W(0, x)

    def M(self, x):
        return self.m0 - self.q0 * x + self.W(1, x)

    def theta(self, x):
        return (self.m0 * x - self.q0 * x * x / 2 + self.W(2, x)) / self.ei

    def v(self, x):
        shear = (self.q0 * x - self.W(1, x)) / self.ks if self.ks is not None else F(0)
        return (self.m0 * x * x / 2 - self.q0 * x ** 3 / 6 + self.W(3, x)) / self.ei + shear

    def N(self, x):
        return self.n0 - self.W(0, x)

    def u(self, x):
        return (self.n0 * x - self.W(1, x)) / self.ea


# ----------------------------------------------------------------------------- the structures
def dot(a, b):
    return sum(x * y for x, y in zip(a, b))


def frame_axes(chord, y_reference):
    """Rational local axes: x along the chord, y = y_reference projected normal to x, z = x cross y.
    The cases use chords and references whose normalizations are rational."""
    def unit(v):
        n2 = dot(v, v)
        r = (math.isqrt(n2.numerator), math.isqrt(n2.denominator))
        assert r[0] ** 2 == n2.numerator and r[1] ** 2 == n2.denominator, "irrational norm"
        n = F(r[0], r[1])
        return [c / n for c in v], n
    ex, length = unit(chord)
    proj = dot(y_reference, ex)
    ey, _ = unit([y - proj * x for y, x in zip(y_reference, ex)])
    ez = [ex[1] * ey[2] - ex[2] * ey[1], ex[2] * ey[0] - ex[0] * ey[2], ex[0] * ey[1] - ex[1] * ey[0]]
    return (ex, ey, ez), length


class Line:
    """Collinear pipes from node:a in one rational frame. `pipes` is [(id, from, to, length)],
    `marks` per pipe a list of fractions (a, b), `q` the global intensity vector and its pi power."""

    def __init__(self, nodes, pipes, axes, supports, marks, q, kq, timoshenko):
        self.nodes, self.pipes, self.axes, self.supports = nodes, pipes, axes, supports
        self.marks, self.kq, self.timoshenko = marks, kq, timoshenko
        self.s = {}  # node -> position along the line
        position = F(0)
        self.s[pipes[0][1]] = F(0)
        self.pipe_start = {}
        for pid, a, b, length in pipes:
            self.pipe_start[pid] = position
            position += length
            self.s[b] = position
        self.Lt = position
        self.local_q = [dot(axis, q) for axis in axes]  # (l_x, l_y, l_z) coefficients of pi**kq
        segs = []
        for pid, _, _, length in pipes:
            for fa, fb in marks.get(pid, []):
                s0 = self.pipe_start[pid]
                segs.append((s0 + fa * length, s0 + fb * length))
        self.segments = segs
        kind = "cantilever" if supports == ("node:a",) else "fixed-fixed"
        self.kind = kind
        ks = KS_P if timoshenko else None
        lx, ly, lz = self.local_q
        self.axial = Plane([(sa, sb, lx) for sa, sb in segs], self.Lt, kind, None, None, axial=True, ea=EA_P)
        # Plane y: v = u_y, theta = r_z, I_z; plane z: v = u_z, theta = -r_y, I_y (I_y = I_z here).
        self.py = Plane([(sa, sb, ly) for sa, sb in segs], self.Lt, kind, EI_P, ks)
        self.pz = Plane([(sa, sb, lz) for sa, sb in segs], self.Lt, kind, EI_P, ks)

    # local actions and motions at s ---------------------------------------
    def section(self, x):
        return {"axial_force": self.axial.N(x), "shear_force_y": self.py.Q(x), "shear_force_z": self.pz.Q(x),
                "torsional_moment": F(0), "bending_moment_y": -self.pz.M(x), "bending_moment_z": self.py.M(x)}

    def local_motion(self, x):
        return [self.axial.u(x), self.py.v(x), self.pz.v(x), F(0), -self.pz.theta(x), self.py.theta(x)]

    def to_global(self, local):
        ex, ey, ez = self.axes
        return [local[0] * ex[c] + local[1] * ey[c] + local[2] * ez[c] for c in range(3)]

    def global_motion(self, node):
        loc = self.local_motion(self.s[node])
        return self.to_global(loc[:3]) + self.to_global(loc[3:])

    def reaction(self, node):
        x = self.s[node]
        sec = self.section(x)
        force = [sec["axial_force"], sec["shear_force_y"], sec["shear_force_z"]]
        moment = [sec["torsional_moment"], sec["bending_moment_y"], sec["bending_moment_z"]]
        sign = -1 if x == 0 else 1  # left: -section(0); right: +section(Lt)
        return [sign * c for c in self.to_global(force)] + [sign * c for c in self.to_global(moment)]


def element_route(line, plane_name):
    """Route 2 for one plane: exact Timoshenko elements assembled and solved.
    Returns nodal (v, theta) and support reactions (F_v, M_theta), and checks closure (route 3)."""
    lq = {"y": line.local_q[1], "z": line.local_q[2]}[plane_name]
    order = [line.pipes[0][1]] + [p[2] for p in line.pipes]
    index = {n: i for i, n in enumerate(order)}
    ndof = 2 * len(order)
    K = [[F(0)] * ndof for _ in range(ndof)]
    f = [F(0)] * ndof
    elements = []
    for pid, a, b, length in line.pipes:
        phi = phi_of(length, line.timoshenko)
        ke = k_t(EI_P, length, phi)
        fe = [F(0)] * 4
        for fa, fb in line.marks.get(pid, []):
            fe = [x + y for x, y in zip(fe, feq_t(lq, length, fa, fb, phi))]
        dofs = [2 * index[a], 2 * index[a] + 1, 2 * index[b], 2 * index[b] + 1]
        for r in range(4):
            f[dofs[r]] += fe[r]
            for c in range(4):
                K[dofs[r]][dofs[c]] += ke[r][c]
        elements.append((pid, dofs, ke, fe, length, phi))
    fixed = set()
    for node in line.supports:
        fixed |= {2 * index[node], 2 * index[node] + 1}
    free = [i for i in range(ndof) if i not in fixed]
    d_free = solve([[K[r][c] for c in free] for r in free], [f[r] for r in free])
    d = [F(0)] * ndof
    for i, v in zip(free, d_free):
        d[i] = v
    reactions = {}
    for node in line.supports:
        rv = rm = F(0)
        for pid, dofs, ke, fe, length, phi in elements:
            de = [d[i] for i in dofs]
            end = [sum(ke[r][c] * de[c] for c in range(4)) - fe[r] for r in range(4)]
            for r in range(4):
                if dofs[r] == 2 * index[node]:
                    rv += end[r]
                if dofs[r] == 2 * index[node] + 1:
                    rm += end[r]
        reactions[node] = (rv, rm)
    # Route 3: each element's interior from its i-end state returns its j-end motion.
    for pid, dofs, ke, fe, length, phi in elements:
        de = [d[i] for i in dofs]
        end = [sum(ke[r][c] * de[c] for c in range(4)) - fe[r] for r in range(4)]
        q0, m0 = -end[0], -end[1]
        segs = [(fa * length, fb * length, lq) for fa, fb in line.marks.get(pid, [])]
        p = Plane(segs, length, "cantilever", EI_P, KS_P if line.timoshenko else None)
        p.q0, p.m0 = q0, m0
        cross_check(f"{line.cid} plane {plane_name} {pid}: closure v(L) from the i end", de[0] + de[1] * length + p.v(length), de[2])
        cross_check(f"{line.cid} plane {plane_name} {pid}: closure theta(L) from the i end", de[1] + p.theta(length), de[3])
    return {n: (d[2 * index[n]], d[2 * index[n] + 1]) for n in order}, reactions


def axial_route(line):
    """Route 2 for the axial line: two-node bars with consistent axial loads."""
    lx = line.local_q[0]
    order = [line.pipes[0][1]] + [p[2] for p in line.pipes]
    index = {n: i for i, n in enumerate(order)}
    n = len(order)
    K = [[F(0)] * n for _ in range(n)]
    f = [F(0)] * n
    for pid, a, b, length in line.pipes:
        k = EA_P / length
        fe = [F(0), F(0)]
        for fa, fb in line.marks.get(pid, []):
            fe[0] += lx * length * ((fb - fb * fb / 2) - (fa - fa * fa / 2))
            fe[1] += lx * length * (fb * fb / 2 - fa * fa / 2)
        i, j = index[a], index[b]
        K[i][i] += k
        K[j][j] += k
        K[i][j] -= k
        K[j][i] -= k
        f[i] += fe[0]
        f[j] += fe[1]
    fixed = {index[s] for s in line.supports}
    free = [i for i in range(n) if i not in fixed]
    uf = solve([[K[r][c] for c in free] for r in free], [f[r] for r in free])
    u = [F(0)] * n
    for i, v in zip(free, uf):
        u[i] = v
    return {node: u[index[node]] for node in order}


def check_line(line):
    """Routes 1-3 agree; cantilever tips also by the unit-load method; global equilibrium."""
    for name, plane in (("y", line.py), ("z", line.pz)):
        nodal, reactions = element_route(line, name)
        for node, (v, th) in nodal.items():
            x = line.s[node]
            cross_check(f"{line.cid} plane {name} {node}: v continuum = element", plane.v(x), v)
            cross_check(f"{line.cid} plane {name} {node}: theta continuum = element", plane.theta(x), th)
        for node, (rv, rm) in reactions.items():
            x = line.s[node]
            sign = -1 if x == 0 else 1
            cross_check(f"{line.cid} plane {name} {node}: reaction force continuum = element", sign * plane.Q(x), rv)
            cross_check(f"{line.cid} plane {name} {node}: reaction moment continuum = element", sign * plane.M(x), rm)
        if line.kind == "cantilever":
            # Unit-load method: tip v = int M(x)(Lt - x)/EI + int Q/K_s; theta = int M/EI.
            lt = line.Lt
            cuts = sorted({F(0), lt} | {s for seg in line.segments for s in seg})
            def integral(fn):
                return sum((x1 - x0) * (fn(x0) + 4 * fn((x0 + x1) / 2) + fn(x1)) / 6 for x0, x1 in zip(cuts, cuts[1:]))
            tip = integral(lambda x: plane.M(x) * (lt - x)) / EI_P
            if line.timoshenko:
                tip += integral(plane.Q) / KS_P
            cross_check(f"{line.cid} plane {name}: tip v by the unit-load method", tip, plane.v(lt))
            cross_check(f"{line.cid} plane {name}: tip theta by the unit-load method", integral(plane.M) / EI_P, plane.theta(lt))
    for node, u in axial_route(line).items():
        cross_check(f"{line.cid} axial {node}: u continuum = element", line.axial.u(line.s[node]), u)
    # Global equilibrium of the line: support reactions balance the load (force and moment about A).
    total = [F(0)] * 3
    moment = [F(0)] * 3
    qg = line.to_global(line.local_q)
    ex = line.axes[0]
    for sa, sb in line.segments:
        w = sb - sa
        centroid = (sa + sb) / 2
        arm = [centroid * c for c in ex]
        for c in range(3):
            total[c] += qg[c] * w
        cross = [arm[1] * qg[2] - arm[2] * qg[1], arm[2] * qg[0] - arm[0] * qg[2], arm[0] * qg[1] - arm[1] * qg[0]]
        for c in range(3):
            moment[c] += cross[c] * w
    for node in line.supports:
        r = line.reaction(node)
        arm = [line.s[node] * c for c in ex]
        for c in range(3):
            total[c] += r[c]
            moment[c] += r[3 + c]
        cross = [arm[1] * r[2] - arm[2] * r[1], arm[2] * r[0] - arm[0] * r[2], arm[0] * r[1] - arm[1] * r[0]]
        for c in range(3):
            moment[c] += cross[c]
    cross_check(f"{line.cid}: support forces balance the load", total, [F(0)] * 3)
    cross_check(f"{line.cid}: support moments balance the load about node:a", moment, [F(0)] * 3)


# ----------------------------------------------------------------------------- publication
def family(unit_name):
    return {"N": "force", "N*m": "moment", "m": "translation", "rad": "rotation", "Pa": "stress"}[unit_name]


def support_id(node):
    return "support:" + node.split(":", 1)[1]


def with_pi(v, k):
    """The same coefficients and radicands, carrying pi**k."""
    return Real({(s, k): c for (s, _), c in v.t.items()})


def hypot_real(components, coef=1, k=0):
    return Real.sqrt_of(sum(c * c for c in components), coef, k)


def elastic_maximum(line, pid, length, area, inertia):
    """max over the pipe of |N|/A_b + hypot(My, Mz) R / I_b (each / pi), with its location.
    Candidates: the pipe's ends, load breakpoints inside it, and the zeros of each plane's Q
    (interior extrema of M); cross-checked on a 1/256 grid."""
    s0 = line.pipe_start[pid]
    cands = {s0, s0 + length}
    for sa, sb in line.segments:
        for x in (sa, sb):
            if s0 < x < s0 + length:
                cands.add(x)
    cuts = sorted(cands)
    for plane in (line.py, line.pz):
        for x0, x1 in zip(cuts, cuts[1:]):
            q0, q1 = plane.Q(x0), plane.Q(x1)
            mid = (x0 + x1) / 2
            loaded = any(sa <= mid <= sb for sa, sb in line.segments)
            if loaded and q0 != q1 and (q0 > 0) != (q1 > 0):
                # Q is linear on a loaded interval: its zero.
                cands.add(x0 + (x1 - x0) * q0 / (q0 - q1))

    def value(x):
        sec = line.section(x)
        return (Real.of(abs(sec["axial_force"]) / area)
                + hypot_real([sec["bending_moment_y"], sec["bending_moment_z"]], R / inertia))

    best_x, best = None, None
    for x in sorted(cands):
        v = value(x)
        lo, hi = v.interval()
        if best is None or lo > best.interval()[1]:
            best_x, best = x, v
    blo, bhi = best.interval()
    grid = all(value(s0 + length * F(i, 256)).interval()[0] <= bhi for i in range(257))
    cross_check(f"{line.cid} {pid}: the maximum on {area == A_N_P and 'nominal' or 'reduced'} bounds a 1/256 grid", grid, True)
    return best, (best_x - s0) / length


def publish(line, k):
    """Rows of one line case. Forces and moments are coefficients of pi**k; motions, stresses
    and maxima of pi**(k - 1)."""
    rows = []
    km = k - 1
    for node in sorted(line.supports):
        r = line.reaction(node)
        sup = support_id(node)
        for comp, val in zip(COMPONENTS_F, r):
            unit_name = "N" if comp.startswith("F") else "N*m"
            rows.append(dict(quantity="support_reaction", entity=sup, component=comp, unit=unit_name,
                             **record(Real.of(val, k))))
        rows.append(dict(quantity="support_reaction_magnitude", entity=sup, component="force_magnitude",
                         unit="N", **record(hypot_real(r[:3], 1, k))))
        rows.append(dict(quantity="support_reaction_magnitude", entity=sup, component="moment_magnitude",
                         unit="N*m", **record(hypot_real(r[3:], 1, k))))
    for nd in sorted(line.s):
        m = line.global_motion(nd)
        for comp, val in zip(COMPONENTS_U, m):
            unit_name = "m" if comp.startswith("u") else "rad"
            rows.append(dict(quantity="nodal_motion", entity=nd, component=comp, unit=unit_name,
                             **record(Real.of(val, km))))
    for pid, a, b, length in line.pipes:
        s0 = line.pipe_start[pid]
        secs = [line.section(s0 + st * length) for st in STATIONS]
        for name, sec in zip(STATION_NAMES, secs):
            for comp in SECTION_COMPONENTS:
                unit_name = "N" if comp in ("axial_force", "shear_force_y", "shear_force_z") else "N*m"
                rows.append(dict(quantity="section_action", entity=pid, station=name, component=comp,
                                 unit=unit_name, **record(Real.of(sec[comp], k))))
        for moment_component, stress_component in (("bending_moment_y", "bending_normal_stress_y"),
                                                   ("bending_moment_z", "bending_normal_stress_z")):
            if all(sec[moment_component] == 0 for sec in secs) and not line.always_both_stresses:
                continue
            for basis, area, inertia in BASES:
                for name, sec in zip(STATION_NAMES, secs):
                    rows.append(dict(quantity="section_stress", entity=pid, station=name, component=stress_component,
                                     section_basis=basis, unit="Pa", **record(Real.of(sec[moment_component] * R / inertia, km))))
        for basis, area, inertia in BASES:
            best, where = elastic_maximum(line, pid, length, area, inertia)
            rows.append(dict(quantity="elastic_maximum", entity=pid, component="maximum_absolute_normal_stress",
                             section_basis=basis, unit="Pa", governing_fraction=str(where),
                             **record(with_pi(best, km))))
    return rows


def zero_scales(rows):
    scales = {}
    for fam in ("force", "moment", "translation", "rotation", "stress"):
        mags = [abs(r["value"]) for r in rows if family(r["unit"]) == fam]
        if not mags:
            continue
        top = max(mags)
        assert top > 0, fam
        unit_name = {"force": "N", "moment": "N*m", "translation": "m", "rotation": "rad", "stress": "Pa"}[fam]
        scales[fam] = {"unit": unit_name, "value": top}
    return scales


# ----------------------------------------------------------------------------- cases
X_AXES = ([F(1), F(0), F(0)], [F(0), F(1), F(0)], [F(0), F(0), F(1)])
WIND_Z = [F(0), F(0), Q_WIND]
SEISMIC_Y = [F(0), Q_SEISMIC_P, F(0)]


def cantilever_line(cid, length, marks, timoshenko, q=WIND_Z, kq=0):
    line = Line({"node:a": [0, 0, 0], "node:b": [length, 0, 0]}, [("pipe:a-b", "node:a", "node:b", length)],
                X_AXES, ("node:a",), {"pipe:a-b": marks}, q, kq, timoshenko)
    line.cid = cid
    return line


def build_lines():
    L, S = F(5, 2), F(1, 2)
    T_, EB = True, False
    lines = []

    def add(cid, line, request):
        line.cid = cid
        line.request = request
        line.always_both_stresses = False
        lines.append(line)

    def cant(cid, length, marks, timo):
        add(cid, cantilever_line(cid, length, marks, timo),
            {"model": "cantilever", "length_m": str(length), "marks": {"pipe:a-b": [[str(a), str(b)] for a, b in marks]}})

    cant("W16T-CANT-INTERIOR", L, [(F(1, 5), F(7, 10))], T_)
    cant("W16T-CANT-PROXIMAL", L, [(F(0), F(1, 2))], T_)
    cant("W16T-CANT-DISTAL", L, [(F(1, 2), F(1))], T_)
    cant("W16T-CANT-SHORT-INTERIOR", S, [(F(1, 5), F(7, 10))], T_)
    cant("W16T-CANT-SHORT-PROXIMAL", S, [(F(0), F(1, 2))], T_)
    cant("W16T-CANT-SYMMETRIC", L, [(F(1, 4), F(3, 4))], T_)
    cant("W16T-CANT-NEARFULL", L, [(F(0), NEAR_FULL)], T_)
    cant("W16T-CANT-FULL-SPANS", L, [(F(0), F(1))], T_)
    cant("W16T-CANT-TWO-EXTENTS", L, [(F(0), F(1, 5)), (F(3, 5), F(1))], T_)
    cant("W16T-CANT-INTERIOR-EB", L, [(F(1, 5), F(7, 10))], EB)
    cant("W16T-CANT-SHORT-INTERIOR-EB", S, [(F(1, 5), F(7, 10))], EB)

    sub = Line({"node:a": [0, 0, 0], "node:m": [1, 0, 0], "node:b": [L, 0, 0]},
               [("pipe:a-m", "node:a", "node:m", F(1)), ("pipe:m-b", "node:m", "node:b", F(3, 2))],
               X_AXES, ("node:a",), {"pipe:a-m": [(F(1, 2), F(1))], "pipe:m-b": [(F(0), F(1, 2))]}, WIND_Z, 0, T_)
    add("W16T-CANT-SUBDIVIDED", sub, {"model": "subdivided_cantilever", "nodes": {"node:m": "1", "node:b": str(L)},
                                       "marks": {"pipe:a-m": [["1/2", "1"]], "pipe:m-b": [["0", "1/2"]]}})
    for cid, timo in (("W16T-FIXED-FIXED", T_), ("W16T-FIXED-FIXED-EB", EB)):
        ff = Line({"node:a": [0, 0, 0], "node:m": [L, 0, 0], "node:c": [2 * L, 0, 0]},
                  [("pipe:a-m", "node:a", "node:m", L), ("pipe:m-c", "node:m", "node:c", L)],
                  X_AXES, ("node:a", "node:c"), {"pipe:a-m": [(F(1, 5), F(7, 10))], "pipe:m-c": [(F(1, 2), F(1))]},
                  WIND_Z, 0, timo)
        add(cid, ff, {"model": "fixed_fixed", "nodes": {"node:m": str(L), "node:c": str(2 * L)},
                      "marks": {"pipe:a-m": [["1/5", "7/10"]], "pipe:m-c": [["1/2", "1"]]}})
    for cid, timo in (("W16T-SKEW", T_), ("W16T-SKEW-EB", EB)):
        chord, yref = [F(1), F(3, 2), F(3)], [F(3), F(-6), F(2)]
        axes, length = frame_axes(chord, yref)
        skew = Line({"node:a": [0, 0, 0], "node:b": [str(c) for c in chord]}, [("pipe:a-b", "node:a", "node:b", length)],
                    axes, ("node:a",), {"pipe:a-b": [(F(1, 5), F(7, 10))]}, WIND_Z, 0, timo)
        add(cid, skew, {"model": "skew_cantilever", "node_b": [str(c) for c in chord], "y_reference": [str(c) for c in yref],
                        "length_m": str(length), "marks": {"pipe:a-b": [["1/5", "7/10"]]}})
        skew.always_both_stresses = True
    return lines


def case_record(line):
    k = line.kq
    rows = publish(line, k)
    phis = {pid: record(phi_of(length, line.timoshenko)) for pid, _, _, length in line.pipes}
    return {"id": line.cid, "route": "EX",
            "shear_model": "timoshenko_annulus_energy_v1" if line.timoshenko else "euler_bernoulli",
            "request": line.request, "supports": list(line.supports), "phi": phis,
            "local_axes": [[str(c) for c in axis] for axis in line.axes],
            "local_intensity_N_per_m": [str(c) for c in line.local_q],
            "method": "continuum force method (Macaulay integrals) = exact Timoshenko element assembly = element closure; "
                      "unit-load tips on cantilevers",
            "zero_scales": zero_scales(rows), "rows": rows}


def combination_record():
    """W16T-COMBINED: the interior Timoshenko wind case plus 1/2 the seismic case (global Y, full span)
    on the same cantilever, as a mechanics combination of the two primitive cases."""
    L = F(5, 2)
    wind = cantilever_line("W16T-COMBINED:wind", L, [(F(1, 5), F(7, 10))], True)
    seismic = cantilever_line("W16T-COMBINED:seismic", L, [(F(0), F(1))], True, SEISMIC_Y, 1)
    for line in (wind, seismic):
        check_line(line)
    fw, fs = COMBINATION_FACTORS
    rows = []
    sw, ss = wind.reaction("node:a"), seismic.reaction("node:a")
    for i, comp in enumerate(COMPONENTS_F):
        unit_name = "N" if comp.startswith("F") else "N*m"
        rows.append(dict(quantity="support_reaction", entity="support:a", component=comp, unit=unit_name,
                         **record(Real.of(fw * sw[i], 0) + Real.of(fs * ss[i], 1))))
    for nd in ("node:a", "node:b"):
        mw, ms = wind.global_motion(nd), seismic.global_motion(nd)
        for i, comp in enumerate(COMPONENTS_U):
            unit_name = "m" if comp.startswith("u") else "rad"
            rows.append(dict(quantity="nodal_motion", entity=nd, component=comp, unit=unit_name,
                             **record(Real.of(fw * mw[i], -1) + Real.of(fs * ms[i], 0))))
    for name, st in zip(STATION_NAMES, STATIONS):
        a, b = wind.section(st * L), seismic.section(st * L)
        for comp in SECTION_COMPONENTS:
            unit_name = "N" if comp in ("axial_force", "shear_force_y", "shear_force_z") else "N*m"
            rows.append(dict(quantity="section_action", entity="pipe:a-b", station=name, component=comp, unit=unit_name,
                             **record(Real.of(fw * a[comp], 0) + Real.of(fs * b[comp], 1))))
    return {"id": "W16T-COMBINED", "route": "EX", "shear_model": "timoshenko_annulus_energy_v1",
            "request": {"model": "cantilever", "length_m": str(L),
                        "cases": {"case:w16t": {"wind_marks": {"pipe:a-b": [["1/5", "7/10"]]}},
                                  "case:w16t-seismic": {"seismic": {"g_factor_y": str(K_Y), "gravity_m_per_s2": str(GRAVITY)}}},
                        "combination": {"id": "combination:w16t", "basis": "mechanics",
                                        "terms": [["case:w16t", str(fw)], ["case:w16t-seismic", str(fs)]]}},
            "method": "the two primitive cases' continuum solutions (each cross-checked by routes 1-3), combined linearly",
            "zero_scales": zero_scales(rows), "rows": rows}


# ----------------------------------------------------------------------------- element controls E1-E9
def element_controls():
    out = []

    def add(cid, what, values, expected):
        ok = cross_check(f"{cid}: {what}", values, expected)
        out.append({"id": cid, "check": what, "agrees": ok, "values": [str(v) for v in values]})

    def fixed_fixed_force_method(w, length, a, b, ei, ks):
        p = Plane([(a * length, b * length, w)], length, "fixed-fixed", ei, ks)
        vj = w * length * (b - a) - p.q0
        mj = w * (b * b - a * a) * length * length / 2 - p.m0 - length * vj
        return [p.q0, p.m0, vj, mj]

    one = F(1)
    # E1: the distal half at L = 1, w = 1, phi = 1 (EI = 1, K_s = 12).
    t = feq_t(one, one, F(1, 2), one, one)
    add("E1", "distal half [7/64, 13/384, 25/64, -19/384]", t, [F(7, 64), F(13, 384), F(25, 64), F(-19, 384)])
    add("E1", "distal half = the fixed-fixed force method", t, fixed_fixed_force_method(one, one, F(1, 2), one, one, F(12)))
    K = k_t(one, one, one)
    tip = solve([[K[2][2], K[2][3]], [K[3][2], K[3][3]]], [t[2], t[3]])
    add("E1", "cantilever tip v 53/384, theta 7/48", tip, [F(53, 384), F(7, 48)])
    stale = feq_eb(one, one, F(1, 2), one)
    add("E1", "stale Euler-Bernoulli vector [3/32, 5/192, 13/32, -11/192]", stale, [F(3, 32), F(5, 192), F(13, 32), F(-11, 192)])
    add("E1", "the stale vector with Timoshenko K: tip v 54/384, theta 7/48",
        solve([[K[2][2], K[2][3]], [K[3][2], K[3][3]]], [stale[2], stale[3]]), [F(54, 384), F(7, 48)])
    # E2: the proximal half at phi = 1 and 1/3.
    for phi in (one, F(1, 3)):
        add("E2", f"proximal half at phi = {phi}: force method", feq_t(one, one, F(0), F(1, 2), phi),
            fixed_fixed_force_method(one, one, F(0), F(1, 2), one, 12 / phi))
    # E3: the interior [1/5, 7/10] over phi in {1e-6, 1/3, 1, 2, 17/2, 40}.
    for phi in (F(1, 10 ** 6), F(1, 3), one, F(2), F(17, 2), F(40)):
        L = F(5, 2)
        ei = F(7, 3)
        ks = 12 * ei / (phi * L * L)
        add("E3", f"interior at phi = {phi}: force method", feq_t(F(-11, 4), L, F(1, 5), F(7, 10), phi),
            fixed_fixed_force_method(F(-11, 4), L, F(1, 5), F(7, 10), ei, ks))
    # E4: symmetric [1/4, 3/4] equals Euler-Bernoulli in exact arithmetic.
    add("E4", "symmetric span: feq_T = feq_EB (exact arithmetic)", feq_t(one, F(5, 2), F(1, 4), F(3, 4), F(17, 2)),
        feq_eb(one, F(5, 2), F(1, 4), F(3, 4)))
    # E5: the full span is the existing full-span result.
    add("E5", "full span = [wL/2, wL^2/12, wL/2, -wL^2/12]", feq_t(F(3), F(5, 2), F(0), one, F(2)),
        [F(15, 4), F(75, 48), F(15, 4), F(-75, 48)])
    # E6: phi -> 0: dV0 / phi -> (wL/2)(g(a) - g(b)) at rate phi; equality at phi = 0.
    w, L, a, b = F(1), F(5, 2), F(1, 5), F(7, 10)
    limit = w * L / 2 * (g(a) - g(b))
    prev = None
    for kpow in range(1, 9):
        phi = F(1, 10 ** kpow)
        dv = feq_t(w, L, a, b, phi)[0] - feq_eb(w, L, a, b)[0]
        gap = abs(dv / phi - limit)
        add("E6", f"phi = 1e-{kpow}: |dV0/phi - limit| = phi |limit|/(1+phi)", [gap], [phi * abs(limit) / (1 + phi)])
        if prev is not None:
            add("E6", f"phi = 1e-{kpow}: the gap decreases", [gap < prev], [True])
        prev = gap
    add("E6", "phi = 0 equals Euler-Bernoulli", feq_t(w, L, a, b, F(0)), feq_eb(w, L, a, b))
    # E7: fixed-fixed 2L as two elements, load in element 1 = one element of length 2L at phi/4.
    for phi in (F(1, 3), F(2), F(9)):
        for a, b in ((F(1, 5), F(7, 10)), (F(0), F(1, 2)), (F(1, 2), F(1))):
            L = F(5, 2)
            t1 = feq_t(F(-11, 4), L, a, b, phi)
            K = k_t(F(7, 3), L, phi)
            A = [[K[2][2] + K[0][0], K[2][3] + K[0][1]], [K[3][2] + K[1][0], K[3][3] + K[1][1]]]
            d = solve(A, [t1[2], t1[3]])
            left = [-(K[0][2] * d[0] + K[0][3] * d[1] - t1[0]), -(K[1][2] * d[0] + K[1][3] * d[1] - t1[1])]
            right = [-(K[2][0] * d[0] + K[2][1] * d[1]), -(K[3][0] * d[0] + K[3][1] * d[1])]
            add("E7", f"phi = {phi}, [{a}, {b}]: two elements = one 2L element at phi/4", left + right,
                feq_t(F(-11, 4), 2 * L, a / 2, b / 2, phi / 4))
    # E8: a cantilever split into 1, 2 and 4 elements, the extent [1/5, 7/10] crossing element boundaries.
    L, phi_l = F(5, 2), F(17, 4)
    ei = F(7, 3)
    ks = 12 * ei / (phi_l * L * L)
    tips = []
    for n in (1, 2, 4):
        le = L / n
        phi = 12 * ei / (ks * le * le)
        ndof = 2 * (n + 1)
        Kg = [[F(0)] * ndof for _ in range(ndof)]
        fg = [F(0)] * ndof
        for e in range(n):
            x0, x1 = e * le, (e + 1) * le
            lo, hi = max(x0, F(1, 5) * L), min(x1, F(7, 10) * L)
            fe = feq_t(F(-11, 4), le, (lo - x0) / le, (hi - x0) / le, phi) if hi > lo else [F(0)] * 4
            ke = k_t(ei, le, phi)
            dofs = [2 * e, 2 * e + 1, 2 * e + 2, 2 * e + 3]
            for r in range(4):
                fg[dofs[r]] += fe[r]
                for c in range(4):
                    Kg[dofs[r]][dofs[c]] += ke[r][c]
        free = list(range(2, ndof))
        dsol = solve([[Kg[r][c] for c in free] for r in free], [fg[r] for r in free])
        tips.append(dsol[-2:])
    add("E8", "1, 2 and 4 elements give the same tip (v, theta)", tips[1] + tips[2], tips[0] + tips[0])
    # E9: unequal planes: E = G = A = J = 1, A_s = 12, L = 1, I_y = 1 (phi_y = 1), I_z = 2 (phi_z = 2).
    add("E9", "local y (phi_z = 2) distal half [11/96, 7/192, 37/96, -3/64]", feq_t(one, one, F(1, 2), one, F(2)),
        [F(11, 96), F(7, 192), F(37, 96), F(-3, 64)])
    add("E9", "local z (phi_y = 1) distal half [7/64, 13/384, 25/64, -19/384] (r_y takes the moments negated)",
        feq_t(one, one, F(1, 2), one, one), [F(7, 64), F(13, 384), F(25, 64), F(-19, 384)])
    return out


# ----------------------------------------------------------------------------- document
def inputs_record():
    return {
        "section": {"outside_diameter_m": str(D), "wall_thickness_m": str(T), "mill_tolerance_m": str(C_M),
                    "corrosion_allowance_m": str(C_A), "insulation_thickness_m": str(T_I),
                    "material_density_kg_per_m3": str(RHO_M), "contents_density_kg_per_m3": str(RHO_C),
                    "insulation_density_kg_per_m3": str(RHO_I)},
        "material": {"elastic_modulus_Pa": str(E), "poisson_ratio": str(NU)},
        "wind": {"pressure_Pa": str(WIND_P), "shape_factor": str(WIND_C), "direction": "global_z"},
        "seismic": {"gravity_acceleration_m_per_s2": str(GRAVITY), "g_factor_y": str(K_Y)},
        "near_full_end_fraction": str(NEAR_FULL),
    }


def derived_record():
    return {
        "nominal_inner_radius_m": str(R_N), "reduced_wall_m": str(T_R),
        "area_nominal_m2": record(Real.of(A_N_P, 1)), "area_reduced_m2": record(Real.of(A_R_P, 1)),
        "second_moment_nominal_m4": record(Real.of(I_N_P, 1)), "second_moment_reduced_m4": record(Real.of(I_R_P, 1)),
        "shear_factor_kappa_E": record(KAPPA), "shear_area_m2": record(Real.of(A_S_P, 1)),
        "phi_at_L_5_2": record(phi_of(F(5, 2), True)), "phi_at_L_1_2": record(phi_of(F(1, 2), True)),
        "phi_at_L_1": record(phi_of(F(1), True)), "phi_at_L_3_2": record(phi_of(F(3, 2), True)),
        "phi_at_L_7_2": record(phi_of(F(7, 2), True)),
        "wind_intensity_N_per_m": record(Q_WIND),
        "seismic_intensity_N_per_m": record(Real.of(Q_SEISMIC_P, 1)),
        "mass_per_length_kg_per_m": record(Real.of(MU_EX_P, 1)),
    }


def document(cases, controls):
    return {
        "document": "openpipestress.w16_timoshenko_partial_wind_references",
        "status": "candidate frozen references; derived and cross-checked by their author, not yet independently refuted",
        "units": {"force": "N", "moment": "N*m", "translation": "m", "rotation": "rad", "stress": "Pa"},
        "theory": "small-displacement linear-elastic Timoshenko (or Euler-Bernoulli) straight members; K_s = G kappa_E A_n "
                  "on the nominal annulus; rotations are section rotations",
        "conventions": {
            "support_reaction": "global action of the support on the structure at its node (moments about that node)",
            "support_reaction_magnitude": "Euclidean norm of the support's force or moment components",
            "nodal_motion": "global translation and right-handed rotation",
            "section_action": "element-local action on the +x face of the i-side portion at station s/L (local x from i to j); "
                              "a node-on-element end action is -section(0) at end_i and +section(1) at end_j",
            "section_stress": "bending normal stress M / Z of the station's section action on the named basis, Z = I / R",
            "elastic_maximum": "max over the pipe of |N|/A_b + hypot(M_y, M_z)/Z_b on the named basis; governing_fraction "
                               "is its location s/L",
            "local_axes": "x along the chord from i to j; y = y_reference projected normal to x; z = x cross y",
            "exact": "a sum of rational * [sqrt(rational)] * [pi or 1/pi]; irrational values carry a rigorous interval",
        },
        "criterion": "|observed - expected| <= 1e-9 * |expected|; an exactly zero expectation uses 1e-9 * zero_scale "
                     "of its family (largest magnitude of that family in the case)",
        "inputs": inputs_record(),
        "derived": derived_record(),
        "element_controls": controls,
        "crosscheck": list(CROSS),
        "cases": cases,
    }


def dumps(obj):
    def inline(o):
        return json.dumps(o, ensure_ascii=False, separators=(", ", ": "))

    def emit(o, level):
        pad = " " * level
        if isinstance(o, dict) and ("exact" in o or "check" in o):
            return inline(o)
        if isinstance(o, dict):
            items = [f'{pad} {json.dumps(k)}: {emit(v, level + 1)}' for k, v in o.items()]
            return "{\n" + ",\n".join(items) + "\n" + pad + "}" if items else "{}"
        if isinstance(o, list) and o and all(isinstance(x, dict) for x in o):
            items = [f"{pad} {emit(v, level + 1)}" for v in o]
            return "[\n" + ",\n".join(items) + "\n" + pad + "]"
        return inline(o)
    return emit(obj, 0) + "\n"


def sha256sums(files):
    lines = [f"{hashlib.sha256(data).hexdigest()}  {path}" for path, data in sorted(files.items())]
    return ("\n".join(lines) + "\n").encode()


def build():
    CROSS.clear()
    cross_check("kappa_E: cancellation-free form equals the contract form", KAPPA, kappa_energy_contract_form(R_N / R, NU))
    controls = element_controls()
    cases = []
    for line in build_lines():
        check_line(line)
        cases.append(case_record(line))
    cases.append(combination_record())
    doc = document(cases, controls)
    text = dumps(doc)
    assert json.loads(text) == json.loads(json.dumps(doc))
    files = {"references.json": text.encode()}
    for fixed in FIXED_FILES:
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
    print(f"{len(cases)} cases, {sum(len(c['rows']) for c in cases)} rows")
    print(f"cross-checks: {len(CROSS)} recorded, {len(bad)} disagreeing")
    for c in cases:
        tip = [r for r in c["rows"] if r["quantity"] == "nodal_motion" and r["entity"] in ("node:b", "node:m")
               and r["component"] in ("uy", "uz")]
        print(c["id"], ", ".join(f"{r['entity']} {r['component']} {r['value']:.6g}" for r in tip))
    if args.write:
        if bad:
            print("not written: a cross-check disagrees")
            return 1
        for path, data in files.items():
            if path in FIXED_FILES:
                continue
            target = OUT / path
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(data)
        print(f"wrote {len(files) - len([f for f in FIXED_FILES if f in files])} files")
        return 0
    failures = [p for p, d in files.items() if not (OUT / p).exists() or (OUT / p).read_bytes() != d]
    committed = {str(p.relative_to(OUT)) for p in OUT.rglob("*") if p.is_file() and "__pycache__" not in p.parts}
    extra = sorted(committed - set(files))
    if failures or extra or bad:
        print("MISMATCH: " + ", ".join(failures + [f"unexpected {e}" for e in extra]))
        return 1
    print(f"verified {len(files)} files byte for byte (SHA256SUMS included)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
