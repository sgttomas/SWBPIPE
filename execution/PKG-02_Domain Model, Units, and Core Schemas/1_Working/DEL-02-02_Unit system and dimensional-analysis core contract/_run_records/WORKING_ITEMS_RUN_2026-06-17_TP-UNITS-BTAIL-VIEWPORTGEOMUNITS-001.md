# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-VIEWPORTGEOMUNITS-001

Date: 2026-06-17
Persona: WORKING_ITEMS
Supporting deliverable: DEL-02-02 Unit system and dimensional-analysis core contract
Primary deliverable: DEL-07-01 3D viewport and centerline editor
Tranche: TP-UNITS-BTAIL-VIEWPORTGEOMUNITS-001
Smoke evidence: TP-MAC-209

## Supporting Role

This record supports the viewport tranche by documenting the unit-system
evidence effect: explicit viewport node and pipe authoring intents now carry
length unit-dimension validation status in `validation.unit_validation`.

## Evidence

- Browser preview records declared model metadata:
  `length=model_metadata_unit_dimension_declared_catalog_unavailable_browser_preview`.
- Desktop/Tauri catalog routes can record DEC-018 accepted dimension matches,
  mismatch, loading, or unreviewed statuses from the same shared helper.
- The queued intent UI exposes the status before application, preserving the
  distinction between unit evidence on the proposed operation and
  operation-applier validation, which reports a normalized pass/fail state.
- Generic one-click viewport gesture placeholders remain `not_run` because
  they do not yet carry explicit geometry/unit payloads.

## Validation

- `npm test --workspace apps/desktop -- App.test.tsx` passed 55/55 tests.
- `npm run test:e2e --workspace apps/desktop -- r2-smoke.spec.ts` passed
  14/14 Playwright tests.
- `npm test --workspace apps/desktop` passed 18/18 files and 398/398 tests.
- `npm run build --workspace apps/desktop` passed with the existing Vite
  large-chunk warning.

## Boundary Review

No DEC-018 catalog constant change, schema dimension enum change, unit
conversion API change, operation-applier validation semantics, solver behavior,
protected standards content, private payload, lifecycle state transition,
release-readiness claim, professional approval, certification, sealing,
authentication, or code-compliance claim changed.
