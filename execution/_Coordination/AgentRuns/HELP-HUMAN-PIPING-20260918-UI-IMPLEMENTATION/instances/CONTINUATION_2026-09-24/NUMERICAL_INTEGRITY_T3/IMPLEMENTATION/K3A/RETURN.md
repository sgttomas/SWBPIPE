# I2 return: slice K3a (`Wide<2>` arithmetic), implemented

**Status: complete.**
- Every K3a-row item and condition C1 is met.
- All suites pass, including src-tauri.
- The fixture-identity run is byte-identical: 112 of 112 outputs.
- All 7 seeded mutants are killed.
- No committed byte outside the write set changed.
- Nothing is committed: I made no Git writes, and the manager commits.

**Where:**
- Worktree `<k3a-worktree>`, branch `codex/piping-k3a-20260926`, base S11-K head `4912dc636` (PR #973).
- `P/` = `projects/chirality-piping/`. `FK/` = `P/core/solver/frame_kernel/`.

**Read:**
- Root `AGENTS.md`, `agents/AGENT_TASK.md`, `TASK_BRIEFS/_COMMON.md` and `I2_K3A_IMPLEMENTATION.md`, all from the T3 worktree at `3b2fbe022`.
- `ROOT_SELECTION_DESIGNS.md`.
- D1 `DESIGN.md` at sha256 `fb62ef4a…` (verified): revision 5a table, §4.1.1, §4.1.2, §4.3.1, §4.11, §6, §7.4.
- `R5_4_CURVED.md` at `2c9fae78…` (verified).
- `REVIEW/VERIFY_R5.md` item 17 and `probe_atan_p128.{py,stdout}.txt`.
- `FK/exact_sum.rs` at the base, and S11-K's `RETURN.md` and fixture-diff harness.

## 1. Files changed

| File | Kind | Lines |
|---|---|---|
| `FK/src/structural.rs` | edit: the single `mod retained;` (MOD-D). No re-export was needed | +1 |
| `FK/src/structural/retained/mod.rs` | new | 14 |
| `FK/src/structural/retained/wide.rs` | new | 966 |
| `FK/tests/retained_wide/wide_tests.rs` | new: `wide.rs`'s `#[cfg(test)] #[path]` module (22 tests) | 1115 |
| `FK/tests/retained_wide/gen_wide_vectors.py` | new: standard-library generator | 1068 |
| `FK/tests/retained_wide/targeted.txt` | new vectors | 2072 |
| `FK/tests/retained_wide/split.txt` | new vectors | 822 |
| `FK/tests/retained_wide/atan.txt` | new vectors | 3305 |
| `FK/tests/retained_wide/differential_sample.txt` | new: the first 1,000 records of each stream | 2000 |
| `FK/tests/retained_wide/differential.txt` | new: seeds, counts, stream and chunk sha256 | 14 |
| `FK/tests/retained_wide/SHA256SUMS` | new: sha256 of the five files above, verified by a test | 5 |

**Notes on the layout:**
- The source hashes are in `_run_records/source_sha256.txt`.
- **Why the tests use a `#[path]` module.** `retained` is crate-private (MOD-D and the brief allow no public surface), so an integration test under `tests/` cannot reach it. The test source therefore lives under `FK/tests/retained_wide/`, as the write set requires, and is compiled as `wide.rs`'s unit-test module. It runs in `frame_kernel`'s own `cargo test`, and so in hosted CI (D1 §7.4). `tests/retained_wide/` has no `main.rs`, so cargo does not treat it as a separate test target.
- **`#![allow(dead_code)]`** in `retained/mod.rs` covers the time until K-D5, the first caller. K-D5 removes it.
- **Unchanged:** no Cargo.toml, lockfile, product, fixture, schema or other file. `exact_sum.rs` and `ExactAccumulator` are unmodified. `frame_kernel` stays dependency-free.

## 2. How each K3a-row item and C1 was met

| Item | Implementation (`wide.rs`) | Evidence |
|---|---|---|
| `Wide<2>`: sign, `i64` exponent, `[u64; 2]` significand | `Wide<const L: usize>`; only `Wide<2>` is implemented. The value is (−1)^s·m·2^(e−127) with m normalized. Zero is signed | construction and lift tests |
| + − × ÷ √, rounded to nearest with ties to even, at runtime p ≤ 128 | `WideArith::new(p)`, 2 ≤ p ≤ 128. The exact result of the exact operands is rounded once, through one rounding routine (`round_pack`). Addition uses a 256-bit window, with an exact jam-free sticky for subtraction. Multiplication is a full 128×128 product. Division and square root are bit-serial with the remainder as sticky | p = 53 against hardware; targeted classes; the differentials |
| Integer only, bitwise reproducible | `u64`/`u128` operations only, with no floating point in the arithmetic | — |
| Exponent extremes refused, never wrapped | exponent arithmetic is done in `i128`, and every result is checked against ±2^62 (`ExponentRange`); `from_parts` validates | `exponent_extremes_are_refused_never_wrapped` |
| Exact lift from f64 | normal, subnormal and ±0; NaN and ±∞ give `NonFinite` | `lift_from_f64_…` (20,000 round trips, bitwise) |
| Exact split into at most three binary64 terms, with the sub-2^−1074 allowance | bits 127…75, 74…22 and 21…0, each an exact binary64 with the value's sign, zero pieces omitted. **Exact unless the value has set bits below 2^−1074.** Those are dropped (truncation toward zero) and flagged; the remainder is then nonzero, below 2^−1074 in magnitude and has the value's sign, so K-D5's miss per coefficient is below 2^−1074·\|u_j\| (D1 §4.3.1). A value ≥ 2^1024 is refused. `add_product_to` feeds `ExactAccumulator::add_product` | split vectors (822), 5·10^4 random and adversarial values, 2·10^4 `add_product` round trips |
| Arctangent, open interval 0 < φ < π (C1) | `included_angle(s, c)` = 2·atan(s/(1+c)), and `atan_positive(t)`. Half-angle steps while t ≥ 1/20 (decided exactly), at most 5 reductions and at most 15 terms (both proved, both enforced), the series summed smallest first. Restricted to 53 ≤ p ≤ 128, the range of the proof | §3 |
| Work counter | `WorkCounter`: add, sub, mul, div, sqrt and atan counts, saturating. It includes the arctangent's internal operations | `work_counter_…`; also used to assert the reduction and term limits on every arctangent vector |

**Differences from V1's emulation.** In the arctangent, the series tail is summed smallest first, and the stop test is against 2^(e_t−p−1), where V1 summed forward and stopped at ulp(acc)/4. This is an implementation detail below the design's resolution ("sum the series"). It gives a smaller proved bound and a smaller measured error. I report it here rather than treat it as a design change.

## 3. The arctangent (C1): proved bound and measured tolerance

**Proved** (full proof in the `wide.rs` module documentation):
- For exact inputs, |φ̂ − φ| ≤ 23.55·2^−p·φ, that is **at most 23.6 ulp of p**, for every input in the domain and 53 ≤ p ≤ 128.
- For `atan_positive` the bound is (1.54 + 4k)·u.
- **Structure of the proof:**
  1. Lemma A: a relative perturbation η of t moves atan t by at most \|η\|/(1−\|η\|)² relative.
  2. The initial quotient has error γ₂.
  3. Each reduction has error at most 4u/(1−u)³.
  4. The series (term errors, a smallest-first summation of at most 14 terms, the stop-rule tail and the final rounding) has error at most 1.54u.
  5. Scaling by 2^(k+1) is exact.
  6. Five reductions and 15 terms always suffice: tan(π/64·(1+23u)) < 0.0492 < 1/20, and 0.05³⁰/31 < 2^−130.
- **Audit** (generator, p = 128, all referenced inputs): the worst measured per-step errors are 1.78u (bound 2u), 2.39u (bound 4u) and 1.16u (bound 1.54u). The most reductions observed is 5, and the most series terms 15.

**Measured:**
- Against a 160-digit reference, computed two independent ways (half-angle reduction with Taylor, and Euler's series with Machin's π) that must agree to 1e−140.
- **Worst error at p = 128: 5.41 ulp** (vector `rand128:1181`).
- Other precisions: p = 53: 3.42; p = 64: 2.67; p = 100: 3.31; p = 127: 2.81.
- The Rust test and the Python emulation report the same figure (5.4111).
- **Distribution at p = 128:** ≤ 1 ulp: 2057; ≤ 2: 631; ≤ 2.69: 162; ≤ 4: 196; ≤ 6: 7; > 6: 0.
- **V1's variant** (forward summation) reaches **7.28 ulp** on the same set. So V1's ≤ 2.69 ulp over 13 angles does not hold beyond those angles. That is expected with wider sampling.

**Angle set** (3,305 vectors: 3,293 referenced and 12 refusals):
- V1's 13 angles, binary64 inputs;
- φ = 10^−k for k = 1…300 (binary64 inputs) and k = 1…40 (128-bit inputs);
- 200 log-uniform small angles, 10^−300 to 1;
- π − 10^−k for k = 1…10 (binary64; k ≥ 8 refused, since c rounds to −1) and k = 1…22 (128-bit; k ≥ 20 refused);
- 200 log-uniform near-π angles;
- (s, c) = (2^j, −1 + 2^−128) for φ within 2^−5000 of π;
- π/2 and other landmarks (π/3, 2π/3, π/4, 3π/4, π/6, 5π/6, π/64, 63π/64, 31π/32; both input widths);
- 1,500 uniform random angles (128-bit inputs) and 300 (binary64 inputs);
- 200 inconsistent (s, c) pairs;
- 240 vectors at p ∈ {53, 64, 100, 127};
- `atan_positive` over t = 10^k for k = −300…300 (step 3), 2^j for j = −1100…1100 (step 100) and ±100000, the 1/20 threshold ± 3 ulp, and inputs at each reduction-step boundary ± 2 ulp;
- domain refusals.

**Specified tolerance:**
- The tests assert **6 ulp**: the measured bound rounded up. It covers 5.41 and is ≤ 23.6.
- They also assert the proved 23.6 ulp on every vector, and bitwise equality with the generator's emulation of the algorithm.
- ROOT accepted the proof as meeting C1 (manager relay, 2026-09-26).

## 4. Tests: results and per-crate counts

**K3a's 22 tests** (`FK`, in `retained::wide::tests`):
- sha256 known answers; the committed vectors' sha256.
- lift; `from_parts`; precision range; signs of zero; work counter.
- p = 53 against hardware for + − × ÷ √ (10^5 random normal-range pairs per operation, plus ties).
- Targeted hard classes (2,072 vectors):
  - exact ties at p = 53, 63, 64, 65, 127 and 128, for add, sub and mul, and for div and sqrt where a tie is representable (div to p = 127; sqrt to p = 63);
  - carry-out and renormalization;
  - cancellation within one ulp and within 2^−p′ (p′ = 53…128, including across a binade);
  - exact and near-exact division; perfect squares and their neighbours at ±1 ulp of 128 bits and of p;
  - sticky paths (alignment beyond 128 bits, a lone low product bit, a constructed division remainder, x² + 1);
  - TwoSum and Dekker TwoProduct at p ∈ {53, 64, 128}.
- TwoSum and TwoProduct exactness on binary64 inputs (independent of the vectors).
- Exponent extremes; the 1/20 threshold.
- Both differentials.
- The split: vectors, random values, and the `add_product` round trip.
- The arctangent: vectors and domain.

Result: **22 passed** (11.9 s single-threaded, 7.5 s parallel, debug).

**Seeded Fraction differential:**
- **Choice:** the full streams are not committed (about 26 MB). Committed instead: the generator, the seeds, the stream and per-10^5-chunk sha256 digests (`differential.txt`) and the first 1,000 records of each stream (`differential_sample.txt`). The Rust test regenerates the operands with the same SplitMix64 rules, checks the 1,000 sample records one by one (operands and result), then each chunk digest, then the stream digest.
- **p = 128:** seed `4b33415f57494445`, **1,000,000 operations** across + − × ÷ √ (each about 1/5), stream sha256 `b568d2c0de2cbcb8c50a1064e0dff856721f80ec021a1b360f22ac66a31253a2`.
- **Mixed p ∈ [2, 128]:** seed `4b33415f4d495844`, 200,000 operations, sha256 `f8b008bc06b4688c4b9f9af6c29a853e4f7013be2436aa5f8c02cbd2da443603`.
- **Oracle:** `fractions.Fraction` exact results, rounded once. The square root uses `math.isqrt` with an exactness test.
- **Operand rules:** 128-bit significands with short, all-ones and lone-low-bit patterns; exponents in ±2000; gaps up to ±1500; near-equal and straddling-binade operands; signed zeros.
- `gen_wide_vectors.py --check` regenerates every committed file byte-identically (`_run_records/generator/`).

**Per-crate suites** (`cargo test --offline --locked`, toolchain 1.97.1, `_run_records/suites/`):

| Crate | Passed | Failed | Ignored |
|---|---|---|---|
| core/solver/frame_kernel | 135 (127 unit, of which 22 are new; 2 site table; 6 doc; base 113) | 0 | 0 |
| core/solver/straight_pipe | 39 | 0 | 0 |
| core/solver/curved_bend | 25 | 0 | 0 |
| core/loads/load_case_algebra | 21 | 0 | 0 |
| core/solver/sparse_direct | 25 | 0 | 0 |
| core/solver/nonlinear_integration | 69 | 0 | 0 |
| core/loads/primitive_loads | 49 | 0 | 0 |
| core/solver/linear_supports | 15 | 0 | 0 |
| core/solver/nonlinear_supports | 22 | 0 | 0 |
| core/solver/diagnostics | 24 | 0 | 0 |
| core/solver/performance_harness | 25 | 0 | 0 |
| core/loads/stress_recovery | 48 | 0 | 0 |
| core/loads/user_loads | 28 | 0 | 0 |
| core/loads/self_weight_wasm | 14 | 0 | 0 |
| core/product_physics | 448 | 0 | 1 (pre-existing) |
| core/model_operations/operation_applier | 194 | 0 | 0 |
| core/runner/headless (`--no-fail-fast`) | 83 | 0 | 0 |
| core/reporting/result_export | 91 | 0 | 0 |
| validation/benchmarks/mechanics | 41 | 0 | 0 |
| validation/benchmarks/nonlinear | 19 | 0 | 0 |
| validation/benchmarks/stress | 23 | 0 | 0 |
| validation/benchmarks/physics_audit_regression | 15 | 0 | 0 |
| validation/benchmarks/numerical_integrity | 0 (no tests; builds) | 0 | 0 |
| apps/desktop/src-tauri | 114 | 0 | 0 |

The counts outside `frame_kernel` equal S11-K's post-regeneration run.

## 5. Fixture identity

- **Method:** S11-K's harness (`fixdiff_main.rs.txt`, unchanged; sha256 in `_run_records/fixture_diff/harness_source.txt`) ran every committed JSON request or model under `P/fixtures`, `P/validation` and `P/core` through `run_linear_static_preview_value_with_mode`, in both modes.
- **Sides:** the base is a read-only `git archive` of `4912dc636`; the candidate is the K3a worktree. The base and candidate `core` trees differ only in the K3a files.
- **Result: 112 of 112 outputs byte-identical** (fixtures 72, validation 30, core 10). This includes 6 outputs that are `ERR`/`PANIC` identically on both sides (inputs that are not runnable requests).
- Per-output sha256: `_run_records/fixture_diff/fixture_identity.json`.
- **Zero committed-byte or fixture changes.**

## 6. Mutation table

| Mutant | Patch | Result | Killing tests |
|---|---|---|---|
| M1 round toward zero | `if false && round && (rest \|\| keep & 1 == 1)` | killed | 9, including targeted, both differentials, p53 hardware |
| M2 dropped sticky bit | `let rest = m.any_below(drop - 1);` | killed | targeted, both differentials, arctangent, 1/20 threshold |
| M3 ties away from zero | `if round {` | killed | targeted, both differentials, p53 hardware, split→accumulator, arctangent |
| M4 off-by-one limb shift | `shr_sticky` cross-limb `self.hi << (127 - n)` | killed | targeted, both differentials, split vectors and random, arctangent |
| M4b alignment shift off by one | `.shr_sticky(gap + 1)` | killed | 13 tests |
| M5 arctangent, one reduction fewer | at most 4 reductions, then the series | killed | arctangent vectors (the series hits its limit; bitwise) |
| M6 arctangent, truncated series | 8 terms | killed | arctangent vectors (bitwise). With the bitwise assertion removed for one diagnostic run (restored): 2.09·10^8 ulp, caught by the reference check alone |

- **Command:** `cargo test --offline --locked --lib retained -- --test-threads=4`.
- **Records:** patches, logs, the driver and the JSON are in `_run_records/mutations/`.
- `wide.rs` was restored after the run and verified by sha256.
- No survivors.

## 7. Toolchain and host

- **Toolchain:** rustc and cargo 1.97.1, `CARGO_INCREMENTAL=0`, own target `<k3a-target>`, `--offline --locked` for every repository crate. Python 3.11.15 (DEC-025 venv), standard library only.
- **Formatting:** rustfmt 1.8.0 from the host's stable toolchain, applied to the K3a files only (1.97.1 has no rustfmt component). Pre-existing format differences in `lib.rs` and `functionals.rs` were left alone.
- **Whitespace:** `git diff --no-index --check` is clean on every new file.
- **Host incident (undone):** one `RUSTUP_TOOLCHAIN=1.94.1` call made rustup auto-install a `1.94.1` toolchain (570 MB). I uninstalled it at once. From then on I used `RUSTUP_AUTO_INSTALL=0`.
- **Cargo discipline:** one cargo job at a time, checked with `pgrep`, with no waits needed. Heavy runs were held until the manager released the host.
- **Disk:** free disk, measured after each heavy run with its build output still present, was at least 9.1 GB (product_physics) and 9.6 GB (src-tauri, 3.6 GB of build output), and was 13 GB at the end. My target was pruned after each crate and is empty at the end.

## 8. Not done, and open items

- **No Git writes.** The manager commits, and K3a lands after S11-K merges (C5), rebased or merged by the manager.
- **Slice K3's scope, not built here:** rounded conversion back to binary64 with its outcomes; L = 4, 8 and 16.
- **Proof scope:** the proved bound is for exact inputs. K-D5's s and c carry formation error, which its own check measures.
- **For D1's record:** V1's 2.69-ulp figure is superseded by the proof (23.6) and by this set's measured 5.41. V1's variant reaches 7.28 on this set.
