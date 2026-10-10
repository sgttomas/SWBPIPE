"""B2's Python reader controls (B2-C, final for J1: CONTRACT with REVISION_01 and REVISION_02).

The base is PP's committed W-CB3 combination successor (`retained_precision_combination_successor_<mode>.json`, both
modes): case A selected, case B `not_required` with one OperandPreparation, and A + B `retained_selected`. Each edit is
resealed by the shared format rule with S-6's steps (`apply_mutation`), so each refusal is the check named, not a hash.
Edits whose shape the producer cannot emit are reader controls, not producer execution.
"""
from copy import deepcopy
import hashlib
import json
from pathlib import Path

import pytest

from core.analysis_runs import retained_precision as rp
from core.analysis_runs import preview_physics_evidence as ppe
from test_retained_precision_contract import _rehash_ref, apply_mutation

ROOT = Path(__file__).resolve().parents[1]
MODES = ("sparse_interactive", "dense_scrutiny")
B = ["retained_precision", "body"]
C0 = B + ["combinations", 0]
UNSUPPORTED = ("G0", "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED")
FORMATION = ("G0", "RETAINED_PRECISION_FORMATION_MISMATCH")
RECEIPT = ("G1", "RETAINED_PRECISION_RECEIPT_MISMATCH")
ENCODING = ("G2", "RETAINED_PRECISION_ENCODING_MISMATCH")
COVERAGE = ("G3", "RETAINED_PRECISION_COVERAGE_MISMATCH")
DIAGNOSTIC = ("G4", "RETAINED_PRECISION_DIAGNOSTIC_MISMATCH")
ATTEMPT = ("G5", "RETAINED_PRECISION_ATTEMPT_MISMATCH")
WORK = ("G5", "RETAINED_PRECISION_WORK_MISMATCH")
PRODUCT = ("G5", "RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH")
ROW_METHOD = ("G6", "RETAINED_PRECISION_ROW_METHOD_MISMATCH")
INVOCATION = ("G8", "RETAINED_PRECISION_INVOCATION_MISMATCH")
PREPARATION = ("G8", "RETAINED_PRECISION_PREPARATION_MISMATCH")
EVIDENCE = ("G7", "SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID")


def base(mode="sparse_interactive"):
    doc = json.loads((ROOT / f"fixtures/results/retained_precision_combination_successor_{mode}.json").read_text())
    return doc["source"], doc["invocation"]


def _set(path, value):
    return {"op": "set", "path": path, "value": value}


def verdict(source, invocation):
    try:
        result = rp.validate_retained_precision(deepcopy(source), deepcopy(invocation))
        return ("pass", result["standing"])
    except rp.RetainedPrecisionError as error:
        return (error.gate, error.code)


def edited(edits, mode="sparse_interactive", change=None, invocation_change=None, after=None):
    """The base with `edits` (shared edit grammar) and an optional in-place `change(source)`, resealed (S-6)."""
    source, invocation = base(mode)
    source = apply_mutation(source, {"edits": edits, "rehash": "all"})
    if change is not None:
        change(source)
        source = apply_mutation(source, {"edits": [], "rehash": "all"})
    if invocation_change is not None:
        invocation_change(invocation["request"]["model"])
        source["retained_precision"]["body"]["invocation"]["value"] = rp._hash("source_blocks_invocation_v1", invocation)
        source = apply_mutation(source, {"edits": [], "rehash": "all"})
    if after is not None:
        after(source)
    return source, invocation


def body(source):
    return source["retained_precision"]["body"]


# ---------------------------------------------------------------------------------------------------------------
# The producer-solved successors read through every gate.

@pytest.mark.parametrize("mode", MODES)
def test_b2_combination_successor_reads_bound_unbound_and_transport(mode):
    source, invocation = base(mode)
    result = rp.validate_retained_precision(deepcopy(source), deepcopy(invocation))
    assert (result["invocation_bound"], result["numerical_eligible"], result["standing"]) == (True, True, "eligible")
    assert result == rp._validate_draft(deepcopy(source), deepcopy(invocation))
    cases = [c for c in result["classifications"] if c["basis_ref"]["ref_type"] == "load_case"]
    combined = [c for c in result["classifications"] if c["basis_ref"]["ref_type"] == "combination"]
    # The case's classes first, then the combination's (CONTRACT §5): every combination row but its five force and
    # five moment support magnitudes, which are certified rows too, is classified by G5c as a case's row is.
    assert result["classifications"] == cases + combined
    assert [c["result_id"] for c in combined] == body(source)["combinations"][0]["result_ids"]
    assert {c["class"] for c in combined} == {"absolute_verified", "relative_verified", "input_derived"}
    assert sorted(c["result_id"] for c in combined if c["class"] == "absolute_verified") == sorted(x["result_id"] for x in body(source)["combinations"][0]["selection"]["absolute_verified"])
    unbound = rp.validate_retained_precision(deepcopy(source))
    assert (unbound["invocation_bound"], unbound["numerical_eligible"], unbound["standing"]) == (False, False, "needs_recompute")
    transport = rp.validate_retained_precision_transport(deepcopy(source))
    assert transport["standing"] == "needs_recompute" and transport["classifications"] == []


# ---------------------------------------------------------------------------------------------------------------
# G0 (CONTRACT §8): the table as an internal parameter; 8 cross-check, 2 definition and 1 control tests.

def statics(edit_table=None, edit_combination=None):
    copy = rp._preview_statics()
    table = json.loads(copy["table"])
    if edit_table is not None:
        edit_table(table)
        copy["table"] = (json.dumps(table, indent=2, ensure_ascii=True) + "\n").encode()
        copy["table_hash"] = hashlib.sha256(copy["table"]).hexdigest()
    if edit_combination is not None:
        edit_combination(copy["definitions"][rp.COMBINATION_DEFINITION_ID])
    return copy


def g0(edit_table=None, edit_combination=None, receipt=None):
    receipt = base()[0]["retained_precision"] if receipt is None else receipt
    try:
        rp._g0_preview(deepcopy(receipt), statics(edit_table, edit_combination))
        return "pass"
    except rp.RetainedPrecisionError as error:
        return (error.gate, error.code, error.detail)


def test_b2_g0_positive_control_on_the_packaged_table():
    assert g0() == "pass"
    assert rp._preview_statics()["table_hash"] == rp.TABLE_HASH


