#!/usr/bin/env python3
"""I61 U6e: every corpus entry's Python outcome (pass with classes, or the first failure).
Usage: outcomes.py P_ROOT CORPUS OUT_JSON"""
import json, sys, importlib.util
from copy import deepcopy
from pathlib import Path
P, CORPUS, OUT = Path(sys.argv[1]), Path(sys.argv[2]), Path(sys.argv[3])
sys.path.insert(0, str(P))
from core.analysis_runs import retained_precision as rp
spec = importlib.util.spec_from_file_location("harness", P / "tests/test_retained_precision_contract.py")
h = importlib.util.module_from_spec(spec); spec.loader.exec_module(h)
data = json.loads(CORPUS.read_text())
def outcome(source, invocation):
    try:
        r = rp._validate_draft(source, invocation)
        return {"observed": "pass", "classes": r["classifications"]}
    except rp.RetainedPrecisionError as e:
        return {"observed": [e.gate, e.code]}
out = {"reader_sha256": __import__("hashlib").sha256((P / "core/analysis_runs/retained_precision.py").read_bytes()).hexdigest(),
       "corpus_sha256": __import__("hashlib").sha256(CORPUS.read_bytes()).hexdigest(), "cases": {}, "mutations": {}, "must_pass": {}}
for f in data["cases"]:
    o = outcome(deepcopy(f["source"]), deepcopy(f["invocation"]))
    out["cases"][f["id"]] = {"observed": o["observed"], "classifications_equal": o.get("classes") == f["expected_classifications"]}
for kind in ("mutations", "must_pass"):
    for e in data[kind]:
        fixture = next(f for f in data["cases"] if f["id"] == e["base"])
        source, invocation = h.apply_entry(fixture, e)
        o = outcome(source, invocation)
        exp = e.get("expected")
        if "expected_by_reader" in e: exp = e["expected_by_reader"]["python"]
        want = exp if isinstance(exp, str) else [exp["gate"], exp["code"]]
        out[kind][e["id"]] = {"observed": o["observed"], "expected": want, "match": o["observed"] == want}
OUT.write_text(json.dumps(out, indent=1) + "\n")
bad = [k for kind in ("mutations", "must_pass") for k, v in out[kind].items() if not v["match"]]
badc = [k for k, v in out["cases"].items() if v["observed"] != "pass" or not v["classifications_equal"]]
print(len(out["cases"]), len(out["mutations"]), len(out["must_pass"]), "mismatch:", bad, "cases:", badc)
