# Procedure: DEL-00-06 Diagnostics, warning, and result-envelope contract

## Purpose
Execute and review this deliverable-local architecture document kit without crossing the PKG-00 boundary.

## Prerequisites
- Root bootstrap and governance documents have been read.
- `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.3 is the current basis.
- Coordination mode remains Full DAG with DAG authoring deferred.
- Blocker computation remains disabled until a human-approved acyclic DAG exists.

## Execution Steps
1. Inventory diagnostic producers and consumers across the architecture layers.
2. Define common fields, warning classes, severity levels, and result-envelope status categories.
3. Map warning classes to GUI, CLI, report, and machine-readable behavior.
4. Check diagnostic wording against professional-authority and IP/data-boundary invariants.
5. Record schema and namespace choices as TBD for human architecture review.

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
