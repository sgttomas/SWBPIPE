"""RV99 case generator for S-I1 (independent of I73's generator).

Writes rv99_cases.json: evaluator cases (formula, inputs with bounds,
sample tuples inside the input enclosures) and runner cases (a pack with one
check, solver/user values, bounds, samples of the bounded solver value).
All numbers in formulas and inputs are carried as binary64 hex bit strings so
every reader gets exact values. Invented values only.
"""
from __future__ import annotations

import json
import math
import random
import struct
import sys

sys.path.insert(0, __import__("os").path.dirname(__file__))
from rv99_oracle import Undef, my_enclosure, nd, nu, point_eval  # noqa: E402

R = random.Random(int(__import__("os").environ.get("RV99_SEED", "990099")))
U = "ratio"
DIM = "dimensionless"


def hx(x: float) -> str:
    return "0x" + struct.pack(">d", x).hex()


def lit(v, dim=DIM, unit=U):
    if not math.isfinite(v):
        v = 1.0
    return {"node": "literal", "quantity": {"value": v, "dimension": dim, "unit_ref": unit}}


def var(i):
    return {"node": "variable_ref", "variable_id": i}


def cmp_(op, a, b):
    return {"node": "compare", "operator": op, "left": a, "right": b}


def bin_(op, a, b):
    return {"node": "binary", "operator": op, "left": a, "right": b}


def un(op, a):
    return {"node": "unary", "operator": op, "operand": a}


def logic(op, a, b):
    return {"node": "logical", "operator": op, "left": a, "right": b}


def sel(c, t, e):
    return {"node": "select", "condition": c, "then": t, "else": e}


def agg(f, ops):
    return {"node": "aggregate", "function": f, "operands": ops}


def table(rows, tid="t1", adim=DIM, aunit=U, rdim=DIM, runit=U):
    return {"table_id": tid, "argument_dimension": adim, "argument_unit_ref": aunit,
            "result_dimension": rdim, "result_unit_ref": runit,
            "rows": [{"argument": a, "result": r} for a, r in rows]}


def interp(t, a):
    return {"node": "interpolate", "table": t, "argument": a}


def lookup(mode, t, a):
    return {"node": "lookup", "mode": mode, "table": t, "argument": a}


OPS = ["less_than", "less_than_or_equal", "greater_than", "greater_than_or_equal",
       "equal", "not_equal"]
ORDER_OPS = OPS[:4]


def to_hex_tree(node):
    """Copy of a formula with literal values and table numbers as hex strings."""
    if isinstance(node, dict):
        out = {}
        for k, v in node.items():
            if k in ("value", "argument", "result") and isinstance(v, float):
                out[k] = hx(v)
            else:
                out[k] = to_hex_tree(v)
        return out
    if isinstance(node, list):
        return [to_hex_tree(v) for v in node]
    return node


# ------------------------------------------------------------------ samples
def okey(x):
    b = struct.unpack(">q", struct.pack(">d", x))[0]
    return b if b >= 0 else -(b & 0x7FFFFFFFFFFFFFFF)


def ofromkey(k):
    if k >= 0:
        return struct.unpack(">d", struct.pack(">q", k))[0]
    return struct.unpack(">d", struct.pack(">q", (-k) | -0x8000000000000000))[0]


def samples_for(enc, extra=(), n_random=12, enumerate_max=48):
    lo, hi = enc
    klo, khi = okey(lo), okey(hi)
    if khi - klo + 1 <= enumerate_max:
        return [ofromkey(k) for k in range(klo, khi + 1)]
    s = {lo, hi, nu(lo), nd(hi), (lo + hi) / 2}
    for e in extra:
        if lo <= e <= hi:
            s.add(e)
            for f in (nd(e), nu(e)):
                if lo <= f <= hi:
                    s.add(f)
    if lo <= 0.0 <= hi:
        s.update([0.0, 5e-324, -5e-324])
    for _ in range(n_random):
        s.add(ofromkey(R.randint(klo, khi)))
        s.add(lo + (hi - lo) * R.random())
    return sorted(x for x in s if lo <= x <= hi and math.isfinite(x))


def product(lists, cap=300):
    tuples = [[]]
    for lst in lists:
        tuples = [t + [x] for t in tuples for x in lst]
        if len(tuples) > 20 * cap:
            tuples = R.sample(tuples, 20 * cap)
    if len(tuples) > cap:
        corners = [t for t in tuples if all(t[i] in (lists[i][0], lists[i][-1]) for i in range(len(lists)))]
        rest = [t for t in tuples if t not in corners]
        tuples = corners + R.sample(rest, cap - len(corners))
    return tuples


