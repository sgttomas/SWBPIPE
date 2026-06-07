"""Bounded local redaction controls for report/export representations.

The functions in this module operate on already-parsed dictionaries/lists and
return a redacted copy. They do not mutate the source payload, read files, move
quarantine material, transmit data, or infer sensitivity from value text.
"""

from __future__ import annotations

from dataclasses import asdict, dataclass
from typing import Any
import copy


REDACTED_VALUE = "[REDACTED]"
OMITTED_SENTINEL = object()

PUBLIC_CONTEXTS = {"public_report", "public_example", "shared_model", "downstream_tool"}
EXPORT_CONTEXTS = PUBLIC_CONTEXTS | {"local_private"}

SAFE_PRIVACY_CLASSES = {"public", "public_metadata", "invented_public_example"}
PRIVATE_PRIVACY_CLASSES = {
    "private_project_data",
    "private_material_data",
    "private_component_data",
    "private_rule_pack_data",
    "owner_standard_data",
    "company_design_basis_data",
    "path_data",
    "secret_like_data",
}
UNKNOWN_PRIVACY_CLASSES = {"unknown", "TBD", None}
PUBLIC_REDIS_STATUSES = {"public_permissive", "invented_non_engineering_example"}
UNKNOWN_REDIS_STATUSES = {"unknown", "TBD", None}
PRIVATE_REDIS_STATUSES = {"private_only"}

