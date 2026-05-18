"""Deterministic accessibility/usability baseline records for DEL-07-06."""

from __future__ import annotations

from collections import Counter
from collections.abc import Iterable, Mapping
from copy import deepcopy
import hashlib
import json
from typing import Any


PROFESSIONAL_BOUNDARY = {
    "human_review_required": True,
    "software_makes_compliance_claim": False,
    "software_makes_certification_claim": False,
    "software_makes_sealing_claim": False,
    "software_makes_approval_claim": False,
    "software_makes_authentication_claim": False,
}

PROVENANCE = {
    "source_name": "OpenPipeStress DEL-07-06 accessibility/usability baseline",
    "source_location": "core/gui/accessibility/engine.py",
    "source_license": "project-governed",
    "redistribution_status": "public_permissive",
    "privacy_classification": "public_metadata",
}

SURFACE_NAMES = {
    "DEL-07-01": "viewport_editor",
    "DEL-07-02": "model_tree_property_inspector",
    "DEL-07-03": "material_component_rule_pack_editors",
    "DEL-07-04": "missing_data_warning_ux",
    "DEL-07-05": "results_viewer",
    "DEL-07-07": "solve_execution_ux",
}

REQUIRED_SURFACES = tuple(SURFACE_NAMES)
OUTCOMES = {"pass", "warning", "fail", "not_applicable"}
SEVERITIES = {"info", "minor", "major", "blocking"}
PUBLIC_PRIVACY_CLASSES = {"invented_public_example", "public_metadata"}
REFERENCE_ONLY_PRIVACY_CLASSES = {"private_reference_only"}
PROTECTED_CONTENT_MARKERS = (
    "allowable " + "stress table",
    "stress " + "intensification factor table",
    "vendor " + "catalog value",
    "real " + "secret",
    "code " + "compliant",
    "accessibility " + "certified",
    "professionally " + "approved",
)


def build_accessibility_usability_baseline(
    *,
    baseline_id: str,
    gui_contract_records: Iterable[Mapping[str, Any]] | None = None,
) -> dict[str, Any]:
    """Build a deterministic contract-level accessibility/usability review.

    This reviewer consumes already-built GUI contract records. It does not run
    a desktop GUI, mutate source records, infer missing engineering data, or
    assert a final accessibility target.
    """

    findings: list[dict[str, Any]] = []
    invalid_inputs: list[Any] = []
    records = []
    for index, record in enumerate(gui_contract_records or []):
        if isinstance(record, Mapping):
            records.append(deepcopy(record))
        else:
            invalid_inputs.append({"input_index": index, "input_type": type(record).__name__})

    for invalid in invalid_inputs:
        _append_finding(
            findings,
            category="review_input",
            outcome="fail",
            severity="blocking",
            source_surface="DEL-07-06:accessibility_usability_baseline",
            affected_control=f"gui_contract_records[{invalid['input_index']}]",
            remediation_note="Provide a mapping-style GUI contract record for deterministic review.",
            evidence_refs=[invalid],
        )

    records = sorted(records, key=_record_sort_key)
    for record in records:
        _review_surface(record, findings)

    _review_required_surface_coverage(records, findings)
    _review_workflow_continuity(records, findings)
    _review_runtime_boundary(findings)

    findings = _stable_findings(findings)
    return {
        "schema_version": "0.1.0",
        "deliverable_id": "DEL-07-06",
        "package_id": "PKG-07",
        "scope_item": "SOW-036",
        "objectives": ["OBJ-006"],
        "baseline_id": str(baseline_id),
        "review_mode": "deterministic_contract_records_no_live_desktop_runtime",
        "accessibility_target_status": "TBD_by_human_project_authority",
        "desktop_runtime_evaluation": "not_performed",
        "source_surfaces": [_surface_summary(record) for record in records],
        "findings": findings,
        "summary": _summary(findings),
        "review_policy": {
            "auto_fill_missing_data": False,
            "mutates_gui_contract_records": False,
            "preserves_diagnostics": True,
            "preserves_assumptions": True,
            "preserves_provenance": True,
            "preserves_privacy_classification": True,
            "software_makes_accessibility_conformance_claim": False,
        },
        "professional_boundary": deepcopy(PROFESSIONAL_BOUNDARY),
        "provenance": deepcopy(PROVENANCE),
    }


