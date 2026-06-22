#!/usr/bin/env python3
"""Tests for the TP-MAC-01 product-preview service slice."""

from pathlib import Path
import sys


ROOT = Path(__file__).resolve().parents[2]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.product_preview import (  # noqa: E402
    build_analysis_run_preview,
    build_agent_proposal_preview,
    build_model_tree,
    build_report_packet_preview,
    load_design_knowledge,
    load_preview_model,
    run_preview_mechanics,
    validate_preview_model,
)
from core.product_preview.service import canonical_json  # noqa: E402


def test_preview_model_fixture_is_valid_and_invented():
    model = load_preview_model()
    validation = validate_preview_model(model)

    assert model["document_kind"] == "openpipestress.product_preview.model"
    assert validation["status"] == "passed"
    assert model["data_boundary"]["public_examples_policy"] == "invented_or_cleared_data_only"
    assert "protected_owner_or_standards_data" in model["data_boundary"]["protected_source_policy"]
    assert "code compliant" not in canonical_json(model).lower()


def test_model_tree_preserves_stable_entity_ids():
    tree = build_model_tree()
    entity_ids = {item["id"] for item in tree["entities"]}

    assert {"node:N-100", "pipe:P-120", "support:S-130", "component:C-140"} <= entity_ids
    assert all(item["properties"] for item in tree["entities"])


def test_design_knowledge_has_visible_provenance_and_diagnostics():
    knowledge = load_design_knowledge()

    assert knowledge["records"]
    assert all(record["provenance"] == "invented_example" for record in knowledge["records"])
    assert {item["code"] for item in knowledge["diagnostics"]} == {
        "RULE_CHECK_NOT_PERFORMED",
        "SUPPORT_STIFFNESS_UNRESOLVED",
    }


