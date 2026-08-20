RequestedBy: WI-PKG02-DEL0204
RunID: HELP-HUMAN-PIPING-20260820-R6-ENGINEERING
ParentInstanceID: WI-PKG02-DEL0204
ChildInstanceID: TASK-PKG02-DEL0204-AMENDMENT-6
Role: TASK
TaskSkill: software-bounded-implementation
WorkingRoot: `{REPO_ROOT}/projects/chirality-piping`
ScopePath: `{WORKING_ROOT}/execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-04_Plugin and extension domain contracts`
AcceptedBasis: `357a58b56726feba49507534159c3fbc4656b818`; integrated review v6 finding; Amendments 1-5/V31; current DEL requirements.
AllowedReadTargets: repository/project/TASK/software-workflow instructions, selected deliverable kit/recent run records, adapter/plugin code/tests/schemas.
AllowedWriteTargets: `core/adapters/framework/plugin_verification.py`; `tests/test_adapter_plugin_verification_del_02_04.py`; `tests/test_plugin_manifest_schema.py` only if exact direct regression requires it; adjacent DEL-02-04 `_run_records/TASK_RUN_2026-08-20_*.md`; this instance return/status.
Objective: canonicalize and byte-bound every schema-generated mismatch path segment and sanitize/bound caller-derived mismatch text after the manifest snapshot. Add direct and composed finite adversarial-key and huge-key regressions asserting canonical bounded schema-valid references/messages, protected marker precedence, and runtime non-dispatch. Do not weaken schema acceptance or invent values.
Checks: complete N1 focused+existing adapter/plugin/schema suite; composed canonical schema assertions; exact write containment; `git diff --check`.
ExpectedReturn: exact changed paths/hashes, checks, residuals, durable run record. No manager/status/MEMORY/fan-in/N2/N3/Git writes.
EXCLUSIONS: no schema/policy/lifecycle/runtime/network/telemetry/protected data/N2/N3/Git/receipt/PR.
