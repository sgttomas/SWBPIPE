"""I92 suites, base (I1) against head, test by test: vitest's JSON reports keyed by (file relative to
apps/desktop, full test name). Usage: compare_suites.py <base vitest.json> <head vitest.json> <out.json>"""
import json, sys, collections

def load(path):
    r = json.load(open(path))
    out = {}
    for t in r["testResults"]:
        rel = t["name"].split("/apps/desktop/", 1)[1]
        for a in t["assertionResults"]:
            key = rel + " :: " + a["fullName"]
            n = 1
            while (key if n == 1 else f"{key} #{n}") in out: n += 1  # a repeated title, in file order
            out[key if n == 1 else f"{key} #{n}"] = a["status"]
    summary = {k: r[k] for k in ("numTotalTestSuites", "numPassedTestSuites", "numFailedTestSuites", "numTotalTests", "numPassedTests", "numFailedTests", "numPendingTests", "numTodoTests", "success")}
    return out, summary

base, bs = load(sys.argv[1]); head, hs = load(sys.argv[2])
added = sorted(set(head) - set(base)); removed = sorted(set(base) - set(head))
changed = sorted(k for k in set(base) & set(head) if base[k] != head[k])
statuses = lambda d: dict(collections.Counter(d.values()))
result = {"base": {"summary": bs, "statuses": statuses(base)}, "head": {"summary": hs, "statuses": statuses(head)},
          "added": [[k, head[k]] for k in added], "removed": [[k, base[k]] for k in removed], "changed": [[k, base[k], head[k]] for k in changed]}
json.dump(result, open(sys.argv[3], "w"), indent=1)
print("base", bs, statuses(base)); print("head", hs, statuses(head))
print(f"added {len(added)}, removed {len(removed)}, changed {len(changed)}")
for k in added: print("ADDED", head[k], k)
for k in removed: print("REMOVED", base[k], k)
for k in changed: print("CHANGED", base[k], "->", head[k], k)
