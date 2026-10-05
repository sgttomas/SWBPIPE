#!/usr/bin/env python3
"""I67 U6d: compare the base and candidate sweep outputs (zzI67Sweep.test.ts).
Usage: compare_sweep.py <sweep_base> <sweep_cand> (prefixes of .tsv/.jsonl)."""
import json, sys
SUCCESSOR = "openpipestress.result_semantics/0.3.0/preview-physics-retained-1"
def load(prefix):
    rows = {}
    for line in open(prefix + ".jsonl"):
        r = json.loads(line); rows[r["key"]] = r
    return rows
b, c = load(sys.argv[1]), load(sys.argv[2])
assert set(b) == set(c), (len(b), len(c))
same_existing = diff_existing = 0
succ = {"same": 0, "diff": 0}
out = []
for k in sorted(b):
    x, y = b[k], c[k]
    eq = json.dumps(x["o"], sort_keys=True) == json.dumps(y["o"], sort_keys=True)
    if x["id"] == SUCCESSOR:
        succ["same" if eq else "diff"] += 1
        continue
    if eq: same_existing += 1
    else:
        diff_existing += 1
        fields = [f for f in x["o"] if json.dumps(x["o"][f], sort_keys=True) != json.dumps(y["o"].get(f), sort_keys=True)]
        out.append(f"  DIFF {k} ({x['id']}): fields {fields}")
ids = {}
for r in b.values(): ids[str(r["id"])] = ids.get(str(r["id"]), 0) + 1
print(f"envelopes: {len(b)}; by producer identity: {json.dumps(ids, sort_keys=True)}")
print(f"existing identities: {same_existing} identical, {diff_existing} different")
print("\n".join(out))
print(f"successor identity: {succ['same']} identical, {succ['diff']} different (expected to change)")
# Successor outcome summary in the candidate.
routes = {}
for k, r in c.items():
    if r["id"] == SUCCESSOR:
        o = r["o"]; key = (o["route"], json.dumps(o["standing_null"]["findings"]) if isinstance(o["standing_null"], dict) else o["standing_null"], o["refusal"] is not None, str(o["v03"])[:60])
        routes[key] = routes.get(key, 0) + 1
for k, n in sorted(routes.items(), key=lambda kv: -kv[1]): print(f"  cand successor: {n} x route={k[0]} findings={k[1]} refused_output={k[2]} v03={k[3]}")
