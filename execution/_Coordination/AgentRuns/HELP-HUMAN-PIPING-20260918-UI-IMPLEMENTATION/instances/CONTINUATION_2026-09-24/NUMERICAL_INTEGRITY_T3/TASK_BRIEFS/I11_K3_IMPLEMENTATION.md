# I11: implement slice K3 (the rest of W1's arithmetic: `Wide<L>` for L = 4, 8, 16, and the binary64 conversion)

This is an implementation TASK. Read Root `AGENTS.md`, `agents/AGENT_TASK.md` and `_COMMON.md` first. The Mac host rules in `I8R_K1_RESUME.md` ("The Mac host") override `_COMMON.md`'s host section and apply to you in full, using K3's paths below in place of K1's.

## Roles

- ROOT (HELP_HUMAN) dispatches you directly, as a background subagent, and is your return path. There is no separate T3 manager on the Mac.
- Make no Git writes and no index operations. ROOT commits.
- Record the delegation mechanism in RETURN.

## Purpose

K3 is the §6 row "K3: the rest of W1's arithmetic": `retained/wide.rs` "(L = 4, 8, 16; runtime p; conversion outcomes to f64: normal, subnormal, underflow, overflow)", tested by "§4.11 classes, the differential and the mutants". It runs "in parallel with K-D5 and K1, after K3a". K-D5 and K1 have merged, so K3 now runs beside K2b. K4 (W1a) waits on K1 and K3.

