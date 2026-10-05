#!/usr/bin/env python3
"""I67 U6d round 03: per-test outcomes, base (merged head 52052ece61) vs candidate, with the
round's declared renames: the parity block's title (14 -> 20 scenarios, Rust and Python),
the format pin's title (v1 14-case -> v2 20-case), and the TS-local F1 pin, replaced by the
shared declared_differences consumer as ROOT directed. Usage: compare_outcomes_r3.py base.json cand.json"""
import json, sys
F = "features/results/retainedPrecisionIntegration.test.tsx"
OLD_PARITY = "the 14 shared parity scenarios agree with Rust U6a "
NEW_PARITY = "the 20 shared parity scenarios agree with Rust U6a and Python U6b "
SF1 = "the historical v0.2 builder never drops a receipt (RV91 SF-1; Python's F-U6b-3 twin) "
RENAME = {"the inputs and the pinned identity uses PP's byte-identical successors and the shared 14-case file":
          "the inputs and the pinned identity uses PP's byte-identical successors and the shared 20-case file (format v2)",
          # Round 02's SF-1 tests, now parameterized over the 0.1.0 and 0.2.0 shapes (RV91 N-1).
          SF1 + "a legacy-shaped source without a receipt or token still builds (control)": SF1 + "a 0.1.0 legacy-shaped source without a receipt or token still builds (control)",
          SF1 + "a legacy-shaped source carrying a retained_precision object is refused ANALYSIS_LEGACY_SOURCE_DOWNGRADE_FORBIDDEN": SF1 + "a 0.1.0 legacy-shaped source carrying a retained_precision object is refused ANALYSIS_LEGACY_SOURCE_DOWNGRADE_FORBIDDEN",
          SF1 + "a legacy-shaped source carrying a null retained_precision member is refused ANALYSIS_LEGACY_SOURCE_DOWNGRADE_FORBIDDEN": SF1 + "a 0.1.0 legacy-shaped source carrying a null retained_precision member is refused ANALYSIS_LEGACY_SOURCE_DOWNGRADE_FORBIDDEN",
          SF1 + "a legacy-shaped source carrying one W1 token row is refused ANALYSIS_LEGACY_SOURCE_DOWNGRADE_FORBIDDEN": SF1 + "a 0.1.0 legacy-shaped source carrying a W1 token on the first row is refused ANALYSIS_LEGACY_SOURCE_DOWNGRADE_FORBIDDEN"}
REPLACED_PREFIX = ("sparse_interactive: the declared parity difference F1 (pinned in TS only)", "dense_scrutiny: the declared parity difference F1 (pinned in TS only)")
def load(p):
    d = json.load(open(p)); out = {}
    for f in d["testResults"]:
        name = f["name"].split("/src/", 1)[1]
        for a in f["assertionResults"]: out[(name, a["fullName"])] = a["status"]
    return d, out
bd, b = load(sys.argv[1]); cd, c = load(sys.argv[2])
print(f"base: {bd['numTotalTests']} tests, {bd['numPassedTests']} passed, {bd['numFailedTests']} failed")
print(f"cand: {cd['numTotalTests']} tests, {cd['numPassedTests']} passed, {cd['numFailedTests']} failed")
mapped, unchanged, changed, replaced = {}, 0, [], []
for (f, t), st in sorted(b.items()):
    nt = RENAME.get(t, NEW_PARITY + t[len(OLD_PARITY):] if (f == F and t.startswith(OLD_PARITY)) else t)
    if f == F and t.startswith(REPLACED_PREFIX):
        replaced.append((t, st)); continue
    got = c.get((f, nt), "ABSENT")
    mapped[(f, nt)] = True
    if got == st: unchanged += 1
    else: changed.append((f, t, nt, st, got))
print(f"base tests with the same outcome (after declared renames): {unchanged}")
print(f"base tests whose outcome changed: {len(changed)}")
for f, t, nt, x, y in changed: print(f"  {f} :: {t} -> {nt if nt != t else '(same title)'} :: {x} -> {y}")
print(f"base tests replaced (TS-local F1 pin -> shared declared_differences): {len(replaced)}")
for t, st in replaced: print(f"  {t} :: {st}")
added = sorted(k for k in c if k not in mapped)
files = {}
for k in added: files.setdefault(k[0], []).append((k[1], c[k]))
print(f"candidate tests not in base: {len(added)}")
for f, items in sorted(files.items()):
    print(f"  {f}: {len(items)} ({', '.join(f'{s}={sum(1 for _, x in items if x == s)}' for s in sorted({x for _, x in items}))})")
    for t, s in items: print(f"    {s} :: {t}")
