"""Deterministic metadata-only controls for local-first storage records.

The functions in this module classify explicit storage metadata only. They do
not read or write files, choose operating-system paths, store payloads, open
SQLite handles, transmit data, manage secrets, or make professional/security
certification claims.
"""

from __future__ import annotations

from dataclasses import asdict, dataclass, field
import hashlib
import json
from typing import Any, Iterable, Mapping


PUBLIC_SHARED_CONTEXTS = {
    "public_repository",
    "public_example",
    "public_fixture",
    "public_report",
    "shared_model",
    "downstream_tool",
}
LOCAL_PRIVATE_CONTEXTS = {
    "local_private",
    "private_project_package",
    "private_library",
    "private_rule_pack",
    "private_report",
    "private_diagnostic",
    "local_cache",
}
BLOCKED_REMOTE_CONTEXTS = {
    "cloud_storage",
    "cloud_sync",
    "hosted_database",
    "network_share",
    "network_sync",
    "remote_service",
}
DIRECT_SQL_CONTEXTS = {"direct_sql", "raw_sqlite", "sqlite_table_access"}
STORAGE_CONTEXTS = (
    PUBLIC_SHARED_CONTEXTS
    | LOCAL_PRIVATE_CONTEXTS
    | BLOCKED_REMOTE_CONTEXTS
    | DIRECT_SQL_CONTEXTS
)

SYMBOLIC_STORAGE_CLASSES = {
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
    "local_private",
}

PRIVATE_RECORD_KINDS = {
    "private_project",
    "project_model",
    "project_package",
    "project_store",
    "private_rule_pack",
    "private_material_library",
    "private_component_library",
    "private_report",
    "private_diagnostic",
    "private_cache",
    "secret_reference",
}
PUBLIC_RECORD_KINDS = {"public_metadata", "public_example_metadata"}
STORAGE_RECORD_KINDS = PRIVATE_RECORD_KINDS | PUBLIC_RECORD_KINDS | {"unknown"}

PRIVATE_PRIVACY_CLASSES = {
    "private_project_data",
    "private_rule_pack_data",
    "private_material_data",
    "private_component_data",
    "private_report_data",
    "private_diagnostic_data",
    "private_cache_data",
    "owner_standard_data",
    "company_design_basis_data",
    "path_data",
    "secret_like_data",
}
PUBLIC_PRIVACY_CLASSES = {"public_metadata", "invented_public_example"}
UNKNOWN_STATUSES = {"unknown", "TBD", "", None}
PUBLIC_REDIS_STATUSES = {"public_permissive", "invented_non_engineering_example"}
PRIVATE_REDIS_STATUSES = {"private_only"}
BLOCKING_REVIEW_STATUSES = {"quarantined", "rejected"}
PROTECTED_STATUSES = {"protected_suspected"}

PAYLOAD_KEYS = {
    "value",
    "text",
    "payload",
    "content",
    "contents",
    "raw_value",
    "project_payload",
    "rule_payload",
    "material_values",
    "component_values",
    "report_payload",
    "diagnostic_payload",
    "cache_payload",
    "sqlite_payload",
}
SECRET_MATERIAL_KEYS = {
    "secret",
    "secret_value",
    "credential",
    "credential_value",
    "token",
    "password",
    "api_key",
    "access_key",
    "authorization_header",
    "private_key",
}
CONCRETE_PATH_KEYS = {
    "path",
    "file_path",
    "absolute_path",
    "local_path",
    "repository_path",
    "db_path",
    "sqlite_path",
    "uri",
}
CLOUD_NETWORK_KEYS = {
    "cloud_url",
    "remote_url",
    "network_location",
    "cloud_bucket",
    "cloud_sync_target",
    "hosted_database",
}
DIRECT_SQL_KEYS = {
    "sql",
    "query",
    "table",
    "table_name",
    "sqlite_handle",
    "raw_sqlite_handle",
    "connection_string",
}
SECRET_NAME_MARKERS = (
    "secret",
    "credential",
    "token",
    "password",
    "api_key",
    "access_key",
    "authorization",
    "private_key",
)


@dataclass(frozen=True)
class StorageDiagnostic:
    code: str
    severity: str
    path: str
    message: str
    remediation: str

    def as_schema_dict(self) -> dict[str, str]:
        return asdict(self)


