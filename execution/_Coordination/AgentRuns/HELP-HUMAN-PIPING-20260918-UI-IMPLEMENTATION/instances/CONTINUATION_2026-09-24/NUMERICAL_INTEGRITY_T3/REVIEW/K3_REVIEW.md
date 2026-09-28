# RV12: independent full-diff review of slice K3 (`Wide<L>` at L = 4, 8, 16; the binary64 conversion; K4's arithmetic)

**Verdict: PASS.** There are no BLOCKING findings. There are 2 SHOULD-FIX findings (S1, S2) and 7 NOTEs (N1–N7).

**Reviewer:** RV12 (TASK, Type 2), working under `TASK_BRIEFS/RV12_K3_REVIEW.md`, `_COMMON.md` and the Mac host rules of `I8R_K1_RESUME.md`. ROOT (HELP_HUMAN) dispatched me directly and is my return path.
- I am independent of K3. I did not design, implement or test it (I11 is excluded).
- I looked for defects and fixed nothing. I made no Git writes. The implementer's worktree `<wt>/k3` was read only (plus one read-only GEN-8 run, see §3.9).
- Every build and run used `git archive` copies under `<wt>/scratch/rv12`, targets under `<wt>/rv12-target` (deleted after use), the default profiles (FK tests at opt-level 0), and no added profile.
- My run records are in `REVIEW/_run_records/k3_review/`, with their own `SHA256SUMS`.

**Abbreviations:** `FK/` = `P/core/solver/frame_kernel/`; `multi.rs` = `FK/src/structural/retained/wide/multi.rs`; `K3T` = `FK/tests/retained_wide_k3/k3_tests.rs`; `K3/` = `T3/IMPLEMENTATION/K3/`. Line numbers are at `b7e93650e`.

## 1. Revisions reviewed

- **Candidate:** PR #1041, branch `codex/piping-k3-20260928`, head `b7e93650e`. `git ls-remote` gives the same head for the remote branch, and `gh pr view 1041` gives `headRefOid` `b7e93650e…` (`checks/merge_check.txt`, `checks/ci_status.txt`). I did not run `git fetch`, because it writes refs; the objects were already present.
- **Slice:** `eb52114e9..b7e93650e`, commit by commit: `74add6078` (checkpoint A), `8cacbfaf4` (the test profile), `9aee9854c` (the guard test), `664ef5c5e` (records), `e83e22356` (profile withdrawn), `b7e93650e` (records addendum). `6e18505e3` is the branch base; its product trees (`core`, `validation`, `fixtures`) equal `eb52114e9`'s.
- **The merge `de719cbdc` adds only main's changes.** Its parents are `664ef5c5e` and main `98b1723b1`, with merge base `6e18505e3`.
  - `git diff 664ef5c5e de719cbdc` and `git diff 6e18505e3 98b1723b1` have the same sha256 (`4db98a7b…`).
  - `git diff 98b1723b1 de719cbdc` and `git diff 6e18505e3 664ef5c5e` have the same sha256 (`3e24fcdc…`).
  - Main's product changes are two test files only: FK `tests/m03_skew_scope.rs` and NI `src/structural_adapter/k1_tests.rs`.
- **`FK/Cargo.toml` at the head equals main's.** The blob is `956eeb6d…` at `b7e93650e`, `98b1723b1`, `6e18505e3` and `eb52114e9`, and the sha256 is `c124ff55…`, as RETURN addendum 1 states.
- **Source hashes at the head:** `multi.rs` `4d230364…`, `wide.rs` `96236b57…`, `mod.rs` `524bb677…`, `K3T` `ba51213c…`, `gen_wide_k3_vectors.py` `fc63433e…`; `K3/RETURN.md` `d07ce81b…`, `K3/CHANGE_RECORD.md` `18a440f3…`.

## 2. Findings

