# Guidance: DEL-16-03 User acceptance and operation audit trail

## Purpose

This deliverable exists to make model-operation acceptance reviewable and reproducible. It connects structured model operations and validation/diff preview outcomes to a durable audit surface that records whether an operation was accepted or rejected and preserves the metadata needed to understand the resulting model-state history.

## Principles

- Route model changes through structured operations. SOW-069 states that GUI and agent edits pass through schema validation, constraint validation, diff preview, and controlled application through the model engine.
- Preserve review context for accepted operations. SOW-070 identifies operation history, rationale, assumptions, affected entities, and audit metadata as needed for reproducible model-state review.
- Treat user acceptance as the default gate. `_CONTEXT.md` and OI-016 preserve user acceptance unless a later human-approved decision changes the autonomy level.
- Keep professional authority separate. The project permits computation and audit support, but not certification, sealing, approval, authentication, or automatic code-compliance claims.
- Prefer explicit TBDs over silent defaults. Unknown schema fields, autonomy details, persistence mechanics, and acceptance criteria remain TBD until supported by a sealed implementation brief or accepted architecture decision.

## Considerations

The audit trail depends on upstream operation schema and validation/diff preview surfaces recorded in the approved local DAG-002 mirror. The mirror is evidence for sequencing and context only; it is not implementation evidence that the upstream surfaces are present in this folder.

The audit trail should preserve enough metadata to make later model-state review reproducible, but this setup pass does not define exact storage tables, event IDs, actor identity model, timestamp precision, or hash fields. Those are implementation details and remain TBD.

Public fixtures or examples for this deliverable should avoid protected standards data, proprietary project records, and code-specific acceptance criteria unless they have documented public redistribution rights.

## Trade-offs

| Topic | Conservative guidance |
|---|---|
| Acceptance metadata detail | Capture source-backed minimum fields first; add implementation-specific fields only when the schema or service contract exists. |
| Rejected-operation retention | Record rejected operations because `_CONTEXT.md` names accepted/rejected operations, but exact retention policy is TBD. |
| Agent autonomy | Keep user acceptance as the default; do not infer autonomous acceptance from agent proposal capability. |
| Professional wording | Use audit/review/development acceptance language; avoid professional approval or code-compliance wording. |
| Dependency mirror handling | Preserve approved DAG-002 rows as ACTIVE; do not reinterpret the mirror as a fresh extraction result. |

## Examples

TBD. No source-backed operation audit-log fixture, schema field names, or acceptance workflow examples are present in the accessible source set for this folder.

## Conflict Table (for human ruling)

No source conflicts were identified during Pass 1/2 drafting. The following unresolved items are not conflicts; they are source gaps to resolve later:

| Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling (TBD) |
|---|---|---|---|---|---|---|
| TBD | Exact audit-log schema, persistence mechanism, actor identity model, timestamp precision, and retention policy are not specified in the accessible sources. | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` | No implementation schema present | Datasheet Conditions; Specification Requirements; Procedure Steps | Future sealed Type 2 implementation brief | TBD |
