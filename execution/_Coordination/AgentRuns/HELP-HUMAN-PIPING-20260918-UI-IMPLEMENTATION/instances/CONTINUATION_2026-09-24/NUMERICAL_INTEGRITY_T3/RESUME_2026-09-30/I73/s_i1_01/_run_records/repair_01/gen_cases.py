"""I73 S-I1: generator for fixtures/rule_interval/rule_interval_cases.json.

Expected outcomes are derived here by hand, case by case, from T3 D2 revision
5b.3 §4.11.3: each quantity case spells out its own chain of outward interval
operations below (a five-line arithmetic, independent of both evaluators'
tree walkers, structural checks and table logic), and each truth case states
its outcome. Neither the Rust nor the Python evaluator is run here. All
values are invented. Usage: python gen_cases.py <output.json>
"""
import json
import math
import re
import struct
import sys

INF = math.inf
nd = lambda x: math.nextafter(x, -INF)  # noqa: E731
nu = lambda x: math.nextafter(x, INF)  # noqa: E731


def bits(x):
    return "0x" + struct.pack(">d", x).hex()


# -- hand arithmetic (D2 §4.11.3) -------------------------------------------
def bound(q, b):
    return (q, q) if b == 0 else (nd(q - b), nu(q + b))


def add(a, b):
    return (nd(a[0] + b[0]), nu(a[1] + b[1]))


def sub(a, b):
    return (nd(a[0] - b[1]), nu(a[1] - b[0]))


def mul(a, b):
    p = [a[0] * b[0], a[0] * b[1], a[1] * b[0], a[1] * b[1]]
    return (nd(min(p)), nu(max(p)))


def div(a, b):
    assert not (b[0] <= 0 <= b[1])
    p = [a[0] / b[0], a[0] / b[1], a[1] / b[0], a[1] / b[1]]
    return (nd(min(p)), nu(max(p)))


def pt(x):
    return (x, x)


def hull(a, b):
    return (min(a[0], b[0]), max(a[1], b[1]))


def segment(r0, r1, a0, a1, x):
    """r0 + (r1 - r0) * ((x - a0) / (a1 - a0)), every operation outward."""
    return add(pt(r0), mul(sub(pt(r1), pt(r0)), div(sub(x, pt(a0)), sub(pt(a1), pt(a0)))))


# -- formula builders -----------------------------------------------------------
U_S, U_R = "invented_stress_unit", "ratio"


def lit(value, dimension="stress", unit=U_S):
    s = repr(float(value))
    mantissa, _, exponent = s.partition("e")
    digits = re.sub(r"[^0-9]", "", mantissa).strip("0")
    assert len(digits) <= 15 and abs(int(exponent or 0)) <= 22, s  # exact in any JSON reader
    return {"node": "literal", "quantity": {"value": value, "dimension": dimension, "unit_ref": unit}}


def rlit(value):
    return lit(value, "dimensionless", U_R)


def var(i):
    return {"node": "variable_ref", "variable_id": i}


def bi(op, l, r):
    return {"node": "binary", "operator": op, "left": l, "right": r}


def cmp(op, l, r):
    return {"node": "compare", "operator": op, "left": l, "right": r}


def lg(op, l, r):
    return {"node": "logical", "operator": op, "left": l, "right": r}


def un(op, x):
    return {"node": "unary", "operator": op, "operand": x}


def sel(c, t, e):
    return {"node": "select", "condition": c, "then": t, "else": e}


def agg(f, *xs):
    return {"node": "aggregate", "function": f, "operands": list(xs)}


def table(tid, rows, arg_dim="dimensionless", arg_unit=U_R):
    return {"table_id": tid, "argument_dimension": arg_dim, "argument_unit_ref": arg_unit,
            "result_dimension": "stress", "result_unit_ref": U_S,
            "rows": [{"argument": a, "result": r} for a, r in rows]}


def interp(t, x):
    return {"node": "interpolate", "table": t, "argument": x}


