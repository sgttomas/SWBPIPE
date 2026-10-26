"""pressure-2 readers on load/reference-state publications: shared Rust/Python adversarial parity and carriers.

B7b (A4/A5) retired load-reference-1 (`resolved_straight_load_state_v1` under `2.0.0/exact_straight_pressure_v2`):
a load/reference-state model now publishes pressure-2 (`3.0.0/exact_pressure_v3`) with
`contract_evidence.load_reference_states`. The case file is shared with the Rust reader test
``core/reporting/result_export/tests/load_reference_contract.rs``, whose v3 twins it holds: ``dispatch`` is the raw
dispatch (Rust ``semantic_contract::for_source``; here the verifier's ``_source_contract``) and ``validator`` the
pressure-2 reader called directly (``validate_pressure_evidence``). Both readers must reproduce every expectation,
so they accept and reject the same inputs with the same error. pressure-2 has no transport (metadata-only) reader,
so the case file holds no transport cases. Carrier fixtures ``fixtures/results/pressure_load_state_*`` are derived
from the four current producer envelopes by the product builder; they prove reader preservation, not producer
authentication or engineering acceptance.

b7-plan R2: the Python reader is the test verifier (``tests/verifiers``), which these cases check against the
Python expectations; product Python reads through the Rust reader, whose text ``test_base_reader_verifier.py``
checks on this corpus.
"""
from __future__ import annotations

from copy import deepcopy
import hashlib
import json
import os
from pathlib import Path

import pytest

from core.analysis_runs.compatibility import (
    PRESSURE_CONTRACT_ID, PRESSURE_CONTRACT_SHA256, _source_contract, build_analysis_run, build_analysis_run_v0_3,
    numerical_use_standing, validate_analysis_run_v0_3, verify_analysis_run_record,
)
from core.analysis_runs import rust_reader
from tests.verifiers.load_reference_evidence import TRANSPORT_SCHEMA_SHA256
from tests.verifiers.pressure_2 import pressure_table, validate_pressure_evidence, verify_pressure_table
from tests.verifiers import compatibility as verifier
from core.serialization.canonical_json.adapter import canonical_sha256_checked_v1

PROJECT = Path(__file__).resolve().parents[1]
CASES_PATH = PROJECT / "core/reporting/result_export/tests/fixtures/load_reference_mutations.json"
CASES = json.loads(CASES_PATH.read_text())
FROZEN = {
    "fixtures/product_preview/pressure/load_reference/connected.request.json": "43989de7b7bb11688daac07f5cfe9564bbfaa29e99d80f02ea7746a270b44de0",
    # T4-U6 (D-5): the regenerated producer envelopes (both section-basis families);
    # O4 (DEL-04-08 `o4-source-extension.md` §3.3): regenerated for limitation [7] only.
    # connected: silent-zero diagnostic added; no number moved (two LOAD_STATE_SOURCE_EXCLUDED warnings).
    # W-7 global end rows added; no other number moved (connected +48 rows, pressure +24 rows).
    # W-15 (DEL-03-05): regenerated for the pressure-2 identity limitation (identity bytes only).
    # W-16 slice c (DEL-05-01): regenerated for the applied-loads limitation's equivalent-static refusal sentence only.
    "fixtures/product_preview/pressure/load_reference/connected-sparse_interactive.raw.json": "3cb90d4c96548501867d16c02bbd00fee1b789ae16e871bae8d00adddb96a1c1",
    "fixtures/product_preview/pressure/load_reference/connected-dense_scrutiny.raw.json": "f5a7cafc3a979de965161e6c00950bb66cc1a6adddf43d7928a071431a274576",
    "fixtures/product_preview/pressure/load_reference/pressure.request.json": "a187c0949ce6fad0f357f9439daf95b63886be582f95ebee4181d327da882eca",
    "fixtures/product_preview/pressure/load_reference/pressure-sparse_interactive.raw.json": "eec9a1808994f7f3fd92c433cee4ba972d5e707b9bc3fc604d32062da7f200dd",
    "fixtures/product_preview/pressure/load_reference/pressure-dense_scrutiny.raw.json": "8fd9abc53e39825dccc3b4f0dcb8474de518911ca238a04dc89eea5f5adaf329",
    "fixtures/results/semantic_contract_v0_3_pressure_2.json": PRESSURE_CONTRACT_SHA256,
    "schemas/load_reference_state.schema.json": TRANSPORT_SCHEMA_SHA256,
}
RAW = ["connected-sparse", "connected-dense", "pressure-sparse", "pressure-dense"]
# The current producer envelopes the carriers derive from (and the case file's sources of the same names).
RAW_PATHS = {f"{stem}-{short}": f"fixtures/product_preview/pressure/load_reference/{stem}-{mode}.raw.json"
             for stem in ("connected", "pressure") for short, mode in (("sparse", "sparse_interactive"), ("dense", "dense_scrutiny"))}
