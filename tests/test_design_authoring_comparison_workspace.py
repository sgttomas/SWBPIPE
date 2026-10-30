#!/usr/bin/env python3
"""Focused tests for DEL-07-08 design-authoring and comparison workspace."""

from __future__ import annotations

from copy import deepcopy
import json
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.comparison.analysis_run.engine import compare_analysis_runs  # noqa: E402
from core.comparison.model_state.engine import compare_model_states  # noqa: E402
from core.constraints.validation.engine import validate_constraint_envelope  # noqa: E402
from core.gui.design_workspace import (  # noqa: E402
    build_design_authoring_comparison_workspace,
    canonical_json,
)
from core.gui.warnings import build_warning_ux_contract  # noqa: E402
from core.model_operations.audit_trail import record_operation_audit_trail  # noqa: E402
from core.model_operations.validation_preview import validate_and_preview_operations  # noqa: E402


FORBIDDEN_CLAIMS = {
    "code compliant",
    "certified",
    "sealed",
    "authenticated",
    "professional approval",
    "engineering acceptance",
    "external validation",
}

OPERATION_FIXTURE = ROOT / "fixtures" / "model_operations" / "invented_operation_set_valid.json"
MODEL_STATE_FIXTURE = ROOT / "fixtures" / "model_operations" / "invented_accepted_model_state.json"


def ref(object_type, value):
    return {"object_type": object_type, "ref": value}


def provenance():
    return {
        "source_name": "invented public test fixture",
        "source_location": "tests/test_design_authoring_comparison_workspace.py",
        "source_license": "project-governed",
        "contributor": "OpenPipeStress test",
        "contributor_certification": "invented non-engineering fixture",
        "redistribution_status": "invented_non_engineering_example",
        "review_status": "pending",
        "privacy_classification": "invented_public_example",
    }


def professional_boundary():
    return {
        "human_review_required": True,
        "software_makes_compliance_claim": False,
        "software_makes_certification_claim": False,
        "software_makes_sealing_claim": False,
        "software_makes_approval_claim": False,
        "software_makes_authentication_claim": False,
    }


def design_knowledge():
    return {
        "schema_version": "0.1.0",
        "deliverable_id": "DEL-13-01",
        "package_id": "PKG-13",
        "scope_item": "SOW-067",
        "objectives": ["OBJ-014"],
        "data_boundary": {
            "public_examples_policy": "invented_or_cleared_data_only",
            "protected_source_policy": "no_bundled_protected_owner_or_standards_data",
            "private_data_policy": "user_controlled_private_paths",
            "unit_policy": "unit_bearing_values_require_explicit_unit_metadata",
            "professional_boundary": professional_boundary(),
        },
        "design_knowledge": {
            "knowledge_set_id": "dk:invented",
            "project_ref": ref("Project", "project:invented"),
            "model_ref": ref("Model", "model:invented"),
            "records": [
                {
                    "id": "dk:line:1",
                    "record_kind": "line_data",
                    "name": "Invented public line",
                    "line_identifier": "L-invented",
                    "service": "TBD",
                    "attributes": [],
                    "source_notes": [],
                    "assumptions": [
                        {
                            "assumption_id": "assumption:line-service",
                            "statement": "Invented service remains unresolved.",
                            "status": "unresolved",
                            "affected_refs": [ref("DesignKnowledgeRecord", "dk:line:1")],
                            "provenance": provenance(),
                        }
                    ],
                    "provenance": provenance(),
                },
                {
                    "id": "dk:req:access",
                    "record_kind": "requirement",
                    "name": "Invented access note",
                    "requirement_type": "access",
                    "statement": "Invented review fixture, not project data.",
                    "parameters": [],
                    "source_notes": [],
                    "assumptions": [],
                    "provenance": provenance(),
                },
            ],
            "diagnostics": [
                {
                    "code": "DK-TBD-SERVICE",
                    "class": "TBD",
                    "severity": "warning",
                    "source": ref("DesignKnowledgeRecord", "dk:line:1"),
                    "affected_object": ref("DesignKnowledgeRecord", "dk:line:1"),
                    "message": "Invented line service is unresolved.",
                    "remediation": "Retain TBD until user-supplied data is available.",
                    "provenance": provenance(),
                }
            ],
            "provenance": provenance(),
        },
    }


