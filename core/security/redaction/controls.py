"""Bounded local redaction controls for report/export representations.

The functions in this module operate on already-parsed dictionaries/lists and
return a redacted copy. They do not mutate the source payload, read files, move
quarantine material, transmit data, or infer sensitivity from value text.
"""

from __future__ import annotations

from dataclasses import asdict, dataclass, replace
from typing import Any
import copy


REDACTED_VALUE = "[REDACTED]"
OMITTED_SENTINEL = object()

PUBLIC_CONTEXTS = {"public_report", "public_example", "shared_model", "downstream_tool"}
# W-18b (DEL-08-01 Design `report-user-release.md` §3.1): a report of the user's own
# session in which the user releases their own project data for sharing, for that one
# report.  It is admitted only on route DREP-PACKAGE-USER-RELEASE-010 with the wrapper's
# user-release intent; payload content never selects or grants it.  No Python route
# uses it.
USER_RELEASED_REPORT_CONTEXT = "user_released_report"
EXPORT_CONTEXTS = PUBLIC_CONTEXTS | {"local_private", USER_RELEASED_REPORT_CONTEXT}

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
# The private classes other than the user's own project data (§2.7): redacted in the
# user-released context whatever their redistribution status.
NON_PROJECT_PRIVATE_CLASSES = PRIVATE_PRIVACY_CLASSES - {"private_project_data"}
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
# Input contract (docs/security/redaction_export_controls.md, "Input contract"; HELP_HUMAN
# ruling 2026-10-27).  The flag strings all three implementations recognize, after trimming
# and lowercasing; identical in redactionExportControls.ts and redaction_binding.rs.
RECOGNIZED_FLAG_STRINGS = frozenset(
    {"true", "yes", "explicit", "present", "local_private", "explicit_local_private"}
)
# Trimmed from an intent flag: ASCII whitespace only, U+0009-U+000D and U+0020.
INTENT_FLAG_TRIM = "\t\n\x0b\x0c\r "
# Trimmed from a blocking flag: the union of what Python's str.strip(), JavaScript's
# String.prototype.trim (redactionExportControls.ts) and Rust's str::trim (the mirror in
# redaction_binding.rs) removed before the contract, so no implementation reads a blocking
# flag as unset where it read it as set.  The same code points as BLOCKING_FLAG_TRIM there.
BLOCKING_FLAG_TRIM = "".join(
    chr(code_point)
    for code_point in (
        0x0009, 0x000A, 0x000B, 0x000C, 0x000D,
        0x001C, 0x001D, 0x001E, 0x001F,
        0x0020, 0x0085, 0x00A0, 0x1680,
        0x2000, 0x2001, 0x2002, 0x2003, 0x2004, 0x2005,
        0x2006, 0x2007, 0x2008, 0x2009, 0x200A,
        0x2028, 0x2029, 0x202F, 0x205F, 0x3000, 0xFEFF,
    )
)
# A privacy or redistribution label that is not a string (nor null): a stated label that
# matches no class.  It is not unknown, safe, private or protected, and it does not fall
# back to the provenance.
NON_STRING_LABEL = object()

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


def assert_user_release_intent_context(
    export_context: str, explicit_user_release_intent: Any
) -> None:
    """Refuse user-release intent with any context but ``user_released_report``.

    The flag therefore cannot enable export in another context (§3.3).
    """

    if explicit_user_release_intent is None or explicit_user_release_intent is False:
        return
    if export_context != USER_RELEASED_REPORT_CONTEXT:
        raise ValueError(
            "explicit_user_release_intent is admitted only in the "
            f"{USER_RELEASED_REPORT_CONTEXT} context, not in {export_context}"
        )


def _assert_redaction_walk_context(export_context: Any) -> None:
    """Refuse ``user_released_report`` outright, in every call and before any walk
    (§3.2), as the generic route exporter does.

    The walk decides a value-bearing object as a whole and keeps an unlabelled
    value, so in that context it would admit what the user-release screen refuses
    (a marker or a non-project private class on a descendant of a labelled
    object).  ``classify_export_item`` decides one item by the rule.
    """

    if export_context == USER_RELEASED_REPORT_CONTEXT:
        raise ValueError(
            "USER_RELEASE_ROUTE_REQUIRED: the redaction walk does not admit the "
            f"{USER_RELEASED_REPORT_CONTEXT} context; only the user-release projector does"
        )


