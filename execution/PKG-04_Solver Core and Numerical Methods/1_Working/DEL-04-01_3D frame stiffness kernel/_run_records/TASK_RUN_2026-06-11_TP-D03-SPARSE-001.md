---
run-id: TASK_RUN_2026-06-11_TP-D03-SPARSE-001
timestamp: 2026-06-11T21:45:10-06:00
run-status: SUCCESS
control-surface: INLINE
scope-path: /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/core/solver
task-profile: NONE
task-skill: NONE
resolved-skill-path: NONE
resolved-skill-version: UNKNOWN
resolved-task-profile-requirement: NONE
companion-files: []
allowed-tools:
  - unrestricted
write-authorization: EXPLICIT_BRIEF_TEXT
runtime-overrides: {}
---

# TASK RUN - TP-D03-SPARSE-001 (DEC-023 sparse skyline solver, first bounded slice)

## Tranche and Authority Basis

- **Tranche:** completion-plan Phase D item D7, first bounded slice (`plans/PLAN_2026-06-10_prd_completion.md` §3 Phase D row D7).
- **Binding authority:** human ruling `DEC-023` (`execution/_Decomposition/SOFTWARE_DECOMP.md` §12 row DEC-023), selecting Option C of the decision packet `execution/_Coordination/_DECISIONS/D-03_sparse_solver_model_scale.md`: hand-rolled in-repo sparse skyline/banded direct solver; bandwidth/profile-ordered DOF numbering; skyline storage of the symmetric stiffness; in-repo LDLᵀ profile factorization; single-threaded fixed operation order preserving byte-reproducibility; zero new numerical dependencies; ordering/factorization/singularity-conditioning reporting carrying benchmark evidence under FR-008 and the D-04 tolerance regime.
- **Tolerance basis:** `DEC-026` (same table): analytic reference-result class seeded at 1.0e-9 relative. All parity test tolerances cite this seed; no tolerance was invented.
- **Design authority context:** PKG-04 deliverable contracts (DEL-04-01 `_CONTEXT.md` names the "sparse solve interface"; DEL-04-05/-06 Guidance forbid invented thresholds); `docs/PRD.md` §20 scale targets.

## Requested Tasks

- New crate under `core/solver/` per sibling conventions (manifest style, README, inline test layout, per-crate `.gitignore`).
- Deterministic bandwidth/profile-ordered DOF numbering, documented in the crate README.
- Skyline (profile) storage of the symmetric stiffness matrix.
- In-repo LDLᵀ profile factorization and solve, std-only, single-threaded, fixed operation order.
- Singularity/conditioning reporting integrated with the existing diagnostics taxonomy; resolve the `SparseSolverTbd` instrumented markers per the ruling.
- Parity evidence vs the existing dense solve on invented models (small frames plus a larger generated banded model).
- Performance-harness extension making the sparse path measurable alongside the dense path (measurement only; no thresholds).
- Frame-kernel live-path integration only if a small, obviously-safe seam; otherwise a handoff plan (it was not small — handoff surfaced below).

## Expected Outputs

- `core/solver/sparse_direct/` crate; bounded updates to `core/solver/diagnostics` and `core/solver/performance_harness`; this run record; dated MEMORY.md section.

## Tools Used

- shell `cargo fmt` / `cargo test` per touched crate.
- shell `git status` / `git diff --check` / `rg`-style `grep` for bounded intake and boundary review.
- patch editor for allowed file edits.
- One temporary `cargo run --example` probe (created and deleted within the run) to observe deterministic sparse behavior on the singular fixture before pinning a test.

## Tool Policy Compliance

N/A - no explicit tool allowlist was active.

## Write Authorization

- Inline brief explicitly authorized writes within `core/solver/**`, one run record in the owning PKG-04 deliverable folder, and an optional dated MEMORY.md section there.
- Owning deliverable discovered as `DEL-04-01_3D frame stiffness kernel` (its `_CONTEXT.md` description includes the "sparse solve interface").

## Outputs Produced

### New crate `core/solver/sparse_direct/` (open_pipe_stress_sparse_direct)

