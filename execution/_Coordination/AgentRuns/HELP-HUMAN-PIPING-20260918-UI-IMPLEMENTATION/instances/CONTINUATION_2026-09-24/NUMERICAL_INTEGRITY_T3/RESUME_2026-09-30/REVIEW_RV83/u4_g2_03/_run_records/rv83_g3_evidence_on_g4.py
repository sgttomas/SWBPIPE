"""RV83: is every function my G3 probes found hidden now reached on I65's G4 graph?
Read-only, stdlib. Matches G3 keys (file:line:name at 5ae5fe4f0f) to G4 keys
(file:line:name at b1f80234dc) by (file, name); a (file, name) counts as reached if any
G4 node with that file and name is reachable from the G4 root.
Usage: python3 rv83_g3_evidence_on_g4.py <RV83 u4_g2_02 _run_records> <G4 _run_records> <out json>
"""
import json, sys, collections
g3r, g4r, out_p = sys.argv[1:4]
me = json.load(open(g3r + "/rv83_missed_edges.out.json"))
cc = json.load(open(g3r + "/rv83_chained_calls.out.json"))
g4 = json.load(open(g4r + "/callgraph_edges.json"))["edges"]
tb = json.load(open(g4r + "/text_budget.caps.out.json"))
reach, todo = set(tb["roots"]), list(tb["roots"])
while todo:
    v = todo.pop()
    for w in g4.get(v, []):
        if w not in reach:
            reach.add(w); todo.append(w)
def fn(k):
    f, _, n = k.rsplit(":", 2)
    return (f, n)
g4_by = collections.defaultdict(list)
for k in g4:
    g4_by[fn(k)].append(k)
def status(keys):
    res = {}
    for k in sorted(set(keys)):
        cands = g4_by.get(fn(k), [])
        res[k] = ("reached" if any(c in reach for c in cands) else ("node, unreached" if cands else "no G4 node"))
    return res
generic = status(me["functions_hidden_behind_them"])
chained_defs = status(d for v in cc["chained_call_names_with_unreached_defs"].values() for d in v["unreached_defs"])
chained_closure = status(cc.get("hidden_closure", []))
summ = lambda s: collections.Counter(s.values())
json.dump({"g4_reachable": len(reach), "generic_hidden": generic, "generic_summary": summ(generic),
           "chained_unreached_defs": chained_defs, "chained_summary": summ(chained_defs)}, open(out_p, "w"), indent=1)
print("G4 reachable", len(reach)); print("generic", summ(generic)); print("chained defs", summ(chained_defs))
for k, v in chained_defs.items():
    if v != "reached":
        print("  ", v, k)
for k, v in generic.items():
    if v != "reached":
        print("  generic:", v, k)
