#!/usr/bin/env python3
"""Stdlib checks for the material library schema."""

import json
from pathlib import Path
import sys


ROOT = Path(__file__).resolve().parents[1]
TESTS_DIR = Path(__file__).resolve().parent
if str(TESTS_DIR) not in sys.path:
    sys.path.insert(0, str(TESTS_DIR))

from schema_validation import (  # noqa: E402
    JsonSchemaDependencyMissing,
    validate_instance,
    validate_schema_document,
)

SCHEMA_PATH = ROOT / "schemas" / "material.schema.yaml"
FIXTURE_PATH = ROOT / "fixtures" / "material" / "invented_material_library_valid.json"

CANONICAL_DIMENSIONS = {
    "dimensionless",
    "length",
    "mass",
    "time",
    "temperature",
    "temperature_interval",
    "angle",
    "rotation",
    "force",
    "moment",
    "pressure",
    "stress",
    "area",
    "volume",
    "density",
    "linear_stiffness",
    "rotational_stiffness",
    "displacement",
    "velocity",
    "acceleration",
    "thermal_conductivity",
    "specific_heat",
    "thermal_expansion_coefficient",
    "second_moment_area",
    "section_modulus",
    "mass_per_length",
    "volume_per_length",
    "slope",
    "TBD",
}

RETIRED_DIMENSIONS = {
    "temperature_difference",
    "area_moment",
    "conductivity",
    "stiffness",
}

REQUIRED_TOP_LEVEL = {
    "schema_version",
    "material_library",
    "material_records",
    "property_definitions",
    "completeness_rules",
    "diagnostics",
    "open_decisions",
}

REQUIRED_MATERIAL_FIELDS = {
    "material_id",
    "name",
    "material_family",
    "privacy_class",
    "redistribution_status",
    "properties",
    "allowables",
    "completeness",
    "provenance",
    "review_status",
}

REQUIRED_PROVENANCE_FIELDS = {
    "source_name",
    "source_location",
    "source_license",
    "contributor",
    "contributor_certification",
    "redistribution_status",
    "review_status",
}

FORBIDDEN_PUBLIC_DATA_TEXT = {
    "ASME",
    "B31",
    "ASTM",
    "CODE_COMPLIANT",
    "certified material",
    "sealed",
    "automatic compliance",
    "professional approval by the software",
}


def load_json(path):
    with path.open(encoding="utf-8") as handle:
        return json.load(handle)


def walk_strings(value):
    if isinstance(value, str):
        yield value
    elif isinstance(value, dict):
        for item in value.values():
            yield from walk_strings(item)
    elif isinstance(value, list):
        for item in value:
            yield from walk_strings(item)


def walk_keys(value):
    if isinstance(value, dict):
        for key, item in value.items():
            yield key
            yield from walk_keys(item)
    elif isinstance(value, list):
        for item in value:
            yield from walk_keys(item)


def required_at(schema, definition_name):
    return set(schema["$defs"][definition_name]["required"])


def enum_at(schema, definition_name):
    return set(schema["$defs"][definition_name]["enum"])


