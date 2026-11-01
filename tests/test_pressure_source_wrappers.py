"""J3/J5 wrappers over controlled Rust verdicts and invented record inputs.

These checks cover reader vocabulary, refusal propagation and record copy/hash
binding. They do not validate mechanics, define a numerical oracle or provide a
native witness; the unmodified J2 corpus checks the actual Rust readers.
"""
from copy import deepcopy
import hashlib
import json
from pathlib import Path
from types import SimpleNamespace

import pytest

from core.analysis_runs import compatibility as c
from core.analysis_runs import rust_reader

CONTRACT_ID = "openpipestress.result_semantics/0.3.0/pressure-source-1"
# B7b (a43583c625, "Rebase retained exact evidence onto current pressure tables"): the current pressure-source-1
# table, policy PRESSURE-SOURCE-1, 69 signatures (was 4a1873e7..., the table before its B7b re-issue).
# T4-U6 (D-5, §2.4): re-issued with one stress signature per (recovery token x section basis), 88
# signatures (was dec4f4e7...).
CONTRACT_SHA256 = "84f37c63c6cc5ec7539d6ee2b39c338bd2c6ef16b25772005fe0c8f9300fddb9"
assert CONTRACT_SHA256 == c.PRESSURE_SOURCE_CONTRACT_SHA256
BINDING = {"id": CONTRACT_ID, "sha256": CONTRACT_SHA256}


def record_test_text(value):
    """A deterministic hash seam for record wiring only; no claim of JCS parity."""
    return json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":"), allow_nan=False)


def record_test_hash(value):
    return hashlib.sha256(record_test_text(value).encode()).hexdigest()


def invented_source():
    return {
        "schema_version": "0.2.0", "document_kind": "MechanicsResult",
        "run_id": "invented:pressure-wrapper:run", "model_ref": "invented:pressure-wrapper:model",
        "producer": {"component_name": "open_pipe_stress_product_physics", "component_version": "0.2.0", "semantic_contract_id": CONTRACT_ID},
        "numerical_quality": {"status": "sensitive", "cases": []},
        "formulation_basis": {"profile_id": "exact_pressure_v3", "limitations": ["Invented wrapper input; Rust verdict is controlled"]},
        "status": {"mechanics": "MECHANICS_SOLVED", "rule_check": "RULE_INPUTS_INCOMPLETE"},
        "results": [{"id": "invented:pressure-wrapper:row", "kind": "global_nodal_rotation_x", "unit": "rad", "value": 0.01,
                     "metadata": {"component": "nodal_rotation_x"}, "basis_ref": {"ref_type": "load_case", "ref_id": "invented:pressure-wrapper:case"}}],
        "diagnostics": [], "source_block_recovery": {"body": {"policy": "PRESSURE-SOURCE-1"}},
        "contract_evidence": {"invented_wrapper_evidence": "copied unchanged"},
    }


@pytest.fixture
def controlled_rust(monkeypatch):
    """The real wrapper parses runner envelopes; only the runner's outcomes are controlled."""
    calls = []
    outcomes = {
        "dispatch": {"ok": True, "route": "pressure_source", "binding": deepcopy(BINDING), "error": None,
                     "standing": {"route": "pressure_source", "token": "needs_recompute", "status": "needs_recompute", "eligible": False,
                                  "findings": ["SOURCE_BLOCKS_VALIDATED_INVOCATION_REQUIRED"]}},
        "pressure-source": {"ok": True, "eligible": False, "findings": []},
        "pressure-source-transport": {"ok": True},
        "standing": {"ok": True, "token": "needs_recompute", "reason": "LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE_IN_T1"},
    }

    def completed(command, *, input, capture_output, check, shell):
        assert command[:2] == ["/invented/openpipestress-runner", "read"]
        assert (capture_output, check, shell) == (True, False, False)
        reader = command[2]
        calls.append((reader, json.loads(input)))
        outcome = deepcopy(outcomes[reader])
        envelope = {"artifact": rust_reader.ARTIFACT, "schema_version": rust_reader.SCHEMA_VERSION, "reader": reader, "outcome": outcome}
        return SimpleNamespace(returncode=0 if outcome["ok"] else 1, stdout=json.dumps(envelope).encode(), stderr=b"")

    monkeypatch.setattr(rust_reader, "runner_executable", lambda: Path("/invented/openpipestress-runner"))
    monkeypatch.setattr(rust_reader.subprocess, "run", completed)
    monkeypatch.setattr(c, "canonical_json_checked_v1", record_test_text)
    return outcomes, calls


def build_record(source):
    return c.build_analysis_run(source, input_manifest_ref={"object_type": "InputManifest", "ref": "invented:pressure-wrapper:manifest"},
                                input_manifest_hash="2" * 64, hash_fn=record_test_hash)


def reseal(record):
    record["analysis_run"]["hashes"][0]["value"] = record_test_hash(c.analysis_record_projection(record))
    assert c.verify_analysis_run_record(record, record_test_hash) == "match"


def test_pressure_route_uses_explicit_raw_and_transport_readers(controlled_rust):
    _, calls = controlled_rust
    source = invented_source()
    assert rust_reader.dispatch(source)["route"] == "pressure_source"
    invocation = {"request": {"model": {"schema_version": "0.4.0"}}, "solver_mode": "dense_scrutiny"}
    assert rust_reader.validate("pressure-source", source, invocation=invocation) == {"ok": True, "eligible": False, "findings": []}
    rust_reader.transport("pressure-source", source)
    assert calls[-2:] == [("pressure-source", {"source": source, "invocation": invocation}), ("pressure-source-transport", {"source": source})]
    assert c.numerical_use_standing(source, [], invocation) == "needs_recompute"


