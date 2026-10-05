# I69, I70, I71: U8-2 (corpus 07l) and U8-3 (the Rust and TS alignment)

TASKs (Type 2). Read `BRIEFS/U8_COMMON.md` first; it binds you. Your plan sections are PLAN §1.2 (the "Corpus and the three readers" row), §1.4 (U8-2 and U8-3) and decision 6.

**Dispatched only if L = 0 publishes** (I68's probe and Part 2). If L = 0 is deferred, U8 has no corpus change, and ROOT does not dispatch this brief.

**The order is sequential:** I69 first (Python and the corpus). Then I70 (Rust) and I71 (TS) in parallel, on disjoint files, one cargo job at a time. All three share `WT/f2a-u8`.

## I69: corpus 07l (Python)

**Your fence:**
- `P/fixtures/results/retained_precision_cases.json`;
- `P/tests/test_retained_precision_contract.py`.

**Add:**
1. **Two producer-solved bases,** the L = 0 successor in each mode. Each is a byte-identical copy of I68's pinned fixture (D-U6-5), with case-level `provenance` naming the producer, the entry (`run_linear_static_preview_value_with_retained_direct`), the build identity and the U8 head.
2. **The L = 0 mutations** from `R/I62/coverage_shared_python_01/SNAPSHOT_05_PLAN.md` §1.2 (for example `isolated_rotation_stop` at G5a SCALE). Each has its expected first gate and code.
3. **Must-pass entries** for the two bases.
4. **The top-level `provenance.claim`,** amended to: "synthetic controls plus listed producer-solved bases; no native Current evidence" (decision 6).

**Then:**
- Python passes the full 07l: the corpus test, the 24-file sweep and the three retained suites;
- the counts move from (15, 278, 24) to (17, 278 + k, 24 + j), at `tests/test_retained_precision_contract.py:640–643`.

**If Python's reader rejects a faithful base,** that is a reader defect. **Stop and return.** A Python reader fix is a reader review only, but ROOT rules first.

**Records:** `NUM/R/I69/u8_corpus_07l_01/`, with the corpus sha256, the counts, the mutations' first gates, and SHA256SUMS.

## I70: Rust alignment (after I69)

**Your fence:** `P/core/reporting/result_export/tests/retained_precision_contract.rs`, counts and pins only, at `:276–281`.

**Not** the Rust reader's `src/` code, including its `cfg(test)` module. It runs the full 07l.

**If the Rust reader rejects a faithful base, STOP.** A Rust reader source fix changes the D1 call graph (precommit) and re-opens re-qualification (RR:10474). Return to ROOT.

**Records:** `NUM/R/I70/u8_rust_07l_01/`.

## I71: TS alignment (after I69, in parallel with I70)

**Your fence:** `P/apps/desktop/src/features/results/retainedPrecision.test.ts`, counts and pins only.

It runs the full 07l under vitest, with tsc clean. **If TS's reader rejects a faithful base, stop and return.**

**Records:** `NUM/R/I71/u8_ts_07l_01/`.

## Budget and return

- I69: 2–3 h.
- I70 and I71: 1–1.5 h each.

Each returns once, with changed files and hashes, the full-corpus outcome, and anything ROOT must rule on.
