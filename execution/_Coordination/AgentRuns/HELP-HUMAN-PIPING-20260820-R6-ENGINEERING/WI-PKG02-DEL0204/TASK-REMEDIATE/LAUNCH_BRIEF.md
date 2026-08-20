RequestedBy: WI-PKG02-DEL0204
RunID: HELP-HUMAN-PIPING-20260820-R6-ENGINEERING
ParentInstanceID: WI-PKG02-DEL0204
ChildInstanceID: TASK-PKG02-DEL0204-REMEDIATE
Role: TASK
TaskSkill: software-bounded-implementation
WorkingRoot: `{REPO_ROOT}/projects/chirality-piping`
ScopePath: `{WORKING_ROOT}/execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-04_Plugin and extension domain contracts`
PackageID: PKG-02
DeliverableIDs: [DEL-02-04]
ApplyEdits: true

Objective: remediate all four blocking findings in `../TASK-REVIEW/RETURN.md` within the same N1 node, with no scope or policy expansion.

AcceptedBasis: initial implementation attempt plus attempt-1 review; DEL-02-04 REQ-02/03/05/06/07/14, AC-001, VER-001; canonical `schemas/plugin_manifest.schema.yaml`; existing canonical quantity shape `{value, unit, dimension, provenance}` and dimension vocabulary from `core/units/schema_vocabulary.py`; existing `build_result` diagnostic/result-envelope basis.

AllowedWriteTargets:

- `{WORKING_ROOT}/core/adapters/framework/**`
- `{WORKING_ROOT}/tests/test_adapter_plugin_verification_del_02_04.py`
- `{ScopePath}/_run_records/**`

Tasks:

1. Make canonical plugin-manifest schema conformance executable against an already-loaded schema mapping, without file/network/process access and without a new dependency. Support every JSON Schema construct actually used by the canonical plugin schema, including local `$ref`, required/additionalProperties, types, const/enum, patterns, length/item constraints, unique items, arrays, and nested definitions. Missing schema or schema mismatch must return structured blocking findings. Build a schema-valid invented manifest in the focused test without editing the historical fixture, and prove missing/wrong-type/additional-property/direct-persistence shapes reject.
2. Add an explicit in-memory unit verification input using the already accepted quantity shape (`value`, `unit`, `dimension`, `provenance`) plus caller-declared expected dimension/path. Missing unit/dimension/provenance or a dimension mismatch must return structured unit findings; a valid invented quantity must pass. `None`/malformed unit evidence fails closed; an explicit empty list is valid only as a declaration that no dimensional values are present.
3. Add the existing project result envelope to the composed result using `build_result`, so every finding has code, class, severity, source, affected object, message, remediation, and provenance. Tests must assert class propagation for unit, provenance, protected-content, and generic adapter blocking outcomes.
4. Contain malformed nested adapter types/exceptions and return `REJECTED`, never raise; test `capabilities=None` and an unhashable capability element.
5. Preserve provenance/quarantine/runtime non-dispatch behavior and every runtime/transport/permission decision as `TBD`. Run focused + existing adapter/plugin tests and scope validation.

AcceptanceCriteria: all four review findings closed; all five residual dimensions executable; schema-invalid manifests reject; missing/incompatible units reject; suspected protected content quarantines; full diagnostic envelope present; malformed input never raises; valid pair remains runtime-blocked; tests and containment pass.

ExpectedOutputs: remediated product/tests, new immutable TASK run record, exact checks/scope evidence, residual risks.

EXCLUSIONS: no fixture/schema/desktop/status/memory edit; no dependency/install/network/telemetry/runtime loader; no register/decision/DAG/decomposition/PRD/lifecycle/Git/receipt changes.
