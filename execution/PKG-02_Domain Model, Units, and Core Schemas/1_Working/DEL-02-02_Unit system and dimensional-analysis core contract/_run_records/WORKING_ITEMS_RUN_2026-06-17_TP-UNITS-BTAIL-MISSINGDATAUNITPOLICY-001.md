# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-MISSINGDATAUNITPOLICY-001

## Scope

- Persona: WORKING_ITEMS.
- Tranche: Phase B-tail missing-data unit-input policy visibility.
- Primary deliverable: DEL-07-04 missing-data warning and blocking UX.
- Supporting deliverable: DEL-02-02 unit system and dimensional-analysis core
  contract.

## Supporting Role

- DEL-02-02 provides the DEC-018 unit-system authority cited by the new
  missing-data warning packet evidence.
- The tranche exposes the unit-policy witness at the GUI review boundary; it
  does not change the unit catalog, schema dimension enum, or conversion API.

## Validation

- `npm test --workspace apps/desktop -- --run src/App.test.tsx` passed 1/1
  file and 56/56 tests.
- `PLAYWRIGHT_WORKERS=1 npm run test:e2e --workspace apps/desktop -- e2e/r2-smoke.spec.ts -g "R2 desktop preview smoke covers solve, results, report, and viewport overlay"`
  passed 2/2 focused Playwright tests.
- `npm test --workspace apps/desktop` passed 18/18 files and 399/399 tests.
- `npm run build --workspace apps/desktop` passed with the existing Vite
  large-chunk warning.
- DEC-025 sweep evidence is recorded separately at closeout.

## Boundary

- No DEC-018 catalog constant, schema dimension enum, unit conversion API,
  protected standards content, private data, lifecycle transition,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.
