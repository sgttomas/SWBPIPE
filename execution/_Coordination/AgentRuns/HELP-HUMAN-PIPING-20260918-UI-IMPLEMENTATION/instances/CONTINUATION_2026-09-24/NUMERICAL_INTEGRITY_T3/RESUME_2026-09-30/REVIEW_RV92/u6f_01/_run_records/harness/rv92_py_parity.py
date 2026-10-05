"""RV92 (U6f): the Python column of the parity table. argv: P probes_dir out.jsonl"""
import copy, json, sys, time

P, DIR, OUT = sys.argv[1:4]
sys.path.insert(0, P)
from core.analysis_runs import compatibility as c  # noqa: E402
from core.analysis_runs import retained_precision as rp  # noqa: E402
from core.analysis_runs.records import build_preview_analysis_run_envelope  # noqa: E402

MANIFEST = {"input_manifest_ref": {"object_type": "InputManifest", "ref": "manifest:rv92"}, "input_manifest_hash": "1" * 64}
ERRS = (ValueError, KeyError, TypeError, AttributeError, IndexError)


def r(fn, fmt=lambda v: ""):
    try:
        return "ok:" + fmt(fn())
    except ERRS as e:
        return "err:" + (str(e) if isinstance(e, ValueError) else type(e).__name__ + ":" + str(e)).replace("\t", " ").replace("\n", " ")
    except rp.RetainedPrecisionError as e:
        return "err:RPE:" + e.code
    except Exception as e:  # noqa: BLE001
        return "err:" + type(e).__name__ + ":" + str(e)[:200]


out = open(OUT, "w")
for entry in json.load(open(f"{DIR}/index.json")):
    p = json.load(open(f"{DIR}/{entry['file']}"))
    src, inv, req = p["source"], p["invocation"], p["requested"]
    pid = (src.get("producer") or {}).get("semantic_contract_id") if isinstance(src.get("producer"), dict) else None
    o = {"id": p["id"]}
    o["transport"] = r(lambda: c._source_contract(src, check_receipt=False), lambda v: v[0])
    o["raw"] = r(lambda: c._source_contract(src), lambda v: v[0])
    try:
        o["standing"] = c.numerical_use_standing(src, req, inv)
    except ERRS as e:
        o["standing"] = "raise:" + type(e).__name__
    o["fresh"] = c.is_fresh_contract_id(pid)
    o["standing_reason"] = r(lambda: c.standing_reason(src), lambda v: str(v))
    rows = src.get("results") if isinstance(src.get("results"), list) else []
    t0 = time.time()
    o["binding"] = [c.rule_binding_refusal(src, row) for row in rows]
    o["binding_ms"] = int((time.time() - t0) * 1000)
    head = []
    for k in ("max_displacement", "max_open_formula_stress"):
        ref = ((src.get("summary") or {}).get(k) or {}).get("result_ref")
        row = next((x for x in rows if x.get("id") == ref), None)
        head.append(c.rule_binding_refusal(src, row) if row is not None else "absent")
    o["headline_binding"] = head
    o["summary"] = c.classification_summary(src, inv)
    # AnalysisRun: the router, then validate; then receipt mutations on the record.
    rec = {}
    try:
        record = c.build_analysis_run(copy.deepcopy(src), **MANIFEST)
        o["ar_build"] = "ok:" + record["schema_version"]
    except ERRS as e:
        record = None
        o["ar_build"] = "err:" + str(e)
    except rp.RetainedPrecisionError as e:
        record = None
        o["ar_build"] = "err:RPE:" + e.code
    except Exception as e:  # noqa: BLE001
        record = None
        o["ar_build"] = "err:" + type(e).__name__ + ":" + str(e)[:200]
    if record is not None and record["schema_version"] == "0.3.0":
        run = record["analysis_run"]
        o["ar_validate"] = r(lambda: c.validate_analysis_run_v0_3(record, src))
        o["ar_verify"] = c.verify_analysis_run_record(record)
        o["ar_receipt_equal"] = (json.dumps(run.get("retained_precision"), sort_keys=True) == json.dumps(src.get("retained_precision"), sort_keys=True)) if "retained_precision" in run else None
        if "retained_precision" in run:
            for name, fn in (("dropped", lambda x: x["analysis_run"].pop("retained_precision")),
                             ("receipt_sha_zero", lambda x: x["analysis_run"]["retained_precision"].__setitem__("receipt_sha256", "0" * 64)),
                             ("body_charged_plus1", lambda x: x["analysis_run"]["retained_precision"]["body"]["work"].__setitem__("charged", x["analysis_run"]["retained_precision"]["body"]["work"]["charged"] + 1)),
                             ("charged_as_float", lambda x: x["analysis_run"]["retained_precision"]["body"]["work"].__setitem__("charged", float(x["analysis_run"]["retained_precision"]["body"]["work"]["charged"]))),
                             ("null", lambda x: x["analysis_run"].__setitem__("retained_precision", None))):
                m = copy.deepcopy(record)
                fn(m)
                rec[name] = r(lambda: c.validate_analysis_run_v0_3(m, src))
        else:
            m = copy.deepcopy(record)
            m["analysis_run"]["retained_precision"] = {"receipt_sha256": "0" * 64}
            rec["seeded_other"] = r(lambda: c.validate_analysis_run_v0_3(m, src))
        # Reopen: the record's copy authenticates the raw publication again.
        if "retained_precision" in run:
            back = copy.deepcopy(src)
            back["retained_precision"] = copy.deepcopy(run["retained_precision"])
            o["ar_reopen"] = r(lambda: rp.validate_retained_precision(back, inv), lambda v: f"bound={v['invocation_bound']},eligible={v['numerical_eligible']}")
            o["ar_reopen_standing"] = c.numerical_use_standing(back, req, inv)
    o["ar_mutations"] = rec
    o["ar_v02"] = r(lambda: c.build_analysis_run_v0_2(copy.deepcopy(src), **MANIFEST), lambda v: v["schema_version"])
    o["ar_wrapper_010"] = r(lambda: build_preview_analysis_run_envelope(copy.deepcopy(src)), lambda v: v["schema_version"])
    out.write(json.dumps(o, sort_keys=True) + "\n")
    out.flush()
