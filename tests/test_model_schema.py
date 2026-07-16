#!/usr/bin/env python3
"""Stdlib checks for the canonical domain model schema."""

import json
import sys
from copy import deepcopy
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
TESTS_DIR = Path(__file__).resolve().parent
if str(TESTS_DIR) not in sys.path:
    sys.path.insert(0, str(TESTS_DIR))

from schema_validation import (  # noqa: E402
    JsonSchemaDependencyMissing,
    schema_for_definition,
    validate_instance,
    validate_schema_document,
)

SCHEMA_PATH = ROOT / "schemas" / "model.schema.yaml"
MINIMAL_PROJECT_FIXTURE = ROOT / "fixtures" / "domain" / "invented_minimal_project_model.json"
PHYSICAL_MODEL_FIXTURE = ROOT / "fixtures" / "domain" / "invented_physical_source_of_truth_model.json"

REQUIRED_DEFS = {
    "Assumption",
    "Project",
    "Model",
    "ModelRole",
    "Node",
    "Element",
    "Component",
    "Material",
    "Section",
    "Support",
    "LoadCase",
    "Combination",
    "RulePackRef",
    "Result",
    "ReportSettings",
    "Report",
    "Quantity",
    "Provenance",
    "Diagnostic",
    "Checksum",
    "Reference",
    "TraceabilityLink",
    "DirectionVector",
    "NodalForceLoadRecord",
    "NodalMomentLoadRecord",
    "ElementPointForceLoadRecord",
    "ElementUniformDistributedForceLoadRecord",
}

REQUIRED_ANALYSIS_STATUSES = {
    "MODEL_INCOMPLETE",
    "MECHANICS_SOLVED",
    "RULE_INPUTS_INCOMPLETE",
    "USER_RULE_CHECKED",
    "USER_RULE_FAILED",
    "HUMAN_REVIEW_REQUIRED",
    "HUMAN_APPROVED_FOR_PROJECT",
}

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

FORBIDDEN_SCHEMA_TEXT = {
    "CODE_COMPLIANT",
    "ASME",
    "B31",
    "certified",
    "sealed",
    "automatic compliance",
    "professional approval by the software",
}


def load_schema():
    with SCHEMA_PATH.open(encoding="utf-8") as schema_file:
        return json.load(schema_file)


def load_json(path):
    with path.open(encoding="utf-8") as fixture_file:
        return json.load(fixture_file)


def provenance():
    return {
        "source_name": "PKG-02 invented schema test",
        "source_location": "tests/test_model_schema.py",
        "source_license": "project invented public example",
        "contributor": "OpenPipeStress Type 2 worker",
        "contributor_certification": "invented non-engineering example",
        "redistribution_status": "public_permissive",
        "review_status": "accepted",
    }


def quantity(value, unit, dimension):
    return {
        "value": value,
        "unit": unit,
        "dimension": dimension,
        "provenance": provenance(),
    }


def reference(object_type, object_id):
    return {
        "object_type": object_type,
        "id": object_id,
    }


def normalized_minimal_fixture_for_current_schema():
    fixture = load_json(MINIMAL_PROJECT_FIXTURE)
    load = fixture["project"]["models"][0]["load_cases"][0]["loads"][0]
    load["load_record_type"] = "nodal_force"
    return fixture


def normalized_physical_fixture_for_current_schema():
    fixture = load_json(PHYSICAL_MODEL_FIXTURE)
    physical_model = fixture["model"]
    for element in physical_model["elements"]:
        if element["element_type"] == "straight_pipe":
            coordinate_system = element["local_coordinate_system"]
            coordinate_system["y_reference"] = [0.0, 1.0, 0.0]
            coordinate_system["provenance"] = physical_model["provenance"]
    loads = physical_model["load_cases"][0]["loads"]
    loads[0]["load_record_type"] = "element_uniform_distributed_force"
    loads[0]["quantity"]["unit"] = "N/m"
    loads[0]["quantity"]["dimension"] = "force_per_length"
    loads[0]["span"] = {
        "start_fraction": quantity(0.0, "1", "dimensionless"),
        "end_fraction": quantity(1.0, "1", "dimensionless"),
    }
    loads[1]["load_record_type"] = "element_point_force"
    loads[1]["station_fraction"] = quantity(0.5, "1", "dimensionless")
    loads[2]["load_record_type"] = "nodal_moment"
    loads[2]["target_ref"] = reference("Node", "N-1")
    return fixture


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


