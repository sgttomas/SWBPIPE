# I67 return: U7 slice F follow-on (the summary's `withheld` follows TS's standing)

I67 is a TASK (Type 2) under ROOT. I worked to ROOT's message on `cfda60403f`, ruling 4 on `u7_slice_f_01` §7.4. I did not delegate.

**Verdict: done. The fix fails closed, and every control passes.**
- **Fix:** the summary's `withheld` now counts as Current only when TS's standing is eligible, live native capture included.
- **Test:** a new test pins the not-Current count on both D-U7-4 forms in both modes.
- **Mutant:** the mutant that removes the condition is killed, by exactly those four tests.
- **Suites:** 3,541 of 3,541 pass, and `tsc` is clean. No existing outcome changed.

## Basis, host and fence

- **Worktree:** `WT/f2a-u7`, at `cfda60403f` on `codex/piping-f2a-u7-20261004`. The follow-on is uncommitted, for ROOT to commit.
- **Git:** no Git writes. Reads used `GIT_OPTIONAL_LOCKS=0`.
- **When:** 2026-10-04, 18:45Z to 18:50Z. The memory guard (PID 5387) ran throughout.
- **Host:** only Vitest and `tsc` were run.
  - No install, build, Cargo or Python.
  - `TMPDIR` was set to `WT/scratch/i67_u6d/tmp`.
  - Nothing native or DEC-025.
- **Fence:** TS only, 2 files.

| File (TS = P/apps/desktop/src) | sha256 | Change |
|---|---|---|
| TS/features/results/retainedPrecisionStanding.ts | `b22e8460…` | `classificationSummary` passes `requestedRefs(model)` only when `retainedPrecisionStanding(source, model).eligible`, and `[]` otherwise; the docstring says so (+4 −2) |
| TS/features/results/retainedPrecisionIntegration.test.tsx | `77dea429…` | One `it.each` over D-U7-4's forms × fixtures (4 tests; +12) |

The fix:
```ts
return validation ? classificationSummaryFrom(validation, source!, retainedPrecisionStanding(source!, model).eligible ? requestedRefs(model) : []) : [];
```

`classificationSummaryFrom` (the Rust-parity seam) is unchanged. Empty requested refs give the not-Current count by D2 4.9.4.

## The test

The test runs on each D-U7-4 form (`invocation_without_native_capture`, `stale_current_model_same_case_ids`), in both modes, delivered through `applyShared`. It asserts:
- TS's standing finding is `NATIVE_CAPTURE_REQUIRED`;
- D2 4.9.4 alone, the seam with the model's requested refs, would read Current: `withheld` 69;
- the not-Current count is 97;
- `classificationSummary(received, model)` equals the not-Current summary.

## Controls

**Suites** (`_run_records/`):
- **The base** is lane `cand7`. Its 18 slice F files hash-equal `cfda60403f`'s (`lane_base_cand7_vs_cfda60403f.txt`, 18 of 18). Its run is `r7_final`: 3,537 of 3,537, `tsc` clean.
- **The follow-on** is lane `cand8`, which is `cand7` plus the 2 files. Its run is `r8_final`: 3,541 of 3,541, `tsc` clean.
- **Comparison** (`compare_cfda60403f_vs_followon.txt`): no existing test changed outcome. The only additions are the 4 new tests, which pass.

**Mutants** (lane `mut7` = the worktree, `mutants_f2.*`; the 4 retained and 9 related TS test files). All 3 are killed by assertion:

| Id | Mutation | Result |
|---|---|---|
| W01 | **The condition removed** (ROOT's mutant) | Killed, by exactly the 4 new tests |
| W02 | The condition replaced by D2 4.9.4 alone (`retainedStandingFrom`), without the live capture | Killed, by the same 4 tests |
| W03 | Never Current (`[]` always) | Killed, by 6 existing tests, including the post-U7 path's `withheld` 69 |

**The TS oracle dump** was rerun on `cand8`, over the same 380 inputs:
- Against `u7_slice_f_01`'s candidate dump (`oracle_dump_vs_f01.txt`), only the 4 D-U7-4 inputs' `ipc` summary changes: `withheld` goes from 69 to 97, which equals the pre-U7 base.
- Slice F's oracle comparison (`ts_oracle_diff_f02.txt`) passes, now with the expectation that the `ipc` summary changes on exactly the `ipc` token set (4).
- Every other field and value is unchanged, and it still equals the oracle.

Records are in `_run_records/`, with placeholder paths only; a leak grep found nothing.
