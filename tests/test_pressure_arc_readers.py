"""T4-U2: the test-only pressure-2 verifier admits realized-arc envelopes over
the shared arc corpus (the Rust reader's ``tests/pressure_arc_contract.rs``
reads the same bytes; the Rust reader is the only product reader), refuses
each broken arc binding by name (T4-U4b: section stress, toroidal, arc
maximum and directional intensified rows included), and covers a model whose only pipe is a
replaced span by its connector (T4-RV23 NOTE-4). (B7b deleted the v2 contract
and its arc-refusal check with it, as in the Rust twin.)
"""
from copy import deepcopy
import json
from pathlib import Path

import pytest
from tests.verifiers.pressure_2 import (
    CONTRACT_ID as PRESSURE_ID, validate_exact_pressure_evidence, validate_pressure_evidence, verify_source,
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
    # T4-U6 (D-5 §2.1, §2.5): the straight's hoop row is published on both section bases, each id ending in its
    # basis; the arc copy keeps the pair.
    twins = [r for r in e["results"] if r["kind"] == "pipe_lame_hoop_stress_v2" and r["entity_ref"] == "pipe:S1"
             and r["metadata"]["location"] == "midspan"][:2]
    for source in twins:
        basis = source["id"].rsplit(":", 1)[1]
        assert basis in ("nominal_section", "reduced_section"), basis
        hoop = deepcopy(source)
        hoop["entity_ref"], hoop["id"] = "pipe:BEND", f"result:arc-hoop:{basis}"
        e["results"].append(hoop)


def _bend_rows(e, kind, location, component=None):
    return [r for r in e["results"] if r["kind"] == kind and r["entity_ref"] == "pipe:BEND"
            and r["metadata"]["location"] == location and (component is None or r["metadata"].get("component") == component)]


def _scale(kind, location, component=None, factor=1.001):
    def mutate(e):
        r = _bend_rows(e, kind, location, component)[0]
        r["value"] = r["value"] * factor
    return mutate


def _drop(kind, location, component=None, every=False):
    def mutate(e):
        found = _bend_rows(e, kind, location, component)
        ids = {r["id"] for r in (found if every else found[:1])}
        e["results"] = [r for r in e["results"] if r["id"] not in ids]
    return mutate


def _coverage_stale(e):
    for coverage in e["contract_evidence"]["exact_cases"][0]["stress_maximum_coverage"]:
        coverage["unavailable_pipe_ids"], coverage["complete"] = ["pipe:BEND"], False


def _bend_radius(e):
    member(region(e)["geometry"], "pipe:BEND")["bend_radius_m"] *= 1.01


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
    ("arc region key removed", lambda e: region(e).pop("tangency_rule"), "region shape"),
    ("arc approximation", lambda e: region(e).__setitem__("approximation", "long_straight_annulus_small_strain_v2"), "arc region profile"),
    ("arc stress definitions", lambda e: region(e)["arc_stress_definitions"].__setitem__(0, "changed"), "arc region profile"),
    # T4-U4b (§2.1): arcs no longer withhold their maximum.
    ("arc withholding revived", lambda e: region(e).__setitem__("withheld_on_arcs", {"reason": "withheld", "kinds": ["pipe_elastic_normal_stress_maximum_v2"]}), "region shape"),
    ("arc coverage stale", _coverage_stale, "maximum member coverage"),
    ("arc sigma plus", _scale("element_local_normal_stress_extreme", "midspan", "sigma_plus"), "sigma extreme value or coverage"),
    ("arc sigma minus missing", _drop("element_local_normal_stress_extreme", "midspan", "sigma_minus"), "sigma extreme value or coverage"),
    ("arc transverse shear", _scale("element_local_transverse_shear_stress_maximum", "midspan"), "transverse shear value"),
    ("arc toroidal hoop", _scale("pipe_toroidal_membrane_hoop_stress_v1", "midspan"), "toroidal hoop value"),
    ("arc radial", _scale("pipe_toroidal_radial_surface_stress_v1", "midspan"), "toroidal radial value"),
    ("arc maximum value", _scale("pipe_elastic_normal_stress_maximum_v2", "governing_station", factor=0.9), "extrema published witness"),
    ("arc maximum removed", _drop("pipe_elastic_normal_stress_maximum_v2", "governing_station", every=True), "extrema result binding"),
    ("arc bend radius", _bend_radius, "toroidal hoop value"),
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
    assert len(CASES) == 6 and len(ARC_CASES) == 4
    assert {load_state(e) for _, e in ARC_CASES} == {True, False}


@pytest.mark.parametrize("label,envelope", CASES)
def test_verifier_and_dispatch_admit_the_arc_corpus(label, envelope):
    assert envelope["producer"]["semantic_contract_id"] == PRESSURE_ID
    validate_pressure_evidence(deepcopy(envelope))
    validate_exact_pressure_evidence(deepcopy(envelope))
    verify_source(deepcopy(envelope))
    verify_source(deepcopy(envelope), raw=False)


@pytest.mark.parametrize("label,envelope", ARC_CASES)
@pytest.mark.parametrize("what,mutate,detail", MUTATIONS, ids=[m[0] for m in MUTATIONS])
def test_broken_arc_bindings_are_refused_by_name(label, envelope, what, mutate, detail):
    broken = deepcopy(envelope)
    mutate(broken)
    with pytest.raises(ValueError) as refused:
        validate_pressure_evidence(broken)
    # The 0.4.0 pre-pass names its own codes (SOURCE_LOAD_REFERENCE_REGION_SHAPE).
    assert str(refused.value).lower().replace("_", " ").endswith(detail.lower()), str(refused.value)


DIRECTIONAL = "curved_bend_directional_intensified_bending_stress_v1"
DIRECTIONAL_MAXIMUM = "curved_bend_directional_intensified_bending_maximum_v1"


def _measures(e):
    return e["contract_evidence"]["exact_cases"][0]["intensified_measures"]


def _swap_pair(e):
    for m in _measures(e):
        m["i_in_plane"], m["i_out_of_plane"] = m["i_out_of_plane"], m["i_in_plane"]


# T4-U4b (§2.4): directional intensified rows on the directional-SIF arc case (as the Rust test).
DIRECTIONAL_MUTATIONS = [
    ("directional value", _scale(DIRECTIONAL, "midspan"), "directional value"),
    ("directional maximum value", _scale(DIRECTIONAL_MAXIMUM, "governing_station", factor=0.9), "arc maximum bounds"),
    ("directional factor", lambda e: _measures(e)[0].__setitem__("i_in_plane", 2.4), "directional value"),
    ("directional pair swapped", _swap_pair, "directional value"),
    ("directional record dropped", lambda e: _measures(e).pop(), "unbound directional row"),
    ("directional row dropped", _drop(DIRECTIONAL, "midspan"), "directional record binding"),
    ("directional maximum dropped", _drop(DIRECTIONAL_MAXIMUM, "governing_station", every=True), "directional record binding"),
    ("directional record on a straight", lambda e: _measures(e)[0].__setitem__("pipe_id", "pipe:S1"), "directional record on a straight member"),
]


@pytest.mark.parametrize("what,mutate,detail", DIRECTIONAL_MUTATIONS, ids=[m[0] for m in DIRECTIONAL_MUTATIONS])
def test_broken_directional_bindings_are_refused_by_name(what, mutate, detail):
    label, envelope = next((l, e) for l, e in CASES if "directional SIF" in l)
    broken = deepcopy(envelope)
    mutate(broken)
    with pytest.raises(ValueError) as refused:
        validate_pressure_evidence(broken)
    assert str(refused.value).lower().replace("_", " ").endswith(detail.lower()), str(refused.value)


def test_a_replaced_span_only_model_needs_its_connector():
    label, envelope = next((l, e) for l, e in CASES if l.startswith("replaced span only"))
    assert envelope["contract_evidence"]["exact_cases"][0]["pipe_sections"] == []
    broken = deepcopy(envelope)
    broken["contract_evidence"]["connector"] = []
    with pytest.raises(ValueError) as refused:
        validate_pressure_evidence(broken)
    assert str(refused.value).endswith("material/geometry coverage"), str(refused.value)