def assert_required_fields(schema, definition_name, record):
    assert required_at(schema, definition_name) <= set(record)


def check_schema_contract():
    schema = load_schema()
    defs = schema["$defs"]

    assert schema["$schema"] == "https://json-schema.org/draft/2020-12/schema"
    assert schema["additionalProperties"] is False
    assert "default" not in set(walk_keys(schema))
    assert {"schema_version", "project"} <= set(schema["required"])
    assert REQUIRED_DEFS <= set(defs)

    assert REQUIRED_ANALYSIS_STATUSES <= enum_at(schema, "AnalysisStatus")
    assert "CODE_COMPLIANT" not in enum_at(schema, "AnalysisStatus")

    assert {
        "id",
        "name",
        "unit_system",
        "privacy_class",
        "storage_policy",
        "models",
        "rule_pack_refs",
        "report_settings",
        "reports",
        "diagnostics",
        "hashes",
    } <= required_at(schema, "Project")

    assert {
        "model_role",
        "nodes",
        "elements",
        "components",
        "materials",
        "sections",
        "supports",
        "load_cases",
        "combinations",
        "results",
        "diagnostics",
        "unresolved_assumptions",
        "traceability_links",
        "design_knowledge_refs",
        "constraint_refs",
        "equipment_interface_refs",
        "operation_refs",
        "model_state_refs",
        "analysis_run_refs",
        "comparison_refs",
        "handoff_package_refs",
        "external_reference_refs",
        "provenance",
    } <= required_at(schema, "Model")
    assert {
        "physical_source_of_truth",
        "analytical_solver_model",
        "derived_view",
        "TBD",
    } <= enum_at(schema, "ModelRole")

    assert {"id", "statement", "status", "affected_refs", "provenance"} <= required_at(
        schema, "Assumption"
    )
    assert {"unresolved", "resolved", "rejected", "TBD"} <= set(
        defs["Assumption"]["properties"]["status"]["enum"]
    )

    assert {
        "id",
        "trace_type",
        "source_ref",
        "target_ref",
        "diagnostics",
        "provenance",
    } <= required_at(schema, "TraceabilityLink")
    assert {
        "physical_to_analytical",
        "operation_application",
        "state_snapshot",
        "analysis_run",
        "comparison",
        "handoff",
        "external_reference",
        "TBD",
    } <= set(defs["TraceabilityLink"]["properties"]["trace_type"]["enum"])

    assert {
        "DesignKnowledge",
        "Constraint",
        "EquipmentInterface",
        "ModelOperation",
        "ModelState",
        "AnalysisRun",
        "Comparison",
        "HandoffPackage",
        "ExternalProverReference",
        "Assumption",
        "TraceabilityLink",
    } <= set(defs["Reference"]["properties"]["object_type"]["enum"])

    assert {"value", "unit", "dimension", "provenance"} <= required_at(
        schema, "Quantity"
    )
    dimensions = defs["Quantity"]["properties"]["dimension"]["enum"]
    assert dimensions == CANONICAL_DIMENSIONS
    assert not (set(dimensions) & RETIRED_DIMENSIONS)

    coordinate_system = defs["CoordinateSystem"]
    assert coordinate_system["additionalProperties"] is False
    assert {"type", "axes"} <= set(coordinate_system["required"])
    assert {"y_reference", "provenance"} <= set(coordinate_system["properties"])
    assert "local_x_axis" not in coordinate_system["properties"]
    assert defs["DirectionVector"]["minItems"] == 3
    assert defs["DirectionVector"]["maxItems"] == 3

    element_contract_text = json.dumps(defs["Element"])
    assert '"const": "straight_pipe"' in element_contract_text
    assert '"y_reference"' in element_contract_text

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
    } <= set(defs["Component"]["properties"]["component_type"]["enum"])
    assert {
        "source_name",
        "source_location",
        "source_license",
        "contributor",
        "contributor_certification",
        "redistribution_status",
        "review_status",
    } <= required_at(schema, "Provenance")

    assert {
        "code",
        "class",
        "severity",
        "source",
        "affected_object",
        "message",
        "remediation",
        "provenance",
    } <= required_at(schema, "Diagnostic")
    assert {
        "SOLVE_BLOCKING",
        "RULE_CHECK_BLOCKING",
        "PROVENANCE_WARNING",
        "ASSUMPTION_WARNING",
        "NONLINEAR_WARNING",
        "IP_BOUNDARY_WARNING",
    } <= set(defs["Diagnostic"]["properties"]["class"]["enum"])

    assert {"algorithm", "canonicalization", "payload_ref", "value"} <= required_at(
        schema, "Checksum"
    )
    assert {"JCS", "NONE", "TBD"} <= set(
        defs["Checksum"]["properties"]["canonicalization"]["enum"]
    )

    assert [item["$ref"] for item in defs["LoadRecord"]["oneOf"]] == [
        "#/$defs/NodalForceLoadRecord",
        "#/$defs/NodalMomentLoadRecord",
        "#/$defs/ElementPointForceLoadRecord",
        "#/$defs/ElementUniformDistributedForceLoadRecord",
    ]
    for load_def in [
        "NodalForceLoadRecord",
        "NodalMomentLoadRecord",
        "ElementPointForceLoadRecord",
        "ElementUniformDistributedForceLoadRecord",
    ]:
        assert defs[load_def]["additionalProperties"] is False
        assert {
            "load_record_type",
            "target_ref",
            "direction",
            "quantity",
            "provenance",
        } <= required_at(schema, load_def)
    assert defs["NodalForceLoadRecord"]["properties"]["load_record_type"]["const"] == (
        "nodal_force"
    )
    assert defs["NodalMomentLoadRecord"]["properties"]["load_record_type"]["const"] == (
        "nodal_moment"
    )
    assert defs["ElementPointForceLoadRecord"]["properties"]["load_record_type"][
        "const"
    ] == "element_point_force"
    assert defs["ElementUniformDistributedForceLoadRecord"]["properties"][
        "load_record_type"
    ]["const"] == "element_uniform_distributed_force"
    # DEC-068 item 1 + DEC-077: exact selection remains available and an
    # explicit solve temperature enables bounded linear interpolation.
    assert "modulus_basis_ref" in defs["LoadCase"]["properties"]
    modulus_basis_ref = defs["LoadCase"]["properties"]["modulus_basis_ref"]
    assert "exact" in modulus_basis_ref["description"]
    assert "mutually exclusive" in modulus_basis_ref["description"]
    modulus_basis_temperature = defs["LoadCase"]["properties"][
        "modulus_basis_temperature"
    ]
    assert modulus_basis_temperature["$ref"] == "#/$defs/QuantityValue"
    assert "linear interpolation" in modulus_basis_temperature["description"]
    assert "extrapolation" in modulus_basis_temperature["description"]
    assert {
        "modulus_basis_ref",
        "modulus_basis_temperature",
    } == set(defs["LoadCase"]["allOf"][0]["not"]["required"])
    assert "modulus_basis_records" in defs["Combination"]["properties"]
    modulus_record = defs["ModulusBasisRecord"]
    assert modulus_record["additionalProperties"] is False
    assert {"load_case_ref", "modulus_basis_ref"} <= set(modulus_record["required"])

    # DEC-068 item 2: static-equivalent occasional-load generation inputs.
    assert "equivalent_static_generation" in defs["LoadCase"]["properties"]
    equivalent_static = defs["EquivalentStaticGeneration"]
    assert equivalent_static["additionalProperties"] is False
    assert {"provenance"} <= set(equivalent_static["required"])
    assert {"seismic", "wind", "provenance"} <= set(equivalent_static["properties"])
    seismic = defs["SeismicEquivalentStaticInput"]
    assert seismic["additionalProperties"] is False
    assert {"gravity_acceleration", "g_factors"} <= set(seismic["required"])
    g_factors = seismic["properties"]["g_factors"]
    assert g_factors["additionalProperties"] is False
    assert g_factors["minProperties"] == 1
    assert set(g_factors["properties"]) == {"x", "y", "z"}
    wind = defs["WindEquivalentStaticInput"]
    assert wind["additionalProperties"] is False
    assert {
        "pressure",
        "shape_factor",
        "direction",
        "exposed_element_refs",
    } <= set(wind["required"])
    assert wind["properties"]["exposed_element_refs"]["minItems"] == 1
    assert (
        defs["AccelerationQuantity"]["allOf"][1]["properties"]["dimension"]["const"]
        == "acceleration"
    )
    assert (
        defs["PressureQuantity"]["allOf"][1]["properties"]["dimension"]["const"]
        == "pressure"
    )
    assert (
        defs["DimensionlessQuantity"]["allOf"][1]["properties"]["dimension"]["const"]
        == "dimensionless"
    )
    assert defs["ForceDirection"]["enum"] == ["X", "Y", "Z"]
    assert defs["MomentDirection"]["enum"] == ["RX", "RY", "RZ"]
    assert defs["ForceQuantity"]["allOf"][1]["properties"]["dimension"]["const"] == "force"
    assert defs["MomentQuantity"]["allOf"][1]["properties"]["dimension"]["const"] == "moment"
    assert (
        defs["ForcePerLengthQuantity"]["allOf"][1]["properties"]["dimension"]["const"]
        == "force_per_length"
    )
    assert defs["FractionQuantity"]["allOf"][1]["properties"]["dimension"]["const"] == (
        "dimensionless"
    )
    assert defs["FractionQuantity"]["allOf"][1]["properties"]["value"] == {
        "minimum": 0,
        "maximum": 1,
    }
    assert {
        "id",
        "support_type",
        "target_ref",
        "directions",
        "properties",
        "provenance",
    } <= required_at(schema, "Support")

    assert {"diagnostics", "provenance"} <= required_at(schema, "Result")
    assert {
        "input_manifest",
        "hashes",
        "analysis_statuses",
        "diagnostics",
        "rule_pack_refs",
        "provenance_summary",
        "professional_boundary_notice",
    } <= required_at(schema, "Report")
    assert {"checksum", "source_notice", "required_input_refs"} <= required_at(
        schema, "RulePackRef"
    )

    all_text = "\n".join(walk_strings(schema))
    for forbidden in FORBIDDEN_SCHEMA_TEXT:
        assert forbidden not in all_text


