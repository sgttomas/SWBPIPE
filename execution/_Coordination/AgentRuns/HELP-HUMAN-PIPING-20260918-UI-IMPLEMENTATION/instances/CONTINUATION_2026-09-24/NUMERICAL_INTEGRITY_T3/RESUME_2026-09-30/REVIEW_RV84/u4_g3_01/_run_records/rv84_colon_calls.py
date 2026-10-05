"""RV84: calls written immediately after a single ':' (compact json!/struct-literal syntax,
e.g. `"k":f(x)` or `Field{a:f(x)}`) are excluded by callgraph.py's CALL lookbehind. Find them in
reachable fn bodies and report which caller->callee edges are absent."""
import json, re, os, sys, collections
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), 'rv84_augment_scc.py')).read().split("METHOD = re.compile")[0])
I65 = sys.argv[1]
tb = json.load(open(os.path.join(I65, "text_budget.caps.out.json")))
fm = tb["function_multiplicity"]
oe = cg["edges"]
roots = [k for k in nodes if k.rsplit(":", 1)[1] in ("run_linear_static_preview_value_with_retained_direct", "prepare_observed")]
reach, todo = set(roots), list(roots)
while todo:
    v = todo.pop()
    for w in oe.get(v, []):
        if w not in reach: reach.add(w); todo.append(w)
COLON = re.compile(r"(?<!:):\s{0}([A-Za-z_][A-Za-z0-9_]*)\s*(?:::<[^>]*>)?\s*\(")
missing = []
for k in sorted(reach):
    try: bd = body(k)
    except Exception: continue
    have = {t.rsplit(":", 1)[1] for t in oe.get(k, [])}
    for m in COLON.finditer(bd):
        nm = m.group(1)
        if nm in byname and nm not in have:
            tg = [t for t in byname[nm] if crate[t] in allowed.get(crate[k], {crate[k]})]
            if tg:
                missing.append({"caller": k, "name": nm, "targets": tg, "M_caller": fm.get(k),
                                "targets_reached": [t for t in tg if t in reach]})
json.dump(missing, open(os.path.join(os.path.dirname(os.path.abspath(__file__)), 'rv84_colon_calls.out.json'), "w"), indent=1)
print(len(missing), "colon-call edges absent")
c = collections.Counter((m["caller"].rsplit("/", 1)[1], m["name"]) for m in missing)
for (cl, nm), v in c.most_common(60): print(v, cl, "->", nm)
newt = {t for m in missing for t in m["targets"] if t not in reach}
print("targets not otherwise reached:", len(newt)); [print("  ", t) for t in sorted(newt)]
