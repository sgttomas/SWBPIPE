# K1 change record: the W3 kernel sparse representation and the sparse M03 gate

This is the draft PR record for slice K1 of T3 (numerical integrity), following `.agents/skills/chirality-change/SKILL.md`.
- **Implementation:** I8 (TASK, cloud session) drafted it to a work-in-progress commit. I8R (TASK, the owner's Mac) compiled, finished and verified it.
- **Detail:** `RETURN.md`.

- **Branch:** `codex/piping-k1-20260928`, from main `134eefc24` (F1a merged).
- **Candidate verified:** `19925122b`. The records are added on top.
- **Merge order:** K1's PR cannot merge before K2a's (brief, condition 2).
- **Basis:**
  - D1 `DESIGN.md` revision 5a.2 (`fb62ef4a…`): §4.8 W3; the K1, K2b, K5, K6 and F1 rows of §6; §7.1; §7.3 mutations 8 and 10;
  - `TASK_BRIEFS/I8_K1_IMPLEMENTATION.md` with addenda 1–4, and `I8R_K1_RESUME.md`;
  - `ROOT_RULINGS_V1.md`: "K1: spawn timing and no both-entry gate", "K1: the S11 site table…", "K1: extending the K-D5 and option-(c) source pins…", "K1: the S11-F site test's KERNEL list, and the handoff";
  - ROOT's checkpoint rulings on the Mac (RETURN §1).

## What changes

K1 gives the kernel one sparse representation of the global stiffness, and the same M03 gate written over it, in O(nnz) memory. **The product keeps calling today's entries.** F1b will switch it (D1 §4.8; ROOT's F1 split).

- **`FK/structural/sparse.rs` (new):**
  - `SparsePattern`, a pure function of the coupled positions, so it depends on neither labels nor input order;
  - `SparseStiffness`, one coalesced value per pattern entry, summed in the dense assembly's order, so each entry is bit-identical to the dense entry;
  - `assemble_sparse_stiffness` with `SparseAssemblyOptions` (`#[non_exhaustive]`, so K2b can add the formation-time scale b without touching callers) and `StiffnessBlock`;
  - `reduce_assembled_sparse_system` (KS2 over the pattern), and `multiply` and `reactions` (E12 from sparse rows);
  - `SparseStructuralSystem` and its typed and formation-checked siblings;
  - the sparse `validate`, prepare (KS1), contribution audit, condition matrix, skyline factor fill, load audit, and negative witness and verification, over pattern pairs only.
- **`FK/structural.rs`:**
  - `mod sparse;` and its re-exports;
  - the gate's shared stages are written once, over two private traits:
    - `Represented` (the source equations) and `PreparedGate` (the prepared system);
    - the stages: the original-equation residual with KS3, the intended-action audit, the load-row audit, KS1's exact right-hand side and `finish_checked_factor` with K-D5's EF step;
  - `estimate_rcond`'s body and the skyline LDL are shared through a private trait and a method of the same names.
  - **The dense public API is unchanged in signature and in bytes.**
- **`sparse_direct/src/structural.rs`:**
  - the pattern path's `order_sparse_structural`: `adjacency_from_symmetric_entries`, RCM and `SymmetricProfileMatrix::from_entries_with_order`;
  - `factor_sparse_structural_ldlt`, `solve_sparse_prepared` and the three pattern-taking solves.
  - `factor_structural_ldlt` and `solve_structural_sparse` are unchanged.