def canonical_json(value: Any) -> str:
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=True)


def _review_surface(record: Mapping[str, Any], findings: list[dict[str, Any]]) -> None:
    deliverable_id = _text(record.get("deliverable_id"), "DEL:UNKNOWN")
    if deliverable_id == "DEL-07-01":
        _review_viewport(record, findings)
    elif deliverable_id == "DEL-07-02":
        _review_model_tree(record, findings)
    elif deliverable_id == "DEL-07-03":
        _review_editors(record, findings)
    elif deliverable_id == "DEL-07-04":
        _review_warnings(record, findings)
    elif deliverable_id == "DEL-07-05":
        _review_results(record, findings)
    elif deliverable_id == "DEL-07-07":
        _review_solve_execution(record, findings)
    else:
        _append_finding(
            findings,
            category="review_input",
            outcome="warning",
            severity="major",
            source_surface=_source_surface(record),
            affected_control="deliverable_id",
            remediation_note="Use a known PKG-07 GUI contract surface or add an explicit reviewer.",
            evidence_refs=[{"deliverable_id": deliverable_id}],
        )

    _review_source_diagnostics(record.get("diagnostics"), record, "diagnostics", findings)
    _review_professional_boundary(record, findings)
    _review_provenance_and_privacy(record, findings)
    _review_content_boundary(record, findings)


def _review_viewport(record: Mapping[str, Any], findings: list[dict[str, Any]]) -> None:
    source = _source_surface(record)
    session = _mapping(record.get("viewport_session"))
    status = _mapping(record.get("viewport_status"))
    interaction = _mapping(session.get("interaction_state"))
    active_tool = _text(interaction.get("active_tool"), "")
    command_intents = _list(session.get("command_intents"))
    primitives = _list(session.get("view_primitives"))

    _append_finding(
        findings,
        category="keyboard_path",
        outcome="pass" if active_tool and active_tool != "TBD" else "warning",
        severity="minor",
        source_surface=source,
        affected_control="viewport_session.interaction_state.active_tool",
        remediation_note="Expose the active tool state through the keyboard-reachable viewport controls.",
        evidence_refs=[{"active_tool": active_tool or "TBD"}],
    )

    _append_finding(
        findings,
        category="focus_order",
        outcome="pass" if command_intents else "warning",
        severity="minor" if command_intents else "major",
        source_surface=source,
        affected_control="viewport_session.command_intents",
        remediation_note="Keep command intents reachable from the viewport review path before durable model mutation.",
        evidence_refs=[{"command_intent_count": len(command_intents)}],
    )

    undo_policy = _text(interaction.get("undo_redo_policy"), "")
    reversible = [item for item in command_intents if _mapping(item).get("reversible") is True]
    _append_finding(
        findings,
        category="review_workflow_continuity",
        outcome="pass" if reversible and "reversible" in undo_policy else "warning",
        severity="minor",
        source_surface=source,
        affected_control="viewport_session.interaction_state.undo_redo_policy",
        remediation_note="Keep undo/redo affordances tied to reversible command intents and refreshed diagnostics.",
        evidence_refs=[
            {"reversible_command_count": len(reversible)},
            {"undo_redo_policy": undo_policy or "TBD"},
        ],
    )

    labeled = [
        item
        for item in primitives
        if _has_visible_text(_mapping(_mapping(item).get("style")).get("label"))
    ]
    _append_finding(
        findings,
        category="readable_label",
        outcome="pass" if primitives and len(labeled) == len(primitives) else "warning",
        severity="minor",
        source_surface=source,
        affected_control="viewport_session.view_primitives.style.label",
        remediation_note="Maintain text labels for viewport primitives so status color is not the only review cue.",
        evidence_refs=[
            {"primitive_count": len(primitives)},
            {"labeled_primitive_count": len(labeled)},
        ],
    )

    color_only = [
        _mapping(item).get("primitive_id")
        for item in primitives
        if _mapping(_mapping(item).get("style")).get("status_color") not in (None, "neutral")
        and not _list(_mapping(item).get("diagnostic_refs"))
    ]
    _append_finding(
        findings,
        category="warning_visibility",
        outcome="pass" if not color_only else "warning",
        severity="major" if color_only else "minor",
        source_surface=source,
        affected_control="viewport_session.view_primitives.diagnostic_refs",
        remediation_note="Pair warning status visuals with diagnostic references or text status.",
        evidence_refs=[{"color_status_without_diagnostic_ref": sorted(map(str, color_only))}],
    )

    _review_source_diagnostics(session.get("diagnostics"), record, "viewport_session.diagnostics", findings)
    _review_professional_boundary(status, findings, source_surface=source, affected_control="viewport_status.professional_boundary")


