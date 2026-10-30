"""Structured operation validation and preview for DEL-16-02.

The engine validates proposed operation records and produces deterministic
preview deltas against an accepted model-state snapshot. It never mutates the
accepted input state and never treats a preview as an accepted technical result.
"""

from __future__ import annotations

from copy import deepcopy
import json
from pathlib import Path
from typing import Any, Mapping

from core.units.schema_vocabulary import canonical_dimension_ids


VALIDATION_PREVIEW_VERSION = "0.1.0"

SUPPORTED_OPERATION_KINDS = {
    "add",
    "move",
    "modify",
    "delete",
    "reconnect",
    "constraint",
    "load",
    "support",
    "design_knowledge",
}
SUPPORTED_CHANGE_KINDS = {
    "add_object",
    "remove_object",
    "set_field",
    "move_geometry",
    "reconnect",
    "update_constraint",
    "update_load",
    "update_support",
    "attach_design_knowledge",
}
UNIT_VALUE_KINDS = {"quantity"}
CANONICAL_DIMENSIONS = canonical_dimension_ids()

PROFESSIONAL_BOUNDARY = {
    "human_review_required": True,
    "software_makes_compliance_claim": False,
    "software_makes_certification_claim": False,
    "software_makes_sealing_claim": False,
    "software_makes_approval_claim": False,
    "software_makes_authentication_claim": False,
}

ENGINE_PROVENANCE = {
    "source_name": "OpenPipeStress DEL-16-02 operation validation preview",
    "source_location": "core/model_operations/validation_preview/engine.py",
    "source_license": "project-governed",
    "contributor": "OpenPipeStress Type 2 worker",
    "contributor_certification": "implementation-only-no-professional-claim",
    "redistribution_status": "public_permissive",
    "review_status": "pending",
    "privacy_classification": "public_metadata",
}


def validate_and_preview_operations(
    operation_envelope: Mapping[str, Any],
    accepted_model_state: Mapping[str, Any],
    *,
    constraint_diagnostics: list[Mapping[str, Any]] | None = None,
) -> dict[str, Any]:
    """Validate structured operations and return a deterministic preview."""

    source_state = deepcopy(dict(accepted_model_state))
    operation_set = operation_envelope.get("operation_set", {})
    operations = _list(operation_set.get("operations")) if isinstance(operation_set, Mapping) else []
    diagnostics: list[dict[str, Any]] = []
    diagnostics.extend(_validate_envelope(operation_envelope))
    diagnostics.extend(_schema_diagnostics(operation_envelope))
    diagnostics.extend(_model_basis_diagnostics(operation_envelope, source_state))
    diagnostics.extend(_constraint_diagnostics(constraint_diagnostics or []))

    entity_index = _entity_index(source_state)
    preview_rows: list[dict[str, Any]] = []

    for index, operation in enumerate(operations):
        if not isinstance(operation, Mapping):
            diagnostics.append(
                _diagnostic(
                    "OP-RECORD-NOT-OBJECT",
                    "blocking",
                    "OPERATION_SCHEMA_BLOCKING",
                    "Operation entries must be structured objects.",
                    "Provide each operation as a model_operation schema record.",
                    [_affected("ModelOperation", f"index:{index}")],
                )
            )
            continue

        op_ref = str(operation.get("operation_id", f"operation:{index:04d}"))
        op_diagnostics = _operation_diagnostics(operation, op_ref, entity_index)
        diagnostics.extend(op_diagnostics)
        if any(item["severity"] == "blocking" for item in op_diagnostics):
            preview_rows.append(_blocked_preview(op_ref, op_diagnostics))
            continue

        for change_index, change in enumerate(_list(operation.get("changes"))):
            preview_rows.append(_preview_change(op_ref, change_index, change, entity_index))

    has_blocking = any(item["severity"] == "blocking" for item in diagnostics)
    result = {
        "schema_version": VALIDATION_PREVIEW_VERSION,
        "deliverable_id": "DEL-16-02",
        "package_id": "PKG-16",
        "scope_item": "SOW-069",
        "objectives": ["OBJ-015"],
        "operation_set_ref": _operation_set_ref(operation_set),
        "accepted_model_state_ref": _state_ref(source_state),
        "validation": {
            "schema_validation": "blocked" if _has_class(diagnostics, "OPERATION_SCHEMA_BLOCKING") else "passed",
            "constraint_validation": "blocked" if _has_blocking(constraint_diagnostics or []) else "passed",
            "unit_validation": "blocked"
            if _has_code(diagnostics, "OP-UNIT-METADATA-MISSING")
            or _has_code(diagnostics, "OP-UNIT-DIMENSION-UNKNOWN")
            else "passed",
            "diff_preview_status": "blocked_by_validation" if has_blocking else "generated",
            "application_status": "not_applied",
        },
        "diff_preview": sorted(preview_rows, key=canonical_json),
        "diagnostics": _stable(diagnostics),
        "accepted_model_state_unchanged": source_state == accepted_model_state,
        "professional_boundary": deepcopy(PROFESSIONAL_BOUNDARY),
        "provenance": deepcopy(ENGINE_PROVENANCE),
    }
    return result


