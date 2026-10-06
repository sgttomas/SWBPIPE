"""I75: compare builder output dumps (base vs candidate) for byte identity on non-successor routes."""
import json, sys, collections
def load(p):
    m = collections.defaultdict(set); routes = {}
    for line in open(p):
        r = json.loads(line); k = (r["kind"], r["input"]); m[k].add(r["output"]); routes[k] = r["route"]
    return m, routes
b, br = load(sys.argv[1]); c, cr = load(sys.argv[2])
SUCC = "retained_preview_physics"
def summary(name, m, routes):
    cnt = collections.Counter((k[0], routes[k]) for k in m)
    print(name, "distinct inputs:", len(m), dict(sorted(cnt.items())))
    nondet = [k for k in m if len(m[k]) > 1]
    print("  inputs with more than one output (nondeterministic):", len(nondet))
summary("base", b, br); summary("cand", c, cr)
both = [k for k in b if k in c]
diff = [k for k in both if b[k] != c[k]]
print("non-successor inputs in both:", sum(1 for k in both if br[k] != SUCC), "differing:", sum(1 for k in diff if br[k] != SUCC))
for k in diff: print("  DIFF", k[0], br[k], k[1])
base_only = [k for k in b if k not in c]; cand_only = [k for k in c if k not in b]
print("base-only inputs:", len(base_only), collections.Counter((k[0], br[k]) for k in base_only))
print("cand-only inputs:", len(cand_only), collections.Counter((k[0], cr[k]) for k in cand_only))
print("successor inputs in base:", sum(1 for k in b if br[k] == SUCC))
