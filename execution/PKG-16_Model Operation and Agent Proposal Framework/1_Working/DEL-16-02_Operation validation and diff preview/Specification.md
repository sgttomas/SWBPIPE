# Specification: DEL-16-02 Operation validation and diff preview

## Scope

This deliverable covers the backend feature slice that validates proposed structured model operations and creates deterministic diff previews before those operations are applied. It is grounded in SOW-069 and OBJ-015. Source: `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md#PKG-16`; `docs/_Registers/ScopeLedger.csv`.

This deliverable excludes hidden model mutations, autonomous engineering acceptance, product claims of professional approval or code compliance, and ownership of the structured operation schema itself. The operation schema is upstream DEL-16-01. Source: `_CONTEXT.md#Package Reference`; `Dependencies.csv` row `DAG-002-E0827`; `docs/CONTRACT.md#Invariant index`.

## Requirements

| ID | Requirement | Source | Verification |
|---|---|---|---|
| REQ-16-02-001 | Proposed GUI and agent edits must enter this slice as structured model operations, not as direct hidden mutations of accepted model state. | SOW-069 in `_CONTEXT.md`; `docs/_Registers/ScopeLedger.csv`; `execution/_Decomposition/SOFTWARE_DECOMP.md#PKG-16`; `schemas/model_operation.schema.json` | `tests/test_model_operation_schema.py` checks structured-operations-only and direct mutation forbidden schema constants; `tests/test_operation_validation_preview.py::test_direct_mutation_request_is_rejected` checks current engine blocking behavior. |
| REQ-16-02-002 | The slice must perform JSON Schema 2020-12 validation against the DEL-16-01 operation schema before treating an operation as schema-passed. | SOW-069; `_CONTEXT.md#Description`; `schemas/model_operation.schema.json`; `core/model_operations/validation_preview/engine.py` | Current engine uses `Draft202012Validator` on `schemas/model_operation.schema.json`; focused tests include valid fixture validation and `test_schema_invalid_envelope_cannot_report_schema_passed`. |
| REQ-16-02-003 | The slice must preserve blocking constraint-validation findings before controlled application. | SOW-069; `Dependencies.csv` row `DAG-002-E0828`; `core/model_operations/validation_preview/engine.py` | Current engine accepts injected `constraint_diagnostics` and maps blocking findings to `OP-CONSTRAINT-BLOCKING` with `constraint_validation: blocked`; final constraint-engine API remains TBD pending DEL-13-03. |
| REQ-16-02-004 | The slice must create deterministic diff previews before controlled application for supported operation/change inputs. | SOW-069; `_CONTEXT.md#Description`; `Dependencies.csv` rows `DAG-002-E0829`, `DAG-002-E0830`; `core/model_operations/validation_preview/engine.py` | `test_valid_operation_generates_stable_preview_without_mutating_state` compares canonical JSON for repeated preview output and checks before/after fixture values; final diff payload contract beyond current fixture-backed rows remains TBD. |
| REQ-16-02-005 | Invalid operations must be blocked before application and accepted model state must remain unchanged. | `_CONTEXT.md#Context Envelope`; `execution/_Decomposition/SOFTWARE_DECOMP.md#PKG-16`; `core/model_operations/validation_preview/engine.py` | Current output reports `application_status: not_applied`, `accepted_model_state_unchanged`, blocked preview status for validation failures, and no mutation of the accepted input state. Controlled persistence/application API remains outside this slice. |
| REQ-16-02-006 | Validation and preview outcomes must use structured diagnostics/result-envelope conventions and must not claim certification, sealing, approval, authentication, professional approval, or code compliance. | `execution/_Decomposition/SOFTWARE_DECOMP.md#8`; `docs/SPEC.md#4.3`; `docs/CONTRACT.md#Invariant index`; `schemas/model_operation.schema.json` | Current engine returns structured diagnostics, provenance, and professional-boundary booleans; `test_output_boundary_language_does_not_make_prohibited_claims` and schema tests check prohibited claim boundaries. Final cross-package diagnostic/result-envelope schema mapping remains TBD. |
| REQ-16-02-007 | The implementation must preserve layer/module boundaries: GUI, application services, domain core, schemas, validation, diagnostics, persistence, and tests must not be bypassed by adapters, plugins, or agents. | `execution/_Decomposition/SOFTWARE_DECOMP.md#8`; `docs/SPEC.md#4.4`; `core/model_operations/validation_preview/engine.py` | Current slice is a backend validation/preview module with read-only fixture tests and no GUI, persistence, approval, or application behavior; downstream application/user acceptance remains DEL-16-03. |
| REQ-16-02-008 | The slice must be covered by validation tests aligned with the approved layered testing baseline. | `_CONTEXT.md#Anticipated Artifacts`; `execution/_Decomposition/SOFTWARE_DECOMP.md#8` | Focused tests cover schema failure, constraint failure, canonical dimension blocking, preview determinism, model-basis/current-hash checks, no-apply/no-mutation behavior, and professional-boundary cases. |

