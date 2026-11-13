"""Tranche 3 of the invented CAEPIPE benchmark models (DEL-17-05, REQ-017 to REQ-020, AC-007; the "Benchmark method"
section of docs/RELEASE_TARGET_v1.0.md). `benchmark_models.generate` appends it after tranche 2; it draws nothing
from the seeded random sequence (every member is designed), so tranches 1 and 2 keep their draws and their bytes.

Series S1, S2 and S3 are linear static one-parameter series; family D is dynamics, folded from the dynamics lane's
wrapper files. Every prediction below is textbook mechanics derived in this module's own text; nothing here imports
product code for a value, runs either program, or records a result of either.

S1, transverse shear in straight pipe (hypothesis shear_deformation). Four cantilevers of one outside diameter and one
short span, differing only in wall thickness, under weight and a tip force. Euler-Bernoulli tip deflection
    delta_EB = P L^3/(3 E I) + w L^4/(8 E I),
Timoshenko tip deflection, the shear force V(x) = P + w (L - x) integrated over the span:
    delta_T = delta_EB + (P L + w L^2/2)/(kappa G A),
with Cowper's shear coefficient of a hollow circular section, m = r_i/r_o,
    kappa = 6 (1 + nu) (1 + m^2)^2 / ((7 + 6 nu) (1 + m^2)^2 + (20 + 12 nu) m^2)
(G. R. Cowper, "The shear coefficient in Timoshenko's beam theory", Journal of Applied Mechanics 33(2), 335-340, 1966,
Table 1, hollow circle). The predicted relative difference is (delta_T - delta_EB)/|delta_EB|, which for a tip force
P alone is sign(P) 3 E I/(kappa G A L^2), of magnitude 3 (E/(kappa G)) (r_o^2 + r_i^2)/(4 L^2); negative here, since
the force is downward. At one outside diameter it GROWS as the
wall thins (kappa falls toward 0.53 and (r_o^2 + r_i^2) rises toward 2 r_o^2), so the thickest member shows the
smallest shear fraction, not the largest; every member here shows it clearly because the span is short.

S2, shear and flexibility in a bend (hypotheses bend_transverse_shear, shear_deformation, bend_flexibility_application).
One realized 90-degree bend with user flexibility factor 1 between two short tangents in the X-Z plane, both ends
anchored, a uniform temperature change; three members differing only in wall thickness. Anchors at both ends make
the in-plane problem three times indeterminate; Castigliano's second theorem on the plane frame gives it in closed
form. Release B and let it carry the support's action (F_x, F_z, M) on the pipe. With unit-load actions m_i, n_i,
v_i (moment, axial and transverse force at a section for a unit value of the i-th redundant), the flexibility is
    f_ij = sum over the path of  integral [ k m_i m_j/(E I) + n_i n_j/(E A) + v_i v_j/(kappa G A) ] ds,
with k the bend's flexibility factor on the arc (1 here) and 1 on the tangents. Released, B moves alpha dT (r_B - r_A)
without turning, so compatibility is f Q = -alpha dT (x_B - x_A, z_B - z_A, 0). On the arc (centre (a, R), angle
phi from 0 to pi/2, ds = R dphi) every unit-load action is c_0 + c_c cos(phi) + c_s sin(phi), so every integral is
elementary (pi/2, 1, 1, pi/4, pi/4, 1/2 for 1, cos, sin, cos^2, sin^2, sin cos); on the tangents each is linear in s.
Four energy sets are evaluated: bending only; bending and axial (the energy of SWBPIPE's formulation for a document
without a pressure contract: Euler-Bernoulli members with axial strain and no shear deformation, closed form checked
against its solve); bending, axial and shear (kappa as in S1); and bending and axial with the anchors' stated default
stiffness (S3) added as F^2/(2 k_t) + M^2/(2 k_r) at each anchor.

S3, the finite stiffness of a "rigid" support (hypothesis rigid_support_stiffness). The target program's public
technical reference page "Anchor" (https://docs.sstcae.com/tech_manual/anchor.htm, read 2026-11-12; CAEPIPE-ANCHOR in
execution/PKG-17/DEL-17-01/Source_Basis_Register.md) states that its rigid stiffness is set internally to 1e12 lb/inch
in translation and 1e12 inch-lb/radian in rotation; in SI (1 lbf = 4.4482216152605 N, 1 inch = 0.0254 m, exact by
definition) k_t = 1.75126835246476e14 N/m and k_r = 1.129848290276167e11 N m/rad. A straight pipe anchored at both
ends: SWBPIPE's anchors are ideal. Under a temperature change, with springs k_t at both ends, the pipe's free growth
alpha dT L is taken up by its own shortening N L/(E A) and the two end movements N/k_t:
    alpha dT L = N L/(E A) + 2 N/k_t,  so  N = E A alpha dT / (1 + 2 E A/(k_t L)),
against N = E A alpha dT for ideal anchors. Under weight, by symmetry both ends rotate equally; a simply supported
span's end rotation w L^3/(24 E I) less the rotation M L/(2 E I) of equal end moments equals M/k_r, so
    M = (w L^2/12) / (1 + 2 E I/(k_r L)),
against w L^2/12 for ideal anchors; the end translations N/k_t move the span rigidly and change no moment. Shear
deformation changes neither (a symmetric span's section rotations at its ends are those of bending alone). The
tracked models keep SWBPIPE's ideal anchors, so each file is the ordinary anchor record.

Family D, dynamics. The ten cases of the dynamics lane's wrapper files (dynamics family draft 4, 2026-11-12), built by
the same arithmetic as their generator; their reference values are quoted from dynamics_reference_quotes.json, where
the wrappers' source and standing are kept. SWBPIPE solves nothing dynamic here: models, requests, declarations and
third values only. Each case's runs are declared ADDITIONS of whole documented options records to the written file
(`added_mbf_text`): the dynamics options record and the spectrum table; what no documented record carries is entered
by the person at the program's window, and each run says so.
"""

from __future__ import annotations

import copy
from decimal import Decimal, localcontext
import hashlib
import json
import math
from pathlib import Path
from typing import Any

from tools.caepipe_correlation import benchmark_models as bm
from tools.caepipe_correlation.benchmark_designed import DESIGNED_MATERIAL, designed_od, designed_wall
from tools.caepipe_correlation.benchmark_models import (
    BEND_K, BEND_SHEAR, EXPANSION, OPERATING, SHEAR, SUSTAINED, Builder, closed, corner_bend, section_properties,
)


TRANCHE = "3"
PROVENANCE = ("invented CAEPIPE benchmark input (DEL-17-05, tranche 3); seeded generator; not library, component, "
              "catalog, standard or code-rule data")
RIGID = "rigid_support_stiffness"
DATE_READ = "2026-11-12"
_PREC = 60


def _url(kind: str, page: str) -> str:
    return f"https://docs.sstcae.com/{kind}/{page}.htm"


IMPORT_PAGE = (f"{_url('users_manual', 'import_mbf')}, section \"Format of .MBF\" (CAEPIPE-IMPORT-MBF, "
               f"execution/PKG-17/DEL-17-01/Source_Basis_Register.md), read {DATE_READ}")
ANCHOR_PAGE = (f"{_url('tech_manual', 'anchor')} (technical reference, \"Anchor\"; CAEPIPE-ANCHOR, "
               f"execution/PKG-17/DEL-17-01/Source_Basis_Register.md), read {DATE_READ}")
SPECTRUMS_PAGE = f"{_url('users_manual', 'spectrums')} (user's manual, \"Spectrums\"), read {DATE_READ}"
LOAD_CASES_PAGE = f"{_url('users_manual', 'load_cases')} (user's manual, \"Load cases\"), read {DATE_READ}"
DYNAMICS_PAGE = (f"{_url('tech_manual', 'dynamic_analysis')} (technical reference, \"Dynamic analysis\", section "
                 f"\"Response Spectrum\"), read {DATE_READ}")
COWPER = ("G. R. Cowper, \"The shear coefficient in Timoshenko's beam theory\", Journal of Applied Mechanics 33(2), "
          "335-340 (1966), Table 1, hollow circular section")
CASTIGLIANO = ("Castigliano's second theorem on a plane frame (complementary energy of bending, axial and transverse "
               "shear; e.g. S. Timoshenko and D. H. Young, Theory of Structures, 2nd ed., McGraw-Hill 1965, the "
               "chapter on statically indeterminate frames)")

# -- the anchor's stated default stiffness (S3) ----------------------------------------------------------------------
_LBF_N = Decimal("4.4482216152605")  # 0.45359237 kg x 9.80665 m/s2, exact by definition
_INCH_M = Decimal("0.0254")


def _rigid_stiffness() -> tuple[float, float]:
    with localcontext() as context:
        context.prec = _PREC
        stated = Decimal("1e12")
        return float(stated * _LBF_N / _INCH_M), float(stated * _LBF_N * _INCH_M)


K_TRANSLATION, K_ROTATION = _rigid_stiffness()
RIGID_SUPPORT = {
    "source": ANCHOR_PAGE,
    "stated": ("\"Internally, the rigid stiffness value is set to 1x10^12 (lb./inch) for translational stiffness and "
               "1x10^12 (inch-lb./radian) for rotational stiffness\" (the page, section \"Stiffness\")"),
    "translational_n_per_m": K_TRANSLATION,
    "rotational_n_m_per_rad": K_ROTATION,
    "conversion": ("1 lbf = 4.4482216152605 N and 1 inch = 0.0254 m, exact by definition; a routine units conversion, "
                   "computed in decimal at 60 digits and rounded once"),
    "swbpipe": "SWBPIPE's anchor is ideal: every restrained component is held exactly",
}


# -- textbook closed forms (binary64 results of decimal working) ----------------------------------------------------
def _d(value: float) -> Decimal:
    return Decimal(value)


def cowper_kappa(od_mm: float, wall_mm: float, nu: float) -> float:
    """Cowper's shear coefficient of a hollow circle (COWPER)."""
    with localcontext() as context:
        context.prec = _PREC
        m = (_d(od_mm) - 2 * _d(wall_mm)) / _d(od_mm)
        one_m2 = 1 + m * m
        n = _d(nu)
        return float(6 * (1 + n) * one_m2 ** 2 / ((7 + 6 * n) * one_m2 ** 2 + (20 + 12 * n) * m * m))


def _relative(value: float, base: float) -> float:
    return (value - base) / abs(base)


# -- S1 ---------------------------------------------------------------------------------------------------------------
# Designed values (benchmark_designed.py): OD 330 mm by the rule; D/t 4 to 20 (the thick-wall end, beside G1's shear-dt
# series at D/t 10 to 100); a span of 1.2 m (L/OD 3.6, so the shear term is several percent of the deflection); a tip
# force of 200 kN downward, which dominates the weight of the span.
S1_OD, S1_SPAN, S1_TIP_FORCE, S1_DT = designed_od(330.0), 1.2, -200000.0, (4.0, 6.0, 10.0, 20.0)


def designed_builder(model_id: str) -> Builder:
    """A tranche 3 builder on the designed material (benchmark_designed.DESIGNED_MATERIAL)."""
    b = Builder(model_id, 0)
    b.material = DESIGNED_MATERIAL
    b.provenance = PROVENANCE
    return b