@dataclass(frozen=True)
class StorageRecord:
    record_id: str
    record_kind: str
    label: str
    storage_locality: str
    privacy_classification: str
    redistribution_status: str
    review_status: str
    source_state: str
    source_note: str = "TBD"
    checksum: str | None = None
    checksum_status: str = "TBD"
    value_descriptor: str = "metadata_reference_only"
    contains_payload: bool = False
    secret_material_present: bool = False
    concrete_path_present: bool = False
    cloud_or_network_reference: bool = False
    direct_sql_access: bool = False
    unresolved_tbd: tuple[str, ...] = field(default_factory=tuple)

    def metadata_dict(self) -> dict[str, Any]:
        """Return a metadata-only representation of the record."""

        return {
            "record_id": self.record_id,
            "record_kind": self.record_kind,
            "label": self.label,
            "storage_locality": self.storage_locality,
            "privacy_classification": self.privacy_classification,
            "redistribution_status": self.redistribution_status,
            "review_status": self.review_status,
            "source_state": self.source_state,
            "source_note": self.source_note,
            "checksum": self.checksum,
            "checksum_status": self.checksum_status,
            "value_descriptor": self.value_descriptor,
            "contains_payload": self.contains_payload,
            "secret_material_present": self.secret_material_present,
            "concrete_path_present": self.concrete_path_present,
            "cloud_or_network_reference": self.cloud_or_network_reference,
            "direct_sql_access": self.direct_sql_access,
            "unresolved_tbd": list(self.unresolved_tbd),
        }


@dataclass(frozen=True)
class StorageDecision:
    decision_id: str
    record_id: str
    record_kind: str
    target_context: str
    action: str
    reason_code: str
    metadata_only: bool
    default_posture: str
    diagnostics: tuple[StorageDiagnostic, ...]
    metadata: dict[str, Any]

    @property
    def blocked(self) -> bool:
        return self.action == "block_storage" or any(
            diagnostic.severity == "BLOCKING" for diagnostic in self.diagnostics
        )

    def as_schema_dict(self) -> dict[str, Any]:
        data = asdict(self)
        data["diagnostics"] = [
            diagnostic.as_schema_dict() for diagnostic in self.diagnostics
        ]
        return data


@dataclass(frozen=True)
class StorageGuardResult:
    target_context: str
    decisions: tuple[StorageDecision, ...]
    diagnostics: tuple[StorageDiagnostic, ...]
    safe_manifest: tuple[dict[str, Any], ...]

    @property
    def blocked(self) -> bool:
        return any(decision.blocked for decision in self.decisions)

    def summary(self) -> dict[str, Any]:
        return {
            "target_context": self.target_context,
            "record_count": len(self.decisions),
            "blocked": self.blocked,
            "blocking_count": sum(
                1 for diagnostic in self.diagnostics if diagnostic.severity == "BLOCKING"
            ),
            "warning_count": sum(
                1 for diagnostic in self.diagnostics if diagnostic.severity == "WARNING"
            ),
            "metadata_only": all(decision.metadata_only for decision in self.decisions),
            "cloud_or_network_block_count": sum(
                1
                for decision in self.decisions
                if decision.reason_code
                in {"CLOUD_OR_NETWORK_CONTEXT_BLOCKED", "CLOUD_OR_NETWORK_STORAGE_BLOCKED"}
            ),
            "direct_sql_block_count": sum(
                1
                for decision in self.decisions
                if decision.reason_code
                in {"DIRECT_SQL_CONTEXT_BLOCKED", "DIRECT_SQL_ACCESS_BLOCKED"}
            ),
            "secret_material_stored": False,
            "payloads_stored": False,
            "concrete_paths_emitted": False,
            "professional_claims_made": False,
        }

    def as_schema_dict(self) -> dict[str, Any]:
        return {
            "target_context": self.target_context,
            "decisions": [decision.as_schema_dict() for decision in self.decisions],
            "diagnostics": [
                diagnostic.as_schema_dict() for diagnostic in self.diagnostics
            ],
            "safe_manifest": list(self.safe_manifest),
            "summary": self.summary(),
        }


def storage_record(
    *,
    record_id: str,
    record_kind: str,
    label: str,
    storage_locality: str | None = None,
    privacy_classification: str | None = None,
    redistribution_status: str = "private_only",
    review_status: str = "pending",
    source_state: str = "private_user_supplied",
    source_note: str = "TBD",
    checksum: str | None = None,
    checksum_status: str = "TBD",
    value_descriptor: str | None = None,
    contains_payload: bool = False,
    secret_material_present: bool = False,
    concrete_path_present: bool = False,
    cloud_or_network_reference: bool = False,
    direct_sql_access: bool = False,
) -> StorageRecord:
    """Build a metadata-only storage record for guard evaluation."""

    kind = _storage_kind(record_kind)
    locality = storage_locality or _storage_locality_for_kind(kind)
    privacy = privacy_classification or _privacy_for_kind(kind)
    descriptor = value_descriptor or f"metadata-only {kind} storage reference"
    cloud_or_network_reference = (
        cloud_or_network_reference or locality in BLOCKED_REMOTE_CONTEXTS
    )
    direct_sql_access = direct_sql_access or locality in DIRECT_SQL_CONTEXTS
    concrete_path_present = concrete_path_present or _looks_concrete_path(locality)

    return StorageRecord(
        record_id=record_id,
        record_kind=kind,
        label=label,
        storage_locality=locality,
        privacy_classification=privacy,
        redistribution_status=redistribution_status,
        review_status=review_status,
        source_state=source_state,
        source_note=source_note,
        checksum=checksum,
        checksum_status=checksum_status,
        value_descriptor=descriptor,
        contains_payload=contains_payload,
        secret_material_present=secret_material_present,
        concrete_path_present=concrete_path_present,
        cloud_or_network_reference=cloud_or_network_reference,
        direct_sql_access=direct_sql_access,
        unresolved_tbd=_unresolved_items(
            source_note=source_note,
            checksum_status=checksum_status,
            redistribution_status=redistribution_status,
            review_status=review_status,
            source_state=source_state,
        ),
    )


