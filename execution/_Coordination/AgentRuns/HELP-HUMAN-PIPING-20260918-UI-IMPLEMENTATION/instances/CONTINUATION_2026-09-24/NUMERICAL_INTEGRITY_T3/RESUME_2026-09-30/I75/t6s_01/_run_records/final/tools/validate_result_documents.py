"""I75: validate dumped TS successor result documents under results.v0.3 and the dispatcher (the T6S branch's schemas), with negative controls."""
import json, sys, pathlib
P = pathlib.Path(sys.argv[1]); out = pathlib.Path(sys.argv[2])
sys.path.insert(0, str(P / "tests"))
from schema_validation import validate_instance
for name in ["results.v0.3.schema.yaml", "results.schema.yaml"]:
    schema = json.loads((P / "schemas" / name).read_text())
    for f in sorted(out.glob("rd_*.json")):
        doc = json.loads(f.read_text())
        try: validate_instance(schema, doc, schema_label=name, instance_label=f.name); print("VALID", name, f.name)
        except AssertionError as e: print("INVALID", name, f.name, str(e)[:1500])
schema = json.loads((P / "schemas/results.v0.3.schema.yaml").read_text())
for f in sorted(out.glob("rd_*.json")):
    doc = json.loads(f.read_text())
    for label, edit in [("receipt removed", lambda d: d["result_envelope"].pop("retained_precision")), ("class code outside the successor set", lambda d: next(x for x in d["result_envelope"]["row_disclosures"] if x["reason_code"] == "retained_precision_absolute_verified").__setitem__("reason_code", "retained_precision_invented"))]:
        d = json.loads(json.dumps(doc)); edit(d)
        try: validate_instance(schema, d); print("UNEXPECTED-VALID", f.name, label)
        except AssertionError: print("invalid-as-expected", f.name, label)
