# WORKING_ITEMS Run Record - TP-R4-D7-SPARSEDEFAULTPROMOTE-001

## Scope

- Agent persona: `WORKING_ITEMS`
- Package: `PKG-04_Solver Core and Numerical Methods`
- Deliverable: `DEL-04-05_Sparse solver performance harness`
- Tranche: `TP-R4-D7-SPARSEDEFAULTPROMOTE-001`
- Target stage: R4 / Phase D under `DEC-048`
- Governing sparse rulings: `DEC-050`, `DEC-053`
- Human ruling basis: `D-26 = O-B`, hold at R4 pending named sparse
  default-promotion evidence/repair.

## Objective

Close the named R4 sparse residual selected by `DEC-053`:

- sparse default promotion;
- sparse timing;
- allocator/RSS memory;
- CI;
- hardware-normalized evidence;
- practical size band;
- true condition number;
- sparse conditioning evidence beyond the bounded generated-grid pivot-ratio
  proxy.

The closure is local R4 evidence and interaction-mode policy only. It does not
create release timing/RSS thresholds, hosted-CI activation, cross-machine
hardware-normalized pass/fail gates, professional approval, certification,
sealing, authentication, or code-compliance acceptance.

## Changed Surfaces

- `core/solver/nonlinear_integration`
  - Added `LinearSolveMode::{SparseInteractive,DenseScrutiny}`.
  - Made sparse interactive the default reduced linearized solve basis.
  - Preserved dense scrutiny as an explicit mode.
  - Records solver mode, solution basis, sparse profile stats, pivot-ratio
    proxy, sparse residual, and explicit dense-fallback status.
- `core/product_physics`
  - Added `PreviewSolverMode::{SparseInteractive,DenseScrutiny}`.
  - Added `run_linear_static_preview_with_mode`.
  - Emits `linear_solver_mode_basis` result rows and explicit
    `SPARSE_INTERACTIVE_DENSE_FALLBACK` diagnostics when fallback occurs.
  - Keeps dense scrutiny sparse-parity evidence available when selected.
- `apps/desktop/src-tauri`, `apps/desktop/src/services/previewService.ts`, and
  `apps/desktop/src/features/solve/SolvePanel.tsx`
  - Expose sparse interactive / dense scrutiny mode selection.
  - Default to sparse interactive.
  - Add solve-job packet metadata for the selected solver mode.
- `core/solver/performance_harness`
  - Adds `SparseDefaultPromotionObservationRecord`.
  - Adds `run_sparse_default_promotion_observation_suite`.
  - Adds deterministic true-condition-number observation via reduced dense
    symmetric matrix eigenvalue extrema for the bounded observation set.
  - Adds the JSON example emitter
    `examples/sparse_default_promotion_observation.rs`.
- `validation/benchmarks/sparse_default_promotion_observation.dec053.json`
  - Records 9 observations: 3 chain/line models, 3 generated grid models
    including `4x3` and `6x8`, and 3 product/nonlinear proxy fixtures.
  - Each record includes practical size band, reduced DOFs, dense/sparse timing,
    value-storage and RSS observations, platform/hardware metadata linkage,
    dense/sparse parity, residual, repeat determinism, pivot-ratio proxy, true
    condition number, and sparse/dense path roles.
- `validation/benchmarks/sparse_default_promotion_policy.dec053.json`
  - Records sparse interactive default policy, dense scrutiny policy, fallback
    visibility policy, required observation fields, and R4 closure statuses.
- `tests/test_sparse_default_promotion_observation.py`
  - Guards the 9-record packet, required fields, policy closure statuses, and
    absence of old dense-default/TBD closure text.
- Governance and coordination surfaces
  - `execution/_Decomposition/SOFTWARE_DECOMP.md` records `DEC-053`.
  - `D-26` moves to `RULED` in the decision register.
  - Coordination, roadmap, completion plan, completion log, and next-instance
    prompt now route the next R4 step to a refreshed final exit-chain packet.

## Validation

Focused checks:

- `cargo test --manifest-path core/solver/nonlinear_integration/Cargo.toml --lib`
  passed: 12 tests.
- `cargo test --manifest-path core/product_physics/Cargo.toml --lib` passed:
  45 tests.
- `cargo test --manifest-path core/solver/performance_harness/Cargo.toml --lib`
  passed: 20 tests.
- `python3 -m pytest -q tests/test_sparse_suitability_observation.py tests/test_sparse_default_promotion_observation.py`
  passed: 2 tests.
- `npm run generate:product-preview-mechanics` regenerated the invented product
  preview mechanics fixture with sparse interactive default metadata.
- `python3 -m pytest -q tests/product_preview/test_product_preview_service.py tests/test_results_schema.py tests/test_analysis_run_records.py`
  passed: 20 tests.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml --lib` passed:
  62 tests.
- `npm run test:desktop` passed: 19 files / 407 tests.
- `npm run build:desktop` passed.
- `git diff --check` passed.
- `python3 tools/coordination/list_deliverable_status.py --dag DAG-007 --format table --summary`
  passed with `CHECKING=8`, `IN_PROGRESS=92`, `ISSUED=1`.

DEC-025 local evidence sweep:

- `python3 tools/release/run_evidence_sweep.py --execute` passed all five
  surfaces.
- Summary:
  `validation/evidence/sweeps/SWEEP_20260623T020002Z_3194bd29f417-dirty.json`.
- Bound commit hash at sweep start:
  `3194bd29f417b8ca5489a5a524b16a460fb63260`.
- `working_tree_dirty=true`, as expected for this uncommitted tranche evidence.
- Surface statuses: cargo crate sweep `pass`, Python pytest `pass`, desktop
  Vitest plus wasm build `pass`, desktop Playwright dev+dist `pass`, desktop
  production build `pass`.

## Boundaries

- Sparse interactive is a preview/render/live-iteration default, not a release
  performance claim.
- Dense scrutiny remains available for detailed review and parity/scrutiny
  work.
- Timing/RSS/hardware data are observed and hardware-labeled only; no
  cross-machine threshold is asserted.
- True condition numbers are computed only for the bounded DEC-053 observation
  set and do not replace future release validation.
- No protected standards content, proprietary benchmark data, hidden support
  defaults, private project data, release-readiness claim, professional
  approval, certification, sealing, authentication, or code-compliance claim is
  introduced.

## R4 Handoff

`DEC-053` keeps R4 as the current target stage. The named sparse residual repair
has landed; the next R4 step is a refreshed final exit-chain verification
packet that includes `TP-R4-D7-SPARSEDEFAULTPROMOTE-001` and then presents a
new human R4 exit / R4-to-R5 ruling opportunity.