def classify_storage_record(
    record: StorageRecord | Mapping[str, Any],
) -> StorageDecision:
    """Classify one storage record using only explicit metadata."""

    normalized = _normalize_storage_record(record)
    diagnostics = _classification_diagnostics(normalized)
    reason = _classification_reason(normalized, diagnostics)
    action = "block_storage" if _has_blocking(diagnostics) else "include_metadata_only"
    metadata = normalized.metadata_dict()

    return StorageDecision(
        decision_id=_stable_id(
            "LFS-CLS",
            {
                "record": metadata,
                "target_context": "classification",
                "action": action,
                "reason_code": reason,
            },
        ),
        record_id=normalized.record_id,
        record_kind=normalized.record_kind,
        target_context="classification",
        action=action,
        reason_code=reason,
        metadata_only=True,
        default_posture=_default_posture(normalized, diagnostics),
        diagnostics=tuple(diagnostics),
        metadata=metadata,
    )


def guard_storage_records(
    records: Iterable[StorageRecord | Mapping[str, Any]],
    *,
    target_context: str,
    explicit_user_intent: bool = False,
    explicit_local_private_intent: bool | None = None,
) -> StorageGuardResult:
    """Return context-specific guard decisions and a safe metadata manifest."""

    if explicit_local_private_intent is not None:
        explicit_user_intent = explicit_local_private_intent
    if target_context not in STORAGE_CONTEXTS:
        raise ValueError(f"unsupported target_context: {target_context}")

    decisions: list[StorageDecision] = []
    diagnostics: list[StorageDiagnostic] = []
    manifest: list[dict[str, Any]] = []

    for source in records:
        record = _normalize_storage_record(source)
        decision = _guard_record(
            record,
            target_context=target_context,
            explicit_user_intent=explicit_user_intent,
        )
        decisions.append(decision)
        diagnostics.extend(decision.diagnostics)
        manifest.append(_safe_manifest_metadata(record, target_context))

    return StorageGuardResult(
        target_context=target_context,
        decisions=tuple(decisions),
        diagnostics=tuple(diagnostics),
        safe_manifest=tuple(manifest),
    )


