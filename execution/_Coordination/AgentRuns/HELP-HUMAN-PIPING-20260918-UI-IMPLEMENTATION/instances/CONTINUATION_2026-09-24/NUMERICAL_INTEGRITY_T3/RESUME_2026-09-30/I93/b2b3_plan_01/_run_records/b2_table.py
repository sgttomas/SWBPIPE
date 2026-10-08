"""I93: PLAN.md section 3.3's table, from b2_bracket.out.json and b2_mid.out.json (stdlib only).
5 % step: the smallest 256 MiB multiple M with E+R + 0.05*TAV_W <= 0.9 M; TAV_W from the c tree for LOW,
the c + z tree for HIGH, and c + share_T * (increment) for MID. Heap = E+R - R (R = 64 MiB).
Usage: python3 b2_table.py <b2_bracket.out.json> <b2_mid.out.json>
"""
import json, sys

b = json.load(open(sys.argv[1])); m = json.load(open(sys.argv[2]))
G, R, ST = 1 << 30, 64 << 20, 256 << 20


def step(e, tav):
    need = -(-(e + int(0.05 * tav)) * 10 // 9)
    return (-(-need // ST) * ST) / G


mid = {(p["c"], p["z"]): p for p in m["points"]}
share = m["share_T"]
print("budget_0.9x12GiB", b["budget_12GiB"])
for p in b["points"]:
    c, z = p["c"], p["z"]
    tl, th = p["TAV_W_low"], p["TAV_W_high"]
    tm = tl + int(share * (th - tl))
    for mode in ("dense", "sparse"):
        lo, hi = p["modes"][mode]["low"], p["modes"][mode]["high"]
        md = mid.get((c, z), {}).get("modes", {}).get(mode, {}).get("mid")
        print(f"c={c} z={z} {mode}: low {lo} mid {md} high {hi}; 5% step GiB low {step(lo, tl)} "
              f"mid {step(md, tm) if md else None} high {step(hi, th)}")
for name, (c, z, which) in {"Ceq3_high(c2z1)": (2, 1, "high"), "c3z1_mid": (3, 1, "mid"), "c3z1_high": (3, 1, "high")}.items():
    if which == "mid":
        e = mid[(c, z)]["modes"]["dense"]["mid"]
    else:
        e = [p for p in b["points"] if (p["c"], p["z"]) == (c, z)][0]["modes"]["dense"]["high"]
    h = e - R
    print(f"{name}: dense E+R {e}; heap {h} = {h / G:.2f} GiB; of 16 GiB {h / (16 * G):.3f}; of 32 GiB {h / (32 * G):.3f}")
for tag, v in b["reduced_as_case_equivalents"].items():
    print(tag, {k: (v[k]["E_mov_plus_R"], round(v[k]["text_budget_at_12GiB"], 4), v[k]["step256_5pct"] / G) for k in ("dense", "sparse")})
