# RV2: independent full-diff review of slice K3a (`Wide<2>` arithmetic)

**Verdict: PASS.** There are no BLOCKING findings. There are 2 SHOULD-FIX findings (S1, S2) and 7 NOTEs (N1–N7).

**Reviewer:** RV2 (TASK, Type 2), working under `TASK_BRIEFS/RV2_K3A_REVIEW.md` and `_COMMON.md`.
- I am independent of K3a. I did not design W1 or K3a, review their design, implement K3a or advise on it.
- I found defects and fixed nothing.
- I made no Git writes. The candidate worktree was read only.
- I built and ran everything in scratch copies made with `git archive`, not in a worktree. This was to keep free disk above the 8 GB floor.

## 1. Revisions reviewed

| Item | Value |
|---|---|
| Candidate | branch `codex/piping-k3a-20260926`, head `a2e804a757359d589f4c31ea8e36a923f28ccb8c` |
| Base | `4912dc6368be87636334162acae6e64ca7427ac2` (the S11-K PR head at K3a's cut) |
| Diff reviewed | `git diff 4912dc636..a2e804a75`, complete: 60 files, +17 984 lines |
| After S11-K merged | The manager merged origin/main (S11-K, PR973, `3488a236a`) into the branch, giving head `43da7a24e41dd3d46c60f324bcfbf883e1e26f22` (parents `a2e804a75` and `3488a236a`). **`diff <(git diff 4912dc636 a2e804a75) <(git diff 3488a236a 43da7a24e)` is empty**: I ran it in the K3a worktree, and the output is 0 bytes. Between the two bases, only `FK/src/structural/s11k_tests.rs` changed inside `frame_kernel`. So this review of `a2e804a75` covers K3a's diff at `43da7a24e`. |
| Basis | T3 worktree at `d00aab779`: `ROOT_SELECTION_DESIGNS.md` (C1, C5); `DESIGN_NUMERICS/DESIGN.md`, sha256 `fb62ef4a…` (verified; the K3a row, §4.1.1 `wide.rs`, §4.11, §4.3.1, §7.4); `TASK_BRIEFS/I2_K3A_IMPLEMENTATION.md`; `ROOT_RULINGS_V1.md` "K3a arctangent and host disk"; `REVIEW/VERIFY_R5.md` item 17 and `probe_atan_p128.{py,stdout}.txt` |

`P/` = `projects/chirality-piping/`. `FK/` = `P/core/solver/frame_kernel/`. `K3A/` = `T3/IMPLEMENTATION/K3A/` in the candidate.

## 2. Findings

| ID | Severity | Site | Evidence | Resolution |
|---|---|---|---|---|
| S1 | SHOULD-FIX | `FK/src/structural/retained/wide.rs` (`ATAN_TOLERANCE_ULPS` and the "Measured, and the specified test tolerance" paragraph); `K3A/RETURN.md` §3; `K3A/CHANGE_RECORD.md` ("The arctangent") | The records call 6 ulp "the measured bound, rounded up" and "Specified test tolerance … (measured, C1)". It is not a bound. On my independent set, the input `angle 128 +ff4014cc2258bc73504f91ab8019dddcp-1 -9c9e9003feb63e97c537ba9a6156981bp-4` (φ ≈ 1.6473) gives **6.0818 ulp** at p = 128. My Newton reference and I2's own 160-digit reference agree on that figure, and I2's Python emulation reproduces the Rust result bit for bit. So this is real behaviour of the specified algorithm, not a defect in the arithmetic. It stays inside the proved 23.6 ulp and my own bound of 16.2 ulp (§4). Current tests still pass, because they assert 6 ulp only on the committed vectors. | Say plainly in all three places that 6 ulp is a regression tolerance for the committed vectors only. The accuracy contract, which K-D5 may cite, is the proved bound: 23.55·2⁻ᵖ relative, or 23.6 ulp. Record RV2's 6.08-ulp input as the counterexample. A code change is optional. If the RV2 input is added to `atan.txt` (through the generator and `SHA256SUMS`), the tolerance must rise to cover it and stay ≤ 23.6, as ROOT's ruling allows. Either path closes S1. |
| S2 | SHOULD-FIX | `FK/tests/retained_wide/atan.txt` and `gen_wide_vectors.py` (the arctangent vector set) | The documentation and the records state that the series tail is summed smallest first, and the test claims bitwise equality with the emulation. My mutant R2 sums the tail largest first, and it **survives** the whole suite (22 of 22 passed). On my 26,000 arctangent vectors, R2 changes 13 results by 1 lsb, for example `angle 128 +afe8cd16013c9f292103f35b822d7f35p-1 +b9fd2bf4af032d764c3c967ef87dbbc0p-1`. None of the 3,293 committed vectors separates the two orders. Accuracy is unaffected, because the proof holds for any order of the tail. What is missing is that the committed vectors do not pin the documented, disclosed summation order, so a later refactor could change results silently. | Add at least one input that separates the orders (the 13 are in `_run_records/k3a_review/r2_distinguishing_inputs.txt`) to the generator's arctangent set. Regenerate `atan.txt` and `SHA256SUMS`, and confirm that R2 is then killed. |
| N1 | NOTE | `K3A/RETURN.md` §1 and §4; `K3A/CHANGE_RECORD.md` ("Tests and results") | K3a adds **19** tests, not 22. `wide_tests.rs` has 19 `#[test]` items. `cargo test --lib -- --list` gives 108 unit tests at the base and 127 at the candidate. The base total is 108 + 2 + 6 = **116**, not 113. The figure 22 is the count matched by the filter `retained`: 19 new tests plus 3 pre-existing `exact_boundary::tests::retained_*` tests. The candidate total of 135 is correct. | Correct the counts in the records. The mutation results are unaffected. |
| N2 | NOTE | `K3A/RETURN.md` §7 ("Whitespace: … clean on every new file") | Over the full diff, `git diff --check` reports "new blank line at EOF" in 24 raw logs under `K3A/_run_records/suites/`. The claim holds for source, tests, vectors and records outside `_run_records` (`git diff --check … -- ':!**/_run_records/**'` is clean), but the exception is not recorded. The skill treats this as cosmetic, and raw tool output is left verbatim. | Record the exception, or narrow the claim. No edit to the logs is needed. |
| N3 | NOTE | `wide.rs` module documentation ("Proved bound … For exact inputs, the result φ̂ satisfies …") | The arctangent returns `ExponentRange` instead of a value when t² or t³ leaves ±2⁶²: roughly e_t > 2⁶¹, or e_t < −2⁶²/3. `exponent_extremes_are_refused_never_wrapped` asserts this. The bound statement reads as if every domain input returns a value. The result is a refusal, never a wrong value, and K-D5's inputs are O(1). | Add "or is refused with `ExponentRange`" to the bound statement. |
| N4 | NOTE | `wide.rs` proof, step 5 | This is only slack and loose wording in the proof; nothing is wrong. The stop-rule tail is written as (1 + γ₃₁) where 1/(1 − γ₃₁) is exact, which is an O(u²) difference. The margin from 23.54u to 23.55u to 23.6 ulp absorbs every such term at p ≥ 53. The proof also bounds the sensitivity of atan by 1 and each reduction by 4u. | None required. My tighter first-order bound (§4) may be cited if useful. |
| N5 | NOTE | K-D5 interface (`included_angle`) | With 128-bit inputs, angles within about 10⁻¹⁹ of π have c rounded to exactly −1. They are refused with `AngleDomain`. My set had 462 such refusals, all correctly refused. I2's set records the same effect (`pi128` k ≥ 20). This is correct for the stated domain (1 + c > 0), but K-D5 must handle it: a near-straight-back elbow is a refusal, not an angle. | K-D5 should map `AngleDomain` or `ExponentRange` from the re-formation to its "cannot re-form" route (D1 §4.3.1). No change to K3a. |
| N6 | NOTE | `wide_tests.rs` `atan_error` | The measured error, the magnitude of got − hi − lo, is formed with two roundings at p = 128, so the error measure itself carries a relative error near 2⁻¹²⁸. That could only flip a verdict within 2⁻¹²⁸ of a limit. | None required. |
| N7 | NOTE | `K3A/CHANGE_RECORD.md` | The record names the branch and base but not the checked head, which could not be known before the commit. It also points to RETURN §5 for the fixture result instead of stating "112 of 112 byte-identical". | The manager adds the candidate, PR and merge revisions in the PR record, as the chirality-change skill requires. |

## 3. Checks

### 3.1 The arctangent proof, checked step by step

I read the proof in the `wide.rs` module documentation and in `RETURN.md` §3, and checked every step by hand against the code in `atan_of_positive` and `included_angle`.

- **Lemma A: valid, and loose.** The mean value theorem gives |atan t̃ − atan t| = |η|t/(1 + ξ²) with ξ ≥ t(1 − |η|). Then t/(1 + t²(1 − |η|)²) ≤ atan(t)/(1 − |η|)², using t/(1 + t²) ≤ atan t. A sharper constant, |η|/(1 − |η|), also holds. The looser one is still a valid bound.
- **Initial step: valid.** `d = rnd(1 + c)` and `t = rnd(s/d)` give t̂₀ = T(1 + δ₂)/(1 + δ₁), so |η₀| ≤ 2u/(1 − u) ≤ γ₂. The domain check on the rounded d is exactly equivalent to 1 + c > 0: round-to-nearest in this format has no underflow, so it never turns a positive value into zero or a negative one.
- **Half-angle reduction error: valid.**
  - a = t²(1 + δ₁), and 1 + a = (1 + t²)(1 + δ₁t²/(1 + t²)).
  - r = √b·(1 + δ₃) with 1 + ρ ∈ [(1 − u)², (1 + u)²].
  - 1 + r = (1 + √(1 + t²))(1 + ρ′), where ρ′ = ρ·√(1 + t²)/(1 + √(1 + t²)), which lies between 0 and ρ.
  - Then t′ = R(t)(1 + δ₅)/((1 + ρ′)(1 + δ₄)). The worst case is (1 + u)/(1 − u)³ − 1 = (4u − 3u² + u³)/(1 − u)³ ≤ β₄. The other side, (4u + 3u² + u³)/(1 + u)³, is smaller.
  - The code matches: `t2 = mul(t,t)`, `b = add(1,t2)`, `r = sqrt(b)`, `d = add(1,r)`, `t = div(t,d)`.
- **Propagation through the reductions (the ×2 per step): valid.** The exact map halves atan, so θ̂_{j+1} = (θ̂_j/2)(1 + α_{j+1}). Relative errors therefore add; they do not double. The final 2^k scaling in `sum.mul_pow2(reductions)` and the ×2 in `included_angle` (`mul_pow2(1)`) are exact exponent shifts. They cannot fail, because φ < π.
- **At most 5 reductions to reach t < 0.05: valid.**
  - θ̂₀ < π/2, so θ̂₅ < (π/64)(1 + 22u + O(u²)). Then tan θ̂₅ < tan(π/64)·(1 + ε) = 0.049127… < 1/20 for any u ≤ 2⁻⁵³.
  - The loop test `at_least_one_twentieth` is exact: 5·sig ≥ 2^(125 − e), with the out-of-range branches k ≤ 0 → true and k ≥ 131 → false. I checked it by hand.
  - So the `ArctangentLimit` branch is unreachable.
- **Series truncation after at most 15 terms: valid.**
  - The stop test is `term.exponent < e_t − p − 1`, which means |computed term| < 2^(e_t − p − 1).
  - For N = 15, |a₁₅| < 2^(e_t+1)·0.05³⁰/31·(1 + γ₃₁) ≈ 2^e_t·6.0·10⁻⁴¹. The stop threshold is 2^e_t·2⁻¹²⁹ ≈ 2^e_t·1.47·10⁻³⁹, so a₁₅ always stops at p ≤ 128. The documentation's "0.05³⁰/31 < 2⁻¹³⁰" is the same inequality.
  - The alternating-series tail is at most |a_N| ≤ (u/2)·t·(1 + O(u)), because 2^e_t ≤ t.
- **Rounding error per operation at p: valid.** Every operation goes through `round_pack`, which rounds once and correctly (§3.5), so |δ| ≤ u = 2⁻ᵖ. The divisor 2n + 1 is exact (`from_f64`), and the negation is exact.
  - Each computed term has relative error at most γ_{2n+1}: t² rounded once, then n multiplications and one division.
  - The term sum Σ_{n≥1}|a_n| ≤ t³/(3(1 − t²)) < 8.354·10⁻⁴·t.
  - Term errors are at most γ₂₉ times that sum. Summation error is at most γ₁₃(1 + γ₂₉) times it (m ≤ 14 terms; the first `add` onto zero is exact). The final t + tail is rounded once.
  - Since atan t ≥ 0.99916·t, |σ| ≤ (1 + 0.5004 + 0.0351)u ≤ 1.54u.
- **Final assembly: valid.** φ̂ = φ(1 + α₀)∏(1 + α_j)(1 + σ), with first-order coefficient 2 + 4·5 + 1.5355 = 23.5355, so at most 23.55u. With u·φ̂ < ulp_p(φ̂), this is below 23.6 ulp_p(φ̂).
- **Domain: correctly enforced.**
  - s ≤ 0, including −0, and 1 + c ≤ 0 give `AngleDomain`. p < 53 gives `InvalidPrecision`, and p > 128 cannot be constructed.
  - t up to 2^±100000 is tested (`t2:±100000`).
  - Only exponents beyond about ±2⁶⁰ are refused (N3).
- **Precision range 53 ≤ p ≤ 128: correct and necessary.** p ≤ 128 is needed for the 15-term cutoff. p ≥ 53 makes the reduction-count margin and the O(u²) terms harmless. At p = 10, for example, tan(π/64·(1 + 22u)) would already exceed 0.05.

**Conclusion: 23.6 ulp is a valid upper bound for exact inputs.** My own, tighter bound is in §4.

### 3.2 Smallest-first tail summation

- **What the code does.** It collects the terms, sums the tail in reverse order (smallest first, `terms[1..count].iter().rev()`), and adds t last with one rounding.
- **What the proof assumes.** The proof uses the generic recursive-summation bound γ_{m−1}·Σ|x_i|, which holds for any order of the tail terms. The load-bearing assumption is that t is kept out of the running sum and added once at the end. The code does exactly that, so the proof holds for this order and for any other order of the tail.
- **Deterministic.** The order is fixed and the arithmetic is integer-only.
- **Why V1's variant scores higher.**
  - V1's variant (`v1_variant` in the generator, and V1's `probe_atan_p128`) sums forward from t, and stops when a term falls below ulp(acc)/4. Each of up to about 13 terms above the ulp is therefore added to a running value near t, with a rounding of up to half an ulp of t each time.
  - I2's order sums the tail in a binade about 2⁻¹⁰ lower, so those roundings cost about 8·10⁻⁴ ulp each. Only the final addition of t costs up to half an ulp.
  - I decomposed the two worst vectors with the generator's emulation:
    - `rand128:768`: series error 0.33 ulp (I2) against −2.67 ulp (V1); totals 4.28 and 7.28.
    - `rand128:1181`: 0.47 ulp against −0.53 ulp; totals 5.41 and 4.41.
  - The reduction errors, up to about 4 ulp here, are common to both variants. The extra 3 ulp in V1's series is the forward-summation cost.
  - On any single input either variant can be worse; I2's has the smaller worst case. On RV2's 6.08-ulp input, V1's variant gives 4.08.
