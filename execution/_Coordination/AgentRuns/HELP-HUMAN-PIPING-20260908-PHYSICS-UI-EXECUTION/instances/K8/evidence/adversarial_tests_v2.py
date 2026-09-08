#!/usr/bin/env python3
"""Focused executable probes for the K8 V2 runtime-validation contract."""

from __future__ import annotations

import copy
import json
from pathlib import Path

from jsonschema import Draft202012Validator

from build_candidate_v2 import artifacts
from semantic_validator_v2 import (
    interpretation_mode,
    serialize_strict_json,
    validate_reader,
    validate_writer,
)


def code(errors) -> str:
    assert errors, "mutation unexpectedly passed"
    return errors[0].code


def row_at(document, predicate=lambda value: True):
    for result_set in document["result_envelope"]["result_sets"]:
        for value in result_set["values"]:
            if predicate(value):
                return result_set, value
    raise AssertionError("no matching row")


def assert_both(base, schema, mapping, mutate, expected):
    writer_case = copy.deepcopy(base)
    mutate(writer_case)
    payload, errors = validate_writer(writer_case, schema, mapping)
    assert payload is None and code(errors) == expected, (expected, [e.to_dict() for e in errors[:3]])

    reader_case = copy.deepcopy(base)
    mutate(reader_case)
    payload = json.dumps(reader_case, separators=(",", ":"), allow_nan=True)
    parsed, errors = validate_reader(payload, schema, mapping)
    assert parsed is None and code(errors) == expected, (expected, [e.to_dict() for e in errors[:3]])


def main() -> None:
    schema, rows, base = artifacts()
    mapping = {(r["kind"], r["unit"]): r for r in rows}
    payload, errors = validate_writer(base, schema, mapping)
    assert payload is not None and not errors
    parsed, errors = validate_reader(payload, schema, mapping)
    assert parsed is not None and not errors

    checks = {}

    def duplicate(document):
        result_set = document["result_envelope"]["result_sets"][0]
        duplicate_set = copy.deepcopy(result_set)
        duplicate_set["set_id"] += ":duplicate-probe"
        duplicate_set["values"] = [copy.deepcopy(result_set["values"][0])]
        document["result_envelope"]["result_sets"].append(duplicate_set)
    assert_both(base, schema, mapping, duplicate, "RESULT_EXPORT_DUPLICATE_RESULT_ID")
    checks["duplicate_result_id"] = "PASS"

    mutations = {
        "id": ("RESULT_EXPORT_SOURCE_RECORD_ID_MISMATCH", lambda row: row.__setitem__("result_id", row["result_id"] + ":changed")),
        "value": ("RESULT_EXPORT_SOURCE_RECORD_VALUE_MISMATCH", lambda row: row.__setitem__("magnitude", row["magnitude"] + 1)),
        "unit": ("RESULT_EXPORT_SOURCE_RECORD_UNIT_MISMATCH", lambda row: row.__setitem__("unit", row["unit"] + ":changed")),
    }
    for label, (expected, row_mutation) in mutations.items():
        def apply(document, row_mutation=row_mutation):
            _, row = row_at(document)
            row_mutation(row)
        assert_both(base, schema, mapping, apply, expected)
        checks[f"mirror_{label}"] = "PASS"

    def metadata(document):
        _, row = row_at(document, lambda value: "metadata" in value)
        row["metadata"] = dict(row["metadata"], component=row["metadata"]["component"] + ":changed")
    assert_both(base, schema, mapping, metadata, "RESULT_EXPORT_SOURCE_RECORD_METADATA_MISMATCH")
    def metadata_presence(document):
        _, row = row_at(document, lambda value: "metadata" in value)
        del row["metadata"]
    assert_both(base, schema, mapping, metadata_presence, "RESULT_EXPORT_SOURCE_RECORD_METADATA_MISMATCH")
    checks["metadata_value_and_presence"] = "PASS"

    def object_ref(document):
        _, row = row_at(document)
        row["object_ref"]["ref_id"] += ":changed"
    assert_both(base, schema, mapping, object_ref, "RESULT_EXPORT_SOURCE_RECORD_REFERENCE_MISMATCH")
    checks["object_reference"] = "PASS"

    def basis_ref(document):
        _, row = row_at(document)
        row["basis_ref"]["ref_id"] += ":changed"
    assert_both(base, schema, mapping, basis_ref, "RESULT_EXPORT_SOURCE_RECORD_REFERENCE_MISMATCH")
    checks["basis_reference"] = "PASS"

    def source_refs(document):
        _, row = row_at(document, lambda value: len(value.get("source_result_refs", [])) > 1)
        row["source_result_refs"].reverse()
    assert_both(base, schema, mapping, source_refs, "RESULT_EXPORT_SOURCE_RECORD_REFERENCE_MISMATCH")
    checks["source_result_ref_order"] = "PASS"

    def nonfinite(document):
        _, row = row_at(document)
        row["magnitude"] = float("nan")
        row["source_record"]["value"] = float("nan")
    assert_both(base, schema, mapping, nonfinite, "RESULT_EXPORT_NON_FINITE_NUMBER")
    def positive_infinity(document):
        _, row = row_at(document)
        row["magnitude"] = float("inf")
        row["source_record"]["value"] = float("inf")
    assert_both(base, schema, mapping, positive_infinity, "RESULT_EXPORT_NON_FINITE_NUMBER")
    checks["nonfinite_writer_and_reader"] = "PASS_NAN_AND_POSITIVE_INFINITY"

    def owner(document):
        _, row = row_at(document, lambda value: "pending_P5" in value["classification"]["semantic_status"])
        row["classification"]["owner_semantics"] = "PKG08_TRANSPORT_ONLY"
    assert_both(base, schema, mapping, owner, "RESULT_EXPORT_CLASSIFICATION_OWNER_MISMATCH")
    schema_case = copy.deepcopy(base)
    owner(schema_case)
    assert list(Draft202012Validator(schema).iter_errors(schema_case))
    checks["owner_runtime_and_schema"] = "PASS"

    def unsupported(document):
        _, row = row_at(document)
        row["source_record"]["unit"] = "future-unit"
        row["unit"] = "future-unit"
    assert_both(base, schema, mapping, unsupported, "RESULT_EXPORT_NATIVE_KIND_UNIT_UNSUPPORTED")
    checks["unsupported_kind_unit"] = "PASS"

    _, pending = row_at(base, lambda value: "pending_P5" in value["classification"]["semantic_status"])
    assert interpretation_mode(pending) == "preserve_only"
    _, observation = row_at(base, lambda value: value["classification"]["semantic_status"] == "transport_observation_only")
    assert interpretation_mode(observation) == "preserve_only"
    assert base["result_envelope"]["result_sets"][0]["set_type"] == "mechanics"
    checks["row_status_over_set_type"] = "PASS"

    checks["valid_writer_reader_round_trip"] = "PASS"
    checks["total_adversarial_invariants"] = 13
    result = {"status": "PASS", "checks": checks}
    (Path(__file__).resolve().parent / "ADVERSARIAL_VALIDATION_V2.json").write_text(
        json.dumps(result, indent=2) + "\n"
    )
    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
