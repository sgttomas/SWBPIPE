#!/usr/bin/env python3
"""Focused tests for DEL-14-02 analysis-run record generation."""

import json
import sys
from copy import deepcopy
from hashlib import sha256
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
TESTS_DIR = Path(__file__).resolve().parent
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))
if str(TESTS_DIR) not in sys.path:
    sys.path.insert(0, str(TESTS_DIR))

from schema_validation import (  # noqa: E402
    JsonSchemaDependencyMissing,
    validate_instance,
    validate_schema_document,
)

from core.analysis_runs.records import (
    PHYSICAL_PROJECT_CONTAINER,
    build_preview_analysis_run_envelope,
    canonical_json,
    validate_analysis_run_envelope,
)
from core.project_persistence import (  # noqa: E402
    build_project_persistence_envelope,
    round_trip_project_envelope,
    validate_project_persistence_envelope,
)


PREVIEW_RESULT_PATH = ROOT / "fixtures" / "product_preview" / "invented_mechanics_result.json"
ANALYSIS_RUN_SCHEMA_PATH = ROOT / "schemas" / "analysis_run.schema.json"


def preview_result():
    return json.loads(PREVIEW_RESULT_PATH.read_text(encoding="utf-8"))


def analysis_run_schema():
    return json.loads(ANALYSIS_RUN_SCHEMA_PATH.read_text(encoding="utf-8"))


def manifest_evidence():
    payload = {
        "schema_version": "1.0.0",
        "document_kind": "openpipestress.current_session_input_manifest",
        "model_ref": "project:invented-loop-01",
        "solver_mode": "sparse_interactive",
        "test_basis": "invented-del-14-02-focused-evidence",
    }
    digest = sha256(canonical_json(payload).encode("utf-8")).hexdigest()
    return (
        {
            "object_type": "InputManifest",
            "ref": f"input-manifest:project-invented-loop-01:{digest}",
        },
        digest,
    )


def build_run(result=None, **kwargs):
    manifest_ref, manifest_hash = manifest_evidence()
    return build_preview_analysis_run_envelope(
        result or preview_result(),
        input_manifest_ref=manifest_ref,
        input_manifest_hash=manifest_hash,
        **kwargs,
    )


def _skip_or_note_missing_jsonschema(exc):
    if "pytest" in sys.modules:
        import pytest

        pytest.skip(str(exc))
    print(f"SKIP: {exc}")


def test_preview_result_builds_deterministic_immutable_run_record():
    first = build_run()
    second = build_run()

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


def test_sorted_compact_json_has_exact_bytes_and_stable_hash_without_jcs_claim():
    first = {"z": "café", "a": [1, {"b": True}]}
    equivalent = {"a": [1, {"b": True}], "z": "café"}
    expected = b'{"a":[1,{"b":true}],"z":"caf\\u00e9"}'

    first_bytes = canonical_json(first).encode("utf-8")
    equivalent_bytes = canonical_json(equivalent).encode("utf-8")

    assert first_bytes == expected
    assert equivalent_bytes == expected
    assert sha256(first_bytes).hexdigest() == (
        "9fe3dcafeafda5780fb9062a482c657fdd139cb98c1d4e4356e7123b64665753"
    )

    envelope = build_run()
    checksums = envelope["analysis_run"]["hashes"] + [
        checksum
        for result_ref in envelope["analysis_run"]["result_refs"]
        for checksum in result_ref["hash_refs"]
    ]
    assert {item["canonicalization"] for item in checksums} == {
        "SORTED_COMPACT_JSON"
    }
    assert "JCS" not in canonical_json(checksums)
    assert envelope["analysis_run"]["reproducibility"]["input_manifest_hashes"][
        0
    ]["canonicalization"] == "JCS"


