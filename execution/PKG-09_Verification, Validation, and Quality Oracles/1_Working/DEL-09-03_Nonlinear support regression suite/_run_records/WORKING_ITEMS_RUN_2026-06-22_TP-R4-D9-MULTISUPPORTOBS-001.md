# WORKING ITEMS RUN - TP-R4-D9-MULTISUPPORTOBS-001

Date: 2026-06-22
Agent: WORKING_ITEMS
Package: PKG-09 - Verification, Validation, and Quality Oracles
Deliverable: DEL-09-03 - Nonlinear support regression suite
Target stage: R4 / Phase D

## Objective

Add a bounded multi-DOF / multi-support assembled nonlinear observation fixture
to reduce the R4/D9 validation-depth gap, while keeping non-seed threshold
promotion, displacement/reaction-delta thresholds, energy thresholds, sparse
default behavior, external validation thresholds, release thresholds, and R4
exit readiness out of scope.

## Authority Basis

- PRD §22.5: R4 requires nonlinear support validation cases to converge.
- `DEC-044`: the assembled nonlinear solve is owned by the PKG-04 integration
  tranche bridging DEL-04-04 and DEL-04-01; the classifier remains the
  per-iteration state oracle.
- `DEC-046`: class-tiered convergence policy can promote measured entries, but
  unmeasured/non-seed entries remain `TBD`.
- `TP-R4-D9-EXITGAP-001`: records multi-DOF / multi-support nonlinear fixture
  depth as an R4/D9 residual.

## Changes

- Added `NL-ASSEMBLED-MULTI-DOF-MULTI-SUPPORT-OBS-ORIGINAL`, an invented
  two-node assembled frame fixture with a Ux one-way support and Uy gap support
  changing state in the same first iteration.
- Exposed separate APIs:
  `assembled_multisupport_depth_inventory()`,
  `assembled_multisupport_depth_convergence_observations()`, and
  `assembled_multisupport_depth_residual_observations()`.
- Kept the fixture outside `assembled_fixture_inventory()` and attached
  `TP-R4-D9-MULTISUPPORT-OBS-TBD` with `ConvergencePolicyStatus::Tbd`; the
  final residual observations do not cite the accepted free-DOF force/moment
  threshold policy.
- Added `validation/hand_calcs/nonlinear/assembled_multi_support_multi_dof.md`
  plus README and Python guardrail coverage.
- Updated the completion plan, completion log, strategic roadmap,
  coordination record, and R4 gap packet to reflect observation-only depth
  evidence without closing R4 readiness.

## Validation

Focused validation:

- `cargo fmt --manifest-path validation/benchmarks/nonlinear/Cargo.toml` -
  passed.
- `cargo test --manifest-path validation/benchmarks/nonlinear/Cargo.toml` -
  passed, 12 tests.
- `python3 -m pytest -q tests/test_nonlinear_support_regression.py` - passed,
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
  `validation/evidence/sweeps/SWEEP_20260622T071252Z_00effc54b1ad-dirty.json`
- `git diff --check` - passed.

## Boundaries

- This tranche adds observation-only validation depth. It does not promote
  non-seed force/displacement/energy thresholds, displacement-delta thresholds,
  reaction-delta thresholds, sparse live/default thresholds, product-preview
  thresholds, release thresholds, or external validation thresholds.
- It does not add protected standards content, private project data, hidden
  support defaults, lifecycle transition, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim.

## Residuals

- Remaining D6/D9 work includes non-seed force/displacement/energy thresholds,
  displacement/reaction-delta threshold axes, broader multi-support acceptance
  coverage, deeper spring-hanger behavior, profile-direct sparse
  assembly/default sparse promotion, external validation threshold evidence,
  and final R4 exit-chain evidence.
