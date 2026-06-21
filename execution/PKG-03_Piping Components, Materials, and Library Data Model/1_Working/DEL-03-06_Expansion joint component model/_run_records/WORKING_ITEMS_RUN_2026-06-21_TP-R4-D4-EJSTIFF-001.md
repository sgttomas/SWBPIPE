# WORKING_ITEMS Run Record - TP-R4-D4-EJSTIFF-001

Date: 2026-06-21
Agent: WORKING_ITEMS
Deliverable: DEL-03-06 - Expansion joint component model
Package: PKG-03 - Piping Components, Materials, and Library Data Model
Tranche: TP-R4-D4-EJSTIFF-001
Target stage: R4 / Phase D

## Scope

Landed the DEL-03-06 expansion-joint app-absorption slice under `DEC-045` /
D-18 Option C. The preview model, product-physics adapter, desktop app, native
package review, viewport, and report packet now surface invented expansion
joint evidence with explicit pipe mapping, user-entered effective pressure
area, movement limit, hardware/manufacturer provenance, pressure-thrust
reference, and user-entered axial/lateral/angular/torsional stiffness values.

This run preserves the D4 boundary: expansion-joint stiffness rows are
dedicated user-stiffness macro-element review evidence for the preview product
surface. They do not claim a full assembled global macro-element solve, a
nonlinear iteration loop, code-derived stiffness values, manufacturer catalog
values, or pressure-thrust load generation.

## Files Touched In This Deliverable's Evidence Surface

- `core/product_physics/src/lib.rs`
- `core/product_physics/src/validation.rs`
- `fixtures/product_preview/invented_preview_model.json`
- `fixtures/product_preview/invented_mechanics_result.json`
- `apps/desktop/src/types.ts`
- `apps/desktop/src/features/model-workspace/modelView.ts`
- `apps/desktop/src/features/model-tree/ModelTree.tsx`
- `apps/desktop/src/features/model-tree/PropertyInspector.tsx`
- `apps/desktop/src/features/validation-evidence/ValidationEvidencePanel.tsx`
- `apps/desktop/src/features/native-package/NativePackagePanel.tsx`
- `apps/desktop/src/features/viewport/PipeViewport.tsx`
- `apps/desktop/src/features/report/ReportPanel.tsx`
- `apps/desktop/src/App.test.tsx`
- `apps/desktop/e2e/r2-smoke.spec.ts`
- `apps/desktop/SMOKE.md`

## Implemented Evidence

- Added expansion-joint geometry/modifier fields to the product-physics preview
  component inputs: `expansion_joint_pipe_ref`, `effective_area`,
  `movement_limit`, hardware/manufacturer/pressure-thrust/source references,
  and axial/lateral/angular/torsional user stiffness values.
- Added unit normalization and validation for expansion-joint effective area,
  movement limit, linear stiffness, and rotational stiffness quantities.
- Added invented `component:C-150`, mapped to `pipe:P-130`, with user-entered
  stiffness and provenance values and no protected/default catalog or
  code-derived factor source.
- Added `EXPANSION_JOINT_*` diagnostics for missing geometry input, invalid
  mapping, missing/invalid stiffness inputs, unsupported mechanics interface,
  and reviewed user-entered stiffness evidence.
- Appended four `component_user_stiffness_macro_element_review` result rows for
  `component:C-150`; the regenerated mechanics fixture now has 753 result rows
  and `summary.component_user_stiffness_macro_element_count = 4`.
- Added expansion-joint field visibility in the viewport, model view, property
  inspector, model tree/search/grid, validation panels, native-package
  witnesses, and report exports.

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

No lifecycle state, review disposition, DAG pointer, dependency register,
release-readiness claim, professional approval, certification, sealing,
authentication, or code-compliance claim changed. No protected standards
content, populated code table, code-derived expansion-joint factor,
proprietary catalog value, private project data, network path, telemetry
feature, pressure-thrust load generation, global nonlinear solve, or
repository-default private-data write was introduced.

## Residual

D4 expansion-joint app absorption is landed for the invented preview path.
Full assembled nonlinear solve behavior, pressure-thrust load application, and
R4 validation benchmarks remain out of scope for this slice. Broader Phase D
work remains: D5 spring-hanger scope gate (`D-15`), D6 nonlinear solve, D7
sparse live-path timing gate (`D-17`), D8 broader component provenance
reporting, and D9 R4 validation evidence.
