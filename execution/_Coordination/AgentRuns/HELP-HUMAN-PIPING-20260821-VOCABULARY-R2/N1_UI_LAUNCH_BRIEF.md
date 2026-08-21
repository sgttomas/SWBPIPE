# N1 UI launch brief

- Instance: `WORKING-ITEMS-VOCAB-R2-N1-UI`; role `WORKING_ITEMS`; package `PKG-07`; selected deliverables `DEL-07-01` and `DEL-07-02`.
- Basis: run plan v1 and accepted bend implementation commit `b988d9d0e4a7048ac28a73bbe53ce045c631dff8`.
- Objective: replicate intent construction, viewport creation tools, inspector creation forms, and end-to-end tests for tee, reducer, valve, and flange, in that order.
- Allowed writes: `apps/desktop/src/features/component-creation/componentIntent.ts`, `apps/desktop/src/features/viewport/PipeViewport.tsx`, `apps/desktop/src/features/model-tree/PropertyInspector.tsx`, `apps/desktop/src/App.test.tsx`, directly coupled focused test files if needed, and instance-local N1 UI return/check records. Do not write resolver, deliverable status, coverage, receipt, shared handoff, export, or Git state.
- Return: changed paths, per-kind behavior, tests/commands/results, blockers, exact residuals, and validated package-level verdict.
- Escalate: any new normative/design choice, overlap, or scope expansion.
