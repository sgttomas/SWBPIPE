# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-EXTERNALPROVERUNITS-001

Date: 2026-06-17

Persona: WORKING_ITEMS

Tranche: `TP-UNITS-BTAIL-EXTERNALPROVERUNITS-001`

Supporting deliverable: `DEL-02-02` Unit system and dimensional-analysis core
contract

Primary deliverable: `DEL-15-04`

## Scope

Supporting unit-system evidence for the external-prover metadata/unit-policy
slice. The desktop External Prover Boundary panel now records explicit
unit-policy evidence for metadata-only external review packets without
claiming target conversion or external-prover execution.

## Unit Evidence

The packet records `unit-system:dec-018-si-dual-display`,
`entered_units_preserved`, source model units, result units when a mechanics
run exists, empty target export units, and
`conversion_policy=external_prover_metadata_records_units_without_target_conversion`
with `conversion_performed=false`. The external-prover-specific witness policy
keeps the evidence descriptive and reviewer-facing only.

## Validation

- `npm test --workspace apps/desktop -- App.test.tsx` passed 55/55 tests.
- `npm test --workspace apps/desktop` passed 18/18 files and 398/398 tests.
- `npm run test:e2e --workspace apps/desktop -- r2-smoke.spec.ts` passed
  14/14 Playwright tests.
- `npm run build --workspace apps/desktop` passed with the existing Vite
  large-chunk warning.
- `git diff --check` passed.

## Boundary

No DEC-018 catalog constant change, schema dimension enum change, unit
conversion API change, solver/prover invocation, target parser,
commercial-result ingestion, protected standards content, private payload,
lifecycle state transition, release-readiness claim, professional approval,
certification, sealing, authentication, or code-compliance claim changed.
