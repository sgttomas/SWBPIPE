# D-03 — Sparse Solver / Model-Scale Strategy (Sparse Library vs Bounded Dense Limit with Diagnostics)

**Status:** RULED — 2026-06-11 the human project authority selected **Option C** (hand-rolled in-repo sparse skyline/profile direct solver; zero new numerical dependencies; determinism posture preserved). Recorded as `DEC-023` in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12; register row updated.
**Prepared:** 2026-06-11 by TASK (Type 2), requested by WORKING_ITEMS (Type 1), tranche TP-DECIDE-PREP-001 decision-preparation subscope.
**Register row:** `execution/_Coordination/_DECISIONS/_REGISTER.md` row D-03.
**Plan basis:** `plans/PLAN_2026-06-10_prd_completion.md` §2 row D-03; §3 Phase D (rows D6/D7).
**Epistemic posture:** evidence below is `FACT` with citations; inferences and general numerical-libraries knowledge are labeled `ASSUMPTION`; the recommendation is labeled `PROPOSAL`; unknowns stay `TBD`. This packet decides nothing.

---

## 1. Decision statement and scope

**Decide:** the model-scale strategy for the linear-static mechanics solver — adopt a sparse linear-algebra implementation (third-party library or hand-rolled), or keep the existing dense solve behind an explicit, measured, diagnosed model-size bound — including the determinism, dependency-audit, and diagnostic-honesty posture of whichever path is selected.

**In scope:** global stiffness matrix storage and factorization strategy; whether/when a sparse path is adopted and from where (library vs in-repo); the bounded dense limit and its named diagnostic if dense is retained; how the bound is derived (measured, not invented); reproducibility requirements on the solve path (the evidence chain depends on byte/numerical reproducibility); dependency-audit/license posture of any new crate.

**Out of scope:** element formulations and stress recovery (DEL-04-02, PKG-05 — unchanged by storage strategy); nonlinear active-set iteration logic itself (Phase D row D6 — it multiplies solve *count*, not solve *method*); numerical tolerance and coverage thresholds (D-04); solver *tolerance policy*, which `TolerancePolicyTbd` tracks separately (`core/solver/diagnostics/src/lib.rs:535–542`); dynamic analysis (FR-024, Could).

D-03 blocks Phase D scale targets and PRD §23.1 "Solver stability for large models" (`plans/PLAN_2026-06-10_prd_completion.md:42`; `docs/PRD.md:1264`), and is timed "Before Phase D nonlinear work (iteration multiplies solve cost)" (`plans/PLAN_2026-06-10_prd_completion.md:42`). Phase D row D7 consumes the ruling: "Scale strategy per D-03: sparse assembly/solve if accepted (or documented dense bounds), performance-harness extension" (§3 Phase D table).

---

## 2. Current state evidence

Citations pinned at repo HEAD `5079a8fa7`. Symbol names are the durable anchors if line numbers drift.

### 2.1 The current solver is dense, hand-rolled, and dependency-free

- **FACT — storage:** the frame kernel's global matrix type is `pub type DenseMatrix = Vec<Vec<f64>>` (`core/solver/frame_kernel/src/lib.rs:25`). `assemble_global_stiffness` allocates the full `total_dofs × total_dofs` dense matrix (`vec![vec![0.0; total_dofs]; total_dofs]`) and scatters 12×12 element matrices into it (`lib.rs:669–693`).
- **FACT — factorization:** `solve_dense` is in-repo Gaussian elimination with partial pivoting and back-substitution; zero-pivot guard `DENSE_SOLVE_ZERO_PIVOT_GUARD = 1.0e-12` (commented "not the project solver tolerance policy"), failure surfaces as `FrameKernelError::SingularSystem { pivot }` (`lib.rs:20–21, 805–865, 359, 404`).
- **FACT — dependency posture:** `core/solver/frame_kernel/Cargo.toml` has an **empty** `[dependencies]` section — pure Rust, no nalgebra, no external linear algebra. The crate describes itself as "Code-neutral 3D frame stiffness kernel" and its header excludes standards content (`lib.rs:1–4`).
- **FACT — DOF bookkeeping:** fixed 6 DOF/node (`DOF_PER_NODE`, `lib.rs:9`), global index = `node_index * 6 + dof` (`node_dof_index`, `lib.rs:75–77`), element scatter map via `element_dof_map` (`lib.rs:79–87`). Boundary handling partitions restrained/prescribed DOFs into a `ReducedSystem { stiffness, force, free_dofs }` (`lib.rs:601–606, 695–705`) — dense again after reduction.
- **FACT — no size limit exists:** no model-size bound, node cap, or "model too large" diagnostic exists anywhere in `core/solver/` or `core/product_physics/` (symbol search for size-limit names returns nothing). An oversized model today would degrade silently (memory growth, long solve) rather than produce a named diagnostic.

