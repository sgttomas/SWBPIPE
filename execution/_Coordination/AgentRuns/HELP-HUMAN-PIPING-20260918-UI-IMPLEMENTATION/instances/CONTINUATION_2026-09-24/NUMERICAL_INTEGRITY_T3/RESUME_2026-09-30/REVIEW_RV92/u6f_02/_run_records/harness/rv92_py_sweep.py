"""RV92 (U6f): existing-behaviour sweep, Python carriers and schemas. Base-compatible.
argv: P sweep_dir out.jsonl"""
import copy, hashlib, json, sys

P, DIR, OUT = sys.argv[1:4]
sys.path.insert(0, P)
from core.analysis_runs import compatibility as c  # noqa: E402
from core.analysis_runs.records import build_preview_analysis_run_envelope  # noqa: E402
from tests.schema_validation import validate_instance, load_schema  # noqa: E402

MANIFEST = {"input_manifest_ref": {"object_type": "InputManifest", "ref": "manifest:rv92"}, "input_manifest_hash": "1" * 64}
from pathlib import Path  # noqa: E402
SCHEMAS = {"results_doc": load_schema(Path(f"{P}/schemas/results.v0.3.schema.yaml")),
           "analysis_run": json.load(open(f"{P}/schemas/analysis_run.v0.3.schema.json")),
           "stress_neutral": json.load(open(f"{P}/schemas/stress_neutral_export.v0.3.schema.json"))}
sha = lambda v: hashlib.sha256(json.dumps(v, sort_keys=True).encode()).hexdigest()


def r(fn, fmt=lambda v: ""):
    try:
        return "ok:" + fmt(fn())
    except Exception as e:  # noqa: BLE001
        return "err:" + (str(e) if isinstance(e, ValueError) else type(e).__name__ + ":" + str(e))[:240].replace("\n", " ")


def schema_check(kind, doc):
    if kind == "results_doc" and doc.get("schema_version") != "0.3.0":
        return "n/a"
    if kind == "analysis_run" and doc.get("schema_version") != "0.3.0":
        return "n/a"
    try:
        validate_instance(SCHEMAS[kind], doc, instance_label="rv92")
        return "valid"
    except AssertionError as e:
        return "invalid"
    except Exception as e:  # noqa: BLE001
        return "invalid:" + type(e).__name__


out = open(OUT, "w")
for entry in json.load(open(f"{DIR}/index.json")):
    p = json.load(open(f"{DIR}/{entry['file']}"))
    src, group = p["source"], entry["group"]
    o = {"id": p["id"], "group": group}
    if group in ("results_doc", "analysis_run", "stress_neutral"):
        o["schema"] = schema_check(group, src)
        if group == "analysis_run":
            o["verify"] = r(lambda: c.verify_analysis_run_record(src), lambda v: str(v))
        if group == "stress_neutral":
            try:
                from core.handoff.stress_neutral import package_v0_3 as sn
                o["sn_validate"] = r(lambda: sn.validate_stress_neutral_export_package_v0_3(src))
            except ImportError as e:
                o["sn_validate"] = "import:" + str(e)
        out.write(json.dumps(o, sort_keys=True) + "\n")
        continue
    req, inv = p["requested"], p["invocation"]
    pid = (src.get("producer") or {}).get("semantic_contract_id") if isinstance(src.get("producer"), dict) else None
    o["transport"] = r(lambda: c._source_contract(src, check_receipt=False), lambda v: v[0])
    o["raw"] = r(lambda: c._source_contract(src), lambda v: v[0])
    o["standing_nq"] = r(lambda: c.numerical_use_standing(src, req), str)
    o["standing_empty"] = r(lambda: c.numerical_use_standing(src, []), str)
    if inv is not None:
        o["standing_inv"] = r(lambda: c.numerical_use_standing(src, req, inv), str)
    o["fresh"] = c.is_fresh_contract_id(pid)
    o["standing_reason"] = r(lambda: c.standing_reason(src), str)
    rows = src.get("results") if isinstance(src.get("results"), list) else []
    o["binding"] = [r(lambda row=row: c.rule_binding_refusal(src, row), str) for row in rows]
    rec = {}
    def build():
        rec["v"] = c.build_analysis_run(copy.deepcopy(src), **MANIFEST)
        return rec["v"]
    o["ar_build"] = r(build, lambda v: v["schema_version"] + ":" + sha(v))
    if "v" in rec and rec["v"]["schema_version"] == "0.3.0":
        o["ar_validate"] = r(lambda: c.validate_analysis_run_v0_3(rec["v"], src))
        o["ar_schema"] = schema_check("analysis_run", rec["v"])
    o["ar_v02"] = r(lambda: c.build_analysis_run_v0_2(copy.deepcopy(src), **MANIFEST), lambda v: sha(v))
    o["ar_wrapper_010"] = r(lambda: build_preview_analysis_run_envelope(copy.deepcopy(src)), lambda v: sha(v))
    out.write(json.dumps(o, sort_keys=True) + "\n")
    out.flush()
