"""Physical-to-analytical transform contract for OpenPipeStress.

This module derives a solver-oriented centerline/frame model from a supplied
physical source model. It is intentionally provider-neutral: records are plain
schema-shaped mappings, and incomplete or non-representable inputs become
deterministic diagnostics instead of inferred engineering defaults.
"""

from __future__ import annotations

from copy import deepcopy
from dataclasses import dataclass
from typing import Any, Iterable, Mapping


CENTERLINE_ELEMENT_TYPES = {"straight_pipe", "frame", "rigid"}
SUPPORTED_ANALYTICAL_METADATA_COMPONENT_TYPES = {
    "rigid",
    "valve",
    "flange",
    "reducer",
    "specialty",
}
SUPPORTED_SUPPORT_TYPES = {
    "anchor",
    "guide",
    "line_stop",
    "restraint",
    "spring",
    "gap",
    "friction",
}
UNIT_FIELDS = {"value", "unit", "dimension", "provenance"}
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
}
DOF_AXES = ("UX", "UY", "UZ", "RX", "RY", "RZ")
MODEL_ARRAY_FIELDS = (
    "components",
    "results",
    "unresolved_assumptions",
    "design_knowledge_refs",
    "constraint_refs",
    "equipment_interface_refs",
    "operation_refs",
    "model_state_refs",
    "analysis_run_refs",
    "comparison_refs",
    "handoff_package_refs",
    "external_reference_refs",
)


@dataclass(frozen=True)
class TransformSettings:
    """Stable transform settings carried into deterministic output identity."""

    analytical_model_id: str = "ANALYTICAL-DERIVED"
    analytical_model_name: str = "Derived analytical solver model"
    contract_version: str = "DEL-13-04-0.1"


@dataclass(frozen=True)
class TransformResult:
    """Physical-to-analytical transform result."""

    analytical_model: dict[str, Any]
    diagnostics: tuple[dict[str, Any], ...]
    traceability_links: tuple[dict[str, Any], ...]

    @property
    def has_blocking_findings(self) -> bool:
        return any(item.get("severity") == "blocking" for item in self.diagnostics)

    def to_dict(self) -> dict[str, Any]:
        return {
            "analytical_model": deepcopy(self.analytical_model),
            "diagnostics": [deepcopy(item) for item in self.diagnostics],
            "traceability_links": [deepcopy(item) for item in self.traceability_links],
            "has_blocking_findings": self.has_blocking_findings,
        }


