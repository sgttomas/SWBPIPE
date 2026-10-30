"""Deterministic design-authoring and comparison workspace records for DEL-07-08.

The workspace is a GUI-facing composition layer. It consumes already-available
contract records and returns inspectable panel state; it does not mutate
accepted model state, execute solvers, or turn comparison output into
professional acceptance.
"""

from __future__ import annotations

from copy import deepcopy
import hashlib
import json
from typing import Any, Mapping

from core.gui.pkg02_boundary import (
    analysis_boundary_contract,
    hash_boundary_status,
    normalize_analysis_statuses,
    persistence_hash_contract,
    pkg02_diagnostic,
)


WORKSPACE_VERSION = "0.1.0"

PROFESSIONAL_BOUNDARY = {
    "human_review_required": True,
    "software_makes_compliance_claim": False,
    "software_makes_certification_claim": False,
    "software_makes_sealing_claim": False,
    "software_makes_approval_claim": False,
    "software_makes_authentication_claim": False,
}

DATA_BOUNDARY = {
    "public_examples_policy": "invented_or_cleared_data_only",
    "protected_source_policy": "no_bundled_protected_owner_or_standards_data",
    "private_data_policy": "references_and_redacted_metadata_only",
    "unit_policy": "unit_bearing_values_require_explicit_unit_metadata",
}

PROVENANCE = {
    "source_name": "OpenPipeStress DEL-07-08 design workspace contract",
    "source_location": "core/gui/design_workspace/engine.py",
    "source_license": "project-governed",
    "contributor": "OpenPipeStress Type 2 worker",
    "contributor_certification": "implementation-only-no-professional-claim",
    "redistribution_status": "public_permissive",
    "review_status": "pending",
    "privacy_classification": "public_metadata",
}


def build_design_authoring_comparison_workspace(
    *,
    workspace_id: str,
    design_knowledge_envelope: Mapping[str, Any] | None = None,
    constraint_validation: Mapping[str, Any] | Any | None = None,
    warning_contract: Mapping[str, Any] | None = None,
    model_states: list[Mapping[str, Any]] | None = None,
    analysis_runs: list[Mapping[str, Any]] | None = None,
    model_state_comparison: Mapping[str, Any] | None = None,
    analysis_run_comparison: Mapping[str, Any] | Any | None = None,
    comparison_mapping_contract: Mapping[str, Any] | None = None,
    tolerance_profile: Mapping[str, Any] | None = None,
    operation_preview: Mapping[str, Any] | None = None,
    operation_audit: Mapping[str, Any] | None = None,
    selected_refs: Mapping[str, Any] | None = None,
) -> dict[str, Any]:
    """Build a deterministic GUI workspace record from upstream contracts."""

    diagnostics: list[dict[str, Any]] = []
    design_panel = _design_knowledge_panel(design_knowledge_envelope, diagnostics)
    warning_panel = _constraint_warning_panel(constraint_validation, warning_contract, diagnostics)
    browser = _state_run_browser(model_states or [], analysis_runs or [], diagnostics)
    comparisons = _comparison_tables(
        model_state_comparison,
        analysis_run_comparison,
        comparison_mapping_contract,
        tolerance_profile,
        diagnostics,
    )
    operation_review = _operation_diff_review(operation_preview, operation_audit, diagnostics)
    overlays = _graphical_overlays(
        model_state_comparison,
        analysis_run_comparison,
        operation_preview,
        diagnostics,
    )

    record = {
        "schema_version": WORKSPACE_VERSION,
        "deliverable_id": "DEL-07-08",
        "package_id": "PKG-07",
        "scope_item": "SOW-076",
        "objectives": ["OBJ-015", "OBJ-016"],
        "workspace_id": str(workspace_id),
        "design_knowledge_panel": design_panel,
        "constraint_warning_panel": warning_panel,
        "state_run_browser": browser,
        "comparison_tables": comparisons,
        "graphical_overlays": overlays,
        "operation_diff_review": operation_review,
        "diagnostics": _stable(diagnostics),
        "mutation_boundary": {
            "workspace_mutates_accepted_model_state": False,
            "hidden_accepted_model_mutation": False,
            "gui_originated_changes_route": "structured_operation_or_application_service_command_intent_only",
            "accepted_operation_requires_explicit_user_acceptance_record": True,
        },
        "analysis_boundary_contract": analysis_boundary_contract(),
        "persistence_hash_contract": persistence_hash_contract(
            gui_hash_scope="workspace_composition_hash_only_not_project_payload_hash"
        ),
        "data_boundary": deepcopy(DATA_BOUNDARY),
        "professional_boundary": deepcopy(PROFESSIONAL_BOUNDARY),
        "provenance": deepcopy(PROVENANCE),
    }
    record["review_state_routing"] = _review_state_routing(
        selected_refs,
        (
            design_panel,
            warning_panel,
            browser,
            comparisons,
            overlays,
            operation_review,
        ),
        record["diagnostics"],
    )
    record["workspace_hash"] = _hash_payload(
        {
            "workspace_id": record["workspace_id"],
            "design_knowledge_panel": design_panel,
            "constraint_warning_panel": warning_panel,
            "state_run_browser": browser,
            "comparison_tables": comparisons,
            "graphical_overlays": overlays,
            "operation_diff_review": operation_review,
            "diagnostics": record["diagnostics"],
            "mutation_boundary": record["mutation_boundary"],
            "analysis_boundary_contract": record["analysis_boundary_contract"],
            "persistence_hash_contract": record["persistence_hash_contract"],
        }
    )
    return record


