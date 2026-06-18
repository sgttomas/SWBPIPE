# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-PRIMDELETEUNITVALID-001

Date: 2026-06-17

Persona: WORKING_ITEMS

Tranche: `TP-UNITS-BTAIL-PRIMDELETEUNITVALID-001`

Primary deliverable: `DEL-05-01` Primitive load case engine

Supporting deliverables: `DEL-02-02`, `DEL-05-05`

## Scope

Bounded Phase B-tail primitive-load delete/unit-validation evidence slice while
C5.7 remains human-execution gated. The selected load-manager primitive-load
delete operation intent now records unit-dimension validation status for the
existing unit-bearing primitive quantity instead of `unit_validation=not_run`.

## Changes

- Updated `apps/desktop/src/features/load-cases/LoadCaseManagerPanel.tsx` so
  `buildDeletePrimitiveLoadIntent` receives the unit-catalog route, preserves
  the primitive's existing unit and dimension, and calls the same
  unit-dimension status helper used by primitive creation and magnitude edits.
- Updated the primitive-load delete preview to expose
  `unit_validation=<status>`.
- Extended the App test for deleting `load:L-100-Y` to assert the visible
  browser-preview fallback:
  `model_metadata_unit_dimension_declared_catalog_unavailable_browser_preview`.
- Updated `apps/desktop/SMOKE.md` TP-MAC-210, the active completion plan,
  completion log, and affected deliverable memories.

## Validation

- `npm test --workspace apps/desktop -- App.test.tsx` passed 55/55 tests.
- `npm test --workspace apps/desktop` passed 18/18 files and 398/398 tests.
- `npm run test:e2e --workspace apps/desktop -- r2-smoke.spec.ts` passed
  14/14 Playwright tests.
- `npm run build --workspace apps/desktop` passed with the existing Vite
  large-chunk warning.
- `git diff --check` passed.

## Boundary

Primitive-load delete intent evidence only. No DEC-018 catalog constant
change, schema dimension enum change, unit conversion API change,
operation-applier validation semantics, solver behavior, load engine behavior,
protected standards content, private payload, lifecycle state transition,
release-readiness claim, professional approval, certification, sealing,
authentication, or code-compliance claim changed.
