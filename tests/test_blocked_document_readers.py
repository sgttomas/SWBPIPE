"""DEL-04-07, the Python twins: a blocked document carrying passed verdicts, old form and new,
is never eligible, and a solved document carrying an inspection-only evaluation is refused
by name. The documents are built from a committed solved result, as the Rust test
core/reporting/result_export/tests/blocked_document_verdicts.rs builds them."""
from __future__ import annotations

import copy
import json
from pathlib import Path

import pytest

from tests.verifiers import compatibility as c
from tests.verifiers import physics_source as ps
from tests.verifiers import reader_verdict as rv
from tests.verifiers import retained_precision as rp
from tests.verifiers.source_blocks import domain_hash

ROOT = Path(__file__).resolve().parents[1]
LEAD = "Inspection only (prior_outcome="
TAIL = "): this document is blocked (MODEL_INCOMPLETE); no result of this case is published, and this evaluation is retained for inspection, not as a verdict on a published result. "


def solved() -> dict:
    return json.loads((ROOT / "fixtures/results/preview_physics_invented_sparse.json").read_text())


def bases(source: dict) -> list:
    return [case["basis_ref"] for case in source["numerical_quality"]["cases"]]


def old_blocked() -> dict:
    source = solved()
    blocked = copy.deepcopy(source)
    blocked["status"]["mechanics"] = "MODEL_INCOMPLETE"
    blocked["results"] = []
    blocked["summary"]["max_open_formula_stress"] = None
    blocked["summary"]["max_displacement"] = None
    blocked["summary"]["component_stress_modifier_count"] = 0
    blocked["contract_evidence"] = {"preview_cases": [], "combination_gates": []}
    blocked["diagnostics"] = [d for d in source["diagnostics"] if d["id"].startswith("diagnostic:numerical-integrity:")] + [
        {"id": "diagnostic:preview-physics:joint-equilibrium:13:component:C-1", "code": "JOINT_ELEMENT_EQUILIBRIUM_UNQUALIFIED", "severity": "blocking",
         "message": "hand-built blocked control", "source": "core/product_physics", "affected_refs": ["component:C-1", "pipe:P-1"]}]
    return blocked


def new_blocked() -> dict:
    blocked = old_blocked()
    for d in blocked["diagnostics"]:
        if d["code"] in {"NUMERICAL_INTEGRITY_CHECKS_PASSED", "NUMERICAL_INTEGRITY_SENSITIVE"}:
            d["message"] = f"{LEAD}{d['code']}{TAIL}{d['message']}"
            d["code"] = c.INSPECTION_ONLY_CODE
            d["severity"] = "info"
    blocked["numerical_quality"]["status"] = "not_assessed"
    for case in blocked["numerical_quality"]["cases"]:
        case.update(structural_status="numerically_unresolved", solve_quality="not_assessed", model_matrix_fidelity="not_assessed", accuracy_evidence="not_claimed")
    return blocked


def test_a_blocked_document_with_passed_verdicts_is_never_eligible_old_or_new():
    for label, source in (("old", old_blocked()), ("new", new_blocked())):
        assert c.numerical_use_standing(source, bases(source)) == "needs_recompute", label
        standing = rv.standing(source)
        assert standing.get("eligible") is not True, (label, standing)
        assert "MECHANICS_NOT_SOLVED" in standing.get("findings", []), (label, standing)
        assert "INTEGRITY_INSPECTION_ONLY_IN_SOLVED_DOCUMENT" not in standing.get("findings", []), label


def test_a_solved_document_carrying_an_inspection_only_evaluation_is_refused_by_name():
    source = solved()
    assert c.numerical_use_standing(source, bases(source)) == "numerically_eligible", "the control"
    target = next(d for d in source["diagnostics"] if d["code"] == "NUMERICAL_INTEGRITY_CHECKS_PASSED")
    target["message"] = f"{LEAD}NUMERICAL_INTEGRITY_CHECKS_PASSED{TAIL}{target['message']}"
    target["code"] = c.INSPECTION_ONLY_CODE
    target["severity"] = "info"
    assert c.numerical_use_standing(source, bases(source)) == "unsupported"
    standing = rv.standing(source)
    assert standing.get("eligible") is not True, standing
    assert "INTEGRITY_INSPECTION_ONLY_IN_SOLVED_DOCUMENT" in standing.get("findings", []), standing


def inspect(source: dict) -> int:
    """Rewrite every passed or sensitive integrity diagnostic as blocked_envelope writes it."""
    count = 0
    for d in source["diagnostics"]:
        if d["code"] in ("NUMERICAL_INTEGRITY_CHECKS_PASSED", "NUMERICAL_INTEGRITY_SENSITIVE"):
            d["message"] = f"{LEAD}{d['code']}{TAIL}{d['message']}"
            d["code"], d["severity"] = c.INSPECTION_ONLY_CODE, "info"
            count += 1
    return count