def test_mechanics_result_keeps_status_boundaries_separate():
    result = run_preview_mechanics()
    result_ids = {item["id"] for item in result["results"]}

    assert result["status"]["mechanics"] == "MECHANICS_SOLVED"
    assert result["status"]["rule_check"] == "RULE_INPUTS_INCOMPLETE"
    assert result["status"]["professional_acceptance"] == "NOT_PROVIDED"
    assert result["accepted_model_state_mutated"] is False
    assert "RULE_CHECK_INPUTS_MISSING" in {item["code"] for item in result["diagnostics"]}
    assert "COMBINATION_STRESS_SUMMARY_SKIPPED" in {item["code"] for item in result["diagnostics"]}
    assert result["summary"]["load_case_count"] == 2
    assert result["summary"]["component_pressure_thrust_load_count"] == 2
    assert result["summary"]["max_displacement"]["result_ref"] == "result:disp:node-N-140"
    assert "result:force:pipe-P-120:axial" in result_ids
    assert "result:force:pipe-P-120:axial:end-j" in result_ids
    assert "result:force:pipe-P-120:shear-y" in result_ids
    assert "result:force:pipe-P-120:shear-y:end-j" in result_ids
    assert "result:force:pipe-P-120:quarter-1:shear-y" in result_ids
    assert "result:force:pipe-P-120:midspan:axial" in result_ids
    assert "result:force:pipe-P-120:midspan:shear-z" in result_ids
    assert "result:force:pipe-P-120:quarter-3:shear-z" in result_ids
    assert "result:loadcase:load-L-200:force:pipe-P-120:axial" in result_ids
    assert "result:combination:combination-C-OPER-ALT:force:pipe-P-120:axial" in result_ids
    assert "result:combination:combination-C-OPER-ALT:force:pipe-P-120:quarter-1:shear-y" in result_ids
    assert "result:moment:pipe-P-120:bending-z" in result_ids
    assert "result:moment:pipe-P-120:bending-z:end-j" in result_ids
    assert "result:moment:pipe-P-120:quarter-1:bending-z" in result_ids
    assert "result:moment:pipe-P-120:midspan:bending-z" in result_ids
    assert "result:stress:pipe-P-120" in result_ids
    assert "result:stress:pipe-P-120:end-i:axial-normal" in result_ids
    assert "result:stress:pipe-P-120:end-j:torsional-shear" in result_ids
    assert "result:stress:pipe-P-120:quarter-1:torsional-shear" in result_ids
    assert "result:stress:pipe-P-120:midspan:torsional-shear" in result_ids
    assert "result:stress:pipe-P-120:end-i:pressure-hoop" in result_ids
    assert "result:stress:pipe-P-120:quarter-1:pressure-hoop" in result_ids
    assert "result:stress:pipe-P-120:quarter-1:pressure-longitudinal" not in result_ids
    assert "result:stress:pipe-P-120:quarter-1:shear-y" not in result_ids
    assert "result:nonlinear-support:iteration-count" in result_ids
    assert "result:nonlinear-support:final-residual-count" in result_ids
    assert "result:nonlinear-support:converged-flag" in result_ids
    assert "result:nonlinear-support:support-NL-140:state-code" in result_ids
    assert "result:nonlinear-support:support-NL-140:uy-reaction" in result_ids
    assert "result:nonlinear-support:support-NL-130-FRIC:state-code" in result_ids
    assert "result:nonlinear-support:support-NL-130-FRIC:uz-displacement" in result_ids
    assert "result:nonlinear-support:support-NL-130-FRIC:uz-reaction" in result_ids
    assert "result:nonlinear-support:support-NL-130-FRIC:friction-normal-reaction" in result_ids
    assert "result:pressure-thrust:component-C-150" in result_ids
    assert "result:loadcase:load-L-200:pressure-thrust:component-C-150" in result_ids
    assert "result:combination:combination-C-OPER-ALT:pressure-thrust:component-C-150" in result_ids
    assert "TOLERANCE_POLICY_TBD" not in {item["code"] for item in result["diagnostics"]}
    assert "NONLINEAR_SUPPORT_LOOP_CONVERGED" in {item["code"] for item in result["diagnostics"]}
    assert "EXPANSION_JOINT_PRESSURE_THRUST_APPLIED" in {item["code"] for item in result["diagnostics"]}
    axial = next(item for item in result["results"] if item["id"] == "result:force:pipe-P-120:axial")
    axial_end_j = next(item for item in result["results"] if item["id"] == "result:force:pipe-P-120:axial:end-j")
    nonlinear_iteration_count = next(
        item for item in result["results"] if item["id"] == "result:nonlinear-support:iteration-count"
    )
    nonlinear_free_force_residual = next(
        item for item in result["results"] if item["id"] == "result:nonlinear-support:free-dof-force-residual"
    )
    nonlinear_free_moment_residual = next(
        item for item in result["results"] if item["id"] == "result:nonlinear-support:free-dof-moment-residual"
    )
    nonlinear_free_work_residual = next(
        item for item in result["results"] if item["id"] == "result:nonlinear-support:free-dof-work-residual"
    )
    assert "DEC-046-CV-B-product-preview-active-set-count-v1" in nonlinear_iteration_count["metadata"]["basis"]
    assert "policy_status=accepted" in nonlinear_iteration_count["metadata"]["basis"]
    for residual in (nonlinear_free_force_residual, nonlinear_free_moment_residual):
        assert (
            "DEC-046-CV-B-product-preview-free-dof-force-moment-residual-v1"
            in residual["metadata"]["basis"]
        )
        assert "threshold_policy_status=accepted" in residual["metadata"]["basis"]
        assert "residual_basis=free_dof_force_moment_equilibrium" in residual["metadata"]["basis"]
        assert "threshold=TBD" not in residual["metadata"]["basis"]
    assert "DEC-046-CV-B-product-preview-free-dof-work-residual-v1" in nonlinear_free_work_residual["metadata"]["basis"]
    assert "threshold_policy_status=accepted" in nonlinear_free_work_residual["metadata"]["basis"]
    assert "residual_basis=free_dof_work_residual" in nonlinear_free_work_residual["metadata"]["basis"]
    assert "DEC-046-CV-B-product-preview-general-energy-residual-v1" in nonlinear_free_work_residual["metadata"]["basis"]
    assert "general_energy_threshold_policy_status=accepted" in nonlinear_free_work_residual["metadata"]["basis"]
    assert "general_energy_threshold=0 N*m" in nonlinear_free_work_residual["metadata"]["basis"]
    assert "general_energy_threshold=TBD" not in nonlinear_free_work_residual["metadata"]["basis"]
    assert "observed_residual_only" not in nonlinear_free_work_residual["metadata"]["basis"]
    nonlinear_loop_messages = [
        item["message"]
        for item in result["diagnostics"]
        if item["code"] == "NONLINEAR_SUPPORT_LOOP_CONVERGED"
    ]
    assert any(
        "DEC-046-CV-B-product-preview-general-energy-residual-v1" in message
        for message in nonlinear_loop_messages
    )
    assert any(
        "DEC-046-CV-B-product-preview-displacement-reaction-delta-threshold-v1" in message
        for message in nonlinear_loop_messages
    )
    nonlinear_reaction = next(
        item for item in result["results"] if item["id"] == "result:nonlinear-support:support-NL-140:uy-reaction"
    )
    nonlinear_friction_state = next(
        item
        for item in result["results"]
        if item["id"] == "result:nonlinear-support:support-NL-130-FRIC:state-code"
    )
    nonlinear_friction_displacement = next(
        item
        for item in result["results"]
        if item["id"] == "result:nonlinear-support:support-NL-130-FRIC:uz-displacement"
    )
    nonlinear_friction_reaction = next(
        item
        for item in result["results"]
        if item["id"] == "result:nonlinear-support:support-NL-130-FRIC:uz-reaction"
    )
    nonlinear_friction_normal = next(
        item
        for item in result["results"]
        if item["id"] == "result:nonlinear-support:support-NL-130-FRIC:friction-normal-reaction"
    )
    shear_quarter = next(
        item
        for item in result["results"]
        if item["id"] == "result:force:pipe-P-120:quarter-1:shear-y"
    )
    torsional_stress_end_j = next(
        item
        for item in result["results"]
        if item["id"] == "result:stress:pipe-P-120:end-j:torsional-shear"
    )
    pressure_hoop = next(
        item
        for item in result["results"]
        if item["id"] == "result:stress:pipe-P-120:end-i:pressure-hoop"
    )
    combination_axial = next(
        item
        for item in result["results"]
        if item["id"] == "result:combination:combination-C-OPER-ALT:force:pipe-P-120:axial"
    )
    pressure_thrust = next(
        item for item in result["results"] if item["id"] == "result:pressure-thrust:component-C-150"
    )
    pressure_thrust_combination = next(
        item
        for item in result["results"]
        if item["id"] == "result:combination:combination-C-OPER-ALT:pressure-thrust:component-C-150"
    )
    assert axial["metadata"]["coordinate_system"] == "element_local"
    assert axial["metadata"]["location"] == "end_i"
    assert axial["metadata"]["component"] == "axial_force"
    assert axial["basis_ref"] == {"ref_type": "load_case", "ref_id": "load:L-100"}
    assert nonlinear_iteration_count["kind"] == "nonlinear_support_active_set_iteration_count"
    assert nonlinear_iteration_count["value"] == 1
    assert nonlinear_reaction["kind"] == "nonlinear_support_final_reaction"
    assert nonlinear_reaction["entity_ref"] == "support:NL-140"
    assert nonlinear_reaction["value"] < 0
    assert nonlinear_friction_state["value"] == 3
    assert nonlinear_friction_state["metadata"]["basis"].endswith("final_state=sliding")
    assert nonlinear_friction_displacement["value"] != 0
    assert nonlinear_friction_reaction["value"] == 0
    assert nonlinear_friction_normal["kind"] == "nonlinear_support_friction_normal_reaction_derived"
    assert nonlinear_friction_normal["value"] == 49.010116
    assert "derived_support_reaction" in nonlinear_friction_normal["metadata"]["basis"]
    assert "source_ref=support:S-130" in nonlinear_friction_normal["metadata"]["basis"]
    assert "source_dof=uy" in nonlinear_friction_normal["metadata"]["basis"]
    assert "derived_normal_force_model=TBD" not in nonlinear_friction_normal["metadata"]["basis"]
    assert axial_end_j["metadata"]["coordinate_system"] == "element_local"
    assert axial_end_j["metadata"]["location"] == "end_j"
    assert axial_end_j["metadata"]["component"] == "axial_force"
    assert shear_quarter["metadata"]["coordinate_system"] == "element_local"
    assert shear_quarter["metadata"]["location"] == "quarter_1"
    assert shear_quarter["metadata"]["component"] == "shear_force_y"
    assert shear_quarter["metadata"]["basis"] == "interpolated_from_endpoint_resultants"
    assert torsional_stress_end_j["unit"] == "MPa"
    assert torsional_stress_end_j["metadata"]["coordinate_system"] == "element_local"
    assert torsional_stress_end_j["metadata"]["location"] == "end_j"
    assert torsional_stress_end_j["metadata"]["component"] == "torsional_shear_stress"
    assert (
        torsional_stress_end_j["metadata"]["basis"]
        == "recovered_from_open_mechanics_stress_components"
    )
    assert pressure_hoop["metadata"]["coordinate_system"] == "pipe_section"
    assert pressure_hoop["metadata"]["component"] == "pressure_hoop_stress"
    assert combination_axial["basis_ref"] == {
        "ref_type": "combination",
        "ref_id": "combination:C-OPER-ALT",
    }
    assert combination_axial["source_result_refs"] == [
        "result:force:pipe-P-120:axial",
        "result:loadcase:load-L-200:force:pipe-P-120:axial",
    ]
    assert combination_axial["metadata"]["basis"] == "explicit_user_linear_combination"
    assert pressure_thrust["kind"] == "expansion_joint_pressure_thrust_load_review"
    assert pressure_thrust["value"] == 21600
    assert pressure_thrust["unit"] == "N"
    assert pressure_thrust["source_result_refs"] == ["load:L-100-P-EJ"]
    assert pressure_thrust["metadata"]["location"] == "pipe:P-130"
    assert "effective_area=0.018" in pressure_thrust["metadata"]["basis"]
    assert "load_side_user_effective_area" in pressure_thrust["metadata"]["basis"]
    assert pressure_thrust_combination["value"] == 27000
    assert pressure_thrust_combination["source_result_refs"] == [
        "result:pressure-thrust:component-C-150",
        "result:loadcase:load-L-200:pressure-thrust:component-C-150",
    ]