def lookup(t, mode, x):
    return {"node": "lookup", "table": t, "mode": mode, "argument": x}


def inp(i, q, b, dimension="stress", unit=U_S):
    return {"variable_id": i, "dimension": dimension, "unit_ref": unit,
            "value_bits": bits(q), "bound_bits": bits(b), "value_text": repr(q), "bound_text": repr(b)}


X = lambda q, b: inp("x", q, b)  # noqa: E731
Y = lambda q, b: inp("y", q, b)  # noqa: E731
Z = lambda q, b: inp("z", q, b, "dimensionless", U_R)  # noqa: E731
F = lambda q, b: inp("f", q, b, "force", "invented_force_unit")  # noqa: E731

CASES = []


def case(case_id, description, formula, inputs, expected, notes=(), negative_control=False):
    CASES.append({"case_id": case_id, "description": description,
                  "negative_control": negative_control, "formula": formula,
                  "inputs": inputs, "expected": expected,
                  "notes": [list(n) for n in notes]})


def q(enclosure, dimension="stress", unit=U_S):
    return {"kind": "quantity", "dimension": dimension, "unit_ref": unit,
            "enclosure": None if enclosure is None else [bits(enclosure[0]), bits(enclosure[1])]}


def t(truth):
    return {"kind": "truth", "truth": truth}


def blocked(*findings):
    return {"kind": "blocked", "findings": [list(f) for f in findings]}


# -- points, bounds and outward arithmetic ------------------------------------
case("literal_point", "A literal is an exact point.", lit(2.5), [], q(pt(2.5)))
case("variable_zero_bound_point", "b = 0 binds the exact point q.", var("x"), [X(1.5, 0.0)], q(pt(1.5)))
case("variable_interval_bound", "b > 0 binds [next_down(q-b), next_up(q+b)].",
     var("x"), [X(1.5, 0.5)], q(bound(1.5, 0.5)))
case("variable_subnormal_bound", "A subnormal b needs no special case.",
     var("x"), [X(0.0, 5e-324)], q(bound(0.0, 5e-324)))
case("add_points_steps_outward", "Every floating operation steps outward, even on points.",
     bi("add", lit(1.0), lit(2.0)), [], q(add(pt(1.0), pt(2.0))))
xe, ye = bound(1.5, 0.5), bound(10.0, 1.0)
case("add_intervals", "Endpoint sums, outward.", bi("add", var("x"), var("y")),
     [X(1.5, 0.5), Y(10.0, 1.0)], q(add(xe, ye)))
case("subtract_intervals", "Crossed endpoint differences, outward.", bi("subtract", var("x"), var("y")),
     [X(1.5, 0.5), Y(10.0, 1.0)], q(sub(xe, ye)))
case("multiply_mixed_signs", "Min and max of the four endpoint products.",
     bi("multiply", var("z"), var("x")), [Z(-1.0, 2.0), X(2.0, 1.0)],
     q(mul(bound(-1.0, 2.0), bound(2.0, 1.0))))
case("multiply_derived_product_unit", "stress x area is force, units joined in byte order.",
     bi("multiply", var("x"), lit(2.0, "area", "invented_area_unit")), [X(3.0, 0.25)],
     q(mul(bound(3.0, 0.25), pt(2.0)), "force", "invented_area_unit*invented_stress_unit"))
case("divide_positive_divisor", "Min and max of the four endpoint quotients.",
     bi("divide", var("x"), var("z")), [X(10.0, 1.0), Z(2.0, 0.5)],
     q(div(bound(10.0, 1.0), bound(2.0, 0.5))))
case("divide_negative_divisor", "A divisor range wholly below zero.",
     bi("divide", var("x"), var("z")), [X(10.0, 1.0), Z(-2.0, 0.5)],
     q(div(bound(10.0, 1.0), bound(-2.0, 0.5))))
case("divide_same_dimension_ratio", "stress / stress is a ratio.",
     bi("divide", var("x"), var("y")), [X(10.0, 1.0), Y(4.0, 0.5)],
     q(div(bound(10.0, 1.0), bound(4.0, 0.5)), "dimensionless", "ratio"))
