"""RT-A5 ``minuend_final`` (DEL-05-02 Work operating-case.md section 7.6): the Python mirror of RE
``preview_held_status.rs``.

A preview-physics-2 declared operating comparison whose subtrahend is the subtrahend case's loads solved with each
nonlinear support held at the minuend's final status: its held gates, its held-operand rows (basis
``held_status_operand``, under the combination), its rows' sign convention and its evidence
(``contract_evidence.held_status_derivation``).

- Without the model: every record has a closed shape in gate order, is bound to a declared subtraction's
  ``difference_basis_derivation`` record (source = minuend, target = subtrahend), the document's solver mode, and a
  gate that is exactly its outcome; the source's qualification by the outcome, its standing by the minuend's
  confirmed evidence line, the target's modulus key by the subtrahend's modulus record and each derived normal's
  source by the minuend's derived-normal row. Admitted records only: ``held`` covers exactly the minuend's published state-code
  rows, each held state equal to its code and its DOF the row's location, and the held-operand rows exist. Each
  ordinary decision is re-derived from its published inputs in binary64, in the classifier's order; each strict-gap
  decision by the decision rule over its published exact signs and prior (never from binary64 bits; the signs are the
  proof's and are not verified here). The standing, the conditions, HELD_STATUS_RELEASED and the held solve's
  convergence and exit diagnostic follow from the decisions;
  the held sign is only on the difference rows and the typed basis only on held-operand rows.
- With the model: the model-free checks first; then each support's node, DOF, behaviour, sense or closing direction,
  gap, friction coefficient, explicit normal and derived-normal source (support, node and DOF), the target's modulus
  key against the subtrahend case's, and each trial's ground and the sliding-ground
  rule from both cases' authored imposed motion. A case with ``condition_use`` is refused as unverifiable.
- Claimed: both readings detect a forgery of what they check. Solved values are never recomputed, so a forgery that
  alters solved values coherently with their decisions passes both readings.
"""
from __future__ import annotations

import math
import struct
from collections.abc import Mapping
from typing import Any

from .units import to_canonical

MEMBER = "held_status_derivation"
OPERAND_BASIS = "held_status_operand"
HELD_SUPPORT_STATE = "held_at_minuend_final"
RELEASED = "HELD_STATUS_RELEASED"
GATES = ("HELD_STATUS_SOURCE_UNAVAILABLE", "HELD_STATUS_SOURCE_UNQUALIFIED", "HELD_STATUS_BINDING",
         "HELD_STATUS_SLIDING_GROUND_DIFFERS", "HELD_STATUS_SOLVE_FAILED")
SLIDING_GROUND_DIFFERS = "HELD_STATUS_SLIDING_GROUND_DIFFERS"
SIGN = ("positive value follows explicit user result-state subtraction (minuend minus subtrahend) of matching source result "
        "sign conventions; operating minus sustained difference at the operating support status: the minuend case's own "
        "solved state minus the subtrahend case's loads solved with each nonlinear support held at the minuend's final "
        "status, a held-constraint comparison that can need a reaction a unilateral support cannot supply and is not an "
        "admissible sustained state, under the static, history-free friction model; derived from case results; not an "
        "equilibrium state")
ESTABLISHED = ("Established within its stated numerical bound for the explicitly held system at this case's loads. The "
               "verdict does not establish this case's own support state, an admissible sustained state, a load history "
               "or uniqueness.")
