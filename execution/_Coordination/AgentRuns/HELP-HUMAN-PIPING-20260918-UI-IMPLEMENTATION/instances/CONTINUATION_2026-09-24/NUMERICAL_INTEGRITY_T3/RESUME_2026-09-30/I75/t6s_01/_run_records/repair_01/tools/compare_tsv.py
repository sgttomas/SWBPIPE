"""I75 REPAIR_01: compare two per-test lists (status, file, full name), test by test."""
import sys, collections
def load(p):
    rows = [l.rstrip("\n").split("\t") for l in open(p)][1:]
    out = collections.Counter(); status = {}
    for st, f, name in rows:
        out[(f, name)] += 1; status.setdefault((f, name), []).append(st)
    return status
b = load(sys.argv[1]); c = load(sys.argv[2])
nb = sum(len(v) for v in b.values()); nc = sum(len(v) for v in c.values())
print("base tests", nb, "passed", sum(v.count("passed") for v in b.values()))
print("cand tests", nc, "passed", sum(v.count("passed") for v in c.values()))
added = sorted(k for k in c if k not in b); removed = sorted(k for k in b if k not in c)
changed = sorted(k for k in b if k in c and sorted(b[k]) != sorted(c[k]))
print("added", len(added)); [print("  +", k[0], "::", k[1]) for k in added]
print("removed", len(removed)); [print("  -", k[0], "::", k[1]) for k in removed]
print("status changed", len(changed)); [print("  ~", b[k], "->", c[k], k[0], "::", k[1]) for k in changed]
print("unchanged", sum(len(b[k]) for k in b if k in c and sorted(b[k]) == sorted(c[k])))
