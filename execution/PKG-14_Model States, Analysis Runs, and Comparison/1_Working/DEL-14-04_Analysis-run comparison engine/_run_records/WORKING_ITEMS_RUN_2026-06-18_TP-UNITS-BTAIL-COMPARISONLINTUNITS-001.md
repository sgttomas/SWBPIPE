# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-COMPARISONLINTUNITS-001

Date: 2026-06-18

Persona: WORKING_ITEMS

Primary deliverable: DEL-08-05 - Report protected-content linter

Supporting deliverable: DEL-14-04 - Analysis-run comparison engine

## Scope

Supporting record for a bounded Phase B-tail Report Content Lint inventory
slice. The tranche records the existing Comparison workspace
equal-explicit-unit matching surface in the Report Content Lint public
unit-policy inventory.

## Changes

- Report Content Lint now inventories
  `apps/desktop/src/features/comparison/ComparisonPanel.tsx`.
- The lint packet now includes the Comparison workspace target ref with
  `unit_policy_surface_id=comparison-unit-policy`.
- The public unit-policy target count increased from 32 to 33; conversion
  witness count remains two and lint conversion remains false.

## Validation

Passed:

- `npm run test --workspace apps/desktop -- App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
  - 1/1 selected test passed.
- `npm run test --workspace apps/desktop -- App.test.tsx -t "round trips local create, save, and open project controls without external file copies"`
  - 1/1 selected test passed.
- `npm run test:e2e --workspace apps/desktop -- --grep "R2 desktop preview smoke"`
  - 2/2 focused configured-project tests passed.
- `npm run test --workspace apps/desktop`
  - 18/18 files and 399/399 tests passed.
- `npm run build --workspace apps/desktop`
  - Passed with the existing Vite large-chunk warning.
- `npm run test:e2e --workspace apps/desktop -- --workers=1`
  - 18/18 tests passed.

## Boundary

This is supporting inventory evidence only. It does not change comparison
delta math, result mapping, tolerance profile, default tolerance, solver
behavior, unit conversion behavior, protected standards content, private data,
lifecycle state, release readiness, professional approval, certification,
sealing, authentication, or code-compliance posture.

## Handoff

DEL-14-04 remains current comparison-engine support context for this
report-lint inventory evidence. C5.7 remains human-execution gated.
