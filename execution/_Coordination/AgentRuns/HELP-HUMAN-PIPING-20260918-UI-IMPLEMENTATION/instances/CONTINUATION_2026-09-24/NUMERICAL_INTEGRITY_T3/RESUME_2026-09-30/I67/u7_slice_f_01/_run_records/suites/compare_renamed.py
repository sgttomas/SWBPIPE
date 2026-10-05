#!/usr/bin/env python3
"""I67 U7 slice F part 2: per-test outcomes of two Vitest JSON reports with the candidate's
title renames applied to the earlier report's names (RENAMES, in order). Every earlier test must
exist in the candidate under its renamed title; changed outcomes and candidate-only tests are listed.
Usage: compare_renamed.py <earlier.json> <cand.json>"""
import json, sys
RENAMES = [
    ("with eligibility held, every one of these reads exactly as before (NOT_NUMERICALLY_ELIGIBLE)", "with the held reader (u7.held), every one of these reads as before U7 (NOT_NUMERICALLY_ELIGIBLE)"),
    ("refuses a registered successor before any rule backend call, and unregistered bytes as not native", "passes a registered successor to the rule backend with its invocation (U7); copies and refused bytes are not native; the held reader's needs_recompute is refused before any backend call"),
    ("the rule-check gate and binding refusals (eligibility held)", "the rule-check gate and binding refusals"),
    ("the simulated post-U7 path through the real carriers", "the post-U7 path through the real carriers"),
    (": token and status with eligibility forced", ": token and status"),
    ("are exactly the five ruled entries, each with a ruling, forms and one expectation per language", "are exactly the six ruled entries, each with a ruling, forms and one expectation per language, in closed field sets"),
    ("the shared 20-case file (format v3)", "the shared 20-case file (format v4)"),
    ("the post-U7 standing rules (seams; the reader's flag is untouched)", "the post-U7 standing rules (seams, independent of the reader's flag)"),
    ("other consumers are unchanged and never make a successor Current", "other consumers are unchanged; the standing session Current reads of a successor"),
    ("a registered successor is fresh but never numerically eligible, so it is never Current", "a registered successor is fresh and numerically eligible for its captured model only (U7)"),
]
def load(p):
    d = json.load(open(p)); out = {}
    for f in d["testResults"]:
        name = f["name"].split("/src/", 1)[1]
        for a in f["assertionResults"]:
            out[(name, a["fullName"])] = a["status"]
    return d, out
def rename(title):
    hits = []
    for a, b in RENAMES:
        if a in title: title = title.replace(a, b); hits.append(a[:40])
    return title, hits
bd, b = load(sys.argv[1]); cd, c = load(sys.argv[2])
print(f"earlier: {bd['numTotalTests']} tests, {bd['numPassedTests']} passed, {bd['numFailedTests']} failed")
print(f"cand: {cd['numTotalTests']} tests, {cd['numPassedTests']} passed, {cd['numFailedTests']} failed")
mapped, renamed, missing, changed = {}, 0, [], []
for (f, t), st in sorted(b.items()):
    nt, hits = rename(t)
    renamed += bool(hits)
    if (f, nt) not in c: missing.append((f, t)); continue
    mapped[(f, nt)] = (f, t)
    if c[(f, nt)] != st: changed.append((f, nt, st, c[(f, nt)]))
print(f"earlier tests renamed: {renamed}; missing from the candidate after renames: {len(missing)}")
for m in missing: print("  MISSING", m)
print(f"outcome changes: {len(changed)}")
for x in changed: print(f"  {x[0]} :: {x[1]} :: {x[2]} -> {x[3]}")
only = sorted(k for k in c if k not in mapped)
print(f"tests only in the candidate: {len(only)}")
for k in only: print(f"  {k[0]} :: {k[1]} :: {c[k]}")
