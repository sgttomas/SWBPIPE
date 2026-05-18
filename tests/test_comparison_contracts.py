#!/usr/bin/env python3
"""Stdlib checks for DEL-14-05 comparison mapping and tolerance schemas."""

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
MAPPING_SCHEMA_PATH = ROOT / "schemas" / "comparison_mapping.schema.json"
TOLERANCE_SCHEMA_PATH = ROOT / "schemas" / "comparison_tolerance.schema.json"

REQUIRED_MAPPING_ROOT = {
    "schema_version",
    "deliverable_id",
    "package_id",
    "scope_item",
    "objectives",
    "comparison_contract_status",
    "comparison_review",
}

REQUIRED_TOLERANCE_ROOT = {
    "schema_version",
    "deliverable_id",
    "package_id",
    "scope_item",
    "objectives",
    "tolerance_contract_status",
    "tolerance_profile",
}

REQUIRED_MAPPING_DEFS = {
    "AffectedRef",
    "AssumptionRecord",
    "Checksum",
    "ComparisonContractStatus",
    "ComparisonParticipant",
    "ComparisonReview",
    "CsvExportContract",
    "Diagnostic",
    "ExportContract",
    "Id",
    "JsonExportContract",
    "MappingConfidence",
    "MappingRecord",
    "MappingStatus",
    "PrivacyClassification",
    "ProfessionalBoundary",
    "Provenance",
    "RedistributionStatus",
    "Reference",
    "ReportSectionExportRef",
    "ReviewMetadata",
    "ReviewStatus",
    "StableRecordReference",
    "UnmatchedClassification",
    "UnmatchedRecord",
}

REQUIRED_TOLERANCE_DEFS = {
    "AssumptionRecord",
    "Checksum",
    "Diagnostic",
    "DimensionId",
    "Id",
    "PrivacyClassification",
    "ProfessionalBoundary",
    "Provenance",
    "RedistributionStatus",
    "Reference",
    "ReviewMetadata",
    "ReviewStatus",
    "ToleranceContractStatus",
    "ToleranceProfile",
    "ToleranceRule",
}

FORBIDDEN_SCHEMA_TEXT = {
    "formal " + "acceptance",
    "code " + "compliant",
    "certified " + "by software",
    "sealed " + "by software",
    "professional approval " + "by the software",
    "commercial prover " + "ingestion",
}


def load_json(path):
    with path.open(encoding="utf-8") as schema_file:
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


def assert_common_root(schema, required_root):
    assert schema["$schema"] == "https://json-schema.org/draft/2020-12/schema"
    assert schema["additionalProperties"] is False
    assert "default" not in set(walk_keys(schema))
    assert required_root <= set(schema["required"])
    assert schema["properties"]["deliverable_id"]["const"] == "DEL-14-05"
    assert schema["properties"]["package_id"]["const"] == "PKG-14"
    assert schema["properties"]["scope_item"]["const"] == "SOW-073"
    assert schema["properties"]["objectives"]["contains"]["const"] == "OBJ-016"


def assert_professional_boundary(schema):
    professional = schema["$defs"]["ProfessionalBoundary"]["properties"]
    assert professional["human_review_required"]["const"] is True
    assert professional["software_makes_compliance_claim"]["const"] is False
    assert professional["software_makes_certification_claim"]["const"] is False
    assert professional["software_makes_sealing_claim"]["const"] is False
    assert professional["software_makes_approval_claim"]["const"] is False
    assert professional["software_makes_authentication_claim"]["const"] is False


def assert_forbidden_text_absent(*schemas):
    joined_strings = "\n".join(
        string.lower() for schema in schemas for string in walk_strings(schema)
    )
    for forbidden in FORBIDDEN_SCHEMA_TEXT:
        assert forbidden.lower() not in joined_strings


