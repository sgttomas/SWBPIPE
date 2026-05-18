"""Internal analytical-model to solver-boundary DTO adapter.

This module accepts canonical ``analytical_solver_model`` mappings and emits
plain deterministic records for lower solver-boundary orchestration. It is not
a public API, CLI, preview runtime, or load application engine.
"""

from __future__ import annotations

from copy import deepcopy
from dataclasses import dataclass
import hashlib
import json
from math import isfinite
from typing import Any, Iterable, Mapping


ADAPTER_CONTRACT_ID = "DEL-13-04-internal-analytical-solver-boundary-adapter-0.1"
AXES = ("X", "Y", "Z")
DOF_AXES = ("UX", "UY", "UZ", "RX", "RY", "RZ")
VECTOR_TOLERANCE = 1.0e-12
LOAD_RECORD_CONTRACTS = {
    "nodal_force": {
        "keys": {"load_record_type", "target_ref", "direction", "quantity", "provenance"},
        "target_type": "Node",
        "quantity_dimension": "force",
        "direction_axes": ("X", "Y", "Z"),
    },
    "nodal_moment": {
        "keys": {"load_record_type", "target_ref", "direction", "quantity", "provenance"},
        "target_type": "Node",
        "quantity_dimension": "moment",
        "direction_axes": ("RX", "RY", "RZ"),
    },
    "element_point_force": {
        "keys": {
            "load_record_type",
            "target_ref",
            "station_fraction",
            "direction",
            "quantity",
            "provenance",
        },
        "target_type": "Element",
        "quantity_dimension": "force",
        "direction_axes": ("X", "Y", "Z"),
    },
    "element_uniform_distributed_force": {
        "keys": {
            "load_record_type",
            "target_ref",
            "span",
            "direction",
            "quantity",
            "provenance",
        },
        "target_type": "Element",
        "quantity_dimension": "force_per_length",
        "direction_axes": ("X", "Y", "Z"),
    },
}

MATERIAL_PROPERTY_DIMENSIONS = {
    "elastic_modulus": "stress",
    "shear_modulus": "stress",
}
SECTION_PROPERTY_DIMENSIONS = {
    "area": "area",
    "second_moment_y": "second_moment_area",
    "second_moment_z": "second_moment_area",
    "torsion_constant": "second_moment_area",
}
OPTIONAL_SECTION_PROPERTY_DIMENSIONS = {
    "mass_per_length": "mass_per_length",
}


@dataclass(frozen=True)
class SolverBoundaryAdapterResult:
    """Deterministic DTO result for the internal solver boundary."""

    model_ref: dict[str, str]
    source_model_ref: dict[str, str] | None
    nodes: tuple[dict[str, Any], ...]
    straight_pipe_connectivity: tuple[dict[str, Any], ...]
    property_bindings: tuple[dict[str, Any], ...]
    support_targets: tuple[dict[str, Any], ...]
    load_case_records: tuple[dict[str, Any], ...]
    load_applications: tuple[dict[str, Any], ...]
    adapter_dto_records: tuple[dict[str, Any], ...]
    load_case_diagnostics: tuple[dict[str, Any], ...]
    diagnostics: tuple[dict[str, Any], ...]

    @property
    def has_blocking_findings(self) -> bool:
        findings = (*self.diagnostics, *self.load_case_diagnostics)
        return any(item.get("severity") == "blocking" for item in findings)

    def to_dict(self) -> dict[str, Any]:
        return {
            "adapter_contract_id": ADAPTER_CONTRACT_ID,
            "model_ref": deepcopy(self.model_ref),
            "source_model_ref": deepcopy(self.source_model_ref),
            "nodes": [deepcopy(item) for item in self.nodes],
            "straight_pipe_connectivity": [
                deepcopy(item) for item in self.straight_pipe_connectivity
            ],
            "property_bindings": [deepcopy(item) for item in self.property_bindings],
            "support_targets": [deepcopy(item) for item in self.support_targets],
            "load_case_records": [deepcopy(item) for item in self.load_case_records],
            "load_applications": [
                deepcopy(item) for item in self.load_applications
            ],
            "adapter_dto_records": [
                deepcopy(item) for item in self.adapter_dto_records
            ],
            "load_case_diagnostics": [
                deepcopy(item) for item in self.load_case_diagnostics
            ],
            "diagnostics": [deepcopy(item) for item in self.diagnostics],
            "has_blocking_findings": self.has_blocking_findings,
        }


