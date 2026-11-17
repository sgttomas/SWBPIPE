#!/usr/bin/env python3
"""Seismic component-weight references: the self-weight plan evaluated at the seismic case's gravity.

Standard-library Python 3 only. This generator imports, reads and runs no
product code and reads only its package's files (the outputs it writes,
README.md and its own source). Every expected
value is derived here in exact rational arithmetic, with pi carried
symbolically, from linear-elastic small-displacement Euler-Bernoulli frame
mechanics and from the seismic rule SWBPIPE's Agent 0 decided for DEL-05-01:
the generated seismic load is "the self-weight plan evaluated at the seismic
case's gravity, with each entered component force W held fixed". For each
global axis, on every member the load per length is the g-factor times the
seismic case's gravity times the member's mass per length; for a member whose
component weight the self-weight plan applies, in place of that load, the force
is the g-factor times the component's weight W at the component's node, with
the moments of that force about the node from the centre-of-gravity vector; a
member whose component weight the plan does not apply (here: a component
declared not consumed by the solve, DEL-03-05, or one whose entered weight is
exactly zero, which is not applied by the Agent 0's decision) keeps its
distributed load.

This is a force convention, not a statement that the seismic load uses one
fixed mass. Equal gravities are a sufficient condition for it to coincide with
acceleration of the model's self-weight mass: when the seismic case's gravity
equals the gravity of an applied self-weight, the two agree. When they differ,
the pipes' loads scale with the seismic case's gravity and the component force
does not; the difference the component makes is k W (1 - g_s/g_w), which tends
to zero with W, so in the limit of an applied weight tending to zero the two
agree at any gravity (boundary 4, a continuity witness). No entered document
reaches that limit: SWBPIPE's Agent 0 decided, not yet scope text, that "a
component's weight is applied only when it is a finite positive force; a weight
of zero is not applied". An entered weight of exactly zero is therefore not
applied, and its member keeps its distributed load (CTRL-ZERO-AS-NOT-APPLIED,
the reference for that document). Where the applied self-weight's gravity is absent
or ambiguous, the equivalence of the two mass models is not established; the
values here are what the convention yields. A product is expected to carry a
model-check warning, an exception the convention names, on a seismic case that
carries an applied component weight and states a gravity different from the
applied self-weight's; the absence of that warning establishes no mass model.

Every quantity is a Laurent polynomial in pi with rational coefficients and
exponents -1, 0 or 1: masses per length and section properties are rational
multiples of pi, component weights are rational. The response is solved by two
methods:

  method 1  direct stiffness: 12-DOF Euler-Bernoulli frame elements along
            global X, work-consistent equivalent nodal loads, exact Gaussian
            elimination; end forces k u - f_eq; reactions from nodal balance.
  method 2  statics and unit-load virtual work: section actions as exact
            polynomials of x, deflections by exact integration of M m / EI,
            N n / EA and T t / GJ; the both-ends-anchored twin by the force
            method (the six actions at D are the redundants).

The two methods are not wholly independent. Besides the inputs and the
generated loads they share the geometry, the section and stiffness
construction (member_props), the exact arithmetic (Fraction and the class P)
and the linear solver (gauss_solve, through solve_p: method 1 for the
displacements, method 2 for the redundants). They differ in how the response
is formed: element stiffness assembly against section-action statics and
virtual-work integrals.

Every response row of method 1 must equal method 2 exactly. The further checks
are invariants of one solution, not further solution methods: global
equilibrium of every case about the origin; the force and the moment about
the origin of the equivalent nodal loads against the resultant of the
generated loads; member and free-node balance; the row identity of the Z
seismic case against the weight case; and the parity of each seismic resultant
with the weight. Parity calls the same generation and resultant functions on
both sides, so it shows the decided rule's identity with the weight case but
does not independently establish which constituents of mass are selected.
Finally the whole build is repeated with every input replaced by its nearest
binary64 value, which must change no sign of any row and no discrete outcome
(which rows are zero, which controls differ from their reference and in which
sense, which parity holds).

Decimals carry 40 significant digits; `value` is the nearest binary64, decided
by lower and upper rational bounds of pi that must give the same double.

Modes:
  python3 -I scw_reference.py           check: rebuild every output in memory and
                                         compare byte for byte with the committed
                                         files, SHA256SUMS included
  python3 -I scw_reference.py --write   write the outputs (intentional changes only)
"""
import argparse
import hashlib
import json
import sys
from decimal import Decimal, localcontext
from fractions import Fraction as F
from pathlib import Path

HERE = Path(__file__).resolve().parent
OUT = HERE.parent
FIXED_FILES = ("README.md", "generators/scw_reference.py")


# ----------------------------------------------------------------------------- pi, bounded
def _arccot_scaled(x, scale):
    """floor-ish arccot(x) * scale by the alternating series, integer arithmetic."""
    term = scale // x
    total, n, sign, x2 = term, 1, 1, x * x
    while term:
        term //= x2
        n += 2
        sign = -sign
        total += sign * (term // n)
    return total


_SCALE = 10 ** 120
_PI_SCALED = 4 * (4 * _arccot_scaled(5, _SCALE) - _arccot_scaled(239, _SCALE))  # Machin
PI_LO = F(_PI_SCALED - 10 ** 6, _SCALE)  # the series' truncation error is far below 10**6 units
PI_HI = F(_PI_SCALED + 10 ** 6, _SCALE)
_PI_KNOWN = F(3141592653589793238462643383279502884197, 10 ** 39)
assert PI_LO < _PI_KNOWN + F(1, 10 ** 39) and PI_HI > _PI_KNOWN and PI_HI - PI_LO < F(1, 10 ** 110)


class P:
    """sum of c_e * pi**e, e an integer, c_e rational."""
    __slots__ = ("c",)
    __hash__ = None

    def __init__(self, c=None):
        self.c = {e: F(v) for e, v in sorted((c or {}).items()) if v != 0}

    @staticmethod
    def lift(v):
        return v if isinstance(v, P) else P({0: v})

    def __add__(self, o):
        o = P.lift(o)
        d = dict(self.c)
        for e, v in o.c.items():
            d[e] = d.get(e, 0) + v
        return P(d)

    __radd__ = __add__

    def __neg__(self):
        return P({e: -v for e, v in self.c.items()})

    def __sub__(self, o):
        return self + (-P.lift(o))

    def __rsub__(self, o):
        return P.lift(o) - self

    def __mul__(self, o):
        if isinstance(o, P):
            d = {}
            for e1, v1 in self.c.items():
                for e2, v2 in o.c.items():
                    d[e1 + e2] = d.get(e1 + e2, 0) + v1 * v2
            return P(d)
        return P({e: v * o for e, v in self.c.items()})

    __rmul__ = __mul__

    def __truediv__(self, s):
        return P({e: v / s for e, v in self.c.items()})

    def shift(self, k):
        return P({e + k: v for e, v in self.c.items()})

    def __eq__(self, o):
        return self.c == P.lift(o).c

    def is_zero(self):
        return not self.c

    def bounds(self):
        lo = hi = F(0)
        for e, v in self.c.items():
            a, b = v * PI_LO ** e, v * PI_HI ** e
            lo += min(a, b)
            hi += max(a, b)
        return lo, hi

    def sign(self):
        if not self.c:
            return 0
        lo, hi = self.bounds()
        if lo > 0:
            return 1
        if hi < 0:
            return -1
        raise SystemExit(f"sign of {self.text()} not decided by the pi bounds")

    def abs(self):
        return -self if self.sign() < 0 else self

    def binary64(self):
        lo, hi = self.bounds()
        a, b = float(lo), float(hi)
        if a != b:
            raise SystemExit(f"binary64 rounding of {self.text()} not decided by the pi bounds")
        return a

    def decimal40(self):
        if not self.c:
            return "0"
        lo, hi = self.bounds()
        mid = (lo + hi) / 2
        with localcontext() as ctx:
            ctx.prec = 80
            v = Decimal(mid.numerator) / Decimal(mid.denominator)
            return format(v, ".39e")

    def text(self):
        if not self.c:
            return "0"
        parts = []
        for e in sorted(self.c, reverse=True):
            v = self.c[e]
            body = str(abs(v))
            body = body if e == 0 else f"{body}*pi" if e == 1 else f"{body}/pi" if e == -1 else f"{body}*pi**{e}"
            parts.append(("-" if v < 0 else "+", body))
        s = ("-" if parts[0][0] == "-" else "") + parts[0][1]
        for sg, body in parts[1:]:
            s += f" {sg} {body}"
        return s

    def record(self):
        return {"exact": self.text(), "decimal": self.decimal40(), "value": self.binary64()}


def rec(v):
    return P.lift(v).record()


def cross(a, b):
    return (a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])