RELEASED_STANDING = "Not established: the held status is not consistent with this case's loads; rows are inspection evidence."
GAP_STANDING = "Gap supports keep their recovery-basis branch (R1); not established."
NOT_ESTABLISHED_PREFIX = "Not established: "
NOT_ESTABLISHED_SUFFIX = ". The held status is kept by this case's classifier; rows are inspection evidence."
SUBTRACTION_BASIS = "explicit_user_result_state_subtraction"
STATE_CODE = "nonlinear_support_active_set_state_code"
ITERATION_COUNT = "nonlinear_support_active_set_iteration_count"
MODULUS_RECORD = "combination_modulus_basis_record"
EVIDENCE = "SOURCE_PREVIEW_PHYSICS_HELD_STATUS_EVIDENCE"
BINDING = "SOURCE_PREVIEW_PHYSICS_HELD_STATUS_BINDING"
DERIVATION = "SOURCE_PREVIEW_PHYSICS_HELD_STATUS_DERIVATION"
ROW = "SOURCE_PREVIEW_PHYSICS_HELD_STATUS_ROW"
MODEL = "SOURCE_PREVIEW_PHYSICS_HELD_STATUS_MODEL"
UNVERIFIED = "SOURCE_PREVIEW_PHYSICS_COMBINATION_GATE_UNVERIFIED"
RECORD_KEYS = {"combination_id", "subtrahend_support_state", "source", "target", "outcome", "held", "held_solve", "standing"}
STATES = ("inactive", "active", "sticking", "sliding")
DOFS = ("UX", "UY", "UZ", "RX", "RY", "RZ")
NORMAL_DERIVED = "nonlinear_support_friction_normal_reaction_derived"
NONCONVERGENCE = "NONLINEAR_SUPPORT_NONCONVERGENCE"
UNAVAILABLE = "HELD_STATUS_SOURCE_UNAVAILABLE"
UNQUALIFIED = "HELD_STATUS_SOURCE_UNQUALIFIED"
BASE_MATERIAL = "material_base_values"


def _require(condition: Any, code: str) -> None:
    if not condition:
        raise ValueError(code)


def _items(value: Any) -> list:
    return value if isinstance(value, list) else []


def _keys(value: Any, keys: set) -> bool:
    return isinstance(value, Mapping) and set(value) == set(keys)


def _text(value: Any, code: str) -> str:
    _require(isinstance(value, str) and value != "", code)
    return value


def _ref(row: Any, kind: str) -> str | None:
    basis = row.get("basis_ref") if isinstance(row, Mapping) else None
    return basis.get("ref_id") if isinstance(basis, Mapping) and basis.get("ref_type") == kind else None


def is_held_row(row: Any) -> bool:
    """A held-operand row: its id is the combination's ``:held:`` id."""
    return _ref(row, "combination") is not None and isinstance(row.get("id"), str) and ":held:" in row["id"]


def _bits(value: Any) -> float:
    _require(isinstance(value, str) and len(value) == 16 and all(c in "0123456789abcdef" for c in value), EVIDENCE)
    x = struct.unpack(">d", struct.pack(">Q", int(value, 16)))[0]
    _require(math.isfinite(x), EVIDENCE)
    return x


def _hex(x: float) -> str:
    return f"{struct.unpack('>Q', struct.pack('>d', x))[0]:016x}"


def _metadata(row: Mapping) -> Mapping:
    return row.get("metadata") if isinstance(row.get("metadata"), Mapping) else {}


def _classify_ordinary(entry: Mapping, behavior: str, held: str) -> tuple[str, bool, bool, float]:
    """Section 7.2's classifier over one ordinary entry's binary64 inputs, in the classifier's order."""
    inputs = entry["inputs"]
    delta = _bits(inputs.get("displacement_bits"))
    r = _bits(inputs.get("reaction_bits"))
    _bits(inputs.get("ground_bits"))
    engaged = held == "active"
    def active(yes: bool) -> str:
        return "active" if yes else "inactive"
    if behavior in ("one_way", "lift_off"):
        _require(entry.get("sense") in ("positive_reaction", "negative_reaction"), EVIDENCE)
        positive = entry["sense"] == "positive_reaction"
        if engaged:
            state = r > 0.0 if positive else r < 0.0
        else:
            state = delta < 0.0 if positive else delta > 0.0
        return active(state), True, False, 0.0
    if behavior == "gap":
        _require(entry.get("closes_when") in ("positive_displacement", "negative_displacement"), EVIDENCE)
        positive = entry["closes_when"] == "positive_displacement"
        gap = _bits(entry.get("gap_bits"))
        if engaged:
            state = r <= 0.0 if positive else r >= 0.0
        else:
            state = delta >= gap if positive else delta <= -gap
        return active(state), True, False, 0.0
    _require(behavior == "friction", EVIDENCE)
    mu = _bits(inputs.get("coefficient_bits"))
    normal = _bits(inputs.get("normal_trial_bits"))
    signed = _bits(inputs.get("normal_signed_bits"))
    applied = _bits(inputs.get("applied_force_bits"))
    _require(inputs.get("normal_source") in ("explicit", "derived"), EVIDENCE)
    derived = inputs["normal_source"] == "derived"
    _require(normal == (abs(signed) if derived else signed), DERIVATION)
    zero_limit = mu * abs(normal) == 0.0
    if normal <= 0.0:
        return "inactive", True, zero_limit, normal
    if held != "sliding":
        return ("sticking" if abs(r) <= mu * abs(normal) else "sliding"), True, zero_limit, normal
    sense = inputs.get("slip_sense")
    _require(type(sense) is int and sense in (1, -1), EVIDENCE)
    branch = True
    if derived:
        beta = inputs.get("beta")
        _require(type(beta) is int and beta in (-1, 0, 1), EVIDENCE)
        if mu == 0.0 or sense == 0:
            branch = True
        elif beta == 0:
            branch = signed == 0.0
        else:
            branch = beta * signed >= 0.0
        _require(isinstance(inputs.get("branch_admissible"), bool) and inputs["branch_admissible"] is branch, DERIVATION)
        if not branch:
            return "sliding", False, zero_limit, normal
    limit = mu * normal
    if limit == 0.0:
        sliding = applied == 0.0 and delta != 0.0
    else:
        sliding = applied != 0.0 and delta != 0.0 and applied * delta < 0.0 and r * delta < 0.0
    return ("sliding" if sliding else "sticking"), branch, zero_limit, normal


