"""RV113 (RV-R): reviewer probes for B1 SR-RS, written from DESIGN_v2 §2 and §3.2-§3.3.

Each probe is a shared-format entry (base, edits, invocation_edits, rehash "all") with
the verdict this reviewer expects at the head (`want`) and at I1 (`want_i1`). Verdicts:
{"admitted": eligible} or {"gate": g, "code": c}. The Rust harness materializes and
validates them; compare_probes.py compares.

Usage: python gen_probes.py <corpus json> <out json>
"""
import copy
import json
import sys

corpus_path, out_path = sys.argv[1], sys.argv[2]
d = json.load(open(corpus_path))
C = {c["id"]: c for c in d["cases"]}
MP = {m["id"]: m for m in d["must_pass"]}

PRODUCT = "RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH"
ATTEMPT = "RETAINED_PRECISION_ATTEMPT_MISMATCH"
COVERAGE = "RETAINED_PRECISION_COVERAGE_MISMATCH"
WORK = "RETAINED_PRECISION_WORK_MISMATCH"
PREP = "RETAINED_PRECISION_PREPARATION_MISMATCH"
DIAG = "RETAINED_PRECISION_DIAGNOSTIC_MISMATCH"
ROWM = "RETAINED_PRECISION_ROW_METHOD_MISMATCH"


def G(gate, code):
    return {"gate": gate, "code": code}


def ADM(eligible):
    return {"admitted": eligible}


def rb(*p):
    return ["retained_precision", "body", *p]


def S(path, value):
    return {"path": list(path), "op": "set", "value": value}


def RM(path):
    return {"path": list(path), "op": "remove"}


probes = []


def probe(pid, base, edits, want, want_i1=None, inv=None, item="", note=""):
    probes.append({
        "id": pid, "item": item, "base": base, "edits": edits,
        "invocation_edits": inv or [], "rehash": "all",
        "want": want, "want_i1": want if want_i1 is None else want_i1, "note": note,
    })


STAGES = ["preparation", "native", "proof_start", "projection", "maxima", "values",
          "aliases", "certificate", "observables", "g5a"]

# ---------------------------------------------------------------------------------
# Item 2: R-D38 (4b), my own derivation on two_case_two_groups_synthetic. Case 1 is
# rewritten into (4b)'s shape as DESIGN §2's corpus pin derives d38_beside_selected:
# remove its Run and execution_order entry; remove the Builds whose origin is its Run
# (3..5) and its own group (group 1, which only its Run used); remove its source from
# the call's source_refs; keep case 0's group and builds; recompute charged and the
# call's after-value; a typed CaptureError::Origin cause. Its selected-case artefacts
# become an unavailable case's: the RETAINED_PRECISION_UNAVAILABLE diagnostic, rows
# without recovery_method, no selection, method or source identity.
G2 = "two_case_two_groups_synthetic"
gs = C[G2]["source"]
gb = gs["retained_precision"]["body"]
c1 = gb["cases"][1]
assert c1["status"] == "selected" and gb["cases"][0]["status"] == "selected"
assert [b["origin"]["run"] for b in gb["builds"]] == [0, 0, 0, 1, 1, 1]
assert [g["source_refs"] for g in gb["groups"]] == [[0], [1]]
d38_case = {
    "basis_ref": c1["basis_ref"], "ordinary": c1["ordinary"], "product_attempt_ref": 1,
    "status": "unavailable",
    "reason": {"code": "source_unavailable", "phase": "preparation",
               "cause": {"kind": "prepared_product_failure", "product_attempt_ref": 1}},
    "diagnostic_ref": "diagnostic:retained:synthetic-stiff", "run": None, "source_ref": 1,
}
a1 = copy.deepcopy(gb["product_attempts"][1])
a1["run_ref"] = None
a1["proof"] = None
a1["stages"] = {k: "not_entered" for k in STAGES}
a1["stages"]["preparation"] = "completed"
a1["stages"]["native"] = "failed"
a1["result"] = {"kind": "unavailable", "error": {"kind": "capture", "cause": {"kind": "origin", "cause": {"kind": "capacity"}}}}
F = C["two_case_facade_after_certificate_synthetic"]["source"]["retained_precision"]["body"]
a1["adapter"] = copy.deepcopy(F["product_attempts"][1]["adapter"])
assert a1["adapter"]["fault"] is None
a1["overlay_work"] = {"entered": 0, "checks": 0, "lost": False}
a1["g5a_work"] = {"entered": 0, "checks": 0, "lost": False}
after0 = gb["cases"][0]["run"]["invocation_after"]
call0 = copy.deepcopy(gb["calls"][0])
call0["owner_refs"] = [{"kind": "case", "index": 0}]
call0["source_refs"] = [0]
call0["run_refs"] = [0]
call0["invocation_after"] = after0
assert gs["diagnostics"][16]["id"] == "diagnostic:retained:synthetic-stiff"
rows = copy.deepcopy(gs["results"])
for r in rows:
    if r["basis_ref"]["ref_id"] == c1["basis_ref"]["ref_id"]:
        r.pop("recovery_method", None)
