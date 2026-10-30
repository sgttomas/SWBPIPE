#!/usr/bin/env python3
"""Focused tests for DEL-07-06 accessibility/usability baseline records."""

from copy import deepcopy
import json
from pathlib import Path
import sys


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.gui.accessibility import (  # noqa: E402
    build_accessibility_usability_baseline,
    canonical_json,
)
from core.gui.editors import build_editor_contract  # noqa: E402
from core.gui.model_tree import build_model_tree_property_inspector  # noqa: E402
from core.gui.results_viewer import build_results_viewer_contract  # noqa: E402
from core.gui.solve_execution import build_solve_execution_ux  # noqa: E402
from core.gui.warnings import build_warning_ux_contract  # noqa: E402


VIEWPORT_FIXTURE = ROOT / "fixtures" / "gui" / "invented" / "viewport_editor_session.json"
REQUIRED_SURFACES = {"DEL-07-01", "DEL-07-02", "DEL-07-03", "DEL-07-04", "DEL-07-05", "DEL-07-07"}
REQUIRED_FINDING_FIELDS = {
    "finding_id",
    "category",
    "outcome",
    "source_surface",
    "affected_control",
    "severity",
    "remediation_note",
    "evidence_refs",
}
REQUIRED_CATEGORIES = {
    "keyboard_path",
    "focus_order",
    "readable_label",
    "warning_visibility",
    "result_review_visibility",
    "solve_state_feedback",
    "review_workflow_continuity",
    "contrast_readability",
    "source_diagnostic_visibility",
    "professional_boundary",
    "privacy_boundary",
}
FORBIDDEN_TEXT = (
    "code " + "compliant",
    "certif" + "ied",
    "seal" + "ed",
    "professional " + "acceptance",
    "professional " + "approval",
    "accessibility " + "certified",
    "real " + "secret",
)


def load_viewport_contract():
    with VIEWPORT_FIXTURE.open(encoding="utf-8") as json_file:
        return json.load(json_file)


def invented_contract_records():
    return [
        load_viewport_contract(),
        build_model_tree_property_inspector(
            project_id="invented-accessibility-project",
            selected_ref={"ref_type": "pipe_run", "ref_id": "pipe-P1"},
            entities=[
                {
                    "entity_id": "node-N1",
                    "entity_type": "node",
                    "label": "Invented node N1",
                    "sort_key": "001",
                    "properties": [
                        {
                            "field_id": "elevation",
                            "label": "Elevation",
                            "value": 12.0,
                            "unit": "m",
                            "editable": True,
                        }
                    ],
                    "provenance_state": "invented_public_example",
                    "privacy_classification": "invented_public_example",
                },
                {
                    "entity_id": "pipe-P1",
                    "entity_type": "pipe_run",
                    "label": "Invented pipe P1",
                    "sort_key": "002",
                    "properties": [
                        {
                            "field_id": "nominal_size",
                            "label": "Nominal size",
                            "value": "TBD",
                            "unit": "mm",
                            "editable": True,
                        }
                    ],
                    "privacy_classification": "invented_public_example",
                },
            ],
        ),
        build_editor_contract(
            editor_set_id="invented-accessibility-editor-set",
            editors=[
                {
                    "editor_id": "material-editor",
                    "editor_kind": "material",
                    "target_ref": {"ref_type": "material", "ref_id": "mat-public-1"},
                    "library_classification": "invented_public_example",
                    "fields": [
                        {
                            "field_id": "density",
                            "label": "Density",
                            "value": 7850.0,
                            "unit": "kg/m^3",
                            "dimension": "density",
                            "source_ref": {"ref_type": "invented_source", "ref_id": "src-1"},
                        }
                    ],
                },
                {
                    "editor_id": "rule-pack-reference",
                    "editor_kind": "rule_pack_reference",
                    "target_ref": {"ref_type": "rule_pack", "ref_id": "rp-invented-ref"},
                    "library_classification": "private_reference_only",
                    "fields": [{"field_id": "rule_pack_name", "label": "Rule pack name", "value": "TBD"}],
                    "rule_pack_lifecycle": {"state": "referenced"},
                },
            ],
        ),
        build_warning_ux_contract(
            warning_set_id="invented-accessibility-warning-set",
            conditions=[
                {
                    "warning_id": "warn-missing-support",
                    "warning_class": "incomplete_data",
                    "target_ref": {"ref_type": "support", "ref_id": "support-TBD"},
                    "message": "Invented support has unresolved stiffness.",
                    "source_status": "missing",
                },
                {
                    "warning_id": "warn-assumption",
                    "warning_class": "assumption",
                    "target_ref": {"ref_type": "load_case", "ref_id": "load-1"},
                    "source_status": "provided",
                    "assumption_ref": {"ref_type": "assumption", "ref_id": "assumption-1"},
                },
            ],
        ),
        build_results_viewer_contract(
            result_set_id="invented-accessibility-results",
            result_items=[
                {
                    "result_id": "disp-node-1",
                    "result_kind": "displacement",
                    "analysis_status": ["MECHANICS_SOLVED", "HUMAN_REVIEW_REQUIRED"],
                    "hashes": [
                        {
                            "algorithm": "sha256",
                            "canonicalization": "JCS",
                            "payload_ref": {"ref_type": "ResultEnvelope", "ref_id": "results:accessibility"},
                            "value": "sha256:invented-accessibility-result",
                        }
                    ],
                    "provenance_refs": [{"ref_type": "AnalysisRun", "ref_id": "run:accessibility"}],
                    "values": [
                        {
                            "value_id": "ux",
                            "component": "UX",
                            "numeric_value": 1.2,
                            "unit": "mm",
                            "dimension": "displacement",
                            "location_ref": {"ref_type": "node", "ref_id": "N1"},
                        }
                    ],
                    "overlay_enabled": True,
                    "overlay_target_ref": {"ref_type": "node", "ref_id": "N1"},
                },
                {
                    "result_id": "stress-tbd",
                    "result_kind": "stress",
                    "values": [{"value_id": "smax", "numeric_value": "TBD", "unit": "MPa", "dimension": "stress"}],
                },
            ],
        ),
        build_solve_execution_ux(
            run_panel_id="invented-accessibility-run-panel",
            events=[
                {"event_id": "queued", "state": "queued", "progress_percent": 0},
                {
                    "event_id": "running",
                    "state": "running",
                    "progress_percent": 40,
                    "diagnostic_refs": [{"ref_type": "diagnostic", "ref_id": "diag-1"}],
                    "warning_refs": [{"ref_type": "warning", "ref_id": "warn-missing-support"}],
                },
            ],
        ),
    ]


