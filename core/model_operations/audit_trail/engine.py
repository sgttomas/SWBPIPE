"""Deterministic operation acceptance audit records for DEL-16-03.

The audit trail records user acceptance or rejection decisions over structured
model operations, validation outcomes, and diff-preview references. It does not
apply operations to model state.
"""

from __future__ import annotations

from copy import deepcopy
import hashlib
import json
from typing import Any, Mapping


AUDIT_TRAIL_VERSION = "0.1.0"

PROFESSIONAL_BOUNDARY = {
    "human_review_required": True,
    "software_makes_compliance_claim": False,
    "software_makes_certification_claim": False,
    "software_makes_sealing_claim": False,
    "software_makes_approval_claim": False,
    "software_makes_authentication_claim": False,
}

ENGINE_PROVENANCE = {
    "source_name": "OpenPipeStress DEL-16-03 operation audit trail",
    "source_location": "core/model_operations/audit_trail/engine.py",
    "source_license": "project-governed",
    "contributor": "OpenPipeStress Type 2 worker",
    "contributor_certification": "implementation-only-no-professional-claim",
    "redistribution_status": "public_permissive",
    "review_status": "pending",
    "privacy_classification": "public_metadata",
}


def record_operation_audit_trail(
    operation_envelope: Mapping[str, Any],
    *,
    validation_outcome: Mapping[str, Any] | None = None,
    diff_preview_ref: Mapping[str, Any] | None = None,
    acceptance_signal: Mapping[str, Any] | None = None,
    actor: Mapping[str, Any] | None = None,
    source: Mapping[str, Any] | None = None,
    timestamp: str | None = None,
    rationale: str | None = None,
    audit_metadata: Mapping[str, Any] | None = None,
    accepted_model_state: Mapping[str, Any] | None = None,
) -> dict[str, Any]:
    """Create deterministic audit records without mutating accepted state."""

    source_state = deepcopy(dict(accepted_model_state)) if isinstance(accepted_model_state, Mapping) else None
    operation_set = operation_envelope.get("operation_set", {}) if isinstance(operation_envelope, Mapping) else {}
    operations = _list(operation_set.get("operations")) if isinstance(operation_set, Mapping) else []
    diagnostics = _input_diagnostics(
        operation_envelope,
        validation_outcome,
        diff_preview_ref,
        acceptance_signal,
        timestamp,
        rationale,
    )

    records = []
    for index, operation in enumerate(operations):
        if isinstance(operation, Mapping):
            records.append(
                _record_for_operation(
                    operation,
                    index,
                    operation_set,
                    validation_outcome,
                    diff_preview_ref,
                    acceptance_signal,
                    actor,
                    source,
                    timestamp,
                    rationale,
                    audit_metadata,
                    source_state,
                )
            )
        else:
            diagnostics.append(
                _diagnostic(
                    "AUDIT-OPERATION-NOT-OBJECT",
                    "blocking",
                    "OPERATION_AUDIT_BLOCKING",
                    "Operation history entry is not a structured object.",
                    "Provide DEL-16-01 operation records before audit recording.",
                    [_affected("ModelOperation", f"index:{index}")],
                )
            )

    decision_counts = _decision_counts(records)
    trail = {
        "schema_version": AUDIT_TRAIL_VERSION,
        "deliverable_id": "DEL-16-03",
        "package_id": "PKG-16",
        "scope_item": "SOW-070",
        "objectives": ["OBJ-015"],
        "operation_set_ref": _operation_set_ref(operation_set),
        "accepted_model_state_ref": _state_ref(source_state),
        "audit_posture": {
            "default_user_acceptance_required": True,
            "accepted_records_require_explicit_user_signal": True,
            "rejected_records_mutate_accepted_state": False,
        },
        "decision_counts": decision_counts,
        "records": sorted(records, key=canonical_json),
        "diagnostics": _stable(diagnostics),
        "accepted_model_state_unchanged": (
            True if source_state is None else source_state == accepted_model_state
        ),
        "professional_boundary": deepcopy(PROFESSIONAL_BOUNDARY),
        "provenance": deepcopy(ENGINE_PROVENANCE),
    }
    trail["audit_trail_hash"] = _hash_payload(
        {
            "operation_set_ref": trail["operation_set_ref"],
            "accepted_model_state_ref": trail["accepted_model_state_ref"],
            "records": trail["records"],
            "diagnostics": trail["diagnostics"],
        }
    )
    return trail


def canonical_json(value: Any) -> str:
    """Serialize audit output with stable key ordering."""

    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=True)


