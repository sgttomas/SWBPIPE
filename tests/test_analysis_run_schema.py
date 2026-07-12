#!/usr/bin/env python3
"""Stdlib checks for the analysis run schema."""

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCHEMA_PATH = ROOT / "schemas" / "analysis_run.schema.json"

REQUIRED_ROOT = {
    "schema_version",
    "deliverable_id",
    "package_id",
    "scope_item",
    "objectives",
    "run_contract_status",
    "analysis_run",
}

REQUIRED_DEFS = {
    "AnalysisRunRecord",
    "AnalysisStatus",
    "Checksum",
    "Diagnostic",
    "Id",
    "ImmutabilityPolicy",
    "LibraryRef",
    "PrivacyClassification",
    "ProfessionalBoundary",
    "Provenance",
    "RedistributionStatus",
    "Reference",
    "Reproducibility",
    "ResultRef",
    "ReviewStatus",
    "RulePackRef",
    "RunContractStatus",
    "SolverVersion",
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

    assert schema["properties"]["deliverable_id"]["const"] == "DEL-14-02"
    assert schema["properties"]["package_id"]["const"] == "PKG-14"
    assert schema["properties"]["scope_item"]["const"] == "SOW-072"
    assert schema["properties"]["objectives"]["contains"]["const"] == "OBJ-016"

    contract = defs["RunContractStatus"]["properties"]
    assert contract["record_contract"]["const"] == "schema_first_analysis_run_records"
    assert contract["model_state_binding"]["const"] == "schemas/model_state.schema.json"
    assert contract["result_binding"]["const"] == "schemas/results.schema.yaml"
    assert (
        contract["physical_project_container"]["$ref"]
        == "#/$defs/PhysicalProjectContainer"
    )
    physical = defs["PhysicalProjectContainer"]["properties"]
    assert physical["profile"]["const"] == "sqlite_local_project_store"
    assert physical["decision_ref"]["const"] == "SCA-003"
    assert physical["storage_role"]["const"] == "local_store_index_projection"
    assert {
        "sorted_compact_json_payload",
        "canonical_json_jcs_payload",
    } <= set(physical["canonical_truth"]["enum"])
    assert "does not describe the current DEL-14-02 serializer" in (
        physical["canonical_truth"]["description"]
    )
    assert physical["sql_public_contract"]["const"] is False
    assert physical["direct_sql_access_allowed"]["const"] is False
    assert physical["hosted_db_allowed"]["const"] is False
    assert physical["network_required"]["const"] is False
    assert physical["sidecars_rebuildable"]["const"] is True
    assert contract["external_validation_boundary"]["const"] == (
        "reference_only_not_determined_by_software"
    )

    run_required = required_at(schema, "AnalysisRunRecord")
    assert {
        "run_id",
        "run_name",
        "run_kind",
        "created_at",
        "model_state_ref",
        "solver_version",
        "settings_ref",
        "unit_system_ref",
        "load_basis_refs",
        "diagnostics",
        "result_refs",
        "rule_pack_refs",
        "library_refs",
        "hashes",
        "analysis_status",
        "reproducibility",
        "immutability_policy",
        "professional_boundary",
        "provenance",
    } <= run_required
    assert {
        "mechanics_solve",
        "rule_check",
        "combined_analysis",
        "export_generation",
        "comparison_input",
    } <= set(defs["AnalysisRunRecord"]["properties"]["run_kind"]["enum"])

    status = enum_at(schema, "AnalysisStatus")
    assert {
        "MODEL_INCOMPLETE",
        "MECHANICS_SOLVED",
        "RULE_INPUTS_INCOMPLETE",
        "USER_RULE_CHECKED",
        "USER_RULE_FAILED",
        "HUMAN_REVIEW_REQUIRED",
    } <= status
    assert status.isdisjoint(FORBIDDEN_STATUS)
    assert (
        defs["AnalysisRunRecord"]["properties"]["analysis_status"]["contains"]["const"]
        == "HUMAN_REVIEW_REQUIRED"
    )

    checksum_required = required_at(schema, "Checksum")
    assert {
        "algorithm",
        "canonicalization",
        "payload_ref",
        "payload_scope",
        "value",
    } <= checksum_required
    canonicalizations = set(
        defs["Checksum"]["properties"]["canonicalization"]["enum"]
    )
    assert {"SORTED_COMPACT_JSON", "JCS", "NONE", "TBD"} <= canonicalizations
    assert "does not claim RFC 8785/JCS conformance" in (
        defs["Checksum"]["properties"]["canonicalization"]["description"]
    )
    assert {
        "analysis_run_record",
        "model_state_record",
        "solver_settings",
        "load_basis",
        "result_envelope",
        "result_value",
        "audit_manifest",
    } <= set(defs["Checksum"]["properties"]["payload_scope"]["enum"])

    immutability = defs["ImmutabilityPolicy"]["properties"]
    assert immutability["run_record_is_read_only"]["const"] is True
    assert immutability["mutation_policy"]["const"] == (
        "changes_create_new_analysis_run"
    )
    assert immutability["new_run_required_for_change"]["const"] is True
    assert immutability["hash_invalidates_external_acceptance"]["const"] is True

    assert {
        "input_manifest_refs",
        "environment_refs",
        "determinism_notes",
        "unresolved_tbd",
    } <= required_at(schema, "Reproducibility")
    assert "Result" in set(defs["Reference"]["properties"]["object_type"]["enum"])
    assert {
        "result_ref",
        "result_family",
        "hash_refs",
        "privacy_classification",
        "provenance",
    } <= required_at(schema, "ResultRef")
    assert {
        "rule_pack_id",
        "version",
        "checksum",
        "source_notice",
        "redistribution_status",
        "private_payload_redacted",
        "provenance",
    } <= required_at(schema, "RulePackRef")
    assert defs["RulePackRef"]["properties"]["private_payload_redacted"]["const"] is True

    professional = defs["ProfessionalBoundary"]["properties"]
    assert professional["human_review_required"]["const"] is True
    assert professional["software_makes_compliance_claim"]["const"] is False
    assert professional["software_makes_certification_claim"]["const"] is False
    assert professional["software_makes_sealing_claim"]["const"] is False
    assert professional["software_makes_approval_claim"]["const"] is False
    assert professional["software_makes_authentication_claim"]["const"] is False

    joined_strings = "\n".join(walk_strings(schema)).lower()
    for forbidden in FORBIDDEN_SCHEMA_TEXT:
        assert forbidden.lower() not in joined_strings


if __name__ == "__main__":
    main()
