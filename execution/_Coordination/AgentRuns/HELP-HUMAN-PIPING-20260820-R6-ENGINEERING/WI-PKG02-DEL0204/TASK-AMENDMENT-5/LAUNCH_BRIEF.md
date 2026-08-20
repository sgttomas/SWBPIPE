RequestedBy: WI-PKG02-DEL0204
RunID: HELP-HUMAN-PIPING-20260820-R6-ENGINEERING
ParentInstanceID: WI-PKG02-DEL0204
ChildInstanceID: TASK-PKG02-DEL0204-AMENDMENT-5
Role: TASK
TaskSkill: software-bounded-implementation
WorkingRoot: `{REPO_ROOT}/projects/chirality-piping`
ScopePath: `{WORKING_ROOT}/execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-04_Plugin and extension domain contracts`
ApplyEdits: true
AcceptedBasis: `357a58b56726feba49507534159c3fbc4656b818`; current Amendment 4/V26 accepted N1 worktree; integrated review v5 finding.
AllowedWriteTargets:
  - `{WORKING_ROOT}/core/adapters/framework/adapter_framework.py`
  - `{WORKING_ROOT}/core/adapters/framework/plugin_verification.py`
  - `{WORKING_ROOT}/tests/test_adapter_framework_contract.py`
  - `{WORKING_ROOT}/tests/test_adapter_plugin_verification_del_02_04.py`
  - `{ScopePath}/_run_records/`
AllowedTools: read, rg, apply_patch, registered tests/check helpers authorized by the skill.
Tasks:
  - Before any validation, traversal, diagnostic construction, or envelope-boundary derivation, normalize every caller-supplied adapter declaration/result payload, unit catalog, and unit-evidence value into independently bounded plain exact-JSON snapshots.
  - Fail closed with structured findings for hostile/custom Mapping or list accessors, non-exact primitives/containers, cycles, depth overflow, nonfinite numbers, and serialization failures. No caller exception may escape and caller evidence must remain unchanged.
  - Preserve protected/quarantined marker precedence whenever markers are safely observable through exact built-in containers and exact primitive values; do not invoke hostile accessors merely to search for markers.
  - Ensure all downstream adapter/unit validation, diagnostics, provenance/privacy derivation, unit-catalog lookups, and result-envelope construction use only the detached snapshots or conservative absent/malformed contexts.
  - Add direct adapter-framework and public composed-verifier regressions for hostile Mapping/list/accessor, cycle, depth, and nonfinite inputs across adapter, unit_catalog, and unit_evidence, including safely observable protected/quarantined combinations. Require structured REJECTED/QUARANTINE, exact schema-valid envelopes, `verification_passed=False` when inputs are malformed, and `runtime_dispatched=False`.
  - Preserve Amendment 1-4 behavior, canonical plugin-schema authentication, and runtime non-dispatch.
AcceptanceChecks: complete N1 focused/existing pytest suite passes, including composed-result canonical schema assertions; exact changed-path containment and diff check PASS; no raw caller traversal or exception escape remains.
ExpectedOutputs: exact changed paths/hashes, behavioral summary, checks, scope evidence, residuals, and durable TASK run record.
EXCLUSIONS: no schema, fixture, lifecycle/status/memory/manager/fan-in, N2/N3, decision/register/DAG/decomposition, runtime/transport/loader, network, Git, receipt, or PR change.
