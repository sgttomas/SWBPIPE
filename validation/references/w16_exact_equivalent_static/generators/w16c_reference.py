#!/usr/bin/env python3
"""W-16 slice c independent references: equivalent-static generation on a cantilever.

Standard-library Python 3 only. This generator imports, reads and runs no
product code and reads no file other than the ones it writes. Every expected
value is derived here in exact rational arithmetic from closed forms of
linear-elastic small-displacement beam theory (DEL-05-01 Design
`exact-equivalent-static.md` section 5):

  seismic   q = mu * k_g * g on the whole span, mu the route's mass per length;
  wind      q_w = p * C * (D + 2 t_i) on the fractions [a, b] of the span.

One invented cantilever along global X, fixed at A (x = 0), free at B (x = L),
L = 5/2 m (L != 1, so a missing length factor shows). The exact route (EX)
forms mu on the D-5 bases: metal on the nominal wall t, contents on the
pressure bore r_p = R - t_r with t_r = (t - c_m) - c_a, insulation over R to
R + t_i; stiffness on the nominal wall. The pressure-free twin (PF), authored
without a corrosion allowance, forms metal and contents on the single wall
t - c_m and its stiffness on that wall too. Their mass difference is the
design's closed form

  mu_EX - mu_PF = pi [rho_m c_m (D - 2t + c_m) + rho_c {2 c_a (R - t + c_m) + c_a^2}],

cross-checked here against the direct difference.

Closed forms (cantilever, load q over x in [aL, bL], EI on the route's stiffness
wall, Timoshenko shear rigidity G A_s with the energy-matched annular factor
kappa_E):

  |R| = q L (b - a);  |M_0| = q L^2 (b^2 - a^2) / 2;
  u_L = q L^4 [(b^3 - a^3) - (b^4 - a^4)/4] / (6 E I) (+ q L^2 / (2 G A_s), full span, Timoshenko);
  theta_L = q L^3 (b^3 - a^3) / (6 E I) (section rotation; no shear term);
  V(x) = q (bL - max(x, aL))^+;  M(x) = q [((bL - x)^+)^2 - ((aL - x)^+)^2] / 2;
  bending stress M / Z on each published section basis (Z = I / R).

Every closed form is cross-checked against an independent route: unit-load
member integration of M(x) and V(x) by Simpson's rule on each interval between
load breakpoints (exact for these cubic integrands), and load resultants by
direct integration. Every result is q * pi**k with q rational: forces and
moments carry k = 1 (mu and I are rational multiples of pi), translations,
rotations and stresses k = 0. Decimals are printed to 40 significant digits
from a Machin value of pi; binary64 values are the correctly rounded nearest
doubles, decided by lower and upper rational bounds of pi.

Modes:
  python3 -I w16c_reference.py           check: rebuild every output in memory and
                                          compare byte for byte with the committed
                                          files, SHA256SUMS included
  python3 -I w16c_reference.py --write   write the outputs (intentional changes only)
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
FIXED_FILES = ("README.md", "generators/w16c_reference.py")

# ----------------------------------------------------------------------------- pi
getcontext().prec = 100


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


PI_DEC = 16 * _atan_inv(5) - 4 * _atan_inv(239)  # Machin, ~95 correct digits
_PI_90 = F(str(PI_DEC)[:92])  # "3." + 90 digits, truncated
PI_LO, PI_HI = _PI_90, _PI_90 + F(1, 10 ** 90)
assert PI_LO < F(314159265358979323846, 10 ** 20) + F(1, 10 ** 20) and PI_HI > F(314159265358979323846, 10 ** 20)


def binary64(q, k):
    """Nearest double to q * pi**k (k in -1, 0, 1), the same for both rational bounds of pi."""
    if k == 0 or q == 0:
        return float(q)
    lo, hi = (float(q * PI_LO), float(q * PI_HI)) if k == 1 else (float(q / PI_HI), float(q / PI_LO))
    if lo != hi:
        raise SystemExit(f"binary64 rounding of {q}*pi**{k} is not decided by the pi bounds")
    return lo


def decimal40(q, k):
    if q == 0:
        return "0"
    getcontext().prec = 100
    v = Decimal(q.numerator) / Decimal(q.denominator)
    v = v * PI_DEC if k == 1 else v / PI_DEC if k == -1 else v
    return format(v.normalize().__format__(".39e"))


def exact_text(q, k):
    s = str(q)
    return s if k == 0 or q == 0 else f"{s}*pi" if k == 1 else f"{s}/pi"


def record(q, k):
    q = F(q)
    return {"exact": exact_text(q, k), "decimal": decimal40(q, k), "value": binary64(q, k)}


# ----------------------------------------------------------------------------- inputs (invented)
L = F(5, 2)  # m, cantilever length along global X (L != 1)
D = F(1, 5)  # m, outside diameter
R = D / 2
T = F(1, 100)  # m, nominal wall
C_M = F(1, 800)  # m, mill tolerance
C_A = F(1, 500)  # m, corrosion allowance (EX only; the PF twin omits it)
T_I = F(1, 20)  # m, insulation thickness
RHO_M, RHO_C, RHO_I = F(7850), F(1000), F(150)  # kg/m^3: metal, contents, insulation
E = F(200 * 10 ** 9)  # Pa
NU = F(3, 10)  # exact route: G = E / (2 (1 + nu))
G = E / (2 * (1 + NU))  # also authored on the PF twin (no Timoshenko term there)
GRAVITY = F(981, 100)  # m/s^2, the user's explicit gravity input
K_Y = F(1, 4)  # the user's lateral g-factor along global Y
WIND_P = F(1200)  # Pa, the user's wind pressure
WIND_C = F(7, 10)  # the user's shape factor
PARTIAL = (F(1, 5), F(7, 10))  # the marked fractions [a, b] of the partial-span wind case
STATIONS = (F(0), F(1, 4), F(1, 2), F(3, 4), F(1))
STATION_NAMES = ("0", "1/4", "1/2", "3/4", "1")
COMPONENTS_F = ("Fx", "Fy", "Fz", "Mx", "My", "Mz")
COMPONENTS_U = ("ux", "uy", "uz", "rx", "ry", "rz")
SECTION_COMPONENTS = ("axial_force", "shear_force_y", "shear_force_z",
                      "torsional_moment", "bending_moment_y", "bending_moment_z")

# Radii and walls (all exact).
T_R = (T - C_M) - C_A  # EX reduced wall, the stress basis "reduced_section"
R_N = R - T  # EX nominal inner radius (stiffness, metal mass)
R_P = R - T_R  # EX pressure bore radius (contents mass)
R_PF = R - (T - C_M)  # PF single inner radius (metal, contents and stiffness)


def annulus_p(r_o, r_i):
    """Area / pi of the annulus between r_i and r_o."""
    return r_o * r_o - r_i * r_i


def inertia_p(r_i):
    """Second moment / pi of the pipe annulus R..r_i: (R^4 - r_i^4) / 4."""
    return (R ** 4 - r_i ** 4) / 4


def mass_p(metal_inner, bore):
    """Mass per length / pi: metal R..metal_inner, contents 0..bore, insulation R..R + t_i."""
    return RHO_M * annulus_p(R, metal_inner) + RHO_C * bore * bore + RHO_I * annulus_p(R + T_I, R)


def kappa_energy(m, nu):
    """The energy-matched annular shear factor kappa_E, cancellation-free denominator."""
    n1 = 1 + nu
    c = (8 * nu + 14) * nu + 7
    den = c * (1 - m * m) ** 2 + 48 * m * m * n1 * n1
    return 6 * n1 * n1 * (1 + m * m) ** 2 / den


def kappa_energy_contract_form(m, nu):
    den = (7 + 14 * nu + 8 * nu * nu) * (1 + m * m) ** 2 + 4 * m * m * (5 + 10 * nu + 4 * nu * nu)
    return 6 * (1 + nu) ** 2 * (1 + m * m) ** 2 / den


MU_EX_P = mass_p(R_N, R_P)
MU_PF_P = mass_p(R_PF, R_PF)
MU_DIFF_P = RHO_M * C_M * (D - 2 * T + C_M) + RHO_C * (2 * C_A * (R - T + C_M) + C_A * C_A)
I_N_P = inertia_p(R_N)  # EX stiffness and nominal stress basis
I_R_P = inertia_p(R - T_R)  # EX reduced stress basis
I_PF_P = inertia_p(R_PF)  # PF stiffness and stress
KAPPA = kappa_energy(R_N / R, NU)
A_S_P = KAPPA * annulus_p(R, R_N)  # shear area / pi on the nominal annulus
W_EXPOSED = D + 2 * T_I  # projected diameter
Q_WIND = WIND_P * WIND_C * W_EXPOSED  # N/m, rational (no pi)

CROSS = []


def cross_check(name, a, b):
    CROSS.append({"check": name, "agrees": a == b})
    if a != b:
        print(f"CROSS-CHECK FAILED: {name}: {a} != {b}", file=sys.stderr)


# ----------------------------------------------------------------------------- beam statics (closed forms)
def pos(v):
    return v if v > 0 else F(0)


def shear(qv, a, b, x):
    """V(x) / (q's unit): load beyond x on [aL, bL]."""
    return qv * (pos(b * L - max(x, a * L)) if x < b * L else F(0))


def moment(qv, a, b, x):
    """|M(x)|: moment at x of the load beyond x on [aL, bL]."""
    return qv * (pos(b * L - x) ** 2 - pos(a * L - x) ** 2) / 2


def simpson(f, lo, hi):
    if hi <= lo:
        return F(0)
    return (hi - lo) * (f(lo) + 4 * f((lo + hi) / 2) + f(hi)) / 6


def piecewise(f, a, b):
    """Integral of f over [0, L], exact by Simpson on each interval between load breakpoints."""
    cuts = sorted({F(0), a * L, b * L, L})
    return sum(simpson(f, x0, x1) for x0, x1 in zip(cuts, cuts[1:]))


def loaded(qv, a, b, weight):
    """Integral over [0, L] of the load (q on [aL, bL], zero elsewhere) times `weight`: the load is
    constant on each interval between breakpoints, so Simpson's rule per interval is exact."""
    cuts = sorted({F(0), a * L, b * L, L})
    return sum((qv if a * L <= (x0 + x1) / 2 <= b * L else F(0)) * simpson(weight, x0, x1)
               for x0, x1 in zip(cuts, cuts[1:]))


def cantilever(case_id, qv, a, b, ei, gas, axis):
    """One cantilever response in rational coefficients. `qv` is q / pi**k (k = 1 for seismic,
    whose mu carries pi; k = 0 for wind); `ei` and `gas` are EI / pi and G A_s / pi. Forces and
    moments are then coefficients of pi**k, translations, rotations and stresses of pi**(k - 1).
    `axis` is "y" (load +Y, bending about local z) or "z" (load +Z, bending about local y).
    Returns reactions, motions and sections."""
    total = qv * L * (b - a)
    m0 = qv * L * L * (b * b - a * a) / 2
    tip_bending = qv * L ** 4 * ((b ** 3 - a ** 3) - (b ** 4 - a ** 4) / 4) / (6 * ei)
    tip_shear = qv * L * L / (2 * gas) if gas is not None else F(0)
    if gas is not None:
        assert (a, b) == (0, 1), "Timoshenko references are full span only"
    tip_rotation = qv * L ** 3 * (b ** 3 - a ** 3) / (6 * ei)
    # Independent route: resultants by direct integration of the load; tip motion by the unit-load
    # method (unit tip force: m(x) = L - x; unit tip couple: m(x) = 1; shear v(x) = 1).
    cross_check(f"{case_id}: resultant |R| by integration", total, loaded(qv, a, b, lambda x: F(1)))
    cross_check(f"{case_id}: |M_0| by integration", m0, loaded(qv, a, b, lambda x: x))
    cross_check(f"{case_id}: M(0) by statics equals |M_0|", m0, moment(qv, a, b, F(0)))
    cross_check(f"{case_id}: V(0) by statics equals |R|", total, shear(qv, a, b, F(0)))
    cross_check(f"{case_id}: tip bending deflection by unit-load integration", tip_bending,
                piecewise(lambda x: moment(qv, a, b, x) * (L - x), a, b) / ei)
    cross_check(f"{case_id}: tip rotation by unit-load integration", tip_rotation,
                piecewise(lambda x: moment(qv, a, b, x), a, b) / ei)
    if gas is not None:
        cross_check(f"{case_id}: tip shear deflection by unit-load integration", tip_shear,
                    piecewise(lambda x: shear(qv, a, b, x), a, b) / gas)
    tip = tip_bending + tip_shear
    zero_f = {c: F(0) for c in COMPONENTS_F}
    zero_u = {c: F(0) for c in COMPONENTS_U}
    reactions = dict(zero_f)
    motion_b = dict(zero_u)
    if axis == "y":
        # Load +Y along +X: the support pushes back -Y and holds Mz = -(x cross q) = -M_0.
        reactions["Fy"], reactions["Mz"] = -total, -m0
        motion_b["uy"], motion_b["rz"] = tip, tip_rotation
    else:
        # Load +Z along +X: x cross z = -y, so the load's moment about A is -M_0 y; the support holds +M_0.
        reactions["Fz"], reactions["My"] = -total, m0
        motion_b["uz"], motion_b["ry"] = tip, -tip_rotation
    sections = []
    for s in STATIONS:
        x = s * L
        row = {c: F(0) for c in SECTION_COMPONENTS}
        v, m = shear(qv, a, b, x), moment(qv, a, b, x)
        # The +x face of the i-side portion [0, x] carries the j-side portion's load: force +q along
        # the load axis and moment (x_load - x) x q, i.e. +M z-ward for +Y load and -M y-ward for +Z.
        if axis == "y":
            row["shear_force_y"], row["bending_moment_z"] = v, m
        else:
            row["shear_force_z"], row["bending_moment_y"] = v, -m
        sections.append(row)
    # Equilibrium: the end_i action (-section(0)) is the support reaction in the local frame (here global).
    if axis == "y":
        cross_check(f"{case_id}: end_i shear equals the support force", -sections[0]["shear_force_y"], reactions["Fy"])
        cross_check(f"{case_id}: end_i moment equals the support moment", -sections[0]["bending_moment_z"], reactions["Mz"])
    else:
        cross_check(f"{case_id}: end_i shear equals the support force", -sections[0]["shear_force_z"], reactions["Fz"])
        cross_check(f"{case_id}: end_i moment equals the support moment", -sections[0]["bending_moment_y"], reactions["My"])
    nodes = {"node:a": zero_u, "node:b": motion_b}
    return {"support:a": reactions}, nodes, sections


# ----------------------------------------------------------------------------- cases
# (id, route, shear model, load, fractions)
CASES = (
    ("W16C-SEISMIC-EX-EB", "EX", "euler_bernoulli", "seismic", (F(0), F(1))),
    ("W16C-SEISMIC-EX-TIMOSHENKO", "EX", "timoshenko_annulus_energy_v1", "seismic", (F(0), F(1))),
    ("W16C-SEISMIC-PF", "PF", "euler_bernoulli", "seismic", (F(0), F(1))),
    ("W16C-WIND-FULL-EX-EB", "EX", "euler_bernoulli", "wind", (F(0), F(1))),
    ("W16C-WIND-FULL-EX-TIMOSHENKO", "EX", "timoshenko_annulus_energy_v1", "wind", (F(0), F(1))),
    ("W16C-WIND-FULL-PF", "PF", "euler_bernoulli", "wind", (F(0), F(1))),
    ("W16C-WIND-PARTIAL-EX-EB", "EX", "euler_bernoulli", "wind", PARTIAL),
    ("W16C-WIND-PARTIAL-PF", "PF", "euler_bernoulli", "wind", PARTIAL),
)


def family(unit_name):
    return {"N": "force", "N*m": "moment", "m": "translation", "rad": "rotation", "Pa": "stress"}[unit_name]


def publish(case_id, reactions, nodes, sections, bases, axis, k):
    """Rows of one case: forces and moments are coefficients of pi**k, the rest of pi**(k - 1)."""
    rows = []
    for sup in sorted(reactions):
        for comp in COMPONENTS_F:
            unit_name = "N" if comp.startswith("F") else "N*m"
            rows.append(dict(quantity="support_reaction", entity=sup, component=comp, unit=unit_name,
                             **record(reactions[sup][comp], k)))
    for nd in sorted(nodes):
        for comp in COMPONENTS_U:
            unit_name = "m" if comp.startswith("u") else "rad"
            rows.append(dict(quantity="nodal_motion", entity=nd, component=comp, unit=unit_name,
                             **record(nodes[nd][comp], k - 1)))
    for name, sec in zip(STATION_NAMES, sections):
        for comp in SECTION_COMPONENTS:
            unit_name = "N" if comp in ("axial_force", "shear_force_y", "shear_force_z") else "N*m"
            rows.append(dict(quantity="section_action", entity="pipe:a-b", station=name, component=comp,
                             unit=unit_name, **record(sec[comp], k)))
    # One set of actions, stressed on each published basis: sigma = M / Z, Z = I / R (pi cancels).
    moment_component, stress_component = (("bending_moment_z", "bending_normal_stress_z") if axis == "y"
                                          else ("bending_moment_y", "bending_normal_stress_y"))
    for basis, inertia in bases:
        for name, sec in zip(STATION_NAMES, sections):
            rows.append(dict(quantity="section_stress", entity="pipe:a-b", station=name, component=stress_component,
                             section_basis=basis, unit="Pa", **record(sec[moment_component] * R / inertia, k - 1)))
    scales = {}
    for fam, (unit_name, kf) in (("force", ("N", k)), ("moment", ("N*m", k)), ("translation", ("m", k - 1)),
                                 ("rotation", ("rad", k - 1)), ("stress", ("Pa", k - 1))):
        mags = [abs(F(r["exact"].replace("*pi", "").replace("/pi", ""))) for r in rows if family(r["unit"]) == fam]
        top = max(mags)
        assert top > 0, (case_id, fam)
        scales[fam] = dict(unit=unit_name, **record(top, kf))
    return rows, scales


def inputs_record():
    return {
        "geometry": {"nodes": {"node:a": [0, 0, 0], "node:b": [str(L), 0, 0]},
                     "pipe": {"id": "pipe:a-b", "from": "node:a", "to": "node:b", "y_reference": [0, 1, 0]},
                     "support": {"id": "support:a", "node": "node:a", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"]},
                     "length_m": str(L)},
        "section": {"outside_diameter_m": str(D), "wall_thickness_m": str(T), "mill_tolerance_m": str(C_M),
                    "corrosion_allowance_m_exact_route": str(C_A), "insulation_thickness_m": str(T_I),
                    "material_density_kg_per_m3": str(RHO_M), "contents_density_kg_per_m3": str(RHO_C),
                    "insulation_density_kg_per_m3": str(RHO_I)},
        "material": {"elastic_modulus_Pa": str(E), "poisson_ratio_exact_route": str(NU),
                     "shear_modulus_Pa_preview_route": str(G)},
        "seismic": {"gravity_acceleration_m_per_s2": str(GRAVITY), "g_factor_y": str(K_Y)},
        "wind": {"pressure_Pa": str(WIND_P), "shape_factor": str(WIND_C), "direction": "global_z",
                 "partial_fractions": [str(PARTIAL[0]), str(PARTIAL[1])]},
    }


def derived_record():
    return {
        "reduced_wall_t_r_m": str(T_R), "nominal_inner_radius_r_n_m": str(R_N), "pressure_bore_radius_r_p_m": str(R_P),
        "preview_inner_radius_m": str(R_PF),
        "mass_per_length_exact_route_kg_per_m": record(MU_EX_P, 1),
        "mass_per_length_preview_route_kg_per_m": record(MU_PF_P, 1),
        "mass_difference_ex_minus_pf_kg_per_m": record(MU_DIFF_P, 1),
        "second_moment_nominal_m4": record(I_N_P, 1),
        "second_moment_reduced_m4": record(I_R_P, 1),
        "second_moment_preview_m4": record(I_PF_P, 1),
        "shear_factor_kappa_E": record(KAPPA, 0),
        "shear_area_m2": record(A_S_P, 1),
        "seismic_intensity_exact_route_N_per_m": record(MU_EX_P * K_Y * GRAVITY, 1),
        "seismic_intensity_preview_route_N_per_m": record(MU_PF_P * K_Y * GRAVITY, 1),
        "wind_intensity_N_per_m": record(Q_WIND, 0),
    }


def build_cases():
    cross_check("mass difference: closed form equals mu_EX - mu_PF", MU_DIFF_P, MU_EX_P - MU_PF_P)
    cross_check("kappa_E: cancellation-free form equals the contract form",
                KAPPA, kappa_energy_contract_form(R_N / R, NU))
    cross_check("reduced wall: t_r = (t - c_m) - c_a is the pressure bore's wall", R - R_P, T_R)
    cases = []
    for case_id, route, shear_model, load, (a, b) in CASES:
        exact = route == "EX"
        inertia = I_N_P if exact else I_PF_P
        if load == "seismic":
            # q = mu k_g g = pi (mu / pi) k_g g: forces carry pi, motions and stresses are rational.
            qv, k, axis = (MU_EX_P if exact else MU_PF_P) * K_Y * GRAVITY, 1, "y"
        else:
            # q_w = p C (D + 2 t_i) is rational: forces are rational, motions and stresses carry 1/pi.
            qv, k, axis = Q_WIND, 0, "z"
        timoshenko = shear_model == "timoshenko_annulus_energy_v1"
        assert not (timoshenko and not exact), "the preview route has no Timoshenko member"
        reactions, nodes, sections = cantilever(case_id, qv, a, b, E * inertia,
                                                G * A_S_P if timoshenko else None, axis)
        bases = [("nominal_section", I_N_P), ("reduced_section", I_R_P)] if exact else [("preview_section", I_PF_P)]
        rows, scales = publish(case_id, reactions, nodes, sections, bases, axis, k)
        cases.append(case_record(case_id, route, shear_model, load, (a, b), rows, scales, nodes, k))
    return cases


def case_record(case_id, route, shear_model, load, fractions, rows, scales, nodes, k):
    tip = nodes["node:b"]
    key = {"tip_translation_m": record(tip["uy"] if load == "seismic" else tip["uz"], k - 1),
           "tip_rotation_rad": record(tip["rz"] if load == "seismic" else tip["ry"], k - 1)}
    return {"id": case_id, "route": route, "shear_model": shear_model, "load": load,
            "fractions": [str(fractions[0]), str(fractions[1])],
            "method": "closed-form cantilever statics and flexure; unit-load member integration cross-check",
            "key_values": key, "zero_scales": scales, "rows": rows}


def document(cases):
    return {
        "document": "openpipestress.w16_exact_equivalent_static_references",
        "status": "candidate frozen references; derived and cross-checked by the author TASK, not yet independently refuted",
        "units": {"force": "N", "moment": "N*m", "translation": "m", "rotation": "rad", "stress": "Pa"},
        "theory": "small-displacement linear-elastic cantilever; Euler-Bernoulli flexure, plus the Timoshenko shear "
                  "deflection q L^2 / (2 G A_s) with A_s = kappa_E A_n on the nominal annulus where selected; "
                  "rotations are section rotations",
        "conventions": {
            "support_reaction": "global action of the support on the structure at its node (moments about that node)",
            "nodal_motion": "global translation and right-handed rotation",
            "section_action": "element-local action on the +x face of the i-side portion at station s/L (local x from i to j); "
                              "a node-on-element end action is -section(0) at end_i and +section(1) at end_j",
            "section_stress": "bending normal stress M / Z of the station's section action on the named basis, "
                              "Z = I / R with I on that basis's wall; it follows the section action at every station, "
                              "end_i included (no end-action sign)",
            "local_axes": "x along global X from A to B; y = global Y (y_reference); z = global Z",
            "exact": "rational q, q*pi or q/pi",
        },
        "criterion": "|observed - expected| <= 1e-9 * |expected|; an exactly zero expectation uses 1e-9 * zero_scale "
                     "of its family (largest nonzero magnitude of that family in the case)",
        "inputs": inputs_record(),
        "derived": derived_record(),
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
    cases = build_cases()
    doc = document(cases)
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
        print(c["id"], ", ".join(f"{k} {v['value']:.6g}" for k, v in c["key_values"].items()))
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
