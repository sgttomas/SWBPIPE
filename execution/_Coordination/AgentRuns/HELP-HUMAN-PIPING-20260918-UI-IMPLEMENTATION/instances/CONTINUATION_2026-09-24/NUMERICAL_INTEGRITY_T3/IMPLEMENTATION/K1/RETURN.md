# I8R return: slice K1 (the W3 kernel sparse representation and the sparse M03 gate)

**Status: complete for the pre-K2a tree, and green.** The candidate is `19925122b` on `codex/piping-k1-20260928` (base main `134eefc24`). These records are added on top. I8R made no Git writes; ROOT verified and committed each checkpoint.

- **Suites:** all 39 manifests of CI's cargo profile. There are no new failures; the only failures are the three Mac platform tests of the Mac main baseline, failing identically.
- **T9 (Mac-only):** 112 of 112 outputs byte-identical, base against candidate.
- **Parity:** K is bitwise equal, the order and profile are equal, and the reports are byte-identical in `Debug` to today's dense-derived sparse path.
- **Mutations:** 23 run, with the NONE control first. Every mutant is killed at a behavioural or pin assertion except K1-LABEL, which is killed only by a production invariant (§10). Every original pin mutant keeps its original kill set.
- **Stop rules:** none triggered.

**Pending:** the K2a interaction tests and the re-run on the combined tree, after K2a merges (§13).

Records use `<wt>`, `<scratch>`, `<VENV>` and `<home>` placeholders; there are no machine paths.

## 1. Brief, basis, delegation and rulings

- **Briefs:**
  - `TASK_BRIEFS/I8R_K1_RESUME.md`. It overrides `_COMMON.md`'s "Host resources" with the Mac's rules: no swap; `-j 8` at most; `RUST_TEST_THREADS=4`; at most two cargo jobs at once; no dense matrix at 10,000 members or more; mutants at most three at once, at `-j 4`.
  - `TASK_BRIEFS/I8_K1_IMPLEMENTATION.md` with addenda 1–4, which remains the brief.
  - `_COMMON.md` and `HANDOFF_2026-09-28_TO_LOCAL.md` §§4–5.
  - `IMPLEMENTATION/K1/WIP_STATE.md`: I8's handoff state. It is historical; this RETURN supersedes it.
