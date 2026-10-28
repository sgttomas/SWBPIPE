"""Independent closed-form hand calculations for the CAEPIPE correlation pilot cases (DEL-17-05).

Pure Python and `math` only; nothing here imports or calls the SWBPIPE solver.
Each case function returns `{quantity_id: {"value", "unit", "formula", "basis"}}`
in SI base units (m, rad, N, N*m) with SWBPIPE's sign conventions: global
right-handed X, Y, Z axes, right-hand rotations, and support reactions as the
action of the support on the pipe.

`basis` is "Euler-Bernoulli" for the primary witness (bending without shear
deformation, the formulation of SWBPIPE's frame preview). Where shear
deformation changes a compared quantity, `alternative_analytic_values(case_id)`
gives the independent Timoshenko value itself (bending plus transverse shear
flexibility L / (kappa G A)), keyed by hypothesis id and quantity id; a case
declaration of that hypothesis compares it with CAEPIPE (DEL-17-05 Design §6).
The shear coefficient KAPPA = 0.5, i.e. a shape factor 1 / kappa = 2 in the
shear flexibility "shape factor x L / (G A)", is an ASSUMPTION: the CAEPIPE
technical reference (docs.sstcae.com, "Fiber Reinforced Plastic Piping (FRP)",
"Stiffness matrix") names a shape-factor shear term but publishes no value; 2
is the thin-wall circular tube value. Its CAEPIPE value is a question for SST
(DEL-17-01 CQ-17-01-010).

All inputs are invented; no value is library, catalog or code-rule data.
"""

from __future__ import annotations

import math
from typing import Any, Callable


# Shared invented section and material (case JSON `model` carries the same numbers).
OD = 0.1683  # m
WALL = 0.00711  # m
E = 200.0e9  # Pa
NU = 0.3
G = E / (2.0 * (1.0 + NU))  # = E / 2.6
ALPHA = 1.2e-5  # 1/K
RHO = 7850.0  # kg/m3
GRAVITY = 9.80665  # m/s2
KAPPA = 0.5  # ASSUMPTION: thin-wall tube shear coefficient (shape factor 1 / KAPPA = 2)
KAPPA_NOTE = (
    "ASSUMPTION: Timoshenko shear coefficient kappa = 0.5, i.e. shape factor 2 in the shear flexibility "
    "shape factor x L / (G A) (thin-wall circular tube); the CAEPIPE technical reference names the shape-factor "
    "term without a value (docs.sstcae.com tech_manual, Fiber Reinforced Plastic Piping (FRP), Stiffness matrix); "
    "its CAEPIPE value is a question for SST (CQ-17-01-010)"
)

BASIS = "Euler-Bernoulli"
ALTERNATIVE_BASIS = "Timoshenko"
SHEAR_HYPOTHESIS = "shear-deformation"


def section() -> dict[str, float]:
    inner = OD - 2.0 * WALL
    area = math.pi / 4.0 * (OD**2 - inner**2)
    inertia = math.pi / 64.0 * (OD**4 - inner**4)
    return {"A": area, "I": inertia, "J": 2.0 * inertia, "Di": inner}


def _row(value: float, unit: str, formula: str) -> dict[str, Any]:
    return {"value": value, "unit": unit, "formula": formula, "basis": BASIS}


def _alt(value: float, unit: str, formula: str) -> dict[str, Any]:
    return {"value": value, "unit": unit, "formula": formula, "basis": ALTERNATIVE_BASIS, "assumption": KAPPA_NOTE}


def shear_area() -> float:
    """kappa A: the effective shear area of the ASSUMPTION kappa = 0.5."""
    return KAPPA * section()["A"]


# ---------------------------------------------------------------------------
# cpc-01 / cpc-06 / cpc-07 / cpc-08: cantilever with a tip force (span series)
# ---------------------------------------------------------------------------
CANTILEVER_L = 6.0
CANTILEVER_P = -2000.0  # N along global Y at the tip
CANTILEVER_SPANS = {
    "cpc-07-cantilever-span-3m": 3.0,
    "cpc-01-cantilever-force": 6.0,
    "cpc-06-cantilever-fault-injected": 6.0,
    "cpc-08-cantilever-span-12m": 12.0,
}