def vadd(a, b):
    return tuple(x + y for x, y in zip(a, b))


def vscale(a, s):
    return tuple(x * s for x in a)


ZERO3 = (P(), P(), P())
AXES = {"x": (F(1), F(0), F(0)), "y": (F(0), F(1), F(0)), "z": (F(0), F(0), F(1))}

# ----------------------------------------------------------------------------- inputs (invented, exact)
INPUTS = {
    # geometry along global X: A(0) - B - C - D; the valve body is B-C, mapped by the valve at C
    "L_ab": F(9, 4), "L_bc": F(3, 5), "L_cd": F(7, 4),  # m
    # pipe section, the same on all three members (the body takes the mapped pipe section)
    "D": F(13, 50), "t": F(3, 250), "c_m": F(3, 2000), "c_a": F(1, 400), "t_i": F(3, 40),  # m
    "rho_m": F(7810), "rho_c": F(870), "rho_i": F(130),  # kg/m^3
    "E": F(197 * 10 ** 9), "nu": F(7, 25),  # Pa; G = E / (2 (1 + nu)), authored with that value on PF
    # valve at C mapping B-C: mechanics_geometry_only, weight applied
    "W_v": F(2350), "r_v": (F(-7, 40), F(3, 25), F(1, 8)), "s_v": F(3),  # N; m; stiffness scale
    # flange at B mapping A-B (two-mapped model only): not_solver_consumed, weight not applied
    "W_f": F(640), "r_f": (F(1, 20), F(-1, 10), F(3, 40)),
    # load inputs
    "g_w": F(979, 100),  # m/s^2, the weight case's gravity, acting along -Z
    "g_s": F(979, 100),  # m/s^2, the seismic cases' gravity (equal to g_w: the parity cases)
    "g_s_other": F(10),  # m/s^2, the seismic gravity of the G10 case (differs from g_w)
    "k": (F(3, 10), F(7, 20), F(2, 5)),  # g-factors along +X, +Y, +Z
}
# Boundary gravities (CANT only). g_w_bits is the exact value of the binary64 nearest 9.79, used as the
# weight case's and the seismic case's gravity alike (equal to the last bit); g_s_ulp is the next binary64
# above it. In [8, 16) adjacent binary64 values are 2**-49 apart.
_G_W_BITS = F(float(F(979, 100)))
_ULP = F(1, 2 ** 49)
assert 8 <= _G_W_BITS < 16 and F(float(_G_W_BITS + _ULP)) == _G_W_BITS + _ULP
assert float(_G_W_BITS + _ULP) > float(_G_W_BITS) and F(float(_G_W_BITS + _ULP / 2)) in (_G_W_BITS, _G_W_BITS + _ULP)
INPUTS["g_w_bits"] = _G_W_BITS
INPUTS["W_v_zero"] = F(0)  # N, boundary 4: a valve weight of exactly zero (applied only in the limit cases)
INPUTS["g_s_ulp"] = _G_W_BITS + _ULP
INPUT_UNITS = {"L_ab": "m", "L_bc": "m", "L_cd": "m", "D": "m", "t": "m", "c_m": "m", "c_a": "m", "t_i": "m",
               "rho_m": "kg/m^3", "rho_c": "kg/m^3", "rho_i": "kg/m^3", "E": "Pa", "nu": "1",
               "W_v": "N", "r_v": "m", "s_v": "1", "W_f": "N", "r_f": "m",
               "g_w": "m/s^2", "g_s": "m/s^2", "g_s_other": "m/s^2", "k": "1",
               "g_w_bits": "m/s^2", "g_s_ulp": "m/s^2", "W_v_zero": "N"}

ROUTES = ("PF", "EX")
MODELS = ("CANT", "FIXED")
NODES = ("A", "B", "C", "D")
MEMBERS = (("AB", 0, 1), ("BC", 1, 2), ("CD", 2, 3))
DOF = ("ux", "uy", "uz", "rx", "ry", "rz")
ACT = ("Fx", "Fy", "Fz", "Mx", "My", "Mz")
SUPPORTS = {"CANT": ("A",), "FIXED": ("A", "D")}
MASS_BASIS = {"PF": "source_od_effective_wall_areas/v1", "EX": "source_od_nominal_wall_pressure_bore_areas/v1"}


def binary64_inputs(inp):
    out = {}
    for k, v in inp.items():
        out[k] = tuple(F(float(x)) for x in v) if isinstance(v, tuple) else F(float(v))
    return out


# ----------------------------------------------------------------------------- sections and masses
def section(inp, route):
    """All areas and second moments as coefficients of pi (value = coefficient * pi)."""
    R = inp["D"] / 2
    t, cm = inp["t"], inp["c_m"]
    if route == "EX":
        t_r = (t - cm) - inp["c_a"]
        r_metal, r_bore, r_stiff = R - t, R - t_r, R - t
    else:
        r_metal = r_bore = r_stiff = R - (t - cm)
    metal = inp["rho_m"] * (R * R - r_metal * r_metal)
    contents = inp["rho_c"] * r_bore * r_bore
    insulation = inp["rho_i"] * ((R + inp["t_i"]) ** 2 - R * R)
    a = R * R - r_stiff * r_stiff
    i = (R ** 4 - r_stiff ** 4) / 4
    G = inp["E"] / (2 * (1 + inp["nu"]))
    return {"R": R, "r_metal": r_metal, "r_bore": r_bore, "r_stiff": r_stiff,
            "mu_metal": metal, "mu_contents": contents, "mu_insulation": insulation,
            "mu": metal + contents + insulation, "A": a, "I": i, "J": 2 * i, "G": G}


def geometry(inp):
    x = [F(0)]
    for key in ("L_ab", "L_bc", "L_cd"):
        x.append(x[-1] + inp[key])
    return x


def member_props(inp, route, valve_applied=True):
    """Per member: (EA, GJ, EI) as coefficients of pi, with the valve body's scale where it is applied."""
    sec = section(inp, route)
    props = {}
    for name, _, _ in MEMBERS:
        # a not-consumed component's body solves as its pipe: the flange always, the valve in BND-NONE
        s = inp["s_v"] if (name == "BC" and valve_applied) else F(1)
        props[name] = (s * inp["E"] * sec["A"], s * sec["G"] * sec["J"], s * inp["E"] * sec["I"])
    return props


