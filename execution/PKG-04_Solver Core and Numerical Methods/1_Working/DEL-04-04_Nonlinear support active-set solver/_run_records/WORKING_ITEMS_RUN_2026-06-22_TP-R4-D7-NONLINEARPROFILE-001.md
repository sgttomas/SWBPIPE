# WORKING_ITEMS Run Record - TP-R4-D7-NONLINEARPROFILE-001

**Date:** 2026-06-22
**Agent:** WORKING_ITEMS
**Persona:** deliverable-scoped content production for OpenPipeStress
**Tranche:** `TP-R4-D7-NONLINEARPROFILE-001`
**Scope:** nonlinear integration direct reduced profile-entry sparse evidence
under `DEC-050`
**Lifecycle posture:** implementation/evidence update only; no deliverable
issuance, release readiness, professional approval, certification, sealing,
authentication, or code-compliance claim.

## Objective

Narrow the D7 sparse residual by moving `core/solver/nonlinear_integration`
sparse evidence off dense-derived reduced profile construction. Dense remains
the active nonlinear solve path and parity oracle; this tranche does not
promote sparse to the default solver.

## Authority And Inputs

- Human ruling: `DEC-050`, approving an R4 sparse evidence lane while dense
  remains default.
- Sparse strategy basis: `DEC-023`, accepting the in-repo
  `core/solver/sparse_direct` skyline/profile direct sparse solver.
- Assembled nonlinear integration owner/method: `DEC-044`.
- Current target stage: R4 / Phase D under `DEC-048`.

## Work Performed

- Added `assembly_basis` to `SparseLinearSolveEvidence` and set observed and
  unavailable records to `direct_reduced_profile_entries`.
- Changed nonlinear sparse observation to assemble lower-triangle
  `SymmetricMatrixEntry` values from the assembled global stiffness and active
  `free_dofs` map, then call `solve_symmetric_system_from_entries`.
- Computed sparse residuals against the direct entry system and preserved the
  non-blocking unavailable sparse-evidence posture.
- Updated assembled-loop limitation text to distinguish observed direct
  reduced profile-entry evidence from remaining default sparse promotion and
  large-model suitability thresholds.
- Added focused regression coverage for non-contiguous free-DOF mapping and
  asserted the observed sparse evidence basis in the active-set loop test.

## Validation

- `cargo fmt --manifest-path core/solver/nonlinear_integration/Cargo.toml --check`
  passed.
- `cargo test --manifest-path core/solver/nonlinear_integration/Cargo.toml`
  passed 11/11 tests.
- `cargo test --manifest-path core/product_physics/Cargo.toml` passed 44/44
  tests.
- `cargo test --manifest-path validation/benchmarks/nonlinear/Cargo.toml`
  passed 16/16 tests.
- Full DEC-025 evidence sweep passed 5/5 surfaces:
  `validation/evidence/sweeps/SWEEP_20260622T102208Z_479c6fd7d97a-dirty.json`.

## Boundaries Preserved

- Dense remains the default nonlinear solve path and parity oracle.
- Sparse evidence remains solver observation, not a load effect or acceptance
  threshold.
- This does not implement default sparse promotion, timing/memory thresholds,
  practical-size bands, conditioning/CI thresholds, or release-readiness.
- No protected standards content, proprietary benchmark output, private
  project data, network path, telemetry feature, professional approval,
  certification, sealing, authentication, or code-compliance claim was
  introduced.

## Residuals And Next Step

- Default sparse promotion and large-model sparse suitability thresholds remain
  follow-on work under D7/D9.
- Non-seed convergence thresholds, accepted displacement/reaction-delta
  thresholds, deeper spring-hanger behavior, external validation thresholds,
  and final R4 exit evidence remain open Phase D work.
