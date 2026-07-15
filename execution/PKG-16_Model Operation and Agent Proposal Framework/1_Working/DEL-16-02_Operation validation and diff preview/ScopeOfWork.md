---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-16-02
package_id: PKG-16
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@e8f59a63372f38d9e788ac39b39995558f5aba73
project_scope_refs: [SOW-069]
package_objective_refs: [OBJ-015]
---

# Scope of Work — DEL-16-02

## Purpose and Objective Traceability

This Scope of Work defines `DEL-16-02` in service of project scope [SOW-069] and package objectives [OBJ-015].

- **OUT-001** — A backend operation-validation and deterministic diff-preview contract that blocks invalid structured operations before application and preserves accepted model state is produced.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-16-02 Operation validation and diff preview

> #### Datasheet: DEL-16-02 Operation validation and diff preview
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-002 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-16-02-DECL-002`.
>

### CLM-003 — Identification

> ##### Identification
>
> | Field | Value | Source |
> |---|---|---|
> | Deliverable ID | DEL-16-02 | `_CONTEXT.md` |
> | Name | Operation validation and diff preview | `_CONTEXT.md` |
> | Package | PKG-16 Model Operation and Agent Proposal Framework | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md#PKG-16` |
> | Type | BACKEND_FEATURE_SLICE | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` |
> | Scope item | SOW-069 | `_CONTEXT.md`; `docs/_Registers/ScopeLedger.csv` |
> | Objective support | OBJ-015 | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md#Objectives` |
> | Context envelope | M | `_CONTEXT.md`; `docs/_Registers/ContextBudgetQA.csv` |
>

### CLM-004 — Attributes

> ##### Attributes
>
> | Attribute | Current value |
> |---|---|
> | Primary function | Run schema validation, constraint validation, and deterministic diff preview before model operations are applied. Source: `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md#PKG-16`. |
> | Anticipated artifacts | Operation validator; diff preview service; validation tests. Source: `_CONTEXT.md`; `docs/_Registers/Deliverables.csv`. |
> | Operation input boundary | Structured model operation schema from DEL-16-01 is an approved upstream dependency. Source: `Dependencies.csv` rows `DAG-002-E0827`; `execution/_DAG/DAG-006/DAG-002_EdgeDispositionReview.md#DAG2-RD-012`. |
> | Constraint input boundary | Constraint validation engine DEL-13-03 is an approved upstream dependency. Source: `Dependencies.csv` row `DAG-002-E0828`. |
> | Diff input boundary | Model-state comparison engine DEL-14-03 and comparison mapping/tolerance/export contracts DEL-14-05 are approved upstream dependencies. Source: `Dependencies.csv` rows `DAG-002-E0829` and `DAG-002-E0830`. |
> | Diagnostics input boundary | Solver diagnostics and singularity detection DEL-04-06 is an approved upstream dependency. Source: `Dependencies.csv` row `DAG-002-E0831`. |
> | Architecture basis | Rust core/application services, schema-first envelopes, JSON Schema 2020-12, JCS-compatible hash basis where JSON payloads are hashed, and layered test gates are dispatchable context constraints. Source: `_CONTEXT.md#Architecture Basis Injection`; `execution/_Decomposition/SOFTWARE_DECOMP.md#8`. |
> | Implementation location | `core/model_operations/validation_preview/engine.py` implements the current validation and deterministic preview slice; focused evidence is in `tests/test_operation_validation_preview.py`, `tests/test_model_operation_schema.py`, `schemas/model_operation.schema.json`, and `fixtures/model_operations/`. |
>

### CLM-005 — Conditions

