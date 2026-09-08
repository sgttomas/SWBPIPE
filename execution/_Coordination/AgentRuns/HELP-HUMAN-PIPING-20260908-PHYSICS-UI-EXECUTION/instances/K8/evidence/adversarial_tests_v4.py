#!/usr/bin/env python3
"""Systematic K8 V4 totality proof over every relation-consumed field."""

from __future__ import annotations

import copy
import json
from typing import Any

from build_candidate_v3 import artifacts
from semantic_validator_v4 import (
    interpretation_mode,
    normalize_wire_version,
    validate_reader,
    validate_writer,
)


JSON_SUBSTITUTIONS = {
    "null": None,
    "boolean": True,
    "number": 17.25,
    "string": "__v4_adversarial_string__",
    "array": [None],
    "object": {},
}


def first_code(errors) -> str:
    assert errors
    return errors[0].code


def set_path(document: Any, path: tuple[Any, ...], value: Any) -> None:
    parent = document
    for part in path[:-1]:
        parent = parent[part]
    parent[path[-1]] = value


def row_at(document, predicate=lambda value: True):
    for result_set in document["result_envelope"]["result_sets"]:
        for row in result_set["values"]:
            if predicate(row):
                return result_set, row
    raise AssertionError("no matching row")


def assert_both(document, schema, mapping, expected: str) -> None:
    payload, errors = validate_writer(copy.deepcopy(document), schema, mapping)
    assert payload is None and first_code(errors) == expected, [e.to_dict() for e in errors[:3]]
    wire = json.dumps(document, ensure_ascii=False, separators=(",", ":"), allow_nan=True)
    parsed, errors = validate_reader(wire, schema, mapping)
    assert parsed is None and first_code(errors) == expected, [e.to_dict() for e in errors[:3]]


def relation_fixture(base):
    fixture = copy.deepcopy(base)
    selected_set, selected_row = row_at(
        fixture,
        lambda row: (
            "metadata" in row
            and bool(row.get("source_result_refs"))
            and "basis_ref" in row["source_record"]
            and "metadata" in row["source_record"]
            and bool(row["source_record"].get("source_result_refs"))
        ),
    )
    selected_set["values"] = [selected_row]
    fixture["result_envelope"]["result_sets"] = [selected_set]
    return fixture


def relation_fields() -> list[tuple[str, tuple[Any, ...], str]]:
    root = ("result_envelope",)
    set_path_ = root + ("result_sets", 0)
    row = set_path_ + ("values", 0)
    source = row + ("source_record",)
    classification = row + ("classification",)
    fields = [
        ("outer_schema_version", ("schema_version",), "string"),
        ("result_envelope", root, "object"),
        ("inner_schema_version", root + ("schema_version",), "string"),
        ("result_sets", root + ("result_sets",), "array"),
        ("result_set_item", set_path_, "object"),
        ("set_basis_ref", set_path_ + ("basis_ref",), "object"),
        ("set_basis_ref_type", set_path_ + ("basis_ref", "ref_type"), "string"),
        ("set_basis_ref_id", set_path_ + ("basis_ref", "ref_id"), "string"),
        ("values", set_path_ + ("values",), "array"),
        ("row_item", row, "object"),
        ("result_id", row + ("result_id",), "string"),
        ("magnitude", row + ("magnitude",), "number"),
        ("unit", row + ("unit",), "string"),
        ("family", row + ("family",), "string"),
        ("dimension", row + ("dimension",), "string"),
        ("object_ref", row + ("object_ref",), "object"),
        ("object_ref_type", row + ("object_ref", "ref_type"), "string"),
        ("object_ref_id", row + ("object_ref", "ref_id"), "string"),
        ("row_basis_ref", row + ("basis_ref",), "object"),
        ("row_basis_ref_type", row + ("basis_ref", "ref_type"), "string"),
        ("row_basis_ref_id", row + ("basis_ref", "ref_id"), "string"),
        ("row_source_result_refs", row + ("source_result_refs",), "array"),
        ("row_source_result_ref_item", row + ("source_result_refs", 0), "string"),
        ("row_metadata", row + ("metadata",), "object"),
        ("source_record", source, "object"),
        ("source_id", source + ("id",), "string"),
        ("source_kind", source + ("kind",), "string"),
        ("source_value", source + ("value",), "number"),
        ("source_unit", source + ("unit",), "string"),
        ("source_entity_ref", source + ("entity_ref",), "string"),
        ("source_basis_ref", source + ("basis_ref",), "object"),
        ("source_basis_ref_type", source + ("basis_ref", "ref_type"), "string"),
        ("source_basis_ref_id", source + ("basis_ref", "ref_id"), "string"),
        ("source_result_refs", source + ("source_result_refs",), "array"),
        ("source_result_ref_item", source + ("source_result_refs", 0), "string"),
        ("source_metadata", source + ("metadata",), "object"),
        ("classification", classification, "object"),
        ("source_family", classification + ("source_family",), "string"),
        ("semantic_status", classification + ("semantic_status",), "string"),
        ("canonical_family", classification + ("canonical_family",), "string"),
        ("canonical_dimension", classification + ("canonical_dimension",), "string"),
        ("owner_semantics", classification + ("owner_semantics",), "string"),
    ]
    for side, metadata in (("row", row + ("metadata",)), ("source", source + ("metadata",))):
        for key in ("component", "coordinate_system", "location", "basis", "sign_convention"):
            fields.append((f"{side}_metadata_{key}", metadata + (key,), "string"))
    return fields


