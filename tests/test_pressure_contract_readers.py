"""T4-U2a: Python reader dispatch on the exact pressure contract identity.

Reads the shared generated pressure-1 corpus (Rust ``tests/pressure_contract.rs``
and TypeScript ``pressureContractEvidence.test.ts`` read the same bytes). v2 is
unchanged and still refuses p_pa < 0; v3 admits straight families with signed
p_pa; neither is read as the other; unknown identities are refused.
"""
from copy import deepcopy
import json
from pathlib import Path

import pytest
from core.analysis_runs.load_reference_evidence import validate_load_reference_evidence
from core.analysis_runs.physics_evidence import (
    LOAD_REFERENCE_ID, PHYSICS_ID, PRESSURE_ID, V2_READ_AS_V3, V3_READ_AS_V2,
    validate_exact_pressure_evidence, validate_physics_evidence, validate_pressure_evidence,
)

PROJECT = Path(__file__).resolve().parents[1]
CORPUS = json.loads((PROJECT / "fixtures/results/pressure_v3_straight_reader_corpus.json").read_text())
CASES = [(case["label"], case["envelope"]) for case in CORPUS["cases"]]


def load_state(envelope):
    return "load_reference_states" in envelope["contract_evidence"]


def relabel(envelope, version, mode):
    out = deepcopy(envelope)
    for case in out["contract_evidence"]["exact_cases"]:
        case["profile_mode"] = mode
    for region in out["contract_evidence"]["pressure"]:
        region["profile_version"], region["profile_mode"] = version, mode
    return out


def v2_reader(envelope):
    (validate_load_reference_evidence if load_state(envelope) else validate_physics_evidence)(envelope)


def test_corpus_shape():
    assert len(CASES) == 4 and {load_state(e) for _, e in CASES} == {True, False}
    assert all(r["p_pa"] < 0 for _, e in CASES if not load_state(e) for r in e["contract_evidence"]["pressure"])


@pytest.mark.parametrize("label,envelope", CASES)
def test_pressure_1_reader_and_dispatch_admit_the_corpus(label, envelope):
    assert envelope["producer"]["semantic_contract_id"] == PRESSURE_ID
    validate_pressure_evidence(deepcopy(envelope))
    validate_exact_pressure_evidence(deepcopy(envelope))


@pytest.mark.parametrize("label,envelope", CASES)
def test_neither_contract_is_read_as_the_other(label, envelope):
    with pytest.raises(ValueError) as refused:
        v2_reader(deepcopy(envelope))
    assert str(refused.value).endswith(V3_READ_AS_V2)
    as_v2 = relabel(envelope, "2.0.0", "exact_straight_pressure_v2")
    with pytest.raises(ValueError) as refused:
        validate_pressure_evidence(deepcopy(as_v2))
    assert str(refused.value).endswith(V2_READ_AS_V3)
    if load_state(envelope):
        v2_reader(as_v2)
    else:
        with pytest.raises(ValueError, match="region pressure basis"):
            v2_reader(as_v2)


@pytest.mark.parametrize("label,envelope", CASES)
def test_unknown_identities_are_refused(label, envelope):
    unknown = deepcopy(envelope)
    unknown["producer"]["semantic_contract_id"] = "openpipestress.result_semantics/0.3.0/pressure-2"
    with pytest.raises(ValueError, match="SOURCE_PHYSICS_PRESSURE_CONTRACT_UNKNOWN"):
        validate_exact_pressure_evidence(unknown)
    profile = deepcopy(envelope)
    profile["formulation_basis"]["profile_id"] = "exact_straight_pressure_v2"
    with pytest.raises(ValueError, match="pressure-1 formulation profile"):
        validate_exact_pressure_evidence(profile)
    with pytest.raises(ValueError, match="case profile"):
        validate_pressure_evidence(relabel(envelope, "3.0.0", "exact_pressure_v4"))
    relabelled = deepcopy(envelope)
    relabelled["producer"]["semantic_contract_id"] = LOAD_REFERENCE_ID if load_state(envelope) else PHYSICS_ID
    with pytest.raises(ValueError) as refused:
        validate_exact_pressure_evidence(relabelled)
    assert str(refused.value).endswith(V3_READ_AS_V2)


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
