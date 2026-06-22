# WORKING ITEMS RUN - TP-R4-D9-WORKPOLICY-001

Date: 2026-06-22
Agent: WORKING_ITEMS
Package: PKG-09 - Verification, Validation, and Quality Oracles
Deliverable: DEL-09-03 - Nonlinear support regression suite
Target stage: R4 / Phase D

## Objective

Promote a bounded `DEC-046` final-iteration free-DOF work residual policy for
the measured zero-work residual products already present in the current
assembled validation seed, the accepted multi-support companion fixture, and
the invented product-preview nonlinear surface.

## Authority Basis

- PRD §22.5: R4 requires nonlinear support validation cases to converge.
- `DEC-044`: the assembled nonlinear solve is owned by the PKG-04 integration
  tranche bridging DEL-04-04 and DEL-04-01; the classifier remains the
  per-iteration state oracle.
- `DEC-046`: class-tiered convergence policy can promote measured entries, but
  unmeasured/non-seed entries remain `TBD`.
- `TP-R4-D9-ENERGYOBS-001`: added final-iteration free-DOF work residual
  observations without promoting a threshold.
- `TP-R4-D9-MULTISUPPORTPOLICY-001`: added one accepted Ux/Uy multi-support
  companion under narrow active-set and free-DOF force/moment policies while
  leaving work/energy threshold promotion open.

## Changes

- Added current-seed and multi-support free-DOF work residual policy records:
  `DEC-046-CV-B-free-dof-work-residual-validation-v1` and
  `DEC-046-CV-B-multisupport-free-dof-work-residual-validation-v1`, each with
  a `0.0 N-m` final-iteration work residual limit.
- Promoted the invented product-preview free-DOF work residual metadata to
  `DEC-046-CV-B-product-preview-free-dof-work-residual-v1` while preserving
  `general_energy_threshold=TBD`.
- Updated nonlinear benchmark policy helpers/tests, product-preview regression
  tests, desktop unit/e2e row-count expectations, hand-calculation notes,
  benchmark README text, coordination surfaces, R4 gap planning, and
  deliverable memory.
- Regenerated `fixtures/product_preview/invented_mechanics_result.json` so the
  result envelope carries accepted free-DOF work residual rows.

## Validation

Focused validation:

- `cargo fmt --manifest-path validation/benchmarks/nonlinear/Cargo.toml --check` -
  passed.
- `cargo fmt --manifest-path core/product_physics/Cargo.toml --check` -
  passed.
- `cargo test --manifest-path validation/benchmarks/nonlinear/Cargo.toml` -
  passed, 15 tests.
- `cargo test --manifest-path core/product_physics/Cargo.toml` - passed, 44
  tests.
- `python3 -m pytest -q tests/test_nonlinear_support_regression.py tests/product_preview/test_product_preview_service.py tests/test_results_schema.py tests/test_analysis_run_records.py` -
  passed, 28 tests.
- JSON validation passed for
  `validation/benchmarks/nonlinear/free_dof_work_policy.dec046.json`,
  `validation/benchmarks/nonlinear/multisupport_free_dof_work_policy.dec046.json`,
  and `fixtures/product_preview/invented_mechanics_result.json`.
- `npm run test:desktop` - passed, 407 tests.
- `npm run test:e2e:desktop` - passed, 18 tests.
- `git diff --check` - passed.

Closeout validation:

- `python3 tools/release/run_evidence_sweep.py --execute` - passed all five
  DEC-025 surfaces:
  - cargo crate sweep;
  - repository pytest;
  - desktop Vitest after wasm build;
  - Playwright dev/dist desktop lanes;
  - desktop production build.
- Passing sweep summary:
  `validation/evidence/sweeps/SWEEP_20260622T081626Z_719254ef37f8-dirty.json`

## Boundaries

- This tranche promotes only measured final-iteration free-DOF work residual
  products for the current assembled validation seed, the accepted Ux/Uy
  multi-support companion, and the current invented product-preview surface.
- It does not promote general energy, displacement-delta, reaction-delta,
  sparse default, product-preview release, external validation,
  spring-hanger, or general multi-support threshold policies.
- It does not add protected standards content, private project data, hidden
  support defaults, lifecycle transition, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim.

## Residuals

- Remaining D6/D9 work includes profile-direct sparse assembly/default sparse
  promotion, non-seed force/displacement threshold promotion beyond the
  accepted companion, general energy threshold promotion,
  displacement/reaction-delta threshold axes, deeper spring-hanger behavior,
  external validation threshold evidence, broader multi-support acceptance
  coverage, and final R4 exit-chain evidence.
