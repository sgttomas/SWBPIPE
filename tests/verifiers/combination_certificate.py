"""RT-A3 §4 (DEL-05-02 Work ``combination-certificate.md``; ``operating-case.md`` §4.3): test-only mirror of RE
``combination_certificate.rs``. Re-derives ``contract_evidence.combination_certificate`` on preview-physics-2 and
pressure-2 bit for bit from the published rows and the operand cases' published group statements, and refuses a
forged bound, verdict, row binding, operand-bound binding, eligibility or diagnostic with the Rust reader's codes.

Exact arithmetic is ``fractions.Fraction``. The Rust reader's exact rationals (``load_case_algebra::Dyadic``) refuse a
significand above 4,096 bits; this mirror applies the same limit after every exact operation, so both fail closed on the
same inputs. Independence: standard-library imports only, and its siblings ``exact_combinations`` (the correctly rounded
``norm3``) and ``section_basis`` (bit equality).
"""
from __future__ import annotations

from collections.abc import Mapping
from fractions import Fraction
import math
import struct
from typing import Any

from .exact_combinations import norm3
from .section_basis import bit_equal

MEMBER = "combination_certificate"
WITHIN = "COMBINATION_ROWS_WITHIN_STATED_BOUND"
NOT_ASSESSED = "COMBINATION_ROWS_NOT_ASSESSED"
DIAGNOSTIC_PREFIX = "diagnostic:combination-certificate:"
CODE_PREFIX = "COMBINATION_ROWS_"
SOURCE = "core/product_physics"
PASSED = "NUMERICAL_INTEGRITY_CHECKS_PASSED"
SENSITIVE = "NUMERICAL_INTEGRITY_SENSITIVE"


def _from_bits(value: int) -> float:
    return struct.unpack(">d", struct.pack(">Q", value))[0]


def _bits(x: float) -> int:
    return struct.unpack(">Q", struct.pack(">d", x))[0]


CRITERION = _from_bits(_bits(1e-9) - 1)
DESIGN_FLOOR = _from_bits(0x3DD0000000000000)
DESIGN_FLOOR_MIN_SCALE = _from_bits((1023 - 988) << 52)
MAX_BITS = 4096

AXIAL = "element_local_axial_force"
SHEAR_Y, SHEAR_Z = "element_local_shear_force_y", "element_local_shear_force_z"
TORSION = "element_local_torsional_moment"
BENDING_Y, BENDING_Z = "element_local_bending_moment_y", "element_local_bending_moment_z"
EFFECTIVE = "pipe_effective_axial_force_v2"
WALL = "pipe_wall_axial_force_v2"
END_ACTION = "pipe_wall_endpoint_action_v2"
SUPPORT_COMPONENT = "support_reaction_component_v2"
SUPPORT_FORCE = "support_reaction_force_magnitude_v2"
SUPPORT_MOMENT = "support_reaction_moment_magnitude_v2"
MODULUS_RECORD = "combination_modulus_basis_record"
MEMBER_KEYS = {"combinations", "operand_cases"}
CASE_KEYS = {"groups", "load_case_id"}
OPERAND_GROUP_KEYS = {"bound", "entity", "floor_scale", "group", "location", "moment", "q", "rows"}
RECORD_KEYS = {"assessment", "basis", "combination_id", "expression", "groups", "reason", "subtrahend_support_state", "uncovered_row_count"}


class _Unavailable(Exception):
    """A Dyadic refusal (beyond 4,096 significand bits)."""


def _fail(code: str) -> ValueError:
    return ValueError(f"SOURCE_COMBINATION_CERTIFICATE_{code}")


def _require(ok: Any, code: str) -> None:
    if not ok:
        raise _fail(code)


def _items(value: Any) -> list[Any]:
    return value if isinstance(value, list) else []


def _array(value: Any) -> list[Any]:
    """A member array, refused by its shape when it is anything but an array (Rust `array`)."""
    _require(isinstance(value, list), "SHAPE")
    return value


