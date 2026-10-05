"""RV91: validate the TS-built successor AnalysisRun records against U6c's
schemas (as committed at c89a7a986c), with the repo's own helper; also a
negative control (the receipt dropped) and a positive base control."""
import json, sys
from copy import deepcopy
from pathlib import Path
U = Path(sys.argv[1]); S = Path(sys.argv[2])
sys.path.insert(0, str(U))
from tests.schema_validation import validate_instance
out = {}
for name in ["analysis_run.schema.json", "analysis_run.v0.3.schema.json"]:
    schema = json.loads((U / "schemas" / name).read_text())
    for mode in ["sparse_interactive", "dense_scrutiny"]:
        rec = json.loads((S / f"review/review_cand.json.analysis_run_{mode}.json").read_text())
        try: validate_instance(schema, rec, instance_label=mode); out[f"{name}:{mode}"] = "valid"
        except AssertionError as e: out[f"{name}:{mode}"] = "INVALID: " + str(e)[:300]
        dropped = deepcopy(rec); del dropped["analysis_run"]["retained_precision"]
        try: validate_instance(schema, dropped, instance_label=mode); out[f"{name}:{mode}:dropped"] = "valid (control failed)"
        except AssertionError as e: out[f"{name}:{mode}:dropped"] = "refused (control ok)"
print(json.dumps(out, indent=1))
(S / "review/schema_check.json").write_text(json.dumps(out, indent=1))
