# Procedure: DEL-00-03 Application service command-query-job model

## Purpose
Execute and review this deliverable-local architecture document kit without crossing the PKG-00 boundary.

## Prerequisites
- Root bootstrap and governance documents have been read.
- `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.3 is the current basis.
- Coordination mode remains Full DAG with DAG authoring deferred.
- Blocker computation remains disabled until a human-approved acyclic DAG exists.

## Execution Steps
1. Identify service operations implied by GUI, CLI, solver, storage, reports, and adapters.
2. Classify each operation as command, query, or background job.
3. Define required inputs, outputs, diagnostics, cancellation, progress, and transaction behavior at the contract level.
4. Check each service path for unit-safety, provenance, and protected-data boundary enforcement.
5. Record unresolved implementation choices as TBD for human architecture review.

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
