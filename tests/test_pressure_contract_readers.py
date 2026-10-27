"""pressure-2 straight families: the test-only verifier and the product refusal.

Reads the shared generated pressure-2 corpus (the Rust reader's
``tests/pressure_contract.rs`` reads the same bytes; the Rust reader is the
only product reader). The independent verifier (``tests/verifiers/pressure_2.py``)
admits v3 straight families with signed p_pa. B7b retired the v2 contract: no
v2 reader remains, evidence relabelled with the retired v2 profile is refused at
the current profile check, a statement under a retired v2 identity is refused by
name (``SOURCE_EXACT_PRESSURE_V2_RETIRED``) and unknown identities are refused.
Python product paths take the Rust reader's verdict (b7-plan R2) through its
pressure route: a raw statement is read, a transported one is refused by name
(there is no pressure-2 transport reader), and B7b admits a 0.3 AnalysisRun
record built from the raw pressure-2 statement.
"""
import ast
from copy import deepcopy
import json
from pathlib import Path

import pytest
from core.analysis_runs import rust_reader
from core.analysis_runs.compatibility import (
    PRESSURE_CONTRACT_ID, PRESSURE_CONTRACT_SHA256, _source_contract, build_analysis_run_v0_3, numerical_use_standing,
)
from tests.verifiers.pressure_2 import (
    CONTRACT_ID, RETIRED_EXACT_V2_IDS, SOURCE_EXACT_PRESSURE_V2_RETIRED, V2_RETIRED_TEXT,
    validate_exact_pressure_evidence, validate_pressure_evidence, verify_source,
)

PROJECT = Path(__file__).resolve().parents[1]
CORPUS = json.loads((PROJECT / "fixtures/results/pressure_v3_straight_reader_corpus.json").read_text())
CASES = [(case["label"], case["envelope"]) for case in CORPUS["cases"]]
VERIFIER = PROJECT / "tests/verifiers/pressure_2.py"
# b7-plan R2: Python product paths read through the Rust verdict, whose
# `pressure` route reads raw pressure-2 and refuses a transported one by name.
PRESSURE_TRANSPORT_REFUSAL = "SOURCE_PRESSURE_TRANSPORT_UNSUPPORTED"


def load_state(envelope):
    return "load_reference_states" in envelope["contract_evidence"]


def relabel(envelope, version, mode):
    out = deepcopy(envelope)
    for case in out["contract_evidence"]["exact_cases"]:
        case["profile_mode"] = mode
    for region in out["contract_evidence"]["pressure"]:
        region["profile_version"], region["profile_mode"] = version, mode
    return out


def test_corpus_shape():
    assert len(CASES) == 4 and {load_state(e) for _, e in CASES} == {True, False}
    assert all(r["p_pa"] < 0 for _, e in CASES if not load_state(e) for r in e["contract_evidence"]["pressure"])


@pytest.mark.parametrize("label,envelope", CASES)
def test_verifier_and_dispatch_admit_the_corpus(label, envelope):
    assert envelope["producer"]["semantic_contract_id"] == CONTRACT_ID
    validate_pressure_evidence(deepcopy(envelope))
    validate_exact_pressure_evidence(deepcopy(envelope))
    verify_source(deepcopy(envelope))
    verify_source(deepcopy(envelope), raw=False)


@pytest.mark.parametrize("label,envelope", CASES)
def test_python_product_paths_take_the_rust_verdict_of_pressure_1(label, envelope):
    contract_id, sha256, _ = _source_contract(deepcopy(envelope))
    assert (contract_id, sha256) == (PRESSURE_CONTRACT_ID, PRESSURE_CONTRACT_SHA256)
    with pytest.raises(ValueError, match=PRESSURE_TRANSPORT_REFUSAL):
        _source_contract(deepcopy(envelope), check_receipt=False)
    # B7b: pressure-2 is a current record contract; the record binds the pressure-2 table.
    record = build_analysis_run_v0_3(deepcopy(envelope), input_manifest_ref={"object_type": "InputManifest", "ref": "manifest:x"}, input_manifest_hash="1" * 64)
    refs = record["analysis_run"]["result_refs"]
    assert refs and {(r["semantic_contract"]["id"], r["semantic_contract"]["sha256"]) for r in refs} == {(PRESSURE_CONTRACT_ID, PRESSURE_CONTRACT_SHA256)}
    bases = [case["basis_ref"] for case in envelope["numerical_quality"]["cases"]]
    # One verdict and one standing: the Rust reader's pressure route.
    assert numerical_use_standing(deepcopy(envelope), bases) == rust_reader.standing(deepcopy(envelope), bases, None)["token"]
    model = {"load_cases": [{"id": basis["ref_id"]} for basis in bases]}
    dispatch = rust_reader.dispatch(deepcopy(envelope), model)
    assert (dispatch["ok"], dispatch["route"], dispatch["binding"]) == (True, "pressure", {"id": PRESSURE_CONTRACT_ID, "sha256": PRESSURE_CONTRACT_SHA256})
    assert dispatch["standing"]["route"] == "pressure"
    assert dispatch["standing"]["eligible"] == (envelope["numerical_quality"]["status"] == "checks_passed")
    assert rust_reader.validate("pressure", deepcopy(envelope), model=model) == {"ok": True}


