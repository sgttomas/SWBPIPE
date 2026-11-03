"""RT-A4 (DEL-05-02 ``absolute-sum-srss-combinations.md`` sections 4-5 and 10; review r1 P2): test-only mirror of RE
``preview_combination_gates.rs``. Re-derives preview-physics-2's combination gates and unsigned rows from what the
received document allows and, when it is supplied, from its model.

- Model-free: ``LOAD_COMBINATION_OPERAND_WITHHELD`` needs its diagnostic naming at least one combination withheld earlier in the gate
  order; ``CONSTANT_EFFORT_UNSIGNED_COMBINATION_OPERAND`` and ``CONSTANT_EFFORT_COMBINATION_OPERAND_MIXED`` turn on an
  expression and kappa that the document never states for a withheld combination, so they are refused as unverifiable.
  An absolute sum's or SRSS's rows are re-derived bit for bit from their source rows and the factors of the
  combination's ``combination_operands`` record (a model with nonlinear supports publishes one); with no record they
  are refused as unverifiable. The flat gates (nonlinear, unit factor sum, modulus) keep the model-free reading they had.
- With the model: every gate, in the model's evaluation order after S1's interim removal, re-derived as the producer
  forms it, and every unsigned row stating its basis and sign convention and, when formed from operand rows, equal to A
  or R of the model's factors, bit for bit.

Independence: standard-library imports only, and its siblings ``combination_graph`` and ``units``.
"""
from __future__ import annotations

import math
from collections.abc import Mapping
from typing import Any

from . import combination_graph as graph
from .units import to_canonical

NONLINEAR = "NONLINEAR_COMBINATION_REQUIRES_SOLVE"
CONSTANT_EFFORT = "CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE"
CONSTANT_EFFORT_UNSIGNED = "CONSTANT_EFFORT_UNSIGNED_COMBINATION_OPERAND"
CONSTANT_EFFORT_MIXED = "CONSTANT_EFFORT_COMBINATION_OPERAND_MIXED"
MODULUS_MIXED = "COMBINATION_MODULUS_BASIS_MIXED"
STATE_CODE = "nonlinear_support_active_set_state_code"
MAGNITUDES = {"displacement_magnitude", "support_reaction_force_magnitude_v2", "support_reaction_moment_magnitude_v2"}
MODULUS_RECORD = "combination_modulus_basis_record"
TRANSLATIONS = {"UX", "Ux", "ux", "UY", "Uy", "uy", "UZ", "Uz", "uz"}
ROTATIONS = {"RX", "Rx", "rx", "RY", "Ry", "ry", "RZ", "Rz", "rz"}
UNSIGNED_BASES = {"absolute_sum": (graph.ABSOLUTE_SUM_BASIS, graph.ABSOLUTE_SUM_SIGN), "srss": (graph.SRSS_BASIS, graph.SRSS_SIGN)}


def _fail(code: str) -> ValueError:
    return ValueError(f"SOURCE_PREVIEW_PHYSICS_COMBINATION_{code}")


def _require(condition: Any, code: str) -> None:
    if not condition:
        raise _fail(code)


def _items(value: Any) -> list:
    return value if isinstance(value, list) else []


def _combination_of(row: Any) -> str | None:
    basis = row.get("basis_ref") if isinstance(row, Mapping) else None
    return basis.get("ref_id") if isinstance(basis, Mapping) and basis.get("ref_type") == "combination" else None


def _unsigned_claim(row: Mapping[str, Any]) -> tuple[str, bool] | None:
    metadata = row.get("metadata") or {}
    by_basis = {graph.ABSOLUTE_SUM_BASIS: "absolute_sum", graph.SRSS_BASIS: "srss"}.get(metadata.get("basis"))
    by_sign = {graph.ABSOLUTE_SUM_SIGN: "absolute_sum", graph.SRSS_SIGN: "srss"}.get(metadata.get("sign_convention"))
    claimed = by_basis or by_sign
    return None if claimed is None else (claimed, by_basis == by_sign)


def _bits(value: float) -> int:
    import struct
    return struct.unpack("<Q", struct.pack("<d", value))[0]