def adapt_analytical_solver_model(
    analytical_model: Mapping[str, Any] | None,
) -> SolverBoundaryAdapterResult:
    """Adapt a strict analytical solver model into deterministic DTO records."""

    diagnostics: list[dict[str, Any]] = []
    load_case_records: list[dict[str, Any]] = []
    load_case_diagnostics: list[dict[str, Any]] = []

    if not isinstance(analytical_model, Mapping):
        model_ref = _ref("Model", "UNRESOLVED-ANALYTICAL-MODEL")
        diagnostics.append(
            _diagnostic(
                "ASBA-SOURCE-MODEL-MISSING",
                "blocking",
                model_ref,
                "The solver-boundary adapter requires an analytical_solver_model mapping.",
                "Provide a canonical analytical_solver_model mapping.",
            )
        )
        return _result(
            model_ref,
            None,
            [],
            [],
            [],
            [],
            load_case_records,
            [],
            [],
            load_case_diagnostics,
            diagnostics,
        )

    model_id = str(analytical_model.get("id", "UNRESOLVED-ANALYTICAL-MODEL"))
    model_ref = _ref("Model", model_id)
    source_model_ref = (
        deepcopy(analytical_model.get("source_model_ref"))
        if isinstance(analytical_model.get("source_model_ref"), Mapping)
        else None
    )
    if analytical_model.get("model_role") != "analytical_solver_model":
        diagnostics.append(
            _diagnostic(
                "ASBA-SOURCE-ROLE-UNEXPECTED",
                "blocking",
                model_ref,
                "Solver-boundary DTO adaptation accepts analytical_solver_model records only.",
                "Derive or select the analytical_solver_model before adapting to the solver boundary.",
            )
        )
        return _result(
            model_ref,
            source_model_ref,
            [],
            [],
            [],
            [],
            load_case_records,
            [],
            [],
            load_case_diagnostics,
            diagnostics,
        )

    nodes, node_index_by_id, node_diagnostics = _adapt_nodes(analytical_model.get("nodes"))
    diagnostics.extend(node_diagnostics)
    node_by_id = {node["node_id"]: node for node in nodes}

    material_by_id = _record_map(analytical_model.get("materials"))
    section_by_id = _record_map(analytical_model.get("sections"))

    (
        straight_pipes,
        straight_pipe_index_by_id,
        straight_pipe_diagnostics,
    ) = _adapt_straight_pipes(
        analytical_model.get("elements"),
        node_index_by_id,
        node_by_id,
        material_by_id,
        section_by_id,
    )
    diagnostics.extend(straight_pipe_diagnostics)

    property_bindings, property_diagnostics = _adapt_property_bindings(
        straight_pipes,
        material_by_id,
        section_by_id,
    )
    diagnostics.extend(property_diagnostics)

    supports, support_diagnostics = _adapt_supports(
        analytical_model.get("supports"),
        node_index_by_id,
        straight_pipe_index_by_id,
    )
    diagnostics.extend(support_diagnostics)

    load_case_records, load_applications, load_diagnostics = _adapt_load_cases(
        analytical_model.get("load_cases"),
        node_index_by_id,
        straight_pipe_index_by_id,
    )
    load_case_diagnostics.extend(load_diagnostics)
    adapter_dto_records = _adapter_dto_identity_records(load_applications)

    return _result(
        model_ref,
        source_model_ref,
        nodes,
        straight_pipes,
        property_bindings,
        supports,
        load_case_records,
        load_applications,
        adapter_dto_records,
        load_case_diagnostics,
        diagnostics,
    )


def _adapt_nodes(
    records: Any,
) -> tuple[list[dict[str, Any]], dict[str, int], list[dict[str, Any]]]:
    nodes: list[dict[str, Any]] = []
    diagnostics: list[dict[str, Any]] = []
    for record in _sorted_records(records):
        node_id = str(record.get("id", "UNRESOLVED-NODE"))
        node_ref = _ref("Node", node_id)
        coordinates = record.get("coordinates")
        dofs = record.get("degrees_of_freedom")
        gaps = _required_keys(record, ("id", "coordinates", "degrees_of_freedom"))
        if gaps:
            diagnostics.extend(
                _missing_diagnostics("ASBA-NODE-MISSING-FIELD", gaps, node_ref)
            )
            continue
        if not isinstance(dofs, Mapping) or any(
            dofs.get(axis) in (None, "", "TBD") for axis in DOF_AXES
        ):
            diagnostics.append(
                _diagnostic(
                    "ASBA-NODE-DOF-UNRESOLVED",
                    "blocking",
                    node_ref,
                    "Node degree-of-freedom states must be explicit at the solver boundary.",
                    "Supply UX, UY, UZ, RX, RY, and RZ states without TBD values.",
                )
            )
            continue
        coordinate_values: list[float] = []
        coordinate_units: dict[str, dict[str, str]] = {}
        coordinate_quantities: dict[str, dict[str, Any]] = {}
        for axis in ("x", "y", "z"):
            quantity, quantity_diagnostics = _quantity(
                coordinates.get(axis) if isinstance(coordinates, Mapping) else None,
                "length",
                node_ref,
                "ASBA-NODE-COORDINATE",
            )
            diagnostics.extend(quantity_diagnostics)
            if quantity is None:
                break
            coordinate_values.append(quantity["value"])
            coordinate_units[axis] = {
                "unit": quantity["unit"],
                "dimension": quantity["dimension"],
            }
            coordinate_quantities[axis] = quantity
        if len(coordinate_values) != 3:
            continue
        node_index = len(nodes)
        nodes.append(
            {
                "node_id": node_id,
                "node_index": node_index,
                "coordinates": coordinate_values,
                "coordinate_units": coordinate_units,
                "coordinate_quantities": coordinate_quantities,
                "degrees_of_freedom": deepcopy(dict(dofs)),
                "source_ref": node_ref,
            }
        )
    return nodes, {item["node_id"]: item["node_index"] for item in nodes}, diagnostics