D38 = [
    S(rb("cases", 1), d38_case),
    S(rb("product_attempts", 1), a1),
    S(rb("calls", 0), call0),
    S(rb("groups"), [gb["groups"][0]]),
    S(rb("builds"), gb["builds"][:3]),
    S(rb("work", "charged"), after0),
    S(rb("work", "execution_order"), [{"kind": "case", "index": 0}]),
    S(["diagnostics", 16, "code"], "RETAINED_PRECISION_UNAVAILABLE"),
    S(["results"], rows),
]
I1_D38 = G("G5", PRODUCT)  # before B1: an entered native stage needs a Run


def d38(pid, extra, want, want_i1=None, note=""):
    probe(pid, G2, D38 + extra, want, I1_D38 if want_i1 is None else want_i1, item="2 (4b)", note=note)


def at1(*p):
    return rb("product_attempts", 1, *p)


d38("d38_base", [], ADM(False), note="(4b) beside a selected case: admitted, not eligible")
d38("m1_error_kind_native", [S(at1("result", "error"), {"kind": "native", "run_ref": 1})], G("G5", PRODUCT))
d38("m2_native_completed", [S(at1("stages", "native"), "completed")], G("G5", PRODUCT), G("G5", PRODUCT))
d38("m3_run_ref_without_run", [S(at1("run_ref"), 1)], G("G5", PRODUCT), G("G5", PRODUCT))
d38("m4_execution_order_lists_case", [S(rb("work", "execution_order"), [{"kind": "case", "index": 0}, {"kind": "case", "index": 1}])], G("G3", COVERAGE), G("G3", COVERAGE))
d38("m5_proof_start_completed", [S(at1("stages", "proof_start"), "completed")], G("G5", PRODUCT))
d38("m6_attempt_source_null", [S(at1("source_ref"), None)], G("G5", PRODUCT))
d38("m7_call_source_refs_lists_source", [S(rb("calls", 0, "source_refs"), [0, 1])], G("G5", ATTEMPT), G("G5", ATTEMPT))
d38("m7_group0_lists_source", [S(rb("groups", 0, "source_refs"), [0, 1])], G("G5", ATTEMPT), G("G5", ATTEMPT))
d38("m7_own_group_kept", [S(rb("groups"), gb["groups"])], G("G5", ATTEMPT), G("G5", ATTEMPT))
d38("m7_call_position_without_run", [S(rb("calls", 0), gb["calls"][0] | {"invocation_after": after0})], G("G5", ATTEMPT), G("G5", ATTEMPT))
d38("m8_case_source_other", [S(rb("cases", 1, "source_ref"), 0)], G("G5", PRODUCT))
d38("m8_case_source_null", [S(rb("cases", 1, "source_ref"), None)], G("G5", PRODUCT))
d38("x_builds_of_removed_run_kept", [S(rb("builds"), gb["builds"])], G("G5", WORK), G("G5", WORK))
d38("x_charged_not_recomputed", [S(rb("work", "charged"), gb["work"]["charged"])], G("G5", WORK), G("G5", WORK))
d38("x_call_after_not_recomputed", [S(rb("calls", 0, "invocation_after"), gb["calls"][0]["invocation_after"])], G("G5", WORK), G("G5", WORK))
d38("x_case_run_kept", [S(rb("cases", 1, "run"), c1["run"])], G("G3", COVERAGE), G("G3", COVERAGE))
d38("x_cause_capture_accounting", [S(at1("result", "error", "cause"), {"kind": "accounting", "event": "source_visit"})], G("G5", WORK), note="R1': a CaptureError accounting{event} is WORK for any attempt")
d38("x_cause_native_unavailable", [S(at1("result", "error", "cause"), {"kind": "native_unavailable"})], ADM(False), note="(4b) carries the actual capture cause")
d38("x_preparation_failed", [S(at1("stages", "preparation"), "failed")], G("G5", PRODUCT), G("G5", PRODUCT))
d38("x_test_hook_shape_no_source", [S(at1("source_ref"), None), S(rb("cases", 1, "source_ref"), None), S(rb("sources"), [gb["sources"][0]])], G("G5", PRODUCT), note="the D1 test-hook D38 shape stays refused (DESIGN §2)")
d38("x_cause_names_attempt0", [S(rb("cases", 1, "reason", "cause", "product_attempt_ref"), 0)], G("G5", PRODUCT), G("G5", PRODUCT))
d38("x_reason_code_kernel_unresolved", [S(rb("cases", 1, "reason", "code"), "kernel_unresolved")], G("G5", PRODUCT))
d38("x_reason_phase_kernel", [S(rb("cases", 1, "reason", "phase"), "kernel")], G("G5", PRODUCT))
d38("x_reason_cause_receipt_failure", [S(rb("cases", 1, "reason", "cause"), {"kind": "receipt_failure", "check": "association", "field_path": "rv113"})], G("G5", PRODUCT), G("G5", PRODUCT))
d38("x_proof_kept", [S(at1("proof"), gb["product_attempts"][1]["proof"])], G("G5", PRODUCT), G("G5", PRODUCT))
d38("x_observables_g5a_entered", [S(at1("stages", "observables"), "failed"), S(at1("stages", "g5a"), "failed")], G("G5", PRODUCT), G("G5", PRODUCT))
d38("x_values_entered", [S(at1("stages", "values"), "failed")], G("G5", PRODUCT), G("G5", PRODUCT))
d38("x_result_ready", [S(at1("result"), {"kind": "ready"})], G("G5", PRODUCT), G("G5", PRODUCT))
d38("x_attempt_source_of_case0", [S(at1("source_ref"), 0), S(rb("cases", 1, "source_ref"), 0)], G("G5", PRODUCT), G("G5", PRODUCT))
d38("x_selected_diagnostic_kept", [S(["diagnostics", 16, "code"], "RETAINED_PRECISION_SELECTED")], G("G4", DIAG), G("G4", DIAG))
d38("x_recovery_method_kept", [S(["results"], gs["results"])], G("G6", ROWM), G("G5", PRODUCT))
d38("x_native_not_entered_no_run", [S(at1("stages", "native"), "not_entered"), S(at1("result", "error", "kind"), "capture")], G("G5", PRODUCT), G("G5", PRODUCT), note="capture with native not_entered and preparation completed: error_stages refuses")