@pytest.mark.parametrize("label, edit", [
    ("receipt_bindings.canonicalization", lambda t: t["receipt_bindings"].update(canonicalization="openpipestress_jcs_ijson_v2")),
    ("receipt_bindings.method", lambda t: t["receipt_bindings"].update(method="other_method")),
    ("receipt_bindings.projection_policy", lambda t: t["receipt_bindings"].update(projection_policy="RP-LOGICAL-ATTEMPTS-v2")),
    ("receipt_bindings.work.case_limit", lambda t: t["receipt_bindings"]["work"].update(case_limit=20_000_000_001)),
    ("receipt_bindings.work.invocation_limit", lambda t: t["receipt_bindings"]["work"].update(invocation_limit=60_000_000_001)),
    ("receipt_bindings.work_policy", lambda t: t["receipt_bindings"].update(work_policy="W1-LME-20B-60B-v2")),
    ("receipt_policy", lambda t: t.update(receipt_policy="M03-INTEGRITY-MP-v3")),
    ("accuracy_classification.policy", lambda t: t["accuracy_classification"].update(policy="RP-FACADE-SI-v3")),
])
def test_b2_g0_table_constant_cross_check(label, edit):
    """§8 row 6: a test-only table whose bound value differs from the reader's constant, its own hash pinned to it,
    with the receipt unchanged."""
    assert g0(edit) == UNSUPPORTED + ("table/constant cross-check",), label


def test_b2_g0_definitions_reordered_or_rebound():
    """§8 row 4: the list is [DEF-O, DEF-C] in that order, and each packaged definition's H is its constant."""
    reorder = lambda t: t["product_formation_definitions"].reverse()
    assert g0(reorder) == FORMATION + ("formation definitions",)
    assert g0(lambda t: t["product_formation_definitions"][1].update(sha256="0" * 64)) == FORMATION + ("formation definitions",)
    assert g0(edit_combination=lambda d: d.update(version=2)) == FORMATION + ("formation definitions",)


def test_b2_g0_row_9_definition_ids_are_the_tables():
    """§8 row 9 (m1, m2, m8, m65): an unknown id on a combination attempt or an operand preparation fails G0; DEF-C's id
    on a case attempt and DEF-O's on an OperandPreparation pass G0 (the shape is G1's)."""
    source, invocation = base()
    for path in (B + ["product_attempts", 1, "definition_id"], B + ["operand_preparations", 0, "definition_id"]):
        assert verdict(*edited([_set(path, "RP-UNKNOWN-v1")])) == UNSUPPORTED, path
    assert verdict(*edited([_set(B + ["product_attempts", 0, "definition_id"], rp.COMBINATION_DEFINITION_ID)])) == RECEIPT
    assert verdict(*edited([_set(B + ["product_attempts", 1, "definition_id"], rp.DEFINITION_ID)])) == RECEIPT
    assert verdict(*edited([_set(B + ["operand_preparations", 0, "definition_id"], rp.COMBINATION_DEFINITION_ID)])) == RECEIPT


# ---------------------------------------------------------------------------------------------------------------
# G1: the three inner hashes (REVISION_01 §5.2), each edited and the publication and receipt hashes resealed.

def resealed_outer(source):
    body(source)["publication_sha256"] = rp._hash("retained_precision_publication_mp_v2", {k: v for k, v in source.items() if k != "retained_precision"})
    source["retained_precision"]["receipt_sha256"] = rp._hash("retained_precision_receipt_mp_v2", body(source))


@pytest.mark.parametrize("label, change", [
    ("the combination's source_identity_sha256", lambda b: b["combinations"][0].update(source_identity_sha256="0" * 64)),
    ("CombinationSource.operands[1].source_identity_sha256", lambda b: b["sources"][2]["operands"][1].update(source_identity_sha256="0" * 64)),
    ("the operand-prepared CaseSource's preparation hash under retained_precision_preparation_v1",
     lambda b: b["sources"][1]["preparation"].update(sha256=rp._hash("retained_precision_preparation_v1", rp._preparation_payload(b["operand_preparations"][0], rp.DEFINITION_HASH)))),
])
def test_b2_g1_inner_hashes(label, change):
    """Each inner hash edited; the hashes that depend on it (S-6's later steps) and the outer ones resealed, so only
    the edited conjunct can refuse."""
    source, invocation = base()
    assert verdict(source, invocation) == ("pass", "eligible")
    change(body(source))
    b = body(source)
    if "operand-prepared" in label:  # step 4 then step 5 over the edited CaseSource
        b["sources"][2]["operands"][1]["source_identity_sha256"] = rp._source_hash(b["sources"][1])
    if "combination's" not in label:  # step 5 over the (edited) CombinationSource
        b["combinations"][0]["source_identity_sha256"] = rp._source_hash(b["sources"][2])
    resealed_outer(source)
    assert verdict(source, invocation) == RECEIPT, label
    assert verdict(source, None) == RECEIPT, label


def test_b2_g1_shapes_and_g2_encodings():
    assert verdict(*edited([_set(B + ["operand_preparations"], [])])) == RECEIPT  # m13: absent when empty (C-6)
    assert verdict(*edited([_set(B + ["groups", 1, "imports", 0, "slot"], "s2048")])) == RECEIPT
    assert verdict(*edited([_set(C0 + ["expression", "terms", 0, "factor"], "3FF0000000000000")])) == ENCODING  # m16
    assert verdict(*edited([_set(B + ["calls", 1, "requested_operands", 0, "factor"], "7ff8000000000000")])) == ENCODING  # m17


# ---------------------------------------------------------------------------------------------------------------
# G3 (CONTRACT §10.1 (a)-(g), REVISION_01 §4.2 (h)-(i), C3a-7).

def renumber_attempts(source):
    """The combination attempt moved before the case attempt (m26), every reference renumbered."""
    b = body(source)
    b["product_attempts"].reverse()
    for i, a in enumerate(b["product_attempts"]):
        a["id"] = i
    b["cases"][0]["product_attempt_ref"] = 1
    b["sources"][0]["preparation"]["attempt_ref"] = 1
    b["combinations"][0]["product_attempt_ref"] = 0


def move_operand_source(source):
    """m68: the operand-prepared CaseSource after the CombinationSource, indices and references renumbered."""
    b = body(source)
    b["sources"] = [b["sources"][0], b["sources"][2], b["sources"][1]]
    for i, s in enumerate(b["sources"]):
        s["index"] = i
    b["operand_preparations"][0]["source_ref"] = 2
    b["combinations"][0]["source_ref"] = 1
    b["combinations"][0]["run"]["origin"]["source_ref"] = 1
    b["product_attempts"][1]["source_ref"] = 1
    b["sources"][1]["operands"][1]["source_ref"] = 2
    b["calls"][1]["source_refs"] = [1]
    b["calls"][1]["requested_operands"][1]["source_ref"] = 2
    b["groups"][1]["source_refs"] = [1]
    b["groups"][1]["first_source_ref"] = 1


