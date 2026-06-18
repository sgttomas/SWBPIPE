# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-COMPARISONGUIUNITS-001

Date: 2026-06-17

Persona: WORKING_ITEMS

Tranche: `TP-UNITS-BTAIL-COMPARISONGUIUNITS-001`

Supporting deliverable: `DEL-02-02` Unit system and dimensional-analysis core
contract

Primary deliverable: `DEL-14-04`

## Scope

Supporting unit-system evidence for the comparison workspace/unit-policy
slice. The desktop comparison packet now records explicit unit-policy
evidence for mapped result deltas without claiming conversion or tolerance
acceptance.

## Unit Evidence

The packet records `unit-system:dec-018-si-dual-display`,
`entered_units_preserved`, equal-explicit-unit matching policy, matched result
units, unmatched row counts, and
`conversion_policy=comparison_workspace_preserves_result_units_without_conversion`
with `conversion_performed=false`.

## Validation

- `npm test --workspace apps/desktop -- App.test.tsx` passed 56/56 tests.
- `npm run test:e2e --workspace apps/desktop -- r2-smoke.spec.ts` passed
  14/14 Playwright tests.
- `npm test --workspace apps/desktop` passed 18/18 files and 399/399 tests.
- `npm run build --workspace apps/desktop` passed with the existing Vite
  large-chunk warning.
- `git diff --check` passed.

## Boundary

No DEC-018 catalog constant change, schema dimension enum change, unit
conversion API change, comparison delta math, default tolerance, solver
convergence policy, protected standards content, private payload, lifecycle
state transition, release-readiness claim, professional approval,
certification, sealing, authentication, or code-compliance claim changed.
