# RV8: independent complete-diff review of slice K1

**Verdict: PASS.** There are no BLOCKING findings. There are 3 SHOULD-FIX findings and 7 NOTEs.
- Every SHOULD-FIX is a **test gap**: a mutant of mine survives K1's whole frame_kernel, sparse_direct and nonlinear_integration suites, but it diverges on admissible product-shaped inputs.
- No **parity failure** was found. The candidate's code agrees bit for bit with the dense assembly and gate in 45,686 checks. Its dense behaviour is byte-identical to the base.

## Reviewer, brief and delegation

- **Reviewer:** RV8, a Type 2 TASK dispatched directly by ROOT (HELP_HUMAN) and run as a background subagent of ROOT's session on the owner's Mac.
  - The brief is `TASK_BRIEFS/RV8_K1_REVIEW.md`. It was spawned as RV7 and renumbered by ROOT at `a90e7699b`; the content is unchanged. The working paths keep their spawn names (`<wt>/rv7-target`, `<scratch>` = `<wt>/scratch/rv7`).
  - I did not draft, implement or test K1, and I did not design W3 or check its design. I made no Git writes and did not delegate.
- **Read:**
  - Root `AGENTS.md`, `agents/AGENT_TASK.md`, `_COMMON.md`, `I8R_K1_RESUME.md` ("The Mac host") and the brief's Basis list:
    - DESIGN.md 5a.2 (`fb62ef4a…`, hash verified): §4.8, the §6 rows, §7.1 and §7.3;
    - `I8_K1_IMPLEMENTATION.md` with addenda 1–4;
    - `ROOT_RULINGS_V1.md`: S11-K option (c), the K-D5 sections, the F1 split, K2a corrections 1–3, and the four K1 sections;
    - `PLATFORM_CALIBRATION_MAC/RECORD.md` and `suites/SUMMARY.md`;
    - the candidate's `IMPLEMENTATION/K1/` (CHANGE_RECORD, RETURN, WIP_STATE, `_run_records/`, `wip/`).
  - The complete diff `134eefc24..43f9e6a78`, commit by commit, and the dense code each sparse stage mirrors.

## Revisions reviewed

- **Candidate:** `codex/piping-k1-20260928` at `43f9e6a78` (tree `3048ebed0`). It has four commits on the base main `134eefc24`, which is the merge base:
  - (a) `826a9eed4`, the K1 slice;
  - (b) `85626dbe0`, the `formation_check.rs` site-table rows;
  - (c) `4319854dc`, the KERNEL-list hunk;
  - (d) `43f9e6a78`, the records.
- **Reshape claims, verified:**
  - The head's whole tree equals the pre-reshape snapshot `3513fd8ab`'s (both are `3048ebed0`).
  - `git diff 19925122b 43f9e6a78` touches only `T3/IMPLEMENTATION/K1/**`, so the code tree equals the tested tree `19925122b`.
  - Commits (a)–(c) touch only the 10 files of RETURN §2. Commit (d) touches only `T3/IMPLEMENTATION/K1/**`.
- **The WIP artefacts** in (d) are blob-identical to `d08b0efc7`: `WIP_STATE.md`, `kernel_list_s11f.patch` and all nine `wip/` files.
- **Everything built** here came from `git archive` copies in `<scratch>`. Nothing was built or written in `<wt>/k1`.

## Findings

