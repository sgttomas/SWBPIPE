"""DEL-05-03 Design pf-timoshenko.md §4: the pressure-free Timoshenko identity beside preview-physics-2.

The Python mirrors of the readers (`tests/verifiers`) and the product's Rust-routed reading
(`core/analysis_runs/compatibility.py`, the AnalysisRun builder and the stress-neutral package):

- model-free, either complete identity (identity, profile and limitation array together) is admitted and a mixed
  array is refused (`FORMULATION_BASIS`, or the transport reader's "formulation profile or limitations");
- model-bound, a crossed pair (a Timoshenko identity on a model without the selection, or preview-physics-2 on a
  model with it) is refused by name;
- bindings are keyed by `semantic_contract_id`; preview-physics-2's table bytes and hash are unchanged, so existing
  records and fixtures stay compatible.

Until the product publishes one, the Timoshenko sources here are TRANSFORMED STAND-INS: committed Euler-Bernoulli
producer envelopes with the identity, the profile and limitation 0 swapped (`timoshenko_of`). Their rows are
Euler-Bernoulli values; these tests read identity, not mechanics.
"""
from __future__ import annotations

from copy import deepcopy
import hashlib
import json
from pathlib import Path

import pytest

from tests.verifiers import compatibility as vc
from tests.verifiers import preview_physics_evidence as ppe
from tests.verifiers import reader_verdict as rv

PROJECT = Path(__file__).resolve().parents[1]
RESULTS = PROJECT / "fixtures/results"
TIMOSHENKO_ID = "openpipestress.result_semantics/0.3.0/preview-physics-timoshenko-1"
TIMOSHENKO_PROFILE = "product_preview_mechanics_timoshenko_v1"
TIMOSHENKO_SHA256 = "94538daa46b4795550a8c9e6b4ec05d8eb40d5be34b78c1a630613dc7ae2732f"
PREVIEW_SHA256 = "5a6630d9157ad1f4232c558d0cd57414677add8d1c95cc0a5d19a2579fb3a05b"
SOURCES = {
    "connected_sparse": "fixtures/model_operations/precision_connected_ui_model.json",
    "connected_dense": "fixtures/model_operations/precision_connected_ui_model.json",
    "invented_sparse": "fixtures/product_preview/invented_demo_model.json",
    "invented_dense": "fixtures/product_preview/invented_demo_model.json",
}


def _json(path: Path):
    return json.loads(path.read_text(encoding="utf-8"))


def eb(name: str) -> dict:
    return _json(RESULTS / f"preview_physics_{name}.json")


def timoshenko_of(source: dict) -> dict:
    """A transformed stand-in: the Euler-Bernoulli envelope under the Timoshenko identity."""
    out = deepcopy(source)
    out["producer"]["semantic_contract_id"] = TIMOSHENKO_ID
    out["formulation_basis"] = {"profile_id": TIMOSHENKO_PROFILE, "limitations": list(ppe.TIMOSHENKO_LIMITATIONS)}
    return out


def model(name: str, shear_model: str | None = None) -> dict:
    out = _json(PROJECT / SOURCES[name])
    if shear_model is not None:
        out["model_settings"] = {"shear_model": shear_model}
    return out


