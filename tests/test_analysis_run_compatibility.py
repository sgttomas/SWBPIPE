from __future__ import annotations
from copy import deepcopy
import json
from pathlib import Path

import pytest

from core.analysis_runs.compatibility import (
    analysis_record_projection, build_analysis_run_v0_2, verify_analysis_run_record,
)
from core.serialization.canonical_json.adapter import canonical_sha256_checked_v1
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


def test_v02_checksum_positions_require_exact_scope_profile_reference_and_inventory() -> None:
    valid = build_analysis_run_v0_2(
        mechanics_for_contract(),
        input_manifest_ref=MANIFEST_REF,
        input_manifest_hash=MANIFEST_HASH,
    )
    hashes = valid["analysis_run"]["hashes"]
    record_hash = next(item for item in hashes if item["payload_scope"] == "analysis_run_record")
    received_hash = next(item for item in hashes if item["payload_scope"] == "received_result")
    row_hash = valid["analysis_run"]["result_refs"][0]["hash_refs"][0]
    manifest_hash = valid["analysis_run"]["reproducibility"]["input_manifest_hashes"][0]
    assert (record_hash["canonicalization"], record_hash["payload_ref"]["object_type"]) == (
        "openpipestress_jcs_ijson_v1", "AnalysisRun"
    )
    assert (received_hash["canonicalization"], received_hash["payload_ref"]["object_type"]) == (
        "openpipestress_jcs_ijson_v1", "ResultEnvelope"
    )
    assert (row_hash["payload_scope"], row_hash["canonicalization"], row_hash["payload_ref"]["object_type"]) == (
        "result_row", "openpipestress_jcs_ijson_v1", "Result"
    )
    assert (manifest_hash["payload_scope"], manifest_hash["canonicalization"], manifest_hash["payload_ref"]["object_type"]) == (
        "input_manifest", "rfc8785_jcs", "InputManifest"
    )
    reordered = deepcopy(valid)
    reordered["analysis_run"]["hashes"].reverse()
    validate_instance(dispatcher_schema(), reordered, instance_label="order-independent analysis checksums")

    cases = []
    duplicate_received = deepcopy(valid); duplicate_received["analysis_run"]["hashes"] = [deepcopy(received_hash), deepcopy(received_hash)]; cases.append(duplicate_received)
    duplicate_record = deepcopy(valid); duplicate_record["analysis_run"]["hashes"] = [deepcopy(record_hash), deepcopy(record_hash)]; cases.append(duplicate_record)
    substituted_row = deepcopy(valid); substituted_row["analysis_run"]["result_refs"][0]["hash_refs"] = [deepcopy(manifest_hash)]; cases.append(substituted_row)
    legacy_record_profile = deepcopy(valid); next(item for item in legacy_record_profile["analysis_run"]["hashes"] if item["payload_scope"] == "analysis_run_record")["canonicalization"] = "rfc8785_jcs"; cases.append(legacy_record_profile)
    legacy_received_profile = deepcopy(valid); next(item for item in legacy_received_profile["analysis_run"]["hashes"] if item["payload_scope"] == "received_result")["canonicalization"] = "rfc8785_jcs"; cases.append(legacy_received_profile)
    legacy_row_profile = deepcopy(valid); legacy_row_profile["analysis_run"]["result_refs"][0]["hash_refs"][0]["canonicalization"] = "rfc8785_jcs"; cases.append(legacy_row_profile)
    wrong_record_ref = deepcopy(valid); next(item for item in wrong_record_ref["analysis_run"]["hashes"] if item["payload_scope"] == "analysis_run_record")["payload_ref"]["object_type"] = "ResultEnvelope"; cases.append(wrong_record_ref)
    wrong_received_ref = deepcopy(valid); next(item for item in wrong_received_ref["analysis_run"]["hashes"] if item["payload_scope"] == "received_result")["payload_ref"]["object_type"] = "AnalysisRun"; cases.append(wrong_received_ref)
    wrong_row_ref = deepcopy(valid); wrong_row_ref["analysis_run"]["result_refs"][0]["hash_refs"][0]["payload_ref"]["object_type"] = "InputManifest"; cases.append(wrong_row_ref)
    checked_manifest_profile = deepcopy(valid); checked_manifest_profile["analysis_run"]["reproducibility"]["input_manifest_hashes"][0]["canonicalization"] = "openpipestress_jcs_ijson_v1"; cases.append(checked_manifest_profile)
    wrong_manifest_scope = deepcopy(valid); wrong_manifest_scope["analysis_run"]["reproducibility"]["input_manifest_hashes"][0]["payload_scope"] = "result_row"; cases.append(wrong_manifest_scope)
    wrong_manifest_ref = deepcopy(valid); wrong_manifest_ref["analysis_run"]["reproducibility"]["input_manifest_hashes"][0]["payload_ref"]["object_type"] = "Result"; cases.append(wrong_manifest_ref)
    for case in cases:
        assert_schema_rejects(case)

    for fixture_name in ("legacy_python_v0_1.json", "legacy_desktop_v0_1.json"):
        legacy = json.loads((PROJECT / "fixtures/analysis_runs/invented" / fixture_name).read_text())
        validate_instance(dispatcher_schema(), legacy, instance_label=f"preserved {fixture_name}")


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
    ref = {"object_type": "Result", "ref": "result:test"}
    claim = {"algorithm": "sha256", "value": hashlib.sha256(python_bytes).hexdigest(), "canonicalization": "SORTED_COMPACT_JSON", "payload_scope": "result_value", "payload_ref": ref}
    args = {"expected_payload_scope": "result_value", "expected_payload_ref": ref, "preimage_provenance_established": True}
    assert verify_legacy_checksum(claim, profile=PYTHON_PROFILE, preimage=preimage, **args)["status"] == "match"
    assert verify_legacy_checksum(claim, profile=PYTHON_PROFILE, **args)["status"] == "unverifiable"
    assert verify_legacy_checksum(claim, profile=PYTHON_PROFILE, preimage=preimage, numeric_types_preserved=False, **args)["status"] == "unverifiable"
    tampered = {**claim, "value": "0" * 64}
    assert verify_legacy_checksum(tampered, profile=PYTHON_PROFILE, preimage=preimage, **args)["status"] == "mismatch"
    desktop_bytes = b'{"a":"caf\xc3\xa9","z":1}'
    desktop_claim = {**claim, "value": hashlib.sha256(desktop_bytes).hexdigest(), "canonicalization": "rfc8785_jcs"}
    checked = verify_legacy_checksum(desktop_claim, profile=DESKTOP_PROFILE, serialized_preimage=desktop_bytes, **args)
    assert checked["status"] == "match" and checked["received_claim"] == desktop_claim and checked["claim_rewritten"] is False
    for invalid in [
        {**claim, "algorithm": "sha512"},
        {**claim, "canonicalization": "rfc8785_jcs"},
        {**claim, "payload_scope": "analysis_run_record"},
        {**claim, "payload_ref": {"object_type": "Result", "ref": "result:other"}},
    ]:
        assert verify_legacy_checksum(invalid, profile=PYTHON_PROFILE, preimage=preimage, **args)["status"] == "unverifiable"
    assert verify_legacy_checksum(claim, profile=PYTHON_PROFILE, preimage=preimage, expected_payload_scope="result_value", expected_payload_ref=ref, preimage_provenance_established=False)["status"] == "unverifiable"


