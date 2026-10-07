"""Interval-mode rule evaluation (T3 D2 revision 5b.3 §4.11, option C).

The shared case file ``fixtures/rule_interval/rule_interval_cases.json`` is
consumed here by the Python reference evaluator and in
``core/rules/rule_check_runner/tests/rule_interval_cases.rs`` by the Rust
evaluator; both must reproduce every expected outcome, enclosure bit pattern
and note, which gives Rust/Python parity. An independent oracle then checks
soundness: a float transcription of the ordinary point path and an exact
rational evaluation, at sampled points of every input box, must agree with
each decided outcome and lie inside each enclosure. All values are invented;
nothing here is a professional or code-compliance claim.
"""
from __future__ import annotations

import itertools
import json
import math
import random
import re
import struct
from fractions import Fraction
from pathlib import Path

import pytest

from core.analysis_runs import rule_interval as ri

ROOT = Path(__file__).resolve().parents[1]
CASES_PATH = ROOT / "fixtures" / "rule_interval" / "rule_interval_cases.json"
DOCUMENT = json.loads(CASES_PATH.read_text(encoding="utf-8"))
CASES = DOCUMENT["cases"]

EVERY_FEATURE = {
    "literal", "variable_ref", "unary:negate", "unary:abs", "unary:not",
    "binary:add", "binary:subtract", "binary:multiply", "binary:divide",
    "compare:less_than", "compare:less_than_or_equal", "compare:greater_than",
    "compare:greater_than_or_equal", "compare:equal", "compare:not_equal",
    "logical:and", "logical:or", "select", "aggregate:min", "aggregate:max",
    "interpolate", "lookup:exact", "lookup:step", "unsupported_form", "unsafe_host_access",
}

# D2 §4.11.5's negative controls, each pinned by at least one case.
D2_NEGATIVE_CONTROLS = {
    "abs_straddle_ge", "square_le", "divide_by_zero_range", "not_straddle",
    "two_inputs_interior_extremum", "equal_overlap", "not_equal_overlap",
    "select_unknown_straddle", "interpolate_partly_out_of_range",
}


def nd(x: float) -> float:
    return math.nextafter(x, -math.inf)


def nu(x: float) -> float:
    return math.nextafter(x, math.inf)


def features(node: dict, out: set) -> set:
    kind = node["node"]
    if kind in ("unary", "binary", "compare", "logical"):
        out.add(f"{kind}:{node['operator']}")
    elif kind == "aggregate":
        out.add(f"aggregate:{node['function']}")
    elif kind == "lookup":
        out.add(f"lookup:{node['mode']}")
    else:
        out.add(kind)
    for key in ("operand", "left", "right", "condition", "then", "else", "argument"):
        if key in node:
            features(node[key], out)
    for operand in node.get("operands", []):
        features(operand, out)
    return out


def case_inputs(case: dict) -> list[dict]:
    inputs = []
    for i in case["inputs"]:
        item = {"variable_id": i["variable_id"], "dimension": i["dimension"], "unit_ref": i["unit_ref"],
                "value": ri.from_bits(i["value_bits"]), "bound": ri.from_bits(i["bound_bits"])}
        if "enclosure_bits" in i:  # an explicit enclosure (or none) replaces the bound
            ends = i["enclosure_bits"]
            item["enclosure"] = None if ends is None else (ri.from_bits(ends[0]), ri.from_bits(ends[1]))
        inputs.append(item)
    return inputs


def outcome_of(result: dict) -> dict:
    """The Python result in the case file's ``expected`` form."""
    if result["value"] is None:
        return {"kind": "blocked", "findings": [list(f) for f in result["findings"]]}
    value = result["value"]
    if value["kind"] == "truth":
        return {"kind": "truth", "truth": value["truth"]}
    enclosure = value["enclosure"]
    return {"kind": "quantity", "dimension": value["dimension"], "unit_ref": value["unit_ref"],
            "enclosure": None if enclosure is None else [ri.bits(enclosure[0]), ri.bits(enclosure[1])]}


