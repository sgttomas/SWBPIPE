#!/usr/bin/env python3
"""RV111: summarize mutants.json, and diff each survivor's dumps against the candidate's.
Usage: rv111_mutant_summary.py <scratch dir> <out.json>"""
import json
import os
import sys

S, OUT = sys.argv[1], sys.argv[2]
res = json.load(open(f"{S}/mutants/mutants.json"))


def load(path):
    out = {}
    if not os.path.exists(path):
        return None
    with open(path, encoding="utf-8") as fh:
        for line in fh:
            parts = line.rstrip("\n").split("\t")
            out[parts[0]] = parts[1]
    return out


cand = {f: load(f"{S}/dumps/cand/{f}") for f in ("ee_point.tsv", "ee_interval.tsv", "run.tsv", "probe.tsv")}
summary = {}
for mid, r in sorted(res.items()):
    failed = sorted(set(sum((r.get(k, {}).get("failed", []) for k in ("ee_lib", "runner_pp", "runner_all")), [])))
    entry = {"killed": r.get("killed"), "compile_error": r.get("compile_error"), "killed_by": failed}
    mdir = f"{S}/dumps/mut_{mid}"
    if os.path.isdir(mdir):
        diffs = {}
        for f, cmap in cand.items():
            m = load(f"{mdir}/{f}")
            if m is None:
                diffs[f] = "missing"
                continue
            d = [k for k in cmap if m.get(k) != cmap[k]]
            diffs[f] = {"lines": len(cmap), "differ": len(d), "examples": d[:5]}
        entry["differential_vs_candidate"] = diffs
    summary[mid] = entry
json.dump(summary, open(OUT, "w"), indent=1, sort_keys=True)
for mid, e in summary.items():
    dv = e.get("differential_vs_candidate")
    dtxt = "" if dv is None else " | diff " + ", ".join(
        f"{k}:{v['differ'] if isinstance(v, dict) else v}" for k, v in dv.items())
    print(f"{mid:45s} killed={e['killed']} n={len(e['killed_by'])} {('compile_error' if e['compile_error'] else '')}{dtxt}")
