#!/usr/bin/env python3
"""Stdlib checks for the section and component library schemas."""

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

SECTION_SCHEMA_PATH = ROOT / "schemas" / "section.schema.yaml"
COMPONENT_SCHEMA_PATH = ROOT / "schemas" / "component.schema.yaml"
MODEL_SCHEMA_PATH = ROOT / "schemas" / "model.schema.yaml"
UNITS_SCHEMA_PATH = ROOT / "schemas" / "units.schema.yaml"
COMBINED_FIXTURE_PATH = (
    ROOT / "fixtures" / "component" / "invented_section_component_library_valid.json"
)
SECTION_FIXTURE_PATH = ROOT / "fixtures" / "component" / "invented_section_library_valid.json"
COMPONENT_FIXTURE_PATH = (
    ROOT / "fixtures" / "component" / "invented_component_library_valid.json"
)

RETIRED_DIMENSIONS = {
    "temperature_difference",
    "area_moment",
    "stiffness",
    "reference",
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
    "CODE_COMPLIANT",
    "certified component",
    "certified section",
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
    section_schema = load_json(SECTION_SCHEMA_PATH)
    component_schema = load_json(COMPONENT_SCHEMA_PATH)
    model_schema = load_json(MODEL_SCHEMA_PATH)
    units_schema = load_json(UNITS_SCHEMA_PATH)
    combined_fixture = load_json(COMBINED_FIXTURE_PATH)
    section_fixture = load_json(SECTION_FIXTURE_PATH)
    component_fixture = load_json(COMPONENT_FIXTURE_PATH)
    accepted_dimensions = set(units_schema["$defs"]["DimensionId"]["enum"])

    for schema in (section_schema, component_schema):
        assert schema["$schema"] == "https://json-schema.org/draft/2020-12/schema"
        assert schema["additionalProperties"] is False
        assert "default" not in set(walk_keys(schema))
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
        "schema_version",
        "section_library",
        "section_records",
        "dimension_definitions",
        "property_definitions",
        "completeness_rules",
        "diagnostics",
        "open_decisions",
    } <= set(section_schema["required"])
    assert {
        "section_id",
        "name",
        "section_type",
        "privacy_class",
        "redistribution_status",
        "dimensions",
        "properties",
        "completeness",
        "provenance",
        "review_status",
    } <= required_at(section_schema, "SectionRecord")
    assert {
        "outside_diameter",
        "wall_thickness",
        "corrosion_allowance",
        "nominal_size_label",
        "user_defined",
        "TBD",
    } <= enum_at(section_schema, "SectionDimensionKind")
    assert {
        "cross_section_area",
        "inside_diameter",
        "moment_of_inertia",
        "section_modulus",
        "mass_per_length",
        "user_defined",
        "TBD",
    } <= enum_at(section_schema, "SectionPropertyKind")
    section_dimensions = enum_at(section_schema, "SectionDimension")
    assert {
        "second_moment_area",
        "section_modulus",
        "volume_per_length",
        "mass_per_length",
    } <= section_dimensions
    assert section_dimensions <= accepted_dimensions
    assert not (section_dimensions & RETIRED_DIMENSIONS)
    assert {
        "SECTION_DIMENSION_MISSING",
        "SECTION_UNIT_MISSING",
        "SECTION_PROVENANCE_MISSING",
        "SECTION_PROTECTED_CONTENT_SUSPECTED",
        "SECTION_CALCULATION_INPUT_INVALID",
        "SECTION_DIMENSION_INCONSISTENT",
        "SECTION_CATALOG_VALUE_NOT_PUBLIC",
    } <= enum_at(section_schema, "SectionDiagnosticCode")
    assert {"class", "source"} <= required_at(section_schema, "SectionDiagnostic")

    assert {
        "schema_version",
        "component_library",
        "component_family_contracts",
        "component_records",
        "field_definitions",
        "completeness_rules",
        "diagnostics",
        "open_decisions",
    } <= set(component_schema["required"])
    assert {
        "component_id",
        "name",
        "component_type",
        "privacy_class",
        "redistribution_status",
        "fields",
        "completeness",
        "provenance",
        "review_status",
    } <= required_at(component_schema, "ComponentRecord")
    assert {
        "bend",
        "elbow",
        "branch",
        "reducer",
        "valve",
        "flange",
        "expansion_joint",
        "rigid",
        "specialty",
        "other",
        "TBD",
    } <= enum_at(component_schema, "ComponentType")
    model_component_types = set(
        model_schema["$defs"]["Component"]["properties"]["component_type"]["enum"]
    )
    assert enum_at(component_schema, "ComponentType") == model_component_types
    assert {
        "bend_centerline_radius",
        "bend_included_angle",
        "bend_plane_orientation",
        "bend_geometry_source_reference",
        "branch_run_size",
        "branch_header_size",
        "branch_connection_angle",
        "branch_connection_type",
        "branch_reinforcement_area",
        "branch_reinforcement_reference",
        "branch_geometry_source_reference",
        "rigid_body_length",
        "connection_end_a_reference",
        "connection_end_b_reference",
        "stiffness_behavior_reference",
        "rigid_component_source_reference",
        "weight",
        "center_of_gravity",
        "linear_stiffness",
        "rotational_stiffness",
        "effective_area",
        "movement_limit",
        "sif_user_value",
        "flexibility_factor_user_value",
        "manufacturer_reference",
        "TBD",
    } <= enum_at(component_schema, "ComponentFieldKind")
    component_dimensions = enum_at(component_schema, "ComponentQuantityDimension")
    assert {"linear_stiffness", "rotational_stiffness"} <= component_dimensions
    assert component_dimensions <= accepted_dimensions
    assert not (component_dimensions & RETIRED_DIMENSIONS)
    assert {
        "COMPONENT_FIELD_MISSING",
        "COMPONENT_UNIT_MISSING",
        "COMPONENT_PROVENANCE_MISSING",
        "COMPONENT_PROTECTED_CONTENT_SUSPECTED",
        "COMPONENT_MODIFIER_NOT_PUBLIC",
        "BEND_GEOMETRY_INCOMPLETE",
        "BEND_RULE_INPUT_MISSING",
        "BRANCH_GEOMETRY_INCOMPLETE",
        "BRANCH_REINFORCEMENT_DATA_MISSING",
        "BRANCH_RULE_INPUT_MISSING",
        "RIGID_COMPONENT_GEOMETRY_INCOMPLETE",
        "RIGID_COMPONENT_MASS_DATA_MISSING",
        "RIGID_COMPONENT_STIFFNESS_DATA_MISSING",
        "RIGID_COMPONENT_CATALOG_VALUE_NOT_PUBLIC",
        "EXPANSION_JOINT_STIFFNESS_DATA_MISSING",
        "EXPANSION_JOINT_EFFECTIVE_AREA_MISSING",
        "EXPANSION_JOINT_MOVEMENT_LIMIT_MISSING",
        "EXPANSION_JOINT_HARDWARE_DATA_MISSING",
        "EXPANSION_JOINT_MANUFACTURER_VALUE_NOT_PUBLIC",
    } <= enum_at(component_schema, "ComponentDiagnosticCode")
    assert {"class", "source"} <= required_at(component_schema, "ComponentDiagnostic")
    assert {
        "contract_id",
        "component_types",
        "geometry_field_kinds",
        "rule_modifier_field_kinds",
        "source_metadata_field_kinds",
        "mechanics_interface",
        "protected_value_policy",
        "review_status",
    } <= required_at(component_schema, "ComponentFamilyContract")

    assert (
        combined_fixture["strict_section_fixture_ref"]
        == "fixtures/component/invented_section_library_valid.json"
    )
    assert (
        combined_fixture["strict_component_fixture_ref"]
        == "fixtures/component/invented_component_library_valid.json"
    )

    assert section_fixture["section_library"]["library_scope"] == "public_schema_fixture"
    assert section_fixture["section_records"][0]["redistribution_status"] == "TBD"
    assert section_fixture["section_records"][0]["dimensions"][0]["value_status"] == "missing"
    assert section_fixture["section_records"][0]["completeness"][0]["status"] == "incomplete"
    assert section_fixture["diagnostics"][0]["code"] == "SECTION_DIMENSION_MISSING"
    assert section_fixture["diagnostics"][0]["class"] == "SOLVE_BLOCKING"
    assert section_fixture["diagnostics"][0]["source"] == (
        "fixtures/component/invented_section_library_valid.json"
    )

    assert component_fixture["component_library"]["library_scope"] == "public_schema_fixture"
    bend_contract = component_fixture["component_family_contracts"][0]
    assert {"bend", "elbow"} <= set(bend_contract["component_types"])
    assert "bend_centerline_radius" in bend_contract["geometry_field_kinds"]
    assert "sif_user_value" in bend_contract["rule_modifier_field_kinds"]
    assert bend_contract["protected_value_policy"] == "schema_slots_only"
    branch_contract = component_fixture["component_family_contracts"][1]
    assert branch_contract["component_types"] == ["branch"]
    assert "branch_run_size" in branch_contract["geometry_field_kinds"]
    assert "branch_reinforcement_reference" in branch_contract["source_metadata_field_kinds"]
    assert "sif_user_value" in branch_contract["rule_modifier_field_kinds"]
    assert branch_contract["protected_value_policy"] == "schema_slots_only"
    rigid_contract = component_fixture["component_family_contracts"][2]
    assert {"valve", "flange", "reducer", "rigid", "specialty"} <= set(
        rigid_contract["component_types"]
    )
    assert "rigid_body_length" in rigid_contract["geometry_field_kinds"]
    assert "connection_end_a_reference" in rigid_contract["geometry_field_kinds"]
    assert "weight" in rigid_contract["rule_modifier_field_kinds"]
    assert "center_of_gravity" in rigid_contract["rule_modifier_field_kinds"]
    assert "linear_stiffness" in rigid_contract["rule_modifier_field_kinds"]
    assert "rotational_stiffness" in rigid_contract["rule_modifier_field_kinds"]
    assert rigid_contract["protected_value_policy"] == "schema_slots_only"
    expansion_contract = component_fixture["component_family_contracts"][3]
    assert expansion_contract["component_types"] == ["expansion_joint"]
    assert "effective_area" in expansion_contract["geometry_field_kinds"]
    assert "movement_limit" in expansion_contract["geometry_field_kinds"]
    assert "linear_stiffness" in expansion_contract["rule_modifier_field_kinds"]
    assert "rotational_stiffness" in expansion_contract["rule_modifier_field_kinds"]
    assert "hardware_reference" in expansion_contract["rule_modifier_field_kinds"]
    assert "manufacturer_reference" in expansion_contract["source_metadata_field_kinds"]
    assert expansion_contract["protected_value_policy"] == "schema_slots_only"
    assert component_fixture["component_records"][0]["redistribution_status"] == "TBD"
    fixture_field_kinds = {
        field["field_kind"] for field in component_fixture["component_records"][0]["fields"]
    }
    assert "bend_centerline_radius" in fixture_field_kinds
    assert "bend_included_angle" in fixture_field_kinds
    assert (
        component_fixture["component_records"][0]["fields"][0][
            "public_repository_value_policy"
        ]
        == "schema_shape_only"
    )
    assert component_fixture["component_records"][0]["completeness"][0]["status"] == "incomplete"
    assert component_fixture["diagnostics"][0]["code"] == "BEND_GEOMETRY_INCOMPLETE"
    assert component_fixture["diagnostics"][0]["class"] == "SOLVE_BLOCKING"
    branch_record = component_fixture["component_records"][1]
    assert branch_record["component_type"] == "branch"
    branch_field_kinds = {field["field_kind"] for field in branch_record["fields"]}
    assert "branch_run_size" in branch_field_kinds
    assert "branch_header_size" in branch_field_kinds
    assert "branch_connection_angle" in branch_field_kinds
    assert "branch_reinforcement_reference" in branch_field_kinds
    assert "sif_user_value" in branch_field_kinds
    assert branch_record["fields"][-1]["public_repository_value_policy"] == "no_public_code_specific_values"
    assert branch_record["completeness"][0]["status"] == "incomplete"
    assert branch_record["completeness"][0]["diagnostic_code"] == "BRANCH_RULE_INPUT_MISSING"
    assert component_fixture["diagnostics"][1]["code"] == "BRANCH_RULE_INPUT_MISSING"
    rigid_record = component_fixture["component_records"][2]
    assert rigid_record["component_type"] == "rigid"
    rigid_field_kinds = {field["field_kind"] for field in rigid_record["fields"]}
    assert "rigid_body_length" in rigid_field_kinds
    assert "connection_end_a_reference" in rigid_field_kinds
    assert "weight" in rigid_field_kinds
    assert "center_of_gravity" in rigid_field_kinds
    assert "linear_stiffness" in rigid_field_kinds
    assert "rotational_stiffness" in rigid_field_kinds
    assert (
        rigid_record["fields"][2]["public_repository_value_policy"]
        == "no_public_proprietary_catalog_values"
    )
    assert rigid_record["completeness"][0]["status"] == "incomplete"
    assert (
        rigid_record["completeness"][0]["diagnostic_code"]
        == "RIGID_COMPONENT_GEOMETRY_INCOMPLETE"
    )
    assert component_fixture["diagnostics"][2]["code"] == "RIGID_COMPONENT_GEOMETRY_INCOMPLETE"
    expansion_record = component_fixture["component_records"][3]
    assert expansion_record["component_type"] == "expansion_joint"
    expansion_field_kinds = {field["field_kind"] for field in expansion_record["fields"]}
    assert "linear_stiffness" in expansion_field_kinds
    assert "rotational_stiffness" in expansion_field_kinds
    assert "effective_area" in expansion_field_kinds
    assert "movement_limit" in expansion_field_kinds
    assert "hardware_reference" in expansion_field_kinds
    assert "manufacturer_reference" in expansion_field_kinds
    assert (
        expansion_record["fields"][0]["public_repository_value_policy"]
        == "private_user_supplied_only"
    )
    assert expansion_record["completeness"][0]["status"] == "incomplete"
    assert (
        expansion_record["completeness"][0]["diagnostic_code"]
        == "EXPANSION_JOINT_STIFFNESS_DATA_MISSING"
    )
    assert (
        component_fixture["diagnostics"][3]["code"]
        == "EXPANSION_JOINT_STIFFNESS_DATA_MISSING"
    )

    all_text = "\n".join(
        [
            *walk_strings(section_schema),
            *walk_strings(component_schema),
            *walk_strings(combined_fixture),
            *walk_strings(section_fixture),
            *walk_strings(component_fixture),
        ]
    )
    for forbidden in FORBIDDEN_PUBLIC_DATA_TEXT:
        assert forbidden not in all_text


