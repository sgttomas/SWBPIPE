"""RV99 targeted corners (after the mutant survey): a divisor range that ends
at -0/0 with a zero numerator, and table arguments whose enclosure ends exactly
on a row argument."""
import json, sys
sys.path.insert(0, __import__("os").path.dirname(__file__))
from gen_rv99 import (make_eval_case, cmp_, bin_, un, lit, var, table, interp, lookup, nd, nu)
cases = []
zero_over = lambda d: bin_("divide", lit(0.0), d)
for q, b in [(0.0, 1.0), (0.5, 1.0), (0.0, 1e-300), (1e-310, 1e-310)]:
    cases.append(make_eval_case(f"x_zero_over_neg_abs_{q}_{b}", cmp_("less_than_or_equal", zero_over(un("negate", un("abs", var("x")))), lit(1.0)), [("x", q, b)], extra_points=[0.0]))
    cases.append(make_eval_case(f"x_zero_over_abs_{q}_{b}", cmp_("less_than_or_equal", zero_over(un("abs", var("x"))), lit(1.0)), [("x", q, b)], extra_points=[0.0]))
    cases.append(make_eval_case(f"x_zero_over_min_{q}_{b}", cmp_("less_than_or_equal", zero_over(bin_("multiply", var("x"), lit(0.0))), lit(1.0)), [("x", q, b)], extra_points=[0.0]))
t = table([(0.0, 0.0), (1.0, 10.0), (2.0, 100.0)])
for name, node in [("step", lambda a: lookup("step", t, a)), ("interp", lambda a: interp(t, a))]:
    for r in [1.0, 2.0]:
        # enclosure hi exactly r (q = nd(r), tiny b), and lo exactly r (q = nu(r))
        for tag, q in [("hi_at_row", nd(r)), ("lo_at_row", nu(r))]:
            for lim in [5.0, 10.0, 50.0, 100.0]:
                for op in ["less_than_or_equal", "greater_than_or_equal", "less_than", "greater_than"]:
                    cases.append(make_eval_case(f"x_{name}_{tag}_{r}_{op}_{lim}", cmp_(op, node(var("x")), lit(lim)), [("x", q, 2.0**-60)], extra_points=[r]))
json.dump({"eval": cases, "runner": []}, open(sys.argv[1], "w"))
print(len(cases), "extra cases")
