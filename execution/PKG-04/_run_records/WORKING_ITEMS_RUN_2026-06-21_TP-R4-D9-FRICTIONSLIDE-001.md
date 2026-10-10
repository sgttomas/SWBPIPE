# WORKING_ITEMS Run Record - TP-R4-D9-FRICTIONSLIDE-001

Date: 2026-06-21
Timestamp: 2026-06-21T02:58:38-06:00
Agent: WORKING_ITEMS
Target stage: R4 / Phase D
Tranche: TP-R4-D9-FRICTIONSLIDE-001
Decision basis: `DEC-044`, `DEC-046`

## Objective

Continue D6/D9 residual work by adding explicit-normal sliding-friction
assembled and product-preview coverage. The tranche proves deterministic
sliding-state convergence for an invented friction support with user-entered
normal reaction input.

This is a partial D6/D9 landing. It does not implement a derived normal-force
model, set measured class-tiered convergence values, bind the sparse solver to
the live path, or claim R4 exit validation.

## Changes

- Added a deterministic active-set anti-chatter rule in
  `core/solver/nonlinear_supports`: a previously sliding friction support
  remains `Sliding` through a released DOF while nonzero trial displacement
  persists.
- Added an assembled dense-loop friction sliding regression in
  `core/solver/nonlinear_integration`.
- Added `NL-ASSEMBLED-FRICTION-SLIDE-ORIGINAL` to the nonlinear benchmark
  inventory with an explicit invented normal reaction and visible
  `TOLERANCE_POLICY_TBD`.
- Added product-preview friction support `support:NL-130-FRIC` and regenerated
  the invented mechanics result fixture. The preview result envelope now
  exposes state-code, displacement, zero released reaction, and explicit normal
  reaction evidence rows for the sliding support.
- Updated desktop/unit/e2e tests and export counts for the expanded model and
  783-row result envelope.

## Validation

Passed:

- `cargo fmt --manifest-path core/solver/nonlinear_supports/Cargo.toml -- --check`
- `cargo fmt --manifest-path core/solver/nonlinear_integration/Cargo.toml -- --check`
- `cargo fmt --manifest-path core/product_physics/Cargo.toml -- --check`
- `cargo fmt --manifest-path validation/benchmarks/nonlinear/Cargo.toml -- --check`
- `cargo test --manifest-path core/solver/nonlinear_supports/Cargo.toml`
  - 17 tests
- `cargo test --manifest-path core/solver/nonlinear_integration/Cargo.toml`
  - 8 tests
- `cargo test --manifest-path validation/benchmarks/nonlinear/Cargo.toml`
  - 7 tests
- `cargo test --manifest-path core/product_physics/Cargo.toml`
  - 39 tests
- `python3 -m pytest tests/product_preview/test_product_preview_service.py tests/test_nonlinear_support_regression.py tests/test_results_schema.py tests/test_analysis_run_records.py`
  - 28 tests
- `npm test --workspace apps/desktop -- previewService App.test.tsx`
  - 67 tests
- `npm run build --workspace apps/desktop`
- `npm run test:e2e --workspace apps/desktop -- --workers=1 --grep "R2 desktop preview smoke covers"`
  - 2 Playwright tests
- `git diff --check`
- `python3 tools/release/run_evidence_sweep.py --execute`
  - overall pass across cargo crate sweep, Python pytest, desktop Vitest,
    desktop Playwright dev/dist, and desktop production build
  - summary:
    `validation/evidence/sweeps/SWEEP_20260621T085427Z_93a25e03201f-dirty.json`

## Boundaries

- Explicit invented normal-reaction input only; no derived friction normal-force
  model and no catalog/default normal-force source.
- Dense active-set loop only; sparse live-path adoption remains gated by
  `D-17`.
- No measured class-tiered convergence values; unmeasured `DEC-046` entries
  remain `TBD`.
- No lifecycle transition, release-readiness claim, professional approval,
  certification, sealing, authentication, code-compliance claim, protected
  standards content, public default, or private data was introduced.

## Residuals

- Derived friction normal-force modeling remains open.
- Measured class-tiered convergence values remain `TBD` until governed evidence
  exists.
- Sparse live-path adoption remains gated by `D-17`.
- Broader live-solver coverage, the PRD section 16.2 branch-assembly benchmark,
  and the R4 exit evidence package remain open.