def reserved_notice(d: Any) -> bool:
    """Whether a diagnostic is in the certificate's closed reserved vocabulary, whatever its id spelling (Rust
    `reserved_notice`): its id in the certificate's namespace, its code in the `COMBINATION_ROWS_` family, or its
    message stating rows within, or not assessed against, a stated bound."""
    if not isinstance(d, Mapping):
        return False

    def text(key: str) -> str:
        value = d.get(key)
        return value if isinstance(value, str) else ""

    message = text("message")
    return (text("id").startswith(DIAGNOSTIC_PREFIX) or text("code").startswith(CODE_PREFIX)
            or ("stated bound" in message and ("within" in message or "against" in message)))


def _keys(value: Any, keys: set[str]) -> bool:
    return isinstance(value, Mapping) and set(value) == keys


def _number(x: float) -> Any:
    return x if math.isfinite(x) else None


def _identity(*parts: str) -> str:
    """The producer's exact_source_identity: ``{UTF-8 byte length}:{part}`` per part."""
    return "".join(f"{len(p.encode('utf-8'))}:{p}" for p in parts)


# ----------------------------------------------------------------------------- exact and upward arithmetic

def _checked(value: Fraction) -> Fraction:
    """Dyadic's normalization limit: the odd significand has at most 4,096 bits."""
    if value != 0:
        n = abs(value.numerator)
        n >>= (n & -n).bit_length() - 1
        if n.bit_length() > MAX_BITS:
            raise _Unavailable()
    return value


def _exact_sum(a: float, b: float, s: float) -> bool:
    return Fraction(a) + Fraction(b) == Fraction(s)


def up_add(a: float, b: float) -> float:
    s = a + b
    if not math.isfinite(s):
        return math.inf
    return s if _exact_sum(a, b, s) else math.nextafter(s, math.inf)


def up_mul(a: float, b: float) -> float:
    try:
        p = a * b
    except OverflowError:
        return math.inf
    if not math.isfinite(p):
        return math.inf
    return p if Fraction(a) * Fraction(b) == Fraction(p) else math.nextafter(p, math.inf)


def up_sqrt(x: float) -> float:
    if not math.isfinite(x):
        return math.inf
    r = math.sqrt(x)
    return r if Fraction(r) * Fraction(r) == Fraction(x) else math.nextafter(r, math.inf)


def up_norm(values: list[float]) -> float:
    total = 0.0
    for v in values:
        total = up_add(total, up_mul(abs(v), abs(v)))
    return up_sqrt(total)


def round_up_magnitude(value: Fraction) -> float:
    """The smallest binary64 not below |value| (+inf beyond the finite range)."""
    magnitude = abs(value)
    if magnitude == 0:
        return 0.0
    if magnitude > Fraction(1.7976931348623157e308):
        return math.inf
    c = magnitude.numerator / magnitude.denominator  # correctly rounded (CPython)
    return c if Fraction(c) >= magnitude else math.nextafter(c, math.inf)


def rounding_error(derived: float, terms: list[tuple[float, float]]) -> float:
    try:
        total = _checked(Fraction(derived))
        for factor, value in terms:
            product = _checked(Fraction(value) * Fraction(factor))
            total = _checked(total - product)
    except _Unavailable:
        return math.inf
    return round_up_magnitude(total)


def verdict(q: float, q_hat: float, bound: float, s: float) -> tuple[str, str | None]:
    unavailable = ("not_established", "comparison_unavailable")
    values = (q, q_hat, bound, s)
    if not all(isinstance(x, float) and math.isfinite(x) for x in values) or q < 0 or q_hat < q or bound < 0 or s < 0:
        return unavailable
    try:
        fires = _checked(Fraction(bound) - _checked(Fraction(q) * Fraction(CRITERION))) > 0
    except _Unavailable:
        return unavailable
    if not (s >= DESIGN_FLOOR_MIN_SCALE and q_hat >= DESIGN_FLOOR * s):
        return ("not_assessed", "below_floor")
    if fires:
        return ("not_established", "bound_exceeds_criterion")
    return ("established", None)


