# D-17 - Sparse Solver Live-Path Adoption Timing

**Status:** AWAITING_RULING - packet drafted; no ruling recorded. Only the human project authority rules.
**Prepared:** 2026-06-21 by WORKING_ITEMS, per the Application Integration And Issuance Loop decision-escalation step.
**Register row:** `execution/_Coordination/_DECISIONS/_REGISTER.md` row D-17.
**Plan basis:** `plans/PLAN_2026-06-17_prd_completion.md` §2.3 row D-17 and Phase D row D7.
**Epistemic posture:** evidence below is `FACT` with citations; inferences are labeled `ASSUMPTION`; the recommendation is labeled `PROPOSAL`; unknowns stay `TBD`. This packet decides nothing.

Citations pinned at repo HEAD `54f581a0b`. Symbol names are durable anchors if line numbers drift.

---

## 1. Decision Statement And Scope

**Decide:** whether the accepted in-repo sparse solver strategy (`DEC-023`,
`core/solver/sparse_direct`) should be bound into the live R4 solve path now,
and if so whether it becomes the default path or a governed evidence lane while
the dense path remains the default verification oracle.

This is a **timing and integration-shape decision**, not a sparse-strategy
decision. `DEC-023` already selected the in-repo skyline/profile direct solver
strategy. D-17 decides how that accepted solver enters the live
`frame_kernel` / `core/product_physics` / `core/solver/nonlinear_integration`
path during R4.

**In scope:** live reduced-system solve binding, solver-mode selection shape,
dense parity/fallback posture, diagnostics/reporting required while sparse is
not the default, and whether profile-direct assembly is a prerequisite or a
follow-on.

**Out of scope:** choosing a different sparse library, changing governed
verification tolerances (`DEC-024`/`DEC-026`), changing the nonlinear
convergence policy (`DEC-046`), claiming PRD R4 exit, issuing release readiness,
asserting professional/code-compliance acceptability, or using protected/private
benchmark data. This packet does not implement sparse adoption.

**What this blocks.** Phase D item **D7 sparse-solver live-path adoption** is
gated by D-17. It also affects the remaining D6/D9 evidence because the
assembled nonlinear loop multiplies the cost of each linear solve, and PRD §20
expects large models to use sparse matrix methods. If D-17 waits for ruling,
the sparse crate can remain measured in the harness but not treated as adopted
by the live product path.

---

## 2. Current State Evidence

### 2.1 Sparse Strategy Is Ruled And The First Solver Slice Exists

- **FACT:** `DEC-023` accepts D-03 Option C: a hand-rolled in-repo sparse
  skyline/banded direct solver with bandwidth/profile-ordered DOF numbering,
  skyline/profile storage, in-repo factorization, fixed deterministic operation
  order, zero new numerical dependencies, and benchmark evidence
  (`execution/_Decomposition/SOFTWARE_DECOMP.md` §12 row `DEC-023`).
- **FACT:** `core/solver/sparse_direct/src/lib.rs` states that it implements
  `DEC-023`: deterministic reverse Cuthill-McKee ordering, skyline/profile
  storage, and in-repo LDL^T profile factorization and solve
  (`core/solver/sparse_direct/src/lib.rs:1-11`).
- **FACT:** the sparse crate declares its pivot guard as an internal
  verification guard, not the project solver tolerance policy
  (`core/solver/sparse_direct/src/lib.rs:17-21`).
- **FACT:** the completion log records `TP-D03-SPARSE-001` as landed: new
  `sparse_direct`, diagnostics integration, and performance-harness sparse
  observations with parity tests and no threshold assertions
  (`plans/PLAN_COMPLETION_LOG.md`, "D7 first slice landed").

**Implication (`ASSUMPTION`):** D-17 should not re-open library selection. The
remaining choice is whether and how the accepted solver is used by live product
solves in R4.

### 2.2 Sparse Is Measured In The Harness, Not Yet Adopted By The Live Path

- **FACT:** `core/solver/performance_harness` imports
  `open_pipe_stress_sparse_direct::solve_symmetric_system` and records
  sparse-path observations alongside the dense path
  (`core/solver/performance_harness/src/lib.rs:16`, `:79-90`).
