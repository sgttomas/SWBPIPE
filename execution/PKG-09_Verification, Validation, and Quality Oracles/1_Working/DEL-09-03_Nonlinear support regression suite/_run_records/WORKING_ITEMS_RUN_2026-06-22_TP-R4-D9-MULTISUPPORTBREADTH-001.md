# WORKING ITEMS RUN - TP-R4-D9-MULTISUPPORTBREADTH-001

Date: 2026-06-22
Agent: WORKING_ITEMS
Package: PKG-09 - Verification, Validation, and Quality Oracles
Deliverable: DEL-09-03 - Nonlinear support regression suite
Target stage: R4 / Phase D

## Objective

Broaden the accepted multi-support nonlinear validation companion evidence by
adding a second public-original multi-DOF fixture under the existing narrow
`DEC-046` active-set, free-DOF force/moment, and free-DOF work policies.

## Authority Basis

- PRD §22.5: R4 requires nonlinear support validation cases to converge.
- `DEC-044`: the assembled nonlinear solve is owned by the PKG-04 integration
  tranche; the nonlinear support classifier remains the per-iteration state
  oracle.
- `DEC-046`: class-tiered convergence policy can promote measured entries, but
  unmeasured/non-seed entries remain `TBD`.
- `TP-R4-D9-MULTISUPPORTPOLICY-001`: added the first accepted Ux/Uy
  one-way/gap multi-support companion fixture.
- `TP-R4-D9-WORKPOLICY-001`: added bounded free-DOF work residual policies for
  the current assembled seed, product-preview surface, and accepted
  multi-support companion.

## Changes

- Added `NL-ASSEMBLED-MULTI-DOF-GAP-LIFT-OFF-ACCEPTED-ORIGINAL`, an invented
  two-node assembled frame fixture with simultaneous Ux lift-off release and Uy
  gap closure.
- Extended `assembled_multisupport_acceptance_inventory()` and corresponding
  convergence/residual observations to include both accepted multi-support
  companion fixtures.
- Updated the multi-support active-set-count, free-DOF force/moment, and
  free-DOF work JSON policy records so the evidence fixture IDs name the
  two-fixture public-original set.
- Added the hand-calculation/provenance note
  `validation/hand_calcs/nonlinear/assembled_multi_support_gap_lift_off_acceptance.md`
  and updated the hand-calc/benchmark indexes.
- Updated nonlinear regression guardrails, coordination text, tactical plan
  rows, strategic plan text, the R4 gap packet, completion log, and DEL-09-03
  memory.

## Validation

Focused validation:

- `cargo fmt --manifest-path validation/benchmarks/nonlinear/Cargo.toml --check` -
  passed.
- `cargo test --manifest-path validation/benchmarks/nonlinear/Cargo.toml` -
  passed, 15 tests.
- `python3 -m pytest tests/test_nonlinear_support_regression.py -q` - passed,
  8 tests.
- JSON validation passed for
  `validation/benchmarks/nonlinear/multisupport_convergence_policy.dec046.json`,
  `validation/benchmarks/nonlinear/multisupport_free_dof_force_moment_policy.dec046.json`,
  and `validation/benchmarks/nonlinear/multisupport_free_dof_work_policy.dec046.json`.
- `git diff --check` - passed.

Closeout validation:

- `python3 tools/release/run_evidence_sweep.py --execute` - passed all five
  DEC-025 surfaces:
  - cargo crate sweep;
  - repository pytest;
  - desktop Vitest after wasm build;
  - Playwright dev/dist desktop lanes;
  - desktop production build.
- Passing sweep summary:
  `validation/evidence/sweeps/SWEEP_20260622T083239Z_5a955e5e84b1-dirty.json`

## Boundaries

- This tranche broadens only the accepted public-original multi-support
  companion fixture set from one fixture to two fixtures.
- It does not promote non-seed force/displacement thresholds beyond the
  accepted fixture set, displacement-delta thresholds, reaction-delta
  thresholds, general energy thresholds, sparse-default behavior,
  product-preview release thresholds, external validation thresholds, or CI
  thresholds.
- It does not add protected standards content, private project data, hidden
  support defaults, lifecycle transition, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim.

## Residuals

- Remaining D6/D9 work includes profile-direct sparse assembly/default sparse
  promotion, non-seed force/displacement threshold promotion beyond the
  accepted fixture set, general energy threshold promotion,
  displacement/reaction-delta threshold axes, deeper spring-hanger behavior,
  external validation threshold evidence, broader R4 validation packaging, and
  final R4 exit-chain evidence.