def main():
    records = invented_contract_records()
    records_before = canonical_json(deepcopy(records))

    baseline = build_accessibility_usability_baseline(
        baseline_id="invented-accessibility-baseline",
        gui_contract_records=records,
    )
    again = build_accessibility_usability_baseline(
        baseline_id="invented-accessibility-baseline",
        gui_contract_records=list(reversed(records)),
    )

    assert canonical_json(baseline) == canonical_json(again)
    assert canonical_json(records) == records_before
    assert baseline["deliverable_id"] == "DEL-07-06"
    assert baseline["review_mode"] == "deterministic_contract_records_no_live_desktop_runtime"
    assert baseline["accessibility_target_status"] == "TBD_by_human_project_authority"
    assert baseline["desktop_runtime_evaluation"] == "not_performed"
    assert baseline["review_policy"]["auto_fill_missing_data"] is False
    assert baseline["review_policy"]["mutates_gui_contract_records"] is False
    assert baseline["review_policy"]["software_makes_accessibility_conformance_claim"] is False
    assert baseline["professional_boundary"]["software_makes_compliance_claim"] is False

    surface_ids = {item["deliverable_id"] for item in baseline["source_surfaces"]}
    assert surface_ids == REQUIRED_SURFACES

    findings = baseline["findings"]
    assert findings == sorted(findings, key=canonical_json)
    assert all(REQUIRED_FINDING_FIELDS <= set(item) for item in findings)
    assert all(item["source_surface"] for item in findings)
    assert all(item["affected_control"] for item in findings)
    assert all(item["remediation_note"] for item in findings)
    assert REQUIRED_CATEGORIES <= {item["category"] for item in findings}
    assert {"pass", "warning", "fail", "not_applicable"} <= {item["outcome"] for item in findings}
    assert baseline["summary"]["by_outcome"]["fail"] >= 1
    assert baseline["summary"]["by_outcome"]["warning"] >= 1
    assert baseline["summary"]["by_outcome"]["not_applicable"] >= 1

    text = canonical_json(baseline)
    for expected in (
        "PROPERTY_VALUE_UNRESOLVED",
        "RULE_PACK_CHECKSUM_MISSING",
        "EDITOR_FIELD_VALUE_UNRESOLVED",
        "WARNING_SOURCE_STATUS_UNRESOLVED",
        "RESULT_VALUE_OR_UNIT_UNRESOLVED",
        "SOLVE_UX_TERMINAL_STATE_PENDING",
        "unresolved_TBD",
    ):
        assert expected in text

    lowered = text.lower()
    for term in FORBIDDEN_TEXT:
        assert term not in lowered


def test_accessibility_usability_baseline_main():
    main()


def test_gui_baseline_remains_verified_not_validated_with_contrast_target_held():
    baseline = build_accessibility_usability_baseline(
        baseline_id="invented-accessibility-validation-boundary",
        gui_contract_records=invented_contract_records(),
    )
    contrast_findings = [
        item for item in baseline["findings"] if item["category"] == "contrast_readability"
    ]

    assert baseline["review_mode"] == "deterministic_contract_records_no_live_desktop_runtime"
    assert baseline["desktop_runtime_evaluation"] == "not_performed"
    assert baseline["accessibility_target_status"] == "TBD_by_human_project_authority"
    assert baseline["review_policy"]["software_makes_accessibility_conformance_claim"] is False
    assert contrast_findings
    assert all(item["outcome"] == "warning" for item in contrast_findings)
    assert any(
        item["affected_control"] == "human_selected_accessibility_target"
        for item in contrast_findings
    )


if __name__ == "__main__":
    main()