def test_pressure_raw_and_transport_table_binding_is_reviewed_without_parsing_rows(controlled_rust, monkeypatch):
    _, calls = controlled_rust
    source = invented_source()

    def row_read_forbidden(*args):
        raise AssertionError("A header/reader selection must not parse a row table")

    monkeypatch.setattr(c, "_semantic_rows", row_read_forbidden)
    expected = (CONTRACT_ID, CONTRACT_SHA256, c._PRESSURE_SOURCE_CONTRACT_PATH)
    assert c._source_contract(source) == expected
    assert c._source_contract(source, check_receipt=False) == expected
    assert calls[1] == ("pressure-source", {"source": source})
    assert calls[2][0] == "dispatch" and "results" not in calls[2][1]["source"]
    assert calls[3] == ("pressure-source-transport", {"source": source})


@pytest.mark.parametrize("reader,check_receipt", [("pressure-source", True), ("pressure-source-transport", False)])
def test_pressure_wrapper_propagates_rust_refusal_without_a_fallback(controlled_rust, reader, check_receipt):
    outcomes, _ = controlled_rust
    code = "SOURCE_BLOCKS_DOWNGRADE_FORBIDDEN"
    outcomes[reader] = {"ok": False, "error": {"code": code, "detail": None}}
    with pytest.raises(ValueError, match=code):
        c._source_contract(invented_source(), check_receipt=check_receipt)


def test_pressure_dispatch_refusal_never_selects_a_reader(controlled_rust):
    outcomes, calls = controlled_rust
    outcomes["dispatch"] = {"ok": False, "route": "unsupported", "binding": None,
                            "error": {"code": "SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED", "detail": None},
                            "standing": {"route": "unsupported", "token": "unsupported", "status": "needs_recompute", "eligible": False,
                                         "findings": ["SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED"]}}
    with pytest.raises(ValueError, match="SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED"):
        build_record(invented_source())
    assert [reader for reader, _ in calls] == ["dispatch"]


def test_pressure_reader_response_shape_and_binding_fail_closed(controlled_rust):
    outcomes, _ = controlled_rust
    outcomes["pressure-source"]["eligible"] = "false"
    with pytest.raises(RuntimeError, match="RUST-READER-RESPONSE-INVALID"):
        rust_reader.validate("pressure-source", invented_source())
    outcomes["pressure-source"]["eligible"] = False
    outcomes["dispatch"]["binding"]["sha256"] = "0" * 64
    with pytest.raises(RuntimeError, match="RUST-READER-BINDING-UNKNOWN"):
        c._source_contract(invented_source())


def test_current_pressure_record_copies_both_namespaces(controlled_rust):
    source = invented_source()
    before = deepcopy(source)
    record = build_record(source)
    assert source == before and record["schema_version"] == "0.3.0"
    run = record["analysis_run"]
    assert run["reproducibility"]["semantic_contract"] == BINDING
    assert run["result_refs"][0]["semantic_contract"] == {**BINDING, "signature_id": "supported-source-005"}
    for field in ("source_block_recovery", "contract_evidence"):
        assert run[field] == source[field] and run[field] is not source[field]
    assert c.verify_analysis_run_record(record, record_test_hash) == "match"


@pytest.mark.parametrize("field,code", [
    ("source_block_recovery", "ANALYSIS_SOURCE_BLOCK_RECEIPT_MISMATCH"),
    ("contract_evidence", "ANALYSIS_PHYSICS_SOURCE_EVIDENCE_MISMATCH"),
])
def test_pressure_record_rejects_resealed_changed_copies(controlled_rust, field, code):
    source = invented_source()
    record = build_record(source)
    record["analysis_run"][field] = {"invented_substitution": True}
    reseal(record)
    with pytest.raises(ValueError, match=code):
        c.validate_analysis_run_v0_3(record, source, hash_fn=record_test_hash)


def test_pressure_record_rejects_resealed_binding_and_row_hash_changes(controlled_rust):
    source = invented_source()
    record = build_record(source)
    wrong_binding = deepcopy(record)
    wrong_binding["analysis_run"]["reproducibility"]["semantic_contract"]["sha256"] = "0" * 64
    reseal(wrong_binding)
    with pytest.raises(ValueError, match="ANALYSIS_SEMANTIC_CONTRACT_MISMATCH"):
        c.validate_analysis_run_v0_3(wrong_binding, source, hash_fn=record_test_hash)
    wrong_row = deepcopy(record)
    wrong_row["analysis_run"]["result_refs"][0]["hash_refs"][0]["value"] = "0" * 64
    reseal(wrong_row)
    with pytest.raises(ValueError, match="ANALYSIS_ROW_SOURCE_BINDING_MISMATCH"):
        c.validate_analysis_run_v0_3(wrong_row, source, hash_fn=record_test_hash)


def test_pressure_record_recomputes_received_source_digest(controlled_rust):
    source = invented_source()
    record = build_record(source)
    changed = deepcopy(source)
    changed["results"][0]["value"] = 0.02
    assert c.verify_analysis_run_record(record, record_test_hash) == "match"
    with pytest.raises(ValueError, match="ANALYSIS_RECEIVED_SOURCE_MISMATCH"):
        c.validate_analysis_run_v0_3(record, changed, hash_fn=record_test_hash)
