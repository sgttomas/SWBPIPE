#!/usr/bin/env python3
"""The DEL-08-06 comparison-section parity vector (DEL-08-06 AC-002, VER-002).

Every expected section set is computed by the reference assembler,
`core/reporting/state_comparison_handoff_sections/engine.py`
(`build_state_comparison_handoff_report_sections`), from invented saved-state
and run comparison records in the form the desktop writes them
(`apps/desktop/src/features/comparison/comparisonRecords.ts`) and in the
reference comparison engines' forms. The desktop assembler
(`apps/desktop/src/features/report/comparisonSections.ts`) must give the same
set, compared in RFC 8785 form, by
`apps/desktop/src/features/report/comparisonSections.parity.test.ts`;
`tests/test_state_comparison_handoff_report_sections.py` checks the fixture
against this writer. All records are invented and hold no engineering data.

Usage:
    python3 tools/fixtures/state_comparison_handoff_comparison_parity.py          # write the fixture
    python3 tools/fixtures/state_comparison_handoff_comparison_parity.py --check  # compare, write nothing
"""
from __future__ import annotations

from copy import deepcopy
import json
import sys
from pathlib import Path

sys.dont_write_bytecode = True
PROJECT = Path(__file__).resolve().parents[2]
if str(PROJECT) not in sys.path:
    sys.path.insert(0, str(PROJECT))
from core.reporting.state_comparison_handoff_sections import (  # noqa: E402
    build_state_comparison_handoff_report_sections,
)

FIXTURE = PROJECT / "fixtures/reports/invented/state_comparison_handoff_comparison_parity.json"

BOUNDARY = {
    "human_review_required": True,
    "software_makes_compliance_claim": False,
    "software_makes_certification_claim": False,
    "software_makes_sealing_claim": False,
    "software_makes_approval_claim": False,
    "software_makes_authentication_claim": False,
    "software_creates_professional_reliance_record": False,
    "software_creates_external_validation_record": False,
}

RECORD_PROVENANCE = {
    "source_name": "Invented comparison parity record",
    "source_location": "tools/fixtures/state_comparison_handoff_comparison_parity.py",
    "source_license": "invented-non-engineering",
    "contributor": "test fixture",
    "contributor_attestation": "not_asserted",
    "redistribution_status": "invented_non_engineering_example",
    "review_status": "pending",
    "privacy_classification": "invented_public_example",
}

SET_PROVENANCE = {
    "source_name": "Invented comparison parity fixture",
    "source_location": "fixtures/reports/invented/state_comparison_handoff_comparison_parity.json",
    "source_license": "invented-non-engineering",
    "contributor": "test fixture",
    "contributor_attestation": "not_asserted",
    "redistribution_status": "invented_non_engineering_example",
    "review_status": "pending",
    "privacy_classification": "invented_public_example",
}


def record_hash(kind: str, comparison_id: str, digit: str) -> list[dict]:
    return [{
        "algorithm": "sha256",
        "canonicalization": "rfc8785_jcs",
        "payload_ref": {"object_type": kind, "ref": comparison_id},
        "payload_scope": "comparison_record_without_hashes",
        "value": digit * 64,
    }]


