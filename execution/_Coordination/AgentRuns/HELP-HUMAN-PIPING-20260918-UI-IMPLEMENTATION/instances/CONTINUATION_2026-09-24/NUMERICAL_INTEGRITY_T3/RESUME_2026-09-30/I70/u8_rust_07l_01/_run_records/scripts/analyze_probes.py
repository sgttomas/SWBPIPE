"""Summarize the probe logs: per-entry listing (control) and kill matrix (mutants).

usage: analyze_probes.py <logs/probes dir> <out dir>
Indices below (cases 15, mutations 278, must-pass 24) are 07k entries; the rest are 07l's.
"""
import json
import re
import sys
from pathlib import Path

d, out = Path(sys.argv[1]), Path(sys.argv[2])
out.mkdir(parents=True, exist_ok=True)
TEST = re.compile(r"^test (.+?) \.\.\. (ok|FAILED|ignored)$")
K = {"I70_CASE": 15, "I70_MUTATION": 278, "I70_MUST_PASS": 24}
summary = {}
for log in sorted(d.glob("*.log")):
    name = log.stem
    rc = (d / f"{name}.rc").read_text().strip()
    tests, entries, summ = {}, {k: [] for k in K}, None
    for line in log.read_text(errors="replace").splitlines():
        if m := TEST.match(line):
            tests[m.group(1)] = m.group(2)
        for k in K:
            if line.startswith(k + " "):
                entries[k].append(json.loads(line[len(k) + 1:]))
        if line.startswith("I70_SUMMARY "):
            summ = json.loads(line[len("I70_SUMMARY "):])
    failed = sorted(t for t, s in tests.items() if s == "FAILED")
    mism = {k: [(e["i"], e["id"]) for e in v if not e["match"]] for k, v in entries.items()}
    in07k = {k: [x for x in v if x[0] < K[k]] for k, v in mism.items()}
    summary[name] = {
        "rc": rc,
        "tests": {s: list(tests.values()).count(s) for s in ("ok", "FAILED", "ignored")},
        "failed_tests": failed,
        "listing_counts": {k: len(v) for k, v in entries.items()},
        "probe_summary": summ,
        "mismatched_entries": mism,
        "mismatched_07k_entries": in07k,
    }
    if name == "none":
        with open(out / "full_07l_listing.jsonl", "w") as f:
            for k in K:
                for e in entries[k]:
                    f.write(json.dumps({"kind": k, **e}) + "\n")
json.dump(summary, open(out / "probe_summary.json", "w"), indent=2)
for name, s in summary.items():
    print(name, s["rc"], s["tests"], "failed:", s["failed_tests"])
    print("   mismatched:", s["mismatched_entries"])
    print("   07k mismatched:", {k: len(v) for k, v in s["mismatched_07k_entries"].items()}, "summary:", s["probe_summary"])
