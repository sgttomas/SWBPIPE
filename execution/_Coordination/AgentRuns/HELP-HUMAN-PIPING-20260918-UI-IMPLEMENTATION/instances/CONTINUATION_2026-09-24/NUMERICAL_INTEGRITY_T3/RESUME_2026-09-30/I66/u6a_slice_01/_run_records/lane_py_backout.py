"""I66 U6a control 2, Python lane (scratch only): the receipt carried by the Rust
derivative is byte-equal to the source's and revalidates in the Python reader."""
import json, sys
from copy import deepcopy
from pathlib import Path
P = Path(sys.argv[1]); OUT = Path(sys.argv[2])
sys.path.insert(0, str(P))
from core.analysis_runs import retained_precision as rp
for mode in ("sparse_interactive", "dense_scrutiny"):
    fixture = json.loads((P / f"fixtures/results/retained_precision_milestone_successor_{mode}.json").read_text())
    doc = json.loads((OUT / f"derivative_{mode}.json").read_text())
    carried = doc["result_envelope"]["retained_precision"]
    canon = lambda v: json.dumps(v, sort_keys=True, separators=(",", ":"), ensure_ascii=False)
    assert canon(carried) == canon(fixture["source"]["retained_precision"]), mode
    before = rp.validate_retained_precision(deepcopy(fixture["source"]), deepcopy(fixture["invocation"]))
    back = deepcopy(fixture["source"]); back["retained_precision"] = carried
    after = rp.validate_retained_precision(back, deepcopy(fixture["invocation"]))
    assert after == before and after["numerical_eligible"] is False and after["standing"] == "needs_recompute", mode
    reasons = [x["reason_code"] for x in doc["result_envelope"]["row_disclosures"]]
    absolute = sorted(c["result_id"] for c in after["classifications"] if c["class"] == "absolute_verified")
    disclosed = sorted(x["source_result_id"] for x in doc["result_envelope"]["row_disclosures"] if x["reason_code"] == "retained_precision_absolute_verified")
    assert absolute == disclosed, mode
    for x in doc["result_envelope"]["row_disclosures"]:
        if x["reason_code"] == "retained_precision_absolute_verified":
            c = next(c for c in after["classifications"] if c["result_id"] == x["source_result_id"])
            assert c["bound_bits"] in x["message"], mode
    print(f"{mode}: receipt byte-equal; python reader revalidates ({len(after['classifications'])} classes; {len(absolute)} absolute disclosed; eligible={after['numerical_eligible']})")
