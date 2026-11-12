#!/usr/bin/env python3
"""Independent references: generated wind and static seismic loads on the exact
pressure route in a load and reference-state document.

Standard-library Python 3 only. This generator imports, reads and runs no
product code and reads no file other than the ones it writes. Every expected
value is derived here in exact rational arithmetic from linear-elastic,
small-displacement frame mechanics and from the governing scope text
(DEL-05-01 static-equivalent generation and mass basis, DEL-04-07 pressure on
the exact route, DEL-03-08 mass per length, DEL-05-03 section bases and shear
model). All inputs are invented.

Two statically indeterminate models, each anchored (all six degrees of freedom)
at both ends:

  STRAIGHT  A(0,0,0) - M(L1,0,0) - B(L1+L2,0,0), two collinear members of one
            section, Timoshenko (timoshenko_annulus_energy_v1);
  FRAME     A(0,0,0) - C(LA,0,0) - B(LA,LB,0), two legs at a right-angle corner,
            Euler-Bernoulli, axial, torsional and both bending deformations.

Loads, each a separate component solved on its own and in combination:

  thermal   free strain alpha * dT on every member, modulus selected at the
            operating temperature;
  seismic   q = g_factor * g * mu on every member, mu on the exact route's mass
            basis source_od_nominal_wall_pressure_bore_areas/v1 (metal on the
            nominal wall, contents on the pressure bore, insulation annulus);
  wind      q = p_w * C * (D + 2 t_ins) on the marked spans and fractions;
  pressure  "pressure elongation on": closed-end free strain
            (1 - 2 nu) p A_i / (E A_s) on every pressurized member, A_i on the
            pressure bore, A_s the nominal section area; no thrust, because no
            location in these models is unbalanced. "pressure elongation off":
            no pressure contribution.

Method 1: direct stiffness method, each member subdivided at its load
breakpoints so every member load is uniform over a whole element, exact
Timoshenko / Euler-Bernoulli element stiffness, work-equivalent fixed-end
actions. Method 2 (cross-check): force method on the released structure
(a bent cantilever from A), the six reactions at B as redundants, flexibility
and nodal motions by unit-load integration (axial, two shears, torsion, two
bendings, free strain) with Simpson's rule on each interval between load
breakpoints (exact: integrands are at most cubic). Method 2 also solves the
pressure "on" component a second way, in the wall-force formulation (Poisson
shortening -2 nu p A_i/(E A_n), terminal end loads p A_i, junction fluid force
-sum p A s), which must give the same motions and reactions.

Every structural property carries one factor pi (areas and second moments of
an annulus), so all solves are done with pi factored out; values are
sum_k q_k pi**k with rational q_k and k in {-1, 0, 1}. Decimals are printed to
40 significant digits from a Machin value of pi; binary64 values are the
correctly rounded nearest doubles, decided by rational bounds of pi.

Modes:
  python3 -I exact_load_state_reference.py           check: rebuild every output in
                                                      memory and compare byte for byte
                                                      with the committed files
  python3 -I exact_load_state_reference.py --write   write the outputs (intentional changes only)
"""
import argparse
import hashlib
import json
import sys
from decimal import Decimal, getcontext
from fractions import Fraction as F
from pathlib import Path

HERE = Path(__file__).resolve().parent
OUT = HERE.parent
FIXED_FILES = ("README.md", "generators/exact_load_state_reference.py")

# ----------------------------------------------------------------------------- pi
getcontext().prec = 110


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


PI_DEC = 16 * _atan_inv(5) - 4 * _atan_inv(239)
_PI_90 = F(str(PI_DEC)[:92])  # "3." + 90 digits, truncated
PI_LO, PI_HI = _PI_90, _PI_90 + F(1, 10 ** 90)
assert PI_LO < F(314159265358979323847, 10 ** 20) and PI_HI > F(314159265358979323846, 10 ** 20)


class V:
    """sum_k t[k] * pi**k with rational t[k]."""
    __slots__ = ("t",)

    def __init__(self, t=None):
        self.t = {k: F(v) for k, v in (t or {}).items() if v != 0}

    @staticmethod
    def of(q, k):
        return V({k: q})

    def __add__(self, o):
        t = dict(self.t)
        for k, v in o.t.items():
            t[k] = t.get(k, 0) + v
        return V(t)

    def __sub__(self, o):
        return self + (-o)

    def __neg__(self):
        return V({k: -v for k, v in self.t.items()})

    def __mul__(self, r):
        r = F(r)
        return V({k: v * r for k, v in self.t.items()})

    __rmul__ = __mul__

    def shift(self, d):
        return V({k + d: v for k, v in self.t.items()})

    def __eq__(self, o):
        return isinstance(o, V) and self.t == o.t

    def __hash__(self):
        return hash(tuple(sorted(self.t.items())))

    def zero(self):
        return not self.t

    def bounds(self):
        lo = hi = F(0)
        for k, v in self.t.items():
            a, b = v * PI_LO ** k, v * PI_HI ** k
            lo += min(a, b)
            hi += max(a, b)
        return lo, hi

    def sign(self):
        lo, hi = self.bounds()
        if not self.t:
            return 0
        if lo > 0:
            return 1
        if hi < 0:
            return -1
        raise SystemExit("sign not decided by the pi bounds")


Z = V()


def vzero3():
    return [Z, Z, Z]


def binary64(x):
    lo, hi = x.bounds()
    if float(lo) != float(hi):
        raise SystemExit(f"binary64 rounding of {x.t} is not decided by the pi bounds")
    return float(lo) + 0.0


def decimal40(x):
    if x.zero():
        return "0"
    getcontext().prec = 110
    s = Decimal(0)
    for k, v in x.t.items():
        s += Decimal(v.numerator) / Decimal(v.denominator) * PI_DEC ** k
    return format(s.normalize(), ".39e")


def exact_text(x):
    if x.zero():
        return "0"
    parts = []
    for k in sorted(x.t, reverse=True):
        v = x.t[k]
        s = str(abs(v))
        s = s if k == 0 else f"{s}*pi" if k == 1 else f"{s}/pi" if k == -1 else f"{s}*pi**{k}"
        parts.append(("-" if v < 0 else "+", s))
    text = ("-" if parts[0][0] == "-" else "") + parts[0][1]
    for sg, s in parts[1:]:
        text += f" {sg} {s}"
    return text


def record(x):
    if not isinstance(x, V):
        x = V.of(F(x), 0)
    return {"exact": exact_text(x), "decimal": decimal40(x), "value": binary64(x)}


# ----------------------------------------------------------------------------- vectors
def crossFV(r, f):
    """r (Fractions) x f (V)."""
    return [f[2] * r[1] - f[1] * r[2], f[0] * r[2] - f[2] * r[0], f[1] * r[0] - f[0] * r[1]]


