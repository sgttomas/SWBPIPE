"""RV79 confirmation 05: D34 (-0 anywhere in the receipt fails G2) and its +0 controls, run on the
head under review and, for comparison, on the previous head. Edits go through the shared harness
(rehash "all"); results-row edits check that D34 stays receipt-only."""
import json, sys, traceback
from copy import deepcopy
sys.path.insert(0, ".")
from core.analysis_runs import retained_precision as rp
from tests.test_retained_precision_contract import apply_entry, corpus
C = corpus(); BASE = {f["id"]: f for f in C["cases"]}
B = ["retained_precision", "body"]
def S(path, value): return {"path": B + path, "op": "set", "value": value}
def run(base, edits):
    src, inv = apply_entry(BASE[base], {"base": base, "edits": edits, "rehash": "all"})
    try:
        r = rp._validate_draft(src, inv)
        return "PASS" + ("" if r["classifications"] == BASE[base]["expected_classifications"] else " (classifications differ)"), None
    except rp.RetainedPrecisionError as e:
        return f"{e.gate} {e.code}", [f.lineno for f in traceback.extract_tb(e.__traceback__) if f.filename.endswith("retained_precision.py")]
F2, PP2, O = "two_case_facade_after_certificate_synthetic", "two_case_preparation_failure_synthetic", "ordinary_prepared_synthetic"
A1 = ["product_attempts", 1]
out = []
for v in (-0.0, 0, 0.0, 1):
    sanity = {"kind": "unavailable", "error": {"kind": "g5a", "cause": {"kind": "sanity", "body": 0, "quantity_kind": v}}}
    got, lines = run(F2, [S(A1 + ["result"], sanity)])
    out.append({"id": f"quantity_kind_{v!r}", "field": "G5aError.quantity_kind (enum [0,1])", "value": repr(v), "python": got, "raise_lines": lines})
for v in (-0.0, 0, 0.0):
    decline = {"input_owner": {"case_index": 1, "case_id": "case:unavailable-row", "material_basis_ref": 0},
               "constructor_counts": {"nodes": 0, "members": 1, "springs": 0, "constraints": 6, "nodal_terms": 6, "stations": 3, "supports": 1, "id_utf8_bytes": 0, "directional_springs": v},
               "error": {"tag": "no_nodes"}}
    got, lines = run(PP2, [S(["cases", 1, "source_decline"], decline)])
    out.append({"id": f"directional_springs_{v!r}", "field": "constructor_counts.directional_springs (const 0)", "value": repr(v), "python": got, "raise_lines": lines})
for v in (-0.0, 0.0, 0):
    got, lines = run(O, [S(["cases", 0, "run", "records", 0, "corrections"], v)])
    out.append({"id": f"U_corrections_{v!r}", "field": "PhysicalRecord.corrections (U)", "value": repr(v), "python": got, "raise_lines": lines})
# -0 nested in a list of integers (layout_bytes is [U;8]) and in a deeper list (adapter counts [U;10])
for path, label in ((A1 + ["preparation", "members", 0, "work", "layout_bytes", 0], "PreparationWork.layout_bytes[0] (U in a list)") if False else (["product_attempts", 0, "adapter", "counts", 0], "AdapterTrace.counts[0] (U in a list)"),):
    base_v = BASE[O]["source"]["retained_precision"]["body"]["product_attempts"][0]["adapter"]["counts"][0]
    for v in ((-0.0,) if base_v == 0 else ()) + (float(base_v),):
        got, lines = run(O, [S(path, v)])
        out.append({"id": f"list_item_{v!r}", "field": label, "value": repr(v), "base_value": base_v, "python": got, "raise_lines": lines})
# outside the receipt: a results row value of -0.0 is not a receipt number (D34 is receipt-only)
row_i = next(i for i, r in enumerate(BASE[O]["source"]["results"]) if r["value"] == 0)
src_row = BASE[O]["source"]["results"][row_i]
got, lines = run(O, [{"path": ["results", row_i, "value"], "op": "set", "value": -0.0}])
out.append({"id": "results_row_value_negative_zero", "field": f"results[{row_i}].value (outside the receipt)", "value": "-0.0", "python": got, "raise_lines": lines})
for p in out: print(json.dumps(p))
