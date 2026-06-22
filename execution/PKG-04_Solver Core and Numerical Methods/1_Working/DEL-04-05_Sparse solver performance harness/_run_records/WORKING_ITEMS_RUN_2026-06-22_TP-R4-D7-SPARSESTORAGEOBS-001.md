# WORKING_ITEMS Run Record - TP-R4-D7-SPARSESTORAGEOBS-001

**Agent:** WORKING_ITEMS  
**Date:** 2026-06-22  
**Tranche:** `TP-R4-D7-SPARSESTORAGEOBS-001`  
**Scope:** deterministic storage-footprint observations in `DEL-04-05` sparse
performance harness evidence.

## Objective

Advance the remaining R4/D7 sparse timing-memory evidence gap by recording
deterministic f64 value-storage observations for reduced dense matrices and
sparse profile entries, without promoting sparse as the default solve path or
setting memory/timing thresholds.

## Authorities

- `execution/_Coordination/NEXT_INSTANCE_PROMPT.md`
- `execution/_Coordination/_COORDINATION.md`
- `plans/PLAN_2026-06-17_prd_completion.md` Phase D row D7
- `plans/VERIFICATION_2026-06-21_r4_exit_gap.md`
- `execution/_Coordination/_DECISIONS/D-17_sparse_solver_live_path_adoption.md`
- `execution/_Decomposition/SOFTWARE_DECOMP.md` decision log entries
  `DEC-023`, `DEC-026`, and `DEC-050`

## Changed Surfaces

- `core/solver/performance_harness/src/lib.rs`
  - Added deterministic value-storage fields for reduced dense matrices,
    original sparse profile entries, and ordered sparse profile entries.
  - Added suite-summary aggregates for reduced dense value-storage bytes and
    ordered profile value-storage bytes.
  - Added generated-grid sparse suitability value-storage observations and an
    ordered-sparse-vs-dense value-storage ratio.
  - Preserved dense as default and kept threshold/default-promotion boundaries
    in record limitations.
- `core/solver/performance_harness/README.md`
  - Documents value-storage observations as deterministic f64 storage evidence
    that excludes allocator/container overhead.
- `validation/benchmarks/sparse_suitability_observation.dec050.json`
  - Lists storage-footprint metric names and records
    `memory_observation_status=deterministic_value_storage_observed_threshold_tbd`.
- `tests/test_sparse_suitability_observation.py`
  - Guards the new metric names and non-threshold boundary.
- Coordination, active plan, gap packet, completion log, and DEL-04-05 memory
  surfaces were updated to include the tranche and remaining residuals.

## Validation

- `cargo fmt --manifest-path core/solver/performance_harness/Cargo.toml`
- `python3 -m json.tool validation/benchmarks/sparse_suitability_observation.dec050.json >/dev/null`
- `cargo test --manifest-path core/solver/performance_harness/Cargo.toml`
  - Result: 19/19 tests passed.
- `pytest -q tests/test_sparse_suitability_observation.py`
  - Result: 1/1 tests passed.
- `git diff --check`
- Full DEC-025 evidence sweep:
  - `validation/evidence/sweeps/SWEEP_20260622T132612Z_052c8cb5e277-dirty.json`

## Boundary

- Dense remains the product/default solve path and parity oracle.
- The storage values are deterministic f64 value-storage observations only;
  they exclude allocator/container overhead, resident-set size, platform memory
  counters, and hardware normalization.
- This tranche does not set timing thresholds, allocator/RSS memory thresholds,
  conditioning/CI thresholds, practical-size bands, hardware-normalized
  methodology, release/external validation thresholds, or default sparse
  promotion.
- No protected standards content, proprietary benchmark output, private project
  data, lifecycle transition, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim changed.

## Handoff State

- Accepted upstream snapshots/authorities consumed: `DAG-006`, `DEC-023`,
  `DEC-026`, `DEC-050`, and current R4 coordination state.
- Derivative package status: performance harness storage-footprint observations
  and metadata are derivative evidence from invented/public-original generated
  fixtures, not decomposition truth.
- Closure verdict: tranche implemented and focused validation passed; full
  DEC-025 sweep is recorded above.
- Remaining blockers: default sparse promotion, allocator/RSS memory
  thresholds, timing thresholds, practical-size bands, conditioning/CI
  thresholds, hardware-normalized methodology, non-seed D6/D9 nonlinear
  threshold residuals, deeper spring-hanger behavior, external validation
  thresholds, broader R4 validation package work, and final R4 exit evidence.
