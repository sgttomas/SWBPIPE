# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-PROJECTVALIDATIONLINTUNITS-001

Date: 2026-06-18

Persona: WORKING_ITEMS

Primary deliverable: DEL-08-05 - Report protected-content linter

Supporting deliverable: DEL-02-05 - Project persistence and round-trip serialization

## Scope

Supporting DEL-02-05 role for the bounded Phase B-tail Report Content Lint
inventory slice. The Project Validation Preflight packet already exposes
unit-policy evidence for local project unit round-trip metadata; this tranche
makes the report-lint public inventory consume that surface.

## Changes

- Report Content Lint now scans
  `apps/desktop/src/features/project-validation/ProjectValidationPanel.tsx`.
- The lint unit-policy inventory includes
  `unit_policy_surface_id=project-validation-unit-policy`.
- Unit-policy target count increases to 20; target-format conversion-witness
  count remains two.

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

Supporting report-lint inventory evidence only. No project persistence
semantics, validation-preflight semantics, migration policy,
project-envelope schema, model hash canonicalization, unit-conversion API,
DEC-018 catalog constant, schema dimension enum, report-linter
protected-content semantics, protected standards content, private data,
lifecycle transition, release-readiness claim, professional approval,
certification, sealing, authentication, or code-compliance claim changed.
