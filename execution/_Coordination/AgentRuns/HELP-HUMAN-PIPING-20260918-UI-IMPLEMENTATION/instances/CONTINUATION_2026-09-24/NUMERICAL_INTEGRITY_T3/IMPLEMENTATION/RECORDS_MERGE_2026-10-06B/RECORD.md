# Records-only PR #1103: squash-merged

**The merge.** [#1103](https://github.com/sgttomas/chirality/pull/1103) was squash-merged on 2026-10-06 at 23:19:13Z as `d8c88774d0`.
- It has a single parent, main `f8ed4f0551` (`_run_records/PARENTS.txt`).
- Command: `gh pr merge 1103 --squash --match-head-commit 3f8a405c33`, with an explicit subject and body (`_run_records/SQUASH_BODY.txt`), after confirming main was unmoved at `f8ed4f0551`.
- The squash commit's tree equals the PR head's.

**The content.** NUM's `projects/chirality-piping/execution/` at `ea0e288e8a`: 104 added, 2 modified, 0 deleted, nothing outside `execution/`.
- No product PR was open at the cut, so nothing was left out (A1-S-1).
- The squash body corrects the PR description's "RV97's addenda": only ADDENDUM_02 is added (RV103 N-6).

**Gates** (the records-only set):

| Gate | Result | Evidence |
|---|---|---|
| GEN-8 on the exact head | 1 passed (ROOT), and 1 passed (RV103, independently) | `_run_records/gen8.txt`; `R/REVIEW_RV103/records_01/` |
| The automatic CI on `3f8a405c33` | 4/4 succeeded | `_run_records/CI_RUNS_3f8a405c33.txt` |
| Independent review | RV103: PASS (0/1/6). S-1 and the notes are addressed on the integration branch and reach main with the next records PR | `R/REVIEW_RV103/records_01/` |

**Not in the PR:** RV103's review, I75's REPAIR_01, RV101's ADDENDUM_01, I79's return, this record, and the rulings after `ea0e288e8a`. They reach main with the next records PR.
