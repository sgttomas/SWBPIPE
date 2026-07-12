#!/usr/bin/env python3
"""Focused tests for DEL-13-04 physical-to-analytical transform contract."""

import json
import sys
from copy import deepcopy
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
TESTS_DIR = Path(__file__).resolve().parent
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))
if str(TESTS_DIR) not in sys.path:
    sys.path.insert(0, str(TESTS_DIR))

from core.model_transform.physical_to_analytical.contract import (  # noqa: E402
    transform_physical_to_analytical,
)
from schema_validation import schema_for_definition, validate_instance  # noqa: E402


MODEL_SCHEMA_PATH = ROOT / "schemas" / "model.schema.yaml"
PHYSICAL_SOURCE_FIXTURE = ROOT / "fixtures" / "domain" / "invented_physical_source_of_truth_model.json"
PREVIEW_ONLY_FIXTURE_KEYS = {
    "load_kind",
    "local_x_axis",
}


FORBIDDEN_CLAIMS = {
    "certification",
    "certified",
    "sealing",
    "sealed",
    "authentication",
    "code compliant",
    "professional approval",
    "engineering acceptance",
}


def provenance(source_name="invented transform fixture"):
    return {
        "source_name": source_name,
        "source_location": f"fixtures/{source_name.replace(' ', '_')}",
        "source_license": "public_permissive",
        "contributor": "DEL-13-04 test",
        "contributor_certification": "invented non-engineering transform fixture",
        "redistribution_status": "public_permissive",
        "review_status": "pending",
    }


def ref(object_type, item_id):
    return {"object_type": object_type, "id": item_id}


def quantity(value=1.0, unit="m", dimension="length"):
    return {
        "value": value,
        "unit": unit,
        "dimension": dimension,
        "provenance": provenance("quantity source"),
    }


def load_json(path):
    return json.loads(path.read_text(encoding="utf-8"))


def canonical_physical_model():
    return load_json(PHYSICAL_SOURCE_FIXTURE)["model"]


def all_keys(value):
    if isinstance(value, dict):
        for key, item in value.items():
            yield key
            yield from all_keys(item)
    elif isinstance(value, list):
        for item in value:
            yield from all_keys(item)


def physical_model():
    return canonical_physical_model()


def codes(result):
    return {item["code"] for item in result.diagnostics}


def all_text(value):
    if isinstance(value, str):
        yield value
    elif isinstance(value, dict):
        for item in value.values():
            yield from all_text(item)
    elif isinstance(value, (list, tuple)):
        for item in value:
            yield from all_text(item)


