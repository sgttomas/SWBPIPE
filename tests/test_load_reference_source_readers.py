"""load-reference-source-1 readers: shared Rust/Python adversarial parity and carriers.

The case file is shared with the Rust reader test
``core/reporting/result_export/tests/load_reference_source_contract.rs``. Both
readers must reproduce every expectation, so they accept and reject the same
inputs with the same error. Carrier fixtures under
``fixtures/results/load_reference_source_*`` are derived from the ten committed
joined producer envelopes by these readers; they prove reader preservation, not
producer authentication, numerical eligibility or engineering acceptance. All
inputs are invented.
"""
from __future__ import annotations

from copy import deepcopy
import hashlib
import json
import os
from pathlib import Path

import pytest

from core.analysis_runs.compatibility import (
    LOAD_REFERENCE_CONTRACT_ID, LOAD_REFERENCE_SOURCE_CONTRACT_ID, LOAD_REFERENCE_SOURCE_CONTRACT_SHA256,
    LOAD_REFERENCE_SOURCE_PROFILE, _source_contract, build_analysis_run, build_analysis_run_v0_3,
    numerical_use_standing, validate_analysis_run_v0_3, verify_analysis_run_record,
)
from core.analysis_runs.load_reference_evidence import validate_load_reference_evidence
from core.analysis_runs.load_reference_source import (
    POLICY, load_reference_source_table, validate_load_reference_source_evidence,
    validate_load_reference_source_transport_metadata, verify_table,
)
from core.analysis_runs.physics_source import validate_physics_source
from core.analysis_runs.source_blocks import domain_hash
from core.serialization.canonical_json.adapter import canonical_sha256_checked_v1

PROJECT = Path(__file__).resolve().parents[1]
CASES_PATH = PROJECT / "core/reporting/result_export/tests/fixtures/load_reference_source_mutations.json"
CASES = json.loads(CASES_PATH.read_text())
TABLE_PATH = PROJECT / "fixtures/results/semantic_contract_v0_3_load_reference_source_1.json"
NAMES = ["n05", "n06", "fields", "mixed", "eigen_motion"]
MODES = {"sparse": "sparse_interactive", "dense": "dense_scrutiny"}
JOINED = [f"{name}-{mode}" for name in NAMES for mode in MODES]
FROZEN = {
    "fixtures/results/semantic_contract_v0_3_load_reference_source_1.json": "d1628194a7730f427843b00228dd233cf92b8e7d26f3bc31c660a3ea59e28337",
    "fixtures/results/semantic_contract_v0_3_physics_source_1.json": "ba13f2aefd7a38bd725e5f111e6ec30144bc8776aa957c6278ee7b1178298ba1",
}
INHERITED = "SOURCE_LOAD_REFERENCE_JOIN_PHYSICS_SOURCE"
MANIFEST = {"object_type": "InputManifest", "ref": "manifest:load-reference-source-reader-test"}
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


def _reseal(doc, op):
    """Recompute the receipt commitments so that a later check can be reached."""
    evidence = doc["contract_evidence"]
    body = doc["source_block_recovery"]["body"]
    if op["physical"] is not None:
        for index, (case, exact) in enumerate(zip(body["cases"], evidence["exact_cases"])):
            pressure = [p for p in evidence["pressure"] if p["load_case_id"] == exact["load_case_id"]]
            if op["physical"] == "joined":
                case["physical_evidence_sha256"] = domain_hash("load_reference_source_case_evidence_v1", {"exact_case": exact, "pressure": pressure, "load_reference_state": evidence["load_reference_states"][index]})
            elif op["physical"] == "joined_without_record":
                case["physical_evidence_sha256"] = domain_hash("load_reference_source_case_evidence_v1", {"exact_case": exact, "pressure": pressure})
            elif op["physical"] == "physics":
                case["physical_evidence_sha256"] = domain_hash("physics_source_case_evidence_v1", {"exact_case": exact, "pressure": pressure})
            else:
                raise AssertionError(op)
    if op["publication"]:
        body["publication_sha256"] = domain_hash("source_blocks_publication_v1", {key: value for key, value in doc.items() if key != "source_block_recovery"})
    if op["receipt"]:
        doc["source_block_recovery"]["receipt_sha256"] = domain_hash("source_blocks_receipt_v1", body)


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
        elif kind == "reseal":
            _reseal(doc, op)
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
    if expected == INHERITED:
        return actual.split(": ", 1)[0] == INHERITED
    return actual == expected