def canonical_json(value: Any) -> str:
    """Serialize workspace records with stable key ordering."""

    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=True)


def _design_knowledge_panel(
    design_knowledge_envelope: Mapping[str, Any] | None,
    diagnostics: list[dict[str, Any]],
) -> dict[str, Any]:
    source = _plain_record(design_knowledge_envelope)
    if not isinstance(source, Mapping):
        diagnostics.append(
            _diagnostic(
                "WORKSPACE-DESIGN-KNOWLEDGE-MISSING",
                "warning",
                "DesignKnowledgeEnvelope:TBD",
                "Design knowledge panel input is unavailable.",
            )
        )
        return {
            "panel_id": "design_knowledge",
            "availability": "unavailable_missing_input",
            "knowledge_set_ref": _ref("DesignKnowledgeSet", "TBD"),
            "project_ref": None,
            "model_ref": None,
            "records": [],
            "summary": _record_summary([]),
            "diagnostics": [],
            "data_boundary": None,
            "provenance": None,
        }

    knowledge = source.get("design_knowledge", source)
    if not isinstance(knowledge, Mapping):
        diagnostics.append(
            _diagnostic(
                "WORKSPACE-DESIGN-KNOWLEDGE-NOT-OBJECT",
                "blocking",
                "DesignKnowledgeEnvelope:TBD",
                "Design knowledge input is not a structured object.",
            )
        )
        knowledge = {}

    records = [_design_record(item, diagnostics) for item in _list(knowledge.get("records"))]
    if not records:
        diagnostics.append(
            _diagnostic(
                "WORKSPACE-DESIGN-KNOWLEDGE-RECORDS-MISSING",
                "warning",
                _text(knowledge.get("knowledge_set_id"), "DesignKnowledgeSet:TBD"),
                "Design knowledge panel has no record rows to display.",
            )
        )

    source_diagnostics = _diagnostic_records(_list(knowledge.get("diagnostics")))
    return {
        "panel_id": "design_knowledge",
        "availability": "available" if records else "available_with_no_records",
        "knowledge_set_ref": _ref("DesignKnowledgeSet", _text(knowledge.get("knowledge_set_id"), "TBD")),
        "project_ref": deepcopy(knowledge.get("project_ref")),
        "model_ref": deepcopy(knowledge.get("model_ref")),
        "records": sorted(records, key=canonical_json),
        "summary": _record_summary(records),
        "diagnostics": source_diagnostics,
        "data_boundary": deepcopy(source.get("data_boundary")),
        "provenance": deepcopy(knowledge.get("provenance")),
    }


def _design_record(record: Any, diagnostics: list[dict[str, Any]]) -> dict[str, Any]:
    if not isinstance(record, Mapping):
        diagnostics.append(
            _diagnostic(
                "WORKSPACE-DESIGN-RECORD-NOT-OBJECT",
                "blocking",
                "DesignKnowledgeRecord:TBD",
                "Design knowledge records must be structured objects.",
            )
        )
        return {
            "record_ref": _ref("DesignKnowledgeRecord", "TBD"),
            "record_kind": "TBD",
            "name": "TBD",
            "review_status": "TBD",
            "privacy_classification": "TBD",
            "hashes": [],
            "source_notes": [],
            "assumptions": [],
            "unresolved_assumption_ids": [],
            "has_unresolved_tbd": True,
            "provenance": None,
        }

    record_id = _text(record.get("id"), "record:TBD")
    provenance = record.get("provenance") if isinstance(record.get("provenance"), Mapping) else {}
    assumptions = _list(record.get("assumptions"))
    unresolved_assumptions = [
        _text(item.get("assumption_id"), "assumption:TBD")
        for item in assumptions
        if isinstance(item, Mapping) and _text(item.get("status"), "TBD") in {"unresolved", "TBD"}
    ]
    return {
        "record_ref": _ref("DesignKnowledgeRecord", record_id),
        "record_kind": _text(record.get("record_kind"), "TBD"),
        "name": _text(record.get("name"), record_id),
        "review_status": _text(provenance.get("review_status"), "TBD"),
        "privacy_classification": _text(provenance.get("privacy_classification"), "TBD"),
        "hashes": deepcopy(_list(record.get("hashes"))),
        "source_notes": deepcopy(_list(record.get("source_notes"))),
        "assumptions": deepcopy(assumptions),
        "unresolved_assumption_ids": sorted(unresolved_assumptions),
        "has_unresolved_tbd": _contains_tbd(record),
        "provenance": deepcopy(record.get("provenance")),
    }