def check_domain_fixtures():
    schema = load_schema()
    minimal = load_json(MINIMAL_PROJECT_FIXTURE)
    physical = load_json(PHYSICAL_MODEL_FIXTURE)

    assert set(minimal) == {"schema_version", "project"}
    assert minimal["schema_version"] == "0.1.0"
    project = minimal["project"]
    assert_required_fields(schema, "Project", project)
    assert project["privacy_class"] == "public"
    assert project["storage_policy"] == "public_example"
    assert project["models"]

    model = project["models"][0]
    assert_required_fields(schema, "Model", model)
    assert model["model_role"] == "analytical_solver_model"
    assert model["coordinate_system"]["type"] == "cartesian"
    assert model["load_cases"], "fixture must exercise unit-bearing load records"
    quantity = model["load_cases"][0]["loads"][0]["quantity"]
    assert_required_fields(schema, "Quantity", quantity)
    assert quantity["dimension"] == "force"
    assert_required_fields(schema, "Provenance", quantity["provenance"])
    assert_required_fields(schema, "ReportSettings", project["report_settings"])
    assert_required_fields(schema, "Checksum", project["hashes"][0])

    physical_model = physical["model"]
    assert_required_fields(schema, "Model", physical_model)
    assert physical_model["model_role"] == "physical_source_of_truth"
    assert physical_model["unresolved_assumptions"]
    assert_required_fields(schema, "Assumption", physical_model["unresolved_assumptions"][0])
    assert physical_model["traceability_links"]
    assert_required_fields(schema, "TraceabilityLink", physical_model["traceability_links"][0])
    assert physical_model["diagnostics"]
    assert_required_fields(schema, "Diagnostic", physical_model["diagnostics"][0])

    fixture_text = "\n".join(
        walk_strings({"minimal": minimal, "physical": physical})
    )
    for forbidden in FORBIDDEN_SCHEMA_TEXT:
        assert forbidden not in fixture_text


