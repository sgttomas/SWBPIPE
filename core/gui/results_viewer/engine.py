"""Deterministic result-viewer records for DEL-07-05."""

from __future__ import annotations

from copy import deepcopy
import json
from math import isfinite
from typing import Any, Mapping

from core.gui.pkg02_boundary import (
    analysis_boundary_contract,
    hash_boundary_status,
    normalize_analysis_statuses,
    persistence_hash_contract,
    pkg02_diagnostic,
    quantity_metadata,
    source_of_truth_boundary,
    unit_contract,
)


RESULT_KINDS = {"displacement", "rotation", "force", "moment", "reaction", "stress", "ratio"}


def build_results_viewer_contract(*, result_set_id: str, result_items: list[Mapping[str, Any]]) -> dict[str, Any]:
    diagnostics: list[dict[str, Any]] = []
    views = [_result_view(item, diagnostics) for item in result_items]
    return {
        "schema_version": "0.1.0",
        "deliverable_id": "DEL-07-05",
        "package_id": "PKG-07",
        "scope_item": "SOW-023",
        "objectives": ["OBJ-006", "OBJ-007"],
        "result_set_id": str(result_set_id),
        "views": sorted(views, key=canonical_json),
        "diagnostics": sorted(diagnostics, key=canonical_json),
        "viewer_policy": "tabular_and_overlay_descriptors_only",
        "solver_execution": "not_performed",
        "software_makes_professional_acceptance_claim": False,
        "unit_contract": unit_contract(),
        "analysis_boundary_contract": analysis_boundary_contract(),
        "persistence_hash_contract": persistence_hash_contract(
            gui_hash_scope="result_view_descriptor_only_not_project_payload_hash"
        ),
        "source_of_truth_boundary": source_of_truth_boundary(
            surface="DEL-07-05 results_viewer_contract",
            mutation_route="read_only_result_envelope_projection",
        ),
    }


def canonical_json(value: Any) -> str:
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=True)


def _result_view(item: Mapping[str, Any], diagnostics: list[dict[str, Any]]) -> dict[str, Any]:
    result_id = _text(item.get("result_id"), "result:TBD")
    kind = _text(item.get("result_kind"), "unknown")
    if kind not in RESULT_KINDS:
        diagnostics.append(_diagnostic("RESULT_KIND_UNSUPPORTED", "blocking", result_id))
    values = [_value_record(result_id, value, diagnostics) for value in _list(item.get("values"))]
    if not values:
        diagnostics.append(_diagnostic("RESULT_VALUES_MISSING", "blocking", result_id))
    availability = "available" if values and not _has_tbd(values) else "unresolved_TBD"
    analysis_status = normalize_analysis_statuses(
        item.get("analysis_status"),
        diagnostics,
        result_id,
        required=availability == "available",
    )
    hashes = _hashes_for(item)
    provenance_refs = deepcopy(_list(item.get("provenance_refs")))
    value_provenance_refs = [value.get("provenance_ref") for value in values if value.get("provenance_ref")]
    if availability == "available" and not hashes:
        diagnostics.append(_diagnostic("RESULT_HASH_EVIDENCE_MISSING", "warning", result_id))
    if availability == "available" and not provenance_refs and not value_provenance_refs:
        diagnostics.append(_diagnostic("RESULT_PROVENANCE_REF_MISSING", "warning", result_id))
    return {
        "result_id": result_id,
        "result_kind": kind,
        "source_ref": deepcopy(item.get("source_ref")),
        "analysis_status": analysis_status,
        "analysis_boundary": {
            "contract_ref": "DEL-02-03",
            "human_acceptance_record_policy": "external_hash_bound_human_record_only",
            "human_acceptance_refs": deepcopy(_list(item.get("human_acceptance_refs"))),
        },
        "hashes": hashes,
        "hash_boundary": hash_boundary_status(hashes),
        "provenance_refs": provenance_refs,
        "diagnostic_refs": deepcopy(_list(item.get("diagnostic_refs"))),
        "table_columns": _columns(values),
        "values": values,
        "overlay_descriptor": {
            "enabled": bool(item.get("overlay_enabled", False)),
            "target_ref": deepcopy(item.get("overlay_target_ref")),
            "style_token": _text(item.get("style_token"), "TBD"),
        },
        "availability": availability,
    }


def _value_record(result_id: str, value: Any, diagnostics: list[dict[str, Any]]) -> dict[str, Any]:
    if not isinstance(value, Mapping):
        diagnostics.append(_diagnostic("RESULT_VALUE_NOT_OBJECT", "blocking", result_id))
        return {"value_id": "value:TBD", "numeric_value": "TBD", "unit": "TBD", "dimension": "TBD"}
    value_id = _text(value.get("value_id"), "value:TBD")
    numeric = value.get("numeric_value", "TBD")
    unit = value.get("unit", "TBD")
    dimension = value.get("dimension", "TBD")
    if numeric == "TBD" or unit in (None, "TBD"):
        diagnostics.append(_diagnostic("RESULT_VALUE_OR_UNIT_UNRESOLVED", "warning", result_id))
    if isinstance(numeric, (int, float)) and not isfinite(float(numeric)):
        diagnostics.append(_diagnostic("RESULT_VALUE_NONFINITE", "blocking", result_id))
    unit_metadata = quantity_metadata(
        value=numeric,
        unit=unit,
        dimension=dimension,
        diagnostics=diagnostics,
        target_ref=f"{result_id}:{value_id}",
        code_prefix="RESULT",
    )
    return {
        "value_id": value_id,
        "component": _text(value.get("component"), "scalar"),
        "numeric_value": numeric,
        "unit": unit,
        "dimension": dimension,
        "unit_metadata": unit_metadata,
        "location_ref": deepcopy(value.get("location_ref")),
        "provenance_ref": deepcopy(value.get("provenance_ref")),
    }


def _columns(values: list[Mapping[str, Any]]) -> list[str]:
    columns = {"value_id", "component", "numeric_value", "unit", "dimension"}
    if any(item.get("location_ref") for item in values):
        columns.add("location_ref")
    return sorted(columns)


def _has_tbd(values: list[Mapping[str, Any]]) -> bool:
    return any(item.get("numeric_value") == "TBD" or item.get("unit") in (None, "TBD") for item in values)


def _diagnostic(code: str, severity: str, target_ref: str) -> dict[str, Any]:
    contract_ref = "DEL-02-05" if "HASH" in code or "PROVENANCE" in code else "DEL-02-03"
    return pkg02_diagnostic(code, severity, target_ref, contract_ref=contract_ref)


def _hashes_for(item: Mapping[str, Any]) -> list[Any]:
    hashes: list[Any] = []
    for key in ("hashes", "result_hashes", "evidence_hashes"):
        hashes.extend(deepcopy(_list(item.get(key))))
    return hashes


def _list(value: Any) -> list[Any]:
    return value if isinstance(value, list) else []


def _text(value: Any, fallback: str) -> str:
    text = str(value).strip() if value is not None else ""
    return text or fallback