def model_state(state_id, entities):
    return {
        "model_state": {
            "state_id": state_id,
            "state_name": f"Invented {state_id}",
            "state_kind": "comparison_basis",
            "created_at": "2026-05-09T00:00:00Z",
            "model_ref": ref("Model", "model:invented"),
            "parent_state_refs": [],
            "tags": [],
            "notes": [],
            "external_references": [],
            "unresolved_assumptions": [
                {
                    "assumption_id": f"assumption:{state_id}",
                    "statement": "Invented unresolved state assumption.",
                    "status": "unresolved",
                    "affected_refs": [ref("ModelState", state_id)],
                    "provenance": provenance(),
                }
            ],
            "warnings": [],
            "analysis_status": ["MODEL_INCOMPLETE"],
            "hashes": [
                {
                    "algorithm": "sha256",
                    "canonicalization": "JCS",
                    "payload_ref": ref("ModelState", state_id),
                    "payload_scope": "model_state_record",
                    "value": f"hash-{state_id}",
                }
            ],
            "immutability_policy": {
                "snapshot_is_read_only": True,
                "mutation_policy": "changes_create_new_model_state",
                "new_state_required_for_change": True,
                "hash_invalidates_external_acceptance": True,
            },
            "professional_boundary": professional_boundary(),
            "provenance": provenance(),
        },
        "entities": entities,
    }


def entity(stable_id, **fields):
    return {
        "stable_id": stable_id,
        "category": "Component",
        "reference": {"object_type": "Entity", "ref": stable_id, "label": stable_id},
        **fields,
    }


def run_record(run_id, model_state_id, result_ref):
    return {
        "analysis_run": {
            "run_id": run_id,
            "run_name": f"{run_id} invented fixture",
            "run_kind": "mechanics_solve",
            "created_at": "2026-05-09T00:00:00Z",
            "model_state_ref": ref("ModelState", model_state_id),
            "solver_version": {"solver_id": "invented-solver", "version": "0.0.0-fixture"},
            "settings_ref": ref("SolverSettings", f"settings:{run_id}"),
            "unit_system_ref": ref("UnitSystem", "fixture-units"),
            "load_basis_refs": [ref("LoadCase", "LC-1")],
            "diagnostics": [],
            "result_refs": [ref("ResultEnvelope", result_ref)],
            "rule_pack_refs": [],
            "library_refs": [],
            "hashes": [
                {
                    "algorithm": "sha256",
                    "canonicalization": "JCS",
                    "payload_ref": ref("AnalysisRun", run_id),
                    "payload_scope": "analysis_run_record",
                    "value": f"hash-{run_id}",
                }
            ],
            "analysis_status": ["MECHANICS_SOLVED", "HUMAN_REVIEW_REQUIRED"],
            "reproducibility": {
                "input_manifest_refs": [],
                "environment_refs": [],
                "determinism_notes": ["invented deterministic fixture"],
                "unresolved_tbd": [],
            },
        }
    }


def result_envelope(envelope_id, run_id, result_id, magnitude, unit):
    return {
        "result_envelope": {
            "envelope_id": envelope_id,
            "run_ref": ref("AnalysisRun", run_id),
            "result_sets": [
                {
                    "result_set_id": f"set:{envelope_id}",
                    "quantity_results": [
                        {
                            "result_id": result_id,
                            "family": "stress",
                            "object_ref": ref("PipeElement", "E1"),
                            "basis_ref": ref("LoadCase", "LC-1"),
                            "magnitude": magnitude,
                            "unit": unit,
                            "dimension": "stress",
                            "provenance": {"source_name": "invented fixture"},
                        }
                    ],
                }
            ],
            "diagnostics": [],
        }
    }


def operation_envelope():
    with OPERATION_FIXTURE.open(encoding="utf-8") as fixture_file:
        envelope = json.load(fixture_file)
    operation = next(
        item
        for item in envelope["operation_set"]["operations"]
        if item["operation_id"] == "op:resize"
    )
    operation["operation_status"] = "ready_for_user_review"
    envelope["operation_set"]["operations"] = [operation]
    return envelope