- **The regenerated figures match.** Regenerating with `gen_wide_vectors.py` gives V1's variant at 7.2784 ulp and I2's at 5.4111 ulp.

### 3.3 The `#[path]` test module

- **The attribute and its path.** `wide.rs` ends with `#[cfg(test)] #[path = "../../../tests/retained_wide/wide_tests.rs"] mod tests;`. A `#[path]` on a module declared in a non-`mod.rs` file resolves relative to that file's directory, so it lands on `FK/tests/retained_wide/wide_tests.rs`. Its `include_str!` calls resolve beside that file.
- **It runs in `frame_kernel`'s own suite.** `cargo test` on `FK` ran 127 unit tests, including all 19 `structural::retained::wide::tests::*`, plus 2 `s11_site_table` tests and 6 doc tests, all passing.
- **It runs in CI.** CI's numerical job (`.github/workflows/piping-desktop-e2e.yml` → `tools/ci/numerical_ci.py`) runs `cargo test --offline --manifest-path` for every manifest under `P/core` and `P/validation/benchmarks`. Any change under `P/core/` sets `numerical_required` (`tools/ci/e2e_plan.py` `numerical_input`).
- **Not silently excluded.** `tests/retained_wide/` has no `main.rs`, so cargo does not auto-discover it as a separate integration target; it is compiled only through the `#[path]`. The unit-test list grew by exactly the 19 tests.
- **Nothing test-only leaks.** The module is `#[cfg(test)]`. The non-test `cargo build --lib` has no warnings. The non-test code contains only the documentation constants `ATAN_TOLERANCE_ULPS` and `ATAN_PROVED_BOUND_TENTH_ULPS`, which have no behaviour.
- **Release mode is sound.** `cargo test --release --lib retained` passes: 22 matched, 19 of them K3a's, with `debug_assert`s compiled out. It reports the same arctangent worst figures.
- **Doc tests are unaffected.** `cargo test --doc` gives 6 passed, the same 6 as the base. The only change is that the `structural.rs` doc-test line numbers shift by 1. The proof's indented list continuations are not parsed as code blocks.