def test_mapping_schema_contract():
    schema = load_json(MAPPING_SCHEMA_PATH)
    defs = schema["$defs"]

    assert_common_root(schema, REQUIRED_MAPPING_ROOT)
    assert REQUIRED_MAPPING_DEFS <= set(defs)

    contract = defs["ComparisonContractStatus"]["properties"]
    assert contract["record_contract"]["const"] == (
        "schema_first_comparison_mapping_review_records"
    )
    assert contract["model_state_binding"]["const"] == "schemas/model_state.schema.json"
    assert contract["analysis_run_binding"]["const"] == "schemas/analysis_run.schema.json"
    assert contract["result_envelope_binding"]["const"] == "schemas/results.schema.yaml"
    assert contract["tolerance_profile_binding"]["const"] == (
        "schemas/comparison_tolerance.schema.json"
    )
    assert contract["export_contract_scope"]["const"] == "json_csv_review_contract_only"
    assert contract["report_rendering"]["const"] == (
        "reserved_reference_only_not_implemented"
    )
    assert contract["comparison_engine"]["const"] == "not_implemented_by_this_contract"
    assert contract["external_validation_boundary"]["const"] == (
        "reference_only_not_determined_by_software"
    )

    participant_required = required_at(schema, "ComparisonParticipant")
    assert {
        "participant_id",
        "side",
        "model_state_ref",
        "analysis_run_ref",
        "result_envelope_ref",
        "unit_system_ref",
        "hash_refs",
        "provenance",
    } <= participant_required
    stable_ref = defs["StableRecordReference"]["properties"]
    assert {
        "model_state_record",
        "analysis_run_record",
        "result_export_envelope",
    } <= set(stable_ref["record_type"]["enum"])
    assert {
        "schemas/model_state.schema.json",
        "schemas/analysis_run.schema.json",
        "schemas/results.schema.yaml",
    } <= set(stable_ref["schema_ref"]["enum"])

    mapping_status = enum_at(schema, "MappingStatus")
    assert {
        "automatic_match",
        "manual_match",
        "unresolved_mapping",
        "unmatched_left",
        "unmatched_right",
        "ignored",
        "TBD",
    } <= mapping_status
    mapping_required = required_at(schema, "MappingRecord")
    assert {
        "mapping_id",
        "mapping_kind",
        "mapping_status",
        "left_ref",
        "right_ref",
        "affected_refs",
        "confidence",
        "review",
        "provenance",
    } <= mapping_required
    assert {
        "exact_stable_id",
        "manual_reviewed",
        "manual_unreviewed",
        "heuristic_candidate",
        "unresolved",
    } <= set(defs["MappingConfidence"]["properties"]["confidence_level"]["enum"])

    assert {
        "left_only",
        "right_only",
        "missing_counterpart",
        "intentionally_ignored",
        "not_comparable",
        "scope_excluded",
        "unresolved_TBD",
    } <= enum_at(schema, "UnmatchedClassification")
    assert {
        "unmatched_id",
        "classification",
        "subject_ref",
        "affected_refs",
        "review",
        "provenance",
    } <= required_at(schema, "UnmatchedRecord")

    json_export = defs["JsonExportContract"]["properties"]
    for key in {
        "stable_ids_required",
        "mapping_ids_required",
        "unit_metadata_required",
        "tolerance_profile_refs_required",
        "diagnostics_required",
        "provenance_required",
        "assumptions_required",
        "hash_refs_required",
        "professional_boundary_notice_required",
    }:
        assert json_export[key]["const"] is True
    csv_columns = set(defs["CsvExportContract"]["properties"]["required_columns"]["items"]["enum"])
    assert {
        "review_row_id",
        "comparison_id",
        "mapping_id",
        "unit",
        "dimension",
        "tolerance_profile_ref",
        "diagnostic_codes",
        "provenance_ref",
        "assumption_ids",
        "professional_boundary_notice",
    } <= csv_columns
    assert (
        defs["ReportSectionExportRef"]["properties"]["rendering_status"]["const"]
        == "reserved_reference_only_not_implemented"
    )
    assert_professional_boundary(schema)


def test_tolerance_schema_contract():
    schema = load_json(TOLERANCE_SCHEMA_PATH)
    defs = schema["$defs"]

    assert_common_root(schema, REQUIRED_TOLERANCE_ROOT)
    assert REQUIRED_TOLERANCE_DEFS <= set(defs)

    contract = defs["ToleranceContractStatus"]["properties"]
    assert contract["record_contract"]["const"] == (
        "schema_first_unit_aware_tolerance_profiles"
    )
    assert contract["unit_system_binding"]["const"] == "schemas/units.schema.yaml"
    assert contract["comparison_mapping_binding"]["const"] == (
        "schemas/comparison_mapping.schema.json"
    )
    assert contract["default_numeric_tolerances"]["const"] == (
        "not_defined_by_this_contract"
    )
    assert contract["result_delta_engine"]["const"] == "not_implemented_by_this_contract"
    assert contract["external_validation_boundary"]["const"] == (
        "reference_only_not_determined_by_software"
    )

    profile_required = required_at(schema, "ToleranceProfile")
    assert {
        "profile_id",
        "profile_name",
        "profile_status",
        "unit_system_ref",
        "applicable_dimensions",
        "rules",
        "diagnostics",
        "assumptions",
        "hashes",
        "review",
        "professional_boundary",
        "provenance",
    } <= profile_required
    assert {
        "dimensionless",
        "length",
        "force",
        "moment",
        "stress",
        "displacement",
        "rotation",
        "ratio",
    } <= enum_at(schema, "DimensionId")

    rule_required = required_at(schema, "ToleranceRule")
    assert {
        "rule_id",
        "result_family",
        "dimension_id",
        "unit_ref",
        "quantity_kind",
        "tolerance_value",
        "tolerance_value_status",
        "normalization_basis",
        "review",
        "provenance",
    } <= rule_required
    tolerance_value = defs["ToleranceRule"]["properties"]["tolerance_value"]
    assert "default" not in set(walk_keys(tolerance_value))
    assert {"TBD", "externally_governed_reference_required"} <= set(
        tolerance_value["oneOf"][1]["enum"]
    )
    assert {
        "externally_governed",
        "project_specific_review_required",
        "not_defined",
        "TBD",
    } <= set(defs["ToleranceRule"]["properties"]["tolerance_value_status"]["enum"])
    assert {
        "same_unit_required",
        "unit_conversion_required",
        "dimensionless_direct",
        "not_applicable",
        "TBD",
    } <= set(defs["ToleranceRule"]["properties"]["normalization_basis"]["enum"])
    assert_professional_boundary(schema)


def test_contracts_avoid_prohibited_scope_and_defaults():
    mapping = load_json(MAPPING_SCHEMA_PATH)
    tolerance = load_json(TOLERANCE_SCHEMA_PATH)

    assert_forbidden_text_absent(mapping, tolerance)
    all_keys = set(walk_keys(mapping)) | set(walk_keys(tolerance))
    assert "default" not in all_keys


if __name__ == "__main__":
    test_mapping_schema_contract()
    test_tolerance_schema_contract()
    test_contracts_avoid_prohibited_scope_and_defaults()
