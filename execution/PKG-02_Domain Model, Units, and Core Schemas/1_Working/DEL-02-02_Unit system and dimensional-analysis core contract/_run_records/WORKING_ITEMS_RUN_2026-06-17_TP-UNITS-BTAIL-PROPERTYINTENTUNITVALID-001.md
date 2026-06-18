# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-PROPERTYINTENTUNITVALID-001

Date: 2026-06-17
Persona: WORKING_ITEMS
Supporting deliverable: DEL-02-02 Unit system and dimensional-analysis core contract
Primary deliverable: DEL-07-02 Model tree and property inspector
Tranche: TP-UNITS-BTAIL-PROPERTYINTENTUNITVALID-001
Smoke evidence: TP-MAC-206

## Supporting Role

This record supports the Property Inspector tranche by documenting the
unit-system evidence effect: unit-bearing editor intents now carry an explicit
unit-dimension validation status in `validation.unit_validation`.

## Evidence

- Browser preview records declared model metadata:
  `model_metadata_unit_dimension_declared_catalog_unavailable_browser_preview`.
- Desktop/Tauri catalog routes can record DEC-018 accepted dimension matches,
  mismatch, loading, or unreviewed statuses from the same basis display.
- Create-section, create-material, create-support, node-coordinate edit, and
  load-primitive edit previews have focused App/Playwright evidence.

## Validation

- `npm test --workspace apps/desktop -- App.test.tsx` passed 55/55 tests.
- `npm run test:e2e --workspace apps/desktop -- r2-smoke.spec.ts` passed 14/14
  Playwright tests.
- `npm test --workspace apps/desktop` passed 18/18 files and 397/397 tests.
- `npm run build --workspace apps/desktop` passed with the existing Vite
  large-chunk warning.

## Boundary Review

No DEC-018 catalog constant change, schema dimension enum change, unit
conversion API change, solver behavior, protected standards content, private
payload, lifecycle state transition, release-readiness claim, professional
approval, certification, sealing, authentication, or code-compliance claim
changed.