def _normalize_storage_record(record: StorageRecord | Mapping[str, Any]) -> StorageRecord:
    if isinstance(record, StorageRecord):
        return record
    if not isinstance(record, Mapping):
        raise TypeError("record must be a StorageRecord or mapping")

    key_map = {str(key).lower(): value for key, value in record.items()}
    kind = _storage_kind(_string(key_map.get("record_kind") or key_map.get("kind"), "unknown"))
    if kind not in STORAGE_RECORD_KINDS:
        kind = _infer_record_kind(key_map, kind)

    storage_locality = _string(
        key_map.get("storage_locality")
        or key_map.get("symbolic_path_class")
        or key_map.get("path_class"),
        _storage_locality_for_kind(kind),
    )
    payload_keys_present = bool(PAYLOAD_KEYS & set(key_map))
    secret_keys_present = bool(SECRET_MATERIAL_KEYS & set(key_map))
    concrete_path_present = bool(key_map.get("concrete_path_present", False)) or any(
        key in key_map and _looks_concrete_path(key_map[key])
        for key in CONCRETE_PATH_KEYS
    )
    cloud_or_network_reference = (
        bool(key_map.get("cloud_or_network_reference", False))
        or storage_locality in BLOCKED_REMOTE_CONTEXTS
        or bool(CLOUD_NETWORK_KEYS & set(key_map))
    )
    direct_sql_access = (
        bool(key_map.get("direct_sql_access", False))
        or storage_locality in DIRECT_SQL_CONTEXTS
        or bool(DIRECT_SQL_KEYS & set(key_map))
    )
    contains_payload = bool(key_map.get("contains_payload", False)) or payload_keys_present
    secret_material_present = (
        bool(key_map.get("secret_material_present", False))
        or secret_keys_present
        or (_looks_secret_like(kind, key_map) and payload_keys_present)
    )
    unsafe_detail_present = (
        contains_payload
        or secret_material_present
        or concrete_path_present
        or cloud_or_network_reference
        or direct_sql_access
    )

    source_note = _string(key_map.get("source_note"), "TBD")
    value_descriptor = _string(
        key_map.get("value_descriptor"),
        "mapping storage record normalized without payload copy",
    )
    if unsafe_detail_present:
        source_note = "unsafe storage detail withheld"
        value_descriptor = "metadata-only normalized record; raw storage detail withheld"

    return StorageRecord(
        record_id=_string(key_map.get("record_id") or key_map.get("id"), "unknown"),
        record_kind=kind,
        label=_string(key_map.get("label") or key_map.get("name"), "unknown"),
        storage_locality=storage_locality,
        privacy_classification=_string(
            key_map.get("privacy_classification"), _privacy_for_kind(kind)
        ),
        redistribution_status=_string(
            key_map.get("redistribution_status"), "unknown"
        ),
        review_status=_string(key_map.get("review_status"), "unknown"),
        source_state=_string(key_map.get("source_state"), "unknown"),
        source_note=source_note,
        checksum=_optional_string(key_map.get("checksum")),
        checksum_status=_string(key_map.get("checksum_status"), "TBD"),
        value_descriptor=value_descriptor,
        contains_payload=contains_payload,
        secret_material_present=secret_material_present,
        concrete_path_present=concrete_path_present,
        cloud_or_network_reference=cloud_or_network_reference,
        direct_sql_access=direct_sql_access,
        unresolved_tbd=_tuple_strings(key_map.get("unresolved_tbd", ()))
        + _unresolved_items(
            storage_locality=key_map.get("storage_locality"),
            privacy_classification=key_map.get("privacy_classification"),
            redistribution_status=key_map.get("redistribution_status"),
            review_status=key_map.get("review_status"),
            source_state=key_map.get("source_state"),
            source_note=key_map.get("source_note"),
            checksum_status=key_map.get("checksum_status"),
        ),
    )


def _classification_diagnostics(record: StorageRecord) -> list[StorageDiagnostic]:
    diagnostics: list[StorageDiagnostic] = []
    path = record.record_id

    if record.record_kind == "unknown":
        diagnostics.append(
            _diagnostic(
                "STORAGE_RECORD_KIND_UNKNOWN",
                "WARNING",
                path,
                "Storage record kind is unresolved.",
                "Record an explicit project, rule-pack, library, report, diagnostic, cache, secret, or public metadata kind.",
            )
        )
    if record.cloud_or_network_reference:
        diagnostics.append(
            _diagnostic(
                "CLOUD_OR_NETWORK_STORAGE_BLOCKED",
                "BLOCKING",
                path,
                "Cloud or network storage metadata was requested or detected.",
                "Keep MVP storage local-only unless a later approved workflow authorizes an exception.",
            )
        )
    if record.direct_sql_access:
        diagnostics.append(
            _diagnostic(
                "DIRECT_SQL_ACCESS_BLOCKED",
                "BLOCKING",
                path,
                "Direct SQL or raw SQLite interface metadata was requested or detected.",
                "Use application-service persistence boundaries instead of direct SQL/table access.",
            )
        )
    if record.secret_material_present:
        diagnostics.append(
            _diagnostic(
                "SECRET_MATERIAL_METADATA_ONLY_REQUIRED",
                "BLOCKING",
                path,
                "Secret-like material is not allowed in a storage metadata record.",
                "Replace secret material with an opaque local reference descriptor.",
            )
        )
    if record.contains_payload:
        diagnostics.append(
            _diagnostic(
                "STORAGE_PAYLOAD_METADATA_ONLY_REQUIRED",
                "BLOCKING",
                path,
                "Payload data is not allowed in a storage metadata record.",
                "Keep project, rule, material, component, report, diagnostic, cache, and secret payloads outside the metadata record.",
            )
        )
    if record.concrete_path_present:
        diagnostics.append(
            _diagnostic(
                "CONCRETE_PATH_REDUCED_TO_SAFE_METADATA",
                "WARNING",
                path,
                "Concrete path-like detail was detected and withheld from the safe metadata representation.",
                "Use symbolic storage classes or opaque local references instead of concrete paths.",
            )
        )
    if record.storage_locality == "PUBLIC_REPOSITORY_CONTENT" and _is_private(record):
        diagnostics.append(
            _diagnostic(
                "PUBLIC_REPOSITORY_PRIVATE_STORAGE_BLOCKED",
                "BLOCKING",
                path,
                "Private storage metadata targets public repository content.",
                "Keep private storage records outside public repository paths by default.",
            )
        )
    if record.redistribution_status in UNKNOWN_STATUSES:
        diagnostics.append(
            _diagnostic(
                "REDISTRIBUTION_STATUS_UNRESOLVED",
                "WARNING",
                path,
                "Redistribution status is unresolved.",
                "Record redistribution status before public/shared storage handling.",
            )
        )
    if record.privacy_classification in UNKNOWN_STATUSES:
        diagnostics.append(
            _diagnostic(
                "PRIVACY_CLASSIFICATION_UNRESOLVED",
                "WARNING",
                path,
                "Privacy classification is unresolved.",
                "Record explicit privacy classification before public/shared storage handling.",
            )
        )
    if record.source_state in UNKNOWN_STATUSES:
        diagnostics.append(
            _diagnostic(
                "SOURCE_STATE_UNRESOLVED",
                "WARNING",
                path,
                "Source/provenance state is unresolved.",
                "Record source state and source note for the metadata record.",
            )
        )
    if record.review_status in BLOCKING_REVIEW_STATUSES:
        diagnostics.append(
            _diagnostic(
                "REVIEW_DISPOSITION_BLOCKED",
                "BLOCKING",
                path,
                "Review disposition blocks storage handling.",
                "Resolve rejected or quarantined disposition before use.",
            )
        )
    if (
        record.redistribution_status in PROTECTED_STATUSES
        or record.review_status in PROTECTED_STATUSES
    ):
        diagnostics.append(
            _diagnostic(
                "PROTECTED_SOURCE_SUSPECTED",
                "BLOCKING",
                path,
                "Storage metadata indicates suspected protected source content.",
                "Route for human review and keep it out of public artifacts.",
            )
        )
    return diagnostics