# ------------------------------------------------------------------ values
def rand_mag():
    r = R.random()
    if r < 0.6:
        return 10 ** R.uniform(-3, 6)
    if r < 0.8:
        return 10 ** R.uniform(-15, 15)
    if r < 0.95:
        return 10 ** R.uniform(-300, 300)
    return R.choice([5e-324, 2.2250738585072014e-308, 1.7976931348623157e308, 1e154, 1e-154])


def rand_val():
    v = rand_mag()
    if R.random() < 0.35:
        v = -v
    if R.random() < 0.05:
        v = 0.0
    return v


def rand_bound(q):
    r = R.random()
    ulp = math.ulp(q) if q != 0 else 5e-324
    if r < 0.12:
        return 0.0
    if r < 0.30:
        return ulp * R.choice([0.25, 0.5, 1, 2, 3, 10])
    if r < 0.50:
        return abs(q) * R.choice([1e-16, 1e-15, 1e-12, 1e-9]) or 5e-324
    if r < 0.75:
        return abs(q) * R.choice([1e-6, 1e-3, 1e-2, 0.1]) or 1e-300
    if r < 0.85:
        return R.choice([5e-324, 1e-320, 2.2250738585072014e-308])
    if r < 0.95:
        return abs(q) * R.choice([0.5, 1.0, 2.0, 10.0]) or 1.0
    return R.choice([1e300, 1.7976931348623157e308, 1e-10])


def rand_table(n=None):
    n = n or R.randint(2, 5)
    a = sorted({rand_val() for _ in range(n)})
    while len(a) < 2:
        a = sorted({rand_val() for _ in range(n)})
    if R.random() < 0.3:
        base = R.uniform(-10, 10)
        a = sorted({base + i * R.choice([1.0, 0.5, 1e-12, 2.0 ** -40]) for i in range(n)})
    rows = []
    for x in a:
        r = rand_val()
        if R.random() < 0.3:
            r = R.choice([1e20, -1e20, 8000.0, 1.0, 0.0, 1e300, -1e300])
        rows.append((x, r))
    return table(rows, tid=R.choice(["t1", "tab_a"]))


# ------------------------------------------------------------------ random trees
VARS = ["x", "y", "z"]


def rand_q(depth):
    if depth <= 0 or R.random() < 0.25:
        return var(R.choice(VARS)) if R.random() < 0.7 else lit(rand_val())
    c = R.random()
    if c < 0.08:
        return un("negate", rand_q(depth - 1))
    if c < 0.16:
        return un("abs", rand_q(depth - 1))
    if c < 0.30:
        return bin_("add", rand_q(depth - 1), rand_q(depth - 1))
    if c < 0.42:
        return bin_("subtract", rand_q(depth - 1), rand_q(depth - 1))
    if c < 0.56:
        return bin_("multiply", rand_q(depth - 1), rand_q(depth - 1))
    if c < 0.68:
        return bin_("divide", rand_q(depth - 1), rand_q(depth - 1))
    if c < 0.74:
        return agg(R.choice(["min", "max"]), [rand_q(depth - 1) for _ in range(R.randint(1, 3))])
    if c < 0.80:
        return sel(rand_b(depth - 1), rand_q(depth - 1), rand_q(depth - 1))
    if c < 0.90:
        return interp(rand_table(), rand_q(depth - 1))
    if c < 0.96:
        return lookup("step", rand_table(), rand_q(depth - 1))
    return lookup("exact", rand_table(), rand_q(depth - 1))


def rand_b(depth):
    c = R.random()
    if depth <= 0 or c < 0.5:
        return cmp_(R.choice(OPS), rand_q(depth - 1), rand_q(depth - 1))
    if c < 0.65:
        return un("not", rand_b(depth - 1))
    if c < 0.9:
        return logic(R.choice(["and", "or"]), rand_b(depth - 1), rand_b(depth - 1))
    return sel(rand_b(depth - 1), rand_b(depth - 1), rand_b(depth - 1))


def used_vars(node, out):
    if isinstance(node, dict):
        if node.get("node") == "variable_ref":
            out.add(node["variable_id"])
        for v in node.values():
            used_vars(v, out)
    elif isinstance(node, list):
        for v in node:
            used_vars(v, out)
    return out


