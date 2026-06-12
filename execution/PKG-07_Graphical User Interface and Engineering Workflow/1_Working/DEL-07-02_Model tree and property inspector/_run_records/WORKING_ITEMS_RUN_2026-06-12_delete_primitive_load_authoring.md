# WORKING_ITEMS Run Record - TP-APP-R2-DELPRIMLOAD-001

Date: 2026-06-12
Persona: WORKING_ITEMS
Deliverable: DEL-07-02 Model tree and property inspector
Package: PKG-07 Graphical User Interface and Engineering Workflow

## Scope

Bounded app-integration tranche from completion-plan Phase A11:
add primitive-load deletion authoring to the Load Cases manager while keeping
mutation routed through the structured operation apply panel.

Write scope for this deliverable:
- `apps/desktop/src/types.ts`
- `apps/desktop/src/features/load-cases/LoadCaseManagerPanel.tsx`
- `apps/desktop/src/App.test.tsx`
- `apps/desktop/SMOKE.md`

## Changes

- Added primitive-load delete rationale and `Queue delete` control to the
  selected primitive-load editor.
- The UI queues a structured `delete_primitive_load` operation with the
  selected primitive id/category/target/direction/magnitude as the before
  state guard and does not mutate the model directly.
- Applying the operation through `OperationApplyPanel` removes the primitive
  load row and updates load-case counts in the manager.

## Validation

- Focused App Vitest for primitive-load deletion: 1/1 passed.
- Full desktop Vitest: 192/192 passed.
- Desktop production build: passed.
- Playwright R2 smoke: 1/1 passed.
- Live Chrome smoke at `http://127.0.0.1:5173/`: primitive-load deletion flow
  passed with zero console errors.

## Boundary

The UI remains a review-and-apply operation authoring surface. It does not
insert protected content, supply code-specific engineering defaults, bypass
the operation seam, write private project data to the repository, or make any
release-readiness, professional approval, certification, sealing,
authentication, or code-compliance claim.

## Handoff

A11 has landed support deletion and primitive-load deletion. Remaining A11
entity deletion families are node, pipe run, load case, and full combination
deletion. A12 full from-blank solve/report rehearsal remains open. Lifecycle
state is unchanged.
