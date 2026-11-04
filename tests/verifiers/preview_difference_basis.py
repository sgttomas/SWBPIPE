"""Operating-case slice 1b (DEL-05-02 Work operating-case.md section 1.2 and "As built (1b)"; DEL-02-01 Work
model-document-0.9.0.md section 2.2): the Python mirror of RE ``preview_difference_basis.rs``.

A declared operating comparison (``result_state_subtraction`` with ``difference_basis = "operating_minus_sustained"``)
on preview-physics-2: its gates G-1 (``EXPANSION_BASIS_SUSTAINED_NOT_IN_OPERATING``) and G-2
(``EXPANSION_BASIS_MODULUS_MIXED``), its rows' sign convention and its G-1 derivation evidence
(``contract_evidence.difference_basis_derivation``).

- With the model: after the member passes the same shape, count-domain, binding and recurrence checks as the
  model-free reading, G-1 is re-derived from the model's own primitive loads (category, target, direction, dimension,
  magnitude normalized by this package's independent unit table, value bits and canonical unit); the published
  evidence must equal it, and the gate must be G-1's, else G-2's from the cases' modulus-basis keys. There is no
  condition resolver here (decision M1): when either operand case carries a ``condition_use`` the verdict is refused
  as unverifiable (``SOURCE_PREVIEW_PHYSICS_COMBINATION_GATE_UNVERIFIED``), never read model-free instead.
- Without the model: fail closed by default (``MODEL_FREE = "refuse"``, option C) until the independent contract
  review of option A clears. Under ``mode="verify"`` (option A) the evidence's closed shape, canonical order, counts
  (an integer from 1 to 2^53 - 1, ``MAX_COUNT``) and bindings are checked, a case id recurring across records must
  carry byte-identical contents, G-1 is re-derived from its multisets and G-2 from the published modulus-basis records.
  Scope: that checks the record's internal derivation and its binding to the document, not its truth to the absent
  model; a coherent forgery cannot be authenticated without the model.
"""
from __future__ import annotations

import json
import math
import struct
from collections.abc import Mapping
from typing import Any

from . import combination_graph as graph
from .units import to_canonical

MEMBER = "difference_basis_derivation"
SUSTAINED_NOT_IN_OPERATING = "EXPANSION_BASIS_SUSTAINED_NOT_IN_OPERATING"
MODULUS_MIXED = "EXPANSION_BASIS_MODULUS_MIXED"
SIGN = ("positive value follows explicit user result-state subtraction (minuend minus subtrahend) of matching source "
        "result sign conventions; operating minus sustained difference: a signed difference of two independently solved "
        "states, each using its own reported support state and carrying its own numerical standing under the static, "
        "history-free friction model; derived from case results; not an equilibrium state")
TOKEN = "operating_minus_sustained"
SUBTRACTION_BASIS = "explicit_user_result_state_subtraction"
MODULUS_RECORD = "combination_modulus_basis_record"
UNVERIFIED = "SOURCE_PREVIEW_PHYSICS_COMBINATION_GATE_UNVERIFIED"
GATE_DERIVATION = "SOURCE_PREVIEW_PHYSICS_COMBINATION_GATE_DERIVATION"
EVIDENCE = "SOURCE_PREVIEW_PHYSICS_DIFFERENCE_BASIS_EVIDENCE"
BINDING = "SOURCE_PREVIEW_PHYSICS_DIFFERENCE_BASIS_BINDING"
ROW = "SOURCE_PREVIEW_PHYSICS_DIFFERENCE_BASIS_ROW"
MODEL = "SOURCE_PREVIEW_PHYSICS_DIFFERENCE_BASIS_MODEL"
# The count domain: an integer from 1 to 2^53 - 1. The producer counts a case's primitive loads, and a published
# statement is a checked-JSON document (openpipestress_jcs_ijson_v1: integers within +-(2^53 - 1)), so no larger count
# can be published; a float (1.0), a boolean, a string or a negative number is not a count. The Rust reader states the
# same domain (count_of).
MAX_COUNT = 2**53 - 1
# Fail closed until the independent contract review of option A clears ("refuse" is option C, "verify" option A).
MODEL_FREE = "refuse"
# The producer's expected_load_dimension: a load's dimension token and the unit table's dimension.
LOAD_DIMENSIONS = {"force": "force", "moment": "moment", "force_per_length": "force_per_length", "pressure": "pressure",
                   "temperature_change": "temperature_interval", "temperature_interval": "temperature_interval",
                   "acceleration": "acceleration", "displacement": "displacement", "rotation": "rotation"}


def _require(condition: Any, code: str) -> None:
    if not condition:
        raise ValueError(code)


def _items(value: Any) -> list:
    return value if isinstance(value, list) else []


