#!/usr/bin/env python3
"""Focused tests for DEL-08-06 state/comparison/handoff report sections."""

from __future__ import annotations

from copy import deepcopy
from pathlib import Path
import sys


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.reporting.state_comparison_handoff_sections import (  # noqa: E402
    build_state_comparison_handoff_report_sections,
    canonical_json,
    diagnostics_for_report_sections,
)


FORBIDDEN_OUTPUT_PHRASES = {
    "code " + "compliant",
    "cert" + "ified",
    "se" + "aled",
    "authentic" + "ated",
    "professional " + "approval",
    "external " + "validation",
    "engineering " + "acceptance",
}


def ref(object_type: str, value: str) -> dict[str, str]:
    return {"object_type": object_type, "ref": value}


def provenance(source_name: str = "Invented DEL-08-06 fixture") -> dict[str, str]:
    return {
        "source_name": source_name,
        "source_location": "tests/test_state_comparison_handoff_report_sections.py",
        "source_license": "project-invented-test-data",
        "contributor": "OpenPipeStress",
        "contributor_attestation": "invented non-engineering fixture",
        "redistribution_status": "invented_non_engineering_example",
        "review_status": "machine_checked",
        "review_classification": "machine_checked",
        "privacy_classification": "invented_public_example",
    }


def checksum(payload_ref: dict[str, str], scope: str, value: str) -> dict[str, object]:
    return {
        "algorithm": "sha256",
        "canonicalization": "JCS",
        "payload_ref": payload_ref,
        "payload_scope": scope,
        "value": value,
    }


def boundary() -> dict[str, bool]:
    return {
        "human_review_required": True,
        "software_makes_compliance_claim": False,
        "software_makes_certification_claim": False,
        "software_makes_sealing_claim": False,
        "software_makes_approval_claim": False,
        "software_makes_authentication_claim": False,
        "software_creates_professional_reliance_record": False,
        "software_creates_external_validation_record": False,
    }


def model_state() -> dict[str, object]:
    state_id = "state:invented-del-08-06"
    return {
        "model_state": {
            "state_id": state_id,
            "state_name": "Invented state fixture",
            "state_kind": "handoff_basis",
            "created_at": "2026-05-06T00:00:00Z",
            "model_ref": ref("Model", "model:invented-del-08-06"),
            "parent_state_refs": [],
            "tags": [],
            "notes": [],
            "external_references": [],
            "unresolved_assumptions": [
                {
                    "assumption_id": "assumption:invented-state-review",
                    "statement": "Invented state fixture requires human review before use.",
                    "affected_refs": [ref("ModelState", state_id)],
                    "provenance": provenance("Invented state assumption"),
                }
            ],
            "warnings": [
                {
                    "code": "STATE-INVENTED-WARNING",
                    "severity": "warning",
                    "message": "Invented state warning for traceability.",
                    "affected_refs": [ref("ModelState", state_id)],
                    "provenance": provenance("Invented state warning"),
                }
            ],
            "analysis_status": ["MECHANICS_SOLVED", "HUMAN_REVIEW_REQUIRED"],
            "hashes": [checksum(ref("ModelState", state_id), "model_state_record", "sha256:invented-state")],
            "immutability_policy": {"snapshot_is_read_only": True},
            "professional_boundary": boundary(),
            "provenance": provenance("Invented model state"),
        }
    }


def analysis_run() -> dict[str, object]:
    run_id = "run:invented-del-08-06"
    return {
        "analysis_run": {
            "run_id": run_id,
            "run_name": "Invented run fixture",
            "run_kind": "mechanics_solve",
            "created_at": "2026-05-06T00:01:00Z",
            "model_state_ref": ref("ModelState", "state:invented-del-08-06"),
            "solver_version": {
                "solver_id": "invented-solver",
                "version": "0.0.0-fixture",
                "build_hash": "sha256:invented-solver",
            },
            "settings_ref": ref("SolverSettings", "settings:invented-del-08-06"),
            "unit_system_ref": ref("UnitSystem", "units:invented-del-08-06"),
            "load_basis_refs": [ref("LoadCase", "load:invented")],
            "diagnostics": [],
            "result_refs": [ref("ResultEnvelope", "results:invented-del-08-06")],
            "rule_pack_refs": [
                {
                    "rule_pack_id": "rule-pack:invented-del-08-06",
                    "version": "0.1.0",
                    "checksum": checksum(ref("RulePack", "rule-pack:invented-del-08-06"), "rule_pack_reference_metadata", "sha256:invented-rule"),
                    "source_notice": "Invented metadata-only rule-pack reference.",
                    "private_payload_redacted": True,
                    "provenance": provenance("Invented rule-pack reference"),
                }
            ],
            "library_refs": [ref("Library", "library:invented-del-08-06")],
            "hashes": [checksum(ref("AnalysisRun", run_id), "analysis_run_record", "sha256:invented-run")],
            "analysis_status": ["MECHANICS_SOLVED", "HUMAN_REVIEW_REQUIRED"],
            "reproducibility": {
                "input_manifest_refs": [ref("Manifest", "manifest:invented-del-08-06")],
                "environment_refs": [ref("Environment", "environment:invented")],
                "determinism_notes": ["invented deterministic fixture"],
                "unresolved_tbd": [],
            },
            "professional_boundary": boundary(),
            "provenance": provenance("Invented analysis run"),
        }
    }