## Standards

| Standard or governing source | Applicability | Status |
|---|---|---|
| JSON Schema 2020-12 | Approved schema basis for public schemas/interchange. | Current schema file is `schemas/model_operation.schema.json`; current engine validates it with `Draft202012Validator`. |
| JCS-compatible canonical JSON hashing | Applies where JSON payloads are hashed for reproducibility or acceptance binding. | Current schema/fixture use `accepted_model_state_hash` with `canonicalization: JCS`; current engine compares the operation model-basis hash and operation required current hashes against the accepted model-state hash. Final hash payload scope beyond current model-state basis checks remains TBD. |
| CONTRACT invariants | Applies to no-invention, professional boundary, no hidden defaults, unit/schema checks, and agent output status. | Source-supported from `docs/CONTRACT.md#Invariant index`. |
| DAG-002 active dependency mirror | Provides approved predecessor/evidence surface for this folder. | Source-supported from `Dependencies.csv` and `_DEPENDENCIES.md`; enum normalization is not performed in this workflow by project instruction. |
| External engineering standards | TBD. No accessible source text in this deliverable defines code clauses or engineering values for operation validation. |

## Verification

| Verification target | Required check |
|---|---|
| Schema-validation path | Valid operation fixtures pass schema checks; malformed or schema-incomplete operations fail with structured diagnostics from `schemas/model_operation.schema.json`. |
| Constraint-validation path | Injected blocking constraint findings are preserved as `OP-CONSTRAINT-BLOCKING` and `constraint_validation: blocked`. Exact DEL-13-03 API remains TBD. |
| Diff preview determinism | Same operation and same model basis produce stable canonical JSON output; current rows expose `before`, `after`, `preview_status`, and `application_status: not_applied`. Final DEL-14 payload contract remains TBD. |
| Model-basis/hash checks | Accepted state must expose `physical_source_of_truth` role and current state hash; operation model basis and per-operation required current hashes must match the accepted state hash. |
| Canonical dimension checks | Quantity payloads require value, unit, and dimension metadata; dimensions outside the current canonical vocabulary block unit validation. Deeper target-field dimensional compatibility remains outside this slice. |
| No hidden mutation | Failed validation and preview-only flows do not mutate accepted model state and do not apply operations. Persistence/application behavior outside this preview slice remains TBD. |
| Professional boundary | Outputs are diagnostics, validation outcomes, and previews only; no automatic human approval or code-compliance status is emitted. |
| Dependency fidelity | Approved DAG-002 mirror rows remain ACTIVE and are not retired, deleted, or reclassified by this setup workflow. |

## Documentation

- Operation validator design notes: current implementation is `core/model_operations/validation_preview/engine.py`.
- Diff preview service contract: current deterministic row shape is implemented in `core/model_operations/validation_preview/engine.py`; final contract remains TBD pending DEL-14-03, DEL-14-05, and downstream application/user-acceptance interfaces.
- Validation test plan and fixtures: current focused tests are `tests/test_operation_validation_preview.py` and `tests/test_model_operation_schema.py`; current fixtures are `fixtures/model_operations/invented_operation_set_valid.json` and `fixtures/model_operations/invented_accepted_model_state.json`.
- Diagnostic/result-envelope mapping: current slice emits structured diagnostics/provenance/professional-boundary fields; final cross-package envelope mapping remains TBD.
- Dependency preservation note: existing `Dependencies.csv` remains the approved local DAG-002 mirror/evidence surface.