def build_s1(model_id: str, dt: float) -> dict[str, Any]:
    b = designed_builder(model_id)
    wall = designed_wall(S1_OD, dt)
    a, mid, tip = b.node("A", (0, 0, 0)), b.node("M", (S1_SPAN / 2, 0, 0)), b.node("B", (S1_SPAN, 0, 0))
    pipe = b.run("P", [a, mid, tip], S1_OD, wall)[0]
    anchor = b.anchor("A", a)
    b.force("tip", tip, "global_y", S1_TIP_FORCE)
    b.add_weight()
    _, e, nu, _, _ = b.material
    area, inertia = section_properties(S1_OD, wall)
    w, p, span = b.weight_per_length(pipe), S1_TIP_FORCE, S1_SPAN
    ei = e * inertia
    eb = p * bm.cr_pow(span, 3) / (3 * ei) + w * bm.cr_pow(span, 4) / (8 * ei)
    kappa = cowper_kappa(S1_OD, wall, nu)
    g = e / (2.0 * (1.0 + nu))
    shear = (p * span + w * bm.cr_pow(span, 2) / 2) / (kappa * g * area)
    support = (p + w * span) / K_TRANSLATION + (p * span + w * bm.cr_pow(span, 2) / 2) * span / K_ROTATION
    closed_forms = [
        closed("displacement", "y", SUSTAINED, eb, "m", "P L^3/(3EI) + w L^4/(8EI)", node=tip),
        closed("rotation", "z", SUSTAINED, p * bm.cr_pow(span, 2) / (2 * ei) + w * bm.cr_pow(span, 3) / (6 * ei), "rad",
               "P L^2/(2EI) + w L^3/(6EI)", node=tip),
        closed("reaction_force", "y", SUSTAINED, -(p + w * span), "N", "-(P + w L)", basis="statics; invented inputs",
               support=anchor),
        closed("reaction_moment", "z", SUSTAINED, -(p * span + w * bm.cr_pow(span, 2) / 2), "N*m", "-(P L + w L^2/2)",
               basis="statics; invented inputs", support=anchor),
    ]
    predictions = [{
        "quantity": "displacement", "component": "y", "load_case": SUSTAINED, "node": tip, "unit": "m",
        "values": {"euler_bernoulli": eb, "timoshenko": eb + shear,
                   "euler_bernoulli_with_stated_anchor_stiffness": eb + support},
        "relative_difference": {"timoshenko": _relative(eb + shear, eb),
                                "euler_bernoulli_with_stated_anchor_stiffness": _relative(eb + support, eb)},
        "shear_coefficient": kappa, "shear_coefficient_source": COWPER, "shear_modulus": g,
        "formula": {"euler_bernoulli": "P L^3/(3EI) + w L^4/(8EI)",
                    "timoshenko": "P L^3/(3EI) + w L^4/(8EI) + (P L + w L^2/2)/(kappa G A)",
                    "euler_bernoulli_with_stated_anchor_stiffness": ("P L^3/(3EI) + w L^4/(8EI) + (P + w L)/k_t + "
                                                                     "(P L + w L^2/2) L/k_r")},
        "statement": ("textbook predictions of two beam theories and of two support idealisations; SWBPIPE's "
                      "formulation for this document is Euler-Bernoulli with ideal anchors (closed_form). Neither "
                      "prediction states what the target program does: the series can refute either"),
    }]
    return {"model": b.model(), "closed_form": closed_forms, "predictions": predictions,
            "inexact": {SUSTAINED: [SHEAR, RIGID]}, "parameters": {"od": S1_OD, "dt": dt, "wall": wall, "span": span,
                                                                   "tip_force": p,
                                                                   "material": DESIGNED_MATERIAL[0]}}


# -- S2 ---------------------------------------------------------------------------------------------------------------
# Designed values: OD 330 mm by the rule; D/t 6, 15 and 40; tangents of 0.6 m and a bend radius of 1.5 OD, so the L is
# short beside its diameter and the shear and axial energies show; a temperature change of 150 K.
S2_OD, S2_TANGENT, S2_DELTA_T, S2_DT = designed_od(330.0), 0.6, 150.0, (6.0, 15.0, 40.0)
S2_RD = 1.5


def _arc_integral(f: tuple[Decimal, Decimal, Decimal], g: tuple[Decimal, Decimal, Decimal], pi: Decimal) -> Decimal:
    """integral over phi in [0, pi/2] of (f0 + fc cos + fs sin)(g0 + gc cos + gs sin) dphi."""
    f0, fc, fs = f
    g0, gc, gs = g
    return (f0 * g0 * pi / 2 + (f0 * gc + fc * g0) + (f0 * gs + fs * g0) + (fc * gc + fs * gs) * pi / 4
            + (fc * gs + fs * gc) / 2)


def _line_integral(f: tuple[Decimal, Decimal], g: tuple[Decimal, Decimal], length: Decimal) -> Decimal:
    """integral over s in [0, length] of (f0 + f1 s)(g0 + g1 s) ds."""
    return (f[0] * g[0] * length + (f[0] * g[1] + f[1] * g[0]) * length ** 2 / 2 + f[1] * g[1] * length ** 3 / 3)


def _solve3(matrix: list[list[Decimal]], rhs: list[Decimal]) -> list[Decimal]:
    rows = [row[:] + [value] for row, value in zip(matrix, rhs)]
    for col in range(3):
        pivot = max(range(col, 3), key=lambda r: abs(rows[r][col]))
        rows[col], rows[pivot] = rows[pivot], rows[col]
        for r in range(3):
            if r != col:
                factor = rows[r][col] / rows[col][col]
                rows[r] = [x - factor * y for x, y in zip(rows[r], rows[col])]
    return [rows[i][3] / rows[i][i] for i in range(3)]


def castigliano_l(a: float, b_len: float, radius: float, e: float, nu: float, alpha: float, delta_t: float,
                  od_mm: float, wall_mm: float, k: float, *, axial: bool, shear: bool,
                  support: tuple[float, float] | None = None) -> dict[str, float]:
    """Anchor reactions of the anchored L of S2 (module text): tangent a along +X from A at the origin, a 90-degree arc
    of radius R toward +Z, tangent b along +Z to B. Returns the supports' actions on the pipe in global axes: Fx, Fz
    and My at A and at B."""
    area, inertia = section_properties(od_mm, wall_mm)
    kappa = cowper_kappa(od_mm, wall_mm, nu)
    with localcontext() as context:
        context.prec = _PREC
        pi = bm._PI
        A_, B_, R = _d(a), _d(b_len), _d(radius)
        EI, EA = _d(e) * _d(inertia), _d(e) * _d(area)
        GA = _d(kappa) * _d(e) / (2 * (1 + _d(nu))) * _d(area)
        zero = Decimal(0)
        # Unit-load actions per segment: (moment, axial, transverse) for Q = (Fx, Fz, M) at B.
        tangent_b = [((-B_, Decimal(1)), (zero, zero), (Decimal(-1), zero)),
                     ((zero, zero), (Decimal(1), zero), (zero, zero)),
                     ((Decimal(1), zero), (zero, zero), (zero, zero))]
        arc = [((-B_, -R, zero), (zero, Decimal(1), zero), (zero, zero, Decimal(-1))),
               ((R, zero, -R), (zero, zero, Decimal(1)), (zero, Decimal(1), zero)),
               ((Decimal(1), zero, zero), (zero, zero, zero), (zero, zero, zero))]
        tangent_a = [((-(R + B_), zero), (Decimal(1), zero), (zero, zero)),
                     ((A_ + R, Decimal(-1)), (zero, zero), (Decimal(1), zero)),
                     ((Decimal(1), zero), (zero, zero), (zero, zero))]
        flex = [[zero] * 3 for _ in range(3)]
        for i in range(3):
            for j in range(3):
                total = zero
                for segment, length in ((tangent_b, B_), (tangent_a, A_)):
                    total += _line_integral(segment[i][0], segment[j][0], length) / EI
                    if axial:
                        total += _line_integral(segment[i][1], segment[j][1], length) / EA
                    if shear:
                        total += _line_integral(segment[i][2], segment[j][2], length) / GA
                total += R * _d(k) * _arc_integral(arc[i][0], arc[j][0], pi) / EI
                if axial:
                    total += R * _arc_integral(arc[i][1], arc[j][1], pi) / EA
                if shear:
                    total += R * _arc_integral(arc[i][2], arc[j][2], pi) / GA
                flex[i][j] = total
        dx, dz = A_ + R, R + B_
        if support is not None:
            kt, kr = _d(support[0]), _d(support[1])
            # Each anchor: F^2/(2 k_t) + M^2/(2 k_r). At B the actions are Q; at A they are -Q and the moment
            # -(M + dx Fz - dz Fx), whose unit-load values are (dz, -dx, -1) up to the common sign.
            at_a = (dz, -dx, Decimal(-1))
            for i in range(3):
                for j in range(3):
                    translation = (1 if i == j and i < 2 else 0) * 2 / kt
                    rotation = ((1 if i == j == 2 else 0) + at_a[i] * at_a[j]) / kr
                    flex[i][j] += translation + rotation
        strain = _d(alpha) * _d(delta_t)
        fx, fz, moment = _solve3(flex, [-strain * dx, -strain * dz, zero])
        moment_a = -(moment + dx * fz - dz * fx)
        # The plane moment m (x to z sense) is the global moment -m about Y.
        return {"A_Fx": float(-fx), "A_Fz": float(-fz), "A_My": float(-moment_a),
                "B_Fx": float(fx), "B_Fz": float(fz), "B_My": float(-moment)}


def build_s2(model_id: str, dt: float) -> dict[str, Any]:
    b = designed_builder(model_id)
    wall = designed_wall(S2_OD, dt)
    radius = S2_RD * S2_OD / 1000.0
    a = b.node("A", (0, 0, 0))
    ti = (S2_TANGENT + radius, 0.0, 0.0)  # tangent length R for 90 degrees
    near, far, _, end = corner_bend(b, "BEND1", a, ti, (0.0, 0.0, 1.0), radius, S2_TANGENT, 1.0, S2_OD, wall)
    tip = b.node("B", end)
    b.pipe("P1", a, near, S2_OD, wall)
    b.pipe("P2", far, tip, S2_OD, wall)
    anchor_a, anchor_b = b.anchor("A", a), b.anchor("B", tip)
    b.add_thermal(S2_DELTA_T)
    _, e, nu, alpha, _ = b.material
    common = dict(a=S2_TANGENT, b_len=S2_TANGENT, radius=radius, e=e, nu=nu, alpha=alpha, delta_t=S2_DELTA_T,
                  od_mm=S2_OD, wall_mm=wall, k=1.0)
    sets = {"bending": castigliano_l(**common, axial=False, shear=False),
            "bending_axial": castigliano_l(**common, axial=True, shear=False),
            "bending_axial_shear": castigliano_l(**common, axial=True, shear=True),
            "bending_axial_with_stated_anchor_stiffness": castigliano_l(**common, axial=True, shear=False,
                                                                        support=(K_TRANSLATION, K_ROTATION))}
    swb = sets["bending_axial"]
    formula = "Castigliano, bending and axial strain energy, ideal anchors (module text)"
    targets = (("reaction_force", "x", "A_Fx", anchor_a, "N"), ("reaction_force", "z", "A_Fz", anchor_a, "N"),
               ("reaction_moment", "y", "A_My", anchor_a, "N*m"), ("reaction_force", "x", "B_Fx", anchor_b, "N"),
               ("reaction_force", "z", "B_Fz", anchor_b, "N"), ("reaction_moment", "y", "B_My", anchor_b, "N*m"))
    closed_forms = [closed(quantity, component, EXPANSION, swb[key], unit, formula, basis=CASTIGLIANO + "; invented "
                           "inputs", support=support) for quantity, component, key, support, unit in targets]
    predictions = [{
        "quantity": quantity, "component": component, "load_case": EXPANSION, "support": support, "unit": unit,
        "values": {name: values[key] for name, values in sets.items()},
        "relative_difference": {name: _relative(values[key], swb[key]) for name, values in sets.items()
                                if name != "bending_axial"},
        "basis": CASTIGLIANO, "shear_coefficient": cowper_kappa(S2_OD, wall, nu), "shear_coefficient_source": COWPER,
        "statement": ("textbook predictions of four strain-energy sets on one curved-beam idealisation; relative "
                      "differences are against bending_axial, SWBPIPE's formulation for this document. The predicted "
                      "difference of adding shear is bending_axial_shear's; none states what the target program does"),
    } for quantity, component, key, support, unit in targets]
    return {"model": b.model(), "closed_form": closed_forms, "predictions": predictions,
            "inexact": {EXPANSION: [SHEAR, BEND_K, BEND_SHEAR, RIGID]},
            "parameters": {"od": S2_OD, "dt": dt, "wall": wall, "rd": S2_RD, "bend_radius": radius,
                           "tangent": S2_TANGENT, "angle": 90.0, "k": 1.0, "delta_t": S2_DELTA_T,
                           "material": DESIGNED_MATERIAL[0]}}


# -- S3 ---------------------------------------------------------------------------------------------------------------
# Designed values: OD 620 mm by the rule and D/t 10, a stout section so that 2 E A/(k_t L) is visible; lengths from
# 0.5 m to 40 m, so the predicted axial reduction runs from clearly above the exact class's 1e-4 to negligible; a
# temperature change of 150 K.
S3_OD, S3_DT, S3_DELTA_T, S3_LENGTHS = designed_od(620.0), 10.0, 150.0, (0.5, 1.0, 2.5, 10.0, 40.0)


