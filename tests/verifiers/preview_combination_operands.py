"""Operating-case slice 1a (DEL-05-02 Design ``pf-nonlinear-operand-standing.md`` §3): test-only mirror of RE
``preview_combination_operands.rs``. Re-derives the preview-physics-2 ``combination_operands`` statement of a model
with nonlinear supports and its two notices from the received document; it checks the statement's internal
consistency, not any case's standing (each operand's quality record is the document's own, compared bit for bit).

Independence: standard-library imports only, and its siblings ``section_basis`` (bit equality), ``units`` and
``combination_graph``.

a record may also state an absolute sum, an
SRSS, or a mechanics combination admitted by decision OQ-1 (re-derived from the earlier records); a term names a load
case or a combination; a combination operand is ``{combination_id}`` and states its standing in its own record.
"""
from __future__ import annotations

from collections.abc import Mapping
from typing import Any

from . import combination_graph as graph
from .section_basis import bit_equal
from .units import to_canonical

MEMBER = "combination_operands"
STATES_DIFFER = "COMBINATION_OPERAND_SUPPORT_STATES_DIFFER"
NOT_PASSED = "COMBINATION_OPERAND_CHECKS_NOT_PASSED"
STATE_CODE = "nonlinear_support_active_set_state_code"
STANDING = "derived_from_case_results"
SOURCE = "core/product_physics"
RECORD_KEYS = {"combination_id", "basis", "expression", "operands", "standing"}
OPERAND_KEYS = {"load_case_id", "modulus_basis_ref", "modulus_basis_temperature", "numerical_quality"}
NAMED = 8
LINEAR_BASIS = "explicit_user_linear_combination"
MODES = ("min", "max", "min_abs", "max_abs")
DISPLACEMENT = "nonlinear_support_final_displacement"
RELATIVE_DISPLACEMENT = "nonlinear_support_final_relative_displacement"
MODULUS_RECORD = "combination_modulus_basis_record"
SUPPORT_COMPONENT = "support_reaction_component_v2"
SUPPORT_COMPONENTS = ("Fx", "Fy", "Fz", "Mx", "My", "Mz")
DISPLACEMENT_COMPONENTS = ("global_nodal_displacement_x", "global_nodal_displacement_y", "global_nodal_displacement_z")
STATE_TOKENS = {0: "inactive", 1: "active", 2: "sticking", 3: "sliding"}


def _fail(code: str) -> ValueError:
    return ValueError(f"SOURCE_PREVIEW_PHYSICS_COMBINATION_OPERANDS_{code}")


def _require(ok: Any, code: str) -> None:
    if not ok:
        raise _fail(code)


def _items(value: Any) -> list:
    return value if isinstance(value, list) else []


def _text(value: Any) -> bool:
    return isinstance(value, str) and bool(value)


def _keys(value: Any, keys: set[str]) -> bool:
    return isinstance(value, Mapping) and set(value) == keys


def _identity(text: str) -> str:
    return f"{len(text.encode('utf-8'))}:{text}"


def _basis(row: Mapping[str, Any], ref_type: str) -> str | None:
    basis = row.get("basis_ref")
    if isinstance(basis, Mapping) and basis.get("ref_type") == ref_type and isinstance(basis.get("ref_id"), str):
        return basis["ref_id"]
    return None