### 2.2 The sparse choice is an institutionally reserved TBD, already instrumented as such

- **FACT:** DEC-012 reserves "solver numerical library" (among others) as an implementation-level TBD "unless a sealed brief or later human ruling resolves them" (`execution/_Decomposition/SOFTWARE_DECOMP.md` §12 Decision log, row DEC-012, `:582`). DEC-009 fixes the Rust-core baseline the choice must live inside (`:579`).
- **FACT:** the diagnostics crate carries dedicated codes `SparseSolverTbd` and `TolerancePolicyTbd` in `SolverDiagnosticCode` (`core/solver/diagnostics/src/lib.rs:136–146`); `sparse_solver_tbd_diagnostic()` emits the warning "sparse numerical solver selection remains TBD; dense solve path is for bounded verification only" (`lib.rs:526–533`). The dense path is **self-described as a stand-in**.
- **FACT:** PKG-04 deliverable guidance records the same posture: DEL-04-01 "Sparse solver library | TBD" and performance targets TBD (`execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-01_3D frame stiffness kernel/Guidance.md:32, 34, 41`); DEL-04-05 lists "Approved sparse solver/library choice — TBD", "Approved practical model-size taxonomy — TBD", and forbids inventing runtime/memory/model-size thresholds (`DEL-04-05_Sparse solver performance harness/Guidance.md:11, 44–47`); DEL-04-06 keeps conditioning thresholds TBD pending solver-library evidence (`DEL-04-06_Solver diagnostics and singularity detection/Guidance.md:25, 46`).

### 2.3 PRD scale targets and selection criteria

- **FACT:** PRD §20 initial performance targets: solve small tutorial models interactively; "Solve medium models with hundreds of nodes in seconds on a standard engineering workstation"; "Solve large models with thousands of nodes using sparse matrix methods"; keep the GUI responsive with progress, cancellation, and diagnostic logs for long solves; "Performance must not compromise auditability or unit safety" (`docs/PRD.md:1128–1138`).
- **FACT:** the PRD's lineage statement names "sparse 3D frame finite elements" as part of the modernization thesis (`docs/PRD.md:39`), and §19.2 technology selection criteria include "Numerical performance", "Sparse matrix support", and "Open-source dependency licenses" (`docs/PRD.md:1098–1099, 1104`).
- **FACT:** FR-008 (Must) requires solving linear static global flexibility problems against benchmarks (`docs/PRD.md:341`); §15.1 requires "Solver diagnostics" as report content (`docs/PRD.md:864`); §23.1 lists "Solver stability for large models" and "Reproducibility of reports across platforms" as success metrics (`docs/PRD.md:1264, 1266`).
- **ASSUMPTION (arithmetic projection from the FACT storage layout, unmeasured):** at 6 DOF/node, a dense `f64` matrix is `(6n)² × 8` bytes — ~2.9 MB at 100 nodes, ~288 MB at 1,000 nodes, ~7.2 GB at 5,000 nodes, before `Vec<Vec<_>>` per-row overhead, with O(m³) elimination cost. The PRD's "thousands of nodes" target is therefore not plausibly servable by the current dense path; "hundreds of nodes" may be. **TBD:** actual measured memory/time at any size — no measured records exist in-repo.