**What K3a built** (PR #983, merged at `3e861f53c`; records `IMPLEMENTATION/K3A/`, `K3A_MERGE/` and `REVIEW/K3A_REVIEW.md`):
- The type `Wide<const L: usize>`. Only `impl Wide<2>` exists, and every helper (`U256`, `round_pack`, `divide_significands`, `sqrt_significand`) is specific to 128 bits.
- `WideArith`: + − × ÷ √, each correctly rounded to nearest, ties to even, at 2 ≤ p ≤ 128. It uses integers only. An exponent outside ±2^62 is refused, never wrapped.
- An exact lift from f64, covering normal, subnormal and ±0. NaN and ±∞ are refused.
- The exact split into at most three binary64 terms, and `add_product_to`, for K-D5.
- The included-angle arctangent, for 53 ≤ p ≤ 128 only. The proved contract is 23.6 ulp; 6.1 ulp is a regression tolerance for the committed vectors.
- `WorkCounter`: saturating counts by operation kind.
- The tests: 19 of them; the targeted, split and arctangent vectors; and two seeded differentials, at p = 128 and at mixed p ≤ 128. The generator, seeds and digests are committed, but the streams are not. Nine mutants were killed.

K3a's own records name what it left for K3: "Rounded conversion to binary64 (normal, subnormal, underflow and overflow outcomes), L = 4, 8 and 16, and the differential at those precisions are slice K3" (CHANGE_RECORD, "Remaining limits"; RETURN §8).

**What K3 adds** (`DESIGN.md` §4.1.1, the `wide.rs` bullet; §4.11):
1. **The widths.** "`Wide<const L: usize>` … with L = 2, 4, 8, 16 for 128, 256, 512 and 1024 bits." K3 adds L = 4, 8 and 16.
2. **The operations.** "`+ − × ÷ √`, each rounded to nearest, ties to even, at a runtime precision p ≤ 64L."
   - W1 uses these precisions: the candidates p = 128, 256 and 512, each "verified at 2p", with a ceiling of 1024 (§4.1.6);
   - and the refinement residual "re-formed at p + 64" (§4.1.4), so 192, 320 and 576.
3. **The lift.** "Exact conversion from `f64`", at every width.
4. **The conversion to binary64.** "Correctly rounded conversion to `f64`, with an explicit outcome: normal, subnormal (with its relative precision bound), underflow to zero, or overflow" (§4.1.1; §5 item 7).
   - It covers every width, **including L = 2**. K3a has none.
5. **The work counter** at every width (§4.1.1). §4.1.7 counts budgets "in limb-multiply equivalents per attempt" (Q9).
6. **Integer-only arithmetic,** "so results are bitwise reproducible across platforms" (§4.1.1, §4.1.8).
7. **Subject to Q3 and Q4:** the small pieces of arithmetic K4 needs and cannot add itself. K4 writes "new files under `FK/structural/retained/` only" (§6, K4 row), so it cannot edit `wide.rs`. The pieces are:
   - exact widening, and correctly rounded narrowing, between widths;
   - TwoSum and TwoProduct at every width (§4.1.1 `RetainedCombination`; §4.1.2 item 4; a §4.11 class);
   - a correctly rounded constructor from an exact integer times 2^e, for §4.1.2 item 5's ledger projection.

**What K3 does not do:**
- no method code (K4's);
- no caller outside `retained`;
- no change to any existing `Wide<2>` result, to `Wide<2>`'s `Debug` format, or to the `Display` text of any existing `WideError` variant;
- no arctangent or split beyond L = 2 (Q6);
- no dependency and no lockfile change (§4.11: in-repo, "No lockfile changes").

## ROOT rulings for this slice (2026-09-28)

A TASK drafted this brief. ROOT reviewed it and ruled on its open questions as follows. The questions and their options remain at the end, for the record.

1. **Q1: add beside `Wide<2>`, option (a).**
   - K3a's `impl Wide<2>` code paths stay byte for byte.
   - The new widths are built beside them. The new core is instantiated at L = 2 **in tests only**, and cross-checked bitwise against K3a's path over K3a's vectors and differentials.
   - `Wide<2>` is a published-byte surface through K-D5 (D-5 evidence line EF values; `WideError` `Display`). **Any change to a `Wide<2>` result, `Debug` token or existing `Display` string stops the work.**
2. **Q2: no published byte changes, by construction.**
   - The evidence is T9 at 112 of 112 (Mac-only), K-D5's suites unchanged, and the new `Display` pin.
   - **The both-entry gate is not run.** K3 adds no product caller.
3. **Q3: all of K4's arithmetic is included.** That is: exact widening; correctly rounded narrowing; TwoSum and TwoProduct at every width; a correctly rounded constructor from an exact integer times 2^e; and per-width work counts. Each is tested against `Fraction`.
4. **Q4: the recommended split.**
   - K3 supplies the integer constructor in `wide.rs`.
   - **K4** adds the one-function `pub(crate)` read accessor to `exact_sum.rs`, as a declared write-set extension in K4's brief.
   - K3 does not touch `exact_sum.rs`.
5. **Q5: the same convention as K2b's publication outcomes, in K3's own type.** The type sits in `wide.rs`, since K3 cannot import from K2b's branch. The convention:
   - an exact ±0 is normal and keeps its sign;
   - a subnormal carries 2^-1075/|result|, rounded upward;
   - underflow means only a nonzero value rounding to ±0, and overflow a value rounding to ±∞;
   - neither ever returns a value silently.

   Unifying K2b's and K3's types is deferred to K4, F1b or F2a, whichever consumes both first.
6. **Q6: the arctangent and the split stay at L = 2.**
7. **Q7: measure first.**
   - Record the debug wall times for D at checkpoint A. ROOT rules then on keeping 10^6 per precision in the default suite, or adding a `[profile.test]` opt-level to FK's `Cargo.toml`.
   - Do not move the full streams into `#[ignore]` tests.
   - Until ROOT rules, `Cargo.toml` stays unchanged.
8. **Q8:** this is for K4's brief. K3 builds L = 4, 8 and 16 only.
9. **Q9: the recommendation is adopted.**
   - K3a's counts are unchanged at L = 2.
   - Add per-width counts, and a counter mergeable across widths within one attempt.
   - Add a stated, deterministic limb-multiply-equivalent cost per operation kind and width, set out at checkpoint 0.
   - The limits stay ROOT's, set from measurement.
10. **The write set's "ROOT to rule" rows:**
    - **`retained/mod.rs`:** documentation, plus at most the module declaration lines a child module needs, declared in CHANGE_RECORD.
    - **A child module of `wide.rs`** (for example `retained/wide/<name>.rs`) is allowed if `wide.rs` would otherwise pass about 2,500 lines, again declared.
    - **`exact_sum.rs`:** not K3's (Q4).
    - **`FK/Cargo.toml`:** not until Q7 is ruled.

## Scope: kernel only

**K3's new surface has no product caller.** `retained` is private to `FK/structural.rs` (`mod retained;` at `:5`, MOD-D, landed by K3a), and its items are `pub(crate)`. K4 will be the first caller.

**`wide.rs` itself is on the product path, through K-D5.** This was not the case when the K3 row was written. The call chain is:
1. `PP:4441` (`solve_preview_reduced_system`) calls `solve_assembled_with_formation_check`;
2. that calls `SA:174` (dense) or `SA:469` (pattern);
3. which reaches `formation_check::check` (`FK/structural.rs:459`, `sparse.rs:993`);
4. which uses `Wide2`, `WideArith` and `WideError` (`formation_check.rs:32`, `:229`).

This path runs on every Passed linear case. It carries two published surfaces:
- **the EF values** in the D-5 evidence line;
- **`WideError`'s `Display` text,** which is published as `detail=retained arithmetic: {e}` (`formation_check.rs:145` → `PP:1123-1124`).

So **the existing `Wide<2>` surface is a published-byte surface.** Its results, its `Debug` format and every existing `WideError` `Display` string must stay byte-identical. "K3 changes no published byte" holds only under that condition (Q1, Q2).

**Write set** (re-locate every line on your base):

| File | Change | Status |
|---|---|---|
| `FK/src/structural/retained/wide.rs` | the widths, the operations, the conversion, the work counter, and the Q3 items | in the K3 row |
| `FK/src/structural/retained/mod.rs` | module documentation only (it now reads "Slice K3a adds only …") | comments: certain. **Any code line: ROOT to rule** |
| new `FK/tests/retained_wide_k3/` | the test module (compiled as a `#[cfg(test)] #[path]` module of `wide.rs`, as K3a's is); a standard-library generator with `--check`; the vectors; and its own `SHA256SUMS` | in the row (tests). A new directory is needed because K3a's `committed_vectors_match_their_recorded_sha256` panics on any file not in its list |
| a child module of `wide.rs` (for example `retained/wide/<name>.rs`), declared from `wide.rs`, if `wide.rs` grows past a reviewable size | a code split only | **ROOT to rule** |
| `FK/src/exact_sum.rs` | one additive `pub(crate)` read accessor, only if ROOT gives the ledger projection to K3 | **ROOT to rule** (Q4; recommended: not K3) |
| `FK/Cargo.toml` | a `[profile.test]` entry, only if Q7 needs one | **ROOT to rule** (recommended: no) |
| `T3/IMPLEMENTATION/K3/**` in `<wt>/k3` | records | certain |

**Not in scope. Stop and ask before touching any of these:**
- `formation_check.rs` and its tests (K-D5's live code);
- `FK/structural.rs` (the `mod retained;` line already exists);
- `FK/lib.rs`, `sparse.rs`, `load_ledger.rs`, `SA` and `PP` (K2b's and the facade's);
- K3a's `FK/tests/retained_wide/**`, which stays byte-identical, including its generator, vectors and `SHA256SUMS`;
- `s11_site_table.rs` and `s11f_site_test.rs`. `wide.rs` is in neither table's source list, and K3 adds no load, force or right-hand-side accumulation. If one seems needed, stop;
- dependencies and lockfiles;
- the committed fixtures.

**Constraints on the implementation:**
- **No heap allocation per operation.** §4.11 gives this as a property of the in-repo choice: "Stack-allocated limbs mean no allocation per operation".
  - Rust 1.97.1 has no `generic_const_exprs`. So the 2L-limb intermediates (full products, and division and square-root remainders) need a fixed-size stack buffer or per-width code.
  - If this cannot hold, say so at checkpoint 0.
- **The `WideError` enum.** New variants may be added. No existing variant, or its `Display` text, may change.
- **`dead_code`.** Follow K-D5's precedent (`IMPLEMENTATION/KD5/RETURN.md` item 8, reviewed in `KD5_REVIEW.md`): a per-item `#[allow(dead_code)] // <reason>` on each item with no non-test caller, and a warning-free non-test FK build.
  - Where K3 gives one of K-D5's "K3 API" items a caller, remove its allowance. Otherwise relabel the reason "K4 API".
  - No module-wide or `cfg_attr` allowance.

## Basis (read in this order)

1. Root `AGENTS.md`, `agents/AGENT_TASK.md`, `_COMMON.md`, and `I8R_K1_RESUME.md` "The Mac host".
2. `T3/DESIGN_NUMERICS/DESIGN.md` revision 5a.2 (sha256 `fb62ef4a…`; hash-pinned, don't edit):
   - §4.1.1, the `wide.rs` bullet and `RetainedCombination`;
   - §4.1.2, items 4–6 and the exact-sum rule;
   - §4.1.4, the p + 64 residual;
   - §4.1.6, the schedule;
   - §4.1.7, the work units;
   - §4.1.8, determinism;
   - §4.11, the in-repo recommendation and the V1-S9 test plan;
   - §5 item 7, representability;
   - §6: the K3a, K3 and K4 rows, and the order;
   - §7.3, mutations 5, 6 and 13–16;
   - §7.4.
3. `T3/ROOT_SELECTION_DESIGNS.md`: the order, and C1.
4. `T3/ROOT_RULINGS_V1.md`:
   - "D5_CHECK" item 3 (why K3 was split);
   - "K3a arctangent and host disk";
   - "K2b: rulings on I10's checkpoint-0 plan" item 3 (the publication outcomes, for consistency with Q5).
5. **K3a's records:**
   - `TASK_BRIEFS/I2_K3A_IMPLEMENTATION.md` and `I2_K3A_RV2_FIXES.md`;
   - `IMPLEMENTATION/K3A/RETURN.md` (sha256 `3413a94a…`; §2, §3, §4, §8 and "RV2 fixes");
   - `CHANGE_RECORD.md` (`a94c121e…`);
   - `K3A_MERGE/RECORD.md` (`ddad9baa…`; the gates and N8);
   - `REVIEW/K3A_REVIEW.md` (`619e4311…`; §3.3–§3.6 and Addendum A).
6. **K-D5's records:** `IMPLEMENTATION/KD5/RETURN.md` item 8 and its list of the 13 per-item allowances.
7. **The code on your base:**
   - `retained/wide.rs` (`9f97ea03…`) and `mod.rs`;
   - `FK/tests/retained_wide/**`;
   - `formation_check.rs` (`:32`, `:145`, `:229`);
   - `formation_check_tests.rs` (`:11`, a test-only rounding helper through the split and `ExactAccumulator`);
   - `FK/exact_sum.rs` (the `ExactAccumulator` API: `round`, `round_scaled`);
   - `PP:1110-1125` and `:4441`;
   - `SA:174` and `:469`.
8. `T3/OWNER_DIRECTION.md`, "Owner decision (2026-09-28): DEC-025 on the Mac".
9. `TASK_BRIEFS/I10_K2B_IMPLEMENTATION.md`, for coordination only.

## Base, branch and paths

- **Branch:** `codex/piping-k3-20260928`, from current main. ROOT creates it in `<wt>/k3` and records the SHA at spawn.
  - At drafting, main is `6e18505e3`. Its `P/core`, `P/validation` and `P/fixtures` trees equal those of `eb52114e9` (K1 merged), so ROOT's Mac baseline of main at `eb52114e9` applies unless main moves.
- **Target:** `<wt>/k3-target`.
- **Scratch:** `<wt>/scratch/i11`.
- **Mutants:** one clean copy and one clean target per mutant, under `<wt>/k3-mut/<mutant>/`. Delete each target afterwards.
- **Python:** `<VENV>`. The generator uses the standard library only (`fractions`, `math.isqrt`, `hashlib`), like K3a's.

## Coordination with K2b (I10)

- **K2b is running** on `codex/piping-k2b-20260928` in `<wt>/k2b`. Its write set (the I10 brief and ROOT's checkpoint-0 rulings 4–5) is:
  - `FK/lib.rs`, `FK/structural.rs`, `FK/structural/sparse.rs` and `FK/load_ledger.rs` (`force_scaled`);
  - `SA`;
  - `nonlinear_integration/src/s11k_tests.rs`, `FK/tests/s11_site_table.rs` and `PP/tests/s11f_site_test.rs`;
  - new test files. At drafting these are `FK/tests/k2b_force_scaling.rs` and `SA`'s `structural_adapter/k2b_models.rs`.
- **The write sets are disjoint.** This holds for the recommended form, and also if Q4 adds `exact_sum.rs` to K3, because K2b does not write `exact_sum.rs`. Overlap would arise only if K3 needed a site-table row (a stop) or an edit to `FK/structural.rs` (not needed).
- **Semantic touchpoints, for ROOT:**
  1. K2b's tests run K-D5's formation check under b, and that check calls `Wide<2>`. Because K3 leaves `Wide<2>` bit-identical, neither branch's tests move.
  2. I10's uncommitted draft in `<wt>/k2b` defines a `Representability` for unscaled binary64 publication:
     - an exact zero counts as normal;
     - a subnormal carries its relative precision, 2^-1075/|value| rounded upward;
     - nonzero underflow and overflow are refused.

     K3's conversion outcome expresses the same §5 item 7 categories for `Wide` → f64. K3 cannot import a type from a parallel branch. Q5 recommends the same convention, so that K4 and F2a can map both onto one publication rule.
- **Merge order: either may merge first.** The second merges main, then re-runs its suites and T9 before its PR merges.
- **Host.** Two T3 implementers share the Mac.
  - Each keeps to I8R's caps: at most two of its own cargo jobs, `-j 8`, `RUST_TEST_THREADS=4`, and at most three mutants at once at `-j 4`.
  - ROOT may serialize the heavy phases: T9 builds, mutation batches and generator runs.

## Required tests

**A. Nothing existing moves**
- K3a's `tests/retained_wide/**` is byte-identical.
  - Its 19 tests pass unchanged. They pin `Wide<2>`'s `Debug` tokens, `from_parts`, the error names, `at_least_one_twentieth` and the arctangent bits.
  - `gen_wide_vectors.py --check` reports OK for all six of its files.
- K-D5's tests pass unchanged:
  - `formation_check_tests.rs`;
  - `nonlinear_integration/src/structural_adapter/kd5_tests.rs`;
  - `PP/tests/formation_check_runtime.rs`;
  - `PP/src/f1a_tests.rs`.
- The existing `WideError` `Display` strings are pinned by a new test, byte for byte.
- `frame_kernel`'s full suite passes, and so does CI's 39-manifest profile, against the Mac baseline of main. Every failure must be identical to a Mac-main failure.
- The non-test FK build has no warnings.
- T9 (see Gates).

**B. The arithmetic at L = 4, 8 and 16** (§4.11), for + − × ÷ √:
- **p = 53, bitwise against hardware binary64,** over random normal-range operands (at least 10^5 per operation per width, as in K3a).
- **p = 64L, against exact-rational vectors** from the generator.
- **The targeted hard classes.** They run at p ∈ {53, 128, 256, 512, 1024} with p ≤ 64L; at every limb boundary (64k − 1, 64k and 64k + 1 for k = 1 … L); and at the refinement precisions 192, 320 and 576. The classes are:
  - exact ties to even at each limb boundary;
  - carry-out and renormalization on addition and multiplication, including a carry through every limb (all ones plus half an ulp at L = 16);
  - massive cancellation: operands within one ulp, and within 2^-p′;
  - exact and near-exact division and square root (perfect squares, and values one ulp away);
  - sticky-bit paths, including a nonzero tail in a limb far below the round bit;
  - `i64` exponent extremes, refused and never wrapped.
- **Signs of zero; the lift; the precision range.** p outside [2, 64L] is refused.

**C. The conversion to binary64, at every width including L = 2**
- **The outcomes,** with the zero convention Q5 settles.
- **The boundary classes, at every width:**
  - the largest subnormal and the smallest normal;
  - values in [2^-1022 − 2^-1075, 2^-1022), which round up into the normal range;
  - exactly 2^-1075, a tie that rounds to zero and so underflows;
  - 2^-1075 plus the smallest tail each width can carry, which rounds to the smallest subnormal;
  - subnormal ties with sticky bits far below them (hundreds of binades down at L = 16). **No double rounding.**
  - the largest binary64 value (MAX); just below the overflow midpoint 2^1024 − 2^970 (rounds to MAX); the midpoint itself (overflows);
  - Wide exponents near ±2^62 (overflow or underflow, never a wrap).
- **A seeded conversion differential against `Fraction`,** at least 10^6 values per width, concentrated near the subnormal and overflow boundaries.
- **Lifted binary64 values** convert back to the same bits, with the normal or subnormal outcome.
- **A cross-check against `ExactAccumulator::round` at L = 2.** Feed the split terms to the accumulator and round once. The bits must be the same wherever both are defined.
  - The check applies only to values whose split is exact (no set bit below 2^-1074). The split truncates below that, which would double-round.
  - The accumulator gives +0.0 for underflow and `NonRepresentable` for overflow. Where the conversion reports underflow or overflow, these are the only allowed differences.

**D. The seeded differential** (§4.11: "at least 10^6 operations per precision, generated with a fixed recorded seed")
- At least 10^6 operations at each of p = 256, 512 and 1024 (the full width of L = 4, 8 and 16).
- A mixed-p stream per width, p ∈ [2, 64L], weighted toward the precisions W1 uses.
- K3a's scheme, which ROOT accepted at K3a's merge: the generator, the recorded seed, the stream and per-chunk sha256 digests, and the first 1,000 records of each stream committed. The Rust test regenerates the operands.
- The oracle is `fractions.Fraction`, rounded once; square roots use `math.isqrt` with an exactness test.
- **Record the debug-build wall time** of these tests per width, as an observation for Q7. It is not a performance claim.

**E. Subject to Q3**
- **TwoSum and TwoProduct are error-free** at every width and at every class precision: s + e = x ∘ y exactly, with |e| ≤ ulp(s)/2.
- **Widening and narrowing.** Widening is exact and round-trips. Narrowing is the correct rounding to the target p.
- **The integer constructor** matches `Fraction`. That includes a 68-limb magnitude with quantum 2^-2148, like `ExactAccumulator`'s, with ties and far sticky bits.
- **Arithmetic-level analogues of §7.3 mutations 13 and 16.** V1's check-L sum (1e80, 1e-8, −1e80), and a duplicate-operand cancellation (two equal terms, then a third term 2^-300 as large), must be exact through an expansion, and must lose the small term when folded at p.

**F. The work counter**
- Counts by operation kind and by width, saturating.
- The limb-cost function of Q9, if ROOT adopts one.
- K3a's `WorkCounter` fields and `rounded_operations()` keep their meaning at L = 2. K3a's work-counter test and K-D5's cost test pass unchanged.

## Mutants

Run from clean copies, with a NONE control first. Each mutant must be killed at a behavioural assertion; name the killing test. A survivor is a defect to report; never weaken a test to kill it.

| # | Mutant (in the new code unless stated) | Intended kill |
|---|---|---|
| M1 | round toward zero | B targeted ties; D; p = 53 hardware |
| M2 | dropped sticky bit | B sticky class; D |
| M3 | ties away from zero | B ties at limb boundaries; D |
| M4 | off-by-one shift across a **middle** limb (limb 2→3, reachable only at L ≥ 4) | B limb-boundary classes at L = 4, 8, 16; D |
| M5 | addition carry chain stopped after one limb | B carry through every limb |
| M6 | schoolbook product drops a cross term or the top carry | B carry class for ×; D |
| M7 | division or square-root remainder ignored as sticky | B near-exact ÷ and √ |
| M8 | conversion double-rounds (to 53 bits, then to the subnormal quantum) | C subnormal ties with far sticky bits |
| M9 | conversion flushes subnormals (reports underflow) | C largest subnormal; conversion differential |
| M10 | overflow midpoint rounds down to MAX | C overflow midpoint |
| M11 | nonzero underflow returned as a silent ±0.0 | C 2^-1075 tie; underflow outcome test |
| M12 | sign of zero, or of an underflow, lost in the conversion | C sign cases |
| M13 | unchecked exponent arithmetic in the new path (wraps) | B exponent extremes |
| M14 | (Q3) TwoSum returns e = 0 | E error-free tests; the §7.3-16 analogue |
| M15 | (Q3) narrowing truncates instead of rounding | E narrowing |
| M16 | a width's operations not charged, or charged to the wrong width | F |
| M17 | K3a's `round_pack` tie rule changed (the L = 2 path) | A: K3a's suite unchanged, and K-D5's tests |
| — | your own, at least two | — |

§7.3's method mutations that rest on the arithmetic need K4's method, and are K4's: 5 (fixed precision), 6 (no 2p verification), 13 (folded loads), 14 (term-by-term combination), 15 (combination outside the stop rule) and 16 (sequential assembly). K3's E analogues make sure K4's kills rest on tested error-free transformations.

## Gates (ROOT runs the PR)

- **Suites.** `frame_kernel`'s full suite, and CI's 39-manifest profile with `--no-fail-fast`, against ROOT's Mac baseline of main.
- **T9 (Mac-only).** The committed-fixture diff, as I8R and I10 run it: base and candidate both built on this Mac from `git archive` copies, and compared with each other.
  - 112 of 112 byte-identical is expected.
  - Because `wide.rs` is live through K-D5, T9 is real evidence here, not a formality.
  - **Any committed-byte change stops the work.**
- **The both-entry gate is not run** (subject to Q2).
- **An independent complete-diff review.** It includes an oracle independent of K3's generator, as RV2 did for K3a.
- **Hosted CI** green on the candidate head.
- **DEC-025** under the owner's Mac decision (`OWNER_DIRECTION.md`, 2026-09-28):
  - the Mac sandboxed sweep, where the only cargo failures are the three known platform tests, identical to Mac main;
  - pytest, vitest and the build pass;
  - hosted Linux CI's numerical cargo job supplies the clean Linux cargo run;
  - the deviation is recorded in the merge record.
- **No native witnesses.** K3 is kernel only and touches no native path, as with K3a.

## Checkpoints

End your turn at each one with a status for ROOT: the changed files, the results, and any stop. ROOT verifies, commits and resumes you.

- **0: a plan, before any product code.** It covers:
  - Q1's structure: how the new widths are built, and how `Wide<2>` stays byte-identical;
  - the type and function signatures (the widths, the arithmetic context, the conversion outcome, and the Q3 items);
  - how the 2L-limb intermediates avoid heap allocation;
  - the conversion algorithm, rounded once from the exact value;
  - the generator layout, seeds and counts;
  - the dead-code plan;
  - the test and mutant list;
  - your position on each open question that is still unresolved.
- **A:** a clean compile; the targeted tests (B, C, E); K3a's and K-D5's tests unchanged; a warning-free non-test build; and the debug wall times for D.
- **B:** the suites against the Mac baseline of main, and T9.
- **C:** the mutation table, with the NONE control first.
- **D:** CHANGE_RECORD and RETURN, with `_run_records/` and SHA256SUMS.

**Stop and report** (end your turn) on any of these:
- a change to any existing `Wide<2>` result, `Debug` token or `WideError` `Display` string: a K3a vector, a differential digest, a K-D5 test or a T9 byte moves;
- a needed edit outside the write set;
- a new dependency;
- a K3a or K-D5 test failing;
- a mutant that survives;
- a design item that cannot be implemented as specified (integer only; no allocation per operation);
- a SIGKILL from the memory guard. Check `<wt>/guard/memguard.log`, and do not retry blindly.

## Return

- **Files:** `T3/IMPLEMENTATION/K3/` on the K3 branch, with CHANGE_RECORD (following `.agents/skills/chirality-change/SKILL.md`), RETURN, `_run_records/` and SHA256SUMS.
  - Placeholders only (`<wt>`, `<scratch>`, `<VENV>`); no machine paths and no model identifiers.
  - State the platform (`aarch64-apple-darwin`, rustc 1.97.1). State that T9 is a Mac-only comparison.
- **RETURN covers:**
  - the files and their line counts;
  - how each K3 item was met, with the design's words;
  - the per-crate counts against the baseline;
  - each differential's seed, count and digests;
  - the conversion boundary results;
  - the mutation table;
  - T9;
  - the toolchain and host;
  - the delegation mechanism;
  - what was not done.
- **RETURN has a "K4 interface" section with exact signatures,** as K1's §12 did for F1b.

## Open questions for ROOT (each with a recommendation)

**Q1. Generalize `Wide<2>`, or add beside it?**
- **(a) Add.** Leave K3a's `impl Wide<2>` code paths as they are. Build L = 4, 8 and 16 beside them, for example as per-width impls from one macro, or as a generic core under distinct names, so that inherent methods do not overlap with `Wide<2>`'s. Instantiate the new core at L = 2 in tests only, and cross-check it bitwise against K3a's path over K3a's vectors and differentials.
- **(b) Refactor.** Move every width, `Wide<2>` included, onto one generic core.
- **Recommendation: (a).** `Wide<2>` runs on every Passed linear case through K-D5.
  - Under (a), its bits and its speed stay unchanged by construction.
  - Under (b), any slowdown of the product path would go unmeasured: timing claims are not allowed on the Mac (I8R), and K6 owns them.
  - Correct rounding is unique, so the cross-check under (a) is a strong test of both paths. The cost of (a) is some duplicated arithmetic.

**Q2. Evidence for "no published byte changes".**
- **Recommendation:** under Q1(a), accept "no published byte changes, by construction", with this evidence:
  - T9, 112 of 112;
  - K-D5's suites unchanged;
  - the `Display` pin in A.

  Do not run the both-entry gate (as for K1 and K2b).
- Under Q1(b), add the recorded op counts of K-D5's cost test, which must be unchanged. There would still be no timing claim.

**Q3. Arithmetic for K4, beyond the row's letter.** K4 cannot edit `wide.rs`. Its method needs:
- exact widening, for the stop rule's |q_p − q_2p| ≤ 2^-64·max(…) across widths (§4.1.6), and for the p + 64 residual (§4.1.4);
- correctly rounded narrowing;
- TwoSum and TwoProduct at every width, for exact assembly (§4.1.2 item 4) and exact combinations (§4.1.1);
- a correctly rounded constructor from an exact integer times 2^e (Q4);
- work counts per width (Q9).

**Recommendation: include all of them.** Each is small and each is tested against `Fraction`. Without them, K4 would need `wide.rs` in its write set.

**Q4. §4.1.2 item 5's projection has no owning slice.** The design says `ExactAccumulator` "gains one further projection, to `Wide<L>` at p". That touches `exact_sum.rs`, which is in neither K3's nor K4's write set. `exact_sum.rs` is live S11 code, and it is in the S11 site table's sources.
- **Recommendation:**
  - K3 supplies the arithmetic: Q3's integer constructor, in `wide.rs`, tested against `Fraction`.
  - K4 adds a one-function `pub(crate)` read accessor to `exact_sum.rs`, as a declared write-set extension.
  - This keeps K3 out of a live S11 file.
- **Alternative:** K3 takes the accessor. `exact_sum.rs` then joins K3's write set, which stays disjoint from K2b's.

**Q5. The conversion's outcome encoding.**
- **Recommendation: mirror the convention in I10's draft.**
  - An exact ±0 converts to ±0.0 with the normal outcome, keeping its sign. §4.1.2's "+0.0" rule stays with the publishing caller.
  - A subnormal carries its relative precision as 2^-1075/|result| rounded upward. The absolute bound is 2^-1075.
  - Underflow means only a nonzero value whose correct rounding is ±0.
  - Overflow means a value whose correct rounding is ±∞.
  - Neither underflow nor overflow ever returns a value silently.
- ROOT may later unify the two types at K4 or F2a.

**Q6. The arctangent and the split stay at L = 2.**
- **Recommendation: yes.**
  - The arctangent's proof holds only for 53 ≤ p ≤ 128. The 15-term cutoff needs p ≤ 128 (RV2 §3.1).
  - W1a has no angles. W1c (T4) would reopen the arctangent, with a new proof.
  - Only K-D5 uses the split. W1b's p-bit ledger terms (F3) may need a wider split later; that is F3's to raise.

**Q7. The differential's size against CI time.** §4.11 requires at least 10^6 operations per precision, and §7.4 runs them in FK's own suite, and so in hosted CI, which builds in debug.
- K3a's whole `retained` filter, including its 1.2 million differential operations at p ≤ 128, ran in 11.9 s single-threaded in debug (K3a RETURN §4).
- Per-operation cost grows about as L² for multiplication, and for the bit-serial division and square root. So the L = 16 stream may take many minutes in a debug build. This is an estimate, to be measured at A.
- **Recommendation:**
  - Keep 10^6 per precision in the default suite if ROOT accepts the measured time.
  - Otherwise, a `[profile.test]` opt-level in FK's `Cargo.toml` (no lockfile change). This keeps every vector in CI.
  - Do not move the full streams into `#[ignore]` tests, which would depart from §7.4.

**Q8. An observation for K4, not a K3 decision: the ceiling and the p + 64 residual.**
- A 512-bit candidate is verified at 1024. If the verification solve also refines at p + 64 = 1088 bits (§4.1.4), `Wide<16>` (1024 bits) cannot hold it.
- **Recommendation:** K3 builds the row's widths (4, 8, 16). ROOT settles the ceiling's refinement in K4's brief: either no p + 64 residual on the ceiling's verification-only solve, or one more width.

**Q9. The work unit.**
- K3a counts operations by kind. §4.1.7 counts budgets "in limb-multiply equivalents per attempt", and an attempt spans widths: a 128 solve, a 192 residual, a 256 verification, and so on.
- **Recommendation:**
  - K3 keeps K3a's counts unchanged at L = 2.
  - It adds counts per width, a counter that can be merged across widths within one attempt, and a stated, deterministic limb-multiply-equivalent cost per operation kind and width, set out at checkpoint 0.
  - The limits themselves stay ROOT's, set from W3 and W5 measurements (§4.1.7).
