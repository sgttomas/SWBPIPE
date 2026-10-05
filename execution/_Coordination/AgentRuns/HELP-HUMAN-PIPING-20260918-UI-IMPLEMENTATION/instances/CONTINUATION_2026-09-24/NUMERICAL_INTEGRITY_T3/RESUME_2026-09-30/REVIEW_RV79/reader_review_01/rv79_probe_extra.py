"""RV79 extra probes (run from P on a clean archived copy with both BIN variables set).

1. m06: does the shared pin `cert_failed_before_summary_g5a_passed` isolate I57 s3's
   "a passed G5a requires non-null coverage"? Re-run with mutant M06 applied in memory.
2. malformed top-level inputs: every one must fail with a typed G-gate error.
3. unsafe or non-integral U values that cannot be rehashed.
"""
import sys, types, traceback
from copy import deepcopy
sys.path.insert(0, ".")
from core.analysis_runs import retained_precision as rp
from tests.test_retained_precision_contract import apply_entry, corpus

C = corpus(); BASE = {f["id"]: f for f in C["cases"]}

def outcome(fn):
    try:
        r = fn(); return "PASS (eligible=%s)" % r["numerical_eligible"]
    except rp.RetainedPrecisionError as e:
        tb = [f.lineno for f in traceback.extract_tb(e.__traceback__) if f.filename.endswith("retained_precision.py")]
        return f"{e.gate} {e.code} lines={tb}"
    except Exception as e:
        return f"UNTYPED {type(e).__name__}"

print("== 1. M06 isolation")
m = deepcopy(next(x for x in C["mutations"] if x["id"] == "cert_failed_before_summary_g5a_passed"))
src, inv = apply_entry(BASE[m["base"]], m)
print("shared entry, reviewed reader:", outcome(lambda: rp._validate_draft(src, inv)))
text = open(rp.__file__).read()
old = '            or stages["g5a"] == "completed" or checks["g5a"]["kind"] == "passed"):'
assert text.count(old) == 1
mod = types.ModuleType("rv79_m06"); mod.__file__ = rp.__file__; mod.__package__ = "core.analysis_runs"
exec(compile(text.replace(old, "            ):"), rp.__file__, "exec"), mod.__dict__)
def under_m06(entry):
    s, i = apply_entry(BASE[entry["base"]], entry)
    try:
        mod._validate_draft(s, i); return "PASS"
    except mod.RetainedPrecisionError as e:
        return f"{e.gate} {e.code}"
print("shared entry, M06 reader:", under_m06(m))
iso = deepcopy(m)
for ed in iso["edits"]:
    if ed["path"][-1] == "stages": ed["value"]["observables"] = "completed"
    if ed["path"][-1] == "checks": ed["value"]["observables"] = {"kind": "passed"}
s2, i2 = apply_entry(BASE[iso["base"]], iso)
print("isolating variant (observables completed+passed), reviewed reader:", outcome(lambda: rp._validate_draft(s2, i2)))
print("isolating variant, M06 reader:", under_m06(iso))

print("== 2. malformed top-level inputs")
base = C["cases"][0]
def t(name, mutate=None, inv=True, replace=None):
    s = deepcopy(base["source"]); i = deepcopy(base["invocation"])
    if replace is not None: s = replace
    elif mutate is not None: mutate(s)
    print(name, outcome(lambda: rp._validate_draft(s, i if inv else None)))
t("producer_list", lambda s: s.__setitem__("producer", []))
t("formulation_basis_str", lambda s: s.__setitem__("formulation_basis", "x"))
t("retained_precision_list", lambda s: s.__setitem__("retained_precision", []))
t("source_is_list", replace=[])
t("body_product_attempts_dict", lambda s: s["retained_precision"]["body"].__setitem__("product_attempts", {}))
t("results_not_list", lambda s: s.__setitem__("results", {}))
t("numerical_quality_missing", lambda s: s.pop("numerical_quality"))
t("diagnostics_missing", lambda s: s.pop("diagnostics"))
t("no_invocation_base", inv=False)

print("== 3. unsafe or non-integral U without rehash")
for name, value in (("charged_2^53", 2 ** 53), ("charged_1.5", 1.5), ("charged_integral_float", "float")):
    t(name, lambda s, v=value: s["retained_precision"]["body"]["work"].__setitem__("charged", float(s["retained_precision"]["body"]["work"]["charged"]) if v == "float" else v))
