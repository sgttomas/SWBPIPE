"""U6b (I66): the F2a preview successor through the Python carriers (dispatch,
standing, binding refusal, classification summary, the AnalysisRun, and the
refusals of the legacy wrapper and the stress-neutral packager).

The milestone inputs are byte-identical copies of PP's pinned successor files
(D-U6-5): producer test outputs, not native Current evidence. Since U7 (D-U7-5) the
reader's eligibility is on: with its actual invocation and requested cases the milestone
stands numerically_eligible, and without them needs_recompute."""
from copy import deepcopy
import hashlib
import json
from pathlib import Path

import pytest

from core.analysis_runs import compatibility as c
from core.analysis_runs import retained_precision as rp

ROOT = Path(__file__).resolve().parents[1]
SUCC = c.PREVIEW_PHYSICS_RETAINED_CONTRACT_ID
PINNED = {
    "sparse_interactive": ("ac6986b0680e0df9d88c33a5bf4635372fc3b83cbb9080da44e6d32dbdca59dc",
                           "efc1a39bbe83840df6bd0761c932b8020285d3b8c005ba8fd0b45ba10d667494", [25, 69, 3, 1]),
    "dense_scrutiny": ("6cd1d249e5352aaffbd2b7d7349c74a1d0e0572df35500f66be49c3cad95c9b5",
                       "3e26499f17caff8f5fc0d46406bbe54acf43e8cb16e761784aa5074413b0ac4a", [25, 69, 3, 2]),
}
MODES = sorted(PINNED)
NOTICE = "Retained-precision recovery is unavailable for this load case. Its published rows keep their ordinary values, standing and diagnostics."


def milestone(mode):
    """D-U6-5: the pinned bytes, checked here; returns (source, invocation)."""
    raw = (ROOT / f"fixtures/results/retained_precision_milestone_successor_{mode}.json").read_bytes()
    assert hashlib.sha256(raw).hexdigest() == PINNED[mode][0]
    doc = json.loads(raw)
    assert doc["source"]["retained_precision"]["receipt_sha256"] == PINNED[mode][1]
    assert doc["invocation"]["solver_mode"] == mode
    return doc["source"], doc["invocation"]


def requested(invocation):
    return [{"ref_type": "load_case", "ref_id": case["id"]} for case in invocation["request"]["model"]["load_cases"]]


def rehash(source):
    """A forged but hash-consistent statement: the edit reaches the gate it targets."""
    source = deepcopy(source)
    body = source["retained_precision"]["body"]
    body["publication_sha256"] = rp._hash("retained_precision_publication_mp_v2", {k: v for k, v in source.items() if k != "retained_precision"})
    source["retained_precision"]["receipt_sha256"] = rp._hash("retained_precision_receipt_mp_v2", body)
    return source


def projected(source):
    """The reader's G7 projection: the same statement under the base identity."""
    base = deepcopy(source)
    del base["retained_precision"]
    base["producer"]["semantic_contract_id"] = c.PREVIEW_PHYSICS_CONTRACT_ID
    base["formulation_basis"]["profile_id"] = "product_preview_mechanics_v1"
    for row in base["results"]:
        row.pop("recovery_method", None)
    return base


def build(source):
    return c.build_analysis_run(source, input_manifest_ref={"object_type": "InputManifest", "ref": "manifest:u6b"}, input_manifest_hash="1" * 64)


def dispatch(source):
    try:
        c._source_contract(source)
        return "ok"
    except ValueError as error:
        return str(error)


def raises(code, fn, *args):
    with pytest.raises(ValueError) as error:
        fn(*args)
    assert str(error.value) == code, str(error.value)


@pytest.mark.parametrize("mode", MODES)
def test_dispatch_admits_the_successor_through_the_accepted_reader_only(mode):
    source, invocation = milestone(mode)
    contract, sha, path = c._source_contract(source)
    assert (contract, sha, path.name) == (SUCC, c.PREVIEW_PHYSICS_RETAINED_CONTRACT_SHA256, "semantic_contract_v0_3_preview_physics_retained_1.json")
    assert hashlib.sha256(path.read_bytes()).hexdigest() == rp.TABLE_HASH == sha
    assert c.is_fresh_contract_id(SUCC) and SUCC in c.CURRENT_RECORD_CONTRACT_IDS
    assert c.standing_reason(source) is None
    # The reader's first code; a G7 failure keeps the base validator's own text.
    broken = deepcopy(source)
    broken["retained_precision"]["receipt_sha256"] = "0" * 64
    raises("RETAINED_PRECISION_RECEIPT_MISMATCH", c._source_contract, broken)
    unit = deepcopy(source)
    index = next(i for i, row in enumerate(unit["results"]) if row["kind"] == "linear_solver_mode_basis")
    unit["results"][index]["unit"] = "m"
    unit = rehash(unit)
    with pytest.raises(rp.RetainedPrecisionError) as reader:
        rp.validate_retained_precision(unit)
    assert reader.value.gate == "G7" and reader.value.detail and reader.value.detail != reader.value.code
    raises(reader.value.detail, c._source_contract, unit)
    # The Python reader has no transport validator: a transported successor is refused.
    raises("SOURCE_PRODUCER_CONTRACT_UNSUPPORTED", lambda s: c._source_contract(s, check_receipt=False), source)


