"""RT-L2 (DEL-04-07 Design pressure-elongation.md §5): the independent pressure-3 verifier is
switch-aware, keyed on each case's ``pressure_elongation`` evidence, as the Rust reader is
(``core/reporting/result_export/tests/pressure_elongation_contract.rs``). Included: the straight
Poisson coefficient is sign*2nu and a realized arc carries its signed nonzero free strain; not
included: sign*1 and exactly zero. A case whose evidence and coefficients (or arc strain) disagree is
refused by name.

The inputs are the committed straight and arc reader corpora with the member stamped and, for "not
included", the Poisson coefficients and arc strains re-formed by hand: the readers check bindings and
declared semantics, never the assembled values."""
from __future__ import annotations

import json
from copy import deepcopy
from pathlib import Path

import pytest

from tests.verifiers.pressure_3 import CONTRACT_ID, validate_pressure_evidence

ROOT = Path(__file__).resolve().parents[1]


def _corpus(name: str) -> list[tuple[str, dict]]:
    corpus = json.loads((ROOT / f"fixtures/results/{name}.json").read_text())
    return [(case["label"], case["envelope"]) for case in corpus["cases"]]


def _arc_loads(envelope):
    return [load for region in envelope["contract_evidence"]["pressure"] for load in region["applied_loads"] if "arc_pressure_strain" in load]


def _poisson_terms(envelope):
    return [term for case in envelope["contract_evidence"]["exact_cases"] for group in case["pressure_rhs_assembly"]["groups"]
            for term in group.get("terms", []) if term["kind"] == "poisson_eigen"]


STRAIGHT = _corpus("pressure_v3_straight_reader_corpus")
ARCS = [(label, e) for label, e in _corpus("pressure_v3_arc_reader_corpus") if _arc_loads(e)]


def stamped(envelope: dict, included: bool) -> dict:
    e = deepcopy(envelope)
    e["producer"]["semantic_contract_id"] = CONTRACT_ID
    token = "included" if included else "not_included"
    pressurized = set()
    for region in e["contract_evidence"]["pressure"]:
        region["pressure_elongation"] = token
        pressurized.add(region["load_case_id"])
    for case in e["contract_evidence"]["exact_cases"]:
        if case["load_case_id"] in pressurized:
            case["pressure_rhs_assembly"]["pressure_elongation"] = token
    if not included:
        for load in _arc_loads(e):
            load["arc_pressure_strain"] = 0.0
        for term in _poisson_terms(e):
            term["coefficient"] = -1.0 if term["coefficient"] < 0 else 1.0
    return e


def _refused(e: dict, name: str) -> None:
    with pytest.raises(ValueError, match=name):
        validate_pressure_evidence(e)


@pytest.mark.parametrize("label,envelope", STRAIGHT + ARCS)
@pytest.mark.parametrize("included", [True, False])
def test_both_settings_are_admitted(label, envelope, included):
    validate_pressure_evidence(stamped(envelope, included))


def test_the_switch_is_exercised():
    assert any(_poisson_terms(e) for _, e in STRAIGHT) and ARCS


@pytest.mark.parametrize("label,envelope", STRAIGHT)
def test_a_straight_coefficient_of_the_other_setting_is_refused_by_name(label, envelope):
    included = stamped(envelope, True)
    for term in _poisson_terms(included):
        term["coefficient"] = -1.0 if term["coefficient"] < 0 else 1.0
    _refused(included, "SOURCE_PHYSICS_PRESSURE_ELONGATION_COEFFICIENT_MISMATCH")
    excluded = stamped(envelope, False)
    for term, original in zip(_poisson_terms(excluded), _poisson_terms(stamped(envelope, True))):
        term["coefficient"] = original["coefficient"]
    _refused(excluded, "SOURCE_PHYSICS_PRESSURE_ELONGATION_COEFFICIENT_MISMATCH")
    neither = stamped(envelope, False)
    _poisson_terms(neither)[0]["coefficient"] = 0.75
    _refused(neither, "RHS term coefficient binding")


@pytest.mark.parametrize("label,envelope", ARCS)
def test_an_arc_strain_of_the_other_setting_is_refused_by_name(label, envelope):
    included = stamped(envelope, True)
    for load in _arc_loads(included):
        load["arc_pressure_strain"] = 0.0
    _refused(included, "SOURCE_PHYSICS_ARC_PRESSURE_ELONGATION_MISMATCH")
    excluded = stamped(envelope, False)
    for load, original in zip(_arc_loads(excluded), _arc_loads(stamped(envelope, True))):
        load["arc_pressure_strain"] = original["arc_pressure_strain"]
    _refused(excluded, "SOURCE_PHYSICS_ARC_PRESSURE_ELONGATION_MISMATCH")


@pytest.mark.parametrize("label,envelope", STRAIGHT + ARCS)
def test_the_setting_is_required_closed_and_agreed_within_a_case(label, envelope):
    base = stamped(envelope, True)
    missing = deepcopy(base)
    del missing["contract_evidence"]["pressure"][0]["pressure_elongation"]
    with pytest.raises(ValueError):
        validate_pressure_evidence(missing)
    unknown = deepcopy(base)
    unknown["contract_evidence"]["pressure"][0]["pressure_elongation"] = "on"
    _refused(unknown, "SOURCE_PHYSICS_PRESSURE_ELONGATION_INVALID")
    case = base["contract_evidence"]["pressure"][0]["load_case_id"]
    at = next(i for i, c in enumerate(base["contract_evidence"]["exact_cases"]) if c["load_case_id"] == case)
    disagree = deepcopy(base)
    disagree["contract_evidence"]["exact_cases"][at]["pressure_rhs_assembly"]["pressure_elongation"] = "not_included"
    _refused(disagree, "SOURCE_PHYSICS_PRESSURE_ELONGATION_CASE_MISMATCH")
    absent = deepcopy(base)
    del absent["contract_evidence"]["exact_cases"][at]["pressure_rhs_assembly"]["pressure_elongation"]
    _refused(absent, "pressure RHS shape")