# The same (4b) on F_BASE's case 1 (its Run reused case 0's builds), derived here
# independently of I90's helper, with mode-code and parity probes on the unavailable case.
FB = "two_case_facade_after_certificate_synthetic"
fs = C[FB]["source"]
fb = fs["retained_precision"]["body"]
assert all(b["origin"]["run"] == 0 for b in fb["builds"])
fa1 = copy.deepcopy(fb["product_attempts"][1])
fa1["run_ref"] = None
fa1["proof"] = None
fa1["stages"] = {k: "not_entered" for k in STAGES}
fa1["stages"]["preparation"] = "completed"
fa1["stages"]["native"] = "failed"
fa1["result"] = {"kind": "unavailable", "error": {"kind": "capture", "cause": {"kind": "origin", "cause": {"kind": "allocation"}}}}
fcall = copy.deepcopy(fb["calls"][0])
fafter = fb["cases"][0]["run"]["invocation_after"]
fcall.update({"owner_refs": [{"kind": "case", "index": 0}], "source_refs": [0], "run_refs": [0], "invocation_after": fafter})
fgroups = copy.deepcopy(fb["groups"])
assert len(fgroups) == 1
fgroups[0]["source_refs"] = [0]
F38 = [
    S(rb("cases", 1, "run"), None),
    S(rb("cases", 1, "reason"), {"code": "source_unavailable", "phase": "preparation", "cause": {"kind": "prepared_product_failure", "product_attempt_ref": 1}}),
    S(rb("product_attempts", 1), fa1),
    S(rb("calls", 0), fcall),
    S(rb("groups"), fgroups),
    S(rb("work", "charged"), fafter),
    S(rb("work", "execution_order"), [{"kind": "case", "index": 0}]),
]
probe("f38_base", FB, F38, ADM(False), I1_D38, item="2 (4b)")
d38("m7_call_position0_names_source1", [S(rb("calls", 0, "source_refs"), [1])], G("G5", ATTEMPT), G("G5", ATTEMPT), note="the call's position names the (4b) source for case 0's Run")

