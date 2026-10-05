import json, re, sys, os, collections
I65 = sys.argv[1]
tb = json.load(open(os.path.join(I65, "text_budget.caps.out.json")))
LB = json.load(open(os.path.join(I65, "loop_bounds.json")))
rules = [(re.compile(r["re"]), r["bound"], r["why"], r["re"]) for r in LB["loops"]]
def which(h):
    if re.match(r"for\s.*\bin\s*\[", h): return ("array", None)
    for rx, b, why, src in rules:
        if rx.search(h): return (b, src)
    return (None, None)
zero = collections.defaultdict(list)
for r in tb["rows"]:
    if not r["reached"]: continue
    for h in r["loops"]:
        b, src = which(h)
        if b == "0":
            zero[(h, src)].append(f'{r["file"].split("/")[-1]}:{r["line"]} {r["kind"]} fnM={tb["function_multiplicity"].get(r["fn"])}')
# also loop headers on call edges (site_loops) zeroed
cg = json.load(open(os.path.join(I65, "callgraph_edges.json")))
fm = tb["function_multiplicity"]
ez = collections.defaultdict(list)
for a, b_, stacks in cg["site_loops"]:
    if a not in fm: continue
    for st in stacks:
        for h in st:
            b, src = which(h)
            if b == "0":
                ez[(h, src)].append(a.rsplit("/", 1)[1] + " -> " + b_.rsplit("/", 1)[1])
print("text-site loops zeroed:", len(zero))
for (h, src), v in sorted(zero.items()):
    print(" H:", h[:100], "| RULE:", src[:40], "|", v[:2])
print("call-site loops zeroed:", len(ez))
for (h, src), v in sorted(ez.items()):
    print(" H:", h[:100], "| RULE:", src[:40], "|", v[:2])