def analysis_comparison() -> dict[str, object]:
    return {
        "comparison_id": "comparison:invented-run-delta",
        "run_context": {
            "left": {"run_id": "run:left", "hashes": [checksum(ref("AnalysisRun", "run:left"), "analysis_run_record", "sha256:left")]},
            "right": {"run_id": "run:right", "hashes": [checksum(ref("AnalysisRun", "run:right"), "analysis_run_record", "sha256:right")]},
        },
        "result_deltas": [
            {
                "mapping_id": "mapping:invented-stress",
                "mapping_status": "manual_match",
                "left_result_id": "result:left-stress",
                "right_result_id": "result:right-stress",
                "result_family": "stress",
                "dimension": "stress",
                "normalized_unit": "Pa",
                "left_magnitude": 1000.0,
                "right_magnitude": 1001.0,
                "left_normalized_magnitude": 1000.0,
                "right_normalized_magnitude": 1001.0,
                "normalized_delta": 1.0,
                "absolute_normalized_delta": 1.0,
                "tolerance_profile_ref": "tolerance:invented-review",
                "classification": "requires_human_review",
            }
        ],
        "settings_deltas": [],
        "diagnostics": [],
        "professional_boundary": boundary(),
        "provenance": provenance("Invented analysis comparison"),
    }


def handoff_package() -> dict[str, object]:
    return {
        "deliverable_id": "DEL-15-01",
        "handoff_package_manifest": {
            "package_identity": {
                "handoff_package_id": "handoff:invented-del-08-06",
                "manifest_id": "manifest:invented-del-08-06",
                "package_schema_version": "0.1.0",
            },
            "model_hash": checksum(ref("Model", "model:invented-del-08-06"), "model_hash", "sha256:invented-model"),
            "units_manifest": {
                "unit_system_ref": ref("UnitSystem", "units:invented-del-08-06"),
                "force_unit": "N",
                "stress_unit": "Pa",
                "provenance": provenance("Invented units manifest"),
            },
            "entity_ids": {
                "component_ids": ["component:invented-pipe"],
                "analysis_run_ids": ["run:invented-del-08-06"],
            },
            "library_refs": [],
            "rule_pack_refs": [],
            "target_mapping_metadata": {
                "target_system_kind": "generic_downstream_modeling",
                "target_mapping_refs": [ref("TargetMapping", "target-map:invented-del-08-06")],
                "provenance": provenance("Invented target mapping metadata"),
            },
            "unsupported_behavior_flags": [
                {
                    "flag_id": "unsupported:external-tool-not-invoked",
                    "behavior_label": "external_tool_not_invoked",
                    "status": "not_implemented",
                    "human_review_required": True,
                    "provenance": provenance("Invented unsupported target flag"),
                }
            ],
            "unresolved_assumptions": [],
            "warnings": [],
            "diagnostics": [],
            "privacy": {
                "classification": "invented_public_example",
                "private_payload_embedded": False,
                "protected_payload_embedded": False,
            },
            "review_classification": "machine_checked",
            "provenance": provenance("Invented handoff manifest"),
            "professional_boundary": boundary(),
        },
    }


def section_kwargs() -> dict[str, object]:
    return {
        "section_set_id": "report-sections:invented-del-08-06",
        "model_states": [model_state()],
        "analysis_runs": [analysis_run()],
        "analysis_run_comparisons": [analysis_comparison()],
        "handoff_packages": [handoff_package()],
        "source_notes": ["invented public test fixture; no protected standards content"],
    }


def test_sections_are_deterministic_and_represent_all_required_families():
    first = build_state_comparison_handoff_report_sections(**section_kwargs())
    second = build_state_comparison_handoff_report_sections(**deepcopy(section_kwargs()))

    assert canonical_json(first) == canonical_json(second)
    assert first["deliverable_id"] == "DEL-08-06"
    assert first["package_id"] == "PKG-08"
    assert {item["section_kind"] for item in first["sections"]["state_run_sections"]} == {
        "analysis_run_record",
        "model_state_record",
    }
    assert first["sections"]["comparison_sections"][0]["section_kind"] == "analysis_run_comparison"
    assert first["sections"]["handoff_sections"][0]["section_kind"] == "handoff_package_manifest"