# ----------------------------------------------------------------------------- load generation
def generate(inp, route, spec):
    """The generated loads of one case: member loads per length (P triples) and nodal wrenches.

    spec keys: kind ('weight' | 'seismic'), g (the gravity the generation uses),
    vec (weight: unit gravity direction; seismic: the g-factor triple),
    valve ('reference' | 'today' | 'double' | 'no_moment' | 'rescaled' | 'not_consumed'),
    flange (None | 'not_consumed' | 'keyed_on_mapped'), mapped_g (gravity used for the
    rescaled control's division), W_v (the valve's entered weight, if not the input W_v)."""
    sec = section(inp, route)
    mu = P({1: sec["mu"]})
    g = spec["g"]
    accel = vscale(spec["vec"], g)  # acceleration triple applied to masses
    unit_force = spec["vec"]  # weight: W * ghat; seismic: W * (kx, ky, kz)
    member_loads, wrenches = {}, []
    valve = spec.get("valve", "reference")
    flange = spec.get("flange")
    for name, _, _ in MEMBERS:
        distributed = True
        if name == "BC" and valve not in ("today", "double", "not_consumed"):
            distributed = False
        if name == "AB" and flange == "keyed_on_mapped":
            distributed = False
        if distributed:
            member_loads[name] = tuple(mu * a for a in accel)
    if valve not in ("today", "not_consumed"):
        w_v = spec.get("W_v", inp["W_v"])
        f_scale = w_v if valve != "rescaled" else w_v / spec["mapped_g"] * g
        force = tuple(P.lift(f_scale * u) for u in unit_force)
        moment = ZERO3 if valve == "no_moment" else cross(inp["r_v"], force)
        wrenches.append(("C", force, tuple(P.lift(m) for m in moment)))
    if flange == "keyed_on_mapped":
        force = tuple(P.lift(inp["W_f"] * u) for u in unit_force)
        wrenches.append(("B", force, tuple(P.lift(m) for m in cross(inp["r_f"], force))))
    return member_loads, wrenches


def resultant(inp, member_loads, wrenches):
    x = geometry(inp)
    idx = {n: i for i, n in enumerate(NODES)}
    force, moment = ZERO3, ZERO3
    for name, i, j in MEMBERS:
        if name in member_loads:
            l = x[j] - x[i]
            f = vscale(member_loads[name], l)
            force = vadd(force, f)
            moment = vadd(moment, cross(((x[i] + x[j]) / 2, F(0), F(0)), f))
    for node, f, m in wrenches:
        force = vadd(force, f)
        moment = vadd(moment, vadd(m, cross((x[idx[node]], F(0), F(0)), f)))
    return force, moment


# ----------------------------------------------------------------------------- method 1: direct stiffness
def elem_k(l, ea, gj, ei):
    k = [[F(0)] * 12 for _ in range(12)]

    def put(dofs, mat):
        for a, da in enumerate(dofs):
            for b, db in enumerate(dofs):
                k[da][db] += mat[a][b]
    put((0, 6), [[ea / l, -ea / l], [-ea / l, ea / l]])
    put((3, 9), [[gj / l, -gj / l], [-gj / l, gj / l]])
    c = ei / l ** 3
    put((1, 5, 7, 11), [[12 * c, 6 * l * c, -12 * c, 6 * l * c],
                        [6 * l * c, 4 * l * l * c, -6 * l * c, 2 * l * l * c],
                        [-12 * c, -6 * l * c, 12 * c, -6 * l * c],
                        [6 * l * c, 2 * l * l * c, -6 * l * c, 4 * l * l * c]])
    put((2, 4, 8, 10), [[12 * c, -6 * l * c, -12 * c, -6 * l * c],
                        [-6 * l * c, 4 * l * l * c, 6 * l * c, 2 * l * l * c],
                        [-12 * c, 6 * l * c, 12 * c, 6 * l * c],
                        [-6 * l * c, 2 * l * l * c, 6 * l * c, 4 * l * l * c]])
    return k


def elem_feq(l, q):
    qx, qy, qz = q
    return [qx * (l / 2), qy * (l / 2), qz * (l / 2), P(), qz * (-l * l / 12), qy * (l * l / 12),
            qx * (l / 2), qy * (l / 2), qz * (l / 2), P(), qz * (l * l / 12), qy * (-l * l / 12)]


def gauss_solve(a, rhs_list):
    """Solve a x = b exactly for each rational b in rhs_list."""
    n = len(a)
    m = [row[:] + [b[i] for b in rhs_list] for i, row in enumerate(a)]
    nr = len(rhs_list)
    for col in range(n):
        piv = next(r for r in range(col, n) if m[r][col] != 0)
        m[col], m[piv] = m[piv], m[col]
        p = m[col][col]
        m[col] = [v / p for v in m[col]]
        for r in range(n):
            if r != col and m[r][col] != 0:
                f = m[r][col]
                m[r] = [vr - f * vc for vr, vc in zip(m[r], m[col])]
    return [[m[i][n + j] for i in range(n)] for j in range(nr)]


def solve_p(a, b):
    """Solve (pi * a) x = b for a rational matrix a and a P vector b."""
    exps = sorted({e for v in b for e in v.c})
    if not exps:
        return [P() for _ in b]
    sols = gauss_solve(a, [[v.c.get(e, F(0)) for v in b] for e in exps])
    x = [P() for _ in b]
    for e, s in zip(exps, sols):
        x = [xi + P({e - 1: si}) for xi, si in zip(x, s)]
    return x


def stiffness_method(inp, route, model, member_loads, wrenches, valve_applied=True):
    x = geometry(inp)
    props = member_props(inp, route, valve_applied)
    n = 6 * len(NODES)
    K = [[F(0)] * n for _ in range(n)]
    f = [P() for _ in range(n)]
    nodal = [P() for _ in range(n)]
    idx = {nd: i for i, nd in enumerate(NODES)}
    elems = {}
    for name, i, j in MEMBERS:
        l = x[j] - x[i]
        ke = elem_k(l, *props[name])
        fe = elem_feq(l, member_loads.get(name, ZERO3))
        dofs = list(range(6 * i, 6 * i + 6)) + list(range(6 * j, 6 * j + 6))
        for a, da in enumerate(dofs):
            f[da] = f[da] + fe[a]
            for b, db in enumerate(dofs):
                K[da][db] += ke[a][b]
        elems[name] = (ke, fe, dofs)
    for node, force, moment in wrenches:
        for c in range(3):
            nodal[6 * idx[node] + c] += force[c]
            nodal[6 * idx[node] + 3 + c] += moment[c]
    f = [a + b for a, b in zip(f, nodal)]
    fixed = {6 * idx[s] + c for s in SUPPORTS[model] for c in range(6)}
    free = [d for d in range(n) if d not in fixed]
    uf = solve_p([[K[r][c] for c in free] for r in free], [f[d] for d in free])
    u = [P() for _ in range(n)]
    for d, v in zip(free, uf):
        u[d] = v
    end = {}
    node_sum = [P() for _ in range(n)]
    for name, (ke, fe, dofs) in elems.items():
        ue = [u[d] for d in dofs]
        fend = []
        for a in range(12):
            acc = P()
            for b in range(12):
                if ke[a][b] != 0:
                    acc = acc + ue[b] * ke[a][b]
            fend.append(acc.shift(1) - fe[a])
        end[name] = fend
        for a, d in enumerate(dofs):
            node_sum[d] = node_sum[d] + fend[a]
    reactions = {s: [node_sum[6 * idx[s] + c] - nodal[6 * idx[s] + c] for c in range(6)] for s in SUPPORTS[model]}
    # free-DOF balance: the element end forces at a free node equal its applied nodal load
    balance = all(node_sum[d] == nodal[d] for d in free)
    disp = {nd: u[6 * i:6 * i + 6] for i, nd in enumerate(NODES)}
    # the assembled load vector (equivalent nodal loads plus nodal wrenches): its force and its moment
    # about the origin, each nodal force acting at its node (x_i, 0, 0)
    feq_force = tuple(sum((f[6 * i + c] for i in range(len(NODES))), P()) for c in range(3))
    feq_moment = ZERO3
    for i in range(len(NODES)):
        node_f = (f[6 * i], f[6 * i + 1], f[6 * i + 2])
        node_m = (f[6 * i + 3], f[6 * i + 4], f[6 * i + 5])
        feq_moment = vadd(feq_moment, vadd(node_m, cross((x[i], F(0), F(0)), node_f)))
    return {"disp": disp, "reactions": reactions, "end": end, "free_balance": balance,
            "feq_force": feq_force, "feq_moment": feq_moment}