def canonical_json(value: Any) -> str:
    """Serialize preview output with stable key ordering."""

    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=True)


def _validate_envelope(envelope: Mapping[str, Any]) -> list[dict[str, Any]]:
    diagnostics: list[dict[str, Any]] = []
    if not isinstance(envelope, Mapping):
        return [
            _diagnostic(
                "OP-ENVELOPE-NOT-OBJECT",
                "blocking",
                "OPERATION_SCHEMA_BLOCKING",
                "Operation envelope must be a structured object.",
                "Provide a model_operation schema envelope.",
                [_affected("ModelOperationEnvelope", "missing")],
            )
        ]
    for field in (
        "schema_version",
        "deliverable_id",
        "package_id",
        "scope_item",
        "objectives",
        "operation_set",
    ):
        if field not in envelope:
            diagnostics.append(
                _diagnostic(
                    "OP-ENVELOPE-FIELD-MISSING",
                    "blocking",
                    "OPERATION_SCHEMA_BLOCKING",
                    f"Operation envelope is missing {field}.",
                    "Provide the required model_operation contract field.",
                    [_affected("ModelOperationEnvelope", field)],
                )
            )
    if envelope.get("deliverable_id") != "DEL-16-01":
        diagnostics.append(
            _diagnostic(
                "OP-UPSTREAM-SCHEMA-BINDING",
                "blocking",
                "OPERATION_SCHEMA_BLOCKING",
                "Operation validation consumes DEL-16-01 structured operation records.",
                "Bind operation input to the upstream model_operation schema before preview.",
                [_affected("deliverable_id", str(envelope.get("deliverable_id")))],
            )
        )
    return diagnostics


def _schema_diagnostics(envelope: Mapping[str, Any]) -> list[dict[str, Any]]:
    try:
        from jsonschema import Draft202012Validator
    except ModuleNotFoundError:
        return [
            _diagnostic(
                "OP-SCHEMA-VALIDATOR-MISSING",
                "blocking",
                "OPERATION_SCHEMA_BLOCKING",
                "JSON Schema 2020-12 validator is unavailable.",
                "Install the repository development dependencies before operation preview.",
                [_affected("ModelOperationEnvelope", "schema-validator")],
            )
        ]

    schema_path = Path(__file__).resolve().parents[3] / "schemas" / "model_operation.schema.json"
    with schema_path.open(encoding="utf-8") as schema_file:
        schema = json.load(schema_file)
    Draft202012Validator.check_schema(schema)
    validator = Draft202012Validator(schema)
    diagnostics: list[dict[str, Any]] = []
    for error in sorted(validator.iter_errors(envelope), key=lambda item: list(item.path)):
        diagnostics.append(
            _diagnostic(
                "OP-SCHEMA-VALIDATION-FAILED",
                "blocking",
                "OPERATION_SCHEMA_BLOCKING",
                f"Operation envelope failed DEL-16-01 JSON Schema at {_schema_path(error.path)}: {error.message}",
                "Provide an operation envelope conforming to schemas/model_operation.schema.json.",
                [_affected("ModelOperationEnvelope", _schema_path(error.path))],
            )
        )
    return diagnostics