case("divide_derived_quotient_unit", "moment / force is length, unit numerator/denominator.",
     bi("divide", lit(12.0, "moment", "invented_moment_unit"), var("f")), [F(4.0, 0.5)],
     q(div(pt(12.0), bound(4.0, 0.5)), "length", "invented_moment_unit/invented_force_unit"))
case("negate_exact", "Negation is exact: [-hi, -lo].", un("negate", var("x")), [X(1.5, 0.5)],
     q((-xe[1], -xe[0])))
pe, ne, se = bound(5.0, 1.0), bound(-5.0, 1.0), bound(-1.0, 2.0)
case("abs_positive", "abs of a non-negative range is the range.", un("abs", var("x")), [X(5.0, 1.0)], q(pe))
case("abs_negative", "abs of a non-positive range is [-hi, -lo].", un("abs", var("x")), [X(-5.0, 1.0)],
     q((-ne[1], -ne[0])))
case("abs_straddle", "abs of a straddling range is [0, max(-lo, hi)].", un("abs", var("x")),
     [X(-1.0, 2.0)], q((0.0, max(-se[0], se[1]))))
ae, be = bound(10.0, 1.0), bound(5.0, 10.0)
case("min_aggregate", "Endpoint-wise min (exact).", agg("min", var("x"), var("y")),
     [X(10.0, 1.0), Y(5.0, 10.0)], q((min(ae[0], be[0]), min(ae[1], be[1]))))
case("max_aggregate", "Endpoint-wise max (exact).", agg("max", var("x"), var("y")),
     [X(10.0, 1.0), Y(5.0, 10.0)], q((max(ae[0], be[0]), max(ae[1], be[1]))))
case("min_single_operand", "A one-operand min is its operand.", agg("min", var("x")), [X(10.0, 1.0)], q(ae))
case("rounding_hides_real_excess", "fl(1 + 2^-53) = 1, but the exact sum exceeds 1: never T.",
     cmp("less_than_or_equal", bi("add", var("x"), var("y")), lit(1.0)),
     [X(1.0, 0.0), Y(2.0 ** -53, 0.0)], t("U"), negative_control=True)

# -- comparisons over x in about [9, 11] --------------------------------------
X10 = X(10.0, 1.0)
for cid, op, limit, truth, nc in [
    ("le_true", "less_than_or_equal", 12.0, "T", False),
    ("le_false", "less_than_or_equal", 8.0, "F", False),
    ("le_straddle", "less_than_or_equal", 10.0, "U", True),
    ("lt_true", "less_than", 12.0, "T", False),
    ("lt_false", "less_than", 8.0, "F", False),
    ("ge_straddle", "greater_than_or_equal", 10.0, "U", True),
    ("ge_false", "greater_than_or_equal", 12.0, "F", False),
    ("gt_true", "greater_than", 8.0, "T", False),
    ("gt_false", "greater_than", 12.0, "F", False),
    ("gt_straddle", "greater_than", 10.0, "U", True),
    ("equal_overlap", "equal", 10.0, "U", True),
    ("equal_disjoint", "equal", 20.0, "F", False),
    ("not_equal_overlap", "not_equal", 10.0, "U", True),
    ("not_equal_disjoint", "not_equal", 20.0, "T", False),
]:
    case(cid, f"x {op} {limit} with x = 10 +/- 1.", cmp(op, var("x"), lit(limit)), [X10], t(truth),
         negative_control=nc)
hi_end, lo_end = ae[1], ae[0]
case("lt_shared_end", "x < x.hi: the shared end is not strictly below.", cmp("less_than", var("x"), var("y")),
     [X10, Y(hi_end, 0.0)], t("U"), negative_control=True)
case("le_shared_end", "x <= x.hi holds at the shared end.", cmp("less_than_or_equal", var("x"), var("y")),
     [X10, Y(hi_end, 0.0)], t("T"))