### 3.4 The seeded-differential regeneration scheme

- **It detects a wrong result anywhere in the stream.** For every record, the Rust test hashes (op, [p], status, sign, exponent, both limbs) into a per-10⁵-record chunk digest and the whole-stream digest, and compares both with `differential.txt`. A wrong result at any index changes its chunk digest, so detection does not depend on the 1,000 sampled records. The samples add two things: a check of the generator port (operands record by record) and readable failure messages.
  - Operands are not hashed. A port divergence would make the digests mismatch, which is a false failure, not a false pass. The only false pass is a SHA-256 collision.
  - Any error other than `DivisionByZero` panics.
  - The record encoding matches the Python `enc()` byte for byte (I read both).
- **`gen_wide_vectors.py --check` regenerates everything byte for byte.** Run from the candidate archive with Python 3.11.15 (DEC-025 venv), it printed `OK` for `targeted.txt`, `split.txt`, `atan.txt`, `differential_sample.txt`, `differential.txt` and `SHA256SUMS` (53 s). The digests `b568d2c0…31253a2` (p128) and `f8b008bc…a443603` (mixed) were recomputed from the oracle.
- **The expected values come from `Fraction`, not from the code under test.**
  - The generator's `apply` → `o_add`/`o_mul`/`o_div` use exact `Fraction`s, rounded by `rnd`. `o_sqrt` uses `isqrt` with at least p + 2 root bits and an exactness sticky.
  - No generator output is read from Rust.
  - I checked the generator's oracle with my own, independent one (§3.5): all 2,000 sample records, all 1,712 targeted non-EFT vectors, all 360 TwoSum/TwoProduct vectors (s = rnd(x∘y) and s + e = x∘y exactly) and all 822 split vectors agree with it. There were no mismatches.

