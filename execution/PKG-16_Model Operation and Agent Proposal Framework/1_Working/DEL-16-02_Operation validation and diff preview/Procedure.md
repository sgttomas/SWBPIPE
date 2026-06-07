# Procedure: DEL-16-02 Operation validation and diff preview

## Purpose

This procedure describes the current conservative workflow for using the DEL-16-02 validation-preview slice and retaining evidence. It is aligned with `core/model_operations/validation_preview/engine.py`, `schemas/model_operation.schema.json`, focused fixtures, and focused tests. It does not define downstream persistence, application, or human-acceptance behavior.

## Prerequisites

| Prerequisite | Source | Status |
|---|---|---|
| DEL-16-01 Structured model operation schema | `Dependencies.csv` row `DAG-002-E0827`; `schemas/model_operation.schema.json` | Approved upstream dependency; current JSON Schema 2020-12 file and fixture evidence are available; final schema ownership remains upstream DEL-16-01. |
| DEL-13-03 Constraint validation engine | `Dependencies.csv` row `DAG-002-E0828` | Approved upstream dependency; current slice consumes injected blocking constraint diagnostics; exact API remains TBD. |
| DEL-14-03 Model-state comparison engine | `Dependencies.csv` row `DAG-002-E0829` | Approved upstream dependency; current engine produces deterministic before/after preview rows from accepted model-state fixture evidence; final integration remains TBD. |
| DEL-14-05 Comparison mapping, tolerance, and export contracts | `Dependencies.csv` row `DAG-002-E0830` | Approved upstream dependency; default tolerances TBD. |
| DEL-04-06 Solver diagnostics and singularity detection | `Dependencies.csv` row `DAG-002-E0831` | Approved upstream dependency; current slice emits structured diagnostics/provenance, while final cross-package diagnostic mapping remains TBD. |
| Architecture basis AB-00-01, AB-00-02, AB-00-03, AB-00-04, AB-00-06, AB-00-07, AB-00-08 | `_CONTEXT.md#Architecture Basis Injection` | Dispatchable context constraints; not implementation evidence. |

## Steps

1. Confirm the proposed edit is represented as a structured model operation.
   - Use `schemas/model_operation.schema.json` for the current structured operation contract.
   - Preserve DEL-16-01 as the owner of final schema semantics.
   - Source: SOW-069; DEL-16-01 dependency; `tests/test_model_operation_schema.py`.

2. Run schema validation against the approved operation schema.
   - The current engine uses `Draft202012Validator` against `schemas/model_operation.schema.json`.
   - Accept schema-conformant operation envelopes for later checks.
   - Return `OP-SCHEMA-VALIDATION-FAILED` diagnostics and `schema_validation: blocked` for schema failures.

3. Check model-basis and operation-current-hash preconditions.
   - Require accepted model state to be bound to `physical_source_of_truth`.
   - Require accepted model state to expose a current hash.
   - Require operation model basis and operation `required_current_hashes` to match the accepted model-state hash.
   - Emit blocking precondition diagnostics for missing or stale basis/hash evidence.

4. Check operation/change-level blockers.
   - Block unsupported operation or change taxonomies.
   - Block direct mutation requests.
   - Block missing quantity unit metadata and dimensions outside the current canonical vocabulary.
   - Block unresolved target references for change kinds that require existing accepted-state targets.

5. Incorporate constraint validation results.
   - Pass current `constraint_diagnostics` into the validation-preview call when available.
   - Blocking constraint findings are preserved as `OP-CONSTRAINT-BLOCKING` and `constraint_validation: blocked`.
   - Exact constraint engine interface remains TBD.

6. Generate deterministic diff preview output before application.
   - Use the current accepted model-state snapshot and operation envelope.
   - Current preview rows include `operation_id`, `change_id`, `change_kind`, `target_ref`, `preview_status`, `before`, `after`, and `application_status`.
   - Keep final preview payload fields and tolerance defaults TBD until upstream/downstream contracts define them.

7. Block invalid operations before application.
   - If schema or constraint validation fails, do not hand the operation to the controlled application path.
   - Current output always preserves `application_status: not_applied` and reports whether accepted model state was unchanged.
   - Preserve diagnostics and preview status without professional approval or code-compliance claims.

8. Hand valid, previewed operations to the later controlled application/user-acceptance workflow.
   - This deliverable does not implement hidden mutation or autonomous engineering acceptance.
   - User acceptance and operation audit trail are owned by DEL-16-03.

## Verification

| Check | Evidence to retain |
|---|---|
| Structured-operation input check | `tests/test_model_operation_schema.py` schema constants and `tests/test_operation_validation_preview.py::test_direct_mutation_request_is_rejected`. |
| Schema validation check | `tests/test_model_operation_schema.py`; `tests/test_operation_validation_preview.py::test_schema_invalid_envelope_cannot_report_schema_passed`. |
| Constraint validation check | `tests/test_operation_validation_preview.py::test_unresolved_target_and_constraint_findings_are_blocking`. |
| Diff determinism check | `tests/test_operation_validation_preview.py::test_valid_operation_generates_stable_preview_without_mutating_state`. |
| Model-basis/hash check | `tests/test_operation_validation_preview.py::test_model_role_and_current_hash_are_required_before_preview`. |
| Canonical dimension check | `tests/test_operation_validation_preview.py::test_missing_unit_metadata_blocks_preview` and `tests/test_operation_validation_preview.py::test_unknown_dimension_blocks_unit_validation`; `tests/test_model_operation_schema.py` canonical dimension assertions. |
| No-apply-on-invalid check | Current validation result keeps `application_status: not_applied`; focused tests check direct mutation blocking and accepted-state non-mutation. |
| Professional-boundary check | `tests/test_operation_validation_preview.py::test_output_boundary_language_does_not_make_prohibited_claims`; `tests/test_model_operation_schema.py` professional-boundary constants. |

## Records

- Operation validator implementation notes: `core/model_operations/validation_preview/engine.py`.
- Diff preview service contract: current deterministic fixture-backed row shape is implemented in `core/model_operations/validation_preview/engine.py`; final contract remains TBD.
- Validation test cases and fixture inventory: `tests/test_operation_validation_preview.py`, `tests/test_model_operation_schema.py`, `fixtures/model_operations/invented_operation_set_valid.json`, and `fixtures/model_operations/invented_accepted_model_state.json`.
- Diagnostic/result-envelope mapping notes: current structured diagnostics/provenance/professional-boundary output exists; final cross-package mapping remains TBD.
- Dependency preservation record: existing `Dependencies.csv` approved DAG-006 rows remain ACTIVE and unchanged by this setup workflow.
