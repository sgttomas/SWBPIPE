"""I66 U6c lane (scratch only): U6a's Rust-produced successor derivatives (both
modes) against the candidate results.v0.3, which closes U6a's F2."""
import json, sys
from copy import deepcopy
from pathlib import Path
P = Path(sys.argv[1]); OUT = Path(sys.argv[2]); sys.path.insert(0, str(P))
from tests.schema_validation import validate_instance
schema = json.loads((P / "schemas/results.v0.3.schema.yaml").read_text())
for mode in ("sparse_interactive", "dense_scrutiny"):
    doc = json.loads((OUT / f"derivative_{mode}.json").read_text())
    n = sum(1 for x in doc["result_envelope"]["row_disclosures"] if x["reason_code"].startswith("retained_precision_"))
    validate_instance(schema, doc, instance_label=mode)
    relabel = deepcopy(doc); e = relabel["result_envelope"]
    e.pop("retained_precision"); e["producer"]["semantic_contract_id"] = "openpipestress.result_semantics/0.3.0/preview-physics-1"
    e["semantic_contract_ref"]["ref_id"] = e["producer"]["semantic_contract_id"]; e["formulation_basis"]["profile_id"] = "product_preview_mechanics_v1"
    try:
        validate_instance(schema, relabel, instance_label="relabelled"); verdict = "ACCEPTED (unexpected)"
    except AssertionError as err:
        verdict = "refused: " + str(err).splitlines()[1][:160]
    print(f"{mode}: derivative valid under results.v0.3 ({n} class disclosures); relabelled to preview-physics-1 without receipt: {verdict}")