def accepted_model_state():
    with MODEL_STATE_FIXTURE.open(encoding="utf-8") as fixture_file:
        return json.load(fixture_file)


def fixture_workspace_inputs(*, user_acceptance=False):
    left_state = model_state(
        "state:left",
        [
            entity("component:pipe-1", diameter={"value": 100.0, "unit": "mm", "dimension": "length"}),
            entity("component:left-only", note="left"),
        ],
    )
    right_state = model_state(
        "state:right",
        [
            entity("component:pipe-1", diameter={"value": 125.0, "unit": "mm", "dimension": "length"}),
            entity("component:right-only", note="right"),
        ],
    )
    preview = validate_and_preview_operations(operation_envelope(), accepted_model_state())
    acceptance_signal = {
        "decision": "accept",
        "accepted": True,
        "actor_type": "user" if user_acceptance else "agent",
        "actor_ref": "user:reviewer" if user_acceptance else "agent:assistant",
        "source_role": "project_user" if user_acceptance else "automation",
        "decided_at": "2026-05-09T12:00:00Z",
        "rationale": "Invented review signal for workspace test.",
    }
    audit = record_operation_audit_trail(
        operation_envelope(),
        validation_outcome=preview,
        diff_preview_ref={
            "object_type": "DiffPreview",
            "ref": "preview:resize",
            "hash": "sha256:invented-preview-001",
        },
        acceptance_signal=acceptance_signal,
        accepted_model_state=accepted_model_state(),
    )
    return {
        "workspace_id": "workspace:invented",
        "design_knowledge_envelope": design_knowledge(),
        "constraint_validation": validate_constraint_envelope(None),
        "warning_contract": build_warning_ux_contract(
            warning_set_id="warnings:invented",
            conditions=[
                {
                    "warning_id": "warn:missing-support-stiffness",
                    "warning_class": "incomplete_data",
                    "target_ref": ref("Support", "support:TBD"),
                    "message": "Invented support stiffness remains TBD.",
                    "source_status": "missing",
                }
            ],
        ),
        "model_states": [left_state, right_state],
        "analysis_runs": [
            run_record("run:left", "state:left", "results:left"),
            run_record("run:right", "state:right", "results:right"),
        ],
        "model_state_comparison": compare_model_states(left_state, right_state),
        "analysis_run_comparison": compare_analysis_runs(
            left_run=run_record("run:left", "state:left", "results:left"),
            right_run=run_record("run:right", "state:right", "results:right"),
            left_results=result_envelope("results:left", "run:left", "result:left:stress", 1000.0, "kPa"),
            right_results=result_envelope("results:right", "run:right", "result:right:stress", 1000500.0, "Pa"),
            mappings=[
                {
                    "mapping_id": "map:stress:E1",
                    "mapping_status": "manual_match",
                    "left_ref": ref("Result", "result:left:stress"),
                    "right_ref": ref("Result", "result:right:stress"),
                    "normalized_unit": "Pa",
                }
            ],
            tolerance_profile={
                "tolerance_profile": {
                    "profile_id": "tol:invented",
                    "rules": [
                        {
                            "rule_id": "tol-rule:stress",
                            "result_family": "stress",
                            "dimension_id": "stress",
                            "unit_ref": ref("Unit", "Pa"),
                            "tolerance_value": 0.25,
                        }
                    ],
                }
            },
            unit_conversions={("kPa", "Pa", "stress"): 1000.0},
            comparison_id="comparison:analysis",
        ),
        "tolerance_profile": {
            "tolerance_profile": {
                "profile_id": "tol:invented",
                "profile_status": "project_specific_review_required",
                "rules": [{"rule_id": "tol-rule:stress"}],
                "diagnostics": [],
                "assumptions": [],
                "hashes": [],
                "review": {"status": "pending"},
                "provenance": provenance(),
            }
        },
        "operation_preview": preview,
        "operation_audit": audit,
        "selected_refs": {"panel_id": "comparison_tables", "row_id": "component:pipe-1"},
    }


