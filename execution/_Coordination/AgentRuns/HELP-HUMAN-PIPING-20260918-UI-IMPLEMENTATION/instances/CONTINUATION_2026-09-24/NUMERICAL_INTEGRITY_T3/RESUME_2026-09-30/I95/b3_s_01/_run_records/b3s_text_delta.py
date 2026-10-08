"""I95 B3-S: per-function TEXT delta between two text_budget outputs (same line basis).
Usage: python3 b3s_text_delta.py <base text_budget json> <exact text_budget json> [top]"""
import json, sys, collections
b, d = (json.load(open(p)) for p in sys.argv[1:3])
top = int(sys.argv[3]) if len(sys.argv) > 3 else 40
def byfn(t):
    c, dd = collections.Counter(), collections.Counter()
    for r in t["rows"]:
        k = (r["fn"] or "").split("/src/")[-1]
        c[k] += r["req"]
        if r["kind"] == "diag":
            dd[k] += r["mult"]
    return c, dd
cd, dd = byfn(d); cb, db = byfn(b)
Md, Mb = d["function_multiplicity"], b["function_multiplicity"]
short = lambda k: k.split("/src/")[-1]
Ms_d = {short(k): v for k, v in Md.items()}; Ms_b = {short(k): v for k, v in Mb.items()}
rows = []
for f in set(cd) | set(cb):
    if cd[f] != cb[f] or dd[f] != db[f]:
        rows.append((cd[f] - cb[f], dd[f] - db[f], f, Ms_b.get(f, 0), Ms_d.get(f, 0)))
rows.sort(key=lambda x: -x[0])
print(json.dumps({"TAV": [b["total_text_requested_bytes"], d["total_text_requested_bytes"]],
                  "delta": d["total_text_requested_bytes"] - b["total_text_requested_bytes"],
                  "D": [b["D_diagnostics"], d["D_diagnostics"]], "functions_changed": len(rows)}))
for r in rows[:top]:
    print(f"{r[0]:>14,} D{r[1]:>+7} M {r[3]}->{r[4]}  {r[2]}")
