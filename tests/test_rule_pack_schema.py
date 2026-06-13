#!/usr/bin/env python3
"""Stdlib checks for the rule-pack schema."""

import json
import sys
from copy import deepcopy
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCHEMA_PATH = ROOT / "schemas" / "rule_pack.schema.yaml"
EXAMPLE_PATH = ROOT / "examples" / "rule_packs" / "invented_demo.yaml"

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

REQUIRED_TOP_LEVEL = {
    "metadata",
    "classification",
    "required_inputs",
    "formula_declarations",
    "value_slots",
    "check_definitions",
    "diagnostics",
    "checksums",
    "provenance",
    "professional_boundary",
}

REQUIRED_PROVENANCE = {
    "source_name",
    "source_type",
    "source_location",
    "source_license",
    "contributor",
    "contributor_certification",
    "redistribution_status",
    "protected_content_review",
    "review_status",
}

MIN_ITEM_ARRAYS = {
    "required_inputs",
    "formula_declarations",
    "value_slots",
    "check_definitions",
    "diagnostics",
    "open_decisions",
}

REQUIRED_CHECKSUM = {
    "algorithm",
    "canonicalization",
    "payload_ref",
    "payload_scope",
    "verification_status",
    "value",
}

REQUIRED_DIAGNOSTIC_POLICY = {
    "rule_check_blocking",
    "missing_input",
    "unit_mismatch",
    "provenance_gap",
    "redistribution_gap",
    "checksum_mismatch",
    "protected_content_suspected",
    "evaluator_error",
}

REQUIRED_PROFESSIONAL_BOUNDARY = {
    "software_makes_compliance_claim",
    "software_makes_certification_claim",
    "software_makes_sealing_claim",
    "software_makes_approval_claim",
    "software_makes_authentication_claim",
    "human_review_required",
    "human_acceptance_record_software_generated",
    "external_human_acceptance_ref_allowed",
    "hash_bound_human_acceptance_required",
    "reliance_notice",
}

FORBIDDEN_SCHEMA_TEXT = {
    "CODE_COMPLIANT",
    "certified",
    "sealed",
    "automatic compliance",
    "professional approval by the software",
}


def load_schema():
    with SCHEMA_PATH.open(encoding="utf-8") as schema_file:
        return json.load(schema_file)


def load_example():
    with EXAMPLE_PATH.open(encoding="utf-8") as example_file:
        return json.load(example_file)


def definition(schema, name):
    return schema["$defs"][name]


def required_at(schema, definition_name):
    return set(definition(schema, definition_name)["required"])


def walk_strings(value):
    if isinstance(value, str):
        yield value
    elif isinstance(value, dict):
        for item in value.values():
            yield from walk_strings(item)
    elif isinstance(value, list):
        for item in value:
            yield from walk_strings(item)


EXPRESSION_NODE_KINDS = {
    "literal",
    "variable_ref",
    "unary",
    "binary",
    "compare",
    "logical",
    "select",
    "aggregate",
    "interpolate",
    "lookup",
}

EVALUATOR_REFUSAL_MARKERS = {
    "unsupported_form",
    "unsafe_host_access",
}

REQUIRED_USER_TABLE = {
    "table_id",
    "argument_dimension",
    "argument_unit_ref",
    "result_dimension",
    "result_unit_ref",
    "rows",
}

FROZEN_GRAMMAR_STATUS = "frozen_open_pipe_stress_declared_expression"
FROZEN_EXPRESSION_LANGUAGE = "open_pipe_stress_declared_expression"
SEMVER_PATTERN = "^[0-9]+\\.[0-9]+\\.[0-9]+$"


