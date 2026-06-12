# WORKING_ITEMS Run Record - TP-APP-R2-DELCOMBINATION-001

Date: 2026-06-12
Persona: WORKING_ITEMS
Deliverable: DEL-07-02 Model tree and property inspector
Package: PKG-07 Graphical User Interface and Engineering Workflow

## Scope

Bounded app-integration tranche from completion-plan Phase A11:
add full-combination deletion authoring to the Load Cases manager while
keeping mutation routed through the structured operation apply panel.

Write scope for this deliverable:
- `apps/desktop/src/types.ts`
- `apps/desktop/src/features/load-cases/LoadCaseManagerPanel.tsx`
- `apps/desktop/src/App.test.tsx`
- `apps/desktop/e2e/r2-smoke.spec.ts`
- `apps/desktop/SMOKE.md`

## Changes

- Added whole-combination delete rationale and `Queue delete combo` control
  to the selected combination editor.
- The UI queues a structured `delete_combination` operation with the selected
  combination id/label/basis/terms as the before-state guard and does not
  mutate the model directly.
- Applying the operation through `OperationApplyPanel` removes the
  combination row and updates combination counts in the manager.
- The Playwright R2 smoke asserts the whole-combination delete preview beside
  existing combination basis and term editor checks.

## Validation

- Focused App Vitest for combination deletion: 1/1 passed.
- Full desktop Vitest: 195/195 passed.
- Desktop production build: passed.
- Playwright R2 smoke: 1/1 passed.
- Live in-app browser smoke at `http://127.0.0.1:5173/`: whole-combination
  deletion flow passed with zero console errors.

## Boundary

The UI remains a review-and-apply operation authoring surface. It does not
insert protected content, supply code-specific engineering defaults, bypass
the operation seam, write private project data to the repository, or make any
release-readiness, professional approval, certification, sealing,
authentication, or code-compliance claim.

## Handoff

A11 has landed support deletion, primitive-load deletion, and full
combination deletion. Remaining A11 entity deletion families are node, pipe
run, and load case deletion. A12 full from-blank solve/report rehearsal
remains open. Lifecycle state is unchanged.