- **`SA` (`nonlinear_integration/src/structural_adapter.rs`):**
  - `AssemblyEvidence::new` builds through a shared `EvidenceParts` with an n×n store (today's arithmetic and layout).
  - The new `SparseAssemblyEvidence` uses a per-pattern-entry store, and has pattern-taking `solve_assembled`, `solve_assembled_with_formation_check`, and a crate-private `solve` (C3-detect).
  - In `DenseScrutiny` mode these materialize the dense view and run today's dense Cholesky; in `SparseInteractive` mode they run the gate over the pattern.
  - `solve`, `solve_assembled`, `solve_assembled_with_formation_check`, `solve_binary64` and `solve_structural_sparse_binary64` are unchanged. The nonlinear loop (`nonlinear_integration/src/lib.rs`) is not touched.

### Declared extensions (each its own item)

1. **The S11 site table** (`frame_kernel/tests/s11_site_table.rs`; ROOT's option (a)): sparse.rs in SOURCES and 12 rows.
   - Stiffness sites are exemptions with reasons.
   - Load, force and RHS sums go through `ExactAccumulator` (count 0).
   - The binary64-fold mutant `K1-SITE-SPARSE` is killed.
2. **K-D5's `formation_check.rs` in the same table**, as a separate commit (ROOT): one row, `pow2`, an integer exponent step.
   - It has no plain binary64 load, force or RHS fold, so there is no K-D5 finding.
   - `K1-SITE-FC` is killed.
3. **The K-D5 and option-(c) source pins** (`nonlinear_integration/src/s11k_tests.rs`, additive, ROOT-approved):
   - blanking and definition counts scoped by impl block;
   - identifier-boundary tokens;
   - two definitions of the formation-checked entry, one per adapter impl;
   - the pattern path's exact and formation entries added to the forbidden lists;
   - the behavioural loop pin (3) added in `structural_adapter/k1_tests.rs`.
   - The three required mutants are killed, and every original pin mutant keeps its original kill set (RETURN §10).
4. **sparse.rs in `product_physics/tests/s11f_site_test.rs`'s KERNEL list** (ROOT), a write-set extension into PP tests only, committed on its own: the Source is appended, and 11 `FORCE_FUNCTIONS` rows are added. `K1-S11F-KERNEL` is killed at rule 6.

## Standing, values and bytes

- **No product call site changes, and no value or byte changes.** PP's code is not edited. PP, `source_recovery` and the nonlinear loop still call `AssemblyEvidence::new`, `solve_assembled_with_formation_check` and the `_binary64` entries.
  - Their dense path now runs the refactored shared code: the generic gate stages and the shared evidence builder.
  - That is shown byte-identical by the committed-fixture diff and by the unchanged suites.
- **No new caller of any new entry outside its own crate's tests** (`_run_records/callers.txt`).
- **The committed-fixture diff (T9), Mac-only:** 112 of 112 outputs are byte-identical, Mac base `134eefc24` against Mac candidate `19925122b`, both built from `git archive` copies (core 10, fixtures 72, validation 30; 6 are `ERR` on both).
  - The Mac base hashes equal ROOT's Mac main hashes on all 112.
  - Mac outputs are never compared with Linux records (`T3/PLATFORM_CALIBRATION_MAC/`).
  - The stop rule did not trigger.
- **The explicit-zero rule** (RETURN §3.7). An entry whose coalesced value is exactly 0 stays in the pattern: the contribution audit, the residual and the witness see it. It takes no part in adjacency, ordering or profile, exactly like a zero of today's dense matrix. The rule is applied by value, on the prepared (scaled, symmetrized) matrix that the factor orders, so the order and the profile are today's.
- **The conditional files are not used:** `formation_check.rs` and `exact_boundary.rs` are unedited.
  - The sparse representation reaches K-D5's check through a `StructuralSystem` view with empty stiffness. `check` reads only `force` and `free_dofs`, and the attempt's factor through `solve` (`formation_check.rs:193-194, 223`).
  - `formation_check.rs` gains only its site-table rows (item 2).

## Files (against `134eefc24`; line counts at `19925122b`)

| File | +/− | Lines |
|---|---|---|
| `P/core/solver/frame_kernel/src/structural/sparse.rs` (new) | +1830 | 1830 |
| `P/core/solver/frame_kernel/src/structural/sparse/tests.rs` (new) | +1485 | 1485 |
| `P/core/solver/frame_kernel/src/structural.rs` | +417 −165 | 2532 |
| `P/core/solver/sparse_direct/src/structural.rs` | +99 −2 | 226 |
| `P/core/solver/sparse_direct/src/structural/k1_tests.rs` (new) | +811 | 811 |
| `P/core/solver/nonlinear_integration/src/structural_adapter.rs` | +784 −282 | 2286 |
| `P/core/solver/nonlinear_integration/src/structural_adapter/k1_tests.rs` (new) | +1020 | 1020 |
| `P/core/solver/nonlinear_integration/src/s11k_tests.rs` (pin extension) | +86 −20 | 1497 |
| `P/core/solver/frame_kernel/tests/s11_site_table.rs` (sparse.rs hunks 1, 3; formation_check.rs hunks 2, 4) | +28 | 566 |
| `P/core/product_physics/tests/s11f_site_test.rs` (KERNEL list) | +62 | 1540 |

There are no other product, fixture, schema, Cargo or lockfile changes. `FK/lib.rs`, `PP`, `nonlinear_integration/src/lib.rs`, `curved_bend` and `diagnostics` are untouched.

## Checks (RETURN §5–§11)

All run on `aarch64-apple-darwin` with rustc 1.97.1, `CARGO_INCREMENTAL=0` and `--offline --locked`.
- **Suites:** every one of the 39 manifests of CI's cargo profile, `--no-fail-fast`, on a `git archive` of `19925122b`.
  - Each manifest's counts equal the Mac main baseline's, except frame_kernel (149 → 163), sparse_direct (25 → 30) and nonlinear_integration (89 → 98), which gain 28 new tests, all passing.
  - Per test: 0 changed, 0 removed.
  - The only failures are the three Mac platform tests of the baseline (PP `t13_committed_fallback_uz…`; headless `load_reference_one_actual_solve…` and `cli_load_reference_one…`). Their failure output is byte-identical to main's.
- **T9:** 112 of 112 byte-identical (Mac-only), as above.
- **Parity:**
  - K is bitwise equal, entry for entry, including two modulus bases and 13 K-D5 models with realized bends.
  - The order and profile equal the dense-derived path's, including explicit zeros.
  - The `StructuralReport` and `StructuralSolution` are byte-identical in `Debug` to today's dense-derived sparse path (legacy, C3-detect, typed and formation-checked; free DOFs ascending and permuted).
  - Outcome classes and values agree on N01–N09, R01–R07, NP-B, NP-D and T0R's M05-T, M05-R1 and M05-SPRING.
  - The K-D5 parity cases and the loop pin pass.
  - The O(nnz) witness visits exactly the stored pairs.
- **Mutations:** the NONE control is clean. 23 mutants were run, with 0 compile-only.
  - Every mutant is killed at a behavioural or pin assertion, except K1-LABEL. That one is killed only by a production invariant: a fail-closed panic at `SparsePattern` construction, `sparse.rs:153`. It is recorded as not demonstrated.
  - K1-LABEL-ORDER is mutation 10's demonstrated form.
  - Every original pin mutant (K-D5's M32a, M32b and E4; S11-K's RV-OPT1, RV-OPT3, RV-OPT4 and RV-PUB) keeps its original kill set.
- **The gate was not run,** per ROOT's ruling: K1 changes no published byte, and the gate runs at F1b.

## Remaining

- **K2a interaction tests:** drafted (`wip/k1_k2a_interaction.rs.txt`), and landed only after K2a merges and main is merged into this branch. The combined tree is then re-run.
- **Belonging to ROOT and the manager:** the PR's hosted CI (the surface-4 dispatch), the DEC-025 sweep, the independent complete-diff review, the history reshaping (including the separate `formation_check.rs` site-table commit), and GEN-8 on the records commit.
- **F1b:** switching PP to the pattern path, wiring the resource guard and the dense-scrutiny ceiling, and the both-entry gate. **K2b:** b at formation. **K6:** timing and memory.
