"""The RT-A3/RT-A4 seam on the exact route (DEL-05-02 ``absolute-sum-srss-combinations.md`` section 5.1; DEL-04-07
``pressure-contract.md``, "Combination gate"): EX gate 4, a consuming constant-effort support, is one more fixed-load
source of the kappa rule, under its own codes, after gate 3.

The Python mirror (``tests/verifiers/exact_combinations.py``) is checked at its gate on nested expressions, and end to end
on envelopes the headless preview runner solves from an invented model written here (the connector model of
``rt_a4_combinations.rs`` with a constant-effort support at the connector's free end): the actual CLI
(``openpipestress-runner read pressure`` via ``core/analysis_runs/rust_reader.py``) and the mirror admit every envelope
with and without its model and refuse each forged gate with the same code.
"""
from __future__ import annotations

import copy
import json
import os
import subprocess
from pathlib import Path

import pytest

from core.analysis_runs import rust_reader
from tests.verifiers import combination_graph as graph
from tests.verifiers import exact_combinations as combinations
from tests.verifiers import reader_verdict as py

ROOT = Path(__file__).resolve().parents[1]
CONNECTOR = ROOT / "core/product_physics/tests/fixtures/silent_zero/stress_free_connector_empty.request.json"
PROV = "rt-a4 invented seam input; not library, component, material-catalog or code data"
CE = "CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE"
CE_UNSIGNED = "CONSTANT_EFFORT_UNSIGNED_COMBINATION_OPERAND"
CE_MIXED = "CONSTANT_EFFORT_COMBINATION_OPERAND_MIXED"
IR = "COMBINATION_INSTALLED_REFERENCE_REQUIRES_SOLVE"
IR_UNSIGNED = "INSTALLED_REFERENCE_UNSIGNED_COMBINATION_OPERAND"
WITHHELD_OPERAND = "LOAD_COMBINATION_OPERAND_WITHHELD"
GATE = "SOURCE_PHYSICS_COMBINATION_GATE"
BOUND = 2.0 ** -46


# ----------------------------------------------------------------------------- the mirror's gate

def _gate_codes(chain, connector, constant_effort):
    """Each combination's gate code from the mirror's ``_gate``, reading the chain in order as the pass does. ``chain``
    holds (id, basis, expression value); every load case shares one modulus key and no member facts."""
    cases = {"case:A", "case:B", "case:C"}
    g, codes, withheld = graph.Graph(), {}, []
    modulus = {case: None for case in cases}
    for cid, basis, value in chain:
        expression = combinations.Expression(basis, value, cases)
        g.add(cid, basis, expression.operands, expression.factors)
        reason = combinations._gate(cid, expression, g, modulus, withheld, {"connector": connector}, False, constant_effort)
        codes[cid] = reason[0] if reason else None
        if reason:
            withheld.append((cid, reason[0]))
    return codes


def _terms(*items):
    return {"terms": [{"load_case_id" if kind == "case" else "combination_id": oid, "factor": f} for kind, oid, f in items]}


