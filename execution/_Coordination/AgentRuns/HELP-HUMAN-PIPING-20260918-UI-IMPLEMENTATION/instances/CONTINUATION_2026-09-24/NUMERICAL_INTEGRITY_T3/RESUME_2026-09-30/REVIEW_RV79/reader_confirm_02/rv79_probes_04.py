"""RV79 confirmation, third probe batch: a selected case left without its C3 attempt (code moved
from PRODUCT_ATTEMPT to ATTEMPT when the check moved into _g5_ordinary)."""
import json, sys, traceback
from copy import deepcopy
sys.path.insert(0, ".")
from core.analysis_runs import retained_precision as rp
from tests.test_retained_precision_contract import apply_entry, corpus
C = corpus(); BASE = {f["id"]: f for f in C["cases"]}
B = ["retained_precision", "body"]
def S(path, value): return {"path": B + path, "op": "set", "value": value}
TC = "two_case_synthetic"; b = BASE[TC]["source"]["retained_precision"]["body"]
a1 = deepcopy(b["product_attempts"][1]); a1["id"] = 0
edits = [S(["product_attempts"], [a1]), S(["cases", 0, "product_attempt_ref"], None), S(["cases", 1, "product_attempt_ref"], 0),
         S(["sources", 0, "preparation"], None), S(["sources", 1, "preparation", "attempt_ref"], 0)]
src, inv = apply_entry(BASE[TC], {"base": TC, "edits": edits, "rehash": "all"})
try:
    rp._validate_draft(src, inv); got, lines = "PASS", None
except rp.RetainedPrecisionError as e:
    got, lines = f"{e.gate} {e.code}", [f.lineno for f in traceback.extract_tb(e.__traceback__) if f.filename.endswith("retained_precision.py")]
print(json.dumps({"id": "selected_case_without_c3_attempt", "base": TC, "python": got, "raise_lines": lines,
                  "rv79_reading": "G5 RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH (C3 reference, D16) or an earlier gate"}))
