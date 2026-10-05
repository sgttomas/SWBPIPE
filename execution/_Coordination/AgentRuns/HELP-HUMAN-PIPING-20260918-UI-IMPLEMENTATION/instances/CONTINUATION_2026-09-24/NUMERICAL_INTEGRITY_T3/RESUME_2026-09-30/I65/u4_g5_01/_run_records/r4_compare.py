"""I65 U4 G5, RV83 R-4: compare the R-4 graph and text run with G4's (stdlib only).
Usage: python3 r4_compare.py <G4 _run_records> <R-4 scratch dir> > r4_compare.out.json"""
import json, os, sys
G4, O = sys.argv[1:3]
def load(*p):
    return json.load(open(os.path.join(*p)))
def edges(p):
    return {k: {x if isinstance(x, str) else x[0] for x in v} for k, v in load(p)["edges"].items()}
ea, eb = edges(os.path.join(G4, "callgraph_edges.json")), edges(os.path.join(O, "edges_r4.json"))
root = [k for k in ea if k.endswith(":run_linear_static_preview_value_with_retained_direct")]
def reach(e):
    seen, st = set(root), list(root)
    while st:
        for v in e.get(st.pop(), ()):
            if v not in seen:
                seen.add(v); st.append(v)
    return seen
ra, rb = reach(ea), reach(eb)
ga, gb = load(G4, "callgraph.out.json"), load(O, "cg_r4.out.json")
ia, ib = load(G4, "callgraph_implicit.out.json"), load(O, "cg_r4_implicit.out.json")
ta, tb = load(G4, "text_budget.caps.l128.out.json"), load(O, "sens_r4", "text_budget.caps.out.json")
rows = lambda t: {(r["file"], r["line"], r["kind"]): (r["mult"], r["req"]) for r in t["rows"]}
fa, fb = ta["function_multiplicity"], tb["function_multiplicity"]
added = sorted((u, v) for u in eb for v in eb[u] - ea.get(u, set()))
removed = sorted((u, v) for u in ea for v in ea[u] - eb.get(u, set()))
aud = load(O, "audit_r4.json")
out = {
    "graph": {"nodes": [ga["nodes"], gb["nodes"]], "edges": [ga["edges"], gb["edges"]], "edges_added": len(added), "edges_removed": len(removed),
              "added": added, "removed": removed},
    "reachable_from_direct_root": [len(ra), len(rb)], "newly_reachable": sorted(rb - ra), "no_longer_reachable": sorted(ra - rb),
    "explicit_cycles": {"g4": len(ga["cyclic_components"]), "r4": len(gb["cyclic_components"]),
                        "r4_multi_node": [c for c in gb["cyclic_components"] if len(c) > 1],
                        "same_components": sorted(map(sorted, ga["cyclic_components"])) == sorted(map(sorted, gb["cyclic_components"]))},
    "implicit_candidates": {"g4": len(ia["cyclic_components"]), "r4": len(ib["cyclic_components"]),
                            "same_components": sorted(map(sorted, ia["cyclic_components"])) == sorted(map(sorted, ib["cyclic_components"]))},
    "text_l128": {"tav": [ta["total_text_requested_bytes"], tb["total_text_requested_bytes"]], "complete": [ta["complete"], tb["complete"]],
                  "rows_identical": rows(ta) == rows(tb), "reachable_fns": [ta["reachable_fns"], tb["reachable_fns"]],
                  "functions_added": sorted(set(fb) - set(fa)), "functions_removed": sorted(set(fa) - set(fb)),
                  "multiplicity_changed": sorted((k, fa[k], fb[k]) for k in fa if k in fb and fa[k] != fb[k])},
    "audit": {"unresolved_call_tokens": aud["unresolved_call_tokens"], "by_reason": aud["by_reason"],
              "self_typed_reasons_left": sum(1 for r in aud["rows"] if "Self" in r["reason"])},
}
print(json.dumps(out, indent=1))