def check_schema_contract():
    schema = load_schema()
    defs = schema["$defs"]

    assert schema["$schema"] == "https://json-schema.org/draft/2020-12/schema"
    assert schema["additionalProperties"] is False
    assert REQUIRED_TOP_LEVEL <= set(schema["required"])
    assert "grammar_version" in schema["required"]
    assert schema["properties"]["grammar_version"]["pattern"] == SEMVER_PATTERN
    for property_name in MIN_ITEM_ARRAYS:
        assert schema["properties"][property_name]["minItems"] == 1

    metadata = definition(schema, "RulePackMetadata")
    assert {
        "rule_pack_id",
        "schema_version",
        "rule_pack_version",
        "lifecycle_status",
        "source_notice",
    } <= set(metadata["required"])

    classification = definition(schema, "RulePackClassification")
    assert classification["properties"]["protected_content_review_required"][
        "const"
    ] is True
    assert classification["properties"]["private_values_allowed"]["const"] is True
    assert "private_user_data_not_committed" in classification["properties"][
        "public_repository_policy"
    ]["enum"]

    provenance = definition(schema, "ProvenanceRecord")
    assert REQUIRED_PROVENANCE <= set(provenance["required"])
    assert {
        "invented_public_example",
        "user_private_basis",
        "protected_suspected",
    } <= set(provenance["properties"]["source_type"]["enum"])
    assert "completed_no_protected_content" in provenance["properties"][
        "protected_content_review"
    ]["enum"]

    checksum = definition(schema, "Checksum")
    assert REQUIRED_CHECKSUM <= set(checksum["required"])
    assert {"JCS", "NONE", "TBD"} <= set(
        checksum["properties"]["canonicalization"]["enum"]
    )
    assert "CALLER_SUPPLIED_JCS_BYTES_UNVERIFIED" in checksum["properties"][
        "canonicalization"
    ]["enum"]
    assert {
        "rule_pack_payload",
        "formula_declaration",
        "value_slot_declaration",
        "non_json_asset_manifest",
        "TBD",
    } <= set(checksum["properties"]["payload_scope"]["enum"])
    assert "deferred_to_DEL_06_04" in checksum["properties"][
        "verification_status"
    ]["enum"]
    checksum_set = definition(schema, "ChecksumSet")
    assert {
        "rule_pack_checksum",
        "payload_checksums",
        "checksum_lifecycle_status",
        "hash_basis",
    } <= set(checksum_set["required"])

    required_input = definition(schema, "RequiredInput")
    assert {
        "quantity_intent",
        "completeness_status",
        "missing_value_diagnostic",
        "provenance",
        "redistribution_status",
        "provenance_required",
        "redistribution_status_required",
    } <= set(required_input["required"])
    assert required_input["properties"]["provenance_required"]["const"] is True
    assert required_input["properties"]["redistribution_status_required"][
        "const"
    ] is True

    formula = definition(schema, "FormulaDeclaration")
    assert formula["properties"]["arbitrary_code_execution_allowed"]["const"] is False
    assert {
        "declarative_ast",
        "symbolic_reference",
        "structured_expression",
    } <= set(formula["properties"]["declaration_form"]["enum"])
    assert {
        "declaration_payload",
        "completeness_status",
    } <= set(formula["required"])
    assert formula["properties"]["input_refs"]["minItems"] == 1
    assert "no_protected_text_tables_or_copied_formulas" in formula["properties"][
        "protected_content_policy"
    ]["enum"]
    assert formula["properties"]["output_dimension"]["$ref"] == "#/$defs/QuantityIntent"
    payload = definition(schema, "FormulaDeclarationPayload")
    assert {
        "payload_kind",
        "payload_summary",
        "grammar_status",
        "arbitrary_code_execution_allowed",
        "protected_content_policy",
    } <= set(payload["required"])
    assert payload["properties"]["arbitrary_code_execution_allowed"]["const"] is False
    assert {
        FROZEN_GRAMMAR_STATUS,
        "grammar_not_selected",
        "future_human_approved_grammar_required",
        "TBD",
    } <= set(payload["properties"]["grammar_status"]["enum"])
    assert payload["properties"]["expression_ast"]["$ref"] == "#/$defs/ExpressionNode"
    declarative_gate = payload["allOf"][0]
    assert declarative_gate["if"]["properties"]["payload_kind"]["const"] == (
        "declarative_ast"
    )
    assert declarative_gate["then"]["required"] == ["expression_ast"]
    assert declarative_gate["then"]["properties"]["grammar_status"]["const"] == (
        FROZEN_GRAMMAR_STATUS
    )
    formula_gate = formula["allOf"][0]
    assert formula_gate["if"]["properties"]["declaration_form"]["const"] == (
        "declarative_ast"
    )
    assert formula_gate["then"]["properties"]["expression_language"]["const"] == (
        FROZEN_EXPRESSION_LANGUAGE
    )

    expression_node = definition(schema, "ExpressionNode")
    node_kinds = set()
    for branch in expression_node["oneOf"]:
        node_kinds.add(branch["properties"]["node"]["const"])
        assert branch["additionalProperties"] is False
    assert node_kinds == EXPRESSION_NODE_KINDS
    assert not (node_kinds & EVALUATOR_REFUSAL_MARKERS)
    aggregate_branch = next(
        branch
        for branch in expression_node["oneOf"]
        if branch["properties"]["node"]["const"] == "aggregate"
    )
    assert aggregate_branch["properties"]["operands"]["minItems"] == 1

    expression_quantity = definition(schema, "ExpressionQuantity")
    assert set(expression_quantity["required"]) == {"value", "dimension", "unit_ref"}
    assert expression_quantity["properties"]["dimension"]["$ref"] == (
        "#/$defs/DimensionId"
    )
    assert "unit_required" not in expression_quantity["properties"]
    assert "dimension_check_required" not in expression_quantity["properties"]

    user_table = definition(schema, "UserTableValue")
    assert REQUIRED_USER_TABLE <= set(user_table["required"])
    assert user_table["properties"]["rows"]["minItems"] == 1
    table_row = user_table["properties"]["rows"]["items"]
    assert set(table_row["required"]) == {"argument", "result"}

    quantity_intent = definition(schema, "QuantityIntent")
    assert {
        "dimension",
        "unit_ref",
        "unit_required",
        "dimension_check_required",
    } <= set(quantity_intent["required"])
    assert quantity_intent["properties"]["dimension"]["$ref"] == "#/$defs/DimensionId"
    dimensions = definition(schema, "DimensionId")["enum"]
    assert dimensions == CANONICAL_DIMENSIONS
    assert not (set(dimensions) & RETIRED_DIMENSIONS)

    value_slot = definition(schema, "UserSuppliedValueSlot")
    assert {
        "quantity_intent",
        "value_status",
        "missing_value_diagnostic",
        "provenance",
        "redistribution_status",
        "provenance_required",
        "redistribution_status_required",
        "review_status",
        "completeness_status",
    } <= set(value_slot["required"])
    assert value_slot["properties"]["provenance_required"]["const"] is True
    assert value_slot["properties"]["redistribution_status_required"]["const"] is True

    check = definition(schema, "CheckDefinition")
    assert {"RULE_INPUTS_INCOMPLETE", "USER_RULE_CHECKED", "USER_RULE_FAILED"} <= set(
        definition(schema, "AnalysisStatus")["enum"]
    )
    assert "HUMAN_APPROVED_FOR_PROJECT" not in definition(schema, "AnalysisStatus")[
        "enum"
    ]
    assert {"acceptability_basis", "result_statuses", "diagnostic_policy"} <= set(
        check["required"]
    )
    assert "value_slot_refs" in check["required"]
    assert check["properties"]["required_input_refs"]["minItems"] == 1
    assert check["properties"]["value_slot_refs"]["minItems"] == 1
    diagnostic_policy = definition(schema, "DiagnosticPolicy")
    assert REQUIRED_DIAGNOSTIC_POLICY <= set(diagnostic_policy["required"])

    diagnostic_codes = set(definition(schema, "RulePackDiagnosticCode")["enum"])
    assert {
        "RULE_CHECK_BLOCKING",
        "RULE_INPUT_MISSING",
        "RULE_UNIT_MISMATCH",
        "RULE_PROVENANCE_WARNING",
        "RULE_REDISTRIBUTION_WARNING",
        "RULE_CHECKSUM_MISMATCH",
        "RULE_PROTECTED_CONTENT_WARNING",
        "RULE_EVALUATOR_ERROR",
        "RULE_INCOMPLETE_DATA",
        "RULE_PROFESSIONAL_BOUNDARY_NOTICE",
    } <= diagnostic_codes
    diagnostic = definition(schema, "RulePackDiagnostic")
    assert {
        "diagnostic_class",
        "blocks_rule_check",
    } <= set(diagnostic["required"])
    assert {
        "rule_check_blocking",
        "checksum_mismatch",
        "professional_boundary_notice",
    } <= set(definition(schema, "RulePackDiagnosticClass")["enum"])

    professional_boundary = definition(schema, "ProfessionalBoundary")
    assert REQUIRED_PROFESSIONAL_BOUNDARY <= set(professional_boundary["required"])
    assert (
        professional_boundary["properties"]["software_makes_compliance_claim"][
            "const"
        ]
        is False
    )
    assert professional_boundary["properties"]["human_review_required"]["const"] is True
    assert (
        professional_boundary["properties"][
            "human_acceptance_record_software_generated"
        ]["const"]
        is False
    )
    assert professional_boundary["properties"]["software_makes_approval_claim"][
        "const"
    ] is False
    assert professional_boundary["properties"][
        "software_makes_authentication_claim"
    ]["const"] is False
    assert professional_boundary["properties"][
        "hash_bound_human_acceptance_required"
    ]["const"] is True

    open_decision_topics = set(
        definition(schema, "OpenDecision")["properties"]["topic"]["enum"]
    )
    assert {
        "expression_grammar",
        "evaluator_library",
        "private_encryption_default",
        "storage_container",
        "checksum_library",
    } <= open_decision_topics

    all_text = "\n".join(walk_strings(schema))
    for forbidden in FORBIDDEN_SCHEMA_TEXT:
        assert forbidden not in all_text