# ----------------------------------------------------------------------------- method 2: statics and virtual work
class Poly:
    """polynomial in x with P coefficients: {degree: P}."""

    def __init__(self, c=None):
        self.c = {d: v for d, v in (c or {}).items() if not P.lift(v).is_zero()}
        self.c = {d: P.lift(v) for d, v in self.c.items()}

    def __add__(self, o):
        d = dict(self.c)
        for k, v in o.c.items():
            d[k] = d.get(k, P()) + v
        return Poly(d)

    def __neg__(self):
        return Poly({k: -v for k, v in self.c.items()})

    def __sub__(self, o):
        return self + (-o)

    def mul(self, o):
        d = {}
        for k1, v1 in self.c.items():
            for k2, v2 in o.c.items():
                d[k1 + k2] = d.get(k1 + k2, P()) + v1 * v2
        return Poly(d)

    def at(self, xv):
        return sum((v * xv ** k for k, v in self.c.items()), P())

    def integral(self, a, b):
        return sum((v * ((b ** (k + 1) - a ** (k + 1)) / (k + 1)) for k, v in self.c.items()), P())


def const(v):
    return Poly({0: v})


def lin(a, b):
    """a + b x"""
    return Poly({0: a, 1: b})


def section_actions(inp, member_loads, wrenches, seg):
    """Outboard resultant at a section in member seg, about the section point (x, 0, 0):
    the action of the outboard part on the +x face of the inboard part. Six Poly in x."""
    x = geometry(inp)
    idx = {nd: i for i, nd in enumerate(NODES)}
    name, i, j = MEMBERS[seg]
    xb = x[j]
    Fs = [Poly(), Poly(), Poly()]
    Ms = [Poly(), Poly(), Poly()]

    def add_point(xn, force, moment):
        arm = lin(xn, F(-1))  # x_n - x
        for c in range(3):
            Fs[c] = Fs[c] + const(force[c])
            Ms[c] = Ms[c] + const(moment[c])
        Ms[1] = Ms[1] - arm.mul(const(force[2]))
        Ms[2] = Ms[2] + arm.mul(const(force[1]))
    for node, force, moment in wrenches:
        if x[idx[node]] >= xb:
            add_point(x[idx[node]], force, moment)
    for k, (nm, a, b) in enumerate(MEMBERS):
        q = member_loads.get(nm)
        if q is None or k < seg:
            continue
        if k > seg:
            l = x[b] - x[a]
            add_point((x[a] + x[b]) / 2, vscale(q, l), ZERO3)
        else:
            span = lin(xb, F(-1))  # xb - x
            half_sq = span.mul(span).mul(const(F(1, 2)))
            for c in range(3):
                Fs[c] = Fs[c] + span.mul(const(q[c]))
            Ms[1] = Ms[1] - half_sq.mul(const(q[2]))
            Ms[2] = Ms[2] + half_sq.mul(const(q[1]))
    return Fs + Ms


def cantilever_response(inp, route, member_loads, wrenches, valve_applied=True):
    """Fixed at A, free elsewhere: reaction at A, displacements at every node, end forces."""
    x = geometry(inp)
    props = member_props(inp, route, valve_applied)
    S = [section_actions(inp, member_loads, wrenches, k) for k in range(len(MEMBERS))]
    reaction_a = [-S[0][c].at(x[0]) for c in range(6)]
    end = {}
    for k, (name, i, j) in enumerate(MEMBERS):
        end[name] = [-S[k][c].at(x[i]) for c in range(6)] + [S[k][c].at(x[j]) for c in range(6)]
    disp = {"A": [P()] * 6}
    for p_idx in range(1, len(NODES)):
        xp = x[p_idx]
        u = []
        for comp in range(6):
            total = P()
            for k, (name, i, j) in enumerate(MEMBERS):
                if x[j] > xp:
                    break
                ea, gj, ei = props[name]
                N, Vy, Vz, T, My, Mz = S[k]
                arm = lin(xp, F(-1))
                if comp == 0:
                    term = N.mul(const(1 / ea))
                elif comp == 1:
                    term = Mz.mul(arm).mul(const(1 / ei))
                elif comp == 2:
                    term = (-My).mul(arm).mul(const(1 / ei))
                elif comp == 3:
                    term = T.mul(const(1 / gj))
                elif comp == 4:
                    term = My.mul(const(1 / ei))
                else:
                    term = Mz.mul(const(1 / ei))
                total = total + term.integral(x[i], x[j]).shift(-1)  # stiffnesses are coefficients of pi
            u.append(total)
        disp[NODES[p_idx]] = u
    return {"reaction_a": reaction_a, "end": end, "disp": disp}


def virtual_work_method(inp, route, model, member_loads, wrenches, valve_applied=True):
    if model == "CANT":
        r = cantilever_response(inp, route, member_loads, wrenches, valve_applied)
        return {"disp": r["disp"], "reactions": {"A": r["reaction_a"]}, "end": r["end"]}
    # force method: release D, compatibility u_D = 0 with the six actions at D as redundants
    base = cantilever_response(inp, route, member_loads, wrenches, valve_applied)["disp"]["D"]
    flex = []  # flex[j][i] = displacement j at D due to unit action i at D (coefficients of 1/pi)
    for i in range(6):
        unit_f = tuple(P.lift(F(1) if c == i else F(0)) for c in range(3))
        unit_m = tuple(P.lift(F(1) if c + 3 == i else F(0)) for c in range(3))
        d = cantilever_response(inp, route, {}, [("D", unit_f, unit_m)], valve_applied)["disp"]["D"]
        flex.append([v.c.get(-1, F(0)) for v in d])
        assert all(set(v.c) <= {-1} for v in d)
    fmat = [[flex[i][j] for i in range(6)] for j in range(6)]
    # solve_p solves (pi * fmat) y = base; the flexibility is fmat / pi, so red = -pi**2 * y
    red = [(-v).shift(2) for v in solve_p(fmat, base)]
    red_f, red_m = tuple(red[:3]), tuple(red[3:])
    r = cantilever_response(inp, route, member_loads, list(wrenches) + [("D", red_f, red_m)], valve_applied)
    return {"disp": r["disp"], "reactions": {"A": r["reaction_a"], "D": red}, "end": r["end"]}