def _unsigned_rows(rows: list, by_id: dict, cid: str, basis: str, terms: list) -> None:
    srss = basis == "srss"
    basis_text, sign = UNSIGNED_BASES[basis]
    factors = [f for _, f in terms]
    for row in rows:
        if _combination_of(row) != cid or row.get("kind") == MODULUS_RECORD:
            continue
        metadata = row.get("metadata") or {}
        _require(metadata.get("basis") == basis_text and metadata.get("sign_convention") == sign, "UNSIGNED_ROW")
        if row.get("kind") in MAGNITUDES:
            continue  # the norm of the combined components, checked with the other combinations
        refs = row.get("source_result_refs")
        _require(isinstance(refs, list) and len(refs) == len(terms), "UNSIGNED_SOURCES")
        values = []
        for ref, (operand, _) in zip(refs, terms):
            src = by_id.get(ref) if isinstance(ref, str) else None
            smeta = (src or {}).get("metadata") or {}
            _require(src is not None and graph.owns(operand, src) and src.get("kind") == row.get("kind")
                     and src.get("entity_ref") == row.get("entity_ref") and src.get("unit") == row.get("unit")
                     and smeta.get("component") == metadata.get("component") and smeta.get("location") == metadata.get("location"),
                     "UNSIGNED_SOURCES")
            _require(type(src.get("value")) in (int, float), "UNSIGNED_SOURCES")
            values.append(float(src["value"]))
        expected = graph.unsigned_value(srss, factors, values)
        value = row.get("value")
        _require(expected is not None and type(value) in (int, float) and _bits(float(value)) == _bits(expected), "UNSIGNED_VALUE")


def _model_terms(combination: Mapping[str, Any]) -> list:
    terms = []
    for term in _items(combination.get("terms")):
        if not isinstance(term, Mapping) or type(term.get("factor")) not in (int, float):
            continue
        if isinstance(term.get("combination"), str):
            terms.append((("combination", term["combination"]), float(term["factor"])))
        elif isinstance(term.get("load_case"), str):
            terms.append((("case", term["load_case"]), float(term["factor"])))
    return terms


def check(source: Mapping[str, Any], model: Mapping[str, Any] | None = None) -> None:
    """The gates and unsigned rows of a solved preview-physics-2 document; ``model``, when supplied, is its model."""
    if (source.get("status") or {}).get("mechanics") != "MECHANICS_SOLVED":
        return
    evidence = source.get("contract_evidence") or {}
    gates = _items(evidence.get("combination_gates"))
    rows = [r for r in _items(source.get("results")) if isinstance(r, Mapping)]
    by_id = {r["id"]: r for r in rows if isinstance(r.get("id"), str)}
    if model is not None:
        _model_check(model, gates, rows, by_id)
    else:
        _free_check(source, gates, rows, by_id)


def _free_check(source: Mapping[str, Any], gates: list, rows: list, by_id: dict) -> None:
    diagnostics = [d for d in _items(source.get("diagnostics")) if isinstance(d, Mapping)]
    withheld: set[str] = set()
    for gate in gates:
        cid, reason = gate.get("combination_id"), gate.get("reason")
        if reason is None:
            continue
        if reason == graph.OPERAND_WITHHELD:
            named = [d for d in diagnostics if d.get("code") == graph.OPERAND_WITHHELD and _items(d.get("affected_refs"))[:1] == [cid]]
            _require(len(named) == 1, "GATE_DERIVATION")
            operands = _items(named[0].get("affected_refs"))[1:]
            _require(operands and all(isinstance(o, str) and o in withheld for o in operands), "GATE_DERIVATION")
        elif reason in (CONSTANT_EFFORT_UNSIGNED, CONSTANT_EFFORT_MIXED):
            raise _fail("GATE_UNVERIFIED")
        withheld.add(cid)
    records = {r.get("combination_id"): r for r in _items((source.get("contract_evidence") or {}).get("combination_operands")) if isinstance(r, Mapping)}
    checked: set[str] = set()
    for row in rows:
        cid, claim = _combination_of(row), _unsigned_claim(row)
        if cid is None or claim is None:
            continue
        claimed, consistent = claim
        _require(consistent, "UNSIGNED_ROW")
        if cid in checked:
            continue
        checked.add(cid)
        record = records.get(cid)
        _require(record is not None, "UNSIGNED_UNVERIFIED")
        _require(record.get("basis") == claimed, "UNSIGNED_ROW")
        terms = [graph.evidence_term(t) for t in _items((record.get("expression") or {}).get("terms"))]
        _require(all(t is not None for t in terms), "UNSIGNED_SOURCES")
        _unsigned_rows(rows, by_id, cid, claimed, terms)


def _modulus_key(case: Mapping[str, Any]) -> str | None:
    reference, temperature = case.get("modulus_basis_ref"), case.get("modulus_basis_temperature")
    if isinstance(reference, str) and temperature is None:
        return f"exact:{reference}"
    if reference is None and temperature is not None:
        try:
            kelvin = to_canonical(temperature.get("value"), temperature.get("unit"), "temperature")
        except (ValueError, TypeError, AttributeError) as exc:
            raise _fail("MODEL") from exc
        return f"temperature_kelvin_bits:{_bits(kelvin):016x}"
    _require(reference is None and temperature is None, "MODEL")
    return None