@pytest.mark.parametrize("label, edits, change", [
    ("(a) gate evidence entry renamed", [_set(["contract_evidence", "combination_gates", 0, "combination_id"], "combination:other")], None),
    ("(a) gate evidence entry removed", [_set(["contract_evidence", "combination_gates"], [])], None),
    ("(b) the combination id equals a case id", [], lambda s: rename_combination(s, "case:b")),
    ("(c) result_ids loses a row", [], lambda s: body(s)["combinations"][0]["result_ids"].pop()),
    ("(c) result_ids gains a case row", [], lambda s: body(s)["combinations"][0]["result_ids"].append(s["results"][0]["id"])),
    ("(c) two result_ids swapped", [], lambda s: swap(body(s)["combinations"][0]["result_ids"], 0, 1)),
    ("(d) the operand preparation owned by the selected case", [_set(B + ["operand_preparations", 0, "owner_ref", "index"], 0)], None),
    ("(d) requested_by empty of its combination", [_set(B + ["operand_preparations", 0, "requested_by"], [1])], None),
    ("(d) the operand preparation duplicated", [], lambda s: body(s)["operand_preparations"].append(dict(deepcopy(body(s)["operand_preparations"][0]), id=1))),
    ("(d) the operand preparation removed", [], lambda s: body(s).pop("operand_preparations")),
    ("(d) a prepared record's source not naming it", [_set(B + ["sources", 1, "preparation", "operand_preparation_ref"], 1)], None),
    ("(e) the combination attempt before the case attempt", [], renumber_attempts),
    ("(f) execution_order loses the combination Run", [_set(B + ["work", "execution_order"], [{"kind": "case", "index": 0}])], None),
    ("(f) an execution_order combination index changed", [_set(B + ["work", "execution_order", 1, "index"], 1)], None),
    ("(g) a CaseSource named by a case and a record", [_set(B + ["operand_preparations", 0, "source_ref"], 0)], None),
    ("(i) the operand-prepared CaseSource after the CombinationSource", [], move_operand_source),
    ("the combination attempt's owner index", [_set(B + ["product_attempts", 1, "owner_ref", "index"], 1)], None),
    ("a CombinationSource naming another id", [_set(B + ["sources", 2, "owner", "combination_id"], "combination:other")], None),
    ("a combination row's basis naming no entry", [_set(["results", -1, "basis_ref", "ref_id"], "combination:other")], lambda s: body(s)["combinations"][0]["result_ids"].remove(s["results"][-1]["id"])),
    ("(g) a CombinationSource no entry names", [], lambda s: body(s)["sources"].append(dict(deepcopy(body(s)["sources"][2]), index=3))),
    ("the combination attempt's id", [_set(B + ["product_attempts", 1, "id"], 5)], None),
    ("a projection outcome on a displacement magnitude row", [_set(B + ["product_attempts", 1, "proof", "projection_outcomes", 0, "row_index"], 0)], None),
    ("the combination attempt's summary roster reversed", [], lambda s: body(s)["product_attempts"][1]["proof"]["summary_coverage"].reverse()),
    ("(d) requested_by repeated", [_set(B + ["operand_preparations", 0, "requested_by"], [0, 0])], None),
    ("(d) requested_by naming a missing combination", [_set(B + ["operand_preparations", 0, "requested_by"], [0, 1])], None),
])
def test_b2_g3_coverage(label, edits, change):
    assert verdict(*edited(edits, change=change)) == COVERAGE, label


def swap(items, i, j):
    items[i], items[j] = items[j], items[i]


def rename_combination(source, new):
    old = body(source)["combinations"][0]["basis_ref"]["ref_id"]
    text = json.dumps(source).replace(json.dumps(old), json.dumps(new))
    source.clear()
    source.update(json.loads(text))


def _two_records():
    """A reader-logic body for G3 (h) and (i), out of B2's domain (two not_required operand cases): A + B + C with A
    selected, B and C each with a prepared operand preparation and its CaseSource, in first-need order."""
    case = lambda cid, status, source_ref=None: {"basis_ref": {"ref_type": "load_case", "ref_id": cid}, "status": status, "source_ref": source_ref}
    record = lambda ri, owner, source_ref: {"id": ri, "owner_ref": {"kind": "case", "index": owner}, "requested_by": [0], "result": {"kind": "prepared"}, "source_ref": source_ref}
    source = lambda si, ci, preparation: {"index": si, "owner": {"kind": "case", "case_index": ci, "case_id": "abc"[ci]}, "preparation": preparation}
    terms = [{"case_id": x, "factor": "3ff0000000000000"} for x in "abc"]
    body = {"cases": [case("a", "selected", 0), case("b", "not_required"), case("c", "not_required")],
            "combinations": [{"basis_ref": {"ref_type": "combination", "ref_id": "abc"}, "disposition": "retained_selected", "result_ids": [], "source_ref": 3,
                              "expression": {"kind": "mechanics", "terms": terms}}],
            "operand_preparations": [record(0, 1, 1), record(1, 2, 2)], "product_attempts": [],
            "sources": [source(0, 0, {"attempt_ref": 0, "sha256": ""}), source(1, 1, {"operand_preparation_ref": 0, "sha256": ""}),
                        source(2, 2, {"operand_preparation_ref": 1, "sha256": ""}), {"index": 3, "owner": {"kind": "combination", "combination_index": 0, "combination_id": "abc"}}]}
    snapshot = {"contract_evidence": {"combination_gates": [{"combination_id": "abc", "withheld": False, "reason": None}]}}
    return snapshot, body


def _g3(snapshot, body):
    try:
        rp._g3_combinations(snapshot, body, [c["basis_ref"]["ref_id"] for c in body["cases"]], {"abc": []})
        return "pass"
    except rp.RetainedPrecisionError as error:
        return (error.gate, error.code)


def test_b2_g3_first_need_order_and_record_order_reader_logic():
    snapshot, body = _two_records()
    assert _g3(snapshot, body) == "pass"
    # (h): C's record first, then B's, each source in record order, so only the first-need order fails.
    swapped = deepcopy(body)
    swapped["operand_preparations"] = [dict(swapped["operand_preparations"][1], id=0, source_ref=1), dict(swapped["operand_preparations"][0], id=1, source_ref=2)]
    swapped["sources"][1]["owner"].update(case_index=2, case_id="c")
    swapped["sources"][2]["owner"].update(case_index=1, case_id="b")
    assert _g3(snapshot, swapped) == COVERAGE
    # (g): a CaseSource no case and no record names.
    orphan = deepcopy(body); orphan["cases"][0]["source_ref"] = None
    assert _g3(snapshot, orphan) == COVERAGE
    # (i): the two operand-prepared CaseSources out of record order.
    reordered = deepcopy(body)
    reordered["sources"][1], reordered["sources"][2] = reordered["sources"][2], reordered["sources"][1]
    for i, s in enumerate(reordered["sources"]): s["index"] = i
    reordered["operand_preparations"][0]["source_ref"], reordered["operand_preparations"][1]["source_ref"] = 2, 1
    assert _g3(snapshot, reordered) == COVERAGE
    # (d): a not_required term case with no record, and a record whose requested_by names no combination.
    missing = deepcopy(body); missing["operand_preparations"].pop(); missing["sources"].pop(2)
    for i, s in enumerate(missing["sources"]): s["index"] = i
    missing["combinations"][0]["source_ref"] = 2
    assert _g3(snapshot, missing) == COVERAGE
    stray = deepcopy(body); stray["operand_preparations"][1]["requested_by"] = [0, 1]
    assert _g3(snapshot, stray) == COVERAGE


# ---------------------------------------------------------------------------------------------------------------
# G4 (CONTRACT §10.1).

