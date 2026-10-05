"""RV91: compare base and candidate sweep outcomes, per envelope and field."""
import json, sys, collections
S = sys.argv[1]
def load(p): return {json.loads(l)["key"]: json.loads(l)["o"] for l in open(p)}
b, c = load(f"{S}/sweep/sweep_base.jsonl"), load(f"{S}/sweep/sweep_cand.jsonl")
assert set(b) == set(c), (set(b) ^ set(c))
succ = lambda o: o["identity"] == "openpipestress.result_semantics/0.3.0/preview-physics-retained-1"
lines = []
ids = collections.Counter()
diff_existing = collections.Counter(); diff_succ = collections.Counter(); probe = collections.Counter()
for k in sorted(b):
    ob, oc = b[k], c[k]
    ids[(ob["identity"] or "none").split("/")[-1]] += 1
    fields = [f for f in ob if f != "probe_member" and ob[f] != oc[f]]
    if succ(ob):
        for f in fields: diff_succ[f] += 1
    else:
        for f in fields:
            diff_existing[f] += 1; lines.append(f"EXISTING DIFF {k} {f}: base={json.dumps(ob[f])[:200]} cand={json.dumps(oc[f])[:200]}")
        probe[(json.dumps(ob["probe_member"]["route"]), json.dumps(oc["probe_member"]["route"]), json.dumps(oc["probe_member"]["standing"]))] += 1
existing = sum(1 for k in b if not succ(b[k]))
print(f"envelopes={len(b)} existing={existing} successor={len(b)-existing}")
print("identities:", dict(ids))
print("fields compared per envelope:", len([f for f in next(iter(b.values())) if f != 'probe_member']))
print("existing-identity field differences:", dict(diff_existing) or "NONE")
print("successor field differences (intended):", dict(diff_succ))
print("probe (existing + retained_precision member) base route -> cand route, cand findings:")
for k, v in probe.items(): print("  ", k, v)
for l in lines: print(l)
