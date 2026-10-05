"""RV83: text impact of the zero-edge call tokens (rv83_zero_edge_calls.out.json).
For each distinct called name among tokens whose innermost reachable caller lacks any
edge to that name, sum I65's own per-row text `req` in the subtree of (a) the same-named
definitions NOT reachable on the G4 graph (text that is missing outright) and (b) the
same-named definitions that ARE reachable (text whose multiplicity may miss this caller).
A name with zero text in both subtrees cannot change TEXT, whatever the receiver type.
Usage: python3 rv83_zero_edge_impact.py <zero_edge out json> <G4 _run_records> <out json>
"""
import json, sys, collections
zp, g4r, out_p = sys.argv[1:4]
z = json.load(open(zp))
E = json.load(open(g4r + "/callgraph_edges.json"))["edges"]
tb = json.load(open(g4r + "/text_budget.caps.out.json"))
reach, todo = set(tb["roots"]), list(tb["roots"])
while todo:
    v = todo.pop()
    for w in E.get(v, []):
        if w not in reach:
            reach.add(w); todo.append(w)
by = collections.defaultdict(list)
for k in E:
    by[k.rsplit(":", 1)[1]].append(k)
req_of = collections.Counter()
for r in tb["rows"]:
    if r.get("fn"):
        req_of[r["fn"]] += r.get("req", 0) or 0
# a row's req is 0 when its function has multiplicity 0; use bytes as a per-execution size instead
bytes_of = collections.Counter()
for r in tb["rows"]:
    if r.get("fn"):
        bytes_of[r["fn"]] += r.get("bytes", 0) or 0
def subtree(starts):
    seen, todo = set(), list(starts)
    while todo:
        v = todo.pop()
        if v in seen:
            continue
        seen.add(v); todo.extend(E.get(v, []))
    return seen
names = collections.Counter(r["name"] for r in z["refined_innermost"] if r["innermost_reachable"])
rows = []
for n, c in names.items():
    defs = by.get(n, [])
    un = [d for d in defs if d not in reach]
    re_ = [d for d in defs if d in reach]
    su, sr = subtree(un), subtree(re_)
    rows.append({"name": n, "tokens": c, "unreached_defs": un, "reached_defs": re_,
                 "text_bytes_per_exec_under_unreached": sum(bytes_of[f] for f in su - reach),
                 "text_req_under_reached": sum(req_of[f] for f in sr)})
rows.sort(key=lambda r: -(r["text_bytes_per_exec_under_unreached"] + r["text_req_under_reached"]))
json.dump({"names": rows}, open(out_p, "w"), indent=1)
for r in rows:
    if r["text_bytes_per_exec_under_unreached"] or r["text_req_under_reached"]:
        print(r["name"], r["tokens"], "unreached-text/exec", r["text_bytes_per_exec_under_unreached"], "reached-text req", r["text_req_under_reached"],
              [d.split("src/")[-1] for d in r["unreached_defs"]][:3], [d.split("src/")[-1] for d in r["reached_defs"]][:3])
