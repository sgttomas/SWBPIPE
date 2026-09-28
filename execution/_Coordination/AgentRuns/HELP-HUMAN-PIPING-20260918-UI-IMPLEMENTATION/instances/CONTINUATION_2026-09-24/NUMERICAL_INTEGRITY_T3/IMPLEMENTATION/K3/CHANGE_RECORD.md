# K3 change record: the rest of W1's arithmetic

This is the draft PR record for slice K3 of T3 (numerical integrity), following `.agents/skills/chirality-change/SKILL.md`. The implementation is I11 (TASK, the owner's Mac); the detail is in `RETURN.md`.

- **Branch:** `codex/piping-k3-20260928`, from main `6e18505e3`. Its piping tree equals `eb52114e9`'s, ROOT's Mac baseline.
- **Checked revisions:**
  - checkpoint A, `74add6078`;
  - checkpoint B, `8cacbfaf4` (a test profile, since withdrawn);
  - the guard `9aee9854c` and the records `664ef5c5e`;
  - main `98b1723b1` (the skew M03 pin) merged in as `de719cbdc`;
  - the profile revert `e83e22356` and records addendum 1 `b7e93650e` (PR #1041);
  - after RV12's review (PASS): the tests for its S2 and N1, and records addendum 2.
  - ROOT records the final candidate, PR and merge revisions.
- **Basis:**
  - D1 `DESIGN.md` revision 5a.2 (`fb62ef4a…`): §4.1.1 (`wide.rs`), §4.1.2, §4.1.7, §4.1.8, §4.11, §5 item 7, the K3 row of §6, §7.3, §7.4;
  - `TASK_BRIEFS/I11_K3_IMPLEMENTATION.md` with its ROOT rulings;
  - `ROOT_RULINGS_V1.md`: "K3: spawn and rulings", "K3: rulings on I11's checkpoint-0 plan" (`d2df479f3`), and "K3: Q7, the differential's debug cost" (`973438aa7`) as reversed by "K3: Q7 reversed — the test profile is withdrawn" (`ffc9ea275`);
  - ROOT's relayed checkpoint-B and checkpoint-C rulings (RETURN §1).

## What changes

K3 completes W1's in-repo arithmetic beside K3a's `Wide<2>`, which is unchanged and stays the only arithmetic on the product path, through K-D5. **The new surface has no product caller; K4 will be the first.**

- **`FK/src/structural/retained/wide/multi.rs` (new; the child module ROOT approved):**
  - `Wide<L>` at L = 4, 8 and 16, with the value API: lift, parts, comparisons and `Debug`;
  - `WideContext<L>`: + − × ÷ √, each correctly rounded to nearest with ties to even, at a runtime precision 2 ≤ p ≤ 64L, integer-only, with no allocation per operation;
  - `Wide::to_binary64()` at every width, L = 2 included: rounded once, with the outcome `Normal` (an exact ±0 keeps its sign), `Subnormal` (with 2^−1075/|value| rounded upward), `Underflow` or `Overflow`, the last two with no value;
  - K4's arithmetic: exact `widen`, correctly rounded narrowing (`round`), TwoSum, TwoProduct, and `from_integer` (±magnitude·2^e rounded once; zero gives +0);
  - per-width work counts (`WidthWork`), an attempt counter mergeable across widths (`AttemptWork`), and a stated limb-multiply-equivalent cost per kind and width.
- **`FK/src/structural/retained/wide.rs` (+21 −13).** K3a's `impl Wide<2>`, `WideArith`, `WorkCounter`, `round_pack`, `U256`, the `Debug` for `Wide<2>`, and the existing `WideError` variants and `Display` arms are byte-identical. The declared changes:
  - `pub(crate) mod multi;`;
  - the variant `WideError::OperandPrecision`, appended, with its `Display` arm appended ("retained operand exceeds the working precision");
  - the module-doc pointer to `multi`;
  - the dead-code allowances relabelled to name the real consumer (ROOT's ruling 2):
    - `NotNormalized` loses its allowance: K3's `from_parts` constructs it;
    - `from_parts`, `parts`, `is_sign_negative`, `exponent`, `fits_precision` and `WideArith::precision`: "K3a API, no caller yet (reviewed at T3 close)";
    - `atan_positive`: "later-slice API (W1c; K3 Q6)";
    - **`rounded_operations` and `WideArith::work`:** "test-only: K-D5 measures its cost in tests (K4 counts with multi::AttemptWork)", replacing the untrue "…; K4 budgets use it". Declared here; ROOT accepted it at checkpoint B.
- **`FK/src/structural/retained/mod.rs`:** comments only.
- **`FK/Cargo.toml`: unchanged** against the base.
  - The `[profile.test]` added at checkpoint B is withdrawn (ROOT's `ffc9ea275`).
  - At opt-level ≥ 1, LLVM constant-folds `powi` over constants, which moved the skew pin's figure. Hosted CI's numerical job has room for the full-count streams at opt-level 0.
  - FK's tests build at opt-level 0 like every other crate's, and every stream stays at full count.
- **`FK/tests/retained_wide_k3/` (new; the test module is declared from `multi.rs`, as ROOT approved):**
  - `k3_tests.rs`: 45 tests, including a guard that overflow checks and debug assertions are on in FK's test build, and RV12's S2 value tests;
  - `gen_wide_k3_vectors.py`: standard library only, with `--check`;
  - the vectors: `targeted_l{4,8,16}.txt`, `conversion.txt`, `eft.txt`, `differential.txt` and `differential_sample.txt`;
  - `SHA256SUMS`.
  - The data is about 4.9 MB (ROOT approved about 5 MB).

## Standing, values and bytes

- **No published byte changes.** K3 adds no caller outside `retained`, and K3a's `Wide<2>` path, live through K-D5, is unchanged. The evidence (Q2):
  - **T9 (Mac-only):** 112 of 112 outputs are byte-identical, base `eb52114e9` against the K3 tree, both built on this Mac from `git archive` copies. The base equals ROOT's Mac main hashes on all 112. This is never compared with the Linux records.
  - **K3a's 19 tests and K-D5's suites pass unchanged:** FK's formation-check tests, NI's `kd5_tests`, and PP's `formation_check_runtime` and `f1a_tests`.
  - **A byte-for-byte pin** of every existing `WideError` `Display` string.
  - **The L = 2 core is bitwise equal to K3a's `WideArith`** on all of K3a's vectors and both of its streams, and reproduces K3a's digests.
- **The both-entry gate is not run** (ruling Q2).
- **K3 does not touch** `exact_sum.rs`, `formation_check.rs`, `structural.rs`, `lib.rs`, SA, PP, the site tables, the fixtures, or K3a's `tests/retained_wide/**`.

## Files

| File | +/− | Lines |
|---|---|---|
| `P/core/solver/frame_kernel/src/structural/retained/wide/multi.rs` (new) | +1233 | 1233 |
| `P/core/solver/frame_kernel/src/structural/retained/wide.rs` | +21 −13 | 997 |
| `P/core/solver/frame_kernel/src/structural/retained/mod.rs` | +12 −3 | 24 |
| `P/core/solver/frame_kernel/tests/retained_wide_k3/k3_tests.rs` (new) | +2479 | 2479 |
| `P/core/solver/frame_kernel/tests/retained_wide_k3/gen_wide_k3_vectors.py` (new) | +989 | 989 |
| `P/core/solver/frame_kernel/tests/retained_wide_k3/{targeted_l4,targeted_l8,targeted_l16,conversion,eft,differential,differential_sample}.txt` and `SHA256SUMS` (new; generated) | +17402 | 17402 |

## Checks

All were run on `aarch64-apple-darwin` with rustc 1.97.1, `CARGO_INCREMENTAL=0` and `--offline --locked`, under the Mac host rules.

- **Targeted and class tests** (RETURN §6), 43 of 43:
  - p = 53 against hardware at every width;
  - the hard classes at every required precision (12, 24 and 48 per width);
  - the conversion's boundary classes at L = 2, 4, 8 and 16;
  - a cross-check against `ExactAccumulator::round`;
  - TwoSum, TwoProduct, narrowing and the constructor against Fraction;
  - the §7.3-13 and §7.3-16 analogues;
  - the work counts.
- **The differentials:** 3 × 10^6 full-precision operations, 3 × 2·10^5 mixed-precision operations and 4 × 10^6 conversions, each from a recorded seed, with committed digests. K3a's two streams also run through the new core at L = 2 (RETURN §11).
- **Suites:** all 39 manifests of CI's cargo profile, `--no-fail-fast`, per test against the Mac baseline of main:
  - equal except frame_kernel (179 → 221 at `8cacbfaf4`);
  - the only failures are the three Mac platform tests, with failure blocks identical to main's;
  - warnings identical to main's.
- **FK on the merged tree at opt-level 0** (`de719cbdc` plus the revert): 227 passed, 0 failed, in 230.7 s. With RV12's follow-up tests it is 229 passed, 0 failed, in 249.1 s.
  - That is main `98b1723b1`'s 184 (the baseline's 179 plus the skew pin's 5), all present with the same status, plus K3's 43.
  - The other manifests and T9 are unaffected by the revert: the profile only applied when FK was the root package.
- **T9:** 112 of 112 byte-identical (Mac-only), as above.
- **Mutations, re-run at opt-level 0 on the merged tree:**
  - the NONE control is clean (227);
  - 25 source mutants (M1–M22 with variants), each killed at a behavioural test assertion, with the same killing tests as at checkpoint C and none compile-only;
  - adding a `[profile.test]` with `overflow-checks = false` (P1) or `debug-assertions = false` (P2) is killed by the guard test;
  - M17 (K3a's tie rule) is killed by K3a's suite and by K3's L = 2 cross-check, **not by K-D5's tests** (a note, ROOT; RETURN §13);
  - RV12's survivors RV1, RV2 and RV4 are killed by the new S2 tests, and its RV5 by the new `taildecides` vectors at L = 4, 8 and 16 (RETURN addendum 2).
- **Hygiene:**
  - rustfmt (stable 1.9.0) is clean on K3's files;
  - the non-test FK build has no warnings;
  - `git diff --check` is clean;
  - K3a's `gen_wide_vectors.py --check` gives OK for 6 of 6, and K3's `gen_wide_k3_vectors.py --check` for 8 of 8;
  - there are no machine paths in the diff or the records.

## Remaining

- **For ROOT and the manager:**
  - the profile-revert and records-addendum commits;
  - hosted CI, recording the numerical job's time in the merge record (Q7 condition);
  - the DEC-025 sweep under the owner's Mac decision;
  - the independent complete-diff review, with an oracle independent of K3's generator;
  - GEN-8 on the committed records.
- **K4:**
  - the first caller;
  - **the correctly rounded sum of an exact expansion,** which the K4 interface lacks (RV12's S1; ROOT assigns it in K4's brief);
  - RV12's N2–N5 (RETURN addendum 2): record each context once; `from_integer`'s flat cost; netting `ExactAccumulator`'s two magnitudes; 10^6-operation streams at K4's working precisions;
  - the `exact_sum.rs` read accessor (Q4);
  - Q8, the ceiling's p + 64 residual;
  - p = 128 and 192 run at L = 4.
- **Deferred:**
  - unifying `Binary64Outcome` with K2b's `Representability` (K4, F1b or F2a);
  - a faster ÷ and √ (Knuth's algorithm D, with its own vectors);
  - the review of the unused K3a accessors at T3 close.
- **The lesson (ROOT's, a T3-close note on test hygiene):** a test must not depend on how the compiler evaluates a function of unspecified precision. K3's tests call no such function (RETURN, addendum 1).