def test_canonical_physical_source_fixture_validates_and_transforms_deterministically():
    schema = load_json(MODEL_SCHEMA_PATH)
    source = canonical_physical_model()

    assert validate_instance(
        schema_for_definition(schema, "Model"),
        source,
        schema_label=f"{MODEL_SCHEMA_PATH}#/$defs/Model",
        instance_label=str(PHYSICAL_SOURCE_FIXTURE),
    )
    assert source["model_role"] == "physical_source_of_truth"
    assert "source_model_ref" not in source
    assert PREVIEW_ONLY_FIXTURE_KEYS.isdisjoint(set(all_keys(source)))
    assert [item["id"] for item in source["nodes"]] == ["N-1", "N-2"]
    assert [item["id"] for item in source["elements"]] == ["E-1"]
    assert [item["id"] for item in source["materials"]] == ["MAT-1"]
    assert [item["id"] for item in source["sections"]] == ["SEC-1"]
    assert [item["id"] for item in source["supports"]] == ["SUP-1"]
    assert [item["id"] for item in source["load_cases"]] == ["LC-1"]
    assert source["diagnostics"][0]["code"] == "ASSUMPTION_VISIBLE"

    first = transform_physical_to_analytical(source).to_dict()
    second = transform_physical_to_analytical(deepcopy(source)).to_dict()

    assert first == second
    assert not first["has_blocking_findings"]
    assert first["diagnostics"] == []

    analytical = first["analytical_model"]
    assert analytical["model_role"] == "analytical_solver_model"
    assert analytical["source_model_ref"] == ref("Model", "PHYS-1")
    assert analytical["diagnostics"] == []
    assert analytical["provenance"]["source_location"] == "physical-model:PHYS-1"
    assert [item["id"] for item in analytical["nodes"]] == ["N-1", "N-2"]

    element = analytical["elements"][0]
    assert element["id"] == "E-1"
    assert element["element_type"] == "straight_pipe"
    assert element["material_ref"] == ref("Material", "MAT-1")
    assert element["section_ref"] == ref("Section", "SEC-1")
    assert element["local_coordinate_system"]["y_reference"] == [0.0, 1.0, 0.0]
    assert element["result_stations"] == [quantity(0.0), quantity(1.0), quantity(2.0)]

    support = analytical["supports"][0]
    assert support["id"] == "SUP-1"
    assert support["target_ref"] == ref("Node", "N-1")
    assert support["properties"]["linear_stiffness"] == quantity(1.0, "N/m", "linear_stiffness")

    load_case = analytical["load_cases"][0]
    assert load_case["id"] == "LC-1"
    assert load_case["provenance"] == provenance("load case source")
    assert [load["target_ref"] for load in load_case["loads"]] == [
        ref("Element", "E-1"),
        ref("Element", "E-1"),
        ref("Node", "N-2"),
    ]
    assert [load["load_record_type"] for load in load_case["loads"]] == [
        "element_uniform_distributed_force",
        "element_point_force",
        "nodal_moment",
    ]
    assert [load["quantity"] for load in load_case["loads"]] == [
        quantity(1.0, "N/m", "force_per_length"),
        quantity(2.0, "N", "force"),
        quantity(3.0, "N*m", "moment"),
    ]
    assert load_case["loads"][0]["span"] == {
        "start_fraction": quantity(0.0, "1", "dimensionless"),
        "end_fraction": quantity(1.0, "1", "dimensionless"),
    }
    assert load_case["loads"][1]["station_fraction"] == quantity(0.5, "1", "dimensionless")

    trace_sources = {
        (link["trace_type"], link["source_ref"]["object_type"], link["source_ref"]["id"])
        for link in analytical["traceability_links"]
    }
    assert {
        ("physical_to_analytical", "Node", "N-1"),
        ("physical_to_analytical", "Node", "N-2"),
        ("physical_to_analytical", "Element", "E-1"),
        ("physical_to_analytical", "Material", "MAT-1"),
        ("physical_to_analytical", "Section", "SEC-1"),
        ("physical_to_analytical", "Support", "SUP-1"),
        ("physical_to_analytical", "LoadCase", "LC-1"),
    } <= trace_sources


def test_transform_is_deterministic_traceable_and_preserves_source_model():
    source = physical_model()
    original = deepcopy(source)

    first = transform_physical_to_analytical(source).to_dict()
    second = transform_physical_to_analytical(deepcopy(source)).to_dict()

    assert first == second
    assert source == original
    analytical = first["analytical_model"]
    assert analytical["model_role"] == "analytical_solver_model"
    assert analytical["source_model_ref"] == ref("Model", "PHYS-1")
    assert [item["id"] for item in analytical["nodes"]] == ["N-1", "N-2"]
    assert [item["id"] for item in analytical["elements"]] == ["E-1"]
    assert {link["source_ref"]["id"] for link in analytical["traceability_links"]} >= {
        "N-1",
        "N-2",
        "E-1",
        "MAT-1",
        "SEC-1",
        "SUP-1",
        "LC-1",
    }
    assert not first["has_blocking_findings"]


def test_missing_units_and_unsupported_physical_records_are_explicit_findings():
    source = physical_model()
    next(item for item in source["nodes"] if item["id"] == "N-2")["coordinates"]["x"]["unit"] = ""
    source["elements"].append(
        {
            "id": "E-SOLID",
            "element_type": "solid",
            "start_node_ref": ref("Node", "N-1"),
            "end_node_ref": ref("Node", "N-2"),
            "material_ref": ref("Material", "MAT-1"),
            "section_ref": ref("Section", "SEC-1"),
            "local_coordinate_system": {"type": "cartesian", "axes": ["X", "Y", "Z"]},
            "result_stations": [quantity(0.0)],
        }
    )

    result = transform_physical_to_analytical(source)

    assert {
        "PTA-NODE-COORDINATE-UNIT",
        "PTA-ELEMENT-TYPE-UNSUPPORTED",
        "PTA-ELEMENT-END-NODE-UNRESOLVED",
    } <= codes(result)
    assert result.has_blocking_findings
    assert [item["id"] for item in result.analytical_model["nodes"]] == ["N-1"]
    assert [item["id"] for item in result.analytical_model["elements"]] == []
    assert any(
        link["source_ref"]["id"] == "E-SOLID"
        and link["target_ref"]["object_type"] == "Diagnostic"
        for link in result.traceability_links
    )


