#!/usr/bin/env python3
"""Focused tests for DEL-14-04 analysis-run comparison."""

import json
import sys
from copy import deepcopy
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.comparison.analysis_run.engine import (
    compare_analysis_runs,
    comparison_dict,
    derive_exact_result_id_mappings,
)
from schema_validation import validate_instance


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
MAPPING_SCHEMA_PATH = ROOT / "schemas" / "comparison_mapping.schema.json"


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


def dec026_pair_tolerance_profile():
    return {
        "tolerance_profile": {
            "profile_id": "TP-dec026-mixed-unit-corpus",
            "rules": [
                {
                    "rule_id": "TR-dec026-stress-relative-absolute",
                    "result_family": "stress",
                    "dimension_id": "stress",
                    "unit_ref": ref("Unit", "Pa"),
                    "tolerance_value": "externally_governed_reference_required",
                    "relative_tolerance_value": 1.0e-3,
                    "absolute_tolerance_value": 10.0,
                    "tolerance_pair_policy": "relative_plus_absolute_floor",
                    "tolerance_value_status": "externally_governed",
                    "normalization_basis": "unit_conversion_required",
                },
                {
                    "rule_id": "TR-dec026-force-absolute-floor",
                    "result_family": "force",
                    "dimension_id": "force",
                    "unit_ref": ref("Unit", "N"),
                    "tolerance_value": "externally_governed_reference_required",
                    "relative_tolerance_value": 1.0e-9,
                    "absolute_tolerance_value": 0.05,
                    "tolerance_pair_policy": "relative_plus_absolute_floor",
                    "tolerance_value_status": "externally_governed",
                    "normalization_basis": "unit_conversion_required",
                },
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


def test_exact_stable_result_id_mapping_is_produced_and_round_trips():
    inputs = fixture_inputs()
    stable_id = "result:stress:E1"
    inputs["left_results"]["result_envelope"]["result_sets"][0][
        "quantity_results"
    ][0]["result_id"] = stable_id
    inputs["right_results"]["result_envelope"]["result_sets"][0][
        "quantity_results"
    ][0]["result_id"] = stable_id
    inputs["left_results"]["result_envelope"]["result_sets"][0][
        "quantity_results"
    ][0].update({"magnitude": 1_000_000.0, "unit": "Pa"})

    produced = derive_exact_result_id_mappings(
        inputs["left_results"], inputs["right_results"]
    )
    round_tripped = json.loads(json.dumps(produced, sort_keys=True))

    assert len(round_tripped) == 1
    automatic = round_tripped[0]
    assert automatic["mapping_status"] == "automatic_match"
    assert automatic["left_ref"] == ref("Result", stable_id)
    assert automatic["right_ref"] == ref("Result", stable_id)
    assert automatic["mapping_evidence"]["evidence_kind"] == "stable_id_alignment"
    assert automatic["mapping_evidence"]["stable_id_preservation"] == (
        "left_and_right_refs_preserved"
    )
    assert automatic["mapping_evidence"]["manual_review_state"] == "not_manual"
    assert automatic["mapping_evidence"]["source_refs"] == automatic["affected_refs"]
    assert automatic["confidence"]["confidence_level"] == "exact_stable_id"
    assert automatic["review"]["review_status"] == "pending"
    assert automatic["review"]["reviewed_at"] == "TBD"
    mapping_schema = json.loads(MAPPING_SCHEMA_PATH.read_text(encoding="utf-8"))
    assert validate_instance(
        {
            "$schema": mapping_schema["$schema"],
            "$defs": mapping_schema["$defs"],
            "$ref": "#/$defs/MappingRecord",
        },
        automatic,
        schema_label="DEL-14-05 MappingRecord projection",
        instance_label="DEL-14-04 exact stable-ID mapping",
    )

    inputs["mappings"] = round_tripped
    output = comparison_dict(compare_analysis_runs(**inputs))
    assert len(output["result_deltas"]) == 1
    assert output["result_deltas"][0]["mapping_id"] == f"AUTO-{stable_id}"
    assert output["result_deltas"][0]["left_result_id"] == stable_id
    assert output["result_deltas"][0]["right_result_id"] == stable_id


def test_nonidentical_or_ambiguous_result_ids_require_manual_mapping():
    inputs = fixture_inputs()

    assert derive_exact_result_id_mappings(
        inputs["left_results"], inputs["right_results"]
    ) == ()

    manual = json.loads(json.dumps(inputs["mappings"], sort_keys=True))
    inputs["mappings"] = manual
    output = comparison_dict(compare_analysis_runs(**inputs))
    assert output["result_deltas"][0]["mapping_id"] == "MAP-stress-E1"
    assert output["result_deltas"][0]["left_result_id"] == "left:stress:E1"
    assert output["result_deltas"][0]["right_result_id"] == "right:stress:E1"

    duplicate_left = deepcopy(inputs["left_results"])
    quantities = duplicate_left["result_envelope"]["result_sets"][0][
        "quantity_results"
    ]
    quantities.append(deepcopy(quantities[0]))
    right = deepcopy(inputs["right_results"])
    right["result_envelope"]["result_sets"][0]["quantity_results"][0][
        "result_id"
    ] = "left:stress:E1"
    assert derive_exact_result_id_mappings(duplicate_left, right) == ()


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


def test_dec026_mixed_unit_relative_absolute_tolerance_corpus():
    inputs = fixture_inputs()
    inputs["left_results"] = result_envelope(
        "RES-left",
        "RUN-left",
        [
            quantity_result("left:stress:E1", "stress", "E1", 1000.0, "kPa", "stress"),
            quantity_result("left:force:E1", "force", "E1", 0.0, "N", "force"),
        ],
    )
    inputs["right_results"] = result_envelope(
        "RES-right",
        "RUN-right",
        [
            quantity_result("right:stress:E1", "stress", "E1", 1000500.0, "Pa", "stress"),
            quantity_result("right:force:E1", "force", "E1", 0.01, "lbf", "force"),
        ],
    )
    inputs["mappings"] = [
        mapping(
            "MAP-force-E1",
            "left:force:E1",
            "right:force:E1",
            normalized_unit="N",
        ),
        mapping(
            "MAP-stress-E1",
            "left:stress:E1",
            "right:stress:E1",
            normalized_unit="Pa",
        ),
    ]
    inputs["tolerance_profile"] = dec026_pair_tolerance_profile()
    inputs["unit_conversions"] = {
        ("kPa", "Pa", "stress"): 1000.0,
        ("lbf", "N", "force"): 4.4482216152605,
    }

    output = comparison_dict(compare_analysis_runs(**inputs))
    deltas = {item["mapping_id"]: item for item in output["result_deltas"]}

    assert output["diagnostics"] == []
    assert set(deltas) == {"MAP-force-E1", "MAP-stress-E1"}
    assert deltas["MAP-stress-E1"]["left_normalized_magnitude"] == 1_000_000.0
    assert deltas["MAP-stress-E1"]["right_normalized_magnitude"] == 1_000_500.0
    assert deltas["MAP-stress-E1"]["absolute_normalized_delta"] == 500.0
    assert deltas["MAP-stress-E1"]["classification"] == "within_tolerance_profile"
    assert deltas["MAP-stress-E1"]["classification_basis"] == (
        "caller_supplied_relative_absolute_tolerance_rule"
    )
    assert deltas["MAP-stress-E1"]["tolerance_rule_id"] == (
        "TR-dec026-stress-relative-absolute"
    )
    assert deltas["MAP-force-E1"]["right_magnitude"] == 0.01
    assert round(deltas["MAP-force-E1"]["right_normalized_magnitude"], 12) == round(
        0.044482216152605,
        12,
    )
    assert deltas["MAP-force-E1"]["classification"] == "within_tolerance_profile"
    assert deltas["MAP-force-E1"]["tolerance_rule_id"] == "TR-dec026-force-absolute-floor"

    missing_conversion_inputs = deepcopy(inputs)
    missing_conversion_inputs["unit_conversions"] = {("kPa", "Pa", "stress"): 1000.0}
    missing_output = comparison_dict(compare_analysis_runs(**missing_conversion_inputs))

    assert [item["mapping_id"] for item in missing_output["result_deltas"]] == ["MAP-stress-E1"]
    assert {item["code"] for item in missing_output["diagnostics"]} == {
        "ARC-UNIT-CONVERSION-UNSUPPORTED"
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


def test_carried_run_diagnostics_are_preserved_as_review_evidence():
    inputs = fixture_inputs()
    inputs["left_run"]["analysis_run"]["diagnostics"] = [
        {
            "severity": "blocking",
            "message": "Invented left run diagnostic remains review evidence.",
            "remediation": "Resolve the left run diagnostic before relying on comparison output.",
        }
    ]
    inputs["right_run"]["analysis_run"]["diagnostics"] = [
        {
            "severity": "warning",
            "message": "Invented right run diagnostic remains review evidence.",
            "remediation": "Review the right run diagnostic before relying on comparison output.",
        }
    ]

    output = comparison_dict(compare_analysis_runs(**inputs))
    run_diagnostics = [
        item for item in output["diagnostics"] if item["class"] == "RUN_DIAGNOSTIC"
    ]

    assert output["has_blocking_findings"] is True
    assert {item["code"] for item in run_diagnostics} == {
        "ARC-RUN-DIAGNOSTIC-LEFT",
        "ARC-RUN-DIAGNOSTIC-RIGHT",
    }
    assert {item["severity"] for item in run_diagnostics} == {"blocking", "warning"}
    assert {
        tuple((ref["object_type"], ref["ref"]) for ref in item["affected_refs"])
        for item in run_diagnostics
    } == {
        (("AnalysisRun", "RUN-left"),),
        (("AnalysisRun", "RUN-right"),),
    }
    assert {
        item["message"] for item in run_diagnostics
    } == {
        "Invented left run diagnostic remains review evidence.",
        "Invented right run diagnostic remains review evidence.",
    }


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
    test_exact_stable_result_id_mapping_is_produced_and_round_trips()
    test_nonidentical_or_ambiguous_result_ids_require_manual_mapping()
    test_unit_normalized_delta_and_classification_keep_raw_evidence_separate()
    test_dec026_mixed_unit_relative_absolute_tolerance_corpus()
    test_incompatible_or_missing_unit_metadata_produces_diagnostics_not_deltas()
    test_missing_mapping_and_result_data_are_explicit_findings()
    test_carried_run_diagnostics_are_preserved_as_review_evidence()
    test_output_does_not_emit_prohibited_professional_claims()
