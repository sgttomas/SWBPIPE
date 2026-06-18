# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-PROJECTVALIDATIONLINTUNITS-001

Date: 2026-06-18

Persona: WORKING_ITEMS

Primary deliverable: DEL-08-05 - Report protected-content linter

Supporting deliverable: DEL-02-02 - Unit system and dimensional-analysis core contract

## Scope

Supporting DEL-02-02 role for the bounded Phase B-tail Report Content Lint
inventory slice. The tranche aligns the report-lint public unit-policy
inventory with the existing DEC-018 evidence emitted by the Project
Validation Preflight panel and packet.

## Changes

- Report Content Lint now includes the Project Validation Preflight public
  unit-policy surface.
- The lint packet exports a `project-validation-unit-policy` target ref.
- Unit-policy target count increases from 19 to 20 while
  `conversion_witness_target_count` remains two and
  `lint_performs_conversion=false`.

## Validation

Passed:

- `npm run test --workspace apps/desktop -- src/App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
  - Initially caught the missing explicit lint target entry, then passed 1/1
    selected test after the target list was updated.
- `npx playwright test e2e/r2-smoke.spec.ts -g "R2 desktop preview smoke covers solve, results, report, and viewport overlay" --project=chromium-desktop`
  - 1/1 focused Chromium desktop test passed.
- `npm run test --workspace apps/desktop -- src/App.test.tsx -t "carries queued editor intents into the report packet as review-only operation context"`
  - 1/1 selected test passed after assertion repair.
- `npm run test --workspace apps/desktop`
  - 18/18 files passed; 399/399 tests passed.
- `npm run build --workspace apps/desktop`
  - Passed with the existing Vite large-chunk warning.
- `npx playwright test e2e/r2-smoke.spec.ts --workers=1`
  - 18/18 tests passed.

## Boundary

Supporting unit-evidence inventory only. No DEC-018 catalog constant, schema
dimension enum, unit-conversion API, validation-preflight semantics,
persistence semantics, report-linter protected-content semantics, protected
standards content, private data, lifecycle transition, release-readiness
claim, professional approval, certification, sealing, authentication, or
code-compliance claim changed.
