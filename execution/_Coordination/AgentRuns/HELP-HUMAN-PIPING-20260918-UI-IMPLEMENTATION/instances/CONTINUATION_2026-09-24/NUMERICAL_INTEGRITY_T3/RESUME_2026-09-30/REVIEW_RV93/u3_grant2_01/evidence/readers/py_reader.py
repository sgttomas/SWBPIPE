#!/usr/bin/env python3
"""RV93: the candidate's own Python F2a reader on the successor bytes the actual Direct entry
published (registered build). Draft entry (eligibility off) plus the public entry, and the schema."""
import json, os, sys
from pathlib import Path
P = Path(sys.argv[1]); D = Path(sys.argv[2])
sys.path.insert(0, str(P))
from core.analysis_runs import retained_precision as rp
import jsonschema
schema = json.loads((P / "schemas/retained_precision_mp_v2.schema.json").read_text())
val = jsonschema.Draft202012Validator(schema)
print("module", Path(rp.__file__).resolve().relative_to(P.resolve()), "IMPLEMENTATION_COMPLETE", rp._IMPLEMENTATION_COMPLETE)
for f in sorted(D.glob("u1_milestone_*.json")):
    c = json.loads(f.read_text())
    errs = list(val.iter_errors(c["source"]["retained_precision"]))
    try:
        r = rp._validate_draft(c["source"], c["invocation"])
        from collections import Counter
        print(f"PY {f.name} PASS standing={r['standing']} eligible={r['numerical_eligible']} invocation_bound={r['invocation_bound']} classes={len(r['classifications'])} by_class={dict(sorted(Counter(x['class'] for x in r['classifications']).items()))} schema_violations={len(errs)}")
    except rp.RetainedPrecisionError as e:
        print(f"PY {f.name} FIRST {e.gate} {e.code} {e.detail} schema_violations={len(errs)}")
    try:
        rp.validate_retained_precision(c["source"], c["invocation"]); print(f"PY {f.name} public entry admitted")
    except rp.RetainedPrecisionError as e:
        print(f"PY {f.name} public entry refuses {e.gate} {e.code} (eligibility off on this branch)")
    # negative control: one flipped receipt hash byte must be refused
    bad = json.loads(f.read_text()); h = bad["source"]["retained_precision"]["receipt_sha256"]
    bad["source"]["retained_precision"]["receipt_sha256"] = ("0" if h[0] != "0" else "1") + h[1:]
    try:
        rp._validate_draft(bad["source"], bad["invocation"]); print(f"PY {f.name} NEGATIVE CONTROL ADMITTED (unexpected)")
    except rp.RetainedPrecisionError as e:
        print(f"PY {f.name} negative control refused {e.gate} {e.code}")
