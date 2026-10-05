"""RV87 (u4_g6_01): price the identifier-bearing copies the audit table misses (found by RV87's
by-type probe; each read by hand, listed below), in each G6 run (whole, W, X), at the run's
convention req = mult * max(8, 2 * size).
Usage: python3 rv87_class_probe_gaps.py <G6 _run_records>   (stdlib only)"""
import json, os, sys, collections
G = sys.argv[1]
P = "core/product_physics/src/"
# (site, kind, priced bytes of the identifier part now, bound by source, extra literal context ignored, why)
GAPS = [
    (P + "lib.rs", 1632, "format", 20, 131, "integrity_dof_label(model, dof): '{node.id}:{UX..RZ}' (lib.rs:1067-1072), node id <= 128; priced by the int rule (`_dof`) ahead of the composite_id rule that names it"),
    (P + "lib.rs", 1644, "format", 20, 131, "integrity_dof_label(model, dof)"),
    (P + "lib.rs", 1708, "format", 20, 131, "integrity_dof_label(model, record.global_dof)"),
    (P + "lib.rs", 1721, "format", 20, 131, "integrity_dof_label(model, *dof)"),
    (P + "lib.rs", 1739, "format", 20, 131, "integrity_dof_label(model, *dof)"),
    (P + "lib.rs", 1763, "format", 8241, 8352, "TPL entry bound built from :1699-1753, each low by 111 B (the label above)"),
    (P + "retained_product.rs", 3125, "push_str", 327, 1024, "adapter copy(s): out.push_str(s); callers copy r.id (:3640, a result id) and the basis text (:563); priced by the `^(v|s|t|..)` f64 rule; G4's lex_site_size for it is keyed at the b1f80234dc line 2963, stale at 1e323058f3"),
]
res = {}
for v in ("", "_W", "_X"):
    run = json.load(open(os.path.join(G, "text_g6", f"text_budget{v}.caps.out.json")))
    idx = collections.defaultdict(list)
    for r in run["rows"]:
        idx[(r["file"], r["line"], r["kind"])].append(r)
    rows, tot = [], 0
    for f, line, kind, now, bound, why in GAPS:
        rr = idx.get((f, line, kind), [])
        m = max([x["mult"] for x in rr] + [0])
        size = max([x["bytes"] for x in rr] + [0])
        # the identifier part moves from `now` to `bound` inside the site's size
        new = size - now + bound
        d = m * (max(8, 2 * new) - max(8, 2 * size)) if m else 0
        # adapter copy(): only the result-id calls (2) and the basis text (1) exceed 327 by source
        if line == 3125 and m:
            d = 2 * (2 * 1024 - 2 * 327) + 1 * (2 * 600 - 2 * 327)
        rows.append({"site": f"{f.split('core/')[-1]}:{line}", "kind": kind, "mult": m, "size_now": size, "size_at_bound": new, "delta": d, "why": why})
        tot += d
    res[v or "whole"] = {"rows": rows, "total": tot}
print(json.dumps(res, indent=1))
