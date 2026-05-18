"""Deterministic local-first controls for secret/private-library references.

This module handles metadata records only. It never reads referenced files,
stores credential values, stores private-library payloads, moves quarantine
material, contacts a service, or makes rights/professional determinations.
"""

from __future__ import annotations

from dataclasses import asdict, dataclass, field
import hashlib
import json
from typing import Any, Iterable, Mapping


PUBLIC_RELEASE_CONTEXTS = {
    "public_report",
    "public_example",
    "public_fixture",
    "shared_model",
    "downstream_tool",
}
LOCAL_RELEASE_CONTEXTS = {"local_private", "private_registry", "private_diagnostic"}
RELEASE_CONTEXTS = PUBLIC_RELEASE_CONTEXTS | LOCAL_RELEASE_CONTEXTS

PRIVATE_LIBRARY_KINDS = {
    "private_library",
    "private_library_reference",
    "private_material_library",
    "private_component_library",
    "private_section_library",
    "private_rule_pack",
    "owner_design_basis",
}
PRIVATE_PATH_KINDS = {"private_path", "private_path_reference"}
SECRET_KINDS = {"secret_field", "secret_field_reference", "credential_placeholder"}
REFERENCE_KINDS = PRIVATE_LIBRARY_KINDS | PRIVATE_PATH_KINDS | SECRET_KINDS

PRIVATE_PRIVACY_CLASSES = {
    "private_library_data",
    "private_material_data",
    "private_component_data",
    "private_rule_pack_data",
    "private_project_data",
    "owner_standard_data",
    "company_design_basis_data",
    "path_data",
    "secret_like_data",
    "credential_reference",
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
    "library_payload",
    "private_payload",
    "path_payload",
    "material_values",
    "component_values",
    "rule_formula",
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
}
SECRET_NAME_MARKERS = (
    "secret",
    "credential",
    "token",
    "password",
    "api_key",
    "access_key",
    "authorization",
)


@dataclass(frozen=True)
class GuardDiagnostic:
    code: str
    severity: str
    path: str
    message: str
    remediation: str

    def as_schema_dict(self) -> dict[str, str]:
        return asdict(self)


@dataclass(frozen=True)
class ReferenceRecord:
    reference_id: str
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
    unresolved_tbd: tuple[str, ...] = field(default_factory=tuple)

    def metadata_dict(self) -> dict[str, Any]:
        """Return the metadata-only view used by guard manifests."""

        return {
            "reference_id": self.reference_id,
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
            "unresolved_tbd": list(self.unresolved_tbd),
        }


@dataclass(frozen=True)
class ReferenceClassification:
    classification_id: str
    reference_id: str
    record_kind: str
    storage_locality: str
    privacy_classification: str
    redistribution_status: str
    review_status: str
    source_state: str
    checksum: str | None
    checksum_status: str
    default_posture: str
    diagnostics: tuple[GuardDiagnostic, ...]
    metadata: dict[str, Any]

    @property
    def blocked(self) -> bool:
        return any(diagnostic.severity == "BLOCKING" for diagnostic in self.diagnostics)

    def as_schema_dict(self) -> dict[str, Any]:
        data = asdict(self)
        data["diagnostics"] = [
            diagnostic.as_schema_dict() for diagnostic in self.diagnostics
        ]
        return data


@dataclass(frozen=True)
class GuardDecision:
    decision_id: str
    reference_id: str
    record_kind: str
    release_context: str
    action: str
    reason_code: str
    metadata_only: bool

    def as_schema_dict(self) -> dict[str, Any]:
        return asdict(self)


@dataclass(frozen=True)
class GuardResult:
    release_context: str
    decisions: tuple[GuardDecision, ...]
    diagnostics: tuple[GuardDiagnostic, ...]
    safe_manifest: tuple[dict[str, Any], ...]

    @property
    def blocked(self) -> bool:
        return any(decision.action == "block_release" for decision in self.decisions)

    def summary(self) -> dict[str, Any]:
        return {
            "release_context": self.release_context,
            "reference_count": len(self.decisions),
            "blocked": self.blocked,
            "blocking_count": sum(
                1 for diagnostic in self.diagnostics if diagnostic.severity == "BLOCKING"
            ),
            "warning_count": sum(
                1 for diagnostic in self.diagnostics if diagnostic.severity == "WARNING"
            ),
            "metadata_only": all(decision.metadata_only for decision in self.decisions),
            "cloud_transmission_attempted": False,
            "external_secret_manager_used": False,
            "encryption_or_key_management_finalized": False,
            "professional_claims_made": False,
        }

    def as_schema_dict(self) -> dict[str, Any]:
        return {
            "release_context": self.release_context,
            "decisions": [decision.as_schema_dict() for decision in self.decisions],
            "diagnostics": [
                diagnostic.as_schema_dict() for diagnostic in self.diagnostics
            ],
            "safe_manifest": list(self.safe_manifest),
            "summary": self.summary(),
        }


