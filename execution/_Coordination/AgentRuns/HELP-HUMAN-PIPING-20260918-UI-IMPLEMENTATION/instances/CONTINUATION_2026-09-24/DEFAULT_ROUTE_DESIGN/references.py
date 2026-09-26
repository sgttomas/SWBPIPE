"""Candidate frozen hand-statics references for tranche T0R (default route), revision 3.

Revision 3 applies S0-prime items 1-10 (REFERENCE_CHECK/REVISION_2/RETURN.md) to the
revision_2 block only; revision-1 keys and values stay byte-unchanged.

Revision 2 keeps every revision-1 key and value unchanged (S0 compared them) and adds
the references required by ROOT_RULINGS.md rulings 1, 3, 8, 10 and 11 and notes N-1 and
N-6. Every expectation that is zero carries an explicit "zero_scale" for the criterion
|observed - expected| <= 1e-9 * max(|expected|, scale).

Standard library only. Nothing here imports, calls or reads product code, product
fixtures or product tests. Every expectation follows from rigid-body statics,
Euler-Bernoulli cantilever compliance, spring compatibility and the elastic
circular-annulus section relations (see DESIGN.md, "Frozen references").

Run:  python3 references.py > _run_records/references.stdout.txt
The printed values, not this script, are the frozen expectations; the script is
the reproducible derivation. Inputs are invented test values, not engineering
library data, code rules or material standards.
"""
from __future__ import annotations

import json
import math

PI = math.pi


def annulus(outside_diameter: float, wall: float) -> dict:
    ro = outside_diameter / 2.0
    ri = ro - wall
    area = PI * (ro * ro - ri * ri)
    inertia = PI * (ro**4 - ri**4) / 4.0
    return {"ro": ro, "ri": ri, "A": area, "I": inertia, "J": 2.0 * inertia, "Z": inertia / ro}


def cross(a, b):
    return (a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])


def add(*vs):
    return tuple(sum(c) for c in zip(*vs))


def neg(v):
    return tuple(-c for c in v)


def scale(k, v):
    return tuple(k * c for c in v)


def norm(v):
    return math.sqrt(sum(c * c for c in v))


def dot(a, b):
    return sum(x * y for x, y in zip(a, b))


def anchor_action(point_loads, couples=(), line_loads=(), origin=(0.0, 0.0, 0.0)):
    """Support-on-pipe action (F, M about origin) of a single anchor at origin.

    point_loads: [(position, force)], couples: [couple], line_loads: [(start, end, w)]
    with uniform w (force per length) on the straight segment start->end.
    Equilibrium: F + sum(applied) = 0 and M + sum(r x applied) + sum(couples) = 0.
    """
    force = (0.0, 0.0, 0.0)
    moment = (0.0, 0.0, 0.0)
    for position, f in point_loads:
        r = tuple(p - o for p, o in zip(position, origin))
        force = add(force, f)
        moment = add(moment, cross(r, f))
    for c in couples:
        moment = add(moment, c)
    for start, end, w in line_loads:
        length = norm(tuple(e - s for s, e in zip(start, end)))
        resultant = scale(length, w)
        centroid = tuple((s + e) / 2.0 - o for s, e, o in zip(start, end, origin))
        force = add(force, resultant)
        moment = add(moment, cross(centroid, resultant))
    return neg(force), neg(moment)


def fmt(v):
    return [float(repr(c)) if c != 0 else 0.0 for c in v]


out: dict = {}
E = 200e9  # invented test modulus, Pa
C1 = annulus(0.12, 0.01)  # the T0 probe section
S1 = annulus(0.10, 0.01)  # STRESS_REFERENCE S1 section (ri=0.04, ro=0.05)
out["sections"] = {"C1_od0.12_wall0.01": C1, "S1_od0.10_wall0.01": S1}

# ---------------------------------------------------------------- M14
# REF-M14-A: probe case A. 1 m cantilever along +x, anchor at x=0, tip Fy=Fz=1000 N.
# Root |My|=|Mz|=1000 N*m, N=0, so max|sigma| = hypot(My,Mz)/Z at the root.
out["REF-M14-A"] = {
    "circular_normal_max_Pa": math.hypot(1000.0, 1000.0) / C1["Z"],
    "location": "pipe root (x=0), load case A",
    "negative_control_abs_sum_Pa": 2000.0 / C1["Z"],
}
# REF-M14-S1: S1 section, 1 m cantilever, tip Fx=1800*pi (tension), Fy=73.8*pi, Fz=55.35*pi.
na, by, bz = 1800 * PI / S1["A"], 55.35 * PI / S1["Z"], 73.8 * PI / S1["Z"]
out["REF-M14-S1"] = {
    "axial_Pa": na, "bending_a_Pa": by, "bending_b_Pa": bz,
    "circular_normal_max_Pa": abs(na) + math.hypot(by, bz),
    "negative_control_abs_sum_Pa": abs(na) + abs(by) + abs(bz),
    "rotated_variant": "same Fx, transverse tip force Fz=92.25*pi only (bending 5 MPa on one axis)",
    "rotated_circular_normal_max_Pa": abs(na) + 92.25 * PI / S1["Z"],
    "with_tip_torque_221.4pi_torsional_shear_Pa": 221.4 * PI * S1["ro"] / S1["J"],
    "pure_torque_only_normal_max_Pa": 0.0,
}
# REF-M14-X1: STRESS_REFERENCE X1 realized on the ordinary route. S1 section, L=1 m, pin at i
# (UX,UY,UZ,RX), roller at j (UY,UZ); uniform w_z = 8e6*Z/L^2 N/m and an end couple about z at
# j of 1e6*Z N*m. Then b(t)=My/Z = 4 t(1-t) MPa and c(t)=Mz/Z = t MPa, a=0.
t_star = (6.0 - math.sqrt(2.0)) / 8.0
f_star = math.sqrt(71.0 + 8.0 * math.sqrt(2.0)) / 8.0 * 1e6
f = lambda t: math.hypot(4 * t * (1 - t), t) * 1e6  # noqa: E731
dense = max(f(i / 200000.0) for i in range(200001))
out["REF-M14-X1"] = {
    "w_z_N_per_m": 8e6 * S1["Z"], "end_couple_Nm": 1e6 * S1["Z"],
    "t_star": t_star, "circular_normal_max_Pa": f_star,
    "dense_sampling_crosscheck_Pa": dense,
    "negative_control_old_eight_sign_candidates_Pa": max(f(0.0), f(1.0), f(3 / 8), f(5 / 8)),
    "negative_control_abs_sum_objective_max_Pa": (4 * (5 / 8) * (3 / 8) + 5 / 8) * 1e6,
}