- **FACT:** `run_fixture_repeat` assembles/reduces once, then solves the same
  reduced system with dense first and sparse observation second
  (`core/solver/performance_harness/src/lib.rs:405-515`).
- **FACT:** `observe_sparse_path` calls
  `solve_symmetric_system(reduced_stiffness, reduced_force)`, records
  factorization diagnostics, sparse-vs-dense solution delta, residual, repeat
  determinism, profile entries, and elapsed time
  (`core/solver/performance_harness/src/lib.rs:522-575`).
- **FACT:** the harness still emits an explicit adoption-status diagnostic and
  limitation that "live solve-path adoption of the DEC-023 sparse skyline solver
  remains TBD" (`core/solver/performance_harness/src/lib.rs:394-404`,
  `:607-612`, `:747-749`).

**Implication (`ASSUMPTION`):** evidence exists to wire a bounded live lane, but
the current evidence posture is still "measured alongside dense," not "live
path adopted."

### 2.3 Live Product And Nonlinear Paths Still Call The Dense Solver

- **FACT:** `core/solver/nonlinear_integration` imports and calls
  `solve_dense`; its loop assembles, reduces with prescribed displacements, and
  solves the reduced system through the dense path
  (`core/solver/nonlinear_integration/src/lib.rs:10-11`, `:207`,
  `:535-541`).
- **FACT:** `core/product_physics` imports and calls `solve_dense`; its preview
  solve assembles global stiffness, reduces, and solves dense
  (`core/product_physics/src/lib.rs:9`, `:585`, `:748-749`).
- **FACT:** `core/solver/frame_kernel` exposes `solve_dense` as the current
  linear solve function (`core/solver/frame_kernel/src/lib.rs:805`).
- **FACT (`ASSUMPTION` on absence):** a repo search for
  `solve_symmetric_system` in live product/nonlinear paths finds it only in the
  performance harness and sparse crate tests, not in `core/product_physics` or
  `core/solver/nonlinear_integration`.

**Implication (`ASSUMPTION`):** without a D-17 ruling and follow-on tranche,
R4 nonlinear/product evidence remains dense-only at the live solve boundary.

### 2.4 Current Sparse API Still Starts From A Dense Reduced Matrix

- **FACT:** `sparse_direct::solve_symmetric_system(dense, force)` accepts a
  dense symmetric matrix and vector, then builds ordering/profile/factorization
  from that dense input (`core/solver/sparse_direct/src/lib.rs`; public API and
  storage constructors).
- **FACT:** the completion plan names profile-direct assembly as the follow-on
  after D-17 live-path adoption (`plans/PLAN_2026-06-17_prd_completion.md` row
  D-17 and Phase D row D7).

**Implication (`ASSUMPTION`):** binding the sparse solve now can de-risk the
solver lane and diagnostics, but it will not yet satisfy the full memory and
assembly-performance intent of PRD §20 for large models. Full sparse assembly
remains a follow-on unless the human chooses a more aggressive option.

### 2.5 PRD And SPEC Press Toward Sparse, But R4 Exit Does Not Name Sparse

- **FACT:** PRD §20 requires small tutorial models to solve interactively,
  medium models with hundreds of nodes in seconds, and large models with
  thousands of nodes using sparse matrix methods; it also requires responsive
  GUI behavior and progress/cancel/diagnostic logs for long solves
  (`docs/PRD.md:1128-1138`).
- **FACT:** PRD §22.5 R4 deliverables include components, spring hangers, and
  gaps/lift-off/friction. Its explicit exit criteria are "Nonlinear support
  validation cases converge" and "Component provenance appears in reports";
  sparse adoption is not named as an R4 exit criterion
  (`docs/PRD.md:1223-1237`).
- **FACT:** SPEC §5.5 requires deterministic solver results and reportable
  sparse-solver settings, tolerances, and conditioning warnings
  (`docs/SPEC.md:498-508`).

**Implication (`ASSUMPTION`):** sparse live adoption is not by itself an R4 exit
condition, but leaving all live R4 solves dense conflicts with the project-scale
trajectory and keeps D7 open.

---

## 3. Open Questions Awaiting Ruling

