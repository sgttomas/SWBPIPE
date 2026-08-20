RequestedBy: WI-PKG02-DEL0204
RunID: HELP-HUMAN-PIPING-20260820-R6-ENGINEERING
ParentInstanceID: WI-PKG02-DEL0204
ChildInstanceID: TASK-PKG02-DEL0204-AMENDMENT-4
Role: TASK
TaskSkill: software-bounded-implementation
WorkingRoot: `{REPO_ROOT}/projects/chirality-piping`
ScopePath: `{WORKING_ROOT}/execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-04_Plugin and extension domain contracts`
ApplyEdits: true
AcceptedBasis: `357a58b56726feba49507534159c3fbc4656b818`; current Amendment 3/V20 accepted N1 worktree; integrated review v4 finding.
AllowedWriteTargets:
  - `{WORKING_ROOT}/core/adapters/framework/adapter_framework.py`
  - `{WORKING_ROOT}/tests/test_adapter_framework_contract.py`
  - `{WORKING_ROOT}/tests/test_adapter_plugin_verification_del_02_04.py`
  - `{ScopePath}/_run_records/`
AllowedTools: read, rg, apply_patch, registered tests/check helpers authorized by the skill.
Tasks:
  - Emit blocking adapter provenance findings when a complete canonical declaration or operation-result provenance record is not cleared: `review_status` is `rejected`, `needs_review`, or `TBD`, and preserve fail-closed handling for unresolved/non-cleared redistribution postures.
  - Emit a quarantine finding when declaration or operation-result privacy classification is `protected_suspected`.
  - Add direct declaration and public composed-verifier regressions for both declaration and operation-result provenance/privacy cases, including malformed-capability combinations that prove quarantine precedence remains visible.
  - Preserve private/protected top-level envelope dominance and runtime non-dispatch.
  - Make the smallest coherent product/test change; do not alter canonical schemas or acceptance policy.
AcceptanceChecks: complete N1 focused/existing pytest suite passes, including composed-result canonical schema assertions; explicit changed-path containment and diff check PASS; no exception/bypass path.
ExpectedOutputs: exact changed paths, behavioral summary, checks, scope evidence, residuals, and durable TASK run record.
EXCLUSIONS: no N2/N3, schema, lifecycle/status/memory/manager/fan-in, decision/register/DAG/decomposition, runtime/transport/loader, network, Git, receipt, PR, or shared coordination edits.