def _review_model_tree(record: Mapping[str, Any], findings: list[dict[str, Any]]) -> None:
    source = _source_surface(record)
    nodes = _list(record.get("tree_nodes"))
    selection = _mapping(record.get("selection"))
    inspector = _mapping(record.get("property_inspector"))
    fields = _list(inspector.get("fields"))

    _append_finding(
        findings,
        category="keyboard_path",
        outcome="pass" if nodes else "warning",
        severity="minor" if nodes else "major",
        source_surface=source,
        affected_control="tree_nodes",
        remediation_note="Provide keyboard traversal over model tree nodes for engineering review.",
        evidence_refs=[{"tree_node_count": len(nodes)}],
    )

    _append_finding(
        findings,
        category="focus_order",
        outcome="pass" if selection.get("selected") and inspector.get("status") == "selection_resolved" else "warning",
        severity="minor" if selection.get("selected") else "major",
        source_surface=source,
        affected_control="selection_to_property_inspector",
        remediation_note="Keep focus continuity from the selected model-tree item into its property inspector fields.",
        evidence_refs=[
            {"selected": bool(selection.get("selected"))},
            {"inspector_status": _text(inspector.get("status"), "TBD")},
        ],
    )

    missing_labels = [item.get("field_id", "field:TBD") for item in fields if not _has_visible_text(item.get("label"))]
    _append_finding(
        findings,
        category="readable_label",
        outcome="pass" if fields and not missing_labels else "warning",
        severity="minor",
        source_surface=source,
        affected_control="property_inspector.fields.label",
        remediation_note="Keep inspector field labels readable and independent of raw field identifiers.",
        evidence_refs=[{"missing_label_field_ids": sorted(map(str, missing_labels))}],
    )

    missing_units = [
        item.get("field_id", "field:TBD")
        for item in fields
        if isinstance(item.get("value"), (int, float)) and item.get("unit") in (None, "TBD")
    ]
    _append_finding(
        findings,
        category="result_review_visibility",
        outcome="pass" if not missing_units else "warning",
        severity="major" if missing_units else "minor",
        source_surface=source,
        affected_control="property_inspector.fields.unit",
        remediation_note="Keep units visible for unit-bearing values in review surfaces.",
        evidence_refs=[{"numeric_fields_missing_units": sorted(map(str, missing_units))}],
    )