def _model_basis_diagnostics(
    envelope: Mapping[str, Any],
    accepted_model_state: Mapping[str, Any],
) -> list[dict[str, Any]]:
    diagnostics: list[dict[str, Any]] = []
    operation_set = envelope.get("operation_set", {}) if isinstance(envelope, Mapping) else {}
    model_basis = operation_set.get("model_basis") if isinstance(operation_set, Mapping) else None
    state_hash = _state_hash(accepted_model_state)

    if not isinstance(model_basis, Mapping):
        diagnostics.append(
            _diagnostic(
                "OP-MODEL-BASIS-MISSING",
                "blocking",
                "OPERATION_PRECONDITION_BLOCKING",
                "Operation set is missing physical source-of-truth model-basis evidence.",
                "Bind the operation set to the accepted physical_source_of_truth model and current model-state hash.",
                [_affected("ModelOperation", str(operation_set.get("operation_set_id", "unknown")))],
            )
        )
    elif model_basis.get("canonical_model_role") != "physical_source_of_truth":
        diagnostics.append(
            _diagnostic(
                "OP-MODEL-ROLE-BLOCKED",
                "blocking",
                "OPERATION_PRECONDITION_BLOCKING",
                "Operation set is not bound to the editable physical_source_of_truth model role.",
                "Refresh the operation basis from the canonical physical source-of-truth model.",
                [_affected("ModelOperation", str(operation_set.get("operation_set_id", "unknown")))],
            )
        )

    if _model_role(accepted_model_state) != "physical_source_of_truth":
        diagnostics.append(
            _diagnostic(
                "OP-ACCEPTED-STATE-MODEL-ROLE-BLOCKED",
                "blocking",
                "OPERATION_PRECONDITION_BLOCKING",
                "Accepted model state is not bound to a physical_source_of_truth model.",
                "Provide a canonical accepted model-state snapshot for the editable source model.",
                [_affected("ModelState", _state_ref(accepted_model_state)["ref"])],
            )
        )

    if not state_hash:
        diagnostics.append(
            _diagnostic(
                "OP-MODEL-STATE-HASH-MISSING",
                "blocking",
                "OPERATION_PRECONDITION_BLOCKING",
                "Accepted model state does not expose a current model-state hash.",
                "Provide the current model-state hash before operation preview.",
                [_affected("ModelState", _state_ref(accepted_model_state)["ref"])],
            )
        )

    if isinstance(model_basis, Mapping) and state_hash:
        basis_hash = _checksum_value(model_basis.get("accepted_model_state_hash"))
        if basis_hash != state_hash:
            diagnostics.append(
                _diagnostic(
                    "OP-MODEL-BASIS-HASH-MISMATCH",
                    "blocking",
                    "OPERATION_PRECONDITION_BLOCKING",
                    "Operation model-basis hash does not match the current accepted model state.",
                    "Rebuild the operation set against the current accepted model-state hash.",
                    [_affected("ModelState", _state_ref(accepted_model_state)["ref"])],
                )
            )

    for operation in _list(operation_set.get("operations")) if isinstance(operation_set, Mapping) else []:
        if isinstance(operation, Mapping):
            operation_id = str(operation.get("operation_id", "unknown"))
            preconditions = operation.get("preconditions")
            hashes = _list(preconditions.get("required_current_hashes")) if isinstance(preconditions, Mapping) else []
            model_state_hashes = [
                item for item in hashes if isinstance(item, Mapping) and item.get("payload_scope") == "model_state_record"
            ]
            if not model_state_hashes:
                diagnostics.append(
                    _diagnostic(
                        "OP-CURRENT-HASH-MISSING",
                        "blocking",
                        "OPERATION_PRECONDITION_BLOCKING",
                        "Operation preconditions do not include a required current model-state hash.",
                        "Add a model_state_record checksum to required_current_hashes before preview.",
                        [_affected("ModelOperation", operation_id)],
                    )
                )
            elif state_hash and all(_checksum_value(item) != state_hash for item in model_state_hashes):
                diagnostics.append(
                    _diagnostic(
                        "OP-CURRENT-HASH-MISMATCH",
                        "blocking",
                        "OPERATION_PRECONDITION_BLOCKING",
                        "Operation required current hash does not match the accepted model state.",
                        "Refresh the operation against the current accepted model-state hash.",
                        [_affected("ModelOperation", operation_id)],
                    )
                )
    return diagnostics


