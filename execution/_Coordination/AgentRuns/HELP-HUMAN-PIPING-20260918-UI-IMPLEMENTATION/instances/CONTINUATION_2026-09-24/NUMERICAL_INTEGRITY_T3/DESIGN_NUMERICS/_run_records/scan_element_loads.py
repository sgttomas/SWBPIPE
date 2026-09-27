"""Scan committed JSON (outside execution/) for load cases that put two or more
element-targeted primitive loads on one element: only such members can have a
recovery-side load sum (equivalents, stations, extrema w, curved intensity) with
three or more operands, where exact summation could change a result bit. Also
counts nodal loads per (case, node, direction) for completeness. Read-only."""
import json, os, sys, collections
root = sys.argv[1]
def models(o):
    if isinstance(o, dict):
        if isinstance(o.get("load_cases"), list):
            yield o
        for v in o.values():
            yield from models(v)
    elif isinstance(o, list):
        for v in o:
            yield from models(v)
cases = 0; hits = []; maxe = 0; files = 0
for d, _, fs in os.walk(root):
    if "/execution" in d or "node_modules" in d or "/target" in d:
        continue
    for f in fs:
        if not f.endswith(".json"):
            continue
        p = os.path.join(d, f)
        if os.path.getsize(p) > 20_000_000:
            continue
        try:
            obj = json.load(open(p))
        except Exception:
            continue
        seen = False
        for m in models(obj):
            for c in m["load_cases"]:
                if not isinstance(c, dict):
                    continue
                cases += 1; seen = True
                per = collections.Counter()
                for l in c.get("primitive_loads") or []:
                    if isinstance(l, dict) and (l.get("target") or {}).get("type") == "element":
                        per[((l["target"].get("pipe") or l["target"].get("element")), l.get("direction"), l.get("category"))] += 1
                        per[("__element__", (l["target"].get("pipe") or l["target"].get("element")))] += 1
                for k, n in per.items():
                    if k[0] == "__element__":
                        maxe = max(maxe, n)
                        if n >= 2:
                            hits.append({"file": os.path.relpath(p, root), "case": c.get("id"), "element": k[1], "element_loads": n})
        files += seen
print(json.dumps({"files_with_load_cases": files, "cases": cases, "max_element_loads_on_one_element": maxe,
                  "elements_with_2plus_loads": hits}, indent=1))