def _constraint_warning_panel(
    constraint_validation: Mapping[str, Any] | Any | None,
    warning_contract: Mapping[str, Any] | None,
    diagnostics: list[dict[str, Any]],
) -> dict[str, Any]:
    validation = _plain_record(constraint_validation)
    warnings = _plain_record(warning_contract)

    if not isinstance(validation, Mapping):
        diagnostics.append(
            _diagnostic(
                "WORKSPACE-CONSTRAINT-VALIDATION-MISSING",
                "warning",
                "ConstraintValidation:TBD",
                "Constraint validation output is unavailable.",
            )
        )
        validation = {}
    if not isinstance(warnings, Mapping):
        diagnostics.append(
            _diagnostic(
                "WORKSPACE-WARNING-CONTRACT-MISSING",
                "warning",
                "WarningUXContract:TBD",
                "Warning UX contract output is unavailable.",
            )
        )
        warnings = {}

    constraint_diagnostics = _diagnostic_records(_list(validation.get("diagnostics")))
    warning_records = sorted([deepcopy(item) for item in _list(warnings.get("warnings"))], key=canonical_json)
    warning_diagnostics = _diagnostic_records(_list(warnings.get("diagnostics")))
    has_blocking = bool(validation.get("has_blocking_findings")) or _has_blocking(
        constraint_diagnostics + warning_diagnostics + warning_records
    )

    return {
        "panel_id": "constraint_warning",
        "availability": "available" if validation or warnings else "unavailable_missing_input",
        "constraint_validation": {
            "has_blocking_findings": has_blocking,
            "diagnostics": constraint_diagnostics,
        },
        "warnings": warning_records,
        "warning_diagnostics": warning_diagnostics,
        "blocking_summary": deepcopy(
            warnings.get(
                "blocking_summary",
                {"has_blocking_items": has_blocking, "blocking_warning_ids": []},
            )
        ),
        "unresolved_tbd_visible": _contains_tbd(validation) or _contains_tbd(warnings),
        "source_contract_refs": [
            _contract_ref(validation, "ConstraintValidation"),
            _contract_ref(warnings, "WarningUXContract"),
        ],
    }


def _state_run_browser(
    model_states: list[Mapping[str, Any]],
    analysis_runs: list[Mapping[str, Any]],
    diagnostics: list[dict[str, Any]],
) -> dict[str, Any]:
    state_rows = [_model_state_browser_row(item, index, diagnostics) for index, item in enumerate(model_states)]
    run_rows = [_analysis_run_browser_row(item, index, diagnostics) for index, item in enumerate(analysis_runs)]
    if not state_rows:
        diagnostics.append(
            _diagnostic(
                "WORKSPACE-MODEL-STATES-MISSING",
                "warning",
                "ModelState:TBD",
                "State browser has no immutable model-state records.",
            )
        )
    if not run_rows:
        diagnostics.append(
            _diagnostic(
                "WORKSPACE-ANALYSIS-RUNS-MISSING",
                "warning",
                "AnalysisRun:TBD",
                "Run browser has no analysis-run records.",
            )
        )

    return {
        "panel_id": "state_run_browser",
        "availability": "available" if state_rows or run_rows else "unavailable_missing_input",
        "model_states": sorted(state_rows, key=canonical_json),
        "analysis_runs": sorted(run_rows, key=canonical_json),
        "persistence_hash_boundary": _state_run_hash_boundary(state_rows, run_rows, diagnostics),
        "summary": {
            "model_state_count": len(state_rows),
            "analysis_run_count": len(run_rows),
            "states_with_unresolved_assumptions": sum(
                1 for item in state_rows if item["unresolved_assumption_count"] > 0
            ),
            "runs_with_diagnostics": sum(1 for item in run_rows if item["diagnostic_count"] > 0),
        },
    }


def _model_state_browser_row(
    state: Mapping[str, Any],
    index: int,
    diagnostics: list[dict[str, Any]],
) -> dict[str, Any]:
    source = _plain_record(state)
    if not isinstance(source, Mapping):
        diagnostics.append(
            _diagnostic(
                "WORKSPACE-MODEL-STATE-NOT-OBJECT",
                "blocking",
                f"ModelState:index:{index}",
                "Model-state browser input is not a structured object.",
            )
        )
        source = {}
    record = source.get("model_state", source)
    if not isinstance(record, Mapping):
        record = {}
    state_id = _text(record.get("state_id"), f"state:{index:04d}")
    immutability = record.get("immutability_policy") if isinstance(record.get("immutability_policy"), Mapping) else {}
    analysis_status = normalize_analysis_statuses(record.get("analysis_status"), diagnostics, state_id)
    hashes = deepcopy(_list(record.get("hashes")))
    return {
        "state_ref": _ref("ModelState", state_id),
        "state_name": _text(record.get("state_name"), state_id),
        "state_kind": _text(record.get("state_kind"), "TBD"),
        "created_at": _text(record.get("created_at"), "TBD"),
        "model_ref": deepcopy(record.get("model_ref")),
        "parent_state_refs": deepcopy(_list(record.get("parent_state_refs"))),
        "analysis_status": analysis_status,
        "hashes": hashes,
        "hash_boundary": hash_boundary_status(hashes),
        "warnings": deepcopy(_list(record.get("warnings"))),
        "unresolved_assumption_count": len(_list(record.get("unresolved_assumptions"))),
        "unresolved_assumptions": deepcopy(_list(record.get("unresolved_assumptions"))),
        "snapshot_is_read_only": bool(immutability.get("snapshot_is_read_only", True)),
        "mutation_policy": _text(immutability.get("mutation_policy"), "changes_create_new_model_state"),
        "review_status": _review_status(record.get("provenance")),
        "privacy_classification": _privacy_classification(record.get("provenance")),
        "route": "model_state_detail",
    }


