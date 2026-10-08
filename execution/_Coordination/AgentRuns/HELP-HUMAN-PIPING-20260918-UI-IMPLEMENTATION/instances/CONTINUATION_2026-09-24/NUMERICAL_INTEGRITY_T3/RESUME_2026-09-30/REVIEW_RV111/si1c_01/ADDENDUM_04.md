# RV111 — addendum 04: #1112's second package re-cut (ADDENDUM_03 R-1)

TASK (Type 2) RV111, continued by ROOT, the return path. I delegated nothing, read `WT/si1c-pr` and NUM only, and made no Git writes. Placeholders are as in `REVIEW.md`. My earlier sealed files are untouched; this addendum and `evidence/addendum_04/` are sealed in `SHA256SUMS.addendum_04`.

## Result

**CONFIRMED, with one residual NOTE (R-2).** PR head `13d02273f48f22b97d7e4bace87b4189d7f03594` (pushed) is one commit over `2881cb1969`.

- **The delta is three files only:** `T/IMPLEMENTATION/SI1C/{CHANGE_RECORD.md, PR_BODY.md, SHA256SUMS}`.
  - Each equals NUM `38e01032c7`'s blob, and `73e8087df7` is an ancestor of `38e01032c7`.
  - `citations.json` and the five slice files are unchanged, and the PR's diff against main is still the same 9 files.
  - The package's `SHA256SUMS` verifies (3/3).
  - Main's `source_equality.py` (5/5, `--int 38e01032c7`) and `check_citations.py` pass, re-run from my scratch copies.
- **Nothing else changed.** The word diff (`evidence/addendum_04/package_word_diff.txt`) holds only the two sentences below.
- **The GitHub description** equals the new `PR_BODY.md` without its title line, apart from blank lines at either end.

## The two sentences

- **CHANGE_RECORD §3: exact.** "In `run_rule_checks_with_bounds` with b > 0, a check that binds a bound is byte-identical except where N-4 applies. A check that binds no bound follows the point path, so D applies to it as in point runs (RV111 ADDENDUM_03 R-1; I88 counted 1,513 such lines)."
  - A check whose formula binds an input with b > 0 runs interval mode, which is unchanged except for N-4's naming before the mode is chosen.
  - A check that binds no bound runs the point path.
  - The 1,513 is I88's RETURN §4.2, condition 4.
- **PR_BODY: exact for a nonzero bound.** "In a bounded rule run, a check that binds a solver bound reads in interval mode and changes only where N-4 applies. A check that binds no bound reads on the point path, so option D applies to it as above."

**R-2 (NOTE).** A solver bound of exactly zero binds the exact point. That is `SolverResultBound`'s and D2 §4.11.2's rule, and it is also why plain equals b = 0. Such a check reads on the point path, so D applies to it, not "interval mode" as the PR_BODY sentence says. CHANGE_RECORD's "with b > 0" covers this; PR_BODY does not.

An invalid or duplicate bound leaves the input unsupplied, so completeness blocks and nothing changes; that case is consistent with the sentence.

If the package is re-cut for any other reason: "a check that binds a nonzero solver bound reads in interval mode …". I do not recommend a re-cut for this alone; my own ADDENDUM_03 wording had the same gap.

## Host

- No cargo and no heavy job. The two gate scripts ran from my scratch copies, writing only to `WT/scratch/rv111_si1c_01/a4/`. One `gh pr view` read.
- Git reads used `GIT_OPTIONAL_LOCKS=0`. No Git writes and no DEC-025. Absolute paths only.
- No wait is running.