def test_b2_g4_combination_diagnostics():
    selected = "diagnostic:retained-precision:combination:ab:selected"
    drop = lambda s: s.update(diagnostics=[d for d in s["diagnostics"] if d["id"] != selected])
    assert verdict(*edited([], change=drop)) == DIAGNOSTIC  # m31
    both = lambda s: next(d for d in s["diagnostics"] if d["id"] == selected).update(affected_refs=["combination:ab", "case:a"])
    assert verdict(*edited([], change=both)) == DIAGNOSTIC  # m34
    unavailable = lambda s: s["diagnostics"].append({"id": "diagnostic:retained-precision:combination:ab:unavailable", "code": "RETAINED_PRECISION_UNAVAILABLE",
                                                     "severity": "info", "message": "x", "source": "core/product_physics", "affected_refs": ["combination:ab"]})
    assert verdict(*edited([], change=unavailable)) == DIAGNOSTIC
    stray = lambda s: next(d for d in s["diagnostics"] if d["id"] == selected).update(affected_refs=["combination:other"])
    assert verdict(*edited([], change=stray)) == DIAGNOSTIC


# ---------------------------------------------------------------------------------------------------------------
# G5 native class (CONTRACT §10.1; REVISION_01 §4.1 #7-#10).

@pytest.mark.parametrize("label, edits, change, expected", [
    ("requested_operands swapped (m38)", [], lambda s: body(s)["calls"][1]["requested_operands"].reverse(), ATTEMPT),
    ("a requested operand's source the case batch's", [_set(B + ["calls", 1, "requested_operands", 1, "source_ref"], 0)], None, ATTEMPT),
    ("representative_source_ref operand 1's (m39)", [_set(B + ["sources", 2, "representative_source_ref"], 1)], None, ATTEMPT),
    ("operands[0].case_index the other case (m40)", [_set(B + ["sources", 2, "operands", 0, "case_index"], 1)], None, ATTEMPT),
    ("an operand's factor", [_set(B + ["sources", 2, "operands", 1, "factor"], "4000000000000000")], None, ATTEMPT),
    ("the CombinationSource's stiffness", [_set(B + ["sources", 2, "stiffness_sha256"], "0" * 64)], None, ATTEMPT),
    ("the CombinationSource's ledger not the selection's", [_set(B + ["sources", 2, "ledger_sha256"], "0" * 64)], None, ATTEMPT),
    ("an import from the prepared operand (m45)", [_set(B + ["groups", 1, "imports", 0, "operand_index"], 1)], None, ATTEMPT),
    ("an import's selected_run", [_set(B + ["groups", 1, "imports", 0, "selected_run"], 1)], None, ATTEMPT),
    ("the combination group's imports removed (m42)", [], lambda s: body(s)["groups"][1].pop("imports"), ATTEMPT),
    ("a case group with imports", [_set(B + ["groups", 0, "imports"], [])], None, ATTEMPT),
    ("the mechanics Call's owner index", [_set(B + ["calls", 1, "owner_refs", 0, "index"], 1)], None, ATTEMPT),
    ("the combination Call run_refs emptied", [_set(B + ["calls", 1, "run_refs"], [])], None, ATTEMPT),
    ("one more requested operand", [], lambda s: body(s)["calls"][1]["requested_operands"].append(deepcopy(body(s)["calls"][1]["requested_operands"][0])), ATTEMPT),
    ("a requested operand's factor only", [_set(B + ["calls", 1, "requested_operands", 1, "factor"], "4000000000000000")], None, ATTEMPT),
    ("a second mechanics Call for the entry", [], lambda s: body(s)["calls"].append(dict(deepcopy(body(s)["calls"][1]), id=2, run_refs=[], source_refs=[],
        invocation_before=body(s)["calls"][1]["invocation_after"], result={"kind": "pre_source_refusal", "stage": "operand_validation", "reason": {"space": "combination", "tag": "operands_differ"}})), ATTEMPT),
    ("the combination Call's invocation_before (m43)", [_set(B + ["calls", 1, "invocation_before"], 1)], None, WORK),
    ("work.charged the batch Call's after (m44)", [], lambda s: body(s)["work"].update(charged=body(s)["calls"][0]["invocation_after"]), WORK),
])
def test_b2_g5_native_class(label, edits, change, expected):
    assert verdict(*edited(edits, change=change)) == expected, label


def native(b):
    try:
        rp._g5_combination_native(b, [], lambda ok: rp._need(ok, "G5", "ATTEMPT_MISMATCH"))
        return "pass"
    except rp.RetainedPrecisionError as error:
        return (error.gate, error.code)


def test_b2_g5_native_no_operands_keeps_the_authored_terms():
    """CONTRACT §2.5 (PR-B2 ruling 2): `requested_operands` are the expression's terms whatever the Call's result, a
    `no_operands` pre-source refusal included; an empty list against a non-empty expression is refused."""
    source, _ = base()
    b = body(source)
    assert native(b) == "pass"
    b["calls"][1].update(result={"kind": "pre_source_refusal", "stage": "operand_validation", "reason": {"space": "combination", "tag": "no_operands"}}, source_refs=[], run_refs=[])
    b["combinations"][0].update(run=None, source_ref=None)
    assert native(b) == "pass"
    b["calls"][1]["requested_operands"] = []
    assert native(b) == ATTEMPT


# ---------------------------------------------------------------------------------------------------------------
# G5 ordinary class: the disposition rule and D6a (CONTRACT §10.1).

def test_b2_g5_disposition_rule_and_d6a():
    assert verdict(*edited([_set(["contract_evidence", "combination_gates", 0], {"combination_id": "combination:ab", "withheld": True, "reason": "NONLINEAR_COMBINATION_REQUIRES_SOLVE"})])) == ATTEMPT
    source, _ = base()
    b, diags, quality = body(source), source["diagnostics"], source["numerical_quality"]["cases"]
    gates = source["contract_evidence"]["combination_gates"]
    rp._g5_ordinary(b, b["cases"], diags, quality, gates)
    def refused(change):
        bad = deepcopy(b); bad_gates = deepcopy(gates); change(bad, bad_gates)
        with pytest.raises(rp.RetainedPrecisionError) as error:
            rp._g5_ordinary(bad, bad["cases"], diags, quality, bad_gates)
        return (error.value.gate, error.value.code)
    assert refused(lambda x, g: x["combinations"][0].update(disposition="ordinary")) == ATTEMPT
    assert refused(lambda x, g: g[0].update(withheld=True, reason="NONLINEAR_COMBINATION_REQUIRES_SOLVE")) == ATTEMPT
    assert refused(lambda x, g: x["combinations"][0]["expression"]["terms"].__setitem__(1, x["combinations"][0]["expression"]["terms"][0])) == ATTEMPT
    assert refused(lambda x, g: x["cases"][0].update(status="not_required")) == ATTEMPT
    assert refused(lambda x, g: x["combinations"][0]["diagnostic_refs"].reverse()) == ATTEMPT
    assert refused(lambda x, g: x["combinations"][0]["diagnostic_refs"].append("diagnostic:retained-precision:combination:ab:selected")) == ATTEMPT
    ordinary = deepcopy(b)
    ordinary["combinations"][0] = {k: ordinary["combinations"][0][k] for k in ("basis_ref", "expression", "result_ids", "diagnostic_refs")}
    ordinary["combinations"][0].update(disposition="ordinary", reason="no_retained_mechanics", expression={"kind": "result_state_subtraction", "minuend_id": "case:a", "subtrahend_id": "case:b"})
    rp._g5_ordinary(ordinary, ordinary["cases"], diags, quality, gates)
    withheld = deepcopy(ordinary)
    withheld["combinations"][0].update(disposition="base_withheld", reason="CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE", expression=b["combinations"][0]["expression"])
    rp._g5_ordinary(withheld, withheld["cases"], diags, quality, [dict(gates[0], withheld=True, reason="CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE")])
    with pytest.raises(rp.RetainedPrecisionError):
        rp._g5_ordinary(withheld, withheld["cases"], diags, quality, [dict(gates[0], withheld=True, reason="NONLINEAR_COMBINATION_REQUIRES_SOLVE")])


