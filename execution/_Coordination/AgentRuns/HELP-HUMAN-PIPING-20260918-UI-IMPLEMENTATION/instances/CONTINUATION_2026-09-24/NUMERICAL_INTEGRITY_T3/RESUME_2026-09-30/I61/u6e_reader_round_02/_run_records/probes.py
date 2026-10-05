#!/usr/bin/env python3
"""I61 07h: cross-reader probes. RV90's six S1 probes (affected_refs as a string, a number or an
object, each listed and unlisted), its typed-integrity string probe, its N1 second-case probes,
its N2 reordered-envelope probes and the two real milestone receipts, applied with the corpus
harness (rehash "all") and written as one applied JSONL for the Python, Rust and TypeScript dumps.

Usage: probes.py P_ROOT OUT_JSONL
"""
import json, sys, importlib.util
from copy import deepcopy
from pathlib import Path

P, OUT = Path(sys.argv[1]), Path(sys.argv[2])
sys.path.insert(0, str(P))
spec = importlib.util.spec_from_file_location("harness", P / "tests/test_retained_precision_contract.py")
h = importlib.util.module_from_spec(spec); spec.loader.exec_module(h)
corpus = json.loads((P / "fixtures/results/retained_precision_cases.json").read_text())
case = lambda name: next(f for f in corpus["cases"] if f["id"] == name)
set_ = lambda path, value: {"path": path, "op": "set", "value": value}
REFS = lambda i: ["retained_precision", "body", "ordinary_attempts", i, "diagnostic_refs"]
rows = []


def add(group, pid, fixture, edits, expected):
    source, invocation = h.apply_entry(fixture, {"id": pid, "edits": edits, "rehash": "all"})
    rows.append({"kind": group, "id": pid, "expected": expected, "source": source, "invocation": invocation})


ATTEMPT = ["G5", "RETAINED_PRECISION_ATTEMPT_MISMATCH"]
G7 = "G7 (per-reader code)"
one = case("ordinary_prepared_synthetic")
CID = "case:six-component-load"
ds = one["source"]["diagnostics"]
exact0 = one["source"]["retained_precision"]["body"]["ordinary_attempts"][0]["diagnostic_refs"]
k = next(j for j, d in enumerate(ds) if d["code"] == "LOAD_CATEGORY_PREVIEW_MAPPED")
for name, value in (("string", CID), ("number", 5), ("object", {CID: 1})):
    add("s1", f"affected_refs_{name}_listed", one, [set_(["diagnostics", k, "affected_refs"], value)], ATTEMPT)
    add("s1", f"affected_refs_{name}_unlisted", one, [set_(["diagnostics", k, "affected_refs"], value),
                                                       set_(REFS(0), [x for x in exact0 if x != ds[k]["id"]])], G7)
ki = next(j for j, d in enumerate(ds) if d["code"] == "NUMERICAL_INTEGRITY_SENSITIVE")
add("s1", "typed_integrity_affected_refs_string", one, [set_(["diagnostics", ki, "affected_refs"], CID)], ATTEMPT)

two = case("two_case_synthetic")
e1 = two["source"]["retained_precision"]["body"]["ordinary_attempts"][1]["diagnostic_refs"]
add("n1", "second_case_swapped", two, [set_(REFS(1), [e1[1], e1[0]] + e1[2:])], ATTEMPT)
add("n1", "second_case_relaxed_form", two, [set_(REFS(1), e1[-1:])], ATTEMPT)

moved = ds[:k] + ds[k + 1:] + [ds[k]]
exact_moved = [d["id"] for d in moved if isinstance(d.get("affected_refs"), list) and CID in d["affected_refs"]
               and not d["code"].startswith("RETAINED_PRECISION_")]
add("n2", "envelope_reordered_exact", one, [set_(["diagnostics"], moved), set_(REFS(0), exact_moved)], "pass")
add("n2", "envelope_reordered_strict_prefix", one, [set_(["diagnostics"], moved), set_(REFS(0), exact_moved[:-1])], ATTEMPT)
add("n2", "corpus_base_truncated_last", one, [set_(REFS(0), exact0[:-1])], ATTEMPT)

for mode in ("sparse_interactive", "dense_scrutiny"):
    doc = json.loads((P / f"fixtures/results/retained_precision_milestone_successor_{mode}.json").read_text())
    rows.append({"kind": "milestone", "id": mode + ":unmodified", "expected": "pass", "source": deepcopy(doc["source"]), "invocation": deepcopy(doc["invocation"])})

with open(OUT, "w") as f:
    for r in rows:
        f.write(json.dumps(r) + "\n")
print("probes", len(rows))
