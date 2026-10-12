"""pressure-1 straight families: the test-only verifier and the product refusal.

Reads the shared generated pressure-1 corpus (the Rust reader's
``tests/pressure_contract.rs`` reads the same bytes; the Rust reader is the
only product reader). The independent verifier (``tests/verifiers/pressure_1.py``)
admits v3 straight families with signed p_pa, refuses v2 evidence offered as
pressure-1 and pressure-1 evidence under a v2 identity by name, and refuses
unknown identities. Python product paths take the Rust reader's verdict (b7-plan
R2), whose header dispatch refuses pressure-1 outside the base routes; the v2
verifiers do not read v3 evidence.
"""
import ast
from copy import deepcopy
import json
from pathlib import Path

import pytest
from core.analysis_runs import rust_reader
from core.analysis_runs.compatibility import (
    _source_contract, build_analysis_run_v0_3, numerical_use_standing,
)
from tests.verifiers.load_reference_evidence import validate_load_reference_evidence
from tests.verifiers.physics_evidence import validate_physics_evidence
from tests.verifiers.pressure_1 import (
    CONTRACT_ID, LOAD_REFERENCE_ID, NOT_PRESSURE_1, PHYSICS_ID, V2_READ_AS_V3, V3_READ_AS_V2,
    validate_exact_pressure_evidence, validate_pressure_evidence, verify_source,
)

PROJECT = Path(__file__).resolve().parents[1]
CORPUS = json.loads((PROJECT / "fixtures/results/pressure_v3_straight_reader_corpus.json").read_text())
CASES = [(case["label"], case["envelope"]) for case in CORPUS["cases"]]
VERIFIER = PROJECT / "tests/verifiers/pressure_1.py"
# b7-plan R2: Python product paths read through the Rust verdict
# (`reader_verdict::route`), which names no pressure-1 route. T4-U2's interim
# Python-side name (until R2) is retired with it.
PRESSURE_1_REFUSAL = "SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED"


def load_state(envelope):
    return "load_reference_states" in envelope["contract_evidence"]


def relabel(envelope, version, mode):
    out = deepcopy(envelope)
    for case in out["contract_evidence"]["exact_cases"]:
        case["profile_mode"] = mode
    for region in out["contract_evidence"]["pressure"]:
        region["profile_version"], region["profile_mode"] = version, mode
    return out


def v2_product_reader(envelope):
    (validate_load_reference_evidence if load_state(envelope) else validate_physics_evidence)(envelope)


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
def test_python_product_paths_take_the_rust_refusal_of_pressure_1(label, envelope):
    for read in (_source_contract, lambda e: _source_contract(e, check_receipt=False),
                 lambda e: build_analysis_run_v0_3(e, input_manifest_ref={"object_type": "InputManifest", "ref": "manifest:x"}, input_manifest_hash="1" * 64)):
        with pytest.raises(ValueError, match=PRESSURE_1_REFUSAL):
            read(deepcopy(envelope))
    bases = [case["basis_ref"] for case in envelope["numerical_quality"]["cases"]]
    # The standing is the Rust reader's own verdict (b7-plan R2), which reads pressure-1.
    assert numerical_use_standing(deepcopy(envelope), bases) == rust_reader.standing(deepcopy(envelope), bases, None)["token"]
    # The v2 product readers do not read v3 evidence (their closed v2 checks refuse it).
    with pytest.raises(ValueError):
        v2_product_reader(deepcopy(envelope))


@pytest.mark.parametrize("label,envelope", CASES)
def test_neither_contract_is_read_as_the_other(label, envelope):
    as_v2 = relabel(envelope, "2.0.0", "exact_straight_pressure_v2")
    with pytest.raises(ValueError) as refused:
        validate_pressure_evidence(deepcopy(as_v2))
    assert str(refused.value).endswith(V2_READ_AS_V3)
    relabelled = deepcopy(envelope)
    relabelled["producer"]["semantic_contract_id"] = LOAD_REFERENCE_ID if load_state(envelope) else PHYSICS_ID
    with pytest.raises(ValueError) as refused:
        validate_exact_pressure_evidence(relabelled)
    assert str(refused.value).endswith(V3_READ_AS_V2)
    # A v2 publication under a v2 identity is not this verifier's to read.
    as_v2["producer"]["semantic_contract_id"] = relabelled["producer"]["semantic_contract_id"]
    with pytest.raises(ValueError, match=NOT_PRESSURE_1):
        validate_exact_pressure_evidence(as_v2)


@pytest.mark.parametrize("label,envelope", CASES)
def test_unknown_identities_are_refused(label, envelope):
    unknown = deepcopy(envelope)
    unknown["producer"]["semantic_contract_id"] = "openpipestress.result_semantics/0.3.0/pressure-2"
    with pytest.raises(ValueError, match="SOURCE_PHYSICS_PRESSURE_CONTRACT_UNKNOWN"):
        validate_exact_pressure_evidence(unknown)
    with pytest.raises(ValueError, match="SOURCE_PRODUCER_CONTRACT_UNSUPPORTED"):
        verify_source(unknown)
    profile = deepcopy(envelope)
    profile["formulation_basis"]["profile_id"] = "exact_straight_pressure_v2"
    with pytest.raises(ValueError, match="pressure-1 formulation profile"):
        validate_exact_pressure_evidence(profile)
    with pytest.raises(ValueError, match="SOURCE_FORMULATION_BASIS_UNSUPPORTED"):
        verify_source(profile)
    with pytest.raises(ValueError, match="case profile"):
        validate_pressure_evidence(relabel(envelope, "3.0.0", "exact_pressure_v4"))


@pytest.mark.parametrize("label,envelope", CASES)
def test_null_or_non_mapping_metadata_is_refused_by_name(label, envelope):
    # T4-RV13 N-1: a null producer or formulation basis is a named ValueError, never AttributeError.
    for producer in (None, [], "pressure-1"):
        broken = deepcopy(envelope)
        broken["producer"] = producer
        with pytest.raises(ValueError, match="SOURCE_PHYSICS_PRESSURE_CONTRACT_UNKNOWN"):
            validate_exact_pressure_evidence(broken)
    for basis in (None, [], "exact_pressure_v3"):
        broken = deepcopy(envelope)
        broken["formulation_basis"] = basis
        with pytest.raises(ValueError, match="pressure-1 formulation profile"):
            validate_exact_pressure_evidence(broken)


@pytest.mark.parametrize("label,envelope", CASES)
def test_header_refusals_are_named(label, envelope):
    for mutate, code in [
            (lambda e: e.__setitem__("schema_version", "0.1.0"), "SOURCE_SCHEMA_VERSION_UNSUPPORTED"),
            (lambda e: e.__setitem__("source_block_recovery", {}), "SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN"),
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
        elif isinstance(node, ast.ImportFrom):
            names.add("." * node.level + (node.module or ""))
    return names


def test_the_verifier_is_independent_and_test_only():
    # Standard library only: no product module and no producer bridge
    # (core/serialization/canonical_json/adapter.py, core/units/adapter.py).
    assert _imports(VERIFIER) <= {"__future__", "collections.abc", "copy", "hashlib", "json", "math", "pathlib", "re", "struct", "typing"}
    # No product module imports it (the VP-STATIC gate binds it by path and runs it isolated).
    for root in ("core", "apps", "tools"):
        for path in (PROJECT / root).rglob("*.py"):
            if "node_modules" in path.parts or "target" in path.parts:
                continue
            assert not any(name.startswith(("tests", "verifiers")) for name in _imports(path)), path