def crossFF(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def dotVF(v, f):
    s = Z
    for a, b in zip(v, f):
        if b:
            s = s + a * b
    return s


def addv(a, b):
    return [x + y for x, y in zip(a, b)]


def sub3(a, b):
    return [x - y for x, y in zip(a, b)]


def axes(ex):
    """Right-handed rational local axes for an axis-aligned member direction ex."""
    ref = [F(0), F(0), F(1)] if ex[2] == 0 else [F(1), F(0), F(0)]
    ey = crossFF(ref, ex)
    ez = crossFF(ex, ey)
    for v in (ey, ez):
        assert sum(c * c for c in v) == 1
    return ex, ey, ez


# ----------------------------------------------------------------------------- inputs (invented)
def default_inputs():
    d = F
    return {
        # geometry, m
        "straight_L1": d("1.9"), "straight_L2": d("3.3"),
        "frame_LA": d("4.1"), "frame_LB": d("2.7"),
        # section S1 (straight): outside diameter, nominal wall, mill tolerance, corrosion allowance, insulation thickness, m
        "S1_D": d("0.229"), "S1_t": d("0.0117"), "S1_cm": d("0.0011"), "S1_ca": d("0.0019"), "S1_tins": d("0.064"),
        "S1_rho_metal": d(7870), "S1_rho_contents": d(912), "S1_rho_insulation": d(168),  # kg/m^3
        # section S2 (frame)
        "S2_D": d("0.181"), "S2_t": d("0.0083"), "S2_cm": d("0.0007"), "S2_ca": d("0.0013"), "S2_tins": d("0.041"),
        "S2_rho_metal": d(7870), "S2_rho_contents": d(1037), "S2_rho_insulation": d(133),
        # material: modulus points at the installation and the operating temperature, Pa; nu at both; secant alpha, 1/K
        "T_installation": d("288.5"), "T_operating": d("571.3"),  # K
        "E_installation": d("203.7e9"), "E_operating": d("181.9e9"),
        "nu": d("0.29"), "alpha": d("13.1e-6"),
        # pressure, Pa (internal differential, zero external)
        "p": d("3.7e6"),
        # seismic
        "gravity": d("9.79"), "g_factor_x": d("0.31"), "g_factor_y": d("0.47"), "g_factor_z": d("0.23"),
        # wind
        "wind_pressure": d(940), "wind_shape_factor": d("0.65"),
        "straight_span_start": d("0.15"), "straight_span_end": d("0.7"),
        "frame_span_start": d("0.25"), "frame_span_end": d("0.8"),
    }


INPUT_UNITS = {
    "straight_L1": "m", "straight_L2": "m", "frame_LA": "m", "frame_LB": "m",
    "S1_D": "m", "S1_t": "m", "S1_cm": "m", "S1_ca": "m", "S1_tins": "m",
    "S1_rho_metal": "kg/m^3", "S1_rho_contents": "kg/m^3", "S1_rho_insulation": "kg/m^3",
    "S2_D": "m", "S2_t": "m", "S2_cm": "m", "S2_ca": "m", "S2_tins": "m",
    "S2_rho_metal": "kg/m^3", "S2_rho_contents": "kg/m^3", "S2_rho_insulation": "kg/m^3",
    "T_installation": "K", "T_operating": "K", "E_installation": "Pa", "E_operating": "Pa",
    "nu": "1", "alpha": "1/K", "p": "Pa", "gravity": "m/s^2",
    "g_factor_x": "1", "g_factor_y": "1", "g_factor_z": "1",
    "wind_pressure": "Pa", "wind_shape_factor": "1",
    "straight_span_start": "1", "straight_span_end": "1", "frame_span_start": "1", "frame_span_end": "1",
}

COMP_F = ("Fx", "Fy", "Fz", "Mx", "My", "Mz")
COMP_U = ("ux", "uy", "uz", "rx", "ry", "rz")
AXES = {"x": [F(1), F(0), F(0)], "y": [F(0), F(1), F(0)], "z": [F(0), F(0), F(1)]}


# ----------------------------------------------------------------------------- sections and materials
def section(inp, s):
    D, t, cm, ca, tins = (inp[f"{s}_{k}"] for k in ("D", "t", "cm", "ca", "tins"))
    R = D / 2
    t_r = (t - cm) - ca
    r_n, r_p = R - t, R - t_r
    valid = {"cm_plus_ca_below_t": cm + ca < t, "reduced_wall_positive": t_r > 0, "nominal_bore_positive": r_n > 0,
             "insulation_non_negative": tins >= 0}
    A_n = R * R - r_n * r_n  # /pi
    I_n = (R ** 4 - r_n ** 4) / 4  # /pi
    A_i = r_p * r_p  # /pi, the pressure bore
    metal = inp[f"{s}_rho_metal"] * A_n
    contents = inp[f"{s}_rho_contents"] * A_i
    insulation = inp[f"{s}_rho_insulation"] * ((R + tins) ** 2 - R * R)
    mu = metal + contents + insulation  # /pi, kg/m
    # second route: diameters, pi/4 (D^2 - d^2)
    d_n, d_p, D_i = D - 2 * t, D - 2 * t_r, D + 2 * tins
    mu_diam = (inp[f"{s}_rho_metal"] * (D * D - d_n * d_n) + inp[f"{s}_rho_contents"] * d_p * d_p
               + inp[f"{s}_rho_insulation"] * (D_i * D_i - D * D)) / 4
    return {"R": R, "t_r": t_r, "r_n": r_n, "r_p": r_p, "A_n": A_n, "I_n": I_n, "J_n": 2 * I_n, "A_i": A_i,
            "mu_metal": metal, "mu_contents": contents, "mu_insulation": insulation, "mu": mu, "mu_diam": mu_diam,
            "D_proj": D + 2 * tins, "valid": valid}


def kappa_energy(m, nu):
    """DEL-05-03 contract form of the energy-matched annular shear factor."""
    m2 = m * m
    a, b, n1 = 1 + m2, 1 - m2, 1 + nu
    c = (8 * nu + 14) * nu + 7
    Dd = c * (b * b) + 48 * (m2 * (n1 * n1))
    return (6 * (n1 * n1)) * (a * a) / Dd


# ----------------------------------------------------------------------------- models
def models(inp):
    L1, L2, LA, LB = inp["straight_L1"], inp["straight_L2"], inp["frame_LA"], inp["frame_LB"]
    o = F(0)
    return {
        "STRAIGHT": {
            "nodes": {"node:a": [o, o, o], "node:m": [L1, o, o], "node:b": [L1 + L2, o, o]},
            "members": [("pipe:a-m", "node:a", "node:m"), ("pipe:m-b", "node:m", "node:b")],
            "anchors": [("support:a", "node:a"), ("support:b", "node:b")],
            "section": "S1", "shear_model": "timoshenko_annulus_energy_v1",
            "wind_direction": "y",
            "wind_exposed_pipe_refs": ["pipe:a-m"],
            "wind_spans": [("pipe:m-b", inp["straight_span_start"], inp["straight_span_end"])],
            "pressure_regions": [{"id": "region:a-b", "member_pipe_ids": ["pipe:a-m", "pipe:m-b"],
                                  "terminals": [("node:a", "transfers_to_wall"), ("node:b", "transfers_to_wall")]}],
        },
        "FRAME": {
            "nodes": {"node:a": [o, o, o], "node:c": [LA, o, o], "node:b": [LA, LB, o]},
            "members": [("pipe:a-c", "node:a", "node:c"), ("pipe:c-b", "node:c", "node:b")],
            "anchors": [("support:a", "node:a"), ("support:b", "node:b")],
            "section": "S2", "shear_model": "euler_bernoulli",
            "wind_direction": "z",
            "wind_exposed_pipe_refs": ["pipe:a-c"],
            "wind_spans": [("pipe:c-b", inp["frame_span_start"], inp["frame_span_end"])],
            "pressure_regions": [
                {"id": "region:a-c", "member_pipe_ids": ["pipe:a-c"],
                 "terminals": [("node:a", "transfers_to_wall"), ("node:c", "transfers_to_wall")]},
                {"id": "region:c-b", "member_pipe_ids": ["pipe:c-b"],
                 "terminals": [("node:c", "transfers_to_wall"), ("node:b", "transfers_to_wall")]}],
        },
    }


def member_geom(model, mid):
    for m, i, j in model["members"]:
        if m == mid:
            P0, P1 = model["nodes"][i], model["nodes"][j]
            d = sub3(P1, P0)
            L = sum(abs(c) for c in d)  # axis-aligned
            ex = [c / L for c in d]
            assert sum(c * c for c in ex) == 1
            return P0, P1, L, axes(ex)
    raise KeyError(mid)


def props(inp, model, moduli):
    sec = section(inp, model["section"])
    E = inp["E_installation"] if moduli == "cold" else inp["E_operating"]
    nu = inp["nu"]
    G = E / (2 * (1 + nu))
    kap = kappa_energy(sec["r_n"] / sec["R"], nu) if model["shear_model"] != "euler_bernoulli" else None
    return {"E": E, "G": G, "EA": E * sec["A_n"], "GJ": G * sec["J_n"], "EI": E * sec["I_n"],
            "GAs": None if kap is None else G * kap * sec["A_n"], "kappa": kap, "sec": sec}


# ----------------------------------------------------------------------------- load sets
def generated_loads(inp, model, kind):
    """Generated member loads of one source kind: list of (member, a, b, global q vector of V, source id)."""
    sec = section(inp, model["section"])
    out = []
    if kind.startswith("seismic_"):
        ax = kind[-1]
        q = inp["gravity"] * inp[f"g_factor_{ax}"] * sec["mu"]
        for m, _, _ in model["members"]:
            out.append((m, F(0), F(1), [V.of(q * c, 1) for c in AXES[ax]], f"{m}/seismic_{ax}"))
    elif kind == "wind":
        q = inp["wind_pressure"] * inp["wind_shape_factor"] * sec["D_proj"]
        dvec = AXES[model["wind_direction"]]
        for m in model["wind_exposed_pipe_refs"]:
            out.append((m, F(0), F(1), [V.of(q * c, 0) for c in dvec], f"{m}/wind"))
        for m, a, b in model["wind_spans"]:
            out.append((m, a, b, [V.of(q * c, 0) for c in dvec], f"{m}/wind[{a},{b}]"))
    return out


def free_strain(inp, model, moduli, kind):
    """Per-member free strain (rational) of a strain component."""
    P = props(inp, model, moduli)
    if kind == "thermal":
        e = inp["alpha"] * (inp["T_operating"] - inp["T_installation"])
    elif kind == "pressure_on":
        e = (1 - 2 * inp["nu"]) * inp["p"] * P["sec"]["A_i"] / (P["E"] * P["sec"]["A_n"])
    elif kind == "pressure_poisson":
        e = -2 * inp["nu"] * inp["p"] * P["sec"]["A_i"] / (P["E"] * P["sec"]["A_n"])
    else:
        raise KeyError(kind)
    return {m: e for m, _, _ in model["members"]}


def wall_thrusts(inp, model):
    """Wall-force formulation point loads: each region terminal's end load p A_i along the member's outward axis."""
    sec = section(inp, model["section"])
    loads = {}
    for reg in model["pressure_regions"]:
        for m in reg["member_pipe_ids"]:
            P0, P1, L, (ex, _, _) = member_geom(model, m)
            for node, transfer in reg["terminals"]:
                assert transfer == "transfers_to_wall"
                pos = model["nodes"][node]
                if pos == P0:
                    out = [-c for c in ex]
                elif pos == P1:
                    out = ex
                else:
                    continue
                f = [V.of(inp["p"] * sec["A_i"] * c, 1) for c in out]
                loads[node] = addv(loads.get(node, vzero3()), f)
    return loads


# ----------------------------------------------------------------------------- method 1: stiffness
def k_local(EA, GJ, EI, phi, L):
    k = [[F(0)] * 12 for _ in range(12)]

    def put(idx, mat, c):
        for a, ia in enumerate(idx):
            for b, ib in enumerate(idx):
                k[ia][ib] += c * mat[a][b]
    put([0, 6], [[1, -1], [-1, 1]], EA / L)
    put([3, 9], [[1, -1], [-1, 1]], GJ / L)
    c = EI / ((1 + phi) * L ** 3)
    put([1, 5, 7, 11], [[12, 6 * L, -12, 6 * L], [6 * L, (4 + phi) * L * L, -6 * L, (2 - phi) * L * L],
                        [-12, -6 * L, 12, -6 * L], [6 * L, (2 - phi) * L * L, -6 * L, (4 + phi) * L * L]], c)
    put([2, 4, 8, 10], [[12, -6 * L, -12, -6 * L], [-6 * L, (4 + phi) * L * L, 6 * L, (2 - phi) * L * L],
                        [-12, 6 * L, 12, 6 * L], [-6 * L, (2 - phi) * L * L, 6 * L, (4 + phi) * L * L]], c)
    return k


def breakpoints(inp, model, m):
    pts = {F(0), F(1)}
    for kind in ("wind", "seismic_x"):
        for mm, a, b, _, _ in generated_loads(inp, model, kind):
            if mm == m:
                pts |= {a, b}
    return sorted(pts)


class Stiffness:
    """Assembled, eliminated stiffness of one model at one modulus selection (pi factored out)."""

    def __init__(self, inp, model, moduli):
        self.inp, self.model, self.moduli = inp, model, moduli
        P = props(inp, model, moduli)
        self.P = P
        nodes = dict(model["nodes"])
        self.elems = []
        for m, i, j in model["members"]:
            P0, P1, L, (ex, ey, ez) = member_geom(model, m)
            bps = breakpoints(inp, model, m)
            names = [i] + [f"{m}@{f}" for f in bps[1:-1]] + [j]
            for f, n in zip(bps[1:-1], names[1:-1]):
                nodes[n] = [a + c * f * L for a, c in zip(P0, ex)]
            for e in range(len(bps) - 1):
                le = (bps[e + 1] - bps[e]) * L
                phi = F(0) if P["GAs"] is None else 12 * P["EI"] / (P["GAs"] * le * le)
                self.elems.append({"member": m, "i": names[e], "j": names[e + 1], "f0": bps[e], "f1": bps[e + 1],
                                   "L": le, "R": [ex, ey, ez], "k": k_local(P["EA"], P["GJ"], P["EI"], phi, le)})
        self.nodes = nodes
        anchored = {n for _, n in model["anchors"]}
        self.free = [n for n in nodes if n not in anchored]
        self.dof = {(n, c): 6 * a + c for a, n in enumerate(self.free) for c in range(6)}
        nd = 6 * len(self.free)
        K = [[F(0)] * nd for _ in range(nd)]
        for el in self.elems:
            kg = self.to_global_matrix(el)
            idx = self.idx(el)
            for a, ia in enumerate(idx):
                if ia is None:
                    continue
                for b, ib in enumerate(idx):
                    if ib is not None:
                        K[ia][ib] += kg[a][b]
        self.K = K
        self._factor()

    def idx(self, el):
        return [self.dof.get((el["i"], c)) for c in range(6)] + [self.dof.get((el["j"], c)) for c in range(6)]

    @staticmethod
    def T(el):
        R = el["R"]
        T = [[F(0)] * 12 for _ in range(12)]
        for blk in range(4):
            for r in range(3):
                for c in range(3):
                    T[3 * blk + r][3 * blk + c] = R[r][c]
        return T

    def to_global_matrix(self, el):
        T, k = self.T(el), el["k"]
        kT = [[sum(k[r][s] * T[s][c] for s in range(12)) for c in range(12)] for r in range(12)]
        return [[sum(T[s][r] * kT[s][c] for s in range(12)) for c in range(12)] for r in range(12)]

    def _factor(self):
        n = len(self.K)
        A = [row[:] for row in self.K]
        self.ops = []
        for c in range(n):
            p = next(r for r in range(c, n) if A[r][c] != 0)
            A[c], A[p] = A[p], A[c]
            self.ops.append(("swap", c, p))
            for r in range(c + 1, n):
                if A[r][c]:
                    f = A[r][c] / A[c][c]
                    A[r] = [x - f * y for x, y in zip(A[r], A[c])]
                    self.ops.append(("elim", r, c, f))
        self.U = A

    def solve_tilde(self, b):
        b = list(b)
        for op in self.ops:
            if op[0] == "swap":
                _, c, p = op
                b[c], b[p] = b[p], b[c]
            else:
                _, r, c, f = op
                b[r] = b[r] - b[c] * f
        n = len(b)
        x = [Z] * n
        for c in range(n - 1, -1, -1):
            s = b[c]
            for j in range(c + 1, n):
                if self.U[c][j]:
                    s = s - x[j] * self.U[c][j]
            x[c] = s * (1 / self.U[c][c])
        return x

    def fixed_end(self, el, dist, eps):
        """Local fixed-end actions (V) of the element: uniform loads covering it and a free strain."""
        f0 = [Z] * 12
        L = el["L"]
        for m, a, b, q, _ in dist:
            if m != el["member"] or not (a <= el["f0"] and el["f1"] <= b):
                continue
            ql = [dotVF(q, row) for row in el["R"]]
            f0[0] = f0[0] - ql[0] * (L / 2)
            f0[6] = f0[6] - ql[0] * (L / 2)
            f0[1] = f0[1] - ql[1] * (L / 2)
            f0[7] = f0[7] - ql[1] * (L / 2)
            f0[5] = f0[5] - ql[1] * (L * L / 12)
            f0[11] = f0[11] + ql[1] * (L * L / 12)
            f0[2] = f0[2] - ql[2] * (L / 2)
            f0[8] = f0[8] - ql[2] * (L / 2)
            f0[4] = f0[4] + ql[2] * (L * L / 12)
            f0[10] = f0[10] - ql[2] * (L * L / 12)
        e = eps.get(el["member"], F(0))
        if e:
            n = V.of(self.P["EA"] * e, 1)
            f0[0] = f0[0] + n
            f0[6] = f0[6] - n
        return f0

    def solve(self, dist, eps, nodal=None):
        """Solve for distributed loads, free strains and nodal loads (global, at free nodes only)."""
        nodal = nodal or {}
        nd = len(self.K)
        rhs = [Z] * nd
        f0s = []
        for el in self.elems:
            f0 = self.fixed_end(el, dist, eps)
            f0s.append(f0)
            T = self.T(el)
            g = [Z] * 12
            for r in range(12):
                for s in range(12):
                    if T[s][r]:
                        g[r] = g[r] + f0[s] * T[s][r]
            for a, ia in enumerate(self.idx(el)):
                if ia is not None:
                    rhs[ia] = rhs[ia] - g[a]
        for n, vec in nodal.items():
            assert n in self.free
            for c in range(6):
                rhs[self.dof[(n, c)]] = rhs[self.dof[(n, c)]] + vec[c]
        ut = self.solve_tilde(rhs)  # pi * u
        disp = {n: [Z] * 6 for n in self.nodes}
        for (n, c), i in self.dof.items():
            disp[n][c] = ut[i].shift(-1)
        # element end actions, global, on the member from its nodes
        ends = []
        node_force = {n: [Z] * 6 for n in self.nodes}
        for el, f0 in zip(self.elems, f0s):
            T = self.T(el)
            ug = [ut[i] if i is not None else Z for i in self.idx(el)]  # pi * u
            ul = [Z] * 12
            for r in range(12):
                for s in range(12):
                    if T[r][s]:
                        ul[r] = ul[r] + ug[s] * T[r][s]
            fl = [f0[r] + sum((ul[s] * el["k"][r][s] for s in range(12) if el["k"][r][s]), Z) for r in range(12)]
            fg = [Z] * 12
            for r in range(12):
                for s in range(12):
                    if T[s][r]:
                        fg[r] = fg[r] + fl[s] * T[s][r]
            ends.append(fg)
            for c in range(6):
                node_force[el["i"]][c] = node_force[el["i"]][c] + fg[c]
                node_force[el["j"]][c] = node_force[el["j"]][c] + fg[6 + c]
        reactions = {}
        for sid, n in self.model["anchors"]:
            reactions[sid] = [node_force[n][c] - (nodal.get(n, [Z] * 6)[c]) for c in range(6)]
        member_ends = {}
        for m, i, j in self.model["members"]:
            els = [(el, fg) for el, fg in zip(self.elems, ends) if el["member"] == m]
            member_ends[m] = {"i": els[0][1][:6], "j": els[-1][1][6:]}
        # residual of the free-node equilibrium: sum of member end actions minus nodal load = 0
        for n in self.free:
            for c in range(6):
                assert node_force[n][c] - nodal.get(n, [Z] * 6)[c] == Z
        return {"disp": {n: disp[n] for n in self.model["nodes"]}, "reactions": reactions, "member_ends": member_ends}


# ----------------------------------------------------------------------------- method 2: force method
class ForceMethod:
    """Released structure: bent cantilever from the anchor at the first node; redundants: the six reactions at the last."""

    def __init__(self, inp, model, moduli):
        self.inp, self.model = inp, model
        self.P = props(inp, model, moduli)
        self.path, arc = [], F(0)
        for m, i, j in model["members"]:
            P0, P1, L, R = member_geom(model, m)
            self.path.append({"m": m, "P0": P0, "P1": P1, "L": L, "R": R, "arc0": arc, "arc1": arc + L,
                              "bps": breakpoints(inp, model, m)})
            arc += L
        self.A = model["nodes"][model["anchors"][0][1]]
        self.B = model["nodes"][model["anchors"][1][1]]
        assert self.path[0]["P0"] == self.A and self.path[-1]["P1"] == self.B
        self.node_arc = {}
        for seg in self.path:
            self.node_arc[tuple(seg["P0"])] = seg["arc0"]
            self.node_arc[tuple(seg["P1"])] = seg["arc1"]

    def section(self, k, s, dist, points, X):
        """Action on the +x face of the i-side portion at s on path member k: global (F, M about the point)."""
        seg = self.path[k]
        Xp = [a + c * s for a, c in zip(seg["P0"], seg["R"][0])]
        Fo, Mo = vzero3(), vzero3()
        for m, a, b, q, _ in dist:
            kk = next(i for i, sg in enumerate(self.path) if sg["m"] == m)
            if kk < k:
                continue
            sg = self.path[kk]
            lo = a * sg["L"] if kk > k else max(s, a * sg["L"])
            hi = b * sg["L"]
            if hi <= lo:
                continue
            Fr = [c * (hi - lo) for c in q]
            mid = [p + c * (lo + hi) / 2 for p, c in zip(sg["P0"], sg["R"][0])]
            Fo = addv(Fo, Fr)
            Mo = addv(Mo, crossFV(sub3(mid, Xp), Fr))
        for pos, Fv, Mv in points:
            if self.node_arc[tuple(pos)] >= seg["arc1"]:
                Fo = addv(Fo, Fv)
                Mo = addv(Mo, addv(crossFV(sub3(pos, Xp), Fv), Mv))
        Fo = addv(Fo, X[:3])
        Mo = addv(Mo, addv(crossFV(sub3(self.B, Xp), X[:3]), X[3:]))
        return Fo, Mo, Xp

    def _integrate(self, upto_arc, real, virtual, eps):
        """Unit-load integral to the point at arc upto_arc. Returns pi * (flexural part) and the free-strain part."""
        P = self.P
        flex, strain = Z, Z
        for k, seg in enumerate(self.path):
            if seg["arc0"] >= upto_arc:
                break
            ex, ey, ez = seg["R"]
            pts = sorted({b * seg["L"] for b in seg["bps"]})
            e = eps.get(seg["m"], F(0))

            def f(s):
                Fr, Mr, Xp = real(k, s)
                Fv, Mv = virtual(Xp)
                acc = Z
                terms = [(Fr, Fv, ex, P["EA"]), (Mr, Mv, ex, P["GJ"]), (Mr, Mv, ey, P["EI"]), (Mr, Mv, ez, P["EI"])]
                if P["GAs"] is not None:
                    terms += [(Fr, Fv, ey, P["GAs"]), (Fr, Fv, ez, P["GAs"])]
                for rv, vv, axis, stiff in terms:
                    vr = sum(a * b for a, b in zip(vv, axis))
                    if vr:
                        acc = acc + dotVF(rv, axis) * (vr / stiff)
                n = sum(a * b for a, b in zip(Fv, ex))
                return acc, n
            for lo, hi in zip(pts, pts[1:]):
                h = hi - lo
                (a0, n0), (a1, n1), (a2, n2) = f(lo), f((lo + hi) / 2), f(hi)
                flex = flex + (a0 + a1 * 4 + a2) * (h / 6)
                strain = strain + V.of((n0 + 4 * n1 + n2) * h / 6 * e, 0)
        return flex, strain

    @staticmethod
    def unit(P, comp):
        e = [F(int(i == comp % 3)) for i in range(3)]
        if comp < 3:
            return lambda Xp: (e, crossFF(sub3(P, Xp), e))
        return lambda Xp: ([F(0)] * 3, e)

    def solve(self, dist, eps, points=()):
        points = [(pos, Fv, Mv) for pos, Fv, Mv in points]
        zeroX = [Z] * 6
        arcB = self.path[-1]["arc1"]
        # flexibility of the redundants (pi * true flexibility, rational)
        Phi = [[F(0)] * 6 for _ in range(6)]
        for kk in range(6):
            Xu = [V.of(F(int(i == kk)), 0) for i in range(6)]
            for r in range(6):
                fl, _ = self._integrate(arcB + 1, lambda k, s: self.section(k, s, [], [], Xu), self.unit(self.B, r), {})
                assert set(fl.t) <= {0}
                Phi[r][kk] = fl.t.get(0, F(0))
        d0 = []
        for r in range(6):
            fl, st = self._integrate(arcB + 1, lambda k, s: self.section(k, s, dist, points, zeroX), self.unit(self.B, r), eps)
            d0.append(fl.shift(-1) + st)
        # Phi X = -pi d0
        X = gauss(Phi, [-(d.shift(1)) for d in d0])
        real = lambda k, s: self.section(k, s, dist, points, X)
        disp = {}
        for n, pos in self.model["nodes"].items():
            arc = self.node_arc[tuple(pos)]
            vals = []
            for c in range(6):
                fl, st = self._integrate(arc, real, self.unit(pos, c), eps)
                vals.append(fl.shift(-1) + st)
            disp[n] = vals
        member_ends = {}
        for k, seg in enumerate(self.path):
            F0, M0, _ = self.section(k, F(0), dist, points, X)
            F1, M1, _ = self.section(k, seg["L"], dist, points, X)
            member_ends[seg["m"]] = {"i": [-x for x in F0 + M0], "j": F1 + M1}
        pa = {tuple(pos): (Fv, Mv) for pos, Fv, Mv in points}
        PA = pa.get(tuple(self.A), (vzero3(), vzero3()))
        fA = member_ends[self.path[0]["m"]]["i"]
        reactions = {self.model["anchors"][0][0]: [fA[c] - (PA[0] + PA[1])[c] for c in range(6)],
                     self.model["anchors"][1][0]: X}
        return {"disp": disp, "reactions": reactions, "member_ends": member_ends}


def gauss(M, b):
    n = len(M)
    A = [row[:] for row in M]
    b = list(b)
    for c in range(n):
        p = next(r for r in range(c, n) if A[r][c] != 0)
        A[c], A[p], b[c], b[p] = A[p], A[c], b[p], b[c]
        for r in range(c + 1, n):
            if A[r][c]:
                f = A[r][c] / A[c][c]
                A[r] = [x - f * y for x, y in zip(A[r], A[c])]
                b[r] = b[r] - b[c] * f
    x = [Z] * n
    for c in range(n - 1, -1, -1):
        s = b[c]
        for j in range(c + 1, n):
            if A[c][j]:
                s = s - x[j] * A[c][j]
        x[c] = s * (1 / A[c][c])
    return x


# ----------------------------------------------------------------------------- results helpers
def add_results(a, b):
    return {"disp": {n: [x + y for x, y in zip(a["disp"][n], b["disp"][n])] for n in a["disp"]},
            "reactions": {s: [x + y for x, y in zip(a["reactions"][s], b["reactions"][s])] for s in a["reactions"]},
            "member_ends": {m: {e: [x + y for x, y in zip(a["member_ends"][m][e], b["member_ends"][m][e])]
                                for e in ("i", "j")} for m in a["member_ends"]}}


def zero_results(model):
    return {"disp": {n: [Z] * 6 for n in model["nodes"]},
            "reactions": {s: [Z] * 6 for s, _ in model["anchors"]},
            "member_ends": {m: {"i": [Z] * 6, "j": [Z] * 6} for m, _, _ in model["members"]}}


def flat(res):
    rows = {}
    for n, v in res["disp"].items():
        for c in range(6):
            rows[f"motion/{n}/{COMP_U[c]}"] = v[c]
    for s, v in res["reactions"].items():
        for c in range(6):
            rows[f"reaction/{s}/{COMP_F[c]}"] = v[c]
    for m, e in res["member_ends"].items():
        for end in ("i", "j"):
            for c in range(6):
                rows[f"member_end/{m}/{end}/{COMP_F[c]}"] = e[end][c]
    return rows


def family(row_id):
    comp = row_id.rsplit("/", 1)[1]
    return {"ux": "translation", "uy": "translation", "uz": "translation", "rx": "rotation", "ry": "rotation",
            "rz": "rotation", "Fx": "force", "Fy": "force", "Fz": "force", "Mx": "moment", "My": "moment",
            "Mz": "moment"}[comp]


def unit_of(fam):
    return {"translation": "m", "rotation": "rad", "force": "N", "moment": "N*m"}[fam]


def equilibrium(model, res, dist, points=()):
    """Sum of reactions and applied loads: force and moment about the origin. Returns six V."""
    Fs, Ms = vzero3(), vzero3()
    for sid, n in model["anchors"]:
        r = res["reactions"][sid]
        Fs = addv(Fs, r[:3])
        Ms = addv(Ms, addv(crossFV(model["nodes"][n], r[:3]), r[3:]))
    for m, a, b, q, _ in dist:
        P0, _, L, (ex, _, _) = member_geom(model, m)
        Fr = [c * ((b - a) * L) for c in q]
        mid = [p + c * (a + b) / 2 * L for p, c in zip(P0, ex)]
        Fs = addv(Fs, Fr)
        Ms = addv(Ms, crossFV(mid, Fr))
    for pos, Fv, Mv in points:
        Fs = addv(Fs, Fv)
        Ms = addv(Ms, addv(crossFV(pos, Fv), Mv))
    return Fs + Ms


# ----------------------------------------------------------------------------- cases
COMPONENT_KINDS = {
    # id suffix: (moduli, dist kinds, strain kinds)
    "pressure_elongation@installation": ("cold", [], ["pressure_on"]),
    "pressure_elongation@operating": ("hot", [], ["pressure_on"]),
    "thermal@operating": ("hot", [], ["thermal"]),
    "wind@installation": ("cold", ["wind"], []),
    "wind@operating": ("hot", ["wind"], []),
    "seismic_x@operating": ("hot", ["seismic_x"], []),
    "seismic_y@operating": ("hot", ["seismic_y"], []),
    "seismic_z@operating": ("hot", ["seismic_z"], []),
}

CASES = {
    "STRAIGHT": [
        ("REF", "cold", [], False),
        ("THERMAL", "hot", ["thermal@operating"], True),
        ("WIND-COLD", "cold", ["wind@installation"], True),
        ("WIND-HOT", "hot", ["wind@operating"], True),
        ("SEISMIC-X", "hot", ["seismic_x@operating"], True),
        ("SEISMIC-Y", "hot", ["seismic_y@operating"], True),
        ("SEISMIC-Z", "hot", ["seismic_z@operating"], True),
        ("THERMAL-WIND", "hot", ["thermal@operating", "wind@operating"], True),
    ],
    "FRAME": [
        ("REF", "cold", [], False),
        ("THERMAL", "hot", ["thermal@operating"], True),
        ("WIND-COLD", "cold", ["wind@installation"], True),
        ("WIND-HOT", "hot", ["wind@operating"], True),
        ("SEISMIC-X", "hot", ["seismic_x@operating"], True),
        ("SEISMIC-Y", "hot", ["seismic_y@operating"], True),
        ("SEISMIC-Z", "hot", ["seismic_z@operating"], True),
        ("THERMAL-SEISMIC-Y", "hot", ["thermal@operating", "seismic_y@operating"], True),
    ],
}


def loads_of(inp, model, comps, moduli):
    dist, eps = [], {}
    for cid in comps:
        cm, dk, sk = COMPONENT_KINDS[cid]
        assert cm == moduli, (cid, moduli)
        for k in dk:
            dist += generated_loads(inp, model, k)
        for k in sk:
            for m, e in free_strain(inp, model, moduli, k).items():
                eps[m] = eps.get(m, F(0)) + e
    return dist, eps


def pressure_component(moduli):
    return "pressure_elongation@installation" if moduli == "cold" else "pressure_elongation@operating"


class Checks(list):
    def add(self, name, ok, detail=""):
        self.append({"check": name, "agrees": bool(ok), **({"detail": detail} if detail else {})})


def build_all(inp, checks):
    mods = models(inp)
    out = {"components": {}, "cases": [], "alternative": {}, "sections": {}}
    for s in ("S1", "S2"):
        sec = section(inp, s)
        out["sections"][s] = sec
        checks.add(f"{s}: mass per length, annulus radii against diameters", sec["mu"] == sec["mu_diam"])
        checks.add(f"{s}: section validity (c_m + c_a < t, positive walls and bores)", all(sec["valid"].values()))
    for name, model in mods.items():
        stiff = {mo: Stiffness(inp, model, mo) for mo in ("cold", "hot")}
        force = {mo: ForceMethod(inp, model, mo) for mo in ("cold", "hot")}
        comp_res = {}
        for cid, (mo, dk, sk) in COMPONENT_KINDS.items():
            dist, eps = loads_of(inp, model, [cid], mo)
            r1 = stiff[mo].solve(dist, eps)
            r2 = force[mo].solve(dist, eps)
            checks.add(f"{name} {cid}: stiffness method equals force method (all rows)", flat(r1) == flat(r2))
            eq = equilibrium(model, r1, dist)
            checks.add(f"{name} {cid}: global equilibrium (force and moment about the origin)", all(x.zero() for x in eq))
            comp_res[cid] = r1
            out["components"][f"{name}/{cid}"] = {"model": name, "moduli": mo, "result": r1, "dist": dist, "eps": eps}
        # a free strain proportional to 1/E on members sharing E: forces independent of E, motions proportional to 1/E
        pc, ph = flat(comp_res["pressure_elongation@installation"]), flat(comp_res["pressure_elongation@operating"])
        ratio = inp["E_operating"] / inp["E_installation"]
        checks.add(f"{name}: pressure elongation forces equal at both modulus selections, motions in the ratio E_operating/E_installation",
                   all(pc[k] == ph[k] for k in pc if not k.startswith("motion/"))
                   and all(pc[k] == ph[k] * ratio for k in pc if k.startswith("motion/")))
        wc, wh = flat(comp_res["wind@installation"]), flat(comp_res["wind@operating"])
        checks.add(f"{name}: wind at the two modulus selections: forces equal, motions in the ratio E_operating/E_installation, "
                   "and the motions differ",
                   all(wc[k] == wh[k] for k in wc if not k.startswith("motion/"))
                   and all(wc[k] == wh[k] * ratio for k in wc if k.startswith("motion/"))
                   and any(wc[k] != wh[k] for k in wc))
        # wall-force formulation of the same pressure state (Poisson shortening, terminal end loads, junction loads)
        sec = section(inp, model["section"])
        thr = wall_thrusts(inp, model)
        pts = [(model["nodes"][n], f, vzero3()) for n, f in thr.items()]
        for mo in ("cold", "hot"):
            eps_w = free_strain(inp, model, mo, "pressure_poisson")
            rw = force[mo].solve([], eps_w, pts)
            re = comp_res[pressure_component(mo)]
            same = (all(rw["disp"][n] == re["disp"][n] for n in model["nodes"])
                    and all(rw["reactions"][s] == re["reactions"][s] for s, _ in model["anchors"]))
            checks.add(f"{name} pressure ({mo} modulus): wall-force formulation gives the same motions and reactions", same)
            pa = V.of(inp["p"] * sec["A_i"], 1)
            ok = True
            for m, _, _ in model["members"]:
                _, _, _, (ex, _, _) = member_geom(model, m)
                for end, sg in (("i", -1), ("j", 1)):
                    dw = sub3(rw["member_ends"][m][end][:3], re["member_ends"][m][end][:3])
                    ok &= all(d == pa * (sg * c) for d, c in zip(dw, ex))
                    ok &= all(a == b for a, b in zip(rw["member_ends"][m][end][3:], re["member_ends"][m][end][3:]))
            checks.add(f"{name} pressure ({mo} modulus): wall force exceeds effective force by p*A_i at every member end", ok)
            eqw = equilibrium(model, rw, [], pts)
            checks.add(f"{name} pressure ({mo} modulus): wall-force formulation in global equilibrium", all(x.zero() for x in eqw))
        # alternative reading of "off": terminal end loads and junction loads applied, no free strain
        alt = force["hot"].solve([], {}, pts)
        out["alternative"][name] = {"result": alt, "thrusts": thr}
        checks.add(f"{name}: alternative-reading thrust response in global equilibrium",
                   all(x.zero() for x in equilibrium(model, alt, [], pts)))
        # cases
        for cid, mo, comps, pressurized in CASES[name]:
            res = {}
            for setting in ("off", "on"):
                use = list(comps) + ([pressure_component(mo)] if (pressurized and setting == "on") else [])
                dist, eps = loads_of(inp, model, use, mo)
                direct = stiff[mo].solve(dist, eps) if use else zero_results(model)
                summed = zero_results(model)
                for c in use:
                    summed = add_results(summed, comp_res[c])
                checks.add(f"{name}-{cid} pressure elongation {setting}: direct solve equals the sum of its components",
                           flat(direct) == flat(summed))
                if use:
                    fm = force[mo].solve(dist, eps)
                    checks.add(f"{name}-{cid} pressure elongation {setting}: force method direct solve agrees",
                               flat(direct) == flat(fm))
                    checks.add(f"{name}-{cid} pressure elongation {setting}: global equilibrium",
                               all(x.zero() for x in equilibrium(model, direct, dist)))
                res[setting] = (direct, use)
            out["cases"].append({"model": name, "id": f"{name}-{cid}", "moduli": mo, "components": comps,
                                 "pressurized": pressurized, "res": res,
                                 "generated": [g for c in comps for k in COMPONENT_KINDS[c][1]
                                               for g in generated_loads(inp, model, k)]})
        # closed forms
        if name == "STRAIGHT":
            P = props(inp, model, "hot")
            Nth = V.of(-P["EA"] * inp["alpha"] * (inp["T_operating"] - inp["T_installation"]), 1)
            r = comp_res["thermal@operating"]
            checks.add("STRAIGHT thermal: restrained axial force -E*A*alpha*dT, no motion",
                       r["member_ends"]["pipe:a-m"]["j"][0] == Nth and all(x.zero() for v in r["disp"].values() for x in v))
            Rp = V.of((1 - 2 * inp["nu"]) * inp["p"] * sec["A_i"], 1)
            r = comp_res["pressure_elongation@operating"]
            checks.add("STRAIGHT pressure elongation: anchor A reaction +(1-2nu)p*A_i along X, no motion",
                       r["reactions"]["support:a"][0] == Rp and r["reactions"]["support:b"][0] == -Rp
                       and all(x.zero() for v in r["disp"].values() for x in v))
            # fixed-fixed beam, uniform transverse load over both members (seismic Z), Timoshenko: wL/2 reactions
            q = inp["gravity"] * inp["g_factor_z"] * sec["mu"]
            Ltot = inp["straight_L1"] + inp["straight_L2"]
            r = comp_res["seismic_z@operating"]
            checks.add("STRAIGHT seismic Z: end reactions -qL/2 and end moments +-qL^2/12 (symmetric uniform load)",
                       r["reactions"]["support:a"][2] == V.of(-q * Ltot / 2, 1)
                       and r["reactions"]["support:a"][4] == V.of(q * Ltot * Ltot / 12, 1)
                       and r["reactions"]["support:b"][4] == V.of(-q * Ltot * Ltot / 12, 1))
    return out


# ----------------------------------------------------------------------------- discrete outcomes
def signature(out):
    sig = []
    for s, sec in out["sections"].items():
        sig.append((s, tuple(sorted(sec["valid"].items()))))
    for c in out["cases"]:
        off, on = flat(c["res"]["off"][0]), flat(c["res"]["on"][0])
        for k in off:
            sig.append((c["id"], k, off[k].sign(), on[k].sign(), off[k] == on[k]))
    for k, comp in out["components"].items():
        for r, v in flat(comp["result"]).items():
            sig.append((k, r, v.sign()))
    return sig


# ----------------------------------------------------------------------------- document
def vec_record(v):
    return [record(x) for x in v]


def gen_record(inp, model, g):
    m, a, b, q, sid = g
    P0, _, L, (ex, _, _) = member_geom(model, m)
    Lload = (b - a) * L
    return {"source": sid, "member": m, "start_fraction": str(a), "end_fraction": str(b),
            "loaded_length_m": record(Lload),
            "intensity_N_per_m": vec_record(q),
            "resultant_N": vec_record([c * Lload for c in q]),
            "resultant_point_m": [str(p + c * (a + b) / 2 * L) for p, c in zip(P0, ex)],
            "factor": 1}


def rows_of(case):
    off, on = flat(case["res"]["off"][0]), flat(case["res"]["on"][0])
    rows = []
    for k in off:
        fam = family(k)
        row = {"id": k, "family": fam, "unit": unit_of(fam)}
        if off[k] == on[k]:
            row["pressure_elongation"] = "independent"
            row["expected"] = record(off[k])
        else:
            row["pressure_elongation"] = "dependent"
            row["expected_pressure_elongation_off"] = record(off[k])
            row["expected_pressure_elongation_on"] = record(on[k])
        rows.append(row)
    return rows


def model_record(inp, name, model):
    P = {mo: props(inp, model, mo) for mo in ("cold", "hot")}
    return {
        "nodes": {n: [str(c) for c in p] for n, p in model["nodes"].items()},
        "members": [{"id": m, "end_i": i, "end_j": j, "section": model["section"],
                     "length_m": str(member_geom(model, m)[2])} for m, i, j in model["members"]],
        "anchors": [{"id": s, "node": n, "restrained": list(COMP_U)} for s, n in model["anchors"]],
        "shear_model": model["shear_model"],
        "pressure_regions": [{"id": r["id"], "member_pipe_ids": r["member_pipe_ids"],
                              "pressure_basis": "internal_differential_zero_external_v1",
                              "pressure_Pa": str(inp["p"]),
                              "terminals": [{"node_ref": n, "closure_transfer": t} for n, t in r["terminals"]]}
                             for r in model["pressure_regions"]],
        "wind": {"direction": f"global_{model['wind_direction']}", "exposed_pipe_refs": model["wind_exposed_pipe_refs"],
                 "exposed_spans": [{"pipe_ref": m, "start_fraction": str(a), "end_fraction": str(b)}
                                   for m, a, b in model["wind_spans"]]},
        "stiffness": {label: {"E_Pa": str(P[mo]["E"]), "G_Pa": str(P[mo]["G"]),
                           "EA_N": record(V.of(P[mo]["EA"], 1)), "EI_N_m2": record(V.of(P[mo]["EI"], 1)),
                           "GJ_N_m2": record(V.of(P[mo]["GJ"], 1)),
                           **({"kappa_E": record(P[mo]["kappa"]), "GA_s_N": record(V.of(P[mo]["GAs"], 1))}
                              if P[mo]["GAs"] is not None else {})}
                      for mo, label in (("cold", "installation"), ("hot", "operating"))},
    }


def section_record(inp, s, sec):
    return {
        "t_r_m": str(sec["t_r"]), "nominal_inner_radius_m": str(sec["r_n"]), "pressure_bore_radius_m": str(sec["r_p"]),
        "A_n_m2": record(V.of(sec["A_n"], 1)), "I_n_m4": record(V.of(sec["I_n"], 1)), "J_n_m4": record(V.of(sec["J_n"], 1)),
        "A_i_m2": record(V.of(sec["A_i"], 1)),
        "mass_per_length_kg_per_m": {"metal_nominal_wall": record(V.of(sec["mu_metal"], 1)),
                                     "contents_pressure_bore": record(V.of(sec["mu_contents"], 1)),
                                     "insulation": record(V.of(sec["mu_insulation"], 1)),
                                     "total": record(V.of(sec["mu"], 1))},
        "mass_basis": "source_od_nominal_wall_pressure_bore_areas/v1",
        "projected_diameter_m": str(sec["D_proj"]),
    }


def ledger(case):
    gens = case["generated"]
    return {"generated_sources": [{"source": g[4], "member": g[0], "factor": 1} for g in gens],
            "count": len(gens),
            "statement": "each generated load above enters the case's record of load sources exactly once, at factor 1; "
                         "no other generated load enters it; the thermal states and the pressure regions are not "
                         "generated loads, and this package states nothing about whether or how they are listed"}


REFUSALS = [
    {"id": "REFUSE-WIND-NO-SHAPE-FACTOR", "document": "either model, the WIND-HOT case with wind.shape_factor absent",
     "expected": "the case is refused by name, naming the missing input; no wind load is generated; no result is published for that case"},
    {"id": "REFUSE-WIND-NO-PRESSURE", "document": "either model, the WIND-HOT case with wind.pressure absent",
     "expected": "refused by name as above"},
    {"id": "REFUSE-WIND-NO-DIRECTION", "document": "either model, the WIND-HOT case with wind.direction absent",
     "expected": "refused by name as above"},
    {"id": "REFUSE-SEISMIC-NO-GRAVITY", "document": "either model, the SEISMIC-Y case with seismic.gravity_acceleration absent and g_factor_y present",
     "expected": "the case is refused by name, naming the missing gravity; no seismic load is generated; no result is published for that case"},
    {"id": "REFUSE-WIND-SPAN-NOT-A-PIPE", "document": "either model, the WIND-HOT case with an exposed_spans entry whose pipe_ref is 'support:a' (a support, not a pipe of the model)",
     "expected": "the case is refused by name, naming the span and its target; no wind load is generated for the case"},
    {"id": "REFUSE-WIND-SPAN-INVERTED", "document": "either model, the WIND-HOT case with the exposed span's start_fraction and end_fraction exchanged",
     "expected": "the case is refused by name as an inverted span (DEL-05-01 AC-003); no wind load is generated for the case"},
]


def build():
    inp = default_inputs()
    checks = Checks()
    out = build_all(inp, checks)
    # discrete outcomes with every input replaced by its binary64 value
    inp_b = {k: F(float(v)) for k, v in inp.items()}
    changed = sorted(k for k in inp if inp_b[k] != inp[k])
    out_b = build_all(inp_b, Checks())
    checks.add("discrete outcomes (section validity, sign of every row and component row, setting dependence) "
               "unchanged with every input replaced by its binary64 value",
               signature(out) == signature(out_b), f"{len(changed)} of {len(inp)} inputs change under binary64 rounding")
    mods = models(inp)
    doc = {
        "document": "swbpipe.exact_load_state_equivalent_static_references",
        "status": "candidate frozen references; derived and cross-checked by the author TASK, not yet independently refuted",
        "units": {"force": "N", "moment": "N*m", "translation": "m", "rotation": "rad", "length": "m"},
        "theory": "small-displacement linear-elastic 3D frame; STRAIGHT Timoshenko (energy-matched annular factor on the nominal annulus), FRAME Euler-Bernoulli; axial and torsional deformation included; rotations are section rotations",
        "conventions": {
            "motion": "global translation and right-handed rotation of the node",
            "reaction": "global force of the anchor on the structure at its node, moment about that node",
            "member_end": "global force and moment exerted on the member by its end node, moment about that end node (end i at the member's first node)",
            "pressure_elongation": "'independent': the row is the same with the per-case option off and on; 'dependent': both values are given",
            "exact": "sum of rational multiples of pi**k, k in {-1, 0, 1}",
        },
        "inputs": {k: {"exact": str(v), "unit": INPUT_UNITS[k]} for k, v in inp.items()},
        "derived": {
            "temperature_change_K": str(inp["T_operating"] - inp["T_installation"]),
            "thermal_free_strain": record(inp["alpha"] * (inp["T_operating"] - inp["T_installation"])),
            "pressure_elongation_free_strain": {
                f"{s}@{lab}": record((1 - 2 * inp["nu"]) * inp["p"] * out["sections"][s]["A_i"]
                                     / ((inp["E_installation"] if lab == "installation" else inp["E_operating"]) * out["sections"][s]["A_n"]))
                for s in ("S1", "S2") for lab in ("installation", "operating")},
            "sections": {s: section_record(inp, s, sec) for s, sec in out["sections"].items()},
        },
        "models": {n: model_record(inp, n, m) for n, m in mods.items()},
        "generated_loads": {n: {k: [gen_record(inp, m, g) for g in generated_loads(inp, m, k)]
                                for k in ("wind", "seismic_x", "seismic_y", "seismic_z")} for n, m in mods.items()},
        "components": [{"id": k, "model": c["model"], "modulus_selection": "installation" if c["moduli"] == "cold" else "operating",
                        "rows": [{"id": r, "family": family(r), "unit": unit_of(family(r)), "expected": record(v)}
                                 for r, v in flat(c["result"]).items()]}
                       for k, c in out["components"].items()],
        "cases": [],
        "refusals": REFUSALS,
        "alternative_reading_not_expected": {
            n: {"reading": "pressure elongation off, read as: each region terminal's end load p*A_i and each junction's fluid force applied, no free strain (README, Open readings 1)",
                "thrusts_N": {node: vec_record(f) for node, f in a["thrusts"].items()},
                "rows": [{"id": r, "value": record(v)} for r, v in flat(a["result"]).items() if not v.zero()]}
            for n, a in out["alternative"].items()},
        "crosscheck": list(checks),
    }
    for c in out["cases"]:
        mo = c["moduli"]
        model = mods[c["model"]]
        doc["cases"].append({
            "id": c["id"], "model": c["model"],
            "load_reference_state": {
                "element_states": [{"pipe_ref": m, "material_selection": f"point at the {'installation' if mo == 'cold' else 'operating'} temperature",
                                    "thermal_state": ("coefficient: alpha * (T_operating - T_installation)"
                                                      if "thermal@operating" in c["components"] else "none")}
                                   for m, _, _ in model["members"]],
                "load_sources": ledger(c)},
            "pressure_regions": "as the model" if c["pressurized"] else "none",
            "superposition": {"pressure_elongation_off": c["res"]["off"][1], "pressure_elongation_on": c["res"]["on"][1]},
            "generated_loads": [gen_record(inp, model, g) for g in c["generated"]],
            "expected_outcome": "published as zero without a finding (DEL-04-07 REQ-007)" if not c["components"] else "solved",
            "rows": rows_of(c),
        })
    return doc, checks


def dumps(obj):
    def inline(o):
        return json.dumps(o, ensure_ascii=False, separators=(", ", ": "))

    def emit(o, level):
        pad = " " * level
        if isinstance(o, dict) and ("exact" in o or "check" in o or "source" in o and "factor" in o and len(o) == 3):
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
    doc, checks = build()
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
    nrows = sum(len(c["rows"]) for c in doc["cases"])
    ndep = sum(1 for c in doc["cases"] for r in c["rows"] if r["pressure_elongation"] == "dependent")
    print(f"{len(doc['cases'])} cases, {nrows} case rows ({ndep} pressure-elongation dependent), "
          f"{sum(len(c['rows']) for c in doc['components'])} component rows")
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
