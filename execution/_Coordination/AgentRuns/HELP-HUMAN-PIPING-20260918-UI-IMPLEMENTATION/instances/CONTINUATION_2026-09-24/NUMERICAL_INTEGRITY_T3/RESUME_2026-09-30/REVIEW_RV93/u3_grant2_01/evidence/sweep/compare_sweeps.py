#!/usr/bin/env python3
"""RV93: compare the four route sweeps. Rows are `input route mode result`; for the retained
entries the result carries the publication kind, its sha256, the envelope's sha256, the W1 result
and the full admission report (Debug)."""
import collections, json, sys
from pathlib import Path
D = Path(sys.argv[1])
def load(tag):
    rows = {}
    for line in (D / f"{tag}.tsv").read_text().splitlines():
        name, route, mode, rest = line.split("\t", 3)
        rows[(name, route, mode)] = rest
    return rows
s = {t: load(t) for t in ["cand_reg", "base_reg", "cand_stale", "base_stale"]}
out = {}
for a, b in [("cand_reg", "base_reg"), ("cand_stale", "base_stale")]:
    assert s[a].keys() == s[b].keys()
    diff = [k for k in s[a] if s[a][k] != s[b][k]]
    out[f"{a}_vs_{b}"] = {"rows": len(s[a]), "differing": len(diff), "first": diff[:5]}
# registered vs Stale: which rows differ, and how
regs, stal = s["cand_reg"], s["cand_stale"]
kinds = collections.Counter()
detail = []
for k in regs:
    if regs[k] == stal[k]:
        kinds[(k[1], "identical")] += 1
        continue
    r, t = regs[k], stal[k]
    if k[1] in ("direct", "headless"):
        rp = dict(x.split("=", 1) for x in r.split("\t")[1:4]); tp = dict(x.split("=", 1) for x in t.split("\t")[1:4])
        same_pub = rp["pub"] == tp["pub"]; same_env = rp["env"] == tp["env"]
        kinds[(k[1], f"{r.split(chr(9))[0]}:{'same-pub' if same_pub else 'pub-differs'}:{'same-env' if same_env else 'env-differs'}:w1={rp['w1'][:60]}")] += 1
        detail.append([*k, r.split("\t")[0], rp["w1"][:90], same_pub, same_env])
    else:
        kinds[(k[1], "DIFFERS")] += 1
        detail.append([*k, "NON-RETAINED ROUTE DIFFERS"])
out["reg_vs_stale_kinds"] = {f"{a}|{b}": n for (a, b), n in sorted(kinds.items())}
out["reg_vs_stale_rows"] = detail
# the Stale admission reports: every retained row reports Stale
stale_profiles = collections.Counter()
for k, v in stal.items():
    if k[1] in ("direct", "headless"):
        stale_profiles["Stale" if "profile: Stale" in v else ("Registered" if "profile: Registered" in v else v.split("\t")[0][:30])] += 1
reg_profiles = collections.Counter()
for k, v in regs.items():
    if k[1] in ("direct", "headless"):
        reg_profiles["Stale" if "profile: Stale" in v else ("Registered" if "profile: Registered" in v else v.split("\t")[0][:30])] += 1
out["profiles"] = {"registered_build": dict(reg_profiles), "stale_build": dict(stale_profiles)}
# Stale: every retained publication equals the value route's bytes (sha of envelope == value sha)
mism = []
for (name, route, mode), v in stal.items():
    if route in ("direct", "headless") and not v.startswith(("ERR", "PANIC")):
        fields = dict(x.split("=", 1) for x in v.split("\t")[1:4])
        value = stal[(name, "value", mode)]
        if v.split("\t")[0] != "Ordinary" or fields["pub"] != value or fields["w1"] != "None":
            mism.append([name, route, mode, v[:120], value[:70]])
out["stale_retained_not_plain_value"] = mism
# Registered: classify each Direct publication against the value route
cls = collections.Counter(); reg_rows = []
for (name, route, mode), v in regs.items():
    if route != "direct":
        continue
    value = regs[(name, "value", mode)]
    if v.startswith(("ERR", "PANIC")):
        cls["error (" + ("same as value" if value == v else "value: " + value[:20]) + ")"] += 1; continue
    f = dict(x.split("=", 1) for x in v.split("\t")[1:4])
    kind = v.split("\t")[0]
    if kind == "Successor": c = "successor"
    elif f["pub"] == value: c = "exact"
    else: c = "other(notice?)"
    cls[c] += 1
    reg_rows.append([name, mode, c, f["w1"][:100], "env==value" if f["env"] == value else "ENV!=VALUE"])
out["registered_direct_classes"] = dict(cls)
out["registered_direct_rows"] = reg_rows
print(json.dumps(out, indent=1))
