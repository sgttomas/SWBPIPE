# RV111 — addendum 02: SI1c's PR head (scope, package and equality)

TASK (Type 2) RV111, continued by ROOT, the return path. The dispatch is ROOT's message for PR #1112 (draft), RR "RV111 confirms SI1c's repair round; SI1c merges into NUM; its PR is prepared". I delegated nothing, read the worktree `WT/si1c-pr` only, and made no Git writes. Placeholders are as in `REVIEW.md`. `REVIEW.md`, `SHA256SUMS`, `ADDENDUM_01.md` and `SHA256SUMS.addendum_01` are untouched; this addendum and `evidence/addendum_02/` are sealed in `SHA256SUMS.addendum_02`.

## Result

**CONFIRMED, with one SHOULD-FIX in the package's wording.** BLOCKING 0, SHOULD-FIX 1, NOTE 2.

- **Scope and equality hold.** The PR head `b8bc059e356e240e0d5bda9f18d6c22f4bf12332` (pushed; one commit on main `e33f3e2f1b0a0271da3ef4db8521b3981257b5ce`) carries exactly the five slice files I reviewed and confirmed, at `f5665f8862`'s blobs, plus the four package files, at NUM `52c36a03d3`'s blobs.
- **No public meaning changes beyond the owner's option D and N-4 as ruled.**
- **The package's numbers are right.** But three of its sentences claim byte identity too broadly: they leave out N-4 (SF-1). I recommend fixing those three sentences before the merge, but the code is not affected. ROOT decides whether to re-cut.