def test_case_file_shape_and_coverage():
    assert DOCUMENT["document_kind"] == "openpipestress.rule_interval_cases"
    assert len(CASES) >= 60
    ids = [case["case_id"] for case in CASES]
    assert len(ids) == len(set(ids))
    covered = set()
    for case in CASES:
        features(case["formula"], covered)
        for item in case["inputs"]:
            # The readable text matches the bits it annotates.
            assert repr(ri.from_bits(item["value_bits"])) == item["value_text"]
            assert repr(ri.from_bits(item["bound_bits"])) == item["bound_text"]
            if item.get("enclosure_bits") is not None:
                assert len(item["enclosure_bits"]) == 2
    assert EVERY_FEATURE <= covered, EVERY_FEATURE - covered
    assert D2_NEGATIVE_CONTROLS <= {c["case_id"] for c in CASES if c["negative_control"]}
    # Formula literals are short decimals every JSON reader parses exactly.
    for number in re.findall(r'"(?:value|argument|result)": (-?[0-9.eE+-]+)',
                             CASES_PATH.read_text(encoding="utf-8")):
        mantissa, _, exponent = number.lower().partition("e")
        assert len(re.sub(r"[^0-9]", "", mantissa).strip("0")) <= 15, number
        assert abs(int(exponent or 0)) <= 22, number


@pytest.mark.parametrize("case", CASES, ids=[c["case_id"] for c in CASES])
def test_reference_evaluator_matches_the_shared_case(case):
    result = ri.evaluate_interval(case["formula"], case_inputs(case))
    assert outcome_of(result) == case["expected"]
    if case["expected"]["kind"] != "blocked":
        assert [list(n) for n in result["notes"]] == case["notes"]
    if case["negative_control"]:
        assert outcome_of(result) != {"kind": "truth", "truth": "T"}


# -- the independent oracle -----------------------------------------------------

class Block(Exception):
    """The point path blocks at this point: neither pass nor fail."""


def point_value(node: dict, env: dict, exact: bool):
    """The ordinary point path (Rust ``evaluate``), as floats or as exact rationals."""
    kind = node["node"]
    if kind == "literal":
        value = float(node["quantity"]["value"])
        return Fraction(value) if exact else value
    if kind == "variable_ref":
        if node["variable_id"] not in env:
            raise Block
        return env[node["variable_id"]]
    if kind == "unary":
        value = point_value(node["operand"], env, exact)
        return {"negate": lambda v: -v, "abs": abs, "not": lambda v: not v}[node["operator"]](value)
    if kind in ("binary", "compare", "logical"):
        left = point_value(node["left"], env, exact)
        right = point_value(node["right"], env, exact)
        operator = node["operator"]
        if kind == "binary":
            if operator == "divide":
                if right == 0:
                    raise Block
                quotient = left / right
                if not exact and not math.isfinite(quotient):
                    # A same-dimension ratio blocks (NonFiniteInput); other quotients
                    # carry the value, and Block stays the conservative reading.
                    raise Block
                return quotient
            return {"add": left + right, "subtract": left - right, "multiply": left * right}[operator]
        if kind == "compare":
            return {"less_than": left < right, "less_than_or_equal": left <= right,
                    "greater_than": left > right, "greater_than_or_equal": left >= right,
                    "equal": left == right, "not_equal": left != right}[operator]
        return (left and right) if operator == "and" else (left or right)
    if kind == "select":
        condition = point_value(node["condition"], env, exact)
        then_value = point_value(node["then"], env, exact)
        else_value = point_value(node["else"], env, exact)
        return then_value if condition else else_value
    if kind == "aggregate":
        values = [point_value(o, env, exact) for o in node["operands"]]
        if not values:
            raise Block
        return min(values) if node["function"] == "min" else max(values)
    if kind in ("interpolate", "lookup"):
        rows = [(float(r["argument"]), float(r["result"])) for r in node["table"]["rows"]]
        if exact:
            rows = [(Fraction(a), Fraction(r)) for a, r in rows]
        x = point_value(node["argument"], env, exact)
        if not exact and math.isnan(x):
            raise Block  # NonFiniteInput (interpolate, step); TableKeyNotFound (exact)
        first, last = rows[0][0], rows[-1][0]
        for argument, result in rows:
            if argument == x:
                return result
        if x < first or x > last or node.get("mode") == "exact":
            raise Block
        if node.get("mode") == "step":
            return [r for a, r in rows if a <= x][-1]
        for (a0, r0), (a1, r1) in zip(rows, rows[1:]):
            if a0 < x < a1:
                return r0 + (r1 - r0) * ((x - a0) / (a1 - a0))
    raise Block


