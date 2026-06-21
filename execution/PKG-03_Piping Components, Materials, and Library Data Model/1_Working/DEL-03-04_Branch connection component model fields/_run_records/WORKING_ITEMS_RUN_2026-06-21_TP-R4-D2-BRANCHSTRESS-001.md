# WORKING_ITEMS Run Record - TP-R4-D2-BRANCHSTRESS-001

Date: 2026-06-21
Agent: WORKING_ITEMS
Deliverable: DEL-03-04 - Branch connection component model fields
Package: PKG-03 - Piping Components, Materials, and Library Data Model
Tranche: TP-R4-D2-BRANCHSTRESS-001
Target stage: R4 / Phase D

## Scope

Landed the DEL-03-04 component-data side of the D2 branch connection
app-absorption slice under `DEC-045` / D-18 Option C. The preview mechanics
adapter and desktop app now consume branch component records from the invented
preview model, including header/branch pipe references, run/header sizes,
connection angle/type, reinforcement reference, geometry source, user-entered
header and branch SIFs, user-entered flexibility factor, modifier source, and
`mechanics_geometry_only` interface metadata.

This run did not add protected standards tables, default engineering factors,
private catalog values, code-derived branch SIF/flexibility content, or
private project data. The invented fixture remains user-entered preview data.

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
- `apps/desktop/src/features/viewport/PipeViewport.tsx`
- `apps/desktop/src/features/report/ReportPanel.tsx`
- `apps/desktop/src/App.test.tsx`
- `apps/desktop/e2e/r2-smoke.spec.ts`

## Implemented Evidence

- Added branch geometry and branch side-specific modifier fields to the
  product-physics preview component inputs.
- Added unit validation and unit normalization for branch run/header sizes,
  connection angle, optional reinforcement area, header SIF, branch SIF, and
  flexibility factor.
- Added nonblocking diagnostics for missing branch geometry, invalid/missing
  branch user modifiers, unsupported mechanics interface, and invalid branch
  pipe-to-node mappings.
- Added invented `component:C-120` with user-entered branch values and no
  protected/default factor source.
- Added branch field visibility in the viewport, model view, property inspector,
  model tree, editor contract, missing-data/rule-check/validation panels,
  native-package witnesses, and report exports.

## Validation

- `cargo test --manifest-path core/product_physics/Cargo.toml` - passed 33/33
  unit tests.
- `npm test --workspace apps/desktop -- App.test.tsx` - passed 57/57 tests.
- `npm test --workspace apps/desktop` - passed 19/19 files, 406/406 tests.
- `npm run build:desktop` - passed; retained the existing Vite chunk-size
  warning.
- `npm run test:e2e:desktop` - passed 18/18 Playwright tests.

## Boundary

No lifecycle state, review disposition, DAG pointer, dependency register,
release-readiness claim, professional approval, certification, sealing,
authentication, or code-compliance claim changed. No protected standards
content, populated code table, private project data, network path, telemetry
feature, frame-stiffness behavior change, or repository-default private-data
write was introduced.

## Residual

D2 branch-object app absorption is landed for the invented preview path. The
PRD §16.2 branch-assembly benchmark remains D9 validation evidence. Broader
Phase D work remains: D3 rigid/semi-rigid components, D4 expansion joints,
D5 spring-hanger scope gate, D6 nonlinear solve, D8 broader component
provenance reporting, and D9 R4 validation evidence.