def table_args(node, out):
    if isinstance(node, dict):
        if "rows" in node and isinstance(node["rows"], list):
            for r in node["rows"]:
                out.append(r["argument"])
        for v in node.values():
            table_args(v, out)
    elif isinstance(node, list):
        for v in node:
            table_args(v, out)
    return out


def near(v):
    """A limit near v: the value, a neighbour, or a relative perturbation."""
    c = R.random()
    if not math.isfinite(v):
        return rand_val()
    if c < 0.3:
        return v
    if c < 0.5:
        return R.choice([nd(v), nu(v), nd(nd(v)), nu(nu(v))])
    if c < 0.8:
        return v * (1 + R.choice([-1, 1]) * R.choice([1e-16, 1e-14, 1e-10, 1e-6, 1e-3]))
    return rand_val()


def make_eval_case(cid, formula, inputs, extra_points=()):
    """inputs: list of (id, q, b)."""
    encs = []
    for i, q, b in inputs:
        e = my_enclosure(q, b)
        encs.append(e if e is not None else (q, q))
    targs = table_args(formula, [])
    lists = [samples_for(e, extra=list(extra_points) + targs + [q]) for e, (_, q, _) in zip(encs, inputs)]
    tuples = product(lists) if lists else [[]]
    return {
        "id": cid,
        "formula": to_hex_tree(formula),
        "inputs": [{"id": i, "value": hx(q), "bound": hx(b), "dimension": DIM, "unit_ref": U}
                   for i, q, b in inputs],
        "samples": [[hx(x) for x in t] for t in tuples],
        "bisect": len(inputs) == 1,
    }


def gen_random_eval(n):
    cases = []
    while len(cases) < n:
        depth = R.randint(1, 4)
        left = rand_q(depth)
        vs = sorted(used_vars(left, set()))
        if not vs:
            continue
        qs = {v: rand_val() for v in vs}
        if R.random() < 0.3:
            # put an argument on a table row
            ta = table_args(left, [])
            if ta:
                qs[vs[0]] = R.choice(ta)
        try:
            pv = point_eval(left, qs)[1]
        except (Undef, KeyError):
            pv = rand_val()
        if R.random() < 0.15:
            formula = left  # a quantity formula
        else:
            formula = cmp_(R.choice(OPS), left, lit(near(pv) if isinstance(pv, float) else rand_val()))
            if R.random() < 0.2:
                formula = logic(R.choice(["and", "or"]), formula, rand_b(1))
            if R.random() < 0.1:
                formula = un("not", formula)
        inputs = [(v, qs[v], rand_bound(qs[v])) for v in vs]
        if all(b == 0.0 for _, _, b in inputs):
            inputs[0] = (inputs[0][0], inputs[0][1], rand_bound(inputs[0][1]) or 1e-9)
        cases.append(make_eval_case(f"rand_{len(cases):05d}", formula, inputs))
    return cases