| ID | Severity | Site | Evidence | Resolution |
|---|---|---|---|---|
| RV8-1 | SHOULD-FIX | Test gap: K1's bitwise-K parity tests do not pin the **accumulation order of blocks** (realized bends). `sparse.rs:496` is correct today. | Mutant `RV8-BLOCK-ORDER` (`for block in blocks.iter().rev()`) survives FK, SD and NI in full. No K1 model has two blocks at one node with a third contribution; K1-ORDER reverses only frames and users. My parity probe kills it on a tee at a bend–bend junction (two realized bends and a straight branch at one node): K bits, `from_pattern_and_contributions`, multiply, reactions, and every gate entry in both modes (42 named mismatches), plus 9,651 on generated models. | Add a junction case (two realized bends and a third element at one node) to `k1_coalesced_values_…` and the SA byte-identity tests, and show RV8-BLOCK-ORDER killed. F1b must pass PP's curved order (see RV8-N7). |
| RV8-2 | SHOULD-FIX | Test gap: the **spring-against-block order** on a shared diagonal is not discriminated (`sparse.rs:496–508`). | Mutant `RV8-SPRING-FIRST` (springs added before blocks) survives all three suites. FK's tree has springs on a block node (DOF 43), but the values happen to round the same. The probe kills it on a realized bend with support springs at both ends (magnitudes 4.4e7–9.7e8, both flexibility factors; 44 named mismatches: K bits, multiply, reactions), plus 10,809 on generated models. | Add a bend-end support-spring case whose spring and bend terms round differently in the two orders (for example `bend-supports-f1-k5` in `parity/main.rs.txt`) to the bitwise-K test, and show RV8-SPRING-FIRST killed. |
| RV8-3 | SHOULD-FIX | Test gap: the **formation check's load input** in the sparse representation is not discriminated (`sparse.rs:970–994` passes `force_terms`, which is correct today). | Mutant `RV8-FC-TERMS` (the sparse check drops the ledger terms and uses the folded force) survives all three suites. `k1_kd5_parity_cases_…` uses one ledger term per load, so the terms' exact net equals the rounded force. The probe kills it on K-D5's models with each load split into three terms whose exact sum is not the rounded net: 11 mismatches in `SparseInteractive` on F122, CSKEW_8_5, CSKEW_30_N122, CPLANAR_60, PROBE_C, CSKEW_30_RADIUS_MISMATCH and BENDING_SOFT, where the `FormationCheck` records differ. This is the brief's "the same inputs" requirement for ρ. | Add a split-ledger case to the K-D5 parity test, in `SparseInteractive`, on a demoting model, and show RV8-FC-TERMS killed. |
| RV8-N1 | NOTE | Records: CHANGE_RECORD ("Candidate verified: `19925122b`"), RETURN (§1, §2, §6(j), §8, §12) and `kernel_list_s11f.patch.NOTE.md` cite pre-reshape commits (`19925122b`, `9d4ba0e17`, `d08b0efc7`). | These commits are reachable only through `codex/piping-k1-wip-20260928`. The mapping to the reshaped commits is only in commit (a)'s message. | Keep the WIP branch, or add one line mapping `19925122b` to `4319854dc` (the same code tree). |
| RV8-N2 | NOTE | RETURN §16 | The sanitization bullet appears twice. It says the stripping made `git diff --check` clean. On the records commit, `git diff --check` still reports 3 trailing-space lines, all in the WIP `.patch` files (patch-syntax context lines, kept byte-identical). This is disclosed in commit (d)'s message, not in RETURN. | Deduplicate, and state the `.patch` exception in RETURN §16. |
| RV8-N3 | NOTE | `sparse_direct/structural.rs:56–75` | `order_sparse_structural` builds `SymmetricProfileMatrix::from_entries_with_order`, which allocates and fills a profile-sized `values` vector (`lib.rs:249`) before the counts are returned. RETURN §12's "counts before the factor allocates" is literally true, but one profile-sized allocation has already happened. | For F1b's profile-ceiling guard: compute the counts from the skyline alone, or accept one profile-sized allocation. |
| RV8-N4 | NOTE | Pin extension (`s11k_tests.rs`) | Of my five evasions, a `use … as` alias inside `solve_binary64` and a route through a new sibling module both pass every text pin. They are caught only by the behavioural pins. This is the same as at the base, and is the documented limit. The identifier-boundary rule no longer flags an identifier that merely ends in an exact-entry name; that was a coincidental catch before. No loss of reach was found (§5). | None required. |
| RV8-N5 | NOTE | RETURN §6 reviewer items (a)–(j) | Each corrects a vacuous or wrong fixture, or strengthens an assertion. None weakens a required test (§7). | None. |
| RV8-N6 | NOTE | Invalid-input differences (WIP_STATE §2.6; RETURN §3.8) | All four are acceptable. None is reachable from a product path today, or through SA after F1b (§11). | None. |
| RV8-N7 | NOTE | F1b interface (RETURN §12) | The signatures are exact at the head. §12 omits `SparsePositiveFactor::pivots()`. Nothing maps SA's `CurvedBendStiffnessElement` slots to `StiffnessBlock`s, so F1b must keep PP's curved order itself (see RV8-1). | Optional: a crate-local helper in SA or in F1b. |

