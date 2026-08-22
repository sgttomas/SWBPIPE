"""Metadata-only local-first enforcement for governed export routes.

The route decision consumes only wrapper-owned route metadata.  It never
reads, classifies, stores, or transmits the payload being exported.
"""

from __future__ import annotations

from dataclasses import asdict, dataclass
from typing import Any

from .controls import guard_storage_records, storage_record


# Existing governed route identifiers.  Unknown identifiers fail closed so a
# new writer cannot acquire storage authority merely by inventing a route ID.
GOVERNED_LOCAL_FIRST_ROUTE_IDS = frozenset(
    {
        "DOTH-CAEPIPE-LOCAL-006",
        "DOTH-FORMAT-003",
        "DOTH-HANDOFF-002",
        "DOTH-JSON-001",
        "DOTH-PRIVATE-004",
        "DREP-HTML-SAVE-005",
        "DREP-IPC-003",
        "DREP-JSON-002",
        "DREP-LINT-JSON-007",
        "DREP-PACKAGE-SAVE-009",
        "DREP-UI-001/DREP-JSON-002",
        "REXC-CORE-001",
        "REXC-CORE-002",
        "REXC-CORE-003",
        "REXC-CORE-004",
        "REXC-CORE-005",
        "REXC-CORE-006",
        "REXC-CORE-007",
        "REXC-CORE-008",
        "REXC-LOSSLESS-COUNT-TEST",
    }
)

ROUTE_STORAGE_CONTEXTS = {
    "public_report": "public_report",
    "public_example": "public_example",
    "shared_model": "shared_model",
    "downstream_tool": "downstream_tool",
    "local_private": "local_private",
}


@dataclass(frozen=True)
class LocalFirstRouteDecision:
    """Safe evidence that one route passed or failed local-first admission."""

    route_id: str
    export_context: str
    storage_context: str
    action: str
    reason_code: str
    blocked: bool
    metadata_only: bool = True
    explicit_local_private_intent: bool = False

    def as_schema_dict(self) -> dict[str, Any]:
        return asdict(self)


def enforce_local_first_route(
    *,
    route_id: Any,
    export_context: Any,
    explicit_local_private_intent: bool = False,
) -> LocalFirstRouteDecision:
    """Admit a governed route using route metadata only.

    Public/shared/downstream routes retain their existing redaction behavior.
    A local-private route additionally requires intent supplied by its owning
    wrapper.  Payload-carried intent is never an input to this function.
    """

    normalized_route_id = route_id if isinstance(route_id, str) else ""
    normalized_context = export_context if isinstance(export_context, str) else ""
    normalized_intent = explicit_local_private_intent is True
    storage_context = ROUTE_STORAGE_CONTEXTS.get(normalized_context, "invalid")

    if normalized_route_id not in GOVERNED_LOCAL_FIRST_ROUTE_IDS:
        return LocalFirstRouteDecision(
            route_id=normalized_route_id,
            export_context=normalized_context,
            storage_context=storage_context,
            action="block_storage",
            reason_code="LOCAL_FIRST_ROUTE_UNKNOWN",
            blocked=True,
            explicit_local_private_intent=normalized_intent,
        )
    if storage_context == "invalid":
        return LocalFirstRouteDecision(
            route_id=normalized_route_id,
            export_context=normalized_context,
            storage_context=storage_context,
            action="block_storage",
            reason_code="LOCAL_FIRST_EXPORT_CONTEXT_INVALID",
            blocked=True,
            explicit_local_private_intent=normalized_intent,
        )

    local_private = storage_context == "local_private"
    record = storage_record(
        record_id=f"route:{normalized_route_id}",
        record_kind="private_report" if local_private else "public_metadata",
        label="governed export route metadata",
        storage_locality=(
            "USER_REPORT_OUTPUT_ROOT" if local_private else "PUBLIC_EXAMPLE_CONTENT"
        ),
        privacy_classification=(
            "private_report_data" if local_private else "public_metadata"
        ),
        redistribution_status="private_only" if local_private else "public_permissive",
        review_status="accepted",
        source_state="wrapper_owned_route_metadata",
        source_note="governed route metadata",
        checksum_status="not_applicable",
    )
    guarded = guard_storage_records(
        [record],
        target_context=storage_context,
        explicit_local_private_intent=normalized_intent,
    )
    decision = guarded.decisions[0]
    return LocalFirstRouteDecision(
        route_id=normalized_route_id,
        export_context=normalized_context,
        storage_context=storage_context,
        action=decision.action,
        reason_code=decision.reason_code,
        blocked=guarded.blocked,
        explicit_local_private_intent=normalized_intent,
    )