def gen_straddle_eval():
    """RV99's own boundary straddles at each comparison (single and pair)."""
    cases = []
    for c in [100.0, 1.0, -3.5, 0.0, 1e-300, 1e300, 5e-324]:
        ulp = math.ulp(c) if c != 0 else 5e-324
        configs = {
            "below": (c - 1000 * ulp if c != 0 else -1e-320, 10 * ulp),
            "hi_touch": (nd(c), ulp / 8),          # enclosure hi == c
            "straddle": (c, ulp * 4),
            "lo_touch": (nu(c), ulp / 8),          # enclosure lo == c
            "above": (c + 1000 * ulp if c != 0 else 1e-320, 10 * ulp),
            "point": (c, 0.0),
            "wide": (c, abs(c) * 0.5 + 1.0),
        }
        for name, (q, b) in configs.items():
            if not math.isfinite(q):
                continue
            for op in OPS:
                cases.append(make_eval_case(f"str_{op}_{name}_{c!r}", cmp_(op, var("x"), lit(c)),
                                            [("x", q, b)], extra_points=[c]))
                cases.append(make_eval_case(f"strr_{op}_{name}_{c!r}", cmp_(op, lit(c), var("x")),
                                            [("x", q, b)], extra_points=[c]))
    # two interval inputs sharing an end
    for op in OPS:
        for (qx, bx, qy, by, tag) in [
            (1.0, 2.0 ** -60, nu(nu(1.0)), 2.0 ** -60, "shared_end"),
            (1.0, 2.0 ** -40, 1.0, 2.0 ** -40, "same_box"),
            (1.0, 2.0 ** -60, 2.0, 2.0 ** -60, "disjoint"),
            (5.0, 1.0, 6.0, 1.0, "overlap"),
        ]:
            cases.append(make_eval_case(f"pair_{op}_{tag}", cmp_(op, var("x"), var("y")),
                                        [("x", qx, bx), ("y", qy, by)]))
    # D2 §4.11.5 negative controls, RV99's own instances
    t = table([(0.0, 0.0), (1.0, 10.0), (2.0, 15.0)])
    negs = [
        ("neg_abs", cmp_("greater_than_or_equal", un("abs", var("x")), lit(1e-3)), [("x", 0.0, 1e-3)]),
        ("neg_square", cmp_("less_than_or_equal", bin_("multiply", var("x"), var("x")), lit(4.0)), [("x", 2.0, 1e-9)]),
        ("neg_square2", cmp_("less_than_or_equal", bin_("multiply", var("x"), var("x")), lit(4.0)), [("x", 0.0, 2.0)]),
        ("neg_div0", cmp_("less_than", bin_("divide", lit(1.0), var("x")), lit(1e300)), [("x", 1e-3, 1e-3)]),
        ("neg_not", un("not", cmp_("greater_than", var("x"), lit(5.0))), [("x", 5.0, 1e-12)]),
        ("neg_two_inputs", cmp_("less_than_or_equal", bin_("multiply", var("x"), var("y")), lit(0.0)),
         [("x", 0.0, 1.0), ("y", 0.0, 1.0)]),
        ("neg_eq", cmp_("equal", var("x"), var("y")), [("x", 1.0, 0.5), ("y", 1.2, 0.5)]),
        ("neg_ne", cmp_("not_equal", var("x"), var("y")), [("x", 1.0, 0.5), ("y", 1.2, 0.5)]),
        ("neg_select", cmp_("less_than_or_equal",
                            sel(cmp_("less_than", var("x"), lit(1.0)), lit(0.5), lit(2.0)), lit(1.0)),
         [("x", 1.0, 1e-9)]),
        ("neg_interp_out", cmp_("less_than_or_equal", interp(t, var("x")), lit(100.0)), [("x", 2.0, 1e-6)]),
        ("neg_interp_row", cmp_("less_than_or_equal", interp(t, var("x")), lit(10.0)), [("x", 1.0, 1e-9)]),
        ("neg_step_row", cmp_("less_than_or_equal", lookup("step", t, var("x")), lit(10.0)), [("x", 2.0, 1e-9)]),
        ("neg_exact_range", cmp_("less_than_or_equal", lookup("exact", t, var("x")), lit(100.0)), [("x", 1.0, 1e-9)]),
        ("neg_eager_select", sel(cmp_("less_than", lit(0.0), lit(1.0)), cmp_("less_than", var("x"), lit(10.0)),
                                 cmp_("less_than", bin_("divide", lit(1.0), var("x")), lit(1.0))), [("x", 0.0, 1.0)]),
        ("neg_eager_or", logic("or", cmp_("less_than", lit(0.0), lit(1.0)),
                               cmp_("less_than", interp(t, var("x")), lit(1.0))), [("x", 1.9, 0.5)]),
        ("neg_x_minus_x", cmp_("equal", bin_("subtract", var("x"), var("x")), lit(0.0)), [("x", 3.0, 1e-9)]),
        ("neg_cancel", cmp_("less_than_or_equal", bin_("subtract", bin_("add", var("x"), lit(1e16)), lit(1e16)),
                            lit(0.0)), [("x", 0.5, 0.25)]),
        ("neg_min_max", cmp_("less_than", agg("max", [var("x"), var("y")]), agg("min", [var("x"), var("y")])),
         [("x", 1.0, 1e-9), ("y", 1.0, 1e-9)]),
    ]
    for cid, f, ins in negs:
        cases.append(make_eval_case(cid, f, ins, extra_points=[0.0, 1.0, 2.0, 5.0]))
    # I73's interpolation counterexample family, perturbed (RV99's own)
    for k in range(40):
        big = R.choice([1e20, 1e18, 3e19, -1e20, 7e22])
        small = R.choice([8000.0, 1.0, 0.0, -5.0])
        a0 = -R.choice([1e10, 3e9, 1e12])
        rows = [(a0, big), (1.0, small), (2.0, small)]
        tt = table(rows)
        q = R.choice([0.5, 0.9, 1.0, nd(1.0), 0.75])
        b = R.choice([0.5, 0.1, 1e-9, 0.25, 2.0 ** -20])
        f = cmp_(R.choice(["greater_than_or_equal", "less_than_or_equal", "greater_than", "less_than"]),
                 interp(tt, var("x")), lit(R.choice([small / 2 if small else 1.0, small, 4000.0, 1e10])))
        cases.append(make_eval_case(f"interp_cx_{k:02d}", f, [("x", q, b)], extra_points=[1.0, nd(1.0), q]))
    return cases


