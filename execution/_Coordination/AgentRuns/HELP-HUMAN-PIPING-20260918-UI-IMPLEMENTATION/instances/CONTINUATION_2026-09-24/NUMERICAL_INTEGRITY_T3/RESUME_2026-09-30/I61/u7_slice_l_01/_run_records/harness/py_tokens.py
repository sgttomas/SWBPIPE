#!/usr/bin/env python3
"""I61 U7 slice L: Python standing tokens on the live successors (the PP Direct entry's bytes).
argv: P_ROOT LIVE_DIR OUT_JSON. Reader: validate_retained_precision; carriers: numerical_use_standing,
classification_summary. Forms: with the invocation; without it; D-U7-4 (a) the invocation argument with no
native capture (Python reads the argument as for 'fixture'); D-U7-4 (b) a stale current model (Python has
no current-model input beyond the requested refs, which stay the invocation's)."""
import copy, json, sys
P, LIVE, OUT = sys.argv[1:4]
sys.path.insert(0, P)
from core.analysis_runs import compatibility as c
from core.analysis_runs import retained_precision as rp
out = {}
for mode in ("sparse_interactive", "dense_scrutiny"):
    doc = json.load(open(f"{LIVE}/u3g2_successor_{mode}.json"))
    src, inv = doc["source"], doc["invocation"]
    req = [{"ref_type": "load_case", "ref_id": x["id"]} for x in inv["request"]["model"]["load_cases"]]
    v = rp.validate_retained_precision(copy.deepcopy(src), copy.deepcopy(inv))
    v0 = rp.validate_retained_precision(copy.deepcopy(src), None)
    withheld = lambda x: [s["withheld"] for s in c.classification_summary(copy.deepcopy(src), x)]
    out[mode] = {
        "reader": {"with_invocation": {"numerical_eligible": v["numerical_eligible"], "standing": v["standing"]},
                   "without_invocation": {"numerical_eligible": v0["numerical_eligible"], "standing": v0["standing"]}},
        "token": {"with_invocation": c.numerical_use_standing(copy.deepcopy(src), req, copy.deepcopy(inv)),
                  "without_invocation": c.numerical_use_standing(copy.deepcopy(src), req),
                  "d_u7_4_no_native_capture": c.numerical_use_standing(copy.deepcopy(src), req, copy.deepcopy(inv)),
                  "d_u7_4_stale_current_model": c.numerical_use_standing(copy.deepcopy(src), req, copy.deepcopy(inv))},
        "withheld": {"with_invocation": withheld(copy.deepcopy(inv)), "without_invocation": withheld(None)},
    }
json.dump(out, open(OUT, "w"), indent=1, sort_keys=True)
print(json.dumps({m: x["token"] for m, x in out.items()}))
