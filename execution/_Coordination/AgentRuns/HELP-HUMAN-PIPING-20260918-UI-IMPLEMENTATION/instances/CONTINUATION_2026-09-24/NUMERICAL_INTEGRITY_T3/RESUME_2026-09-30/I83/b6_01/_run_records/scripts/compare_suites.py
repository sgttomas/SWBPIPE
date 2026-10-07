"""Base against head, test by test: Python (junit xml), vitest (json), Rust (cargo test log).
Usage: compare_suites.py <logs/base> <logs/head> <out.json>"""
import collections, json, re, sys
import xml.etree.ElementTree as ET
B, H, OUT = sys.argv[1], sys.argv[2], sys.argv[3]

def py(d):
    out = {}
    for tc in ET.parse(f"{d}/py.xml").getroot().iter("testcase"):
        key = f'{tc.get("classname")}::{tc.get("name")}'
        status = "failed" if tc.find("failure") is not None or tc.find("error") is not None else "skipped" if tc.find("skipped") is not None else "passed"
        out[key] = status
    return out

def ts(d):
    res = json.load(open(f"{d}/vitest.json")); out = {}; seen = collections.Counter()
    for r in res["testResults"]:
        f = r["name"].split("/apps/desktop/")[-1]
        for a in r["assertionResults"]:
            k = (f, a["fullName"]); seen[k] += 1
            out[f"{f} :: {a['fullName']} #{seen[k]}"] = a["status"]
    return out

def rs(d):
    out = {}; binary = None
    for line in open(f"{d}/rs.log"):
        m = re.match(r"\s+Running (\S+)", line)
        if m: binary = m.group(1); continue
        m = re.match(r"\s+Doc-tests (\S+)", line)
        if m: binary = "doc " + m.group(1); continue
        m = re.match(r"test (.+?) \.\.\. (ok|FAILED|ignored)", line)
        if m: out[f"{binary} :: {m.group(1)}"] = {"ok": "passed", "FAILED": "failed", "ignored": "skipped"}[m.group(2)]
    return out

report = {}
for name, fn in (("python", py), ("vitest", ts), ("rust", rs)):
    b, h = fn(B), fn(H)
    report[name] = {
        "base": dict(collections.Counter(b.values())), "head": dict(collections.Counter(h.values())),
        "added": sorted(k for k in h if k not in b), "removed": sorted(k for k in b if k not in h),
        "changed_outcome": sorted(k for k in b if k in h and b[k] != h[k]),
    }
    print(name, report[name]["base"], "->", report[name]["head"], "added", len(report[name]["added"]), "removed", len(report[name]["removed"]), "changed", len(report[name]["changed_outcome"]))
json.dump(report, open(OUT, "w"), indent=1)
