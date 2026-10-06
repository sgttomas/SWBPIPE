"""Reference interval evaluator for rule formulas (T3 D2 revision 5b.3 §4.11).

Option C, conservative interval binding: the frozen rule formula language
(``expression_evaluator``, grammar 1.0.0) evaluated over binary64 enclosures,
with a three-valued outcome. Python has no rule runner; this module is the
reference used only by the shared parity cases
(``fixtures/rule_interval/rule_interval_cases.json``) and the validation
harness. It mirrors Rust's ``evaluate_interval`` decision for decision:

* every floating operation is followed by one outward ulp step on each end
  (``math.nextafter``), so an enclosure contains both the exact real result
  and the binary64 result the point path computes, for every point of the box;
* comparisons are ``T`` only if they hold everywhere in the box, ``F`` only if
  they fail everywhere, and ``U`` otherwise; ``not``, ``and`` and ``or`` are
  Kleene; ``select`` is eager and joins its branches under ``U``;
* interpolation evaluates the point path's own formula per segment, each
  operation stepped outward, and joins the segments (D2 5b.3);
* eager U: a possible point-path block or a non-finite value anywhere (a
  divisor range containing 0, a table argument outside the table, an exact
  lookup over a range, a non-finite end, an input with no finite enclosure)
  makes the whole result ``U`` (a quantity loses its enclosure);
* structural problems (types, dimensions, units, tables, references, grammar
  version, statuses) block with the point path's finding codes and subjects.

The module has no dependencies, performs no I/O and makes no compliance claim.
"""
from __future__ import annotations

import math
import struct
from typing import Any

GRAMMAR_VERSION = "1.0.0"
SUPPORTED_GRAMMAR_VERSIONS = ("1.0.0",)
TRUE, FALSE, INDETERMINATE = "T", "F", "U"

DIVIDE_BY_ZERO_RANGE = "divide_by_zero_range"
TABLE_ARGUMENT_RANGE = "table_argument_range"
EXACT_LOOKUP_RANGE = "exact_lookup_range"
NON_FINITE_ENCLOSURE = "non_finite_enclosure"

DIMENSIONS = (
    "dimensionless", "length", "mass", "time", "temperature", "temperature_interval",
    "angle", "rotation", "force", "moment", "pressure", "stress", "area", "volume",
    "density", "linear_stiffness", "rotational_stiffness", "displacement", "velocity",
    "acceleration", "thermal_conductivity", "specific_heat",
    "thermal_expansion_coefficient", "second_moment_area", "section_modulus",
    "mass_per_length", "volume_per_length", "slope", "TBD",
)

# The enumerated dimension-product table of grammar 1.0.0 (Rust
# ``DIMENSION_PRODUCTS``), as (factor_a, factor_b, product), commutative.
DIMENSION_PRODUCTS = (
    ("length", "length", "area"),
    ("area", "length", "volume"),
    ("force", "length", "moment"),
    ("pressure", "area", "force"),
    ("stress", "area", "force"),
    ("mass", "acceleration", "force"),
    ("density", "volume", "mass"),
    ("mass_per_length", "length", "mass"),
    ("volume_per_length", "length", "volume"),
    ("linear_stiffness", "length", "force"),
    ("linear_stiffness", "displacement", "force"),
    ("rotational_stiffness", "angle", "moment"),
    ("rotational_stiffness", "rotation", "moment"),
    ("stress", "section_modulus", "moment"),
    ("section_modulus", "length", "second_moment_area"),
    ("velocity", "time", "length"),
    ("acceleration", "time", "velocity"),
    ("thermal_expansion_coefficient", "temperature_interval", "dimensionless"),
)

STATUSES = (
    "MODEL_INCOMPLETE", "MECHANICS_SOLVED", "RULE_INPUTS_INCOMPLETE",
    "USER_RULE_CHECKED", "USER_RULE_FAILED", "HUMAN_REVIEW_REQUIRED",
    "HUMAN_APPROVED_FOR_PROJECT",
)


class FormulaDecodeError(ValueError):
    """The formula JSON is not a decodable expression (Rust ``DecodeError``)."""