def test_workspace_is_deterministic_and_preserves_review_records():
    inputs = fixture_workspace_inputs()
    first = build_design_authoring_comparison_workspace(**inputs)
    second = build_design_authoring_comparison_workspace(**deepcopy(inputs))

    assert canonical_json(first) == canonical_json(second)
    assert first["deliverable_id"] == "DEL-07-08"
    assert first["workspace_hash"].startswith("sha256:")
    assert first["design_knowledge_panel"]["summary"]["record_count"] == 2
    assert first["design_knowledge_panel"]["summary"]["records_with_unresolved_tbd"] == 1
    assert first["constraint_warning_panel"]["warnings"][0]["warning_class"] == "incomplete_data"
    assert first["state_run_browser"]["summary"]["model_state_count"] == 2
    assert first["comparison_tables"]["model_state_rows"]["unmatched_rows"]
    assert first["comparison_tables"]["analysis_run_rows"]["result_rows"][0]["tolerance_profile_ref"] == "tol:invented"
    assert first["graphical_overlays"]["overlays"]
    assert first["mutation_boundary"]["workspace_mutates_accepted_model_state"] is False
    assert first["persistence_hash_contract"]["contract_ref"] == "DEL-02-05"
    assert first["state_run_browser"]["persistence_hash_boundary"]["model_state_hashes_present"] is True
    assert first["state_run_browser"]["persistence_hash_boundary"]["analysis_run_hashes_present"] is True
    assert first["state_run_browser"]["model_states"][0]["hash_boundary"]["hashes_present"] is True


def test_operation_review_requires_explicit_user_acceptance_record():
    without_user = build_design_authoring_comparison_workspace(**fixture_workspace_inputs(user_acceptance=False))
    row = without_user["operation_diff_review"]["rows"][0]

    assert row["can_be_represented_as_accepted_operation"] is False
    assert row["review_state"] == "held_for_user_acceptance"
    assert row["accepted_model_state_mutated_by_workspace"] is False

    with_user = build_design_authoring_comparison_workspace(**fixture_workspace_inputs(user_acceptance=True))
    row = with_user["operation_diff_review"]["rows"][0]

    assert row["can_be_represented_as_accepted_operation"] is True
    assert row["review_state"] == "explicit_user_acceptance_recorded"
    assert row["workspace_application_status"] == "not_applied"


def test_missing_inputs_and_mutation_signals_remain_visible():
    missing = build_design_authoring_comparison_workspace(workspace_id="workspace:missing")

    assert missing["review_state_routing"]["route_state"] == "input_unavailable_review"
    assert "design_knowledge" in missing["review_state_routing"]["unavailable_panel_ids"]
    assert "WORKSPACE-DESIGN-KNOWLEDGE-MISSING" in {item["code"] for item in missing["diagnostics"]}

    inputs = fixture_workspace_inputs()
    unsafe_preview = deepcopy(inputs["operation_preview"])
    unsafe_preview["diff_preview"][0]["application_status"] = "applied"
    inputs["operation_preview"] = unsafe_preview
    output = build_design_authoring_comparison_workspace(**inputs)

    assert "WORKSPACE-OPERATION-MUTATION-SIGNAL-BLOCKED" in {item["code"] for item in output["diagnostics"]}
    assert output["operation_diff_review"]["rows"][0]["workspace_application_status"] == "not_applied"


def test_output_boundary_language_does_not_make_prohibited_claims():
    output = build_design_authoring_comparison_workspace(**fixture_workspace_inputs())
    text = canonical_json(output).lower()

    for forbidden in FORBIDDEN_CLAIMS:
        assert forbidden not in text
    assert output["professional_boundary"]["software_makes_approval_claim"] is False


def main():
    test_workspace_is_deterministic_and_preserves_review_records()
    test_operation_review_requires_explicit_user_acceptance_record()
    test_missing_inputs_and_mutation_signals_remain_visible()
    test_output_boundary_language_does_not_make_prohibited_claims()


if __name__ == "__main__":
    main()
