"""RV84 confirmation: S-2. For the G4 text run, list (a) every reached text row with a zero
multiplicity and the loop headers that zeroed it, (b) the packet's own zero_matched_headers,
and (c) the rows at the six loops RV84 cited (re-pointed to b1f80234dc) with their multiplicity.
Usage: python3 rv84c_zero_rules.py <G4 _run_records>"""
import json, re, os, sys, collections
G4 = sys.argv[1]
tb = json.load(open(os.path.join(G4, "text_budget.caps.out.json")))
LB = json.load(open(os.path.join(G4, "loop_bounds.g4.json")))
fm = tb["function_multiplicity"]
rules = [(re.compile(r["re"]), r["bound"], r.get("why", ""), r["re"]) for r in LB["loops"]]
def which(h):
    if re.match(r"for\s.*\bin\s*\[", h): return ("array", None)
    for rx, b, why, src in rules:
        if rx.search(h): return (b, src)
    return (None, None)
zero_rows = collections.defaultdict(list)
for r in tb["rows"]:
    if not r["reached"] or not r["fn"] or not fm.get(r["fn"]): continue
    for h in r["loops"]:
        b, src = which(h)
        if b == "0":
            zero_rows[(h, src)].append(f'{r["file"].split("/")[-1]}:{r["line"]}')
cited = {("lib.rs", l) for l in (11156, 11246, 11321, 12609, 12702, 9986, 10109, 10012)} | {("retained_product.rs", 2162)}
cited_rows = []
for r in tb["rows"]:
    f = r["file"].split("/")[-1]
    if r["file"].startswith("core/product_physics") and r["loops"]:
        for h in r["loops"]:
            pass
    if (f, r["line"]) in cited or any(("local_components" in h or "boundaries.windows" in h or "zip(&intensity)" in h
                                       or re.search(r"\) in components\b", h) or "components.into_iter" in h) for h in r["loops"]):
        cited_rows.append({"site": f + ":" + str(r["line"]), "kind": r["kind"], "mult": r["mult"], "fnM": fm.get(r["fn"]), "loops": r["loops"]})
out = {"zero_matched_headers_packet": tb.get("zero_matched_headers"),
       "reached_rows_zeroed_by_rule": {f"{h} || RULE {src}": v for (h, src), v in sorted(zero_rows.items())},
       "cited_loop_rows": cited_rows}
json.dump(out, open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "rv84c_zero_rules.out.json"), "w"), indent=1)
print("packet zero_matched_headers:", len(tb.get("zero_matched_headers") or []))
print("reached rows zeroed (distinct headers):", len(zero_rows))
for (h, src), v in sorted(zero_rows.items()): print("  Z", h[:110], "| RULE", (src or "")[:45], "|", v[:2])
print("cited loop rows:")
for c in cited_rows: print("  ", c["site"], c["kind"], "mult", c["mult"], "fnM", c["fnM"], c["loops"][-1][:70])
