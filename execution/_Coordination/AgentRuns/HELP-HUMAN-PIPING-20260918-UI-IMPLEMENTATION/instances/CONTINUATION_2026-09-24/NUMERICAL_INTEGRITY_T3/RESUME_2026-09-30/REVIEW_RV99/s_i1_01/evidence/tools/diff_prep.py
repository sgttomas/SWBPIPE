"""RV99 point-mode differential inputs (independent of I73's control 1).

Packs: the two committed rule packs (examples/rule_packs/invented_demo.yaml and
fixtures/product_preview/invented_demo_rule_pack.json), demo-pack variants
(every acceptability_relation token, an invalid and an empty token, declared
units MPa and Pa, a restricted result_statuses), and one pack per RV99 random
formula (x, z solver results; y user value; ratio slot). Values: every finite
(value, unit) row of the committed run fixtures (product_preview
invented_mechanics_result*.json and fixtures/results/**/*.json), bound in the
declared unit and in its own unit, plus scenarios (no solver value, a refused
solver result, no slot, MODEL_INCOMPLETE, zero and negative limits).

usage: diff_prep.py <P root> <rv99_cases.json> <out.json>
"""
from __future__ import annotations

import copy
import glob
import json
import math
import os
import random
import struct
import sys

P, CASES, OUT = sys.argv[1:4]
R = random.Random(9999)


def hx(x):
    return "0x" + struct.pack(">d", float(x)).hex()


def hexify_numbers(node):
    if isinstance(node, dict):
        return {k: (hx(v) if k in ("value", "argument", "result") and isinstance(v, (int, float))
                    and not isinstance(v, bool) else hexify_numbers(v)) for k, v in node.items()}
    if isinstance(node, list):
        return [hexify_numbers(v) for v in node]
    return node


packs = []
demo_paths = ["examples/rule_packs/invented_demo.yaml", "fixtures/product_preview/invented_demo_rule_pack.json"]
for p in demo_paths:
    packs.append({"id": p, "doc": hexify_numbers(json.load(open(os.path.join(P, p))))})
base = json.load(open(os.path.join(P, demo_paths[0])))
for rel in ["less_than", "less_than_or_equal", "greater_than", "greater_than_or_equal", "equal", "bogus", ""]:
    d = copy.deepcopy(base)
    d["check_definitions"][0]["acceptability_relation"] = rel
    packs.append({"id": f"demo_rel_{rel or 'empty'}", "doc": hexify_numbers(d)})
for unit in ["MPa", "Pa"]:
    d = copy.deepcopy(base)
    for r in d["required_inputs"]:
        r["quantity_intent"]["unit_ref"] = unit
    packs.append({"id": f"demo_unit_{unit}", "doc": hexify_numbers(d)})
d = copy.deepcopy(base)
d["check_definitions"][0]["result_statuses"] = ["RULE_INPUTS_INCOMPLETE", "USER_RULE_FAILED"]
packs.append({"id": "demo_restricted_statuses", "doc": hexify_numbers(d)})
n_demo = len(packs)

# fixture rows
files = glob.glob(os.path.join(P, "fixtures/product_preview/invented_mechanics_result*.json")) + \
    glob.glob(os.path.join(P, "fixtures/results/**/*.json"), recursive=True)
pairs = set()


def walk(n):
    if isinstance(n, dict):
        v, u = n.get("value"), n.get("unit") or n.get("unit_ref")
        if isinstance(v, (int, float)) and not isinstance(v, bool) and isinstance(u, str) and math.isfinite(v):
            pairs.add((float(v), u))
        for x in n.values():
            walk(x)
    elif isinstance(n, list):
        for x in n:
            walk(x)


for f in sorted(files):
    walk(json.load(open(f)))
pairs = sorted(pairs)

