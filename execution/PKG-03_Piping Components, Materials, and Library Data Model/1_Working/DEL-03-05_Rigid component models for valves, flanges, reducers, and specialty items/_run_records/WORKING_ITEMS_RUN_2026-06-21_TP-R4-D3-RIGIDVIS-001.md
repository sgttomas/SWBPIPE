# WORKING_ITEMS Run Record - TP-R4-D3-RIGIDVIS-001

Date: 2026-06-21
Agent: WORKING_ITEMS
Deliverable: DEL-03-05 - Rigid component models for valves, flanges, reducers, and specialty items
Package: PKG-03 - Piping Components, Materials, and Library Data Model
Tranche: TP-R4-D3-RIGIDVIS-001
Target stage: R4 / Phase D

## Scope

Landed the DEL-03-05 rigid/semi-rigid component app-absorption slice under
`DEC-045` / D-18 Option C. The preview model, product-physics adapter, desktop
app, native package review, and report packet now surface invented rigid
component evidence for a valve-family component with explicit mapping to a
pipe, user-entered dimensions, weight, center of gravity, stiffness/scaling
values, source notes, and `mechanics_geometry_only` interface metadata.

This run preserves the D3 boundary: rigid/semi-rigid component evidence is data,
mapping, provenance, and diagnostics only. The frame kernel has no scoped rigid
component element or stiffness-scale hook in this slice, so no frame stiffness
or solve-result behavior was changed.

## Files Touched In This Deliverable's Evidence Surface

- `core/product_physics/src/lib.rs`
- `core/product_physics/src/validation.rs`
- `fixtures/product_preview/invented_preview_model.json`
- `fixtures/product_preview/invented_mechanics_result.json`
- `apps/desktop/src/types.ts`
- `apps/desktop/src/features/model-workspace/modelView.ts`
- `apps/desktop/src/features/model-tree/ModelTree.tsx`
- `apps/desktop/src/features/model-tree/PropertyInspector.tsx`
- `apps/desktop/src/features/editor-contract/EditorContractPanel.tsx`
- `apps/desktop/src/features/missing-data/MissingDataBlockingPanel.tsx`
- `apps/desktop/src/features/rule-check/RuleCheckPanel.tsx`
- `apps/desktop/src/features/validation-evidence/ValidationEvidencePanel.tsx`
- `apps/desktop/src/features/project-validation/ProjectValidationPanel.tsx`
- `apps/desktop/src/features/native-package/NativePackagePanel.tsx`
- `apps/desktop/src/features/viewport/PipeViewport.tsx`
- `apps/desktop/src/features/report/ReportPanel.tsx`
- `apps/desktop/src/App.test.tsx`
- `apps/desktop/e2e/r2-smoke.spec.ts`

## Implemented Evidence

- Added rigid component geometry/modifier fields to the product-physics preview
  component inputs: `rigid_pipe_ref`, `rigid_body_length`, end sizes, weight,
  center of gravity, connection references, stiffness behavior/source
  references, stiffness scaling, linear stiffness, and rotational stiffness.
- Added unit normalization and unit validation for rigid component length,
  force, COG vector length, dimensionless stiffness scaling, linear stiffness,
  and rotational stiffness quantities.
- Added invented `component:C-130` with user-entered rigid/semi-rigid values,
  mapped to `pipe:P-130`, with no protected/default catalog or code-derived
  factor source.
- Added rigid component diagnostics for missing geometry input, invalid mapping,
  missing stiffness input, invalid stiffness input, and reviewed user-entered
  stiffness-scaling evidence.
- Added rigid component field visibility in the viewport, model view, property
  inspector, model tree/search/grid, editor contract, missing-data blockers,
  rule-check diagnostics, validation panels, native-package witnesses, and
  report exports.

## Validation

- `cargo test --manifest-path core/product_physics/Cargo.toml` - passed 33/33
  unit tests.
- `python3 -m pytest tests/product_preview/test_product_preview_service.py tests/test_results_schema.py tests/test_analysis_run_records.py` -
  passed 20/20 tests.
- `npm test --workspace apps/desktop -- App.test.tsx` - passed 57/57 tests.
- `npm test --workspace apps/desktop` - passed 19/19 files, 406/406 tests.
- `npm run build:desktop` - passed; retained the existing Vite chunk-size
  warning.
- `npm run test:e2e --workspace apps/desktop -- --project=chromium-desktop -g "R2 desktop preview smoke"` -
  passed 1/1 focused Playwright test.
- `npm run test:e2e:desktop` - passed 18/18 Playwright tests.

## Boundary

No lifecycle state, review disposition, DAG pointer, dependency register,
release-readiness claim, professional approval, certification, sealing,
authentication, or code-compliance claim changed. No protected standards
content, populated code table, code-derived rigid component factor, proprietary
catalog value, private project data, network path, telemetry feature,
frame-stiffness behavior change, or repository-default private-data write was
introduced.

## Residual

D3 rigid/semi-rigid component app absorption is landed for the invented preview
path. Full rigid macro-element solve behavior remains out of scope for this
slice; D3 records user-entered stiffness/scaling evidence and mapping
diagnostics only. Broader Phase D work remains: D4 expansion joints, D5
spring-hanger scope gate, D6 nonlinear solve, D8 broader component provenance
reporting, and D9 R4 validation evidence.
