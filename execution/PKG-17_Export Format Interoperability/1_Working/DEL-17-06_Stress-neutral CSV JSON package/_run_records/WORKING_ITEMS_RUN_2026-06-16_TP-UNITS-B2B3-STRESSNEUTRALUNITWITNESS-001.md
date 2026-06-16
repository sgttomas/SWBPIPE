# WORKING_ITEMS Run Record - TP-UNITS-B2B3-STRESSNEUTRALUNITWITNESS-001

Date: 2026-06-16
Persona: WORKING_ITEMS
Deliverable: DEL-17-06 Stress-neutral CSV JSON package
Package: PKG-17 Export Format Interoperability

## Scope

Bounded B2/B3 unit evidence tranche for desktop stress-neutral CSV/JSON
per-row unit preservation witnesses.

## Changes

- Added `unit_preservation_witnesses` to the desktop stress-neutral downloaded
  JSON package.
- Added `unit_preservation_witnesses.json` to the package manifest.
- Added one witness per exported result row with source result ref/field,
  source value/unit/dimension, target row ref/field, target
  value/unit/dimension, `conversion_performed=false`, DEC-018/DEL-02-02 and
  DEL-17-06 basis refs, and preview provenance.
- Added validation checks for one witness per row and source/target row
  agreement.
- Added a visible desktop panel line for witness count and conversion policy.

## Validation

- `npm test -- --run src/App.test.tsx` - PASS, 54 tests.
- `npm test` - PASS, 386 tests.
- `npm run build` - PASS with existing Vite chunk-size warning.
- `npm run build:wasm && npx playwright test e2e/r2-smoke.spec.ts` - PASS,
  10 tests.

## Boundary

Desktop stress-neutral review package only. No Python/schema export-package
contract change, vendor-format claim, target compatibility claim, comparison
pass/fail claim, solver validation, protected standards content, private-data
ingestion, release-readiness claim, professional approval, certification,
sealing, authentication, or code-compliance claim.

## Residuals

Stress-neutral comparison pass/fail semantics, tolerance profiles, external
target interpretation, release gates, and professional acceptance remain
outside this package. Broader B3 mixed-unit round-trip, conversion-witness,
incompatible-unit rejection, and D-04/DEC-026 tolerance corpus remain future
guarded work outside this unit-preservation witness.