def transform_physical_to_analytical(
    physical_model: Mapping[str, Any] | None,
    settings: TransformSettings | None = None,
) -> TransformResult:
    """Derive a deterministic analytical model from a physical source model.

    The source mapping is never mutated. Records that cannot be represented in
    the project centerline/frame boundary are omitted with traceable diagnostics.
    """

    settings = settings or TransformSettings()
    diagnostics: list[dict[str, Any]] = []
    traceability_links: list[dict[str, Any]] = []

    if not isinstance(physical_model, Mapping):
        diagnostic = _diagnostic(
            "PTA-SOURCE-MODEL-MISSING",
            "blocking",
            "SOLVE_BLOCKING",
            _ref("Model", "UNRESOLVED-SOURCE"),
            "Physical-to-analytical transform requires a supplied physical model.",
            "Provide a schema-backed physical_source_of_truth model before deriving an analytical solver model.",
        )
        diagnostics.append(diagnostic)
        analytical_model = _empty_model(settings, "UNRESOLVED-SOURCE", diagnostics, traceability_links)
        return _result(analytical_model, diagnostics, traceability_links)

    source_model_id = str(physical_model.get("id", "UNRESOLVED-SOURCE"))
    source_model_ref = _ref("Model", source_model_id)
    if physical_model.get("model_role") != "physical_source_of_truth":
        diagnostics.append(
            _diagnostic(
                "PTA-SOURCE-ROLE-UNEXPECTED",
                "warning",
                "ASSUMPTION_WARNING",
                source_model_ref,
                "Source model role is not physical_source_of_truth.",
                "Confirm the editable physical model source of truth before relying on the derived analytical view.",
            )
        )

    coordinate_system = deepcopy(
        physical_model.get("coordinate_system")
        if isinstance(physical_model.get("coordinate_system"), Mapping)
        else {"type": "TBD", "axes": ["X", "Y", "Z"]}
    )
    if coordinate_system.get("type") == "TBD":
        diagnostics.append(
            _diagnostic(
                "PTA-COORDINATE-SYSTEM-MISSING",
                "blocking",
                "SOLVE_BLOCKING",
                source_model_ref,
                "Physical model coordinate_system is missing or unresolved.",
                "Supply an explicit global coordinate system; the transform does not infer one.",
            )
        )

    nodes, node_diags, node_links = _copy_nodes(physical_model.get("nodes"), source_model_ref)
    diagnostics.extend(node_diags)
    traceability_links.extend(node_links)
    node_ids = {node["id"] for node in nodes}

    materials, material_diags, material_links = _copy_records_with_units(
        "Material",
        physical_model.get("materials"),
        "properties",
        "PTA-MATERIAL",
    )
    diagnostics.extend(material_diags)
    traceability_links.extend(material_links)
    material_ids = {item["id"] for item in materials}

    sections, section_diags, section_links = _copy_records_with_units(
        "Section",
        physical_model.get("sections"),
        "properties",
        "PTA-SECTION",
    )
    diagnostics.extend(section_diags)
    traceability_links.extend(section_links)
    section_ids = {item["id"] for item in sections}

    components_by_id, component_diags, component_links = _classify_components(
        physical_model.get("components")
    )
    diagnostics.extend(component_diags)
    traceability_links.extend(component_links)

    elements, referenced_component_ids, element_diags, element_links = _copy_elements(
        physical_model.get("elements"),
        components_by_id,
        node_ids,
        material_ids,
        section_ids,
    )
    diagnostics.extend(element_diags)
    traceability_links.extend(element_links)
    element_ids = {item["id"] for item in elements}
    components = [
        deepcopy(component)
        for component_id, component in sorted(components_by_id.items())
        if component_id in referenced_component_ids
        and component.get("component_type") in SUPPORTED_ANALYTICAL_METADATA_COMPONENT_TYPES
    ]

    supports, support_diags, support_links = _copy_supports(
        physical_model.get("supports"),
        node_ids,
        element_ids,
    )
    diagnostics.extend(support_diags)
    traceability_links.extend(support_links)

    load_cases, load_diags, load_links = _copy_load_cases(
        physical_model.get("load_cases"),
        node_ids,
        element_ids,
    )
    diagnostics.extend(load_diags)
    traceability_links.extend(load_links)

    combinations = _sorted_deepcopy_records(physical_model.get("combinations"))
    for combination in combinations:
        traceability_links.append(_trace_link("Combination", combination["id"]))

    analytical_model = {
        "id": settings.analytical_model_id,
        "name": settings.analytical_model_name,
        "description": (
            f"Derived centerline/frame analytical view from physical model {source_model_id}; "
            f"contract {settings.contract_version}."
        ),
        "model_role": "analytical_solver_model",
        "source_model_ref": source_model_ref,
        "coordinate_system": coordinate_system,
        "nodes": nodes,
        "elements": elements,
        "components": components,
        "materials": materials,
        "sections": sections,
        "supports": supports,
        "load_cases": load_cases,
        "combinations": combinations,
        "results": [],
        "diagnostics": _stable_diagnostics(diagnostics),
        "unresolved_assumptions": deepcopy(physical_model.get("unresolved_assumptions", [])),
        "traceability_links": [],
        "design_knowledge_refs": deepcopy(physical_model.get("design_knowledge_refs", [])),
        "constraint_refs": deepcopy(physical_model.get("constraint_refs", [])),
        "equipment_interface_refs": deepcopy(physical_model.get("equipment_interface_refs", [])),
        "operation_refs": deepcopy(physical_model.get("operation_refs", [])),
        "model_state_refs": deepcopy(physical_model.get("model_state_refs", [])),
        "analysis_run_refs": [],
        "comparison_refs": [],
        "handoff_package_refs": [],
        "external_reference_refs": [],
        "provenance": _transform_provenance(source_model_id),
    }
    analytical_model["traceability_links"] = _stable_traceability(traceability_links, analytical_model["diagnostics"])

    return _result(analytical_model, diagnostics, analytical_model["traceability_links"])


def transform_dict(result: TransformResult) -> dict[str, Any]:
    return result.to_dict()


