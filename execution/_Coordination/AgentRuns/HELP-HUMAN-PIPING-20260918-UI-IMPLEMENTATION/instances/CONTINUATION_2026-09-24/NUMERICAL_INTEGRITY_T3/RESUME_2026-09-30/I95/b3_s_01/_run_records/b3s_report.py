"""I95 B3-S: the tables of STUDY.md from the sweep's in-build results (stdlib only).

For every chain (mc_chain = I82's, unchanged; ur, urc, er, erc = b3s_exact_chain.py's variants) and c = 1..3:
per-mode E+R, the binding phase, every phase, TAV_W, TAV_X, D, D_env; the smallest M (E+R <= 0.9 M); the
5 % M (the smallest 256 MiB multiple with E+R + 0.05 * TAV <= 0.9 M in both modes, TAV the binding branch's
text: TAV_X when an X phase binds, else TAV_W); the margin and text-error budget at 10.5, 11, 11.5 and 12 GiB.
Also W3's terms (I82 STUDY section 5's split) and X1's, and the c = 1 profile-tree change against I82's d1_c1.
Usage: python3 b3s_report.py <runs dir> <I82 run records dir> <law record> <out json>
"""
import json, os, sys

runs, rec82, law, out = sys.argv[1:5]
sys.path.insert(0, rec82)
import b1_eval

GiB = 1 << 30
STEP = 256 << 20
R = 64 << 20
CHAINS = ["mc_chain", "ur", "urc", "er", "erc"]


def load(ch, c):
    return json.load(open(os.path.join(runs, ch, f"c{c}", "inbuild.json")))


def branch_tav(d, mode, phase):
    s = d["summary"][mode]
    return s["TAV_X"] if phase.startswith("X") else s["TAV_W"]