def redact_export_payload(
    payload: Any,
    *,
    export_context: str,
    explicit_local_private_intent: bool = False,
    explicit_user_release_intent: bool = False,
) -> RedactionResult:
    """Return a redacted export/report representation and findings.

    Sensitivity decisions are based only on explicit metadata fields. A
    value-bearing object without metadata is treated as unresolved and is
    redacted or blocked according to the export context.
    """

    if export_context not in EXPORT_CONTEXTS:
        raise ValueError(f"unsupported export_context: {export_context}")
    assert_user_release_intent_context(export_context, explicit_user_release_intent)
    _assert_redaction_walk_context(export_context)

    return _redact(
        payload,
        export_context=export_context,
        explicit_local_private_intent=explicit_local_private_intent,
        route=False,
    )


def redact_route_projection(
    projected: Any,
    *,
    export_context: str,
    explicit_local_private_intent: bool = False,
    explicit_user_release_intent: bool = False,
) -> RedactionResult:
    """The redaction walk over the route projector's projection, which alone reads
    value-record nodes (``route_control._project``)."""

    if export_context not in EXPORT_CONTEXTS:
        raise ValueError(f"unsupported export_context: {export_context}")
    assert_user_release_intent_context(export_context, explicit_user_release_intent)
    _assert_redaction_walk_context(export_context)
    return _redact(
        projected,
        export_context=export_context,
        explicit_local_private_intent=explicit_local_private_intent,
        route=True,
    )


def _redact(
    payload: Any, *, export_context: str, explicit_local_private_intent: bool, route: bool
) -> RedactionResult:
    decisions: list[RedactionDecision] = []
    findings: list[RedactionFinding] = []
    redacted = _walk(
        copy.deepcopy(payload),
        path="$",
        export_context=export_context,
        explicit_local_private_intent=explicit_local_private_intent,
        decisions=decisions,
        findings=findings,
        route=route,
    )
    if redacted is OMITTED_SENTINEL:
        redacted = None
    return RedactionResult(redacted, tuple(decisions), tuple(findings))


def classify_export_item(
    item: dict[str, Any],
    *,
    export_context: str,
    explicit_local_private_intent: bool = False,
    explicit_user_release_intent: bool = False,
    path: str = "$",
) -> RedactionDecision:
    """Classify a single metadata-bearing export item."""

    if export_context not in EXPORT_CONTEXTS:
        raise ValueError(f"unsupported export_context: {export_context}")
    assert_user_release_intent_context(export_context, explicit_user_release_intent)
    decision, _finding = _classify(
        item,
        path=path,
        export_context=export_context,
        explicit_local_private_intent=explicit_local_private_intent,
        explicit_user_release_intent=explicit_user_release_intent is True,
        decision_index=1,
        finding_index=1,
    )
    return decision


ROUTE_KEY_PREFIX = "__route_key__"


class ValueRecordNode:
    """The route projector's node for a value record: the record as an item, and its
    projected members.  Known by type, never by a member an input could carry (Codex
    privacy review of #1722, P1-1): JSON input holds only plain dicts and lists."""

    __slots__ = ("item", "projected")

    def __init__(self, item: dict[str, Any], projected: dict[str, Any]) -> None:
        self.item = item
        self.projected = projected
ACTION_RANK = {
    "include": 0,
    "warning_only": 1,
    "redact_value": 2,
    "redact_field": 2,
    "omit_field": 3,
    "block_export": 4,
}


def _raised_decision(
    decision: RedactionDecision, floor: RedactionDecision | None, omit: bool
) -> RedactionDecision:
    """A decision raised to a value record's item decision where that is stricter, keeping
    the item's own reason (HELP_HUMAN ruling 1(a)); beneath an omitted unsafe path key,
    omitted unless it blocks (ruling 2(a))."""

    if floor is not None and ACTION_RANK[floor.action] > ACTION_RANK[decision.action]:
        decision = replace(decision, action=floor.action, reason_code=floor.reason_code)
    if floor is not None and omit and decision.action != "block_export":
        decision = replace(decision, action="omit_field", reason_code=floor.reason_code)
    return decision


