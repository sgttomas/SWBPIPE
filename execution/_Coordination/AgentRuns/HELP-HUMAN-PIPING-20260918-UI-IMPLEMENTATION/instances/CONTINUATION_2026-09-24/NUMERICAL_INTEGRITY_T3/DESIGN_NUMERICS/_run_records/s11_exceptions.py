#!/usr/bin/env python3
"""D1 revision 5, D5C-5: the predicted S11 exceptions of the "no Passed breach" gate, as
(entry, case, quantity) triples, derived from R1's frozen negative controls (standard library only).

Usage (from T3/): python3 DESIGN_NUMERICS/_run_records/s11_exceptions.py REFERENCES/references.json

Rule: main folds a DOF's load contributions left to right in the authored order (P1: main matches
NC-FLOAT-SUM-<authored order> at every mismatching case). So for each RF-CANCEL case, the control
whose id names the case's own authored order predicts main's values; its `values` keys are the
quantities that breach under the binding net-governed scale. For RF-CANCEL-UDL the authored order
is (A, node, B); main's fold order was observed by P1 to differ from both R1 controls (ratio 46.5 at
W1e8), so both controls' keys are taken. Entries: the captured route refuses every G = 1e80 case at
capture (V1-S5), so 1e80 cases are exceptions on the historical typed entry only.
This is a prediction. The committed list is P1's frozen-reference record (results.json); any
difference between the two is reported, not absorbed.
"""
import json
import sys

ref = json.load(open(sys.argv[1]))["cases"]
out = []
for cid, c in ref.items():
    if not cid.startswith("RF-CANCEL"):
        continue
    parts = cid.split("-")
    if parts[2] == "UDL":
        ctrls = [n for n in c.get("negative_controls", []) if n["id"].startswith("NC-FLOAT-SUM") and n.get("discriminates")]
        big = parts[3] != "W1e5"
    else:
        order = parts[4]
        ctrls = [n for n in c.get("negative_controls", []) if n["id"] == "NC-FLOAT-SUM-" + order and n.get("discriminates")]
        big = parts[3] in ("G1e7", "G1e8", "G1e80")
    keys = sorted({k for n in ctrls for k in n.get("values", {})})
    if not keys or not big:
        continue
    entries = ["typed"] if ("1e80" in cid) else ["captured", "typed"]
    for e in entries:
        for k in keys:
            out.append([e, cid, k])
cases = {}
for e, cid, k in out:
    cases.setdefault(e, set()).add(cid)
print(json.dumps({"triples": out, "count": len(out),
                  "cases_per_entry": {e: sorted(v) for e, v in cases.items()},
                  "case_counts": {e: len(v) for e, v in cases.items()}}, indent=1))
