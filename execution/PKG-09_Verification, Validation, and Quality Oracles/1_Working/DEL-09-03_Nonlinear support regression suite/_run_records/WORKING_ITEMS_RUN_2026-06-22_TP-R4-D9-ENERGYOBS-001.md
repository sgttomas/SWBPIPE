# WORKING ITEMS RUN - TP-R4-D9-ENERGYOBS-001

Date: 2026-06-22
Agent: WORKING_ITEMS
Package: PKG-09 - Verification, Validation, and Quality Oracles
Deliverable: DEL-09-03 - Nonlinear support regression suite
Target stage: R4 / Phase D

## Objective

Add a bounded observation-only free-DOF work residual axis for the assembled
nonlinear loop, reducing the R4/D9 energy/work evidence gap without promoting a
new threshold policy or changing release readiness.

## Authority Basis

- PRD §22.5: R4 requires nonlinear support validation cases to converge.
- `DEC-044`: the assembled nonlinear solve is owned by the PKG-04 integration
  tranche bridging DEL-04-04 and DEL-04-01; the classifier remains the
  per-iteration state oracle.
- `DEC-046`: class-tiered convergence policy can promote measured entries, but
  unmeasured/non-seed entries remain `TBD`.
- `TP-R4-D9-EXITGAP-001`: records force/displacement/energy threshold axes and
  broader D9 validation evidence as R4 residuals.

## Changes

- Added `max_abs_free_dof_work_residual` to
  `core/solver/nonlinear_integration::NonlinearResidualObservation`, computed
  as the max absolute free-DOF `reaction * displacement` residual product in
  the final linearized solve.
- Exposed the same observation through
  `validation/benchmarks/nonlinear::ForceDisplacementResidualObservation` with
  `free_dof_work_threshold_policy = None`.
- Added product-preview result rows with kind
  `nonlinear_support_observed_free_dof_work_residual`, excluded from load-case
  combination algebra and marked `threshold=TBD`.
- Updated nonlinear hand-calculation notes, policy limitation text,
  coordination/planning records, and Python regression guards to distinguish
  observation-only work residual evidence from accepted free-DOF force/moment
  residual thresholds.

## Validation

Focused validation:

- `cargo fmt --manifest-path core/solver/nonlinear_integration/Cargo.toml --check` -
  passed.
- `cargo fmt --manifest-path validation/benchmarks/nonlinear/Cargo.toml --check` -
  passed.
- `cargo fmt --manifest-path core/product_physics/Cargo.toml --check` - passed.
- `cargo test --manifest-path core/solver/nonlinear_integration/Cargo.toml` -
  passed, 10 tests.
- `cargo test --manifest-path validation/benchmarks/nonlinear/Cargo.toml` -
  passed, 12 tests.
- `cargo test --manifest-path core/product_physics/Cargo.toml` - passed, 44
  tests.
- `python3 -m pytest -q tests/test_nonlinear_support_regression.py` - passed, 8
  tests.

Closeout validation:

- `python3 tools/release/run_evidence_sweep.py --execute` - passed all five
  DEC-025 surfaces:
  - cargo crate sweep: 33 manifests passed;
  - repository pytest: 362 passed;
  - desktop Vitest: 407 passed;
  - Playwright dev/dist: 18 + 1 passed;
  - desktop production build: passed.
- Passing sweep summary:
  `validation/evidence/sweeps/SWEEP_20260622T072805Z_be55844f721d-dirty.json`
- `git diff --check` - passed.

## Boundaries

- This tranche records work residual observations only. It does not promote a
  free-DOF work/energy threshold, non-seed force/displacement threshold,
  displacement-delta threshold, reaction-delta threshold, sparse default
  threshold, product-preview release threshold, or external validation
  threshold.
- It does not add protected standards content, private project data, hidden
  support defaults, lifecycle transition, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim.

## Residuals

- Remaining D6/D9 work includes non-seed force/displacement and free-DOF
  work/energy threshold promotion, displacement/reaction-delta threshold axes,
  broader multi-support acceptance coverage, deeper spring-hanger behavior,
  profile-direct sparse assembly/default sparse promotion, external validation
  threshold evidence, and final R4 exit-chain evidence.
