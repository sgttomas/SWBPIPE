# WORKING_ITEMS Run Record - TP-APP-R2-DELLOADCASE-001

Date: 2026-06-12
Persona: WORKING_ITEMS
Deliverable: DEL-07-02 Model tree and property inspector
Package: PKG-07 Graphical User Interface and Engineering Workflow

## Scope

Bounded app-integration tranche from completion-plan Phase A11:
add whole-load-case deletion authoring to the Load Cases manager while keeping
mutation routed through the structured operation apply panel.

Write scope for this deliverable:
- `apps/desktop/src/types.ts`
- `apps/desktop/src/features/load-cases/LoadCaseManagerPanel.tsx`
- `apps/desktop/src/styles.css`
- `apps/desktop/src/App.test.tsx`
- `apps/desktop/e2e/r2-smoke.spec.ts`
- `apps/desktop/SMOKE.md`

## Changes

- Added load-case delete rationale and `Queue delete case` control to the
  selected load-case editor.
- The UI queues a structured `delete_load_case` operation with the selected
  load-case id/label/kind/status/primitive-count display as the before-state
  guard and does not mutate the model directly.
- Applying the operation through `OperationApplyPanel` removes an unreferenced
  load-case row and updates load-case counts in the manager.
- The UI exposes the referenced-load block through the existing operation
  diagnostics when a combination term still points at the selected load case.
- The Playwright R2 smoke asserts the load-case delete preview beside existing
  load metadata and combination editor checks.

## Validation

- Focused App Vitest for load-case deletion: 2/2 passed.
- Full desktop Vitest: 201/201 passed.
- Desktop production build: passed.
- Playwright R2 smoke: 1/1 passed.
- Live in-app browser smoke at `http://127.0.0.1:5173/`: load-case
  create-then-delete flow passed with zero console errors.

## Boundary

The UI remains a review-and-apply operation authoring surface. It does not
insert protected content, supply code-specific engineering defaults, bypass
the operation seam, write private project data to the repository, cascade
deletion into combinations, or make any release-readiness, professional
approval, certification, sealing, authentication, or code-compliance claim.

## Handoff

A11 has landed support deletion, primitive-load deletion, full-combination
deletion, and load-case deletion. Remaining A11 entity deletion families are
node and pipe-run deletion. A12 full from-blank solve/report rehearsal remains
open, and A8 should carry that journey as automated R2 exit evidence.
Lifecycle state is unchanged.
