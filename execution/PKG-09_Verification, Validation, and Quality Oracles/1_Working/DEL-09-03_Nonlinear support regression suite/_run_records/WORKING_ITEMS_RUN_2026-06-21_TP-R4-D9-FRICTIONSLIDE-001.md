---
run-id: WORKING_ITEMS_RUN_2026-06-21_TP-R4-D9-FRICTIONSLIDE-001
timestamp: 2026-06-21T02:58:38-06:00
run-status: SUCCESS
control-surface: INLINE
scope-path: execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-03_Nonlinear support regression suite
task-profile: validation-qa
task-skill: NONE
resolved-skill-path: NONE
resolved-skill-version: UNKNOWN
---

## Requested Tasks

- Continue Phase D/R4 D6/D9 nonlinear residual work after explicit-normal
  friction sticking, gap, and lift-off evidence.
- Add assembled sliding-friction validation coverage using explicit invented
  normal-reaction input only.
- Surface product-preview/result-envelope sliding evidence without claiming a
  governed derived normal-force model or R4 exit closure.

## Outputs Produced

- Added `NL-ASSEMBLED-FRICTION-SLIDE-ORIGINAL`, an assembled dense active-set
  friction fixture that ends `Sliding` with explicit normal reaction evidence.
- Added `validation/hand_calcs/nonlinear/assembled_friction_sliding.md` and
  indexed it in the nonlinear hand-calculation README and focused pytest
  requirements.
- Updated solver/product tests so the invented preview path includes
  `support:NL-130-FRIC`, final state code `3`, nonzero released displacement,
  zero released reaction, and explicit normal-reaction evidence rows.
- Updated desktop/e2e expectations for the expanded result envelope and model
  export sidecars.

## Validation Evidence

- `cargo test --manifest-path validation/benchmarks/nonlinear/Cargo.toml`: passed; 7 tests.
- `python3 -m pytest tests/product_preview/test_product_preview_service.py tests/test_nonlinear_support_regression.py tests/test_results_schema.py tests/test_analysis_run_records.py`: passed; 28 tests.
- `cargo test --manifest-path core/solver/nonlinear_supports/Cargo.toml`: passed; 17 tests.
- `cargo test --manifest-path core/solver/nonlinear_integration/Cargo.toml`: passed; 8 tests.
- `cargo test --manifest-path core/product_physics/Cargo.toml`: passed; 39 tests.
- `npm test --workspace apps/desktop -- previewService App.test.tsx`: passed; 67 tests.
- `python3 tools/release/run_evidence_sweep.py --execute`: passed all five surfaces; summary `validation/evidence/sweeps/SWEEP_20260621T085427Z_93a25e03201f-dirty.json`.
- `git diff --check`: passed.

## Dependency Notes

- `D-15` remains `AWAITING_RULING` and continues to gate D5 spring-hanger
  scope.
- `D-17` remains `NOT_PREPARED` and continues to gate D7 sparse live-path
  adoption timing.
- `D-16`, `D-18`, `D-19`, and `D-23` are ruled by `DEC-044`, `DEC-045`,
  `DEC-046`, and `DEC-048`.

## Residual Scope

- Derived friction normal-force modeling remains open; this run uses explicit
  invented normal-reaction evidence only.
- Measured class-tiered convergence values remain `TBD` under `DEC-046`.
- Sparse live-path adoption remains gated by `D-17`.
- Broader live-solver coverage, the PRD section 16.2 branch-assembly benchmark,
  and the R4 exit evidence package remain open.
- This run does not make a release, professional approval, certification,
  sealing, authentication, code-compliance, lifecycle, or R4 exit claim.
