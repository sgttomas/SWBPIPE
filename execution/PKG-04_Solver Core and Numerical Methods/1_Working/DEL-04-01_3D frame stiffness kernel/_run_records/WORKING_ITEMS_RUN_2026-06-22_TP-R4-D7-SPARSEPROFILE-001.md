# WORKING_ITEMS Run Record - TP-R4-D7-SPARSEPROFILE-001

**Date:** 2026-06-22
**Agent:** WORKING_ITEMS
**Persona:** deliverable-scoped content production for OpenPipeStress
**Tranche:** `TP-R4-D7-SPARSEPROFILE-001`
**Scope:** product-preview direct reduced profile-entry assembly for the `DEC-050`
sparse evidence lane
**Lifecycle posture:** implementation/evidence update only; no deliverable
issuance, release readiness, professional approval, certification, sealing,
authentication, or code-compliance claim.

## Objective

Narrow the D7 sparse residual by moving product-preview sparse evidence off the
dense-derived profile constructor. Dense remains the default product solve path
and parity oracle; this tranche does not promote sparse to the default solver.

## Authority And Inputs

- Human ruling: `DEC-050`, approving the R4 sparse evidence lane while dense
  remains default.
- Sparse strategy basis: `DEC-023`, accepting `core/solver/sparse_direct` as the
  in-repo skyline/profile direct sparse solver.
- Current target stage: R4 / Phase D under `DEC-048`.
- Active handoff: `execution/_Coordination/NEXT_INSTANCE_PROMPT.md` and
  `execution/_Coordination/_COORDINATION.md`.

## Work Performed

- Added explicit symmetric-entry profile storage and solve APIs to
  `core/solver/sparse_direct`, including duplicate-entry summation and parity
  tests against the dense-derived sparse path.
- Added deterministic product reduced-entry assembly from frame elements,
  expansion-joint user-stiffness elements, and spring support entries.
- Updated product sparse evidence metadata to record
  `assembly=direct_reduced_profile_entries`,
  `profile_direct_assembly=observed`, and
  `default_sparse_promotion=follow_on`.
- Regenerated `fixtures/product_preview/invented_mechanics_result.json`.
- Updated sparse diagnostics for the new explicit-entry error and status
  wording.
- Updated coordination, roadmap, and completion-plan residual wording.

## Validation

- `cargo test --manifest-path core/solver/sparse_direct/Cargo.toml` passed 20/20
  tests.
- `cargo test --manifest-path core/solver/diagnostics/Cargo.toml` passed 24/24
  tests.
- `cargo test --manifest-path core/product_physics/Cargo.toml` passed 44/44
  tests after fixture regeneration.
- Full DEC-025 evidence sweep passed:
  `validation/evidence/sweeps/SWEEP_20260622T085210Z_799ebcc0dee5-dirty.json`
  (`overall_status=pass`, 5/5 surfaces).

## Boundaries Preserved

- Dense remains the default product solve path and parity oracle.
- Sparse evidence rows remain solver evidence, not load effects.
- No release-readiness, lifecycle transition, professional approval,
  certification, sealing, authentication, or code-compliance claim was made.
- No protected standards content, proprietary benchmark output, private project
  data, network path, or telemetry feature was introduced.

## Residuals And Next Step

- Default sparse promotion remains follow-on work after measured suitability.
- Nonlinear/core profile-direct sparse promotion remains follow-on work.
- Non-seed convergence thresholds, deeper spring-hanger behavior, external
  validation thresholds, and final R4 exit evidence remain open Phase D work.
