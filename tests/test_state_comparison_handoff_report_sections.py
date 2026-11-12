#!/usr/bin/env python3
"""Focused tests for DEL-08-06 state/comparison/handoff report sections."""

from __future__ import annotations

from copy import deepcopy
import json
from pathlib import Path
import sys


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.reporting.state_comparison_handoff_sections import (  # noqa: E402
    build_persisted_project_report_sections,
    build_state_comparison_handoff_report_sections,
    canonical_json,
    diagnostics_for_report_sections,
)
from core.project_persistence import (  # noqa: E402
    build_project_persistence_envelope,
    round_trip_project_envelope,
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


def test_desktop_shared_parity_fixture_is_generated_by_authoritative_engine():
    fixture_path = ROOT / "fixtures/reports/invented/state_comparison_handoff_desktop_parity.json"
    fixture = json.loads(fixture_path.read_text(encoding="utf-8"))

    actual = build_state_comparison_handoff_report_sections(**fixture["inputs"])

    assert actual == fixture["expected"]


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
            "warnings": [
                {
                    "code": "RUN-INVENTED-WARNING",
                    "severity": "warning",
                    "message": "Invented run warning preserved for report review.",
                    "provenance": provenance("Invented run warning"),
                }
            ],
            "unresolved_assumptions": [
                {
                    "assumption_id": "assumption:invented-run-review",
                    "statement": "Invented run assumption remains unresolved.",
                    "provenance": provenance("Invented run assumption"),
                }
            ],
            "limitations": [
                {
                    "limitation_id": "limitation:invented-run-scope",
                    "statement": "Invented run covers the bounded mechanics fixture only.",
                }
            ],
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


def export_workflow() -> dict[str, object]:
    return {
        "export_workflow_id": "export:invented-del-08-06",
        "source_handoff_package": handoff_package(),
        "export_payload": {
            "model_hash": checksum(ref("Model", "model:invented-del-08-06"), "model_hash", "sha256:invented-model"),
            "units_manifest": {
                "unit_system_ref": ref("UnitSystem", "units:invented-del-08-06"),
                "force_unit": "N",
                "stress_unit": "Pa",
            },
            "entity_ids": {"component_ids": ["component:invented-pipe"]},
            "target_mapping_metadata": {
                "target_system_kind": "generic_downstream_modeling",
                "target_mapping_refs": [ref("TargetMapping", "target-map:invented-export")],
            },
            "unsupported_target_records": [
                {
                    "record_id": "unsupported:invented-export-record",
                    "status": "not_implemented",
                    "human_review_required": True,
                    "private_payload": "private export data must not pass through",
                }
            ],
            "unresolved_assumptions": [],
            "warnings": [],
        },
        "diagnostics": [],
        "privacy": {
            "classification": "invented_public_example",
            "private_payload_embedded": False,
            "protected_payload_embedded": False,
        },
        "professional_boundary": boundary(),
        "provenance": provenance("Invented export workflow"),
    }


def external_prover_metadata() -> dict[str, object]:
    return {
        "metadata_record_id": "external-prover:invented-del-08-06",
        "handoff_package_refs": [ref("HandoffPackage", "handoff:invented-del-08-06")],
        "target_mapping_refs": [ref("TargetMapping", "target-map:invented-del-08-06")],
        "export_workflow_refs": [ref("ExportWorkflow", "export:invented-del-08-06")],
        "immutable_model_state_refs": [ref("ModelState", "state:invented-del-08-06")],
        "unsupported_target_flags": [
            {
                "flag_id": "unsupported:external-prover-metadata-only",
                "status": "metadata_only",
                "human_review_required": True,
            }
        ],
        "assumptions": [],
        "warnings": [],
        "diagnostics": [],
        "professional_boundary": boundary(),
        "provenance": provenance("Invented external prover metadata"),
    }


def section_kwargs() -> dict[str, object]:
    return {
        "section_set_id": "report-sections:invented-del-08-06",
        "model_states": [model_state()],
        "analysis_runs": [analysis_run()],
        "analysis_run_comparisons": [analysis_comparison()],
        "handoff_packages": [handoff_package()],
        "export_workflows": [export_workflow()],
        "external_prover_metadata": [external_prover_metadata()],
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
    assert {item["section_kind"] for item in first["sections"]["handoff_sections"]} == {
        "export_workflow_record",
        "external_prover_metadata",
        "handoff_package_manifest",
    }


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
    handoff_section = next(
        item
        for item in record["sections"]["handoff_sections"]
        if item["section_kind"] == "handoff_package_manifest"
    )
    export_section = next(
        item
        for item in record["sections"]["handoff_sections"]
        if item["section_kind"] == "export_workflow_record"
    )
    external_section = next(
        item
        for item in record["sections"]["handoff_sections"]
        if item["section_kind"] == "external_prover_metadata"
    )

    assert state_section["state_ref"]["ref"] == "state:invented-del-08-06"
    assert state_section["hash_refs"][0]["value"] == "sha256:invented-state"
    assert state_section["warnings"][0]["code"] == "STATE-INVENTED-WARNING"
    assert state_section["assumptions"][0]["assumption_id"] == "assumption:invented-state-review"
    assert state_section["privacy_classification"] == "invented_public_example"
    assert state_section["review_state"] == "machine_checked"
    assert run_section["unit_context"] == ref("UnitSystem", "units:invented-del-08-06")
    assert run_section["warnings"][0]["code"] == "RUN-INVENTED-WARNING"
    assert run_section["assumptions"][0]["assumption_id"] == "assumption:invented-run-review"
    assert run_section["limitations"][0]["limitation_id"] == "limitation:invented-run-scope"
    assert run_section["limitations"][0]["human_review_required"] is True
    assert run_section["solver_context"]["version"] == "0.0.0-fixture"
    assert run_section["source_provenance"]["source_name"] == "Invented analysis run"
    assert run_section["analysis_status"] == ["HUMAN_REVIEW_REQUIRED", "MECHANICS_SOLVED"]
    assert run_section["rule_pack_refs"][0]["checksum"]["value"] == "sha256:invented-rule"
    # REQ-011: the section bounds what it restates of the result differences.
    assert comparison_section["largest_differences"][0]["differences"][0]["normalized_unit"] == "Pa"
    assert "unit_normalized_deltas" not in comparison_section
    assert comparison_section["tolerance_profile_refs"] == ["tolerance:invented-review"]
    assert handoff_section["model_hash"]["value"] == "sha256:invented-model"
    assert handoff_section["unsupported_target_flags"][0]["human_review_required"] is True
    assert export_section["handoff_package_ref"]["ref"] == "handoff:invented-del-08-06"
    assert export_section["unsupported_target_records"][0]["human_review_required"] is True
    assert "private_payload" not in export_section["unsupported_target_records"][0]
    assert external_section["target_mapping_refs"][0] == ref("TargetMapping", "target-map:invented-del-08-06")


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


def test_persisted_run_history_binds_to_report_sections_without_state_inference():
    persistence_envelope = build_project_persistence_envelope(
        project_id="project:persisted-report-fixture",
        project_name="Invented persisted report fixture",
        model_payload={"model_id": "model:invented-del-08-06"},
        model_state_records=[model_state()],
        analysis_run_records=[analysis_run()],
    )
    round_trip = round_trip_project_envelope(persistence_envelope)
    assert round_trip["semantic_equal"] is True

    record = build_persisted_project_report_sections(
        section_set_id="sections:persisted-invented-run",
        persistence_envelope=round_trip["envelope"],
    )
    binding = record["application_service_binding"]
    run_section = next(
        item
        for item in record["sections"]["state_run_sections"]
        if item["section_kind"] == "analysis_run_record"
    )

    assert binding == {
        "source": "project.run_history",
        "workflow_grain": "persisted_run_history_to_report_sections",
        "canonical_records_only": True,
        "project_mutated": False,
        "solver_executed": False,
        "human_approval_inferred": False,
        "code_compliance_inferred": False,
        "external_payload_partitioning": "unchanged_source_contract",
    }
    assert run_section["warnings"][0]["code"] == "RUN-INVENTED-WARNING"
    assert run_section["assumptions"][0]["assumption_id"] == "assumption:invented-run-review"
    assert run_section["limitations"][0]["limitation_id"] == "limitation:invented-run-scope"
    assert run_section["analysis_status"] == ["HUMAN_REVIEW_REQUIRED", "MECHANICS_SOLVED"]
    assert "CODE_COMPLIANT" not in canonical_json(record)
    assert "HUMAN_APPROVED_FOR_PROJECT" not in canonical_json(record)
    assert persistence_envelope["hash"]["payload_partition_status"] == "json_payloads_defined"


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


def test_disallowed_analysis_status_is_diagnosed_and_not_emitted():
    kwargs = section_kwargs()
    kwargs["analysis_runs"] = [analysis_run()]
    kwargs["analysis_runs"][0]["analysis_run"]["analysis_status"].append("HUMAN_APPROVED_FOR_PROJECT")

    record = build_state_comparison_handoff_report_sections(**kwargs)
    run_section = next(
        item
        for item in record["sections"]["state_run_sections"]
        if item["section_kind"] == "analysis_run_record"
    )
    codes = {item["code"] for item in record["diagnostics"]}
    text = canonical_json(record)

    assert "SCH-ANALYSIS-STATUS-BLOCKED" in codes
    assert "HUMAN_APPROVED_FOR_PROJECT" not in run_section["analysis_status"]
    assert "HUMAN_APPROVED_FOR_PROJECT" in text


def test_external_prover_execution_claim_is_out_of_scope():
    kwargs = section_kwargs()
    kwargs["external_prover_metadata"] = [external_prover_metadata()]
    kwargs["external_prover_metadata"][0]["external_prover_executed"] = True

    record = build_state_comparison_handoff_report_sections(**kwargs)
    codes = {item["code"] for item in record["diagnostics"]}

    assert "SCH-EXTERNAL-PROVER-EXECUTION-OUT-OF-SCOPE" in codes


def test_numeric_comparison_deltas_need_units_and_dimensions():
    kwargs = section_kwargs()
    kwargs["analysis_run_comparisons"] = [analysis_comparison()]
    del kwargs["analysis_run_comparisons"][0]["result_deltas"][0]["dimension"]

    record = build_state_comparison_handoff_report_sections(**kwargs)
    codes = {item["code"] for item in record["diagnostics"]}

    assert "SCH-COMPARISON-DELTA-UNIT-MISSING" in codes
    assert "SCH-NUMERIC-UNIT-METADATA-MISSING" in codes


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
    test_disallowed_analysis_status_is_diagnosed_and_not_emitted()
    test_external_prover_execution_claim_is_out_of_scope()
    test_numeric_comparison_deltas_need_units_and_dimensions()
    test_boundary_flags_cannot_be_flipped_on_section_set()
    test_output_boundary_language_does_not_make_prohibited_claims_for_clean_sources()


if __name__ == "__main__":
    main()


def _comparison_parity_writer():
    import importlib.util

    path = ROOT / "tools/fixtures/state_comparison_handoff_comparison_parity.py"
    spec = importlib.util.spec_from_file_location("state_comparison_handoff_comparison_parity", path)
    module = importlib.util.module_from_spec(spec)
    assert spec.loader is not None
    spec.loader.exec_module(module)
    return module


def test_comparison_parity_vector_is_the_reference_engine_output():
    """DEL-08-06 AC-002: the shared comparison vector the desktop assembler is held to is
    this engine's output for its invented records, written by its writer."""

    writer = _comparison_parity_writer()
    assert writer.FIXTURE.read_text(encoding="utf-8") == writer.text()
    fixture = json.loads(writer.FIXTURE.read_text(encoding="utf-8"))
    for case in fixture["cases"]:
        assert build_state_comparison_handoff_report_sections(**case["inputs"]) == case["expected"], case["name"]
    by_name = {case["name"]: case["expected"] for case in fixture["cases"]}
    assert by_name["both_kinds"] == by_name["both_kinds_permuted"]


def test_state_comparison_section_carries_changed_entities_with_values_and_its_hash():
    """REQ-011: every difference in inputs, with both stored values and their units, the
    origin reference beside a changed entity, and the record's hash. REQ-009: a field of a
    provenance record keeps only its path and hashes."""

    writer = _comparison_parity_writer()
    record = writer.state_record()
    kwargs = section_kwargs()
    kwargs["model_state_comparisons"] = [record]

    section = next(
        item
        for item in build_state_comparison_handoff_report_sections(**kwargs)["sections"]["comparison_sections"]
        if item["section_kind"] == "model_state_comparison"
    )

    assert section["hash_refs"] == record["hashes"]
    assert {row["classification"] for row in section["changed_entities"]} == {
        "added", "removed", "changed", "mapped_changed", "unresolved"
    }
    assert section["left_ref"]["model_hash"] == record["left_state_ref"]["model_hash"]
    mapped = next(row for row in section["changed_entities"] if row["classification"] == "mapped_changed")
    assert mapped["changes"] == [{"field": "length", "left": {"value": 2.0, "unit": "m"}, "right": {"value": 2.5, "unit": "m"}}]
    node = next(row for row in section["changed_entities"] if (row["left_ref"] or {}).get("object_type") == "node")
    assert node["changes"][0]["left"] == {"x": 1.5, "y": 0, "z": 0, "unit": "m"}
    material = next(row for row in section["changed_entities"] if (row["left_ref"] or {}).get("object_type") == "material")
    assert material["left_origin_ref"] == "library:material:inv-lib#rec-1"
    content = [change for change in material["changes"] if change["path"].startswith("provenance_record")]
    assert content == [
        {"path": "provenance_record.source_name", "change_kind": "changed", "left_sha256": "1" * 64, "right_sha256": "2" * 64},
        {"path": "provenance_record.verification_note", "change_kind": "added", "right_sha256": "3" * 64},
    ]
    assert "never restates" not in canonical_json(section)
    assert section["csv"] == {"table": "state-comparison", "row_count": 13, "statement": section["csv"]["statement"]}


def test_run_comparison_section_bounds_result_differences():
    """REQ-011: counts per family and reason, the ten largest per family and unit by
    absolute difference (one per object, ties by result identifier, never zero), the first
    ten pairs not compared and rows unmatched per family, and what is not listed."""

    writer = _comparison_parity_writer()
    fixture = json.loads(writer.FIXTURE.read_text(encoding="utf-8"))
    case = next(item for item in fixture["cases"] if item["name"] == "result_difference_bound")
    record = case["inputs"]["analysis_run_comparisons"][0]
    result = build_state_comparison_handoff_report_sections(**case["inputs"])
    section = result["sections"]["comparison_sections"][0]

    assert section["hash_refs"] == record["hashes"]
    assert "not_compared" not in section["run_context"] and "unmatched" not in section["run_context"]
    assert section["run_context"]["left"]["standing"] == {"status": "needs_recompute", "findings": ["INVENTED_FINDING"]}
    groups = {(group["result_family"], group["normalized_unit"]): group["differences"] for group in section["largest_differences"]}
    # Formed by hand from the invented records.
    assert [entry["result_id"] for entry in groups[("stress", "MPa")]] == [
        "result:stress:twin-large", "result:stress:tie-a", "result:stress:tie-b",
        "result:stress:12", "result:stress:11", "result:stress:10", "result:stress:09",
        "result:stress:08", "result:stress:07", "result:stress:06",
    ]
    assert [entry["result_id"] for entry in groups[("stress", "Pa")]] == ["result:stress:in-pa"]
    assert [entry["result_id"] for entry in groups[("force", "N")]] == ["result:force:a", "result:force:b"]
    first = groups[("stress", "MPa")][0]
    assert {key: first[key] for key in ("object_ref", "basis_ref", "component", "location", "left_magnitude", "right_magnitude", "normalized_delta", "normalized_unit")} == {
        "object_ref": "pipe:inv-twin", "basis_ref": {"ref_type": "load_case", "ref_id": "case:inv-1"}, "component": "fx",
        "location": "end_i", "left_magnitude": 1.0, "right_magnitude": 31.0, "normalized_delta": 30.0, "normalized_unit": "MPa",
    }
    counts = {item["result_family"]: item for item in section["result_difference_counts"]["by_family"]}
    assert counts["stress"]["pairs_compared"] == 18 and counts["stress"]["pairs_differing"] == 17
    assert counts["force"]["pairs_compared"] == 3 and counts["force"]["pairs_differing"] == 2
    assert counts["moment"]["pairs_not_compared"] == {"value_not_numeric": 12}
    assert counts["moment"]["rows_unmatched"] == {"left": {"identity_differs": 6}, "right": {"identity_differs": 5}}
    listed = section["pairs_not_compared_listed"][0]
    assert listed["result_family"] == "moment"
    assert [row["result_id"] for row in listed["rows"]] == [f"result:moment:nc-{index:02d}" for index in range(1, 11)]
    assert section["rows_unmatched_listed"][0]["rows"][0]["differing_fields"] == ["family", "dimension"]
    assert section["summary_coverage"] == {
        "result_families": ["force", "moment", "stress"],
        "differences_not_listed": 19 - 13,
        "pairs_not_compared_not_listed": 2,
        "rows_unmatched_not_listed": 1,
        "where": section["summary_coverage"]["where"],
    }
    assert not [item for item in result["diagnostics"] if item["severity"] == "blocking"]


def test_run_comparison_section_is_no_larger_for_ten_times_the_rows():
    """AC-001: a record with ten times the result rows gives a section no larger, apart
    from the width of its counts."""

    writer = _comparison_parity_writer()

    def section_bytes(scale: int) -> int:
        deltas = [
            writer.delta(f"result:stress:{index:05d}", 100.5, 100.5 + index % 97 + 0.25, index % 97 + 0.25, unit="MPa",
                         dimension="stress", family="stress", object_ref=f"pipe:inv-{index:05d}")
            for index in range(1, 101 * scale)
        ]
        record = writer.run_record("analysis-run-comparison:runrec:inv-s:runrec:inv-t", deltas=deltas)
        result = build_state_comparison_handoff_report_sections(**writer.inputs([], [record], "comparison-scale"))
        section = result["sections"]["comparison_sections"][0]
        return len(canonical_json(section))

    one, ten = section_bytes(1), section_bytes(10)
    count_digits = 8 * 2
    assert ten <= one + count_digits, (one, ten)


def test_comparison_without_hashes_gives_an_empty_hash_inventory():
    record = build_state_comparison_handoff_report_sections(**section_kwargs())
    assert record["sections"]["comparison_sections"][0]["hash_refs"] == []