@pytest.mark.parametrize("mode", MODES)
def test_standing_is_eligible_only_with_the_invocation_and_comes_only_from_the_receipt(mode):
    source, invocation = milestone(mode)
    refs = requested(invocation)
    assert c.numerical_use_standing(source, refs) == "needs_recompute"
    assert c.numerical_use_standing(source, refs, invocation) == "numerically_eligible"
    validation = rp.validate_retained_precision(source, invocation)
    assert validation["invocation_bound"] and validation["numerical_eligible"]
    row = deepcopy(source)
    row["results"][0]["value"] = row["results"][0]["value"] * 2.0 + 1.0
    assert c.numerical_use_standing(row, refs) == "unsupported"
    assert c.numerical_use_standing(row, refs, invocation) == "unsupported"
    foreign = deepcopy(invocation)
    foreign["solver_mode"] = MODES[1 - MODES.index(mode)]
    assert c.numerical_use_standing(source, refs, foreign) == "unsupported"
    # numerical_quality never contributes: a hash-consistent checks_passed claim is
    # refused by the reader (D6b), never read by the ordinary branch.
    claimed = deepcopy(source)
    claimed["numerical_quality"]["status"] = "checks_passed"
    claimed["numerical_quality"]["cases"][0].update(solve_quality="checks_passed", structural_status="passive_model_basis",
                                                     model_matrix_fidelity="represented_equations_retained")
    claimed = rehash(claimed)
    with pytest.raises(rp.RetainedPrecisionError):
        rp.validate_retained_precision(claimed)
    assert c.numerical_use_standing(claimed, refs) == "unsupported"
    assert c.numerical_use_standing(claimed, refs, invocation) == "unsupported"


@pytest.mark.parametrize("mode", MODES)
def test_a_solved_status_is_required_before_the_eligibility_conjunct(mode):
    """U7: a hash-consistent successor whose mechanics status is not MECHANICS_SOLVED is refused
    at G7 by the base preview validator (a blocked envelope carries no preview evidence), so the
    reader's own MECHANICS_SOLVED conjunct is never the deciding one (I66 U7 slice F, mutant C2)."""
    source, invocation = milestone(mode)
    for status in ("MODEL_INCOMPLETE", "MECHANICS_FAILED", "NOT_RUN"):
        unsolved = deepcopy(source)
        unsolved["status"]["mechanics"] = status
        with pytest.raises(rp.RetainedPrecisionError) as error:
            rp.validate_retained_precision(rehash(unsolved), invocation)
        assert (error.value.gate, error.value.code) == ("G7", "SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID"), status
        assert c.numerical_use_standing(rehash(unsolved), requested(invocation), invocation) == "unsupported"


def test_standing_rule_conjuncts_with_eligibility_set():
    """The standing rule's conjuncts, through the seam (the reader's eligibility is on since U7)."""
    source, invocation = milestone("sparse_interactive")
    refs = requested(invocation)
    validation = dict(rp.validate_retained_precision(source, invocation), numerical_eligible=True)
    assert c._retained_standing_from(validation, source, refs) == "numerically_eligible"
    assert c._retained_standing_from(dict(validation, numerical_eligible=False), source, refs) == "needs_recompute"
    assert c._retained_standing_from(dict(validation, invocation_bound=False), source, refs) == "needs_recompute"
    for other in ([], [{"ref_type": "load_case", "ref_id": "other"}], refs + [{"ref_type": "load_case", "ref_id": "other"}]):
        assert c._retained_standing_from(validation, source, other) == "needs_recompute"
    unsolved = deepcopy(source)
    unsolved["status"]["mechanics"] = "MODEL_INCOMPLETE"
    assert c._retained_standing_from(validation, unsolved, refs) == "needs_recompute"
    not_required = deepcopy(source)
    not_required["retained_precision"]["body"]["cases"][0]["status"] = "not_required"
    q = not_required["numerical_quality"]["cases"][0]
    q.update(solve_quality="checks_passed", structural_status="passive_model_basis",
             model_matrix_fidelity="represented_equations_retained", accuracy_evidence="not_claimed")
    assert q["evidence_refs"]
    assert c._retained_standing_from(validation, not_required, refs) == "numerically_eligible"
    for field, value in (("solve_quality", "sensitive"), ("structural_status", "numerically_unresolved"),
                         ("model_matrix_fidelity", "assembly_uncertainty"), ("accuracy_evidence", "unresolved"),
                         ("evidence_refs", []), ("evidence_refs", ["diagnostic:absent"]),
                         ("basis_ref", {"ref_type": "load_case", "ref_id": "other"})):
        bad = deepcopy(not_required)
        bad["numerical_quality"]["cases"][0][field] = value
        assert c._retained_standing_from(validation, bad, refs) == "needs_recompute", field
    duplicate = deepcopy(not_required)
    duplicate["results"].append(deepcopy(duplicate["results"][0]))
    assert c._retained_standing_from(validation, duplicate, refs) == "needs_recompute"
    short = deepcopy(not_required)
    short["numerical_quality"]["cases"] = []
    assert c._retained_standing_from(validation, short, refs) == "needs_recompute"
    # Any other case status is not eligible, even with ordinarily eligible quality.
    for status in ("unavailable", "other", None):
        case = deepcopy(not_required)
        case["retained_precision"]["body"]["cases"][0]["status"] = status
        assert c._retained_standing_from(validation, case, refs) == "needs_recompute", status
    # RV88 N-3 (Q06): with two cases the requested refs must follow the receipt's case order.
    two = deepcopy(source)
    second = dict(deepcopy(two["retained_precision"]["body"]["cases"][0]), basis_ref={"ref_type": "load_case", "ref_id": "case-2"})
    two["retained_precision"]["body"]["cases"].append(second)
    ordered = [case["basis_ref"] for case in two["retained_precision"]["body"]["cases"]]
    assert c._retained_standing_from(validation, two, ordered) == "numerically_eligible"
    for wrong in (ordered[::-1], ordered[:1], ordered[1:], ordered + ordered[:1]):
        assert c._retained_standing_from(validation, two, wrong) == "needs_recompute", wrong


