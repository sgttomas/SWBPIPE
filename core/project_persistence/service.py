"""Schema-shaped project persistence helpers for TP-PER-01.

This module builds deterministic project persistence envelopes for invented or
cleared payloads and implements the SCA-003 MVP local SQLite store/index
profile. SQLite is used as a local payload container and projection substrate;
sorted-key compact JSON payload bytes remain the domain truth. This byte basis
is deterministic but is not RFC 8785/JCS.
"""

from __future__ import annotations

from copy import deepcopy
from hashlib import sha256
import json
from pathlib import Path
import sqlite3
from typing import Any, Mapping, Sequence


SCHEMA_VERSION = "0.1.0"
DOCUMENT_KIND = "openpipestress.project_persistence"
# Model document 0.8.0 (DEL-02-01 Design model-document-0.8.0.md §3.1, §3.3): the model payload's version is the
# desktop's. Every non-current model document is refused by name on validate, create, save and open, with the
# desktop's state, code and text (the applier's `model_document_version`, which src-tauri delegates to; the shared
# table is fixtures/model_document/version_states.json): an older one as MODEL_SCHEMA_VERSION_RETIRED (then
# RETIRED_PROJECT_RUNS_NOTE, as `refuse_model_document` appends it), a newer one as
# MODEL_SCHEMA_VERSION_NEWER_THAN_SUPPORTED, and a missing, non-string or non-semver one as
# MODEL_SCHEMA_VERSION_UNSUPPORTED. Nothing is migrated. The project's recorded runs are not gated here (owner ruling
# "0.8.0 a2").
MODEL_VERSION = "0.8.0"
RETIRED_MODEL_VERSIONS = ("0.1.0", "0.2.0", "0.3.0", "0.4.0", "0.5.0", "0.6.0", "0.7.0")
MODEL_SCHEMA_VERSION_RETIRED = "MODEL_SCHEMA_VERSION_RETIRED"
MODEL_SCHEMA_VERSION_NEWER_THAN_SUPPORTED = "MODEL_SCHEMA_VERSION_NEWER_THAN_SUPPORTED"
MODEL_SCHEMA_VERSION_UNSUPPORTED = "MODEL_SCHEMA_VERSION_UNSUPPORTED"
RETIRED_PROJECT_RUNS_NOTE = (
    "Recorded runs keep their own format and existing read and compare behaviour; this change adds no "
    "model-document-version gate for recorded runs. Existing integrity checks and retained-precision/source-reader "
    "restrictions still apply. Run history is listed for the active project's id, so this project's runs are reached "
    "through a current project with the same project id, and run comparison keeps its eligibility rules.")
PROJECT_SCHEMA_REF = "schemas/project_persistence.schema.yaml"
MODEL_SCHEMA_REF = "schemas/model.schema.yaml"
MODEL_STATE_SCHEMA_REF = "schemas/model_state.schema.json"
ANALYSIS_RUN_SCHEMA_REF = "schemas/analysis_run.schema.json"
SQLITE_STORE_PROFILE = "sqlite_local_project_store"
SQLITE_STORE_SCHEMA_VERSION = "1"
CANONICALIZATION_LABEL = "SORTED_COMPACT_JSON"
CANONICAL_TRUTH_LABEL = "sorted_compact_json_payload"

PROFESSIONAL_BOUNDARY = {
    "human_review_required": True,
    "software_makes_compliance_claim": False,
    "software_makes_certification_claim": False,
    "software_makes_sealing_claim": False,
    "software_makes_approval_claim": False,
    "software_makes_authentication_claim": False,
}

PRIVATE_DATA_MARKER = {
    "classification": "public_permissive",
    "redistribution_status": "public_permissive",
    "default_transmission_allowed": False,
    "quarantine_required": False,
    "review_status": "accepted",
}

ENGINE_PROVENANCE = {
    "source_name": "OpenPipeStress TP-PER-01 project persistence service",
    "source_location": "core/project_persistence/service.py",
    "source_license": "project-governed",
    "contributor": "OpenPipeStress Type 2 worker",
    "contributor_certification": "implementation-only-no-professional-claim",
    "redistribution_status": "public_permissive",
    "review_status": "accepted",
}


def physical_container_profile() -> dict[str, Any]:
    """Return the accepted SCA-003 physical container profile."""

    return {
        "status": "accepted",
        "profile": SQLITE_STORE_PROFILE,
        "decision_ref": "SCA-003",
        "storage_role": "local_store_index_projection",
        "canonical_truth": CANONICAL_TRUTH_LABEL,
        "sql_public_contract": False,
        "direct_sql_access_allowed": False,
        "hosted_db_allowed": False,
        "network_required": False,
        "sidecars_rebuildable": True,
        "notes": "SQLite stores canonical JSON bytes and indexed projections; SQL is not a public contract.",
    }


