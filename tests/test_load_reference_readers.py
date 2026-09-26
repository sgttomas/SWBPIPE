"""load-reference-1 readers: shared Rust/Python adversarial parity and carriers.

The case file is shared with the Rust reader test
``core/reporting/result_export/tests/load_reference_contract.rs``. Both readers
must reproduce every expectation, so they accept and reject the same inputs with
the same error. Carrier fixtures under ``fixtures/results/load_reference_*`` are
derived from the four frozen producer envelopes by these readers; they prove
reader preservation, not producer authentication or engineering acceptance.
"""
from __future__ import annotations

from copy import deepcopy
import hashlib
import json
import math
import os
from pathlib import Path

import pytest

from core.analysis_runs.compatibility import (
    LOAD_REFERENCE_CONTRACT_ID, LOAD_REFERENCE_CONTRACT_SHA256, LOAD_REFERENCE_PROFILE,
    PHYSICS_CONTRACT_ID, _source_contract, build_analysis_run, build_analysis_run_v0_3,
    numerical_use_standing, validate_analysis_run_v0_3, verify_analysis_run_record,
)
from core.analysis_runs.load_reference_evidence import (
    TRANSPORT_SCHEMA_SHA256, load_reference_table, validate_load_reference_evidence,
    validate_load_reference_transport_metadata, verify_load_reference_table,
)
from core.serialization.canonical_json.adapter import canonical_sha256_checked_v1

PROJECT = Path(__file__).resolve().parents[1]
CASES_PATH = PROJECT / "core/reporting/result_export/tests/fixtures/load_reference_mutations.json"
CASES = json.loads(CASES_PATH.read_text())
FROZEN = {
    "fixtures/product_preview/load_reference/connected.request.json": "ff0ce8724d7734c2cd8d10ce866085d80741adadc0da529521084dc8382453a2",
    "fixtures/product_preview/load_reference/connected-sparse_interactive.raw.json": "915965a446ff04c6c4b7f0209ffff34c4ac040a0d4b8243d875dc7217a9cb74c",
    "fixtures/product_preview/load_reference/connected-dense_scrutiny.raw.json": "3824035cabf2d2a151756e04b46112ccbdcf4ce8237f49e68b5d3b36b8dfb2bf",
    "fixtures/product_preview/load_reference/pressure.request.json": "52375ad60d05074ed6064b72d78d9c24d19b1a53915bcc46c2b424be7c39ef69",
    "fixtures/product_preview/load_reference/pressure-sparse_interactive.raw.json": "71be3e4fb1af54e962d5e4939a518fcab6f7896842ab4f08212a8212ae47f134",
    "fixtures/product_preview/load_reference/pressure-dense_scrutiny.raw.json": "b46eeb8a5dc82bb9e0398b8d58db07eb73906eb25110d68b0032fd12df1d0707",
    "fixtures/results/semantic_contract_v0_3_load_reference_1.json": "44bc41c06f589fab6ce931ac0eaa5344765ff64fd5f880cc2dd69ecb839c4f4d",
    "schemas/load_reference_state.schema.json": TRANSPORT_SCHEMA_SHA256,
}
RAW = ["connected-sparse", "connected-dense", "pressure-sparse", "pressure-dense"]
PHYSICS_PREFIX = "SOURCE_LOAD_REFERENCE_PHYSICS_EVIDENCE"
RESULTS: list[dict] = []


def source(name: str) -> dict:
    return json.loads((PROJECT / CASES["sources"][name]).read_text())


def _tokens(path: str) -> list[str]:
    assert path.startswith("/"), path
    return [token.replace("~1", "/").replace("~0", "~") for token in path[1:].split("/")]


def _resolve(doc, path):
    for token in _tokens(path) if path else []:
        doc = doc[int(token)] if isinstance(doc, list) else doc[token]
    return doc


def _parent(doc, path):
    tokens = _tokens(path)
    parent = doc
    for token in tokens[:-1]:
        parent = parent[int(token)] if isinstance(parent, list) else parent[token]
    return parent, tokens[-1]


