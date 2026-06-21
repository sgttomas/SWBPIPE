# WORKING ITEMS RUN - TP-R4-D9-FORCEDISPRESID-001

Date: 2026-06-21
Agent: WORKING_ITEMS
Package: PKG-09 - Verification, Validation, and Quality Oracles
Deliverable: DEL-09-03 - Nonlinear support regression suite
Target stage: R4 / Phase D

## Objective

Record force/displacement residual observations for the current dense
assembled nonlinear loop and product-preview surface without promoting any new
threshold policy beyond the accepted `DEC-046` active-set-count validation-seed
policy.

## Authority Basis

- `DEC-044`: assembled nonlinear solve is owned by a PKG-04 integration
  tranche bridging DEL-04-04 and DEL-04-01; the classifier remains the
  per-iteration state oracle.
- `DEC-046`: class-tier convergence policy may promote only measured entries;
  unmeasured/non-seed entries remain `TBD`.
- PRD 22.5: R4 requires nonlinear support validation cases converge.
- `plans/PLAN_2026-06-17_prd_completion.md` D6/D9 residual: force/displacement
  residual evidence remained outside the accepted active-set-count policy.

## Changes

- Added `NonlinearResidualObservation` to
  `core/solver/nonlinear_integration`, recording active-set changed-support
  count, max displacement/reaction deltas from the previous iteration, and
  final free-DOF force/moment residuals.
- Added product-preview result rows for observed max translation/rotation
  delta, observed max force/moment reaction delta, and observed free-DOF
  force/moment residual. These rows are excluded from load-combination
  arithmetic and carry `observed_residual_only; threshold=TBD` basis text.
- Added `ForceDisplacementResidualObservation` and
  `assembled_force_displacement_residual_observations()` to the nonlinear
  benchmark crate, with `threshold_policy: None`.
- Updated nonlinear hand-calc notes to include the current assembled-fixture
  force/displacement residual observations and the explicit no-threshold
  boundary.
- Updated focused Python regression coverage, this deliverable memory, the
  completion plan, and the current roadmap summary.

## Validation

Focused validation:

- `cargo test --manifest-path core/solver/nonlinear_integration/Cargo.toml` -
  passed, 10 tests.
- `cargo test --manifest-path core/product_physics/Cargo.toml` - passed, 41
  tests.
- `cargo test --manifest-path validation/benchmarks/nonlinear/Cargo.toml` -
  passed, 10 tests.
- `python3 -m pytest tests/test_nonlinear_support_regression.py -q` - passed,
  8 tests.

Closeout validation:

- `python3 tools/release/run_evidence_sweep.py --execute` - passed all five
  DEC-025 surfaces:
  - cargo crate sweep: 33 manifests passed;
  - repository pytest: 362 passed;
  - desktop Vitest: 407 passed;
  - Playwright dev/dist: 18 + 1 passed;
  - desktop production build: passed.
- Passing sweep summary:
  `validation/evidence/sweeps/SWEEP_20260621T111451Z_eaf10ef3c49a-dirty.json`
- `git diff --check` - passed.

## Boundaries

- This tranche records observations only. It does not create or promote a
  force/displacement residual threshold.
- The accepted governed convergence policy remains
  `DEC-046-CV-B-active-set-count-validation-v1` for active-set
  changed-support-count validation-seed evidence only.
- Product-preview convergence thresholds, sparse live-path behavior, external
  validation thresholds, and R4 exit-chain readiness remain outside this
  tranche.
- No protected standards content, private project data, hidden support
  defaults, lifecycle transition, release-readiness claim, professional
  approval, certification, sealing, authentication, or code-compliance claim
  was added.

## Residuals

- `D-15` spring-hanger scope remains awaiting ruling unless separately ruled.
- `D-17` sparse live-path adoption remains awaiting ruling unless separately
  ruled.
- Remaining unblocked D6/D9 work is limited to observation/evidence surfaces
  that do not depend on those gates; threshold promotion requires additional
  measured evidence and/or human policy ruling.
