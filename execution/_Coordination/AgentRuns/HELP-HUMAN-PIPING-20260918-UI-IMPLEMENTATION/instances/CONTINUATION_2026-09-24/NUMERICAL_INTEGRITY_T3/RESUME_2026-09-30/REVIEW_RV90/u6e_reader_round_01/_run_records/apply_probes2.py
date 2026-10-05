"""RV90 probes 2: a non-typed case diagnostic after the typed one in envelope order (strict-prefix list)."""
import sys, json, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from helpers import apply
P = sys.argv[1]; out = sys.argv[2]
c = json.load(open(f"{P}/fixtures/results/retained_precision_cases.json")); base = next(x for x in c["cases"] if x["id"] == "ordinary_prepared_synthetic")
ds = base["source"]["diagnostics"]; k = next(j for j, d in enumerate(ds) if d["code"] == "LOAD_CATEGORY_PREVIEW_MAPPED")
moved = ds[:k] + ds[k+1:] + [ds[k]]
cid = "case:six-component-load"
exact = [d["id"] for d in moved if isinstance(d.get("affected_refs"), list) and cid in d["affected_refs"] and not d["code"].startswith("RETAINED_PRECISION_")]
REFS = ["retained_precision", "body", "ordinary_attempts", 0, "diagnostic_refs"]
rows = []
s, i = apply(base, {"edits": [{"path": ["diagnostics"], "op": "set", "value": moved}, {"path": REFS, "op": "set", "value": exact}], "rehash": "all"})
rows.append(("probe2", "envelope_reordered_exact", s, i, "pass"))
s, i = apply(base, {"edits": [{"path": ["diagnostics"], "op": "set", "value": moved}, {"path": REFS, "op": "set", "value": exact[:-1]}], "rehash": "all"})
rows.append(("probe2", "envelope_reordered_strict_prefix", s, i, ["G5", "RETAINED_PRECISION_ATTEMPT_MISMATCH"]))
with open(out, "w") as f:
    for kind, id_, s, i, exp in rows:
        f.write(json.dumps({"kind": kind, "id": id_, "expected": exp, "source": s, "invocation": i}) + "\n")
print("exact", exact)