> ##### Conditions
>
> | Condition | Data state |
> |---|---|
> | Mutation boundary | GUI and agent edits are structured model operations and must not mutate accepted engineering state directly. Source: SOW-069 in `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md#Scope Ledger`. |
> | Invalid operation behavior | Invalid operations are blocked before application. Source: `_CONTEXT.md#Context Envelope`; `execution/_Decomposition/SOFTWARE_DECOMP.md#PKG-16`. |
> | Professional boundary | The package excludes hidden model mutations and autonomous engineering acceptance. Source: `_CONTEXT.md#Package Reference`; `docs/CONTRACT.md#Invariant index`. |
> | Diagnostics and result envelopes | Diagnostics/result envelopes must preserve source, severity/class, affected object, message/remediation, provenance, and no certification/compliance claims where applicable. Source: `execution/_Decomposition/SOFTWARE_DECOMP.md#8`; `docs/SPEC.md#4.3`. |
> | Current validation flow | The current engine validates required envelope fields, runs `Draft202012Validator` against `schemas/model_operation.schema.json`, checks accepted model-state basis/hash and operation current hashes, imports blocking constraint diagnostics, then emits either generated preview rows or blocked preview rows with `application_status: not_applied`. |
> | Current diff preview shape | The current deterministic fixture-backed preview rows include `operation_id`, `change_id`, `change_kind`, `target_ref`, `preview_status`, `before`, `after`, and `application_status`. Final diff payload contract beyond this slice remains TBD pending DEL-14-03/DEL-14-05 and later application contracts. |
> | Current operation schema boundary | `schemas/model_operation.schema.json` is a JSON Schema 2020-12 contract for DEL-16-01 operation envelopes, with structured-operations-only mutation route, `direct_model_mutation_allowed: false`, downstream user-acceptance/audit bindings, operation/change taxonomies, required model basis/current hashes, unit requirements, diagnostics, provenance, and professional-boundary fields. Final upstream ownership remains DEL-16-01. |
> | Canonical dimension check | Current validation blocks quantity payload dimensions outside the accepted canonical dimension vocabulary exposed by the engine/tests and schema checks. Deeper target-field dimensional compatibility remains outside this slice. |
> | Direct mutation blocking | Current validation blocks direct accepted-model mutation signals such as applied operation validation status or forbidden auto-accepted operation statuses; output still reports `application_status: not_applied`. |
>

### CLM-006 — Construction

> ##### Construction
>
> | Construct | Expected role | Status |
> |---|---|---|
> | Operation validator | Accepts or rejects proposed structured model operations using required envelope checks, JSON Schema 2020-12 validation, model-basis/current-hash checks, canonical dimension checks, target-reference checks, direct-mutation blocking, and injected blocking constraint diagnostics before any application. | Implemented in `core/model_operations/validation_preview/engine.py`; final constraint-engine API integration remains TBD. |
> | Diff preview service | Produces deterministic before/after preview rows for supported change kinds and blocked preview rows when operation-local validation blocks preview. | Implemented in current engine and fixture tests; final diff payload contract beyond current deterministic evidence remains TBD. |
> | Validation tests | Exercise stable preview/no mutation, missing unit metadata, unknown dimensions, unresolved targets, blocking constraint diagnostics, direct mutation rejection, JSON Schema failure, model-role/current-hash checks, and prohibited-claim boundary. | Focused tests exist in `tests/test_operation_validation_preview.py`; schema contract checks exist in `tests/test_model_operation_schema.py`. |
> | Result/diagnostic envelope integration | Reports validation statuses, sorted diagnostics, accepted model-state reference/hash, `accepted_model_state_unchanged`, `professional_boundary`, and provenance without approval or compliance claims. | Implemented for this slice; final cross-package diagnostic/result-envelope schema mapping remains TBD. |
>

### CLM-007 — References