def main():
    schema = load_json(SCHEMA_PATH)
    fixture = load_json(FIXTURE_PATH)
    defs = schema["$defs"]

    assert schema["$schema"] == "https://json-schema.org/draft/2020-12/schema"
    assert schema["additionalProperties"] is False
    assert "default" not in set(walk_keys(schema))
    assert REQUIRED_TOP_LEVEL <= set(schema["required"])

    assert REQUIRED_MATERIAL_FIELDS <= required_at(schema, "MaterialRecord")
    assert REQUIRED_PROVENANCE_FIELDS <= required_at(schema, "Provenance")
    assert {
        "public_permissive",
        "private_only",
        "unknown",
        "protected_suspected",
        "rejected",
        "TBD",
    } <= enum_at(schema, "RedistributionStatus")

    assert {
        "density",
        "elastic_modulus",
        "poisson_ratio",
        "thermal_expansion_coefficient",
        "allowable_stress",
        "user_defined",
        "TBD",
    } <= enum_at(schema, "MaterialPropertyKind")

    # DEC-068 item 1: temperature-indexed slots carry exact-selection solve
    # semantics; interpolation remains an open decision (drafted as D-38).
    temperature_ref = defs["MaterialPropertyValue"]["properties"]["temperature_ref"]
    assert temperature_ref["$ref"] == "#/$defs/Reference"
    assert "exact" in temperature_ref["description"]
    assert "no interpolation" in temperature_ref["description"]
    assert (
        "temperature_interpolation_policy"
        in defs["OpenDecision"]["properties"]["topic"]["enum"]
    )
    assert {
        "density",
        "stress",
        "temperature",
        "temperature_interval",
        "thermal_conductivity",
        "specific_heat",
        "thermal_expansion_coefficient",
        "dimensionless",
        "TBD",
    } <= enum_at(schema, "MaterialPropertyDimension")
    material_dimensions = enum_at(schema, "MaterialPropertyDimension")
    assert material_dimensions <= CANONICAL_DIMENSIONS
    assert not (material_dimensions & RETIRED_DIMENSIONS)

    allowable = defs["MaterialAllowableSlot"]
    assert {
        "allowable_id",
        "allowable_kind",
        "value_status",
        "public_repository_value_policy",
        "required_for",
        "provenance",
        "review_status",
    } <= set(allowable["required"])
    assert "no_public_code_specific_values" in set(
        allowable["properties"]["public_repository_value_policy"]["enum"]
    )
    assert "private_user_supplied_only" in set(
        allowable["properties"]["public_repository_value_policy"]["enum"]
    )

    completeness_rule = defs["CompletenessRule"]
    assert {
        "rule_id",
        "applies_to",
        "required_property_kinds",
        "required_for",
        "missing_behavior",
        "diagnostic_code",
        "review_status",
    } <= set(completeness_rule["required"])
    assert "diagnostic_blocking" in set(
        completeness_rule["properties"]["missing_behavior"]["enum"]
    )

    diagnostic_codes = enum_at(schema, "MaterialDiagnosticCode")
    assert {
        "MATERIAL_PROPERTY_MISSING",
        "MATERIAL_UNIT_MISSING",
        "MATERIAL_PROVENANCE_MISSING",
        "MATERIAL_PROTECTED_CONTENT_SUSPECTED",
        "MATERIAL_ALLOWABLE_NOT_PUBLIC",
    } <= diagnostic_codes
    assert {"class", "source"} <= required_at(schema, "MaterialDiagnostic")

    open_decision = defs["OpenDecision"]
    assert {
        "public_material_fixture_policy",
        "accepted_material_source_catalog",
        "allowable_value_storage_policy",
        "temperature_interpolation_policy",
    } <= set(open_decision["properties"]["topic"]["enum"])

    assert fixture["material_library"]["library_scope"] == "public_schema_fixture"
    assert fixture["material_records"][0]["redistribution_status"] == "TBD"
    assert (
        fixture["material_records"][0]["allowables"][0][
            "public_repository_value_policy"
        ]
        == "no_public_code_specific_values"
    )
    assert fixture["material_records"][0]["completeness"][0]["status"] == "incomplete"
    assert fixture["diagnostics"][0]["code"] == "MATERIAL_PROPERTY_MISSING"
    assert fixture["diagnostics"][0]["class"] == "SOLVE_BLOCKING"
    assert fixture["diagnostics"][0]["source"] == (
        "fixtures/material/invented_material_library_valid.json"
    )

    all_text = "\n".join([*walk_strings(schema), *walk_strings(fixture)])
    for forbidden in FORBIDDEN_PUBLIC_DATA_TEXT:
        assert forbidden not in all_text


def check_jsonschema_validation():
    schema = load_json(SCHEMA_PATH)
    fixture = load_json(FIXTURE_PATH)
    try:
        assert validate_schema_document(schema, schema_label=str(SCHEMA_PATH))
        assert validate_instance(
            schema,
            fixture,
            schema_label=str(SCHEMA_PATH),
            instance_label=str(FIXTURE_PATH),
        )
    except JsonSchemaDependencyMissing as exc:
        _skip_or_note_missing_jsonschema(exc)


def _skip_or_note_missing_jsonschema(exc):
    if "pytest" in sys.modules:
        import pytest

        pytest.skip(str(exc))
    print(f"SKIP: {exc}")


def test_material_schema_contract():
    main()


def test_material_schema_jsonschema_validation_helper():
    check_jsonschema_validation()


if __name__ == "__main__":
    main()
    check_jsonschema_validation()