# ---------------------------------------------------------------- M33
EI = E * C1["I"]
out["REF-M33-G"] = {
    "case_A": "tip Fy=1000, Fz=1000 (listed first)", "case_B": "tip Fy=3000 (listed second)",
    "case_A_max_Pa": math.hypot(1000.0, 1000.0) / C1["Z"],
    "case_B_max_Pa": 3000.0 / C1["Z"],
    "headline_Pa": 3000.0 / C1["Z"], "headline_case": "case_B", "headline_location": "pipe root",
    "negative_control_first_case_headline_Pa": math.hypot(1000.0, 1000.0) / C1["Z"],
    "tip_displacement_A_mm": math.hypot(1000.0, 1000.0) / (3 * EI) * 1e3,
    "tip_displacement_B_mm": 3000.0 / (3 * EI) * 1e3,
    "displacement_headline_mm": 3000.0 / (3 * EI) * 1e3,
    "permutation": "reverse case order; headline value, case and location unchanged",
}

# ---------------------------------------------------------------- M05
F, M = anchor_action([], couples=[(500.0, 0.0, 0.0)])
out["REF-M05-T"] = {"anchor_F_N": fmt(F), "anchor_M_Nm": fmt(M), "force_magnitude_N": norm(F),
                    "moment_magnitude_Nm": norm(M),
                    "negative_control": "a force-norm-only publication shows 0 N and no moment"}
tip = (2.0, 0.0, 0.0)
F1, M1 = anchor_action([(tip, (10.0, -20.0, 30.0))], couples=[(4.0, 5.0, -6.0)])
F2, M2 = anchor_action([(tip, (10.0, -20.0, 30.0))], couples=[(4.0, 5.0, -6.0)],
                       line_loads=[((0, 0, 0), tip, (0.0, -3.0, 0.0))])
out["REF-M05-R1"] = {"geometry": "2 m cantilever along +x", "anchor_F_N": fmt(F1), "anchor_M_Nm": fmt(M1),
                     "with_uniform_0_-3_0_N_per_m": {"anchor_F_N": fmt(F2), "anchor_M_Nm": fmt(M2)}}


def rot_z(deg):
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return lambda v: (c * v[0] - s * v[1], s * v[0] + c * v[1], v[2])


perm = lambda v: (v[2], v[0], v[1])  # noqa: E731  exact 120 deg rotation about (1,1,1)
for name, q in (("perm_xyz_to_yzx", perm), ("rot_z_30deg", rot_z(30.0))):
    Fq, Mq = anchor_action([(q(tip), q((10.0, -20.0, 30.0)))], couples=[q((4.0, 5.0, -6.0))],
                           line_loads=[((0, 0, 0), q(tip), q((0.0, -3.0, 0.0)))])
    out["REF-M05-R1"][f"rotated_{name}"] = {"anchor_F_N": fmt(Fq), "anchor_M_Nm": fmt(Mq),
                                            "equals_Q_times_unrotated": [fmt(q(F2)), fmt(q(M2))]}
# REF-M05-SPRING: C1 section, 1 m cantilever, tip spring k=1e6 N/m in global y, tip Fy=1000.
k_beam, k_s = 3 * EI / 1.0**3, 1.0e6
u = 1000.0 / (k_beam + k_s)
spring = -k_s * u
Fa, Ma = anchor_action([((1.0, 0.0, 0.0), (0.0, 1000.0 + spring, 0.0))])
out["REF-M05-SPRING"] = {"tip_u_m": u, "spring_on_pipe_Fy_N": spring, "anchor_F_N": fmt(Fa),
                         "anchor_M_Nm": fmt(Ma),
                         "check": "anchor Fy + spring Fy + 1000 = 0; anchor Mz balances the net tip force"}
