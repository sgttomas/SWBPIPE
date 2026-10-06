"""RV99 checker: soundness against the Rust point path and RV99's exact
oracle, Rust/Python parity, ruling-3 no-panic, and runner outcome contracts.

usage: rv99_check.py <cases.json> <out.json> <rule_interval.py dir> [--no-exact]
"""
from __future__ import annotations

import collections
import math
import json
import re
import struct
import sys
from fractions import Fraction

sys.path.insert(0, __import__("os").path.dirname(__file__))
from rv99_oracle import Undef, exact_eval, my_enclosure  # noqa: E402

cases_path, out_path, ref_dir = sys.argv[1:4]
NO_EXACT = "--no-exact" in sys.argv
sys.path.insert(0, ref_dir)
import rule_interval as ref  # noqa: E402  (I73's Python reference, for parity only)


def f(h):
    return struct.unpack(">d", int(h, 16).to_bytes(8, "big"))[0]


def hx(x):
    return "0x" + struct.pack(">d", x).hex()


def defloat(node):
    if isinstance(node, dict):
        return {k: (f(v) if k in ("value", "argument", "result") and isinstance(v, str)
                    and v.startswith("0x") else defloat(v)) for k, v in node.items()}
    if isinstance(node, list):
        return [defloat(v) for v in node]
    return node


cases = json.load(open(cases_path))
out = json.load(open(out_path))
viol = collections.defaultdict(list)
stats = collections.Counter()

class _Snake(dict):
    def get(self, k, d=None):
        return re.sub(r"(?<!^)([A-Z])", r"_\1", k).lower() if k != "Tbd" else "TBD"


DIMMAP = _Snake()