def _keys(value: Any, keys: set[str]) -> bool:
    return isinstance(value, Mapping) and set(value) == keys


def _str(value: Any) -> bool:
    return isinstance(value, str)


def _combination_of(row: Any) -> str | None:
    basis = row.get("basis_ref") if isinstance(row, Mapping) else None
    return basis.get("ref_id") if isinstance(basis, Mapping) and basis.get("ref_type") == "combination" else None


def _bits(value: float) -> str:
    return f"{struct.unpack('>Q', struct.pack('>d', value))[0]:016x}"


def _from_bits(text: str) -> float:
    return struct.unpack(">d", struct.pack(">Q", int(text, 16)))[0]


def _target_key(target: Any) -> tuple[str, str, str] | None:
    if not isinstance(target, Mapping):
        return None
    kind = target.get("type")
    if kind == "node" and _keys(target, {"type", "node"}) and _str(target["node"]):
        return ("node", target["node"], "")
    if kind == "element" and _keys(target, {"type", "pipe"}) and _str(target["pipe"]):
        return ("element", target["pipe"], "")
    if kind == "support" and _keys(target, {"type", "support", "dof"}) and _str(target["support"]) and _str(target["dof"]):
        return ("support", target["support"], target["dof"])
    return None


def _target_json(key: tuple) -> dict:
    if key[1] == "support":
        return {"type": "support", "support": key[2], "dof": key[3]}
    if key[1] == "element":
        return {"type": "element", "pipe": key[2]}
    return {"type": "node", "node": key[2]}


def _contained(minuend: dict, subtrahend: dict) -> bool:
    return all(minuend.get(key, 0) >= count for key, count in subtrahend.items())


def _read_multiset(entries: Any) -> dict:
    """Closed entries, strictly increasing in the canonical order (each content once), integral counts >= 1 and finite
    values. The canonical order: byte-wise over category, target type, target ids, direction, dimension, value bits
    (16 lowercase hexadecimal digits), unit."""
    _require(isinstance(entries, list), EVIDENCE)
    out: dict = {}
    last = None
    for entry in entries:
        _require(_keys(entry, {"category", "target", "direction", "dimension", "magnitude", "count"}), EVIDENCE)
        target = _target_key(entry["target"])
        _require(target is not None, EVIDENCE)
        magnitude = entry["magnitude"]
        _require(_keys(magnitude, {"value_bits", "unit"}), EVIDENCE)
        hexed, unit = magnitude["value_bits"], magnitude["unit"]
        _require(_str(hexed) and len(hexed) == 16 and all(c in "0123456789abcdef" for c in hexed) and _str(unit), EVIDENCE)
        _require(math.isfinite(_from_bits(hexed)), EVIDENCE)
        count = entry["count"]
        _require(type(count) is int and 1 <= count <= MAX_COUNT, EVIDENCE)
        _require(all(_str(entry[k]) for k in ("category", "direction", "dimension")), EVIDENCE)
        key = (entry["category"], target[0], target[1], target[2], entry["direction"], entry["dimension"], hexed, unit)
        _require(last is None or last < key, EVIDENCE)
        last = key
        out[key] = count
    return out


def _multiset_json(multiset: dict) -> list:
    return [{"category": k[0], "target": _target_json(k), "direction": k[4], "dimension": k[5],
             "magnitude": {"value_bits": k[6], "unit": k[7]}, "count": count} for k, count in sorted(multiset.items())]


def _read(source: Mapping[str, Any]) -> list[dict]:
    evidence = source.get("contract_evidence") or {}
    if MEMBER not in evidence:
        return []
    member = evidence[MEMBER]
    _require(isinstance(member, list) and member, EVIDENCE)
    gates = [g for g in _items(evidence.get("combination_gates")) if isinstance(g, Mapping)]
    position = {g.get("combination_id"): i for i, g in enumerate(gates)}
    cases = {c.get("load_case_id") for c in _items(evidence.get("preview_cases")) if isinstance(c, Mapping)}
    records, last = [], None
    for record in member:
        _require(_keys(record, {"combination_id", "difference_basis", "minuend", "subtrahend"}) and record["difference_basis"] == TOKEN, EVIDENCE)
        cid = record["combination_id"]
        _require(_str(cid) and cid, EVIDENCE)
        _require(cid in position, BINDING)
        _require(last is None or position[cid] > last, EVIDENCE)
        last = position[cid]
        sides = {}
        for side in ("minuend", "subtrahend"):
            value = record[side]
            _require(_keys(value, {"load_case_id", "primitive_contents"}), EVIDENCE)
            _require(_str(value["load_case_id"]) and value["load_case_id"] in cases, BINDING)
            sides[side] = (value["load_case_id"], _read_multiset(value["primitive_contents"]))
        _require(sides["minuend"][0] != sides["subtrahend"][0], BINDING)
        records.append({"id": cid, "minuend": sides["minuend"][0], "subtrahend": sides["subtrahend"][0],
                        "m": sides["minuend"][1], "s": sides["subtrahend"][1]})
    # Review r1 P1: one producer resolution gives each case one multiset, so a case id that recurs (in any record, as
    # minuend or subtrahend) carries byte-identical contents: the same JSON, members sorted (as the Rust reader's
    # serialization), so member order alone is not a difference.
    bound: dict = {}
    for record in member:
        for side in ("minuend", "subtrahend"):
            text = json.dumps(record[side]["primitive_contents"], ensure_ascii=False, separators=(",", ":"), sort_keys=True)
            _require(bound.setdefault(record[side]["load_case_id"], text) == text, BINDING)
    return records