def test_generated_analysis_run_envelope_validates_against_schema():
    envelope = build_run()
    schema = analysis_run_schema()

    try:
        assert validate_schema_document(schema, schema_label=str(ANALYSIS_RUN_SCHEMA_PATH))
        assert validate_instance(
            schema,
            envelope,
            schema_label=str(ANALYSIS_RUN_SCHEMA_PATH),
            instance_label="generated DEL-14-02 analysis run envelope",
        )
    except JsonSchemaDependencyMissing as exc:
        _skip_or_note_missing_jsonschema(exc)


def test_generated_run_binds_run_basis_diagnostics_and_boundary_fields():
    state_ref = {"object_type": "ModelState", "ref": "state:accepted-invented-preview"}
    settings_ref = {"object_type": "SolverSettings", "ref": "settings:linear-static-review"}
    unit_ref = {"object_type": "UnitSystem", "ref": "unit-system:invented-si"}
    load_refs = [
        {"object_type": "LoadCase", "ref": "load:L-100"},
        {"object_type": "LoadCombination", "ref": "combination:C-OPER-ALT"},
    ]
    build_ref = {"object_type": "ExternalReference", "ref": "build:del-14-02-evidence"}

    envelope = build_run(
        model_state_ref=state_ref,
        settings_ref=settings_ref,
        unit_system_ref=unit_ref,
        load_basis_refs=load_refs,
        solver_name="open_pipe_stress_preview_solver",
        solver_version="0.1.0-test",
        build_ref=build_ref,
    )
    run = envelope["analysis_run"]

    assert run["model_state_ref"] == state_ref
    assert run["settings_ref"] == settings_ref
    assert run["unit_system_ref"] == unit_ref
    assert run["load_basis_refs"] == load_refs
    assert run["solver_version"]["solver_name"] == "open_pipe_stress_preview_solver"
    assert run["solver_version"]["solver_version"] == "0.1.0-test"
    assert run["solver_version"]["build_ref"] == build_ref
    assert run["diagnostics"]
    assert all(item["code"] and item["provenance"] for item in run["diagnostics"])
    assert run["result_refs"]
    assert all(item["hash_refs"] for item in run["result_refs"])
    assert {item["payload_scope"] for item in run["hashes"]} >= {
        "analysis_run_record",
        "result_envelope",
    }
    assert run["professional_boundary"] == {
        "human_review_required": True,
        "software_makes_compliance_claim": False,
        "software_makes_certification_claim": False,
        "software_makes_sealing_claim": False,
        "software_makes_approval_claim": False,
        "software_makes_authentication_claim": False,
    }


def test_run_contract_status_uses_sca_003_local_project_store():
    envelope = build_run()
    contract = envelope["run_contract_status"]
    physical = contract["physical_project_container"]

    assert contract["record_contract"] == "schema_first_analysis_run_records"
    assert physical == PHYSICAL_PROJECT_CONTAINER
    assert physical["profile"] == "sqlite_local_project_store"
    assert physical["decision_ref"] == "SCA-003"
    assert physical["storage_role"] == "local_store_index_projection"
    assert physical["canonical_truth"] == "sorted_compact_json_payload"
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
    assert "sorted_compact_json_payload" in physical["canonical_truth"]["enum"]
    assert "backward compatibility" in physical["canonical_truth"]["description"]
    assert physical["sql_public_contract"]["const"] is False
    assert physical["direct_sql_access_allowed"]["const"] is False


def test_result_refs_bind_computed_result_ids_to_hashes():
    envelope = build_run()
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


