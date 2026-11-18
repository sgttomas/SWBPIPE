"""DEL-04-11 REQ-014: the verifiers' restatement of the variable spring preload decision and gate text.

Mirrors ``core/reporting/result_export/src/spring_preload.rs``'s unit tests: each refusal of the producer's
``spring_preload::dispositions`` that a captured model can carry, and the fixed-load gate text naming its source.
Invented data.
"""
from __future__ import annotations

import copy

from tests.verifiers import spring_preload


def _model(patch: dict) -> dict:
    support = {"id": "sh", "node": "n", "family": "variable_spring_hanger", "restraints": ["UZ"],
               "hanger": {"hanger_type": "variable_spring_hanger", "stiffness": {"dof": "UZ", "value": {"value": 1.0e5, "unit": "N/m"}},
                          "cold_load": {"value": 500.0, "unit": "N"}}}
    for key, value in patch.items():
        if value is None:
            support.pop(key, None)
        else:
            support[key] = copy.deepcopy(value)
    return {"nodes": [{"id": "n"}], "supports": [support]}


def test_the_restated_decision_applies_and_refuses_as_the_producer_does() -> None:
    assert spring_preload.applying(_model({})) == [{"id": "sh", "node": "n", "dof": "UZ", "force": 500.0}]
    hanger = {"hanger_type": "variable_spring_hanger", "stiffness": {"dof": "UZ"}}
    for patch in ({"nonlinear": {"behavior": "one_way"}},
                  {"direction": {"x": 1.0, "y": 0.0, "z": 1.0}},
                  {"hanger": hanger},
                  {"hanger": {**hanger, "cold_load": {"value": -1.0, "unit": "N"}}},
                  {"restraints": [], "hanger": {"hanger_type": "variable_spring_hanger", "cold_load": {"value": 5.0, "unit": "N"}}},
                  {"restraints": ["RY"], "hanger": {"hanger_type": "variable_spring_hanger", "stiffness": {"dof": "RY"},
                                                    "cold_load": {"value": 5.0, "unit": "N"}}},
                  {"node": "elsewhere"},
                  {"family": "spring", "hanger": None}):
        assert spring_preload.applying(_model(patch)) == [], patch
    assert spring_preload.applying(_model({"direction": {"x": 0.0, "y": 0.0, "z": 1.0}}))


def test_the_gate_text_names_its_source_and_keeps_constant_effort_text() -> None:
    code = "CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE"
    assert spring_preload.fixed_load_gate_message(code, True, False, "old") == "old"
    assert spring_preload.fixed_load_gate_message("COMBINATION_MODULUS_BASIS_MIXED", False, True, "old") == "old"
    assert spring_preload.fixed_load_gate_message(code, False, True, "old") == (
        "Combination withheld: a variable spring hanger's cold load would be counted by the sum of factors instead of once.")