## 1. Complete-diff review and callers

**Read, commit by commit:** `FK/structural.rs` (+417 −165), `sparse.rs` (1830), `sparse/tests.rs` (1485), sparse_direct `structural.rs` (+99 −2) and `k1_tests.rs` (811), `structural_adapter.rs` (+784 −282), SA `k1_tests.rs` (1020), `s11k_tests.rs` (+86 −20), `s11_site_table.rs` (+20 in (a), +8 in (b)), and `s11f_site_test.rs` (+62 in (c)). The line counts and 16-hex sha256 prefixes of RETURN §2 match the head.

**The dense refactor** (`FK/structural.rs`) is behaviour-preserving by reading:
- `evaluate_original_residual_bound`, `audit_intended_action` and `finish_checked_factor` now iterate `Represented::row`/`sums_row`. The dense implementations yield the full row, zeros included, in ascending column order, and every stage skips a zero coefficient or an empty expansion exactly as before.
- `audit_load_row` takes the same `parts` list, built by the same expression, now outside the function.
- `exact_scaled_rhs` receives the same (K_ic, g_c) sequence in `prescribed` order.
- `estimate_rcond`'s body is moved unchanged. The dense `column(j)` is `a[i][j]` over i.
- `ProfileFactor::factor_structural_profile` is the old loop verbatim, with `free_dofs[order[i]]`.
- **SA:** `AssemblyEvidence::new` through `EvidenceParts` with `DenseEvidenceStore` performs the same additions in the same order. The allowance pass is row-major, and the error is the same.

**Independent caller list** (my own lexer, `callers/rv8_callers.py.txt`; comments and literals blanked; test context from `#[cfg(test)]` module declarations, inline test items and `tests/` directories). It scanned every `.rs` file under `projects/chirality-piping` at the base and at the head, for 64 names: the named dense and sparse entries, every refactored stage, and the new FK, SD and SA types and entries.
- **Outside FK/structural\*, SD/structural.rs and SA,** the non-test uses are identical at the base and the head (`callers/compare_with_i8r_callers.txt`):
  - PP `lib.rs:4418/4441`, `solve_preview_reduced_system`: `AssemblyEvidence::new` and `.solve_assembled_with_formation_check(`. This is the one product call, on the dense `AssemblyEvidence`.
  - PP `source_recovery.rs:879`, `prepare_sources`: `AssemblyEvidence::new`.
  - NI `lib.rs`: `AssemblyEvidence::new` (:600), `.solve_binary64(` (:1990), `solve_structural_dense_binary64(` (:2008) and `solve_structural_sparse_binary64(` (:2013).
  - No non-test use of any new name (`Sparse*`, `assemble_sparse_stiffness`, `reduce_assembled_sparse_system`, the pattern solves and prepares) exists outside the three defining modules. No example, bench or other crate names one.
- **Against I8R's `callers.txt`** (non-test rows), for the 21 shared names:
  - identical `file:line` sets for 18;
  - for `solve_sparse_prepared`, `sparse_negative_pair_witness` and `audit_sparse_load_fidelity`, the only extra rows in mine are `use` and re-export lines, which my scan counts.

## 2. Dense behaviour unchanged: the differential

**Probe** (`dense_differential/dense_diff.rs.txt`): one source, built unchanged against `git archive` copies of the base and of the head. It calls only base APIs and prints `Debug` of every result.
- **Kernel references**, each through:
  - `solve_structural_dense`, `_binary64`, `_with_force_terms` (plain ledger and a (G, v, −G) ledger at G = 1e12);
  - `solve_assembled_structural_dense`, `solve_formation_checked_structural_dense`;
  - sparse_direct's `solve_structural_sparse`, and SA's `solve_structural_sparse_binary64`;
  - the prepared systems (`Debug` of `PreparedSystem`), `negative_pair_witness`, `verify_negative_direction` (16+ directions), `estimate_rcond`, Cholesky, today's `factor_structural_ldlt` and a reversed full-profile `factor_structural_profile`;
  - `evaluate_original_residual`, `evaluate_assembled_original_residual` and `audit_load_fidelity`.
