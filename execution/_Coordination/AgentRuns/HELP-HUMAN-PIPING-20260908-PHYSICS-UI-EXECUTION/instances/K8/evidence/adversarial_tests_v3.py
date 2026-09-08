#!/usr/bin/env python3
"""Focused K8 V3 totality and wire-ingress probes."""

from __future__ import annotations

import copy
import json
from pathlib import Path

from build_candidate_v3 import artifacts
from semantic_validator_v3 import normalize_wire_version, validate_reader, validate_writer


HERE = Path(__file__).resolve().parent


def first_code(errors) -> str:
    assert errors
    return errors[0].code


def assert_both(value, schema, mapping, expected):
    payload, errors = validate_writer(copy.deepcopy(value), schema, mapping)
    assert payload is None and first_code(errors) == expected
    parsed, errors = validate_reader(json.dumps(value, allow_nan=True), schema, mapping)
    assert parsed is None and first_code(errors) == expected


def first_row(document):
    return document["result_envelope"]["result_sets"][0]["values"][0]


def main() -> None:
    schema, rows, base = artifacts()
    mapping = {(row["kind"], row["unit"]): row for row in rows}
    payload, errors = validate_writer(base, schema, mapping)
    assert payload is not None and not errors
    parsed, errors = validate_reader(payload, schema, mapping)
    assert parsed is not None and not errors

    top_shapes = [None, [], "scalar", 7, True]
    for value in top_shapes:
        assert_both(value, schema, mapping, "RESULT_EXPORT_SCHEMA_INVALID")

    nested_cases = {}
    for label, mutate in {
        "result_envelope_null": lambda d: d.__setitem__("result_envelope", None),
        "result_sets_null": lambda d: d["result_envelope"].__setitem__("result_sets", None),
        "result_set_scalar": lambda d: d["result_envelope"].__setitem__("result_sets", [3]),
        "values_scalar": lambda d: d["result_envelope"]["result_sets"][0].__setitem__("values", "bad"),
        "row_null": lambda d: d["result_envelope"]["result_sets"][0].__setitem__("values", [None]),
        "source_record_null": lambda d: first_row(d).__setitem__("source_record", None),
        "classification_null": lambda d: first_row(d).__setitem__("classification", None),
        "classification_scalar": lambda d: first_row(d).__setitem__("classification", "bad"),
        "object_ref_null": lambda d: first_row(d).__setitem__("object_ref", None),
    }.items():
        case = copy.deepcopy(base)
        mutate(case)
        assert_both(case, schema, mapping, "RESULT_EXPORT_SCHEMA_INVALID")
        nested_cases[label] = "PASS_WRITER_READER"

    for which in ("outer", "inner"):
        case = copy.deepcopy(base)
        if which == "outer":
            case["schema_version"] = "9.9.9"
        else:
            case["result_envelope"]["schema_version"] = "9.9.9"
        assert_both(case, schema, mapping, "RESULT_EXPORT_SCHEMA_VERSION_UNSUPPORTED")

    ingress = {
        "direct_writer": ("requested_schema_version", ("0.2.0",)),
        "tauri_raw_command": ("requested_schema_version", ("0.2.0",)),
        "report_package_cli": ("requested_result_schema_version", ("0.1.0", "0.2.0")),
    }
    ingress_results = {}
    for boundary, (field, supported) in ingress.items():
        for label, raw, expected in [
            ("absent", {}, "RESULT_EXPORT_SCHEMA_VERSION_REQUIRED"),
            ("null", {field: None}, "RESULT_EXPORT_SCHEMA_VERSION_REQUIRED"),
            ("empty", {field: ""}, "RESULT_EXPORT_SCHEMA_VERSION_REQUIRED"),
            ("unknown", {field: "9.9.9"}, "RESULT_EXPORT_SCHEMA_VERSION_UNSUPPORTED"),
        ]:
            version, error = normalize_wire_version(raw, field, supported)
            assert version is None and error is not None and error.code == expected
            ingress_results[f"{boundary}_{label}"] = "PASS"
        version, error = normalize_wire_version({field: supported[-1]}, field, supported)
        assert version == supported[-1] and error is None
        ingress_results[f"{boundary}_supported"] = "PASS"

    result = {
        "status": "PASS",
        "valid_830_row_round_trip": "PASS",
        "top_level_shape_cases": len(top_shapes),
        "nested_shape_cases": nested_cases,
        "unknown_outer_inner_version": "PASS_WRITER_READER",
        "ingress_cases": ingress_results,
        "uncaught_exceptions": 0,
    }
    (HERE / "ADVERSARIAL_VALIDATION_V3.json").write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