> ##### References
>
> - `_CONTEXT.md` - deliverable identity, scope, objective, package reference, architecture-basis injection.
> - `_REFERENCES.md` - source inventory for this deliverable.
> - `Dependencies.csv` - approved DAG-006 local mirror/evidence surface for active upstream dependencies.
> - `execution/_Decomposition/SOFTWARE_DECOMP.md` - accepted revision 0.7 scope, package, deliverable, objective, and architecture-basis context.
> - `docs/_Registers/Deliverables.csv` - deliverable identity and anticipated artifacts.
> - `docs/_Registers/ScopeLedger.csv` - SOW-069 wording and product-boundary notes.
> - `docs/_Registers/ContextBudgetQA.csv` - context-envelope row.
> - `docs/CONTRACT.md` - invariants for data, authority, units, diagnostics, and agent behavior.
> - `docs/SPEC.md` - schema-first, persistence, diagnostics, result-envelope, and professional-boundary context.

## Completion and Reliance Basis — Epistemology

### CLM-008 — Specification: DEL-16-02 Operation validation and diff preview

> #### Specification: DEL-16-02 Operation validation and diff preview
>

### CLM-009 — Scope

> ##### Scope
>
> This deliverable covers the backend feature slice that validates proposed structured model operations and creates deterministic diff previews before those operations are applied. It is grounded in SOW-069 and OBJ-015. Source: `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md#PKG-16`; `docs/_Registers/ScopeLedger.csv`.
>
> This deliverable excludes hidden model mutations, autonomous engineering acceptance, product claims of professional approval or code compliance, and ownership of the structured operation schema itself. The operation schema is upstream DEL-16-01. Source: `_CONTEXT.md#Package Reference`; `Dependencies.csv` row `DAG-002-E0827`; `docs/CONTRACT.md#Invariant index`.
>

### CLM-010 — Requirements

> ##### Requirements
>
> | ID | Requirement | Source | Verification |
> |---|---|---|---|
> | REQ-16-02-001 | Proposed GUI and agent edits must enter this slice as structured model operations, not as direct hidden mutations of accepted model state. | SOW-069 in `_CONTEXT.md`; `docs/_Registers/ScopeLedger.csv`; `execution/_Decomposition/SOFTWARE_DECOMP.md#PKG-16`; `schemas/model_operation.schema.json` | `tests/test_model_operation_schema.py` checks structured-operations-only and direct mutation forbidden schema constants; `tests/test_operation_validation_preview.py::test_direct_mutation_request_is_rejected` checks current engine blocking behavior. |
> | REQ-16-02-002 | The slice must perform JSON Schema 2020-12 validation against the DEL-16-01 operation schema before treating an operation as schema-passed. | SOW-069; `_CONTEXT.md#Description`; `schemas/model_operation.schema.json`; `core/model_operations/validation_preview/engine.py` | Current engine uses `Draft202012Validator` on `schemas/model_operation.schema.json`; focused tests include valid fixture validation and `test_schema_invalid_envelope_cannot_report_schema_passed`. |
> | REQ-16-02-003 | The slice must preserve blocking constraint-validation findings before controlled application. | SOW-069; `Dependencies.csv` row `DAG-002-E0828`; `core/model_operations/validation_preview/engine.py` | Current engine accepts injected `constraint_diagnostics` and maps blocking findings to `OP-CONSTRAINT-BLOCKING` with `constraint_validation: blocked`; final constraint-engine API remains TBD pending DEL-13-03. |
> | REQ-16-02-004 | The slice must create deterministic diff previews before controlled application for supported operation/change inputs. | SOW-069; `_CONTEXT.md#Description`; `Dependencies.csv` rows `DAG-002-E0829`, `DAG-002-E0830`; `core/model_operations/validation_preview/engine.py` | `test_valid_operation_generates_stable_preview_without_mutating_state` compares canonical JSON for repeated preview output and checks before/after fixture values; final diff payload contract beyond current fixture-backed rows remains TBD. |
> | REQ-16-02-005 | Invalid operations must be blocked before application and accepted model state must remain unchanged. | `_CONTEXT.md#Context Envelope`; `execution/_Decomposition/SOFTWARE_DECOMP.md#PKG-16`; `core/model_operations/validation_preview/engine.py` | Current output reports `application_status: not_applied`, `accepted_model_state_unchanged`, blocked preview status for validation failures, and no mutation of the accepted input state. Controlled persistence/application API remains outside this slice. |
> | REQ-16-02-006 | Validation and preview outcomes must use structured diagnostics/result-envelope conventions and must not claim certification, sealing, approval, authentication, professional approval, or code compliance. | `execution/_Decomposition/SOFTWARE_DECOMP.md#8`; `docs/SPEC.md#4.3`; `docs/CONTRACT.md#Invariant index`; `schemas/model_operation.schema.json` | Current engine returns structured diagnostics, provenance, and professional-boundary booleans; `test_output_boundary_language_does_not_make_prohibited_claims` and schema tests check prohibited claim boundaries. Final cross-package diagnostic/result-envelope schema mapping remains TBD. |
> | REQ-16-02-007 | The implementation must preserve layer/module boundaries: GUI, application services, domain core, schemas, validation, diagnostics, persistence, and tests must not be bypassed by adapters, plugins, or agents. | `execution/_Decomposition/SOFTWARE_DECOMP.md#8`; `docs/SPEC.md#4.4`; `core/model_operations/validation_preview/engine.py` | Current slice is a backend validation/preview module with read-only fixture tests and no GUI, persistence, approval, or application behavior; downstream application/user acceptance remains DEL-16-03. |
> | REQ-16-02-008 | The slice must be covered by validation tests aligned with the approved layered testing baseline. | `_CONTEXT.md#Anticipated Artifacts`; `execution/_Decomposition/SOFTWARE_DECOMP.md#8` | Focused tests cover schema failure, constraint failure, canonical dimension blocking, preview determinism, model-basis/current-hash checks, no-apply/no-mutation behavior, and professional-boundary cases. |
>