REDACTION_METADATA_KEYS = {
    "field_class",
    "privacy_classification",
    "redistribution_status",
    "review_status",
    "provenance",
    "export_policy",
    "professional_claim",
}
STORAGE_PRIVACY_METADATA_KEYS = {
    "contains_payload",
    "payload_present",
    "secret_material_present",
    "concrete_path_present",
    "cloud_or_network_reference",
    "direct_sql_access",
    "storage_bypass",
    "storage_bypass_requested",
    "direct_storage_bypass",
    "application_service_bypass",
    "local_private_intent",
    "explicit_local_private_intent",
    "user_intent",
    "symbolic_path_class",
    "path_class",
    "storage_locality",
}
PAYLOAD_DETAIL_KEYS = {
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
CONCRETE_PATH_DETAIL_KEYS = {
    "path",
    "file_path",
    "absolute_path",
    "local_path",
    "repository_path",
    "db_path",
    "sqlite_path",
    "concrete_path",
    "uri",
}
CLOUD_NETWORK_DETAIL_KEYS = {
    "cloud_url",
    "remote_url",
    "network_location",
    "cloud_bucket",
    "cloud_sync_target",
    "hosted_database",
}
DIRECT_SQL_DETAIL_KEYS = {
    "sql",
    "query",
    "table",
    "table_name",
    "sqlite_handle",
    "raw_sqlite_handle",
    "connection_string",
}
UNSAFE_DETAIL_KEYS = (
    PAYLOAD_DETAIL_KEYS
    | SECRET_MATERIAL_KEYS
    | CONCRETE_PATH_DETAIL_KEYS
    | CLOUD_NETWORK_DETAIL_KEYS
    | DIRECT_SQL_DETAIL_KEYS
)


@dataclass(frozen=True)
class RedactionDecision:
    decision_id: str
    path: str
    field_class: str
    privacy_classification: str
    redistribution_status: str
    review_status: str
    export_context: str
    action: str
    reason_code: str
    source_metadata_present: bool


@dataclass(frozen=True)
class RedactionFinding:
    finding_id: str
    code: str
    class_: str
    severity: str
    path: str
    action: str
    message: str
    remediation: str

    def as_schema_dict(self) -> dict[str, Any]:
        data = asdict(self)
        data["class"] = data.pop("class_")
        return data


@dataclass(frozen=True)
class RedactionResult:
    payload: Any
    decisions: tuple[RedactionDecision, ...]
    findings: tuple[RedactionFinding, ...]

    @property
    def blocked(self) -> bool:
        return any(decision.action == "block_export" for decision in self.decisions)

    def summary(self) -> dict[str, Any]:
        return {
            "decision_count": len(self.decisions),
            "finding_count": len(self.findings),
            "redacted_count": sum(
                1
                for decision in self.decisions
                if decision.action in {"redact_value", "redact_field"}
            ),
            "omitted_count": sum(
                1 for decision in self.decisions if decision.action == "omit_field"
            ),
            "warning_count": sum(
                1 for finding in self.findings if finding.severity == "WARNING"
            ),
            "blocking_count": sum(
                1 for finding in self.findings if finding.severity == "BLOCKING"
            ),
            "cloud_transmission_attempted": False,
            "professional_claims_made": False,
        }


def redact_export_payload(
    payload: Any,
    *,
    export_context: str,
    explicit_local_private_intent: bool = False,
) -> RedactionResult:
    """Return a redacted export/report representation and findings.

    Sensitivity decisions are based only on explicit metadata fields. A
    value-bearing object without metadata is treated as unresolved and is
    redacted or blocked according to the export context.
    """

    if export_context not in EXPORT_CONTEXTS:
        raise ValueError(f"unsupported export_context: {export_context}")

    decisions: list[RedactionDecision] = []
    findings: list[RedactionFinding] = []
    redacted = _walk(
        copy.deepcopy(payload),
        path="$",
        export_context=export_context,
        explicit_local_private_intent=explicit_local_private_intent,
        decisions=decisions,
        findings=findings,
    )
    if redacted is OMITTED_SENTINEL:
        redacted = None
    return RedactionResult(redacted, tuple(decisions), tuple(findings))


def classify_export_item(
    item: dict[str, Any],
    *,
    export_context: str,
    explicit_local_private_intent: bool = False,
    path: str = "$",
) -> RedactionDecision:
    """Classify a single metadata-bearing export item."""

    if export_context not in EXPORT_CONTEXTS:
        raise ValueError(f"unsupported export_context: {export_context}")
    decision, _finding = _classify(
        item,
        path=path,
        export_context=export_context,
        explicit_local_private_intent=explicit_local_private_intent,
        decision_index=1,
        finding_index=1,
    )
    return decision


def _walk(
    value: Any,
    *,
    path: str,
    export_context: str,
    explicit_local_private_intent: bool,
    decisions: list[RedactionDecision],
    findings: list[RedactionFinding],
) -> Any:
    if isinstance(value, dict):
        if _is_value_bearing(value):
            decision, finding = _classify(
                value,
                path=path,
                export_context=export_context,
                explicit_local_private_intent=explicit_local_private_intent,
                decision_index=len(decisions) + 1,
                finding_index=len(findings) + 1,
            )
            decisions.append(decision)
            if finding is not None:
                findings.append(finding)
            return _apply_action(value, decision.action)

        output: dict[str, Any] = {}
        for key, item in value.items():
            child = _walk(
                item,
                path=f"{path}.{key}",
                export_context=export_context,
                explicit_local_private_intent=explicit_local_private_intent,
                decisions=decisions,
                findings=findings,
            )
            if child is not OMITTED_SENTINEL:
                output[key] = child
        return output

    if isinstance(value, list):
        output_list = []
        for index, item in enumerate(value):
            child = _walk(
                item,
                path=f"{path}[{index}]",
                export_context=export_context,
                explicit_local_private_intent=explicit_local_private_intent,
                decisions=decisions,
                findings=findings,
            )
            if child is not OMITTED_SENTINEL:
                output_list.append(child)
        return output_list

    return value


def _is_value_bearing(item: dict[str, Any]) -> bool:
    return (
        "value" in item
        or "text" in item
        or _has_explicit_metadata(item)
        or _has_unsafe_item_detail(item)
    )


def _classify(
    item: dict[str, Any],
    *,
    path: str,
    export_context: str,
    explicit_local_private_intent: bool,
    decision_index: int,
    finding_index: int,
) -> tuple[RedactionDecision, RedactionFinding | None]:
    metadata_present = _has_explicit_metadata(item) or bool(UNSAFE_DETAIL_KEYS & set(item))
    field_class = _string_or_unknown(item.get("field_class", "unknown"))
    privacy = item.get("privacy_classification")
    redistribution = item.get("redistribution_status")
    review_status = item.get("review_status", "unknown")
    local_private_intent = _local_private_intent(
        item,
        explicit_local_private_intent=explicit_local_private_intent,
    )

    provenance = item.get("provenance")
    if isinstance(provenance, dict):
        privacy = privacy if privacy is not None else provenance.get("privacy_classification")
        redistribution = (
            redistribution
            if redistribution is not None
            else provenance.get("redistribution_status")
        )
        review_status = (
            review_status
            if review_status not in {None, "unknown"}
            else provenance.get("review_status", "unknown")
        )

    action, reason = _action_for(
        privacy=privacy,
        redistribution=redistribution,
        review_status=review_status,
        metadata_present=metadata_present,
        export_context=export_context,
        explicit_local_private_intent=local_private_intent,
        professional_claim=bool(item.get("professional_claim", False)),
        contains_payload=_contains_payload(item),
        secret_material_present=_secret_material_present(item),
        cloud_or_network_reference=_cloud_or_network_reference(item),
        direct_sql_access=_direct_sql_access(item),
        storage_bypass=_storage_bypass_requested(item),
        concrete_path_present=_concrete_path_present(item),
    )
    decision = RedactionDecision(
        decision_id=f"REDC-DEC-{decision_index:04d}",
        path=path,
        field_class=field_class,
        privacy_classification=_string_or_unknown(privacy),
        redistribution_status=_string_or_unknown(redistribution),
        review_status=_string_or_unknown(review_status),
        export_context=export_context,
        action=action,
        reason_code=reason,
        source_metadata_present=metadata_present,
    )
    return decision, _finding_for(decision, finding_index)


def _action_for(
    *,
    privacy: Any,
    redistribution: Any,
    review_status: Any,
    metadata_present: bool,
    export_context: str,
    explicit_local_private_intent: bool,
    professional_claim: bool,
    contains_payload: bool,
    secret_material_present: bool,
    cloud_or_network_reference: bool,
    direct_sql_access: bool,
    storage_bypass: bool,
    concrete_path_present: bool,
) -> tuple[str, str]:
    if professional_claim:
        return "block_export", "PROFESSIONAL_BOUNDARY_BLOCKED"
    if cloud_or_network_reference:
        return "block_export", "CLOUD_OR_NETWORK_REFERENCE_BLOCKED"
    if direct_sql_access:
        return "block_export", "DIRECT_SQL_ACCESS_BLOCKED"
    if storage_bypass:
        return "block_export", "STORAGE_BYPASS_BLOCKED"
    if secret_material_present:
        return "block_export", "SECRET_MATERIAL_BLOCKED"
    if contains_payload:
        return "block_export", "PAYLOAD_METADATA_ONLY_REQUIRED"
    if concrete_path_present:
        return "redact_value", "CONCRETE_PATH_REDACTED"
    if privacy == "protected_suspected" or redistribution == "protected_suspected":
        return "block_export", "PROTECTED_CONTENT_BLOCKED"
    if review_status in {"quarantined", "rejected"}:
        return "block_export", "PROTECTED_CONTENT_BLOCKED"
    if not metadata_present:
        return _unknown_action(export_context), "MISSING_METADATA_REDACTED"
    if redistribution in UNKNOWN_REDIS_STATUSES:
        return _unknown_action(export_context), "REDISTRIBUTION_STATUS_UNKNOWN"
    if privacy in UNKNOWN_PRIVACY_CLASSES:
        return _unknown_action(export_context), "UNKNOWN_PROVENANCE_WARNING"
    if privacy in SAFE_PRIVACY_CLASSES and redistribution in PUBLIC_REDIS_STATUSES:
        reason = (
            "SAFE_INVENTED_PUBLIC_EXAMPLE"
            if privacy == "invented_public_example"
            else "SAFE_PUBLIC_METADATA"
        )
        return "include", reason
    if privacy in PRIVATE_PRIVACY_CLASSES or redistribution in PRIVATE_REDIS_STATUSES:
        if export_context == "local_private":
            if explicit_local_private_intent:
                return "warning_only", "PRIVATE_LOCAL_ALLOWED_WITH_WARNING"
            return "block_export", "LOCAL_PRIVATE_INTENT_REQUIRED"
        return "redact_value", "PRIVATE_DATA_REDACTED"
    return _unknown_action(export_context), "UNKNOWN_PROVENANCE_WARNING"


def _unknown_action(export_context: str) -> str:
    if export_context == "local_private":
        return "warning_only"
    return "redact_value"


def _finding_for(
    decision: RedactionDecision, finding_index: int
) -> RedactionFinding | None:
    if decision.action == "include":
        return None

    severity = "BLOCKING" if decision.action == "block_export" else "WARNING"
    finding_class = {
        "PRIVATE_DATA_REDACTED": "PRIVATE_DATA_WARNING",
        "PRIVATE_LOCAL_ALLOWED_WITH_WARNING": "PRIVATE_DATA_WARNING",
        "UNKNOWN_PROVENANCE_WARNING": "PROVENANCE_WARNING",
        "REDISTRIBUTION_STATUS_UNKNOWN": "PROVENANCE_WARNING",
        "MISSING_METADATA_REDACTED": "PROVENANCE_WARNING",
        "PROTECTED_CONTENT_BLOCKED": "IP_BOUNDARY_WARNING",
        "LOCAL_PRIVATE_INTENT_REQUIRED": "PRIVATE_DATA_WARNING",
        "PROFESSIONAL_BOUNDARY_BLOCKED": "PROFESSIONAL_BOUNDARY_WARNING",
        "PAYLOAD_METADATA_ONLY_REQUIRED": "STORAGE_BOUNDARY_WARNING",
        "SECRET_MATERIAL_BLOCKED": "PRIVATE_DATA_WARNING",
        "CLOUD_OR_NETWORK_REFERENCE_BLOCKED": "STORAGE_BOUNDARY_WARNING",
        "DIRECT_SQL_ACCESS_BLOCKED": "STORAGE_BOUNDARY_WARNING",
        "STORAGE_BYPASS_BLOCKED": "STORAGE_BOUNDARY_WARNING",
        "CONCRETE_PATH_REDACTED": "STORAGE_BOUNDARY_WARNING",
    }[decision.reason_code]
    message = {
        "warning_only": "Export retained the value only in a local/private context with an explicit warning.",
        "redact_value": "Export value was replaced because metadata does not support public/shared release.",
        "redact_field": "Export field was redacted because metadata does not support release.",
        "omit_field": "Export field was omitted because metadata does not support release.",
        "block_export": "Export is blocked until the metadata or boundary issue is resolved.",
    }[decision.action]
    remediation = (
        "Record explicit privacy, provenance, redistribution, review, and user-intent metadata before export."
    )
    return RedactionFinding(
        finding_id=f"REDC-FND-{finding_index:04d}",
        code=decision.reason_code,
        class_=finding_class,
        severity=severity,
        path=decision.path,
        action=decision.action,
        message=message,
        remediation=remediation,
    )


def _apply_action(item: dict[str, Any], action: str) -> Any:
    if action in {"include", "warning_only"}:
        return item
    if action == "omit_field":
        return OMITTED_SENTINEL
    redacted = _without_unsafe_detail(item)
    if action == "redact_field":
        return {
            key: redacted[key]
            for key in redacted
            if key in {"field_id", "record_id", "field_class", "privacy_classification"}
        }
    if action == "redact_value":
        if "value" in redacted:
            redacted["value"] = REDACTED_VALUE
        if "text" in redacted:
            redacted["text"] = REDACTED_VALUE
        redacted["privacy_classification"] = "redacted"
        return redacted
    if action == "block_export":
        if "value" in redacted:
            redacted["value"] = REDACTED_VALUE
        if "text" in redacted:
            redacted["text"] = REDACTED_VALUE
        redacted["privacy_classification"] = "redacted"
        return redacted
    raise ValueError(f"unsupported redaction action: {action}")


def _without_unsafe_detail(item: dict[str, Any]) -> dict[str, Any]:
    return {key: value for key, value in item.items() if key not in UNSAFE_DETAIL_KEYS}


def _has_explicit_metadata(item: dict[str, Any]) -> bool:
    if REDACTION_METADATA_KEYS & set(item):
        return True
    return any(
        key in item and _is_scalar_metadata_value(item[key])
        for key in STORAGE_PRIVACY_METADATA_KEYS
    )


def _has_unsafe_item_detail(item: dict[str, Any]) -> bool:
    keys = set(item)
    unsafe_keys = UNSAFE_DETAIL_KEYS & keys
    if not unsafe_keys:
        return False
    if {"field_id", "record_id", "id", "field_class"} & keys:
        return True
    return bool(keys) and keys <= UNSAFE_DETAIL_KEYS


def _contains_payload(item: dict[str, Any]) -> bool:
    return _truthy(item.get("contains_payload")) or _truthy(
        item.get("payload_present")
    ) or bool(PAYLOAD_DETAIL_KEYS & set(item))


def _secret_material_present(item: dict[str, Any]) -> bool:
    return _truthy(item.get("secret_material_present")) or bool(
        SECRET_MATERIAL_KEYS & set(item)
    )


def _cloud_or_network_reference(item: dict[str, Any]) -> bool:
    return _truthy(item.get("cloud_or_network_reference")) or bool(
        CLOUD_NETWORK_DETAIL_KEYS & set(item)
    )


def _direct_sql_access(item: dict[str, Any]) -> bool:
    return _truthy(item.get("direct_sql_access")) or bool(
        DIRECT_SQL_DETAIL_KEYS & set(item)
    )


def _storage_bypass_requested(item: dict[str, Any]) -> bool:
    return any(
        _truthy(item.get(key))
        for key in (
            "storage_bypass",
            "storage_bypass_requested",
            "direct_storage_bypass",
            "application_service_bypass",
        )
    )


def _concrete_path_present(item: dict[str, Any]) -> bool:
    return _truthy(item.get("concrete_path_present")) or bool(
        CONCRETE_PATH_DETAIL_KEYS & set(item)
    )


def _local_private_intent(
    item: dict[str, Any],
    *,
    explicit_local_private_intent: bool,
) -> bool:
    explicit_item_intent = _item_intent(item)
    if explicit_item_intent is False:
        return False
    return explicit_local_private_intent or explicit_item_intent is True


def _item_intent(item: dict[str, Any]) -> bool | None:
    for key in ("local_private_intent", "explicit_local_private_intent", "user_intent"):
        if key in item:
            return _truthy(item[key])
    export_policy = item.get("export_policy")
    if isinstance(export_policy, dict):
        for key in (
            "local_private_intent",
            "explicit_local_private_intent",
            "user_intent",
        ):
            if key in export_policy:
                return _truthy(export_policy[key])
    return None


def _truthy(value: Any) -> bool:
    if isinstance(value, bool):
        return value
    if isinstance(value, str):
        return value.strip().lower() in {
            "true",
            "yes",
            "explicit",
            "present",
            "local_private",
            "explicit_local_private",
        }
    if isinstance(value, (dict, list, tuple, set)):
        return False
    return bool(value)


def _is_scalar_metadata_value(value: Any) -> bool:
    return not isinstance(value, (dict, list, tuple, set))


def _string_or_unknown(value: Any) -> str:
    if value is None:
        return "unknown"
    if isinstance(value, str) and value:
        return value
    return "unknown"