def _derived_gate(contained: bool, same_modulus: bool) -> str | None:
    if not contained:
        return SUSTAINED_NOT_IN_OPERATING
    return None if same_modulus else MODULUS_MIXED


def _require_gate(gates: list, cid: str, derived: str | None) -> None:
    gate = next((g for g in gates if g.get("combination_id") == cid), None)
    _require(gate is not None and gate.get("withheld") is (derived is not None) and gate.get("reason") == derived, GATE_DERIVATION)


def _rows_sign(rows: list, declared: set) -> None:
    for row in rows:
        cid = _combination_of(row)
        if cid is None or row.get("kind") == MODULUS_RECORD:
            continue
        metadata = row.get("metadata") if isinstance(row.get("metadata"), Mapping) else {}
        sign = metadata.get("sign_convention")
        if cid in declared:
            _require(sign is None or (sign == SIGN and metadata.get("basis") == SUBTRACTION_BASIS), ROW)
        else:
            _require(sign != SIGN, ROW)


def check(source: Mapping[str, Any], model: Mapping[str, Any] | None = None, mode: str | None = None) -> None:
    """A solved preview-physics-2 document's declared subtractions; ``mode`` defaults to ``MODEL_FREE``."""
    if model is not None:
        _model_check(source, model)
    else:
        _free_check(source, MODEL_FREE if mode is None else mode)


def _free_check(source: Mapping[str, Any], mode: str) -> None:
    evidence = source.get("contract_evidence") or {}
    gates = [g for g in _items(evidence.get("combination_gates")) if isinstance(g, Mapping)]
    rows = [r for r in _items(source.get("results")) if isinstance(r, Mapping)]
    def sign_of(row):
        return (row.get("metadata") or {}).get("sign_convention") if isinstance(row.get("metadata"), Mapping) else None
    claimed = (MEMBER in evidence or any(g.get("reason") in (SUSTAINED_NOT_IN_OPERATING, MODULUS_MIXED) for g in gates)
               or any(_combination_of(r) is not None and sign_of(r) == SIGN for r in rows))
    if not claimed:
        return
    _require(mode == "verify", UNVERIFIED)
    records = _read(source)
    declared = {r["id"] for r in records}
    for gate in gates:
        if gate.get("reason") in (SUSTAINED_NOT_IN_OPERATING, MODULUS_MIXED):
            _require(gate.get("combination_id") in declared, EVIDENCE)
    for row in rows:
        if _combination_of(row) is not None and sign_of(row) == SIGN:
            _require(_combination_of(row) in declared, EVIDENCE)
    by_id = {r["id"]: r for r in rows if _str(r.get("id"))}
    for record in records:
        cid = record["id"]
        modulus = [(r.get("entity_ref"), (r.get("metadata") or {}).get("basis")) for r in rows
                   if r.get("kind") == MODULUS_RECORD and _combination_of(r) == cid]
        _require(len(modulus) == 2 and modulus[0][0] == record["minuend"] and modulus[1][0] == record["subtrahend"], BINDING)
        for row in rows:
            if _combination_of(row) != cid or row.get("kind") == MODULUS_RECORD:
                continue
            sources = []
            for ref in _items(row.get("source_result_refs")):
                _require(_str(ref) and ref in by_id, BINDING)
                sources.append(by_id[ref])
            owners = [(s.get("basis_ref") or {}).get("ref_id") if (s.get("basis_ref") or {}).get("ref_type") == "load_case" else None
                      for s in sources if _combination_of(s) != cid]
            _require(all(o in (record["minuend"], record["subtrahend"]) for o in owners), BINDING)
            if len(owners) == 2:
                _require(owners == [record["minuend"], record["subtrahend"]], BINDING)
        expression = {"minuend_id": record["minuend"], "subtrahend_id": record["subtrahend"]}
        for member in ("combination_operands", "combination_derivation"):
            stated = evidence.get(member)
            listed = _items(stated.get("combinations")) if isinstance(stated, Mapping) else _items(stated)
            for r in listed:
                if isinstance(r, Mapping) and r.get("combination_id") == cid:
                    _require(r.get("basis") == "result_state_subtraction" and r.get("expression") == expression, BINDING)
        _require_gate(gates, cid, _derived_gate(_contained(record["m"], record["s"]), modulus[0][1] == modulus[1][1]))
    _rows_sign(rows, declared)


