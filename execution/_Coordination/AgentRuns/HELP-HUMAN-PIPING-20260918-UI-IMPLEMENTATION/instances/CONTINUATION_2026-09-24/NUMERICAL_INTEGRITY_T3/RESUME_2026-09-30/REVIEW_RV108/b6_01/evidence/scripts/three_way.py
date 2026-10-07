"""RV108: three-way comparison (Python, Rust, TS) per probe and entry point."""
import json, sys
from collections import Counter, defaultdict
def load(p): return {json.loads(l)["id"]: json.loads(l) for l in open(p)}
py, rs, ts = load(sys.argv[1]), load(sys.argv[2]), load(sys.argv[3])
tsb = load(sys.argv[4]) if len(sys.argv) > 4 else None
def norm(x, lang, ep):
    if "absent" in x: return "absent"
    if x.get("ok"):
        return "ok" + ("/elig" if x.get("eligible") else "")
    if "gate" in x: return f'{x["gate"]} {x["code"]}'
    if "error" in x:
        e = x["error"]; import re; m = re.match(r"[A-Z][A-Z0-9_]*", e); return "err " + (m.group(0) if m else e[:60])
    return json.dumps(x)[:80]
EPS = ("raw_inv", "raw_none", "transport", "c_transport")
groups = defaultdict(list)
for pid in py:
    fam = py[pid]["family"]
    for ep in EPS:
        p, r, t = norm(py[pid][ep], "py", ep), norm(rs[pid][ep], "rs", ep), norm(ts[pid][ep], "ts", ep)
        tb = norm(tsb[pid][ep], "ts", ep) if tsb else None
        key = (ep, p, r, t) if tb is None else (ep, p, r, t, tb)
        groups[key].append(pid)
for ep in EPS:
    print(f"=== {ep}: (python | rust | ts head{' | ts base' if tsb else ''}) -> count, example")
    for key, ids in sorted(((k, v) for k, v in groups.items() if k[0] == ep), key=lambda kv: -len(kv[1])):
        agree = len(set(key[1:4])) == 1
        print(("   " if agree else "!! ") + " | ".join(key[1:]), "->", len(ids), "e.g.", ids[0])