def test_transform_rejects_noncanonical_quantity_dimensions():
    source = physical_model()
    source["sections"][0]["properties"]["legacy_area_moment"] = quantity(
        1.0,
        "m4",
        "area_moment",
    )

    result = transform_physical_to_analytical(source)

    assert "PTA-UNIT-DIMENSION-UNKNOWN" in codes(result)
    assert result.has_blocking_findings
    assert result.analytical_model["sections"] == []
    assert [item["id"] for item in result.analytical_model["elements"]] == []


def test_straight_pipe_transform_preserves_solver_needed_records():
    result = transform_physical_to_analytical(physical_model())
    analytical = result.analytical_model

    assert not result.has_blocking_findings
    element = analytical["elements"][0]
    material = analytical["materials"][0]
    section = analytical["sections"][0]
    load_case = analytical["load_cases"][0]

    assert element["element_type"] == "straight_pipe"
    assert element["start_node_ref"] == ref("Node", "N-1")
    assert element["end_node_ref"] == ref("Node", "N-2")
    assert element["material_ref"] == ref("Material", "MAT-1")
    assert element["section_ref"] == ref("Section", "SEC-1")
    assert element["result_stations"][0]["unit"] == "m"
    assert element["result_stations"][0]["dimension"] == "length"

    assert material["properties"]["elastic_modulus"]["unit"] == "Pa"
    assert material["properties"]["elastic_modulus"]["dimension"] == "stress"
    assert section["properties"]["area"]["unit"] == "m2"
    assert section["properties"]["area"]["dimension"] == "area"
    assert section["properties"]["second_moment_area"]["unit"] == "m4"
    assert section["properties"]["second_moment_area"]["dimension"] == "second_moment_area"
    assert load_case["loads"][0]["target_ref"] == ref("Element", "E-1")
    assert load_case["loads"][0]["load_record_type"] == "element_uniform_distributed_force"
    assert load_case["loads"][0]["quantity"]["unit"] == "N/m"
    assert load_case["loads"][0]["quantity"]["dimension"] == "force_per_length"
    assert load_case["loads"][0]["span"]["start_fraction"]["dimension"] == "dimensionless"
    assert load_case["loads"][0]["span"]["end_fraction"]["dimension"] == "dimensionless"
    assert load_case["loads"][1]["load_record_type"] == "element_point_force"
    assert load_case["loads"][1]["station_fraction"]["dimension"] == "dimensionless"
    assert load_case["loads"][1]["quantity"]["dimension"] == "force"
    assert load_case["loads"][2]["load_record_type"] == "nodal_moment"
    assert load_case["loads"][2]["target_ref"] == ref("Node", "N-2")
    assert load_case["loads"][2]["quantity"]["dimension"] == "moment"

    trace_targets = {
        (link["source_ref"]["object_type"], link["source_ref"]["id"]): link["target_ref"]
        for link in analytical["traceability_links"]
    }
    for item in [
        ("Element", "E-1"),
        ("Material", "MAT-1"),
        ("Section", "SEC-1"),
        ("LoadCase", "LC-1"),
    ]:
        assert trace_targets[item]["object_type"] == item[0]
        assert trace_targets[item]["id"] == item[1]


