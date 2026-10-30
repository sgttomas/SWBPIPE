"""O-10: after a run, each O10_LOOPS_LAST rule (a header rule appended last) may have matched only the loops it was
written for. Reads the chain's SQ2_RULES.json (o10_loops_last) and a point's looplog.json; prints each rule's matched
functions and exits 3 if any match lies outside the rule's listed functions. Usage: o10_check.py <chain dir> <run dir>..."""
import json, os, re, sys
chain, runs = sys.argv[1], sys.argv[2:]
rules = json.load(open(os.path.join(chain, "SQ2_RULES.json"))).get("o10_loops_last", [])
bad, out = [], []
for run in runs:
    ll = json.load(open(os.path.join(run, "looplog.json")))
    for r in rules:
        fns = set()
        for ctx, h, b, v in ll:
            if h == r["header"]:
                m = re.match(r"(?:call|site) (\S+?)(?: ->| @)", str(ctx))
                fns.add(m.group(1) if m else str(ctx))
        # O-10 review: allowed functions are `file:fn` (the line dropped), never a bare name
        fk = lambda f: ":".join(f.split(":")[::2]) if f.count(":") == 2 else f
        stray = [f for f in fns if fk(f) not in r["fns"]]
        out.append({"run": os.path.basename(run), "header": r["header"], "bound": r["bound"], "matched_fns": sorted(fns), "stray": stray})
        if stray: bad.append(out[-1])
mult0 = json.load(open(os.path.join(chain, "SQ2_RULES.json"))).get("o10_mult0_sites", [])
for run in runs:
    rows = json.load(open(os.path.join(run, "work", "text_budget.caps.out.json")))["rows"]
    for k in mult0:
        hit = [r for r in rows if r["file"].endswith("/" + k.split(":")[0]) and r["line"] == int(k.split(":")[1])]
        pos = [r for r in hit if r.get("mult")]
        out.append({"run": os.path.basename(run), "mult0_site": k, "rows": len(hit), "positive": len(pos)})
        if pos or not hit: bad.append(out[-1])
print(json.dumps(out, indent=1))
if bad:
    print("O10 CHECK FAILED: " + json.dumps(bad)); sys.exit(3)