def _operation_diagnostics(
    operation: Mapping[str, Any],
    op_ref: str,
    entity_index: Mapping[str, Mapping[str, Any]],
) -> list[dict[str, Any]]:
    diagnostics: list[dict[str, Any]] = []
    for field in (
        "operation_id",
        "operation_kind",
        "operation_status",
        "target_refs",
        "changes",
        "validation",
        "professional_boundary",
    ):
        if field not in operation:
            diagnostics.append(
                _diagnostic(
                    "OP-RECORD-FIELD-MISSING",
                    "blocking",
                    "OPERATION_SCHEMA_BLOCKING",
                    f"Operation record is missing {field}.",
                    "Provide the required operation field before preview.",
                    [_affected("ModelOperation", op_ref)],
                )
            )

    if operation.get("operation_kind") not in SUPPORTED_OPERATION_KINDS:
        diagnostics.append(
            _diagnostic(
                "OP-KIND-UNSUPPORTED",
                "blocking",
                "OPERATION_SCHEMA_BLOCKING",
                "Operation kind is outside the structured operation taxonomy.",
                "Use a supported operation kind or route the proposal to a later extension.",
                [_affected("ModelOperation", op_ref)],
            )
        )

    if _direct_mutation_requested(operation):
        diagnostics.append(
            _diagnostic(
                "OP-DIRECT-MUTATION-BLOCKED",
                "blocking",
                "OPERATION_PRECONDITION_BLOCKING",
                "Operation requests direct accepted-model mutation.",
                "Keep the operation as a structured proposal for validation, preview, and later user review.",
                [_affected("ModelOperation", op_ref)],
            )
        )

    for change_index, change in enumerate(_list(operation.get("changes"))):
        if not isinstance(change, Mapping):
            diagnostics.append(
                _diagnostic(
                    "OP-CHANGE-NOT-OBJECT",
                    "blocking",
                    "OPERATION_SCHEMA_BLOCKING",
                    "Operation change entries must be structured objects.",
                    "Provide each change as a model_operation change record.",
                    [_affected("OperationChange", f"{op_ref}:{change_index}")],
                )
            )
            continue
        diagnostics.extend(_change_diagnostics(change, op_ref, change_index, entity_index))

    return diagnostics


def _change_diagnostics(
    change: Mapping[str, Any],
    op_ref: str,
    change_index: int,
    entity_index: Mapping[str, Mapping[str, Any]],
) -> list[dict[str, Any]]:
    diagnostics: list[dict[str, Any]] = []
    change_ref = str(change.get("change_id", f"{op_ref}:change:{change_index:04d}"))
    if change.get("change_kind") not in SUPPORTED_CHANGE_KINDS:
        diagnostics.append(
            _diagnostic(
                "OP-CHANGE-KIND-UNSUPPORTED",
                "blocking",
                "OPERATION_SCHEMA_BLOCKING",
                "Change kind is outside the structured operation taxonomy.",
                "Use a supported change kind or record the proposal as unsupported.",
                [_affected("OperationChange", change_ref)],
            )
        )

    payload = change.get("value_payload")
    if not isinstance(payload, Mapping):
        diagnostics.append(
            _diagnostic(
                "OP-PAYLOAD-MISSING",
                "blocking",
                "OPERATION_SCHEMA_BLOCKING",
                "Operation change does not include a structured value_payload.",
                "Provide value_payload with scalar, quantity, reference, or structured patch values.",
                [_affected("OperationChange", change_ref)],
            )
        )
    elif payload.get("value_kind") in UNIT_VALUE_KINDS:
        quantities = _list(payload.get("quantity_values"))
        if not quantities or any(not _quantity_has_units(item) for item in quantities):
            diagnostics.append(
                _diagnostic(
                    "OP-UNIT-METADATA-MISSING",
                    "blocking",
                    "UNIT_WARNING",
                    "Quantity operation payloads require value, unit, and dimension metadata.",
                    "Provide complete quantity metadata or hold the operation for user review.",
                    [_affected("OperationChange", change_ref)],
                )
            )
        invalid_dimensions = [
            str(item.get("dimension"))
            for item in quantities
            if isinstance(item, Mapping)
            and item.get("dimension") is not None
            and item.get("dimension") not in CANONICAL_DIMENSIONS
        ]
        if invalid_dimensions:
            diagnostics.append(
                _diagnostic(
                    "OP-UNIT-DIMENSION-UNKNOWN",
                    "blocking",
                    "UNIT_WARNING",
                    "Quantity operation payload includes dimensions outside the accepted PKG-02 vocabulary.",
                    "Use an accepted canonical dimension identifier or hold the operation for unit-contract review.",
                    [_affected("OperationChange", change_ref)],
                )
            )

    if change.get("change_kind") in {"remove_object", "set_field", "move_geometry", "reconnect"}:
        target_id = _target_id(change)
        if target_id and target_id not in entity_index:
            diagnostics.append(
                _diagnostic(
                    "OP-TARGET-REF-UNRESOLVED",
                    "blocking",
                    "OPERATION_PRECONDITION_BLOCKING",
                    "Operation targets an entity absent from the accepted model state.",
                    "Refresh the model-state basis or correct the operation target reference.",
                    [_affected("OperationChange", change_ref)],
                )
            )
    return diagnostics