def test_supported_component_metadata_passes_through_when_referenced_by_valid_element():
    source = physical_model()
    component = {
        "id": "COMP-VALVE-1",
        "component_type": "valve",
        "name": "Invented metadata valve",
        "geometry": {
            "face_to_face": quantity(1.0, "m", "length"),
        },
        "mechanics_modifiers": [],
        "provenance": provenance("component metadata source"),
    }
    source["components"] = [component]
    source["elements"][0]["component_ref"] = ref("Component", "COMP-VALVE-1")

    result = transform_physical_to_analytical(source)
    analytical = result.analytical_model

    assert not result.has_blocking_findings
    assert result.diagnostics == ()
    assert analytical["components"] == [component]
    assert analytical["elements"][0]["component_ref"] == ref("Component", "COMP-VALVE-1")

    trace_targets = {
        (link["source_ref"]["object_type"], link["source_ref"]["id"]): link["target_ref"]
        for link in analytical["traceability_links"]
    }
    assert trace_targets[("Component", "COMP-VALVE-1")] == ref("Component", "COMP-VALVE-1")
    assert trace_targets[("Element", "E-1")] == ref("Element", "E-1")
    scalar_links = [
        link
        for link in analytical["traceability_links"]
        if link.get("source_field_path")
    ]
    assert scalar_links == [
        {
            "id": "TRACE-PTA-FIELD-Component-COMP-VALVE-1-geometry-face-to-face",
            "trace_type": "physical_to_analytical",
            "source_ref": ref("Component", "COMP-VALVE-1"),
            "target_ref": ref("Component", "COMP-VALVE-1"),
            "source_field_path": (
                "components[id=COMP-VALVE-1].geometry.face_to_face.value"
            ),
            "target_field_path": (
                "components[id=COMP-VALVE-1].geometry.face_to_face.value"
            ),
            "diagnostics": [],
            "provenance": scalar_links[0]["provenance"],
        }
    ]


def test_component_scalar_trace_is_not_emitted_for_invalid_quantity_metadata():
    source = physical_model()
    source["components"] = [
        {
            "id": "COMP-VALVE-1",
            "component_type": "valve",
            "name": "Invented metadata valve",
            "geometry": {
                "face_to_face": quantity(1.0, "m", "length"),
            },
            "mechanics_modifiers": [],
            "provenance": provenance("component metadata source"),
        }
    ]
    del source["components"][0]["geometry"]["face_to_face"]["dimension"]
    source["elements"][0]["component_ref"] = ref("Component", "COMP-VALVE-1")

    result = transform_physical_to_analytical(source)

    assert "PTA-COMPONENT-GEOMETRY-UNIT" in codes(result)
    assert not [
        link
        for link in result.traceability_links
        if link.get("source_field_path")
    ]


def test_unsupported_component_reference_blocks_element_without_analytical_approximation():
    source = physical_model()
    source["components"] = [
        {
            "id": "COMP-BEND-1",
            "component_type": "bend",
            "name": "Invented unsupported bend metadata",
            "geometry": {
                "bend_centerline_radius": quantity(1.0, "m", "length"),
            },
            "mechanics_modifiers": [],
            "provenance": provenance("unsupported component source"),
        }
    ]
    source["elements"][0]["component_ref"] = ref("Component", "COMP-BEND-1")

    result = transform_physical_to_analytical(source)

    assert {
        "PTA-COMPONENT-TYPE-UNSUPPORTED",
        "PTA-ELEMENT-COMPONENT-UNSUPPORTED",
    } <= codes(result)
    assert result.has_blocking_findings
    assert result.analytical_model["components"] == []
    assert result.analytical_model["elements"] == []
    assert any(
        link["source_ref"] == ref("Component", "COMP-BEND-1")
        and link["target_ref"]["object_type"] == "Diagnostic"
        for link in result.traceability_links
    )
    assert any(
        link["source_ref"] == ref("Element", "E-1")
        and link["target_ref"]["object_type"] == "Diagnostic"
        for link in result.traceability_links
    )


