# I2: implement slice K3a (the `Wide<2>` arithmetic K-D5 needs)

This is an implementation TASK. Read `_COMMON.md` first. This brief overrides it where they differ (writes and builds).

## Purpose

Implement slice K3a of the selected design (`ROOT_SELECTION_DESIGNS.md`): the part of W1's retained-precision arithmetic that the formation check K-D5 needs. It is a small, heavily tested, in-repo arithmetic core. It adds no dependency and **changes no published value**: nothing in the product calls it until K-D5.

## Governing basis (read in this order)

1. `T3/ROOT_SELECTION_DESIGNS.md`, especially **condition C1** (the arctangent) and **C5** (landing and timing).
2. `T3/DESIGN_NUMERICS/DESIGN.md`, **revision 5a.2** (`932698d7a`, sha256 `fb62ef4a…`):
   - the K3a row of the §6 slice table;
   - §4.1, the `wide.rs` bullet;
   - §4.11, build feasibility and the V1-S9 test plan;
   - §7.4, arithmetic testing;
   - §4.3.1, where K-D5 uses `Wide<2>`: the exact split into at most three binary64 terms for `ExactAccumulator::add_product`.
3. `T3/DESIGN_NUMERICS/R5_4_CURVED.md` (`2c9fae78`): the arctangent's role, and the fact that sine and cosine come from square roots only.
4. `T3/REVIEW/VERIFY_R5.md` (`8097e13b`) and `REVIEW/_run_records/verify_r5/probe_atan_p128*`: V1's measured arctangent bound and its 13 angles.
5. S11-K's `exact_sum.rs` and `ExactAccumulator` on the S11-K branch, which you interface with. Do not modify them.

## Base, worktree and branch

- ROOT creates a dedicated worktree `<k3a-worktree>`, on a new branch from **the S11-K head** (the candidate after I1's regeneration commit). That lets you build against `exact_sum.rs` and `ExactAccumulator`.
- **K3a lands only after S11-K merges** to main, with its own full-gate PR (C5). Before its PR, the manager rebases or merges K3a onto main containing S11-K.
- Make no Git writes. The manager commits.

## Write set (from the K3a row)

- **New:** `P/core/solver/frame_kernel/src/structural/retained/mod.rs` and `retained/wide.rs`.
- **Edit:** `P/core/solver/frame_kernel/src/structural.rs`, **only** to add the single `mod retained;` declaration (MOD-D) and any `pub(crate)` re-export it needs. Nothing else in that file.
- **New tests:** vectors and their generator under `P/core/solver/frame_kernel/tests/`. The generator is a standard-library Python script, checked in as a `.py` beside the tests. The vectors are committed with their sha256 recorded.
- **Records:** `T3/IMPLEMENTATION/K3A/**` in `<k3a-worktree>`.
- No lockfile, Cargo.toml dependency, product, fixture or schema change. `frame_kernel` stays dependency-free.

## What to build

**`Wide<2>`:** sign, an `i64` exponent, and a `[u64; 2]` significand.
- `+ − × ÷ √`, each correctly rounded to nearest, ties to even, at a runtime precision p ≤ 128.
- Integer-only arithmetic, so results are bitwise reproducible across platforms.
- Refuse `i64` exponent extremes; never wrap.

**Conversion and splitting:**
- An exact lift from `f64`, covering normal, subnormal and ±0; non-finite input is refused.
- An exact split of a `Wide<2>` value into **at most three binary64 terms** whose exact sum equals the value, for `ExactAccumulator::add_product`. It includes the design's sub-2^-1074 allowance: state exactly when the split is exact and what happens below 2^-1074, as the design specifies.

**The arctangent (C1):** atan for the open interval 0 < φ < π, as used for the included angle.
- Half-angle reduction to t < 0.05, then the series, with at most 5 reductions and at most 15 terms.
- Either **prove** a bound, or **record the measured bound as its specified tolerance**. Say which, in the code documentation and in the return.
- Test vectors must go **beyond V1's 13 angles**: across 0 < φ < π, including near both ends (φ → 0⁺ and φ → π⁻, several decades), π/2, and small angles. Compare against a high-precision rational or `decimal` reference (at least 120 digits), with a few-ulp tolerance at p = 128. Report the worst error you observe.

**A work counter** (operations counted), as the design specifies.

## Tests (V1-S9's plan, scoped to L = 2)

- **At p = 53:** bitwise equal to hardware binary64 over random normal-range operands, for + − × ÷ √.
- **Targeted hard classes** at p ∈ {53, 128}, for every operation:
  - exact ties to even at the limb boundaries (bits 63/64, 127/128);
  - carry-out and renormalization;
  - massive cancellation (operands equal to within one ulp, and to within 2^-p′);
  - exact and near-exact division and square root (perfect squares, and values one ulp from them);
  - sticky-bit paths;
  - the TwoSum and TwoProduct error-free transformations;
  - exponent extremes refused.
- **A seeded `Fraction` differential** at p = 128: at least 10^6 operations across + − × ÷ √, from a fixed recorded seed. If committing all the vectors is too large, commit the generator, the seed and a sha256 of the generated stream, and have the Rust test regenerate or stream them deterministically. Say which you chose.
- **The split:** exactness on random and adversarial values, and the sub-2^-1074 allowance.
- **The arctangent:** as above.
- **Seeded rounding mutants,** each of which must be killed: round toward zero, a dropped sticky bit, ties away from zero, an off-by-one limb shift, and an arctangent mutant (for example, one reduction fewer, or a truncated series). Record each mutant's patch, the command, and the killing test. A survivor is a defect to report; never weaken a test to kill it.
- **Existing suites:** `frame_kernel`'s full suite passes. Every crate that depends on `frame_kernel` by path still compiles and passes: at least `product_physics`, `nonlinear_integration`, `sparse_direct`, `straight_pipe`, `curved_bend`, `load_case_algebra`, `runner/headless`, `result_export` and `apps/desktop/src-tauri`. Since nothing calls K3a yet, there must be **zero** committed-byte or fixture changes. Confirm this by running the committed fixture requests through base and candidate, as S11-K did. Any difference is a stop.

## Build rules

- `RUSTUP_TOOLCHAIN=1.97.1`, `CARGO_INCREMENTAL=0`, `CARGO_TARGET_DIR=<t3-target>`, `--offline --locked`.
- **One heavy cargo job at a time across T3.** Check `pgrep -x cargo` before every build or test, and wait while another runs. RV1 may be reviewing S11-K on the host.
- Keep free disk above about 8 GB, and prune only your own output.
- Run `cargo fmt` on the changed files. Check whitespace with `git diff --no-index --check` or grep. **Make no Git index operations.**

## Disclosure and PR record

Draft `T3/IMPLEMENTATION/K3A/CHANGE_RECORD.md` following `.agents/skills/chirality-change/SKILL.md`. It covers:
- what was added;
- that no published value changes;
- the arctangent bound and its status (proved or measured);
- the test plan and results;
- the mutation table;
- the dependency statement (none).

## Return

Write `T3/IMPLEMENTATION/K3A/RETURN.md`, with logs under `_run_records/` and a `SHA256SUMS`, and no machine paths. It covers:
- the files changed and their line counts;
- how each K3a-row item and C1 was met;
- the per-crate test counts;
- the differential's seed, operation count and hash;
- the arctangent worst error, with its angle set;
- the mutation table;
- the fixture-identity result;
- the toolchain;
- what was not done.

Send the manager a SendMessage summary. Message the manager at once if a design item cannot be implemented as specified, or if any committed byte changes. Don't improvise a different design.