def bits(value: float) -> str:
    """``0x`` plus the 16 hex digits of a binary64 value."""
    return "0x" + struct.pack(">d", value).hex()


def from_bits(text: str) -> float:
    return struct.unpack(">d", int(text, 16).to_bytes(8, "big"))[0]


# -- Enclosure arithmetic ----------------------------------------------------

def _outward(lo: float, hi: float) -> tuple[float, float] | None:
    """One outward ulp step on each end of a just-rounded result."""
    if not (math.isfinite(lo) and math.isfinite(hi)):
        return None
    lo, hi = math.nextafter(lo, -math.inf), math.nextafter(hi, math.inf)
    if math.isfinite(lo) and math.isfinite(hi):
        return (lo, hi)
    return None


def _min2(a: float, b: float) -> float:
    return b if b < a else a


def _max2(a: float, b: float) -> float:
    return b if b > a else a


def enclosure_from_bound(value: float, bound: float) -> tuple[float, float] | None:
    """``[next_down(fl(q - b)), next_up(fl(q + b))]``; ``b == 0`` is the point q."""
    if not math.isfinite(value) or not math.isfinite(bound) or bound < 0.0:
        return None
    if bound == 0.0:
        return (value, value)
    return _outward(value - bound, value + bound)


def _add(a, b):
    return _outward(a[0] + b[0], a[1] + b[1])


def _subtract(a, b):
    return _outward(a[0] - b[1], a[1] - b[0])


def _extremes(values):
    lo = hi = values[0]
    for value in values[1:]:
        lo, hi = _min2(lo, value), _max2(hi, value)
    return lo, hi


def _multiply(a, b):
    return _outward(*_extremes([a[0] * b[0], a[0] * b[1], a[1] * b[0], a[1] * b[1]]))


def _divide(a, b):
    return _outward(*_extremes([a[0] / b[0], a[0] / b[1], a[1] / b[0], a[1] / b[1]]))


def _contains_zero(e) -> bool:
    return e[0] <= 0.0 <= e[1]


def _hull(a, b):
    return (_min2(a[0], b[0]), _max2(a[1], b[1]))


def _abs(e):
    lo, hi = e
    if lo >= 0.0:
        return e
    if hi <= 0.0:
        return (-hi, -lo)
    return (0.0, _max2(-lo, hi))


def _is_point(e) -> bool:
    return e[0] == e[1]


def _equal(a, b) -> str:
    if _is_point(a) and _is_point(b) and a[0] == b[0]:
        return TRUE
    if a[1] < b[0] or b[1] < a[0]:
        return FALSE
    return INDETERMINATE


def _compare(operator: str, a, b) -> str:
    if a is None or b is None:
        return INDETERMINATE
    if operator == "less_than_or_equal":
        return TRUE if a[1] <= b[0] else FALSE if a[0] > b[1] else INDETERMINATE
    if operator == "less_than":
        return TRUE if a[1] < b[0] else FALSE if a[0] >= b[1] else INDETERMINATE
    if operator == "greater_than_or_equal":
        return TRUE if a[0] >= b[1] else FALSE if a[1] < b[0] else INDETERMINATE
    if operator == "greater_than":
        return TRUE if a[0] > b[1] else FALSE if a[1] <= b[0] else INDETERMINATE
    if operator == "equal":
        return _equal(a, b)
    if operator == "not_equal":
        return _negate(_equal(a, b))
    raise AssertionError(operator)


def _negate(truth: str) -> str:
    return {TRUE: FALSE, FALSE: TRUE, INDETERMINATE: INDETERMINATE}[truth]


def _and(a: str, b: str) -> str:
    if FALSE in (a, b):
        return FALSE
    return TRUE if a == b == TRUE else INDETERMINATE


def _or(a: str, b: str) -> str:
    if TRUE in (a, b):
        return TRUE
    return FALSE if a == b == FALSE else INDETERMINATE


# -- Formula decoding (Rust ``rule_pack_document::decode_expression``) --------

def _member(node: dict, key: str) -> Any:
    if key not in node:
        raise FormulaDecodeError(f"missing required member '{key}'")
    return node[key]


