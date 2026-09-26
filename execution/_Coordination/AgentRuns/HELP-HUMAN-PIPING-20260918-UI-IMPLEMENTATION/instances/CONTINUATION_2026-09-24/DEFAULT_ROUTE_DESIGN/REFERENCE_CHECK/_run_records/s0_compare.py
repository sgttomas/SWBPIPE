#!/usr/bin/env python3
"""S0 comparison of the checker's values (s0_derive.out dict) with the author's frozen stdout."""
import json, sys, math, importlib.util
spec = importlib.util.spec_from_file_location("d", "/tmp/s0check/s0_derive.py")
d = importlib.util.module_from_spec(spec); spec.loader.exec_module(d); mine = d.out
a = json.load(open(sys.argv[1]))
def rel(x, y, scale=0.0):
    den = max(abs(x), abs(y), scale)
    return 0.0 if den == 0 else abs(x - y) / den
rows = []
def cmp(case, q, m, t, scale=0.0):
    r = rel(m, t, scale); rows.append((case, q, m, t, r, r <= 1e-9))
def cmpv(case, q, m, t, scale):
    for i, (x, y) in enumerate(zip(m, t)):
        cmp(case, f"{q}[{i}]", float(x), float(y), scale)
A = a
cmp("M14-A", "root max Pa", mine["M14-A root maximum [Pa]"], A["REF-M14-A"]["circular_normal_max_Pa"])
cmp("M14-A", "ctrl abs-sum Pa", mine["M14-A abs-sum control [Pa]"], A["REF-M14-A"]["negative_control_abs_sum_Pa"])
S = A["REF-M14-S1"]
cmp("M14-S1", "normal max Pa", mine["S1 normal maximum [Pa]"], S["circular_normal_max_Pa"])
cmp("M14-S1", "ctrl abs-sum Pa", mine["S1 abs-sum control [Pa]"], S["negative_control_abs_sum_Pa"])
cmp("M14-S1", "rotated max Pa", mine["S1 rotated normal maximum [Pa]"], S["rotated_circular_normal_max_Pa"])
cmp("M14-S1", "torsional shear Pa", mine["S1+Mx torsional shear [Pa]"], S["with_tip_torque_221.4pi_torsional_shear_Pa"])
cmp("M14-S1", "pure torque normal Pa", mine["S1 pure torque normal maximum [Pa]"], S["pure_torque_only_normal_max_Pa"], 7e6)
X = A["REF-M14-X1"]
cmp("M14-X1", "t*", mine["X1 t* analytic"], X["t_star"])
cmp("M14-X1", "max Pa", mine["X1 maximum [Pa]"], X["circular_normal_max_Pa"])
cmp("M14-X1", "ctrl eight-sign Pa", mine["X1 eight-sign candidates control [Pa]"], X["negative_control_old_eight_sign_candidates_Pa"])
cmp("M14-X1", "ctrl abs-sum Pa", mine["X1 abs-sum objective control [Pa]"], X["negative_control_abs_sum_objective_max_Pa"])
cmp("M14-X1", "w_z N/m", 8e6 * d.secS["Z"], X["w_z_N_per_m"]); cmp("M14-X1", "couple N*m", 1e6 * d.secS["Z"], X["end_couple_Nm"])
G = A["REF-M33-G"]
cmp("M33-G", "headline Pa", mine["M33 headline (max over A,B) [Pa]"], G["headline_Pa"])
cmp("M33-G", "disp headline mm", mine["M33 displacement headline [m]"] * 1e3, G["displacement_headline_mm"])
cmp("M33-G", "case A tip mm", mine["M33 case A tip displacement [m]"] * 1e3, G["tip_displacement_A_mm"])
cmp("M33-G", "ctrl first-case Pa", mine["M33 first-case control [Pa]"], G["negative_control_first_case_headline_Pa"])
T = A["REF-M05-T"]
cmpv("M05-T", "anchor", mine["M05-T anchor"], T["anchor_F_N"] + T["anchor_M_Nm"], 500)
cmp("M05-T", "|M|", 500.0, T["moment_magnitude_Nm"]); cmp("M05-T", "|F|", 0.0, T["force_magnitude_N"], 500)
R = A["REF-M05-R1"]
cmpv("M05-R1", "point", mine["R1 point loads"], R["anchor_F_N"] + R["anchor_M_Nm"], 60)
w = R["with_uniform_0_-3_0_N_per_m"]; cmpv("M05-R1", "with w", mine["R1 with distributed"], w["anchor_F_N"] + w["anchor_M_Nm"], 60)
for mk, ak in (("cyclic x->y->z->x", "perm_xyz_to_yzx"), ("Rz(30deg)", "rot_z_30deg")):
    rr = R[f"rotated_{ak}"]
    cmpv("M05-R1", f"{ak} direct", mine[f"R1 rotated {mk}: direct"], rr["anchor_F_N"] + rr["anchor_M_Nm"], 60)
    cmpv("M05-R1", f"{ak} Q.R", mine[f"R1 rotated {mk}: Q.R"], rr["equals_Q_times_unrotated"][0] + rr["equals_Q_times_unrotated"][1], 60)
