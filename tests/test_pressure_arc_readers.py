"""T4-U2: the Python pressure-1 reader admits realized-arc envelopes over the
shared arc corpus (Rust ``tests/pressure_arc_contract.rs`` and TypeScript
``pressureArcEvidence.test.ts`` read the same bytes), refuses each broken arc
binding by name, refuses arc evidence under v2, and covers a model whose only
pipe is a replaced span by its connector (T4-RV23 NOTE-4).
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
CORPUS = json.loads((PROJECT / "fixtures/results/pressure_v3_arc_reader_corpus.json").read_text())
CASES = [(case["label"], case["envelope"]) for case in CORPUS["cases"]]
ARC_CASES = [(label, e) for label, e in CASES if any("bend_adjacent_junctions" in r for r in e["contract_evidence"]["pressure"])]


def load_state(envelope):
    return "load_reference_states" in envelope["contract_evidence"]


def row(e, kind, entity, location):
    return next(r for r in e["results"] if r["kind"] == kind and r["entity_ref"] == entity and r["metadata"]["location"] == location)


def region(e):
    return e["contract_evidence"]["pressure"][0]


def member(items, pipe):
    return next(item for item in items if item["pipe_id"] == pipe)


def _arc_hoop_row(e):
    hoop = deepcopy(row(e, "pipe_lame_hoop_stress_v2", "pipe:S1", "midspan"))
    hoop["entity_ref"], hoop["id"] = "pipe:BEND", "result:arc-hoop"
    e["results"].append(hoop)


def _maximum_not_withheld(e):
    coverage = e["contract_evidence"]["exact_cases"][0]["stress_maximum_coverage"]
    coverage["unavailable_pipe_ids"], coverage["complete"] = [], True


def _strain_sign(e):
    load = member(region(e)["applied_loads"], "pipe:BEND")
    load["arc_pressure_strain"] = -load["arc_pressure_strain"]


def _cap_removal(e):
    load = member(region(e)["applied_loads"], "pipe:BEND")
    load["bend_cap_pair_removed_global_n"][0][0] = 2.0 * load["bend_cap_pair_removed_global_n"][0][0] + 1.0


def _end_row_missing(e):
    rid = row(e, "pipe_wall_endpoint_action_v2", "pipe:BEND", "end_j")["id"]
    e["results"] = [r for r in e["results"] if r["id"] != rid]
    region(e)["result_ids"] = [r for r in region(e)["result_ids"] if r != rid]


def _bend_term_on_straight(e):
    groups = e["contract_evidence"]["exact_cases"][0]["pressure_rhs_assembly"]["groups"]
    next(t for g in groups for t in g["terms"] if t["kind"] == "terminal_cap")["kind"] = "bend_cap_removed"


def _junction_tangent(e):
    region(e)["bend_adjacent_junctions"][0]["t_in_global"][1] += 1e-3


# (label, mutation, detail): the same mutations as the Rust reader's test.
MUTATIONS = [
    ("arc sign", lambda e: row(e, "pipe_wall_axial_force_v2", "pipe:BEND", "midspan")["metadata"].__setitem__("sign_convention", "tension-positive material wall section resultant Nw"), "physical sign convention"),
    ("arc hoop row", _arc_hoop_row, "arc withheld"),
    ("arc maximum not withheld", _maximum_not_withheld, "arc withheld"),
    ("arc region key removed", lambda e: region(e).pop("tangency_rule"), "region shape"),
    ("arc approximation", lambda e: region(e).__setitem__("approximation", "long_straight_annulus_small_strain_v2"), "arc region profile"),
    ("arc withheld reason", lambda e: region(e)["withheld_on_arcs"].__setitem__("reason", "withheld"), "arc region profile"),
    ("arc member kind", lambda e: member(region(e)["geometry"], "pipe:BEND").__setitem__("member_kind", "realized_mitre"), "arc geometry"),
    ("arc strain sign", _strain_sign, "arc applied load"),
    ("arc cap removal", _cap_removal, "arc applied load"),
    ("junction dropped", lambda e: region(e)["bend_adjacent_junctions"].pop(), "arc junctions"),
    ("junction theta", lambda e: region(e)["bend_adjacent_junctions"][0].__setitem__("theta_rad", 2e-3), "arc junctions"),
    ("junction tangent", _junction_tangent, "arc junction tangent"),
    ("arc end row missing", _end_row_missing, "pressure station coverage"),
    ("bend term on a straight", _bend_term_on_straight, "arc RHS term"),
    ("case-free quantity row", lambda e: row(e, "global_nodal_displacement_x", "node:B", "node").pop("basis_ref"), "row case reference"),
]


def test_corpus_shape():
    assert len(CASES) == 5 and len(ARC_CASES) == 3
    assert {load_state(e) for _, e in ARC_CASES} == {True, False}


@pytest.mark.parametrize("label,envelope", CASES)
def test_pressure_1_reader_and_dispatch_admit_the_arc_corpus(label, envelope):
    assert envelope["producer"]["semantic_contract_id"] == PRESSURE_ID
    validate_pressure_evidence(deepcopy(envelope))
    validate_exact_pressure_evidence(deepcopy(envelope))


@pytest.mark.parametrize("label,envelope", ARC_CASES)
@pytest.mark.parametrize("what,mutate,detail", MUTATIONS, ids=[m[0] for m in MUTATIONS])
def test_broken_arc_bindings_are_refused_by_name(label, envelope, what, mutate, detail):
    broken = deepcopy(envelope)
    mutate(broken)
    with pytest.raises(ValueError) as refused:
        validate_pressure_evidence(broken)
    # The 0.4.0 pre-pass names its own codes (SOURCE_LOAD_REFERENCE_REGION_SHAPE).
    assert str(refused.value).lower().replace("_", " ").endswith(detail.lower()), str(refused.value)


@pytest.mark.parametrize("label,envelope", ARC_CASES)
def test_arc_evidence_is_refused_under_v2(label, envelope):
    as_v2 = deepcopy(envelope)
    for case in as_v2["contract_evidence"]["exact_cases"]:
        case["profile_mode"] = "exact_straight_pressure_v2"
    for r in as_v2["contract_evidence"]["pressure"]:
        r["profile_version"], r["profile_mode"] = "2.0.0", "exact_straight_pressure_v2"
    reader = validate_load_reference_evidence if load_state(envelope) else validate_physics_evidence
    with pytest.raises(ValueError) as refused:
        reader(as_v2)
    assert str(refused.value).endswith("arc unsupported"), str(refused.value)


def test_a_replaced_span_only_model_needs_its_connector():
    label, envelope = next((l, e) for l, e in CASES if l.startswith("replaced span only"))
    assert envelope["contract_evidence"]["exact_cases"][0]["pipe_sections"] == []
    broken = deepcopy(envelope)
    broken["contract_evidence"]["connector"] = []
    with pytest.raises(ValueError) as refused:
        validate_pressure_evidence(broken)
    assert str(refused.value).endswith("material/geometry coverage"), str(refused.value)
