#!/usr/bin/env python3
"""Focused tests for DEL-14-04 analysis-run comparison."""

import sys
from copy import deepcopy
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.comparison.analysis_run.engine import compare_analysis_runs, comparison_dict


FORBIDDEN_CLAIMS = {
    "certification",
    "certified",
    "sealing",
    "sealed",
    "authentication",
    "code " + "compliant",
    "professional " + "approval",
    "engineering " + "acceptance",
}


def ref(object_type, value):
    return {"object_type": object_type, "ref": value}


def run_record(run_id, model_state_id, result_ref):
    return {
        "run_id": run_id,
        "run_name": f"{run_id} invented fixture",
        "run_kind": "mechanics_solve",
        "created_at": "2026-05-05T00:00:00Z",
        "model_state_ref": ref("ModelState", model_state_id),
        "solver_version": {
            "solver_id": "invented-solver",
            "version": "0.0.0-fixture",
            "build_hash": "invented",
        },
        "settings_ref": ref("SolverSettings", f"settings:{run_id}"),
        "unit_system_ref": ref("UnitSystem", "fixture-units"),
        "load_basis_refs": [ref("LoadCase", "LC-1")],
        "diagnostics": [],
        "result_refs": [ref("ResultEnvelope", result_ref)],
        "rule_pack_refs": [ref("RulePack", "RP-fixture")],
        "library_refs": [ref("Library", "LIB-fixture")],
        "hashes": [
            {
                "algorithm": "sha256",
                "canonicalization": "JCS",
                "payload_ref": ref("AnalysisRun", run_id),
                "payload_scope": "analysis_run_record",
                "value": f"hash-{run_id}",
            }
        ],
        "analysis_status": ["MECHANICS_SOLVED", "HUMAN_REVIEW_REQUIRED"],
        "reproducibility": {
            "input_manifest_refs": [ref("Manifest", f"manifest:{run_id}")],
            "environment_refs": [ref("Environment", "fixture")],
            "determinism_notes": ["invented deterministic fixture"],
            "unresolved_tbd": [],
        },
    }


def quantity_result(result_id, family, object_id, magnitude, unit, dimension):
    return {
        "result_id": result_id,
        "family": family,
        "object_ref": ref("PipeElement", object_id),
        "basis_ref": ref("LoadCase", "LC-1"),
        "magnitude": magnitude,
        "unit": unit,
        "dimension": dimension,
        "provenance": {"source_name": "invented fixture"},
    }


def result_envelope(envelope_id, run_id, quantities):
    return {
        "result_envelope": {
            "envelope_id": envelope_id,
            "run_ref": ref("AnalysisRun", run_id),
            "result_sets": [
                {
                    "result_set_id": f"set:{envelope_id}",
                    "quantity_results": quantities,
                }
            ],
            "diagnostics": [],
        }
    }


def mapping(mapping_id, left_result_id, right_result_id, **extra):
    base = {
        "mapping_id": mapping_id,
        "mapping_status": "manual_match",
        "left_ref": ref("Result", left_result_id),
        "right_ref": ref("Result", right_result_id),
    }
    base.update(extra)
    return base


def tolerance_profile(value=0.25):
    return {
        "tolerance_profile": {
            "profile_id": "TP-invented-review",
            "rules": [
                {
                    "rule_id": "TR-stress-review",
                    "result_family": "stress",
                    "dimension_id": "stress",
                    "unit_ref": ref("Unit", "Pa"),
                    "tolerance_value": value,
                    "tolerance_value_status": "project_specific_review_required",
                    "normalization_basis": "unit_conversion_required",
                }
            ],
        }
    }


def fixture_inputs():
    left_run = {"analysis_run": run_record("RUN-left", "MS-left", "RES-left")}
    right_run = {"analysis_run": run_record("RUN-right", "MS-right", "RES-right")}
    left_results = result_envelope(
        "RES-left",
        "RUN-left",
        [quantity_result("left:stress:E1", "stress", "E1", 1000.0, "kPa", "stress")],
    )
    right_results = result_envelope(
        "RES-right",
        "RUN-right",
        [quantity_result("right:stress:E1", "stress", "E1", 1000500.0, "Pa", "stress")],
    )
    return {
        "left_run": left_run,
        "right_run": right_run,
        "left_results": left_results,
        "right_results": right_results,
        "mappings": [
            mapping(
                "MAP-stress-E1",
                "left:stress:E1",
                "right:stress:E1",
                normalized_unit="Pa",
            )
        ],
        "tolerance_profile": tolerance_profile(),
        "unit_conversions": {("kPa", "Pa", "stress"): 1000.0},
        "left_settings": {"solver": "linear", "iterations": 1, "audit_mode": "draft"},
        "right_settings": {"solver": "linear", "iterations": 2, "new_flag": True},
        "comparison_id": "CMP-fixture",
    }


