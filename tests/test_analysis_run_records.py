#!/usr/bin/env python3
"""Focused tests for DEL-14-02 analysis-run record generation."""

import json
import sys
from copy import deepcopy
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.analysis_runs.records import (
    PHYSICAL_PROJECT_CONTAINER,
    build_preview_analysis_run_envelope,
    canonical_json,
    validate_analysis_run_envelope,
)


PREVIEW_RESULT_PATH = ROOT / "fixtures" / "product_preview" / "invented_mechanics_result.json"
ANALYSIS_RUN_SCHEMA_PATH = ROOT / "schemas" / "analysis_run.schema.json"


def preview_result():
    return json.loads(PREVIEW_RESULT_PATH.read_text(encoding="utf-8"))


def test_preview_result_builds_deterministic_immutable_run_record():
    first = build_preview_analysis_run_envelope(preview_result())
    second = build_preview_analysis_run_envelope(preview_result())

    assert canonical_json(first) == canonical_json(second)
    assert first["deliverable_id"] == "DEL-14-02"
    run = first["analysis_run"]
    assert run["run_id"] == "run:preview-linear-static-001"
    assert run["immutability_policy"]["run_record_is_read_only"] is True
    assert run["immutability_policy"]["new_run_required_for_change"] is True
    assert "HUMAN_REVIEW_REQUIRED" in run["analysis_status"]
    assert "MECHANICS_SOLVED" in run["analysis_status"]
    assert "RULE_INPUTS_INCOMPLETE" in run["analysis_status"]
    assert validate_analysis_run_envelope(first) == []


def test_run_contract_status_uses_sca_003_local_project_store():
    envelope = build_preview_analysis_run_envelope(preview_result())
    contract = envelope["run_contract_status"]
    physical = contract["physical_project_container"]

    assert contract["record_contract"] == "schema_first_analysis_run_records"
    assert physical == PHYSICAL_PROJECT_CONTAINER
    assert physical["profile"] == "sqlite_local_project_store"
    assert physical["decision_ref"] == "SCA-003"
    assert physical["storage_role"] == "local_store_index_projection"
    assert physical["canonical_truth"] == "canonical_json_jcs_payload"
    assert physical["sql_public_contract"] is False
    assert physical["direct_sql_access_allowed"] is False
    assert physical["hosted_db_allowed"] is False
    assert physical["network_required"] is False
    assert physical["sidecars_rebuildable"] is True


def test_analysis_run_schema_binds_physical_container_to_sca_003_profile():
    schema = json.loads(ANALYSIS_RUN_SCHEMA_PATH.read_text(encoding="utf-8"))
    defs = schema["$defs"]
    contract = defs["RunContractStatus"]["properties"]
    physical = defs["PhysicalProjectContainer"]["properties"]

    assert (
        contract["physical_project_container"]["$ref"]
        == "#/$defs/PhysicalProjectContainer"
    )
    assert physical["profile"]["const"] == "sqlite_local_project_store"
    assert physical["decision_ref"]["const"] == "SCA-003"
    assert physical["canonical_truth"]["const"] == "canonical_json_jcs_payload"
    assert physical["sql_public_contract"]["const"] is False
    assert physical["direct_sql_access_allowed"]["const"] is False


