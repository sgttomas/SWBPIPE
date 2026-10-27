"""W-16 slice b (DEL-05-02 Design `exact-combinations.md` §6, §8): the test-only Python verifier
of the pressure-2 combination pass (`tests/verifiers/exact_combinations.py`) over the shared
combination corpus, mirroring the Rust reader's forgery tests
(`core/reporting/result_export/tests/exact_combination_contract.rs`) with the same refusal codes.
"""
from __future__ import annotations

from copy import deepcopy
import json
from pathlib import Path
import math
import struct
import sys

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
    assert len(cases) == 5
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


def test_a_celsius_selection_temperature_binds_after_normalization():
    """Review r1 (1): the producer records the authored 100 degC normalized to kelvin; the model binding normalizes the
    captured quantity the same way, and refuses another temperature or the same number in another unit."""
    source, model = entry("W16B Celsius")
    assert source["contract_evidence"]["exact_combinations"][0]["operands"][0]["modulus_basis_temperature"] == {"value": 100.0 + 273.15, "unit": "K"}
    assert code(source, model) is None
    kelvin = deepcopy(model)
    for case in kelvin["load_cases"]:
        case["modulus_basis_temperature"] = {"value": 100.0 + 273.15, "unit": "K"}
    assert code(source, kelvin) is None
    for value, unit in ((101.0, "degC"), (100.0, "K"), (100.0, "degF"), (100.0, "furlong")):
        other = deepcopy(model)
        other["load_cases"][1]["modulus_basis_temperature"] = {"value": value, "unit": unit}
        assert code(source, other) == "SOURCE_PHYSICS_COMBINATION_MODEL_OPERAND", (value, unit)


def test_a_model_without_combinations_refuses_combination_records():
    """Review r1 (2): a supplied model is complete; absent, null and [] all state none."""
    source, model = entry("W16B line")
    absent = deepcopy(model)
    del absent["combinations"]
    null = deepcopy(model)
    null["combinations"] = None
    empty = deepcopy(model)
    empty["combinations"] = []
    for what, other in (("absent", absent), ("null", null), ("[]", empty)):
        assert code(source, other) == "SOURCE_PHYSICS_COMBINATION_MODEL_COVERAGE", what


@pytest.mark.parametrize("prefix, namespace", [("W16B line", "derived:"), ("W16B line", "source-missing:"),
                                               ("W16B line", "source-recovery-excluded"), ("W16B connector", "gate:"),
                                               ("W16B MO-F", "row-withheld:"), ("W16B line", "maximum:")])
@pytest.mark.parametrize("field, value", [("message", "combination rows are certified"), ("affected_refs", ["comb:forged"]),
                                          ("affected_refs", []), ("source", "core/forged"), ("severity", "blocking")])
def test_forged_diagnostics_are_refused(prefix, namespace, field, value):
    """Review r1 (5): every combination diagnostic is re-derived whole."""
    source, _ = entry(prefix)
    index = next(i for i, d in enumerate(source["diagnostics"]) if d["id"].startswith(f"diagnostic:exact-combination:{namespace}"))
    source["diagnostics"][index][field] = value
    assert code(source) == "SOURCE_PHYSICS_COMBINATION_DIAGNOSTICS"


