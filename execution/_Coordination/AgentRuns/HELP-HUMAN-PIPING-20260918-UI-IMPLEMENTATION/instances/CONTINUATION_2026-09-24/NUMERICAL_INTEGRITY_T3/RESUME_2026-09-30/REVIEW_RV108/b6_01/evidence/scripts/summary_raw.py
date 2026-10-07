"""RV108: raw-path agreement summary (Python head, Rust, TS head, TS base) over a probe output set."""
import json, sys
from collections import Counter
def load(p): return {json.loads(l)["id"]: json.loads(l) for l in open(p)}
py, rs, ts, tb = (load(p) for p in sys.argv[1:5])
def n(x):
    if x.get("ok"): return "ok"
    return f'{x["gate"]} {x["code"]}' if "gate" in x else "err"
for ep in ("raw_inv", "raw_none"):
    head = Counter(); base = Counter(); cls = Counter()
    for pid in py:
        p, r, t, b = n(py[pid][ep]), n(rs[pid][ep]), n(ts[pid][ep]), n(tb[pid][ep])
        head[p == r == t] += 1; base[p == r == b] += 1
        if not (p == r == t): cls[(p, r, t)] += 1
    print(f"{ep}: probes {len(py)}; all three agree at head {head[True]}, at base {base[True]}; disagreements at head {head[False]}")
    for k, v in cls.most_common(): print("   ", v, "x  Python", k[0], "| Rust", k[1], "| TS", k[2])