def _copy_nodes(records: Any, source_model_ref: Mapping[str, str]) -> tuple[list[dict[str, Any]], list[dict[str, Any]], list[dict[str, Any]]]:
    diagnostics: list[dict[str, Any]] = []
    links: list[dict[str, Any]] = []
    nodes: list[dict[str, Any]] = []
    if not isinstance(records, list) or not records:
        diagnostics.append(
            _diagnostic(
                "PTA-NODES-MISSING",
                "blocking",
                "SOLVE_BLOCKING",
                source_model_ref,
                "Physical model contains no usable nodes for centerline/frame analysis.",
                "Supply explicit nodes with coordinates and degree-of-freedom states.",
            )
        )
        return nodes, diagnostics, links
    for record in _sorted_records(records):
        node_id = str(record.get("id", "UNRESOLVED-NODE"))
        node_ref = _ref("Node", node_id)
        missing = _missing_fields(record, ("id", "coordinates", "degrees_of_freedom"))
        unit_gaps = _quantity_gaps(record.get("coordinates"), node_ref, "PTA-NODE-COORDINATE-UNIT")
        dof_gaps = _dof_gaps(record.get("degrees_of_freedom"), node_ref)
        if missing or unit_gaps or dof_gaps:
            diagnostics.extend(_missing_field_diagnostics("PTA-NODE-MISSING-FIELD", missing, node_ref))
            diagnostics.extend(unit_gaps)
            diagnostics.extend(dof_gaps)
            links.append(_trace_link("Node", node_id, target_type="Diagnostic"))
            continue
        nodes.append(deepcopy(record))
        links.append(_trace_link("Node", node_id))
    return nodes, diagnostics, links


def _copy_records_with_units(
    object_type: str,
    records: Any,
    quantity_field: str,
    code_prefix: str,
) -> tuple[list[dict[str, Any]], list[dict[str, Any]], list[dict[str, Any]]]:
    diagnostics: list[dict[str, Any]] = []
    links: list[dict[str, Any]] = []
    copied: list[dict[str, Any]] = []
    for record in _sorted_records(records):
        record_id = str(record.get("id", f"UNRESOLVED-{object_type.upper()}"))
        record_ref = _ref(object_type, record_id)
        unit_gaps = _quantity_gaps(record.get(quantity_field), record_ref, f"{code_prefix}-UNIT")
        if unit_gaps:
            diagnostics.extend(unit_gaps)
            links.append(_trace_link(object_type, record_id, target_type="Diagnostic"))
            continue
        copied.append(deepcopy(record))
        links.append(_trace_link(object_type, record_id))
    return copied, diagnostics, links


def _classify_components(
    records: Any,
) -> tuple[dict[str, dict[str, Any]], list[dict[str, Any]], list[dict[str, Any]]]:
    diagnostics: list[dict[str, Any]] = []
    links: list[dict[str, Any]] = []
    components_by_id: dict[str, dict[str, Any]] = {}
    for record in _sorted_records(records):
        component_id = str(record.get("id", "UNRESOLVED-COMPONENT"))
        component_ref = _ref("Component", component_id)
        missing = _missing_fields(
            record,
            ("id", "component_type", "geometry", "mechanics_modifiers", "provenance"),
        )
        if missing:
            diagnostics.extend(
                _missing_field_diagnostics(
                    "PTA-COMPONENT-MISSING-FIELD",
                    missing,
                    component_ref,
                )
            )
            links.append(_trace_link("Component", component_id, target_type="Diagnostic"))
            continue
        unit_gaps = _quantity_gaps(record.get("geometry"), component_ref, "PTA-COMPONENT-GEOMETRY-UNIT")
        if unit_gaps:
            diagnostics.extend(unit_gaps)
            links.append(_trace_link("Component", component_id, target_type="Diagnostic"))
            continue
        component_type = record.get("component_type")
        if component_type not in SUPPORTED_ANALYTICAL_METADATA_COMPONENT_TYPES:
            diagnostics.append(
                _diagnostic(
                    "PTA-COMPONENT-TYPE-UNSUPPORTED",
                    "warning",
                    "ASSUMPTION_WARNING",
                    component_ref,
                    f"Component type {component_type!r} is not represented in solver-ready analytical output by this tranche.",
                    "Keep the component in the physical source model or add a later governed analytical representation before solving it.",
                )
            )
            links.append(_trace_link("Component", component_id, target_type="Diagnostic"))
            continue
        components_by_id[component_id] = deepcopy(record)
        links.append(_trace_link("Component", component_id))
        links.extend(
            _field_scalar_trace_links(
                "Component",
                component_id,
                collection_name="components",
                container_name="geometry",
                fields=record.get("geometry"),
            )
        )
    return components_by_id, diagnostics, links