CASES = ROOT / "fixtures/results/retained_precision_carrier_cases.json"
# RR "RV88 on U6a, U6c, U6b (and U6d)": the only ruled differences between the
# languages' carriers; any other difference is a defect.
# RR "RV92 (U6f) on the whole of U6" adds the fifth (N-2, with N-5) and widens I67-F2 (N-3).
# RR "The memory branch merged into NUM; U7 slices T and P committed" adds D-U7-4 (format v4).
DECLARED = {"I67-F1:unregistered_invalid_statement", "I67-F2:display_only_binding_precheck",
            "F-U6b-2:python_refuses_transport", "F5:refused_statement_binding",
            "RV92-N2-N5:ts_refuses_token_rows_at_the_header", "D-U7-4:ts_requires_live_native_capture"}
# Every field a v4 form may carry; any other field is a defect, never silently ignored.
FORM_FIELDS = {"label", "fixtures", "invocation", "capture", "requested", "edits", "current_model_edits", "subject", "expected"}


def shared_cases():
    cases = json.loads(CASES.read_text())
    assert cases["format"] == "I66-U6-CARRIER-CASES-v4"
    docs = {}
    for fid, spec in cases["fixtures"].items():
        raw = (ROOT / spec["path"]).read_bytes()
        assert hashlib.sha256(raw).hexdigest() == spec["sha256"]
        doc = json.loads(raw)
        docs[fid] = doc if spec["shape"] == "milestone" else {"source": doc, "invocation": None}
    return cases, docs


def apply_case(case, doc):
    """(source, invocation or None, requested refs) of one shared case."""
    source, invocation = deepcopy(doc["source"]), deepcopy(doc["invocation"])
    for edit in case["edits"]:
        assert edit["op"] == "set"
        target = source if edit["target"] == "source" else invocation
        for key in edit["path"][:-1]:
            target = target[key]
        target[edit["path"][-1]] = edit["value"]
    refs = requested(doc["invocation"]) if case["requested"] == "invocation" else case["requested"]
    if isinstance(case["invocation"], dict):
        return source, deepcopy(case["invocation"]), refs
    return source, (invocation if case["invocation"] is not None else None), refs


def test_shared_carrier_cases_python():
    """The shared standing-parity scenarios: the same expectations Rust (U6a)
    and TypeScript (U6d) read from this file, the F-5 guard forms included."""
    cases, docs = shared_cases()
    for case in cases["cases"]:
        source, context, refs = apply_case(case, docs[case["fixture"]])
        assert c.numerical_use_standing(source, refs, context) == case["expected_standing"], case["id"]
        assert dispatch(source) == case["expected_dispatch"], case["id"]
    assert len(cases["cases"]) == 20
    # The raw fixtures' guard cases are refused only through their edit.
    raw = [fid for fid, spec in cases["fixtures"].items() if spec["shape"] == "raw"]
    assert raw == ["legacy_preview_0_1", "preview_physics_1_invented_sparse", "source_blocks_n05_sparse"]
    for fid in raw:
        assert dispatch(docs[fid]["source"]) == "ok" and c.numerical_use_standing(docs[fid]["source"], []) == "needs_recompute", fid


def expected_summary(source, current=False):
    """'by_validated_class': per receipt case, the reader's class counts, interval
    binding 0, and the not-Current withheld count (these forms carry no invocation).
    With `current`, the Current count: only absolute and not-covered rows."""
    classes = rp.validate_retained_precision(source)["classifications"]
    out = []
    for case in source["retained_precision"]["body"]["cases"]:
        mine = [item["class"] for item in classes if item["basis_ref"] == case["basis_ref"]]
        n = {name: mine.count(name) for name in ("relative_verified", "absolute_verified", "not_covered", "input_derived", "non_quantity")}
        out.append({"case_id": case["basis_ref"]["ref_id"], **n, "interval_bindable": 0,
                    "withheld": n["absolute_verified"] + n["not_covered"] if current else
                    n["relative_verified"] + n["absolute_verified"] + n["not_covered"] + n["input_derived"]})
    return out


