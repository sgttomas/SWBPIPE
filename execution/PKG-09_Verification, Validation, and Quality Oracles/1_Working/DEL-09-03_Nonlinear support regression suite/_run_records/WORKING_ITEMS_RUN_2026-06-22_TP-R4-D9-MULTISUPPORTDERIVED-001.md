# WORKING ITEMS RUN - TP-R4-D9-MULTISUPPORTDERIVED-001

Date: 2026-06-22
Agent: WORKING_ITEMS
Package: PKG-09 - Verification, Validation, and Quality Oracles
Deliverable: DEL-09-03 - Nonlinear support regression suite
Target stage: R4 / Phase D

## Objective

Broaden the accepted multi-support nonlinear validation companion evidence by
adding a sixth public-original multi-DOF fixture that combines derived-normal
friction sliding with gap closure under the existing narrow `DEC-046`
active-set, free-DOF force/moment, and free-DOF work policies.

## Authority Basis

- PRD §22.5: R4 requires nonlinear support validation cases to converge.
- `DEC-044`: the assembled nonlinear solve is owned by the PKG-04 integration
  tranche; the nonlinear support classifier remains the per-iteration state
  oracle.
- `DEC-046`: class-tiered convergence policy can promote measured entries, but
  unmeasured/non-seed entries remain `TBD`.
- Prior accepted multi-support companions:
  `TP-R4-D9-MULTISUPPORTPOLICY-001`,
  `TP-R4-D9-MULTISUPPORTBREADTH-001`,
  `TP-R4-D9-MULTISUPPORTFRICTION-001`,
  `TP-R4-D9-MULTISUPPORT3DOF-001`, and
  `TP-R4-D9-MULTISUPPORTROT-001`.

## Changes

- Added `NL-ASSEMBLED-MULTI-DOF-DERIVED-NORMAL-GAP-ACCEPTED-ORIGINAL`, an
  invented two-node assembled frame fixture with derived-normal friction
  sliding on Ux and gap closure on Uz.
- Used a separate restrained Uy support-normal source DOF so the derived
  normal reaction is named fixture evidence, not a hidden normal-force default.
- Extended `assembled_multisupport_acceptance_inventory()` and corresponding
  convergence/residual observations to include six accepted multi-support
  companion fixtures.
- Updated multi-support active-set-count, free-DOF force/moment, free-DOF
  work, and displacement/reaction-delta observation JSON records so evidence
  fixture IDs name the six-fixture public-original set.
- Added the hand-calculation/provenance note
  `validation/hand_calcs/nonlinear/assembled_multi_support_derived_normal_gap_acceptance.md`
  and updated hand-calc/benchmark indexes, focused regression guardrails,
  coordination text, plan surfaces, the R4 gap packet, completion log, and
  DEL-09-03 memory.

## Validation

Focused validation:

- `cargo fmt --check --manifest-path validation/benchmarks/nonlinear/Cargo.toml`
  - passed.
- `cargo test --manifest-path validation/benchmarks/nonlinear/Cargo.toml --quiet`
  - passed, 16 tests.
- `python3 -m pytest -q tests/test_nonlinear_support_regression.py` - passed,
  8 tests.
- `jq empty` over the four updated multi-support DEC-046 JSON records - passed.

Full DEC-025 evidence sweep:

- `python3 tools/release/run_evidence_sweep.py --execute` - passed all five
  surfaces:
  - cargo crate sweep;
  - repository pytest, 362 tests;
  - desktop Vitest, 407 tests;
  - Playwright dev/dist desktop lanes, 18 + 1 tests;
  - desktop production build.
- Passing sweep summary:
  `validation/evidence/sweeps/SWEEP_20260622T103936Z_5058ec872d37-dirty.json`

## Boundaries

- This tranche broadens only the accepted public-original multi-support
  companion fixture set from five fixtures to six fixtures.
- It does not promote non-seed force/displacement thresholds beyond the
  accepted six-fixture set, displacement-delta thresholds, reaction-delta
  thresholds, general energy thresholds, sparse-default behavior,
  product-preview release thresholds, external validation thresholds, or CI
  thresholds.
- It does not add protected standards content, private project data, hidden
  support defaults, lifecycle transition, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim.

## Residuals

- Remaining D6/D9 work includes default sparse promotion, large-model sparse
  suitability thresholds, non-seed force/displacement threshold promotion
  beyond the accepted six-fixture set, general energy threshold promotion,
  accepted displacement/reaction-delta thresholds, deeper spring-hanger
  behavior, external validation threshold evidence, broader R4 validation
  packaging, and final R4 exit-chain evidence.
