#!/usr/bin/env python3
"""Per-manifest and per-test comparison of two run_suites_nff.sh outputs (DEC-025 against the Mac baseline).
Usage: python3 compare_suites.py <baseline dir> <candidate dir>   (each holding suites.log and suites/)
Prints, per manifest: counts on both sides, tests removed or changed in outcome, tests added that are not ok,
and the number of tests added ok. A test is a `test <name> ... <outcome>` line; ` - should panic` is dropped."""
import json, os, re, sys
def load(d):
    counts, tests = {}, {}
    for line in open(os.path.join(d, "suites.log")):
        m = re.match(r"^(\d+) passed=(\d+) failed=(\d+) ignored=(\d+) (\S+)$", line.strip())
        if m:
            counts[m.group(5)] = tuple(int(x) for x in m.group(1, 2, 3, 4))
    for f in sorted(os.listdir(os.path.join(d, "suites"))):
        if not f.endswith("_Cargo.toml.log"): continue
        key = f[4:-len("_Cargo.toml.log")]
        out = {}
        for line in open(os.path.join(d, "suites", f), errors="replace"):
            m = re.match(r"^test (.*) \.\.\. (\S+)$", line.rstrip("\n"))
            if m: out[m.group(1).replace(" - should panic", "")] = m.group(2)
        tests[key] = out
    return counts, tests
bc, bt = load(sys.argv[1]); cc, ct = load(sys.argv[2])
assert sorted(bc) == sorted(cc) and sorted(bt) == sorted(ct), "manifest sets differ"
identical, rows = 0, []
for man in sorted(bc):
    key = man[: -len("/Cargo.toml")].replace("/", "_")
    b, c = bt[key], ct[key]
    removed = sorted(f"{t} ... {b[t]}" + ("" if t not in c else f" -> {c[t]}") for t in b if c.get(t) != b[t])
    added = [t for t in c if t not in b]
    added_bad = sorted(f"{t} ... {c[t]}" for t in added if c[t] != "ok")
    if bc[man] == cc[man] and not removed and not added:
        identical += 1; continue
    rows.append({"manifest": man, "baseline": bc[man], "candidate": cc[man], "removed_or_changed": removed,
                 "added_ok": sum(1 for t in added if c[t] == "ok"), "added_not_ok": added_bad,
                 "failing_candidate": sorted(t for t in c if c[t] == "FAILED")})
print(f"manifests {len(bc)}; identical {identical}; differing {len(rows)}")
for r in rows:
    print(f"\n== {r['manifest']}: baseline rc/passed/failed/ignored {r['baseline']}  candidate {r['candidate']}")
    print(f"   added ok: {r['added_ok']}")
    for x in r["added_not_ok"]: print(f"   added, not ok: {x}")
    for x in r["removed_or_changed"]: print(f"   removed or changed: {x}")
    for x in r["failing_candidate"]: print(f"   failing in candidate: {x}")