def retrieval_sidecar_manifest() -> list[dict[str, Any]]:
    """Return the default rebuildable, hash-neutral local retrieval sidecars."""

    return [
        {
            "sidecar_kind": "sqlite_fts5_bm25",
            "authoritative": False,
            "rebuildable": True,
            "affects_project_hash": False,
            "local_only": True,
            "artifact_ref": _artifact_ref("retrieval_sidecar", "sidecar:sqlite-fts5-bm25"),
            "notes": "Local FTS5/BM25 index; may be deleted and rebuilt from canonical payloads.",
        }
    ]


def build_project_persistence_envelope(
    *,
    project_id: str,
    project_name: str,
    model_payload: Mapping[str, Any],
    model_state_refs: Sequence[Mapping[str, Any]] | None = None,
    analysis_run_records: Sequence[Mapping[str, Any]] | None = None,
    analysis_run_refs: Sequence[Mapping[str, Any]] | None = None,
    result_envelope_refs: Sequence[Mapping[str, Any]] | None = None,
    result_refs: Sequence[Mapping[str, Any]] | None = None,
    model_state_records: Sequence[Mapping[str, Any]] | None = None,
    provenance_manifest: Sequence[Mapping[str, Any]] | None = None,
) -> dict[str, Any]:
    """Build a deterministic persistence envelope around a canonical model."""

    model_payload_copy = deepcopy(dict(model_payload))
    run_history = _run_history(
        model_state_refs=model_state_refs,
        model_state_records=model_state_records,
        analysis_run_refs=analysis_run_refs,
        analysis_run_records=analysis_run_records,
        result_envelope_refs=result_envelope_refs,
        result_refs=result_refs,
    )
    project = {
        "project_id": project_id,
        "project_name": project_name,
        "unit_system_ref": _artifact_ref("schema", "unit-system:SI-preview"),
        "model_payload": model_payload_copy,
        "rule_pack_refs": [],
        "provenance_manifest": [
            deepcopy(item) for item in (provenance_manifest or [ENGINE_PROVENANCE])
        ],
        "private_data": deepcopy(PRIVATE_DATA_MARKER),
        "human_acceptance_refs": [],
        "run_history": run_history,
    }
    base_envelope = {
        "schema_version": SCHEMA_VERSION,
        "document_kind": DOCUMENT_KIND,
        "physical_container": physical_container_profile(),
        "project": project,
        "migration": {
            "status": "current",
            "source_schema_version": SCHEMA_VERSION,
            "target_schema_version": SCHEMA_VERSION,
            "db_migration_status": "current",
            "product_schema_migration_status": "current",
            "migration_framework": "application_service_separate_db_and_product_schema",
            "diagnostics": [],
        },
        "external_artifacts": [],
        "retrieval_sidecars": retrieval_sidecar_manifest(),
        "round_trip_manifest": {
            "serialization": "sorted_compact_json",
            "semantic_equality": [
                "model_content",
                "unit_metadata",
                "loads",
                "rule_pack_refs",
                "provenance_metadata",
                "reproducibility_metadata",
            ],
            "volatile_fields": [],
            "normalization_rules": [
                "schema_approved_only",
                "no_silent_engineering_defaults",
                "documented_volatile_field_exclusion",
            ],
        },
        "validation_profile": {
            "schema_validation": True,
            "model_schema_delegation": MODEL_SCHEMA_REF,
            "unit_metadata_check": True,
            "provenance_check": True,
            "rule_pack_reference_check": True,
            "protected_content_check": True,
            "private_data_check": True,
            "professional_boundary_check": True,
            "telemetry_default": "off",
        },
        "service_operations": _service_operations(),
        "professional_boundary": deepcopy(PROFESSIONAL_BOUNDARY),
        "diagnostics": [],
    }
    base_envelope["hash"] = _hash_metadata(_envelope_without_hash(base_envelope))
    return base_envelope


