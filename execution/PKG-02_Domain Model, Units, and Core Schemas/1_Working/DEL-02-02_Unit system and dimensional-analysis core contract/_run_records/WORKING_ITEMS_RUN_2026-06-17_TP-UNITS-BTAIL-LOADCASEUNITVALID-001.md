# WORKING_ITEMS RUN - TP-UNITS-BTAIL-LOADCASEUNITVALID-001

- Date: 2026-06-17
- Persona: WORKING_ITEMS
- Tranche: `TP-UNITS-BTAIL-LOADCASEUNITVALID-001`
- Smoke target: `TP-MAC-205`
- Role: supporting unit evidence

## Scope

Support the DEL-05-01 primitive-load tranche by recording unit-dimension
validation status in unit-bearing load operation intents.

## Unit Evidence

- Unit-bearing primitive load create/edit intents no longer emit
  `unit_validation=not_run`.
- Browser preview status:
  `model_metadata_unit_dimension_declared_catalog_unavailable_browser_preview`.
- Desktop/Tauri catalog route statuses include `dec018_catalog_dimension_match`
  for accepted DEC-018 unit/dimension matches and explicit mismatch/unreviewed
  alternatives.

## Validation

- `npm test --workspace apps/desktop -- App.test.tsx` passed 55/55 tests.
- `npm run test:e2e --workspace apps/desktop -- -g "R2 desktop preview smoke"`
  passed 2/2 Playwright tests.
- `npm test --workspace apps/desktop` passed 18/18 files and 397/397 tests.
- `npm run build --workspace apps/desktop` passed with the existing Vite
  large-chunk warning.

## Boundary

No DEC-018 catalog constant change, schema dimension enum change, unit
conversion API change, primitive-load engine behavior, protected standards
content, private data, lifecycle transition, release-readiness claim,
professional approval, certification, sealing, authentication, or
code-compliance claim changed.
