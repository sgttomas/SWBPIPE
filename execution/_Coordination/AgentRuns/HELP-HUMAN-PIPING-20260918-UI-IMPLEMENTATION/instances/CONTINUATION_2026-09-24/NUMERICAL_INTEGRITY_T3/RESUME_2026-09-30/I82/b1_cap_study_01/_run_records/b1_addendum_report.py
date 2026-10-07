"""I82 B1-S ADDENDUM_01: the options within M <= 12 GiB (owner, 2026-10-07) against P1 (stdlib only).

Reads WT/scratch/i82_b1_study/mc_runs/<tag>/inbuild.json and profile_tree.json for every point of
the given points json, and writes one JSON with, per point and mode: E_mov,max + R, E_mov,max (heap: requested +
moving bytes, without R), the binding phase and its terms, the smallest M meeting 0.9 M in both modes,
the smallest M meeting 0.9 M with a 5 % text-error budget, and the text-error budget (margin to 0.9 M over
TAV_W, QUAL section 3's measure) at 5.25, 10, 11 and 12 GiB. P1 is also given as its two tiers' maximum.
Usage: python3 b1_addendum_report.py <points json> <mc_runs dir> <out json>
"""
import json, os, sys

pts = json.load(open(sys.argv[1])); runs = sys.argv[2]
GiB = 2**30; R = 64 << 20
CANDS = {"5.25 GiB": 21 * 256 * 2**20, "10 GiB": 10 * GiB, "11 GiB": 11 * GiB, "12 GiB": 12 * GiB}
def mmin(e):
    return -(-e * 10 // 9)
out = {"candidates": CANDS, "points": {}}
for p in pts:
    d = json.load(open(os.path.join(runs, p["tag"], "inbuild.json")))
    tree = json.load(open(os.path.join(runs, p["tag"], "work", "profile_tree.json")))
    row = {"caps": p["caps"], "D": d["summary"]["D"], "D_converged": d["summary"].get("D_converged"),
           "text_complete": d["summary"]["text_complete"], "missing_atoms": d["missing_atoms"], "D_env": d["text_atoms"]["D_env"]}
    worst = max(d["modes"][m]["E_mov_plus_R"] for m in ("sparse", "dense"))
    row["M_min_both"] = mmin(worst)
    row["M_min_rounded_256MiB"] = -(-row["M_min_both"] // (256 << 20)) * (256 << 20)
    row["M_min_5pct_both"] = max(-(-(d["modes"][m]["E_mov_plus_R"] * 20 + d["modes"][m]["components"]["TAV_W"]) * 10 // 180) for m in ("sparse", "dense"))
    for mode in ("sparse", "dense"):
        md = d["modes"][mode]; e = md["E_mov_plus_R"]; tw = md["components"]["TAV_W"]
        full = next(k for k in tree["phases"][mode] if k.split(" ")[0] == md["phase"])
        comp = md["components"]
        budgets = {name: {"margin": int(0.9 * M) - e, "text_budget": round((int(0.9 * M) - e) / tw, 4), "fraction_of_M": round(e / M, 4)}
                   for name, M in list(CANDS.items()) + [("rounded M_min", row["M_min_rounded_256MiB"])]}
        row[mode] = {"phase": md["phase"], "E_mov_plus_R": e, "E_mov_max_heap": e - R, "TAV_W": tw, "M_min": mmin(e),
                     "budgets": budgets, "phases": {k: v["E_mov_plus_R"] for k, v in md["phases"].items()},
                     "binding": {"phase": full,
                                 "requested": {t: comp[t] for t in tree["phases"][mode][full]["requested"]},
                                 "moving": {t: comp[t] for t in tree["phases"][mode][full]["moving"]},
                                 "T16_stages": {k: comp[k] for k in comp if k.startswith("T16_P")},
                                 "T17_stages": {k: comp[k] for k in comp if k.startswith("T17_V")}}}
    out["points"][p["tag"]] = row
t1, t2 = out["points"]["d1_c1"], out["points"]["t_c3_k16_l64"]
out["P1_two_tiers"] = {"M_min_both": max(t1["M_min_both"], t2["M_min_both"]),
                       "E_mov_plus_R_max": max(t[m]["E_mov_plus_R"] for t in (t1, t2) for m in ("sparse", "dense"))}
json.dump(out, open(sys.argv[3], "w"), indent=1)
for t, r in out["points"].items():
    d, s = r["dense"], r["sparse"]
    print(f"{t:16s} dense {d['phase']} {d['E_mov_plus_R']:>13,} sparse {s['phase']} {s['E_mov_plus_R']:>13,} "
          f"Mmin {r['M_min_both']:>14,} ({r['M_min_both']/GiB:.2f} GiB) D {r['D']:>7} conv {r['D_converged']} "
          + " ".join(f"{k}:{min(d['budgets'][k]['text_budget'], s['budgets'][k]['text_budget']):.3f}" for k in CANDS))
