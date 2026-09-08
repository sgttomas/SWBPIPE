#!/usr/bin/env python3
"""K8 V4 total writer/reader validator with schema-first type dispatch."""

from __future__ import annotations

import math
from collections.abc import Iterable
from typing import Any

from jsonschema import Draft202012Validator

import semantic_validator_v2 as v2


ValidationError = v2.ValidationError
StrictJsonError = v2.StrictJsonError
parse_strict_json = v2.parse_strict_json
serialize_strict_json = v2.serialize_strict_json
interpretation_mode = v2.interpretation_mode


_STRUCTURAL_KEYWORDS = {
    "additionalProperties",
    "dependentRequired",
    "maxItems",
    "maxProperties",
    "minItems",
    "minProperties",
    "required",
    "type",
}


def _pointer(parts: Iterable[Any]) -> str:
    encoded = [str(part).replace("~", "~0").replace("/", "~1") for part in parts]
    return "/" + "/".join(encoded) if encoded else ""


def _strict_host_error(value: Any) -> ValidationError | None:
    """Prove that a host value is finite, acyclic strict JSON without recursion."""
    active: set[int] = set()
    stack: list[tuple[bool, Any, str]] = [(False, value, "")]
    while stack:
        leaving, item, pointer = stack.pop()
        if leaving:
            active.remove(id(item))
            continue

        if item is None or isinstance(item, (str, bool, int)):
            continue
        if isinstance(item, float):
            if not math.isfinite(item):
                return ValidationError(
                    "RESULT_EXPORT_NON_FINITE_NUMBER", pointer or "/", None,
                    "all numeric values must be finite before schema validation or serialization",
                )
            continue
        if isinstance(item, (dict, list)):
            identity = id(item)
            if identity in active:
                return ValidationError(
                    "RESULT_EXPORT_SCHEMA_INVALID", pointer or "/", None,
                    "cyclic host containers are not representable as strict JSON",
                )
            active.add(identity)
            stack.append((True, item, pointer))
            if isinstance(item, dict):
                children: list[tuple[bool, Any, str]] = []
                for key, child in item.items():
                    if not isinstance(key, str):
                        return ValidationError(
                            "RESULT_EXPORT_SCHEMA_INVALID", pointer or "/", None,
                            "strict JSON object keys must be strings",
                        )
                    escaped = key.replace("~", "~0").replace("/", "~1")
                    children.append((False, child, f"{pointer}/{escaped}"))
                stack.extend(reversed(children))
            else:
                stack.extend(
                    (False, child, f"{pointer}/{index}")
                    for index, child in reversed(list(enumerate(item)))
                )
            continue
        return ValidationError(
            "RESULT_EXPORT_SCHEMA_INVALID", pointer or "/", None,
            f"host value of type {type(item).__name__} is not representable as strict JSON",
        )
    return None


def _json_type(value: Any) -> str:
    if value is None:
        return "null"
    if isinstance(value, bool):
        return "boolean"
    if isinstance(value, (int, float)):
        return "number"
    if isinstance(value, str):
        return "string"
    if isinstance(value, list):
        return "array"
    return "object"


def _direct_structural_failure(error: Any) -> bool:
    if error.validator in _STRUCTURAL_KEYWORDS:
        return True
    if error.validator == "const":
        return _json_type(error.instance) != _json_type(error.validator_value)
    if error.validator == "enum":
        allowed_types = {_json_type(candidate) for candidate in error.validator_value}
        return _json_type(error.instance) not in allowed_types
    return False


def _structural_leaves(error: Any):
    """Yield structural/type leaves from a complete Draft 2020-12 error tree."""
    if _direct_structural_failure(error):
        yield error
    for child in error.context:
        yield from _structural_leaves(child)


def _as_schema_error(error: Any) -> ValidationError:
    return ValidationError(
        "RESULT_EXPORT_SCHEMA_INVALID",
        _pointer(error.absolute_path),
        None,
        error.message,
    )


def _structural_errors(schema_errors: list[Any]) -> list[ValidationError]:
    unique: dict[tuple[str, str], ValidationError] = {}
    for root_error in schema_errors:
        for leaf in _structural_leaves(root_error):
            converted = _as_schema_error(leaf)
            unique[(converted.json_pointer, converted.message)] = converted
    return [unique[key] for key in sorted(unique)]


def normalize_wire_version(
    raw_request: Any,
    field: str,
    supported_versions: tuple[str, ...],
) -> tuple[str | None, ValidationError | None]:
    """Normalize an optional wire field after host and structural validation."""
    host_error = _strict_host_error(raw_request)
    if host_error:
        return None, host_error
    ingress_schema = {
        "type": "object",
        "properties": {field: {"type": ["string", "null"]}},
    }
    schema_errors = list(Draft202012Validator(ingress_schema).iter_errors(raw_request))
    structural = _structural_errors(schema_errors)
    if structural:
        return None, structural[0]

    requested = raw_request.get(field)
    if requested is None or not requested.strip():
        return None, ValidationError(
            "RESULT_EXPORT_SCHEMA_VERSION_REQUIRED", f"/{field}", None,
            f"{field} is required and must be non-empty",
        )
    if requested not in supported_versions:
        return None, ValidationError(
            "RESULT_EXPORT_SCHEMA_VERSION_UNSUPPORTED", f"/{field}", None,
            f"requested version {requested!r} is not supported",
        )
    return requested, None


def validate_document(
    document: Any,
    schema: dict[str, Any],
    mapping: dict[tuple[str, str], dict[str, str]],
) -> list[ValidationError]:
    """Validate every strict-JSON host value without hiding programming defects."""
    host_error = _strict_host_error(document)
    if host_error:
        return [host_error]

    # Run the complete supplied schema once. Structural/type failures, including
    # nested allOf/oneOf contexts, gate every later lookup and semantic relation.
    schema_errors = sorted(
        Draft202012Validator(schema).iter_errors(document),
        key=lambda item: tuple(str(part) for part in item.absolute_path),
    )
    structural = _structural_errors(schema_errors)
    if structural:
        return structural

    envelope = document["result_envelope"]
    if document["schema_version"] != v2.VERSION or envelope["schema_version"] != v2.VERSION:
        return [ValidationError(
            "RESULT_EXPORT_SCHEMA_VERSION_UNSUPPORTED", "/schema_version", None,
            "outer and inner result schema versions must both be supported 0.2.0",
        )]

    relation_errors = v2.validate_semantics(document, mapping)
    if relation_errors:
        return relation_errors
    return [_as_schema_error(error) for error in schema_errors]


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
    if not isinstance(payload, (bytes, str)):
        return None, [ValidationError(
            "RESULT_EXPORT_SCHEMA_INVALID", "", None,
            "serialized input must be bytes or a string",
        )]
    try:
        document = parse_strict_json(payload)
    except StrictJsonError as error:
        return None, [error.error]
    errors = validate_document(document, schema, mapping)
    return (document, []) if not errors else (None, errors)
