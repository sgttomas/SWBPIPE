"""I82 B1-S: the in-build evaluator of a profile tree (stdlib only).

The same arithmetic as the generated `profile` module and `admission_bound` (PP retained_memory.rs):
every form is a linear form over layout atoms (checked u64), T12_T15/T16/T17/T25 are the exprs'
sums and maxima, each phase is requested + max(moving), E_mov,max is the maximum over the seven
phases, and the bound is E_mov,max + R <= M (R = RESERVED_STACK_BYTES = 64 MiB). Atom values are
the registered build's in-build values as its own law test printed them (`I65_G5_ATOM <binding>
<name> <in-build> <assumed>` in I72's U8 Pass B law record, the registered dev/test identity at a
basis whose PP, FK and reader trees equal main d8c88774d0's). Text atoms come from the tree itself.
Usage (module): load_atoms(law_record) -> dict; evaluate(tree, atoms) -> per-mode result.
"""
import json, sys

R = 64 << 20
U64 = (1 << 64) - 1

def load_atoms(path):
    atoms = {}
    for line in open(path):
        if line.startswith("I65_G5_ATOM"):
            head, name, inbuild, assumed = line.rstrip("\n").split("\t")   # "I65_G5_ATOM <binding>"
            atoms[name] = int(inbuild)
    return atoms

def form(f, v):
    tot = f.get("1", 0)
    for a, c in f.items():
        if a == "1":
            continue
        tot += c * v[a]
        if tot > U64:
            raise OverflowError(a)
    return tot

def node(tree, name, v, memo):
    if isinstance(name, dict):
        if "sum" in name:
            return sum(node(tree, c, v, memo) for c in name["sum"])
        return max(node(tree, c, v, memo) for c in name["max"])
    if name in memo:
        return memo[name]
    x = node(tree, tree["exprs"][name], v, memo) if name in tree["exprs"] else form(tree["forms"][name], v)
    memo[name] = x
    return x

def evaluate(tree, atoms, M=None):
    v = dict(atoms)
    for k, x in tree["text_atoms"].items():
        if k.startswith("Text("):
            v[k] = x
    out = {}
    for mode in ("sparse", "dense"):
        memo, ph = {}, {}
        for pname, p in tree["phases"][mode].items():
            rq = sum(node(tree, c, v, memo) for c in p["requested"])
            mv = max(node(tree, c, v, memo) for c in p["moving"])
            ph[pname.split(" ")[0]] = {"requested": rq, "moving": mv, "E_mov_plus_R": rq + mv + R}
        worst = max(ph.items(), key=lambda kv: kv[1]["E_mov_plus_R"])
        out[mode] = {"phases": ph, "phase": worst[0], "E_mov_max": worst[1]["E_mov_plus_R"] - R,
                     "E_mov_plus_R": worst[1]["E_mov_plus_R"], "components": {k: memo[k] for k in sorted(memo)}}
    return out

if __name__ == "__main__":
    tree = json.load(open(sys.argv[1])); atoms = load_atoms(sys.argv[2])
    missing = sorted({a for f in tree["forms"].values() for a in f if a != "1" and a not in atoms and not a.startswith("Text(")})
    r = evaluate(tree, atoms)
    print(json.dumps({"missing_atoms": missing, "sparse": {k: r["sparse"][k] for k in ("phase", "E_mov_max", "E_mov_plus_R")},
                      "dense": {k: r["dense"][k] for k in ("phase", "E_mov_max", "E_mov_plus_R")},
                      "phases": {m: r[m]["phases"] for m in r}}, indent=1))
