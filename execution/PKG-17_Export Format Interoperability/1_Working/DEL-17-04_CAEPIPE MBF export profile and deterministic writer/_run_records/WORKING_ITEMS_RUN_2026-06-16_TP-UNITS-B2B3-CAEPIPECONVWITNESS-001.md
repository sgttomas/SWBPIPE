# WORKING_ITEMS Run Record - TP-UNITS-B2B3-CAEPIPECONVWITNESS-001

Date: 2026-06-16
Persona: WORKING_ITEMS
Deliverable: DEL-17-04 CAEPIPE MBF export profile and deterministic writer
Package: PKG-17 Export Format Interoperability

## Scope

Bounded B2/B3 unit evidence tranche for desktop CAEPIPE MBF source-to-target
node-coordinate conversion witnesses.

## Changes

- Added `conversion_witnesses` to the desktop CAEPIPE MBF downloaded JSON
  package.
- Added `conversion_witnesses.json` to the package manifest.
- Converted emitted MBF node coordinate fields from source model metres to
  target millimeters in the desktop smoke-subset payload and MBF text.
- Added one witness per node coordinate component with source node/field,
  source value/unit/dimension, target value/unit/field, conversion factor,
  DEC-018/DEL-02-02 basis refs, and preview provenance.
- Added validation checks for witness count and millimeter target units.
- Added a visible desktop panel line for witness count and target length unit.

## Validation

- `npm test -- --run src/App.test.tsx` - PASS, 54 tests.
- `npm test` - PASS, 386 tests.
- `npm run build` - PASS with existing Vite chunk-size warning.
- `npm run build:wasm && npx playwright test e2e/r2-smoke.spec.ts` - PASS,
  10 tests.

## Boundary

Desktop CAEPIPE MBF smoke package only. No Python/schema export-package
contract change, CAEPIPE target version/profile closure, direct MBF stable-ID
carrier claim, downstream import compatibility claim, external CAEPIPE
execution, solver validation, protected standards content, private-data
ingestion, release-readiness claim, professional approval, certification,
sealing, authentication, or code-compliance claim.

## Residuals

CAEPIPE target version/profile, definitive MBF record-family and required-field
subset, direct MBF stable-ID carrier, external execution, CSV parsing,
runtime/API/GUI integration beyond this desktop smoke witness, lifecycle
acceptance decisions, and target-specific compatibility claims remain future
guarded work.