- **The cases:**
  - N07, R01, R02 (K_used with K_original's contribution), R03 (lost load, including C3), R04 (with and without contributions, and all −0.0), R05 (omitted spring), R05-ok and R06;
  - NP-B (n = 8, 64, 80; c = 0.75 and 0.750000000001);
  - NP-D (skew, NaN, +∞, duplicate cancellation, overflow, underflow, lift-off);
  - NP-A (N05's stored matrix);
  - signed-zero, cancellation and subnormal right-hand-side cases;
  - indefinite 3×3 systems, and a 5-DOF system with ascending and permuted free and prescribed lists;
  - a 6-member settled skew chain with SA's real evidence and a formation source (available, unavailable, reversed free list, bare).
- **Product-shaped cases through SA's `AssemblyEvidence`:**
  - all 13 of K-D5's models (realized bends), each at E and 0.9E (two modulus bases);
  - each with a settlement, a spring on a prescribed DOF, two springs on one free DOF and a zero spring;
  - an E6-based mixed model: a star node joined by four skew members, a duplicate frame, two user elements (one duplicating a member), an explicit untraced slot, repeated springs and a nonzero prescribed value;
  - each through `solve_assembled`, the formation-checked entry (selected, unmatched and not selected) and `solve_binary64` in both modes, plus `Debug` of the evidence itself (the n×n allowances and counts).
- **Result:**
  - 2,713 lines and 58.9 MB, byte-identical: sha256 `a96f5eae…` for both (`dense_differential/SUMMARY.txt`);
  - 79 cases, with 1,200 Passed, 232 Sensitive, 159 formation records (62 `Estimate`, 97 unavailable), and refusals of every class (unresolved, Range, InvalidInput, Asymmetric, NegativeEnergy).
- **Product level:** `validation/benchmarks/numerical_integrity`'s observer (N01–N09 through PP in both modes, plus the NP-A, N07 and NP-D observations), built `--release` from each archive. 38 lines, byte-identical: sha256 `8788e3b3…`.
- **T9** was not re-run by me. Its records are consistent: base equals candidate, and base equals ROOT's Mac main hashes (`<wt>/scratch/calib/fixdiff/sha_native.txt`) on 112 of 112, by my own cross-check.

## 3. Parity attacks

**Probe** (`parity/main.rs.txt`, candidate only). Every comparison is bitwise, or of `Debug` bytes, against the dense form on the same inputs.
- **Checks:**
  - K bits, both from `assemble_sparse_stiffness` and from SA's contribution list (`from_contributions`, `from_pattern_and_contributions`);
  - that every entry outside the pattern is +0.0; `from_dense` round trips;
  - `reduce_assembled_sparse_system` against both dense reductions;
  - `multiply` and `reactions` against PP's `multiply_matrix_vector` and `restrained_reactions`, with ±0.0 in u;
  - the SA evidence (allowances, counts, contributions);
  - `solve_assembled` and the formation-checked entry (matched and unmatched), in both modes, for plain and cancelling ledgers;
  - the legacy pattern path, C3-detect, typed and bare;
  - the RCM order and profile against the dense-derived path;
  - the witness and three `verify` directions.
- **Models:**
  - 800 generated product-shaped models: 400 seeds at E = 2.0e11 and 1.93e11, a third of them on an axis-aligned grid. They include:

    | Property | Models |
    |---|---|
    | Skew members and a star node with four or more elements | 772 |
    | Duplicate and reversed-duplicate connectivity | 782 |
    | Frames, users and explicit blocks together | 720 |
    | Traced 90° bends | 430 |
    | Blocks that negate a duplicated frame (exact cancellation) | 345 |
    | Blocks with −0.0 entries | 586 |
    | −0.0 springs | 526 |
    | Springs on prescribed DOFs | 804 |
    | Nonzero, zero and −0.0 prescribed values | all |
    | Stored explicit zeros | 844 |
    | Non-identity RCM order | 1,210 |

    222 of the runs repeat with the free list reversed and the prescribed list rotated.
  - A **zero on one basis only:** a fixed block equal to −K(E = 2e11) of a member re-formed per basis. It gives 70 stored zeros at 2e11 and 20 at 1.93e11, and a different order per basis.
  - A 300-member skew chain and a 300-member comb with 20 springs (order and profile at scale; the dense witness is skipped above 200 DOFs).
  - 300 random indefinite 3–12-DOF systems, a third with a reversed free list (283 witnesses).
  - K-D5's 13 models at two bases with three-term split ledgers.
  - An S-bend, a bend with support springs at both ends (8 spring magnitudes, two flexibility factors), and a tee at a bend–bend junction.
- **Outcomes** (`parity/parity_candidate.txt`):
  - `SparseInteractive` plain: 599 Passed, 2 Sensitive, 37 unresolved, 3 NegativeEnergy and 407 Asymmetric. The Asymmetric cases are explicit untraced skew blocks, which carry no formation allowance.
  - The formation-checked entry: 561 Sensitive with a record, 38 Passed.
- **Result: 45,686 checks, 0 mismatches.** No bitwise, order, profile, witness, reduction, multiply or reaction divergence.

**By reading:**
- The coalescing adds, for every entry, the same values in the same order as the dense `+=` sequence: frames, users, blocks (`scatter_block` has the same local loops as `assemble_element_contribution` and PP's `add_curved_bend_stiffness_contributions`), then springs.
- **The pattern path's adjacency and profile** equal the dense-derived path's. Both skip exact zeros, and the prepared matrix is exactly symmetric (±0 aside, which `!=` treats as equal).
- **The witness:** an unstored pair's energy is a_ii + a_jj > 0 from original positive diagonals, so skipping unstored pairs returns the dense search's first witness.
- **`multiply`'s zero-sign rule** is right. Inserting ±0 terms changes only the sign of a zero sum, as a set.

## 4. The O(nnz) claims

- **By reading, every stage visits pattern entries only.** `sparse_audit_contributions` has expansions per entry, prepared-row columns and row couplings in `prescribed` order. The residual, intended action and load audit use `row`/`sums_row`. The rcond norm reads prepared rows. `validate_sparse(_prepared)` is O(nnz). The witness and `verify` visit stored pairs only.
- **There is no n×n allocation on the pattern path:**
  - `vec![vec!` occurs only in `to_dense`, and `from_dense` takes a dense input;
  - SA's `SparseInteractive` path uses `SparseEvidenceStore`, and `dense_symmetry_view`/`to_dense` appear only in `DenseScrutiny`;
  - the formation check allocates O(n) plus per-element 12×12 matrices.
- **The pair-count test is not vacuous.** On the 40-member chain (240 free DOFs), visited = stored lower pairs, and the assertion is `visited × 10 < 28,680`. `K1-DENSEPAIRS` is killed there (re-killed, :1214).
- **The storage counts at 10,000 members** go through the pattern path only (`large()` uses `assemble_sparse_stiffness`, `prepare_sparse_structural` and `order_sparse_structural`). My re-run reproduces RETURN §11's table exactly (`storage_counts_rv8.txt`). I allocated nothing dense at 10,000 members.

## 5. The pin extension, against the pins' original intent

- **The intent:** the nonlinear loop's closed-gap solves stay on the named binary64 path and never reach an exact or formation-checked entry. The extension is additive:
  - blanking by impl block, with two definitions of the formation entry (one per adapter impl) and a third failing;
  - identifier-boundary tokens;
  - the pattern entries added to both forbidden lists;
  - the product side unchanged (one call, inside `solve_preview_reduced_system`);
  - ROOT's behavioural pin (3), `k1_nonlinear_loop_reaches_neither_formation_entry`. Its precondition is that both formation entries demote F122; the loop's first iteration is Passed and bit-equal to `solve_binary64`.
- **ROOT's three required mutants, re-killed:**
  - `K1-PIN-BINARY64`: :566, plus the behavioural unsupported_gap tests (lib.rs:4781 and :4827);
  - `K1-PIN-LOOP`: :519 and :1130, and the loop errors;
  - `K1-PIN-THIRD`: :1130 (3 ≠ 2).
  - Also `K1-PIN-BINARY64-B` (:584) and `K1-PIN-LOOP-B` (pin (3) at k1_tests.rs:1000).
- **Original kill sets kept:**
  - `KD5-E4` fails exactly its original two tests (:1477 and :1130; originally :1411 and :1071);
  - `S11K-RV-OPT4` fails its original five, plus two;
  - `S11K-RV-PUB` fails its original FK test (:660), plus NI's unsupported_gap ×2;
  - M32a and M32b were not re-run by me. I8R's table shows supersets of `KD5/_run_records/repair/mutations/MUTANTS.txt`.
- **Evasions** (`mutations/rv8_mutants.py.txt`; NI in full):

  | Evasion | Change | Text pins | Behavioural |
  |---|---|---|---|
  | Alias | `use …::prepare_structural as legacy_prepare;` called in `solve_binary64`, with the required legacy text kept in dead code | **missed** | caught: `option_c_closed_gap_…` precondition :796, `kd5_…unit_force_…` :1457, unsupported_gap ×2 |
  | String | the required text only in a string literal, and the exact prepare called | caught: :566 (the lexer blanks the literal) | caught |
  | New impl | a second `impl AssemblyEvidence` block, whose `solve_binary64_v2` calls `self.solve`; the loop calls it, with the legacy call in dead code | caught: :584 (the new block is not blanked) | caught: :823, :913, unsupported_gap ×2 |
  | Sibling module | new `bridge.rs` runs K1's exact pattern solve (`solve_sparse_structural`); the loop calls `bridge::run`, with the legacy call in dead code | **missed** | caught: :823, :913, :1272, pin (3) :1009, unsupported_gap ×2 |
  | UFCS | the loop calls `SparseAssemblyEvidence::solve_assembled_with_formation_check(&sev, …)` | caught: :1130 | caught |

- **No evasion escapes both.** The two text misses are the pins' documented limit, and the same at the base (RV8-N4).

## 6. The site table and the KERNEL list

- **ROOT's five conditions hold:**
  - (b) and (c) are separate commits, and each only adds lines;
  - no existing row, count or disposition changes;
  - every sparse.rs site has a disposition, and its load, force and RHS sums are `ExactAccumulator` (`reactions`, `reduce_assembled_sparse_system` and `legacy_zero_product_fold` at count 0);
  - both extensions are declared in CHANGE_RECORD and RETURN;
  - no stop condition triggered.
- **Re-killed:**
  - `K1-SITE-SPARSE` and `K1-SITE-FC` at `s11_site_table.rs:524` (`multiply` table 1, source 2);
  - `K1-S11F-KERNEL` at `s11f_site_test.rs:1194`. That run needs the full piping tree: a core-only archive fails to compile PP's `include_str!` of a fixture.
- **The `|rhs|` norm is a perturbation-estimate norm.** In `sparse_audit_contributions`, `rhs_norm = Σ|rhs_r|` serves only as the denominator of `assembly_load_perturbation_estimate`, and in the exact-zero test `delta_rhs_norm ≠ 0 ∧ rhs_norm = 0`. A sum of non-negative terms is zero exactly when every term is. It is never a published load, has no cancellation, and carries the dense `audit_contributions` row's disposition. I agree with I8's claim.
- **`formation_check.rs` has no binary64 load, force or RHS fold.** By reading, ρ starts from the ledger terms (`term.accumulate`) or the force (`rows[r].add`) in one `ExactAccumulator` per free row, and the element terms enter through exact splits. The only scanner site is `pow2`'s integer `k -= step`. The `Wide2` sums in `quad` are stiffness re-formation at p = 128.

## 7. RETURN §6 reviewer items (the checkpoint-A fixture changes)

- **(a)** A type annotation.
- **(b) The formation allowances.** They correct vacuous comparisons: without them the skew chains are refused as `Asymmetric` in both representations, so no gate stage is compared. The replicated form matches SA's builder, and the SA tests use real evidence.
- **(c) The bare checks.** A strengthening: parity with no evidence, and `Ok` is now asserted on the typed and C3 solves.
- **(d) The assembly-refusal fixture.** It corrects a wrong fixture. E = 1e308 overflowed at formation, so the drafted test exercised formation, not "assembled stiffness". The new fixture reaches the named assembly refusal, and the test now asserts the dense error's name.
- **(e) Witness case 2.** It corrects a vacuous fixture. The drafted matrix's pair energy for (2, 0) is exactly 0 after scaling, so neither search finds a witness. The new case gives −0.5, across the unstored pairs.
- **(f) C3-detect at G = 1e12 on an unsettled chain.** It corrects a vacuous fixture. At G = 1e8 the loss (about 3e-9 N) is below the audit target, so the parity was compared only on an unflagged row. The KS1–KS3 part keeps a nonzero settlement.
- **(g) The O(n⁴) dense cross-check on a 6-member chain.** Not a weakening: the O(nnz) count assertion stays on the 40-member chain, and witness parity on indefinite systems is covered by `k1_negative_witness_matches_…` (and by my 300 indefinite systems).
- **(h), (i), (j)** Readability; a corrected over-strict inequality (explicit zeros can lie outside the skyline); and the formatting of the applied KERNEL hunk.
- **None weakens a required test** (RV8-N5).

## 8. The mutation table

- **Re-killed in my own clean archives** (`mutations/kills.txt`, `mutations/logs/`). NONE ran first and was clean (163 + 30 + 98). Every sampled kill site equals RETURN §10's:

  | Mutant | Kill sites |
  |---|---|
  | `K1-M8` | 10 sites: FK :377, :576, :875; SD :566; NI :261 and others |
  | `K1-ORDER` | FK :377, :576, :875; SD :566 |
  | `K1-KFC` | FK :568 ×4; SD :566, :690 |
  | `K1-DENSEPAIRS` | FK :1214 |
  | `K1-LABEL-ORDER` | SD :422, :481, :566; NI :323, :350, :436, :514 ×3 |
  | The pin mutants and originals | as §5 |
  | `K1-SITE-SPARSE`, `K1-SITE-FC`, `K1-S11F-KERNEL` | as §6 |

- **Truthful statement of K1-LABEL.** RETURN §10 and CHANGE_RECORD state it as ROOT ruled: K1-LABEL is "killed only by a production invariant … not demonstrated", and K1-LABEL-ORDER is mutation 10's demonstrated form. The relabel test's 1e-9 agreement does not catch it; order parity and byte identity do.
- **No mutant is killed only by a source-text pin where a behavioural test was expected.** Those killed by text alone are killed by design: `K1-PIN-THIRD` (a definition), the site-table mutants and the KERNEL-list mutant.
- **My own mutants:**

  | Mutant | Change | K1's suites | RV8 probe |
  |---|---|---|---|
  | `RV8-BLOCK-VALUE` | a curved-block-only value error: each block's [0][0] × (1 + 2⁻⁵²) | killed: FK :377, :576; SD :566; NI :261 | — |
  | `RV8-BLOCK-ORDER` | blocks in reverse order | **survives** | killed (RV8-1) |
  | `RV8-SPRING-FIRST` | springs before blocks | **survives** | killed (RV8-2) |
  | `RV8-FC-TERMS` | the sparse formation check ignores the ledger terms | **survives** | killed, 11 (RV8-3) |
  | `RV8-EV-*` | five pin evasions | §5 | — |

## 9. The F1b interface

The signatures at the head equal RETURN §12, item for item (extracted from the source), except that `SparsePositiveFactor::pivots()` exists but is not listed. Against the brief's list:

| The brief's item | At the head |
|---|---|
| Pattern builder per modulus basis | `assemble_sparse_stiffness`, once per basis; `SparsePattern::from_connectivity` |
| Sparse `AssemblyEvidence` | `SparseAssemblyEvidence::new(pattern, …)`, `with_force_terms`, the formation primitives (internal), `storage_counts` |
| Plain and formation-checked pattern solves | Both return today's `StructuralSolution` |
| Reduction with prescribed displacements, and partition maps | `reduce_assembled_sparse_system` with `free_position` |
| Reactions from sparse rows | `SparseStiffness::reactions`, bit-equal to PP's `restrained_reactions` in my probe |
| Dense view | `to_dense`, `dense_symmetry_view` |
| Storage counts | `SparseStorageCounts`, `SparseEvidenceCounts`, `profile_entry_count` (see RV8-N3), `entry_count` |
| Options struct for K2b's b | `#[non_exhaustive] SparseAssemblyOptions` with `new()`/`Default`; no caller can build it with a literal |

The one F1b hazard is RV8-N7 (the curved-block order).

## 10. Records and hygiene

- **SHA256SUMS:** all 122 entries verify and cover every file in `IMPLEMENTATION/K1/`.
- **GEN-8 passes on the head** (1 passed, 10 deselected; `gen8.txt`).
  - It was run read-only in `<wt>/k1` (at `43f9e6a78`, clean), with `GIT_OPTIONAL_LOCKS=0`, `PYTHONDONTWRITEBYTECODE=1` and `-p no:cacheprovider`. Nothing in the worktree or its git directory changed: same index mtime, size and hash, and no newer file.
  - GEN-8 needs a git working tree, so an archive copy cannot run it.
- **No machine paths and no model identifiers** in `IMPLEMENTATION/K1/**` (grep).
- **T9 is stated as Mac-only** in RETURN §1 and §8, CHANGE_RECORD and `t9_summary.txt`.
- **The suites claim is exact** (`suites_compare.txt`): 0 changed and 0 removed; 28 added and passing; the only failures are the three Mac-main platform tests, with byte-identical failure blocks.
  - My own FK, SD and NI runs on the head give 163, 30 and 98, all passing.
- **The sanitization is disclosed:** 600-character cuts with sha256, placeholders, and trailing whitespace stripped at commit, with the `.patch` exception (RV8-N2).
- **"Gate not run"** cites ROOT's ruling (RETURN §14).
- **"Not done" is honest** (§15): the K2a interaction tests and the combined re-run; no timing claims; the untouched out-of-scope files; no K2b, K5, K6 or F1b guard work.

## 11. The invalid-input differences

| Difference | Why it is acceptable |
|---|---|
| `contribution outside pattern` (`InvalidInput`) | Dense would record a rounding difference and refuse through the audit (the R05 analogue). Through SA it is unreachable: `SparseAssemblyEvidence::new` refuses an entry outside the given pattern, and `check_pattern` requires the same pattern for K. |
| A block or spring node outside the model (`InvalidNodeIndex`) | Dense `+=` would panic. PP validates the nodes upstream, so it is unreachable, and graceful where it is not. |
| A stiffness on another pattern | A new API guard only. |
| A binary64 binding on the sparse prepare | Unreachable from any public sparse entry: the bindings are Legacy, Assembled, AuditTerms or formation. It is defensive. |

None of the four is reachable from a product path today, and none through SA after F1b.

## What I ran

All runs were on `aarch64-apple-darwin` with rustc and cargo 1.97.1 (`toolchain.txt`), `CARGO_INCREMENTAL=0`, `--offline --locked`, `-j 8` for single jobs or `-j 4` for mutants (at most three at once), and `RUST_TEST_THREADS=4`. The targets were under `<wt>/rv7-target`, and the mutant targets were deleted after each run. The memory guard never fired (`<wt>/guard/memguard.log` shows only its start lines).
- **Suites on the head:** FK, SD and NI in full.
- **Differential:**
  - the dense-diff probe (debug), base and head;
  - the numerical_integrity observer (release), base and head.
- **Parity:**
  - the parity probe (release) on the head, 45,686 checks;
  - the same probe against the three surviving mutants.
- **Mutations:** 26 runs (`mutations/logs/`): NONE; 13 of I8R's K1 and pin mutants; 3 originals (KD5-E4, S11K-RV-OPT4, S11K-RV-PUB); and 9 of my own (4 value and order mutants, 5 evasions). K1-S11F-KERNEL ran in PP, `--test s11f_site_test`.
- **Other:**
  - the storage-count test with `--nocapture`;
  - GEN-8;
  - my caller scan on the base and the head;
  - SHA256SUMS and path, model-identifier and whitespace scans.

## What I did not check

- **T9:** not re-run. Only its records were cross-checked (§2).
- **The 39-manifest suites:** not re-run beyond FK, SD and NI. I relied on `suites_compare.txt`.
- **Hosted CI (Linux)** and the DEC-025 sweep are outside my run.
- **Not re-run:** the both-entry gate (ROOT: not run for K1), and K-D5's M32a and M32b.
- **K2a interaction:** not reviewed; the draft `wip/k1_k2a_interaction.rs.txt` was only read. That is the delta check after ROOT merges main with K2a.
- **The error paths of `reduce_assembled_sparse_system`** were compared by reading only. The probe compared the Ok paths.
- **No timing or memory measurement** (K6).