### CLM-011 — Standards

> ##### Standards
>
> | Standard or governing source | Applicability | Status |
> |---|---|---|
> | JSON Schema 2020-12 | Approved schema basis for public schemas/interchange. | Current schema file is `schemas/model_operation.schema.json`; current engine validates it with `Draft202012Validator`. |
> | JCS-compatible canonical JSON hashing | Applies where JSON payloads are hashed for reproducibility or acceptance binding. | Current schema/fixture use `accepted_model_state_hash` with `canonicalization: JCS`; current engine compares the operation model-basis hash and operation required current hashes against the accepted model-state hash. Final hash payload scope beyond current model-state basis checks remains TBD. |
> | CONTRACT invariants | Applies to no-invention, professional boundary, no hidden defaults, unit/schema checks, and agent output status. | Source-supported from `docs/CONTRACT.md#Invariant index`. |
> | DAG-002 active dependency mirror | Provides approved predecessor/evidence surface for this folder. | Source-supported from `Dependencies.csv` and `_DEPENDENCIES.md`; enum normalization is not performed in this workflow by project instruction. |
> | External engineering standards | TBD. No accessible source text in this deliverable defines code clauses or engineering values for operation validation. |
>

### CLM-012 — Verification

> ##### Verification
>
> | Verification target | Required check |
> |---|---|
> | Schema-validation path | Valid operation fixtures pass schema checks; malformed or schema-incomplete operations fail with structured diagnostics from `schemas/model_operation.schema.json`. |
> | Constraint-validation path | Injected blocking constraint findings are preserved as `OP-CONSTRAINT-BLOCKING` and `constraint_validation: blocked`. Exact DEL-13-03 API remains TBD. |
> | Diff preview determinism | Same operation and same model basis produce stable canonical JSON output; current rows expose `before`, `after`, `preview_status`, and `application_status: not_applied`. Final DEL-14 payload contract remains TBD. |
> | Model-basis/hash checks | Accepted state must expose `physical_source_of_truth` role and current state hash; operation model basis and per-operation required current hashes must match the accepted state hash. |
> | Canonical dimension checks | Quantity payloads require value, unit, and dimension metadata; dimensions outside the current canonical vocabulary block unit validation. Deeper target-field dimensional compatibility remains outside this slice. |
> | No hidden mutation | Failed validation and preview-only flows do not mutate accepted model state and do not apply operations. Persistence/application behavior outside this preview slice remains TBD. |
> | Professional boundary | Outputs are diagnostics, validation outcomes, and previews only; no automatic human approval or code-compliance status is emitted. |
> | Dependency fidelity | Approved DAG-002 mirror rows remain ACTIVE and are not retired, deleted, or reclassified by this setup workflow. |
>

