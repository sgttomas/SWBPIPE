"""Decision-support rationale records for DEL-16-04.

The rationale record binds agent reasoning metadata to operation/audit context
without accepting engineering work, applying operations, or mutating accepted
model state.
"""

from __future__ import annotations

from copy import deepcopy
import hashlib
import json
import re
from typing import Any, Mapping


AGENT_RATIONALE_VERSION = "0.1.0"

PROFESSIONAL_BOUNDARY = {
    "human_review_required": True,
    "agent_output_is_decision_support_only": True,
    "software_makes_compliance_claim": False,
    "software_makes_certification_claim": False,
    "software_makes_sealing_claim": False,
    "software_makes_approval_claim": False,
    "software_makes_authentication_claim": False,
    "software_makes_external_validation_claim": False,
    "software_can_accept_engineering_work": False,
    "software_can_mutate_accepted_model_state": False,
}

ENGINE_PROVENANCE = {
    "source_name": "OpenPipeStress DEL-16-04 agent rationale guard",
    "source_location": "core/model_operations/agent_rationale/engine.py",
    "source_license": "project-governed",
    "contributor": "OpenPipeStress Type 2 worker",
    "contributor_certification": "implementation-only-no-professional-claim",
    "redistribution_status": "public_permissive",
    "review_status": "pending",
    "privacy_classification": "public_metadata",
}

PROHIBITED_CLAIM_PATTERNS = {
    "AUTHORITY-COMPLIANCE": re.compile(
        r"\b(code[-\s]?compliant|complies\s+with\s+code|meets\s+code|code\s+approval)\b",
        re.IGNORECASE,
    ),
    "AUTHORITY-CERTIFICATION": re.compile(
        r"\b(certified|certifies|certification|certify)\b",
        re.IGNORECASE,
    ),
    "AUTHORITY-SEALING": re.compile(
        r"\b(sealed|sealing|engineer'?s?\s+seal|professional\s+seal)\b",
        re.IGNORECASE,
    ),
    "AUTHORITY-AUTHENTICATION": re.compile(
        r"\b(authenticated|authenticates|authentication)\b",
        re.IGNORECASE,
    ),
    "AUTHORITY-PROFESSIONAL-APPROVAL": re.compile(
        r"\b(professional\s+approval|approved\s+by\s+(?:a\s+)?professional|engineer\s+approved)\b",
        re.IGNORECASE,
    ),
    "AUTHORITY-EXTERNAL-VALIDATION": re.compile(
        r"\b(externally\s+validated|third[-\s]?party\s+validated|validated\s+by\s+external)\b",
        re.IGNORECASE,
    ),
    "AUTHORITY-AUTONOMOUS-ACCEPTANCE": re.compile(
        r"\b(agent[-\s]?accepted|auto[-\s]?approved|autonomous\s+engineering\s+acceptance|accepted\s+engineering\s+state)\b",
        re.IGNORECASE,
    ),
}