def _string(node: dict, key: str) -> str:
    value = _member(node, key)
    if not isinstance(value, str):
        raise FormulaDecodeError(f"member '{key}' must be a string")
    return value


def _number(node: dict, key: str) -> float:
    value = _member(node, key)
    if isinstance(value, bool) or not isinstance(value, (int, float)):
        raise FormulaDecodeError(f"member '{key}' must be a finite number")
    return float(value)


def _dimension(token: str) -> str:
    if token not in DIMENSIONS:
        raise FormulaDecodeError(f"unknown dimension token '{token}'")
    return token


def _flag(node: dict, key: str) -> bool:
    value = node.get(key, True)
    return value if isinstance(value, bool) else True


def _quantity(node: Any) -> dict:
    if not isinstance(node, dict):
        raise FormulaDecodeError("expected a JSON object")
    return {
        "value": _number(node, "value"),
        "dimension": _dimension(_string(node, "dimension")),
        "unit_ref": _string(node, "unit_ref"),
        "unit_required": _flag(node, "unit_required"),
        "dimension_check_required": _flag(node, "dimension_check_required"),
    }


def _table(node: Any) -> dict:
    if not isinstance(node, dict):
        raise FormulaDecodeError("expected a JSON object")
    argument_dimension = _dimension(_string(node, "argument_dimension"))
    result_dimension = _dimension(_string(node, "result_dimension"))
    rows = _member(node, "rows")
    if not isinstance(rows, list):
        raise FormulaDecodeError("member 'rows' must be an array")
    decoded = []
    for row in rows:
        if not isinstance(row, dict):
            raise FormulaDecodeError("expected a JSON object")
        decoded.append((_number(row, "argument"), _number(row, "result")))
    return {
        "table_id": _string(node, "table_id"),
        "argument_dimension": argument_dimension,
        "argument_unit_ref": _string(node, "argument_unit_ref"),
        "result_dimension": result_dimension,
        "result_unit_ref": _string(node, "result_unit_ref"),
        "rows": decoded,
    }


_OPERATORS = {
    "unary": ("negate", "abs", "not"),
    "binary": ("add", "subtract", "multiply", "divide"),
    "compare": ("less_than", "less_than_or_equal", "greater_than",
                "greater_than_or_equal", "equal", "not_equal"),
    "logical": ("and", "or"),
}


def decode_formula(node: Any) -> tuple:
    """Decode node-tagged formula JSON into nested tuples."""
    if not isinstance(node, dict):
        raise FormulaDecodeError("expected a JSON object")
    kind = _string(node, "node")
    if kind == "literal":
        return ("literal", _quantity(_member(node, "quantity")))
    if kind == "variable_ref":
        return ("variable_ref", _string(node, "variable_id"))
    if kind in _OPERATORS:
        operator = _string(node, "operator")
        if operator not in _OPERATORS[kind]:
            raise FormulaDecodeError(f"unknown {kind} operator '{operator}'")
        if kind == "unary":
            return ("unary", operator, decode_formula(_member(node, "operand")))
        return (kind, operator, decode_formula(_member(node, "left")),
                decode_formula(_member(node, "right")))
    if kind == "select":
        return ("select", decode_formula(_member(node, "condition")),
                decode_formula(_member(node, "then")), decode_formula(_member(node, "else")))
    if kind == "aggregate":
        function = _string(node, "function")
        if function not in ("min", "max"):
            raise FormulaDecodeError(f"unknown aggregate function '{function}'")
        operands = _member(node, "operands")
        if not isinstance(operands, list):
            raise FormulaDecodeError("member 'operands' must be an array")
        return ("aggregate", function, tuple(decode_formula(o) for o in operands))
    if kind == "interpolate":
        return ("table", None, _table(_member(node, "table")),
                decode_formula(_member(node, "argument")))
    if kind == "lookup":
        mode = _string(node, "mode")
        if mode not in ("exact", "step"):
            raise FormulaDecodeError(f"unknown lookup mode '{mode}'")
        return ("table", mode, _table(_member(node, "table")),
                decode_formula(_member(node, "argument")))
    if kind == "unsupported_form":
        return ("unsupported_form", _string(node, "form_id"))
    if kind == "unsafe_host_access":
        return ("unsafe_host_access", _string(node, "request"))
    raise FormulaDecodeError(f"unknown expression node kind '{kind}'")


