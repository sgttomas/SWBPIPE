"""RV83: do my u4_g2_03 R-4 tokens now produce edges on I65's R-4 graph?
For each token in u4_g2_03's rv83_self_receivers.out.json whose same-file target exists
(the 24 user-method calls: `local = Self`, `param: Self`, and self-calls inside brackets),
check the R-4 edge innermost-caller -> target, and the token's label in I65's audit_r4.json.
Usage: python3 rv83_r4_tokens.py <u4_g2_03 self_receivers json> <u4_g2_03 zero_edge json> <edges_r4.json> <audit_r4.json> <out json>
"""
import json, sys, collections
sp, zp, ep, ap, out_p = sys.argv[1:6]
toks = json.load(open(sp))["tokens"]
z = json.load(open(zp))["refined_innermost"]
E = json.load(open(ep))["edges"]
audit = json.load(open(ap))
arows = collections.defaultdict(list)
for r in audit["rows"]:
    arows[(r["line"], r["name"])].append(r["reason"])
res = []
for t in toks:
    if not t["same_file_defs"]:
        continue                      # std calls inside brackets (no user target)
    caller = next(r["innermost"] for r in z if r["line"] == t["line"] and r["name"] == t["call"])
    hit = [d for d in t["same_file_defs"] if d in E.get(caller, [])]
    res.append({"line": t["line"], "call": t["call"], "kind": t["kind"], "caller": caller,
                "edge_now": bool(hit), "targets_now": hit, "audit_r4_labels": arows.get((t["line"], t["call"]), [])})
json.dump({"tokens": res, "count": len(res), "with_edge": sum(r["edge_now"] for r in res)}, open(out_p, "w"), indent=1)
print(len(res), "user-method tokens;", sum(r["edge_now"] for r in res), "now have an edge")
for r in res:
    print(("OK " if r["edge_now"] else "MISSING ") + r["kind"], r["line"].split("src/")[-1], "." + r["call"], r["audit_r4_labels"])
print("audit reasons (top):", collections.Counter(x["reason"] for x in audit["rows"]).most_common(40))
