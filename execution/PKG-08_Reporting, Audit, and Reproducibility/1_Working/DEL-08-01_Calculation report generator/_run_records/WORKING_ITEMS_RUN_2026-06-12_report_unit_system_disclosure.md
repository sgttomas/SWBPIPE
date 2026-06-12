# WORKING_ITEMS Run Record - TP-UNITS-B2-REPORTUNITS-001

Date: 2026-06-12
Persona: WORKING_ITEMS
Deliverable: DEL-08-01 Calculation report generator
Package: PKG-08 Reporting, Audit, and Reproducibility

## Scope

Bounded B2 app-integration tranche: add unit-system disclosure to the
frontend report packet and point the rendered-report adapter at the DEC-018
unit-system reference without changing the strict report-generator schema.

Write scope for this deliverable:
- `apps/desktop/src/features/report/ReportPanel.tsx`
- `apps/desktop/src/features/report/renderableReportInput.ts`
- `apps/desktop/src/features/report/renderedReport.test.tsx`
- `apps/desktop/src/App.test.tsx`
- `apps/desktop/e2e/r2-smoke.spec.ts`
- `apps/desktop/SMOKE.md`

## Changes

- The Report Packet panel now displays a `Unit system` line naming
  `unit-system:dec-018-si-dual-display`, model units, distinct result units,
  and `conversion=false`.
- The report-packet JSON export now carries `unit_system_disclosure` with the
  model unit map, result unit inventory, entered-unit preservation posture,
  and no report-time conversion claim.
- The rendered-report input uses the DEC-018 unit-system reference in its
  existing `model_input_summary.unit_system_ref` field.

## Validation

- Focused report/App Vitest: 53/53 passed.
- Playwright R2 smoke: 2/2 passed after wasm engine build.
- Full desktop Vitest: 216/216 passed.
- Desktop production build: passed with the pre-existing Vite chunk-size
  warning.

## Boundary

The report packet remains technical-preview evidence. It does not perform
unit conversion, broaden the report-generator schema, include a browser
fallback catalog, add protected content or private project data, or make any
release-readiness, professional approval, certification, sealing,
authentication, or code-compliance claim.

## Handoff

B2 still needs solver-boundary normalization, broader visible unit
entry/pickers, report renderer body expansion beyond packet disclosure,
imports/exports, and rule-pack unit I/O. Lifecycle state is unchanged.