### 3.5 Correct rounding, independently

I wrote my own oracle (`rv2_oracle.py`). It shares no code with I2's generator.
- **Rounding.** It rounds a `Fraction` by comparing it exactly with the two neighbouring p-bit values and their midpoint.
- **Square root.** It decides by bisection on exact squares, then compares with the midpoint squared.
- **Self-test.** It matched hardware binary64 on 100,000 operations (add, sub, mul, div, sqrt) at p = 53.

I then ran 17,060 of my own vectors through the candidate's `Wide` in a scratch probe (`rv2_probe.rs`), at p ∈ {53, 64, 65, 127, 128}. They cover:
- random operands of 1 to 128 bits;
- exact ties to even with both parities, for add and sub (and at p = 128, ties formed by the sum);
- the same ties with sticky bits 140, 200 and 300 binades below, on either side;
- multiplication ties (products of exactly p + 1 bits);
- division ties and ±1-lsb neighbours;
- near-midpoint quotients with 128-bit divisors;
- perfect squares and ±1 lsb;
- (m + ½)² cut to 128 bits, which are near-midpoint square roots;
- massive cancellation (1–3 ulp apart, across a binade, and x − x);
- carries (all-ones + half-ulp, all-ones², √ of all-ones);
- signed zeros for every operation;
- 3,011 f64 lifts: random patterns, 30 % subnormal, ±0, extremes, ±∞ and NaN;
- 3,000 splits: across the binary64 range, near 2⁻¹⁰⁷⁴, and below it.

