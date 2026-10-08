"""Base against head, test by test, from pytest junit XML (as I83's compare_suites.py, Python part).
Usage: compare_suites.py <base py.xml> <head py.xml> <out.json>"""
import collections, json, sys
import xml.etree.ElementTree as ET


def outcomes(path):
    out = {}
    for tc in ET.parse(path).getroot().iter("testcase"):
        key = f'{tc.get("classname")}::{tc.get("name")}'
        assert key not in out, key
        out[key] = "failed" if tc.find("failure") is not None or tc.find("error") is not None else "skipped" if tc.find("skipped") is not None else "passed"
    return out


b, h = outcomes(sys.argv[1]), outcomes(sys.argv[2])
report = {"base": dict(collections.Counter(b.values())), "head": dict(collections.Counter(h.values())),
          "added": sorted(k for k in h if k not in b), "removed": sorted(k for k in b if k not in h),
          "changed_outcome": sorted(k for k in b if k in h and b[k] != h[k]),
          "added_outcomes": dict(collections.Counter(h[k] for k in h if k not in b))}
json.dump(report, open(sys.argv[3], "w"), indent=1)
print("python", report["base"], "->", report["head"], "added", len(report["added"]), report["added_outcomes"],
      "removed", len(report["removed"]), "changed", len(report["changed_outcome"]))
for k in report["added"]: print("  +", k)
for k in report["removed"]: print("  -", k)
for k in report["changed_outcome"]: print("  ~", k, b[k], "->", h[k])
