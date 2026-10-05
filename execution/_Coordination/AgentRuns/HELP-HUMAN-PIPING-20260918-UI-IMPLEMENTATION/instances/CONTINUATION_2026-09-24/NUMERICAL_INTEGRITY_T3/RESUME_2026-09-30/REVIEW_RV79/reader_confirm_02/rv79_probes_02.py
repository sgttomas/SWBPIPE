"""RV79 confirmation probes on READER b36739112a (run from P on a clean archive, both BIN vars set).

Every probe from reader_review_01 (same edits) plus new probes, each with the outcome the
D1-D18 rulings require. Edits use the shared corpus grammar and the shared harness rehash
(tests/test_retained_precision_contract.apply_entry, rehash "all").
"""
import json, sys, traceback
from copy import deepcopy
sys.path.insert(0, ".")
from core.analysis_runs import retained_precision as rp
from tests.test_retained_precision_contract import apply_entry, corpus

C = corpus()
BASE = {f["id"]: f for f in C["cases"]}
B = ["retained_precision", "body"]
def S(path, value): return {"path": B + path, "op": "set", "value": value}
def RAW(path, value): return {"path": path, "op": "set", "value": value}
def body(base): return BASE[base]["source"]["retained_precision"]["body"]

def run(base, edits):
    source, invocation = apply_entry(BASE[base], {"base": base, "edits": edits, "rehash": "all"})
    try:
        rp._validate_draft(source, invocation)
        return "PASS", None
    except rp.RetainedPrecisionError as e:
        tb = [f.lineno for f in traceback.extract_tb(e.__traceback__) if f.filename.endswith("retained_precision.py")]
        return f"{e.gate} {e.code}", tb

out = []
def probe(pid, finding, base, edits, ruled, decision):
    got, lines = run(base, edits)
    out.append({"id": pid, "finding": finding, "base": base, "python": got, "raise_lines": lines, "ruled": ruled, "decision": decision, "agrees": got == ruled})

O, F2, PP2, TG, LAD, TC = ("ordinary_prepared_synthetic", "two_case_facade_after_certificate_synthetic", "two_case_preparation_failure_synthetic",
                           "two_case_two_groups_synthetic", "p512_ladder_synthetic", "two_case_synthetic")
V = "verification_failure_skip_synthetic"
ZERO = "0000000000000000"
G5A, G5W, G5P = "G5 RETAINED_PRECISION_ATTEMPT_MISMATCH", "G5 RETAINED_PRECISION_WORK_MISMATCH", "G5 RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH"
G3C = "G3 RETAINED_PRECISION_COVERAGE_MISMATCH"
fault = {"kind": "overflow", "event": "map_write"}

# ---------------- reader_review_01 probes, ruled expectations
probe("A1_area_zero", "S3", O, [S(["sources", 0, "section_terms", 0, "area"], ZERO), S(["cases", 0, "selection", "section_terms", 0, "area"], ZERO)], "G5b RETAINED_PRECISION_SECTION_MISMATCH", "D18")
probe("A2_modulus_zero", "S3", O, [S(["sources", 0, "section_terms", 0, "section_modulus"], ZERO), S(["cases", 0, "selection", "section_terms", 0, "section_modulus"], ZERO)], "G5b RETAINED_PRECISION_SECTION_MISMATCH", "D18")
probe("D1_R1_plus_report_outcome", "S2", F2, [S(["product_attempts", 1, "adapter", "fault"], fault), S(["ordinary_attempts", 0, "initial", "outcome"], "checks_passed")], G5A, "D3")
probe("D2_R1_plus_rcond_label", "S2", F2, [S(["product_attempts", 1, "adapter", "fault"], fault), S(["cases", 0, "selection", "rcond_label"], "x")], G5A, "D3")
probe("D3_R1_plus_unlisted_diagnostic", "S2", F2, [S(["product_attempts", 1, "adapter", "fault"], fault), S(["ordinary_attempts", 0, "diagnostic_refs"], ["diagnostic:numerical-integrity:case:six-component-load", "diagnostic:nope"])], G5A, "D3/D6a")
eo = deepcopy(body(TC)["work"]["execution_order"])
probe("R1a_execution_order_dropped", "S1", TC, [S(["work", "execution_order"], eo[:1])], G3C, "D1")
probe("R1b_execution_order_duplicated", "S1", TC, [S(["work", "execution_order"], [eo[0], eo[0]])], G3C, "D1")
probe("R1c_execution_order_swapped", "S1", TC, [S(["work", "execution_order"], [eo[1], eo[0]])], G3C, "D1")
old1 = body(TC)["product_attempts"][1]["operational"]["old"]
probe("R3_sourced_complete_old_long", "S1", TC, [S(["product_attempts", 1, "operational", "old"], old1 + [dict(old1[0], member=1)])], G3C, "D1")
probe("R3b_sourceless_complete_old_empty", "B2f", PP2, [S(["product_attempts", 1, "operational", "old"], [])], G3C, "D1 (CaseSource count)")
probe("R4_source_preparation_null", "B1a", F2, [S(["sources", 1, "preparation"], None)], G5P, "D4a")
probe("R4b_source_preparation_foreign", "B1a", F2, [S(["sources", 1, "preparation", "attempt_ref"], 0)], G5P, "D4a")
probe("R5_ordinary_material_basis", "B1b", TG, [S(["ordinary_attempts", 1, "material_basis_ref"], 0)], G5P, "D4b")
probe("R6c_preparation_error_with_selected_run", "B1c", F2,
      [S(["product_attempts", 1, "result", "error"], {"kind": "preparation", "capture": {"kind": "storage", "detail": "adapter vector"}, "section": None}),
       S(["cases", 1, "reason", "code"], "source_unavailable"), S(["cases", 1, "reason", "phase"], "preparation")], G5P, "D4d")
