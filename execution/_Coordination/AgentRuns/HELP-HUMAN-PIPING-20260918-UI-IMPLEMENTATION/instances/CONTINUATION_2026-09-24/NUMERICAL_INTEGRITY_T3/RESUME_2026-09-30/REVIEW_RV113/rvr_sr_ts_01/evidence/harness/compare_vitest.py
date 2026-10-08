"""RV113: vitest JSON reports, test by test. Usage: python compare_vitest.py <i1.json> <head.json> <out.json>"""
import json, re, sys
def tests(path):
    d = json.load(open(path)); out = {}
    for f in d["testResults"]:
        name = re.sub(r"^.*/apps/desktop/", "", f["name"])
        for a in f["assertionResults"]:
            key = f"{name} :: {a['fullName']}"
            n = 1
            while f"{key} #{n}" in out: n += 1
            out[f"{key} #{n}"] = a["status"]  # duplicate names are numbered in report order
    return d, out
di, a = tests(sys.argv[1]); dh, b = tests(sys.argv[2])
tally = lambda r: {s: sum(1 for v in r.values() if v == s) for s in sorted(set(r.values()))}
diff = [{"test": k, "i1": a.get(k), "head": b.get(k)} for k in sorted(set(a) | set(b)) if a.get(k) != b.get(k)]
res = {"i1": {"files": di["numTotalTestSuites"] if "numTotalTestSuites" in di else None, "tests": tally(a)}, "head": {"tests": tally(b)}, "differences": diff}
json.dump(res, open(sys.argv[3], "w"), indent=1)
print(json.dumps({"i1": tally(a), "head": tally(b), "differences": len(diff)}))
for x in diff: print("  ", json.dumps(x)[:300])