# Shared with the Rust reader's `magnitudes_are_correctly_rounded_across_the_binary64_range` (x, y, z, norm bits).
NORM_VECTORS = [
    (0x0000000000000b92, 0x0000000000000f04, 0x0000000000000f61, 0x0000000000001869),
    (0x0000000000000849, 0x000000000000079d, 0x000000000000071f, 0x0000000000000d51),
    (0x000000000000048b, 0x80000000000009ed, 0x00000000000000b6, 0x0000000000000af0),
    (0x00000000000004ee, 0x00000000000000e1, 0x0000000000000ee0, 0x0000000000000fb2),
    (0x000000009cb706ad, 0x0000000ae4ee4542, 0x00000003935dd386, 0x0000000b7b7cf22f),
    (0x000001900977a960, 0x00000000010b4404, 0x0000ea91ac1fffc3, 0x0000ea93013c3a59),
    (0x12f4f126169bc518, 0x12775fc622e83df0, 0x9342a0caf96608b6, 0x1342a3bc244895bc),
    (0x9427dd509407296c, 0x13bd702c24f33d74, 0x133cbb58103ef1fe, 0x1427dd99346cb9ef),
    (0x1def7d457c49b3b2, 0x1d43b757e7d6cc30, 0x1c9e59279ebd0e32, 0x1def7d45df0bc176),
    (0x9d851c287ef477d6, 0x1e2ab2359fe7d0f2, 0x1dcc86713d5c804a, 0x1e2ab329f71f3be1),
    (0xb2ab523aa02fae74, 0x3191f1b98c7c7bf0, 0x31a64f1c8b404c28, 0x32ab523aa03a4349),
    (0x331f0c9a81b18ad6, 0x3356486f444e6d9a, 0x321a36b0956309a0, 0x33565e06a1179d74),
    (0x3443e93c81d6850c, 0x3451aa1e9f48246f, 0x34e448162ec6d36e, 0x34e44818b77ae67f),
    (0x34275f3f1a850e88, 0xb570f21a420f8721, 0x35632bd65adaec3b, 0x3573780f45fd833d),
    (0x3e4cfe91246e8da2, 0xbe46a7b47543ddcc, 0x3ef48a4c3e3b547b, 0x3ef48a4cc2104c16),
    (0x3fde2089564330e0, 0x3f1675559add9dd8, 0x3e6399175cca8484, 0x3fde20895ea21ef5),
    (0x4966873c6e6adbac, 0x4816d35125b18c4b, 0x4937cfbd6c9bfc28, 0x4966b95a983d9649),
    (0x48afe453bb92c56e, 0x488bcb6014ff7ade, 0x49088cc1df49eccf, 0x49088e1d01b72aa7),
    (0x4c18dab51e28b8b7, 0x4bea1ea121fb7eb2, 0xcbe1b79efd72ad17, 0x4c192a5dee3e4eeb),
    (0x4aaba5665782a09e, 0x4ba836b2b4ab6f9d, 0x4bf2be53c0331d6a, 0x4bf2c23c503c7ecc),
    (0x5e630db06ce88204, 0x5f4b7e715406b1d8, 0x5ea000dd74717b6c, 0x5f4b7e719ef59e60),
    (0x5e0e70073d47ca1c, 0xde162e052f4006a2, 0x5fe25b5eb3550eb0, 0x5fe25b5eb3550eb0),
    (0x6b1c9f35a1b0dea9, 0xeb2daa52ff14040e, 0x6b4d3a270d46e7f0, 0x6b4e5d6c112b135d),
    (0x6b69d7d5d739a5e4, 0x6a3cddc541057a1c, 0x6ac78b5fc7293aca, 0x6b69d7d682d3ee73),
    (0x7cce80edb45874ba, 0x7cb308b1ce0b8024, 0x7e0ee68356bf64e1, 0x7e0ee68356bf7567),
    (0x7ca18db6dc4f8976, 0xfdc2f938bfdc0196, 0x7dfb057bf58412e6, 0x7dfb2014060080fb),
    (0xfed341cc49226afc, 0x7fd173bb005d3a1d, 0x7e216ffd5eec53cf, 0x7fd173bb0067d9f1),
    (0x7e5b9c93558f51b2, 0xfedc5d12426909ba, 0x7eb0ae96d781778f, 0x7edcab32098a2c82),
]


def _bits(x):
    return struct.unpack(">Q", struct.pack(">d", x))[0]


def _float(bits):
    return struct.unpack(">d", struct.pack(">Q", bits))[0]


def test_magnitudes_are_correctly_rounded_across_the_binary64_range():
    """Review r1 (4): the exact derivation neither underflows nor overflows before its square root."""
    tiny = _float(1)
    assert combinations.norm3(1e-200, 0.0, 0.0) == 1e-200
    assert combinations.norm3(1e200, 0.0, 0.0) == 1e200
    assert combinations.norm3(3 * 2.0 ** -700, 4 * 2.0 ** -700, 0.0) == 5 * 2.0 ** -700
    assert combinations.norm3(3 * 2.0 ** 900, 4 * 2.0 ** 900, 0.0) == 5 * 2.0 ** 900
    assert combinations.norm3(3 * tiny, 4 * tiny, 0.0) == 5 * tiny
    assert combinations.norm3(sys.float_info.max, 0.0, 0.0) == sys.float_info.max
    assert combinations.norm3(sys.float_info.max, sys.float_info.max, 0.0) == math.inf
    assert combinations.norm3(1e-160, 2e-160, 3e-160) == 3.7416573867739414e-160
    for x, y, z, r in NORM_VECTORS:
        assert _bits(combinations.norm3(_float(x), _float(y), _float(z))) == r, (hex(x), hex(y), hex(z))
