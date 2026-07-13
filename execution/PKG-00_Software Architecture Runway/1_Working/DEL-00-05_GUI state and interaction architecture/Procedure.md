# Procedure: DEL-00-05 GUI state and interaction architecture

## Purpose
Execute and review this deliverable-local architecture document kit without crossing the PKG-00 boundary.

## Prerequisites
- Root bootstrap and governance documents have been read.
- `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 is the current basis.
- Coordination uses approved `DAG-007` graph authority for relationship context; deliverable-local `_STATUS.md` remains lifecycle authority.
- Do not infer lifecycle promotion or implementation readiness from DAG context; human approval remains required for lifecycle changes.

## Execution Steps
1. Inventory GUI workflows implied by SPEC and PKG-07 deliverables, treating `DEL-07-02` as the accepted delegated owner of model-tree and property-inspector behavior under D-41 `DEC-074` O2.
2. Classify state into durable project, transient session, viewport, selection, editor, and background-job domains.
3. Define edit transaction and undo/redo rules at the architecture level.
4. Check that GUI state cannot bypass application services, diagnostics, unit validation, or data-boundary checks.
5. Record framework and state-library choices as TBD for human architecture review.
6. Verify that delegated `DEL-07-02` behavior conforms to this architecture basis without moving implementation, duplicating feature ownership, or expanding either deliverable's scope.

## Verification Checks
- Confirm the deliverable path is inside `execution/PKG-00_Software Architecture Runway/`.
- Confirm `_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, and `_SEMANTIC.md` exist.
- Confirm document-kit content maps to the scope items in `_CONTEXT.md`.
- Confirm no PKG-01 through PKG-12 files are modified.
- Confirm protected-content and professional-authority guardrails are stated.
- Confirm unresolved decisions are visible as `TBD`.
- Confirm property-inspector behavior evidence is attributed to `DEL-07-02`, while `DEL-00-05` remains the architecture owner.

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

## D-41 R5 T7 PDU-054 current declaration

Earlier setup-era statements on this surface are retained as historical setup context where applicable; this section is the active current-state declaration. The GUI state boundary now has command-backed selection and operation flows in the implemented slice. Current upstream authority is SOFTWARE_DECOMP revision 0.8 with DAG-007 coordination; this declaration does not claim completion of the full GUI product surface.
