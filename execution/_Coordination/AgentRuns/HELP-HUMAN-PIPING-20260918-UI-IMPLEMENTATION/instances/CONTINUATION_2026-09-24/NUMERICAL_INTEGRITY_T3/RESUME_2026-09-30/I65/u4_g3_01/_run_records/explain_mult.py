"""Explain a function's multiplicity in text_budget output: list incoming call sites with
M(caller), loop headers and the product. Read-only helper for TEXT.md review."""
import json, sys, re, collections
sys.argv += []
tb = json.load(open(sys.argv[1])); cg = json.load(open(sys.argv[2])); lb = json.load(open(sys.argv[3]))
M = tb["function_multiplicity"]
rules = [(re.compile(r["re"]), r["bound"]) for r in lb["loops"]]
counts = lb["counts"]["caps"]
def bound(h):
    for rx, b in rules:
        if rx.search(h): return eval(b, {"max": max, "min": min}, dict(counts)), b
    m = re.search(r"\bin\s*\(?\s*(\d+)(?:usize)?\s*\.\.(=?)\s*(\d+)\b", h)
    if m: return int(m.group(3)) - int(m.group(1)) + (1 if m.group(2) else 0), "lit"
    return None, None
for target in sys.argv[4:]:
    print("==", target, M.get([k for k in M if k.endswith(target)][0] if [k for k in M if k.endswith(target)] else "", 0))
    for a, b, stacks in cg["site_loops"]:
        if b.endswith(target) and M.get(a, 0):
            for st in stacks:
                prod, parts = 1, []
                for h in st:
                    v, why = bound(h); parts.append((h[:60], v)); prod *= (v if v is not None else 0)
                print("   ", M.get(a, 0), "x", prod, "=", M.get(a, 0) * prod, a.split("src/")[-1], parts)
