# WORKING_ITEMS Run Record - TP-R4-D7-NONLINEARPROFILE-001

**Date:** 2026-06-22
**Agent:** WORKING_ITEMS
**Persona:** deliverable-scoped content production for OpenPipeStress
**Tranche:** `TP-R4-D7-NONLINEARPROFILE-001`
**Scope:** nonlinear/core direct reduced profile-entry sparse evidence posture
**Lifecycle posture:** implementation/evidence update only; no deliverable
issuance, release readiness, professional approval, certification, sealing,
authentication, or code-compliance claim.

## Objective

Complete the nonlinear/core side of the `DEC-050` direct-profile sparse
evidence follow-on. Product-preview sparse evidence already used direct reduced
profile entries; this tranche gives the assembled nonlinear integration loop
the same explicit evidence basis while retaining dense as default.

## Work Performed

- `core/solver/nonlinear_integration` now records
  `assembly_basis=direct_reduced_profile_entries` for sparse evidence.
- The sparse observer builds entry-based reduced systems from the active
  free-DOF map and calls `core/solver/sparse_direct` through
  `solve_symmetric_system_from_entries`.
- Sparse residual observations are computed from the same direct entries.
- The evidence limitation text now names the remaining sparse residual as
  default sparse promotion / large-model suitability rather than
  a direct-profile evidence gap.
- Coordination and roadmap surfaces were updated so future instances do not
  reselect the direct-profile evidence gap as open work.

## Validation

- Nonlinear integration Rust tests passed 11/11.
- Product physics Rust tests passed 44/44.
- Nonlinear benchmark Rust tests passed 16/16.
- `cargo fmt --manifest-path core/solver/nonlinear_integration/Cargo.toml --check`
  passed.
- Full DEC-025 evidence sweep passed:
  `validation/evidence/sweeps/SWEEP_20260622T102208Z_479c6fd7d97a-dirty.json`
  (`overall_status=pass`, 5/5 surfaces).

## Boundaries Preserved

- Dense remains the default solve path and parity oracle.
- No default sparse promotion, timing/memory threshold, practical-size band,
  conditioning threshold, release-readiness, professional approval,
  certification, sealing, authentication, or code-compliance claim was made.
- No protected standards content, proprietary benchmark model, private project
  data, network path, or telemetry feature was introduced.

## Residuals And Next Step

- Default sparse promotion and large-model sparse suitability evidence remain
  follow-on work.
- Non-seed convergence thresholds, deeper spring-hanger behavior, external
  validation thresholds, and final R4 exit evidence remain open Phase D work.
