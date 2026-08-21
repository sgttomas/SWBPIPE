# Sealed bounded-execution brief — N1

- RequestedBy: `HELP_HUMAN`
- RunID: `HELP-HUMAN-PIPING-20260820-R9-STALE-HASH`
- ParentInstanceID: `HELP_HUMAN`
- ExecutorInstanceID: `WORKING-ITEMS-PKG05-DEL0504-STALE-HASH`
- PackageID: `PKG-05`
- DeliverableID: `DEL-05-04`
- Objective: close exactly the runtime stale-hash acceptance-reuse Remaining item by changing the authoritative Rust operation-applier runtime.
- ScopePath: `projects/chirality-piping/core/model_operations/operation_applier/`
- AcceptedBasis: `DAG-009`; `DEC-020`; R5; `Receipt-120`; `cd823be3badd034c86390f2707dcf01952c782f0`; exact DEL-05-04 Remaining item.
- Dependencies: satisfied DEL-05-04 upstream execution rows in DAG-009; existing Rust/JCS model-hash and operation-application contracts.
- DeclaredReads: activation sources; operation-applier crate; contract corpus runner/fixtures; applicable desktop adapter tests; registered check definitions.
- AllowedTools: read, `rg`, `cargo`, `npm`, project validation/check commands, `apply_patch`, Git read-only inspection.
- AllowedWriteTargets: exactly the activation write fence in `ACTIVATION.md`.
- ExpectedOutputs: runtime hash-claim evaluation; deterministic blocking diagnostic; blocked status without applied model/acceptance reuse; focused Rust tests; native-to-Wasm corpus parity evidence; deliverable-local status/memory/run record; frozen-diff review package.
- AcceptanceCriteria:
  1. Evaluate a supplied hash only when algorithm, canonicalization, and payload scope make equality meaningful.
  2. A matching current hash preserves ordinary validate/apply behavior.
  3. A stale or mismatched hash emits a stable blocking diagnostic, returns `application_status=blocked`, and returns no `applied_model` or acceptance reuse.
  4. Malformed or unsupported hash metadata fails closed rather than being treated as current.
  5. No supplied hash preserves existing before-state staleness behavior.
  6. Native and Wasm contract-corpus behavior is identical.
  7. Professional-boundary claims remain unchanged.
- EXCLUSIONS: no change to acceptance criteria, no UI workflow ownership, no lifecycle/release gate, no root/shared coordination writes, no commit/push/PR.
- Escalation: stop only for scope/acceptance changes; otherwise remediate implementation or test failures inside N1.

This record was frozen before product-code edits.