### CLM-013 — Documentation

> ##### Documentation
>
> - Operation validator design notes: current implementation is `core/model_operations/validation_preview/engine.py`.
> - Diff preview service contract: current deterministic row shape is implemented in `core/model_operations/validation_preview/engine.py`; final contract remains TBD pending DEL-14-03, DEL-14-05, and downstream application/user-acceptance interfaces.
> - Validation test plan and fixtures: current focused tests are `tests/test_operation_validation_preview.py` and `tests/test_model_operation_schema.py`; current fixtures are `fixtures/model_operations/invented_operation_set_valid.json` and `fixtures/model_operations/invented_accepted_model_state.json`.
> - Diagnostic/result-envelope mapping: current slice emits structured diagnostics/provenance/professional-boundary fields; final cross-package envelope mapping remains TBD.
> - Dependency preservation note: existing `Dependencies.csv` remains the approved local DAG-002 mirror/evidence surface.

- **AC-001** — The contract preserves JSON Schema 2020-12 validation, model-basis and current-hash preconditions, canonical dimension and target-reference checks, injected blocking constraint diagnostics, direct-mutation rejection, deterministic before/after preview rows, not-applied and unchanged-state evidence, structured diagnostics/provenance, upstream and downstream ownership boundaries, invented fixture evidence, professional non-authority, and explicit TBDs for final constraint APIs, diff/tolerance payloads, persistence/application, and human dispositions.

## Production and Verification Method — Praxeology

### CLM-014 — Procedure: DEL-16-02 Operation validation and diff preview

> #### Procedure: DEL-16-02 Operation validation and diff preview
>

### CLM-015 — Purpose

> ##### Purpose
>
> This procedure describes the current conservative workflow for using the DEL-16-02 validation-preview slice and retaining evidence. It is aligned with `core/model_operations/validation_preview/engine.py`, `schemas/model_operation.schema.json`, focused fixtures, and focused tests. It does not define downstream persistence, application, or human-acceptance behavior.
>

### CLM-016 — Prerequisites

> ##### Prerequisites
>
> | Prerequisite | Source | Status |
> |---|---|---|
> | DEL-16-01 Structured model operation schema | `Dependencies.csv` row `DAG-002-E0827`; `schemas/model_operation.schema.json` | Approved upstream dependency; current JSON Schema 2020-12 file and fixture evidence are available; final schema ownership remains upstream DEL-16-01. |
> | DEL-13-03 Constraint validation engine | `Dependencies.csv` row `DAG-002-E0828` | Approved upstream dependency; current slice consumes injected blocking constraint diagnostics; exact API remains TBD. |
> | DEL-14-03 Model-state comparison engine | `Dependencies.csv` row `DAG-002-E0829` | Approved upstream dependency; current engine produces deterministic before/after preview rows from accepted model-state fixture evidence; final integration remains TBD. |
> | DEL-14-05 Comparison mapping, tolerance, and export contracts | `Dependencies.csv` row `DAG-002-E0830` | Approved upstream dependency; default tolerances TBD. |
> | DEL-04-06 Solver diagnostics and singularity detection | `Dependencies.csv` row `DAG-002-E0831` | Approved upstream dependency; current slice emits structured diagnostics/provenance, while final cross-package diagnostic mapping remains TBD. |
> | Architecture basis AB-00-01, AB-00-02, AB-00-03, AB-00-04, AB-00-06, AB-00-07, AB-00-08 | `_CONTEXT.md#Architecture Basis Injection` | Dispatchable context constraints; not implementation evidence. |
>