def _record_for_operation(
    operation: Mapping[str, Any],
    index: int,
    operation_set: Mapping[str, Any],
    validation_outcome: Mapping[str, Any] | None,
    diff_preview_ref: Mapping[str, Any] | None,
    acceptance_signal: Mapping[str, Any] | None,
    actor: Mapping[str, Any] | None,
    source: Mapping[str, Any] | None,
    timestamp: str | None,
    rationale: str | None,
    audit_metadata: Mapping[str, Any] | None,
    accepted_model_state: Mapping[str, Any] | None,
) -> dict[str, Any]:
    operation_id = str(operation.get("operation_id", f"operation:{index:04d}"))
    signal_decision = _signal_decision(acceptance_signal)
    explicit_user_acceptance = _explicit_user_acceptance(acceptance_signal)
    rejected = signal_decision == "reject"
    accepted_candidate = explicit_user_acceptance and signal_decision == "accept"
    record_diagnostics = _record_diagnostics(
        operation_id,
        accepted_candidate,
        rejected,
        operation,
        validation_outcome,
        diff_preview_ref,
        acceptance_signal,
        timestamp,
        rationale,
        accepted_model_state,
    )
    accepted = accepted_candidate and not any(item["severity"] == "blocking" for item in record_diagnostics)
    status = "accepted" if accepted else "rejected" if rejected else "held_for_user_acceptance"
    record = {
        "audit_record_id": _record_id(
            operation_id,
            operation,
            validation_outcome,
            diff_preview_ref,
            acceptance_signal,
            timestamp,
            rationale,
            audit_metadata,
        ),
        "operation_id": operation_id,
        "operation_set_ref": _operation_set_ref(operation_set),
        "decision": {
            "status": status,
            "explicit_user_acceptance": explicit_user_acceptance,
            "acceptance_required": True,
            "accepted_model_state_mutated": False,
            "decided_at": _first_text(
                _mapping_get(acceptance_signal, "decided_at"),
                timestamp,
                _mapping_get(audit_metadata, "recorded_at"),
                "TBD",
            ),
            "rationale": _first_text(_mapping_get(acceptance_signal, "rationale"), rationale, "TBD"),
        },
        "actor": _actor(actor, acceptance_signal),
        "source": _source(source, operation),
        "affected_entities": _affected_entities(operation),
        "operation_history": {
            "operation_kind": operation.get("operation_kind", "TBD"),
            "operation_status": operation.get("operation_status", "TBD"),
            "author_type": operation.get("author_type", "TBD"),
            "target_refs": deepcopy(_list(operation.get("target_refs"))),
            "preconditions": deepcopy(operation.get("preconditions", "TBD")),
            "changes": deepcopy(_list(operation.get("changes"))),
            "validation": deepcopy(operation.get("validation", "TBD")),
            "diagnostics": deepcopy(_list(operation.get("diagnostics"))),
            "diff_preview_refs": deepcopy(_list(operation.get("diff_preview_refs"))),
            "assumptions": deepcopy(_list(operation.get("assumptions"))),
            "provenance": deepcopy(operation.get("provenance", "TBD")),
        },
        "validation_outcome": _validation_outcome(validation_outcome, operation_id),
        "diff_preview_ref": _preview_ref(diff_preview_ref, operation_id),
        "accepted_model_state_ref": _state_ref(accepted_model_state),
        "unresolved_assumptions": _unresolved_assumptions(operation),
        "audit_metadata": _audit_metadata(audit_metadata),
        "diagnostics": _stable(record_diagnostics),
        "professional_boundary": deepcopy(PROFESSIONAL_BOUNDARY),
    }
    record["record_hash"] = _hash_payload(
        {
            "operation_id": record["operation_id"],
            "decision": record["decision"],
            "actor": record["actor"],
            "source": record["source"],
            "affected_entities": record["affected_entities"],
            "operation_history": record["operation_history"],
            "validation_outcome": record["validation_outcome"],
            "diff_preview_ref": record["diff_preview_ref"],
            "accepted_model_state_ref": record["accepted_model_state_ref"],
            "unresolved_assumptions": record["unresolved_assumptions"],
            "audit_metadata": record["audit_metadata"],
            "diagnostics": record["diagnostics"],
        }
    )
    return record


