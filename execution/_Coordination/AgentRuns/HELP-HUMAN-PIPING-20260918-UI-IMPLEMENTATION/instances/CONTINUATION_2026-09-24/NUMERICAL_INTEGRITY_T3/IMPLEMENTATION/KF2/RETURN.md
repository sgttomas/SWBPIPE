# I20 return: slice KF2 (K6's N10: the dense negative-pair witness in O(n²), every result unchanged)

## 0. At a glance

**What changed:**
- **The witness is now O(n²).** Today, `negative_pair_witness` (FK's dense witness search) builds an n-vector for each of its n(n−1)/2 pairs, re-validates the whole system, and scans n² cells. That is O(n⁴): about 31 days at 6,006 DOFs, as a lower bound (§4.1).
- **The new witness** evaluates each pair's only four cells in O(1), with exactly the arithmetic and order of `verify_negative_direction` (the verifier, left unchanged). Only a pair judged a witness is built into a direction and passed to the verifier, which produces the published value.
- **Measured:** at 6,006 DOFs the witness takes 0.875 s in release. N10's two models, which were killed at 1,800 s, now end with the dense factor's refusal in about 67–68 s through the product, and about 203 s through `k6_observe` (three solves).

**What did not change:**
- **Every result, bit for bit, errors included.** The equality holds for every value of `PreparedSystem`, not only for those that `prepare_bound` builds (§3; plan §4).
- **Every committed output and gate envelope:**
  - T9 is 112 of 112 byte-identical, and the extra corpus 16 of 16;
  - gate part 1's 884 runs are identical to a fresh Mac base run, full envelopes included.

**Checks:**
- **Differential tests** against verbatim copies of today's two functions (n = 2 to 100), including exact allowance ties, errors, and corrupted systems. The count tests show O(1) cells per pair and at most one verification.
- **Mutations:** 11 mutants are killed, and NONE passes.
- **Suites:** FK, the site table, SD, NI (with SA), H and src-tauri pass. PP fails only its known Mac platform test `t13`, identically on the base.

**Routed, not in KF2:**
- the dense pivot screen, a separate slice (§8.1);
- the dense route's cancellation note, for the owner (§8.2).

## 1. Brief, basis, delegation and paths

- **Brief:** `T3/TASK_BRIEFS/I20_KF2_IMPLEMENTATION.md`, committed at numerics `6d9832db2`. It is read with `_COMMON.md`, and with `I8R_K1_RESUME.md:24-50` for the Mac host.
- **Rulings** (`T3/ROOT_RULINGS_V1.md`, numerics):
  - "KF2: spawn (ROOT, 2026-09-30)";
  - "KF2: rulings on I20's checkpoint-0 plan", `2778326cb`: Q1–Q7, the screen declined and split out;
  - "KF2: checkpoint A accepted; B granted", `bbbfce02a`;
  - "KF2: checkpoint B accepted; D now", `1642bd310`.
- **Plan:** `PLAN_CHECKPOINT0.md` (sha256 `76337c43…`), accepted and committed at `573bd3835`. Its §4 is the full equality argument.
- **Branch:** `codex/piping-kf2-20260930` in `<wt>/kf2`, from main `78f55f927` (K6b merged).
- **Commits (made by ROOT):**
  - `573bd3835`: checkpoint 0;
  - `1b10121fa`: A, meaning the code, the tests, the site-table row and `_run_records/a/`;
  - `6caa38e23`: B, meaning `_run_records/b/`;
  - D is these records.
- **Delegation:**
  - ROOT (HELP_HUMAN, Agent 0) dispatched I20 directly as a Type 2 TASK, using the host's background-subagent mechanism: a Claude Code subagent of ROOT's session.
  - Rulings arrived as in-session messages at checkpoints 0, A and B.
  - I20 delegated nothing and made no Git write or index operation, apart from the read-only `git archive` and `git show`.
  - The write boundary (this folder, plus the approved files in §9) is set by the brief and the rulings, and was kept by I20. No host mechanism is known to enforce it.
- **Abbreviations:**
  - `P/` = `projects/chirality-piping/`;
  - `T3/` = `P/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/`;
  - `FK` = `P/core/solver/frame_kernel/src/`, `FKS` = `FK/structural.rs`, `FKT` = `P/core/solver/frame_kernel/tests/`;
  - `SA` = `P/core/solver/nonlinear_integration/src/structural_adapter.rs`;
  - `SD` = `P/core/solver/sparse_direct/src/`;
  - `H` = `P/core/solver/performance_harness/`;
  - `PP` = `P/core/product_physics/src/lib.rs`;
  - `APP` = `P/apps/desktop/src-tauri/src/lib.rs`.
- **Platform:** Mac (`aarch64-apple-darwin`, Apple M5 Max, 128 GiB, no swap), rustc 1.97.1, rustfmt 1.9.0-stable (`_run_records/a/toolchain.txt`).

## 2. The design (FKS:2194-2264 at `1b10121fa`)

- **`negative_pair_witness`** (`:2195-2199`) keeps its signature, its doc comment and all its callers:
  - FKS's `solve_prepared_dense`;
  - SA `:2017`;
  - SD `:37` and `SD/structural/k1_tests.rs:371`;
  - H's `k6/staged.rs:474`.

  It now delegates to the new private `negative_pair_witness_counted` (`:2201-2264`). This is the one helper Q7 allows, and it holds the loop, the counts and the pair evaluation inline.
- **Per pair (i, j < i), in today's order:**
  1. It computes today's sign from `matrix[i][j] >= 0.0`.
  2. It evaluates the cells (j,j), (j,i), (i,j) and (i,i), skipping a cell whose `prepared.matrix` entry is zero. Each cell uses exactly the verifier's operations: `radix_scale`, two `checked_product`s, and the `checked_value` sums for energy and magnitude.
  3. It forms `allowance = 64·γ(3·terms + 2)·magnitude` and tests `energy < -allowance`.
  4. Only on a witness verdict does it build `v = e_i + sign·e_j` and call `verify_negative_direction(prepared, &v)?`, returning its `Some`. That call is exactly today's loop body.
- **Pairs with a zero coupling are visited, with no skip.**
- **The counts** (`visited`, `evaluated`, `verified`) are returned only by the helper, for the tests.
- **The site table** gains one row (Q5): `("FK/structural.rs", "negative_pair_witness_counted", 6, …)` (`FKT/s11_site_table.rs:166`).
- **PP's site test** is unaffected. The helper reads no force token, because the verifier builds the published direction.
- **Test module:** one `#[cfg(test)] mod kf2_witness_tests;` line (Q1), at FKS:2700-2701. rustfmt orders module lines, which puts it before `s11f_tests`.

## 3. The equality argument (summary; the full argument is the plan's §4)

For every `PreparedSystem`, including corrupted ones, the new witness returns what today's returns: the same `Err`, the same `Ok(None)`, or the same first `NegativeEnergy`, with bit-identical direction, energy and allowance.

- **Validation:** `validate_prepared` is deterministic on immutable data, so today's per-pair re-validation always repeats the first call's result.
- **The direction check:** the verifier's check always passes for e_i + s·e_j.
- **The cells:** the verifier's non-skipped cells for that direction are exactly the four cells, in its row-major order, with the same five operations and operands (rustc does not contract or reassociate f64 arithmetic). So both calls reach the same first error, with its fixed message, or the same bits for energy, magnitude and terms.
- **The verdict:** the same expression gives the same bits and so the same verdict. On a witness, the new code makes today's verifier call itself, so the published value, and any error from mapping the direction, are today's.
- **Why the design is safe:** a guard that is too permissive only costs one extra verification. So only a guard that is too strict, or one that raises a different error, could change a result, and the differential and count tests look for exactly those.
- **Where the old search's errors come from:** on systems `prepare_bound` builds, an error can come from the product underflowing (a subnormal coupling at exponent sum 0) or from the energy overflowing. The `radix_scale` and mapping errors need a corrupted system.
- ROOT checked and accepted the argument ("KF2: rulings on I20's checkpoint-0 plan").

## 4. Cost

### 4.1 Before (plan §2)

- Per pair: one n-vector allocation, then `validate(source)`, then the rest of `validate_prepared`, then the n² scan. That is about 3N² + 3n² reads on SA's dense route, where N is the total DOF count and n the free count.
- T_old ≈ 3·c_r·n⁴, where c_r is the time per read.
- **Calibration:** V-K's record of RF-MECH-DISC-CHAIN100 (n = 624, 312 s, no witness found) gives c_r ≈ 0.68 ns.
- **At N10's n = 6,000:** 18.0 M pairs, about 0.15 s per pair, so T_old ≈ 2.7 × 10⁶ s, about 31 days. That is a lower bound. K6's kill came about 0.06% of the way through the pairs.

### 4.2 After (measured)

- **The formula:** O(N² + n²), from one validation, O(1) per pair and at most one verification.
- **T5, debug, n = 600:** 179,700 pairs visited, 2 cells per pair plus 2 per stored coupling, 0 verifications.
- **T6, release, `#[ignore]`, n = 6,006:** 18,033,015 pairs visited and 36,086,034 cells evaluated. That is 2 × pairs + 2 × 10,002 stored couplings, with 0 verifications. The witness took **0.875 s**, against a stated bound of 30 s (`_run_records/a/t6_release.log`).
- **N10's two models, through H's unchanged `k6_observe`,** release, built from a `git archive` (Q2) (`_run_records/a/n10/`):

  | Model (dense) | Factor | Witness | Outcome | Process |
  |---|---|---|---|---|
  | RF-LARGE-CHAIN-n01000-ROT | refused at DOF 6001 after 66.26 s | 0.937 s, no pair | the factor's `NumericallyUnresolved` | 203 s (staged run plus two entries), exit 0 |
  | RF-LARGE-TREE-n01000-AX | refused at DOF 6002 after 65.63 s | 0.913 s, no pair | the same | 203 s, exit 0 |

  SA's two entries equal the staged run, and K is bitwise equal. Heap peak was about 3.48 GB.

  These are observations with their load, not timing claims. Host load was about 5–7.

## 5. Tests (`FK/structural/kf2_witness_tests.rs`, new, 969 lines, 14 tests)

- **T1, the reference copies:** verbatim copies of `verify_negative_direction` and `negative_pair_witness` at `78f55f927` (FKS:2150-2215), renamed. One test compares the reference verifier with the product's verifier on arbitrary directions.
- **T2, the comparator:** it compares the outcome shape, the error variant and message, `to_bits()` of energy, allowance and every direction entry, and the `Debug` bytes as well.
- **T3, the differential:**
  - **Sizes:** n ∈ {2, 3, 4, 5, 8, 13, 21, 34, 48, 64}, plus 80 and 100 with no witness.
  - **Witness position:** at the first pair, at the last pair, in the middle, none, and several (the first must win).
  - **Couplings:** zero couplings (+0.0 and −0.0) beside a witness, and both coupling signs.
  - **Random systems:** 240, with 142 witnesses and 98 without. They include shuffled free maps, prescribed DOFs, and skewed sources under symmetry evidence.
  - **Errors:** energy overflow and product underflow, each placed before and after a witness.
  - **Corrupted systems:** shape corruptions, a replaced source, a negative source diagonal behind a zero coupling, a zero-coupling diagonal whose scaling overflows, two failing cells whose first error depends on order, an error in mapping the direction after the verdict, a hidden source coupling, and a pair of magnitude 0.
  - **Exact allowance ties:** 9 found on built 2×2 systems (plan §8), with b ± 1 ulp on each side of the verdict.
  - **A rounding family** that checks it can detect a cell-order change: 4 of 12,288 pairs change verdict in the reversed order.
- **T4, the counts:**
  - `verified` is 1 exactly when a witness is returned;
  - on `Ok(None)`, `visited` is n(n−1)/2;
  - `evaluated` equals an independent count of nonzero cells;
  - `evaluated ≤ 4·visited`.
- **T5 and T6** are in §4.2.
- **T7, N10 in release,** used `k6_observe` (§4.2).
- **Output:** `_run_records/a/kf2_targeted.log` records 17 passed and 1 ignored (T6), together with the existing `krev04` and `k1_negative_witness_*` tests, and the tests' discrimination counts.

## 6. Mutations (`_run_records/a/mutation_table.md`, `mutants/`)

Each mutant ran on a clean `git archive 573bd3835` copy of `P/core`, with the candidate's three files overlaid, one edit (`mutants.py.txt`) and its own target, one at a time, all deleted afterwards. The tests were FK's lib filtered to `kf2_`, `krev04` and `k1_negative_witness`, then the site table.

| ID | Mutation | Verdict | Killed by |
|---|---|---|---|
| NONE | none | passes | 17 passed, 1 ignored |
| M1 (brief 1) | skip the first pair | killed | 12 tests (results and counts) |
| M2 (brief 2) | the four cells in reverse order | killed | the rounding family (a verdict flip), and the first error depending on cell order (a corrupted system) |
| M3 (brief 3) | allowance with 3·(terms+1)+2 | killed | the ties (the "one below" pair missed), the rounding family |
| M3′ | 3·terms+1 (too small; results unchanged by design) | killed | the ties and the rounding family, by the verification count |
| M4 (brief 4) | return the last witness | killed | several witnesses, errors, random systems, zero couplings |
| M5 (brief 5) | skip zero-coupling pairs | killed | a zero-coupling witness (a corrupted system, by result), and the counts on built systems |
| M6 | sign rule `>=` → `>` | killed | the zero-coupling witness's direction |
| M7 | verdict `<` → `<=` | killed | the exact tie and the magnitude-0 pair, by the verification count |
| M8 | drop the validation before the loop | killed | the corrupted-shape test and `krev04` |
| M9 | guard always true (today's search) | killed | 6 count tests failed, then the 600 s alarm stopped the binary (exit 142; §10) |
| M10 | the cell's zero test removed | killed | 7 tests (results and counts) |

## 7. Results

### 7.1 A (`_run_records/a/`)

- **The KF2 and witness tests:** 17 passed, 1 ignored (T6, which passes in release).
- **FK's full suite:** 415 passed, 0 failed, 1 ignored. There are 0 warnings in FK's build.
- **The site table:** 3 of 3.
- **The callers' suites** (`cargo test --offline --locked`, as CI runs them):
  - SD: 30 passed;
  - NI, with SA: 134 passed;
  - H: 74 passed, plus `k6_alloc`;
  - PP (`--no-fail-fast`): the lib passes 412 and fails 1, the known Mac platform test `s11g_tests::t13_committed_fallback_uz_is_byte_identical`. Every integration test passes, including `s11f_site_test` (11 of 11).
- **The PP failure** is identical on a `git archive` of the base (`base_pp_t13.log`) and in ROOT's three Mac-main calibration runs.
- **PP's warnings** (6 in the lib test build) are the base's own.
- **rustfmt:**
  - the new test file and the site table are clean;
  - FKS's hunks are clean;
  - the only rustfmt hunks under FK are the base's own, in `exact_boundary/functionals*`.

### 7.2 B (`_run_records/b/`; base `78f55f927` and candidate `1b10121fa`, as `git archive` trees that differ only in the three FK files)

- **T9:** I used S11-K's `fixdiff_main.rs` (`ec089c1d…`), built in release.
  - 112 of 112 outputs are byte-identical, and F1b's extra corpus is 16 of 16.
  - The base also equals the platform calibration's Mac hashes.
- **Gate part 1: PASS.**
  - **Method:** the full-envelope probe (`cd1052f7…`, with the 6 GiB heap cap), and G1's `gate_run_base_full.py`, `run.py`, `compare.py` and K-D5's `gate_check.py`, all unchanged. The requests were re-generated and match the calibration's hashes, 223 of 223.
  - **Identical runs:** all 884 runs are identical in outcome, ok, exit code, summary and full envelope sha256 (818 on each side) and error text. `diff -rq` also finds `full/`, `envelopes/` and `stderr/` identical.
  - **`gate_check`:** 764 evaluated, 332 trusted, 0 trusted breach triples on both sides. There is no heap-cap abort and no timeout.
  - **Witness reach:** 16 dense runs reach the witness: FX-NP-A-ulp-{.75, 1, 1.5, 2} and the r1e-12 cases RF-CHAIN-A-n10, RF-CHAIN-T-n10, RF-SKEW-T-PIN-OFF-122 and RF-WEAK-W-L. It finds no pair, and those runs are byte-identical. No gate run publishes a `NegativeEnergy`.
  - **The two `runs.jsonl`** (606 MB each: base `c42981e5…`, candidate `11dfe829…`) stay uncommitted in `<wt>/scratch/i20/b/gate/`, as F1b's did. They are indexed by `part1/index_*.tsv`.
- **Gate part 2, candidate only:** all four dense N10 runs end in 67.4–68.0 s, far inside 1,800 s. Each is `refused_blocked` with the dense factor's `NUMERICAL_INTEGRITY_UNRESOLVED` (DOF 6001 for CHAIN-ROT, 6002 for TREE-AX), on both entries, with standing `needs_recompute`. The base's timeouts are on record from F1b's gate and K6's B2.
- **The src-tauri suite:** 116 passed, 0 failed, from a clean `git archive 1b10121fa`. F1b's run had the same count.

## 8. Routed items

### 8.1 The dense pivot screen (declined for KF2; split out as its own slice, with an owner-facing note)

- **The finding** (plan §7):
  - The dense Cholesky charges `2i + 2` operations per pivot. The skyline charges `2(i − first_i) + 2`.
  - At the refusing rows the dense counts are 11,992 and 11,994, a screen of 8.52e-11·scale. The rows' natural-order profile counts are 16 and 14, a screen of about 1e-13·scale.
  - The predicted pivot/scale is about 3.4e-11 (CHAIN-ROT, N1000.UY) and about 2.4e-11 (TREE-AX, B500.UZ). These are predictions from the model's structure. No probe measured them.
- **A profile-based count is honest.** The dense factor's L entries before a row's first nonzero are exact ±0, so those operations round nothing (plan §7.3).
- **What a change would alter:** every dense report's published `PivotEvidence.operation_count` and `.screen` (PP:1106-1107), and so T9's 56 dense outputs and the dense part-1 envelopes.
- **The expected class changes** (dense refused, becoming published as the sparse path publishes):
  - certainly RF-LARGE-CHAIN-n01000-ROT and RF-LARGE-TREE-n01000-AX (part 2);
  - likely RF-CHAIN-T-n10-r1e-12 (R1 says it "must be solved") and RF-CHAIN-A-n10-r1e-12;
  - as candidates, RF-SKEW-T-PIN-OFF-122-r1e-12 and RF-WEAK-W-L-r1e-12.
- **KF2's own runs confirm the situation:**
  - B's part 1 shows those four r1e-12 cases, and the FX-NP-A-ulp family, refused by the dense factor;
  - part 2 shows the N10 pair refused by the dense factor, where the sparse path publishes Sensitive.
- The class disagreement with sparse remains until that slice.

### 8.2 Cancellation (an observation for the owner; routed by ROOT to T6 and T9)

- **Where cancellation is observed:** only at two cooperative checkpoints, before the solve starts (APP:1688-1692) and before publication (APP:1711-1717, where the result is discarded).
- **The solve itself** (APP:1697-1699 → PP → SA `solve_assembled_with_formation_check`, SA:688-725 → `solve_prepared`, SA:2005-2020) runs to completion.
- **Consequence:** today a cancelled N10 dense job keeps its thread and about 3.4 GB until the witness finishes.
- **After KF2** it reaches the publication checkpoint in about factor time. The dense factor, 65–188 s at 1,000 members up to K6's ceiling, is now the long step.
- KF2 adds no budget or cancellation check (Q4).

## 9. Files

| File | Change | sha256 |
|---|---|---|
| `FKS` | +57 −6: the delegation, `negative_pair_witness_counted`, the `#[cfg(test)]` module line | `2d794ee8bd481f57508f9f2884834b8061ca3643348a1e8755810fbeb3a02d7e` (base `a25b24c9…`) |
| `FK/structural/kf2_witness_tests.rs` | new, 969 lines | `91478d3ae07ebb9faa29290b885ed94ffde191277982fe9c0159615201a2959a` |
| `FKT/s11_site_table.rs` | +2: the declared row and its comment | `b15aa8b82382808da06814a4355b7c6e553e7efd292d5332510328ff2411b121` (base `d02f2bce…`) |
| `T3/IMPLEMENTATION/KF2/` | `PLAN_CHECKPOINT0.md`, `RETURN.md`, `CHANGE_RECORD.md`, `SHA256SUMS`, `_run_records/a/` (its own `SHA256SUMS` `5a316924…`), `_run_records/b/` (its own `SHA256SUMS` `247459f7…`) | see `SHA256SUMS` |

Nothing else changes: no FKP, SD, NI/SA, PP, H, app or tooling file, and no screen change.

## 10. What was not done, limits and disclosures

**Not done:**
- The dense screen change: declined and split out (§8.1).
- The plan's optional §7.2 probe, which would have measured the refusing rows' pivots. ROOT did not ask for it, so the §8.1 pivot figures are predictions.
- A re-run of the base's part 2 (per the ruling).
- Linux runs.
- Timing claims.
- The PR's gates are ROOT's: the independent review, hosted CI with the dispatch, DEC-025 with a fresh sweep target, and GEN-8.
- Native desktop witnesses.

**Disclosures:**
- **M9's exit 142** is my 600 s `perl` alarm (SIGALRM) on the mutant's test binary. M9 restores today's O(n⁴) search, so its T5 would run for hours. Six count tests had already failed when the alarm fired.
- **An extra caller:** SD calls the dense witness (SD:37, and `k1_tests.rs:371` in its tests). The brief did not name it. The signature is unchanged, and SD's suite passes (Q6).
- **Load and overlap:**
  - RV23's FK `cargo test` (from `<wt>/rv23`) and other agents' builds overlapped A's suites and B's gate.
  - During part 1, 1-minute load was 5.2–15.2 on the base run and 10.5–14.2 on the candidate run, with other agents' cargo present in 65 and 74 of 74 samples.
  - During T7, load was about 5–7.
  - Part 2's inherited load-wait rule held the first run for 180 s.
  - No timing is compared, and every time here is an observation.
- **The rounding family's discriminating power is thin:** 4 flips in 12,288 pairs. M2 is also killed by the error-order test on a corrupted system.
- **M5 on built systems** is visible only through the counts, since the plan's proof shows it changes no result there. Its result kill is the corrupted zero-coupling witness.
- **T5 and T6 use axis-aligned chains** built with FK's own frame assembly and R1's SEC_N (J = 2I). A skewed chain would need symmetry evidence, and it would change only the number of stored couplings.
- **The two part-1 `runs.jsonl` files** are not committed (606 MB each). Their hashes are in `_run_records/b/gate/uncommitted_sha256.txt`.
- **B's derived scripts:** `compare_gate_kf2.py` (from K5's comparison: the G1 formula check is skipped for a fresh base, and it writes index TSVs) and `gate_part2_cand.py` (from F1b's part-2 driver: candidate only). Each script's docstring lists its changes.
- **Scratch kept until KF2 merges,** for the reviewer: `<wt>/scratch/i20/` (including `b/`, about 8.6 GB) and `<wt>/kf2-target` (about 4.3 GB).

## Addendum 1: RV24's review (PASS) and RV24-1's tests (ROOT's "KF2: rulings on RV24's review", numerics `78950b720`)

**The review.** RV24 (`T3/REVIEW/KF2_REVIEW.md`; records `T3/REVIEW/_run_records/kf2_review/`) reviewed head `f2b8c85a2`: **PASS**, with 0 BLOCKING, 1 SHOULD-FIX and 5 NOTEs.
- ROOT asks for RV24-1's tests before merge. They are test-only.
- This addendum records the tests, the NOTEs, and a correction to B's wording. B's records are hash-bound and unchanged.

**The change.** `FK/structural/kf2_witness_tests.rs` gains 262 lines and no removals: 1,231 lines, sha256 `4a805ef758361151f837159686109aa67ad46003785a667d32e4911d8a270f29`, rustfmt-clean. No other file under `P/core` changes: FKS and the site table are unchanged from `f2b8c85a2`.

### A1.1 RV24-1: the adopted cases (`:888-1149`)

The cases are adopted from RV24's harness (`scripts/rv24_probe.rs.txt`: `rv24_adversarial_cases` and `rv24_edge_scans_every_term_count`), onto this file's reference copies and checks (`check`: bitwise equality with the reference, plus the count invariants).

- **Helpers:**
  - `skew_allowed`: a source whose asymmetry symmetry evidence of f64::MAX allows;
  - `run_corrupted`: RV24's `run_corrupt`, which prepares an identity source with the same free map, then replaces the prepared matrix, exponents and source;
  - `local_verdict_charged`: V's sums with the allowance charged for a given number of terms, computed independently of the product code.
- **`kf2_rv24_asymmetric_source_error_order`:** three 2×2 corrupted pairs on asymmetric sources, where (j,i) and (i,j) fail differently.
  - (j,i) overflows the magnitude and (i,j) underflows in the product. The result is `Range("arithmetic outside normal range")`.
  - The reverse gives `Range("product overflow or underflow")`.
  - A (j,i) that fails in `radix_scale` at exponent sum 30 gives `Range("radix scaling loses normal range")`.
- **`kf2_rv24_edge_scans_by_term_count_and_skew`:** RV24's edge scan.
  - It covers seven patterns of nonzero cells (4, 3, 3, 2, 2, 1 and 1 terms), each with a symmetric source and a skewed one (couplings k and k(1 + 10⁻³u) under f64::MAX evidence).
  - Each scan runs 80 ulps around the verdict's crossing of the last evaluated cell, with 60 repetitions and the pair first, last or in the middle.
  - That is 840 scans and 67,200 systems. Every pattern that can cross does cross, in all 60 scans.
  - **Its self-checks of discriminating power:**
    - 61 skewed pairs change verdict when the two couplings trade places, as under a swapped cell order or a transposed source read;
    - 17,050 pairs of 1–3 terms change verdict when the allowance charges four cells.
- **Debug time:**
  - the two new tests take 1.4 s;
  - the KF2 and witness tests together (`kf2_`, `krev04`, `k1_negative_witness`) take 10.2 s, against 9.3 s before: 19 passed, 1 ignored (`_run_records/rv24/kf2_targeted.log`);
  - FK's full lib suite takes 683 s at 2 threads, against 680 s at A.

### A1.2 The three mutants, killed by the committed tests (`_run_records/rv24/mutants/`)

**Method:**
- RV24's own edit texts (`rv24_mutants.py.txt`, sha256 `fd619dd8…`, unchanged), each on a clean `git archive f2b8c85a2` copy of `P/core` with the new test file overlaid.
- Each copy had its own target and was deleted afterwards. The mutants ran one at a time.
- The tests were FK's lib filtered to `kf2_`, `krev04` and `k1_negative_witness`, then the site table.

| ID | Mutation | Result | Killed by |
|---|---|---|---|
| NONE | none | 19 passed, 1 ignored; the site table 3 of 3 | — |
| RV24-M1 | the coupling cells swapped to (j,j), (i,j), (j,i), (i,i) | **killed** | `kf2_rv24_asymmetric_source_error_order`, by result (the reference gives `Range("arithmetic outside normal range")`, the mutant `Range("product overflow or underflow")`); and `kf2_rv24_edge_scans_by_term_count_and_skew`, by the verification count on a skewed 3-term pair |
| RV24-M4b | the allowance charged for four cells whatever `terms` is | **killed** | `kf2_rv24_edge_scans_by_term_count_and_skew`, by result: on a 3-term pair the reference publishes the witness and the mutant gives `Ok(None)` |
| RV24-M5 | the source cell read as `stiffness[free[b]][free[a]]` | **killed** | as RV24-M1 |

The site table passes under every mutant, since none changes an accumulation.

### A1.3 FK's full suite (`_run_records/rv24/fk_suite.log`)

417 passed, 0 failed and 1 ignored (the release-only cost test). That is lib 351 plus the integration tests 66, with 0 warnings. At A it was 415 passed.

### A1.4 RV24's NOTEs

- **RV24-N1 (recorded): two equivalent mutants.** Both survive every test, as the argument predicts.
  - RV24-M2 takes the sign from `matrix[j][i]`. `validate_prepared` requires `matrix[i][j] == matrix[j][i]` before the loop, and `>= 0.0` agrees on IEEE-equal values.
  - RV24-M9 checks the magnitude before the energy. Both checks give one message and have no side effect before the `?`.
- **RV24-N2: the routes that reach the dense witness.** It is not dense-scrutiny-only.
  - SA:2017 (`solve_prepared`) runs it after a failed factor in either mode, including SD's skyline LDLᵀ in `SparseInteractive`.
  - SA:282-306 (`solve_binary64`) and SA:1996-2003 (`solve_structural_sparse_binary64`) reach it too, and the product's nonlinear active-set loop calls them (NI `lib.rs:1990` and `:2013`).
  - SD:31-39 (`solve_structural_sparse`) reaches it.
  - The linear product routes use the pattern evidence and the sparse witness.
  - The caller list in §2 is complete and the signature is unchanged. RETURN §0 and §2's "dense scrutiny" wording should be read with these routes, and ROOT names them in the PR text.
  - **Measured by RV24:** in gate part 1 the dense witness ran in exactly the 16 dense runs B named, and in no sparse run.
- **RV24-N3: what B's runs exercise, and a correction to B's wording.**
  - **No T9 output reaches the witness.** None carries a factor pivot refusal or a `NegativeEnergy`. The 12 unresolved outputs are refused at prepare ("positive diagonal contribution absorbed"). So T9's 112 of 112 is an invariance check that does not run the new code.
  - **Part 1's 16 witness runs** see n ≤ 61 and find no pair. The path that finds a witness is exercised only by the unit tests (CHANGE_RECORD "Limits").
  - **Correction** to `_run_records/b/gate/part1/SUMMARY.txt` ("16 dense runs … publish the dense factor's refusal") and RETURN §7.2 and §8.1. In all 16 the dense factor refuses and the witness runs, but not all publish the refusal:
    - the captured-entry FX-NP-A runs (ulp .75, 1, 1.5 and 2) publish a recovered response: `MECHANICS_SOLVED`, with `SOURCE_BLOCK_RECOVERY_SELECTED` and the ordinary attempt's refusal in their diagnostics;
    - the typed FX-NP-A runs and the eight r1e-12 runs publish the refusal.
  - **A count discrepancy:** RV24 writes "the eight captured-entry FX-NP-A runs". B's index (`part1/index_cand.tsv`) shows four captured-entry FX-NP-A witness runs, all solved, and four typed ones, all `refused_blocked`. The captured runs for ulp .25 and .5 are solved as well, but do not reach the witness.
  - All 16 are byte-identical to the base, as B recorded.
- **RV24-N4: carried to the split-out dense-screen slice.** Plan §7's claims check out: the counts, the screens, the predicted pivot/scale (3.37e-11 and 2.41e-11), Lemma Z and the part-1 disagreement list. For that slice:
  1. measure the refusing rows' pivots, which are still predictions;
  2. RF-CHAIN-T/A's dense counts of 118 and 112 fit n = 61 free DOFs, as RV24's instrumented runs show;
  3. enumerate class changes by running, since a dense refusal can also become a publication where sparse refuses;
  4. the published `PivotEvidence.operation_count` would change meaning, which is a contract note.
- **RV24-N5 (recorded): the cost reproduces.**
  - RV24 measured the new search at 0.006, 0.028, 0.157 and 0.781 s at n = 750, 1,500, 3,000 and 6,006 (0.966 s in ROOT's ruling text).
  - The old search grows about ×16 per doubling.
  - RV24's extrapolation for its banded system without symmetry evidence is about 20 days at 6,000, the same order as §4.1's 31-day lower bound.
  - All of these are observations under shared load.

### A1.5 Files and records

- `FK/structural/kf2_witness_tests.rs`: sha256 `4a805ef7…`, as above.
- `_run_records/rv24/`: `kf2_targeted.log`, `fk_suite.log`, `mutants/` (NONE, RV24-M1, M4b and M5, each with its log, diff and build log), `rv24_mutants.py.txt`, `run_rv24_mutants.sh.txt` and its own `SHA256SUMS`.
- `CHANGE_RECORD.md` gains addendum 1, and the folder's `SHA256SUMS` is refreshed.

**Host:** one cargo job of mine at a time, `-j 4`, `RUST_TEST_THREADS=2`, `<wt>/kf2-target`, with the memory guard running. No timing is compared.
