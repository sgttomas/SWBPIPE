"""Probe: an unreferenced (orphan) Build appended to a must-pass base, rehashed, through PY."""
import json, sys
from copy import deepcopy
from pathlib import Path
P = Path(sys.argv[1]); sys.path.insert(0, str(P)); sys.path.insert(0, str(Path(__file__).parent))
from core.analysis_runs import retained_precision as rp
import importlib.util
spec = importlib.util.spec_from_file_location("census_lib", Path(__file__).parent / "census_lib.py")
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)
data = json.loads((P / "fixtures/results/retained_precision_cases.json").read_text())
for fx in data["cases"]:
    b = fx["source"]["retained_precision"]["body"]
    if not b["builds"]: continue
    builds = deepcopy(b["builds"]); extra = deepcopy(builds[-1]); extra["id"] = len(builds); builds.append(extra)
    src, inv = lib.apply_entry(fx, {"edits": [{"path": ["retained_precision", "body", "builds"], "op": "set", "value": builds}], "rehash": "all"})
    try: r = rp.validate_retained_precision(src, inv); out = "pass " + r["standing"]
    except rp.RetainedPrecisionError as e: out = f"{e.gate} {e.code}"
    print(fx["id"], "orphan build ->", out)
