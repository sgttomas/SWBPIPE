# WORKING_ITEMS Run Record - TP-UNITS-B2B3-PCFCONVWITNESS-001

Date: 2026-06-16
Persona: WORKING_ITEMS
Deliverable: DEL-02-02 Unit system and dimensional-analysis core contract
Package: PKG-02 Domain Model, Units, and Core Schemas

## Scope

Supporting B2/B3 unit evidence for desktop PCF source-to-target conversion
witnesses.

## Changes

- Recorded DEC-018/DEL-02-02 basis refs in each desktop PCF conversion witness.
- Witnesses cover node coordinate and pipe OD/wall length fields converted to
  PCF millimeters, including source value/unit/dimension, target value/unit,
  and conversion factor.
- No unit catalog, conversion constant, schema, runtime solver, or Python
  export-package contract changed.

## Validation

- `npm test --workspace apps/desktop -- App.test.tsx` - PASS, 54 tests.
- `npm test --workspace apps/desktop` - PASS, 386 tests.
- `npm run build --workspace apps/desktop` - PASS with existing Vite
  chunk-size warning.
- `npm run test:e2e --workspace apps/desktop -- --grep "R2 desktop preview smoke"` - PASS, 2 tests.

## Boundary

No protected-content ingestion, private-data ingestion, target compatibility
claim, solver-deck validation claim, release-readiness claim, professional
approval, certification, sealing, authentication, or code-compliance claim.

## Residuals

Import round-trip unit I/O and the broader B3 mixed-unit round-trip,
conversion-witness, incompatible-unit rejection, and D-04/DEC-026 tolerance
corpus remain future guarded work outside this PCF desktop package witness.
