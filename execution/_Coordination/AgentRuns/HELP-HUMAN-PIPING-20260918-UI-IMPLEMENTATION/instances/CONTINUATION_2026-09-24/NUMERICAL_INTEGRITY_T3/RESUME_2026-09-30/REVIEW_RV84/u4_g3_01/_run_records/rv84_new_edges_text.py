import json, sys, collections, re, os
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), 'rv84_augment_scc.py')).read().split("# Tarjan on reach")[0])
I65 = sys.argv[1]
tb = json.load(open(os.path.join(I65, "text_budget.caps.out.json")))
fm = tb["function_multiplicity"]   # ancestors of text sites with M
oe = {k: set(v) for k, v in cg["edges"].items()}
hits = []
for k in reach:
    for t in edges[k] - oe.get(k, set()):
        if t in fm and fm[t] > 0:
            hits.append((k, t, fm.get(k)))
print(len(hits))
for h in sorted(hits)[:40]: print(h[0].rsplit('/',1)[1], '->', h[1].rsplit('/',1)[1], 'M(caller)=', h[2])