**Result: 0 mismatches** (add 2,515, sub 2,125, mul 1,670, div 2,325, sqrt 2,414, f64 3,011, split 3,000).
- The lift is exact for normal and subnormal values and keeps the sign of zero. Non-finite input gives `NonFinite`.
- The split has at most 3 terms, strictly decreasing, with the value's sign. It is exact exactly when no set bit lies below 2⁻¹⁰⁷⁴. Otherwise it sets the flag, and the remainder is nonzero, below 2⁻¹⁰⁷⁴, with the value's sign; the kept part is the truncation toward zero. Values ≥ 2¹⁰²⁴ are refused.
- The design's allowance (D1 §4.3.1: truncation below 2⁻¹⁰⁷⁴·|u_j|) is what `add_product_to` reports.

**Code reading.** I reread `round_pack`, `add_exact_operands`, `mul_u128`, `divide_significands`, `sqrt_significand`, `shr_sticky`, `any_below`, `from_f64`, `split_binary64` and `exact_binary64` for boundary errors and found none. In particular:
- the sticky-precondition cases hold: sum ≥ 2²⁵⁵; a sticky difference is ≥ 2²⁵⁴ because the gap is ≥ 129; the quotient is ≥ 2¹²⁸; the root is ≥ 2¹²⁸;
- the borrow `big − small_mag − 1` with the rest 1 − f is correct;
- the rounding carry at p = 128 is correct;
- all shift amounts stay below the operand width;
- the exponent is checked after the rounding carry.

