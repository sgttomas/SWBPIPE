# RV2: independent full-diff review of slice K3a

This is a review TASK. Read `_COMMON.md` first; this brief overrides it where they differ.

You must be independent: you did not design W1 or K3a, did not review their design, and did not implement K3a. Your job is to find defects, not to confirm. Report what you find; you fix nothing.

## Candidate

- **Branch:** `codex/piping-k3a-20260926`, in `<wt>/k3a`.
- **Candidate head:** `a2e804a757359d589f4c31ea8e36a923f28ccb8c`.
- **Base:** `4912dc636`, the S11-K PR head at K3a's cut.
- **Scope:** review the full diff `4912dc636..a2e804a75`. That covers `FK/src/structural.rs` (one line), `FK/src/structural/retained/{mod.rs,wide.rs}`, `FK/tests/retained_wide/**` (tests, generator, vectors, SHA256SUMS), and the records under `T3/IMPLEMENTATION/K3A/**`.
- K3a's PR opens only after S11-K merges, rebased or merged onto main. This review does not need S11-K merged.
- **Write set:** `T3/REVIEW/K3A_REVIEW.md` and `T3/REVIEW/_run_records/k3a_review/**` (with its own SHA256SUMS), in the **numerics** worktree (`<wt>/numerics`). Do not write in `<wt>/k3a`. Mutations and builds run in a scratch copy.

## Basis

1. `T3/ROOT_SELECTION_DESIGNS.md`: **C1** (the arctangent) and **C5**.
2. `T3/DESIGN_NUMERICS/DESIGN.md` revision 5a.2 (`fb62ef4a`): the K3a slice row, §4.1's `wide.rs` bullet, §4.11 (V1-S9's test plan) and §7.4.
3. `T3/TASK_BRIEFS/I2_K3A_IMPLEMENTATION.md`: I2's brief.
4. `T3/ROOT_RULINGS_V1.md`: "K3a arctangent and host disk". ROOT accepted the proved 23.6 ulp at p = 128, with the measured 5.41 ulp recorded alongside and a test tolerance of at most 23.6 that covers 5.41.
5. `T3/REVIEW/VERIFY_R5.md` and its `probe_atan_p128*`: V1's measured 2.69 ulp over 13 angles.

## What to check (at least)

1. **The arctangent proof itself.** Read the proof in `wide.rs`'s documentation and in `IMPLEMENTATION/K3A/RETURN.md` §3, and **check it step by step**, not only through the tests:
   - the half-angle reduction error;
   - the reduction count (≤ 5) to reach t < 0.05;
   - the series truncation after ≤ 15 terms;
   - the rounding error per operation at p;
   - error propagation through the reductions (the ×2 per reduction);
   - the final ×2 in `included_angle`;
   - the domain (0 < φ < π, 1 + c > 0, t up to 2^±100000);
   - the claimed range 53 ≤ p ≤ 128.
   Is 23.6 ulp a valid upper bound? Give your own bound if you can.
2. **The smallest-first tail summation** I2 disclosed. Is it what the proof assumes? Does the proof hold for this order? Is it deterministic? V1's forward summation reached 7.28 ulp on the same set; explain the difference.
3. **The `#[path]` test module.** `wide_tests.rs` is compiled as `wide.rs`'s test module via `#[path]`, because `retained` is crate-private. Check:
   - it runs under `frame_kernel`'s own `cargo test`, and so in CI;
   - it is not silently excluded;
   - nothing test-only leaks into the non-test build;
   - the arrangement is sound under `cargo test --release` and doc tests.
4. **The seeded-differential regeneration scheme.** The streams are not committed. The Rust test regenerates operands from the seeds (`4b33415f57494445` at p = 128, 10^6 ops; `4b33415f4d495844`, mixed p, 2·10^5 ops) and checks 1,000 sample records, the per-10^5-chunk digests and the stream digest against the committed values. Check:
   - that the scheme actually detects a wrong result anywhere in the stream, not only in the sampled records;
   - that the generator `gen_wide_vectors.py --check` regenerates every committed vector file byte for byte;
   - that the expected values come from `Fraction`, not from the code under test.
5. **Correct rounding, independently.** Spot-check + − × ÷ √ at p ∈ {53, 64, 65, 127, 128} against your own `Fraction` computation, including ties to even at the limb boundaries, sticky paths, massive cancellation, and near-exact ÷ and √. Check the exact f64 lift (normal, subnormal, ±0; non-finite refused) and the split (exactness above the minimum subnormal, and the truncation flag below it).
6. **The mutation sample.** Read the mutation table and re-run a sample in scratch: at least M2 (dropped sticky), M4 (limb shift), M5 (one reduction fewer) and M6 (truncated series). Confirm each is killed by the named test. Construct at least one mutant of your own in the arctangent or the split, and report whether it is killed.
7. **Zero byte change.** Check the fixture-identity result (112 of 112 outputs byte-identical to base) by re-running S11-K's harness or an equivalent on base and candidate, or by reading and spot-checking I2's records. Confirm there is no Cargo.toml or lockfile change and no dependency.
8. **The `dead_code` allowance** is limited to the `retained` module: an inner `#![allow(dead_code)]` in `retained/mod.rs`, with nothing wider. K-D5 is to remove it.
9. **Hygiene:** no machine paths, `cargo fmt` clean, `git diff --check` clean (or the exceptions recorded), the records' SHA256SUMS verify, and `CHANGE_RECORD.md` follows `.agents/skills/chirality-change/SKILL.md`.

## Running things

- `RUSTUP_TOOLCHAIN=1.97.1`, `RUSTUP_AUTO_INSTALL=0`, `CARGO_INCREMENTAL=0`, `--offline --locked`, and your own `CARGO_TARGET_DIR` under `<scratch>` or `<wt>/rv2-target`, pruned when done.
- **Cargo priority on the host:** I1's S11-K fixes, then ROOT's DEC-025 sweep, then you, then I3. Check `pgrep -x cargo` before each build, and hold while the sweep runs.
- Keep free disk above about 8 GB. Skip no tests and raise no timeouts. Make no Git writes.

## Verdict and return

Write `T3/REVIEW/K3A_REVIEW.md`, containing:
- the revisions reviewed;
- a findings table (ID, severity BLOCKING / SHOULD-FIX / NOTE, site, evidence, resolution);
- a section per check;
- your own arctangent bound and your reading of the proof;
- what you ran;
- what you did not check.

The verdict is **PASS** (no unresolved BLOCKING findings) or **FAIL**. Send the manager a SendMessage summary with the verdict, the counts and the file's sha256.