PHYSICS_PREFIX = "SOURCE_LOAD_REFERENCE_PHYSICS_EVIDENCE"
# The declared class (as in test_base_reader_verifier): both readers refuse, each in its own words. B7b reads these
# statements with the ordinary pressure-2 reader, whose Python words are a generic code with a detail where the Rust
# reader (the shared case's expectation, checked below against the product verdict) names a specific code.
DECLARED_WORDS = {
    "MISSING-load-reference-states": "SOURCE_PHYSICS_EVIDENCE_INVALID: material shape",
    "MISSING-contract-evidence": "SOURCE_PHYSICS_EVIDENCE_INVALID: namespace",
    "CLOSED-connector": f"{PHYSICS_PREFIX}: SOURCE_PHYSICS_EVIDENCE_INVALID: connector record shape",
}
RESULTS: list[dict] = []


def source(name: str) -> dict:
    return json.loads((PROJECT / CASES["sources"][name]).read_text())


def actual(name: str) -> dict:
    return json.loads((PROJECT / RAW_PATHS[name]).read_text())


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


def build(doc, ref="manifest:load-reference-reader-test"):
    return build_analysis_run(doc, input_manifest_ref={"object_type": "InputManifest", "ref": ref}, input_manifest_hash="1" * 64)


def test_frozen_inputs_and_table_identity():
    for path, digest in FROZEN.items():
        assert hashlib.sha256((PROJECT / path).read_bytes()).hexdigest() == digest, path
    table = pressure_table()
    assert table["semantic_contract_id"] == PRESSURE_CONTRACT_ID and table["formulation_profile_id"] == "exact_pressure_v3"
    # B5 (last entry, B7b): pressure-2 is published, so no current table reserves it.
    assert table["reserved_inactive_successors"] == []
    # The retired load-reference-1 table left with v2; no current table carries its profile.
    assert not (PROJECT / "fixtures/results/semantic_contract_v0_3_load_reference_1.json").exists()
    for raw in RAW:
        assert CASES["sources"][raw] == RAW_PATHS[raw]
        assert actual(raw)["producer"]["semantic_contract_id"] == PRESSURE_CONTRACT_ID
        assert "load_reference_states" in actual(raw)["contract_evidence"]


