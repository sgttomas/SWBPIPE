#!/usr/bin/env python3
"""Focused tests for the internal analytical solver-boundary adapter."""

import json
import sys
from copy import deepcopy
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.model_transform.physical_to_analytical._solver_boundary_adapter import (  # noqa: E402
    adapt_analytical_solver_model,
)
from core.model_transform.physical_to_analytical.contract import (  # noqa: E402
    transform_physical_to_analytical,
)


PHYSICAL_SOURCE_FIXTURE = (
    ROOT / "fixtures" / "domain" / "invented_physical_source_of_truth_model.json"
)


def load_json(path):
    return json.loads(path.read_text(encoding="utf-8"))


def canonical_physical_model():
    return load_json(PHYSICAL_SOURCE_FIXTURE)["model"]


def analytical_solver_model():
    result = transform_physical_to_analytical(canonical_physical_model())
    assert not result.has_blocking_findings
    return result.analytical_model


def ref(object_type, item_id):
    return {"object_type": object_type, "id": item_id}


def codes(items):
    return {item["code"] for item in items}


def test_adapter_emits_deterministic_solver_boundary_dtos():
    source = analytical_solver_model()

    first = adapt_analytical_solver_model(source).to_dict()
    second = adapt_analytical_solver_model(deepcopy(source)).to_dict()

    assert first == second
    assert first["model_ref"] == ref("Model", "ANALYTICAL-DERIVED")
    assert first["source_model_ref"] == ref("Model", "PHYS-1")
    assert first["diagnostics"] == []
    assert not first["has_blocking_findings"]

    assert [node["node_id"] for node in first["nodes"]] == ["N-1", "N-2"]
    assert first["nodes"][0]["node_index"] == 0
    assert first["nodes"][0]["coordinates"] == [0.0, 0.0, 0.0]
    assert first["nodes"][0]["coordinate_units"]["x"] == {
        "unit": "m",
        "dimension": "length",
    }
    assert first["nodes"][1]["node_index"] == 1
    assert first["nodes"][1]["coordinates"] == [2.0, 0.0, 0.0]

    assert first["straight_pipe_connectivity"] == [
        {
            "element_id": "E-1",
            "element_index": 0,
            "node_i_id": "N-1",
            "node_i_index": 0,
            "node_j_id": "N-2",
            "node_j_index": 1,
            "material_id": "MAT-1",
            "section_id": "SEC-1",
            "local_coordinate_system": source["elements"][0]["local_coordinate_system"],
            "derived_axis_x": [1.0, 0.0, 0.0],
            "y_reference": [0.0, 1.0, 0.0],
            "solver_orientation_status": "governed_y_reference_ready",
            "source_ref": ref("Element", "E-1"),
        }
    ]

    binding = first["property_bindings"][0]
    assert binding["binding_id"] == "BIND-STRAIGHT-PIPE-E-1"
    assert binding["element_id"] == "E-1"
    assert binding["material_properties"]["elastic_modulus"]["dimension"] == "stress"
    assert binding["material_properties"]["shear_modulus"]["unit"] == "Pa"
    assert binding["section_properties"]["area"]["dimension"] == "area"
    assert (
        binding["section_properties"]["second_moment_y"]["dimension"]
        == "second_moment_area"
    )
    assert (
        binding["section_properties"]["second_moment_z"]["dimension"]
        == "second_moment_area"
    )
    assert (
        binding["section_properties"]["torsion_constant"]["dimension"]
        == "second_moment_area"
    )
    assert binding["section_properties"]["mass_per_length"]["dimension"] == "mass_per_length"

    assert first["support_targets"] == [
        {
            "support_id": "SUP-1",
            "support_type": "anchor",
            "target_type": "Node",
            "target_id": "N-1",
            "target_index": 0,
            "directions": ["UX", "UY", "UZ", "RX", "RY", "RZ"],
            "properties": source["supports"][0]["properties"],
            "source_ref": ref("Support", "SUP-1"),
        }
    ]

    assert first["load_case_records"] == [
        {
            "load_case_id": "LC-1",
            "load_case_index": 0,
            "name": "Invented load case",
            "load_type": "weight",
            "load_count": 3,
            "solver_application_status": "solver_ready",
            "source_ref": ref("LoadCase", "LC-1"),
        }
    ]
    assert first["load_case_diagnostics"] == []
    assert [
        (item["load_record_type"], item["target_type"], item["target_id"], item["direction"])
        for item in first["load_applications"]
    ] == [
        ("element_uniform_distributed_force", "Element", "E-1", "Y"),
        ("element_point_force", "Element", "E-1", "Y"),
        ("nodal_moment", "Node", "N-2", "RZ"),
    ]
    assert first["load_applications"][0]["quantity"]["dimension"] == "force_per_length"
    assert first["load_applications"][0]["span"]["start_fraction"]["dimension"] == "dimensionless"
    assert first["load_applications"][0]["span"]["end_fraction"]["value"] == 1.0
    assert first["load_applications"][1]["station_fraction"]["value"] == 0.5
    assert first["load_applications"][2]["quantity"]["dimension"] == "moment"
    assert [item["dto_id"] for item in first["adapter_dto_records"]] == [
        "dto:load_application:LC-1:0",
        "dto:load_application:LC-1:1",
        "dto:load_application:LC-1:2",
    ]
    first_dto = first["adapter_dto_records"][0]
    assert first_dto["dto_kind"] == "load_application"
    assert first_dto["source_ref"] == ref("LoadCase", "LC-1:load:0")
    assert first_dto["result_trace_anchor"] == {
        "ref_type": "adapter_dto",
        "ref_id": "dto:load_application:LC-1:0",
    }
    assert first_dto["solver_input_trace_anchor"] == {
        "ref_type": "solver_input",
        "ref_id": "solver_input:load_application:LC-1:0",
    }
    assert first_dto["source_chain"] == [
        {
            "trace_id": "trace:analytical-load-to-adapter-dto:LC-1:0",
            "trace_type": "analytical_model_to_adapter_dto",
            "source_ref": ref("LoadCase", "LC-1:load:0"),
            "target_ref": {
                "ref_type": "adapter_dto",
                "ref_id": "dto:load_application:LC-1:0",
            },
            "provenance": first_dto["provenance"],
        },
        {
            "trace_id": "trace:adapter-dto-to-solver-input:LC-1:0",
            "trace_type": "adapter_dto_to_solver_input",
            "source_ref": {
                "ref_type": "adapter_dto",
                "ref_id": "dto:load_application:LC-1:0",
            },
            "target_ref": {
                "ref_type": "solver_input",
                "ref_id": "solver_input:load_application:LC-1:0",
            },
            "provenance": first_dto["provenance"],
        },
    ]
    assert first_dto["payload_hash_ref"]["algorithm"] == "sha256"
    assert first_dto["payload_hash_ref"]["canonicalization"] == "JCS"
    assert first_dto["payload_hash_ref"]["payload_ref"] == ref(
        "AdapterDTO", "dto:load_application:LC-1:0"
    )
    assert len(first_dto["payload_hash_ref"]["value"]) == 64


