"""Operating-case slice 1a (DEL-05-02 Design ``pf-nonlinear-operand-standing.md`` §3): test-only mirror of RE
``preview_combination_operands.rs``. Re-derives the preview-physics-2 ``combination_operands`` statement of a model
with nonlinear supports and its two notices from the received document; it checks the statement's internal
consistency, not any case's standing (each operand's quality record is the document's own, compared bit for bit).

Independence: standard-library imports only, and its siblings ``section_basis`` (bit equality) and ``units``.
"""
from __future__ import annotations

from collections.abc import Mapping
from typing import Any

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


def _evaluation_order(basis: Any, expression: Any) -> list[str]:
    """Minuend then subtrahend; a range envelope's ids sorted (code-point order is UTF-8 byte order)."""
    if basis == "result_state_subtraction":
        _require(_keys(expression, {"minuend_id", "subtrahend_id"}), "SHAPE")
        a, b = expression["minuend_id"], expression["subtrahend_id"]
        _require(_text(a) and _text(b) and a != b, "SHAPE")
        return [a, b]
    if basis == "range_envelope":
        _require(_keys(expression, {"operand_ids", "mode"}) and expression["mode"] in MODES, "SHAPE")
        ids = expression["operand_ids"]
        _require(isinstance(ids, list) and ids and all(_text(i) for i in ids), "SHAPE")
        ordered = sorted(ids)
        _require(len(set(ordered)) == len(ordered), "SHAPE")
        return ordered
    raise _fail("SHAPE")


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
    """§3: presence, shape, standing, row binding and both notices; a blocked envelope carries neither."""
    evidence = source.get("contract_evidence") or {}
    member = evidence.get(MEMBER) if MEMBER in evidence else None
    rows = _items(source.get("results"))
    diagnostics = _items(source.get("diagnostics"))
    nonlinear = any(isinstance(r, Mapping) and r.get("kind") == STATE_CODE for r in rows)
    admitted = [g["combination_id"] for g in _items(evidence.get("combination_gates"))
                if isinstance(g, Mapping) and g.get("withheld") is False and isinstance(g.get("combination_id"), str)]
    _require((MEMBER in evidence) == (solved and nonlinear and bool(admitted)), "PRESENCE")
    # In a solved nonlinear document every mechanics combination is withheld.
    if solved and nonlinear:
        _require(not any(isinstance(r, Mapping) and _basis(r, "combination") is not None and isinstance(r.get("metadata"), Mapping)
                         and r["metadata"].get("basis") == LINEAR_BASIS for r in rows), "MECHANICS_ADMITTED")
    expected: list[dict[str, Any]] = []
    if MEMBER in evidence:
        _require(isinstance(member, list), "SHAPE")
        _require(len(member) == len(admitted), "PRESENCE")
        quality = _items((source.get("numerical_quality") or {}).get("cases"))
        by_id = {r["id"]: r for r in rows if isinstance(r, Mapping) and isinstance(r.get("id"), str)}
        for record, gate_id in zip(member, admitted):
            _require(_keys(record, RECORD_KEYS) and record["standing"] == STANDING, "SHAPE")
            _require(record["combination_id"] == gate_id, "PRESENCE")
            cid = gate_id
            order = _evaluation_order(record["basis"], record["expression"])
            operands = record["operands"]
            _require(isinstance(operands, list) and len(operands) == len(order), "SHAPE")
            for operand, case in zip(operands, order):
                _require(_keys(operand, OPERAND_KEYS) and operand["load_case_id"] == case, "SHAPE")
                _require(operand["modulus_basis_ref"] is None or _text(operand["modulus_basis_ref"]), "SHAPE")
                t = operand["modulus_basis_temperature"]
                _require(t is None or (_keys(t, {"value", "unit"}) and type(t["value"]) in (int, float) and _text(t["unit"])), "SHAPE")
                stated = next((q for q in quality if isinstance(q, Mapping) and isinstance(q.get("basis_ref"), Mapping)
                               and q["basis_ref"].get("ref_type") == "load_case" and q["basis_ref"].get("ref_id") == case), None)
                _require(stated is not None and bit_equal(stated, operand["numerical_quality"]), "STANDING")
            # Rows: references by row kind and basis.
            row_basis = "explicit_user_range_envelope" if record["basis"] == "range_envelope" else "explicit_user_result_state_subtraction"
            for row in rows:
                if not isinstance(row, Mapping) or _basis(row, "combination") != cid or row.get("kind") == MODULUS_RECORD:
                    continue
                metadata = row.get("metadata")
                _require(metadata is None or (isinstance(metadata, Mapping) and metadata.get("basis") == row_basis), "ROWS")
                refs = row.get("source_result_refs")
                refs = refs if isinstance(refs, list) else []
                kind = row.get("kind")
                components = None
                if record["basis"] == "result_state_subtraction" and kind == "displacement_magnitude":
                    components = [(k, None) for k in DISPLACEMENT_COMPONENTS]
                elif record["basis"] == "result_state_subtraction" and kind in ("support_reaction_force_magnitude_v2", "support_reaction_moment_magnitude_v2"):
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
                    # Direct rows name one matching case row per operand, in evaluation order.
                    _require(len(refs) == len(order), "ROWS")
                    for ref, case in zip(refs, order):
                        source = by_id.get(ref) if isinstance(ref, str) else None
                        _require(source is not None and _basis(source, "load_case") == case and source.get("kind") == kind
                                 and source.get("entity_ref") == row.get("entity_ref") and source.get("unit") == row.get("unit")
                                 and (source.get("metadata") or {}).get("component") == (metadata or {}).get("component")
                                 and (source.get("metadata") or {}).get("location") == (metadata or {}).get("location"), "ROWS")
            for notice in (_states_notice(cid, order, rows), _standing_notice(cid, operands)):
                if notice is not None:
                    expected.append(notice)
    stated = [d for d in diagnostics if isinstance(d, Mapping) and d.get("code") in (STATES_DIFFER, NOT_PASSED)]
    _require(len(stated) == len(expected), "NOTICE")
    for notice in expected:
        _require(sum(1 for d in stated if bit_equal(d, notice)) == 1, "NOTICE")


def model_checks(source: Mapping[str, Any], model: Mapping[str, Any]) -> None:
    """§3 with a supplied model: one record per subtraction or range envelope, in model order, when the model has a
    nonlinear support, and none otherwise; ids, bases, expressions and operand selections are the model's."""
    if (source.get("status") or {}).get("mechanics") != "MECHANICS_SOLVED":
        return
    nonlinear = any(isinstance(s, Mapping) and s.get("nonlinear") is not None for s in _items(model.get("supports")))
    combinations = model.get("combinations")
    combinations = [] if combinations is None else combinations
    _require(isinstance(combinations, list), "MODEL")
    combinations = [c for c in combinations if isinstance(c, Mapping) and c.get("basis") != "mechanics"]
    records = _items((source.get("contract_evidence") or {}).get(MEMBER))
    _require(len(records) == (len(combinations) if nonlinear else 0), "MODEL")
    cases = _items(model.get("load_cases"))
    for record, combination in zip(records, combinations):
        _require(record.get("combination_id") == combination.get("id") and record.get("basis") == combination.get("basis"), "MODEL")
        if combination.get("basis") == "result_state_subtraction":
            expression = {"minuend_id": combination.get("minuend_id"), "subtrahend_id": combination.get("subtrahend_id")}
        else:
            expression = {"operand_ids": combination.get("operand_ids"), "mode": combination.get("mode")}
        _require(bit_equal(record.get("expression"), expression), "MODEL")
        for operand in _items(record.get("operands")):
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