def state_record(comparison_id: str = "model-state-comparison:state:inv-a:state:inv-b", entity_id: str = "node:inv-2") -> dict:
    """Every classification: the desktop's added, removed, changed and unchanged, and the
    reference engine's mapped_changed, mapped_unchanged and unresolved rows."""

    def ref(family: str, value: str) -> dict:
        return {"object_type": family, "ref": value}

    entities = [
        {"row_id": "row:node:inv-1", "classification": "added", "match_basis": "unmatched", "mapping_id": None,
         "left_ref": None, "right_ref": ref("node", "node:inv-1"), "changes": []},
        {"row_id": "row:support:inv-9", "classification": "removed", "match_basis": "unmatched", "mapping_id": None,
         "left_ref": ref("support", "support:inv-9"), "right_ref": None, "changes": []},
        {"row_id": f"row:node:{entity_id}", "classification": "changed", "match_basis": "stable_id", "mapping_id": None,
         "left_ref": ref("node", entity_id), "right_ref": ref("node", entity_id), "changes": [
             {"path": "position", "change_kind": "unit_changed", "left_unit": "m", "right_unit": "mm"},
             {"path": "label", "change_kind": "changed"},
             {"path": "restraints[0]", "change_kind": "added"},
         ]},
        {"row_id": "row:material:inv-3", "classification": "unchanged", "match_basis": "stable_id", "mapping_id": None,
         "left_ref": ref("material", "material:inv-3"), "right_ref": ref("material", "material:inv-3"), "changes": []},
        {"row_id": "row:map-1", "classification": "mapped_changed", "match_basis": "explicit_mapping", "mapping_id": "map-1",
         "left_ref": ref("Entity", "seg:inv-old"), "right_ref": ref("Entity", "seg:inv-new"),
         "changes": [{"field": "length", "left": {"value": 2.0, "unit": "m"}, "right": {"value": 2.5, "unit": "m"}}]},
        {"row_id": "row:map-2", "classification": "mapped_unchanged", "match_basis": "manual_mapping", "mapping_id": "map-2",
         "left_ref": ref("Entity", "seg:inv-p"), "right_ref": ref("Entity", "seg:inv-q"), "changes": []},
        {"row_id": "row:map-3", "classification": "unresolved", "match_basis": "explicit_mapping", "mapping_id": "map-3",
         "left_ref": ref("Entity", "seg:inv-gone"), "right_ref": None, "changes": []},
    ]
    return {
        "comparison_id": comparison_id,
        "comparison_kind": "model_state_to_model_state",
        "left_state_ref": {"object_type": "ModelState", "ref": "state:inv-a", "model_hash": "sha256:" + "a" * 64},
        "right_state_ref": {"object_type": "ModelState", "ref": "state:inv-b", "model_hash": "sha256:" + "b" * 64},
        "summary": {"added": 1, "removed": 1, "changed": 1, "unchanged": 1, "mapped_changed": 1,
                    "mapped_unchanged": 1, "unresolved": 1, "total": 7, "unit_changed_fields": 1},
        "entity_counts": {"left": 6, "right": 6},
        "entities": entities,
        "settings": {"mapping_basis": "stable_id_within_family", "manual_mappings": "none",
                     "unit_conversion": "none", "tolerance": "not_tolerance_checked"},
        "diagnostics": [],
        "privacy_classification": "invented_public_example",
        "review_status": "pending",
        "professional_boundary": deepcopy(BOUNDARY),
        "provenance": deepcopy(RECORD_PROVENANCE),
        "hashes": record_hash("ModelStateComparison", comparison_id, "c"),
    }


NOT_COMPARED_REASONS = [
    "semantic_identity_differs",
    "left_run_needs_recompute",
    "right_run_needs_recompute",
    "family_not_compared",
    "units_differ",
    "value_not_numeric",
    "dimension_missing",
]

UNMATCHED_REASONS = ["no_row_with_this_result_id", "identity_differs", "duplicate_result_id", "result_id_missing"]


def identity(result_id: str, family: str | None = "force", dimension: str | None = "force") -> dict:
    return {"result_id": result_id, "family": family, "object_ref": "element:inv-1", "basis_ref": None,
            "dimension": dimension, "component": "fx", "location": "end_i", "basis": None,
            "coordinate_system": "local", "sign_convention": None}


def delta(result_id: str, left, right, difference, unit: str = "N", dimension: str | None = "force", basis: str = "stable_result_identity") -> dict:
    row = {
        "mapping_id": f"auto:{result_id}", "match_basis": basis, "left_result_id": result_id, "right_result_id": result_id,
        "result_family": "force", "identity": identity(result_id, dimension=dimension),
        "normalized_unit": unit, "left_magnitude": left, "right_magnitude": right, "normalized_delta": difference,
        "tolerance_profile_ref": None, "classification": "not_tolerance_checked", "classification_basis": "no_tolerance_profile",
    }
    if dimension is not None:
        row["dimension"] = dimension
    return row


