"""RT-A4 (DEL-05-02 absolute-sum-srss-combinations.md): the independent verifier's combination algebra.

The Python mirror (``tests/verifiers/combination_graph.py``) reproduces every row of the frozen V-1 kernel table bit
for bit, computes kappa exactly with the 4,096-bit representation (V-10's chain), and decides decision OQ-1 as the
product does, RT-A3's reference shape included.
"""
from __future__ import annotations

import json
import struct
from fractions import Fraction
from pathlib import Path

from tests.verifiers import combination_graph as graph

PROJECT = Path(__file__).resolve().parents[1]
T = 5e-324


def _float(hex_bits: str) -> float:
    return struct.unpack(">d", int(hex_bits, 16).to_bytes(8, "big"))[0]


def _bits(value: float) -> str:
    return "0x%016x" % struct.unpack(">Q", struct.pack(">d", value))[0]


def test_v1_kernel_table_is_reproduced_bit_for_bit():
    table = json.loads((PROJECT / "validation/references/rt_a4_combinations/references.json").read_text())
    for case in table["cases"]:
        factors = [_float(t[0]) for t in case["terms"]]
        values = [_float(t[1]) for t in case["terms"]]
        for key, srss in (("absolute_sum", False), ("srss", True)):
            got = graph.unsigned_value(srss, factors, values)
            assert ("non_representable" if got is None else _bits(got)) == case[key], (case["id"], key)


def test_v10_kappa_chain_and_the_representation_bound():
    g = graph.Graph()
    a, b = ("case", "A"), ("case", "B")
    g.add("M0", "mechanics", [a, b], [1.0, T])
    g.add("D0", "result_state_subtraction", [("combination", "M0"), a], [])
    g.add("M1", "mechanics", [a, ("combination", "D0")], [1.0, T])
    g.add("D1", "result_state_subtraction", [("combination", "M1"), a], [])
    g.add("M2", "mechanics", [a, ("combination", "D1")], [1.0, T])
    d2 = g.add("D2", "result_state_subtraction", [("combination", "M2"), a], [])
    assert d2 == Fraction(T) ** 3 and d2 != 0
    assert graph.unsigned_gate([d2])
    assert g.add("M3", "mechanics", [a, ("combination", "D2")], [1.0, T]) == graph.UNREPRESENTABLE
    q = g.add("Q", "mechanics", [a, ("combination", "D2")], [2.0, T])
    assert q == graph.UNREPRESENTABLE and graph.mechanics_gate(q, [Fraction(1), d2]) == "unit"
    # Flat decisions are the unit factor-sum rule.
    for factors, admitted in (([0.3, 0.7], True), ([1.2, -0.2], True), ([1 / 3, 2 / 3], True), ([1 + 2 ** -46], True),
                              ([2.0], False), ([1 + 2 ** -45], False), ([2.0 ** 52, -2.0 ** 52], False)):
        h = graph.Graph()
        kappa = h.add("X", "mechanics", [("case", f"c{i}") for i in range(len(factors))], factors)
        assert (graph.mechanics_gate(kappa, [Fraction(1)] * len(factors)) is None) is admitted, factors


def test_decision_oq1_and_rt_a3_reference_shape():
    g = graph.Graph()
    a, b = ("case", "A"), ("case", "B")
    g.add("D", "result_state_subtraction", [a, b], [])
    g.add("E", "srss", [("combination", "D")], [1.0])
    g.add("S", "srss", [a], [1.0])
    g.add("R", "range_envelope", [a, b], [])
    assert g.solved_state_plus_increments([(a, 1.0), (("combination", "E"), 1.0)])
    for terms in ([(a, 1.0)], [(a, 1.0), (b, 1.0), (("combination", "E"), 1.0)], [(a, 1.5), (("combination", "E"), 1.0)],
                  [(b, 1.0), (("combination", "S"), 1.0)], [(b, 1.0), (("combination", "R"), 1.0)]):
        assert not g.solved_state_plus_increments(terms), terms
