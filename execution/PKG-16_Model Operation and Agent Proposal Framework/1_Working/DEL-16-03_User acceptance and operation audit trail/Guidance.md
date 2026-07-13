# Guidance: DEL-16-03 User acceptance and operation audit trail

<!-- D41-R5-T7-PDU055-CURRENTNESS -->
## D-41 R5 T7 PDU-055 current declaration

Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.

Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.

PDU-055 cited claim(s): `DEL-16-03-DECL-003`.

## Purpose

This deliverable exists to make model-operation acceptance reviewable and reproducible. It connects structured model operations and validation/diff preview outcomes to a durable audit surface that records whether an operation was accepted or rejected and preserves the metadata needed to understand the resulting model-state history.

## Principles

- Route model changes through structured operations. SOW-069 states that GUI and agent edits pass through schema validation, constraint validation, diff preview, and controlled application through the model engine.
- Preserve review context for accepted operations. SOW-070 identifies operation history, rationale, assumptions, affected entities, and audit metadata as needed for reproducible model-state review.
- Treat explicit user acceptance as the current accepted-record gate. The current audit implementation requires `accepted: true`, `decision: accept`, and `actor_type: user`; otherwise the record is held for user acceptance unless it is explicitly rejected.
- Require nonblocking validation and preview evidence before accepted status. Current accepted records require passed schema, constraint, and unit validation, generated diff preview, `application_status: not_applied`, a hash-bound diff-preview reference, and current accepted-state hash evidence.
- Keep audit recording separate from operation application. The current module records audit payloads and does not mutate accepted model state.
- Keep professional authority separate. The project permits computation and audit support, but not certification, sealing, approval, authentication, or automatic code-compliance claims.
- Prefer explicit TBDs over silent defaults. Unknown schema fields, autonomy details, persistence mechanics, and acceptance criteria remain TBD until supported by a sealed implementation brief or accepted architecture decision.

## Considerations

The audit trail depends on upstream operation schema and validation/diff preview surfaces recorded in the approved local DAG mirror. Current implementation evidence for those surfaces is outside this deliverable folder and read-only for this run: `schemas/model_operation.schema.json`, `core/model_operations/validation_preview/engine.py`, `fixtures/model_operations/invented_operation_set_valid.json`, and `fixtures/model_operations/invented_accepted_model_state.json`.

The current audit trail preserves enough metadata in returned payloads to make a model-operation acceptance decision reviewable: operation history, affected entities, actor/source metadata, validation outcome, diff-preview reference, accepted model-state reference/hash, rationale, assumptions, audit metadata, deterministic record hashes, top-level audit-trail hash, and visible diagnostics. It does not define storage tables, durable event streams, final actor identity policy, timestamp precision policy, long-term retention, or operation application.

Public fixtures or examples for this deliverable should avoid protected standards data, proprietary project records, and code-specific acceptance criteria unless they have documented public redistribution rights.

## Trade-offs

| Topic | Conservative guidance |
|---|---|
| Acceptance metadata detail | Capture source-backed minimum fields first; add implementation-specific fields only when the schema or service contract exists. |
| Rejected-operation retention | Current payload records rejected operations without mutating state; durable retention duration and storage remain TBD. |
| Agent autonomy | Keep explicit user acceptance as the accepted-record gate; do not infer autonomous acceptance from agent proposal capability. |
| Blocked validation | Treat blocking validation or preview evidence as preventing accepted status; record diagnostics visibly instead of upgrading status. |
| Accepted-state handling | Treat audit recording as nonmutating; operation application belongs outside this slice until separately resolved. |
| Professional wording | Use audit/review/development acceptance language; avoid professional approval or code-compliance wording. |
| Dependency mirror handling | Preserve approved DAG-006 rows as ACTIVE; do not reinterpret the mirror as a fresh extraction result. |

## Examples

Current source-backed examples are the invented operation and accepted-state fixtures under `fixtures/model_operations/`, exercised by `tests/test_operation_audit_trail.py`, `tests/test_operation_validation_preview.py`, and `tests/test_model_operation_schema.py`. They are public invented examples and do not establish protected engineering facts.

## Conflict Table (for human ruling)

No source conflicts were identified during Pass 1/2 drafting. The following unresolved items are not conflicts; they are source gaps to resolve later:

| Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling (TBD) |
|---|---|---|---|---|---|---|
| TBD | Durable persistence container, long-term retention policy, final actor identity model beyond `actor_type`/`actor_ref`/`source_role`, timestamp precision policy beyond current fixture evidence, operation application outside this slice, and human review dispositions are not resolved by the current implementation evidence. | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md`; `core/model_operations/audit_trail/engine.py`; focused tests | No persistence container, retention policy, final identity policy, timestamp policy, application workflow, or human disposition ruling present | Datasheet Conditions; Specification Documentation; Procedure Records | Future sealed Type 2 implementation brief or human governance ruling | TBD |
