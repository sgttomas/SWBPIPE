# WORKING ITEMS RUN - TP-R4-D9-CONVPOLICY-001

Date: 2026-06-21
Agent: WORKING_ITEMS
Package: PKG-09 - Verification, Validation, and Quality Oracles
Deliverable: DEL-09-03 - Nonlinear support regression suite
Target stage: R4 / Phase D

## Objective

Promote the `DEC-046` class-tier convergence policy only where the current
assembled validation evidence supports it: the active-set
changed-support-count residual used by the public-original assembled nonlinear
validation seed.

## Authority Basis

- `DEC-046` / D-19: class-tiered convergence policy by nonlinear support class;
  unmeasured entries remain `TBD`.
- PRD §22.5: R4 exit requires nonlinear support validation cases converge.
- `plans/PLAN_2026-06-17_prd_completion.md` Phase D D6/D9 residual: governed
  class-tier threshold promotion where evidence supports it.
- `validation/hand_calcs/nonlinear/convergence_observations.md`: observed
  current assembled fixture behavior before this tranche.

## Changes

- Added machine-readable governed policy record:
  `validation/benchmarks/nonlinear/convergence_policy.dec046.json`.
- Added `DEC_046_ACTIVE_SET_COUNT_POLICY_REF`,
  `ConvergencePolicyEntry`, and `governed_convergence_policy_entries()` in
  `validation/benchmarks/nonlinear/src/lib.rs`.
- Bound the six assembled validation fixtures to
  `DEC-046-CV-B-active-set-count-validation-v1` with:
  - residual basis: active-set changed-support count;
  - relative residual tolerance: `0.0`;
  - absolute residual floor: `0.0`;
  - max iteration cap: `4`;
  - classes: one-way, gap, lift-off, friction.
- Updated assembled fixture notes and the convergence observation note to cite
  the accepted policy instead of `TBD`.
- Updated focused Python guardrails to validate the policy JSON, the accepted
  policy ref, per-class entries, evidence fixtures, and boundary wording.
- Updated the active completion plan, completion log, and DEL-09-03 memory.

## Validation

Focused validation:

- `cargo fmt --manifest-path validation/benchmarks/nonlinear/Cargo.toml -- --check` - passed
- `cargo test --manifest-path validation/benchmarks/nonlinear/Cargo.toml` - passed, 9 tests
- `python3 -m pytest tests/test_nonlinear_support_regression.py -q` - passed, 8 tests

Closeout validation:

- `git diff --check` - passed
- `npm run test:desktop` - passed, 407 tests, after one transient Vitest
  timing failure inside the first full sweep attempt on the pre-existing
  endpoint-pick UI test
- `python3 tools/release/run_evidence_sweep.py --execute` - passed all five
  surfaces on rerun:
  - cargo crate sweep: 33 manifests passed;
  - repository pytest: 362 passed;
  - desktop Vitest: 407 passed;
  - Playwright dev/dist: 18 + 1 passed;
  - desktop production build: passed.
- Passing sweep summary:
  `validation/evidence/sweeps/SWEEP_20260621T103336Z_a02ab8f77612-dirty.json`

## Boundaries

- Product-preview convergence policy remains `TBD` and still emits
  `TOLERANCE_POLICY_TBD` where applicable.
- Force/displacement residuals, energy residuals, sparse live-path behavior,
  external validation thresholds, and any loosening of the accepted active-set
  count threshold remain outside this tranche.
- No protected standards content, private project data, hidden support defaults,
  lifecycle transition, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim was added.

## Residuals

- D5 spring-hanger scope remains gated by `D-15`.
- D7 sparse live-path adoption remains gated by `D-17`.
- R4 exit evidence package remains open under D9.