case("gt_shared_end", "x > x.lo: the shared end is not strictly above.", cmp("greater_than", var("x"), var("y")),
     [X10, Y(lo_end, 0.0)], t("U"), negative_control=True)
case("ge_shared_end", "x >= x.lo holds at the shared end.", cmp("greater_than_or_equal", var("x"), var("y")),
     [X10, Y(lo_end, 0.0)], t("T"))
case("equal_points", "= is T for two equal points.", cmp("equal", var("x"), lit(10.0)), [X(10.0, 0.0)], t("T"))
case("not_equal_points", "!= is F for two equal points.", cmp("not_equal", var("x"), lit(10.0)),
     [X(10.0, 0.0)], t("F"))

# -- Kleene logic -------------------------------------------------------------
yes, no, unk = (cmp("less_than", var("x"), lit(12.0)), cmp("greater_than", var("x"), lit(12.0)),
                cmp("less_than_or_equal", var("x"), lit(10.0)))
case("not_straddle", "not(x > c) with x straddling c.", un("not", cmp("greater_than", var("x"), lit(10.0))),
     [X10], t("U"), negative_control=True)
case("not_true", "not T is F.", un("not", yes), [X10], t("F"))
case("and_false_dominates", "F and U is F.", lg("and", no, unk), [X10], t("F"))
case("and_true_unknown", "T and U is U.", lg("and", yes, unk), [X10], t("U"), negative_control=True)
case("or_true_dominates", "T or U is T (U from a straddling comparison).", lg("or", yes, unk), [X10], t("T"))
case("or_false_unknown", "F or U is U.", lg("or", no, unk), [X10], t("U"), negative_control=True)

# -- select -------------------------------------------------------------------
case("select_true_takes_then", "Condition T: the then-branch.", sel(yes, lit(1.0), lit(2.0)), [X10], q(pt(1.0)))
case("select_false_takes_else", "Condition F: the else-branch.", sel(no, lit(1.0), lit(2.0)), [X10], q(pt(2.0)))
case("select_unknown_hull", "Condition U: the hull of both quantity branches.", sel(unk, lit(1.0), lit(2.0)),
     [X10], q((1.0, 2.0)))
case("select_unknown_straddle", "A select with a U condition whose branches straddle the limit.",
     cmp("less_than_or_equal", sel(unk, lit(1.0), lit(20.0)), lit(5.0)), [X10], t("U"), negative_control=True)
case("select_unknown_booleans_agree", "Condition U, both boolean branches T: T.",
     sel(unk, yes, cmp("greater_than", var("x"), lit(8.0))), [X10], t("T"))
case("select_unknown_booleans_disagree", "Condition U, branches T and F: U.", sel(unk, yes, no), [X10], t("U"),
     negative_control=True)

# -- D2 §4.11.5 negative controls ----------------------------------------------
case("abs_straddle_ge", "abs(x) >= c with x straddling 0.",
     cmp("greater_than_or_equal", un("abs", var("x")), lit(0.5)), [X(0.0, 1.0)], t("U"), negative_control=True)
case("square_le", "x*x <= c (ratio z*z) with the square straddling c.",
     cmp("less_than_or_equal", bi("multiply", var("z"), var("z")), rlit(1.0)), [Z(0.0, 1.5)], t("U"),
     negative_control=True)
case("square_dependency", "The dependency effect widens z*z to [-2.25, 2.25]: U, never a false T or F.",
     cmp("greater_than_or_equal", bi("multiply", var("z"), var("z")), rlit(-0.5)), [Z(0.0, 1.5)], t("U"))
case("divide_by_zero_range", "Division by a range containing 0.",
     cmp("less_than_or_equal", bi("divide", lit(1.0), var("z")), lit(1e9)), [Z(0.0, 1.0)], t("U"),
     notes=[("divide_by_zero_range", "divide")], negative_control=True)
