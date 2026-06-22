# WORKING_ITEMS Run Record - TP-R4-D7-SPARSECONDITIONPOLICY-001

## Scope

- Agent persona: `WORKING_ITEMS`
- Package: `PKG-04_Solver Core and Numerical Methods`
- Deliverable: `DEL-04-05_Sparse solver performance harness`
- Tranche: `TP-R4-D7-SPARSECONDITIONPOLICY-001`
- Target stage: R4 / Phase D under `DEC-048`
- Governing sparse ruling: `DEC-050`

## Objective

Reduce the R4/D7 sparse residual by promoting one bounded generated-grid sparse
pivot-ratio conditioning policy for the existing sparse suitability observation
set.

The policy is intentionally narrow: it governs only the generated invented grid
fixtures already used by `DEC-050` sparse suitability observations and uses the
sparse factorization pivot proxy `max(abs(accepted_pivots)) /
min(abs(accepted_pivots))`. It is not a true matrix condition-number threshold.

## Changed Surfaces

- `core/solver/performance_harness/src/lib.rs`
  - Emits `DEC-050-SPARSE-GENERATED-GRID-PIVOT-CONDITIONING-POLICY-v1`.
  - Adds `sparse_pivot_condition_ratio_estimate` to sparse suitability
    observation records.
  - Records policy status
    `accepted_for_generated_grid_pivot_ratio_observation_set`.
  - Records generated-grid limit `1.0e16`.
- `validation/benchmarks/sparse_conditioning_threshold_policy.dec050.json`
  - Adds the governed sparse generated-grid pivot-conditioning policy record.
- `validation/benchmarks/sparse_suitability_observation.dec050.json`
  - Cites the conditioning policy and records the new observed metric.
- `validation/benchmarks/sparse_suitability_threshold_policy.dec050.json`
  - Preserves the existing dense-vs-sparse relative delta, sparse residual,
    repeatability, and nonpositive-pivot threshold policy while moving
    remaining conditioning work beyond the generated-grid pivot proxy.
- `tests/test_sparse_suitability_observation.py`
  - Guards the new policy record, observation references, metric, fixture IDs,
    and still-`TBD` residuals.
- `core/solver/performance_harness/README.md`
  - Documents the policy boundary and remaining sparse threshold residuals.
- Coordination and planning surfaces
  - Update R4 residual wording so `conditioning/CI` no longer implies that all
    generated-grid pivot-ratio conditioning evidence remains open.

## Validation

- `cargo fmt --manifest-path core/solver/performance_harness/Cargo.toml -- --check`
  passed.
- `cargo test --manifest-path core/solver/performance_harness/Cargo.toml --quiet`
  passed: 19 tests.
- `python3 -m pytest -q tests/test_sparse_suitability_observation.py` passed:
  1 test.
- JSON syntax checks passed for:
  - `validation/benchmarks/sparse_suitability_observation.dec050.json`
  - `validation/benchmarks/sparse_suitability_threshold_policy.dec050.json`
  - `validation/benchmarks/sparse_conditioning_threshold_policy.dec050.json`
- Full DEC-025 sweep passed 5/5 surfaces:
  `validation/evidence/sweeps/SWEEP_20260622T163532Z_3068ec39ed5e-dirty.json`.

## Boundaries

- Dense remains the default solve path and sparse remains an evidence/parity
  lane only.
- No default sparse promotion is made.
- No timing threshold, allocator/RSS memory threshold, practical-size band, CI
  gate, hardware-normalized methodology, true matrix condition-number threshold,
  release threshold, external validation threshold, professional approval,
  certification, sealing, authentication, or code-compliance acceptance is
  introduced.
- No protected standards content, proprietary benchmark data, hidden support
  defaults, or private project data is introduced.

## R4 Handoff

This tranche narrows the D7 sparse residual to work beyond the generated-grid
pivot-ratio proxy. R4 remains not ready for exit until `D-25` is ruled or the
selected residual set is closed by later bounded tranches.
