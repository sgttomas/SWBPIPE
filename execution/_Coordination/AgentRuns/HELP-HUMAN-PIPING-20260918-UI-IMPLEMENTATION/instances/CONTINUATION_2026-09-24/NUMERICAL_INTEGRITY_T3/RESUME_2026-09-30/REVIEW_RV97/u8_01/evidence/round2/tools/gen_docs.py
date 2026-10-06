"""RV97 round 2: write every 07l corpus item as one document ({"label","source","invocation"}) using the
candidate's own Python entry semantics (tests/test_retained_precision_contract.py apply_entry), so that
all three readers see identical bytes. Also writes expectations.json. Usage: gen_docs.py P DOCS_DIR"""
import json, sys
from copy import deepcopy
from pathlib import Path
P, out = Path(sys.argv[1]), Path(sys.argv[2])
sys.path.insert(0, str(P)); sys.path.insert(0, str(P / "tests"))
import test_retained_precision_contract as t
c = t.corpus()
cases = {f["id"]: f for f in c["cases"]}
exp = []
def write(name, label, source, invocation, expected):
    (out / f"{name}.json").write_text(json.dumps({"label": label, "source": source, "invocation": invocation}))
    exp.append({"file": name, "label": label, **expected})
for i, f in enumerate(c["cases"]):
    write(f"c{i:02d}_bound", f"case:{f['id']}", deepcopy(f["source"]), deepcopy(f["invocation"]),
          {"kind": "case", "expected": f["expected"], "classifications_of": f["id"]})
    write(f"c{i:02d}_unbound", f"case:{f['id']}:no-invocation", deepcopy(f["source"]), None,
          {"kind": "case_unbound", "expected": t.NOT_ELIGIBLE, "classifications_of": f["id"]})
for i, m in enumerate(c["mutations"]):
    source, invocation = t.apply_entry(cases[m["base"]], m)
    write(f"m{i:03d}", f"mutation:{i}:{m['id']}", source, invocation,
          {"kind": "mutation", "expected": m["expected"], "expected_by_reader": m.get("expected_by_reader"), "base": m["base"]})
for i, m in enumerate(c["must_pass"]):
    source, invocation = t.apply_entry(cases[m["base"]], m)
    write(f"p{i:02d}", f"must_pass:{i}:{m['id']}", source, invocation,
          {"kind": "must_pass", "expected": m["expected_eligibility"], "classifications_of": m["base"], "base": m["base"]})
(out.parent / "expectations.json").write_text(json.dumps({"expectations": exp, "expected_classifications": {k: v["expected_classifications"] for k, v in cases.items()}}))
print("documents", len(exp))