def _model_multiset(case: Mapping[str, Any]) -> dict:
    out: dict = {}
    for load in _items(case.get("primitive_loads")):
        _require(isinstance(load, Mapping), MODEL)
        target = _target_key(load.get("target"))
        _require(target is not None and all(_str(load.get(k)) for k in ("category", "direction", "dimension")), MODEL)
        magnitude = load.get("magnitude") if isinstance(load.get("magnitude"), Mapping) else {}
        dimension = LOAD_DIMENSIONS.get(load["dimension"])
        if dimension is not None:
            try:
                value = to_canonical(magnitude.get("value"), magnitude.get("unit"), dimension)
            except (ValueError, TypeError) as exc:
                raise ValueError(MODEL) from exc
            from .units import CANONICAL
            unit = CANONICAL[dimension][0]
        else:
            value, unit = magnitude.get("value"), magnitude.get("unit")
            _require(type(value) in (int, float) and _str(unit), MODEL)
            value = float(value)
        _require(math.isfinite(value), MODEL)
        key = (load["category"], target[0], target[1], target[2], load["direction"], load["dimension"], _bits(value), unit)
        out[key] = out.get(key, 0) + 1
    return out


def kept_combinations(model: Mapping[str, Any]) -> tuple[list, list]:
    """The combinations the solve gates, in evaluation order: S1's interim withholding (as slice 1b leaves it) removes
    a ``minuend_final`` combination and every combination naming one."""
    combinations = [c for c in _items(model.get("combinations")) if isinstance(c, Mapping)]
    cases = {c.get("id") for c in _items(model.get("load_cases")) if isinstance(c, Mapping) and _str(c.get("id"))}
    removed: set = set()
    for index in graph.model_order(combinations, cases):
        c = combinations[index]
        if c.get("subtrahend_support_state") == "minuend_final" or any(o[0] == "combination" and o[1] in removed for o in graph.model_operands(c, cases)):
            removed.add(c.get("id"))
    kept = [c for c in combinations if c.get("id") not in removed]
    return kept, graph.model_order(kept, cases)


def declared(combination: Mapping[str, Any]) -> bool:
    return combination.get("basis") == "result_state_subtraction" and combination.get("difference_basis") == TOKEN


def model_gate(model: Mapping[str, Any], combination: Mapping[str, Any]) -> tuple[str | None, dict]:
    from .preview_combination_gates import _modulus_key
    cases = [c for c in _items(model.get("load_cases")) if isinstance(c, Mapping)]
    def case(key):
        found = next((c for c in cases if c.get("id") == combination.get(key)), None)
        _require(found is not None, MODEL)
        return found
    minuend, subtrahend = case("minuend_id"), case("subtrahend_id")
    _require(all(c.get("condition_use") is None for c in (minuend, subtrahend)), UNVERIFIED)
    m, s = _model_multiset(minuend), _model_multiset(subtrahend)
    record = {"combination_id": combination.get("id"), "difference_basis": TOKEN,
              "minuend": {"load_case_id": minuend["id"], "primitive_contents": _multiset_json(m)},
              "subtrahend": {"load_case_id": subtrahend["id"], "primitive_contents": _multiset_json(s)}}
    return _derived_gate(_contained(m, s), _modulus_key(minuend) == _modulus_key(subtrahend)), record


def _model_check(source: Mapping[str, Any], model: Mapping[str, Any]) -> None:
    # The member's closed shape, count domain, bindings and case recurrence, by the one validator the model-free
    # reading uses, before any comparison with the model's evidence (dict equality would take True or 1.0 for 1).
    _read(source)
    evidence = source.get("contract_evidence") or {}
    gates = [g for g in _items(evidence.get("combination_gates")) if isinstance(g, Mapping)]
    kept, order = kept_combinations(model)
    expected, ids = [], set()
    for index in order:
        combination = kept[index]
        if not declared(combination):
            continue
        derived, record = model_gate(model, combination)
        _require_gate(gates, combination.get("id"), derived)
        expected.append(record)
        ids.add(combination.get("id"))
    if MEMBER in evidence:
        _require(expected and evidence[MEMBER] == expected, MODEL)
    else:
        _require(not expected, MODEL)
    _rows_sign([r for r in _items(source.get("results")) if isinstance(r, Mapping)], ids)
