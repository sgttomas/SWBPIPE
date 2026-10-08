"""RV114: an independent re-pricing of I93's B2 memory answer (PLAN.md section 3.3), with I82's
evaluator (b1_eval.py, unchanged, imported read-only) on I82's committed profile trees and I72's
in-build atoms. Written fresh: it does not import I93's scripts or sets.

Per-form rule (RV114's own reading of what one retained mechanics combination adds at D1's caps):
  * forms whose value is identical in the c and c+z trees (invocation scope) are left as they are;
  * "case-like" forms (a combination owns one CasePrep, Run, frozen candidate, trace, its envelope rows
    and one receipt entry): taken from the c+z tree in every estimate;
  * "ordinary-side" forms (per requested case ordinary observations, seeds, the X branch): from the c tree;
  * "partial" forms (O_base, TAV_W, NOTICE, NOTICE_moving, T12): LOW takes c, HIGH takes c+z.
The classification is derived mechanically from the trees (which forms differ between c and c+z) and
then assigned by name; any differing form not named stops the script.
HIGH adds a combined-ledger surcharge of (h-1) per-case T12 per combination, with h = 3 (the proposed
term cap, repeats counted), and also reports I93's h = c variant.
Usage: python rv114_price.py <I82 _run_records> <I72 law_record.txt> <I65 text_budget_W.caps.out.json> <out.json>
"""
import collections, copy, hashlib, json, os, sys

rec, law, text_w, out_path = sys.argv[1:5]
assert hashlib.sha256(open(os.path.join(rec, "b1_eval.py"), "rb").read()).hexdigest().startswith("c404e8db")
assert hashlib.sha256(open(law, "rb").read()).hexdigest().startswith("cbf34c52")
sys.path.insert(0, rec)
import b1_eval  # noqa: E402

atoms = b1_eval.load_atoms(law)
G = 1 << 30
R = 64 << 20
BUDGET12 = (9 * 12 * G) // 10
STEP = 256 << 20


def tree(tag):
    p = os.path.join(rec, "profile_trees", tag + ".json")
    if not os.path.exists(p):
        p = os.path.join(rec, "addendum_01", "profile_trees", tag + ".json")
    return json.load(open(p))


T = {c: tree("d1_c%d" % c) for c in (1, 2, 3, 4)}
T[6] = tree("d1_c6")

CASE_LIKE = {"T13", "T14", "T15", "STAGED", "T16_P1", "T16_P2", "T16_P3", "T16_P4", "T16_moving",
             "T17_V1", "T17_V2_clone", "T17_V2_hash", "T17_V3", "T17_V4", "T17_V5", "T17_V6",
             "T17_moving_publication", "T17_output", "SUCC", "BODY"}
ORDINARY_SIDE = {"T11", "T11_late_capture", "T11_ordinary_seed", "TAV_X", "T17_V3_ordinary"}
PARTIAL = {"O_base_sparse", "O_base_dense", "TAV_W", "NOTICE", "NOTICE_moving", "T12"}


def differing(tc, tz):
    return sorted(n for n in tz["forms"] if tz["forms"][n] != tc["forms"][n])


def build(tc, tz, est, text_atoms_from="cz"):
    h = copy.deepcopy(tz)
    for n in differing(tc, tz):
        if n.startswith("T25_") or n in ORDINARY_SIDE:
            h["forms"][n] = copy.deepcopy(tc["forms"][n])
        elif n in CASE_LIKE:
            pass
        elif n in PARTIAL:
            if est == "low":
                h["forms"][n] = copy.deepcopy(tc["forms"][n])
        else:
            raise SystemExit("unclassified differing form " + n)
    if text_atoms_from == "c":
        h["text_atoms"] = copy.deepcopy(tc["text_atoms"])
    return h


def ev(t):
    r = b1_eval.evaluate(t, atoms)
    return {m: (r[m]["E_mov_plus_R"], r[m]["phase"], r[m]["components"].get("T12")) for m in r}


