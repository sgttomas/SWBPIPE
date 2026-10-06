"""RV99's own oracles for S-I1 (independent of I73's code).

* ``point_eval``: a float transcription of the base point path
  (``expression_evaluator::evaluate``), used only to place limits near the
  point value when generating cases. The authoritative point path in the
  checks is the Rust base code itself (run by the harness).
* ``exact_eval``: the same formula over exact rationals (``Fraction``), with
  eager undefinedness (any zero divisor, out-of-range table argument or
  exact-lookup miss anywhere makes the whole result undefined, as the point
  path blocks eagerly).

Formulas are node-tagged dicts (the rule-pack JSON shape) whose numbers are
Python floats. All values are invented.
"""
from __future__ import annotations

import math
from fractions import Fraction


class Undef(Exception):
    pass


def _rows(table):
    return [(r["argument"], r["result"]) for r in table["rows"]]


# ---------------------------------------------------------------- point path
def point_eval(node, env):
    """Returns ('q', float) or ('b', bool); raises Undef on a block."""
    k = node["node"]
    if k == "literal":
        return ("q", node["quantity"]["value"])
    if k == "variable_ref":
        return ("q", env[node["variable_id"]])
    if k == "unary":
        t, v = point_eval(node["operand"], env)
        op = node["operator"]
        if op == "not":
            return ("b", not v)
        return ("q", -v if op == "negate" else abs(v))
    if k == "binary":
        _, a = point_eval(node["left"], env)
        _, b = point_eval(node["right"], env)
        op = node["operator"]
        if op == "add":
            return ("q", a + 1.0 * b)
        if op == "subtract":
            return ("q", a + -1.0 * b)
        if op == "multiply":
            return ("q", a * b)
        if b == 0.0:
            raise Undef("div0")
        try:
            return ("q", a / b)
        except OverflowError:
            return ("q", math.copysign(math.inf, a) * math.copysign(1.0, b))
    if k == "compare":
        _, a = point_eval(node["left"], env)
        _, b = point_eval(node["right"], env)
        return ("b", _cmp(node["operator"], a, b))
    if k == "logical":
        _, a = point_eval(node["left"], env)
        _, b = point_eval(node["right"], env)
        return ("b", (a and b) if node["operator"] == "and" else (a or b))
    if k == "select":
        _, c = point_eval(node["condition"], env)
        t = point_eval(node["then"], env)
        e = point_eval(node["else"], env)
        return t if c else e
    if k == "aggregate":
        vals = [point_eval(o, env)[1] for o in node["operands"]]
        s = vals[0]
        for v in vals[1:]:
            s = min(s, v) if node["function"] == "min" else max(s, v)
        return ("q", s)
    if k in ("interpolate", "lookup"):
        _, x = point_eval(node["argument"], env)
        rows = _rows(node["table"])
        first, last = rows[0][0], rows[-1][0]
        mode = node.get("mode")
        if mode == "exact":
            for a, r in rows:
                if a == x:
                    return ("q", r)
            raise Undef("exact")
        if x != x or x < first or x > last:
            raise Undef("range")
        if mode == "step":
            g = None
            for a, r in rows:
                if a <= x:
                    g = r
            return ("q", g)
        for a, r in rows:
            if a == x:
                return ("q", r)
        for (a0, r0), (a1, r1) in zip(rows, rows[1:]):
            if a0 < x < a1:
                return ("q", r0 + (r1 - r0) * ((x - a0) / (a1 - a0)))
        raise Undef("bracket")
    raise Undef(k)


def _cmp(op, a, b):
    return {
        "less_than": a < b,
        "less_than_or_equal": a <= b,
        "greater_than": a > b,
        "greater_than_or_equal": a >= b,
        "equal": a == b,
        "not_equal": a != b,
    }[op]


# ---------------------------------------------------------------- exact path
def exact_eval(node, env):
    """Exact rational semantics; env maps ids to Fraction. Raises Undef."""
    k = node["node"]
    if k == "literal":
        return ("q", Fraction(node["quantity"]["value"]))
    if k == "variable_ref":
        return ("q", env[node["variable_id"]])
    if k == "unary":
        t, v = exact_eval(node["operand"], env)
        op = node["operator"]
        if op == "not":
            return ("b", not v)
        return ("q", -v if op == "negate" else abs(v))
    if k == "binary":
        _, a = exact_eval(node["left"], env)
        _, b = exact_eval(node["right"], env)
        op = node["operator"]
        if op == "add":
            return ("q", a + b)
        if op == "subtract":
            return ("q", a - b)
        if op == "multiply":
            return ("q", a * b)
        if b == 0:
            raise Undef("div0")
        return ("q", a / b)
    if k == "compare":
        _, a = exact_eval(node["left"], env)
        _, b = exact_eval(node["right"], env)
        return ("b", _cmp(node["operator"], a, b))
    if k == "logical":
        _, a = exact_eval(node["left"], env)
        _, b = exact_eval(node["right"], env)
        return ("b", (a and b) if node["operator"] == "and" else (a or b))
    if k == "select":
        _, c = exact_eval(node["condition"], env)
        t = exact_eval(node["then"], env)
        e = exact_eval(node["else"], env)
        return t if c else e
    if k == "aggregate":
        vals = [exact_eval(o, env)[1] for o in node["operands"]]
        return ("q", min(vals) if node["function"] == "min" else max(vals))
    if k in ("interpolate", "lookup"):
        _, x = exact_eval(node["argument"], env)
        rows = [(Fraction(a), Fraction(r)) for a, r in _rows(node["table"])]
        first, last = rows[0][0], rows[-1][0]
        mode = node.get("mode")
        if mode == "exact":
            for a, r in rows:
                if a == x:
                    return ("q", r)
            raise Undef("exact")
        if x < first or x > last:
            raise Undef("range")
        if mode == "step":
            g = None
            for a, r in rows:
                if a <= x:
                    g = r
            return ("q", g)
        for a, r in rows:
            if a == x:
                return ("q", r)
        for (a0, r0), (a1, r1) in zip(rows, rows[1:]):
            if a0 < x < a1:
                return ("q", r0 + (r1 - r0) * ((x - a0) / (a1 - a0)))
        raise Undef("bracket")
    raise Undef(k)


def nd(x):
    return math.nextafter(x, -math.inf)


def nu(x):
    return math.nextafter(x, math.inf)


def my_enclosure(q, b):
    """RV99's own transcription of D2 §4.11.2's input enclosure."""
    if b == 0.0:
        return (q, q)
    if not (math.isfinite(q) and math.isfinite(b)) or b < 0.0:
        return None
    lo, hi = q - b, q + b
    if not (math.isfinite(lo) and math.isfinite(hi)):
        return None
    lo, hi = nd(lo), nu(hi)
    if not (math.isfinite(lo) and math.isfinite(hi)):
        return None
    return (lo, hi)
