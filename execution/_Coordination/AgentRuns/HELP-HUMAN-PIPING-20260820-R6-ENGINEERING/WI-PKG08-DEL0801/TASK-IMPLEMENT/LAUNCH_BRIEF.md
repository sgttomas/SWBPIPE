---
requested_by: WI-PKG08-DEL0801
run_id: HELP-HUMAN-PIPING-20260820-R6-ENGINEERING
parent_instance_id: WI-PKG08-DEL0801
child_instance_id: TASK-IMPLEMENT
package_id: PKG-08
deliverable_ids: [DEL-08-01]
task_skill: software-bounded-implementation
apply_edits: true
status: frozen
---

# Sealed TASK brief — cross-layer component provenance

ScopePath: `{WORKING_ROOT}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-R6-ENGINEERING/WI-PKG08-DEL0801/TASK-IMPLEMENT`

WorkingRoot: `{REPO_ROOT}/projects/chirality-piping`

Objective: implement the smallest coherent executable integration proof that production TypeScript report-package generation carries component provenance into the Rust report-package consumer/output.

AcceptedBasis: Git `357a58b56726feba49507534159c3fbc4656b818`; DAG-009; R5; the frozen parent activation/work graph; DEL-08-01 `SOW_V1`; the live `buildReportPackageRequest` -> Tauri `ReportPackageRequest` -> Rust `assemble_wire_request`/package output seam.

Dependencies: DAG-unblocked; no child predecessor.

DeclaredReads: root/project instructions; `docs/SOFTWARE_WORKFLOW_PROFILE.md`; `software-workflow.json`; TASK and software-bounded-implementation contracts; frozen parent records; DEL-08-01 kit/recent run records; relevant `apps/desktop/src/features/report/**`, `apps/desktop/src-tauri/src/report_package_bridge.rs`, `core/reporting/report_package/**`, and invented report fixtures.

AllowedTools: read, rg, repository-native editor/apply_patch, focused Vitest and Cargo test/format commands explicitly required by this activation, `git diff`/`git status` read-only checks, and the software-workflow selection/containment tools. No network/install/release/destructive commands. Do not delegate.

AllowedWriteTargets:

- `{WORKING_ROOT}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-R6-ENGINEERING/WI-PKG08-DEL0801/TASK-IMPLEMENT/**`
- `{WORKING_ROOT}/apps/desktop/src/features/report/**`
- `{WORKING_ROOT}/core/reporting/report_package/**`
- `{WORKING_ROOT}/fixtures/reports/invented/**` only for a component-provenance cross-layer fixture
- `{WORKING_ROOT}/execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-01_Calculation report generator/_run_records/**`

Tasks:

1. Inspect the smallest current report-package producer/consumer/test surface.
2. Add an exact, deterministic cross-layer fixture or lossless exact projection bound by a TypeScript test to production `buildReportPackageRequest` output.
3. Consume the same fixture/projection through the Rust report-package boundary and prove the produced HTML/package member contains the component-provenance value and source evidence.
4. Cover both a present provenance component and a component missing provenance; the latter must remain an explicit `COMPONENT_PROVENANCE_MISSING` warning/non-accepted finding, never silently accepted.
5. Prove malformed component provenance fails closed at Rust deserialization/consumer validation. Preserve `private_only`, `pending`, and `private_project_data` classifications.
6. Change production source only if the proof exposes a concrete defect, and keep any repair minimal.
7. Run the focused TypeScript and Rust checks, format check if Rust is touched, and validate write containment.

AcceptanceCriteria: exact shared fixture/projection; TS production-output equality; Rust consumer/output assertion; present/missing/malformed coverage; no invented public reclassification; focused Vitest and report-package Cargo checks PASS; all writes contained; no unrelated cleanup.

EXCLUSIONS: `.opsproj` residual; lifecycle/register/decision/DAG/decomposition/PRD changes; report public-export/redaction policy changes; cloud/network/telemetry; protected/private real data; receipts; Git staging/commit/push/PR; sibling coordination paths.

ExpectedReturn: exact changed paths; concise behavior proof; exact commands/results; containment result; production-defect disposition; residual risks; blockers/rerun triggers. Do not remove the DEL-08-01 Remaining item; the parent owns fan-in closeout.

Escalation: stop and return if a public contract, broader write scope, protected/private real data, or acceptance-policy decision is required.