def group_norm(values: list[float]) -> float | None:
    if len(values) == 1:
        return abs(values[0])
    if len(values) == 2:
        return norm3(values[0], values[1], 0.0)
    if len(values) == 3:
        return norm3(values[0], values[1], values[2])
    return None


# ----------------------------------------------------------------------------- coverage (Work §1 table)

def row_sets(group: str, location: str) -> list[list[tuple[str, str, str | None]]]:
    at = location if location in ("end_i", "end_j", "quarter_1", "midspan", "quarter_3") else None
    station = location in ("quarter_1", "midspan", "quarter_3")
    out: list[list[tuple[str, str, str | None]]] = []
    if group == "axial":
        if at:
            out.append([(AXIAL, at, None)])
        if station:
            out.append([(EFFECTIVE, location, None)])
        if location in ("cut_i", "cut_j"):
            out.append([(EFFECTIVE, "end_i" if location == "cut_i" else "end_j", None)])
    elif group == "shear" and at:
        out.append([(SHEAR_Y, at, None), (SHEAR_Z, at, None)])
    elif group == "torsion" and at:
        out.append([(TORSION, at, None)])
    elif group == "bending" and at:
        out.append([(BENDING_Y, at, None), (BENDING_Z, at, None)])
    elif group == "pipe_wall":
        walls = {"wall_end_i": (END_ACTION, "end_i"), "wall_end_j": (END_ACTION, "end_j"), "wall_cut_i": (WALL, "end_i"),
                 "wall_cut_j": (WALL, "end_j"), "wall_quarter_1": (WALL, "quarter_1"), "wall_midspan": (WALL, "midspan"),
                 "wall_quarter_3": (WALL, "quarter_3")}
        if location in walls:
            kind, row_location = walls[location]
            out.append([(kind, row_location, None)])
    elif location == "node":
        if group == "support_force":
            out.append([(SUPPORT_COMPONENT, "node", c) for c in ("Fx", "Fy", "Fz")])
        elif group == "support_moment":
            out.append([(SUPPORT_COMPONENT, "node", c) for c in ("Mx", "My", "Mz")])
        elif group == "support_force_magnitude":
            out.append([(SUPPORT_FORCE, "node", None)])
        elif group == "support_moment_magnitude":
            out.append([(SUPPORT_MOMENT, "node", None)])
    return out


MOMENT_CLASS = {"axial": False, "shear": False, "pipe_wall": False, "support_force": False, "support_force_magnitude": False,
                "torsion": True, "bending": True, "support_moment": True, "support_moment_magnitude": True}


class _Rows:
    def __init__(self, results: list[Any]):
        self.results = results
        self.case_index: dict[tuple, list[int]] = {}
        self.by_sources: dict[tuple, list[int]] = {}
        self.of_combination: dict[str, list[int]] = {}
        for i, row in enumerate(results):
            if not isinstance(row, Mapping):
                continue
            basis = row.get("basis_ref") if isinstance(row.get("basis_ref"), Mapping) else {}
            ref_type, ref_id, kind = basis.get("ref_type"), basis.get("ref_id"), row.get("kind")
            if not isinstance(ref_id, str):
                continue
            if ref_type == "load_case":
                metadata = row.get("metadata") if isinstance(row.get("metadata"), Mapping) else {}
                entity, location = row.get("entity_ref"), metadata.get("location")
                if not isinstance(entity, str) or not isinstance(location, str):
                    continue
                component = (metadata.get("component") or "") if kind == SUPPORT_COMPONENT else None
                self.case_index.setdefault((ref_id, kind, entity, location, component), []).append(i)
            elif ref_type == "combination" and kind != MODULUS_RECORD:
                self.of_combination.setdefault(ref_id, []).append(i)
                sources = tuple(s for s in _items(row.get("source_result_refs")) if isinstance(s, str))
                self.by_sources.setdefault((ref_id, sources), []).append(i)

    def value(self, i: int) -> float:
        v = self.results[i].get("value")
        return float(v) if isinstance(v, (int, float)) and not isinstance(v, bool) else math.nan

    def id(self, i: int) -> str:
        return self.results[i].get("id") or ""

    def coverage(self, case: str, entity: str, group: str, location: str, q: float) -> list[list[int] | None]:
        out: list[list[int] | None] = []
        for specs in row_sets(group, location):
            rows = []
            for kind, row_location, component in specs:
                hits = self.case_index.get((case, kind, entity, row_location, component), [])
                if len(hits) != 1:
                    rows = None
                    break
                rows.append(hits[0])
            if rows is None:
                out.append(None)
                continue
            norm = group_norm([self.value(i) for i in rows])
            out.append(rows if norm is not None and _bits(norm) == _bits(q) else None)
        return out


