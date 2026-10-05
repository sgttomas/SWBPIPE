#!/usr/bin/env python3
"""I66 U7 slice F, the oracle-diff control. For each language, compare the base lane's dump
(the U7 base head, flags off) with the candidate's (flags on), field by field, and check:
(1) only `numerical_eligible`, the reader's standing label, the carrier token and the
summary's `withheld` counts change; (2) they change on exactly the oracle's set; (3) every
candidate value equals slice A's oracle (`oracle_post_u7.json`) and every base value its
pre-U7 run (`oracle_pre_u7.json`), on every key the oracle covers.
The summary's `withheld` follows the standing of the input's own invocation cases (the
summary takes no requested refs), so it changes iff the reader is eligible after U7 and the
token for that invocation's cases is eligible.
Usage: compare_oracle.py ORACLE_PRE ORACLE_POST PY_BASE PY_CAND RS_BASE RS_CAND OUT_TXT"""
import json, sys

pre, post, pyb, pyc, rsb, rsc, out = sys.argv[1:8]
PRE, POST = json.load(open(pre)), json.load(open(post))
EXPECT_FIELDS = {"numerical_eligible", "reader_standing", "carrier_token", "summary"}


def oracle(o, key):
    parts = key.split("|")
    if parts[0] == "cases":
        return o["corpus"]["cases"][parts[1]][parts[2]]
    if parts[0] in ("must_pass", "mutations"):
        return o["corpus"][parts[0]][parts[1]]
    if parts[0] == "carrier":
        return o["carrier_cases"][parts[1]]
    if parts[0] == "milestone":
        return o["milestone"][parts[1]][parts[2]]
    return None


def sibling_with_invocation(key):
    """The input with the same source and invocation and the invocation's own cases."""
    parts = key.split("|")
    if parts[0] == "milestone":
        return f"milestone|{parts[1]}|with_invocation"
    if parts[0] == "carrier" and parts[1].split(":")[1] in ("no_requested", "other_requested", "invocation"):
        return f"carrier|{parts[1].split(':')[0]}:invocation"
    if parts[0] == "declared" and parts[1] == "D-U7-4:ts_requires_live_native_capture":
        return f"milestone|{parts[3].split('_', 1)[1]}|with_invocation"
    return key


def expected_changes(key):
    """The fields the oracle says change at U7 for this input."""
    if key.startswith("declared|D-U7-4"):
        o_pre, o_post = PRE["milestone"][key.split("|")[3].split("_", 1)[1]]["with_invocation"], POST["milestone"][key.split("|")[3].split("_", 1)[1]]["with_invocation"]
    elif key.startswith("declared|"):
        return set()                       # slice A: the five U6 entries are unchanged at U7
    else:
        o_pre, o_post = oracle(PRE, key), oracle(POST, key)
    fields = {f for f in ("numerical_eligible", "carrier_token") if o_pre[f] != o_post[f]}
    if o_pre["reader_standing"] != o_post["reader_standing"]:
        fields.add("reader_standing")
    sib = sibling_with_invocation(key)
    sib_post = oracle(POST, sib) if not sib.startswith("declared|") else o_post
    sib_pre = oracle(PRE, sib) if not sib.startswith("declared|") else o_pre
    if o_post["numerical_eligible"] and sib_post["carrier_token"] == "numerically_eligible" and sib_pre["carrier_token"] != "numerically_eligible":
        fields.add("summary")
    return fields


lines, ok = [], True
for lang, bpath, cpath in (("python", pyb, pyc), ("rust", rsb, rsc)):
    B, C = json.load(open(bpath)), json.load(open(cpath))
    assert B.keys() == C.keys()
    changed = {k: {f for f in set(B[k]) | set(C[k]) if B[k].get(f) != C[k].get(f)} for k in B}
    other = {k: v - EXPECT_FIELDS for k, v in changed.items() if v - EXPECT_FIELDS}
    mismatch = {}
    for k in B:
        want = expected_changes(k)
        got = changed[k] if lang == "python" else changed[k] - {"reader_standing"}
        if lang == "rust":
            want = want - {"reader_standing"}
        if got != want:
            mismatch[k] = {"changed": sorted(got), "oracle": sorted(want)}
    # Values against the oracle where it covers the key.
    values = []
    for k in B:
        for side, dump, o in (("base", B, PRE), ("cand", C, POST)):
            ref = oracle(o, k) if not k.startswith("declared|") else None
            if ref is None:
                continue
            row = dump[k]
            pairs = [("carrier_token", "carrier_token")]
            if row.get("reader") == "pass":
                pairs += [("numerical_eligible", "numerical_eligible"), ("invocation_bound", "invocation_bound")]
                if lang == "python":
                    pairs.append(("reader_standing", "reader_standing"))
            for f_dump, f_oracle in pairs:
                if row.get(f_dump) != ref[f_oracle]:
                    values.append((k, side, f_dump, row.get(f_dump), ref[f_oracle]))
    n_changed = sum(1 for v in changed.values() if v)
    by_set = {}
    for k, v in changed.items():
        if v:
            by_set.setdefault(k.split("|")[0], 0)
            by_set[k.split("|")[0]] += 1
    lines.append(f"{lang}: {len(B)} inputs; {n_changed} changed ({by_set}); fields outside the eligible path changed: {len(other)} {other}; "
                 f"changed set differs from the oracle's: {len(mismatch)} {mismatch}; values differing from the oracle: {len(values)} {values[:10]}")
    ok = ok and not other and not mismatch and not values
    eligible_keys = sorted(k for k in C if C[k].get("carrier_token") == "numerically_eligible")
    lines.append(f"{lang}: candidate inputs standing numerically_eligible: {len(eligible_keys)}")
    lines.append(f"{lang}: changed keys: {sorted(k for k, v in changed.items() if v)}")
lines.append("RESULT: " + ("PASS" if ok else "FAIL"))
open(out, "w").write("\n".join(lines) + "\n")
print("\n".join(lines)[:6000])