# (4a), unchanged by B1: native failed with the case's own non-selected Run. No shared base
# has one (the contract test's D30 note), so this is a positive witness at validate: F_BASE's
# case 1 Run made idle in its ready group (CasePrep refusal: ledger_unavailable), its attempt
# native-failed with a native error naming that Run, and the case kernel_refused.
fr1 = copy.deepcopy(fb["cases"][1]["run"])
fr1.update({"records": [], "attempts": [], "case_charge": 0, "invocation_increment": 0,
            "invocation_after": fr1["invocation_before"], "cache_after": fr1["cache_before"],
            "kernel_terminal": {"kind": "refused", "reason": {"space": "refusal", "tag": "ledger_unavailable", "error": {"tag": "accumulator", "error": {"tag": "non_finite"}}}}})
fa4 = copy.deepcopy(fa1)
fa4["run_ref"] = 1
fa4["result"] = {"kind": "unavailable", "error": {"kind": "native", "run_ref": 1}}
F4A = [
    S(rb("cases", 1, "run"), fr1),
    S(rb("cases", 1, "reason"), {"code": "kernel_refused", "phase": "kernel", "cause": {"kind": "prepared_product_failure", "product_attempt_ref": 1}}),
    S(rb("product_attempts", 1), fa4),
    S(rb("calls", 0, "invocation_after"), fr1["invocation_before"]),
    S(rb("work", "charged"), fr1["invocation_before"]),
]
probe("a4_native_failed_with_refused_run", FB, F4A, ADM(False), ADM(False), item="2 (4a)", note="(4a) admitted before and after B1")
probe("a4_capture_with_refused_run", FB, F4A + [S(rb("product_attempts", 1, "result", "error"), {"kind": "capture", "cause": {"kind": "origin", "cause": {"kind": "capacity"}}})], ADM(False), ADM(False), item="2 (4a)", note="(4a) with a capture error and a Run: reason_table's capture-with-Run arm")
probe("a4_reason_as_4b", FB, F4A + [S(rb("cases", 1, "reason", "code"), "source_unavailable"), S(rb("cases", 1, "reason", "phase"), "preparation")], G("G5", PRODUCT), G("G5", PRODUCT), item="2 (4a)")

# ---------------------------------------------------------------------------------
# Item 3: G8 per case (text B, P1-P4) on non-selected cases, sparse.
UR = "case:unavailable-row"


def row_index(src, kind, case):
    return [i for i, r in enumerate(src["results"]) if r["kind"] == kind and r["basis_ref"]["ref_id"] == case][0]


def remove_case_row(src, kind, case):
    """Remove the case's row of `kind` and shift its attempt's projection row indices
    (a row index counts the case's own rows), as a faithful delete-and-reseal does."""
    gi = row_index(src, kind, case)
    rows = [i for i, r in enumerate(src["results"]) if r["basis_ref"]["ref_id"] == case]
    local = rows.index(gi)
    body = src["retained_precision"]["body"]
    ci = [i for i, c in enumerate(body["cases"]) if c["basis_ref"]["ref_id"] == case][0]
    edits = [RM(["results", gi])]
    ai = body["cases"][ci]["product_attempt_ref"]
    if ai is not None and body["product_attempts"][ai]["proof"] is not None:
        po = copy.deepcopy(body["product_attempts"][ai]["proof"]["projection_outcomes"])
        for o in po:
            assert o["row_index"] != local
            if o["row_index"] > local:
                o["row_index"] -= 1
        edits.append(S(rb("product_attempts", ai, "proof", "projection_outcomes"), po))
    return edits


DENSE = C["ordinary_prepared_dense_synthetic"]["source"]
PARITY = DENSE["results"][1]
assert PARITY["kind"] == "sparse_live_path_dense_parity_relative_delta" and "recovery_method" in PARITY


def parity_for(case, pid, method=False):
    r = copy.deepcopy(PARITY)
    r["id"] = pid
    r["basis_ref"] = {"ref_type": "load_case", "ref_id": case}
    if not method:
        r.pop("recovery_method", None)
    return r


