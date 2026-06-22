# WORKING ITEMS RUN - TP-R4-D9-DISPREACTIONPOLICY-001

Date: 2026-06-22
Agent: WORKING_ITEMS
Package: PKG-09 - Verification, Validation, and Quality Oracles
Deliverable: DEL-09-03 - Nonlinear support regression suite
Target stage: R4 / Phase D

## Objective

Promote the current accepted nonlinear validation displacement/reaction delta
surfaces from observation-only ledgers to governed, fixture-evidence-envelope
threshold policies where the deterministic accepted fixture evidence supports
that promotion.

## Authority Basis

- PRD section 22.5: R4 requires nonlinear support validation cases to converge.
- `DEC-046`: class-tiered convergence policy can promote measured entries, but
  unmeasured/non-seed entries remain `TBD`.
- Prior evidence:
  - `TP-R4-D9-DISPREACTIONOBS-001` created the displacement/reaction delta
    observation ledgers.
  - `TP-R4-D9-WORKPOLICY-001` promoted bounded free-DOF work residual policies.
  - The accepted multi-support set currently contains eight public-original
    fixtures through `TP-R4-D9-MULTISUPPORTCASCADE-001`.

## Changes

- Added `validation/benchmarks/nonlinear/displacement_reaction_delta_policy.dec046.json`
  for the current assembled validation seed:
  - one-way/lift-off/friction limits: `100.0 mm`, `0.0 rad`, `10.0 N`,
    `0.0 N-m`;
  - gap limits: `50.0 mm`, `0.0 rad`, `5.0 N`, `0.0 N-m`.
- Added
  `validation/benchmarks/nonlinear/multisupport_displacement_reaction_delta_policy.dec046.json`
  for the accepted eight-fixture multi-support validation set:
  `100.0 mm`, `0.005 rad`, `10.0 N`, and `3.0 N-m`.
- Updated `validation/benchmarks/nonlinear/src/lib.rs` with
  `DisplacementReactionDeltaPolicyEntry`, governed policy inventories, residual
  predicates, and accepted policy refs on the current seed / accepted
  multi-support residual observations.
- Updated the focused Python guard, benchmark README, convergence observation
  note, seed fixture notes, accepted multi-support fixture notes, completion
  plan, coordination text, and DEL-09-03 memory.

## Validation

Focused validation:

- `cargo fmt --check --manifest-path validation/benchmarks/nonlinear/Cargo.toml`
  - passed.
- `cargo test --manifest-path validation/benchmarks/nonlinear/Cargo.toml`
  - passed, 17 tests.
- `python3 -m pytest -q tests/test_nonlinear_support_regression.py` - passed,
  8 tests.
- `python3 -m json.tool` over both new policy records and the generated sweep
  record - passed.
- `git diff --check` - passed.

Full DEC-025 evidence sweep:

- `python3 tools/release/run_evidence_sweep.py --execute` - passed all five
  surfaces:
  - cargo crate sweep;
  - repository pytest, 363 tests;
  - desktop Vitest, 407 tests;
  - Playwright dev/dist desktop lanes, 18 + 1 tests;
  - desktop production build.
- Passing sweep summary:
  `validation/evidence/sweeps/SWEEP_20260622T115411Z_b7426f628f24-dirty.json`

## Boundaries

- This tranche promotes only the current assembled validation seed and accepted
  eight-fixture multi-support set.
- The multi-support depth fixture remains observation-only / `TBD`.
- Product-preview displacement/reaction delta thresholds remain `TBD`.
- Broader non-seed force/displacement thresholds, broader displacement/reaction
  delta thresholds, general energy thresholds, sparse-default behavior, external
  validation thresholds, release thresholds, and CI thresholds remain open.
- No protected standards content, private project data, hidden support defaults,
  lifecycle transition, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim was added.

## Residuals

- Remaining D6/D9 work includes default sparse promotion, large-model sparse
  suitability thresholds, non-seed force/displacement threshold promotion beyond
  the accepted eight-fixture set, product-preview and broader
  displacement/reaction delta thresholds, general energy threshold promotion,
  deeper spring-hanger behavior, external validation threshold evidence, broader
  R4 validation packaging, and final R4 exit-chain evidence.
