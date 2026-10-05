#!/usr/bin/env python3
"""I66 read-origin manifest (records only): sha256 of every committed file I66 read.

Read-only `git cat-file blob <rev>:<path>` with GIT_OPTIONAL_LOCKS=0; nothing is
written except the output JSON. Usage, from NUM:

    GIT_OPTIONAL_LOCKS=0 python3 <this file> <out.json>
"""
import hashlib
import json
import os
import subprocess
import sys

P = "projects/chirality-piping"
T3 = f"{P}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3"
R = f"{T3}/RESUME_2026-09-30"

READS = [
    # Instructions and brief.
    ("HEAD", "AGENTS.md"), ("HEAD", "agents/AGENT_TASK.md"), ("HEAD", f"{P}/AGENTS.md"),
    ("HEAD", f"{R}/BRIEFS/I66_U6_CARRIERS_SCOPING.md"),
    # Plans, contracts, designs, rulings and reviews.
    ("HEAD", f"{R}/FIRST_PUBLICATION_PATH.md"), ("HEAD", f"{R}/I30/f2a_checkpoint0_01/SOURCE_MAP.md"),
    ("HEAD", f"{R}/I61/step4_plan_01/PLAN.md"), ("HEAD", f"{R}/I32/f2a_wire_c1/WIRE_CONTRACT.md"),
    ("HEAD", f"{R}/I32/f2a_wire_c2/CONTRACT_DELTA.md"), ("HEAD", f"{R}/I52/prepared_public_contract_02/C3_DELTA.md"),
    ("HEAD", f"{R}/I52/reader_integration_04/RETURN.md"), ("HEAD", f"{R}/I52/reader_integration_04/MANIFEST.json"),
    ("HEAD", f"{T3}/ROOT_RULINGS_V1.md"), ("HEAD", f"{T3}/DESIGN_NUMERICS/DESIGN.md"), ("HEAD", f"{T3}/DESIGN_STANDING/DESIGN.md"),
    ("HEAD", f"{R}/REVIEW_RV78/carried_artefacts_01/REVIEW.md"), ("HEAD", f"{R}/REVIEW_RV79/reader_confirm_06/REVIEW.md"),
    ("HEAD", f"{R}/REVIEW_RV80/reader_confirm_05/REVIEW.md"), ("HEAD", f"{R}/I61/u1_serializer_01/RETURN.md"),
    ("HEAD", f"{R}/I61/u1_serializer_01/_run_records/mutants.py"), ("HEAD", f"{R}/I61/u1_serializer_01/_run_records/mutants_summary.txt"),
    ("HEAD", f"{R}/I61/u1_serializer_02/_run_records/outputs_sha256.txt"), ("HEAD", f"{R}/I61/u3_facade_01/RETURN.md"),
    ("HEAD", f"{R}/I61/u5_reference_01/_run_records/inputs_sha256.txt"), ("HEAD", f"{R}/I61/u5_reference_01/_run_records/run_u5.sh"),
    ("HEAD", f"{R}/I61/receipt_experiment_02/_run_records/checks_and_class_parity.txt"),
    ("HEAD", f"{R}/verification/rv69_c3_selection_02/CHECKS.json"),
    ("HEAD", f"{P}/execution/_Coordination/WorkGraphs/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/WORK_GRAPH.md"),
    # Maintained code and schemas (NUM HEAD).
    ("HEAD", f"{P}/core/reporting/result_export/src/semantic_contract.rs"), ("HEAD", f"{P}/core/reporting/result_export/src/derivative.rs"),
    ("HEAD", f"{P}/core/reporting/result_export/src/retained_precision.rs"), ("HEAD", f"{P}/core/reporting/result_export/src/preview_physics_evidence.rs"),
    ("HEAD", f"{P}/core/reporting/result_export/src/lib.rs"), ("HEAD", f"{P}/core/reporting/result_export/tests/derivative_contract.rs"),
    ("HEAD", f"{P}/core/reporting/result_export/tests/preview_physics_contract.rs"),
    ("HEAD", f"{P}/core/analysis_runs/compatibility.py"), ("HEAD", f"{P}/core/analysis_runs/retained_precision.py"),
    ("HEAD", f"{P}/core/analysis_runs/records.py"), ("HEAD", f"{P}/core/analysis_runs/preview_physics_evidence.py"),
    ("HEAD", f"{P}/core/handoff/stress_neutral/package_v0_3.py"), ("HEAD", f"{P}/tests/test_retained_precision_contract.py"),
    ("HEAD", f"{P}/core/runner/headless/src/lib.rs"), ("HEAD", f"{P}/core/runner/headless/src/result_envelope_binding.rs"),
    ("HEAD", f"{P}/apps/desktop/src-tauri/src/lib.rs"), ("HEAD", f"{P}/core/product_physics/src/lib.rs"),
    ("HEAD", f"{P}/core/product_physics/src/retained_wire_tests.rs"), ("HEAD", f"{P}/core/product_physics/src/retained_facade_tests.rs"),
    ("HEAD", f"{P}/core/product_physics/tests/retained_precision_admission.rs"),
    ("HEAD", f"{P}/apps/desktop/src/features/results/numericalResultQuality.ts"), ("HEAD", f"{P}/apps/desktop/src/features/results/resultSemantics.ts"),
    ("HEAD", f"{P}/apps/desktop/src/features/results/knownSemanticLimitations.ts"), ("HEAD", f"{P}/apps/desktop/src/features/results/KnownSemanticNotices.tsx"),
    ("HEAD", f"{P}/apps/desktop/src/features/results/retainedPrecision.ts"), ("HEAD", f"{P}/apps/desktop/src/features/results/loadReferenceSourceEvidence.ts"),
    ("HEAD", f"{P}/apps/desktop/src/features/results/loadReferenceOutputAvailability.ts"), ("HEAD", f"{P}/apps/desktop/src/features/results/LoadReferenceOutputGate.tsx"),
    ("HEAD", f"{P}/apps/desktop/src/features/results/HistoricalRunContext.tsx"), ("HEAD", f"{P}/apps/desktop/src/features/results/previewPhysicsEvidence.ts"),
    ("HEAD", f"{P}/apps/desktop/src/features/results/physicsSourceRecovery.ts"), ("HEAD", f"{P}/apps/desktop/src/types.ts"),
    ("HEAD", f"{P}/apps/desktop/src/services/analysisRunCompatibility.ts"), ("HEAD", f"{P}/apps/desktop/src/services/previewService.ts"),
    ("HEAD", f"{P}/apps/desktop/src/services/ruleCheckService.ts"), ("HEAD", f"{P}/apps/desktop/src/features/workspace/workspaceSession.ts"),
    ("HEAD", f"{P}/apps/desktop/src/features/workspace/resultsSessionState.ts"), ("HEAD", f"{P}/apps/desktop/src/features/result-export/resultExportAdapter.ts"),
    ("HEAD", f"{P}/apps/desktop/src/features/report/reportPackageRequest.ts"),
    ("HEAD", f"{P}/schemas/results.v0.3.schema.yaml"), ("HEAD", f"{P}/schemas/analysis_run.v0.3.schema.json"),
    ("HEAD", f"{P}/schemas/stress_neutral_export.v0.3.schema.json"), ("HEAD", f"{P}/schemas/retained_precision_mp_v2.schema.json"),
    # U3 grant 1b, committed (I61's working tree was not read).
    ("4b31bbf23a", f"{P}/core/product_physics/src/lib.rs"),
    ("bee3dc07ca", f"{P}/core/product_physics/src/lib.rs"),
]


def git(*args, binary=False):
    env = dict(os.environ, GIT_OPTIONAL_LOCKS="0")
    return subprocess.run(["git", *args], capture_output=True, env=env, text=not binary)


def main(out):
    head = git("rev-parse", "HEAD").stdout.strip()
    rows = []
    for rev, path in READS:
        blob = git("cat-file", "blob", f"{rev}:{path}", binary=True)
        if blob.returncode != 0:
            rows.append({"rev": rev, "path": path, "error": "missing"})
            continue
        rows.append({"rev": rev, "path": path, "bytes": len(blob.stdout), "sha256": hashlib.sha256(blob.stdout).hexdigest()})
    with open(out, "w", encoding="utf-8") as fh:
        json.dump({"head": head, "reads": rows}, fh, indent=1)
        fh.write("\n")
    print(json.dumps({"head": head, "files": len(rows), "missing": sum("error" in r for r in rows)}))


if __name__ == "__main__":
    main(sys.argv[1])