fm = row_index(fs, "linear_solver_mode_basis", UR)
probe("g8_unavailable_mode_code_2_in_sparse", FB, [S(["results", fm, "value"], 2.0)], G("G8", PREP), ADM(False), item="3 G8")
probe("g8_unavailable_mode_code_3", FB, [S(["results", fm, "value"], 3.0)], G("G8", PREP), ADM(False), item="3 G8")
probe("g8_unavailable_mode_row_duplicated", FB, [S(["results"], fs["results"] + [dict(fs["results"][fm], id="result:rv113:mode-dup")])], G("G8", PREP), ADM(False), item="3 G8")
probe("g8_unavailable_mode_row_removed", FB, remove_case_row(fs, "linear_solver_mode_basis", UR), G("G8", PREP), ADM(False), item="3 G8", note="the attempt's projection row indices shifted with the row")
probe("g8_unavailable_parity_in_sparse_P3", FB, [S(["results"], fs["results"] + [parity_for(UR, "result:rv113:parity-u")])], G("G8", PREP), ADM(False), item="3 G8")
probe("g8_unavailable_parity_with_method_G6_first", FB, [S(["results"], fs["results"] + [parity_for(UR, "result:rv113:parity-um", True)])], G("G6", ROWM), G("G6", ROWM), item="3 G8")
probe("g8_unavailable_requested_mode_flipped", FB, [S(rb("ordinary_attempts", 1, "requested_mode"), "dense_scrutiny")], G("G8", PREP), G("G8", PREP), item="3 G8")
probe("g8_selected_mode_code_2_in_sparse", FB, [S(["results", row_index(fs, "linear_solver_mode_basis", "case:six-component-load"), "value"], 2.0)], G("G8", PREP), G("G8", PREP), item="3 G8")
# (4b) case: the G8 loop reaches it too (G5 admits it first at the head).
d38rows = rows
dm = row_index(gs, "linear_solver_mode_basis", c1["basis_ref"]["ref_id"])
d38("g8_d38_case_mode_code_3", [S(["results", dm, "value"], 3.0)], G("G8", PREP), note="G8's loop reaches a (4b) unavailable case")
d38("g8_d38_case_parity_in_sparse_P3", [S(["results"], d38rows + [parity_for(c1["basis_ref"]["ref_id"], "result:rv113:parity-d38")])], G("G8", PREP))

# Dense two-case statements, both selected (two_case_two_groups), and the F_BASE pair.
def to_dense(src, cases):
    edits = [S(rb("ordinary_attempts", i, "requested_mode"), "dense_scrutiny") for i in range(len(cases))]
    rows = copy.deepcopy(src["results"])
    for case in cases:
        rows[row_index(src, "linear_solver_mode_basis", case)]["value"] = 2.0
    return edits, rows


INV_DENSE = [S(["solver_mode"], "dense_scrutiny")]
A0, A1 = "case:six-component-load", "case:stiff-basis"
dz_edits, dz_rows = to_dense(gs, [A0, A1])


def dense(pid, extra_rows, extra, want, want_i1, note=""):
    probe(pid, G2, dz_edits + [S(["results"], dz_rows + extra_rows)] + extra, want, want_i1, inv=INV_DENSE, item="3 G8", note=note)


def w2_published(i, src):
    report = src["retained_precision"]["body"]["ordinary_attempts"][i]["initial"]["report_diagnostic_ref"]
    return [
        S(rb("ordinary_attempts", i, "initial"), {"kind": "structural_failure", "error": {"tag": "range", "detail": "rv113"}, "diagnostic_ref": None}),
        S(rb("ordinary_attempts", i, "w2"), {"kind": "published", "trigger": {"tag": "evaluation", "error": {"tag": "range", "detail": "rv113"}}, "force_scale_exponent": 4, "report_diagnostic_ref": report}),
    ]


def w2_failed(i, src):
    report = src["retained_precision"]["body"]["ordinary_attempts"][i]["initial"]["report_diagnostic_ref"]
    return [
        S(rb("ordinary_attempts", i, "initial"), {"kind": "structural_failure", "error": {"tag": "range", "detail": "rv113"}, "diagnostic_ref": None}),
        S(rb("ordinary_attempts", i, "w2"), {"kind": "failed", "trigger": {"tag": "evaluation", "error": {"tag": "range", "detail": "rv113"}}, "failure": {"tag": "not_engaged"}, "diagnostic_ref": report}),
    ]


