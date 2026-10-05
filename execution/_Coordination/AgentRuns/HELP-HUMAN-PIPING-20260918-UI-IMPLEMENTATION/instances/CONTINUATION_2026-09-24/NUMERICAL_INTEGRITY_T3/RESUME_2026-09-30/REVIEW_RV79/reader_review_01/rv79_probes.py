"""RV79 single- and dual-defect probes of the Python reader (run from P on the archived head).

Each probe edits a shared base with the corpus edit grammar, rehashes everything with the
shared harness (tests/test_retained_precision_contract.apply_entry), and records the
reader's first failure, or PASS. Expectations are RV79's contract readings (see REVIEW.md).
"""
import json, sys
from copy import deepcopy
sys.path.insert(0, ".")
from core.analysis_runs import retained_precision as rp
from tests.test_retained_precision_contract import apply_entry, corpus

C = corpus()
BASE = {f["id"]: f for f in C["cases"]}
B = ["retained_precision", "body"]

def S(path, value): return {"path": B + path, "op": "set", "value": value}

def run(base, edits, inv_edits=None, rehash="all"):
    entry = {"base": base, "edits": edits, "rehash": rehash}
    if inv_edits: entry["invocation_edits"] = inv_edits
    source, invocation = apply_entry(BASE[base], entry)
    try:
        rp._validate_draft(source, invocation)
        return "PASS"
    except rp.RetainedPrecisionError as e:
        return f"{e.gate} {e.code}"

def body(base): return BASE[base]["source"]["retained_precision"]["body"]

probes = []
def probe(pid, question, base, edits, contract, note=""):
    got = run(base, edits)
    probes.append({"id": pid, "question": question, "base": base, "python": got, "rv79_contract_reading": contract, "agrees": got == contract, "note": note})

O, F2, PP2, TG, LAD = "ordinary_prepared_synthetic", "two_case_facade_after_certificate_synthetic", "two_case_preparation_failure_synthetic", "two_case_two_groups_synthetic", "p512_ladder_synthetic"
ZERO = "0000000000000000"

# --- own hypotheses
probe("A1_area_zero", "G5b stress scale with a zero section area", O,
      [S(["sources", 0, "section_terms", 0, "area"], ZERO), S(["cases", 0, "selection", "section_terms", 0, "area"], ZERO)],
      "G5b RETAINED_PRECISION_SCALE_MISMATCH", "C1:150 G5b; Python divides 0 and labels the exception with gate G5a")
probe("A2_modulus_zero", "G5b stress scale with a zero section modulus", O,
      [S(["sources", 0, "section_terms", 0, "section_modulus"], ZERO), S(["cases", 0, "selection", "section_terms", 0, "section_modulus"], ZERO)],
      "G5b RETAINED_PRECISION_SCALE_MISMATCH", "C1:150 G5b")
# dual defects: product WORK (R1) + an ordinary/native check that Python runs after the work list
fault = {"kind": "overflow", "event": "map_write"}
probe("D1_R1_plus_report_outcome", "within-G5 class order: ordinary report outcome vs product WORK", F2,
      [S(["product_attempts", 1, "adapter", "fault"], fault), S(["ordinary_attempts", 0, "initial", "outcome"], "checks_passed")],
      "G5 RETAINED_PRECISION_ATTEMPT_MISMATCH", "C3:304 ordinary references precede work equations")
probe("D2_R1_plus_rcond_label", "within-G5 class order: native selected summary vs product WORK", F2,
      [S(["product_attempts", 1, "adapter", "fault"], fault), S(["cases", 0, "selection", "rcond_label"], "x")],
      "G5 RETAINED_PRECISION_ATTEMPT_MISMATCH", "C1:148 selected summaries are native-class; C3:304 native first")
probe("D3_R1_plus_unlisted_diagnostic", "within-G5 class order: ordinary diagnostic_refs vs product WORK", F2,
      [S(["product_attempts", 1, "adapter", "fault"], fault), S(["ordinary_attempts", 0, "diagnostic_refs"], ["diagnostic:numerical-integrity:case:six-component-load", "diagnostic:nope"])],
      "G5 RETAINED_PRECISION_ATTEMPT_MISMATCH", "C3:304")
# --- Rust known differences (I63)
eo = deepcopy(body("two_case_synthetic")["work"]["execution_order"])
probe("R1a_execution_order_dropped", "Rust 1: execution-order bijection", "two_case_synthetic",
      [S(["work", "execution_order"], eo[:1])], "G3 RETAINED_PRECISION_COVERAGE_MISMATCH", "C1:146 G3 'execution-order bijection'; C2:117")
probe("R1b_execution_order_duplicated", "Rust 1: execution-order bijection", "two_case_synthetic",
      [S(["work", "execution_order"], [eo[0], eo[0]])], "G3 RETAINED_PRECISION_COVERAGE_MISMATCH", "C1:146")
