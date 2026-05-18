"""Deterministic solve execution UX records for DEL-07-07."""

from __future__ import annotations

from copy import deepcopy
import json
from typing import Any, Mapping

from core.gui.pkg02_boundary import (
    analysis_boundary_contract,
    hash_boundary_status,
    normalize_analysis_statuses,
    persistence_hash_contract,
    pkg02_diagnostic,
    source_of_truth_boundary,
)


VALID_STATES = {"queued", "running", "cancelling", "cancelled", "completed", "failed"}
TERMINAL_STATES = {"cancelled", "completed", "failed"}


def build_solve_execution_ux(*, run_panel_id: str, events: list[Mapping[str, Any]]) -> dict[str, Any]:
    diagnostics: list[dict[str, Any]] = []
    timeline = [_event_record(index, item, diagnostics) for index, item in enumerate(events)]
    final_state = timeline[-1]["state"] if timeline else "queued"
    if final_state not in TERMINAL_STATES:
        diagnostics.append(_diagnostic("SOLVE_UX_TERMINAL_STATE_PENDING", "warning", run_panel_id))
    return {
        "schema_version": "0.1.0",
        "deliverable_id": "DEL-07-07",
        "package_id": "PKG-07",
        "scope_item": "SOW-055",
        "objectives": ["OBJ-006", "OBJ-007"],
        "run_panel_id": str(run_panel_id),
        "timeline": timeline,
        "final_state": final_state,
        "cancellation": _cancellation(timeline),
        "diagnostics": sorted(diagnostics, key=canonical_json),
        "solver_execution": "not_performed_by_gui_contract",
        "job_orchestration": "invented_state_transitions_only",
        "software_makes_professional_acceptance_claim": False,
        "analysis_boundary_contract": analysis_boundary_contract(),
        "persistence_hash_contract": persistence_hash_contract(
            gui_hash_scope="solve_progress_display_only_not_project_payload_hash"
        ),
        "source_of_truth_boundary": source_of_truth_boundary(
            surface="DEL-07-07 solve_execution_ux",
            mutation_route="application_service_job_intent_and_result_envelope_only",
        ),
    }


def canonical_json(value: Any) -> str:
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=True)


def _event_record(index: int, event: Mapping[str, Any], diagnostics: list[dict[str, Any]]) -> dict[str, Any]:
    event_id = _text(event.get("event_id"), f"event:{index:04d}")
    state = _text(event.get("state"), "queued")
    if state not in VALID_STATES:
        diagnostics.append(_diagnostic("SOLVE_UX_STATE_UNSUPPORTED", "blocking", event_id))
    progress = event.get("progress_percent", 0)
    if not isinstance(progress, (int, float)) or progress < 0 or progress > 100:
        diagnostics.append(_diagnostic("SOLVE_UX_PROGRESS_INVALID", "blocking", event_id))
        progress = "TBD"
    result_available = bool(event.get("result_available", state == "completed"))
    analysis_status = normalize_analysis_statuses(
        event.get("analysis_status"),
        diagnostics,
        event_id,
        required=result_available,
    )
    model_hashes = deepcopy(_list(event.get("model_hashes")) + _list(event.get("model_state_hashes")))
    result_hashes = deepcopy(_list(event.get("result_hashes")) + _list(event.get("evidence_hashes")))
    all_hashes = model_hashes + result_hashes
    provenance_refs = deepcopy(_list(event.get("provenance_refs")))
    if result_available and not all_hashes:
        diagnostics.append(_diagnostic("SOLVE_UX_HASH_EVIDENCE_MISSING", "warning", event_id))
    if result_available and not provenance_refs:
        diagnostics.append(_diagnostic("SOLVE_UX_PROVENANCE_REF_MISSING", "warning", event_id))
    return {
        "event_id": event_id,
        "sequence": index,
        "state": state,
        "progress_percent": progress,
        "message": _text(event.get("message"), state),
        "diagnostic_refs": deepcopy(_list(event.get("diagnostic_refs"))),
        "warning_refs": deepcopy(_list(event.get("warning_refs"))),
        "analysis_status": analysis_status,
        "analysis_boundary": {
            "contract_ref": "DEL-02-03",
            "human_acceptance_record_policy": "external_hash_bound_human_record_only",
            "human_acceptance_refs": deepcopy(_list(event.get("human_acceptance_refs"))),
        },
        "model_hashes": model_hashes,
        "result_hashes": result_hashes,
        "hash_boundary": hash_boundary_status(all_hashes),
        "provenance_refs": provenance_refs,
        "cancellation_requested": bool(event.get("cancellation_requested", False)),
        "source_ref": deepcopy(event.get("source_ref")),
    }


def _cancellation(timeline: list[Mapping[str, Any]]) -> dict[str, Any]:
    requested = [item for item in timeline if item["cancellation_requested"] or item["state"] in {"cancelling", "cancelled"}]
    return {
        "requested": bool(requested),
        "request_event_ids": [item["event_id"] for item in requested],
        "terminal_state": timeline[-1]["state"] if timeline else "queued",
        "mutates_solver_process_directly": False,
    }


def _diagnostic(code: str, severity: str, target_ref: str) -> dict[str, Any]:
    contract_ref = "DEL-02-05" if "HASH" in code or "PROVENANCE" in code else "DEL-02-03"
    return pkg02_diagnostic(code, severity, target_ref, contract_ref=contract_ref)


def _list(value: Any) -> list[Any]:
    return value if isinstance(value, list) else []


def _text(value: Any, fallback: str) -> str:
    text = str(value).strip() if value is not None else ""
    return text or fallback
