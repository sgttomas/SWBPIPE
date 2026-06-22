# WORKING ITEMS RUN - TP-R4-D9-MULTISUPPORTFRICTION-001

Date: 2026-06-22
Agent: WORKING_ITEMS
Package: PKG-09 - Verification, Validation, and Quality Oracles
Deliverable: DEL-09-03 - Nonlinear support regression suite
Target stage: R4 / Phase D

## Objective

Broaden the accepted multi-support nonlinear validation companion evidence by
adding a third public-original multi-DOF fixture that includes friction behavior
under the existing narrow `DEC-046` active-set, free-DOF force/moment, and
free-DOF work policies.

## Authority Basis

- PRD §22.5: R4 requires nonlinear support validation cases to converge.
- `DEC-044`: the assembled nonlinear solve is owned by the PKG-04 integration
  tranche; the nonlinear support classifier remains the per-iteration state
  oracle.
- `DEC-046`: class-tiered convergence policy can promote measured entries, but
  unmeasured/non-seed entries remain `TBD`.
- `TP-R4-D9-MULTISUPPORTPOLICY-001`: added the first accepted Ux/Uy one-way/gap
  multi-support companion fixture.
- `TP-R4-D9-MULTISUPPORTBREADTH-001`: added the accepted lift-off/gap
  multi-support companion fixture.
- `TP-R4-D9-WORKPOLICY-001`: added bounded free-DOF work residual policies for
  the current assembled seed, product-preview surface, and accepted
  multi-support companion set.

## Changes

- Added `NL-ASSEMBLED-MULTI-DOF-FRICTION-GAP-ACCEPTED-ORIGINAL`, an invented
  two-node assembled frame fixture with simultaneous Ux friction sliding and Uy
  gap closure.
- Extended `assembled_multisupport_acceptance_inventory()` and corresponding
  convergence/residual observations to include three accepted multi-support
  companion fixtures.
- Updated the multi-support active-set-count, free-DOF force/moment, and
  free-DOF work JSON policy records so the evidence fixture IDs name the
  three-fixture public-original set.
- Added the hand-calculation/provenance note
  `validation/hand_calcs/nonlinear/assembled_multi_support_friction_gap_acceptance.md`
  and updated the hand-calc/benchmark indexes.
- Updated nonlinear regression guardrails, coordination text, tactical plan
  rows, strategic plan text, the R4 gap packet, completion log, and DEL-09-03
  memory.

## Validation

Focused validation:

- `cargo test --quiet` in `validation/benchmarks/nonlinear` - passed, 15 tests.
- `python3 -m pytest tests/test_nonlinear_support_regression.py` - passed,
  8 tests.
- `cargo fmt --check` in `validation/benchmarks/nonlinear` - passed after
  formatting.
- Full DEC-025 evidence sweep:
  `python3 tools/release/run_evidence_sweep.py --execute` - passed all five
  surfaces:
  - cargo crate sweep;
  - repository pytest;
  - desktop Vitest after wasm build;
  - Playwright dev/dist desktop lanes;
  - desktop production build.
- Passing sweep summary:
  `validation/evidence/sweeps/SWEEP_20260622T090832Z_9b32ef0b35d0-dirty.json`

## Boundaries

- This tranche broadens only the accepted public-original multi-support
  companion fixture set from two fixtures to three fixtures.
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

- Remaining D6/D9 work includes default sparse promotion, nonlinear/core
  profile-direct sparse promotion, non-seed force/displacement threshold
  promotion beyond the accepted fixture set, general energy threshold
  promotion, displacement/reaction-delta threshold axes, deeper spring-hanger
  behavior, external validation threshold evidence, broader R4 validation
  packaging, and final R4 exit-chain evidence.