def gen_panic_eval():
    """ROOT ruling 3: the point path's panic inputs, in interval mode."""
    st, su = "stress", "MPa"
    cases = []

    def stress_case(cid, formula, inputs):
        c = make_eval_case(cid, formula, [(i, q, b) for i, q, b in inputs])
        for inp in c["inputs"]:
            inp["dimension"], inp["unit_ref"] = st, su
        c["samples"] = c["samples"][:16]
        c["bisect"] = False
        return c

    ratio = bin_("divide", var("x"), var("y"))
    for bx in (0.0, 1e292, 1.0):
        for by in (0.0, 1e-324, 1e-320):
            cases.append(stress_case(f"panic_quot_{bx!r}_{by!r}",
                                     cmp_("less_than_or_equal", ratio, lit(1.0)),
                                     [("x", 1e308, bx), ("y", 1e-308, by)]))
            cases.append(stress_case(f"panic_quotq_{bx!r}_{by!r}", ratio,
                                     [("x", 1e308, bx), ("y", 1e-308, by)]))
    zz = bin_("multiply", bin_("multiply", var("z"), lit(1e300)), lit(1e300))
    nan = bin_("subtract", zz, zz)
    t = table([(0.0, 0.0), (1.0, 1.0)])
    for name, node in [("interp", interp(t, nan)), ("step", lookup("step", t, nan)),
                       ("exact", lookup("exact", t, nan))]:
        for bz in (0.0, 1e-9, 1.0):
            c = make_eval_case(f"panic_nan_{name}_{bz!r}", cmp_("less_than_or_equal", node, lit(1.0)),
                               [("z", 1.0, bz)])
            c["samples"] = c["samples"][:8]
            c["bisect"] = False
            cases.append(c)
            c2 = make_eval_case(f"panic_nanq_{name}_{bz!r}", node, [("z", 1.0, bz)])
            c2["samples"] = c2["samples"][:8]
            c2["bisect"] = False
            cases.append(c2)
    # NaN through min/max and select, overflow everywhere
    for name, node in [
        ("min_nan", agg("min", [nan, lit(1.0)])),
        ("max_nan", agg("max", [lit(1.0), nan])),
        ("sel_nan", sel(cmp_("less_than", nan, lit(1.0)), lit(1.0), lit(2.0))),
        ("abs_inf", un("abs", zz)),
        ("neg_inf", un("negate", zz)),
        ("inf_div_inf", bin_("divide", zz, zz)),
        ("inf_times_zero", bin_("multiply", zz, lit(0.0))),
    ]:
        for bz in (0.0, 1e-9):
            c = make_eval_case(f"panic_{name}_{bz!r}", cmp_("less_than_or_equal", node, lit(1.0)),
                               [("z", 1.0, bz)])
            c["samples"] = c["samples"][:8]
            c["bisect"] = False
            cases.append(c)
    # extreme bounds and values
    for q, b in [(0.0, 1.7976931348623157e308), (1.7976931348623157e308, 1.0),
                 (-1.7976931348623157e308, 1e292), (1.0, 5e-324), (5e-324, 5e-324),
                 (1e308, 1e308), (-1e308, 1e308)]:
        c = make_eval_case(f"extreme_{q!r}_{b!r}",
                           cmp_("less_than_or_equal", bin_("multiply", var("x"), lit(2.0)), lit(1.0)),
                           [("x", q, b)])
        c["samples"] = c["samples"][:16]
        cases.append(c)
    return cases


# ------------------------------------------------------------------ runner
STRESS_UNITS = ["MPa", "kPa", "Pa", "psi", "ksi", "GPa", "bar"]
TEMP_UNITS = ["degC", "K", "degF"]


