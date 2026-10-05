"""RV79 confirmation, second probe batch: the converse of D4c (an unavailable C3 attempt whose
case claims a C2 cause instead of prepared_product_failure), which also bypasses the D4d table."""
import json, sys, traceback
from copy import deepcopy
sys.path.insert(0, ".")
from core.analysis_runs import retained_precision as rp
from tests.test_retained_precision_contract import apply_entry, corpus

C = corpus(); BASE = {f["id"]: f for f in C["cases"]}
B = ["retained_precision", "body"]
def S(path, value): return {"path": B + path, "op": "set", "value": value}
def run(base, edits):
    source, invocation = apply_entry(BASE[base], {"base": base, "edits": edits, "rehash": "all"})
    try:
        rp._validate_draft(source, invocation); return "PASS", None
    except rp.RetainedPrecisionError as e:
        return f"{e.gate} {e.code}", [f.lineno for f in traceback.extract_tb(e.__traceback__) if f.filename.endswith("retained_precision.py")]
F2 = "two_case_facade_after_certificate_synthetic"; PP2 = "two_case_preparation_failure_synthetic"
G5P = "G5 RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH"
receipt_cause = {"kind": "receipt_failure", "check": "association", "field_path": "x"}
precondition = {"kind": "unavailable_precondition", "precondition": "capture", "affected_refs": []}
prep_err = {"kind": "preparation", "capture": {"kind": "storage", "detail": "adapter vector"}, "section": None}
out = []
def probe(pid, base, edits, ruled, basis):
    got, lines = run(base, edits)
    out.append({"id": pid, "base": base, "python": got, "raise_lines": lines, "ruled": ruled, "basis": basis, "agrees": got == ruled})
probe("C2cause_on_unavailable_attempt_receipt_failure", F2, [S(["cases", 1, "reason", "cause"], receipt_cause)], G5P,
      "S06:25-30: ppf is the branch for an actual unavailable C3 attempt; C2 branches only without one (receipt_failure only after a Ready product)")
probe("C2cause_on_unavailable_attempt_precondition", F2, [S(["cases", 1, "reason", "cause"], precondition)], G5P, "S06:25-30")
probe("R6c_via_C2cause_preparation_error_selected_run", F2,
      [S(["product_attempts", 1, "result", "error"], prep_err), S(["cases", 1, "reason", "cause"], receipt_cause)], G5P,
      "S06:37 (D4d) evaded when the case does not claim ppf")
probe("Pprime_C2cause_on_unavailable_attempt", PP2, [S(["cases", 1, "reason", "cause"], precondition)], G5P, "S06:25-30")
for p in out: print(json.dumps(p))
