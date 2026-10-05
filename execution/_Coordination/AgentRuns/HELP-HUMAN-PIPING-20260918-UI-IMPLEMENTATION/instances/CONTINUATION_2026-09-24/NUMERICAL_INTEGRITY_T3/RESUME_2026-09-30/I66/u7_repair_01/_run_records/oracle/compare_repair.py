#!/usr/bin/env python3
"""I66 U7 repair (RV94 S-1): the oracle diff rerun. For each language, compare the base dump
(`ffe65ef203`: the summary Current when the token with the invocation's own cases is eligible)
with the candidate's (the summary Current only when the token with the caller's requested refs
is eligible, as TS since `e5e1693ceb`). Checks:
(1) only `summary` changes;
(2) it changes on exactly the inputs whose token with the invocation's own cases is eligible
    while the token with the input's requested refs is not;
(3) in the candidate, on every passing input, `withheld` is the Current count iff the
    carrier token is `numerically_eligible` (the alignment); in the base, iff the token with
    the invocation's own cases is.
Usage: compare_repair.py PY_BASE PY_CAND RS_BASE RS_CAND OUT_TXT"""
import json, sys

pyb, pyc, rsb, rsc, out = sys.argv[1:6]


def is_current(summary):
    """Current iff every case withholds only absolute and not-covered rows."""
    return all(case["withheld"] == case["absolute_verified"] + case["not_covered"] for case in summary)


def distinguishes(summary):
    """Whether Current and not-Current counts differ for this statement."""
    return any(case["relative_verified"] + case["input_derived"] for case in summary)


lines, ok = [], True
for lang, bpath, cpath in (("python", pyb, pyc), ("rust", rsb, rsc)):
    B, C = json.load(open(bpath)), json.load(open(cpath))
    assert B.keys() == C.keys()
    changed = {k: sorted(f for f in set(B[k]) | set(C[k]) if B[k].get(f) != C[k].get(f)) for k in B}
    other = {k: v for k, v in changed.items() if v and v != ["summary"]}
    expected = {k for k in B if B[k].get("token_with_own_cases") == "numerically_eligible" and C[k].get("carrier_token") != "numerically_eligible"
                 and C[k].get("summary") and distinguishes(C[k]["summary"])}
    got = {k for k, v in changed.items() if v == ["summary"]}
    align_c = [k for k in C if C[k].get("summary") and distinguishes(C[k]["summary"]) and is_current(C[k]["summary"]) != (C[k]["carrier_token"] == "numerically_eligible")]
    align_b = [k for k in B if B[k].get("summary") and distinguishes(B[k]["summary"]) and is_current(B[k]["summary"]) != (B[k]["token_with_own_cases"] == "numerically_eligible")]
    lines.append(f"{lang}: {len(B)} inputs; changed {len(got)} (summary only): {sorted(got)}")
    lines.append(f"{lang}: fields other than summary changed: {len(other)} {other}")
    lines.append(f"{lang}: changed set equals the expected set: {got == expected} (expected {len(expected)}; missing {sorted(expected - got)}; extra {sorted(got - expected)})")
    lines.append(f"{lang}: candidate summary Current iff token eligible, violations: {len(align_c)} {align_c[:5]}; base rule (own cases) violations: {len(align_b)} {align_b[:5]}")
    for k in sorted(got):
        lines.append(f"{lang}:   {k}: withheld {[c['withheld'] for c in B[k]['summary']]} -> {[c['withheld'] for c in C[k]['summary']]} (token {C[k]['carrier_token']})")
    ok = ok and not other and got == expected and not align_c and not align_b
lines.append("RESULT: " + ("PASS" if ok else "FAIL"))
open(out, "w").write("\n".join(lines) + "\n")
print("\n".join(lines))