def systematic_matrix(fixture, schema, mapping):
    fields = relation_fields()
    first_codes: dict[str, int] = {}
    cases = 0
    boundary_calls = 0
    for label, path, expected_type in fields:
        for replacement_type, replacement in JSON_SUBSTITUTIONS.items():
            case = copy.deepcopy(fixture)
            set_path(case, path, copy.deepcopy(replacement))
            try:
                payload, writer_errors = validate_writer(copy.deepcopy(case), schema, mapping)
                wire = json.dumps(case, ensure_ascii=False, separators=(",", ":"), allow_nan=False)
                parsed, reader_errors = validate_reader(wire, schema, mapping)
            except Exception as error:
                raise AssertionError(f"uncaught {label}/{replacement_type}: {error!r}") from error
            if payload is not None or parsed is not None:
                assert payload is not None and parsed is not None
                assert not writer_errors and not reader_errors
                writer_code = "ACCEPT"
            else:
                writer_code = first_code(writer_errors)
                reader_code = first_code(reader_errors)
                assert writer_code == reader_code, (
                    label, replacement_type, writer_code, reader_code,
                )
            if replacement_type != expected_type:
                assert writer_code == "RESULT_EXPORT_SCHEMA_INVALID", (
                    label, replacement_type, writer_code,
                )
            first_codes[writer_code] = first_codes.get(writer_code, 0) + 1
            cases += 1
            boundary_calls += 2
    return fields, cases, boundary_calls, first_codes