def _eligible(diagnostics: list[Any], case: str) -> bool:
    wanted = f"diagnostic:numerical-integrity:{case}"
    found = next((d for d in diagnostics if isinstance(d, Mapping) and d.get("id") == wanted), None)
    if found is None:
        return False
    message = found.get("message") if isinstance(found.get("message"), str) else ""
    return found.get("code") == PASSED or (
        found.get("code") == SENSITIVE and f"S11-G recovery guard (R-b'): load case {case} is Sensitive because" in message)


def held_reason(token: Any) -> str | None:
    """Premise (Work `combination-certificate.md` §1): the operand bounds are each operand case's own. A held subtrahend
    (any `subtrahend_support_state` but absent or `own`) is a different system from its load case: not assessed."""
    if token is None or token == "own":
        return None
    _require(isinstance(token, str), "RECORD_SHAPE")
    return (f"its subtrahend is held at another case's support state (subtrahend_support_state {token}), a different "
            f"system from its load case; the operand bounds are each operand case's own")


def support_token(combination: Mapping[str, Any]) -> str | None:
    """The record token of a model combination's `subtrahend_support_state`."""
    if "subtrahend_support_state" not in combination:
        return None
    value = combination["subtrahend_support_state"]
    return value if value in ("own", "minuend_final") else "unrecognized"


def _terms(record: Mapping[str, Any], cases: set[str]) -> list[tuple[str, float]]:
    expression = record.get("expression") if isinstance(record.get("expression"), Mapping) else {}
    basis = record.get("basis")
    if basis == "mechanics":
        terms = _items(expression.get("terms"))
        if any(not isinstance(t, Mapping) or "combination_id" in t or t.get("load_case_id") not in cases for t in terms):
            raise LookupError("a combination operand is outside the certificate")
        return [(t["load_case_id"], float(t["factor"]) if isinstance(t.get("factor"), (int, float)) else math.nan) for t in terms]
    if basis == "result_state_subtraction":
        m, s = expression.get("minuend_id"), expression.get("subtrahend_id")
        if not isinstance(m, str) or not isinstance(s, str):
            raise LookupError("an incomplete expression")
        if m not in cases or s not in cases:
            raise LookupError("a combination operand is outside the certificate")
        return [(m, 1.0), (s, -1.0)]
    if basis == "range_envelope":
        raise LookupError("range-envelope certification is deferred")
    raise LookupError("absolute sums and SRSS are outside the certificate")


def _region(pressure: list[Any], case: str, member: str) -> Mapping[str, Any] | None:
    hits = [r for r in pressure if isinstance(r, Mapping) and r.get("load_case_id") == case and member in _items(r.get("member_pipe_ids"))]
    return hits[0] if len(hits) == 1 else None


def _equal_pressure(pressure: list[Any], minuend: str, subtrahend: str, member: str) -> bool:
    """D-6's premise (Rust `equal_pressure`): bit-identical pressure inputs of the member in both operand cases."""
    a, b = _region(pressure, minuend, member), _region(pressure, subtrahend, member)
    if a is None or b is None:
        return False
    if not all(f in a and f in b and bit_equal(a[f], b[f]) for f in ("p_pa", "external_pressure_increment_pa", "pressure_basis")):
        return False
    return all(_entry_equal(a, b, f, member) for f in ("geometry", "materials"))