def not_assessed(source: dict) -> dict:
    source["status"]["mechanics"] = "MODEL_INCOMPLETE"
    source["numerical_quality"]["status"] = "not_assessed"
    for case in source["numerical_quality"]["cases"]:
        case["solve_quality"] = "not_assessed"
    return source


def reseal(source: dict) -> dict:
    """The source-block publication and receipt hashes after an edit (the Rust test's reseal)."""
    body = source["source_block_recovery"]["body"]
    body["publication_sha256"] = domain_hash("source_blocks_publication_v1", {k: v for k, v in source.items() if k != "source_block_recovery"})
    source["source_block_recovery"]["receipt_sha256"] = domain_hash("source_blocks_receipt_v1", body)
    return source


def test_the_source_block_reader_never_reads_an_inspection_only_evaluation():
    folder = ROOT / "fixtures/product_preview/pressure_source/physics_source"
    doc = json.loads((folder / "mixed-sparse_interactive.raw.json").read_text())
    invocation = {"request": json.loads((folder / "mixed.request.json").read_text()), "solver_mode": "sparse_interactive"}
    refs = bases(doc)
    assert ps.validate_physics_source(doc, invocation) is True, "the control"
    assert reseal(copy.deepcopy(doc)) == doc
    named = copy.deepcopy(doc)
    assert inspect(named) == 2
    named = reseal(named)
    with pytest.raises(ValueError, match="^SOURCE_BLOCKS_INSPECTION_ONLY_IN_SOLVED_DOCUMENT$"):
        ps.validate_physics_source(named, invocation)
    assert c.numerical_use_standing(named, refs, invocation) == "unsupported"
    old = copy.deepcopy(doc)
    old["status"]["mechanics"] = "MODEL_INCOMPLETE"
    old = reseal(old)
    assert ps.validate_physics_source(old, invocation) is False
    assert c.numerical_use_standing(old, refs, invocation) == "needs_recompute"
    new = copy.deepcopy(doc)
    inspect(new)
    new = reseal(not_assessed(new))
    with pytest.raises(ValueError, match="^SOURCE_BLOCKS_ORDINARY_REPORT$"):
        ps.validate_physics_source(new, invocation)
    assert c.numerical_use_standing(new, refs, invocation) != "numerically_eligible"
    # reader_verdict, the unpinned dispatcher (the Rust twin).
    model = invocation["request"]["model"]
    assert rv.standing(doc, model, invocation)["eligible"] is True, "the control"
    assert rv.standing(named, model, invocation)["eligible"] is not True
    for form in (old, new):
        standing = rv.standing(form, model, invocation)
        assert standing["eligible"] is not True and "MECHANICS_NOT_SOLVED" in standing["findings"], standing
        assert "INTEGRITY_INSPECTION_ONLY_IN_SOLVED_DOCUMENT" not in standing["findings"], standing


@pytest.mark.parametrize("mode", ["sparse_interactive", "dense_scrutiny"])
def test_the_retained_reader_never_reads_an_inspection_only_evaluation(mode):
    def rehash(source):
        body = source["retained_precision"]["body"]
        body["publication_sha256"] = rp._hash("retained_precision_publication_mp_v2", {k: v for k, v in source.items() if k != "retained_precision"})
        source["retained_precision"]["receipt_sha256"] = rp._hash("retained_precision_receipt_mp_v2", body)
        return source
    doc = json.loads((ROOT / f"fixtures/results/retained_precision_milestone_successor_{mode}.json").read_text())
    source, invocation = doc["source"], doc["invocation"]
    refs = [{"ref_type": "load_case", "ref_id": case["id"]} for case in invocation["request"]["model"]["load_cases"]]
    assert c.numerical_use_standing(source, refs, invocation) == "numerically_eligible", "the control"
    named = copy.deepcopy(source)
    assert inspect(named) == 1
    named = rehash(named)
    with pytest.raises(rp.RetainedPrecisionError) as refused:
        rp.validate_retained_precision(named, invocation)
    assert (refused.value.gate, refused.value.code) == ("G4", "RETAINED_PRECISION_INSPECTION_ONLY_IN_SOLVED_DOCUMENT")
    assert c.numerical_use_standing(named, refs, invocation) == "unsupported"
    blocked = rehash(not_assessed(copy.deepcopy(named)))
    with pytest.raises(rp.RetainedPrecisionError) as refused:
        rp.validate_retained_precision(blocked, invocation)
    assert (refused.value.gate, refused.value.code) == ("G5", "RETAINED_PRECISION_ATTEMPT_MISMATCH")
    assert c.numerical_use_standing(blocked, refs, invocation) == "unsupported"
    # reader_verdict on the retained route reads through retained_precision, a protected reader (the Rust twin): the
    # blocked form is refused by that reader's own gate, never named as solved.
    model = invocation["request"]["model"]
    assert rv.standing(source, model, invocation)["eligible"] is True, "the control"
    assert rv.standing(named, model, invocation)["eligible"] is not True
    standing = rv.standing(blocked, model, invocation)
    assert standing["eligible"] is not True and standing["findings"] == ["RETAINED_PRECISION_ATTEMPT_MISMATCH"], standing
