#!/usr/bin/env python3
"""I85 B1-SP I3: the witnesses' outcomes, base (2ba2f81863) against head. The witness tests run
with --nocapture, so a test's status can share a line with its prints; this takes every
`I65_G5_WITNESS*` print, the witness test names and the `test result` lines, and compares them.
Usage: witness_lines.py <base log> <head log> <base lines out> <head lines out>"""
import re, sys
def lines(path):
    text = open(path, encoding="utf-8", errors="replace").read()
    out = [m.group(0).rstrip() for m in re.finditer(r"I65_G5_WITNESS\S* [^\n]*", text)]
    tests = sorted(set(re.findall(r"^test (retained_memory::witness_tests::\S+) \.\.\.", text, re.M)))
    results = re.findall(r"^test result: [^\n]*", text, re.M)
    results = [re.sub(r"finished in [0-9.]+s", "finished in Ns", r) for r in results]
    return out, tests, results
bo, bt, br = lines(sys.argv[1]); ho, ht, hr = lines(sys.argv[2])
open(sys.argv[3], "w").write("\n".join(bo + bt + br) + "\n"); open(sys.argv[4], "w").write("\n".join(ho + ht + hr) + "\n")
print(f"base: {len(bo)} witness lines, {len(bt)} witness tests, results {br}")
print(f"head: {len(ho)} witness lines, {len(ht)} witness tests, results {hr}")
norm = lambda rs: [re.sub(r"\d+ filtered out", "N filtered out", r) for r in rs]
print(f"witness lines identical: {bo == ho}; tests identical: {bt == ht}; results identical but for the filtered-out count (the lib's +3 tests): {norm(br) == norm(hr)}")