def _analysis_run_browser_row(
    run: Mapping[str, Any],
    index: int,
    diagnostics: list[dict[str, Any]],
) -> dict[str, Any]:
    source = _plain_record(run)
    if not isinstance(source, Mapping):
        diagnostics.append(
            _diagnostic(
                "WORKSPACE-ANALYSIS-RUN-NOT-OBJECT",
                "blocking",
                f"AnalysisRun:index:{index}",
                "Analysis-run browser input is not a structured object.",
            )
        )
        source = {}
    record = source.get("analysis_run", source)
    if not isinstance(record, Mapping):
        record = {}
    run_id = _text(record.get("run_id"), f"run:{index:04d}")
    reproducibility = (
        record.get("reproducibility") if isinstance(record.get("reproducibility"), Mapping) else {}
    )
    analysis_status = normalize_analysis_statuses(record.get("analysis_status"), diagnostics, run_id)
    hashes = deepcopy(_list(record.get("hashes")))
    return {
        "run_ref": _ref("AnalysisRun", run_id),
        "run_name": _text(record.get("run_name"), run_id),
        "run_kind": _text(record.get("run_kind"), "TBD"),
        "created_at": _text(record.get("created_at"), "TBD"),
        "model_state_ref": deepcopy(record.get("model_state_ref")),
        "result_refs": deepcopy(_list(record.get("result_refs"))),
        "settings_ref": deepcopy(record.get("settings_ref")),
        "unit_system_ref": deepcopy(record.get("unit_system_ref")),
        "analysis_status": analysis_status,
        "diagnostic_count": len(_list(record.get("diagnostics"))),
        "diagnostics": deepcopy(_list(record.get("diagnostics"))),
        "hashes": hashes,
        "hash_boundary": hash_boundary_status(hashes),
        "unresolved_tbd": deepcopy(_list(reproducibility.get("unresolved_tbd"))),
        "route": "analysis_run_detail",
    }


def _state_run_hash_boundary(
    state_rows: list[Mapping[str, Any]],
    run_rows: list[Mapping[str, Any]],
    diagnostics: list[dict[str, Any]],
) -> dict[str, Any]:
    missing_state_hashes = [row["state_ref"]["ref"] for row in state_rows if not row["hash_boundary"]["hashes_present"]]
    missing_run_hashes = [row["run_ref"]["ref"] for row in run_rows if not row["hash_boundary"]["hashes_present"]]
    for state_ref in missing_state_hashes:
        diagnostics.append(
            _diagnostic(
                "WORKSPACE-MODEL-STATE-HASH-MISSING",
                "warning",
                state_ref,
                "Model-state browser row has no DEL-02-05 hash evidence.",
            )
        )
    for run_ref in missing_run_hashes:
        diagnostics.append(
            _diagnostic(
                "WORKSPACE-ANALYSIS-RUN-HASH-MISSING",
                "warning",
                run_ref,
                "Analysis-run browser row has no DEL-02-05 hash evidence.",
            )
        )
    return {
        "contract_ref": "DEL-02-05",
        "model_state_hashes_present": not missing_state_hashes,
        "analysis_run_hashes_present": not missing_run_hashes,
        "missing_model_state_hash_refs": sorted(missing_state_hashes),
        "missing_analysis_run_hash_refs": sorted(missing_run_hashes),
        "human_acceptance_invalidation": "hash_bound_external_records_do_not_survive_payload_hash_change",
        "workspace_hash_scope": "ui_composition_review_state_only",
        "inherited_record_contracts": ["PKG-14 model state and analysis run records", "PKG-16 operation audit records"],
    }


def _comparison_tables(
    model_state_comparison: Mapping[str, Any] | None,
    analysis_run_comparison: Mapping[str, Any] | Any | None,
    comparison_mapping_contract: Mapping[str, Any] | None,
    tolerance_profile: Mapping[str, Any] | None,
    diagnostics: list[dict[str, Any]],
) -> dict[str, Any]:
    model_comparison = _plain_record(model_state_comparison)
    analysis_comparison = _plain_record(analysis_run_comparison)

    model_table = _model_comparison_table(model_comparison, diagnostics)
    analysis_table = _analysis_comparison_table(analysis_comparison, diagnostics)
    return {
        "panel_id": "comparison_tables",
        "availability": (
            "available"
            if model_table["availability"] == "available" or analysis_table["availability"] == "available"
            else "unavailable_missing_input"
        ),
        "model_state_rows": model_table,
        "analysis_run_rows": analysis_table,
        "mapping_contract": _mapping_contract_summary(comparison_mapping_contract),
        "tolerance_profile": _tolerance_profile_summary(tolerance_profile),
    }


def _model_comparison_table(
    model_comparison: Any,
    diagnostics: list[dict[str, Any]],
) -> dict[str, Any]:
    if not isinstance(model_comparison, Mapping):
        diagnostics.append(
            _diagnostic(
                "WORKSPACE-MODEL-COMPARISON-MISSING",
                "warning",
                "ModelStateComparison:TBD",
                "Model-state comparison output is unavailable.",
            )
        )
        return {
            "availability": "unavailable_missing_input",
            "left_state_ref": None,
            "right_state_ref": None,
            "summary": {},
            "rows": [],
            "diagnostics": [],
            "unmatched_rows": [],
        }

    rows = [_model_comparison_row(item, index) for index, item in enumerate(_list(model_comparison.get("entities")))]
    return {
        "availability": "available",
        "left_state_ref": deepcopy(model_comparison.get("left_state_ref")),
        "right_state_ref": deepcopy(model_comparison.get("right_state_ref")),
        "summary": deepcopy(model_comparison.get("summary", {})),
        "rows": sorted(rows, key=canonical_json),
        "diagnostics": _diagnostic_records(_list(model_comparison.get("diagnostics"))),
        "unmatched_rows": sorted(
            [
                row
                for row in rows
                if row["classification"] in {"added", "removed", "unresolved"}
                or row["match_basis"] == "unresolved_mapping"
            ],
            key=canonical_json,
        ),
    }


