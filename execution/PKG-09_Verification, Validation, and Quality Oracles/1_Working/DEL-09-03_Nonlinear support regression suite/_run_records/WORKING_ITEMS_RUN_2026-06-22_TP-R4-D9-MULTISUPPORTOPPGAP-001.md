# WORKING ITEMS RUN - TP-R4-D9-MULTISUPPORTOPPGAP-001

Date: 2026-06-22
Agent: WORKING_ITEMS
Package: PKG-09 - Verification, Validation, and Quality Oracles
Deliverable: DEL-09-03 - Nonlinear support regression suite
Target stage: R4 / Phase D

## Objective

Add a bounded gap-only, opposing-direction accepted multi-support companion to
the current `DEC-046` nonlinear support validation fixture set without widening
the accepted policy envelope beyond measured fixture evidence.

## Authority Basis

- PRD section 22.5: R4 requires nonlinear support validation cases to converge.
- `DEC-044`: the assembled nonlinear solve is owned by the PKG-04 integration
  tranche bridging DEL-04-04 and DEL-04-01; the classifier remains the
  per-iteration state oracle.
- `DEC-046`: class-tiered convergence policy can promote measured entries, but
  unmeasured and broader entries remain `TBD`.
- Prior multi-support tranches promoted ten public-original companion fixtures
  under bounded active-set, free-DOF force/moment, free-DOF work,
  general-energy, and displacement/reaction-delta policies.

## Changes

- Added `NL-ASSEMBLED-MULTI-DOF-OPPOSING-GAPS-ACCEPTED-ORIGINAL`, an invented
  assembled frame fixture that closes a positive-direction `Ux` gap and a
  negative-direction `Uy` gap in one multi-support solve.
- Broadened the accepted multi-support validation fixture set from ten
  companions to eleven companions inside the existing active-set, free-DOF
  force/moment, free-DOF work, general-energy, and
  displacement/reaction-delta policy envelopes.
- Updated the multi-support active-set-count, free-DOF force/moment,
  free-DOF work, general-energy, displacement/reaction-delta observation, and
  displacement/reaction-delta policy records so their evidence fixture IDs name
  the eleven-fixture public-original set.
- Updated nonlinear regression guards, benchmark README text, hand-calculation
  notes, coordination surfaces, active R4 planning, the R4 exit-gap packet, and
  DEL-09-03 memory.

## Validation

Focused validation:

- `cargo fmt -- --check` in `validation/benchmarks/nonlinear` - passed.
- `cargo test --quiet` in `validation/benchmarks/nonlinear` - passed, 18
  tests.
- `python3 -m pytest -q tests/test_nonlinear_support_regression.py` - passed,
  8 tests.
- JSON syntax validation passed for the six modified multi-support policy and
  observation records.
- `git diff --check` - passed before the full evidence sweep.

Full DEC-025 evidence sweep:

- `python3 tools/release/run_evidence_sweep.py --execute` - passed all five
  surfaces:
  - cargo crate sweep;
  - repository pytest, 363 tests;
  - desktop Vitest, 407 tests;
  - Playwright dev/dist desktop lanes, 18 + 1 tests;
  - desktop production build.
- Passing sweep summary:
  `validation/evidence/sweeps/SWEEP_20260622T143831Z_7f73b463bd54-dirty.json`

## Boundaries

- This tranche promotes only the new gap-only opposing-direction companion
  inside the current eleven-fixture public-original multi-support set.
- It does not promote non-seed force/displacement thresholds beyond the
  accepted eleven-fixture set, broader displacement/reaction-delta thresholds,
  broader general-energy thresholds, sparse default behavior, release
  thresholds, external validation thresholds, total strain-energy thresholds,
  modal-energy thresholds, CI thresholds, or any lifecycle exit decision.
- It does not add protected standards content, proprietary benchmark output,
  private project data, hidden support defaults, lifecycle transition,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim.

## Residuals

- R4 remains not ready for exit. Remaining D6/D9 work includes non-seed
  force/displacement threshold promotion beyond the accepted eleven-fixture
  set, broader displacement/reaction-delta thresholds beyond accepted
  current/product/fixture surfaces, broader energy thresholds outside accepted
  current/product/fixture surfaces, default sparse promotion plus sparse
  timing/memory/conditioning/CI/hardware-normalized threshold evidence, deeper
  spring-hanger behavior, external validation thresholds, broader R4
  validation package work, and final R4 exit-chain evidence.
