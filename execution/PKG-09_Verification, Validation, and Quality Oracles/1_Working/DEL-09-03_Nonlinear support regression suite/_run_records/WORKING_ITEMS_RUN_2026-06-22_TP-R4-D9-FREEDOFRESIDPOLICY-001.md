# WORKING ITEMS RUN - TP-R4-D9-FREEDOFRESIDPOLICY-001

Date: 2026-06-22
Agent: WORKING_ITEMS
Package: PKG-09 - Verification, Validation, and Quality Oracles
Deliverable: DEL-09-03 - Nonlinear support regression suite
Target stage: R4 / Phase D

## Objective

Promote the current invented nonlinear validation/product-preview free-DOF
force/moment residual surface to explicit `DEC-046` policies where measured
evidence is already zero, while preserving `TBD` status for displacement
deltas, reaction deltas, energy, sparse-default behavior, release thresholds,
external validation thresholds, and non-seed convergence thresholds.

## Authority Basis

- `DEC-044`: the assembled nonlinear solve is owned by the PKG-04 integration
  tranche bridging DEL-04-04 and DEL-04-01; the classifier remains the
  per-iteration state oracle.
- `DEC-046`: class-tiered convergence policy may promote measured entries;
  unmeasured/non-seed entries remain `TBD`.
- `TP-R4-D9-FORCEDISPRESID-001`: the current dense nonlinear integration and
  product-preview surfaces already record final free-DOF force/moment
  residual observations.
- PRD §22.5: R4 requires nonlinear support validation cases to converge.

## Changes

- Added `DEC-046-CV-B-free-dof-force-moment-residual-validation-v1` to
  `validation/benchmarks/nonlinear`, with per-class evidence for one-way, gap,
  lift-off, and friction assembled validation seeds.
- Added machine-readable validation policy evidence:
  `validation/benchmarks/nonlinear/free_dof_force_moment_policy.dec046.json`.
- Updated the nonlinear benchmark force/displacement residual observation
  structure and tests so free-DOF force/moment residuals use the accepted
  threshold policy, while displacement and reaction deltas remain observation
  evidence only.
- Added `DEC-046-CV-B-product-preview-free-dof-force-moment-residual-v1` to
  `core/product_physics` metadata for the current invented product-preview
  free-DOF force/moment residual rows; regenerated
  `fixtures/product_preview/invented_mechanics_result.json`.
- Updated focused nonlinear and product-preview regressions, deliverable memory,
  the completion plan, completion log, strategic roadmap, and current
  coordination handoff surfaces.

## Validation

Focused validation:

- `cargo fmt --manifest-path validation/benchmarks/nonlinear/Cargo.toml` -
  passed.
- `cargo fmt --manifest-path core/product_physics/Cargo.toml` - passed.
- `cargo test --manifest-path validation/benchmarks/nonlinear/Cargo.toml` -
  passed, 11 tests.
- `cargo test --manifest-path core/product_physics/Cargo.toml` - passed, 44
  tests.
- `python3 -m pytest -q tests/test_nonlinear_support_regression.py` - passed,
  8 tests.
- `python3 -m pytest -q tests/product_preview/test_product_preview_service.py tests/test_results_schema.py tests/test_analysis_run_records.py`
  - passed, 20 tests.
- `python3 -m json.tool fixtures/product_preview/invented_mechanics_result.json`
  - passed.

Closeout validation:

- `python3 tools/release/run_evidence_sweep.py --execute` - passed all five
  DEC-025 surfaces:
  - cargo crate sweep: 33 manifests passed;
  - repository pytest: 362 passed;
  - desktop Vitest: 407 passed;
  - Playwright dev/dist: 18 + 1 passed;
  - desktop production build: passed.
- Passing sweep summary:
  `validation/evidence/sweeps/SWEEP_20260622T065649Z_150b107259dd-dirty.json`
- `git diff --check` - passed.

## Boundaries

- This tranche promotes only final-iteration free-DOF force and moment
  residual thresholds for the current public-original assembled validation seed
  and current invented product-preview dense-loop surface.
- It does not promote displacement-delta, reaction-delta, energy, sparse live
  path, sparse-default, external validation, release, or non-seed thresholds.
- It adds no protected standards content, private project data, hidden support
  defaults, lifecycle transition, release-readiness claim, professional
  approval, certification, sealing, authentication, or code-compliance claim.

## Residuals

- Remaining D6/D9 work includes non-seed force/displacement/energy thresholds,
  displacement/reaction-delta threshold axes, multi-DOF / multi-support
  nonlinear fixture depth, deeper spring-hanger behavior, profile-direct sparse
  assembly/default sparse promotion, external validation threshold evidence,
  and final R4 exit-chain evidence.