1. **Timing:** Must `core/solver/sparse_direct` enter the live solve path during
   R4, or can live adoption wait until R5/post-R4? `TBD`.
2. **Default posture:** If adopted in R4, is sparse the default live solve path,
   or an explicit user/developer/diagnostic solver-mode lane while dense remains
   default? `TBD`.
3. **Parity posture:** If sparse is live in R4, must dense parity/fallback be
   run in tests only, in diagnostic builds, or opportunistically for small
   invented/product fixtures? `TBD`.
4. **Profile-direct assembly:** Is dense assembly/reduction followed by sparse
   factorization acceptable for the R4 adoption slice, or must full
   profile-direct assembly land first? `TBD`.
5. **Diagnostic wording:** When a live sparse lane lands, should
   `SparseSolverTbd` be retired, reworded to "sparse default promotion TBD," or
   preserved only for dense-mode/product paths that have not opted into sparse?
   `TBD`.
6. **Evidence bar:** What focused evidence is required before default promotion:
   sparse/dense parity, residual bounds, repeat determinism, nonlinear-loop
   fixture parity, performance observations, or a human-reviewed threshold?
   `TBD`.

---

## 4. Options

### Option A - R4 Default Sparse Reduced-System Solve

Bind `sparse_direct` into the live reduced-system solve path during R4 and make
it the default for product and nonlinear integration solves. Keep dense as a
test oracle and fallback for explicit diagnostic use, not as the normal product
path.

- **For:** fastest closure of the "live path still dense" D7 gap; aligns
  visibly with `DEC-023` and PRD §20; gives D6 iterative solves immediate
  sparse factorization coverage.
- **Against:** the current sparse API still consumes a dense reduced matrix, so
  this does not yet remove dense assembly/memory; defaulting before a live-path
  parity/fallback contract is implemented increases regression risk in the R4
  nonlinear hinge.
- **Risk:** medium-high integration risk before R4 exit evidence, especially
  around singular/ill-conditioned diagnostics and fallback semantics.
- **Boundary:** code-neutral if diagnostics remain explicit and no thresholds or
  professional claims are inferred.

### Option B - R4 Live Sparse Evidence Lane, Dense Default (Recommended)

Bind `sparse_direct` into the live solve boundary during R4 as an explicit
solver-mode/evidence lane. Dense remains the default product path and parity
oracle until a later measured promotion. The live lane records sparse
observations, diagnostics, residuals, and parity against dense on invented
product/nonlinear fixtures. Profile-direct assembly is a follow-on.

- **For:** moves D7 from harness-only evidence into the live product/nonlinear
  seam without forcing a default flip before profile-direct assembly exists;
  preserves dense behavior for R4 exit work; gives the next tranche a concrete
  seam for diagnostics, reportable sparse settings, and promotion criteria.
- **Against:** two live solve modes add short-term complexity; PRD §20 large
  model intent remains only partially satisfied until sparse becomes default
  and profile-direct assembly/performance thresholds land.
- **Risk:** medium implementation risk, controlled by explicit mode selection,
  dense parity evidence, and no release/performance claims.
- **Boundary:** strongest for the current R4 state. Every output remains
  mechanics-only, deterministic, and diagnostic; no protected data, code tables,
  vendor values, or professional acceptability claims enter the repo.

### Option C - Defer Live Adoption To R5/Post-R4

Keep `sparse_direct` in the harness only for R4. Continue all product and
nonlinear integration solves through `solve_dense`. Prepare live sparse adoption
after R4 exit evidence or during R5 performance packaging.

- **For:** lowest immediate risk to D6/D9 R4 evidence; R4 exit text does not
  explicitly name sparse; avoids a dual-solver seam during current nonlinear
  work.
- **Against:** leaves D7 open, keeps `SparseSolverTbd` live-adoption warnings
  accurate, and lets R4 assembled nonlinear evidence stress the dense path only
  even though PRD §20 and SOW-035 point toward sparse practical-scale behavior.
- **Risk:** low implementation risk, medium planning/scale risk.
- **Boundary:** safe but postpones the adoption evidence the project already
  selected in `DEC-023`.

### Option D - Full Profile-Direct Sparse Assembly And Default In R4

