#!/usr/bin/env python3
"""I66 U6b existing-identity control. Run from a lane root (P of base or candidate):
for every committed JSON document under fixtures/ and tests/ that carries a
`results` list (top level, or under `source`/`envelope`), record the Python
carriers' outcomes: dispatch, standing (no refs; the quality-case refs),
standing_reason, binding refusals, AnalysisRun build/validate, the explicit 0.2
constructor and the 0.1.0 wrapper. Outputs one TSV row per document and carrier."""
import hashlib, json, sys
from copy import deepcopy
from pathlib import Path

sys.path.insert(0, ".")
from core.analysis_runs import compatibility as c  # noqa: E402
from core.analysis_runs.records import build_preview_analysis_run_envelope  # noqa: E402


def digest(value):
    return hashlib.sha256(json.dumps(value, sort_keys=True, separators=(",", ":")).encode()).hexdigest()[:16]


def run(fn):
    try:
        out = fn()
        return "ok:" + (digest(out) if not isinstance(out, (str, type(None))) else str(out))
    except Exception as error:  # noqa: BLE001 - the control records every outcome
        return f"err:{type(error).__name__}:{str(error)[:120]}"


def candidates(doc):
    if isinstance(doc, dict):
        if isinstance(doc.get("results"), list):
            yield "", doc
        for key in ("source", "envelope", "source_envelope", "mechanics_result"):
            if isinstance(doc.get(key), dict) and isinstance(doc[key].get("results"), list):
                yield key, doc[key]


manifest = {"object_type": "InputManifest", "ref": "manifest:u6b-sweep"}
rows = []
for path in sorted(list(Path("fixtures").rglob("*.json")) + list(Path("tests").rglob("*.json"))):
    try:
        doc = json.loads(path.read_text())
    except Exception:  # noqa: BLE001
        continue
    for where, source in candidates(doc):
        name = f"{path}{'#' + where if where else ''}"
        quality = source.get("numerical_quality") if isinstance(source.get("numerical_quality"), dict) else {}
        refs = [q.get("basis_ref") for q in quality.get("cases", []) if isinstance(q, dict)]
        invocation = doc.get("invocation") if isinstance(doc.get("invocation"), dict) else None
        out = {
            "dispatch": run(lambda: list(c._source_contract(deepcopy(source))[:2])),
            "transport": run(lambda: list(c._source_contract(deepcopy(source), check_receipt=False)[:2])),
            "standing_none": run(lambda: c.numerical_use_standing(deepcopy(source), [])),
            "standing_refs": run(lambda: c.numerical_use_standing(deepcopy(source), refs, invocation)),
            "standing_reason": run(lambda: c.standing_reason(deepcopy(source))),
            "binding": run(lambda: [c.rule_binding_refusal(source, row) for row in source["results"]]),
            "summary": run(lambda: c.classification_summary(deepcopy(source), invocation)),
            "build": run(lambda: c.build_analysis_run(deepcopy(source), input_manifest_ref=manifest, input_manifest_hash="1" * 64)),
            "v0_2": run(lambda: c.build_analysis_run_v0_2(deepcopy(source), input_manifest_ref=manifest, input_manifest_hash="1" * 64)),
            "wrapper_0_1": run(lambda: build_preview_analysis_run_envelope(deepcopy(source))),
        }
        try:
            record = c.build_analysis_run(deepcopy(source), input_manifest_ref=manifest, input_manifest_hash="1" * 64)
            out["validate"] = run(lambda: c.validate_analysis_run_v0_3(record, deepcopy(source)) if record.get("schema_version") == "0.3.0" else "v0_2")
        except Exception as error:  # noqa: BLE001
            out["validate"] = "n/a"
        for carrier, value in out.items():
            rows.append(f"{name}\t{carrier}\t{value}")
print("\n".join(rows))