# ----------------------------------------------------------------------------- cases
def case_specs(inp):
    k = inp["k"]
    down = (F(0), F(0), F(-1))
    specs = [
        ("WEIGHT-NZ", "reference", "weight case, gravity g_w along -Z",
         {"kind": "weight", "g": inp["g_w"], "vec": down}),
        ("SEISMIC-X", "reference", "seismic, g-factor along X only, gravity g_s = g_w",
         {"kind": "seismic", "g": inp["g_s"], "vec": (k[0], F(0), F(0))}),
        ("SEISMIC-Y", "reference", "seismic, g-factor along Y only, gravity g_s = g_w",
         {"kind": "seismic", "g": inp["g_s"], "vec": (F(0), k[1], F(0))}),
        ("SEISMIC-Z", "reference", "seismic, g-factor along Z only, gravity g_s = g_w",
         {"kind": "seismic", "g": inp["g_s"], "vec": (F(0), F(0), k[2])}),
        ("SEISMIC-XYZ", "reference", "seismic, all three g-factors at once, gravity g_s = g_w",
         {"kind": "seismic", "g": inp["g_s"], "vec": k}),
        ("SEISMIC-XYZ-G10", "reference", "seismic, all three g-factors, the case's own gravity g_s_other != g_w",
         {"kind": "seismic", "g": inp["g_s_other"], "vec": k}),
        ("TWO-WEIGHT-NZ", "reference", "two mapped members: valve applied, flange not consumed; weight, -Z",
         {"kind": "weight", "g": inp["g_w"], "vec": down, "flange": "not_consumed"}),
        ("TWO-SEISMIC-XYZ", "reference", "two mapped members: valve applied, flange not consumed; seismic XYZ",
         {"kind": "seismic", "g": inp["g_s"], "vec": k, "flange": "not_consumed"}),
        ("CTRL-A-TODAY", "control", "control (a): today's rule; B-C loaded with its pipe mass, no component force",
         {"kind": "seismic", "g": inp["g_s"], "vec": k, "valve": "today"}),
        ("CTRL-B-DOUBLE", "control", "control (b): B-C pipe mass load and the component force both, double count",
         {"kind": "seismic", "g": inp["g_s"], "vec": k, "valve": "double"}),
        ("CTRL-C-NO-MOMENT", "control", "control (c): component force at C with the centre-of-gravity moment omitted",
         {"kind": "seismic", "g": inp["g_s"], "vec": k, "valve": "no_moment"}),
        ("CTRL-G-RESCALED", "control", "control (fixed-mass inertia): the G10 case's inputs with the valve force "
         "k (W_v / g_w) g_s_other, the inertia of the mass W_v / g_w the weight case fixes",
         {"kind": "seismic", "g": inp["g_s_other"], "vec": k, "valve": "rescaled", "mapped_g": inp["g_w"],
          "rule": "control: fixed-mass inertia"}),
        ("CTRL-TWO-KEYED-ON-MAPPED", "control", "control: two mapped members, the rule keyed on 'mapped'; "
         "the not-consumed flange also gets k W_f at B and A-B loses its load",
         {"kind": "seismic", "g": inp["g_s"], "vec": k, "flange": "keyed_on_mapped",
          "rule": "control: keyed on mapped"}),
        ("CTRL-K-TIMES-WEIGHT-CASE", "control", "control: the G10 case's inputs (authored gravity g_s_other) "
         "generated at the weight case's gravity g_w instead, i.e. k times the existing weight case",
         {"kind": "seismic", "g": inp["g_w"], "authored_g": inp["g_s_other"], "vec": k,
          "rule": "decided, at the weight case's gravity instead of the authored gravity"}),
        # boundary cases, CANT only
        ("BND-BITS-WEIGHT-NZ", "reference", "boundary 1: weight case at g_w_bits (the binary64 nearest 9.79) "
         "along -Z",
         {"kind": "weight", "g": inp["g_w_bits"], "vec": down, "models": ("CANT",)}),
        ("BND-BITS-SEISMIC-XYZ", "reference", "boundary 1: seismic XYZ at g_w_bits, equal to the weight case's "
         "gravity to the last bit",
         {"kind": "seismic", "g": inp["g_w_bits"], "vec": k, "weight_g": inp["g_w_bits"], "models": ("CANT",)}),
        ("BND-NONE-WEIGHT-NZ", "reference", "boundary 2: valve and flange both declared not consumed, so the "
         "weight case applies no component weight; weight at g_w along -Z",
         {"kind": "weight", "g": inp["g_w"], "vec": down, "valve": "not_consumed", "flange": "not_consumed",
          "valve_applied": False, "models": ("CANT",)}),
        ("BND-NONE-SEISMIC-XYZ-G10", "reference", "boundary 2: no component weight applied; seismic XYZ at "
         "g_s_other != g_w; no component force is carried; the equivalence of the two mass models is not "
         "established for this case, and the values are what the convention yields",
         {"kind": "seismic", "g": inp["g_s_other"], "vec": k, "valve": "not_consumed", "flange": "not_consumed",
          "valve_applied": False, "models": ("CANT",)}),
        ("BND-ULP-SEISMIC-XYZ", "reference", "boundary 3: valve applied, centre of gravity offset on all three "
         "axes; seismic XYZ at g_s_ulp, one binary64 unit above the weight case's g_w_bits",
         {"kind": "seismic", "g": inp["g_s_ulp"], "vec": k, "weight_g": inp["g_w_bits"], "models": ("CANT",)}),
        ("BND-ZERO-WEIGHT-NZ", "limit", "boundary 4, limit (continuity witness): the self-weight plan in the "
         "limit of the valve's applied weight tending to zero; weight at g_w along -Z. No entered document "
         "reaches it: a weight of zero is not applied (decided)",
         {"kind": "weight", "g": inp["g_w"], "vec": down, "W_v": inp["W_v_zero"], "models": ("CANT",),
          "rule": "self-weight plan, in the limit of an applied weight tending to zero"}),
        ("BND-ZERO-SEISMIC-XYZ-G10", "limit", "boundary 4, limit (continuity witness): the decided rule in the "
         "limit of the valve's applied weight tending to zero; seismic XYZ at g_s_other != g_w; B-C loses its "
         "distributed load and the component force and couple are exactly 0. No entered document reaches it: "
         "a weight of zero is not applied (decided)",
         {"kind": "seismic", "g": inp["g_s_other"], "vec": k, "W_v": inp["W_v_zero"], "models": ("CANT",),
          "rule": "decided, in the limit of an applied weight tending to zero"}),
        ("CTRL-ZERO-AS-NOT-APPLIED", "reference", "boundary 4, an entered valve weight of exactly zero: the "
         "weight is not applied (decided), so B-C keeps its distributed seismic load and there is no component "
         "force or couple (the scale is still applied). The reference for that document, and the negative "
         "control against the limit rows",
         {"kind": "seismic", "g": inp["g_s_other"], "vec": k, "W_v": inp["W_v_zero"], "valve": "today",
          "own_weight_valve": "today", "models": ("CANT",),
          "rule": "decided: a weight of zero is not applied"}),
    ]
    return specs


CONTROL_OF = {"CTRL-A-TODAY": "SEISMIC-XYZ", "CTRL-B-DOUBLE": "SEISMIC-XYZ", "CTRL-C-NO-MOMENT": "SEISMIC-XYZ",
              "CTRL-G-RESCALED": "SEISMIC-XYZ-G10", "CTRL-TWO-KEYED-ON-MAPPED": "TWO-SEISMIC-XYZ",
              "CTRL-K-TIMES-WEIGHT-CASE": "SEISMIC-XYZ-G10", "CTRL-ZERO-AS-NOT-APPLIED": "BND-ZERO-SEISMIC-XYZ-G10"}
DEFAULT_RULE = {"weight": "self-weight plan", "seismic": "decided"}
CONTROL_RULE = {"CTRL-A-TODAY": "control (a): today's rule", "CTRL-B-DOUBLE": "control (b): double count",
                "CTRL-C-NO-MOMENT": "control (c): moment omitted"}


def spec_rule(cid, spec):
    return spec.get("rule") or CONTROL_RULE.get(cid) or DEFAULT_RULE[spec["kind"]]


def weight_gravity(inp, spec):
    """The gravity of the applied self-weight a seismic case is compared with."""
    return spec.get("weight_g", inp["g_w"])


def carries_component_force(spec):
    return spec.get("valve", "reference") not in ("today", "not_consumed")