@pytest.mark.parametrize("version", ["99.0.0", "0.3.0", "0.2", "0.2.0 ", "", None, False, 0, 0.2, [], {}])
def test_explicit_legacy_constructor_rejects_unknown_or_nonstring_source_version(version) -> None:
    source = {"schema_version": version, "results": [], "diagnostics": []}
    before = deepcopy(source)
    with pytest.raises(ValueError, match="^ANALYSIS_LEGACY_SOURCE_VERSION_UNSUPPORTED$"):
        build_analysis_run_v0_2(source, input_manifest_ref=MANIFEST_REF, input_manifest_hash=MANIFEST_HASH)
    assert source == before


def test_explicit_legacy_constructor_rejects_missing_source_version() -> None:
    source = {"results": [], "diagnostics": []}
    before = deepcopy(source)
    with pytest.raises(ValueError, match="^ANALYSIS_LEGACY_SOURCE_VERSION_UNSUPPORTED$"):
        build_analysis_run_v0_2(source, input_manifest_ref=MANIFEST_REF, input_manifest_hash=MANIFEST_HASH)
    assert source == before


@pytest.mark.parametrize("version", ["0.1.0", "0.2.0"])
@pytest.mark.parametrize("header", ["producer", "numerical_quality", "formulation_basis"])
@pytest.mark.parametrize("value", [None, False, {}, []])
def test_explicit_legacy_constructor_rejects_header_presence_even_when_empty(version, header, value) -> None:
    source = {"schema_version": version, header: value, "results": [], "diagnostics": []}
    before = deepcopy(source)
    with pytest.raises(ValueError, match="^ANALYSIS_LEGACY_SOURCE_DOWNGRADE_FORBIDDEN$"):
        build_analysis_run_v0_2(source, input_manifest_ref=MANIFEST_REF, input_manifest_hash=MANIFEST_HASH)
    assert source == before