def expected(case: dict, kind: str) -> str | None:
    if f"{kind}_python" in case:
        return case[f"{kind}_python"]
    if kind == "joined":
        return case.get("joined", expected(case, "dispatch"))
    return case.get(kind)


def bases(raw):
    return [case["basis_ref"] for case in raw["numerical_quality"]["cases"]]


def metadata(raw):
    """Transport view as a carrier retains it: no rows, diagnostics or status."""
    return {"schema_version": raw["schema_version"], **{key: raw[key] for key in ("producer", "numerical_quality", "formulation_basis", "contract_evidence", "source_block_recovery", "carrier_evidence") if key in raw}}


def test_pinned_table_and_inputs():
    for path, digest in FROZEN.items():
        assert hashlib.sha256((PROJECT / path).read_bytes()).hexdigest() == digest, path
    assert LOAD_REFERENCE_SOURCE_CONTRACT_SHA256 == FROZEN["fixtures/results/semantic_contract_v0_3_load_reference_source_1.json"]
    table = load_reference_source_table()
    assert table["semantic_contract_id"] == LOAD_REFERENCE_SOURCE_CONTRACT_ID
    assert table["formulation_profile_id"] == LOAD_REFERENCE_SOURCE_PROFILE
    assert table["source_block_policy"] == POLICY
    # Result rows use the physics-source-1 signatures unchanged.
    physics_source = json.loads((PROJECT / "fixtures/results/semantic_contract_v0_3_physics_source_1.json").read_text())
    assert table["rows"] == physics_source["rows"] and table["canonical_metadata_vocabulary"] == physics_source["canonical_metadata_vocabulary"]
    for name in JOINED:
        raw = source(name)
        assert raw["producer"]["semantic_contract_id"] == LOAD_REFERENCE_SOURCE_CONTRACT_ID and raw["source_block_recovery"]["body"]["policy"] == POLICY


@pytest.mark.parametrize("case", CASES["cases"], ids=lambda case: case["id"])
def test_shared_adversarial_case(case):
    doc = apply(source(case["source"]), case["ops"])
    dispatch = outcome(lambda: _source_contract(doc))
    joined = outcome(lambda: validate_load_reference_source_evidence(doc))
    entry = {"id": case["id"], "dispatch": dispatch, "joined": joined}
    if "lr" in case:
        entry["lr"] = outcome(lambda: validate_load_reference_evidence(doc))
    if "ps" in case:
        entry["ps"] = outcome(lambda: validate_physics_source(doc))
    RESULTS.append(entry)
    assert agrees(dispatch, expected(case, "dispatch")), (dispatch, expected(case, "dispatch"))
    assert agrees(joined, expected(case, "joined")), (joined, expected(case, "joined"))
    for kind in ("lr", "ps"):
        if kind in case:
            assert entry[kind] == expected(case, kind), (kind, entry[kind], expected(case, kind))
    contract = case.get("accept_contract", LOAD_REFERENCE_SOURCE_CONTRACT_ID)
    if dispatch == "accept":
        assert _source_contract(doc)[0] == contract
        record = build_analysis_run(doc, input_manifest_ref=MANIFEST, input_manifest_hash="1" * 64)
        validate_analysis_run_v0_3(record, doc)
        run = record["analysis_run"]
        if contract == LOAD_REFERENCE_SOURCE_CONTRACT_ID:
            # Mirrors physics-source-1: both retained namespaces, never eligible.
            assert run["source_block_recovery"] == doc["source_block_recovery"] and run["contract_evidence"] == doc["contract_evidence"]
        else:
            assert "contract_evidence" not in run and "source_block_recovery" not in run
        assert numerical_use_standing(doc, bases(doc)) == "needs_recompute"
    else:
        with pytest.raises(ValueError):
            build_analysis_run(doc, input_manifest_ref=MANIFEST, input_manifest_hash="1" * 64)
        assert numerical_use_standing(doc, bases(doc)) == "unsupported"