- **Delegation:**
  - I8R is a Type 2 TASK dispatched directly by ROOT (HELP_HUMAN). It ran as a background subagent of ROOT's session on the owner's Mac, started with `I8R_K1_RESUME.md`.
  - The return path was ROOT; on the Mac there is no separate T3 manager.
  - I8R did not delegate.
  - I8 (the cloud session) drafted every file to the WIP commit `d08b0efc7`, and ran no cargo. I8R compiled, finished and verified the drafts.
  - The work ran through checkpoints A–D, each ended with a status message to ROOT. ROOT verified each checkpoint and committed `9d4ba0e17` (checkpoint A's test fixes) and `19925122b` (the KERNEL-list hunk, on its own).
- **Design:** D1 `DESIGN.md` revision 5a.2 (`fb62ef4a…`): §4.8 W3; the K1, K2b, K5, K6 and F1 rows of §6; §7.1; §7.3 mutations 8 and 10.
- **Rulings:** `ROOT_RULINGS_V1.md`, "K1: spawn timing and no both-entry gate", "K1: the S11 site table for sparse.rs and formation_check.rs", "K1: extending the K-D5 and option-(c) source pins for the sparse siblings", and "K1: the S11-F site test's KERNEL list, and the handoff". On the Mac, ROOT also ruled:
  - **(B)** K1 passes the suites if exactly the baseline's three platform tests fail, identically. T9 is Mac base against Mac candidate, from `git archive` copies.
  - **(C)** K1-LABEL and K1-LABEL-ORDER are both recorded as in §10. On mutation 10 and relabelling, the finding is stated as found (§10). K1-PIN-LOOP and K1-PIN-LOOP-B are both recorded.
  - **(D)** The contents of these records. `kernel_list_s11f.patch` stays unedited, with a note beside it.
- **Platform:**
  - `aarch64-apple-darwin`, rustc 1.97.1 and cargo 1.97.1 (`_run_records/toolchain.txt`); stable rustfmt for formatting only.
  - **T9 here is a Mac-only comparison.** The Mac's platform libm differs from the Linux records on 12 of 112 committed outputs (`T3/PLATFORM_CALIBRATION_MAC/` on the numerics branch, `RECORD.md` and `t9/`). No Mac output is compared with a Linux record. CI on Linux stays the authority for the suites.
- **Read:**
  - Root `AGENTS.md` and `agents/AGENT_TASK.md`;
  - the briefs and rulings above;
  - `PLATFORM_CALIBRATION_MAC/RECORD.md` and `suites/SUMMARY.md`;
  - every WIP draft in full;
  - the dense code each sparse stage mirrors: `validate`, `prepare_bound`, `contribution_sums` and `audit_contributions`, `audit_intended_action`, `audit_load_fidelity` and `audit_load_row`, `evaluate_original_residual_bound`, `estimate_rcond`, `finish_checked_factor`, `factor_structural_profile`, `negative_pair_witness` and `verify_negative_direction`;
  - FK's `assemble_global_stiffness_with_user_elements` and the `reduce_*` functions;
  - `SymmetricProfileMatrix`, `adjacency_from_*` and `reverse_cuthill_mckee`;
  - the base `AssemblyEvidence::new`;
  - `formation_check.rs` (`check` and `evaluate`);
  - the S11-K and K-D5 mutation records.

## 2. Files and line counts

Against `134eefc24`; lines and sha256 (first 16 hex digits) at `19925122b`.

| File | +/− | Lines | sha256 |
|---|---|---|---|
| `P/core/solver/frame_kernel/src/structural/sparse.rs` (new) | +1830 | 1830 | `1b7fe1f6829e8a65` |
| `P/core/solver/frame_kernel/src/structural/sparse/tests.rs` (new) | +1485 | 1485 | `e1b93a639c6751f9` |
| `P/core/solver/frame_kernel/src/structural.rs` | +417 −165 | 2532 | `6644b82a7788697e` |
| `P/core/solver/sparse_direct/src/structural.rs` | +99 −2 | 226 | `44ed44fa41c0a427` |
| `P/core/solver/sparse_direct/src/structural/k1_tests.rs` (new) | +811 | 811 | `f1867895b06673ff` |
| `P/core/solver/nonlinear_integration/src/structural_adapter.rs` | +784 −282 | 2286 | `3c95b34964532cc3` |
| `P/core/solver/nonlinear_integration/src/structural_adapter/k1_tests.rs` (new) | +1020 | 1020 | `e306f1aeb89c66d6` |
| `P/core/solver/nonlinear_integration/src/s11k_tests.rs` | +86 −20 | 1497 | `2c38b673945b308f` |
| `P/core/solver/frame_kernel/tests/s11_site_table.rs` | +28 | 566 | `8fe80d02f1607569` |
| `P/core/product_physics/tests/s11f_site_test.rs` | +62 | 1540 | `37930bbad65b8aa9` |

`FK/lib.rs`, `formation_check.rs`, `exact_boundary.rs`, all of `PP`'s source, `source_recovery.rs`, `nonlinear_integration/src/lib.rs`, `curved_bend` and `diagnostics` are untouched. There are no Cargo, lockfile, fixture or schema changes.

## 3. Write-set items

### 3.1 `FK/structural/sparse.rs` (new)

- **`SparsePattern`** stores compressed rows with ascending columns and the index of each entry's transpose.
  - `from_positions` takes the symmetric closure, deduplicated. It is a pure function of the set of positions.
  - `from_connectivity(node_count, &[(node_i, node_j)], diagonal DOFs)`: each element couples its two nodes' 12 DOFs, and each spring adds a diagonal. The rows are sorted, so the pattern depends on neither labels nor element order (mutation 10).
  - Storage counts: `entry_count` and `lower_entry_count`.
- **`SparseStiffness`:**
  - `from_pattern_and_contributions` sums each entry from +0.0 in list order, which is the dense `k[r][c] += v` sequence.
  - `from_contributions`; `from_dense`, which keeps a −0.0, so the values are the dense values bit for bit.
  - `get`, `row`, `to_dense` (the dense view, n²), `storage_counts`.
  - `multiply` is bit-identical to the product's `multiply_matrix_vector`, including the sign of a zero row sum. An absent entry contributes a signed zero, so one +0.0 and one −0.0 stand for all of them.
  - `reactions` is E12 from sparse rows: the formed K·u and the DOF's ledger terms in one exact sum.
- **`assemble_sparse_stiffness(node_count, frames, users, blocks, springs, &SparseAssemblyOptions)`** is the sparse sibling of `assemble_global_stiffness_with_user_elements`, followed by the product's curved and spring additions.
  - It forms each element with the same `global_stiffness()` call, in the same order and after the same node checks, so a formation refusal is the same error for the same first element, before any value is accumulated.
  - It accumulates frames, users, blocks and springs in the dense order.
  - After frames and users, it refuses the first non-finite entry in row-major order as `NonFiniteInput { "assembled stiffness" }`, as the dense assembly does.
  - Blocks (realized curved bends as `StiffnessBlock`) and springs are then added unchecked, as the product adds them.
  - `SparseAssemblyOptions` is `#[non_exhaustive]` with `new()`, for K2b's b.
- **`reduce_assembled_sparse_system`** is the sibling of `reduce_assembled_system` (`None`: restrained at zero) and `…_with_prescribed_displacements` (`Some`).
  - Its errors come in the same order.
  - KS2 is one exact sum per free row, over the row's stored couplings to boundary DOFs.
  - It returns `SparseReducedSystem { free_dofs, prescribed, force: ReducedForce }` with `free_position(dof)`.
  - The reduced stiffness is not materialized.
- **`SparseStructuralSystem`** (`new`, `assembled`, `with_formation_source`) and `SparseSymmetryEvidence`, which holds per-entry allowances and counts.
- **The gate over the pattern:**
  - `validate_sparse`: the same checks in the same order. The symmetry audit visits stored pairs below the diagonal, row-major, which is the dense order of every pair that can differ.
  - `prepare_sparse_bound`:
    - scale exponents;
    - the scaled free block over free positions;
    - KS1 through the shared `exact_scaled_rhs`, with the row's stored couplings in `prescribed` order;
    - `legacy_zero_product_fold`: the dense fold `b − K·g` on a row with no nonzero prescribed product, reduced to its subnormal refusal and its zero-sign rule, with no accumulation;
    - the symmetry projection with allowances per source entry;
    - `sparse_audit_contributions`: expansions per pattern entry, O(nnz), with every term read in the dense order.
  - `ConditionMatrix` for the prepared system; the norm comes from the rows, the matrix being exactly symmetric.
  - `factor_sparse_structural_profile`: skyline rows filled from the stored entries, then the shared `ProfileFactor::factor_structural_profile`.
  - `finish_sparse_structural`: the shared `finish_checked_factor`, labelled "positive skyline LDL".
  - `audit_sparse_load_fidelity`: rows through the shared `audit_load_row` and `unaudited_row`.
  - `verify_sparse_negative_direction` and `sparse_negative_pair_witness`: stored pairs only, in the dense order, O(nnz). `_counted` returns the visited-pair count.
  - An unstored pair has zero coupling, so its energy is a_ii + a_jj > 0 and it can never be the dense search's witness. The witness found is therefore the dense search's first witness.

### 3.2 `FK/structural.rs`

- `mod sparse;` and `pub use sparse::{…}`.
- **Two private traits:** `Represented` (the source equations: view, validate, row, prescribed coupling, contribution sums and rows, load audit, formation check) and `PreparedGate` (what the completion reads from a prepared system), plus the small `Equations` view.
- **Written once over them, keeping their names:** `evaluate_original_residual_bound`, `audit_intended_action` and `finish_checked_factor` (with K-D5's EF step).
  - `audit_load_row` and `unaudited_row` take the `Equations` view and the precomputed (K_ij, u_j) parts.
  - `exact_scaled_rhs` takes the row's couplings.
  - `estimate_rcond`'s body is the provided method of a private `ConditionMatrix` trait.
  - The skyline LDL is `ProfileFactor::factor_structural_profile`, named after the public entry whose site-table row it carries.
- **The dense public API is unchanged in signature.** Every dense stage reads the same terms in the same order, so every dense call is byte-identical (T9 and the unchanged suites, §7–§8).

### 3.3 `sparse_direct/src/structural.rs`

- `order_sparse_structural(&SparsePreparedSystem) -> SparseStructuralOrdering { order, first_columns, profile_entry_count, max_half_bandwidth }`: `adjacency_from_symmetric_entries` on the prepared lower entries, then RCM, then `SymmetricProfileMatrix::from_entries_with_order`.
- `factor_sparse_structural_ldlt` and `solve_sparse_prepared` (factor, or the witness on a refused factor, then the completion: `solve_structural_sparse`'s sequence).
- `solve_sparse_structural`, `solve_assembled_sparse_structural` and `solve_formation_checked_sparse_structural`.
- `factor_structural_ldlt` and `solve_structural_sparse` are unchanged.
- The file still names no "force", which `sparse_direct_factor_inherits_the_prepared_ledger_binding` requires. I8's C3-detect helper was its own draft, removed before any commit (`WIP_STATE.md` §2.4); nothing pre-existing was removed.

### 3.4 `SA` (`nonlinear_integration/src/structural_adapter.rs`)

- **`AssemblyEvidence::new`** builds through the shared `EvidenceParts::new` with a `DenseEvidenceStore` (n×n: today's arithmetic in today's order). `AssemblyEvidence`'s layout and pub fields are unchanged.
  - `geometry`, `symmetry_basis`, `qualified_passive_family` and `formation_source` delegate to shared free functions and to `BodyEvidence`, with unchanged bodies.
- **`SparseAssemblyEvidence::new(pattern, …)`** uses the same builder with a `SparseEvidenceStore`: one allowance and one count per pattern entry.
  - It has `with_force_terms`, `qualified_passive_family`, `geometry`, `pattern`, `contributions`, `absolute_roundoff`, `operation_counts`, `storage_counts` and `dense_symmetry_view`.
  - A stiffness on another pattern is refused (`check_pattern`).
- **The pattern-taking entries:** `solve_assembled`, `solve_assembled_with_formation_check` (the same selection rule and formation source) and the crate-private `solve` (C3-detect when force terms are set).
  - **`SparseInteractive`** runs the gate over the pattern.
  - **`DenseScrutiny`** materializes the dense view (values, allowances, counts) and runs today's dense Cholesky, bit for bit.
- `solve`, `solve_assembled`, `solve_assembled_with_formation_check`, `solve_binary64` and `solve_structural_sparse_binary64` are unchanged.

### 3.5 Declared extensions

Each is a separate item, and the reviewer covers all of them.
1. **The S11 site table, sparse.rs** (`frame_kernel/tests/s11_site_table.rs` hunks 1 and 3; ROOT's option (a) under its five conditions):
   - the Source, plus 12 rows (9 with counts and 3 zero rows: `reactions`, `reduce_assembled_sparse_system` and `legacy_zero_product_fold`);
   - no existing row, count or disposition changes;
   - the stiffness-side sites are exemptions with reasons: coalescing, scatter, the spring diagonal, `multiply` (K·u, as PP's `multiply_matrix_vector`), and `sparse_audit_contributions` (as `audit_contributions`);
   - every load, force or RHS sum is an `ExactAccumulator` sum (count 0);
   - `K1-SITE-SPARSE` is killed.
   - In `sparse_audit_contributions`, the |rhs| magnitude norm has `audit_contributions`' disposition: a norm of the perturbation estimate, not a load sum.
2. **The S11 site table, `formation_check.rs`** (hunks 2 and 4; ROOT: a separate commit on K1's PR):
   - one row, `pow2`, 1, "integer: exponent step";
   - ρ is one `ExactAccumulator` sum per free row, so there is no plain binary64 load, force or RHS fold and no K-D5 finding;
   - `K1-SITE-FC` is killed.
   - The two hunks are `wip/site_table_formation_check_hunks.patch`, which reverse-applies cleanly on `19925122b`.
3. **The pin extension** (`nonlinear_integration/src/s11k_tests.rs`, additive, as ROOT approved in the manager's tightened form):
   - `token_indices`: tokens that begin with a letter match on an identifier boundary; `_`, `.` and `:` tokens match anywhere, as before;
   - `body_in_impl` and `ADAPTER_IMPLS`: blanking and allowance are scoped to `impl AssemblyEvidence` and `impl SparseAssemblyEvidence`;
   - exactly two definitions of `solve_assembled_with_formation_check` in SA, one per impl; a third anywhere fails;
   - the pattern path's exact entries are in `EXACT_ENTRY_POINTS`, and its formation plumbing is in `FORMATION_ENTRY_POINTS`;
   - `solve_binary64` and `solve_structural_sparse_binary64` are checked as before;
   - the PP side is unchanged: exactly one product call, inside `solve_preview_reduced_system`;
   - ROOT's behavioural pin (3) is `k1_nonlinear_loop_reaches_neither_formation_entry`.
   - ROOT's three required mutants are killed, and no original mutant is weakened (§10).
4. **sparse.rs in PP's `s11f_site_test.rs` KERNEL list** (ROOT), a write-set extension into PP tests only, committed on its own (`19925122b`):
   - the Source is appended, so `KERNEL[4]` and `KERNEL[6]` are unchanged;
   - 11 `FORCE_FUNCTIONS` rows are added;
   - `K1-S11F-KERNEL` is killed at rule 6.
   - `kernel_list_s11f.patch` is unedited. `kernel_list_s11f.patch.NOTE.md` records the one formatting difference of the applied hunk.

### 3.6 The conditional files

Not used. `formation_check.rs` and `exact_boundary.rs` are unedited.
- The sparse `Represented::formation_check` passes K-D5's `check` a `StructuralSystem` view whose stiffness is empty. `check` and `evaluate` read only `force` and `free_dofs`, and the attempt's own factor through `solve` (`formation_check.rs:193-194, 223`).
- S11-K's exact boundary is not reached from the pattern path.
- The parity of the `FormationCheck` records and the Passed→Sensitive outcome is tested (§9).

### 3.7 The explicit-zero rule

An entry whose coalesced value is exactly 0 stays in the pattern, and the contribution audit, the residual and the witness see it.
- It takes no part in adjacency, ordering or profile, exactly like a zero of today's dense matrix.
- The rule is applied **by value, on the prepared (scaled, symmetrized) matrix that the factor orders.** So the order and the profile equal today's dense-derived ones.
- Tests:
  - `k1_adjacency_order_and_profile_equal_the_dense_derived_path`: the axis-aligned tree models carry explicit zeros;
  - `k1_explicit_zero_by_cancellation_takes_no_part_in_the_order`: a structural rule, treating zeros as edges, would change the RCM order on its case, and the value rule keeps today's.
- A −0.0 in a caller's dense matrix is stored by `from_dense`. Assembled values never hold −0.0, because every entry starts at +0.0.

### 3.8 Behaviour of the new entries on invalid input

These differ from dense only on the sparse side, and only for invalid input. No existing entry changes.
- A contribution outside the pattern gives `InvalidInput("contribution outside pattern")`.
- A spring or block node outside the model gives `InvalidNodeIndex`, where the dense `+=` would panic.
- SA refuses a stiffness on a different pattern.
- A Binary64 binding on the sparse prepare gives `InvalidInput`: the option-(c) fold is not offered on the pattern, and the loop stays on the dense `_binary64` path until F1b and T5.

## 4. Callers (`_run_records/callers.txt`)

- **Scan:** I8's lexer scan (`scan_callers_k1.py.txt`), run on a `git archive` of `19925122b`. It finds 278 call sites: 74 non-test and 204 test.
- **Product callers are unchanged:**
  - PP `solve_preview_reduced_system`: `AssemblyEvidence::new` and `.solve_assembled_with_formation_check(`, the dense entry;
  - `source_recovery.rs` `prepare_sources`: `AssemblyEvidence::new`;
  - the nonlinear loop: `AssemblyEvidence::new`, `.solve_binary64(`, `solve_structural_dense_binary64(` and `solve_structural_sparse_binary64(`.
- **No non-test caller of any new entry** exists outside its defining crate's own chain: FK sparse.rs → FK structural.rs stages; sparse_direct's pattern solves → FK; SA's `SparseAssemblyEvidence` entries → sparse_direct and FK.
- **The existing named entries** (`solve_structural_dense*`, `solve_structural_sparse*`, `factor_structural_ldlt`, `negative_pair_witness`, `verify_negative_direction`) keep their callers and signatures.
  - The generic stages are called from the dense entries exactly as before: `finish_structural` → `finish_checked_factor`; `evaluate_original_residual*` → `evaluate_original_residual_bound`.
  - T9 and the unchanged suites show no existing caller changes behaviour.

## 5. Tests added (28; all pass)

- **frame_kernel, `structural::sparse::tests` (14):**
  - `k1_pattern_is_a_function_of_the_positions_only`
  - `k1_coalesced_values_are_bit_identical_to_the_dense_assembly` (two modulus bases)
  - `k1_assembly_refusals_match_the_dense_assembly`
  - `k1_gate_stages_are_byte_identical_for_a_fixed_order` (legacy, typed, C3-detect, bare)
  - `k1_s11k_audit_and_ks1_ks3_have_identical_outcomes_in_both_representations`
  - `k1_legacy_zero_product_rows_keep_the_dense_zero_sign_and_range_refusal`
  - `k1_multiply_reactions_and_reduction_match_the_dense_forms`
  - `krev02_on_sparse_…`, `krev03_on_sparse_…`, `krev04_on_sparse_…`, `krev05_on_sparse_…`
  - `k1_negative_witness_matches_the_dense_search_and_verification`
  - `k1_negative_witness_visits_only_pattern_pairs`
  - `k1_matrix_references_have_the_same_outcome_class_in_both_representations`
- **sparse_direct, `structural::k1_tests` (5):**
  - `k1_adjacency_order_and_profile_equal_the_dense_derived_path`
  - `k1_explicit_zero_by_cancellation_takes_no_part_in_the_order`
  - `k1_pattern_path_report_is_byte_identical_to_the_dense_derived_sparse_path`
  - `k1_krev02_through_the_pattern_path`
  - `k1_storage_counts_of_the_rf_large_chain_and_tree`
- **nonlinear_integration, `structural_adapter::k1_tests` (9):**
  - `k1_sparse_evidence_is_the_dense_evidence_on_the_pattern`
  - `k1_pattern_entries_are_byte_identical_to_todays_entries_in_both_modes`
  - `k1_kd5_parity_cases_demote_identically_in_both_representations`
  - `k1_c3_detect_solve_is_identical_in_both_representations`
  - `k1_n_references_have_the_same_outcome_class_and_values_in_both_modes`
  - `k1_t0r_reactions_from_sparse_rows_match_the_references`
  - `k1_relabelled_model_gives_the_same_answers`
  - `krev01_on_sparse_geometry_witnesses_are_unchanged`
  - `k1_nonlinear_loop_reaches_neither_formation_entry`, ROOT's pin (3)

The inputs are invented, and every property is stated in the test files. The adapter tests include K-D5's generated `kd5_models.rs` read-only (13 models, with realized bends).

## 6. Checkpoint A: compile, targeted tests and the fixture changes (reviewer items)

- **Compile check** (`_run_records/checkpoint_a/check_*.log`): `cargo check --tests` on FK, sparse_direct and nonlinear_integration gives 0 errors and 0 warnings. The only error in the drafts was E0689 in a test (item a).
- **First run of the drafts** (`test_fk_k1_1.log`, `test_sd_k1_1.log`, `test_ni_k1_1.log`):
  - FK: 22 passed, 5 failed;
  - sparse_direct: 3 passed, 3 failed;
  - nonlinear_integration: 9 of 9 passed.
  - **Every failure was a test fixture or assertion.** In each failing case the two representations were never observed to differ: either both returned the identical result (identical preparation errors inside `solve_both`), or a dense-side assertion failed before any comparison.
- **No product code was changed** from the WIP.
- **Final:** FK `k1_`/`krev0` 27 passed; FK in full 154 + 3 + 6; sparse_direct `k1_`/`krev0` 6; nonlinear_integration `k1_`, `krev0`, `s11k_tests` and `kd5` 33; `s11_site_table` 3; PP `s11f_site_test` 11.

**Reviewer items: every change I8R made to I8's drafted tests** (in `9d4ba0e17` unless noted), each with its reason:
- **(a)** `sparse/tests.rs`: `vec![0.0_f64; …]` for `folded`. It was E0689, an ambiguous float type.
- **(b)** The FK and sparse_direct test models now carry **formation allowances in `AssemblyEvidence::new`'s form**:
  - each element's `transform_roundoff` bounds and counts (+1 per scatter);
  - one count per spring;
  - then `gamma(scatter count)·Σ|value|`;
  - per pattern entry for the sparse side: FK `Model::dense_symmetry`, `block_formation` and `Pair`; sparse_direct `Model::symmetry` and `Symmetry`.

  *Reason:* the skew chain's global K is symmetric only within formation roundoff. With no allowances, both representations refused with the identical `Asymmetric { row: 1, col: 0, relative_skew: 2.22e-16 }`, so the gate and report comparisons were vacuous. The product always supplies these allowances through SA. FK and sparse_direct cannot depend on SA, so the test replicates the form.
- **(c)** The FK gate test now asserts that the typed and C3-detect solves are `Ok` (before, it asserted nothing). A "bare" check was added, with no allowances: identical outcomes in both representations, and the chain refused as `Asymmetric` in both. sparse_direct's report test has the same bare check.

  *Reason:* the comparisons must not be vacuous, and the validate path without evidence must also be at parity.
- **(d)** The assembly-refusal fixture now uses E = 1e300 and A = 1e8, so each member's EA/L is 1e308 and node 1 sums to inf. The assertion message now shows the error.

  *Reason:* the drafted E = 1e308 overflowed at formation (12·E = inf, "computed local stiffness") and never reached assembly, so the "assembled stiffness" refusal the test names was not exercised.
- **(e)** Witness case 2 is now `[[4,0,5],[0,1,0],[5,0,4]]`, where it was `[[4,0,3],[0,1,0],[3,0,2]]`.

  *Reason:* the drafted matrix is indefinite, but its scaled pair energy for (2,0) is exactly 0, so the dense search finds no witness and the test's non-vacuity assertion failed. The new case has scaled pair energy −0.5, still across the unstored pairs (1,0) and (2,1).
- **(f)** The S11-K audit test's C3-detect part now uses an unsettled six-member chain and a cancelling ledger (G, 0.3, −G) with **G = 1e12**. The KS1–KS3 part uses the settled chain and asserts a nonzero prescribed value.

  *Reason:* at G = 1e8 the fold's loss (about 3e-9 N) is within the audit's target relative to the row's d_i on a six-member chain, so the row was not flagged. A settlement's large K_ij·u_j terms also enter d_i and hide the loss. At G = 1e12 the loss (about 2.4e-5 N) is well above the target.
- **(g) The O(n⁴) dense witness cross-check was reduced to a 6-member chain.** In `k1_negative_witness_visits_only_pattern_pairs`, the O(nnz) count assertion (visited = stored lower pairs, and fewer than a tenth of n(n−1)/2) stays on the 40-member chain.

  *Reason:* the dense search verifies each of the n(n−1)/2 pairs in O(n²). On the 40-member chain (246 free DOFs), that one test took 80.6 s in a debug build (`test_fk_k1_4.log`). This is a test-duration observation, not a performance claim. The dense-versus-sparse "no witness" agreement is now checked on the 6-member chain, and the witness parity on indefinite systems is `k1_negative_witness_matches_…`.
- **(h)** `class()` in the FK and SA tests prints the refusal variant's name instead of `Discriminant(n)`. This is readability only.
- **(i)** In sparse_direct's storage-count test, `profile ≥ free_lower` became `profile ≥ free_lower_nonzero`, and both counts are printed.

  *Reason:* explicit zeros from axis-aligned members can lie outside the skyline. On the AX chain at n = 10, free_lower is 534 and the profile 168. The skyline holds every nonzero entry.
- **(j)** The PP KERNEL-list hunk was applied from `kernel_list_s11f.patch`, with a formatting correction (`19925122b`; the note beside the patch).
- Stable rustfmt was run on the three edited test files only. `rustfmt --check` also reports pre-existing drift in `exact_boundary/functionals*.rs`, which is untouched.

## 7. Suites (per crate; `_run_records/suites/`)

- **Method:**
  - CI's numerical cargo profile: all 39 manifests discovered by `check_release_readiness.py`;
  - `cargo test --offline --locked --no-fail-fast`, one manifest at a time;
  - `CARGO_BUILD_JOBS=8` and `RUST_TEST_THREADS=4`;
  - on a `git archive` of `19925122b` without `execution/`.
- **Baseline:** ROOT's Mac main run, main `649162522`, whose piping tree equals `134eefc24`'s (`T3/PLATFORM_CALIBRATION_MAC/suites/SUMMARY.md`). Its no-fail-fast re-runs were used for PP and headless.
- **Per-test comparison** (`compare_suites.py.txt` → `suites_compare.txt` and `.json`): tests are keyed by manifest, target and name, with a doc test's line number dropped.
  - **0 tests changed result, and 0 were removed. 28 were added, all passing.**

| Manifest | Main (passed / failed / ignored) | K1 |
|---|---|---|
| core/solver/frame_kernel | 149 / 0 / 0 | **163** / 0 / 0 |
| core/solver/sparse_direct | 25 / 0 / 0 | **30** / 0 / 0 |
| core/solver/nonlinear_integration | 89 / 0 / 0 | **98** / 0 / 0 |
| core/product_physics (17 targets) | 522 / 1 / 1 | 522 / 1 / 1 |
| core/runner/headless (8 targets) | 82 / 2 / 0 | 82 / 2 / 0 |
| the other 34 manifests | equal, test for test | equal |

- **The three failures are exactly the baseline's Mac platform tests:**
  - PP `s11g_tests::t13_committed_fallback_uz_is_byte_identical`;
  - headless `load_reference_route_tests::load_reference_one_actual_solve_mints_bound_evidence_and_canonical_document_both_modes`;
  - headless `cli_load_reference_one_both_modes_is_controlled_and_equals_the_library_route`.

  They compare committed Linux bytes; macOS `hypot` is 1 ulp different. Their failure output, from the first `---- ` line to `failures:` with thread ids removed, is **byte-identical to main's**: 300 bytes for PP and 952,879 bytes for headless (`failure_blocks_sha256.txt`).
- In the headless log, lines over 600 characters are cut in these records (§16).

## 8. T9, the committed-fixture diff (Mac-only; `_run_records/t9/`)

- **Setup:**
  - Base: a `git archive` of `134eefc24`. Candidate: a `git archive` of `19925122b`. Both are without `execution/`, on this Mac.
  - The two trees differ only in the files of §2.
- **Harness:** S11-K's `fixdiff_main.rs` (sha256 `ec089c1d4a2a46dd…`, unchanged), `--release --offline --locked`, with the harness lock ROOT used; product_physics's lock is equal in both trees.
- **Inputs:** every committed JSON request or model under `P/core`, `P/fixtures` and `P/validation`, in both modes: 112 outputs (core 10, fixtures 72, validation 30).
- **Result:** **112 of 112 byte-identical**, base against candidate. The raw outputs are identical too.
  - Six outputs are `ERR` documents from three committed inputs (`domain/invented_physical_source_of_truth_model`, `model_operations/invented_accepted_model_state`, `mechanics/tp_phys_014_canonical_analytical_payload`), identical in both.
  - The stop rule did not trigger.
- **Cross-check:** the Mac base hashes equal ROOT's Mac main hashes (`sha_native.txt`, main `649162522`) on all 112.
- **This is a Mac-only comparison.** Never compare these hashes with the Linux records: 12 outputs differ by platform libm (`T3/PLATFORM_CALIBRATION_MAC/`).

## 9. Parity results

- **Bitwise K**, entry for entry:
  - FK: the chain, the settled chain and the branched tree (frames, a user element, an explicit block, springs with two on one DOF, prescribed motion, explicit zeros) at E = 200 GPa and 180 GPa. Each basis gives its own values on the same pattern.
  - SA: K-D5's 13 models, with realized curved bends, against the product's dense order (frames and users, then the curved additions, then the springs).
  - The allowances and counts are equal to the dense evidence, bit for bit.
- **Order and profile:** RCM order, skyline, profile count and half-bandwidth equal the dense-derived path's on every model, with explicit zeros present in the tree.
- **Report byte identity**, which F1b needs. The pattern path's `StructuralSolution` (report, displacements, load fidelity, formation record) is byte-identical in `Debug` to today's `factor_structural_ldlt` path:
  - for legacy, C3-detect, typed and formation-checked systems;
  - on 6 models, with contributions on and off, and with free DOFs ascending and reversed;
  - and through SA's entries in both modes, on the 13 K-D5 models, plain and formation-checked.
- **Outcome classes and values, in both representations and both modes:**
  - N01, N08 (4 loads), N09 (bending and torsion) and N03-RX: Passed, within 1e-9 of the references;
  - N05: Sensitive; N06: NumericallyUnresolved;
  - N02 (loaded, and unloaded = R07), N03-RZ and N04: Mechanism;
  - N07: NegativeEnergy;
  - R01, R04, R06 and NP-B (n = 8, 64, 80; c = 0.75 and 0.750000000001): Passed;
  - R02 and R05: refused by the contribution audit; R03: Sensitive by the load audit;
  - NP-D: skew → Asymmetric; nonfinite → InvalidInput; duplicate cancellation → the zero diagonal is refused.
  - T0R M05-T, M05-R1 (unrotated; permuted; 30° about z; translated) and M05-SPRING: the reactions from sparse rows are bit-equal to the dense E12 form, and within 1e-9 of the references.
  - Near a screen boundary no divergence was seen, so nothing had to be recorded.
- **S11-K:** the load audit flags the folded tip row identically in both representations, including the public audit functions row for row. KS1–KS3 (the typed settled chain) are identical.
- **K-D5:**
  - P1's 122 and CSKEW_8_5 demote with reason `Estimate`, in both representations and both modes.
  - E1 and E6 do not demote.
  - E1 with no macro element gives `formation_check_unavailable` (`curved_bend_source_unmatched:`).
  - Not-selected runs the plain entry: Passed.
  - The records are byte-identical.
- **The loop pin:** the nonlinear loop's first iteration is Passed and bit-equal to the named binary64 solve, while both formation-checked entries demote the same linear system.
- **Relabelling:** the numbering forward and reversed agree within 1e-9. Each numbering is byte-identical to the dense-derived path. sparse_direct's reversed free list is byte-identical too.
- **The O(nnz) witness:** on the 40-member chain it visits exactly the stored lower pairs of the free block, fewer than a tenth of the dense n(n−1)/2. This is a deterministic count.
- **KREV-01 to KREV-05 on sparse:** 01 through SA, both modes and origins; 02 through FK and the pattern path; 03–05 in FK.

## 10. Mutations (`_run_records/mutations/`)

- **Method:**
  - Each mutant got its own clean `git archive` of `19925122b` (without `execution/`, extracted with fresh mtimes) and its own target, under `<wt>/k1-mut/<id>/`. Both were deleted after each run.
  - The NONE control ran first, alone. The rest ran at most three at a time, each at `-j 4` with `RUST_TEST_THREADS=4`.
  - Tests per mutant, all `--no-fail-fast`: frame_kernel in full, sparse_direct in full, nonlinear_integration in full, and PP `--test s11f_site_test --test formation_check_runtime`.
  - The patches are `mutate_k1.py.txt`: I8's draft plus K1-S11F-KERNEL, K1-LABEL-ORDER, K1-PIN-BINARY64-B and K1-PIN-LOOP-B. Every anchor matches exactly once.
- **Control:** NONE exits 0 everywhere (163 + 30 + 98 + 11 + 5).
- **23 mutants: 0 compile-only.** Kill sites are a test and its assertion line (`kill_sites.txt`). Precondition and discrimination assertions are not counted as kills.

| Mutant | Change | Kill sites |
|---|---|---|
| K1-M8 (mutation 8) | the last spring omitted, sparse only | FK `k1_coalesced_values_…` (sparse/tests.rs:377); FK gate (:576); FK multiply (:875); SD report (k1_tests.rs:566); NI `k1_sparse_evidence_…` (k1_tests.rs:261). Entries on the dense view of the same values are refused by the contribution audit (the R05 analogue). |
| K1-ORDER | frames and users accumulated in reverse | FK `k1_coalesced_values_…` (:377; the tree, with 3 contributions at nodes 1 and 2); FK :576, :875; SD :566. It was killed, so no precondition case was added. |
| K1-KFC | K_fc·u_c dropped from the sparse prepare | FK `assert_prepared_equal` rhs (:568), through `k1_gate_stages` (settled chain), `k1_matrix_references` (R04), `k1_s11k_audit` (KS1–KS3) and `krev02_on_sparse`; SD `k1_krev02_…` (:690); SD :566 |
| **K1-LABEL** (I8's draft) | neighbour lists left unsorted | **Killed only by a production invariant: a fail-closed panic at `SparsePattern` construction, sparse.rs:153** ("the symmetric closure holds every transpose"), in every test's model construction. **Not demonstrated as a behavioural kill.** An unsorted row cannot form a valid pattern, so no test assertion can be reached. |
| **K1-LABEL-ORDER** (mutation 10's demonstrated form: a label-dependent order) | natural (label) order in place of RCM in `order_sparse_structural` | SD `k1_adjacency_order_and_profile_…` (k1_tests.rs:422, "order"); SD `k1_explicit_zero_…` (:481); SD :566; NI byte identity: `solve_checked` (k1_tests.rs:514, reached from the N, T0R and relabel tests), `k1_pattern_entries_…` (:323), `k1_kd5_parity_…` (:350), `k1_c3_detect_…` (:436) |
| K1-DENSEPAIRS | the witness scans dense pairs | FK `k1_negative_witness_visits_only_pattern_pairs` (:1214, visited = stored pairs) |
| K1-COALESCE | `=` for `+=` in `from_pattern_and_contributions` | FK `k1_coalesced_values_…` (:392); FK `krev03_on_sparse` (:1048); `s11_site_table` (:524); SD `k1_explicit_zero_…` (:464) |
| K1-COALESCE-SCATTER | `=` for `+=` in `scatter_block` | FK `k1_assembly_refusals_…` (:445, the refusal no longer occurs); FK :377, :569, :875, :1211; `s11_site_table` (:524); SD (:392, :566); NI (:261) |
| K1-SITE-SPARSE | a binary64 fold in `multiply` | `s11_site_table::site_table_is_exactly_the_accumulations_of_the_scanned_files` (:524; `multiply` table 1, source 2) |
| K1-SITE-FC | a binary64 fold in `pow2` | the same test (:524; `pow2` table 1, source 2) |
| K1-S11F-KERNEL (ROOT's KERNEL-list mutant) | `self.source.force.len()` read in `SparsePreparedSystem::free_dofs` | PP `rule_6_kernel_functions_that_touch_the_force_are_the_listed_sites` (s11f_site_test.rs:1194). The only difference in the sets is `(FK/structural/sparse.rs, free_dofs)`. |
| K1-PIN-BINARY64 (required: an exact entry inside `solve_binary64`) | `prepare_structural` in its body | pin `option_c_structural_adapter_legacy_variants_…` (s11k_tests.rs:566, the required legacy text); behavioural `tests::unsupported_gap_inspection_*` (lib.rs:4781, :4827). The :796 and :1457 failures are discrimination preconditions and are not counted. |
| K1-PIN-BINARY64-B (the extended scan itself) | the legacy text kept, plus an exact call | the same pin at **:584**, the impl-scoped forbidden-entry scan ("exact kernel entry point prepare_structural( … outside the exact variants"); behavioural unsupported_gap ×2 |
| K1-PIN-THIRD (required) | a third definition, in a new impl | pin `kd5_nonlinear_sources_name_no_formation_check_entry_point` (s11k_tests.rs:1130; 3 ≠ 2) |
| **K1-PIN-LOOP** (required, as drafted) | the loop calls the pattern formation entry (evidence with no elements) | **pins :519** (`option_c_nonlinear_loop_is_pinned_…`, legacy call target missing) **and :1130** (lib.rs names the entry). **Behavioural pin (3) is not reached:** the loop errors first; pin (3) and about 40 lib.rs tests fail at the `.unwrap()` of the loop call. |
| **K1-PIN-LOOP-B** (the demonstration of pin (3)) | the loop's solve goes through an SA helper that builds the pattern evidence from the assembly's own primitives and calls the pattern formation entry, so the loop completes | **pin (3) `k1_nonlinear_loop_reaches_neither_formation_entry` at k1_tests.rs:1000** (the first iteration is Sensitive; Passed is required); K-D5's `kd5_nonlinear_loop_reaches_no_formation_check` (:1262); `option_c_active_set_…` (:913); `option_c_closed_gap_…` (:823); pins :519 and :1130 |
| KD5-M32a (original) | the loop's `solve_binary64` routed through the formation check | `kd5_nonlinear_loop_reaches_no_formation_check` (:1262; original :1196); `kd5_nonlinear_sources_name_…` (:1155; original :1089); `kd5_nonlinear_loop_unit_force_…` (:1457; original :1391, the same precondition site as K-D5's record); also pin (3) :1000 and :566 |
| KD5-M32b (original) | the loop calls K-D5's typed entry | :1262; :1130 (original :1071); :1457; also :913, :823, :519, pin (3) :1000 and unsupported_gap ×2 |
| KD5-E4 (original, RV5's evasion) | a neutral helper in a sibling module | `kd5_nonlinear_loop_unit_force_solves_…` (:1477; original :1411, the derived friction force); `kd5_nonlinear_sources_name_…` (:1130; original :1071) |
| S11K-RV-OPT1 (original) | the loop reduction switched to exact KS2 | `option_c_closed_gap_…` (:833); `option_c_active_set_…` (:919); `option_c_nonlinear_loop_is_pinned_…` (:519): the original 3 |
| S11K-RV-OPT3 (original) | the loop's dense and sparse solves switched to exact | :823; :519: the original 2 |
| S11K-RV-OPT4 (original) | an exact solve through a helper | :913; :823; :519; unsupported_gap ×2 (lib.rs :4783, :4829): the original 5, plus K-D5's `…sources_name…` (:1201) and the :1457 precondition |
| S11K-RV-PUB (original) | the public residual switched to exact KS3 | FK `structural::s11k_tests::option_c_public_original_residual_stays_binary64_on_coupled_rows` (s11k_tests.rs:660): the original; plus NI unsupported_gap ×2 |

- **No weakening of the original pins.** Every original mutant fails every test of its original kill set, at the same assertions. The line shifts come from the pin extension (+59 to +66 lines in `s11k_tests.rs`).
- **Mutation 10 and relabelling** (stated as found, per ROOT). A label-dependent order is caught by:
  - the order and profile parity test;
  - the byte-identity tests, including the relabel test's byte-identity check at `k1_tests.rs:514`.

  It is not caught by the relabel test's 1e-9 value agreement, because the answers stay within 1e-9. That is a correct observation, not a gap.

## 11. Storage counts (`_run_records/storage_counts.txt`)

These are deterministic counts at kernel level, through the pattern path only; no dense matrix is formed at any size. The table records no timing and no memory growth; K6 owns those.
- **Models:** RF-LARGE-shaped chains and combs of 3 m members (the invented pipe section at 200 GPa and 80 GPa), axis-aligned (AX) or rotated by Q3 (ROT), with the root fixed.
- **Columns:** stored = the global pattern (both triangles); f_lower = the free block's stored lower entries; f_lo_nz = those that are nonzero; profile = the RCM skyline entries.

| Family | n | DOFs | Stored | Lower | f_lower | f_lo_nz (AX / ROT) | Profile (AX / ROT) | Half-bw (AX / ROT) | Dense n² |
|---|---|---|---|---|---|---|---|---|---|
| chain | 10 | 66 | 1,116 | 591 | 534 | 152 / 396 | 168 / 529 | 4 / 13 | 4,356 |
| chain | 100 | 606 | 10,836 | 5,721 | 5,664 | 1,592 / 4,176 | 1,788 / 5,569 | 4 / 13 | 367,236 |
| chain | 1,000 | 6,006 | 108,036 | 57,021 | 56,964 | 15,992 / 41,976 | 17,988 / 55,969 | 4 / 13 | 36,072,036 |
| chain | 10,000 | 60,006 | 1,080,036 | 570,021 | 569,964 | 159,992 / 419,976 | 179,988 / 559,969 | 4 / 13 | 3,600,720,036 |
| comb | 10 | 66 | 1,116 | 591 | 534 | 172 / 446 | 389 / 595 | 17 / 17 | 4,356 |
| comb | 100 | 606 | 10,836 | 5,721 | 5,664 | 1,792 / 4,721 | 4,933 / 6,715 | 19 / 17 | 367,236 |
| comb | 1,000 | 6,006 | 108,036 | 57,021 | 56,964 | 17,992 / 47,471 | 49,933 / 67,915 | 19 / 17 | 36,072,036 |
| comb | 10,000 | 60,006 | 1,080,036 | 570,021 | 569,964 | 179,992 / 474,971 | 499,933 / 679,915 | 19 / 17 | 3,600,720,036 |

The stored and lower counts depend only on the member and node counts, so AX and ROT are equal there. The global counts are 36 × (nodes + 2 × members).

## 12. F1b interface (exact signatures, at `19925122b`)

Everything below is re-exported from `open_pipe_stress_frame_kernel::structural`, except the sparse_direct and SA items, which are in their own crates.

**Pattern and values** (`FK/structural/sparse.rs`):
```rust
pub struct SparsePattern { /* private: dimension, row_starts, columns, transpose */ }
impl SparsePattern {
    pub fn from_positions(dimension: usize, positions: impl IntoIterator<Item = (usize, usize)>) -> Result<Self, StructuralError>;
    pub fn from_connectivity(node_count: usize, elements: &[(usize, usize)], diagonal: &[usize]) -> Result<Self, StructuralError>;
    pub fn dimension(&self) -> usize;
    pub fn entry_count(&self) -> usize;
    pub fn lower_entry_count(&self) -> usize;
    pub fn row_range(&self, row: usize) -> std::ops::Range<usize>;
    pub fn row(&self, row: usize) -> &[usize];
    pub fn find(&self, row: usize, col: usize) -> Option<usize>;
    pub fn column(&self, index: usize) -> usize;
    pub fn transpose(&self, index: usize) -> usize;
}
pub struct SparseStorageCounts { pub dimension: usize, pub stored_entries: usize, pub lower_entries: usize, pub dense_entries: u128 }
pub struct SparseStiffness { /* private: pattern, values */ }
impl SparseStiffness {
    pub fn from_pattern_and_contributions(pattern: SparsePattern, contributions: &[StiffnessContribution]) -> Result<Self, StructuralError>;
    pub fn from_contributions(dimension: usize, contributions: &[StiffnessContribution]) -> Result<Self, StructuralError>;
    pub fn from_dense(dense: &[Vec<f64>]) -> Result<Self, StructuralError>;
    pub fn pattern(&self) -> &SparsePattern;
    pub fn values(&self) -> &[f64];
    pub fn dimension(&self) -> usize;
    pub fn get(&self, row: usize, col: usize) -> f64;
    pub fn row(&self, row: usize) -> impl Iterator<Item = (usize, f64)> + Clone + '_;
    pub fn to_dense(&self) -> Vec<Vec<f64>>;                                   // dense view, n^2
    pub fn storage_counts(&self) -> SparseStorageCounts;
    pub fn multiply(&self, u: &[f64]) -> Result<Vec<f64>, StructuralError>;   // = PP multiply_matrix_vector, bit for bit
    pub fn reactions(&self, u: &[f64], force: &AssembledForce) -> Result<Vec<f64>, StructuralError>; // E12 from sparse rows
}
```

**Assembly, per modulus basis:**
```rust
#[non_exhaustive] pub struct SparseAssemblyOptions {}      // K2b adds b here
impl SparseAssemblyOptions { pub fn new() -> Self; }
pub struct StiffnessBlock { pub node_i: usize, pub node_j: usize, pub stiffness: Matrix12 } // a realized curved bend's global matrix
pub fn assemble_sparse_stiffness(node_count: usize, frames: &[FrameElement], users: &[UserStiffnessElement],
    blocks: &[StiffnessBlock], springs: &[(usize, f64)], options: &SparseAssemblyOptions) -> Result<SparseStiffness, FrameKernelError>;
```

**Reduction with prescribed displacements, and partition maps:**
```rust
pub struct SparseReducedSystem { pub free_dofs: Vec<usize>, pub prescribed: Vec<(usize, f64)>, pub force: ReducedForce, /* private free_position */ }
impl SparseReducedSystem { pub fn free_position(&self, dof: usize) -> Option<usize>; }
pub fn reduce_assembled_sparse_system(stiffness: &SparseStiffness, force: &AssembledForce, boundary_dofs: &[usize],
    displacements: Option<&[f64]>) -> Result<SparseReducedSystem, FrameKernelError>;
```

**Systems and the gate:**
```rust
pub struct SparseSymmetryEvidence<'a> { pub absolute_roundoff: &'a [f64], pub operation_counts: &'a [usize], pub basis: &'a str }
impl<'a> SparseStructuralSystem<'a> {
    pub fn new(stiffness: &'a SparseStiffness, force: &'a [f64], free_dofs: &'a [usize], prescribed: &'a [(usize, f64)],
        contributions: Option<&'a [StiffnessContribution]>, symmetry: Option<SparseSymmetryEvidence<'a>>) -> Self;
    pub fn assembled(stiffness: &'a SparseStiffness, force: &'a AssembledForce, free_dofs: &'a [usize], prescribed: &'a [(usize, f64)],
        contributions: Option<&'a [StiffnessContribution]>, symmetry: Option<SparseSymmetryEvidence<'a>>) -> AssembledSparseStructuralSystem<'a>;
    pub fn with_formation_source(self, source: &'a FormationSource) -> FormationCheckedSparseSystem<'a>;
    pub fn stiffness(&self) -> &'a SparseStiffness; pub fn force(&self) -> &'a [f64];
    pub fn free_dofs(&self) -> &'a [usize]; pub fn prescribed(&self) -> &'a [(usize, f64)];
}
impl<'a> AssembledSparseStructuralSystem<'a> { pub fn system(&self) -> &SparseStructuralSystem<'a>; pub fn force(&self) -> &'a AssembledForce;
    pub fn with_formation_source(self, source: &'a FormationSource) -> FormationCheckedSparseSystem<'a>; }
impl<'a> FormationCheckedSparseSystem<'a> { pub fn system(&self) -> &SparseStructuralSystem<'a>; pub fn source(&self) -> &'a FormationSource; }
impl SparsePreparedSystem<'_> { pub fn free_dofs(&self) -> &[usize]; pub fn scale_exponents(&self) -> &[i32]; pub fn dimension(&self) -> usize;
    pub fn entry_count(&self) -> usize; pub fn lower_entries(&self) -> impl Iterator<Item = (usize, usize, f64)> + '_; }
pub fn prepare_sparse_structural<'s>(system: &'s SparseStructuralSystem<'s>) -> Result<SparsePreparedSystem<'s>, StructuralError>;
pub fn prepare_assembled_sparse_structural<'s>(system: &'s AssembledSparseStructuralSystem<'s>) -> Result<SparsePreparedSystem<'s>, StructuralError>;
pub fn prepare_formation_checked_sparse_structural<'s>(system: &'s FormationCheckedSparseSystem<'s>) -> Result<SparsePreparedSystem<'s>, StructuralError>;
pub fn prepare_sparse_structural_with_force_terms<'s>(system: &'s SparseStructuralSystem<'s>, force_terms: &'s [ForceTerm]) -> Result<SparsePreparedSystem<'s>, StructuralError>;
pub fn factor_sparse_structural_profile<'p, 's>(prepared: &'p SparsePreparedSystem<'s>, order: &[usize], first: &[usize]) -> Result<SparsePositiveFactor<'p, 's>, StructuralError>;
impl SparsePositiveFactor<'_, '_> { pub fn pivots(&self) -> &[PivotEvidence]; }   // (addendum 1, N7)
pub fn finish_sparse_structural(factor: &SparsePositiveFactor<'_, '_>) -> Result<StructuralSolution, StructuralError>;
pub fn audit_sparse_load_fidelity(system: &SparseStructuralSystem<'_>, u: &[f64], force_terms: &[ForceTerm]) -> Result<LoadFidelityReport, StructuralError>;
pub fn verify_sparse_negative_direction(prepared: &SparsePreparedSystem<'_>, direction: &[f64]) -> Result<Option<StructuralError>, StructuralError>;
pub fn sparse_negative_pair_witness(prepared: &SparsePreparedSystem<'_>) -> Result<Option<StructuralError>, StructuralError>;
```

**sparse_direct** (`open_pipe_stress_sparse_direct::structural`):
```rust
pub struct SparseStructuralOrdering { pub order: Vec<usize>, pub first_columns: Vec<usize>, pub profile_entry_count: usize, pub max_half_bandwidth: usize }
pub fn order_sparse_structural(prepared: &SparsePreparedSystem<'_>) -> Result<SparseStructuralOrdering, StructuralError>; // counts before the factor allocates
pub fn factor_sparse_structural_ldlt<'p, 's>(prepared: &'p SparsePreparedSystem<'s>) -> Result<SparsePositiveFactor<'p, 's>, StructuralError>;
pub fn solve_sparse_prepared(prepared: SparsePreparedSystem<'_>) -> Result<StructuralSolution, StructuralError>;
pub fn solve_sparse_structural(system: &SparseStructuralSystem<'_>) -> Result<StructuralSolution, StructuralError>;
pub fn solve_assembled_sparse_structural(system: &AssembledSparseStructuralSystem<'_>) -> Result<StructuralSolution, StructuralError>;
pub fn solve_formation_checked_sparse_structural(system: &FormationCheckedSparseSystem<'_>) -> Result<StructuralSolution, StructuralError>;
```

**SA** (`open_pipe_stress_nonlinear_integration::structural_adapter`), returning today's `StructuralSolution`:
```rust
impl SparseAssemblyEvidence {
    pub fn new(pattern: &SparsePattern, node_count: usize, frames: &[FrameElement], users: &[UserStiffnessElement],
        curved: &[CurvedBendStiffnessElement], springs: &[(usize, f64)]) -> Result<Self, StructuralError>;
    pub fn with_force_terms(mut self, terms: &[ForceTerm]) -> Self;
    pub fn qualified_passive_family(&self) -> bool;
    pub fn geometry(&self, prescribed: &[(usize, f64)]) -> Result<(), StructuralError>;
    pub fn pattern(&self) -> &SparsePattern;
    pub fn contributions(&self) -> &[StiffnessContribution];
    pub fn absolute_roundoff(&self) -> &[f64];
    pub fn operation_counts(&self) -> &[usize];
    pub fn storage_counts(&self) -> SparseEvidenceCounts;                     // { pattern_entries, contributions }
    pub fn dense_symmetry_view(&self) -> (Vec<Vec<f64>>, Vec<Vec<usize>>);    // for dense scrutiny
    pub fn solve_assembled(&self, k: &SparseStiffness, f: &AssembledForce, free: &[usize], prescribed: &[(usize, f64)],
        mode: LinearSolveMode) -> Result<StructuralSolution, StructuralError>;
    pub fn solve_assembled_with_formation_check(&self, k: &SparseStiffness, f: &AssembledForce, free: &[usize],
        prescribed: &[(usize, f64)], mode: LinearSolveMode, curved_sources: &[CurvedBendMacroElement], selected: bool)
        -> Result<StructuralSolution, StructuralError>;
    pub(crate) fn solve(&self, k: &SparseStiffness, f: &[f64], free: &[usize], prescribed: &[(usize, f64)],
        mode: LinearSolveMode) -> Result<StructuralSolution, StructuralError>;   // C3-detect with force terms
}
pub struct SparseEvidenceCounts { pub pattern_entries: usize, pub contributions: usize }
```

**How F1b maps onto this** (the PP sites are the design's and the brief's references, `PP:1620`, `:1751`, `:2330-2342` and `:2697`, not re-located at this base):
- **Assembly,** per modulus basis: `assemble_sparse_stiffness`, then `SparseAssemblyEvidence::new(k.pattern(), …)`.
- **Partition maps:** `reduce_assembled_sparse_system` and `free_position`.
- **Reactions:** `SparseStiffness::reactions`. PP's legacy observation force stays in PP.
- **`solve_preview_reduced_system`:** `SparseAssemblyEvidence::solve_assembled_with_formation_check`.
- **Dense scrutiny,** and `source_recovery`'s n ≤ 256 case if needed: `SparseStiffness::to_dense` and `dense_symmetry_view`.
- **The resource guard:** `SparseStorageCounts`, `SparseEvidenceCounts`, `SparseStructuralOrdering::profile_entry_count` and `SparsePreparedSystem::entry_count`.
- **K2b adds b** in `SparseAssemblyOptions`.
- **Not implemented here:** the dense-scrutiny and sparse resource-guard ceilings (ROOT picks them from measurement, and F1b wires the refusal); the nonlinear loop's move to sparse (F1b with T5).

## 13. K2a interaction (pending)

- **The draft:** `wip/k1_k2a_interaction.rs.txt`, intended as `frame_kernel/tests/k1_k2a_interaction.rs`. It uses K2a's `tests/k2a/rf_range_models.rs`, so it is valid only after K2a merges and ROOT merges main into this branch. It covers:
  - the RF-RANGE members formed or refused identically in both assemblies;
  - K2a's reach_zero and reach_lef refused with the same `NumericalRange { name }`;
  - the same first failing element in both.
- **I8's finding, not re-verified here:** RF-RANGE LEF-small never reaches assembly. `FrameElement::new` refuses it first (`DegenerateAxis`, as K2a records), identically for both representations.
- **Then:** land the test, and re-run §6–§10 on the combined tree (WIP_STATE §4 step 6). K1's PR cannot merge before K2a's.

## 14. The gate

**Not run, per ROOT's ruling** ("K1: spawn timing and no both-entry gate"). K1 is kernel only and changes no published byte. The evidence is the parity tests and T9. The both-entry gate runs at F1b, when PP switches to the sparse representation.

## 15. What was not done, and open items

- **Not done:**
  - The K2a interaction tests and the combined-tree re-run (§13).
  - No timing and no memory-growth claims (K6). The 80.6 s in §6(g) is a debug-build test duration given as the reason for a fixture change, nothing more.
  - No PP, `FK/lib.rs`, `formation_check.rs` (beyond its site-table rows), `exact_boundary.rs`, `nonlinear_integration/src/lib.rs`, `curved_bend` or `diagnostics` edits.
  - No formation-time scaling (K2b), W4 witness or curved screen (K5), performance harness (K6), or resource-guard ceilings (F1b).
  - The pre-existing rustfmt drift in `exact_boundary/functionals*.rs` is not touched.
- **For ROOT and the manager:**
  - the history reshaping, with the separate `formation_check.rs` site-table commit;
  - the PR's hosted CI with the surface-4 dispatch;
  - the DEC-025 sweep;
  - the independent complete-diff review, which covers the declared extensions of §3.5 and the reviewer items of §6;
  - CI on Linux, which stays the authority for the suites.
- **Recorded, not a gap:**
  - K1-LABEL's kill is a fail-closed production invariant only (§10).
  - Mutation 10 is caught by order parity and byte identity, not by the relabel test's 1e-9 agreement (§10).

## 16. Records

- **`_run_records/`** is built by `assemble_run_records.py.txt` from I8R's scratch directory. It contains:
  - `toolchain.txt`, `callers.txt`, `scan_callers_k1.py.txt` and `storage_counts.txt`;
  - `checkpoint_a/`: the compile check and targeted-test logs in run order, including the first failing runs;
  - `suites/`: the runner, the 39 logs, the comparison, and the failure-block hashes;
  - `t9/`: the harness Cargo.toml files, the build and run logs, the output sha256 lists and the summary;
  - `mutations/`: the runner, the patches, the kill-site extractor, `MUTANTS.txt`, `kill_sites.txt` and 24 logs.
- **Sanitized:**
  - Machine paths became `<scratch>`, `<wt>` and `<home>`.
  - In logs, a line over 600 characters (Debug renderings of whole documents in failure output) is cut, with a marker giving the number of characters removed and the sha256 of the full line.
  - The summaries (`MUTANTS.txt`, `kill_sites.txt`, `suites_compare.*`, `callers.txt`) are not cut.
  - Trailing spaces and trailing blank lines were stripped from the logs and `MUTANTS.txt` (ROOT, at commit), so `git diff --check` is clean for the logs. The WIP `.patch` files keep their patch-syntax spaces: an empty context line is a single space, which `--check` flags (see addendum 1, N2). No other byte changed, and no failure block of `suites/failure_blocks_sha256.txt` was affected.
- **The WIP artefacts are unedited:** `WIP_STATE.md`, `kernel_list_s11f.patch` and `wip/`. `kernel_list_s11f.patch.NOTE.md` is new.
- **`SHA256SUMS`** covers every file in this folder except itself.
- **GEN-8** (`pytest tools/practitioner_harness/test_live_baseline.py -k gen8`, run from `<wt>/k1` with `<VENV>`) passed: 1 passed, 10 deselected.
  - GEN-8 scans git-tracked files only, and these records were not yet committed, so it did not cover them.
  - The harness's own `MACHINE_ABS_PATH_RE` (`surface_roles.py`) was therefore applied directly to every file in this folder: 123 files, 0 hits.
  - GEN-8 is to be re-run after ROOT commits the records.

## RETURN addendum 1 (RV8 findings)

**RV8's independent review** (`T3/REVIEW/K1_REVIEW.md`, numerics `369dc2f16`; its probe, models and mutant patches are in `T3/REVIEW/_run_records/k1_review/`):
- **Verdict:** PASS at `43f9e6a78`, with 0 BLOCKING, 3 SHOULD-FIX and 7 NOTE findings.
- **The three SHOULD-FIX findings** are test gaps. Each is a mutant of RV8's that survived K1's frame_kernel, sparse_direct and nonlinear_integration suites. The product code was correct and is unchanged.
- **The fixes are tests only,** committed by ROOT as `340e87a2d` on `43f9e6a78` (append-only: 389 lines at the end of `P/core/solver/nonlinear_integration/src/structural_adapter/k1_tests.rs`).
- **Main (with K2a) was then merged** as `3b86b111f`.

### A1.1 The three fixes

The tests are in nonlinear_integration because that is where realized bends exist; frame_kernel has no dependency on `curved_bend`.
- Each test first asserts, on the dense side only, that its case discriminates the alternative its mutant takes. A mutant therefore fails at the test's behavioural assertion, not at a precondition.
- The cases are RV8's (invented): R = 0.6 m bends in the XY plane on K-D5's pipe section, E 200 GPa and G 80 GPa.

| Finding | Test | Case | Assertions | RV8 mutant (patch unchanged, `rv8_mutants.py.txt`) | Killed at |
|---|---|---|---|---|---|
| RV8-1: the accumulation order of blocks (realized bends) | `k1_bend_bend_tee_adds_the_realized_bends_in_the_products_order_rv8_1` | a tee at a bend–bend junction: b1 (0→1) and b2 (1→2), anchored runs 3–0 and 2–4, and a branch 1–5, so node 1 carries the branch and both bends; flexibility factors 1.0 and 1.7 | precondition: the bends in reverse order change the dense bits. Then: K bits against PP's dense order; the evidence contributions coalesced on the pattern; K·u; E12 reactions; SA's plain and formation-checked entries, byte-identical in both modes and `Ok` | RV8-BLOCK-ORDER (`blocks.iter().rev()` at sparse.rs:496) | `k1_tests.rs:1072`, the K-bits assertion ("bend-bend tee flex=1: K bits") |
| RV8-2: springs against blocks on a shared diagonal | `k1_bend_support_springs_follow_the_bend_in_the_products_order_rv8_2` | a bend between anchored runs 2–0 and 1–3, with support springs on all 12 DOFs of both bend ends, at RV8's 8 magnitudes (1.1e3–9.7e8 N/m) and 2 flexibility factors | the same checks, on 16 cases. The dense precondition, "some magnitude discriminates", is asserted last. It discriminates on 6 cases (k = 4.4e7, 2.1e8 and 9.7e8 N/m at both factors), RV8's k5–k7 | RV8-SPRING-FIRST (springs before blocks) | `k1_tests.rs:1072`, the K-bits assertion ("bend supports flex=1 k=4.4e7: K bits") |
| RV8-3: the formation check's load input in the sparse representation | `k1_split_ledger_formation_check_reads_the_ledger_terms_in_both_representations_rv8_3` | K-D5's F122 and CSKEW_8_5, with each load split into three ledger terms (0.1v, 0.7v, v − 0.1v − 0.7v) | precondition 1: on some load, the exact sum of the terms is not the rounded net. Precondition 2 (dense only): the record with the terms differs from the record with the folded values. Then, in each mode, dense and sparse are byte-identical, and both **demote** (Sensitive, `Estimate`) in both representations | RV8-FC-TERMS (the sparse check drops the ledger terms) | `k1_tests.rs:1386`, the dense/sparse parity assertion ("F122 SparseInteractive: split ledger"). DenseScrutiny passes first, as expected: the pattern entry uses the dense path in that mode. |

- **Runs** (`_run_records/rv8_fixes/`), by checkpoint C's method:
  - each run is a clean `git archive` of `43f9e6a78` (without `execution/`, with fresh mtimes), with the working-tree `k1_tests.rs` copied over it (`overlay.txt`), and its own target under `<wt>/k1-mut/<id>/`, both deleted after the run;
  - tests: FK, SD and NI in full, plus PP `--test s11f_site_test --test formation_check_runtime`; `-j 4`, `RUST_TEST_THREADS=4`, `--no-fail-fast`.
- **NONE (control): clean.** FK 154 + 3 + 6; SD 30; NI 97 + 4 doc (the earlier 98, plus the 3 new); PP 11 + 5.
- **Each RV8 mutant** exits 101 in NI only, with 0 compile errors.
- In `kill_sites.txt`, a "BUILD ERROR" line is the extractor's label for cargo's `error: 1 target failed` summary of failing tests, not a compile error. `MUTANTS.txt` records `compile_errors=0`.
- The worktree run of the three tests, with the list of discriminating cases, is `test_ni_rv8_worktree.log`.
- The files in `rv8_fixes/` were sanitized by `sanitize_copy.py.txt`: the same placeholders and 600-character cuts, plus the removal of trailing whitespace at copy time.

**Mutation-table extension** (to §10):

| Mutant | Change | Kill sites |
|---|---|---|
| RV8-BLOCK-ORDER | realized bends (blocks) added in reverse | NI `k1_bend_bend_tee_…_rv8_1` (k1_tests.rs:1072, K bits) |
| RV8-SPRING-FIRST | springs added before the blocks | NI `k1_bend_support_springs_…_rv8_2` (k1_tests.rs:1072, K bits) |
| RV8-FC-TERMS | the sparse formation check ignores the ledger terms | NI `k1_split_ledger_…_rv8_3` (k1_tests.rs:1386, SparseInteractive parity) |

**No existing kill site moved.** The change only appends at the end of the file, so every existing test and line is unchanged, and the §10 table stands. My earlier mutants were not re-run against the new tests. The new tests can add kill sites to them (for example the bitwise checks for K1-M8 or K1-COALESCE-SCATTER), but cannot move one.

**RV8's suggestion to add the junction to FK's `k1_coalesced_values_…`** was not taken: FK cannot build a realized bend. The NI test pins PP's order with real bends. F1b must still pass PP's curved order to `assemble_sparse_stiffness` (N7).

### A1.2 The NOTEs

- **N1: commit mapping.** The reshape moved the pre-reshape commits this RETURN cites.
  - `d08b0efc7` (I8's WIP) → its code went into (a) `826a9eed4`, and its WIP artefacts into (d) `43f9e6a78`, blob-identical.
  - `9d4ba0e17` (checkpoint A's test fixes) → (a) `826a9eed4`.
  - `19925122b` (the KERNEL-list hunk; the tested candidate of checkpoints B and C) → (c) `4319854dc`. The two have the same code tree, and `git diff 19925122b 4319854dc -- projects/chirality-piping/core` is empty. `19925122b` also carried I8's WIP records, which (c) does not.
  - `3513fd8ab` (the checkpoint-D records snapshot) → (d) `43f9e6a78`, with the same tree `3048ebed0`.
  - (b) `85626dbe0` is the `formation_check.rs` site-table commit split out of (a) at the reshape.
  - **After the reshape:** the RV8 fixes are `340e87a2d`, and the merge of main (`f12e06876`, K2a) is `3b86b111f`.
  - All four pre-reshape commits remain reachable through `origin/codex/piping-k1-wip-20260928`.
- **N2: ROOT's §16 bullet.** ROOT's commit-time edit duplicated the whitespace bullet in §16 and overstated it ("so that `git diff --check` is clean"). The duplicate is deleted, and the remaining line now reads: `git diff --check` is clean for the logs; the WIP `.patch` files keep their patch-syntax spaces (an empty context line is a single space), which `--check` flags. There are three such lines, in `kernel_list_s11f.patch` and `wip/site_table_formation_check_hunks.patch`, kept byte-identical to `d08b0efc7`.
- **N3: `order_sparse_structural` allocates a profile-sized array.** It calls `SymmetricProfileMatrix::from_entries_with_order`, which allocates and fills a profile-sized `values` vector (`sparse_direct/src/lib.rs`) before `SparseStructuralOrdering` returns `profile_entry_count`. §12's "counts before the factor allocates" is true of the factor, but one profile-sized allocation has already happened. **For F1b:** the resource guard must count before calling `order_sparse_structural` (for example from the skyline alone: the first-column pass over the ordered entries), or use a count-only entry. This is not implemented here.
- **N7:** `SparsePositiveFactor::pivots()` is added to the §12 list. **For F1b:** nothing maps SA's curved slots (`CurvedBendStiffnessElement`) to `StiffnessBlock`s. F1b must build the blocks in PP's curved order itself, or add a crate-local helper; RV8-1's test pins that order.
- **N4, N5 and N6** need no action (the text pins' documented limit; the fixture changes are not weakenings; the invalid-input differences are acceptable).

## RETURN addendum 2 (the combined tree: K2a merged, the interaction tests, A–C re-run)

**The tree:**
- `3b86b111f`, ROOT's merge of main `f12e06876` (K2a, PR #1032) into K1 after the RV8 tests `340e87a2d`. There were no conflicts.
- K2a's final head `aad23e82d` is in `f12e06876`. `git diff aad23e82d f12e06876 -- projects/chirality-piping/core` is empty.
- On top: the K2a interaction tests (A2.1) and these records, uncommitted at the time of the runs. Every run below used `3b86b111f` with `frame_kernel/tests/k1_k2a_interaction.rs` copied over it.
- **No product code changed** in this addendum.

**Result:** the combined tree is green.
- The suites add no failure against the Mac baseline of merged main.
- T9 is byte-identical, 112 of 112 (Mac-only).
- 28 mutants and the NONE control were run. Every pre-merge kill site is kept at the same `file:line`, and the two K2a interaction mutants are killed at behavioural assertions.
- No stop rule triggered, and K2a's merged behaviour contradicts no K1 assumption.

### A2.1 Step 6: the K2a interaction tests

**Re-read before adapting the draft:**
- K2a's merged code: `FrameKernelError::NumericalRange { name: &'static str }`, and `local_stiffness`'s checked intermediates (`checked_formation_product`, `checked_formation_quotient`, `checked_formation_value`).
- `tests/k2a/rf_range_models.rs`: `RangeMember` and `RF_RANGE_MEMBERS`, plus the operand tables.
- K2a's own `tests/k2a_checked_formation.rs`:
  - LEF-small is refused on the element route as `DegenerateAxis { "element length" }`;
  - LEF-large is refused as `NumericalRange { "GJ/L: G*J" }` through `local_stiffness`, `global_stiffness` and `assemble_global_stiffness`;
  - every other vector forms bit-identically.
- The product-reach operands in `product_physics/tests/k2a_formation_range_runtime.rs` (`EXACT_ZERO`, `LEAST_SUBNORMAL`).

**How K1 meets K2a.** K1's sparse assembly forms each element with `global_stiffness()?`, in the dense order and before any accumulation. K2a's refusal therefore reaches both representations through the same call. This is as the draft assumed; nothing contradicts it.

**The test:** `P/core/solver/frame_kernel/tests/k1_k2a_interaction.rs` (new, 206 lines, sha256 prefix `cf241452b7a82c75`), I8's draft `wip/k1_k2a_interaction.rs.txt` adapted. The three tests:
- **`k1_k2a_rf_range_members_form_or_refuse_identically_in_both_representations`:** all 162 RF-RANGE members.
  - **Refused, 16** (LEF-large on the three bases): `NumericalRange { "GJ/L: G*J" }`, the same error in both representations and equal to the member's `refusal`.
  - **Formed, 98:** K bit-identical in both.
  - **Degenerate, 48:** LEF-small, L-240 and SIM-a, whose lengths (1.2e-60 m, 1.1e-72 m and 1.5e-36 m) are at or below FK's axis tolerance (1e-12 m). `FrameElement::new` refuses them (`DegenerateAxis { "element length" }`) before either assembly, identically for both.
- **`k1_k2a_reach_zero_and_reach_lef_are_refused_with_the_same_named_error`:** both give `NumericalRange { "12EIy/L^3: (12*E)*Iy" }`, identically in both. K2a's per-site subnormal rows make the least subnormal a refusal at the same site as the zero.
- **`k1_k2a_the_first_failing_element_is_the_same_in_both_representations`:** a normal member, then reach_zero, then a LEF-large member.
  - Both report reach_zero's refusal, which equals the refusal of reach_zero alone.
  - In reverse order, both report LEF-large's.

**Changes from the draft:**
- The degenerate class is explicit: a member is degenerate if and only if its length is at or below the tolerance, and it then gives exactly `DegenerateAxis { "element length" }`. The draft's comment named LEF-small only, but L-240 and SIM-a are degenerate too.
- LEF-small is asserted never to reach assembly.
- `#[allow(dead_code)]` on the shared K2a models module, since this file uses only the members.
- K2a's `member_properties` form, and rustfmt.

**The brief's "RF-RANGE LEF-small … refused with the same named error in both representations"** holds only at the element: LEF-small never reaches either assembly. I8 found this and this test verifies it. Its named `NumericalRange` exists only at `local_stiffness` level, which is not an assembly entry. This agrees with K2a's records (P1: `PIPE_ELEMENT_INPUT_INVALID`).

### A2.2 A: the compile and targeted tests (`_run_records/combined/checkpoint_a/`)

- `cargo check --tests` on FK, SD and NI: 0 errors, 0 warnings.
- Tests, all passing:

| Crate | Passed |
|---|---|
| frame_kernel | 179: lib 154, `k1_k2a_interaction` 3, `k2a_checked_formation` 13, `s11_site_table` 3, doc 6 |
| sparse_direct | 30 |
| nonlinear_integration | 101: lib 97 (with the 3 RV8 tests) and doc 4 |
| PP `s11f_site_test` | 11 |
| PP `formation_check_runtime` | 5 |

### A2.3 B: the suites (`_run_records/combined/suites/`)

- **Method:** all 39 manifests with `--no-fail-fast`, on the combined candidate archive.
- **Baseline:** ROOT's Mac run of merged main `f12e06876`, every manifest with `--no-fail-fast` (`<wt>/scratch/calib/suites_main_f12e06876/`).
- **Per test:** 0 changed and 0 removed. 34 were added, all passing: FK 17 (14 sparse and 3 interaction), NI 12 (9 adapter and 3 RV8), SD 5.
- **Per crate:** frame_kernel 162 → 179; sparse_direct 25 → 30; nonlinear_integration 89 → 101. PP (525 passed, 1 failed, 1 ignored, 18 targets) and headless (82 passed, 2 failed) equal the baseline, as do the other 34 manifests.
- **Failures:** exactly the three Mac platform tests (PP `t13_committed_fallback_uz…`; headless `load_reference_one_actual_solve…` and `cli_load_reference_one…`). Their failure output is byte-identical to the baseline's with thread ids removed: 300 bytes and 952,879 bytes (`failure_blocks_sha256.txt`).

### A2.4 B: T9 (Mac-only; `_run_records/combined/t9/`)

- **Setup:** base, a `git archive` of merged main `f12e06876`; candidate, a `git archive` of `3b86b111f` with the interaction test copied over it. Both are without `execution/`. The trees differ only in K1's files.
- **Harness:** S11-K's `fixdiff_main.rs` (`ec089c1d…`), `--release --offline --locked`.
- **Result:** **112 of 112 byte-identical**, base against candidate (core 10, fixtures 72, validation 30; 6 are `ERR` on both), and the raw outputs are identical.
- **Cross-check:** the base's hashes equal ROOT's Mac main `649162522` hashes on all 112, so K2a changes no committed output on the Mac either.
- This is never compared with Linux records.

### A2.5 C: the mutation table (`_run_records/combined/mutations/`)

- **Method:** checkpoint C's, on the combined tree: `run_mutant_combined.sh.txt`, dispatching through `apply.sh.txt`.
  - Patches: `mutate_k1.py` (as `mutations/mutate_k1.py.txt`), RV8's `rv8_mutants.py` (as `rv8_fixes/rv8_mutants.py.txt`), both unchanged, and the new `mutate_k2a.py.txt`.
  - All 28 anchors match once on the combined tree.
- **NONE: clean.** FK 154 + 3 + 13 + 3 + 6; SD 30; NI 97 + 4; PP 5 + 11.
- **28 mutants:** my 23, RV8's 3 and 2 K2a interaction mutants. **All are killed, with 0 compile errors.**
- **Every pre-merge kill site is kept** (`kill_site_comparison.txt`, from `compare_kill_sites.py.txt`): all 26 pre-merge mutants have 0 sites lost, at the same `file:line`.
- **Kill sites the combined tree adds,** all through the RV8 tests:
  - K1-M8: rv8_2 at k1_tests.rs:1072, and rv8_3 at :1364 (the dense entry on the mutated view is refused);
  - K1-COALESCE: rv8_1 and rv8_2 at :1082 (the coalesced contributions);
  - K1-COALESCE-SCATTER: rv8_1 and rv8_2 at :1072;
  - K1-LABEL-ORDER: rv8_1 and rv8_2 at :1128 (plain byte identity), and rv8_3 at :1386;
  - K1-LABEL: rv8_1 and rv8_2 through the same invariant panic at sparse.rs:153.
- **K1-LABEL is still killed only by that invariant**, as ruled.

**The K2a interaction mutants** (new; `mutate_k2a.py.txt`):

| Mutant | Change | Kill sites |
|---|---|---|
| K1-K2A-SKIP | the sparse assembly skips K2a's checked-formation refusal: a frame refused with `NumericalRange` is accumulated as a zero block | FK `k1_k2a_interaction.rs:104` (`assert_same`'s representation-mismatch arm: dense `Err(NumericalRange)`, sparse `Ok`), in all three tests (RF-RANGE LEF-large M1; reach_zero; "ordered") |
| K1-K2A-FIRST | the sparse assembly refuses a different first element: every frame's formation is checked in reverse order before the unchanged forward formation | FK `k1_k2a_interaction.rs:101` (`assert_eq!(dense, sparse)` on the error, "ordered", in `k1_k2a_the_first_failing_element_…`) |

### A2.6 Callers on the combined tree (`_run_records/combined/callers.txt`)

The same lexer scan, run on the combined candidate, finds 284 sites.
- **The 74 non-test sites are unchanged**, line numbers aside.
- The only additions are test callers: K2a's `k2a_checked_formation.rs` and `k2a_formation_range_runtime.rs`, the interaction test, and the RV8 tests.

### A2.7 Records

- **New folders:** `_run_records/rv8_fixes/` (addendum 1) and `_run_records/combined/` (this addendum).
- **Sanitized at copy** by `rv8_fixes/sanitize_copy.py.txt`: the same placeholders and 600-character cuts as before, and trailing whitespace removed.
- **No existing log was rewritten.**
- `SHA256SUMS` is regenerated over the whole folder.