def _model_comparison_row(row: Any, index: int) -> dict[str, Any]:
    if not isinstance(row, Mapping):
        return {
            "review_row_id": f"model-row:{index:04d}",
            "classification": "unresolved",
            "match_basis": "TBD",
            "mapping_id": None,
            "left_ref": None,
            "right_ref": None,
            "change_count": 0,
            "changes": [],
            "unmatched_classification": "unresolved_TBD",
        }
    classification = _text(row.get("classification"), "unresolved")
    return {
        "review_row_id": _text(row.get("row_id"), f"model-row:{index:04d}"),
        "classification": classification,
        "match_basis": _text(row.get("match_basis"), "TBD"),
        "mapping_id": deepcopy(row.get("mapping_id")),
        "left_ref": deepcopy(row.get("left_ref")),
        "right_ref": deepcopy(row.get("right_ref")),
        "change_count": len(_list(row.get("changes"))),
        "changes": deepcopy(_list(row.get("changes"))),
        "unmatched_classification": _unmatched_classification(classification),
    }


def _analysis_comparison_table(
    analysis_comparison: Any,
    diagnostics: list[dict[str, Any]],
) -> dict[str, Any]:
    if not isinstance(analysis_comparison, Mapping):
        diagnostics.append(
            _diagnostic(
                "WORKSPACE-ANALYSIS-COMPARISON-MISSING",
                "warning",
                "AnalysisRunComparison:TBD",
                "Analysis-run comparison output is unavailable.",
            )
        )
        return {
            "availability": "unavailable_missing_input",
            "comparison_id": "TBD",
            "run_context": {},
            "result_rows": [],
            "settings_rows": [],
            "diagnostics": [],
            "has_blocking_findings": False,
        }

    result_rows = [
        _analysis_result_row(item, index)
        for index, item in enumerate(_list(analysis_comparison.get("result_deltas")))
    ]
    settings_rows = [
        deepcopy(item) for item in _list(analysis_comparison.get("settings_deltas")) if isinstance(item, Mapping)
    ]
    return {
        "availability": "available",
        "comparison_id": _text(analysis_comparison.get("comparison_id"), "TBD"),
        "run_context": deepcopy(analysis_comparison.get("run_context", {})),
        "result_rows": sorted(result_rows, key=canonical_json),
        "settings_rows": sorted(settings_rows, key=canonical_json),
        "diagnostics": _diagnostic_records(_list(analysis_comparison.get("diagnostics"))),
        "has_blocking_findings": bool(analysis_comparison.get("has_blocking_findings")),
    }


def _analysis_result_row(row: Any, index: int) -> dict[str, Any]:
    if not isinstance(row, Mapping):
        return {
            "review_row_id": f"analysis-row:{index:04d}",
            "mapping_id": "TBD",
            "result_family": "TBD",
            "classification": "unresolved",
            "tolerance_profile_ref": None,
            "tolerance_rule_id": None,
            "unit": "TBD",
            "dimension": "TBD",
            "left_magnitude": "TBD",
            "right_magnitude": "TBD",
            "normalized_delta": "TBD",
            "object_refs": {},
        }
    mapping_id = _text(row.get("mapping_id"), f"mapping:{index:04d}")
    return {
        "review_row_id": f"analysis:{mapping_id}",
        "mapping_id": mapping_id,
        "result_family": _text(row.get("result_family"), "TBD"),
        "classification": _text(row.get("classification"), "unresolved"),
        "classification_basis": _text(row.get("classification_basis"), "TBD"),
        "tolerance_profile_ref": deepcopy(row.get("tolerance_profile_ref")),
        "tolerance_rule_id": deepcopy(row.get("tolerance_rule_id")),
        "unit": _text(row.get("normalized_unit"), "TBD"),
        "dimension": _text(row.get("dimension"), "TBD"),
        "left_magnitude": deepcopy(row.get("left_magnitude", "TBD")),
        "right_magnitude": deepcopy(row.get("right_magnitude", "TBD")),
        "left_normalized_magnitude": deepcopy(row.get("left_normalized_magnitude", "TBD")),
        "right_normalized_magnitude": deepcopy(row.get("right_normalized_magnitude", "TBD")),
        "normalized_delta": deepcopy(row.get("normalized_delta", "TBD")),
        "absolute_normalized_delta": deepcopy(row.get("absolute_normalized_delta", "TBD")),
        "object_refs": deepcopy(row.get("object_refs", {})),
    }


def _graphical_overlays(
    model_state_comparison: Mapping[str, Any] | None,
    analysis_run_comparison: Mapping[str, Any] | Any | None,
    operation_preview: Mapping[str, Any] | None,
    diagnostics: list[dict[str, Any]],
) -> dict[str, Any]:
    overlays: list[dict[str, Any]] = []
    model_comparison = _plain_record(model_state_comparison)
    if isinstance(model_comparison, Mapping):
        for index, row in enumerate(_list(model_comparison.get("entities"))):
            if isinstance(row, Mapping):
                overlays.append(_model_overlay(row, index))

    analysis_comparison = _plain_record(analysis_run_comparison)
    if isinstance(analysis_comparison, Mapping):
        for index, row in enumerate(_list(analysis_comparison.get("result_deltas"))):
            if isinstance(row, Mapping):
                overlays.append(_analysis_overlay(row, index))

    preview = _plain_record(operation_preview)
    if isinstance(preview, Mapping):
        for index, row in enumerate(_list(preview.get("diff_preview"))):
            if isinstance(row, Mapping):
                overlays.append(_operation_overlay(row, index))

    if not overlays:
        diagnostics.append(
            _diagnostic(
                "WORKSPACE-OVERLAY-SOURCES-MISSING",
                "warning",
                "GraphicalOverlay:TBD",
                "No comparison or operation preview rows are available for overlay descriptors.",
            )
        )

    return {
        "panel_id": "graphical_overlays",
        "availability": "available" if overlays else "unavailable_missing_input",
        "overlay_policy": "descriptor_only_no_solver_or_geometry_generation",
        "overlays": sorted(overlays, key=canonical_json),
    }