def check_jsonschema_validation():
    section_schema = load_json(SECTION_SCHEMA_PATH)
    component_schema = load_json(COMPONENT_SCHEMA_PATH)
    section_fixture = load_json(SECTION_FIXTURE_PATH)
    component_fixture = load_json(COMPONENT_FIXTURE_PATH)
    try:
        assert validate_schema_document(section_schema, schema_label=str(SECTION_SCHEMA_PATH))
        assert validate_schema_document(
            component_schema,
            schema_label=str(COMPONENT_SCHEMA_PATH),
        )
        assert validate_instance(
            section_schema,
            section_fixture,
            schema_label=str(SECTION_SCHEMA_PATH),
            instance_label=str(SECTION_FIXTURE_PATH),
        )
        assert validate_instance(
            component_schema,
            component_fixture,
            schema_label=str(COMPONENT_SCHEMA_PATH),
            instance_label=str(COMPONENT_FIXTURE_PATH),
        )
    except JsonSchemaDependencyMissing as exc:
        _skip_or_note_missing_jsonschema(exc)


def _skip_or_note_missing_jsonschema(exc):
    if "pytest" in sys.modules:
        import pytest

        pytest.skip(str(exc))
    print(f"SKIP: {exc}")


def test_component_section_schema_contract():
    main()


def test_component_section_jsonschema_validation_helper():
    check_jsonschema_validation()


if __name__ == "__main__":
    main()
    check_jsonschema_validation()