def samples(q: float, b: float, special: list[float], rng: random.Random, enclosure=False):
    """Float sample points of an input's binding enclosure, and exact extras.

    ``enclosure`` is an explicit (lo, hi) pair, ``None`` for an explicit input
    with no finite enclosure, or ``False`` when the input carries a bound."""
    if enclosure is False:
        if b == 0 or not math.isfinite(b) or b < 0:
            return [q], []
        lo, hi = nd(q - b), nu(q + b)
    elif enclosure is None:
        return [q], []
    else:
        lo, hi = enclosure
    if not (math.isfinite(lo) and math.isfinite(hi)) or lo > hi:
        return [q], []
    points = {lo, hi, min(nu(lo), hi), max(nd(hi), lo), q}
    points.update(v for v in special if lo <= v <= hi)
    for _ in range(3):
        points.add(min(max(lo + (hi - lo) * rng.random(), lo), hi))
    flo, fhi = Fraction(lo), Fraction(hi)
    return sorted(points), [(flo + fhi) / 2, flo + (fhi - flo) / 3]


def table_arguments(node: dict, out: list) -> list:
    if "table" in node:
        for row in node["table"]["rows"]:
            a = float(row["argument"])
            out.extend([a, nd(a), nu(a)])
    for key in ("operand", "left", "right", "condition", "then", "else", "argument"):
        if key in node:
            table_arguments(node[key], out)
    for operand in node.get("operands", []):
        table_arguments(operand, out)
    return out


