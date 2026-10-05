# I67 return: the U6d repair round (02)

I67 is a TASK (Type 2) under ROOT, working to ROOT's repair-round message and its two additions. The rulings are RR "RV91 on U6d: PASS; the U6d repair round granted; U7 preconditions added" and "RV88 on U6a, U6c, U6b (and U6d)…". The findings come from `R/REVIEW_RV91/u6d_01/REVIEW.md` and `R/REVIEW_RV88/u6d_01/REVIEW.md`. I did not delegate.

**Verdict: every item is done, and every control passes.**
- **Suite:** Vitest gives 3,431/3,431, and `tsc` is clean.
- **Existing tests:** all 3,417 at `9555b6ffc2` keep their outcome, and the 14 shared parity cases are unchanged.
- **Sweep:** all 63 existing-identity envelopes are identical to base.
- **Mutants:** 113 of 113 are killed, all by assertion. That is all of round 01's 103 plus 10 new, and it includes RV91's RV03, RV04, RV08 and RV09.

**One item waits on I66.** The shared `declared_differences` section has not landed. So TS keeps its local F1 pin, and you sequence the switch.

## Basis, host and fence

- **Worktree:** `WT/f2a-carriers-ts`, on `9555b6ffc2` (U6d, with the S-1 pin applied). The work is uncommitted, and I made no Git writes; Git reads used `GIT_OPTIONAL_LOCKS=0`.
- **When:** 2026-10-04, about 11:25Z to 11:46Z. The memory guard (PID 5387) ran throughout.
- **Runtime:** unchanged from round 01 (`_run_records/runtime.txt`).
  - `P/node_modules` is a symlink to REPO_ROOT's. It still shows as `??`, so do not stage it.
  - The WASM assets are copied, not built.
  - No install, Cargo, native or DEC-025 job was run.
- **Lanes:**
  - `base2` is a `git archive` of `9555b6ffc2`;
  - `cand2` (the sweep) and `mut2` (the mutants) are the candidate.
  
  Both were synced before a final, comment-only edit (`sweep_lane_vs_final_candidate.txt`). Every mutant's text was rechecked: each still matches exactly once on the final candidate.
- **The fence:** 4 files, all already U6d's. Two are product files and two are its own tests.

| File (TS = P/apps/desktop/src) | sha256 | Change |
|---|---|---|
| TS/services/analysisRunCompatibility.ts | `7730e3c0…` | SF-1 |
| TS/features/results/retainedPrecisionStanding.ts | `93f47e1b…` | N-4 / RV88 N-3, plus one doc-comment sentence (RV88 N-2) |
| TS/features/results/retainedPrecisionIntegration.test.tsx | `a8bd9295…` | 9 new tests |
| TS/services/retainedPrecisionAnalysisRun.test.ts | `dfe77965…` | 5 new tests |

The diff is `_run_records/candidate.diff`, and the hashes are in `changed_files_sha256.txt`.

## The items

### 1. SF-1: the historical v0.2 builder never drops a receipt

- **The change:** `buildAnalysisRunV02` keeps its existing first refusal (`HISTORICAL_ANALYSIS_SOURCE_UNSUPPORTED`) unchanged. It then refuses `retainedPrecisionDowngrade(result)` with `ANALYSIS_LEGACY_SOURCE_DOWNGRADE_FORBIDDEN`. That is the same guard as dispatch, so it covers a `retained_precision` member, whether an object or null, and any W1 token row.
- **The code is Python's.** It is already in `compatibility.py` on `origin/main`, and U6b extended it to `retained_precision`.
- **Tests** (`retainedPrecisionAnalysisRun.test.ts`, "the historical v0.2 builder never drops a receipt"):
  - a legacy-shaped control still builds a v0.2 record without the member;
  - each of the three forms is refused with the twin code;
  - a source with precision metadata keeps its existing refusal.
- **Mutants:** V01 (the guard removed), V02 (member only), V03 (truthy member only) and V04 (the wrong code) are all killed.

### 2. N-3 (RV91): tests that kill RV08 and RV09, plus RV03 and RV04

- **RV08** (the successor selected by route rather than producer id in `ruleBindingRefusal`):
  - A test in "the successor's own header requires…" now asserts that a header-broken successor-id envelope refuses its rows `RULE_QUANTITY_NOT_COVERED` under each of the nine header edits. This is Rust's producer-id selection, and it fails closed.
  - RV08 is killed.