probe("R1c_execution_order_swapped", "Rust 1: execution-order actual order", "two_case_synthetic",
      [S(["work", "execution_order"], [eo[1], eo[0]])], "G3 RETAINED_PRECISION_COVERAGE_MISMATCH", "C1:99/146, C2:117 run.id equals execution-order position")
old = deepcopy(body(F2)["product_attempts"][1]["operational"]["old"])
probe("R3_sourced_complete_old_long", "Rust 3: complete old coverage longer than the source member inventory", "two_case_synthetic",
      [S(["product_attempts", 1, "operational", "old"], body("two_case_synthetic")["product_attempts"][1]["operational"]["old"] + [dict(body("two_case_synthetic")["product_attempts"][1]["operational"]["old"][0], member=1)])],
      "G3 RETAINED_PRECISION_COVERAGE_MISMATCH", "F1:118,130; C3:302 member-prefix coverage at G3")
probe("R3b_sourceless_complete_old_empty", "Rust 3 (shared gap): sourceless complete old coverage truncated", PP2,
      [S(["product_attempts", 1, "operational", "old"], [])], "G3 RETAINED_PRECISION_COVERAGE_MISMATCH", "F1:118 complete = exactly M; F1:130 G3")
probe("R4_source_preparation_null", "Rust 4: source preparation back-reference on a sourced unavailable attempt", F2,
      [S(["sources", 1, "preparation"], None)], "G5 RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH", "C3:145-148")
probe("R4b_source_preparation_foreign", "Rust 4: source preparation names another attempt", F2,
      [S(["sources", 1, "preparation", "attempt_ref"], 0)], "G5 RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH", "C3:145-148 (G1 hash skip, G5 association)")
probe("R5_ordinary_material_basis", "Rust 5: product attempt material basis vs its ordinary attempt", TG,
      [S(["ordinary_attempts", 1, "material_basis_ref"], 0)], "G5 RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH", "C3:165-166; C2:149")
probe("R6c_preparation_error_with_selected_run", "Rust 6: preparation error requires no Run and a failed preparation stage", F2,
      [S(["product_attempts", 1, "result", "error"], {"kind": "preparation", "capture": {"kind": "storage", "detail": "adapter vector"}, "section": None}),
       S(["cases", 1, "reason", "code"], "source_unavailable"), S(["cases", 1, "reason", "phase"], "preparation")],
      "G5 RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH", "S06:33-34 'preparation stage failed; no native Run'")
probe("R8_phantom_group", "Rust 8: a group whose call does not exist", O,
      [S(["groups"], deepcopy(body(O)["groups"]) + [dict(deepcopy(body(O)["groups"][0]), id=1, call=7)])],
      "G5 RETAINED_PRECISION_ATTEMPT_MISMATCH", "C2:117,143 call-local groups; ids assigned in actual execution order")
# --- TypeScript known differences (I64)
probe("T3_candidate_with_verification", "TS 3: candidate-role record carrying a verification summary", O,
      [S(["cases", 0, "run", "records", 0, "verification"], deepcopy(body(O)["cases"][0]["run"]["records"][1]["verification"]))],
      "G5 RETAINED_PRECISION_ATTEMPT_MISMATCH", "C1:28 the pass runs on Verification records only; C1:105 snapshots")
rq = deepcopy(body(LAD)["cases"][0]["run"]["records"][0]["outcome"]); rq["reason"]["quantity"]["dof"]["node"] = 99
probe("T2_stop_rule_foreign_quantity", "TS 2: stop-rule reason quantity absent from the layout", LAD,
      [S(["cases", 0, "run", "records", 0, "outcome"], rq), S(["cases", 0, "run", "attempts", 0, "outcome"], rq)],
      "G5 RETAINED_PRECISION_ATTEMPT_MISMATCH", "C1:114 exact quantity/body/kind payload")
probe("T4a_diagnostic_ref_not_naming_case", "TS 4: listed diagnostic ref that does not name the case", O,
      [S(["ordinary_attempts", 0, "diagnostic_refs"], body(O)["ordinary_attempts"][0]["diagnostic_refs"] + ["diagnostic:physics:rule-inputs-missing"])],
      "G5 RETAINED_PRECISION_ATTEMPT_MISMATCH", "C2:161-163 exact diagnostic refs (RV79 reading; ROOT 06b O2 settled only typed refs)")
probe("T4b_duplicate_diagnostic_ref", "TS 4: duplicate diagnostic_refs entry", O,
      [S(["ordinary_attempts", 0, "diagnostic_refs"], body(O)["ordinary_attempts"][0]["diagnostic_refs"] * 2)],
      "G5 RETAINED_PRECISION_ATTEMPT_MISMATCH", "C2:149 exact refs (RV79 reading)")
for p in probes:
    print(json.dumps(p))

# --- second batch (added after the first run)
probes.clear()
probe("G0_case_limit_threshold", "C1 G0 'all registered policy ids/thresholds'", O,
      [S(["work", "case_limit"], 30000000000)], "G0 SOURCE_PRODUCER_CONTRACT_UNSUPPORTED", "C1:143 G0; schema const makes Python fail at G1")