# REF-M05-COMB: cases A1 (tip Fy=+1000, L=1) and T (tip Mx=+500); combinations.
FA, MA = anchor_action([((1.0, 0.0, 0.0), (0.0, 1000.0, 0.0))])
FT, MT = anchor_action([], couples=[(500.0, 0.0, 0.0)])
lin = (add(scale(2, FA), scale(-1, FT)), add(scale(2, MA), scale(-1, MT)))
sub = (add(FA, neg(FT)), add(MA, neg(MT)))
out["REF-M05-COMB"] = {
    "case_A1": {"F": fmt(FA), "M": fmt(MA)}, "case_T": {"F": fmt(FT), "M": fmt(MT)},
    "mechanics_2A1_minus_T": {"F": fmt(lin[0]), "M": fmt(lin[1]), "force_magnitude_N": norm(lin[0]),
                              "moment_magnitude_Nm": norm(lin[1])},
    "subtraction_A1_minus_T": {"F": fmt(sub[0]), "M": fmt(sub[1]), "force_magnitude_N": norm(sub[0]),
                               "moment_magnitude_Nm": norm(sub[1])},
    "negative_control_magnitude_algebra_moment_Nm": 2 * norm(MA) - norm(MT),
    "envelope_max_force_magnitude_N": max(norm(FA), norm(FT)),
}

# ---------------------------------------------------------------- M08 and bends
# REF-M08-L: T0 probe L model. Anchor a(0,0,0); a-b to (1,0,0); b-c to (1,1,0); geometry-only
# bend marker at b with user SIF 1.15 and k 1.08; load at c: case up Fz=+1000, case down Fz=-1000;
# combination up+down (factors 1, 1).
c_pt = (1.0, 1.0, 0.0)
Fu, Mu = anchor_action([(c_pt, (0.0, 0.0, 1000.0))])
m_about_b = cross((0.0, 1.0, 0.0), (0.0, 0.0, 1000.0))
out["REF-M08-L"] = {
    "anchor_up": {"F": fmt(Fu), "M": fmt(Mu)},
    "member_ab_root_torsion_Nm": 1000.0, "member_ab_root_bending_Nm": 1000.0,
    "member_ab_end_b": "pure torsion 1000 N*m, zero bending",
    "member_bc_at_b_moment_Nm": fmt(m_about_b), "member_bc_at_b_bending_Nm": 1000.0,
    "per_case_circular_normal_max_Pa": 1000.0 / C1["Z"],
    "headline_Pa": 1000.0 / C1["Z"], "headline_tie": ["pipe:a-b at a", "pipe:b-c at b"],
    "combination_up_plus_down": "every signed action, stress and support component is zero",
    "negative_control_old_multiplier_row_MPa": 2 * (1000.0 / C1["Z"]) * 1.15 * 1.08 / 1e6,
    "negative_control_multiplier_uses_k": "any published stress that changes with k while actions are fixed",
}
# REF-B1: statically determinate cantilever with a 90 deg arc. Anchor a(0,0,0); straight to
# b(1,0,0); arc radius 0.2 in the xy plane, centre (1,0.2,0), from b to c(1.2,0.2,0); straight
# to d(1.2,1.2,0); tip load Fz=+1000 N at d. Resultant magnitudes in the arc tangent frame
# (x tangent, z bend-plane normal, y in-plane radial) follow from the free body beyond each cut
# and are independent of any flexibility factor.
d_pt, Fz = (1.2, 1.2, 0.0), (0.0, 0.0, 1000.0)
normal = (0.0, 0.0, 1.0)
stations = {}
for label, theta in (("arc_i_at_b", -90.0), ("arc_mid", -45.0), ("arc_j_at_c", 0.0)):
    th = math.radians(theta)
    p = (1.0 + 0.2 * math.cos(th), 0.2 + 0.2 * math.sin(th), 0.0)
    tangent = (-math.sin(th), math.cos(th), 0.0)
    radial = cross(normal, tangent)
    m = cross(tuple(di - pi for di, pi in zip(d_pt, p)), Fz)
    stations[label] = {
        "point": fmt(p), "torsion_abs_Nm": abs(dot(m, tangent)),
        "out_of_plane_bending_abs_Nm": abs(dot(m, radial)),
        "in_plane_bending_abs_Nm": abs(dot(m, normal)),
        "shear_normal_to_plane_abs_N": 1000.0, "axial_abs_N": 0.0,
    }
Fb1, Mb1 = anchor_action([(d_pt, Fz)])
out["REF-B1"] = {"stations": stations, "anchor": {"F": fmt(Fb1), "M": fmt(Mb1)},
                 "flexibility_control": "change the user flexibility factor; these resultants are unchanged",
                 "headline": "withheld while the arc has no qualified maximum"}

# ======================================================================= revision 2
REV2 = {}
Z1, A1_, EI1 = C1["Z"], C1["A"], EI
K_BEAM_1M = 3 * EI1 / 1.0**3  # Euler-Bernoulli tip stiffness of the C1 1 m cantilever


def zero_scaled(value, scale):
    return {"value": value, "zero_scale": scale}