def _classify_strict(entry: Mapping, held: str) -> str:
    """The strict-gap decision rule over the published exact signs and prior."""
    s = entry.get("strict_gap")
    _require(_keys(s, {"prior", "gap_sign", "reaction_sign", "state"}) and s["prior"] == held, EVIDENCE)
    _require(s["gap_sign"] in ("negative", "zero", "positive") and s["reaction_sign"] in ("negative", "zero", "positive"), EVIDENCE)
    active = s["reaction_sign"] != "positive" if held == "active" else s["gap_sign"] != "negative"
    decided = "active" if active else "inactive"
    _require(s["state"] == decided, DERIVATION)
    return decided


def _condition(behavior: str, held: str, derived: bool, zero_limit: bool) -> tuple[str, str]:
    if behavior in ("one_way", "lift_off"):
        return ("unilateral_contact_sign" if held == "active" else "unilateral_separation"), "strict_ball"
    if behavior == "friction":
        if held == "inactive":
            return "friction_normal_nonpositive", "input_decision"
        if held == "sticking":
            return ("stick_capacity_derived" if derived else "stick_capacity_explicit"), "strict_ball"
        if zero_limit:
            return "slip_zero_limit", "strict_ball"
        return ("slip_derived" if derived else "slip_explicit"), "strict_ball"
    return "gap_r1_not_run", "not_run"


def _read_entry(entry: Any) -> dict:
    _require(isinstance(entry, Mapping), EVIDENCE)
    support_id = _text(entry.get("support_id"), EVIDENCE)
    _text(entry.get("node"), EVIDENCE)
    _text(entry.get("dof"), EVIDENCE)
    behavior = _text(entry.get("behavior"), EVIDENCE)
    held, classified = entry.get("held_state"), entry.get("classified_state")
    _require(held in STATES and classified in STATES, EVIDENCE)
    released = entry.get("released")
    _require(released in ("none", "changed_state", "branch"), EVIDENCE)
    keys = {"support_id", "node", "dof", "behavior", "held_state", "classified_state", "decision", "released"}
    if behavior in ("one_way", "lift_off"):
        keys.add("sense")
    elif behavior == "gap":
        keys |= {"closes_when", "gap_bits"}
    else:
        _require(behavior == "friction", EVIDENCE)
    decision = entry.get("decision")
    inputs = entry.get("inputs") if isinstance(entry.get("inputs"), Mapping) else {}
    if decision == "ordinary":
        keys.add("inputs")
        input_keys = {"ground_bits", "displacement_bits", "reaction_bits"}
        if behavior == "friction":
            input_keys |= {"coefficient_bits", "normal_source", "normal_trial_bits", "normal_signed_bits", "applied_force_bits"}
            derived = inputs.get("normal_source") == "derived"
            if derived:
                input_keys.add("normal_source_ref")
            if held == "sliding":
                input_keys.add("slip_sense")
                if derived:
                    input_keys |= {"beta", "branch_admissible"}
        _require(_keys(entry.get("inputs"), input_keys), EVIDENCE)
        if "normal_source_ref" in inputs:
            ref = inputs["normal_source_ref"]
            _require(_keys(ref, {"support", "node", "dof"}), EVIDENCE)
            _text(ref["support"], EVIDENCE)
            _text(ref["node"], EVIDENCE)
            _require(ref["dof"] in DOFS, EVIDENCE)
        decided, branch, zero_limit, normal_trial = _classify_ordinary(entry, behavior, held)
    elif decision == "strict_gap" and behavior == "gap":
        keys.add("strict_gap")
        decided, branch, zero_limit, normal_trial = _classify_strict(entry, held), True, False, 0.0
    else:
        raise ValueError(EVIDENCE)
    _require(_keys(entry, keys), EVIDENCE)
    _require(classified == decided, DERIVATION)
    expected = "branch" if not branch else ("changed_state" if classified != held else "none")
    _require(released == expected, DERIVATION)
    return {"support_id": support_id, "behavior": behavior, "held": held, "released": released, "gap": behavior == "gap",
            "derived": inputs.get("normal_source") == "derived", "zero_limit": zero_limit, "normal_trial": normal_trial,
            "raw": entry}


