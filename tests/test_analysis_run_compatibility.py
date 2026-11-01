"""DEL-14-02 analysis-run records: one current version, 0.3.0 (b7-plan E1).

The record builder records only a result a current record contract reads; a saved
0.1.0 or 0.2.0 record is refused by name, and no older record is written."""
from __future__ import annotations
from copy import deepcopy
import json
from pathlib import Path

import pytest

from core.analysis_runs.compatibility import (
    ANALYSIS_RUN_SCHEMA_VERSION_RETIRED, analysis_record_projection, build_analysis_run, build_analysis_run_v0_3,
    validate_analysis_run_v0_3, verify_analysis_run_record,
)
from core.serialization.canonical_json.adapter import canonical_sha256_checked_v1
from schema_validation import validate_instance

PROJECT = Path(__file__).resolve().parents[1]
MANIFEST_REF = {"object_type": "InputManifest", "ref": "input-manifest:model:test:fixture"}
MANIFEST_HASH = "1" * 64
CURRENT_DEMO_RESULT = PROJECT / "fixtures/product_preview/invented_demo_result_preview_physics_1_sparse.json"


def hash_by_scope(record: dict, scope: str) -> str:
    return next(row["value"] for row in record["analysis_run"]["hashes"] if row["payload_scope"] == scope)


def dispatcher_schema() -> dict:
    return json.loads((PROJECT / "schemas/analysis_run.schema.json").read_text())


def assert_schema_rejects(value: dict) -> None:
    try:
        validate_instance(dispatcher_schema(), value, instance_label="negative analysis record")
    except AssertionError:
        return
    raise AssertionError("schema should reject negative instance")


@pytest.mark.parametrize("version", ["0.1.0", "0.2.0"])
def test_retired_record_versions_are_refused_by_name(version) -> None:
    with pytest.raises(ValueError, match=f"^{ANALYSIS_RUN_SCHEMA_VERSION_RETIRED}: analysis-run {version} is no longer read; re-run the analysis"):
        verify_analysis_run_record({"schema_version": version, "analysis_run": {"hashes": []}})


@pytest.mark.parametrize("version", ["0.9.0", "0.3", "", None, 0.3])
def test_unknown_record_versions_are_unsupported(version) -> None:
    with pytest.raises(ValueError, match="^ANALYSIS-RUN-SCHEMA-VERSION-UNSUPPORTED"):
        verify_analysis_run_record({"schema_version": version, "analysis_run": {"hashes": []}})


def test_a_retired_result_is_never_recorded() -> None:
    # b7-plan E2: the producer-less 0.1.0 mechanics result is retired; the reader
    # refuses it by name, so no record of it is built.
    source = json.loads(CURRENT_DEMO_RESULT.read_text())
    source["schema_version"] = "0.1.0"
    for key in ("producer", "numerical_quality", "formulation_basis", "contract_evidence"):
        del source[key]
    before = deepcopy(source)
    with pytest.raises(ValueError, match="^MECHANICS_RESULT_VERSION_RETIRED$"):
        build_analysis_run(source, input_manifest_ref=MANIFEST_REF, input_manifest_hash=MANIFEST_HASH)
    assert source == before


def test_current_record_validates_verifies_and_leaves_its_source_unchanged(joined) -> None:
    source, record = joined
    assert record["schema_version"] == "0.3.0"
    assert record["run_contract_status"]["record_contract"] == "strict_analysis_run_v0_3"
    assert record["analysis_run"]["provenance"]["source_location"] == "analysis_run.compatibility.v0.3"
    assert len(record["analysis_run"]["result_refs"]) == len(source["results"])
    assert all(row["hash_refs"][0]["payload_scope"] == "result_row" for row in record["analysis_run"]["result_refs"])
    assert verify_analysis_run_record(record) == "match"
    validate_instance(dispatcher_schema(), record, instance_label="analysis record 0.3")
    rebuilt = build_analysis_run(json.loads(JOINED_SOURCE.read_text()), input_manifest_ref=MANIFEST_REF, input_manifest_hash=MANIFEST_HASH)
    assert rebuilt == record