def validate_project_persistence_envelope(envelope: Mapping[str, Any]) -> list[dict[str, Any]]:
    """Return structured diagnostics for TP-PER-01 persistence invariants."""

    diagnostics: list[dict[str, Any]] = []
    if not isinstance(envelope, Mapping):
        return [_diagnostic("PERSISTENCE_ENVELOPE_INVALID", "SCHEMA_VALIDATION", "project_envelope")]

    for field in ("schema_version", "document_kind", "project", "hash", "migration"):
        if not envelope.get(field):
            diagnostics.append(_diagnostic("PERSISTENCE_FIELD_MISSING", "SCHEMA_VALIDATION", field))

    if envelope.get("document_kind") != DOCUMENT_KIND:
        diagnostics.append(_diagnostic("PERSISTENCE_DOCUMENT_KIND_INVALID", "SCHEMA_VALIDATION", "document_kind"))

    physical_container = envelope.get("physical_container", {})
    if not _valid_physical_container(physical_container):
        diagnostics.append(_diagnostic("PERSISTENCE_PHYSICAL_CONTAINER_INVALID", "SCHEMA_VALIDATION", "physical_container"))

    project = envelope.get("project", {})
    if not isinstance(project, Mapping):
        diagnostics.append(_diagnostic("PERSISTENCE_PROJECT_INVALID", "SCHEMA_VALIDATION", "project"))
        project = {}

    if not project.get("model_payload"):
        diagnostics.append(_diagnostic("PERSISTENCE_MODEL_PAYLOAD_MISSING", "SCHEMA_VALIDATION", "project.model_payload"))
    else:
        refusal = model_payload_version_refusal(project.get("model_payload"))
        if refusal is not None:
            diagnostics.append(refusal)

    if not _valid_private_data(project.get("private_data")):
        diagnostics.append(_diagnostic("PERSISTENCE_PRIVATE_DATA_BOUNDARY_INVALID", "PRIVATE_DATA", "project.private_data"))

    provenance_manifest = project.get("provenance_manifest", [])
    if not provenance_manifest or any(not _valid_provenance(item) for item in provenance_manifest):
        diagnostics.append(_diagnostic("PERSISTENCE_PROVENANCE_MISSING", "PROVENANCE_WARNING", "project.provenance_manifest"))

    if envelope.get("professional_boundary") != PROFESSIONAL_BOUNDARY:
        diagnostics.append(_diagnostic("PERSISTENCE_PROFESSIONAL_BOUNDARY_VIOLATION", "PROFESSIONAL_BOUNDARY", "professional_boundary"))

    validation_profile = envelope.get("validation_profile", {})
    if isinstance(validation_profile, Mapping) and validation_profile.get("telemetry_default") != "off":
        diagnostics.append(_diagnostic("PERSISTENCE_TELEMETRY_DEFAULT_INVALID", "PRIVATE_DATA", "validation_profile.telemetry_default"))

    run_history = project.get("run_history", {})
    if not _valid_run_history(run_history):
        diagnostics.append(_diagnostic("PERSISTENCE_RUN_HISTORY_REFS_MISSING", "SCHEMA_VALIDATION", "project.run_history"))

    for diagnostic in validate_external_artifact_references(envelope):
        diagnostics.append(diagnostic)

    expected_hash = _hash_metadata(_envelope_without_hash(envelope))
    actual_hash = envelope.get("hash", {})
    if isinstance(actual_hash, Mapping):
        if actual_hash.get("project_payload_hash", {}).get("value") != expected_hash["project_payload_hash"]["value"]:
            diagnostics.append(_diagnostic("PERSISTENCE_PROJECT_HASH_MISMATCH", "SCHEMA_VALIDATION", "hash.project_payload_hash"))
        if _manifest_values(actual_hash.get("hash_manifest", [])) != _manifest_values(expected_hash["hash_manifest"]):
            diagnostics.append(_diagnostic("PERSISTENCE_HASH_MANIFEST_MISMATCH", "SCHEMA_VALIDATION", "hash.hash_manifest"))

    return sorted(diagnostics, key=canonical_json)


def _semver(raw: Any) -> tuple[str, str, str] | None:
    """`major.minor.patch`, each part one or more ASCII digits, with its leading zeros dropped.

    The parts stay digit strings: nothing converts them to int (whose string conversion is bounded), so a part of
    any length has a defined state, as in the applier's and the TS checks.
    """

    parts = raw.split(".") if isinstance(raw, str) else []
    if len(parts) != 3 or not all(part and all("0" <= ch <= "9" for ch in part) for part in parts):
        return None
    major, minor, patch = (part.lstrip("0") or "0" for part in parts)
    return major, minor, patch


def _semver_compare(left: tuple[str, str, str], right: tuple[str, str, str]) -> int:
    """-1, 0 or 1: parts compare as numbers, a longer part (no leading zeros) larger, then digit by digit."""

    for a, b in zip(left, right):
        key_a, key_b = (len(a), a), (len(b), b)
        if key_a != key_b:
            return 1 if key_a > key_b else -1
    return 0