def record_agent_rationale(
    operation_envelope: Mapping[str, Any],
    *,
    audit_trail: Mapping[str, Any] | None = None,
    validation_context: Mapping[str, Any] | None = None,
    source: Mapping[str, Any] | None = None,
    actor: Mapping[str, Any] | None = None,
    rationale_text: str | None = None,
    assumptions: list[Any] | None = None,
    affected_refs: list[Mapping[str, Any]] | None = None,
    audit_references: list[Mapping[str, Any]] | None = None,
    timestamp: str | None = None,
    accepted_model_state: Mapping[str, Any] | None = None,
) -> dict[str, Any]:
    """Create a deterministic rationale record for operation review metadata."""

    source_state = deepcopy(dict(accepted_model_state)) if isinstance(accepted_model_state, Mapping) else None
    operation_set = operation_envelope.get("operation_set", {}) if isinstance(operation_envelope, Mapping) else {}
    operations = _list(operation_set.get("operations")) if isinstance(operation_set, Mapping) else []

    diagnostics = _input_diagnostics(
        operation_envelope,
        audit_trail,
        validation_context,
        source,
        actor,
        rationale_text,
        timestamp,
    )
    diagnostics.extend(
        _claim_diagnostics(
            _claim_scan_payload(
                rationale_text,
                assumptions,
                source,
                actor,
                operation_envelope,
                audit_trail,
                validation_context,
                affected_refs,
                audit_references,
            )
        )
    )

    operation_records = []
    for index, operation in enumerate(operations):
        if not isinstance(operation, Mapping):
            diagnostics.append(
                _diagnostic(
                    "RATIONALE-OPERATION-NOT-OBJECT",
                    "blocking",
                    "RATIONALE_INPUT_BLOCKING",
                    "Operation entry is not a structured object.",
                    "Provide DEL-16-01 operation records before rationale capture.",
                    [_affected("ModelOperation", f"index:{index}")],
                )
            )
            continue
        operation_records.append(_operation_context(operation, index))

    record = {
        "schema_version": AGENT_RATIONALE_VERSION,
        "deliverable_id": "DEL-16-04",
        "package_id": "PKG-16",
        "scope_item": "SOW-070",
        "objectives": ["OBJ-015", "OBJ-018"],
        "operation_set_ref": _operation_set_ref(operation_set),
        "audit_context": _audit_context(audit_trail),
        "source": _source(source),
        "actor": _actor(actor),
        "recorded_at": _first_text(timestamp, "TBD"),
        "rationale": {
            "text": _first_text(rationale_text, "TBD"),
            "status": "blocked_by_professional_boundary"
            if any(item["severity"] == "blocking" for item in diagnostics)
            else "captured_for_user_review",
            "decision_support_only": True,
            "creates_accepted_operation_record": False,
            "mutates_accepted_model_state": False,
            "bypasses_user_acceptance": False,
        },
        "assumptions": _assumptions(assumptions, operation_records),
        "validation_context": _validation_context(validation_context),
        "affected_entities": _affected_entities(operation_records, affected_refs),
        "audit_references": _audit_references(audit_references, audit_trail),
        "operation_context": operation_records,
        "diagnostics": _stable(diagnostics),
        "accepted_model_state_unchanged": (
            True if source_state is None else source_state == accepted_model_state
        ),
        "professional_boundary": deepcopy(PROFESSIONAL_BOUNDARY),
        "provenance": deepcopy(ENGINE_PROVENANCE),
    }
    record["rationale_record_id"] = "rationale:" + _hash_payload(
        {
            "operation_set_ref": record["operation_set_ref"],
            "audit_context": record["audit_context"],
            "source": record["source"],
            "actor": record["actor"],
            "recorded_at": record["recorded_at"],
            "rationale": record["rationale"],
            "assumptions": record["assumptions"],
            "validation_context": record["validation_context"],
            "affected_entities": record["affected_entities"],
            "audit_references": record["audit_references"],
            "operation_context": record["operation_context"],
            "diagnostics": record["diagnostics"],
        }
    )[:24]
    record["rationale_record_hash"] = _hash_payload(record)
    return record


def canonical_json(value: Any) -> str:
    """Serialize rationale output with stable key ordering."""

    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=True)


def _input_diagnostics(
    operation_envelope: Mapping[str, Any],
    audit_trail: Mapping[str, Any] | None,
    validation_context: Mapping[str, Any] | None,
    source: Mapping[str, Any] | None,
    actor: Mapping[str, Any] | None,
    rationale_text: str | None,
    timestamp: str | None,
) -> list[dict[str, Any]]:
    diagnostics = []
    if not isinstance(operation_envelope, Mapping):
        diagnostics.append(
            _diagnostic(
                "RATIONALE-ENVELOPE-NOT-OBJECT",
                "blocking",
                "RATIONALE_INPUT_BLOCKING",
                "Rationale input must include a structured operation envelope.",
                "Provide a DEL-16-01 operation envelope.",
                [_affected("ModelOperationEnvelope", "missing")],
            )
        )
    elif operation_envelope.get("deliverable_id") != "DEL-16-01":
        diagnostics.append(
            _diagnostic(
                "RATIONALE-UPSTREAM-SCHEMA-BINDING",
                "blocking",
                "RATIONALE_INPUT_BLOCKING",
                "Rationale capture consumes DEL-16-01 structured operation records.",
                "Bind rationale input to the upstream model-operation schema.",
                [_affected("deliverable_id", str(operation_envelope.get("deliverable_id")))],
            )
        )
    if audit_trail is None:
        diagnostics.append(_tbd("RATIONALE-AUDIT-CONTEXT-TBD", "Operation audit context is missing."))
    if validation_context is None:
        diagnostics.append(_tbd("RATIONALE-VALIDATION-CONTEXT-TBD", "Validation context is missing."))
    if source is None:
        diagnostics.append(_tbd("RATIONALE-SOURCE-TBD", "Source metadata is missing."))
    if actor is None:
        diagnostics.append(_tbd("RATIONALE-ACTOR-TBD", "Actor metadata is missing."))
    if not rationale_text:
        diagnostics.append(_tbd("RATIONALE-TEXT-TBD", "Agent rationale text is missing."))
    if not timestamp:
        diagnostics.append(_tbd("RATIONALE-TIMESTAMP-TBD", "Rationale timestamp is missing."))
    return diagnostics


