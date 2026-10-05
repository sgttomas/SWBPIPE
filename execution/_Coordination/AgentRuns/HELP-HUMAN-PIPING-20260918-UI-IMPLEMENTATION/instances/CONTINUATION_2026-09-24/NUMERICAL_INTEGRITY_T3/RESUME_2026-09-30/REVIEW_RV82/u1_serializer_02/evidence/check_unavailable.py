#!/usr/bin/env python3
"""RV82 grant-2 item 4: the unavailable representations (not publications).
For each file: schema-validate retained_precision; derive the expected (code, phase)
from the readers' accepted D4d table (S06; PY:1049-1056, RS:~1988, TS:~752) and the
receipt's own error kind and Run, and compare; check D38's nulls; run the Python
reader and record its first failure (expected G3 under T3)."""
import json, os, sys, glob
import jsonschema
P, D = os.environ["P"], os.environ["DIR"]
sys.path.insert(0, P)
from core.analysis_runs import retained_precision as rp
schema = json.load(open(f"{P}/schemas/retained_precision_mp_v2.schema.json"))
val = jsonschema.Draft202012Validator(schema)
def d4d(error, run):
    if error == "preparation" or (error == "capture" and run is None): return ("source_unavailable", "preparation")
    if error == "native" or (error == "capture" and run["kernel_terminal"]["kind"] != "selected"): return ("kernel_" + run["kernel_terminal"]["kind"], "kernel")
    return ("facade_certificate", "facade")
for f in sorted(glob.glob(f"{D}/g2u_*.json")):
    c = json.load(open(f)); s = c["source"]; b = s["retained_precision"]["body"]
    case, a = b["cases"][0], b["product_attempts"][0]
    errs = list(val.iter_errors(s["retained_precision"]))
    err = a["result"]["error"]["kind"]; run = case.get("run")
    exp = d4d(err, run)
    got = (case["reason"]["code"], case["reason"]["phase"])
    notes = []
    if run is None:
        notes.append("D38-shape " + str(all([case["source_ref"] is None, a["run_ref"] is None, a["source_ref"] is None, b["work"]["execution_order"] == [],
            b["calls"] == [], b["work"]["charged"] == 0, b["sources"] == []])))
    else:
        notes.append(f"run_ref={a['run_ref']} run.id={run['id']} terminal={run['kernel_terminal']['kind']}")
    unav = [d for d in s["diagnostics"] if d["code"] == "RETAINED_PRECISION_UNAVAILABLE"]
    legacy = [d for d in s["diagnostics"] if d["code"] == "SOURCE_BLOCK_RECOVERY_UNAVAILABLE"]
    notes.append(f"unavailable_diag={len(unav)} ref_ok={len(unav)==1 and case['diagnostic_ref']==unav[0]['id']} legacy_kept={len(legacy)} "
                 f"legacy_source={b['ordinary_attempts'][0]['legacy_source']} method_tokens={sum(1 for r in s['results'] if 'recovery_method' in r)}")
    try:
        rp._validate_draft(s, c["invocation"]); first = "PASS (UNEXPECTED under T3)"
    except rp.RetainedPrecisionError as e:
        first = f"{e.gate} {e.code}"
    print(f"{os.path.basename(f)}: error={err} code/phase={got} D4d-expected={exp} {'MATCH' if got == exp else 'MISMATCH'}; schema_violations={len(errs)}; python_first={first}")
    for n in notes: print("    " + n)