# ---- Case definitions left implicit in revision 1 (S0 coverage gaps)
REV2["case_definitions"] = {
    "REF-M14-S1": "length 1 m along +x; anchor at x=0 (all six); loads at the free tip",
    "REF-M14-X1": {"length_m": 1.0, "node_i_restraints": ["UX", "UY", "UZ", "RX"],
                   "node_j_restraints": ["UY", "UZ"], "w_z": "+z over the whole span",
                   "end_couple": "+z at node j"},
    "REF-B1": {"arc_pipe_y_reference": [1 / math.sqrt(2), -1 / math.sqrt(2), 0.0],
               "note": "the product bows the arc toward +y_reference (PP:4928-4935); this value gives the tangent-consistent arc"},
    "zero_scale_rule": "a zero expectation uses the case's governing load-effect magnitude as scale: force components use the case's largest applied force, moments its largest applied moment or force*arm, stresses the case's governing nonzero stress",
}
REV2["zero_scales_rev1"] = {
    "REF-M14-S1.pure_torque_only_normal_max_Pa": 6e6,
    "REF-M05-T.force_components_N": 500.0,
    "REF-M05-T.moment_y_z_Nm": 500.0,
    "REF-M05-R1.zero_components": 60.0,
    "REF-M05-SPRING.zero_components": 1000.0,
    "REF-M05-COMB.zero_components": 2000.0,
    "REF-M08-L.combination_zero_rows": {"force_N": 1000.0, "moment_Nm": 1000.0, "stress_Pa": 1000.0 / Z1},
    "REF-M08-L.member_ab_end_b_bending": 1000.0 / Z1,
    "REF-B1.in_plane_and_axial": {"force_N": 1000.0, "moment_Nm": 1200.0},
}

# ---- R1 translated (moment origin): anchor at O, loads carried with it
O = (5.0, -3.0, 2.0)
tip_t = add(O, (2.0, 0.0, 0.0))
Ft, Mt = anchor_action([(tip_t, (10.0, -20.0, 30.0))], couples=[(4.0, 5.0, -6.0)],
                       line_loads=[(O, tip_t, (0.0, -3.0, 0.0))], origin=O)
wrong = add(Mt, cross(O, Ft))
REV2["REF-M05-R1-TRANSLATED"] = {"anchor_node": list(O), "anchor_F_N": fmt(Ft), "anchor_M_about_node_Nm": fmt(Mt),
                                 "negative_control_M_about_global_origin_Nm": fmt(wrong)}

# ---- COMB: moment envelope, and a displacement-magnitude combination that discriminates
MA_n, MT_n = norm(MA), norm(MT)
u_tip = 1000.0 / K_BEAM_1M
REV2["REF-M05-COMB-2"] = {
    "moment_envelope_A1_T": {"cases": "REF-M05-COMB A1 (tip Fy=+1000) and T (tip Mx=+500)",
                             "envelope_max_moment_magnitude_Nm": max(MA_n, MT_n),
                             "negative_control_sum_Nm": MA_n + MT_n},
    "moment_envelope_A1_A2": {"envelope_max_moment_magnitude_Nm": max(MA_n, norm(anchor_action([((1, 0, 0), (0, 0, 1000.0))])[1])),
                              "negative_control_sum_Nm": MA_n + norm(anchor_action([((1, 0, 0), (0, 0, 1000.0))])[1])},
    "cases": "A1 tip Fy=+1000, A2 tip Fz=+1000 (1 m, C1)",
    "A1_plus_A2_anchor": {"F": fmt(add(FA, anchor_action([((1, 0, 0), (0, 0, 1000.0))])[0])),
                          "M": fmt(add(MA, anchor_action([((1, 0, 0), (0, 0, 1000.0))])[1]))},
    "A1_plus_A2_tip_displacement_magnitude_mm": math.sqrt(2) * u_tip * 1e3,
    "A1_minus_A2_tip_displacement_magnitude_mm": math.sqrt(2) * u_tip * 1e3,
    "negative_control_magnitude_sum_mm": 2 * u_tip * 1e3,
    "negative_control_magnitude_difference_mm": zero_scaled(0.0, u_tip * 1e3),
}

# ---- X1 partial restraints (rigid rows that restrain only some DOFs)
wz, cz = 8e6 * S1["Z"], 1e6 * S1["Z"]
REV2["REF-M14-X1-SUPPORTS"] = {
    "pin_i_support_on_pipe": fmt((0.0, cz / 1.0, -wz / 2.0, 0.0, 0.0, 0.0)),
    "roller_j_support_on_pipe": fmt((0.0, -cz / 1.0, -wz / 2.0, 0.0, 0.0, 0.0)),
    "zero_scale_N_or_Nm": wz / 2.0,
    "check": "sum Fz + w_z*L = 0; couple of the Fy pair (-cz about z) cancels the applied +cz",
    "control_scope": "catches DOF-slot mapping errors in rigid rows that restrain only some DOFs; it does not test attribution",
}

# ---- Thermal axial force in the straight maximum (fixed-fixed, free thermal strain)
alpha, dT = 1.2e-5, 100.0  # invented test values, not library data
N_th = -E * A1_ * alpha * dT
REV2["REF-M14-TH"] = {
    "model": "C1 pipe, 1 m, anchors at both ends, element thermal change +100 K, alpha 1.2e-5/K (invented)",
    "wall_axial_force_N": N_th, "circular_normal_max_Pa": abs(N_th) / A1_,
    "anchor_a_support_on_pipe": fmt((E * A1_ * alpha * dT, 0, 0, 0, 0, 0)),
    "anchor_b_support_on_pipe": fmt((-E * A1_ * alpha * dT, 0, 0, 0, 0, 0)),
    "zero_scale_N": E * A1_ * alpha * dT,
    "zero_scale_Nm": E * A1_ * alpha * dT * C1["ro"],
    "zero_scale_Nm_rationale": "the axial force times the outer radius: the largest moment a misplaced axial force could produce at the section",
}

