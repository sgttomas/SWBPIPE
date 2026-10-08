# RV111 — addendum 03: #1112's package re-cut

TASK (Type 2) RV111, continued by ROOT, the return path. I delegated nothing, read `WT/si1c-pr` and NUM only, and made no Git writes. Placeholders are as in `REVIEW.md`. My earlier sealed files are untouched; this addendum and `evidence/addendum_03/` are sealed in `SHA256SUMS.addendum_03`.

## Result

**CONFIRMED.** PR head `2881cb1969f0aa7b670b9185c453699b5f5b80d7` (pushed, draft) is one commit over `b8bc059e35`, the head of ADDENDUM_02. It changes only the four package files, and each change does what ADDENDUM_02 asked. **One residual NOTE** (R-1 below), which applies equally to my own suggested wording; it changes no count and no decision.

## 1. The delta is the package only

- `git diff --name-status b8bc059e35 2881cb1969` lists exactly `T/IMPLEMENTATION/SI1C/{CHANGE_RECORD.md, PR_BODY.md, SHA256SUMS, citations.json}`, all modified. No slice file changes, and the PR's diff against main is still those 9 files.
- **Each package blob equals NUM `73e8087df7`'s.** `52c36a03d3` is an ancestor of `73e8087df7`, and NUM changes nothing under `P/core` or `P/tests` between them.
- **The package's `SHA256SUMS` verifies** on the new blobs (3/3 OK).
- **Main's `source_equality.py`** (re-run from my scratch copy, `--int 73e8087df7 --package …`): PASS, 5/5, |S| = 5, 4 execution files all inside the package.
- **Main's `check_citations.py`:** PASS, 0 parsed citations, 0 unresolved.
- **`citations.json`** still pins `num_commit` `769d0d0f46`, `source_basis` `f5665f8862`, and empty `citations` and `copies`. Only its `about` gloss changed.

## 2. Each change against ADDENDUM_02 (word diff: `evidence/addendum_03/package_word_diff.txt`)

- **SF-1, all three sentences now include N-4:**
  - **PR_BODY's summary** adds "It also names a non-finite caller value or limit in the runner (N-4)." before "Every other input gives byte-identical results", so "other" now excludes N-4.
  - **PR_BODY's first "What stays the same" bullet** now reads "no non-finite intermediate and no non-finite caller value or limit". The next bullet reads "The interval evaluator is unchanged …" with "Bounded rule checks change only where N-4 applies."
  - **CHANGE_RECORD §3** now reads "no non-finite caller value or limit (N-4)", and "The interval evaluator is unchanged, byte for byte (`evaluate_interval`). `run_rule_checks_with_bounds` with b > 0 is byte-identical except where N-4 applies, as in point runs."
  - **CHANGE_RECORD §2 and PR_BODY's runner bullet** add that a non-finite limit is named with the slot as subject, "in point and bounded runs alike".
- **N-1:** "the plan's comparison truth table", in CHANGE_RECORD §3 and `citations.json`'s `about`. §2.4 is that table at `num_commit` (heading at line 152).
- **N-2:**
  - (a) a limit is "named", and only an input is "noted": CHANGE_RECORD §1 and §2, PR_BODY.
  - (b) "9 new runner tests, and 1 renamed with its expectation revised (2 → 11)": 2 + 9 = 11, and the renamed test is `a_table_check_over_an_overflowing_argument_blocks_at_the_multiply`.
  - (c) CHANGE_RECORD §4 now separates the two: "Its mutants are killed by assertions. Its harness found that every changed runner line differs only by an appended note, and that the interval evaluation lines are identical to main's." That matches ADDENDUM_01 §1, §2 and §6.
- **Nothing else changed.** The word diff contains only these edits, plus "and 433,221 interval evaluation lines. The bounded runner lines differ only where an N-4 value is present", which is `REVIEW.md` §4.3 and is true. Every number is unchanged and still agrees with my records and I88's (ADDENDUM_02 §3). Option D, N-4 and the rulings are stated as before, so no public meaning changed.
- **The GitHub description** equals the new `PR_BODY.md` without its title line, apart from one leading and one trailing blank line (`gh pr view 1112`, a read).

## 3. Residual NOTE

**R-1 (NOTE; my own suggested wording has the same gap).** "`run_rule_checks_with_bounds` with b > 0 is byte-identical except where N-4 applies" (CHANGE_RECORD §3) and "Bounded rule checks change only where N-4 applies" (PR_BODY) are true of every check that binds a bound, which runs interval mode. A bounded run can also contain a check that binds no bound. That check follows the point path, so D applies to it as in a plain run: I88's differential found 1,513 such bounded lines, from RV104's user-input table checks (RETURN §4.2, condition 4). "As in point runs" can be read to cover this, but it is not explicit.

Optional, if the package is ever re-cut: "… except where N-4 applies, and, for a check that binds no bound, where D applies as in a plain run". I do not recommend a re-cut for this alone.

## 4. Host

- No cargo and no heavy job. The two gate scripts ran from my scratch copies, writing only to `WT/scratch/rv111_si1c_01/a3/`. One `gh pr view` read.
- Git reads used `GIT_OPTIONAL_LOCKS=0`. No Git writes and no DEC-025. Absolute paths only.
- No wait is running.