def _review_editors(record: Mapping[str, Any], findings: list[dict[str, Any]]) -> None:
    source = _source_surface(record)
    editors = _list(record.get("editors"))
    _append_finding(
        findings,
        category="keyboard_path",
        outcome="pass" if editors else "warning",
        severity="minor" if editors else "major",
        source_surface=source,
        affected_control="editors",
        remediation_note="Provide keyboard traversal through material, component, and rule-pack editor surfaces.",
        evidence_refs=[{"editor_count": len(editors)}],
    )

    for editor in editors:
        editor_record = _mapping(editor)
        editor_id = _text(editor_record.get("editor_id"), "editor:TBD")
        fields = _list(editor_record.get("fields"))
        missing_labels = [
            field.get("field_id", "field:TBD")
            for field in fields
            if not _has_visible_text(_mapping(field).get("label"))
        ]
        _append_finding(
            findings,
            category="readable_label",
            outcome="pass" if fields and not missing_labels else "warning",
            severity="minor",
            source_surface=source,
            affected_control=f"editors[{editor_id}].fields.label",
            remediation_note="Keep editor field labels explicit for keyboard and review workflows.",
            evidence_refs=[{"missing_label_field_ids": sorted(map(str, missing_labels))}],
        )

        save_intent = _mapping(editor_record.get("save_intent"))
        _append_finding(
            findings,
            category="review_workflow_continuity",
            outcome="pass" if save_intent.get("mutates_persistent_project") is False else "fail",
            severity="blocking" if save_intent.get("mutates_persistent_project") is not False else "minor",
            source_surface=source,
            affected_control=f"editors[{editor_id}].save_intent",
            remediation_note="Route save actions through service validation instead of direct persistent mutation.",
            evidence_refs=[{"mutates_persistent_project": save_intent.get("mutates_persistent_project", "TBD")}],
        )

    private_policy = _text(record.get("private_payload_policy"), "")
    _append_finding(
        findings,
        category="warning_visibility",
        outcome="pass" if private_policy == "references_and_checksums_only_no_private_payload_copy" else "warning",
        severity="major" if not private_policy else "minor",
        source_surface=source,
        affected_control="private_payload_policy",
        remediation_note="Keep private library data as references and checksums in editor review records.",
        evidence_refs=[{"policy": private_policy or "TBD"}],
    )


def _review_warnings(record: Mapping[str, Any], findings: list[dict[str, Any]]) -> None:
    source = _source_surface(record)
    warnings = _list(record.get("warnings"))
    blocking_summary = _mapping(record.get("blocking_summary"))

    _append_finding(
        findings,
        category="warning_visibility",
        outcome="pass" if warnings else "not_applicable",
        severity="major" if warnings else "info",
        source_surface=source,
        affected_control="warnings",
        remediation_note="Show active warning records with class, severity, message, and remediation text.",
        evidence_refs=[{"warning_count": len(warnings)}],
    )

    for warning in warnings:
        item = _mapping(warning)
        warning_id = _text(item.get("warning_id"), "warning:TBD")
        required = ("warning_class", "severity", "message", "remediation")
        missing = [name for name in required if not _has_visible_text(item.get(name))]
        _append_finding(
            findings,
            category="warning_visibility",
            outcome="pass" if not missing else "fail",
            severity="blocking" if missing else _warning_severity(item),
            source_surface=source,
            affected_control=f"warnings[{warning_id}]",
            remediation_note="Keep warning class, severity, message, and remediation available to reviewers.",
            evidence_refs=[
                {"missing_fields": missing},
                {"blocks_progress": bool(item.get("blocks_progress"))},
                {"warning_class": _text(item.get("warning_class"), "TBD")},
            ],
        )
        if item.get("warning_class") == "assumption":
            _append_finding(
                findings,
                category="review_workflow_continuity",
                outcome="pass",
                severity="major",
                source_surface=source,
                affected_control=f"warnings[{warning_id}].assumption_ref",
                remediation_note="Keep assumptions visible in the same review path as diagnostics and results.",
                evidence_refs=[{"assumption_ref": deepcopy(item.get("assumption_ref"))}],
            )

    blocking_ids = _list(blocking_summary.get("blocking_warning_ids"))
    _append_finding(
        findings,
        category="focus_order",
        outcome="pass" if blocking_summary.get("has_blocking_items") == bool(blocking_ids) else "warning",
        severity="major",
        source_surface=source,
        affected_control="blocking_summary",
        remediation_note="Keep blocking-warning summaries synchronized with warning detail rows.",
        evidence_refs=[
            {"has_blocking_items": bool(blocking_summary.get("has_blocking_items"))},
            {"blocking_warning_ids": sorted(map(str, blocking_ids))},
        ],
    )


