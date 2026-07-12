#!/usr/bin/env python3
"""Stdlib checks for the project persistence schema."""

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCHEMA_PATH = ROOT / "schemas" / "project_persistence.schema.yaml"
FIXTURE_PATH = ROOT / "fixtures" / "persistence" / "invented_persisted_preview_project.json"


def load_schema():
    with SCHEMA_PATH.open(encoding="utf-8") as schema_file:
        return json.load(schema_file)


def load_fixture():
    with FIXTURE_PATH.open(encoding="utf-8") as fixture_file:
        return json.load(fixture_file)


def ref_name(ref):
    return ref.rsplit("/", 1)[-1]


def check_schema_contract():
    schema = load_schema()

    assert schema["$schema"] == "https://json-schema.org/draft/2020-12/schema"
    assert schema["additionalProperties"] is False

    required = set(schema["required"])
    assert {"schema_version", "project", "hash", "migration"} <= required

    properties = schema["properties"]
    assert ref_name(properties["project"]["$ref"]) == "ProjectEnvelope"
    assert ref_name(properties["hash"]["$ref"]) == "HashMetadata"
    assert ref_name(properties["migration"]["$ref"]) == "MigrationStatus"
    assert ref_name(properties["validation_profile"]["$ref"]) == "ValidationProfile"
    assert ref_name(properties["professional_boundary"]["$ref"]) == "ProfessionalBoundary"
    assert (
        ref_name(properties["service_operations"]["items"]["$ref"])
        == "PersistenceOperation"
    )
    assert ref_name(properties["external_artifacts"]["items"]["$ref"]) == (
        "ExternalArtifactReference"
    )
    assert ref_name(properties["retrieval_sidecars"]["items"]["$ref"]) == (
        "RetrievalSidecar"
    )

    defs = schema["$defs"]
    project = defs["ProjectEnvelope"]
    assert {"project_id", "unit_system_ref", "model_payload", "private_data"} <= set(
        project["required"]
    )
    assert (
        ref_name(project["properties"]["human_acceptance_refs"]["items"]["$ref"])
        == "HumanAcceptanceRef"
    )
    assert ref_name(project["properties"]["run_history"]["$ref"]) == "RunHistory"

    model_payload_ref = project["properties"]["model_payload"]["$ref"]
    assert ref_name(model_payload_ref) == "ModelPayload"
    model_payload = defs["ModelPayload"]
    assert {"$ref": "model.schema.yaml"} in model_payload["allOf"]

    hash_metadata = defs["HashMetadata"]
    assert hash_metadata["properties"]["canonicalization"]["const"] == "SORTED_COMPACT_JSON"
    assert {
        "canonicalization",
        "project_payload_hash",
        "hash_manifest",
        "payload_partition_status",
    } <= set(hash_metadata["required"])
    assert "non_json_or_binary_partition_TBD" in set(
        hash_metadata["properties"]["payload_partition_status"]["enum"]
    )
    assert "external_artifacts_by_reference" in set(
        hash_metadata["properties"]["payload_partition_status"]["enum"]
    )

    checksum = defs["Checksum"]
    assert {"algorithm", "canonicalization", "value"} <= set(checksum["required"])
    assert {
        "project_envelope",
        "project_payload",
        "model_payload",
        "rule_pack_reference",
        "input_manifest",
        "report_manifest",
        "external_artifact",
        "model_state_record",
        "analysis_run_record",
        "result_envelope",
        "result_value",
        "retrieval_sidecar_manifest",
        "TBD",
    } <= set(checksum["properties"]["payload_scope"]["enum"])

    migration = defs["MigrationStatus"]
    assert {"status", "source_schema_version", "target_schema_version"} <= set(
        migration["required"]
    )
    assert {
        "current",
        "migration_needed",
        "stale",
        "migrated",
        "unsupported_schema",
        "failed",
        "newer_than_supported",
        "TBD",
    } <= set(migration["properties"]["status"]["enum"])
    assert "db_migration_status" in migration["properties"]
    assert "product_schema_migration_status" in migration["properties"]
    assert {
        "application_service_separate_db_and_product_schema",
        "TBD",
    } <= set(migration["properties"]["migration_framework"]["enum"])

    private_data = defs["PrivateDataMarker"]
    assert {
        "classification",
        "redistribution_status",
        "default_transmission_allowed",
    } <= set(private_data["required"])
    assert private_data["properties"]["default_transmission_allowed"]["const"] is False

    physical = defs["PhysicalContainer"]
    assert {
        "status",
        "profile",
        "decision_ref",
        "storage_role",
        "canonical_truth",
        "sql_public_contract",
        "direct_sql_access_allowed",
        "hosted_db_allowed",
        "network_required",
        "sidecars_rebuildable",
    } <= set(physical["required"])
    assert "accepted" in set(physical["properties"]["status"]["enum"])
    assert "sqlite_local_project_store" in set(
        physical["properties"]["profile"]["enum"]
    )
    assert "SCA-003" in set(physical["properties"]["decision_ref"]["enum"])
    assert physical["properties"]["sql_public_contract"]["const"] is False
    assert physical["properties"]["direct_sql_access_allowed"]["const"] is False
    assert physical["properties"]["hosted_db_allowed"]["const"] is False
    assert physical["properties"]["network_required"]["const"] is False
    assert physical["properties"]["sidecars_rebuildable"]["const"] is True
    assert properties["physical_container"]["$ref"].endswith("/PhysicalContainer")

    external = defs["ExternalArtifactReference"]
    assert {
        "artifact_ref",
        "uri_or_path",
        "hash",
        "size_bytes",
        "classification",
        "verification_status",
        "copy_policy",
    } <= set(external["required"])
    assert external["properties"]["copy_policy"]["const"] == (
        "reference_in_place_by_default"
    )

    sidecar = defs["RetrievalSidecar"]
    assert {"sqlite_fts5_bm25", "numpy_vector_cache_optional"} <= set(
        sidecar["properties"]["sidecar_kind"]["enum"]
    )
    assert sidecar["properties"]["authoritative"]["const"] is False
    assert sidecar["properties"]["rebuildable"]["const"] is True
    assert sidecar["properties"]["affects_project_hash"]["const"] is False
    assert sidecar["properties"]["local_only"]["const"] is True

    round_trip = defs["RoundTripManifest"]
    assert {"serialization", "semantic_equality"} <= set(round_trip["required"])
    assert "sorted_compact_json" in round_trip["properties"]["serialization"]["enum"]
    assert {
        "schema_approved_only",
        "no_silent_engineering_defaults",
        "documented_volatile_field_exclusion",
        "TBD",
    } <= set(round_trip["properties"]["normalization_rules"]["items"]["enum"])

    validation_profile = defs["ValidationProfile"]
    assert {
        "schema_validation",
        "model_schema_delegation",
        "unit_metadata_check",
        "provenance_check",
        "private_data_check",
        "professional_boundary_check",
    } <= set(validation_profile["required"])
    assert (
        validation_profile["properties"]["model_schema_delegation"]["const"]
        == "schemas/model.schema.yaml"
    )
    assert validation_profile["properties"]["telemetry_default"]["const"] == "off"

    operation = defs["PersistenceOperation"]
    assert {
        "create_project",
        "open_project",
        "save_project",
        "validate_project",
        "version_check",
        "migrate_project",
        "TBD",
    } <= set(operation["properties"]["operation"]["enum"])
    assert operation["properties"]["boundary"]["const"] == "application_service"
    assert operation["properties"]["bypass_prohibited"]["const"] is True
    assert {
        "SCHEMA_VALIDATION",
        "MIGRATION",
        "DB_MIGRATION",
        "PRODUCT_SCHEMA_MIGRATION",
        "UNIT_METADATA",
        "PROVENANCE_WARNING",
        "RULE_CHECK_BLOCKING",
        "IP_BOUNDARY_WARNING",
        "PRIVATE_DATA",
        "EXTERNAL_ARTIFACT",
        "PROFESSIONAL_BOUNDARY",
        "TBD",
    } <= set(operation["properties"]["diagnostic_classes"]["items"]["enum"])

    human_acceptance = defs["HumanAcceptanceRef"]
    assert {
        "acceptance_ref",
        "authority_kind",
        "binding_hashes",
        "invalidates_on_hash_change",
    } <= set(human_acceptance["required"])
    assert (
        human_acceptance["properties"]["invalidates_on_hash_change"]["const"] is True
    )
    assert "external_human_review" in set(
        human_acceptance["properties"]["authority_kind"]["enum"]
    )

    boundary = defs["ProfessionalBoundary"]["properties"]
    assert boundary["human_review_required"]["const"] is True
    assert boundary["software_makes_compliance_claim"]["const"] is False
    assert boundary["software_makes_certification_claim"]["const"] is False
    assert boundary["software_makes_sealing_claim"]["const"] is False
    assert boundary["software_makes_approval_claim"]["const"] is False
    assert boundary["software_makes_authentication_claim"]["const"] is False

    run_history = defs["RunHistory"]
    assert {
        "model_state_refs",
        "analysis_run_refs",
        "result_envelope_refs",
        "result_refs",
        "hash_manifest",
    } <= set(run_history["required"])
    assert ref_name(run_history["properties"]["model_state_records"]["items"]["$ref"]) == (
        "model_state.schema.json"
    )
    assert ref_name(run_history["properties"]["analysis_run_records"]["items"]["$ref"]) == (
        "analysis_run.schema.json"
    )

    rule_pack_ref = defs["RulePackRef"]
    assert {
        "public_permissive",
        "private_only",
        "unknown",
        "protected_suspected",
        "TBD",
    } <= set(rule_pack_ref["properties"]["redistribution_status"]["enum"])


