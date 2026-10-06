"""I75: compare two vitest JSON reports test by test (file + full name)."""
import json, sys, collections, pathlib
def load(p):
    d = json.load(open(p)); out = {}
    for f in d["testResults"]:
        name = f["name"]; rel = name[name.index("src/"):] if "src/" in name else name
        for t in f["assertionResults"]:
            key = (rel, t["fullName"])
            if key in out: key = (rel, t["fullName"] + " #dup%d" % sum(1 for k in out if k[0] == rel and k[1].startswith(t["fullName"])))
            out[key] = t["status"]
    return d, out
bd, b = load(sys.argv[1]); cd, c = load(sys.argv[2])
print("base:", bd["numTotalTests"], "passed", bd["numPassedTests"], "failed", bd["numFailedTests"], "files", len(bd["testResults"]))
print("cand:", cd["numTotalTests"], "passed", cd["numPassedTests"], "failed", cd["numFailedTests"], "files", len(cd["testResults"]))
added = sorted(k for k in c if k not in b); removed = sorted(k for k in b if k not in c)
changed = sorted(k for k in b if k in c and b[k] != c[k])
print("added", len(added)); [print("  +", c[k], k[0], "::", k[1]) for k in added]
print("removed", len(removed)); [print("  -", b[k], k[0], "::", k[1]) for k in removed]
print("status changed", len(changed)); [print("  ~", b[k], "->", c[k], k[0], "::", k[1]) for k in changed]
same = sum(1 for k in b if k in c and b[k] == c[k]); print("unchanged", same)