def _review_results(record: Mapping[str, Any], findings: list[dict[str, Any]]) -> None:
    source = _source_surface(record)
    views = _list(record.get("views"))
    _append_finding(
        findings,
        category="keyboard_path",
        outcome="pass" if views else "warning",
        severity="minor" if views else "major",
        source_surface=source,
        affected_control="views",
        remediation_note="Provide keyboard access to result tables and overlay descriptors.",
        evidence_refs=[{"result_view_count": len(views)}],
    )

    for view in views:
        item = _mapping(view)
        result_id = _text(item.get("result_id"), "result:TBD")
        columns = set(map(str, _list(item.get("table_columns"))))
        required_columns = {"value_id", "numeric_value", "unit"}
        _append_finding(
            findings,
            category="result_review_visibility",
            outcome="pass" if required_columns <= columns else "fail",
            severity="blocking" if not required_columns <= columns else "minor",
            source_surface=source,
            affected_control=f"views[{result_id}].table_columns",
            remediation_note="Keep result identity, numeric values, and units visible in review tables.",
            evidence_refs=[{"table_columns": sorted(columns)}],
        )

        if item.get("availability") == "unresolved_TBD":
            _append_finding(
                findings,
                category="result_review_visibility",
                outcome="warning",
                severity="major",
                source_surface=source,
                affected_control=f"views[{result_id}].availability",
                remediation_note="Keep unresolved result values explicit instead of replacing them with defaults.",
                evidence_refs=[{"availability": "unresolved_TBD"}],
            )

        overlay = _mapping(item.get("overlay_descriptor"))
        if overlay.get("enabled") is True:
            style_token = _text(overlay.get("style_token"), "TBD")
            _append_finding(
                findings,
                category="contrast_readability",
                outcome="warning" if style_token == "TBD" else "pass",
                severity="minor",
                source_surface=source,
                affected_control=f"views[{result_id}].overlay_descriptor.style_token",
                remediation_note="Keep result overlay styling reviewable with a non-color status cue and a recorded style token.",
                evidence_refs=[{"style_token": style_token}],
            )


def _review_solve_execution(record: Mapping[str, Any], findings: list[dict[str, Any]]) -> None:
    source = _source_surface(record)
    timeline = _list(record.get("timeline"))
    final_state = _text(record.get("final_state"), "TBD")
    _append_finding(
        findings,
        category="solve_state_feedback",
        outcome="pass" if timeline else "warning",
        severity="minor" if timeline else "major",
        source_surface=source,
        affected_control="timeline",
        remediation_note="Expose solve state, progress, messages, warnings, and diagnostics in deterministic run-panel records.",
        evidence_refs=[{"timeline_event_count": len(timeline)}],
    )

    missing_feedback = []
    for event in timeline:
        event_record = _mapping(event)
        if not _has_visible_text(event_record.get("state")):
            missing_feedback.append(f"{event_record.get('event_id', 'event:TBD')}:state")
        if not _has_visible_text(event_record.get("message")):
            missing_feedback.append(f"{event_record.get('event_id', 'event:TBD')}:message")
        if event_record.get("progress_percent") == "TBD":
            missing_feedback.append(f"{event_record.get('event_id', 'event:TBD')}:progress")
    _append_finding(
        findings,
        category="solve_state_feedback",
        outcome="pass" if not missing_feedback else "warning",
        severity="major" if missing_feedback else "minor",
        source_surface=source,
        affected_control="timeline.events",
        remediation_note="Keep each solve event state, progress, and message visible to reviewers.",
        evidence_refs=[{"missing_feedback_fields": sorted(missing_feedback)}],
    )

    _append_finding(
        findings,
        category="solve_state_feedback",
        outcome="pass" if final_state in {"cancelled", "completed", "failed"} else "warning",
        severity="major" if final_state not in {"cancelled", "completed", "failed"} else "minor",
        source_surface=source,
        affected_control="final_state",
        remediation_note="Keep non-terminal solve states visible until completion, cancellation, or failure is recorded.",
        evidence_refs=[{"final_state": final_state}],
    )

    cancellation = _mapping(record.get("cancellation"))
    if cancellation.get("requested") is True:
        _append_finding(
            findings,
            category="focus_order",
            outcome="pass",
            severity="major",
            source_surface=source,
            affected_control="cancellation",
            remediation_note="Keep cancellation state reachable and visible without direct solver-process mutation.",
            evidence_refs=[
                {"request_event_ids": sorted(map(str, _list(cancellation.get("request_event_ids"))))},
                {"mutates_solver_process_directly": cancellation.get("mutates_solver_process_directly", "TBD")},
            ],
        )