def test_run_binds_distinct_exact_input_manifest_and_explicit_source_dimensions():
    envelope = build_run()
    run = envelope["analysis_run"]
    manifest_ref, manifest_hash = manifest_evidence()
    result_envelope_hash = next(
        item["value"]
        for item in run["hashes"]
        if item["payload_scope"] == "result_envelope"
    )
    dimensions = {
        item["result_ref"]["ref"]: item["source_dimension"]
        for item in run["result_refs"]
    }

    assert run["reproducibility"]["input_manifest_refs"] == [manifest_ref]
    assert run["reproducibility"]["input_manifest_hashes"] == [
        {
            "algorithm": "sha256",
            "canonicalization": "JCS",
            "payload_ref": manifest_ref,
            "payload_scope": "input_manifest",
            "value": manifest_hash,
        }
    ]
    assert manifest_hash != result_envelope_hash
    assert dimensions["result:component-stiffness:component-C-150:axial"] == (
        "linear_stiffness"
    )
    assert dimensions["result:component-stiffness:component-C-150:lateral"] == (
        "linear_stiffness"
    )
    assert dimensions["result:component-stiffness:component-C-150:angular"] == (
        "rotational_stiffness"
    )
    assert dimensions["result:component-stiffness:component-C-150:torsional"] == (
        "rotational_stiffness"
    )


def test_result_family_uses_exact_kind_and_dimension_not_deceptive_unit_text():
    deceptive = preview_result()
    target = next(
        item
        for item in deceptive["results"]
        if item["kind"] == "element_local_axial_force"
    )
    target["unit"] = "MPa"
    run = build_run(deceptive)["analysis_run"]
    bound = next(
        item
        for item in run["result_refs"]
        if item["result_ref"]["ref"] == target["id"]
    )
    assert bound["source_dimension"] == "force"
    assert bound["result_family"] == "force"


def test_explicit_result_dimension_mismatch_blocks_instead_of_inferring():
    mismatched = preview_result()
    target = next(
        item
        for item in mismatched["results"]
        if item["kind"] == "element_local_axial_force"
    )
    target["dimension"] = "stress"
    try:
        build_run(mismatched)
    except ValueError as exc:
        assert "ANALYSIS_RUN_RESULT_DIMENSION_MISMATCH" in str(exc)
    else:
        raise AssertionError("contradictory explicit result dimension was accepted")


def test_missing_or_invalid_manifest_evidence_blocks_without_result_substitution():
    missing = build_preview_analysis_run_envelope(preview_result())
    run = missing["analysis_run"]
    assert run["reproducibility"]["input_manifest_refs"] == []
    assert run["reproducibility"]["input_manifest_hashes"] == []
    assert {
        item["code"] for item in validate_analysis_run_envelope(missing)
    } >= {"ANALYSIS_RUN_INPUT_MANIFEST_MISSING"}

    manifest_ref, manifest_hash = manifest_evidence()
    invalid_hash = manifest_hash.upper()
    try:
        build_preview_analysis_run_envelope(
            preview_result(),
            input_manifest_ref=manifest_ref,
            input_manifest_hash=invalid_hash,
        )
    except ValueError as exc:
        assert "ANALYSIS_RUN_INPUT_MANIFEST_INVALID" in str(exc)
    else:
        raise AssertionError("uppercase manifest SHA-256 was accepted")

    wrong_prefix = {
        "object_type": "InputManifest",
        "ref": f"result-envelope:project-invented-loop-01:{manifest_hash}",
    }
    wrong_model = {
        "object_type": "InputManifest",
        "ref": f"input-manifest:project-different:{manifest_hash}",
    }
    for invalid_ref in (wrong_prefix, wrong_model):
        try:
            build_preview_analysis_run_envelope(
                preview_result(),
                input_manifest_ref=invalid_ref,
                input_manifest_hash=manifest_hash,
            )
        except ValueError as exc:
            assert "ANALYSIS_RUN_INPUT_MANIFEST_INVALID" in str(exc)
        else:
            raise AssertionError(
                "wrong-prefix/model manifest ref with a valid digest was accepted"
            )