def test_comparison_is_deterministic_and_preserves_context():
    inputs = fixture_inputs()

    first = comparison_dict(compare_analysis_runs(**inputs))
    second = comparison_dict(compare_analysis_runs(**deepcopy(inputs)))

    assert first == second
    assert first["comparison_id"] == "CMP-fixture"
    assert first["run_context"]["left"]["run_id"] == "RUN-left"
    assert first["run_context"]["right"]["run_id"] == "RUN-right"
    assert first["run_context"]["left"]["model_state_ref"] == ref("ModelState", "MS-left")
    assert first["run_context"]["right"]["model_state_ref"] == ref("ModelState", "MS-right")
    assert first["run_context"]["left"]["hashes"][0]["value"] == "hash-RUN-left"
    assert first["run_context"]["right"]["solver_version"]["solver_id"] == "invented-solver"


def test_unit_normalized_delta_and_classification_keep_raw_evidence_separate():
    output = comparison_dict(compare_analysis_runs(**fixture_inputs()))

    assert output["diagnostics"] == []
    assert len(output["result_deltas"]) == 1
    delta = output["result_deltas"][0]
    assert delta["mapping_id"] == "MAP-stress-E1"
    assert delta["left_magnitude"] == 1000.0
    assert delta["right_magnitude"] == 1000500.0
    assert delta["raw_delta"] == 999500.0
    assert delta["left_normalized_magnitude"] == 1000000.0
    assert delta["right_normalized_magnitude"] == 1000500.0
    assert delta["normalized_delta"] == 500.0
    assert delta["absolute_normalized_delta"] == 500.0
    assert delta["classification"] == "exceeds_tolerance_profile"
    assert delta["classification_basis"] == "caller_supplied_tolerance_rule"
    assert delta["tolerance_profile_ref"] == "TP-invented-review"
    assert delta["tolerance_rule_id"] == "TR-stress-review"
    assert {item["setting_key"] for item in output["settings_deltas"]} == {
        "audit_mode",
        "iterations",
        "new_flag",
    }


def test_incompatible_or_missing_unit_metadata_produces_diagnostics_not_deltas():
    inputs = fixture_inputs()
    inputs["right_results"]["result_envelope"]["result_sets"][0]["quantity_results"][0][
        "dimension"
    ] = "force"

    dimension_output = comparison_dict(compare_analysis_runs(**inputs))

    assert dimension_output["result_deltas"] == []
    assert {item["code"] for item in dimension_output["diagnostics"]} == {
        "ARC-DIMENSION-INCOMPATIBLE"
    }

    inputs = fixture_inputs()
    inputs["unit_conversions"] = {}

    conversion_output = comparison_dict(compare_analysis_runs(**inputs))

    assert conversion_output["result_deltas"] == []
    assert {item["code"] for item in conversion_output["diagnostics"]} == {
        "ARC-UNIT-CONVERSION-UNSUPPORTED"
    }


def test_missing_mapping_and_result_data_are_explicit_findings():
    inputs = fixture_inputs()
    inputs["mappings"] = [
        mapping(
            "MAP-unresolved",
            "left:stress:E1",
            "right:stress:E1",
            mapping_status="unresolved_mapping",
        ),
        mapping("MAP-missing-result", "left:stress:E1", "right:missing"),
    ]

    output = comparison_dict(compare_analysis_runs(**inputs))

    assert output["result_deltas"] == []
    assert {item["code"] for item in output["diagnostics"]} == {
        "ARC-MAPPING-NOT-COMPARABLE",
        "ARC-RESULT-DATA-MISSING",
    }
    assert output["has_blocking_findings"]


def test_output_does_not_emit_prohibited_professional_claims():
    output = comparison_dict(compare_analysis_runs(**fixture_inputs()))

    boundary = output["professional_boundary"]
    assert boundary["human_review_required"] is True
    assert boundary["software_makes_compliance_claim"] is False
    assert boundary["software_makes_certification_claim"] is False
    assert boundary["software_makes_sealing_claim"] is False
    assert boundary["software_makes_approval_claim"] is False
    assert boundary["software_makes_authentication_claim"] is False

    claim_surface = {
        "diagnostics": output["diagnostics"],
        "result_classifications": [
            {
                "classification": item["classification"],
                "classification_basis": item["classification_basis"],
            }
            for item in output["result_deltas"]
        ],
    }
    text = str(claim_surface).lower()
    for claim in FORBIDDEN_CLAIMS:
        assert claim not in text


if __name__ == "__main__":
    test_comparison_is_deterministic_and_preserves_context()
    test_unit_normalized_delta_and_classification_keep_raw_evidence_separate()
    test_incompatible_or_missing_unit_metadata_produces_diagnostics_not_deltas()
    test_missing_mapping_and_result_data_are_explicit_findings()
    test_output_does_not_emit_prohibited_professional_claims()