def _consuming_constant_effort(support: Mapping[str, Any], nodes: set[str]) -> bool:
    if support.get("nonlinear") is not None:
        return False
    hanger = support.get("hanger") if isinstance(support.get("hanger"), Mapping) else {}
    hanger_type = hanger.get("hanger_type").strip() if isinstance(hanger.get("hanger_type"), str) and hanger.get("hanger_type").strip() else None
    if hanger_type is None and isinstance(support.get("family"), str):
        hanger_type = support["family"].strip()
    if hanger_type != "constant_effort_support":
        return False
    load = (hanger.get("constant_load") or {}).get("value") if isinstance(hanger.get("constant_load"), Mapping) else None
    if type(load) not in (int, float) or not math.isfinite(load) or load <= 0:
        return False
    restraints = _items(support.get("restraints"))
    return (all(isinstance(r, str) and (r in TRANSLATIONS or r in ROTATIONS) for r in restraints)
            and sum(1 for r in restraints if r in TRANSLATIONS) == 1 and support.get("node") in nodes)


def _model_check(model: Mapping[str, Any], gates: list, rows: list, by_id: dict) -> None:
    # A document stating no combination at all makes no gate claim to re-derive.
    if not gates and not any(_combination_of(r) is not None for r in rows):
        return
    combinations = [c for c in _items(model.get("combinations")) if isinstance(c, Mapping)]
    load_cases = [c for c in _items(model.get("load_cases")) if isinstance(c, Mapping)]
    cases = {c["id"] for c in load_cases if isinstance(c.get("id"), str)}
    # S1's interim withholding removes a difference-basis or minuend_final combination and every one naming one.
    removed: set[str] = set()
    for index in graph.model_order(combinations, cases):
        c = combinations[index]
        interim = "difference_basis" in c or c.get("subtrahend_support_state") == "minuend_final"
        if interim or any(o[0] == "combination" and o[1] in removed for o in graph.model_operands(c, cases)):
            removed.add(c.get("id"))
    kept = [c for c in combinations if c.get("id") not in removed]
    order = graph.model_order(kept, cases)
    _require(len(gates) == len(order), "GATE_DERIVATION")
    supports = [s for s in _items(model.get("supports")) if isinstance(s, Mapping)]
    nodes = {n.get("id") for n in _items(model.get("nodes")) if isinstance(n, Mapping)}
    nonlinear = any(s.get("nonlinear") is not None for s in supports) or any(r.get("kind") == STATE_CODE for r in rows)
    consuming = any(_consuming_constant_effort(s, nodes) for s in supports)
    keys = {c["id"]: _modulus_key(c) for c in load_cases if isinstance(c.get("id"), str)}
    combination_graph = graph.Graph()
    withheld: set[str] = set()
    for gate, index in zip(gates, order):
        combination = kept[index]
        cid, basis = combination.get("id"), combination.get("basis")
        operands = graph.model_operands(combination, cases)
        terms = _model_terms(combination) if basis in graph.TERM_BASES else []
        operand_kappa = [combination_graph.kappa_of(o) for o in operands]
        oq1 = basis == "mechanics" and combination_graph.solved_state_plus_increments(terms)
        own = combination_graph.add(cid, basis, operands, [f for _, f in terms])
        derived: str | None
        if any(o[0] == "combination" and o[1] in withheld for o in operands):
            derived = graph.OPERAND_WITHHELD
        elif basis not in graph.TERM_BASES:
            derived = None
        elif basis == "mechanics" and nonlinear and not oq1:
            derived = NONLINEAR
        else:
            derived = None
            if consuming:
                if basis == "mechanics":
                    fixed = graph.mechanics_gate(own, operand_kappa)
                    derived = {"operand": CONSTANT_EFFORT_MIXED, "unit": CONSTANT_EFFORT}.get(fixed) if fixed else None
                elif graph.unsigned_gate(operand_kappa):
                    derived = CONSTANT_EFFORT_UNSIGNED
            if derived is None:
                reached = {keys.get(case) for case in combination_graph.reached.get(cid, []) if case in keys}
                derived = MODULUS_MIXED if len(reached) > 1 else None
        _require(gate.get("combination_id") == cid and gate.get("withheld") is (derived is not None) and gate.get("reason") == derived,
                 "GATE_DERIVATION")
        if derived is not None:
            withheld.add(cid)
        elif basis in UNSIGNED_BASES:
            _unsigned_rows(rows, by_id, cid, basis, terms)
    bases = {c.get("id"): c.get("basis") for c in kept}
    for row in rows:
        cid, claim = _combination_of(row), _unsigned_claim(row)
        if cid is not None and claim is not None:
            _require(claim[1] and bases.get(cid) == claim[0], "UNSIGNED_ROW")
