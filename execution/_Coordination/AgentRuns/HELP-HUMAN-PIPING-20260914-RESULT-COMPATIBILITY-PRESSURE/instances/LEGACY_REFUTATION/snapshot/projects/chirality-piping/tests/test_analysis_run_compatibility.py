from __future__ import annotations
from copy import deepcopy
import json
from pathlib import Path

from core.analysis_runs.compatibility import build_analysis_run_v0_2, verify_analysis_run_record
from core.analysis_runs.records import build_preview_analysis_run_envelope
from core.analysis_runs.legacy import DESKTOP_PROFILE, PYTHON_PROFILE, verify_legacy_checksum
import hashlib
from schema_validation import validate_instance

PROJECT = Path(__file__).resolve().parents[1]
CONTRACT = json.loads((PROJECT / "fixtures/results/semantic_contract_v0_2.json").read_text())
MANIFEST_REF = {"object_type": "InputManifest", "ref": "input-manifest:model:test:fixture"}
MANIFEST_HASH = "1" * 64


def mechanics_for_contract() -> dict:
    rows = []
    for index, semantic in enumerate(CONTRACT["rows"]):
        metadata = {}
        if semantic["component"] is not None:
            metadata["component"] = semantic["component"]
        rows.append({"id": f"result:{index}", "kind": semantic["kind"], "unit": semantic["unit"], "value": float(index), "metadata": metadata})
    return {"schema_version": "0.2.0", "run_id": "run:test", "model_ref": "model:test", "status": {"mechanics": "MECHANICS_SOLVED", "rule_check": "RULE_INPUTS_INCOMPLETE"}, "results": rows, "diagnostics": []}


def hash_by_scope(record: dict, scope: str) -> str:
    return next(row["value"] for row in record["analysis_run"]["hashes"] if row["payload_scope"] == scope)


def test_all_60_signatures_are_accounted_without_mutating_source() -> None:
    source = mechanics_for_contract()
    before = deepcopy(source)
    record = build_analysis_run_v0_2(source, input_manifest_ref=MANIFEST_REF, input_manifest_hash=MANIFEST_HASH)
    assert source == before
    assert record["schema_version"] == "0.2.0"
    assert len(record["analysis_run"]["result_refs"]) == 60
    assert len({row["semantic_contract"]["signature_id"] for row in record["analysis_run"]["result_refs"]}) == 60
    assert all(row["hash_refs"][0]["payload_scope"] == "result_row" for row in record["analysis_run"]["result_refs"])
    assert verify_analysis_run_record(record) == "match"
    schema = json.loads((PROJECT / "schemas/analysis_run.schema.json").read_text())
    validate_instance(schema, record, instance_label="analysis record 0.2")


def test_real_status_fixture_matches_language_neutral_v02_record() -> None:
    result = {
        "schema_version": "0.2.0", "document_kind": "MechanicsResult", "run_id": "run:test", "model_ref": "model:test",
        "status": {"mechanics": "MECHANICS_SOLVED", "rule_check": "RULE_INPUTS_INCOMPLETE", "professional_acceptance": "NOT_PROVIDED"},
        "summary": {}, "results": [{"id": "result:test", "entity_ref": "node:test", "kind": "displacement_magnitude", "value": 1, "unit": "mm", "basis_ref": {"ref_type": "load_case", "ref_id": "load:test"}}], "diagnostics": [],
    }
    manifest = {"model_basis": {"model_ref": "model:test"}, "solver_basis": {"solver_name": "solver", "solver_version": "0.1.0", "solver_build_ref": "build:test", "settings": {"mode": "fixture"}}, "unit_basis": {"project_units": {"length": "mm"}}, "load_basis": {"load_cases": [], "combinations": []}}
    built = build_analysis_run_v0_2(result, input_manifest_ref={"object_type": "InputManifest", "ref": "input-manifest:test"}, input_manifest_hash="1" * 64, input_manifest=manifest)
    fixture = json.loads((PROJECT / "fixtures/analysis_runs/invented/analysis_run_v0_2.json").read_text())
    assert built == fixture
    assert "NOT_PROVIDED" not in built["analysis_run"]["analysis_status"]
    validate_instance(dispatcher_schema(), fixture, instance_label="language-neutral analysis record fixture")