CHAIN = [
    ("flat-unit", "mechanics", _terms(("case", "case:A", 0.3), ("case", "case:B", 0.7))),
    ("flat-over", "mechanics", _terms(("case", "case:A", 1.0), ("case", "case:B", 1.0))),
    ("d", "result_state_subtraction", {"minuend_id": "case:A", "subtrahend_id": "case:B"}),
    ("mech-d", "mechanics", _terms(("combination", "d", 1.0))),
    ("sus+d", "mechanics", _terms(("case", "case:C", 1.0), ("combination", "d", 1.0))),
    ("srss-d", "srss", _terms(("combination", "d", 1.0))),
    ("sus+srss", "mechanics", _terms(("case", "case:C", 1.0), ("combination", "srss-d", 1.0))),
    ("m-near", "mechanics", _terms(("case", "case:A", 1.0 + BOUND))),
    ("d-near", "result_state_subtraction", {"minuend_id": "m-near", "subtrahend_id": "case:B"}),
    ("mech-amplified", "mechanics", _terms(("combination", "d-near", 2.0 ** 46))),
    ("abs-amplified", "absolute_sum", _terms(("combination", "d-near", 2.0 ** 46))),
    ("srss-cases", "srss", _terms(("case", "case:A", 1.0), ("case", "case:C", 1.0))),
    ("env-mixed", "range_envelope", {"operand_ids": ["case:A", "d"], "mode": "max_abs"}),
    ("mech-env", "mechanics", _terms(("combination", "env-mixed", 1.0))),
    ("dep", "result_state_subtraction", {"minuend_id": "srss-cases", "subtrahend_id": "case:A"}),
]
# (id, constant effort alone, installed reference as well)
EXPECTED = {
    "flat-unit": (None, None), "flat-over": (CE, IR), "d": (None, None), "mech-d": (CE, IR), "sus+d": (None, None),
    "srss-d": (None, None), "sus+srss": (None, None), "m-near": (None, None), "d-near": (None, None),
    "mech-amplified": (None, None), "abs-amplified": (CE_UNSIGNED, IR_UNSIGNED), "srss-cases": (CE_UNSIGNED, IR_UNSIGNED),
    "env-mixed": (None, None), "mech-env": (CE_MIXED, IR), "dep": (WITHHELD_OPERAND, WITHHELD_OPERAND),
}


def test_the_mirror_reads_gate_four_on_kappa_with_its_own_codes():
    installed = [{"q_ref": [0.0, 0.0, 1e-3, 0.0, 0.0, 0.0]}]
    assert _gate_codes(CHAIN, [], True) == {cid: want[0] for cid, want in EXPECTED.items()}
    assert _gate_codes(CHAIN, installed, True) == {cid: want[1] for cid, want in EXPECTED.items()}
    assert _gate_codes(CHAIN, installed, False) == {cid: want[1] for cid, want in EXPECTED.items()}
    assert set(_gate_codes(CHAIN, [], False).values()) == {None}
    assert combinations.GATE_MESSAGES[CE_UNSIGNED] == (
        "Combination withheld: a constant-effort support's force acts in every case state; an absolute sum or SRSS of operands "
        "whose fixed-load count is not exactly 0 would count it once per term with its sign discarded.")


T = 2.0 ** -1074
# Section 5.1's two undefined-kappa withholdings: an operand with an undefined kappa (mech-env) and a combination whose own
# kappa is undefined although its operands' are defined (M3 and Q: V-10's chain, kappa(M3) = 1 + t^4 needs more than 4,096
# bits).
UNDEFINED_CHAIN = [
    ("d", "result_state_subtraction", {"minuend_id": "case:A", "subtrahend_id": "case:B"}),
    ("env-mixed", "range_envelope", {"operand_ids": ["case:A", "d"], "mode": "max_abs"}),
    ("mech-env", "mechanics", _terms(("combination", "env-mixed", 1.0))),
    ("M0", "mechanics", _terms(("case", "case:A", 1.0), ("case", "case:B", T))),
    ("D0", "result_state_subtraction", {"minuend_id": "M0", "subtrahend_id": "case:A"}),
    ("M1", "mechanics", _terms(("case", "case:A", 1.0), ("combination", "D0", T))),
    ("D1", "result_state_subtraction", {"minuend_id": "M1", "subtrahend_id": "case:A"}),
    ("M2", "mechanics", _terms(("case", "case:A", 1.0), ("combination", "D1", T))),
    ("D2", "result_state_subtraction", {"minuend_id": "M2", "subtrahend_id": "case:A"}),
    ("M3", "mechanics", _terms(("case", "case:A", 1.0), ("combination", "D2", T))),
    ("Q", "mechanics", _terms(("case", "case:A", 2.0), ("combination", "D2", T))),
]
# (installed reference, constant effort) -> (mech-env, M3 and Q): a constant-effort source alone names an undefined
# operand with PF's operand-mixed code and an undefined own kappa with its own code; an installed reference names both with
# gate 3's code (section 5.1 names no operand-mixed code for it; section 10); with both, gate 3 precedes gate 4.
UNDEFINED_SOURCES = [((0.0, True), CE_MIXED, CE), ((1e-3, False), IR, IR), ((1e-3, True), IR, IR), ((0.0, False), None, None)]