def _guard_record(
    record: StorageRecord,
    *,
    target_context: str,
    explicit_user_intent: bool,
) -> StorageDecision:
    diagnostics = _classification_diagnostics(record)
    action, reason = _guard_action(
        record,
        target_context=target_context,
        explicit_user_intent=explicit_user_intent,
    )
    if reason not in {diagnostic.code for diagnostic in diagnostics}:
        diagnostics.append(
            _diagnostic(
                reason,
                _severity_for_action(action),
                record.record_id,
                _message_for_reason(reason),
                _remediation_for_reason(reason),
            )
        )

    metadata = _safe_manifest_metadata(record, target_context)
    return StorageDecision(
        decision_id=_stable_id(
            "LFS-GRD",
            {
                "record": metadata,
                "target_context": target_context,
                "explicit_user_intent": explicit_user_intent,
                "action": action,
                "reason_code": reason,
            },
        ),
        record_id=record.record_id,
        record_kind=record.record_kind,
        target_context=target_context,
        action=action,
        reason_code=reason,
        metadata_only=True,
        default_posture=_default_posture(record, diagnostics),
        diagnostics=tuple(diagnostics),
        metadata=metadata,
    )


def _guard_action(
    record: StorageRecord,
    *,
    target_context: str,
    explicit_user_intent: bool,
) -> tuple[str, str]:
    if target_context in BLOCKED_REMOTE_CONTEXTS:
        return "block_storage", "CLOUD_OR_NETWORK_CONTEXT_BLOCKED"
    if target_context in DIRECT_SQL_CONTEXTS:
        return "block_storage", "DIRECT_SQL_CONTEXT_BLOCKED"
    if record.cloud_or_network_reference:
        return "block_storage", "CLOUD_OR_NETWORK_STORAGE_BLOCKED"
    if record.direct_sql_access:
        return "block_storage", "DIRECT_SQL_ACCESS_BLOCKED"
    if record.secret_material_present:
        return "block_storage", "SECRET_MATERIAL_METADATA_ONLY_REQUIRED"
    if record.contains_payload:
        return "block_storage", "STORAGE_PAYLOAD_METADATA_ONLY_REQUIRED"
    if record.review_status in BLOCKING_REVIEW_STATUSES:
        return "block_storage", "REVIEW_DISPOSITION_BLOCKED"
    if (
        record.redistribution_status in PROTECTED_STATUSES
        or record.review_status in PROTECTED_STATUSES
    ):
        return "block_storage", "PROTECTED_SOURCE_SUSPECTED"

    if target_context in PUBLIC_SHARED_CONTEXTS:
        if record.concrete_path_present:
            return "block_storage", "CONCRETE_PATH_PUBLIC_LEAKAGE_BLOCKED"
        if _is_private(record):
            return "block_storage", "PRIVATE_STORAGE_PUBLIC_CONTEXT_BLOCKED"
        if record.privacy_classification in UNKNOWN_STATUSES:
            return "block_storage", "UNKNOWN_PRIVACY_PUBLIC_CONTEXT_BLOCKED"
        if record.redistribution_status in UNKNOWN_STATUSES:
            return "block_storage", "UNKNOWN_REDIS_PUBLIC_CONTEXT_BLOCKED"
        return "include_metadata_only", "SAFE_PUBLIC_METADATA"

    if target_context in LOCAL_PRIVATE_CONTEXTS:
        if _is_private(record) and not explicit_user_intent:
            return "block_storage", "LOCAL_PRIVATE_INTENT_REQUIRED"
        if record.concrete_path_present:
            return "include_metadata_only", "CONCRETE_PATH_REDUCED_TO_SAFE_METADATA"
        if _is_private(record):
            return "include_metadata_only", "PRIVATE_LOCAL_METADATA_ALLOWED"
        return "include_metadata_only", "SAFE_PUBLIC_METADATA"

    return "block_storage", "UNSUPPORTED_STORAGE_CONTEXT"