# ---- Constant effort (DEC-049 ideal element: constant force along +axis of its one DOF)
P_ce = 375.0
def ce_case(tip_fy):
    # CE at the tip on UY acts on the pipe with +P_ce; the anchor carries the rest.
    F, M = anchor_action([((1.0, 0.0, 0.0), (0.0, tip_fy + P_ce, 0.0))])
    return {"ce_support_on_pipe": fmt((0.0, P_ce, 0.0, 0.0, 0.0, 0.0)), "anchor_F": fmt(F), "anchor_M": fmt(M),
            "tip_uy_m": (tip_fy + P_ce) / K_BEAM_1M}
W1, W2 = ce_case(-1000.0), ce_case(-500.0)
true_15 = anchor_action([((1.0, 0.0, 0.0), (0.0, -1000.0 - 0.5 * 500.0 + P_ce, 0.0))])
REV2["REF-CE"] = {
    "model": "C1 cantilever 1 m along +x, anchor at a; at tip b a constant-effort support with restraints [UY] and constant_load 375 N",
    "case_W1_tip_Fy_-1000": W1, "case_W2_tip_Fy_-500": W2,
    "balance": "anchor + CE action + applied = 0, CE counted once (its force is in the RHS, so it is not also an applied load)",
    "negative_control_main_reaction_resultant_N": zero_scaled(0.0, P_ce),
    "comb_0.5W1_0.5W2_sum_factors_1": {"published": True, "anchor_Fy_N": 0.5 * W1["anchor_F"][1] + 0.5 * W2["anchor_F"][1],
                                         "ce_applied_N": P_ce},
    "comb_1.0W1_0.5W2_sum_factors_1.5": {"published": False,
                                          "true_anchor_Fy_N_if_solved": true_15[0][1],
                                          "negative_control_linear_anchor_Fy_N": W1["anchor_F"][1] + 0.5 * W2["anchor_F"][1],
                                          "negative_control_ce_applied_N": 1.5 * P_ce},
    "subtraction_W1_minus_W2": {"published": True, "anchor_Fy_N": W1["anchor_F"][1] - W2["anchor_F"][1],
                                 "ce_applied_N": zero_scaled(0.0, P_ce),
                                 "equals_difference_of_states": True},
    "non_consuming_variant": {"model": "same CE with restraints []",
                              "ce_rows": "absent, withheld as CONSTANT_EFFORT_NOT_CONSUMED",
                              "anchor_F_W1": fmt(anchor_action([((1.0, 0.0, 0.0), (0.0, -1000.0, 0.0))])[0]),
                              "anchor_M_W1": fmt(anchor_action([((1.0, 0.0, 0.0), (0.0, -1000.0, 0.0))])[1])},
}

# ---- Two devices at one node, different DOFs; and law + residual devices on one DOF (N-1)
k_s, P_y, P_x = 1.0e6, 1000.0, 2000.0
u2 = P_y / (K_BEAM_1M + k_s)
Fa2, Ma2 = anchor_action([((1.0, 0.0, 0.0), (0.0, P_y - k_s * u2, 0.0))])
REV2["REF-M05-SPRING2"] = {
    "model": "C1 cantilever 1 m; tip spring k=1e6 N/m on UY (record 1) and rigid guide on UX (record 2); tip Fx=+2000, Fy=+1000",
    "spring_on_pipe": fmt((0.0, -k_s * u2, 0.0, 0.0, 0.0, 0.0)),
    "guide_on_pipe": fmt((-P_x, 0.0, 0.0, 0.0, 0.0, 0.0)),
    "anchor": {"F": fmt((0.0, Fa2[1], 0.0)), "M": fmt(Ma2),
               "note": "tip UX is held by the guide, so the axial load goes to the guide and the anchor Fx is zero"},
    "zero_scale_N": P_x,
}
u_g = 350.0 / (K_BEAM_1M + k_s)
REV2["REF-M05-SPRING-GAP"] = {
    "model": "C1 cantilever 1 m; tip spring k=1e6 N/m on UY and a gap support on UY with 1 mm gap (inactive); tip Fy=+350",
    "gap_support_json": {"family": "nonlinear", "restraints": [], "nonlinear": {"behavior": "gap", "dof": "UY", "initial_state": "inactive", "closes_when": "positive_displacement", "gap": {"value": 0.001, "unit": "m"}}},
    "tip_uy_m": u_g, "gap_active": u_g >= 0.001,
    "spring_on_pipe_Fy_N": -k_s * u_g, "gap_on_pipe": zero_scaled(0.0, 350.0),
    "anchor_Fy_N": -(350.0 - k_s * u_g), "anchor_Mz_Nm": -(350.0 - k_s * u_g),
    "attribution": "unique (one residual-determined device): both devices published",
}
REV2["REF-ATTR"] = {
    "model": "C1 cantilever 1 m; tip rigid guide on UY (record 1) and one-way support on UY (record 2); tip Fy=-1000",
    "one_way_support_json": {"family": "nonlinear", "restraints": [], "nonlinear": {"behavior": "one_way", "dof": "UY", "initial_state": "inactive", "active_when": "positive_reaction"}},
    "global_balance_rule": "exempt: both tip devices are withheld, so published rows cannot close the balance; instead assert anchor rows zero, both withheld records present, and no zero-filled tip rows",
    "anchor_on_pipe": zero_scaled([0.0] * 6, 1000.0),
    "tip_nodal_total_on_pipe_Fy_N": 1000.0,
    "device_rows": "both tip devices withheld with SUPPORT_ACTION_ATTRIBUTION_WITHHELD",
    "why_anchor_is_zero": "the prop holds the loaded tip at uy=0, so the beam is unstrained",
    "negative_control": "zero-filling the withheld devices breaks the balance by 1000 N",
    "if_refused_pre_solve": "a targeted refusal is acceptable; record which the product does",
}

