"""T4-U6 independent references: the D-5 section bases on one straight pipe.

Standard library only (fractions, decimal). Imports, reads and runs no product code and reads no
product output; every input is a constant in this file. From any directory:

    python3 -I -B generators/u6_reference.py           # verify every committed file byte for byte
    python3 -I -B generators/u6_reference.py --write   # rewrite them (intentional changes only)

Basis (read, not imported):
- DEL-05-03 Design section-bases-and-timoshenko.md sections 1 (D-5 geometry and its binary64 order),
  2.1 (stress per basis), 2.5 (row naming), 3.1 (kappa_E), 7 and 8, and the rulings in
  section-and-shear-decisions.md (D-5, H4), at b3f4da7661;
- SHEAR_REFERENCE CONTRACT.md sections 1, 3 and 4 and its reference arithmetic (archive f2211ecef6):
  the energy-matched annular factor and the exact static Timoshenko element;
- the repaired T4-I8 freeze (5b07d85cf3; validation/references/t4_i8/) for the row shape, stations,
  sign conventions and the criterion;
- a prepared exact 0.5.0 self-weight input document (read as data) for the generated self-weight's request
  spelling only; no frozen value depends on it.

Every value is exact: a finite sum of rational * pi**k, or a square root of one (support norms and
member maxima), evaluated in 130-digit Decimal. Closed forms are cross-checked by an exact
direct-stiffness solve (SHEAR_REFERENCE section 3 element, 1, 2 and 4 elements), global equilibrium,
end-row identities, the general Lame family and the axial compliance identity.
"""

import hashlib
import json
import math
import sys
from decimal import Decimal, getcontext
from fractions import Fraction as F
from math import isqrt
from pathlib import Path

getcontext().prec = 130
HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
CHECKS = []


def check(label, condition):
    CHECKS.append((label, bool(condition)))
    if not condition:
        print("FAIL", label)


# ---------------------------------------------------------------- pi, two ways (as T4-I8)
def pi_machin():
    getcontext().prec = 140

    def arctan_inv(n):
        x = Decimal(1) / n
        x2 = x * x
        term, total, k = x, x, 1
        while True:
            term *= -x2
            add = term / (2 * k + 1)
            if abs(add) < Decimal(10) ** -138:
                break
            total += add
            k += 1
        return total

    value = 16 * arctan_inv(5) - 4 * arctan_inv(239)
    getcontext().prec = 130
    return +value


def pi_gauss_legendre():
    getcontext().prec = 140
    a, b, t, p = Decimal(1), Decimal(1) / Decimal(2).sqrt(), Decimal(1) / 4, Decimal(1)
    for _ in range(9):
        an = (a + b) / 2
        b = (a * b).sqrt()
        t -= p * (a - an) ** 2
        a = an
        p *= 2
    value = (a + b) ** 2 / (4 * t)
    getcontext().prec = 130
    return +value


PI = pi_machin()
check("pi Machin == Gauss-Legendre to 1e-125", abs(PI - pi_gauss_legendre()) < Decimal(10) ** -125)


# ---------------------------------------------------------------- exact numbers
def rat(c):
    c = F(c)
    return str(c.numerator) if c.denominator == 1 else f"{c.numerator}/{c.denominator}"


class PP:
    """A finite sum of rational * pi**k, exact."""

    def __init__(self, terms=None):
        self.t = {k: F(v) for k, v in (terms or {}).items() if F(v) != 0}

    @staticmethod
    def of(x, k=0):
        return x if isinstance(x, PP) else PP({k: F(x)})

    def __add__(self, o):
        o = PP.of(o)
        t = dict(self.t)
        for k, v in o.t.items():
            t[k] = t.get(k, F(0)) + v
        return PP(t)

    __radd__ = __add__

    def __neg__(self):
        return PP({k: -v for k, v in self.t.items()})

    def __sub__(self, o):
        return self + (-PP.of(o))

    def __rsub__(self, o):
        return PP.of(o) - self

    def __mul__(self, o):
        o = PP.of(o)
        t = {}
        for k1, v1 in self.t.items():
            for k2, v2 in o.t.items():
                t[k1 + k2] = t.get(k1 + k2, F(0)) + v1 * v2
        return PP(t)

    __rmul__ = __mul__

    def __truediv__(self, o):
        o = PP.of(o)
        assert len(o.t) == 1, "division by a non-monomial"
        (k2, v2), = o.t.items()
        return PP({k - k2: v / v2 for k, v in self.t.items()})

    def __rtruediv__(self, o):
        return PP.of(o) / self

    def __eq__(self, o):
        return isinstance(o, (PP, int, F)) and self.t == PP.of(o).t

    def zero(self):
        return not self.t

    def dec(self):
        total = Decimal(0)
        for k, v in sorted(self.t.items()):
            total += Decimal(v.numerator) / Decimal(v.denominator) * PI ** k
        return total

    def text(self):
        if not self.t:
            return "0"
        parts = []
        for k, v in sorted(self.t.items()):
            parts.append(f"({rat(v)})" + ("" if k == 0 else ("*pi" if k == 1 else f"*pi^{k}")))
        return " + ".join(parts)


class Sym:
    """A value with a square root: the expression in exact terms and its 130-digit evaluation."""

    def __init__(self, expression, d):
        self.expression = expression
        self.d = +d

    def zero(self):
        return self.d == 0

    def dec(self):
        return self.d

    def text(self):
        return self.expression


