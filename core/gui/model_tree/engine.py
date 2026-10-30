"""Deterministic model-tree/property-inspector records for DEL-07-02."""

from __future__ import annotations

from copy import deepcopy
import json
from typing import Any, Mapping


GUI_PROFESSIONAL_BOUNDARY = {
    "human_review_required": True,
    "software_makes_compliance_claim": False,
    "software_makes_certification_claim": False,
    "software_makes_sealing_claim": False,
    "software_makes_approval_claim": False,
    "software_makes_authentication_claim": False,
}

PROVENANCE = {
    "source_name": "OpenPipeStress DEL-07-02 model tree contract",
    "source_location": "core/gui/model_tree/engine.py",
    "source_license": "project-governed",
    "redistribution_status": "public_permissive",
    "privacy_classification": "public_metadata",
}


def build_model_tree_property_inspector(
    *,
    project_id: str,
    entities: list[Mapping[str, Any]],
    selected_ref: Mapping[str, str] | None = None,
) -> dict[str, Any]:
    """Build a deterministic GUI tree and inspector record.

    The function only shapes invented or already-cleared GUI state. It does not
    mutate project data, persist edits, run solvers, or fill missing values.
    """

    diagnostics: list[dict[str, Any]] = []
    nodes = [_tree_node(entity, diagnostics) for entity in entities]
    nodes = sorted(nodes, key=lambda item: (item["sort_key"], item["node_id"]))
    selection = _selection(selected_ref, nodes, diagnostics)
    inspector = _inspector(selection, entities, diagnostics)

    return {
        "schema_version": "0.1.0",
        "deliverable_id": "DEL-07-02",
        "package_id": "PKG-07",
        "scope_items": ["SOW-020", "SOW-021"],
        "objectives": ["OBJ-006"],
        "project_ref": str(project_id),
        "tree_nodes": nodes,
        "selection": selection,
        "property_inspector": inspector,
        "diagnostics": sorted(diagnostics, key=canonical_json),
        "mutation_policy": "application_service_command_intents_only",
        "professional_boundary": deepcopy(GUI_PROFESSIONAL_BOUNDARY),
        "provenance": deepcopy(PROVENANCE),
    }


def canonical_json(value: Any) -> str:
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=True)


def _tree_node(entity: Mapping[str, Any], diagnostics: list[dict[str, Any]]) -> dict[str, Any]:
    entity_id = _text(entity.get("entity_id"), "entity:TBD")
    entity_type = _text(entity.get("entity_type"), "unknown")
    label = _text(entity.get("label"), entity_id)
    properties = _list(entity.get("properties"))
    if entity_id.endswith(":TBD") or entity_type == "unknown":
        diagnostics.append(_diagnostic("MODEL_TREE_ENTITY_INCOMPLETE", "blocking", entity_id))
    for prop in properties:
        if isinstance(prop, Mapping) and prop.get("value") in (None, "TBD"):
            diagnostics.append(
                _diagnostic("PROPERTY_VALUE_UNRESOLVED", "warning", f"{entity_id}:{prop.get('field_id', 'field:TBD')}")
            )
    return {
        "node_id": f"{entity_type}:{entity_id}",
        "entity_ref": {"ref_type": entity_type, "ref_id": entity_id},
        "label": label,
        "parent_ref": deepcopy(entity.get("parent_ref")),
        "sort_key": _text(entity.get("sort_key"), label),
        "visibility": _text(entity.get("visibility"), "visible"),
        "validation_state": _validation_state(entity, properties),
        "provenance_state": _text(entity.get("provenance_state"), "not_provided"),
        "privacy_classification": _privacy(entity),
    }


def _selection(
    selected_ref: Mapping[str, str] | None,
    nodes: list[Mapping[str, Any]],
    diagnostics: list[dict[str, Any]],
) -> dict[str, Any]:
    if not selected_ref:
        return {"selected": False, "selected_ref": None}
    selected = {"ref_type": _text(selected_ref.get("ref_type"), "unknown"), "ref_id": _text(selected_ref.get("ref_id"), "unknown")}
    known = any(item["entity_ref"] == selected for item in nodes)
    if not known:
        diagnostics.append(_diagnostic("MODEL_TREE_SELECTION_UNKNOWN", "warning", canonical_json(selected)))
    return {"selected": known, "selected_ref": selected}


def _inspector(
    selection: Mapping[str, Any],
    entities: list[Mapping[str, Any]],
    diagnostics: list[dict[str, Any]],
) -> dict[str, Any]:
    selected = selection.get("selected_ref")
    if not selected:
        return {"status": "no_selection", "entity_ref": None, "fields": []}
    match = next(
        (
            item
            for item in entities
            if _text(item.get("entity_id"), "") == selected["ref_id"]
            and _text(item.get("entity_type"), "") == selected["ref_type"]
        ),
        None,
    )
    if match is None:
        return {"status": "selection_unresolved", "entity_ref": selected, "fields": []}
    fields = []
    for prop in _list(match.get("properties")):
        if not isinstance(prop, Mapping):
            diagnostics.append(_diagnostic("PROPERTY_RECORD_NOT_OBJECT", "blocking", selected["ref_id"]))
            continue
        fields.append(
            {
                "field_id": _text(prop.get("field_id"), "field:TBD"),
                "label": _text(prop.get("label"), prop.get("field_id", "field:TBD")),
                "value": deepcopy(prop.get("value", "TBD")),
                "unit": deepcopy(prop.get("unit")),
                "editable": bool(prop.get("editable", False)),
                "source_ref": deepcopy(prop.get("source_ref")),
                "validation_state": _text(prop.get("validation_state"), "unresolved_TBD" if prop.get("value") in (None, "TBD") else "ready_for_service_validation"),
            }
        )
    return {"status": "selection_resolved", "entity_ref": selected, "fields": sorted(fields, key=canonical_json)}


def _validation_state(entity: Mapping[str, Any], properties: list[Any]) -> str:
    if entity.get("validation_state"):
        return str(entity["validation_state"])
    if any(isinstance(item, Mapping) and item.get("value") in (None, "TBD") for item in properties):
        return "blocked_by_unresolved_property"
    return "ready_for_service_validation"


def _diagnostic(code: str, severity: str, target_ref: str) -> dict[str, str]:
    return {"code": code, "severity": severity, "target_ref": target_ref}


def _privacy(value: Mapping[str, Any]) -> str:
    return _text(value.get("privacy_classification"), "invented_public_example")


def _list(value: Any) -> list[Any]:
    return value if isinstance(value, list) else []


def _text(value: Any, fallback: str) -> str:
    text = str(value).strip() if value is not None else ""
    return text or fallback