def _loop_mode(rows: list, case: str) -> str | None:
    for r in rows:
        if isinstance(r, Mapping) and r.get("kind") == ITERATION_COUNT and _ref(r, "load_case") == case:
            basis = _metadata(r).get("basis")
            if isinstance(basis, str) and "_active_set_loop" in basis:
                return basis.split("_active_set_loop", 1)[0]
            return None
    return None


def _read_record(record: Any) -> dict:
    _require(isinstance(record, Mapping), EVIDENCE)
    admitted = record.get("outcome") == "admitted"
    keys = set(RECORD_KEYS) | ({"conditions"} if admitted and "conditions" in record else set())
    _require(_keys(record, keys) and record["subtrahend_support_state"] == "minuend_final", EVIDENCE)
    cid = _text(record["combination_id"], EVIDENCE)
    src = record["source"]
    _require(_keys(src, {"load_case_id", "solver_mode", "qualification", "standing"}), EVIDENCE)
    minuend = _text(src["load_case_id"], EVIDENCE)
    _text(src["solver_mode"], EVIDENCE)
    _require(src["qualification"] in (None, "qualified", "inspectable_unqualified_gap"), EVIDENCE)
    _require(src["standing"] in (None, "confirmed", "not_established"), EVIDENCE)
    target = record["target"]
    _require(_keys(target, {"load_case_id", "modulus_basis_key"}) and (target["modulus_basis_key"] is None or isinstance(target["modulus_basis_key"], str)), EVIDENCE)
    subtrahend = _text(target["load_case_id"], EVIDENCE)
    outcome = _text(record["outcome"], EVIDENCE)
    _require(admitted or outcome in GATES, EVIDENCE)
    standing = record["standing"]
    _require(_keys(standing, {"kind", "reason"}), EVIDENCE)
    kind, reason = _text(standing["kind"], EVIDENCE), _text(standing["reason"], EVIDENCE)
    entries = record["held"]
    _require(isinstance(entries, list), EVIDENCE)
    out = {"id": cid, "minuend": minuend, "subtrahend": subtrahend, "outcome": outcome, "held": [], "raw": record}
    if not admitted:
        _require(not entries and record["held_solve"] is None and kind == "withheld", EVIDENCE)
        return out
    solve = record["held_solve"]
    _require(_keys(solve, {"converged", "exit_diagnostic"}) and isinstance(solve["converged"], bool), EVIDENCE)
    exit_ = solve["exit_diagnostic"]
    _require(exit_ is None or (_keys(exit_, {"code", "message"}) and isinstance(exit_["code"], str) and isinstance(exit_["message"], str)), EVIDENCE)
    last = None
    for entry in entries:
        read = _read_entry(entry)
        _require(last is None or last < read["support_id"], EVIDENCE)
        last = read["support_id"]
        out["held"].append(read)
    held = out["held"]
    _require(held, EVIDENCE)
    any_released = any(e["released"] != "none" for e in held)
    converged = solve["converged"] is True
    _require(converged == (not any_released), DERIVATION)
    if converged:
        _require(exit_ is None, DERIVATION)
    else:
        _require(isinstance(exit_, Mapping) and exit_["code"] == NONCONVERGENCE and exit_["message"].strip() != "", DERIVATION)
    gap = any(e["gap"] for e in held)
    expected_kind = "gap_inspection" if gap else ("not_established" if any_released else kind)
    _require(kind == expected_kind and kind in ("established", "not_established", "gap_inspection"), DERIVATION)
    if kind == "established":
        reason_ok = reason == ESTABLISHED
    elif kind == "gap_inspection":
        reason_ok = reason == GAP_STANDING
    elif any_released:
        reason_ok = reason == RELEASED_STANDING
    else:
        reason_ok = reason.startswith(NOT_ESTABLISHED_PREFIX) and reason.endswith(NOT_ESTABLISHED_SUFFIX)
    _require(reason_ok, DERIVATION)
    _require(("conditions" in record) == (kind == "established"), DERIVATION)
    if kind == "established":
        conditions = _items(record["conditions"])
        _require(len(conditions) == len(held), EVIDENCE)
        for c, e in zip(conditions, held):
            name, klass = _condition(e["behavior"], e["held"], e["derived"], e["zero_limit"])
            full = isinstance(c, Mapping) and klass == "strict_ball" and "published_bits" in c
            ckeys = {"support_id", "condition", "class"} | ({"published_bits", "radius_bits"} if full else set())
            _require(_keys(c, ckeys) and c["support_id"] == e["support_id"], EVIDENCE)
            _require(c["condition"] == name and c["class"] == klass, DERIVATION)
            if full:
                _bits(c["published_bits"])
                _require(_bits(c["radius_bits"]) >= 0.0, EVIDENCE)
            if klass == "input_decision":
                _require(e["normal_trial"] <= 0.0, DERIVATION)
    return out