def test_rule_revision_changes_only_full_record_identity() -> None:
    source = mechanics_for_contract()
    first = build_analysis_run_v0_2(source, input_manifest_ref=MANIFEST_REF, input_manifest_hash=MANIFEST_HASH)
    revised = build_analysis_run_v0_2(source, input_manifest_ref=MANIFEST_REF, input_manifest_hash=MANIFEST_HASH, rule_check_status="USER_RULE_CHECKED")
    assert hash_by_scope(first, "received_result") == hash_by_scope(revised, "received_result")
    assert hash_by_scope(first, "analysis_run_record") != hash_by_scope(revised, "analysis_run_record")


def test_tamper_and_legacy_profile_are_not_false_matches() -> None:
    record = build_analysis_run_v0_2(mechanics_for_contract(), input_manifest_ref=MANIFEST_REF, input_manifest_hash=MANIFEST_HASH)
    record["analysis_run"]["analysis_status"].append("USER_RULE_FAILED")
    assert verify_analysis_run_record(record) == "mismatch"
    assert verify_analysis_run_record({"schema_version": "0.1.0"}) == "unverifiable"


def test_known_contract_contradictions_block() -> None:
    source = mechanics_for_contract()
    source["results"][0]["unit"] = "Pa"
    try:
        build_analysis_run_v0_2(source, input_manifest_ref=MANIFEST_REF, input_manifest_hash=MANIFEST_HASH)
    except ValueError as error:
        assert "SOURCE_UNIT_CONTRADICTION" in str(error)
    else:
        raise AssertionError("contradiction must block")


def dispatcher_schema() -> dict:
    return json.loads((PROJECT / "schemas/analysis_run.schema.json").read_text())


def assert_schema_rejects(value: dict) -> None:
    try:
        validate_instance(dispatcher_schema(), value, instance_label="negative analysis record")
    except AssertionError:
        return
    raise AssertionError("schema should reject negative instance")


def test_v02_schema_rejects_unknown_version_fields_status_missing_evidence_and_boundary_claims() -> None:
    valid = build_analysis_run_v0_2(mechanics_for_contract(), input_manifest_ref=MANIFEST_REF, input_manifest_hash=MANIFEST_HASH)
    cases = []
    unknown_version = deepcopy(valid); unknown_version["schema_version"] = "0.3.0"; cases.append(unknown_version)
    unknown_field = deepcopy(valid); unknown_field["analysis_run"]["invented"] = True; cases.append(unknown_field)
    forbidden_status = deepcopy(valid); forbidden_status["analysis_run"]["analysis_status"].append("CODE_COMPLIANT"); cases.append(forbidden_status)
    missing_solver = deepcopy(valid); del missing_solver["analysis_run"]["solver_version"]; cases.append(missing_solver)
    missing_row_hash = deepcopy(valid); missing_row_hash["analysis_run"]["result_refs"][0]["hash_refs"] = []; cases.append(missing_row_hash)
    missing_record_hash = deepcopy(valid); missing_record_hash["analysis_run"]["hashes"] = [item for item in missing_record_hash["analysis_run"]["hashes"] if item["payload_scope"] != "analysis_run_record"]; cases.append(missing_record_hash)
    missing_received_hash = deepcopy(valid); missing_received_hash["analysis_run"]["hashes"] = [item for item in missing_received_hash["analysis_run"]["hashes"] if item["payload_scope"] != "received_result"]; cases.append(missing_received_hash)
    missing_result_ref = deepcopy(valid); del missing_result_ref["analysis_run"]["result_refs"][0]["result_ref"]; cases.append(missing_result_ref)
    privacy_violation = deepcopy(valid); privacy_violation["analysis_run"]["result_refs"][0]["privacy_classification"] = "private_unredacted"; cases.append(privacy_violation)
    mutable_revision = deepcopy(valid); mutable_revision["analysis_run"]["immutability_policy"]["new_mechanics_run_required_for_record_revision"] = True; cases.append(mutable_revision)
    boundary_claim = deepcopy(valid); boundary_claim["analysis_run"]["professional_boundary"]["software_makes_approval_claim"] = True; cases.append(boundary_claim)
    provenance_claim = deepcopy(valid); provenance_claim["analysis_run"]["provenance"]["professional_claim"] = True; cases.append(provenance_claim)
    for case in cases:
        assert_schema_rejects(case)


