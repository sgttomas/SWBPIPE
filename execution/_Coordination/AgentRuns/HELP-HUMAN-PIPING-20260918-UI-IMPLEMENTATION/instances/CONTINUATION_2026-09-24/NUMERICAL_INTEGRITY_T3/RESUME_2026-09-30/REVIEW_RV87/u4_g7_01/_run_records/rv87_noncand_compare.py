"""RV87 (u4_g7_01): compare non-candidate sets across bases, line-independently: key =
(file, enclosing fn name, normalized expression, kind/spec, rule class, multiplicity).
Usage: python3 rv87_noncand_compare.py <RV87 G6r non-candidates (u4_g6_02)> <G7 dump (TB_NONCAND_OUT)> [<I65 noncandidates_g7.json>]"""
import json, sys, collections
def norm(e): return " ".join(e.split())
def key_g6(r): return (r["site"].rsplit(":", 1)[0], (r["fn"] or "").rsplit(":", 1)[-1], norm(r["expr"])[:160], r["kind_or_spec"], str(r["class"]), r["mult"])
def key_g7(r): return (r["site"].split("core/")[-1].rsplit(":", 1)[0], (r["fn"] or "").rsplit(":", 1)[-1], norm(r["expr"])[:160], r["kind_or_spec"], str(r["class"]), r["mult"])
a = collections.Counter(key_g6(r) for r in json.load(open(sys.argv[1]))["rows"])
b = collections.Counter(key_g7(r) for r in json.load(open(sys.argv[2])))
out = {"rv87_g6r": sum(a.values()), "g7_mine": sum(b.values()), "only_g6r": [list(k) for k in (a - b)], "only_g7": [list(k) for k in (b - a)]}
if len(sys.argv) > 3:
    c = collections.Counter(key_g7(r) for r in json.load(open(sys.argv[3])))
    out["i65_g7"] = sum(c.values()); out["mine_vs_i65_equal"] = (b == c)
print(json.dumps(out, indent=1))