def _operation_context(operation: Mapping[str, Any], index: int) -> dict[str, Any]:
    operation_id = str(operation.get("operation_id", f"operation:{index:04d}"))
    return {
        "operation_id": operation_id,
        "operation_kind": str(operation.get("operation_kind", "TBD")),
        "operation_status": str(operation.get("operation_status", "TBD")),
        "author_type": str(operation.get("author_type", "TBD")),
        "target_refs": [_ref(item) for item in _list(operation.get("target_refs")) if isinstance(item, Mapping)],
        "change_refs": [
            {
                "change_id": str(change.get("change_id", f"{operation_id}:change:{change_index:04d}")),
                "change_kind": str(change.get("change_kind", "TBD")),
                "target_ref": _ref(change["target_ref"]) if isinstance(change.get("target_ref"), Mapping) else None,
            }
            for change_index, change in enumerate(_list(operation.get("changes")))
            if isinstance(change, Mapping)
        ],
        "unresolved_assumptions": _unresolved_operation_assumptions(operation),
        "diagnostics": deepcopy(_list(operation.get("diagnostics"))),
        "diff_preview_refs": deepcopy(_list(operation.get("diff_preview_refs"))),
        "validation": deepcopy(operation.get("validation", "TBD")),
        "professional_boundary": deepcopy(operation.get("professional_boundary", "TBD")),
    }


def _assumptions(assumptions: list[Any] | None, operation_records: list[Mapping[str, Any]]) -> dict[str, Any]:
    supplied = [_assumption(item) for item in _list(assumptions)]
    unresolved = [item for item in supplied if _assumption_is_unresolved(item)]
    for operation in operation_records:
        for item in _list(operation.get("unresolved_assumptions")):
            if isinstance(item, Mapping):
                unresolved.append(deepcopy(dict(item)))
    return {
        "supplied": sorted(supplied, key=canonical_json),
        "unresolved": sorted(unresolved, key=canonical_json),
        "unresolved_count": len(unresolved),
        "status": "unresolved_present" if unresolved else "none_supplied" if assumptions is None else "resolved_or_not_flagged",
    }


def _validation_context(validation_context: Mapping[str, Any] | None) -> dict[str, Any]:
    if not isinstance(validation_context, Mapping):
        return {"status": "TBD", "diagnostics": [_tbd("RATIONALE-VALIDATION-CONTEXT-TBD", "Validation context is missing.")]}
    result = deepcopy(dict(validation_context))
    result.setdefault("application_status", "not_applied")
    result.setdefault("validation_context_status", "provided")
    return _stable_mapping(result)


def _audit_context(audit_trail: Mapping[str, Any] | None) -> dict[str, Any]:
    if not isinstance(audit_trail, Mapping):
        return {"status": "TBD", "audit_trail_hash": "TBD", "user_acceptance_required": True}
    return _stable_mapping(
        {
            "status": "provided",
            "audit_trail_hash": _first_text(audit_trail.get("audit_trail_hash"), "TBD"),
            "operation_set_ref": deepcopy(audit_trail.get("operation_set_ref", "TBD")),
            "decision_counts": deepcopy(audit_trail.get("decision_counts", "TBD")),
            "user_acceptance_required": True,
            "rationale_creates_audit_acceptance": False,
        }
    )


def _audit_references(
    audit_references: list[Mapping[str, Any]] | None,
    audit_trail: Mapping[str, Any] | None,
) -> list[dict[str, Any]]:
    refs = [deepcopy(dict(item)) for item in _list(audit_references) if isinstance(item, Mapping)]
    if isinstance(audit_trail, Mapping):
        if audit_trail.get("audit_trail_hash"):
            refs.append({"object_type": "OperationAuditTrail", "ref": str(audit_trail["audit_trail_hash"])})
        for record in _list(audit_trail.get("records")):
            if isinstance(record, Mapping) and record.get("audit_record_id"):
                refs.append({"object_type": "OperationAuditRecord", "ref": str(record["audit_record_id"])})
    deduped = {canonical_json(_stable_mapping(item)): _stable_mapping(item) for item in refs}
    return [deduped[key] for key in sorted(deduped)]


def _affected_entities(
    operation_records: list[Mapping[str, Any]],
    affected_refs: list[Mapping[str, Any]] | None,
) -> list[dict[str, str]]:
    refs = [_ref(item) for item in _list(affected_refs) if isinstance(item, Mapping)]
    for operation in operation_records:
        for item in _list(operation.get("target_refs")):
            if isinstance(item, Mapping):
                refs.append(_ref(item))
        for change in _list(operation.get("change_refs")):
            if isinstance(change, Mapping) and isinstance(change.get("target_ref"), Mapping):
                refs.append(_ref(change["target_ref"]))
    deduped = {canonical_json(item): item for item in refs}
    return [deduped[key] for key in sorted(deduped)]