def _model_overlay(row: Mapping[str, Any], index: int) -> dict[str, Any]:
    classification = _text(row.get("classification"), "unresolved")
    row_id = _text(row.get("row_id"), f"model-row:{index:04d}")
    return {
        "overlay_id": f"overlay:model_state:{row_id}",
        "overlay_kind": "model_state_delta",
        "source_row_id": row_id,
        "left_ref": deepcopy(row.get("left_ref")),
        "right_ref": deepcopy(row.get("right_ref")),
        "classification": classification,
        "style_token": _style_token(classification),
        "visible_by_default": classification not in {"unchanged", "mapped_unchanged"},
        "geometry_binding": "uses_upstream_entity_refs_descriptor_only",
        "review_boundary": "visual_review_support_only",
    }


def _analysis_overlay(row: Mapping[str, Any], index: int) -> dict[str, Any]:
    mapping_id = _text(row.get("mapping_id"), f"mapping:{index:04d}")
    classification = _text(row.get("classification"), "unresolved")
    return {
        "overlay_id": f"overlay:analysis_run:{mapping_id}",
        "overlay_kind": "analysis_result_delta",
        "source_row_id": f"analysis:{mapping_id}",
        "object_refs": deepcopy(row.get("object_refs", {})),
        "result_family": _text(row.get("result_family"), "TBD"),
        "classification": classification,
        "normalized_delta": deepcopy(row.get("normalized_delta", "TBD")),
        "unit": _text(row.get("normalized_unit"), "TBD"),
        "tolerance_profile_ref": deepcopy(row.get("tolerance_profile_ref")),
        "tolerance_rule_id": deepcopy(row.get("tolerance_rule_id")),
        "style_token": _style_token(classification),
        "visible_by_default": True,
        "geometry_binding": "uses_upstream_result_object_refs_descriptor_only",
        "review_boundary": "visual_review_support_only",
    }


def _operation_overlay(row: Mapping[str, Any], index: int) -> dict[str, Any]:
    operation_id = _text(row.get("operation_id"), f"operation:{index:04d}")
    change_id = _text(row.get("change_id"), f"change:{index:04d}")
    preview_status = _text(row.get("preview_status"), "TBD")
    return {
        "overlay_id": f"overlay:operation:{operation_id}:{change_id}",
        "overlay_kind": "operation_diff_preview",
        "source_row_id": f"operation:{operation_id}:{change_id}",
        "target_ref": deepcopy(row.get("target_ref")),
        "preview_status": preview_status,
        "style_token": _style_token(preview_status),
        "visible_by_default": preview_status == "previewed",
        "geometry_binding": "uses_diff_preview_before_after_refs_descriptor_only",
        "review_boundary": "operation_review_support_only",
    }


def _operation_diff_review(
    operation_preview: Mapping[str, Any] | None,
    operation_audit: Mapping[str, Any] | None,
    diagnostics: list[dict[str, Any]],
) -> dict[str, Any]:
    preview = _plain_record(operation_preview)
    audit = _plain_record(operation_audit)
    audit_records = _list(audit.get("records")) if isinstance(audit, Mapping) else []
    audit_by_operation = {
        _text(item.get("operation_id"), "operation:TBD"): item
        for item in audit_records
        if isinstance(item, Mapping)
    }

    if not isinstance(preview, Mapping):
        diagnostics.append(
            _diagnostic(
                "WORKSPACE-OPERATION-PREVIEW-MISSING",
                "warning",
                "OperationPreview:TBD",
                "Operation diff-preview output is unavailable.",
            )
        )
        preview = {}
    if not isinstance(audit, Mapping):
        diagnostics.append(
            _diagnostic(
                "WORKSPACE-OPERATION-AUDIT-MISSING",
                "warning",
                "OperationAuditTrail:TBD",
                "Operation audit trail output is unavailable.",
            )
        )
        audit = {}

    preview_rows = _list(preview.get("diff_preview"))
    rows = [
        _operation_review_row(item, index, preview, audit_by_operation, diagnostics)
        for index, item in enumerate(preview_rows)
    ]
    if not rows:
        diagnostics.append(
            _diagnostic(
                "WORKSPACE-OPERATION-ROWS-MISSING",
                "warning",
                "OperationPreview:TBD",
                "Operation review has no diff-preview rows.",
            )
        )

    return {
        "panel_id": "operation_diff_review",
        "availability": "available" if rows else "unavailable_missing_input",
        "operation_set_ref": deepcopy(preview.get("operation_set_ref")),
        "accepted_model_state_ref": deepcopy(preview.get("accepted_model_state_ref")),
        "validation": deepcopy(preview.get("validation", {})),
        "preview_diagnostics": _diagnostic_records(_list(preview.get("diagnostics"))),
        "audit_decision_counts": deepcopy(
            audit.get(
                "decision_counts",
                {"accepted": 0, "rejected": 0, "held_for_user_acceptance": 0},
            )
        ),
        "rows": sorted(rows, key=canonical_json),
        "operation_policy": {
            "workspace_applies_operations": False,
            "workspace_mutates_accepted_state": False,
            "explicit_user_acceptance_required_before_accepted_operation_label": True,
        },
    }


