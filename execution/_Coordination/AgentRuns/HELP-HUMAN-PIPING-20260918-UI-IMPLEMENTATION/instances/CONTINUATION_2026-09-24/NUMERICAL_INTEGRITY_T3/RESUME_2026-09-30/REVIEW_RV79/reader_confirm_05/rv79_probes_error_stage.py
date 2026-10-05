"""RV79 confirmation 05 (incidental): an unavailable attempt's PublicFailure kind against its stage
record, in the direction error -> stage. Base F' case 1: every pipeline stage through certificate
completed, certificate check passed, observables and g5a not entered, error capture{storage}."""
import json, sys, traceback
sys.path.insert(0, ".")
from core.analysis_runs import retained_precision as rp
from tests.test_retained_precision_contract import apply_entry, corpus
C = corpus(); BASE = {f["id"]: f for f in C["cases"]}
B = ["retained_precision", "body"]; F2 = "two_case_facade_after_certificate_synthetic"
def run(error):
    src, inv = apply_entry(BASE[F2], {"base": F2, "edits": [{"path": B + ["product_attempts", 1, "result"], "op": "set", "value": {"kind": "unavailable", "error": error}}], "rehash": "all"})
    try:
        rp._validate_draft(src, inv); return "PASS"
    except rp.RetainedPrecisionError as e:
        return f"{e.gate} {e.code} lines={[f.lineno for f in traceback.extract_tb(e.__traceback__) if f.filename.endswith('retained_precision.py')]}"
storage = {"kind": "storage"}
cases = {
  "control_capture_storage (base)": {"kind": "capture", "cause": {"kind": "storage", "detail": "adapter vector"}},
  "g5a_error_but_g5a_not_entered": {"kind": "g5a", "cause": {"kind": "sanity", "body": 0, "quantity_kind": 0}},
  "observable_error_but_observables_not_entered": {"kind": "observable", "cause": {"kind": "storage", "detail": "x"}},
  "proof_error_but_certificate_passed": {"kind": "proof", "cause": storage},
  "values_error_but_values_completed": {"kind": "values", "cause": storage, "proof": {"kind": "association", "detail": "PP abandoned prepared draft"}},
  "numeric_error_null_cause": {"kind": "numeric", "cause": None},
}
for name, err in cases.items():
    print(json.dumps({"id": name, "python": run(err)}))