probe("R8_phantom_group", "B2a", O, [S(["groups"], deepcopy(body(O)["groups"]) + [dict(deepcopy(body(O)["groups"][0]), id=1, call=7)])], G5A, "D5e")
probe("T3_candidate_with_verification", "B2b", O, [S(["cases", 0, "run", "records", 0, "verification"], deepcopy(body(O)["cases"][0]["run"]["records"][1]["verification"]))], G5A, "D5a")
rq = deepcopy(body(LAD)["cases"][0]["run"]["records"][0]["outcome"]); rq["reason"]["quantity"]["dof"]["node"] = 99
probe("T2_stop_rule_foreign_quantity", "B2c", LAD, [S(["cases", 0, "run", "records", 0, "outcome"], rq), S(["cases", 0, "run", "attempts", 0, "outcome"], rq)], G5A, "D5d")
probe("T4a_diagnostic_ref_not_naming_case", "N4 (superseded)", O, [S(["ordinary_attempts", 0, "diagnostic_refs"], body(O)["ordinary_attempts"][0]["diagnostic_refs"] + ["diagnostic:physics:rule-inputs-missing"])], "PASS", "D6a (checkpoint A: need not name the case)")
probe("T4b_duplicate_diagnostic_ref", "N4", O, [S(["ordinary_attempts", 0, "diagnostic_refs"], body(O)["ordinary_attempts"][0]["diagnostic_refs"] * 2)], G5A, "D6a")
probe("G0_case_limit_threshold", "S4", O, [S(["work", "case_limit"], 30000000000)], "G0 SOURCE_PRODUCER_CONTRACT_UNSUPPORTED", "D2")
fb = deepcopy(body(F2)["product_attempts"][1]); st = {k: ("completed" if k == "preparation" else "not_entered") for k in fb["stages"]}
probe("P_run_ref_null_with_case_run", "B1d", F2, [S(["product_attempts", 1, "run_ref"], None), S(["product_attempts", 1, "stages"], st), S(["product_attempts", 1, "proof"], None)], G5P, "D4e")
probe("T4c_selected_with_checks_passed_report_only", "N4", O, [S(["ordinary_attempts", 0, "initial", "outcome"], "checks_passed")], G5A, "D6b/O2")
r1 = deepcopy(body(V)["cases"][0]["run"]["records"][1]["work"]); r1["own_stages"]["scale"] += 1; r1["wide_lme"] += 1; r1["own_lme"] += 1; r1["verification_lme"] += 1
RUN = ["cases", 0, "run"]
bump = lambda n: [S(RUN + ["attempts", 0, "case_charge"], 8 + n), S(RUN + ["attempts", 0, "invocation_increment"], 8 + n), S(RUN + ["case_charge"], 36 + n), S(RUN + ["invocation_increment"], 36 + n), S(RUN + ["invocation_after"], 36 + n), S(["calls", 0, "invocation_after"], 36 + n), S(["work", "charged"], 36 + n)]
probe("T1_failed_verification_pass_entered", "B2d", V, [S(RUN + ["records", 1, "work"], r1)] + bump(1), G5A, "D5b")
probe("T4c2_selected_checks_passed_quality_and_report", "N4", O, [S(["ordinary_attempts", 0, "initial", "outcome"], "checks_passed"), RAW(["numerical_quality", "cases", 0, "solve_quality"], "checks_passed")], G5A, "D6b")
probe("PR2_empty_coverage_and_empty_inventory", "N3 (superseded)", O, [S(["product_attempts", 0, "proof", "summary_coverage"], []), S(["sources", 0, "body_membership"], [])], "G5a RETAINED_PRECISION_SCALE_MISMATCH", "D1 correction (empty inventory not rejected at G3)")
diags = deepcopy(BASE[O]["source"]["diagnostics"]); sel = next(d for d in diags if d["code"] == "RETAINED_PRECISION_SELECTED")
probe("G4_orphan_selected_diagnostic", "B2e", O, [RAW(["diagnostics"], diags + [dict(deepcopy(sel), id="diagnostic:retained:orphan", affected_refs=["case:not-requested"])])], "G4 RETAINED_PRECISION_DIAGNOSTIC_MISMATCH", "D7")
vf = {"kind": "rejected", "reason": {"space": "attempt", "tag": "verification_failed"}}
probe("N13_rejected_verification_failed_with_completed_verification", "B2h", LAD, [S(["cases", 0, "run", "records", 0, "outcome"], vf), S(["cases", 0, "run", "attempts", 0, "outcome"], vf)], G5A, "D5c")