def _copy_elements(
    records: Any,
    components_by_id: Mapping[str, Mapping[str, Any]],
    node_ids: set[str],
    material_ids: set[str],
    section_ids: set[str],
) -> tuple[list[dict[str, Any]], set[str], list[dict[str, Any]], list[dict[str, Any]]]:
    diagnostics: list[dict[str, Any]] = []
    links: list[dict[str, Any]] = []
    elements: list[dict[str, Any]] = []
    referenced_component_ids: set[str] = set()
    for record in _sorted_records(records):
        element_id = str(record.get("id", "UNRESOLVED-ELEMENT"))
        element_ref = _ref("Element", element_id)
        element_type = record.get("element_type")
        component_ref = record.get("component_ref")
        component_id = component_ref.get("id") if isinstance(component_ref, Mapping) else None
        missing = _missing_fields(
            record,
            ("id", "element_type", "start_node_ref", "end_node_ref", "material_ref", "section_ref", "local_coordinate_system", "result_stations"),
        )
        blockers: list[dict[str, Any]] = _missing_field_diagnostics("PTA-ELEMENT-MISSING-FIELD", missing, element_ref)
        if element_type not in CENTERLINE_ELEMENT_TYPES:
            if element_type == "component_link" and str(component_id) in components_by_id:
                pass
            else:
                blockers.append(
                    _diagnostic(
                        "PTA-ELEMENT-TYPE-UNSUPPORTED",
                        "warning",
                        "ASSUMPTION_WARNING",
                        element_ref,
                        f"Element type {element_type!r} is outside the centerline/frame transform boundary.",
                        "Represent the item as supported centerline/frame data or keep it in the physical model only.",
                    )
                )
        if isinstance(component_ref, Mapping):
            if not component_id or str(component_id) not in components_by_id:
                blockers.append(
                    _diagnostic(
                        "PTA-ELEMENT-COMPONENT-UNSUPPORTED",
                        "blocking",
                        "SOLVE_BLOCKING",
                        element_ref,
                        "Element component_ref does not resolve to a supported analytical metadata component.",
                        "Do not approximate unsupported physical components; provide a supported component record or leave the element out of solver-ready output.",
                    )
                )
        blockers.extend(_reference_gap(record.get("start_node_ref"), node_ids, "Node", "PTA-ELEMENT-START-NODE-UNRESOLVED", element_ref))
        blockers.extend(_reference_gap(record.get("end_node_ref"), node_ids, "Node", "PTA-ELEMENT-END-NODE-UNRESOLVED", element_ref))
        blockers.extend(_reference_gap(record.get("material_ref"), material_ids, "Material", "PTA-ELEMENT-MATERIAL-UNRESOLVED", element_ref))
        blockers.extend(_reference_gap(record.get("section_ref"), section_ids, "Section", "PTA-ELEMENT-SECTION-UNRESOLVED", element_ref))
        blockers.extend(_quantity_gaps(record.get("result_stations"), element_ref, "PTA-ELEMENT-STATION-UNIT"))
        if blockers:
            diagnostics.extend(blockers)
            links.append(_trace_link("Element", element_id, target_type="Diagnostic"))
            continue
        elements.append(deepcopy(record))
        if component_id is not None:
            referenced_component_ids.add(str(component_id))
        links.append(_trace_link("Element", element_id))
    return elements, referenced_component_ids, diagnostics, links