def test_rule_revision_changes_only_full_record_identity(joined) -> None:
    source, first = joined
    revised = build_analysis_run_v0_3(source, input_manifest_ref=MANIFEST_REF, input_manifest_hash=MANIFEST_HASH, rule_check_status="USER_RULE_CHECKED")
    assert hash_by_scope(first, "received_result") == hash_by_scope(revised, "received_result")
    assert hash_by_scope(first, "analysis_run_record") != hash_by_scope(revised, "analysis_run_record")


def test_tamper_is_not_a_false_match(joined) -> None:
    _, record = joined
    tampered = deepcopy(record)
    tampered["analysis_run"]["analysis_status"].append("USER_RULE_FAILED")
    assert verify_analysis_run_record(tampered) == "mismatch"


def test_schema_rejects_retired_versions_unknown_fields_status_missing_evidence_and_boundary_claims(joined) -> None:
    _, valid = joined
    cases = []
    for version in ("0.1.0", "0.2.0"):
        retired = deepcopy(valid); retired["schema_version"] = version; cases.append(retired)
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


def test_checksum_positions_require_exact_scope_profile_reference_and_inventory(joined) -> None:
    _, valid = joined
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
    other_record_profile = deepcopy(valid); next(item for item in other_record_profile["analysis_run"]["hashes"] if item["payload_scope"] == "analysis_run_record")["canonicalization"] = "rfc8785_jcs"; cases.append(other_record_profile)
    other_received_profile = deepcopy(valid); next(item for item in other_received_profile["analysis_run"]["hashes"] if item["payload_scope"] == "received_result")["canonicalization"] = "rfc8785_jcs"; cases.append(other_received_profile)
    other_row_profile = deepcopy(valid); other_row_profile["analysis_run"]["result_refs"][0]["hash_refs"][0]["canonicalization"] = "rfc8785_jcs"; cases.append(other_row_profile)
    wrong_record_ref = deepcopy(valid); next(item for item in wrong_record_ref["analysis_run"]["hashes"] if item["payload_scope"] == "analysis_run_record")["payload_ref"]["object_type"] = "ResultEnvelope"; cases.append(wrong_record_ref)
    wrong_received_ref = deepcopy(valid); next(item for item in wrong_received_ref["analysis_run"]["hashes"] if item["payload_scope"] == "received_result")["payload_ref"]["object_type"] = "AnalysisRun"; cases.append(wrong_received_ref)
    wrong_row_ref = deepcopy(valid); wrong_row_ref["analysis_run"]["result_refs"][0]["hash_refs"][0]["payload_ref"]["object_type"] = "InputManifest"; cases.append(wrong_row_ref)
    checked_manifest_profile = deepcopy(valid); checked_manifest_profile["analysis_run"]["reproducibility"]["input_manifest_hashes"][0]["canonicalization"] = "openpipestress_jcs_ijson_v1"; cases.append(checked_manifest_profile)
    wrong_manifest_scope = deepcopy(valid); wrong_manifest_scope["analysis_run"]["reproducibility"]["input_manifest_hashes"][0]["payload_scope"] = "result_row"; cases.append(wrong_manifest_scope)
    wrong_manifest_ref = deepcopy(valid); wrong_manifest_ref["analysis_run"]["reproducibility"]["input_manifest_hashes"][0]["payload_ref"]["object_type"] = "Result"; cases.append(wrong_manifest_ref)
    for case in cases:
        assert_schema_rejects(case)


# Copy checks: a pressure-source-1 record of a load/reference-state model carries
# both the source-block receipt and the contract evidence. Each forged copy is
# resealed with its own record checksum, so only the copy check can refuse it.
JOINED_SOURCE = PROJECT / "fixtures/product_preview/pressure_source/load_reference_source/n05-sparse_interactive.raw.json"
RECEIPT_MISMATCH = "^ANALYSIS_SOURCE_BLOCK_RECEIPT_MISMATCH$"
EVIDENCE_MISMATCH = "^ANALYSIS_PHYSICS_SOURCE_EVIDENCE_MISMATCH$"