def test_analysis_run_preview_binds_mechanics_results_to_immutable_run_record():
    preview = build_analysis_run_preview()
    run = preview["analysis_run"]
    result_refs = {item["result_ref"]["ref"]: item for item in run["result_refs"]}
    load_basis_refs = {(item["object_type"], item["ref"]) for item in run["load_basis_refs"]}

    assert preview["deliverable_id"] == "DEL-14-02"
    assert run["run_id"] == "run:preview-linear-static-001"
    assert run["immutability_policy"]["run_record_is_read_only"] is True
    assert "HUMAN_REVIEW_REQUIRED" in run["analysis_status"]
    assert "result:force:pipe-P-120:axial" in result_refs
    assert "result:force:pipe-P-120:axial:end-j" in result_refs
    assert "result:force:pipe-P-120:midspan:axial" in result_refs
    assert "result:force:pipe-P-120:quarter-1:shear-y" in result_refs
    assert "result:force:pipe-P-120:shear-y" in result_refs
    assert "result:combination:combination-C-OPER-ALT:force:pipe-P-120:axial" in result_refs
    assert "result:combination:combination-C-OPER-ALT:force:pipe-P-120:quarter-1:shear-y" in result_refs
    assert "result:stress:pipe-P-120:end-j:torsional-shear" in result_refs
    assert "result:stress:pipe-P-120:quarter-1:torsional-shear" in result_refs
    assert result_refs["result:force:pipe-P-120:axial"]["result_family"] == "force"
    assert result_refs["result:combination:combination-C-OPER-ALT:force:pipe-P-120:axial"]["result_family"] == "force"
    assert result_refs["result:force:pipe-P-120:axial"]["hash_refs"][0]["payload_scope"] == "result_value"
    assert (
        result_refs["result:combination:combination-C-OPER-ALT:force:pipe-P-120:axial"]["hash_refs"][0][
            "payload_scope"
        ]
        == "result_value"
    )
    assert result_refs["result:force:pipe-P-120:axial:end-j"]["result_family"] == "force"
    assert result_refs["result:stress:pipe-P-120:end-j:torsional-shear"]["result_family"] == "stress"
    assert {
        ("LoadCase", "load:L-100"),
        ("LoadCase", "load:L-200"),
        ("Combination", "combination:C-OPER-ALT"),
    } <= load_basis_refs
    assert run["professional_boundary"]["software_makes_compliance_claim"] is False


