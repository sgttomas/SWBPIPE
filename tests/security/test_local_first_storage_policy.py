#!/usr/bin/env python3
"""Policy checks for the local-first storage contract."""

import json
from pathlib import Path
import sys


ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT))

from core.security.local_first_storage import (  # noqa: E402
    classify_storage_record,
    guard_storage_records,
    storage_record,
)

POLICY_PATH = ROOT / "docs" / "security" / "local_first_storage_policy.md"

TRACEABILITY = {
    "deliverable_id: DEL-12-01",
    "package_id: PKG-12",
    "SOW-029",
    "OBJ-010",
}

LOCAL_FIRST_PRIVATE_CLASSES = {
    "Private project models",
    "private rule packs",
    "private material and component libraries",
    "owner standards",
    "company design bases",
    "credentials",
    "secrets",
    "diagnostics",
    "reports",
    "generated outputs",
    "user controlled",
}

SYMBOLIC_PATH_CLASSES = {
    "PUBLIC_REPOSITORY_CONTENT",
    "PUBLIC_EXAMPLE_CONTENT",
    "USER_CHOSEN_PROJECT_PACKAGE",
    "USER_PRIVATE_LIBRARY_ROOT",
    "USER_PRIVATE_RULE_PACK_ROOT",
    "USER_REPORT_OUTPUT_ROOT",
    "USER_DIAGNOSTIC_BUNDLE_ROOT",
    "USER_IMPORT_STAGING_ROOT",
    "USER_EXPORT_STAGING_ROOT",
    "LOCAL_CACHE_ROOT",
    "USER_SECRET_REFERENCE",
}

PERSISTENCE_BASELINE = {
    "versioned persistence",
    "schema-governed payloads",
    "unit-aware data",
    "provenance-preserving records",
    "migration-aware status",
    "deterministic round-trip serialization",
    "canonical JSON/JCS-compatible hashes",
    "local SQLite storage as a payload/index substrate",
    "rebuildable, hash-neutral SQLite FTS5/BM25 retrieval sidecars",
}

OPEN_DECISIONS = {
    "Operating-system roots and application data directories",
    "DB migration and product schema migration framework",
    "Encryption, secret storage, and key management",
    "Redaction workflow and export staging behavior",
    "Private-library registry and secret/private-library handling",
    "Import/export formats and adapter behavior",
    "Cloud exception workflow",
}

NO_BYPASS_SURFACES = {
    "plugins",
    "adapters",
    "import/export paths",
    "reports",
    "telemetry",
    "CLI runners",
    "diagnostics",
    "tests",
    "application services",
}

REAL_PATH_MARKERS = {
    "/Users/",
    "/home/",
    "C:\\",
    "file://",
    "s3://",
    "gs://",
    "https://",
    "http://",
}

DISALLOWED_COMMITMENTS = {
    "cloud storage is enabled",
    "cloud sync is enabled",
    "hosted database is enabled",
    "network is required",
    "encryption is provided",
    "secrets are stored in",
    "default path is",
    "storage root is",
}


def decision_codes(result):
    return {decision.reason_code for decision in result.decisions}


def diagnostic_codes(result):
    return {diagnostic.code for diagnostic in result.diagnostics}


def invented_project_storage_record():
    return storage_record(
        record_id="invented.project.metadata",
        record_kind="project_model",
        label="Invented project metadata reference",
        checksum="sha256:0000000000000000000000000000000000000000000000000000000000000000",
        checksum_status="placeholder_checksum_recorded",
        source_note="invented local placeholder source note",
        redistribution_status="private_only",
        review_status="pending",
    )


def policy_text():
    return POLICY_PATH.read_text(encoding="utf-8")


def lower_policy_text():
    return policy_text().lower()