fb = deepcopy(body(F2)["product_attempts"][1])
st = {k: ("completed" if k == "preparation" else "not_entered") for k in fb["stages"]}
probe("P_run_ref_null_with_case_run", "C3:167 run_ref null iff no native call; S06 same-Run references", F2,
      [S(["product_attempts", 1, "run_ref"], None), S(["product_attempts", 1, "stages"], st), S(["product_attempts", 1, "proof"], None)],
      "G5 RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH", "C3:165-169; S06 'Every Run reference must resolve to the same actual C2 Run'")
probe("T4c_selected_with_checks_passed_quality", "TS 4: selected case whose ordinary quality is checks_passed", O,
      [S(["ordinary_attempts", 0, "initial", "outcome"], "checks_passed")] ,
      "G5 RETAINED_PRECISION_ATTEMPT_MISMATCH", "C2:151-156; C1:101 (report outcome must equal solve_quality)")
for p in probes:
    print(json.dumps(p))

# --- third batch: TS item 1 (failed verification whose record shows the pass was entered)
probes.clear()
V = "verification_failure_skip_synthetic"
r1 = deepcopy(body(V)["cases"][0]["run"]["records"][1]["work"])
r1["own_stages"]["scale"] += 1; r1["wide_lme"] += 1; r1["own_lme"] += 1; r1["verification_lme"] += 1
RUN = ["cases", 0, "run"]
probe("T1_failed_verification_pass_entered", "TS 1: escalating failed verification whose record shows verification-pass work", V,
      [S(RUN + ["records", 1, "work"], r1), S(RUN + ["attempts", 0, "case_charge"], 9), S(RUN + ["attempts", 0, "invocation_increment"], 9),
       S(RUN + ["case_charge"], 37), S(RUN + ["invocation_increment"], 37), S(RUN + ["invocation_after"], 37),
       S(["calls", 0, "invocation_after"], 37), S(["work", "charged"], 37)],
      "G5 RETAINED_PRECISION_ATTEMPT_MISMATCH", "C1:27,31; adaptive.rs:4593-4611 a pass failure is terminal and never escalates")
probes.append({"id": "T4c2_selected_checks_passed_quality_and_report", "python": run(O, [S(["ordinary_attempts", 0, "initial", "outcome"], "checks_passed"),
               {"path": ["numerical_quality", "cases", 0, "solve_quality"], "op": "set", "value": "checks_passed"}]),
               "rv79_contract_reading": "G5 RETAINED_PRECISION_ATTEMPT_MISMATCH (or an earlier base-contract failure)", "agrees": None,
               "note": "C1:101 not_required is the ordinary-pass branch; a selected case with checks_passed quality is not emitted"})
for p in probes:
    print(json.dumps(p))

# --- fourth batch: parity rule 2 with an emptied inventory
probes.clear()
probe("PR2_empty_coverage_and_empty_inventory", "parity rule 2: non-null empty coverage roster", O,
      [S(["product_attempts", 0, "proof", "summary_coverage"], []), S(["sources", 0, "body_membership"], [])],
      "G3 RETAINED_PRECISION_COVERAGE_MISMATCH", "I57 s1 'no empty complete vector'; ROOT parity rule 2")
for p in probes:
    print(json.dumps(p))

# --- fifth batch: G4 orphan retained diagnostic
probes.clear()
diags = deepcopy(BASE[O]["source"]["diagnostics"])
sel = next(d for d in diags if d["code"] == "RETAINED_PRECISION_SELECTED")
orphan = dict(deepcopy(sel), id="diagnostic:retained:orphan", affected_refs=["case:not-requested"])
probes.append({"id": "G4_orphan_selected_diagnostic", "question": "C1:147 G4 exactly one selected/unavailable diagnostic per corresponding case",
               "base": O, "python": run(O, [{"path": ["diagnostics"], "op": "set", "value": diags + [orphan]}]),
               "rv79_contract_reading": "G4 RETAINED_PRECISION_DIAGNOSTIC_MISMATCH (or the base contract's own reference failure at G7)", "agrees": None,
               "note": "an extra retained diagnostic naming no requested case"})
for p in probes:
    print(json.dumps(p))

# --- sixth batch: N13 reason domain for a rejected candidate whose verification completed
probes.clear()
vf = {"kind": "rejected", "reason": {"space": "attempt", "tag": "verification_failed"}}
probe("N13_rejected_verification_failed_with_completed_verification", "N13: rejected(verification_failed) although its verification completed", LAD,
      [S(["cases", 0, "run", "records", 0, "outcome"], vf), S(["cases", 0, "run", "attempts", 0, "outcome"], vf)],
      "G5 RETAINED_PRECISION_ATTEMPT_MISMATCH", "C1:27 VerificationFailed only for a failed verification; adaptive.rs:4576-4590")
for p in probes:
    print(json.dumps(p))