def build_s3(model_id: str, length: float) -> dict[str, Any]:
    b = designed_builder(model_id)
    wall = designed_wall(S3_OD, S3_DT)
    a, mid, end = b.node("A", (0, 0, 0)), b.node("M", (length / 2, 0, 0)), b.node("B", (length, 0, 0))
    pipe = b.run("P", [a, mid, end], S3_OD, wall)[0]
    anchor_a, anchor_b = b.anchor("A", a), b.anchor("B", end)
    b.add_weight()
    b.add_thermal(S3_DELTA_T)
    _, e, _, alpha, _ = b.material
    area, inertia = section_properties(S3_OD, wall)
    w = b.weight_per_length(pipe)
    axial = e * area * alpha * S3_DELTA_T
    moment = -w * bm.cr_pow(length, 2) / 12
    axial_ratio = 2 * e * area / (K_TRANSLATION * length)
    moment_ratio = 2 * e * inertia / (K_ROTATION * length)
    closed_forms = [
        closed("reaction_force", "x", EXPANSION, axial, "N", "E A alpha dT (support on pipe at the start)",
               support=anchor_a),
        closed("reaction_force", "x", EXPANSION, -axial, "N", "-E A alpha dT (support on pipe at the end)",
               support=anchor_b),
        closed("reaction_force", "y", SUSTAINED, -w * length / 2, "N", "-w L/2", basis="statics; invented inputs",
               support=anchor_a),
        closed("reaction_force", "y", SUSTAINED, -w * length / 2, "N", "-w L/2", basis="statics; invented inputs",
               support=anchor_b),
        closed("reaction_moment", "z", SUSTAINED, moment, "N*m", "-w L^2/12 (support on pipe at the start)",
               support=anchor_a),
        closed("reaction_moment", "z", SUSTAINED, -moment, "N*m", "w L^2/12 (support on pipe at the end)",
               support=anchor_b),
        closed("displacement", "y", SUSTAINED, w * bm.cr_pow(length, 4) / (384 * e * inertia), "m",
               "w L^4/(384 E I) at midspan", node=mid),
    ]
    finite_axial = axial / (1 + axial_ratio)
    finite_moment = moment / (1 + moment_ratio)
    predictions = [
        {"quantity": "reaction_force", "component": "x", "load_case": EXPANSION, "support": anchor_a, "unit": "N",
         "values": {"ideal_anchors": axial, "stated_anchor_stiffness": finite_axial},
         "relative_difference": _relative(finite_axial, axial),
         "formula": "E A alpha dT / (1 + 2 E A/(k_t L))", "ratio_2EA_over_ktL": axial_ratio},
        {"quantity": "reaction_moment", "component": "z", "load_case": SUSTAINED, "support": anchor_a, "unit": "N*m",
         "values": {"ideal_anchors": moment, "stated_anchor_stiffness": finite_moment},
         "relative_difference": _relative(finite_moment, moment),
         "formula": "-(w L^2/12) / (1 + 2 E I/(k_r L))", "ratio_2EI_over_krL": moment_ratio},
    ]
    return {"model": b.model(), "closed_form": closed_forms, "predictions": predictions,
            "inexact": {SUSTAINED: [SHEAR, RIGID], EXPANSION: [RIGID]},
            "parameters": {"od": S3_OD, "dt": S3_DT, "wall": wall, "length": length, "delta_t": S3_DELTA_T,
                           "material": DESIGNED_MATERIAL[0]}}


S_SERIES = {
    "S1": ("shear-thick-wall", "dt", S1_DT, build_s1, SHEAR,
           "transverse shear in straight pipe: short thick cantilevers under weight and a tip force, one outside "
           "diameter and span, differing only in wall thickness"),
    "S2": ("bend-shear-axial-wall", "dt", S2_DT, build_s2, BEND_SHEAR,
           "shear and flexibility in a bend: one realized 90-degree bend with user k 1 between short tangents, both "
           "ends anchored, thermal; differing only in wall thickness"),
    "S3": ("rigid-support-length", "length", S3_LENGTHS, build_s3, RIGID,
           "finite stiffness of a rigid support: a straight pipe anchored at both ends under weight and a temperature "
           "change, differing only in length"),
}
SERIES_TEXT = {
    "shear-thick-wall": (
        "S1 complements the tranche 1 series shear-dt (OD 219.1 mm, span 3 m, L/OD 13.7, D/t 10 to 100, weight, tip "
        "force and thermal): S1 takes the thick-wall end (D/t 4 to 20) at L/OD 3.7 with a tip force that dominates the "
        "weight, so the shear term is several percent of the deflection and the dependence of the shear coefficient on "
        "the wall ratio is resolved well above the printed resolution; shear-dt measures the same input where it is "
        "near the named-inexact tolerance. HYPOTHESIS, which the series can refute: a program whose beams carry "
        "transverse shear with a hollow-circle coefficient shows each member's tip deflection larger than the "
        "Euler-Bernoulli value by about the predicted relative difference; one without shear shows none. The predicted "
        "fraction grows as the wall thins at this diameter (module text)."),
    "bend-shear-axial-wall": (
        "HYPOTHESIS, which the series can refute: the anchor reactions of each member lie near one of the four "
        "strain-energy predictions (bending; bending and axial; bending, axial and shear; bending and axial with the "
        "stated anchor stiffness), and the member-to-member trend of the difference identifies which energy terms a "
        "program carries. The flexibility factor is 1 on both sides; whether either program applies another factor to "
        "the bend is the named input bend_flexibility_application."),
    "rigid-support-length": (
        "HYPOTHESIS, which the series can refute: a program whose rigid anchor is the stated finite stiffness shows "
        "the expansion axial force reduced by the predicted factor 1/(1 + 2 E A/(k_t L)) and the sustained end moment by "
        "1/(1 + 2 E I/(k_r L)); one with ideal anchors shows neither. The lengths take the axial reduction from about "
        "5e-4, clearly above the exact class's 1e-4, down to about 6e-6, negligible. DECLARED BEFORE ANY RUN: a later "
        "SWBPIPE side that states the vendor's stiffness explicitly, as springs, may change a row's numerical standing, "
        "because a very stiff finite support worsens the system's conditioning; that would be reported as its own "
        "finding with the stiffness named, and it would say nothing about agreement. That side is not built. This "
        "series is also the convention series for triage of anchored cases in tranches 1 and 2, whose files cannot name "
        "this input without changing their bytes."),
}


# -- family D (dynamics) --------------------------------------------------------------------------------------------
QUOTES = Path(__file__).resolve().parent / "dynamics_reference_quotes.json"
D_PROV = ("invented CAEPIPE benchmark input (DEL-17-05; dynamics family draft, DEL-04-09 and DEL-04-10); "
          "not library, component, catalog, standard or code-rule data")
D_FOLDED = ("folded into the seeded generator from the dynamics lane's wrapper files, dynamics family draft 4, "
            "2026-11-12: the model members by the same arithmetic, byte for byte; the reference values quoted from "
            "tools/caepipe_correlation/dynamics_reference_quotes.json")
G = 9.80665
OD = 0.0603
WALL = 0.00391
LENGTH = 15.0
RHO = 7850.0
E = 200.0e9
G_SHEAR = 78.125e9
ID = OD - 2.0 * WALL
AREA = math.pi / 4.0 * (OD * OD - ID * ID)
MBAR = RHO * AREA
WEIGHT = RHO * AREA * G
RADIUS = 1.5 * OD
AXES = {"x": (1.0, 0.0, 0.0), "y": (0.0, 1.0, 0.0), "z": (0.0, 0.0, 1.0)}


def _q(value, unit):
    return {"unit": unit, "value": value}


def _dyn_pipe(pid, start, end, yref):
    return {"from": start, "to": end, "id": pid, "material": "material:dyn-a", "provenance": D_PROV,
            "section": {"outside_diameter": _q(OD, "m"), "wall_thickness": _q(WALL, "m"),
                        "material_density": _q(RHO, "kg/m^3")},
            "y_reference": yref}


def _dyn_weight(pid):
    return {"category": "weight", "dimension": "force_per_length", "direction": "global_y", "id": f"load:weight:{pid}",
            "magnitude": _q(-WEIGHT, "N/m"),
            "provenance": D_PROV + "; w = -rho A g with g = 9.80665 m/s2 and A the bare metal area",
            "target": {"pipe": pid, "type": "element"}}


def _dyn_model(model_id, nodes, pipes, components, loads, supports):
    return {
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "professional_acceptance": "not_provided",
                            "rule_check": "not_performed_user_rule_inputs_missing"},
        "combinations": [], "components": components, "document_kind": "openpipestress.product_preview.model",
        "load_cases": [{"id": "case:weight", "provenance": D_PROV, "primitive_loads": loads}],
        "materials": [{"id": "material:dyn-a", "elastic_modulus": _q(E, "Pa"), "shear_modulus": _q(G_SHEAR, "Pa"),
                       "temperature_points": [], "thermal_expansion_coefficient": _q(1.2e-05, "1/degC"),
                       "provenance": D_PROV}],
        "nodes": nodes, "pipe_segments": pipes,
        "project": {"id": f"project:{model_id}", "name": f"project:{model_id}",
                    "units": {"angle": "rad", "force": "N", "length": "m", "pressure": "Pa", "stress": "Pa",
                              "temperature": "degC"}},
        "schema_version": "0.9.0",
        "supports": supports,
    }


def straight_model(model_id: str, members: int) -> dict[str, Any]:
    names = ["A"] + [f"M{i}" for i in range(1, members)] + ["B"]
    nodes = [{"id": f"node:{name}", "position": {"x": LENGTH * i / members, "y": 0.0, "z": 0.0}, "provenance": D_PROV}
             for i, name in enumerate(names)]
    pipes, loads = [], []
    for i in range(members):
        pid = f"pipe:P{i + 1}"
        pipes.append(_dyn_pipe(pid, f"node:{names[i]}", f"node:{names[i + 1]}", {"x": 0.0, "y": 1.0, "z": 0.0}))
        loads.append(_dyn_weight(pid))
    supports = [{"family": "anchor", "id": "support:A", "node": "node:A", "provenance": D_PROV,
                 "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"]},
                {"id": "support:B-z", "node": "node:B", "provenance": D_PROV, "restraints": ["UZ"]}]
    return _dyn_model(model_id, nodes, pipes, [], loads, supports)


def _add(a, b, s=1.0):
    return tuple(a[i] + s * b[i] for i in range(3))


def _pos(p):
    return {"x": p[0], "y": p[1], "z": p[2]}


def piping_model(model_id: str, legs, members) -> dict[str, Any]:
    nodes, pipes, comps, loads = [], [], [], []

    def node(name, p):
        nodes.append({"id": f"node:{name}", "position": _pos(p), "provenance": D_PROV})

    def pipe(pid, a, b, yref):
        pipes.append(_dyn_pipe(pid, f"node:{a}", f"node:{b}", _pos(yref)))
        loads.append(_dyn_weight(pid))

    dirs = [tuple(sign * c for c in AXES[axis]) for axis, sign, _ in legs]
    corner, prev, count = (0.0, 0.0, 0.0), "A", 0
    node("A", corner)
    for leg, ((axis, _, length), d, n) in enumerate(zip(legs, dirs, members), 1):
        first, last = leg == 1, leg == len(legs)
        begin = corner if first else _add(corner, d, RADIUS)
        nxt = _add(corner, d, length)
        end = nxt if last else _add(nxt, d, -RADIUS)
        yref = AXES["x"] if axis == "y" else AXES["y"]
        for i in range(1, n + 1):
            count += 1
            t = i / n
            p = tuple(begin[j] + t * (end[j] - begin[j]) for j in range(3))
            name = ("B" if last else f"BEND{leg}-near") if i == n else f"L{leg}M{i}"
            node(name, p)
            pipe(f"pipe:P{count}", prev, name, yref)
            prev = name
        if not last:
            d2 = dirs[leg]
            far = _add(nxt, d2, RADIUS)
            node(f"BEND{leg}-far", far)
            out = tuple((d[j] - d2[j]) / math.sqrt(2.0) for j in range(3))
            pipe(f"pipe:BEND{leg}", f"BEND{leg}-near", f"BEND{leg}-far", out)
            comps.append({
                "geometry": {"bend_angle": _q(math.pi / 2.0, "rad"),
                             "bend_geometry_source_reference": "invented benchmark bend (dynamics family draft)",
                             "bend_pipe_ref": f"pipe:BEND{leg}", "bend_radius": _q(RADIUS, "m")},
                "id": f"component:BEND{leg}", "kind": "bend",
                "mechanics_interface": {"solver_consumption": "curved_bend_macro_element"},
                "modifiers": {"flexibility_factor_user_value": _q(1.0, "none"),
                              "source_reference": "invented benchmark value (dynamics family draft)"},
                "node": f"node:BEND{leg}-near", "provenance": D_PROV})
            prev = f"BEND{leg}-far"
        corner = nxt
    supports = [{"family": "anchor", "id": f"support:{n}", "node": f"node:{n}", "provenance": D_PROV,
                 "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"]} for n in ("A", "B")]
    return _dyn_model(model_id, nodes, pipes, comps, loads, supports)


# Euler-Bernoulli continuum frequencies, correctly rounded (decimal Newton iteration; no platform libm).
def _dec_trig(x: Decimal) -> tuple[Decimal, Decimal, Decimal, Decimal]:
    sin, cos = bm._taylor_sin_cos(x)
    ex = x.exp()
    return sin, cos, (ex - 1 / ex) / 2, (ex + 1 / ex) / 2