def test_adapter_does_not_infer_missing_solver_property_bindings():
    source = analytical_solver_model()
    del source["materials"][0]["properties"]["shear_modulus"]

    result = adapt_analytical_solver_model(source).to_dict()

    assert result["straight_pipe_connectivity"][0]["element_id"] == "E-1"
    assert result["property_bindings"] == []
    assert "ASBA-MATERIAL-PROPERTY-MISSING" in codes(result["diagnostics"])


def test_adapter_diagnoses_noncanonical_load_semantics_without_mapping_them():
    source = analytical_solver_model()
    source["load_cases"][0]["loads"][0]["load_kind"] = "point_force"

    result = adapt_analytical_solver_model(source).to_dict()

    assert "ASBA-LOAD-NONCANONICAL-SEMANTICS-UNSUPPORTED" in codes(
        result["load_case_diagnostics"]
    )
    assert "ASBA-LOAD-SEMANTICS-AMBIGUOUS" not in codes(result["load_case_diagnostics"])
    assert result["load_case_records"][0]["solver_application_status"] == "blocked_by_diagnostics"
    assert [item["load_index"] for item in result["load_applications"]] == [1, 2]
    assert "user_loads" not in result


def test_adapter_blocks_invalid_straight_pipe_y_reference_orientation():
    source = analytical_solver_model()
    source["elements"][0]["local_coordinate_system"]["y_reference"] = [1.0, 0.0, 0.0]

    parallel = adapt_analytical_solver_model(source).to_dict()

    assert parallel["straight_pipe_connectivity"] == []
    assert "ASBA-ELEMENT-Y-REFERENCE-PARALLEL" in codes(parallel["diagnostics"])

    source = analytical_solver_model()
    source["elements"][0]["local_coordinate_system"]["y_reference"] = [0.0, 0.0, 0.0]

    zero = adapt_analytical_solver_model(source).to_dict()

    assert zero["straight_pipe_connectivity"] == []
    assert "ASBA-ELEMENT-Y-REFERENCE-ZERO" in codes(zero["diagnostics"])

    source = analytical_solver_model()
    source["elements"][0]["local_coordinate_system"]["y_reference"] = [0.0, float("nan"), 0.0]

    nonfinite = adapt_analytical_solver_model(source).to_dict()

    assert nonfinite["straight_pipe_connectivity"] == []
    assert "ASBA-ELEMENT-Y-REFERENCE-NONFINITE" in codes(nonfinite["diagnostics"])


