#!/usr/bin/env python3
"""I61 U7 slice L: the standing-token table on the live successors, three languages, against slice A's
oracle (with/without invocation) and the case file's D-U7-4 expectations (declared difference).
Usage: token_table.py TOKENS_DIR ORACLE_POST_U7 CARRIER_CASES OUT_MD"""
import json, sys
D, ORACLE, CASES, OUT = sys.argv[1:5]
py, rs, ts = (json.load(open(f"{D}/{l}_tokens.json")) for l in ("py", "rs", "ts"))
oracle = json.load(open(ORACLE))["milestone"]
cases = json.load(open(CASES))
d74 = next(e for e in cases["declared_differences"] if e["id"].startswith("D-U7-4"))
want74 = {f["label"]: f["expected"] for f in d74["forms"]}
rows, misses = [], []
forms = [("with_invocation", "milestone with its invocation"), ("without_invocation", "milestone without it"),
         ("d_u7_4_no_native_capture", "D-U7-4 (a) invocation_without_native_capture"),
         ("d_u7_4_stale_current_model", "D-U7-4 (b) stale_current_model_same_case_ids")]
for mode in ("sparse_interactive", "dense_scrutiny"):
    for key, label in forms:
        got = {"python": py[mode]["token"][key], "rust": rs[mode]["token"][key], "typescript": ts[mode]["token"][key]["token"]}
        if key in ("with_invocation", "without_invocation"):
            exp = {l: oracle[mode][key]["carrier_token"] for l in got}; basis = "slice A oracle"
        else:
            f = want74["invocation_without_native_capture" if key.endswith("capture") else "stale_current_model_same_case_ids"]
            exp = {l: f[l]["standing"] for l in got}; basis = "case file D-U7-4 (declared)"
        ok = got == exp
        if not ok: misses.append((mode, key, got, exp))
        finding = ts[mode]["token"][key]["findings"]
        rows.append(f"| {mode} | {label} | {got['python']} | {got['rust']} | {got['typescript']}{' (' + finding[0] + ')' if finding else ''} | {basis}: {'match' if ok else 'MISMATCH'} |")
    rows.append(f"| {mode} | TS job IPC, with its invocation | — | — | {ts[mode]['token']['with_invocation_job']['token']} | slice A oracle: {'match' if ts[mode]['token']['with_invocation_job']['token'] == oracle[mode]['with_invocation']['carrier_token'] else 'MISMATCH'} |")
hdr = "| mode | form | Python | Rust | TypeScript | expected |\n|---|---|---|---|---|---|\n"
text = hdr + "\n".join(rows) + "\n"
w = []
for mode in ("sparse_interactive", "dense_scrutiny"):
    w.append(f"| {mode} | {py[mode]['withheld']['with_invocation']} / {py[mode]['withheld']['without_invocation']} | {rs[mode]['withheld']['with_invocation']} / {rs[mode]['withheld']['without_invocation']} | {ts[mode]['withheld']['with_invocation']} / {ts[mode]['withheld']['without_invocation']} | {ts[mode]['withheld']['d_u7_4_no_native_capture']} / {ts[mode]['withheld']['d_u7_4_stale_current_model']} |")
    r = {"python": py[mode]["reader"], "rust": rs[mode]["reader"], "typescript": ts[mode]["reader"]}
    w2 = all(r[l]["with_invocation"]["numerical_eligible"] and not r[l]["without_invocation"]["numerical_eligible"] for l in r)
    if not w2: misses.append((mode, "reader", r))
text += "\n| mode | Python withheld (with / without invocation) | Rust | TypeScript | TypeScript D-U7-4 (a) / (b) |\n|---|---|---|---|---|\n" + "\n".join(w) + "\n"
text += f"\nReader-level `numerical_eligible`: true with the invocation and false without it, in all three languages and both modes: {not any(m[1] == 'reader' for m in misses)}.\n"
text += f"\nMismatches: {misses if misses else 'none'}\n"
open(OUT, "w").write(text); print(text)