def _root(kind: str, start: str) -> Decimal:
    x = Decimal(start)
    for _ in range(60):
        sin, cos, sinh, cosh = _dec_trig(x)
        if kind == "cantilever":  # 1 + cos x cosh x = 0
            f, df = 1 + cos * cosh, -sin * cosh + cos * sinh
        else:  # tan x = tanh x: sin x cosh x - cos x sinh x = 0
            f, df = sin * cosh - cos * sinh, 2 * sin * sinh
        x -= f / df
    return x


def eb_frequencies(kind: str) -> list[float]:
    starts = {"cantilever": ("1.875", "4.694", "7.855"), "propped": ("3.9266", "7.0686", "10.2102")}[kind]
    out = []
    with localcontext() as context:
        context.prec = _PREC + 10
        od, inner = _d(OD), _d(OD) - 2 * _d(WALL)
        inertia = bm._PI / 64 * (od ** 4 - inner ** 4)
        mass = _d(RHO) * bm._PI / 4 * (od ** 2 - inner ** 2)
        scale = (_d(E) * inertia / (mass * _d(LENGTH) ** 4)).sqrt() / (2 * bm._PI)
        for start in starts:
            x = _root(kind, start)
            out.append(float(x * x * scale))
    return out


FLAT = {"id": "spectrum:dyn-flat", "abscissa": "frequency", "abscissa_unit": "Hz",
        "ordinate": "pseudo_spectral_acceleration", "ordinate_unit": "m/s^2",
        "ordinate_other_program": "acceleration in mm/sec2 (multiply each ordinate by 1000)",
        "interpolation": {"abscissa": "linear", "ordinate": "linear"}, "damping_ratio": 0.05,
        "points": [[0.125, 2.5], [1000.0, 2.5]]}
TWO = {"id": "spectrum:dyn-two", "abscissa": "frequency", "abscissa_unit": "Hz",
       "ordinate": "pseudo_spectral_acceleration", "ordinate_unit": "m/s^2",
       "ordinate_other_program": "acceleration in mm/sec2 (multiply each ordinate by 1000)",
       "interpolation": {"abscissa": "linear", "ordinate": "linear"}, "damping_ratio": 0.05,
       "points": [[0.125, 1.5], [2.0, 4.5], [16.0, 2.25]]}
SPECTRUM_NAMES = {"spectrum:dyn-flat": "DYNFLAT", "spectrum:dyn-two": "DYNTWO"}


def _identity_forms() -> list[dict]:
    return [
        {"basis": "section identity; invented inputs", "quantity": "metal_area", "formula": "pi/4 (OD^2 - (OD - 2t)^2)",
         "unit": "m^2", "value": AREA},
        {"basis": "weight identity; invented inputs", "quantity": "weight_per_length", "formula": "rho A g, g = 9.80665",
         "unit": "N/m", "value": WEIGHT},
    ]


def straight_closed_form() -> list[dict]:
    out = _identity_forms() + [{"basis": "mass identity; invented inputs", "quantity": "total_mass",
                                "formula": "rho A L", "unit": "kg", "value": MBAR * LENGTH}]
    for n, value in enumerate(eb_frequencies("cantilever"), 1):
        out.append({"basis": "Euler-Bernoulli continuum, cantilever (XY plane); the limit of the member series",
                    "quantity": "frequency", "plane": "XY", "plane_mode": n,
                    "formula": "x^2/(2 pi) sqrt(EI/(rho A L^4)), 1 + cos x cosh x = 0", "unit": "Hz", "value": value})
    for n, value in enumerate(eb_frequencies("propped"), 1):
        out.append({"basis": "Euler-Bernoulli continuum, propped cantilever (XZ plane); the limit of the member series",
                    "quantity": "frequency", "plane": "XZ", "plane_mode": n,
                    "formula": "x^2/(2 pi) sqrt(EI/(rho A L^4)), tan x = tanh x", "unit": "Hz", "value": value})
    return out


REFERENCE_NOTE = ("from validation/references/f2_spectrum/benchmark_pack/pack_summary.json: references with a "
                  "refutation record, in pull request 268 and not yet on main. continuum = Rayleigh continuum closed "
                  "form; consistent = SWBPIPE's stated formulation at this member count; lumped = half-and-half "
                  "translational lumped mass at this member count, NOT SWBPIPE's formulation, the size of the named "
                  "inexact input mass_formulation. Each side's periodic set is the first six modes of its own merged "
                  "list, or all its finite modes where fewer exist")
MODE_COUNT_NOTE = ("the counts differ between the sides where a diagonal mass formulation has fewer finite modes: at 1 "
                   "member it has the Y bending mode and one axial mode (2) against 5 on the consistent side; at 2 "
                   "members 5 (three bending, two axial) against the first 6")
NOTE_CAPTURE = ("the capture also takes every other mode sum and direction sum the program offers, as declared "
                "variants of this file; the comparable setting is the one stated here")


def straight_wrapper(rank, model_id, members, table, modes_other, modes_swb, missing, rigid, quotes):
    directions = [{"direction": "Y", "spectrum": table["id"], "scale_factor": 1.0}]
    if members > 1:
        directions.append({"direction": "Z", "spectrum": table["id"], "scale_factor": 1.0})
    request = {
        "other_program_settings": {
            "cutoff_frequency_hz": 900.0 if table is FLAT else 15.0, "number_of_modes": modes_other,
            "include_missing_mass": missing, "spectrum_mode_sum": "SRSS", "spectrum_direction_sum": "SRSS",
            "gravitational_acceleration_m_per_s2": G, "gravitational_direction": "-Y", "note": NOTE_CAPTURE},
        "swbpipe_modal_request": {"mode_count": modes_swb, "frequency_cutoff": None},
        "swbpipe_spectrum_request": {
            "directions": directions, "modal_combination": "srss", "directional_combination": "srss",
            "missing_mass": {"apply": bool(missing), "rigid_frequency_hz": rigid,
                             "rigid_combination_series": ["srss", "absolute_sum"] if missing else []}},
        "spectra": [copy.deepcopy(table)],
        "conditions": [
            "every extracted mode and the rigid frequency lie strictly inside the table (SWBPIPE refuses outside it; "
            "the other program uses the end value)",
        ] + ([
            "the table is flat above the last periodic mode, so the ordinate at the rigid frequency and the maximum "
            "ordinate after the last extracted mode are the same number",
            "SWBPIPE extracts one mode more than the other program: its extraction must reach the rigid frequency, and "
            "that further mode is rigid and enters the static solve; the periodic modes are the same on both sides",
        ] if missing else []),
    }
    return {
        "document_kind": bm.MODEL_KIND, "rank": rank, "model_id": model_id, "family": "DYN-C",
        "family_description": "one straight pipe along X, anchored at A, UZ held at the tip B: a cantilever in the XY "
                              "plane and a propped cantilever in the XZ plane; no repeated mode",
        "parameters": {"members": members, "length": LENGTH, "od": OD, "wall": WALL, "rho": RHO,
                       "metal_area": AREA, "weight_per_length": WEIGHT, "g": G, "elastic_modulus": E,
                       "shear_modulus": G_SHEAR, "slenderness_length_over_od": LENGTH / OD},
        "inexact_inputs": {"dynamic": ["mass_formulation", "shear_deformation"]
                           + (["rigid_combination"] if missing else [])},
        "inexact_input_sizes": {
            "shear_deformation": "ESTIMATE, not a reference value: SWBPIPE has no shear deformation in dynamics; if "
                                 "the other program includes it, under 1e-3 relative on every compared frequency at "
                                 "this slenderness (about five times the Rayleigh rotary term)",
            "mass_formulation": "per member count and response kind in validation/references/f2_spectrum "
                                "(closed_form/screening_basis.json and beams_fe.json; pull request 268, not yet on main)",
        },
        "series": ["members"], "sampling": "series",
        "third_value": "closed form (continuum) in the limit of the member series; per member count and per mass "
                       "formulation in validation/references/f2_spectrum (pull request 268, not yet on main)",
        "dynamic_request": request,
        "compared_quantities": [
            {"quantity": "frequency", "bind": "mode order, with the plane of each mode"},
            {"quantity": "displacement", "node": "node:B", "component": "y", "from_direction": "Y"},
            {"quantity": "reaction_force", "support": "support:A", "component": "y", "from_direction": "Y"},
            {"quantity": "reaction_moment", "support": "support:A", "component": "z", "from_direction": "Y"},
        ] + ([
            {"quantity": "reaction_force", "support": "support:A", "component": "z", "from_direction": "Z"},
            {"quantity": "reaction_moment", "support": "support:A", "component": "y", "from_direction": "Z"},
            {"quantity": "reaction_force", "support": "support:B-z", "component": "z", "from_direction": "Z"},
        ] if members > 1 else []),
        "closed_form": straight_closed_form(),
        "reference_values": copy.deepcopy(quotes[model_id]["reference_values"]),
        "reference_values_note": REFERENCE_NOTE,
        "mode_count_note": MODE_COUNT_NOTE,
        "model": straight_model(model_id, members),
        "piping_code": bm.PIPING_CODE, "reference_temperature_c": bm.REFERENCE_TEMPERATURE_C,
        "provenance": {"contributor_note": D_PROV, "license": "project-original", "origin": "invented",
                       "produced_by_caepipe": False, "redistribution_status": "public_permissive"},
    }


def diagonal_wrapper(quotes) -> dict[str, Any]:
    model_id, members, k = "cpb-dyn-d-n08-flat", 8, 8.0
    doc = straight_wrapper(5, model_id, members, FLAT, 6, 6, 0, None, quotes)
    step = LENGTH / members / math.sqrt(3.0)
    for i, node in enumerate(doc["model"]["nodes"]):
        node["position"] = {"x": step * i, "y": step * i, "z": step * i}
    doc["model"]["supports"] = [
        {"family": "anchor", "id": "support:A", "node": "node:A", "provenance": D_PROV,
         "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"]},
        {"family": "spring", "id": "support:B-x", "node": "node:B", "provenance": D_PROV, "restraints": ["UX"],
         "stiffness": {"dof": "UX", "value": _q(k, "N/m")}},
    ]
    doc["family"] = "DYN-D"
    doc["family_description"] = ("the case C pipe along the space diagonal, anchored at A, one linear spring along "
                                 "global X at the tip B: two simple bending modes about 5 percent apart, both excited "
                                 "by a spectrum along Y")
    doc["parameters"]["tip_spring_x"] = k
    doc["series"], doc["sampling"] = [], "single"
    doc["inexact_inputs"] = {"dynamic": ["mass_formulation", "shear_deformation"]}
    doc["dynamic_request"]["swbpipe_spectrum_request"]["directions"] = [
        {"direction": "Y", "spectrum": FLAT["id"], "scale_factor": 1.0}]
    doc["dynamic_request"]["conditions"].append(
        "the two lowest modes are about 5 percent apart: this is the case on which the absolute and closely spaced "
        "mode sums differ from SRSS, so every mode sum the other program offers is meaningful here")
    doc["compared_quantities"] = (
        [{"quantity": "frequency", "bind": "mode order"}]
        + [{"quantity": "displacement", "node": "node:B", "component": c, "from_direction": "Y"} for c in "xyz"]
        + [{"quantity": "reaction_force", "support": "support:A", "component": c, "from_direction": "Y"} for c in "xyz"]
        + [{"quantity": "reaction_moment", "support": "support:A", "component": c, "from_direction": "Y"} for c in "xyz"]
        + [{"quantity": "reaction_force", "support": "support:B-x", "component": "x", "from_direction": "Y"}])
    doc["closed_form"] = straight_closed_form()[:3]
    doc["third_value"] = ("a discrete reference on SWBPIPE's stated formulation at this member count, quoted in "
                          "discrete_reference_values, and a continuum closed form to within a small stated axial "
                          "coupling in reference_values")
    doc["discrete_reference_values"] = copy.deepcopy(quotes[model_id]["discrete_reference_values"])
    doc["member_frame"] = {
        "y_reference": "global Y on every member",
        "x": [1 / math.sqrt(3.0)] * 3,
        "y": [-1 / math.sqrt(6.0), 2 / math.sqrt(6.0), -1 / math.sqrt(6.0)],
        "z": [-1 / math.sqrt(2.0), 0.0, 1 / math.sqrt(2.0)],
        "note": "x from the near node to the far node, y = y_reference made normal to x, z = x cross y. The pipe is "
                "round, so every compared quantity (frequencies, global displacements, global support loads) is "
                "independent of this choice. Element end forces in member axes are OUTSIDE the like-for-like set: "
                "they depend on each program's local axis rule, and the discrete reference tabulates them in a "
                "different member frame (y_reference global Z)",
    }
    return doc