def run_record(comparison_id: str = "analysis-run-comparison:runrec:inv-1:current-run", deltas: list[dict] | None = None) -> dict:
    """Compared pairs, every not-compared reason, unmatched rows on both sides with every
    reason, the run gate, settings deltas and diagnostic changes."""

    return {
        "comparison_id": comparison_id,
        "comparison_kind": "analysis_run_to_analysis_run",
        "run_context": {
            "left": {"object_type": "AnalysisRun", "ref": "runrec:inv-1", "side_kind": "recorded_run",
                     "run_record_hash": "sha256:" + "d" * 64, "result_hash": "sha256:" + "e" * 64},
            "right": {"object_type": "AnalysisRun", "ref": "current-run", "side_kind": "current_session_run",
                      "run_record_hash": "not_recorded", "result_hash": "sha256:" + "f" * 64},
            "run_gate": [],
            "summary": {"mapped_pairs": 9, "compared": 2, "compared_nonzero": 1, "not_compared": 7,
                        "unmatched_left": 4, "unmatched_right": 3, "context_deltas": 2,
                        "diagnostics_added": 1, "diagnostics_removed": 1, "diagnostics_common": 0},
            "not_compared": [
                {"result_id": f"result:inv-nc-{index}", "identity": identity(f"result:inv-nc-{index}", dimension=None if reason == "dimension_missing" else "force"),
                 "left_unit": "N", "right_unit": "kN" if reason == "units_differ" else "N", "reasons": [reason]}
                for index, reason in enumerate(NOT_COMPARED_REASONS)
            ],
            "unmatched": [
                *[{"side": "left", "result_id": f"result:inv-um-{index}", "identity": identity(f"result:inv-um-{index}"),
                   "unit": "N", "reason": reason, "differing_fields": ["object_ref"] if reason == "identity_differs" else []}
                  for index, reason in enumerate(UNMATCHED_REASONS)],
                *[{"side": "right", "result_id": f"result:inv-um-{index}", "identity": identity(f"result:inv-um-{index}"),
                   "unit": "N", "reason": reason, "differing_fields": ["object_ref"] if reason == "identity_differs" else []}
                  for index, reason in enumerate(UNMATCHED_REASONS[1:], start=1)],
            ],
            "tolerance": "not_tolerance_checked",
        },
        "result_deltas": deltas if deltas is not None else [
            delta("result:inv-c-1", 10, 12.5, 2.5),
            delta("result:inv-c-2", -3.25, -3.25, 0),
        ],
        "settings_deltas": [
            {"setting_key": "settings.sparse_evidence_lane", "left_value": True, "right_value": False, "delta_kind": "changed"},
            {"setting_key": "load_case:case:inv-added", "left_value": None, "right_value": True, "delta_kind": "added"},
        ],
        "diagnostics": [
            {"code": "INV-DIAGNOSTIC-A", "severity": "warning", "affected_refs": ["node:inv-1"], "message": "Invented solve diagnostic.", "change": "removed"},
            {"code": "INV-DIAGNOSTIC-B", "severity": None, "affected_refs": [], "message": None, "change": "added"},
        ],
        "privacy_classification": "invented_public_example",
        "review_status": "pending",
        "professional_boundary": deepcopy(BOUNDARY),
        "provenance": deepcopy(RECORD_PROVENANCE),
        "hashes": record_hash("AnalysisRunComparison", comparison_id, "0"),
    }


def inputs(states: list[dict], runs: list[dict], set_id: str) -> dict:
    return {
        "section_set_id": set_id,
        "model_states": [],
        "analysis_runs": [],
        "model_state_comparisons": states,
        "analysis_run_comparisons": runs,
        "handoff_packages": [],
        "export_workflows": [],
        "external_prover_metadata": [],
        "source_notes": ["Invented comparison parity fixture; no engineering data."],
        "provenance": deepcopy(SET_PROVENANCE),
    }


