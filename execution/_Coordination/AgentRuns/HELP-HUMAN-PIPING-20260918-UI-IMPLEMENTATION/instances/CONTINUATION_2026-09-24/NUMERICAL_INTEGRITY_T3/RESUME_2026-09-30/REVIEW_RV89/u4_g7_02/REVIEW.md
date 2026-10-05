# RV89: U4 Pass B on the U7 head (slice Q)

**Reviewer:** RV89, TASK (Type 2), dispatched directly by ROOT. No descendants. This continues RV89's G7 reviews (`R/REVIEW_RV89/u4_g7_01/`: REVIEW.md and ADDENDUM_01.md).

**Candidate:**
- **The U7 head:** `cfda60403f` on `codex/piping-f2a-u7-20261004`.
- **The run:** I65's Pass B on it, `R/I65/u4_g7_04/` (RETURN.md `7b439fc1…`; `runs/u7/` and `runs/u7a/`), with u4_g7_03's tools, which carry RV89 N-4 and N-5.
- **The new entries:** three in `delta_reviewed.json`, now 6 entries.
- **The flag's evidence:** I66's, `R/I66/u7_slice_f_01/RETURN.md` §3.

**My copy:**
- WT/rv89_u7/base, a `git archive` of `cfda60403f`.
- Its projects/chirality-piping equals the tree: 2,950 of 2,950 blobs, with no extra file.
- Against `7f07a2f7b4`, outside `execution/`, there are 22 changed files. The Rust ones are those ROOT lists.

**Oracles:** RV89's own.
- My registered build of `cfda60403f`: law tests, all nine witnesses, the challenge, PP, runner/headless and my 71-input sweep.
- A counting-allocator probe of the eligibility switch, run as committed and with only the flag set back to `false`.
- Pass B's `entry` and `outcomes` gates and `delta_inventory2.py`, run by me on my copy (`evidence/`).

## Verdict: **PASS**

| | Count |
|---|---|
| BLOCKING | 0 |
| SHOULD-FIX | 0 |
| NOTE | 0 |

**The three new reviewed entries are right. The exit-6 delta is exactly the six tests. The entry is unchanged byte for byte, and the maxima are unchanged.** The TS-only head `e5e1693ceb` needs no Pass B rerun, and neither do fixture- or test-only commits, provided the conditions in §3 hold. A mechanical rerun at U9's gate suffices.

## 1. The three new reviewed entries

**The flag** (`retained_precision.rs:4267–4269`, D1-live):
- **The hunk.** It is line-neutral: two comment lines and the token `false` → `true` on :4269.
- **Its one use.** `IMPLEMENTATION_COMPLETE` is read only at :4309. PP's production code never reads `numerical_eligible`: it uses the precommit's `Ok`/`Err` only, at lib.rs:3161.
- **What the switch newly evaluates borrows only:** `Option::is_some`, `Index<&str>`, `Value == &str`, the `list` slice, `iter().all`, `text` → `&str` and `matches!` on literals. There is no text, and `Validation`'s layout is the same `bool` (the reader layouts stay 56/64/96/16).
- **Measured** (`evidence/eligibility_alloc_probe.txt`, `rv89_u7_alloc_probe.rs`):
  - **The harness:** a counting global allocator over all threads, so it includes W1's reserved-stack thread. It counted the milestone in both modes, twice each:
    - (a) the precommit reader `validate(successor, Some(invocation))`, with the invocation PP builds;
    - (b) the same reader without the invocation;
    - (c) the whole Direct entry.
  - **The comparison:** my copy as committed, against the same copy with only the flag set to `false`.
  - **Result:** as committed, (a) reports `numerical_eligible = true` (98 / 99 classifications), so the eligible branch is reached; with the flag off it reports `false`.
  - **The allocation counts and bytes are identical in all 12 pairs:**

    | Mode | Reader with invocation | Reader without | Whole Direct entry |
    |---|---|---|---|
    | Sparse | 75,446 allocations / 6,643,320 B | 71,766 / 6,237,548 B | 150,408 / 16,113,738 B |
    | Dense | 75,658 / 6,663,353 B | 71,978 / 6,257,589 B | 151,077 / 16,169,786 B |

  - The reader figures equal I66's in its §3.
  - **The successor's length is unchanged:** 113,733 / 114,894 B.
- **The switch is allocation-free and text-free on D1 in the registered build. Confirmed.**

**`lib.rs:2235` and `:2254`** (doc only): each adds an inline `#[doc = "…"]` attribute (slice P, RV92 N-6) on the line of an unchanged `pub fn successor` or `pub fn into_parts` signature. A doc attribute is metadata. It generates no code, no runtime text and no type change. My printed record and published bytes are unchanged (§2). **Confirmed.**

**No entry is needed elsewhere:**
- `lib.rs:3156` and `semantic_contract.rs:581–582` are comments (`no-code`).
- `retained_wire_tests.rs:122` is the U7 pin in a `test` file, which is not one of the three qualification-test files.
- My run of `delta_inventory2.py` (`ba1faa1c..cfda60403f`, on my copy):
  - with an empty table, it stops (5) on exactly six hunks: grant 2's three `cfg-test-stmt` and these three `item`;
  - with I65's table, it passes.
  - The other rows are 11 `test`, 1 `generated`, 2 `no-code` and 18 `not-d1`. The two changed fixtures are embedded only by `#[cfg(test)]` code (retained_precision.rs:4348–4353) and by result_export's integration tests.