def _review_required_surface_coverage(
    records: list[Mapping[str, Any]],
    findings: list[dict[str, Any]],
) -> None:
    present = {_text(record.get("deliverable_id"), "DEL:UNKNOWN") for record in records}
    for deliverable_id in REQUIRED_SURFACES:
        _append_finding(
            findings,
            category="review_scope_coverage",
            outcome="pass" if deliverable_id in present else "warning",
            severity="minor" if deliverable_id in present else "major",
            source_surface="DEL-07-06:accessibility_usability_baseline",
            affected_control=f"required_surface[{deliverable_id}]",
            remediation_note="Provide invented GUI contract records for each required baseline surface.",
            evidence_refs=[
                {"required_deliverable_id": deliverable_id},
                {"surface_name": SURFACE_NAMES[deliverable_id]},
            ],
        )


def _review_workflow_continuity(
    records: list[Mapping[str, Any]],
    findings: list[dict[str, Any]],
) -> None:
    present = {_text(record.get("deliverable_id"), "DEL:UNKNOWN") for record in records}
    missing = [item for item in REQUIRED_SURFACES if item not in present]
    _append_finding(
        findings,
        category="review_workflow_continuity",
        outcome="pass" if not missing else "warning",
        severity="major" if missing else "minor",
        source_surface="DEL-07-06:accessibility_usability_baseline",
        affected_control="model_create_warn_solve_result_review_path",
        remediation_note="Keep model creation, missing-data review, solve feedback, assumptions, and result review connected.",
        evidence_refs=[{"missing_surface_ids": missing}],
    )


def _review_runtime_boundary(findings: list[dict[str, Any]]) -> None:
    _append_finding(
        findings,
        category="live_runtime_accessibility_tree",
        outcome="not_applicable",
        severity="info",
        source_surface="DEL-07-06:accessibility_usability_baseline",
        affected_control="desktop_runtime",
        remediation_note="Run desktop accessibility-tree tooling only after the GUI runtime and human target are authorized.",
        evidence_refs=[{"desktop_runtime_evaluation": "not_performed"}],
    )
    _append_finding(
        findings,
        category="contrast_readability",
        outcome="warning",
        severity="minor",
        source_surface="DEL-07-06:accessibility_usability_baseline",
        affected_control="human_selected_accessibility_target",
        remediation_note="Record the human-selected measurement target before treating contrast/readability as measured.",
        evidence_refs=[{"accessibility_target_status": "TBD_by_human_project_authority"}],
    )


def _review_source_diagnostics(
    diagnostics_value: Any,
    record: Mapping[str, Any],
    affected_prefix: str,
    findings: list[dict[str, Any]],
) -> None:
    diagnostics = _list(diagnostics_value)
    source = _source_surface(record)
    for diagnostic in diagnostics:
        item = _mapping(diagnostic)
        code = _text(item.get("code"), _text(item.get("diagnostic_id"), "DIAGNOSTIC:TBD"))
        severity = _text(item.get("severity"), "warning")
        target = item.get("target_ref", item.get("affected_ref", code))
        _append_finding(
            findings,
            category="source_diagnostic_visibility",
            outcome="fail" if severity == "blocking" else "warning",
            severity="blocking" if severity == "blocking" else "major",
            source_surface=source,
            affected_control=f"{affected_prefix}[{code}]",
            remediation_note="Keep upstream diagnostics visible in the review path and do not replace them with defaults.",
            evidence_refs=[
                {"code": code},
                {"source_severity": severity},
                {"target_ref": deepcopy(target)},
            ],
        )