# ---------------- new probes for the confirmation
# D5b with a verification shared build (the pass's obtain_verify, adaptive.rs:4286) instead of verification_lme
b4 = deepcopy(body(V)["builds"][4])
b5 = dict(b4, id=5, slot="v256", origin={"call": 0, "run": 0, "physical_record": 1, "phase": "verification_shared"}, work=1,
          stages={k: (1 if k == "formation" else 0) for k in b4["stages"]})
w5 = deepcopy(body(V)["cases"][0]["run"]["records"][1]["work"]); w5["verification_shared_built_here"] = True; w5["verification_shared_lme"] = 1; w5["shared_stages"]["formation"] += 1
ca = deepcopy(body(V)["cases"][0]["run"]["cache_after"]); ca.insert(4, {"slot": "v256", "build": 5})
probe("D5b_vbuild_on_escalating_failed_verification", "new (D5b)", V,
      [S(["builds"], deepcopy(body(V)["builds"]) + [b5]), S(RUN + ["records", 1, "verification_shared_build_ref"], 5), S(RUN + ["records", 1, "work"], w5), S(RUN + ["cache_after"], ca)] + bump(1),
      G5A, "D5b ('a record showing the verification pass ran'); TypeScript checks the v-build (I64 06d item 1)")
probe("D5b_vbuild_control_without_failure", "control", V,
      [S(["builds"], deepcopy(body(V)["builds"]) + [b5]), S(RUN + ["records", 1, "verification_shared_build_ref"], 5), S(RUN + ["records", 1, "work"], w5), S(RUN + ["cache_after"], ca)] + bump(1)
      + [S(RUN + ["records", 1, "outcome"], {"kind": "failed", "reason": {"space": "attempt", "tag": "stop", "stop": {"space": "stop", "tag": "structure"}}})],
      G5A, "control: the same build with a non-escalating stop must still fail somewhere (schedule)")
# A selected case pointing at a foreign source (G1 identity recheck removed at G5, N7)
probe("N7_selected_source_ref_foreign", "N7", TC, [S(["cases", 0, "source_ref"], 1)], "not PASS", "N7 removal must not open a hole")
# D4d native error with a selected Run still rejected (Python's 6a reading)
probe("D4d_native_error_selected_run", "6a", F2, [S(["product_attempts", 1, "result", "error"], {"kind": "native", "run_ref": 1}),
      S(["cases", 1, "reason", "code"], "kernel_unresolved"), S(["cases", 1, "reason", "phase"], "kernel")], G5P, "D4d")
# Integral float counter (D10, G2)
probe("D10_integral_float_counter", "N5", O, [S(["work", "charged"], float(body(O)["work"]["charged"]))], "G2 RETAINED_PRECISION_ENCODING_MISMATCH", "D10 (may be masked at G1 if the checked JSON refuses floats)")
# D2 receipt_version bool and malformed producer typing
probe("D2_receipt_version_true", "S4/N2", O, [S(["receipt_version"], True)], "G0 SOURCE_PRODUCER_CONTRACT_UNSUPPORTED", "D2 (receipt_version exactly int 1)")
probe("N2_producer_list", "N2", O, [RAW(["producer"], [])], "G0 SOURCE_PRODUCER_CONTRACT_UNSUPPORTED", "D2 typed malformed producer")

for p in out:
    print(json.dumps(p))
