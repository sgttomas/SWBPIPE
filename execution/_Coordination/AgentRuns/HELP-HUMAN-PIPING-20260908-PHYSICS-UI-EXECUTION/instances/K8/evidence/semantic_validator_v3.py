#!/usr/bin/env python3
"""Total K8 V3 reference validator and raw-wire version normalizer."""

from __future__ import annotations

import math
from typing import Any

from jsonschema import Draft202012Validator

import semantic_validator_v2 as v2


ValidationError = v2.ValidationError
StrictJsonError = v2.StrictJsonError
parse_strict_json = v2.parse_strict_json
serialize_strict_json = v2.serialize_strict_json
interpretation_mode = v2.interpretation_mode


def normalize_wire_version(
    raw_request: Any,
    field: str,
    supported_versions: tuple[str, ...],
) -> tuple[str | None, ValidationError | None]:
    """Normalize a raw/optional wire field before a typed DTO is constructed."""
    if not isinstance(raw_request, dict):
        return None, ValidationError(
            "RESULT_EXPORT_SCHEMA_INVALID", "", None,
            "serialized request must be a JSON object",
        )
    requested = raw_request.get(field)
    if requested is None or (isinstance(requested, str) and not requested.strip()):
        return None, ValidationError(
            "RESULT_EXPORT_SCHEMA_VERSION_REQUIRED", f"/{field}", None,
            f"{field} is required and must be non-empty",
        )
    if not isinstance(requested, str):
        return None, ValidationError(
            "RESULT_EXPORT_SCHEMA_INVALID", f"/{field}", None,
            f"{field} must be a string",
        )
    if requested not in supported_versions:
        return None, ValidationError(
            "RESULT_EXPORT_SCHEMA_VERSION_UNSUPPORTED", f"/{field}", None,
            f"requested version {requested!r} is not supported",
        )
    return requested, None


def _shape_error(pointer: str, message: str) -> list[ValidationError]:
    return [ValidationError("RESULT_EXPORT_SCHEMA_INVALID", pointer, None, message)]


def _guard_relation_shapes(document: Any) -> list[ValidationError]:
    if not isinstance(document, dict):
        return _shape_error("", "result export document must be a JSON object")
    envelope = document.get("result_envelope")
    if not isinstance(envelope, dict):
        return _shape_error("/result_envelope", "result_envelope must be an object")
    result_sets = envelope.get("result_sets")
    if not isinstance(result_sets, list):
        return _shape_error("/result_envelope/result_sets", "result_sets must be an array")
    for set_index, result_set in enumerate(result_sets):
        set_pointer = f"/result_envelope/result_sets/{set_index}"
        if not isinstance(result_set, dict):
            return _shape_error(set_pointer, "result set must be an object")
        values = result_set.get("values")
        if not isinstance(values, list):
            return _shape_error(f"{set_pointer}/values", "values must be an array")
        for row_index, row in enumerate(values):
            row_pointer = f"{set_pointer}/values/{row_index}"
            if not isinstance(row, dict):
                return _shape_error(row_pointer, "quantity result must be an object")
            for name in ("source_record", "classification", "object_ref"):
                if not isinstance(row.get(name), dict):
                    return _shape_error(f"{row_pointer}/{name}", f"{name} must be an object")
    return []


def validate_document(
    document: Any,
    schema: dict[str, Any],
    mapping: dict[tuple[str, str], dict[str, str]],
) -> list[ValidationError]:
    """Return stable errors for every host value; never raise for bad shape."""
    shape_errors = _guard_relation_shapes(document)
    if shape_errors:
        return shape_errors

    envelope = document["result_envelope"]
    if document.get("schema_version") != v2.VERSION or envelope.get("schema_version") != v2.VERSION:
        return [ValidationError(
            "RESULT_EXPORT_SCHEMA_VERSION_UNSUPPORTED", "/schema_version", None,
            "outer and inner result schema versions must both be supported 0.2.0",
        )]

    for pointer, number in v2._walk_numbers(document):
        if isinstance(number, float) and not math.isfinite(number):
            return [ValidationError(
                "RESULT_EXPORT_NON_FINITE_NUMBER", pointer, None,
                "all numeric values must be finite before schema validation or serialization",
            )]

    # Relation validation remains before the general schema pass so well-shaped
    # mirror/owner failures retain their stable, specific codes.
    relation_errors = v2.validate_semantics(document, mapping)
    if relation_errors:
        return relation_errors
    schema_errors = sorted(
        Draft202012Validator(schema).iter_errors(document),
        key=lambda item: tuple(str(part) for part in item.absolute_path),
    )
    return [ValidationError(
        "RESULT_EXPORT_SCHEMA_INVALID",
        "/" + "/".join(str(part) for part in error.absolute_path),
        None,
        error.message,
    ) for error in schema_errors]


def validate_writer(document: Any, schema: dict[str, Any], mapping):
    errors = validate_document(document, schema, mapping)
    if errors:
        return None, errors
    try:
        payload = serialize_strict_json(document)
        parsed = parse_strict_json(payload)
    except StrictJsonError as error:
        return None, [error.error]
    errors = validate_document(parsed, schema, mapping)
    return (payload, []) if not errors else (None, errors)


def validate_reader(payload: bytes | str, schema: dict[str, Any], mapping):
    try:
        document = parse_strict_json(payload)
    except StrictJsonError as error:
        return None, [error.error]
    errors = validate_document(document, schema, mapping)
    return (document, []) if not errors else (None, errors)