case("divide_by_zero_point", "Division by the exact point 0 also reads U (D2 5b.3; non-pass either way).",
     cmp("less_than_or_equal", bi("divide", var("x"), rlit(0.0)), lit(1.0)), [X10], t("U"),
     notes=[("divide_by_zero_range", "divide")], negative_control=True)
case("two_inputs_interior_extremum", "Two interval inputs, the extremum interior.",
     cmp("less_than_or_equal", bi("subtract", var("x"), var("y")), lit(5.0)), [X10, Y(5.0, 10.0)], t("U"),
     negative_control=True)
T1 = table("invented_tent_table", [(0.0, 0.0), (1.0, 10.0), (2.0, 0.0)])
case("interpolate_partly_out_of_range", "Interpolation partly out of range: U (part of the range would block).",
     cmp("less_than_or_equal", interp(T1, var("z")), lit(100.0)), [Z(1.9, 0.5)], t("U"),
     notes=[("table_argument_range", "invented_tent_table")], negative_control=True)

# -- interpolation and lookups --------------------------------------------------
case("interpolate_point_at_row", "A point at a row argument is the row's exact result.",
     interp(T1, var("z")), [Z(1.0, 0.0)], q(pt(10.0)))
case("interpolate_point_inside_segment", "A point inside a segment: the formula's chain, each step outward.",
     interp(T1, var("z")), [Z(0.25, 0.0)], q(segment(0.0, 10.0, 0.0, 1.0, pt(0.25))))
ze = bound(1.0, 0.25)
case("interpolate_range_spanning_row", "A range spanning the peak row: the two segments joined.",
     interp(T1, var("z")), [Z(1.0, 0.25)],
     q(hull(segment(0.0, 10.0, 0.0, 1.0, (ze[0], 1.0)), segment(10.0, 0.0, 1.0, 2.0, (1.0, ze[1])))))
T2 = table("invented_steep_table", [(-1e10, 1e20), (1.0, 8000.0), (2.0, 8000.0)])
ce = bound(1.2, 0.7)
steep = hull(segment(1e20, 8000.0, -1e10, 1.0, (ce[0], 1.0)), segment(8000.0, 8000.0, 1.0, 2.0, (1.0, ce[1])))
assert steep[0] <= 0.0 and steep[1] >= 5000007999.5  # the point path's 0.0 and the exact value at 0.5
case("interpolate_rounding_near_row", "The point path rounds to 0.0 just left of the row at 1; the enclosure covers it.",
     interp(T2, var("z")), [Z(1.2, 0.7)], q(steep))
case("interpolate_rounding_near_row_not_true", "D2 5b.3's counterexample: the old hull rule read T here.",
     cmp("greater_than_or_equal", interp(T2, var("z")), lit(4000.0)), [Z(1.2, 0.7)], t("U"),
     negative_control=True)
T3 = table("invented_step_table", [(1.0, 5.0), (2.0, 7.0), (3.0, 6.0)])
case("lookup_step_spanning_rows", "The hull of every row the range spans.", lookup(T3, "step", var("z")),
     [Z(2.0, 0.6)], q((5.0, 7.0)))
case("lookup_step_single_row", "A range inside one step is that row's exact result.",
     lookup(T3, "step", var("z")), [Z(2.5, 0.25)], q(pt(7.0)))
case("lookup_step_out_of_range", "A step lookup partly out of range: U.", lookup(T3, "step", var("z")),
     [Z(2.9, 0.2)], q(None), notes=[("table_argument_range", "invented_step_table")])
case("lookup_exact_point_hit", "An exact lookup of a point follows the point path.",
     lookup(T3, "exact", var("z")), [Z(2.0, 0.0)], q(pt(7.0)))
case("lookup_exact_range", "An exact lookup over a range: U.", lookup(T3, "exact", var("z")), [Z(2.0, 0.5)],
     q(None), notes=[("exact_lookup_range", "invented_step_table")])
case("lookup_exact_point_miss", "A point miss blocks exactly as the point path does.",
     lookup(T3, "exact", var("z")), [Z(2.5, 0.0)], blocked(("TableKeyNotFound", "invented_step_table")))