def check_invented_example_shape():
    schema = load_schema()
    example = load_example()
    for property_name in MIN_ITEM_ARRAYS:
        assert example[property_name], property_name
    output_dimension = example["formula_declarations"][0]["output_dimension"]
    assert required_at(schema, "QuantityIntent") <= set(output_dimension)
    assert output_dimension["dimension"] == "dimensionless"
    assert output_dimension["unit_ref"] == "ratio"
    assert output_dimension["unit_required"] is True
    assert output_dimension["dimension_check_required"] is True

    for required_input in example["required_inputs"]:
        assert required_at(schema, "RequiredInput") <= set(required_input)
        assert required_at(schema, "ProvenanceRecord") <= set(
            required_input["provenance"]
        )
        assert required_input["redistribution_status"] == (
            required_input["provenance"]["redistribution_status"]
        )

    assert example["grammar_version"] == "1.0.0"

    formula = example["formula_declarations"][0]
    assert required_at(schema, "FormulaDeclaration") <= set(formula)
    assert formula["declaration_form"] == "declarative_ast"
    assert formula["expression_language"] == FROZEN_EXPRESSION_LANGUAGE
    assert required_at(schema, "FormulaDeclarationPayload") <= set(
        formula["declaration_payload"]
    )
    assert formula["declaration_payload"]["grammar_status"] == FROZEN_GRAMMAR_STATUS
    expression_ast = formula["declaration_payload"]["expression_ast"]
    assert expression_ast["node"] == "binary"
    assert expression_ast["operator"] == "divide"
    assert expression_ast["left"]["variable_id"] == "demo_actual_quantity"
    assert expression_ast["right"]["variable_id"] == "demo_limit_quantity"
    assert formula["declaration_payload"]["arbitrary_code_execution_allowed"] is False
    assert formula["arbitrary_code_execution_allowed"] is False
    assert formula["completeness_status"] == "complete"

    value_slot = example["value_slots"][0]
    assert required_at(schema, "UserSuppliedValueSlot") <= set(value_slot)
    assert value_slot["provenance_required"] is True
    assert value_slot["redistribution_status_required"] is True

    check = example["check_definitions"][0]
    assert check["value_slot_refs"]
    assert REQUIRED_DIAGNOSTIC_POLICY <= set(check["diagnostic_policy"])

    checksum = example["checksums"]["rule_pack_checksum"]
    assert REQUIRED_CHECKSUM <= set(checksum)
    assert checksum["verification_status"] == "deferred_to_DEL_06_04"
    assert example["checksums"]["checksum_lifecycle_status"] == "deferred_to_DEL_06_04"

    diagnostic = example["diagnostics"][0]
    assert required_at(schema, "RulePackDiagnostic") <= set(diagnostic)
    assert diagnostic["diagnostic_class"] == "professional_boundary_notice"
    assert diagnostic["blocks_rule_check"] is False

    professional_boundary = example["professional_boundary"]
    assert REQUIRED_PROFESSIONAL_BOUNDARY <= set(professional_boundary)
    assert professional_boundary["software_makes_approval_claim"] is False
    assert professional_boundary["software_makes_authentication_claim"] is False
    assert professional_boundary["hash_bound_human_acceptance_required"] is True

    open_topics = {item["topic"] for item in example["open_decisions"]}
    assert {
        "expression_grammar",
        "evaluator_library",
        "private_encryption_default",
        "storage_container",
        "checksum_library",
        "result_envelope_integration",
    } <= open_topics