@pytest.mark.parametrize("sources, operand, own", UNDEFINED_SOURCES)
def test_the_mirror_names_undefined_kappa_withholdings_by_source(sources, operand, own):
    q_ref_x, ce = sources
    connector = [{"q_ref": [q_ref_x, 0.0, 0.0, 0.0, 0.0, 0.0]}] if q_ref_x else []
    codes = _gate_codes(UNDEFINED_CHAIN, connector, ce)
    assert codes == {cid: (operand if cid == "mech-env" else own if cid in ("M3", "Q") else None) for cid, _, _ in UNDEFINED_CHAIN}
    assert combinations.GATE_MESSAGES[CE_MIXED] == (
        "Combination withheld: an operand's fixed-load count is undefined, so the number of times this combination counts a "
        "constant-effort support's force cannot be decided.")


# ----------------------------------------------------------------------------- end to end

def _model(q_ref_x, ce, chain):
    model = json.loads(CONNECTOR.read_text(encoding="utf-8"))["model"]
    model["project"]["id"] = f"project:rt-a4-seam-{'ir' if q_ref_x else 'no-ir'}-{'ce' if ce else 'no-ce'}"
    if q_ref_x:
        oc = model["components"][0]["objective_connector"]
        oc["reference_state"] = "prestressed"
        oc["q_ref"]["translation"]["x"] = q_ref_x
    if ce:
        model["supports"].append({"id": "support:ce", "node": "node:b", "family": "constant_effort_support", "restraints": ["UY"],
                                  "hanger": {"hanger_type": "constant_effort_support", "constant_load": {"value": 375.0, "unit": "N"},
                                             "travel_range": {"value": 0.05, "unit": "m"}, "source_reference": "rt_a4_invented"},
                                  "provenance": PROV})

    def force(lid, value):
        return {"id": lid, "category": "concentrated_force", "target": {"type": "node", "node": "node:b"}, "direction": "global_y",
                "magnitude": {"value": value, "unit": "N"}, "dimension": "force", "provenance": PROV}
    model["load_cases"] = [{"id": f"case:{name}", "primitive_loads": [force(f"load:{name}", value)], "pressure_regions": [], "provenance": PROV}
                           for name, value in (("A", 1000.0), ("B", -400.0), ("C", 250.0))]
    authored = []
    for cid, basis, value in chain:
        if basis == "result_state_subtraction":
            authored.append({"id": cid, "basis": basis, "provenance": PROV, **value})
        elif basis == "range_envelope":
            authored.append({"id": cid, "basis": basis, "provenance": PROV, **value})
        else:
            authored.append({"id": cid, "basis": basis, "provenance": PROV,
                             "terms": [{"load_case" if "load_case_id" in t else "combination": t.get("load_case_id", t.get("combination_id")),
                                        "factor": t["factor"]} for t in value["terms"]]})
    model["combinations"] = authored
    return model


@pytest.fixture(scope="module")
def solve(tmp_path_factory):
    runner = os.environ.get("OPENPIPESTRESS_PREVIEW_RUNNER_BIN")
    if not runner or not Path(runner).is_file():
        pytest.skip("OPENPIPESTRESS_PREVIEW_RUNNER_BIN is not supplied")
    directory = tmp_path_factory.mktemp("rt_a4_seam")

    def run(model):
        path = directory / f"{model['project']['id'].split(':')[-1]}.json"
        path.write_text(json.dumps(model), encoding="utf-8")
        completed = subprocess.run([runner, str(path), "--explicit-local-private-intent"], capture_output=True, check=True)
        envelope = json.loads(completed.stdout)["payload"]["mechanics_envelope"]
        assert envelope["status"]["mechanics"] == "MECHANICS_SOLVED"
        return envelope
    return run


