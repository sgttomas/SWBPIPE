# RT-A3 P: the nonlinear loop's scale repair (plan and record)

**Status.** Working plan and record of the TASK implementing R-4 §3.6 ("P: the nonlinear loop's
scale") on `claude/rt-a3-p` from main `252bef969`. It is revisable until review and records no
acceptance. Governing scope: [`route-unification-r4.md`](route-unification-r4.md) §3.6, §3.10, §6.1
items 4 and 6 and §6.3 O-2. Out of scope: new nonlinear mechanics, and any change to the active-set
rules, states, admission, gates or the certificate.

Paths: NI = `core/solver/nonlinear_integration/src`, FK = `core/solver/frame_kernel/src`,
SD = `core/solver/sparse_direct/src`, PP = `core/product_physics/src`.

## 1. The loop today (main `252bef969`)

- **Entry.** `solve_active_set_frame_with_mode_and_springs` (NI `lib.rs`) applies the §8.30
  correction-2 guard (`dense_loop_memory_guard`, 88·n² + 16·f² bytes against
  ⌊0.9 × 12 GiB⌋, in both modes, before any n² allocation), then assembles the dense global K
  (`assemble_global_stiffness_with_connectors`, curved slots, springs) and the dense
  `AssemblyEvidence` (four n×n buffers while it is built, two kept).
- **Per iteration** (`solve_iteration_with_sliding_friction_evidence`): with k derived-normal sliding
  candidates, one base and k unit-load coupling solves, each hard-coded `DenseScrutiny`, then the
  final solve in the requested mode: k + 2 full solves, each its own factorization. Every solve is
  `solve_linearized_system_evidence`: a dense reduced copy (`reduce_system_with_prescribed_displacements_binary64`),
  FK's dense `prepare_structural_binary64` (`prepare_bound`, with its n×n `Expansion` audit), the
  factor by mode (dense Cholesky, or SD's skyline LDL of the dense prepared matrix), the gate
  (`finish_structural`), product equilibrium, the legacy DEC-050/053 observation (a further sparse
  factor, and in dense mode a dense LU) and reactions by a dense matvec.
- **Gap scrutiny** (`structural_adapter::scrutinize_gaps`) runs FK's exact boundary context on the
  dense system; that context refuses any system above 256 DOFs (`Budget`) before reading K.
- **Completion and confirmation.** The result's `ProductCompletion` clones the dense K. The single
  post-loop confirmation (`confirmation::confirm_converged_state`) rebuilds a dense `AssemblyEvidence`
  and runs `confirm_with_formation_check` on the dense K (dense prepare, factor by mode).
- **PP.** PP assembles sparsely per basis already; every n² buffer on the nonlinear path is inside NI.
  PP `lib.rs` applies NI's guard once per invocation (`NONLINEAR_DENSE_MEMORY_LIMIT_EXCEEDED`).

## 2. Design

The loop holds one of two representations, chosen by mode:

- **Dense scrutiny: unchanged.** The dense K, dense `AssemblyEvidence` and today's k + 2 full solves,
  each with its own dense factorization, bit for bit. Reusing a dense factor across right-hand sides
  would keep the factor and a second n_f² prepared matrix alive across the next preparation's
  `Expansion` audit, above the guard's priced 16 bytes per free entry, so dense scrutiny keeps one
  factorization per solve (the scrutiny mode's cost profile is the guard's, as priced).
- **Sparse interactive: on the pattern.** `assemble_sparse_stiffness` (frames, connectors, curved
  blocks, springs; values bit-identical to the dense entries, K6 D1) and `SparseAssemblyEvidence`
  (the same builder, allowances bit-identical, K1). Each solve:
  - partitions and folds the reduced right-hand side in binary64, in prescribed order, as the dense
    `_binary64` reduction does (new FK `reduce_sparse_system_with_prescribed_displacements_binary64`);
  - prepares on the pattern with the option-(c) binary64 binding (new FK
    `prepare_sparse_structural_binary64`; today `prepare_sparse_bound` refuses that binding, "the
    nonlinear loop stays on the dense `_binary64` path until F1b"), and orders, factors and finishes
    on the pattern (SD `factor_sparse_structural_ldlt`, FK `finish_sparse_structural`);
  - evaluates product equilibrium over the pattern (new FK `evaluate_sparse_original_residual`, the
    binary64 binding of the shared `evaluate_original_residual_bound`), reactions by
    `SparseStiffness::multiply` minus f (bit-identical to the dense matvec), and the legacy
    observation from the pattern's free entries.
- **The per-iteration step stays one unit** (manager's steer, for RT-A5's held `minuend_final`):
  `solve_iteration_with_sliding_friction_evidence(system, input, boundary, states, previous, …, mode)`
  takes the loop's assembled system and a given support-state assignment, and in sparse mode
  factors once and solves the base, the unit-load coupling and the final right-hand sides on that
  factor. It is crate-internal; RT-A5 needs a public wrapper over it (§7). The confirmation stays a
  separate post-loop unit (kd5 §1.5: never called from an iteration).
- **Gap scrutiny in sparse mode** passes the dense view only when n ≤ 256 (the exact context's own
  limit) and an empty one otherwise; the context refuses n > 256 with `Budget` before reading K, so
  the evidence is the dense path's bit for bit (regression in §4).
- **Completion** holds the representation the loop used. **Confirmation** in sparse mode builds
  `SparseAssemblyEvidence` and confirms on the pattern (new FK `finish_sparse_confirmation` and
  `SparseConfirmationAttempt::evaluate_at`, the same `formation_check::check_at` with pattern
  accessors).

## 3. Numerical meaning

- **K6/K1 bit identity.** K6 pins the staged kernel sequence to the adapter's entries, bit for bit in
  `Debug` (`performance_harness/tests/k6_staged.rs`), and the pattern assembly to the dense assembly
  (`k6_parity.rs` D1). Underneath, FK `sparse/tests.rs` pins the sparse prepared system to the dense
  one entry for entry (`assert_prepared_equal`) and SD `structural/k1_tests.rs` pins the pattern
  path's order, profile and report to the dense-derived sparse path. P adds the binary64 binding to
  both regressions: the sparse binary64 preparation must equal `prepare_structural_binary64` entry
  for entry (rhs bits, exponents, matrix, audit, projection), including signed-zero and subnormal
  rows and rows coupled to nonzero prescribed values, and the pattern path's binary64 solve must equal
  the dense-derived sparse binary64 solve in `Debug`.
- **Consequence.** A bit-identical prepared matrix gives the same RCM order, skyline and factor, so
  sparse mode's final solve is today's bit for bit, and so is the confirmation.
- **Dense mode** runs today's code; every committed nonlinear output must be byte-identical in dense
  mode (§4).
- **The declared move (step 4).** Sparse mode's coupling solves move from dense Cholesky to the
  requested mode's skyline LDL. Only φ⁰ (and through it the final iterate) can change, only on cases
  with a derived-normal sliding support, and only in sparse mode. Each coupling right-hand side still
  passes its own M03 gate (the 64·γ(n) residual certificate after at most three refinements), so K-1's
  enclosure basis is unchanged; the published friction row is Stage A/B's certified value, whose ball
  contains φ* independent of φ⁰'s bits. The check (§4) compares the old and new φ⁰ with each other and
  with the certified ball, and the published rows of every committed coupled case.
- **Factor reuse (step 3)** must be bit-identical to separate solves: the shared-factor completion
  (new FK `finish_sparse_structural_on`) validates the right-hand side's prepared system and requires
  its prepared matrix to equal the factor's bit for bit; a factor is a deterministic function of that
  matrix, so each right-hand side's solution and report are the separate solve's.
- **K-1.** The loop rules (seeds, deferral, slip direction, normal branch, classification) are not
  touched. The solve path is: coupling solves move mode and share a factor. K-1's trace is re-checked
  against the new path, the note and script are revised where they name the solve path, and the
  script and the K-1 corpus are re-run in both modes.

## 4. Steps, commits and evidence

| Step | Change | Evidence |
|---|---|---|
| 0 | This plan | `deliverables check` |
| 1 | The named scale guard: already on main as §8.30 correction 2 (88·n² + 16·f² ≤ 104·n², both modes, before any n² allocation, `NONLINEAR_DENSE_MEMORY_LIMIT_EXCEEDED`). No formula change. | New regressions: sparse-mode refusal by name at the threshold (estimate = limit + 1) and admission at the limit and below, at the loop entry and in PP |
| 2 | The loop on the pattern in sparse mode (FK binary64 sparse prepare and reduction, sparse residual entry; SA `solve_binary64` on the pattern; NI representation switch; gap view; completion). Coupling solves stay `DenseScrutiny` (SA's dense materialization) so no output moves. | FK/SD binary64 parity regressions (K6/K1); NI parity: the pattern loop equals the dense-derived loop in sparse mode, `Debug` for `Debug`, on every NI nonlinear model; all committed outputs byte-identical in both modes |
| 4 | Coupling solves in the requested mode (declared φ⁰ move, sparse mode only) | Old/new φ⁰ per coupled case against Stage A's ball and K-1; published rows of every committed coupled case unchanged or bound-explained; dense mode byte-identical |
| 3 | Factor once per iteration in sparse mode (base, units and final on one factor) | `finish_sparse_structural_on` refuses another matrix; loop outputs bit-identical to step 4's in both modes |
| 5 | Confirmation on the pattern in sparse mode | Confirmation `Debug`-identical to the dense-derived confirmation on the kd5 corpus; committed outputs byte-identical |
| K-1 | Trace re-check, note and script revision | Script re-run; `rt_a3_nonlinear_k1_twins.rs` both modes |

Steps 3 and 4 are taken in the order 4, 3, so that the move (step 4) and the bit-identical
performance change (step 3) have separate evidence.

**Committed-output check.** Every committed nonlinear output is checked by its maintained test (PP
`kd5_nonlinear_tests.rs`, `w16_nonlinear_motion_tests.rs`, `rt_a3_nonlinear*` tests, the K-1 twins,
NI's suites and the product-preview fixture tests). In addition, a scratch diff builds
`openpipestress-runner` at main and at each step's head and solves every committed nonlinear request
(the kd5 requests, the W-16 nonlinear-motion requests, the PF Timoshenko requests and the
product-preview and results fixtures) in both modes, comparing output bytes.

## 5. Pricing (no profile write)

NI is on the retained static chain. The tool is run read-only on the step heads into an output
directory outside the checkout (`/Users/ryan/dev/chirality-t3/scratch/rt-a3/p/`), compared with
main's tree, as §8.49 did. Nothing is written to `retained_memory.rs`, any §8.x section,
`REGISTERED_PROFILES` or witness records; registration takes its D3 slot (after #105B, PF1b, I3a).
The result (priced sites touched, coefficient moves, frame growth) is recorded in §8 below.

## 6. Heavy run (planned, not run; needs Agent 0's approval)

A K6-style observation of one loop iteration at 1,000, 2,000 and 10,000 members (R-4 §5.4 M-3) in
§8 below, with its command and estimated resources. No exclusive or App window and no timing run is
taken by this TASK.

## 7. Open matters for Agent 0

- **The sparse-mode guard after P.** The guard prices the dense loop's buffers; after P, sparse mode
  holds none of them. It is kept unchanged in both modes, so admission does not change. Lifting or
  re-pricing it in sparse mode is O-2's decision, after the heavy run (§6).
- **RT-A5's held solve** calls the step through a public wrapper that this TASK does not add.

## 8. Record

### 8.1 Step 1: the named guard

The guard was already on main (§8.30 correction 2). It is unchanged. The regression
`the_guard_refuses_exactly_above_its_estimate_and_never_below_104_n_squared` (NI
`dense_memory_guard_tests.rs`) ties it to R-4 §3.6's 104·(6N)²: it refuses exactly when its own
estimate exceeds the limit, never admits less than 104·(6N)² does, and equals it on a fully free
model (1,759 nodes admitted, 1,760 refused). The existing regressions remain: refusal by name at the
estimate minus one and admission at the estimate (loop entry, both modes), and PP's
`nonlinear_dense_memory_guard.rs` refusal at the real limit with a peak heap increment below 64 MiB.

### 8.2 Step 2: the loop on the pattern

- **K6/K1.** FK `p_binary64_binding_is_byte_identical_for_a_fixed_order`,
  `p_binary64_rows_keep_the_dense_fold_sign_range_and_order` and
  `p_binary64_reduction_refuses_as_the_dense_reduction` (`sparse/tests.rs`), FK's option-(c) residual
  pin extended to the pattern entry (`s11k_tests.rs`), and SD's
  `k1_pattern_path_report_is_byte_identical_to_the_dense_derived_sparse_path` extended to the binary64
  binding. All pass.
- **NI.** A test-only shadow check in `solve_active_set_frame_with_mode_and_springs` re-runs every
  sparse-mode loop solve of NI's tests on the dense-derived sequence and asserts `Debug` equality (the
  completion's stiffness aside). `p_tests.rs` shows that it runs, and covers the strict-gap view's
  256-DOF boundary on both sides. A mutation (the pattern solve on the exact, non-binary64
  preparation) fails 18 NI tests, the shadow among them. The option-(c) source pins are extended to
  the pattern siblings: the siblings must reach only the binary64 preparations and contain no other
  exact entry.
- **Committed outputs.** A scratch driver over PP's public value entry solved 50 committed nonlinear
  requests and models in both modes (100 outputs, 80 of them solving with nonlinear rows): the kd5
  requests, the W-16 nonlinear-motion requests, the PF Timoshenko nonlinear requests, PP's preview
  fixtures and the demo models, each at the current model schema as their tests run them. Main
  `252bef969` and this branch with the old coupling are byte-identical on all 100. PP's lib tests
  (1,105) pass.

### 8.3 Step 4: the coupling move (sparse mode only)

- **Where it moves.** With the coupling in the requested mode, 7 of the 100 outputs move, all sparse:
  `preview_physics_invented`, `preview_physics_range_envelope`, the demo, kd5 F-2 and W-16 A2,
  A2-translated and A3. No dense output moves.
- **What moves.** In every moved output the status and the `numerical_quality` standing are
  unchanged, and no support state, support reaction, friction or displacement row moves. What moves:
  - the loop's binary64 evidence rows: iterate deltas (at most 1.3e-13 relative on F-2, 1.8e-11 on
    the demo's 0.0096 mm delta, 1.7e-13 mm absolute), free-row residuals and the work residual (one
    or a few binary64 quanta), and one work row's observed governing DOF (F-2, A3);
  - certified member rows of magnitude 1e-44 to 1e-33 (zero up to noise);
  - the integrity diagnostics' text, which embeds the final iterate's report.
- **Bound check (independent of the solver's bits).** Each moved certified member row was compared
  against its own certified radius from a scratch build that prints every straight-member certified
  row: |new − old| ≤ r_old + r_new for every moved row, the worst at 0.164 of the sum (the demo and
  `preview_physics_invented`, 39 of 672 rows moved), 1.7e-7 on F-2 and 6.5e-7 on A3.
- **References.** kd5's F-2 (the strongly coupled case, c·g = −0.748) stays within its bound of the
  exact coupled force in both modes (PP `kd5_nonlinear_tests.rs`), and the K-1 twin corpus passes in
  both modes. NI `p_the_coupling_move_stays_inside_the_certified_friction_bound` runs an inclined
  coupled cantilever both ways. On that small model the two couplings give the same φ⁰ bits, so it is
  a regression of the mechanism, not evidence of a bounded move; the corpus above is that evidence.
- **Regenerated with their maintained generators:**
  - `fixtures/product_preview/invented_demo_result_preview_physics_1_sparse.json` and the two
    generation records (`generate_product_preview_mechanics.mjs`). The records also had stale source
    hashes from earlier merges;
  - `fixtures/results/preview_physics_invented_sparse.json` and
    `preview_physics_range_envelope_sparse.json` (`preview_physics_capture`).

  The carrier cases' pinned hash of `preview_physics_invented_sparse.json` follows. The dense and
  refused-demo envelopes are byte-identical. The native smoke's demo expectations (837 rows,
  `checks_passed` in both modes) are unchanged.

### 8.4 Step 3: one factor per iterate (sparse mode)

- FK keeps a factor for further right-hand sides (`SparsePositiveFactor::share` and
  `SharedSparseFactor`, which hold a copy of the prepared matrix it factors) and completes another
  right-hand side on it (`finish_sparse_structural_on`). That completion validates the prepared
  system as the factor entry does, refuses a prepared matrix that is not the factor's bit for bit,
  and then runs the same completion. FK `p_a_shared_factor_completes_other_right_hand_sides_as_separate_solves`
  shows equality with separate solves on several right-hand sides, and the refusal.
- In NI's step, the first sparse-mode solve of an iterate factors and keeps the factor; the coupling
  and final solves complete on it (`SparseAssemblyEvidence::solve_binary64_shared`).
  `p_sparse_mode_factors_once_per_iterate` counts one factorization per iterate and two completions
  (unit and final) on each coupled iterate; dense scrutiny makes none. The shadow check compares
  every sparse-mode NI solve with the dense-derived separate solves.
- The 100 corpus outputs are byte-identical to step 4's.

### 8.5 Step 5: the confirmation on the pattern (sparse mode)

- FK `finish_sparse_confirmation` and `SparseConfirmationAttempt::evaluate_at` are the pattern
  siblings of `finish_confirmation` and `ConfirmationAttempt::evaluate_at`: the same
  `formation_check::check_at`, reading K and the prepared matrix through the pattern.
- The NI adapter's `SparseAssemblyEvidence::confirm_with_formation_check` is the pattern sibling of
  the single confirmation entry. `confirm_with_applied` calls it for a pattern completion in sparse
  mode, and the dense entry otherwise (a pattern completion confirmed in dense scrutiny lends its
  dense view). The confirmation is still one attempt after convergence.
- The R2, option-(c), formation-entry and K5 W4 source pins are extended to "one entry per adapter
  impl, called once per representation in `confirm_with_applied`". The pattern plumbing is added to
  the plumbing the product and the loop must not name, and the mutants still fail.
- Evidence: `p_the_pattern_confirmation_equals_the_dense_derived_one` (six cases, five of them
  Established, `Debug`-equal to the dense-derived confirmation), and the 100 corpus outputs are
  byte-identical to step 3's.