# ---------------------------------------------------------------------------------------------------------------
# G5 products: the combination attempt, the reason table, operand preparations and N-5 (CONTRACT §4; REVISION_01 §3.3).

ORIGIN = {"kind": "origin", "cause": {"kind": "missing_selected_origin", "operand": 1}}


def failed_native(source):
    """The combination's attempt recast as a non-selected Run's (W-CB2's shape on W-CB3's bytes; a reader control)."""
    b = body(source)
    c = b["combinations"][0]
    for key in ("method", "source_identity_sha256", "selection"):
        c.pop(key)
    c.update(disposition="retained_unavailable", diagnostic_ref="diagnostic:retained-precision:combination:ab:unavailable",
             reason={"code": "combination_unresolved", "phase": "kernel", "cause": {"kind": "prepared_product_failure", "product_attempt_ref": 1}})
    c["run"]["kernel_terminal"] = {"kind": "unresolved", "reason": {"space": "unresolved", "tag": "ceiling"}}
    a = b["product_attempts"][1]
    a["stages"] = dict.fromkeys(a["stages"], "not_entered")
    a["stages"]["native"] = "failed"
    a.update(proof=None, result={"kind": "unavailable", "error": {"kind": "native", "run_ref": c["run"]["id"]}})


def test_b2_g5_combination_attempt_reason_table_reader_logic():
    """§4's reason table and the stage rules, on the attempt alone (a recast body, so G5's earlier passes are not run)."""
    source, _ = base()
    failed_native(source)
    b = body(source)
    rows = {c["basis_ref"]["ref_id"]: [r for r in source["results"] if r["basis_ref"] == c["basis_ref"]] for c in b["combinations"]}
    fail = lambda ok: rp._need(ok, "G5", "PRODUCT_ATTEMPT_MISMATCH")
    def run(change=None):
        bad = deepcopy(b)
        if change: change(bad)
        try:
            rp._g5_combination_attempt(bad, bad["product_attempts"][1], 1, rows, fail, lambda ok: None)
            rp._g5_typed(bad["product_attempts"][1], fail, rp.COMBINATION_STAGE_ORDER, rp.COMBINATION_ERROR_STAGE_RECORDS)
            rp._g5_combination_entries(bad, fail)
            return "pass"
        except rp.RetainedPrecisionError as error:
            return (error.gate, error.code)
    assert run() == "pass"
    assert run(lambda x: x["product_attempts"][1]["stages"].update(native="completed")) == PRODUCT  # m52
    assert run(lambda x: x["combinations"][0]["reason"].update(code="kernel_unresolved")) == PRODUCT  # m53
    assert run(lambda x: x["combinations"][0]["reason"].update(code="facade_certificate", phase="facade")) == PRODUCT
    assert run(lambda x: x["product_attempts"][1]["result"].update(error={"kind": "capture", "cause": ORIGIN})) == PRODUCT  # m67, N-5
    assert run(lambda x: x["product_attempts"][1]["result"].update(error={"kind": "capture", "cause": {"kind": "storage", "detail": "x"}})) == "pass"
    assert run(lambda x: x["product_attempts"][1]["result"].update(error={"kind": "preparation", "capture": {"kind": "storage", "detail": "x"}, "section": None})) == PRODUCT
    assert run(lambda x: x["combinations"][0]["reason"]["cause"].update(product_attempt_ref=0)) == PRODUCT
    assert run(lambda x: x["product_attempts"][1].update(result={"kind": "ready"})) == PRODUCT


def recast_unavailable(cause, call_ref=None):
    def change(source):
        b = body(source)
        c = b["combinations"][0]
        for key in ("method", "source_identity_sha256", "selection"):
            c.pop(key)
        c.update(disposition="retained_unavailable", call_ref=call_ref, product_attempt_ref=None, run=None, source_ref=None,
                 diagnostic_ref="diagnostic:retained-precision:combination:ab:unavailable",
                 reason={"code": "combination_unresolved", "phase": "preparation", "cause": cause})
    return change


def test_b2_g5_no_call_causes_reader_logic():
    """The `retained_unavailable` reason table's no-Call rows (`_g5_combination_entries`)."""
    source, _ = base()
    b = body(source)
    fail = lambda ok: rp._need(ok, "G5", "PRODUCT_ATTEMPT_MISMATCH")
    def run(cause, change=None, call_ref=None):
        s = deepcopy(source)
        recast_unavailable(cause, call_ref)(s)
        if change: change(body(s))
        try:
            rp._g5_combination_entries(body(s), fail)
            return "pass"
        except rp.RetainedPrecisionError as error:
            return (error.gate, error.code)
    refused_record = lambda x: x["operand_preparations"][0].update(result={"kind": "refused", "error": {"kind": "preparation", "capture": {"kind": "storage", "detail": "x"}, "section": None}})
    opf = {"kind": "operand_preparation_failure", "operand_preparation_ref": 0}
    assert run(opf, refused_record) == "pass"
    assert run(opf) == PRODUCT  # the named record is prepared
    assert run(opf, lambda x: (refused_record(x), x["cases"][1].update(status="unavailable", source_ref=None))) == PRODUCT  # a term lacks a source
    osu = {"kind": "operand_source_unavailable", "operand_index": 1}
    assert run(osu, lambda x: x["cases"][1].update(status="unavailable", source_ref=None)) == "pass"
    assert run(dict(osu, operand_index=0), lambda x: x["cases"][1].update(status="unavailable", source_ref=None)) == PRODUCT  # m49
    assert run(osu) == PRODUCT  # B is not_required: no term lacks a source
    reason = {"space": "combination", "tag": "operands_differ"}
    to_refusal = lambda x: x["calls"][1].update(result={"kind": "pre_source_refusal", "stage": "operand_validation", "reason": reason}, run_refs=[], source_refs=[])
    assert run(reason, to_refusal, call_ref=1) == "pass"
    assert run(reason, to_refusal) == PRODUCT  # m48: call_ref null with a CombinationReason cause
    assert run({"space": "combination", "tag": "no_selected_operand"}, to_refusal, call_ref=1) == PRODUCT
    assert run({"kind": "source_error", "error": {"tag": "no_nodes"}}) == PRODUCT