### 2.4 Existing instrumentation: a determinism/conditioning harness that measures neither time nor memory

- **FACT:** `core/solver/performance_harness` ("Deterministic sparse-solver performance and conditioning harness… does not… select a production sparse solver", `src/lib.rs:1–5`) already records per-fixture `total_dofs`, `stiffness_nonzero_count`, `reduced_stiffness_nonzero_count`, repeat-run determinism (`max_abs_solution_delta` from first run), `max_abs_residual`, and a diagonal `ConditioningObservation` (`lib.rs:61–90`), and stamps every run with the `SparseSolverTbd`/`TolerancePolicyTbd` warnings (`lib.rs:269–276, 388–389, 496–497`).
- **FACT:** `HarnessRunRecord` contains **no elapsed-time or memory fields** (`lib.rs:71–90`). Deriving a measured dense bound requires the "performance-harness extension" the plan already names in row D7. Fixture provenance is typed (`FixtureProvenanceStatus::InventedPublic` etc., `lib.rs:19–32`), so scale fixtures can stay invented/public.

### 2.5 What the product actually solves today

- **FACT:** the desktop app's only live solve path is the native Tauri command `run_preview_mechanics` → `open_pipe_stress_product_physics::run_linear_static_preview` (`apps/desktop/src-tauri/src/lib.rs:8, 847, 1559`; `apps/desktop/src-tauri/Cargo.toml:16`), which calls `assemble_global_stiffness` / `reduce_system` / `solve_dense` (`core/product_physics/src/lib.rs:8–9, 390, 545`) with support springs added on the diagonal (`lib.rs:394–396`).
- **FACT:** the evidenced solve scale is the invented preview model — **5 nodes, 4 pipe segments, 3 supports** (`fixtures/product_preview/invented_preview_model.json`; `fixtures/product_preview/invented_mechanics_result.json:12` `node_count: 5`) — i.e. 30 global DOFs. The smoke record's "647 computed result rows" (`apps/desktop/SMOKE.md:254–256, 263–266, 318`) are result-*kind* rows recovered from that 5-node solve, not 647 DOFs. No larger model has ever been solved in-product.
- **FACT:** browser mode does not solve at all: `runBrowserPreviewMechanics` returns the canned fixture for the unedited model and a blocked envelope for edited models (`apps/desktop/src/services/previewService.ts:33, 384–389`). The wasm32 engine adopted under DEC-020 is the *operation* engine (`operation_applier`), not a mechanics engine (`SOFTWARE_DECOMP.md:590`; `apps/desktop/scripts/build-wasm-engine.mjs:19`).

### 2.6 Determinism and dependency-boundary constraints on whichever path is chosen

- **FACT:** repeat-run reproducibility is already an explicit harness observable (§2.4), and cross-platform report reproducibility is a PRD success metric (§2.3). Phase D row D6 wraps the active-set classifier (`core/solver/nonlinear_supports/src/lib.rs:1–5` — "does not assemble a global solve") in a global iteration loop, so every nonlinear case multiplies solve count.
- **ASSUMPTION:** the current in-repo dense elimination is single-threaded straight-line `f64` arithmetic with a deterministic pivot rule, so identical inputs give identical bytes on a given build, and IEEE-754 semantics make cross-platform agreement achievable; third-party high-performance kernels commonly use SIMD dispatch, threading, and blocking strategies that can vary results across CPUs/platforms unless explicitly constrained. **TBD:** no cross-platform numerical-identity measurement exists in-repo for any path.
- **FACT:** dependency-license diligence is a named selection criterion (`docs/PRD.md:1104`), and the only external crates in the solve path today are `serde`/`serde_json` in the adapter (`core/product_physics/Cargo.toml`), keeping the mechanics core trivially auditable.

---

## 3. Open questions awaiting ruling

