"""I75: schema negative controls and a status summary for a dumped successor package."""
import json, sys, collections, pathlib
P = pathlib.Path(sys.argv[1]); out = pathlib.Path(sys.argv[2])
sys.path.insert(0, str(P / "tests"))
from schema_validation import validate_instance
schema = json.loads((P / "schemas/stress_neutral_export.v0.3.schema.json").read_text())
for f in sorted(out.glob("sn_*.json")):
    p = json.loads(f.read_text())
    for label, edit in [("receipt removed", lambda q: q.pop("retained_precision")), ("annotations removed", lambda q: q.pop("source_annotations")), ("csv_encoding removed", lambda q: q["export_profile"].pop("csv_encoding")), ("source_block_recovery added", lambda q: q.__setitem__("source_block_recovery", {})), ("contract_evidence removed", lambda q: q.pop("contract_evidence"))]:
        q = json.loads(json.dumps(p)); edit(q)
        try: validate_instance(schema, q); print("UNEXPECTED-VALID", f.name, label)
        except AssertionError: print("invalid-as-expected", f.name, label)
    print("summary", f.name, "validation", p["validation_report"]["validation_status"], "ready", p["validation_ready"], "rows", len(p["result_rows"]), "witnesses", len(p["unit_preservation_witnesses"]), dict(collections.Counter(f'{d["code"]}/{d["severity"]}' for d in p["diagnostics"])))
