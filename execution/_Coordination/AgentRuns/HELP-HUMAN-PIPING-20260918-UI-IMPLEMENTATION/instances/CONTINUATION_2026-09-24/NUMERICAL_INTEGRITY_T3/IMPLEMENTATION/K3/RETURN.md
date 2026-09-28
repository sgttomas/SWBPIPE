# I11 return: slice K3 (the rest of W1's arithmetic)

> **Addendum 1 (Q7 reversed) supersedes §8 and the `FK/Cargo.toml` row of §2.** FK has no test profile. The evidence was re-run at opt-level 0 on the merged tree `de719cbdc`. See the end of this file.

**Status: complete, with no stop.**
- `Wide<L>` is added at L = 4, 8 and 16 beside K3a's unchanged `Wide<2>`.
- The correctly rounded conversion to binary64, with its outcome, is added at every width.
- K4's arithmetic and per-width work counts are added.
- **K3a's `Wide<2>` code is byte-identical.** The only `wide.rs` changes are the lines ROOT approved.
- **No published byte changes:** T9 is 112 of 112 byte-identical (Mac-only).
- **Suites:** the 39-manifest suites equal the Mac baseline of main, apart from K3's added tests.
- **Mutations:** all 25 source mutants and both profile mutants are killed at test assertions.

**Where:**
- Branch `codex/piping-k3-20260928` in `<wt>/k3`, from main `6e18505e3`.
- Its piping tree equals that of `eb52114e9`, ROOT's Mac baseline.
- Checkpoints committed by ROOT:
  - A: `74add6078`;
  - B, the test profile: `8cacbfaf4`;
  - after C: the guard test, then these records. ROOT commits both and records their revisions.

**Abbreviations:** `P/` = `projects/chirality-piping/`; `FK/` = `P/core/solver/frame_kernel/`; `T3/` = the NUMERICAL_INTEGRITY_T3 folder; `K3T` = `FK/tests/retained_wide_k3/k3_tests.rs`.

## 1. Brief, basis, delegation and rulings

- **Brief:** `T3/TASK_BRIEFS/I11_K3_IMPLEMENTATION.md`, with its "ROOT rulings for this slice".
- **Basis read in the brief's order**, with hashes checked where pinned:
  - Root `AGENTS.md`, `agents/AGENT_TASK.md`, `_COMMON.md`, and `I8R_K1_RESUME.md` "The Mac host".
  - D1 `DESIGN.md` revision 5a.2 (sha256 `fb62ef4a…`, verified): §4.1.1, §4.1.2 items 4–6 and the exact-sum rule, §4.1.4, §4.1.6, §4.1.7, §4.1.8, §4.11, §5 item 7, §6 (the K3a, K3 and K4 rows), §7.3 and §7.4.
  - `ROOT_SELECTION_DESIGNS.md`, the order and C1.
  - `ROOT_RULINGS_V1.md`: D5_CHECK item 3; "K3a arctangent and host disk"; "K2b: rulings on I10's checkpoint-0 plan" item 3.
  - K3a's records, all verified: `IMPLEMENTATION/K3A/RETURN.md` (`3413a94a…`), `CHANGE_RECORD.md` (`a94c121e…`), `K3A_MERGE/RECORD.md` (`ddad9baa…`) and `REVIEW/K3A_REVIEW.md` (`619e4311…`); and the briefs `I2_K3A_IMPLEMENTATION.md` and `I2_K3A_RV2_FIXES.md`.
  - `IMPLEMENTATION/KD5/RETURN.md` item 8 (the 13 allowances).
  - The code at the base: `retained/wide.rs` (`9f97ea03…`, verified), `mod.rs`, `FK/tests/retained_wide/**`, `formation_check.rs` (:32, :145, :229), `formation_check_tests.rs:11`, `FK/exact_sum.rs`, `PP:1110-1125` and :4441, and `SA:174` and :469.
  - `OWNER_DIRECTION.md`, DEC-025 on the Mac.
  - `I10_K2B_IMPLEMENTATION.md`, for coordination. I also read I10's `Representability` in `<wt>/k2b` read-only, to align the conversion's convention (Q5).
- **Delegation mechanism:** a background subagent of ROOT's (HELP_HUMAN) session, started with the brief and resumed by ROOT's messages at each checkpoint. I did not delegate. I made no Git writes and no index operations; ROOT committed and pushed.
- **ROOT's rulings applied:**
  - the brief's rulings 1–10;
  - "K3: rulings on I11's checkpoint-0 plan" (`d2df479f3`): the child module and its test module; dead-code labels naming the real consumer; `from_integer` of zero gives +0; the exact-−0 difference allowed; the literal and scaled §7.3 analogues; about 5 MB of test data; bit-serial ÷ and √;
  - "K3: Q7, the differential's debug cost" (`973438aa7`): the test profile;
  - ROOT's relayed checkpoint-B and checkpoint-C rulings: the `rounded_operations`/`work` relabel accepted; the P1/P2 guard test; the M17 note.

## 2. Files and line counts (against `6e18505e3`)

| File | Change | Lines |
|---|---|---|
| `FK/src/structural/retained/wide/multi.rs` | new: the child module (ruling 10, approved) | 1,233 |
| `FK/src/structural/retained/wide.rs` | +21 −13: `pub(crate) mod multi;`, the appended variant `OperandPrecision` and its appended `Display` arm, the module-doc pointer, and the allowance relabels (§5) | 997 |
| `FK/src/structural/retained/mod.rs` | +12 −3: comments only | 24 |
| `FK/Cargo.toml` | +9: `[profile.test]` (Q7, §8) | 20 |
| `FK/tests/retained_wide_k3/k3_tests.rs` | new: the `#[path]` test module of `multi.rs`, 43 tests (42 at A, and the guard) | 2,352 |
| `FK/tests/retained_wide_k3/gen_wide_k3_vectors.py` | new: the standard-library generator, with `--check` | 969 |
| `FK/tests/retained_wide_k3/targeted_l4.txt`, `targeted_l8.txt`, `targeted_l16.txt` | new vectors | 785, 1,589, 3,197 |
| `FK/tests/retained_wide_k3/conversion.txt`, `eft.txt` | new vectors | 400, 1,242 |
| `FK/tests/retained_wide_k3/differential.txt`, `differential_sample.txt` | new: the stream manifest, and the first 1,000 records of each stream | 86, 10,000 |
| `FK/tests/retained_wide_k3/SHA256SUMS` | new: sha256 of the seven vector files, checked by a test | 7 |

- **Size:** the vector data is about 4.9 MB (the samples 2.9 MB; targeted L = 16 1.2 MB). ROOT approved about 5 MB.
- **Unchanged:** `FK/Cargo.lock` and every other manifest and lockfile. There are no dependencies.
- **Not touched:** `exact_sum.rs`, `formation_check.rs`, `structural.rs`, `lib.rs`, `sparse.rs`, `load_ledger.rs`, SA, PP, the site tables and the fixtures.
- **K3a's `FK/tests/retained_wide/**` is byte-identical.** `gen_wide_vectors.py --check` gives OK for all six files.
- The source hashes are in `_run_records/source_sha256.txt`.

## 3. How each K3 item was met (D1 §4.1.1, §4.11; the brief's "What K3 adds")

