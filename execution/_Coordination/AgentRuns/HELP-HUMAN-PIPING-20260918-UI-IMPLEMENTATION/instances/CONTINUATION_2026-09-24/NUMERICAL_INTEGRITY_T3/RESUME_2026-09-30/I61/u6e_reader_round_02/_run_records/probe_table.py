#!/usr/bin/env python3
"""I61 07h: the cross-reader probe table (07g readers against 07h readers, all three languages)."""
import json, sys
D = sys.argv[1]
rows = [json.loads(l) for l in open(f"{D}/applied_probes.jsonl")]
o = {f"{l}_{v}": json.load(open(f"{D}/out_{l}_{v}.json")) for v in ("07g", "07h") for l in ("py", "rs", "ts")}
fmt = lambda x: "pass " + str(x["classes"]) if isinstance(x, dict) else "/".join(x).replace("RETAINED_PRECISION_", "RP_").replace("SOURCE_PREVIEW_PHYSICS_", "SPP_")
gate = lambda x: "pass " + str(x["classes"]) if isinstance(x, dict) else x[0]
code = lambda x: "pass" if isinstance(x, dict) else x[1]
print("| probe | expected | py 07g | rs 07g | ts 07g | py 07h | rs 07h | ts 07h |")
print("|---|---|---|---|---|---|---|---|")
miss, gate_div, code_div = [], [], []
for r in rows:
    k = r["kind"] + ":" + r["id"]; e = r["expected"]
    es = e if isinstance(e, str) else "/".join(e).replace("RETAINED_PRECISION_", "RP_")
    print(f"| {k} | {es} | " + " | ".join(fmt(o[n][k]) for n in ("py_07g", "rs_07g", "ts_07g", "py_07h", "rs_07h", "ts_07h")) + " |")
    new = [o[n][k] for n in ("py_07h", "rs_07h", "ts_07h")]
    if isinstance(e, list) and any(x != e for x in new): miss.append(k)
    if e == "pass" and any(not isinstance(x, dict) for x in new): miss.append(k)
    if e.startswith("G7") if isinstance(e, str) else False:
        if any(isinstance(x, dict) or x[0] != "G7" for x in new): miss.append(k)
    if len({json.dumps(gate(x)) for x in new}) > 1: gate_div.append(k)
    if len({json.dumps(code(x)) for x in new}) > 1: code_div.append(k)
print("\nexpectation misses (07h readers):", miss)
print("07h cross-reader gate divergences:", gate_div)
print("07h cross-reader code divergences:", code_div)