| ID | Severity | Site | Evidence | Resolution |
|---|---|---|---|---|
| S1 | SHOULD-FIX | `K3/RETURN.md` §14 ("K4 interface") and §15 ("For K4's brief"); `K3/CHANGE_RECORD.md` ("K4's arithmetic") | **The K4 interface lacks one piece that §4.1.2 and §4.1.4 require: a single correct rounding of an exact multi-term sum of p-bit terms.** The design forms K entries (§4.1.2 item 4), the reduced right-hand side (item 6), the p + 64 residual (§4.1.4 item 2), and also reactions and recovery sums (§4.1.5) and combinations (§4.1.1), each as "one exact expansion … rounded once to p". K3 supplies what builds the expansion (TwoSum, TwoProduct). It does not supply what rounds the expansion once: `round` takes one value, and `from_integer` takes one integer. The binary64 route cannot carry these terms either. `ExactAccumulator` has quantum 2⁻²¹⁴⁸ and binary64 inputs, and the split into binary64 terms stays at L = 2 (Q6). A naive fold is wrong. In my demo (`oracle/fold_demo.*`) at p = 128 the terms are e₁ = 1 + 2⁻¹²⁷, e₂ = 2⁻¹²⁸ and e₃ = −2⁻⁴⁰⁰. Rounded once, the sum is 1 + 2⁻¹²⁷. Folded at p in either order, it is 1 + 2⁻¹²⁶. So K4 would have to write a correctly rounded expansion sum, or a wide integer accumulator feeding `from_integer`, which is arithmetic that Q3 meant to keep out of K4. RETURN §15 does not mention this gap. §4.1.6 is expressible with the delivered API: `widen`, then TwoSum at the wider width for an exact difference, then `mul_pow2(-64)`, `from_f64` and `cmp_value`. | Record the gap in RETURN §15 (a records addendum), and ROOT assigns it in K4's brief. Either K4 writes the primitive in its own `retained/` file, tested against `Fraction` (including ties decided by a far tail), or it becomes a declared `multi.rs` write-set extension for K4. No K3 code change is needed for K3 to merge. |
| S2 | SHOULD-FIX | `K3T` (value API of `Wide<4/8/16>` and the TwoSum/TwoProduct precondition): `multi.rs:898-904` (`mul_pow2`), `:908-932` (`cmp_value`), `:606` and `:628` (the `fits` checks) | **Three of my mutants survive FK's whole suite (227 passed, 0 failed):** RV1 (`cmp_value` orders two negative values as if they were positive), RV2 (`mul_pow2` scales by 2^(2k)), and RV4 (TwoSum checks only its first operand's precision).<br>**The code is correct:** my oracle finds 0 mismatches on 3,624 `mul_pow2`, `cmp_value` and `fits_precision` operations and on 180 refusal cases. But no test pins these behaviours:<br>- `cmp_value` is tested only on a mixed-sign pair and on equal magnitudes (`K3T:1082-1089`);<br>- `mul_pow2` only for range refusals and as an input to self-consistent checks (`K3T:988-992`, `:2181-2186`);<br>- the refusal only for `two_sum(wide, _)` and `two_product(_, wide)` (`K3T:2018-2019`).<br>**These are K4-facing:** the stop rule's 2⁻⁶⁴ scaling and its comparisons (§4.1.6), and the error-free precondition (§4.1.2 item 4). The oracle-sensitivity run detects all three (504, 1,010 and 90 mismatches). | Add value assertions to `K3T`:<br>- `cmp_value` on same-sign pairs of different magnitude, both signs and both orders;<br>- `mul_pow2(k)` for k ≠ 0, compared with `from_parts`;<br>- `two_sum(_, wide)` and `two_product(wide, _)` refused.<br>Then confirm RV1, RV2 and RV4 are killed. A tests-only follow-up, or K4's first commit, would do. |
| N1 | NOTE | `multi.rs:496-505` (far subtraction: `(big − small − 1) + (1 − f)`) | My RV5 drops the (1 − f) tail after the borrow. It is killed by one test only: the L = 2 cross-check on K3a's p = 128 stream (`K3T:711`). No L = 4, 8 or 16 vector or stream has a far subtraction whose tail decides a tie (p = 64L, big odd, small all ones at gap 64L + 1). The generic core is shared, so the kill is valid. My 2,250 such cases at L = 4, 8 and 16 pass on the candidate and detect RV5 (150 mismatches). RV3, the borrow dropped entirely, is killed widely (6 tests). | Optional: add the class to the generator's targeted set (`oracle/tail_decides.py.txt` constructs it). |
| N2 | NOTE | `multi.rs:1081-1086` (`AttemptWork::record`), `:1123` (`WideContext` derives `Clone`); RETURN §14 ("record each context once") | `record` adds a context's cumulative counts. There is no reset or `take`, and a cloned context carries its counts. So recording a context twice, or recording a clone and its original, double-counts the attempt's work. The counts are deterministic, so the effect is reproducible, but it is silent. | K4's brief states the pattern (one context per width and attempt, recorded once when the attempt ends). An optional `take` would make misuse impossible. |
| N3 | NOTE | `multi.rs:987-998`; RETURN §7 ("taken from the algorithm's step count") | `from_integer` is charged as `Round`, 2L limb-multiply equivalents, whatever the magnitude's length. The ledger projection passes 68 limbs, so its real step count is about 68 + 2L limb operations. The table is deterministic and stated, so budgets remain reproducible, but that sentence does not hold for this kind. | Qualify the sentence in RETURN §7, or have K4's brief account for it when ROOT sets the limits (§4.1.7). |
| N4 | NOTE | RETURN §15 ("`from_integer` takes its limbs and the 2⁻²¹⁴⁸ quantum as they are") | `ExactAccumulator` holds separate positive and negative magnitudes (`exact_sum.rs:45-48`), not one signed integer. K4's read accessor must first net them into a sign and a magnitude, using the accumulator's private `compare` and `subtract`. Only then can `from_integer` take the limbs. | State this in K4's brief, beside the Q4 accessor. |
| N5 | NOTE | §4.11 ("at least 10⁶ operations per precision"); brief item D; ROOT's checkpoint-0 ruling 8 | The default suite has 10⁶ operations at p = 256, 512 and 1024, as the brief specifies. K4 will also run p = 128 and 192 at L = 4, 320 at L = 8 and 576 at L = 16. At those precisions the default suite's exercise at their own width is the mixed streams, whose p distribution gives about 1.4·10⁴, 0.7·10⁴ and 0.4·10⁴ operations respectively. The generic core also runs 1.2·10⁶ operations at p ≤ 128 at L = 2 (K3a's streams). My oracle adds several thousand at each of these precisions (`oracle/class_stats_all.txt`). | None required. If ROOT reads §4.11 per W1 working precision, K4's brief can add a full-count stream at p = 128 on L = 4. |
| N6 | NOTE | `multi.rs:489-492` (`lost` in the carry branch of addition) | The bit shifted out on a carry is always zero. A carry out of the 2L-limb window needs an exponent gap below 64L, and then window bit 0 is empty, because the small operand's lowest bit lands at 64L − gap > 0. It is harmless. A mutant that drops it is equivalent, so it cannot be killed. | None. |
| N7 | NOTE | `K3T:1809-1816` (`subnormal_relative_precision_is_rounded_upward`) | The "least upper value" check compares with (m − 1)·2^q. That is not the predecessor when m = 2⁵² (a result just after a carry into the next binade), where the check is weaker. The same test's comparison with K2b's formula covers the case. My oracle checks the true predecessor (`nextafter`) on every subnormal outcome: 92,937 of them, 0 mismatches. | None required. |

## 3. Checks (the brief's items 1–9)

### 3.1 `Wide<2>` untouched, and K-D5's published-byte surface safe

- **`wide.rs` against `eb52114e9`** (`checks/wide_and_mod_diff_vs_eb52114e9.txt`). Only the lines ROOT approved at checkpoint 0 change:
  - the module-doc pointer;
  - `pub(crate) mod multi;`;
  - the appended `WideError::OperandPrecision`, with its doc comment and its appended `Display` arm ("retained operand exceeds the working precision");
  - the allowance changes:
    - `NotNormalized` loses its allowance, because `multi`'s `from_parts` constructs it;
    - six items are relabelled "K3a API, no caller yet (reviewed at T3 close)";
    - `atan_positive` is relabelled "later-slice API (W1c; K3 Q6)";
    - `rounded_operations` and `work` are relabelled "test-only: … (K4 counts with multi::AttemptWork)", which ROOT accepted at B.
  - No `impl Wide<2>`, `round_pack`, `U256`, `WideArith`, `WorkCounter`, `Debug` or existing `Display` line changed.
  - `mod.rs` changes in comments only.
  - K3a's `FK/tests/retained_wide/**`, `formation_check.rs`, `exact_sum.rs`, `structural.rs`, `lib.rs` and `FK/Cargo.lock` are unchanged since `eb52114e9`.
- **K3a's tests and generator.** All 19 of K3a's tests pass in FK's full run on the head archive. `gen_wide_vectors.py --check` gives OK for 6 of 6 files and reproduces K3a's digests: p128 `b568d2c0…`, mixed `f8b008bc…` (`head_tests/k3a_gen_wide_vectors_check.log`).
- **The `Display` pin** (`K3T:430-490`) covers every existing string byte for byte. That is 12 strings: the nine unit variants, `InvalidPrecision(7)`, and the three `Accumulator(SumError::…)` forms, checked against `exact_sum.rs:31-39`. It also pins the new variant's string and its `Debug` token.
- **T9, spot-checked at the final head (Mac-only)** (`t9/`).
  - **Harness:** S11-K's `fixdiff_main.rs` (`ec089c1d…`) and ROOT's harness lock (`1c69935d…`), built `--release --offline --locked` against a `git archive` of `b7e93650e` (core, fixtures, validation).
  - **Results:**
    - 112 outputs;
    - 112 of 112 byte-identical to ROOT's Mac main hashes (`scratch/calib/fixdiff/sha_native.txt`);
    - 112 of 112 identical to I11's candidate list (built at `74add6078`);
    - 6 `ERR` and 0 `PANIC`, as I11 recorded.
  - **Why this covers the head:** the product difference between `74add6078` and `b7e93650e` is test files only (`checks/merge_check.txt`).
  - **Not compared** with any Linux record.

### 3.2 The arithmetic, against my own oracle

**The oracle** (`oracle/rv12_oracle.py.txt`) is mine, written without reading K3's generator.
- Values are dyadic integers (n, k), so no power 2^k is ever materialized, and exponents near ±2⁶² are exact.
- Rounding is to nearest, ties to even, at p bits, from the exact value:
  - + − × are computed exactly;
  - ÷ is a long quotient with the remainder as sticky;
  - √ uses `math.isqrt` on a scaled radicand with at least 2p + 6 bits, and an exactness test.
- An addend more than 64L + 16 binades below the other is replaced by ±2^(lead − 64L − 12). That has the same effect for every p ≤ 64L, and it keeps the arithmetic finite at gaps near 2⁶³.

**The probe** (`oracle/rv12_probe.rs.txt`) is a `#[cfg(test)]` module added only to a scratch copy of FK. It calls K3's `pub(crate)` API: `WideContext` and the value methods.

**Results:**
- **1,023,720 operations, 0 mismatches** (`oracle/check_all.txt`), over 30 + 1 seeded random batches, 6 targeted batches and the named boundaries. Per operation, summed over L = 4, 8 and 16: add 158,773; sub 110,358; mul 54,335; div 84,956; sqrt 68,462. Also TwoSum 31,741, TwoProduct 28,277, narrowing and re-rounding 33,759, `from_integer` 23,250, widening 3,720, `from_f64` 46,469 and context construction 558.
- **The classes exercised** (`oracle/class_stats_all.txt`, summed over widths):
  - **ties:** add/sub about 22,000, mul 21,616, div 21,479, sqrt 5,763 (exact squares of p + 1-bit odd roots), TwoSum 6,955 and TwoProduct 3,195;
  - **carries into the next binade:** 50,000+;
  - **far sticky bits:** a tail beyond the 2L-limb window, or a lone bit in limb 0, in add/sub, mul, narrowing and `from_integer`;
  - **massive cancellation:** 80,439 add/sub results at least 60 binades below both operands, and 406 exact zeros;
  - **near-exact ÷ and √:** products q·b ± 1 unit in the last place, and r² ± 1;
  - **exponent extremes:** 4,281 `ExponentRange` refusals, including results exactly at ±2⁶², carries past 2⁶², and ±2⁶² sums;
  - **precisions:** every limb boundary 64k − 1, 64k and 64k + 1, W1's 53, 128, 192, 256, 320, 512, 576 and 1024, p = 2 and 3, and random p.
- **Hard constructed cases, all agreeing:**
  - 2,250 far subtractions whose tail decides a tie at p = 64L (`oracle/check_tail.txt`);
  - 3,624 `mul_pow2`, `cmp_value` and `fits_precision` operations (`check_ops3.txt`);
  - 180 wide-operand refusals (`check_wide_operand.txt`).
- **Signs of zero:** IEEE 754 under round-to-nearest for + − × ÷ √, including −0 − −0 = +0, x − x = +0, 0 × −x = −0 and √−0 = −0. `from_integer` of a zero magnitude gives +0 whatever the flag (ruling 3).
- **Sensitivity** (`mutations/oracle_sensitivity.json`). The same oracle files, run through probe builds that carry one mutant each, detect RV1 (504 mismatches), RV2 (1,010), RV3 (12,544), RV4 (90), RV5 (150) and I11's M8 (7,053, plus 32 of the named boundaries). M17 is K3a's `round_pack` and outside the probe's reach, as expected.

### 3.3 The conversion to binary64

`to_binary64` (`multi.rs:715-756`) rounds once from the exact significand at the binade's quantum, max(e − 52, −1074). There is no 53-bit intermediate.

**My oracle's conversion** is a separate integer implementation. It is cross-checked, wherever the value is materializable, against CPython's correctly rounded `float(Fraction)` (integer true division), including the `OverflowError` at the midpoint. The relative precision is checked by definition: r ≥ 1/(2k), and `nextafter(r, 0)` < 1/(2k).

- **379,062 conversions at L = 2, 4, 8 and 16, 0 mismatches:** normal 199,566, subnormal 92,937, underflow 51,807 and overflow 34,752.
- **The named boundary cases** (`oracle/boundary_report.txt`, 152 cases, both signs, every width; all agree):

| Case | Outcome |
|---|---|
| ±0 | `Normal(±0.0)`, sign kept |
| 2⁻¹⁰⁷⁴ | `Subnormal`, relative precision 0.5 |
| exactly 2⁻¹⁰⁷⁵ | `Underflow`, sign kept, no value |
| 2⁻¹⁰⁷⁵ ± the smallest tail each width carries | + gives the least subnormal; − gives `Underflow` |
| largest subnormal | `000fffffffffffff`, relative precision `3ca0000000000002` |
| 2⁻¹⁰²² − 2⁻¹⁰⁷⁵ (a tie) | `Normal(2⁻¹⁰²²)` |
| 2⁻¹⁰²² − 2⁻¹⁰⁷⁵ minus the smallest tail | largest subnormal |
| subnormal ties, exact and with a far sticky bit; the double-rounding trap 2⁻¹⁰⁷⁵ + 2^−(1075+64L−1) | correct, with no double rounding |
| MAX; the midpoint 2¹⁰²⁴ − 2⁹⁷⁰ minus the smallest tail | `Normal(MAX)` |
| the midpoint itself, the midpoint plus a tail, and 2¹⁰²⁴ | `Overflow` |
| 2^(±2⁶²) | `Overflow` / `Underflow`, never a wrap |

- **The zero conventions** are as ruled (Q5, ruling 3). The exact-zero sign is kept; underflow and overflow keep their sign and return no value (`value()` is `None`).
- **Lifted binary64 values** return the same bits at every width. The outcome is subnormal exactly when the value is.
- **`relative_precision`** equals fl↑(2⁻¹⁰⁷⁵/|value|) on every subnormal, by the definition check above.

### 3.4 K4's arithmetic (Q3) and the "K4 interface"

- **TwoSum and TwoProduct are error-free.** On 60,018 cases, s = fl_p(a ∘ b) and e = a ∘ b − s exactly, with e representable at p. The cases include 10,150 ties, operands with huge exponent gaps (s = the larger operand, e = the smaller), exact results (e = +0), and zero operands.
- **Widening** is exact (3,720 cases). The compile-time `M ≥ L` guard works: `widen::<4>` on a `Wide<16>` fails to compile.
- **Narrowing and re-rounding** are the correct rounding (33,759 cases, from widths 4, 8 and 16 to widths 4, 8 and 16, with ties and far sticky bits).
- **`from_integer`** matches the ledger's quantum. There are 23,250 cases, mostly 68-limb magnitudes at 2⁻²¹⁴⁸, with ties, far sticky bits in limb 0, zero with the sign flag set, and exponents near ±2⁶².
- **The "K4 interface" (RETURN §14)** is accurate to the code: every signature matches `multi.rs` and `wide.rs`.
  - It is complete for §4.1.6's stop rule and schedule, with the Q8 exception at 1,088 bits already disclosed.
  - It is complete for §4.1.4's p + 64 re-formation at 192, 320 and 576.
  - It is complete for §4.1.2's frame formation (+ − × ÷ √ from lifted binary64).
  - **It is not complete for the exact-sum rule** (S1), and RETURN §15 omits two usage points (N2, N4).

### 3.5 No heap allocation per operation, and integers only

- `multi.rs` has no `Vec`, `Box`, `String` or `format!` in the arithmetic path. The 2L-limb intermediates are `CoreWidth::Double` arrays on the stack (`[u64; 2L]` per width), and the helpers work on slices.
- √ reads the radicand bit by bit. `from_integer` borrows the caller's slice. `Debug` allocates only when formatting.
- The only `f64` uses are `from_bits` and `to_bits` to build results, and `is_finite` in the lift. The relative precision uses u128 `div_ceil`.
- The `debug_assert!` preconditions (sticky implies at least p + 1 bits, `place` never loses a top bit) hold by construction for every caller. Addition keeps at least 128L − 1 bits, and ÷ and √ keep at least 64L + 1 bits, with p ≤ 64L.

### 3.6 The work counter and its cost table (Q9)

- **Counting.** Counts are by kind, per width (`WidthWork`), and saturating. They are charged even when the operation errs, which matches §4.1.7's "failed … work is all charged".
- **Merging.** `AttemptWork` merges across L = 4, 8 and 16 and across attempts.
- **Cost.** The limb-multiply cost is a pure function of (kind, L), pinned by `limb_multiply_cost_table_is_pinned`. It is deterministic.
- **K3a's `WorkCounter`** at L = 2 is unchanged.
- **Mutants.** I11's M16a (wrong slot) and M16b (a charge dropped) are killed.
- **Caveats:** N2 (record once) and N3 (the `from_integer` cost).

### 3.7 The mutation table

**Re-kills from clean archives** (`mutations/`; the same edits as I11's driver; at most three at once, at `-j 4`):
- **The control.** My NONE control for the filtered runs passed its 38 tests. The whole-suite control is my clean head run: 227 passed.
- **The filtered runs** use targeted test filters, as ROOT allowed.

| Mutant | Result | Behavioural kills (test @ assertion) |
|---|---|---|
| M1 round toward zero | killed | `targeted_hard_classes_…_l4/_l8/_l16` @ `K3T:856`; `carry_and_borrow_…` @ `K3T:931` |
| M4 off-by-one middle-limb shift | killed | `targeted_hard_classes_…` ×3 @ `K3T:856`; `two_sum_two_product_narrowing_…` @ `K3T:1859`; `two_sum_and_two_product_are_error_free_…` @ `K3T:1993` |
| M8 conversion double-rounds | killed | `conversion_boundary_outcomes_…` @ `K3T:1466`; `conversion_boundary_vectors_…` @ `K3T:1372` |
| M11 silent zero for underflow | killed | `conversion_boundary_outcomes_…` @ `K3T:1457`; `conversion_outcomes_follow_the_zero_convention` @ `K3T:1530`; `conversion_boundary_vectors_…` @ `K3T:1372`; `exponent_extremes_…` @ `K3T:1001` |
| M14 TwoSum e = 0 | killed | `two_sum_and_two_product_are_error_free_…` @ `K3T:2000`; `check_l_and_duplicate_…` @ `K3T:2196`; `two_sum_two_product_narrowing_…` @ `K3T:1860`; `l2_core_matches_k3a_on_k3a_targeted_vectors` @ `K3T:537` |
| M17 K3a's tie rule (L = 2 path) | killed | K3a's suite: `wide_tests.rs:419`, `:517`, `:795`, `:822`, `:952`, `:1033`; `l2_core_matches_k3a_on_k3a_targeted_vectors` @ `K3T:544`. FK's 5 `formation_check` unit tests, run in the same filtered batch, pass under M17, which confirms I11's M17 note for FK |
| P1 `[profile.test] overflow-checks = false` (whole suite) | killed | only the guard, `K3T:416` (226 passed, 1 failed) |

**My own mutants, aimed at gaps I suspected** (the whole FK suite, from a full `core/` archive):

| Mutant | Result | Detail |
|---|---|---|
| RV1 `cmp_value` (−,−) ordered as (+,+) | **survived** (227 passed) | S2 |
| RV2 `mul_pow2` scales by 2^(2k) | **survived** (227 passed) | S2 |
| RV3 far subtraction without the borrow (no decrement) | killed | 6 tests: `targeted_hard_classes_…` ×3 @ `K3T:856`; `seeded_fraction_differential_at_mixed_precision_l4` @ `K3T:1318`; both L = 2 cross-checks @ `K3T:544`, `:711` |
| RV4 TwoSum checks only its first operand | **survived** (227 passed) | S2 |
| RV5 far subtraction drops the (1 − f) tail | killed, thinly | 1 test: `l2_core_matches_k3a_on_k3a_p128_differential_and_its_digests` @ `K3T:711` (N1) |

- **No mutant was killed only by a panic or at compile time.** Every kill above is an `assert` in a test file. My classifier takes the last panic of each failure block and requires an `assert` line under `tests/`.
- A first attempt at the whole-suite mutants did not compile, because the archive held FK alone and `s11_site_table.rs` reads sibling crates. It is recorded in `mutations/first_attempt_fk_only_archive/` and was re-run with the whole `core/` tree. It is not a kill.
- **I11's table.** Its O0 summary (`K3/_run_records/q7_reversed/mutations/summary.json`) and its checkpoint-C summary have identical behavioural kill sets for all 25 source mutants. M9's two extra failures are not counted in either.

### 3.8 The Q7 history

- **The records state the tried and withdrawn profile honestly.**
  - CHANGE_RECORD lists `8cacbfaf4` as "a test profile, since withdrawn", and gives the `powi` constant-folding cause.
  - RETURN addendum 1 says what it supersedes (§8, the §2 row, the §13 P1/P2 definitions and "with the committed profile", and §9's FK line). It keeps the earlier logs as they were.
  - I11's part is stated plainly: the checkpoint-B checks could not see the effect on a base without the skew pin.
- **`FK/Cargo.toml` equals main's** (§1). `e83e22356` removes exactly the nine lines `8cacbfaf4` added, and rewords the guard's doc comment only (4 comment lines; the kill-site lines `:416` and `:420` are unchanged).
- **The opt-level 0 evidence is real.**
  - Every one of the 32 logs under `K3/_run_records/q7_reversed/` shows `test` profile `[unoptimized + debuginfo]`.
  - Their FK run is 227 passed, and the lists `fk_{base_eb52114e9,main,cand}_tests.txt` give 179, 184 and 227.
  - My own O0 run of the head archive gives 227 passed, 0 failed: lib 197, `k1_k2a_interaction` 3, `k2a_checked_formation` 13, `m03_skew_scope` 5, `s11_site_table` 3, doc 6. It includes K3's 43 tests (`head_tests/fk_head_full_o0.log`). The observed wall time was about 235 s.
  - The O0 NONE control is 227 passed. Each re-killed sample above behaves as I11 recorded.
- **The lesson.** K3's tests and `multi.rs` call no function of unspecified precision. There are no `powi`, `powf`, `exp*`, `ln*`, `log*`, trigonometric, hyperbolic, `hypot` or `cbrt` calls. Only `sqrt`, `mul_add` and `next_up` appear, all exactly specified.
- **Hosted CI on `b7e93650e` is green.** The "Numerical cargo suite" ran from 09:37:33 to 09:48:28 UTC, about 10.9 minutes of its 45 (`checks/ci_status.txt`). That figure is for ROOT's merge record (Q7 condition 5).

### 3.9 Records and hygiene

- **`K3/SHA256SUMS`:** 281 entries, all OK, covering every file in the folder except itself, with no extra or missing entries.
- **The test-data `SHA256SUMS`:** 7 of 7 OK.
- **GEN-8 passes on the head:** 1 passed, 10 deselected. It was run from `<wt>/k3` at `b7e93650e` with `PYTHONDONTWRITEBYTECODE=1` and `-p no:cacheprovider`, and `git status` was clean before and after.
- **No machine paths, no model identifiers.** In K3's source, tests and records at the head, there are 0 files with a machine path (GEN-8's pattern shape) and 0 with a model or vendor name. The placeholders used are `<wt>`, `<scratch>`, `<VENV>` and `<home>`.
- **T9 is stated as Mac-only** in CHANGE_RECORD, RETURN §10 and the T9 summary, and never compared with Linux.
- **The dead-code labels are truthful.**
  - `multi.rs` has 33 per-item allowances, all "K4 API", and `wide.rs` has 12. There is no module-wide or `cfg_attr` allowance.
  - None of the relabelled K3a items has a non-test caller in `FK/src`. Only `formation_check_tests.rs` calls `work()` and `rounded_operations()`.
  - A fresh non-test FK build gives no warnings.
- **The test data is 4,912,990 bytes** in the seven vector files, within ROOT's "about 5 MB" (ruling 6). The 1,000-record samples are 2.9 MB of that.
- **The digests cover every record.**
  - Each stream has a digest per 10⁵-record chunk and a whole-stream sha256. The Rust tests regenerate every operand and hash every record (`K3T:1247-1334`, `:1606-1652`).
  - K3's `gen_wide_k3_vectors.py --check` gives OK for 8 of 8 files, and reproduces all ten stream digests listed in RETURN §11.
- **Formatting.** `rustfmt 1.9.0-stable --check` is clean on `wide.rs` (with `multi.rs`) and on `K3T`. `git diff --check 98b1723b1 b7e93650e` is clean.

## 4. What I ran

All cargo runs used `RUSTUP_TOOLCHAIN=1.97.1`, `RUSTUP_AUTO_INSTALL=0`, `CARGO_INCREMENTAL=0` and `--offline --locked`, with rustc and cargo 1.97.1 on `aarch64-apple-darwin` (macOS 26.6.2).
- **Limits:** `-j 8` for single jobs and `RUST_TEST_THREADS=4`. At most two of my cargo jobs ran at once, outside the mutant batches, and at most three mutants at once at `-j 4`.
- **Python:** 3.13.14 from `<VENV>`, standard library only.
- **The memory guard** did not fire. Other agents shared the host; the load average reached about 30 during the mutant batch.

| Run | Result |
|---|---|
| FK full suite, head archive, opt-level 0 | 227 passed, 0 failed |
| FK non-test build, fresh target | no warnings |
| `gen_wide_vectors.py --check` (K3a); `gen_wide_k3_vectors.py --check` (K3) | 6/6 OK; 8/8 OK |
| RV12 oracle through the probe: seeded batches, targeted batches, named boundaries | 1,023,720 operations, 0 mismatches |
| `mul_pow2`, `cmp_value`, `fits_precision`; tail-decides-tie; wide-operand refusal | 3,624 + 2,250 + 180, 0 mismatches |
| Oracle sensitivity (probe builds with RV1–RV5, M8, M17) | all RV mutants and M8 detected; M17 out of the probe's reach |
| Mutants M1, M4, M8, M11, M14, M17 (filtered) and P1 (whole suite) | all killed at assertions |
| My mutants RV1–RV5 (whole suite) | RV3 and RV5 killed; RV1, RV2 and RV4 survive (S2) |
| T9 at the head (release harness, Mac-only) | 112/112 equal to ROOT's Mac main hashes |
| K3 records `SHA256SUMS`; the test-data `SHA256SUMS`; GEN-8; the hygiene greps | all pass (§3.9) |
| Merge identity and `FK/Cargo.toml` | as §1 |

The scripts, logs and results, with their hashes, are in `REVIEW/_run_records/k3_review/`, which has its own `SHA256SUMS`.
- The operation files (about 270 MB) and the probe outputs (about 137 MB) are not committed. They regenerate from the recorded seeds (`README.txt`), and their sha256 values are in `oracle/ops_and_outputs_sha256.txt`.
- All my targets are deleted, and so are the mutant and probe copies.

## 5. What I did not check

- **The other 38 manifests of CI's cargo profile.** I did not re-run them. K3 changes only FK. I rely on I11's per-test comparison at `8cacbfaf4`, on the fact that the merge and the revert add only tests and a manifest change that never applied outside FK, and on ROOT's DEC-025 sweep and hosted CI.
- **K-D5's NI and PP suites at the head.** I did not run them. `Wide<2>` is byte-identical, and T9 at the head is 112 of 112.
- **The T9 base.** I did not rebuild it. I compared the head with ROOT's Mac main hashes, which I11's records show equal to its base build.
- **Most of I11's mutants.** I re-killed a sample of 7 of the 27, and read the rest from I11's summaries.
- **Formal proofs.** I found no counterexample to TwoSum's error-freeness, but I did not prove it for this arithmetic. I rely on Knuth's theorem, which holds for binary round-to-nearest with no underflow in this ±2⁶² exponent range, and on 31,741 checked cases.
- **K4's design** beyond the interface questions in §3.4.
- **Timing.** I make no performance claim. The wall times quoted are observations.
- **Linux.** Nothing here was compared with Linux records.

## Delta check at 2511f5a3c

**Delta verdict: PASS.**
- S2 and N1 are resolved.
- S1 is now stated accurately in the records, and ROOT assigns the primitive in K4's brief.
- N2–N5 are recorded as notes for K4's brief. N6 and N7 needed no action.
- There is one new NOTE, D1.
- The review's verdict stands: PASS, with no BLOCKING findings.

**The delta:** `b7e93650e..2511f5a3c` on `codex/piping-k3-20260928`.
- `e62837f7e` is tests only: two new tests in `K3T`, a `taildecides` class in the generator, 32 vectors appended to each `targeted_l{4,8,16}.txt`, and three changed `SHA256SUMS` lines.
- `2511f5a3c` is records only: RETURN addendum 2, CHANGE_RECORD, `K3/SHA256SUMS`, and 22 new files under `_run_records/rv12_fixes/`.
- The remote head is `2511f5a3c` (`git ls-remote`), and `e62837f7e`'s parent is `b7e93650e`.
- I did not fetch. The objects were already present.

**Records:** `REVIEW/_run_records/k3_review/delta_2511f5a3c/` holds 28 files with their own `SHA256SUMS`. The review folder's `SHA256SUMS` gains those 28 entries, and its 80 earlier lines are unchanged.

### D.1 Checks

**No arithmetic change** (`checks/delta_checks.txt`).
- `multi.rs`, `wide.rs`, `mod.rs` and `FK/Cargo.toml` have the same blobs at `2511f5a3c` as at `b7e93650e`: `07eb3fc1…`, `ddc41ca7…`, `e11ac446…` and `956eeb6d…`.
- Nothing under `FK/src`, `FK/Cargo.lock` or K3a's `tests/retained_wide/` changed.
- The only product paths changed are the six files under `FK/tests/retained_wide_k3/`.
- In the records, only `CHANGE_RECORD.md`, `RETURN.md` and `SHA256SUMS` are modified. The other 22 changes are new files under `rv12_fixes/`, so no earlier run record changed.

**Earlier vectors byte-identical** (`checks/vectors_prefix_check.txt`).
- In each `targeted_l{4,8,16}.txt`, the first 785, 1,589 and 3,197 lines are byte-identical to the old file. The appended 32 lines per width are all `taildecides`.
- `conversion.txt`, `eft.txt`, `differential.txt` and `differential_sample.txt` have unchanged blobs.
- The vector `SHA256SUMS` changes only in its three targeted lines, and checks 7 of 7 OK on the archive.
- `gen_wide_k3_vectors.py --check` gives OK for 8 of 8, and all ten stream digests are unchanged. K3a's `gen_wide_vectors.py --check` gives OK for 6 of 6.
- The vector data is now 4,959,650 bytes, within ROOT's "about 5 MB".

**The new vectors, against my own oracle** (`checks/taildecides_oracle.txt`).
- All 96 `taildecides` vectors agree with the review's `rv12_oracle.py`.
- 24 of them, the 8 per width at p = 64L and gap 64L + 1, are ones where RV5's arithmetic (the tail dropped) gives a different result.
- At p = 64L the expected result equals big in 16 of 16 vectors per width, as addendum 2 states.

**The new tests are sound.**
- `cmp_value_and_mul_pow2_pin_order_and_scale_at_every_width` covers:
  - same-sign pairs of different magnitude, both signs and both orders, including one-unit neighbours and a carry into the next binade;
  - `mul_pow2(k)` for k ≠ 0, against `from_parts` and against a correctly rounded product by the binary64 2^k (built from bits), at p = 64L;
  - large k, against `from_parts`.
- `two_sum_and_two_product_refuse_a_wide_operand_in_either_position` builds exactly p bits and p + 1 bits at p = 53, 64 and 64L − 1. It checks refusal in both positions for both operations, and acceptance at exactly p bits.
- The coverage assertion for `taildecides` at 64L and 64L − 1 is in `targeted_classes` (`K3T:900-907`).

**The four mutants, re-run from clean archives of `2511f5a3c`** (`mutations/`).
- I used my review patches verbatim. I11's `rv12_fixes` patches are the same four edits.
- The NONE control ran first: 8 passed.
- Test filters were aimed at the new tests. RV5 ran with only the L = 4, 8 and 16 targeted tests, so K3a's L = 2 cross-check cannot be its kill.

| Mutant | Result | Behavioural kill (test @ assertion) |
|---|---|---|
| RV1 `cmp_value` (−,−) ordered as (+,+) | killed | `cmp_value_and_mul_pow2_…` @ `K3T:1149` (`small.neg().cmp_value(&large.neg())`) |
| RV2 `mul_pow2` scales by 2^(2k) | killed | `cmp_value_and_mul_pow2_…` @ `K3T:1172` (against `from_parts`) |
| RV4 TwoSum checks only its first operand | killed | `two_sum_and_two_product_refuse_…` @ `K3T:1214` (`c.two_sum(&one, &wide)`) |
| RV5 far subtraction drops the (1 − f) tail | killed at every width | `targeted_hard_classes_…_l4`, `_l8` and `_l16` @ `K3T:856`, first failing on a `taildecides add` vector at p = 64L at each width |

Every kill is an `assert` in a test file. None is a panic or a compile failure. The kill sites match I11's addendum 2.

**S1 is stated accurately** (RETURN addendum 2, §15's pointer, and CHANGE_RECORD "Remaining → K4").
- It names the same D1 sites (§4.1.1, §4.1.2 items 4 and 6, §4.1.4 item 2, §4.1.5).
- It says why `round`, `from_integer`, `ExactAccumulator` and the L = 2 split do not provide the rounding.
- It gives the counterexample with the correct mechanism: the tie at e₂ is decided by e₃'s far tail.
- It states both remedies (an expansion sum, or a wide integer accumulator feeding `from_integer`) and requires a `Fraction` test with far-tail ties.
- It leaves the assignment to ROOT in K4's brief.
- Its summary of what the API does cover matches my §3.4.
- N2–N5 are carried faithfully. N5's "K4 adds 10⁶-operation streams at the precisions it uses (per ROOT)" is recorded as ROOT's decision.

**FK at opt-level 0 on the head** (`head_tests/`, archive of `2511f5a3c`, no profile).
- 229 passed, 0 failed: lib 199, `k1_k2a_interaction` 3, `k2a_checked_formation` 13, `m03_skew_scope` 5, `s11_site_table` 3, doc 6.
- That is the review's 227 plus the two new tests. The observed wall time was about 236 s.
- A fresh non-test build has no warnings. `rustfmt --check` is clean.

**Records hygiene** (`checks/`).
- **`K3/SHA256SUMS`:** 303 of 303 entries OK, and the listed set equals the file set.
- **Machine paths and model names:** 0 files in K3's source, tests and records at `2511f5a3c`.
- **Whitespace:** `git diff --check b7e93650e 2511f5a3c` is clean.
- **GEN-8:** passes on `<wt>/k3` at `2511f5a3c` (1 passed, 10 deselected), with `git status` clean before and after.
- **Hosted CI on `2511f5a3c`:** was still running when I checked (`checks/ci_status.txt`). Its result and the numerical job's time are for ROOT's merge record.

### D.2 Findings of the delta

| ID | Severity | Site | Evidence | Resolution |
|---|---|---|---|---|
| D1 | NOTE | `K3/CHANGE_RECORD.md` "Checks" (line 76: "Targeted and class tests (RETURN §6), 43 of 43"; line 90: "… plus K3's 43") | CHANGE_RECORD now says `k3_tests.rs` has 45 tests and that FK is 229 with the follow-ups. Two nearby lines still carry the pre-follow-up count 43. RETURN's body keeps its earlier figures by design, and addendum 2 gives the new ones, so RETURN is consistent. CHANGE_RECORD is the PR record, so a reader meets both counts there. | Optional: "45 of 45" and "plus K3's 45" (or "43, then 45 with RV12's follow-ups") in the PR record. It does not block. |

### D.3 What I did not check in the delta

- **The 38 other manifests and T9.** I did not re-run them. The delta changes only FK test files and records; no product source changed, so T9's outputs cannot move.
- **The whole-suite form of the four mutants.** I used targeted filters, as ROOT allowed. I11's `rv12_fixes` records have them over the whole suite, with the same kill sites.
- **Hosted CI's final result** on `2511f5a3c`.