def _entry_equal(a: Mapping[str, Any], b: Mapping[str, Any], field: str, member: str) -> bool:
    ea = [e for e in _items(a.get(field)) if isinstance(e, Mapping) and e.get("pipe_id") == member]
    eb = [e for e in _items(b.get(field)) if isinstance(e, Mapping) and e.get("pipe_id") == member]
    return bool(ea) and bool(eb) and bit_equal(ea[0], eb[0])


def within_message(cid: str, subtraction: bool, e: int, n: int, a: int, u: int) -> str:
    states = "the difference of two independently solved states" if subtraction else "a combination of independently solved states"
    return (f"combination {cid}: {e} derived row group(s) are established within their stated bound against the operand cases' "
            f"intended systems; {n} are not established (the bound exceeds the 1e-9 criterion, as where the operands nearly "
            f"cancel, or the comparison is unavailable); {a} are not assessed (below the floor); {u} of its rows are covered by no "
            f"group and remain inspection results. This is a numerical statement about {states}; it does not establish the "
            f"meaning of \"expansion\" or engineering acceptance, and these rows do not count toward the document's numerical "
            f"eligibility. Each group's bound and verdict are stated in contract_evidence.combination_certificate.")


def not_assessed_message(cid: str, reason: str) -> str:
    return (f"combination {cid}: its rows are not assessed against a stated bound ({reason}); they remain inspection results "
            f"and do not count toward the document's numerical eligibility.")


def _diagnostic(cid: str, code: str, message: str) -> dict[str, Any]:
    return {"id": f"{DIAGNOSTIC_PREFIX}{_identity(cid)}", "code": code, "severity": "info", "message": message,
            "source": SOURCE, "affected_refs": [cid]}


def _gates(evidence: Mapping[str, Any]) -> list[tuple[str, bool]] | None:
    if isinstance(evidence.get("combination_gates"), list):
        return [(g.get("combination_id") or "", g.get("withheld") is True) for g in evidence["combination_gates"] if isinstance(g, Mapping)]
    if isinstance(evidence.get("exact_combinations"), list):
        return [(r.get("combination_id") or "", isinstance(r.get("gate"), Mapping) and r["gate"].get("withheld") is True)
                for r in evidence["exact_combinations"] if isinstance(r, Mapping)]
    return None