def test_storage_record_classification_is_deterministic_and_metadata_only():
    first = classify_storage_record(invented_project_storage_record())
    second = classify_storage_record(invented_project_storage_record())

    assert first == second
    assert first.decision_id.startswith("LFS-CLS-")
    assert first.reason_code == "PRIVATE_METADATA_LOCAL_FIRST"
    assert first.default_posture == "local_private_metadata_requires_explicit_intent"
    assert first.metadata["storage_locality"] == "USER_CHOSEN_PROJECT_PACKAGE"
    assert first.metadata["contains_payload"] is False
    assert first.metadata["secret_material_present"] is False
    assert first.metadata["direct_sql_access"] is False


def test_public_repository_blocks_private_storage_payloads_without_leaking_values():
    unsafe_records = [
        {
            "record_id": "invented.project.payload",
            "record_kind": "project_model",
            "label": "Invented project payload",
            "privacy_classification": "private_project_data",
            "redistribution_status": "private_only",
            "review_status": "pending",
            "source_state": "private_user_supplied",
            "project_payload": {"invented": "PROJECT_PAYLOAD_SHOULD_NOT_SURVIVE"},
        },
        {
            "record_id": "invented.rule.payload",
            "record_kind": "private_rule_pack",
            "label": "Invented rule payload",
            "privacy_classification": "private_rule_pack_data",
            "redistribution_status": "private_only",
            "review_status": "pending",
            "source_state": "private_user_supplied",
            "rule_payload": "RULE_PAYLOAD_SHOULD_NOT_SURVIVE",
        },
        {
            "record_id": "invented.material.payload",
            "record_kind": "private_material_library",
            "label": "Invented material payload",
            "privacy_classification": "private_material_data",
            "redistribution_status": "private_only",
            "review_status": "pending",
            "source_state": "private_user_supplied",
            "material_values": {"invented": "MATERIAL_PAYLOAD_SHOULD_NOT_SURVIVE"},
        },
        {
            "record_id": "invented.component.payload",
            "record_kind": "private_component_library",
            "label": "Invented component payload",
            "privacy_classification": "private_component_data",
            "redistribution_status": "private_only",
            "review_status": "pending",
            "source_state": "private_user_supplied",
            "component_values": {"invented": "COMPONENT_PAYLOAD_SHOULD_NOT_SURVIVE"},
        },
        {
            "record_id": "invented.report.payload",
            "record_kind": "private_report",
            "label": "Invented report payload",
            "privacy_classification": "private_report_data",
            "redistribution_status": "private_only",
            "review_status": "pending",
            "source_state": "private_user_supplied",
            "report_payload": "REPORT_PAYLOAD_SHOULD_NOT_SURVIVE",
        },
        {
            "record_id": "invented.diagnostic.payload",
            "record_kind": "private_diagnostic",
            "label": "Invented diagnostic payload",
            "privacy_classification": "private_diagnostic_data",
            "redistribution_status": "private_only",
            "review_status": "pending",
            "source_state": "private_user_supplied",
            "diagnostic_payload": "DIAGNOSTIC_PAYLOAD_SHOULD_NOT_SURVIVE",
        },
        {
            "record_id": "invented.cache.payload",
            "record_kind": "private_cache",
            "label": "Invented cache payload",
            "privacy_classification": "private_cache_data",
            "redistribution_status": "private_only",
            "review_status": "pending",
            "source_state": "private_user_supplied",
            "cache_payload": "CACHE_PAYLOAD_SHOULD_NOT_SURVIVE",
        },
        {
            "record_id": "invented.secret.payload",
            "record_kind": "secret_reference",
            "label": "Invented secret payload",
            "privacy_classification": "secret_like_data",
            "redistribution_status": "private_only",
            "review_status": "pending",
            "source_state": "credential_reference",
            "secret_value": "SECRET_PAYLOAD_SHOULD_NOT_SURVIVE",
        },
    ]

    result = guard_storage_records(
        unsafe_records,
        target_context="public_repository",
    )
    serialized = json.dumps(result.as_schema_dict(), sort_keys=True)

    assert result.blocked is True
    assert result.summary()["metadata_only"] is True
    assert decision_codes(result) == {
        "STORAGE_PAYLOAD_METADATA_ONLY_REQUIRED",
        "SECRET_MATERIAL_METADATA_ONLY_REQUIRED",
    }
    assert "STORAGE_PAYLOAD_METADATA_ONLY_REQUIRED" in diagnostic_codes(result)
    for leaked in {
        "PROJECT_PAYLOAD_SHOULD_NOT_SURVIVE",
        "RULE_PAYLOAD_SHOULD_NOT_SURVIVE",
        "MATERIAL_PAYLOAD_SHOULD_NOT_SURVIVE",
        "COMPONENT_PAYLOAD_SHOULD_NOT_SURVIVE",
        "REPORT_PAYLOAD_SHOULD_NOT_SURVIVE",
        "DIAGNOSTIC_PAYLOAD_SHOULD_NOT_SURVIVE",
        "CACHE_PAYLOAD_SHOULD_NOT_SURVIVE",
        "SECRET_PAYLOAD_SHOULD_NOT_SURVIVE",
    }:
        assert leaked not in serialized


