"""RV90 probes 3: non-array affected_refs on a case-naming diagnostic (listed / unlisted), F5 parity."""
import sys, json, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from helpers import apply
P = sys.argv[1]; out = sys.argv[2]
c = json.load(open(f"{P}/fixtures/results/retained_precision_cases.json")); base = next(x for x in c["cases"] if x["id"] == "ordinary_prepared_synthetic")
ds = base["source"]["diagnostics"]; k = next(j for j, d in enumerate(ds) if d["code"] == "LOAD_CATEGORY_PREVIEW_MAPPED")
cid = "case:six-component-load"; exact = base["source"]["retained_precision"]["body"]["ordinary_attempts"][0]["diagnostic_refs"]
REFS = ["retained_precision", "body", "ordinary_attempts", 0, "diagnostic_refs"]
rows = []
for name, value in (("string", cid), ("number", 5), ("object", {cid: 1})):
    s, i = apply(base, {"edits": [{"path": ["diagnostics", k, "affected_refs"], "op": "set", "value": value}], "rehash": "all"})
    rows.append(("probe3", f"affected_refs_{name}_listed", s, i, None))
    s, i = apply(base, {"edits": [{"path": ["diagnostics", k, "affected_refs"], "op": "set", "value": value}, {"path": REFS, "op": "set", "value": [x for x in exact if x != ds[k]["id"]]}], "rehash": "all"})
    rows.append(("probe3", f"affected_refs_{name}_unlisted", s, i, None))
with open(out, "w") as f:
    for kind, id_, s, i, exp in rows:
        f.write(json.dumps({"kind": kind, "id": id_, "expected": exp, "source": s, "invocation": i}) + "\n")
print(len(rows))
# pre-existing family: the typed report diagnostic (integrity) with affected_refs as the case id string
rows2 = []
ki = next(j for j, d in enumerate(ds) if d["code"] == "NUMERICAL_INTEGRITY_SENSITIVE")
s, i = apply(base, {"edits": [{"path": ["diagnostics", ki, "affected_refs"], "op": "set", "value": cid}], "rehash": "all"})
rows2.append(("probe3", "typed_integrity_affected_refs_string", s, i, None))
with open(out, "a") as f:
    for kind, id_, s, i, exp in rows2:
        f.write(json.dumps({"kind": kind, "id": id_, "expected": exp, "source": s, "invocation": i}) + "\n")
print(len(rows) + len(rows2))