def check_jsonschema_validation():
    schema = load_schema()
    example = load_example()
    try:
        from jsonschema import Draft202012Validator
    except ModuleNotFoundError as exc:
        _skip_or_note_missing_jsonschema(exc)
        return

    Draft202012Validator.check_schema(schema)
    validator = Draft202012Validator(schema)
    errors = sorted(validator.iter_errors(example), key=lambda error: list(error.path))
    if errors:
        formatted = "\n".join(_format_error(error) for error in errors[:10])
        remaining = len(errors) - 10
        suffix = f"\n... {remaining} more validation errors" if remaining > 0 else ""
        raise AssertionError(f"{EXAMPLE_PATH} failed JSON Schema validation:\n{formatted}{suffix}")


def _validator_or_skip(schema):
    try:
        from jsonschema import Draft202012Validator
    except ModuleNotFoundError as exc:
        _skip_or_note_missing_jsonschema(exc)
        return None

    Draft202012Validator.check_schema(schema)
    return Draft202012Validator(schema)


def _assert_invalid_instance(validator, instance, expected_path):
    errors = sorted(validator.iter_errors(instance), key=lambda error: list(error.path))
    if not errors:
        raise AssertionError("instance unexpectedly passed JSON Schema validation")
    paths = {_error_path(error) for error in errors}
    assert expected_path in paths, paths