def test_legacy_schema_rejects_unknown_version_fields_and_authority_status() -> None:
    source = {"run_id": "run:legacy", "model_ref": "model:legacy", "status": {"mechanics": "MECHANICS_SOLVED", "rule_check": "RULE_INPUTS_INCOMPLETE"}, "results": [{"id": "result:legacy", "kind": "displacement_magnitude", "unit": "mm", "value": 1.0}], "diagnostics": []}
    legacy = build_preview_analysis_run_envelope(source, input_manifest_ref={"object_type": "InputManifest", "ref": f"input-manifest:model-legacy:{MANIFEST_HASH}"}, input_manifest_hash=MANIFEST_HASH)
    validate_instance(dispatcher_schema(), legacy, instance_label="valid strict legacy record")
    unknown = deepcopy(legacy); unknown["schema_version"] = "0.9.0"; assert_schema_rejects(unknown)
    extra = deepcopy(legacy); extra["invented"] = True; assert_schema_rejects(extra)
    authority = deepcopy(legacy); authority["analysis_run"]["analysis_status"].append("APPROVED"); assert_schema_rejects(authority)


def test_frozen_legacy_profiles_dispatch_and_desktop_nested_contract_is_strict() -> None:
    python_record = json.loads((PROJECT / "fixtures/analysis_runs/invented/legacy_python_v0_1.json").read_text())
    desktop_record = json.loads((PROJECT / "fixtures/analysis_runs/invented/legacy_desktop_v0_1.json").read_text())
    validate_instance(dispatcher_schema(), python_record, instance_label="frozen Python 0.1 record")
    validate_instance(dispatcher_schema(), desktop_record, instance_label="frozen desktop 0.1 record")
    cases = []
    item = deepcopy(desktop_record); item["analysis_run"]["hashes"][0]["canonicalization"] = "SORTED_COMPACT_JSON"; cases.append(item)
    item = deepcopy(desktop_record); item["analysis_run"]["result_refs"][0]["hash_refs"] = []; cases.append(item)
    item = deepcopy(desktop_record); item["analysis_run"]["professional_boundary"]["software_makes_approval_claim"] = True; cases.append(item)
    item = deepcopy(desktop_record); item["analysis_run"]["immutability_policy"]["run_record_is_read_only"] = False; cases.append(item)
    item = deepcopy(desktop_record); item["analysis_run"]["analysis_status"].append("TBD"); cases.append(item)
    item = deepcopy(desktop_record); item["run_contract_status"]["physical_project_container"] = {}; cases.append(item)
    for case in cases:
        assert_schema_rejects(case)


def test_legacy_hash_profiles_require_exact_preimage_evidence() -> None:
    preimage = {"z": 1.0, "a": "café"}
    python_bytes = json.dumps(preimage, sort_keys=True, separators=(",", ":"), ensure_ascii=True).encode()
    claim = {"value": hashlib.sha256(python_bytes).hexdigest(), "canonicalization": "SORTED_COMPACT_JSON"}
    assert verify_legacy_checksum(claim, profile=PYTHON_PROFILE, preimage=preimage)["status"] == "match"
    assert verify_legacy_checksum(claim, profile=PYTHON_PROFILE)["status"] == "unverifiable"
    assert verify_legacy_checksum(claim, profile=PYTHON_PROFILE, preimage=preimage, numeric_types_preserved=False)["status"] == "unverifiable"
    tampered = {**claim, "value": "0" * 64}
    assert verify_legacy_checksum(tampered, profile=PYTHON_PROFILE, preimage=preimage)["status"] == "mismatch"
    desktop_bytes = b'{"a":"caf\xc3\xa9","z":1}'
    desktop_claim = {"value": hashlib.sha256(desktop_bytes).hexdigest(), "canonicalization": "rfc8785_jcs"}
    checked = verify_legacy_checksum(desktop_claim, profile=DESKTOP_PROFILE, serialized_preimage=desktop_bytes)
    assert checked["status"] == "match" and checked["received_claim"] == desktop_claim and checked["claim_rewritten"] is False
