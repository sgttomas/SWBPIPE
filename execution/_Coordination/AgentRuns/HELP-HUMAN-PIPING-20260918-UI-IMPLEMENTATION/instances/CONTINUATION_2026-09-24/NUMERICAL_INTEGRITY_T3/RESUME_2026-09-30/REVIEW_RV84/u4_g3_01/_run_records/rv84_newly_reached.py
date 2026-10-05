import json, sys, collections, re, os
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), 'rv84_augment_scc.py')).read().split("# Tarjan on reach")[0])
I65 = sys.argv[1]
tb = json.load(open(os.path.join(I65, "text_budget.caps.out.json")))
# original reach: BFS on original edges
oe = cg["edges"]
r0, todo = set(roots), list(roots)
while todo:
    v = todo.pop()
    for w in oe.get(v, []):
        if w not in r0: r0.add(w); todo.append(w)
newr = reach - r0
rows = [r for r in tb["rows"] if r["fn"] in newr]
print("orig reach", len(r0), "aug reach", len(reach), "new", len(newr), "text rows in new fns", len(rows))
byfile = collections.Counter(r["file"] for r in rows)
print(byfile.most_common(20))
excluded = set()
lb = json.load(open(os.path.join(I65, "loop_bounds.json")))
json.dump({"new_fns": sorted(newr), "text_rows": rows}, open(os.path.join(os.path.dirname(os.path.abspath(__file__)), 'rv84_newly_reached.out.json'), "w"), indent=1)
