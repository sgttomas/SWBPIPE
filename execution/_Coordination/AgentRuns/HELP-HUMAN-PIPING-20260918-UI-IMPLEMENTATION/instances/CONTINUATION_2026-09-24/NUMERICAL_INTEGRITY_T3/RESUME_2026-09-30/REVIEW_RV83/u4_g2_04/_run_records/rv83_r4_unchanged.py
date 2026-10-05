"""RV83: is TEXT, the recursion inventory and the admission maximum unchanged under R-4?
Compares, row by row, I65's G4 l<=128 outputs with the R-4 outputs (same basis, same caps):
the four text runs (full, envelope, X, W), the g4_caps summaries, and the cyclic components
of the explicit and implicit call graphs. Read-only, stdlib.
Usage: python3 rv83_r4_unchanged.py <G4 _run_records> <G5 _run_records/r4> <out json>
"""
import json, sys
g4, r4, out_p = sys.argv[1:4]
out = {}
pairs = [("full", "text_budget.caps.l128.out.json", "text_budget.caps.r4.out.json"),
         ("env", "text_budget_env.caps.l128.out.json", "text_budget_env.caps.r4.out.json"),
         ("X", "text_budget_X.caps.l128.out.json", "text_budget_X.caps.r4.out.json"),
         ("W", "text_budget_W.caps.l128.out.json", "text_budget_W.caps.r4.out.json")]
for tag, a_f, b_f in pairs:
    a, b = json.load(open(f"{g4}/{a_f}")), json.load(open(f"{r4}/{b_f}"))
    key = lambda r: (r["file"], r["line"], r["kind"], r.get("fn"))
    ra = {key(r): r for r in a["rows"]}; rb = {key(r): r for r in b["rows"]}
    diff = []
    for k in sorted(set(ra) | set(rb), key=str):
        x, y = ra.get(k), rb.get(k)
        if x is None or y is None or x.get("req") != y.get("req") or x.get("mult") != y.get("mult") or x.get("reached") != y.get("reached"):
            diff.append({"site": k, "g4": None if x is None else {f: x.get(f) for f in ("reached", "mult", "bytes", "req")},
                         "r4": None if y is None else {f: y.get(f) for f in ("reached", "mult", "bytes", "req")}})
    fm_a, fm_b = a["function_multiplicity"], b["function_multiplicity"]
    out[tag] = {"total_g4": a["total_text_requested_bytes"], "total_r4": b["total_text_requested_bytes"],
                "moving_g4": a["total_text_moving_bytes"], "moving_r4": b["total_text_moving_bytes"],
                "D_g4": a["D_diagnostics"], "D_r4": b["D_diagnostics"], "complete": [a["complete"], b["complete"]],
                "rows_differing": diff,
                "fm_entries_added": sorted(set(fm_b) - set(fm_a)), "fm_entries_removed": sorted(set(fm_a) - set(fm_b)),
                "positive_sites": [a["sites_with_positive_multiplicity"], b["sites_with_positive_multiplicity"]]}
ca = json.load(open(f"{g4}/g4_caps.caps.eps2.l128.out.json")); cb = json.load(open(f"{r4}/g4_caps.caps.eps2.r4.out.json"))
out["g4_caps_identical"] = ca == cb
if not out["g4_caps_identical"]:
    out["g4_caps_diff_keys"] = [k for k in set(ca) | set(cb) if ca.get(k) != cb.get(k)]
ga, gb = json.load(open(f"{g4}/callgraph.out.json")), json.load(open(f"{r4}/cg_r4.out.json"))
ia, ib = json.load(open(f"{g4}/callgraph_implicit.out.json")), json.load(open(f"{r4}/cg_r4_implicit.out.json"))
norm = lambda cs: sorted(sorted(c) for c in cs)
out["explicit_cycles"] = {"g4": len(ga["cyclic_components"]), "r4": len(gb["cyclic_components"]),
                          "identical": norm(ga["cyclic_components"]) == norm(gb["cyclic_components"])}
out["implicit_cycles"] = {"g4": len(ia["cyclic_components"]), "r4": len(ib["cyclic_components"]),
                          "identical": norm(ia["cyclic_components"]) == norm(ib["cyclic_components"])}
out["root_depths"] = {"g4": ga.get("root_depths"), "r4": gb.get("root_depths")}
json.dump(out, open(out_p, "w"), indent=1)
for tag, *_ in pairs:
    o = out[tag]
    print(tag, o["total_g4"], o["total_r4"], "moving", o["moving_g4"], o["moving_r4"], "D", o["D_g4"], o["D_r4"],
          "rows differing", len(o["rows_differing"]), "fm +", len(o["fm_entries_added"]), "-", len(o["fm_entries_removed"]), "pos", o["positive_sites"])
    for d in o["rows_differing"][:6]:
        print("    ", d)
print("g4_caps identical:", out["g4_caps_identical"], out.get("g4_caps_diff_keys"))
print("explicit:", out["explicit_cycles"], "implicit:", out["implicit_cycles"], "depths:", out["root_depths"])