def case_values(inp, route, model, spec, check):
    valve_applied = spec.get("valve_applied", True)
    loads, wrenches = generate(inp, route, spec)
    force, moment = resultant(inp, loads, wrenches)
    m1 = stiffness_method(inp, route, model, loads, wrenches, valve_applied)
    rows = []
    x = geometry(inp)
    for name, _, _ in MEMBERS:
        if name in loads:
            for c, lab in enumerate(("qx", "qy", "qz")):
                rows.append((f"load/{name}/{lab}", loads[name][c]))
    for node, f, m in wrenches:
        for c in range(3):
            rows.append((f"load/{node}/{ACT[c]}", f[c]))
        for c in range(3):
            rows.append((f"load/{node}/{ACT[3 + c]}", m[c]))
    for c in range(3):
        rows.append((f"resultant/{ACT[c]}", force[c]))
    for c in range(3):
        rows.append((f"resultant/{ACT[3 + c]}", moment[c]))
    resp = []
    for nd in NODES:
        for c in range(6):
            resp.append((f"disp/{nd}/{DOF[c]}", m1["disp"][nd][c]))
    for s in SUPPORTS[model]:
        for c in range(6):
            resp.append((f"reaction/{s}/{ACT[c]}", m1["reactions"][s][c]))
    for name, _, _ in MEMBERS:
        for c in range(12):
            resp.append((f"end/{name}/{'ij'[c // 6]}/{ACT[c % 6]}", m1["end"][name][c]))
    rows += resp
    if check is not None:
        m2 = virtual_work_method(inp, route, model, loads, wrenches, valve_applied)
        same = (all(m1["disp"][nd][c] == m2["disp"][nd][c] for nd in NODES for c in range(6))
                and all(m1["reactions"][s][c] == m2["reactions"][s][c] for s in SUPPORTS[model] for c in range(6))
                and all(m1["end"][n][c] == m2["end"][n][c] for n, _, _ in MEMBERS for c in range(12)))
        check("two methods agree on every response row", same)
        check("free-node balance of element end forces", m1["free_balance"])
        check("equivalent nodal loads: force equals the load resultant's force", m1["feq_force"] == force)
        check("equivalent nodal loads: moment about the origin equals the load resultant's moment",
              all((a - b).is_zero() for a, b in zip(m1["feq_moment"], moment)))
        # global equilibrium about the origin: applied resultant + reactions = 0
        idx = {nd: i for i, nd in enumerate(NODES)}
        rf, rm = ZERO3, ZERO3
        for s, r in m1["reactions"].items():
            rf = vadd(rf, tuple(r[:3]))
            rm = vadd(rm, vadd(tuple(r[3:]), cross((x[idx[s]], F(0), F(0)), tuple(r[:3]))))
        check("global equilibrium, forces", all((a + b).is_zero() for a, b in zip(force, rf)))
        check("global equilibrium, moments about the origin", all((a + b).is_zero() for a, b in zip(moment, rm)))
        check("supports do not move", all(v.is_zero() for s in SUPPORTS[model] for v in m1["disp"][s]))
        # each member in equilibrium under its end forces and its own load
        for name, i, j in MEMBERS:
            e = m1["end"][name]
            q = loads.get(name, ZERO3)
            l = x[j] - x[i]
            tf = vadd(vadd(tuple(e[:3]), tuple(e[6:9])), vscale(q, l))
            tm = vadd(vadd(tuple(e[3:6]), tuple(e[9:12])),
                      vadd(cross((l, F(0), F(0)), tuple(e[6:9])), cross((l / 2, F(0), F(0)), vscale(q, l))))
            check(f"member {name} equilibrium", all(v.is_zero() for v in tf + tm))
    return {"rows": rows, "force": force, "moment": moment, "loads": loads, "wrenches": wrenches}


def weight_moment_along(inp, route, axis_vec, g, flange, valve, w_v):
    """Total weight and the moment about the origin of the weight if gravity pointed along axis_vec."""
    spec = {"kind": "weight", "g": g, "vec": axis_vec, "flange": flange, "valve": valve, "W_v": w_v}
    loads, wrenches = generate(inp, route, spec)
    force, moment = resultant(inp, loads, wrenches)
    total = sum((P.lift(c) * a for c, a in zip(force, axis_vec)), P())
    return total, moment


def parity(inp, route, cid, val, spec, g_ref):
    """Seismic resultant against sum over axes of k_a * (weight under gravity g_ref along +e_a).

    The weight is the self-weight plan's for the case's components: a control's altered rule is not
    carried over to the weight it is compared with."""
    flange = spec.get("flange")
    if flange == "keyed_on_mapped":
        flange = "not_consumed"  # the weight case follows self-weight: the not-consumed flange's pipe keeps its weight
    valve = "not_consumed" if spec.get("valve") == "not_consumed" else "reference"
    valve = spec.get("weight_valve", valve)  # an explicit weight-case rule (the zero-weight document's own)
    exp_f, exp_m = ZERO3, ZERO3
    total = None
    for a, axis in enumerate(("x", "y", "z")):
        ka = spec["vec"][a]
        tot, mom = weight_moment_along(inp, route, AXES[axis], g_ref, flange, valve, spec.get("W_v", inp["W_v"]))
        total = tot
        exp_f = vadd(exp_f, vscale(AXES[axis], tot * ka))
        exp_m = vadd(exp_m, vscale(mom, ka))
    df = tuple(a - b for a, b in zip(val["force"], exp_f))
    dm = tuple(a - b for a, b in zip(val["moment"], exp_m))
    holds = all(v.is_zero() for v in df + dm)
    return total, exp_f, exp_m, df, dm, holds


def build_values(inp, check):
    """All cases' exact values; check(name, ok) records cross-checks (None skips the second method)."""
    out = {}
    for model in MODELS:
        for route in ROUTES:
            for cid, role, desc, spec in case_specs(inp):
                if model not in spec.get("models", MODELS):
                    continue
                out[(model, route, cid)] = (role, desc, spec, case_values(inp, route, model, spec, check))
    return out


# ----------------------------------------------------------------------------- the document
CROSS = []


def record_check(name, ok):
    CROSS.append((name, ok))
    if not ok:
        print(f"CROSS-CHECK FAILED: {name}", file=sys.stderr)


def family(row):
    head = row.split("/")[0]
    if head in ("load", "resultant", "reaction", "end"):
        last = row.split("/")[-1]
        if head == "load" and last.startswith("q"):
            return "load_per_length"
        return "force" if last.startswith("F") else "moment"
    return "translation" if row.split("/")[-1].startswith("u") else "rotation"


def difference_summary(ref_rows, ctrl_rows):
    ref = dict(ref_rows)
    ctl = dict(ctrl_rows)
    out = {}
    for fam in ("force", "moment", "translation", "rotation"):
        best = None
        for row, v in ctl.items():
            if row.startswith("load/") or family(row) != fam:
                continue
            d = (v - ref.get(row, P())).abs()
            if best is None or (d - best[1]).sign() > 0:
                best = (row, d)
        out[fam] = {"row": best[0], "abs_difference": rec(best[1])}
    return out