def check_jsonschema_validation():
    schema = load_schema()
    minimal = normalized_minimal_fixture_for_current_schema()
    physical = normalized_physical_fixture_for_current_schema()
    try:
        assert validate_schema_document(schema, schema_label=str(SCHEMA_PATH))
        assert validate_instance(
            schema,
            minimal,
            schema_label=str(SCHEMA_PATH),
            instance_label=str(MINIMAL_PROJECT_FIXTURE),
        )
        model_schema = schema_for_definition(schema, "Model")
        assert validate_instance(
            model_schema,
            physical["model"],
            schema_label=f"{SCHEMA_PATH}#/$defs/Model",
            instance_label=f"{PHYSICAL_MODEL_FIXTURE}:model",
        )
    except JsonSchemaDependencyMissing as exc:
        _skip_or_note_missing_jsonschema(exc)


def check_typed_load_records_and_orientation_validation():
    schema = load_schema()
    load_record_schema = schema_for_definition(schema, "LoadRecord")
    element_schema = schema_for_definition(schema, "Element")

    valid_nodal_force = {
        "load_record_type": "nodal_force",
        "target_ref": reference("Node", "N-1"),
        "direction": "X",
        "quantity": quantity(1.0, "N", "force"),
        "provenance": provenance(),
    }
    valid_nodal_moment = {
        "load_record_type": "nodal_moment",
        "target_ref": reference("Node", "N-1"),
        "direction": "RZ",
        "quantity": quantity(1.0, "N-m", "moment"),
        "provenance": provenance(),
    }
    valid_element_point_force = {
        "load_record_type": "element_point_force",
        "target_ref": reference("Element", "E-1"),
        "station_fraction": quantity(0.5, "1", "dimensionless"),
        "direction": "Z",
        "quantity": quantity(1.0, "N", "force"),
        "provenance": provenance(),
    }
    valid_uniform_force = {
        "load_record_type": "element_uniform_distributed_force",
        "target_ref": reference("Element", "E-1"),
        "span": {
            "start_fraction": quantity(0.0, "1", "dimensionless"),
            "end_fraction": quantity(1.0, "1", "dimensionless"),
        },
        "direction": "Y",
        "quantity": quantity(1.0, "N/m", "force_per_length"),
        "provenance": provenance(),
    }
    valid_straight_pipe = {
        "id": "E-1",
        "element_type": "straight_pipe",
        "start_node_ref": reference("Node", "N-1"),
        "end_node_ref": reference("Node", "N-2"),
        "material_ref": reference("Material", "MAT-1"),
        "section_ref": reference("Section", "SEC-1"),
        "local_coordinate_system": {
            "type": "cartesian",
            "axes": ["X", "Y", "Z"],
            "y_reference": [0.0, 1.0, 0.0],
            "provenance": provenance(),
        },
        "result_stations": [quantity(0.0, "m", "length")],
    }

    try:
        for record in [
            valid_nodal_force,
            valid_nodal_moment,
            valid_element_point_force,
            valid_uniform_force,
        ]:
            assert validate_instance(load_record_schema, record)
        assert validate_instance(element_schema, valid_straight_pipe)

        invalid_moment_direction = deepcopy(valid_nodal_force)
        invalid_moment_direction["direction"] = "RX"
        assert_invalid_instance(load_record_schema, invalid_moment_direction)

        invalid_target = deepcopy(valid_element_point_force)
        invalid_target["target_ref"] = reference("Node", "N-1")
        assert_invalid_instance(load_record_schema, invalid_target)

        invalid_uniform_dimension = deepcopy(valid_uniform_force)
        invalid_uniform_dimension["quantity"] = quantity(1.0, "N", "force")
        assert_invalid_instance(load_record_schema, invalid_uniform_dimension)

        invalid_fraction = deepcopy(valid_element_point_force)
        invalid_fraction["station_fraction"] = quantity(1.1, "1", "dimensionless")
        assert_invalid_instance(load_record_schema, invalid_fraction)

        missing_orientation = deepcopy(valid_straight_pipe)
        del missing_orientation["local_coordinate_system"]["y_reference"]
        assert_invalid_instance(element_schema, missing_orientation)

        forbidden_local_x = deepcopy(valid_straight_pipe)
        forbidden_local_x["local_coordinate_system"]["local_x_axis"] = [1.0, 0.0, 0.0]
        assert_invalid_instance(element_schema, forbidden_local_x)
    except JsonSchemaDependencyMissing as exc:
        _skip_or_note_missing_jsonschema(exc)