@pytest.mark.parametrize("case", CASES["table_cases"], ids=lambda case: case["id"])
def test_table_bytes_are_pinned(case):
    data = (PROJECT / case.get("file", "fixtures/results/semantic_contract_v0_3_load_reference_source_1.json")).read_text()
    if case["find"] is not None:
        assert case["find"] in data
        data = data.replace(case["find"], case["replace"], 1)
    result = outcome(lambda: verify_table(data.encode()))
    RESULTS.append({"id": case["id"], "table": result})
    assert result == case["expect"]


@pytest.mark.parametrize("case", CASES["transport_cases"], ids=lambda case: case["id"])
def test_transport_metadata_case(case):
    meta = metadata(apply(source(case["source"]), case["ops"]))
    result = outcome(lambda: validate_load_reference_source_transport_metadata(meta))
    RESULTS.append({"id": case["id"], "transport": result})
    assert agrees(result, case["expect"]), result
    if result == "accept":
        assert _source_contract(meta, check_receipt=False)[0] == LOAD_REFERENCE_SOURCE_CONTRACT_ID


def test_language_specific_cases_are_declared():
    special = [case for case in CASES["cases"] if any(key.endswith(("_rust", "_python")) for key in case)]
    assert {case["id"] for case in special} == {"NUM-NONFINITE-member-E", "NUM-NONFINITE-region-pressure", "NUM-N2-integer-overflow-positive", "NUM-N2-integer-first-overflow"}
    assert all(case.get("note") for case in special)


def test_existing_readers_refuse_every_joined_envelope_under_their_own_labels():
    for name in JOINED:
        raw = source(name)
        assert outcome(lambda: validate_load_reference_evidence(raw)) == "SOURCE_LOAD_REFERENCE_FOREIGN_METHOD_EVIDENCE"
        assert outcome(lambda: validate_physics_source(raw)) == "PHYSICS_SOURCE_RECEIPT_SHAPE"
        as_lr = deepcopy(raw)
        as_lr["producer"]["semantic_contract_id"] = LOAD_REFERENCE_CONTRACT_ID
        as_lr["formulation_basis"]["profile_id"] = "resolved_straight_load_state_v1"
        assert outcome(lambda: _source_contract(as_lr)) == "SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN"
        as_ps = deepcopy(raw)
        as_ps["producer"]["semantic_contract_id"] = "openpipestress.result_semantics/0.3.0/physics-source-1"
        as_ps["formulation_basis"]["profile_id"] = "exact_straight_pressure_v2"
        assert outcome(lambda: _source_contract(as_ps)) == "SOURCE_LOAD_REFERENCE_EVIDENCE_FORBIDDEN"


def _carrier_path(name: str, kind: str) -> Path:
    stem, mode = name.rsplit("-", 1)
    return PROJECT / f"fixtures/results/load_reference_source_{stem}_{mode}.{kind}.json"


def _carrier_bytes(value) -> bytes:
    """Compact bytes plus a newline: the carriers are large and regenerated by rerun."""
    return (json.dumps(value, separators=(",", ":"), ensure_ascii=False) + "\n").encode()


