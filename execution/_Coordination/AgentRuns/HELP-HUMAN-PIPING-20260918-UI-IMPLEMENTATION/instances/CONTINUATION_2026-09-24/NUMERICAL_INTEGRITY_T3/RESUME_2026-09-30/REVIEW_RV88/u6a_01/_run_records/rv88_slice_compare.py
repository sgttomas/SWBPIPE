"""RV88 (U6a review): the slice, by independent derivation.

Oracles: the receipt's own selection lists (absolute_verified with bound bits,
not_covered, input_derived_dofs), the BASE lane's derivative of the reader
projection (base code at 7e4f5a51dd), and the retained/base semantic tables.
None of the candidate's carrier code is used here.
"""
import hashlib
import json
import struct
import sys
from pathlib import Path

S = Path(sys.argv[1])          # slice dir
FX = Path(sys.argv[2])         # candidate fixtures/results
out = []
fail = []


def resolve(doc, ptr):
    cur = doc
    for p in ptr.strip("/").split("/"):
        cur = cur[int(p)] if isinstance(cur, list) else cur[p]
    return cur


CHK = ("received_carrier_row_checksum", "original_producer_row_checksum")


def strip_ptr(x):
    # Row checksums bind the raw row, which carries the W1 token in the successor and
    # not in the projection; they are verified separately against JCS(successor row).
    return {k: v for k, v in x.items() if k not in ("target_field_path",) + CHK}


tables = {
    "retained": json.loads((FX / "semantic_contract_v0_3_preview_physics_retained_1.json").read_text()),
}
for mode in ("sparse_interactive", "dense_scrutiny"):
    raw_bytes = (FX / f"retained_precision_milestone_successor_{mode}.json").read_bytes()
    fixture = json.loads(raw_bytes)
    src = fixture["source"]
    sel = src["retained_precision"]["body"]["cases"][0]["selection"]
    absolute = {x["result_id"]: x["bound"] for x in sel["absolute_verified"]}
    not_cov = set(sel["not_covered"])
    succ = json.loads((S / f"cand_{mode}_successor.json").read_text())
    base_proj = json.loads((S / f"base_{mode}_projection.json").read_text())
    e, be = succ["result_envelope"], base_proj["result_envelope"]
    # (a) receipt byte-equal: canonical bytes of the carried receipt vs the file's receipt.
    carried = (S / f"cand_{mode}_receipt_from_derivative.json").read_text()
    src_canon = json.dumps(src["retained_precision"], sort_keys=True, separators=(",", ":"), ensure_ascii=False)
    car_canon = json.dumps(json.loads(carried), sort_keys=True, separators=(",", ":"), ensure_ascii=False)
    out.append(f"{mode}\treceipt_equal_python_canonical\t{src_canon == car_canon}")
    out.append(f"{mode}\treceipt_in_document_equal\t{e['retained_precision'] == src['retained_precision']}")
    out.append(f"{mode}\treceipt_sha256\t{src['retained_precision']['receipt_sha256']}")
    if src_canon != car_canon or e["retained_precision"] != src["retained_precision"]:
        fail.append(f"{mode}: receipt not equal")
    # (b) envelope-level copies.
    for key in ("producer", "numerical_quality", "formulation_basis", "contract_evidence"):
        if e[key] != src[key]:
            fail.append(f"{mode}: {key} not copied")
    if e["semantic_contract_ref"]["ref_id"] != src["producer"]["semantic_contract_id"]:
        fail.append(f"{mode}: semantic_contract_ref")
    # (c) per-row dispositions against the receipt's lists and the base projection's derivative.
    rows = src["results"]
    acc, bacc = e["row_accounting"], be["row_accounting"]
    if len(acc) != len(rows) or len(bacc) != len(rows):
        fail.append(f"{mode}: accounting cardinality {len(acc)} {len(bacc)} {len(rows)}")
    n_abs = n_same = 0
    for i, row in enumerate(rows):
        a, b = acc[i], bacc[i]
        rid = row["id"]
        if a["source_result_id"] != rid or a["source_row_index"] != i:
            fail.append(f"{mode}: accounting order at {i}")
        target = resolve(succ, a["target_field_path"])
        btarget = resolve(base_proj, b["target_field_path"])
        if rid in absolute or rid in not_cov:
            n_abs += rid in absolute
            want_code = "retained_precision_absolute_verified" if rid in absolute else "retained_precision_not_covered"
            ok = a["disposition"] == "disclosed" and target["reason_code"] == want_code
            ok &= target["source_value"] == row["value"] and target["source_unit"] == row.get("unit")
            ok &= target["source_result_id"] == rid and target["source_kind"] == row["kind"]
            if rid in absolute:
                bits = absolute[rid]
                bval = struct.unpack(">d", bytes.fromhex(bits))[0]
                msg = target["message"]
                ok &= f"(binary64 {bits})" in msg
                # Rust {:e} prints the shortest round-trip mantissa: parse it back to the same bits.
                dec = msg.split("b = ", 1)[1].split(" ", 1)[0]
                ok &= struct.pack(">d", float(dec)).hex() == bits
                ok &= "stop" not in msg and "enclos" not in msg and "interval" not in msg
                # Everything else about the disclosure matches what base would have disclosed/exported for the row.
                for k in ("object_ref", "source_field_path", "received_carrier_row_checksum", "original_producer_row_checksum", "source_annotation_ref"):
                    if k in target and k in btarget and target[k] != btarget[k]:
                        ok = False
                out.append(f"{mode}\tabs\t{rid}\t{row['kind']}\t{row.get('unit')}\tbase_disposition={b['disposition']}\tb={dec}")
            if not ok:
                fail.append(f"{mode}: class row {rid}: {a['disposition']} {target.get('reason_code')}")
        else:
            n_same += 1
            if strip_ptr(a) != strip_ptr(b) or strip_ptr(target) != strip_ptr(btarget):
                fail.append(f"{mode}: non-class row {rid} differs from base projection: {a['disposition']} vs {b['disposition']}")
            if target.get("reason_code", "").startswith("retained_precision_"):
                fail.append(f"{mode}: non-class row {rid} claims a class code")
    out.append(f"{mode}\tabsolute_rows_disclosed\t{n_abs}\tnot_covered\t{len(not_cov)}\trows_identical_to_base\t{n_same}\trows\t{len(rows)}")
    # (d) the remaining document parts equal base's except the expected envelope-level changes.
    diff_keys = sorted(k for k in set(e) | set(be) if e.get(k) != be.get(k))
    out.append(f"{mode}\tenvelope_keys_differing_from_base_projection\t{diff_keys}")
    # Quantity values: successor's exported quantities are base's minus the class rows.
    bvals = {v.get("result_id") or json.dumps(v.get("object_ref")) + v.get("quantity_kind", ""): v for v in be["result_sets"][0]["values"]}
    out.append(f"{mode}\tquantity_values\tsucc={len(e['result_sets'][0]['values'])}\tbase={len(be['result_sets'][0]['values'])}")
    out.append(f"{mode}\tdisclosures\tsucc={len(e['row_disclosures'])}\tbase={len(be['row_disclosures'])}")

# (e) the retained table keeps preview-physics-1's dispositions per kind ("table disposition").
rt = tables["retained"]
out.append("retained_table_keys\t" + ",".join(sorted(rt.keys())))
(S / "compare.tsv").write_text("\n".join(out) + "\n")
print("\n".join(l for l in out if "\tabs\t" not in l))
print("FAILURES:", len(fail))
for f in fail:
    print("  ", f)