| ID | Severity | Path | Evidence | Remedy |
|---|---|---|---|---|
| SF-1 | SHOULD-FIX | `T/IMPLEMENTATION/SI1C/PR_BODY.md` (the summary paragraph; "What stays the same", first bullet); `CHANGE_RECORD.md` §3, second bullet | **Three statements of byte identity leave out N-4:** (1) PR_BODY's summary: "Every other input gives byte-identical results." (2) PR_BODY: "Every input with no non-finite intermediate gives byte-identical results, in the evaluator and in `run_rule_checks`." (3) CHANGE_RECORD §3: "Interval mode is unchanged, byte for byte: `evaluate_interval` and `run_rule_checks_with_bounds` with b > 0." **A non-finite caller value or limit is not an intermediate, yet N-4 changes its runner bytes:** the finding, the note, and the removed `null`. It does so in point runs and, since inputs are resolved before the mode is chosen, in bounded runs too. My review measured this: bounded lines differ exactly where an N-4 value is present (`REVIEW.md` §4.3; 6,506 b = 1 and 5,002 b-relative lines). `evaluate_interval` itself is byte-identical. CHANGE_RECORD §3's first bullet and `REVIEW.md` already state it correctly ("no non-finite intermediate and no N-4 value"). | Qualify the three sentences. For example: (1) "Every other input gives byte-identical results, apart from N-4's naming of a non-finite caller value or limit." (2) "Every input with no non-finite intermediate and no non-finite caller value or limit gives byte-identical results …". (3) "Interval mode is unchanged: `evaluate_interval` byte for byte, and `run_rule_checks_with_bounds` with b > 0 except N-4's naming of a non-finite caller value or limit." RV111 confirms a re-cut by reading only the package diff. |
| N-1 | NOTE | `CHANGE_RECORD.md` §3, last bullet; `citations.json` `about` | **The "I87 §2.4" note resolves, but its gloss is wrong.** It calls §2.4 "the plan's consumer-row table". In `R/I87/si1c_plan_01/PLAN.md` (sha256 `0eb2459d…`, the same blob at the index's `num_commit` `769d0d0f46` and at NUM `52c36a03d3`), §2.3 is the consumer rows and §2.4 is "The comparison truth table (point path today)". The test comment it explains (`the_comparison_truth_table_is_unreachable`: "I87 §2.4: every row pairs a non-finite operand … with a comparison") cites §2.4 correctly. | With SF-1, if re-cut: "§2.4, the plan's comparison truth table". |
| N-2 | NOTE | `PR_BODY.md` "What changes"; `CHANGE_RECORD.md` §1, §2 and §4 | **Three small imprecisions, none affecting a count or a decision:** (a) "named …, never bound, and noted" fits a caller value. A limit carries no note: it is named, and its `limit_value` is omitted. (b) "9 new runner tests, 1 revised, 1 renamed" are one test, `a_nan_table_argument_check_blocks`, revised in round 0 and renamed in round 1. With `an_overflowing_ratio_check_blocks_and_the_run_carries_on`, that gives 2 + 9 = 11. (c) §4 says ADDENDUM_01 "used its own mutants" and then gives the differential's results. The mutants killed N06, N09 and the four note forms; the appended-note and interval results came from my harness. | Optional, with SF-1. |

## 1. Scope

- **`git diff --name-status main..PR`** is exactly the 5 slice files (modified) and the 4 package files (added) under `T/IMPLEMENTATION/SI1C/` (`evidence/addendum_02/scope_and_blobs.txt`).
- **Each slice blob at the PR head equals `f5665f8862`'s**, the head I confirmed in ADDENDUM_01: README `d31e63d0…`, `EE` `55a1f42b…`, `RCR` `39e37d4d…`, the runner test file `204676cc…`, `test_rule_interval.py` `febe5035…`. Main has not changed any of them since `025c1cf326`: their blobs at `e33f3e2f1b` equal those at `025c1cf326`, which is an ancestor of `e33f3e2f1b`.
- **Line counts:** +9/−2, +823/−139, +87/−4, +534/−11 and +23/−5. They equal CHANGE_RECORD §1's table, and its sha256 prefixes equal my ADDENDUM_01 copies.
- **Each package blob at the PR head equals NUM `52c36a03d3`'s.** `083e1a06e9` (SI1c merged into NUM) is an ancestor of `52c36a03d3`.

## 2. Equality: main's `source_equality.py`, re-run

- I used main's `T/IMPLEMENTATION/F2A_D1/source_equality.py` (the PR does not change it), run from a scratch copy with `--repo WT/si1c-pr --pr b8bc059e35… --int 52c36a03d3 --main e33f3e2f1b… --package T/IMPLEMENTATION/SI1C`.
- **Result: PASS, 5 of 5 checks** (`evidence/addendum_02/se.txt`, `se.json`). B = `e33f3e2f1b`, |S| = 5, 5 of 5 paths identical in blob and mode, no merge rule needed, 4 execution files all inside the package with their sums matching, and nothing unexplained.
- My output is byte-identical to ROOT's `se.txt`.

## 3. The package tells the truth

- **The package's `SHA256SUMS` verifies** on the PR's blobs: CHANGE_RECORD, PR_BODY and citations.json all OK.
- **`check_citations.py`** is main's, run from a scratch copy against main..PR, with the package's `citations.json` as the index. **Result: PASS**, with 0 parsed citations: 0 resolved, 0 ambiguous, 0 unresolved, 0 verification failures (`citations.txt`). This is the same result as ROOT's.
- **`citations.json` is coherent with the checker.** `citations: []` and `copies: {}`; `source_basis` is `f5665f8862`; `source_base` is main; `num_commit` `769d0d0f46` is on NUM's branch and an ancestor of `52c36a03d3`. The `about` text explains the one hand-stated reference. Its gloss is N-1.
- **The I87 §2.4 note resolves:** PLAN.md §2.4 exists at `num_commit` (heading at line 152), in the same blob as at NUM.
- **Numbers I checked against my records and I88's** (every one agrees):
  - **I88:** 226,336 point, 906,300 interval and 300,594 runner lines, with 0 violations (RETURN §0); 57 mutants at the head, 40 killed and 17 equivalent with 0 of 808,318 lines differing (REPAIR_01, of which 7 are carried from round 0, as REPAIR_01 states).
  - **RV111:** 88,155 flagged lines; 46,902 decided booleans and 21,868 finite quantities on main; 56,252 evaluator, 60,174 runner and 433,221 interval identical lines; N-4 on 35,304 lines; PASS 0/1/3; ADDENDUM_01 with no residual finding; N06, N09 and the four note forms killed by assertions.
  - **Suites:** `expression_evaluator` lib 57 → 65 ("7 new tests, 4 renamed and 1 split in two" is exact); `rule_check_runner` 35 → 44 (`point_path_non_finite_run` 2 → 11); `rule_pack_document` 10; pytest 193.
  - **PR_BODY "What stays the same":** "about 370,000 point-path evaluator lines" is 226,336 + 144,407 = 370,743. "400,000 runner lines" is 300,594 + 106,884 = 407,478, so "about 400,000" holds. "All 433,221 interval lines in the independent differential" is mine. The wording of that bullet is SF-1.
- **Also true:**
  - "No new finding code" and no grammar, corpus, schema, dependency, lock, src-tauri or desktop change (the fence);
  - "SI1b's ratio block is kept";
  - the `NaN ≠ 100` and `inf ≥ 100` examples (I79's reproducers);
  - "these are agent reviews, not personal review by the owner";
  - N-3 routed to S-I2 planning (CHANGE_RECORD §6), as a carry-forward.
- **No machine path, personal name or address** appears in the package.

## 4. No change to public meaning beyond option D and N-4

- The slice is byte-identical to the head I reviewed and confirmed.
- **Its behaviour changes are exactly D:** a block at the producer, so point-path checks over a non-finite intermediate read `RULE_INPUTS_INCOMPLETE` instead of a decided result. The owner decided this as a repair within grammar 1.0.0.
- **And N-4 (N4-1):** findings, notes and the removed `null`, with no status, diagnostic or relation change, including ruling 1's unsupplied raw value in another unit and ruling 2's appended note.
- Comments and the README follow the code. No other output changes (`REVIEW.md` §4.2–§4.5; ADDENDUM_01 §6).

## 5. Host

- **No cargo and no heavy job.** The two gate scripts are standard-library Python and Git reads (seconds each), run from scratch copies, writing only to `WT/scratch/rv111_si1c_01/a2/`.
- Git reads used `GIT_OPTIONAL_LOCKS=0`. No Git writes and no DEC-025.
- Absolute paths only. No wait was started, and none of mine is running.
