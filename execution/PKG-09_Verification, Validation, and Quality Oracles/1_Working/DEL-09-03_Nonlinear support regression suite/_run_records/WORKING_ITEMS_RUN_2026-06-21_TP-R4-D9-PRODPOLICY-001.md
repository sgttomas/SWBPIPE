# WORKING ITEMS RUN - TP-R4-D9-PRODPOLICY-001

Date: 2026-06-21
Agent: WORKING_ITEMS
Package: PKG-09 - Verification, Validation, and Quality Oracles
Deliverable: DEL-09-03 - Nonlinear support regression suite
Target stage: R4 / Phase D

## Objective

Promote the product-preview nonlinear dense-loop active-set-count convergence
surface to a named accepted DEC-046 policy where current invented
product-preview evidence already covers the support classes, while preserving
`TBD` status for force/displacement, sparse live-path, external validation, and
release thresholds.

## Authority Basis

- `DEC-046`: convergence policy is class-tiered by nonlinear support class;
  unmeasured entries remain `TBD`.
- `TP-R4-D9-CONVPOLICY-001`: the current assembled validation seed accepted
  active-set changed-support-count residual, tolerance `0.0`, absolute floor
  `0.0`, and max iteration cap `4` for one-way, gap, lift-off, and friction.
- Current product-preview regressions exercise one-way/friction in the
  checked-in fixture plus separate gap/lift-off/friction/mixed product tests.
- `plans/PLAN_2026-06-17_prd_completion.md` D6/D9 residual: product-preview
  convergence thresholds remained outside the accepted active-set-count policy.

## Changes

- Added `DEC-046-CV-B-product-preview-active-set-count-v1` in
  `core/product_physics` for product-preview active-set-count convergence.
- Changed product-preview nonlinear loop construction from
  `ConvergencePolicyStatus::Tbd` to `ConvergencePolicyStatus::Accepted`, with
  residual tolerance `0.0`, absolute floor `0.0`, and max iteration cap `4`.
- Added deterministic support-class metadata to nonlinear iteration-count
  result rows.
- Updated product diagnostics to state that the active-set-count policy is
  accepted while force/displacement and release thresholds remain `TBD`.
- Updated the checked-in product-preview mechanics fixture and the Python
  service regression so the fixture no longer expects
  `TOLERANCE_POLICY_TBD` for this active-set-count surface.
- Updated desktop preview service/App assertions for the promoted policy and
  resulting two-warning diagnostic-count reduction.
- Updated completion-plan, roadmap, R4 gap packet, completion log, and this
  deliverable memory.

## Validation

Focused validation:

- `cargo test --manifest-path core/product_physics/Cargo.toml` - passed, 41
  tests.
- `python3 -m pytest -q tests/product_preview/test_product_preview_service.py tests/test_nonlinear_support_regression.py tests/test_results_schema.py tests/test_analysis_run_records.py`
  - passed, 28 tests.
- `python3 -m json.tool fixtures/product_preview/invented_mechanics_result.json`
  - passed.
- `npm run test:desktop` - passed, 407 tests.

Closeout validation:

- Full DEC-025 evidence sweep:
  `validation/evidence/sweeps/SWEEP_20260621T113918Z_756f319251c0-dirty.json`
  - passed.
- `git diff --check` - passed.

## Boundaries

- This tranche promotes only the product-preview active-set changed-support
  count policy for current invented product-preview nonlinear support classes.
- It does not promote force/displacement residual thresholds, sparse live-path
  behavior, external validation thresholds, release thresholds, or R4 exit
  readiness.
- It adds no protected standards content, private project data, hidden support
  defaults, lifecycle transition, release-readiness claim, professional
  approval, certification, sealing, authentication, or code-compliance claim.

## Residuals

- `D-15` spring-hanger scope remains awaiting ruling unless separately ruled.
- `D-17` sparse live-path adoption remains awaiting ruling unless separately
  ruled.
- Remaining D6/D9 work is force/displacement threshold promotion, external
  validation threshold evidence, sparse live-path adoption after `D-17`, and
  final R4 exit-chain evidence after the D5/D7 blockers are resolved.
