#!/usr/bin/env python3
"""Independent references: generated static seismic and wind loads interacting with the rest of a load
and reference-state document on the exact pressure route.

Standard-library Python 3 only. This generator imports, reads and runs no product code and reads no file
other than the ones it writes. Every expected value is derived here in exact rational arithmetic from
linear-elastic, small-displacement Euler-Bernoulli frame mechanics and from the governing text: DEL-05-01
(static-equivalent generation, mass basis, self-weight plan), DEL-04-07 REQ-011 (a generated load on a
load and reference-state document), DEL-03-05 (a rigid-family component's weight, centre of gravity and
stiffness scale), DEL-03-08 (mass per length), DEL-05-03 (section bases), DEL-04-04 (the lift-off state
test) and DEL-05-02 (subtraction, SRSS, absolute sum and the fixed-load count), with the seismic
component-weight rule SWBPIPE's Agent 0 decided (recorded in ../seismic_component_weight/README.md).
All inputs are invented.

Three models, each a load and reference-state document in which every case selects the same material
point and carries no thermal strain:

  CEF  a constant-effort frame: anchor A, a pipe, a valve body (weight W at its node, stiffness scale s),
       a pipe along +Y, a riser along -Z to anchor E; a constant-effort support on UZ at B, on the first run.
       Cases SUS (the self-weight plan's weight primitives, listed in the ledger) and EQX, EQY, EQZ
       (the same weight primitives and the case's generated seismic load on one axis). Terms
       dEQx = EQx - SUS, and their SRSS and absolute sum.
  LO   a cantilever with a lift-off support on UZ (contact on a positive reaction) and a constant-effort
       support at its tip. Cases SUS, EQZ-SEATED and EQZ-LIFTED; the lift-off threshold g-factor.
  JR   two runs anchored at both ends of each, joined by a connector that replaces the span between
       them. A generation-only seismic case (three axes) and a generation-only wind case; a wind mark
       on the replaced span is refused.

Method 1: direct stiffness, exact 12x12 Euler-Bernoulli elements rotated by exact direction cosines,
work-equivalent nodal loads of a uniform member load. Method 2: force method on a chain cantilevered
from its root anchor, with the actions of the other support as redundants, section actions from statics
and flexibilities and motions by unit-load integration (axial, torsion and bending), Simpson's rule
(exact: the integrands are at most cubic). Closed forms check selected rows a third way.

Every stiffness carries one factor pi (areas and second moments of an annulus), so all solves are done
with pi factored out; values are sums of rational multiples of pi**k. Decimals are printed to 40
significant digits from a Machin value of pi; binary64 values are the correctly rounded nearest
doubles, decided by rational bounds of pi 1e-90 apart. An SRSS value is the square root of such a sum.

Modes:
  python3 -I -B generated_interactions_reference.py           check: rebuild every output in memory and compare byte for
                                          byte with the committed files; change no file
  python3 -I -B generated_interactions_reference.py --write   write the outputs (intentional changes only)
"""
import argparse
import hashlib
import json
import sys
from decimal import Decimal, getcontext
from fractions import Fraction as F
from math import isqrt
from pathlib import Path

HERE = Path(__file__).resolve().parent
OUT = HERE.parent
FIXED_FILES = ("README.md", "generators/generated_interactions_reference.py")
COMP = ("x", "y", "z")
DOFS = ("ux", "uy", "uz", "rx", "ry", "rz")
ACTS = ("Fx", "Fy", "Fz", "Mx", "My", "Mz")


# ----------------------------------------------------------------------------- pi
def _atan_inv_bounds(x, n):
    """Partial sums of arctan(1/x) after n and n + 1 terms: an alternating series, so they bracket it."""
    s, sign, p = F(0), 1, F(1, x)
    sums = []
    for k in range(n + 1):
        s += sign * p / (2 * k + 1)
        sums.append(s)
        sign, p = -sign, p / (x * x)
    a, b = sums[-2], sums[-1]
    return min(a, b), max(a, b)


_A5 = _atan_inv_bounds(5, 80)
_A239 = _atan_inv_bounds(239, 30)
PI_LO = 16 * _A5[0] - 4 * _A239[1]
PI_HI = 16 * _A5[1] - 4 * _A239[0]
assert F(0) < PI_HI - PI_LO < F(1, 10 ** 90)
getcontext().prec = 110
PI_DEC = Decimal((PI_LO + PI_HI).numerator) / Decimal(2 * (PI_LO + PI_HI).denominator)


class P:
    """A finite sum of rational multiples of pi**k."""
    __slots__ = ("t",)

    def __init__(self, t=None):
        self.t = {k: v for k, v in (t or {}).items() if v}

    @staticmethod
    def of(q, k=0):
        return P({k: F(q)})

    def __add__(self, o):
        o = o if isinstance(o, P) else P.of(o)
        t = dict(self.t)
        for k, v in o.t.items():
            t[k] = t.get(k, 0) + v
        return P(t)

    __radd__ = __add__

    def __sub__(self, o):
        return self + (-(o if isinstance(o, P) else P.of(o)))

    def __rsub__(self, o):
        return P.of(o) - self

    def __neg__(self):
        return P({k: -v for k, v in self.t.items()})

    def __mul__(self, o):
        if isinstance(o, P):
            t = {}
            for a, x in self.t.items():
                for b, y in o.t.items():
                    t[a + b] = t.get(a + b, 0) + x * y
            return P(t)
        return P({k: v * o for k, v in self.t.items()})

    __rmul__ = __mul__

    def shift(self, d):
        return P({k + d: v for k, v in self.t.items()})

    def zero(self):
        return not self.t

    def __eq__(self, o):
        o = o if isinstance(o, P) else P.of(o)
        return self.t == o.t

    def __hash__(self):
        return hash(tuple(sorted(self.t.items())))

    def bounds(self):
        lo = hi = F(0)
        for k, v in self.t.items():
            a, b = v * PI_LO ** k, v * PI_HI ** k
            lo, hi = lo + min(a, b), hi + max(a, b)
        return lo, hi

    def sign(self):
        if self.zero():
            return 0
        lo, hi = self.bounds()
        if lo > 0:
            return 1
        if hi < 0:
            return -1
        raise SystemExit(f"sign of {self.t} is not decided by the pi bounds")

    def dec(self):
        s = Decimal(0)
        for k, v in self.t.items():
            s += Decimal(v.numerator) / Decimal(v.denominator) * PI_DEC ** k
        return s


Z = P()


def absP(x):
    return -x if x.sign() < 0 else x


def binary64(x):
    lo, hi = x.bounds()
    if float(lo) != float(hi):
        raise SystemExit(f"binary64 rounding of {x.t} is not decided by the pi bounds")
    return float(lo) + 0.0


def decimal40(x):
    return "0" if x.zero() else format(x.dec().normalize(), ".39e")


def exact_text(x):
    if x.zero():
        return "0"
    parts = []
    for k in sorted(x.t, reverse=True):
        v = x.t[k]
        s = str(abs(v))
        s = s if k == 0 else f"{s}*pi" if k == 1 else f"{s}/pi" if k == -1 else (
            f"{s}*pi**{k}" if k > 0 else f"{s}/pi**{-k}")
        parts.append(("-" if v < 0 else "+", s))
    text = ("-" if parts[0][0] == "-" else "") + parts[0][1]
    for sg, s in parts[1:]:
        text += f" {sg} {s}"
    return text


def record(x):
    x = x if isinstance(x, P) else P.of(x)
    return {"exact": exact_text(x), "decimal": decimal40(x), "value": binary64(x)}