# -- Evaluation ---------------------------------------------------------------

class _Blocked(Exception):
    """A point-path structural block (the Rust evaluator's ``?`` return)."""


class _State:
    def __init__(self, bindings: dict, overlays: dict):
        self.bindings, self.overlays = bindings, overlays
        self.findings: list[tuple[str, str]] = []
        self.notes: list[tuple[str, str]] = []
        self.sources: list[str] = []

    def block(self, code: str, subject: str):
        self.findings.append((code, subject))
        raise _Blocked

    def note(self, code: str, subject: str) -> None:
        self.notes.append((code, subject))

    def finite(self, enclosure, subject: str):
        if enclosure is None:
            self.note(NON_FINITE_ENCLOSURE, subject)
        return enclosure


def _meta(dimension: str, unit_ref: str) -> dict:
    return {"dimension": dimension, "unit_ref": unit_ref}


def _units_match(a: dict, b: dict) -> bool:
    return a["unit_ref"].strip() == b["unit_ref"].strip()


def _product(left: str, right: str) -> str | None:
    for a, b, product in DIMENSION_PRODUCTS:
        if (a, b) in ((left, right), (right, left)):
            return product
    return None


def _quotient(numerator: str, denominator: str):
    candidates: list[str] = []
    for a, b, product in DIMENSION_PRODUCTS:
        if product != numerator:
            continue
        if b == denominator and a not in candidates:
            candidates.append(a)
        if a == denominator and b not in candidates:
            candidates.append(b)
    if not candidates:
        return "unrepresentable"
    return candidates[0] if len(candidates) == 1 else "ambiguous"


def _binary_meta(state: _State, operator: str, left: dict, right: dict) -> dict:
    """The point path's dimension and unit algebra (``eval_binary``)."""
    if operator in ("add", "subtract"):
        if left["dimension"] != right["dimension"]:
            state.block("DimensionMismatch", "add_subtract")
        if not _units_match(left, right):
            state.block("UnitMismatch", "add_subtract")
        return left
    if operator == "multiply":
        if left["dimension"] == "dimensionless":
            return right
        if right["dimension"] == "dimensionless":
            return left
        product = _product(left["dimension"], right["dimension"])
        if product is None:
            state.block("UnsupportedExpressionForm", "multiply")
        if product == "dimensionless":
            return _meta(product, "ratio")
        a, b = sorted((left["unit_ref"].strip(), right["unit_ref"].strip()))
        return _meta(product, f"{a}*{b}")
    # divide
    if right["dimension"] == "dimensionless":
        return left
    if left["dimension"] == right["dimension"]:
        if not _units_match(left, right):
            state.block("UnitMismatch", "divide")
        return _meta("dimensionless", "ratio")
    quotient = _quotient(left["dimension"], right["dimension"])
    if quotient in ("ambiguous", "unrepresentable"):
        state.block("UnsupportedExpressionForm", "divide")
    if quotient == "dimensionless":
        return _meta(quotient, "ratio")
    return _meta(quotient, f"{left['unit_ref'].strip()}/{right['unit_ref'].strip()}")


