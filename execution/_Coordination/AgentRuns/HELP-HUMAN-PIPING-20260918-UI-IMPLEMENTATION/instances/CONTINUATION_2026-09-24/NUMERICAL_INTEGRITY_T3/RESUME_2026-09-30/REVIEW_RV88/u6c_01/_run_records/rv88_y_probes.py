"""RV88 (U6c review): RV78-N2's Y1-Y11 as discriminating pairs over all 17
successor statements (15 corpus bases + both milestones), plus the U6a Rust
derivatives, with shape-valid foreign members for Y4 and a real value row for
Y11, so a refusal can only come from the clause under test.
Usage: python rv88_y_probes.py <lane P> <u6a slice dir> <out.tsv>"""
import json
import sys
from copy import deepcopy
from pathlib import Path

P, SLICE, OUT = Path(sys.argv[1]), Path(sys.argv[2]), Path(sys.argv[3])
sys.path.insert(0, str(P))
sys.path.insert(0, str(P / "tests"))
from tests.schema_validation import validate_instance  # noqa: E402
from tests.test_load_reference_schema import _results_document  # noqa: E402  (document scaffold only)

SCHEMA = json.loads((P / "schemas/results.v0.3.schema.yaml").read_text())
PREVIEW = "openpipestress.result_semantics/0.3.0/preview-physics-1"
METHOD = "contribution_preserving_multiprecision_v1"
FOREIGN_SBR = json.loads((P / "fixtures/product_preview/source_blocks/n05-sparse_interactive.raw.json").read_text())["source_block_recovery"]


def ok(doc):
    try:
        validate_instance(SCHEMA, doc, instance_label="rv88")
        return True
    except AssertionError:
        return False


def doc_for(source):
    d = _results_document(source, source["producer"]["semantic_contract_id"])
    d["result_envelope"]["retained_precision"] = deepcopy(source["retained_precision"])
    return d


corpus = json.loads((P / "fixtures/results/retained_precision_cases.json").read_text())
statements = [(c["id"], c["source"]) for c in corpus["cases"]]
for mode in ("sparse_interactive", "dense_scrutiny"):
    statements.append((f"milestone_{mode}", json.loads((P / f"fixtures/results/retained_precision_milestone_successor_{mode}.json").read_text())["source"]))
docs = [(label, doc_for(s), s) for label, s in statements]
for f in sorted(SLICE.glob("cand_*_successor.json")):
    d = json.loads(f.read_text())
    docs.append((f"U6A_RUST/{f.name}", d, None))


def y11(e):
    vals = e["result_sets"][0]["values"]
    if not vals:
        return False
    vals[0]["recovery_method"] = METHOD
    return True


EDITS = {
    "Y1_receipt_policy_v1": lambda e: e["retained_precision"]["body"].__setitem__("policy", "M03-INTEGRITY-MP-v1"),
    "Y2_receipt_unknown_member": lambda e: e["retained_precision"]["body"].__setitem__("rv88", 1),
    "Y3_receipt_empty_body": lambda e: e.__setitem__("retained_precision", {"body": {}, "receipt_sha256": "0" * 64}),
    "Y4_source_block_recovery_shape_valid": lambda e: e.__setitem__("source_block_recovery", deepcopy(FOREIGN_SBR)),
    "Y5_preview_profile": lambda e: e["formulation_basis"].__setitem__("profile_id", "product_preview_mechanics_v1"),
    "Y6_limitations_changed": lambda e: e["formulation_basis"].__setitem__("limitations", e["formulation_basis"]["limitations"][:-1]),
    "Y7_contract_ref_mismatch": lambda e: e["semantic_contract_ref"].__setitem__("ref_id", PREVIEW),
    "Y8_no_contract_evidence": lambda e: e.pop("contract_evidence"),
    "Y11_real_value_row_with_token": y11,
    "missing_receipt": lambda e: e.pop("retained_precision"),
    "null_receipt": lambda e: e.__setitem__("retained_precision", None),
}
rows = []
for label, d, source in docs:
    base_ok = ok(d)
    rows.append(f"{label}\tY0_control\t{'valid' if base_ok else 'INVALID'}")
    for name, edit in EDITS.items():
        x = deepcopy(d)
        applied = edit(x["result_envelope"])
        if applied is False:
            rows.append(f"{label}\t{name}\tnot_applicable")
            continue
        rows.append(f"{label}\t{name}\t{'ADMITTED' if ok(x) else 'refused'}")
    if source is not None:
        proj = deepcopy(source); del proj["retained_precision"]
        proj["producer"]["semantic_contract_id"] = PREVIEW; proj["formulation_basis"]["profile_id"] = "product_preview_mechanics_v1"
        for r in proj["results"]:
            r.pop("recovery_method", None)
        pd = _results_document(proj, PREVIEW)
        rows.append(f"{label}\tY10_projection_on_base_branch\t{'valid' if ok(pd) else 'INVALID'}")
        pd["result_envelope"]["retained_precision"] = deepcopy(source["retained_precision"])
        rows.append(f"{label}\tY9_receipt_on_base_branch\t{'ADMITTED' if ok(pd) else 'refused'}")
OUT.write_text("\n".join(rows) + "\n")
print(len(docs), "documents")