def cantilever(length: float) -> dict[str, dict[str, Any]]:
    s = section()
    force = CANTILEVER_P
    ei = E * s["I"]
    return {
        "n1_uy": _row(force * length**3 / (3.0 * ei), "m",
                      f"uy(L) = P L^3 / (3 E I), P = -2000 N (signed, global Y), L = {length:g} m"),
        "n1_rz": _row(force * length**2 / (2.0 * ei), "rad",
                      "rz(L) = dv/dx = P L^2 / (2 E I) (right-hand rotation about +Z of a member along +X)"),
        "a0_fy": _row(-force, "N", "Fy_A = -P (support on pipe; vertical equilibrium)"),
        "a0_mz": _row(-length * force, "N*m",
                      "Mz_A = -(r_tip x F)_z = -(L * P) (support on pipe; moment equilibrium about the anchor)"),
    }


def cantilever_timoshenko(length: float) -> dict[str, dict[str, Any]]:
    """Timoshenko tip deflection; the tip rotation and the reactions are unchanged by shear (constant shear force)."""
    s = section()
    force = CANTILEVER_P
    value = force * length**3 / (3.0 * E * s["I"]) + force * length / (G * shear_area())
    return {"n1_uy": _alt(value, "m",
                          f"uy(L) = P L^3 / (3 E I) + P L / (kappa G A), kappa = 0.5, G = E / (2 (1 + nu)), "
                          f"P = -2000 N, L = {length:g} m")}


def cpc_01_cantilever_force() -> dict[str, dict[str, Any]]:
    return cantilever(CANTILEVER_L)


# ---------------------------------------------------------------------------
# cpc-02: straight pipe anchored at both ends, uniform temperature rise
# ---------------------------------------------------------------------------
THERMAL_L = 6.0
DELTA_T = 100.0  # K


def cpc_02_anchored_thermal() -> dict[str, dict[str, Any]]:
    s = section()
    thrust = E * s["A"] * ALPHA * DELTA_T
    return {
        "a0_fx": _row(thrust, "N",
                      "Fx_A(x=0) = +E A alpha dT (restrained free expansion; the anchor pushes the pipe toward +X)"),
        "a1_fx": _row(-thrust, "N", "Fx_B(x=L) = -E A alpha dT"),
    }


def cpc_06_cantilever_fault_injected() -> dict[str, dict[str, Any]]:
    return cpc_01_cantilever_force()


def cpc_07_cantilever_span_3m() -> dict[str, dict[str, Any]]:
    return cantilever(CANTILEVER_SPANS["cpc-07-cantilever-span-3m"])


def cpc_08_cantilever_span_12m() -> dict[str, dict[str, Any]]:
    return cantilever(CANTILEVER_SPANS["cpc-08-cantilever-span-12m"])


# ---------------------------------------------------------------------------
# cpc-03: straight pipe anchored at both ends under self-weight
# ---------------------------------------------------------------------------
WEIGHT_L = 6.0


def weight_per_length() -> float:
    """Signed weight per length along global Y (negative: gravity along -Y)."""
    return -RHO * section()["A"] * GRAVITY


def cpc_03_anchored_weight() -> dict[str, dict[str, Any]]:
    s = section()
    w, length = weight_per_length(), WEIGHT_L
    ei = E * s["I"]
    return {
        "mid_uy": _row(w * length**4 / (384.0 * ei), "m",
                       "uy(L/2) = w L^4 / (384 E I), fixed-fixed, w = -rho A g (signed, global Y), L = 6 m"),
        "a0_fy": _row(-w * length / 2.0, "N", "Fy_A = -w L / 2"),
        "a1_fy": _row(-w * length / 2.0, "N", "Fy_B = -w L / 2"),
        "a0_mz": _row(-w * length**2 / 12.0, "N*m",
                      "Mz_A(x=0) = -w L^2 / 12 (counter-clockwise for downward w: the anchor stops the end rotation)"),
        "a1_mz": _row(w * length**2 / 12.0, "N*m", "Mz_B(x=L) = +w L^2 / 12 (antisymmetric to Mz_A)"),
    }