def test_result_refs_bind_computed_result_ids_to_hashes():
    envelope = build_preview_analysis_run_envelope(preview_result())
    run = envelope["analysis_run"]
    refs = {item["result_ref"]["ref"]: item for item in run["result_refs"]}

    assert "result:force:pipe-P-120:axial" in refs
    assert "result:force:pipe-P-120:axial:end-j" in refs
    assert "result:force:pipe-P-120:midspan:axial" in refs
    assert "result:force:pipe-P-120:quarter-1:shear-y" in refs
    assert "result:force:pipe-P-120:shear-y" in refs
    assert "result:combination:combination-C-OPER-ALT:force:pipe-P-120:axial" in refs
    assert "result:combination:combination-C-OPER-ALT:force:pipe-P-120:quarter-1:shear-y" in refs
    assert "result:stress:pipe-P-120:end-j:torsional-shear" in refs
    assert "result:stress:pipe-P-120:quarter-1:torsional-shear" in refs
    axial = refs["result:force:pipe-P-120:axial"]
    axial_end_j = refs["result:force:pipe-P-120:axial:end-j"]
    axial_midspan = refs["result:force:pipe-P-120:midspan:axial"]
    shear_quarter = refs["result:force:pipe-P-120:quarter-1:shear-y"]
    combination_axial = refs["result:combination:combination-C-OPER-ALT:force:pipe-P-120:axial"]
    combination_shear_quarter = refs[
        "result:combination:combination-C-OPER-ALT:force:pipe-P-120:quarter-1:shear-y"
    ]
    torsional_stress_end_j = refs["result:stress:pipe-P-120:end-j:torsional-shear"]
    torsional_stress_quarter = refs["result:stress:pipe-P-120:quarter-1:torsional-shear"]
    assert axial["result_ref"]["object_type"] == "Result"
    assert axial["result_family"] == "force"
    assert axial_end_j["result_family"] == "force"
    assert axial_midspan["result_family"] == "force"
    assert shear_quarter["result_family"] == "force"
    assert combination_axial["result_family"] == "force"
    assert combination_shear_quarter["result_family"] == "force"
    assert torsional_stress_end_j["result_family"] == "stress"
    assert torsional_stress_quarter["result_family"] == "stress"
    assert axial["privacy_classification"] == "invented_public_example"
    assert axial_end_j["privacy_classification"] == "invented_public_example"
    assert axial_midspan["privacy_classification"] == "invented_public_example"
    assert shear_quarter["privacy_classification"] == "invented_public_example"
    assert combination_axial["privacy_classification"] == "invented_public_example"
    assert combination_shear_quarter["privacy_classification"] == "invented_public_example"
    assert torsional_stress_end_j["privacy_classification"] == "invented_public_example"
    assert torsional_stress_quarter["privacy_classification"] == "invented_public_example"
    assert axial["hash_refs"][0]["payload_scope"] == "result_value"
    assert axial_end_j["hash_refs"][0]["payload_scope"] == "result_value"
    assert shear_quarter["hash_refs"][0]["payload_scope"] == "result_value"
    assert combination_axial["hash_refs"][0]["payload_scope"] == "result_value"
    assert combination_shear_quarter["hash_refs"][0]["payload_scope"] == "result_value"
    assert torsional_stress_end_j["hash_refs"][0]["payload_scope"] == "result_value"
    assert torsional_stress_quarter["hash_refs"][0]["payload_scope"] == "result_value"
    assert axial["hash_refs"][0]["payload_ref"] == {
        "object_type": "Result",
        "ref": "result:force:pipe-P-120:axial",
    }
    assert axial_end_j["hash_refs"][0]["payload_ref"] == {
        "object_type": "Result",
        "ref": "result:force:pipe-P-120:axial:end-j",
    }
    assert combination_axial["hash_refs"][0]["payload_ref"] == {
        "object_type": "Result",
        "ref": "result:combination:combination-C-OPER-ALT:force:pipe-P-120:axial",
    }
    assert torsional_stress_end_j["hash_refs"][0]["payload_ref"] == {
        "object_type": "Result",
        "ref": "result:stress:pipe-P-120:end-j:torsional-shear",
    }
    assert any(item["payload_scope"] == "result_envelope" for item in run["hashes"])
    assert any(item["payload_scope"] == "analysis_run_record" for item in run["hashes"])


def test_result_mutation_changes_corresponding_result_hash():
    base = preview_result()
    changed = deepcopy(base)
    for item in changed["results"]:
        if item["id"] == "result:force:pipe-P-120:axial":
            item["value"] = item["value"] + 1.0

    base_run = build_preview_analysis_run_envelope(base)["analysis_run"]
    changed_run = build_preview_analysis_run_envelope(changed)["analysis_run"]

    def result_hash(run):
        for item in run["result_refs"]:
            if item["result_ref"]["ref"] == "result:force:pipe-P-120:axial":
                return item["hash_refs"][0]["value"]
        raise AssertionError("axial force ref missing")

    assert result_hash(base_run) != result_hash(changed_run)


def test_validation_blocks_missing_review_boundary_and_result_hashes():
    envelope = build_preview_analysis_run_envelope(preview_result())
    run = envelope["analysis_run"]
    run["analysis_status"] = ["MECHANICS_SOLVED"]
    run["professional_boundary"]["software_makes_compliance_claim"] = True
    run["result_refs"][0]["hash_refs"] = []

    diagnostics = validate_analysis_run_envelope(envelope)
    codes = {item["code"] for item in diagnostics}

    assert "ANALYSIS_RUN_HUMAN_REVIEW_REQUIRED_MISSING" in codes
    assert "ANALYSIS_RUN_PROFESSIONAL_BOUNDARY_VIOLATION" in codes
    assert "ANALYSIS_RUN_RESULT_HASH_MISSING" in codes