def model_document_version_state(model_payload: Any) -> dict[str, Any]:
    """The model document's version state, as the desktop gives it: status, versions, detail and refusal code."""

    version = model_payload.get("schema_version") if isinstance(model_payload, Mapping) else None
    raw = version if isinstance(version, str) else ""
    if raw == MODEL_VERSION:
        return {"status": "current", "source_schema_version": raw, "target_schema_version": raw, "code": None,
                "detail": f"Model document {MODEL_VERSION} is the current version; a pressure contract, when present, "
                          "and solve eligibility are validated separately."}
    parsed = _semver(raw)
    if parsed is None:
        return {"status": "unsupported_schema", "source_schema_version": raw or "missing_schema_version",
                "target_schema_version": MODEL_VERSION, "code": MODEL_SCHEMA_VERSION_UNSUPPORTED,
                "detail": "Model document has no valid semver schema_version; refusing to open it for editing without "
                          "coercion."}
    if _semver_compare(parsed, _semver(MODEL_VERSION)) > 0:
        return {"status": "newer_than_supported", "source_schema_version": raw, "target_schema_version": MODEL_VERSION,
                "code": MODEL_SCHEMA_VERSION_NEWER_THAN_SUPPORTED,
                "detail": f"Model document schema_version {raw} is newer than the current {MODEL_VERSION}; refusing to "
                          "open it for editing (no down-migration)."}
    detail = (f"model document {raw} is retired; re-author it as model document {MODEL_VERSION} (a pressure-free model "
              "omits pressure_contract). Nothing is migrated." if raw in RETIRED_MODEL_VERSIONS else
              f"model document {raw} is not a supported version; re-author it as model document {MODEL_VERSION}. "
              "Nothing is migrated.")
    return {"status": "retired_schema", "source_schema_version": raw, "target_schema_version": MODEL_VERSION,
            "code": MODEL_SCHEMA_VERSION_RETIRED, "detail": f"{MODEL_SCHEMA_VERSION_RETIRED}: {detail}"}


def model_payload_version_refusal(model_payload: Any) -> dict[str, Any] | None:
    """The named refusal of a non-current model document, as the desktop states it; None for the current one."""

    state = model_document_version_state(model_payload)
    if state["code"] is None:
        return None
    diagnostic = _diagnostic(state["code"], "SCHEMA_VALIDATION", "project.model_payload.schema_version")
    note = f" {RETIRED_PROJECT_RUNS_NOTE}" if state["status"] == "retired_schema" else ""
    diagnostic["message"] = f"{state['detail']}{note}"
    diagnostic["remediation"] = f"Re-author the model as model document {MODEL_VERSION}; nothing is migrated."
    return diagnostic


def canonical_json(value: Any) -> str:
    """Return sorted compact Python JSON; this is not RFC 8785/JCS."""

    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=True)


def project_hash_manifest(envelope: Mapping[str, Any]) -> list[dict[str, Any]]:
    """Return deterministic hash records for a persistence envelope."""

    return _hash_metadata(_envelope_without_hash(envelope))["hash_manifest"]


def create_project_store(store_path: str | Path, envelope: Mapping[str, Any]) -> dict[str, Any]:
    """Create a local SQLite project store and save the canonical envelope."""

    return save_project_store(store_path, envelope)


def open_project_store(store_path: str | Path) -> dict[str, Any]:
    """Open a local SQLite project store and return its canonical envelope."""

    path = Path(store_path)
    if not path.exists():
        return {
            "store_path": str(path),
            "envelope": None,
            "diagnostics": [_diagnostic("PERSISTENCE_STORE_MISSING", "MIGRATION", "sqlite_project_store")],
        }
    with sqlite3.connect(path) as connection:
        _initialize_store(connection)
        row = connection.execute(
            "SELECT canonical_json, project_hash FROM canonical_payloads WHERE payload_scope = ?",
            ("project_envelope",),
        ).fetchone()
        if row is None:
            return {
                "store_path": str(path),
                "envelope": None,
                "diagnostics": [_diagnostic("PERSISTENCE_STORE_PAYLOAD_MISSING", "SCHEMA_VALIDATION", "project_envelope")],
            }
        envelope = json.loads(row[0])
        diagnostics = validate_project_persistence_envelope(envelope)
        expected_hash = envelope.get("hash", {}).get("project_payload_hash", {}).get("value")
        if expected_hash and row[1] != expected_hash:
            diagnostics.append(_diagnostic("PERSISTENCE_STORE_HASH_MISMATCH", "SCHEMA_VALIDATION", "project_envelope"))
        return {
            "store_path": str(path),
            "envelope": envelope,
            "diagnostics": sorted(diagnostics, key=canonical_json),
            "hash_manifest": project_hash_manifest(envelope),
        }


