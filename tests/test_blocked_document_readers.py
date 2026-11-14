"""DEL-04-07, the Python twins: a blocked document carrying passed verdicts, old form and new,
is never eligible, and a solved document carrying an inspection-only evaluation is refused
by name. The documents are built from a committed solved result, as the Rust test
core/reporting/result_export/tests/blocked_document_verdicts.rs builds them."""
from __future__ import annotations

import copy
import json
from pathlib import Path

from tests.verifiers import compatibility as c
from tests.verifiers import reader_verdict as rv

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