def _preview_change(
    op_ref: str,
    change_index: int,
    change: Mapping[str, Any],
    entity_index: Mapping[str, Mapping[str, Any]],
) -> dict[str, Any]:
    change_kind = str(change.get("change_kind", "TBD"))
    change_ref = str(change.get("change_id", f"{op_ref}:change:{change_index:04d}"))
    target_id = _target_id(change)
    before = deepcopy(entity_index.get(target_id)) if target_id else None
    after = deepcopy(before)
    status = "previewed"

    if change_kind == "add_object":
        after = _object_from_payload(change)
    elif change_kind == "remove_object":
        after = None
    elif before is not None:
        after = _patched_object(before, change)
    else:
        status = "blocked_by_validation"

    return {
        "operation_id": op_ref,
        "change_id": change_ref,
        "change_kind": change_kind,
        "target_ref": _target_ref(change),
        "preview_status": status,
        "before": before,
        "after": after,
        "application_status": "not_applied",
    }


def _blocked_preview(op_ref: str, diagnostics: list[Mapping[str, Any]]) -> dict[str, Any]:
    return {
        "operation_id": op_ref,
        "change_id": "blocked",
        "change_kind": "blocked_by_validation",
        "target_ref": None,
        "preview_status": "blocked_by_validation",
        "blocking_diagnostic_codes": sorted(
            str(item["code"]) for item in diagnostics if item.get("severity") == "blocking"
        ),
        "before": None,
        "after": None,
        "application_status": "not_applied",
    }


def _entity_index(model_state: Mapping[str, Any]) -> dict[str, Mapping[str, Any]]:
    entities = model_state.get("entities", model_state.get("model_state", {}).get("entities", []))
    indexed: dict[str, Mapping[str, Any]] = {}
    for entity in _list(entities):
        if isinstance(entity, Mapping):
            entity_id = _entity_id(entity)
            if entity_id:
                indexed[entity_id] = deepcopy(dict(entity))
    return indexed


def _object_from_payload(change: Mapping[str, Any]) -> dict[str, Any] | None:
    payload = change.get("value_payload")
    if not isinstance(payload, Mapping):
        return None
    for value in _list(payload.get("structured_values")):
        if isinstance(value, Mapping):
            return deepcopy(dict(value))
    return {
        "stable_id": _target_id(change) or str(change.get("change_id", "new-object")),
        "category": change.get("target_object_type", "TBD"),
        "value_payload": deepcopy(dict(payload)),
    }


def _patched_object(entity: Mapping[str, Any], change: Mapping[str, Any]) -> dict[str, Any]:
    patched = deepcopy(dict(entity))
    payload = change.get("value_payload", {})
    if isinstance(payload, Mapping):
        for item in _list(payload.get("structured_values")):
            if isinstance(item, Mapping):
                patched.update(deepcopy(dict(item)))
        scalars = _list(payload.get("scalar_values"))
        if scalars:
            patched["preview_scalar_values"] = deepcopy(scalars)
        quantities = _list(payload.get("quantity_values"))
        if quantities:
            patched["preview_quantity_values"] = deepcopy(quantities)
    return patched


def _constraint_diagnostics(items: list[Mapping[str, Any]]) -> list[dict[str, Any]]:
    diagnostics: list[dict[str, Any]] = []
    for item in items:
        if item.get("severity") == "blocking":
            diagnostics.append(
                _diagnostic(
                    "OP-CONSTRAINT-BLOCKING",
                    "blocking",
                    "CONSTRAINT_VALIDATION_BLOCKING",
                    "Constraint validation reported a blocking finding.",
                    "Resolve the constraint finding before operation application.",
                    [_affected("Diagnostic", str(item.get("code", "constraint:blocking")))],
                )
            )
    return diagnostics