def sqrt_of(rad):
    """sqrt of a PP: exact when the PP is c pi^(2j) with c a rational square, else symbolic."""
    if rad.zero():
        return PP()
    if len(rad.t) == 1:
        (k, v), = rad.t.items()
        if k % 2 == 0 and v > 0:
            n, d = isqrt(v.numerator), isqrt(v.denominator)
            if n * n == v.numerator and d * d == v.denominator:
                return PP({k // 2: F(n, d)})
    assert rad.dec() > 0
    return Sym(f"sqrt({rad.text()})", rad.dec().sqrt())


def v_add(a, b):
    if isinstance(a, PP) and isinstance(b, PP):
        return a + b
    return Sym(f"{a.text()} + {b.text()}", a.dec() + b.dec())


def v_div(a, z):
    if isinstance(a, PP):
        return a / z
    return Sym(f"({a.text()}) / ({z.text()})", a.dec() / z.dec())


def v_abs(a):
    if isinstance(a, PP):
        return a if a.dec() >= 0 else -a
    return a if a.d >= 0 else Sym(f"-({a.expression})", -a.d)


def norm3(vec):
    return sqrt_of(vec[0] * vec[0] + vec[1] * vec[1] + vec[2] * vec[2])


KIND = {0: "rational", 1: "rational_times_pi", -1: "rational_over_pi"}


def exact_json(v):
    if isinstance(v, Sym):
        return {"kind": "symbolic", "expression": v.expression}
    if not v.t:
        return {"kind": "rational", "rational": "0"}
    if len(v.t) == 1:
        (k, c), = v.t.items()
        if k in KIND:
            return {"kind": KIND[k], "rational": rat(c)}
    return {"kind": "pi_polynomial", "terms": [{"pi_power": k, "rational": rat(c)} for k, c in sorted(v.t.items())]}


def dec40(d):
    return "0" if d == 0 else format(d, ".39e")


def qty(v, unit):
    v = PP.of(v) if not isinstance(v, (PP, Sym)) else v
    d = v.dec()
    out = {"unit": unit, "exact": exact_json(v), "decimal": dec40(d), "value": float(d)}
    if isinstance(v, Sym):
        out["evaluation"] = "Decimal at 130 digits; pi by Machin, checked against Gauss-Legendre"
    return out


# ---------------------------------------------------------------- inputs (invented)
PROV = "invented T4-U6 section-basis reference input; not library, catalog, manufacturer or code-rule data"
UNITS = {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}
OD, WALL, MILL, ALLOWANCE = F(1, 5), F(1, 100), F(1, 800), F(1, 500)  # 0.2, 0.01, 0.00125, 0.002 m
E_MOD, NU, ALPHA = F(200 * 10 ** 9), F(3, 10), F(3, 250000)        # Pa, 1, 1/degC (1.2e-5)
G_MOD = E_MOD / (2 * (1 + NU))
LENGTH, PRESSURE, DELTA_T = F(2), F(5 * 10 ** 6), F(40)              # m, Pa, degC
RHO_PIPE, RHO_CONTENTS, GRAVITY = F(7850), F(1000), F(980665, 100000)  # kg/m^3, kg/m^3, m/s^2 (along -Z)
TIP_FORCE = (F(0), F(1200), F(-900))   # N at node B, global (a 3-4-5 pair: hypot 1500)
TIP_MOMENT = (F(600), F(0), F(0))      # N*m at node B (torsion)
AXIAL_FORCE = F(20000)                 # N at node B in the unpressurized load case
BASES = ("nominal_section", "reduced_section")
STATIONS = [("end_i", F(0)), ("quarter_1", F(1, 4)), ("midspan", F(1, 2)), ("quarter_3", F(3, 4)), ("end_j", F(1))]
EB, TIMO = "euler_bernoulli", "timoshenko_annulus_energy_v1"
# The generated self-weight as the 0.5.0 model format records it: the section carries material_density and
# contents_density; the load case carries a distributed_force primitive whose provenance is the generation record
# (method ids, gravity, mass inputs, mass per length and the generated payload). The spelling follows a prepared
# exact 0.5.0 self-weight input document supplied by the epoch manager (read as data; README, generated self-weight).
SELF_WEIGHT_METHOD = "pipe_mass_per_length_times_explicit_axis_acceleration/v2"
SELF_WEIGHT_MASS_METHOD = "source_od_nominal_wall_pressure_bore_areas/v1"
GRAVITY_AXIS = "global_z"
UNBOUND_MODEL_HASH = "sha256:" + "0" * 64


# ---------------------------------------------------------------- section geometry (Design section 1)
def annulus(od, wall):
    od, wall = F(od), F(wall)
    ro = od / 2
    ri = ro - wall
    As = PP({1: wall * (od - wall)})
    I = As * PP({0: (ro * ro + ri * ri) / 4})
    check(f"As == pi(ro^2 - ri^2) [{od}, {wall}]", As == PP({1: ro * ro - ri * ri}))
    check(f"I == pi(ro^4 - ri^4)/4 [{od}, {wall}]", I == PP({1: (ro ** 4 - ri ** 4) / 4}))
    return {"wall": wall, "ro": ro, "ri": ri, "As": As, "Ai": PP({1: ri * ri}), "I": I, "J": I * 2, "Z": I / PP({0: ro})}


def d5_sections(mill, allowance):
    """Nominal section on the wall t; reduced section on t_r = (t - c_m) - c_a; pressure bore r_i,r = OD/2 - t_r."""
    t_r = (WALL - mill) - allowance
    check(f"c_m + c_a < t [{mill}, {allowance}]", 0 <= mill and 0 <= allowance and mill + allowance < WALL)
    nominal, reduced = annulus(OD, WALL), annulus(OD, t_r)
    check("pressure bore r_i,r == OD/2 - t + c_m + c_a", reduced["ri"] == OD / 2 - WALL + mill + allowance)
    return nominal, reduced, t_r


def kappa_energy(ri, ro, nu):
    """kappa_E (SHEAR_REFERENCE section 1) in the Design section 3.1 cancellation-free form, exactly."""
    m = F(ri) / F(ro)
    m2 = m * m
    a, b, n1 = 1 + m2, 1 - m2, 1 + nu
    c = (8 * nu + 14) * nu + 7
    D = c * (b * b) + 48 * (m2 * (n1 * n1))
    check("kappa_E denominator equals the CONTRACT form", D == (7 + 14 * nu + 8 * nu * nu) * a * a + 4 * m2 * (5 + 10 * nu + 4 * nu * nu))
    return (6 * (n1 * n1)) * (a * a) / D


def kappa_binary64(ri, ro, nu):
    """The same operations in binary64 (Python float: IEEE round to nearest, no fused multiply-add)."""
    m = ri / ro
    m2 = m * m
    a = 1 + m2
    b = 1 - m2
    n1 = 1 + nu
    c = (8 * nu + 14) * nu + 7
    D = c * (b * b) + 48 * (m2 * (n1 * n1))
    return (6 * (n1 * n1)) * (a * a) / D


def binary64_order(mill, allowance):
    D_, t_, m_, a_ = float(OD), float(WALL), float(mill), float(allowance)
    t_r = (t_ - m_) - a_
    ri_r = D_ / 2 - t_r
    other = D_ / 2 - t_ + m_ + a_
    ri_n = D_ / 2 - t_
    return {"order": "t_r = (t - c_m) - c_a, then r_i,r = OD/2 - t_r; r_i,n = OD/2 - t (Design section 1)",
            "t_r": t_r, "t_r_hex": t_r.hex(), "ri_reduced": ri_r, "ri_reduced_hex": ri_r.hex(),
            "ri_nominal": ri_n, "ri_nominal_hex": ri_n.hex(),
            "the_other_order": "((OD/2 - t) + c_m) + c_a", "ri_reduced_by_the_other_order": other,
            "ri_reduced_by_the_other_order_hex": other.hex(), "orders_round_alike": other == ri_r,
            "zero_allowance_reproduces_t_minus_c_m": ((t_ - m_) - 0.0) == (t_ - m_)}


# ---------------------------------------------------------------- physics of one evaluation
def physics(coincident, shear, mut=None):
    """The section, load and stiffness bases of one evaluation; mut names a wrong mapping (a discriminator)."""
    mut = mut or {}
    mill, allowance = (F(0), F(0)) if coincident else (MILL, ALLOWANCE)
    nominal, reduced, t_r = d5_sections(mill, allowance)
    walls = {"nominal": WALL, "mill_only": WALL - mill, "allowance_only": WALL - allowance, "reduced": t_r}
    stiff = annulus(OD, walls[mut.get("stiffness", "nominal")])
    bases = {"nominal_section": nominal, "reduced_section": reduced}
    if mut.get("reduced_section") == "mill_only":
        bases["reduced_section"] = annulus(OD, walls["mill_only"])
    bore_ri = {"pressure_bore": reduced["ri"], "nominal_bore": nominal["ri"], "mill_only_bore": OD / 2 - walls["mill_only"]}
    cap_ri = bore_ri[mut.get("cap_bore", "pressure_bore")]
    contents_ri = bore_ri[mut.get("contents_bore", "pressure_bore")]
    lame_ri = {b: bases[b]["ri"] for b in BASES}
    if mut.get("nominal_lame_at_pressure_bore"):
        lame_ri["nominal_section"] = reduced["ri"]
    shear_model = mut.get("shear_model", shear)
    phys = {"coincident": coincident, "mill": mill, "allowance": allowance, "t_r": t_r, "nominal": nominal, "reduced": reduced,
            "stiff": stiff, "bases": bases, "lame_ri": lame_ri, "Ai": PP({1: cap_ri * cap_ri}),
            "eps_p_area": annulus(OD, walls[mut["eps_p_area"]])["As"] if "eps_p_area" in mut else stiff["As"],
            "poisson_area": annulus(OD, walls[mut["poisson_area"]])["As"] if "poisson_area" in mut else stiff["As"],
            "w": GRAVITY * (RHO_PIPE * annulus(OD, walls[mut.get("metal", "nominal")])["As"] + RHO_CONTENTS * PP({1: contents_ri * contents_ri})),
            "copy": mut.get("copy"), "shear_model": shear_model, "kappa": None, "Ks": None}
    if shear_model == TIMO:
        sec = annulus(OD, walls[mut.get("shear_section", mut.get("stiffness", "nominal"))])
        kappa = kappa_energy(sec["ri"], sec["ro"], NU)
        phys["kappa"], phys["shear_area"] = kappa, sec["As"] * kappa
        phys["Ks"] = phys["shear_area"] * G_MOD
    return phys


LOAD_CASES = {
    "case:pressure-tip": {"pressure": True, "tip": True, "axial": False, "weight": False, "thermal": False, "restrained": False,
                          "description": "internal pressure 5 MPa with both closures transferring to the wall, and a tip force (0, 1200, -900) N and torque 600 N*m at node B"},
    "case:axial-tip": {"pressure": False, "tip": True, "axial": True, "weight": False, "thermal": False, "restrained": False,
                       "description": "no pressure region (explicitly empty pressure_regions): a tip force (20000, 1200, -900) N and torque 600 N*m at node B"},
    "case:self-weight": {"pressure": False, "tip": False, "axial": False, "weight": True, "thermal": False, "restrained": False,
                         "description": "generated self-weight: pipe metal (7850 kg/m^3) on the nominal wall and contents (1000 kg/m^3) on the pressure bore, g = 9.80665 m/s^2 along -Z"},
    "case:combined": {"pressure": True, "tip": True, "axial": False, "weight": True, "thermal": False, "restrained": False,
                      "description": "case:pressure-tip plus the generated self-weight of case:self-weight"},
    "case:restrained-pressure": {"pressure": True, "tip": False, "axial": False, "weight": False, "thermal": False, "restrained": True,
                                 "description": "both ends anchored; internal pressure 5 MPa with both closures transferring to the wall"},
    "case:restrained-pressure-thermal": {"pressure": True, "tip": False, "axial": False, "weight": False, "thermal": True, "restrained": True,
                                         "description": "both ends anchored; internal pressure 5 MPa (closures transferring) and a uniform temperature change of 40 degC"},
}


def respond(phys, lc):
    """Closed-form response of the straight member A (x = 0) -> B (x = L), local axes = global."""
    L, E, nu = LENGTH, E_MOD, NU
    P = PRESSURE * phys["Ai"] if lc["pressure"] else PP()
    w = phys["w"] if lc["weight"] else PP()
    Fx = AXIAL_FORCE if lc["axial"] else F(0)
    Fy, Fz = (TIP_FORCE[1], TIP_FORCE[2]) if lc["tip"] else (F(0), F(0))
    Mx = TIP_MOMENT[0] if lc["tip"] else F(0)
    eps_th = ALPHA * DELTA_T if lc["thermal"] else F(0)
    A, I, J = phys["stiff"]["As"], phys["stiff"]["I"], phys["stiff"]["J"]
    EA, EI, GJ = A * E, I * E, J * G_MOD
    # free strain of the pressurized tube: the eps_p area and the Poisson-eigenstrain area are the stiffness As
    # unless a discriminator moves one of them (Design section 1; T4-I8 RV1 N-5)
    fsp = PP()
    if lc["pressure"]:
        fsp = P / EA - P * (2 * nu) / (phys["poisson_area"] * E) + P * (1 - 2 * nu) / (phys["eps_p_area"] * E) - P * (1 - 2 * nu) / EA
    Ks = phys["Ks"]
    r = {"P": P, "w": w}
    if not lc["restrained"]:
        N = P + Fx                                     # wall force (N_w) or element axial force
        strain = PP.of(eps_th) + (N - P) / EA + fsp
        r["N"] = N
        r["disp"] = {"node:A": [PP()] * 6}
        uy = PP.of(Fy) * L ** 3 / (EI * 3)
        uz = PP.of(Fz) * L ** 3 / (EI * 3) - w * L ** 4 / (EI * 8)
        if Ks is not None:
            uy = uy + PP.of(Fy) * L / Ks
            uz = uz + PP.of(Fz) * L / Ks - w * L ** 2 / (Ks * 2)
        r["disp"]["node:B"] = [strain * L, uy, uz, PP.of(Mx) * L / GJ,
                               PP.of(-Fz) * L ** 2 / (EI * 2) + w * L ** 3 / (EI * 6), PP.of(Fy) * L ** 2 / (EI * 2)]
        r["supports"] = {"support:A": [PP.of(-Fx), PP.of(-Fy), PP.of(-Fz) + w * L,
                                       PP.of(-Mx), PP.of(L * Fz) - w * L ** 2 / 2, PP.of(-L * Fy)]}

        def cut(s):
            x = L - s
            return [N, PP.of(Fy), PP.of(Fz) - w * x, PP.of(Mx), PP.of(-x * Fz) + w * x * x / 2, PP.of(x * Fy)]
        r["cut"] = cut
        r["applied"] = {"force": [PP.of(Fx), PP.of(Fy), PP.of(Fz) - w * L], "moment_about_A": [PP.of(Mx), PP.of(-L * Fz) + w * L * L / 2, PP.of(L * Fy)]}
    else:
        N = P - EA * (PP.of(eps_th) + fsp)
        r["N"] = N
        r["disp"] = {"node:A": [PP()] * 6, "node:B": [PP()] * 6}
        r["supports"] = {"support:A": [P - N] + [PP()] * 5, "support:B": [N - P] + [PP()] * 5}
        r["cut"] = lambda s: [N, PP(), PP(), PP(), PP(), PP()]
        r["applied"] = {"force": [PP()] * 3, "moment_about_A": [PP()] * 3}
        r["free_extension"] = (PP.of(eps_th) + fsp) * L   # the same document with B released (zero-scale basis)
    return r


def stresses(phys, lc, cut_values):
    """Each stress of one section on each basis b: N/A_b, M/Z_b, T r_o/J_b and b's own Lame solution."""
    N, Vy, Vz, T, My, Mz = cut_values
    out = {}
    for b in BASES:
        s = phys["bases"][b]
        d = {"axial": N / s["As"], "bending_y": My / s["Z"], "bending_z": Mz / s["Z"], "torsion": T * s["ro"] / s["J"]}
        if lc["pressure"]:
            rl = phys["lame_ri"][b]
            outer = PRESSURE * 2 * PP({1: rl * rl}) / s["As"]
            d.update({"lame_inner_radial": PP.of(-PRESSURE), "lame_outer_radial": PP(), "lame_inner_hoop": outer + PRESSURE, "lame_outer_hoop": outer})
        out[b] = d
    if phys["copy"] == "reduced_from_nominal":
        out["reduced_section"] = dict(out["nominal_section"])
    elif phys["copy"] == "nominal_from_reduced":
        out["nominal_section"] = dict(out["reduced_section"])
    return out


def maximum(phys, lc, r):
    """Per basis: max over the member of |N/A_b| + hypot(M_y, M_z)/Z_b. |M_y| and |M_z| are nonincreasing
    toward the free end here (checked), so the maximum is at end_i."""
    N, Vy, Vz, T, My, Mz = r["cut"](F(0))
    h = sqrt_of(My * My + Mz * Mz)
    grid = [r["cut"](LENGTH * F(k, 64)) for k in range(65)]
    check("hypot(M_y, M_z) is largest at end_i", all(abs(g[4].dec()) <= abs(My.dec()) and abs(g[5].dec()) <= abs(Mz.dec()) for g in grid))
    out = {}
    for b in BASES:
        s = phys["bases"][b]
        out[b] = v_add(v_abs(N / s["As"]), v_div(h, s["Z"])) if not (isinstance(h, PP) and h.zero()) else v_abs(N / s["As"])
    if phys["copy"] == "reduced_from_nominal":
        out["reduced_section"] = out["nominal_section"]
    elif phys["copy"] == "nominal_from_reduced":
        out["nominal_section"] = out["reduced_section"]
    return out


# ---------------------------------------------------------------- rows (T4-I8 shape)
TRANS = [("element_local_shear_force_y", "shear_force_y", "N", 1), ("element_local_shear_force_z", "shear_force_z", "N", 2),
         ("element_local_torsional_moment", "torsional_moment", "N*m", 3),
         ("element_local_bending_moment_y", "bending_moment_y", "N*m", 4), ("element_local_bending_moment_z", "bending_moment_z", "N*m", 5)]
STRESS = [("element_local_bending_normal_stress_y", "bending_normal_stress_y", "bending_y"),
          ("element_local_bending_normal_stress_z", "bending_normal_stress_z", "bending_z"),
          ("element_local_torsional_shear_stress", "torsional_shear_stress", "torsion")]
PRESSURE_STRESS = [("pipe_axial_membrane_stress_v2", "axial_membrane_stress", "axial"),
                   ("pipe_lame_radial_stress_v2", "lame_inner_radial_stress", "lame_inner_radial"),
                   ("pipe_lame_radial_stress_v2", "lame_outer_radial_stress", "lame_outer_radial"),
                   ("pipe_lame_hoop_stress_v2", "lame_inner_hoop_stress", "lame_inner_hoop"),
                   ("pipe_lame_hoop_stress_v2", "lame_outer_hoop_stress", "lame_outer_hoop")]
PIPE = "pipe:A-B"


def rows_of(phys, lc):
    """Every published row of one load case: (identity, value, row unit, reference unit, frame)."""
    r = respond(phys, lc)
    rows = []

    def add(kind, entity, component, location, unit, frame, value, basis=None, ref_unit=None):
        rows.append(((kind, entity, component, location, basis), value, unit, ref_unit or unit, frame))

    cuts = {loc: r["cut"](LENGTH * f) for loc, f in STATIONS}
    st = {loc: stresses(phys, lc, cuts[loc]) for loc, _ in STATIONS}
    for loc, _ in STATIONS:
        if lc["pressure"]:
            add("pipe_wall_axial_force_v2", PIPE, "wall_axial_force", loc, "N", "element_local", cuts[loc][0])
            add("pipe_effective_axial_force_v2", PIPE, "effective_axial_force", loc, "N", "element_local", cuts[loc][0] - r["P"])
            for kind, comp, key in PRESSURE_STRESS:
                for b in BASES:
                    add(kind, PIPE, comp, loc, "Pa", "pipe_section", st[loc][b][key], b)
    if lc["pressure"]:
        add("pipe_wall_endpoint_action_v2", PIPE, "wall_axial_end_action", "end_i", "N", "element_local", -cuts["end_i"][0])
        add("pipe_wall_endpoint_action_v2", PIPE, "wall_axial_end_action", "end_j", "N", "element_local", cuts["end_j"][0])
    else:  # members outside every pressure region publish the element axial family (row identity inferred, see README)
        for loc, _ in STATIONS:
            add("element_local_axial_force", PIPE, "axial_force", loc, "N", "element_local", -cuts[loc][0] if loc == "end_i" else cuts[loc][0])
    for loc, _ in STATIONS:
        sign = -1 if loc == "end_i" else 1   # end rows node-on-element: end_i = -(j-side cut at 0), end_j = +(cut at L)
        for kind, comp, unit, idx in TRANS:
            add(kind, PIPE, comp, loc, unit, "element_local", cuts[loc][idx] * sign)
    for loc, _ in STATIONS:
        if not lc["pressure"]:
            for b in BASES:
                add("element_local_axial_normal_stress", PIPE, "axial_normal_stress", loc, "MPa", "element_local", st[loc][b]["axial"], b, "Pa")
        for kind, comp, key in STRESS:
            for b in BASES:
                add(kind, PIPE, comp, loc, "MPa", "element_local", st[loc][b][key], b, "Pa")
    mx = maximum(phys, lc, r)
    for b in BASES:
        add("pipe_elastic_normal_stress_maximum_v2", PIPE, "maximum_absolute_normal_stress", "governing_station", "Pa", "pipe_section", mx[b], b)
    for nid, d in r["disp"].items():
        for a, axis in enumerate("xyz"):
            add(f"global_nodal_displacement_{axis}", nid, f"nodal_displacement_{axis}", "node", "mm", "global", d[a], None, "m")
        for a, axis in enumerate("xyz"):
            add(f"global_nodal_rotation_{axis}", nid, f"nodal_rotation_{axis}", "node", "rad", "global", d[3 + a])
    for sid, comps in r["supports"].items():
        for c, v in zip(["Fx", "Fy", "Fz", "Mx", "My", "Mz"], comps):
            add("support_reaction_component_v2", sid, c, "node", "N" if c[0] == "F" else "N*m", "global", v)
        add("support_reaction_force_magnitude_v2", sid, "force_magnitude", "node", "N", "global", norm3(comps[:3]))
        add("support_reaction_moment_magnitude_v2", sid, "moment_magnitude", "node", "N*m", "global", norm3(comps[3:]))
    evidence = []
    if lc["pressure"]:   # contract_evidence.pressure terminal vectors: the cap thrust P = p Ai on the pressure bore
        for node, sign in (("node:A", -1), ("node:B", 1)):
            for field in ("closure_pressure_load_global_n", "pipe_cap_transfer_global_n"):
                for comp, v in zip("xyz", (r["P"] * sign, PP(), PP())):
                    evidence.append(((field, node, comp), v))
    return rows, evidence, r, cuts


def second_method_checks(phys, lc, r, cuts):
    """Independent cross-checks of one evaluation."""
    L = LENGTH
    if not lc["restrained"]:
        # global equilibrium: support + applied (caps cancel on the free member) == 0
        sup = r["supports"]["support:A"]
        check("global force balance", all((sup[a] + r["applied"]["force"][a]).zero() for a in range(3)))
        check("global moment balance about A", all((sup[3 + a] + r["applied"]["moment_about_A"][a]).zero() for a in range(3)))
        # end-row identities: node-on-element end_i equals the support action at A for this cantilever
        c0 = r["cut"](F(0))
        check("end_i node-on-element == support-on-pipe at A (transverse, moments)",
              (-c0[1] - sup[1]).zero() and (-c0[2] - sup[2]).zero() and (-c0[3] - sup[3]).zero() and (-c0[4] - sup[4]).zero() and (-c0[5] - sup[5]).zero())
        # exact direct-stiffness solve with the SHEAR_REFERENCE element, 1, 2 and 4 elements, both planes
        EI_r = (phys["stiff"]["I"] * E_MOD).t[1]
        Ks_r = None if phys["Ks"] is None else phys["Ks"].t[1]
        Fy = TIP_FORCE[1] if lc["tip"] else F(0)
        Fz = TIP_FORCE[2] if lc["tip"] else F(0)
        w_r = r["w"].t.get(1, F(0))
        for n in (1, 2, 4):
            vy, ty = beam_fe(n, L, EI_r, Ks_r, Fy, F(0))
            vz_f, tz_f = beam_fe(n, L, EI_r, Ks_r, Fz, F(0))
            vz_w, tz_w = beam_fe(n, L, EI_r, Ks_r, F(0), -w_r)
            dB = r["disp"]["node:B"]
            check(f"FE ({n} elements) u_y(B) == closed form", PP({-1: vy}) == dB[1])
            check(f"FE ({n} elements) r_z(B) == closed form", PP({-1: ty}) == dB[5])
            check(f"FE ({n} elements) u_z(B) == closed form", PP({-1: vz_f, 0: vz_w}) == dB[2])
            check(f"FE ({n} elements) r_y(B) == -theta (x-z plane)", PP({-1: -tz_f, 0: -tz_w}) == dB[4])
    else:
        sA, sB = r["supports"]["support:A"], r["supports"]["support:B"]
        check("restrained axial balance with the caps", (sA[0] + sB[0]).zero())
    if lc["pressure"]:
        sh = stresses(phys, lc, cuts["midspan"])
        for b in BASES:
            s = phys["bases"][b]
            ro, ri = s["ro"], s["ri"]
            C = PRESSURE * ri * ri / (ro * ro - ri * ri)
            Dc = C * ro * ro
            # the basis's own annulus: the general Lame family sigma_r = C - D/r^2, sigma_h = C + D/r^2
            check(f"Lame {b}: general family", sh[b]["lame_inner_radial"] == PP.of(C - Dc / (ri * ri))
                  and sh[b]["lame_outer_radial"] == PP.of(C - Dc / (ro * ro)) and sh[b]["lame_inner_hoop"] == PP.of(C + Dc / (ri * ri))
                  and sh[b]["lame_outer_hoop"] == PP.of(C + Dc / (ro * ro)))
        # the axial compliance identity on the stiffness As: eps - eps_th = (Nw - 2 nu P)/(E As)
        if not lc["restrained"]:
            eps = r["disp"]["node:B"][0] / LENGTH
            eps_th = ALPHA * DELTA_T if lc["thermal"] else F(0)
            check("axial compliance identity on the stiffness As",
                  eps - eps_th == (r["N"] - r["P"] * (2 * NU)) / (phys["stiff"]["As"] * E_MOD))
        else:
            eps_th = ALPHA * DELTA_T if lc["thermal"] else F(0)
            check("restrained wall force Nw = 2 nu P - E As alpha dT on the stiffness As",
                  r["N"] == r["P"] * (2 * NU) - phys["stiff"]["As"] * E_MOD * eps_th)


def solve_linear(A, b):
    n = len(A)
    M = [row[:] + [b[i]] for i, row in enumerate(A)]
    for c in range(n):
        piv = next(r for r in range(c, n) if M[r][c] != 0)
        M[c], M[piv] = M[piv], M[c]
        for r in range(n):
            if r != c and M[r][c] != 0:
                f = M[r][c] / M[c][c]
                M[r] = [x - f * y for x, y in zip(M[r], M[c])]
    return [M[i][n] / M[i][i] for i in range(n)]


def beam_fe(n, L, EI, Ks, tip, w):
    """Cantilever fixed at x = 0 in n equal exact static elements (SHEAR_REFERENCE section 3; Ks None is
    Euler-Bernoulli), tip force and full-span uniform w with the consistent vector [wh/2, wh^2/12, wh/2,
    -wh^2/12] (section 4: identical for both theories on a prismatic member). EI, Ks and w are the
    rational coefficients of pi; the result is (v, theta) at the tip in the same scaling."""
    h = L / n
    phi = F(0) if Ks is None else 12 * EI / (Ks * h * h)
    c = EI / (h ** 3 * (1 + phi))
    k = [[12, 6 * h, -12, 6 * h], [6 * h, (4 + phi) * h * h, -6 * h, (2 - phi) * h * h],
         [-12, -6 * h, 12, -6 * h], [6 * h, (2 - phi) * h * h, -6 * h, (4 + phi) * h * h]]
    k = [[c * F(v) for v in row] for row in k]
    size = 2 * (n + 1)
    K = [[F(0)] * size for _ in range(size)]
    R = [F(0)] * size
    for e in range(n):
        dofs = [2 * e, 2 * e + 1, 2 * e + 2, 2 * e + 3]
        feq = [w * h / 2, w * h * h / 12, w * h / 2, -w * h * h / 12]
        for a in range(4):
            R[dofs[a]] += feq[a]
            for b_ in range(4):
                K[dofs[a]][dofs[b_]] += k[a][b_]
    R[2 * n] += tip
    free = list(range(2, size))
    sol = solve_linear([[K[i][j] for j in free] for i in free], [R[i] for i in free])
    return sol[-2], sol[-1]


# ---------------------------------------------------------------- ledgers, zero scales and criteria
FAMILY_OF_UNIT = {"N": "force", "N*m": "moment", "m": "length", "rad": "rotation"}
UNIT_FACTOR = {("m", "mm"): F(1000), ("Pa", "MPa"): F(1, 1000000)}
AXIAL_FORCE_KINDS = {"pipe_wall_axial_force_v2", "pipe_effective_axial_force_v2", "pipe_wall_endpoint_action_v2", "element_local_axial_force"}
STRESS_GROUP = {"pipe_axial_membrane_stress_v2": "axial", "element_local_axial_normal_stress": "axial",
                "element_local_bending_normal_stress_y": "bending_torsion", "element_local_bending_normal_stress_z": "bending_torsion",
                "element_local_torsional_shear_stress": "bending_torsion", "pipe_lame_radial_stress_v2": "lame", "pipe_lame_hoop_stress_v2": "lame",
                "pipe_elastic_normal_stress_maximum_v2": "maximum"}


def family(identity, ref_unit):
    """Zero-scale family of a row (the groups of T4-I7 and T4-I8): axial and transverse forces, moments,
    axial and transverse translations, rotations, and per basis the axial, bending/torsion, Lame and
    maximum stresses."""
    kind, entity, comp, loc, basis = identity
    if ref_unit == "Pa":
        return f"stress_{STRESS_GROUP[kind]}@{basis}"
    if ref_unit == "N":
        if kind == "support_reaction_force_magnitude_v2":
            return "force_magnitude"
        return "force_axial" if kind in AXIAL_FORCE_KINDS or comp == "Fx" else "force_transverse"
    if ref_unit == "m":
        return "length_axial" if comp == "nodal_displacement_x" else "length_transverse"
    return FAMILY_OF_UNIT[ref_unit]


FALLBACK = {"force_transverse": ["force_axial"], "force_axial": ["force_transverse"], "force_magnitude": ["force_axial", "force_transverse"],
            "length_transverse": ["length_axial"], "length_axial": ["length_transverse"],
            "stress_bending_torsion": ["stress_axial", "stress_maximum"], "stress_axial": ["stress_bending_torsion", "stress_maximum"],
            "stress_lame": ["stress_axial"], "stress_maximum": ["stress_axial"]}


def zero_scales(rows, evidence, r, lc):
    """Per load case and family: max |expected| over the family's rows; if the family is identically zero,
    the scale of its sibling family (FALLBACK: transverse forces take the axial force scale, as T4-I8's
    MILLTOL takes P; bending and torsional stresses take the same basis's axial stress scale, as T4-I8 takes
    P/As); if every row of that unit is zero (a fully restrained member), the characteristic scale named in
    the definition (moments: the axial force scale times L; translations and rotations: the free extension)."""
    members, unit_of = {}, {}
    for ident, v, unit, ref_unit, frame in rows:
        fam = family(ident, ref_unit)
        members.setdefault(fam, []).append(v)
        unit_of[fam] = ref_unit
    out, defs = {}, {}  # the terminal evidence entries take the force scales of the rows
    for fam in sorted(members):
        best = max(members[fam], key=lambda v: abs(v.dec()))
        if not best.zero():
            out[fam] = v_abs(best)
            defs[fam] = f"max |expected| over this load case's {fam} rows"
    for fam in sorted(members):
        if fam in out:
            continue
        group, _, basis = fam.partition("@")
        src = next((c for c in (p + ("@" + basis if basis else "") for p in FALLBACK.get(group, [])) if c in out), None)
        if src:
            out[fam] = out[src]
            defs[fam] = f"identically zero here: scale_{src}"
        elif unit_of[fam] == "N*m":
            out[fam] = out["force_axial"] * LENGTH
            defs[fam] = "identically zero here (no moment in this load case): scale_force_axial times L"
        else:
            assert unit_of[fam] in ("m", "rad") and lc["restrained"], fam
            ext = v_abs(r["free_extension"])
            out[fam] = ext if unit_of[fam] == "m" else ext / LENGTH
            defs[fam] = ("identically zero here (both ends anchored): |free extension| of the same document with node B released, "
                         "L (alpha dT + (1-2nu) pAi/(E As)) with As the stiffness section" + ("" if unit_of[fam] == "m" else ", over L"))
    return out, defs, unit_of


def name_of(ident, v):
    kind, entity, comp, loc, basis = ident
    if v.zero():
        return "zero"
    return f"{entity}|{comp}|{loc}" + (f"|{basis}" if basis else "")


def ledger(base, cid, rows, evidence, scales, scale_units):
    expected, out_rows, ev_rows = {}, [], []
    scale_q = {fam: qty(v, scale_units[fam]) for fam, v in scales.items()}
    for ident, v, unit, ref_unit, frame in rows:
        kind, entity, comp, loc, basis = ident
        fam = family(ident, ref_unit)
        name = name_of(ident, v)
        if v.zero():
            name = f"zero_{FAMILY_OF_UNIT.get(ref_unit, 'stress')}"
        if name not in expected:
            expected[name] = qty(v, ref_unit)
        else:
            assert expected[name]["exact"] == exact_json(v) and expected[name]["unit"] == ref_unit, name
        factor = UNIT_FACTOR.get((ref_unit, unit), F(1))
        origin = {"kind": "analytical", "pointer": f"{base}/expected/{name}", "reference_unit": ref_unit, "transform": "identity"}
        if v.zero():
            s = scales[fam]
            origin["zero_scale"] = {"tag": f"scale_{fam}", "base": {"pointer": f"{base}/zero_scales/scale_{fam}", "reference_unit": scale_units[fam]},
                                    "scale_unit": scale_units[fam]}
            tol = Decimal("1e-9") * abs(s.dec()) * Decimal(factor.numerator) / Decimal(factor.denominator)
            crit = {"kind": "zero_scale", "relative_tolerance": 0.0, "absolute_tolerance": float(tol)}
        else:
            crit = {"kind": "relative", "relative_tolerance": 1e-9, "absolute_tolerance": 0.0}
        entry = {"kind": kind, "entity_ref": entity, "component": comp, "location": loc}
        if basis:
            entry["section_basis"] = basis
        entry.update({"unit": unit, "coordinate_system": frame, "basis_ref": {"ref_type": "load_case", "ref_id": cid},
                      "expected": name, "reference_origin": origin, "criterion": crit})
        out_rows.append(entry)
    for (field, node, comp), v in evidence:
        name = "zero_force" if v.zero() else f"terminal|{node}|{field}|{comp}"
        if name not in expected:
            expected[name] = qty(v, "N")
        e = {"namespace": "contract_evidence.pressure", "basis_ref": {"ref_type": "load_case", "ref_id": cid}, "record": "terminal",
             "key": {"node_ref": node}, "field": field, "component": comp, "definition": {"closure_transfer": "transfers_to_wall"},
             "unit": "N", "expected": name}
        if v.zero():
            fam = "force_axial" if comp == "x" else "force_transverse"
            e["zero_scale"] = {"tag": f"scale_{fam}", "base": {"pointer": f"{base}/zero_scales/scale_{fam}", "reference_unit": "N"}, "scale_unit": "N"}
            e["criterion"] = {"kind": "zero_scale", "relative_tolerance": 0.0, "absolute_tolerance": float(Decimal("1e-9") * abs(scales[fam].dec()))}
        else:
            e["criterion"] = {"kind": "relative", "relative_tolerance": 1e-9, "absolute_tolerance": 0.0}
        ev_rows.append(e)
    keys = [tuple(r_[k] for k in ("kind", "entity_ref", "component", "location")) + (r_.get("section_basis"),) for r_ in out_rows]
    check(f"{base}: row identities unique", len(keys) == len(set(keys)))
    stress_kinds = {k for k, *_ in PRESSURE_STRESS} | {k for k, *_ in STRESS} | {"element_local_axial_normal_stress", "pipe_elastic_normal_stress_maximum_v2"}
    nom = {k[:4] for k in keys if k[4] == "nominal_section"}
    red = {k[:4] for k in keys if k[4] == "reduced_section"}
    check(f"{base}: every stress row on both bases, no basis elsewhere",
          nom == red and all((k[0] in stress_kinds) == (k[4] is not None) for k in keys))
    return {"expected": expected, "zero_scales": scale_q, "rows": out_rows, "evidence_rows": ev_rows}


def tolerance(v, scale, ref_unit):
    if not v.zero():
        return Decimal("1e-9") * abs(v.dec())
    return Decimal("1e-9") * abs(scale.dec())


# ---------------------------------------------------------------- discriminators
MUTANTS = [
    ("mill_tolerance_in_stiffness", {"stiffness": "mill_only"}, "stiffness section (A, I, J, the eps_p area and the shear section) on t - c_m instead of the nominal wall", None),
    ("allowance_in_stiffness", {"stiffness": "allowance_only"}, "stiffness section on t - c_a instead of the nominal wall", None),
    ("stiffness_on_reduced_section", {"stiffness": "reduced"}, "stiffness section on the reduced wall t_r (both leaks; the retired single section)", None),
    ("cap_area_on_nominal_bore", {"cap_bore": "nominal_bore"}, "Ai for the caps and the Poisson pair from the nominal bore r_i,n instead of the pressure bore", "pressure"),
    ("eps_p_area_on_reduced_section", {"eps_p_area": "reduced"}, "eps_p = (1-2nu) pAi/(E As) formed with the reduced As while the stiffness is E As,n", "pressure"),
    ("poisson_eigenstrain_area_on_reduced_section", {"poisson_area": "reduced"}, "the Poisson eigenstrain -2 nu pAi/(E As) formed with the reduced As while the stiffness is E As,n", "pressure"),
    ("contents_on_nominal_bore", {"contents_bore": "nominal_bore"}, "contents mass on the nominal bore pi r_i,n^2 instead of the pressure bore", "weight"),
    ("metal_mass_on_reduced_wall", {"metal": "reduced"}, "pipe metal mass on the reduced wall instead of the nominal wall", "weight"),
    ("reduced_rows_copied_from_nominal", {"copy": "reduced_from_nominal"}, "one basis for both families: every reduced-section stress row and maximum equals the nominal one", None),
    ("nominal_rows_copied_from_reduced", {"copy": "nominal_from_reduced"}, "one basis for both families: every nominal-section stress row and maximum equals the reduced one (the retired single family)", None),
    ("allowance_omitted_from_reduced_section", {"reduced_section": "mill_only", "cap_bore": "mill_only_bore", "contents_bore": "mill_only_bore"},
     "the allowance omitted from the reduced wall and from the pressure bore (a mill-only bore, OD/2 - (t - c_m)), in stresses, caps and contents", None),
    ("nominal_lame_at_pressure_bore", {"nominal_lame_at_pressure_bore": True}, "the nominal-basis Lame rows formed with the pressure bore (2 p pi r_i,r^2/As,n) instead of the nominal annulus's own solution", "pressure"),
    ("kappa_on_reduced_section", {"shear_section": "reduced"}, "kappa_E and A_s = kappa A formed on the reduced annulus instead of the nominal (stiffness) annulus", TIMO),
    ("shear_deformation_omitted", {"shear_model": EB}, "Euler-Bernoulli deflection published for a timoshenko_annulus_energy_v1 document", TIMO),
    ("shear_deformation_applied_under_euler_bernoulli", {"shear_model": TIMO}, "Timoshenko deflection published for an euler_bernoulli document", EB),
]
LISTED = 16
MIN_DISTANCE = Decimal(1000)


def discriminator(cid_base, phys_ok, lc, mut, ok_rows, scales):
    _, coincident, shear = cid_base
    phys_bad = physics(coincident, shear, mut)
    bad_rows, _, _, _ = rows_of(phys_bad, lc)
    assert [x[0] for x in bad_rows] == [x[0] for x in ok_rows]
    moved = []
    for (ident, v, unit, ref_unit, frame), (_, wv, *_rest) in zip(ok_rows, bad_rows):
        tol = tolerance(v, scales[family(ident, ref_unit)], ref_unit)
        dist = abs(wv.dec() - v.dec()) / tol
        if dist >= MIN_DISTANCE:
            moved.append((dist, ident, wv, ref_unit))
    return moved


def listed_rows(moved):
    best = {}
    for dist, ident, wv, ref_unit in moved:
        key = (ident[0], ident[2], ident[4])
        if key not in best or dist > best[key][0]:
            best[key] = (dist, ident, wv, ref_unit)
    chosen = sorted(best.values(), key=lambda x: -x[0])[:LISTED]
    out = []
    for dist, (kind, entity, comp, loc, basis), wv, ref_unit in chosen:
        row = {"kind": kind, "entity_ref": entity, "component": comp, "location": loc}
        if basis:
            row["section_basis"] = basis
        row.update({"wrong": qty(wv, ref_unit), "distance_in_tolerances": format(dist, ".3e")})
        out.append(row)
    return out


# ---------------------------------------------------------------- documents (model 0.5.0, current exact contract)
def node(id_, x):
    return {"id": id_, "position": {"x": float(x), "y": 0.0, "z": 0.0}, "provenance": PROV}


def anchor(id_, node_ref):
    return {"id": id_, "node": node_ref, "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROV}


def section_doc(coincident, densities=False):
    sec = {"outside_diameter": {"value": float(OD), "unit": "m"}, "wall_thickness": {"value": float(WALL), "unit": "m"}}
    if not coincident:
        sec["mill_tolerance"] = {"value": float(MILL), "unit": "m"}
    sec["corrosion_allowance"] = {"value": 0.0 if coincident else float(ALLOWANCE), "unit": "m"}
    if densities:
        sec["material_density"] = {"value": float(RHO_PIPE), "unit": "kg/m^3"}
        sec["contents_density"] = {"value": float(RHO_CONTENTS), "unit": "kg/m^3"}
    return sec


def self_weight_binary64(coincident):
    """The authored generation record's binary64 operands: the D-5 rule (Design section 1) in its natural
    order from the binary64 leaves, metal pi*t*(OD - t) on the nominal wall and contents pi*r_i,r*r_i,r on the
    pressure bore (r_i,r in Design section 1's order); mass = rho_pipe*A_metal + rho_contents*A_contents;
    magnitude = mass*g with g signed along global Z. These are request inputs, not frozen values: the frozen rows
    use the exact intensity w, and the binary64 rounding of the inputs is absorbed by the criterion."""
    D, t = float(OD), float(WALL)
    m_, a_ = (0.0, 0.0) if coincident else (float(MILL), float(ALLOWANCE))
    ri_r = D / 2 - ((t - m_) - a_)
    metal = math.pi * t * (D - t)
    contents = math.pi * ri_r * ri_r
    mass = float(RHO_PIPE) * metal + float(RHO_CONTENTS) * contents
    return mass, mass * -float(GRAVITY)


def self_weight_load(cid, coincident):
    """One generated self-weight primitive on the pipe in load case cid, in the model format's spelling."""
    mass, magnitude = self_weight_binary64(coincident)
    lid = f"load:self-weight:{len(cid)}:{cid}:{len(PIPE)}:{PIPE}"
    payload = {"id": lid, "category": "distributed_force", "target": {"type": "element", "pipe": PIPE}, "direction": GRAVITY_AXIS,
               "magnitude": {"value": magnitude, "unit": "N/m"}, "dimension": "force_per_length"}
    sec = section_doc(coincident, densities=True)
    inputs = {"outside_diameter": sec["outside_diameter"], "wall_thickness": sec["wall_thickness"], "mill_tolerance": sec.get("mill_tolerance"),
              "material_density": sec["material_density"], "contents_density": sec["contents_density"],
              "insulation_thickness": None, "insulation_density": None, "corrosion_allowance": sec["corrosion_allowance"]}
    record = {"method": SELF_WEIGHT_METHOD, "mass_method": SELF_WEIGHT_MASS_METHOD,
              "gravity": {"value": -float(GRAVITY), "unit": "m/s^2", "axis": GRAVITY_AXIS}, "normalized_acceleration_m_per_s2": -float(GRAVITY),
              "request_provenance": PROV, "pipe_id": PIPE, "pipe_provenance": PROV, "section_ref": None, "referenced_section": None,
              "mass_inputs": inputs, "mass_kg_per_m": mass, "contents_absent": False, "insulation_absent": True,
              "source_model_hash": UNBOUND_MODEL_HASH, "normalized_dependencies": dict({"section_ref": None}, **inputs),
              "generated_payload": payload}
    load = dict(payload)
    load["provenance"] = json.dumps(record, separators=(",", ":"), ensure_ascii=False, allow_nan=False)
    return load


def case_slug(cid):
    return cid[len("case:"):]


def load_case_doc(cid, lc, coincident=False):
    """Every primitive-load id names its load case (load:<kind>:<case slug>:<axis or pipe>; the generated self-weight's
    own spelling names it too), so the ids are unique across the whole model, as the product requires."""
    loads = []
    slug = case_slug(cid)
    if lc["tip"]:
        comps = [(AXIAL_FORCE if lc["axial"] else F(0)), TIP_FORCE[1], TIP_FORCE[2]]
        for axis, v in zip(("global_x", "global_y", "global_z"), comps):
            if v != 0:
                loads.append({"id": f"load:tip-force:{slug}:{axis}", "category": "concentrated_force", "target": {"type": "node", "node": "node:B"},
                              "direction": axis, "magnitude": {"value": float(v), "unit": "N"}, "dimension": "force", "provenance": PROV})
        loads.append({"id": f"load:tip-torque:{slug}:rotation_x", "category": "concentrated_moment", "target": {"type": "node", "node": "node:B"},
                      "direction": "rotation_x", "magnitude": {"value": float(TIP_MOMENT[0]), "unit": "N*m"}, "dimension": "moment", "provenance": PROV})
    if lc["weight"]:
        loads.append(self_weight_load(cid, coincident))
    if lc["thermal"]:
        loads.append({"id": f"load:thermal:{slug}:{PIPE}", "category": "thermal", "target": {"type": "element", "pipe": PIPE}, "direction": "global_x",
                      "magnitude": {"value": float(DELTA_T), "unit": "degC"}, "dimension": "temperature_interval", "provenance": PROV})
    regions = []
    if lc["pressure"]:
        regions.append({"id": "region:" + cid[len("case:"):], "member_pipe_ids": [PIPE], "pressure_basis": "internal_differential_zero_external_v1",
                        "pressure": {"value": float(PRESSURE), "unit": "Pa"},
                        "terminals": [{"node_ref": "node:A", "closure_transfer": "transfers_to_wall", "provenance": PROV},
                                      {"node_ref": "node:B", "closure_transfer": "transfers_to_wall", "provenance": PROV}],
                        "provenance": PROV})
    return {"id": cid, "primitive_loads": loads, "pressure_regions": regions, "provenance": PROV}


def request_doc(project, coincident, shear, restrained, thermal, cases, densities=False):
    material = {"id": "material:u6", "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
                "elastic_modulus": {"value": float(E_MOD), "unit": "Pa"}, "poisson_ratio": {"value": float(NU), "unit": "1"}}
    if thermal:
        material["thermal_expansion_coefficient"] = {"value": float(ALPHA), "unit": "1/degC"}
    material["provenance"] = PROV
    supports = [anchor("support:A", "node:A")] + ([anchor("support:B", "node:B")] if restrained else [])
    model = {"schema_version": "0.5.0", "document_kind": "openpipestress.product_preview.model",
             "pressure_contract": {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": shear},
             "project": {"id": project, "units": UNITS},
             "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing",
                                 "professional_acceptance": "not_provided"},
             "nodes": [node("node:A", 0), node("node:B", LENGTH)],
             "pipe_segments": [{"id": PIPE, "from": "node:A", "to": "node:B", "section": section_doc(coincident, densities), "material": "material:u6",
                                "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0}, "provenance": PROV}],
             "supports": supports, "components": [], "materials": [material],
             "load_cases": [load_case_doc(cid, LOAD_CASES[cid], coincident) for cid in cases], "combinations": []}
    return {"model": model, "materials": []}


def without_ids(items):
    return [{k: v for k, v in item.items() if k != "id"} for item in items]


def check_primitive_load_ids(name, doc):
    """The product requires primitive-load ids unique across the whole model; each authored id names its case."""
    ids = [l["id"] for c in doc["model"]["load_cases"] for l in c["primitive_loads"]]
    check(f"{name}: primitive-load ids unique across the whole model", len(ids) == len(set(ids)))
    check(f"{name}: every primitive-load id names its load case",
          all(l["id"].startswith(("load:tip-force:" + case_slug(c["id"]) + ":", "load:tip-torque:" + case_slug(c["id"]) + ":",
                                  "load:thermal:" + case_slug(c["id"]) + ":",
                                  f"load:self-weight:{len(c['id'])}:{c['id']}:"))
              for c in doc["model"]["load_cases"] for l in c["primitive_loads"]))
    sources = [s["source_ref"] for c in doc["model"]["load_cases"] for s in c.get("analysis_state", {}).get("load_sources", [])]
    check(f"{name}: every load_sources entry names a primitive load of its own case",
          all({s["source_ref"] for s in c.get("analysis_state", {}).get("load_sources", [])} <= {l["id"] for l in c["primitive_loads"]}
              for c in doc["model"]["load_cases"]) and len(sources) == len(set(sources)))


def check_self_weight_request(plain, weighted, coincident, w_exact):
    """The generated-self-weight request is the variant's request with the densities on its section and the
    generated load: same geometry, material, supports and contract; case:combined is case:pressure-tip plus the
    generated load; the generation record is self-consistent and its intensity is the frozen w to binary64."""
    p0, g0 = plain["model"], weighted["model"]
    check("self-weight request: same contract, nodes, supports, materials and status as the variant's request",
          all(p0[k] == g0[k] for k in ("schema_version", "document_kind", "pressure_contract", "analysis_status", "nodes", "supports", "components",
                                         "materials", "combinations")))
    ps, gs = p0["pipe_segments"][0], g0["pipe_segments"][0]
    check("self-weight request: the section is the variant's section plus material_density and contents_density",
          {k: v for k, v in gs.items() if k != "section"} == {k: v for k, v in ps.items() if k != "section"}
          and {k: v for k, v in gs["section"].items() if k not in ("material_density", "contents_density")} == ps["section"]
          and gs["section"]["material_density"] == {"value": float(RHO_PIPE), "unit": "kg/m^3"}
          and gs["section"]["contents_density"] == {"value": float(RHO_CONTENTS), "unit": "kg/m^3"})
    cases = {c["id"]: c for c in g0["load_cases"]}
    plain_cases = {c["id"]: c for c in p0["load_cases"]}
    for cid, case in cases.items():
        generated = [l for l in case["primitive_loads"] if l["id"].startswith("load:self-weight:")]
        check(f"self-weight request {cid}: exactly one generated self-weight load", len(generated) == 1)
        load = generated[0]
        record = json.loads(load["provenance"])
        sec = gs["section"]
        check(f"self-weight request {cid}: load id is load:self-weight:<len>:<case>:<len>:<pipe>",
              load["id"] == f"load:self-weight:{len(cid)}:{cid}:{len(PIPE)}:{PIPE}")
        check(f"self-weight request {cid}: the generated payload is the load itself",
              record["generated_payload"] == {k: v for k, v in load.items() if k != "provenance"})
        check(f"self-weight request {cid}: mass inputs and dependencies are the section's fields",
              all(record["mass_inputs"][k] == sec.get(k) for k in ("outside_diameter", "wall_thickness", "mill_tolerance", "material_density",
                                                                   "contents_density", "corrosion_allowance"))
              and record["mass_inputs"]["insulation_thickness"] is None and record["mass_inputs"]["insulation_density"] is None
              and record["normalized_dependencies"] == dict({"section_ref": None}, **record["mass_inputs"]))
        check(f"self-weight request {cid}: gravity -g along global Z; magnitude = mass * g in binary64",
              record["gravity"] == {"value": -float(GRAVITY), "unit": "m/s^2", "axis": "global_z"} and load["direction"] == "global_z"
              and record["normalized_acceleration_m_per_s2"] == -float(GRAVITY)
              and load["magnitude"]["value"] == record["mass_kg_per_m"] * -float(GRAVITY))
        rel = abs(Decimal(-load["magnitude"]["value"]) - w_exact.dec()) / w_exact.dec()
        check(f"self-weight request {cid}: the authored intensity is the frozen w within 2^-50 relative", rel < Decimal(2) ** -50)
        mass_exact = RHO_PIPE * annulus(OD, WALL)["As"] + RHO_CONTENTS * d5_sections(*((F(0), F(0)) if coincident else (MILL, ALLOWANCE)))[1]["Ai"]
        check(f"self-weight request {cid}: frozen w == g (rho_pipe As,n + rho_contents pi r_i,r^2)", mass_exact * GRAVITY == w_exact)
        rest = [l for l in case["primitive_loads"] if l is not load]
        if cid == "case:combined":
            ref = plain_cases.get("case:pressure-tip")
            check("self-weight request case:combined: the tip loads (each under its own case's id) and the pressure region of case:pressure-tip",
                  ref is not None and without_ids(rest) == without_ids(ref["primitive_loads"])
                  and [l["id"] for l in rest] == [l["id"].replace(":pressure-tip:", ":combined:", 1) for l in ref["primitive_loads"]]
                  and without_ids(case["pressure_regions"]) == without_ids(ref["pressure_regions"]))
        else:
            check(f"self-weight request {cid}: the generated load alone, pressure regions explicitly empty",
                  rest == [] and case["pressure_regions"] == [])


def request_doc_load_state_group(plain):
    """The same document with the load/reference-state group (B7b: 0.4.0's rules inside model 0.5.0), formed
    as T4-I8's to_040 forms it: an installed reference configuration with a direct strain reference per
    pipe; the material's expansion coefficient and every thermal primitive moved into each case's element
    thermal_state (constant_alpha_interval, engineering interval); every remaining primitive listed in
    load_sources with factor 1; every support an active model device."""
    doc = json.loads(json.dumps(plain))
    m = doc["model"]
    m["reference_configurations"] = [{"id": "reference:installed", "label": "Installed reference",
                                      "geometry_ref": {"kind": "authored_model_geometry"},
                                      "member_references": [{"pipe_ref": p["id"], "basis": {"kind": "direct_strain_reference"},
                                                             "fit": {"kind": "none"}, "provenance": PROV} for p in m["pipe_segments"]],
                                      "provenance": PROV}]
    alpha = {mat["id"]: mat.pop("thermal_expansion_coefficient", None) for mat in m["materials"]}
    for case in m["load_cases"]:
        thermal = {l["target"]["pipe"]: l["magnitude"] for l in case["primitive_loads"] if l["category"] == "thermal"}
        case["primitive_loads"] = [l for l in case["primitive_loads"] if l["category"] != "thermal"]
        states = []
        for p in m["pipe_segments"]:
            if p["id"] in thermal:
                ts = {"kind": "constant_alpha_interval", "coefficient": alpha[p["material"]], "temperature_change": thermal[p["id"]],
                      "coefficient_meaning": "engineering_interval", "provenance": PROV}
            else:
                ts = {"kind": "unchanged_reference", "provenance": PROV}
            states.append({"pipe_ref": p["id"], "material_selection": {"kind": "explicit_base_properties", "material_ref": p["material"],
                                                                        "applicability_reference": "invented analytical basis declared applicable (T4-U6)"},
                           "thermal_state": ts})
        case["analysis_state"] = {"contract": "openpipestress.load_reference_state/1.0.0", "reference_configuration_ref": "reference:installed",
                                  "element_states": states,
                                  "support_states": [{"support_ref": s["id"], "participation": {"kind": "active_model_device"}} for s in m["supports"]],
                                  "load_sources": [{"source_ref": l["id"], "factor": 1.0} for l in case["primitive_loads"]],
                                  "history": {"kind": "independent_equilibrium"}, "provenance": PROV}
    p0, g0 = plain["model"], m
    check("load/reference-state twin: same nodes, pipes, supports and pressure regions",
          all(p0[k] == g0[k] for k in ("nodes", "pipe_segments", "supports"))
          and [c["pressure_regions"] for c in p0["load_cases"]] == [c["pressure_regions"] for c in g0["load_cases"]])
    check("load/reference-state twin: each thermal primitive becomes alpha dT in its case's element state",
          all({l["target"]["pipe"]: l["magnitude"]["value"] for l in cp["primitive_loads"] if l["category"] == "thermal"}
              == {e["pipe_ref"]: e["thermal_state"]["temperature_change"]["value"] for e in cg["analysis_state"]["element_states"]
                  if e["thermal_state"]["kind"] == "constant_alpha_interval"}
              for cp, cg in zip(p0["load_cases"], g0["load_cases"]))
          and all(e["thermal_state"]["coefficient"]["value"] == float(ALPHA) for c in g0["load_cases"] for e in c["analysis_state"]["element_states"]
                  if e["thermal_state"]["kind"] == "constant_alpha_interval"))
    return doc


def dump(value):
    return (json.dumps(value, indent=1, ensure_ascii=False, allow_nan=False) + "\n").encode()


# ---------------------------------------------------------------- the cases
CASES = [
    # (case id, coincident, restrained, variants: [(shear model, request, load/reference-state-group twin or None,
    #  generated-self-weight request or None)], the request's load cases, the generated-self-weight request's load cases)
    ("U6-BASES-001", False, False, [(EB, "U6-BASES-001-EB", None, "U6-BASES-001-EB-SELF-WEIGHT"),
                                    (TIMO, "U6-BASES-001-TIMO", None, "U6-BASES-001-TIMO-SELF-WEIGHT")],
     ["case:pressure-tip", "case:axial-tip"], ["case:self-weight", "case:combined"]),
    ("U6-BASES-002", False, True, [(EB, "U6-BASES-002", "U6-BASES-002-GROUP", None), (TIMO, "U6-BASES-002-TIMO", None, None)],
     ["case:restrained-pressure", "case:restrained-pressure-thermal"], []),
    ("U6-BASES-COINCIDENT", True, False, [(EB, "U6-BASES-COINCIDENT", None, None)], ["case:pressure-tip", "case:axial-tip"], []),
]
DESCRIPTION = {
    "U6-BASES-001": "A pressurized straight cantilever (anchored at A, free at B, L = 2 m) with mill tolerance 0.00125 m and corrosion allowance 0.002 m on OD 0.2 m, wall 0.01 m: the same model on each basis. Displacements on the nominal stiffness; reactions; the pressure thrust on the pressure bore; every stress row on the nominal and the reduced section with per-basis maxima. Euler-Bernoulli and Timoshenko (timoshenko_annulus_energy_v1, kappa_E from the nominal annulus) documents. The self-weight load cases are in a second request per variant, whose section carries the pipe metal and contents densities and whose load cases carry the generated self-weight.",
    "U6-BASES-002": "The same pipe anchored at both ends: the restrained wall force Nw = 2 nu P - E As alpha dT exercises the eps_p area as the stiffness As (Nw does not depend on As under pressure alone) and the thermal eigenload on the nominal A. No transverse response, so the rows are the same under timoshenko_annulus_energy_v1.",
    "U6-BASES-COINCIDENT": "U6-BASES-001's Euler-Bernoulli runnable load cases with c_m = c_a = 0 (mill tolerance absent, corrosion allowance 0): the two families coincide and both are published (H4), with one expected value per row.",
}


def section_evidence(phys):
    def block(s):
        return {"wall_m": qty(s["wall"], "m"), "ri_m": qty(s["ri"], "m"), "As_m2": qty(s["As"], "m^2"), "I_m4": qty(s["I"], "m^4"),
                "J_m4": qty(s["J"], "m^4"), "Z_m3": qty(s["Z"], "m^3")}
    bt = {"model": phys["shear_model"]}
    if phys["shear_model"] == TIMO:
        bt.update({"shear_factor": qty(phys["kappa"], "1"), "shear_area_m2": qty(phys["shear_area"], "m^2")})
    return {"pipe_id": PIPE, "geometry_basis": "authored_od_wall_d5_sections_v1", "outside_diameter_m": qty(OD, "m"), "ro_m": qty(OD / 2, "m"),
            "wall_thickness_m": qty(WALL, "m"), "mill_tolerance_m": qty(phys["mill"], "m"), "corrosion_allowance_m": qty(phys["allowance"], "m"),
            "nominal_section": block(phys["nominal"]), "reduced_section": block(phys["reduced"]),
            "pressure_bore": {"ri_m": qty(phys["reduced"]["ri"], "m"), "Ai_m2": qty(phys["reduced"]["Ai"], "m^2")}, "beam_theory": bt}


def shear_factor_exhibit(phys):
    """kappa evaluated in binary64 in the Design section 3.1 order from the binary64 leaves, for the evidence
    readers; kept outside the pipe_sections evidence object, which has exactly the Design's keys."""
    ri, ro = float(OD) / 2 - float(WALL), float(OD) / 2
    kb = kappa_binary64(ri, ro, float(NU))
    return {"value": kb, "hex": kb.hex(), "from": "the Design section 3.1 operation order on the binary64 leaves r_i,n = OD/2 - t, r_o = OD/2, nu",
            "relative_difference_from_exact": format(float(abs(F(kb) - phys["kappa"]) / phys["kappa"]), ".3e")}


EVIDENCE_KEYS = ["pipe_id", "geometry_basis", "outside_diameter_m", "ro_m", "wall_thickness_m", "mill_tolerance_m", "corrosion_allowance_m",
                 "nominal_section", "reduced_section", "pressure_bore", "beam_theory"]


def build():
    files = {}
    docs = {}
    oracle_cases = {}
    disc_count = 0
    for case_id, coincident, restrained, variants, runnable, weighted in CASES:
        case_out = {"description": DESCRIPTION[case_id], "variants": {}, "wrong_result_discriminators": []}
        for shear, request, group_request, weight_request in variants:
            phys = physics(coincident, shear)
            base = f"/cases/{case_id}/variants/{shear}"
            evidence = section_evidence(phys)
            check("pipe_sections evidence has exactly the Design section 2.5 keys, in order", list(evidence) == EVIDENCE_KEYS
                  and all(list(evidence[s]) == ["wall_m", "ri_m", "As_m2", "I_m4", "J_m4", "Z_m3"] for s in ("nominal_section", "reduced_section"))
                  and list(evidence["pressure_bore"]) == ["ri_m", "Ai_m2"]
                  and list(evidence["beam_theory"]) == (["model", "shear_factor", "shear_area_m2"] if shear == TIMO else ["model"]))
            var = {"request": None, "pipe_sections_evidence": evidence, "load_cases": {}}
            if shear == TIMO:
                var["binary64_exhibits"] = {"shear_factor": shear_factor_exhibit(phys)}
            if request:
                doc = request_doc(f"project:invented-t4-u6-{request.lower()}", coincident, shear, restrained,
                                  any(LOAD_CASES[c]["thermal"] for c in runnable), runnable)
                check_primitive_load_ids(request, doc)
                docs[request] = doc
                data = dump(doc)
                files[f"requests/{request}.request.json"] = data
                var["request"] = {"path": f"requests/{request}.request.json", "sha256": hashlib.sha256(data).hexdigest(), "load_cases": list(runnable)}
                if restrained and shear == TIMO:
                    var["request"]["note"] = ("the euler_bernoulli request with shear_model timoshenko_annulus_energy_v1 (see shear_model_note): "
                                              "its rows, zero scales and discriminators equal the euler_bernoulli variant's, and its "
                                              "pipe_sections evidence equals U6-BASES-001's timoshenko_annulus_energy_v1 evidence (checked)")
            if group_request:
                gdoc = request_doc_load_state_group(doc)
                gdoc["model"]["project"]["id"] = f"project:invented-t4-u6-{group_request.lower()}"
                check_primitive_load_ids(group_request, gdoc)
                gdata = dump(gdoc)
                files[f"requests/{group_request}.request.json"] = gdata
                var["request_load_state_group"] = {
                    "path": f"requests/{group_request}.request.json", "sha256": hashlib.sha256(gdata).hexdigest(), "load_cases": list(runnable),
                    "note": "the same physical inputs with the load/reference-state group (thermal as each case's element thermal_state); judged against the same rows"}
            if weight_request:
                wdoc = request_doc(f"project:invented-t4-u6-{weight_request.lower()}", coincident, shear, restrained,
                                   any(LOAD_CASES[c]["thermal"] for c in weighted), weighted, densities=True)
                check_self_weight_request(doc, wdoc, coincident, phys["w"])
                check_primitive_load_ids(weight_request, wdoc)
                wdata = dump(wdoc)
                files[f"requests/{weight_request}.request.json"] = wdata
                mass, magnitude = self_weight_binary64(coincident)
                var["request_self_weight"] = {
                    "path": f"requests/{weight_request}.request.json", "sha256": hashlib.sha256(wdata).hexdigest(), "load_cases": list(weighted),
                    "note": ("the variant's request with material_density and contents_density on the section and, in each load case, the generated "
                             "self-weight as a distributed_force primitive whose provenance is the generation record; case:combined is "
                             "case:pressure-tip plus that load"),
                    "generated_self_weight": {
                        "method": SELF_WEIGHT_METHOD, "mass_method": SELF_WEIGHT_MASS_METHOD,
                        "binary64_order": ("metal pi*t*(OD - t) on the nominal wall; contents pi*r_i,r*r_i,r on the pressure bore, "
                                          "r_i,r = OD/2 - ((t - c_m) - c_a); mass = rho_pipe*metal + rho_contents*contents; magnitude = mass*g, g = -9.80665 along global Z"),
                        "mass_kg_per_m": mass, "magnitude_n_per_m": magnitude,
                        "relative_difference_from_exact_w": format(float(abs(Decimal(-magnitude) - phys["w"].dec()) / phys["w"].dec()), ".3e"),
                        "source_model_hash": "all zeros, as in the prepared example document: the authored record binds no source model hash",
                        "spelling": "from a prepared exact 0.5.0 self-weight input document (README, generated self-weight)"}}
            for cid in runnable + weighted:
                lc = LOAD_CASES[cid]
                rows, evidence, r, cuts = rows_of(phys, lc)
                second_method_checks(phys, lc, r, cuts)
                scales, defs, units = zero_scales(rows, evidence, r, lc)
                led = ledger(f"{base}/load_cases/{cid}", cid, rows, evidence, scales, units)
                led["zero_scale_definitions"] = {f"scale_{k}": v for k, v in defs.items()}
                led["zero_scales"] = {f"scale_{k}": v for k, v in led["zero_scales"].items()}
                for row in led["rows"]:
                    if "zero_scale" in row["reference_origin"]:
                        assert row["reference_origin"]["zero_scale"]["tag"] in led["zero_scales"]
                entry = {"description": lc["description"],
                         "request_status": "authored" if cid in runnable else "authored in the generated-self-weight request (request_self_weight)",
                         "absent_row_kinds": (["element_local_axial_force", "element_local_axial_normal_stress", "pipe_section_pressure_hoop_stress",
                                               "pipe_section_pressure_longitudinal_stress"] if lc["pressure"] else
                                              ["pipe_wall_axial_force_v2", "pipe_effective_axial_force_v2", "pipe_axial_membrane_stress_v2",
                                               "pipe_lame_radial_stress_v2", "pipe_lame_hoop_stress_v2", "pipe_wall_endpoint_action_v2"])}
                if lc["weight"]:
                    entry["self_weight"] = {"pipe_metal_density": qty(RHO_PIPE, "kg/m^3"), "contents_density": qty(RHO_CONTENTS, "kg/m^3"),
                                            "gravity": qty(GRAVITY, "m/s^2"), "gravity_direction": "global -Z", "insulation": "none",
                                            "rule": "w = g (rho_pipe As,n + rho_contents pi r_i,r^2): metal on the nominal wall, contents on the pressure bore (D-5, H4)",
                                            "intensity": qty(r["w"], "N/m")}
                if lc["pressure"]:
                    entry["pressure_thrust"] = {"P": qty(r["P"], "N"), "rule": "P = p pi r_i,r^2 on the pressure bore"}
                entry.update(led)
                var["load_cases"][cid] = entry
                # discriminators of this variant and load case
                for mid, mut, why, scope in MUTANTS:
                    if scope == "pressure" and not lc["pressure"] or scope == "weight" and not lc["weight"] or scope in (EB, TIMO) and scope != shear:
                        continue
                    if coincident:
                        continue  # every D-5 mapping coincides when c_m = c_a = 0
                    moved = discriminator((case_id, coincident, shear), phys, lc, mut, rows, scales)
                    if not moved:
                        continue
                    listed = listed_rows(moved)
                    case_out["wrong_result_discriminators"].append({
                        "id": mid, "variant": shear, "load_case": cid, "why": why, "rows_moved": len(moved),
                        "max_distance_in_tolerances": format(max(m[0] for m in moved), ".3e"),
                        "min_listed_distance_in_tolerances": format(min(Decimal(x["distance_in_tolerances"]) for x in listed), ".3e"),
                        "listed_rows": listed})
                    disc_count += 1
            case_out["variants"][shear] = var
        if restrained:
            case_out["shear_model_note"] = "the rows are the same under timoshenko_annulus_energy_v1: no transverse response, and the axial and torsional stiffness do not involve the shear area"
        oracle_cases[case_id] = case_out
    # every named mutant is exercised somewhere
    seen = {d["id"] for c in oracle_cases.values() for d in c["wrong_result_discriminators"]}
    check("every discriminator moves at least one row by >= 1e3 tolerances", seen == {m[0] for m in MUTANTS})
    # the restrained wall force does not move with the stiffness section under pressure alone (RV1 N-5)
    d002 = [d for d in oracle_cases["U6-BASES-002"]["wrong_result_discriminators"] if d["load_case"] == "case:restrained-pressure"]
    check("U6-BASES-002 pressure only: stiffness-section mutants do not move the restrained wall force",
          not any(d["id"] in ("mill_tolerance_in_stiffness", "allowance_in_stiffness", "stiffness_on_reduced_section") for d in d002))
    check("U6-BASES-002 pressure only: the eps_p and Poisson area mutants move it",
          {"eps_p_area_on_reduced_section", "poisson_eigenstrain_area_on_reduced_section"} <= {d["id"] for d in d002})
    check("U6 kappa_E of the nominal annulus is SHEAR_REFERENCE S-B's 33.219654/66.137512",
          physics(False, TIMO)["kappa"] == F("33.219654") / F("66.137512"))
    # U6-BASES-002-TIMO binds the restrained case's frozen rows under Timoshenko: no new value
    c002, c001 = oracle_cases["U6-BASES-002"], oracle_cases["U6-BASES-001"]
    v_eb, v_timo = c002["variants"][EB], c002["variants"][TIMO]
    eb_base, timo_base = f"/cases/U6-BASES-002/variants/{EB}/", f"/cases/U6-BASES-002/variants/{TIMO}/"
    check("U6-BASES-002-TIMO: its load cases (rows, expected values, zero scales, terminal evidence) are the euler_bernoulli variant's",
          json.dumps(v_timo["load_cases"]) == json.dumps(v_eb["load_cases"]).replace(eb_base, timo_base))
    check("U6-BASES-002-TIMO: its pipe_sections evidence and binary64 exhibit are U6-BASES-001's timoshenko variant's",
          v_timo["pipe_sections_evidence"] == c001["variants"][TIMO]["pipe_sections_evidence"]
          and v_timo["binary64_exhibits"] == c001["variants"][TIMO]["binary64_exhibits"])
    d_eb = [d for d in c002["wrong_result_discriminators"] if d["variant"] == EB]
    d_timo = [d for d in c002["wrong_result_discriminators"] if d["variant"] == TIMO]
    check("U6-BASES-002-TIMO: its discriminators are the euler_bernoulli variant's, entry for entry",
          bool(d_eb) and [dict(d, variant=EB) for d in d_timo] == d_eb)
    p_eb, p_timo = docs["U6-BASES-002"]["model"], docs["U6-BASES-002-TIMO"]["model"]
    check("U6-BASES-002-TIMO: the U6-BASES-002 document apart from shear_model and the project id",
          {k: v for k, v in p_timo.items() if k not in ("pressure_contract", "project")} == {k: v for k, v in p_eb.items() if k not in ("pressure_contract", "project")}
          and dict(p_timo["pressure_contract"], shear_model=EB) == p_eb["pressure_contract"] and p_timo["pressure_contract"]["shear_model"] == TIMO
          and dict(p_timo["project"], id=p_eb["project"]["id"]) == p_eb["project"])
    oracle = header()
    oracle["section_geometry"] = {
        "dual": geometry_block(False), "coincident": geometry_block(True)}
    oracle["cases"] = oracle_cases
    rows_total = sum(len(lc["rows"]) for c in oracle_cases.values() for v in c["variants"].values() for lc in v["load_cases"].values())
    oracle["counts"] = {"load_case_evaluations": sum(len(v["load_cases"]) for c in oracle_cases.values() for v in c["variants"].values()),
                        "rows": rows_total, "discriminators": disc_count}
    files["oracle.json"] = dump(oracle)
    return files, oracle


def geometry_block(coincident):
    mill, allowance = (F(0), F(0)) if coincident else (MILL, ALLOWANCE)
    nominal, reduced, t_r = d5_sections(mill, allowance)
    return {"inputs": {"outside_diameter": qty(OD, "m"), "wall_thickness": qty(WALL, "m"), "mill_tolerance": qty(mill, "m"),
                       "corrosion_allowance": qty(allowance, "m")},
            "reduced_wall": qty(t_r, "m"),
            "binary64_order": binary64_order(mill, allowance),
            "kappa_E_nominal": qty(kappa_energy(nominal["ri"], nominal["ro"], NU), "1"),
            "kappa_E_reduced_discriminator_only": qty(kappa_energy(reduced["ri"], reduced["ro"], NU), "1")}


def header():
    return {
        "schema_version": "independent.section_bases_reference/1.0.0",
        "purpose": "Independent analytical references for T4-U6 (D-5 section bases) and the Timoshenko variant shared with T4-U8: one straight pipe, frozen before any product implementation; not product requests and not observed solver outputs.",
        "status": ("frozen by the T4-U6/T4-U8 independent reference TASK and refuted independently; the refutation's four findings were repaired "
                   "in 6817b72ca9 without a value change, and an independent refuter confirmed all four repairs on 2026-10-16. An "
                   "independent refuter also confirmed the three later changes on 2026-10-16: (a) the primitive-load id repair "
                   "a2d3f27020 (every primitive-load id names its load case; U6-BASES-001-EB, U6-BASES-001-TIMO and U6-BASES-COINCIDENT "
                   "had reused their tip load ids in case:pressure-tip and case:axial-tip, which the product refuses as DUPLICATE_ID) is "
                   "id-only: reverting the new ids gives the old request bytes exactly, and oracle.json changed only in its status and "
                   "six request sha256 bindings; (b) U6-BASES-002-TIMO (2621c1c783), which binds U6-BASES-002's frozen rows to a "
                   "timoshenko_annulus_energy_v1 request, repeats the euler_bernoulli variant's rows, zero scales, evidence rows and 17 "
                   "discriminators, and the case is shear-model independent (both nodes fully anchored, axial loads only); (c) the two "
                   "generated-self-weight requests (request_self_weight) are faithful to the frozen case:self-weight and case:combined "
                   "definitions (densities, metal on the nominal wall, contents on the pressure bore, the authored binary64 order), and "
                   "an independent recomputation of the support reactions, node B u_z and r_y, and root V_z and M_y matches the frozen "
                   "rows to within 1.5e-40 relative. No primitive-load id is duplicated in any request"),
        "basis": {"design": "DEL-05-03 Design section-bases-and-timoshenko.md sections 1, 2.1, 2.5, 3.1, 7 and 8, and section-and-shear-decisions.md (D-5, H4), at b3f4da7661",
                  "shear": "SHEAR_REFERENCE CONTRACT.md sections 1, 3 and 4 and its reference arithmetic (archive f2211ecef6)",
                  "conventions": "validation/references/t4_i8 (the repaired T4-I8 freeze, 5b07d85cf3): row shape, stations, signs, criteria",
                  "independence": "no product code, test or output was read or run"},
        "numbers": "Invented test quantities; decimal inputs are exact rational targets (the 1e-9 criterion absorbs their binary64 rounding); no material-library, component or code-rule data.",
        "numeric_representation": {"exact_is_authoritative": True,
                                   "kinds": ["rational", "rational_times_pi", "rational_over_pi", "pi_polynomial: sum of rational * pi^k",
                                             "symbolic: a square root of a pi_polynomial, alone or added to one (support norms, member maxima), with evaluation"],
                                   "decimal": "40 significant digits of the 130-digit evaluation", "value": "binary64 rounding of the exact value"},
        "d5": {"nominal_section": "wall t: stiffness A, I, J (and the Timoshenko shear area), pipe metal mass, the As of eps_p = (1-2nu) pAi/(E As), nominal-basis stresses",
               "reduced_section": "wall t_r = (t - c_m) - c_a: reduced-basis stresses",
               "pressure_bore": "r_i,r = OD/2 - t_r: Ai for the caps and the Poisson pair, contents mass",
               "each_section": "As = pi w (OD - w); I = As (ro^2 + ri^2)/4; J = 2I; Z = I/ro with its own wall w and ri",
               "stresses": "per basis b: sigma = N/A_b (Nw/A_b on pressurized members), M_y/Z_b, M_z/Z_b, T ro/J_b; Lame of b's own annulus: radial -p and 0, hoop 2p pi r_i,b^2/A_b at ro and that plus p at r_i,b; maximum max |N/A_b| + hypot(M_y, M_z)/Z_b",
               "kappa_E": "Design section 3.1 from the nominal annulus: m = r_i,n/r_o; D = c (1-m^2)^2 + 48 m^2 (1+nu)^2, c = (8nu+14)nu+7; kappa = 6 (1+nu)^2 (1+m^2)^2/D; A_s = kappa A_n; Ks = G A_s, G = E/(2(1+nu))"},
        "sign_conventions": {"frame": "local x = global X (A -> B), y_reference (0,1,0): local y = global Y, local z = global Z",
                             "stations": "end_i, quarter_1, midspan, quarter_3, end_j at 0, 1/4, 1/2, 3/4, 1 of L",
                             "end_rows": "element_local_* force and moment rows and pipe_wall_endpoint_action_v2 at end_i and end_j are node-on-element: end_i = -(j-side cut at 0), end_j = +(cut at L)",
                             "station_rows": "the j-side section action (the action of the part beyond the cut, about the cut point)",
                             "stress_rows": "from the j-side section action at all five locations, ends included; element_local stresses in MPa (reference in Pa)",
                             "supports": "support-on-pipe, global; moment about the attached node; magnitudes are the force and moment norms",
                             "displacements": "global; translations in mm (reference in m), right-hand rotations in rad; a Timoshenko member's rotations are section rotations",
                             "x_z_plane": "v = u_z, theta = -r_y (Design section 3.3)"},
        "row_identity": "(kind, entity_ref, component, location) plus section_basis on every stress row (nominal_section or reduced_section; Design section 2.5); basis_ref names the load case",
        "criteria": {"nonzero": "|observed - expected| <= 1e-9 |expected|",
                     "zero": "|observed| <= 1e-9 |zero_scale| in the row unit (criterion.absolute_tolerance, precomputed); the zero scales are per load case and family (force, moment, length, rotation, and stress per basis)",
                     "negative": "each listed discriminator row: |observed - wrong| > max(absolute_tolerance, 1e-9 max(|observed|, |wrong|)); every listed row lies >= 1e3 tolerances from the reference",
                     "modes": "both solver modes, the same reference", "no_new_threshold": "the existing T4 1e-9 criterion only"},
        "inferred_identities": "element_local_axial_force/axial_force and element_local_axial_normal_stress/axial_normal_stress (case:axial-tip, a member outside every pressure region) follow the sibling kinds' pattern; their exact component spelling and published locations were not readable without product files (README, open items)",
    }


def main(argv):
    files, oracle = build()
    ok = all(c for _, c in CHECKS)
    print(f"checks: {sum(1 for _, c in CHECKS if c)} passed, {sum(1 for _, c in CHECKS if not c)} failed")
    print("counts:", json.dumps(oracle["counts"]))
    if not ok:
        print("RESULT: FAIL")
        return 1
    if "--write" in argv:
        for rel, data in files.items():
            path = ROOT / rel
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(data)
        print("wrote", ", ".join(sorted(files)))
        return 0
    bad = [rel for rel, data in files.items() if not (ROOT / rel).exists() or (ROOT / rel).read_bytes() != data]
    extra = sorted(str(p.relative_to(ROOT)) for p in (ROOT / "requests").glob("*.json") if str(p.relative_to(ROOT)) not in files) if (ROOT / "requests").exists() else []
    if bad or extra:
        print("MISMATCH:", ", ".join(bad + [f"unexpected {x}" for x in extra]))
        return 1
    print(f"RESULT: PASS; {len(files)} files reproduce byte for byte")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
