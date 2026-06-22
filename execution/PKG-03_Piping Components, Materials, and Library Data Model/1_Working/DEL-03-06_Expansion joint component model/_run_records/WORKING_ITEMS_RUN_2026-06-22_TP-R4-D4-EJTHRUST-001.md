# WORKING_ITEMS Run Record - TP-R4-D4-EJTHRUST-001

Date: 2026-06-22
Agent: WORKING_ITEMS
Deliverable: DEL-03-06 - Expansion joint component model
Package: PKG-03 - Piping Components, Materials, and Library Data Model
Tranche: TP-R4-D4-EJTHRUST-001
Target stage: R4 / Phase D

## Scope

Landed the `DEC-045` D4 expansion-joint pressure-thrust follow-on for the
invented preview path. The product-physics preview now generates load-side
pressure-thrust result rows for an expansion-joint component when all required
inputs are explicit: a component mapped to the pressured pipe, positive finite
user-entered effective area, and named pressure load-case primitives.

For `component:C-150` mapped to `pipe:P-130`, the generated rows use
`effective_area = 0.018 m^2`; `load:L-100-P-EJ` at `1.2 MPa` produces
`21600 N`, `load:L-200-P-EJ` at `0.6 MPa` produces `10800 N`, and the
`C-OPER-ALT` load combination produces `27000 N`.

## Files Touched

- `core/product_physics/src/lib.rs`
- `core/product_physics/src/validation.rs`
- `fixtures/product_preview/invented_preview_model.json`
- `fixtures/product_preview/invented_mechanics_result.json`
- `apps/desktop/src/features/report/ReportPanel.tsx`
- `apps/desktop/src/App.test.tsx`
- `apps/desktop/e2e/r2-smoke.spec.ts`
- `tests/product_preview/test_product_preview_service.py`
- `plans/PLAN_2026-06-17_prd_completion.md`
- `plans/PLAN_COMPLETION_LOG.md`
- `execution/_Coordination/_COORDINATION.md`
- `execution/_Coordination/NEXT_INSTANCE_PROMPT.md`
- DEL-03-06 / DEL-05-01 / DEL-08-03 memory and run-record surfaces

## Implemented Evidence

- Added expansion-joint pressure-thrust source metadata that records component
  id, pipe id, effective-area value/unit/provenance, pressure load id,
  load-case id, pressure value/unit, solver-consumption mode, and boundary
  note.
- Kept ordinary pipe-internal-area pressure-thrust as the fallback path when no
  eligible expansion-joint component mapping is present.
- Added invented EJ-mapped pressure primitives to the public preview model for
  `L-100` and `L-200`; both carry invented provenance and explicit pressure
  units.
- Regenerated the public mechanics fixture to include primary load-case and
  combination pressure-thrust rows, a summary count of two primary
  component-pressure-thrust loads, and
  `EXPANSION_JOINT_PRESSURE_THRUST_APPLIED` diagnostics.
- Added report packet/export evidence for component pressure thrust, including
  selected result refs and `pressure_thrust_result_refs` in component
  provenance.
- Added Rust, Python, and desktop assertions for the pressure-thrust rows,
  source metadata, diagnostics, report lines, export fields, and regenerated
  fixture counts.

## Validation

- `cargo fmt --manifest-path core/product_physics/Cargo.toml` - passed.
- Fixture JSON parse checks for `invented_preview_model.json` and
  `invented_mechanics_result.json` - passed.
- `cargo test --manifest-path core/product_physics/Cargo.toml` - passed
  44/44 tests.
- `npm run generate:product-preview-mechanics` - passed.
- `python3 -m pytest tests/product_preview/test_product_preview_service.py` -
  passed 9/9 tests.
- `npm test --workspace apps/desktop` - passed 19/19 files and 407/407 tests.
- `PLAYWRIGHT_WORKERS=1 npm run test:e2e --workspace apps/desktop -- --grep
  "R2 desktop preview smoke covers solve, results, report, and viewport
  overlay" --workers=1` - passed 2/2 Playwright project instances.
- Full DEC-025 evidence sweep - passed 5/5 surfaces:
  `validation/evidence/sweeps/SWEEP_20260622T063558Z_d3f658288543-dirty.json`.

## Boundary

This tranche uses only invented/user-entered preview data: explicit effective
area, explicit pressure load primitives, and explicit component/pipe mapping.
It does not introduce protected standards content, code-derived pressure-thrust
coefficients, manufacturer/catalog defaults, private project data, network
behavior, telemetry, release readiness, certification, sealing, authentication,
professional approval, or code-compliance claims.

## Residual

D4 pressure-thrust load generation is landed for the invented preview path.
Broader R4 residuals remain: force/displacement or energy threshold promotion
beyond active-set-count policies, product-preview non-active-set threshold axes,
deeper multi-DOF / multi-support validation fixtures, spring-hanger solve
behavior, profile-direct/default sparse promotion, and final R4 exit-chain
evidence.