def _safe_manifest_metadata(record: StorageRecord, target_context: str) -> dict[str, Any]:
    metadata = record.metadata_dict()
    if record.concrete_path_present:
        metadata["storage_locality"] = (
            record.storage_locality
            if record.storage_locality in SYMBOLIC_STORAGE_CLASSES
            else "concrete_path_detail_withheld"
        )
        metadata["source_note"] = "concrete_path_detail_withheld"
        metadata["value_descriptor"] = "metadata-only storage reference; concrete detail withheld"
    if record.cloud_or_network_reference:
        metadata["source_note"] = "cloud_or_network_detail_withheld"
        metadata["value_descriptor"] = "metadata-only storage reference; cloud/network detail withheld"
    if record.direct_sql_access:
        metadata["source_note"] = "direct_sql_detail_withheld"
        metadata["value_descriptor"] = "metadata-only storage reference; direct SQL detail withheld"
    if record.secret_material_present:
        metadata["source_note"] = "secret_material_withheld"
        metadata["value_descriptor"] = "metadata-only storage reference; secret material withheld"
    if record.contains_payload:
        metadata["source_note"] = "payload_withheld"
        metadata["value_descriptor"] = "metadata-only storage reference; payload withheld"
    if target_context in PUBLIC_SHARED_CONTEXTS and _is_private(record):
        metadata["checksum"] = None
        metadata["checksum_status"] = "withheld_private_metadata"
        metadata["source_note"] = "withheld_private_metadata"
        metadata["value_descriptor"] = "metadata-only private storage record; private detail withheld"
    return metadata


def _classification_reason(
    record: StorageRecord, diagnostics: list[StorageDiagnostic]
) -> str:
    if _has_blocking(diagnostics):
        return next(
            diagnostic.code
            for diagnostic in diagnostics
            if diagnostic.severity == "BLOCKING"
        )
    if _is_private(record):
        return "PRIVATE_METADATA_LOCAL_FIRST"
    if record.privacy_classification in PUBLIC_PRIVACY_CLASSES:
        return "SAFE_PUBLIC_METADATA"
    return "REVIEW_REQUIRED"


def _default_posture(
    record: StorageRecord, diagnostics: list[StorageDiagnostic]
) -> str:
    if _has_blocking(diagnostics):
        return "blocked_until_metadata_only_local_first"
    if _is_private(record):
        return "local_private_metadata_requires_explicit_intent"
    if record.privacy_classification in PUBLIC_PRIVACY_CLASSES:
        return "public_metadata_allowed"
    return "review_required"


def _diagnostic(
    code: str, severity: str, path: str, message: str, remediation: str
) -> StorageDiagnostic:
    return StorageDiagnostic(
        code=code,
        severity=severity,
        path=path,
        message=message,
        remediation=remediation,
    )