Do not accept a reduced-system-only binding. Require full profile-direct
assembly/reduction into sparse storage and make sparse the default live path
within R4.

- **For:** closest to the long-term PRD §20 model-scale architecture; avoids an
  intermediate dense-matrix handoff that might be discarded later.
- **Against:** largest R4 blast radius; requires new assembly/storage interfaces
  and likely broader benchmark/performance policy before the current R4 exit
  evidence can stabilize.
- **Risk:** high. This could consume D6/D9 capacity and should be treated as a
  separate implementation program if chosen.
- **Boundary:** code-neutral if still invented/public and diagnostic-only, but
  the implementation scope is much broader than the current D7 adoption slice.

---

## 5. Advisory Recommended Disposition (PROPOSAL - human rules)

**Recommend Option B: R4 live sparse evidence lane, dense default.**

Rationale: Option B is the smallest ruling that honors `DEC-023` and the plan's
"bind in R4 alongside the nonlinear loop" direction while preserving the
current R4 exit path. The sparse solver is already deterministic and measured,
but the live product/nonlinear paths are still dense and the sparse API still
starts from dense reduced matrices. A live evidence lane creates the real seam
needed for D7 without over-claiming PRD §20 large-model readiness or forcing a
default solver change before profile-direct assembly, sparse setting reporting,
and promotion criteria are measured.

Recommended implementation shape if O-B is accepted:

1. Add a solver-mode seam at the reduced-system solve boundary used by
   `core/solver/nonlinear_integration` and `core/product_physics`.
2. Keep dense as the default mode for the technical preview.
3. Add an explicit sparse mode that calls
   `sparse_direct::solve_symmetric_system` on the same reduced system.
4. Record sparse diagnostics/observations and sparse-vs-dense parity evidence
   for invented product and nonlinear fixtures.
5. Reword `SparseSolverTbd` only after live sparse mode exists; until then, the
   current live-adoption TBD wording remains accurate.
6. Leave profile-direct assembly and default sparse promotion as named follow-on
   work, not silent assumptions.

---

## 6. Decision Dependencies And Relationships

- **D-03 / DEC-023 (already ruled):** D-17 consumes the selected solver
  strategy; it does not re-open library selection.
- **D-16 / DEC-044 (already ruled):** the assembled nonlinear integration
  tranche is the natural binding point for the per-iteration solve lane.
- **D-19 / DEC-046 (already ruled):** solver convergence policy remains
  separate. D-17 may record residual/parity evidence, but does not change
  convergence thresholds.
- **D9 validation:** sparse live-path evidence is one remaining D9 gap after
  the current assembled/product/branch benchmark seeds. D-17 ruling determines
  whether that evidence is R4 live-path evidence or deferred.
- **DEL-04-05:** continues to own deterministic performance and sparse-path
  observations. If O-B or O-A is accepted, DEL-04-05 should capture parity,
  timing, residual, and diagnostic observations for the live lane.

---

## 7. Ruling Record

`TBD`. Human project authority has not ruled D-17. On ruling, record the
accepted option in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12 (or the
successor governing register), update `_REGISTER.md`, then select the follow-on
D7 implementation tranche.

---

## 8. References

- `execution/_Decomposition/SOFTWARE_DECOMP.md` §12 row `DEC-023`.
- `plans/PLAN_2026-06-17_prd_completion.md` §2.3 row D-17 and Phase D row D7.
- `plans/PLAN_COMPLETION_LOG.md` entry "D7 first slice landed: in-repo sparse skyline direct solver (`TP-D03-SPARSE-001`)."
- `docs/PRD.md:1128-1138` (§20 performance requirements).
- `docs/PRD.md:1223-1237` (§22.5 R4 deliverables and exit criteria).
- `docs/SPEC.md:498-508` (§5.5 numerical quality and diagnostics).
- `core/solver/sparse_direct/src/lib.rs` - accepted sparse direct solver.
- `core/solver/performance_harness/src/lib.rs` - sparse observation lane and live-adoption TBD diagnostics.
- `core/solver/nonlinear_integration/src/lib.rs` - current dense nonlinear integration solve.
- `core/product_physics/src/lib.rs` - current dense product-preview solve.