@pytest.mark.parametrize("case", CASES["cases"], ids=lambda case: case["id"])
def test_shared_adversarial_case(case):
    doc = apply(source(case["source"]), case["ops"])
    # The verifier's dispatch against the Python expectation; the product (Rust) admits exactly the same statements.
    dispatch = outcome(lambda: verifier._source_contract(doc))
    assert (outcome(lambda: _source_contract(doc)) == "accept") == (dispatch == "accept")
    validator = outcome(lambda: validate_pressure_evidence(doc))
    RESULTS.append({"id": case["id"], "dispatch": dispatch, "validator": validator})
    if case["id"] in DECLARED_WORDS:
        # The verifier's own words, pinned here; the product (Rust) verdict carries the shared expectation.
        assert not any(key.endswith("_python") for key in case), case["id"]
        assert dispatch == validator == DECLARED_WORDS[case["id"]], (dispatch, validator)
        # The Rust pressure reader (the product's `read pressure`) names the shared expectation's code.
        product = outcome(lambda: rust_reader.validate("pressure", doc))
        assert agrees(product, case["dispatch"]), (product, case["dispatch"])
    else:
        assert agrees(dispatch, expected(case, "dispatch")), (dispatch, expected(case, "dispatch"))
        assert agrees(validator, expected(case, "validator")), (validator, expected(case, "validator"))
    if dispatch == "accept" and "carrier_python" in case:
        # Both readers admit it; the checked-JSON carrier profile does not.
        assert _source_contract(doc)[0] == PRESSURE_CONTRACT_ID
        assert outcome(lambda: build(doc)) == case["carrier_python"]
        assert numerical_use_standing(doc, bases(doc)) == "numerically_eligible"
    elif dispatch == "accept":
        assert _source_contract(doc)[0] == PRESSURE_CONTRACT_ID
        record = build(doc)
        validate_analysis_run_v0_3(record, doc)
        assert "contract_evidence" not in record["analysis_run"] and "source_block_recovery" not in record["analysis_run"]
    else:
        with pytest.raises(ValueError):
            build(doc)
        assert numerical_use_standing(doc, bases(doc)) == "unsupported"


@pytest.mark.parametrize("case", CASES["table_cases"], ids=lambda case: case["id"])
def test_table_bytes_are_pinned(case):
    data = (PROJECT / "fixtures/results/semantic_contract_v0_3_pressure_2.json").read_text()
    if case["find"] is not None:
        assert case["find"] in data
        data = data.replace(case["find"], case["replace"], 1)
    result = outcome(lambda: verify_pressure_table(data.encode()))
    RESULTS.append({"id": case["id"], "table": result})
    assert result == case["expect"]


def test_pressure_one_has_no_transport_cases():
    # pressure-2 has no transport (metadata-only) reader in Rust; a transported statement is refused by name, in
    # the product (through Rust) and in the verifier alike.
    assert CASES["transport_cases"] == []
    raw = actual("connected-sparse")
    meta = {"schema_version": raw["schema_version"], **{key: raw[key] for key in ("producer", "numerical_quality", "formulation_basis", "contract_evidence")}}
    assert outcome(lambda: _source_contract(meta, check_receipt=False)) == "SOURCE_PRESSURE_TRANSPORT_UNSUPPORTED"
    assert outcome(lambda: verifier._source_contract(meta, check_receipt=False)) == "SOURCE_PRESSURE_TRANSPORT_UNSUPPORTED"


def test_the_declared_words_cases_are_shared_cases_refused_by_both_readers():
    ids = {case["id"]: case for case in CASES["cases"]}
    assert set(DECLARED_WORDS) <= set(ids)
    assert all(ids[name].get("dispatch") not in (None, "accept") for name in DECLARED_WORDS)


def test_nonfinite_cases_are_the_only_language_specific_representation():
    special = [case for case in CASES["cases"] if "dispatch_rust" in case or "dispatch_python" in case]
    assert {case["id"] for case in special} == {"NONFINITE-member-E", "NONFINITE-factor", "NONFINITE-row-value", "NONFINITE-region-pressure", "N2-integer-overflow-positive", "N2-integer-overflow-negative", "N2-integer-first-overflow"}
    assert all(case.get("note") for case in special)


