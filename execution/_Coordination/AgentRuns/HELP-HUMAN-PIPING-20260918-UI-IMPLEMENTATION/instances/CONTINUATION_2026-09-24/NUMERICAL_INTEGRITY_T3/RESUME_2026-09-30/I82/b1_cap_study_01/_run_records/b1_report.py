"""I82 B1-S: the study's tables from the per-point in-build evaluations (stdlib only).

Reads WT/scratch/i82_b1_study/mc_runs/<tag>/inbuild.json for every point of points_final.json and writes one
JSON with: per point and mode, E_mov,max + R, the binding phase, the fraction of today's M and of 6.0 GiB, the
margin to 0.9 M at both, the smallest M meeting the margin rule (ceil((E_mov,max + R)/0.9)), the text-error
budget (margin / TAV_W, QUAL section 3's measure) at today's M, at 6.0 GiB and at the 256 MiB-rounded smallest
M; for the named candidates, every phase and the binding phase's terms; and the D1-caps fit in c.
Usage: python3 b1_report.py <points json> <mc_runs dir> <out json>
"""
import json, os, sys

pts = json.load(open(sys.argv[1])); runs = sys.argv[2]
M_TODAY, M6, R = 4_026_531_840, 6 * 2**30, 64 << 20
STEP = 256 << 20
def mmin(e):
    return -(-e * 10 // 9)
def rounded(e):
    return -(-mmin(e) // STEP) * STEP
def margin(e, M):
    return int(0.9 * M) - e
out = {"M_today": M_TODAY, "M_6GiB": M6, "budget_0.9M_today": int(0.9 * M_TODAY), "budget_0.9M_6GiB": int(0.9 * M6), "points": {}}
for p in pts:
    d = json.load(open(os.path.join(runs, p["tag"], "inbuild.json")))
    row = {"caps": p["caps"], "D": d["summary"]["D"], "D_converged": d["summary"].get("D_converged"),
           "text_complete": d["summary"]["text_complete"], "missing_atoms": d["missing_atoms"],
           "D_env": d["text_atoms"]["D_env"], "TAV": d["text_atoms"]["TAV_text_requested"]}
    both = 0
    for mode in ("sparse", "dense"):
        md = d["modes"][mode]; e = md["E_mov_plus_R"]; tw = md["components"]["TAV_W"]
        both = max(both, e)
        row[mode] = {"phase": md["phase"], "E_mov_plus_R": e, "TAV_W": tw,
                     "frac_M_today": round(e / M_TODAY, 4), "frac_M_6GiB": round(e / M6, 4),
                     "margin_today": margin(e, M_TODAY), "margin_6GiB": margin(e, M6),
                     "M_min": mmin(e),
                     "text_budget_today": round(margin(e, M_TODAY) / tw, 4),
                     "text_budget_6GiB": round(margin(e, M6) / tw, 4),
                     "phases": {k: v["E_mov_plus_R"] for k, v in md["phases"].items()}}
    row["M_min_both"] = mmin(both); row["M_rounded_256MiB"] = rounded(both)
    for mode in ("sparse", "dense"):
        e = row[mode]["E_mov_plus_R"]; tw = row[mode]["TAV_W"]
        row[mode]["margin_rounded"] = margin(e, row["M_rounded_256MiB"])
        row[mode]["text_budget_rounded"] = round(margin(e, row["M_rounded_256MiB"]) / tw, 4)
    if p["tag"] in ("d1_c1", "d1_c2", "t_c3_k16_l64", "u_c4_k12_l64_L128", "t_c4_k12_l64", "u_c3_k16_l64_L96", "u_c8_k4_l8", "v_c3_k8_l64", "u_c4_k8_l32"):
        tree = json.load(open(os.path.join(runs, p["tag"], "work", "profile_tree.json")))
        row["binding_terms"] = {}
        for mode in ("sparse", "dense"):
            ph = d["modes"][mode]["phase"]
            full = next(k for k in tree["phases"][mode] if k.split(" ")[0] == ph)
            comp = d["modes"][mode]["components"]
            req = {t: comp[t] for t in tree["phases"][mode][full]["requested"]}
            mov = {t: comp[t] for t in tree["phases"][mode][full]["moving"]}
            row["binding_terms"][mode] = {"phase": full, "requested": req, "moving": mov,
                                          "T16_stages": {k: comp[k] for k in comp if k.startswith("T16_P")},
                                          "T17_stages": {k: comp[k] for k in comp if k.startswith("T17_V")}}
    out["points"][p["tag"]] = row
# the D1-caps sequence and its exact differences in c
seq = [out["points"][f"d1_c{c}"]["dense"]["E_mov_plus_R"] for c in (1, 2, 3, 4)]
out["d1_dense_differences"] = {"first": [seq[i + 1] - seq[i] for i in range(3)], "second": [seq[i + 2] - 2 * seq[i + 1] + seq[i] for i in range(2)]}
json.dump(out, open(sys.argv[3], "w"), indent=1)
for t, r in out["points"].items():
    print(f"{t:22s} D={r['D']:>6} dense {r['dense']['phase']} {r['dense']['E_mov_plus_R']:>12} ({r['dense']['frac_M_6GiB']:.4f} of 6GiB) "
          f"sparse {r['sparse']['E_mov_plus_R']:>12}  Mmin {r['M_min_both']:>12}  Mr {r['M_rounded_256MiB']/2**30:.2f} GiB "
          f"tb6 {r['dense']['text_budget_6GiB']:.4f}")
