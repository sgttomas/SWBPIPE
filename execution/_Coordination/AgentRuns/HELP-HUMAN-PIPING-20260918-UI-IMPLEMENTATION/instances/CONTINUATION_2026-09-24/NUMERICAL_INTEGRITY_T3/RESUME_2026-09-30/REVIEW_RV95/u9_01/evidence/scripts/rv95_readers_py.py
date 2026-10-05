"""RV95: the Python reader on the live milestone successors, with and without the invocation,
plus a hostile mode-swapped invocation. Run from P with the repository venv."""
import json, sys, glob, os, hashlib, collections
sys.path.insert(0, os.getcwd())
from core.analysis_runs.retained_precision import validate_retained_precision
out = []
for path in sorted(glob.glob(os.path.join(sys.argv[1], "*.json"))):
    doc = json.load(open(path))
    src, inv = doc["source"], doc["invocation"]
    other = dict(inv, solver_mode="dense_scrutiny" if inv["solver_mode"] == "sparse_interactive" else "sparse_interactive")
    for label, arg in (("with_invocation", inv), ("without_invocation", None), ("mode_swapped", other)):
        try:
            r = validate_retained_precision(src, arg)
            classes = collections.Counter(c if isinstance(c, str) else json.dumps(c, sort_keys=True) for c in [x.get("class") if isinstance(x, dict) else x for x in r["classifications"]])
            out.append({"file": os.path.basename(path), "case": label, "ok": True, "invocation_bound": r["invocation_bound"],
                        "numerical_eligible": r["numerical_eligible"], "standing": r["standing"], "publication_sha256": r["publication_sha256"],
                        "n_classifications": len(r["classifications"])})
        except Exception as e:
            out.append({"file": os.path.basename(path), "case": label, "ok": False, "gate": getattr(e, "gate", None), "code": getattr(e, "code", str(e)[:80])})
json.dump(out, open(sys.argv[2], "w"), indent=1)
for o in out: print(o)