1. **The widths.** "`Wide<const L: usize>` … with L = 2, 4, 8, 16". K3a's struct is unchanged. L = 4, 8 and 16 gain the value API and `WideContext<L>` through `SupportedWidth`, implemented only for those three.
   - **Q1:** K3a's `impl Wide<2>` is untouched. The new generic core (free functions over `CoreWidth`) runs at L = 2 only in the tests, and there it is bitwise equal to K3a's `WideArith` (§6 A).
2. **The operations.** "`+ − × ÷ √`, each rounded to nearest, ties to even, at a runtime precision p ≤ 64L": `WideContext::<L>::new(p)` accepts 2 ≤ p ≤ 64L, else `InvalidPrecision(p)`. Each operation rounds the exact result once.
   - **Addition:** a 2L-limb window, with K3a's exact sticky-borrow scheme.
   - **Multiplication:** the 2L-limb schoolbook product.
   - **Division:** bit-serial, 64L + 2 quotient bits.
   - **Square root:** bit-serial, 64L + 2 root bits.
   - The ÷ and √ remainders are the sticky bit.
   - W1's precisions are covered: 128, 256, 512 and 1024, and 192, 320 and 576 for the p + 64 residual.
3. **The lift.** "Exact conversion from `f64`": `Wide::<L>::from_f64` is exact for normal, subnormal and ±0, and refuses NaN and ±∞.
4. **The conversion to binary64.** "Correctly rounded conversion to `f64`, with an explicit outcome: normal, subnormal (with its relative precision bound), underflow to zero, or overflow" (§4.1.1; §5 item 7).
   - `Wide::<L>::to_binary64() -> Binary64Outcome` for every L, **L = 2 included**.
   - **Rounded once** from the exact significand at the binary64 quantum of the value's binade: 2^(e−52), or 2^−1074 in the subnormal range. There is no 53-bit intermediate, so there is no double rounding (M8 is killed).
   - **The Q5 convention:**
     - an exact ±0 is `Normal(±0.0)` and keeps its sign;
     - `Subnormal { value, relative_precision }`, where `relative_precision` = 2^−1075/|value|, rounded upward with integer arithmetic;
     - `Underflow { negative }` only for a nonzero value whose rounding is ±0;
     - `Overflow { negative }` for a value whose rounding is ±∞, from the midpoint 2^1024 − 2^970 up;
     - neither of the last two returns a value (`value()` gives `None`).
5. **The work counter** at every width: `WidthWork` counts by kind, saturating. `AttemptWork` holds one per width (4, 8, 16), merges across widths within an attempt, and states the cost in limb-multiply equivalents (§7).
6. **Integer-only arithmetic:** u64/u128 limb operations only. The one place K3 needs a rounded-up binary64 (the relative precision) uses integer division. It also agrees bit for bit with K2b's fused-multiply-add formula (§6 C).
7. **Q3, K4's arithmetic:**
   - **Widening:** `widen::<M>()`, exact, with M ≥ L checked at compile time.
   - **Narrowing:** `WideContext::round::<M>()`, correctly rounded from any width (narrowing, re-rounding and widening with rounding).
   - **TwoSum and TwoProduct** at every width. Operands must have at most p bits (`OperandPrecision`). TwoSum is Knuth's six operations at p; TwoProduct is the exact product minus its rounding. An exact result gives e = +0.
   - **The constructor** `from_integer(negative, magnitude: &[u64], exponent)`: a correctly rounded constructor from ±magnitude·2^exponent for any little-endian length, such as `ExactAccumulator`'s 68 limbs at 2^−2148. A zero magnitude gives +0 whatever the flag (ruling 3; D1 §4.1.2).
   - **Q4:** K4 adds the `exact_sum.rs` read accessor. K3 did not touch `exact_sum.rs`.