# ---- Nonlinear superposition counterexample (ruling 1)
g = 0.0004
u_c = 800.0 / K_BEAM_1M
REV2["REF-NL-C3"] = {
    "model": "C1 cantilever 1 m; gap stop at the tip closing on +UY after 0.4 mm; cases P1, P2 each tip Fy=+800; mechanics combination P1+P2",
    "gap_support_json": {"family": "nonlinear", "restraints": [], "nonlinear": {"behavior": "gap", "dof": "UY", "initial_state": "inactive", "closes_when": "positive_displacement", "gap": {"value": 0.0004, "unit": "m"}}},
    "case_tip_uy_m": u_c, "case_stop_force_N": zero_scaled(0.0, 800.0), "case_stop_active": u_c >= g,
    "superposed_uy_m": 2 * u_c, "superposition_penetrates_stop": 2 * u_c > g,
    "true_combined_load_uy_m": g, "true_combined_load_stop_on_pipe_Fy_N": -(1600.0 - K_BEAM_1M * g),
    "expectation": "mechanics combination rows withheld with NONLINEAR_COMBINATION_REQUIRES_SOLVE; case rows kept; subtraction P1-P2 published as a labelled signed difference",
    "stress_reference_C3_1d": {"k": 1e6, "g": 0.001, "F_each": 800.0, "superposed_u": 0.0016,
                               "true_u": 0.001, "true_lambda": 600.0},
}

# ---- Intensified equal-factor measure i*hypot(My,Mz)/Z (ruling 10); member Z, no k, no axial
def bend_at(point, beyond_loads, axis):
    m = (0.0, 0.0, 0.0)
    for pos, f in beyond_loads:
        m = add(m, cross(tuple(a - b for a, b in zip(pos, point)), f))
    ax = axis
    torsion = dot(m, ax)
    bending = norm(tuple(mi - torsion * ai for mi, ai in zip(m, ax)))
    force = add(*[f for _, f in beyond_loads])
    return torsion, bending, dot(force, ax)
i_b = 1.15
L1 = [((1.0, 1.0, 0.0), (0.0, 0.0, 1000.0))]
L2 = [((1.0, 1.0, 0.0), (1000.0, 0.0, 1000.0))]
t_bc1, b_bc1, _ = bend_at((1, 0, 0), L1, (0, 1, 0))
t_ab1, b_ab1, _ = bend_at((1, 0, 0), L1, (1, 0, 0))
t_bc2, b_bc2, n_bc2 = bend_at((1, 0, 0), L2, (0, 1, 0))
t_ab2, b_ab2, n_ab2 = bend_at((1, 0, 0), L2, (1, 0, 0))
REV2["REF-I-L"] = {
    "model": "REF-M08-L geometry; geometry-only bend marker at b with user SIF i=1.15 (k=1.08 not consumed)",
    "L1_load_c_Fz_1000": {"pipe_bc_end_i_Pa": i_b * b_bc1 / Z1,
                           "pipe_ab_end_j_Pa": zero_scaled(i_b * b_ab1 / Z1, i_b * 1000.0 / Z1),
                           "pipe_ab_end_j_torsion_Nm": t_ab1},
    "L2_load_c_F_1000_0_1000": {"pipe_bc_end_i_Pa": i_b * b_bc2 / Z1, "pipe_ab_end_j_Pa": i_b * b_ab2 / Z1,
                                 "pipe_ab_end_j_axial_N": n_ab2,
                                 "anchor": {"F": fmt(anchor_action(L2)[0]), "M": fmt(anchor_action(L2)[1])},
                                 "negative_control_abs_sum_bc_Pa": i_b * 2000.0 / Z1,
                                 "negative_control_with_k_bc_Pa": i_b * 1.08 * b_bc2 / Z1,
                                 "negative_control_axial_added_unintensified_ab_Pa": abs(n_ab2) / A1_ + i_b * b_ab2 / Z1,
                                 "negative_control_axial_added_intensified_ab_Pa": i_b * (abs(n_ab2) / A1_ + b_ab2 / Z1)},
    "no_k_variant": {"model": "L1 marker with sif_user_value 1.15 and no flexibility_factor_user_value",
                     "pipe_bc_end_i_Pa": i_b * b_bc1 / Z1, "note": "the measure does not require k (NOTE-1)"},
    "flexibility_control": "k 1.08 -> 2.16 leaves every value unchanged",
    "combination_control": "no intensified row for any combination",
    "missing_sif_control": "marker without sif_user_value: no row, and no default factor",
}
T_loads_bc = [((1.0, 1.0, 0.0), (800.0, 0.0, 1000.0))]  # revision 3: Fx=+800 at c separates header from b-e
T_loads_be = [((2.0, 0.0, 0.0), (0.0, 0.0, 500.0))]
_, b_hdr, _ = bend_at((1, 0, 0), T_loads_bc + T_loads_be, (1, 0, 0))
_, b_br, _ = bend_at((1, 0, 0), T_loads_bc, (0, 1, 0))
_, b_be, _ = bend_at((1, 0, 0), T_loads_be, (1, 0, 0))
REV2["REF-I-T"] = {
    "model": "anchor a(0,0,0); header a-b to b(1,0,0) (branch_header_pipe_ref), header continuation b-e to e(2,0,0) (not referenced), branch b-c to c(1,1,0) (branch_branch_pipe_ref); tee at b with header SIF 1.3 and branch SIF 2.0; loads (Fx=+800, Fz=+1000) at c and Fz=+500 at e",
    "header_ab_end_j_bending_Nm": b_hdr, "branch_bc_end_i_bending_Nm": b_br,
    "negative_control_header_equals_unreferenced_be_Pa": 1.3 * b_be / Z1,
    "header_ab_end_j_Pa": 1.3 * b_hdr / Z1, "branch_bc_end_i_Pa": 2.0 * b_br / Z1,
    "unreferenced_be_end_i": "no row; COMPONENT_INTENSIFIED_COVERAGE_INCOMPLETE names pipe b-e",
    "unreferenced_be_bending_Nm_for_information": b_be,
}

