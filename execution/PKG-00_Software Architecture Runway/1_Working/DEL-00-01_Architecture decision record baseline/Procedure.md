# Procedure: DEL-00-01 Architecture decision record baseline

## Purpose
Execute and review this deliverable-local architecture document kit without crossing the PKG-00 boundary.

## Prerequisites
- Root bootstrap and governance documents have been read.
- `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.3 is the current basis.
- Coordination mode remains Full DAG with DAG authoring deferred.
- Blocker computation remains disabled until a human-approved acyclic DAG exists.

## Execution Steps
1. Read the current open issues and locate architecture questions routed to PKG-00.
2. Create or update the ADR inventory for decisions that block package-level implementation planning.
3. For each decision, record status, context, candidate options, consequences, and the required human ruling.
4. Confirm no ADR embeds protected standards text, proprietary tables, copied formulas, or vendor-private data.
5. Package the ADR inventory and template for human architecture review.

## Verification Checks
- Confirm the deliverable path is inside `execution/PKG-00_Software Architecture Runway/`.
- Confirm `_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, and `_SEMANTIC.md` exist.
- Confirm document-kit content maps to the scope items in `_CONTEXT.md`.
- Confirm no PKG-01 through PKG-12 files are modified.
- Confirm protected-content and professional-authority guardrails are stated.
- Confirm unresolved decisions are visible as `TBD`.

## Records to Preserve
- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`
- `_run_records/TASK_RUN_*.md`
- `_STATUS.md`

## Completion Condition
This deliverable is ready for human architecture review when the document kit exists, semantic artifacts exist, lifecycle state is `SEMANTIC_READY`, and all unresolved architecture decisions are visible rather than silently resolved.
