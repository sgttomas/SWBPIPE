# WORKING_ITEMS RUN - TP-UNITS-BTAIL-RENDEREDREPORTUNITS-001

## Scope

- Package: PKG-02 Domain Model, Units, and Core Schemas.
- Supporting deliverable: DEL-02-02 Unit system and dimensional-analysis core
  contract.
- Primary deliverable: DEL-08-01 Calculation report generator.
- Tranche: `TP-UNITS-BTAIL-RENDEREDREPORTUNITS-001`.
- Smoke ID: `TP-MAC-232`.

## Supporting Unit Evidence

- The Rendered Report panel now exposes the render-input unit basis before
  the desktop-only renderer route is invoked.
- `rendered-report-unit-basis` reports
  `unit_system=unit-system:dec-018-si-dual-display`, sorted model units,
  solved result units or `results=none`, `conversion=false`, and
  `source=renderable_report_input`.
- The row mirrors the existing renderable-report input unit display summary
  without changing renderer behavior or report-time conversion policy.

## Validation

- `npm run test --workspace apps/desktop -- src/features/report/renderedReport.test.tsx`
  passed 8/8 tests.
- `npx playwright test e2e/r2-smoke.spec.ts -g "R2 desktop preview smoke covers solve, results, report, and viewport overlay|R2 from-blank GUI journey authors the A12 rehearsal script" --project=chromium-desktop`
  passed 2/2 focused Chromium desktop tests.
- `git diff --check` passed.
- `npm run test --workspace apps/desktop` passed 18/18 files and 399/399
  tests.
- `npm run build --workspace apps/desktop` passed with the existing Vite
  large-chunk warning.
- `npx playwright test e2e/r2-smoke.spec.ts --workers=1` passed 18/18 tests.

## Boundary

- No unit conversion or inference behavior changed.
- No DEC-018 catalog constant, schema dimension enum, unit-conversion API,
  renderer command, report schema, canonical hash behavior, save/print gate,
  report-time conversion, protected standards content, private data,
  lifecycle transition, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim changed.