def _input_diagnostics(
    operation_envelope: Mapping[str, Any],
    validation_outcome: Mapping[str, Any] | None,
    diff_preview_ref: Mapping[str, Any] | None,
    acceptance_signal: Mapping[str, Any] | None,
    timestamp: str | None,
    rationale: str | None,
) -> list[dict[str, Any]]:
    diagnostics = []
    if not isinstance(operation_envelope, Mapping):
        diagnostics.append(
            _diagnostic(
                "AUDIT-ENVELOPE-NOT-OBJECT",
                "blocking",
                "OPERATION_AUDIT_BLOCKING",
                "Audit input must be a structured operation envelope.",
                "Provide a DEL-16-01 operation envelope.",
                [_affected("ModelOperationEnvelope", "missing")],
            )
        )
    if validation_outcome is None:
        diagnostics.append(_tbd("AUDIT-VALIDATION-OUTCOME-TBD", "Validation outcome is missing."))
    if diff_preview_ref is None:
        diagnostics.append(_tbd("AUDIT-DIFF-PREVIEW-REF-TBD", "Diff-preview reference is missing."))
    if acceptance_signal is None:
        diagnostics.append(_tbd("AUDIT-USER-DECISION-TBD", "Explicit user acceptance or rejection signal is missing."))
    if not timestamp and not _mapping_get(acceptance_signal, "decided_at"):
        diagnostics.append(_tbd("AUDIT-TIMESTAMP-TBD", "Decision timestamp is missing."))
    if not rationale and not _mapping_get(acceptance_signal, "rationale"):
        diagnostics.append(_tbd("AUDIT-RATIONALE-TBD", "Decision rationale is missing."))
    return diagnostics


def _record_diagnostics(
    operation_id: str,
    accepted_candidate: bool,
    rejected: bool,
    operation: Mapping[str, Any],
    validation_outcome: Mapping[str, Any] | None,
    diff_preview_ref: Mapping[str, Any] | None,
    acceptance_signal: Mapping[str, Any] | None,
    timestamp: str | None,
    rationale: str | None,
    accepted_model_state: Mapping[str, Any] | None,
) -> list[dict[str, Any]]:
    diagnostics = []
    if not accepted_candidate and not rejected:
        diagnostics.append(
            _diagnostic(
                "AUDIT-EXPLICIT-USER-ACCEPTANCE-REQUIRED",
                "blocking",
                "USER_ACCEPTANCE_REQUIRED",
                "Operation cannot be recorded as accepted without an explicit user acceptance signal.",
                "Record a user accept or reject decision before accepted-state application.",
                [_affected("ModelOperation", operation_id)],
            )
        )
    if validation_outcome is None:
        diagnostics.append(_tbd("AUDIT-VALIDATION-OUTCOME-TBD", "Validation outcome is missing."))
    if diff_preview_ref is None:
        diagnostics.append(_tbd("AUDIT-DIFF-PREVIEW-REF-TBD", "Diff-preview reference is missing."))
    if accepted_candidate:
        diagnostics.extend(_acceptance_gate_diagnostics(operation_id, operation, validation_outcome, diff_preview_ref, accepted_model_state))
    if not timestamp and not _mapping_get(acceptance_signal, "decided_at"):
        diagnostics.append(_tbd("AUDIT-TIMESTAMP-TBD", "Decision timestamp is missing."))
    if not rationale and not _mapping_get(acceptance_signal, "rationale"):
        diagnostics.append(_tbd("AUDIT-RATIONALE-TBD", "Decision rationale is missing."))
    return diagnostics


