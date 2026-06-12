# WORKING_ITEMS Run Record - TP-UNITS-B2-REPORTBODY-001

Date: 2026-06-12
Persona: WORKING_ITEMS
Deliverable: DEL-08-01 Calculation report generator
Package: PKG-08 Reporting, Audit, and Reproducibility

## Scope

Bounded B2 report-body tranche: render explicit model/result unit evidence in
the hash-bound HTML calculation report, using a backward-compatible schema
extension and the existing DEC-021 renderer seam.

Write scope for this deliverable:
- `core/reporting/report_generator/src/lib.rs`
- `schemas/report_generator.schema.yaml`
- `core/reporting/report_renderer/src/lib.rs`
- `core/reporting/report_renderer/tests/render.rs`
- `fixtures/reports/invented/calculation_report_fixture.json`
- `apps/desktop/src/features/report/renderableReportInput.ts`
- `apps/desktop/src/features/report/ReportPanel.tsx`
- `apps/desktop/src/features/report/renderedReport.test.tsx`
- `apps/desktop/src-tauri/src/lib.rs`
- `apps/desktop/SMOKE.md`

## Changes

- `ModelInputSummary` now accepts optional `unit_display_summary`, preserving
  compatibility for older renderer inputs.
- The React renderer input builder populates entered-unit storage convention,
  model unit map, result-unit inventory, display policy, and
  `conversion_performed=false`.
- The Rust renderer prints those rows in the `Model Input Summary` section.
- The shared report fixture and Tauri renderer-command test assert the
  rendered HTML contains the unit summary rows.
- The A12 from-blank rehearsal helper replaces fixture unit evidence with the
  actual authored model units and solved result units before rendering.

## Validation

- Report-generator schema contract test: passed.
- Report-generator cargo tests: 10/10 passed.
- Report-renderer cargo tests: 8/8 passed.
- Tauri Rust tests: 32/32 passed.
- Focused report/App Vitest: 53/53 passed.
- Full desktop Vitest: 216/216 passed.
- Desktop production build: passed with the pre-existing Vite chunk-size
  warning.
- Playwright R2 smoke: 2/2 passed after wasm engine build.

## Boundary

The rendered report remains technical-preview, hash-bound HTML evidence. This
tranche does not perform report-time unit conversion, add a project-wide
unit-system picker, change rule-pack behavior, add import/export unit
conversion, synthesize a browser fallback catalog, include private or
protected content, or make any release-readiness, professional approval,
certification, sealing, authentication, or code-compliance claim.

## Handoff

B2 still needs broader app unit entry/pickers outside material/section create
forms, imports/exports, and rule-pack unit I/O. B3 still owns broader
mixed-unit round-trip, conversion-witness, incompatible-unit rejection, and
D-04/DEC-026 tolerance corpus coverage. Lifecycle state is unchanged.