def _adapt_straight_pipes(
    records: Any,
    node_index_by_id: Mapping[str, int],
    node_by_id: Mapping[str, Mapping[str, Any]],
    material_by_id: Mapping[str, Mapping[str, Any]],
    section_by_id: Mapping[str, Mapping[str, Any]],
) -> tuple[list[dict[str, Any]], dict[str, int], list[dict[str, Any]]]:
    straight_pipes: list[dict[str, Any]] = []
    diagnostics: list[dict[str, Any]] = []
    for record in _sorted_records(records):
        element_id = str(record.get("id", "UNRESOLVED-ELEMENT"))
        element_ref = _ref("Element", element_id)
        gaps = _required_keys(
            record,
            (
                "id",
                "element_type",
                "start_node_ref",
                "end_node_ref",
                "material_ref",
                "section_ref",
                "local_coordinate_system",
            ),
        )
        if gaps:
            diagnostics.extend(
                _missing_diagnostics("ASBA-ELEMENT-MISSING-FIELD", gaps, element_ref)
            )
            continue
        if record.get("element_type") != "straight_pipe":
            diagnostics.append(
                _diagnostic(
                    "ASBA-ELEMENT-TYPE-UNSUPPORTED",
                    "blocking",
                    element_ref,
                    f"Element type {record.get('element_type')!r} is not adapted to straight-pipe solver DTOs.",
                    "Provide a straight_pipe analytical element or add a separate internal adapter for the element type.",
                )
            )
            continue

        start_node_id = _ref_id(record.get("start_node_ref"))
        end_node_id = _ref_id(record.get("end_node_ref"))
        material_id = _ref_id(record.get("material_ref"))
        section_id = _ref_id(record.get("section_ref"))
        reference_diagnostics = []
        reference_diagnostics.extend(
            _reference_diagnostics(
                start_node_id,
                node_index_by_id,
                element_ref,
                "ASBA-ELEMENT-START-NODE-UNRESOLVED",
                "start node",
            )
        )
        reference_diagnostics.extend(
            _reference_diagnostics(
                end_node_id,
                node_index_by_id,
                element_ref,
                "ASBA-ELEMENT-END-NODE-UNRESOLVED",
                "end node",
            )
        )
        reference_diagnostics.extend(
            _reference_diagnostics(
                material_id,
                material_by_id,
                element_ref,
                "ASBA-ELEMENT-MATERIAL-UNRESOLVED",
                "material",
            )
        )
        reference_diagnostics.extend(
            _reference_diagnostics(
                section_id,
                section_by_id,
                element_ref,
                "ASBA-ELEMENT-SECTION-UNRESOLVED",
                "section",
            )
        )
        coordinate_system = record.get("local_coordinate_system")
        if not isinstance(coordinate_system, Mapping) or coordinate_system.get("type") != "cartesian":
            reference_diagnostics.append(
                _diagnostic(
                    "ASBA-ELEMENT-LOCAL-COORDINATE-SYSTEM-UNSUPPORTED",
                    "blocking",
                    element_ref,
                    "Straight-pipe connectivity requires an explicit cartesian local coordinate system record.",
                    "Supply the canonical local_coordinate_system mapping without relying on an inferred orientation.",
                )
            )
        elif set(coordinate_system) != {"type", "axes", "y_reference", "provenance"}:
            unexpected_keys = sorted(
                set(coordinate_system) - {"type", "axes", "y_reference", "provenance"}
            )
            reference_diagnostics.append(
                _diagnostic(
                    "ASBA-ELEMENT-LOCAL-COORDINATE-SYSTEM-NONCANONICAL",
                    "blocking",
                    element_ref,
                    (
                        "Straight-pipe connectivity requires canonical local_coordinate_system "
                        "fields type, axes, y_reference, and provenance."
                    ),
                    (
                        "Supply governed straight-pipe y_reference metadata and remove "
                        f"noncanonical orientation keys: {', '.join(unexpected_keys) or 'none'}."
                    ),
                )
            )
        elif coordinate_system.get("axes") != list(AXES):
            reference_diagnostics.append(
                _diagnostic(
                    "ASBA-ELEMENT-LOCAL-AXES-UNSUPPORTED",
                    "blocking",
                    element_ref,
                    "Straight-pipe connectivity currently records canonical X/Y/Z local axes only.",
                    "Preserve non-default orientation data outside this canonical solver-boundary adapter until a governed orientation contract is added.",
                )
            )
        else:
            orientation_diagnostics = _orientation_diagnostics(
                coordinate_system,
                node_by_id.get(str(start_node_id)),
                node_by_id.get(str(end_node_id)),
                element_ref,
            )
            reference_diagnostics.extend(orientation_diagnostics)
        if reference_diagnostics:
            diagnostics.extend(reference_diagnostics)
            continue

        element_index = len(straight_pipes)
        straight_pipes.append(
            {
                "element_id": element_id,
                "element_index": element_index,
                "node_i_id": start_node_id,
                "node_i_index": node_index_by_id[start_node_id],
                "node_j_id": end_node_id,
                "node_j_index": node_index_by_id[end_node_id],
                "material_id": material_id,
                "section_id": section_id,
                "local_coordinate_system": deepcopy(dict(coordinate_system)),
                "derived_axis_x": _derived_axis_x(
                    node_by_id[str(start_node_id)]["coordinates"],
                    node_by_id[str(end_node_id)]["coordinates"],
                ),
                "y_reference": list(coordinate_system["y_reference"]),
                "solver_orientation_status": "governed_y_reference_ready",
                "source_ref": element_ref,
            }
        )
    return (
        straight_pipes,
        {item["element_id"]: item["element_index"] for item in straight_pipes},
        diagnostics,
    )