def test_report_packet_preview_materializes_read_only_audit_context():
    packet = build_report_packet_preview()

    assert packet["document_kind"] == "openpipestress.product_preview.report_packet"
    assert packet["packet_id"] == "report-packet:run:preview-linear-static-001"
    assert packet["source_run_ref"] == {
        "object_type": "AnalysisRun",
        "ref": "run:preview-linear-static-001",
    }
    assert "result:disp:node-N-140" in packet["selected_result_refs"]
    assert "result:force:pipe-P-120:axial" in packet["selected_result_refs"]
    assert "result:force:pipe-P-120:axial:end-j" in packet["selected_result_refs"]
    assert "result:force:pipe-P-120:midspan:axial" in packet["selected_result_refs"]
    assert "result:force:pipe-P-120:quarter-1:shear-y" in packet["selected_result_refs"]
    assert "result:force:pipe-P-120:shear-y" in packet["selected_result_refs"]
    assert "result:combination:combination-C-OPER-ALT:force:pipe-P-120:axial" in packet["selected_result_refs"]
    assert (
        "result:combination:combination-C-OPER-ALT:force:pipe-P-120:quarter-1:shear-y"
        in packet["selected_result_refs"]
    )
    assert "result:stress:pipe-P-120:end-j:torsional-shear" in packet["selected_result_refs"]
    assert "result:stress:pipe-P-120:quarter-1:torsional-shear" in packet["selected_result_refs"]
    assert packet["analysis_run_context"]["deliverable_id"] == "DEL-14-02"
    assert packet["analysis_run_context"]["immutability_policy"]["run_record_is_read_only"] is True
    assert packet["analysis_run_context"]["result_value_hash_count"] >= 1
    assert packet["analysis_run_context"]["result_envelope_hash_refs"][0]["payload_scope"] == "result_envelope"
    assert packet["proposal_context"]["application_status"] == "not_applied"
    assert packet["proposal_context"]["accepted_model_state_mutated"] is False
    assert packet["report_packet_status"] == {
        "materialization": "read_only_context_packet",
        "rendered_calculation_report": False,
        "result_export_payload": False,
        "external_handoff_payload": False,
        "professional_acceptance_record": False,
    }
    assert packet["professional_boundary"]["software_makes_compliance_claim"] is False
    assert packet["privacy_boundary"]["private_payload_embedded"] is False
    assert packet["privacy_boundary"]["protected_payload_embedded"] is False
    assert any(item["payload_scope"] == "report_packet_context" for item in packet["hash_refs"])
    assert any(
        item["payload_ref"] == {
            "object_type": "Result",
            "ref": "result:force:pipe-P-120:axial",
        }
        for item in packet["hash_refs"]
    )
    assert any(
        item["payload_ref"] == {
            "object_type": "Result",
            "ref": "result:force:pipe-P-120:axial:end-j",
        }
        for item in packet["hash_refs"]
    )
    assert any(
        item["payload_ref"] == {
            "object_type": "Result",
            "ref": "result:combination:combination-C-OPER-ALT:force:pipe-P-120:axial",
        }
        for item in packet["hash_refs"]
    )
    assert any(
        item["payload_ref"] == {
            "object_type": "Result",
            "ref": "result:stress:pipe-P-120:end-j:torsional-shear",
        }
        for item in packet["hash_refs"]
    )
    assert "code compliant" not in canonical_json(packet).lower()
    assert "professional approval" not in canonical_json(packet).lower()