def test_result_mutation_changes_corresponding_result_hash():
    base = preview_result()
    changed = deepcopy(base)
    for item in changed["results"]:
        if item["id"] == "result:force:pipe-P-120:axial":
            item["value"] = item["value"] + 1.0

    base_run = build_run(base)["analysis_run"]
    changed_run = build_run(changed)["analysis_run"]

    def result_hash(run):
        for item in run["result_refs"]:
            if item["result_ref"]["ref"] == "result:force:pipe-P-120:axial":
                return item["hash_refs"][0]["value"]
        raise AssertionError("axial force ref missing")

    assert result_hash(base_run) != result_hash(changed_run)


def test_persistence_history_preserves_analysis_run_basis_after_model_change():
    analysis_run = build_run(
        model_state_ref={"object_type": "ModelState", "ref": "state:original-accepted"},
        settings_ref={"object_type": "SolverSettings", "ref": "settings:original-solve"},
        unit_system_ref={"object_type": "UnitSystem", "ref": "unit-system:original-si"},
        load_basis_refs=[{"object_type": "LoadCase", "ref": "load:L-100"}],
    )
    result_envelope_ref = next(
        item["payload_ref"]
        for item in analysis_run["analysis_run"]["hashes"]
        if item["payload_scope"] == "result_envelope"
    )

    def persisted_project(model_revision):
        return build_project_persistence_envelope(
            project_id="project:del-14-02-run-history",
            project_name="DEL-14-02 invented run-history check",
            model_payload={
                "model_id": "model:invented-preview",
                "revision": model_revision,
            },
            model_state_refs=[{"ref_kind": "model_state", "ref": "state:original-accepted"}],
            analysis_run_records=[analysis_run],
            result_envelope_refs=[
                {"ref_kind": "result_envelope", "ref": result_envelope_ref["ref"]}
            ],
        )

    original = persisted_project("original")
    changed_model = persisted_project("later-unrelated-model-edit")
    round_trip = round_trip_project_envelope(original)

    original_history = original["project"]["run_history"]
    changed_history = changed_model["project"]["run_history"]
    restored_history = round_trip["envelope"]["project"]["run_history"]

    assert validate_project_persistence_envelope(original) == []
    assert round_trip["semantic_equal"] is True
    assert round_trip["diagnostics"] == []
    assert original["hash"]["project_payload_hash"]["value"] != changed_model["hash"][
        "project_payload_hash"
    ]["value"]
    assert original_history["analysis_run_records"] == changed_history["analysis_run_records"]
    assert original_history["hash_manifest"] == changed_history["hash_manifest"]
    assert restored_history["analysis_run_records"] == original_history[
        "analysis_run_records"
    ]
    persisted_run = restored_history["analysis_run_records"][0]["analysis_run"]
    assert persisted_run["model_state_ref"] == {
        "object_type": "ModelState",
        "ref": "state:original-accepted",
    }
    assert persisted_run["settings_ref"]["ref"] == "settings:original-solve"
    assert persisted_run["unit_system_ref"]["ref"] == "unit-system:original-si"
    assert persisted_run["load_basis_refs"] == [
        {"object_type": "LoadCase", "ref": "load:L-100"}
    ]
    assert any(
        item["payload_scope"] == "analysis_run_record"
        and item["payload_ref"]["ref"] == persisted_run["run_id"]
        for item in restored_history["hash_manifest"]
    )


def test_validation_blocks_missing_review_boundary_and_result_hashes():
    envelope = build_run()
    run = envelope["analysis_run"]
    run["analysis_status"] = ["MECHANICS_SOLVED"]
    run["professional_boundary"]["software_makes_compliance_claim"] = True
    run["result_refs"][0]["hash_refs"] = []

    diagnostics = validate_analysis_run_envelope(envelope)
    codes = {item["code"] for item in diagnostics}

    assert "ANALYSIS_RUN_HUMAN_REVIEW_REQUIRED_MISSING" in codes
    assert "ANALYSIS_RUN_PROFESSIONAL_BOUNDARY_VIOLATION" in codes
    assert "ANALYSIS_RUN_RESULT_HASH_MISSING" in codes