def cpc_03_timoshenko() -> dict[str, dict[str, Any]]:
    s = section()
    w, length = weight_per_length(), WEIGHT_L
    # Shear strain V/(kappa G A) with V(x) = -w (L/2 - x) integrated over 0..L/2; the end
    # moments and reactions are unchanged by symmetry (bending rotation is still fixed by M/EI).
    value = w * length**4 / (384.0 * E * s["I"]) + w * length**2 / (8.0 * G * shear_area())
    return {"mid_uy": _alt(value, "m", "uy(L/2) = w L^4 / (384 E I) + w L^2 / (8 kappa G A), fixed-fixed, "
                                       "w = -rho A g, L = 6 m, kappa = 0.5")}


# ---------------------------------------------------------------------------
# cpc-04: horizontal L in the X-Z plane with a 90 degree bend, thermal
# ---------------------------------------------------------------------------
L_A = (0.0, 0.0)  # (x, z) of anchor A
L_TI = (4.0, 0.0)  # tangent intersection
L_B = (4.0, 3.0)  # anchor B
BEND_R = 0.25
BEND_K = 1.0


def _simpson(func: Callable[[float], float], length: float) -> float:
    """Integral over 0..length; exact for the polynomials of degree <= 3 used here."""
    return length / 6.0 * (func(0.0) + 4.0 * func(length / 2.0) + func(length))


def _arc_product(f: tuple[float, float, float], g: tuple[float, float, float], theta: float) -> float:
    """Closed-form integral over 0..theta of (a + b cos p + c sin p)(a' + b' cos p + c' sin p) dp."""
    a, b, c = f
    a2, b2, c2 = g
    i_1 = theta
    i_c = math.sin(theta)
    i_s = 1.0 - math.cos(theta)
    i_cc = theta / 2.0 + math.sin(2.0 * theta) / 4.0
    i_ss = theta / 2.0 - math.sin(2.0 * theta) / 4.0
    i_sc = math.sin(theta) ** 2 / 2.0
    return (a * a2 * i_1 + (a * b2 + a2 * b) * i_c + (a * c2 + a2 * c) * i_s
            + b * b2 * i_cc + c * c2 * i_ss + (b * c2 + b2 * c) * i_sc)


