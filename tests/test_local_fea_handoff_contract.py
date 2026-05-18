#!/usr/bin/env python3
"""Stdlib checks for the local FEA handoff contract."""

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCHEMA_PATH = ROOT / "schemas" / "local_fea_handoff.schema.yaml"
GUIDANCE_PATH = ROOT / "docs" / "local_analysis" / "local_fea_handoff_guidance.md"

REQUIRED_ROOT = {
    "schema_version",
    "deliverable_id",
    "package_id",
    "scope_items",
    "objective",
    "contract_status",
    "handoff_package",
}

REQUIRED_DEFS = {
    "Assumption",
    "ChecksumRef",
    "ContractStatus",
    "Diagnostic",
    "EntityIds",
    "GuidanceAssessment",
    "HandoffGuidanceLabel",
    "HandoffPackage",
    "LocalRegion",
    "PrivacyContext",
    "ProfessionalBoundary",
    "Provenance",
    "Reference",
    "Reproducibility",
    "SourceRefs",
    "TransferBasis",
    "UnitsManifest",
    "UnsupportedBehaviorFlag",
}

REQUIRED_DIAGNOSTIC_CLASSES = {
    "SOLVE_BLOCKING",
    "RULE_CHECK_BLOCKING",
    "PROVENANCE_WARNING",
    "ASSUMPTION_WARNING",
    "NONLINEAR_WARNING",
    "IP_BOUNDARY_WARNING",
    "UNIT_WARNING",
    "LOCAL_HANDOFF_BLOCKING",
    "LOCAL_HANDOFF_WARNING",
    "PRIVACY_WARNING",
    "EXPORT_BLOCKING",
}

REQUIRED_GUIDANCE_LABELS = {
    "global_centerline_expected_sufficient_for_screening",
    "local_detail_review_consider",
    "local_shell_solid_handoff_consider",
    "global_to_local_transfer_inputs_incomplete",
    "human_review_required",
    "TBD",
}

def load_schema():
    with SCHEMA_PATH.open(encoding="utf-8") as schema_file:
        return json.load(schema_file)


def required_at(schema, definition_name):
    return set(schema["$defs"][definition_name]["required"])


def enum_at(schema, definition_name):
    return set(schema["$defs"][definition_name]["enum"])


def walk_keys(value):
    if isinstance(value, dict):
        for key, item in value.items():
            yield key
            yield from walk_keys(item)
    elif isinstance(value, list):
        for item in value:
            yield from walk_keys(item)