def private_library_reference(
    *,
    reference_id: str,
    library_kind: str,
    label: str,
    version: str = "TBD",
    checksum: str | None = None,
    checksum_status: str = "TBD",
    source_note: str = "TBD",
    redistribution_status: str = "private_only",
    review_status: str = "pending",
    privacy_classification: str | None = None,
    storage_locality: str = "USER_PRIVATE_LIBRARY_ROOT",
    source_state: str = "private_user_supplied",
    contains_payload: bool = False,
) -> ReferenceRecord:
    """Build a metadata-only private-library reference record."""

    kind = _library_kind(library_kind)
    privacy = privacy_classification or _privacy_for_library_kind(kind)
    unresolved = _unresolved_items(
        version=version,
        checksum_status=checksum_status,
        source_note=source_note,
        redistribution_status=redistribution_status,
        review_status=review_status,
    )
    return ReferenceRecord(
        reference_id=reference_id,
        record_kind=kind,
        label=label,
        storage_locality=storage_locality,
        privacy_classification=privacy,
        redistribution_status=redistribution_status,
        review_status=review_status,
        source_state=source_state,
        source_note=source_note,
        checksum=checksum,
        checksum_status=checksum_status,
        value_descriptor=f"metadata-only {kind} reference; version={version}",
        contains_payload=contains_payload,
        unresolved_tbd=unresolved,
    )


def private_path_reference(
    *,
    reference_id: str,
    path_class: str,
    label: str,
    source_note: str = "TBD",
    redistribution_status: str = "private_only",
    review_status: str = "pending",
    storage_locality: str = "local_private",
    contains_payload: bool = False,
) -> ReferenceRecord:
    """Build a symbolic private-path reference record."""

    unresolved = _unresolved_items(
        source_note=source_note,
        redistribution_status=redistribution_status,
        review_status=review_status,
    )
    return ReferenceRecord(
        reference_id=reference_id,
        record_kind="private_path_reference",
        label=label,
        storage_locality=storage_locality,
        privacy_classification="path_data",
        redistribution_status=redistribution_status,
        review_status=review_status,
        source_state="private_user_supplied",
        source_note=source_note,
        checksum=None,
        checksum_status="not_applicable",
        value_descriptor=f"symbolic path class only: {path_class}",
        contains_payload=contains_payload,
        unresolved_tbd=unresolved,
    )


def secret_field_reference(
    *,
    reference_id: str,
    field_name: str,
    secret_descriptor: str,
    placeholder_key_id: str = "fake-key-id:TBD",
    review_status: str = "pending",
    source_note: str = "placeholder descriptor only",
    secret_material_present: bool = False,
) -> ReferenceRecord:
    """Build a secret-like field record that carries no usable value."""

    unresolved = _unresolved_items(
        placeholder_key_id=placeholder_key_id,
        review_status=review_status,
        source_note=source_note,
    )
    return ReferenceRecord(
        reference_id=reference_id,
        record_kind="secret_field_reference",
        label=field_name,
        storage_locality="USER_SECRET_REFERENCE",
        privacy_classification="secret_like_data",
        redistribution_status="private_only",
        review_status=review_status,
        source_state="credential_reference",
        source_note=source_note,
        checksum=None,
        checksum_status="not_applicable",
        value_descriptor=(
            f"secret descriptor={secret_descriptor}; placeholder_key_id={placeholder_key_id}"
        ),
        contains_payload=False,
        secret_material_present=secret_material_present,
        unresolved_tbd=unresolved,
    )