def test_tables_and_pins():
    assert hashlib.sha256((RESULTS / "semantic_contract_v0_3_preview_physics_timoshenko_1.json").read_bytes()).hexdigest() == TIMOSHENKO_SHA256
    assert hashlib.sha256((RESULTS / "semantic_contract_v0_3_preview_physics_2.json").read_bytes()).hexdigest() == PREVIEW_SHA256
    assert vc.PREVIEW_PHYSICS_CONTRACTS[TIMOSHENKO_ID][:2] == (TIMOSHENKO_SHA256, RESULTS / "semantic_contract_v0_3_preview_physics_timoshenko_1.json")
    assert vc.PREVIEW_PHYSICS_CONTRACT_SHA256 == PREVIEW_SHA256
    assert ppe.TIMOSHENKO_LIMITATIONS[1:] == ppe.LIMITATIONS[1:] and ppe.TIMOSHENKO_LIMITATIONS[0] != ppe.LIMITATIONS[0]
    assert TIMOSHENKO_ID in vc.FRESH_CONTRACT_IDS
    from core.analysis_runs import compatibility as product
    assert TIMOSHENKO_ID in product.FRESH_CONTRACT_IDS and TIMOSHENKO_ID in product.CURRENT_RECORD_CONTRACT_IDS
    assert product._TABLES[TIMOSHENKO_ID][0] == TIMOSHENKO_SHA256
    assert product._TABLES[product.PREVIEW_PHYSICS_CONTRACT_ID][0] == PREVIEW_SHA256
    from core.handoff.stress_neutral import package
    assert {product.PREVIEW_PHYSICS_CONTRACT_ID, TIMOSHENKO_ID} <= package.SUPPORTED_METHODS
    assert package._method_namespaces(TIMOSHENKO_ID) == package._method_namespaces(product.PREVIEW_PHYSICS_CONTRACT_ID) == {"contract_evidence"}


def test_schemas_admit_both_identities_and_hashes():
    analysis = _json(PROJECT / "schemas/analysis_run.v0.3.schema.json")
    pairs = [(b["properties"]["id"]["const"], b["properties"]["sha256"]["const"]) for b in analysis["$defs"]["SemanticContract"]["oneOf"]]
    assert (vc.PREVIEW_PHYSICS_CONTRACT_ID, PREVIEW_SHA256) in pairs and (TIMOSHENKO_ID, TIMOSHENKO_SHA256) in pairs
    neutral = _json(PROJECT / "schemas/stress_neutral_export.v0.3.schema.json")
    consts = [b["properties"]["semantic_contract"]["properties"] for b in neutral["oneOf"] if "semantic_contract" in b.get("properties", {})]
    assert {"id": {"const": TIMOSHENKO_ID}, "sha256": {"const": TIMOSHENKO_SHA256}} in consts
    assert {"id": {"const": vc.PREVIEW_PHYSICS_CONTRACT_ID}, "sha256": {"const": PREVIEW_SHA256}} in consts


@pytest.mark.parametrize("name", list(SOURCES))
def test_verifier_model_free_readers_admit_either_complete_identity(name):
    for source, identity, sha in ((eb(name), vc.PREVIEW_PHYSICS_CONTRACT_ID, PREVIEW_SHA256),
                                  (timoshenko_of(eb(name)), TIMOSHENKO_ID, TIMOSHENKO_SHA256)):
        ppe.validate_preview_physics_evidence(source)
        ppe.validate_transport_metadata(source)
        assert vc._source_contract(source)[:2] == (identity, sha)
        assert vc._source_contract(source, check_receipt=False)[:2] == (identity, sha)
        assert rv.validate("preview-physics", source) == {"ok": True}
        assert rv.transport("preview-physics", source) == {"ok": True}
        dispatch = rv.dispatch(source)
        assert (dispatch["route"], dispatch["binding"]) == ("preview_physics", {"id": identity, "sha256": sha})


def _mixed():
    base = eb("connected_sparse")
    ti = timoshenko_of(base)
    cases = []
    m = deepcopy(ti); m["formulation_basis"]["profile_id"] = "product_preview_mechanics_v1"; cases.append(("ti id, eb profile", m))
    m = deepcopy(ti); m["formulation_basis"]["limitations"] = list(ppe.LIMITATIONS); cases.append(("ti id, eb limitations", m))
    m = deepcopy(ti); m["formulation_basis"]["limitations"][0] = ppe.LIMITATIONS[0]; cases.append(("ti id, eb limitation 0", m))
    m = deepcopy(base); m["formulation_basis"]["profile_id"] = TIMOSHENKO_PROFILE; cases.append(("eb id, ti profile", m))
    m = deepcopy(base); m["formulation_basis"]["limitations"] = list(ppe.TIMOSHENKO_LIMITATIONS); cases.append(("eb id, ti limitations", m))
    m = deepcopy(base); m["formulation_basis"] = {"profile_id": TIMOSHENKO_PROFILE, "limitations": list(ppe.TIMOSHENKO_LIMITATIONS)}
    cases.append(("eb id, ti profile and limitations", m))
    return cases


