# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-COMPARISONGUIUNITS-001

Date: 2026-06-17

Persona: WORKING_ITEMS

Tranche: `TP-UNITS-BTAIL-COMPARISONGUIUNITS-001`

Supporting deliverable: `DEL-14-05` Comparison mapping, tolerance, and export
contracts

Primary deliverable: `DEL-14-04`

## Scope

Supporting comparison-policy evidence for the desktop comparison
workspace/unit-policy slice. The packet now records explicit equal-unit
matching and tolerance-not-checked policy without changing schema contracts or
adding governed tolerance values.

## Evidence

The comparison packet records DEC-018, DEC-026, and DEL-14-05 basis refs,
matched result units, unmatched row counts, `conversion_performed=false`, and
`tolerance_profile_ref=TBD`.

## Validation

- `npm test --workspace apps/desktop -- App.test.tsx` passed 56/56 tests.
- `npm run test:e2e --workspace apps/desktop -- r2-smoke.spec.ts` passed
  14/14 Playwright tests.
- `npm test --workspace apps/desktop` passed 18/18 files and 399/399 tests.
- `npm run build --workspace apps/desktop` passed with the existing Vite
  large-chunk warning.
- `git diff --check` passed.

## Boundary

No schema change, governed tolerance value, default tolerance, external
validation decision, comparison pass/fail claim, release threshold,
professional approval, certification, sealing, authentication, or
code-compliance claim changed.
