#!/usr/bin/env python3
"""I67 U6d: compare per-test outcomes of two Vitest JSON reports (base vs candidate).
Usage: compare_outcomes.py <base.json> <cand.json> > out.txt. Keys are (file under src/, full name)."""
import json, sys
def load(p):
    d = json.load(open(p))
    out = {}
    for f in d["testResults"]:
        name = f["name"].split("/src/", 1)[1]
        for a in f["assertionResults"]:
            out[(name, a["fullName"])] = a["status"]
    return d, out
bd, b = load(sys.argv[1]); cd, c = load(sys.argv[2])
print(f"base: {bd['numTotalTests']} tests, {bd['numPassedTests']} passed, {bd['numFailedTests']} failed; files {len({k[0] for k in b})}")
print(f"cand: {cd['numTotalTests']} tests, {cd['numPassedTests']} passed, {cd['numFailedTests']} failed; files {len({k[0] for k in c})}")
changed = [(k, b[k], c.get(k, "ABSENT")) for k in sorted(b) if c.get(k) != b[k]]
added = sorted(k for k in c if k not in b)
print(f"existing tests with a different outcome or absent: {len(changed)}")
for k, x, y in changed: print(f"  {k[0]} :: {k[1]} :: {x} -> {y}")
files = {}
for k in added: files.setdefault(k[0], []).append(c[k])
print(f"tests only in candidate: {len(added)}")
for f, s in sorted(files.items()): print(f"  {f}: {len(s)} ({', '.join(f'{x}={s.count(x)}' for x in sorted(set(s)))})")