- `Cargo.toml` — empty `[dependencies]` (std only; DEC-023 zero-new-dependency requirement); `open_pipe_stress_frame_kernel` as dev-dependency for parity tests only.
- `src/lib.rs` —
  - `reverse_cuthill_mckee`: deterministic RCM ordering (symmetrized/sorted/deduped adjacency; components by lowest index; pseudo-peripheral start via level-structure iteration; all ties by ascending `(degree, index)`; no randomization).
  - `SymmetricProfileMatrix`: skyline (profile) storage, lower triangle by rows, identity or permuted construction from a dense symmetric matrix; documented symmetric-input contract (always reads the original lower-triangle entry).
  - `factorize_ldlt`: in-repo LDLᵀ profile factorization, fixed operation order, no pivot reordering; `SPARSE_SOLVE_ZERO_PIVOT_GUARD = 1.0e-12` mirroring the dense path's guard precedent (explicitly not tolerance policy); `FactorizationReport` with singular-pivot location (ordered + original indices), accepted-pivot extrema, pivot-ratio conditioning proxy (documented as a proxy), nonpositive-pivot count and first location.
  - `ProfileFactorization::solve_ordered` and `solve_symmetric_system` (order → store → factor → solve → inverse-permute) returning ordering/profile/factorization observations.
- `README.md` — documents the RCM choice and its determinism properties, the numerical determinism posture, the pivot guard and conditioning proxy, the symmetric-input contract, dependency posture, non-compliance boundary, and adoption-pending integration status.

### `core/solver/diagnostics` updates

- New `SolverDiagnosticCode::NonPositivePivot` (Warning / ASSUMPTION_WARNING class) with remediation text.
- `diagnostic_from_sparse_error`: singular pivots map to `SingularSystem` Failure with both ordered and original reduced indices and stable `reduced-dof:<n>` affected refs; invalid inputs map to blocking `InvalidNumericInput`.
- `diagnostics_from_factorization_report`: deterministic report-derived diagnostics (nonpositive pivots located and counted).
- `sparse_solver_tbd_diagnostic` reworded per the ruling: selection is resolved by DEC-023 (in-repo skyline LDLᵀ, `core/solver/sparse_direct`); the remaining `TBD` is live solve-path adoption. Code/severity/class unchanged (`SparseSolverTbd`, Warning, TBD class). `tolerance_policy_tbd_diagnostic` unchanged per DEC-026 (continues to fire until governed values are measured).
- README updated; 9 new tests.

### `core/solver/performance_harness` updates

- Sparse path runs alongside the dense path on the same reduced system in `run_fixture_repeat`: new `SparseSolveObservation` (ordering id, original/ordered profile entry counts and half-bandwidths, pivot extrema, pivot-ratio conditioning proxy, nonpositive-pivot count, sparse-vs-dense parity delta, sparse residual, repeat determinism delta, elapsed nanos) and `dense_first_solve_elapsed_nanos` on `HarnessRunRecord`. All measurement-only; no thresholds asserted (D-04 governs).
- Documented deterministic diagnostic ordering (status TBDs, dense conditioning, dense failure, sparse report diagnostics, sparse conditioning, sparse failure).
- New `invented_grid_frame_fixture(x, y)` (synthetic planar grid frame; fixed base row; invented lateral load) for banded-but-not-chain ordering evidence.
- Suite summary aggregates: sparse observation count, total original/ordered profile entries, max parity delta, max sparse residual.
- Record/suite assumptions and limitations updated to the DEC-023 truth ("live solve-path adoption ... remains TBD" replaces "sparse numerical library remains TBD").
- README updated; 6 new tests.

### `core/solver/frame_kernel` (README only)

- One stale sentence ("The project sparse numerical library remains `TBD`") replaced with DEC-023-aware wording (strategy resolved; live-path binding remains TBD; dense path is the parity oracle). No code change.

## Validation Evidence

All commands `cargo test --manifest-path <crate>/Cargo.toml`, all green:

| Crate | Result |
|---|---|
| `core/solver/sparse_direct` | 18 passed, 0 failed (all new) |
| `core/solver/diagnostics` | 24 passed, 0 failed (15 existing + 9 new) |
| `core/solver/performance_harness` | 18 passed, 0 failed (12 existing + 6 new) |
| `core/solver/frame_kernel` (sanity; code untouched) | 34 passed, 0 failed |
| `core/solver/nonlinear_supports` (downstream diagnostics consumer; untouched) | 16 passed, 0 failed |

- `cargo fmt --check` clean on all touched crates.
- `git diff --check` clean on all touched paths.
- Parity evidence: sparse-vs-dense max-abs solution delta within 1.0e-9 (DEC-026 analytic seed) scaled by the dense reference solution magnitude on: 4-element chain, 4x3 grid frame, 60-element chain (366 total DOFs / 360 reduced), and 6x8 grid frame (288 total DOFs / 252 reduced). Repeat sparse solves asserted bitwise identical.
- Hand-checked LDLᵀ case asserts exact (tolerance-free) factor pivots and solution on a dyadic-exact 3x3 system.

