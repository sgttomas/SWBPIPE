#!/usr/bin/env python3
"""Stdlib checks for the immutable model state schema."""

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCHEMA_PATH = ROOT / "schemas" / "model_state.schema.json"

REQUIRED_ROOT = {
    "schema_version",
    "deliverable_id",
    "package_id",
    "scope_item",
    "objectives",
    "state_contract_status",
    "model_state",
}

REQUIRED_DEFS = {
    "AnalysisStatus",
    "AssumptionRecord",
    "Checksum",
    "Diagnostic",
    "ExternalReference",
    "Id",
    "ImmutabilityPolicy",
    "ModelStateRecord",
    "PhysicalProjectContainer",
    "PrivacyClassification",
    "ProfessionalBoundary",
    "Provenance",
    "RedistributionStatus",
    "Reference",
    "ReviewStatus",
    "StateContractStatus",
    "StateNote",
    "StateTag",
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

    assert schema["properties"]["deliverable_id"]["const"] == "DEL-14-01"
    assert schema["properties"]["package_id"]["const"] == "PKG-14"
    assert schema["properties"]["scope_item"]["const"] == "SOW-071"
    assert schema["properties"]["objectives"]["contains"]["const"] == "OBJ-016"

    contract = defs["StateContractStatus"]["properties"]
    assert contract["record_contract"]["const"] == "schema_first_model_state_records"
    assert contract["persistence_binding"]["const"] == (
        "schemas/project_persistence.schema.yaml"
    )
    assert contract["canonicalization"]["const"] == "JCS_compatible_json_payload_hashes"
    assert (
        contract["physical_project_container"]["$ref"]
        == "#/$defs/PhysicalProjectContainer"
    )
    assert contract["external_human_acceptance"]["const"] == (
        "hash_bound_external_record_only"
    )

    physical = defs["PhysicalProjectContainer"]["properties"]
    assert physical["profile"]["const"] == "sqlite_local_project_store"
    assert physical["decision_ref"]["const"] == "SCA-003"
    assert physical["storage_role"]["const"] == "local_store_index_projection"
    assert physical["canonical_truth"]["const"] == "canonical_json_jcs_payload"
    assert physical["sql_public_contract"]["const"] is False
    assert physical["direct_sql_access_allowed"]["const"] is False
    assert physical["hosted_db_allowed"]["const"] is False
    assert physical["network_required"]["const"] is False
    assert physical["sidecars_rebuildable"]["const"] is True

    state_required = required_at(schema, "ModelStateRecord")
    assert {
        "state_id",
        "state_name",
        "state_kind",
        "created_at",
        "model_ref",
        "parent_state_refs",
        "tags",
        "notes",
        "external_references",
        "unresolved_assumptions",
        "warnings",
        "analysis_status",
        "hashes",
        "immutability_policy",
        "professional_boundary",
        "provenance",
    } <= state_required
    assert {
        "design_snapshot",
        "pre_solve_snapshot",
        "post_solve_snapshot",
        "comparison_basis",
        "handoff_basis",
    } <= set(defs["ModelStateRecord"]["properties"]["state_kind"]["enum"])

    immutability = defs["ImmutabilityPolicy"]["properties"]
    assert immutability["snapshot_is_read_only"]["const"] is True
    assert immutability["mutation_policy"]["const"] == "changes_create_new_model_state"
    assert immutability["new_state_required_for_change"]["const"] is True
    assert immutability["hash_invalidates_external_acceptance"]["const"] is True

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

    checksum_required = required_at(schema, "Checksum")
    assert {
        "algorithm",
        "canonicalization",
        "payload_ref",
        "payload_scope",
        "value",
    } <= checksum_required
    assert {"JCS", "NONE", "TBD"} <= set(
        defs["Checksum"]["properties"]["canonicalization"]["enum"]
    )
    assert {
        "model_state_record",
        "model_payload",
        "state_payload_partition",
        "external_reference",
        "audit_manifest",
        "TBD",
    } <= set(defs["Checksum"]["properties"]["payload_scope"]["enum"])

    assert {
        "document",
        "drawing",
        "model_file",
        "audit_manifest",
        "review_record",
        "external_file",
    } <= set(defs["ExternalReference"]["properties"]["reference_type"]["enum"])

    professional = defs["ProfessionalBoundary"]["properties"]
    assert professional["human_review_required"]["const"] is True
    assert professional["software_makes_compliance_claim"]["const"] is False
    assert professional["software_makes_certification_claim"]["const"] is False
    assert professional["software_makes_sealing_claim"]["const"] is False
    assert professional["software_makes_approval_claim"]["const"] is False
    assert professional["software_makes_authentication_claim"]["const"] is False

    provenance_required = required_at(schema, "Provenance")
    assert {
        "source_name",
        "source_location",
        "source_license",
        "contributor",
        "contributor_certification",
        "redistribution_status",
        "review_status",
        "privacy_classification",
    } <= provenance_required
    assert {
        "public_permissive",
        "private_only",
        "unknown",
        "protected_suspected",
        "invented_non_engineering_example",
    } <= enum_at(schema, "RedistributionStatus")

    joined_strings = "\n".join(walk_strings(schema)).lower()
    for forbidden in FORBIDDEN_SCHEMA_TEXT:
        assert forbidden.lower() not in joined_strings


if __name__ == "__main__":
    main()