def gen_runner(n):
    cases = []
    formulas = {
        "ratio": (bin_("divide", var("actual"), var("limit")), True),
        "pred_le": (cmp_("less_than_or_equal", var("actual"), var("limit")), False),
        "pred_lt": (cmp_("less_than", var("actual"), var("limit")), False),
        "pred_ge": (cmp_("greater_than_or_equal", var("actual"), var("limit")), False),
        "pred_gt": (cmp_("greater_than", var("actual"), var("limit")), False),
        "pred_eq": (cmp_("equal", var("actual"), var("limit")), False),
        "pred_ne": (cmp_("not_equal", var("actual"), var("limit")), False),
        "margin": (bin_("divide", bin_("subtract", var("limit"), var("actual")), var("limit")), True),
        "abs_dev": (bin_("divide", un("abs", bin_("subtract", var("actual"), var("limit"))), var("limit")), True),
        "headroom": (bin_("divide", var("limit"), bin_("subtract", var("limit"), var("actual"))), True),
        "self": (bin_("divide", var("actual"), var("actual")), True),
        "max2": (bin_("divide", agg("max", [var("actual"), un("negate", var("actual"))]), var("limit")), True),
    }
    for k in range(n):
        temp = R.random() < 0.25
        fname = R.choice(list(formulas))
        formula, quantity = formulas[fname]
        if temp:
            dim, units = "temperature", TEMP_UNITS
        else:
            dim, units = "stress", STRESS_UNITS
        declared = R.choice(units)
        entered = R.choice(units) if R.random() < 0.6 else declared
        limit_declared = declared
        limit_value = R.choice([100.0, 1.0, 250.0, 300.0, 1e-6, 1e6, 0.0, -40.0, 1e300])
        # choose q so that the normalized actual is near the limit (approx.)
        q = limit_value * R.choice([1.0, 1 + 1e-15, 1 - 1e-15, 1 + 1e-9, 0.5, 2.0, 1 + 1e-3])
        q = _guess_entered(q, declared, entered, dim)
        if R.random() < 0.15:
            q = rand_val()
        b = rand_bound(q)
        if b == 0.0 and R.random() < 0.7:
            b = abs(q) * 1e-12 or 1e-300
        relation = R.choice([None, "less_than", "less_than_or_equal", "greater_than", "greater_than_or_equal"])
        slot_value = R.choice([1.0, 0.5, 2.0, 0.0, 1e-12, -1.0]) if quantity else None
        enc = my_enclosure(q, b) or (q, q)
        cases.append({
            "id": f"run_{k:04d}_{fname}",
            "formula": to_hex_tree(formula),
            "dimension": dim,
            "declared_unit": declared,
            "entered_unit": entered,
            "actual": hx(q),
            "bound": hx(b),
            "limit": hx(limit_value),
            "limit_unit": limit_declared,
            "slot": hx(slot_value) if slot_value is not None else None,
            "relation": relation,
            "samples": [hx(x) for x in samples_for(enc, extra=[q], n_random=10)],
        })
    # ruling 3: the runner panic input with a bound
    for b in (1e292, 1.0, 1e-300):
        cases.append({
            "id": f"run_panic_quot_{b!r}", "formula": to_hex_tree(formulas["ratio"][0]),
            "dimension": "stress", "declared_unit": "MPa", "entered_unit": "MPa",
            "actual": hx(1e308), "bound": hx(b), "limit": hx(1e-308), "limit_unit": "MPa",
            "slot": hx(1.0), "relation": None, "samples": [hx(1e308)],
        })
    return cases


_STRESS_F = {"Pa": 1.0, "kPa": 1e3, "MPa": 1e6, "GPa": 1e9, "bar": 1e5,
             "psi": 6894.757293168361, "ksi": 6894757.293168361}


def _guess_entered(v_declared, declared, entered, dim):
    if dim == "stress":
        return v_declared * _STRESS_F[declared] / _STRESS_F[entered]
    to_k = {"K": lambda x: x, "degC": lambda x: x + 273.15, "degF": lambda x: (x + 459.67) * 5 / 9}
    from_k = {"K": lambda x: x, "degC": lambda x: x - 273.15, "degF": lambda x: x * 9 / 5 - 459.67}
    return from_k[entered](to_k[declared](v_declared))


def main(out):
    cases = {
        "eval": gen_straddle_eval() + gen_panic_eval() + gen_random_eval(int(__import__("os").environ.get("RV99_NEVAL", "2500"))),
        "runner": gen_runner(int(__import__("os").environ.get("RV99_NRUN", "600"))),
    }
    with open(out, "w") as fh:
        json.dump(cases, fh)
    print(len(cases["eval"]), "eval cases;", sum(len(c["samples"]) for c in cases["eval"]), "samples")
    print(len(cases["runner"]), "runner cases;", sum(len(c["samples"]) for c in cases["runner"]), "samples")


if __name__ == "__main__":
    main(sys.argv[1])