def _stricter_floor(
    inherited: RedactionDecision | None, own: RedactionDecision | None
) -> RedactionDecision | None:
    """The stricter of two floors; on a tie, the enclosing one."""

    if inherited is None:
        return own
    if own is None:
        return inherited
    return own if ACTION_RANK[own.action] > ACTION_RANK[inherited.action] else inherited


def _walk_value_record(
    node: ValueRecordNode,
    *,
    path: str,
    export_context: str,
    explicit_local_private_intent: bool,
    decisions: list[RedactionDecision],
    findings: list[RedactionFinding],
    floor: RedactionDecision | None,
    inherited_omit: bool,
) -> Any:
    """A value record (HELP_HUMAN decision, 2026-10-28: "take the stricter decision per
    case"): a ``{value, ...}`` record with metadata but no scalar label, which TypeScript
    decided as one item and Python projected.  Its item decision is taken first; a block is
    the whole record's (ruling 3(a)).  Otherwise the record is decided as projected, every
    ``value`` and ``text`` leaf raised to the item decision where that is stricter (1(a)),
    and, when the item decision redacts, the ``privacy_classification`` leaves raised too
    (the item's redaction replaces that member) and every unsafe path key dropped entirely,
    key included (2(a)).  Identical in redactionExportControls.ts (``walkValueRecord``) and the
    Rust mirror (``route_value_record_outcome``)."""

    item = node.item
    item_decision, _ = _classify(
        item,
        path=path,
        export_context=export_context,
        explicit_local_private_intent=explicit_local_private_intent,
        explicit_user_release_intent=False,
        decision_index=len(decisions) + 1,
        finding_index=len(findings) + 1,
    )
    if item_decision.action == "block_export":
        decisions.append(item_decision)
        finding = _finding_for(item_decision, len(findings) + 1)
        if finding is not None:
            findings.append(finding)
        return _apply_action(item, item_decision.action)
    redacts = ACTION_RANK[item_decision.action] >= ACTION_RANK["redact_value"]
    output: dict[str, Any] = {}
    for key, child in node.projected.items():
        member = key[len(ROUTE_KEY_PREFIX):] if key.startswith(ROUTE_KEY_PREFIX) else key
        own_omit = redacts and member in UNSAFE_DETAIL_KEYS
        # The item's redaction replaces ``value``, ``text`` and ``privacy_classification``
        # (``_apply_action``).
        raise_ = own_omit or member in {"value", "text"} or (redacts and member == "privacy_classification")
        # A floor and an omission inherited from an enclosing value record still hold here
        # (Codex privacy review of #1722, P1-3): the stricter of the two floors applies.
        member_floor = _stricter_floor(floor, item_decision if raise_ else None)
        omit = own_omit or inherited_omit
        result = _walk(
            child,
            path=f"{path}.{key}",
            export_context=export_context,
            explicit_local_private_intent=explicit_local_private_intent,
            decisions=decisions,
            findings=findings,
            route=True,
            floor=member_floor,
            omit=omit,
        )
        if not omit and result is not OMITTED_SENTINEL:
            output[key] = result
    return output


def _walk(
    value: Any,
    *,
    path: str,
    export_context: str,
    explicit_local_private_intent: bool,
    decisions: list[RedactionDecision],
    findings: list[RedactionFinding],
    route: bool = False,
    floor: RedactionDecision | None = None,
    omit: bool = False,
) -> Any:
    """The redaction walk.  Beneath a redacting floor the whole node is replaced once its
    decisions are taken, its extra metadata and nested members included, as the enclosing
    item's redaction replaced the member (Codex privacy review of #1722, P1-2); a block
    beneath still blocks."""

    result = _walk_node(
        value,
        path=path,
        export_context=export_context,
        explicit_local_private_intent=explicit_local_private_intent,
        decisions=decisions,
        findings=findings,
        route=route,
        floor=floor,
        omit=omit,
    )
    if (
        floor is not None
        and ACTION_RANK[floor.action] >= ACTION_RANK["redact_value"]
        and result is not OMITTED_SENTINEL
    ):
        return REDACTED_VALUE
    return result


