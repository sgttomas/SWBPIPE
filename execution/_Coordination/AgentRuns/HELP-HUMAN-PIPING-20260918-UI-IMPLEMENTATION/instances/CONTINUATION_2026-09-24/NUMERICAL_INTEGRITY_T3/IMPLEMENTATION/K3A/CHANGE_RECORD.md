# K3a change record: `Wide<2>` arithmetic for the D-5 formation check

Draft PR record for slice K3a of T3 (numerical integrity), following `.agents/skills/chirality-change/SKILL.md`. Implemented by I2 (TASK).

- **Branch:** `codex/piping-k3a-20260926`, from the S11-K PR head `4912dc636` (PR #973, under review).
- **Revisions checked:**
  - base `4912dc6368be87636334162acae6e64ca7427ac2` (S11-K PR head at the cut);
  - candidate reviewed by RV2: `a2e804a757359d589f4c31ea8e36a923f28ccb8c`;
  - after S11-K merged (PR #973, `3488a236a`): `43da7a24e41dd3d46c60f324bcfbf883e1e26f22`. RV2 found K3a's diff identical across the two bases;
  - RV2's fixes are applied on `43da7a24e`, uncommitted. The manager records the final candidate, PR and merge revisions.
- **Fixture identity:** 112 of 112 outputs byte-identical (RETURN §5).
- **Landing:** K3a lands only after S11-K merges (ROOT selection C5). Before its own full-gate PR, the manager rebases or merges it onto a main that contains S11-K.
- **Basis:**
  - `ROOT_SELECTION_DESIGNS.md`, conditions C1 and C5;
  - D1 `DESIGN.md` revision 5a.2 (`fb62ef4a…`): §6 K3a row, §4.1.1, §4.11, §7.4, §4.3.1;
  - `R5_4_CURVED.md` (`2c9fae78…`);
  - V1's `REVIEW/VERIFY_R5.md` item 17 and `probe_atan_p128`.

## What was added

| File | Change | Lines |
|---|---|---|
| `P/core/solver/frame_kernel/src/structural.rs` | one line: `mod retained;` (MOD-D) | +1 |
| `…/frame_kernel/src/structural/retained/mod.rs` | new: the module, with `#![allow(dead_code)]` until K-D5 calls it | 14 |
| `…/frame_kernel/src/structural/retained/wide.rs` | new: `Wide<2>`, `WideArith`, `WorkCounter`, the split, the arctangent, and the proof in its documentation | 976 |
| `…/frame_kernel/tests/retained_wide/wide_tests.rs` | new: 19 unit tests, compiled as `wide.rs`'s `#[path]` test module | 1119 |
| `…/frame_kernel/tests/retained_wide/gen_wide_vectors.py` | new: standard-library generator (Fraction and decimal oracle) | 1102 |
| `…/tests/retained_wide/{targeted,split,atan,differential_sample,differential}.txt` and `SHA256SUMS` | new: generated vectors and their sha256 (checked by a test) | 8232 |

**What the code does:**
- **`Wide<2>`:** a sign, an `i64` exponent and a `[u64; 2]` significand.
- **Arithmetic:** `+ − × ÷ √`, each correctly rounded to nearest, ties to even, at a runtime precision 2 ≤ p ≤ 128.
- **Integers only:** bit-serial division and square root, so results are bitwise reproducible.
- **Exponent range:** exponents are checked against ±2^62. A result outside that range is refused and never wraps.
- **Conversion and split:**
  - an exact lift from `f64`, covering normal, subnormal and ±0; NaN and infinities are refused;
  - an exact split into at most three binary64 terms (bits 127…75, 74…22 and 21…0);
  - `add_product_to` feeds the split to `ExactAccumulator::add_product`.
- **Arctangent:** the included angle 2·atan(s/(1+c)) on 0 < φ < π, and `atan_positive`.
- **Work counter:** counts operations by kind.

**Split allowance:**
- The split is exact unless the value has set bits below 2^−1074.
- Those bits are dropped, and a flag reports it. The remainder is then nonzero, below 2^−1074, and has the value's sign.
- For K-D5 this means a miss of less than 2^−1074·|u_j| per coefficient, which is the per-row allowance of D1 §4.3.1.
- Values of 2^1024 or more are refused.

**Dependencies: none.** No Cargo.toml or lockfile change; `frame_kernel` stays dependency-free.

## No published value changes

Nothing in the product calls `retained` yet; K-D5 will be its first caller.
- The dependent suites pass unchanged (see RETURN §4).
- The fixture-identity run gives the result in RETURN §5.
- No fixture, schema, product source or committed output changed.

## The arctangent (C1): the proved bound is the contract; 6.1 ulp is a regression tolerance

- **Algorithm:** t = s/(1+c), then half-angle steps t ← t/(1+√(1+t²)) while t ≥ 1/20 (compared exactly), then the series, then ×2^(k+1). Every step is rounded to p.
- **Series stopping and summation:** the series stops before the first term below 2^(e_t−p−1). Its tail is summed smallest first, and t is added last. This summation order is an implementation choice below the design's resolution; it differs from V1's emulated forward summation.
- **Limits:** at most 5 reductions and at most 15 terms. Both are proved and both are enforced.
- **Proved bound, the accuracy contract K-D5 may cite:** at most 23.6 ulp of p for 53 ≤ p ≤ 128, for exact inputs. Otherwise the call is refused with `ExponentRange`, when t² or a series power leaves ±2^62; it never returns a wrong value. The proof is in the `wide.rs` module documentation. The generator's audit agrees with each step's bound at p = 128: initial error 1.78u against 2u, a reduction 2.39u against 4u, the series 1.16u against 1.54u.
- **Measured:** the worst error is 6.08 ulp at p = 128, over 3,307 referenced vectors, on RV2's input (φ ≈ 1.6473). The worst on I2's original set was 5.41 ulp. The same figures come from the Python emulation and from the Rust test.
  - V1's forward-summation variant reaches 7.28 ulp on the same set, so V1's 2.69 ulp over 13 angles does not hold beyond those angles.
- **Regression tolerance, not a bound:** 6.1 ulp, for the committed vectors only, raised just enough to cover RV2's input. Other inputs may exceed it. The proved 23.6 ulp is also asserted on every vector.
- **Summation order pinned:** 13 inputs that separate smallest-first from largest-first tail summation (RV2's R2) are in the vectors.

## Tests and results

Plan: V1-S9, scoped to L = 2. The frame_kernel suite gives 135 passed (127 unit, of which 19 are new; 2 site-table; 6 doc). The base had 116.

**At p = 53:**
- bitwise equality with hardware binary64 for + − × ÷ √, over 10^5 random normal-range operand pairs per operation (plus explicit ties).

**Targeted hard classes (2,072 vectors, at p ∈ {53, 63, 64, 65, 127, 128}):**
- exact ties to even at the limb boundaries;
- carry-out and renormalization;
- cancellation within one ulp and within 2^−p′;
- exact and near-exact division and square root;
- sticky-bit paths;
- TwoSum and TwoProduct (Dekker) at p ∈ {53, 64, 128}.

**Further unit tests:**
- exponent extremes refused;
- signs of zero;
- the precision range;
- the work counter.

**Fraction differential (regenerated in the test, not committed):**
- 10^6 operations at p = 128, seed `4b33415f57494445`, stream sha256 `b568d2c0…31253a2`;
- 2·10^5 operations at mixed p ∈ [2, 128], seed `4b33415f4d495844`, sha256 `f8b008bc…a443603`.
- The Rust test regenerates the operands with the same SplitMix64 rules and compares:
  - the first 1,000 records of each stream against committed vectors;
  - each chunk of 10^5 records by sha256;
  - the whole-stream sha256.

**The split:**
- 822 committed vectors: overflow, the subnormal boundary and truncation cases;
- 5·10^4 random and adversarial values;
- 2·10^4 `add_product` round trips against the correctly rounded product.

**The arctangent:**
- 3,319 vectors (3,307 referenced, 12 refusals), including RV2's S1 input and its 13 R2-distinguishing inputs: V1's 13 angles; decades toward 0 down to 1e−300; decades toward π down to π−1e−19 (128-bit inputs); π/2 and other landmarks; 1,800 random angles; inconsistent (s, c) pairs; t from 1e−300 to 1e300 and 2^±100000; and the 1/20 threshold and the reduction-step boundaries.
- The test checks bitwise agreement with the emulation, the error against a 160-digit reference (computed two independent ways) within the 6.1-ulp regression tolerance and within the proved bound, and the reduction and term limits through the work counter.

**Dependent crates:** RETURN §4.

## Mutation table

Command: `cargo test --offline --locked --lib retained -- --test-threads=4`, run per mutant with `_run_records/mutations/mutate.py.txt`. Patches and logs are in `_run_records/mutations/`.

| Mutant | Patch (in `wide.rs`) | Result | Killing tests (examples) |
|---|---|---|---|
| M1 round toward zero | `if false && round && (…)` in `round_pack` | killed | targeted, differential p128 and mixed, p53 hardware, TwoSum/TwoProduct, split→accumulator, arctangent |
| M2 dropped sticky bit | `let rest = m.any_below(drop - 1);` | killed | targeted, both differentials, arctangent, 1/20 threshold |
| M3 ties away from zero | `if round {` | killed | targeted, both differentials, p53 hardware, split→accumulator, arctangent |
| M4 off-by-one limb shift | cross-limb `self.hi << (127 - n)` in `shr_sticky` | killed | targeted, both differentials, split vectors, split random, arctangent |
| M4b alignment shift off by one | `.shr_sticky(gap + 1)` | killed | 13 tests |
| M5 arctangent, one reduction fewer | at most 4 reductions, then the series | killed | arctangent vectors (the series limit is reached; bitwise mismatch) |
| R1 (RV2) series stop threshold off by one | `threshold = e_t − p` | killed | arctangent vectors (bitwise); re-run after the RV2 fixes |
| R2 (RV2) tail summed largest first | `terms[1..count].iter()` | killed after the RV2 fixes (it survived before) | arctangent vectors (bitwise, first at `rv2r2:0`) |
| M6 arctangent, truncated series | 8 terms | killed | arctangent vectors (bitwise). Diagnostic run with the bitwise assertion removed: 2.09·10^8 ulp, caught by the reference check alone |

No survivors.

## Remaining limits

- K3a is unreachable from the product until K-D5.
- `#![allow(dead_code)]` is removed by K-D5.
- Rounded conversion to binary64 (normal, subnormal, underflow and overflow outcomes), L = 4, 8 and 16, and the differential at those precisions are slice K3.
- The arctangent's proved bound assumes exact inputs. K-D5's s and c carry their own formation error, which K-D5's check measures.