def _operation_review_row(
    row: Any,
    index: int,
    preview: Mapping[str, Any],
    audit_by_operation: Mapping[str, Mapping[str, Any]],
    diagnostics: list[dict[str, Any]],
) -> dict[str, Any]:
    if not isinstance(row, Mapping):
        diagnostics.append(
            _diagnostic(
                "WORKSPACE-OPERATION-PREVIEW-ROW-NOT-OBJECT",
                "blocking",
                f"OperationPreview:index:{index}",
                "Operation preview row is not a structured object.",
            )
        )
        row = {}

    operation_id = _text(row.get("operation_id"), f"operation:{index:04d}")
    change_id = _text(row.get("change_id"), f"change:{index:04d}")
    audit_record = audit_by_operation.get(operation_id)
    decision = audit_record.get("decision") if isinstance(audit_record, Mapping) else {}
    if not isinstance(decision, Mapping):
        decision = {}

    source_application_status = _text(
        row.get("application_status"),
        _text(_mapping_get(preview.get("validation"), "application_status"), "TBD"),
    )
    if source_application_status != "not_applied":
        diagnostics.append(
            _diagnostic(
                "WORKSPACE-OPERATION-MUTATION-SIGNAL-BLOCKED",
                "blocking",
                operation_id,
                "Operation review input indicates application outside the workspace boundary.",
            )
        )

    explicit_acceptance = (
        decision.get("status") == "accepted"
        and bool(decision.get("explicit_user_acceptance"))
        and not bool(decision.get("accepted_model_state_mutated"))
    )
    return {
        "review_row_id": f"operation:{operation_id}:{change_id}",
        "operation_id": operation_id,
        "change_id": change_id,
        "change_kind": _text(row.get("change_kind"), "TBD"),
        "target_ref": deepcopy(row.get("target_ref")),
        "preview_status": _text(row.get("preview_status"), "TBD"),
        "source_application_status": source_application_status,
        "workspace_application_status": "not_applied",
        "accepted_model_state_mutated_by_workspace": False,
        "can_be_represented_as_accepted_operation": explicit_acceptance,
        "review_state": _operation_review_state(row, decision, explicit_acceptance),
        "audit_decision": {
            "status": _text(decision.get("status"), "no_audit_record"),
            "explicit_user_acceptance": bool(decision.get("explicit_user_acceptance", False)),
            "accepted_model_state_mutated": bool(decision.get("accepted_model_state_mutated", False)),
            "decided_at": deepcopy(decision.get("decided_at")),
        },
        "before_ref": _preview_object_ref(row.get("before"), "before"),
        "after_ref": _preview_object_ref(row.get("after"), "after"),
        "blocking_diagnostic_codes": deepcopy(_list(row.get("blocking_diagnostic_codes"))),
    }


def _operation_review_state(
    row: Mapping[str, Any],
    decision: Mapping[str, Any],
    explicit_acceptance: bool,
) -> str:
    if explicit_acceptance:
        return "explicit_user_acceptance_recorded"
    if decision.get("status") == "rejected":
        return "rejected_operation_recorded"
    if decision.get("status") == "held_for_user_acceptance":
        return "held_for_user_acceptance"
    if row.get("preview_status") == "blocked_by_validation":
        return "blocked_by_validation"
    if row.get("preview_status") == "previewed":
        return "preview_ready_for_user_review"
    return "preview_unavailable"


def _review_state_routing(
    selected_refs: Mapping[str, Any] | None,
    panels: tuple[Mapping[str, Any], ...],
    diagnostics: list[Mapping[str, Any]],
) -> dict[str, Any]:
    unavailable = [
        _text(panel.get("panel_id"), "panel:TBD")
        for panel in panels
        if panel.get("availability") == "unavailable_missing_input"
    ]
    blocking_codes = sorted(
        _text(item.get("code"), "TBD")
        for item in diagnostics
        if _text(item.get("severity"), "") == "blocking"
    )
    if blocking_codes:
        route_state = "blocked_findings_visible"
    elif unavailable:
        route_state = "input_unavailable_review"
    else:
        route_state = "review_navigation_available"
    return {
        "route_state": route_state,
        "selected_refs": deepcopy(dict(selected_refs or {})),
        "available_panel_ids": sorted(
            _text(panel.get("panel_id"), "panel:TBD")
            for panel in panels
            if panel.get("availability") != "unavailable_missing_input"
        ),
        "unavailable_panel_ids": sorted(unavailable),
        "blocking_diagnostic_codes": blocking_codes,
    }


def _mapping_contract_summary(contract: Mapping[str, Any] | None) -> dict[str, Any]:
    source = _plain_record(contract)
    if not isinstance(source, Mapping):
        return {"availability": "unavailable_missing_input", "mapping_count": 0, "unmatched_count": 0}
    review = source.get("comparison_review", source)
    if not isinstance(review, Mapping):
        review = {}
    return {
        "availability": "available",
        "comparison_id": _text(review.get("comparison_id"), "TBD"),
        "mapping_count": len(_list(review.get("mappings"))),
        "unmatched_count": len(_list(review.get("unmatched_records"))),
        "diagnostics": _diagnostic_records(_list(review.get("diagnostics"))),
        "review": deepcopy(review.get("review")),
        "provenance": deepcopy(review.get("provenance")),
    }


