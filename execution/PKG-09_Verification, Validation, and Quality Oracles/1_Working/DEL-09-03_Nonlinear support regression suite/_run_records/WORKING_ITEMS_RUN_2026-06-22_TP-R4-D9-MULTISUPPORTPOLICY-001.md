# WORKING ITEMS RUN - TP-R4-D9-MULTISUPPORTPOLICY-001

Date: 2026-06-22
Agent: WORKING_ITEMS
Package: PKG-09 - Verification, Validation, and Quality Oracles
Deliverable: DEL-09-03 - Nonlinear support regression suite
Target stage: R4 / Phase D

## Objective

Add one bounded accepted-policy companion fixture for an invented public
multi-DOF / multi-support nonlinear case, reducing the R4/D9 multi-support
acceptance gap without promoting unrelated residual, energy, sparse, release,
or external-validation thresholds.

## Authority Basis

- PRD §22.5: R4 requires nonlinear support validation cases to converge.
- `DEC-044`: the assembled nonlinear solve is owned by the PKG-04 integration
  tranche bridging DEL-04-04 and DEL-04-01; the classifier remains the
  per-iteration state oracle.
- `DEC-046`: class-tiered convergence policy can promote measured entries, but
  unmeasured/non-seed entries remain `TBD`.
- `TP-R4-D9-MULTISUPPORTOBS-001`: established an observation-only
  multi-DOF / multi-support Ux/Uy fixture and left broader accepted-policy
  coverage open.
- `TP-R4-D9-ENERGYOBS-001`: preserved free-DOF work/energy threshold promotion
  as a residual after adding observation-only work residual evidence.

## Changes

- Added `NL-ASSEMBLED-MULTI-DOF-MULTI-SUPPORT-ACCEPTED-ORIGINAL`, an invented
  public-original two-node frame case with simultaneous Ux one-way release and
  Uy gap closure.
- Added narrow `DEC-046` policy references for this non-seed companion:
  `DEC-046-CV-B-multisupport-active-set-count-validation-v1` and
  `DEC-046-CV-B-multisupport-free-dof-force-moment-residual-validation-v1`.
- Added machine-readable policy records for the multi-support active-set-count
  and free-DOF force/moment residual limits.
- Preserved the prior observation-only multi-support fixture separately from
  the accepted inventory.
- Updated nonlinear benchmark README content, hand-calculation notes,
  regression guards, coordination surfaces, and R4 exit-gap planning to record
  the accepted companion and the remaining open threshold axes.

## Validation

Focused validation:

- `cargo fmt --manifest-path validation/benchmarks/nonlinear/Cargo.toml --check` -
  passed.
- `cargo test --manifest-path validation/benchmarks/nonlinear/Cargo.toml` -
  passed, 14 tests.
- `python3 -m pytest -q tests/test_nonlinear_support_regression.py` - passed, 8
  tests.

Closeout validation:

- `python3 tools/release/run_evidence_sweep.py --execute` - passed all five
  DEC-025 surfaces:
  - cargo crate sweep;
  - repository pytest;
  - desktop Vitest after wasm build;
  - Playwright dev/dist desktop lanes;
  - desktop production build.
- Passing sweep summary:
  `validation/evidence/sweeps/SWEEP_20260622T074318Z_c2e41707f1d5-dirty.json`
- `git diff --check` - passed.

## Boundaries

- This tranche promotes only one invented non-seed Ux/Uy multi-support
  validation companion under narrow `DEC-046` active-set-count and free-DOF
  force/moment residual policies.
- It does not promote displacement-delta, reaction-delta, free-DOF work/energy,
  sparse default, product-preview release, external validation, spring-hanger,
  or general multi-support threshold policies.
- It does not add protected standards content, private project data, hidden
  support defaults, lifecycle transition, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim.

## Residuals

- Remaining D6/D9 work includes profile-direct sparse assembly/default sparse
  promotion, non-seed force/displacement and free-DOF work/energy threshold
  promotion beyond this companion, displacement/reaction-delta threshold axes,
  deeper spring-hanger behavior, external validation threshold evidence,
  broader multi-support acceptance coverage, and final R4 exit-chain evidence.
