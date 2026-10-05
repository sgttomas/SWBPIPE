"""RV92 (U6f): the receipt's survival, Python. argv: P survival_dir step(a|b)"""
import copy, json, sys

P, DIR, STEP = sys.argv[1:4]
sys.path.insert(0, P)
from core.analysis_runs import compatibility as c  # noqa: E402
from core.analysis_runs import retained_precision as rp  # noqa: E402

MANIFEST = {"input_manifest_ref": {"object_type": "InputManifest", "ref": "manifest:rv92"}, "input_manifest_hash": "1" * 64}
log = open(f"{DIR}/py_step_{STEP}.jsonl", "a")
for mode in ("sparse_interactive", "dense_scrutiny"):
    doc = json.load(open(f"{P}/fixtures/results/retained_precision_milestone_successor_{mode}.json"))
    src, inv = doc["source"], doc["invocation"]
    req = [{"ref_type": "load_case", "ref_id": x["id"]} for x in inv["request"]["model"]["load_cases"]]
    original = json.dumps(src["retained_precision"], separators=(",", ":"))
    if STEP == "a":
        record = c.build_analysis_run(copy.deepcopy(src), **MANIFEST)
        c.validate_analysis_run_v0_3(record, src)
        json.dump(record, open(f"{DIR}/py_ar_{mode}.json", "w"))
        log.write(json.dumps({"mode": mode, "built": record["schema_version"], "verify": c.verify_analysis_run_record(record),
                              "receipt_byte_equal": json.dumps(record["analysis_run"]["retained_precision"], separators=(",", ":")) == original}) + "\n")
        continue
    base = rp.validate_retained_precision(src, inv)
    for carrier, path, keys in (("rust_derivative", f"{DIR}/rust_derivative_{mode}.json", ("result_envelope", "retained_precision")),
                                ("python_analysis_run", f"{DIR}/py_ar_{mode}.json", ("analysis_run", "retained_precision")),
                                ("ts_analysis_run", f"{DIR}/ts_ar_{mode}.json", ("analysis_run", "retained_precision"))):
        try:
            d = json.load(open(path))
        except FileNotFoundError:
            log.write(json.dumps({"mode": mode, "carrier": carrier, "missing": True}) + "\n")
            continue
        receipt = d[keys[0]].get(keys[1])
        back = copy.deepcopy(src)
        back["retained_precision"] = receipt
        try:
            v = rp.validate_retained_precision(back, inv)
            ok, err = v == base, None
        except rp.RetainedPrecisionError as e:
            ok, err = False, f"{e.gate}:{e.code}"
        log.write(json.dumps({"mode": mode, "carrier": carrier, "receipt_byte_equal": json.dumps(receipt, separators=(",", ":")) == original,
                              "revalidated": ok, "error": err, "standing": c.numerical_use_standing(back, req, inv),
                              "standing_no_invocation": c.numerical_use_standing(back, req)}) + "\n")
        if carrier == "python_analysis_run":
            try:
                c.validate_analysis_run_v0_3(d, back)
                log.write(json.dumps({"mode": mode, "carrier": "python_analysis_run_validate", "result": "ok"}) + "\n")
            except ValueError as e:
                log.write(json.dumps({"mode": mode, "carrier": "python_analysis_run_validate", "result": str(e)}) + "\n")
if STEP == "b":
    # Schema: the Rust derivative under results.v0.3, both AnalysisRun records under the dispatcher.
    from pathlib import Path
    from tests.schema_validation import validate_instance, load_schema
    results = load_schema(Path(f"{P}/schemas/results.v0.3.schema.yaml"))
    run_schema = json.load(open(f"{P}/schemas/analysis_run.schema.json"))
    for mode in ("sparse_interactive", "dense_scrutiny"):
        for carrier, path, schema in (("rust_derivative", f"{DIR}/rust_derivative_{mode}.json", results),
                                      ("python_analysis_run", f"{DIR}/py_ar_{mode}.json", run_schema),
                                      ("ts_analysis_run", f"{DIR}/ts_ar_{mode}.json", run_schema)):
            try:
                validate_instance(schema, json.load(open(path)), instance_label=carrier)
                verdict = "valid"
            except AssertionError as e:
                verdict = "invalid: " + str(e)[:300]
            except FileNotFoundError:
                verdict = "missing"
            log.write(json.dumps({"mode": mode, "carrier": carrier + "_schema", "result": verdict}) + "\n")
