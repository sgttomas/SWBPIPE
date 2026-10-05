"""RV87 (u4_g6_02): row-by-row diff of the G6 and repaired (g6r) TEXT runs, whole/W/X; and the
text_args.g4.json key changes (site rules, id_audit) between G6 and g6r.
Usage: python3 rv87_g6r_rowdiff.py <G6 _run_records> <g6r _run_records_g6r>   (stdlib only)"""
import json, os, sys, collections
G, R = sys.argv[1], sys.argv[2]
out = {}
for v in ("", "_W", "_X"):
    a = json.load(open(os.path.join(G, "text_g6", f"text_budget{v}.caps.out.json")))
    b = json.load(open(os.path.join(R, "text_g6r", f"text_budget{v}.caps.out.json")))
    def idx(rows):
        d = collections.defaultdict(list)
        for r in rows:
            d[(r["file"], r["line"], r["kind"])].append(r)
        return d
    A, B = idx(a["rows"]), idx(b["rows"])
    ch = []
    for k in sorted(set(A) | set(B)):
        ra, rb = A.get(k, []), B.get(k, [])
        for i in range(max(len(ra), len(rb))):
            x = ra[i] if i < len(ra) else None; y = rb[i] if i < len(rb) else None
            xa = (x["mult"], x["bytes"], x["req"]) if x else None; yb = (y["mult"], y["bytes"], y["req"]) if y else None
            if xa != yb:
                ch.append({"site": f"{k[0].split('core/')[-1]}:{k[1]}", "kind": k[2], "g6": xa, "g6r": yb,
                           "delta": (yb[2] if yb else 0) - (xa[2] if xa else 0)})
    out[v or "whole"] = {"g6": a["total_text_requested_bytes"], "g6r": b["total_text_requested_bytes"],
                         "delta": b["total_text_requested_bytes"] - a["total_text_requested_bytes"],
                         "complete": b["complete"], "D": [a["D_diagnostics"], b["D_diagnostics"]], "changed": ch}
ta = json.load(open(os.path.join(G, "text_args.g4.json"))); tb = json.load(open(os.path.join(R, "text_args.g4.json")))
keys = {}
for sect in sorted(set(ta) | set(tb)):
    x, y = ta.get(sect), tb.get(sect)
    if x == y:
        continue
    if isinstance(x, dict) and isinstance(y, dict):
        keys[sect] = {"removed": sorted(set(x) - set(y)), "added": sorted(set(y) - set(x)),
                      "changed": sorted(k for k in set(x) & set(y) if x[k] != y[k])[:200]}
    elif isinstance(x, list) and isinstance(y, list):
        keys[sect] = {"g6_len": len(x), "g6r_len": len(y), "added": [e for e in y if e not in x], "removed": [e for e in x if e not in y]}
    else:
        keys[sect] = {"g6": str(x)[:200], "g6r": str(y)[:200]}
out["text_args_changes"] = keys
out["id_audit_entries"] = [sum(len(v) for v in ta["id_audit"].values()), sum(len(v) for v in tb["id_audit"].values())]
print(json.dumps(out, indent=1))