def test_adapter_blocks_unordered_uniform_load_span_at_runtime():
    source = analytical_solver_model()
    span = source["load_cases"][0]["loads"][0]["span"]
    span["start_fraction"]["value"] = 0.75
    span["end_fraction"]["value"] = 0.25

    result = adapt_analytical_solver_model(source).to_dict()

    assert "ASBA-LOAD-SPAN-FRACTION-ORDER" in codes(result["load_case_diagnostics"])
    assert result["load_case_records"][0]["solver_application_status"] == "blocked_by_diagnostics"
    assert [item["load_index"] for item in result["load_applications"]] == [1, 2]


def test_adapter_preserves_load_record_failures_as_result_boundary_diagnostics():
    source = analytical_solver_model()
    unsupported_mapping = {
        "load_record_type": "element_thermal_gradient",
        "target_ref": ref("Element", "E-1"),
        "direction": "Y",
        "quantity": deepcopy(source["load_cases"][0]["loads"][1]["quantity"]),
        "provenance": deepcopy(source["load_cases"][0]["loads"][1]["provenance"]),
    }
    unresolved_target = deepcopy(source["load_cases"][0]["loads"][1])
    unresolved_target["target_ref"] = ref("Element", "E-MISSING")
    wrong_dimension = deepcopy(source["load_cases"][0]["loads"][0])
    wrong_dimension["quantity"]["dimension"] = "force"
    nonfinite_value = deepcopy(source["load_cases"][0]["loads"][2])
    nonfinite_value["quantity"]["value"] = float("inf")
    source["load_cases"][0]["loads"] = [
        "unsupported-load-record",
        unsupported_mapping,
        unresolved_target,
        wrong_dimension,
        nonfinite_value,
    ]

    result = adapt_analytical_solver_model(source).to_dict()

    assert result["load_applications"] == []
    assert result["adapter_dto_records"] == []
    assert result["load_case_records"] == [
        {
            "load_case_id": "LC-1",
            "load_case_index": 0,
            "name": "Invented load case",
            "load_type": "weight",
            "load_count": 5,
            "solver_application_status": "blocked_by_diagnostics",
            "source_ref": ref("LoadCase", "LC-1"),
        }
    ]
    assert {
        "ASBA-LOAD-RECORD-UNSUPPORTED",
        "ASBA-LOAD-RECORD-TYPE-UNSUPPORTED",
        "ASBA-LOAD-TARGET-UNRESOLVED",
        "ASBA-LOAD-QUANTITY-DIMENSION-UNSUPPORTED",
        "ASBA-LOAD-QUANTITY-NONFINITE",
    } <= codes(result["load_case_diagnostics"])
    assert result["diagnostics"] == []
    assert result["has_blocking_findings"]
    assert "user_loads" not in result


def test_adapter_requires_analytical_solver_model_role():
    source = analytical_solver_model()
    source["model_role"] = "physical_source_of_truth"

    result = adapt_analytical_solver_model(source).to_dict()

    assert result["nodes"] == []
    assert result["straight_pipe_connectivity"] == []
    assert result["property_bindings"] == []
    assert result["support_targets"] == []
    assert result["load_case_records"] == []
    assert result["load_applications"] == []
    assert result["load_case_diagnostics"] == []
    assert codes(result["diagnostics"]) == {"ASBA-SOURCE-ROLE-UNEXPECTED"}


if __name__ == "__main__":
    test_adapter_emits_deterministic_solver_boundary_dtos()
    test_adapter_does_not_infer_missing_solver_property_bindings()
    test_adapter_diagnoses_noncanonical_load_semantics_without_mapping_them()
    test_adapter_blocks_invalid_straight_pipe_y_reference_orientation()
    test_adapter_blocks_unordered_uniform_load_span_at_runtime()
    test_adapter_preserves_load_record_failures_as_result_boundary_diagnostics()
    test_adapter_requires_analytical_solver_model_role()
