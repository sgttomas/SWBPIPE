#!/usr/bin/env python3
"""I67 U7 slice F part 2, the TS oracle-diff control. Compares the TS dump of the base lane
(the U7 base head 12a849a7bd, TS flag off) with the candidate's (flag on) over I66's 380 inputs,
and checks against slice A's oracle (oracle_pre_u7.json, oracle_post_u7.json):
 (1) only the eligible-path fields change: the reader's numerical_eligible and standing, the
     `live` and `ipc` standings (token, findings, status, eligible) and the summaries
     (`withheld` only); the reader's outcome, classes, invocation_bound, transport checks and
     binding refusals never change;
 (2) the reader fields and the `live` token change on exactly the oracle's set (I66's mapping:
     a declared D-U7-4 input is the milestone with its invocation);
 (3) the `ipc` token changes on exactly the oracle's set less the D-U7-4 inputs, where TS's
     declared difference keeps needs_recompute (its finding moves from NOT_NUMERICALLY_ELIGIBLE
     to NATIVE_CAPTURE_REQUIRED);
 (4) every base and candidate value equals the oracle's pre and post value on every key it covers.
Usage: compare_ts_oracle.py ORACLE_PRE ORACLE_POST TS_BASE TS_CAND OUT_TXT"""
import json, sys

pre, post, tb, tc, out = sys.argv[1:6]
PRE, POST = json.load(open(pre)), json.load(open(post))
B, C = json.load(open(tb)), json.load(open(tc))
D74 = "declared|D-U7-4:ts_requires_live_native_capture|"


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
    if key.startswith(D74):
        return o["milestone"][parts[3].split("_", 1)[1]]["with_invocation"]
    return None


def flat(row):
    f = {}
    for k, v in row.items():
        if isinstance(v, dict):
            for kk, vv in v.items():
                f[f"{k}.{kk}"] = vv
        elif k.endswith("summary") and isinstance(v, list):
            f[f"{k}.counts"] = [{x: y for x, y in c.items() if x != "withheld"} for c in v]
            f[f"{k}.withheld"] = [c["withheld"] for c in v]
        else:
            f[k] = v
    return f


ALLOWED = {"numerical_eligible", "reader_standing", "live.token", "live.findings", "live.status", "live.eligible", "live_summary.withheld",
           "ipc.token", "ipc.findings", "ipc.status", "ipc.eligible", "ipc_summary.withheld"}
assert B.keys() == C.keys() and len(B) == 380
changed = {k: {f for f in set(flat(B[k])) | set(flat(C[k])) if flat(B[k]).get(f) != flat(C[k]).get(f)} for k in B}
other = {k: sorted(v - ALLOWED) for k, v in changed.items() if v - ALLOWED}
o_elig = {k for k in B if oracle(PRE, k) and oracle(PRE, k)["numerical_eligible"] != oracle(POST, k)["numerical_eligible"]}
o_token = {k for k in B if oracle(PRE, k) and oracle(PRE, k)["carrier_token"] != oracle(POST, k)["carrier_token"]}
got_elig = {k for k, v in changed.items() if "numerical_eligible" in v}
got_rstand = {k for k, v in changed.items() if "reader_standing" in v}
got_live = {k for k, v in changed.items() if "live.token" in v}
got_ipc = {k for k, v in changed.items() if "ipc.token" in v}
want_ipc = {k for k in o_token if k in C and "ipc" in C[k] and not k.startswith(D74)}
d74_findings = {k: (B[k]["ipc"]["findings"], C[k]["ipc"]["findings"], B[k]["ipc"]["token"], C[k]["ipc"]["token"]) for k in B if k.startswith(D74)}
withheld_live = {k for k, v in changed.items() if "live_summary.withheld" in v}
withheld_ipc = {k for k, v in changed.items() if "ipc_summary.withheld" in v}


def sibling_with_invocation(key):
    """I66's rule: the summary follows the input's own invocation cases, so its sibling is the
    input with the same source and invocation and the invocation's own cases."""
    parts = key.split("|")
    if parts[0] == "milestone":
        return f"milestone|{parts[1]}|with_invocation"
    if parts[0] == "carrier" and parts[1].split(":")[1] in ("no_requested", "other_requested", "invocation"):
        return f"carrier|{parts[1].split(':')[0]}:invocation"
    if key.startswith(D74):
        return f"milestone|{parts[3].split('_', 1)[1]}|with_invocation"
    return key