def credential_placeholder(
    *,
    reference_id: str,
    placeholder_key_id: str,
    credential_descriptor: str,
    review_status: str = "pending",
    source_note: str = "placeholder descriptor only",
) -> ReferenceRecord:
    """Build an opaque credential placeholder record."""

    return secret_field_reference(
        reference_id=reference_id,
        field_name="credential_placeholder",
        secret_descriptor=credential_descriptor,
        placeholder_key_id=placeholder_key_id,
        review_status=review_status,
        source_note=source_note,
        secret_material_present=False,
    )


def classify_reference(reference: ReferenceRecord | Mapping[str, Any]) -> ReferenceClassification:
    """Classify one reference using only explicit metadata."""

    record = _normalize_reference(reference)
    diagnostics = _classification_diagnostics(record)
    return ReferenceClassification(
        classification_id=_stable_id("SPL-CLS", record.metadata_dict()),
        reference_id=record.reference_id,
        record_kind=record.record_kind,
        storage_locality=record.storage_locality,
        privacy_classification=record.privacy_classification,
        redistribution_status=record.redistribution_status,
        review_status=record.review_status,
        source_state=record.source_state,
        checksum=record.checksum,
        checksum_status=record.checksum_status,
        default_posture=_default_posture(record, diagnostics),
        diagnostics=tuple(diagnostics),
        metadata=record.metadata_dict(),
    )


def classify_references(
    references: Iterable[ReferenceRecord | Mapping[str, Any]]
) -> tuple[ReferenceClassification, ...]:
    """Classify references in input order with deterministic per-record IDs."""

    return tuple(classify_reference(reference) for reference in references)


def guard_reference_release(
    references: Iterable[ReferenceRecord | Mapping[str, Any]],
    *,
    release_context: str,
    explicit_local_private_intent: bool = False,
) -> GuardResult:
    """Return guard decisions and a metadata-only manifest for references."""

    if release_context not in RELEASE_CONTEXTS:
        raise ValueError(f"unsupported release_context: {release_context}")

    decisions: list[GuardDecision] = []
    diagnostics: list[GuardDiagnostic] = []
    manifest: list[dict[str, Any]] = []

    for index, source in enumerate(references, start=1):
        record = _normalize_reference(source)
        decision, decision_diagnostics = _guard_record(
            record,
            release_context=release_context,
            explicit_local_private_intent=explicit_local_private_intent,
            index=index,
        )
        decisions.append(decision)
        diagnostics.extend(decision_diagnostics)
        manifest.append(record.metadata_dict())

    return GuardResult(
        release_context=release_context,
        decisions=tuple(decisions),
        diagnostics=tuple(diagnostics),
        safe_manifest=tuple(manifest),
    )