def piping_wrapper(rank, model_id, family, description, legs, members, directions, modes, quotes):
    table = FLAT
    arc = math.pi / 2.0 * RADIUS
    straight = sum(length for _, _, length in legs) - 2.0 * RADIUS * (len(legs) - 1)
    developed = straight + arc * (len(legs) - 1)
    request = {
        "other_program_settings": {
            "cutoff_frequency_hz": 900.0, "number_of_modes": modes, "include_missing_mass": 0,
            "spectrum_mode_sum": "SRSS", "spectrum_direction_sum": "SRSS",
            "gravitational_acceleration_m_per_s2": G, "gravitational_direction": "-Y", "note": NOTE_CAPTURE},
        "swbpipe_modal_request": {"mode_count": modes, "frequency_cutoff": None},
        "swbpipe_spectrum_request": {
            "directions": [{"direction": d, "spectrum": table["id"], "scale_factor": s} for d, s in directions],
            "modal_combination": "srss", "directional_combination": "srss",
            "missing_mass": {"apply": False, "rigid_frequency_hz": None, "rigid_combination_series": []}},
        "spectra": [copy.deepcopy(table)],
        "conditions": ["every extracted mode lies strictly inside the table (SWBPIPE refuses outside it; the other "
                       "program uses the end value)",
                       "the mode count cuts at a wide gap, so that a different mass formulation does not change which "
                       "modes are extracted"],
    }
    return {
        "document_kind": bm.MODEL_KIND, "rank": rank, "model_id": model_id, "family": family,
        "family_description": description,
        "parameters": {"legs": [{"axis": a, "sign": s, "corner_to_corner_length": length, "members": n}
                                for (a, s, length), n in zip(legs, members)],
                       "bend_radius": RADIUS, "rd": 1.5, "k": 1.0, "od": OD, "wall": WALL, "rho": RHO,
                       "metal_area": AREA, "weight_per_length": WEIGHT, "g": G, "elastic_modulus": E,
                       "shear_modulus": G_SHEAR, "developed_length": developed, "total_mass": MBAR * developed},
        "inexact_inputs": {"dynamic": ["mass_formulation", "bend_mass_formulation", "shear_deformation",
                                       "bend_flexibility_application", "bend_transverse_shear"]},
        "inexact_input_sizes": {
            "mass_formulation": "no prediction: this layout has no closed form. On the straight case C the "
                                "difference between a consistent and a lumped mass at members of this length is "
                                "tabulated in validation/references/f2_spectrum",
            "bend_mass_formulation": "SWBPIPE carries a realized bend's mass as two rigid half-arc bodies, each with "
                                     "its half-arc's exact mass, centroid and second moments; each bend here is "
                                     f"{arc / developed:.4f} of the developed length",
            "shear_deformation": "ESTIMATE, not a reference value: SWBPIPE has no shear deformation in dynamics; the "
                                 "shortest leg here is over 30 outside diameters",
        },
        "series": [], "sampling": "single",
        "mode_count_basis": copy.deepcopy(quotes[model_id].get("mode_count_basis")),
        "third_value": "NONE. No closed form exists for this layout and no independent reference has been generated "
                       "for it, so agreement here compares two programs and is not a check against a third value",
        "dynamic_request": request,
        "compared_quantities": [
            {"quantity": "frequency", "bind": "mode order, with SWBPIPE's cluster identifier"},
            {"quantity": "displacement", "node": "every node", "component": "x, y, z"},
            {"quantity": "reaction_force", "support": "support:A and support:B", "component": "x, y, z (global)"},
            {"quantity": "reaction_moment", "support": "support:A and support:B", "component": "x, y, z (global)"},
        ],
        "closed_form": _identity_forms() + [
            {"basis": "mass identity; invented inputs; straights plus quarter arcs", "quantity": "total_mass",
             "formula": "rho A (sum of legs - 2 R (legs - 1) + (pi/2) R (legs - 1))", "unit": "kg",
             "value": MBAR * developed}],
        "model": piping_model(model_id, legs, members),
        "piping_code": bm.PIPING_CODE, "reference_temperature_c": bm.REFERENCE_TEMPERATURE_C,
        "provenance": {"contributor_note": D_PROV, "license": "project-original", "origin": "invented",
                       "produced_by_caepipe": False, "redistribution_status": "public_permissive"},
    }


STRAIGHT_CASES = (
    # rank, id, members, table, modes (other program), modes (SWBPIPE), missing mass, rigid frequency
    (1, "cpb-dyn-c-n08-flat", 8, FLAT, 6, 6, 0, None),
    (2, "cpb-dyn-c-n02-flat", 2, FLAT, 5, 6, 0, None),
    (3, "cpb-dyn-c-n16-flat", 16, FLAT, 6, 6, 0, None),
    (4, "cpb-dyn-c-n08-trunc", 8, FLAT, 2, 3, 1, 1.25),
    (6, "cpb-dyn-c-n08-two", 8, TWO, 6, 6, 0, None),
    (8, "cpb-dyn-c-n04-flat", 4, FLAT, 6, 6, 0, None),
    (10, "cpb-dyn-c-n01-flat", 1, FLAT, 2, 5, 0, None),
)
PIPING_CASES = (
    (7, "cpb-dyn-m-flat", "DYN-M",
     "a three-leg run in three planes (X, then up Y, then Z) with two realized 90 degree bends, anchored at both "
     "ends, under spectra in three directions",
     [("x", 1, 3.0), ("y", 1, 2.0), ("z", 1, 2.5)], [4, 3, 4], [("X", 1.0), ("Y", 0.8), ("Z", 0.6)], 7),
    (9, "cpb-dyn-l-flat", "DYN-L",
     "a horizontal L (X, then Z) with one realized 90 degree bend, anchored at both ends, under spectra along X "
     "(in the plane) and Y (out of it)",
     [("x", 1, 4.0), ("z", 1, 3.0)], [4, 3], [("X", 1.0), ("Y", 1.0)], 7),
)
# Static inputs of each dynamics model's weight case, added beside the wrapper's dynamic list (the wrappers name none
# for the static case): a cantilever, propped cantilever or diagonal pipe under weight bends, so shear deformation is
# named; the piping layouts have bends; every case has an anchor.
D_STATIC_INEXACT = {"DYN-C": [SHEAR, RIGID], "DYN-D": [SHEAR, RIGID],
                    "DYN-L": [SHEAR, BEND_K, BEND_SHEAR, RIGID], "DYN-M": [SHEAR, BEND_K, BEND_SHEAR, RIGID]}


def dynamics_documents() -> list[dict[str, Any]]:
    """The ten dynamics documents in rank order (without the generator's common members)."""
    quotes = json.loads(QUOTES.read_text(encoding="utf-8"))["models"]
    docs = [straight_wrapper(*case, quotes) for case in STRAIGHT_CASES]
    docs.append(diagonal_wrapper(quotes))
    docs += [piping_wrapper(*case, quotes) for case in PIPING_CASES]
    for doc in docs:
        static = list(D_STATIC_INEXACT[doc["family"]])
        doc["inexact_inputs"].update({"case:weight": static, OPERATING: static})
        doc["comparison_conventions"] = D_CONVENTIONS + (D_DIAGONAL_CONVENTIONS if doc["family"] == "DYN-D" else [])
    return sorted(docs, key=lambda doc: doc["rank"])


# The dynamics lane's comparison conventions, carried into every dynamics record (and the diagonal case's own).
D_CONVENTIONS = [
    "relative deviation = (value - continuum) / |continuum|, the magnitude of the continuum value in the denominator",
    ("where a mode's two largest translational components are equal in magnitude, the shape's sign and the "
     "participation factor's sign are not compared; frequency, effective mass and combined magnitudes are"),
]
D_DIAGONAL_CONVENTIONS = [
    "element end forces in member axes are outside the like-for-like set for this case (member_frame)",
    ("under the SRSS mode sum this case's Y and Z components are equal, and under CQC they are not "
     "(discrete_reference_values)"),
]


# -- the declared addition of documented options records ------------------------------------------------------------
# Whole records a declared run may ADD to the writer's file, each documented on the vendor's public import page
# (IMPORT_PAGE): the dynamics options record OPMISC ("Options > Analysis > Dynamics & Misc"), placed with the other
# options records, before MATERIAL; and the spectrum table SPCMS, placed after LOADS and before LAYOUT (the page's
# section order: LOADS, BMATERIALS, BSECTIONS, BLOADS, SPCMS, SPLVL, LAYOUT; the writer writes none of the others).
ADDABLE_RECORDS = {"OPMISC": "MATERIAL", "SPCMS": "LAYOUT"}
OPMISC_FIELDS = ["cutoff frequency (Hz)", "number of modes", "include missing mass: 0 off, 1 on",
                 "include friction in dynamics: 0 off, 1 on", "include hanger stiffness: 0 off, 1 on",
                 "spectrum mode sum: 1 absolute, 2 SRSS, 3 closely spaced, 4 NRL",
                 "spectrum direction sum: 1 absolute, 2 SRSS", "spectrum group sum: 1 absolute, 2 SRSS",
                 "gravitational acceleration (m/sec2 in SI)"]
OPMISC_OMITTED = ("the tenth field, gravitational direction (valid from version 15.10), is not written: the public page "
                  "names it but does not state its values; the person checks the Direction of Gravity in the window "
                  "before the analysis")
SPCMS_FIELDS = ("first line: name (up to 32 characters), number of entries, abscissa interpolation, ordinate "
                "interpolation, abscissa units, ordinate units; then one line per entry: name, abscissa, ordinate")
SPCMS_CODES = ("the page states the SPCMS fields but not their codes; the codes used, abscissa and ordinate "
               "interpolation 0 = linear, abscissa units 0 = frequency (Hz), ordinate units 3 = acceleration "
               "(mm/sec2), are those the same page states for the fields of the same names in the SPLVL record, and "
               "the spectrums page states for a spectrum text file's units; that they hold for SPCMS is an inference, "
               "so each run's window entries ask the person to check the table as the window shows it")
MODE_SUMS = {1: ("abs", "absolute_sum"), 2: ("srss", "srss"), 3: ("closely-spaced", "ten_percent"), 4: ("nrl", None)}
DIRECTION_SUMS = {1: ("abs", "absolute_sum"), 2: ("srss", "srss")}
CLOSELY_SPACED = ("the technical reference states that the closely spaced method is the ten percent method of NRC "
                  "Guide 1.92 (DYNAMICS_PAGE); SWBPIPE's ten_percent rule corresponds only where it is the same rule")
REQ005_DEPENDENCE = ("relies on REQ-005 as placed in execution/PKG-17/DEL-17-05/ScopeOfWork.md, which admits a declared "
                     "addition of whole documented options records; these runs remain capture-only pending the "
                     "run-record and comparison work that scope's Not met block names")
# The highest-ranked cases whose every mode sum and direction sum the format page documents is captured (the project's
# Agent 0's ruling on ranks 1 to 3), and the diagonal case, whose wrapper states that every mode sum is meaningful on
# it; it has one direction, so only its mode sum varies.
ALL_SUMS_RANKS = (1, 2, 3)
ALL_MODE_SUMS_RANKS = (5,)


def _num(value: float) -> str:
    value = float(value)
    return str(int(value)) if value.is_integer() and abs(value) < 1e15 else repr(value)


def opmisc_line(settings: dict[str, Any], mode_sum: int, direction_sum: int) -> str:
    return ",".join([_num(settings["cutoff_frequency_hz"]), str(settings["number_of_modes"]),
                     str(settings["include_missing_mass"]), "1", "1", str(mode_sum), str(direction_sum), "1",
                     _num(settings["gravitational_acceleration_m_per_s2"])])


def spcms_lines(table: dict[str, Any]) -> list[str]:
    name = SPECTRUM_NAMES[table["id"]]
    lines = [f"{name},{len(table['points'])},0,0,0,3"]
    lines += [f"{name},{_num(f)},{_num(a * 1000.0)}" for f, a in table["points"]]
    return lines


def added_mbf_text(writer_text: str, records: list[dict[str, Any]]) -> str:
    """The run's file: the writer's text with whole records added, each {"heading", "lines"} with `lines` the
    heading and its data lines. Refused unless every heading is in ADDABLE_RECORDS, occurs nowhere in the writer's text
    and is added once; every line is non-empty with no carriage return or line feed; and the record's place, the line
    before its following heading, occurs exactly once. The result is not writer output, is never the bound file, and
    removing the added lines gives the writer's text back byte for byte."""
    text = writer_text
    seen = set()
    for record in records:
        heading, lines = record["heading"], list(record["lines"])
        if heading not in ADDABLE_RECORDS:
            raise ValueError(f"{heading!r} is not a record a declared run may add")
        if heading in seen or f"\r\n{heading}\r\n" in writer_text:
            raise ValueError(f"the {heading} record is already in the file")
        seen.add(heading)
        if not lines or lines[0] != heading or len(lines) < 2:
            raise ValueError(f"the {heading} record must be its heading followed by its data lines")
        if any(not isinstance(line, str) or not line or "\r" in line or "\n" in line for line in lines):
            raise ValueError("an added line must be non-empty, with no carriage return or line feed")
        marker = f"\r\n{ADDABLE_RECORDS[heading]}\r\n"
        if text.count(marker) != 1:
            raise ValueError(f"the file must carry the {ADDABLE_RECORDS[heading]} heading exactly once")
        text = text.replace(marker, "\r\n" + "\r\n".join(lines) + marker)
    return text


