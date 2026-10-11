"""SQ2: evaluate profile trees' forms and phases at given atom values (an I65_G5_ATOM listing from a law-test log, or the
tree's own assumed values), and compare two trees form by form. Usage: tree_eval.py <atoms log|-> <tree A> [<tree B>]"""
import json, sys, re
atoms_p, ta = sys.argv[1], sys.argv[2]; tb = sys.argv[3] if len(sys.argv) > 3 else None
def load_atoms(p, tree):
    if p == "-": return {**tree["assumed"], **tree["text_atoms"]}
    vals = {}
    for line in open(p):
        if "I65_G5_ATOM" in line:
            b, name, v, a = line.split("I65_G5_ATOM ", 1)[1].rstrip("\n").split("\t")
            vals[name] = int(v)
    return vals
def ev_form(f, A):
    return sum(c * (1 if k == "1" else A[k]) for k, c in f.items())
def ev_tree(t, A):
    A = dict(A); A.update({k: v for k, v in t["text_atoms"].items() if k.startswith("Text(")})   # text atoms are the tree's own
    F = {k: ev_form(v, A) for k, v in t["forms"].items()}
    def ex(e):
        if isinstance(e, str): return F[e] if e in F else ex(t["exprs"][e])
        if "sum" in e: return sum(ex(x) for x in e["sum"])
        if "max" in e: return max(ex(x) for x in e["max"])
        raise ValueError(e)
    out = {}
    for mode, ph in t["phases"].items():
        out[mode] = {p.split(" ")[0]: (sum(ex(x) for x in v["requested"]), max(ex(x) for x in v["moving"])) for p, v in ph.items()}
    return F, out
A_tree = json.load(open(ta)); A = load_atoms(atoms_p, A_tree)
missing = [k for f in A_tree["forms"].values() for k in f if k != "1" and k not in A]
if missing: print("MISSING ATOMS", sorted(set(missing))[:10])
Fa, Pa = ev_tree(A_tree, A)
R = A_tree["R"]
for mode in ("sparse", "dense"):
    best = max(Pa[mode].items(), key=lambda kv: sum(kv[1]))
    print(f"A {mode}: max {best[0]} E+R={sum(best[1]) + R:,}  " + " ".join(f"{p}={sum(v)+R:,}" for p, v in Pa[mode].items()))
if tb:
    B_tree = json.load(open(tb)); Fb, Pb = ev_tree(B_tree, A)
    for mode in ("sparse", "dense"):
        best = max(Pb[mode].items(), key=lambda kv: sum(kv[1]))
        print(f"B {mode}: max {best[0]} E+R={sum(best[1]) + R:,}  " + " ".join(f"{p}={sum(v)+R:,}" for p, v in Pb[mode].items()))
    for k in sorted(set(Fa) | set(Fb), key=lambda k: -abs(Fb.get(k, 0) - Fa.get(k, 0))):
        d = Fb.get(k, 0) - Fa.get(k, 0)
        if d: print(f"  form {k}: {Fa.get(k,0):,} -> {Fb.get(k,0):,} ({d:+,})")