def l_bend_flexibility(*, shear: bool, arc_axial: bool) -> list[list[float]]:
    """3x3 flexibility of anchor B (ux, uz, ry) under (Fx, Fz, My) at B with A fixed.

    Castigliano over the path A -> T1 (leg 1, along +X) -> arc -> T2 -> B (leg 2,
    along +Z). The moment about Y at a path point p from loads at B is
    M = My + (zB - z) Fx - (xB - x) Fz. Terms: bending of both legs and the arc
    (arc bending flexibility multiplied by k), axial of both legs, axial of the
    arc when `arc_axial`, transverse shear (kappa G A) everywhere when `shear`.
    """
    s = section()
    ei, ea, kga = E * s["I"], E * s["A"], KAPPA * G * s["A"]
    xb, zb = L_B
    radius = BEND_R
    t1 = (L_TI[0] - radius, 0.0)
    t2 = (L_TI[0], radius)
    centre = (t1[0], t2[1])
    leg1, leg2 = t1[0] - L_A[0], zb - t2[1]
    theta = math.pi / 2.0

    def leg1_m(i: int) -> Callable[[float], float]:
        # point (s, 0) for s in 0..leg1
        return [lambda s_: zb - 0.0, lambda s_: -(xb - s_), lambda s_: 1.0][i]

    def leg2_m(i: int) -> Callable[[float], float]:
        # point (4, 0.25 + s) for s in 0..leg2
        return [lambda s_: zb - (t2[1] + s_), lambda s_: -(xb - t2[0]), lambda s_: 1.0][i]

    # Arc point p(phi) = (xc + R sin phi, zc - R cos phi), phi in 0..pi/2; ds = R dphi.
    arc_m = [((zb - centre[1]), radius, 0.0), (-(xb - centre[0]), 0.0, radius), (1.0, 0.0, 0.0)]
    arc_n = [(0.0, 1.0, 0.0), (0.0, 0.0, 1.0), (0.0, 0.0, 0.0)]  # F . tangent, tangent = (cos, sin)
    arc_v = [(0.0, 0.0, -1.0), (0.0, 1.0, 0.0), (0.0, 0.0, 0.0)]  # F . normal, normal = (-sin, cos)
    leg1_n, leg2_n = [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]
    leg1_v, leg2_v = [0.0, 1.0, 0.0], [-1.0, 0.0, 0.0]

    flex = [[0.0] * 3 for _ in range(3)]
    for i in range(3):
        for j in range(3):
            value = _simpson(lambda s_: leg1_m(i)(s_) * leg1_m(j)(s_), leg1) / ei
            value += _simpson(lambda s_: leg2_m(i)(s_) * leg2_m(j)(s_), leg2) / ei
            value += BEND_K * radius * _arc_product(arc_m[i], arc_m[j], theta) / ei
            value += (leg1_n[i] * leg1_n[j] * leg1 + leg2_n[i] * leg2_n[j] * leg2) / ea
            if arc_axial:
                value += radius * _arc_product(arc_n[i], arc_n[j], theta) / ea
            if shear:
                value += (leg1_v[i] * leg1_v[j] * leg1 + leg2_v[i] * leg2_v[j] * leg2) / kga
                value += radius * _arc_product(arc_v[i], arc_v[j], theta) / kga
            flex[i][j] = value
    return flex


def _solve3(matrix: list[list[float]], rhs: list[float]) -> list[float]:
    """Gaussian elimination with partial pivoting (3x3)."""
    rows = [list(matrix[i]) + [rhs[i]] for i in range(3)]
    for col in range(3):
        pivot = max(range(col, 3), key=lambda r: abs(rows[r][col]))
        rows[col], rows[pivot] = rows[pivot], rows[col]
        for r in range(col + 1, 3):
            factor = rows[r][col] / rows[col][col]
            for c in range(col, 4):
                rows[r][c] -= factor * rows[col][c]
    out = [0.0, 0.0, 0.0]
    for r in (2, 1, 0):
        out[r] = (rows[r][3] - sum(rows[r][c] * out[c] for c in range(r + 1, 3))) / rows[r][r]
    return out


def l_bend_reactions(*, shear: bool, arc_axial: bool) -> dict[str, float]:
    xb, zb = L_B
    free = [ALPHA * DELTA_T * (xb - L_A[0]), ALPHA * DELTA_T * (zb - L_A[1]), 0.0]
    fx_b, fz_b, my_b = _solve3(l_bend_flexibility(shear=shear, arc_axial=arc_axial), [-value for value in free])
    # Equilibrium of the pipe: forces sum to zero; moments about A: My_A + My_B + (zB Fx_B - xB Fz_B) = 0.
    return {
        "b_fx": fx_b, "b_fz": fz_b, "b_my": my_b,
        "a_fx": -fx_b, "a_fz": -fz_b, "a_my": -my_b - (zb * fx_b - xb * fz_b),
    }


_L_FORMULA = (
    "force method: release anchor B; free thermal displacement of B = alpha dT (xB - xA, zB - zA) = alpha dT (4, 3) m, "
    "ry = 0; flexibility F_ij = int m_i m_j / (E I) ds over leg 1 (3.75 m), leg 2 (2.75 m) and the quarter arc "
    "(R = 0.25 m, bending x k = 1) + int n_i n_j / (E A) ds over legs and arc; m = My + (zB - z) Fx - (xB - x) Fz; "
    "legs integrated by Simpson's rule (exact for these polynomials), arc in closed form; transverse shear and "
    "arc ovalization beyond k are not included; R_B = -F^-1 delta_free; R_A from equilibrium "
    "(My_A = -My_B - (zB Fx_B - xB Fz_B))"
)