# ---- REF-B1 signed values (S0 item 2): arc tangent frame and chord frame
C_arc, R_arc = (1.0, 0.2, 0.0), 0.2
b_pt, c_pt2 = (1.0, 0.0, 0.0), (1.2, 0.2, 0.0)
rad_i = tuple(x - y for x, y in zip(b_pt, C_arc))
rad_j = tuple(x - y for x, y in zip(c_pt2, C_arc))
zl = tuple(v / norm(cross(rad_i, rad_j)) for v in cross(rad_i, rad_j))
xl = tuple(v / norm(rad_i) for v in rad_i)
yl = cross(zl, xl)
def arc_frame(theta):
    loc = lambda a, b, c: tuple(a * xl[k] + b * yl[k] + c * zl[k] for k in range(3))  # noqa: E731
    point = tuple(C_arc[k] + R_arc * (math.cos(theta) * xl[k] + math.sin(theta) * yl[k]) for k in range(3))
    return point, loc(-math.sin(theta), math.cos(theta), 0.0), loc(-math.cos(theta), -math.sin(theta), 0.0), zl
signed = {}
for label, frac in (("end_i_b", 0.0), ("quarter_1", 0.25), ("midspan", 0.5), ("quarter_3", 0.75), ("end_j_c", 1.0)):
    p, t, inward, nrm = arc_frame(frac * math.pi / 2)
    m = cross(tuple(d - q for d, q in zip(d_pt, p)), Fz)
    signed[label] = {"N": dot(Fz, t), "Vy": dot(Fz, inward), "Vz": dot(Fz, nrm), "T": dot(m, t),
                     "My": dot(m, inward), "Mz": dot(m, nrm)}
xc = tuple(v / norm(tuple(a - b for a, b in zip(c_pt2, b_pt))) for v in tuple(a - b for a, b in zip(c_pt2, b_pt)))
yc = (1 / math.sqrt(2), -1 / math.sqrt(2), 0.0)
zc = cross(xc, yc)
def chord(point):
    m = cross(tuple(d - q for d, q in zip(d_pt, point)), Fz)
    return [dot(Fz, xc), dot(Fz, yc), dot(Fz, zc), dot(m, xc), dot(m, yc), dot(m, zc)]
REV2["REF-B1-SIGNED"] = {
    "section_resultant_convention": "action of everything beyond the cut (j side) reduced to the section point; frame (x tangent b->c, y toward the centre, z bend-plane normal = unit(radial_i x radial_j))",
    "bend_plane_normal": fmt(zl),
    "stations": signed,
    "chord_frame": {"x": fmt(xc), "y": fmt(yc), "z": fmt(zc),
                    "cut_face_b": chord(b_pt), "cut_face_c": chord(c_pt2),
                    "product_end_i_rows_node_on_element": [-v for v in chord(b_pt)],
                    "product_end_j_rows_node_on_element": chord(c_pt2)},
    "endpoint_stress_rows_tangent_frame": {
        "end_i_bending_normal_y_Pa": signed["end_i_b"]["My"] / Z1,
        "end_i_torsional_shear_Pa": signed["end_i_b"]["T"] * C1["ro"] / C1["J"],
        "end_j_bending_normal_y_Pa": signed["end_j_c"]["My"] / Z1,
        "zero_scale_Pa": 1000.0 / Z1},
    "zero_scale": {"force_N": 1000.0, "moment_Nm": 1200.0},
}

