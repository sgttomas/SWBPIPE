"""T4-U3 (S14): the Python pressure-1 reader admits objective connector records
and rows over the shared connector corpus (Rust ``tests/connector_contract.rs``
and TypeScript ``pressureConnectorEvidence.test.ts`` read the same bytes),
refuses each broken binding by name, and every other contract refuses
connector evidence.
"""
from copy import deepcopy
import json
from pathlib import Path

import pytest
from core.analysis_runs.load_reference_evidence import validate_load_reference_evidence
from core.analysis_runs.physics_evidence import (
    PRESSURE_ID, validate_exact_pressure_evidence, validate_physics_evidence, validate_pressure_evidence,
)

PROJECT = Path(__file__).resolve().parents[1]
CORPUS = json.loads((PROJECT / "fixtures/results/pressure_v3_connector_reader_corpus.json").read_text())
CASES = [(case["label"], case["envelope"]) for case in CORPUS["cases"]]


def load_state(envelope):
    return "load_reference_states" in envelope["contract_evidence"]


def first_connector_row(envelope):
    return next(r for r in envelope["results"] if r["kind"].startswith("connector_"))


def record(envelope):
    return envelope["contract_evidence"]["connector"][0]


def _push_duplicate_row(envelope):
    row = deepcopy(first_connector_row(envelope))
    row["id"] = "result:connector:duplicate"
    envelope["results"].append(row)


def _push_replaced_span_row(envelope):
    row = deepcopy(next(r for r in envelope["results"] if r["kind"] == "element_local_axial_force"))
    row["id"], row["entity_ref"] = "result:replaced-span", "pipe:P-130"
    envelope["results"].append(row)


def _remove_connector_row(envelope):
    envelope["results"].remove(first_connector_row(envelope))


# (label, mutation, detail): the same mutations as the Rust reader's test.
MUTATIONS = [
    ("record removed", lambda e: e["contract_evidence"].__setitem__("connector", []), "unbound connector row"),
    ("record duplicated", lambda e: e["contract_evidence"]["connector"].append(deepcopy(record(e))), "duplicate connector record"),
    ("record extra key", lambda e: record(e).__setitem__("extra", 1), "connector record shape"),
    ("record hardware tied", lambda e: record(e).__setitem__("hardware", "tied"), "connector record law"),
    ("record q_ref short", lambda e: record(e).__setitem__("q_ref", [0.0, 0.0]), "connector record frame"),
    ("record rotation scale", lambda e: record(e)["work_matrix"].__setitem__("rotation_scale_rad", 2.0), "connector work matrix"),
    ("replaced span published", lambda e: record(e).__setitem__("replaced_pipe_id", "pipe:P-120"), "connector replaced span published"),
    ("row removed", _remove_connector_row, "connector row coverage"),
    ("row unbound", lambda e: first_connector_row(e).__setitem__("entity_ref", "component:C-999"), "unbound connector row"),
    ("row unknown kind", lambda e: first_connector_row(e).__setitem__("kind", "connector_energy_v1"), "unknown connector kind"),
    ("row unit", lambda e: first_connector_row(e).__setitem__("unit", "mm"), "connector row semantics"),
    ("row basis", lambda e: first_connector_row(e)["metadata"].__setitem__("basis", "objective_connector_v1;replaces_span=pipe:P-120;symmetric_midpoint_small_rotation_v1"), "connector row semantics"),
    ("row sign", lambda e: first_connector_row(e)["metadata"].__setitem__("sign_convention", "global end action of the connector on its node (node on element), f = B^T g"), "connector row semantics"),
    ("row duplicated", _push_duplicate_row, "duplicate connector row"),
    ("replaced span row", _push_replaced_span_row, "connector replaced span published"),
]


def test_corpus_shape():
    assert len(CASES) == 2 and {load_state(e) for _, e in CASES} == {True, False}
    assert all(len(e["contract_evidence"]["connector"]) == 1 for _, e in CASES)


@pytest.mark.parametrize("label,envelope", CASES)
def test_pressure_1_reader_and_dispatch_admit_the_connector_corpus(label, envelope):
    assert envelope["producer"]["semantic_contract_id"] == PRESSURE_ID
    validate_pressure_evidence(deepcopy(envelope))
    validate_exact_pressure_evidence(deepcopy(envelope))


@pytest.mark.parametrize("label,envelope", CASES)
@pytest.mark.parametrize("what,mutate,detail", MUTATIONS, ids=[m[0] for m in MUTATIONS])
def test_broken_connector_bindings_are_refused_by_name(label, envelope, what, mutate, detail):
    broken = deepcopy(envelope)
    mutate(broken)
    with pytest.raises(ValueError) as refused:
        validate_pressure_evidence(broken)
    assert str(refused.value).endswith(f"SOURCE_PHYSICS_EVIDENCE_INVALID: {detail}"), str(refused.value)


@pytest.mark.parametrize("label,envelope", CASES)
def test_every_other_contract_refuses_connector_evidence(label, envelope):
    as_v2 = deepcopy(envelope)
    for case in as_v2["contract_evidence"]["exact_cases"]:
        case["profile_mode"] = "exact_straight_pressure_v2"
    reader = validate_load_reference_evidence if load_state(envelope) else validate_physics_evidence
    with pytest.raises(ValueError) as refused:
        reader(deepcopy(as_v2))
    assert "CONNECTOR_UNSUPPORTED" in str(refused.value) or str(refused.value).endswith("unsupported connector"), str(refused.value)
    as_v2["contract_evidence"]["connector"] = []
    with pytest.raises(ValueError) as refused:
        reader(as_v2)
    assert str(refused.value).endswith("unsupported connector"), str(refused.value)