P0 = parity_for(A0, "result:rv113:parity-a0", True)
P1 = parity_for(A1, "result:rv113:parity-a1", True)
P1b = parity_for(A1, "result:rv113:parity-a1b", True)
dense("dz_no_parity_b0_admitted", [], [], ADM(True), G("G8", PREP), note="text B: absent parity at b = 0 admitted (the disclosed limit's shape)")
dense("dz_parity_case0_only", [P0], [], ADM(True), G("G8", PREP))
dense("dz_parity_both", [P0, P1], [], ADM(True), ADM(True))
dense("dz_parity_twice_case1_P2", [P0, P1, P1b], [], G("G8", PREP), G("G8", PREP))
dense("dz_parity_case1_w2_published_P4", [P0, P1], w2_published(1, gs), G("G8", PREP), ADM(True))
dense("dz_parity_case0_w2_published_P4", [P0, P1], w2_published(0, gs), G("G8", PREP), ADM(True), note="P4 on case 0 (loop order)")
dense("dz_no_parity_case1_w2_published", [P0], w2_published(1, gs), ADM(True), G("G8", PREP), note="P5 deferred: the published case's mode row still states observed fields")
dense("dz_parity_case1_w2_failed", [P0, P1], w2_failed(1, gs), ADM(True), ADM(True), note="P4 is exactly w2 published")
# Dense, F_BASE pair (case 1 unavailable with a Run): a parity row on the unavailable case.
fz_edits, fz_rows = to_dense(fs, ["case:six-component-load", UR])
PF0 = parity_for("case:six-component-load", "result:rv113:parity-f0", True)
PU = parity_for(UR, "result:rv113:parity-fu")
PUb = parity_for(UR, "result:rv113:parity-fub")
probe("fz_unavailable_one_parity_b0", FB, fz_edits + [S(["results"], fz_rows + [PF0, PU])], ADM(False), ADM(False), inv=INV_DENSE, item="3 G8")
probe("fz_unavailable_two_parity_P2", FB, fz_edits + [S(["results"], fz_rows + [PF0, PU, PUb])], G("G8", PREP), ADM(False), inv=INV_DENSE, item="3 G8")
probe("fz_unavailable_parity_w2_published_P4", FB, fz_edits + [S(["results"], fz_rows + [PF0, PU])] + w2_published(1, fs), G("G8", PREP), ADM(False), inv=INV_DENSE, item="3 G8")
probe("fz_unavailable_no_parity", FB, fz_edits + [S(["results"], fz_rows + [PF0])], ADM(False), ADM(False), inv=INV_DENSE, item="3 G8")
# The disclosed limit on the producer-solved dense L = 0 base and the dense synthetic base.
L0 = C["u8_l0_isolated_node_dense_scrutiny"]["source"]
probe("limit_l0_dense_parity_deleted", "u8_l0_isolated_node_dense_scrutiny", remove_case_row(L0, "sparse_live_path_dense_parity_relative_delta", "case"), ADM(True), G("G8", PREP), item="3 limit", note="F-1 text B's disclosed limit: the deletion, resealed with its row indices, is not detected")
probe("limit_l0_dense_parity_deleted_indices_unshifted", "u8_l0_isolated_node_dense_scrutiny", [RM(["results", row_index(L0, "sparse_live_path_dense_parity_relative_delta", "case")])], G("G3", COVERAGE), G("G3", COVERAGE), item="3 limit", note="a bare deletion is caught by G3's projection row indices")
probe("limit_dense_synthetic_parity_deleted", "ordinary_prepared_dense_synthetic", remove_case_row(DENSE, "sparse_live_path_dense_parity_relative_delta", "case:six-component-load"), ADM(True), G("G8", PREP), item="3 limit")
L0S = C["u8_l0_isolated_node_sparse_interactive"]["source"]
probe("limit_l0_sparse_mode_row_deleted_P1", "u8_l0_isolated_node_sparse_interactive", remove_case_row(L0S, "linear_solver_mode_basis", "case"), G("G8", PREP), G("G8", PREP), item="3 limit", note="P1 still catches a deleted mode row")

# ---------------------------------------------------------------------------------
# Item 4: G5's not_required rule on 07j's must-pass (two_case_preparation_failure).
PB = "two_case_preparation_failure_synthetic"
ps = C[PB]["source"]
NR = MP["not_required_second_case_checks_passed"]["edits"]
assert NR[1] == RM(rb("product_attempts", 1))


def nr(pid, extra, want, want_i1, base_edits=None, note=""):
    probe(pid, PB, (NR if base_edits is None else base_edits) + extra, want, want_i1, item="4 G5", note=note)


def oa1(*p):
    return rb("ordinary_attempts", 1, *p)