@pytest.mark.parametrize("label,source", _mixed(), ids=[c[0] for c in _mixed()])
def test_verifier_refuses_a_mixed_identity(label, source):
    with pytest.raises(ValueError, match="formulation profile or limitations"):
        ppe.validate_preview_physics_evidence(source)
    with pytest.raises(ValueError, match="formulation profile or limitations"):
        ppe.validate_transport_metadata(source)
    with pytest.raises(ValueError):
        vc._source_contract(source)
    assert rv.validate("preview-physics", source)["ok"] is False


@pytest.mark.parametrize("name", list(SOURCES))
def test_verifier_model_bound_reader_refuses_a_crossed_pair(name):
    base, ti = eb(name), timoshenko_of(eb(name))
    absent, explicit, selected = model(name), model(name, "euler_bernoulli"), model(name, "timoshenko_annulus_energy_v1")
    assert rv.validate("preview-physics", base, absent) == {"ok": True}
    assert rv.validate("preview-physics", base, explicit) == {"ok": True}
    assert rv.validate("preview-physics", ti, selected) == {"ok": True}
    for source, bound in ((ti, absent), (ti, explicit), (base, selected), (base, model(name, "cowper"))):
        outcome = rv.validate("preview-physics", source, bound)
        assert outcome["error"]["code"] == rv.SHEAR_MODEL_IDENTITY, outcome
        standing = rv.standing(source, bound)
        assert standing["eligible"] is False and standing["findings"][0].startswith(rv.SHEAR_MODEL_IDENTITY)
    assert rv.standing(ti, selected)["findings"] == []


# The product's Rust-routed reading: these need the runner binary (skipped without it, conftest).

@pytest.mark.parametrize("name", list(SOURCES))
def test_product_reads_both_identities_and_refuses_crossed_pairs(name):
    from core.analysis_runs import compatibility as product, rust_reader
    for source, identity, sha in ((eb(name), product.PREVIEW_PHYSICS_CONTRACT_ID, PREVIEW_SHA256),
                                  (timoshenko_of(eb(name)), TIMOSHENKO_ID, TIMOSHENKO_SHA256)):
        assert product._source_contract(source)[:2] == (identity, sha)
        assert product._source_contract(source, check_receipt=False)[:2] == (identity, sha)
        assert rust_reader.dispatch(source)["binding"] == {"id": identity, "sha256": sha}
        bases = [case["basis_ref"] for case in source["numerical_quality"]["cases"]]
        assert product.numerical_use_standing(source, bases) == "numerically_eligible"
    for label, mixed in _mixed():
        with pytest.raises(ValueError):
            product._source_contract(mixed)


@pytest.mark.parametrize("name", ["connected_sparse", "invented_dense"])
def test_product_analysis_run_records_the_timoshenko_binding(name):
    from core.analysis_runs.compatibility import build_analysis_run_v0_3, validate_analysis_run_v0_3
    from tests.test_preview_physics_consumer_contract import build
    ti = timoshenko_of(eb(name))
    record = build(ti)
    assert record["analysis_run"]["reproducibility"]["semantic_contract"] == {"id": TIMOSHENKO_ID, "sha256": TIMOSHENKO_SHA256}
    assert {ref["semantic_contract"]["id"] for ref in record["analysis_run"]["result_refs"]} == {TIMOSHENKO_ID}
    validate_analysis_run_v0_3(record, ti)
    # The record binds its identity: it does not validate against the Euler-Bernoulli source.
    with pytest.raises(ValueError):
        validate_analysis_run_v0_3(record, eb(name))
    assert build_analysis_run_v0_3 is not None