def check_persistence_fixture_contract():
    fixture = load_fixture()

    assert fixture["document_kind"] == "openpipestress.project_persistence"
    assert fixture["physical_container"]["status"] == "accepted"
    assert fixture["physical_container"]["profile"] == "sqlite_local_project_store"
    assert fixture["physical_container"]["decision_ref"] == "SCA-003"
    assert fixture["physical_container"]["direct_sql_access_allowed"] is False
    assert fixture["physical_container"]["hosted_db_allowed"] is False
    assert fixture["physical_container"]["network_required"] is False
    assert fixture["migration"]["db_migration_status"] == "current"
    assert fixture["migration"]["product_schema_migration_status"] == "current"
    assert fixture["migration"]["migration_framework"] == (
        "application_service_separate_db_and_product_schema"
    )
    assert fixture["retrieval_sidecars"][0]["sidecar_kind"] == "sqlite_fts5_bm25"
    assert fixture["retrieval_sidecars"][0]["affects_project_hash"] is False
    assert fixture["round_trip_manifest"]["serialization"] == "sorted_compact_json"
    assert "no_silent_engineering_defaults" in fixture["round_trip_manifest"][
        "normalization_rules"
    ]
    assert fixture["validation_profile"]["model_schema_delegation"] == (
        "schemas/model.schema.yaml"
    )
    assert fixture["validation_profile"]["telemetry_default"] == "off"
    assert fixture["project"]["private_data"]["default_transmission_allowed"] is False
    assert fixture["professional_boundary"]["human_review_required"] is True
    assert not fixture["professional_boundary"]["software_makes_compliance_claim"]
    assert not fixture["professional_boundary"]["software_makes_certification_claim"]
    assert not fixture["professional_boundary"]["software_makes_sealing_claim"]
    assert not fixture["professional_boundary"]["software_makes_approval_claim"]
    assert not fixture["professional_boundary"]["software_makes_authentication_claim"]

    scopes = {item["payload_scope"] for item in fixture["hash"]["hash_manifest"]}
    assert {"project_payload", "model_payload", "project_envelope"} <= scopes
    run_history = fixture["project"]["run_history"]
    assert run_history["analysis_run_refs"]
    assert run_history["result_envelope_refs"]
    assert run_history["result_refs"]
    assert "hash_manifest" in run_history


def test_project_persistence_schema_contract():
    check_schema_contract()


def test_project_persistence_fixture_preserves_foundation_boundaries():
    check_persistence_fixture_contract()


def main():
    check_schema_contract()
    check_persistence_fixture_contract()


if __name__ == "__main__":
    main()