def _normalize_reference(reference: ReferenceRecord | Mapping[str, Any]) -> ReferenceRecord:
    if isinstance(reference, ReferenceRecord):
        return reference
    if not isinstance(reference, Mapping):
        raise TypeError("reference must be a ReferenceRecord or mapping")

    key_map = {str(key).lower(): value for key, value in reference.items()}
    record_kind = _string(key_map.get("record_kind") or key_map.get("kind"), "unknown")
    if record_kind not in REFERENCE_KINDS:
        record_kind = _infer_record_kind(key_map, record_kind)

    payload_keys_present = bool(PAYLOAD_KEYS & set(key_map))
    secret_keys_present = bool(SECRET_MATERIAL_KEYS & set(key_map))
    contains_payload = bool(key_map.get("contains_payload", False)) or payload_keys_present
    secret_material_present = (
        bool(key_map.get("secret_material_present", False))
        or secret_keys_present
        or _looks_secret_like(record_kind, key_map)
        and payload_keys_present
    )
    unresolved = _tuple_strings(key_map.get("unresolved_tbd", ()))

    return ReferenceRecord(
        reference_id=_string(key_map.get("reference_id") or key_map.get("id"), "unknown"),
        record_kind=record_kind,
        label=_string(key_map.get("label") or key_map.get("name"), "unknown"),
        storage_locality=_string(key_map.get("storage_locality"), "unknown"),
        privacy_classification=_string(key_map.get("privacy_classification"), "unknown"),
        redistribution_status=_string(key_map.get("redistribution_status"), "unknown"),
        review_status=_string(key_map.get("review_status"), "unknown"),
        source_state=_string(key_map.get("source_state"), "unknown"),
        source_note=_string(key_map.get("source_note"), "TBD"),
        checksum=_optional_string(key_map.get("checksum")),
        checksum_status=_string(key_map.get("checksum_status"), "TBD"),
        value_descriptor=_string(
            key_map.get("value_descriptor"),
            "mapping reference normalized without payload copy",
        ),
        contains_payload=contains_payload,
        secret_material_present=secret_material_present,
        unresolved_tbd=unresolved
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


def _classification_diagnostics(record: ReferenceRecord) -> list[GuardDiagnostic]:
    diagnostics: list[GuardDiagnostic] = []
    path = record.reference_id

    if record.record_kind == "unknown":
        diagnostics.append(
            _diagnostic(
                "REFERENCE_KIND_UNKNOWN",
                "WARNING",
                path,
                "Reference kind is unresolved.",
                "Record an explicit private-library, private-path, or credential-placeholder kind.",
            )
        )
    if record.secret_material_present:
        diagnostics.append(
            _diagnostic(
                "SECRET_MATERIAL_REFERENCE_ONLY_REQUIRED",
                "BLOCKING",
                path,
                "Secret-like material was detected in a reference record.",
                "Replace the material with an opaque reference descriptor before storage or export.",
            )
        )
    if record.contains_payload and _is_private_library(record):
        diagnostics.append(
            _diagnostic(
                "PRIVATE_LIBRARY_PAYLOAD_REFERENCE_ONLY_REQUIRED",
                "BLOCKING",
                path,
                "Private-library payload data is present in a reference record.",
                "Store only metadata, checksum, source note, and review disposition in the reference.",
            )
        )
    if record.contains_payload and _is_private_path(record):
        diagnostics.append(
            _diagnostic(
                "PRIVATE_PATH_PAYLOAD_REFERENCE_ONLY_REQUIRED",
                "BLOCKING",
                path,
                "Private path payload data is present in a reference record.",
                "Store only a symbolic path class or opaque local reference.",
            )
        )
    if record.redistribution_status in UNKNOWN_STATUSES:
        diagnostics.append(
            _diagnostic(
                "REDISTRIBUTION_STATUS_UNRESOLVED",
                "WARNING",
                path,
                "Redistribution status is unresolved.",
                "Record redistribution status before public/shared release.",
            )
        )
    if record.privacy_classification in UNKNOWN_STATUSES:
        diagnostics.append(
            _diagnostic(
                "PRIVACY_CLASSIFICATION_UNRESOLVED",
                "WARNING",
                path,
                "Privacy classification is unresolved.",
                "Record explicit privacy classification before release.",
            )
        )
    if record.source_state in UNKNOWN_STATUSES:
        diagnostics.append(
            _diagnostic(
                "SOURCE_STATE_UNRESOLVED",
                "WARNING",
                path,
                "Source/provenance state is unresolved.",
                "Record source state and source note for the reference.",
            )
        )
    if record.review_status in BLOCKING_REVIEW_STATUSES:
        diagnostics.append(
            _diagnostic(
                "REVIEW_DISPOSITION_BLOCKED",
                "BLOCKING",
                path,
                "Review disposition blocks release.",
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
                "Reference metadata indicates suspected protected source content.",
                "Keep the reference out of public artifacts and route for human review.",
            )
        )
    return diagnostics


def _guard_record(
    record: ReferenceRecord,
    *,
    release_context: str,
    explicit_local_private_intent: bool,
    index: int,
) -> tuple[GuardDecision, list[GuardDiagnostic]]:
    diagnostics = _classification_diagnostics(record)
    action, reason = _guard_action(
        record,
        release_context=release_context,
        explicit_local_private_intent=explicit_local_private_intent,
    )
    severity = "BLOCKING" if action == "block_release" else "WARNING"
    if reason not in {"SAFE_PUBLIC_METADATA", "PRIVATE_LOCAL_METADATA_ALLOWED"}:
        diagnostics.append(
            _diagnostic(
                reason,
                severity,
                record.reference_id,
                _message_for_reason(reason),
                _remediation_for_reason(reason),
            )
        )
    elif reason == "PRIVATE_LOCAL_METADATA_ALLOWED":
        diagnostics.append(
            _diagnostic(
                reason,
                "WARNING",
                record.reference_id,
                "Private metadata is retained for local/private use with explicit intent.",
                "Keep the record local and do not attach private payload values.",
            )
        )

    return (
        GuardDecision(
            decision_id=f"SPL-GRD-{index:04d}",
            reference_id=record.reference_id,
            record_kind=record.record_kind,
            release_context=release_context,
            action=action,
            reason_code=reason,
            metadata_only=True,
        ),
        diagnostics,
    )


def _guard_action(
    record: ReferenceRecord,
    *,
    release_context: str,
    explicit_local_private_intent: bool,
) -> tuple[str, str]:
    if record.secret_material_present:
        return "block_release", "SECRET_MATERIAL_REFERENCE_ONLY_REQUIRED"
    if record.contains_payload and _is_private_library(record):
        return "block_release", "PRIVATE_LIBRARY_PAYLOAD_REFERENCE_ONLY_REQUIRED"
    if record.contains_payload and _is_private_path(record):
        return "block_release", "PRIVATE_PATH_PAYLOAD_REFERENCE_ONLY_REQUIRED"
    if record.review_status in BLOCKING_REVIEW_STATUSES:
        return "block_release", "REVIEW_DISPOSITION_BLOCKED"
    if (
        record.redistribution_status in PROTECTED_STATUSES
        or record.review_status in PROTECTED_STATUSES
    ):
        return "block_release", "PROTECTED_SOURCE_SUSPECTED"

    if release_context in PUBLIC_RELEASE_CONTEXTS:
        if _is_private(record) and record.redistribution_status in UNKNOWN_STATUSES:
            return "block_release", "UNKNOWN_REDIS_PRIVATE_DATA_BLOCKED"
        if record.privacy_classification in UNKNOWN_STATUSES:
            return "block_release", "UNKNOWN_PRIVACY_PUBLIC_BLOCKED"
        if release_context == "public_fixture" and _is_private(record):
            return "block_release", "PRIVATE_REFERENCE_PUBLIC_FIXTURE_BLOCKED"
        if _is_private(record):
            return "include_metadata_only", "PRIVATE_REFERENCE_METADATA_ONLY"
        return "include_metadata_only", "SAFE_PUBLIC_METADATA"

    if _is_private(record) and not explicit_local_private_intent:
        return "block_release", "LOCAL_PRIVATE_INTENT_REQUIRED"
    if _is_private(record):
        return "include_metadata_only", "PRIVATE_LOCAL_METADATA_ALLOWED"
    return "include_metadata_only", "SAFE_PUBLIC_METADATA"


def _default_posture(
    record: ReferenceRecord, diagnostics: list[GuardDiagnostic]
) -> str:
    if any(diagnostic.severity == "BLOCKING" for diagnostic in diagnostics):
        return "blocked_until_reference_only_metadata"
    if _is_private(record):
        return "local_private_metadata_only"
    if record.privacy_classification in PUBLIC_PRIVACY_CLASSES:
        return "public_metadata_allowed"
    return "review_required"


def _diagnostic(
    code: str, severity: str, path: str, message: str, remediation: str
) -> GuardDiagnostic:
    return GuardDiagnostic(
        code=code,
        severity=severity,
        path=path,
        message=message,
        remediation=remediation,
    )


def _message_for_reason(reason: str) -> str:
    return {
        "SECRET_MATERIAL_REFERENCE_ONLY_REQUIRED": "Credential or secret-like material is not allowed in reference records.",
        "PRIVATE_LIBRARY_PAYLOAD_REFERENCE_ONLY_REQUIRED": "Private-library payload data is not allowed in reference records.",
        "PRIVATE_PATH_PAYLOAD_REFERENCE_ONLY_REQUIRED": "Private path payload data is not allowed in reference records.",
        "REVIEW_DISPOSITION_BLOCKED": "Review disposition blocks release.",
        "PROTECTED_SOURCE_SUSPECTED": "Reference metadata indicates suspected protected source content.",
        "UNKNOWN_REDIS_PRIVATE_DATA_BLOCKED": "Private data with unresolved redistribution status cannot enter a public/shared release.",
        "UNKNOWN_PRIVACY_PUBLIC_BLOCKED": "Unresolved privacy classification cannot enter a public/shared release.",
        "PRIVATE_REFERENCE_PUBLIC_FIXTURE_BLOCKED": "Private references are not emitted into public fixtures.",
        "PRIVATE_REFERENCE_METADATA_ONLY": "Only metadata for the private reference is allowed in this release context.",
        "LOCAL_PRIVATE_INTENT_REQUIRED": "Local/private reference use requires explicit local/private intent.",
    }[reason]


def _remediation_for_reason(reason: str) -> str:
    return {
        "SECRET_MATERIAL_REFERENCE_ONLY_REQUIRED": "Use an opaque placeholder descriptor or local credential-reference ID.",
        "PRIVATE_LIBRARY_PAYLOAD_REFERENCE_ONLY_REQUIRED": "Keep library payloads outside the reference manifest.",
        "PRIVATE_PATH_PAYLOAD_REFERENCE_ONLY_REQUIRED": "Use a symbolic path class or opaque local reference only.",
        "REVIEW_DISPOSITION_BLOCKED": "Resolve review disposition before release.",
        "PROTECTED_SOURCE_SUSPECTED": "Route metadata for human review and keep it out of public artifacts.",
        "UNKNOWN_REDIS_PRIVATE_DATA_BLOCKED": "Record redistribution status or keep the item out of public/shared release.",
        "UNKNOWN_PRIVACY_PUBLIC_BLOCKED": "Record privacy classification before release.",
        "PRIVATE_REFERENCE_PUBLIC_FIXTURE_BLOCKED": "Use invented public-example metadata instead of private references.",
        "PRIVATE_REFERENCE_METADATA_ONLY": "Do not include formulas, values, paths, file contents, or credential material.",
        "LOCAL_PRIVATE_INTENT_REQUIRED": "Record explicit local/private intent before retaining private metadata.",
    }[reason]


def _infer_record_kind(key_map: Mapping[str, Any], fallback: str) -> str:
    labels = " ".join(
        _string(key_map.get(key), "")
        for key in ("reference_id", "id", "label", "name", "field_name")
    ).lower()
    if any(marker in labels for marker in SECRET_NAME_MARKERS):
        return "secret_field_reference"
    if "path" in labels:
        return "private_path_reference"
    if "rule" in labels:
        return "private_rule_pack"
    if "material" in labels:
        return "private_material_library"
    if "component" in labels:
        return "private_component_library"
    return fallback


def _library_kind(library_kind: str) -> str:
    kind = library_kind.strip().lower()
    aliases = {
        "material": "private_material_library",
        "component": "private_component_library",
        "section": "private_section_library",
        "rule_pack": "private_rule_pack",
        "rule-pack": "private_rule_pack",
        "owner_design_basis": "owner_design_basis",
    }
    return aliases.get(kind, kind)


def _privacy_for_library_kind(kind: str) -> str:
    if kind == "private_material_library":
        return "private_material_data"
    if kind == "private_component_library":
        return "private_component_data"
    if kind == "private_rule_pack":
        return "private_rule_pack_data"
    if kind == "owner_design_basis":
        return "company_design_basis_data"
    return "private_library_data"


def _is_private_library(record: ReferenceRecord) -> bool:
    return record.record_kind in PRIVATE_LIBRARY_KINDS


def _is_private_path(record: ReferenceRecord) -> bool:
    return record.record_kind in PRIVATE_PATH_KINDS


def _is_secret(record: ReferenceRecord) -> bool:
    return record.record_kind in SECRET_KINDS


def _is_private(record: ReferenceRecord) -> bool:
    return (
        _is_private_library(record)
        or _is_private_path(record)
        or _is_secret(record)
        or record.privacy_classification in PRIVATE_PRIVACY_CLASSES
        or record.redistribution_status in PRIVATE_REDIS_STATUSES
        or record.storage_locality
        in {
            "local_private",
            "USER_PRIVATE_LIBRARY_ROOT",
            "USER_PRIVATE_RULE_PACK_ROOT",
            "USER_SECRET_REFERENCE",
            "USER_CHOSEN_PROJECT_PACKAGE",
        }
    )


def _looks_secret_like(record_kind: str, key_map: Mapping[str, Any]) -> bool:
    labels = " ".join(
        _string(key_map.get(key), "")
        for key in ("reference_id", "id", "label", "name", "field_name")
    ).lower()
    return record_kind in SECRET_KINDS or any(
        marker in labels for marker in SECRET_NAME_MARKERS
    )


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