def _adapt_property_bindings(
    straight_pipes: Iterable[Mapping[str, Any]],
    material_by_id: Mapping[str, Mapping[str, Any]],
    section_by_id: Mapping[str, Mapping[str, Any]],
) -> tuple[list[dict[str, Any]], list[dict[str, Any]]]:
    bindings: list[dict[str, Any]] = []
    diagnostics: list[dict[str, Any]] = []
    for pipe in straight_pipes:
        element_id = str(pipe["element_id"])
        element_ref = _ref("Element", element_id)
        material = material_by_id[str(pipe["material_id"])]
        section = section_by_id[str(pipe["section_id"])]
        material_properties, material_diagnostics = _required_quantity_properties(
            material.get("properties"),
            MATERIAL_PROPERTY_DIMENSIONS,
            _ref("Material", str(pipe["material_id"])),
            "ASBA-MATERIAL",
        )
        section_properties, section_diagnostics = _required_quantity_properties(
            section.get("properties"),
            SECTION_PROPERTY_DIMENSIONS,
            _ref("Section", str(pipe["section_id"])),
            "ASBA-SECTION",
        )
        optional_section_properties, optional_section_diagnostics = _optional_quantity_properties(
            section.get("properties"),
            OPTIONAL_SECTION_PROPERTY_DIMENSIONS,
            _ref("Section", str(pipe["section_id"])),
            "ASBA-SECTION",
        )
        binding_diagnostics = [
            *material_diagnostics,
            *section_diagnostics,
            *optional_section_diagnostics,
        ]
        if binding_diagnostics:
            diagnostics.extend(binding_diagnostics)
            continue
        bindings.append(
            {
                "binding_id": f"BIND-STRAIGHT-PIPE-{element_id}",
                "element_id": element_id,
                "element_index": pipe["element_index"],
                "material_id": pipe["material_id"],
                "section_id": pipe["section_id"],
                "material_properties": material_properties,
                "section_properties": {
                    **section_properties,
                    **optional_section_properties,
                },
                "source_ref": element_ref,
            }
        )
    return bindings, diagnostics


def _adapt_supports(
    records: Any,
    node_index_by_id: Mapping[str, int],
    element_index_by_id: Mapping[str, int],
) -> tuple[list[dict[str, Any]], list[dict[str, Any]]]:
    supports: list[dict[str, Any]] = []
    diagnostics: list[dict[str, Any]] = []
    for record in _sorted_records(records):
        support_id = str(record.get("id", "UNRESOLVED-SUPPORT"))
        support_ref = _ref("Support", support_id)
        gaps = _required_keys(
            record, ("id", "support_type", "target_ref", "directions", "properties")
        )
        if gaps:
            diagnostics.extend(
                _missing_diagnostics("ASBA-SUPPORT-MISSING-FIELD", gaps, support_ref)
            )
            continue
        directions = record.get("directions")
        if (
            not isinstance(directions, list)
            or not directions
            or any(direction not in DOF_AXES for direction in directions)
        ):
            diagnostics.append(
                _diagnostic(
                    "ASBA-SUPPORT-DIRECTIONS-UNSUPPORTED",
                    "blocking",
                    support_ref,
                    "Support target DTOs require explicit UX, UY, UZ, RX, RY, or RZ directions.",
                    "Supply canonical support directions without inferring restrained axes from support type.",
                )
            )
            continue
        target_ref = record.get("target_ref")
        target_type = target_ref.get("object_type") if isinstance(target_ref, Mapping) else None
        target_id = _ref_id(target_ref)
        target_index: int | None = None
        if target_type == "Node" and target_id in node_index_by_id:
            target_index = node_index_by_id[target_id]
        elif target_type == "Element" and target_id in element_index_by_id:
            target_index = element_index_by_id[target_id]
        else:
            diagnostics.append(
                _diagnostic(
                    "ASBA-SUPPORT-TARGET-UNRESOLVED",
                    "blocking",
                    support_ref,
                    "Support target does not resolve to an adapted node or straight-pipe element.",
                    "Provide a support target that is present in the adapted solver-boundary records.",
                )
            )
            continue
        supports.append(
            {
                "support_id": support_id,
                "support_type": record["support_type"],
                "target_type": target_type,
                "target_id": target_id,
                "target_index": target_index,
                "directions": list(directions),
                "properties": deepcopy(record.get("properties", {})),
                "source_ref": support_ref,
            }
        )
    return supports, diagnostics


