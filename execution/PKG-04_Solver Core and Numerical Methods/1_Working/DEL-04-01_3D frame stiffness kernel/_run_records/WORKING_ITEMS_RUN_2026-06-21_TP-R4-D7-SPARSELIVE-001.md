# WORKING_ITEMS Run Record - TP-R4-D7-SPARSELIVE-001

**Date:** 2026-06-21
**Agent:** WORKING_ITEMS
**Persona:** deliverable-scoped content production for OpenPipeStress
**Tranche:** `TP-R4-D7-SPARSELIVE-001`
**Scope:** `DEC-050` sparse live evidence lane for the current R4 dense solve path
**Lifecycle posture:** implementation/evidence update only; no deliverable issuance,
release readiness, professional approval, certification, sealing,
authentication, or code-compliance claim.

## Objective

Implement the approved `D-17` Option B ruling (`DEC-050`): add an R4 live
sparse evidence lane while dense remains the default product solve path and
parity oracle. This tranche does not promote sparse to the default solver and
does not implement profile-direct sparse assembly.

## Authority And Inputs

- Human ruling: `DEC-050`, approving `D-17` Option B.
- Prior sparse strategy basis: `DEC-023` accepts `core/solver/sparse_direct` as
  the in-repo skyline/profile direct sparse solver.
- Current target stage: R4 / Phase D under `DEC-048`.
- Active coordination basis:
  `execution/_Coordination/NEXT_INSTANCE_PROMPT.md`,
  `execution/_Coordination/_COORDINATION.md`, `docs/PLAN.md`, and
  `plans/PLAN_2026-06-17_prd_completion.md`.

## Work Performed

- Added sparse sidecar observation to `core/solver/nonlinear_integration` on the
  same reduced systems solved by the dense default path. Iteration records now
  expose reduced DOF count, original/ordered profile and bandwidth, nonpositive
  pivot count, pivot proxy, dense-vs-sparse deltas, sparse residual, and
  non-blocking sparse failure status.
- Preserved the frame-kernel dense provider boundary: `core/solver/frame_kernel`
  was not changed and still supplies assembly, reduction, and dense verification
  behavior to the integration layer.
- Added product result-envelope evidence rows in `core/product_physics`:
  `sparse_live_path_dense_parity_relative_delta` for the default and explicit
  second invented load cases. These rows are excluded from load-combination
  algebra because they are solver evidence, not load effects.
- Regenerated the invented mechanics fixture. Result rows increased from 800 to
  802, adding two unitless sparse parity evidence rows with `DEC-050` metadata.
- Updated desktop unit and Playwright smoke expectations for the 802-row result
  surface and the native-package result-quantity witness.

## Validation

- `cargo fmt --manifest-path core/solver/nonlinear_integration/Cargo.toml --check`
  passed.
- `cargo test --manifest-path core/solver/nonlinear_integration/Cargo.toml`
  passed 10/10 tests.
- `cargo fmt --manifest-path core/product_physics/Cargo.toml --check` passed.
- `cargo test --manifest-path core/product_physics/Cargo.toml` passed 43/43
  tests.
- `cargo fmt --manifest-path core/solver/diagnostics/Cargo.toml --check`
  passed.
- `cargo test --manifest-path core/solver/diagnostics/Cargo.toml` passed 24/24
  tests.
- `cargo fmt --manifest-path core/solver/performance_harness/Cargo.toml --check`
  passed.
- `cargo test --manifest-path core/solver/performance_harness/Cargo.toml`
  passed 18/18 tests.
- `pytest tests/product_preview/test_product_preview_service.py tests/test_results_schema.py -q`
  passed 11/11 tests.
- `npm test --workspace apps/desktop` passed 19/19 test files and 407/407
  tests.
- From `apps/desktop`,
  `npm exec -- playwright test e2e/r2-smoke.spec.ts --workers=1` passed 18/18
  Playwright checks after updating the stale 800-row smoke expectations.
- Full DEC-025 evidence sweep passed:
  `validation/evidence/sweeps/SWEEP_20260621T205711Z_c771567ed6a8-dirty.json`
  (`overall_status=pass`, 5/5 surfaces).
- `git diff --check` passed after evidence records were written.

## Boundaries Preserved

- Dense remains the default product solve path and parity oracle.
- No `frame_kernel` API or implementation change was made.
- No profile-direct sparse assembly, sparse-default promotion, timing/memory
  threshold, release gate, lifecycle transition, professional approval,
  certification, sealing, authentication, or code-compliance claim was made.
- No protected standards content, proprietary catalog value, private project
  data, network path, or telemetry feature was introduced.

## Residuals And Next Step

- Profile-direct sparse assembly remains a follow-on item.
- Default sparse promotion remains a measured follow-on item.
- Non-seed convergence thresholds, deeper spring-hanger behavior, and the
  remaining D9 validation package remain open Phase D work.
