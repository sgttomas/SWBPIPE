# WORKING_ITEMS Run Record - TP-R4-D8-COMPPROVREPORT-001

**Date:** 2026-06-21  
**Persona:** WORKING_ITEMS  
**Tranche:** `TP-R4-D8-COMPPROVREPORT-001`  
**Primary phase item:** R4 / Phase D / D8 - component provenance in reports  
**Primary package:** `PKG-08` - Reporting, Audit, and Reproducibility  
**Touched deliverables:** `DEL-08-01`, `DEL-08-03`  

## Objective

Move the R4 exit criterion "component provenance appears in reports" from
preview report-packet evidence toward the hash-bound rendered-report path.

## Changes

- `apps/desktop/src/features/report/renderableReportInput.ts`
  now maps every preview component into `report_sections.user_supplied_values`
  with category `component_provenance:<kind>`, `required_for =
  ["reporting", "human_review"]`, and source refs bound to the component ID.
- The same adapter now adds per-component provenance records to
  `report_sections.provenance_notes`, including component provenance,
  geometry source-reference fields, and modifier source references where
  present.
- Missing component provenance is now surfaced as a
  `COMPONENT_PROVENANCE_MISSING` / `PROVENANCE_WARNING` report-section
  diagnostic and as a non-accepted missing-data finding instead of being
  silently omitted.
- `apps/desktop/src/features/report/renderedReport.test.tsx` now asserts the
  rendered-report input carries complete component provenance rows and a
  missing-provenance warning.
- `core/reporting/report_renderer/tests/render.rs` now asserts the
  hash-bound HTML renderer displays component provenance user-value and
  provenance-note content in the warnings/assumptions/provenance section.
- `core/reporting/report_renderer/src/lib.rs` was normalized by `cargo fmt`
  while formatting the touched crate.

## Validation

- `cargo fmt --manifest-path core/reporting/report_renderer/Cargo.toml -- --check` - passed after formatting.
- `cargo test --manifest-path core/reporting/report_renderer/Cargo.toml` - passed (8 renderer integration tests).
- `npm test --workspace apps/desktop -- renderedReport` - passed (1 file, 8 tests).
- `npm run build --workspace apps/desktop` - passed with the existing Vite large-chunk warning.
- `git diff --check` - passed.

## Boundary Review

- Component provenance is report evidence only; it does not compute or supply
  code-derived component factors, protected standards values, proprietary
  catalog data, or public defaults.
- Missing component provenance remains visible as a warning/finding; it is not
  converted into accepted input.
- No lifecycle state, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim changed.

## Residuals

- D8 is landed for the current hash-bound rendered-report provenance path, but
  the broader R4 exit evidence package remains open under D9.
- D5 remains gated by `D-15`; D7 remains gated by `D-17`.
- D6/D9 nonlinear residuals still include the derived friction normal-force
  model, sliding friction validation, measured convergence values, broader
  live-solver coverage, and the PRD section 16.2 branch-assembly benchmark.