def _evaluation_order(basis: Any, expression: Any, cases: set[str]) -> tuple[list[tuple[str, str]], list[float]]:
    """Terms in order; minuend then subtrahend; a range envelope's ids sorted (code-point order is UTF-8 byte order).
    An id names the load case when one has it, a combination otherwise."""
    if basis in graph.TERM_BASES:
        _require(_keys(expression, {"terms"}) and isinstance(expression["terms"], list) and expression["terms"], "SHAPE")
        operands, factors = [], []
        for term in expression["terms"]:
            parsed = graph.evidence_term(term)
            _require(parsed is not None and parsed[0] not in operands, "SHAPE")
            operands.append(parsed[0])
            factors.append(parsed[1])
        return operands, factors
    if basis == "result_state_subtraction":
        _require(_keys(expression, {"minuend_id", "subtrahend_id"}), "SHAPE")
        a, b = expression["minuend_id"], expression["subtrahend_id"]
        _require(_text(a) and _text(b) and a != b, "SHAPE")
        return [graph.resolve(a, cases), graph.resolve(b, cases)], []
    if basis == "range_envelope":
        _require(_keys(expression, {"operand_ids", "mode"}) and expression["mode"] in MODES, "SHAPE")
        ids = expression["operand_ids"]
        _require(isinstance(ids, list) and ids and all(_text(i) for i in ids), "SHAPE")
        ordered = sorted(ids)
        _require(len(set(ordered)) == len(ordered), "SHAPE")
        return [graph.resolve(i, cases) for i in ordered], []
    raise _fail("SHAPE")


ROW_BASES = {"range_envelope": ("explicit_user_range_envelope", None),
             "result_state_subtraction": ("explicit_user_result_state_subtraction", None),
             "absolute_sum": (graph.ABSOLUTE_SUM_BASIS, graph.ABSOLUTE_SUM_SIGN),
             "srss": (graph.SRSS_BASIS, graph.SRSS_SIGN),
             "mechanics": (LINEAR_BASIS, graph.SOLVED_STATE_PLUS_INCREMENTS_SIGN)}


def _notice(id_: str, code: str, severity: str, message: str, refs: list[str]) -> dict[str, Any]:
    return {"id": id_, "code": code, "severity": severity, "message": message, "source": SOURCE, "affected_refs": refs}


def _states_notice(cid: str, operands: list[str], rows: list) -> dict[str, Any] | None:
    states: list[tuple[str, str, float]] = []
    for row in rows:
        if not isinstance(row, Mapping) or row.get("kind") != STATE_CODE:
            continue
        case, support, value = _basis(row, "load_case"), row.get("entity_ref"), row.get("value")
        _require(case is not None and _text(support), "STATE_CODE")
        _require(type(value) in (int, float) and value in (0, 1, 2, 3), "STATE_CODE")
        _require(not any(c == case and s == support for c, s, _ in states), "STATE_CODE")
        states.append((case, support, float(value)))
    supports: list[str] = []
    for case in operands:
        for c, s, _ in states:
            if c == case and s not in supports:
                supports.append(s)
    def sense(case: str, support: str) -> float | None:
        for kind in (RELATIVE_DISPLACEMENT, DISPLACEMENT):
            row = next((r for r in rows if isinstance(r, Mapping) and r.get("kind") == kind and _basis(r, "load_case") == case
                        and r.get("entity_ref") == support), None)
            if row is not None and type(row.get("value")) in (int, float):
                return row["value"]
        return None

    def reported(code: float | None, slip: float | None) -> str:
        if code is None:
            return "unreported"
        if code == 3:
            return ("sliding (positive slip)" if slip is not None and slip > 0
                    else "sliding (negative slip)" if slip is not None and slip < 0 else "sliding (slip sense unavailable)")
        return STATE_TOKENS[int(code)]

    differing: list[tuple[str, list[str]]] = []
    for support in supports:
        tokens = [reported(next((v for c, s, v in states if c == case and s == support), None), sense(case, support)) for case in operands]
        if not all(t == tokens[0] and t != "unreported" and "unavailable" not in t for t in tokens):
            differing.append((support, tokens))
    if not differing:
        return None
    named = "; ".join(f"{support} ({', '.join(f'{case}: {state}' for case, state in zip(operands, tokens))})"
                      for support, tokens in differing[:NAMED])
    if len(differing) > NAMED:
        named += f"; and {len(differing) - NAMED} more"
    message = (f"combination {cid}: the reported final state of {len(differing)} nonlinear support(s) differs between its operand cases: {named}. "
               "Each operand case is solved independently from the model's authored initial support states, under the static, "
               "history-free support model, and keeps its own reported support state; the combination's rows are derived from "
               "those case results and are not one solved state.")
    return _notice(f"diagnostic:preview-physics:combination-operand-states:{_identity(cid)}", STATES_DIFFER, "info", message,
                   [cid] + [support for support, _ in differing])


