"""RV92: is the Python AnalysisRun receipt-copy check exact? argv: P out.json"""
import copy, json, sys
P, OUT = sys.argv[1:3]
sys.path.insert(0, P)
from core.analysis_runs import compatibility as c
from core.analysis_runs import retained_precision as rp
from core.serialization.canonical_json.adapter import canonical_sha256_checked_v1
MANIFEST = {"input_manifest_ref": {"object_type": "InputManifest", "ref": "manifest:rv92"}, "input_manifest_hash": "1" * 64}
res = {}
for mode in ("sparse_interactive", "dense_scrutiny"):
    doc = json.load(open(f"{P}/fixtures/results/retained_precision_milestone_successor_{mode}.json"))
    src, inv = doc["source"], doc["invocation"]
    record = c.build_analysis_run(copy.deepcopy(src), **MANIFEST)
    def reseal(rec):
        h = [x for x in rec["analysis_run"]["hashes"] if x["payload_scope"] == "analysis_run_record"][0]
        h["value"] = canonical_sha256_checked_v1(c.analysis_record_projection(rec))
    for name, path, value in (("int0_to_false", ["body", "builds", 0, "id"], False), ("int1_to_true", None, True), ("int_to_float", ["body", "builds", 0, "id"], 0.0), ("int1_to_float", ["body", "builds", 1, "id"], 1.0), ("int0_to_null", ["body", "builds", 0, "id"], None), ("int0_to_string", ["body", "builds", 0, "id"], "0")):
        m = copy.deepcopy(record)
        rec = m["analysis_run"]["retained_precision"]
        if path is None:
            # first integer 1 in the receipt
            def find(x, p):
                if isinstance(x, dict):
                    for k, v in x.items():
                        r = find(v, p + [k])
                        if r: return r
                elif isinstance(x, list):
                    for i, v in enumerate(x):
                        r = find(v, p + [i])
                        if r: return r
                elif type(x) is int and x == 1:
                    return p
            path = find(rec, [])
        t = rec
        for k in path[:-1]: t = t[k]
        t[path[-1]] = value
        out = {"path": path}
        try:
            c.validate_analysis_run_v0_3(m, src); out["unsealed"] = "ok"
        except ValueError as e: out["unsealed"] = str(e)
        try:
            reseal(m); c.validate_analysis_run_v0_3(m, src); out["resealed"] = "ok"
        except ValueError as e: out["resealed"] = str(e)
        back = copy.deepcopy(src); back["retained_precision"] = m["analysis_run"]["retained_precision"]
        try:
            rp.validate_retained_precision(back, inv); out["copy_revalidates"] = True
        except rp.RetainedPrecisionError as e: out["copy_revalidates"] = f"{e.gate}:{e.code}"
        res[f"{mode}:{name}"] = out
    # Pre-existing pattern check: the same == on source_block_recovery (source-blocks-1).
json.dump(res, open(OUT, "w"), indent=1)
print(json.dumps(res, indent=1))