def test_non_axis_aligned_load_metadata_survives_transform():
    source = physical_model()
    source_nodes = {item["id"]: item for item in source["nodes"]}
    source_nodes["N-2"]["coordinates"]["x"] = quantity(0.0)
    source_nodes["N-2"]["coordinates"]["y"] = quantity(4.0)
    source["elements"][0]["local_coordinate_system"] = {
        "type": "cartesian",
        "axes": ["X", "Y", "Z"],
        "y_reference": [1, 0, 0],
        "provenance": provenance("orientation source"),
    }
    source["load_cases"][0]["loads"] = [
        {
            "load_record_type": "element_uniform_distributed_force",
            "target_ref": ref("Element", "E-1"),
            "span": {
                "start_fraction": quantity(0.0, "1", "dimensionless"),
                "end_fraction": quantity(1.0, "1", "dimensionless"),
            },
            "direction": "X",
            "quantity": quantity(1.0, "N/m", "force_per_length"),
            "provenance": provenance("distributed x load source"),
        },
        {
            "load_record_type": "element_point_force",
            "target_ref": ref("Element", "E-1"),
            "station_fraction": quantity(0.5, "1", "dimensionless"),
            "direction": "X",
            "quantity": quantity(2.0, "N", "force"),
            "provenance": provenance("point x force source"),
        },
    ]

    result = transform_physical_to_analytical(source)
    analytical = result.analytical_model

    assert not result.has_blocking_findings
    assert analytical["source_model_ref"] == ref("Model", "PHYS-1")

    nodes = {item["id"]: item for item in analytical["nodes"]}
    assert nodes["N-2"]["coordinates"]["x"] == quantity(0.0)
    assert nodes["N-2"]["coordinates"]["y"] == quantity(4.0)

    element = analytical["elements"][0]
    assert element["id"] == "E-1"
    assert element["local_coordinate_system"]["y_reference"] == [1, 0, 0]
    assert element["local_coordinate_system"]["provenance"] == provenance("orientation source")

    load_case = analytical["load_cases"][0]
    loads = load_case["loads"]
    assert load_case["id"] == "LC-1"
    assert load_case["provenance"] == provenance("load case source")
    assert loads[0]["load_record_type"] == "element_uniform_distributed_force"
    assert loads[0]["target_ref"] == ref("Element", "E-1")
    assert loads[0]["direction"] == "X"
    assert loads[0]["span"]["start_fraction"] == quantity(0.0, "1", "dimensionless")
    assert loads[0]["span"]["end_fraction"] == quantity(1.0, "1", "dimensionless")
    assert loads[0]["quantity"] == quantity(1.0, "N/m", "force_per_length")
    assert loads[0]["provenance"] == provenance("distributed x load source")
    assert loads[1]["load_record_type"] == "element_point_force"
    assert loads[1]["target_ref"] == ref("Element", "E-1")
    assert loads[1]["direction"] == "X"
    assert loads[1]["station_fraction"] == quantity(0.5, "1", "dimensionless")
    assert loads[1]["quantity"] == quantity(2.0, "N", "force")
    assert loads[1]["provenance"] == provenance("point x force source")

    trace_targets = {
        (link["source_ref"]["object_type"], link["source_ref"]["id"]): link["target_ref"]
        for link in analytical["traceability_links"]
    }
    assert trace_targets[("Element", "E-1")] == ref("Element", "E-1")
    assert trace_targets[("LoadCase", "LC-1")] == ref("LoadCase", "LC-1")


def test_equivalent_static_generation_inputs_survive_transform_and_validate():
    schema = load_json(MODEL_SCHEMA_PATH)
    source = physical_model()
    generation = {
        "seismic": {
            "gravity_acceleration": quantity(9.80665, "m/s^2", "acceleration"),
            "g_factors": {
                "x": quantity(0.3, "1", "dimensionless"),
                "z": quantity(0.2, "1", "dimensionless"),
            },
        },
        "wind": {
            "pressure": quantity(480.0, "Pa", "pressure"),
            "shape_factor": quantity(0.7, "1", "dimensionless"),
            "direction": "X",
            "exposed_element_refs": [ref("Element", "E-1")],
        },
        "provenance": provenance("equivalent static generation source"),
    }
    source["load_cases"][0]["equivalent_static_generation"] = deepcopy(generation)

    assert validate_instance(
        schema_for_definition(schema, "LoadCase"), source["load_cases"][0]
    )

    result = transform_physical_to_analytical(source)
    analytical = result.analytical_model

    assert not result.has_blocking_findings
    load_case = analytical["load_cases"][0]
    assert load_case["equivalent_static_generation"] == generation
    assert validate_instance(schema_for_definition(schema, "LoadCase"), load_case)


