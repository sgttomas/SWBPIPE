#!/usr/bin/env python3
"""S0' comparison: checker's s0p_derive values against the author's revision_2 output."""
import json, sys, importlib.util
spec = importlib.util.spec_from_file_location("d", "/tmp/s0check2/s0p_derive.py")
d = importlib.util.module_from_spec(spec); spec.loader.exec_module(d); m = d.out
A = json.load(open(sys.argv[1]))["revision_2"]
rows = []
def rel(x, y, s):
    den = max(abs(x), abs(y), s)
    return 0.0 if den == 0 else abs(x - y) / den
def c(case, q, mine, theirs, s=0.0):
    if isinstance(mine, bool) or isinstance(theirs, bool):
        rows.append((case, q, str(mine), str(theirs), 0.0 if mine == theirs else 1.0)); return
    rows.append((case, q, float(mine), float(theirs), rel(float(mine), float(theirs), s)))
def cv(case, q, mine, theirs, s):
    for i, (x, y) in enumerate(zip(mine, theirs)):
        c(case, f"{q}[{i}]", x, y, s)
r = A["REF-M05-R1-TRANSLATED"]
cv("R1T", "anchor F", m["R1T anchor F"], r["anchor_F_N"], 60); cv("R1T", "anchor M node", m["R1T anchor M about node"], r["anchor_M_about_node_Nm"], 60)
cv("R1T", "ctrl M origin", m["R1T control M about global origin"], r["negative_control_M_about_global_origin_Nm"], 60)
r = A["REF-M05-COMB-2"]
c("COMB2", "envelope max |M|", m["COMB2 envelope max |M|"], r["moment_envelope_A1_A2"]["envelope_max_moment_magnitude_Nm"])
c("COMB2", "ctrl sum |M| (cases A1,A2)", m["COMB2 control sum |M|"], r["moment_envelope_A1_A2"]["negative_control_sum_Nm"])
cv("COMB2", "A1+A2 anchor", m["COMB2 A1+A2 anchor"], r["A1_plus_A2_anchor"]["F"] + r["A1_plus_A2_anchor"]["M"], 1000)
c("COMB2", "A1+A2 |u| mm", m["COMB2 A1+A2 |u| mm"], r["A1_plus_A2_tip_displacement_magnitude_mm"])
c("COMB2", "A1-A2 |u| mm", m["COMB2 A1-A2 |u| mm"], r["A1_minus_A2_tip_displacement_magnitude_mm"])
c("COMB2", "ctrl mag sum mm", m["COMB2 control magnitude sum mm"], r["negative_control_magnitude_sum_mm"])
c("COMB2", "ctrl mag diff mm", m["COMB2 control magnitude difference mm"], r["negative_control_magnitude_difference_mm"]["value"], r["negative_control_magnitude_difference_mm"]["zero_scale"])
r = A["REF-M14-X1-SUPPORTS"]
cv("X1S", "pin", m["X1 pin i on pipe"], r["pin_i_support_on_pipe"], r["zero_scale_N_or_Nm"])
cv("X1S", "roller", m["X1 roller j on pipe"], r["roller_j_support_on_pipe"], r["zero_scale_N_or_Nm"])
r = A["REF-M14-TH"]
c("TH", "wall N", m["TH wall axial force N"], r["wall_axial_force_N"]); c("TH", "max Pa", m["TH max normal Pa"], r["circular_normal_max_Pa"])
cv("TH", "anchor a", m["TH anchor a on pipe"], r["anchor_a_support_on_pipe"], r["zero_scale_N"])
cv("TH", "anchor b", m["TH anchor b on pipe"], r["anchor_b_support_on_pipe"], r["zero_scale_N"])
r = A["REF-CE"]
w1, w2 = r["case_W1_tip_Fy_-1000"], r["case_W2_tip_Fy_-500"]
cv("CE", "W1 anchor", m["CE W1 anchor"], w1["anchor_F"] + w1["anchor_M"], 1000); c("CE", "W1 tip uy", m["CE W1 tip uy"], w1["tip_uy_m"])
cv("CE", "W1 CE on pipe", m["CE support on pipe"], w1["ce_support_on_pipe"], 1000)
cv("CE", "W2 anchor", m["CE W2 anchor"], w2["anchor_F"] + w2["anchor_M"], 500); c("CE", "W2 tip uy", m["CE W2 tip uy"], w2["tip_uy_m"])
cv("CE", "W2 CE on pipe", m["CE support on pipe"], w2["ce_support_on_pipe"], 500)
k5 = r["comb_0.5W1_0.5W2_sum_factors_1"]; k15 = r["comb_1.0W1_0.5W2_sum_factors_1.5"]
c("CE", "0.5/0.5 anchor Fy (linear = true)", m["CE 0.5/0.5 linear anchor Fy"], k5["anchor_Fy_N"]); c("CE", "0.5/0.5 true anchor Fy", m["CE 0.5/0.5 true anchor Fy"], k5["anchor_Fy_N"])
c("CE", "0.5/0.5 applied", m["CE 0.5/0.5 applied"], k5["ce_applied_N"])
c("CE", "1.0/0.5 ctrl linear anchor", m["CE 1.0/0.5 linear anchor Fy (control)"], k15["negative_control_linear_anchor_Fy_N"])
c("CE", "1.0/0.5 ctrl applied", m["CE 1.0/0.5 applied (control)"], k15["negative_control_ce_applied_N"])
c("CE", "1.0/0.5 true anchor", m["CE 1.0/0.5 true anchor Fy"], k15["true_anchor_Fy_N_if_solved"])
c("CE", "W1-W2 anchor Fy", m["CE W1-W2 anchor Fy"], r["subtraction_W1_minus_W2"]["anchor_Fy_N"])
c("CE", "W1-W2 applied", m["CE W1-W2 applied"], r["subtraction_W1_minus_W2"]["ce_applied_N"]["value"], 375)
nc = r["non_consuming_variant"]; cv("CE", "non-consuming W1 anchor", m["CE non-consuming W1 anchor"], nc["anchor_F_W1"] + nc["anchor_M_W1"], 1000)
r = A["REF-M05-SPRING2"]
cv("SPRING2", "spring", m["SPRING2 spring on pipe"], r["spring_on_pipe"], r["zero_scale_N"])
cv("SPRING2", "guide", m["SPRING2 guide on pipe"], r["guide_on_pipe"], r["zero_scale_N"])
cv("SPRING2", "anchor", m["SPRING2 anchor"], r["anchor"]["F"] + r["anchor"]["M"], r["zero_scale_N"])
r = A["REF-M05-SPRING-GAP"]
c("GAP", "tip uy", m["GAP tip uy"], r["tip_uy_m"]); c("GAP", "spring Fy", m["GAP spring on pipe Fy"], r["spring_on_pipe_Fy_N"])
c("GAP", "gap on pipe", m["GAP gap on pipe"], r["gap_on_pipe"]["value"], r["gap_on_pipe"]["zero_scale"])
c("GAP", "gap active", not m["GAP inactive (|u| < 1 mm)"], r["gap_active"])
c("GAP", "anchor Fy", m["GAP anchor Fy, Mz"][0], r["anchor_Fy_N"]); c("GAP", "anchor Mz", m["GAP anchor Fy, Mz"][1], r["anchor_Mz_Nm"])
r = A["REF-ATTR"]
cv("ATTR", "anchor", m["ATTR anchor"], r["anchor_on_pipe"]["value"], r["anchor_on_pipe"]["zero_scale"])
c("ATTR", "tip nodal total Fy", m["ATTR tip nodal total on pipe Fy"], r["tip_nodal_total_on_pipe_Fy_N"])
r = A["REF-NL-C3"]
c("NLC3", "case uy", m["NLC3 case uy"], r["case_tip_uy_m"]); c("NLC3", "case stop active", m["NLC3 case stop active"], r["case_stop_active"])
c("NLC3", "case stop force", 0.0, r["case_stop_force_N"]["value"], r["case_stop_force_N"]["zero_scale"])
c("NLC3", "superposed uy", m["NLC3 superposed uy"], r["superposed_uy_m"]); c("NLC3", "penetrates", m["NLC3 superposed penetrates"], r["superposition_penetrates_stop"])
c("NLC3", "true uy", m["NLC3 true uy"], r["true_combined_load_uy_m"]); c("NLC3", "true stop Fy", m["NLC3 true stop on pipe Fy"], r["true_combined_load_stop_on_pipe_Fy_N"])
s = r["stress_reference_C3_1d"]; su, tu, lam = m["C3 1d superposed u, true u, lambda"]
c("NLC3", "C3 1d superposed u", su, s["superposed_u"]); c("NLC3", "C3 1d true u", tu, s["true_u"]); c("NLC3", "C3 1d lambda", lam, s["true_lambda"])
r = A["REF-I-L"]; l1, l2 = r["L1_load_c_Fz_1000"], r["L2_load_c_F_1000_0_1000"]
c("I-L", "L1 ab end j", m["IL L1 ab end j Pa"], l1["pipe_ab_end_j_Pa"]["value"], l1["pipe_ab_end_j_Pa"]["zero_scale"])
c("I-L", "L1 ab end j T", m["IL L1 ab end j T"], l1["pipe_ab_end_j_torsion_Nm"]); c("I-L", "L1 bc end i", m["IL L1 bc end i Pa"], l1["pipe_bc_end_i_Pa"])
c("I-L", "L2 ab end j", m["IL L2 ab end j Pa"], l2["pipe_ab_end_j_Pa"]); c("I-L", "L2 bc end i", m["IL L2 bc end i Pa"], l2["pipe_bc_end_i_Pa"])
c("I-L", "L2 ab axial N", m["IL L2 ab axial N"], l2["pipe_ab_end_j_axial_N"])
cv("I-L", "L2 anchor", m["IL L2 anchor"], l2["anchor"]["F"] + l2["anchor"]["M"], 1000)
c("I-L", "ctrl abs-sum bc", m["IL L2 control abs-sum bc Pa"], l2["negative_control_abs_sum_bc_Pa"])
c("I-L", "ctrl with k bc", m["IL L2 control with k bc Pa"], l2["negative_control_with_k_bc_Pa"])
c("I-L", "ctrl axial ab (un-intensified)", m["IL L2 control axial un-intensified ab Pa"], l2["negative_control_axial_added_unintensified_ab_Pa"])
r = A["REF-I-T"]
c("I-T", "header ab end j", m["IT header ab end j Pa"], r["header_ab_end_j_Pa"]); c("I-T", "branch bc end i", m["IT branch bc end i Pa"], r["branch_bc_end_i_Pa"])
c("I-T", "b-e bending (info)", m["IT be bending Nm (info)"], r["unreferenced_be_bending_Nm_for_information"])
r = A["REF-B1-SIGNED"]
cv("B1S", "normal", m["B1 bend-plane normal"], r["bend_plane_normal"], 1)
for mk, ak in (("end_i_b", "end_i_b"), ("quarter_1", "quarter_1"), ("midspan", "midspan"), ("quarter_3", "quarter_3"), ("end_j_c", "end_j_c")):
    st = r["stations"][ak]
    cv("B1S", ak, m[f"B1 {mk} (N,Vy,Vz,T,My,Mz)"], [st[x] for x in ("N", "Vy", "Vz", "T", "My", "Mz")], 1200)
