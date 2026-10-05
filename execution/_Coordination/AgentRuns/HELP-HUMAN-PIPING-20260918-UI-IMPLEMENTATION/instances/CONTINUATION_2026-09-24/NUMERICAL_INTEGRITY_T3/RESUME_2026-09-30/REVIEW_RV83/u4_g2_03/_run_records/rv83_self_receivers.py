"""RV83 probe: method calls on receivers typed `Self` (a local bound to `Self {..}` /
`Self::..`, or a parameter of type Self/&Self/&mut Self), and calls inside index brackets
(`a[b.m(..)]`), that produce no edge on I65's G4 graph. For each, the same-file
definitions of that method (the impl's own methods) are the likely true targets; report
whether they are reachable and the text below them (I65's own row `bytes` and `req`).
Run from the snapshot's projects/chirality-piping.
Usage: python3 rv83_self_receivers.py <zero_edge out json> <G4 _run_records> <out json>
"""
import json, re, sys, collections
zp, g4r, out_p = sys.argv[1:4]
z = json.load(open(zp))
cg = json.load(open(g4r + "/callgraph_edges.json")); E, S = cg["edges"], cg["spans"]
tb = json.load(open(g4r + "/text_budget.caps.out.json"))
reach, todo = set(tb["roots"]), list(tb["roots"])
while todo:
    v = todo.pop()
    for w in E.get(v, []):
        if w not in reach:
            reach.add(w); todo.append(w)
bytes_of, req_of = collections.Counter(), collections.Counter()
for r in tb["rows"]:
    if r.get("fn"):
        bytes_of[r["fn"]] += r.get("bytes", 0) or 0; req_of[r["fn"]] += r.get("req", 0) or 0
def subtree(starts):
    seen, todo = set(), list(starts)
    while todo:
        v = todo.pop()
        if v in seen: continue
        seen.add(v); todo.extend(E.get(v, []))
    return seen
src = {}
out = []
for r in z["refined_innermost"]:
    if r["form"] != "method" or not r["innermost_reachable"]:
        continue
    rel = r["line"].rsplit(":", 1)[0]
    if rel not in src: src[rel] = open(rel, encoding="utf-8").read()
    t = src[rel]; a, b = S[r["innermost"]][1], S[r["innermost"]][2]
    head = t[t.rfind("fn ", 0, a):a]; body = t[a:b]
    pre = t[:r["pos"]].rstrip()[:-1].rstrip()          # text before the '.'
    m = re.search(r"([a-z_][a-z0-9_]*)$", pre)
    recv = m.group(1) if m else None
    kind = None
    if recv and re.search(r"\blet\s+(?:mut\s+)?" + recv + r"\s*=\s*Self\b", body): kind = "local = Self"
    elif recv and re.search(r"[(,]\s*(?:mut\s+)?" + recv + r"\s*:\s*&?\s*(?:mut\s+)?Self\b", head): kind = "param: Self"
    elif re.search(r"\[[^\[\]]*$", t[t.rfind("\n", 0, r["pos"]):r["pos"]]): kind = "inside [..]"
    if not kind: continue
    defs = [k for k in E if k.startswith(rel + ":") and k.rsplit(":", 1)[1] == r["name"]]
    sub = subtree(defs)
    out.append({"line": r["line"], "call": r["name"], "receiver": recv, "kind": kind, "same_file_defs": defs,
                "defs_reachable": [d in reach for d in defs],
                "text_bytes_per_exec_below": sum(bytes_of[f] for f in sub), "text_req_below": sum(req_of[f] for f in sub)})
json.dump({"tokens": out}, open(out_p, "w"), indent=1)
print(len(out), "tokens;", collections.Counter(o["kind"] for o in out))
for o in out:
    print(o["kind"], o["line"].split("src/")[-1], o["receiver"], "." + o["call"], o["defs_reachable"], "text/exec", o["text_bytes_per_exec_below"], "req", o["text_req_below"])

# Part 2: would any dropped (caller -> same-file target) edge close a cycle? A cycle exists
# iff the caller is reachable from the target on the G4 graph (or caller == target).
cyc = []
for o in out:
    for d in o["same_file_defs"]:
        caller = next(r["innermost"] for r in z["refined_innermost"] if r["line"] == o["line"] and r["name"] == o["call"])
        if caller == d or caller in subtree([d]):
            cyc.append({"caller": caller, "target": d})
res = json.load(open(out_p)); res["cycles_closed_by_dropped_edges"] = cyc
json.dump(res, open(out_p, "w"), indent=1)
print("cycles closed by dropped Self/bracket edges:", cyc)
