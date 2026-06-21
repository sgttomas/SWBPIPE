---
run-id: WORKING_ITEMS_RUN_2026-06-21_TP-R4-D6-FRICTIONNORMAL-001
timestamp: 2026-06-21T03:22:54-06:00
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
  friction sticking/sliding evidence.
- Add a derived friction normal-force validation path using an explicit named
  support-normal DOF, not a catalog/default/protected normal-force value.
- Surface product-preview/result-envelope derived normal evidence without
  claiming measured release convergence values or R4 exit closure.

## Outputs Produced

- Added `NL-ASSEMBLED-FRICTION-DERIVED-NORMAL-ORIGINAL`, an assembled dense
  active-set fixture that derives friction normal evidence from a restrained
  normal-source DOF.
- Added
  `validation/hand_calcs/nonlinear/assembled_friction_derived_normal.md` and
  indexed it in the nonlinear hand-calculation README and focused pytest
  requirements.
- Updated the invented product-preview fixture so `support:NL-130-FRIC` derives
  normal-reaction evidence from `support:S-130` / `UY`.
- Regenerated `fixtures/product_preview/invented_mechanics_result.json`; it
  remains at 783 result rows and now emits
  `nonlinear_support_friction_normal_reaction_derived` rows for the friction
  support.
- Updated Python and desktop fixture expectations for the derived evidence kind,
  source metadata, and derived normal-reaction values.

## Validation Evidence

- `cargo fmt --manifest-path core/solver/nonlinear_integration/Cargo.toml -- --check`: passed.
- `cargo fmt --manifest-path core/product_physics/Cargo.toml -- --check`: passed.
- `cargo fmt --manifest-path validation/benchmarks/nonlinear/Cargo.toml -- --check`: passed.
- `cargo test --manifest-path core/solver/nonlinear_integration/Cargo.toml`: passed; 10 tests.
- `cargo test --manifest-path core/product_physics/Cargo.toml`: passed; 40 tests.
- `cargo test --manifest-path validation/benchmarks/nonlinear/Cargo.toml`: passed; 7 tests.
- `python3 -m pytest tests/product_preview/test_product_preview_service.py tests/test_nonlinear_support_regression.py tests/test_results_schema.py tests/test_analysis_run_records.py`: passed; 28 tests.
- `npm test --workspace apps/desktop -- previewService App.test.tsx`: passed; 67 tests.
- `npm run build:desktop`: passed.
- `python3 tools/release/run_evidence_sweep.py --execute`: passed all five surfaces; summary `validation/evidence/sweeps/SWEEP_20260621T092312Z_53b592aee006-dirty.json`.
- `git diff --check`: passed.

## Dependency Notes

- `D-15` remains `AWAITING_RULING` and continues to gate D5 spring-hanger
  scope.
- `D-17` remains `NOT_PREPARED` and continues to gate D7 sparse live-path
  adoption timing.
- `D-16`, `D-18`, `D-19`, and `D-23` are ruled by `DEC-044`, `DEC-045`,
  `DEC-046`, and `DEC-048`.

## Residual Scope

- Measured class-tiered convergence values remain `TBD` under `DEC-046`.
- Sparse live-path adoption remains gated by `D-17`.
- Broader live-solver coverage, the PRD section 16.2 branch-assembly benchmark,
  and the R4 exit evidence package remain open.
- This run does not make a release, professional approval, certification,
  sealing, authentication, code-compliance, lifecycle, or R4 exit claim.
