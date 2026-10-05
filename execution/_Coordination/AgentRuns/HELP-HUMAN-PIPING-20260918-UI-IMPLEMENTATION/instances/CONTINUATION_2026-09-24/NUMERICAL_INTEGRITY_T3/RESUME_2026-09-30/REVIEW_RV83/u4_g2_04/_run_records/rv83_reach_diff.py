"""RV83: reachable sets from the Direct root on G4's graph and on the R-4 graph; edge diff.
Usage: python3 rv83_reach_diff.py <G4 callgraph_edges.json> <G4 text_budget (roots)> <edges_r4.json> <out json>"""
import json, sys
a_e, a_t, b_e, out_p = sys.argv[1:5]
A = json.load(open(a_e))["edges"]; B = json.load(open(b_e))["edges"]
roots = json.load(open(a_t))["roots"]
def reach(E):
    s, t = set(roots), list(roots)
    while t:
        v = t.pop()
        for w in E.get(v, []):
            if w not in s: s.add(w); t.append(w)
    return s
ra, rb = reach(A), reach(B)
ea = {(k, w) for k, v in A.items() for w in v}; eb = {(k, w) for k, v in B.items() for w in v}
out = {"reach_g4": len(ra), "reach_r4": len(rb), "gained": sorted(rb - ra), "lost": sorted(ra - rb),
       "edges_g4": len(ea), "edges_r4": len(eb), "edges_added": len(eb - ea), "edges_removed": len(ea - eb),
       "removed": sorted(ea - eb)}
json.dump(out, open(out_p, "w"), indent=1)
print(out["reach_g4"], out["reach_r4"], "gained", [g.split("src/")[-1] for g in out["gained"]], "lost", out["lost"])
print("edges", out["edges_g4"], out["edges_r4"], "+", out["edges_added"], "-", out["edges_removed"])
for k, w in out["removed"]:
    print("   removed", k.split("src/")[-1], "->", w.split("src/")[-1])
