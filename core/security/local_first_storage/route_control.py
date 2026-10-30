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
        "DOTH-CSV-011",
        "DOTH-FORMAT-003",
        "DOTH-HANDOFF-002",
        "DOTH-HANDOFF-PACKAGE-008",
        "DOTH-JSON-001",
        "DOTH-MODEL-DOC-007",
        "DOTH-PRIVATE-004",
        "DREP-HTML-SAVE-005",
        "DREP-IPC-003",
        "DREP-JSON-002",
        "DREP-LINT-JSON-007",
        "DREP-PACKAGE-SAVE-009",
        "DREP-PACKAGE-USER-RELEASE-010",
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

# RT-R6 (DEL-08-04-R14): the CSV table save route.  It and every route in
# ``LOCAL_PRIVATE_ONLY_ROUTE_IDS`` are admitted only in the ``local_private`` context
# with explicit intent; any other context is refused (``LOCAL_PRIVATE_ROUTE_REQUIRED``).
# The TypeScript catalogue (``redactionExportControls.ts``) carries the same set.
CSV_TABLE_ROUTE_ID = "DOTH-CSV-011"
LOCAL_PRIVATE_ONLY_ROUTE_IDS = frozenset({CSV_TABLE_ROUTE_ID})

# W-18b (DEL-08-01 Design `report-user-release.md` §3.3): the user-release route and its
# context.  A user-released report is written to the user's own local path by the same
# atomic save, so its route metadata is guarded under the existing ``private_report``
# storage context; the storage vocabulary does not change.  No Python route uses them.
USER_RELEASE_ROUTE_ID = "DREP-PACKAGE-USER-RELEASE-010"
USER_RELEASED_REPORT_CONTEXT = "user_released_report"

ROUTE_STORAGE_CONTEXTS = {
    "public_report": "public_report",
    "public_example": "public_example",
    "shared_model": "shared_model",
    "downstream_tool": "downstream_tool",
    "local_private": "local_private",
    USER_RELEASED_REPORT_CONTEXT: "private_report",
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
    # Recorded only when the route or the context is the user release's, or a
    # user-release intent is stated, so the evidence of every existing route with an
    # absent or false flag keeps its exact members.
    explicit_user_release_intent: bool | None = None

    def as_schema_dict(self) -> dict[str, Any]:
        data = asdict(self)
        if data["explicit_user_release_intent"] is None:
            del data["explicit_user_release_intent"]
        return data


def enforce_local_first_route(
    *,
    route_id: Any,
    export_context: Any,
    explicit_local_private_intent: bool = False,
    explicit_user_release_intent: bool = False,
) -> LocalFirstRouteDecision:
    """Admit a governed route using route metadata only.

    Public/shared/downstream routes retain their existing redaction behavior.
    A local-private route additionally requires intent supplied by its owning
    wrapper.  The user-released context is admitted only on its own route and
    only with the wrapper's user-release intent; user-release intent with any
    other context is refused (``USER_RELEASE_ROUTE_REQUIRED``).  Payload-carried
    intent is never an input to this function.
    """

    normalized_route_id = route_id if isinstance(route_id, str) else ""
    normalized_context = export_context if isinstance(export_context, str) else ""
    normalized_intent = explicit_local_private_intent is True
    release_intent = explicit_user_release_intent is True
    # Any user-release intent other than None or False: the predicate of the flag
    # assertion (``assert_user_release_intent_context``).  It is refused below in every
    # context but ``user_released_report``, so it cannot be admitted in another context.
    release_intent_stated = (
        explicit_user_release_intent is not None and explicit_user_release_intent is not False
    )
    user_release_route = normalized_route_id == USER_RELEASE_ROUTE_ID
    user_release_context = normalized_context == USER_RELEASED_REPORT_CONTEXT
    # Recorded only where it is the subject of the decision, so the evidence of every
    # existing route with an absent or false flag keeps its exact members.
    release_evidence = (
        release_intent
        if user_release_route or user_release_context or release_intent_stated
        else None
    )
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
            explicit_user_release_intent=release_evidence,
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
            explicit_user_release_intent=release_evidence,
        )
    if user_release_route != user_release_context or (
        release_intent_stated and not user_release_context
    ):
        return LocalFirstRouteDecision(
            route_id=normalized_route_id,
            export_context=normalized_context,
            storage_context=storage_context,
            action="block_storage",
            reason_code="USER_RELEASE_ROUTE_REQUIRED",
            blocked=True,
            explicit_local_private_intent=normalized_intent,
            explicit_user_release_intent=release_evidence,
        )
    if user_release_route:
        return _user_release_route_decision(
            route_id=normalized_route_id,
            storage_context=storage_context,
            explicit_local_private_intent=normalized_intent,
            explicit_user_release_intent=release_intent,
        )

    if (
        normalized_route_id in LOCAL_PRIVATE_ONLY_ROUTE_IDS
        and storage_context != "local_private"
    ):
        return LocalFirstRouteDecision(
            route_id=normalized_route_id,
            export_context=normalized_context,
            storage_context=storage_context,
            action="block_storage",
            reason_code="LOCAL_PRIVATE_ROUTE_REQUIRED",
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


def _user_release_route_decision(
    *,
    route_id: str,
    storage_context: str,
    explicit_local_private_intent: bool,
    explicit_user_release_intent: bool,
) -> LocalFirstRouteDecision:
    """Route 010 in ``user_released_report`` (§3.3)."""

    if not explicit_user_release_intent:
        return LocalFirstRouteDecision(
            route_id=route_id,
            export_context=USER_RELEASED_REPORT_CONTEXT,
            storage_context=storage_context,
            action="block_storage",
            reason_code="USER_RELEASE_INTENT_REQUIRED",
            blocked=True,
            explicit_local_private_intent=explicit_local_private_intent,
            explicit_user_release_intent=False,
        )
    record = storage_record(
        record_id=f"route:{route_id}",
        record_kind="private_report",
        label="governed export route metadata",
        storage_locality="USER_REPORT_OUTPUT_ROOT",
        privacy_classification="private_report_data",
        redistribution_status="private_only",
        review_status="accepted",
        source_state="wrapper_owned_route_metadata",
        source_note="governed route metadata",
        checksum_status="not_applicable",
    )
    guarded = guard_storage_records(
        [record],
        target_context=storage_context,
        explicit_local_private_intent=True,
    )
    decision = guarded.decisions[0]
    return LocalFirstRouteDecision(
        route_id=route_id,
        export_context=USER_RELEASED_REPORT_CONTEXT,
        storage_context=storage_context,
        action=decision.action,
        reason_code=(
            decision.reason_code if guarded.blocked else "USER_RELEASE_METADATA_ALLOWED"
        ),
        blocked=guarded.blocked,
        explicit_local_private_intent=explicit_local_private_intent,
        explicit_user_release_intent=True,
    )
