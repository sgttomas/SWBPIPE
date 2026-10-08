"""RV108 Python probe runner: each entry point over each probe; outputs JSONL."""
import json, sys, hashlib
from copy import deepcopy
from pathlib import Path
PROOT = Path(sys.argv[1]); IN = Path(sys.argv[2]); OUT = Path(sys.argv[3])
sys.path.insert(0, str(PROOT))
from core.analysis_runs import retained_precision as rp
from core.analysis_runs import compatibility as c

def canon(x): return hashlib.sha256(json.dumps(x, sort_keys=True, separators=(",", ":")).encode()).hexdigest()
def run(fn):
    try:
        r = fn()
        if isinstance(r, tuple): r = [r[0], r[1], Path(r[2]).name]
        return {"ok": True, "sha": canon(r), "eligible": r.get("numerical_eligible") if isinstance(r, dict) else None}
    except rp.RetainedPrecisionError as e:
        return {"gate": e.gate, "code": e.code, "detail": e.detail}
    except ValueError as e:
        return {"error": str(e)}
    except Exception as e:
        return {"exception": type(e).__name__, "msg": str(e)[:200]}
probes = json.loads(IN.read_text())
has_t = hasattr(rp, "validate_retained_precision_transport")
with OUT.open("w") as f:
    for p in probes:
        s, inv = p["source"], p["invocation"]
        row = {"id": p["id"], "family": p.get("family")}
        row["raw_inv"] = run(lambda: rp.validate_retained_precision(deepcopy(s), deepcopy(inv)))
        row["raw_none"] = run(lambda: rp.validate_retained_precision(deepcopy(s)))
        row["transport"] = run(lambda: rp.validate_retained_precision_transport(deepcopy(s))) if has_t else {"absent": True}
        row["c_raw"] = run(lambda: c._source_contract(deepcopy(s)))
        row["c_transport"] = run(lambda: c._source_contract(deepcopy(s), check_receipt=False))
        f.write(json.dumps(row) + "\n")
print("done", len(probes))
