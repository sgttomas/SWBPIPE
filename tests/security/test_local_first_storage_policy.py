#!/usr/bin/env python3
"""Policy checks for the local-first storage contract."""

from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
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


def policy_text():
    return POLICY_PATH.read_text(encoding="utf-8")


def lower_policy_text():
    return policy_text().lower()


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
