# WORKING_ITEMS RUN - TP-UNITS-BTAIL-REPORTLINTUNITS-001

## Scope

- Package: PKG-08 Reporting, Audit, and Reproducibility.
- Primary deliverable: DEL-08-05 Report protected-content linter.
- Supporting deliverable: DEL-02-02 Unit system and dimensional-analysis core
  contract.
- Tranche: `TP-UNITS-BTAIL-REPORTLINTUNITS-001`.
- Smoke ID: `TP-MAC-231`.

## Primary Evidence

- The Report Content Lint panel now exposes visible and exported unit-policy
  inventory evidence for explicit public report/export lint targets.
- `report-lint-unit-policy` reports 17 unit-policy targets, two
  target-format conversion-witness targets, and `lint_conversion=false`.
- The downloaded lint JSON carries `unit_policy_evidence` with
  `evidence_kind=public_surface_unit_policy_inventory`,
  `lint_performs_conversion=false`,
  `lint_asserts_target_format_compatibility=false`, and target refs for the
  PCF and CAEPIPE MBF conversion-witness surfaces.

## Validation

- `npm run test --workspace apps/desktop -- src/App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
  passed 1/1 selected test.
- `npx playwright test e2e/r2-smoke.spec.ts -g "R2 desktop preview smoke covers solve, results, report, and viewport overlay" --project=chromium-desktop`
  passed 1/1 focused Chromium desktop test.
- `git diff --check` passed.
- `npm run test --workspace apps/desktop` passed 18/18 files and 399/399
  tests.
- `npm run build --workspace apps/desktop` passed with the existing Vite
  large-chunk warning.
- `npx playwright test e2e/r2-smoke.spec.ts --workers=1` passed 18/18 tests.

## Boundary

- No linter protected-content semantics, legal clearance, redaction controls,
  target writer compatibility, unit-conversion API, DEC-018 catalog constant,
  schema dimension enum, protected standards content, private data, lifecycle
  transition, release-readiness claim, professional approval, certification,
  sealing, authentication, or code-compliance claim changed.
