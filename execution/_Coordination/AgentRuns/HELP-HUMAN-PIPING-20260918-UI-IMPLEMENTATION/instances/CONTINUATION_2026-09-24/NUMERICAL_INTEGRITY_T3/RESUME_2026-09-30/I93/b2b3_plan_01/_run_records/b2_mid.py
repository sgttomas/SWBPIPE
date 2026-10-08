"""I93 B2/B3-P: a MID estimate between b2_bracket.py's LOW and HIGH for z combinations beside c cases.

Two shares, both read from committed records, replace the bracket's all-or-nothing choice for the two
largest bracketed forms:
  TAV_W   the share of TEXT that a retained combination repeats. From I65's c = 1 TEXT site rows
          (u4_g7_01 pass_a text_budget_W.caps.out.json): bytes in W1-owner, serializer, precommit-reader
          and preview-row files count as repeated (a combination owns a frozen candidate, its own rows and
          its own receipt entry); bytes in the ordinary solve, model building, loads, supports and units
          count as not repeated (a combination has no ordinary solve). PP lib.rs is split by function:
          its row- and receipt-side functions (named below) count as repeated, the rest not.
  O_base  the per-case increment's share carried by the envelope-row atoms (ResultItem, row text, the row
          maps and their hash table), evaluated atom by atom with the in-build atoms between c and c + z.
MID = LOW + share_T * (TAV_W(c+z) - TAV_W(c)) + [O_base(c+z) - O_base(c)] restricted to row atoms.
It is an estimate, not a bound: B2's G5 on the real code replaces it.
Usage: python3 b2_mid.py <I82 _run_records> <I72 law record> <I65 text_budget_W.caps.out.json> <out json>
"""
import collections, copy, json, os, sys

rec, law, text_w, out_path = sys.argv[1:5]
sys.path.insert(0, rec)
import b1_eval  # noqa: E402
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

atoms = b1_eval.load_atoms(law)
T = {t: json.load(open(os.path.join(rec, "profile_trees", t + ".json"))) for t in ("d1_c1", "d1_c2", "d1_c3", "d1_c4")}

# ---- the TEXT share
rows = json.load(open(text_w))["rows"]
REPEATED_FILES = ("core/product_physics/src/retained_product.rs", "core/product_physics/src/retained_wire.rs",
                  "core/product_physics/src/retained_receipt.rs", "core/product_physics/src/preview_physics.rs",
                  "core/reporting/result_export/", "core/serialization/canonical_json/")
LIB_REPEATED_FNS = ("qualified_load_case_result_id", "append_integrity_report", "support_contribution_summary",
                    "exact_straight_summary_extrema", "run_linear_static_preview_observed", "retained_w1",
                    "integrity_diagnostic_id", "append_sparse_live_path_evidence", "append_linear_solver_mode_evidence")
rep = notrep = 0
by = collections.Counter()
for r in rows:
    b = r.get("req", 0)
    if not b:
        continue
    f, fn = r["file"], r["fn"]
    is_rep = f.startswith(REPEATED_FILES) or (f == "core/product_physics/src/lib.rs" and fn.split(":")[-1] in LIB_REPEATED_FNS)
    if is_rep:
        rep += b
    else:
        notrep += b
    by[("repeated" if is_rep else "not_repeated", f)] += b
share_T = rep / (rep + notrep)

ROW_ATOMS = ("s(ResultItem)", "Text(row)", "Node(String,BTreeMap)", "Node(String,ResultItem)", "HashReq((String,String))")


def val(form, v):
    return {a: (c if a == "1" else c * v[a]) for a, c in form.items()}


def ev(tree):
    return b1_eval.evaluate(tree, atoms)


v = dict(atoms)


def o_rows_delta(tc, tcz, mode):
    fc, fz = tc["forms"]["O_base_" + mode], tz_forms(tcz, mode)
    vv = dict(v)
    for k, x in tcz["text_atoms"].items():
        if k.startswith("Text("):
            vv[k] = x
    d = 0
    for a in set(fc) | set(fz):
        if a == "1" or not a.startswith(ROW_ATOMS):
            continue
        d += (fz.get(a, 0) - fc.get(a, 0)) * vv[a]
    return d


def tz_forms(t, mode):
    return t["forms"]["O_base_" + mode]


import b2_bracket_lib as BL  # noqa: E402  (the bracket's hybrid, shared)

res = {"share_T": share_T, "text_repeated_bytes_c1": rep, "text_not_repeated_bytes_c1": notrep,
       "text_by_file": [[k[0], k[1], b] for k, b in by.most_common(40)], "points": []}
GIB = 1 << 30
budget12 = (9 * 12 * GIB) // 10
for c, z, tc, tcz in ((3, 1, "d1_c3", "d1_c4"), (2, 1, "d1_c2", "d1_c3"), (1, 2, "d1_c1", "d1_c3"), (2, 2, "d1_c2", "d1_c4"), (1, 1, "d1_c1", "d1_c2")):
    lo = ev(BL.hybrid(T[tc], T[tcz], True))
    dT = T[tcz]["forms"]["TAV_W"]["1"] - T[tc]["forms"]["TAV_W"]["1"]
    pt = {"c": c, "z": z, "dTAV_W": dT, "modes": {}}
    for m in ("dense", "sparse"):
        dO = o_rows_delta(T[tc], T[tcz], m)
        mid = lo[m]["E_mov_plus_R"] + int(share_T * dT) + dO
        tav_mid = T[tc]["forms"]["TAV_W"]["1"] + int(share_T * dT)
        pt["modes"][m] = {"low": lo[m]["E_mov_plus_R"], "mid": mid, "O_base_row_delta": dO,
                          "mid_under_12GiB_by": budget12 - mid, "mid_text_budget_12GiB": (budget12 - mid) / tav_mid,
                          "mid_step256_5pct_GiB": (-(-(-(-(mid + int(0.05 * tav_mid)) * 10 // 9)) // (256 << 20)) * (256 << 20)) / GIB}
    res["points"].append(pt)
json.dump(res, open(out_path, "w"), indent=1)
print("share_T", round(share_T, 4), "repeated", rep, "not", notrep)
for pt in res["points"]:
    d = pt["modes"]["dense"]
    print(pt["c"], pt["z"], "dTAV_W", pt["dTAV_W"], "dense low", d["low"], "mid", d["mid"], "O_rows", d["O_base_row_delta"],
          "under12", d["mid_under_12GiB_by"], "tb12", round(d["mid_text_budget_12GiB"], 4), "M5%", d["mid_step256_5pct_GiB"])