def _copy_supports(records: Any, node_ids: set[str], element_ids: set[str]) -> tuple[list[dict[str, Any]], list[dict[str, Any]], list[dict[str, Any]]]:
    diagnostics: list[dict[str, Any]] = []
    links: list[dict[str, Any]] = []
    supports: list[dict[str, Any]] = []
    for record in _sorted_records(records):
        support_id = str(record.get("id", "UNRESOLVED-SUPPORT"))
        support_ref = _ref("Support", support_id)
        blockers = _missing_field_diagnostics(
            "PTA-SUPPORT-MISSING-FIELD",
            _missing_fields(record, ("id", "support_type", "target_ref", "directions", "properties", "provenance")),
            support_ref,
        )
        if record.get("support_type") not in SUPPORTED_SUPPORT_TYPES:
            blockers.append(
                _diagnostic(
                    "PTA-SUPPORT-TYPE-UNSUPPORTED",
                    "warning",
                    "NONLINEAR_WARNING",
                    support_ref,
                    f"Support type {record.get('support_type')!r} is not represented by this transform contract.",
                    "Map the support to a supported centerline/frame support record or retain it as physical-only data.",
                )
            )
        target_ref = record.get("target_ref")
        target_type = target_ref.get("object_type") if isinstance(target_ref, Mapping) else None
        target_ids = node_ids if target_type == "Node" else element_ids if target_type == "Element" else set()
        blockers.extend(_reference_gap(target_ref, target_ids, str(target_type), "PTA-SUPPORT-TARGET-UNRESOLVED", support_ref))
        blockers.extend(_quantity_gaps(record.get("properties"), support_ref, "PTA-SUPPORT-PROPERTY-UNIT"))
        if blockers:
            diagnostics.extend(blockers)
            links.append(_trace_link("Support", support_id, target_type="Diagnostic"))
            continue
        supports.append(deepcopy(record))
        links.append(_trace_link("Support", support_id))
    return supports, diagnostics, links


def _copy_load_cases(records: Any, node_ids: set[str], element_ids: set[str]) -> tuple[list[dict[str, Any]], list[dict[str, Any]], list[dict[str, Any]]]:
    diagnostics: list[dict[str, Any]] = []
    links: list[dict[str, Any]] = []
    load_cases: list[dict[str, Any]] = []
    for record in _sorted_records(records):
        load_case_id = str(record.get("id", "UNRESOLVED-LOADCASE"))
        load_case_ref = _ref("LoadCase", load_case_id)
        blockers = _missing_field_diagnostics(
            "PTA-LOADCASE-MISSING-FIELD",
            _missing_fields(record, ("id", "name", "load_type", "loads", "provenance")),
            load_case_ref,
        )
        for index, load in enumerate(record.get("loads", []) if isinstance(record.get("loads"), list) else []):
            load_ref = _ref("LoadCase", f"{load_case_id}:load:{index}")
            target_ref = load.get("target_ref") if isinstance(load, Mapping) else None
            target_type = target_ref.get("object_type") if isinstance(target_ref, Mapping) else None
            target_ids = node_ids if target_type == "Node" else element_ids if target_type == "Element" else set()
            blockers.extend(_reference_gap(target_ref, target_ids, str(target_type), "PTA-LOAD-TARGET-UNRESOLVED", load_ref))
            blockers.extend(_quantity_gaps(load.get("quantity") if isinstance(load, Mapping) else None, load_ref, "PTA-LOAD-QUANTITY-UNIT"))
        if blockers:
            diagnostics.extend(blockers)
            links.append(_trace_link("LoadCase", load_case_id, target_type="Diagnostic"))
            continue
        load_cases.append(deepcopy(record))
        links.append(_trace_link("LoadCase", load_case_id))
    return load_cases, diagnostics, links


def _quantity_gaps(value: Any, affected_ref: Mapping[str, str], code: str) -> list[dict[str, Any]]:
    diagnostics: list[dict[str, Any]] = []
    quantity_shaped = (
        isinstance(value, Mapping)
        and "value" in value
        and bool({"unit", "dimension", "provenance"} & set(value))
    )
    if quantity_shaped:
        if not UNIT_FIELDS <= set(value) or value.get("unit") in (None, "", "TBD") or value.get("dimension") in (None, "", "TBD"):
            diagnostics.append(_unit_diagnostic(code, affected_ref))
        elif value.get("dimension") not in CANONICAL_DIMENSIONS:
            diagnostics.append(
                _diagnostic(
                    "PTA-UNIT-DIMENSION-UNKNOWN",
                    "blocking",
                    "SOLVE_BLOCKING",
                    affected_ref,
                    f"Quantity dimension {value.get('dimension')!r} is not in the accepted PKG-02 dimension vocabulary.",
                    "Use accepted dimension identifiers such as second_moment_area, linear_stiffness, rotational_stiffness, slope, or TBD for unresolved values.",
                )
            )
        return diagnostics
    if isinstance(value, Mapping):
        for item in value.values():
            diagnostics.extend(_quantity_gaps(item, affected_ref, code))
    elif isinstance(value, list):
        for item in value:
            diagnostics.extend(_quantity_gaps(item, affected_ref, code))
    return diagnostics