def expected_binding(source, expected):
    rows = source["results"]
    if expected == "by_validated_class":
        classes = {item["result_id"]: item["class"] for item in rp.validate_retained_precision(source)["classifications"]}
        want = [{"absolute_verified": c.RULE_QUANTITY_BELOW_VERIFIED_FLOOR, "not_covered": c.RULE_QUANTITY_NOT_COVERED}.get(classes[row["id"]]) for row in rows]
        assert c.RULE_QUANTITY_BELOW_VERIFIED_FLOOR in want and None in want
        return want
    rule, code = expected.split(":", 1)
    code = None if code == "none" else code
    if rule == "every_row":
        return [code] * len(rows)
    assert rule == "source_blocks_summary"
    headline = ((source.get("summary") or {}).get("max_open_formula_stress") or {}).get("result_ref")
    want = [code if row["kind"] == "open_formula_stress_summary" or row["id"] == headline else None for row in rows]
    assert code in want and None in want
    return want


def test_declared_differences_python():
    """Each ruled difference, form by form, with Python's own expectation; Rust
    asserts its own from the same entries, and TS (I67) its own."""
    cases, docs = shared_cases()
    assert all(p in cases["scope"] for p in ("G7 parity compares the reader's (gate, code)", "parity there compares only accept against refuse",
                                              "no carrier authenticates producer origin", "a blocked envelope is refused at G7 with each language's own base code", "Python SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID", "An invalid enum value in a not_required case's quality is refused at G7 with each language's own code", "Python SOURCE_NUMERICAL_CASE_INVALID"))
    entries = cases["declared_differences"]
    assert {entry["id"] for entry in entries} == DECLARED and len(entries) == len(DECLARED)
    seen = set()
    for entry in entries:
        assert entry["ruling"] and entry["kind"] in {"language", "semantics"} and entry["forms"], entry["id"]
        for form in entry["forms"]:
            assert set(form["expected"]) == {"rust", "python", "typescript"}, (entry["id"], form["label"])
            assert set(form) <= FORM_FIELDS, (entry["id"], form["label"], set(form) - FORM_FIELDS)
            # v4 'capture': 'none' is TS's (no IPC capture); Python reads the invocation argument as for 'fixture'.
            assert form.get("capture", "none") == "none" and ("capture" not in form or form["invocation"] == "fixture"), form["label"]
            expected = form["expected"]["python"]
            for fid in form["fixtures"]:
                source, context, refs = apply_case(form, docs[fid])
                label = (entry["id"], form["label"], fid)
                # v4 'current_model_edits' change only TS's current model: applied here to a copy of the
                # invocation's model, they must change it, and Python's inputs (source, invocation, refs) stay as they are.
                if "current_model_edits" in form:
                    model = deepcopy(docs[fid]["invocation"]["request"]["model"])
                    before = deepcopy(model)
                    for edit in form["current_model_edits"]:
                        assert edit["op"] == "set" and set(edit) == {"path", "op", "value"}, label
                        target = model
                        for key in edit["path"][:-1]:
                            target = target[key]
                        assert edit["path"][-1] in target, label
                        target[edit["path"][-1]] = edit["value"]
                    assert model != before and context == docs[fid]["invocation"], label
                    seen.add(("current_model_edits", True))
                if "capture" in form:
                    seen.add(("capture", form["capture"]))
                seen.add((form["subject"], expected.get(form["subject"])))
                if form["subject"] == "standing":
                    assert c.numerical_use_standing(source, refs, context) == expected["standing"], label
                elif form["subject"] == "transport":
                    assert dispatch_transport(source) == expected["transport"], label
                elif form["subject"] == "summary":
                    want = expected_summary(source, current=expected["summary"] == "by_validated_class_current") if expected["summary"] in ("by_validated_class", "by_validated_class_current") else []
                    assert want and c.classification_summary(source, context, refs) == want, label
                else:
                    assert form["subject"] == "binding", label
                    assert [c.rule_binding_refusal(source, row) for row in source["results"]] == expected_binding(source, expected["binding"]), label
    # Every subject, vocabulary and v4 field that Python consumes is exercised, D-U7-4's side included.
    assert {subject for subject, _ in seen} == {"standing", "transport", "binding", "summary", "current_model_edits", "capture"}
    assert ("standing", "numerically_eligible") in seen and ("summary", "by_validated_class_current") in seen


def dispatch_transport(source):
    try:
        c._source_contract(source, check_receipt=False)
        return "ok"
    except ValueError as error:
        return str(error)


