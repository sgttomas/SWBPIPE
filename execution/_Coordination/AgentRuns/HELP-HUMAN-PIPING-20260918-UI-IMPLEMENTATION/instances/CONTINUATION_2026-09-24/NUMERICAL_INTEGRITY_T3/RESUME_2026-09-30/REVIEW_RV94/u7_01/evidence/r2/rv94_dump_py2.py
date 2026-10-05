"""RV94 Python dumper (scratch only). Usage: python rv94_dump_py.py <P root> <inputs.jsonl> <out.jsonl>"""
import hashlib
import json
import sys
from copy import deepcopy
from pathlib import Path

P = Path(sys.argv[1]).resolve()
sys.path.insert(0, str(P))
from core.analysis_runs import retained_precision as rp  # noqa: E402
from core.analysis_runs import compatibility as cp  # noqa: E402

out = []
for line in open(sys.argv[2]):
    r = json.loads(line)
    src, inv, req = r["source"], r["invocation"], r["requested"]
    rec = {"id": r["id"]}
    try:
        v = rp.validate_retained_precision(deepcopy(src), deepcopy(inv))
        counts = {}
        for c in v["classifications"]:
            counts[c["class"]] = counts.get(c["class"], 0) + 1
        rec["reader"] = {"ok": True, "invocation_bound": v["invocation_bound"], "numerical_eligible": v["numerical_eligible"],
                         "standing": v["standing"], "publication_sha256": v["publication_sha256"], "class_counts": counts,
                         "classes_sha256": hashlib.sha256(json.dumps(v["classifications"], sort_keys=True).encode()).hexdigest()}
    except rp.RetainedPrecisionError as e:
        rec["reader"] = {"ok": False, "gate": e.gate, "code": e.code}
    try:
        rec["token"] = cp.numerical_use_standing(deepcopy(src), deepcopy(req), deepcopy(inv))
    except Exception as e:  # a carrier must not raise
        rec["token"] = f"RAISED:{type(e).__name__}"
    try:
        import inspect
        if "requested_basis_refs" in inspect.signature(cp.classification_summary).parameters:
            rec["summary"] = cp.classification_summary(deepcopy(src), deepcopy(inv), deepcopy(req))
            rec["summary_norefs"] = cp.classification_summary(deepcopy(src), deepcopy(inv))
        else:
            rec["summary"] = cp.classification_summary(deepcopy(src), deepcopy(inv))
    except Exception as e:
        rec["summary"] = f"RAISED:{type(e).__name__}"
    out.append(rec)
Path(sys.argv[3]).write_text("".join(json.dumps(x, sort_keys=True) + "\n" for x in out))
print(len(out), "dumped")