def cases() -> list[dict]:
    second_state = state_record("model-state-comparison:state:inv-b:state:inv-c")
    second_run = run_record("analysis-run-comparison:runrec:inv-1:runrec:inv-2")
    number_forms = run_record("analysis-run-comparison:runrec:inv-3:runrec:inv-4", deltas=[
        delta("result:inv-n-1", 1e-05, 2.0, 1.99999),
        delta("result:inv-n-2", 1e16, 1.5e16, 5e15),
        delta("result:inv-n-3", 1.5e-07, 123456789.125, 123456789.12499985),
        delta("result:inv-n-4", -0.0, 1e21, 1e21),
        delta("result:inv-n-5", 0.1, 0.30000000000000004, 0.20000000000000004),
        delta("result:inv-n-6", 0.0001, 1e-4, 0.0, basis="manual_mapping"),
    ])
    sparse_state = {"comparison_id": "model-state-comparison:state:inv-x:state:inv-y"}
    return [
        {"name": "state_classifications",
         "description": "A saved-state comparison with every classification: added, removed, changed (with a unit change) and unchanged as the desktop writes them, and mapped_changed, mapped_unchanged (manual) and unresolved as the reference engine writes them.",
         "inputs": inputs([state_record()], [], "comparison-parity:state")},
        {"name": "run_reasons_and_unmatched",
         "description": "A run comparison with compared pairs, every not-compared reason (no number), unmatched rows on both sides with every reason, settings deltas and diagnostic changes.",
         "inputs": inputs([], [run_record()], "comparison-parity:run")},
        {"name": "delta_without_dimension",
         "description": "A run delta with no dimension: the set carries it with the blocking unit diagnostics and not as a bare number.",
         "inputs": inputs([], [run_record(deltas=[delta("result:inv-nd-1", 1.0, 3.0, 2.0, dimension=None)])], "comparison-parity:no-dimension")},
        {"name": "authority_term_in_identifier",
         "description": "An entity identifier that holds an authority term as a substring: the reference rejects the source text with a blocking diagnostic.",
         "inputs": inputs([state_record(entity_id="node:inv-seal-2")], [], "comparison-parity:authority")},
        {"name": "both_kinds",
         "description": "Two saved-state and two run comparisons.",
         "inputs": inputs([state_record(), second_state], [run_record(), second_run], "comparison-parity:both")},
        {"name": "both_kinds_permuted",
         "description": "The records of both_kinds supplied in the other order: the set is the same.",
         "inputs": inputs([second_state, state_record()], [second_run, run_record()], "comparison-parity:both")},
        {"name": "number_forms",
         "description": "Values whose Python and JavaScript JSON texts differ (1e-05, 2.0, 1e+16, 1.5e-07, -0.0, 1e+21, 0.0001) and a manual mapping.",
         "inputs": inputs([], [number_forms], "comparison-parity:numbers")},
        {"name": "sparse_state_record",
         "description": "A saved-state record holding only its identifier: its unavailable values are stated as unresolved and its missing boundary as a warning.",
         "inputs": inputs([sparse_state], [], "comparison-parity:sparse")},
    ]


# Non-integral floats and the reference's canonical text of each, which the desktop's
# ordering key (`pythonJson`) must reproduce; and strings that `ensure_ascii` escapes.
NUMBER_TEXT_VALUES = [1e-05, 1.5e-07, 0.0001, 0.30000000000000004, 123456789.125, -2.5e-300,
                      5e-324, 1250000000000000.5, 0.1, -0.000123, 4.35, 1e-04 * 3]
STRING_TEXT_VALUES = ["plain", "Zürich Ω 配管 😀", "quote \" and \\", "\u0001\u001f\u007f\b\f\n\r\t", "\u2028"]


def texts() -> list[list]:
    return [[value, json.dumps(value, ensure_ascii=True)] for value in [*NUMBER_TEXT_VALUES, *STRING_TEXT_VALUES]]


def fixture() -> dict:
    built = []
    for case in cases():
        built.append({**case, "expected": build_state_comparison_handoff_report_sections(**deepcopy(case["inputs"]))})
    return {
        "description": "DEL-08-06 comparison-section parity vector: each case's expected section set is the reference assembler's (core/reporting/state_comparison_handoff_sections/engine.py) for its invented comparison records. Written by tools/fixtures/state_comparison_handoff_comparison_parity.py; never edited by hand.",
        "provenance": deepcopy(SET_PROVENANCE),
        "cases": built,
        "canonical_texts": texts(),
    }


def text() -> str:
    return json.dumps(fixture(), indent=1, sort_keys=True) + "\n"


def main(argv: list[str]) -> int:
    rendered = text()
    if "--check" in argv:
        current = FIXTURE.read_text(encoding="utf-8") if FIXTURE.exists() else ""
        if current != rendered:
            print(f"{FIXTURE.relative_to(PROJECT)} is stale; regenerate with {Path(__file__).relative_to(PROJECT)}", file=sys.stderr)
            return 1
        return 0
    FIXTURE.write_text(rendered, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
