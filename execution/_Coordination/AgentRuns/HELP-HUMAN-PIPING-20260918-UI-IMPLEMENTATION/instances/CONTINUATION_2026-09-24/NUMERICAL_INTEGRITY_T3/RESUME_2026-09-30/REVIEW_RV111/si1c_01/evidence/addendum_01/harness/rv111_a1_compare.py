#!/usr/bin/env python3
"""RV111 addendum 01: the repaired head's dumps (dumps/head2) against round 0's candidate (dumps/cand,
7f233b2e01) and main (dumps/base, 025c1cf326), and its runner lines against the schema.
Expectation: evaluator point and interval lines and the probe are byte-identical; a runner line differs
only where an N-4 input already carried a note, and there the head's note is exactly
"<main's note>; <N-4 note>" (ruling 2); every head line validates.
Usage: rv111_a1_compare.py <scratch> <schema> <report.json>"""
import gzip
import json
import sys
from collections import Counter

import jsonschema

S, SCHEMA, REPORT = sys.argv[1:4]
N4 = "non-finite value (NaN or ±inf, after unit normalization): not bound"


def load(path):
    opener = gzip.open if path.endswith(".gz") else open
    out = {}
    with opener(path, "rt", encoding="utf-8") as fh:
        for line in fh:
            parts = line.rstrip("\n").split("\t")
            out[parts[0]] = parts[1]
    return out


rep = {"counts": Counter(), "violations": []}
c = rep["counts"]
for name in ("ee_point.tsv", "ee_interval.tsv", "probe.tsv"):
    head = load(f"{S}/dumps/head2/{name}")
    cand = load(f"{S}/dumps/cand/{name}.gz")
    assert set(head) == set(cand), name
    same = sum(1 for k in head if head[k] == cand[k])
    c[f"{name}:lines"] = len(head)
    c[f"{name}:identical_to_round0"] = same
    if name == "ee_interval.tsv":
        base = load(f"{S}/dumps/base/{name}.gz")
        c[f"{name}:identical_to_main"] = sum(1 for k in head if head[k] == base[k])

head = load(f"{S}/dumps/head2/run.tsv")
cand = load(f"{S}/dumps/cand/run.tsv.gz")
base = load(f"{S}/dumps/base/run.tsv.gz")
assert set(head) == set(cand) == set(base)
validator = jsonschema.Draft202012Validator(json.load(open(SCHEMA, encoding="utf-8")))
c["run:lines"] = len(head)
for k in head:
    h = json.loads(head[k])
    if list(validator.iter_errors(h)):
        rep["violations"].append(("schema", k))
    if "null" in head[k]:
        rep["violations"].append(("null", k))
    if head[k] == cand[k]:
        c["run:identical_to_round0"] += 1
        continue
    r0, b = json.loads(cand[k]), json.loads(base[k])
    ok = True
    for hc, cc, bc in zip(h["checks"], r0["checks"], b["checks"]):
        for hi, ci, bi in zip(hc["bound_inputs"], cc["bound_inputs"], bc["bound_inputs"]):
            if hi.get("note") == ci.get("note"):
                continue
            existing = bi.get("note")
            if ci.get("note") == N4 and existing and hi.get("note") == f"{existing}; {N4}":
                c[f"run:note_appended:{existing.split(' ')[0]}"] += 1
                hi["note"] = ci["note"]
            else:
                ok = False
    if ok and h == r0:
        c["run:differs_only_by_appended_notes"] += 1
    else:
        rep["violations"].append(("run_other_difference", k))
rep["counts"] = dict(c)
rep["violation_count"] = len(rep["violations"])
rep["violations"] = rep["violations"][:40]
json.dump(rep, open(REPORT, "w"), indent=1, sort_keys=True)
print(json.dumps({"counts": rep["counts"], "violation_count": rep["violation_count"]}, indent=1))