def _review_professional_boundary(
    record: Mapping[str, Any],
    findings: list[dict[str, Any]],
    *,
    source_surface: str | None = None,
    affected_control: str = "professional_boundary",
) -> None:
    source = source_surface or _source_surface(record)
    issues = []
    reviewed = []
    for path, key, value in _walk_key_values(record):
        if key == "human_review_required":
            reviewed.append(path)
            if value is not True:
                issues.append({"path": path, "expected": True, "actual": value})
        elif key.startswith("software_makes_"):
            reviewed.append(path)
            if value is not False:
                issues.append({"path": path, "expected": False, "actual": value})
        elif key == "professional_boundary_preserved":
            reviewed.append(path)
            if value is not True:
                issues.append({"path": path, "expected": True, "actual": value})

    _append_finding(
        findings,
        category="professional_boundary",
        outcome="pass" if reviewed and not issues else "fail" if issues else "warning",
        severity="blocking" if issues else "minor",
        source_surface=source,
        affected_control=affected_control,
        remediation_note="Keep human-review boundaries explicit and all software authority-claim flags false.",
        evidence_refs=[
            {"reviewed_field_count": len(reviewed)},
            {"issues": issues},
        ],
    )


def _review_provenance_and_privacy(record: Mapping[str, Any], findings: list[dict[str, Any]]) -> None:
    source = _source_surface(record)
    provenance_paths = [path for path, key, value in _walk_key_values(record) if key == "provenance" and isinstance(value, Mapping)]
    _append_finding(
        findings,
        category="provenance_visibility",
        outcome="pass" if provenance_paths else "warning",
        severity="minor" if provenance_paths else "major",
        source_surface=source,
        affected_control="provenance",
        remediation_note="Keep source/provenance records visible where GUI review records expose data or diagnostics.",
        evidence_refs=[{"provenance_path_count": len(provenance_paths)}],
    )

    privacy_values = sorted(
        {
            str(value)
            for _path, key, value in _walk_key_values(record)
            if key == "privacy_classification"
        }
    )
    if not privacy_values:
        _append_finding(
            findings,
            category="privacy_boundary",
            outcome="warning",
            severity="major",
            source_surface=source,
            affected_control="privacy_classification",
            remediation_note="Carry privacy classifications through GUI review records.",
            evidence_refs=[],
        )
        return

    disallowed = [
        value
        for value in privacy_values
        if value not in PUBLIC_PRIVACY_CLASSES and value not in REFERENCE_ONLY_PRIVACY_CLASSES
    ]
    _append_finding(
        findings,
        category="privacy_boundary",
        outcome="pass" if not disallowed else "fail",
        severity="blocking" if disallowed else "minor",
        source_surface=source,
        affected_control="privacy_classification",
        remediation_note="Keep invented/public or reference-only privacy classifications explicit in public review records.",
        evidence_refs=[
            {"privacy_classifications": privacy_values},
            {"disallowed_privacy_classifications": disallowed},
        ],
    )


def _review_content_boundary(record: Mapping[str, Any], findings: list[dict[str, Any]]) -> None:
    text = canonical_json(record).lower()
    marker_count = sum(1 for marker in PROTECTED_CONTENT_MARKERS if marker in text)
    _append_finding(
        findings,
        category="content_boundary",
        outcome="pass" if marker_count == 0 else "fail",
        severity="blocking" if marker_count else "minor",
        source_surface=_source_surface(record),
        affected_control="source_contract_text",
        remediation_note="Keep public GUI review records limited to invented or redistributable metadata.",
        evidence_refs=[{"restricted_marker_count": marker_count}],
    )


def _append_finding(
    findings: list[dict[str, Any]],
    *,
    category: str,
    outcome: str,
    severity: str,
    source_surface: str,
    affected_control: str,
    remediation_note: str,
    evidence_refs: Iterable[Any] | None = None,
) -> None:
    base = {
        "category": category,
        "outcome": outcome if outcome in OUTCOMES else "warning",
        "source_surface": source_surface,
        "affected_control": affected_control,
        "severity": severity if severity in SEVERITIES else "major",
        "remediation_note": remediation_note,
        "evidence_refs": sorted(_normalize_evidence(evidence_refs or []), key=canonical_json),
        "professional_boundary_preserved": True,
    }
    base["finding_id"] = "AUB-" + hashlib.sha256(canonical_json(base).encode("utf-8")).hexdigest()[:12]
    findings.append(base)