# ---- Kinked-arc geometry (decision list: y_reference warning); product construction PP:4928-5018
def arc_end_tangents(b, c, yref, radius):
    chord_v = tuple(y - x for x, y in zip(b, c)); L = norm(chord_v); cu = tuple(v / L for v in chord_v)
    ax = dot(yref, cu); pn = tuple(yr - ax * u for yr, u in zip(yref, cu)); pm = norm(pn)
    h = math.sqrt(radius**2 - (L / 2) ** 2)
    centre = tuple((x + y) / 2 - h * n / pm for x, y, n in zip(b, c, pn))
    ri = tuple(x - y for x, y in zip(b, centre)); rj = tuple(x - y for x, y in zip(c, centre))
    n = cross(ri, rj); nu = tuple(v / norm(n) for v in n)
    ti = cross(nu, tuple(v / norm(ri) for v in ri)); tj = cross(nu, tuple(v / norm(rj) for v in rj))
    return centre, ti, tj
kinked = {}
for name, yref in (("consistent", yc), ("kinked_y_ref_0_0_1", (0.0, 0.0, 1.0))):
    centre, ti, tj = arc_end_tangents(b_pt, c_pt2, yref, R_arc)
    kinked[name] = {"centre": fmt(centre),
                    "angle_at_b_deg": math.degrees(math.acos(max(-1.0, min(1.0, dot(ti, (1.0, 0.0, 0.0)))))),
                    "angle_at_c_deg": math.degrees(math.acos(max(-1.0, min(1.0, dot(tj, (0.0, 1.0, 0.0))))))}
REV2["REF-B1-TANGENCY"] = {"cases": kinked,
                           "expectation": "consistent: no CURVED_BEND_TANGENT_DISCONTINUITY; kinked: warning naming both ends, solve still published"}

# ---- REF-B2: statically indeterminate arc (N-6); thin curved beam, bending compliance scaled by k,
# axial compliance included, shear and curvature coupling neglected (the element's stated energy terms).
EA1 = E * A1_
def b2(k, n=20000):
    P = 1000.0
    h = (math.pi / 2) / n
    s = [0.0, 0.0]
    for idx in range(n + 1):
        phi = -math.pi / 2 + idx * h
        w = 1 if idx in (0, n) else (4 if idx % 2 else 2)
        px, py = 1.0 + R_arc * math.cos(phi), 0.2 + R_arc * math.sin(phi)
        dx, dy = 1.2 - px, 0.2 - py
        m0, m1 = dx * P, -dy          # M = m0 + X*m1 (in-plane moment of the j-side forces (X, P))
        n0, n1 = P * math.cos(phi), -math.sin(phi)  # N = (X, P).t, t = (-sin phi, cos phi)
        s[0] += w * (k * m0 * m1 / EI1 + n0 * n1 / EA1)
        s[1] += w * (k * m1 * m1 / EI1 + n1 * n1 / EA1)
    X = -s[0] / s[1]
    return {"roller_on_pipe_Fx_N": X,
            "anchor_on_pipe": fmt((-X, -P, 0.0, 0.0, 0.0, 0.2 * X - 0.2 * P)),
            "section_at_b_tangent_frame_for_information": {"N": X, "Vy_inward": P, "Mz_in_plane": 0.2 * P - 0.2 * X},
            "chord_frame_cut_face_b": fmt(((X + P) / math.sqrt(2), (X - P) / math.sqrt(2), 0.0, 0.0, 0.0, -0.2 * (P - X))),
            "product_end_i_rows_node_on_element": fmt((-(X + P) / math.sqrt(2), -(X - P) / math.sqrt(2), 0.0, 0.0, 0.0, 0.2 * (P - X)))}
REV2["REF-B2"] = {
    "model": "arc b(1,0,0)->c(1.2,0.2,0), R 0.2, y_reference as REF-B1, C1 section, E 200 GPa; anchor at b; roller at c restraining UX only; in-plane load Fy=+1000 at c",
    "assumptions": "thin curved beam: bending compliance k/EI, axial 1/EA, no shear or curvature coupling (curved_bend end_flexibility terms)",
    "k1": b2(1.0), "k2": b2(2.0), "k4": b2(4.0),
    "quadrature_check_k2_n40000": b2(2.0, 40000)["roller_on_pipe_Fx_N"],
    "point": "resultants change with k (indeterminate), unlike REF-B1; no published stress scales with k at fixed resultants",
    "chord_frame": "x (1,1,0)/sqrt2, y = y_reference (1,-1,0)/sqrt2, z = x cross y = (0,0,-1); the tangent-frame values name no force row",
    "s2_confirmation": "S2a confirms the product accepts an arc span with no adjacent straight pipe; if refused, record the refusal",
}

# ---- Displacement tie at the per-case site (SF-6)
u_t = 1000.0 / K_BEAM_1M
REV2["REF-M33-TIE"] = {
    "model": "anchor o(0,0,0); cantilevers o->p(1,0,0) and o->q(-1,0,0), C1; case: Fz=+1000 at p and at q",
    "max_displacement_mm": u_t * 1e3, "tied_nodes": ["node:p", "node:q"],
    "tie_rule": "record both raw magnitudes in both node orders and both solver modes; if they are bitwise equal, assert the identity tie-break (same node for either order); if not, accept either node when both lie within 1e-9 of the value and report 'tie not exercised'; no new tolerance",
}

REV2["tie_rule_REF-M08-L"] = REV2["REF-M33-TIE"]["tie_rule"].replace("node", "location")
REV2["global_balance_rule"] = "every reference checks force and moment balance from published support rows and applied loads, except REF-ATTR (explicit exemption above)"
out["revision_2"] = REV2
print(json.dumps(out, indent=1, sort_keys=True))