def _adapt_load_cases(
    records: Any,
    node_index_by_id: Mapping[str, int],
    element_index_by_id: Mapping[str, int],
) -> tuple[list[dict[str, Any]], list[dict[str, Any]], list[dict[str, Any]]]:
    load_case_records: list[dict[str, Any]] = []
    load_applications: list[dict[str, Any]] = []
    diagnostics: list[dict[str, Any]] = []
    for load_case in _sorted_records(records):
        load_case_id = str(load_case.get("id", "UNRESOLVED-LOADCASE"))
        load_case_ref = _ref("LoadCase", load_case_id)
        gaps = _required_keys(load_case, ("id", "name", "load_type", "loads", "provenance"))
        if gaps:
            diagnostics.extend(
                _missing_diagnostics("ASBA-LOADCASE-MISSING-FIELD", gaps, load_case_ref)
            )
            continue
        loads = load_case.get("loads")
        if not isinstance(loads, list):
            diagnostics.append(
                _diagnostic(
                    "ASBA-LOADCASE-LOADS-MISSING",
                    "blocking",
                    load_case_ref,
                    "Load case does not contain a canonical loads array.",
                    "Provide explicit canonical LoadRecord entries.",
                )
            )
            continue
        load_case_index = len(load_case_records)
        load_application_start = len(load_applications)
        load_diagnostic_start = len(diagnostics)
        for index, load in enumerate(loads):
            load_ref = _ref("LoadCase", f"{load_case_id}:load:{index}")
            if not isinstance(load, Mapping):
                diagnostics.append(
                    _diagnostic(
                        "ASBA-LOAD-RECORD-UNSUPPORTED",
                        "blocking",
                        load_ref,
                        "Load entries must be canonical LoadRecord mappings.",
                        "Provide a mapping with target_ref, quantity, and provenance.",
                    )
                )
                continue
            load_record_type = load.get("load_record_type")
            contract = LOAD_RECORD_CONTRACTS.get(load_record_type)
            if contract is None:
                diagnostics.append(
                    _diagnostic(
                        "ASBA-LOAD-RECORD-TYPE-UNSUPPORTED",
                        "blocking",
                        load_ref,
                        f"Load record type {load_record_type!r} is not a strict canonical load union member.",
                        "Use nodal_force, nodal_moment, element_point_force, or element_uniform_distributed_force.",
                    )
                )
                continue
            expected_keys = contract["keys"]
            extra_keys = sorted(set(load) - expected_keys)
            if extra_keys:
                diagnostics.append(
                    _diagnostic(
                        "ASBA-LOAD-NONCANONICAL-SEMANTICS-UNSUPPORTED",
                        "blocking",
                        load_ref,
                        f"Load record contains noncanonical solver-semantics keys: {', '.join(extra_keys)}.",
                        "Remove noncanonical keys and use the strict typed LoadRecord union.",
                    )
                )
                continue
            gaps = _required_keys(load, expected_keys)
            if gaps:
                diagnostics.extend(
                    _missing_diagnostics("ASBA-LOAD-MISSING-FIELD", gaps, load_ref)
                )
                continue

            target_ref = load.get("target_ref")
            target_type = target_ref.get("object_type") if isinstance(target_ref, Mapping) else None
            target_id = _ref_id(target_ref)
            expected_target_type = str(contract["target_type"])
            target_resolves = False
            target_index: int | None = None
            if target_type == expected_target_type == "Node" and target_id in node_index_by_id:
                target_index = node_index_by_id[target_id]
                target_resolves = True
            elif (
                target_type == expected_target_type == "Element"
                and target_id in element_index_by_id
            ):
                target_index = element_index_by_id[target_id]
                target_resolves = True
            if not target_resolves:
                diagnostics.append(
                    _diagnostic(
                        "ASBA-LOAD-TARGET-UNRESOLVED",
                        "blocking",
                        load_ref,
                        "Canonical load target does not resolve to an adapted node or straight-pipe element.",
                        "Resolve the canonical target before applying load semantics.",
                    )
                )
                continue
            quantity, quantity_diagnostics = _quantity(
                load.get("quantity"),
                str(contract["quantity_dimension"]),
                load_ref,
                "ASBA-LOAD-QUANTITY",
            )
            diagnostics.extend(quantity_diagnostics)
            if quantity is None:
                continue
            direction = load.get("direction")
            if direction not in contract["direction_axes"]:
                diagnostics.append(
                    _diagnostic(
                        "ASBA-LOAD-DIRECTION-UNSUPPORTED",
                        "blocking",
                        load_ref,
                        f"Load direction {direction!r} is not valid for {load_record_type}.",
                        "Use a canonical force or moment direction matching the load record type.",
                    )
                )
                continue

            application = {
                "load_case_id": load_case_id,
                "load_case_index": load_case_index,
                "load_index": index,
                "load_record_type": str(load_record_type),
                "target_type": target_type,
                "target_id": target_id,
                "target_index": target_index,
                "direction": direction,
                "quantity": quantity,
                "source_ref": load_ref,
            }
            if load_record_type == "element_point_force":
                station, station_diagnostics = _quantity(
                    load.get("station_fraction"),
                    "dimensionless",
                    load_ref,
                    "ASBA-LOAD-STATION-FRACTION",
                )
                diagnostics.extend(station_diagnostics)
                if station is None:
                    continue
                application["station_fraction"] = station
            elif load_record_type == "element_uniform_distributed_force":
                span, span_diagnostics = _span(load.get("span"), load_ref)
                diagnostics.extend(span_diagnostics)
                if span is None:
                    continue
                application["span"] = span
            load_applications.append(application)

        emitted_count = len(load_applications) - load_application_start
        diagnostic_count = len(diagnostics) - load_diagnostic_start
        load_case_records.append(
            {
                "load_case_id": load_case_id,
                "load_case_index": load_case_index,
                "name": str(load_case["name"]),
                "load_type": str(load_case["load_type"]),
                "load_count": len(loads),
                "solver_application_status": (
                    "solver_ready"
                    if emitted_count == len(loads) and diagnostic_count == 0
                    else "blocked_by_diagnostics"
                ),
                "source_ref": load_case_ref,
            }
        )
    return load_case_records, load_applications, diagnostics