@pytest.mark.parametrize("label, edits, change", [
    ("the prepared record's stage failed (m51)", [_set(B + ["operand_preparations", 0, "stage"], "failed")], None),
    ("the record's ordinary_attempt_ref", [_set(B + ["operand_preparations", 0, "ordinary_attempt_ref"], 0)], None),
    ("a prepared record's member refused", [_set(B + ["operand_preparations", 0, "preparation", "members", 0, "result"], {"kind": "refused", "error": {"kind": "invalid_geometry"}})], None),
    ("a member's conversion dropped", [], lambda s: body(s)["operand_preparations"][0]["preparation"]["members"][0]["conversions"].pop()),
])
def test_b2_g5_operand_preparation(label, edits, change):
    assert verdict(*edited(edits, change=change))[0] == "G5", label


def test_b2_g5_refused_operand_preparation_reader_logic():
    """C3a-7's G5 on a refused record: stage, source and N-5 (m50, m66)."""
    source, _ = base()
    b = body(source)
    record = b["operand_preparations"][0]
    record.update(result={"kind": "refused", "error": {"kind": "preparation", "capture": {"kind": "storage", "detail": "x"}, "section": None}}, stage="failed", source_ref=None)
    record["preparation"]["members"] = []
    record["operational"]["new"] = []
    fail = lambda ok: rp._need(ok, "G5", "PRODUCT_ATTEMPT_MISMATCH")
    rp._g5_operand_preparations(b, fail, lambda ok: None)
    for change in (lambda r: r.update(source_ref=0), lambda r: r["result"]["error"].update(capture=ORIGIN), lambda r: r.update(stage="completed")):
        bad = deepcopy(b); change(bad["operand_preparations"][0])
        with pytest.raises(rp.RetainedPrecisionError) as error:
            rp._g5_operand_preparations(bad, fail, lambda ok: None)
        assert (error.value.gate, error.value.code) == PRODUCT


# ---------------------------------------------------------------------------------------------------------------
# G5a-G5c, G6 and R-COMB-1.

def test_b2_g5b_g5c_and_g6_on_the_combination():
    scales = C0 + ["selection", "body_scales", 0, "translation"]
    source, _ = base()
    word = body(source)["combinations"][0]["selection"]["body_scales"][0]["translation"]
    assert verdict(*edited([_set(scales, format(int(word, 16) + 1, "016x"))])) == ("G5b", "RETAINED_PRECISION_SCALE_MISMATCH")  # m54
    row = body(source)["combinations"][0]["result_ids"][3]
    assert verdict(*edited([_set(C0 + ["selection", "not_covered"], [row])])) == ("G5c", "RETAINED_PRECISION_CLASSIFICATION_MISMATCH")  # m55
    assert verdict(*edited([], change=lambda s: s["results"][226].pop("recovery_method"))) == ROW_METHOD  # m57
    assert verdict(*edited([], change=lambda s: s["results"][0].pop("recovery_method"))) == ROW_METHOD


def test_b2_g5a_combination_data_facts_come_from_the_nonzero_operands():
    """DEF-C `lanes.loads`: a body with an operand term c_i*v_ij != 0 at a free DOF has data, read from every operand
    with a nonzero factor; operand 0's own terms alone do not decide it."""
    source, _ = base()
    b = body(source)
    representative, data = rp._numeric_source(b, "combination", 2)
    assert representative is b["sources"][0] and data == [b["sources"][0], b["sources"][1]]
    b["sources"][2]["operands"][1]["factor"] = "0000000000000000"
    assert rp._numeric_source(b, "combination", 2)[1] == [b["sources"][0]]


def test_b2_r_comb_1_classes_reader_logic():
    """CONTRACT §5 (R), REVISION_01 §2, REVISION_02 §4.1: a retained_unavailable combination's rows, and an ordinary
    combination's naming a case that is not not_required, are not_covered (records non_quantity), scale_bits null and
    normalized_bits from the value; an ordinary combination over not_required cases only adds nothing."""
    source, _ = base()
    b = body(source)
    rows = {c["basis_ref"]["ref_id"]: [r for r in source["results"] if r["basis_ref"] == c["basis_ref"]] for c in b["combinations"]}
    assert rp._r_comb_1(b, rows) == []
    record = dict(rows["combination:ab"][0], kind="combination_modulus_basis_record", unit="1", value=2.0)
    rows["combination:ab"].append(record)
    b["combinations"][0].update(disposition="retained_unavailable")
    classes = rp._r_comb_1(b, rows)
    assert [c["result_id"] for c in classes] == [r["id"] for r in rows["combination:ab"]]
    assert [c["class"] for c in classes] == ["not_covered"] * (len(classes) - 1) + ["non_quantity"]
    assert all(c["scale_bits"] is None and c["normalized_bits"] == rp.bits(rp._normalized(r)) for c, r in zip(classes, rows["combination:ab"]))
    b["combinations"][0].update(disposition="ordinary", expression={"kind": "mechanics", "terms": [{"case_id": "case:b", "factor": "4000000000000000"}]})
    assert rp._r_comb_1(b, rows) == []
    b["combinations"][0]["expression"] = {"kind": "range_envelope", "operand_ids": ["case:a", "case:b"], "mode": "max"}
    assert len(rp._r_comb_1(b, rows)) == len(rows["combination:ab"])


# ---------------------------------------------------------------------------------------------------------------
# G8: the invocation's combinations and expressions, K4CMB, R-8 and the operand preparation's binding.

@pytest.mark.parametrize("label, change", [
    ("a term's factor", lambda m: m["combinations"][0]["terms"][1].update(factor=2.0)),
    ("a term's case", lambda m: m["combinations"][0]["terms"][1].update(load_case="case:a")),
    ("terms reversed", lambda m: m["combinations"][0]["terms"].reverse()),
    ("the basis", lambda m: m["combinations"][0].update(basis="range_envelope")),
    ("the id", lambda m: m["combinations"][0].update(id="combination:other")),
    ("one more combination", lambda m: m["combinations"].append(deepcopy(m["combinations"][0]))),
    ("combinations removed", lambda m: m.pop("combinations")),
    ("combinations null", lambda m: m.update(combinations=None)),
])
def test_b2_g8_invocation_expressions(label, change):
    assert verdict(*edited([], invocation_change=change)) == INVOCATION, label