def _standing_notice(cid: str, operands: list) -> dict[str, Any] | None:
    failing = []
    for operand in operands:
        if not isinstance(operand, Mapping) or "load_case_id" not in operand or "support_state" in operand:
            continue
        quality = operand.get("numerical_quality") if isinstance(operand.get("numerical_quality"), Mapping) else {}
        solve = quality.get("solve_quality") if isinstance(quality.get("solve_quality"), str) else "not_assessed"
        if solve != "checks_passed":
            failing.append((operand.get("load_case_id"), solve))
    if not failing:
        return None
    named = ", ".join(f"{case} ({quality})" for case, quality in failing)
    message = (f"combination {cid} uses operand case(s) whose numerical integrity checks did not pass: {named}; its rows are "
               "still published, as derived inspection results without their own numerical enclosure; each operand's standing "
               "is stated in contract_evidence.combination_operands")
    return _notice(f"diagnostic:preview-physics:combination-operand-standing:{_identity(cid)}", NOT_PASSED, "warning", message,
                   [cid] + [case for case, _ in failing])


def check(source: Mapping[str, Any], solved: bool) -> None:
    """presence, shape, standing, row binding and both notices; a blocked envelope carries neither."""
    evidence = source.get("contract_evidence") or {}
    member = evidence.get(MEMBER) if MEMBER in evidence else None
    rows = _items(source.get("results"))
    diagnostics = _items(source.get("diagnostics"))
    nonlinear = any(isinstance(r, Mapping) and r.get("kind") == STATE_CODE for r in rows)
    admitted = [g["combination_id"] for g in _items(evidence.get("combination_gates"))
                if isinstance(g, Mapping) and g.get("withheld") is False and isinstance(g.get("combination_id"), str)]
    _require((MEMBER in evidence) == (solved and nonlinear and bool(admitted)), "PRESENCE")
    expected: list[dict[str, Any]] = []
    quality = _items((source.get("numerical_quality") or {}).get("cases"))
    cases = {q["basis_ref"]["ref_id"] for q in quality if isinstance(q, Mapping) and isinstance(q.get("basis_ref"), Mapping)
             and q["basis_ref"].get("ref_type") == "load_case" and isinstance(q["basis_ref"].get("ref_id"), str)}
    recorded = graph.Graph()
    # In a solved nonlinear document a linear-combination row belongs to a recorded mechanics combination, which
    # decision OQ-1 admitted (checked on its record below).
    if solved and nonlinear:
        mechanics = {r.get("combination_id") for r in (member if isinstance(member, list) else [])
                     if isinstance(r, Mapping) and r.get("basis") == "mechanics"}
        _require(not any(isinstance(r, Mapping) and _basis(r, "combination") is not None and _basis(r, "combination") not in mechanics
                         and isinstance(r.get("metadata"), Mapping) and r["metadata"].get("basis") == LINEAR_BASIS for r in rows),
                 "MECHANICS_ADMITTED")
    if MEMBER in evidence:
        _require(isinstance(member, list), "SHAPE")
        _require(len(member) == len(admitted), "PRESENCE")
        by_id = {r["id"]: r for r in rows if isinstance(r, Mapping) and isinstance(r.get("id"), str)}
        for record, gate_id in zip(member, admitted):
            _require(_keys(record, RECORD_KEYS) and record["standing"] == STANDING, "SHAPE")
            _require(record["combination_id"] == gate_id, "PRESENCE")
            cid = gate_id
            basis = record["basis"]
            order, factors = _evaluation_order(basis, record["expression"], cases)
            # In a solved nonlinear document a mechanics combination is admitted only by decision OQ-1.
            if basis == "mechanics":
                _require(recorded.solved_state_plus_increments(list(zip(order, factors))), "MECHANICS_ADMITTED")
            operands = record["operands"]
            _require(isinstance(operands, list) and len(operands) == len(order), "SHAPE")
            # RT-A5 (operating-case section 7.6): a minuend_final subtraction's subtrahend is held; its standing is the
            # held solve's (held_standing, bound by preview_held_status), never the case's numerical quality.
            from .preview_held_status import HELD_SUPPORT_STATE, is_held_row, held_standing_of
            held_position = None
            for position, (operand, (kind, case)) in enumerate(zip(operands, order)):
                if isinstance(operand, Mapping) and "support_state" in operand:
                    standing = operand.get("held_standing")
                    _require(kind == "case" and basis == "result_state_subtraction" and position == 1
                             and _keys(operand, OPERAND_KEYS | {"support_state", "held_standing"}) and operand["load_case_id"] == case
                             and operand["support_state"] == HELD_SUPPORT_STATE and operand["numerical_quality"] is None
                             and _keys(standing, {"kind", "reason"}) and _text(standing["kind"]) and _text(standing["reason"])
                             and held_standing_of(source, cid) == standing, "SHAPE")
                    held_position = position
                    continue
                if kind == "combination":
                    _require(bit_equal(operand, {"combination_id": case}) and recorded.knows(case), "SHAPE")
                    continue
                _require(_keys(operand, OPERAND_KEYS) and operand["load_case_id"] == case, "SHAPE")
                _require(operand["modulus_basis_ref"] is None or _text(operand["modulus_basis_ref"]), "SHAPE")
                t = operand["modulus_basis_temperature"]
                _require(t is None or (_keys(t, {"value", "unit"}) and type(t["value"]) in (int, float) and _text(t["unit"])), "SHAPE")
                stated = next((q for q in quality if isinstance(q, Mapping) and isinstance(q.get("basis_ref"), Mapping)
                               and q["basis_ref"].get("ref_type") == "load_case" and q["basis_ref"].get("ref_id") == case), None)
                _require(stated is not None and bit_equal(stated, operand["numerical_quality"]), "STANDING")
            recorded.add(cid, basis, order, factors)
            # Rows: references by row kind and basis.
            row_basis, sign = ROW_BASES[basis]
            for row in rows:
                if not isinstance(row, Mapping) or _basis(row, "combination") != cid or row.get("kind") == MODULUS_RECORD:
                    continue
                if held_position is not None and is_held_row(row):
                    continue  # the held system's row (preview_held_status)
                metadata = row.get("metadata")
                _require(metadata is None or (isinstance(metadata, Mapping) and metadata.get("basis") == row_basis), "ROWS")
                _require(metadata is None or sign is None or metadata.get("sign_convention") == sign, "ROWS")
                refs = row.get("source_result_refs")
                refs = refs if isinstance(refs, list) else []
                kind = row.get("kind")
                components = None
                if basis != "range_envelope" and kind == "displacement_magnitude":
                    components = [(k, None) for k in DISPLACEMENT_COMPONENTS]
                elif basis != "range_envelope" and kind in ("support_reaction_force_magnitude_v2", "support_reaction_moment_magnitude_v2"):
                    components = [(SUPPORT_COMPONENT, c) for c in SUPPORT_COMPONENTS]
                if components is not None:
                    # Recomputed magnitudes name this combination's component rows, in order.
                    _require(len(refs) == len(components), "ROWS")
                    for ref, (component_kind, component) in zip(refs, components):
                        source = by_id.get(ref) if isinstance(ref, str) else None
                        _require(source is not None and _basis(source, "combination") == cid and source.get("kind") == component_kind
                                 and source.get("entity_ref") == row.get("entity_ref")
                                 and (component is None or (source.get("metadata") or {}).get("component") == component), "ROWS")
                else:
                    # Direct rows name one matching operand row per operand, in evaluation order.
                    _require(len(refs) == len(order), "ROWS")
                    for position, (ref, operand) in enumerate(zip(refs, order)):
                        source = by_id.get(ref) if isinstance(ref, str) else None
                        owned = source is not None and ((_basis(source, "combination") == cid and is_held_row(source))
                                                        if held_position == position else graph.owns(operand, source))
                        _require(owned and source.get("kind") == kind
                                 and source.get("entity_ref") == row.get("entity_ref") and source.get("unit") == row.get("unit")
                                 and (source.get("metadata") or {}).get("component") == (metadata or {}).get("component")
                                 and (source.get("metadata") or {}).get("location") == (metadata or {}).get("location"), "ROWS")
            # A held operand's comparison is HELD_STATUS_RELEASED's, not this notice's.
            order_cases = [case for position, (kind, case) in enumerate(order) if kind == "case" and position != held_position]
            for notice in (_states_notice(cid, order_cases, rows), _standing_notice(cid, operands)):
                if notice is not None:
                    expected.append(notice)
    stated = [d for d in diagnostics if isinstance(d, Mapping) and d.get("code") in (STATES_DIFFER, NOT_PASSED)]
    _require(len(stated) == len(expected), "NOTICE")
    for notice in expected:
        _require(sum(1 for d in stated if bit_equal(d, notice)) == 1, "NOTICE")


