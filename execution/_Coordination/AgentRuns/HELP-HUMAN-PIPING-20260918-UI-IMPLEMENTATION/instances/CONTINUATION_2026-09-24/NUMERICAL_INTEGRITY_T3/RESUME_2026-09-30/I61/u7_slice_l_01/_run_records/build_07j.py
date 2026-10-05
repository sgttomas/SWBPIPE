#!/usr/bin/env python3
"""I61 U7 slice L (C04): stage snapshot 07j, one shared must-pass entry whose second case is
`not_required` and passes every gate.

Usage: build_07j.py P_ROOT OUT_CORPUS [--probe]
The entry is built on `two_case_preparation_failure_synthetic`, whose case 1 has no run and no
source (a preparation failure), by the contract's not_required form:
  - C1 WIRE_CONTRACT.md:101: a not_required case carries the common members only (no attempts,
    selected summaries, token or made-up zero-work solve); it means the ordinary pass;
  - the schema's `Case` not_required branch: {basis_ref, ordinary, product_attempt_ref: null,
    status: "not_required"};
  - D6b (C1:101; reader rule): a not_required case has no product attempt and its ordinary quality
    is `checks_passed`; its ordinary initial report states that outcome;
  - the preview-physics-1 base: a checks-passed case publishes an info
    NUMERICAL_INTEGRITY_CHECKS_PASSED diagnostic as its evidence (the base fixtures' form);
  - C1 G4/G6: only an unavailable case carries RETAINED_PRECISION_UNAVAILABLE, and only selected
    rows carry the W1 token (case 1's rows carry none already).
The expectation is the slice A oracle's rule R1/R2 (C1:160; D2 §4.9.4): eligible with its invocation,
and the carrier token numerically_eligible (the not_required case is ordinarily eligible).
"""
import json, sys
from copy import deepcopy
from pathlib import Path

P, OUT = Path(sys.argv[1]), Path(sys.argv[2])
PROBE = "--probe" in sys.argv
corpus_path = P / "fixtures/results/retained_precision_cases.json"
raw = corpus_path.read_bytes(); data = json.loads(raw); before = deepcopy(data)
assert json.dumps(data, indent=2).encode() + b"\n" == raw
assert (len(data["cases"]), len(data["mutations"]), len(data["must_pass"])) == (15, 277, 23), "07i"
BASE = "two_case_preparation_failure_synthetic"
base = next(c for c in data["cases"] if c["id"] == BASE)
s = base["source"]; b = s["retained_precision"]["body"]
case1 = b["cases"][1]
assert case1["status"] == "unavailable" and case1["run"] is None and case1["source_ref"] is None and case1["product_attempt_ref"] == 1
assert len(b["product_attempts"]) == 2 and b["product_attempts"][1]["owner_ref"] == {"kind": "case", "index": 1}
assert b["product_attempts"][1]["source_ref"] is None and b["product_attempts"][1]["run_ref"] is None
assert [x for x in b["work"]["execution_order"]] == [{"kind": "case", "index": 0}]
cid = case1["basis_ref"]["ref_id"]
ds = s["diagnostics"]
k_unavail = next(i for i, d in enumerate(ds) if d["code"] == "RETAINED_PRECISION_UNAVAILABLE")
assert ds[k_unavail]["id"] == case1["diagnostic_ref"] and ds[k_unavail]["affected_refs"] == [cid]
k_integrity = next(i for i, d in enumerate(ds) if d["id"] == f"diagnostic:numerical-integrity:{cid}")
assert ds[k_integrity]["code"] == "NUMERICAL_INTEGRITY_SENSITIVE"
assert s["numerical_quality"]["cases"][1]["evidence_refs"] == [ds[k_integrity]["id"]]
o1 = b["ordinary_attempts"][1]
assert o1["initial"] == {"kind": "report", "report_diagnostic_ref": ds[k_integrity]["id"], "outcome": "sensitive"}
assert k_unavail > k_integrity
R = ["retained_precision", "body"]
set_ = lambda path, value: {"path": path, "op": "set", "value": value}
edits = [
    set_(R + ["cases", 1], {"basis_ref": case1["basis_ref"], "ordinary": case1["ordinary"], "product_attempt_ref": None, "status": "not_required"}),
    {"path": R + ["product_attempts", 1], "op": "remove"},
    {"path": ["diagnostics", k_unavail], "op": "remove"},
    set_(["diagnostics", k_integrity, "code"], "NUMERICAL_INTEGRITY_CHECKS_PASSED"),
    set_(["diagnostics", k_integrity, "severity"], "info"),
    set_(["numerical_quality", "cases", 1, "solve_quality"], "checks_passed"),
    set_(R + ["ordinary_attempts", 1, "initial", "outcome"], "checks_passed"),
]
entry = {"id": "not_required_second_case_checks_passed", "base": BASE, "edits": edits, "rehash": "all", "expected": "pass",
         "expected_eligibility": {"invocation_bound": True, "numerical_eligible": True, "standing": "eligible"}}
if PROBE:
    # The reader is the test of "passes every gate"; the expectation above is the oracle's.
    sys.path.insert(0, str(P))
    import importlib.util
    spec = importlib.util.spec_from_file_location("harness", P / "tests/test_retained_precision_contract.py")
    h = importlib.util.module_from_spec(spec); spec.loader.exec_module(h)
    from core.analysis_runs import retained_precision as rp
    from core.analysis_runs import compatibility as c
    src, inv = h.apply_entry(base, entry)
    try:
        v = rp.validate_retained_precision(src, inv)
        req = [{"ref_type": "load_case", "ref_id": x["id"]} for x in inv["request"]["model"]["load_cases"]]
        print("PASS", v["numerical_eligible"], v["standing"], "classes", len(v["classifications"]) == len(base["expected_classifications"]),
              v["classifications"] == base["expected_classifications"], "token", c.numerical_use_standing(src, req, inv))
    except rp.RetainedPrecisionError as e:
        print("REFUSED", e.gate, e.code, getattr(e, "detail", None))
    sys.exit(0)
data["must_pass"].append(entry)
for key in data:
    if key != "must_pass":
        assert data[key] == before[key], key
assert data["must_pass"][:23] == before["must_pass"]
OUT.write_text(json.dumps(data, indent=2) + "\n")
print("staged", OUT, len(data["cases"]), len(data["mutations"]), len(data["must_pass"]))
