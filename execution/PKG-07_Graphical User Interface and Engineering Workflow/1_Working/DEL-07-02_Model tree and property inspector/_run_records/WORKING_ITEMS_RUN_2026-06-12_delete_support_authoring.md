# WORKING_ITEMS Run Record - TP-APP-R2-DELSUPPORT-001

Date: 2026-06-12
Persona: WORKING_ITEMS
Deliverable: DEL-07-02 Model tree and property inspector
Package: PKG-07 Graphical User Interface and Engineering Workflow

## Scope

Bounded app-integration tranche from completion-plan Phase A11:
add support deletion authoring to the Property Inspector while keeping
mutation routed through the structured operation apply panel.

Write scope for this deliverable:
- `apps/desktop/src/types.ts`
- `apps/desktop/src/App.tsx`
- `apps/desktop/src/features/model-tree/PropertyInspector.tsx`
- `apps/desktop/src/App.test.tsx`
- `apps/desktop/SMOKE.md`

## Changes

- Added a `Delete support` inspector section that appears only when the
  selected entity is an existing support.
- The action queues a structured `delete_support` intent with current support
  label as the before-state guard, `after=not_present`, unit `none`, and
  dimension `dimensionless`.
- Applying a successful delete through the existing Apply Operations panel
  removes the support row and selects the project row so the inspector never
  points at a removed entity.
- Added UI coverage for both accepted unreferenced support deletion and
  referenced-support blocking after creating an imposed-displacement primitive
  load.

## Validation

- Focused App Vitest for support deletion/blocking: 2/2 passed.
- Full desktop Vitest: 189/189 passed.
- Desktop production build: passed.
- Playwright R2 smoke: 1/1 passed.
- Live Chrome smoke at `http://127.0.0.1:5173/`: support deletion flow passed
  with zero console errors.

## Boundary

The UI remains a review-and-apply operation authoring surface. It does not
insert protected content, supply code-specific engineering defaults, bypass
the operation seam, write private project data to the repository, or make any
release-readiness, professional approval, certification, sealing,
authentication, or code-compliance claim.

## Handoff

A11 has landed support deletion only. Remaining A11 entity deletion families
are node, pipe run, load case, primitive load, and full combination deletion.
A12 full from-blank solve/report rehearsal remains open. Lifecycle state is
unchanged.
