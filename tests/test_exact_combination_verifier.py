"""W-16 slice b (DEL-05-02 Design `exact-combinations.md` §6, §8): the test-only Python verifier
of the pressure-2 combination pass (`tests/verifiers/exact_combinations.py`) over the shared
combination corpus, mirroring the Rust reader's forgery tests
(`core/reporting/result_export/tests/exact_combination_contract.rs`) with the same refusal codes.
"""
from __future__ import annotations

from copy import deepcopy
import json
from pathlib import Path
import struct

import pytest

from tests.verifiers import exact_combinations as combinations
from tests.verifiers import reader_verdict as rv

ROOT = Path(__file__).resolve().parents[1]
CORPUS = ROOT / "fixtures/results/pressure_v3_combination_reader_corpus.json"


def corpus():
    return json.loads(CORPUS.read_text(encoding="utf-8"))["cases"]


def entry(prefix):
    case = next(c for c in corpus() if c["label"].startswith(prefix))
    return deepcopy(case["envelope"]), deepcopy(case["model"])


def code(source, model=None):
    outcome = rv.validate("pressure", source, model)
    return None if outcome["ok"] else outcome["error"]["code"]


def index(source, predicate):
    return next(i for i, r in enumerate(source["results"]) if r.get("basis_ref", {}).get("ref_type") == "combination" and predicate(r))


def test_every_corpus_envelope_is_admitted_with_and_without_its_model():
    cases = corpus()
    assert len(cases) == 4
    for case in cases:
        assert code(case["envelope"]) is None, case["label"]
        assert code(case["envelope"], case["model"]) is None, case["label"]


def test_forged_rows_are_refused():
    source, _ = entry("W16B line")
    i = index(source, lambda r: r["kind"] == "pipe_wall_axial_force_v2")
    forged = deepcopy(source)
    bits = struct.unpack(">Q", struct.pack(">d", forged["results"][i]["value"]))[0] + 1
    forged["results"][i]["value"] = struct.unpack(">d", struct.pack(">Q", bits))[0]
    assert code(forged) == "SOURCE_PHYSICS_COMBINATION_ROW"
    forged = deepcopy(source)
    del forged["results"][i]
    assert code(forged) == "SOURCE_PHYSICS_COMBINATION_ROW_COVERAGE"
    forged = deepcopy(source)
    extra = deepcopy(forged["results"][i])
    extra["id"] = "result:combination:forged"
    forged["results"].append(extra)
    assert code(forged) == "SOURCE_PHYSICS_COMBINATION_ROW_COVERAGE"
    forged = deepcopy(source)
    forged["results"][i]["source_result_refs"][0] = "result:forged"
    assert code(forged) == "SOURCE_PHYSICS_COMBINATION_ROW"
    j = index(source, lambda r: r["kind"] == "pipe_axial_membrane_stress_v2")
    forged = deepcopy(source)
    basis = forged["results"][j]["metadata"]["basis"]
    forged["results"][j]["metadata"]["basis"] = (basis.replace("nominal_section", "reduced_section") if basis.endswith("nominal_section")
                                                 else basis.replace("reduced_section", "nominal_section"))
    assert code(forged) == "SOURCE_PHYSICS_COMBINATION_ROW"
    forged = deepcopy(source)
    forged["results"][i]["metadata"]["sign_convention"] = "certified combination"
    assert code(forged) == "SOURCE_PHYSICS_COMBINATION_ROW"
    forged = deepcopy(source)
    del forged["contract_evidence"]["exact_combinations"]
    assert code(forged) == "SOURCE_PHYSICS_COMBINATION_EVIDENCE_MISSING"
    forged = deepcopy(source)
    forged["contract_evidence"]["exact_combinations"] = []
    assert code(forged) == "SOURCE_PHYSICS_COMBINATION_EVIDENCE_SHAPE"


def _record(source, mutate):
    forged = deepcopy(source)
    mutate(forged["contract_evidence"]["exact_combinations"][0])
    return forged


@pytest.mark.parametrize("mutation, expected", [
    (lambda r: r.__setitem__("gate", {"withheld": True, "reason": "COMBINATION_MODULUS_BASIS_MIXED"}), "SOURCE_PHYSICS_COMBINATION_GATE"),
    (lambda r: r.__setitem__("standing", "certified"), "SOURCE_PHYSICS_COMBINATION_STANDING"),
    (lambda r: r.__setitem__("source_recovery", "attempted"), "SOURCE_PHYSICS_COMBINATION_SOURCE_RECOVERY"),
    (lambda r: r["operands"][1]["numerical_quality"].__setitem__("solve_quality", "sensitive"), "SOURCE_PHYSICS_COMBINATION_OPERAND_STANDING"),
    (lambda r: r["operands"][0].__setitem__("modulus_basis_ref", "point:forged"), "SOURCE_PHYSICS_COMBINATION_GATE"),
    (lambda r: r["recoveries"][0].__setitem__("value", 1.0), "SOURCE_PHYSICS_COMBINATION_RECOVERY"),
    (lambda r: next(w for w in r["recoveries"] if w["sign"] == "negated").__setitem__("sign", "unchanged"), "SOURCE_PHYSICS_COMBINATION_RECOVERY"),
    (lambda r: r["recoveries"].pop(), "SOURCE_PHYSICS_COMBINATION_RECOVERY"),
    (lambda r: next(row for row in r["rows"] if row["sources"]).__setitem__("sources", []), "SOURCE_PHYSICS_COMBINATION_ROW_EVIDENCE"),
    (lambda r: r["missing_sources"].pop(), "SOURCE_PHYSICS_COMBINATION_MISSING_SOURCES"),
    (lambda r: r.__setitem__("withheld", [{"name": "support-action:9:support:a", "reason": "load-case algebra refused"}]), "SOURCE_PHYSICS_COMBINATION_WITHHELD"),
    (lambda r: r.__setitem__("certified", True), "SOURCE_PHYSICS_COMBINATION_RECORD_SHAPE"),
])
def test_forged_records_are_refused(mutation, expected):
    source, _ = entry("W16B line")
    assert code(_record(source, mutation)) == expected


