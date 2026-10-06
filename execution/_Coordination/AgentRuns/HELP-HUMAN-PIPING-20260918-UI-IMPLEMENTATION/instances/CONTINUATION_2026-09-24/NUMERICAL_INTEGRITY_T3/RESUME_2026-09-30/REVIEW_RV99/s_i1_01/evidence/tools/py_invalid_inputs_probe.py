"""RV99: the Python reference (rule_interval.py at 8f956d399a, exported with
`git show`) on invalid bounds and explicit enclosures. Run from a folder that
holds that file. Compare: Rust's runner blocks a NaN, negative or infinite b
(RULE_EVALUATOR_ERROR, input unsupplied), Rust's enclosure_from_bound returns
None for them, and Rust's evaluate_interval blocks an inverted or non-finite
IntervalBinding (InvalidReference / NonFiniteInput)."""
import rule_interval as ri
f = {"node": "compare", "operator": "less_than_or_equal", "left": {"node": "variable_ref", "variable_id": "x"},
     "right": {"node": "literal", "quantity": {"value": 10.0, "dimension": "stress", "unit_ref": "u"}}}
for b in [0.0, 1.0, float("nan"), -1.0, float("inf"), 20.0]:
    r = ri.evaluate_interval(f, [{"variable_id": "x", "value": 9.5, "dimension": "stress", "unit_ref": "u", "bound": b}])
    print("bound", repr(b), r["value"], r["findings"], r["notes"])
for enc in [(11.0, 9.0), (float("nan"), 1.0), (float("-inf"), 0.0)]:
    r = ri.evaluate_interval(f, [{"variable_id": "x", "value": 9.5, "dimension": "stress", "unit_ref": "u", "enclosure": enc}])
    print("enclosure", enc, r["value"], r["findings"], r["notes"])
print("python rust-analog enclosure_from_bound(9.5, nan) ->", ri.enclosure_from_bound(9.5, float("nan")),
      "; (9.5, -1.0) ->", ri.enclosure_from_bound(9.5, -1.0))
