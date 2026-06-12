# WORKING_ITEMS Run Record - TP-APP-R2-CREATESUPPORT-001

Date: 2026-06-12
Persona: WORKING_ITEMS
Deliverable: DEL-07-02 Model tree and property inspector
Package: PKG-07 Graphical User Interface and Engineering Workflow

## Scope

Bounded app-integration tranche from completion-plan Phase A10:
add the first from-scratch support authoring form to the Property Inspector.

Write scope for this deliverable:
- `apps/desktop/src/features/model-tree/PropertyInspector.tsx`
- `apps/desktop/src/styles.css`
- `apps/desktop/src/App.test.tsx`
- `apps/desktop/SMOKE.md`

## Changes

- Added a `Create support` inspector section with support id, label,
  existing-node selector, restraint checkboxes, provenance, and a
  `Queue support` action.
- The form emits a structured `create_support` intent; it does not mutate the
  model directly.
- Applying the queued operation through the existing Apply Operations panel
  selects the newly created support in the model tree and shows the support
  fields in the inspector.
- Added compact checkbox styling for restraint tokens.

## Validation

- Focused App Vitest for support creation: 1/1 passed.
- Full desktop Vitest: 177/177 passed.
- Desktop production build: passed.
- Playwright R2 smoke: 1/1 passed.
- Live Chrome smoke at `http://127.0.0.1:5173/`: support creation flow passed
  with zero console errors.

## Boundary

The UI remains a review-and-apply operation authoring surface. It does not
insert protected content, supply code-specific engineering defaults, bypass
the operation seam, write private project data to the repository, or make any
release-readiness, professional approval, certification, sealing,
authentication, or code-compliance claim.

## Handoff

A10 still needs material and section creation forms. A11 deletion coverage
remains open. Lifecycle state is unchanged.
