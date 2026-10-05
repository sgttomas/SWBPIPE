"""RV87 (u4_g7_01): an independent strongly-connected-component check of a TEXT call graph.
For the graph reached from the Direct root: every multi-function SCC, with and without the
edge_zero edges, and whether its members are ancestors of a text site (inventory or lexicon).
Iterative Tarjan, own code. Usage: python3 rv87_scc.py <edges json> <loop_bounds json>
<inventory json> <lexicon json> [root]   (stdlib only)"""
import json, sys, collections
E = json.load(open(sys.argv[1]))["edges"]
LB = json.load(open(sys.argv[2]))
INV = json.load(open(sys.argv[3]))["rows"]
LEX = json.load(open(sys.argv[4]))["rows"]
root = sys.argv[5] if len(sys.argv) > 5 else "run_linear_static_preview_value_with_retained_direct"
short = lambda k: k.split("/src/")[-1]
EZ = {(e["caller"], e["callee"]) for e in LB.get("edge_zero", [])}
roots = [k for k in E if k.rsplit(":", 1)[1] == root]
reach, todo = set(roots), list(roots)
while todo:
    v = todo.pop()
    for w in E.get(v, []):
        if w not in reach:
            reach.add(w); todo.append(w)
# function spans for site attribution: keys are file:line:name; a site belongs to the reached fn
# with the largest start line <= site line in that file (as the run's fn_of does approximately)
byfile = collections.defaultdict(list)
for k in reach:
    f, l, n = k.rsplit(":", 2); byfile[f].append((int(l), k))
for f in byfile: byfile[f].sort()
site_fns = set(r["fn"] for r in LEX if r.get("fn") in reach)
import bisect
for r in INV:
    lst = byfile.get(r["file"], [])
    i = bisect.bisect_right([a for a, _ in lst], r["line"]) - 1
    if i >= 0: site_fns.add(lst[i][1])
rev = collections.defaultdict(set)
for v in reach:
    for w in E.get(v, []):
        if w in reach: rev[w].add(v)
anc, todo = set(site_fns), list(site_fns)
while todo:
    v = todo.pop()
    for u in rev[v]:
        if u not in anc: anc.add(u); todo.append(u)
def tarjan(skip):
    def adj(v): return [w for w in E.get(v, []) if w in reach and not (skip and (short(v), short(w)) in EZ)]
    idx, low, on, st, out, c = {}, {}, set(), [], [], 0
    for v0 in sorted(reach):
        if v0 in idx: continue
        idx[v0] = low[v0] = c; c += 1; st.append(v0); on.add(v0); work = [(v0, iter(adj(v0)))]
        while work:
            v, it = work[-1]; pushed = False
            for w in it:
                if w not in idx:
                    idx[w] = low[w] = c; c += 1; st.append(w); on.add(w); work.append((w, iter(adj(w)))); pushed = True; break
                if w in on: low[v] = min(low[v], idx[w])
            if pushed: continue
            work.pop()
            if work: low[work[-1][0]] = min(low[work[-1][0]], low[v])
            if low[v] == idx[v]:
                comp = []
                while True:
                    w = st.pop(); on.discard(w); comp.append(w)
                    if w == v: break
                if len(comp) > 1: out.append(sorted(short(x) for x in comp))
    selfl = sorted(short(v) for v in reach if v in adj(v))
    return out, selfl
res = {"reached": len(reach), "ancestors_of_text": len(anc)}
for skip in (False, True):
    m, s = tarjan(skip)
    res["with_edge_zero_cut" if skip else "lexical"] = {"multi_function_sccs": m,
        "multi_function_sccs_among_text_ancestors": [c for c in m if any(("core/" + x) in anc or any(a.endswith("/src/" + x) for a in anc) for x in c)],
        "self_loops": s, "self_loops_among_text_ancestors": [x for x in s if any(a.endswith("/src/" + x) for a in anc)]}
print(json.dumps(res, indent=1))
