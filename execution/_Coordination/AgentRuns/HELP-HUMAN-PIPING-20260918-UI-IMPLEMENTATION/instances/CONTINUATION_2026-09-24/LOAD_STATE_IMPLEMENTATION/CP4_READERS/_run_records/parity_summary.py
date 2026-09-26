"""CP4_READERS: join the per-case outcome logs of both languages (run from WORKING_ROOT)."""
import json, sys
from pathlib import Path
RR = Path(sys.argv[1])
cases = json.loads(Path("core/reporting/result_export/tests/fixtures/load_reference_mutations.json").read_text())
py = {r["id"]: r for r in json.loads((RR / "parity/python_outcomes.json").read_text())}
rs = {r["id"]: r for r in json.loads((RR / "parity/rust_outcomes.json").read_text())}
rs.update({r["id"]: r for r in json.loads((RR / "parity/rust_table_transport_outcomes.json").read_text())})
new = [c["id"] for c in cases["cases"] + cases["transport_cases"] if c["id"].startswith(("N2-", "N3-", "ACCEPT-N3", "TRANSPORT-N2", "TRANSPORT-N3"))]
rows, counts = [], {"exact": 0, "language_specific": 0, "different": 0, "accept_vs_reject": 0}
for cid in list(py):
    p, r = py[cid], rs.get(cid)
    assert r is not None, cid
    for kind in ("dispatch", "validator", "table", "transport"):
        if kind not in p: continue
        a, b = p[kind], r[kind]
        if a == b: counts["exact"] += 1
        elif (a == "accept") != (b == "accept") and b != "JSON_PARSE_REJECTED": counts["accept_vs_reject"] += 1
        elif a.split(": ")[0] == b.split(": ")[0]: counts["exact"] += 0; counts.setdefault("same_leading_code", 0); counts["same_leading_code"] += 1
        else: counts["language_specific"] += 1
for cid in new:
    p, r = py[cid], rs[cid]
    rows.append({"id": cid, "python": {k: v for k, v in p.items() if k != "id"}, "rust": {k: v for k, v in r.items() if k != "id"}})
out = {"total_ids_python": len(py), "total_ids_rust": len(rs), "comparisons": counts, "new_cases": rows}
(RR / "parity_summary.json").write_text(json.dumps(out, indent=1) + "\n")
print(json.dumps(counts))
for row in rows: print(row["id"], row["python"], row["rust"])