1. **Strategy:** sparse now, or bounded dense now? If sparse: third-party library or in-repo implementation? If bounded dense: the bound's *derivation* must come from measured harness data — `TBD` until measured (no time/memory instrumentation exists yet, §2.4).
2. **Determinism bar:** must the solver produce byte-identical results across platforms (strongest evidence-chain posture), or identical-within-declared-tolerance (which itself waits on D-04)? This materially constrains library candidates (§2.6 ASSUMPTION).
3. **Dependency posture:** is a new third-party numerical dependency acceptable in the mechanics core at all, given the zero-dependency kernel (§2.1) and license-diligence criterion (§2.3)? `TBD`: no project dependency-acceptance policy document exists beyond PRD §19.2 criteria.
4. **Diagnostic shape if dense is retained:** a named blocking-or-warning code (e.g. a new `SolverDiagnosticCode` variant alongside `SparseSolverTbd`, §2.2) when a model exceeds the bound — and whether exceeding blocks the solve or warns and proceeds. Silent degradation is excluded by the project's diagnostic-honesty posture (§2.2, §2.3 PRD §20 "diagnostic logs for long solves").
5. **Trigger and timing:** if staged, what measured trigger reopens the question (model size in DOFs? measured solve time? memory ceiling?), and is the re-decision pinned to the Phase D lead-up (the plan's stated D-03 timing) or to a measured event? `TBD` numbers either way until the harness measures them.
6. **Nonlinear interaction:** D6's iteration loop re-solves repeatedly; does the ruling require the chosen path to support efficient refactorization/reuse (e.g. factor-once-iterate-many), or is that an implementation detail below ruling level? (`plans/PLAN_2026-06-10_prd_completion.md` §3 Phase D rows D6/D7.)

---

## 4. Options

All options keep the existing kernel API seams (`assemble_global_stiffness` / `reduce_system` / solve, §2.1), keep `SparseSolverTbd` honesty until resolved, are local-only (no network/cloud), and make no release/certification claims.

### Option A — Stay dense with an explicit measured bound and a named diagnostic

Keep `solve_dense` as the only path. Extend the performance harness with elapsed-time/peak-memory observation (plan row D7's "performance-harness extension"), measure invented/public fixtures at increasing sizes on reference hardware, and have the human accept a bounded model-size limit derived from those measurements. Exceeding the bound emits a named diagnostic (new `SolverDiagnosticCode` variant) and blocks or warns per ruling; the bound and its measurement provenance appear in solver diagnostics and reports (PRD §15.1, §2.3).

- For: cheapest; zero new dependencies preserves the zero-dependency, trivially auditable kernel (§2.1) and the strongest determinism posture (§2.6); replaces today's silent-degradation gap (§2.1) with an honest, evidence-backed limit; defers the library question without pretending it is closed (`SparseSolverTbd` remains emitted).
- Against: arithmetic projection says "thousands of nodes" is out of reach dense (§2.3 ASSUMPTION), so PRD §20's sparse target and the §2 lineage statement stay unmet — acceptable only if explicitly re-scoped or re-scheduled by the human; Phase D nonlinear iteration multiplies the dense cost (§2.6); the bound is hardware-relative (`TBD` how the reference workstation is defined).

### Option B — Adopt a pure-Rust sparse linear-algebra dependency

Introduce a third-party sparse library behind the existing kernel seams: sparse assembly (triplet → CSR/CSC) in `assemble_global_stiffness`'s successor and a sparse direct factorization (symmetric-positive-definite Cholesky-class) for the reduced system. **ASSUMPTION (general knowledge, not repo fact; all properties unverified in-repo and `TBD` pending audit):** candidate pure-Rust crates exist — e.g. `faer` (dense+sparse high-performance kernels, permissive license), `sprs` (sparse structures, permissive license, factorization support more limited), `nalgebra-sparse` (Apache-2.0, less mature factorization) — each requiring license, MSRV, maintenance-health, and determinism verification before adoption.

- For: the only option that plausibly reaches PRD §20 "thousands of nodes" at bounded engineering cost; mature library numerics reduce in-house defect risk; aligns with §19.2 "Sparse matrix support" criterion.
- Against: first numerical dependency in the mechanics core — license/supply-chain audit burden and a larger trusted base for the evidence chain (§2.6); ASSUMPTION: SIMD/threading/blocking in performance-oriented libraries can break cross-platform byte-identity unless pinned/configured, and verifying that is real work (`TBD`: no such measurement exists); library evolution couples reproducibility-of-record to upstream release behavior; dependency-acceptance policy itself is an open question (§3 Q3).

### Option C — Hand-rolled sparse skyline/banded direct solver in-repo

Implement the classic piping-stress-lineage approach in the kernel: bandwidth/profile-ordered DOF numbering, skyline (profile) storage of the symmetric stiffness, and an in-repo LDLᵀ/Cholesky profile factorization — same zero-dependency posture as today, sparse-scaled.

- For: full determinism control (single-threaded, fixed operation order — the same byte-reproducibility argument as the current dense path, §2.6); zero new dependencies, smallest auditable trusted base; frame topologies are exactly the banded/profile structures this method was built for (ASSUMPTION: classic structural-analysis practice, general knowledge); harness nonzero/conditioning instrumentation already anticipates it (§2.4).
- Against: highest implementation and verification cost — ordering, profile storage, factorization, and singularity reporting all become in-repo numerical code needing its own benchmark evidence (FR-008, D-04 interplay); in-house numerics carry defect risk a mature library has already paid down; slowest path to Phase D if scheduled before D6/D7; performance ceiling below tuned library kernels (ASSUMPTION) — likely irrelevant at "thousands of nodes" but unmeasured (`TBD`).

### Option D — Staged: measured dense bound now; named sparse re-decision at the Phase D gate

Option A executed now (harness time/memory extension → measured bound → named diagnostic), plus a **pre-committed, named re-decision** (suggested D-03b) at the Phase D lead-up — before D6/D7 implementation starts — that selects Option B vs Option C using the harness measurements, with the determinism bar and dependency-acceptance criteria fixed *now* as part of this ruling so candidate evaluation is mechanical, not re-litigated.

- For: nothing unmeasured gets decided — bound, trigger, and library choice all become evidence-driven; Phases A–C proceed unblocked at today's scale (§2.5: largest evidenced solve is 5 nodes); the honesty diagnostics stay accurate throughout; mirrors the accepted D-10 pattern (canonical-now + named follow-up) already ratified by the human authority.
- Against: two rulings instead of one — if the human already knows sparse is mandatory for the product thesis (§2.3 lineage), staging adds latency to Phase D; risk that D-03b slips and Phase D starts against an unsuitable dense base; the measured-trigger discipline requires the harness extension to actually get scheduled (it currently exists in no tranche except as the D7 row).

---

## 5. Recommendation — `PROPOSAL`

Adopt **Option D — staged, with the determinism and dependency bars fixed in this ruling**:

1. **Now (any Phase A–C tranche):** extend the performance harness with elapsed-time and peak-memory observation over invented/public scale fixtures (the plan's D7 "performance-harness extension", pulled forward); record measured solve behavior at stepped sizes on declared reference hardware. All numbers stay `TBD` until those records exist.
2. **Now:** introduce the bounded-dense diagnostic — a named `SolverDiagnosticCode` variant (e.g. `ModelSizeBoundExceeded`) emitted when a model exceeds the human-accepted measured bound, with the bound value and its measurement provenance carried in the diagnostic and report content (PRD §15.1). Until a measured bound is accepted, the existing `SparseSolverTbd` warning remains the honest stand-in; no silent degradation in either state.
3. **Now:** fix the acceptance bars for any future sparse path: (a) single-threaded deterministic mode with repeat-run byte-identity demonstrated in the harness and cross-platform identity measured (not assumed); (b) permissive license verified and recorded; (c) the dense path retained as a verification oracle for sparse-vs-dense agreement on shared fixtures (the harness's residual/delta instrumentation already fits, §2.4).
4. **Phase D lead-up:** rule D-03b — Option B (library; candidates per §4 ASSUMPTION, audited against bar 3) vs Option C (in-repo skyline) — using the measured records from step 1. ASSUMPTION-level expectation, for planning only: if a candidate library passes the determinism bar, B is the cost-effective choice; if none does, C is the lineage-proven fallback. This packet does not pre-select.
5. **Scope honesty:** until D-03b lands, PRD §20's "thousands of nodes using sparse matrix methods" is explicitly *open*, not silently assumed met; the register row and `SparseSolverTbd` say so.

Rationale: every concrete number this decision needs (bound, trigger, candidate performance, cross-platform variance) is currently unmeasured (§2.3 TBD, §2.4), and the project's own deliverable guidance forbids inventing them (DEL-04-05 `Guidance.md:11`). The only thing rulable *today* on evidence is the strategy shape and the bars. Option A alone leaves the PRD sparse target unowned; B or C chosen today would be chosen on assumptions this repo cannot yet substantiate.

This recommendation is a `PROPOSAL` only. It confers no authority and changes no state.

---

## 6. Downstream impact map

| Surface | Impact of this ruling |
|---|---|
| **Phase D row D7** | Gets its concrete shape: harness time/memory extension + measured bound + named diagnostic now; sparse implementation (B vs C) at D-03b (`plans/PLAN_2026-06-10_prd_completion.md` §3 Phase D D7). |
| **Phase D row D6 (nonlinear solve)** | Iteration-loop design learns whether it wraps a dense or sparse factorization, and whether factor-reuse is required (§3 Q6); D-03 timing exists precisely because D6 multiplies solve cost (§2 plan row). |
| **`core/solver/frame_kernel`** | Dense seams (`assemble_global_stiffness`/`reduce_system`/`solve_dense`, §2.1) stay; a sparse successor lands behind the same boundary; zero-dependency posture changes only if D-03b selects Option B. |
| **`core/solver/diagnostics`** | New bound-exceeded code joins `SolverDiagnosticCode`; `SparseSolverTbd` wording/severity revisited when D-03b lands (§2.2). |
| **`core/solver/performance_harness`** | Gains time/memory fields in `HarnessRunRecord` and stepped-scale invented fixtures; becomes the evidence source for the bound, the D-03b choice, and sparse-vs-dense agreement oracles (§2.4). |
| **DEC-012 / PKG-04 TBD registers** | The "solver numerical library" reservation in DEC-012 (§2.2) is partially resolved (strategy + bars) with the library identity deferred to D-03b; DEL-04-01/-05/-06 open questions update at their formal reviews. |
| **D-04 (tolerances/coverage)** | Sparse-vs-dense agreement tolerances and conditioning thresholds become D-04 inputs once measured; this ruling adds measurement surfaces but sets no numeric thresholds (`_REGISTER.md` row D-04). |
| **PRD §20 / §23.1 acceptance** | "Hundreds of nodes in seconds" becomes a measurable harness claim; "thousands of nodes / sparse methods" is owned by D-03b rather than silently lapsing (§2.3). |
| **Desktop app solve path** | `run_preview_mechanics` (§2.5) inherits the bound diagnostic through the existing diagnostics envelope; no GUI contract change until sparse lands; long-solve progress/cancel (PRD §20) becomes relevant at measured scale. |

---

## 7. Authority and ruling record

Only the **human project authority** rules on D-03. Agents prepared this packet and may not certify, approve, or adopt it.

Per existing decision practice, the accepted ruling is recorded as a `DEC` entry in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12 (Decision log, `:566`) — as done for D-01 (`DEC-018`), D-10 (`DEC-021`), and the operation-seam ruling (`DEC-020`, `:590`) — after which the dispatching persona updates `execution/_Coordination/_DECISIONS/_REGISTER.md` row D-03 from `NOT_PREPARED`/`AWAITING_RULING` to `RULED` with a pointer (`_REGISTER.md` header). If Option D is selected, the named follow-up D-03b should be appended to the register as a new row per the register's append rule. This packet does not edit the register and does not resolve the decision.
