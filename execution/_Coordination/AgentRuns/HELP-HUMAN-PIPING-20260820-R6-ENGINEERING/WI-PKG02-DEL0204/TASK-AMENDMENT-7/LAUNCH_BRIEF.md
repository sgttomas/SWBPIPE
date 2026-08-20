RequestedBy: WI-PKG02-DEL0204
RunID: HELP-HUMAN-PIPING-20260820-R6-ENGINEERING
ParentInstanceID: WI-PKG02-DEL0204
ChildInstanceID: TASK-PKG02-DEL0204-AMENDMENT-7
Role: TASK
TaskSkill: software-bounded-implementation
WorkingRoot: `{REPO_ROOT}/projects/chirality-piping`
ScopePath: `{WORKING_ROOT}/execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-04_Plugin and extension domain contracts`
AcceptedBasis: `357a58b56726feba49507534159c3fbc4656b818`; integrated review v7; Amendments 1-6/V33; DEL requirements.
AllowedReadTargets: instructions, software workflow, selected deliverable/run evidence, focused adapter/plugin code/tests/schemas.
AllowedWriteTargets: `core/adapters/framework/plugin_verification.py`; `tests/test_adapter_plugin_verification_del_02_04.py`; adjacent DEL-02-04 `_run_records/TASK_RUN_2026-08-20_*.md`; this task return/status.
Objective: make normalized-success envelope derivation treat schema-valid `metadata.status="quarantined"` identically to malformed fallback: force protected/quarantine top-level privacy/provenance and never public-reviewed. Add direct/composed interactions with public/private/protected evidence, canonical envelope schema, and runtime false. Do not widen semantics.
Checks: complete N1 suite; composed schema assertions; exact containment; `git diff --check`.
ExpectedReturn: exact paths/hashes/checks/run record. No closeout/N2/N3/schema/Git writes.
EXCLUSIONS: no policy/schema/lifecycle/runtime/network/telemetry/N2/N3/Git/receipt/PR.
