#!/usr/bin/env python3
"""I67 U6d round 04: per-test outcomes.
(a) Every file but retainedPrecisionIntegration.test.tsx: base 6383e8e70e against the candidate,
    by exact title. (At base the integration file fails to collect: its v2 consumer cannot read v3.)
(b) The integration file: its last full run (round 03, 76477534f6's TS files) against the candidate,
    with the round's declared renames (format v2 -> v3; the declared block, v2 entries -> v3 forms).
Usage: compare_outcomes_r4.py base4.json r3_cand.json r4_cand.json"""
import json, sys
F = "features/results/retainedPrecisionIntegration.test.tsx"
def load(p):
    d = json.load(open(p)); out = {}
    for f in d["testResults"]:
        name = f["name"].split("/src/", 1)[1]
        for a in f["assertionResults"]: out[(name, a["fullName"])] = a["status"]
    return d, out
bd, b = load(sys.argv[1]); rd, r = load(sys.argv[2]); cd, c = load(sys.argv[3])
print(f"base 6383e8e70e: {bd['numTotalTests']} tests, {bd['numPassedTests']} passed, {bd['numFailedTests']} failed; failed suites: {[f['name'].split('/src/',1)[1] for f in bd['testResults'] if f['status'] != 'passed']}")
print(f"candidate: {cd['numTotalTests']} tests, {cd['numPassedTests']} passed, {cd['numFailedTests']} failed")
same = [k for k in b if k[0] != F and c.get(k) == b[k]]
diff = [(k, b[k], c.get(k, "ABSENT")) for k in sorted(b) if k[0] != F and c.get(k) != b[k]]
extra = sorted(k for k in c if k[0] != F and k not in b)
print(f"(a) other files: {len(same)} identical, {len(diff)} changed, {len(extra)} new")
for k, x, y in diff: print(f"   CHANGED {k[0]} :: {k[1]} :: {x} -> {y}")
for k in extra: print(f"   NEW {k[0]} :: {k[1]} :: {c[k]}")
RENAME = {"the inputs and the pinned identity uses PP's byte-identical successors and the shared 20-case file (format v2)":
          "the inputs and the pinned identity uses PP's byte-identical successors and the shared 20-case file (format v3)",
          "the declared differences, with TypeScript's expectations are exactly the four ruled entries, each with a ruling and one expectation per language":
          "the declared differences, with TypeScript's expectations are exactly the five ruled entries, each with a ruling, forms and one expectation per language"}
DECL = "the declared differences, with TypeScript's expectations "
V2_TO_V3 = {"I67-F1:unregistered_invalid_statement": "edited_row:no_invocation", "I67-F2:display_only_binding_precheck": "none:binding",
            "F-U6b-2:python_refuses_transport": "unedited", "F5:refused_statement_binding": "edited_row:no_invocation"}
rr = {k: v for k, v in r.items() if k[0] == F}
mapped, same_i, changed_i = set(), 0, []
for (f, t), st in sorted(rr.items()):
    nt = RENAME.get(t, t)
    for eid, label in V2_TO_V3.items():
        for fx in ("milestone_sparse_interactive", "milestone_dense_scrutiny"):
            if t == f"{DECL}{eid} on {fx}": nt = f"{DECL}{eid} / {label} on {fx}"
    got = c.get((f, nt), "ABSENT"); mapped.add((f, nt))
    if got == st: same_i += 1
    else: changed_i.append((t, nt, st, got))
new_i = sorted(k for k in c if k[0] == F and k not in mapped)
print(f"(b) integration file: {len(rr)} tests in round 03; {same_i} keep their outcome (after declared renames); {len(changed_i)} changed; {len(new_i)} new")
for t, nt, x, y in changed_i: print(f"   CHANGED {t} -> {nt} :: {x} -> {y}")
for k in new_i: print(f"   NEW {k[1]} :: {c[k]}")
