"""Which base-header defects reach G7 in the Python reader (hash-consistent edits)."""
import json, sys
from pathlib import Path
P = Path(sys.argv[1]); sys.path.insert(0, str(P)); sys.path.insert(0, str(P / "tests"))
from core.analysis_runs import retained_precision as rp
import test_retained_precision_contract as t
c = t.corpus(); cases = {f["id"]: f for f in c["cases"]}
nr = next(m for m in c["must_pass"] if m["id"] == "not_required_second_case_checks_passed")
probes = []
for base in ["ordinary_prepared_synthetic", "two_case_preparation_failure_synthetic"]:
    st = [x["status"] for x in cases[base]["source"]["retained_precision"]["body"]["cases"]]
    for i in range(len(st)):
        for field, val in [("accuracy_evidence", "estimated"), ("structural_status", "mechanism_detected"), ("model_matrix_fidelity", "reduced"), ("solve_quality", "bogus"), ("evidence_refs", [""]), ("extra", 1)]:
            probes.append((f"{base}:case{i}:{st[i]}:{field}", base, [], [{"path": ["numerical_quality", "cases", i, field], "op": "set", "value": val}]))
for field, val in [("accuracy_evidence", "estimated"), ("structural_status", "mechanism_detected"), ("model_matrix_fidelity", "reduced"), ("evidence_refs", [""])]:
    probes.append((f"not_required_entry:case1:{field}", nr["base"], nr["edits"], [{"path": ["numerical_quality", "cases", 1, field], "op": "set", "value": val}]))
for base in ["ordinary_prepared_synthetic"]:
    for name, path, val in [("quality_status", ["numerical_quality", "status"], "bogus"), ("quality_policy", ["numerical_quality", "integrity_policy"], "x"),
                            ("quality_extra", ["numerical_quality", "extra"], 1), ("limitations_empty", ["formulation_basis", "limitations"], []),
                            ("limitations_other", ["formulation_basis", "limitations"], ["x"]), ("contract_evidence_null", ["contract_evidence"], None),
                            ("source_block_recovery", ["source_block_recovery"], None), ("carrier_evidence", ["carrier_evidence"], {}),
                            ("producer_extra", ["producer", "extra"], 1), ("formulation_extra", ["formulation_basis", "extra"], 1)]:
        probes.append((f"{base}:{name}", base, [], [{"path": path, "op": "set", "value": val}]))
out = []
for label, base, pre, edits in probes:
    e = {"id": label, "base": base, "edits": pre + edits, "rehash": "all"}
    try:
        s, inv = t.apply_entry(cases[base], e)
        rp._validate_draft(s, inv); r = "pass"
    except rp.RetainedPrecisionError as x:
        r = f"{x.gate} {x.code}"
    except Exception as x:
        r = f"HARNESS {type(x).__name__} {x}"
    out.append({"label": label, "base": base, "edits": pre + edits, "python": r})
    print(label, "->", r)
json.dump(out, open(sys.argv[2], "w"), indent=1)