### CLM-017 — Steps

> ##### Steps
>
> 1. Confirm the proposed edit is represented as a structured model operation.
>    - Use `schemas/model_operation.schema.json` for the current structured operation contract.
>    - Preserve DEL-16-01 as the owner of final schema semantics.
>    - Source: SOW-069; DEL-16-01 dependency; `tests/test_model_operation_schema.py`.
>
> 2. Run schema validation against the approved operation schema.
>    - The current engine uses `Draft202012Validator` against `schemas/model_operation.schema.json`.
>    - Accept schema-conformant operation envelopes for later checks.
>    - Return `OP-SCHEMA-VALIDATION-FAILED` diagnostics and `schema_validation: blocked` for schema failures.
>
> 3. Check model-basis and operation-current-hash preconditions.
>    - Require accepted model state to be bound to `physical_source_of_truth`.
>    - Require accepted model state to expose a current hash.
>    - Require operation model basis and operation `required_current_hashes` to match the accepted model-state hash.
>    - Emit blocking precondition diagnostics for missing or stale basis/hash evidence.
>
> 4. Check operation/change-level blockers.
>    - Block unsupported operation or change taxonomies.
>    - Block direct mutation requests.
>    - Block missing quantity unit metadata and dimensions outside the current canonical vocabulary.
>    - Block unresolved target references for change kinds that require existing accepted-state targets.
>
> 5. Incorporate constraint validation results.
>    - Pass current `constraint_diagnostics` into the validation-preview call when available.
>    - Blocking constraint findings are preserved as `OP-CONSTRAINT-BLOCKING` and `constraint_validation: blocked`.
>    - Exact constraint engine interface remains TBD.
>
> 6. Generate deterministic diff preview output before application.
>    - Use the current accepted model-state snapshot and operation envelope.
>    - Current preview rows include `operation_id`, `change_id`, `change_kind`, `target_ref`, `preview_status`, `before`, `after`, and `application_status`.
>    - Keep final preview payload fields and tolerance defaults TBD until upstream/downstream contracts define them.
>
> 7. Block invalid operations before application.
>    - If schema or constraint validation fails, do not hand the operation to the controlled application path.
>    - Current output always preserves `application_status: not_applied` and reports whether accepted model state was unchanged.
>    - Preserve diagnostics and preview status without professional approval or code-compliance claims.
>
> 8. Hand valid, previewed operations to the later controlled application/user-acceptance workflow.
>    - This deliverable does not implement hidden mutation or autonomous engineering acceptance.
>    - User acceptance and operation audit trail are owned by DEL-16-03.
>

### CLM-018 — Verification

> ##### Verification
>
> | Check | Evidence to retain |
> |---|---|
> | Structured-operation input check | `tests/test_model_operation_schema.py` schema constants and `tests/test_operation_validation_preview.py::test_direct_mutation_request_is_rejected`. |
> | Schema validation check | `tests/test_model_operation_schema.py`; `tests/test_operation_validation_preview.py::test_schema_invalid_envelope_cannot_report_schema_passed`. |
> | Constraint validation check | `tests/test_operation_validation_preview.py::test_unresolved_target_and_constraint_findings_are_blocking`. |
> | Diff determinism check | `tests/test_operation_validation_preview.py::test_valid_operation_generates_stable_preview_without_mutating_state`. |
> | Model-basis/hash check | `tests/test_operation_validation_preview.py::test_model_role_and_current_hash_are_required_before_preview`. |
> | Canonical dimension check | `tests/test_operation_validation_preview.py::test_missing_unit_metadata_blocks_preview` and `tests/test_operation_validation_preview.py::test_unknown_dimension_blocks_unit_validation`; `tests/test_model_operation_schema.py` canonical dimension assertions. |
> | No-apply-on-invalid check | Current validation result keeps `application_status: not_applied`; focused tests check direct mutation blocking and accepted-state non-mutation. |
> | Professional-boundary check | `tests/test_operation_validation_preview.py::test_output_boundary_language_does_not_make_prohibited_claims`; `tests/test_model_operation_schema.py` professional-boundary constants. |
>