def build_document(values):
    cases = []
    vmap = values
    stats = {"parity_cases": 0, "parity_holds": 0, "row_identity_rows": 0}
    for (model, route, cid), (role, desc, spec, val) in vmap.items():
        entry = {"id": f"SCW-{model}-{route}-{cid}", "model": model, "route": route, "mass_basis": MASS_BASIS[route],
                 "role": role, "description": desc, "load_kind": spec["kind"],
                 "gravity": rec(spec["g"]),
                 "gravity_authored": rec(spec.get("authored_g", spec["g"])),
                 "rule": spec_rule(cid, spec)}
        if spec["kind"] == "seismic":
            entry["g_factors"] = [rec(v) for v in spec["vec"]]
        else:
            entry["gravity_direction"] = [str(v) for v in spec["vec"]]
        if spec["kind"] == "seismic" and role in ("reference", "limit"):
            g_auth, g_wt = spec.get("authored_g", spec["g"]), weight_gravity(INPUTS, spec)
            carries = carries_component_force(spec)
            entry["expected_model_check_warning"] = {
                "expected": carries and g_auth != g_wt,
                "carries_applied_component_force": carries,
                "seismic_gravity_authored": rec(g_auth),
                "applied_self_weight_gravity": rec(g_wt)}
            if role == "limit":
                # no expectation: no entered document can carry this case
                entry["expected_model_check_warning"]["expected"] = None
                entry["expected_model_check_warning"]["applicable"] = False
                entry["expected_model_check_warning"]["reason"] = (
                    "not applicable: no entered document can carry this case, because a component weight of "
                    "zero is not applied (decided by SWBPIPE's Agent 0); carries_applied_component_force "
                    "describes the limit model only")
            elif "own_weight_valve" in spec:
                entry["expected_model_check_warning"]["reason"] = (
                    "no component weight is applied on this document (a weight of zero is not applied), so the "
                    "decided condition is not met even though the gravities differ")
        if cid in CONTROL_OF:
            ref_id = CONTROL_OF[cid]
            ref_val = vmap[(model, route, ref_id)][3]
            entry["compared_with"] = f"SCW-{model}-{route}-{ref_id}"
            entry["resultant_difference"] = {
                ACT[c]: rec((val["force"] + val["moment"])[c] - (ref_val["force"] + ref_val["moment"])[c])
                for c in range(6)}
            entry["largest_response_difference"] = difference_summary(ref_val["rows"], val["rows"])
        if spec["kind"] == "seismic":
            g_ref = spec["g"]
            g_wt = weight_gravity(INPUTS, spec)
            total, ef, em, df, dm, holds = parity(INPUTS, route, cid, val, spec, g_wt)
            par = {"weight_gravity": rec(g_wt),
                   "total_weight_at_weight_gravity": rec(total),
                   "expected_force_from_weight": [rec(v) for v in ef],
                   "expected_moment_from_weight": [rec(v) for v in em],
                   "difference_force": [rec(v) for v in df],
                   "difference_moment": [rec(v) for v in dm],
                   "holds": holds}
            if role == "reference":
                stats["parity_cases"] += 1
                stats["parity_holds"] += int(holds)
            if g_ref != g_wt:
                total2, ef2, em2, df2, dm2, holds2 = parity(INPUTS, route, cid, val, spec, g_ref)
                par["at_the_seismic_case_gravity"] = {
                    "total_weight": rec(total2),
                    "expected_force_from_weight": [rec(v) for v in ef2],
                    "expected_moment_from_weight": [rec(v) for v in em2],
                    "holds": holds2}
                if role in ("reference", "limit") and "own_weight_valve" not in spec:
                    # the gap from the weight at g_wt is the pipes' share alone: the distributed loads
                    # scaled by (g_ref - g_wt) / g_ref; the component force carries no gravity
                    share = {m: tuple(q * ((g_ref - g_wt) / g_ref) for q in qs) for m, qs in val["loads"].items()}
                    sf, sm = resultant(INPUTS, share, [])
                    par["pipes_share_force"] = [rec(v) for v in sf]
                    par["pipes_share_moment"] = [rec(v) for v in sm]
                    record_check("parity gap equals the pipes' share",
                                 all((a - b).is_zero() for a, b in zip(df + dm, sf + sm)))
            if "own_weight_valve" in spec:
                # the block above compares with the limit's weight (zero weight applied), as first frozen; this
                # document's own weight case does not apply the zero weight, so its pipe keeps its weight
                par["weight_compared"] = ("the limit's weight case (zero weight applied, BND-ZERO-WEIGHT-NZ), "
                                          "kept as first frozen; this document's own weight case is own_weight_case")
                own = dict(spec, weight_valve=spec["own_weight_valve"])
                o_tot, o_ef, o_em, o_df, o_dm, o_holds = parity(INPUTS, route, cid, val, own, g_wt)
                s_tot, s_ef, s_em, s_df, s_dm, s_holds = parity(INPUTS, route, cid, val, own, g_ref)
                share = {m: tuple(q * ((g_ref - g_wt) / g_ref) for q in qs) for m, qs in val["loads"].items()}
                sf, sm = resultant(INPUTS, share, [])
                par["own_weight_case"] = {
                    "description": "the weight this document's self-weight plan gives: the zero weight not "
                                   "applied, B-C keeping its distributed weight",
                    "total_weight_at_weight_gravity": rec(o_tot),
                    "difference_force": [rec(v) for v in o_df],
                    "difference_moment": [rec(v) for v in o_dm],
                    "holds": o_holds,
                    "holds_at_the_seismic_case_gravity": s_holds,
                    "pipes_share_force": [rec(v) for v in sf],
                    "pipes_share_moment": [rec(v) for v in sm]}
                record_check("parity gap equals the pipes' share",
                             all((a - b).is_zero() for a, b in zip(o_df + o_dm, sf + sm)))
            entry["parity"] = par
        entry["rows"] = [dict(row=r, **rec(v)) for r, v in val["rows"]]
        cases.append(entry)
    # row-by-row identity: seismic Z response = -k_z * weight(-Z) response when g_s = g_w
    kz = INPUTS["k"][2]
    identity = []
    for model in MODELS:
        for route in ROUTES:
            for s_id, w_id in (("SEISMIC-Z", "WEIGHT-NZ"),):
                srows = [(r, v) for r, v in vmap[(model, route, s_id)][3]["rows"] if not r.startswith("load/")
                         and not r.startswith("resultant/")]
                wrows = dict(vmap[(model, route, w_id)][3]["rows"])
                ok = all(v == wrows[r] * (-kz) for r, v in srows)
                identity.append({"seismic": f"SCW-{model}-{route}-{s_id}", "weight": f"SCW-{model}-{route}-{w_id}",
                                 "relation": "seismic_row = -k_z * weight_row", "rows": len(srows), "holds": ok})
                record_check(f"row identity {model} {route} seismic Z vs weight -Z", ok)
            # the two-mapped model's references equal the one-mapped model's: the not-consumed flange changes nothing
            for a, b in (("TWO-SEISMIC-XYZ", "SEISMIC-XYZ"), ("TWO-WEIGHT-NZ", "WEIGHT-NZ")):
                ra, rb = vmap[(model, route, a)][3]["rows"], vmap[(model, route, b)][3]["rows"]
                record_check(f"{model} {route} {a} equals {b}", [(r, v.c) for r, v in ra] == [(r, v.c) for r, v in rb])
    # the boundary cases' stated outcomes
    by_id = {c["id"]: c for c in cases}
    for route in ROUTES:
        bits = by_id[f"SCW-CANT-{route}-BND-BITS-SEISMIC-XYZ"]
        none = by_id[f"SCW-CANT-{route}-BND-NONE-SEISMIC-XYZ-G10"]
        ulp = by_id[f"SCW-CANT-{route}-BND-ULP-SEISMIC-XYZ"]
        record_check("boundary 1: parity exact, no warning",
                     bits["parity"]["holds"] and not bits["expected_model_check_warning"]["expected"])
        record_check("boundary 2: no component force, no warning, parity at its own gravity",
                     not none["expected_model_check_warning"]["carries_applied_component_force"]
                     and not none["expected_model_check_warning"]["expected"]
                     and none["parity"]["at_the_seismic_case_gravity"]["holds"])
        record_check("boundary 3: warning, parity fails by the pipes' share",
                     ulp["expected_model_check_warning"]["expected"] and not ulp["parity"]["holds"]
                     and "pipes_share_force" in ulp["parity"])
        zkey = ("CANT", route, "BND-ZERO-SEISMIC-XYZ-G10")
        zero = vmap[zkey][3]
        ctrl = vmap[("CANT", route, "CTRL-ZERO-AS-NOT-APPLIED")][3]
        wrench = [v for _, f, m in zero["wrenches"] for v in f + m]
        record_check("boundary 4, limit: B-C loses its load; the component force and couple are exactly zero",
                     "BC" not in zero["loads"] and len(wrench) == 6 and all(v.is_zero() for v in wrench))
        record_check("boundary 4: the zero-weight document keeps the B-C load and differs from the limit",
                     "BC" in ctrl["loads"] and any(not (a - b).is_zero() for a, b in
                                                   zip(ctrl["force"] + ctrl["moment"], zero["force"] + zero["moment"])))
        zdoc = by_id[f"SCW-CANT-{route}-CTRL-ZERO-AS-NOT-APPLIED"]
        zlim = by_id[f"SCW-CANT-{route}-BND-ZERO-SEISMIC-XYZ-G10"]
        record_check("boundary 4: the limit has no warning expectation; the zero-weight document expects none",
                     zlim["expected_model_check_warning"]["expected"] is None
                     and zdoc["expected_model_check_warning"]["expected"] is False
                     and zdoc["parity"]["own_weight_case"]["holds_at_the_seismic_case_gravity"])
        # finding-1 limit: as the applied weight tends to zero the fixed-mass-inertia force equals the convention's
        fm_spec = dict(vmap[zkey][2], valve="rescaled", mapped_g=INPUTS["g_w"])
        fm_loads, fm_wrenches = generate(INPUTS, route, fm_spec)
        record_check("boundary 4, limit: fixed-mass inertia coincides with the convention at zero weight",
                     fm_loads.keys() == zero["loads"].keys()
                     and all(a == b for m in fm_loads for a, b in zip(fm_loads[m], zero["loads"][m]))
                     and all(a == b for (_, f1, m1), (_, f2, m2) in zip(fm_wrenches, zero["wrenches"])
                             for a, b in zip(f1 + m1, f2 + m2)))
    return cases, identity, stats