@pytest.mark.parametrize("mode", MODES)
def test_analysis_run_carries_the_complete_receipt_and_it_comes_back_out(mode):
    from tests.schema_validation import validate_instance
    source, invocation = milestone(mode)
    original = deepcopy(source)
    record = build(source)
    assert source == original
    run = record["analysis_run"]
    assert run["retained_precision"] == source["retained_precision"]
    assert run["reproducibility"]["semantic_contract"] == {"id": SUCC, "sha256": c.PREVIEW_PHYSICS_RETAINED_CONTRACT_SHA256}
    assert all(ref["semantic_contract"]["id"] == SUCC for ref in run["result_refs"])
    assert "contract_evidence" not in run and "source_block_recovery" not in run
    assert c.verify_analysis_run_record(record) == "match"
    c.validate_analysis_run_v0_3(record, source)
    validate_instance(json.loads((ROOT / "schemas/analysis_run.schema.json").read_text()), record, instance_label=f"{mode} successor record")
    # Back out: the record's receipt authenticates the raw publication again.
    back = deepcopy(source)
    back["retained_precision"] = deepcopy(run["retained_precision"])
    assert rp.validate_retained_precision(back, invocation) == rp.validate_retained_precision(source, invocation)
    # Drop or alter the carried receipt.
    dropped = deepcopy(record)
    del dropped["analysis_run"]["retained_precision"]
    raises(c.ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH, c.validate_analysis_run_v0_3, dropped, source)
    altered = deepcopy(record)
    altered["analysis_run"]["retained_precision"]["body"]["work"]["charged"] = 1
    raises(c.ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH, c.validate_analysis_run_v0_3, altered, source)
    resealed = deepcopy(record)
    resealed["analysis_run"]["retained_precision"]["receipt_sha256"] = "0" * 64
    raises(c.ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH, c.validate_analysis_run_v0_3, resealed, source)
    # RV92 S-1: the copy is compared as canonical bytes, with the record's own
    # checksum resealed, so only the receipt check can refuse. false is not 0 and
    # true is not 1; an integral float is the same JSON value (D25).
    def reseal(index, value):
        rec = deepcopy(record)
        rec["analysis_run"]["retained_precision"]["body"]["builds"][index]["id"] = value
        rec["analysis_run"]["hashes"][0]["value"] = c.canonical_sha256_checked_v1(c.analysis_record_projection(rec))
        assert c.verify_analysis_run_record(rec) == "match"
        return rec
    builds = source["retained_precision"]["body"]["builds"]
    assert [(type(b["id"]), b["id"]) for b in builds[:2]] == [(int, 0), (int, 1)]
    for index, value in ((0, False), (1, True)):
        raises(c.ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH, c.validate_analysis_run_v0_3, reseal(index, value), source)
    for index, value in ((0, 0.0), (1, 1.0)):
        rec = reseal(index, value)
        c.validate_analysis_run_v0_3(rec, source)
        back = dict(source, retained_precision=rec["analysis_run"]["retained_precision"])
        assert rp.validate_retained_precision(back, invocation) == rp.validate_retained_precision(source, invocation)
    # A copy outside the checked JSON profile equals nothing (it cannot be resealed).
    unsafe = deepcopy(record)
    unsafe["analysis_run"]["retained_precision"]["body"]["builds"][0]["id"] = 2 ** 60
    raises(c.ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH, c.validate_analysis_run_v0_3, unsafe, source)
    # A record checked against an edited statement fails at the reader first.
    edited = deepcopy(source)
    edited["retained_precision"]["receipt_sha256"] = "1" * 64
    raises("RETAINED_PRECISION_RECEIPT_MISMATCH", c.validate_analysis_run_v0_3, record, edited)