def test_forged_expressions_and_the_model_binding_are_refused():
    source, model = entry("W16B line")
    for factor in (1.25, 1.3000000000000003):
        forged = _record(source, lambda r: r["expression"]["terms"][1].__setitem__("factor", factor))
        assert code(forged).startswith("SOURCE_PHYSICS_COMBINATION_")
    other = deepcopy(model)
    other["combinations"][0]["terms"][1]["factor"] = 1.5
    assert code(source, other) == "SOURCE_PHYSICS_COMBINATION_MODEL_EXPRESSION"
    fewer = deepcopy(model)
    fewer["combinations"].pop()
    assert code(source, fewer) == "SOURCE_PHYSICS_COMBINATION_MODEL_COVERAGE"


def test_forged_gates_and_withheld_rows_are_refused():
    source, _ = entry("W16B connector")
    records = source["contract_evidence"]["exact_combinations"]
    gated = next(i for i, r in enumerate(records) if r["gate"]["withheld"])
    assert records[gated]["gate"]["reason"] == "COMBINATION_INSTALLED_REFERENCE_REQUIRES_SOLVE"
    forged = deepcopy(source)
    forged["contract_evidence"]["exact_combinations"][gated]["gate"] = {"withheld": False, "reason": None}
    assert code(forged) == "SOURCE_PHYSICS_COMBINATION_GATE"
    forged = deepcopy(source)
    forged["contract_evidence"]["connector"][0]["q_ref"][0] = 0.0
    assert code(forged) is not None
    forged = deepcopy(source)
    forged["diagnostics"] = [d for d in forged["diagnostics"] if d["code"] != "COMBINATION_INSTALLED_REFERENCE_REQUIRES_SOLVE"]
    assert code(forged) == "SOURCE_PHYSICS_COMBINATION_DIAGNOSTICS"
    source, _ = entry("W16B MO-F")
    assert sum(w["reason"] == "tie-rod row" for w in source["contract_evidence"]["exact_combinations"][0]["withheld"]) == 2
    forged = _record(source, lambda r: r["withheld"].pop())
    assert code(forged) == "SOURCE_PHYSICS_COMBINATION_WITHHELD"
    source, model = entry("W16B load state")
    assert source["contract_evidence"]["exact_combinations"][0]["gate"]["reason"] == "COMBINATION_RESOLVED_STIFFNESS_MIXED"
    forged = _record(source, lambda r: r.__setitem__("gate", {"withheld": False, "reason": None}))
    assert code(forged) == "SOURCE_PHYSICS_COMBINATION_GATE"
    assert code(source, model) is None


def test_the_exact_algebra_rounds_once():
    """The verifier's own exact algebra: once-rounded sums of exact products, +0.0 for an exact or
    underflowed zero, ties to even, and the inclusive unit factor-sum rule."""
    expression = combinations.Expression("mechanics", {"terms": [{"load_case_id": "a", "factor": 1.0}, {"load_case_id": "b", "factor": 1.0}]})
    assert expression.evaluate([1.0, 2.0 ** -53]) == 1.0
    assert expression.evaluate([1.0, -1.0]) == 0.0 and struct.pack(">d", expression.evaluate([-0.0, 0.0])) == struct.pack(">d", 0.0)
    three = combinations.Expression("mechanics", {"terms": [{"load_case_id": c, "factor": f} for c, f in (("a", 1.0), ("b", 1.0), ("c", 1.0))]})
    assert three.evaluate([1.0, 2.0 ** -53, 2.0 ** -100]) == 1.0 + 2.0 ** -52
    p46 = 2.0 ** -46
    for factors, admitted in (([1.0], True), ([0.3, 0.7], True), ([1.0, 2.0 ** -54], True), ([1.0, p46], True), ([1.0, -p46], True),
                              ([2.0], False), ([0.0], False), ([2.0 ** 53, -(2.0 ** 53)], False), ([1.0, p46, 2.0 ** -100], False),
                              ([1.0, -p46, -(2.0 ** -100)], False), ([float("nan")], False)):
        assert combinations.unit_factor_sum(factors) is admitted, factors
    assert combinations.norm3(3.0, 4.0, 0.0) == 5.0
    assert combinations.identity(["é", "a:b"]) == "2:é3:a:b"
