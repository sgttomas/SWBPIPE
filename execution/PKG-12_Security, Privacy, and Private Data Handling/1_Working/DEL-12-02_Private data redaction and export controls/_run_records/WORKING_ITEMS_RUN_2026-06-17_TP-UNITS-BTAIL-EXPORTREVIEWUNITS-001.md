# WORKING_ITEMS RUN - TP-UNITS-BTAIL-EXPORTREVIEWUNITS-001

- Date: 2026-06-17
- Persona: WORKING_ITEMS
- Tranche: `TP-UNITS-BTAIL-EXPORTREVIEWUNITS-001`
- Smoke target: `TP-MAC-204`
- Primary deliverable: `DEL-12-02`
- Supporting deliverables: `DEL-02-02`, `DEL-08-04`

## Scope

Bounded Phase B-tail export-review/unit-evidence inventory slice while C5.7
remains human-execution gated.

## Changes

- Added `unit_policy_summary` to the desktop Export Safety Review manifest.
- Added a 27-row `unit_evidence_matrix` that distinguishes unit-bearing local
  export records from metadata-only review records.
- Added visible `export-review-units` UI evidence showing the DEC-018 unit
  system, covered unit-bearing export count, and manifest-level conversion
  status.
- Added App Vitest assertions for the visible line and downloaded manifest
  JSON.

## Validation

- `npm test --workspace apps/desktop -- App.test.tsx` passed 55/55 tests.
- `npm run test:e2e --workspace apps/desktop -- -g "R2 desktop preview smoke"`
  passed 2/2 Playwright tests.
- `npm test --workspace apps/desktop` passed 18/18 files and 397/397 tests.
- `npm run build --workspace apps/desktop` passed with the existing Vite
  large-chunk warning.

## Boundary

Export-review metadata inventory only. No target-specific writer, manifest
unit conversion, runtime redaction-rule change, public transport commitment,
protected standards content, private project payload, lifecycle state
transition, release-readiness claim, professional approval, certification,
sealing, authentication, or code-compliance claim changed.
