RequestedBy: WI-PKG02-DEL0204
RunID: HELP-HUMAN-PIPING-20260820-R6-ENGINEERING
ParentInstanceID: WI-PKG02-DEL0204
ChildInstanceID: TASK-PKG02-DEL0204-IMPLEMENT
Role: TASK
TaskSkill: software-bounded-implementation
WorkingRoot: `{REPO_ROOT}/projects/chirality-piping`
ScopePath: `{WORKING_ROOT}/execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-04_Plugin and extension domain contracts`
PackageID: PKG-02
DeliverableIDs: [DEL-02-04]
ApplyEdits: true

Objective: implement the smallest coherent executable adapter/plugin verification seam and focused regressions that close the exact `_STATUS.md ## Remaining` residual for unit safety, provenance, diagnostics, protected content, and adapter/plugin runtime regression. Preserve every unresolved loader/runtime/permission decision as `TBD`; this is verification and fail-closed gating, not a plugin loader.

AcceptedBasis: `357a58b56726feba49507534159c3fbc4656b818`; DAG-009; R5; `DEL-02-04-REQ-02/03/05/06/07/14`, `AC-001`, `VER-001`; current adapter framework, plugin manifest schema/fixture, and focused tests.

Dependencies: DAG-unblocked; no sibling-node edge.

DeclaredReads:

- root and project instructions, `agents/AGENT_TASK.md`, `skills/software-bounded-implementation/**`, software-workflow profile;
- DEL-02-04 `ScopeOfWork.md`, `_CONTEXT.md`, `_STATUS.md`, `MEMORY.md`, and recent PDU-037 run record;
- `core/adapters/framework/**`, `schemas/plugin_manifest.schema.yaml`, invented plugin/adapter fixtures, and focused adapter/plugin tests.

AllowedWriteTargets:

- `{WORKING_ROOT}/core/adapters/framework/**`
- `{WORKING_ROOT}/schemas/plugin_manifest.schema.yaml` only if strictly required for executable verification consistency
- `{WORKING_ROOT}/tests/test_adapter_plugin_verification_del_02_04.py`
- `{ScopePath}/_run_records/**`

AllowedTools: read, `rg`, `apply_patch`, registered check selection/execution/scope-validation helpers, and direct focused pytest invocation only as the brief-authorized `piping-pytest` subset.

Tasks:

1. Inspect the smallest relevant runtime/schema/test surface.
2. Implement a pure, deterministic, in-memory verification API that composes existing adapter declaration checks with plugin-manifest verification. It must expose structured findings/results and must not dispatch runtime code, access files/network/processes, or select a loader/runtime.
3. Make these five dimensions executable: unit safety, provenance completeness/quarantine posture, diagnostic-envelope compatibility, protected-content controls/quarantine posture, and adapter/plugin runtime regression. Missing/malformed/disabled controls fail closed; suspected protected content quarantines; a valid declaration/manifest remains blocked from runtime because runtime authority is unresolved.
4. Add focused DEL-02-04-bound tests proving the valid verification result and each fail-closed dimension, including the non-dispatch runtime regression.
5. Run the focused test file and any directly affected existing adapter/plugin tests. Return exact commands/results and validate write containment.

AcceptanceCriteria:

- all five named dimensions execute and fail closed where applicable;
- suspected protected content produces quarantine, not acceptance;
- valid declarations still do not dispatch a runtime;
- structured diagnostic findings name code, severity, path, message, and remediation;
- focused tests pass;
- no policy, acceptance criterion, runtime technology, telemetry, network, private/protected fixture, or lifecycle state is invented.

ExpectedOutputs: product source, focused tests, TASK run record, exact check evidence, changed paths, write-containment result, residual risks/blockers.

EXCLUSIONS: no desktop changes unless separately amended; no lifecycle transition; no status/memory edits by the child; no register/decision/DAG/decomposition/PRD changes; no cloud/network/telemetry; no protected/private data; no receipt/Git/push/PR; no writes outside the targets above.

Escalation: return without widening scope if executable behavior requires a human-held runtime/permission decision, a public schema contract change beyond strict consistency, or writes outside the whitelist.