def _message_for_reason(reason: str) -> str:
    return {
        "CLOUD_OR_NETWORK_CONTEXT_BLOCKED": "Cloud or network storage context is not authorized for MVP local-first storage.",
        "DIRECT_SQL_CONTEXT_BLOCKED": "Direct SQL/raw SQLite context is not an authorized storage boundary.",
        "CLOUD_OR_NETWORK_STORAGE_BLOCKED": "Cloud or network storage metadata is blocked.",
        "DIRECT_SQL_ACCESS_BLOCKED": "Direct SQL/raw SQLite access metadata is blocked.",
        "SECRET_MATERIAL_METADATA_ONLY_REQUIRED": "Secret material is not allowed in storage metadata.",
        "STORAGE_PAYLOAD_METADATA_ONLY_REQUIRED": "Payload material is not allowed in storage metadata.",
        "REVIEW_DISPOSITION_BLOCKED": "Review disposition blocks storage handling.",
        "PROTECTED_SOURCE_SUSPECTED": "Storage metadata indicates suspected protected source content.",
        "CONCRETE_PATH_PUBLIC_LEAKAGE_BLOCKED": "Concrete path-like detail cannot enter public/shared storage metadata.",
        "PRIVATE_STORAGE_PUBLIC_CONTEXT_BLOCKED": "Private storage metadata cannot enter public repository or shared contexts by default.",
        "UNKNOWN_PRIVACY_PUBLIC_CONTEXT_BLOCKED": "Unknown privacy classification cannot enter public/shared storage metadata.",
        "UNKNOWN_REDIS_PUBLIC_CONTEXT_BLOCKED": "Unknown redistribution status cannot enter public/shared storage metadata.",
        "LOCAL_PRIVATE_INTENT_REQUIRED": "Local/private storage metadata requires explicit user intent.",
        "CONCRETE_PATH_REDUCED_TO_SAFE_METADATA": "Concrete path-like detail was reduced to safe metadata.",
        "PRIVATE_LOCAL_METADATA_ALLOWED": "Private metadata is retained for local/private handling with explicit user intent.",
        "SAFE_PUBLIC_METADATA": "Public metadata is allowed for the selected storage context.",
        "UNSUPPORTED_STORAGE_CONTEXT": "Storage context is unsupported.",
    }[reason]


def _remediation_for_reason(reason: str) -> str:
    return {
        "CLOUD_OR_NETWORK_CONTEXT_BLOCKED": "Use local-only storage unless a later approved exception workflow exists.",
        "DIRECT_SQL_CONTEXT_BLOCKED": "Use application-service persistence boundaries, not direct SQL/table access.",
        "CLOUD_OR_NETWORK_STORAGE_BLOCKED": "Remove cloud/network storage references from the metadata record.",
        "DIRECT_SQL_ACCESS_BLOCKED": "Remove direct SQL, table, or raw SQLite handle references.",
        "SECRET_MATERIAL_METADATA_ONLY_REQUIRED": "Replace secret material with an opaque local reference descriptor.",
        "STORAGE_PAYLOAD_METADATA_ONLY_REQUIRED": "Store only metadata and keep payload values outside this record.",
        "REVIEW_DISPOSITION_BLOCKED": "Resolve review disposition before storage handling.",
        "PROTECTED_SOURCE_SUSPECTED": "Route for human review and keep it out of public artifacts.",
        "CONCRETE_PATH_PUBLIC_LEAKAGE_BLOCKED": "Use symbolic storage classes or opaque references only.",
        "PRIVATE_STORAGE_PUBLIC_CONTEXT_BLOCKED": "Keep private storage records local/private unless an explicit reviewed export workflow applies.",
        "UNKNOWN_PRIVACY_PUBLIC_CONTEXT_BLOCKED": "Record explicit privacy classification before public/shared handling.",
        "UNKNOWN_REDIS_PUBLIC_CONTEXT_BLOCKED": "Record redistribution status before public/shared handling.",
        "LOCAL_PRIVATE_INTENT_REQUIRED": "Record explicit user intent before retaining private local metadata.",
        "CONCRETE_PATH_REDUCED_TO_SAFE_METADATA": "Verify only symbolic/opaque metadata remains in downstream artifacts.",
        "PRIVATE_LOCAL_METADATA_ALLOWED": "Keep the record local and do not attach payloads, concrete paths, direct SQL, cloud/network targets, or secrets.",
        "SAFE_PUBLIC_METADATA": "No remediation required.",
        "UNSUPPORTED_STORAGE_CONTEXT": "Select a supported local/private or public/shared metadata context.",
    }[reason]


def _severity_for_action(action: str) -> str:
    return "BLOCKING" if action == "block_storage" else "WARNING"


def _has_blocking(diagnostics: list[StorageDiagnostic]) -> bool:
    return any(diagnostic.severity == "BLOCKING" for diagnostic in diagnostics)


def _storage_kind(record_kind: str) -> str:
    kind = record_kind.strip().lower()
    aliases = {
        "project": "private_project",
        "project_data": "private_project",
        "project_model_data": "project_model",
        "rule": "private_rule_pack",
        "rule_pack": "private_rule_pack",
        "rule-pack": "private_rule_pack",
        "material": "private_material_library",
        "material_library": "private_material_library",
        "component": "private_component_library",
        "component_library": "private_component_library",
        "report": "private_report",
        "diagnostic": "private_diagnostic",
        "cache": "private_cache",
        "secret": "secret_reference",
        "credential": "secret_reference",
        "public": "public_metadata",
    }
    return aliases.get(kind, kind)


