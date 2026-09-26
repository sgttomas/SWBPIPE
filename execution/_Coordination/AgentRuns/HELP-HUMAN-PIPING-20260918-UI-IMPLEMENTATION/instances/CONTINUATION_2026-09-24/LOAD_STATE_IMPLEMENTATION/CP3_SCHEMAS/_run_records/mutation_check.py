"""Test-the-tests: each deliberate schema weakening must make the targeted new tests fail.

Run from a scratch copy of WORKING_ROOT (never the worktree):  <python> <this file>
Each mutation edits one schema file in the scratch tree, runs a pytest -k
selection of tests/test_load_reference_schema.py, expects a nonzero exit, and
restores the original bytes.
"""
from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path

ROOT = Path.cwd()
assert "scratchpad" in str(ROOT), "run only in a scratch copy"


def edit(name, change):
    path = ROOT / "schemas" / name
    original = path.read_bytes()
    data = json.loads(original)
    change(data)
    path.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")
    return path, original


def results_branch(data):
    return data["$defs"]["ResultEnvelope"]["oneOf"][4]


MUTATIONS = [
    ("results LR branch drops source_block_recovery prohibition", "results.v0.3.schema.yaml",
     lambda d: results_branch(d).pop("not"), "results_load_reference_branch_rejects and source_block_recovery"),
    ("results LR branch drops required contract_evidence", "results.v0.3.schema.yaml",
     lambda d: results_branch(d).pop("required"), "results_load_reference_branch_rejects and missing_evidence"),
    ("results LR profile const becomes physics profile", "results.v0.3.schema.yaml",
     lambda d: results_branch(d)["properties"]["formulation_basis"]["properties"]["profile_id"].update(const="exact_straight_pressure_v2"),
     "results_load_reference_branch_accepts"),
    ("analysis_run LR SemanticContract pair uses physics hash", "analysis_run.v0.3.schema.json",
     lambda d: d["$defs"]["SemanticContract"]["oneOf"][4]["properties"]["sha256"].update(
         const="9a2cf6268b57bd5265a1a115497c07450819dd4d03cd5ab618097bd9d19da8cc"),
     "analysis_run_load_reference_branch_mirrors_physics"),
    ("analysis_run LR branch permits source_block_recovery", "analysis_run.v0.3.schema.json",
     lambda d: d["$defs"]["AnalysisRun"]["oneOf"][4]["not"]["anyOf"].pop(0),
     "analysis_run_load_reference_branch_rejects and source_block_recovery"),
    ("stress-neutral LR branch drops source_block_recovery prohibition", "stress_neutral_export.v0.3.schema.json",
     lambda d: d["oneOf"][4].pop("not"), "stress_neutral_load_reference_branch_rejects and source_block_recovery"),
    ("Quantity opened", "load_reference_state.schema.json",
     lambda d: d["$defs"]["Quantity"].pop("additionalProperties"), "authored_unknown_key"),
    ("fit none opened", "load_reference_state.schema.json",
     lambda d: d["$defs"]["FitReference"]["oneOf"][0].pop("additionalProperties"), "fit_with_both and none"),
    ("history accepts any kind", "load_reference_state.schema.json",
     lambda d: d["$defs"]["History"]["oneOf"][0]["properties"]["kind"].pop("const"), "unknown_discriminant and history"),
    ("member record opened", "load_reference_state.schema.json",
     lambda d: d["$defs"]["LoadReferenceMember"].pop("additionalProperties"), "raw_unknown_key and members"),
    ("record drops required source_recovery", "load_reference_state.schema.json",
     lambda d: d["$defs"]["LoadReferenceStateRecord"]["required"].remove("source_recovery"), "raw_missing_key and source_recovery"),
    ("pressure copy drifts from physics", "load_reference_state.schema.json",
     lambda d: d["$defs"]["LoadReferencePressureEvidence"]["properties"]["p_pa"].update(minimum=0),
     "physics_shape_copies_differ_only"),
    ("operating_temperature made required", "load_reference_state.schema.json",
     lambda d: d["$defs"]["ElementState"]["required"].append("operating_temperature"),
     "missing_key_is_rejected_unless and operating_temperature"),
]

failures = 0
for title, name, change, selection in MUTATIONS:
    path, original = edit(name, change)
    try:
        completed = subprocess.run([sys.executable, "-m", "pytest", "-q", "-p", "no:cacheprovider",
                                    "tests/test_load_reference_schema.py", "-k", selection],
                                   cwd=ROOT, capture_output=True, text=True)
    finally:
        path.write_bytes(original)
    summary = completed.stdout.strip().splitlines()[-1] if completed.stdout.strip() else completed.stderr[-300:]
    caught = completed.returncode == 1 and "failed" in summary
    failures += not caught
    print(f"{'CAUGHT' if caught else 'MISSED'}  {title}  [-k {selection}]  -> {summary}")
print("MUTATION CHECK", "PASS" if failures == 0 else f"FAIL ({failures} missed)")
sys.exit(1 if failures else 0)