cf = r["chord_frame"]
cv("B1S", "chord cut b", m["B1 chord cut-face b"], cf["cut_face_b"], 1200); cv("B1S", "chord cut c", m["B1 chord cut-face c"], cf["cut_face_c"], 1200)
cv("B1S", "product end_i", m["B1 product end_i rows (node-on-element = -cut face)"], cf["product_end_i_rows_node_on_element"], 1200)
cv("B1S", "product end_j", m["B1 product end_j rows (node-on-element = +cut face)"], cf["product_end_j_rows_node_on_element"], 1200)
cv("B1S", "chord axes", m["B1 chord axes x,y,z"], cf["x"] + cf["y"] + cf["z"], 1)
es = r["endpoint_stress_rows_tangent_frame"]
c("B1S", "end_i sigma_by", m["B1 end_i bending normal y Pa (My/Z)"], es["end_i_bending_normal_y_Pa"])
c("B1S", "end_i tau_t", m["B1 end_i torsional shear Pa (T ro/J)"], es["end_i_torsional_shear_Pa"])
c("B1S", "end_j sigma_by", m["B1 end_j bending normal y Pa"], es["end_j_bending_normal_y_Pa"])
r = A["REF-B1-TANGENCY"]["cases"]
cv("B1T", "consistent centre", m["B1T consistent centre"], r["consistent"]["centre"], 1)
cv("B1T", "consistent angles", m["B1T consistent angles b,c deg"], [r["consistent"]["angle_at_b_deg"], r["consistent"]["angle_at_c_deg"]], 60)
cv("B1T", "kinked centre", m["B1T kinked (0,0,1) centre"], r["kinked_y_ref_0_0_1"]["centre"], 1)
cv("B1T", "kinked angles", m["B1T kinked (0,0,1) angles b,c deg"], [r["kinked_y_ref_0_0_1"]["angle_at_b_deg"], r["kinked_y_ref_0_0_1"]["angle_at_c_deg"]], 60)
r = A["REF-B2"]
for kf, kk in ((1, "k1"), (2, "k2"), (4, "k4")):
    e = r[kk]
    c("B2", f"{kk} roller Fx", m[f"B2 k={kf} roller on pipe Fx"], e["roller_on_pipe_Fx_N"])
    sb = e["section_at_b_tangent_frame_for_information"]
    cv("B2", f"{kk} section b (N,Vy,Mz)", m[f"B2 k={kf} section at b (N, Vy_inward, Mz)"], [sb["N"], sb["Vy_inward"], sb["Mz_in_plane"]], 1000)
    cv("B2", f"{kk} anchor", m[f"B2 k={kf} anchor on pipe"], e["anchor_on_pipe"], 1000)
c("B2", "k2 quadrature", m["B2 k=2 quadrature n=40000"], r["quadrature_check_k2_n40000"])
r = A["REF-M33-TIE"]
c("TIE", "max |u| mm", m["TIE |u| mm at p and q"][0], r["max_displacement_mm"])
print(f"{'case':8} {'quantity':34} {'checker':>24} {'author':>24} {'rel diff':>9} ok")
bad = 0
for case, q, x, y, rr in rows:
    ok = rr <= 1e-9; bad += not ok
    xs = f"{x:24.17g}" if isinstance(x, float) else f"{x:>24}"
    ys = f"{y:24.17g}" if isinstance(y, float) else f"{y:>24}"
    print(f"{case:8} {q:34} {xs} {ys} {rr:9.2e} {'yes' if ok else 'NO'}")
print("rows", len(rows), "disagreements", bad, "max rel diff among agreeing", max(r[4] for r in rows if r[4] <= 1e-9))