nr("nr_must_pass", [], ADM(True), ADM(True))
nr("nr_w2_published_evaluation", w2_published(1, ps), ADM(True), G("G5", ATTEMPT))
nr("nr_w2_published_formation", [
    S(oa1("initial"), {"kind": "formation_failure", "error": {"tag": "numerical_range", "name": "rv113"}, "basis_index": 0}),
    S(oa1("w2"), {"kind": "published", "trigger": {"tag": "formation", "error": {"tag": "numerical_range", "name": "rv113"}}, "force_scale_exponent": -3, "report_diagnostic_ref": ps["retained_precision"]["body"]["ordinary_attempts"][1]["initial"]["report_diagnostic_ref"]}),
], ADM(True), G("G5", ATTEMPT))
nr("nr_w2_failed_checks_passed", w2_failed(1, ps), ADM(True), G("G5", ATTEMPT), note="DESIGN's rule has no W2 conjunct")
nr("nr_structural_failure_not_triggered", [S(oa1("initial"), {"kind": "structural_failure", "error": {"tag": "range", "detail": "rv113"}, "diagnostic_ref": None})], ADM(True), G("G5", ATTEMPT), note="initial attempted, not a report; W2 untriggered")
nr("nr_report_outcome_sensitive", [S(oa1("initial", "outcome"), "sensitive")], G("G5", ATTEMPT), G("G5", ATTEMPT))
nr("nr_initial_not_attempted", [S(oa1("initial"), {"kind": "not_attempted", "cause": "ineligible"})], G("G5", ATTEMPT), G("G5", ATTEMPT))
nr("nr_verdict_sensitive", [S(["numerical_quality", "cases", 1, "solve_quality"], "sensitive")], G("G5", ATTEMPT), G("G5", ATTEMPT))
nr("nr_w2_published_verdict_sensitive", w2_published(1, ps) + [S(["numerical_quality", "cases", 1, "solve_quality"], "sensitive")], G("G5", ATTEMPT), G("G5", ATTEMPT))
nr("nr_verdict_not_assessed", [S(["numerical_quality", "cases", 1, "solve_quality"], "not_assessed")], G("G5", ATTEMPT), G("G5", ATTEMPT))
nr("nr_report_with_w2_published", [S(oa1("w2"), w2_published(1, ps)[1]["value"])], G("G5", ATTEMPT), G("G5", ATTEMPT), note="D6c: W2 needs an initial failure")
keep_attempt = [e for e in NR if e != RM(rb("product_attempts", 1))]
nr("nr_product_attempt_ref_with_attempt", [S(rb("cases", 1, "product_attempt_ref"), 1)], G("G5", ATTEMPT), G("G5", ATTEMPT), base_edits=keep_attempt, note="reachable past G3: the attempt exists and names the case")
nr("nr_product_attempt_ref_dangling", [S(rb("cases", 1, "product_attempt_ref"), 0)], G("G3", COVERAGE), G("G3", COVERAGE))
# Dense: W2-published not_required case with and without a parity row (P4).
nz_edits = [S(rb("ordinary_attempts", 0, "requested_mode"), "dense_scrutiny"), S(rb("ordinary_attempts", 1, "requested_mode"), "dense_scrutiny")]


def nr_dense(pid, extra_rows, extra, want, want_i1):
    src = copy.deepcopy(ps)
    rows = copy.deepcopy(src["results"])
    for case in ["case:six-component-load", UR]:
        rows[row_index(src, "linear_solver_mode_basis", case)]["value"] = 2.0
    probe(pid, PB, NR + nz_edits + [S(["results"], rows + extra_rows)] + extra, want, want_i1, inv=INV_DENSE, item="4 G5")


NP = parity_for(UR, "result:rv113:parity-nr")
NP0 = parity_for("case:six-component-load", "result:rv113:parity-nr0", True)
nr_dense("nz_w2_published_no_parity", [NP0], w2_published(1, ps), ADM(True), G("G5", ATTEMPT))
nr_dense("nz_w2_published_parity_P4", [NP0, NP], w2_published(1, ps), G("G8", PREP), G("G5", ATTEMPT))
nr_dense("nz_report_parity_b0", [NP0, NP], [], ADM(True), ADM(True))
nr_dense("nz_report_no_parity_b0", [NP0], [], ADM(True), ADM(True), )

# A registered CaseSource that nothing names, beside 07j's T-7 preparation failure (case 1,
# no source): pre-existing behaviour, recorded for the D38 audit's "nothing names it" clause.
orphan = copy.deepcopy(ps["retained_precision"]["body"]["sources"][0])
orphan["index"] = 1
orphan["owner"] = {"kind": "case", "case_index": 1, "case_id": UR}
orphan["preparation"] = {"attempt_ref": 1, "sha256": orphan["preparation"]["sha256"]} if isinstance(orphan.get("preparation"), dict) else orphan.get("preparation")
probe("orphan_source_beside_t7", PB, [S(rb("sources"), ps["retained_precision"]["body"]["sources"] + [orphan])], {"observe": True}, {"observe": True}, item="2 audit", note="observation only")

