#!/usr/bin/env python3
"""S0'' comparison of the checker's s0pp_derive values with the author's revision-3 changes."""
import json, sys, importlib.util
spec = importlib.util.spec_from_file_location("d", "/tmp/s0check3/s0pp_derive.py")
d = importlib.util.module_from_spec(spec); spec.loader.exec_module(d); m = d.out
A = json.load(open(sys.argv[1]))["revision_2"]
rows = []
def c(case, q, x, y, s=0.0):
    den = max(abs(x), abs(y), s); r = 0.0 if den == 0 else abs(x - y) / den
    rows.append((case, q, float(x), float(y), r))
def cv(case, q, xs, ys, s):
    for i, (x, y) in enumerate(zip(xs, ys)): c(case, f"{q}[{i}]", x, y, s)
r = A["REF-M05-COMB-2"]
c("COMB2", "A1/T envelope", m["COMB2 A1/T envelope |M|"], r["moment_envelope_A1_T"]["envelope_max_moment_magnitude_Nm"])
c("COMB2", "A1/T sum control", m["COMB2 A1/T sum control"], r["moment_envelope_A1_T"]["negative_control_sum_Nm"])
c("COMB2", "A1/A2 envelope", m["COMB2 A1/A2 envelope |M|"], r["moment_envelope_A1_A2"]["envelope_max_moment_magnitude_Nm"])
c("COMB2", "A1/A2 sum control", m["COMB2 A1/A2 sum control"], r["moment_envelope_A1_A2"]["negative_control_sum_Nm"])
l2 = A["REF-I-L"]["L2_load_c_F_1000_0_1000"]
c("I-L", "axial un-intensified", m["IL L2 ab axial un-intensified Pa"], l2["negative_control_axial_added_unintensified_ab_Pa"])
c("I-L", "axial intensified", m["IL L2 ab axial intensified Pa"], l2["negative_control_axial_added_intensified_ab_Pa"])
c("I-L", "no-k variant bc", 1.15 * 1000.0 / d.Z, A["REF-I-L"]["no_k_variant"]["pipe_bc_end_i_Pa"])
r = A["REF-I-T"]
c("I-T", "header Pa", m["IT header ab end j Pa"], r["header_ab_end_j_Pa"])
c("I-T", "branch Pa", m["IT branch bc end i Pa"], r["branch_bc_end_i_Pa"])
c("I-T", "header bending Nm", m["IT header ab end j Pa"] * d.Z / 1.3, r["header_ab_end_j_bending_Nm"])
c("I-T", "branch bending Nm", m["IT branch bc end i Pa"] * d.Z / 2.0, r["branch_bc_end_i_bending_Nm"])
c("I-T", "control b-e-equal", m["IT control: header SIF on b-e moment Pa"], r["negative_control_header_equals_unreferenced_be_Pa"])
c("I-T", "b-e bending", m["IT b-e bending Nm (unreferenced)"], r["unreferenced_be_bending_Nm_for_information"])
for kf, kk in ((1, "k1"), (2, "k2"), (4, "k4")):
    cv("B2", f"{kk} chord cut b", m[f"B2 k={kf} chord cut face b"], A["REF-B2"][kk]["chord_frame_cut_face_b"], 1000)
    cv("B2", f"{kk} end_i", m[f"B2 k={kf} product end_i (negated)"], A["REF-B2"][kk]["product_end_i_rows_node_on_element"], 1000)
c("TH", "moment zero scale", m["TH |N|*ro (candidate moment scale)"], A["REF-M14-TH"]["zero_scale_Nm"])
print(f"{'case':6} {'quantity':22} {'checker':>24} {'author':>24} {'rel diff':>9} ok")
for case, q, x, y, rr in rows:
    print(f"{case:6} {q:22} {x:24.17g} {y:24.17g} {rr:9.2e} {'yes' if rr <= 1e-9 else 'NO'}")
print("rows", len(rows), "disagreements", sum(r[4] > 1e-9 for r in rows), "max rel diff", max(r[4] for r in rows))
