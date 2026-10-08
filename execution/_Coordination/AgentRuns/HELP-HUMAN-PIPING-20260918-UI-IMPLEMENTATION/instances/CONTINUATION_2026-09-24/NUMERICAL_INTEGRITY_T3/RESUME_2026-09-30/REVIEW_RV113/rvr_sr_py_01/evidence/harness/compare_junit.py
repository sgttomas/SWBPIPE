"""RV113 (RV-R): pytest junit XML, test by test. Usage: compare_junit.py <out.json> <label=xml> ...
The first two labelled runs are compared (A then B): added, removed, changed outcomes; each run's tally.
A later `fix:` pair may supersede named tests' outcomes in A and B (e.g. a rerun of files whose inputs were
missing from the copy): label `A+fix=<xml>` and `B+fix=<xml>`."""
import json
import sys
import xml.etree.ElementTree as ET


def outcomes(path):
    out = {}
    for tc in ET.parse(path).getroot().iter("testcase"):
        name = f'{tc.get("classname")}::{tc.get("name")}'
        kind = "passed"
        for child in tc:
            if child.tag in ("failure", "error"):
                kind = "failed" if child.tag == "failure" else "error"
            elif child.tag == "skipped":
                kind = "skipped"
        assert name not in out, name
        out[name] = kind
    return out


def tally(o):
    t = {}
    for v in o.values():
        t[v] = t.get(v, 0) + 1
    return t


runs = {}
for arg in sys.argv[2:]:
    label, path = arg.split("=", 1)
    runs[label] = outcomes(path)
labels = [k for k in runs if "+fix" not in k]
a, b = labels[0], labels[1]
A, B = dict(runs[a]), dict(runs[b])
fixed = {}
for lab, base in ((a, A), (b, B)):
    if f"{lab}+fix" in runs:
        for k, v in runs[f"{lab}+fix"].items():
            assert k in base, k
            fixed.setdefault(lab, {})[k] = [base[k], v]
            base[k] = v
res = {"runs": {k: tally(v) for k, v in runs.items()}, "after_fix": {a: tally(A), b: tally(B)},
       "superseded": {k: len(v) for k, v in fixed.items()},
       "added": sorted((k, B[k]) for k in B if k not in A), "removed": sorted((k, A[k]) for k in A if k not in B),
       "changed": sorted((k, A[k], B[k]) for k in A if k in B and A[k] != B[k])}
json.dump(res, open(sys.argv[1], "w"), indent=1)
print(json.dumps({"runs": res["runs"], "after_fix": res["after_fix"], "added": len(res["added"]), "removed": len(res["removed"]),
                  "changed": len(res["changed"])}))
for x in res["added"]: print("ADDED", x)
for x in res["changed"]: print("CHANGED", x)
for x in res["removed"]: print("REMOVED", x)