### CLM-019 — Records

> ##### Records
>
> - Operation validator implementation notes: `core/model_operations/validation_preview/engine.py`.
> - Diff preview service contract: current deterministic fixture-backed row shape is implemented in `core/model_operations/validation_preview/engine.py`; final contract remains TBD.
> - Validation test cases and fixture inventory: `tests/test_operation_validation_preview.py`, `tests/test_model_operation_schema.py`, `fixtures/model_operations/invented_operation_set_valid.json`, and `fixtures/model_operations/invented_accepted_model_state.json`.
> - Diagnostic/result-envelope mapping notes: current structured diagnostics/provenance/professional-boundary output exists; final cross-package mapping remains TBD.
> - Dependency preservation record: existing `Dependencies.csv` approved DAG-006 rows remain ACTIVE and unchanged by this setup workflow.

- **VER-001** — Validate the contract and review source parity, schema/constraint/unit/hash/target/direct-mutation blocking paths, deterministic preview shape and no-application behavior, accepted-state nonmutation, dependency and layer boundaries, structured diagnostics/provenance, retained downstream API and tolerance TBDs, and absence of hidden mutation, protected defaults, or professional-authority claims.

## Governing Values and Decisions — Axiology

### CLM-020 — Guidance: DEL-16-02 Operation validation and diff preview

> #### Guidance: DEL-16-02 Operation validation and diff preview
>

### CLM-021 — Purpose

> ##### Purpose
>
> This deliverable exists to make proposed model edits reviewable and blockable before they become accepted model changes. It supports OBJ-015 by turning GUI and agent changes into validated, previewed, auditable model-operation flows. Source: `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md#Objectives`; `docs/_Registers/ScopeLedger.csv`.
>

### CLM-022 — Principles

> ##### Principles
>
> | Principle | Guidance |
> |---|---|
> | Treat operations as the mutation boundary | All GUI and agent edits should be represented as structured operations before application. Do not design a bypass path for direct accepted-state mutation. Source: SOW-069; `_CONTEXT.md#Package Reference`. |
> | Validate before applying | The current engine performs required-envelope checks, JSON Schema 2020-12 validation against `schemas/model_operation.schema.json`, model-basis/current-hash checks, injected blocking constraint-diagnostic handling, operation/change checks, and preview status generation before any application status can leave `not_applied`. Source: SOW-069; `_CONTEXT.md#Description`; `core/model_operations/validation_preview/engine.py`. |
> | Preview deterministically | Current preview output is stable for the same operation and accepted model basis using canonical JSON comparison in focused tests. Preview rows include before/after state snippets and remain `application_status: not_applied`. Final comparison payload and tolerance contract beyond current fixture evidence remains TBD. |
> | Preserve diagnostics | Validation failures and warnings should remain structured and provenance-aware. Source: architecture basis AB-00-06 in `_CONTEXT.md`; `docs/SPEC.md#4.3`. |
> | Preserve professional boundaries | Validation and preview results are decision-support/control-surface outputs, not professional approval or code-compliance claims. Source: `docs/CONTRACT.md#Invariant index`; `docs/SPEC.md#4.3`. |
> | Prefer explicit TBDs | Current schema path, engine path, fixture inventory, and focused test behavior are established. Final constraint-engine API, final diff payload/tolerance contract, persistence/application behavior, and human review dispositions remain `TBD` rather than inferred. Source: `docs/CONTRACT.md#Invariant index`. |
>

### CLM-023 — Considerations

