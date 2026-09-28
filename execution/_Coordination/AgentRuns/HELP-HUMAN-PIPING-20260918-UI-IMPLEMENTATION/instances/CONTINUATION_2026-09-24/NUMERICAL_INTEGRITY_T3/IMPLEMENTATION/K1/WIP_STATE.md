# K1 work-in-progress state (I8, handoff)

TASK I8 wrote this state record at a clean stopping point, for the local session that continues slice K1. The brief is `TASK_BRIEFS/I8_K1_IMPLEMENTATION.md` (numerics commit `1f4001174`). The base is main `134eefc24`, on branch `codex/piping-k1-20260928` in `<wt>/k1`.

**Nothing has been compiled or run.** No cargo command was run: the manager held the token, and the compile check was cancelled at the handoff. Every file below is a full draft that parses as far as I could check by reading it. The drafts are not placeholders, except where a file is marked otherwise.

Checks that did run use only the standard library (scripts under `wip/`):
- Rust formatting of the new files, using the stable toolchain's rustfmt. The 1.97.1 toolchain has no rustfmt.
- A standard-library port of both S11 site-test scanners. It gives the same counts as the tests' tables on the base.
- An anchor check of every mutation patch.
- A lexer caller scan.

No Git operations were made.

## 1. Files, per file