def check(source: Mapping[str, Any]) -> None:
    """The member's checks (Rust `combination_certificate::check`); a no-op without it."""
    evidence = source.get("contract_evidence") if isinstance(source.get("contract_evidence"), Mapping) else {}
    solved = isinstance(source.get("status"), Mapping) and source["status"].get("mechanics") == "MECHANICS_SOLVED"
    gate_list = _gates(evidence) or []
    member = evidence.get(MEMBER)
    # Absence of the member is no §4 certificate claim, whatever the document's age; it selects no historical
    # semantics. No certificate notice may then stand, under any id. A stated member is read in full, and only on a
    # solved document with a combination; a partial or malformed one is refused, never read as absence (Rust alike).
    if MEMBER not in evidence:
        _require(not any(reserved_notice(d) for d in _items(source.get("diagnostics"))), "PRESENCE")
        return
    _require(solved and bool(gate_list), "PRESENCE")
    _require(_keys(member, MEMBER_KEYS), "SHAPE")
    operand_cases = _array(member.get("operand_cases"))
    records = _array(member.get("combinations"))
    results = _items(source.get("results"))
    diagnostics = _items(source.get("diagnostics"))
    rows = _Rows(results)
    quality = source.get("numerical_quality") if isinstance(source.get("numerical_quality"), Mapping) else {}
    case_order = [c["basis_ref"]["ref_id"] for c in _items(quality.get("cases"))
                  if isinstance(c, Mapping) and isinstance(c.get("basis_ref"), Mapping) and c["basis_ref"].get("ref_type") == "load_case"
                  and isinstance(c["basis_ref"].get("ref_id"), str)]
    cases = set(case_order)
    pressure = _items(evidence.get("pressure"))

    operands: dict[str, list[dict[str, Any]]] = {}
    for case in operand_cases:
        _require(_keys(case, CASE_KEYS), "SHAPE")
        cid = case.get("load_case_id")
        _require(isinstance(cid, str), "SHAPE")
        _require(cid in cases and cid not in operands, "OPERAND_CASE")
        _require(_eligible(diagnostics, cid), "OPERAND_ELIGIBILITY")
        groups, seen = [], set()
        for g in _array(case.get("groups")):
            _require(_keys(g, OPERAND_GROUP_KEYS), "SHAPE")
            entity, location, group, moment, q = g.get("entity"), g.get("location"), g.get("group"), g.get("moment"), g.get("q")
            _require(isinstance(entity, str) and isinstance(location, str) and isinstance(group, str) and isinstance(moment, bool)
                     and isinstance(q, (int, float)) and not isinstance(q, bool), "SHAPE")
            _require((entity, location, group) not in seen, "OPERAND_GROUP_DUPLICATE")
            seen.add((entity, location, group))
            _require(MOMENT_CLASS.get(group, moment) == moment, "OPERAND_GROUP_CLASS")
            floor = g.get("floor_scale")
            _require(isinstance(floor, (int, float)) and not isinstance(floor, bool), "SHAPE")
            bound = g.get("bound")
            _require(bound is None or (isinstance(bound, (int, float)) and not isinstance(bound, bool)), "SHAPE")
            q, floor = float(q), float(floor)
            bound = None if bound is None else float(bound)
            _require(math.isfinite(q) and q >= 0 and math.isfinite(floor) and floor >= 0
                     and (bound is None or (math.isfinite(bound) and bound >= 0)), "OPERAND_GROUP_VALUE")
            expected = [None if s is None else [rows.id(i) for i in s] for s in rows.coverage(cid, entity, group, location, q)]
            _require(bit_equal(g.get("rows"), expected), "OPERAND_ROWS")
            groups.append({"entity": entity, "location": location, "group": group, "q": q, "bound": bound, "floor_scale": floor})
        operands[cid] = groups

    _require(len(records) == len(gate_list), "RECORDS")
    used: set[str] = set()
    expected_diagnostics: list[dict[str, Any]] = []
    for record, (cid, withheld) in zip(records, gate_list):
        _require(_keys(record, RECORD_KEYS) and record.get("combination_id") == cid, "RECORD_SHAPE")
        # The stated expression is the EX record's; on PF it is bound by the model (`model_checks`), as slice 1a's
        # operand records are.
        for stated in _items(evidence.get("exact_combinations")):
            if isinstance(stated, Mapping) and stated.get("combination_id") == cid:
                _require(bit_equal(stated.get("expression"), record.get("expression")) and stated.get("basis") == record.get("basis"), "EXPRESSION")
        rows_of = rows.of_combination.get(cid, [])
        reason = held_reason(record.get("subtrahend_support_state"))
        terms: list[tuple[str, float]] = []
        if reason is not None:
            pass
        elif withheld:
            reason = "withheld by its gate"
        else:
            try:
                terms = _terms(record, cases)
                missing = []
                for case, _ in terms:
                    if not _eligible(diagnostics, case) and case not in missing:
                        missing.append(case)
                if missing:
                    reason = f"operand case(s) {', '.join(missing)} carry no certified rows"
            except LookupError as outside:
                reason = str(outside)
        if reason is not None:
            if not withheld:
                expected_diagnostics.append(_diagnostic(cid, NOT_ASSESSED, not_assessed_message(cid, reason)))
            expected = {"combination_id": cid, "basis": record.get("basis"), "expression": record.get("expression"),
                        "subtrahend_support_state": record.get("subtrahend_support_state"), "assessment": "not_assessed", "reason": reason, "groups": [], "uncovered_row_count": len(rows_of)}
            _require(bit_equal(record, expected), "RECORD")
            continue
        for case, _ in terms:
            _require(case in operands, "OPERAND_CASE_MISSING")
            used.add(case)
        subtraction = record.get("basis") == "result_state_subtraction"
        derived: list[dict[str, Any]] = []
        formed: dict[tuple, int] = {}
        for group in operands[terms[0][0]]:
            magnitude = {"support_force_magnitude": ("support_force", SUPPORT_FORCE),
                         "support_moment_magnitude": ("support_moment", SUPPORT_MOMENT)}.get(group["group"])
            if magnitude is not None:
                components, kind = magnitude
                index = formed.get((group["entity"], group["location"], components, 0))
                if index is None:
                    continue
                component = derived[index]
                candidates = [r for r in rows_of if results[r].get("kind") == kind and results[r].get("entity_ref") == group["entity"]]
                if len(candidates) != 1:
                    continue
                row = candidates[0]
                values = [rows.value(r) for r in component["_rows"]]
                value = rows.value(row)
                if len(values) != 3 or _bits(norm3(*values)) != _bits(value):
                    continue
                root = 0.0 if value == 0.0 else math.nextafter(value, math.inf) - value
                bound = up_add(component["_bound"], root)
                q = abs(value)
                q_hat = max(q, up_add(q, bound))
                v, why = verdict(q, q_hat, bound, component["_floor"])
                derived.append({"json": {"entity": group["entity"], "location": group["location"], "group": group["group"], "row_set": 0,
                                         "rows": [rows.id(row)], "factors_bound": _number(component["_bound"]), "rounding_error": _number(root),
                                         "bound": _number(bound), "q": _number(q), "q_hat": _number(q_hat), "floor_scale": _number(component["_floor"]),
                                         "verdict": v, "reason": why, "equal_pressure": None},
                                "_rows": [row], "_bound": bound, "_floor": component["_floor"]})
                continue
            per_operand = []
            for case, _ in terms:
                own = next((g for g in operands[case] if (g["entity"], g["location"], g["group"]) == (group["entity"], group["location"], group["group"])), None)
                if own is None or own["bound"] is None:
                    break
                per_operand.append((own, own["bound"], rows.coverage(case, own["entity"], own["group"], own["location"], own["q"])))
            if len(per_operand) != len(terms):
                continue
            sets = min(len(c) for _, _, c in per_operand) if per_operand else 0
            for set_index in range(sets):
                operand_rows = [c[set_index] for _, _, c in per_operand]
                if any(r is None for r in operand_rows):
                    continue

                def spec(i):
                    return [(results[r].get("kind"), (results[r].get("metadata") or {}).get("location")) for r in operand_rows[i]]

                if any(spec(i) != spec(0) for i in range(1, len(terms))):
                    continue
                n = len(operand_rows[0])
                found = []
                for c in range(n):
                    hits = rows.by_sources.get((cid, tuple(rows.id(r[c]) for r in operand_rows)), [])
                    if len(hits) != 1:
                        break
                    found.append(hits[0])
                if len(found) != n:
                    continue
                values = [rows.value(r) for r in found]
                errors = [rounding_error(values[c], [(f, rows.value(r[c])) for (_, f), r in zip(terms, operand_rows)]) for c in range(n)]
                factors_bound, floor = 0.0, 0.0
                for (_, f), (own, bound, _) in zip(terms, per_operand):
                    factors_bound = up_add(factors_bound, up_mul(abs(f), bound))
                    floor = max(floor, up_mul(abs(f), own["floor_scale"]))
                error = up_norm(errors)
                bound = up_add(factors_bound, error)
                q = group_norm(values)
                q = math.nan if q is None else q
                q_hat = max(q, up_add(q, bound))
                v, why = verdict(q, q_hat, bound, floor)
                formed[(group["entity"], group["location"], group["group"], set_index)] = len(derived)
                derived.append({"json": {"entity": group["entity"], "location": group["location"], "group": group["group"], "row_set": set_index,
                                         "rows": [rows.id(r) for r in found], "factors_bound": _number(factors_bound), "rounding_error": _number(error),
                                         "bound": _number(bound), "q": _number(q), "q_hat": _number(q_hat), "floor_scale": _number(floor),
                                         "verdict": v, "reason": why, "equal_pressure": None},
                                "_rows": found, "_bound": bound, "_floor": floor})
        if subtraction and pressure:
            for d in derived:
                if d["json"]["group"] != "pipe_wall":
                    continue
                member_id = d["json"]["entity"]
                if not _equal_pressure(pressure, terms[0][0], terms[1][0], member_id):
                    continue
                location = d["json"]["location"]
                location = location[len("wall_"):] if location.startswith("wall_") else location
                elastic = next(({"entity": member_id, "location": location, "group": "axial", "row_set": s}
                                for s in range(4) if (member_id, location, "axial", s) in formed), None)
                d["json"]["equal_pressure"] = {"elastic_group": elastic}
        covered = {r for d in derived for r in d["_rows"]}
        uncovered = sum(1 for r in rows_of if r not in covered)

        def count(v):
            return sum(1 for d in derived if d["json"]["verdict"] == v)

        expected_diagnostics.append(_diagnostic(cid, WITHIN, within_message(cid, subtraction, count("established"), count("not_established"),
                                                                             count("not_assessed"), uncovered)))
        expected = {"combination_id": cid, "basis": record.get("basis"), "expression": record.get("expression"),
                    "subtrahend_support_state": record.get("subtrahend_support_state"), "assessment": "assessed",
                    "reason": None, "groups": [d["json"] for d in derived], "uncovered_row_count": uncovered}
        _require(bit_equal(record, expected), "RECORD")
    stated = [c.get("load_case_id") for c in operand_cases if isinstance(c, Mapping)]
    _require(stated == [c for c in case_order if c in used], "OPERAND_CASES")
    # The diagnostics, whole, and none other in the reserved vocabulary (any id spelling).
    published = [d for d in diagnostics if reserved_notice(d)]
    _require(len(published) == len(expected_diagnostics), "DIAGNOSTICS")
    for expected in expected_diagnostics:
        matching = [d for d in published if d.get("id") == expected["id"]]
        _require(len(matching) == 1 and bit_equal(matching[0], expected), "DIAGNOSTICS")


