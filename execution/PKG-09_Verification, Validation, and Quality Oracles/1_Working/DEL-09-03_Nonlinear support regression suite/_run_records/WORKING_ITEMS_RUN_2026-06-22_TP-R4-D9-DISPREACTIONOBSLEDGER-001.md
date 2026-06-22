# WORKING ITEMS RUN - TP-R4-D9-DISPREACTIONOBSLEDGER-001

Date: 2026-06-22
Agent: WORKING_ITEMS
Package: PKG-09 - Verification, Validation, and Quality Oracles
Deliverable: DEL-09-03 - Nonlinear support regression suite
Target stage: R4 / Phase D

## Objective

Refresh the standalone multi-support displacement/reaction-delta observation
ledger so it names the same ten accepted public-original multi-support fixture
IDs as the accepted `DEC-046` displacement/reaction-delta threshold policy, and
add regression coverage to prevent future observation/policy fixture-list drift.

## Authority Basis

- PRD section 22.5: R4 requires nonlinear support validation cases to converge.
- `DEC-046`: class-tiered convergence policy can promote measured entries, but
  unmeasured/non-seed entries remain `TBD`.
- `TP-R4-D9-DISPREACTIONOBS-001` created the observation-only
  displacement/reaction-delta ledgers.
- `TP-R4-D9-DISPREACTIONPOLICY-001` promoted the current assembled seed and
  accepted multi-support fixture-set displacement/reaction-delta threshold
  policies.
- `TP-R4-D9-MULTISUPPORTNEGAP-001` and
  `TP-R4-D9-MULTISUPPORT4CLASS-001` expanded the accepted multi-support fixture
  set to ten fixtures.

## Changes

- Added the negative-direction gap/one-way and four-class fixture IDs to
  `validation/benchmarks/nonlinear/multisupport_displacement_reaction_delta_observation.dec046.json`.
- Added focused Python regression coverage that loads the multi-support
  displacement/reaction observation record and asserts its evidence fixture IDs
  match both the accepted policy record and the expected ten-fixture inventory.

## Validation

Focused validation:

- `python3 -m pytest -q tests/test_nonlinear_support_regression.py` - passed,
  8 tests.
- `python3 -m json.tool validation/benchmarks/nonlinear/multisupport_displacement_reaction_delta_observation.dec046.json`
  - passed.
- `git diff --check` - passed.

Full DEC-025 evidence sweep:

- `python3 tools/release/run_evidence_sweep.py --execute` - passed all five
  surfaces:
  - cargo crate sweep;
  - repository pytest, 363 tests;
  - desktop Vitest, 407 tests;
  - Playwright dev/dist desktop lanes, 18 + 1 tests;
  - desktop production build.
- Passing sweep summary:
  `validation/evidence/sweeps/SWEEP_20260622T131130Z_d4165f4fa5fe-dirty.json`

## Boundaries

- This tranche refreshes metadata parity for the observation ledger only.
- It does not change the accepted threshold values, promote a new threshold
  policy, move the observation-only depth fixture into acceptance coverage, or
  alter solver behavior.
- It does not add protected standards content, private project data, hidden
  support defaults, lifecycle transition, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim.

## Residuals

- Remaining D6/D9 work includes default sparse promotion, large-model sparse
  suitability thresholds, non-seed force/displacement threshold promotion
  beyond the accepted ten-fixture set, general energy threshold promotion,
  broader displacement/reaction-delta thresholds beyond the current seed,
  ten-fixture, and product-preview surfaces, deeper spring-hanger behavior,
  external validation threshold evidence, broader R4 validation packaging, and
  final R4 exit-chain evidence.
