---
requested_by: WI-PKG08-DEL0801
run_id: HELP-HUMAN-PIPING-20260820-R6-ENGINEERING
parent_instance_id: WI-PKG08-DEL0801
child_instance_id: TASK-REVIEW-4
package_id: PKG-08
deliverable_ids: [DEL-08-01]
task_skill: software-code-review
apply_edits: false
status: frozen
review_attempt: 4
---

# Sealed fresh read-only full-N2 review after Amendment 3

ScopePath: `{WORKING_ROOT}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-R6-ENGINEERING/WI-PKG08-DEL0801/TASK-REVIEW-4`

ImplementationBrief: original implementation brief, Amendments 1–3, integrated-review findings through v8, and implementation returns through `RETURN_ATTEMPT_4.md`.

AcceptedBasis and DiffBasis: review 100% of the complete final four-file N2 product/test diff from original Git basis `357a58b56726feba49507534159c3fbc4656b818`, not only the uncommitted Amendment 3 delta. Exact diff: 360 additions, 4 deletions. Reject if hashes differ from `RETURN_ATTEMPT_4.md`: production TS `514b05...db99`; test TS `c3fe95...3437`; Rust `4a04fb...89e1`; fixture `3b2070...d110`.

PriorFindings to backcheck:

1. Fixture root/version drift — Amendment 1.
2. Test exact-session model/manifest/analysis-run basis mismatch — Amendment 2.
3. Production same-ID different model payload accepted against a verified manifest — Amendment 3.

VerificationEvidence: TASK and amendment records; manager rerun Vitest PASS 8/8; registered desktop build/typecheck PASS; report-package Cargo PASS 19/19 plus doc tests; fmt PASS; containment and basis-scoped diff check PASS.

PROFILE_PATH: `{WORKING_ROOT}/software-workflow.json` under `docs/SOFTWARE_WORKFLOW_PROFILE.md`.

AllowedWriteTargets: none. Managed read-only TASK; parent owns runtime records. Do not edit or create a TASK-local run record.

DeclaredReads: all frozen briefs/findings/returns; exact complete four-file diff from original basis; canonical hash service; manifest verification/builder; mechanics/analysis-run builder; report request producer/callers; Rust DTO/assembly/renderer; DEL-08-01 contract/residual; project and software-code-review instructions.

AllowedTools: read, rg, read-only Git diff/status/hash inspection, and read-only scope/check selection. Do not run tests, edit, install, release, use network, or delegate.

Review objective: independently decide whether the production defect is correctly and minimally fixed and the full N2 evidence is sufficient. Verify gate ordering before rendering/package assembly; canonical-service reuse without fallback; exact digest validation/equality; stable fail-closed error; same-ID changed-value rejection; insertion-order equivalence; model hash reuse; manifest/analysis-run binding; all prior fixture/provenance/Rust output gates; scope, security, regression risk, and test strength.

EXCLUSIONS: no repair, lifecycle/register/decision/DAG/decomposition/PRD/receipt/Git change; no `.opsproj` work; no policy reinterpretation.

ExpectedReturn: PASS with no actionable findings, or exact findings; 100% full-diff coverage; disposition of all three prior findings; residual risk; fan-in validity. No lifecycle acceptance.