def save_project_store(store_path: str | Path, envelope: Mapping[str, Any]) -> dict[str, Any]:
    """Save canonical envelope bytes into a local SQLite project store."""

    path = Path(store_path)
    path.parent.mkdir(parents=True, exist_ok=True)
    source = deepcopy(dict(envelope))
    source["physical_container"] = physical_container_profile()
    source.setdefault("retrieval_sidecars", retrieval_sidecar_manifest())
    source.setdefault("external_artifacts", [])
    source["hash"] = _hash_metadata(_envelope_without_hash(source))
    diagnostics = validate_project_persistence_envelope(source)
    if any(item.get("severity") == "blocking" for item in diagnostics):
        return {"store_path": str(path), "envelope": source, "diagnostics": diagnostics}

    canonical_payload = canonical_json(source)
    project_hash = source["hash"]["project_payload_hash"]["value"]
    with sqlite3.connect(path) as connection:
        _initialize_store(connection)
        connection.execute(
            """
            INSERT OR REPLACE INTO metadata(key, value)
            VALUES (?, ?), (?, ?), (?, ?), (?, ?)
            """,
            (
                "store_schema_version",
                SQLITE_STORE_SCHEMA_VERSION,
                "profile",
                SQLITE_STORE_PROFILE,
                "canonical_truth",
                CANONICAL_TRUTH_LABEL,
                "direct_sql_public_contract",
                "false",
            ),
        )
        connection.execute(
            """
            INSERT OR REPLACE INTO canonical_payloads(payload_scope, payload_ref, canonical_json, project_hash)
            VALUES (?, ?, ?, ?)
            """,
            ("project_envelope", "project-envelope", canonical_payload, project_hash),
        )
        _store_project_projection(connection, source)
        connection.commit()
    return {
        "store_path": str(path),
        "envelope": source,
        "diagnostics": diagnostics,
        "hash_manifest": project_hash_manifest(source),
    }


def _version_status(raw: Any) -> str:
    """One current version (DEL-02-05 CLM-003): every other version is refused by name, never migrated."""

    version = _semver(raw)
    if version is None:
        return "unsupported_schema"
    order = _semver_compare(version, _semver(SCHEMA_VERSION))
    if order == 0:
        return "current"
    return "newer_than_supported" if order > 0 else "retired_schema"


def version_check_project_store(store_path: str | Path) -> dict[str, Any]:
    """Return the store's DB migration status and the product schema's version status (no product migration)."""

    opened = open_project_store(store_path)
    envelope = opened.get("envelope")
    if not isinstance(envelope, Mapping):
        return {
            "db_migration_status": "failed",
            "product_schema_migration_status": "unsupported_schema",
            "diagnostics": opened.get("diagnostics", []),
        }
    return {
        "db_migration_status": "current",
        "product_schema_migration_status": _version_status(envelope.get("schema_version")),
        "diagnostics": opened.get("diagnostics", []),
    }


def rebuild_retrieval_sidecars(store_path: str | Path) -> dict[str, Any]:
    """Rebuild local retrieval indexes without changing canonical project hashes."""

    opened = open_project_store(store_path)
    envelope = opened.get("envelope")
    if not isinstance(envelope, Mapping):
        return opened
    before_hash = envelope["hash"]["project_payload_hash"]["value"]
    with sqlite3.connect(Path(store_path)) as connection:
        _initialize_store(connection)
        connection.execute("DELETE FROM retrieval_documents")
        project = envelope.get("project", {})
        text = canonical_json(project)
        connection.execute(
            "INSERT INTO retrieval_documents(ref, content) VALUES (?, ?)",
            ("project_payload", text),
        )
        connection.commit()
    after = open_project_store(store_path)
    after_envelope = after.get("envelope", {})
    after_hash = after_envelope.get("hash", {}).get("project_payload_hash", {}).get("value")
    return {
        "store_path": str(Path(store_path)),
        "sidecar_kind": "sqlite_fts5_bm25",
        "before_hash": before_hash,
        "after_hash": after_hash,
        "hash_neutral": before_hash == after_hash,
        "diagnostics": after.get("diagnostics", []),
    }


