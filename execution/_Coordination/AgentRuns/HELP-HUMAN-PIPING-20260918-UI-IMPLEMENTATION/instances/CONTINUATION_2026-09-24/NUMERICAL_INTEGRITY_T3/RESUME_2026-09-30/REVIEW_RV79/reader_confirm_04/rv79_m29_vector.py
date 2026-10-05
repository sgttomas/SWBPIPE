"""RV79 confirmation 04: a single-defect vector that separates M29 (_integral truncating non-integral floats)."""
import json, sys, types, traceback
from copy import deepcopy
sys.path.insert(0, ".")
from core.analysis_runs import retained_precision as rp
from tests.test_retained_precision_contract import corpus
C = corpus(); f = next(x for x in C["cases"] if x["id"] == "ordinary_prepared_synthetic")
text = open(rp.__file__).read()
old = "if type(value) is float and math.isfinite(value) and value == int(value) and not"
assert text.count(old) == 1
m29 = types.ModuleType("rv79_m29"); m29.__file__ = rp.__file__; m29.__package__ = "core.analysis_runs"
exec(compile(text.replace(old, "if type(value) is float and math.isfinite(value) and not"), rp.__file__, "exec"), m29.__dict__)
def outcome(mod, path, value):
    src = deepcopy(f["source"]); b = src["retained_precision"]["body"]; t = b
    for k in path[:-1]: t = t[k]
    t[path[-1]] = value
    src["retained_precision"]["receipt_sha256"] = rp._hash("retained_precision_receipt_mp_v2", b)
    try: mod._validate_draft(src, deepcopy(f["invocation"])); return "PASS"
    except mod.RetainedPrecisionError as e: return f"{e.gate} {e.code}"
for path, value in ((["receipt_version"], 1.5), (["work", "case_limit"], 20000000000.5), (["work", "invocation_limit"], 60000000000.5)):
    print(json.dumps({"field": "/".join(path), "value": value, "reviewed_reader": outcome(rp, path, value), "M29_reader": outcome(m29, path, value), "ruled": "G0 SOURCE_PRODUCER_CONTRACT_UNSUPPORTED (D2/D32)"}))
