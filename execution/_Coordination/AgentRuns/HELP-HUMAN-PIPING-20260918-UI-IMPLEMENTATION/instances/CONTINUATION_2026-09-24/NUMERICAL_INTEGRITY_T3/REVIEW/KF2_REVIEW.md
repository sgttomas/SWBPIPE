# RV24: independent review of slice KF2 (K6's N10: the dense negative-pair witness in O(n²), every result unchanged)

- **Reviewer:** RV24 (Type 2 TASK, independent reviewer). I did not write KF2. I used I20's tests only as suites to re-run and as mutation targets, never as my oracles.
- **PR:** [#1060](https://github.com/sgttomas/chirality/pull/1060), branch `codex/piping-kf2-20260930`.
- **Head reviewed:** `f2b8c85a2` (`f2b8c85a2d40d9b86e14a1b8b90274444d01a4e0`). Base main `78f55f927` (`78f55f927db47c7f44299fd32793d4b64e8b3572`). The implementer was I20.
- **Date:** 2026-09-29 (host clock; the rulings are dated 2026-09-30).
- **Verdict: PASS.** No BLOCKING finding. 1 SHOULD-FIX and 5 NOTEs.

**In short.**
- **The equality holds.** I read the new `negative_pair_witness_counted` (FKS:2210-2264 at the head) against today's `verify_negative_direction` and `negative_pair_witness` (FKS:2152-2215 at the base) line by line. The pair order, the sign rule, the cells visited, the zero test, the five operations per cell with their operands in order, the `terms` count, the `gamma` expression, the verdict and the error order are the verifier's own. Validation cannot change between calls, and the direction check always passes. Only a witness verdict reaches the verifier, which is unchanged and builds the published value. I found no way to make the two disagree.
- **My own differential harness agrees, 0 differences.** The oracle is a verbatim copy of the two base functions, generated from the base's bytes (`make_old_fns.sh`), not I20's copies. It covers 24,000 corrupted systems (random zero patterns with ±0 on either side, edge exponents up to ±2046, asymmetric sources, hazard values), 2,864 systems `prepare_structural` builds (n up to 99, with skew, ±0 couplings, subnormal and 10³⁰⁸ couplings), 134,400 steps of ulp scans across the verdict for every pattern of nonzero cells (1 to 4 terms), 20,000 skewed pairs, 29 adversarial cases (negative zeros, subnormals, overflow in the magnitude but not the energy, `radix_scale` at the normal range's edges, two failing cells of different kinds, the mapping error, n = 0 and 1, i32 overflow), and n = 100 and 120. It passes in debug and in release. In the corrupted set, 1,960 systems have an error before a witness and 1,393 a witness before an error.
- **I20's reference copies are verbatim.** Diffed against the base's bytes, they differ only in the renamed identifiers and a dropped `pub`.
- **Mutants:** 17 runs from clean copies. NONE passes. Six of I20's eleven, re-run with I20's exact edit text, are killed. Of my ten, five are killed by I20's tests. **Three survive every I20 and FK test and are killed only by my harness** (RV24-1): the two coupling cells swapped, the source cell read transposed, and the allowance charged for all four cells. Two are equivalent (RV24-N1).
- **B's records hold.**
  - The two uncommitted `runs.jsonl` match their recorded sha256.
  - My own comparison of the two files finds that the only fields that differ in 884 records are `wall_seconds`, `peak_rss_kib`, `memorystatus_level_end` and `solve_seconds`.
  - T9's lists are 112 of 112 and 16 of 16, and the base equals the calibration's Mac list.
  - Part 2's four outcomes are the factor's pivot refusal (DOF 6001 and 6002), with no `NegativeEnergy`.
- **I re-ran the gate from my own archives.**
  - **A sample:** FX-NP-A-ulp-1 and -2, RF-CHAIN-T-n10-r1e-12, RF-CHAIN-A-n10-r1e-12 and RF-WEAK-W-L-r1e-12, in both modes and both entries, on base, head and an instrumented head. All 20 are byte-identical base to head and equal I20's recorded full-envelope sha256 on both sides.
  - **Then the whole of part 1** except the six 10,000-member cases: 860 runs on the instrumented head, all 860 equal to I20's candidate hashes.
  - **Where the witness ran:** in exactly the 16 dense runs B named (n ≤ 61, no pair found).
  - **N10 CHAIN-ROT,** both dense entries, reproduces part 2's envelopes. There the witness visited all 17,997,000 pairs and verified none.
- **Reach:** the signature is unchanged, and every caller is the base's (a lexer scan at head and base). The site-table row is exact (6 matches), and the table passes. The dense witness can also be reached from `sparse_interactive` through SA's dense-evidence routes (RV24-N2), but no gate run takes that path.
- **The screen diagnosis checks out** as a claim: the counts, the screens, the predicted pivots, the profile-count lemma, and the part-1 disagreement list (RV24-N4, for the split-out slice).
- **Cost:** O(n²) by the code and by the counts. The release T6 reproduces at 0.966 s for n = 6,006, where I20 measured 0.875 s. The old search grows about ×16 per doubling of n.

## Findings

| ID | Class | Site | Evidence | Remedy |
|---|---|---|---|---|
| RV24-1 | SHOULD-FIX | `FK/structural/kf2_witness_tests.rs` (the differential set, `:419-890`), against `FKS:2228-2251` at the head | **Three single-edit regressions of the guard survive every test the PR commits** (I20's 14 and FK's `krev04` and `k1_negative_witness_*`):<br>- **RV24-M1**, the coupling cells swapped to (j,j), (i,j), (j,i), (i,i);<br>- **RV24-M5**, the source cell read as `stiffness[free[b]][free[a]]`;<br>- **RV24-M4b**, the allowance as `gamma(3 * 4 + 2)` whatever `terms` is.<br>Each breaks the equality the PR claims for every `PreparedSystem` (plan §4; RETURN §3), and my harness kills each on corrupted systems (`mutations/RV24-M1.log`, `RV24-M5.log`, `RV24-M4b.log`):<br>- **M1 and M5, on an evidence-allowed asymmetric source** with (j,i) = 10³⁰⁸ behind a 10³⁰⁸ diagonal and (i,j) subnormal: today returns `Range("arithmetic outside normal range")`, the mutant `Range("product overflow or underflow")`.<br>- **M1 and M5, on a skewed four-term pair at the verdict's edge:** today publishes a `NegativeEnergy`, the mutant `Ok(None)`.<br>- **M4b, on a three-term pair (a zero (j,j) cell) at the edge:** today publishes a witness, the mutant `Ok(None)`.<br>**Why I20's tests miss them:**<br>- the rounding family and the ties use symmetric sources (`Case::plain`), where the two coupling terms are equal;<br>- the only order-sensitive error case compares two *diagonal* cells (`:836-850`);<br>- every edge case has four terms.<br>**On systems `prepare_bound` builds,** these mutants are result-equivalent except on an exact tie:<br>- an edge pair's four terms lie in [1, 4), so the energy's partial sums are the same in either coupling order;<br>- the two coupling cells share one exponent sum, so their first error does not depend on the order;<br>- a built pair has 2 or 4 terms, and its 2-term pairs have energy in [2, 8).<br>My scan of 1.6 M built-like edge pairs found no flip (`checks/built_order_search.out`). **The code is right; the committed tests are narrower than the claim.** | Add to `kf2_witness_tests.rs`, test-only:<br>(a) the two asymmetric-source error-order pairs, in either order;<br>(b) a skewed-source edge scan (couplings k and k(1 + 10⁻³) under f64::MAX evidence);<br>(c) edge scans for 2- and 3-term pairs on corrupted systems.<br>Confirm each kills RV24-M1, M4b and M5. No product byte changes. My cases can be adopted: `scripts/rv24_probe.rs.txt`, `rv24_adversarial_cases` and `rv24_edge_scans_every_term_count`. |
| RV24-N1 | NOTE | FKS:2222 (sign), :2246-2247 (the two checks) | **Two mutants are equivalent, and survive everything:**<br>- **RV24-M2** takes `sign` from `matrix[j][i]`. `validate_prepared` requires `matrix[i][j] == matrix[j][i]` (IEEE equality, FKS:692-698) before the loop, and `x >= 0.0` agrees on IEEE-equal values: +0.0 and −0.0 are both `>= 0.0`.<br>- **RV24-M9** checks the magnitude before the energy. Both are `checked_value`, with one message and no side effect before the `?`, so any error is the same `Err`. | None. |
| RV24-N2 | NOTE | SA:2005-2020 (`solve_prepared`, both modes), SA:282-306 (`solve_binary64`), SA:1996-2003 (`solve_structural_sparse_binary64`), NI `lib.rs:1990` and `:2013` (the nonlinear active-set loop), SD:31-39 | **The dense witness is not dense-scrutiny-only.** SA:2017 runs it after a failed factor in either mode, including SD's skyline LDLᵀ in `SparseInteractive`. The product's nonlinear loop reaches it through `AssemblyEvidence::solve_binary64` and `solve_structural_sparse_binary64`; the linear product routes use the pattern evidence and the sparse witness. The brief and RETURN describe the reach as "dense scrutiny and NI's structural adapter"; the caller list itself (FKS:1965, SA:2017, SD:37, H `staged.rs:474`, plus test callers) is complete. **Measured:** in the gate's part 1 the dense witness ran in exactly the 16 dense runs B named, and in no sparse run (`gate/instr_part1_candidate.summary.txt`). The equality makes the mode irrelevant. | State the sparse-mode routes in the PR text. No code change. |
| RV24-N3 | NOTE | KF2 B, `t9/` and `gate/part1/SUMMARY.txt` | **What B's runs exercise.**<br>- **No T9 output reaches the witness.** None carries a factor pivot refusal or a `NegativeEnergy`; the 12 unresolved outputs are the prepare stage's "positive diagonal contribution absorbed". So 112 of 112 is an invariance check that does not run the new code.<br>- **Part 1's 16 witness runs** see n ≤ 61 and find no pair. The found-witness path is exercised only by the unit tests (CHANGE_RECORD "Limits" says so).<br>- **SUMMARY's wording:** "16 dense runs … publish the dense factor's refusal". The eight captured-entry FX-NP-A runs publish a recovered (solved) response, with the ordinary attempt's refusal in their diagnostics. | None; wording only. |
| RV24-N4 | NOTE | Plan §7 (split-out screen slice) | **The claim check holds:**<br>- the counts: 11,992 and 11,994 dense, 16 and 14 profile;<br>- the screens: 8.52e-11 against 1.14e-13 and 9.95e-14;<br>- the predicted pivot/scale: 3.37e-11 (guided tip, exactly 5 k_t) and 2.41e-11 (spine guided by the branch, ×500 restraint), which refuse dense and pass profile by ×296 and ×243 (`checks/screen_check.out`);<br>- Lemma Z: every l_ik before f_i is ±0, and each operation on it is exact, `checked_quotient` included (a = ±0 disables its underflow test);<br>- the part-1 disagreement list matches B's index exactly.<br>**For the slice:**<br>(a) measure the refusing rows; the pivots are still predictions;<br>(b) RF-CHAIN-T/A's dense counts 118 and 112 are consistent with the n = 61 free DOFs my instrumented runs show, not 60;<br>(c) enumerate class changes by running, not only dense-refused and sparse-published pairs, since a dense refusal can also become a publication where sparse refuses;<br>(d) `PivotEvidence.operation_count` is published (PP:1106-1107), so its meaning changes; that is a contract note. | To the split-out slice. |
| RV24-N5 | NOTE | FKS:2219-2263; plan §2 and §5 | **Cost, observed** (load 8.6; no claim):<br>- **The new search:** 0.006, 0.028, 0.157 and 0.781 s at n = 750, 1,500, 3,000 and 6,006 (21 to 43 ns per pair), with one validation (0.24 s at 6,006) and 0 verifications. The per-pair time grows because the column read `matrix[j][i]` strides across rows. The O(1) work per pair is the code's, and the counts confirm it.<br>- **The old search:** 0.0034, 0.048 and 0.89 s at n = 40, 80 and 160 (×14 and ×18 per doubling). Extrapolated, that is about 20 days at 6,000 on my banded system without symmetry evidence, the same order as I20's 31-day lower bound. | None. |

## 1. The equality claim (priority 1)

**Line by line** (base `78f55f927` against head `f2b8c85a2`; line numbers are FKS's):

| Aspect | Today (the search `:2195-2215`, the verifier `:2152-2193`) | KF2 (`:2210-2264`) | Same? |
|---|---|---|---|
| Validation | `validate_prepared` at `:2198`, and again in every verifier call (`:2156`) | once, at `:2213` | yes: it reads only `&PreparedSystem` and its source through shared references (no interior mutability), so every call returns the first call's value |
| The pair order | `for i in 0..n { for j in 0..i` (`:2200-2201`) | `:2219-2220` | yes |
| The sign | `v[j] = if matrix[i][j] >= 0.0 { -1.0 } else { 1.0 }` (`:2204-2208`) | `:2222-2226`, the same expression | yes |
| The direction check | `direction.len() != matrix.len()` or non-finite (`:2158`) | none | it always passes: v has length n and entries in {0, ±1} |
| The cells visited | row-major over all n², skipping `matrix[r][c] == 0.0 \|\| v[r] == 0.0 \|\| v[c] == 0.0` (`:2166`) | (j,j), (j,i), (i,j), (i,i), skipping `matrix[a][b] == 0.0` (`:2228-2240`) | yes: v is nonzero only at j < i, so the verifier's non-skipped cells are exactly these, in this order |
| A cell's operations | `radix_scale(stiffness[free[r]][free[c]], exp[r] + exp[c])`, `checked_product(checked_product(v[r], orig), v[c])`, then `energy` and `magnitude` checked, `terms += 1` (`:2169-2176`) | `:2241-2248`, with (da, db) = (s,s), (s,1), (1,s), (1,1) = (v[r], v[c]) | yes: the same operands in the same order. rustc does not contract or reassociate f64 |
| The allowance and verdict | `64.0 * gamma(3 * terms + 2) * magnitude`; `energy < -allowance` (`:2179-2180`) | `:2251-2252` | yes |
| The published value | `mapped`, `energy` and `allowance` built in the verifier (`:2181-2189`), including a mapping `radix_scale` `Err` | the unchanged verifier, called on the same v (`:2254-2257`) | yes |
| Error order | the first failing operation in the cell order; the mapping after the verdict | the same; then the verifier's mapping | yes |

**Where it could have broken, and did not:**
- **An error at a different pair.** Today's search can fail only inside the verifier. Validation is `Ok` after the first call, and the direction check passes, so the verifier's first failure at pair p is the first failure among p's own non-skipped cells. That is exactly where the guard fails.
- **Panics.** Validation makes every index safe: the matrix is square n×n, n = `free_dofs.len()` = `scale_exponents.len()`, every free DOF is below N, and the stiffness is N×N. The one panic left is the i32 exponent sum in a debug build. It sits in the same expression, at the same cell, in both; in release both wrap. My adversarial case shows `PANIC:attempt to add with overflow` for both in debug, and `NONE` for both in release. Only the panic's source line differs, and that is not a return value.
- **Negative zeros.** `matrix[i][j] = -0.0` with `matrix[j][i] = +0.0` passes validation. Both code paths take s = −1 and skip both cells. A −0.0 source value gives a −0.0 term, and 0.0 + (−0.0) = +0.0 in both.
- **The skip uses the prepared matrix, not the source.** Both skip on `prepared.matrix` and evaluate the source, so a nonzero source behind a zero prepared cell is invisible to both.

**My harness** (`scripts/rv24_probe.rs.txt`, mounted in a copy of the head only; its oracle is `scripts/rv24_old_fns.rs.txt`, generated by `scripts/make_old_fns.sh.txt` from `git show 78f55f927:…/structural.rs` lines 2152-2215). It runs, per system, the oracle, the public `negative_pair_witness` and the private counted function, each under `catch_unwind`. It compares them as strings carrying every `to_bits()` of a `NegativeEnergy`, the `Debug` of any other error and the panic message. It also checks the count invariants with its own cell count. Results (`probes/harness_debug.out`, `harness_release.out`):

| Set | Systems | Outcomes | Coverage |
|---|---|---|---|
| Corrupted, random (n 2–15) | 24,000 | 4,944 witnesses, 8,456 none, 5,106 `radix` errors, 1,338 product underflows, 245 energy or magnitude range errors, 3,911 validation refusals | the per-pair event layout: error then witness 1,960; witness then error 1,393; error only 4,729; witness only 3,551; first pair 4,457; last pair 1,032 |
| Built by `prepare_structural` (n 2–99) | 2,864 (136 refused by prepare) | 1,991 witnesses, 856 none, 17 range errors | 9 error-then-witness, 32 witness-then-error |
| Edge scans (7 patterns × symmetric or skewed × 60 × 160 ulps) | 134,400 | the verdict crosses in every scan of P4, P3a, P3b and P2d, and in skewed P2c | 1- to 4-term pairs; the pair first, last or in the middle |
| Skewed four-term pairs | 20,000 | 6,397 witnesses | 3,989 whose energy bits depend on the coupling order |
| Adversarial | 29 | see `harness_debug.out` | including overflow in the magnitude but not the energy, and a subnormal partial energy |
| Large | 4 (n 100, 120) | none, and a witness at the last pair | |
| The oracle verifier against the product's verifier | 2,400 directions | all equal | |

**0 differences, in debug and in release.**

## 2. The tests

- **The reference copies are verbatim** (`checks/reference_copies_and_scope.txt`). `reference_verify_negative_direction` (test `:17-58`) equals base `:2152-2193` except that `pub` is dropped and the name changed. `reference_negative_pair_witness` (test `:61-81`) equals base `:2195-2215` after renaming. They call the base's unchanged helpers (`validate_prepared`, `radix_scale`, `checked_*` and `gamma` are byte-identical at the head).
- **The plan's coverage claims are real as stated:**
  - witnesses first, last, in the middle, absent and several;
  - ±0 couplings beside a witness;
  - both signs;
  - n from 2 to 64 with witnesses and 80 and 100 without;
  - 240 random systems;
  - 9 exact ties with b ± 1 ulp;
  - overflow and underflow before and after a witness;
  - corrupted shapes, sources and exponents;
  - the counts.
  
  I re-ran them: FK's full suite passes, 415 with 1 ignored (`suites/fk_full_suite_head_debug.log`).
- **They are narrower than the claim** in three places (RV24-1):
  - no asymmetric source at an edge or with order-sensitive errors in the coupling cells;
  - no edge case with fewer than four terms;
  - the order-sensitive error test uses only the diagonal cells.
  
  I20 disclosed the rounding family's thinness (4 flips in 12,288). My skewed and edge sets show what is missing.

## 3. Mutants

Each run used a clean copy of the head's `core` (from my `git archive`), one edit, its own debug target (deleted afterwards), and one job at `-j 4`. The tests were FK's lib filtered to `kf2_`, `krev04`, `k1_negative_witness` and `rv24_`, then `FKT/s11_site_table`. The driver is `scripts/rv24_run_mutants.sh.txt`, the edits `scripts/rv24_mutants.py.txt` (I20's six with the edit text of their `mutants.py.txt`, checked equal), and the results `mutations/summary.txt` and the per-mutant logs.

| Mutant | Change | Verdict | Killed by |
|---|---|---|---|
| NONE | the head | passes (24 run, 2 ignored; site table 3 of 3) | — |
| I20-M1 | skip the first pair | killed | 12 I20/FK tests, 6 RV24 |
| I20-M2 | the four cells reversed | killed | 2 I20 (the error order, the rounding family), 3 RV24 |
| I20-M3p | `3 * terms + 1` | killed | 2 I20 (the count), 1 RV24 |
| I20-M5 | skip zero-coupling pairs | killed | 8 I20, 6 RV24 |
| I20-M6 | sign `>` | killed | 1 I20, 3 RV24 |
| I20-M10 | the zero test removed | killed | 7 I20, 6 RV24 |
| RV24-M1 | the coupling cells swapped | **killed only by RV24** | RV24's adversarial, corrupted and edge tests (RV24-1) |
| RV24-M2 | sign from `matrix[j][i]` | survives, equivalent | RV24-N1 |
| RV24-M3 | the guard's error swallowed and the pair skipped | killed | 3 I20 and the site table; 4 RV24 |
| RV24-M4a | `terms` counted on zero cells | killed | 7 I20, 6 RV24 |
| RV24-M4b | the allowance charges all four cells | **killed only by RV24** | RV24's edge scans (RV24-1) |
| RV24-M5 | the source read transposed | **killed only by RV24** | as RV24-M1 (RV24-1) |
| RV24-M6 | the exponent sum `e[a] + e[a]` | killed | 3 I20, 2 RV24 |
| RV24-M7 | the verified direction's sign flipped | killed | 9 I20/FK, 6 RV24 |
| RV24-M8 | the magnitude sums signed terms (a more permissive guard) | killed | 2 I20 (the count), 3 RV24 |
| RV24-M9 | the magnitude checked before the energy | survives, equivalent | RV24-N1 |

- Not re-run: I20-M3, M4, M7, M8 and M9. M9 restores the O(n⁴) search, and I20 records its 600 s alarm.
- **No survivor is a defect in the code.** The three RV24-only kills are gaps in the committed tests (RV24-1).

## 4. B's records

(`checks/b_records.txt`, `checks/runs_jsonl_diff.out`, `gate/`)
- **Integrity:** KF2's three `SHA256SUMS` verify: 112 entries, 113 files, no machine path. RETURN, CHANGE_RECORD, SHA256SUMS and `b/SHA256SUMS` hash to the values in ROOT's rulings.
- **T9:**
  - `sha_base.txt` and `sha_cand.txt` are byte-identical (112 lines: 56 dense and 56 sparse), and so are the extra lists (16).
  - The base list equals `PLATFORM_CALIBRATION_MAC/t9/output_sha256_main_mac_native.txt`.
  - This supports "112 of 112". T9 does not run the witness (RV24-N3).
- **Part 1, the recorded runs:**
  - `part1_base/runs.jsonl` hashes to `c42981e5…` (605,711,611 B) and `part1_cand/runs.jsonl` to `11dfe829…` (605,711,700 B), each with 884 records, as `gate/uncommitted_sha256.txt` records.
  - **My structural diff of the two** (`scripts/rv24_runs_diff.py.txt`, not I20's comparer): the key sets are equal, and across all 884 records the only paths that differ are `/wall_seconds` (881), `/peak_rss_kib` (835), `/memorystatus_level_end` (121) and `/probe/run/solve_seconds` (882). Envelopes, full-envelope sha256, `stderr_tail`, exit codes and outcomes are all equal. This supports "884 of 884 identical".
- **Part 1, re-run by me:**
  - **The builds:** P1's probe (`main.rs` `cd1052f7…`, the committed record; `Cargo.lock` `d7bdd546…`, the one the Mac baseline README names), built in release from my own archives of base and head, plus an instrumented head. The instrumentation is one `eprintln!` in `negative_pair_witness` (`scripts/rv24_instrumentation.diff.txt`).
  - **The requests:** I20's regenerated `gen_out` equals the calibration's `gen_out_sha256.txt` (223 of 223).
  - **The sample** (`gate/sample_part1.out`): 5 requests × 2 modes × 2 entries = 20 runs. All 20 are byte-identical base to head. Each also equals I20's recorded full-envelope sha256 (base against `index_base`, head against `index_cand`) and I20's recorded summary object. The instrumented head's envelope equals the head's. The witness ran in all 10 dense runs and in no sparse run.
  - **All of part 1 except the six 10,000-member cases** (`gate/instr_part1_candidate.*`; 860 runs, 2 at a time, 7.6 min): 860 of 860 full envelopes equal I20's candidate index, with 0 timeouts and 0 non-zero exits. The witness ran in 16 runs, exactly B's list: n = 61 (RF-CHAIN-T and RF-CHAIN-A), 6 (RF-SKEW-T-PIN-OFF-122), 13 (RF-WEAK-W-L) and 7 (the four FX-NP-A), with no pair found and 0 verifications.
  - **Why the 10,000-member cases were skipped:** the host rule forbids a dense matrix at that size. F1b's guard refuses their dense runs before any n² allocation, and their sparse runs take the pattern route.
- **Part 2:**
  - All four committed records are `NumericallyUnresolved("nonpositive or cancellation-unresolved structural pivot")` at DOF 6001 (CHAIN-ROT) and 6002 (TREE-AX), with exit 0, no timeout, 67.4–68.0 s, 0 results and no `NegativeEnergy` anywhere. The captured entries also record source recovery's budget stop.
  - That is the factor's error, returned by `unwrap_or` after the witness found nothing.
  - **My instrumented CHAIN-ROT run** (`gate/instr_n10_chain_rot.*`): both dense entries reproduce part 2's full envelopes (`68cf1929…`, `cb611210…`) in 66.9 and 67.1 s. The witness visited 17,997,000 pairs, evaluated 36,065,952 cells and verified none.
- **src-tauri:** the committed log shows 116 passed and 0 failed (lib), matching F1b's count. I did not re-run it.

## 5. Reach

(`checks/reach_scan.out`, a lexer scan that strips comments and literals, at head and base.) The token `negative_pair_witness` occurs at:
- **FKS:1965** (`solve_prepared_dense`);
- **SA:2017** (`solve_prepared`, both modes; RV24-N2);
- **SD:37** (`solve_structural_sparse`, after the skyline LDLᵀ);
- **H `k6/staged.rs:474`** (`dense_factor_finish`, and its `use` at `:29`);
- **the tests:** FK's `krev04` (FKS:3006, :3021), K1's `sparse/tests.rs:1181`, `:1226`, SD's `k1_tests.rs:12`, `:371`, and I20's new module.

The base has the same set, less I20's module. A whole-repository `git grep -w` finds no other file. The declared signature is identical at head and base: `pub fn negative_pair_witness(prepared: &PreparedSystem<'_>) -> Result<Option<StructuralError>, StructuralError>`. The PR changes exactly the three FK files outside `execution/` (`checks/reference_copies_and_scope.txt`).

**The site table** (`FKT/s11_site_table.rs:166`): the row `("FK/structural.rs", "negative_pair_witness_counted", 6, …)` matches the function's six scanned sites: `visited += 1`, `terms += 1`, `evaluated += terms`, `verified += 1`, and the two self-assignment folds at `:2246-2247`. The table passes (3 of 3), and it also kills RV24-M3, whose closure removes the folds. `negative_pair_witness` itself has no site, and `verify_negative_direction` keeps its 3.

## 6. The screen diagnosis (claim check only; RV24-N4)

- **The mechanism:** the dense `cholesky` charges `2 * j + 2` at the pivot (FKS:1863; `screen_pivot` at :1523-1548, where screen = 64γ(count)·scale). The dense scale is |a_ii| + Σ l_ik², so scale ≈ 2a_ii.
- **The counts and indices:** with N0 restrained, N1000.UY is ordered 5,995 and B500.UZ 5,996. The dense counts are 11,992 and 11,994. The first nonzeros are N999.UX (5,988; the rotated axis couples all 12) and P500.UZ (5,990; axial only), so the profile counts are 16 and 14.
- **The pivots:** my own arithmetic from SEC_N (`checks/screen_check.out`):
  - **CHAIN-ROT:** the guided-tip Schur complement with UX condensed and UZ clamped is exactly 5·12EI/L³ = 1.200e-2 N/m. With a_ii = 1.782e8, pivot/scale = 3.37e-11.
  - **TREE-AX:** the branch's EI/L_e is 500 times the spine's EI/L, so P500 is effectively guided. That gives 12EI/L³ = 1.921e-2 N/m, in series with the branch's axial 3.98e8, and pivot/scale = 2.41e-11.
  - Both refuse at 8.52e-11 and pass at 1.14e-13 and 9.95e-14 (×296 and ×243).
  - This is consistent with I15's hypothesis, and still unmeasured.
- **Lemma Z is sound:**
  - by induction, each l_ik with k < f_i is ±0: `checked_product` of a ±0 is ±0, ±0 − ±0 is ±0, and `checked_quotient(±0, l_kk > 0)` is ±0 without error;
  - at the pivot, each such term is +0, and sum − (+0) and scale + 0 are exact;
  - so the first 2f_i operations round nothing, and the profile count bounds the same computed values.
- **The list:** B's part-1 index has exactly these dense-against-sparse disagreements:
  - RF-CHAIN-A-n10, RF-CHAIN-T-n10, RF-SKEW-T-PIN-OFF-122 and RF-WEAK-W-L at r1e-12: dense refused, sparse solved;
  - RF-SKEW-T-CANT-OFF-122-r1e-12: the reverse;
  - CONT-n10000: F1b's guard.
  
  This matches plan §7.4. The expected changes are plausible; the four r1e-12 cases are the ones where the witness ran.

## 7. Cost

- **By the code:** one validation (O(N² + n²)), then per pair one sign read, four zero tests, at most four cell evaluations, and one `gamma`. Only a witness verdict allocates and verifies, and then it returns: `verified` ≤ 1 on every system in my harness.
- **The counts:** at n = 6,006 there are 18,033,015 pairs, 36,086,034 cells and 0 verifications for I20's chain (T6, reproduced exactly), and 36,198,030 cells for my banded system.
- **The timings** are in RV24-N5 (`probes/cost_release.out`). They are observations under shared load, not claims. No budget is needed in the witness at this cost (ROOT's Q4).

## Reviewer, brief and delegation

- **Reviewer.** RV24 is a Type 2 TASK, dispatched by ROOT (HELP_HUMAN, Agent 0) directly through the host's background-subagent mechanism: a Claude Code subagent of ROOT's session. ROOT is the only return path. I delegated nothing.
- **Brief:** ROOT's RV24 dispatch message (the candidate, the seven review items, the host rules and the output), with `_COMMON.md` and `I8R_K1_RESUME.md:24-50`.
- **Read:**
  - Root `AGENTS.md` and `agents/AGENT_TASK.md`;
  - `T3/TASK_BRIEFS/_COMMON.md`, `I8R_K1_RESUME.md:1-60` and `I20_KF2_IMPLEMENTATION.md`;
  - every KF2 section of `ROOT_RULINGS_V1.md`, from "KF2: spawn" to "KF2: D accepted; PR to review";
  - `REVIEW/KF3_REVIEW.md` (the format);
  - on the KF2 branch, `PLAN_CHECKPOINT0.md` (all of it; §3, §4 and §7 closely), `RETURN.md`, `CHANGE_RECORD.md`, `_run_records/a/` (the mutation script and table) and `_run_records/b/` (T9, part 1, part 2 and src-tauri);
  - K6's RETURN §8.4–8.5 at `78f55f927`;
  - the full code diff `78f55f927...f2b8c85a2`;
  - at base and head, FKS `:250-380`, `:540-700`, `:1186-1380`, `:1515-1550`, `:1825-1875`, `:1940-1970` and `:2100-2264`;
  - SA `:180-310`, `:640-800`, `:1985-2020`;
  - SD `:20-40`;
  - H `k6/staged.rs:465-480`;
  - NI `lib.rs:1980-2020`;
  - PP `lib.rs:1095-1110`;
  - I20's `kf2_witness_tests.rs` (all of it);
  - `FKT/s11_site_table.rs` (header, scanner and row).
- **Ran:**
  - **Where:** clean `git archive` copies of head and base (`projects/chirality-piping`, `execution/` excluded) under `<wt>/rv24/`; my probe copy and one copy per mutant under `<wt>/rv24-mut/`; targets under `<wt>/rv24-target/` and each mutant's own.
  - **Settings:** `RUSTUP_TOOLCHAIN=1.97.1`, `RUSTUP_AUTO_INSTALL=0`, `CARGO_INCREMENTAL=0`, `--offline --locked`, `-j 4`, `RUST_TEST_THREADS=2`, one cargo job of mine at a time. Python 3.13, standard library only.
  - **The work:**
    - FK's full suite (debug);
    - my harness in debug and release;
    - the release cost tests (I20's T6 and mine);
    - 17 mutant runs;
    - three P1 probes;
    - 20 sample runs × 3 probes;
    - 860 instrumented part-1 runs and 4 N10 runs.
  - **The host:** the memory guard ran throughout; its log is unchanged, with no KILLED line. RV23's and I19's cargo ran beside mine.
- **Git: read-only** in `<wt>/kf2` and `<wt>/numerics` (`rev-parse`, `log`, `diff`, `show`, `grep`, `archive`, `ls-files`). No commit, stash, reset, checkout, fetch or push, no index operation, and no GitHub access. I did not edit the KF2 worktree or any branch.
- **Writes:** this file and `T3/REVIEW/_run_records/kf2_review/**`, uncommitted. My copies and targets are deleted.
- **Not done:**
  - NI's, SD's, PP's and H's suites, and src-tauri (they call the witness unchanged, and I20 and B ran them);
  - `gen_k4_vectors.py --check` (KF2 changes no vector file);
  - the ten 10,000-member part-1 runs and TREE-AX's part 2 (the host rule; CHAIN-ROT stands for the pair);
  - five of I20's eleven mutants;
  - Linux.

## Records (`T3/REVIEW/_run_records/kf2_review/`)

- `README.txt`, `SHA256SUMS`.
- `scripts/`: the harness and its generated oracle, the mutant edits and driver, the runs.jsonl comparer, the reach scan, the gate sample and instrumented runners, the instrumentation patch, the probe manifests, the built-order searches and the screen arithmetic, all as `.txt`.
- `probes/`: the harness in debug and release, and the release cost observations.
- `mutations/`: `summary.txt`, the driver log, and each mutant's log, build log and harness tallies.
- `suites/`: FK's full suite at the head.
- `checks/`: the reference copies and scope, B's records, the runs.jsonl diff, the reach scan, the built-order search, the screen arithmetic.
- `gate/`: the probe hashes and inputs, their build logs, the 20-run sample, the 860-run instrumented part 1 and the N10 CHAIN-ROT runs.
- Paths are shown as `<wt>`, `<scratch>`, `<home>` and `<tmp>`.