@pytest.fixture(scope="module")
def joined() -> tuple[dict, dict]:
    source = json.loads(JOINED_SOURCE.read_text())
    record = build_analysis_run_v0_3(source, input_manifest_ref=MANIFEST_REF, input_manifest_hash=MANIFEST_HASH)
    return source, record


def copied_leaf(value, path: tuple):
    for key in path:
        value = value[key]
    return value


def resealed_copy(record: dict, field: str, path: tuple, value) -> dict:
    forged = deepcopy(record)
    copied_leaf(forged["analysis_run"][field], path[:-1])[path[-1]] = value
    seal = next(row for row in forged["analysis_run"]["hashes"] if row["payload_scope"] == "analysis_run_record")
    seal["value"] = canonical_sha256_checked_v1(analysis_record_projection(forged))
    assert verify_analysis_run_record(forged) == "match"
    return forged


def as_javascript_writes(value):
    """JSON.stringify writes an integral float without its fraction."""
    if type(value) is float and value.is_integer():
        return int(value)
    if type(value) is dict:
        return {key: as_javascript_writes(item) for key, item in value.items()}
    if type(value) is list:
        return [as_javascript_writes(item) for item in value]
    return value


@pytest.mark.parametrize(("field", "path", "value", "code"), [
    ("source_block_recovery", ("body", "cases", 0, "source", "force_term_count"), True, RECEIPT_MISMATCH),
    ("source_block_recovery", ("body", "cases", 0, "ordinary_attempt", "quality_case_index"), False, RECEIPT_MISMATCH),
    ("contract_evidence", ("exact_cases", 0, "pressure_rhs_assembly", "screen_is_not_numerical_qualification"), 1, EVIDENCE_MISMATCH),
    ("contract_evidence", ("exact_cases", 0, "pipe_materials", 0, "thermal_consumed"), 0, EVIDENCE_MISMATCH),
])
def test_copy_checks_refuse_a_boolean_for_an_integer(joined, field, path, value, code) -> None:
    source, record = joined
    original = copied_leaf(source[field], path)
    assert original == value and type(original) is not type(value)
    forged = resealed_copy(record, field, path, value)
    # Python equality cannot see the swap; the canonical bytes differ.
    assert forged["analysis_run"][field] == source[field]
    with pytest.raises(ValueError, match=code):
        validate_analysis_run_v0_3(forged, source)


@pytest.mark.parametrize(("field", "path", "value"), [
    ("source_block_recovery", ("body", "cases", 0, "source", "force_term_count"), 1.0),
    ("contract_evidence", ("exact_cases", 0, "pipe_materials", 0, "resolved_eigenstrain"), 0),
])
def test_copy_checks_treat_an_integral_float_as_the_same_json_value(joined, field, path, value) -> None:
    # 1 and 1.0 have the same checked canonical bytes, so the record checksum is
    # unchanged; TS's same() cannot tell them apart either.
    source, record = joined
    original = copied_leaf(source[field], path)
    assert original == value and type(original) is not type(value)
    forged = resealed_copy(record, field, path, value)
    assert hash_by_scope(forged, "analysis_run_record") == hash_by_scope(record, "analysis_run_record")
    validate_analysis_run_v0_3(forged, source)


def test_copy_checks_accept_exact_stored_and_javascript_written_copies(joined) -> None:
    source, record = joined
    validate_analysis_run_v0_3(record, source)
    validate_analysis_run_v0_3(json.loads(json.dumps(record)), source)
    written = as_javascript_writes(record)
    assert json.dumps(written["analysis_run"]["source_block_recovery"]) != json.dumps(record["analysis_run"]["source_block_recovery"])
    assert json.dumps(written["analysis_run"]["contract_evidence"]) != json.dumps(record["analysis_run"]["contract_evidence"])
    validate_analysis_run_v0_3(written, source)