## Boundary Review

- Writes were limited to `core/solver/**` (new `sparse_direct/`, `diagnostics/`, `performance_harness/`, one README sentence in `frame_kernel/`), this run record, and the DEL-04-01 `MEMORY.md` dated section.
- No edits to `docs/`, `schemas/`, `apps/**`, `plans/**`, `execution/_Coordination/**`, `execution/_Decomposition/**`, or `fixtures/**`; all test models are generated in test code (no committed fixtures). No git commit/push; no network; zero new external dependencies; no threads; no protected standards content.
- During closeout, unrelated parallel working-tree changes were visible (apps/desktop, core/reporting, core/rules, init/init-prompt.md, a PKG-06 run record, fixtures/rule_expressions). They were treated as disjoint parallel work and were not edited or reverted. Note: `init/init-prompt.md` carries a trailing-whitespace `git diff --check` finding from that parallel work, not from this run.

## Missing

- none

## Needs Human Ruling

- none required for this slice (DEC-023 ruling implemented as scoped). Follow-ups listed under Dependency Notes are tranche-level, not ruling-level, except where noted in the packet's own follow-up plan (D-04 thresholds).

## Dependency Notes

### Handoff: frame_kernel/product_physics live-path integration plan (out of this tranche)

Integration was not a small, obviously-safe seam: the live solve path is `core/product_physics::run_linear_static_preview` → `assemble_global_stiffness` → support application → `reduce_system` → `solve_dense`, and `core/product_physics` is outside this tranche's write scope; switching the live path also changes the product result envelope, which is evidence-gated. Proposed next bounded tranche:

1. Route the reduced-system solve in `core/product_physics` (and/or expose a seam in `frame_kernel` at its documented "Future Replacement Point") to `open_pipe_stress_sparse_direct::solve_symmetric_system`, retaining `solve_dense` as the verification oracle per the DEC-023 evidence posture.
2. Wire `diagnostic_from_sparse_error` / `diagnostics_from_factorization_report` (already landed in `core/solver/diagnostics`) into the product diagnostics envelope so located singular pivots and nonpositive-pivot warnings reach the desktop surface.
3. Verify the product preview fixture (`fixtures/product_preview/`) parity within the DEC-026 governed regime before any re-baseline; any fixture change is its own evidence-gated step.
4. Revisit `SparseSolverTbd` wording/severity once adoption lands (the code currently records "live solve-path adoption remains TBD" — accurate until then).

### Other residuals / surfaced discrepancies (report, not resolved)

- **Assembly-side sparsity:** this slice consumes the dense-assembled reduced matrix; O(n²) assembly memory is still paid upstream. A follow-up slice (profile-direct assembly from element DOF maps) is needed to fully serve PRD §20 "thousands of nodes" memory scale. The solve-side strategy of DEC-023 is implemented; the assembly seam is the remaining scale cost.
- **Stale prose vs DEC-023 (not edited; outside scope):** DEL-04-06 `Guidance.md`/`Specification.md`/`Datasheet.md` still describe sparse-solver *selection* as TBD; DEL-04-01 `Guidance.md` row "Sparse solver library | TBD" and DEL-04-05 `Guidance.md` "Approved sparse solver/library choice — TBD" are now resolved by DEC-023. These update at their formal deliverable reviews.
- **Timing/memory thresholds:** elapsed-time fields are recorded measurement-only; peak-memory observation remains TBD (profile-entry counts are recorded as the documented storage proxy). Thresholds remain governed by D-04 (DEC-024 Part 2 / D-04b).
- **Conditioning thresholds:** the pivot-ratio proxy is recorded and classified only through caller-supplied thresholds via the existing `classify_condition_ratio`; no numeric thresholds were introduced (DEL-04-06 Guidance keeps them TBD).
- **Conflicts between ruling and existing code:** none found. The instrumented `SparseSolverTbd` message ("selection remains TBD") was the only statement contradicted by the ruling, and updating it was explicitly in-brief.

## Applied Changes

- Created `core/solver/sparse_direct/` (Cargo.toml, .gitignore, README.md, src/lib.rs).
- Updated `core/solver/diagnostics/{Cargo.toml, README.md, src/lib.rs}`.
- Updated `core/solver/performance_harness/{Cargo.toml, README.md, src/lib.rs}`.
- Updated one sentence in `core/solver/frame_kernel/README.md`.
- Created this run record; appended a dated section to DEL-04-01 `MEMORY.md`.

## Proposed Changes

- none beyond the handoff plan above.