def test_b2_g8_expression_kinds_reader_logic():
    """Subtraction (minuend then subtrahend) and range (ids in UTF-8 byte order, and the mode) against the model."""
    b = {"combinations": [{"basis_ref": {"ref_type": "combination", "ref_id": "x"}, "expression": {"kind": "result_state_subtraction", "minuend_id": "a", "subtrahend_id": "b"}},
                          {"basis_ref": {"ref_type": "combination", "ref_id": "y"}, "expression": {"kind": "range_envelope", "operand_ids": ["Z", "a", "é"], "mode": "max_abs"}}]}
    model = {"combinations": [{"id": "x", "basis": "result_state_subtraction", "minuend_id": "a", "subtrahend_id": "b"},
                              {"id": "y", "basis": "range_envelope", "operand_ids": ["é", "a", "Z"], "mode": "max_abs"}]}
    assert rp._model_combinations_match(b, model)
    for change in (lambda m: m["combinations"][0].update(minuend_id="b", subtrahend_id="a"), lambda m: m["combinations"][1].update(mode="max"),
                   lambda m: m["combinations"][1].update(operand_ids=["a", "Z"])):
        bad = deepcopy(model); change(bad)
        assert not rp._model_combinations_match(b, bad)
    reversed_ids = deepcopy(b); reversed_ids["combinations"][1]["expression"]["operand_ids"].reverse()
    assert not rp._model_combinations_match(reversed_ids, model)  # m61


def test_b2_g8_k4cmb_r8_and_the_operand_preparation_binding():
    assert verdict(*edited([_set(B + ["sources", 2, "kernel_source_sha256"], "0" * 64)])) == PREPARATION  # m62
    old_facts = B + ["operand_preparations", 0, "preparation", "members", 0, "old_facts", 0]
    assert verdict(*edited([_set(old_facts, "3fc999999999999b")])) == PREPARATION  # m64
    wall = B + ["sources", 1, "section_terms", 0, "geometry", "effective_wall"]
    assert verdict(*edited([_set(wall, "3f847ae147ae147c")])) == PREPARATION  # m63
    source, _ = base()
    b = body(source)
    need = lambda ok: rp._need(ok, "G8", "PREPARATION_MISMATCH")
    rp._g8_combinations(b, need)
    for change in (lambda x: x["sources"][1]["section_terms"][0].update(length="4008000000000001"),
                   lambda x: x["sources"][1].update(material_basis_ref=1),
                   lambda x: x["combinations"][0]["selection"]["section_terms"][0].update(member_id="M2"),
                   lambda x: x["product_attempts"][1].update(material_basis_ref=1),
                   lambda x: x["sources"][2]["operands"][0].update(factor="4000000000000000")):
        bad = deepcopy(b); change(bad)
        with pytest.raises(rp.RetainedPrecisionError) as error:
            rp._g8_combinations(bad, need)
        assert (error.value.gate, error.value.code) == PREPARATION


def test_b2_k4cmb_is_fk_case_prep_combination_bytes():
    """K4CMB: "K4CMB\\x01", u32le(h), then per operand u64le(factor bits), u32le(len K4SRC) and its K4SRC bytes."""
    source, _ = base()
    b = body(source)
    combined = b["sources"][2]
    k4src = [rp._native_source_encoding(b["sources"][o["source_ref"]], True) for o in combined["operands"]]
    expected = b"K4CMB\x01" + (2).to_bytes(4, "little") + b"".join(int(o["factor"], 16).to_bytes(8, "little") + len(x).to_bytes(4, "little") + x
                                                                  for o, x in zip(combined["operands"], k4src))
    assert hashlib.sha256(expected).hexdigest() == combined["kernel_source_sha256"]


# ---------------------------------------------------------------------------------------------------------------
# N-1 (COMMON ruling 2): the extrema-number demand refuses a negative bound or gap; zero and -0 pass.

EXTREMUM = ["contract_evidence", "preview_cases", 0, "pipe_stress_extrema", 0]


@pytest.mark.parametrize("field", ["global_upper_bound_pa", "certified_gap_pa"])
def test_b2_n1_negative_extrema_values(field):
    source, invocation = edited([_set(EXTREMUM + [field], -1e-300)])
    with pytest.raises(rp.RetainedPrecisionError) as error:
        rp.validate_retained_precision(deepcopy(source), deepcopy(invocation))
    assert (error.value.gate, error.value.code) == EVIDENCE and error.value.detail.endswith("extrema numbers")
    with pytest.raises(rp.RetainedPrecisionError) as error:
        rp.validate_retained_precision_transport(deepcopy(source))
    assert (error.value.gate, error.value.code) == EVIDENCE and error.value.detail.endswith("extrema numbers")
    projected = rp._project(source, rp.PREVIEW_ROUTE)
    for read in (ppe.validate_preview_physics_evidence, ppe.validate_transport_metadata):
        with pytest.raises(ValueError, match="SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID: extrema numbers"):
            read(projected)
    for zero in (0.0, -0.0, 0):
        projected["contract_evidence"]["preview_cases"][0]["pipe_stress_extrema"][0][field] = zero
        ppe.validate_transport_metadata(projected)
    zeroed, invocation = edited([_set(EXTREMUM + [field], 0.0)])
    assert verdict(zeroed, invocation) == ("pass", "eligible")


# ---------------------------------------------------------------------------------------------------------------
# Entry 22 (COMMON ruling 3) on the preview route, and the 07e index rule at B2's new reference sites.

def test_b2_entry_22_preview_twin_invocation_combination_without_its_entry():
    """B2-C §10.1 G8 invocation admits model combinations; an invocation combination with no receipt entry fails the
    entries' equality, still at G8 INVOCATION_MISMATCH (the exact route keeps refusing any combination: B3b)."""
    extra = lambda m: m["combinations"].append({"id": "combination:x", "basis": "mechanics", "label": "x", "terms": [{"load_case": "case:a", "factor": 1.0}]})
    assert verdict(*edited([], invocation_change=extra)) == INVOCATION


def test_b2_rehash_rule_at_the_new_reference_sites():
    """S-6 (REVISION_01 §5.2) with 07e's index rule: an `operand_preparation_ref`, an operand's `source_ref` and a
    combination entry's `source_ref` that are not strict indices are skipped (left to the reader); strict ones rehash."""
    source, _ = base()
    for ref in (True, 0.5, -0.0, 7, None):
        mutated = deepcopy(source)
        b = body(mutated)
        b["sources"][1]["preparation"]["operand_preparation_ref"] = ref
        b["sources"][2]["operands"][1]["source_ref"] = ref
        b["combinations"][0]["source_ref"] = ref
        stale = (b["sources"][1]["preparation"]["sha256"], b["sources"][2]["operands"][1]["source_identity_sha256"], b["combinations"][0]["source_identity_sha256"])
        b["sources"][1]["preparation"]["sha256"] = b["sources"][2]["operands"][1]["source_identity_sha256"] = b["combinations"][0]["source_identity_sha256"] = "0" * 64
        out = body(apply_mutation(mutated, {"edits": [], "rehash": "all"}))
        assert (out["sources"][1]["preparation"]["sha256"], out["sources"][2]["operands"][1]["source_identity_sha256"], out["combinations"][0]["source_identity_sha256"]) == ("0" * 64,) * 3, ref
        assert _rehash_ref(b["sources"], ref) is None
    zeroed = deepcopy(source)
    b = body(zeroed)
    b["sources"][1]["preparation"]["sha256"] = b["sources"][2]["operands"][1]["source_identity_sha256"] = b["combinations"][0]["source_identity_sha256"] = "0" * 64
    assert body(apply_mutation(zeroed, {"edits": [], "rehash": "all"})) == body(source)


