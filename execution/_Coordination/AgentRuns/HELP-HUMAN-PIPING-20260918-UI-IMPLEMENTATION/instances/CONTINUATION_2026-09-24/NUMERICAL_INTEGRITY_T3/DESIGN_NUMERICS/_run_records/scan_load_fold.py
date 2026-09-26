"""Scan committed JSON files for load cases that put >=3 primitive nodal loads on
one (node, direction). Only such DOFs can have a binary64 fold that differs from
the correctly rounded exact sum (a fold of two terms is correctly rounded).
Read-only; standard library. Reports per file: cases scanned, max contributions
on one DOF, and every DOF with >=3."""
import json, os, sys
from fractions import Fraction as Fr
root = sys.argv[1]
hits, scanned_files, scanned_cases, maxc = [], 0, 0, 0
def models(obj):
    if isinstance(obj, dict):
        if "load_cases" in obj and isinstance(obj["load_cases"], list):
            yield obj
        for v in obj.values():
            yield from models(v)
    elif isinstance(obj, list):
        for v in obj:
            yield from models(v)
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
        found = False
        for m in models(obj):
            for case in m["load_cases"]:
                if not isinstance(case, dict):
                    continue
                loads = case.get("primitive_loads") or []
                per = {}
                for l in loads:
                    if not isinstance(l, dict):
                        continue
                    t = l.get("target") or {}
                    if t.get("type") != "node":
                        continue
                    key = (t.get("node"), l.get("direction"))
                    mag = (l.get("magnitude") or {}).get("value")
                    per.setdefault(key, []).append(mag)
                scanned_cases += 1
                found = True
                for key, vals in per.items():
                    maxc = max(maxc, len(vals))
                    if len(vals) >= 3:
                        fold = 0.0
                        for v in vals:
                            fold += float(v)
                        exact = sum(Fr(float(v)) for v in vals)
                        hits.append({"file": os.path.relpath(p, root), "case": case.get("id"), "dof": list(key),
                                     "values": vals, "fold": fold, "fold_equals_rn_exact": fold == float(exact)})
        scanned_files += found
print(json.dumps({"files_with_load_cases": scanned_files, "cases": scanned_cases,
                  "max_nodal_loads_on_one_dof": maxc, "dofs_with_3plus": hits}, indent=1))