def model_checks(source: Mapping[str, Any], model: Mapping[str, Any]) -> None:
    """With the captured model: each record's id, basis and expression are the model's (Rust `model_checks`)."""
    evidence = source.get("contract_evidence") if isinstance(source.get("contract_evidence"), Mapping) else {}
    member = evidence.get(MEMBER)
    if not isinstance(member, Mapping):
        # Absent, or malformed (refused by its shape in `check`, Rust alike).
        return
    combinations = [c for c in _items(model.get("combinations")) if isinstance(c, Mapping)]
    for record in _items(member.get("combinations")):
        c = next((c for c in combinations if c.get("id") == record.get("combination_id")), None)
        _require(c is not None and c.get("basis") == record.get("basis"), "MODEL_BINDING")
        if c["basis"] in ("mechanics", "absolute_sum", "srss"):
            expression = {"terms": [{"combination_id": t["combination"], "factor": t.get("factor")} if t.get("combination") is not None
                                    else {"load_case_id": t.get("load_case"), "factor": t.get("factor")} for t in _items(c.get("terms"))]}
        elif c["basis"] == "result_state_subtraction":
            expression = {"minuend_id": c.get("minuend_id"), "subtrahend_id": c.get("subtrahend_id")}
        else:
            expression = {"operand_ids": c.get("operand_ids"), "mode": c.get("mode")}
        _require(bit_equal(record.get("expression"), expression), "MODEL_BINDING")
        _require(record.get("subtrahend_support_state") == support_token(c), "MODEL_BINDING")
    # A combination withheld before the solve (S1's interim) has no gate and so no record; the records are the
    # document's gates (`check`).


def without_member(source: Mapping[str, Any]) -> Mapping[str, Any]:
    """The document without the member, for the unchanged route readers."""
    evidence = source.get("contract_evidence")
    if not isinstance(evidence, Mapping) or MEMBER not in evidence:
        return source
    view = dict(source)
    view["contract_evidence"] = {k: v for k, v in evidence.items() if k != MEMBER}
    return view