def _provenance_check(raw: Mapping, record: dict, rows: list, diagnostics: list) -> None:
    """The record's provenance against the document (RE ``provenance_check``)."""
    from .nonlinear_support_evidence import CONFIRMED
    src = raw["source"]
    if record["outcome"] == UNAVAILABLE:
        _require(src["qualification"] is None and src["standing"] is None, BINDING)
    else:
        wanted = "inspectable_unqualified_gap" if record["outcome"] == UNQUALIFIED else "qualified"
        _require(src["qualification"] == wanted, BINDING)
        confirmed = any(isinstance(d.get("code"), str) and d["code"].startswith("NUMERICAL_INTEGRITY_")
                        and _items(d.get("affected_refs")) == [record["minuend"]]
                        and isinstance(d.get("message"), str) and CONFIRMED in d["message"] for d in diagnostics)
        _require(src["standing"] == ("confirmed" if confirmed else "not_established"), BINDING)
    label = next((_metadata(r).get("basis") for r in rows if r.get("kind") == MODULUS_RECORD and _ref(r, "combination") == record["id"]
                  and r.get("entity_ref") == record["subtrahend"]), None)
    _require(isinstance(label, str) and _modulus_label_states(raw["target"]["modulus_basis_key"], label), BINDING)
    for entry in _items(raw["held"]):
        derived = next((r for r in rows if r.get("kind") == NORMAL_DERIVED and _ref(r, "load_case") == record["minuend"]
                        and r.get("entity_ref") == entry.get("support_id")), None)
        inputs = entry.get("inputs") if isinstance(entry.get("inputs"), Mapping) else {}
        if "normal_source_ref" not in inputs:
            _require(derived is None, BINDING)
            continue
        basis = _metadata(derived).get("basis") if derived is not None else None
        _require(isinstance(basis, str), BINDING)
        segments = dict(s.split("=", 1) for s in basis.split("; ") if "=" in s)
        ref = inputs["normal_source_ref"]
        _require(segments.get("source_ref") == ref["support"] and str(segments.get("source_dof", "")).upper() == ref["dof"], BINDING)