TYPED = "-table-typed"
FORM_A, FORM_B = "table_added", "table_typed"
from tools.caepipe_correlation.benchmark_hand_entry import (  # noqa: E402
    ANALYSIS_PAGE, PRINT_PAGE, ANALYZE_PAGE, SAVE_PAGE, FILE_MENU_PAGE, finish_steps, numbered, EXPORT_PAGE,
    export_declaration, skip, then_next,
)
SPECTRUM_LOAD_PAGE = ("https://docs.sstcae.com/users_manual/spectrum_load.htm (user's manual, \"Spectrum Load\"), read "
                      "2026-11-13")
# The Spectrum Load page's words for the dialog's fields, quoted exactly.
SPECTRUM_LOAD_QUOTES = {
    "spectrums": ("Select a spectrum from the drop-down combo box, which should have been input in the spectrum table "
                  "for each global direction."),
    "factor": "The multiplying (scale) factor for the spectrum is input here.",
    "mode_sum": "Pick one of three choices, SRSS (square root of sum of squares), Closely spaced or Absolute.",
    "direction_sum": "Pick one of two choices, SRSS (square root of the sum of squares) or Absolute.",
    "row": "Double click on the row to assign the spectrum load.",
    "level_tag": "Type an alpha-numeric name (up to 8 characters long) in this field.",
}
# The window's names for the sums (the Spectrum Load page); the format page's NRL mode sum is not among them.
WINDOW_MODE_SUM = {1: "Absolute", 2: "SRSS", 3: "Closely spaced", 4: None}
WINDOW_DIRECTION_SUM = {1: "Absolute", 2: "SRSS"}
NRL_RULE = ("Series dynamics-nrl-abs and dynamics-nrl-srss, and their -table-typed twins, declare Mode Sum code 4, NRL, "
            "which the import page documents for the OPMISC record; the Spectrum Load page lists only three choices "
            "for the window's Mode Sum (SRSS, Closely spaced, Absolute). Those runs keep the file's documented code; "
            "their instruction reads: Continue only if the displayed Mode Sum is NRL; otherwise record what it shows "
            "and skip.")
SINGLE_FORM = ("cpb-dyn-c-n08-two is declared in the typed form only (series dynamics-comparable-table-typed, run "
               "cpb-dyn-c-n08-two--srss-srss-table-typed). Its spectrum table (DYNTWO, three rows) is carried by no "
               "dry-run sample, so the condition on which the form that writes the table may be run could never "
               "be met; by design the typed form is this case's only form. It is not a run that was skipped")
SINGLE_FORM_MODELS = ("cpb-dyn-c-n08-two",)
CUTOFF_UNIT = ("the import page gives OPMISC's cutoff frequency without a unit; the analysis page states the window's "
               "Cutoff Frequency in Hertz (tab \"Dynamics\"), so the record's value is read as Hz, an inference each "
               "run's window check confirms before the analysis; every other OPMISC field's values are stated on the "
               "import page")
PAIR_RULE = ("each dynamics run but one case's is declared in two forms with different files: form A (series without "
             "suffix) adds the options record and the spectrum table; form B (series ending -table-typed) adds the "
             "options record alone and the person types the spectrum table. At most one of a pair is analysed. Form "
             "A is taken if the dry run's check of the dry-run sample that carries the same spectrum table agreed, form "
             "B otherwise. Only form A's named initial checks, of the spectrum table and of the spectrum options, which "
             "come before its explicit Save and Analyze steps, lead to form B: if either does not agree, form A is not "
             "run, any files its import wrote in its folder are left as they are, its run line says \"switched to "
             "counterpart <form B run id>\", and form B is run from its step 1, with no return to form A. Every other "
             "skip, in either form, runs nothing in its place, not the counterpart either, and goes on to the next "
             "strike number. Every pair ends with an explicit outcome on its two run lines: form A analysed, form B "
             "analysed, or neither run and why. The pair is never both screened")
RUN_SHEET_PAIR_RULE = ("At most one of a pair is analysed: form A if the dry run's check agreed, form B otherwise or when "
                       "form A was switched to it at its named initial checks; every pair ends with an explicit outcome on "
                       "its two run lines, including neither run and why.")
SESSION_ALTERNATIVE = ("offered as an instruction only: every run that is a further combination of a model whose "
                       "comparable run comes earlier in the strike order ends with a paragraph letting the owner, only "
                       "if the command noted at the dry run lets the owner choose the saved file's name and folder, and "
                       "with that comparable run's model still open, change Mode Sum and Direction Sum in Loads > "
                       "Spectrum (the Spectrum Load page names both) instead of importing the run's file; per-file import "
                       "remains the default and is what the pack is checked against. For an NRL run the paragraph "
                       "says the alternative is not open, because the public Spectrum Load page lists no NRL among "
                       "its Mode Sum choices")
DRY_RUN_A = ("Use this run only if the dry run's check of the dry-run sample that carries the same spectrum table agreed "
             "(the owner compared the table the program showed with the stated values and noted agreement on that "
             "sample's run line); otherwise run its counterpart {other} instead.")
DRY_RUN_B = ("Use this run if the dry run's check of the dry-run sample that carries the same spectrum table did not "
             "agree or was not made, or if its counterpart {other} was switched to this run at its step {steps}; "
             "otherwise run its counterpart {other} instead. Once switched to this run, do not return to {other}.")
# Form A's named initial checks, the only route from form A to form B: the spectrum table (step 4) and the spectrum
# options (step 5), both before the explicit Save and Analyze steps.
A_SWITCH_STEPS = (4, 5)
A_SWITCH_STEPS_TEXT = "4 or 5"
# The Analysis page's Misc section opens with these words, quoted exactly (users_manual/analysis.htm); Vertical Direction
# and Gravitational Acceleration and Direction come under it.
MISC_QUOTE = "On this tab you can set miscellaneous options."
DRY_RUN_SINGLE = ("This case is declared in the typed form only: no dry-run sample carries its spectrum table, "
                  "so no form that writes the table is declared.")


# The runs that end with the optional export of their saved model (benchmark_hand_entry.EXPORT_STATEMENT): every
# dynamics run at strike numbers 1 to 10, both forms of a pair and the single typed form; and every hand-entry run.
EXPORT_STRIKES = 10
EXPORT_LOST_DYNAMICS = ("the program's own writing of the spectrum assignment and the analyses chosen, for which the "
                        "vendor's public format page documents no record, and of the options it carries")


def strike_order(rank: int, combo_index: int) -> int:
    """The order in which the dynamics runs are taken and struck (1 first): the ten comparable runs by case rank;
    then the further combinations of ranks 1 to 3 in turn; then the diagonal case's three further mode sums last."""
    if combo_index == 0:
        return rank
    if rank in ALL_SUMS_RANKS:
        return 10 + 7 * (rank - 1) + combo_index
    return 10 + 7 * len(ALL_SUMS_RANKS) + combo_index


def _series_name(ms: int, ds: int) -> str:
    return "dynamics-comparable" if (ms, ds) == (2, 2) else f"dynamics-{MODE_SUMS[ms][0]}-{DIRECTION_SUMS[ds][0]}"


def _spectrum_load_steps(name: str, directions: list[dict[str, Any]], ms: int, ds: int,
                         kind: str) -> list[tuple[str, tuple]]:
    """The Loads > Spectrum entry, in the Spectrum Load page's words, as numbered steps."""
    q = SPECTRUM_LOAD_QUOTES
    listed = {item["direction"]: item for item in directions}
    unassigned = sorted({"X", "Y", "Z"} - set(listed))
    per_direction = "; ".join(f"{d}: {name}, Factor {_num(listed[d]['scale_factor'])}" for d in sorted(listed))
    spectrums = f"X, Y and Z spectrums (\"{q['spectrums']}\"): {per_direction}"
    if unassigned:
        spectrums += (f"; and no spectrum for {', '.join(unassigned)} (if the window requires a spectrum for every "
                      f"direction, {skip(kind)})")
    mode = WINDOW_MODE_SUM[ms]
    mode_sum = (f"Mode Sum (\"{q['mode_sum']}\"): this run declares NRL, which the page does not list. Continue only if "
                f"the displayed Mode Sum is NRL; otherwise record what it shows and skip. {then_next(kind)}"
                if mode is None else f"Mode Sum (\"{q['mode_sum']}\"): {mode}.")
    page = (SPECTRUM_LOAD_PAGE,)
    return [
        (f"Under the menu Loads > Spectrum, type a Level Tag L1 (\"{q['level_tag']}\"). The MBF has no documented "
         f"record for the spectrum assignment, so it is entered here; the file's OPMISC record also states the two "
         f"sums.", (LOAD_CASES_PAGE, SPECTRUM_LOAD_PAGE)),
        (f"Double-click the row (\"{q['row']}\").", page),
        (spectrums + ".", page),
        (f"Factor (\"{q['factor']}\") is a pure multiplier of the spectrum's ordinates, with no unit. If the window "
         f"offers one Factor per direction, enter each as the previous step states it. If the window offers a single "
         f"Factor and this run states the same factor for every direction it loads, enter that value; if it states "
         f"different factors for different directions, {skip(kind)}.", page),
        (mode_sum, page),
        (f"Direction Sum (\"{q['direction_sum']}\"): {WINDOW_DIRECTION_SUM[ds]}.", page),
    ]


