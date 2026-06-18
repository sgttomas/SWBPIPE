# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-MISSINGDATAUNITPOLICY-001

## Scope

- Persona: WORKING_ITEMS.
- Tranche: Phase B-tail missing-data unit-input policy visibility.
- Primary deliverable: DEL-07-04 missing-data warning and blocking UX.
- Supporting deliverable: DEL-02-02 unit system and dimensional-analysis core
  contract.

## Changes

- `MissingDataBlockingPanel` now exports `unit_input_policy_evidence` for the
  warning packet.
- The evidence records DEC-018/DEL-02-02/DEL-07-04 basis refs, sorted project
  units, explicit-unit requirements for unit-bearing missing inputs, and no
  inferred/default/auto-filled unit behavior.
- The panel now renders `missing-data-unit-policy` so browser review shows
  `required=true`, `default_units=false`, `conversion=false`, and the model
  unit signature.

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

- Missing-data review metadata only.
- No solver behavior, rule-check behavior, unit conversion API, DEC-018
  catalog constant, schema dimension enum, protected standards content,
  private payload, lifecycle state transition, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim changed.

## Residual

- C5.7 remains gated on a human packaged A12/R3 pass record; C5.8 remains
  dependent on C5.7.
- Broader B-tail target-format conversion witnesses and remaining app
  unit-policy surfaces stay tracked in the active completion plan.