# -- eager U ------------------------------------------------------------------
case("eager_u_untaken_branch", "A possible block in an untaken branch still makes the check U.",
     cmp("less_than_or_equal", sel(cmp("greater_than", lit(2.0), lit(1.0)), lit(0.0), bi("divide", var("x"), var("z"))),
         lit(1.0)), [X(10.0, 0.0), Z(0.0, 1.0)], t("U"), notes=[("divide_by_zero_range", "divide")],
     negative_control=True)
case("eager_u_beside_true_or", "A possible block beside a true or-operand still makes the check U.",
     lg("or", cmp("greater_than", lit(2.0), lit(1.0)),
        cmp("greater_than", bi("divide", var("x"), var("z")), lit(0.0))),
     [X(10.0, 0.0), Z(0.0, 1.0)], t("U"), notes=[("divide_by_zero_range", "divide")], negative_control=True)
case("overflow_non_finite", "An overflowed end: no finite enclosure, U.",
     cmp("less_than_or_equal", bi("multiply", var("z"), var("x")), lit(1.0)), [Z(2.0, 1.0), X(1e308, 0.0)], t("U"),
     notes=[("non_finite_enclosure", "multiply")], negative_control=True)
case("input_without_finite_enclosure", "q + b overflows: the input has no finite enclosure, U.",
     cmp("less_than_or_equal", var("x"), lit(1.0)), [X(sys.float_info.max, sys.float_info.max)], t("U"),
     notes=[("non_finite_enclosure", "x")])

# -- structural blocks (the point path's findings) --------------------------------
case("blocked_dimension_mismatch", "Adding a temperature to a stress.",
     bi("add", var("x"), lit(1.0, "temperature", "invented_temperature_unit")), [X10],
     blocked(("DimensionMismatch", "add_subtract")))
case("blocked_unit_mismatch", "Adding stresses in different units.",
     bi("add", var("x"), lit(1.0, "stress", "other_stress_unit")), [X10], blocked(("UnitMismatch", "add_subtract")))
case("blocked_type_mismatch", "Comparing a boolean with a quantity.",
     cmp("equal", cmp("less_than", var("x"), lit(1.0)), lit(1.0)), [X10], blocked(("TypeMismatch", "comparison")))
case("blocked_ambiguous_quotient", "force / area is pressure or stress: ambiguous.",
     bi("divide", lit(10.0, "force", "invented_force_unit"), lit(2.0, "area", "invented_area_unit")), [],
     blocked(("UnsupportedExpressionForm", "divide")))
case("blocked_unsupported_form", "An unsupported form.", {"node": "unsupported_form", "form_id": "power"}, [],
     blocked(("UnsupportedExpressionForm", "power")))
case("blocked_unsafe_host_access", "Host access.", {"node": "unsafe_host_access", "request": "filesystem"}, [],
     blocked(("UnsafeConstruct", "filesystem")))
case("blocked_missing_variable", "An unbound variable.", var("w"), [X10], blocked(("MissingVariable", "w")))
case("blocked_table_malformed", "Interpolation needs at least two rows.",
     interp(table("invented_short_table", [(0.0, 1.0)]), var("z")), [Z(0.5, 0.0)],
     blocked(("TableMalformed", "invented_short_table")))

# -- repair round 01 (RV99 S-1, S-2, S-3) ------------------------------------------
NAN = float("nan")


def with_enclosure(item, enclosure):
    """An explicit input enclosure (bit patterns), or None for an input with no
    finite enclosure; it replaces the bound."""
    out = dict(item)
    out["enclosure_bits"] = None if enclosure is None else [bits(enclosure[0]), bits(enclosure[1])]
    return out


TINY = 2.0 ** -40
case("equal_identical_ranges", "x = y with identical non-point ranges: x != y inside the box, so U (RV99 S-1).",
     cmp("equal", var("x"), var("y")), [X(1.0, TINY), Y(1.0, TINY)], t("U"), negative_control=True)
