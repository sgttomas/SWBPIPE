"""I93 B2/B3-P: a bracket for k mechanics combinations beside c load cases, priced with I82's in-build
evaluator (b1_eval.py, unchanged) on I82's committed multi-case profile trees.

Method (evaluator level; no chain run). A hybrid tree for (c cases, z combinations) takes each form's
coefficients from I82's tree at c or at c + z, by the form's scope:
  always c        T11, T11_late_capture, T11_ordinary_seed, INVOC, T19, STATICS, T07_moving, the X branch
                  (T25_*, TAV_X): seeds and observations are per requested case; a combination has no
                  ordinary solve; the raw request grows only by the authored terms (inside the raw caps);
                  exact-block (X) is ineligible when combinations exist (PP lib.rs source_eligible).
  always c + z    T13, T14, T15 (one Run, one frozen candidate, one trace per retained combination),
                  STAGED, T16_*, T16_moving, T17_*, SUCC, BODY (the envelope carries each combination's
                  rows, priced at P_final rows each, and the receipt carries one combination entry,
                  source, call, group and run, priced as one more attempted case).
  bracketed       O_base_*, TAV_W, T12, NOTICE, NOTICE_moving: the LOW estimate takes c (a combination adds
                  no ordinary-solve owners or text, its operand preparations stay within a = c, and no notice);
                  the HIGH estimate takes c + z (a combination priced as one more requested and attempted case).
HIGH is therefore the c + z case tree exactly, plus a ledger surcharge (the combined ledger holds every
operand's load terms; see LEDGER below).
Usage: python3 b2_bracket.py <I82 _run_records dir> <I72 law record> <out json>
"""
import copy, json, os, sys

rec, law, out_path = sys.argv[1:4]
sys.path.insert(0, rec)
import b1_eval  # noqa: E402  (I82's evaluator, unchanged)

atoms = b1_eval.load_atoms(law)
trees = {}
for tag in ("d1_c1", "d1_c2", "d1_c3", "d1_c4", "d1_c8"):
    trees[tag] = json.load(open(os.path.join(rec, "profile_trees", tag + ".json")))
trees["d1_c6"] = json.load(open(os.path.join(rec, "addendum_01", "profile_trees", "d1_c6.json")))
for tag, f in (("c4_l32", "c4_l32.json"), ("c4_m24", "c4_m24.json"), ("c4_m20", "c4_m20.json"), ("c4_l48", "c4_l48.json")):
    trees[tag] = json.load(open(os.path.join(rec, "addendum_01", "profile_trees", f)))
for tag in ("t_c3_k16_l64", "t_c4_k12_l64", "v_c3_k8_l64"):
    trees[tag] = json.load(open(os.path.join(rec, "profile_trees", tag + ".json")))

ALWAYS_C = {"T11", "T11_late_capture", "T11_ordinary_seed", "INVOC", "T19", "STATICS", "T07_moving", "TAV_X"}
ALWAYS_CZ = {"T13", "T14", "T15", "STAGED", "T16_P1", "T16_P2", "T16_P3", "T16_P4", "T16_moving",
             "T17_V1", "T17_V2_clone", "T17_V2_hash", "T17_V3", "T17_V4", "T17_V5", "T17_V6",
             "T17_moving_invocation", "T17_moving_publication", "T17_output", "SUCC", "BODY",
             "TXT_moving", "HELPER_moving"}
BRACKET = {"O_base_sparse", "O_base_dense", "TAV_W", "T12", "NOTICE", "NOTICE_moving"}


def hybrid(t_c, t_cz, low):
    h = copy.deepcopy(t_cz)
    for name in h["forms"]:
        if name.startswith("T25_"):
            h["forms"][name] = copy.deepcopy(t_c["forms"][name])
        elif name in ALWAYS_C:
            h["forms"][name] = copy.deepcopy(t_c["forms"][name])
        elif name in ALWAYS_CZ:
            pass
        elif name in BRACKET:
            if low:
                h["forms"][name] = copy.deepcopy(t_c["forms"][name])
        else:
            raise SystemExit("unclassified form " + name)
    # text atoms (D, D_env, Text(diag_env), ...): the c + z tree's (they price envelope-wide texts)
    return h


def ev(tree):
    r = b1_eval.evaluate(tree, atoms)
    return {m: {"phase": r[m]["phase"], "E_mov_plus_R": r[m]["E_mov_plus_R"],
                "W3": r[m]["phases"]["W3"]["E_mov_plus_R"], "W4": r[m]["phases"]["W4"]["E_mov_plus_R"],
                "components": {k: r[m]["components"][k] for k in ("T12", "T13", "T14", "T15", "T16", "T17")}}
            for m in r}


GIB = 1 << 30
M12 = 12 * GIB
budget12 = (9 * M12) // 10
M105 = int(10.5 * GIB)
budget105 = (9 * M105) // 10


