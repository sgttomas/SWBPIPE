# KF3 checkpoint A: the change, the tests, the suites and the constructed model (I19)

**Status: no stop.**
- Amendment A2 and the partial-stage recording are implemented in `K4R/{bound,verify,adaptive}.rs`, with one reset method in `K4R/wide_sum.rs` (ROOT's ruling 7).
- Test outcomes:
  - FK's full suite passes (348 lib tests, plus the integration tests and doc-tests);
  - `gen_k4_vectors.py --check` passes on every file;
  - all 11 mutants are killed, and the NONE control passes.
- The uncommitted candidate is based on `75a1222a6` (main `0f5d8c7b4` plus the committed plan). Builds were run as one cargo job at a time (`-j 4`, `RUST_TEST_THREADS=2`, target `<wt>/kf3-target`). The memory guard ran and logged no kill.
- **Rulings:** "KF3: rulings on I19's diagnosis and plan; D1 revision 5a.3 amendment A2" (numerics `e49ec616a`).
- **Delegation:** a background subagent of ROOT's session, under D-GOV-35. I made no Git writes and no index operations.

## 1. The change

**`K4R/bound.rs`:**
- Amendment A2's types:
  - `RefusalKind` (Span, Exponent);
  - `BoundPass`;
  - `BoundRefusal` (kind, pass, elimination row);
  - `CertifiedBound`;
  - `BlockRefusal`.
- `refusable` records a block's first refusal and resets the accumulator in full. Every other stop, budget included, propagates.
- `u_pass` and `nl_pass` take the block of each row and a per-block refusal slot:
  - a refusal marks only its own block;
  - every later operation on that block's rows is skipped;
  - every other block runs operation for operation as before.
- `uc_bounds` never fails on a refusal. `BlockBound.refused` records it, and `uc` is `None`, R7's "does not exist".
- **The shift (7c):**
  - `shifted_factor` records exponent refusals per block;
  - `shift_schedule` carries the factorization's refusals and N′_L's, and δ/σ′/S's, into `ShiftResult.refused`;
  - a refused block is not retried.
- **New helpers,** shared by `verify_state` and the test entry `certify`:
  - `shift_start` (7c's start, with the Need and Sigma refusals);
  - `shift_run` (the profile and the schedule);
  - `certificates` (7d with A2);
  - `block_refusals`.
- **7d with A2, in `certificates`:**
  - B_c is the minimum over the bounds formed;
  - a block with data and no bound, after a refusal, stops the attempt with that refusal (ruling 1), and this takes precedence over `uc` (ruling 2);
  - without a refusal, `uc` applies as before.

**`K4R/verify.rs`:**
- `build_verify_shared` no longer stops on a refusal.
- `verify_state` stages each segment at its boundary, with the same values as before, and returns the pass's S_c refusals on every path.
- A `#[cfg(test)]` hook (`hooks::set_no_shift`) reads every est_c as 0, for W2 (ruling 5; V-K's `seeded` is not on main).

**`K4R/adaptive.rs`:**
- `Stage`, `StageWork::total`, `add_to` and `close_stopped`.
- `build_shared` and `solve_case_at`, like the two verification builds, keep the stage in progress and add the charged work their stages do not record to that stage when they stop.
- `AttemptRecord.bound_refusals` (ruling 3) holds the shared build's Uc_c refusals and the pass's S_c refusals, on every path.
- `StageGuard::with_case_room` is test-only.

**`K4R/wide_sum.rs`:** `reset` zeroes both magnitudes in full and keeps the work counts (ruling 7).

**Nothing else moves:**
- Every success path runs the same operations in the same order.
- The S11 site table is unchanged: no new `+=`, `-=`, `.sum(`, `fold(` or self-assignment fold outside `#[cfg(test)]`, and the table test passes.

## 2. The constructed model (ruling 4)

**The plan's slender-chain sizing was wrong.** Measured through GEN's exact emulation of K4:
- slenderness shifts log2 U_c by a constant and does not change its rate;
- a planar chain grows only about 1–4 bits per member.

A search over rational-axis chains found **KF3-UC-SPAN**:
- **Geometry:** a straight chain of 390 members along (−1, 12, −12), each 17 long. y_ref is (−12, −9, −8), orthogonal to the axis with norm 17, so the local axes are rational. It is fixed at node 0, with a tip force and moment.
- **Section:** R1's RF-LARGE section scaled by exact powers of two: A·2^-26, Iy·2^16, Iz·2^28 and J·2^10. These are invented inputs.
- **Growth:** M(L)⁻¹ grows 20.8 bits per member (GEN, at 40 and 80 members), and the first refusal at 256 falls between 370 and 380 members.
- **Size:** 2,340 free DOFs, one block. That is above the plan's "under about 200 members", and it is the smallest chain found.

**Before and after:**

| | Main `0f5d8c7b4` (`git archive`, `_run_records/a/before_main_0f5d8c7b4.txt`) | KF3 |
|---|---|---|
| KF3-UC-SPAN | `Unresolved(ExactSumSpan)`. 128 rejected (verification failed), 256 `Failed(Stop(Span))`. The 256 verification-shared total is 82.39 M LME against 70.51 M staged, so 11.88 M of partial `uc` work is unstaged: RF-LARGE-10,000's pattern. | Selected at 128, verified at 256, token-equal to GEN. |
| KF3-UC-SPAN-ZERO (no loads) | The same stop. | Selected at 128. The block has no data and needs no bound; its refusal is only recorded. |

**KF3-UC-SPAN under KF3:**
- **The refusal:** at 256 the block's Uc_c is refused at `refused:span:backward:66`, and `bound_refusals` records it on the 256 attempt only.
- **The bounds:** B = S_c ≈ 2^62, from one shifted factorization. θ is 8.0e-55, C/allowance 2.6e-37 and Ŵ/V 1.4e-2. E-UNIT, E-UC and E-CHARGE are bit-equal to GEN.
- **Honesty and G5a:** `compare_honest` against GEN's two high-precision solves, 300 and 240 digits (the dense exact solve is out of reach at 2,340 DOFs).
  - 8,983 checks; the worst is 0.969 of its allowance, at `mag.55`.
  - That value is 2.03, just above a power of two, so the binary64 publication's half-ulp alone can reach about 0.985 of the relative allowance. This is the publication rounding, not the method.
  - G5a passes.
- **W2 (the hook):** `Unresolved(ExactSumSpan)`, with tokens `128:rejected:verification_failed 256:failed:Span`, and the refusal recorded, as before A2.

**Debug CI time (ruling 4), this Mac, `RUST_TEST_THREADS=2`:**

| Test | Time |
|---|---|
| KF3-UC-SPAN's schedule | 13.6 s |
| KF3-UC-SPAN-ZERO's schedule | 9.1 s |
| W2 | about 14 s |
| The bit-level test (E-UNIT at 128 and 256, E-UC and E-CHARGE at 256, both models) | about 45 s |

The whole `kf3` filter takes 38–64 s of wall time. No single test exceeds 60 s. GEN's `kf3.txt` takes 2 minutes, and the full `--check` 17:48 against 15:11 at base.

## 3. Tests

**`K4T/bound_tests.rs`:**
- U1 and U2: a synthetic block with K̃ = L·Lᵀ, where L's node block m·[[1, −1], [1, −1]] has m = 2^20 and is nilpotent. M(L)⁻¹ grows 21 bits per node, while ‖K̃⁻¹‖₁ = (1 + 2m)² exactly.
  - Its Uc_c is refused in the backward pass. A second block beside it is bit-identical to its standalone run.
  - B = S_c ≥ the exact norm. Where both bounds exist, B = min(Uc_c, S_c).
  - The certificates and the evidence are checked.
- U5 and the precedence: a data block with neither bound stops; the same block without data does not; a refused block's stop outranks another block's `uc`.
- U3: a budget stop inside `uc` stays a budget stop.
- U4: σ = 2^-9000 makes σ′'s subtraction span, so S_c is refused and not retried, and B = Uc_c. The accumulator is exact after the refusal.

**`K4T/wide_sum_tests.rs`:** `reset` (ruling 7):
- a refused term writes nothing;
- `clear` leaves a limb beyond the used prefix, and `reset` zeroes it;
- the work counts are kept, and the accumulator is exact afterwards.

**`K4T/kf3_tests.rs`** (new, mounted in `adaptive.rs`):
- `kf3.txt` is pinned;
- the bit-level test;
- W1 (KF3-UC-SPAN), W3 (ZERO) and W2, as in §2;
- **every build's stages equal its charged total:**
  - budget sweeps at every stage boundary, halfway into every stage and at 31 even points, for `build_shared`, `solve_case_at`, `build_verify_shared` and `verify_state` on N05 and TWO-SPAN;
  - `Condition` (SKEW-K1E-28) and `Pivot` (PIVOT) in `build_shared`;
  - `ResidualGate` after the fallback (N09-B perturbed);
  - `ResolutionScale` in `verify_state` (EHAT-OVERFLOW);
- a budget stop halfway into `uc` on KF3-UC-SPAN, K6b's case, stages the partial `uc` work.

**`K4T/method_tests.rs`:** the controls test now also asserts, on every attempt of every control and combination, that the stages equal the charged totals (the own work, and the shared plus verification-shared work). `verification_in` takes its record texts.

**`K4T/scale_tests.rs`:**
- `bounds_at` reads a refused block's token, and skips the norm checks where the norm is `-`;
- `certify`'s stop is asserted;
- the helpers are made `pub(super)`.

**GEN (`gen_k4_vectors.py`):**
- the span mirror (`sum_span`, `add_dir`, `mul_up`, where the product's span is asserted), used in the A2-aware passes;
- `uc_from_em_a2`;
- `shift_schedule_em(a2=True)`;
- 7d with A2 in `verify_em`;
- `hp_only` expectations;
- `kf3_models` and `kf3_lines` write `kf3.txt`.

Every earlier record is byte-identical.

## 4. Mutants (`_run_records/a/mutants.txt`)

One at a time, each on a clean copy with its own target, deleted afterwards. Filter: `kf3` plus the reset test.

| Mutant | Result | Killed by |
|---|---|---|
| NONE | 12 of 12 pass | — |
| M1: refused Uc as 0 | killed | 5 tests, incl. W1 (B = 0 fails G5a) and U1 |
| M2: B = max | killed | U1 |
| M3: stop dropped | killed | W2 and U5 |
| M4a: `build_shared` unstaged | killed | the sweep |
| M4b: `solve_case_at` unstaged | killed | the sweep |
| M4c: `build_verify_shared` unstaged | killed | the sweep and the `uc` budget test |
| M4d: `verify_state` unstaged | killed | the sweep |
| M5: refusal not per block | killed | U2 |
| M6: budget as a refusal | killed | U3 |
| M7: a refused block without data stops | killed | W3, U5 and the bit-level test |
| M8: pre-KF3 (any refusal stops) | killed | 7 tests |

## 5. What moved

- **No control changes outcome, row, class, bound or golden work count.** K4's controls test, the goldens, KF1's tests and GEN's `--check` all pass unchanged.
- **The stage records of failed attempts now hold their partial stage,** with their totals unchanged. In `outcomes.txt` that is 24 `Pivot`, 2 `Condition` and 3 `ResolutionScale` attempts. No golden pins these stages.
- **The only controls that move** are the new `kf3.txt` controls, from a stop to selected (§2).
- **For K6b:** its parity check can use equality on every path. Its test that asserts `shared_stages.uc == 0` on a stopped build now sees the partial `uc` work. Whichever of KF3 and K6b merges second updates both (ROOT's ruling).

## 6. Changed files (sha256)

| File | sha256 |
|---|---|
| `FK/src/structural/retained/adaptive.rs` | `a4f91ccfea93920a64118aa9c875b03bacbb1857493c5c1fd03642b4b494730c` |
| `FK/src/structural/retained/bound.rs` | `69fcafc83f37e58c4a150c1c8b86ceb408176601b119a70ba565c44eeaddc243` |
| `FK/src/structural/retained/verify.rs` | `1917ca707785b586ddef7861945debf636cae5353dc3153021721d3f2e03a6c6` |
| `FK/src/structural/retained/wide_sum.rs` | `2c151cdd60d5a7d730b525e0fa73a671429283d34562edc573eb43b660e3689d` |
| `K4T/SHA256SUMS` | `3aebae45d93eb3bc017298f42d316d81a6ef2004a5e86ff414f485b418917fa9` |
| `K4T/bound_tests.rs` | `fdf10144c9490e12a071251d5b1915a124c17dc6290792255fa00f6901e23102` |
| `K4T/gen_k4_vectors.py` | `8c8f22aa8492736624045f866f5f8b47e9b3ee4639e1d038e02fa2425b6c7aa7` |
| `K4T/method_tests.rs` | `8f60c0faab1538b0e6a3785998033d7d2d7c0ae080f6ba01ad63ac8e890c498e` |
| `K4T/scale_tests.rs` | `922f03cf28cdfa402b16e254077a2f2fc3bb553acf77581be9d828177243c350` |
| `K4T/wide_sum_tests.rs` | `94ee29a25bee27b2a21f31212a49a1ac7c6cee72796aa95a13158ab5bc0dcbc8` |
| `K4T/kf3.txt` (new) | `729e7ef86ba0b61353a167a610d53910e0deddb357c71948dd742993fca6f75a` |
| `K4T/kf3_tests.rs` (new) | `43b37a782d4985f2507323d027c0866a17373e94f117a39ae17abec254c88b68` |

Records are in `T3/IMPLEMENTATION/KF3/`: this file and `_run_records/a/`.

## 7. Not done, and what is next

- **B**, in a slot ROOT grants:
  - RF-LARGE-*-n10000 through W1, per plan §9, using main's drivers if V-K and K6b are merged, else read-only `git archive` copies;
  - the refusal's pass and row per frame;
  - honesty against R1.
- **D:** RETURN, CHANGE_RECORD and SHA256SUMS.
- Nothing ran at scale. No timing or memory claims are made.
