"""Compare two census outputs entry by entry and entry point by entry point."""
import json, sys
a, b = json.load(open(sys.argv[1])), json.load(open(sys.argv[2]))
assert a["corpus_sha256"] == b["corpus_sha256"] and a["counts"] == b["counts"], "different corpora"
assert a["rows"].keys() == b["rows"].keys()
diffs = [(k, ep, a["rows"][k][ep], b["rows"][k][ep]) for k in sorted(a["rows"]) for ep in sorted(a["rows"][k]) if a["rows"][k][ep] != b["rows"][k][ep]]
n = sum(len(v) for v in a["rows"].values())
print(f"corpus {a['corpus_sha256']} counts {a['counts']}: {len(a['rows'])} entries x 3 entry points = {n} outcomes; {len(diffs)} differ")
for d in diffs: print("DIFF", *d)
