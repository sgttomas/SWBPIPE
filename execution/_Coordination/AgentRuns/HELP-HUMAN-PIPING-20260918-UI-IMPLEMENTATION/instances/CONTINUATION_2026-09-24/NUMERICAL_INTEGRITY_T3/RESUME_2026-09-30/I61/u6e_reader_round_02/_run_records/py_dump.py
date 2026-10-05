#!/usr/bin/env python3
"""I61 07h: run one Python reader (P_ROOT) over an applied JSONL through the public entry; write outcomes."""
import sys, json
P, inp, out = sys.argv[1], sys.argv[2], sys.argv[3]
sys.path.insert(0, P)
from core.analysis_runs import retained_precision as rp
res = {}
for line in open(inp):
    d = json.loads(line)
    try:
        r = rp.validate_retained_precision(d["source"], d["invocation"])
        got = {"pass": True, "eligible": r["numerical_eligible"], "standing": r["standing"], "classes": [sum(1 for c in r["classifications"] if c["class"] == k) for k in ("relative_verified", "absolute_verified", "input_derived", "non_quantity")]}
    except rp.RetainedPrecisionError as e:
        got = [e.gate, e.code]
    res[d["kind"] + ":" + d["id"]] = got
json.dump(res, open(out, "w"), indent=0, sort_keys=True)
print("validated", len(res), out)
