#!/usr/bin/env python3
"""Executable reference contract for candidate result-export 0.2.0 validation.

This is design evidence, not production code. JSON Schema validates structure;
this validator closes uniqueness, cross-field equality, finite-number, and
row-level interpretation invariants that Draft 2020-12 cannot express.
"""

from __future__ import annotations

import json
import math
from dataclasses import dataclass, asdict
from typing import Any, Callable

from jsonschema import Draft202012Validator


VERSION = "0.2.0"
STANDARDIZED_STATUSES = {
    "existing_canonical_category",
    "source_specific_metadata_standardized",
}


@dataclass(frozen=True)
class ValidationError:
    code: str
    json_pointer: str
    result_id: str | None
    message: str

    def to_dict(self) -> dict[str, Any]:
        return asdict(self)


class StrictJsonError(ValueError):
    def __init__(self, error: ValidationError):
        super().__init__(error.message)
        self.error = error


def _reject_constant(token: str) -> None:
    raise StrictJsonError(ValidationError(
        "RESULT_EXPORT_NON_FINITE_NUMBER", "", None,
        f"non-JSON numeric token {token!r} is forbidden",
    ))


def _reject_duplicate_members(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    value: dict[str, Any] = {}
    for key, item in pairs:
        if key in value:
            raise StrictJsonError(ValidationError(
                "RESULT_EXPORT_JSON_DUPLICATE_MEMBER", "", None,
                f"duplicate JSON object member {key!r} is forbidden",
            ))
        value[key] = item
    return value


def parse_strict_json(payload: bytes | str) -> Any:
    try:
        return json.loads(
            payload,
            parse_constant=_reject_constant,
            object_pairs_hook=_reject_duplicate_members,
        )
    except StrictJsonError:
        raise
    except (UnicodeDecodeError, json.JSONDecodeError) as error:
        raise StrictJsonError(ValidationError(
            "RESULT_EXPORT_JSON_INVALID", "", None, str(error)
        )) from error


def serialize_strict_json(document: Any) -> bytes:
    try:
        return (json.dumps(
            document, ensure_ascii=False, separators=(",", ":"), allow_nan=False
        ) + "\n").encode("utf-8")
    except (TypeError, ValueError) as error:
        raise StrictJsonError(ValidationError(
            "RESULT_EXPORT_NON_FINITE_NUMBER", "", None,
            f"document is not strict finite JSON: {error}",
        )) from error


def _walk_numbers(value: Any, pointer: str = ""):
    if isinstance(value, bool):
        return
    if isinstance(value, (int, float)):
        yield pointer or "/", value
    elif isinstance(value, dict):
        for key, item in value.items():
            escaped = key.replace("~", "~0").replace("/", "~1")
            yield from _walk_numbers(item, f"{pointer}/{escaped}")
    elif isinstance(value, list):
        for index, item in enumerate(value):
            yield from _walk_numbers(item, f"{pointer}/{index}")


def interpretation_mode(row: dict[str, Any]) -> str:
    """Row status controls interpretation; enclosing set_type is irrelevant."""
    status = row.get("classification", {}).get("semantic_status")
    return "standardized" if status in STANDARDIZED_STATUSES else "preserve_only"


def _err(code: str, pointer: str, row: dict[str, Any], message: str) -> ValidationError:
    result_id = row.get("result_id") if isinstance(row, dict) else None
    return ValidationError(code, pointer, result_id, message)


def validate_semantics(
    document: dict[str, Any],
    mapping: dict[tuple[str, str], dict[str, str]],
) -> list[ValidationError]:
    errors: list[ValidationError] = []
    for pointer, number in _walk_numbers(document):
        if isinstance(number, float) and not math.isfinite(number):
            errors.append(ValidationError(
                "RESULT_EXPORT_NON_FINITE_NUMBER", pointer, None,
                "all numeric values must be finite before schema validation or serialization",
            ))

    envelope = document.get("result_envelope", {}) if isinstance(document, dict) else {}
    if document.get("schema_version") != VERSION or envelope.get("schema_version") != VERSION:
        errors.append(ValidationError(
            "RESULT_EXPORT_SCHEMA_VERSION_UNSUPPORTED", "/schema_version", None,
            "outer and inner result schema versions must both be supported 0.2.0",
        ))

    seen: set[str] = set()
    sets = envelope.get("result_sets", []) if isinstance(envelope, dict) else []
    for set_index, result_set in enumerate(sets if isinstance(sets, list) else []):
        set_basis = result_set.get("basis_ref") if isinstance(result_set, dict) else None
        values = result_set.get("values", []) if isinstance(result_set, dict) else []
        for row_index, row in enumerate(values if isinstance(values, list) else []):
            base = f"/result_envelope/result_sets/{set_index}/values/{row_index}"
            if not isinstance(row, dict) or not isinstance(row.get("source_record"), dict):
                continue
            source = row["source_record"]
            result_id = row.get("result_id")
            if isinstance(result_id, str):
                if result_id in seen:
                    errors.append(_err(
                        "RESULT_EXPORT_DUPLICATE_RESULT_ID", f"{base}/result_id", row,
                        "result_id must be unique across every result set in the document",
                    ))
                seen.add(result_id)
            if result_id != source.get("id"):
                errors.append(_err(
                    "RESULT_EXPORT_SOURCE_RECORD_ID_MISMATCH", base, row,
                    "result_id must equal source_record.id",
                ))
            if row.get("magnitude") != source.get("value"):
                errors.append(_err(
                    "RESULT_EXPORT_SOURCE_RECORD_VALUE_MISMATCH", base, row,
                    "magnitude must equal source_record.value",
                ))
            if row.get("unit") != source.get("unit"):
                errors.append(_err(
                    "RESULT_EXPORT_SOURCE_RECORD_UNIT_MISMATCH", base, row,
                    "unit must equal source_record.unit",
                ))
            row_has_metadata = "metadata" in row
            source_has_metadata = "metadata" in source
            if row_has_metadata != source_has_metadata or (
                row_has_metadata and row.get("metadata") != source.get("metadata")
            ):
                errors.append(_err(
                    "RESULT_EXPORT_SOURCE_RECORD_METADATA_MISMATCH", base, row,
                    "metadata presence and JSON value must equal source_record.metadata",
                ))

            object_ref = row.get("object_ref", {})
            expected_basis = source.get("basis_ref", set_basis)
            row_has_refs = "source_result_refs" in row
            source_has_refs = "source_result_refs" in source
            reference_mismatch = (
                not isinstance(object_ref, dict)
                or object_ref.get("ref_type") != "preview_entity"
                or object_ref.get("ref_id") != source.get("entity_ref")
                or row.get("basis_ref") != expected_basis
                or row_has_refs != source_has_refs
                or (row_has_refs and row.get("source_result_refs") != source.get("source_result_refs"))
            )
            if reference_mismatch:
                errors.append(_err(
                    "RESULT_EXPORT_SOURCE_RECORD_REFERENCE_MISMATCH", base, row,
                    "object, basis, and ordered source-result reference projections must follow the 0.2.0 mapping rule",
                ))

            pair = (source.get("kind"), source.get("unit"))
            expected = mapping.get(pair)
            if expected is None:
                errors.append(_err(
                    "RESULT_EXPORT_NATIVE_KIND_UNIT_UNSUPPORTED", base, row,
                    f"native kind/unit pair {pair!r} is not registered",
                ))
                continue
            classification = row.get("classification", {})
            if classification.get("owner_semantics") != expected["owner_semantics"]:
                errors.append(_err(
                    "RESULT_EXPORT_CLASSIFICATION_OWNER_MISMATCH", base, row,
                    "classification.owner_semantics must equal the registered pair owner",
                ))
            for key in ("source_family", "semantic_status", "canonical_family", "canonical_dimension"):
                if classification.get(key) != expected[key]:
                    errors.append(_err(
                        "RESULT_EXPORT_CLASSIFICATION_MISMATCH", base, row,
                        f"classification.{key} must equal the registered pair mapping",
                    ))
                    break
            if row.get("family") != expected["canonical_family"] or row.get("dimension") != expected["canonical_dimension"]:
                errors.append(_err(
                    "RESULT_EXPORT_CLASSIFICATION_MISMATCH", base, row,
                    "top-level family/dimension must equal the registered pair mapping",
                ))
    return errors


def validate_document(
    document: dict[str, Any],
    schema: dict[str, Any],
    mapping: dict[tuple[str, str], dict[str, str]],
) -> list[ValidationError]:
    """Mandatory in-memory writer/reader validator after version dispatch."""
    errors = validate_semantics(document, mapping)
    if errors:
        return errors
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


def validate_writer(
    document: dict[str, Any],
    schema: dict[str, Any],
    mapping: dict[tuple[str, str], dict[str, str]],
) -> tuple[bytes | None, list[ValidationError]]:
    errors = validate_document(document, schema, mapping)
    if errors:
        return None, errors
    try:
        payload = serialize_strict_json(document)
        parsed = parse_strict_json(payload)
    except StrictJsonError as error:
        return None, [error.error]
    read_errors = validate_document(parsed, schema, mapping)
    return (payload, []) if not read_errors else (None, read_errors)


def validate_reader(
    payload: bytes | str,
    schema: dict[str, Any],
    mapping: dict[tuple[str, str], dict[str, str]],
) -> tuple[dict[str, Any] | None, list[ValidationError]]:
    try:
        document = parse_strict_json(payload)
    except StrictJsonError as error:
        return None, [error.error]
    errors = validate_document(document, schema, mapping)
    return (document, []) if not errors else (None, errors)