def assert_invalid_instance(schema, instance):
    try:
        validate_instance(schema, instance)
    except AssertionError:
        return
    raise AssertionError("instance unexpectedly passed JSON Schema validation")


def _skip_or_note_missing_jsonschema(exc):
    if "pytest" in sys.modules:
        import pytest

        pytest.skip(str(exc))
    print(f"SKIP: {exc}")


def test_model_schema_contract():
    check_schema_contract()


def test_domain_fixtures_exercise_foundation_contracts():
    check_domain_fixtures()


def test_model_schema_jsonschema_validation_helper():
    check_jsonschema_validation()


def test_typed_load_records_and_straight_pipe_orientation_contract():
    check_typed_load_records_and_orientation_validation()


def test_traceability_link_field_scalar_paths_are_paired():
    schema = load_schema()
    definition = schema["$defs"]["TraceabilityLink"]
    assert {"source_field_path", "target_field_path"} <= set(
        definition["properties"]
    )
    assert definition["dependentRequired"] == {
        "source_field_path": ["target_field_path"],
        "target_field_path": ["source_field_path"],
    }

    link = deepcopy(load_json(PHYSICAL_MODEL_FIXTURE)["model"]["traceability_links"][0])
    link["source_field_path"] = "components[id=COMP-1].geometry.face_to_face.value"
    link["target_field_path"] = "components[id=COMP-1].geometry.face_to_face.value"
    link_schema = schema_for_definition(schema, "TraceabilityLink")
    assert validate_instance(link_schema, link)

    unpaired = deepcopy(link)
    del unpaired["target_field_path"]
    try:
        validate_instance(link_schema, unpaired)
    except AssertionError as exc:
        assert "target_field_path" in str(exc)
    else:
        raise AssertionError("unpaired field-scalar trace path must be rejected")


def main():
    check_schema_contract()
    check_domain_fixtures()
    check_jsonschema_validation()
    check_typed_load_records_and_orientation_validation()


if __name__ == "__main__":
    main()
