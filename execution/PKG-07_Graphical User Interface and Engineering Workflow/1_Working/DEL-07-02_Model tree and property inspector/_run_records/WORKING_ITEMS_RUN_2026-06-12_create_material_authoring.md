# WORKING_ITEMS Run Record - TP-APP-R2-CREATEMATERIAL-001

Date: 2026-06-12
Persona: WORKING_ITEMS
Deliverable: DEL-07-02 Model tree and property inspector
Package: PKG-07 Graphical User Interface and Engineering Workflow

## Scope

Bounded app-integration tranche from completion-plan Phase A10:
add the second from-scratch entity authoring form to the Property Inspector.

Write scope for this deliverable:
- `apps/desktop/src/features/model-tree/PropertyInspector.tsx`
- `apps/desktop/src/App.test.tsx`
- `apps/desktop/SMOKE.md`

## Changes

- Added a `Create material` inspector section with material id, label,
  elastic modulus, shear modulus, optional thermal expansion coefficient,
  provenance, and a `Queue material` action.
- The form emits a structured `create_material` intent; it does not mutate
  the model directly.
- Applying the queued operation through the existing Apply Operations panel
  selects the newly created material in the model tree and shows the material
  fields in the inspector.

## Validation

- Focused App Vitest for material creation: 1/1 passed.
- Full desktop Vitest: 180/180 passed.
- Desktop production build: passed.
- Playwright R2 smoke: 1/1 passed.
- Live Chrome smoke at `http://127.0.0.1:5173/`: material creation flow
  passed with zero console errors.

## Boundary

The UI remains a review-and-apply operation authoring surface. It does not
insert protected content, supply code-specific engineering defaults, bypass
the operation seam, write private project data to the repository, or make any
release-readiness, professional approval, certification, sealing,
authentication, or code-compliance claim.

## Handoff

A10 still needs section creation forms. A11 deletion coverage remains open.
Lifecycle state is unchanged.
