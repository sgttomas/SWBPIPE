"""RV91: independent Python-side run of the shared carrier cases plus RV91's
extra cases, against U6b (c89a7a986c) as archived under WT/rv91/u6b.
Writes one JSON object per case: {id, standing, dispatch}."""
import json, sys, hashlib
from copy import deepcopy
from pathlib import Path
ROOT = Path(sys.argv[1]); OUT = Path(sys.argv[2]); EXTRA = Path(sys.argv[3])
sys.path.insert(0, str(ROOT))
from core.analysis_runs import compatibility as c
cases = json.loads((ROOT / "fixtures/results/retained_precision_carrier_cases.json").read_text())
extra = json.loads(EXTRA.read_text())
fixtures = {}
for fid, spec in cases["fixtures"].items():
    raw = (ROOT / spec["path"]).read_bytes()
    assert hashlib.sha256(raw).hexdigest() == spec["sha256"], fid
    fixtures[fid] = json.loads(raw)
def apply(edit, source, invocation):
    target = source if edit["target"] == "source" else invocation
    path = edit["path"]
    for key in path[:-1]:
        target = target[key]
    if edit["op"] == "set": target[path[-1]] = edit["value"]
    elif edit["op"] == "delete": del target[path[-1]]
    else: raise SystemExit(edit)
out = []
for case in cases["cases"] + extra:
    doc = fixtures[case["fixture"]]
    source, invocation = deepcopy(doc["source"]), deepcopy(doc["invocation"])
    for edit in case["edits"]: apply(edit, source, invocation)
    refs = [{"ref_type": "load_case", "ref_id": lc["id"]} for lc in doc["invocation"]["request"]["model"]["load_cases"]] if case["requested"] == "invocation" else case["requested"]
    ctx = invocation if case["invocation"] is not None else None
    standing = c.numerical_use_standing(source, refs, ctx)
    try:
        c._source_contract(source); dispatch = "ok"
    except Exception as e:  # first error code
        dispatch = str(e)
    out.append({"id": case["id"], "standing": standing, "dispatch": dispatch,
                "expected_standing": case.get("expected_standing"), "expected_dispatch": case.get("expected_dispatch")})
OUT.write_text("\n".join(json.dumps(o) for o in out) + "\n")
for o in out: print(o)