case("not_equal_identical_ranges", "x != y with identical non-point ranges: x = y inside the box, so U (RV99 S-1).",
     cmp("not_equal", var("x"), var("y")), [X(1.0, TINY), Y(1.0, TINY)], t("U"), negative_control=True)
case("divide_by_zero_end_range", "0 / (-abs(z)) <= 1: the divisor [-1, -0] ends at zero, where the point path blocks (RV99 S-2).",
     cmp("less_than_or_equal", bi("divide", lit(0.0), un("negate", un("abs", var("z")))), lit(1.0)),
     [Z(0.0, 1.0)], t("U"), notes=[("divide_by_zero_range", "divide")], negative_control=True)
for cid, b in [("invalid_bound_nan", NAN), ("invalid_bound_negative", -1.0), ("invalid_bound_infinite", math.inf)]:
    case(cid, f"A bound of {b!r} is not finite and non-negative: enclosure_from_bound gives no enclosure, so U (RV99 S-3).",
         cmp("less_than_or_equal", var("x"), lit(10.0)), [X(9.5, b)], t("U"), notes=[("non_finite_enclosure", "x")])
case("enclosure_explicit_valid", "An explicit enclosure [9, 9.75] binds as given.",
     cmp("less_than_or_equal", var("x"), lit(10.0)), [with_enclosure(X(9.5, 0.0), (9.0, 9.75))], t("T"))
case("enclosure_explicit_none", "An explicit input with no finite enclosure: U.",
     cmp("less_than_or_equal", var("x"), lit(10.0)), [with_enclosure(X(9.5, 0.0), None)], t("U"),
     notes=[("non_finite_enclosure", "x")])
case("enclosure_inverted", "An inverted enclosure (11, 9) is refused as Rust refuses it (RV99 S-3).",
     cmp("less_than_or_equal", var("x"), lit(10.0)), [with_enclosure(X(9.5, 0.0), (11.0, 9.0))],
     blocked(("InvalidReference", "x")))
case("enclosure_nan_end", "An enclosure with a NaN end is refused (RV99 S-3).",
     cmp("less_than_or_equal", var("x"), lit(10.0)), [with_enclosure(X(9.5, 0.0), (NAN, 1.0))],
     blocked(("NonFiniteInput", "x")))
case("enclosure_infinite_end", "An enclosure with an infinite end is refused (RV99 S-3).",
     cmp("less_than_or_equal", var("x"), lit(10.0)), [with_enclosure(X(9.5, 0.0), (-math.inf, 0.0))],
     blocked(("NonFiniteInput", "x")))

DOCUMENT = {
    "document_kind": "openpipestress.rule_interval_cases",
    "case_file_version": 1,
    "basis": "T3 D2 revision 5b.3 §4.11.2-§4.11.5 (option C, conservative interval binding)",
    "notice": ("Invented values only; no protected standards content and no professional, "
               "certification or code-compliance claim. Inputs and expected enclosures are binary64 "
               "bit patterns; formula literals are short decimals that every JSON reader parses exactly. "
               "An input binds [next_down(q-b), next_up(q+b)] when b > 0 and the exact point q when b = 0; any other b "
               "(negative, NaN, infinite) gives no finite enclosure (enclosure_from_bound returns none). An "
               "optional enclosure_bits pair (or null) is an explicit input enclosure that replaces the bound. "
               "Truth values: T (every value passes), F (every value fails), U (indeterminate, never a pass). "
               "A negative control must never read T."),
    "consumers": ["core/rules/rule_check_runner/tests/rule_interval_cases.rs",
                  "tests/test_rule_interval.py"],
    "cases": CASES,
}

if __name__ == "__main__":
    ids = [c["case_id"] for c in CASES]
    assert len(ids) == len(set(ids))
    with open(sys.argv[1], "w", encoding="utf-8") as handle:
        json.dump(DOCUMENT, handle, indent=1, ensure_ascii=False)
        handle.write("\n")
    print(len(CASES), "cases")