def external_artifact_reference(
    *,
    artifact_id: str,
    uri_or_path: str,
    size_bytes: int,
    checksum_value: str,
    classification: str = "private_project",
    verification_status: str = "TBD",
) -> dict[str, Any]:
    """Build a schema-shaped external artifact reference."""

    return {
        "artifact_ref": _artifact_ref("external_file", artifact_id),
        "uri_or_path": uri_or_path,
        "hash": {
            "algorithm": "sha256",
            "canonicalization": "NONE",
            "payload_ref": _artifact_ref("external_file", artifact_id),
            "payload_scope": "external_artifact",
            "value": checksum_value,
        },
        "size_bytes": size_bytes,
        "classification": classification,
        "verification_status": verification_status,
        "copy_policy": "reference_in_place_by_default",
    }


def validate_external_artifact_references(envelope: Mapping[str, Any]) -> list[dict[str, Any]]:
    """Return diagnostics for external artifact references without leaking paths."""

    diagnostics: list[dict[str, Any]] = []
    artifacts = envelope.get("external_artifacts", [])
    if not isinstance(artifacts, Sequence) or isinstance(artifacts, (str, bytes)):
        return diagnostics
    for artifact in artifacts:
        if not isinstance(artifact, Mapping):
            diagnostics.append(_diagnostic("PERSISTENCE_EXTERNAL_ARTIFACT_INVALID", "EXTERNAL_ARTIFACT", "external_artifact"))
            continue
        uri_or_path = str(artifact.get("uri_or_path", ""))
        artifact_ref = artifact.get("artifact_ref", {})
        ref = artifact_ref.get("ref", "external_artifact") if isinstance(artifact_ref, Mapping) else "external_artifact"
        if "://" in uri_or_path:
            continue
        path = Path(uri_or_path)
        if not path.exists():
            diagnostics.append(_diagnostic("PERSISTENCE_EXTERNAL_ARTIFACT_MISSING", "EXTERNAL_ARTIFACT", str(ref)))
            continue
        expected_size = artifact.get("size_bytes")
        if isinstance(expected_size, int) and path.stat().st_size != expected_size:
            diagnostics.append(_diagnostic("PERSISTENCE_EXTERNAL_ARTIFACT_STALE", "EXTERNAL_ARTIFACT", str(ref)))
        expected_hash = artifact.get("hash", {}).get("value") if isinstance(artifact.get("hash"), Mapping) else None
        if expected_hash and _file_sha256(path) != expected_hash:
            diagnostics.append(_diagnostic("PERSISTENCE_EXTERNAL_ARTIFACT_HASH_MISMATCH", "EXTERNAL_ARTIFACT", str(ref)))
        if artifact.get("classification") in {"private_project", "private_library", "protected_suspected"}:
            diagnostics.append(_diagnostic("PERSISTENCE_EXTERNAL_ARTIFACT_PRIVATE", "PRIVATE_DATA", str(ref)))
    return diagnostics


def round_trip_project_envelope(envelope: Mapping[str, Any]) -> dict[str, Any]:
    """Serialize and parse an envelope through canonical JSON."""

    source = deepcopy(dict(envelope))
    restored = json.loads(canonical_json(source))
    source_hash = _checksum(_artifact_ref("project_persistence", "round-trip:source"), "project_envelope", _envelope_without_hash(source))
    restored_hash = _checksum(_artifact_ref("project_persistence", "round-trip:restored"), "project_envelope", _envelope_without_hash(restored))
    return {
        "serialization": "sorted_compact_json",
        "semantic_equal": canonical_json(source) == canonical_json(restored),
        "source_hash": source_hash,
        "round_trip_hash": restored_hash,
        "diagnostics": validate_project_persistence_envelope(restored),
        "envelope": restored,
    }


def _run_history(
    *,
    model_state_refs: Sequence[Mapping[str, Any]] | None,
    model_state_records: Sequence[Mapping[str, Any]] | None,
    analysis_run_refs: Sequence[Mapping[str, Any]] | None,
    analysis_run_records: Sequence[Mapping[str, Any]] | None,
    result_envelope_refs: Sequence[Mapping[str, Any]] | None,
    result_refs: Sequence[Mapping[str, Any]] | None,
) -> dict[str, Any]:
    analysis_records = [deepcopy(dict(item)) for item in (analysis_run_records or [])]
    state_records = [deepcopy(dict(item)) for item in (model_state_records or [])]
    inferred_analysis_refs = []
    for item in analysis_records:
        run_id = item.get("analysis_run", {}).get("run_id", "analysis-run:unknown")
        identity_ref = _artifact_ref("analysis_run", run_id)
        inferred_analysis_refs.append(
            _artifact_ref(
                "analysis_run",
                run_id,
                _checksum(identity_ref, "analysis_run_record", item),
            )
        )
    inferred_result_refs = _result_refs_from_analysis_records(analysis_records)
    history = {
        "model_state_refs": [deepcopy(dict(item)) for item in (model_state_refs or [])],
        "model_state_records": state_records,
        "analysis_run_refs": [deepcopy(dict(item)) for item in (analysis_run_refs or inferred_analysis_refs)],
        "analysis_run_records": analysis_records,
        "result_envelope_refs": [deepcopy(dict(item)) for item in (result_envelope_refs or [])],
        "result_refs": [deepcopy(dict(item)) for item in (result_refs or inferred_result_refs)],
        "hash_manifest": [],
    }
    history["hash_manifest"] = _history_record_hashes(history)
    return history