def _orientation_diagnostics(
    coordinate_system: Mapping[str, Any],
    start_node: Mapping[str, Any] | None,
    end_node: Mapping[str, Any] | None,
    affected_ref: Mapping[str, str],
) -> list[dict[str, Any]]:
    diagnostics: list[dict[str, Any]] = []
    if start_node is None or end_node is None:
        return diagnostics
    axis_x = _derived_axis_x(start_node.get("coordinates"), end_node.get("coordinates"))
    if axis_x is None:
        diagnostics.append(
            _diagnostic(
                "ASBA-ELEMENT-LENGTH-NONFINITE",
                "blocking",
                affected_ref,
                "Straight-pipe adapted node coordinates do not define a finite nonzero local X axis.",
                "Provide distinct finite start and end node coordinates before solver-boundary adaptation.",
            )
        )
        return diagnostics
    y_reference = coordinate_system.get("y_reference")
    if (
        not isinstance(y_reference, list)
        or len(y_reference) != 3
        or any(
            isinstance(component, bool)
            or not isinstance(component, (int, float))
            or not isfinite(float(component))
            for component in y_reference
        )
    ):
        diagnostics.append(
            _diagnostic(
                "ASBA-ELEMENT-Y-REFERENCE-NONFINITE",
                "blocking",
                affected_ref,
                "Straight-pipe y_reference must contain three finite numeric components.",
                "Provide a governed finite y_reference vector.",
            )
        )
        return diagnostics
    y_vector = [float(component) for component in y_reference]
    if _norm(y_vector) <= VECTOR_TOLERANCE:
        diagnostics.append(
            _diagnostic(
                "ASBA-ELEMENT-Y-REFERENCE-ZERO",
                "blocking",
                affected_ref,
                "Straight-pipe y_reference must be nonzero.",
                "Provide a governed nonzero y_reference vector.",
            )
        )
        return diagnostics
    if _norm(_cross(axis_x, y_vector)) <= VECTOR_TOLERANCE:
        diagnostics.append(
            _diagnostic(
                "ASBA-ELEMENT-Y-REFERENCE-PARALLEL",
                "blocking",
                affected_ref,
                "Straight-pipe y_reference must not be parallel to the derived local X axis.",
                "Provide a y_reference vector that defines a stable local orientation with the adapted element axis.",
            )
        )
    return diagnostics


def _derived_axis_x(
    start_coordinates: Any,
    end_coordinates: Any,
) -> list[float] | None:
    if not isinstance(start_coordinates, list) or not isinstance(end_coordinates, list):
        return None
    if len(start_coordinates) != 3 or len(end_coordinates) != 3:
        return None
    delta = [float(end) - float(start) for start, end in zip(start_coordinates, end_coordinates)]
    length = _norm(delta)
    if length <= VECTOR_TOLERANCE:
        return None
    return [component / length for component in delta]


def _span(value: Any, affected_ref: Mapping[str, str]) -> tuple[dict[str, Any] | None, list[dict[str, Any]]]:
    if not isinstance(value, Mapping):
        return None, [
            _diagnostic(
                "ASBA-LOAD-SPAN-MISSING",
                "blocking",
                affected_ref,
                "Element uniform distributed force records require a canonical span mapping.",
                "Provide start_fraction and end_fraction Quantity records.",
            )
        ]
    gaps = _required_keys(value, ("start_fraction", "end_fraction"))
    if gaps:
        return None, _missing_diagnostics("ASBA-LOAD-SPAN-MISSING-FIELD", gaps, affected_ref)
    start, start_diagnostics = _quantity(
        value.get("start_fraction"),
        "dimensionless",
        affected_ref,
        "ASBA-LOAD-SPAN-START-FRACTION",
    )
    end, end_diagnostics = _quantity(
        value.get("end_fraction"),
        "dimensionless",
        affected_ref,
        "ASBA-LOAD-SPAN-END-FRACTION",
    )
    diagnostics = [*start_diagnostics, *end_diagnostics]
    if start is None or end is None:
        return None, diagnostics
    if not (0.0 <= start["value"] <= 1.0 and 0.0 <= end["value"] <= 1.0):
        diagnostics.append(
            _diagnostic(
                "ASBA-LOAD-SPAN-FRACTION-RANGE",
                "blocking",
                affected_ref,
                "Element uniform distributed force span fractions must be in the inclusive range [0, 1].",
                "Provide dimensionless start and end fractions within the element span.",
            )
        )
    if not start["value"] < end["value"]:
        diagnostics.append(
            _diagnostic(
                "ASBA-LOAD-SPAN-FRACTION-ORDER",
                "blocking",
                affected_ref,
                "Element uniform distributed force span requires start_fraction < end_fraction.",
                "Provide an ordered span; JSON Schema cannot compare these sibling values.",
            )
        )
    if diagnostics:
        return None, diagnostics
    return {"start_fraction": start, "end_fraction": end}, []


def _norm(vector: Iterable[float]) -> float:
    return sum(component * component for component in vector) ** 0.5


def _cross(left: list[float], right: list[float]) -> list[float]:
    return [
        left[1] * right[2] - left[2] * right[1],
        left[2] * right[0] - left[0] * right[2],
        left[0] * right[1] - left[1] * right[0],
    ]