def cli(source, model=None):
    payload = {"source": source} if model is None else {"source": source, "model": model}
    passed, outcome = rust_reader._run("pressure", payload)
    return None if passed else outcome["error"]["code"]


def mirror(source, model=None):
    verdict = py.validate("pressure", source, model)
    return None if verdict["ok"] else verdict["error"]["code"]


def both(source, model, expected):
    assert (cli(source, model), mirror(source, model)) == (expected, expected), f"model given: {model is not None}"


def gates(source):
    return {r["combination_id"]: r["gate"]["reason"] for r in source["contract_evidence"]["exact_combinations"]}


@pytest.mark.parametrize("q_ref_x, ce", [(0.0, True), (1e-3, True), (0.0, False)])
def test_both_readers_admit_the_solved_seam_envelopes(solve, q_ref_x, ce):
    model = _model(q_ref_x, ce, CHAIN)
    source = solve(model)
    want = {cid: (pair[1] if q_ref_x else pair[0] if ce else None) for cid, pair in EXPECTED.items()}
    assert gates(source) == want
    for given in (None, model):
        both(source, given, None)
    # Every gate reason, gate 4's two included, is one the pressure-2 schema admits.
    from tests.schema_validation import schema_for_definition, validate_instance
    schema = json.loads((ROOT / "schemas/results_pressure_3.v0.3.schema.json").read_text(encoding="utf-8"))
    validate_instance(schema_for_definition(schema, "PressureContractEvidence"), source["contract_evidence"])


def test_both_readers_refuse_forged_gate_four_reasons(solve):
    model = _model(0.0, True, CHAIN)
    source = solve(model)
    for cid, reason in (("mech-d", IR), ("srss-cases", IR_UNSIGNED), ("srss-cases", CE), ("srss-d", CE_UNSIGNED),
                        ("srss-cases", None), ("mech-d", None)):
        forged = copy.deepcopy(source)
        record = next(r for r in forged["contract_evidence"]["exact_combinations"] if r["combination_id"] == cid)
        record["gate"] = {"withheld": reason is not None, "reason": reason}
        for given in (None, model):
            both(forged, given, GATE)


@pytest.mark.parametrize("sources, operand, own", [case for case in UNDEFINED_SOURCES if case[1]])
def test_both_readers_refuse_swapped_undefined_kappa_codes(solve, sources, operand, own):
    """Each source's undefined-kappa withholdings, end to end; a code swapped in the record and its diagnostic together is
    refused by both readers with and without the model."""
    q_ref_x, ce = sources
    model = _model(q_ref_x, ce, UNDEFINED_CHAIN)
    model["project"]["id"] += "-undefined"
    source = solve(model)
    assert gates(source) == {cid: (operand if cid == "mech-env" else own if cid in ("M3", "Q") else None) for cid, _, _ in UNDEFINED_CHAIN}
    for given in (None, model):
        both(source, given, None)
    for cid, held in (("mech-env", operand), ("M3", own)):
        for code in (CE, CE_MIXED, IR):
            if code == held:
                continue
            forged = copy.deepcopy(source)
            next(r for r in forged["contract_evidence"]["exact_combinations"] if r["combination_id"] == cid)["gate"]["reason"] = code
            diagnostic = next(d for d in forged["diagnostics"]
                              if d["id"].startswith("diagnostic:exact-combination:gate:") and d["affected_refs"] == [cid])
            diagnostic["code"], diagnostic["message"] = code, combinations.GATE_MESSAGES[code]
            for given in (None, model):
                both(forged, given, GATE)
