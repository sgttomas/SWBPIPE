#!/usr/bin/env python3
"""Verify exact final-500 native-delivered result and stress packages."""
from __future__ import annotations

import argparse
from copy import deepcopy
from hashlib import sha256
import importlib.util
import json
import os
from pathlib import Path
import sys

EXPECTED = [
    "manifest.json", "stress_neutral_results.csv", "result_rows.json",
    "unit_system_disclosure.json", "unit_preservation_witnesses.json",
    "stable_id_map.json", "loss_report.json", "validation_report.json",
    "diagnostics.json",
]


def load(path: Path):
    return json.loads(path.read_text(encoding="utf-8"))


p = argparse.ArgumentParser()
p.add_argument("--working-root", type=Path, required=True)
p.add_argument("--checked-json-bin", type=Path, required=True)
p.add_argument("--result", type=Path, required=True)
p.add_argument("--stress", type=Path, required=True)
p.add_argument("--model", type=Path, required=True)
p.add_argument("--mechanics", type=Path, required=True)
p.add_argument("--analysis", type=Path, required=True)
p.add_argument("--members", type=Path, required=True)
p.add_argument("--shared-verifier", type=Path, required=True)
p.add_argument("--report", type=Path, required=True)
a = p.parse_args()
root = a.working_root.resolve()
checked = a.checked_json_bin.resolve()
assert checked.is_file()
os.environ["OPENPIPESTRESS_CHECKED_JSON_BIN"] = str(checked)
sys.path[:0] = [str(root), str(root / "tests")]
from schema_validation import validate_instance
from core.analysis_runs.compatibility import verify_analysis_run_record
from core.handoff.stress_neutral import materialized_members_v0_2, validate_stress_neutral_export_package_v0_2
from core.serialization.canonical_json.adapter import canonical_sha256_checked_v1

spec = importlib.util.spec_from_file_location("shared_delivery_verifier", a.shared_verifier.resolve())
assert spec and spec.loader
shared = importlib.util.module_from_spec(spec)
spec.loader.exec_module(shared)

result, stress = load(a.result), load(a.stress)
model, mechanics, analysis = load(a.model), load(a.mechanics), load(a.analysis)
validate_instance(load(root / "schemas/results.schema.yaml"), result, schema_label="results.schema.yaml", instance_label="final500 delivered result")
validate_instance(load(root / "schemas/stress_neutral_export.schema.json"), stress, schema_label="stress_neutral_export.schema.json", instance_label="final500 delivered stress")
validate_instance(load(root / "schemas/analysis_run.schema.json"), analysis, schema_label="analysis_run.schema.json", instance_label="final500 stored analysis")
assert verify_analysis_run_record(analysis) == "match"
result_summary = shared.verify_result(result, mechanics, canonical_sha256_checked_v1)
model_claim = result["result_envelope"]["reproducibility"]["model_hash"]
assert model_claim["value"] == canonical_sha256_checked_v1(model)
assert model_claim["payload_ref"] == {"ref_type": "model_payload", "ref_id": mechanics["model_ref"]}
assert stress["received_source_checksums"] == analysis["analysis_run"]["hashes"]
validate_stress_neutral_export_package_v0_2(stress)
assert len(stress["result_rows"]) == len(mechanics["results"]) == 67
assert {row["result_id"] for row in stress["result_rows"]} == {row["id"] for row in mechanics["results"]}

expected_members = materialized_members_v0_2(stress)
actual_names = sorted(item.name for item in a.members.iterdir() if item.is_file())
assert actual_names == sorted(EXPECTED)
member_hashes = {}
for name in EXPECTED:
    actual = (a.members / name).read_bytes()
    assert actual == expected_members[name]
    member_hashes[name] = {"sha256": sha256(actual).hexdigest(), "bytes": len(actual)}

tampered = deepcopy(stress)
tampered["result_rows"][0]["value"] += 1
try:
    validate_stress_neutral_export_package_v0_2(tampered)
except (AssertionError, ValueError) as exc:
    tamper_rejection = str(exc)
else:
    raise AssertionError("STRESS_COVERED_FIELD_TAMPER_ACCEPTED")

report = {
    "status": "PASS",
    "candidate": "8ad37207cf088025623aa1e777a97a6fcb802f48",
    "checked_authority": {"path": str(checked), "sha256": sha256(checked.read_bytes()).hexdigest(), "profile": "openpipestress_jcs_ijson_v1"},
    "analysis": {"strict_schema": "PASS", "record_checksum": "match", "row_count": len(analysis["analysis_run"]["result_refs"])},
    "result": {**result_summary, "strict_schema": "PASS", "delivered_sha256": sha256(a.result.read_bytes()).hexdigest()},
    "stress": {"strict_schema": "PASS", "owning_validator": "PASS", "row_count": len(stress["result_rows"]), "tamper_rejection": tamper_rejection, "delivered_sha256": sha256(a.stress.read_bytes()).hexdigest()},
    "members": {"count": len(member_hashes), "exact_owning_materialization": True, "items": member_hashes},
    "standalone_native_csv_control": "withheld_by_registered_local-private-transport_guard; successful CSV is packet-derived csv_text and owning nine-member materialization",
}
a.report.parent.mkdir(parents=True, exist_ok=True)
a.report.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")
print(json.dumps({"status": "PASS", "report": str(a.report.resolve()), "sha256": sha256(a.report.read_bytes()).hexdigest()}, sort_keys=True))
