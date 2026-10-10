# WORKING_ITEMS Run Record - TP-R4-D6-LIVEBUNDLE-001

Date: 2026-06-21
Timestamp: 2026-06-21T04:11:07-06:00
Agent: WORKING_ITEMS
Target stage: R4 / Phase D
Tranche: TP-R4-D6-LIVEBUNDLE-001
Decision basis: `DEC-044` and `DEC-046`

## Objective

Close the remaining D6 broader live-solver coverage residual by proving that
the product-preview dense active-set path can carry multiple nonlinear support
classes in one invented solve, not only one class per focused fixture.

This tranche is regression evidence only. It does not change solver physics,
bind the sparse solver, promote governed convergence thresholds, or claim R4
exit validation.

## Changes

- Added a deterministic `core/product_physics` regression for a bundled
  invented preview model with three nonlinear supports in one dense loop:
  one-way deactivation on `UX`, gap closure on `UY`, and explicit-normal
  friction sliding on `UZ`.
- The mixed solve now asserts:
  - three support state rows are emitted from one product-preview loop;
  - the loop converges in two active-set iterations with final residual count
    `0`;
  - the iteration metadata records `support_count=3`;
  - the one-way support ends `inactive`, the gap support ends `active` at the
    explicit `0.05 mm` clearance, and the friction support ends `sliding`;
  - explicit friction normal evidence remains a support evidence row and is
    still excluded from load-combination algebra.
- Preserved visible `TOLERANCE_POLICY_TBD`,
  `NONLINEAR_SUPPORT_STATE_REVIEW`, and
  `NONLINEAR_SUPPORT_LOOP_CONVERGED` diagnostics.

## Validation

Passed:

- `cargo fmt --manifest-path core/product_physics/Cargo.toml -- --check`
- `cargo test --manifest-path core/product_physics/Cargo.toml`
  - 41 tests
- `python3 -m pytest -q tests/product_preview/test_product_preview_service.py tests/test_results_schema.py tests/test_analysis_run_records.py tests/test_nonlinear_support_regression.py`
  - 28 tests
- `git diff --check`
- `cargo fmt --manifest-path core/solver/nonlinear_integration/Cargo.toml -- --check`
- `cargo test --manifest-path core/solver/nonlinear_integration/Cargo.toml`
  - 10 tests
- `python3 tools/release/run_evidence_sweep.py --execute`
  - overall pass across cargo crate sweep, Python pytest, desktop Vitest,
    desktop Playwright dev/dist, and desktop production build
  - summary:
    `validation/evidence/sweeps/SWEEP_20260621T101429Z_f74897170d66-dirty.json`

## Boundaries

- Product-preview regression evidence only; no public protected standards
  content, proprietary catalog value, private data, network/telemetry feature,
  public engineering default, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim.
- Dense loop only; no sparse live-path binding.
- No measured class-tiered convergence thresholds are promoted; unmeasured
  `DEC-046` entries remain `TBD`.
- No derived friction normal-force model and no generated sliding friction load
  model.
- No lifecycle transition or R4 exit claim.

## Residuals

- Sparse live-path adoption remains gated by `D-17`.
- Governed class-tier convergence threshold promotion remains `TBD` under
  `DEC-046`.
- The remaining D9 validation package and R4 exit evidence package remain open.