def five_pct_M(d):
    need = 0
    for mode in ("sparse", "dense"):
        m = d["modes"][mode]
        need = max(need, (m["E_mov_plus_R"] + 0.05 * branch_tav(d, mode, m["phase"])) / 0.9)
    return -(-int(need) // STEP) * STEP


def five_pct_M_all(d):
    """The 5 % rule applied to every phase (not only the binding one), each with its branch's text."""
    need = 0
    for mode in ("sparse", "dense"):
        for ph, v in d["modes"][mode]["phases"].items():
            need = max(need, (v["E_mov_plus_R"] + 0.05 * branch_tav(d, mode, ph)) / 0.9)
    return -(-int(need) // STEP) * STEP


def at(d, M):
    o = {}
    for mode in ("sparse", "dense"):
        m = d["modes"][mode]
        margin = int(0.9 * M) - m["E_mov_plus_R"]
        o[mode] = {"margin": margin, "budget_pct": round(100 * margin / branch_tav(d, mode, m["phase"]), 2)}
    return o


W3_TERMS = ["TAV_W", "T16", "O_base_dense", "T16_moving", "T12_T15", "STAGED", "HELPER_moving", "STATICS", "TXT_moving"]
table = {}
for ch in CHAINS:
    for c in (1, 2, 3):
        d = load(ch, c)
        row = {"D": d["summary"]["D"], "D_env": d["summary"]["D_env"], "text_complete": d["summary"]["text_complete"],
               "TAV": d["summary"]["TAV"], "TAV_W": d["summary"]["dense"]["TAV_W"], "TAV_X": d["summary"]["dense"]["TAV_X"]}
        for mode in ("sparse", "dense"):
            m = d["modes"][mode]
            row[mode] = {"E_plus_R": m["E_mov_plus_R"], "phase": m["phase"],
                         "phases": {k: v["E_mov_plus_R"] for k, v in m["phases"].items()},
                         "smallest_M": -(-m["E_mov_plus_R"] * 10 // 9)}
        row["five_pct_M"] = five_pct_M(d)
        row["five_pct_M_GiB"] = row["five_pct_M"] / GiB
        row["five_pct_M_every_phase_GiB"] = five_pct_M_all(d) / GiB
        row["X1_at_10.5GiB_dense_budget_pct"] = round(100 * (int(0.9 * 10.5 * GiB) - d["modes"]["dense"]["phases"]["X1"]["E_mov_plus_R"]) / d["summary"]["dense"]["TAV_X"], 2)
        row["at"] = {f"{g} GiB": at(d, int(g * GiB)) for g in (10.5, 11, 11.5, 12)}
        comp = d["modes"]["dense"]["components"]
        row["dense_terms"] = {k: comp.get(k) for k in ("TAV_W", "TAV_X", "T16", "T16_P1", "T16_P2", "T16_P3", "T16_P4", "T16_moving",
                                                       "T17", "T17_V2_hash", "T25", "T25_I1", "O_base_dense", "T12_T15", "STAGED", "SUCC",
                                                       "BODY", "INVOC", "STATICS", "HELPER_moving")}
        table[f"{ch}/c{c}"] = row

# the c = 1 tree change against I82's d1_c1 (forms and text atoms), evaluated with the in-build atoms
atoms = b1_eval.load_atoms(law)
t82 = json.load(open(os.path.join(rec82, "profile_trees", "d1_c1.json")))
tree_change = {}
for ch in CHAINS:
    t = json.load(open(os.path.join(runs, ch, "c1", "work", "profile_tree.json")))
    v82, v = dict(atoms), dict(atoms)
    v82.update({k: x for k, x in t82["text_atoms"].items() if k.startswith("Text(")})
    v.update({k: x for k, x in t["text_atoms"].items() if k.startswith("Text(")})
    forms = {}
    for name in sorted(set(t82["forms"]) | set(t["forms"])):
        a, b = t82["forms"].get(name), t["forms"].get(name)
        if a != b:
            ea = b1_eval.form(a, v82) if a is not None else None
            eb = b1_eval.form(b, v) if b is not None else None
            forms[name] = {"I82": ea, "this": eb, "delta": (eb or 0) - (ea or 0)}
    tree_change[ch] = {"identical": t == t82,
                       "text_atoms": {k: [t82["text_atoms"].get(k), t["text_atoms"].get(k)] for k in sorted(set(t82["text_atoms"]) | set(t["text_atoms"]))
                                      if t82["text_atoms"].get(k) != t["text_atoms"].get(k)},
                       "forms_changed": len(forms), "forms": dict(sorted(forms.items(), key=lambda kv: -abs(kv[1]["delta"])))}

# attribution at c = 3: the exact-route-only credited chain without the census substitution
attribution = None
if os.path.exists(os.path.join(runs, "erc_nocensus", "c3", "inbuild.json")):
    a = json.load(open(os.path.join(runs, "erc_nocensus", "c3", "inbuild.json")))
    attribution = {mode: {"S3": table["mc_chain/c3"][mode]["E_plus_R"], "erc_without_census": a["modes"][mode]["E_mov_plus_R"],
                          "erc": table["erc/c3"][mode]["E_plus_R"],
                          "text_and_diagnostics": a["modes"][mode]["E_mov_plus_R"] - table["mc_chain/c3"][mode]["E_plus_R"],
                          "census_evidence": table["erc/c3"][mode]["E_plus_R"] - a["modes"][mode]["E_mov_plus_R"]} for mode in ("sparse", "dense")}
json.dump({"table": table, "tree_change_c1": tree_change, "attribution_c3": attribution}, open(out, "w"), indent=1)
print("attribution c3:", json.dumps(attribution))
for k, r in table.items():
    print(f"{k:12} dense {r['dense']['E_plus_R']:>15,} {r['dense']['phase']}  sparse {r['sparse']['E_plus_R']:>15,} {r['sparse']['phase']}"
          f"  D {r['D']:>7} TAV_W {r['TAV_W']:>14,} TAV_X {r['TAV_X']:>14,}  5%M {r['five_pct_M_GiB']:.2f} GiB"
          f"  every-phase {r['five_pct_M_every_phase_GiB']:.2f}  @10.5 {r['at']['10.5 GiB']['dense']['budget_pct']}% @12 {r['at']['12 GiB']['dense']['budget_pct']}% X1@10.5 {r['X1_at_10.5GiB_dense_budget_pct']}%")
for ch, tc in tree_change.items():
    print(ch, "c=1 tree identical to I82 d1_c1:", tc["identical"], "forms changed:", tc["forms_changed"], "text atoms changed:", list(tc["text_atoms"]))