### 3.6 The mutation sample

The sample was run in a separate scratch copy with its own target directory, using `cargo test --offline --locked --lib retained -- --test-threads=4` (runner `rv2_mutate.py`). Each patch was applied to `wide.rs`, and the file was restored and verified afterwards.

| Mutant | Patch (in `wide.rs`) | Result | Killing tests |
|---|---|---|---|
| M2 dropped sticky (I2's) | `let rest = m.any_below(drop - 1);` | killed | `targeted_…`, both differentials, `arctangent_vectors_…`, `one_twentieth_…` (5), matching I2's record |
| M4 off-by-one limb shift (I2's) | `shr_sticky`: `self.hi << (127 - n)` | killed | `targeted_…`, both differentials, `split_vectors_…`, `split_is_exact_…`, `arctangent_vectors_…` (6), matching I2's record |
| M5 one reduction fewer (I2's) | at most 4 reductions, then `break` | killed | `arctangent_vectors_…` (1), matching I2's record |
| M6 truncated series (I2's) | `for n in 1..=8` | killed | `arctangent_vectors_…` (1), matching I2's record |
| **R1** series stop threshold off by one (RV2's, arctangent) | `threshold = e_t − p` (was `− p − 1`) | killed | `arctangent_vectors_…` (bitwise; the reference check alone would pass, because the added error is at most 1 ulp) |
| **R2** tail summed largest first (RV2's, arctangent) | `terms[1..count].iter()` (was `.rev()`) | **survived** | none. On RV2's 26,000 arctangent vectors it changes 13 results by 1 lsb (for example `angle 128 +afe8cd16013c9f292103f35b822d7f35p-1 +b9fd2bf4af032d764c3c967ef87dbbc0p-1`). None of the 3,293 committed vectors separates the two orders. See S2 |
| **R3** split flag lost for a wholly dropped piece (RV2's, split) | `drop >= 64` branch skips setting the flag | killed | `split_vectors_…`, `split_is_exact_…` (2) |
| **R4** missing borrow in sticky subtraction (RV2's) | `if false && sticky { diff −= 1 }` | killed | `targeted_…`, `seeded_…_mixed_precision` (2) |
| **R5** split keeps a wrong lsb after truncation (RV2's, split) | `lsb = −1075` after the shift | killed | `lift_…`, `split_feeds_…`, `split_is_exact_…`, `split_vectors_…` (4) |
| **R6** arctangent zero denominator unchecked (RV2's) | domain test `d.negative` only | killed | `arctangent_domain_…`, `arctangent_vectors_…` (2) |
| **R7** square-root sticky dropped (RV2's) | `(root, false)` | killed | `targeted_…`, both differentials, `arctangent_vectors_…` (4) |

Each mutant log shows the mutated crate compiled (11 of 11). `wide.rs` was restored afterwards, and its sha256 prefix `9aa1900f…` was verified.

**A setup error on my side, disclosed.** My first mutant and probe pass shared one `CARGO_TARGET_DIR` across copies whose package identity is the same. Cargo reused a stale binary, so no mutant was compiled (every run showed "3 passed; 105 filtered out"). I discarded that pass and re-ran everything with a separate target directory per copy. The results above come only from the re-run, and every mutant log shows the mutated crate being compiled. A second, harmless slip: my runner's sweep check matched the command lines of waiting shells and kept reporting "busy". I stopped it with SIGINT, which restored `wide.rs` (hash verified), fixed the pattern to match only the Python sweep process, and re-ran all 11 mutants.

### 3.7 Zero byte change

- **No manifest, lockfile, fixture or schema change.** Outside `K3A/**` and `retained/**`, the diff touches only `FK/src/structural.rs`, with `+1 −0`: `mod retained;`. No `Cargo.toml`, `Cargo.lock`, fixture or schema is in the diff, there is no new dependency, and `frame_kernel` has no `[dependencies]`.
- **No change by construction.** `retained` is private and every item is `pub(crate)` or narrower. Nothing outside `retained/` refers to it (grep; the other "retained" hits are unrelated strings in `exact_boundary.rs`). It adds no statics, no global state and no trait impls on foreign types. `#![allow(dead_code)]` is the only allowance, and it is scoped to `retained`. So no product path can reach it, and no published byte can change.
- **I2's fixture run.** I did not re-run the 112-output fixture harness (disk and host priority). I read I2's `fixture_diff/fixture_identity.json`: 112 of 112 identical (fixtures 72, validation 30, core 10), with the 6 ERR/PANIC rows identical on both sides. The harness source hash, `ec089c1d…`, is S11-K's recorded `fixdiff_main.rs.txt`.
- **Test lists compared.** The base and candidate `frame_kernel` test lists differ only by the 19 new tests and the shifted doc-test line numbers.

### 3.8 The `dead_code` allowance

It is an inner `#![allow(dead_code)]` in `retained/mod.rs`, commented "K-D5 removes this allowance", and nothing wider. It is the only `allow` under `retained/`, and `lib.rs` and `structural.rs` gain none. The candidate's `cargo build --lib` and its test builds have no warnings.

### 3.9 Hygiene

- **No machine paths.** The diff has none; the only match is the sanitizing regex in `mutate.py.txt`. The records use `<k3a-worktree>`, `<scratch>` and `<k3a-target>`.
- **rustfmt.**
  - `rustfmt --check --edition 2021` (rustfmt 1.8.0, host stable, because 1.97.1 has no rustfmt) is clean on `retained/mod.rs`, `retained/wide.rs` and `wide_tests.rs`.
  - Checking from `structural.rs` reports only the pre-existing `exact_boundary/functionals{,/tests}.rs` differences, which are identical at the base.
- **`git diff --check`.** It is clean outside `_run_records`. There are 24 raw-log EOF blank lines (N2).
- **Record hashes.** `K3A/SHA256SUMS` verifies all 48 files, and there are no unlisted files. `_run_records/source_sha256.txt` matches the candidate's blobs.
- **CHANGE_RECORD.** It follows `.agents/skills/chirality-change/SKILL.md`: what changed, no published value change, the bound and its status, tests, mutations, dependencies (none) and remaining limits. It keeps the maintained tests out of the dated run directory. See N7 for the revisions to add at PR time.
- **Suites.**
  - I re-ran `frame_kernel` only: 135 passed (127 unit, 2 site-table, 6 doc), against 116 at the base.
  - I read I2's per-crate logs for the dependent crates and did not re-run them. ROOT's DEC-025 sweep on `43da7a24e` covers them.

## 4. My arctangent bound and my reading of the proof

**Reading.** The proof is correct and complete for exact inputs, 53 ≤ p ≤ 128, and results that are not refused. Every step checks (§3.1). Its constants are deliberately coarse: it bounds the sensitivity of atan by 1 and each reduction by 4u.

**My bound** (`rv2_bound.py`). I keep every first-order coefficient exact:
- η₀ ≤ 2u, weighted by κ(T);
- the reduction from t: η ≤ (2 + w(t)(1.5 + 0.5t²/(1 + t²)))u, with w(t) = √(1 + t²)/(1 + √(1 + t²)), weighted by κ(t′);
- the series: σ ≤ (1 + 0.5t/atan t + 42·t²/(3(1 − t²))·t/atan t)u;
- here κ(t) = t/((1 + t²)atan t) ≤ 1 is the relative sensitivity of atan.

Maximising over θ₀ = atan T ∈ (0, π/2) on a grid of 400,000 points plus the reduction-step boundaries gives:
- **|φ̂ − φ| ≤ 16.48·2⁻ᵖ·φ** relative, with the maximum at φ ≈ 1.5987 and k = 5;
- **≤ 16.2 ulp_p(φ̂)**, with the maximum just below φ = 2;
- per reduction count k = 0…5: 3.53, 6.27, 9.00, 11.70, 14.30 and 16.48 u.

Second-order terms are below 10⁻¹³u at p ≥ 53. This is a first-order bound with rigorous per-step inequalities. It is not machine-checked.

**Measured, by me.** I used 26,000 independent vectors and 25,538 referenced results. The reference is Newton's method on sin θ − T cos θ at 260 digits, with π by Gauss–Legendre, sharing no code with I2's references.

| Case | Worst error |
|---|---|
| p = 128, 18,000 random angles (16,000 with 128-bit inputs, 2,000 binary64), plus 1,000 log-uniform toward 0 and 1,000 toward π | **6.0818 ulp** (S1) |
| p = 53 | 4.30 ulp |
| p = 64 | 4.53 ulp |
| p = 65 | 5.32 ulp |
| p = 127 | 5.00 ulp |
| `atan_positive` over 10^[−40, 40] at p = 128 | 4.36 ulp |

- At most 5 reductions were used.
- The p = 128 angle histogram is: < 1 ulp 11,957; [1, 2) 5,536; [2, 3) 1,681; [3, 4) 309; [4, 5) 48; [5, 6) 6; [6, 7) 1.
- My reference agrees with I2's hi + lo on all 3,293 committed vectors to 4.2·10⁻⁷⁸ relative, and reproduces I2's worst, 5.4111 ulp (`rand128:1181`).

**Summary of the bounds.** The proved 23.6 ulp holds. My tighter bound is 16.2 ulp. The measured worst is 6.08 ulp. The committed 6-ulp tolerance is not a bound (S1). All of these are about 10⁻³⁷ relative, far inside K-D5's first-order margin.

## 5. What I ran

All commands used `RUSTUP_TOOLCHAIN=1.97.1`, `RUSTUP_AUTO_INSTALL=0`, `CARGO_INCREMENTAL=0` and `--offline --locked`, with rustc and cargo 1.97.1, a per-copy target directory under `<rv2-target>`, and Python 3.11.15 from the DEC-025 venv (standard library only).
- **Host discipline.** Before each run I checked `pgrep -x cargo` and waited for the sweep (`run_evidence_sweep.py`). Free disk during my runs stayed between 9.6 and 13 GB.
- **Scratch copies.** The candidate and base copies were `git archive` extracts of `FK` plus the five sibling sources that `s11_site_table.rs` reads. The probe and mutant copies were copies of the candidate extract.

| Run | Result |
|---|---|
| `cargo test` (candidate `FK`, debug) | 127 + 2 + 6 passed, 0 failed, 0 ignored, no warnings |
| `cargo test --release --lib retained -- --nocapture` (candidate) | 22 passed; arctangent worst at p = 128 is 5.4111 ulp (`rand128:1181`) |
| `cargo build --lib` (candidate; doc tests are in the full run above, 6 passed) | no warnings |
| `cargo test` and `cargo build --lib` (base `4912dc636`) | 108 + 2 + 6 passed; no warnings |
| `gen_wide_vectors.py --check` | all 6 files OK (byte-identical) |
| `rv2_oracle.py` self-test, then 17,060 probe vectors | 0 mismatches against binary64 and against the candidate |
| I2's committed expectations against the RV2 oracle | 3,712 + 360 + 822, 0 mismatches |
| `rv2_atan.py` (26,000 vectors); I2's references against the RV2 reference | §4 |
| `rv2_bound.py` | §4 |
| `rv2_mutate.py` (11 mutants) | 10 killed, R2 survived (§3.6) |
| R2 through the probe on the 26,000 arctangent vectors | 13 results differ by 1 lsb (S2) |
| `diff <(git diff 4912dc636 a2e804a75) <(git diff 3488a236a 43da7a24e)` | empty |

The scripts, logs and outputs, with their hashes, are in `REVIEW/_run_records/k3a_review/`, which has its own `SHA256SUMS`. The two large probe outputs are not committed: they are regenerated from the seeded scripts, and their sha256 values are recorded there.

## 6. What I did not check

- **The fixture harness.** I did not re-run the 112-output fixture harness. I rely on my reasoning in §3.7 and on reading I2's records.
- **The dependent crates' suites.** I did not re-run them. ROOT's DEC-025 sweep on `43da7a24e` covers them, and I read I2's logs.
- **The merged head as a build.** I did not build or run `43da7a24e`. I checked only the diff identity and that the base change inside `frame_kernel` is `s11k_tests.rs` alone.
- **Machine checking.** My bound in §4 is a first-order analysis evaluated numerically, not a formal proof. I searched for worst cases by random sampling only, not adversarially.
- **Out of K3a's scope.** L = 4, 8 and 16, the rounded conversion back to binary64, K-D5's use of the arctangent, and the error in s and c (the proof assumes exact inputs).
- **Hosted CI.** I did not run it.
