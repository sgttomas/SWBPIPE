#!/usr/bin/env python3
"""Stdlib checks for the unit system schema."""

import json
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
TESTS_DIR = Path(__file__).resolve().parent
if str(TESTS_DIR) not in sys.path:
    sys.path.insert(0, str(TESTS_DIR))

from schema_validation import (  # noqa: E402
    JsonSchemaDependencyMissing,
    _skip_or_note_missing_jsonschema,
    load_schema,
    required_at,
    validate_instance,
    validate_schema_document,
    walk_keys,
    walk_strings,
)

SCHEMA_PATH = ROOT / "schemas" / "units.schema.yaml"
FIXTURE_PATH = ROOT / "fixtures" / "units" / "invented_unit_contract_fixture.json"

CANONICAL_DIMENSIONS = [
    "dimensionless",
    "length",
    "mass",
    "time",
    "temperature",
    "temperature_interval",
    "angle",
    "rotation",
    "force",
    "force_per_length",
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
]

RETIRED_DIMENSIONS = {
    "temperature_difference",
    "area_moment",
    "stiffness",
}

REQUIRED_QUANTITY_FIELDS = {
    "quantity_id",
    "quantity_kind",
    "magnitude",
    "unit_ref",
    "dimension_id",
    "unit_required",
    "missing_unit_behavior",
    "provenance",
}

FORBIDDEN_DEFAULT_TERMS = {
    "assume zero",
    "assumed zero",
    "assume unity",
    "assumed unity",
    "defaults to",
    "default value",
    "fallback default",
    "implicit default",
    "silently",
}


def load_fixture():
    with FIXTURE_PATH.open(encoding="utf-8") as fixture_file:
        return json.load(fixture_file)


def check_schema_contract():
    schema = load_schema(SCHEMA_PATH)
    defs = schema["$defs"]

    assert schema["$schema"] == "https://json-schema.org/draft/2020-12/schema"
    assert schema["additionalProperties"] is False
    assert "default" not in set(walk_keys(schema))

    required_root = {
        "schema_version",
        "unit_system",
        "dimension_records",
        "quantity_records",
        "conversion_declarations",
        "dimension_checks",
        "operation_rules",
        "test_obligations",
        "open_decisions",
        "diagnostics",
    }
    assert required_root <= set(schema["required"])

    dimensions = defs["DimensionId"]["enum"]
    assert dimensions == CANONICAL_DIMENSIONS
    assert not (set(dimensions) & RETIRED_DIMENSIONS)

    vector = defs["DimensionVector"]
    assert {
        "length",
        "mass",
        "time",
        "temperature",
        "angle",
        "electric_current",
        "substance_amount",
        "luminous_intensity",
    } <= set(vector["required"])

    quantity = defs["QuantityRecord"]
    assert REQUIRED_QUANTITY_FIELDS <= set(quantity["required"])
    assert quantity["additionalProperties"] is False
    assert quantity["properties"]["missing_unit_behavior"]["enum"] == [
        "diagnostic_blocking",
        "diagnostic_warning",
        "not_applicable_explicit_dimensionless",
        "TBD",
    ]

    conversion = defs["ConversionDeclaration"]
    assert {
        "conversion_id",
        "source_unit_ref",
        "target_unit_ref",
        "dimension_id",
        "transform_kind",
        "factor_representation",
        "provenance",
        "review_status",
    } <= set(conversion["required"])

    operation = defs["OperationRule"]
    assert {
        "operation",
        "compatibility_rule",
        "unsupported_behavior",
        "diagnostic_codes",
        "review_status",
    } <= set(operation["required"])
    assert {
        "same_dimension_required",
        "derived_dimension_required",
        "explicit_dimensionless_classification_required",
        "unsupported_until_decision",
    } <= set(operation["properties"]["compatibility_rule"]["enum"])
    assert {
        "addition",
        "subtraction",
        "comparison",
        "conversion",
        "multiplication",
        "division",
        "power",
        "dimensionless_classification",
        "import_validation",
        "export_validation",
        "rule_evaluation",
    } <= set(operation["properties"]["operation"]["enum"])

    test_obligation = defs["TestObligation"]
    assert {
        "test_id",
        "test_kind",
        "required_for",
        "fixture_data_policy",
        "gating_status",
        "evidence_ref",
    } <= set(test_obligation["required"])
    assert "conversion_gated_pending_constants" in test_obligation["properties"]["test_kind"]["enum"]
    assert (
        "no_numeric_conversion_constants_until_approved"
        in test_obligation["properties"]["fixture_data_policy"]["enum"]
    )
    assert "blocked_pending_decision" in test_obligation["properties"]["gating_status"]["enum"]

    open_decision = defs["OpenDecision"]
    assert {
        "decision_id",
        "topic",
        "status",
        "blocking_scope",
        "required_before",
        "owner",
        "notes",
    } <= set(open_decision["required"])
    assert {
        "unit_catalog",
        "base_dimension_vector",
        "dimensionless_classification",
        "conversion_tolerance_policy",
        "offset_temperature_semantics",
        "gauge_absolute_pressure_semantics",
        "diagnostic_code_namespace",
    } <= set(open_decision["properties"]["topic"]["enum"])

    diagnostic_codes = set(defs["UnitDiagnosticCode"]["enum"])
    assert {
        "UNIT_MISSING",
        "UNIT_UNKNOWN",
        "UNIT_AMBIGUOUS",
        "DIMENSION_MISMATCH",
        "CONVERSION_UNSUPPORTED",
        "DIMENSIONLESS_CLASSIFICATION_REQUIRED",
        "PROTECTED_UNIT_DATA_SUSPECTED",
        "CONVERSION_TEST_GATED",
    } <= diagnostic_codes
    assert defs["UnitDiagnostic"]["properties"]["code"]["$ref"] == "#/$defs/UnitDiagnosticCode"

    all_text = "\n".join(walk_strings(schema)).lower()
    for forbidden in FORBIDDEN_DEFAULT_TERMS:
        assert forbidden not in all_text