@pytest.mark.parametrize("mode", MODES)
def test_downgrades_are_refused(mode):
    source, invocation = milestone(mode)
    refs = requested(invocation)
    relabelled = deepcopy(source)
    relabelled["producer"]["semantic_contract_id"] = c.PREVIEW_PHYSICS_CONTRACT_ID
    relabelled["formulation_basis"]["profile_id"] = "product_preview_mechanics_v1"
    raises(c.RETAINED_PRECISION_DOWNGRADE_FORBIDDEN, c._source_contract, relabelled)
    raises(c.RETAINED_PRECISION_DOWNGRADE_FORBIDDEN, build, relabelled)
    assert c.numerical_use_standing(relabelled, refs, invocation) == "unsupported"
    tokens = deepcopy(relabelled)
    del tokens["retained_precision"]
    raises(c.RETAINED_PRECISION_DOWNGRADE_FORBIDDEN, c._source_contract, tokens)
    assert c.numerical_use_standing(tokens, refs) == "unsupported"
    # As Rust's header-only for_source_metadata, the transport dispatch reads no
    # rows; the raw dispatch, which the packager also runs, refuses them.
    assert c._source_contract(tokens, check_receipt=False)[:2] == (c.PREVIEW_PHYSICS_CONTRACT_ID, c.PREVIEW_PHYSICS_CONTRACT_SHA256)
    base = projected(source)
    assert dispatch(base) == "ok"
    null = deepcopy(base)
    null["retained_precision"] = None
    raises(c.RETAINED_PRECISION_DOWNGRADE_FORBIDDEN, c._source_contract, null)
    # RV88 N-3 (Q02): one token on the last row alone is enough; another method string is not a token.
    for index, method in ((-1, c.RETAINED_METHOD), (len(base["results"]) // 2, c.RETAINED_METHOD), (0, "other_method")):
        one = deepcopy(base)
        one["results"][index]["recovery_method"] = method
        assert dispatch(one) == ("ok" if method == "other_method" else c.RETAINED_PRECISION_DOWNGRADE_FORBIDDEN), index
        assert c.numerical_use_standing(one, refs) == ("needs_recompute" if method == "other_method" else "unsupported"), index
    # A base record carrying a receipt is refused; the legacy 0.2 builder refuses one too.
    record = build(base)
    c.validate_analysis_run_v0_3(record, base)
    record["analysis_run"]["retained_precision"] = deepcopy(source["retained_precision"])
    raises(c.ANALYSIS_RETAINED_PRECISION_DOWNGRADE_FORBIDDEN, c.validate_analysis_run_v0_3, record, base)
    legacy = {"schema_version": "0.2.0", "run_id": "run:legacy", "model_ref": base["model_ref"], "results": [], "diagnostics": [],
              "retained_precision": deepcopy(source["retained_precision"])}
    raises("ANALYSIS_LEGACY_SOURCE_DOWNGRADE_FORBIDDEN", lambda s: c.build_analysis_run_v0_2(s, input_manifest_ref={"object_type": "InputManifest", "ref": "m"}, input_manifest_hash="1" * 64), legacy)


@pytest.mark.parametrize("mode", MODES)
def test_binding_refusal_follows_the_validated_class_only(mode):
    source, _ = milestone(mode)
    validation = rp.validate_retained_precision(source)
    rows = {row["id"]: row for row in source["results"]}
    expected = {"absolute_verified": c.RULE_QUANTITY_BELOW_VERIFIED_FLOOR, "not_covered": c.RULE_QUANTITY_NOT_COVERED}
    for item in validation["classifications"]:
        assert c.rule_binding_refusal(source, rows[item["result_id"]]) == expected.get(item["class"]), item["result_id"]
    # The headline maxima: the displacement is relative-verified, the stress absolute-verified.
    headline = {key: c.rule_binding_refusal(source, rows[source["summary"][key]["result_ref"]]) for key in ("max_displacement", "max_open_formula_stress")}
    assert headline == {"max_displacement": None, "max_open_formula_stress": c.RULE_QUANTITY_BELOW_VERIFIED_FLOOR}
    broken = deepcopy(source)
    broken["results"][0]["value"] = 12345.0
    assert all(c.rule_binding_refusal(broken, row) == c.RULE_QUANTITY_NOT_COVERED for row in source["results"])
    # A row the statement does not classify (as in Rust) has no class to refuse.
    assert c.rule_binding_refusal(source, {"id": "result:absent"}) is None
    assert c.rule_binding_refusal(source, {}) is None
    base = projected(source)
    assert all(c.rule_binding_refusal(base, row) is None for row in base["results"])


def test_each_class_maps_to_its_refusal():
    assert c._class_binding_refusal("absolute_verified") == c.RULE_QUANTITY_BELOW_VERIFIED_FLOOR
    assert c._class_binding_refusal("not_covered") == c.RULE_QUANTITY_NOT_COVERED
    for cls in ("relative_verified", "input_derived", "non_quantity", None):
        assert c._class_binding_refusal(cls) is None


@pytest.mark.parametrize("mode", MODES)
def test_classification_summary_counts_validated_classes(mode):
    source, invocation = milestone(mode)
    relative, absolute, input_derived, non_quantity = PINNED[mode][2]
    case_id = invocation["request"]["model"]["load_cases"][0]["id"]
    expected = [{"case_id": case_id, "relative_verified": relative, "absolute_verified": absolute, "interval_bindable": 0,
                 "not_covered": 0, "input_derived": input_derived, "non_quantity": non_quantity,
                 "withheld": relative + absolute + input_derived}]
    current = deepcopy(expected)
    current[0]["withheld"] = absolute
    # U7, aligned (RV94 S-1): Current only when the standing with the caller's requested refs is
    # eligible, so only absolute (and not-covered) rows stay withheld; with no refs or other refs, all.
    assert c.numerical_use_standing(source, requested(invocation), invocation) == "numerically_eligible"
    assert c.classification_summary(source, invocation, requested(invocation)) == current
    assert c.classification_summary(source, invocation) == expected
    other_refs = [{"ref_type": "load_case", "ref_id": "other"}]
    assert c.numerical_use_standing(source, other_refs, invocation) == "needs_recompute"
    assert c.classification_summary(source, invocation, other_refs) == expected
    assert c.classification_summary(source, None, requested(invocation)) == expected
    assert c.classification_summary(source) == expected
    broken = deepcopy(source)
    broken["results"][0]["value"] = 12345.0
    assert c.classification_summary(broken, invocation) == []
    assert c.classification_summary(projected(source)) == []
    # Through the seam: only absolute and not-covered rows stay withheld when Current.
    validation = dict(rp.validate_retained_precision(source, invocation), numerical_eligible=True)
    assert c._classification_summary_from(validation, source, requested(invocation)) == current
    assert c._classification_summary_from(validation, source, []) == expected
    assert c._classification_summary_from(validation, source, other_refs) == expected
    classes = deepcopy(validation["classifications"])
    index = next(i for i, item in enumerate(classes) if item["class"] == "relative_verified")
    classes[index]["class"] = "not_covered"
    uncovered = deepcopy(current)
    uncovered[0].update(relative_verified=relative - 1, not_covered=1, withheld=absolute + 1)
    assert c._classification_summary_from(dict(validation, classifications=classes), source, requested(invocation)) == uncovered
    uncovered[0]["withheld"] = relative + absolute + input_derived
    assert c._classification_summary_from(dict(validation, classifications=classes, numerical_eligible=False), source, requested(invocation)) == uncovered
    # Counts are per case: a classification under another case is not counted here.
    stray = dict(classes[index], basis_ref={"ref_type": "load_case", "ref_id": "other"})
    assert c._classification_summary_from(dict(validation, classifications=classes + [stray], numerical_eligible=False), source, requested(invocation)) == uncovered


def test_d_u7_4_forms_python_side_standing_and_summary():
    """RV94 S-1: on both D-U7-4 forms Python, given the invocation and the requested refs, stands
    numerically_eligible and its summary is Current (69 withheld). TS's side (needs_recompute, the
    not-Current 97) is pinned in TS. With other requested refs Python reads needs_recompute and the
    not-Current summary, so the summary never says Current when the standing does not."""
    cases, docs = shared_cases()
    entry = next(e for e in cases["declared_differences"] if e["id"] == "D-U7-4:ts_requires_live_native_capture")
    for form in entry["forms"]:
        assert form["expected"]["python"] == ({"standing": "numerically_eligible"} if form["subject"] == "standing" else {"summary": "by_validated_class_current"}), form["label"]
        for fid in form["fixtures"]:
            source, context, refs = apply_case(form, docs[fid])
            assert c.numerical_use_standing(source, refs, context) == "numerically_eligible", form["label"]
            summary = c.classification_summary(source, context, refs)
            assert summary == expected_summary(source, current=True) and summary[0]["withheld"] == summary[0]["absolute_verified"] == 69, form["label"]
            other = [{"ref_type": "load_case", "ref_id": "other"}]
            assert c.numerical_use_standing(source, other, context) == "needs_recompute"
            assert c.classification_summary(source, context, other) == expected_summary(source) and summary != expected_summary(source)


def test_07j_not_required_case_omitted_or_reordered_is_not_current():
    """RV94 S-1 on 07j's two-case statement whose second case is not_required: with the receipt's
    case order the standing is eligible and the summary Current; with the not_required case omitted,
    or the two reordered, the standing is needs_recompute and the summary not-Current ([73, 0])."""
    from tests.test_retained_precision_contract import apply_entry, corpus
    data = corpus()
    entry = next(e for e in data["must_pass"] if e["id"] == "not_required_second_case_checks_passed")
    fixture = next(f for f in data["cases"] if f["id"] == entry["base"])
    source, invocation = apply_entry(fixture, entry)
    order = [case["basis_ref"] for case in source["retained_precision"]["body"]["cases"]]
    assert [case["status"] for case in source["retained_precision"]["body"]["cases"]] == ["selected", "not_required"]
    assert c.numerical_use_standing(source, order, invocation) == "numerically_eligible"
    current = c.classification_summary(source, invocation, order)
    not_current = c.classification_summary(source, invocation)
    assert [case["withheld"] for case in not_current] == [73, 0] and current != not_current
    assert current == expected_summary(source, current=True) and not_current == expected_summary(source)
    for refs in (order[:1], order[::-1]):
        assert c.numerical_use_standing(source, refs, invocation) == "needs_recompute", refs
        assert c.classification_summary(source, invocation, refs) == not_current, refs


@pytest.mark.parametrize("mode", MODES)
def test_stress_neutral_packager_still_refuses_the_successor(mode):
    """T6's refusal stands (D2 4.9.6; C1:162): schema support is shape only."""
    from core.handoff.stress_neutral import package_v0_3 as sn
    from tests.test_stress_neutral_physics_source import arguments
    source, _ = milestone(mode)
    record = build(source)
    raises("SN-SOURCE-METHOD-UNSUPPORTED", lambda: sn.build_stress_neutral_export_package_v0_3(source_envelope=source, analysis_record=record, **arguments(source, record)))
    base = projected(source)
    base_record = build(base)
    packet = sn.build_stress_neutral_export_package_v0_3(source_envelope=base, analysis_record=base_record, **arguments(base, base_record))
    packet.update(producer=dict(packet["producer"], semantic_contract_id=SUCC), retained_precision=deepcopy(source["retained_precision"]),
                  semantic_contract={"id": SUCC, "sha256": c.PREVIEW_PHYSICS_RETAINED_CONTRACT_SHA256})
    raises("SOURCE_PRODUCER_CONTRACT_UNSUPPORTED", sn.validate_stress_neutral_export_package_v0_3, packet)


@pytest.mark.parametrize("mode", MODES)
def test_legacy_record_wrapper_refuses_a_receipt(mode):
    """D-U6-9: the historical 0.1.0 wrapper cannot carry a receipt."""
    from core.analysis_runs.records import build_preview_analysis_run_envelope
    source, _ = milestone(mode)
    raises("ANALYSIS_RETAINED_PRECISION_DOWNGRADE_FORBIDDEN", build_preview_analysis_run_envelope, source)
    # The wrapper's own preview input still builds; the same input carrying a
    # receipt member (even an empty one) is refused before anything is recorded.
    preview = json.loads((ROOT / "fixtures/product_preview/invented_mechanics_result.json").read_text())
    assert build_preview_analysis_run_envelope(preview)["schema_version"] == "0.1.0"
    for receipt in (deepcopy(source["retained_precision"]), None):
        raises("ANALYSIS_RETAINED_PRECISION_DOWNGRADE_FORBIDDEN", build_preview_analysis_run_envelope, dict(preview, retained_precision=receipt))


def test_legacy_sources_with_a_token_row_are_refused_on_every_python_path():
    """RV88 U6b S-1: a legacy 0.1.0 source with the W1 token on one row (the last)
    is refused by the raw dispatch, standing, the AnalysisRun router, the explicit
    0.2 constructor and the 0.1.0 wrapper, as Rust and TS refuse it. The header-only
    transport dispatch reads no rows, as Rust's for_source_metadata."""
    from core.analysis_runs.records import build_preview_analysis_run_envelope
    legacy = json.loads((ROOT / "fixtures/product_preview/invented_mechanics_result.json").read_text())
    assert legacy["schema_version"] == "0.1.0"
    manifest = {"input_manifest_ref": {"object_type": "InputManifest", "ref": "manifest:u6b"}, "input_manifest_hash": "1" * 64}
    for method, refused in ((c.RETAINED_METHOD, True), ("other_method", False)):
        token = deepcopy(legacy)
        token["results"][-1]["recovery_method"] = method
        if refused:
            raises(c.RETAINED_PRECISION_DOWNGRADE_FORBIDDEN, c._source_contract, token)
            assert c.numerical_use_standing(token, []) == "unsupported"
            raises(c.RETAINED_PRECISION_DOWNGRADE_FORBIDDEN, lambda s: c.build_analysis_run(s, **manifest), token)
            raises("ANALYSIS_LEGACY_SOURCE_DOWNGRADE_FORBIDDEN", lambda s: c.build_analysis_run_v0_2(s, **manifest), token)
            raises("ANALYSIS_RETAINED_PRECISION_DOWNGRADE_FORBIDDEN", build_preview_analysis_run_envelope, token)
        else:
            assert c._source_contract(token)[0] == c.SEMANTIC_CONTRACT_ID
            assert c.numerical_use_standing(token, []) == "needs_recompute"
            assert c.build_analysis_run(token, **manifest)["schema_version"] == "0.2.0"
            assert c.build_analysis_run_v0_2(token, **manifest)["schema_version"] == "0.2.0"
            assert build_preview_analysis_run_envelope(token)["schema_version"] == "0.1.0"
        assert c._source_contract(token, check_receipt=False)[0] == c.SEMANTIC_CONTRACT_ID
    # A legacy receipt member, empty or null, is refused before the version branch.
    for receipt in ({}, None):
        raises(c.RETAINED_PRECISION_DOWNGRADE_FORBIDDEN, c._source_contract, dict(legacy, retained_precision=receipt))


def test_private_seams_have_no_product_callers():
    """As Rust's #[doc(hidden)] guard (RV88 U6a N-3): the eligibility-forcing seams
    are used by compatibility.py itself and by tests only."""
    seams = {"_retained_standing_from": 3, "_classification_summary_from": 2, "_class_binding_refusal": 2}
    files = [path for path in (ROOT / "core").rglob("*.py")] + [path for path in (ROOT / "tools").rglob("*.py")]
    assert len(files) > 20
    for seam, uses in seams.items():
        for path in files:
            count = path.read_text(encoding="utf-8").count(f"{seam}(")
            expected = uses if path == ROOT / "core/analysis_runs/compatibility.py" else 0
            assert count == expected, (seam, str(path.relative_to(ROOT)))


@pytest.mark.parametrize("mode", MODES)
def test_r2_noticed_ordinary_envelope_keeps_its_base_behaviour(mode):
    """R-2: a base publication carrying the unavailable notice is an ordinary
    preview-physics-1 envelope; its carriers and standing equal the plain base."""
    source, invocation = milestone(mode)
    plain = projected(source)
    case_id = invocation["request"]["model"]["load_cases"][0]["id"]
    noticed = deepcopy(plain)
    noticed["diagnostics"].append({"id": f"diagnostic:retained-precision:{case_id}:unavailable", "code": "RETAINED_PRECISION_UNAVAILABLE",
                                   "severity": "info", "message": NOTICE, "source": "core/product_physics", "affected_refs": [case_id]})
    refs = requested(invocation)
    assert c._source_contract(noticed)[:2] == c._source_contract(plain)[:2] == (c.PREVIEW_PHYSICS_CONTRACT_ID, c.PREVIEW_PHYSICS_CONTRACT_SHA256)
    assert c.numerical_use_standing(noticed, refs) == c.numerical_use_standing(plain, refs)
    record = build(noticed)
    c.validate_analysis_run_v0_3(record, noticed)
    assert record["analysis_run"]["diagnostics"][-1]["source_annotation"]["code"] == "RETAINED_PRECISION_UNAVAILABLE"
    assert "retained_precision" not in record["analysis_run"]
    assert all(c.rule_binding_refusal(noticed, row) is None for row in noticed["results"])