def _tolerance_profile_summary(profile: Mapping[str, Any] | None) -> dict[str, Any]:
    source = _plain_record(profile)
    if not isinstance(source, Mapping):
        return {"availability": "unavailable_missing_input", "profile_ref": _ref("ToleranceProfile", "TBD"), "rule_count": 0}
    tolerance = source.get("tolerance_profile", source)
    if not isinstance(tolerance, Mapping):
        tolerance = {}
    return {
        "availability": "available",
        "profile_ref": _ref("ToleranceProfile", _text(tolerance.get("profile_id"), "TBD")),
        "profile_status": _text(tolerance.get("profile_status"), "TBD"),
        "rule_count": len(_list(tolerance.get("rules"))),
        "diagnostics": _diagnostic_records(_list(tolerance.get("diagnostics"))),
        "assumptions": deepcopy(_list(tolerance.get("assumptions"))),
        "hashes": deepcopy(_list(tolerance.get("hashes"))),
        "review": deepcopy(tolerance.get("review")),
        "provenance": deepcopy(tolerance.get("provenance")),
    }


def _record_summary(records: list[Mapping[str, Any]]) -> dict[str, Any]:
    counts: dict[str, int] = {}
    privacy: set[str] = set()
    unresolved = 0
    for record in records:
        kind = _text(record.get("record_kind"), "TBD")
        counts[kind] = counts.get(kind, 0) + 1
        privacy.add(_text(record.get("privacy_classification"), "TBD"))
        if record.get("has_unresolved_tbd") or record.get("unresolved_assumption_ids"):
            unresolved += 1
    return {
        "record_count": len(records),
        "record_kinds": dict(sorted(counts.items())),
        "records_with_unresolved_tbd": unresolved,
        "privacy_classifications": sorted(privacy),
    }


def _diagnostic_records(items: list[Any]) -> list[dict[str, Any]]:
    return sorted([deepcopy(item) for item in items if isinstance(item, Mapping)], key=canonical_json)


def _diagnostic(code: str, severity: str, target_ref: str, message: str) -> dict[str, Any]:
    contract_ref = "DEL-02-05" if "HASH" in code else "DEL-02-03"
    return pkg02_diagnostic(
        code,
        severity,
        target_ref,
        message=message,
        contract_ref=contract_ref,
    )


def _contract_ref(record: Mapping[str, Any], fallback_type: str) -> dict[str, Any]:
    deliverable_id = record.get("deliverable_id")
    if deliverable_id:
        return {"object_type": fallback_type, "ref": str(deliverable_id)}
    return {"object_type": fallback_type, "ref": "TBD"}


def _plain_record(value: Any) -> Any:
    if value is None:
        return None
    to_dict = getattr(value, "to_dict", None)
    if callable(to_dict):
        return deepcopy(to_dict())
    if isinstance(value, Mapping):
        return deepcopy(dict(value))
    return deepcopy(value)


def _preview_object_ref(value: Any, side: str) -> dict[str, Any] | None:
    if value is None:
        return None
    if isinstance(value, Mapping):
        stable_id = _first_text(
            value.get("stable_id"),
            value.get("entity_id"),
            value.get("id"),
            _mapping_get(value.get("reference"), "ref"),
            "TBD",
        )
        return {"object_type": f"PreviewObject:{side}", "ref": stable_id}
    return {"object_type": f"PreviewObject:{side}", "ref": "TBD"}


def _unmatched_classification(classification: str) -> str | None:
    if classification == "added":
        return "right_only"
    if classification == "removed":
        return "left_only"
    if classification == "unresolved":
        return "unresolved_TBD"
    return None


def _style_token(classification: str) -> str:
    tokens = {
        "added": "comparison_added",
        "removed": "comparison_removed",
        "changed": "comparison_changed",
        "mapped_changed": "comparison_changed",
        "unchanged": "comparison_unchanged",
        "mapped_unchanged": "comparison_unchanged",
        "unresolved": "comparison_unresolved",
        "previewed": "operation_preview",
        "blocked_by_validation": "operation_blocked",
        "exceeds_tolerance_profile": "result_delta_exceeds_tolerance",
        "within_tolerance_profile": "result_delta_within_tolerance",
    }
    return tokens.get(classification, "review_required")


def _review_status(provenance: Any) -> str:
    if isinstance(provenance, Mapping):
        return _text(provenance.get("review_status"), "TBD")
    return "TBD"


def _privacy_classification(provenance: Any) -> str:
    if isinstance(provenance, Mapping):
        return _text(provenance.get("privacy_classification"), "TBD")
    return "TBD"


def _has_blocking(items: list[Any]) -> bool:
    for item in items:
        if isinstance(item, Mapping):
            if _text(item.get("severity"), "") == "blocking":
                return True
            if bool(item.get("blocks_progress")):
                return True
    return False


def _contains_tbd(value: Any) -> bool:
    if isinstance(value, str):
        return "TBD" in value
    if isinstance(value, Mapping):
        return any(_contains_tbd(item) for item in value.values())
    if isinstance(value, list):
        return any(_contains_tbd(item) for item in value)
    return value is None


def _hash_payload(value: Any) -> str:
    return "sha256:" + hashlib.sha256(canonical_json(value).encode("utf-8")).hexdigest()


def _stable(items: list[Mapping[str, Any]]) -> list[dict[str, Any]]:
    return sorted([deepcopy(dict(item)) for item in items], key=canonical_json)


def _ref(object_type: str, value: Any) -> dict[str, str]:
    return {"object_type": object_type, "ref": _text(value, "TBD")}


def _mapping_get(value: Any, key: str) -> Any:
    return value.get(key) if isinstance(value, Mapping) else None


def _list(value: Any) -> list[Any]:
    return value if isinstance(value, list) else []


def _first_text(*values: Any) -> str:
    for value in values:
        text = str(value).strip() if value is not None else ""
        if text:
            return text
    return "TBD"


def _text(value: Any, fallback: str) -> str:
    text = str(value).strip() if value is not None else ""
    return text or fallback
