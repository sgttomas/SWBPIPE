#!/usr/bin/env python3
"""I66 U7 slice F, the oracle-diff control (Python): run one lane's Python reader and
carriers on every input and dump the outcomes. Run from that lane's P root.
Per input: the reader's outcome (pass, or gate and code) and classifications digest,
invocation_bound, numerical_eligible, the reader's standing label, the carrier token
(numerical_use_standing with the input's invocation and requested refs), the classification
summary, the header-only transport and, for inputs marked `binding`, every row's binding
refusal. For each milestone input it also records the AnalysisRun build/validate, the
stress-neutral packager's build and the 0.1.0 wrapper (D-U6-9).
Usage: dump_py.py INPUTS_JSON OUT_JSON"""
import hashlib, json, sys
from copy import deepcopy

sys.path.insert(0, "."); sys.path.insert(0, "tests")
from core.analysis_runs import compatibility as c  # noqa: E402
from core.analysis_runs import retained_precision as rp  # noqa: E402


def digest(value):
    return hashlib.sha256(json.dumps(value, sort_keys=True, separators=(",", ":")).encode()).hexdigest()[:16]


def outcome(fn):
    try:
        return ["ok", fn()]
    except Exception as error:  # noqa: BLE001 - every outcome is recorded
        return ["err", type(error).__name__, str(error)[:160]]


rows = {}
for record in json.load(open(sys.argv[1])):
    source, invocation, refs = record["source"], record["invocation"], record["requested"]
    row = {}
    try:
        v = rp.validate_retained_precision(deepcopy(source), deepcopy(invocation))
        row["reader"] = "pass"
        row["classes"] = digest(v["classifications"])
        row["invocation_bound"] = v["invocation_bound"]
        row["numerical_eligible"] = v["numerical_eligible"]
        row["reader_standing"] = v["standing"]
    except rp.RetainedPrecisionError as error:
        row["reader"] = f"{error.gate}:{error.code}"
    row["carrier_token"] = c.numerical_use_standing(deepcopy(source), refs, deepcopy(invocation))
    row["summary"] = c.classification_summary(deepcopy(source), deepcopy(invocation))
    row["transport"] = outcome(lambda: list(c._source_contract(deepcopy(source), check_receipt=False)[:2]))
    if record.get("binding"):
        row["binding"] = digest([c.rule_binding_refusal(source, r) for r in source.get("results", [])])
    if record.get("milestone"):
        from core.analysis_runs.records import build_preview_analysis_run_envelope
        from core.handoff.stress_neutral import package_v0_3 as sn
        from tests.test_stress_neutral_physics_source import arguments
        manifest = {"input_manifest_ref": {"object_type": "InputManifest", "ref": "manifest:u7f"}, "input_manifest_hash": "1" * 64}
        record_ok = outcome(lambda: c.build_analysis_run(deepcopy(source), **manifest))
        row["analysis_run"] = [record_ok[0], digest(record_ok[1]) if record_ok[0] == "ok" else record_ok[1:]]
        if record_ok[0] == "ok":
            row["analysis_run_validate"] = outcome(lambda: c.validate_analysis_run_v0_3(record_ok[1], deepcopy(source)))
            row["packager"] = outcome(lambda: sn.build_stress_neutral_export_package_v0_3(source_envelope=source, analysis_record=record_ok[1], **arguments(source, record_ok[1])))[:3]
        row["wrapper_0_1"] = outcome(lambda: build_preview_analysis_run_envelope(deepcopy(source)))[:3]
    rows[record["key"]] = row
json.dump(rows, open(sys.argv[2], "w"), indent=0, sort_keys=True)
print(len(rows), "rows")