def test_local_private_storage_requires_explicit_user_intent_for_private_metadata():
    blocked = guard_storage_records(
        [invented_project_storage_record()],
        target_context="local_private",
        explicit_user_intent=False,
    )
    allowed = guard_storage_records(
        [invented_project_storage_record()],
        target_context="local_private",
        explicit_user_intent=True,
    )

    assert blocked.blocked is True
    assert "LOCAL_PRIVATE_INTENT_REQUIRED" in decision_codes(blocked)
    assert allowed.blocked is False
    assert "PRIVATE_LOCAL_METADATA_ALLOWED" in decision_codes(allowed)
    assert allowed.safe_manifest[0]["record_id"] == "invented.project.metadata"
    assert allowed.safe_manifest[0]["contains_payload"] is False


def test_public_repository_locality_blocks_even_with_local_private_intent():
    result = guard_storage_records(
        [
            storage_record(
                record_id="invented.public.repo.private.target",
                record_kind="project_model",
                label="Invented public repository private target",
                storage_locality="PUBLIC_REPOSITORY_CONTENT",
                redistribution_status="private_only",
                review_status="pending",
                source_note="invented local placeholder source note",
            )
        ],
        target_context="local_private",
        explicit_user_intent=True,
    )

    assert result.blocked is True
    assert result.decisions[0].blocked is True
    assert "PUBLIC_REPOSITORY_PRIVATE_STORAGE_BLOCKED" in diagnostic_codes(result)