def _hash_metadata(envelope_without_hash: Mapping[str, Any]) -> dict[str, Any]:
    project = deepcopy(dict(envelope_without_hash.get("project", {})))
    run_history = project.get("run_history", {}) if isinstance(project, Mapping) else {}
    manifest = [
        _checksum(_artifact_ref("project_persistence", "project-payload"), "project_payload", project),
        _checksum(_artifact_ref("model_payload", "model-payload"), "model_payload", project.get("model_payload", {})),
        _checksum(_artifact_ref("project_persistence", "project-envelope-without-hash"), "project_envelope", envelope_without_hash),
    ]
    project_payload_hash = manifest[0]
    if isinstance(run_history, Mapping):
        manifest.extend(deepcopy(dict(item)) for item in run_history.get("hash_manifest", []))
        for ref in run_history.get("result_envelope_refs", []):
            if isinstance(ref, Mapping) and ref.get("hash"):
                manifest.append(deepcopy(dict(ref["hash"])))
        for ref in run_history.get("result_refs", []):
            if isinstance(ref, Mapping) and ref.get("hash"):
                manifest.append(deepcopy(dict(ref["hash"])))
    return {
        "canonicalization": CANONICALIZATION_LABEL,
        "project_payload_hash": project_payload_hash,
        "hash_manifest": manifest,
        "payload_partition_status": "external_artifacts_by_reference"
        if envelope_without_hash.get("external_artifacts")
        else "json_payloads_defined",
        "excluded_fields": ["retrieval_sidecars"],
    }


def _history_record_hashes(run_history: Mapping[str, Any]) -> list[dict[str, Any]]:
    hashes: list[dict[str, Any]] = []
    for record in run_history.get("model_state_records", []):
        state_id = record.get("model_state", {}).get("state_id", "model-state:unknown")
        hashes.append(_checksum(_artifact_ref("model_state", state_id), "model_state_record", record))
    for record in run_history.get("analysis_run_records", []):
        run_id = record.get("analysis_run", {}).get("run_id", "analysis-run:unknown")
        hashes.append(_checksum(_artifact_ref("analysis_run", run_id), "analysis_run_record", record))
    return hashes


def _envelope_without_hash(envelope: Mapping[str, Any]) -> dict[str, Any]:
    value = deepcopy(dict(envelope))
    value.pop("hash", None)
    value.pop("retrieval_sidecars", None)
    return value


def _initialize_store(connection: sqlite3.Connection) -> None:
    connection.execute(
        """
        CREATE TABLE IF NOT EXISTS metadata (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        )
        """
    )
    connection.execute(
        """
        CREATE TABLE IF NOT EXISTS canonical_payloads (
            payload_scope TEXT PRIMARY KEY,
            payload_ref TEXT NOT NULL,
            canonical_json TEXT NOT NULL,
            project_hash TEXT NOT NULL
        )
        """
    )
    connection.execute(
        """
        CREATE TABLE IF NOT EXISTS project_projection (
            project_id TEXT PRIMARY KEY,
            project_name TEXT,
            schema_version TEXT NOT NULL,
            document_kind TEXT NOT NULL
        )
        """
    )
    try:
        connection.execute(
            """
            CREATE VIRTUAL TABLE IF NOT EXISTS retrieval_documents
            USING fts5(ref UNINDEXED, content)
            """
        )
    except sqlite3.OperationalError:
        connection.execute(
            """
            CREATE TABLE IF NOT EXISTS retrieval_documents (
                ref TEXT PRIMARY KEY,
                content TEXT NOT NULL
            )
            """
        )


