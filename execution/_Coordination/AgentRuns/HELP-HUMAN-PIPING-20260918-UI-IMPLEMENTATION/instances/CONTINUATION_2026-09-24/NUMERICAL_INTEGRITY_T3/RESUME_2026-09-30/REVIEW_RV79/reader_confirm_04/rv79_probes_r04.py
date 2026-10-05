"""RV79 confirmation 04 (READER abcb16fd27): booleans where an integer is required, -0.0 on the
enum/const integer fields, and the D32 helpers. Each probe edits the parsed base and re-hashes only
the receipt (the edits touch no source binding), then validates against the base invocation."""
import json, sys, traceback
from copy import deepcopy
sys.path.insert(0, ".")
from core.analysis_runs import retained_precision as rp
from tests.test_retained_precision_contract import corpus
C = corpus(); BASE = {f["id"]: f for f in C["cases"]}
def outcome(src, inv):
    try:
        r = rp._validate_draft(src, inv); return "PASS (standing %s)" % r["standing"], None
    except rp.RetainedPrecisionError as e:
        return f"{e.gate} {e.code}", [f.lineno for f in traceback.extract_tb(e.__traceback__) if f.filename.endswith("retained_precision.py")]
    except Exception as e:
        return f"UNTYPED {type(e).__name__}", None
out = []
def probe(pid, base, mutate, ruled, basis):
    f = BASE[base]; src = deepcopy(f["source"]); body = src["retained_precision"]["body"]; mutate(body)
    src["retained_precision"]["receipt_sha256"] = rp._hash("retained_precision_receipt_mp_v2", body)
    got, lines = outcome(src, deepcopy(f["invocation"]))
    ok = (got != "PASS (standing needs_recompute)" and not got.startswith("UNTYPED")) if ruled == "reject" else got == ruled
    out.append({"id": pid, "base": base, "python": got, "raise_lines": lines, "ruled": ruled, "basis": basis, "agrees": ok})
O, TC = "ordinary_prepared_synthetic", "two_case_synthetic"
G0U, G1R = "G0 SOURCE_PRODUCER_CONTRACT_UNSUPPORTED", "G1 RETAINED_PRECISION_RECEIPT_MISMATCH"
def setp(path, value):
    def m(b):
        t = b
        for k in path[:-1]: t = t[k]
        t[path[-1]] = value
    return m
# booleans: G0 fields fail G0 (D2 "absent or of the wrong type"); elsewhere a JSON boolean is not a number (G1 closed shape)
probe("bool_receipt_version", O, setp(["receipt_version"], True), G0U, "D2/D32")
probe("bool_case_limit", O, setp(["work", "case_limit"], True), G0U, "D2/D32")
probe("bool_charged", O, setp(["work", "charged"], True), G1R, "D32: never a bool; G1 shape")
probe("bool_case_source_ref", O, setp(["cases", 0, "source_ref"], True), G1R, "D32 + G1 (the identity check must not be skipped silently into a pass)")
probe("bool_preparation_attempt_ref", TC, setp(["sources", 1, "preparation", "attempt_ref"], True), G1R, "D32")
probe("bool_old_member", O, setp(["product_attempts", 0, "operational", "old", 0, "member"], False), G1R, "D32")
probe("bool_execution_order_index", O, setp(["work", "execution_order", 0, "index"], False), G1R, "D32")
probe("bool_record_index", O, setp(["cases", 0, "run", "records", 0, "index"], False), G1R, "D32")
# -0.0 on U fields (G2 encoding) and on schema enum/const integer fields
probe("negzero_charged_U", O, setp(["work", "charged"], -0.0) if body_charged_zero else setp(["work", "charged"], -0.0), "reject", "C1 G2: no -0 counter") if (body_charged_zero := False) or True else None
probe("negzero_execution_order_index_U", O, setp(["work", "execution_order", 0, "index"], -0.0), "G2 RETAINED_PRECISION_ENCODING_MISMATCH", "C1 G2 / D32: not -0")
for p in out: print(json.dumps(p))
# Unit level: -0.0 against enum/const integer specs (quantity_kind enum [0,1]; constructor_counts.directional_springs const 0)
S = rp._schema()["$defs"]
specs = {"G5aError.quantity_kind enum[0,1]": S["G5aError"]["oneOf"][3]["properties"]["quantity_kind"],
         "source_decline.constructor_counts.directional_springs const 0": S["Case"]["oneOf"][2]["properties"]["source_decline"]["properties"]["constructor_counts"]["properties"]["directional_springs"]}
unit = {}
for name, spec in specs.items():
    try:
        rp._encoding(-0.0, spec); enc = "accepted"
    except rp.RetainedPrecisionError as e:
        enc = f"{e.gate} {e.code}"
    v = {"x": -0.0}; rp._normalize_integrals(v)
    unit[name] = {"G1 _shape(-0.0)": rp._shape(-0.0, spec), "G2 _encoding(-0.0)": enc, "normalized": repr(v["x"]), "_integral(-0.0)": rp._integral(-0.0)}
print(json.dumps({"unit_negative_zero_enum_const": unit}))
print(json.dumps({"unit_integral": {repr(x): rp._integral(x) for x in (1, 1.0, -0.0, 0.0, 1.5, True, float("inf"))}}))