def _set(doc, path, value):
    parent, last = _parent(doc, path)
    if isinstance(parent, list):
        parent[int(last)] = value
    else:
        parent[last] = value


def _matches(item, match):
    return isinstance(item, dict) and all(item.get(key) == value for key, value in match.items())


def apply(doc: dict, ops: list[dict]) -> dict:
    """Mirror of the Rust test harness. Non-finite values become Python floats."""
    doc = deepcopy(doc)
    for op in ops:
        kind = op["op"]
        if kind == "set":
            _set(doc, op["path"], deepcopy(op["value"]))
        elif kind == "remove":
            parent, last = _parent(doc, op["path"])
            if isinstance(parent, list):
                parent.pop(int(last))
            else:
                del parent[last]
        elif kind == "append":
            _resolve(doc, op["path"]).append(deepcopy(op["value"]))
        elif kind == "copy":
            _set(doc, op["path"], deepcopy(_resolve(doc, op["from"])))
        elif kind == "copy_append":
            _resolve(doc, op["path"]).append(deepcopy(_resolve(doc, op["from"])))
        elif kind == "swap":
            first, second = deepcopy(_resolve(doc, op["path"])), deepcopy(_resolve(doc, op["with"]))
            _set(doc, op["path"], second)
            _set(doc, op["with"], first)
        elif kind == "graft":
            _set(doc, op["path"], deepcopy(_resolve(source(op["source"]), op["from"])))
        elif kind == "remove_where":
            array = _resolve(doc, op["path"])
            array[:] = [item for item in array if not _matches(item, op["match"])]
        elif kind == "set_where":
            for item in _resolve(doc, op["path"]):
                if _matches(item, op["match"]):
                    item[op["key"]] = deepcopy(op["value"])
        elif kind == "append_copy_where":
            array = _resolve(doc, op["path"])
            copy = deepcopy(next(item for item in array if _matches(item, op["match"])))
            copy.update(deepcopy(op["set"]))
            array.append(copy)
        elif kind == "nonfinite":
            _set(doc, op["path"], float({"Infinity": "inf", "-Infinity": "-inf", "NaN": "nan"}[op["value"]]))
        elif kind == "json_text":
            _set(doc, op["path"], json.loads(op["value"]))
        else:
            raise AssertionError(f"unknown op {kind}")
    return doc


def outcome(call) -> str:
    try:
        call()
    except ValueError as error:
        return str(error)
    return "accept"


def agrees(actual: str, expected: str) -> bool:
    if expected == PHYSICS_PREFIX:
        return actual.split(": ", 1)[0] == PHYSICS_PREFIX
    return actual == expected


def expected(case: dict, kind: str) -> str:
    return case.get(f"{kind}_python", case.get(kind) if kind == "dispatch" else case.get("validator", case.get("dispatch")))


def bases(raw):
    return [case["basis_ref"] for case in raw["numerical_quality"]["cases"]]


def metadata(raw):
    """Transport view as a carrier retains it: no rows, diagnostics or status."""
    return {"schema_version": raw["schema_version"], **{key: raw[key] for key in ("producer", "numerical_quality", "formulation_basis", "contract_evidence", "source_block_recovery", "carrier_evidence") if key in raw}}


def test_frozen_inputs_and_table_identity():
    for path, digest in FROZEN.items():
        assert hashlib.sha256((PROJECT / path).read_bytes()).hexdigest() == digest, path
    table = load_reference_table()
    physics = json.loads((PROJECT / "fixtures/results/semantic_contract_v0_3_physics_1.json").read_text())
    assert LOAD_REFERENCE_CONTRACT_SHA256 == FROZEN["fixtures/results/semantic_contract_v0_3_load_reference_1.json"]
    assert table["semantic_contract_id"] == LOAD_REFERENCE_CONTRACT_ID and table["formulation_profile_id"] == LOAD_REFERENCE_PROFILE
    # Result rows use the physics-1 signatures unchanged.
    assert table["rows"] == physics["rows"] and table["canonical_metadata_vocabulary"] == physics["canonical_metadata_vocabulary"]
    assert "openpipestress.result_semantics/0.3.0/load-reference-source-1" in table["reserved_inactive_successors"]