def test_sections_preserve_ids_hashes_warnings_assumptions_units_and_review_state():
    record = build_state_comparison_handoff_report_sections(**section_kwargs())
    state_section = next(
        item
        for item in record["sections"]["state_run_sections"]
        if item["section_kind"] == "model_state_record"
    )
    run_section = next(
        item
        for item in record["sections"]["state_run_sections"]
        if item["section_kind"] == "analysis_run_record"
    )
    comparison_section = record["sections"]["comparison_sections"][0]
    handoff_section = record["sections"]["handoff_sections"][0]

    assert state_section["state_ref"]["ref"] == "state:invented-del-08-06"
    assert state_section["hash_refs"][0]["value"] == "sha256:invented-state"
    assert state_section["warnings"][0]["code"] == "STATE-INVENTED-WARNING"
    assert state_section["assumptions"][0]["assumption_id"] == "assumption:invented-state-review"
    assert state_section["privacy_classification"] == "invented_public_example"
    assert state_section["review_state"] == "machine_checked"
    assert run_section["unit_context"] == ref("UnitSystem", "units:invented-del-08-06")
    assert run_section["rule_pack_refs"][0]["checksum"]["value"] == "sha256:invented-rule"
    assert comparison_section["unit_normalized_deltas"][0]["normalized_unit"] == "Pa"
    assert comparison_section["tolerance_profile_refs"] == ["tolerance:invented-review"]
    assert handoff_section["model_hash"]["value"] == "sha256:invented-model"
    assert handoff_section["unsupported_target_flags"][0]["human_review_required"] is True


def test_missing_source_values_become_explicit_findings_and_tbds():
    kwargs = section_kwargs()
    kwargs["analysis_runs"] = [analysis_run()]
    del kwargs["analysis_runs"][0]["analysis_run"]["unit_system_ref"]
    del kwargs["analysis_runs"][0]["analysis_run"]["provenance"]

    record = build_state_comparison_handoff_report_sections(**kwargs)
    codes = {item["code"] for item in record["diagnostics"]}

    assert "SCH-SOURCE-VALUE-MISSING" in codes
    assert any(item["severity"] == "warning" for item in record["diagnostics"])
    assert any("unit_context" in item["tbd_id"] for item in record["unresolved_tbds"])
    assert any("source_provenance" in item["tbd_id"] for item in record["unresolved_tbds"])


def test_authority_claims_are_diagnosed_without_copying_claim_text():
    kwargs = section_kwargs()
    kwargs["model_states"] = [model_state()]
    kwargs["model_states"][0]["model_state"]["professional_boundary"]["software_makes_approval_claim"] = True
    kwargs["model_states"][0]["model_state"]["warnings"][0]["message"] = (
        "Software says code " + "compliant and cert" + "ified."
    )

    record = build_state_comparison_handoff_report_sections(**kwargs)
    codes = {item["code"] for item in record["diagnostics"]}
    text = canonical_json(record).lower()

    assert "SCH-SOFTWARE-AUTHORITY-FLAG-BLOCKED" in codes
    assert "SCH-AUTHORITY-CLAIM-REJECTED" in codes
    assert "software says code " + "compliant" not in text
    assert "omitted_prohibited_authority_or_reliance_claim" in text


def test_boundary_flags_cannot_be_flipped_on_section_set():
    record = build_state_comparison_handoff_report_sections(**section_kwargs())
    mutated = deepcopy(record)
    mutated["professional_boundary"]["human_review_required"] = False
    mutated["professional_boundary"]["software_creates_professional_reliance_record"] = True

    codes = {item["code"] for item in diagnostics_for_report_sections(mutated)}

    assert "SCH-HUMAN-REVIEW-NOT-REQUIRED-BLOCKED" in codes
    assert "SCH-SOFTWARE-AUTHORITY-FLAG-BLOCKED" in codes


def test_output_boundary_language_does_not_make_prohibited_claims_for_clean_sources():
    record = build_state_comparison_handoff_report_sections(**section_kwargs())
    text = canonical_json(record).lower()

    for forbidden in FORBIDDEN_OUTPUT_PHRASES:
        assert forbidden not in text
    assert record["professional_boundary"]["human_review_required"] is True
    assert record["professional_boundary"]["software_makes_compliance_claim"] is False
    assert record["professional_boundary"]["software_makes_certification_claim"] is False
    assert record["professional_boundary"]["software_makes_sealing_claim"] is False
    assert record["professional_boundary"]["software_makes_approval_claim"] is False
    assert record["professional_boundary"]["software_makes_authentication_claim"] is False


def main():
    test_sections_are_deterministic_and_represent_all_required_families()
    test_sections_preserve_ids_hashes_warnings_assumptions_units_and_review_state()
    test_missing_source_values_become_explicit_findings_and_tbds()
    test_authority_claims_are_diagnosed_without_copying_claim_text()
    test_boundary_flags_cannot_be_flipped_on_section_set()
    test_output_boundary_language_does_not_make_prohibited_claims_for_clean_sources()


if __name__ == "__main__":
    main()