def _dof_gaps(value: Any, affected_ref: Mapping[str, str]) -> list[dict[str, Any]]:
    if not isinstance(value, Mapping):
        return [
            _diagnostic(
                "PTA-NODE-DOF-MISSING",
                "blocking",
                "SOLVE_BLOCKING",
                affected_ref,
                "Node degree-of-freedom states are missing.",
                "Supply UX, UY, UZ, RX, RY, and RZ states explicitly; no restraint defaults are inferred.",
            )
        ]
    diagnostics = []
    for axis in DOF_AXES:
        if value.get(axis) in (None, "", "TBD"):
            diagnostics.append(
                _diagnostic(
                    "PTA-NODE-DOF-UNRESOLVED",
                    "blocking",
                    "SOLVE_BLOCKING",
                    affected_ref,
                    f"Node degree-of-freedom state {axis} is missing or TBD.",
                    "Supply explicit degree-of-freedom states; no support or freedom defaults are inferred.",
                )
            )
    return diagnostics


def _reference_gap(ref: Any, known_ids: set[str], object_type: str, code: str, affected_ref: Mapping[str, str]) -> list[dict[str, Any]]:
    if not isinstance(ref, Mapping) or not ref.get("id") or ref.get("id") not in known_ids:
        return [
            _diagnostic(
                code,
                "blocking",
                "SOLVE_BLOCKING",
                affected_ref,
                f"Referenced {object_type} record is missing or omitted from the analytical model.",
                "Supply a resolvable source record or keep the dependent record out of solver-ready output.",
            )
        ]
    return []


def _empty_model(
    settings: TransformSettings,
    source_model_id: str,
    diagnostics: Iterable[dict[str, Any]],
    traceability_links: Iterable[dict[str, Any]],
) -> dict[str, Any]:
    model = {
        "id": settings.analytical_model_id,
        "name": settings.analytical_model_name,
        "model_role": "analytical_solver_model",
        "source_model_ref": _ref("Model", source_model_id),
        "coordinate_system": {"type": "TBD", "axes": ["X", "Y", "Z"]},
        "nodes": [],
        "elements": [],
        "components": [],
        "materials": [],
        "sections": [],
        "supports": [],
        "load_cases": [],
        "combinations": [],
        "results": [],
        "diagnostics": _stable_diagnostics(diagnostics),
        "unresolved_assumptions": [],
        "traceability_links": [],
        "provenance": _transform_provenance(source_model_id),
    }
    for field in MODEL_ARRAY_FIELDS:
        model.setdefault(field, [])
    model["traceability_links"] = _stable_traceability(traceability_links, model["diagnostics"])
    return model


def _missing_fields(record: Mapping[str, Any], fields: Iterable[str]) -> tuple[str, ...]:
    return tuple(field for field in fields if field not in record)


def _missing_field_diagnostics(code: str, fields: Iterable[str], affected_ref: Mapping[str, str]) -> list[dict[str, Any]]:
    return [
        _diagnostic(
            code,
            "blocking",
            "SOLVE_BLOCKING",
            affected_ref,
            f"Required transform field {field!r} is missing.",
            "Supply the field explicitly in the physical source model; the transform does not infer missing solve data.",
        )
        for field in fields
    ]


def _unit_diagnostic(code: str, affected_ref: Mapping[str, str]) -> dict[str, Any]:
    return _diagnostic(
        code,
        "blocking",
        "SOLVE_BLOCKING",
        affected_ref,
        "Unit-bearing value has missing, empty, or TBD unit metadata.",
        "Supply explicit unit and dimension metadata, or mark the quantity as dimensionless when applicable.",
    )


def _diagnostic(
    code: str,
    severity: str,
    diagnostic_class: str,
    affected_ref: Mapping[str, str],
    message: str,
    remediation: str,
) -> dict[str, Any]:
    return {
        "code": code,
        "class": diagnostic_class,
        "severity": severity,
        "source": "DEL-13-04 physical-to-analytical transform contract",
        "affected_object": dict(affected_ref),
        "message": message,
        "remediation": remediation,
        "provenance": _transform_provenance(str(affected_ref.get("id", "UNRESOLVED"))),
    }