## 2. The run

**My own registered build of `cfda60403f`** (`evidence/u7_run.txt`):
- **PP:** 705 passed, 1 failed (t13), 10 ignored. The outcome list is **identical to my final-basis (`7f07a2f7b4`) run**.
- **Against Pass A's reference,** the outcomes gate gives 6, with exactly six added `ok` lines: the five `retained_facade_tests::u3g2_*` and D-U6-5's `u3g2_d_u6_5_carrier_fixtures_are_the_live_successors`.
- `retained_wire_tests::u1_milestone_successor_both_modes` passes with its U7 assertion: eligible with the invocation.
- **Identical to my final-basis run:**
  - runner/headless (85 passed, 2 failed);
  - the law outcomes (42 passed, 0 failed, 9 ignored);
  - the printed record (identity, inputs, layouts, 244 atoms, all phases);
  - all nine witnesses' and the challenge's output lines, with peaks 3,541,898 / 2,252,863 B.
- **The maxima:** 0.8881 M sparse (3,575,778,286 B) and 0.8929 M dense (3,595,488,734 B), unchanged.
- **My sweep of `cfda60403f` is byte-identical to the G6R registered sweep** (sha256 `25cce1e1…ccbb`). Eligibility changes no published byte on my 71 inputs.
- **The entry:** Pass B's entry gate, run on my copy against `git show 0c7827b6ad`, gives equal, threshold `4_026_531_840`.

**I65's run** (`runs/u7/VERDICT.txt`): `DELTAS TO READ exit=6`, and its only non-zero gate is `pp_outcomes:6`. Its outcome diff lists the same six added lines.
- `text_run`, `noncand_run` and `controls_run` are now gates (N-4), and `pass_<tag>/` and the tag's cargo logs are cleared first. `delta_inventory2.py` now has the `qualification-test` class (N-5).

**The exit-6 delta is exactly the six tests, the entry is unchanged byte for byte, and the maxima are unchanged. Confirmed.**

## 3. Later commits on the U7 head

**`e5e1693ceb`** (I67) changes two files only: `apps/desktop/src/features/results/retainedPrecisionStanding.ts` and its `.test.tsx`.
- TypeScript is not compiled into PP's registered build, is not one of the 14 reviewed inputs, and is not embedded by D1 code. Pass B classes it `not-d1`.
- **No Pass B rerun is needed for it.**

**Slice L** (a 07j corpus entry and test plumbing) needs no rerun now, **provided all of the following hold:**
- **(a) The corpus file is not one of the 14 reviewed inputs and is not embedded by D1 production code.** `fixtures/results/retained_precision_cases.json` and `…_carrier_cases.json` meet this today. They are embedded only in `#[cfg(test)] mod u6e_reader_round_tests` and in result_export's integration tests.
- **(b) PP's `Cargo.toml` and `Cargo.lock` are unchanged.** The lock is a reviewed input, so any change to it, even a dev-dependency, makes the registered build **Stale**: every invocation is refused at D1.1 until a new registration is reviewed. That is not a test-only change.
- **(c) No hunk is added to the qualification's own test files** (`retained_memory_law_tests.rs`, `retained_memory_witness_tests.rs`, `tests/retained_memory_challenge.rs`; Pass B gives 6). **No `#[cfg(test)]` statement is added to production lines** (Pass B gives 5).
- **(d) No production `.rs` hunk is added in the D1 crates.**

**If (a)–(d) hold,** such commits cannot change the registered build's identity, inputs, layouts, profile, TEXT or D1 path, and **a mechanical rerun at U9's gate suffices.** If any fails, rerun Pass B on that head before relying on it.

## Execution record

- **Who.** RV89, TASK (Type 2) under ROOT. No descendants.
- **When.** 2026-10-04, about 12:55–13:05 MDT, within the 1.5-hour box.
- **Memory guard.** `memguard.sh` PID 5387 was running, and every cargo job checked it.
- **Cargo.** The default toolchain, `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2` (1 for the witnesses, the challenge and the probe), and `TMPDIR` in scratch. One cargo job at a time, with targets in WT/targets/rv89_u7/.
- **The probe and the flag.** The probe was mounted in my copy's lib.rs, and the flag set to `false`, only for the probe runs. Both lib.rs and retained_precision.rs were then restored from `cfda60403f` with `git archive`, and the copy re-checked equal to the tree.
- **Not run.** No Git writes or index operations (Git reads used `GIT_OPTIONAL_LOCKS=0`), no installs, no native, solver-at-scale or DEC-025 jobs, and nothing in the system temp directory. WT/f2a-u7 and I65's and I66's files were only read.
- **Writes.** Only this folder, WT/rv89_u7/, WT/targets/rv89_u7/ and WT/scratch/rv89_u4_g7_01/u7/. Machine paths in the evidence are replaced by `WT` and `R`.
- **Copies.** WT/rv89_u7 and WT/targets/rv89_u7 are deleted after this report.
- **Evidence** (`evidence/`): the probe and its output (`eligibility_alloc_probe.txt`); the run comparison, the entry gate and the delta inventory (`u7_run.txt`); the PP, runner/headless and law outcomes; and the run scripts.