def derived_record(inp):
    d = {}
    x = geometry(inp)
    d["node_x"] = {nd: rec(v) for nd, v in zip(NODES, x)}
    for route in ROUTES:
        sec = section(inp, route)
        r = {"mass_basis": MASS_BASIS[route]}
        for k in ("r_metal", "r_bore", "r_stiff"):
            r[k] = rec(sec[k])
        for k in ("mu_metal", "mu_contents", "mu_insulation", "mu", "A", "I", "J"):
            r[k] = rec(P({1: sec[k]}))
        r["G"] = rec(sec["G"])
        mu = P({1: sec["mu"]})
        r["pipe_weight_per_length_at_g_w"] = rec(mu * inp["g_w"])
        unmapped = inp["L_ab"] + inp["L_cd"]
        r["total_weight_at_g_w"] = rec(mu * inp["g_w"] * unmapped + inp["W_v"])
        r["total_weight_at_g_s_other"] = rec(mu * inp["g_s_other"] * unmapped + inp["W_v"])
        r["valve_body_pipe_weight_at_g_w"] = rec(mu * inp["g_w"] * inp["L_bc"])
        r["flange_body_pipe_weight_at_g_w"] = rec(mu * inp["g_w"] * inp["L_ab"])
        d[route] = r
    return d


def input_record(inp):
    out = {}
    for k, v in inp.items():
        out[k] = {"exact": [str(c) for c in v] if isinstance(v, tuple) else str(v), "unit": INPUT_UNITS[k]}
    return out


def binary64_robustness(exact_values):
    """Rebuild with every input replaced by its binary64 value; compare signs and discrete outcomes."""
    inp = binary64_inputs(INPUTS)
    if True:
        vals = build_values(inp, None)
        sign_changes = 0
        rows = 0
        for key, (_, _, _, ev) in exact_values.items():
            fv = vals[key][3]
            for (r1, a), (r2, b) in zip(ev["rows"], fv["rows"]):
                assert r1 == r2
                rows += 1
                if a.sign() != b.sign():
                    sign_changes += 1
        outcome_changes = 0
        for (model, route, cid), (role, _, spec, ev) in exact_values.items():
            fv = vals[(model, route, cid)][3]
            if cid in CONTROL_OF:
                ref = CONTROL_OF[cid]
                e_ref, f_ref = exact_values[(model, route, ref)][3], vals[(model, route, ref)][3]
                for c in range(6):
                    de = (ev["force"] + ev["moment"])[c] - (e_ref["force"] + e_ref["moment"])[c]
                    df = (fv["force"] + fv["moment"])[c] - (f_ref["force"] + f_ref["moment"])[c]
                    outcome_changes += int(de.sign() != df.sign())
            if spec["kind"] == "seismic":
                fspec = vals[(model, route, cid)][2]
                pe = parity(INPUTS, route, cid, ev, spec, weight_gravity(INPUTS, spec))[5]
                pf = parity(inp, route, cid, fv, fspec, weight_gravity(inp, fspec))[5]
                outcome_changes += int(pe != pf)
                if role == "reference":  # the expected warning is a discrete outcome too
                    we = carries_component_force(spec) and spec.get("authored_g", spec["g"]) != weight_gravity(INPUTS, spec)
                    wf = carries_component_force(fspec) and fspec.get("authored_g", fspec["g"]) != weight_gravity(inp, fspec)
                    outcome_changes += int(we != wf)
        return rows, sign_changes, outcome_changes


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
        if isinstance(o, list) and o and all(isinstance(v, dict) for v in o):
            items = [f"{pad} {emit(v, level + 1)}" for v in o]
            return "[\n" + ",\n".join(items) + "\n" + pad + "]"
        return inline(o)
    return emit(obj, 0) + "\n"


def sha256sums(files):
    lines = [f"{hashlib.sha256(data).hexdigest()}  {path}" for path, data in sorted(files.items())]
    return ("\n".join(lines) + "\n").encode()


def build():
    CROSS.clear()
    values = build_values(INPUTS, record_check)
    cases, identity, stats = build_document(values)
    rows, sign_changes, outcome_changes = binary64_robustness(values)
    record_check("binary64 inputs change no row sign", sign_changes == 0)
    record_check("binary64 inputs change no discrete outcome", outcome_changes == 0)
    names = {}
    for n, ok in CROSS:
        a = names.setdefault(n, [0, 0])
        a[0] += 1
        a[1] += int(ok)
    doc = {
        "schema": "swbpipe-reference/seismic_component_weight/v1",
        "statement": "Independent references for static-equivalent seismic generated as the self-weight plan "
                     "evaluated at the seismic case's gravity, with each entered component force W held fixed "
                     "(DEL-05-01, decided by SWBPIPE's Agent 0). A force convention, not one fixed mass: it "
                     "coincides with acceleration of the self-weight mass when the seismic case's gravity equals an "
                     "applied self-weight's (a sufficient condition; in the limit of an applied weight tending to "
                     "zero the component adds no difference at any gravity, a limit no entered document reaches, "
                     "since a weight of zero is not applied); where that gravity is absent or ambiguous the "
                     "equivalence "
                     "is not established and the values are what the convention yields. "
                     "Invented inputs; no internal pressure; generated by exact rational arithmetic without "
                     "product code.",
        "conventions": {
            "gravity": "the gravity the case's loads were generated with",
            "gravity_authored": "the gravity the case states; differs from gravity only in CTRL-K-TIMES-WEIGHT-CASE",
            "rule": "the generation rule the case follows: decided, self-weight plan, or a named control",
            "expected_model_check_warning": "on seismic reference cases: a warning is expected when the case "
                                            "carries an applied component force and its authored gravity differs "
                                            "from the applied self-weight's; no warning code is assumed. On a "
                                            "limit case expected is null, applicable is false, with the reason",
            "role": "reference: a document a person can make; control: a wrong rule for comparison; limit: the "
                    "rule evaluated where no entered document reaches (a continuity witness)",
            "axes": "members along global +X; local y = global Y, local z = global Z",
            "disp": "nodal translations ux, uy, uz (m) and right-handed rotations rx, ry, rz (rad), global",
            "reaction": "force and moment the support exerts on the structure at its node, global",
            "end": "force and moment the node exerts on the member at end i and end j, global",
            "load": "generated member loads per length (N/m) and nodal wrenches (N, N m), global",
            "resultant": "sum of the generated loads; moments about the origin",
            "shear_model": "euler_bernoulli on both routes (EX: explicit selection)",
            "pressure": "none: no pressure region, no internal pressure anywhere",
        },
        "inputs": input_record(INPUTS),
        "derived": derived_record(INPUTS),
        "crosscheck": [{"check": n, "count": c[0], "agreeing": c[1]} for n, c in names.items()],
        "binary64_input_check": {"rows_compared": rows, "sign_changes": sign_changes,
                                 "discrete_outcome_changes": outcome_changes},
        "row_identity": identity,
        "cases": cases,
    }
    text = dumps(doc)
    assert json.loads(text) == json.loads(json.dumps(doc))
    files = {"references.json": text.encode()}
    for fixed in FIXED_FILES:
        p = OUT / fixed
        if p.exists():
            files[fixed] = p.read_bytes()
    files["SHA256SUMS"] = sha256sums({k: v for k, v in files.items() if k != "SHA256SUMS"})
    return files, cases, stats


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--write", action="store_true")
    args = parser.parse_args()
    files, cases, stats = build()
    bad = [n for n, ok in CROSS if not ok]
    print(f"{len(cases)} cases, {sum(len(c['rows']) for c in cases)} rows")
    print(f"cross-checks: {len(CROSS)} recorded, {len(bad)} disagreeing")
    print(f"parity: {stats['parity_holds']} of {stats['parity_cases']} seismic reference cases hold at g_w")
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
        print("MISMATCH: " + ", ".join(failures + [f"unexpected {e}" for e in extra] + bad))
        return 1
    print(f"verified {len(files)} files byte for byte (SHA256SUMS included)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