want_withheld_live = {k for k in o_elig if sibling_with_invocation(k) in o_token}
# The ipc summary takes TS's standing model; TS's summary reads D2 4.9.4 without the live
# capture binding, so it also changes on the four D-U7-4 inputs (reported, see RETURN).
want_withheld_ipc = got_ipc | {k for k in B if k.startswith(D74)}
# Values against the oracle (pre for base, post for candidate) where it covers the key.
values = []
for side, dump, o in (("base", B, PRE), ("cand", C, POST)):
    for k, row in dump.items():
        ref = oracle(o, k)
        if ref is None:
            continue
        if row["live"]["token"] != ref["carrier_token"]:
            values.append((k, side, "live.token", row["live"]["token"], ref["carrier_token"]))
        if row.get("reader") == "pass":
            for f in ("numerical_eligible", "invocation_bound", "reader_standing"):
                if row[f] != ref[f]:
                    values.append((k, side, f, row[f], ref[f]))
        if "ipc" in row and not k.startswith(D74) and row["ipc"]["token"] != ref["carrier_token"]:
            values.append((k, side, "ipc.token", row["ipc"]["token"], ref["carrier_token"]))
lines = [
    f"inputs: {len(B)}; changed: {sum(1 for v in changed.values() if v)}",
    f"(1) fields outside the eligible path changed: {len(other)} {other}",
    f"(2) oracle numerical_eligible set: {len(o_elig)}; reader numerical_eligible changed: {len(got_elig)}; equal: {got_elig == o_elig}; reader standing changed equal: {got_rstand == o_elig}",
    f"    oracle carrier_token set: {len(o_token)}; live token changed: {len(got_live)}; equal: {got_live == o_token}",
    f"    by set: {sorted({k.split('|')[0] + ('|D-U7-4' if k.startswith(D74) else '') for k in o_token})}; counts: " + str({p: sum(1 for k in o_token if k.split('|')[0] == p) for p in ('cases', 'must_pass', 'mutations', 'carrier', 'milestone', 'declared')}),
    f"(3) ipc token changed: {len(got_ipc)} {sorted(got_ipc)}; want {len(want_ipc)}; equal: {got_ipc == want_ipc}",
    f"    D-U7-4 ipc (base findings, cand findings, base token, cand token): {d74_findings}",
    f"    withheld changed (live): {len(withheld_live)}; I66's rule (reader eligible after U7 and the invocation-cases sibling's token changes): {len(want_withheld_live)}; equal: {withheld_live == want_withheld_live}",
    f"    withheld changed (ipc): {len(withheld_ipc)}; want (the ipc-token set plus the 4 D-U7-4 inputs): {len(want_withheld_ipc)}; equal: {withheld_ipc == want_withheld_ipc}",
    f"    D-U7-4 ipc summary withheld (base, cand) with TS standing needs_recompute: {[(k.split('|')[2] + '|' + k.split('|')[3], [c['withheld'] for c in B[k]['ipc_summary']], [c['withheld'] for c in C[k]['ipc_summary']]) for k in sorted(B) if k.startswith(D74)]}",
    f"(4) values differing from the oracle: {len(values)} {values[:12]}",
    f"    candidate live numerically_eligible: {sum(1 for r in C.values() if r['live']['token'] == 'numerically_eligible')}; ipc numerically_eligible: {sorted(k for k, r in C.items() if r.get('ipc', {}).get('token') == 'numerically_eligible')}",
    f"changed keys: {sorted(k for k, v in changed.items() if v)}",
]
ok = not other and got_elig == o_elig and got_rstand == o_elig and got_live == o_token and got_ipc == want_ipc and not values and withheld_live == want_withheld_live and withheld_ipc == want_withheld_ipc
lines.append("RESULT: " + ("PASS" if ok else "FAIL"))
open(out, "w").write("\n".join(lines) + "\n")
print("\n".join(lines)[:8000])