@pytest.mark.parametrize("mode", ["sparse", "dense"])
def test_explicit_legacy_constructor_refuses_actual_current_precision_fixture(mode: str) -> None:
    fixture = PROJECT / f"fixtures/product_preview/invented_mechanics_result_precision_1_{mode}.json"
    original_bytes = fixture.read_bytes()
    source = json.loads(original_bytes)
    before = deepcopy(source)
    assert source["schema_version"] == "0.2.0"
    assert source["producer"]["semantic_contract_id"] == "openpipestress.result_semantics/0.3.0/precision-1"
    with pytest.raises(ValueError, match="^ANALYSIS_LEGACY_SOURCE_DOWNGRADE_FORBIDDEN$"):
        build_analysis_run_v0_2(source, input_manifest_ref=MANIFEST_REF, input_manifest_hash=MANIFEST_HASH)
    assert source == before
    assert fixture.read_bytes() == original_bytes


def test_explicit_legacy_constructor_known_versions_preserve_real_hashes_and_semantics() -> None:
    # This is the existing language-neutral one-row historical record basis.
    # Both versions remain historical; no injected hash function or fake digest.
    source = {
        "schema_version": "0.2.0", "document_kind": "MechanicsResult", "run_id": "run:test", "model_ref": "model:test",
        "status": {"mechanics": "MECHANICS_SOLVED", "rule_check": "RULE_INPUTS_INCOMPLETE", "professional_acceptance": "NOT_PROVIDED"},
        "summary": {}, "results": [{"id": "result:test", "entity_ref": "node:test", "kind": "displacement_magnitude", "value": 1, "unit": "mm", "basis_ref": {"ref_type": "load_case", "ref_id": "load:test"}}], "diagnostics": [],
    }
    manifest = {"model_basis": {"model_ref": "model:test"}, "solver_basis": {"solver_name": "solver", "solver_version": "0.1.0", "solver_build_ref": "build:test", "settings": {"mode": "fixture"}}, "unit_basis": {"project_units": {"length": "mm"}}, "load_basis": {"load_cases": [], "combinations": []}}
    records = {}
    for version in ("0.1.0", "0.2.0"):
        received = deepcopy(source)
        received["schema_version"] = version
        source_bytes = json.dumps(received, ensure_ascii=False, separators=(",", ":")).encode("utf-8")
        record = build_analysis_run_v0_2(received, input_manifest_ref={"object_type": "InputManifest", "ref": "input-manifest:test"}, input_manifest_hash="1" * 64, input_manifest=manifest)
        assert json.dumps(received, ensure_ascii=False, separators=(",", ":")).encode("utf-8") == source_bytes
        assert record["schema_version"] == "0.2.0"
        assert hash_by_scope(record, "received_result") == canonical_sha256_checked_v1(received)
        assert hash_by_scope(record, "analysis_run_record") == canonical_sha256_checked_v1(analysis_record_projection(record))
        row = record["analysis_run"]["result_refs"][0]
        assert row["hash_refs"][0]["value"] == canonical_sha256_checked_v1(received["results"][0])
        assert verify_analysis_run_record(record) == "match"
        records[version] = record
    frozen = json.loads((PROJECT / "fixtures/analysis_runs/invented/analysis_run_v0_2.json").read_text())
    assert records["0.2.0"] == frozen
    assert records["0.1.0"]["analysis_run"]["result_refs"] == frozen["analysis_run"]["result_refs"]
    assert records["0.1.0"]["analysis_run"]["reproducibility"]["semantic_contract"] == frozen["analysis_run"]["reproducibility"]["semantic_contract"]
    # The source version is part of the received checksum, not erased or relabelled.
    assert hash_by_scope(records["0.1.0"], "received_result") != hash_by_scope(records["0.2.0"], "received_result")
    assert hash_by_scope(records["0.1.0"], "analysis_run_record") != hash_by_scope(records["0.2.0"], "analysis_run_record")
