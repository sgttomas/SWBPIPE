"""I65 U4 G3: per-mode composition at the caps of the G3-priced terms (stdlib only).

Inputs: ordinary_caps.caps.out.json (T05, which contains T07 and T25 at EPS 6),
t25_caps.caps.eps{6,2}.out.json (T25 at both escape factors), text_closure.caps.json (T08),
producer_caps.caps.out.json (T11-T15). Output: the G-A and G-C spans and the fit check
E_mov + R <= M for the two mutually exclusive branches after exact-block arbitration:
  X: exact-block selected (W1 bypassed, D-15): G-A span = O(without T25) + T25 + text + T11
  W: W1 runs: G-A span = O(without T25) + text + T11; then G-B/G-C add T11 late capture and
     T12-T15 while the complete ordinary owner R_complete stays (here: all of O, conservative);
     G4's T16-T19 are NOT included (they are G4's, design-to-budget).
All values use ASSUMED layouts; the expressions are evaluated in-build by G5.
"""
import json, os
H = os.path.dirname(os.path.abspath(__file__))
M = 4_026_531_840
R = 64 * 2**20
o = json.load(open(os.path.join(H, "ordinary_caps.caps.out.json")))
txt = json.load(open(os.path.join(H, "text_closure.caps.json")))["atoms"]
prod = json.load(open(os.path.join(H, "producer_caps.caps.out.json")))
t25 = {e: json.load(open(os.path.join(H, f"t25_caps.caps.eps{e}.out.json"))) for e in (6, 2)}
T25_ROW = "T25 selected source-blocks finalization (RESIDUALS_G3 T25)"
res = {"M": M, "R": R, "modes": {}}
for mode in ("sparse", "dense"):
    rows = o[mode]["rows"]
    O_all = o[mode]["O_req"]["assumed_bytes"]
    o_t25 = rows[T25_ROW]["assumed_bytes"]
    O_base = O_all - o_t25
    mov_ord = o[mode]["O_mov_extra_largest_old_backing"]["assumed_bytes"]   # includes T25's EPS-6 text growth
    text_req, text_mov_extra = txt["TAV_text_requested"], txt["TAV_text_moving"] - txt["TAV_text_requested"]
    T = prod["terms"]
    for e in (6, 2):
        T25 = t25[e]["T25_requested"]["assumed_bytes"]
        t25_mov = t25[e]["moving_extra"]["assumed_bytes"]
        X_req = O_base + T25 + text_req + T["T11"]
        X_mov = X_req + max(t25_mov, text_mov_extra, 131_072 * 64)
        W_req = O_base + text_req + T["T11"] + T["T12"] + T["T13"] + T["T14"] + T["T15"]
        W_mov = W_req + max(text_mov_extra, 131_072 * 64, 2 * 1024 * 1024)
        res["modes"].setdefault(mode, {})[f"EPS{e}"] = {
            "O_without_T25": O_base, "T25": T25, "text_TAV": text_req, "T11": T["T11"],
            "T12_T15": T["T12"] + T["T13"] + T["T14"] + T["T15"],
            "branch_X_selected": {"E_req": X_req, "E_mov": X_mov, "E_mov_plus_R": X_mov + R,
                                  "fits_M": X_mov + R <= M, "headroom": M - (X_mov + R)},
            "branch_W_w1_G3_part": {"E_req": W_req, "E_mov": W_mov, "E_mov_plus_R": W_mov + R,
                                    "fits_M": W_mov + R <= M, "headroom_for_G4": M - (W_mov + R)},
        }
print(json.dumps(res, indent=1))