def test_cloud_network_direct_sql_secret_and_concrete_path_details_are_sanitized():
    records = [
        {
            "record_id": "invented.cloud.reference",
            "record_kind": "project_model",
            "label": "Invented cloud reference",
            "privacy_classification": "private_project_data",
            "redistribution_status": "private_only",
            "review_status": "pending",
            "source_state": "private_user_supplied",
            "cloud_or_network_reference": True,
            "cloud_sync_target": "CLOUD_SYNC_TARGET_SHOULD_NOT_SURVIVE",
        },
        {
            "record_id": "invented.direct.sql",
            "record_kind": "project_store",
            "label": "Invented direct SQL reference",
            "privacy_classification": "private_project_data",
            "redistribution_status": "private_only",
            "review_status": "pending",
            "source_state": "private_user_supplied",
            "table_name": "SQL_TABLE_DETAIL_SHOULD_NOT_SURVIVE",
        },
        {
            "record_id": "invented.secret.material",
            "record_kind": "secret_reference",
            "label": "Invented secret material",
            "privacy_classification": "secret_like_data",
            "redistribution_status": "private_only",
            "review_status": "pending",
            "source_state": "credential_reference",
            "api_key": "SECRET_MATERIAL_SHOULD_NOT_SURVIVE",
        },
        {
            "record_id": "invented.concrete.path",
            "record_kind": "private_report",
            "label": "Invented concrete path reference",
            "privacy_classification": "private_report_data",
            "redistribution_status": "private_only",
            "review_status": "pending",
            "source_state": "private_user_supplied",
            "file_path": "FAKE_CONCRETE_PRIVATE_PATH_SHOULD_NOT_SURVIVE",
        },
    ]

    result = guard_storage_records(
        records,
        target_context="local_private",
        explicit_user_intent=True,
    )
    serialized = json.dumps(result.as_schema_dict(), sort_keys=True)

    assert result.blocked is True
    assert "CLOUD_OR_NETWORK_STORAGE_BLOCKED" in decision_codes(result)
    assert "DIRECT_SQL_ACCESS_BLOCKED" in decision_codes(result)
    assert "SECRET_MATERIAL_METADATA_ONLY_REQUIRED" in decision_codes(result)
    assert "CONCRETE_PATH_REDUCED_TO_SAFE_METADATA" in decision_codes(result)
    assert any(
        decision.record_id == "invented.concrete.path"
        and decision.action == "include_metadata_only"
        for decision in result.decisions
    )
    for leaked in {
        "CLOUD_SYNC_TARGET_SHOULD_NOT_SURVIVE",
        "SQL_TABLE_DETAIL_SHOULD_NOT_SURVIVE",
        "SECRET_MATERIAL_SHOULD_NOT_SURVIVE",
        "FAKE_CONCRETE_PRIVATE_PATH_SHOULD_NOT_SURVIVE",
    }:
        assert leaked not in serialized


def test_policy_is_traceable_to_deliverable_scope():
    text = policy_text()
    for required in TRACEABILITY:
        assert required in text


def test_local_first_user_control_default_is_explicit():
    text = policy_text()
    assert "OpenPipeStress is local-first by default." in text
    for required in LOCAL_FIRST_PRIVATE_CLASSES:
        assert required in text


def test_repository_is_not_default_private_storage():
    text = policy_text()
    assert "The public repository is not a default durable storage location" in text
    assert "Public repository paths must not be used as default durable storage" in text


def test_symbolic_path_classes_are_defined_without_real_paths():
    text = policy_text()
    for path_class in SYMBOLIC_PATH_CLASSES:
        assert path_class in text
    assert "These names are planning classes, not filesystem paths." in text
    for marker in REAL_PATH_MARKERS:
        assert marker not in text


def test_unresolved_storage_choices_remain_tbd_or_findings():
    text = policy_text()
    assert "remain explicit warning, finding, or unsupported" in text
    for decision in OPEN_DECISIONS:
        assert decision in text


def test_persistence_baseline_is_preserved():
    text = policy_text()
    for required in PERSISTENCE_BASELINE:
        assert required in text
    assert "SCA-003 local SQLite-backed project store/index" in text
    assert "Canonical JSON/JCS-compatible payload bytes remain the domain and interchange" in text
    assert "SQLite tables are storage/projection details, not public contracts." in text
    assert "must not be copied" in text
    assert "explicit export/review workflow" in text


def test_no_bypass_surfaces_are_listed():
    text = policy_text()
    for surface in NO_BYPASS_SURFACES:
        assert surface in text
    assert "No plugin manifest, adapter declaration, CLI option" in text
    assert "can bypass" in text
    assert "direct SQL, raw SQLite handles, table-name dependencies" in text


def test_no_cloud_encryption_or_runtime_storage_commitment():
    lowered = lower_policy_text()
    assert "hosted database" in lowered
    assert "required network" in lowered
    assert "cloud sync" in lowered
    assert "no encryption or secret-storage claim is made here" in lowered
    assert "free to use" in lowered
    assert "usable offline" in lowered
    for disallowed in DISALLOWED_COMMITMENTS:
        assert disallowed not in lowered


def test_no_runtime_storage_code_or_schema_was_added():
    assert not (ROOT / "core" / "storage").exists()
    assert not (ROOT / "apps" / "storage").exists()
    assert not (ROOT / "schemas" / "storage.schema.yaml").exists()