- **RV09** (the registration fingerprint taken after the reader's await), adapting RV91's probe in both modes:
  - a row is edited while `registerRetainedPrecision` awaits;
  - the reader validates its synchronous snapshot, and the edited object reads unregistered (`VALIDATION_REQUIRED`);
  - the exactly restored bytes read the validation.
  - RV09 is killed. The restore uses the original value; `+1−1` is not exact for the dense fixture's tiny values.
- **RV03 and RV04** (optional, done; both modes, under the test-only post-U7 wrapper):
  - mutating the caller's invocation after registration does not change `retainedPrecisionInvocation`;
  - mutating a returned copy does not change the next one.
  - Both are killed.

### 3. N-4 (RV91), with RV88's N-3: the standing text

- **The case count is shown only from a validated registration.** An unvalidated receipt reads "…receipt not validated…; needs recompute.", and a refused one reads "…refused by the retained-precision reader (code); unsupported, values shown for inspection only.". Neither carries "Selected cases".
- **RV88's N-3, folded in:** a delivery refused in this session no longer says "historical values only". The new wording is RV88's suggested text.
- **Tests:**
  - the milestone reads "Selected cases: 1 of 1." only when registered; a copy and a refused delivery carry no count;
  - a valid two-case statement from the shared reader corpus (`two_case_facade_after_certificate_synthetic`: selected, unavailable) carries no count while unregistered, and reads "…Selected cases: 1 of 2." after `registerRetainedPrecision` with its invocation;
  - that case keeps its statuses and expectations in 07h, which is on the carriers branch.
- **Mutants:** T01 and T02 (the count leaking into the unvalidated or refused text) are killed, and so are S29–S31 (rewritten for the new structure).

### 4. The F1 pin (TS-local, pending I66's shared section)

- **The test:** "the declared parity difference F1 (pinned in TS only)", in both modes. Its comment cites I67 F1, RV91 N-1 and the RR ruling.
- **What it asserts,** for an unregistered successor with an edited covered row, or with `numerical_quality` rewritten to `checks_passed`:
  - the reader refuses it (`RETAINED_PRECISION_RECEIPT_MISMATCH`);
  - TS standing is `needs_recompute`, with `RETAINED_PRECISION_VALIDATION_REQUIRED` and `eligible: false`, where Rust and Python read `unsupported`;
  - binding refuses (`RULE_QUANTITY_NOT_COVERED`).
- **The shared section** (ROOT's revised ruling) is not in the carriers branch as committed (`924c6284cb` has no `declared_differences`). As instructed, the TS-local pin stays. When I66's section lands, the TS consumer is a small test change that reads its TS expectations for F1 and F2.

### Also noted

- **RV88's N-2, in a doc comment:** reordering a member's keys in place voids the registration. This is comment-only.
- **I66's SI-unit wording (U6a S-2):** not yet received.
  - TS's `N_RP_ABSOLUTE` label already names the SI unit explicitly ("±b m", "±b Pa", …).
  - The standing text names no bound.
  - I will align the wording when you pass the exact text.
- **RV91's N-2, RV88's S-1 and RV91's N-5:** these are U7 preconditions, so they are not in this round.

## Controls

| Run | Result |
|---|---|
| Base `9555b6ffc2` (lane) | Vitest 3,417/3,417; `tsc` 0 |
| Candidate (worktree, final) | Vitest **3,431/3,431**; `tsc` 0 |

The candidate's tests are the 3,417 existing ones, all with an unchanged outcome (`compare_base_vs_candidate.txt`), plus 14 new: 9 in the integration test file and 5 in the AnalysisRun test file.

- **The 14 shared parity cases:** unchanged, and passing.
- **The sweep** (round 01's sweep, base `9555b6ffc2` against the candidate; `sweep_compare.txt`): 80 envelopes. All 63 existing-identity envelopes are identical, including the v0.2 build outcome, and so are all 17 successors. None carries a receipt on a legacy shape.
- **Mutants** (`mutants_r2.py`, `.json`, `.log`; the mutant lane against the 3 U6d test files and 8 related existing ones):
  - **113 of 113 killed, all by assertion.** The control passes 310/310.
  - That is round 01's 103 (S29 rewritten), plus the new T01, T02, V01–V04, RV03, RV04, RV08 and RV09.

## For ROOT

- **The `declared_differences` switch:** sequence it once I66's section lands.
- **I66's SI-unit text:** please pass the exact wording.
- **Commit:** stage the 4 files only.

## Records

`_run_records/` holds:
- basis and runtime;
- the diff and the changed-file hashes;
- the run scripts;
- base and candidate outcomes, exit codes and `tsc`;
- the outcome comparison;
- the sweep TSVs, logs and comparison, and the lane note;
- the mutant programme, results and log.

All paths in the records are placeholders. SHA256SUMS covers this folder.
