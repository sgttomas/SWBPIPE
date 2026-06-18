# WORKING_ITEMS RUN - TP-UNITS-BTAIL-RENDEREDREPORTUNITS-001

## Scope

- Package: PKG-08 Reporting, Audit, and Reproducibility.
- Primary deliverable: DEL-08-01 Calculation report generator.
- Supporting deliverable: DEL-02-02 Unit system and dimensional-analysis core
  contract.
- Tranche: `TP-UNITS-BTAIL-RENDEREDREPORTUNITS-001`.
- Smoke ID: `TP-MAC-232`.

## Primary Evidence

- The Rendered Report panel now exposes the render-input unit basis before
  invoking the desktop-only renderer route.
- `rendered-report-unit-basis` reports
  `unit_system=unit-system:dec-018-si-dual-display`, sorted model units,
  solved result units or `results=none`, `conversion=false`, and
  `source=renderable_report_input`.
- The row mirrors the existing `buildRenderableReportInput` unit display
  summary; renderer command behavior, report schema, canonical hash behavior,
  save/print gating, and report-time conversion policy are unchanged.

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

- No renderer command, report schema, canonical hash behavior, save/print
  gate, report-time conversion, unit-conversion API, DEC-018 catalog constant,
  schema dimension enum, protected standards content, private data, lifecycle
  transition, release-readiness claim, professional approval, certification,
  sealing, authentication, or code-compliance claim changed.
