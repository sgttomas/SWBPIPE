#!/usr/bin/env python3
"""I85 B1-SP I3: the suite delta, base (2ba2f81863) against head, from sp_suites.sh's `.outcomes`
files (one line per test: `<target> :: test <name> ... <ok|FAILED|ignored>`). For each pair:
tests removed, tests added, and status changes; then the head's registered against Stale, and the
expected delta checked exactly.
Usage: suite_delta.py <suites dir>"""
import os, re, sys
L = sys.argv[1]
def load(name):
    out = {}
    for line in open(os.path.join(L, name + ".outcomes")):
        m = re.match(r"^(.*?) :: test (\S+) \.\.\. (ok|FAILED|ignored)", line)
        if m: out[(m.group(1), m.group(2))] = m.group(3)
    return out
def counts(d): return {s: sum(1 for v in d.values() if v == s) for s in ("ok", "FAILED", "ignored")}
def short(k): return f"{k[0]} :: {k[1]}"
F = "retained_facade_tests::"
RENAMED_FROM = {("src/lib.rs", F + "b1_sp_w_c2_direct_entry_counts_one_run_through_g_c")}
ADDED = {("src/lib.rs", F + n) for n in ["b1_sp_w_c2_direct_entry_publishes_the_pinned_successor", "b1_sp_w_c2_fixtures_are_the_live_successors",
                                       "b1_sp_sf2_selected_not_first_and_two_selected_pins", "b1_sp_constructor_ordinal_is_the_authored_index"]}
NOW_PASSING = {("src/lib.rs", F + n) for n in ["b1_sp_r3p_1_one_case_in_a_runs_on_its_own_slot", "b1_sp_w_c2_through_retained_w1_publishes_t12",
                                             "b1_sp_w_c2_transaction_faults_and_abandonment", "b1_sp_w_c2_transaction_outcomes_and_ordinal_mapping"]}
ok_all = True
for base, head, expect_added, expect_removed, expect_fixed in [
        ("base_reg_pp", "cand_reg_pp", ADDED, RENAMED_FROM, NOW_PASSING),
        ("base_stale_pp", "cand_stale_pp", ADDED, RENAMED_FROM, NOW_PASSING),
        ("base_reg_runner", "cand_reg_runner", set(), set(), set())]:
    b, h = load(base), load(head)
    removed = sorted(set(b) - set(h)); added = sorted(set(h) - set(b))
    changed = sorted(k for k in set(b) & set(h) if b[k] != h[k])
    print(f"== {base} -> {head}")
    print(f"   base {counts(b)}; head {counts(h)}")
    for k in removed: print(f"   - {short(k)} ({b[k]})")
    for k in added: print(f"   + {short(k)} ({h[k]})")
    for k in changed: print(f"   ~ {short(k)}: {b[k]} -> {h[k]}")
    fixed = {k for k in changed if b[k] == "FAILED" and h[k] == "ok"}
    good = (set(removed) == expect_removed and set(added) == expect_added and all(h[k] == "ok" for k in added)
            and fixed == expect_fixed and set(changed) == expect_fixed)
    ok_all &= good
    print(f"   exactly the expected delta: {good}")
r, s = load("cand_reg_pp"), load("cand_stale_pp")
diff = sorted(k for k in set(r) | set(s) if r.get(k) != s.get(k))
print("== head: registered against Stale")
for k in diff: print(f"   {short(k)}: registered {r.get(k)}, Stale {s.get(k)}")
print("   identical" if not diff else f"   {len(diff)} difference(s)")
print("ALL AS EXPECTED" if ok_all else "UNEXPECTED DELTA")
