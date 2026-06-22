# WORKING_ITEMS Run Record - TP-R4-D4-EJMACRO-001

Date: 2026-06-22
Agent: WORKING_ITEMS
Deliverable: DEL-04-01 - 3D frame stiffness kernel
Package: PKG-04 - Solver Core and Numerical Methods
Tranche: TP-R4-D4-EJMACRO-001
Target stage: R4 / Phase D

## Scope

Implemented the `DEC-045` D4 expansion-joint user-stiffness macro-element
mechanics slice. The frame kernel now has an explicit two-node
`UserStiffnessElement` that consumes caller-supplied axial, lateral, angular,
and torsional stiffness values and assembles them into the global stiffness
matrix beside ordinary frame elements.

The product-preview adapter now builds that user-stiffness element for the
invented expansion joint `component:C-150` mapped to `pipe:P-130`, feeds it
through both the linear preview solve and the assembled nonlinear active-set
loop, and regenerates the public invented mechanics fixture. Review rows now
state that the user-entered EJ stiffness is consumed by the assembled
user-stiffness macro-element.

## Files Touched

- `core/solver/frame_kernel/src/lib.rs`
- `core/solver/nonlinear_integration/src/lib.rs`
- `core/product_physics/src/lib.rs`
- `validation/benchmarks/nonlinear/src/lib.rs`
- `fixtures/product_preview/invented_mechanics_result.json`
- `tests/product_preview/test_product_preview_service.py`
- `apps/desktop/src/App.test.tsx`
- `apps/desktop/src/services/previewService.test.ts`
- `apps/desktop/e2e/r2-smoke.spec.ts`
- `apps/desktop/src-tauri/src/lib.rs`
- `apps/desktop/src-tauri/Cargo.lock`
- `apps/desktop/SMOKE.md`

## Implemented Evidence

- Added `UserStiffnessElement` to `open_pipe_stress_frame_kernel` with finite
  positive stiffness validation, orientation handling, local relative-spring
  stiffness terms, and global assembly through
  `assemble_global_stiffness_with_user_elements`.
- Added frame-kernel tests proving directional user-stiffness mapping and
  global-solve participation.
- Carried `user_stiffness_elements` through `NonlinearFrameSolveInput`, so the
  assembled nonlinear support loop uses the same explicit macro-element
  stiffness matrix.
- Built expansion-joint user-stiffness elements in `core/product_physics` from
  the existing `mechanics_geometry_and_user_flexibility` interface, mapped
  pipe endpoint, pipe orientation, and user-entered stiffness quantities.
- Regenerated `fixtures/product_preview/invented_mechanics_result.json`; the
  EJ stiffness changes the invented preview displacement/reaction values and
  removes the prior high-displacement warning from that fixture.
- Updated desktop, Python, Tauri, and smoke expectations to the regenerated
  public fixture and replacement diagnostics context.

## Validation

- `cargo test --manifest-path projects/chirality-piping/core/solver/frame_kernel/Cargo.toml` -
  passed 36/36 unit tests.
- `cargo test --manifest-path projects/chirality-piping/core/solver/nonlinear_integration/Cargo.toml` -
  passed 10/10 unit tests.
- `cargo test --manifest-path projects/chirality-piping/core/product_physics/Cargo.toml` -
  passed 43/43 unit tests.
- `cargo test --manifest-path projects/chirality-piping/validation/benchmarks/nonlinear/Cargo.toml` -
  passed 10/10 unit tests.
- `python3 -m pytest tests/product_preview/test_product_preview_service.py` -
  passed 9/9 tests.
- `cargo test --manifest-path projects/chirality-piping/apps/desktop/src-tauri/Cargo.toml agent_proposal_preserves_selected_review_target_without_mutation_or_claims` -
  passed 1/1 selected test.
- `npm test --workspace apps/desktop -- previewService.test.ts App.test.tsx` -
  passed 67/67 targeted Vitest tests.
- `npm run test:e2e --workspace apps/desktop -- --grep "diagnostic detail exposes linked result unit context"` -
  passed 2/2 Playwright project instances after the replacement diagnostic
  selector was made robust against panel overlap.
- `npm run test:e2e --workspace apps/desktop -- --grep "R2 desktop preview smoke covers solve, results, report, and viewport overlay"` -
  passed 2/2 Playwright project instances after the viewport max-displacement
  expectation was updated to the regenerated fixture.

## Boundary

This tranche consumes only user-entered, public invented preview stiffness
values. It does not introduce protected standards content, manufacturer
defaults, catalog defaults, private project data, network behavior, telemetry,
rule/code allowables, release readiness, certification, sealing,
authentication, professional approval, or code-compliance claims.

## Residual

D4 assembled EJ stiffness mechanics is landed for the invented preview path.
Pressure-thrust load generation remains follow-on load-side work. Broader R4
residuals also remain: force/displacement or energy threshold promotion beyond
active-set-count policy, product-preview non-active-set threshold axes, deeper
multi-DOF / multi-support validation fixtures, spring-hanger solve behavior,
and profile-direct/default sparse promotion.