@pytest.mark.parametrize("name", JOINED)
def test_analysis_run_carrier_mirrors_physics_source_and_is_reproduced_byte_for_byte(name):
    raw = source(name)
    before = deepcopy(raw)
    assert _source_contract(raw) == (LOAD_REFERENCE_SOURCE_CONTRACT_ID, LOAD_REFERENCE_SOURCE_CONTRACT_SHA256, TABLE_PATH)
    assert numerical_use_standing(raw, bases(raw)) == "needs_recompute"
    record = build_analysis_run_v0_3(raw, input_manifest_ref={"object_type": "InputManifest", "ref": f"manifest:load-reference-source-{name}"}, input_manifest_hash="1" * 64)
    assert raw == before
    data = _carrier_bytes(record)
    path = _carrier_path(name, "analysis_run")
    if os.environ.get("LOAD_REFERENCE_SOURCE_WRITE_FIXTURES") == "1":
        path.write_bytes(data)
    assert path.read_bytes() == data
    stored = json.loads(path.read_text())
    validate_analysis_run_v0_3(stored, raw)
    assert verify_analysis_run_record(stored) == "match"
    run = stored["analysis_run"]
    assert run["reproducibility"]["semantic_contract"] == {"id": LOAD_REFERENCE_SOURCE_CONTRACT_ID, "sha256": LOAD_REFERENCE_SOURCE_CONTRACT_SHA256}
    assert run["source_block_recovery"] == raw["source_block_recovery"] and run["contract_evidence"] == raw["contract_evidence"]
    assert len(run["result_refs"]) == len(raw["results"])
    assert all(ref["interpretation"]["status"] != "unavailable" for ref in run["result_refs"] if ref["source_annotation"]["kind"].endswith("_v2"))
    received = next(item for item in run["hashes"] if item["payload_scope"] == "received_result")
    assert received["value"] == canonical_sha256_checked_v1(raw)
    # Records bound to other evidence, receipts or tables are refused.
    for key, mutate, code in [
        ("contract_evidence", lambda r: r["analysis_run"].pop("contract_evidence"), "ANALYSIS_PHYSICS_SOURCE_EVIDENCE_MISMATCH"),
        ("contract_evidence", lambda r: r["analysis_run"]["contract_evidence"].pop("load_reference_states"), "ANALYSIS_PHYSICS_SOURCE_EVIDENCE_MISMATCH"),
        ("source_block_recovery", lambda r: r["analysis_run"].pop("source_block_recovery"), "ANALYSIS_SOURCE_BLOCK_RECEIPT_MISMATCH"),
        ("semantic_contract", lambda r: r["analysis_run"]["reproducibility"]["semantic_contract"].update(sha256="0" * 64), "ANALYSIS_SEMANTIC_CONTRACT_MISMATCH"),
    ]:
        forged = deepcopy(stored)
        mutate(forged)
        with pytest.raises(ValueError, match=code):
            validate_analysis_run_v0_3(forged, raw)


@pytest.mark.parametrize("name", ["lr-fallback-sparse", "lr-fallback-dense"])
def test_fallback_envelopes_remain_ordinary_load_reference_one(name):
    raw = source(name)
    assert _source_contract(raw)[0] == LOAD_REFERENCE_CONTRACT_ID
    assert "source_block_recovery" not in raw
    records = raw["contract_evidence"]["load_reference_states"]
    assert all(r["source_recovery"] == {"status": "not_joined", "code": "LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED"} for r in records)
    assert all(r["solve"]["recovery_method"].startswith("ordinary_") for r in records)
    assert any(d["code"] == "SOURCE_BLOCK_RECOVERY_UNAVAILABLE" for d in raw["diagnostics"])
    # The ordinary sensitive response is inspectable but not eligible.
    assert raw["numerical_quality"]["status"] == "sensitive"
    assert numerical_use_standing(raw, bases(raw)) == "needs_recompute"
    record = build_analysis_run_v0_3(raw, input_manifest_ref=MANIFEST, input_manifest_hash="1" * 64)
    assert "contract_evidence" not in record["analysis_run"] and "source_block_recovery" not in record["analysis_run"]


def test_zzz_write_parity_log():
    """Evidence only: the per-case outcomes for the return record."""
    folder = os.environ.get("LOAD_REFERENCE_SOURCE_PARITY_OUT")
    if not folder:
        pytest.skip("set LOAD_REFERENCE_SOURCE_PARITY_OUT to record per-case outcomes")
    Path(folder).mkdir(parents=True, exist_ok=True)
    (Path(folder) / "python_outcomes.json").write_text(json.dumps(RESULTS, indent=1) + "\n")
