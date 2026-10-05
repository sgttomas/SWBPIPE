#!/usr/bin/env python3
"""I61 U3 grant 2: controls 1 and 2 over the fixture sweeps.

Usage: compare_sweeps.py BASE_B54 UNREG_B43 CAND_STALE CAND_REG OUT_JSON
  BASE_B54   the 324-output sweep at b54caba7ab (pre-U3; grant 1's harness: envelope sha, report)
  UNREG_B43  this grant's harness at b43378d90a (unregistered: no profile, every report Missing)
  CAND_STALE this grant's harness on the candidate built Stale (a non-empty RUSTFLAGS)
  CAND_REG   this grant's harness on the candidate built Registered
Columns of this grant's harness: rel, route, mode, then for the plain routes the envelope sha (or
ERR:/TYPED_PARSE_ERR/PANIC), and for the retained routes: publication sha, class, (profile, first
refusing clause), private W1 result.
"""
import json, sys
b54, b43, stale, reg, out = sys.argv[1:6]
def load(path):
    rows = {}
    for line in open(path).read().splitlines():
        rel, route, mode, rest = line.split("\t", 3)
        rows[(rel, route, mode)] = rest.split("\t")
    return rows
B54, B43, ST, RG = load(b54), load(b43), load(stale), load(reg)
assert B54.keys() == B43.keys() == ST.keys() == RG.keys(), "the same 324 outputs"
first = lambda cols: cols[0].split(" ")[0] if not cols[0].startswith("ERR:") else cols[0]
report = {"outputs": len(RG), "control_1": {}, "control_2": {}}
# Control 1: the unregistered and Stale builds publish base's bytes on every route.
c1 = {"stale_vs_b54_bytes": [], "unreg_vs_b54_bytes": [], "stale_vs_unreg_meta": [], "stale_classes": {}}
for k in RG:
    if first(ST[k]) != first(B54[k]): c1["stale_vs_b54_bytes"].append(k)
    if first(B43[k]) != first(B54[k]): c1["unreg_vs_b54_bytes"].append(k)
    if k[1].startswith("retained_") and not ST[k][0].startswith("ERR:"):
        c1["stale_classes"][ST[k][1]] = c1["stale_classes"].get(ST[k][1], 0) + 1
        # Only the build status differs: Missing (nothing registered) against Stale.
        if ST[k][1:] != [c.replace("Missing", "Stale") for c in B43[k][1:]]:
            c1["stale_vs_unreg_meta"].append(k)
report["control_1"] = {"stale_bytes_differ_from_b54": len(c1["stale_vs_b54_bytes"]), "unreg_bytes_differ_from_b54": len(c1["unreg_vs_b54_bytes"]),
                       "stale_meta_differs_from_unreg_beyond_status": len(c1["stale_vs_unreg_meta"]), "stale_retained_classes": c1["stale_classes"],
                       "differences": [list(k) for k in c1["stale_vs_b54_bytes"] + c1["unreg_vs_b54_bytes"] + c1["stale_vs_unreg_meta"]]}
# Control 2: registered, every route but Direct publishes base's bytes; Direct only as ruled.
c2 = {"non_direct_bytes_differ": [], "direct": {"exact": [], "notice": [], "successor": []}, "direct_exact_bytes_differ": [], "errors_differ": []}
for k in RG:
    rel, route, mode = k
    if route != "retained_direct":
        if first(RG[k]) != first(B54[k]): c2["non_direct_bytes_differ"].append(k)
        continue
    if RG[k][0].startswith("ERR:") or B54[k][0].startswith("ERR:"):
        if RG[k][0] != B54[k][0]: c2["errors_differ"].append(k)
        continue
    cls, meta, cause = RG[k][1], RG[k][2], RG[k][3]
    c2["direct"][cls].append([rel, mode, meta, cause])
    if cls == "exact" and first(RG[k]) != first(B54[k]): c2["direct_exact_bytes_differ"].append(k)
report["control_2"] = {"non_direct_bytes_differ_from_b54": len(c2["non_direct_bytes_differ"]), "errors_differ": len(c2["errors_differ"]),
                       "direct_exact_bytes_differ_from_b54": len(c2["direct_exact_bytes_differ"]),
                       "direct_counts": {c: len(v) for c, v in c2["direct"].items()},
                       "direct_successor": c2["direct"]["successor"], "direct_notice": c2["direct"]["notice"],
                       "direct_exact_admitted": [r for r in c2["direct"]["exact"] if "None)" in r[2]],
                       "differences": [list(k) for k in c2["non_direct_bytes_differ"] + c2["errors_differ"] + c2["direct_exact_bytes_differ"]]}
json.dump(report, open(out, "w"), indent=1); open(out, "a").write("\n")
ok = (not report["control_1"]["differences"] and not report["control_2"]["differences"])
print(json.dumps({k: v for k, v in report["control_1"].items() if k != "differences"}))
print(json.dumps({k: v for k, v in report["control_2"].items() if k not in ("differences", "direct_successor", "direct_notice", "direct_exact_admitted")}))
for r in report["control_2"]["direct_successor"]: print("SUCCESSOR", r)
for r in report["control_2"]["direct_notice"]: print("NOTICE", r)
for r in report["control_2"]["direct_exact_admitted"]: print("EXACT_ADMITTED", r)
print("CONTROLS 1 AND 2 HOLD" if ok else "DIFFERENCES: see " + out)