def _walk_node(
    value: Any,
    *,
    path: str,
    export_context: str,
    explicit_local_private_intent: bool,
    decisions: list[RedactionDecision],
    findings: list[RedactionFinding],
    route: bool,
    floor: RedactionDecision | None,
    omit: bool,
) -> Any:
    # The walk serves the five existing contexts only; ``redact_export_payload``
    # refuses ``user_released_report`` before it is reached (§3.2).
    if route and isinstance(value, ValueRecordNode):
        return _walk_value_record(
            value,
            path=path,
            export_context=export_context,
            explicit_local_private_intent=explicit_local_private_intent,
            decisions=decisions,
            findings=findings,
            floor=floor,
            inherited_omit=omit,
        )
    if isinstance(value, dict):
        if _is_value_bearing(value):
            classified, _ = _classify(
                value,
                path=path,
                export_context=export_context,
                explicit_local_private_intent=explicit_local_private_intent,
                explicit_user_release_intent=False,
                decision_index=len(decisions) + 1,
                finding_index=len(findings) + 1,
            )
            decision = _raised_decision(classified, floor, omit)
            decisions.append(decision)
            finding = _finding_for(decision, len(findings) + 1)
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
                route=route,
                floor=floor,
                omit=omit,
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
                route=route,
                floor=floor,
                omit=omit,
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
    explicit_user_release_intent: bool,
    decision_index: int,
    finding_index: int,
) -> tuple[RedactionDecision, RedactionFinding | None]:
    metadata_present = _has_explicit_metadata(item) or bool(UNSAFE_DETAIL_KEYS & set(item))
    field_class = _string_or_unknown(item.get("field_class", "unknown"))
    privacy = _label(item.get("privacy_classification"))
    redistribution = _label(item.get("redistribution_status"))
    review_status = _review_label(item.get("review_status", "unknown"))
    # Payload-carried local-private intent is not read in the user-released context
    # (§3.2): only the wrapper's release intent decides there.
    local_private_intent = (
        False
        if export_context == USER_RELEASED_REPORT_CONTEXT
        else _local_private_intent(
            item,
            explicit_local_private_intent=explicit_local_private_intent,
        )
    )

    provenance = item.get("provenance")
    if isinstance(provenance, dict):
        privacy = (
            privacy
            if privacy is not None
            else _label(provenance.get("privacy_classification"))
        )
        redistribution = (
            redistribution
            if redistribution is not None
            else _label(provenance.get("redistribution_status"))
        )
        review_status = (
            review_status
            if review_status not in {None, "unknown"}
            else _review_label(provenance.get("review_status", "unknown"))
        )

    action, reason = _action_for(
        privacy=privacy,
        redistribution=redistribution,
        review_status=review_status,
        metadata_present=metadata_present,
        export_context=export_context,
        explicit_local_private_intent=local_private_intent,
        explicit_user_release_intent=explicit_user_release_intent,
        professional_claim=_professional_claim(item),
        contains_payload=_contains_payload(item),
        secret_material_present=_secret_material_present(item),
        cloud_or_network_reference=_cloud_or_network_reference(item),
        direct_sql_access=_direct_sql_access(item),
        storage_bypass=_storage_bypass_requested(item),
        concrete_path_present=_concrete_path_present(item),
        public_basis_barred=carries_proto_member(item),
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
    explicit_user_release_intent: bool,
    professional_claim: bool,
    contains_payload: bool,
    secret_material_present: bool,
    cloud_or_network_reference: bool,
    direct_sql_access: bool,
    storage_bypass: bool,
    concrete_path_present: bool,
    public_basis_barred: bool,
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
    if privacy == "protected_suspected" or redistribution == "protected_suspected":
        return "block_export", "PROTECTED_CONTENT_BLOCKED"
    if review_status in {"quarantined", "rejected"}:
        return "block_export", "PROTECTED_CONTENT_BLOCKED"
    if export_context == USER_RELEASED_REPORT_CONTEXT:
        action, reason = _user_released_action(
            privacy=privacy,
            redistribution=redistribution,
            explicit_user_release_intent=explicit_user_release_intent,
        )
    else:
        action, reason = _context_action(
            privacy=privacy,
            redistribution=redistribution,
            metadata_present=metadata_present,
            export_context=export_context,
            explicit_local_private_intent=explicit_local_private_intent,
        )
    # A concrete path redacts only what would not block (input contract, 2026-10-27): it
    # never turns a protected, quarantined/rejected, intent or user-release block into a
    # redaction.
    if concrete_path_present and action != "block_export":
        return "redact_value", "CONCRETE_PATH_REDACTED"
    # Last, after every other step: an own `__proto__` member turns only a final inclusion
    # into the unknown-label decision (`carries_proto_member`); every other decision stands.
    if public_basis_barred and action == "include":
        if export_context == USER_RELEASED_REPORT_CONTEXT:
            return "block_export", "USER_RELEASE_VALUE_UNBOUND"
        return _unknown_action(export_context), "UNKNOWN_PROVENANCE_WARNING"
    return action, reason


def _context_action(
    *,
    privacy: Any,
    redistribution: Any,
    metadata_present: bool,
    export_context: str,
    explicit_local_private_intent: bool,
) -> tuple[str, str]:
    """Step 10 onward in the five existing contexts (unchanged sequence)."""

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


def _user_released_action(
    *, privacy: Any, redistribution: Any, explicit_user_release_intent: bool
) -> tuple[str, str]:
    """Steps 10-13 of §3.2, only for ``user_released_report``.

    They run after the blocks of steps 1-9 and before any unknown-metadata branch;
    the concrete path (step 7) redacts only what they do not block.  Only the
    wrapper's release intent is read; payload-carried intent is not.
    """

    if isinstance(privacy, str) and privacy in NON_PROJECT_PRIVATE_CLASSES:
        return "redact_value", "PRIVATE_DATA_REDACTED"
    if privacy == "private_project_data":
        if explicit_user_release_intent:
            return "warning_only", "USER_RELEASED_PROJECT_DATA"
        return "block_export", "USER_RELEASE_INTENT_REQUIRED"
    if (
        isinstance(privacy, str)
        and privacy in SAFE_PRIVACY_CLASSES
        and isinstance(redistribution, str)
        and redistribution in PUBLIC_REDIS_STATUSES
    ):
        reason = (
            "SAFE_INVENTED_PUBLIC_EXAMPLE"
            if privacy == "invented_public_example"
            else "SAFE_PUBLIC_METADATA"
        )
        return "include", reason
    return "block_export", "USER_RELEASE_VALUE_UNBOUND"


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
        "USER_RELEASED_PROJECT_DATA": "PRIVATE_DATA_WARNING",
        "USER_RELEASE_INTENT_REQUIRED": "PRIVATE_DATA_WARNING",
        "USER_RELEASE_VALUE_UNBOUND": "PROVENANCE_WARNING",
    }[decision.reason_code]
    message = USER_RELEASE_FINDING_MESSAGES.get(decision.reason_code) or {
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


# The user-released context's own messages (§3.1); every other reason keeps its
# action's message.  Identical to the TypeScript mirror's.
USER_RELEASE_FINDING_MESSAGES = {
    "USER_RELEASED_PROJECT_DATA": "Export retained the user's own project data in this report under the user's release of their project data, with a warning.",
    "USER_RELEASE_INTENT_REQUIRED": "Export is blocked: this report carries the user's project data and no user release of that data was selected for it.",
    "USER_RELEASE_VALUE_UNBOUND": "Export is blocked: a user-released report includes only values whose privacy and redistribution metadata it recognizes.",
}


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
    return _blocking_flag(item.get("contains_payload")) or _blocking_flag(
        item.get("payload_present")
    ) or bool(PAYLOAD_DETAIL_KEYS & set(item))


def _secret_material_present(item: dict[str, Any]) -> bool:
    return _blocking_flag(item.get("secret_material_present")) or bool(
        SECRET_MATERIAL_KEYS & set(item)
    )


def _cloud_or_network_reference(item: dict[str, Any]) -> bool:
    return _blocking_flag(item.get("cloud_or_network_reference")) or bool(
        CLOUD_NETWORK_DETAIL_KEYS & set(item)
    )


def _direct_sql_access(item: dict[str, Any]) -> bool:
    return _blocking_flag(item.get("direct_sql_access")) or bool(
        DIRECT_SQL_DETAIL_KEYS & set(item)
    )


def _storage_bypass_requested(item: dict[str, Any]) -> bool:
    return any(
        _blocking_flag(item.get(key))
        for key in (
            "storage_bypass",
            "storage_bypass_requested",
            "direct_storage_bypass",
            "application_service_bypass",
        )
    )


def _concrete_path_present(item: dict[str, Any]) -> bool:
    return _blocking_flag(item.get("concrete_path_present")) or bool(
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
            return _intent_flag(item[key])
    export_policy = item.get("export_policy")
    if isinstance(export_policy, dict):
        for key in (
            "local_private_intent",
            "explicit_local_private_intent",
            "user_intent",
        ):
            if key in export_policy:
                return _intent_flag(export_policy[key])
    return None


def _blocking_flag(value: Any) -> bool:
    """A blocking flag is set by ``True``, a non-zero number, or a recognized string after
    trimming ``BLOCKING_FLAG_TRIM`` (input contract).  A non-JSON scalar keeps ``bool()``,
    as before the contract."""

    if isinstance(value, bool):
        return value
    if isinstance(value, str):
        return value.strip(BLOCKING_FLAG_TRIM).lower() in RECOGNIZED_FLAG_STRINGS
    if isinstance(value, (dict, list, tuple, set)):
        return False
    return bool(value)


def _intent_flag(value: Any) -> bool:
    """An intent flag is set only by ``True`` or a recognized string after trimming ASCII
    whitespace (input contract); a number, an array or an object is not intent."""

    if value is True:
        return True
    if isinstance(value, str):
        return value.strip(INTENT_FLAG_TRIM).lower() in RECOGNIZED_FLAG_STRINGS
    return False


def _professional_claim(item: dict[str, Any]) -> bool:
    """A professional claim blocks unless it is absent, null or ``False`` (input contract)."""

    claim = item.get("professional_claim")
    return not (claim is None or claim is False)


def carries_proto_member(item: Any) -> bool:
    """Whether an object or its ``provenance`` object carries an own ``__proto__`` member.

    Such an object has no public basis, whatever its labels state (HELP_HUMAN ruling,
    2026-10-27, option (b): "fail closed on the key itself, in all three
    implementations"): a ``__proto__`` key in export data is anomalous, and it may never
    lift or restrict a record by accident.  The route projector gives the record no direct
    public basis, and the classifier reads the member as an unknown label only where the
    decision would otherwise be an inclusion, as the last step of the decision order: every
    other decision stands.  This is deliberately stricter than Python's reading before the
    ruling, which took the key as an ordinary member.  Python receives dicts, parsed or
    built by a caller; a ``__proto__`` key is always an ordinary, visible key of a dict.  Identical in redactionExportControls.ts (``carriesProtoMember``) and
    redaction_binding.rs.
    """

    def own(candidate: Any) -> bool:
        return isinstance(candidate, dict) and "__proto__" in candidate

    return own(item) or (isinstance(item, dict) and own(item.get("provenance")))


def _label(value: Any) -> Any:
    """A privacy or redistribution label as the classifier reads it (input contract): a
    string or null as stated, any other value ``NON_STRING_LABEL``."""

    if value is None or isinstance(value, str):
        return value
    return NON_STRING_LABEL


def _review_label(value: Any) -> Any:
    """A review status that is not a string (nor null) reads as ``unknown`` and so gives way
    to the provenance's (input contract)."""

    if value is None or isinstance(value, str):
        return value
    return "unknown"


def _is_scalar_metadata_value(value: Any) -> bool:
    return not isinstance(value, (dict, list, tuple, set))


def _string_or_unknown(value: Any) -> str:
    if value is None:
        return "unknown"
    if isinstance(value, str) and value:
        return value
    return "unknown"
