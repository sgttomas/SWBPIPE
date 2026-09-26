"""Integration check: the READERS TASK's derived load-reference-1 carriers
validate against the SCHEMAS TASK's carrier schemas; the physics-1 branch
refuses them and the load-reference-1 branch refuses physics carriers.
Run from WORKING_ROOT with a requirements-dev interpreter."""
import hashlib, json, pathlib, sys
root = pathlib.Path(sys.argv[1])
sys.path.insert(0, str(root / "tests"))
from schema_validation import validate_instance
results_schema = json.loads((root / "schemas/results.v0.3.schema.yaml").read_text())
run_root = json.loads((root / "schemas/analysis_run.schema.json").read_text())
run_v03 = json.loads((root / "schemas/analysis_run.v0.3.schema.json").read_text())
report = {"schemas": {p: hashlib.sha256((root / p).read_bytes()).hexdigest() for p in [
    "schemas/results.v0.3.schema.yaml", "schemas/analysis_run.v0.3.schema.json", "schemas/analysis_run.schema.json",
    "schemas/stress_neutral_export.v0.3.schema.json", "schemas/load_reference_state.schema.json"]}, "documents": {}}
def check(schema, instance, label, expect_ok=True):
    try:
        validate_instance(schema, instance, instance_label=label)
        ok = True
    except AssertionError as error:
        ok, message = False, str(error)[:400]
    assert ok == expect_ok, (label, expect_ok, None if ok else message)
    return ok
for path in sorted(root.glob("fixtures/results/load_reference_*.document.json")):
    doc = json.loads(path.read_text())
    assert doc["source"]["producer"]["semantic_contract_id"] if "source" in doc else True
    check(results_schema, doc, path.name)
    # A physics-1 relabel of the same carrier must not validate.
    relabel = json.loads(json.dumps(doc).replace("openpipestress.result_semantics/0.3.0/load-reference-1", "openpipestress.result_semantics/0.3.0/physics-1"))
    check(results_schema, relabel, path.name + " relabelled physics-1", expect_ok=False)
    report["documents"][path.name] = hashlib.sha256(path.read_bytes()).hexdigest()
for path in sorted(root.glob("fixtures/results/load_reference_*.analysis_run.json")):
    record = json.loads(path.read_text())
    check(run_root, record, path.name + " (analysis_run root)")
    check(run_v03, record, path.name + " (analysis_run v0.3)")
    report["documents"][path.name] = hashlib.sha256(path.read_bytes()).hexdigest()
assert len(report["documents"]) == 8, report
print(json.dumps(report, indent=1))