def _error_path(error):
    path = "$"
    for part in error.path:
        if isinstance(part, int):
            path += f"[{part}]"
        else:
            path += f".{part}"
    return path


def _format_error(error):
    path = "$"
    for part in error.path:
        if isinstance(part, int):
            path += f"[{part}]"
        else:
            path += f".{part}"
    return f"{path}: {error.message}"


def _skip_or_note_missing_jsonschema(exc):
    message = (
        "jsonschema>=4,<5 is required for full rule-pack JSON Schema validation; "
        "install with: python3 -m pip install -r requirements-dev.txt"
    )
    if "pytest" in sys.modules:
        import pytest

        pytest.skip(f"{message}; import error: {exc}")
    print(f"SKIP: {message}; import error: {exc}")


def test_rule_pack_schema_contract():
    check_schema_contract()


def test_invented_demo_uses_unit_bearing_formula_output_metadata():
    check_invented_example_shape()


def test_invented_demo_validates_against_rule_pack_schema():
    check_jsonschema_validation()


def test_schema_rejects_missing_or_unsafe_hardened_fields():
    schema = load_schema()
    example = load_example()
    validator = _validator_or_skip(schema)
    if validator is None:
        return

    missing_required_input_provenance = deepcopy(example)
    del missing_required_input_provenance["required_inputs"][0]["provenance"]
    _assert_invalid_instance(
        validator,
        missing_required_input_provenance,
        "$.required_inputs[0]",
    )

    executable_formula = deepcopy(example)
    executable_formula["formula_declarations"][0][
        "arbitrary_code_execution_allowed"
    ] = True
    _assert_invalid_instance(
        validator,
        executable_formula,
        "$.formula_declarations[0].arbitrary_code_execution_allowed",
    )

    missing_checksum_scope = deepcopy(example)
    del missing_checksum_scope["checksums"]["rule_pack_checksum"]["payload_scope"]
    _assert_invalid_instance(
        validator,
        missing_checksum_scope,
        "$.checksums.rule_pack_checksum",
    )

    software_approval_claim = deepcopy(example)
    software_approval_claim["professional_boundary"][
        "software_makes_approval_claim"
    ] = True
    _assert_invalid_instance(
        validator,
        software_approval_claim,
        "$.professional_boundary.software_makes_approval_claim",
    )

    missing_grammar_version = deepcopy(example)
    del missing_grammar_version["grammar_version"]
    _assert_invalid_instance(validator, missing_grammar_version, "$")

    malformed_grammar_version = deepcopy(example)
    malformed_grammar_version["grammar_version"] = "v1"
    _assert_invalid_instance(
        validator,
        malformed_grammar_version,
        "$.grammar_version",
    )

    declarative_without_ast = deepcopy(example)
    del declarative_without_ast["formula_declarations"][0]["declaration_payload"][
        "expression_ast"
    ]
    _assert_invalid_instance(
        validator,
        declarative_without_ast,
        "$.formula_declarations[0].declaration_payload",
    )

    stale_grammar_status = deepcopy(example)
    stale_grammar_status["formula_declarations"][0]["declaration_payload"][
        "grammar_status"
    ] = "grammar_not_selected"
    _assert_invalid_instance(
        validator,
        stale_grammar_status,
        "$.formula_declarations[0].declaration_payload.grammar_status",
    )

    refusal_marker_authored = deepcopy(example)
    refusal_marker_authored["formula_declarations"][0]["declaration_payload"][
        "expression_ast"
    ] = {"node": "unsafe_host_access", "request": "filesystem"}
    _assert_invalid_instance(
        validator,
        refusal_marker_authored,
        "$.formula_declarations[0].declaration_payload.expression_ast",
    )

    relaxed_literal_flags = deepcopy(example)
    relaxed_literal_flags["formula_declarations"][0]["declaration_payload"][
        "expression_ast"
    ] = {
        "node": "literal",
        "quantity": {
            "value": 1.0,
            "dimension": "dimensionless",
            "unit_ref": "ratio",
            "unit_required": False,
        },
    }
    _assert_invalid_instance(
        validator,
        relaxed_literal_flags,
        "$.formula_declarations[0].declaration_payload.expression_ast",
    )

    malformed_table_row = deepcopy(example)
    malformed_table_row["formula_declarations"][0]["declaration_payload"][
        "expression_ast"
    ] = {
        "node": "interpolate",
        "table": {
            "table_id": "demo_invented_table",
            "argument_dimension": "dimensionless",
            "argument_unit_ref": "ratio",
            "result_dimension": "dimensionless",
            "result_unit_ref": "ratio",
            "rows": [{"argument": 1.0}],
        },
        "argument": {
            "node": "variable_ref",
            "variable_id": "demo_actual_quantity",
        },
    }
    _assert_invalid_instance(
        validator,
        malformed_table_row,
        "$.formula_declarations[0].declaration_payload.expression_ast",
    )


def main():
    check_schema_contract()
    check_invented_example_shape()
    check_jsonschema_validation()


if __name__ == "__main__":
    main()
