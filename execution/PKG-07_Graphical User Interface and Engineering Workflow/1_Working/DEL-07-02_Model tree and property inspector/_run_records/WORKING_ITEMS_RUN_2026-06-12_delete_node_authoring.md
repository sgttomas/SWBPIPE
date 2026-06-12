# WORKING_ITEMS Run Record - TP-APP-R2-DELNODE-001

Date: 2026-06-12
Persona: WORKING_ITEMS
Deliverable: DEL-07-02 Model tree and property inspector
Package: PKG-07 Graphical User Interface and Engineering Workflow

## Scope

Bounded app-integration tranche from completion-plan Phase A11:
add node deletion authoring to the Property Inspector while keeping mutation
routed through the structured operation apply panel.

Write scope for this deliverable:
- `apps/desktop/src/types.ts`
- `apps/desktop/src/App.tsx`
- `apps/desktop/src/features/model-tree/PropertyInspector.tsx`
- `apps/desktop/src/App.test.tsx`
- `apps/desktop/e2e/r2-smoke.spec.ts`
- `apps/desktop/SMOKE.md`

## Changes

- Added a selected-node `Delete node intent` panel to the Property Inspector.
- The UI queues a structured `delete_node` operation with the selected node
  label/x/y/z display as the before-state guard and does not mutate the model
  directly.
- Applying the operation through `OperationApplyPanel` removes an unreferenced
  node row and falls back to the project row after the deleted node disappears.
- The UI exposes the referenced-node block through the existing operation
  diagnostics when pipes, supports, components, or primitive loads still point
  at the selected node.
- The Playwright R2 smoke asserts the node delete preview beside existing
  load-manager, pipe-delete, and viewport checks.

## Validation

- Focused App Vitest for node deletion: 2/2 passed.
- Full desktop Vitest: 213/213 passed.
- Desktop production build: passed.
- Playwright R2 smoke: 1/1 passed.
- Live in-app browser smoke at `http://127.0.0.1:5173/`: created
  `node:N-160`, queued and applied its delete intent, verified project-row
  fallback and zero console errors.

## Boundary

The UI remains a review-and-apply operation authoring surface. It does not
insert protected content, supply code-specific engineering defaults, bypass
the operation seam, write private project data to the repository, cascade
deletion into pipes/supports/components/loads, or make any release-readiness,
professional approval, certification, sealing, authentication, or
code-compliance claim.

## Handoff

A11 entity deletion coverage is complete. A12 full from-blank solve/report
rehearsal is the next unblocked plan item, and A8 should carry that journey
as automated R2 exit evidence. Lifecycle state is unchanged.
