---
run-id: WORKING_ITEMS_RUN_2026-06-12_report_body_unit_disclosure
timestamp: 2026-06-12T03:52:11-06:00
persona: WORKING_ITEMS
tranche-id: TP-UNITS-B2-REPORTBODY-001
package-id: PKG-02
deliverable-id: DEL-02-02
run-status: SUCCESS
authority:
  - docs/PRD.md §10 FR-002
  - docs/PRD.md §10 FR-016
  - docs/CONTRACT.md OPS-K-UNIT-1
  - execution/_Decomposition/SOFTWARE_DECOMP.md DEC-018
  - execution/_Decomposition/SOFTWARE_DECOMP.md DEC-021
  - plans/PLAN_2026-06-10_prd_completion.md Phase B B2
write-scope:
  - schemas/report_generator.schema.yaml
  - core/reporting/report_generator/src/lib.rs
  - core/reporting/report_renderer/src/lib.rs
  - core/reporting/report_renderer/tests/render.rs
  - fixtures/reports/invented/calculation_report_fixture.json
  - apps/desktop/src/features/report/renderableReportInput.ts
  - apps/desktop/src/features/report/ReportPanel.tsx
  - apps/desktop/src/features/report/renderedReport.test.tsx
  - apps/desktop/src-tauri/src/lib.rs
  - apps/desktop/SMOKE.md
  - execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-02_Unit system and dimensional-analysis core contract/MEMORY.md
  - execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-01_Calculation report generator/MEMORY.md
  - plans/PLAN_2026-06-10_prd_completion.md
  - plans/PLAN_COMPLETION_LOG.md
---

# WORKING_ITEMS Run - TP-UNITS-B2-REPORTBODY-001

## Scope

Implemented a bounded B2 report-body slice: the hash-bound rendered report
body now carries explicit unit-display evidence in the `Model Input Summary`
section. This extends the prior Report Packet disclosure without adding
report-time conversion or changing lifecycle state.

## Changes

- Added optional `unit_display_summary` to the report-generator
  `ModelInputSummary` contract and `schemas/report_generator.schema.yaml`.
- The summary records `storage_convention`, sorted `model_units`,
  `result_units`, `quantity_display_policy`, and `conversion_performed`.
- The frontend report adapter populates `unit_display_summary` from the same
  model/result unit inventory used by the Report Packet disclosure.
- The Rust report renderer emits the unit storage convention, model units,
  result units, quantity display policy, and report-time conversion flag in
  the rendered HTML.
- The shared invented report fixture carries the new summary, and the A12
  rehearsal helper overwrites it with actual authored model units plus solved
  result units before rendering.

## Validation

- `python3 tests/test_report_generator_contract.py`
  - PASS.
- `cargo test --manifest-path core/reporting/report_generator/Cargo.toml`
  - PASS, 10/10 tests.
- `cargo test --manifest-path core/reporting/report_renderer/Cargo.toml`
  - PASS, 8/8 integration tests.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - PASS, 32/32 tests.
- `npm test --workspace apps/desktop -- --run src/features/report/renderedReport.test.tsx src/App.test.tsx`
  - PASS, 53/53 tests.
- `npm run build:wasm --workspace apps/desktop`
  - PASS.
- `npm test --workspace apps/desktop -- --run`
  - PASS, 216/216 tests.
- `npm run build --workspace apps/desktop`
  - PASS with the pre-existing Vite chunk-size warning.
- `npm run test:e2e --workspace apps/desktop -- --project=chromium-desktop apps/desktop/e2e/r2-smoke.spec.ts`
  - PASS, 2/2 Playwright tests after wasm engine build.

## Boundary Review

- No report-time unit conversion, project-wide unit-system picker,
  import/export unit conversion, rule-pack unit I/O, browser fallback catalog,
  protected standards text, protected dimensional tables, proprietary vendor
  data, private project data, material allowables, SIFs, flexibility factors,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim was added.
- `unit_display_summary` is optional for backward compatibility; existing
  report inputs without the field remain valid.

## Handoffs

- B2 still owns broader app unit entry/pickers outside material/section create
  forms, imports/exports, and rule-pack unit I/O.
- B3 still owns broader mixed-unit round-trip, conversion-witness,
  incompatible-unit rejection, and D-04/DEC-026 tolerance corpus coverage.