def _eval(state: _State, expression: tuple):
    kind = expression[0]
    if kind == "literal":
        quantity = expression[1]
        if not math.isfinite(quantity["value"]):
            state.block("NonFiniteInput", "literal")
        if not (quantity["unit_required"] and quantity["dimension_check_required"]
                and quantity["unit_ref"].strip()):
            state.block("UnitMetadataMissing", "literal")
        v = quantity["value"]
        return ("q", _meta(quantity["dimension"], quantity["unit_ref"]), (v, v))
    if kind == "variable_ref":
        variable_id = expression[1]
        if not variable_id.strip():
            state.block("InvalidReference", "variable_ref")
        if variable_id not in state.bindings:
            state.block("MissingVariable", variable_id)
        binding = state.bindings[variable_id]
        if binding is None:
            state.block("MissingRequiredValue", variable_id)
        state.sources.append(variable_id)
        meta = _meta(binding["dimension"], binding["unit_ref"])
        if variable_id in state.overlays:
            enclosure = state.overlays[variable_id]
            if enclosure is None:
                state.note(NON_FINITE_ENCLOSURE, variable_id)
        else:
            enclosure = (binding["value"], binding["value"])
        return ("q", meta, enclosure)
    if kind == "unary":
        operator, value = expression[1], _eval(state, expression[2])
        if operator == "not":
            if value[0] != "b":
                state.block("TypeMismatch", "unary_not")
            return ("b", _negate(value[1]))
        if value[0] != "q":
            state.block("TypeMismatch", "unary_negate" if operator == "negate" else "unary_abs")
        enclosure = value[2]
        if enclosure is not None:
            enclosure = (-enclosure[1], -enclosure[0]) if operator == "negate" else _abs(enclosure)
        return ("q", value[1], enclosure)
    if kind == "binary":
        operator = expression[1]
        left, right = _eval(state, expression[2]), _eval(state, expression[3])
        if left[0] != "q" or right[0] != "q":
            state.block("TypeMismatch", "binary_expression")
        meta = _binary_meta(state, operator, left[1], right[1])
        a, b = left[2], right[2]
        if operator == "divide":
            if b is None or _contains_zero(b):
                state.note(DIVIDE_BY_ZERO_RANGE, "divide")
                enclosure = None
            else:
                enclosure = None if a is None else state.finite(_divide(a, b), "divide")
        elif a is None or b is None:
            enclosure = None
        else:
            operation = {"add": _add, "subtract": _subtract, "multiply": _multiply}[operator]
            enclosure = state.finite(operation(a, b), operator)
        return ("q", meta, enclosure)
    if kind == "compare":
        operator = expression[1]
        left, right = _eval(state, expression[2]), _eval(state, expression[3])
        if left[0] != "q" or right[0] != "q":
            state.block("TypeMismatch", "comparison")
        if left[1]["dimension"] != right[1]["dimension"]:
            state.block("DimensionMismatch", "comparison")
        if not _units_match(left[1], right[1]):
            state.block("UnitMismatch", "comparison")
        return ("b", _compare(operator, left[2], right[2]))
    if kind == "logical":
        operator = expression[1]
        left, right = _eval(state, expression[2]), _eval(state, expression[3])
        if left[0] != "b" or right[0] != "b":
            state.block("TypeMismatch", "logical_expression")
        return ("b", _and(left[1], right[1]) if operator == "and" else _or(left[1], right[1]))
    if kind == "select":
        condition = _eval(state, expression[1])
        then_value = _eval(state, expression[2])
        else_value = _eval(state, expression[3])
        if condition[0] != "b":
            state.block("TypeMismatch", "select_condition")
        if then_value[0] == "q" and else_value[0] == "q":
            if then_value[1]["dimension"] != else_value[1]["dimension"]:
                state.block("DimensionMismatch", "select_branches")
            if not _units_match(then_value[1], else_value[1]):
                state.block("UnitMismatch", "select_branches")
        elif then_value[0] != else_value[0]:
            state.block("TypeMismatch", "select_branches")
        truth = condition[1]
        if truth == TRUE:
            return then_value
        if truth == FALSE:
            return else_value
        if then_value[0] == "b":
            return ("b", then_value[1] if then_value[1] == else_value[1] else INDETERMINATE)
        a, b = then_value[2], else_value[2]
        return ("q", then_value[1], None if a is None or b is None else _hull(a, b))
    if kind == "aggregate":
        function, operands = expression[1], expression[2]
        if not operands:
            state.block("UnsupportedExpressionForm", function)
        values = []
        for operand in operands:
            value = _eval(state, operand)
            if value[0] != "q":
                state.block("TypeMismatch", function)
            values.append(value)
        first = values[0][1]
        for value in values[1:]:
            if value[1]["dimension"] != first["dimension"]:
                state.block("DimensionMismatch", function)
            if not _units_match(first, value[1]):
                state.block("UnitMismatch", function)
        selected = values[0][2]
        pick = _min2 if function == "min" else _max2
        for value in values[1:]:
            e = value[2]
            selected = None if selected is None or e is None else (
                pick(selected[0], e[0]), pick(selected[1], e[1]))
        return ("q", first, selected)
    if kind == "table":
        return _eval_table(state, expression[1], expression[2], expression[3])
    if kind == "unsupported_form":
        state.block("UnsupportedExpressionForm", expression[1])
    if kind == "unsafe_host_access":
        state.block("UnsafeConstruct", expression[1])
    raise AssertionError(kind)


