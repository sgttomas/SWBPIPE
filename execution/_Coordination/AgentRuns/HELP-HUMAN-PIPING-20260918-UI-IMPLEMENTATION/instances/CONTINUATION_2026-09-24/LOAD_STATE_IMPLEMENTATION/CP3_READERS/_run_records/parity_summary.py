"""Join the Rust and Python per-case outcome logs with the shared case file.

Usage: python parity_summary.py <WORKING_ROOT> <parity_dir> <out.json>
"""
import json
import sys
from pathlib import Path

root, folder, out = Path(sys.argv[1]), Path(sys.argv[2]), Path(sys.argv[3])
cases = json.loads((root / "core/reporting/result_export/tests/fixtures/load_reference_mutations.json").read_text())
rust = {r["id"]: r for r in json.loads((folder / "rust_outcomes.json").read_text()) + json.loads((folder / "rust_table_transport_outcomes.json").read_text())}
python = {r["id"]: r for r in json.loads((folder / "python_outcomes.json").read_text())}
assert set(rust) == set(python), sorted(set(rust) ^ set(python))


def lead(text):
    return text.split(": ", 1)[0]


rows, counts = [], {"exact": 0, "inherited_leading_code": 0, "declared_language_specific": 0, "undeclared_difference": 0}
for group in ("cases", "table_cases", "transport_cases"):
    for case in cases[group]:
        r, p = rust[case["id"]], python[case["id"]]
        for field in ("dispatch", "validator", "table", "transport"):
            if field not in r:
                continue
            if r[field] == p[field]:
                kind = "exact"
            elif lead(r[field]) == lead(p[field]) == "SOURCE_LOAD_REFERENCE_PHYSICS_EVIDENCE":
                kind = "inherited_leading_code"
            elif any(key in case for key in (f"{field}_rust", f"{field}_python")):
                kind = "declared_language_specific"
            else:
                kind = "undeclared_difference"
            counts[kind] += 1
            rows.append({"id": case["id"], "group": group, "field": field, "rust": r[field], "python": p[field], "parity": kind})
accepted = sorted({row["id"] for row in rows if row["field"] in ("dispatch", "table", "transport") and row["rust"] == "accept" and row["python"] == "accept"})
rejected = sorted({row["id"] for row in rows if row["field"] in ("dispatch", "table", "transport") and row["rust"] != "accept" and row["python"] != "accept"})
split = sorted({row["id"] for row in rows if row["field"] in ("dispatch", "table", "transport") and (row["rust"] == "accept") != (row["python"] == "accept")})
out.write_text(json.dumps({"counts": counts, "accepted": accepted, "rejected": rejected, "accept_reject_split": split, "rows": rows}, indent=1) + "\n")
print(json.dumps({"counts": counts, "accepted": len(accepted), "rejected": len(rejected), "accept_reject_split": split}))