@pytest.mark.parametrize("label,envelope", CASES)
def test_retired_v2_labels_are_refused(label, envelope):
    # B7b: no v2 reader remains (Rust `retired_v2_labels_are_refused`). Evidence relabelled with the retired v2
    # profile is refused at the current case-profile check; a retired v2 identity is refused by name.
    as_v2 = relabel(envelope, "2.0.0", "exact_straight_pressure_v2")
    with pytest.raises(ValueError, match="case profile"):
        validate_pressure_evidence(deepcopy(as_v2))
    assert len(RETIRED_EXACT_V2_IDS) == 4
    for identity in RETIRED_EXACT_V2_IDS:
        relabelled = deepcopy(envelope)
        relabelled["producer"]["semantic_contract_id"] = identity
        with pytest.raises(ValueError) as refused:
            validate_exact_pressure_evidence(relabelled)
        assert str(refused.value) == V2_RETIRED_TEXT
        assert str(refused.value).startswith(f"{SOURCE_EXACT_PRESSURE_V2_RETIRED}: ") and "re-author the model as 0.7.0" in str(refused.value)
        # The product paths take the Rust verdict: the same refusal, by name, at the header.
        for check_receipt in (True, False):
            with pytest.raises(ValueError) as refused:
                _source_contract(deepcopy(relabelled), check_receipt=check_receipt)
            assert str(refused.value).startswith(f"{SOURCE_EXACT_PRESSURE_V2_RETIRED}: "), str(refused.value)
        assert rust_reader.dispatch(deepcopy(relabelled), None)["ok"] is False


@pytest.mark.parametrize("label,envelope", CASES)
def test_unknown_identities_are_refused(label, envelope):
    unknown = deepcopy(envelope)
    unknown["producer"]["semantic_contract_id"] = "openpipestress.result_semantics/0.3.0/pressure-3"
    with pytest.raises(ValueError, match="SOURCE_PHYSICS_PRESSURE_CONTRACT_UNKNOWN"):
        validate_exact_pressure_evidence(unknown)
    with pytest.raises(ValueError, match="SOURCE_PRODUCER_CONTRACT_UNSUPPORTED"):
        verify_source(unknown)
    # Model 0.6.0: the retired pressure-1 identity is refused by name at the header (re-run).
    retired = deepcopy(envelope)
    retired["producer"]["semantic_contract_id"] = "openpipestress.result_semantics/0.3.0/pressure-1"
    message = "^SOURCE_PRESSURE_1_RETIRED: pressure-1 results are no longer read; re-run the analysis$"
    with pytest.raises(ValueError, match=message):
        validate_exact_pressure_evidence(retired)
    with pytest.raises(ValueError, match=message):
        _source_contract(retired)
    assert rust_reader.dispatch(retired)["error"]["code"] == "SOURCE_PRESSURE_1_RETIRED"
    # The retired preview-physics-1 identity is refused by name the same way (re-run).
    retired_preview = deepcopy(envelope)
    retired_preview["producer"]["semantic_contract_id"] = "openpipestress.result_semantics/0.3.0/preview-physics-1"
    message = "^SOURCE_PREVIEW_PHYSICS_1_RETIRED: preview-physics-1 results are no longer read; re-run the analysis$"
    with pytest.raises(ValueError, match=message):
        _source_contract(retired_preview)
    assert rust_reader.dispatch(retired_preview)["error"]["code"] == "SOURCE_PREVIEW_PHYSICS_1_RETIRED"
    profile = deepcopy(envelope)
    profile["formulation_basis"]["profile_id"] = "exact_straight_pressure_v2"
    with pytest.raises(ValueError, match="pressure-2 formulation profile"):
        validate_exact_pressure_evidence(profile)
    with pytest.raises(ValueError, match="SOURCE_FORMULATION_BASIS_UNSUPPORTED"):
        verify_source(profile)
    with pytest.raises(ValueError, match="case profile"):
        validate_pressure_evidence(relabel(envelope, "3.0.0", "exact_pressure_v4"))