def _acceptance_gate_diagnostics(
    operation_id: str,
    operation: Mapping[str, Any],
    validation_outcome: Mapping[str, Any] | None,
    diff_preview_ref: Mapping[str, Any] | None,
    accepted_model_state: Mapping[str, Any] | None,
) -> list[dict[str, Any]]:
    diagnostics: list[dict[str, Any]] = []
    validation = validation_outcome.get("validation") if isinstance(validation_outcome, Mapping) else None
    required_validation = {
        "schema_validation": "passed",
        "constraint_validation": "passed",
        "unit_validation": "passed",
        "diff_preview_status": "generated",
        "application_status": "not_applied",
    }
    if not isinstance(validation, Mapping) or any(validation.get(key) != value for key, value in required_validation.items()):
        diagnostics.append(
            _diagnostic(
                "AUDIT-VALIDATION-GATES-BLOCKING",
                "blocking",
                "OPERATION_AUDIT_BLOCKING",
                "Accepted operation records require passed schema, constraint, unit, and generated diff-preview evidence.",
                "Re-run DEL-16-02 validation and preview successfully before recording accepted operation status.",
                [_affected("ModelOperation", operation_id)],
            )
        )

    if not isinstance(diff_preview_ref, Mapping) or not diff_preview_ref.get("hash"):
        diagnostics.append(
            _diagnostic(
                "AUDIT-DIFF-PREVIEW-HASH-MISSING",
                "blocking",
                "OPERATION_AUDIT_BLOCKING",
                "Accepted operation records require a hash-bound diff-preview reference.",
                "Provide a diff-preview reference with a current checksum before accepted audit recording.",
                [_affected("ModelOperation", operation_id)],
            )
        )

    state_hash = _state_hash(accepted_model_state)
    if not state_hash:
        diagnostics.append(
            _diagnostic(
                "AUDIT-MODEL-STATE-HASH-MISSING",
                "blocking",
                "OPERATION_AUDIT_BLOCKING",
                "Accepted operation records require a current accepted model-state hash.",
                "Provide accepted_model_state with a current model-state hash before accepted audit recording.",
                [_affected("ModelOperation", operation_id)],
            )
        )
        return diagnostics

    hashes = _list(operation.get("preconditions", {}).get("required_current_hashes")) if isinstance(operation.get("preconditions"), Mapping) else []
    model_state_hashes = [
        item for item in hashes if isinstance(item, Mapping) and item.get("payload_scope") == "model_state_record"
    ]
    if not model_state_hashes:
        diagnostics.append(
            _diagnostic(
                "AUDIT-OPERATION-CURRENT-HASH-MISSING",
                "blocking",
                "OPERATION_AUDIT_BLOCKING",
                "Operation preconditions do not include a model-state current hash.",
                "Bind the operation precondition to the accepted model-state hash before accepted audit recording.",
                [_affected("ModelOperation", operation_id)],
            )
        )
    elif all(str(item.get("value")) != state_hash for item in model_state_hashes):
        diagnostics.append(
            _diagnostic(
                "AUDIT-OPERATION-CURRENT-HASH-MISMATCH",
                "blocking",
                "OPERATION_AUDIT_BLOCKING",
                "Operation precondition hash does not match the current accepted model-state hash.",
                "Refresh the operation and validation preview against the current model-state hash.",
                [_affected("ModelOperation", operation_id)],
            )
        )
    return diagnostics


def _validation_outcome(validation_outcome: Mapping[str, Any] | None, operation_id: str) -> dict[str, Any]:
    if not isinstance(validation_outcome, Mapping):
        return {"status": "TBD", "diagnostics": [_tbd("AUDIT-VALIDATION-OUTCOME-TBD", "Validation outcome is missing.")]}
    result = deepcopy(dict(validation_outcome))
    preview_rows = _list(result.get("diff_preview"))
    matching_rows = [
        deepcopy(dict(item))
        for item in preview_rows
        if isinstance(item, Mapping) and item.get("operation_id") == operation_id
    ]
    if matching_rows:
        result["operation_diff_preview"] = sorted(matching_rows, key=canonical_json)
    return result


def _preview_ref(diff_preview_ref: Mapping[str, Any] | None, operation_id: str) -> dict[str, Any]:
    if not isinstance(diff_preview_ref, Mapping):
        return {"object_type": "DiffPreview", "ref": "TBD", "operation_id": operation_id}
    result = deepcopy(dict(diff_preview_ref))
    result.setdefault("object_type", "DiffPreview")
    result.setdefault("operation_id", operation_id)
    return result


def _actor(actor: Mapping[str, Any] | None, acceptance_signal: Mapping[str, Any] | None) -> dict[str, Any]:
    result = deepcopy(dict(actor)) if isinstance(actor, Mapping) else {}
    result.setdefault("actor_type", _first_text(_mapping_get(acceptance_signal, "actor_type"), "TBD"))
    result.setdefault("actor_ref", _first_text(_mapping_get(acceptance_signal, "actor_ref"), "TBD"))
    result.setdefault("source_role", _first_text(_mapping_get(acceptance_signal, "source_role"), "TBD"))
    return _stable_mapping(result)


def _source(source: Mapping[str, Any] | None, operation: Mapping[str, Any]) -> dict[str, Any]:
    result = deepcopy(dict(source)) if isinstance(source, Mapping) else {}
    result.setdefault("operation_author_type", str(operation.get("author_type", "TBD")))
    result.setdefault("source_ref", "TBD")
    result.setdefault("source_channel", "TBD")
    return _stable_mapping(result)


def _affected_entities(operation: Mapping[str, Any]) -> list[dict[str, str]]:
    refs: list[dict[str, str]] = []
    for item in _list(operation.get("target_refs")):
        if isinstance(item, Mapping):
            refs.append(_ref(item))
    for change in _list(operation.get("changes")):
        if isinstance(change, Mapping):
            target = change.get("target_ref")
            if isinstance(target, Mapping):
                refs.append(_ref(target))
    deduped = {canonical_json(item): item for item in refs}
    return [deduped[key] for key in sorted(deduped)]


