#!/usr/bin/env python3
"""Stdlib checks for the canonical handoff package schema."""

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCHEMA_PATH = ROOT / "schemas" / "handoff_package.schema.json"

REQUIRED_ROOT = {
    "schema_version",
    "deliverable_id",
    "package_id",
    "scope_item",
    "objectives",
    "handoff_contract_status",
    "handoff_package_manifest",
}

REQUIRED_DEFS = {
    "AnalysisRunRef",
    "AssumptionRecord",
    "Checksum",
    "Diagnostic",
    "EntityIdManifest",
    "HandoffContractStatus",
    "HandoffPackageManifest",
    "Id",
    "LibraryRef",
    "ModelBasis",
    "ModelStateRef",
    "PackageIdentity",
    "PrivacyClassification",
    "PrivacyContext",
    "ProfessionalBoundary",
    "Provenance",
    "RedistributionStatus",
    "Reference",
    "ResultExportRef",
    "ReviewClassification",
    "RulePackRef",
    "TargetMappingMetadata",
    "UnitsManifestRef",
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
    "LOCAL_HANDOFF_WARNING",
    "EXPORT_BLOCKING",
    "TARGET_MAPPING_WARNING",
    "UNSUPPORTED_BEHAVIOR_WARNING",
    "PRIVACY_WARNING",
    "HASH_WARNING",
}

FORBIDDEN_STATUS = {
    "HUMAN_APPROVED_FOR_PROJECT",
    "CODE_COMPLIANT",
    "CERTIFIED",
    "SEALED",
    "APPROVED",
}