def _store_project_projection(connection: sqlite3.Connection, envelope: Mapping[str, Any]) -> None:
    project = envelope.get("project", {}) if isinstance(envelope.get("project"), Mapping) else {}
    connection.execute(
        """
        INSERT OR REPLACE INTO project_projection(project_id, project_name, schema_version, document_kind)
        VALUES (?, ?, ?, ?)
        """,
        (
            str(project.get("project_id", "project:unknown")),
            str(project.get("project_name", "")),
            str(envelope.get("schema_version", "")),
            str(envelope.get("document_kind", "")),
        ),
    )


def _checksum(payload_ref: Mapping[str, Any], payload_scope: str, payload: Any) -> dict[str, Any]:
    return {
        "algorithm": "sha256",
        "canonicalization": CANONICALIZATION_LABEL,
        "payload_ref": deepcopy(dict(payload_ref)),
        "payload_scope": payload_scope,
        "value": sha256(canonical_json(payload).encode("utf-8")).hexdigest(),
    }


def _artifact_ref(ref_kind: str, ref: str, hash_value: Mapping[str, Any] | None = None) -> dict[str, Any]:
    value = {
        "ref_kind": ref_kind,
        "ref": ref,
    }
    if hash_value is not None:
        value["hash"] = deepcopy(dict(hash_value))
    return value


def _result_refs_from_analysis_records(records: Sequence[Mapping[str, Any]]) -> list[dict[str, Any]]:
    refs: list[dict[str, Any]] = []
    seen: set[str] = set()
    for record in records:
        for item in record.get("analysis_run", {}).get("result_refs", []):
            result_ref = item.get("result_ref", {})
            result_id = result_ref.get("ref") or result_ref.get("id")
            if not result_id or result_id in seen:
                continue
            seen.add(str(result_id))
            refs.append(_artifact_ref("result", str(result_id)))
    return refs


def _valid_private_data(value: Any) -> bool:
    if not isinstance(value, Mapping):
        return False
    return (
        value.get("default_transmission_allowed") is False
        and value.get("classification") != "protected_suspected"
        and value.get("redistribution_status") != "protected_suspected"
    )


def _valid_physical_container(value: Any) -> bool:
    if not isinstance(value, Mapping):
        return False
    return (
        value.get("status") == "accepted"
        and value.get("profile") == SQLITE_STORE_PROFILE
        and value.get("decision_ref") == "SCA-003"
        and value.get("sql_public_contract") is False
        and value.get("direct_sql_access_allowed") is False
        and value.get("hosted_db_allowed") is False
        and value.get("network_required") is False
        and value.get("sidecars_rebuildable") is True
    )


def _valid_provenance(value: Any) -> bool:
    if not isinstance(value, Mapping):
        return False
    required = (
        "source_name",
        "source_location",
        "source_license",
        "contributor",
        "contributor_certification",
        "redistribution_status",
        "review_status",
    )
    return all(str(value.get(field, "")).strip() for field in required) and value.get("redistribution_status") != "protected_suspected"


def _valid_run_history(value: Any) -> bool:
    if not isinstance(value, Mapping):
        return False
    return all(value.get(field) for field in ("model_state_refs", "analysis_run_refs", "result_envelope_refs", "result_refs"))


def _manifest_values(manifest: Any) -> list[tuple[str, str, str]]:
    if not isinstance(manifest, Sequence) or isinstance(manifest, (str, bytes)):
        return []
    values = []
    for item in manifest:
        if isinstance(item, Mapping):
            payload_ref = item.get("payload_ref", {})
            ref = payload_ref.get("ref") if isinstance(payload_ref, Mapping) else ""
            values.append((str(item.get("payload_scope", "")), str(ref), str(item.get("value", ""))))
    return sorted(values)


def _diagnostic(code: str, diagnostic_class: str, affected_ref: str) -> dict[str, Any]:
    return {
        "code": code,
        "class": diagnostic_class,
        "severity": "blocking",
        "source": "core/project_persistence/service.py",
        "affected_ref": _artifact_ref("project_persistence", affected_ref),
        "message": f"{code} at {affected_ref}",
        "remediation": "Provide a schema-shaped persistence envelope with explicit provenance, privacy, hashes, and professional-boundary fields.",
    }


def _file_sha256(path: Path) -> str:
    digest = sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def _service_operations() -> list[dict[str, Any]]:
    diagnostic_classes = [
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
    ]
    return [
        {
            "operation": operation,
            "boundary": "application_service",
            "minimum_inputs": ["project_envelope"],
            "minimum_outputs": ["project_envelope", "diagnostics", "hash_manifest"],
            "diagnostic_classes": diagnostic_classes,
            "bypass_prohibited": True,
        }
        for operation in ("create_project", "open_project", "save_project", "validate_project", "version_check")
    ]