def _adapter_dto_identity_records(
    load_applications: Iterable[Mapping[str, Any]],
) -> list[dict[str, Any]]:
    records: list[dict[str, Any]] = []
    for application in load_applications:
        load_case_id = str(application["load_case_id"])
        load_index = int(application["load_index"])
        dto_id = f"dto:load_application:{load_case_id}:{load_index}"
        payload_ref = _ref("AdapterDTO", dto_id)
        payload_hash = _stable_payload_hash(application)
        adapter_anchor = {
            "ref_type": "adapter_dto",
            "ref_id": dto_id,
        }
        solver_input_anchor = {
            "ref_type": "solver_input",
            "ref_id": f"solver_input:load_application:{load_case_id}:{load_index}",
        }
        provenance = _adapter_dto_provenance()
        records.append(
            {
                "dto_id": dto_id,
                "dto_kind": "load_application",
                "source_ref": deepcopy(dict(application["source_ref"])),
                "target_ref": payload_ref,
                "payload_hash_ref": {
                    "algorithm": "sha256",
                    "canonicalization": "JCS",
                    "payload_ref": payload_ref,
                    "value": payload_hash,
                },
                "result_trace_anchor": deepcopy(adapter_anchor),
                "solver_input_trace_anchor": deepcopy(solver_input_anchor),
                "source_chain": [
                    {
                        "trace_id": (
                            "trace:analytical-load-to-adapter-dto:"
                            f"{load_case_id}:{load_index}"
                        ),
                        "trace_type": "analytical_model_to_adapter_dto",
                        "source_ref": deepcopy(dict(application["source_ref"])),
                        "target_ref": deepcopy(adapter_anchor),
                        "provenance": deepcopy(provenance),
                    },
                    {
                        "trace_id": (
                            "trace:adapter-dto-to-solver-input:"
                            f"{load_case_id}:{load_index}"
                        ),
                        "trace_type": "adapter_dto_to_solver_input",
                        "source_ref": deepcopy(adapter_anchor),
                        "target_ref": deepcopy(solver_input_anchor),
                        "provenance": deepcopy(provenance),
                    },
                ],
                "provenance": deepcopy(provenance),
            }
        )
    return sorted(records, key=lambda item: item["dto_id"])


def _adapter_dto_provenance() -> dict[str, str]:
    return {
        "source_name": "OpenPipeStress physical-to-analytical adapter",
        "source_location": (
            "core/model_transform/physical_to_analytical/_solver_boundary_adapter.py"
        ),
        "source_license": "project",
        "contributor": "OpenPipeStress",
        "contributor_certification": (
            "deterministic adapter DTO trace evidence; no solver behavior or "
            "public API expansion"
        ),
        "redistribution_status": "invented_non_engineering_example",
        "review_status": "pending",
    }


def _stable_payload_hash(value: Mapping[str, Any]) -> str:
    encoded = json.dumps(value, sort_keys=True, separators=(",", ":"), allow_nan=False)
    return hashlib.sha256(encoded.encode("utf-8")).hexdigest()


def _required_quantity_properties(
    properties: Any,
    expected_dimensions: Mapping[str, str],
    affected_ref: Mapping[str, str],
    code_prefix: str,
) -> tuple[dict[str, dict[str, Any]], list[dict[str, Any]]]:
    copied: dict[str, dict[str, Any]] = {}
    diagnostics: list[dict[str, Any]] = []
    if not isinstance(properties, Mapping):
        return copied, [
            _diagnostic(
                f"{code_prefix}-PROPERTIES-MISSING",
                "blocking",
                affected_ref,
                "Required solver-boundary property map is missing.",
                "Provide explicit canonical Quantity properties.",
            )
        ]
    for name, expected_dimension in expected_dimensions.items():
        if name not in properties:
            diagnostics.append(
                _diagnostic(
                    f"{code_prefix}-PROPERTY-MISSING",
                    "blocking",
                    affected_ref,
                    f"Required solver-boundary property {name!r} is missing.",
                    "Provide the property explicitly; this adapter does not infer mechanical defaults.",
                )
            )
            continue
        quantity, quantity_diagnostics = _quantity(
            properties.get(name),
            expected_dimension,
            affected_ref,
            f"{code_prefix}-PROPERTY",
        )
        diagnostics.extend(quantity_diagnostics)
        if quantity is not None:
            copied[name] = quantity
    return copied, diagnostics


def _optional_quantity_properties(
    properties: Any,
    expected_dimensions: Mapping[str, str],
    affected_ref: Mapping[str, str],
    code_prefix: str,
) -> tuple[dict[str, dict[str, Any]], list[dict[str, Any]]]:
    copied: dict[str, dict[str, Any]] = {}
    diagnostics: list[dict[str, Any]] = []
    if not isinstance(properties, Mapping):
        return copied, diagnostics
    for name, expected_dimension in expected_dimensions.items():
        if name not in properties:
            continue
        quantity, quantity_diagnostics = _quantity(
            properties.get(name),
            expected_dimension,
            affected_ref,
            f"{code_prefix}-PROPERTY",
        )
        diagnostics.extend(quantity_diagnostics)
        if quantity is not None:
            copied[name] = quantity
    return copied, diagnostics