# ---------------------------------------------------------------------------------
# SR-TS (RR "I92's SR-TS verified; ...", ruling 2): C2's cause branches. TS checks, for an unavailable case whose
# cause is not prepared_product_failure, the cause's phase and code (and, for facade, a selected Run and the owner);
# RS and PY do not. On two_case_synthetic, case 1 becomes unavailable while keeping its Ready attempt and selected
# Run (D19's receipt_failure form). `want` is RS's verdict here; TS's is compared separately.
TC = "two_case_synthetic"
tcs = C[TC]["source"]
tcb = tcs["retained_precision"]["body"]
tc1 = tcb["cases"][1]
di = [i for i, d in enumerate(tcs["diagnostics"]) if d["code"] == "RETAINED_PRECISION_SELECTED" and tc1["basis_ref"]["ref_id"] in d.get("affected_refs", [])][0]
tc_rows = copy.deepcopy(tcs["results"])
for r in tc_rows:
    if r["basis_ref"]["ref_id"] == tc1["basis_ref"]["ref_id"]:
        r.pop("recovery_method", None)


def c2(pid, reason, want, note=""):
    case = {"basis_ref": tc1["basis_ref"], "ordinary": tc1["ordinary"], "product_attempt_ref": 1, "status": "unavailable",
            "reason": reason, "diagnostic_ref": tcs["diagnostics"][di]["id"], "run": tc1["run"], "source_ref": 1}
    probe(pid, TC, [S(rb("cases", 1), case), S(["diagnostics", di, "code"], "RETAINED_PRECISION_UNAVAILABLE"), S(["results"], tc_rows)],
          want, want, item="SR-TS C2", note=note)


RF = {"kind": "receipt_failure", "check": "encoding", "field_path": "retained_precision.body"}
FF = {"kind": "facade_failure", "owner_ref": {"kind": "case", "index": 1}, "row_id": None, "recipe": "identity", "operand_index": None, "check": "identity", "predicate": None}
c2("c2_receipt_ok", {"code": "receipt_encoding", "phase": "receipt", "cause": RF}, ADM(False), "a Ready attempt under receipt_failure (D19); the branch satisfied")
c2("c2_receipt_phase_kernel", {"code": "kernel_unresolved", "phase": "kernel", "cause": RF}, ADM(False), "the branch broken: RS admits")
c2("c2_receipt_code_facade", {"code": "facade_certificate", "phase": "receipt", "cause": RF}, ADM(False), "the branch broken: RS admits")
c2("c2_receipt_phase_preparation", {"code": "source_unavailable", "phase": "preparation", "cause": RF}, ADM(False), "the branch broken: RS admits")
c2("c2_facade_ok", {"code": "facade_certificate", "phase": "facade", "cause": FF}, G("G5", PRODUCT), "D19: a Ready attempt under facade_failure")
c2("c2_facade_phase_kernel", {"code": "facade_certificate", "phase": "kernel", "cause": FF}, G("G5", PRODUCT), "D19 in RS; the branch first in TS")

# ---------------------------------------------------------------------------------
# SR-TS: RV108 N4 (non-object rows; the full reader refuses at G1, transport admits) and N6 (the G7 header codes
# baseHeaderCode's doc names). `want` is the bound verdict; the transport verdicts are read from the output lines.
OP = "ordinary_prepared_synthetic"
ops = C[OP]["source"]
for form, rows in [("null_first", [None] + ops["results"][1:]), ("null_appended", ops["results"] + [None]),
                   ("number_first", [1] + ops["results"][1:]), ("string_first", ["row"] + ops["results"][1:]), ("array_first", [[]] + ops["results"][1:])]:
    probe("n4_" + form, OP, [S(["results"], rows)], G("G1", "RETAINED_PRECISION_RECEIPT_MISMATCH"), G("G1", "RETAINED_PRECISION_RECEIPT_MISMATCH"), item="SR-TS N4", note="transport verdict read separately")
for field in ["structural_status", "model_matrix_fidelity", "accuracy_evidence"]:
    for kind, value in [("list", ["passive_model_basis"]), ("dict", {"value": "passive_model_basis"})]:
        probe(f"n6_{field}_{kind}", OP, [S(["numerical_quality", "cases", 0, field], value)], {"observe": True}, {"observe": True}, item="SR-TS N6")
for kind, value in [("list", ["sensitive"]), ("dict", {"value": "sensitive"})]:
    probe(f"n6_status_{kind}", OP, [S(["numerical_quality", "status"], value)], {"observe": True}, {"observe": True}, item="SR-TS N6")
probe("n6_carrier_evidence_with_case_defect", OP, [S(["carrier_evidence"], {}), S(["numerical_quality", "cases", 0, "structural_status"], ["x"])], {"observe": True}, {"observe": True}, item="SR-TS N6")
probe("n6_contract_evidence_null_and_source_block_recovery", OP, [S(["contract_evidence"], None), S(["source_block_recovery"], {})], {"observe": True}, {"observe": True}, item="SR-TS N6")

json.dump(probes, open(out_path, "w"), indent=1)
print(len(probes), "probes ->", out_path)