def dynamics_runs(document: dict[str, Any], writer_text: str) -> list[dict[str, Any]]:
    """The declared runs of one dynamics model, derived from the document alone: the comparable setting its wrapper
    states, and, on the ranks above, every other mode sum (and direction sum where it has more than one direction) the
    format page documents; each in form A and form B (PAIR_RULE), one run per model per series; the case in
    SINGLE_FORM_MODELS in form B only."""
    request = document["dynamic_request"]
    settings = request["other_program_settings"]
    directions = request["swbpipe_spectrum_request"]["directions"]
    [table] = request["spectra"]
    rank, model_id = document["rank"], document["model_id"]
    single = model_id in SINGLE_FORM_MODELS
    if rank in ALL_SUMS_RANKS:
        combos = [(ms, ds) for ms in MODE_SUMS for ds in DIRECTION_SUMS]
    elif rank in ALL_MODE_SUMS_RANKS:
        combos = [(ms, 2) for ms in MODE_SUMS]
    else:
        combos = [(2, 2)]
    combos.sort(key=lambda combo: combo != (2, 2))  # the comparable run first
    writer_sha = hashlib.sha256(writer_text.encode("utf-8")).hexdigest()
    name = SPECTRUM_NAMES[table["id"]]
    pairs = "; ".join(f"{_num(f)} Hz, {_num(a * 1000.0)} mm/sec2" for f, a in table["points"])
    typed_rows = "; ".join(f"{_num(f)}, {_num(a * 1000.0)}" for f, a in table["points"])
    missing = "checked" if settings["include_missing_mass"] else "unchecked"
    g = _num(settings["gravitational_acceleration_m_per_s2"])
    runs = []
    for index, (ms, ds) in enumerate(combos):
        label = f"{MODE_SUMS[ms][0]}-{DIRECTION_SUMS[ds][0]}"
        series = _series_name(ms, ds)
        opmisc = {"heading": "OPMISC", "lines": ["OPMISC", opmisc_line(settings, ms, ds)]}
        spcms = {"heading": "SPCMS", "lines": ["SPCMS", *spcms_lines(table)]}
        swb_modal = MODE_SUMS[ms][1]
        ids = {FORM_A: f"{model_id}--{label}", FORM_B: f"{model_id}--{label}{TYPED}"}
        comparable_ids = {FORM_A: f"{model_id}--srss-srss", FORM_B: f"{model_id}--srss-srss{TYPED}"}
        for form in ((FORM_B,) if single else (FORM_A, FORM_B)):
            run_id = ids[form]
            other = None if single else ids[FORM_B if form == FORM_A else FORM_A]
            records = [opmisc, spcms] if form == FORM_A else [opmisc]
            text = added_mbf_text(writer_text, records)
            exports = strike_order(rank, index) <= EXPORT_STRIKES
            kind = "dynamics" if single else "pair"
            importing = f"Import {run_id}.mbf: File > Import from the Main window, select the file and click Import"
            if form == FORM_A:
                switch = (f"switch to the counterpart: this run is not run, and any files the import wrote in this run's "
                          f"folder are left as they are; write \"switched to counterpart {other}\" on this run line; then "
                          f"run {other} from its own folder, from its step 1.")
                table_steps = [
                    (DRY_RUN_A.format(other=other), ()),
                    (f"{importing}; it is the model file with the OPMISC and SPCMS records added.", (IMPORT_PAGE,)),
                    ("Select Spectrums from the Misc menu.", (SPECTRUMS_PAGE,)),
                    (f"If the List window for spectrums shows a spectrum named {name} with these "
                     f"{len(table['points'])} pairs of values, {pairs}, go on; otherwise {switch}", (SPECTRUMS_PAGE,)),
                    ("If Options > Spectrum, which sets the units of the X- and Y-axes and the interpolation method, "
                     "shows the X-axis unit Frequency (Hz), the Y-axis unit Acceleration (mm/sec2) and a linear "
                     f"interpolation method, go on; otherwise {switch}", (SPECTRUMS_PAGE,)),
                ]
                assert len(table_steps) == A_SWITCH_STEPS[-1]
                pages = [IMPORT_PAGE, SPECTRUMS_PAGE]
            else:
                table_steps = [
                    (DRY_RUN_SINGLE if single else DRY_RUN_B.format(other=other, steps=A_SWITCH_STEPS_TEXT), ()),
                    (f"{importing}; it is the model file with the OPMISC record added and no spectrum table.",
                     (IMPORT_PAGE,)),
                    ("Options > Spectrum sets the units of the X- and Y-axes and the interpolation method: if it offers "
                     "Frequency (Hz) for the X-axis and Acceleration (mm/sec2) for the Y-axis (the units the same page "
                     "names for a spectrum) and a linear interpolation method (spectra are interpolated linearly or "
                     f"logarithmically), set them; otherwise {skip(kind)}.", (SPECTRUMS_PAGE, DYNAMICS_PAGE)),
                    ("Select Spectrums from the Misc menu.", (SPECTRUMS_PAGE,)),
                    (f"In the List window for spectrums, type a spectrum named {name} in the Name column with "
                     f"{len(table['points'])} pairs of values, each a frequency in Hz then an acceleration in mm/sec2: "
                     f"{typed_rows}.", (SPECTRUMS_PAGE,)),
                ]
                pages = [IMPORT_PAGE, SPECTRUMS_PAGE, DYNAMICS_PAGE]
            other_state = "unchecked" if missing == "checked" else "checked"
            other_action = "check" if missing == "checked" else "uncheck"
            window = table_steps + _spectrum_load_steps(name, directions, ms, ds, kind) + [
                ("Before the analysis, open Options > Analysis, tab Dynamics.", (ANALYSIS_PAGE,)),
                (f"Check that Cutoff Frequency reads {_num(settings['cutoff_frequency_hz'])} Hz; a value that differs "
                 f"is set to the stated value by hand and noted on the run line. The vendor's public import page states "
                 f"the file's cutoff frequency value with no unit; Hertz is taken from the analysis page's description "
                 f"of the window's Cutoff Frequency field. If the tab does not show it, {skip(kind)}.",
                 (IMPORT_PAGE, ANALYSIS_PAGE)),
                (f"Check that Number of Modes reads {settings['number_of_modes']}; a value that differs is set to the "
                 f"stated value by hand and noted on the run line. If the tab does not show it, {skip(kind)}.",
                 (ANALYSIS_PAGE,)),
                (f"Check that Include Missing Mass Correction is {missing}. If it is {other_state}, {other_action} it "
                 f"and note that on the run line. If the tab does not show it, {skip(kind)}.", (ANALYSIS_PAGE,)),
                (f"Select the Misc tab (\"{MISC_QUOTE}\"; the page gives Vertical Direction and Gravitational "
                 f"Acceleration and Direction under it).", (ANALYSIS_PAGE,)),
                (f"Before the analysis, check the gravity the window shows against this run's request (\"Gravitational "
                 f"Acceleration and Direction\"): Vertical Direction Y; Gravitational Acceleration {g} m/sec2; "
                 f"Direction of Gravity downward along Y (-Y). The page says to \"click the “Change” button to modify "
                 f"the Gravitational Acceleration and the corresponding Direction of Gravity\"; change nothing. If the "
                 f"window shows another value or direction, or does not show them, {skip(kind)}.", (ANALYSIS_PAGE,)),
                ("Loads > Load Cases: check the box next to Response Spectrum for analysis.", (LOAD_CASES_PAGE,)),
                *finish_steps(run_id, kind, export_lost=EXPORT_LOST_DYNAMICS if exports else None),
            ]
            closing, closing_steps = None, None
            if index > 0:
                if WINDOW_MODE_SUM[ms] is None:
                    closing = ("A further run of the same model in one session is not open for this run: the public "
                               "Spectrum Load page lists no NRL among its Mode Sum choices; import this run's file.",
                               (SPECTRUM_LOAD_PAGE,))
                else:
                    closing = (
                        f"Only if the command you noted at the dry run lets you choose the saved file's name and folder: "
                        f"instead of importing this run's file, you MAY, with the model of this case's comparable run "
                        f"({comparable_ids[form]}) still open, take the steps below. Importing this run's file remains "
                        f"the default.", ())
                    closing_steps = [
                        (f"Change Mode Sum to {WINDOW_MODE_SUM[ms]} in Loads > Spectrum.", (SPECTRUM_LOAD_PAGE,)),
                        (f"Change Direction Sum to {WINDOW_DIRECTION_SUM[ds]} in Loads > Spectrum.", (SPECTRUM_LOAD_PAGE,)),
                        (f"Save the model as {run_id}.mod in this run's folder by the command you noted at the dry run.",
                         ()),
                        ("Analyze it by File > Analyze (F12).", (ANALYZE_PAGE,)),
                        (f"Print the results to file as {run_id}.print.csv.", (PRINT_PAGE,)),
                        ("Write on this run's run line that the setting was made by hand.", ()),
                    ]
            instruction = numbered(window, closing, closing_steps)
            runs.append({
                "series": series + (TYPED if form == FORM_B else ""), "kind": "option_addition", "form": form,
                "label": label + (TYPED if form == FORM_B else ""), "run_id": run_id, "counterpart_run_id": other,
                "single_form": single,
                "comparable": (ms, ds) == (2, 2), "strike_order": strike_order(rank, index),
                "mbf": {"source": "addition", "writer_output": False, "bound_file": False, "writer_sha256": writer_sha,
                        "added_records": records, "sha256": hashlib.sha256(text.encode("utf-8")).hexdigest()},
                "spectrum_mode_sum": ms, "spectrum_direction_sum": ds,
                "window_mode_sum": WINDOW_MODE_SUM[ms],
                "swbpipe_counterpart": None if swb_modal is None else {
                    "modal_combination": swb_modal, "directional_combination": DIRECTION_SUMS[ds][1]},
                "window_instruction": {"text": instruction,
                                       "sha256": hashlib.sha256(instruction.encode("utf-8")).hexdigest()},
                "pages": pages + [SPECTRUM_LOAD_PAGE, LOAD_CASES_PAGE, ANALYSIS_PAGE, SAVE_PAGE, FILE_MENU_PAGE,
                                  ANALYZE_PAGE, PRINT_PAGE] + ([EXPORT_PAGE] if exports else []),
                **({"export": export_declaration(run_id, EXPORT_LOST_DYNAMICS)} if exports else {}),
                "expected_saved_file": f"{run_id}.mod", "expected_print_file": f"{run_id}.print.csv",
                "screened": False,
                "screening_waits_for": ("a headless modal and response-spectrum entry of the product, which main does "
                                        "not have; until then the capture is held"),
                "depends_on": REQ005_DEPENDENCE,
            })
    return runs


def run_document(document: dict[str, Any], series: str) -> dict[str, Any]:
    """The document of an option-addition or hand-entry run (benchmark_models.series_document delegates here): the
    tracked document's model, as the writer is given it (for a hand-entry case, the written base; the model WITH the
    member is formed only by benchmark_hand_entry.hand_entry_model, which nothing on the loading path calls), marked
    capture_only, with no closed form and no inexact input presented for screening (the tracked ones are kept under
    third_values and declared_inexact_inputs), and what the pack carries: the records to add, the window instruction
    with its SHA-256, and the pages cited. Deterministic; nothing is written."""
    if "hand_entry" in document:
        candidates = [document["hand_entry"]["run"]]
        third = document["hand_entry"]["third_values"]
    elif "dynamic_request" in document:
        candidates = dynamics_runs(document, bm.writer_mbf_text(document))
        third = document["closed_form"]
    else:
        candidates, third = [], []
    matches = [run for run in candidates if run["series"] == series]
    if len(matches) != 1:
        raise ValueError(f"{document.get('model_id')} has no {series!r} run")
    [run] = matches
    out = copy.deepcopy(document)
    out["series_run"] = series
    out["run_id"] = run["run_id"]
    out["run_kind"] = run["kind"]
    out["capture_only"] = True
    out["third_values"] = copy.deepcopy(third)
    out["declared_inexact_inputs"] = copy.deepcopy(document["inexact_inputs"])
    out["closed_form"] = []
    out["inexact_inputs"] = {}
    out["window_instruction"] = copy.deepcopy(run["window_instruction"])
    out["pages"] = list(run["pages"])
    if run["kind"] == "option_addition":
        out["added_records"] = copy.deepcopy(run["mbf"]["added_records"])
    return out


# -- screening classes ----------------------------------------------------------------------------------------------
SCREENING_CLASS_RULE = {
    "rule": ("per static load case: nonlinear where the case's model holds a support whose status can change, else "
             "exact where the case names no inexact input, else named_inexact (core/handoff/caepipe_external/"
             "benchmark.py screening_class); per analysis: a modal analysis is modal; a response-spectrum analysis is "
             "response_spectrum. For a model with declared runs the static rule applies to each screened run's own "
             "inexact inputs"),
    "classes": {"exact": "1e-4", "named_inexact": "5e-3", "nonlinear": "2e-2, and support status must match exactly",
                "modal": "5e-3 on frequencies, MAC >= 0.99",
                "response_spectrum": ("declared by name; its tolerance is set from its closed-form references before "
                                      "any comparison run and stands only once docs/RELEASE_TARGET_v1.0.md states it")},
    "source": "the Screening profile term of execution/PKG-17/DEL-17-05/ScopeOfWork.md and the Benchmark method "
              "section of docs/RELEASE_TARGET_v1.0.md",
}


def static_class(inexact: list[str], *, nonlinear: bool = False) -> str:
    if nonlinear:
        return "nonlinear"
    return "named_inexact" if inexact else "exact"


def static_classes(inexact_inputs: dict[str, list[str]], cases: list[str]) -> dict[str, str]:
    return {case: static_class(inexact_inputs.get(case, [])) for case in cases}


def derived_screening_classes(document: dict[str, Any]) -> dict[str, Any]:
    """The rule applied to a tracked document of tranche 1 or 2, from its own declarations only."""
    cases = sorted({case["id"] for case in document["model"]["load_cases"]} | {OPERATING})
    if document.get("tranche") == "2":
        return {"static_by_run": {run: static_classes(inexact, cases)
                                  for run, inexact in sorted(document["inexact_inputs_by_series"].items())}}
    return {"static": static_classes(document["inexact_inputs"], cases)}


# -- assembly -------------------------------------------------------------------------------------------------------
OPTION_ADDITION = {
    "records": {heading: f"added before the {before} heading" for heading, before in ADDABLE_RECORDS.items()},
    "source": IMPORT_PAGE,
    "opmisc_fields": OPMISC_FIELDS, "opmisc_omitted": OPMISC_OMITTED,
    "spcms_fields": SPCMS_FIELDS, "spcms_codes": SPCMS_CODES, "closely_spaced": CLOSELY_SPACED,
    "conditions": [
        "(a) every added record is one the vendor's public import page documents (OPMISC, SPCMS); any other is refused",
        "(b) whole records only, recorded line by line with the resulting file's hash; the resulting file is not the "
        "bound file and is never imported as one",
        "(c) no node, element, support or load record changes: removing the added lines gives the writer's file byte "
        "for byte",
        "(d) what no documented record carries is done in the program's window by the person, as each run's "
        "window_instruction says: the assignment of a spectrum to directions with its factors and the two sums are "
        "entered (Spectrum Load page); the gravitational acceleration and direction are checked, not entered; the "
        "response-spectrum analysis is selected (Load cases page)",
    ],
    "pairs": PAIR_RULE,
    "run_sheet_pair_rule": RUN_SHEET_PAIR_RULE,
    "opmisc_cutoff_unit": CUTOFF_UNIT,
    "same_session_alternative": SESSION_ALTERNATIVE,
    "nrl": NRL_RULE,
    "single_form": SINGLE_FORM,
    "strike_order": ("every dynamics run carries strike_order, one integer across the part, 1 first; both forms of a "
                     "pair share it: the ten comparable runs by case rank, then the further combinations of ranks 1 to "
                     "3, then the diagonal case's three further mode sums"),
    "files": ("each run is its own case under its run id: the person imports <run_id>.mbf, saves it as <run_id>.mod "
              "before analysing by the command noted at the dry run, and prints to file as <run_id>.print.csv"),
    "depends_on": REQ005_DEPENDENCE,
}