def _privacy_for_kind(kind: str) -> str:
    if kind in {"private_project", "project_model", "project_package", "project_store"}:
        return "private_project_data"
    if kind == "private_rule_pack":
        return "private_rule_pack_data"
    if kind == "private_material_library":
        return "private_material_data"
    if kind == "private_component_library":
        return "private_component_data"
    if kind == "private_report":
        return "private_report_data"
    if kind == "private_diagnostic":
        return "private_diagnostic_data"
    if kind == "private_cache":
        return "private_cache_data"
    if kind == "secret_reference":
        return "secret_like_data"
    if kind in PUBLIC_RECORD_KINDS:
        return "public_metadata"
    return "unknown"


def _storage_locality_for_kind(kind: str) -> str:
    if kind in {"private_project", "project_model", "project_package", "project_store"}:
        return "USER_CHOSEN_PROJECT_PACKAGE"
    if kind == "private_rule_pack":
        return "USER_PRIVATE_RULE_PACK_ROOT"
    if kind in {"private_material_library", "private_component_library"}:
        return "USER_PRIVATE_LIBRARY_ROOT"
    if kind == "private_report":
        return "USER_REPORT_OUTPUT_ROOT"
    if kind == "private_diagnostic":
        return "USER_DIAGNOSTIC_BUNDLE_ROOT"
    if kind == "private_cache":
        return "LOCAL_CACHE_ROOT"
    if kind == "secret_reference":
        return "USER_SECRET_REFERENCE"
    if kind == "public_example_metadata":
        return "PUBLIC_EXAMPLE_CONTENT"
    return "PUBLIC_REPOSITORY_CONTENT" if kind == "public_metadata" else "unknown"


def _infer_record_kind(key_map: Mapping[str, Any], fallback: str) -> str:
    labels = " ".join(
        _string(key_map.get(key), "")
        for key in ("record_id", "id", "label", "name", "field_name")
    ).lower()
    if any(marker in labels for marker in SECRET_NAME_MARKERS):
        return "secret_reference"
    if "rule" in labels:
        return "private_rule_pack"
    if "material" in labels:
        return "private_material_library"
    if "component" in labels:
        return "private_component_library"
    if "report" in labels:
        return "private_report"
    if "diagnostic" in labels:
        return "private_diagnostic"
    if "cache" in labels:
        return "private_cache"
    if "project" in labels or "model" in labels:
        return "private_project"
    return fallback


def _is_private(record: StorageRecord) -> bool:
    return (
        record.record_kind in PRIVATE_RECORD_KINDS
        or record.privacy_classification in PRIVATE_PRIVACY_CLASSES
        or record.redistribution_status in PRIVATE_REDIS_STATUSES
        or record.storage_locality
        in {
            "USER_CHOSEN_PROJECT_PACKAGE",
            "USER_PRIVATE_LIBRARY_ROOT",
            "USER_PRIVATE_RULE_PACK_ROOT",
            "USER_REPORT_OUTPUT_ROOT",
            "USER_DIAGNOSTIC_BUNDLE_ROOT",
            "LOCAL_CACHE_ROOT",
            "USER_SECRET_REFERENCE",
            "local_private",
        }
    )


def _looks_secret_like(kind: str, key_map: Mapping[str, Any]) -> bool:
    labels = " ".join(
        _string(key_map.get(key), "")
        for key in ("record_id", "id", "label", "name", "field_name")
    ).lower()
    return kind == "secret_reference" or any(
        marker in labels for marker in SECRET_NAME_MARKERS
    )


def _looks_concrete_path(value: Any) -> bool:
    if value in UNKNOWN_STATUSES:
        return False
    text = str(value)
    if text in SYMBOLIC_STORAGE_CLASSES:
        return False
    if text.startswith(("USER_", "PUBLIC_", "LOCAL_CACHE_ROOT")):
        return False
    return any(marker in text for marker in ("/", "\\", ":", "FAKE_CONCRETE"))


def _unresolved_items(**values: Any) -> tuple[str, ...]:
    return tuple(name for name, value in values.items() if value in UNKNOWN_STATUSES)


def _stable_id(prefix: str, payload: Mapping[str, Any]) -> str:
    canonical = json.dumps(
        payload,
        sort_keys=True,
        separators=(",", ":"),
        default=list,
    ).encode("utf-8")
    digest = hashlib.sha256(canonical).hexdigest()[:16]
    return f"{prefix}-{digest}"


def _string(value: Any, default: str) -> str:
    if value is None:
        return default
    if isinstance(value, str):
        return value if value else default
    return str(value)


def _optional_string(value: Any) -> str | None:
    if value is None:
        return None
    if isinstance(value, str):
        return value
    return str(value)


def _tuple_strings(value: Any) -> tuple[str, ...]:
    if value in (None, ""):
        return ()
    if isinstance(value, str):
        return (value,)
    return tuple(str(item) for item in value)
