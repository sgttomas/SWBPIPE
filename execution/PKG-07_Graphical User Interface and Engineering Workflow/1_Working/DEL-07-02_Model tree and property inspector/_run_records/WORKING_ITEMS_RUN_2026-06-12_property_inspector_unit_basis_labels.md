# WORKING_ITEMS Run Record - TP-UNITS-B2-INSPECTORLABELS-001

Date: 2026-06-12
Persona: WORKING_ITEMS
Deliverable: DEL-07-02 Model tree and property inspector
Package: PKG-07 Graphical User Interface and Engineering Workflow

## Scope

Bounded app-integration tranche from completion-plan Phase B2: connect the
Property Inspector's visible material/section unit labels to the reviewed
unit-catalog service without adding picker or conversion behavior.

Write scope for this deliverable:
- `apps/desktop/src/features/model-tree/PropertyInspector.tsx`
- `apps/desktop/src/App.test.tsx`
- `apps/desktop/e2e/r2-smoke.spec.ts`
- `apps/desktop/SMOKE.md`

## Changes

- Added a compact `Unit basis` status panel to the Property Inspector.
- The panel reports whether the DEC-018 catalog came from desktop/Tauri mode
  or whether browser preview is using model metadata only.
- Material and pipe-section creation labels now show the source of their unit
  basis (`m, model metadata`, `Pa, model metadata`, etc. in browser preview).
- The queued operation payload units are unchanged and still come from the
  model document.

## Validation

- Focused unit-catalog/App Vitest: 48/48 passed.
- Full desktop Vitest: 216/216 passed.
- Desktop production build: passed with the pre-existing Vite chunk-size
  warning.
- Playwright R2 smoke: 2/2 passed after wasm engine build, including the new
  unit-basis assertions.

## Boundary

The UI remains a review-and-apply operation authoring surface. It does not
perform unit conversion, insert a fallback browser unit catalog, supply
engineering defaults, bypass the operation seam, write private project data to
the repository, or make any release-readiness, professional approval,
certification, sealing, authentication, or code-compliance claim.

## Handoff

B2 still needs broader visible unit entry/pickers, solver-boundary
normalization, report unit-system disclosure, imports/exports, and rule-pack
unit I/O. Lifecycle state is unchanged.