@pytest.mark.parametrize("label,envelope", CASES)
def test_null_or_non_mapping_metadata_is_refused_by_name(label, envelope):
    # T4-RV13 N-1: a null producer or formulation basis is a named ValueError, never AttributeError.
    for producer in (None, [], "pressure-2"):
        broken = deepcopy(envelope)
        broken["producer"] = producer
        with pytest.raises(ValueError, match="SOURCE_PHYSICS_PRESSURE_CONTRACT_UNKNOWN"):
            validate_exact_pressure_evidence(broken)
    for basis in (None, [], "exact_pressure_v3"):
        broken = deepcopy(envelope)
        broken["formulation_basis"] = basis
        with pytest.raises(ValueError, match="pressure-2 formulation profile"):
            validate_exact_pressure_evidence(broken)


@pytest.mark.parametrize("label,envelope", CASES)
def test_header_refusals_are_named(label, envelope):
    for mutate, code in [
            (lambda e: e.__setitem__("schema_version", "0.1.0"), "SOURCE_SCHEMA_VERSION_UNSUPPORTED"),
            (lambda e: e.__setitem__("source_block_recovery", {}), "SOURCE_BLOCKS_DOWNGRADE_FORBIDDEN"),
            (lambda e: e.__setitem__("carrier_evidence", {}), "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED"),
            (lambda e: e.__setitem__("retained_precision", {}), "RETAINED_PRECISION_DOWNGRADE_FORBIDDEN"),
            (lambda e: e["results"][0].__setitem__("recovery_method", "contribution_preserving_multiprecision_v1"), "RETAINED_PRECISION_DOWNGRADE_FORBIDDEN"),
            (lambda e: e["numerical_quality"].__setitem__("status", ["checks_passed"]), "SOURCE_NUMERICAL_QUALITY_INVALID"),
            (lambda e: e["numerical_quality"]["cases"][0].__setitem__("solve_quality", "good"), "SOURCE_NUMERICAL_CASE_INVALID"),
            (lambda e: e["formulation_basis"].__setitem__("limitations", []), "SOURCE_FORMULATION_BASIS_UNSUPPORTED")]:
        broken = deepcopy(envelope)
        mutate(broken)
        with pytest.raises(ValueError, match=code):
            verify_source(broken)


def _imports(path: Path) -> set[str]:
    names = set()
    for node in ast.walk(ast.parse(path.read_text(), filename=str(path))):
        if isinstance(node, ast.Import):
            names.update(item.name for item in node.names)
        elif isinstance(node, ast.ImportFrom) and node.module is None:
            # `from . import name`: the imported sibling module is the name.
            names.update("." * node.level + item.name for item in node.names)
        elif isinstance(node, ast.ImportFrom):
            names.add("." * node.level + node.module)
    return names


def test_the_verifier_is_independent_and_test_only():
    # Standard library only: no product module and no producer bridge
    # (core/serialization/canonical_json/adapter.py, core/units/adapter.py).
    # T4-U5b (DEL-04-07 Design u4b-u5b-implementation.md §5.1): the s_x binding sums exact rationals (fractions).
    stdlib = {"__future__", "collections.abc", "copy", "fractions", "hashlib", "json", "math", "pathlib", "re", "struct", "typing"}
    # T4-U6 (D-5): the section-basis rules shared by the test-only verifiers live in the sibling
    # tests/verifiers/section_basis.py, which is itself standard library only.
    # T4-U4b (§5.1): the new kinds' checks live in the sibling tests/verifiers/u4b.py, standard library only too.
    # W-7 (DEL-04-01 Design global-end-actions.md): the global end-action checks live in the sibling
    # tests/verifiers/global_end.py; W-15 slice d (DEL-03-05 Design §5): the component-body checks live in
    # the sibling tests/verifiers/component_body.py; both standard library only too.
    # W-16 slice b (DEL-05-02 Design exact-combinations.md §6): the combination pass lives in the sibling
    # tests/verifiers/exact_combinations.py, standard library only besides the section-basis bit comparison and
    # (review r1) the verifier's own unit catalog, tests/verifiers/units.py, itself standard library only.
    assert _imports(VERIFIER) <= stdlib | {".component_body", ".exact_combinations", ".global_end", ".section_basis", ".u4b"}
    assert _imports(VERIFIER.with_name("exact_combinations.py")) <= stdlib | {"sys", ".section_basis", ".units"}
    assert _imports(VERIFIER.with_name("units.py")) <= stdlib
    assert _imports(VERIFIER.with_name("section_basis.py")) <= stdlib | {".component_body"}
    assert _imports(VERIFIER.with_name("u4b.py")) <= stdlib
    assert _imports(VERIFIER.with_name("component_body.py")) <= stdlib
    assert _imports(VERIFIER.with_name("global_end.py")) <= stdlib
    # No product module imports it (the VP-STATIC gate binds it by path and runs it isolated).
    for root in ("core", "apps", "tools"):
        for path in (PROJECT / root).rglob("*.py"):
            if "node_modules" in path.parts or "target" in path.parts:
                continue
            assert not any(name.startswith(("tests", "verifiers")) for name in _imports(path)), path