def _modulus_label_states(key: Any, label: str) -> bool:
    """Whether an operand's modulus record label states this modulus key (RE ``modulus_label_states``)."""
    if key is None:
        return label == BASE_MATERIAL
    if key.startswith("exact:"):
        reference = key[len("exact:"):]
        return reference != "" and label.startswith(f"temperature_point:{reference}; selection=exact_user_entered_point;")
    prefix = "selection=declared_solve_temperature; solve_temperature_kelvin="
    if not key.startswith("temperature_kelvin_bits:") or not label.startswith(prefix):
        return False
    bits = key[len("temperature_kelvin_bits:"):]
    try:
        value = float(label[len(prefix):].split(";", 1)[0].strip())
    except ValueError:
        return False
    return len(bits) == 16 and math.isfinite(value) and _hex(value) == bits


def read(source: Mapping[str, Any]) -> list:
    """The member, read and bound to the document (both readings)."""
    evidence = source.get("contract_evidence") or {}
    if MEMBER not in evidence:
        return []
    member = evidence[MEMBER]
    _require(isinstance(member, list) and member, EVIDENCE)
    gates = [g for g in _items(evidence.get("combination_gates")) if isinstance(g, Mapping)]
    position = {g.get("combination_id"): i for i, g in enumerate(gates) if isinstance(g.get("combination_id"), str)}
    declared = {}
    for r in _items(evidence.get("difference_basis_derivation")):
        if isinstance(r, Mapping):
            declared[r.get("combination_id")] = ((r.get("minuend") or {}).get("load_case_id"), (r.get("subtrahend") or {}).get("load_case_id"))
    rows = [r for r in _items(source.get("results")) if isinstance(r, Mapping)]
    diagnostics = [d for d in _items(source.get("diagnostics")) if isinstance(d, Mapping)]
    records, last = [], None
    for raw in member:
        record = _read_record(raw)
        cid = record["id"]
        _require(cid in position, BINDING)
        at = position[cid]
        _require(last is None or at > last, EVIDENCE)
        last = at
        _require(declared.get(cid) == (record["minuend"], record["subtrahend"]), BINDING)
        gate = gates[at]
        admitted = record["outcome"] == "admitted"
        _require(gate.get("withheld") is (not admitted) and (gate.get("reason") is None) == admitted
                 and (admitted or gate.get("reason") == record["outcome"]), BINDING)
        _provenance_check(raw, record, rows, diagnostics)
        notices = [d for d in diagnostics if d.get("code") == RELEASED and _items(d.get("affected_refs"))[:1] == [cid]]
        if admitted:
            _require(_loop_mode(rows, record["minuend"]) == raw["source"]["solver_mode"], BINDING)
            codes = [r for r in rows if r.get("kind") == STATE_CODE and _ref(r, "load_case") == record["minuend"]]
            _require(len(codes) == len(record["held"]), BINDING)
            for entry in record["held"]:
                code = next((r for r in codes if r.get("entity_ref") == entry["support_id"]), None)
                _require(code is not None, BINDING)
                value = code.get("value")
                state = STATES[int(value)] if isinstance(value, (int, float)) and value in (0, 1, 2, 3) else None
                _require(state == entry["held"], BINDING)
                location = _metadata(code).get("location")
                _require(isinstance(location, str) and location.upper() == entry["raw"]["dof"], BINDING)
            _require(any(is_held_row(r) and _ref(r, "combination") == cid for r in rows), BINDING)
            released = [e["support_id"] for e in record["held"] if e["released"] != "none"]
            if not released:
                _require(not notices, DERIVATION)
            else:
                _require(len(notices) == 1 and notices[0].get("severity") == "warning", DERIVATION)
                _require(_items(notices[0].get("affected_refs"))[1:] == released, DERIVATION)
        else:
            _require(not notices, DERIVATION)
        records.append(record)
    return records


def _claimed(source: Mapping[str, Any]) -> bool:
    evidence = source.get("contract_evidence") or {}
    rows = [r for r in _items(source.get("results")) if isinstance(r, Mapping)]
    return (MEMBER in evidence
            or any(isinstance(g, Mapping) and g.get("reason") in GATES for g in _items(evidence.get("combination_gates")))
            or any(_metadata(r).get("basis") == OPERAND_BASIS or _metadata(r).get("sign_convention") == SIGN or is_held_row(r) for r in rows)
            or any(isinstance(d, Mapping) and d.get("code") == RELEASED for d in _items(source.get("diagnostics"))))