def _stable_findings(findings: list[dict[str, Any]]) -> list[dict[str, Any]]:
    deduped = {item["finding_id"]: item for item in findings}
    return sorted(deduped.values(), key=canonical_json)


def _summary(findings: list[Mapping[str, Any]]) -> dict[str, Any]:
    by_outcome = Counter(str(item.get("outcome")) for item in findings)
    by_severity = Counter(str(item.get("severity")) for item in findings)
    by_category = Counter(str(item.get("category")) for item in findings)
    return {
        "total_findings": len(findings),
        "by_outcome": {key: by_outcome.get(key, 0) for key in sorted(OUTCOMES)},
        "by_severity": {key: by_severity.get(key, 0) for key in sorted(SEVERITIES)},
        "by_category": {key: by_category[key] for key in sorted(by_category)},
    }


def _surface_summary(record: Mapping[str, Any]) -> dict[str, Any]:
    return {
        "source_surface": _source_surface(record),
        "deliverable_id": _text(record.get("deliverable_id"), "DEL:UNKNOWN"),
        "record_id": _record_id(record),
        "source_fingerprint": hashlib.sha256(canonical_json(record).encode("utf-8")).hexdigest(),
        "diagnostic_count": len(_list(record.get("diagnostics"))),
        "privacy_classifications": sorted(
            {
                str(value)
                for _path, key, value in _walk_key_values(record)
                if key == "privacy_classification"
            }
        ),
    }


def _record_sort_key(record: Mapping[str, Any]) -> str:
    return canonical_json(
        {
            "deliverable_id": _text(record.get("deliverable_id"), "DEL:UNKNOWN"),
            "record_id": _record_id(record),
            "fingerprint": hashlib.sha256(canonical_json(record).encode("utf-8")).hexdigest(),
        }
    )


def _record_id(record: Mapping[str, Any]) -> str:
    for key in (
        "project_ref",
        "editor_set_id",
        "warning_set_id",
        "result_set_id",
        "run_panel_id",
        "baseline_id",
    ):
        if _has_visible_text(record.get(key)):
            return str(record[key])
    session = _mapping(record.get("viewport_session"))
    if _has_visible_text(session.get("session_id")):
        return str(session["session_id"])
    return "record:TBD"


def _source_surface(record: Mapping[str, Any]) -> str:
    deliverable_id = _text(record.get("deliverable_id"), "DEL:UNKNOWN")
    return f"{deliverable_id}:{SURFACE_NAMES.get(deliverable_id, 'unknown_gui_contract')}"


def _warning_severity(warning: Mapping[str, Any]) -> str:
    if warning.get("blocks_progress") is True:
        return "major"
    severity = _text(warning.get("severity"), "warning")
    return "major" if severity in {"blocking", "warning"} else "minor"


def _normalize_evidence(items: Iterable[Any]) -> list[dict[str, Any]]:
    normalized = []
    for item in items:
        if isinstance(item, Mapping):
            normalized.append(deepcopy(dict(item)))
        elif item is not None:
            normalized.append({"value": str(item)})
    return normalized


def _walk_key_values(value: Any, path: str = "$") -> Iterable[tuple[str, str, Any]]:
    if isinstance(value, Mapping):
        for key in sorted(value):
            child = value[key]
            child_path = f"{path}.{key}"
            yield child_path, str(key), child
            yield from _walk_key_values(child, child_path)
    elif isinstance(value, list):
        for index, child in enumerate(value):
            yield from _walk_key_values(child, f"{path}[{index}]")


def _has_visible_text(value: Any) -> bool:
    return _text(value, "") not in {"", "TBD", "None"}


def _mapping(value: Any) -> Mapping[str, Any]:
    return value if isinstance(value, Mapping) else {}


def _list(value: Any) -> list[Any]:
    return value if isinstance(value, list) else []


def _text(value: Any, fallback: str) -> str:
    text = str(value).strip() if value is not None else ""
    return text or fallback