def model_checks(source: Mapping[str, Any], model: Mapping[str, Any]) -> None:
    """Section 3 with a supplied model: when the model has a nonlinear support the records follow the model's
    evaluation order (absolute-sum-srss section 2), each binding its id, basis, expression and operand selections to
    the model's, and every subtraction or range envelope whose operand combinations are all recorded is recorded; with
    no nonlinear support there is none."""
    if (source.get("status") or {}).get("mechanics") != "MECHANICS_SOLVED":
        return
    nonlinear = any(isinstance(s, Mapping) and s.get("nonlinear") is not None for s in _items(model.get("supports")))
    combinations = model.get("combinations")
    combinations = [] if combinations is None else combinations
    _require(isinstance(combinations, list), "MODEL")
    records = _items((source.get("contract_evidence") or {}).get(MEMBER))
    if not nonlinear:
        _require(not records, "MODEL")
        return
    cases = _items(model.get("load_cases"))
    case_ids = {c.get("id") for c in cases if isinstance(c, Mapping) and isinstance(c.get("id"), str)}
    order = graph.model_order(combinations, case_ids)
    recorded = {r.get("combination_id") for r in records if isinstance(r, Mapping)}
    position = 0
    for record in records:
        at = next((k for k, i in enumerate(order[position:]) if record.get("combination_id") == combinations[i].get("id")), None)
        _require(at is not None, "MODEL")
        combination = combinations[order[position + at]]
        position += at + 1
        _require(record.get("basis") == combination.get("basis") and bit_equal(record.get("expression"), graph.evidence_expression(combination)), "MODEL")
        for operand in _items(record.get("operands")):
            if isinstance(operand, Mapping) and "combination_id" in operand:
                continue
            case = next((c for c in cases if isinstance(c, Mapping) and c.get("id") == operand.get("load_case_id")), None)
            _require(case is not None, "MODEL")
            temperature = case.get("modulus_basis_temperature")
            if temperature is not None:
                try:
                    kelvin = to_canonical(temperature.get("value"), temperature.get("unit"), "temperature")
                except (ValueError, TypeError, AttributeError) as exc:
                    raise _fail("MODEL") from exc
                temperature = {"value": kelvin, "unit": "K"}
            _require(bit_equal(operand.get("modulus_basis_ref"), case.get("modulus_basis_ref"))
                     and bit_equal(operand.get("modulus_basis_temperature"), temperature), "MODEL")
    # Operating-case slice 1b: a subtraction a gate withholds, or the interim withholding removed, has no record.
    admitted = {g.get("combination_id") for g in _items((source.get("contract_evidence") or {}).get("combination_gates"))
                if isinstance(g, Mapping) and g.get("withheld") is False}
    for index in order:
        combination = combinations[index]
        if combination.get("basis") in ("result_state_subtraction", "range_envelope") and combination.get("id") in admitted and all(
                kind == "case" or oid in recorded for kind, oid in graph.model_operands(combination, case_ids)):
            _require(combination.get("id") in recorded, "MODEL")