def _rows_check(source: Mapping[str, Any], records: list) -> None:
    admitted = {r["id"] for r in records if r["outcome"] == "admitted"}
    for row in _items(source.get("results")):
        if not isinstance(row, Mapping):
            continue
        held_row = is_held_row(row)
        metadata = _metadata(row)
        basis, sign = metadata.get("basis"), metadata.get("sign_convention")
        cid = _ref(row, "combination")
        if cid in admitted and row.get("kind") != MODULUS_RECORD:
            if held_row:
                clause = (f"held-status operand of {cid}: the subtrahend case's loads solved with the minuend's final support "
                          "status held; not the subtrahend case's result")
                _require(basis == OPERAND_BASIS and isinstance(sign, str) and (sign == clause or sign.endswith("; " + clause)), ROW)
            else:
                _require(basis != OPERAND_BASIS and (row.get("metadata") is None or (sign == SIGN and basis == SUBTRACTION_BASIS)), ROW)
        else:
            _require(not held_row and basis != OPERAND_BASIS and sign != SIGN, ROW)
        if held_row:
            _require(_ref(row, "load_case") is None, ROW)


def check(source: Mapping[str, Any], model: Mapping[str, Any] | None = None) -> None:
    """A solved preview-physics-2 document's minuend_final subtractions, read with the model when it is supplied."""
    if not _claimed(source):
        return
    records = read(source)
    recorded = {r["id"] for r in records}
    for gate in _items((source.get("contract_evidence") or {}).get("combination_gates")):
        if isinstance(gate, Mapping) and gate.get("reason") in GATES:
            _require(gate.get("combination_id") in recorded, EVIDENCE)
    _rows_check(source, records)
    if model is not None:
        _model_check(source, model, records)


def stated_gate(source: Mapping[str, Any], cid: Any) -> str | None:
    """The held gate a record states for ``cid`` (None when admitted or unrecorded)."""
    for r in _items((source.get("contract_evidence") or {}).get(MEMBER)):
        if isinstance(r, Mapping) and r.get("combination_id") == cid:
            return r.get("outcome") if r.get("outcome") in GATES else None
    return None


def held_standing_of(source: Mapping[str, Any], cid: Any) -> Any:
    """The held standing the admitted record of ``cid`` states."""
    for r in _items((source.get("contract_evidence") or {}).get(MEMBER)):
        if isinstance(r, Mapping) and r.get("combination_id") == cid and r.get("outcome") == "admitted":
            return r.get("standing")
    return None


def minuend_final(combination: Mapping[str, Any]) -> bool:
    from .preview_difference_basis import declared
    return declared(combination) and combination.get("subtrahend_support_state") == "minuend_final"


def _quantity(q: Any, dimension: str) -> float:
    _require(isinstance(q, Mapping), MODEL)
    try:
        return float(to_canonical(q.get("value"), q.get("unit"), dimension))
    except (ValueError, TypeError) as exc:
        raise ValueError(MODEL) from exc


def _ground(case: Mapping, support: str, dof: str) -> float | None:
    out = None
    for load in _items(case.get("primitive_loads")):
        target = load.get("target") if isinstance(load, Mapping) else None
        if (isinstance(target, Mapping) and load.get("category") == "imposed_displacement" and target.get("type") == "support"
                and target.get("support") == support and isinstance(target.get("dof"), str) and target["dof"].upper() == dof):
            value = _quantity(load.get("magnitude"), "rotation" if dof.startswith("R") else "displacement")
            out = (out or 0.0) + value
    return out