@pytest.mark.parametrize("name", RAW)
def test_actual_envelopes_admit_numerically_and_build_pressure_analysis_run(name):
    raw = actual(name)
    before = deepcopy(raw)
    assert _source_contract(raw) == (PRESSURE_CONTRACT_ID, PRESSURE_CONTRACT_SHA256, PROJECT / "fixtures/results/semantic_contract_v0_3_pressure_2.json")
    assert numerical_use_standing(raw, bases(raw)) == "numerically_eligible"
    record = build_analysis_run_v0_3(raw, input_manifest_ref={"object_type": "InputManifest", "ref": "manifest:load-reference-reader-test"}, input_manifest_hash="1" * 64)
    assert raw == before
    assert verify_analysis_run_record(record) == "match"
    run = record["analysis_run"]
    assert run["reproducibility"]["semantic_contract"] == {"id": PRESSURE_CONTRACT_ID, "sha256": PRESSURE_CONTRACT_SHA256}
    assert "contract_evidence" not in run and "source_block_recovery" not in run
    assert len(run["result_refs"]) == len(raw["results"])
    assert all(ref["interpretation"]["status"] != "unavailable" for ref in run["result_refs"] if ref["source_annotation"]["kind"].endswith("_v2"))
    received = next(item for item in run["hashes"] if item["payload_scope"] == "received_result")
    assert received["value"] == canonical_sha256_checked_v1(raw)
    # A downgraded record (carrying evidence a pressure-2 record does not carry) is refused.
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
    return PROJECT / f"fixtures/results/pressure_load_state_{name.replace('-', '_')}.analysis_run.json"


def _carrier_bytes(value) -> bytes:
    """Compact bytes plus a newline: the carriers are large and regenerated by rerun."""
    return (json.dumps(value, separators=(",", ":"), ensure_ascii=False) + "\n").encode()


@pytest.mark.parametrize("name", RAW)
def test_analysis_run_carrier_fixture_is_reproduced_byte_for_byte(name):
    raw = actual(name)
    # The invented manifest reference keeps its pre-B7b text, so that only the declared classes move.
    record = build_analysis_run_v0_3(raw, input_manifest_ref={"object_type": "InputManifest", "ref": f"manifest:load-reference-{name}"}, input_manifest_hash="1" * 64)
    data = _carrier_bytes(record)
    path = _analysis_run_fixture(name)
    if os.environ.get("LOAD_REFERENCE_WRITE_FIXTURES") == "1":
        path.write_bytes(data)
    assert path.read_bytes() == data
    stored = json.loads(path.read_text())
    validate_analysis_run_v0_3(stored, raw)
    assert verify_analysis_run_record(stored) == "match"


def test_retired_load_reference_identity_is_refused_by_name():
    """B7b: a stored load-reference-1 (v2) result is refused at the header by name, in the product (Rust) and the
    verifier; nothing reinterprets it as pressure-2."""
    raw = actual("connected-sparse")
    stored = deepcopy(raw)
    stored["producer"]["semantic_contract_id"] = "openpipestress.result_semantics/0.3.0/load-reference-1"
    stored["formulation_basis"]["profile_id"] = "resolved_straight_load_state_v1"
    text = ("SOURCE_EXACT_PRESSURE_V2_RETIRED: 2.0.0/exact_straight_pressure_v2 results are no longer read; "
            "re-author the model as 0.7.0 with the current exact pressure contract and re-run the analysis")
    assert outcome(lambda: _source_contract(stored)) == text
    assert outcome(lambda: verifier._source_contract(stored)) == text
    assert numerical_use_standing(stored, bases(stored)) == "unsupported"
    with pytest.raises(ValueError, match="SOURCE_EXACT_PRESSURE_V2_RETIRED"):
        build(stored)


def test_zzz_write_parity_log():
    """Evidence only: the per-case outcomes for the return record."""
    folder = os.environ.get("LOAD_REFERENCE_PARITY_OUT")
    if not folder:
        pytest.skip("set LOAD_REFERENCE_PARITY_OUT to record per-case outcomes")
    Path(folder).mkdir(parents=True, exist_ok=True)
    (Path(folder) / "python_outcomes.json").write_text(json.dumps(RESULTS, indent=1) + "\n")