**What K3 did not do** (the brief's list, all kept):
- no method code;
- no caller outside `retained`: the new surface has no non-test caller at all;
- no change to any `Wide<2>` result, its `Debug` format, or any existing `WideError` `Display` string: pinned by a test (§6 A) and shown by K3a's and K-D5's unchanged suites and by T9;
- no arctangent or split beyond L = 2 (Q6);
- no dependency and no lockfile change.

**The implementation constraints:**
- **No heap allocation per operation.** The 2L-limb intermediates are `<Wide<L> as CoreWidth>::Double = [u64; 2L]` on the stack; all helpers work on slices; the √ radicand is read bit by bit, never materialized. `Debug` formatting allocates only when formatting.
- **`WideError`:** one variant is appended, `OperandPrecision` ("retained operand exceeds the working precision"). No existing variant or `Display` arm changed.
- **The test module** `#[cfg(test)] #[path = "../../../../tests/retained_wide_k3/k3_tests.rs"] mod tests;` is declared from `multi.rs` (ruling 1). As a descendant of both `wide` and `multi`, it reaches K3a's `WideArith` and the new core.

## 4. Positions and conventions settled at checkpoint 0 (ROOT's `d2df479f3`)

1. **The child module** `retained/wide/multi.rs`, and the test module declared from it.
2. **Labels** name the real consumer (§5).
3. **`from_integer`** of a zero magnitude returns +0.
4. **The accumulator cross-check** also allows an exact −0 to differ: the conversion gives `Normal(−0.0)`, while the accumulator gives +0.0.
5. **The §7.3-13 and §7.3-16 analogues:**
   - The literal ones (1e80, 1e−8, −1e80), and a duplicate A, −A with a third term 2^−300 as large, are asserted lossy under the fold at p ≤ 256.
   - At p = 320 the fold keeps part of the small term, and at 512 and 1024 it is exact for check L. So those p get no fold assertion.
   - A scaled variant (a small term 2^−(p+16) as large) is asserted lost when folded at every p.
   - All of them are exact through a TwoSum expansion at every p: the components sum exactly to the small term, checked at L = 16, p = 1024.
6. **About 5 MB of data,** with 1,000 samples per stream.
7. **Q7:** see §8.
8. **For K4's brief (Q8):** see §15.

## 5. Dead code (the brief's rule, with ROOT's ruling 2)

- **The non-test FK build has no warnings,** at A and after every later change.
- **`multi.rs`:** 33 per-item `#[allow(dead_code)]`, each labelled "K4 API" (some with a qualifier: budgets, publication, the stop rule, exact assembly, the ledger projection). Each sits on an API entry point with no non-test caller.
  - The set is minimal: with every allowance removed, the non-test build gives 50 warnings, all of them those entry points or the private helpers they keep live.
  - Items with a non-test caller carry none (`OpKind::ALL`, `limb_multiply_cost`, `WidthWork::count`, `WidthWork::limb_multiply_equivalents`).
  - There is no module-wide or `cfg_attr` allowance.
- **K3a's allowances in `wide.rs`** (13 → 12):
  - `WideError::NotNormalized`: **allowance removed.** K3's `from_parts` constructs it in non-test code.
  - `Wide<2>::from_parts`, `parts`, `is_sign_negative`, `exponent`, `fits_precision`, and `WideArith::precision`: "K3a API, no caller yet (reviewed at T3 close)". K4 runs at L ≥ 4.
  - `WideArith::atan_positive`: "later-slice API (W1c; K3 Q6)".
  - **`WorkCounter::rounded_operations` and `WideArith::work`:** "test-only: K-D5 measures its cost in tests (K4 counts with multi::AttemptWork)". Before, they read "…; K4 budgets use it", which is no longer true. The change is comment-only; ROOT accepted it at B as truthful.
  - The test-only constants and `truncated_below_min_subnormal` are unchanged.
- **`mod.rs`'s comment** now says 12, and names K3's rule.

## 6. Tests (43 in `K3T`; all pass)

**A. Nothing existing moves**
- `existing_wide_error_display_strings_are_unchanged` pins all 12 existing strings byte for byte, including the three `Accumulator` forms, plus the new variant.
- `wide2_debug_tokens_and_work_counter_are_unchanged`.
- **The L = 2 core against K3a's `WideArith`, bitwise:**
  - `l2_core_matches_k3a_on_k3a_targeted_vectors`: all 2,072 of K3a's `targeted.txt`, including the TwoSum and TwoProduct lines;
  - `l2_core_matches_k3a_on_k3a_p128_differential_and_its_digests` (10^6 operations) and `…_mixed_differential_and_its_digests` (2·10^5). K3a's operands are regenerated, every result is compared with `WideArith`'s, and **K3a's recorded chunk and stream digests are reproduced from the new core's results.**
- **K3a's own 19 tests pass unchanged.** `gen_wide_vectors.py --check` gives OK for 6 of 6.
- **K-D5's tests pass unchanged:** FK's 7 `formation_check` tests; NI's 12 `kd5_tests` (NI 101 = baseline); PP's `formation_check_runtime` (5) and `f1a_tests` (7).

**B. The arithmetic at L = 4, 8 and 16**
- `p53_matches_hardware_binary64_bitwise_at_l4`, `…_l8` and `…_l16`: 10^5 random normal-range pairs per operation per width, plus ties.
- `targeted_hard_classes_match_the_fraction_oracle_at_l4`, `…_l8` and `…_l16`: 785, 1,589 and 3,197 vectors.
  - They run at every required p: {53, 128, 256, 512, 1024} ∩ [2, 64L], every limb boundary 64k − 1, 64k and 64k + 1, and 192, 320 and 576. That is 12, 24 and 48 precisions. (My checkpoint-A message said 16, 26 and 50; that was a reporting error. The tests assert the coverage.)
  - The test asserts class and operation coverage at each p: ties (add, sub, mul; div and sqrt where representable), carry, cancellation, exact and near-exact ÷ and √, and sticky, including a tail beyond the 2L-limb window and a lone bit in limb 0.
- `carry_and_borrow_run_through_every_limb_at_every_width`: all ones plus one unit (exact), and all ones plus half an ulp at p = 64L (a tie on an odd part) carry through every limb; the borrow runs back.
- `exponent_extremes_are_refused_never_wrapped_at_every_width`, including `from_integer` and narrowing near ±2^62.
- `signs_of_zero_the_lift_parts_and_the_precision_range_at_every_width`: p outside [2, 64L] is refused.

**C. The conversion to binary64 (every width, L = 2 included)**
- `conversion_boundary_vectors_match_the_fraction_oracle`: 400 vectors, with 22 boundary tags asserted at each of L = 2, 4, 8 and 16 (§12).
- `conversion_boundary_outcomes_at_every_width` and `conversion_outcomes_follow_the_zero_convention`.
- `seeded_conversion_differential_at_l2`, `…_l4`, `…_l8` and `…_l16`: 10^6 each (§11).
- `lifted_binary64_values_convert_back_to_the_same_bits_at_every_width`: about 10^5 values per width, subnormal iff the outcome is subnormal.
- `conversion_at_l2_matches_exact_accumulator_round_where_both_are_defined`: up to 2·10^5 split-exact values fed to `ExactAccumulator` and rounded once give the same bits. The only differences are overflow (`NonRepresentable`) and an exact −0 (+0.0). Underflow cannot occur for split-exact values.
- `subnormal_relative_precision_is_rounded_upward`: about 1.02·10^5 values of k, checked exactly (r ≥ 1/(2k) > its predecessor) and against K2b's formula.

**D. The seeded differentials:** 6 streams (§11).

**E. K4's arithmetic**
- `two_sum_two_product_narrowing_and_the_integer_constructor_match_the_fraction_oracle`: `eft.txt`'s 432 TwoSum, 216 TwoProduct, 432 narrowing and 162 integer-constructor vectors.
  - The constructor vectors include 144 shaped like `ExactAccumulator`'s sum (68 limbs, quantum 2^−2148), with ties, far sticky bits in limb 0, and zero with the sign flag set.
- `two_sum_and_two_product_are_error_free_on_random_operands`: s + e = x ∘ y is checked exactly at L = 16, p = 1024, with |e| ≤ ulp(s)/2; operands wider than p are refused.
- `widening_is_exact_and_round_trips` and `narrowing_rounds_ties_and_far_sticky_bits_correctly`.
- `integer_constructor_matches_exact_accumulator_at_p53`: 5,000 accumulator sums in the normal range.
- `check_l_and_duplicate_cancellation_are_exact_through_an_expansion_and_lost_when_folded` (§4 item 5).

**F. The work counter**
- `work_is_counted_by_kind_and_width_and_saturates`, `limb_multiply_cost_table_is_pinned` and `attempt_work_merges_across_widths`.

**Integrity**
- `sha256_known_answers`, `committed_vectors_match_their_recorded_sha256` and `token_round_trip_and_debug_scheme`.

**The profile guard (after C; ROOT's ruling)**
- `test_profile_keeps_overflow_checks_and_debug_assertions_on`:
  - `u64::MAX + 1` on `std::hint::black_box` operands, under `catch_unwind`, must panic;
  - `cfg!(debug_assertions)` must be true.

**Corrections during checkpoint A** (in my own hand-written expectations, not the arithmetic; every generated-vector test passed on its first run; `checkpoint_a/fast1.log` and `fast2.log` hold the failing runs):
1. **The largest subnormal's relative precision** is 1/(2^53 − 2) rounded up. The hand-written bits were wrong; the test now checks it against K2b's formula.
2. **At p ≥ 320,** V1's literal check-L sum keeps part of its small term, and at 512 and 1024 the fold is exact. The expansion check now sums the components exactly, and those p get no fold assertion (§4 item 5).
3. **3·2^(2^62 − 1) is in range.** The refusal case now uses `EXPONENT_LIMIT` itself.
4. **The accumulator cross-check** needed a dedicated band at exponent 1023 to reach overflow.

## 7. Work units (Q9): the stated cost

`limb_multiply_cost(kind, L)`, in limb-multiply equivalents. It is deterministic and taken from the algorithm's step count. It is not a timing claim.

| Kind | Formula | L = 4 | L = 8 | L = 16 |
|---|---|---|---|---|
| add, sub, round (narrowing, the constructor) | 2L | 8 | 16 | 32 |
| mul | L² | 16 | 64 | 256 |
| div | (L + 1)(64L + 2) | 1,290 | 4,626 | 17,442 |
| sqrt | (L + 2)(64L + 2) | 1,548 | 5,140 | 18,468 |
| two_sum | 12L | 48 | 96 | 192 |
| two_product | L² + 4L | 32 | 96 | 320 |

- **Not counted:** the conversion and widening. They are value methods, O(L), with no precision.
- **Unchanged:** K3a's `WorkCounter` at L = 2, and K-D5's cost test.
- **The limits stay ROOT's,** set from the W3 and W5 measurements.

## 8. Q7: the test profile (ROOT's `973438aa7`)

- **`FK/Cargo.toml`:** `[profile.test]` with `opt-level = 1`, `debug-assertions = true` and `overflow-checks = true`.
- It applies only when FK is the root package. Crates that depend on FK keep their own unoptimized test profile, and cargo gives no warning.

**Debug wall times** (opt-level 0, checkpoint A; each test alone and sequential; load averages about 3–8 from other agents; observations, not performance claims):

| Stream / test | Operations | Wall time |
|---|---|---|
| p256 at L = 4 | 10^6 | 31.0 s |
| p512 at L = 8 | 10^6 | 78.2 s |
| p1024 at L = 16 | 10^6 | 215.1 s |
| mixed4, mixed8, mixed16 | 2·10^5 each | 5.5, 13.7 and 39.4 s |
| conv2, conv4, conv8, conv16 | 10^6 each | 2.5, 2.8, 3.5 and 5.0 s |
| p = 53 hardware at L = 4, 8, 16 | 5·10^5 each | 10.1, 27.5 and 85.4 s |
| the L = 2 cross-check on K3a's p128 and mixed streams | 1.2·10^6 | 14.4 and 2.9 s |

**FK's full suite** (`RUST_TEST_THREADS=4`):

| opt-level | Wall time |
|---|---|
| 0 | 231.9 s (lib part 226.2 s; the baseline's lib part is 5.8 s) |
| 1 | 16.1 s, plus a 7.0 s build |
| 2 | not tried: level 1 meets "about a minute" |

With the guard test the suite is 222 tests in 20.7 s.

**The ruling's conditions:**
- **The same test list:** at opt-level 1 the list is identical to opt-level 0's (221 names; `test_list_opt_level_{0,1}.txt`).
- **K3a's and K-D5's pins are unchanged.**
- **No stream is reduced,** and there is no `#[ignore]`.
- **Hosted CI's numerical-job time** goes in the merge record (ROOT).
- The profile's own mutants P1 and P2 are killed by the guard test (§13).

## 9. Suites (per crate; `_run_records/checkpoint_b/suites/`)

- **Method:** ROOT's `run_suites_nff.sh`, verbatim: all 39 manifests of CI's cargo profile, `--no-fail-fast`, `-j 8` and `RUST_TEST_THREADS=4`.
- **The tree:** a `git archive` of `74add6078` with the profile copied over it, without `execution/`. It is byte-identical to `8cacbfaf4`'s tree; the only extra files are `__pycache__` directories created by the run itself.
- **The comparison** is against `<wt>/scratch/calib/suites_main_eb52114e9/`, per test, by `compare_suites.py`.
- **38 manifests are identical per test,** with the same counts.
- **frame_kernel:** 179 → 221 (+42: K3's tests; 0 removed; 0 changed status). At the final candidate, with the guard, it is 222.
- **The only failures are the three Mac platform tests,** as on main, and their failure blocks are byte-identical to main's (with tree roots and thread ids normalized):
  - PP `s11g_tests::t13_committed_fallback_uz_is_byte_identical`;
  - headless `load_reference_route_tests::load_reference_one_actual_solve…`;
  - headless `cli_load_reference_one_both_modes…`.
- **Warnings:** identical to main in all 39 manifests, by manifest and message. PP's 10 dead-code warnings are identical to main's in text, order and source location.

## 10. T9, the committed-fixture diff (Mac-only; `_run_records/checkpoint_b/t9/`)

- **Base:** a `git archive` of `eb52114e9`.
- **Candidate:** a `git archive` of `74add6078` with the profile copied over it, byte-identical to `8cacbfaf4`'s tree (`diff -r`).
- Both are without `execution/`, on this Mac, and the two trees differ only in K3's files.
- **Harness:** S11-K's `fixdiff_main.rs` (`ec089c1d…`, unchanged), with ROOT's harness `Cargo.lock` (`1c69935d…`), built `--release --offline --locked`. PP's lock is equal on both sides.
- **Result: 112 of 112 outputs byte-identical** (core 10, fixtures 72, validation 30). The raw output directories are identical.
  - 6 are `ERR` on both sides; there is no `PANIC`.
- **Cross-check:** the base equals ROOT's Mac main hashes (`fixdiff/sha_native.txt`) on all 112.
- **This is a Mac-only comparison,** never compared with the Linux records (`T3/PLATFORM_CALIBRATION_MAC/`).
- T9 is real evidence here, because `wide.rs` is live through K-D5.

## 11. The differentials: seeds, counts and digests (`FK/tests/retained_wide_k3/differential.txt`)

- **The scheme is K3a's, as accepted at its merge:**
  - the streams are SplitMix64 from a recorded seed (8 ASCII characters, read big-endian);
  - each has a digest per 10^5-record chunk and a whole-stream sha256;
  - the first 1,000 records of each stream are committed;
  - the Rust test regenerates every operand and compares operands against the samples, then every chunk and stream digest.
- **The oracle:** `fractions.Fraction` rounded once; the square root is `math.isqrt` with an exactness test.
- **The conversions** are also cross-checked against CPython's correctly rounded `float(Fraction)`.

| Stream | L | p | Count | Seed | sha256 |
|---|---|---|---|---|---|
| p256 | 4 | 256 | 10^6 | `4b335f5030323536` (K3_P0256) | `f0e952f6cd2bdde4a00cc79da8aa80a6024ae6c502e8707281a3f40d2b7fafde` |
| p512 | 8 | 512 | 10^6 | `4b335f5030353132` (K3_P0512) | `a33ea753cc4c08f0633b66d8914a100a30e66dc0534c29a2a1463c6dcf41c590` |
| p1024 | 16 | 1024 | 10^6 | `4b335f5031303234` (K3_P1024) | `1c48951d9a2d9313cf8acdff0e58360669a6a49e202df39e6bda4ab8cdcf7f20` |
| mixed4 | 4 | mixed | 2·10^5 | `4b335f4d49583034` (K3_MIX04) | `c349ad66ed947815581044683fe1874ac743fadd4b1d0abe1719e5bd0c6facb7` |
| mixed8 | 8 | mixed | 2·10^5 | `4b335f4d49583038` (K3_MIX08) | `cff8b4cebc94cf1acbeb3c02de206a6d9d8d2f5ca9f588570cde5414eb4fb830` |
| mixed16 | 16 | mixed | 2·10^5 | `4b335f4d49583136` (K3_MIX16) | `18cfe622fa201e749626efd81f955ba63d0741fe2120e855a58cf57c97b87a40` |
| conv2 | 2 | — | 10^6 | `4b335f434e563032` (K3_CNV02) | `1ba34603ed627e0aa38e119399a72c0852a4aaef25882b45797f0986418f6bc7` |
| conv4 | 4 | — | 10^6 | `4b335f434e563034` (K3_CNV04) | `e981f6c305bce84eb71b1a15aa2e26f30cb5a02ee3c5a228045d051f481dab39` |
| conv8 | 8 | — | 10^6 | `4b335f434e563038` (K3_CNV08) | `bb71e0ae1e29505d60246c14d679c35318982de334df06254f90f466fed0584a` |
| conv16 | 16 | — | 10^6 | `4b335f434e563136` (K3_CNV16) | `0f8ab6f552d53f9db2c5819cd2db23d79e9a1d2af8eec9dde4100a05aba8c399` |

- **The mixed streams:** half uniform on [2, 64L]; half from {53, 128, 192, 256, 320, 512, 576, 1024} ∩ [2, 64L] and the limb boundaries. The test asserts that every one of those precisions is drawn.
- **The operand rules** generalize K3a's to 64L bits, with an exponent gap up to 64L + 400 so that sticky bits fall beyond the window.
- **Conversion outcomes** (normal / subnormal / underflow / overflow): conv2 465,260 / 293,441 / 165,034 / 76,265, and the same distribution at every width. They are concentrated at 2^−1074, 2^−1022 and 2^1024, with some near ±2^62.
- **K3a's streams through the L = 2 core:** K3a's `p128` (`b568d2c0…`) and `mixed` (`f8b008bc…`) digests are reproduced.
- **The generator** (Python 3.13.14, standard library only) takes 2m13s to write and 2m29s for `--check`. `--check` gives OK for 8 of 8 files.

## 12. The conversion's boundary results (`conversion.txt`; the same at L = 2, 4, 8 and 16; positive shown; the negative mirrors)

| Class | Outcome |
|---|---|
| ±0 | `N` ±0.0, sign kept |
| least subnormal 2^−1074 | `S`, bits `…0001`, relative precision 0.5 |
| largest subnormal | `S` `000fffffffffffff`, relative precision `3ca0000000000002` (1/(2^53 − 2) rounded up) |
| smallest normal | `N` `0010000000000000` |
| [2^−1022 − 2^−1075, 2^−1022), including the tie | `N` `0010000000000000`: rounds up into the normal range |
| just below that interval | `S` largest subnormal |
| exactly 2^−1075 (a tie) | `U`: underflow, no value |
| 2^−1075 plus the smallest tail each width carries | `S` least subnormal |
| 2^−1075 minus a tail | `U` |
| subnormal ties, even and odd kept parts | `S`, rounded to even |
| subnormal ties with a sticky bit 64L − 60 binades down (964 at L = 16) | `S`, rounded up, **with no double rounding** |
| the double-rounding traps (2^−1075 + 2^−1135; (k + ½)·2^−1074 + 2^−1144 with even k) | `S` rounded up, where a first 53-bit rounding would tie down |
| MAX | `N` `7fefffffffffffff` |
| just below the midpoint 2^1024 − 2^970 | `N` MAX |
| the midpoint, and above it; 2^1024 | `O`: overflow, no value |
| exponents at ±2^62 | `O` / `U`, never a wrap |
| normal ties, far sticky bits, carries | `N`, correctly rounded |

## 13. Mutations (`_run_records/mutations/`)

**Method:**
- **Sources:** a clean `git archive` of `8cacbfaf4` (`core/`) per mutant in `<wt>/k3-mut/<id>/`, with its own target, deleted after each run.
- **Patches:** one exact-string patch each, asserted to apply once (`mutate.py.txt`).
- **Command:** `cargo test --offline --locked -j 4 --no-fail-fast` of FK under the committed profile, `RUST_TEST_THREADS=4`. The NONE control ran first and alone; then at most three ran at once.
- **What counts as a kill:** only a failing test whose failing panic is at an `assert` line of a test file. Unwraps, `panic!` calls, panics in the code under test and compile failures do not count. Every mutant compiled.
- **Line numbers** of `K3T` below are at `8cacbfaf4`. The guard test later inserted 21 lines above `K3T` §A, so those lines are 21 higher at the final candidate.
- **Two driver changes before the guard re-run** (`mutate.py.txt` is the final version):
  - an `OVERLAY` of the uncommitted `k3_tests.rs`, recorded per run;
  - classification by the **last** panic of a failure block, because a panic that `catch_unwind` catches is printed first.
  - No checkpoint-C block had more than one panic, so its classification is the same under either rule.

| # | Mutant | Site | Result | Behavioural kills: count, and key killing assertions |
|---|---|---|---|---|
| NONE | control | — | 221 passed (222 with the guard) | — |
| M1 | round toward zero | `multi.rs:418` | killed | 22. Targeted ties (first failure: `tie add 53`), K3T:835; p = 53 hardware, K3T:751 and :767; the carry through every limb, K3T:910; the six streams, K3T:1270; the L = 2 cross-check, K3T:523 and :690 |
| M2 | dropped sticky bit | `multi.rs:415` | killed | 12. `sticky div 53`, K3T:835; the streams; the L = 2 cross-check |
| M3 | ties away from zero | `multi.rs:418` | killed | 18. Targeted ties, K3T:835; p = 53 hardware, K3T:751; narrowing, K3T:2052 and :1851; the streams |
| M4 | off-by-one shift across a middle limb (word 2 takes word 3's bits one place off) | `multi.rs:199` | killed | 14. Limb-boundary classes (`cancel sub` at p = 53, 63 and 64), K3T:835; TwoSum vectors, K3T:1838; the streams |
| M5 | addition carry chain stopped after one limb | `multi.rs:251` | killed | 20. **The carry through every limb, K3T:906**; `carry add`, K3T:835; K3T:942; the streams |
| M6 | schoolbook product drops each row's top carry | `multi.rs:344` | killed | 19. `tie mul` and the carry classes, K3T:835; TwoProduct vectors, K3T:1838; p = 53 hardware; the streams |
| M7 | division remainder ignored as sticky | `multi.rs:548` | killed | 12. `sticky div 53`, K3T:835; the streams, K3T:1270 and :1297 |
| M7b | square-root remainder ignored as sticky | `multi.rs:592` | killed | 12. `sticky sqrt`, K3T:835; the streams |
| M8 | conversion double-rounds (53 bits, then the subnormal quantum) | `multi.rs:727` | killed | 6. `below_up_to_normal`, K3T:1351; K3T:1445; the four conversion streams, K3T:1608 |
| M9 | conversion flushes subnormals | `multi.rs:751` | killed | 7. The largest subnormal, K3T:1420; K3T:1351; K3T:1526; the conversion streams. Two further failures (a `panic!`, an unwrap) are not counted |
| M10 | overflow midpoint rounds down to MAX | `multi.rs:732` | killed | 7. `midpoint`, K3T:1351; K3T:1455; the accumulator cross-check, K3T:1741; the conversion streams, K3T:1608 and :1619 |
| M11 | nonzero underflow returned as a silent zero | `multi.rs:739` | killed | 8. `half_min_subnormal`, K3T:1351; K3T:1436; K3T:1509; K3T:980; the conversion streams |
| M12a | the sign of an exact zero lost | `multi.rs:719` | killed | 7. The zero convention, K3T:1498; the lift round trip, K3T:1693; vectors; streams |
| M12b | the sign of an underflow lost | `multi.rs:740` | killed | 8. K3T:1436, :1509 and :980; vectors; streams |
| M13 | unchecked exponent in the new path (wraps) | `multi.rs:424` | killed | 1. `assert_eq!(c.mul(&big, &big), range)`, K3T:940 |
| M14 | TwoSum returns e = 0 | `multi.rs:614` | killed | 4. TwoSum vectors, K3T:1839; random TwoSum, K3T:1979; **the §7.3-16 analogue, K3T:2175**; K3a's EFT vectors through the L = 2 core, K3T:516 |
| M15 | narrowing truncates | `multi.rs:1190` | killed | 3. Narrowing vectors (`narrow 2 4 53`), K3T:1851; K3T:2057; K3T:962 |
| M16a | operations charged to the wrong width | `multi.rs:1085` | killed | 2. K3T:2258 and :2328 |
| M16b | a width's multiplication not charged | `multi.rs:1168` | killed | 2. K3T:2238 and :2326 |
| M17 | K3a's `round_pack` tie rule (ties away; the L = 2 path) | `wide.rs:654` | killed | 9. **K3a's suite:** K3a's targeted vectors (`wide_tests.rs:517`), p = 53 hardware (:419), both differentials (:795, :822), the split→accumulator test (:952) and the arctangent (:1033). **The L = 2 cross-check:** K3T:523 and :690 (twice). **Not K-D5's tests** (below) |
| M18 | relative precision rounded to nearest | `multi.rs:700` | killed | 7. K3T:1791; K3T:1420; vectors; the conversion streams |
| M19 | integer constructor ignores far limbs (sticky lost) | `multi.rs:461` | killed | 1. The 68-limb, 2^−2148 vectors, K3T:1874 |
| M20 | TwoProduct residual sign wrong when rounded up | `multi.rs:647` | killed | 3. K3T:1839, :1987 and :516 |
| M21 | widening places the limbs at the bottom | `multi.rs:763` | killed | 3. K3T:2027, :1987 and :2180 |
| M22 | division one quotient bit short | `multi.rs:541` | killed | 12. `cancel div` and `nearexactdiv`, K3T:835; the streams; the L = 2 cross-check |
| P1 | profile `overflow-checks = false` | `FK/Cargo.toml:20` | **killed after the guard** | Before the guard: 221 passed, undetected. With the guard (clean archive plus the overlay): 1 failure, `assert!(overflowed.is_err(), …)` at K3T:416 (final numbering) |
| P2 | profile `debug-assertions = false` | `FK/Cargo.toml:19` | killed | 1 failure: `assert!(cfg!(debug_assertions), …)` at K3T:420 (final numbering). Under P2 the guard's overflow still panicked, as expected, at :414, showing overflow checks stay on |

- **The M17 note (ROOT: a note, not a defect).** K-D5's tests do not detect a tie-rule change at 2^−128. Against the M17 archive, every one passes:
  - FK's 7 `formation_check` tests;
  - NI's 16 `kd5` tests;
  - PP's `formation_check_runtime` (5) and `f1a_tests` (7).
  - The logs are `M17_ni_kd5.log`, `M17_pp_fcr.log` and `M17_pp_f1a.log`.
  - So K3a's path is guarded by K3a's suite and by K3's L = 2 cross-check, not by K-D5's pins.
  - My first PP attempt failed to compile only because the mutant copy held `core/` alone. I copied in the rest of the piping tree, without `execution/`, and re-ran.
- **Wall times:** 16–32 s per mutant. All mutant targets are deleted.

## 14. K4 interface (exact signatures, at the candidate)

K4 imports from `super::wide` (K3a) and `super::wide::multi` (K3). `Wide`'s fields are private to `wide` and its descendants, so K4, in sibling modules under `retained/`, reads and builds values through `parts` and `from_parts`.

```rust
// FK/src/structural/retained/wide.rs (K3a; K3 appends one variant)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Wide<const L: usize> { /* negative: bool, exponent: i64, significand: [u64; L] */ }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum WideError { InvalidPrecision(u32), NonFinite, ExponentRange, DivisionByZero, NegativeSqrt,
    NotNormalized, AngleDomain, ArctangentLimit, SplitOverflow, Accumulator(SumError), OperandPrecision }
pub(crate) const MIN_PRECISION: u32 = 2;
pub(crate) const EXPONENT_LIMIT: i64 = 1 << 62;

// FK/src/structural/retained/wide/multi.rs (K3)
pub(crate) trait CoreWidth { type Double: Copy + AsRef<[u64]> + AsMut<[u64]>; const DOUBLE_ZERO: Self::Double; }
    // implemented for Wide<2>, Wide<4>, Wide<8>, Wide<16>
pub(crate) trait SupportedWidth: CoreWidth { const SLOT: usize; }
    // implemented for Wide<4> (0), Wide<8> (1), Wide<16> (2) only

impl<const L: usize> Wide<L> where Wide<L>: SupportedWidth {
    pub(crate) const ZERO: Self;
    pub(crate) const ONE: Self;
    pub(crate) fn from_parts(negative: bool, exponent: i64, significand: [u64; L]) -> Result<Self, WideError>;
    pub(crate) fn parts(&self) -> (bool, i64, [u64; L]);     // canonical limbs (D1 §4.1.8's digest)
    pub(crate) fn from_f64(value: f64) -> Result<Self, WideError>;
    pub(crate) fn is_zero(&self) -> bool;
    pub(crate) fn is_sign_negative(&self) -> bool;
    pub(crate) fn exponent(&self) -> i64;                   // of the leading bit; 0 for zero
    pub(crate) fn neg(&self) -> Self;
    pub(crate) fn abs(&self) -> Self;
    pub(crate) fn fits_precision(&self, p: u32) -> bool;
    pub(crate) fn mul_pow2(&self, k: i64) -> Result<Self, WideError>;
    pub(crate) fn cmp_value(&self, other: &Self) -> std::cmp::Ordering;   // +0 == −0
}
impl<const L: usize> std::fmt::Debug for Wide<L> where Wide<L>: SupportedWidth;  // "Z+"/"Z-" or "{sign}{16L hex}p{e}"

impl<const L: usize> Wide<L> {                               // every width, L = 2 included
    pub(crate) fn to_binary64(&self) -> Binary64Outcome;
    pub(crate) fn widen<const M: usize>(&self) -> Wide<M>;    // exact; M >= L at compile time
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Binary64Outcome {
    Normal(f64),                                            // normal, or an exact ±0 (sign kept)
    Subnormal { value: f64, relative_precision: f64 },      // 2^-1075/|value|, rounded upward
    Underflow { negative: bool },                           // nonzero, rounds to ±0; no value
    Overflow { negative: bool },                            // rounds to ±inf; no value
}
impl Binary64Outcome { pub(crate) fn value(&self) -> Option<f64>; }

#[derive(Debug, Clone)]
pub(crate) struct WideContext<const L: usize> { /* precision: u32, work: WidthWork */ }
impl<const L: usize> WideContext<L> where Wide<L>: SupportedWidth {
    pub(crate) fn new(precision: u32) -> Result<Self, WideError>;   // 2 <= p <= 64L, else InvalidPrecision(p)
    pub(crate) fn precision(&self) -> u32;
    pub(crate) fn work(&self) -> WidthWork;
    pub(crate) fn add(&mut self, a: &Wide<L>, b: &Wide<L>) -> Result<Wide<L>, WideError>;
    pub(crate) fn sub(&mut self, a: &Wide<L>, b: &Wide<L>) -> Result<Wide<L>, WideError>;
    pub(crate) fn mul(&mut self, a: &Wide<L>, b: &Wide<L>) -> Result<Wide<L>, WideError>;
    pub(crate) fn div(&mut self, a: &Wide<L>, b: &Wide<L>) -> Result<Wide<L>, WideError>;
    pub(crate) fn sqrt(&mut self, a: &Wide<L>) -> Result<Wide<L>, WideError>;
    pub(crate) fn round<const M: usize>(&mut self, x: &Wide<M>) -> Result<Wide<L>, WideError>;
    pub(crate) fn from_integer(&mut self, negative: bool, magnitude: &[u64], exponent: i64)
        -> Result<Wide<L>, WideError>;                       // +magnitude·2^exponent rounded once; zero gives +0
    pub(crate) fn two_sum(&mut self, a: &Wide<L>, b: &Wide<L>) -> Result<(Wide<L>, Wide<L>), WideError>;
    pub(crate) fn two_product(&mut self, a: &Wide<L>, b: &Wide<L>) -> Result<(Wide<L>, Wide<L>), WideError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OpKind { Add, Sub, Mul, Div, Sqrt, Round, TwoSum, TwoProduct }
impl OpKind { pub(crate) const ALL: [OpKind; 8]; }
pub(crate) fn limb_multiply_cost(kind: OpKind, limbs: usize) -> u64;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct WidthWork { pub(crate) add: u64, pub(crate) sub: u64, pub(crate) mul: u64, pub(crate) div: u64,
    pub(crate) sqrt: u64, pub(crate) round: u64, pub(crate) two_sum: u64, pub(crate) two_product: u64 }
impl WidthWork {
    pub(crate) fn count(&self, kind: OpKind) -> u64;
    pub(crate) fn operations(&self) -> u64;
    pub(crate) fn limb_multiply_equivalents(&self, limbs: usize) -> u64;
}
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct AttemptWork { /* widths: [WidthWork; 3] */ }
impl AttemptWork {
    pub(crate) fn record<const L: usize>(&mut self, context: &WideContext<L>) where Wide<L>: SupportedWidth; // once per context
    pub(crate) fn merge(&mut self, other: &Self);
    pub(crate) fn width<const L: usize>(&self) -> WidthWork where Wide<L>: SupportedWidth;
    pub(crate) fn limb_multiply_equivalents(&self) -> u64;
}
```

**Contract notes for K4:**
- Every operation rounds the exact result once, to nearest with ties to even. Signs of zero follow IEEE 754 under round-to-nearest.
- An exponent outside ±2^62 is `ExponentRange`, never a wrap.
- `two_sum` and `two_product` refuse operands wider than p bits (`OperandPrecision`). K4 rounds its operands to p first; a binary64 factor at p ≥ 53 always fits.
- `record` adds a context's counts; record each context once.
- Generic K4 code bounds on `where Wide<L>: SupportedWidth`.

## 15. For K4's brief

- **Q8, the ceiling's refinement.** A 512-bit candidate is verified at 1024. A p + 64 residual on that verification solve would need 1088 bits, and `Wide<16>` holds 1024. ROOT settles it in K4's brief: either no p + 64 residual on the ceiling's verification-only solve, or one more width.
- **The widths K4 runs at** (ruling 8). The new core exists at L = 2 only in the tests, so K4 runs p = 128 and 192 at L = 4.
  - The W1 schedule then maps as: 128, 192 and 256 at L = 4; 320 and 512 at L = 8; 576 and 1024 at L = 16.
- **The `exact_sum.rs` read accessor** is K4's (Q4). `from_integer` takes its limbs and the 2^−2148 quantum as they are.
- **Division and square root are bit-serial.** At L = 16 they cost about 70 multiplications each in the cost table. Knuth's algorithm D is left to a later optimization with its own vectors (ruling 7).
- **Unifying K3's `Binary64Outcome` with K2b's `Representability`** is deferred to K4, F1b or F2a (Q5).

## 16. Toolchain and host (`_run_records/toolchain.txt`)

- **Platform:** `aarch64-apple-darwin` (macOS 26.6.2), rustc and cargo 1.97.1, `CARGO_INCREMENTAL=0`, `--offline --locked`.
- **Formatting:** rustfmt 1.9.0 stable, on K3's files only; all clean.
- **Python:** 3.13.14 (`<VENV>`), standard library only. K3a's generator, recorded under 3.11.15, still reproduces its files byte for byte.
- **The Mac host rules were kept:**
  - `-j 8` at most, and `RUST_TEST_THREADS=4`;
  - at most two cargo jobs of mine at once;
  - mutants at most three at once at `-j 4`;
  - own target `<wt>/k3-target`, and mutant sources and targets under `<wt>/k3-mut/`.
- **The memory guard** (`<wt>/guard/memguard.log`) never fired, and there were no SIGKILLs.
- **No timing claims:** every time here is an observation (K6 owns performance).
- **Checkpoint-0 probes:** two `cargo check` probes on a scratch crate. They confirmed:
  - that the width-bounded generic impl and `Debug` coexist with `impl Wide<2>`;
  - that an allowed item keeps its helpers live;
  - that an unused `pub(crate) use` warns, so there are no re-exports.
  They were deleted.

## 17. What was not done, and open items

- **No Git writes.** ROOT commits: first the guard test (tests only), then these records.
- **The both-entry gate was not run** (ruling Q2). K3 adds no product caller.
- **Not run by me; belonging to ROOT and the manager:**
  - hosted CI, and the numerical job's time for the merge record (Q7 condition);
  - the DEC-025 sweep under the owner's Mac decision;
  - the independent complete-diff review, with an oracle independent of K3's generator;
  - GEN-8 on the committed records.
- **No native witnesses:** K3 is kernel only.
- **Q8 and the K2b/K3 outcome unification** are for later briefs (§15).
- **`<wt>/scratch/i11/cand_tree`** (the suites' archive copy) is kept until ROOT's review. `<wt>/k3-mut` and every target are pruned.

## 18. Records

- **`_run_records/`** is built by `assemble_run_records.py.txt` from I11's scratch directory. It contains:
  - `toolchain.txt` and `source_sha256.txt`;
  - `checkpoint_a/`:
    - the generator's run and both `--check` logs;
    - the targeted runs in run order, including the two first failing runs (`fast1.log` and `fast2.log`; the corrections are listed at the end of §6);
    - the non-test build;
    - the full FK suite at opt-level 0;
    - NI's and PP's K-D5 runs;
    - `heavy/`, the per-test logs and wall times;
  - `checkpoint_b/`:
    - the opt-level 1 build and suite, the guard run, and the two test lists;
    - `suites/`: the runner, the comparer, the comparison, the summary log and the 39 logs;
    - `t9/`: the harness manifests, the build and run logs, the output hashes and the summary;
  - `mutations/`:
    - the driver;
    - `MUTANTS.txt`, and `summary.json` with per-mutant `.json`, `.patch` and `.log`;
    - the M17 K-D5 logs;
    - `guard/`: NONE, P1 and P2 with the guard.
- **Sanitized:**
  - Machine paths became `<VENV>`, `<scratch>`, `<wt>` and `<home>`.
  - Trailing spaces and trailing blank lines were stripped.
  - No line was cut, and no other byte changed.
- **GEN-8** (`pytest tools/practitioner_harness/test_live_baseline.py -k gen8`, run from `<wt>/k3` with `<VENV>`) passed: 1 passed, 10 deselected.
  - GEN-8 scans git-tracked files only, and these records were not yet committed.
  - Its `MACHINE_ABS_PATH_RE` (`tools/practitioner_harness/surface_roles.py`) was therefore applied directly to every file in this folder and to K3's source and test files: 202 files, 0 hits.
  - GEN-8 is to be re-run after ROOT commits the records.
- **`git diff --check`** (as `--no-index` against `/dev/null`) is clean on every file in this folder.
- **`SHA256SUMS`** covers every file in this folder except itself.

## Addendum 1: Q7 reversed (ROOT's `ffc9ea275`, "K3: Q7 reversed — the test profile is withdrawn")

### The finding (ROOT's)
- ROOT merged main `98b1723b1` (the skew M03 pin) into K3 as `de719cbdc`, and ran FK's suite under K3's `[profile.test] opt-level = 1`.
- **One test failed:** `m03_skew_scope.rs::m03_skew_pin_rv7_cases_outcomes_and_figures`. 2EI/L's exact error came out 1.1364e-13, against the pinned 1.1378e-13.
- **The cause:** the test forms the second moment from compile-time constants with `od.powi(4)`. At opt-level ≥ 1, LLVM constant-folds `powi` through the host's `pow`; at opt-level 0 the runtime repeated multiplication runs.
- **The premise was false.** Q7's premise ("the results cannot change with the opt-level") holds for + − × ÷ √. It does not hold for functions of unspecified precision that the compiler evaluates.
- **The product is not affected:** its `powi` inputs are runtime data, and T9's release-built outputs have always matched the debug-built tests.

### My part
- At checkpoint B I checked the conditions ROOT set for the profile: the same test list, and K3a's and K-D5's pins unchanged. That base (`6e18505e3`) did not yet contain the skew pin.
- On it, no test of FK's computed through a compile-time-folded function of unspecified precision, so those checks could not see the effect.
- I did not derive the general claim either.
- Hosted CI's whole numerical job takes about 4–5.5 minutes of its 45 on current main (ROOT), so the profile was never needed.

### The revert
- `FK/Cargo.toml` equals main's file byte for byte (sha256 `c124ff5534616207027d932d561b810fed824ee7a095b4a06f738ff9d226c317`, the same bytes as at the base `6e18505e3`). There is no `[profile.test]`.
- **The guard test stays** (§6): it asserts that overflow checks and debug assertions are on in FK's default test build.

### The re-run at opt-level 0, on `de719cbdc` plus the revert (`_run_records/q7_reversed/`)
- **The tree:** a clean `git archive` of `core/`, with the reverted `Cargo.toml` copied over it.
- **FK's full suite** (`--no-fail-fast`, `-j 8`, `RUST_TEST_THREADS=4`): **227 passed, 0 failed.**
  - The breakdown: lib 197, `k1_k2a_interaction` 3, `k2a_checked_formation` 13, `m03_skew_scope` 5, `s11_site_table` 3, doc 6.
  - **Wall time:** 230.7 s for the tests (the lib part 227.3 s), at load averages 2.5–5.4. The build is recorded separately in `fk_cand_build.log`.
- **Per test against main:** main `98b1723b1`'s FK suite (184 tests: the `eb52114e9` baseline's 179 plus the skew pin's 5) passes 184 of 184.
  - All 184 are in the candidate, with the same status.
  - The candidate adds exactly K3's 43.
  - Against the `eb52114e9` baseline: +5 (the skew pin, from main) and +43 (K3), with none removed or changed.
  - The lists are `fk/fk_{base_eb52114e9,main,cand}_tests.txt`.
- **The suites' FK line** on the merged tree is therefore 227 passed, 0 failed, 0 ignored (baseline 179; main 184).
- **The other 38 manifests and T9 are unaffected.**
  - FK's profile only ever applied when FK was the root package. Every other manifest built FK under its own unoptimized test profile, and T9's harness built FK under its own release profile.
  - The merge of main adds only tests (FK `tests/m03_skew_scope.rs`, NI `structural_adapter/k1_tests.rs`).

- **The guard's doc comment** described the withdrawn profile. After the mutation run it was reworded (comment only, the same four lines, so the kill-site lines :416 and :420 are unchanged).
  - FK's full suite on the final worktree tree (the merged tree, the revert and the reworded comment): **227 passed, 0 failed, in 231.5 s**.
  - The non-test FK build has no warnings.
  - The log is `fk/fk_worktree_final_o0_full.log`.

### The mutation table at opt-level 0
- **Method:** clean archives of `de719cbdc` with the reverted `Cargo.toml` overlaid, the NONE control first, then at most three at once at `-j 4`, `RUST_TEST_THREADS=4`.
- **The driver** is `q7_reversed/mutations/mutate.py.txt`, with the same patches as §13 except P1 and P2.
- **NONE:** 227 passed.
- **All 25 source mutants are killed.** For every one, the set of killing tests equals checkpoint C's, test for test. M9 again has its 2 failures that are not counted as kills.
- **P1 and P2** are now a `[profile.test]` added to FK's profile-free manifest, with `overflow-checks = false` (P1) or `debug-assertions = false` (P2). **Both are killed by the guard:** `assert!(overflowed.is_err(), …)` at `k3_tests.rs:416`, and `assert!(cfg!(debug_assertions), …)` at `k3_tests.rs:420`.
- **The M17 note (§13) stands.** At opt-level 0, FK's `formation_check` tests again pass under M17. K-D5's NI and PP tests always built at opt-level 0.
- **Wall time per mutant:** 18–367 s. About 20 s when the failures end the streams early; about 270 s when the streams run to the end.
- `MUTANTS.txt` and `summary.json` give every row, with its kill sites.

### Wall times at opt-level 0 (the Q7 observation, restated)
- FK's full suite: 230.7 s here, and 231.9 s at checkpoint A (§8).
- Per stream: §8's table, which was measured at opt-level 0.
- These are observations, not performance claims. Hosted CI's numerical-job time on K3's PR goes in the merge record (ROOT).

### The lesson (ROOT's, recorded here too)
- A test must not depend on how the compiler evaluates a function of unspecified precision. Tests that need exact values from such functions should compute them with explicit, ordered arithmetic on runtime values, or on `black_box`ed constants.
- **K3's own tests and code call no such function** (checked with `/usr/bin/grep` for `powi`, `powf`, `exp*`, `ln*`, `log*`, trigonometric and hyperbolic functions, `hypot` and `cbrt`: no hits). They use:
  - integer arithmetic;
  - binary64 + − × ÷ and `sqrt`, which IEEE 754 specifies as correctly rounded at every level;
  - `mul_add` and `next_up`, which are specified exactly.

### What this addendum supersedes
- §8 (the profile and its conditions).
- The `FK/Cargo.toml` row of §2: that file is now unchanged against the base.
- In §13, the definitions of P1 and P2 (now "add a profile that switches the check off"), and the phrase "with the committed profile".
- §9's FK line: now 227 on the merged tree.
- `CHANGE_RECORD.md` is updated to match.
- The earlier run records are kept as they were. The new ones are under `_run_records/q7_reversed/`.
