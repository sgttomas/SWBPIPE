# Guidance: DEL-16-02 Operation validation and diff preview

## Purpose

This deliverable exists to make proposed model edits reviewable and blockable before they become accepted model changes. It supports OBJ-015 by turning GUI and agent changes into validated, previewed, auditable model-operation flows. Source: `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md#Objectives`; `docs/_Registers/ScopeLedger.csv`.

## Principles

| Principle | Guidance |
|---|---|
| Treat operations as the mutation boundary | All GUI and agent edits should be represented as structured operations before application. Do not design a bypass path for direct accepted-state mutation. Source: SOW-069; `_CONTEXT.md#Package Reference`. |
| Validate before applying | The current engine performs required-envelope checks, JSON Schema 2020-12 validation against `schemas/model_operation.schema.json`, model-basis/current-hash checks, injected blocking constraint-diagnostic handling, operation/change checks, and preview status generation before any application status can leave `not_applied`. Source: SOW-069; `_CONTEXT.md#Description`; `core/model_operations/validation_preview/engine.py`. |
| Preview deterministically | Current preview output is stable for the same operation and accepted model basis using canonical JSON comparison in focused tests. Preview rows include before/after state snippets and remain `application_status: not_applied`. Final comparison payload and tolerance contract beyond current fixture evidence remains TBD. |
| Preserve diagnostics | Validation failures and warnings should remain structured and provenance-aware. Source: architecture basis AB-00-06 in `_CONTEXT.md`; `docs/SPEC.md#4.3`. |
| Preserve professional boundaries | Validation and preview results are decision-support/control-surface outputs, not professional approval or code-compliance claims. Source: `docs/CONTRACT.md#Invariant index`; `docs/SPEC.md#4.3`. |
| Prefer explicit TBDs | Current schema path, engine path, fixture inventory, and focused test behavior are established. Final constraint-engine API, final diff payload/tolerance contract, persistence/application behavior, and human review dispositions remain `TBD` rather than inferred. Source: `docs/CONTRACT.md#Invariant index`. |

## Considerations

- DEL-16-02 depends on DEL-16-01 for structured operation schema semantics. Current validation consumes `schemas/model_operation.schema.json` and `fixtures/model_operations/invented_operation_set_valid.json`; final ownership of operation schema semantics remains upstream DEL-16-01. Source: `Dependencies.csv` row `DAG-002-E0827`.
- DEL-16-02 depends on DEL-13-03 for constraint validation. Current validation accepts injected blocking constraint diagnostics and maps them to structured blocking diagnostics; final constraint engine API remains TBD. Source: `Dependencies.csv` row `DAG-002-E0828`; `execution/_Decomposition/SOFTWARE_DECOMP.md#Scope Ledger`; `tests/test_operation_validation_preview.py`.
- DEL-16-02 depends on DEL-14-03 and DEL-14-05 for state comparison and mapping/tolerance/export contract context. Preview behavior should remain deterministic and diagnostic, not automatic external validation. Source: `Dependencies.csv` rows `DAG-002-E0829` and `DAG-002-E0830`; SOW-073 notes in decomposition.
- DEL-16-02 depends on DEL-04-06 for diagnostics/warning contract context. Validation and preview failures should preserve diagnostic classes and provenance where the relevant contract is available. Source: `Dependencies.csv` row `DAG-002-E0831`.
- Approved architecture-basis rows from PKG-00 are dispatchable context evidence, not Type 2 implementation authority by themselves. Source: `_CONTEXT.md#Architecture Basis Injection`; `_DEPENDENCIES.md#Authority Boundary`.

## Trade-offs

| Trade-off | Conservative handling |
|---|---|
| Early blocking vs richer preview | Current engine returns blocked preview rows for operation-local blocking diagnostics and reports global `diff_preview_status: blocked_by_validation` when any blocking diagnostic exists. It does not apply operations. Final UX/application handling of partial previews remains downstream. |
| Deterministic preview vs tolerance defaults | Current deterministic preview evidence compares canonical JSON for fixture output and does not define engineering comparison tolerances. Keep default tolerances TBD unless DEL-14-05 or a human product ruling defines them. |
| Agent convenience vs accepted-state safety | Agents may propose operations but do not mutate accepted engineering state directly. Favor an explicit review/apply boundary over convenience shortcuts. |
| Validation detail vs protected-data boundary | Do not encode protected standards text, proprietary values, or code-specific defaults in validation logic. Missing required values become explicit findings. |

## Examples

Current invented-public examples and focused evidence are:

- `schemas/model_operation.schema.json` - JSON Schema 2020-12 structured model operation contract.
- `fixtures/model_operations/invented_operation_set_valid.json` - invented operation-set fixture covering operation/change taxonomy, required current hashes, professional boundary, and model basis.
- `fixtures/model_operations/invented_accepted_model_state.json` - invented accepted model-state fixture with `physical_source_of_truth` role and current state hash.
- `tests/test_operation_validation_preview.py` - focused validation/preview tests for determinism, schema blocking, units/dimensions, constraints, direct mutation, model role/current hash, no mutation, and prohibited-claim boundaries.
- `tests/test_model_operation_schema.py` - schema contract checks for required fields, taxonomies, structured-only mutation route, current hashes, canonical dimensions, and professional-boundary constants.

These examples are implementation evidence for this slice, not final project data or professional validation examples.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| None | No source conflict identified in the accessible slices. Current implementation evidence resolves earlier generic implementation-path/schema-path/test-fixture TBDs, while downstream APIs, final diff contract, persistence/application behavior, and human dispositions remain TBD. | N/A | N/A | N/A | N/A | N/A |
