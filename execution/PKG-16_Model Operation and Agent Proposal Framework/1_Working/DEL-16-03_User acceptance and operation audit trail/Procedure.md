# Procedure: DEL-16-03 User acceptance and operation audit trail

## Purpose

Define the conservative operating procedure for producing and later verifying the DEL-16-03 backend feature slice. This procedure is evidence-aligned guidance for the current narrow implementation and does not claim durable persistence, operation application, lifecycle closure, or downstream governance decisions.

Current evidence now establishes a narrow implementation at `core/model_operations/audit_trail/engine.py` and focused tests at `tests/test_operation_audit_trail.py`. This procedure therefore distinguishes current audit-record behavior from unresolved downstream storage, retention, identity, timestamp, operation-application, and human-disposition matters.

## Prerequisites

- Current deliverable context: `_CONTEXT.md`.
- Governing reference list: `_REFERENCES.md`.
- Accepted decomposition basis: `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7.
- Approved local DAG-002 dependency mirror: `_DEPENDENCIES.md` and `Dependencies.csv`.
- Upstream context recorded in the local mirror, including architecture-basis rows DAG-002-E0744 through DAG-002-E0750 and execution context rows DAG-002-E0832 through DAG-002-E0836.
- Default user-acceptance posture from `_CONTEXT.md` and OI-016.
- Current audit-trail implementation: `core/model_operations/audit_trail/engine.py`.
- Current validation-preview implementation: `core/model_operations/validation_preview/engine.py`.
- Current upstream schema and fixtures: `schemas/model_operation.schema.json`, `fixtures/model_operations/invented_operation_set_valid.json`, and `fixtures/model_operations/invented_accepted_model_state.json`.
- Current focused tests: `tests/test_operation_audit_trail.py`, `tests/test_operation_validation_preview.py`, and `tests/test_model_operation_schema.py`.

## Steps

1. Confirm the work is still bounded to DEL-16-03, PKG-16, SOW-069, SOW-070, and OBJ-015.
2. Confirm that the operation under review is represented as a structured model operation. If not, record a rejection or TBD path rather than applying a hidden mutation.
3. Confirm that schema validation, constraint validation, unit validation, diff preview status, and application status are available from the upstream validation-preview surface.
4. Do not mark an audit record accepted unless validation has passed, diff preview has been generated, application status remains `not_applied`, a hash-bound diff-preview reference is present, and the operation current-hash precondition matches the accepted model-state hash.
5. Require an explicit user-acceptance signal before recording an operation as accepted. Under current behavior this means `accepted: true`, `decision: accept`, and `actor_type: user`.
6. If validation is blocked, accepted-state hash evidence is missing or mismatched, diff-preview hash evidence is missing, or the user signal is absent/non-user, record the operation as held for user acceptance rather than accepted.
7. For accepted operations, preserve operation history, rationale, assumptions, affected entities, validation outcome, diff-preview reference, accepted model-state reference, and audit metadata needed for reproducible model-state review.
8. For rejected operations, preserve rejected disposition and audit metadata without mutating accepted model state. Exact durable rejection-retention policy remains TBD.
9. For missing validation outcome, diff-preview reference, user decision, timestamp, rationale, source, actor, or audit metadata, preserve visible `TBD` diagnostics or placeholder fields rather than silently defaulting.
10. Avoid language or fields that imply professional approval, certification, sealing, authentication, or code compliance.
11. Validate public fixtures and examples for protected-content, provenance, privacy, and data-boundary issues before treating them as acceptable test data.
12. Produce acceptance workflow tests covering accepted, rejected, held-for-user-acceptance, blocked validation, missing hash, missing input/TBD, deterministic output, accepted-state nonmutation, and professional-boundary wording paths.

## Verification

| Check | Expected result |
|---|---|
| Scope check | Deliverable ID, package, scope items, and objective match `_CONTEXT.md`, registers, and decomposition. |
| Acceptance gate check | Proposed operations cannot become accepted audit records without the required user-acceptance signal under the current default posture. |
| Validation gate check | Blocked schema, constraint, unit, or diff-preview evidence prevents accepted audit status. |
| Hash binding check | Accepted audit records require a hash-bound diff-preview reference and accepted model-state hash evidence matching operation preconditions. |
| Audit metadata check | Accepted-operation records preserve source-backed audit metadata categories from SOW-070 and `_CONTEXT.md`. |
| Rejection path check | Rejected operations are recorded as rejected and do not mutate accepted model state. |
| Nonmutation check | Audit recording and validation preview do not apply operations or mutate accepted model state. |
| Assumption/TBD check | Missing implementation details remain visible as TBD or assumptions. |
| Boundary check | Audit records and tests avoid professional-approval claims and protected/private data leakage. |

## Records

- Operation audit log.
- Acceptance workflow tests.
- Review notes for unresolved TBDs and assumptions.
- Protected-content/provenance check results for any public fixtures.
- Future human decisions that change the default acceptance/autonomy posture.
- Future human decisions for durable persistence container, long-term retention policy, final actor identity model, timestamp precision policy, operation application outside this slice, and review finding dispositions.
