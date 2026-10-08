"""RV108: TS base against head, per probe and entry point."""
import json, sys
from collections import Counter
a = {json.loads(l)["id"]: json.loads(l) for l in open(sys.argv[1])}
b = {json.loads(l)["id"]: json.loads(l) for l in open(sys.argv[2])}
assert list(a) == list(b), "probe lists differ"
EPS = ("raw_inv", "raw_none", "transport", "c_raw", "c_transport")
admitted = Counter(); changed = Counter(); kinds = Counter(); bad = []
for pid in a:
    for ep in EPS:
        x, y = a[pid][ep], b[pid][ep]
        if x.get("ok"): admitted[ep] += 1
        if x == y: continue
        changed[ep] += 1
        if x.get("ok") or y.get("ok") or ep not in ("raw_inv", "raw_none") or x.get("gate") != "G7" or y.get("gate") != "G7" or x.get("code") != "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED":
            bad.append((pid, ep, x, y))
        kinds[(ep, x.get("code"), y.get("code"))] += 1
print("probes", len(a), "admitted at base by entry point", dict(admitted))
print("changed by entry point", dict(changed))
for k, n in sorted(kinds.items(), key=str): print(" ", k, n)
print("changes outside 'G7 SOURCE_PRODUCER_CONTRACT_UNSUPPORTED -> G7 <header code>' on the raw entry points:", len(bad))
for x in bad[:50]: print("  BAD", x)
