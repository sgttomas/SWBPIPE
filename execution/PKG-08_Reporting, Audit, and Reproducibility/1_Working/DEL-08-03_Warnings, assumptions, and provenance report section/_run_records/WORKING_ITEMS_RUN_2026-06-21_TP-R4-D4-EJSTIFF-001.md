# WORKING_ITEMS Run Record - TP-R4-D4-EJSTIFF-001

Date: 2026-06-21
Agent: WORKING_ITEMS
Deliverable: DEL-08-03 - Warnings, assumptions, and provenance report section
Package: PKG-08 - Reporting, Audit, and Reproducibility
Tranche: TP-R4-D4-EJSTIFF-001
Target stage: R4 / Phase D

## Scope

Landed the report-provenance evidence side of the D4 expansion-joint
app-absorption slice under `DEC-045`. The preview report packet and JSON export
now preserve expansion-joint pipe mapping, effective pressure area, movement
limit, hardware/manufacturer references, pressure-thrust reference,
user-entered stiffness values, and user-stiffness macro-element review rows for
the invented expansion-joint path.

## Implemented Evidence

- Extended report component provenance records with
  `expansion_joint_pipe_ref`, `effective_area`, `movement_limit`,
  `hardware_reference`, `manufacturer_reference`, `pressure_thrust_reference`,
  axial/lateral/angular/torsional user-entered stiffnesses, and
  `user_stiffness_result_refs`.
- Added a visible "Component stiffness inputs" report line and JSON export
  fields `component_user_stiffness_macro_element_evidence` and
  `component_user_stiffness_macro_element_count`.
- Added `DEL-03-06` to report export deliverable references for
  expansion-joint provenance evidence.
- Kept report boundary flags at no-private-payload, no-protected-content, and
  no release/professional/code-compliance claim.

## Validation

- `cargo test --manifest-path core/product_physics/Cargo.toml` - passed 34/34
  unit tests.
- `python3 -m pytest tests/product_preview/test_product_preview_service.py tests/test_results_schema.py tests/test_analysis_run_records.py` -
  passed 20/20 tests.
- `npm --prefix apps/desktop test -- --run src/App.test.tsx` - passed 57/57
  tests.
- `npm --prefix apps/desktop test -- --run` - passed 19/19 files, 406/406
  tests.
- `npm --prefix apps/desktop run build` - passed; retained the existing Vite
  chunk-size warning.
- `npm --prefix apps/desktop run build:wasm` - passed.
- From `apps/desktop`: `PLAYWRIGHT_WORKERS=1 npm exec playwright test e2e/r2-smoke.spec.ts --workers=1` -
  passed 18/18 Playwright tests.

## Boundary

The report remains a technical preview artifact and makes no release,
professional approval, certification, sealing, authentication, or
code-compliance claim. No protected standards content, private project data,
proprietary catalog data, or code-derived expansion-joint value was introduced.

## Residual

Expansion-joint provenance and user-stiffness review evidence now appear in
the preview report packet for the invented model. Broader D8 report closure
and D9 R4 validation evidence remain open.