def main():
    schema = load_schema()
    defs = schema["$defs"]

    assert GUIDANCE_PATH.exists()
    assert schema["$schema"] == "https://json-schema.org/draft/2020-12/schema"
    assert schema["additionalProperties"] is False
    assert "default" not in set(walk_keys(schema))
    assert REQUIRED_ROOT <= set(schema["required"])
    assert REQUIRED_DEFS <= set(defs)

    assert schema["properties"]["deliverable_id"]["const"] == "DEL-10-03"
    assert schema["properties"]["package_id"]["const"] == "PKG-10"
    assert {"SOW-031", "SOW-049"} <= set(
        schema["properties"]["scope_items"]["items"]["enum"]
    )
    assert schema["properties"]["objective"]["const"] == "OBJ-009"

    status = defs["ContractStatus"]["properties"]
    assert status["contract_kind"]["const"] == (
        "schema_first_local_fea_handoff_contract"
    )
    assert status["global_analysis_role"]["const"] == (
        "primary_global_centerline_frame_model"
    )
    assert status["local_analysis_role"]["const"] == (
        "optional_specialized_shell_solid_handoff"
    )
    for key in [
        "concrete_export_format",
        "target_solver_adapter",
        "mesh_generation",
        "external_solver_invocation",
    ]:
        assert status[key]["const"] == "TBD"
    assert status["professional_decision"]["const"] == "human_review_required"

    package_required = required_at(schema, "HandoffPackage")
    assert {
        "source_refs",
        "local_region",
        "units_manifest",
        "entity_ids",
        "transfer_basis",
        "guidance_assessment",
        "assumptions",
        "unsupported_behavior_flags",
        "warnings",
        "diagnostics",
        "privacy",
        "provenance",
        "professional_boundary",
        "reproducibility",
    } <= package_required

    source_required = required_at(schema, "SourceRefs")
    assert {
        "project_ref",
        "model_ref",
        "result_envelope_ref",
        "global_model_kind",
        "model_hash",
        "result_hash",
    } <= source_required
    assert (
        defs["SourceRefs"]["properties"]["global_model_kind"]["const"]
        == "centerline_frame_global_analysis"
    )

    units_required = required_at(schema, "UnitsManifest")
    assert {
        "coordinate_unit",
        "force_unit",
        "moment_unit",
        "displacement_unit",
        "rotation_unit",
        "stress_unit",
        "temperature_unit",
    } <= units_required
    assert "schemas/units.schema.yaml" in defs["UnitsManifest"]["properties"][
        "dimension_basis"
    ]["enum"]

    entity_required = required_at(schema, "EntityIds")
    assert {
        "component_ids",
        "node_ids",
        "element_ids",
        "station_ids",
        "load_case_ids",
        "result_ids",
    } <= entity_required

    transfer_required = required_at(schema, "TransferBasis")
    assert {
        "load_case_refs",
        "displacement_result_refs",
        "force_result_refs",
        "moment_result_refs",
        "boundary_condition_refs",
        "cut_boundary_refs",
        "transfer_method_label",
        "limitations",
    } <= transfer_required
    assert {
        "result_reference_only",
        "user_reviewed_interpolation_reference",
        "TBD",
    } <= set(defs["TransferBasis"]["properties"]["transfer_method_label"]["enum"])

    assessment_required = required_at(schema, "GuidanceAssessment")
    assert {
        "labels",
        "criteria_input_refs",
        "rationale_refs",
        "human_review_required",
        "software_makes_approval_claim",
        "software_makes_compliance_claim",
        "software_makes_certification_claim",
        "software_makes_sealing_claim",
    } <= assessment_required
    assert REQUIRED_GUIDANCE_LABELS <= enum_at(schema, "HandoffGuidanceLabel")
    assessment = defs["GuidanceAssessment"]["properties"]
    assert assessment["labels"]["contains"]["const"] == "human_review_required"
    assert assessment["human_review_required"]["const"] is True
    assert assessment["software_makes_approval_claim"]["const"] is False
    assert assessment["software_makes_compliance_claim"]["const"] is False
    assert assessment["software_makes_certification_claim"]["const"] is False
    assert assessment["software_makes_sealing_claim"]["const"] is False

    diagnostic_required = required_at(schema, "Diagnostic")
    assert {
        "code",
        "class",
        "severity",
        "source",
        "affected_object",
        "message",
        "remediation",
        "provenance",
    } <= diagnostic_required
    assert REQUIRED_DIAGNOSTIC_CLASSES <= set(
        defs["Diagnostic"]["properties"]["class"]["enum"]
    )

    privacy = defs["PrivacyContext"]["properties"]
    assert privacy["local_only"]["const"] is True
    assert privacy["telemetry_allowed"]["const"] is False
    assert privacy["private_payload_embedded"]["const"] is False
    assert "user_supplied_private_reference" in privacy["classification"]["enum"]
    assert "protected_suspected" in privacy["classification"]["enum"]

    boundary = defs["ProfessionalBoundary"]["properties"]
    assert boundary["human_review_required"]["const"] is True
    assert boundary["software_makes_compliance_claim"]["const"] is False
    assert boundary["software_makes_certification_claim"]["const"] is False
    assert boundary["software_makes_sealing_claim"]["const"] is False
    assert boundary["software_makes_approval_claim"]["const"] is False
    assert boundary["software_makes_authentication_claim"]["const"] is False

    flags = defs["UnsupportedBehaviorFlag"]["properties"]
    assert {
        "mesh_generation_not_performed",
        "external_solver_not_invoked",
        "target_format_not_selected",
        "boundary_transfer_requires_review",
        "local_detail_assumption_unresolved",
        "approximate_global_to_local_transfer",
    } <= set(flags["behavior_label"]["enum"])
    assert {"unsupported", "approximate", "requires_human_review", "TBD"} <= set(
        flags["status"]["enum"]
    )

    checksum = defs["ChecksumRef"]["properties"]
    assert "JCS-compatible-json" in checksum["canonicalization"]["enum"]


if __name__ == "__main__":
    main()