@pytest.mark.parametrize("case", CASES["cases"], ids=lambda case: case["id"])
def test_shared_adversarial_case(case):
    doc = apply(source(case["source"]), case["ops"])
    dispatch = outcome(lambda: _source_contract(doc))
    validator = outcome(lambda: validate_load_reference_evidence(doc))
    RESULTS.append({"id": case["id"], "dispatch": dispatch, "validator": validator})
    assert agrees(dispatch, expected(case, "dispatch")), (dispatch, expected(case, "dispatch"))
    assert agrees(validator, expected(case, "validator")), (validator, expected(case, "validator"))
    if dispatch == "accept" and "carrier_python" in case:
        # Both readers admit it; the checked-JSON carrier profile does not.
        assert _source_contract(doc)[0] == LOAD_REFERENCE_CONTRACT_ID
        assert outcome(lambda: build_analysis_run(doc, input_manifest_ref={"object_type": "InputManifest", "ref": "manifest:load-reference-reader-test"}, input_manifest_hash="1" * 64)) == case["carrier_python"]
        assert numerical_use_standing(doc, bases(doc)) == "numerically_eligible"
    elif dispatch == "accept":
        assert _source_contract(doc)[0] == LOAD_REFERENCE_CONTRACT_ID
        record = build_analysis_run(doc, input_manifest_ref={"object_type": "InputManifest", "ref": "manifest:load-reference-reader-test"}, input_manifest_hash="1" * 64)
        validate_analysis_run_v0_3(record, doc)
        assert "contract_evidence" not in record["analysis_run"] and "source_block_recovery" not in record["analysis_run"]
    else:
        with pytest.raises(ValueError):
            build_analysis_run(doc, input_manifest_ref={"object_type": "InputManifest", "ref": "manifest:load-reference-reader-test"}, input_manifest_hash="1" * 64)
        assert numerical_use_standing(doc, bases(doc)) == "unsupported"


@pytest.mark.parametrize("case", CASES["table_cases"], ids=lambda case: case["id"])
def test_table_bytes_are_pinned(case):
    data = (PROJECT / "fixtures/results/semantic_contract_v0_3_load_reference_1.json").read_text()
    if case["find"] is not None:
        assert case["find"] in data
        data = data.replace(case["find"], case["replace"], 1)
    result = outcome(lambda: verify_load_reference_table(data.encode()))
    RESULTS.append({"id": case["id"], "table": result})
    assert result == case["expect"]


@pytest.mark.parametrize("case", CASES["transport_cases"], ids=lambda case: case["id"])
def test_transport_metadata_case(case):
    meta = metadata(apply(source(case["source"]), case["ops"]))
    result = outcome(lambda: validate_load_reference_transport_metadata(meta))
    RESULTS.append({"id": case["id"], "transport": result})
    assert agrees(result, case["expect"]), result
    if result == "accept":
        assert _source_contract(meta, check_receipt=False)[0] == LOAD_REFERENCE_CONTRACT_ID


def test_nonfinite_cases_are_the_only_language_specific_representation():
    special = [case for case in CASES["cases"] if "dispatch_rust" in case or "dispatch_python" in case]
    assert {case["id"] for case in special} == {"TABLE-id-precision-1", "NONFINITE-member-E", "NONFINITE-factor", "NONFINITE-row-value", "NONFINITE-region-pressure", "N2-integer-overflow-positive", "N2-integer-overflow-negative", "N2-integer-first-overflow"}
    assert all(case.get("note") for case in special)