def sqrt_bounds(q, bits=420):
    """Rational lower and upper bounds of sqrt(q), q >= 0 rational, 2**-bits apart."""
    scale = 4 ** bits
    lo_int = isqrt(q.numerator * scale // q.denominator)
    hi_int = isqrt(-(-q.numerator * scale // q.denominator)) + 1
    return F(lo_int, 2 ** bits), F(hi_int, 2 ** bits)


def sqrt_record(s2):
    """The square root of s2 (a P, nonnegative): exact text, 40 digits and the correctly rounded binary64."""
    if s2.zero():
        return {"exact": "0", "decimal": "0", "value": 0.0}
    lo, hi = s2.bounds()
    assert lo > 0
    a, b = sqrt_bounds(lo)[0], sqrt_bounds(hi)[1]
    if float(a) != float(b):
        raise SystemExit(f"binary64 rounding of sqrt({s2.t}) is not decided by the bounds")
    return {"exact": f"sqrt({exact_text(s2)})", "decimal": format(s2.dec().sqrt().normalize(), ".39e"),
            "value": float(a) + 0.0}


def rn_sqrt(q):
    """The binary64 nearest to sqrt(q), q a nonnegative rational, decided by bounds."""
    if q == 0:
        return 0.0
    a, b = sqrt_bounds(q)
    assert float(a) == float(b)
    return float(a) + 0.0


# ----------------------------------------------------------------------------- vectors
def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def addv(a, b):
    return [x + y for x, y in zip(a, b)]


def subv(a, b):
    return [x - y for x, y in zip(a, b)]


def scalev(a, c):
    return [x * c for x in a]


def dot(a, b):
    s = Z
    for x, y in zip(a, b):
        s = s + x * y
    return s


def zero6():
    return [Z] * 6


def unit(i):
    return [F(1) if k == i else F(0) for k in range(3)]


def axes(ex):
    """Right-handed exact local axes of an axis-aligned member direction ex."""
    ref = [F(0), F(0), F(1)] if ex[2] == 0 else [F(0), F(1), F(0)]
    ey = cross(ref, ex)
    ez = cross(ex, ey)
    return ex, ey, ez


# ----------------------------------------------------------------------------- inputs (invented)
def default_inputs():
    d = F
    return {
        # material point selected by every element state of every case (no thermal strain)
        "E": d(1993, 10) * 10 ** 9, "nu": d(293, 1000), "T_point": d(2937, 10),
        # section S1
        "S1_D": d(2047, 10000), "S1_t": d(109, 10000), "S1_cm": d(13, 10000), "S1_ca": d(21, 10000),
        "S1_tins": d(57, 1000), "S1_rho_metal": d(7857), "S1_rho_contents": d(963),
        "S1_rho_insulation": d(147),
        # section S2
        "S2_D": d(1531, 10000), "S2_t": d(87, 10000), "S2_cm": d(9, 10000), "S2_ca": d(17, 10000),
        "S2_tins": d(43, 1000), "S2_rho_metal": d(7857), "S2_rho_contents": d(1011),
        "S2_rho_insulation": d(139),
        # gravity: the self-weight plan's (weight primitives) and every seismic case's
        "g": d(9797, 1000),
        # CEF geometry, valve, constant-effort support, g-factors
        "CEF_L_AB": d(29, 10), "CEF_L_BC": d(13, 20), "CEF_L_CD": d(23, 10), "CEF_L_DE": d(17, 10),
        "CEF_W_valve": d(1870), "CEF_r_valve": (d(-19, 100), d(7, 100), d(11, 100)), "CEF_scale": d(5, 2),
        "CEF_F_ce": d(2730),
        "CEF_k": (d(37, 100), d(53, 100), d(29, 100)),
        # LO geometry, constant-effort support, g-factors of the two seismic cases
        "LO_L_AB": d(37, 10), "LO_L_BC": d(13, 10), "LO_F_ce": d(1110),
        "LO_k_seated": d(23, 100), "LO_k_lifted": d(71, 100),
        # JR geometry, seismic g-factors, wind
        "JR_L_AM": d(29, 20), "JR_L_MB": d(33, 20), "JR_L_BC": d(4, 5), "JR_L_CN": d(23, 20),
        "JR_L_ND": d(29, 20),
        "JR_k": (d(41, 100), d(27, 100), d(33, 100)),
        "JR_p_wind": d(830), "JR_C_wind": d(77, 100),
    }


UNITS = {"E": "Pa", "nu": "1", "T_point": "K", "g": "m/s^2", "CEF_W_valve": "N", "CEF_r_valve": "m",
         "CEF_scale": "1", "CEF_F_ce": "N", "CEF_k": "1", "LO_F_ce": "N", "LO_k_seated": "1",
         "LO_k_lifted": "1", "JR_k": "1", "JR_p_wind": "Pa", "JR_C_wind": "1"}


def unit_of_input(name):
    if name in UNITS:
        return UNITS[name]
    if "rho" in name:
        return "kg/m^3"
    return "m"


# ----------------------------------------------------------------------------- sections and masses
def section(inp, s):
    D, t, cm, ca, tins = (inp[f"{s}_{k}"] for k in ("D", "t", "cm", "ca", "tins"))
    rm, rc, ri = (inp[f"{s}_rho_{k}"] for k in ("metal", "contents", "insulation"))
    R = D / 2
    rn = R - t                      # nominal wall: stiffness and metal
    tr = (t - cm) - ca              # pressure-design wall
    rp = R - tr                     # pressure bore: contents
    An = R * R - rn * rn            # coefficient of pi
    In = (R ** 4 - rn ** 4) / 4     # coefficient of pi
    mu_metal, mu_contents, mu_ins = rm * An, rc * rp * rp, ri * ((R + tins) ** 2 - R * R)
    # the same masses from diameters (cross-check of the radii)
    dm = rm * (D * D - (D - 2 * t) ** 2) / 4
    dc = rc * (D - 2 * tr) ** 2 / 4
    di = ri * ((D + 2 * tins) ** 2 - D * D) / 4
    return {"R": R, "rn": rn, "tr": tr, "rp": rp, "An": An, "In": In,
            "mu_metal": mu_metal, "mu_contents": mu_contents, "mu_insulation": mu_ins,
            "mu": mu_metal + mu_contents + mu_ins, "projected": D + 2 * tins,
            "diam_check": (dm, dc, di) == (mu_metal, mu_contents, mu_ins),
            "valid": 0 < cm + ca < t and rn > 0 and rp > rn and tr > 0}


# ----------------------------------------------------------------------------- models
def models(inp):
    d = F
    a = inp
    xb = a["CEF_L_AB"]
    xc = xb + a["CEF_L_BC"]
    yd = a["CEF_L_CD"]
    ze = -a["CEF_L_DE"]
    cef = {
        "id": "CEF",
        "nodes": {"node:a": (d(0), d(0), d(0)), "node:b": (xb, d(0), d(0)), "node:c": (xc, d(0), d(0)),
                  "node:d": (xc, yd, d(0)), "node:e": (xc, yd, ze)},
        "members": [
            {"id": "pipe:a-b", "i": "node:a", "j": "node:b", "section": "S1", "scale": d(1)},
            {"id": "pipe:b-c", "i": "node:b", "j": "node:c", "section": "S1", "scale": a["CEF_scale"],
             "component": "component:valve-c"},
            {"id": "pipe:c-d", "i": "node:c", "j": "node:d", "section": "S1", "scale": d(1)},
            {"id": "pipe:d-e", "i": "node:d", "j": "node:e", "section": "S2", "scale": d(1)}],
        "anchors": {"support:a": "node:a", "support:e": "node:e"},
        "restraints": {},
        "constant_effort": {"support:ce-b": ("node:b", 2, a["CEF_F_ce"])},
        "components": {"component:valve-c": {"kind": "valve", "pipe": "pipe:b-c", "node": "node:c",
                                             "W": a["CEF_W_valve"], "r": a["CEF_r_valve"],
                                             "scale": a["CEF_scale"],
                                             "solver_consumption": "mechanics_geometry_only"}},
        "replaced": [],
        "chains": [{"nodes": ["node:a", "node:b", "node:c", "node:d", "node:e"],
                    "redundants": [("node:e", k) for k in range(6)]}],
    }
    xb2 = a["LO_L_AB"]
    xc2 = xb2 + a["LO_L_BC"]
    lo = {
        "id": "LO",
        "nodes": {"node:a": (d(0), d(0), d(0)), "node:b": (xb2, d(0), d(0)), "node:c": (xc2, d(0), d(0))},
        "members": [
            {"id": "pipe:a-b", "i": "node:a", "j": "node:b", "section": "S1", "scale": d(1)},
            {"id": "pipe:b-c", "i": "node:b", "j": "node:c", "section": "S1", "scale": d(1)}],
        "anchors": {"support:a": "node:a"},
        "restraints": {},  # the lift-off support's state decides it
        "lift_off": {"support:lo-b": ("node:b", 2)},
        "constant_effort": {"support:ce-c": ("node:c", 2, a["LO_F_ce"])},
        "components": {},
        "replaced": [],
        "chains": [{"nodes": ["node:a", "node:b", "node:c"], "redundants": []}],
    }
    xm = a["JR_L_AM"]
    xb3 = xm + a["JR_L_MB"]
    xc3 = xb3 + a["JR_L_BC"]
    yn = a["JR_L_CN"]
    yd3 = yn + a["JR_L_ND"]
    jr = {
        "id": "JR",
        "nodes": {"node:a": (d(0), d(0), d(0)), "node:m": (xm, d(0), d(0)), "node:b": (xb3, d(0), d(0)),
                  "node:c": (xc3, d(0), d(0)), "node:n": (xc3, yn, d(0)), "node:d": (xc3, yd3, d(0))},
        "members": [
            {"id": "pipe:a-m", "i": "node:a", "j": "node:m", "section": "S2", "scale": d(1)},
            {"id": "pipe:m-b", "i": "node:m", "j": "node:b", "section": "S2", "scale": d(1)},
            {"id": "pipe:c-n", "i": "node:c", "j": "node:n", "section": "S1", "scale": d(1)},
            {"id": "pipe:n-d", "i": "node:n", "j": "node:d", "section": "S1", "scale": d(1)}],
        "replaced": [{"id": "pipe:b-c", "i": "node:b", "j": "node:c", "section": "S2",
                      "connector": "component:joint-bc"}],
        "anchors": {"support:a": "node:a", "support:b": "node:b", "support:c": "node:c",
                    "support:d": "node:d"},
        "restraints": {},
        "constant_effort": {},
        "components": {},
        "chains": [{"nodes": ["node:a", "node:m", "node:b"], "redundants": [("node:b", k) for k in range(6)]},
                   {"nodes": ["node:c", "node:n", "node:d"], "redundants": [("node:d", k) for k in range(6)]}],
    }
    return {"CEF": cef, "LO": lo, "JR": jr}


def member_geom(model, m):
    xi, xj = model["nodes"][m["i"]], model["nodes"][m["j"]]
    v = [b - a for a, b in zip(xi, xj)]
    L = sum(c * c for c in v)
    # axis-aligned: the length is the one nonzero component's magnitude
    nz = [c for c in v if c != 0]
    assert len(nz) == 1
    Lm = abs(nz[0])
    assert Lm * Lm == L
    return [c / Lm for c in v], Lm


def props(inp, secs, m):
    """Coefficients of pi of EA, GJ and EI of a member (nominal section, stiffness scale)."""
    s = secs[m["section"]]
    E, nu = inp["E"], inp["nu"]
    G = E / (2 * (1 + nu))
    k = m["scale"]
    return {"EA": k * E * s["An"], "GJ": k * G * 2 * s["In"], "EI": k * E * s["In"]}


# ----------------------------------------------------------------------------- loads
def weight_loads(inp, secs, model):
    """The self-weight plan's primitives at gravity g along -Z: what SUS lists in its ledger."""
    g = inp["g"]
    dist, nodal, prims = {}, {}, []
    comp_pipes = {c["pipe"]: cid for cid, c in model["components"].items()}
    for m in model["members"]:
        if m["id"] in comp_pipes:
            continue
        q = P.of(-secs[m["section"]]["mu"] * g, 1)
        dist[m["id"]] = [Z, Z, q]
        prims.append({"source": f"{m['id']}/weight", "kind": "member load per length, whole span",
                      "target": m["id"], "intensity_N_per_m": [record(Z), record(Z), record(q)], "factor": 1})
    for cid, c in model["components"].items():
        Fv = [F(0), F(0), -c["W"]]
        Mv = cross(list(c["r"]), Fv)
        nodal[c["node"]] = [P.of(x) for x in Fv + Mv]
        prims.append({"source": f"{cid}/weight", "kind": "nodal force and moment at the component's node",
                      "target": c["node"], "force_N": [record(x) for x in Fv],
                      "moment_N_m": [record(x) for x in Mv], "factor": 1})
    return dist, nodal, prims


def seismic_loads(inp, secs, model, kvec):
    """Generated seismic loads (the decided rule): q = k g mu on every member, except a member whose
    component weight the self-weight plan applies, which carries k W at the component's node with
    r x F; a connector-replaced span carries none."""
    g = inp["g"]
    dist, nodal, gen = {}, {}, []
    comp_pipes = {c["pipe"]: cid for cid, c in model["components"].items()}
    for m in model["members"]:
        if m["id"] in comp_pipes:
            continue
        mu = secs[m["section"]]["mu"]
        q = [P.of(k * g * mu, 1) for k in kvec]
        dist[m["id"]] = q
        e, L = member_geom(model, m)
        xi = model["nodes"][m["i"]]
        gen.append({"source": f"{m['id']}/seismic", "kind": "generated seismic load per length, whole span",
                    "member": m["id"], "intensity_N_per_m": [record(x) for x in q],
                    "resultant_N": [record(x * L) for x in q],
                    "resultant_point_m": [str(a + c * L / 2) for a, c in zip(xi, e)], "factor": 1})
    for cid, c in model["components"].items():
        Fv = [k * c["W"] for k in kvec]
        Mv = cross(list(c["r"]), Fv)
        nodal[c["node"]] = [P.of(x) for x in Fv + Mv]
        gen.append({"source": f"{cid}/seismic", "kind": "generated seismic force and moment at the component's node",
                    "component": cid, "node": c["node"], "force_N": [record(x) for x in Fv],
                    "moment_about_node_N_m": [record(x) for x in Mv], "factor": 1})
    return dist, nodal, gen


def wind_loads(inp, secs, model, marked, direction):
    q0 = inp["JR_p_wind"] * inp["JR_C_wind"]
    dist, gen = {}, []
    for mid in marked:
        m = next(x for x in model["members"] if x["id"] == mid)
        q = [P.of(q0 * secs[m["section"]]["projected"] if k == direction else 0) for k in range(3)]
        dist[mid] = q
        e, L = member_geom(model, m)
        xi = model["nodes"][m["i"]]
        gen.append({"source": f"{mid}/wind", "kind": "generated wind load per length, whole span",
                    "member": mid, "intensity_N_per_m": [record(x) for x in q],
                    "resultant_N": [record(x * L) for x in q],
                    "resultant_point_m": [str(a + c * L / 2) for a, c in zip(xi, e)], "factor": 1})
    return dist, gen


def merge_loads(*parts):
    dist, nodal = {}, {}
    for dd, nn in parts:
        for k, v in dd.items():
            dist[k] = addv(dist.get(k, [Z, Z, Z]), v)
        for k, v in nn.items():
            nodal[k] = addv(nodal.get(k, zero6()), v)
    return dist, nodal


def scale_loads(loads, c):
    dist, nodal = loads
    return ({k: scalev(v, c) for k, v in dist.items()}, {k: scalev(v, c) for k, v in nodal.items()})


def ce_loads(model):
    nodal = {}
    for sid, (node, dof, f) in model["constant_effort"].items():
        v = zero6()
        v[dof] = P.of(f)
        nodal[node] = addv(nodal.get(node, zero6()), v)
    return {}, nodal


def resultant(model, loads):
    """Force and moment about the origin of member and nodal loads."""
    dist, nodal = loads
    Fr, Mr = [Z, Z, Z], [Z, Z, Z]
    for m in model["members"]:
        if m["id"] in dist:
            e, L = member_geom(model, m)
            w = scalev(dist[m["id"]], L)
            c = [a + b * L / 2 for a, b in zip(model["nodes"][m["i"]], e)]
            Fr = addv(Fr, w)
            Mr = addv(Mr, cross(c, w))
    for n, v in nodal.items():
        x = model["nodes"][n]
        Fr = addv(Fr, v[:3])
        Mr = addv(Mr, addv(cross(list(x), v[:3]), v[3:]))
    return Fr, Mr


# ----------------------------------------------------------------------------- exact linear algebra
def gauss_solve(A, B):
    """Solve A X = B exactly (A: n x n Fractions, B: n x m Fractions) by Gauss-Jordan elimination."""
    n, m = len(A), len(B[0]) if B else 0
    M = [list(A[i]) + list(B[i]) for i in range(n)]
    for c in range(n):
        p = next(r for r in range(c, n) if M[r][c] != 0)
        M[c], M[p] = M[p], M[c]
        piv = M[c][c]
        M[c] = [x / piv for x in M[c]]
        for r in range(n):
            if r != c and M[r][c] != 0:
                f = M[r][c]
                M[r] = [x - f * y for x, y in zip(M[r], M[c])]
    return [row[n:] for row in M]


def solve_P(Kq, b):
    """Solve (pi^s Kq) x = b for x (a list of P), Kq rational; s is folded in by the caller."""
    powers = sorted({k for v in b for k in v.t})
    if not powers:
        return [Z] * len(b)
    cols = [[v.t.get(k, F(0)) for k in powers] for v in b]
    X = gauss_solve(Kq, cols)
    return [P({k: X[i][j] for j, k in enumerate(powers)}) for i in range(len(b))]


# ----------------------------------------------------------------------------- method 1: direct stiffness
def k_local(EA, GJ, EI, L):
    k = [[F(0)] * 12 for _ in range(12)]

    def put(i, j, v):
        k[i][j] += v
        if i != j:
            k[j][i] += v
    a, t = EA / L, GJ / L
    b12, b6, b4, b2 = 12 * EI / L ** 3, 6 * EI / L ** 2, 4 * EI / L, 2 * EI / L
    put(0, 0, a), put(6, 6, a), put(0, 6, -a)
    put(3, 3, t), put(9, 9, t), put(3, 9, -t)
    # v, theta_z
    put(1, 1, b12), put(7, 7, b12), put(1, 7, -b12)
    put(1, 5, b6), put(1, 11, b6), put(5, 7, -b6), put(7, 11, -b6)
    put(5, 5, b4), put(11, 11, b4), put(5, 11, b2)
    # w, theta_y
    put(2, 2, b12), put(8, 8, b12), put(2, 8, -b12)
    put(2, 4, -b6), put(2, 10, -b6), put(4, 8, b6), put(8, 10, b6)
    put(4, 4, b4), put(10, 10, b4), put(4, 10, b2)
    return k


def element_global(model, inp, secs, m):
    e, L = member_geom(model, m)
    ex, ey, ez = axes(e)
    Rm = [ex, ey, ez]
    T = [[F(0)] * 12 for _ in range(12)]
    for blk in range(4):
        for r in range(3):
            for c in range(3):
                T[3 * blk + r][3 * blk + c] = Rm[r][c]
    pr = props(inp, secs, m)
    kl = k_local(pr["EA"], pr["GJ"], pr["EI"], L)
    kt = [[sum(kl[i][k] * T[k][j] for k in range(12) if T[k][j] != 0) for j in range(12)] for i in range(12)]
    return [[sum(T[k][i] * kt[k][j] for k in range(12) if T[k][i] != 0) for j in range(12)] for i in range(12)]


def equivalent_nodal(model, m, w):
    e, L = member_geom(model, m)
    f = scalev(w, L / 2)
    mi = scalev(cross(e, w), L * L / 12)
    return f + mi + f + [-x for x in mi]


def stiffness_solve(model, inp, secs, loads, restrained):
    """restrained: {node: set of dofs} held at zero. Returns motions, member end actions, support actions."""
    dist, nodal = loads
    names = list(model["nodes"])
    idx = {n: 6 * i for i, n in enumerate(names)}
    N = 6 * len(names)
    K = [[F(0)] * N for _ in range(N)]
    f = [Z] * N
    kg = {}
    for m in model["members"]:
        k = element_global(model, inp, secs, m)
        kg[m["id"]] = k
        dd = [idx[m["i"]] + c for c in range(6)] + [idx[m["j"]] + c for c in range(6)]
        for a in range(12):
            for b in range(12):
                if k[a][b]:
                    K[dd[a]][dd[b]] += k[a][b]
        if m["id"] in dist:
            fe = equivalent_nodal(model, m, dist[m["id"]])
            for a in range(12):
                f[dd[a]] = f[dd[a]] + fe[a]
    for n, v in nodal.items():
        for c in range(6):
            f[idx[n] + c] = f[idx[n] + c] + v[c]
    held = {idx[n] + c for n, cs in restrained.items() for c in cs}
    free = [i for i in range(N) if i not in held]
    Kff = [[K[i][j] for j in free] for i in free]
    uf = [x.shift(-1) for x in solve_P(Kff, [f[i] for i in free])]
    u = [Z] * N
    for i, x in zip(free, uf):
        u[i] = x
    motions = {n: u[idx[n]:idx[n] + 6] for n in names}
    ends = {}
    for m in model["members"]:
        k = kg[m["id"]]
        dd = [idx[m["i"]] + c for c in range(6)] + [idx[m["j"]] + c for c in range(6)]
        de = [u[x] for x in dd]
        fe = equivalent_nodal(model, m, dist[m["id"]]) if m["id"] in dist else [Z] * 12
        act = []
        for a in range(12):
            s = Z
            for b in range(12):
                if k[a][b] and not de[b].zero():
                    s = s + de[b] * k[a][b]
            act.append(s.shift(1) - fe[a])
        ends[m["id"]] = {"i": act[:6], "j": act[6:]}
    # node actions: sum of member end actions minus applied nodal load (= what the supports exert)
    node_act = {n: zero6() for n in names}
    for m in model["members"]:
        node_act[m["i"]] = addv(node_act[m["i"]], ends[m["id"]]["i"])
        node_act[m["j"]] = addv(node_act[m["j"]], ends[m["id"]]["j"])
    for n, v in nodal.items():
        node_act[n] = subv(node_act[n], v)
    return {"motions": motions, "ends": ends, "node_actions": node_act}


# ----------------------------------------------------------------------------- method 2: force method
class Chain:
    def __init__(self, model, inp, secs, chain):
        self.model = model
        self.nodes = chain["nodes"]
        self.x = [model["nodes"][n] for n in self.nodes]
        self.mem = []
        for a, b in zip(self.nodes, self.nodes[1:]):
            m = next(x for x in model["members"] if x["i"] == a and x["j"] == b)
            e, L = member_geom(model, m)
            self.mem.append((m, e, L, props(inp, secs, m)))
        self.redundants = chain["redundants"]

    def section(self, j, s, dist, nodal, extra):
        """Resultant (force, moment about the cut point) of everything outboard of the cut at s on member j.
        extra: {node: 6-vector} of further generalized forces (redundants or a unit load)."""
        m, e, L, _ = self.mem[j]
        p = [a + c * s for a, c in zip(self.x[j], e)]
        Fs, Ms = [Z, Z, Z], [Z, Z, Z]

        def add(force, moment, at):
            nonlocal Fs, Ms
            Fs = addv(Fs, force)
            Ms = addv(Ms, addv(cross(subv(at, p), force), moment))
        if m["id"] in dist:
            rem = L - s
            add(scalev(dist[m["id"]], rem), [Z, Z, Z], [a + c * (s + rem / 2) for a, c in zip(self.x[j], e)])
        for q in range(j + 1, len(self.mem)):
            mq, eq, Lq, _ = self.mem[q]
            if mq["id"] in dist:
                add(scalev(dist[mq["id"]], Lq), [Z, Z, Z], [a + c * Lq / 2 for a, c in zip(self.x[q], eq)])
        for q in range(j + 1, len(self.nodes)):
            n = self.nodes[q]
            for src in (nodal, extra):
                if n in src:
                    v = [x if isinstance(x, P) else P.of(x) for x in src[n]]
                    add(v[:3], v[3:], list(self.x[q]))
        return Fs, Ms

    def integrate(self, real, virtual_node, virtual_dof):
        """Unit-load integral: the motion at (virtual_node, virtual_dof) of the released structure under the
        section actions real(j, s)."""
        q = self.nodes.index(virtual_node)
        uvec = [F(0)] * 6
        uvec[virtual_dof] = F(1)
        total = Z
        for j in range(q):
            m, e, L, pr = self.mem[j]

            def f(s):
                Fr, Mr = real(j, s)
                Fu, Mu = self.section(j, s, {}, {}, {virtual_node: uvec})
                N, n = dot(Fr, e), dot(Fu, e)
                T, t = dot(Mr, e), dot(Mu, e)
                Mp = subv(Mr, scalev(e, T))
                mp = subv(Mu, scalev(e, t))
                return (N * n).shift(-1) * (1 / pr["EA"]) + (T * t).shift(-1) * (1 / pr["GJ"]) + \
                    dot(Mp, mp).shift(-1) * (1 / pr["EI"])
            total = total + (f(F(0)) + 4 * f(L / 2) + f(L)) * (L / 6)
        return total

    def solve(self, dist, nodal):
        R = self.redundants

        def unit_extra(node, dof):
            v = [F(0)] * 6
            v[dof] = F(1)
            return {node: v}
        delta0 = [self.integrate(lambda j, s: self.section(j, s, dist, nodal, {}), n, d) for n, d in R]
        flex = [[self.integrate(lambda j, s, n2=n2, d2=d2: self.section(j, s, {}, {}, unit_extra(n2, d2)), n, d)
                 for (n2, d2) in R] for (n, d) in R]
        if R:
            for row in flex:
                for v in row:
                    assert set(v.t) <= {-1}
            Fq = [[v.t.get(-1, F(0)) for v in row] for row in flex]
            X = [x.shift(1) for x in solve_P(Fq, [-v for v in delta0])]
        else:
            X = []
        red = {}
        for (n, d), x in zip(R, X):
            v = red.setdefault(n, zero6())
            v[d] = x

        def final(j, s):
            return self.section(j, s, dist, nodal, red)
        motions = {}
        for n in self.nodes:
            motions[n] = [Z] * 6 if n == self.nodes[0] else [self.integrate(final, n, d) for d in range(6)]
        ends = {}
        for j, (m, e, L, _) in enumerate(self.mem):
            Fi, Mi = final(j, F(0))
            Fj, Mj = final(j, L)
            ends[m["id"]] = {"i": [-x for x in Fi + Mi], "j": Fj + Mj}
        F0, M0 = final(0, F(0))
        support = {self.nodes[0]: [-x for x in F0 + M0]}
        for n, v in red.items():
            support[n] = v
        return {"motions": motions, "ends": ends, "support": support, "redundants": red}


def force_method(model, inp, secs, loads, chains):
    dist, nodal = loads
    out = {"motions": {}, "ends": {}, "support": {}}
    for ch in chains:
        r = Chain(model, inp, secs, ch).solve(dist, nodal)
        for k in out:
            out[k].update(r[k])
    return out


# ----------------------------------------------------------------------------- rows
def solve_both(model, inp, secs, loads, restrained, chains):
    s1 = stiffness_solve(model, inp, secs, loads, restrained)
    s2 = force_method(model, inp, secs, loads, chains)
    return s1, s2


def rows_of(model, s1, ce_rows=True, lift=None):
    """Published rows (exact P values) of one solved state from method 1, in a fixed order."""
    rows = []
    for n in model["nodes"]:
        for c, name in enumerate(DOFS):
            rows.append((f"motion/{n}/{name}", "translation" if c < 3 else "rotation", s1["motions"][n][c]))
    for sid, n in model["anchors"].items():
        for c, name in enumerate(ACTS):
            rows.append((f"reaction/{sid}/{name}", "force" if c < 3 else "moment", s1["node_actions"][n][c]))
    for sid, (n, dof) in model.get("lift_off", {}).items():
        rows.append((f"nonlinear_support/{sid}/reaction_Fz", "force", lift["reaction"]))
        rows.append((f"nonlinear_support/{sid}/relative_uz", "translation", s1["motions"][n][dof]))
    if ce_rows:
        for sid, (n, dof, f) in model["constant_effort"].items():
            rows.append((f"constant_effort/{sid}/F{COMP[dof]}", "force", P.of(f)))
    for m in model["members"]:
        for end in ("i", "j"):
            for c, name in enumerate(ACTS):
                rows.append((f"member_end/{m['id']}/{end}/{name}", "force" if c < 3 else "moment",
                             s1["ends"][m["id"]][end][c]))
    return rows


UNIT = {"translation": "m", "rotation": "rad", "force": "N", "moment": "N*m"}


def publish(rows):
    return [{"id": i, "family": fam, "unit": UNIT[fam], "expected": record(v)} for i, fam, v in rows]


def diff_rows(a, b):
    assert [r[0] for r in a] == [r[0] for r in b]
    return [(i, fam, x - y) for (i, fam, x), (_, _, y) in zip(a, b)]


def methods_agree(model, s1, s2, restrained):
    """Every motion, member end action and support action of method 1 equals method 2's."""
    ok = all(s1["motions"][n] == s2["motions"][n] for n in s2["motions"])
    ok = ok and all(s1["ends"][m] == s2["ends"][m] for m in s2["ends"])
    for n, v in s2["support"].items():
        held = restrained.get(n, set(range(6)))
        ok = ok and all(s1["node_actions"][n][c] == v[c] for c in held)
    return ok


def equilibrium(model, s1, loads, restrained):
    """Global equilibrium (force, moment about the origin) and free-node balance of a method-1 solve."""
    Fr, Mr = resultant(model, loads)
    for n, held in restrained.items():
        v = s1["node_actions"][n]
        x = list(model["nodes"][n])
        f = [v[c] if c in held else Z for c in range(3)]
        mo = [v[c + 3] if c + 3 in held else Z for c in range(3)]
        Fr = addv(Fr, f)
        Mr = addv(Mr, addv(cross(x, f), mo))
    glob = all(x.zero() for x in Fr + Mr)
    free = all(v[c].zero() for n, v in s1["node_actions"].items() for c in range(6)
               if c not in restrained.get(n, set()))
    return glob and free


def member_equilibrium(model, s1, loads):
    dist, _ = loads
    ok = True
    for m in model["members"]:
        e, L = member_geom(model, m)
        xi, xj = list(model["nodes"][m["i"]]), list(model["nodes"][m["j"]])
        ai, aj = s1["ends"][m["id"]]["i"], s1["ends"][m["id"]]["j"]
        w = scalev(dist.get(m["id"], [Z, Z, Z]), L)
        c = [a + b * L / 2 for a, b in zip(xi, e)]
        Fs = addv(addv(ai[:3], aj[:3]), w)
        Ms = addv(addv(addv(cross(xi, ai[:3]), ai[3:]), addv(cross(xj, aj[:3]), aj[3:])), cross(c, w))
        ok = ok and all(x.zero() for x in Fs + Ms)
    return ok


# ----------------------------------------------------------------------------- the build
def build(inp):
    checks = []

    def check(name, ok):
        checks.append({"check": name, "agrees": bool(ok)})

    secs = {s: section(inp, s) for s in ("S1", "S2")}
    for s, v in secs.items():
        check(f"{s}: mass per length from radii equals mass per length from diameters", v["diam_check"])
        check(f"{s}: section validity (0 < c_m + c_a < t; positive nominal and pressure-design walls)", v["valid"])
    mods = models(inp)
    out = {"sections": secs, "models": mods}

    # ------------------------------------------------------------- CEF
    cef = mods["CEF"]
    anchors = {n: set(range(6)) for n in cef["anchors"].values()}
    wd, wn, wprims = weight_loads(inp, secs, cef)
    W = (wd, wn)
    CE = ce_loads(cef)
    cef_cases, cef_terms, cef_seis = {}, {}, {}
    s_sus = solve_both(cef, inp, secs, merge_loads(W, CE), anchors, cef["chains"])
    check("CEF SUS: method 1 equals method 2 on every motion, end action and reaction",
          methods_agree(cef, *s_sus, anchors))
    check("CEF SUS: global equilibrium and free-node balance", equilibrium(cef, s_sus[0], merge_loads(W, CE), anchors))
    check("CEF SUS: member equilibrium", member_equilibrium(cef, s_sus[0], merge_loads(W, CE)))
    cef_cases["SUS"] = {"rows": rows_of(cef, s_sus[0]), "generated": [], "loads": merge_loads(W, CE)}
    for a, ax in enumerate(COMP):
        kvec = [inp["CEF_k"][a] if c == a else F(0) for c in range(3)]
        sd, sn, sgen = seismic_loads(inp, secs, cef, kvec)
        S = (sd, sn)
        L_case = merge_loads(W, CE, S)
        s_eq = solve_both(cef, inp, secs, L_case, anchors, cef["chains"])
        name = f"EQ{ax.upper()}"
        check(f"CEF {name}: method 1 equals method 2", methods_agree(cef, *s_eq, anchors))
        check(f"CEF {name}: global equilibrium and free-node balance", equilibrium(cef, s_eq[0], L_case, anchors))
        check(f"CEF {name}: member equilibrium", member_equilibrium(cef, s_eq[0], L_case))
        cef_cases[name] = {"rows": rows_of(cef, s_eq[0]), "generated": sgen, "loads": L_case}
        # the term, as the difference of the two solved cases
        term = diff_rows(cef_cases[name]["rows"], cef_cases["SUS"]["rows"])
        # the seismic-only response on the same linear structure, solved directly (no constant effort)
        s_only = solve_both(cef, inp, secs, S, anchors, cef["chains"])
        check(f"CEF d{name}: seismic-only solve, method 1 equals method 2", methods_agree(cef, *s_only, anchors))
        only_rows = rows_of(cef, s_only[0], ce_rows=False)
        term_wo_ce = [r for r in term if not r[0].startswith("constant_effort/")]
        check(f"CEF d{name}: the term (case minus SUS) equals the seismic-only response on every row",
              [r[2] for r in term_wo_ce] == [r[2] for r in only_rows])
        check(f"CEF d{name}: the constant-effort force cancels (term row zero; present at F in both cases)",
              all(r[2].zero() for r in term if r[0].startswith("constant_effort/")) and
              all(r[2] == P.of(inp["CEF_F_ce"]) for r in cef_cases[name]["rows"] if r[0].startswith("constant_effort/")))
        # reaction resultant of the term against the generated resultant (EQ-6 form)
        Fg, Mg = resultant(cef, S)
        Rf, Rm = [Z, Z, Z], [Z, Z, Z]
        rr = dict((r[0], r[2]) for r in term)
        for sid, n in cef["anchors"].items():
            f = [rr[f"reaction/{sid}/{c}"] for c in ACTS[:3]]
            mo = [rr[f"reaction/{sid}/{c}"] for c in ACTS[3:]]
            Rf = addv(Rf, f)
            Rm = addv(Rm, addv(cross(list(cef["nodes"][n]), f), mo))
        # closed form of the generated resultant: k g mu L per distributed member plus k W, moments about O
        g = inp["g"]
        cf_F, cf_M = [Z, Z, Z], [Z, Z, Z]
        for m in cef["members"]:
            if m.get("component"):
                continue
            e, L = member_geom(cef, m)
            f = [P.of(x * g * L * secs[m["section"]]["mu"], 1) for x in kvec]
            c = [a2 + b2 * L / 2 for a2, b2 in zip(cef["nodes"][m["i"]], e)]
            cf_F, cf_M = addv(cf_F, f), addv(cf_M, cross(c, f))
        comp = cef["components"]["component:valve-c"]
        fW = [P.of(k * comp["W"]) for k in kvec]
        xc = list(cef["nodes"][comp["node"]])
        cf_F = addv(cf_F, fW)
        cf_M = addv(cf_M, addv(cross(xc, fW), cross(list(comp["r"]), fW)))
        check(f"CEF d{name}: generated resultant equals k g sum(mu L) e + k W e, moments about the origin "
              f"(closed form)", cf_F == Fg and cf_M == Mg)
        check(f"CEF d{name}: the anchors' reaction resultant equals minus the generated resultant",
              all((x + y).zero() for x, y in zip(Rf + Rm, Fg + Mg)))
        cef_terms[f"d{name}"] = {"rows": term, "Fg": Fg, "Mg": Mg, "Rf": Rf, "Rm": Rm, "kvec": kvec}
        cef_seis[name] = (S, sgen)
    # control: the constant-effort force omitted from the seismic cases only
    ctrl = {}
    ce_only = solve_both(cef, inp, secs, CE, anchors, cef["chains"])
    ce_rows = rows_of(cef, ce_only[0], ce_rows=False)
    out["cef"] = {"cases": cef_cases, "terms": cef_terms, "weight_prims": wprims, "ce_response": ce_rows}
    # SRSS and absolute sum on every signed row of the three terms
    names = [r[0] for r in cef_terms["dEQX"]["rows"]]
    srss, absum = [], []
    for i, rid in enumerate(names):
        vals = [cef_terms[t]["rows"][i][2] for t in ("dEQX", "dEQY", "dEQZ")]
        fam = cef_terms["dEQX"]["rows"][i][1]
        s2 = Z
        for v in vals:
            s2 = s2 + v * v
        ab = Z
        for v in vals:
            ab = ab + absP(v)
        rounded = [F(binary64(v)) for v in vals]
        srss_rn = rn_sqrt(sum(x * x for x in rounded))
        abs_rn = float(sum(abs(x) for x in rounded)) + 0.0
        srss.append({"id": rid, "family": fam, "unit": UNIT[fam], "expected": sqrt_record(s2),
                     "binary64_from_rounded_operands": srss_rn, "_s2": s2, "_abs": ab})
        absum.append({"id": rid, "family": fam, "unit": UNIT[fam], "expected": record(ab),
                      "binary64_from_rounded_operands": abs_rn})
    # EQ-4 checks: SRSS <= ABS; SRSS^2 equals the exact sum of squares; ABS >= each |term|
    ok = True
    for s, a in zip(srss, absum):
        s2, ab = s["_s2"], s["_abs"]
        ok = ok and (ab * ab - s2).sign() >= 0 and ab.sign() >= 0
    check("CEF: SRSS <= absolute sum on every signed row (exact)", ok)
    check("CEF: SRSS and absolute sum are zero exactly where all three terms are zero",
          all((s["_s2"].zero()) == (s["_abs"].zero()) for s in srss))
    out["cef"]["srss"], out["cef"]["abs"] = srss, absum
    # control row differences
    ctrl["CE-OMITTED-FROM-SEISMIC"] = [(i, fam, -v) for i, fam, v in ce_rows]
    out["cef"]["controls"] = ctrl

    # ------------------------------------------------------------- LO
    lo = mods["LO"]
    a_lo = {"node:a": set(range(6))}
    seated = {"node:a": set(range(6)), "node:b": {2}}
    wdl, wnl, wprims_lo = weight_loads(inp, secs, lo)
    WL = (wdl, wnl)
    CEL = ce_loads(lo)
    chains_seated = [{"nodes": lo["chains"][0]["nodes"], "redundants": [("node:b", 2)]}]
    chains_lifted = lo["chains"]

    def lo_state(loads, state, label):
        restrained = seated if state == "seated" else a_lo
        chains = chains_seated if state == "seated" else chains_lifted
        s1, s2 = solve_both(lo, inp, secs, loads, restrained, chains)
        check(f"LO {label}, {state}: method 1 equals method 2", methods_agree(lo, s1, s2, restrained))
        check(f"LO {label}, {state}: global equilibrium and free-node balance",
              equilibrium(lo, s1, loads, restrained))
        check(f"LO {label}, {state}: member equilibrium", member_equilibrium(lo, s1, loads))
        R = s1["node_actions"]["node:b"][2] if state == "seated" else Z
        return s1, R

    lo_cases = {}
    seis_unit = seismic_loads(inp, secs, lo, [F(0), F(0), F(1)])
    for name, k in (("SUS", F(0)), ("EQZ-SEATED", inp["LO_k_seated"]), ("EQZ-LIFTED", inp["LO_k_lifted"])):
        sd, sn, sgen = seismic_loads(inp, secs, lo, [F(0), F(0), k])
        L_case = merge_loads(WL, CEL, (sd, sn)) if k else merge_loads(WL, CEL)
        st = {}
        for state in ("seated", "lifted"):
            s1, R = lo_state(L_case, state, name)
            st[state] = {"s1": s1, "R": R, "u": s1["motions"]["node:b"][2]}
        seated_ok = st["seated"]["R"].sign() > 0
        lifted_ok = st["lifted"]["u"].sign() > 0
        check(f"LO {name}: exactly one state qualifies strictly", seated_ok != lifted_ok)
        state = "seated" if seated_ok else "lifted"
        lo_cases[name] = {"k": k, "state": state, "states": st, "generated": sgen if k else [],
                          "loads": L_case,
                          "rows": rows_of(lo, st[state]["s1"], lift={"reaction": st[state]["R"]})}
    check("LO: SUS and EQZ-SEATED are seated, EQZ-LIFTED is lifted",
          [lo_cases[n]["state"] for n in ("SUS", "EQZ-SEATED", "EQZ-LIFTED")] == ["seated", "seated", "lifted"])
    # threshold: the seated reaction is R0 + k R1
    R0 = lo_cases["SUS"]["states"]["seated"]["R"]
    R1 = lo_state((seis_unit[0], seis_unit[1]), "seated", "the generated seismic Z load per unit g-factor")[1]
    assert set(R1.t) == {1}
    kstar = P({k - 1: -v / R1.t[1] for k, v in R0.t.items()})
    check("LO: threshold lies strictly between the seated and lifted g-factors",
          (kstar - inp["LO_k_seated"]).sign() > 0 and (P.of(inp["LO_k_lifted"]) - kstar).sign() > 0)
    # at the threshold both states are weak: seated reaction zero and lifted separation zero
    # (k* carries 1/pi, so the loads at k* are solved with k* as a P factor)
    sd_u, sn_u, _ = seis_unit
    at_star = merge_loads(WL, CEL, ({m: [x * kstar for x in v] for m, v in sd_u.items()}, {}))
    Rs = lo_state(at_star, "seated", "at the threshold")[1]
    s1l = lo_state(at_star, "lifted", "at the threshold")[0]
    check("LO: at the threshold the seated reaction is zero and the lifted separation is zero (both weak)",
          Rs.zero() and s1l["motions"]["node:b"][2].zero())
    # closed forms (third way): lifted cantilever with UDL w and tip force P; seated propped cantilever
    sec = secs["S1"]
    EI = props(inp, secs, lo["members"][0])["EI"]
    a_len, L = inp["LO_L_AB"], inp["LO_L_AB"] + inp["LO_L_BC"]
    Pce = inp["LO_F_ce"]

    def cant(w, x):
        uz = (w * (x * x * (6 * L * L - 4 * L * x + x * x) / 24)).shift(-1) * (1 / EI) + \
            P.of(Pce * x * x * (3 * L - x) / 6 / EI, -1)
        th = (w * (x * (3 * L * L - 3 * L * x + x * x) / 6)).shift(-1) * (1 / EI) + \
            P.of(Pce * x * (2 * L - x) / 2 / EI, -1)
        return uz, -th
    cf_ok = True
    for name, c in lo_cases.items():
        w = P.of(sec["mu"] * inp["g"] * (c["k"] - 1), 1)
        s1 = c["states"]["lifted"]["s1"]
        for node, x in (("node:b", a_len), ("node:c", L)):
            uz, ry = cant(w, x)
            cf_ok = cf_ok and s1["motions"][node][2] == uz and s1["motions"][node][4] == ry
        # seated: R_B = -u_B(released) * 3 EI / a^3; tip motion adds R_B a^2 (3L - a)/(6EI)
        uB, _ = cant(w, a_len)
        RB = (uB * (-3 * EI / a_len ** 3)).shift(1)
        cf_ok = cf_ok and RB == c["states"]["seated"]["R"]
        uC, _ = cant(w, L)
        uC_seated = uC + (RB * (a_len * a_len * (3 * L - a_len) / 6 / EI)).shift(-1)
        cf_ok = cf_ok and c["states"]["seated"]["s1"]["motions"]["node:c"][2] == uC_seated
    check("LO: closed forms (cantilever and propped cantilever with overhang) equal the solves at B and C", cf_ok)
    # terms and the linear control
    lo_terms = {}
    for name in ("EQZ-SEATED", "EQZ-LIFTED"):
        term = diff_rows(lo_cases[name]["rows"], lo_cases["SUS"]["rows"])
        sd, sn, _ = seismic_loads(inp, secs, lo, [F(0), F(0), lo_cases[name]["k"]])
        lin_s1, lin_R = lo_state((sd, sn), "seated", f"seismic Z alone at k of {name}")
        lin = rows_of(lo, lin_s1, ce_rows=True, lift={"reaction": lin_R})
        lin = [(i, fam, Z if i.startswith("constant_effort/") else v) for i, fam, v in lin]
        lo_terms[f"d{name}"] = {"rows": term, "linear": lin}
    check("LO dEQZ-SEATED: the term equals the seismic-only response of the seated structure on every row",
          [r[2] for r in lo_terms["dEQZ-SEATED"]["rows"]] == [r[2] for r in lo_terms["dEQZ-SEATED"]["linear"]])
    check("LO dEQZ-LIFTED: the term differs from the seated structure's seismic-only response",
          [r[2] for r in lo_terms["dEQZ-LIFTED"]["rows"]] != [r[2] for r in lo_terms["dEQZ-LIFTED"]["linear"]])
    check("LO: the constant-effort force is present in every case and cancels in each term",
          all(r[2] == P.of(Pce) for c in lo_cases.values() for r in c["rows"] if r[0].startswith("constant_effort/"))
          and all(r[2].zero() for t in lo_terms.values() for r in t["rows"] if r[0].startswith("constant_effort/")))
    out["lo"] = {"cases": lo_cases, "terms": lo_terms, "kstar": kstar, "R0": R0, "R1": R1,
                 "weight_prims": wprims_lo}

    # ------------------------------------------------------------- JR
    jr = mods["JR"]
    anchors_jr = {n: set(range(6)) for n in jr["anchors"].values()}
    jr_cases = {}
    sd, sn, sgen = seismic_loads(inp, secs, jr, list(inp["JR_k"]))
    marked = [m["id"] for m in jr["members"]]
    wdj, wgen = wind_loads(inp, secs, jr, marked, 2)
    for name, loads, gen in (("EQ", (sd, sn), sgen), ("WIND", (wdj, {}), wgen)):
        s1, s2 = solve_both(jr, inp, secs, loads, anchors_jr, jr["chains"])
        check(f"JR {name}: method 1 equals method 2", methods_agree(jr, s1, s2, anchors_jr))
        check(f"JR {name}: global equilibrium and free-node balance", equilibrium(jr, s1, loads, anchors_jr))
        check(f"JR {name}: member equilibrium", member_equilibrium(jr, s1, loads))
        # closed form: each run is a fixed-fixed beam; interior node at distance x of a span of length L
        cf = True
        for run, (n0, nm, n1) in (("A-B", ("node:a", "node:m", "node:b")), ("C-D", ("node:c", "node:n", "node:d"))):
            m1 = next(m for m in jr["members"] if m["i"] == n0)
            e, _ = member_geom(jr, m1)
            x0, xm_, x1 = (jr["nodes"][n] for n in (n0, nm, n1))
            Lr = sum(abs(b - a) for a, b in zip(x0, x1))
            x = sum(abs(b - a) for a, b in zip(x0, xm_))
            pr = props(inp, secs, m1)
            w = loads[0][m1["id"]]
            u = s1["motions"][nm]
            for c in range(3):
                wc = w[c]
                if e[c] != 0:   # axial
                    exp = (wc * (x * (Lr - x) / 2 / pr["EA"])).shift(-1)
                    cf = cf and u[c] == exp
                else:
                    v = (wc * (x * x * (Lr - x) ** 2 / 24 / pr["EI"])).shift(-1)
                    slope = (wc * (x * (Lr - x) * (Lr - 2 * x) / 12 / pr["EI"])).shift(-1)
                    cf = cf and u[c] == v
                    rot = cross(e, unit(c))
                    for r in range(3):
                        if rot[r] != 0:
                            cf = cf and u[3 + r] == slope * rot[r]
            # end reactions -wL/2 at each anchor of the run, as the anchors carry only this run here
            for n in (n0, n1):
                cf = cf and all(s1["node_actions"][n][c] == -w[c] * (Lr / 2) for c in range(3))
        check(f"JR {name}: fixed-fixed closed forms (interior motion, end forces -wL/2) equal the solve", cf)
        jr_cases[name] = {"rows": rows_of(jr, s1, ce_rows=False), "generated": gen, "loads": loads}
    check("JR EQ: no generated load names the replaced span",
          all(g.get("member") != "pipe:b-c" for g in sgen))
    # control: the replaced span loaded as if it were a pipe (its load lumped to the anchored faces B and C)
    rep = jr["replaced"][0]
    _, Lbc = member_geom(jr, rep)
    q_bc = [P.of(k * inp["g"] * secs["S2"]["mu"], 1) for k in inp["JR_k"]]
    e_bc, _ = member_geom(jr, rep)
    feq = equivalent_nodal(jr, rep, q_bc)
    # with both faces anchored, the span's fixed-end actions go straight into the anchors at B and C
    ctrl_jr = []
    for sid, n, off in (("support:b", "node:b", 0), ("support:c", "node:c", 6)):
        for c in range(6):
            ctrl_jr.append((f"reaction/{sid}/{ACTS[c]}", "force" if c < 3 else "moment", -feq[off + c]))
    out["jr"] = {"cases": jr_cases, "controls": {"REPLACED-SPAN-LOADED": ctrl_jr}, "q_bc": q_bc, "L_bc": Lbc}

    # ------------------------------------------------------------- discrete outcomes for the binary64 check
    signs = {}
    for c, v in cef_cases.items():
        signs[f"CEF/{c}"] = [r[2].sign() for r in v["rows"]]
    for c, v in cef_terms.items():
        signs[f"CEF/{c}"] = [r[2].sign() for r in v["rows"]]
    for c, v in lo_cases.items():
        signs[f"LO/{c}"] = [r[2].sign() for r in v["rows"]] + [v["state"]]
    for c, v in jr_cases.items():
        signs[f"JR/{c}"] = [r[2].sign() for r in v["rows"]]
    out["signs"] = signs
    return out, checks


# ----------------------------------------------------------------------------- the document
def inputs_doc(inp):
    doc = {}
    for k, v in inp.items():
        if isinstance(v, tuple):
            doc[k] = {"exact": [str(x) for x in v], "unit": unit_of_input(k)}
        else:
            doc[k] = {"exact": str(v), "unit": unit_of_input(k)}
    return doc


def binary64_inputs(inp):
    return {k: (tuple(F(float(x)) for x in v) if isinstance(v, tuple) else F(float(v))) for k, v in inp.items()}


def pub_rows(rows):
    return publish(rows)


def model_doc(m):
    d = {"nodes": {n: [str(c) for c in x] for n, x in m["nodes"].items()},
         "members": [{k: (str(v) if isinstance(v, F) else v) for k, v in mm.items()} for mm in m["members"]],
         "anchors": m["anchors"]}
    if m["replaced"]:
        d["replaced_spans"] = m["replaced"]
    if m["constant_effort"]:
        d["constant_effort_supports"] = {s: {"node": n, "dof": DOFS[k], "constant_load_N": str(f),
                                             "direction": f"+{COMP[k].upper()}"}
                                         for s, (n, k, f) in m["constant_effort"].items()}
    if m.get("lift_off"):
        d["lift_off_supports"] = {s: {"node": n, "dof": DOFS[k], "contact": "on a positive reaction (+Z on the pipe)"}
                                  for s, (n, k) in m["lift_off"].items()}
    if m["components"]:
        d["components"] = {c: {"kind": v["kind"], "rigid_pipe_ref": v["pipe"], "node": v["node"],
                               "weight_N": str(v["W"]), "center_of_gravity_m": [str(x) for x in v["r"]],
                               "stiffness_scale": str(v["scale"]), "solver_consumption": v["solver_consumption"]}
                           for c, v in m["components"].items()}
    return d


def strip(rows):
    return [{k: v for k, v in r.items() if not k.startswith("_")} for r in rows]


def build_doc():
    inp = default_inputs()
    res, checks = build(inp)
    # binary64 input check: discrete outcomes unchanged with every input replaced by its nearest binary64
    res_b, checks_b = build(binary64_inputs(inp))
    changed = sorted(k for k in res["signs"] if res["signs"][k] != res_b["signs"][k])
    n_changed_inputs = sum(1 for k, v in inp.items()
                           if (tuple(F(float(x)) for x in v) if isinstance(v, tuple) else F(float(v))) != v)
    checks.append({"check": f"every input replaced by its nearest binary64 ({n_changed_inputs} of {len(inp)} "
                            f"inputs change): every cross-check still agrees, and the sign of every row and "
                            f"every lift-off state are unchanged", "agrees": not changed and all(
                                c["agrees"] for c in checks_b)})
    secs, mods = res["sections"], res["models"]
    g = inp["g"]

    def sec_doc(s):
        v = secs[s]
        return {"pressure_design_wall_t_r_m": str(v["tr"]), "nominal_inner_radius_m": str(v["rn"]),
                "pressure_bore_radius_m": str(v["rp"]),
                "A_nominal_m2": record(P.of(v["An"], 1)), "I_nominal_m4": record(P.of(v["In"], 1)),
                "J_nominal_m4": record(P.of(2 * v["In"], 1)),
                "mu_metal_kg_per_m": record(P.of(v["mu_metal"], 1)),
                "mu_contents_kg_per_m": record(P.of(v["mu_contents"], 1)),
                "mu_insulation_kg_per_m": record(P.of(v["mu_insulation"], 1)),
                "mu_kg_per_m": record(P.of(v["mu"], 1)), "projected_diameter_m": str(v["projected"]),
                "mu_g_N_per_m": record(P.of(v["mu"] * g, 1))}

    cef = res["cef"]
    case_docs = []
    for name in ("SUS", "EQX", "EQY", "EQZ"):
        c = cef["cases"][name]
        case_docs.append({
            "id": f"CEF-{name}", "model": "CEF",
            "role": "sustained: the weight primitives only" if name == "SUS" else
            f"seismic {name[-1]}: the weight primitives and the case's generated seismic load",
            "seismic": None if name == "SUS" else {"gravity_acceleration_m_s2": str(g), "g_factors": [
                str(inp["CEF_k"][a]) if COMP[a] == name[-1].lower() else "0" for a in range(3)]},
            "load_sources": {
                "declared_ledger": [{"source": p["source"], "factor": 1} for p in cef["weight_prims"]],
                "generated": [{"source": x["source"], "factor": 1} for x in c["generated"]],
                "statement": "the declared ledger lists the self-weight plan's weight primitives at factor 1; each "
                             "generated load is consumed beside it and enters the case's record of load sources "
                             "exactly once, with its identity and factor 1; no generated load is named in the "
                             "declared ledger, and no other generated load enters the record"},
            "constant_effort": "support:ce-b applies its constant force in this case",
            "generated_loads": c["generated"],
            "rows": pub_rows(c["rows"])})
    term_docs = []
    for name in ("dEQX", "dEQY", "dEQZ"):
        t = cef["terms"][name]
        term_docs.append({
            "id": f"CEF-{name}", "basis": "result_state_subtraction",
            "minuend": f"CEF-{name[1:]}", "subtrahend": "CEF-SUS", "fixed_load_count": "0 (1 - 1)",
            "expected_outcome": "published, not withheld: a subtraction carries no gate of its own",
            "statement": "linear model: every row equals the response to the case's generated seismic load alone; "
                         "the constant-effort force cancels",
            "generated_resultant": {"force_N": [record(x) for x in t["Fg"]],
                                    "moment_about_origin_N_m": [record(x) for x in t["Mg"]]},
            "reaction_resultant": {"force_N": [record(x) for x in t["Rf"]],
                                   "moment_about_origin_N_m": [record(x) for x in t["Rm"]],
                                   "supports": sorted(mods["CEF"]["anchors"]),
                                   "equals_minus_generated_resultant": all(
                                       (x + y).zero() for x, y in zip(t["Rf"] + t["Rm"], t["Fg"] + t["Mg"]))},
            "rows": pub_rows(t["rows"])})
    comb_docs = [
        {"id": "CEF-EQ-SRSS", "basis": "srss", "operands": ["CEF-dEQX", "CEF-dEQY", "CEF-dEQZ"],
         "factors": [1, 1, 1], "fixed_load_count": "0 (every operand 0)",
         "expected_outcome": "published, not withheld: every operand's fixed-load count is exactly 0 and every "
                             "reached load case selects the same material point",
         "rows": strip(cef["srss"])},
        {"id": "CEF-EQ-ABS", "basis": "absolute_sum", "operands": ["CEF-dEQX", "CEF-dEQY", "CEF-dEQZ"],
         "factors": [1, 1, 1], "fixed_load_count": "0 (every operand 0)",
         "expected_outcome": "published, not withheld (as CEF-EQ-SRSS)", "rows": strip(cef["abs"])},
        {"id": "CEF-EQ-SRSS-OF-CASES", "basis": "srss", "operands": ["CEF-EQX", "CEF-EQY", "CEF-EQZ"],
         "factors": [1, 1, 1], "fixed_load_count": "undefined (each operand 1)",
         "expected_outcome": "withheld whole by the fixed-load count gate (DEL-05-02 REQ-011: an absolute sum or "
                             "SRSS any of whose operands has a fixed-load count other than exactly 0); no rows",
         "rows": []},
    ]
    ctrl = cef["controls"]["CE-OMITTED-FROM-SEISMIC"]
    big = max((abs(binary64(v)), i) for i, fam, v in ctrl if fam == "translation")
    controls = [{"id": "CEF-CTRL-CE-OMITTED-FROM-SEISMIC",
                 "what": "a wrong implementation that drops the constant-effort force from the seismic cases "
                         "(keeping it in SUS): each term would differ from the expected term by these rows",
                 "largest_translation_difference": {"row": big[1], "value": big[0]},
                 "rows": pub_rows(ctrl)}]

    lo = res["lo"]
    lo_docs = []
    for name in ("SUS", "EQZ-SEATED", "EQZ-LIFTED"):
        c = lo["cases"][name]
        st = c["states"]
        lo_docs.append({
            "id": f"LO-{name}", "model": "LO",
            "seismic": None if name == "SUS" else {"gravity_acceleration_m_s2": str(g),
                                                  "g_factors": ["0", "0", str(c["k"])]},
            "load_sources": {"declared_ledger": [{"source": p["source"], "factor": 1} for p in lo["weight_prims"]],
                             "generated": [{"source": x["source"], "factor": 1} for x in c["generated"]]},
            "generated_loads": c["generated"],
            "expected_support_status": {"support:lo-b": "in contact (active)" if c["state"] == "seated"
                                        else "lifted (inactive)"},
            "state_test": {
                "seated": {"reaction_Fz": record(st["seated"]["R"]),
                           "qualifies": st["seated"]["R"].sign() > 0},
                "lifted": {"relative_uz": record(st["lifted"]["u"]),
                           "qualifies": st["lifted"]["u"].sign() > 0},
                "rule": "contact holds while the reaction on the pipe is positive (+Z); a lifted support "
                        "re-engages when the pipe would move below its ground (uz < 0); each margin here is "
                        "strict, so the state is not weak"},
            "rows": pub_rows(c["rows"]),
            "other_state_rows": {"state": "lifted" if c["state"] == "seated" else "seated",
                                 "why": "the state that does not qualify, for a reader to see why",
                                 "rows": pub_rows(rows_of(mods["LO"], st["lifted" if c["state"] == "seated" else "seated"]["s1"],
                                                          lift={"reaction": st["lifted" if c["state"] == "seated" else "seated"]["R"]}))}})
    lo_term_docs = []
    for name in ("dEQZ-SEATED", "dEQZ-LIFTED"):
        t = lo["terms"][name]
        diff = diff_rows(t["rows"], t["linear"])
        lin_big = max((abs(binary64(v)), i) for i, fam, v in diff if fam == "translation")
        lo_term_docs.append({
            "id": f"LO-{name}", "basis": "result_state_subtraction", "minuend": f"LO-{name[1:]}",
            "subtrahend": "LO-SUS",
            "statement": ("both operands seated: the term equals the seated structure's seismic-only response"
                          if name.endswith("SEATED") else
                          "the minuend is lifted and the subtrahend seated: the term is the difference of two "
                          "independently solved equilibria, not a response to the seismic load on one structure"),
            "rows": pub_rows(t["rows"]),
            "linear_superposition_control": {
                "what": "the seated structure's response to the seismic load alone, which a product would publish "
                        "if it formed the term from one structure; for EQZ-LIFTED it is wrong",
                "equals_term": [r[2] for r in t["rows"]] == [r[2] for r in t["linear"]],
                "largest_translation_difference": None if lin_big[0] == 0 else {"row": lin_big[1], "value": lin_big[0]},
                "rows": pub_rows(t["linear"])}})
    ks = lo["kstar"]
    threshold = {
        "g_factor_z": record(ks),
        "derivation": "the seated reaction is R(k) = R0 + k R1 with R0 the reaction under weight and the "
                      "constant-effort force, and R1 the reaction per unit g-factor of the generated seismic Z "
                      "load; k* = -R0 / R1",
        "R0_N": record(lo["R0"]), "R1_N": record(lo["R1"]),
        "at_threshold": "both states are weak: the seated reaction and the lifted separation are both exactly zero",
        "below": f"seated (EQZ-SEATED, k = {inp['LO_k_seated']})", "above": f"lifted (EQZ-LIFTED, k = {inp['LO_k_lifted']})"}

    jr = res["jr"]
    jr_docs = []
    for name in ("EQ", "WIND"):
        c = jr["cases"][name]
        jr_docs.append({
            "id": f"JR-{name}", "model": "JR",
            "inputs": ({"seismic": {"gravity_acceleration_m_s2": str(g), "g_factors": [str(x) for x in inp["JR_k"]]}}
                       if name == "EQ" else
                       {"wind": {"pressure_Pa": str(inp["JR_p_wind"]), "shape_factor": str(inp["JR_C_wind"]),
                                 "direction": "global_z",
                                 "exposed_pipe_refs": [m["id"] for m in mods["JR"]["members"]]}}),
            "load_sources": {"declared_ledger": [], "generated": [{"source": x["source"], "factor": 1}
                                                                  for x in c["generated"]],
                             "statement": "no stored primitive: the case's only action is its generated load, "
                                          "consumed beside an empty declared ledger; it solves"},
            "generated_loads": c["generated"],
            "replaced_span": "pipe:b-c carries no generated load and publishes no member result",
            "rows": pub_rows(c["rows"])})
    jr_ctrl = jr["controls"]["REPLACED-SPAN-LOADED"]
    refusals = [{
        "id": "JR-REFUSE-WIND-ON-REPLACED-SPAN",
        "document": "JR-WIND with exposed_pipe_refs also naming pipe:b-c, the span the connector replaces",
        "expected": "the case is refused by name, naming the replaced span; no wind load is generated; no result "
                    "is published for the case",
        "code": None,
        "code_note": "no scope or frozen reference package names a code for a wind mark on a connector-replaced "
                     "span; the frozen packages t4_i12 and t4_u5a name JOINT_REPLACED_SPAN_LOAD_UNOWNED for a "
                     "stored load or eigenstrain the replaced span owns, and this package does not assume that "
                     "code applies to a generated wind mark"}]

    doc = {
        "document": "swbpipe.exact_load_state_generated_interactions_references",
        "status": "candidate frozen references; derived and cross-checked by the author TASK before the engine "
                  "change they judge, not yet independently refuted; no product test reads them",
        "units": {"force": "N", "moment": "N*m", "translation": "m", "rotation": "rad", "length": "m"},
        "theory": "small-displacement linear-elastic 3D frame, Euler-Bernoulli, axial and torsional deformation "
                  "included; supports rigid; a constant-effort support is a constant nodal force with no stiffness; "
                  "a lift-off support is a rigid restraint on its degree of freedom while in contact and free while "
                  "lifted",
        "conventions": {
            "motion": "global translation and right-handed rotation of the node",
            "reaction": "global force of the anchor on the structure at its node, moment about that node",
            "member_end": "global force and moment exerted on the member by its end node, moment about that end "
                          "node (end i at the member's first node)",
            "constant_effort": "the force the support applies on the pipe, along +axis of its degree of freedom",
            "nonlinear_support": "reaction_Fz: the lift-off support's force on the pipe (0 when lifted); "
                                 "relative_uz: the node's uz less its ground's (the ground does not move here)",
            "exact": "sum of rational multiples of pi**k; an SRSS value is sqrt of such a sum",
            "binary64_from_rounded_operands": "on a combination row: RN(sqrt(sum z_i^2)) or RN(sum |z_i|) with z_i "
                                              "the expected binary64 values of the operand rows, each formed "
                                              "exactly and rounded once"},
        "inputs": inputs_doc(inp),
        "derived": {"sections": {"S1": sec_doc("S1"), "S2": sec_doc("S2")},
                    "G_Pa": str(inp["E"] / (2 * (1 + inp["nu"]))),
                    "mass_basis": "source_od_nominal_wall_pressure_bore_areas/v1: metal on the nominal wall, "
                                  "contents on the pressure bore, insulation annulus"},
        "models": {k: model_doc(v) for k, v in mods.items()},
        "weight_primitives": {"CEF": cef["weight_prims"], "LO": lo["weight_prims"]},
        "cases": case_docs + lo_docs + jr_docs,
        "terms": term_docs + lo_term_docs,
        "combinations": comb_docs,
        "lift_off_threshold": threshold,
        "refusals": refusals,
        "controls": controls + [{
            "id": "JR-CTRL-REPLACED-SPAN-LOADED",
            "what": "a wrong implementation that generates seismic load on the replaced span as on a pipe: with "
                    "both connector faces anchored, the span's work-equivalent end actions go straight into the "
                    "reactions at B and C; JR-EQ's reactions would differ by these rows (every other row is "
                    "unchanged)",
            "rows": pub_rows(jr_ctrl)}],
        "crosscheck": checks,
    }
    return doc, checks


def dumps(obj):
    def inline(o):
        return json.dumps(o, ensure_ascii=False, separators=(", ", ": "))

    def emit(o, level):
        pad = " " * level
        if isinstance(o, dict) and ("exact" in o or "check" in o or ("source" in o and "factor" in o and len(o) == 2)):
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


def build_files():
    doc, checks = build_doc()
    text = dumps(doc)
    assert json.loads(text) == json.loads(json.dumps(doc))
    files = {"references.json": text.encode()}
    for fixed in FIXED_FILES:
        p = OUT / fixed
        if p.exists():
            files[fixed] = p.read_bytes()
    files["SHA256SUMS"] = sha256sums({k: v for k, v in files.items() if k != "SHA256SUMS"})
    return files, doc, checks


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--write", action="store_true")
    args = parser.parse_args()
    files, doc, checks = build_files()
    bad = [c for c in checks if not c["agrees"]]
    nrows = sum(len(c["rows"]) for c in doc["cases"]) + sum(len(t["rows"]) for t in doc["terms"]) + \
        sum(len(c["rows"]) for c in doc["combinations"])
    print(f"{len(doc['cases'])} cases, {len(doc['terms'])} terms, {len(doc['combinations'])} combinations, "
          f"{nrows} expected rows")
    print(f"cross-checks: {len(checks)} recorded, {len(bad)} disagreeing")
    for c in bad:
        print("DISAGREES:", c["check"])
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