> ##### Considerations
>
> - DEL-16-02 depends on DEL-16-01 for structured operation schema semantics. Current validation consumes `schemas/model_operation.schema.json` and `fixtures/model_operations/invented_operation_set_valid.json`; final ownership of operation schema semantics remains upstream DEL-16-01. Source: `Dependencies.csv` row `DAG-002-E0827`.
> - DEL-16-02 depends on DEL-13-03 for constraint validation. Current validation accepts injected blocking constraint diagnostics and maps them to structured blocking diagnostics; final constraint engine API remains TBD. Source: `Dependencies.csv` row `DAG-002-E0828`; `execution/_Decomposition/SOFTWARE_DECOMP.md#Scope Ledger`; `tests/test_operation_validation_preview.py`.
> - DEL-16-02 depends on DEL-14-03 and DEL-14-05 for state comparison and mapping/tolerance/export contract context. Preview behavior should remain deterministic and diagnostic, not automatic external validation. Source: `Dependencies.csv` rows `DAG-002-E0829` and `DAG-002-E0830`; SOW-073 notes in decomposition.
> - DEL-16-02 depends on DEL-04-06 for diagnostics/warning contract context. Validation and preview failures should preserve diagnostic classes and provenance where the relevant contract is available. Source: `Dependencies.csv` row `DAG-002-E0831`.
> - Approved architecture-basis rows from PKG-00 are dispatchable context evidence, not Type 2 implementation authority by themselves. Source: `_CONTEXT.md#Architecture Basis Injection`; `_DEPENDENCIES.md#Authority Boundary`.
>

### CLM-024 — Trade-offs

> ##### Trade-offs
>
> | Trade-off | Conservative handling |
> |---|---|
> | Early blocking vs richer preview | Current engine returns blocked preview rows for operation-local blocking diagnostics and reports global `diff_preview_status: blocked_by_validation` when any blocking diagnostic exists. It does not apply operations. Final UX/application handling of partial previews remains downstream. |
> | Deterministic preview vs tolerance defaults | Current deterministic preview evidence compares canonical JSON for fixture output and does not define engineering comparison tolerances. Keep default tolerances TBD unless DEL-14-05 or a human product ruling defines them. |
> | Agent convenience vs accepted-state safety | Agents may propose operations but do not mutate accepted engineering state directly. Favor an explicit review/apply boundary over convenience shortcuts. |
> | Validation detail vs protected-data boundary | Do not encode protected standards text, proprietary values, or code-specific defaults in validation logic. Missing required values become explicit findings. |
>

### CLM-025 — Examples

> ##### Examples
>
> Current invented-public examples and focused evidence are:
>
> - `schemas/model_operation.schema.json` - JSON Schema 2020-12 structured model operation contract.
> - `fixtures/model_operations/invented_operation_set_valid.json` - invented operation-set fixture covering operation/change taxonomy, required current hashes, professional boundary, and model basis.
> - `fixtures/model_operations/invented_accepted_model_state.json` - invented accepted model-state fixture with `physical_source_of_truth` role and current state hash.
> - `tests/test_operation_validation_preview.py` - focused validation/preview tests for determinism, schema blocking, units/dimensions, constraints, direct mutation, model role/current hash, no mutation, and prohibited-claim boundaries.
> - `tests/test_model_operation_schema.py` - schema contract checks for required fields, taxonomies, structured-only mutation route, current hashes, canonical dimensions, and professional-boundary constants.
>
> These examples are implementation evidence for this slice, not final project data or professional validation examples.
>

### CLM-026 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
> |---|---|---|---|---|---|---|
> | None | No source conflict identified in the accessible slices. Current implementation evidence resolves earlier generic implementation-path/schema-path/test-fixture TBDs, while downstream APIs, final diff contract, persistence/application behavior, and human dispositions remain TBD. | N/A | N/A | N/A | N/A | N/A |

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-069 OBJ-015 | CLM-008 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