def cpc_04_l_bend_thermal() -> dict[str, dict[str, Any]]:
    values = l_bend_reactions(shear=False, arc_axial=True)
    labels = {"a_fx": "Fx at A", "a_fz": "Fz at A", "a_my": "My at A",
              "b_fx": "Fx at B", "b_fz": "Fz at B", "b_my": "My at B"}
    return {
        key: _row(values[key], "N*m" if key.endswith("my") else "N", f"{labels[key]}: {_L_FORMULA}")
        for key in ("a_fx", "a_fz", "a_my", "b_fx", "b_fz", "b_my")
    }


def cpc_04_timoshenko() -> dict[str, dict[str, Any]]:
    values = l_bend_reactions(shear=True, arc_axial=True)
    formula = (_L_FORMULA.replace("transverse shear and arc ovalization beyond k are not included",
                                  "arc ovalization beyond k is not included")
               + "; plus transverse shear int v_i v_j / (kappa G A) ds over legs and arc, kappa = 0.5")
    return {key: _alt(values[key], "N*m" if key.endswith("my") else "N", formula)
            for key in ("a_fx", "a_fz", "a_my", "b_fx", "b_fz", "b_my")}


# ---------------------------------------------------------------------------
# cpc-05: tee junction, imposed displacement UY at the Z-leg anchor
# ---------------------------------------------------------------------------
BRANCH_L = 4.0
IMPOSED_UY = 0.010  # m at A3


def branch_solution(*, shear: bool) -> dict[str, float]:
    """Two-DOF stiffness solve in (uy_J, rx_J).

    By the mirror symmetry about x = 0, ry_J = rz_J = 0 and ux_J = uz_J = 0 (the
    load is out of the X-Z plane). X-legs (A1-J, J-A2): fixed-guided bending,
    transverse stiffness 12 E I / ((1 + Phi) L^3) each, torsion G J / L each
    (J = 2 I). Z-leg (J-A3), local v = uy, theta = dv/dz = -rx: Timoshenko beam
    matrix with Phi = 12 E I / (kappa G A L^2) (Phi = 0 for Euler-Bernoulli).
    """
    s = section()
    length, delta = BRANCH_L, IMPOSED_UY
    ei = E * s["I"]
    phi = 12.0 * ei / (KAPPA * G * s["A"] * length**2) if shear else 0.0
    kb = ei / ((1.0 + phi) * length**3)
    kt = G * s["J"] / length
    # Eq. uy: 2 * 12 kb v + kb (12 v + 6 L theta1 - 12 delta) = 0, theta1 = -r
    # Eq. rx: 2 kt r - M1 = 0, M1 = kb (6 L v + (4 + phi) L^2 theta1 - 6 L delta)
    a11, a12, b1 = 36.0 * kb, -6.0 * length * kb, 12.0 * kb * delta
    a21, a22, b2 = -6.0 * length * kb, 2.0 * kt + (4.0 + phi) * length**2 * kb, -6.0 * length * kb * delta
    det = a11 * a22 - a12 * a21
    v = (b1 * a22 - a12 * b2) / det
    r = (a11 * b2 - a21 * b1) / det
    theta1 = -r
    fy_a3 = kb * (-12.0 * v - 6.0 * length * theta1 + 12.0 * delta)
    m2 = kb * (6.0 * length * v + (2.0 - phi) * length**2 * theta1 - 6.0 * length * delta)
    return {
        "j_uy": v,
        "j_rx": r,
        "a1_fy": -12.0 * kb * v,
        "a2_fy": -12.0 * kb * v,
        "a3_fy": fy_a3,
        "a3_mx": -m2,
        "a1_mx": -kt * r,
        "a2_mx": -kt * r,
    }