def _target_id(change: Mapping[str, Any]) -> str | None:
    target = _target_ref(change)
    if isinstance(target, Mapping):
        return str(target.get("ref")) if target.get("ref") else None
    return None


def _target_ref(change: Mapping[str, Any]) -> Mapping[str, Any] | None:
    if isinstance(change.get("target_ref"), Mapping):
        return deepcopy(dict(change["target_ref"]))
    refs = change.get("target_refs")
    if isinstance(refs, list) and refs and isinstance(refs[0], Mapping):
        return deepcopy(dict(refs[0]))
    payload = change.get("value_payload")
    if isinstance(payload, Mapping):
        refs = payload.get("reference_values")
        if isinstance(refs, list) and refs and isinstance(refs[0], Mapping):
            return deepcopy(dict(refs[0]))
    return None


def _entity_id(entity: Mapping[str, Any]) -> str | None:
    for key in ("stable_id", "entity_id", "id", "ref"):
        if entity.get(key):
            return str(entity[key])
    ref = entity.get("reference")
    if isinstance(ref, Mapping) and ref.get("ref"):
        return str(ref["ref"])
    return None


def _quantity_has_units(item: Any) -> bool:
    return isinstance(item, Mapping) and all(key in item for key in ("value", "unit", "dimension"))


def _direct_mutation_requested(operation: Mapping[str, Any]) -> bool:
    validation = operation.get("validation")
    if isinstance(validation, Mapping) and validation.get("application_status") not in {
        "not_applied",
        "held_for_user_review",
        "downstream_application_required",
        "TBD",
    }:
        return True
    return operation.get("operation_status") in {
        "agent_accepted_engineering_state",
        "auto_approved",
    }


def _operation_set_ref(operation_set: Any) -> dict[str, str]:
    if isinstance(operation_set, Mapping):
        return {"object_type": "ModelOperation", "ref": str(operation_set.get("operation_set_id", "unknown"))}
    return {"object_type": "ModelOperation", "ref": "unknown"}


def _state_ref(model_state: Mapping[str, Any]) -> dict[str, str]:
    state = model_state.get("model_state", model_state)
    if isinstance(state, Mapping):
        result = {"object_type": "ModelState", "ref": str(state.get("state_id", state.get("id", "unknown")))}
        state_hash = _state_hash(model_state)
        if state_hash:
            result["hash"] = state_hash
        return result
    return {"object_type": "ModelState", "ref": "unknown"}


def _state_hash(model_state: Mapping[str, Any]) -> str | None:
    state = model_state.get("model_state", model_state)
    for candidate in (state, model_state):
        if isinstance(candidate, Mapping):
            value = candidate.get("state_hash") or candidate.get("model_state_hash") or candidate.get("hash")
            if value:
                return str(value)
    return None


def _model_role(model_state: Mapping[str, Any]) -> str | None:
    model = model_state.get("model")
    state = model_state.get("model_state")
    for candidate in (model, state, model_state):
        if isinstance(candidate, Mapping) and candidate.get("model_role"):
            return str(candidate["model_role"])
    return None


def _checksum_value(value: Any) -> str | None:
    if isinstance(value, Mapping) and value.get("value"):
        return str(value["value"])
    return None


def _schema_path(parts: Any) -> str:
    path = "$"
    for part in parts:
        path += f"[{part}]" if isinstance(part, int) else f".{part}"
    return path


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


def _affected(object_type: str, ref: str) -> dict[str, str]:
    return {"object_type": object_type, "ref": ref}


def _has_blocking(items: list[Mapping[str, Any]]) -> bool:
    return any(item.get("severity") == "blocking" for item in items)


def _has_code(items: list[Mapping[str, Any]], code: str) -> bool:
    return any(item.get("code") == code for item in items)


def _has_class(items: list[Mapping[str, Any]], diagnostic_class: str) -> bool:
    return any(item.get("class") == diagnostic_class and item.get("severity") == "blocking" for item in items)


def _list(value: Any) -> list[Any]:
    return value if isinstance(value, list) else []


def _stable(diagnostics: list[dict[str, Any]]) -> list[dict[str, Any]]:
    return sorted(diagnostics, key=canonical_json)