runs = []
for i in range(n_demo):
    pid = packs[i]["id"]
    declared = "demo_unit"
    if pid.startswith("demo_unit_"):
        declared = pid.split("_")[-1]
    for v, u in pairs:
        for unit in (declared, u):
            runs.append({"pack": i, "solver": [["demo_actual_quantity", hx(v), unit]], "refused": [],
                         "supplied": [["demo_limit_quantity", hx(100.0), declared, "stress"],
                                      ["demo_limit_slot", hx(1.0), "ratio", "dimensionless"]],
                         "statuses": ["MechanicsSolved"]})
    sample = R.sample(pairs, 40)
    for v, u in sample:
        sv = [["demo_actual_quantity", hx(v), declared]]
        good = [["demo_limit_quantity", hx(100.0), declared, "stress"],
                ["demo_limit_slot", hx(1.0), "ratio", "dimensionless"]]
        runs.append({"pack": i, "solver": [], "refused": [], "supplied": good, "statuses": ["MechanicsSolved"]})
        runs.append({"pack": i, "solver": sv, "refused": [["demo_actual_quantity", "RETAINED_PRECISION_REFUSED"]],
                     "supplied": good, "statuses": ["MechanicsSolved"]})
        runs.append({"pack": i, "solver": sv, "refused": [], "supplied": good[:1], "statuses": ["MechanicsSolved"]})
        runs.append({"pack": i, "solver": sv, "refused": [], "supplied": good, "statuses": ["ModelIncomplete"]})
        runs.append({"pack": i, "solver": sv, "refused": [],
                     "supplied": [["demo_limit_quantity", hx(0.0), declared, "stress"], good[1]],
                     "statuses": ["MechanicsSolved"]})
        runs.append({"pack": i, "solver": sv, "refused": [],
                     "supplied": [["demo_limit_quantity", hx(-v or -1.0), declared, "stress"], good[1]],
                     "statuses": ["MechanicsSolved", "HumanApprovedForProject"]})

# random-formula packs from RV99's evaluator cases
cases = json.load(open(CASES))["eval"]


def decl(i, kind):
    return {"input_id": i, "name": i, "source_kind": kind, "required_for": "rule_check",
            "provenance_required": True, "redistribution_status_required": True,
            "quantity_intent": {"dimension": "dimensionless", "unit_ref": "ratio",
                                "unit_required": True, "dimension_check_required": True}}


for c in cases:
    if not c["id"].startswith(("rand_", "str_", "neg_", "interp_", "pair_")):
        continue
    if any(i["dimension"] != "dimensionless" for i in c["inputs"]):
        continue
    ids = ["x", "y", "z"]
    kinds = {"x": "solver_result", "y": "user_supplied_rule_value", "z": "solver_result"}
    doc = {
        "grammar_version": "1.0.0",
        "metadata": {"rule_pack_id": "rv99_" + c["id"]},
        "required_inputs": [decl(i, kinds[i]) for i in ids],
        "formula_declarations": [{"formula_id": "f", "declaration_payload": {"expression_ast": c["formula"]},
                                  "input_refs": [{"ref_id": i, "ref_type": "required_input"} for i in ids]}],
        "value_slots": [{"slot_id": "s", "slot_kind": "ratio_limit",
                         "quantity_intent": {"dimension": "dimensionless", "unit_ref": "ratio",
                                             "unit_required": True, "dimension_check_required": True}}],
        "check_definitions": [{
            "check_id": "c", "required_input_refs": [{"ref_id": i, "ref_type": "required_input"} for i in ids],
            "value_slot_refs": [{"ref_id": "s", "ref_type": "value_slot"}],
            "formula_ref": {"ref_id": "f", "ref_type": "formula"},
            "acceptability_relation": R.choice(["less_than", "less_than_or_equal", "greater_than",
                                                "greater_than_or_equal"]),
            "result_statuses": ["RULE_INPUTS_INCOMPLETE", "USER_RULE_CHECKED", "USER_RULE_FAILED"],
            "diagnostic_policy": {"missing_input": "RULE_INPUT_MISSING", "evaluator_error": "RULE_EVALUATOR_ERROR"}}],
    }
    packs.append({"id": "rv99_" + c["id"], "doc": doc})
    pi = len(packs) - 1
    names = [i["id"] for i in c["inputs"]]
    for tup in c["samples"][:6]:
        vals = dict(zip(names, tup))
        vals = {i: vals.get(i, hx(1.0)) for i in ids}
        runs.append({"pack": pi, "solver": [["x", vals["x"], "ratio"], ["z", vals["z"], "ratio"]], "refused": [],
                     "supplied": [["y", vals["y"], "ratio", "dimensionless"],
                                  ["s", hx(R.choice([1.0, 0.0, -1.0, 1e300, 1e-300])), "ratio", "dimensionless"]],
                     "statuses": ["MechanicsSolved"]})

json.dump({"packs": packs, "runs": runs}, open(OUT, "w"))
print(len(packs), "packs;", len(runs), "runs;", len(pairs), "fixture (value, unit) rows from", len(files), "files")