def _quantity(
    value: Any,
    expected_dimension: str | None,
    affected_ref: Mapping[str, str],
    code_prefix: str,
) -> tuple[dict[str, Any] | None, list[dict[str, Any]]]:
    if not isinstance(value, Mapping):
        return None, [
            _diagnostic(
                f"{code_prefix}-MISSING",
                "blocking",
                affected_ref,
                "Required Quantity mapping is missing.",
                "Provide a canonical Quantity with explicit value, unit, dimension, and provenance.",
            )
        ]
    gaps = _required_keys(value, ("value", "unit", "dimension", "provenance"))
    if gaps:
        return None, _missing_diagnostics(f"{code_prefix}-MISSING-FIELD", gaps, affected_ref)
    raw_number = value.get("value")
    if (
        isinstance(raw_number, bool)
        or not isinstance(raw_number, (int, float))
        or not isfinite(float(raw_number))
    ):
        return None, [
            _diagnostic(
                f"{code_prefix}-NONFINITE",
                "blocking",
                affected_ref,
                "Quantity value must be finite at the solver boundary.",
                "Provide an explicit finite numeric value.",
            )
        ]
    if value.get("unit") in (None, "", "TBD") or value.get("dimension") in (None, "", "TBD"):
        return None, [
            _diagnostic(
                f"{code_prefix}-UNIT-UNRESOLVED",
                "blocking",
                affected_ref,
                "Quantity unit and dimension must be explicit at the solver boundary.",
                "Provide explicit unit metadata; this adapter does not convert or infer units.",
            )
        ]
    if expected_dimension is not None and value.get("dimension") != expected_dimension:
        return None, [
            _diagnostic(
                f"{code_prefix}-DIMENSION-UNSUPPORTED",
                "blocking",
                affected_ref,
                f"Quantity dimension {value.get('dimension')!r} does not match expected dimension {expected_dimension!r}.",
                "Provide a canonical Quantity with the expected dimension; this adapter does not reinterpret dimensions.",
            )
        ]
    return deepcopy(dict(value)), []


def _record_map(records: Any) -> dict[str, Mapping[str, Any]]:
    return {str(record["id"]): record for record in _sorted_records(records) if "id" in record}


def _sorted_records(records: Any) -> list[Mapping[str, Any]]:
    if not isinstance(records, list):
        return []
    return sorted(
        (item for item in records if isinstance(item, Mapping)),
        key=lambda item: str(item.get("id", "")),
    )


def _reference_diagnostics(
    item_id: str | None,
    known_by_id: Mapping[str, Any],
    affected_ref: Mapping[str, str],
    code: str,
    label: str,
) -> list[dict[str, Any]]:
    if item_id is not None and item_id in known_by_id:
        return []
    return [
        _diagnostic(
            code,
            "blocking",
            affected_ref,
            f"Straight-pipe {label} reference does not resolve to an adapted record.",
            "Provide a resolvable canonical reference before adapting to solver-boundary DTOs.",
        )
    ]


def _required_keys(record: Any, keys: Iterable[str]) -> tuple[str, ...]:
    if not isinstance(record, Mapping):
        return tuple(keys)
    return tuple(key for key in keys if key not in record)


def _missing_diagnostics(
    code: str,
    keys: Iterable[str],
    affected_ref: Mapping[str, str],
) -> list[dict[str, Any]]:
    return [
        _diagnostic(
            code,
            "blocking",
            affected_ref,
            f"Required field {key!r} is missing.",
            "Supply the field explicitly; the adapter does not infer hidden defaults.",
        )
        for key in keys
    ]


def _ref_id(value: Any) -> str | None:
    if not isinstance(value, Mapping) or value.get("id") in (None, "", "TBD"):
        return None
    return str(value["id"])


def _ref(object_type: str, item_id: str) -> dict[str, str]:
    return {"object_type": object_type, "id": str(item_id).replace(" ", "-")}


def _diagnostic(
    code: str,
    severity: str,
    affected_ref: Mapping[str, str],
    message: str,
    remediation: str,
) -> dict[str, Any]:
    return {
        "code": code,
        "class": "SOLVE_BLOCKING" if severity == "blocking" else "ASSUMPTION_WARNING",
        "severity": severity,
        "source": ADAPTER_CONTRACT_ID,
        "affected_object": deepcopy(dict(affected_ref)),
        "message": message,
        "remediation": remediation,
    }


def _stable(items: Iterable[Mapping[str, Any]]) -> tuple[dict[str, Any], ...]:
    return tuple(
        deepcopy(dict(item))
        for item in sorted(
            items,
            key=lambda item: (
                str(item.get("code", "")),
                str(item.get("affected_object", {}).get("id", "")),
                str(item.get("message", "")),
            ),
        )
    )


def _result(
    model_ref: Mapping[str, str],
    source_model_ref: Mapping[str, str] | None,
    nodes: Iterable[Mapping[str, Any]],
    straight_pipes: Iterable[Mapping[str, Any]],
    property_bindings: Iterable[Mapping[str, Any]],
    supports: Iterable[Mapping[str, Any]],
    load_case_records: Iterable[Mapping[str, Any]],
    load_applications: Iterable[Mapping[str, Any]],
    adapter_dto_records: Iterable[Mapping[str, Any]],
    load_case_diagnostics: Iterable[Mapping[str, Any]],
    diagnostics: Iterable[Mapping[str, Any]],
) -> SolverBoundaryAdapterResult:
    return SolverBoundaryAdapterResult(
        model_ref=deepcopy(dict(model_ref)),
        source_model_ref=deepcopy(dict(source_model_ref)) if source_model_ref else None,
        nodes=tuple(deepcopy(dict(item)) for item in nodes),
        straight_pipe_connectivity=tuple(deepcopy(dict(item)) for item in straight_pipes),
        property_bindings=tuple(deepcopy(dict(item)) for item in property_bindings),
        support_targets=tuple(deepcopy(dict(item)) for item in supports),
        load_case_records=tuple(deepcopy(dict(item)) for item in load_case_records),
        load_applications=tuple(deepcopy(dict(item)) for item in load_applications),
        adapter_dto_records=tuple(deepcopy(dict(item)) for item in adapter_dto_records),
        load_case_diagnostics=_stable(load_case_diagnostics),
        diagnostics=_stable(diagnostics),
    )
