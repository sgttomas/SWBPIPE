"""RV87 (u4_g4_02): confirm I65's S-2 repair in the U4 G5 part-2 text run.

1. Diff the part-2 run against its rebase run, row by row (which sites changed, by how much).
2. List the result-id copies still priced below their bound in the part-2 run (sites found by
   RV87's scan of the 1e323058f3 source, each read and classified by hand; listed below with
   the reason), and price each in the run's own convention: req = mult * max(8, 2 * size).
Usage: python3 rv87_s2_confirm.py <part2 _run_records/text_p2> > rv87_s2_confirm.out.json
Stdlib only."""
import json, os, sys
from collections import defaultdict
T = sys.argv[1]
a = json.load(open(os.path.join(T, "text_budget.caps.rebase.out.json")))
b = json.load(open(os.path.join(T, "text_budget.caps.out.json")))
def idx(rows):
    d = defaultdict(list)
    for r in rows:
        d[(r["file"], r["line"], r["kind"])].append(r)
    return d
A, B = idx(a["rows"]), idx(b["rows"])
changed = []
for key in sorted(set(A) | set(B)):
    ra, rb = A.get(key, []), B.get(key, [])
    for i in range(max(len(ra), len(rb))):
        x = ra[i] if i < len(ra) else None
        y = rb[i] if i < len(rb) else None
        xa = (x["mult"], x["bytes"], x["req"]) if x else None
        yb = (y["mult"], y["bytes"], y["req"]) if y else None
        if xa != yb:
            changed.append({"site": f"{key[0]}:{key[1]}", "kind": key[2], "rebase": xa, "part2": yb,
                            "delta": (y["req"] if y else 0) - (x["req"] if x else 0)})
s2_sites = [c for c in changed if c["kind"] != "collect_string" and c["part2"] and c["part2"][0] > 0]
# Remaining result-id copies at 1e323058f3, priced below their bound in the part-2 run.
# (file, line, kind, row-occurrence-index or None, bound bytes for the copied id, class, branch, note)
P = "core/product_physics/src/"
REMAIN = [
    (P + "lib.rs", 5592, "clone_text", 0, 1024, "A", "W,X", "matched[0].id.clone(): matched: Vec<&ResultItem> (source_row_bindings :5570; called beside qualify_source_case_rows :5489-5491)"),
    (P + "source_receipt/rows.rs", 390, "clone_text", 0, 1024, "A", "X", "primary[&s.functional_indices[c]].id.clone(): primary: &BTreeMap<usize, ResultItem> (:332)"),
    (P + "source_receipt/rows.rs", 533, "clone_text", 0, 1024, "A", "X", "vec![primary[&function].id.clone()]"),
    (P + "source_receipt/rows.rs", 308, "format", 0, None, "A", "X", "format!(\"row semantic/value mismatch: {}\", actual.id): actual: &ResultItem (compare :306)"),
    (P + "source_receipt/rows.rs", 625, "format", 0, None, "A", "X", "format!(\"required derived row missing: {}\", d.row.id): d.row: ResultItem"),
    (P + "source_receipt/rows.rs", 590, "clone_text", 0, 1024, "B", "X", "binding.result_id.clone(): FunctionalRowBinding.result_id = matched[0].id (lib.rs:5592)"),
    (P + "source_receipt/rows.rs", 592, "clone_text", 0, 1024, "B", "X", "binding.result_id.clone()"),
    (P + "source_receipt.rs", 1016, "clone_text", 0, 1024, "B", "X", "r.result_id.clone(): RowTreatment.result_id = row.id (finalize_for :867)"),
    (P + "lib.rs", 13044, "to_string", 0, 872, "C", "W,X", "id: id.to_string() in append_endpoint_stress_result: ids from :12881 (872 B), :12898 (757), :12913 (765)"),
    (P + "lib.rs", 13074, "to_string", 0, 292, "C", "W,X", "id: id.to_string() in append_station_stress_result: ids from :12985 (292), :13002 (177), :13017 (185)"),
    (P + "lib.rs", 11617, "to_string", 0, 158, "C", "W,X", "id: id.to_string() in append_endpoint_force_result: ids from :11375-:11416 (<= 158)"),
    (P + "lib.rs", 11649, "to_string", 0, 173, "C", "W,X", "id: id.to_string(): station force ids from :11546-:11581 (<= 173)"),
    (P + "lib.rs", 4624, "clone_text", 0, 140, "C", "W,X", "result_ref: result_id.clone(); result_id from :4609 (140)"),
    (P + "lib.rs", 5276, "clone_text", 0, 322, "C", "W,X", "id: result_id.clone(); result_id from :5275 (322)"),
    (P + "lib.rs", 5281, "clone_text", 1, 322, "C", "W,X", "result_ref: result_id.clone() (second clone on the line)"),
    (P + "lib.rs", 5314, "clone_text", 0, 322, "C", "W,X", "id:result_id.clone(); result_id from :5307 (322)"),
    (P + "lib.rs", 5329, "clone_text", 0, 322, "C", "W,X", "result_ref: result_id.clone()"),
    (P + "lib.rs", 5359, "clone_text", 0, 142, "C", "W,X", "result_ref: result_id.clone(); result_id from :5349 (142)"),
]
rem = []
tot = {"whole": 0, "W": 0, "X": 0}
for f, line, kind, occ, bound, cls, br, note in REMAIN:
    rows = B.get((f, line, kind), [])
    r = rows[occ] if occ < len(rows) else None
    if r is None:
        rem.append({"site": f"{f}:{line}", "missing_row": True}); continue
    size = r["bytes"]
    new = (size - 128 + 1024) if bound is None else max(size, bound)
    req_new = r["mult"] * max(8, 2 * new)
    d = req_new - r["req"]
    rem.append({"site": f"{f}:{line}", "kind": kind, "class": cls, "branch": br, "mult": r["mult"], "priced": size,
                "bound": new, "req": r["req"], "req_at_bound": req_new, "delta": d, "note": note})
    tot["whole"] += d
    if "W" in br: tot["W"] += d
    if "X" in br: tot["X"] += d
INBUILD = {"W3_dense": 3_580_540_218, "W3_sparse": 3_560_829_770, "X1_dense": 3_308_986_800, "X1_sparse": 3_289_276_352}
M = 4_026_531_840; RULE = int(0.9 * M)
effect = {k: {"before": v, "after_upper": v + (tot["W"] if k.startswith("W") else tot["X"]),
              "fraction_after": round((v + (tot["W"] if k.startswith("W") else tot["X"])) / M, 4),
              "margin_to_0.9M_after": RULE - (v + (tot["W"] if k.startswith("W") else tot["X"]))} for k, v in INBUILD.items()}
out = {"rebase_TAV": a["total_text_requested_bytes"], "part2_TAV": b["total_text_requested_bytes"],
       "changed_rows": changed, "s2_positive_sites": len(s2_sites),
       "s2_delta": sum(c["delta"] for c in s2_sites), "collect_string_delta": sum(c["delta"] for c in changed if c["kind"] == "collect_string"),
       "zero_mult_changed": [c["site"] for c in changed if c["part2"] and c["part2"][0] == 0],
       "remaining": rem, "remaining_total": tot, "inbuild_effect_upper": effect}
print(json.dumps(out, indent=1))
