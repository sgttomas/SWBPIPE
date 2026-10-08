"""I82 B1-S: run b1_mc_run.sh over a list of cap points and tabulate E_mov,max + R per mode against
0.9 M at today's M (4,026,531,840 B) and at 6.0 GiB (6,442,450,944 B), with the smallest M that meets
the margin rule (E_mov,max + R <= 0.9 M, i.e. M >= ceil((E_mov,max + R) / 0.9)).
Usage: python3 b1_sweep.py <points json> <out tsv>   (env I82_WT; Python only)
A point is {"tag": ..., "caps": {G4_CAPS}}; caps use the chain's keys (n, m, g, s, r, l, c, a, L).
"""
import json, os, subprocess, sys

WT = os.environ["I82_WT"]
HERE = os.path.dirname(os.path.abspath(__file__))
S = os.path.join(WT, "scratch", "i82_b1_study")
M_TODAY, M_6 = 4_026_531_840, 6 * 2**30
points = json.load(open(sys.argv[1]))
rows = []
for p in points:
    out = os.path.join(S, "mc_runs", p["tag"], "inbuild.json")
    if not (p.get("reuse") and os.path.exists(out)):
        subprocess.run(["bash", os.path.join(HERE, "b1_mc_run.sh"), p["tag"], json.dumps(p["caps"])], check=True,
                       stdout=subprocess.DEVNULL, env=dict(os.environ, I82_WT=WT))
    d = json.load(open(out))
    r = {"tag": p["tag"], "caps": p["caps"], "D": d["summary"]["D"], "D_env": d["text_atoms"]["D_env"],
         "D_converged": d["summary"].get("D_converged"), "complete": d["summary"]["text_complete"],
         "missing_atoms": d["missing_atoms"]}
    for mode in ("sparse", "dense"):
        e = d["modes"][mode]["E_mov_plus_R"]
        r[mode] = {"phase": d["modes"][mode]["phase"], "E_mov_plus_R": e,
                   "frac_M_today": round(e / M_TODAY, 4), "frac_M_6GiB": round(e / M_6, 4),
                   "margin_today": int(0.9 * M_TODAY) - e, "margin_6GiB": int(0.9 * M_6) - e,
                   "M_min": -(-e * 10 // 9)}
    rows.append(r)
    print(json.dumps({"tag": r["tag"], "dense": [r["dense"]["phase"], r["dense"]["E_mov_plus_R"], r["dense"]["frac_M_6GiB"]],
                      "sparse": [r["sparse"]["phase"], r["sparse"]["E_mov_plus_R"]], "D_conv": r["D_converged"]}), flush=True)
json.dump(rows, open(sys.argv[2] + ".json", "w"), indent=1)
with open(sys.argv[2], "w") as f:
    f.write("tag\tc\ta\tn\tm\tg\tl\tL\tD\tD_env\tdense_phase\tdense_E+R\tdense/0.9M_6GiB\tsparse_phase\tsparse_E+R\tM_min(both)\n")
    for r in rows:
        c = r["caps"]
        f.write("\t".join(str(x) for x in [r["tag"], c.get("c", 1), c.get("a", c.get("c", 1)), c.get("n", 32), c.get("m", 32),
                                            c.get("g", 32), c.get("l", 192), c.get("L", c.get("c", 1) * c.get("l", 192)), r["D"], r["D_env"],
                                            r["dense"]["phase"], r["dense"]["E_mov_plus_R"],
                                            round(r["dense"]["E_mov_plus_R"] / (0.9 * M_6), 4), r["sparse"]["phase"],
                                            r["sparse"]["E_mov_plus_R"], max(r["dense"]["M_min"], r["sparse"]["M_min"])]) + "\n")