def test_equivalent_static_generation_rejects_defaulted_or_extra_fields():
    schema = load_json(MODEL_SCHEMA_PATH)
    load_case_schema = schema_for_definition(schema, "LoadCase")
    source = physical_model()
    base = {
        "seismic": {
            "gravity_acceleration": quantity(9.80665, "m/s^2", "acceleration"),
            "g_factors": {"y": quantity(0.3, "1", "dimensionless")},
        },
        "provenance": provenance("equivalent static generation source"),
    }

    def rejects(payload):
        source["load_cases"][0]["equivalent_static_generation"] = payload
        try:
            validate_instance(load_case_schema, source["load_cases"][0])
        except AssertionError:
            return True
        return False

    missing_gravity = deepcopy(base)
    del missing_gravity["seismic"]["gravity_acceleration"]
    assert rejects(missing_gravity)

    empty_axes = deepcopy(base)
    empty_axes["seismic"]["g_factors"] = {}
    assert rejects(empty_axes)

    unknown_field = deepcopy(base)
    unknown_field["code_exposure_category"] = "TBD"
    assert rejects(unknown_field)


def test_modulus_basis_references_survive_transform_and_validate():
    schema = load_json(MODEL_SCHEMA_PATH)
    source = physical_model()
    source["load_cases"][0]["modulus_basis_ref"] = "temperature-point.hot"

    assert validate_instance(
        schema_for_definition(schema, "LoadCase"), source["load_cases"][0]
    )

    combination = {
        "id": "C-RANGE-1",
        "name": "Invented expansion range",
        "basis": "range_envelope",
        "operand_refs": [ref("LoadCase", "LC-1")],
        "mode": "max_abs",
        "modulus_basis_records": [
            {
                "load_case_ref": ref("LoadCase", "LC-1"),
                "modulus_basis_ref": "temperature-point.hot",
            }
        ],
        "provenance": provenance("combination basis source"),
    }
    assert validate_instance(schema_for_definition(schema, "Combination"), combination)
    source["combinations"] = [combination]

    result = transform_physical_to_analytical(source)
    analytical = result.analytical_model

    assert not result.has_blocking_findings
    assert analytical["load_cases"][0]["modulus_basis_ref"] == "temperature-point.hot"
    assert analytical["combinations"][0]["modulus_basis_records"] == combination[
        "modulus_basis_records"
    ]


def test_unresolved_load_quantity_dimension_blocks_transform_without_inference():
    source = physical_model()
    source["load_cases"][0]["loads"].append(
        {
            "load_record_type": "element_uniform_distributed_force",
            "target_ref": ref("Element", "E-1"),
            "span": {
                "start_fraction": quantity(0.0, "1", "dimensionless"),
                "end_fraction": quantity(1.0, "1", "dimensionless"),
            },
            "direction": "Y",
            "quantity": quantity(4.0, "N/m", "TBD"),
            "provenance": provenance("unresolved line-load dimension source"),
        }
    )

    result = transform_physical_to_analytical(source)

    assert "PTA-LOAD-QUANTITY-UNIT" in codes(result)
    assert result.has_blocking_findings
    assert result.analytical_model["load_cases"] == []
    assert any(
        link["source_ref"]["id"] == "LC-1"
        and link["target_ref"]["object_type"] == "Diagnostic"
        for link in result.traceability_links
    )


def test_transform_output_contains_no_prohibited_authority_claims():
    result = transform_physical_to_analytical(physical_model()).to_dict()
    rendered = "\n".join(all_text(result)).lower()

    for claim in FORBIDDEN_CLAIMS:
        assert claim not in rendered


if __name__ == "__main__":
    test_canonical_physical_source_fixture_validates_and_transforms_deterministically()
    test_transform_is_deterministic_traceable_and_preserves_source_model()
    test_missing_units_and_unsupported_physical_records_are_explicit_findings()
    test_transform_rejects_noncanonical_quantity_dimensions()
    test_straight_pipe_transform_preserves_solver_needed_records()
    test_supported_component_metadata_passes_through_when_referenced_by_valid_element()
    test_unsupported_component_reference_blocks_element_without_analytical_approximation()
    test_non_axis_aligned_load_metadata_survives_transform()
    test_unresolved_load_quantity_dimension_blocks_transform_without_inference()
    test_transform_output_contains_no_prohibited_authority_claims()