def test_b2_g0_exact_route_refuses_an_operand_preparation():
    """§8 row 9 on the exact route: its table binds DEF-E only, so an OperandPreparation (DEF-O's id) fails G0."""
    exact = json.loads((ROOT / "fixtures/results/retained_precision_exact_successor_sparse_interactive.json").read_text())["source"]
    receipt = deepcopy(exact["retained_precision"])
    rp._g0_exact(deepcopy(receipt))
    receipt["body"]["operand_preparations"] = deepcopy(body(base()[0])["operand_preparations"])
    with pytest.raises(rp.RetainedPrecisionError) as error:
        rp._g0_exact(receipt)
    assert (error.value.gate, error.value.code) == UNSUPPORTED


def test_b2_g5_native_pre_source_call_names_its_entry_reader_logic():
    """A pre-source refusal's Call has no Run, so only the Call/entry check binds its owner and `call_ref`."""
    source, _ = base()
    b = body(source)
    b["calls"][1].update(result={"kind": "pre_source_refusal", "stage": "operand_validation", "reason": {"space": "combination", "tag": "operands_differ"}}, source_refs=[], run_refs=[])
    b["combinations"][0].update(run=None, source_ref=None)
    assert native(b) == "pass"
    for change in (lambda x: x["calls"][1].update(owner_refs=[{"kind": "combination", "index": 1}]), lambda x: x["combinations"][0].update(call_ref=2),
                   lambda x: x["calls"][1].update(run_refs=[1]),
                   lambda x: x["combinations"][0].update(expression={"kind": "result_state_subtraction", "minuend_id": "case:a", "subtrahend_id": "case:b"})):
        bad = deepcopy(b); change(bad)
        assert native(bad) == ATTEMPT


def test_b2_g5_products_more_reader_logic():
    """The projection list of a Ready combination attempt; a native error's Run; a no-Call cause with a Call; and a
    prepared record that is not complete."""
    assert verdict(*edited([], change=lambda s: body(s)["product_attempts"][1]["proof"]["projection_outcomes"].pop())) == PRODUCT
    source, _ = base()
    failed_native(source)
    b = body(source)
    rows = {c["basis_ref"]["ref_id"]: [r for r in source["results"] if r["basis_ref"] == c["basis_ref"]] for c in b["combinations"]}
    fail = lambda ok: rp._need(ok, "G5", "PRODUCT_ATTEMPT_MISMATCH")
    b["product_attempts"][1]["result"]["error"]["run_ref"] = 0
    with pytest.raises(rp.RetainedPrecisionError):
        rp._g5_combination_attempt(b, b["product_attempts"][1], 1, rows, fail, lambda ok: None)
    s2, _ = base()
    recast_unavailable({"kind": "operand_source_unavailable", "operand_index": 1}, call_ref=1)(s2)
    body(s2)["cases"][1].update(status="unavailable", source_ref=None)
    with pytest.raises(rp.RetainedPrecisionError):
        rp._g5_combination_entries(body(s2), fail)
    s3, _ = base()
    record = body(s3)["operand_preparations"][0]
    record["preparation"]["members"][0]["result"] = {"kind": "refused", "error": {"kind": "invalid_geometry"}}
    record["operational"]["new"] = []
    with pytest.raises(rp.RetainedPrecisionError) as error:
        rp._g5_operand_preparations(body(s3), fail, lambda ok: None)
    assert (error.value.gate, error.value.code) == PRODUCT


def test_b2_g5_native_no_other_group_build_reused_reader_logic():
    """A combination Run reuses only its own group's builds and its imports (CONTRACT §10.1 G5 native)."""
    source, _ = base()
    b = body(source)
    run = b["combinations"][0]["run"]
    b["builds"].append(dict(deepcopy(b["builds"][1]), id=len(b["builds"]), slot="s512"))
    assert native(b) == "pass"
    run["records"][0]["shared_build_ref"] = len(b["builds"]) - 1
    assert native(b) == ATTEMPT


def test_b2_g5_combination_attempt_more_reader_logic():
    """The attempt's own Run reference, a Ready attempt with a failed check, and a refused record's material basis."""
    fail = lambda ok: rp._need(ok, "G5", "PRODUCT_ATTEMPT_MISMATCH")
    source, _ = base()
    failed_native(source)
    b = body(source)
    rows = {c["basis_ref"]["ref_id"]: [r for r in source["results"] if r["basis_ref"] == c["basis_ref"]] for c in b["combinations"]}
    b["product_attempts"][1]["run_ref"] = 0
    with pytest.raises(rp.RetainedPrecisionError):
        rp._g5_combination_attempt(b, b["product_attempts"][1], 1, rows, fail, lambda ok: None)
    s2, _ = base()
    b2 = body(s2)
    a = b2["product_attempts"][1]
    rp._g5_combination_attempt(b2, a, 1, rows, fail, lambda ok: None)
    a["stages"]["g5a"] = "failed"
    a["proof"]["checks"]["g5a"] = {"kind": "failed", "error": {"kind": "g5a", "cause": {}}}
    with pytest.raises(rp.RetainedPrecisionError):
        rp._g5_combination_attempt(b2, a, 1, rows, fail, lambda ok: None)
    s3, _ = base()
    record = body(s3)["operand_preparations"][0]
    record.update(result={"kind": "refused", "error": {"kind": "preparation", "capture": {"kind": "storage", "detail": "x"}, "section": None}}, stage="failed", source_ref=None, material_basis_ref=1)
    record["preparation"]["members"] = []
    record["operational"]["new"] = []
    with pytest.raises(rp.RetainedPrecisionError):
        rp._g5_operand_preparations(body(s3), fail, lambda ok: None)



# ---------------------------------------------------------------------------------------------------------------
# PR-B2 lane X: the shared B2 parity probes, pinned alike in the RS, PY and TS readers. Each shape is a forgery on a
# corpus base in the corpus entry grammar (S-6 rehash), with the bound and unbound reading every reader gives: a gate and
# code, or "pass" (rulings 1-3, a prepared operand preparation's completeness, a refused Call's members, UTF-8 order).

PARITY = json.loads((ROOT / "fixtures/results/retained_precision_b2_parity_probes.json").read_text())


@pytest.mark.parametrize("shape", PARITY["shapes"], ids=lambda s: s["name"])
def test_b2_parity_probes_shared(shape):
    from test_retained_precision_contract import apply_entry, corpus
    fixture = next(c for c in corpus()["cases"] if c["id"] == shape["base"])
    source, invocation = apply_entry(fixture, shape)

    def read(*args):
        try:
            rp.validate_retained_precision(deepcopy(source), *map(deepcopy, args))
            return "pass"
        except rp.RetainedPrecisionError as error:
            return {"gate": error.gate, "code": error.code}
    assert len(PARITY["shapes"]) == 16
    assert (read(invocation), read()) == (shape["expected"], shape["expected_unbound"])
