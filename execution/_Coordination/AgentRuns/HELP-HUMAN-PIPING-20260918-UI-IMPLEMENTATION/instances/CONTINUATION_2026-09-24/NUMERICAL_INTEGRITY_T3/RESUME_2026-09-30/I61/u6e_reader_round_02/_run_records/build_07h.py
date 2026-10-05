#!/usr/bin/env python3
"""I61 U6e repair round: stage snapshot 07h from 07g (RV90 S1, N1, N2, N4).

Usage: build_07h.py P_ROOT OUT_CORPUS
- S1: one mutation, a case-naming diagnostic whose affected_refs is the case id as a string
  (not an array) and still listed: a non-array names no case in every reader.
- N1: one mutation, F5 on the second case (07f's relaxed D6a form for case 1 of two_case_synthetic).
- N2: one mutation, a strict prefix of A2's list (RV90's construction: a naming load diagnostic moved
  after the integrity diagnostic in the envelope, the list without its last element), and one
  must-pass control, the same reordered envelope with its exact list.
- N4: the facade-order premise appended to d37.basis.
Everything else in 07g is copied unchanged (asserted).
"""
import json, sys
from copy import deepcopy
from pathlib import Path

P, OUT = Path(sys.argv[1]), Path(sys.argv[2])
SRC = P / "fixtures/results/retained_precision_cases.json"
data = json.loads(SRC.read_bytes())
before = deepcopy(data)
assert (len(data["cases"]), len(data["mutations"]), len(data["must_pass"])) == (15, 274, 22), "07g"


def a2_list(diagnostics, case_id):
    return [d["id"] for d in diagnostics
            if isinstance(d.get("affected_refs"), list) and case_id in d["affected_refs"]
            and not str(d.get("code")).startswith("RETAINED_PRECISION_")]


def base(name):
    return next(f for f in data["cases"] if f["id"] == name)["source"]


ATTEMPT = {"gate": "G5", "code": "RETAINED_PRECISION_ATTEMPT_MISMATCH"}
BASE, TWO = "ordinary_prepared_synthetic", "two_case_synthetic"
CID = "case:six-component-load"
REFS = lambda i: ["retained_precision", "body", "ordinary_attempts", i, "diagnostic_refs"]
set_ = lambda path, value: {"path": path, "op": "set", "value": value}

one = base(BASE)
ds = one["diagnostics"]
exact0 = one["retained_precision"]["body"]["ordinary_attempts"][0]["diagnostic_refs"]
assert exact0 == a2_list(ds, CID) and len(exact0) == 7
k = next(j for j, d in enumerate(ds) if d["code"] == "LOAD_CATEGORY_PREVIEW_MAPPED")
assert k == 0 and ds[k]["affected_refs"] == ["load:fixture-global_x", CID] and ds[k]["id"] == exact0[0]

# S1: the first load diagnostic's affected_refs is the case id as a string; the base list still lists it.
s1 = {"id": "f5_affected_refs_string_names_no_case", "base": BASE,
      "edits": [set_(["diagnostics", k, "affected_refs"], CID)], "rehash": "all", "expected": ATTEMPT}

# N1: F5 on the second case: case 1's list in 07f's relaxed D6a form (its integrity diagnostic only).
two = base(TWO)
c1 = two["retained_precision"]["body"]["cases"][1]["basis_ref"]["ref_id"]
exact1 = two["retained_precision"]["body"]["ordinary_attempts"][1]["diagnostic_refs"]
assert c1 == "case:zero-load" and exact1 == a2_list(two["diagnostics"], c1) and len(exact1) == 7
assert exact1[-1] == "diagnostic:numerical-integrity:case:zero-load"
n1 = {"id": "f5_ordinary_refs_second_case_relaxed_d6a_form", "base": TWO,
      "edits": [set_(REFS(1), exact1[-1:])], "rehash": "all", "expected": ATTEMPT}

# N2: the first load diagnostic moved to the end of the envelope (after the integrity diagnostic);
# A2's list then ends with it, so the list without its last element is a strict prefix that still
# lists the integrity diagnostic. The exact list over the same envelope is the must-pass control.
moved = ds[:k] + ds[k + 1:] + [ds[k]]
exact_moved = a2_list(moved, CID)
assert exact_moved == exact0[1:] + [exact0[0]] and exact_moved[-2] == "diagnostic:numerical-integrity:case:six-component-load"
n2 = {"id": "f5_ordinary_refs_strict_prefix", "base": BASE,
      "edits": [set_(["diagnostics"], moved), set_(REFS(0), exact_moved[:-1])], "rehash": "all", "expected": ATTEMPT}
n2_control = {"id": "f5_envelope_reordered_exact_list", "base": BASE,
              "edits": [set_(["diagnostics"], moved), set_(REFS(0), exact_moved)], "rehash": "all", "expected": "pass"}

data["mutations"].extend([s1, n1, n2])
data["must_pass"].append(n2_control)

# N4: the facade-order premise behind the exclusion of `capture` at C--------- .
PREMISE = (" Facade order (PP/core/product_physics/src/lib.rs:2962-2974, unchanged from 844448112f to 5e1e2625ac): "
           "prepare_case :2962, then solve_native :2968, and freeze_candidate :2971 runs only after solve_native "
           "returns Ok; the only other caller, project_candidate (retained_product.rs:3644-3646), is reached only "
           "from #[cfg(test)] code. So capture (b) never sees Native not entered, and C--------- is not a "
           "`capture` record.")
assert PREMISE.strip() not in data["d37"]["basis"]
data["d37"]["basis"] += PREMISE

# Unchanged: everything else (asserted).
for key in data:
    if key not in ("mutations", "must_pass", "d37"):
        assert data[key] == before[key], key
assert data["mutations"][:274] == before["mutations"] and data["must_pass"][:22] == before["must_pass"]
assert {k2: v for k2, v in data["d37"].items() if k2 != "basis"} == {k2: v for k2, v in before["d37"].items() if k2 != "basis"}
assert data["d37"]["basis"].startswith(before["d37"]["basis"])
assert list(data) == list(before)

OUT.write_text(json.dumps(data, indent=2) + "\n")
print("staged", OUT, len(data["cases"]), len(data["mutations"]), len(data["must_pass"]))