def _table_valid(state: _State, table: dict, minimum_rows: int) -> bool:
    valid = True
    subject = table["table_id"].strip() or "table"
    if not table["table_id"].strip():
        valid = False
        state.findings.append(("TableMalformed", "table"))
    if not table["argument_unit_ref"].strip() or not table["result_unit_ref"].strip():
        valid = False
        state.findings.append(("TableMalformed", subject))
    if len(table["rows"]) < minimum_rows:
        valid = False
        state.findings.append(("TableMalformed", subject))
    for argument, result in table["rows"]:
        if not (math.isfinite(argument) and math.isfinite(result)):
            valid = False
            state.findings.append(("TableMalformed", subject))
    for (a, _), (b, _) in zip(table["rows"], table["rows"][1:]):
        if not a < b:
            valid = False
            state.findings.append(("TableMalformed", subject))
    return valid


def _eval_table(state: _State, mode: str | None, table: dict, argument_expression: tuple):
    valid = _table_valid(state, table, 2 if mode is None else 1)
    argument = _eval(state, argument_expression)
    if not valid:
        raise _Blocked
    subject = table["table_id"].strip()
    if argument[0] != "q":
        state.block("TypeMismatch", subject)
    if argument[1]["dimension"] != table["argument_dimension"]:
        state.block("DimensionMismatch", subject)
    if argument[1]["unit_ref"].strip() != table["argument_unit_ref"].strip():
        state.block("UnitMismatch", subject)
    rows, x = table["rows"], argument[2]
    first, last = rows[0][0], rows[-1][0]
    meta = _meta(table["result_dimension"], table["result_unit_ref"].strip())
    if mode == "exact":
        if x is not None and _is_point(x):
            for row_argument, row_result in rows:
                if row_argument == x[0]:
                    return ("q", meta, (row_result, row_result))
            state.block("TableOutOfRange" if x[0] < first or x[0] > last
                        else "TableKeyNotFound", subject)
        state.note(EXACT_LOOKUP_RANGE, subject)
        return ("q", meta, None)
    if x is None or not (first <= x[0] and x[1] <= last):
        state.note(TABLE_ARGUMENT_RANGE, subject)
        return ("q", meta, None)
    if mode == "step":
        def governing(value):
            index = 0
            for candidate, (row_argument, _) in enumerate(rows):
                if row_argument <= value:
                    index = candidate
            return index
        start, end = governing(x[0]), governing(x[1])
        enclosure = (rows[start][1], rows[start][1])
        for _, row_result in rows[start + 1:end + 1]:
            enclosure = _hull(enclosure, (row_result, row_result))
        return ("q", meta, enclosure)
    return ("q", meta, state.finite(_interpolate(rows, x), subject))


def _interpolate(rows, x):
    if _is_point(x):
        for row_argument, row_result in rows:
            if row_argument == x[0]:
                return (row_result, row_result)
    joined = None
    for (a0, r0), (a1, r1) in zip(rows, rows[1:]):
        if not (a0 < x[1] and x[0] < a1):
            continue
        segment = _segment(a0, r0, a1, r1, (_max2(x[0], a0), _min2(x[1], a1)))
        if segment is None:
            return None
        joined = segment if joined is None else _hull(joined, segment)
    return joined


def _segment(a0, r0, a1, r1, x):
    """The point path's ``r0 + (r1 - r0) * ((x - a0) / (a1 - a0))``, outward."""
    rise = _subtract((r1, r1), (r0, r0))
    offset = _subtract(x, (a0, a0))
    run = _subtract((a1, a1), (a0, a0))
    if rise is None or offset is None or run is None or _contains_zero(run):
        return None
    fraction = _divide(offset, run)
    scaled = None if fraction is None else _multiply(rise, fraction)
    return None if scaled is None else _add((r0, r0), scaled)


