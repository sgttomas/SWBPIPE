"""RT-A4 (DEL-05-02 ``absolute-sum-srss-combinations.md`` sections 4-5 and 10; review r1 P2; section 10.3): test-only
mirror of RE ``preview_combination_gates.rs``. Re-derives preview-physics-2's combination gates and unsigned rows from the
received document and, when it is supplied, from its model.

- Model-free: the document's ``combination_derivation`` (section 10.3) states the model conditions the gates read and,
  in gate order, every absolute sum's, SRSS's and nested combination's expression with those of the combinations it
  reaches. Its shape, order and closure are checked; from it kappa and decision OQ-1 are re-derived, so each recorded
  combination's withheld-operand, nonlinear and kappa gate is checked exactly (the modulus gate is admitted as stated, as
  for flat combinations), and every unsigned row is A or R of the recorded factors and source rows, bit for bit.
  ``LOAD_COMBINATION_OPERAND_WITHHELD`` also needs its diagnostic naming earlier withheld combinations. A kappa
  withholding without its record, and a row citing an operand combination's row without its record, are refused as
  unverifiable (``..._GATE_UNVERIFIED``); an unsigned row without its record is refused (``..._UNSIGNED_UNVERIFIED``).
  The flat gates (nonlinear, unit factor sum, modulus) keep the model-free reading they had.
- With the model: the published derivation must be exactly the model's (``..._DERIVATION_EVIDENCE``); every gate, in
  the model's evaluation order after S1's interim removal, re-derived as the producer forms it, and every unsigned row
  stating its basis and sign convention and, when formed from operand rows, equal to A or R of the model's factors, bit
  for bit.

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
DERIVATION_MEMBER = "combination_derivation"
DERIVATION_KEYS = {"nonlinear_supports", "consuming_constant_effort", "combinations"}
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


def _gate_before_modulus(basis: str, operand_withheld: bool, nonlinear: bool, oq1: bool, consuming: bool, own: Any,
                         operand_kappa: list) -> str | None:
    """The gate a combination takes before its modulus gate: a withheld operand combination; nonlinear supports
    (mechanics except decision OQ-1); kappa where a consuming constant-effort support applies."""
    if operand_withheld:
        return graph.OPERAND_WITHHELD
    if basis not in graph.TERM_BASES:
        return None
    if basis == "mechanics" and nonlinear and not oq1:
        return NONLINEAR
    if consuming:
        if basis == "mechanics":
            fixed = graph.mechanics_gate(own, operand_kappa)
            return {"operand": CONSTANT_EFFORT_MIXED, "unit": CONSTANT_EFFORT}.get(fixed) if fixed else None
        if graph.unsigned_gate(operand_kappa):
            return CONSTANT_EFFORT_UNSIGNED
    return None


def _non_empty(value: Any) -> str | None:
    return value if isinstance(value, str) and value else None


def _derivation(source: Mapping[str, Any], gates: list) -> dict | None:
    """The ``combination_derivation`` member, read and checked (section 10.3); ``None`` when absent."""
    evidence = source.get("contract_evidence") or {}
    if DERIVATION_MEMBER not in evidence:
        return None
    member = evidence[DERIVATION_MEMBER]

    def check(ok: Any) -> None:
        _require(ok, "DERIVATION_EVIDENCE")

    check(isinstance(member, Mapping) and set(member) == DERIVATION_KEYS)
    nonlinear, consuming, listed = member["nonlinear_supports"], member["consuming_constant_effort"], member["combinations"]
    check(type(nonlinear) is bool and type(consuming) is bool and isinstance(listed, list) and listed)
    cases = {c.get("load_case_id") for c in _items(evidence.get("preview_cases")) if isinstance(c, Mapping)}
    position: dict[str, int] = {}
    for i, g in enumerate(gates):
        if isinstance(g, Mapping) and isinstance(g.get("combination_id"), str):
            position.setdefault(g["combination_id"], i)
    records: list[dict] = []
    last = -1
    for record in listed:
        check(isinstance(record, Mapping) and set(record) == {"combination_id", "basis", "expression"})
        cid, basis, expression = _non_empty(record["combination_id"]), record["basis"], record["expression"]
        check(cid is not None and isinstance(basis, str) and cid in position and position[cid] > last)
        last = position[cid]
        if basis in graph.TERM_BASES:
            check(isinstance(expression, Mapping) and set(expression) == {"terms"} and isinstance(expression["terms"], list) and expression["terms"])
            terms = [graph.evidence_term(t) for t in expression["terms"]]
            check(all(t is not None for t in terms))
            operands = [o for o, _ in terms]
        elif basis == "result_state_subtraction":
            check(isinstance(expression, Mapping) and set(expression) == {"minuend_id", "subtrahend_id"})
            ids = [_non_empty(expression["minuend_id"]), _non_empty(expression["subtrahend_id"])]
            check(all(ids))
            terms, operands = [], [graph.resolve(i, cases) for i in ids]
        elif basis == "range_envelope":
            check(isinstance(expression, Mapping) and set(expression) == {"operand_ids", "mode"} and isinstance(expression["mode"], str)
                  and isinstance(expression["operand_ids"], list) and expression["operand_ids"])
            ids = [_non_empty(i) for i in expression["operand_ids"]]
            check(all(ids))
            terms, operands = [], [graph.resolve(i, cases) for i in sorted(ids)]
        else:
            raise _fail("DERIVATION_EVIDENCE")
        for kind, oid in operands:
            check(oid in cases if kind == "case" else any(r["id"] == oid for r in records))
        records.append({"id": cid, "basis": basis, "operands": operands, "terms": terms})
    for i, record in enumerate(records):
        subject = record["basis"] in UNSIGNED_BASES or any(o[0] == "combination" for o in record["operands"])
        check(subject or any(("combination", record["id"]) in later["operands"] for later in records[i + 1:]))
    operand_records = [o for o in _items(evidence.get("combination_operands")) if isinstance(o, Mapping)]
    by_listed = {r["combination_id"]: r for r in listed}
    for stated in operand_records:
        record = by_listed.get(stated.get("combination_id"))
        if record is not None:
            check(stated.get("basis") == record["basis"] and _same(stated.get("expression"), record["expression"]))
    shows_nonlinear = bool(_items(evidence.get("combination_operands"))) or any(
        isinstance(r, Mapping) and r.get("kind") == STATE_CODE for r in _items(source.get("results")))
    check(nonlinear or not shows_nonlinear)
    return {"nonlinear": nonlinear, "consuming": consuming, "records": records}


def _cites_combination(row: Mapping[str, Any], by_id: dict, cid: str) -> bool:
    for ref in _items(row.get("source_result_refs")):
        src = by_id.get(ref) if isinstance(ref, str) else None
        other = _combination_of(src) if src is not None else None
        if other is not None and other != cid:
            return True
    return False


def check(source: Mapping[str, Any], model: Mapping[str, Any] | None = None) -> None:
    """The gates and unsigned rows of a solved preview-physics-2 document; ``model``, when supplied, is its model."""
    if (source.get("status") or {}).get("mechanics") != "MECHANICS_SOLVED":
        return
    evidence = source.get("contract_evidence") or {}
    gates = _items(evidence.get("combination_gates"))
    rows = [r for r in _items(source.get("results")) if isinstance(r, Mapping)]
    by_id = {r["id"]: r for r in rows if isinstance(r.get("id"), str)}
    if model is not None:
        _model_check(source, model, gates, rows, by_id)
    else:
        _free_check(source, gates, rows, by_id)


def _free_check(source: Mapping[str, Any], gates: list, rows: list, by_id: dict) -> None:
    derivation = _derivation(source, gates)
    records = {r["id"]: r for r in (derivation or {}).get("records", [])}
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
            _require(cid in records, "GATE_UNVERIFIED")
        withheld.add(cid)
    # Section 10.3: each recorded combination's gate, up to its modulus gate, from the records.
    if derivation is not None:
        combination_graph = graph.Graph()
        stated = {g.get("combination_id"): g.get("reason") for g in gates if isinstance(g, Mapping)}
        for record in derivation["records"]:
            cid, basis, operands, terms = record["id"], record["basis"], record["operands"], record["terms"]
            operand_kappa = [combination_graph.kappa_of(o) for o in operands]
            oq1 = basis == "mechanics" and combination_graph.solved_state_plus_increments(terms)
            own = combination_graph.add(cid, basis, operands, [f for _, f in terms])
            operand_withheld = any(o[0] == "combination" and o[1] in withheld for o in operands)
            derived = _gate_before_modulus(basis, operand_withheld, derivation["nonlinear"], oq1, derivation["consuming"], own, operand_kappa)
            reason = stated.get(cid)
            _require(reason == derived if derived is not None else reason in (None, MODULUS_MIXED), "GATE_DERIVATION")
            if derived is None and reason is None and basis in UNSIGNED_BASES:
                _unsigned_rows(rows, by_id, cid, basis, terms)
    # Every unsigned row and every row citing an operand combination's row has its record.
    for row in rows:
        cid = _combination_of(row)
        if cid is None:
            continue
        claim = _unsigned_claim(row)
        if claim is not None:
            claimed, consistent = claim
            _require(consistent, "UNSIGNED_ROW")
            _require(cid in records, "UNSIGNED_UNVERIFIED")
            _require(records[cid]["basis"] == claimed, "UNSIGNED_ROW")
        else:
            _require(cid in records or not _cites_combination(row, by_id, cid), "GATE_UNVERIFIED")


def _model_derivation(kept: list, order: list, cases: set[str], nonlinear: bool, consuming: bool) -> dict | None:
    """The ``combination_derivation`` the model implies; ``None`` when no kept combination is unsigned or nested."""
    def combination_operands(c: Mapping[str, Any]) -> list[str]:
        return [o[1] for o in graph.model_operands(c, cases) if o[0] == "combination"]
    ordered = [kept[i] for i in order]
    recorded = {c.get("id") for c in ordered if c.get("basis") in UNSIGNED_BASES or combination_operands(c)}
    if not recorded:
        return None
    for c in reversed(ordered):
        if c.get("id") in recorded:
            recorded.update(combination_operands(c))
    records = [{"combination_id": c.get("id"), "basis": c.get("basis"), "expression": graph.evidence_expression(c)}
               for c in ordered if c.get("id") in recorded]
    return {"nonlinear_supports": nonlinear, "consuming_constant_effort": consuming, "combinations": records}


def _same(a: Any, b: Any) -> bool:
    """JSON equality with numbers compared as binary64 values (a model's factor 1 and a document's 1.0 are one factor)."""
    number = lambda v: type(v) in (int, float)
    if number(a) and number(b):
        return _bits(float(a)) == _bits(float(b))
    if isinstance(a, list) and isinstance(b, list):
        return len(a) == len(b) and all(_same(x, y) for x, y in zip(a, b))
    if isinstance(a, Mapping) and isinstance(b, Mapping):
        return set(a) == set(b) and all(_same(a[k], b[k]) for k in a)
    return type(a) is type(b) and a == b


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


def _model_check(source: Mapping[str, Any], model: Mapping[str, Any], gates: list, rows: list, by_id: dict) -> None:
    evidence = source.get("contract_evidence") or {}
    # A document stating no combination at all makes no gate claim to re-derive.
    if not gates and not any(_combination_of(r) is not None for r in rows):
        _require(DERIVATION_MEMBER not in evidence, "DERIVATION_EVIDENCE")
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
    # Section 10.3: the published derivation is exactly the model's.
    expected = _model_derivation(kept, order, cases, nonlinear, consuming)
    if DERIVATION_MEMBER in evidence:
        _require(expected is not None and _same(evidence[DERIVATION_MEMBER], expected), "DERIVATION_EVIDENCE")
    else:
        _require(expected is None, "DERIVATION_EVIDENCE")
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
        operand_withheld = any(o[0] == "combination" and o[1] in withheld for o in operands)
        derived = _gate_before_modulus(basis, operand_withheld, nonlinear, oq1, consuming, own, operand_kappa)
        if derived is None and not operand_withheld and basis in graph.TERM_BASES:
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
