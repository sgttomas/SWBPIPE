---
run-id: WORKING_ITEMS_RUN_2026-06-12_report_unit_system_disclosure
timestamp: 2026-06-12T03:19:51-06:00
persona: WORKING_ITEMS
tranche-id: TP-UNITS-B2-REPORTUNITS-001
package-id: PKG-02
deliverable-id: DEL-02-02
run-status: SUCCESS
authority:
  - docs/PRD.md §10 FR-002
  - docs/PRD.md §10 FR-016
  - docs/CONTRACT.md OPS-K-UNIT-1
  - execution/_Decomposition/SOFTWARE_DECOMP.md DEC-018
  - plans/PLAN_2026-06-10_prd_completion.md Phase B B2
write-scope:
  - apps/desktop/src/features/report/ReportPanel.tsx
  - apps/desktop/src/features/report/renderableReportInput.ts
  - apps/desktop/src/features/report/renderedReport.test.tsx
  - apps/desktop/src/App.test.tsx
  - apps/desktop/e2e/r2-smoke.spec.ts
  - apps/desktop/SMOKE.md
  - execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-02_Unit system and dimensional-analysis core contract/MEMORY.md
  - execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-01_Calculation report generator/MEMORY.md
  - execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-02_Unit system and dimensional-analysis core contract/_run_records/**
  - execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-01_Calculation report generator/_run_records/**
  - plans/PLAN_2026-06-10_prd_completion.md
  - plans/PLAN_COMPLETION_LOG.md
---

# WORKING_ITEMS Run - TP-UNITS-B2-REPORTUNITS-001

## Scope

Implemented a bounded B2 report-disclosure slice: the desktop report packet
now visibly identifies the DEC-018 unit-system reference and exports the model
unit map plus result-row unit inventory. This does not add unit conversion or
expand the strict Rust report-generator schema.

## Changes

- Added `DEC018_UNIT_SYSTEM_REF` in `renderableReportInput.ts` and replaced
  the previous `preview-display-label-set` placeholder with
  `unit-system:dec-018-si-dual-display`.
- Added report packet `unit_system_disclosure` with the model unit map,
  distinct result-row units, entered-unit preservation posture, and
  `conversion_performed=false`.
- Added a visible `Unit system` line to the Report Packet panel.
- Added App, rendered-report-input, and Playwright assertions for the
  unit-system reference, model unit map, result unit inventory, and no
  report-time conversion.

## Validation

- `npm test --workspace apps/desktop -- --run src/features/report/renderedReport.test.tsx src/App.test.tsx`
  - PASS, 53 tests across 2 files.
- `npm run test:e2e --workspace apps/desktop -- --project=chromium-desktop apps/desktop/e2e/r2-smoke.spec.ts`
  - PASS, 2 Playwright tests after wasm engine build.
- `npm test --workspace apps/desktop` - PASS, 216 tests across 9 files.
- `npm run build --workspace apps/desktop` - PASS with the pre-existing Vite
  chunk-size warning.

## Boundary Review

- This tranche does not add unit conversion, solver-boundary normalization,
  unit picker behavior, import/export unit conversion, rule-pack unit I/O, or
  a browser fallback catalog.
- The strict report-generator schema is not widened; the rendered-report input
  only changes the existing unit-system reference.
- No protected standards text, protected dimensional tables, proprietary
  vendor data, private project data, engineering defaults, material
  allowables, SIFs, flexibility factors, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim was added.

## Handoffs

- B2 still owns broader visible unit entry/pickers, solver-boundary
  normalization, report renderer body expansion beyond packet disclosure,
  imports/exports, and rule-pack unit I/O.
- B3 still owns broader mixed-unit round-trip, conversion-witness,
  incompatible-unit rejection, and D-04/DEC-026 tolerance corpus coverage.