def smallest_M(e):
    # smallest M with e <= 0.9 M (integer bytes), and the smallest 256 MiB step with a 5 % text budget is
    # reported separately by the caller (it needs TAV_W)
    return -(-e * 10 // 9)


def step256(e, tavw, beta=0.05):
    need = -(-(e + int(beta * tavw)) * 10 // 9)
    step = 256 << 20
    return -(-need // step) * step


res = {"atoms_from": os.path.basename(law), "trees": {}, "points": []}
for tag, t in trees.items():
    e = ev(t)
    res["trees"][tag] = {m: {"phase": e[m]["phase"], "E_mov_plus_R": e[m]["E_mov_plus_R"]} for m in e}
    res["trees"][tag]["TAV_W"] = t["forms"]["TAV_W"]["1"]

# the one-case ledger share inside T12 (dense and sparse equal): T12 per attempted case at D1's caps
t12_c1 = ev(trees["d1_c1"])["dense"]["components"]["T12"]
t12_c3 = ev(trees["d1_c3"])["dense"]["components"]["T12"]
res["T12_per_case_D1"] = {"c1": t12_c1, "c3_per_case": t12_c3 // 3}

pairs = [(1, 1, "d1_c1", "d1_c2"), (2, 1, "d1_c2", "d1_c3"), (1, 2, "d1_c1", "d1_c3"), (3, 1, "d1_c3", "d1_c4"),
         (2, 2, "d1_c2", "d1_c4"), (3, 3, "d1_c3", "d1_c6"), (2, 4, "d1_c2", "d1_c6"), (3, 5, "d1_c3", "d1_c8")]
for c, z, tc, tcz in pairs:
    lo, hi = ev(hybrid(trees[tc], trees[tcz], True)), ev(hybrid(trees[tc], trees[tcz], False))
    # LEDGER surcharge on HIGH: a combination with h operand terms holds sum_i l_i <= h*l load terms in its
    # combined ledger and h*k prescribed pairs, against one case's l. Bounded here by (h - 1) more T12 per
    # combination with h = c (every case an operand once), which over-covers the ledger arrays (T12 also
    # holds the section preparation, which a combination clones once, not h times).
    surcharge = z * max(0, c - 1) * res["T12_per_case_D1"]["c3_per_case"]
    pt = {"c": c, "z": z, "low_tree": f"{tc} + envelope/W1 run forms of {tcz}", "high_tree": tcz,
          "ledger_surcharge_high": surcharge, "modes": {}}
    for m in ("dense", "sparse"):
        L_, H_ = lo[m]["E_mov_plus_R"], hi[m]["E_mov_plus_R"] + surcharge
        pt["modes"][m] = {"low": L_, "low_phase": lo[m]["phase"], "high": H_, "high_phase": hi[m]["phase"],
                          "low_fits_12GiB": L_ <= budget12, "high_fits_12GiB": H_ <= budget12,
                          "low_smallest_M": smallest_M(L_), "high_smallest_M": smallest_M(H_),
                          "low_under_12GiB_by": budget12 - L_, "high_under_12GiB_by": budget12 - H_}
    pt["TAV_W_low"] = trees[tc]["forms"]["TAV_W"]["1"]
    pt["TAV_W_high"] = trees[tcz]["forms"]["TAV_W"]["1"]
    res["points"].append(pt)

# reduced-cap trees, priced as c + z case-equivalents (HIGH): which shapes carry 3 cases + 1 combination
red = {}
for tag in ("c4_l32", "c4_m24", "c4_m20", "c4_l48", "t_c4_k12_l64"):
    e = ev(trees[tag])
    tav = trees[tag]["forms"]["TAV_W"]["1"]
    red[tag] = {m: {"E_mov_plus_R": e[m]["E_mov_plus_R"], "fits_12GiB": e[m]["E_mov_plus_R"] <= budget12,
                    "under_12GiB_by": budget12 - e[m]["E_mov_plus_R"],
                    "text_budget_at_12GiB": (budget12 - e[m]["E_mov_plus_R"]) / tav,
                    "step256_5pct": step256(e[m]["E_mov_plus_R"], tav)} for m in e}
    red[tag]["TAV_W"] = tav
res["reduced_as_case_equivalents"] = red
res["budget_12GiB"] = budget12
res["budget_10_5GiB"] = budget105
json.dump(res, open(out_path, "w"), indent=1)
for pt in res["points"]:
    d, s = pt["modes"]["dense"], pt["modes"]["sparse"]
    print(f"c={pt['c']} z={pt['z']}: dense low {d['low']:,} ({d['low_phase']}) high {d['high']:,} ({d['high_phase']}); "
          f"12GiB budget {budget12:,}: low fits {d['low_fits_12GiB']} high fits {d['high_fits_12GiB']}; "
          f"sparse low {s['low']:,} high {s['high']:,}")
for tag, v in red.items():
    print(tag, {m: (v[m]['E_mov_plus_R'], v[m]['fits_12GiB'], round(v[m]['text_budget_at_12GiB'], 4), v[m]['step256_5pct'] / GIB) for m in ('dense', 'sparse')})
print("T12 per case", res["T12_per_case_D1"])