def _semver(text: str) -> bool:
    parts = text.split(".")
    return len(parts) == 3 and all(
        part and len(part) <= 9 and part.isascii() and part.isdigit()
        and (part == "0" or not part.startswith("0")) for part in parts)


def evaluate_interval(
    formula: Any,
    inputs: list[dict],
    *,
    required_variable_ids: list[str] | None = None,
    statuses: list[str] | None = None,
    grammar_version: str = GRAMMAR_VERSION,
) -> dict:
    """Evaluate decoded or JSON formula ``formula`` in interval mode.

    ``inputs`` are ``{variable_id, value, dimension, unit_ref}`` with an
    optional ``bound`` (b > 0 overlays ``enclosure_from_bound(value, b)``) or an
    explicit ``enclosure`` (a ``(lo, hi)`` pair, or ``None`` for an input with
    no finite enclosure). Returns ``{"findings", "value", "notes",
    "statuses", "source_variable_ids"}``; ``value`` is ``None`` when blocked,
    else ``{"kind": "truth", "truth"}`` or ``{"kind": "quantity",
    "enclosure", "dimension", "unit_ref"}``.
    """
    expression = formula if isinstance(formula, tuple) else decode_formula(formula)
    findings: list[tuple[str, str]] = []
    declared = grammar_version.strip()
    if not declared or not _semver(declared) or declared not in SUPPORTED_GRAMMAR_VERSIONS:
        findings.append(("UnsupportedGrammarVersion", "grammar_version"))
    collected: list[str] = []
    for status in (statuses if statuses is not None else ["MECHANICS_SOLVED"]) or [None]:
        if status is None:
            collected.append("RULE_INPUTS_INCOMPLETE")
        elif status == "HUMAN_APPROVED_FOR_PROJECT":
            findings.append(("StatusBoundaryViolation", "analysis_status"))
            collected.append("HUMAN_REVIEW_REQUIRED")
        elif status not in collected:
            collected.append(status)
    bindings: dict = {}
    overlays: dict = {}
    for item in inputs:
        variable_id = item["variable_id"]
        if not variable_id.strip():
            findings.append(("InvalidReference", "binding"))
            continue
        if variable_id in bindings:
            findings.append(("DuplicateBinding", variable_id))
            continue
        if item.get("value") is None:
            bindings[variable_id] = None
            continue
        if not math.isfinite(item["value"]):
            findings.append(("NonFiniteInput", variable_id))
            continue
        if not item["unit_ref"].strip():
            findings.append(("UnitMetadataMissing", variable_id))
            continue
        bindings[variable_id] = item
        if "enclosure" in item:
            overlays[variable_id] = item["enclosure"]
        elif item.get("bound", 0.0) > 0.0:
            overlays[variable_id] = enclosure_from_bound(item["value"], item["bound"])
    seen: set[str] = set()
    for variable_id in required_variable_ids or []:
        if not variable_id.strip():
            findings.append(("InvalidReference", "required_variable"))
        elif variable_id in seen:
            findings.append(("DuplicateBinding", variable_id))
        else:
            seen.add(variable_id)
            if bindings.get(variable_id) is None:
                findings.append(("MissingRequiredValue", variable_id))
    state = _State(bindings, overlays)
    state.findings = findings
    try:
        value = _eval(state, expression)
    except _Blocked:
        value = None
    result_value = None
    if not state.findings and value is not None:
        indeterminate = bool(state.notes)
        if value[0] == "b":
            result_value = {"kind": "truth",
                            "truth": INDETERMINATE if indeterminate else value[1]}
        else:
            result_value = {"kind": "quantity",
                            "enclosure": None if indeterminate else value[2],
                            "dimension": value[1]["dimension"],
                            "unit_ref": value[1]["unit_ref"]}
    return {
        "findings": state.findings,
        "value": result_value,
        "notes": state.notes,
        "statuses": collected,
        "source_variable_ids": sorted(set(state.sources)),
    }
