"""Probe: each 07l entry through the Python reader, with the reader line that raised (records only).
Usage: probe_entries.py P_ROOT"""
import importlib.util, json, sys, traceback
from copy import deepcopy
from pathlib import Path
P = Path(sys.argv[1]); sys.path.insert(0, str(P)); sys.path.insert(0, str(Path(__file__).parent))
from core.analysis_runs import retained_precision as rp
import entries_07l as E
spec = importlib.util.spec_from_file_location("harness", P / "tests/test_retained_precision_contract.py")
h = importlib.util.module_from_spec(spec); spec.loader.exec_module(h)

def where(error):
    frames = [f for f in traceback.extract_tb(error.__traceback__) if f.filename.endswith("retained_precision.py") and f.name not in ("_need", "need", "<lambda>")]
    f = frames[-1]
    return f"{f.name}:{f.lineno}: {f.line.strip()[:110]}"

for mode in ("sparse_interactive", "dense_scrutiny"):
    doc = json.loads((P / f"fixtures/results/retained_precision_l0_successor_{mode}.json").read_text())
    base = {"id": doc["id"], "source": doc["source"], "invocation": doc["invocation"]}
    m, mp, xm, xmp = E.entries(doc["id"], mode, doc["source"])
    for entry in m + mp + xm + xmp:
        source, invocation = h.apply_entry(base, entry)
        try:
            v = rp._validate_draft(source, invocation)
            print(entry["id"], "PASS", {k: v[k] for k in ("invocation_bound", "numerical_eligible", "standing")},
                  "classes==base", v["classifications"] == rp._validate_draft(deepcopy(doc["source"]), deepcopy(doc["invocation"]))["classifications"])
        except rp.RetainedPrecisionError as e:
            print(entry["id"], "REFUSE", e.gate, e.code, "at", where(e))