def _trace_link(
    source_type: str,
    source_id: str,
    target_type: str | None = None,
) -> dict[str, Any]:
    target_object_type = target_type or source_type
    target_id = source_id if target_object_type != "Diagnostic" else f"DIAG-{source_type}-{source_id}"
    return {
        "id": f"TRACE-PTA-{source_type}-{source_id}".replace("_", "-"),
        "trace_type": "physical_to_analytical",
        "source_ref": _ref(source_type, source_id),
        "target_ref": _ref(target_object_type, target_id),
        "diagnostics": [],
        "provenance": _transform_provenance(source_id),
    }


def _field_scalar_trace_links(
    source_type: str,
    source_id: str,
    *,
    collection_name: str,
    container_name: str,
    fields: Any,
) -> list[dict[str, Any]]:
    if not isinstance(fields, Mapping):
        return []
    links: list[dict[str, Any]] = []
    for field_name, quantity in sorted(fields.items(), key=lambda item: str(item[0])):
        if not isinstance(quantity, Mapping) or not {
            "value",
            "unit",
            "dimension",
            "provenance",
        } <= set(quantity):
            continue
        scalar_path = (
            f"{collection_name}[id={source_id}].{container_name}.{field_name}.value"
        )
        link = _trace_link(source_type, source_id)
        link["id"] = (
            f"TRACE-PTA-FIELD-{source_type}-{source_id}-{container_name}-{field_name}"
            .replace("_", "-")
        )
        link["source_field_path"] = scalar_path
        link["target_field_path"] = scalar_path
        links.append(link)
    return links


def _stable_diagnostics(items: Iterable[Mapping[str, Any]]) -> list[dict[str, Any]]:
    return [deepcopy(item) for item in sorted(items, key=lambda item: (str(item.get("code")), str(item.get("affected_object", {}).get("id")), str(item.get("message"))))]


def _stable_traceability(
    links: Iterable[Mapping[str, Any]],
    diagnostics: Iterable[Mapping[str, Any]],
) -> list[dict[str, Any]]:
    diagnostic_by_target = {
        str(item.get("affected_object", {}).get("id")): deepcopy(item)
        for item in diagnostics
    }
    stable = []
    for link in sorted(links, key=lambda item: str(item.get("id"))):
        copied = deepcopy(link)
        source_id = str(copied.get("source_ref", {}).get("id"))
        diagnostic = diagnostic_by_target.get(source_id)
        if copied.get("target_ref", {}).get("object_type") == "Diagnostic" and diagnostic:
            copied["diagnostics"] = [diagnostic]
        stable.append(copied)
    return stable


def _sorted_records(records: Any) -> list[Mapping[str, Any]]:
    if not isinstance(records, list):
        return []
    return sorted(
        (item for item in records if isinstance(item, Mapping)),
        key=lambda item: str(item.get("id", "")),
    )


def _sorted_deepcopy_records(records: Any) -> list[dict[str, Any]]:
    return [deepcopy(record) for record in _sorted_records(records)]


def _ref(object_type: str, item_id: str) -> dict[str, str]:
    safe_id = str(item_id).replace(" ", "-")
    return {"object_type": object_type if object_type in {"Project", "Model", "Node", "Element", "Component", "Material", "Section", "Support", "LoadCase", "Combination", "Diagnostic"} else "TBD", "id": safe_id}


def _transform_provenance(source_id: str) -> dict[str, str]:
    return {
        "source_name": "DEL-13-04 physical-to-analytical transform contract",
        "source_location": f"physical-model:{source_id}",
        "source_license": "public_permissive",
        "contributor": "OpenPipeStress transform contract",
        "contributor_certification": "schema contract implementation without engineering authority claims",
        "redistribution_status": "public_permissive",
        "review_status": "pending",
    }


def _result(
    analytical_model: Mapping[str, Any],
    diagnostics: Iterable[Mapping[str, Any]],
    traceability_links: Iterable[Mapping[str, Any]],
) -> TransformResult:
    return TransformResult(
        analytical_model=deepcopy(dict(analytical_model)),
        diagnostics=tuple(_stable_diagnostics(diagnostics)),
        traceability_links=tuple(_stable_traceability(traceability_links, diagnostics)),
    )