def oracle_check(formula: dict, inputs: list[dict], outcome: dict, *, negative_control=False, seed=0):
    """Checks ``outcome`` against the point path and exact arithmetic."""
    if outcome["kind"] == "blocked":
        return
    rng = random.Random(seed)
    special = [0.0, -0.0] + table_arguments(formula, [])
    per_input = []
    for item in inputs:
        floats, extras = samples(item["value"], item.get("bound", 0.0), special, rng,
                                 item.get("enclosure", False))
        per_input.append((item["variable_id"], floats, extras))
    combos = []
    for choice in itertools.product(*[floats for _, floats, _ in per_input]):
        env = {vid: v for (vid, _, _), v in zip(per_input, choice)}
        combos.append((env, False))
        combos.append(({k: Fraction(v) for k, v in env.items()}, True))
    for i, (vid, floats, extras) in enumerate(per_input):
        for extra in extras:  # exact interior points, one input at a time
            env = {v: Fraction(f[len(f) // 2]) for v, f, _ in per_input}
            env[vid] = extra
            combos.append((env, True))
    holds_everywhere = True
    for env, exact in combos:
        try:
            value = point_value(formula, env, exact)
        except Block:
            value = Block
        if outcome["kind"] == "truth":
            if outcome["truth"] == "T":
                assert value is True, (env, exact, value)
            elif outcome["truth"] == "F":
                assert value is False, (env, exact, value)
            holds_everywhere = holds_everywhere and value is True
        elif outcome["enclosure"] is not None:
            assert value is not Block, (env, exact)
            lo, hi = (ri.from_bits(text) for text in outcome["enclosure"])
            if exact:
                assert Fraction(lo) <= value <= Fraction(hi), (env, value, lo, hi)
            else:
                assert lo <= value <= hi, (env, value, lo, hi)
    if negative_control:
        # A negative control straddles: some sampled value does not pass.
        assert not holds_everywhere


@pytest.mark.parametrize("case", CASES, ids=[c["case_id"] for c in CASES])
def test_exact_rational_oracle_confirms_the_shared_case(case):
    oracle_check(case["formula"], case_inputs(case), case["expected"],
                 negative_control=case["negative_control"], seed=len(case["case_id"]))


# -- a seeded property over generated formulas ----------------------------------

U_S = "invented_stress_unit"
LITERALS = [0.0, 1.0, -1.0, 0.1, 3.0, -2.5, 7.0, 0.5, 2.0]


def lit(value, ratio=False):
    return {"node": "literal", "quantity": {"value": value, "dimension": "dimensionless" if ratio else "stress",
                                             "unit_ref": "ratio" if ratio else U_S}}


def generate(rng: random.Random, kind: str, depth: int) -> dict:
    leaf = depth == 0 or rng.random() < 0.25
    var = lambda i: {"node": "variable_ref", "variable_id": i}  # noqa: E731
    bi = lambda op, a, b: {"node": "binary", "operator": op, "left": a, "right": b}  # noqa: E731
    table = {"table_id": "invented_property_table", "argument_dimension": "dimensionless",
             "argument_unit_ref": "ratio", "result_dimension": "stress", "result_unit_ref": U_S,
             "rows": [{"argument": a, "result": r} for a, r in [(-2.0, 1.0), (0.0, -3.0), (0.5, 4.0), (3.0, 4.5)]]}
    if kind == "stress":
        if leaf:
            return rng.choice([var("x"), var("y"), lit(rng.choice(LITERALS))])
        pick = rng.randrange(9)
        if pick < 2:
            return bi(("add", "subtract")[pick], generate(rng, "stress", depth - 1), generate(rng, "stress", depth - 1))
        if pick == 2:
            return bi("multiply", generate(rng, "ratio", depth - 1), generate(rng, "stress", depth - 1))
        if pick == 3:
            return bi("divide", generate(rng, "stress", depth - 1), generate(rng, "ratio", depth - 1))
        if pick == 4:
            return {"node": "unary", "operator": rng.choice(["negate", "abs"]),
                    "operand": generate(rng, "stress", depth - 1)}
        if pick == 5:
            return {"node": "aggregate", "function": rng.choice(["min", "max"]),
                    "operands": [generate(rng, "stress", depth - 1) for _ in range(rng.randint(1, 3))]}
        if pick == 6:
            return {"node": "select", "condition": generate(rng, "boolean", depth - 1),
                    "then": generate(rng, "stress", depth - 1), "else": generate(rng, "stress", depth - 1)}
        if pick == 7:
            return {"node": "interpolate", "table": table, "argument": generate(rng, "ratio", depth - 1)}
        return {"node": "lookup", "table": table, "mode": rng.choice(["step", "exact"]),
                "argument": generate(rng, "ratio", depth - 1)}
    if kind == "ratio":
        if leaf:
            return rng.choice([var("z"), lit(rng.choice(LITERALS), ratio=True)])
        pick = rng.randrange(4)
        if pick == 0:
            return bi("divide", generate(rng, "stress", depth - 1), generate(rng, "stress", depth - 1))
        if pick == 3:
            return {"node": "unary", "operator": "abs", "operand": generate(rng, "ratio", depth - 1)}
        return bi(("add", "multiply", "subtract")[pick - 1], generate(rng, "ratio", depth - 1),
                  generate(rng, "ratio", depth - 1))
    operator = rng.choice(["less_than", "less_than_or_equal", "greater_than",
                           "greater_than_or_equal", "equal", "not_equal"])
    if leaf:
        return {"node": "compare", "operator": operator, "left": generate(rng, "stress", 0),
                "right": lit(rng.choice(LITERALS))}
    pick = rng.randrange(5)
    if pick < 2:
        return {"node": "compare", "operator": operator, "left": generate(rng, "stress", depth - 1),
                "right": generate(rng, "stress", depth - 1)}
    if pick == 2:
        return {"node": "logical", "operator": rng.choice(["and", "or"]),
                "left": generate(rng, "boolean", depth - 1), "right": generate(rng, "boolean", depth - 1)}
    if pick == 3:
        return {"node": "unary", "operator": "not", "operand": generate(rng, "boolean", depth - 1)}
    return {"node": "select", "condition": generate(rng, "boolean", depth - 1),
            "then": generate(rng, "boolean", depth - 1), "else": generate(rng, "boolean", depth - 1)}


def test_reference_evaluator_is_sound_on_generated_formulas():
    rng = random.Random(0x1A73)
    tally = {"T": 0, "F": 0, "U": 0, "quantity": 0, "blocked": 0}
    for index in range(250):
        formula = generate(rng, rng.choice(["boolean", "boolean", "stress"]), 3)
        inputs = [
            {"variable_id": "x", "dimension": "stress", "unit_ref": U_S,
             "value": rng.choice([-3.0, 0.0, 1.0, 7.0]), "bound": rng.choice([0.0, 0.25, 1.0, 4.0])},
            {"variable_id": "y", "dimension": "stress", "unit_ref": U_S,
             "value": rng.choice([-2.5, 0.0, 3.0]), "bound": rng.choice([0.0, 1e-9, 1.0])},
            {"variable_id": "z", "dimension": "dimensionless", "unit_ref": "ratio",
             "value": rng.choice([-1.0, 0.0, 0.5, 1.0, 2.0]), "bound": rng.choice([0.0, 0.25, 1.0])},
        ]
        outcome = outcome_of(ri.evaluate_interval(formula, inputs))
        oracle_check(formula, inputs, outcome, seed=index)
        tally[outcome.get("truth", outcome["kind"]) if outcome["kind"] != "quantity" else "quantity"] += 1
    # Every decided and undecided outcome is exercised (blocks are rare here).
    assert all(tally[key] >= 10 for key in ("T", "F", "U", "quantity")), tally


def test_bound_formation_and_bits():
    assert ri.enclosure_from_bound(1.0, 0.0) == (1.0, 1.0)
    assert ri.enclosure_from_bound(1.0, 0.5) == (nd(0.5), nu(1.5))
    assert ri.enclosure_from_bound(0.0, 5e-324) == (-1e-323, 1e-323)
    for bad in (-0.5, math.nan, math.inf):
        assert ri.enclosure_from_bound(1.0, bad) is None
    assert ri.enclosure_from_bound(1.7976931348623157e308, 1.7976931348623157e308) is None
    assert ri.bits(1.0) == "0x3ff0000000000000"
    assert ri.from_bits("0x3ff0000000000000") == 1.0
    assert struct.pack(">d", ri.from_bits(ri.bits(-0.0))) == struct.pack(">d", -0.0)


def test_reference_refuses_invalid_inputs_as_rust_does():
    """RV99 S-3: invalid bounds give no finite enclosure (U), and explicit
    enclosures are validated with Rust's codes and subjects."""
    formula = {"node": "compare", "operator": "less_than_or_equal",
               "left": {"node": "variable_ref", "variable_id": "x"}, "right": lit(10.0)}
    base = {"variable_id": "x", "dimension": "stress", "unit_ref": U_S, "value": 9.5}
    for bound in (math.nan, -1.0, math.inf, -math.inf):
        result = ri.evaluate_interval(formula, [dict(base, bound=bound)])
        assert result["value"] == {"kind": "truth", "truth": "U"}, bound
        assert result["notes"] == [("non_finite_enclosure", "x")]
    for enclosure, code in (((11.0, 9.0), "InvalidReference"), ((math.nan, 1.0), "NonFiniteInput"),
                            ((-math.inf, 0.0), "NonFiniteInput"), ((0.0, math.inf), "NonFiniteInput")):
        result = ri.evaluate_interval(formula, [dict(base, enclosure=enclosure)])
        assert result["value"] is None and result["findings"] == [(code, "x")], enclosure
    # An overlay on an input with no value, a repeated input id, and an empty id.
    result = ri.evaluate_interval(formula, [dict(base, value=None, bound=1.0)])
    assert ("InvalidReference", "x") in result["findings"]
    result = ri.evaluate_interval(formula, [dict(base, bound=1.0), dict(base, bound=1.0)])
    assert result["findings"] == [("DuplicateBinding", "x"), ("DuplicateBinding", "x")]
    result = ri.evaluate_interval(formula, [dict(base, variable_id=" ", bound=1.0)])
    assert ("InvalidReference", "interval_binding") in result["findings"]


def test_decoder_refuses_unknown_forms():
    for node in ({"node": "power"}, {"node": "binary", "operator": "modulo", "left": lit(1.0), "right": lit(1.0)},
                 {"node": "literal", "quantity": {"value": 1.0, "dimension": "bogus", "unit_ref": "u"}},
                 {"node": "lookup", "mode": "nearest", "table": {}, "argument": lit(1.0)}):
        with pytest.raises(ri.FormulaDecodeError):
            ri.evaluate_interval(node, [])
