# WORKING_ITEMS Run Record - TP-R4-D6-LIVECOVER-001

Date: 2026-06-21
Timestamp: 2026-06-21T01:54:23-06:00
Agent: WORKING_ITEMS
Target stage: R4 / Phase D
Tranche: TP-R4-D6-LIVECOVER-001
Decision basis: `DEC-044` and `DEC-046`

## Objective

Continue D6/D9 residual work by broadening product-preview live dense-loop
coverage beyond the already-landed one-way and explicit-normal friction paths.

This tranche adds deterministic product/result-envelope regressions for gap
closure and lift-off release through `core/product_physics::run_linear_static_preview`.
It does not change solver physics, bind the sparse solver, set measured
release convergence values, or claim R4 exit validation.

## Changes

- Refactored product-physics nonlinear preview tests to share a two-node
  invented nonlinear support request helper.
- Added a gap-closure product-preview test:
  - initial state `inactive`;
  - explicit positive-displacement clearance `0.05 mm`;
  - dense active-set loop converges in two iterations;
  - final state code is `1` (`active`);
  - final displacement row reports the explicit clearance.
- Added a lift-off release product-preview test:
  - initial state `active`;
  - explicit positive-reaction contact sense;
  - dense active-set loop converges in two iterations;
  - final state code is `0` (`inactive`);
  - final free-state reaction is zero.
- Preserved visible `TOLERANCE_POLICY_TBD`,
  `NONLINEAR_SUPPORT_STATE_REVIEW`, and
  `NONLINEAR_SUPPORT_LOOP_CONVERGED` diagnostics.

## Validation

Passed:

- `cargo fmt --manifest-path core/product_physics/Cargo.toml -- --check`
- `cargo test --manifest-path core/product_physics/Cargo.toml`
  - 38 tests
- `git diff --check`
- `python3 tools/release/run_evidence_sweep.py --execute`
  - overall pass across cargo crate sweep, Python pytest, desktop Vitest,
    desktop Playwright dev/dist, and desktop production build
  - summary:
    `validation/evidence/sweeps/SWEEP_20260621T075607Z_837d0febe6d9-dirty.json`

## Boundaries

- Product-preview regression evidence only; no public protected standards
  content, proprietary catalog value, private data, network/telemetry feature,
  public engineering default, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim.
- Dense loop only; no sparse live-path binding.
- No measured class-tiered convergence values; unmeasured `DEC-046` entries
  remain `TBD`.
- No derived friction normal-force model and no sliding friction force model.
- No lifecycle transition or R4 exit claim.

## Residuals

- Derived friction normal-force/sliding friction assembled/product validation
  remains open.
- Sparse live-path adoption remains gated by `D-17`.
- Measured class-tiered convergence values remain `TBD` until governed evidence
  exists.
- Broader live-solver coverage, the PRD section 16.2 branch-assembly
  benchmark, component provenance in the rendered report path, and the R4 exit
  evidence package remain open.