| File | State |
|---|---|
| `P/core/solver/frame_kernel/src/structural/sparse.rs` (new) | Full draft (about 1,800 lines): `SparsePattern`, `SparseStiffness`, `assemble_sparse_stiffness`/`SparseAssemblyOptions`/`StiffnessBlock`, `reduce_assembled_sparse_system`, `SparseStructuralSystem` (+ assembled, formation-checked), `validate_sparse`, `SparsePreparedSystem` and the four `prepare_*sparse*` entries, `sparse_audit_contributions`, `legacy_zero_product_fold`, `factor_sparse_structural_profile`, `finish_sparse_structural`, `audit_sparse_load_fidelity`, `verify_sparse_negative_direction`, `sparse_negative_pair_witness` (+ `_counted`), `pair_energy`. rustfmt-formatted. Not compiled |
| `P/core/solver/frame_kernel/src/structural/sparse/tests.rs` (new) | Full draft of the kernel tests (list in §4). rustfmt-formatted. Not compiled |
| `P/core/solver/frame_kernel/src/structural.rs` | Refactored in place. It declares `mod sparse;` and re-exports its items with `pub use sparse::{...}`. The gate stages are generic over two private traits, `Represented` (source equations) and `PreparedGate` (prepared system), plus a small `Equations` view (force, free map, prescribed). They keep **their function names**: `evaluate_original_residual_bound`, `audit_intended_action`, `finish_checked_factor` and `exact_scaled_rhs`, the last now taking the couplings. `audit_load_row` and `unaudited_row` take the `Equations` view and precomputed parts. `estimate_rcond`'s body moved into the same-named provided method of a private `ConditionMatrix` trait. The skyline LDL moved into the same-named associated fn `ProfileFactor::factor_structural_profile`. The public dense API is unchanged in signature. Not compiled |
| `P/core/solver/sparse_direct/src/structural.rs` | Adds `SparseStructuralOrdering`, `order_sparse_structural` (`adjacency_from_symmetric_entries`, RCM, `SymmetricProfileMatrix::from_entries_with_order`), `factor_sparse_structural_ldlt`, `solve_sparse_prepared`, `solve_sparse_structural`, `solve_assembled_sparse_structural` and `solve_formation_checked_sparse_structural`. Existing items are unchanged. Not compiled |
| `P/core/solver/sparse_direct/src/structural/k1_tests.rs` (new) | Full draft (list in §4). rustfmt-formatted. Not compiled |
| `P/core/solver/nonlinear_integration/src/structural_adapter.rs` (`SA`) | `AssemblyEvidence::new` now builds through the shared `EvidenceParts::new` with an `EvidenceStore`: `DenseEvidenceStore` (n×n, today's arithmetic) or `SparseEvidenceStore` (per pattern entry). `geometry`, `symmetry_basis`, `qualified_passive_family` and `formation_source` delegate to shared free fns or `BodyEvidence`. Adds `SparseAssemblyEvidence` and `SparseEvidenceCounts`. `AssemblyEvidence`'s layout and pub fields are unchanged. Declares `#[cfg(test)] mod k1_tests;`. Not compiled |
| `P/core/solver/nonlinear_integration/src/structural_adapter/k1_tests.rs` (new) | Full draft (list in §4). It includes K-D5's generated `kd5_models.rs` read-only via `#[path]`. rustfmt-formatted. Not compiled |
| `P/core/solver/nonlinear_integration/src/s11k_tests.rs` | The pin extension, applied (§2.1). Not compiled |
| `P/core/solver/frame_kernel/tests/s11_site_table.rs` | The site-table extension, applied (§2.2). Not compiled |
| Not edited | `FK/lib.rs`, `formation_check.rs`, `exact_boundary.rs`, `PP` and its tests, `nonlinear_integration/src/lib.rs`, `curved_bend`, `diagnostics` |

Artefacts copied here so they survive the handoff:
- `kernel_list_s11f.patch`
- `wip/site_table_sparse_hunks.patch`
- `wip/site_table_formation_check_hunks.patch`
- `wip/k1_k2a_interaction.rs.txt`
- `wip/mutate_k1.py.txt`
- `wip/scan_callers_k1.py.txt` and its output `wip/callers_draft.txt` (a pre-compile snapshot)
- `wip/site_scan.py.txt` (the port of `s11_site_table.rs`'s scanner)
- `wip/pp_rules.py.txt` (the port of `s11f_site_test.rs` rules 2, 3 and 6)
- `wip/run_check.sh.txt`

## 2. Open items

### 2.1 Pin extension (ROOT approved; applied, not compiled, not mutation-tested)

The file is `nonlinear_integration/src/s11k_tests.rs`. The edits are additive:
- `token_indices`: tokens that begin with a letter match on an identifier boundary; `_`, `.` and `:` tokens match anywhere, as before.
- `body_in_impl` and `ADAPTER_IMPLS = ["impl AssemblyEvidence {", "impl SparseAssemblyEvidence {"]`.
- `option_c_structural_adapter_legacy_variants_reach_only_binary64_entry_points` blanks the four defining bodies per impl block.
- `kd5_nonlinear_sources_name_no_formation_check_entry_point` expects exactly 2 definitions of `solve_assembled_with_formation_check` in SA, one per impl, and allows the plumbing in either entry's body.
- `EXACT_ENTRY_POINTS` gains the pattern path's exact entries.
- `FORMATION_ENTRY_POINTS` gains the pattern formation plumbing.
- `solve_binary64` and `solve_structural_sparse_binary64` are checked exactly as before.
- The PP side is unchanged: one product call.

Pin (3), ROOT's behavioural requirement, is `k1_nonlinear_loop_reaches_neither_formation_entry` in `structural_adapter/k1_tests.rs`.

**Still to do:**
- Compile the pins.
- Run ROOT's three new mutants, all in `wip/mutate_k1.py.txt`: `K1-PIN-BINARY64`, `K1-PIN-LOOP` and `K1-PIN-THIRD`.
- Re-run the original pins' mutants and name each kill site: K-D5 `KD5-M32a`, `KD5-M32b` and `KD5-E4`, and S11-K `S11K-RV-OPT1`, `S11K-RV-OPT3`, `S11K-RV-OPT4` and `S11K-RV-PUB`, reproduced from their records. If any original mutant is no longer killed, stop and tell the manager.

### 2.2 Site table (manager and ROOT approved; applied as separate hunks)

The file is `frame_kernel/tests/s11_site_table.rs`, with four hunks against the base:
- **sparse.rs (K1): hunks 1 and 3.** Hunk 1, after the `exact_boundary.rs` Source (base line 51), is the `FK/structural/sparse.rs` Source. Hunk 3, before the `exact_boundary.rs` rows (base line 109), holds 12 rows: 9 with counts and 3 zero rows (`reactions`, `reduce_assembled_sparse_system`, `legacy_zero_product_fold`). These are `wip/site_table_sparse_hunks.patch`.
- **formation_check.rs (K-D5 gap, ROOT's separate item): hunks 2 and 4.** Hunk 2, after `load_case_algebra` (base line 79), is the Source. Hunk 4, at the table's end (base line 162), is one row: `pow2`, 1, integer exponent step. These are `wip/site_table_formation_check_hunks.patch`, **to land as a separate commit.** `git apply -R --check` succeeds on the current tree.
- By the port, the counts in `FK/structural.rs` and `SA` are identical to the base's, so no existing row changes.
- The port also finds that formation_check.rs's only site is `pow2`'s `k -= step`, an integer exponent step. It has no binary64 load, force or RHS fold, so there is no K-D5 finding.
- In sparse.rs, the `|rhs|` magnitude norm in `sparse_audit_contributions` has the same disposition as the dense `audit_contributions`: a perturbation-estimate norm, not a load sum. Tell the manager explicitly when reporting.
- Mutants: `K1-SITE-SPARSE` and `K1-SITE-FC`.

### 2.3 PP S11-F site-test KERNEL list (ROOT approved; pending, NOT applied)

- `kernel_list_s11f.patch` appends a `FK/structural/sparse.rs` Source to `KERNEL`, at the end so `KERNEL[4]` and `[6]` are unchanged. It adds 11 `FORCE_FUNCTIONS` rows for the sparse.rs functions that the port finds touching the force tokens. No existing entry changes.
- `git apply --check` succeeds. Apply it as its own hunk, a write-set extension into product_physics tests only.
- **Its required mutant is not yet written.** Proposal: add `let _ = self.source.force.len();` to `SparsePreparedSystem::free_dofs`. Rule 6 must then report an unlisted function.
- Without the patch, the unchanged test passes by the port: rules 2, 3 and 6 on `FK/structural.rs`, `SA` and `sparse_direct/structural.rs` are identical to the base's.

### 2.4 The C3-detect helper removed from sparse_direct

**It was my own draft, not pre-existing code.** The helper was `solve_sparse_structural_with_force_terms`. I added it in this slice and removed it before any commit, because `sparse_direct_factor_inherits_the_prepared_ledger_binding` requires that `sparse_direct/structural.rs` never contains "force". Nothing pre-existing was removed. SA's pattern `solve` calls the kernel's `prepare_sparse_structural_with_force_terms` and then sparse_direct's `solve_sparse_prepared`.

### 2.5 K2a interaction tests (drafted, not landed)

- The draft is `wip/k1_k2a_interaction.rs.txt`, intended as `frame_kernel/tests/k1_k2a_interaction.rs`. It uses K2a's `tests/k2a/rf_range_models.rs`, so it is valid only after K2a merges and main is merged into K1's branch.
- It tests three things:
  - RF-RANGE members formed or refused identically in both assemblies;
  - reach_zero and reach_lef refused with the same named error;
  - the same first failing element in both assemblies.
- **Finding:** RF-RANGE LEF-small never reaches assembly. The shared `FrameElement::new` refuses it first (`DegenerateAxis`, as K2a records), identically for both representations. LEF-large and the reach cases do reach assembly and are refused with `NumericalRange`.

### 2.6 Other notes

- **The formation check is not edited.** The sparse `Represented::formation_check` passes `formation_check::check` a `StructuralSystem` view whose stiffness is empty; `check` reads only `force` and `free_dofs`, and the factor through `solve`. Verify with the K-D5 parity tests.
- **Differences only for invalid input, and only on the sparse side:**
  - a contribution outside the pattern gives `InvalidInput("contribution outside pattern")`;
  - a spring or block node outside the model gives `InvalidNodeIndex`, where the dense `+=` would panic;
  - a stiffness on a different pattern is refused by SA's `check_pattern`;
  - a Binary64 binding on the sparse prepare gives `InvalidInput`, since the option-(c) fold is not offered on the pattern (F1b and T5).

## 3. F1b interface, as drafted (exact signatures in the files)

- **Pattern and values (`FK/structural/sparse.rs`, re-exported from `structural`):**
  - `SparsePattern::{from_positions(dimension, impl IntoIterator<(usize,usize)>), from_connectivity(node_count, &[(usize,usize)], diagonal: &[usize]), dimension, entry_count, lower_entry_count, row_range, row, find, column, transpose}`.
  - `SparseStiffness::{from_pattern_and_contributions(SparsePattern, &[StiffnessContribution]), from_contributions(dimension, &[..]), from_dense(&[Vec<f64>]), pattern, values, dimension, get, row, to_dense, storage_counts, multiply(&[f64]), reactions(&[f64], &AssembledForce)}`.
- **Assembly, per modulus basis:** `assemble_sparse_stiffness(node_count, frames: &[FrameElement], users: &[UserStiffnessElement], blocks: &[StiffnessBlock], springs: &[(usize, f64)], options: &SparseAssemblyOptions) -> Result<SparseStiffness, FrameKernelError>`.
  - `SparseAssemblyOptions` is `#[non_exhaustive]` with `new()`, so K2b adds b there without touching callers.
  - `StiffnessBlock { node_i, node_j, stiffness: Matrix12 }` is used for realized curved bends.
  - Order: frames, users, blocks, springs (the product's order). Formation is by the same `global_stiffness` calls, before any accumulation. `NonFiniteInput{"assembled stiffness"}` is raised after frames and users, as the dense assembly does.
- **Sparse `AssemblyEvidence` (`SA`):**
  - `SparseAssemblyEvidence::new(pattern: &SparsePattern, node_count, frames, users, curved: &[CurvedBendStiffnessElement], springs) -> Result<Self, StructuralError>`.
  - Methods: `with_force_terms`, `qualified_passive_family`, `geometry`, `pattern`, `contributions`, `absolute_roundoff`, `operation_counts`, `storage_counts() -> SparseEvidenceCounts { pattern_entries, contributions }` and `dense_symmetry_view() -> (Vec<Vec<f64>>, Vec<Vec<usize>>)`.
- **Pattern solve entries (`SA`), returning today's `StructuralSolution`:**
  - `solve_assembled(&self, k: &SparseStiffness, f: &AssembledForce, free, prescribed, mode)`.
  - `solve_assembled_with_formation_check(&self, k, f, free, prescribed, mode, curved_sources: &[CurvedBendMacroElement], selected: bool)`.
  - `pub(crate) solve(&self, k, f: &[f64], ...)`: C3-detect or legacy vector.
  - `SparseInteractive` runs the gate over the pattern. `DenseScrutiny` materializes the dense view (values, allowances and counts) and runs today's dense Cholesky path, bit for bit.
- **Kernel and sparse_direct entries:**
  - `prepare_sparse_structural`, `prepare_assembled_sparse_structural`, `prepare_formation_checked_sparse_structural`, `prepare_sparse_structural_with_force_terms`, `factor_sparse_structural_profile`, `finish_sparse_structural`, `audit_sparse_load_fidelity`, `sparse_negative_pair_witness`, `verify_sparse_negative_direction`.
  - `order_sparse_structural(&SparsePreparedSystem) -> SparseStructuralOrdering { order, first_columns, profile_entry_count, max_half_bandwidth }`.
  - `factor_sparse_structural_ldlt`, `solve_sparse_prepared`, `solve_sparse_structural`, `solve_assembled_sparse_structural`, `solve_formation_checked_sparse_structural`.
- **Reduction and partition maps:** `reduce_assembled_sparse_system(&SparseStiffness, &AssembledForce, boundary_dofs, displacements: Option<&[f64]>) -> Result<SparseReducedSystem { free_dofs, prescribed, force: ReducedForce }, FrameKernelError>`, with `free_position(dof)`. It matches `reduce_assembled_system*` bit for bit, errors included.
- **Reactions from sparse rows:** `SparseStiffness::reactions`, bit-identical to PP's `restrained_reactions` (E12). `multiply` is bit-identical to PP's `multiply_matrix_vector`, including the sign of zero. PP's legacy observation force (`legacy_observation_force`) stays in PP, an allow-listed lane.
- **Dense view:** `SparseStiffness::to_dense`, `SparseAssemblyEvidence::dense_symmetry_view`.
- **Storage counts:**
  - `SparseStorageCounts { dimension, stored_entries, lower_entries, dense_entries: u128 }`;
  - `SparseEvidenceCounts`;
  - `SparseStructuralOrdering::profile_entry_count`, computed before the factor allocates its rows;
  - `SparsePreparedSystem::entry_count`.
- **Explicit-zero rule:** an entry whose coalesced value is exactly 0 stays in the pattern, and the contribution audit, the residual and the witness see it. It takes no part in adjacency, ordering or profile, exactly like a zero of today's dense matrix. The rule is by value, on the prepared (scaled, symmetrized) matrix that the factor orders, so the order and the profile equal today's. Tests: `k1_adjacency_order_and_profile_equal_the_dense_derived_path` and `k1_explicit_zero_by_cancellation_takes_no_part_in_the_order`, the latter showing that a structural rule would change the order.

## 4. Planned build and test sequence

Use the brief's environment: `RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_INCREMENTAL=0`, `--offline --locked`, and a K1-own target. Check free disk before each step and hold cargo while a DEC-025 sweep runs.

1. **Compile check** (`wip/run_check.sh.txt`): `cargo check --tests` on frame_kernel, sparse_direct and nonlinear_integration. Fix compile errors. After any edit to structural.rs or SA, re-run `site_scan.py` and `pp_rules.py` to keep the counts and sets unchanged.
2. **Targeted tests:**
   - **frame_kernel `k1_`/`krev0`:**
     - pattern positions-only;
     - bitwise coalesced values, including two modulus bases;
     - assembly refusals;
     - fixed-order gate byte-identity (legacy, typed, C3);
     - the S11-K audit and KS1–KS3;
     - legacy zero-sign rows;
     - multiply, reactions and reduction;
     - KREV-02–05 on sparse;
     - witness parity and the pair-count test;
     - matrix references N07, R01–R06, NP-B and NP-D.
   - **sparse_direct `k1_`:**
     - adjacency, order and profile parity;
     - explicit zero by cancellation;
     - report byte-identity against today's dense-derived sparse path (legacy, C3, typed, formation-checked; free ascending and permuted);
     - KREV-02;
     - RF-LARGE storage counts: chain and comb, AX and ROT, n = 10, 100, 1,000, 10,000; printed with `--nocapture` and recorded.
   - **nonlinear_integration `k1_`:**
     - evidence parity on the 13 KD5 models (real bends);
     - pattern entries byte-identical in both modes;
     - K-D5 parity: 122 and CSKEW_8_5 demote, E1 and E6 do not, an unavailable seed, not-selected;
     - C3-detect;
     - N01–N09 classes and values;
     - T0R M05-T, M05-R1 (point loads; perm, rot30, translated) and M05-SPRING reactions from sparse rows;
     - relabelling;
     - KREV-01;
     - the loop pin (3).
   - Then the existing pins, `s11_site_table` and PP `s11f_site_test`.
3. **Suites:** frame_kernel, sparse_direct and nonlinear_integration in full, plus every crate the caller scan reaches (product_physics compiles against SA and FK).
4. **T9:** S11-K's harness `fixdiff_main.rs` (sha256 `ec089c1d…`, in `S11K/_run_records/fixture_diff/`), built against a `git archive` of main `134eefc24` and against K1. Every committed JSON request or model under `P/fixtures`, `P/validation` and `P/core`, in both modes. Expected: all byte-identical. **Any committed-byte change stops the work,** and is reported with its site before anything is regenerated.
5. **Mutations** (`wip/mutate_k1.py.txt`): one clean copy and one clean target per mutant, with a `NONE` control first, as in K-D5's runner `run_mutants_clean.sh`. The 19 mutants (18 patch mutants plus the K-D5 E4 evasion, which the script handles separately):
   - K1's own: `K1-M8` (a spring omitted, sparse only), `K1-ORDER`, `K1-KFC`, `K1-LABEL`, `K1-DENSEPAIRS`, `K1-COALESCE` and `K1-COALESCE-SCATTER`;
   - the site-table mutants: `K1-SITE-SPARSE` and `K1-SITE-FC`;
   - the pin mutants: `K1-PIN-BINARY64`, `K1-PIN-THIRD` and `K1-PIN-LOOP`;
   - the originals re-run: `KD5-M32a`, `KD5-M32b`, `KD5-E4`, `S11K-RV-OPT1`, `S11K-RV-OPT3`, `S11K-RV-OPT4` and `S11K-RV-PUB`;
   - plus the S11-F KERNEL-list mutant once that patch is applied.

   All anchors match once on the current tree. If `K1-ORDER` survives, add a node with three or more skew members, where the accumulation order changes bits, and assert that precondition.
6. **After K2a merges:** merge main in, land `k1_k2a_interaction.rs`, and re-run 1–5 on the combined tree.
7. **Records:** `CHANGE_RECORD.md` and `RETURN.md`, with `_run_records/` (`callers.txt` regenerated after compile), SHA256SUMS and placeholders only. The gate is not run, per ROOT's call. **No timing or memory claims;** K6 owns them.
