# Datasheet: DEL-16-02 Operation validation and diff preview

<!-- D41-R5-T7-PDU055-CURRENTNESS -->
## D-41 R5 T7 PDU-055 current declaration

Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.

Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.

PDU-055 cited claim(s): `DEL-16-02-DECL-002`.

## Identification

| Field | Value | Source |
|---|---|---|
| Deliverable ID | DEL-16-02 | `_CONTEXT.md` |
| Name | Operation validation and diff preview | `_CONTEXT.md` |
| Package | PKG-16 Model Operation and Agent Proposal Framework | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md#PKG-16` |
| Type | BACKEND_FEATURE_SLICE | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` |
| Scope item | SOW-069 | `_CONTEXT.md`; `docs/_Registers/ScopeLedger.csv` |
| Objective support | OBJ-015 | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md#Objectives` |
| Context envelope | M | `_CONTEXT.md`; `docs/_Registers/ContextBudgetQA.csv` |

## Attributes

| Attribute | Current value |
|---|---|
| Primary function | Run schema validation, constraint validation, and deterministic diff preview before model operations are applied. Source: `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md#PKG-16`. |
| Anticipated artifacts | Operation validator; diff preview service; validation tests. Source: `_CONTEXT.md`; `docs/_Registers/Deliverables.csv`. |
| Operation input boundary | Structured model operation schema from DEL-16-01 is an approved upstream dependency. Source: `Dependencies.csv` rows `DAG-002-E0827`; `execution/_DAG/DAG-006/DAG-002_EdgeDispositionReview.md#DAG2-RD-012`. |
| Constraint input boundary | Constraint validation engine DEL-13-03 is an approved upstream dependency. Source: `Dependencies.csv` row `DAG-002-E0828`. |
| Diff input boundary | Model-state comparison engine DEL-14-03 and comparison mapping/tolerance/export contracts DEL-14-05 are approved upstream dependencies. Source: `Dependencies.csv` rows `DAG-002-E0829` and `DAG-002-E0830`. |
| Diagnostics input boundary | Solver diagnostics and singularity detection DEL-04-06 is an approved upstream dependency. Source: `Dependencies.csv` row `DAG-002-E0831`. |
| Architecture basis | Rust core/application services, schema-first envelopes, JSON Schema 2020-12, JCS-compatible hash basis where JSON payloads are hashed, and layered test gates are dispatchable context constraints. Source: `_CONTEXT.md#Architecture Basis Injection`; `execution/_Decomposition/SOFTWARE_DECOMP.md#8`. |
| Implementation location | `core/model_operations/validation_preview/engine.py` implements the current validation and deterministic preview slice; focused evidence is in `tests/test_operation_validation_preview.py`, `tests/test_model_operation_schema.py`, `schemas/model_operation.schema.json`, and `fixtures/model_operations/`. |

## Conditions

| Condition | Data state |
|---|---|
| Mutation boundary | GUI and agent edits are structured model operations and must not mutate accepted engineering state directly. Source: SOW-069 in `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md#Scope Ledger`. |
| Invalid operation behavior | Invalid operations are blocked before application. Source: `_CONTEXT.md#Context Envelope`; `execution/_Decomposition/SOFTWARE_DECOMP.md#PKG-16`. |
| Professional boundary | The package excludes hidden model mutations and autonomous engineering acceptance. Source: `_CONTEXT.md#Package Reference`; `docs/CONTRACT.md#Invariant index`. |
| Diagnostics and result envelopes | Diagnostics/result envelopes must preserve source, severity/class, affected object, message/remediation, provenance, and no certification/compliance claims where applicable. Source: `execution/_Decomposition/SOFTWARE_DECOMP.md#8`; `docs/SPEC.md#4.3`. |
| Current validation flow | The current engine validates required envelope fields, runs `Draft202012Validator` against `schemas/model_operation.schema.json`, checks accepted model-state basis/hash and operation current hashes, imports blocking constraint diagnostics, then emits either generated preview rows or blocked preview rows with `application_status: not_applied`. |
| Current diff preview shape | The current deterministic fixture-backed preview rows include `operation_id`, `change_id`, `change_kind`, `target_ref`, `preview_status`, `before`, `after`, and `application_status`. Final diff payload contract beyond this slice remains TBD pending DEL-14-03/DEL-14-05 and later application contracts. |
| Current operation schema boundary | `schemas/model_operation.schema.json` is a JSON Schema 2020-12 contract for DEL-16-01 operation envelopes, with structured-operations-only mutation route, `direct_model_mutation_allowed: false`, downstream user-acceptance/audit bindings, operation/change taxonomies, required model basis/current hashes, unit requirements, diagnostics, provenance, and professional-boundary fields. Final upstream ownership remains DEL-16-01. |
| Canonical dimension check | Current validation blocks quantity payload dimensions outside the accepted canonical dimension vocabulary exposed by the engine/tests and schema checks. Deeper target-field dimensional compatibility remains outside this slice. |
| Direct mutation blocking | Current validation blocks direct accepted-model mutation signals such as applied operation validation status or forbidden auto-accepted operation statuses; output still reports `application_status: not_applied`. |

## Construction

| Construct | Expected role | Status |
|---|---|---|
| Operation validator | Accepts or rejects proposed structured model operations using required envelope checks, JSON Schema 2020-12 validation, model-basis/current-hash checks, canonical dimension checks, target-reference checks, direct-mutation blocking, and injected blocking constraint diagnostics before any application. | Implemented in `core/model_operations/validation_preview/engine.py`; final constraint-engine API integration remains TBD. |
| Diff preview service | Produces deterministic before/after preview rows for supported change kinds and blocked preview rows when operation-local validation blocks preview. | Implemented in current engine and fixture tests; final diff payload contract beyond current deterministic evidence remains TBD. |
| Validation tests | Exercise stable preview/no mutation, missing unit metadata, unknown dimensions, unresolved targets, blocking constraint diagnostics, direct mutation rejection, JSON Schema failure, model-role/current-hash checks, and prohibited-claim boundary. | Focused tests exist in `tests/test_operation_validation_preview.py`; schema contract checks exist in `tests/test_model_operation_schema.py`. |
| Result/diagnostic envelope integration | Reports validation statuses, sorted diagnostics, accepted model-state reference/hash, `accepted_model_state_unchanged`, `professional_boundary`, and provenance without approval or compliance claims. | Implemented for this slice; final cross-package diagnostic/result-envelope schema mapping remains TBD. |

## References

- `_CONTEXT.md` - deliverable identity, scope, objective, package reference, architecture-basis injection.
- `_REFERENCES.md` - source inventory for this deliverable.
- `Dependencies.csv` - approved DAG-006 local mirror/evidence surface for active upstream dependencies.
- `execution/_Decomposition/SOFTWARE_DECOMP.md` - accepted revision 0.7 scope, package, deliverable, objective, and architecture-basis context.
- `docs/_Registers/Deliverables.csv` - deliverable identity and anticipated artifacts.
- `docs/_Registers/ScopeLedger.csv` - SOW-069 wording and product-boundary notes.
- `docs/_Registers/ContextBudgetQA.csv` - context-envelope row.
- `docs/CONTRACT.md` - invariants for data, authority, units, diagnostics, and agent behavior.
- `docs/SPEC.md` - schema-first, persistence, diagnostics, result-envelope, and professional-boundary context.