def _source(source: Mapping[str, Any] | None) -> dict[str, Any]:
    result = deepcopy(dict(source)) if isinstance(source, Mapping) else {}
    result.setdefault("source_ref", "TBD")
    result.setdefault("source_channel", "TBD")
    result.setdefault("source_kind", "agent_rationale")
    return _stable_mapping(result)


def _actor(actor: Mapping[str, Any] | None) -> dict[str, Any]:
    result = deepcopy(dict(actor)) if isinstance(actor, Mapping) else {}
    result.setdefault("actor_type", "TBD")
    result.setdefault("actor_ref", "TBD")
    result.setdefault("actor_role", "TBD")
    return _stable_mapping(result)


def _claim_scan_payload(*values: Any) -> str:
    return "\n".join(_iter_text_values(values))


def _iter_text_values(value: Any):
    if isinstance(value, str):
        yield value
    elif isinstance(value, Mapping):
        for item in value.values():
            yield from _iter_text_values(item)
    elif isinstance(value, (list, tuple)):
        for item in value:
            yield from _iter_text_values(item)


def _claim_diagnostics(text: str) -> list[dict[str, Any]]:
    diagnostics = []
    for code, pattern in PROHIBITED_CLAIM_PATTERNS.items():
        if pattern.search(text):
            diagnostics.append(
                _diagnostic(
                    f"RATIONALE-{code}-BLOCKED",
                    "blocking",
                    "PROFESSIONAL_BOUNDARY_BLOCKING",
                    "Agent rationale includes unsupported authority language.",
                    "Revise the rationale as decision-support metadata and route acceptance to a human gate.",
                    [_affected("AgentRationale", "professional-boundary")],
                )
            )
    return diagnostics


def _unresolved_operation_assumptions(operation: Mapping[str, Any]) -> list[dict[str, Any]]:
    assumptions = []
    for item in _list(operation.get("assumptions")):
        candidate = _assumption(item)
        if _assumption_is_unresolved(candidate):
            assumptions.append(candidate)
    if isinstance(operation.get("preconditions"), Mapping):
        for item in _list(operation["preconditions"].get("assumptions")):
            assumptions.append(_assumption(item))
    return sorted(assumptions, key=canonical_json)


def _assumption(item: Any) -> dict[str, Any]:
    if isinstance(item, Mapping):
        result = deepcopy(dict(item))
        result.setdefault("status", "TBD")
        return _stable_mapping(result)
    return {"assumption": str(item), "status": "TBD"}


def _assumption_is_unresolved(item: Mapping[str, Any]) -> bool:
    status = str(item.get("status", item.get("resolution_status", "TBD"))).lower()
    return status in {"tbd", "unresolved", "pending", "unknown"}


def _operation_set_ref(operation_set: Any) -> dict[str, str]:
    if isinstance(operation_set, Mapping):
        return {"object_type": "ModelOperation", "ref": str(operation_set.get("operation_set_id", "unknown"))}
    return {"object_type": "ModelOperation", "ref": "unknown"}


def _ref(item: Mapping[str, Any]) -> dict[str, str]:
    return {
        "object_type": str(item.get("object_type", "TBD")),
        "ref": str(item.get("ref", "TBD")),
    }


def _diagnostic(
    code: str,
    severity: str,
    diagnostic_class: str,
    message: str,
    remediation: str,
    affected_refs: list[dict[str, str]],
) -> dict[str, Any]:
    return {
        "code": code,
        "severity": severity,
        "class": diagnostic_class,
        "affected_references": affected_refs,
        "message": message,
        "remediation": remediation,
        "provenance": deepcopy(ENGINE_PROVENANCE),
    }


def _tbd(code: str, message: str) -> dict[str, Any]:
    return _diagnostic(
        code,
        "warning",
        "TBD_VISIBLE",
        message,
        "Provide the missing rationale input before relying on the record for reproducible review.",
        [_affected("AgentRationale", "TBD")],
    )


def _affected(object_type: str, ref: str) -> dict[str, str]:
    return {"object_type": object_type, "ref": ref}


def _hash_payload(value: Any) -> str:
    return hashlib.sha256(canonical_json(value).encode("utf-8")).hexdigest()


def _first_text(*values: Any) -> str:
    for value in values:
        if value not in (None, ""):
            return str(value)
    return "TBD"


def _list(value: Any) -> list[Any]:
    return value if isinstance(value, list) else []


def _stable(items: list[dict[str, Any]]) -> list[dict[str, Any]]:
    return sorted(items, key=canonical_json)


def _stable_mapping(value: Mapping[str, Any]) -> dict[str, Any]:
    return {key: deepcopy(value[key]) for key in sorted(value)}
