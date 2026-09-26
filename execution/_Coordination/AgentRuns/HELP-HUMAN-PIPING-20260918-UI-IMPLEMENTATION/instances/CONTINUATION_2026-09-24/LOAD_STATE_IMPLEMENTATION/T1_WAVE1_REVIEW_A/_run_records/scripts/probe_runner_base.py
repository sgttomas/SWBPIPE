"""Dispatch-only Python probe runner for the base commit."""
import json, pathlib, sys
sys.path.insert(0, str(pathlib.Path(sys.argv[1]).resolve()))
from core.analysis_runs.compatibility import _source_contract
out = []
for p in sorted(pathlib.Path(sys.argv[2]).glob("*.json")):
    v = json.loads(p.read_text())
    try:
        _source_contract(v); r = "accept"
    except ValueError as e:
        r = str(e)
    out.append(json.dumps({"id": p.name, "dispatch": r}))
pathlib.Path(sys.argv[3]).write_text("\n".join(out) + "\n")
