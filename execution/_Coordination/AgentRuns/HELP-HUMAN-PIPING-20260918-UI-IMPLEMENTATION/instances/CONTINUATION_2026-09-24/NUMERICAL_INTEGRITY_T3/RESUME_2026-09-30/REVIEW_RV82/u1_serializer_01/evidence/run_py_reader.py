#!/usr/bin/env python3
"""RV82: the unchanged accepted Python reader (_validate_draft; eligibility off) on U1's files."""
import json, os, sys, traceback
from pathlib import Path
P = Path(os.environ["P"]); D = Path(os.environ["RV82_READ_DIR"])
sys.path.insert(0, str(P))
from core.analysis_runs import retained_precision as rp
import jsonschema
schema = json.loads((P / "schemas/retained_precision_mp_v2.schema.json").read_text())
val = jsonschema.Draft202012Validator(schema)
out = []
print("IMPLEMENTATION_COMPLETE", rp._IMPLEMENTATION_COMPLETE)
for f in sorted(D.glob("u1_milestone_*.json")):
    c = json.loads(f.read_text())
    errs = list(val.iter_errors(c["source"]["retained_precision"]))
    try:
        r = rp._validate_draft(c["source"], c["invocation"])
        out.append(f"PY {f.name} PASS standing={r['standing']} eligible={r['numerical_eligible']} invocation_bound={r['invocation_bound']} classes={len(r['classifications'])} schema_violations={len(errs)}")
        (D / (f.name + ".py_classes.txt")).write_text("".join(f"{x['result_id']}|{x['normalized_bits']}|{x['scale_bits'] or 'null'}|{x['class']}\n" for x in r["classifications"]))
    except rp.RetainedPrecisionError as e:
        out.append(f"PY {f.name} FIRST {e.gate} {e.code} {e.detail} schema_violations={len(errs)}")
    try:
        rp.validate_retained_precision(c["source"], c["invocation"]); out.append(f"PY {f.name} public-API admitted (UNEXPECTED)")
    except rp.RetainedPrecisionError as e:
        out.append(f"PY {f.name} public-API refuses {e.gate} {e.code} (eligibility off)")
(D / "py_results.txt").write_text("\n".join(out) + "\n"); print("\n".join(out))