FORBIDDEN_SCHEMA_TEXT = {
    "formal prover approval status",
    "code compliant",
    "certified by software",
    "sealed by software",
    "professional approval by the software",
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


def walk_strings(value):
    if isinstance(value, str):
        yield value
    elif isinstance(value, dict):
        for item in value.values():
            yield from walk_strings(item)
    elif isinstance(value, list):
        for item in value:
            yield from walk_strings(item)


def main():
    schema = load_schema()
    defs = schema["$defs"]

    assert schema["$schema"] == "https://json-schema.org/draft/2020-12/schema"
    assert schema["additionalProperties"] is False
    assert "default" not in set(walk_keys(schema))
    assert REQUIRED_ROOT <= set(schema["required"])
    assert REQUIRED_DEFS <= set(defs)

    assert schema["properties"]["deliverable_id"]["const"] == "DEL-15-01"
    assert schema["properties"]["package_id"]["const"] == "PKG-15"
    assert schema["properties"]["scope_item"]["const"] == "SOW-074"
    assert schema["properties"]["objectives"]["contains"]["const"] == "OBJ-017"

    contract = defs["HandoffContractStatus"]["properties"]
    assert (
        contract["record_contract"]["const"]
        == "schema_first_canonical_handoff_package_records"
    )
    assert contract["manifest_contract"]["const"] == (
        "schema_first_handoff_manifest_metadata"
    )
    assert contract["model_schema_binding"]["const"] == "schemas/model.schema.yaml"
    assert contract["units_schema_binding"]["const"] == "schemas/units.schema.yaml"
    assert contract["model_state_binding"]["const"] == "schemas/model_state.schema.json"
    assert contract["analysis_run_binding"]["const"] == "schemas/analysis_run.schema.json"
    assert contract["result_export_binding"]["const"] == "schemas/results.schema.yaml"
    assert contract["local_fea_handoff_binding"]["const"] == (
        "schemas/local_fea_handoff.schema.yaml"
    )
    assert contract["audit_hash_binding"]["const"] == (
        "hash_records_follow_audit_manifest_predecessor_semantics"
    )
    assert contract["target_mapping_taxonomy"]["const"] == "reserved_for_DEL-15-02"
    assert contract["physical_package_container"]["const"] == "TBD"
    assert contract["external_prover_status"]["const"] == (
        "not_declared_by_handoff_package"
    )

    manifest_required = required_at(schema, "HandoffPackageManifest")
    assert {
        "package_identity",
        "model_basis",
        "model_hash",
        "units_manifest",
        "entity_ids",
        "model_state_refs",
        "analysis_run_refs",
        "result_export_refs",
        "library_refs",
        "rule_pack_refs",
        "checksums",
        "target_mapping_metadata",
        "unsupported_behavior_flags",
        "unresolved_assumptions",
        "warnings",
        "diagnostics",
        "privacy",
        "redistribution_classification",
        "review_classification",
        "provenance",
        "professional_boundary",
    } <= manifest_required

    assert {
        "handoff_package_id",
        "manifest_id",
        "package_schema_version",
        "created_at",
        "created_by",
        "package_label",
        "package_kind",
    } <= required_at(schema, "PackageIdentity")

    assert {
        "model_ref",
        "model_schema",
        "model_kind",
        "basis_state_ref",
        "source_refs",
        "hash_refs",
        "provenance",
    } <= required_at(schema, "ModelBasis")
    assert defs["ModelBasis"]["properties"]["model_schema"]["const"] == (
        "schemas/model.schema.yaml"
    )

    checksum_required = required_at(schema, "Checksum")
    assert {
        "algorithm",
        "canonicalization",
        "payload_ref",
        "payload_scope",
        "value",
    } <= checksum_required
    assert {"sha256", "sha512", "TBD"} <= set(
        defs["Checksum"]["properties"]["algorithm"]["enum"]
    )
    assert {
        "JCS",
        "JCS_compatible_json_payload_hash",
        "external_file_native",
        "NONE",
        "TBD",
    } <= set(defs["Checksum"]["properties"]["canonicalization"]["enum"])
    assert {
        "handoff_package_manifest",
        "canonical_handoff_record",
        "model_payload",
        "model_hash",
        "units_manifest",
        "entity_id_manifest",
        "model_state_record",
        "analysis_run_record",
        "result_export_record",
        "library_reference_metadata",
        "rule_pack_reference_metadata",
        "local_fea_handoff_record",
        "audit_manifest",
        "external_reference",
    } <= set(defs["Checksum"]["properties"]["payload_scope"]["enum"])

    assert {
        "unit_system_ref",
        "units_schema",
        "dimension_basis",
        "coordinate_unit",
        "force_unit",
        "moment_unit",
        "displacement_unit",
        "rotation_unit",
        "stress_unit",
        "temperature_unit",
        "hash_refs",
        "provenance",
    } <= required_at(schema, "UnitsManifestRef")
    assert defs["UnitsManifestRef"]["properties"]["units_schema"]["const"] == (
        "schemas/units.schema.yaml"
    )

    assert {
        "component_ids",
        "node_ids",
        "element_ids",
        "station_ids",
        "load_case_ids",
        "result_ids",
        "model_state_ids",
        "analysis_run_ids",
        "mapping_id_refs",
    } <= required_at(schema, "EntityIdManifest")

    assert defs["ModelStateRef"]["properties"]["model_state_schema"]["const"] == (
        "schemas/model_state.schema.json"
    )
    assert defs["AnalysisRunRef"]["properties"]["analysis_run_schema"]["const"] == (
        "schemas/analysis_run.schema.json"
    )
    assert defs["ResultExportRef"]["properties"]["result_schema"]["const"] == (
        "schemas/results.schema.yaml"
    )

    assert {
        "library_ref",
        "library_kind",
        "version",
        "checksum",
        "source_notice",
        "redistribution_status",
        "review_classification",
        "privacy_classification",
        "private_payload_redacted",
        "provenance",
    } <= required_at(schema, "LibraryRef")
    assert defs["LibraryRef"]["properties"]["private_payload_redacted"]["const"] is True
    assert {
        "material",
        "section",
        "component",
        "rule_pack",
        "project_private",
        "commercial_tool_reference",
    } <= set(defs["LibraryRef"]["properties"]["library_kind"]["enum"])

    assert {
        "rule_pack_id",
        "version",
        "checksum",
        "source_notice",
        "redistribution_status",
        "review_classification",
        "privacy_classification",
        "private_payload_redacted",
        "provenance",
    } <= required_at(schema, "RulePackRef")
    assert defs["RulePackRef"]["properties"]["private_payload_redacted"]["const"] is True

    privacy = defs["PrivacyContext"]["properties"]
    assert privacy["local_only"]["const"] is True
    assert privacy["telemetry_allowed"]["const"] is False
    assert privacy["private_payload_embedded"]["const"] is False
    assert privacy["protected_payload_embedded"]["const"] is False
    assert privacy["commercial_tool_payload_embedded"]["const"] is False
    assert {
        "public_metadata",
        "invented_public_example",
        "private_project_data",
        "private_library_data",
        "private_rule_pack_data",
        "commercial_tool_metadata",
        "protected_suspected",
        "redacted",
    } <= enum_at(schema, "PrivacyClassification")

    assert {
        "public_permissive",
        "private_only",
        "unknown",
        "protected_suspected",
        "commercial_tool_reference_only",
        "invented_non_engineering_example",
    } <= enum_at(schema, "RedistributionStatus")
    assert {
        "unreviewed",
        "machine_checked",
        "human_review_required",
        "human_reviewed_reference_only",
        "quarantined",
    } <= enum_at(schema, "ReviewClassification")

    target_mapping = defs["TargetMappingMetadata"]["properties"]
    assert target_mapping["mapping_schema_status"]["const"] == "reserved_metadata_only"
    assert target_mapping["detailed_taxonomy_owner"]["const"] == "DEL-15-02"
    assert {
        "local_fea",
        "external_prover",
        "commercial_tool_reference",
        "generic_downstream_modeling",
        "TBD",
    } <= set(target_mapping["target_system_kind"]["enum"])

    flags = defs["UnsupportedBehaviorFlag"]["properties"]
    assert {
        "target_format_not_selected",
        "target_mapping_taxonomy_pending",
        "mesh_generation_not_performed",
        "external_solver_not_invoked",
        "external_prover_not_invoked",
        "commercial_tool_parser_not_implemented",
        "physical_package_not_finalized",
        "boundary_transfer_requires_review",
        "approximate_behavior_requires_review",
        "unsupported_target_behavior",
    } <= set(flags["behavior_label"]["enum"])
    assert {"unsupported", "approximate", "requires_human_review", "not_implemented"} <= set(
        flags["status"]["enum"]
    )
    assert flags["human_review_required"]["const"] is True

    assert REQUIRED_DIAGNOSTIC_CLASSES <= set(
        defs["Diagnostic"]["properties"]["class"]["enum"]
    )

    boundary = defs["ProfessionalBoundary"]["properties"]
    assert boundary["human_review_required"]["const"] is True
    assert boundary["supports_downstream_modeling"]["const"] is True
    assert boundary["supports_downstream_review"]["const"] is True
    assert boundary["software_makes_compliance_claim"]["const"] is False
    assert boundary["software_makes_certification_claim"]["const"] is False
    assert boundary["software_makes_sealing_claim"]["const"] is False
    assert boundary["software_makes_approval_claim"]["const"] is False
    assert boundary["software_makes_authentication_claim"]["const"] is False
    assert (
        boundary["software_creates_professional_reliance_record"]["const"] is False
    )

    joined_strings = "\n".join(walk_strings(schema)).lower()
    for forbidden in FORBIDDEN_SCHEMA_TEXT:
        assert forbidden.lower() not in joined_strings

    joined_enums = set()
    for definition in defs.values():
        if isinstance(definition, dict) and "enum" in definition:
            joined_enums.update(definition["enum"])
    assert joined_enums.isdisjoint(FORBIDDEN_STATUS)


if __name__ == "__main__":
    main()