def test_preview_validation_blocks_empty_ids_and_missing_provenance():
    model = load_preview_model()
    model["nodes"][0]["id"] = ""
    del model["pipe_segments"][0]["provenance"]

    validation = validate_preview_model(model)
    codes = {item["code"] for item in validation["diagnostics"]}

    assert validation["status"] == "blocked"
    assert "PREVIEW_ID_MISSING" in codes
    assert "PREVIEW_PROVENANCE_MISSING" in codes


def test_preview_validation_blocks_invalid_explicit_combinations():
    model = load_preview_model()
    model["combinations"] = [
        {
            "id": "combination:C-BAD",
            "basis": "code",
            "terms": [{"load_case": "load:missing", "factor": float("inf")}],
            "provenance": "invented_example_invalid_combination",
        },
        {
            "id": "combination:C-EMPTY",
            "basis": "mechanics",
            "terms": [],
            "provenance": "invented_example_invalid_combination",
        },
    ]

    validation = validate_preview_model(model)
    codes = {item["code"] for item in validation["diagnostics"]}

    assert validation["status"] == "blocked"
    assert "PREVIEW_COMBINATION_BASIS_UNSUPPORTED" in codes
    assert "PREVIEW_COMBINATION_FACTOR_INVALID" in codes
    assert "PREVIEW_COMBINATION_LOAD_CASE_UNKNOWN" in codes
    assert "PREVIEW_COMBINATION_TERMS_EMPTY" in codes


def test_agent_proposal_is_review_only_and_non_mutating():
    preview = build_agent_proposal_preview()

    assert preview["application_status"] == "not_applied"
    assert preview["accepted_model_state_mutated"] is False
    assert preview["proposal"]["audit_boundary"]["requires_user_acceptance"] is True
    assert preview["proposal"]["audit_boundary"]["mutates_accepted_model_state"] is False
    assert "certified" not in canonical_json(preview).lower()
