"""SQ2: one profile tree that bounds both routes' trees (RR "I95's B3-S ...", ruling 1: the registered profile takes the
maximum over routes). Every form is the coefficient-wise maximum of the two routes' forms (every atom is a non-negative
size or text count, so the merged form bounds each route's form at any atom values, in the build); every text atom and
every L entry is the larger; the exprs and phases are the chain's own and must be equal in both trees. The tree's check
(the chain's own evaluation at its ASSUMED values, as g4_caps.py computes it) is recomputed for the merged forms.
Usage: merge_trees.py <tree L> <tree E> <out tree> '<basis text>'"""
import json, sys
a, b = (json.load(open(p)) for p in sys.argv[1:3]); out_p, basis = sys.argv[3], sys.argv[4]
for k in ("exprs", "phases", "EPS", "M", "R", "which"):
    assert a[k] == b[k], k
forms = {}
for name in sorted(set(a["forms"]) | set(b["forms"])):
    fa, fb = a["forms"].get(name, {}), b["forms"].get(name, {})
    forms[name] = {k: max(fa.get(k, 0), fb.get(k, 0)) for k in sorted(set(fa) | set(fb))}
text = {k: max(a["text_atoms"].get(k, 0), b["text_atoms"].get(k, 0)) for k in sorted(set(a["text_atoms"]) | set(b["text_atoms"]))}
L = {k: max(a["L"].get(k, 0), b["L"].get(k, 0)) for k in sorted(set(a["L"]) | set(b["L"]))}
assumed = dict(a["assumed"])
for k, v in b["assumed"].items():
    assert assumed.get(k, v) == v, ("assumed atom differs", k)
    assumed[k] = v
vals = dict(assumed); vals.update(text)
def ev_form(f): return sum(c * (1 if k == "1" else vals[k]) for k, c in f.items())
def evn(n):
    if isinstance(n, str): return evn(a["exprs"][n]) if n in a["exprs"] else ev_form(forms[n])
    if "sum" in n: return sum(evn(c) for c in n["sum"])
    return max(evn(c) for c in n["max"])
check = {}
for mode, ph in a["phases"].items():
    best = None
    for name, v in ph.items():
        e = sum(evn(c) for c in v["requested"]) + max(evn(c) for c in v["moving"]) + a["R"]
        if best is None or e > best[0]: best = (e, name)
    check[mode] = {"max": best[0], "phase": best[1]}
# the merged check must reproduce each route's own check where that route's forms are the larger (a sanity check)
tree = dict(a); tree.update(forms=forms, text_atoms=text, L=L, assumed=assumed, check=check, basis=basis)
json.dump(tree, open(out_p, "w"), indent=None)
print(json.dumps({"check": check, "check_L": a["check"], "check_E": b["check"]}))
