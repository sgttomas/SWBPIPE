# Procedure: DEL-16-01 Structured model operation schema

## Purpose

Provide the bounded procedure for maintaining and verifying the structured model operation schema. This procedure records current implementation evidence and preserves downstream ownership boundaries.

## Prerequisites

| Prerequisite | Source / Status |
|---|---|
| Deliverable context and decomposition basis | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 |
| Scope and objective mapping | `_CONTEXT.md` rows `SOW-069` and `OBJ-015`; schema root constants |
| Architecture basis constraints | `_CONTEXT.md` "Architecture Basis Injection"; JSON Schema 2020-12 and canonical hash-basis notes |
| Approved dependency mirror | `Dependencies.csv`; preserve active rows unless a later approved workflow says otherwise |
| Schema artifact | Implemented at `schemas/model_operation.schema.json` |
| Operation fixture | Implemented at `fixtures/model_operations/invented_operation_set_valid.json` |
| Accepted model-state fixture | Implemented at `fixtures/model_operations/invented_accepted_model_state.json` |
| Focused DEL-16-01 validation | `tests/test_model_operation_schema.py` |
| Adjacent downstream validation evidence | `tests/test_operation_validation_preview.py`; behavior owned by DEL-16-02 |

## Steps

1. Confirm the operation schema remains scoped to DEL-16-01 and SOW-069 by checking root constants for `deliverable_id`, `package_id`, `scope_item`, and `objectives`.
2. Preserve JSON Schema 2020-12 as the public schema/interchange baseline.
3. Preserve the schema-first operation envelope: `schema_version`, `deliverable_id`, `package_id`, `scope_item`, `objectives`, `operation_contract_status`, and `operation_set`.
4. Preserve required operation-set structure: project reference, model reference, `model_basis`, operations, diagnostics, provenance, and professional boundary.
5. Preserve model-basis hooks for physical source-of-truth model role, accepted model-state reference, and accepted model-state hash.
6. Preserve operation record structure for operation identity/kind/status/author, target references, preconditions, changes, validation state, diagnostics, diff-preview refs, assumptions, provenance, and professional boundary.
7. Preserve the declared operation kinds: `add`, `move`, `modify`, `delete`, `reconnect`, `constraint`, `load`, `support`, and `design_knowledge`.
8. Preserve the declared change kinds: `add_object`, `remove_object`, `set_field`, `move_geometry`, `reconnect`, `update_constraint`, `update_load`, `update_support`, and `attach_design_knowledge`.
9. Keep operation data separate from direct persisted mutation. The schema contract status must retain `structured_operations_only` and `direct_model_mutation_allowed = false`.
10. Keep unit-bearing payload behavior explicit: unit metadata required, dimension check required, and missing unit behavior `emit_diagnostic`.
11. Keep public fixtures invented and schema-focused. Do not embed protected standards text, protected numeric tables, proprietary catalog values, private project data, or engineering defaults.
12. Keep downstream behavior boundaries explicit: validation/diff preview in DEL-16-02, acceptance/audit trail in DEL-16-03, and agent rationale/professional-boundary workflow in DEL-16-04.
13. Record unresolved issues for exact persistence granularity, broader hash partitioning, future operation granularity, and human review dispositions.

## Verification

| Check | Expected Result |
|---|---|
| Schema validation check | `python3 tests/test_model_operation_schema.py` exits 0. |
| Adjacent validation-preview check | `python3 tests/test_operation_validation_preview.py` exits 0; result is downstream behavior evidence only. |
| Category fixture check | The invented operation fixture covers all declared operation kinds and change kinds. |
| Boundary check | Operation records do not claim certification, sealing, approval, authentication, professional approval, engineering acceptance, or code compliance. |
| Mutation-route check | Contract status disallows direct model mutation; adjacent preview behavior does not mutate accepted state. |
| Source-fidelity check | Requirements trace to `_CONTEXT.md`, decomposition, governance docs, schema, fixtures, tests, or the approved local dependency mirror. Unsupported details remain TBD or ASSUMPTION. |
| Dependency mirror check | Existing local dependency rows remain ACTIVE and are not reclassified by this procedure. |

## Records

Current records for this deliverable/evidence slice:

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `MEMORY.md`
- `schemas/model_operation.schema.json`
- `fixtures/model_operations/invented_operation_set_valid.json`
- `fixtures/model_operations/invented_accepted_model_state.json`
- `tests/test_model_operation_schema.py`
- `tests/test_operation_validation_preview.py`
- `_run_records/TASK_RUN_*.md`

Records not changed by this procedure:

- `_STATUS.md`
- `_REVIEW.md`
- `Review_Findings.csv`
- `Dependencies.csv`
- `_DEPENDENCIES.md`
- schemas, fixtures, core code, and tests