for case, res in zip(cases["eval"], out["eval"]):
    cid = case["id"]
    assert cid == res["id"]
    stats["eval_cases"] += 1
    iv = res["interval"]
    if iv.get("panic"):
        viol["interval_panic"].append(cid)
        continue
    formula = defloat(case["formula"])
    inputs = case["inputs"]
    # input enclosures: Rust vs RV99's own transcription, and exact containment
    for inp, enc in zip(inputs, res["input_enclosures"]):
        q, b = f(inp["value"]), f(inp["bound"])
        mine = my_enclosure(q, b)
        mine_h = None if mine is None else [hx(mine[0]), hx(mine[1])]
        if mine_h != enc:
            viol["input_enclosure_mismatch"].append((cid, enc, mine_h))
        if enc is not None:
            lo, hi = f(enc[0]), f(enc[1])
            if not (math.isfinite(lo) and math.isfinite(hi)):
                viol["input_enclosure_non_finite"].append(cid)
                continue
            if not (Fraction(lo) <= Fraction(q) - Fraction(b) and Fraction(q) + Fraction(b) <= Fraction(hi)):
                viol["input_enclosure_not_containing_box"].append(cid)
    # parity with I73's Python reference
    py_inputs = [{"variable_id": i["id"], "value": f(i["value"]), "dimension": i["dimension"],
                  "unit_ref": i["unit_ref"], "bound": f(i["bound"])} for i in inputs]
    try:
        py = ref.evaluate_interval(formula, py_inputs)
    except Exception as e:  # noqa: BLE001
        viol["python_exception"].append((cid, repr(e)))
        py = None
    rv = iv["value"]
    if py is not None:
        pf = [list(x) for x in py["findings"]]
        if pf != iv["findings"]:
            viol["parity_findings"].append((cid, iv["findings"], pf))
        pn = [list(x) for x in py["notes"]]
        if pn != iv["notes"]:
            viol["parity_notes"].append((cid, iv["notes"], pn))
        pv = py["value"]
        if rv is None or pv is None:
            if (rv is None) != (pv is None):
                viol["parity_value"].append((cid, rv, pv))
        elif rv["kind"] != pv["kind"]:
            viol["parity_value"].append((cid, rv, pv))
        elif rv["kind"] == "truth":
            if rv["truth"] != pv["truth"]:
                viol["parity_value"].append((cid, rv, pv))
        else:
            pe = None if pv["enclosure"] is None else [hx(pv["enclosure"][0]), hx(pv["enclosure"][1])]
            if pe != rv["enclosure"] or rv["unit_ref"] != pv["unit_ref"] or \
                    DIMMAP.get(rv["dimension"], rv["dimension"]) != pv["dimension"]:
                viol["parity_value"].append((cid, rv, pv))
        stats["parity_compared"] += 1
    # soundness against the Rust point path at every sample
    points = res["points"]
    samples = res["samples"]
    stats["point_samples"] += len(points)
    if iv["findings"]:
        stats["interval_blocked"] += 1
        if any(not p.startswith("blocked") for p in points):
            viol["structural_block_point_not_blocked"].append(cid)
        continue
    if rv is None:
        viol["no_value_no_findings"].append(cid)
        continue
    if rv["kind"] == "truth":
        t = rv["truth"]
        stats["truth_" + t] += 1
        if t == "T" and any(p != "bool:true" for p in points):
            viol["T_but_point_not_true"].append((cid, sorted(set(points))))
        if t == "F" and any(p != "bool:false" for p in points):
            viol["F_but_point_not_false"].append((cid, sorted(set(points))))
        if t == "U":
            kinds = set(points)
            if kinds in ({"bool:true"}, {"bool:false"}):
                stats["U_conservative_all_points_agree"] += 1
            else:
                stats["U_points_disagree_or_block"] += 1
    else:
        e = rv["enclosure"]
        if e is None:
            stats["quantity_no_enclosure"] += 1
        else:
            stats["quantity_enclosure"] += 1
            lo, hi = f(e[0]), f(e[1])
            if not (math.isfinite(lo) and math.isfinite(hi)):
                viol["quantity_enclosure_non_finite"].append(cid)
                continue
            for p in points:
                if not p.startswith("q:"):
                    viol["enclosure_but_point_not_quantity"].append((cid, p))
                    break
                v = f(p[2:])
                if not (lo <= v <= hi):
                    viol["enclosure_misses_point_value"].append((cid, p, e))
                    break
    # RV99's exact oracle at the samples
    if NO_EXACT:
        continue
    decided = (rv["kind"] == "truth" and rv["truth"] in "TF") or \
              (rv["kind"] == "quantity" and rv["enclosure"] is not None)
    if not decided:
        continue
    ids = [i["id"] for i in inputs]
    for tup in samples[::max(1, len(samples) // 120)]:
        env = {k: Fraction(f(v)) for k, v in zip(ids, tup)}
        stats["exact_samples"] += 1
        try:
            kind, val = exact_eval(formula, env)
        except Undef as u:
            viol["decided_but_exact_undefined"].append((cid, str(u), tup))
            break
        if rv["kind"] == "truth":
            if val != (rv["truth"] == "T"):
                viol["exact_contradicts_truth"].append((cid, rv["truth"], tup))
                break
        else:
            lo, hi = f(rv["enclosure"][0]), f(rv["enclosure"][1])
            if not (Fraction(lo) <= val <= Fraction(hi)):
                viol["exact_outside_enclosure"].append((cid, tup))
                break

MSG = re.compile(r"^enclosure=(\[0x[0-9a-f]{16},0x[0-9a-f]{16}\]|none) unit=\S+(; causes=[a-z_,]+)?$")
for case, res in zip(cases["runner"], out["runner"]):
    cid = case["id"]
    stats["runner_cases"] += 1
    bd = res["bounded"]
    if bd.get("panic"):
        viol["runner_bounded_panic"].append(cid)
        continue
    if res["zero_bound_equal"] == "panic":
        stats["runner_zero_bound_point_path_panic_T3_SI1b"] += 1
    elif res["zero_bound_equal"] is not True:
        viol["runner_zero_bound_differs"].append((cid, res["zero_bound_equal"]))
    st = bd["status"]
    stats["runner_" + st] += 1
    pts = res["points"]
    stats["runner_point_samples"] += len(pts)
    if st == "USER_RULE_CHECKED" and any(p != "USER_RULE_CHECKED" for p in pts):
        viol["runner_checked_but_point_not"].append((cid, sorted(set(pts))))
    if st == "USER_RULE_FAILED" and any(p != "USER_RULE_FAILED" for p in pts):
        viol["runner_failed_but_point_not"].append((cid, sorted(set(pts))))
    if st == "RULE_INPUTS_INCOMPLETE" and set(pts) in ({"USER_RULE_CHECKED"}, {"USER_RULE_FAILED"}):
        stats["runner_U_conservative"] += 1
    codes = bd["diagnostic_codes"]
    finds = bd["evaluator_findings"]
    b = f(case["bound"])
    interval_codes = {"RULE_INTERVAL_ALL_PASS", "RULE_INTERVAL_ALL_FAIL", "RULE_RESULT_INDETERMINATE"}
    outcome = [x for x in finds if x[0] in interval_codes]
    if b > 0 and not math.isfinite(b):
        stats["runner_invalid_bound"] += 1
        if bd["notes"][0] != f"invalid absolute bound inf: treated as unsupplied" or bd["status"] != "RULE_INPUTS_INCOMPLETE":
            viol["runner_invalid_bound_shape"].append((cid, bd["notes"][0], bd["status"]))
    elif b > 0:
        note = bd["notes"][0]
        exp = None
        # Rust {:e}: verify shape only (digits, optional fraction, 'e', exponent)
        if note is not None and not re.fullmatch(r"interval ±-?\d(\.\d+)?e-?\d+ from receipt", note):
            viol["runner_note_format"].append((cid, note))
        if st == "USER_RULE_CHECKED":
            if not (codes.count("RULE_INTERVAL_ALL_PASS") == 1 and len(outcome) == 1
                    and outcome[0][0] == "RULE_INTERVAL_ALL_PASS" and outcome[0][1] == "info"):
                viol["runner_checked_codes"].append((cid, codes, outcome))
        if st == "USER_RULE_FAILED":
            if not (codes.count("RULE_INTERVAL_ALL_FAIL") == 1 and len(outcome) == 1
                    and outcome[0][0] == "RULE_INTERVAL_ALL_FAIL" and outcome[0][1] == "info"):
                viol["runner_failed_codes"].append((cid, codes, outcome))
        if outcome:
            if len(outcome) != 1:
                viol["runner_multiple_outcome_findings"].append(cid)
            o = outcome[0]
            if o[0] == "RULE_RESULT_INDETERMINATE" and (o[1] != "warning" or st != "RULE_INPUTS_INCOMPLETE"):
                viol["runner_indeterminate_shape"].append((cid, o, st))
            if o[2] != "c1":
                viol["runner_outcome_subject"].append((cid, o))
            if not MSG.match(o[3]):
                viol["runner_message_format"].append((cid, o[3]))
            if bd["computed_value"]:
                viol["runner_interval_computed_value_present"].append(cid)
            stats["runner_outcome_" + o[0]] += 1
        for x in finds:
            if x[0] == "RULE_INTERVAL_DIVIDE_BY_ZERO_RANGE":
                stats["runner_div0_finding"] += 1
                if x[1] != "warning":
                    viol["runner_div0_severity"].append(cid)
    else:
        if outcome:
            viol["runner_interval_codes_without_bound"].append(cid)

print(json.dumps(dict(sorted(stats.items())), indent=1))
print("VIOLATIONS:", {k: len(v) for k, v in viol.items()})
for k, v in viol.items():
    print("--", k)
    for item in v[:6]:
        print("   ", str(item)[:600])
