"""Join the Rust and Python per-case outcome logs of the load-reference-source-1
shared cases. Usage: python parity_summary.py <parity_dir> <out.json>. Run from WORKING_ROOT."""
import json
import sys
from pathlib import Path

folder, out = Path(sys.argv[1]), Path(sys.argv[2])
cases = json.loads(Path("core/reporting/result_export/tests/fixtures/load_reference_source_mutations.json").read_text())
declared = {c["id"] for c in cases["cases"] if any(k.endswith(("_rust", "_python")) for k in c)}
inherited = "SOURCE_LOAD_REFERENCE_JOIN_PHYSICS_SOURCE"
rust = {}
for name in ("rust_outcomes.json", "rust_table_transport_outcomes.json"):
    for entry in json.loads((folder / name).read_text()):
        rust.setdefault(entry["id"], {}).update({k: v for k, v in entry.items() if k != "id"})
python = {}
for entry in json.loads((folder / "python_outcomes.json").read_text()):
    python.setdefault(entry["id"], {}).update({k: v for k, v in entry.items() if k != "id"})
assert set(rust) == set(python), set(rust) ^ set(python)
rows, counts = [], {"exact": 0, "same_inherited_leading_code": 0, "declared_language_specific": 0, "undeclared": 0}
split = []
for cid in sorted(rust):
    for kind in sorted(set(rust[cid]) | set(python[cid])):
        r, p = rust[cid].get(kind), python[cid].get(kind)
        if r is None or p is None:
            continue
        if r == p:
            cls = "exact"
        elif r.split(": ", 1)[0] == p.split(": ", 1)[0] == inherited:
            cls = "same_inherited_leading_code"
        elif cid in declared:
            cls = "declared_language_specific"
        else:
            cls = "undeclared"
        counts[cls] += 1
        if (r == "accept") != (p == "accept"):
            split.append([cid, kind, r, p])
        rows.append({"id": cid, "kind": kind, "rust": r, "python": p, "class": cls})
summary = {"case_ids": len(rust), "comparisons": len(rows), **counts, "accepted_by_one_refused_by_other": split,
           "accepted_ids": sorted({r["id"] for r in rows if r["rust"] == "accept" and r["kind"] in ("dispatch", "transport", "table")}), "rows": rows}
out.write_text(json.dumps(summary, indent=1) + "\n")
print(json.dumps({k: v for k, v in summary.items() if k not in ("rows", "accepted_ids")}))
