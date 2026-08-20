RequestedBy: WI-PKG02-DEL0204
RunID: HELP-HUMAN-PIPING-20260820-R6-ENGINEERING
ParentInstanceID: WI-PKG02-DEL0204
ChildInstanceID: TASK-PKG02-DEL0204-REMEDIATE-2
Role: TASK
TaskSkill: software-bounded-implementation
WorkingRoot: `{REPO_ROOT}/projects/chirality-piping`
ScopePath: `{WORKING_ROOT}/execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-04_Plugin and extension domain contracts`
PackageID: PKG-02
DeliverableIDs: [DEL-02-04]
ApplyEdits: true

Objective: close attempt-2 review findings 2/3 without disturbing behavior already accepted in review. The V2 manifest omission is already corrected by parent V3 and needs no product edit.

AcceptedBasis: current integrated implementation; `TASK-REREVIEW/RETURN.md`; DEL-02-04 REQ-02/03/07/14 and AC-001; accepted quantity fields `value/unit/dimension/provenance`; caller-loaded in-memory unit catalog evidence; project result-envelope diagnostic fields.

AllowedWriteTargets:

- `{WORKING_ROOT}/core/adapters/framework/**`
- `{WORKING_ROOT}/tests/test_adapter_plugin_verification_del_02_04.py`
- `{ScopePath}/_run_records/**`

Tasks:

1. Extend the in-memory verifier with explicit caller-loaded unit-catalog evidence mapping accepted unit IDs/symbols to canonical dimensions. Fail closed for missing/malformed catalog, nonnumeric or nonfinite values, noncanonical expected/actual dimensions, unknown units, incompatible unit-to-dimension mappings, and malformed/incomplete canonical provenance. The host-provided unit catalog is evidence, not a new bundled engineering-value table.
2. Propagate exact relevant diagnostic context into the composed result envelope: adapter findings bind adapter identity/provenance; plugin findings bind plugin identity/manifest provenance; unit findings bind the affected unit-evidence path and its quantity provenance. Do not hardcode or falsely attribute actual diagnostics to the invented adapter fixture. For malformed/missing provenance, preserve available input and explicit `TBD`/rejected status rather than silently claiming accepted invented provenance.
3. Add focused regressions for unknown unit, wrong unit for expected dimension, nonnumeric/nonfinite value, noncanonical dimension, malformed provenance, and exact source/provenance propagation for adapter/plugin/unit diagnostics. Preserve full field/class assertions.
4. Preserve schema execution, protected-content quarantine, malformed adapter containment, and runtime non-dispatch. Run focused + existing adapter/plugin tests, diff check, and explicit scope validation.

AcceptanceCriteria: both product blockers closed; no fabricated diagnostic provenance; incompatible/noncanonical quantities reject with UNIT_WARNING; exact context propagation tests pass; all prior behavior remains green; containment passes.

ExpectedOutputs: minimal product/test remediation, new immutable run record, exact checks/paths/risk.

EXCLUSIONS: no schema/fixture/desktop/status/memory/shared coordination/Git/receipt edit; no install/network/telemetry/runtime loader; no policy/decision/register/DAG/decomposition/lifecycle change.
