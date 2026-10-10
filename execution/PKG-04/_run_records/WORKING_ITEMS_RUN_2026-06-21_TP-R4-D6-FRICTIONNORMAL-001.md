# WORKING_ITEMS Run Record - TP-R4-D6-FRICTIONNORMAL-001

Date: 2026-06-21
Timestamp: 2026-06-21T03:22:54-06:00
Agent: WORKING_ITEMS
Target stage: R4 / Phase D
Tranche: TP-R4-D6-FRICTIONNORMAL-001
Decision basis: `DEC-044` and `DEC-046`

## Objective

Continue D6/D9 residual work by replacing the open friction normal-force
placeholder with a bounded, explicit-source model: a friction support may name
a restrained linear support DOF as its normal-reaction source, and the dense
assembled active-set loop derives the normal-reaction magnitude from the
absolute reaction at that DOF.

This tranche does not bind the sparse solver, set measured release convergence
values, implement spring hangers, add protected/catalog/default normal forces,
or claim R4 exit validation.

## Changes

- Added `DerivedFrictionNormalReaction` to `core/solver/nonlinear_integration`.
- Extended `NonlinearFrameSolveInput` with
  `derived_friction_normal_reactions`.
- Updated the dense active-set trial-state path so friction supports use either:
  - explicit `FrictionNormalReaction` evidence; or
  - an absolute reaction at a caller-named support-normal DOF.
- Added validation that rejects duplicate explicit and derived normal sources
  for the same friction support.
- Preserved dense-loop assumptions and limitations around public invented
  evidence, `DEC-046` `TBD` tolerance policy, and no release/professional
  reliance claim.

## Validation

Passed:

- `cargo fmt --manifest-path core/solver/nonlinear_integration/Cargo.toml -- --check`
- `cargo fmt --manifest-path core/product_physics/Cargo.toml -- --check`
- `cargo fmt --manifest-path validation/benchmarks/nonlinear/Cargo.toml -- --check`
- `cargo test --manifest-path core/solver/nonlinear_integration/Cargo.toml`
  - 10 tests
- `cargo test --manifest-path core/product_physics/Cargo.toml`
  - 40 tests
- `cargo test --manifest-path validation/benchmarks/nonlinear/Cargo.toml`
  - 7 tests
- `python3 -m pytest tests/product_preview/test_product_preview_service.py tests/test_nonlinear_support_regression.py tests/test_results_schema.py tests/test_analysis_run_records.py`
  - 28 tests
- `npm test --workspace apps/desktop -- previewService App.test.tsx`
  - 67 tests
- `npm run build:desktop`
- `python3 tools/release/run_evidence_sweep.py --execute`
  - overall pass across 33 cargo crates, repository pytest, desktop Vitest,
    desktop Playwright dev/dist, and desktop production build
  - summary:
    `validation/evidence/sweeps/SWEEP_20260621T092312Z_53b592aee006-dirty.json`
- `git diff --check`

## Boundaries

- Explicit public invented source DOF only; no protected standards content,
  proprietary catalog value, public support default, private data, network or
  telemetry feature, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim.
- Dense loop only; no sparse live-path binding.
- No measured class-tiered convergence values; unmeasured `DEC-046` entries
  remain `TBD`.
- No lifecycle transition or R4 exit claim.

## Residuals

- Measured class-tiered convergence values remain `TBD` until governed evidence
  exists.
- Sparse live-path adoption remains gated by `D-17`.
- Broader live-solver coverage, the PRD section 16.2 branch-assembly
  benchmark, and the R4 exit evidence package remain open.
