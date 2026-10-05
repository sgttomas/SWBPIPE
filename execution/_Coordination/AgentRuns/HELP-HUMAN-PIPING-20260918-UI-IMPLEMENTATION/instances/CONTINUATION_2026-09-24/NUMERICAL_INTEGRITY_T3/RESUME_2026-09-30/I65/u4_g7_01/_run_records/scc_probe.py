"""List the reached call graph's cycles: multi-member SCCs and self-loops, with/without edge_zero
and recursion_factor (stdlib only)."""
import json, sys, collections
edges_p, lb_p, root = sys.argv[1:4]
E = json.load(open(edges_p))["edges"]
LB = json.load(open(lb_p))
short = lambda k: k.split("/src/")[-1]
EZ = {(e["caller"], e["callee"]) for e in LB.get("edge_zero", [])}
RECUR = LB.get("recursion_factor", {})
roots = [k for k in E if k.endswith(":" + root)]
reach, todo = set(roots), list(roots)
while todo:
    v = todo.pop()
    for w in E.get(v, []):
        if w not in reach: reach.add(w); todo.append(w)
def sccs(skip):
    idx, low, on, st, out, c = {}, {}, set(), [], [], [0]
    def adj(v): return [w for w in E.get(v, []) if w in reach and not (skip and (short(v), short(w)) in EZ)]
    for v0 in sorted(reach):
        if v0 in idx: continue
        work = [(v0, iter(adj(v0)))]; idx[v0] = low[v0] = c[0]; c[0] += 1; st.append(v0); on.add(v0)
        while work:
            v, it = work[-1]; adv = False
            for w in it:
                if w not in idx:
                    idx[w] = low[w] = c[0]; c[0] += 1; st.append(w); on.add(w); work.append((w, iter(adj(w)))); adv = True; break
                elif w in on: low[v] = min(low[v], idx[w])
            if adv: continue
            work.pop()
            if work: low[work[-1][0]] = min(low[work[-1][0]], low[v])
            if low[v] == idx[v]:
                comp = []
                while True:
                    w = st.pop(); on.discard(w); comp.append(w)
                    if w == v: break
                out.append(comp)
    return out, adj
for skip in (False, True):
    comps, adj = sccs(skip)
    multi = [sorted(short(x) for x in c) for c in comps if len(c) > 1]
    selfl = sorted(short(v) for v in reach if v in adj(v))
    print(json.dumps({"edge_zero_applied": skip, "reached": len(reach), "multi_member_sccs": multi,
                      "self_loops": [(s, s.rsplit(":", 1)[1] in RECUR) for s in selfl]}, indent=1))