# Model document 0.8.0 (DEL-02-01 Design model-document-0.8.0.md §3.2; owner ruling "0.8.0 a2"): a
# pressure-2 run recorded before the ripple, main's record at 2e4748eada verbatim, binds the 0.7.0-era table
# (3e2aac3c…) and keeps its existing read; that digest is the recorded basis of exactly the current table.
_RECORDED_070 = Path(__file__).resolve().parents[1] / "fixtures/persistence/recorded_model_0_7_0"


def test_a_pressure_2_run_recorded_at_model_0_7_0_reads_as_before():
    from core.analysis_runs import compatibility as c
    envelope = json.loads((_RECORDED_070 / "pressure_plain_connected_sparse.analysis_run.json").read_text())
    source = json.loads((_RECORDED_070 / "pressure_connected_mechanics_sparse.json").read_text())
    assert envelope["analysis_run"]["reproducibility"]["semantic_contract"]["sha256"].startswith("3e2aac3c")
    assert c.verify_analysis_run_record(envelope) == "match"
    c.validate_analysis_run_v0_3(envelope, source, expected_basis_refs=envelope["analysis_run"]["load_basis_refs"])


def test_the_recorded_basis_warrants_no_other_digest():
    from core.analysis_runs import compatibility as c
    envelope = json.loads((_RECORDED_070 / "pressure_plain_connected_sparse.analysis_run.json").read_text())
    source = json.loads((_RECORDED_070 / "pressure_connected_mechanics_sparse.json").read_text())
    changed = json.loads(json.dumps(envelope))
    changed["analysis_run"]["reproducibility"]["semantic_contract"]["sha256"] = "0" * 64
    with pytest.raises(ValueError, match="ANALYSIS_SEMANTIC_CONTRACT_MISMATCH"):
        c.validate_analysis_run_v0_3(changed, source, expected_basis_refs=changed["analysis_run"]["load_basis_refs"])
    mixed = json.loads(json.dumps(envelope))
    mixed["analysis_run"]["result_refs"][0]["semantic_contract"]["sha256"] = c.PRESSURE_CONTRACT_SHA256
    with pytest.raises(ValueError, match="ANALYSIS_ROW_SOURCE_BINDING_MISMATCH"):
        c.validate_analysis_run_v0_3(mixed, source, expected_basis_refs=mixed["analysis_run"]["load_basis_refs"])


# Model document 0.9.0 (DEL-02-01 Design model-document-0.9.0.md §3.2; owner ruling "0.8.0 a2"): a pressure-2 run
# recorded before the 0.9.0 ripple, main's record at 5bb2f9ff0 verbatim, binds the 0.8.0-era table (5451fbf3…, matched
# by prefix; the fixture is outside semantic_contract_tables.py repin's reach) and keeps its existing read.
_RECORDED_080 = Path(__file__).resolve().parents[1] / "fixtures/persistence/recorded_model_0_8_0"


def test_a_pressure_2_run_recorded_at_model_0_8_0_reads_as_before():
    from core.analysis_runs import compatibility as c
    envelope = json.loads((_RECORDED_080 / "pressure_plain_connected_sparse.analysis_run.json").read_text())
    source = json.loads((_RECORDED_080 / "pressure_connected_mechanics_sparse.json").read_text())
    model = json.loads((_RECORDED_080 / "exact_pressure_connected_request.json").read_text())["model"]
    assert model["schema_version"] == "0.8.0"
    assert envelope["analysis_run"]["reproducibility"]["semantic_contract"]["sha256"].startswith("5451fbf3")
    assert c.verify_analysis_run_record(envelope) == "match"
    c.validate_analysis_run_v0_3(envelope, source, expected_basis_refs=envelope["analysis_run"]["load_basis_refs"])
    changed = json.loads(json.dumps(envelope))
    changed["analysis_run"]["reproducibility"]["semantic_contract"]["sha256"] = "0" * 64
    with pytest.raises(ValueError, match="ANALYSIS_SEMANTIC_CONTRACT_MISMATCH"):
        c.validate_analysis_run_v0_3(changed, source, expected_basis_refs=changed["analysis_run"]["load_basis_refs"])