def step5(e, tav):
    need = -(-(e + int(0.05 * tav)) * 10 // 9)
    return -(-need // STEP) * STEP


out = {"differing_forms": {}, "points": {}, "sensitivity": {}}
for c in (1, 2, 3):
    out["differing_forms"]["c%d_c%d" % (c, c + 1)] = differing(T[c], T[c + 1])
t12_case = ev(T[3])["dense"][2] // 3
out["T12_per_case_c3"] = t12_case
S3 = ev(T[3])
out["S3"] = {m: S3[m][0] for m in S3}

# I65 c = 1 TEXT rows: RV114's own file-level split of what a retained combination repeats
rows = json.load(open(text_w))["rows"]
by_file = collections.Counter()
for r in rows:
    by_file[r["file"]] += r.get("req", 0)
total_text = sum(by_file.values())
W1_SIDE = ("core/product_physics/src/retained_product.rs", "core/product_physics/src/retained_wire.rs",
           "core/product_physics/src/retained_receipt.rs", "core/reporting/result_export/src/retained_precision.rs",
           "core/serialization/canonical_json/")
ROW_SIDE = ("core/reporting/result_export/src/preview_physics_evidence.rs", "core/product_physics/src/preview_physics.rs",
            "core/reporting/result_export/")
w1 = sum(b for f, b in by_file.items() if f.startswith(W1_SIDE))
rowside = sum(b for f, b in by_file.items() if f.startswith(ROW_SIDE) and not f.startswith(W1_SIDE))
out["text_split"] = {"total": total_text, "w1_side_files": w1, "row_side_files": rowside,
                     "share_w1_only": w1 / total_text, "share_w1_plus_rows": (w1 + rowside) / total_text,
                     "lib_rs_total": by_file["core/product_physics/src/lib.rs"],
                     "stress_recovery_total": by_file["core/loads/stress_recovery/src/lib.rs"]}

for c, z in ((1, 1), (2, 1), (1, 2), (3, 1), (2, 2)):
    tc, tz = T[c], T[c + z]
    tav_c, tav_z = tc["forms"]["TAV_W"]["1"], tz["forms"]["TAV_W"]["1"]
    lo, hi = ev(build(tc, tz, "low")), ev(build(tc, tz, "high"))
    lo_tc = ev(build(tc, tz, "low", "c"))
    sur_h3 = z * 2 * t12_case
    sur_hc = z * max(0, c - 1) * t12_case
    pt = {}
    for m in ("dense", "sparse"):
        L, H, Lc = lo[m][0], hi[m][0], lo_tc[m][0]
        pt[m] = {"low": L, "low_phase": lo[m][1], "low_text_atoms_c": Lc, "high_h_eq_c": H + sur_hc,
                 "high_h3": H + sur_h3, "high_phase": hi[m][1],
                 "step5_low_GiB": step5(L, tav_c) / G, "step5_low_text_atoms_c_GiB": step5(Lc, tav_c) / G,
                 "step5_high_h3_GiB": step5(H + sur_h3, tav_z) / G,
                 "margin12_low": BUDGET12 - L, "margin12_low_text_atoms_c": BUDGET12 - Lc}
        # the TAV_W share s of one case's increment that a combination may repeat before the 12 GiB 5 % line
        # fails, starting from LOW with c's text atoms (the most favourable starting point)
        d = tav_z - tav_c
        room = BUDGET12 - Lc - int(0.05 * tav_c)
        pt[m]["breakeven_share_from_low_text_atoms_c"] = room / (1.05 * d) if d else None
    pt["TAV_W_c"], pt["TAV_W_cz"] = tav_c, tav_z
    out["points"]["c%d_z%d" % (c, z)] = pt

# HIGH of C_eq <= 3 against S3 at 10.5 GiB
M105 = int(10.5 * G)
b105 = (9 * M105) // 10
for key in ("c2_z1", "c1_z2"):
    hh = out["points"][key]["dense"]["high_h3"]
    out["sensitivity"][key + "_high_h3_vs_S3"] = {"delta_to_S3": hh - out["S3"]["dense"],
                                                  "text_budget_at_10_5GiB": (b105 - hh) / T[3]["forms"]["TAV_W"]["1"]}
# a fourth case against S3; MID-free ratio bounds for one combination at c = 3
c4 = ev(T[4])["dense"][0]
out["fourth_case_increment"] = c4 - out["S3"]["dense"]
p31 = out["points"]["c3_z1"]["dense"]
out["combination_as_fraction_of_case"] = {"low": (p31["low"] - out["S3"]["dense"]) / (c4 - out["S3"]["dense"]),
                                          "high_h3": (p31["high_h3"] - out["S3"]["dense"]) / (c4 - out["S3"]["dense"])}
json.dump(out, open(out_path, "w"), indent=1, sort_keys=True)
print(json.dumps(out, indent=1, sort_keys=True))