def _unresolved_assumptions(operation: Mapping[str, Any]) -> list[dict[str, Any]]:
    assumptions = []
    for item in _list(operation.get("assumptions")):
        if isinstance(item, Mapping):
            status = str(item.get("status", item.get("resolution_status", "TBD"))).lower()
            if status in {"tbd", "unresolved", "pending", "unknown"} or "status" not in item:
                assumptions.append(deepcopy(dict(item)))
        else:
            assumptions.append({"assumption": str(item), "status": "TBD"})
    if isinstance(operation.get("preconditions"), Mapping):
        for item in _list(operation["preconditions"].get("assumptions")):
            if isinstance(item, Mapping):
                assumptions.append(deepcopy(dict(item)))
            else:
                assumptions.append({"assumption": str(item), "status": "TBD"})
    return sorted(assumptions, key=canonical_json)


def _audit_metadata(audit_metadata: Mapping[str, Any] | None) -> dict[str, Any]:
    result = deepcopy(dict(audit_metadata)) if isinstance(audit_metadata, Mapping) else {}
    result.setdefault("recorded_at", "TBD")
    result.setdefault("audit_metadata_status", "provided" if audit_metadata else "TBD")
    return _stable_mapping(result)


def _explicit_user_acceptance(acceptance_signal: Mapping[str, Any] | None) -> bool:
    if not isinstance(acceptance_signal, Mapping):
        return False
    return (
        acceptance_signal.get("accepted") is True
        and _signal_decision(acceptance_signal) == "accept"
        and str(acceptance_signal.get("actor_type", "")).lower() == "user"
    )


def _signal_decision(acceptance_signal: Mapping[str, Any] | None) -> str:
    if not isinstance(acceptance_signal, Mapping):
        return "TBD"
    return str(acceptance_signal.get("decision", "TBD")).lower()


def _decision_counts(records: list[dict[str, Any]]) -> dict[str, int]:
    counts = {"accepted": 0, "rejected": 0, "held_for_user_acceptance": 0}
    for record in records:
        status = record["decision"]["status"]
        if status in counts:
            counts[status] += 1
    return counts


def _record_id(
    operation_id: str,
    operation: Mapping[str, Any],
    validation_outcome: Mapping[str, Any] | None,
    diff_preview_ref: Mapping[str, Any] | None,
    acceptance_signal: Mapping[str, Any] | None,
    timestamp: str | None,
    rationale: str | None,
    audit_metadata: Mapping[str, Any] | None,
) -> str:
    return "audit:" + _hash_payload(
        {
            "operation_id": operation_id,
            "operation": operation,
            "validation_outcome": validation_outcome,
            "diff_preview_ref": diff_preview_ref,
            "acceptance_signal": acceptance_signal,
            "timestamp": timestamp,
            "rationale": rationale,
            "audit_metadata": audit_metadata,
        }
    )[:24]


def _hash_payload(value: Any) -> str:
    return hashlib.sha256(canonical_json(value).encode("utf-8")).hexdigest()


def _operation_set_ref(operation_set: Any) -> dict[str, str]:
    if isinstance(operation_set, Mapping):
        return {"object_type": "ModelOperation", "ref": str(operation_set.get("operation_set_id", "unknown"))}
    return {"object_type": "ModelOperation", "ref": "unknown"}


def _state_ref(model_state: Mapping[str, Any] | None) -> dict[str, str]:
    if not isinstance(model_state, Mapping):
        return {"object_type": "ModelState", "ref": "TBD"}
    state = model_state.get("model_state", model_state)
    if isinstance(state, Mapping):
        result = {"object_type": "ModelState", "ref": str(state.get("state_id", state.get("id", "unknown")))}
        state_hash = _state_hash(model_state)
        if state_hash:
            result["hash"] = state_hash
        return result
    return {"object_type": "ModelState", "ref": "unknown"}


def _state_hash(model_state: Mapping[str, Any] | None) -> str | None:
    if not isinstance(model_state, Mapping):
        return None
    state = model_state.get("model_state", model_state)
    for candidate in (state, model_state):
        if isinstance(candidate, Mapping):
            value = candidate.get("state_hash") or candidate.get("model_state_hash") or candidate.get("hash")
            if value:
                return str(value)
    return None


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
        "Provide the missing audit input before relying on the record for reproducible review.",
        [_affected("OperationAuditTrail", "TBD")],
    )


def _affected(object_type: str, ref: str) -> dict[str, str]:
    return {"object_type": object_type, "ref": ref}


def _mapping_get(value: Mapping[str, Any] | None, key: str) -> Any:
    return value.get(key) if isinstance(value, Mapping) else None


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