@pytest.mark.parametrize("name", RAW)
def test_actual_envelopes_admit_numerically_and_build_physics_shaped_analysis_run(name):
    raw = source(name)
    before = deepcopy(raw)
    assert _source_contract(raw) == (LOAD_REFERENCE_CONTRACT_ID, LOAD_REFERENCE_CONTRACT_SHA256, PROJECT / "fixtures/results/semantic_contract_v0_3_load_reference_1.json")
    assert numerical_use_standing(raw, bases(raw)) == "numerically_eligible"
    record = build_analysis_run_v0_3(raw, input_manifest_ref={"object_type": "InputManifest", "ref": "manifest:load-reference-reader-test"}, input_manifest_hash="1" * 64)
    assert raw == before
    assert verify_analysis_run_record(record) == "match"
    run = record["analysis_run"]
    assert run["reproducibility"]["semantic_contract"] == {"id": LOAD_REFERENCE_CONTRACT_ID, "sha256": LOAD_REFERENCE_CONTRACT_SHA256}
    assert "contract_evidence" not in run and "source_block_recovery" not in run
    assert len(run["result_refs"]) == len(raw["results"])
    assert all(ref["interpretation"]["status"] != "unavailable" for ref in run["result_refs"] if ref["source_annotation"]["kind"].endswith("_v2"))
    received = next(item for item in run["hashes"] if item["payload_scope"] == "received_result")
    assert received["value"] == canonical_sha256_checked_v1(raw)
    # A downgraded record (carrying evidence physics-1 does not carry) is refused.
    downgraded = deepcopy(record)
    downgraded["analysis_run"]["contract_evidence"] = deepcopy(raw["contract_evidence"])
    with pytest.raises(ValueError, match="ANALYSIS_PHYSICS_SOURCE_DOWNGRADE_FORBIDDEN"):
        validate_analysis_run_v0_3(downgraded, raw)
    # A wrong table hash in the record is refused.
    forged = deepcopy(record)
    forged["analysis_run"]["reproducibility"]["semantic_contract"]["sha256"] = "0" * 64
    with pytest.raises(ValueError, match="ANALYSIS_SEMANTIC_CONTRACT_MISMATCH"):
        validate_analysis_run_v0_3(forged, raw)


def _analysis_run_fixture(name: str) -> Path:
    return PROJECT / f"fixtures/results/load_reference_{name.replace('-', '_')}.analysis_run.json"


def _carrier_bytes(value) -> bytes:
    """Compact bytes plus a newline: the carriers are large and regenerated by rerun."""
    return (json.dumps(value, separators=(",", ":"), ensure_ascii=False) + "\n").encode()


@pytest.mark.parametrize("name", RAW)
def test_analysis_run_carrier_fixture_is_reproduced_byte_for_byte(name):
    raw = source(name)
    record = build_analysis_run_v0_3(raw, input_manifest_ref={"object_type": "InputManifest", "ref": f"manifest:load-reference-{name}"}, input_manifest_hash="1" * 64)
    data = _carrier_bytes(record)
    path = _analysis_run_fixture(name)
    if os.environ.get("LOAD_REFERENCE_WRITE_FIXTURES") == "1":
        path.write_bytes(data)
    assert path.read_bytes() == data
    stored = json.loads(path.read_text())
    validate_analysis_run_v0_3(stored, raw)
    assert verify_analysis_run_record(stored) == "match"


def test_physics_one_dispatch_is_unchanged_and_load_reference_evidence_is_forbidden():
    physics = source("physics-sparse")
    assert _source_contract(physics)[0] == PHYSICS_CONTRACT_ID
    physics["contract_evidence"]["load_reference_states"] = []
    assert outcome(lambda: _source_contract(physics)) == "SOURCE_LOAD_REFERENCE_EVIDENCE_FORBIDDEN"
    meta = metadata(physics)
    assert outcome(lambda: _source_contract(meta, check_receipt=False)) == "SOURCE_LOAD_REFERENCE_EVIDENCE_FORBIDDEN"


def test_zzz_write_parity_log():
    """Evidence only: the per-case outcomes for the return record."""
    folder = os.environ.get("LOAD_REFERENCE_PARITY_OUT")
    if not folder:
        pytest.skip("set LOAD_REFERENCE_PARITY_OUT to record per-case outcomes")
    Path(folder).mkdir(parents=True, exist_ok=True)
    (Path(folder) / "python_outcomes.json").write_text(json.dumps(RESULTS, indent=1) + "\n")
