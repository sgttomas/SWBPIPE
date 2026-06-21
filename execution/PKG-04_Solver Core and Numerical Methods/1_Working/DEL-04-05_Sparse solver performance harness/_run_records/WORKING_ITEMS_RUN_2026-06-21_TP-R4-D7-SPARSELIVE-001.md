# WORKING_ITEMS Run Record - TP-R4-D7-SPARSELIVE-001

**Date:** 2026-06-21
**Agent:** WORKING_ITEMS
**Persona:** deliverable-scoped content production for OpenPipeStress
**Tranche:** `TP-R4-D7-SPARSELIVE-001`
**Scope:** `DEC-050` sparse live evidence lane and sparse-harness posture update
**Lifecycle posture:** implementation/evidence update only; no deliverable issuance,
release readiness, professional approval, certification, sealing,
authentication, or code-compliance claim.

## Objective

Implement the approved `D-17` Option B ruling (`DEC-050`) by exposing live sparse
evidence beside the dense R4 product solve path. Dense remains the default path
and parity oracle; sparse-default promotion and profile-direct assembly remain
follow-on work.

## Authority And Inputs

- Human ruling: `DEC-050`, approving `D-17` Option B.
- Prior sparse strategy basis: `DEC-023` accepts `core/solver/sparse_direct` as
  the in-repo skyline/profile direct sparse solver.
- Current target stage: R4 / Phase D under `DEC-048`.
- DEL-04-05 prior state: the performance harness already measured sparse
  observations alongside dense; its open item was live solve-path adoption.

## Work Performed

- Bound `core/solver/sparse_direct` as a non-blocking evidence observer in
  `core/solver/nonlinear_integration` and `core/product_physics`.
- Preserved dense as the default solver while recording live sparse parity,
  residual, pivot, bandwidth, and profile metadata.
- Updated diagnostics and performance-harness limitation wording so they no
  longer say sparse strategy is unresolved or live evidence is absent. The
  remaining TBD is narrower: profile-direct assembly and default sparse
  promotion.
- Added product result-envelope evidence rows:
  `result:sparse-live:dense-parity-relative-delta` and
  `result:loadcase:load-L-200:sparse-live:dense-parity-relative-delta`.
- Updated desktop unit and Playwright smoke expectations for the 802-row result
  surface and the native-package result-quantity witness.

## Validation

- `cargo fmt --manifest-path core/solver/performance_harness/Cargo.toml --check`
  passed.
- `cargo test --manifest-path core/solver/performance_harness/Cargo.toml`
  passed 18/18 tests.
- `cargo fmt --manifest-path core/solver/diagnostics/Cargo.toml --check`
  passed.
- `cargo test --manifest-path core/solver/diagnostics/Cargo.toml` passed 24/24
  tests.
- `cargo fmt --manifest-path core/solver/nonlinear_integration/Cargo.toml --check`
  passed.
- `cargo test --manifest-path core/solver/nonlinear_integration/Cargo.toml`
  passed 10/10 tests.
- `cargo fmt --manifest-path core/product_physics/Cargo.toml --check` passed.
- `cargo test --manifest-path core/product_physics/Cargo.toml` passed 43/43
  tests.
- `npm test --workspace apps/desktop` passed 19/19 test files and 407/407
  tests.
- From `apps/desktop`,
  `npm exec -- playwright test e2e/r2-smoke.spec.ts --workers=1` passed 18/18
  Playwright checks after updating stale fixture-count expectations.
- Full DEC-025 evidence sweep passed:
  `validation/evidence/sweeps/SWEEP_20260621T205711Z_c771567ed6a8-dirty.json`
  (`overall_status=pass`, 5/5 surfaces).
- `git diff --check` passed after evidence records were written.

## Boundaries Preserved

- Dense remains the default product solve path and parity oracle.
- Sparse observations are evidence rows, not load effects, and are excluded
  from combination algebra.
- No release timing, memory, practical-size band, conditioning, CI threshold,
  default sparse promotion, professional approval, certification, sealing,
  authentication, or code-compliance claim was introduced.
- No protected standards content, proprietary benchmark model, private project
  data, network path, or telemetry feature was introduced.

## Residuals And Next Step

- Profile-direct sparse assembly remains follow-on work.
- Sparse-default promotion remains follow-on work after measured suitability.
- Non-seed convergence thresholds, deeper spring-hanger behavior, and the
  remaining D9 validation package remain open Phase D work.