P = A["REF-M05-SPRING"]
cmp("SPRING", "spring Fy N", mine["SPRING spring-on-pipe Fy [N]"], P["spring_on_pipe_Fy_N"])
cmpv("SPRING", "anchor", mine["SPRING anchor"], P["anchor_F_N"] + P["anchor_M_Nm"], 1000)
C = A["REF-M05-COMB"]
cmpv("COMB", "2A1-T", mine["COMB 2A1-T"], C["mechanics_2A1_minus_T"]["F"] + C["mechanics_2A1_minus_T"]["M"], 2000)
cmp("COMB", "2A1-T |M|", mine["COMB 2A1-T |M|"], C["mechanics_2A1_minus_T"]["moment_magnitude_Nm"])
cmp("COMB", "2A1-T |F|", mine["COMB 2A1-T |F|"], C["mechanics_2A1_minus_T"]["force_magnitude_N"])
cmp("COMB", "A1-T |M|", mine["COMB A1-T |M|"], C["subtraction_A1_minus_T"]["moment_magnitude_Nm"])
cmp("COMB", "envelope |F|", mine["COMB envelope(A1,T) |F|"], C["envelope_max_force_magnitude_N"])
cmp("COMB", "ctrl magnitude algebra", mine["COMB magnitude-algebra control 2|M_A1|-|M_T|"], C["negative_control_magnitude_algebra_moment_Nm"])
L = A["REF-M08-L"]
cmp("M08-L", "headline Pa", mine["L headline [Pa]"], L["headline_Pa"])
cmpv("M08-L", "anchor up", mine["L up anchor"], L["anchor_up"]["F"] + L["anchor_up"]["M"], 1000)
cmp("M08-L", "ctrl comb SIF MPa", 2 * mine["L main SIF row i*k*sigma (k=1.08) [Pa]"] / 1e6, L["negative_control_old_multiplier_row_MPa"])
B = A["REF-B1"]["stations"]
for ml, al in (("b", "arc_i_at_b"), ("midspan", "arc_mid"), ("c", "arc_j_at_c")):
    Tm, My, Mz = mine[f"B1 {ml}: (T,My,Mz)"]
    cmp("B1", f"{al} |T|", abs(Tm), B[al]["torsion_abs_Nm"], 1200); cmp("B1", f"{al} |Mo|", abs(My), B[al]["out_of_plane_bending_abs_Nm"], 1200)
    cmp("B1", f"{al} |Mi|", abs(Mz), B[al]["in_plane_bending_abs_Nm"], 1200)
cmpv("B1", "anchor", mine["B1 anchor"], A["REF-B1"]["anchor"]["F"] + A["REF-B1"]["anchor"]["M"], 1200)
print(f"{'case':8} {'quantity':28} {'checker':>24} {'author':>24} {'rel diff':>10} ok")
for c, q, m, t, r, ok in rows:
    print(f"{c:8} {q:28} {m:24.17g} {t:24.17g} {r:10.2e} {'yes' if ok else 'NO'}")
print("ALL AGREE" if all(r[5] for r in rows) else "DISAGREEMENT")
print("rows:", len(rows), "max rel diff:", max(r[4] for r in rows))
print("REF-CE in author output:", "REF-CE" in A)