_BRANCH_FORMULA = (
    "two-DOF stiffness solve in (uy_J, rx_J): 36 kb v - 6 L kb r = 12 kb d; -6 L kb v + (2 G J / L + 4 L^2 kb) r = "
    "-6 L kb d; kb = E I / L^3, J = 2 I, L = 4 m, d = +0.010 m at A3; X-legs fixed-guided (rz_J = 0 by symmetry) "
    "in bending and torsion, Z-leg in bending with theta = dv/dz = -rx"
)


def cpc_05_branch_imposed_displacement() -> dict[str, dict[str, Any]]:
    values = branch_solution(shear=False)
    extra = {
        "j_uy": "uy_J = v",
        "j_rx": "rx_J = r",
        "a1_fy": "Fy_A1 = -12 kb v (X-leg end force on the pipe)",
        "a2_fy": "Fy_A2 = -12 kb v",
        "a3_fy": "Fy_A3 = kb (-12 v - 6 L theta1 + 12 d), theta1 = -r",
        "a3_mx": "Mx_A3 = -M2, M2 = kb (6 L v + 2 L^2 theta1 - 6 L d)",
    }
    units = {"j_uy": "m", "j_rx": "rad", "a1_fy": "N", "a2_fy": "N", "a3_fy": "N", "a3_mx": "N*m"}
    return {key: _row(values[key], units[key], f"{extra[key]}; {_BRANCH_FORMULA}") for key in units}


def cpc_05_timoshenko() -> dict[str, dict[str, Any]]:
    values = branch_solution(shear=True)
    units = {"j_uy": "m", "j_rx": "rad", "a1_fy": "N", "a2_fy": "N", "a3_fy": "N", "a3_mx": "N*m"}
    formula = (_BRANCH_FORMULA + "; Timoshenko: kb = E I / ((1 + Phi) L^3), Phi = 12 E I / (kappa G A L^2), "
               "kappa = 0.5, Z-leg near-end term (4 + Phi) L^2 kb, far-end moment (2 - Phi) L^2 kb")
    return {key: _alt(values[key], unit, formula) for key, unit in units.items()}


HAND_CALCS: dict[str, Callable[[], dict[str, dict[str, Any]]]] = {
    "cpc-01-cantilever-force": cpc_01_cantilever_force,
    "cpc-02-anchored-thermal": cpc_02_anchored_thermal,
    "cpc-03-anchored-weight": cpc_03_anchored_weight,
    "cpc-04-l-bend-thermal": cpc_04_l_bend_thermal,
    "cpc-05-branch-imposed-displacement": cpc_05_branch_imposed_displacement,
    "cpc-06-cantilever-fault-injected": cpc_06_cantilever_fault_injected,
    "cpc-07-cantilever-span-3m": cpc_07_cantilever_span_3m,
    "cpc-08-cantilever-span-12m": cpc_08_cantilever_span_12m,
}

# hypothesis id -> case id -> Timoshenko values of the quantities shear deformation changes.
ALTERNATIVE_ANALYTICS: dict[str, dict[str, Callable[[], dict[str, dict[str, Any]]]]] = {
    SHEAR_HYPOTHESIS: {
        **{case_id: (lambda length=length: cantilever_timoshenko(length))
           for case_id, length in CANTILEVER_SPANS.items()},
        "cpc-03-anchored-weight": cpc_03_timoshenko,
        "cpc-04-l-bend-thermal": cpc_04_timoshenko,
        "cpc-05-branch-imposed-displacement": cpc_05_timoshenko,
    },
}


def analytic_values(case_id: str) -> dict[str, dict[str, Any]]:
    """The Euler-Bernoulli witness values of a case, keyed by quantity id."""
    if case_id not in HAND_CALCS:
        raise KeyError(f"no hand calculation for case {case_id!r}")
    return HAND_CALCS[case_id]()


def alternative_analytic_values(case_id: str) -> dict[str, dict[str, dict[str, Any]]]:
    """Independent alternative analytic values of a case: {hypothesis id: {quantity id: row}} (SI).

    Empty for a case no hypothesis changes. Each row carries its formula, basis and ASSUMPTION.
    """
    return {hypothesis: functions[case_id]() for hypothesis, functions in ALTERNATIVE_ANALYTICS.items()
            if case_id in functions}