def _model_check(source: Mapping[str, Any], model: Mapping[str, Any], records: list) -> None:
    from .preview_combination_gates import _modulus_key
    from .preview_difference_basis import kept_combinations, model_gate
    cases = [c for c in _items(model.get("load_cases")) if isinstance(c, Mapping)]
    supports = [s for s in _items(model.get("supports")) if isinstance(s, Mapping)]
    combinations = [c for c in _items(model.get("combinations")) if isinstance(c, Mapping)]
    rows = [r for r in _items(source.get("results")) if isinstance(r, Mapping)]
    def case(cid):
        found = next((c for c in cases if c.get("id") == cid), None)
        _require(found is not None, MODEL)
        return found
    def bits_of(x):
        return None if x is None else _hex(x)
    for record in records:
        combination = next((c for c in combinations if c.get("id") == record["id"]), None)
        _require(combination is not None and minuend_final(combination) and combination.get("minuend_id") == record["minuend"]
                 and combination.get("subtrahend_id") == record["subtrahend"], MODEL)
        minuend, subtrahend = case(record["minuend"]), case(record["subtrahend"])
        _require(all(c.get("condition_use") is None for c in (minuend, subtrahend)), UNVERIFIED)
        try:
            key = _modulus_key(subtrahend)
        except Exception as exc:  # the gates reader's own refusal: the model contradicts the key
            raise ValueError(MODEL) from exc
        _require(record["raw"]["target"]["modulus_basis_key"] == key, MODEL)
        if record["outcome"] == SLIDING_GROUND_DIFFERS:
            differs = False
            for r in rows:
                if r.get("kind") == STATE_CODE and _ref(r, "load_case") == record["minuend"] and r.get("value") == 3:
                    dof = str(_metadata(r).get("location") or "").upper()
                    try:
                        a, b = _ground(minuend, r.get("entity_ref"), dof), _ground(subtrahend, r.get("entity_ref"), dof)
                    except ValueError:
                        continue
                    differs = differs or bits_of(a) != bits_of(b)
            _require(differs, MODEL)
        for entry in record["held"]:
            raw = entry["raw"]
            support = next((s for s in supports if s.get("id") == entry["support_id"]), None)
            _require(support is not None and isinstance(support.get("nonlinear"), Mapping), MODEL)
            nonlinear = support["nonlinear"]
            _require(raw["node"] == support.get("node") and raw["behavior"] == nonlinear.get("behavior"), MODEL)
            _require(isinstance(nonlinear.get("dof"), str), MODEL)
            dof = nonlinear["dof"].upper()
            _require(raw["dof"] == dof, MODEL)
            behavior = entry["behavior"]
            if behavior == "one_way":
                _require(raw["sense"] == nonlinear.get("active_when"), MODEL)
            elif behavior == "lift_off":
                _require(raw["sense"] == nonlinear.get("contact_when"), MODEL)
            elif behavior == "gap":
                _require(raw["closes_when"] == nonlinear.get("closes_when"), MODEL)
                _require(raw["gap_bits"] == _hex(_quantity(nonlinear.get("gap"), "displacement")), MODEL)
            else:
                inputs = raw["inputs"]
                mu = (nonlinear.get("friction_coefficient") or {}).get("value")
                _require(type(mu) in (int, float) and inputs["coefficient_bits"] == _hex(float(mu)), MODEL)
                if nonlinear.get("normal_reaction") is not None:
                    n = _quantity(nonlinear["normal_reaction"], "force")
                    _require(inputs["normal_source"] == "explicit" and inputs["normal_signed_bits"] == _hex(n), MODEL)
                else:
                    # The derived normal's source is the model's own assignment.
                    assigned = nonlinear.get("normal_reaction_source")
                    _require(inputs["normal_source"] == "derived" and isinstance(assigned, Mapping), MODEL)
                    stated = inputs["normal_source_ref"]
                    source_support = next((s for s in supports if s.get("id") == assigned.get("support_ref")), None)
                    _require(source_support is not None and stated["support"] == assigned.get("support_ref")
                             and isinstance(assigned.get("dof"), str) and assigned["dof"].upper() == stated["dof"]
                             and stated["node"] == source_support.get("node"), MODEL)
            if raw["decision"] == "ordinary":
                g = _ground(subtrahend, entry["support_id"], dof)
                _require(raw["inputs"]["ground_bits"] == _hex(g if g is not None else 0.0), MODEL)
            if entry["held"] == "sliding":
                _require(bits_of(_ground(minuend, entry["support_id"], dof)) == bits_of(_ground(subtrahend, entry["support_id"], dof)), MODEL)
    kept, order = kept_combinations(model)
    recorded = {r["id"] for r in records}
    for index in order:
        combination = kept[index]
        if minuend_final(combination) and model_gate(model, combination)[0] is None:
            _require(combination.get("id") in recorded, MODEL)
