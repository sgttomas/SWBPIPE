# WORKING_ITEMS Run Record - TP-R4-D7-SPARSEPROFILE-001

**Date:** 2026-06-22
**Agent:** WORKING_ITEMS
**Persona:** deliverable-scoped content production for OpenPipeStress
**Tranche:** `TP-R4-D7-SPARSEPROFILE-001`
**Scope:** direct profile-entry sparse solve API and product sparse evidence
posture update
**Lifecycle posture:** implementation/evidence update only; no deliverable
issuance, release readiness, professional approval, certification, sealing,
authentication, or code-compliance claim.

## Objective

Add a direct explicit-entry sparse solve surface and use it for product-preview
sparse evidence so `DEC-050` evidence rows no longer depend on dense-derived
profile construction. Dense remains the product solver and parity oracle.

## Work Performed

- `core/solver/sparse_direct` now exposes `SymmetricMatrixEntry`,
  `SymmetricProfileMatrix::from_entries`, adjacency from explicit symmetric
  entries, and `solve_symmetric_system_from_entries`.
- Entry assembly sums duplicate lower-triangle contributions and reports
  out-of-range entry indices as sparse numeric input errors.
- Product sparse evidence records now cite direct reduced profile-entry
  assembly and still record default sparse promotion as follow-on.
- Diagnostics status wording now distinguishes observed product direct profile
  assembly from unresolved default sparse promotion.

## Validation

- Sparse direct Rust tests passed 20/20.
- Diagnostics Rust tests passed 24/24.
- Product physics Rust tests passed 44/44.
- Full DEC-025 evidence sweep passed:
  `validation/evidence/sweeps/SWEEP_20260622T085210Z_799ebcc0dee5-dirty.json`
  (`overall_status=pass`, 5/5 surfaces).

## Boundaries Preserved

- No timing, memory, practical-size band, conditioning, CI threshold, default
  sparse promotion, release-readiness, professional approval, certification,
  sealing, authentication, or code-compliance claim was introduced.
- No protected standards content, proprietary benchmark model, private project
  data, network path, or telemetry feature was introduced.

## Residuals And Next Step

- Default sparse promotion remains measured follow-on work.
- Nonlinear/core profile-direct promotion and larger-scale sparse suitability
  evidence remain follow-on work.