def main() -> None:
    schema, rows, base = artifacts()
    mapping = {(row["kind"], row["unit"]): row for row in rows}

    payload, errors = validate_writer(base, schema, mapping)
    assert payload is not None and not errors
    parsed, errors = validate_reader(payload, schema, mapping)
    assert parsed is not None and not errors
    original_sources = [
        row["source_record"]
        for result_set in base["result_envelope"]["result_sets"]
        for row in result_set["values"]
    ]
    parsed_sources = [
        row["source_record"]
        for result_set in parsed["result_envelope"]["result_sets"]
        for row in result_set["values"]
    ]
    assert len(original_sources) == 830 and parsed_sources == original_sources

    fixture = relation_fixture(base)
    fixture_payload, fixture_errors = validate_writer(fixture, schema, mapping)
    assert fixture_payload is not None and not fixture_errors
    fields, matrix_cases, boundary_calls, matrix_codes = systematic_matrix(
        fixture, schema, mapping
    )

    named_cases = {}
    for label, value in (
        ("source_record_kind_array", []),
        ("source_record_kind_object", {}),
        ("source_record_kind_number", 7),
        ("source_record_unit_array", []),
        ("source_record_unit_object", {}),
    ):
        case = copy.deepcopy(fixture)
        field = "kind" if "kind" in label else "unit"
        case["result_envelope"]["result_sets"][0]["values"][0]["source_record"][field] = value
        assert_both(case, schema, mapping, "RESULT_EXPORT_SCHEMA_INVALID")
        named_cases[label] = "PASS_WRITER_READER"

    non_string_key = copy.deepcopy(fixture)
    non_string_key[7] = "writer-only host key"
    result, errors = validate_writer(non_string_key, schema, mapping)
    assert result is None and first_code(errors) == "RESULT_EXPORT_SCHEMA_INVALID"
    named_cases["writer_non_string_object_key"] = "PASS"

    def assert_mutation(label, mutate, expected):
        case = copy.deepcopy(fixture)
        mutate(case)
        assert_both(case, schema, mapping, expected)
        named_cases[label] = "PASS_WRITER_READER"

    def duplicate(document):
        result_set = document["result_envelope"]["result_sets"][0]
        duplicate_set = copy.deepcopy(result_set)
        duplicate_set["set_id"] += ":duplicate"
        document["result_envelope"]["result_sets"].append(duplicate_set)

    assert_mutation("duplicate_result_id", duplicate, "RESULT_EXPORT_DUPLICATE_RESULT_ID")
    assert_mutation(
        "mirror_id",
        lambda document: document["result_envelope"]["result_sets"][0]["values"][0].__setitem__(
            "result_id", "result:v4:changed"
        ),
        "RESULT_EXPORT_SOURCE_RECORD_ID_MISMATCH",
    )
    assert_mutation(
        "mirror_value",
        lambda document: document["result_envelope"]["result_sets"][0]["values"][0].__setitem__(
            "magnitude", 1.25
        ),
        "RESULT_EXPORT_SOURCE_RECORD_VALUE_MISMATCH",
    )
    assert_mutation(
        "mirror_unit",
        lambda document: document["result_envelope"]["result_sets"][0]["values"][0].__setitem__(
            "unit", "kN"
        ),
        "RESULT_EXPORT_SOURCE_RECORD_UNIT_MISMATCH",
    )
    assert_mutation(
        "metadata_mirror",
        lambda document: document["result_envelope"]["result_sets"][0]["values"][0]["metadata"].__setitem__(
            "component", "v4-changed-component"
        ),
        "RESULT_EXPORT_SOURCE_RECORD_METADATA_MISMATCH",
    )
    assert_mutation(
        "reference_projection",
        lambda document: document["result_envelope"]["result_sets"][0]["values"][0]["object_ref"].__setitem__(
            "ref_id", "component:v4-changed"
        ),
        "RESULT_EXPORT_SOURCE_RECORD_REFERENCE_MISMATCH",
    )
    assert_mutation(
        "owner",
        lambda document: document["result_envelope"]["result_sets"][0]["values"][0]["classification"].__setitem__(
            "owner_semantics", "PKG08_TRANSPORT_ONLY"
        ),
        "RESULT_EXPORT_CLASSIFICATION_OWNER_MISMATCH",
    )

    def unknown_pair(document):
        row = document["result_envelope"]["result_sets"][0]["values"][0]
        row["source_record"]["kind"] = "future-native-kind"
        row["source_record"]["unit"] = "future-unit"
        row["unit"] = "future-unit"

    assert_mutation(
        "unknown_kind_unit_pair", unknown_pair,
        "RESULT_EXPORT_NATIVE_KIND_UNIT_UNSUPPORTED",
    )

    for label, number in (
        ("nan", float("nan")),
        ("positive_infinity", float("inf")),
        ("negative_infinity", float("-inf")),
    ):
        case = copy.deepcopy(fixture)
        row = case["result_envelope"]["result_sets"][0]["values"][0]
        row["magnitude"] = number
        row["source_record"]["value"] = number
        assert_both(case, schema, mapping, "RESULT_EXPORT_NON_FINITE_NUMBER")
        named_cases[f"strict_finite_{label}"] = "PASS_WRITER_READER"

    pending_row = fixture["result_envelope"]["result_sets"][0]["values"][0]
    assert "pending_P5" in pending_row["classification"]["semantic_status"]
    assert pending_row["classification"]["owner_semantics"] == "PKG04_05_MECHANICS"
    assert interpretation_mode(pending_row) == "preserve_only"
    named_cases["pressure_p5_owner_and_preserve_only_sequence"] = "PASS"

    duplicate_member = '{"schema_version":"0.2.0","schema_version":"0.2.0"}'
    parsed, errors = validate_reader(duplicate_member, schema, mapping)
    assert parsed is None and first_code(errors) == "RESULT_EXPORT_JSON_DUPLICATE_MEMBER"
    named_cases["duplicate_json_member_reader"] = "PASS"

    ingress = {
        "direct_writer": ("requested_schema_version", ("0.2.0",)),
        "tauri_raw_command": ("requested_schema_version", ("0.2.0",)),
        "report_package_cli": ("requested_result_schema_version", ("0.1.0", "0.2.0")),
    }
    ingress_cases = 0
    for field, supported in ingress.values():
        for raw, expected in (
            ({}, "RESULT_EXPORT_SCHEMA_VERSION_REQUIRED"),
            ({field: None}, "RESULT_EXPORT_SCHEMA_VERSION_REQUIRED"),
            ({field: ""}, "RESULT_EXPORT_SCHEMA_VERSION_REQUIRED"),
            ({field: 7}, "RESULT_EXPORT_SCHEMA_INVALID"),
            ({field: "9.9.9"}, "RESULT_EXPORT_SCHEMA_VERSION_UNSUPPORTED"),
        ):
            version, error = normalize_wire_version(raw, field, supported)
            assert version is None and error is not None and error.code == expected
            ingress_cases += 1
        version, error = normalize_wire_version({field: supported[-1]}, field, supported)
        assert version == supported[-1] and error is None
        ingress_cases += 1

    result = {
        "status": "PASS",
        "valid_830_row_round_trip": "PASS_EXACT_SOURCE_RECORD_EQUALITY",
        "relation_fields": len(fields),
        "json_value_classes": list(JSON_SUBSTITUTIONS),
        "systematic_substitution_cases": matrix_cases,
        "writer_reader_boundary_calls": boundary_calls,
        "systematic_first_code_counts": matrix_codes,
        "named_regressions": named_cases,
        "version_ingress_cases": ingress_cases,
        "uncaught_exceptions": 0,
        "writes_performed_by_test": 0,
    }
    print(json.dumps(result, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
