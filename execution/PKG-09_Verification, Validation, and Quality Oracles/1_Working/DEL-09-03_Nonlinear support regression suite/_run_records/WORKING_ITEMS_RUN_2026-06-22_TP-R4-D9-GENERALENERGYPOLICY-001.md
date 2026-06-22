# WORKING ITEMS RUN - TP-R4-D9-GENERALENERGYPOLICY-001

Date: 2026-06-22
Agent: WORKING_ITEMS
Package: PKG-09 - Verification, Validation, and Quality Oracles
Deliverable: DEL-09-03 - Nonlinear support regression suite
Target stage: R4 / Phase D

## Objective

Promote a bounded `DEC-046` general-energy residual policy for already
evidenced nonlinear residual surfaces: the current public-original assembled
validation seed, the accepted ten-fixture multi-support companion set, and the
current invented product-preview nonlinear surface.

## Authority Basis

- PRD section 22.5: R4 requires nonlinear support validation cases to converge.
- `DEC-044`: the assembled nonlinear solve is owned by the PKG-04 integration
  tranche bridging DEL-04-04 and DEL-04-01; the classifier remains the
  per-iteration state oracle.
- `DEC-046`: class-tiered convergence policy can promote measured entries, but
  unmeasured and broader entries remain `TBD`.
- `TP-R4-D9-ENERGYOBS-001` and `TP-R4-D9-WORKPOLICY-001`: added and promoted
  final-iteration free-DOF work residual observations for the current seed,
  accepted fixture set, and product-preview surface.
- `TP-R4-D9-MULTISUPPORT4CLASS-001`: completed the current accepted
  ten-fixture multi-support companion set used by this policy.

## Changes

- Added current-seed and multi-support general-energy residual policy records:
  `DEC-046-CV-B-general-energy-residual-validation-v1` and
  `DEC-046-CV-B-multisupport-general-energy-residual-validation-v1`, each with
  a `0.0 N-m` final-iteration residual energy limit.
- Promoted the invented product-preview general-energy residual metadata to
  `DEC-046-CV-B-product-preview-general-energy-residual-v1` while preserving
  sparse-default, release, external, total strain-energy, modal-energy, and CI
  threshold residuals as open.
- Updated nonlinear benchmark policy helpers/tests, product-preview tests,
  generated product-preview fixture evidence, hand-calculation notes, benchmark
  README text, coordination surfaces, active R4 planning, the R4 gap packet,
  and DEL-09-03 memory.

## Validation

Focused validation:

- `cargo fmt --manifest-path validation/benchmarks/nonlinear/Cargo.toml --check`
  - passed.
- `cargo fmt --manifest-path core/product_physics/Cargo.toml --check` -
  passed.
- `cargo fmt --manifest-path core/solver/nonlinear_integration/Cargo.toml --check`
  - passed.
- `cargo test --quiet --manifest-path validation/benchmarks/nonlinear/Cargo.toml`
  - passed, 18 tests.
- `cargo test --quiet --manifest-path core/product_physics/Cargo.toml` -
  passed, 44 tests.
- `cargo test --quiet --manifest-path core/solver/nonlinear_integration/Cargo.toml`
  - passed, 11 tests.
- `python3 -m pytest -q tests/test_nonlinear_support_regression.py tests/product_preview/test_product_preview_service.py tests/test_results_schema.py tests/test_analysis_run_records.py`
  - passed, 28 tests.
- JSON syntax validation passed for
  `validation/benchmarks/nonlinear/general_energy_policy.dec046.json`,
  `validation/benchmarks/nonlinear/multisupport_general_energy_policy.dec046.json`,
  and `fixtures/product_preview/invented_mechanics_result.json`.
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
  `validation/evidence/sweeps/SWEEP_20260622T142119Z_1b32c80965d5-dirty.json`

## Boundaries

- This tranche promotes only measured final-iteration general-energy residuals
  for the current assembled validation seed, the accepted ten-fixture
  multi-support companion set, and the current invented product-preview
  surface.
- It does not promote broader non-seed force/displacement thresholds, broader
  displacement/reaction-delta thresholds, sparse default behavior, release
  thresholds, external validation thresholds, total strain-energy thresholds,
  modal-energy thresholds, CI thresholds, or any lifecycle exit decision.
- It does not add protected standards content, private project data, hidden
  support defaults, lifecycle transition, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim.

## Residuals

- Remaining D6/D9 work includes non-seed force/displacement threshold promotion
  beyond the accepted ten-fixture set, broader displacement/reaction-delta
  thresholds beyond accepted current/product/fixture surfaces, broader
  non-seed/release/external/sparse-default/total/modal/CI energy threshold
  axes, deeper spring-hanger behavior, default sparse promotion plus sparse
  timing/memory/conditioning/CI/hardware-normalized threshold evidence, broader
  R4 validation packaging, and final R4 exit-chain evidence.