def check_unit_fixture():
    schema = load_schema(SCHEMA_PATH)
    fixture = load_fixture()

    assert set(schema["required"]) <= set(fixture)
    assert set(fixture["unit_system"]) >= required_at(schema, "UnitSystemRecord")
    assert fixture["unit_system"]["storage_convention"] == "entered_units_preserved"
    assert fixture["dimension_records"]
    assert {item["dimension_id"] for item in fixture["dimension_records"]} >= {
        "length",
        "force",
    }

    quantity = fixture["quantity_records"][0]
    assert required_at(schema, "QuantityRecord") <= set(quantity)
    assert quantity["unit_required"] is True
    assert quantity["missing_unit_behavior"] == "diagnostic_blocking"

    diagnostic_codes = {item["code"] for item in fixture["diagnostics"]}
    assert "UNIT_MISSING" in diagnostic_codes
    assert fixture["dimension_checks"][0]["outcome"] == "blocked"
    diagnostic_items = schema["$defs"]["DimensionCheckRecord"]["properties"]["diagnostics"][
        "items"
    ]
    assert {"oneOf"} <= set(diagnostic_items)
    assert any(
        item.get("$ref") == "#/$defs/Reference" for item in diagnostic_items["oneOf"]
    )
    assert fixture["dimension_checks"][0]["diagnostics"][0] == {
        "ref_type": "UnitDiagnostic",
        "ref_id": "unit-diagnostic:missing-unit",
    }

    operation_kinds = {item["operation"] for item in fixture["operation_rules"]}
    assert {"addition", "conversion", "rule_evaluation"} <= operation_kinds

    open_topics = {item["topic"] for item in fixture["open_decisions"]}
    assert {
        "unit_catalog",
        "conversion_tolerance_policy",
        "canonical_calculation_basis",
        "offset_temperature_semantics",
        "gauge_absolute_pressure_semantics",
    } <= open_topics
    assert {item["status"] for item in fixture["open_decisions"]} == {"open_TBD"}

    gated_tests = {
        item["test_kind"]: item for item in fixture["test_obligations"]
    }
    assert gated_tests["conversion_gated_pending_constants"]["gating_status"] == (
        "blocked_pending_decision"
    )
    assert fixture["conversion_declarations"][0]["transform_kind"] == "unsupported_TBD"


def check_jsonschema_validation():
    schema = load_schema(SCHEMA_PATH)
    fixture = load_fixture()
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


def test_units_schema_contract():
    check_schema_contract()


def test_unit_fixture_preserves_missing_unit_diagnostics_and_tbd_gates():
    check_unit_fixture()


def test_unit_schema_jsonschema_validation_helper():
    check_jsonschema_validation()


def main():
    check_schema_contract()
    check_unit_fixture()
    check_jsonschema_validation()


if __name__ == "__main__":
    main()