def add_tranche3(seed: int, files: dict[str, str], entries: list[dict], series: dict[str, dict]) -> None:
    """Append tranche 3's model files, manifest entries and series to the generator's output."""
    common = {"generator": {"version": bm.GENERATOR_VERSION, "seed": seed}, "piping_code": bm.PIPING_CODE,
              "reference_temperature_c": bm.REFERENCE_TEMPERATURE_C, "tranche": TRANCHE}
    for family, (name, swept, values, build, hypothesis, description) in S_SERIES.items():
        series[name] = {"swept": swept, "hypothesis": hypothesis, "description": SERIES_TEXT[name], "members": []}
        if family == "S3":
            series[name]["rigid_support"] = RIGID_SUPPORT
        for index, value in enumerate(values, 1):
            model_id = f"cpb-{family.lower()}-{index:02d}"
            built = build(model_id, value)
            cases = sorted({case["id"] for case in built["model"]["load_cases"]} | {OPERATING})
            inexact = {case: built["inexact"].get(case, []) for case in cases if case != OPERATING}
            inexact[OPERATING] = sorted({item for items in inexact.values() for item in items})
            document = {
                "document_kind": bm.MODEL_KIND, "model_id": model_id, "family": family,
                "family_description": description, **common, "sampling": "series",
                "parameters": built["parameters"], "series": [name], "inexact_inputs": inexact,
                "screening_classes": {"static": static_classes(inexact, cases)},
                "closed_form": built["closed_form"], "predictions": built["predictions"],
                "provenance": {"origin": "invented", "license": "project-original",
                               "redistribution_status": "public_permissive", "produced_by_caepipe": False,
                               "contributor_note": PROVENANCE},
                "model": built["model"],
            }
            series[name]["members"].append({"model_id": model_id, "value": value})
            _emit(document, files, entries, runs=None)
    series["members"] = {"swept": "members", "hypothesis": "mass_formulation",
                         "description": ("DYN-C: one straight pipe cut into 1, 2, 4, 8 and 16 members; cpb-dyn-c-n08-"
                                         "trunc and cpb-dyn-c-n08-two also differ in their request (missing mass, "
                                         "table), as their wrappers state"), "members": []}
    for document in dynamics_documents():
        document.update(common)
        document["folded_from"] = D_FOLDED
        cases = ["case:weight", OPERATING]
        document["screening_classes"] = {"static": static_classes(document["inexact_inputs"], cases),
                                         "modal": "modal", "response_spectrum": "response_spectrum"}
        if "members" in document["series"]:
            series["members"]["members"].append({"model_id": document["model_id"],
                                                 "value": document["parameters"]["members"]})
        _emit(document, files, entries, runs=dynamics_runs)
    # Part two: cases entered by hand at the window (benchmark_hand_entry.py), after family D.
    from tools.caepipe_correlation import benchmark_hand_entry as hand

    for document in hand.documents(seed, common, static_classes):
        _emit(document, files, entries, runs=None)
        entries[-1]["hand_entry_rank"] = document["hand_entry_rank"]
        entries[-1]["runs"] = [{**document["hand_entry"]["run"],
                                "mbf": {"source": "writer output: the written base", "writer_output": True,
                                        "sha256": entries[-1]["mbf_sha256"]},
                                "swbpipe_model_hash": bm.canonical_model_hash(hand.hand_entry_model(document)),
                                "screened": False,
                                "screening_waits_for": (
                                    hand.SERIES_WAIT + ", as well as SWBPIPE's own result for the case published by "
                                    "the product as checks passed and the capture's binding to the model actually run; "
                                    "until then the capture is held and not compared, and the series's load values are "
                                    "never fitted to the captured result"
                                    if hand.CONSTANT_LOAD in document["inexact_inputs"][OPERATING] else
                                    "SWBPIPE's own result for the case published by the product as checks passed; until "
                                    "then the capture is held")}]
        entries[-1]["held"] = document["hand_entry"]["held"]
        series.setdefault("hand-entry", {"kind": "hand_entry", "description": HAND_SERIES_TEXT, "members": []})
        series["hand-entry"]["members"].append({"model_id": document["model_id"],
                                                "run_id": document["hand_entry"]["run"]["run_id"],
                                                "rank": document["hand_entry_rank"]})
    for entry in entries:
        if entry.get("tranche") != TRANCHE:
            continue
        for run in entry.get("runs", []):
            if run["kind"] != "option_addition":
                continue
            name = run["series"]
            if name not in series:
                ms, ds = MODE_SUMS[run["spectrum_mode_sum"]][0], DIRECTION_SUMS[run["spectrum_direction_sum"]][0]
                form = "spectrum table typed at the window (form B)" if run["form"] == FORM_B else (
                    "spectrum table added to the file (form A)")
                what = ("the comparable setting each wrapper states" if run["comparable"] else
                        "a further combination the format page documents")
                series[name] = {"kind": "option_addition", "form": run["form"], "spectrum_mode_sum": ms,
                                "spectrum_direction_sum": ds,
                                "description": f"mode sum {ms}, direction sum {ds}: {what}; {form}. {PAIR_RULE}",
                                "members": []}
            series[name]["members"].append({"model_id": entry["model_id"], "run_id": run["run_id"],
                                            "counterpart_run_id": run["counterpart_run_id"],
                                            "strike_order": run["strike_order"]})


def export_statement() -> dict[str, Any]:
    """The manifest's run-independent account of the optional exports."""
    from tools.caepipe_correlation import benchmark_hand_entry as hand

    return {"statement": hand.EXPORT_STATEMENT, "page": hand.EXPORT_PAGE,
            "quotes": [hand.EXPORT_QUOTE_MENU, hand.EXPORT_QUOTE_TYPE, hand.EXPORT_QUOTE_BUTTON],
            "runs": ("the five hand-entry runs and every dynamics run at strike numbers 1 to 10 (both forms of each pair "
                     "and the single typed form at strike 6); no other run"),
            "condition": "the dry run's check (v), on its sacrificial duplicate (dry_run.export_trial)"}


def dry_run_declaration() -> dict[str, Any]:
    from tools.caepipe_correlation import benchmark_hand_entry as hand

    return copy.deepcopy(hand.DRY_RUN)


def hand_entry_declaration() -> dict[str, Any]:
    from tools.caepipe_correlation import benchmark_hand_entry as hand

    return {"rule": ("the tracked document's model is the WRITTEN BASE; the case's SWBPIPE model is the base with the "
                     "member's support records appended (benchmark_hand_entry.hand_entry_model); no support or load "
                     "record is added to a written file"),
            "held": hand.HELD, "held_until_series": {"cpb-h-01": hand.HELD_UNTIL_SERIES},
            "one_way_cases": hand.ONE_WAY, "skip": hand.skip_rule("hand"),
            "files": ("each run is its own case under its run id: the person imports <run_id>.mbf, enters the one "
                      "member, saves it as <run_id>.mod before analysing by the command noted at the dry run, never "
                      "over the .mbf, and prints to file as <run_id>.print.csv; nothing of SWBPIPE's reads the saved "
                      "file"),
            "left_out": hand.LEFT_OUT}


HAND_SERIES_TEXT = (
    "part two: each case is one SWBPIPE model holding a member the writer refuses, and its written base, the same model "
    "without that member; the person imports the base, enters the one member by the run's instruction, saves under a "
    "new name beside the base and prints. Ranked most valuable first, so that a sitting cut short strikes from the "
    "bottom. Each capture is held until SWBPIPE's own result for that case is published as checks passed; cpb-h-01's "
    "also until an independently justified one-parameter series supplies the evidence REQ-019 requires for its input "
    "constant_support_load")


def _emit(document: dict[str, Any], files: dict[str, str], entries: list[dict], *, runs) -> None:
    text = bm._dump(document)
    relative = f"models/{document['model_id']}.json"
    files[relative] = text
    writer_text = bm.writer_mbf_text(document)
    entry = {"model_id": document["model_id"], "family": document["family"], "tranche": TRANCHE,
             "sampling": document["sampling"], "parameters": document["parameters"], "series": document["series"],
             "file": relative, "model_hash": bm.canonical_model_hash(document["model"]),
             "file_sha256": hashlib.sha256(text.encode("ascii")).hexdigest(),
             "layout_row_count": bm.layout_rows(document),
             "mbf_sha256": hashlib.sha256(writer_text.encode("utf-8")).hexdigest(),
             "screening_classes": document["screening_classes"]}
    if "rank" in document:
        entry["rank"] = document["rank"]
    if runs is not None:
        entry["runs"] = runs(document, writer_text)
    entries.append(entry)


# -- the rigid_support_stiffness override of tranches 1 and 2 (the project's Agent 0, 13 November 2026) --------------
RIGID_OVERRIDE_TEXT = (
    "The class was changed on 13 November 2026 after first-tranche screens of 28 October and 12 November 2026, on grounds "
    "independent of any result: the vendor's public anchor page states a finite default stiffness, and textbook "
    "mechanics predicts its effect (series S3). The reviewer who raised it had seen no result. Which class these cases "
    "are compared under is decided by the owner at their review of the benchmark record; until then each is reported "
    "under both, and no agreement count is stated for them under the named class alone.")
RIGID_OVERRIDE_RULE = (
    "a model names rigid_support_stiffness on every load case its thermal loads enter (the thermal case and the "
    "operating sum) when it has two or more anchors, each of which restrains the pipe's axial growth; a model with one "
    "anchor (a free thermal cantilever) is not relabelled. For tranches 1 and 2 the input is named by this manifest "
    "override, which leaves the 82 documents' bytes unchanged; tranche 3's documents name it themselves")
RIGID_OVERRIDE_CONSUMER = (
    "the override is consumed only by a later change with its own review: core/handoff/caepipe_external/benchmark.py "
    "derives a case's class from the run document's inexact_inputs and does not read this override on this branch")


def _anchor_count(model: dict[str, Any]) -> int:
    return sum(1 for support in model["supports"] if support.get("family") == "anchor")


def _thermal_cases(model: dict[str, Any]) -> list[str]:
    cases = [case["id"] for case in model["load_cases"]
             if any(load.get("category") == "thermal" for load in case["primitive_loads"])]
    return cases + ([OPERATING] if cases else [])


def rigid_override(document: dict[str, Any]) -> dict[str, Any] | None:
    """The rigid_support_stiffness override of an earlier-tranche document, by RIGID_OVERRIDE_RULE, with both classes
    for each case it names the input on; None where the rule names nothing."""
    model = document["model"]
    cases = _thermal_cases(model)
    if _anchor_count(model) < 2 or not cases:
        return None

    def per_case(inexact: dict[str, list[str]]) -> dict[str, Any]:
        out = {}
        for case in cases:
            declared = list(inexact.get(case, []))
            out[case] = {"inputs_as_declared": declared, "inputs_named": sorted({*declared, RIGID}),
                         "class_as_first_screened": static_class(declared),
                         "class_named": static_class([*declared, RIGID])}
        return out

    override = {"input": RIGID, "rule": RIGID_OVERRIDE_RULE, "consumed_by": RIGID_OVERRIDE_CONSUMER}
    if document.get("tranche") == "2":
        override["runs"] = {run: per_case(inexact)
                            for run, inexact in sorted(document["inexact_inputs_by_series"].items())}
    else:
        override["cases"] = per_case(document["inexact_inputs"])
    changed = [(run, case) for run, by_case in override.get("runs", {"": override.get("cases", {})}).items()
               for case, both in by_case.items() if both["class_as_first_screened"] != both["class_named"]]
    if changed:
        override["both_classes"] = RIGID_OVERRIDE_TEXT
    return override


def rigid_override_declaration(entries: list[dict[str, Any]]) -> dict[str, Any]:
    """The manifest's account of the override: rule, text, consumer, and every declaration whose class it changes."""
    changed = []
    for entry in entries:
        override = entry.get("inexact_input_override")
        if not override:
            continue
        for run, by_case in override.get("runs", {None: override.get("cases", {})}).items():
            for case, both in sorted(by_case.items()):
                if both["class_as_first_screened"] != both["class_named"]:
                    changed.append({"model_id": entry["model_id"], "run": run, "case": case,
                                    "class_as_first_screened": both["class_as_first_screened"],
                                    "class_named": both["class_named"]})
    return {"input": RIGID, "rule": RIGID_OVERRIDE_RULE, "text": RIGID_OVERRIDE_TEXT, "consumed_by": RIGID_OVERRIDE_CONSUMER,
            "class_changed": changed}
