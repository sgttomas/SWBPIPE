# Records-only PR #1105: squash-merged

**The merge.** [#1105](https://github.com/sgttomas/chirality/pull/1105) was squash-merged on 2026-10-07 at 00:42:00Z as `47a3bdfcf5`.
- It has a single parent, main `bfb26596bf` (`_run_records/PARENTS.txt`).
- Command: `gh pr merge 1105 --squash --match-head-commit 736f3fb7b2`, with an explicit subject and body (`_run_records/SQUASH_BODY.txt`), after confirming main was unmoved.
- The squash commit's tree equals the PR head's.

**The content.** NUM's `projects/chirality-piping/execution/` at `030020aca3`: 218 added, 2 modified, 0 deleted, nothing outside `execution/`.
- No product PR was open at the cut.
- The squash body names the `SESSION_2026-10-06/SHA256SUMS.dec025_mac` seal, which the PR description had omitted (RV106 N-5).

**Gates** (the records-only set):

| Gate | Result | Evidence |
|---|---|---|
| GEN-8 on the exact head | 1 passed (ROOT), and 1 passed (RV106, independently) | `_run_records/gen8.txt`; `R/REVIEW_RV106/records_01/` |
| The automatic CI on `736f3fb7b2` | 4/4 succeeded | `_run_records/CI_RUNS_736f3fb7b2.txt` |
| Independent review | RV106: PASS (0/0/5) | `R/REVIEW_RV106/records_01/` |

**Not in the PR:** RV104's review, I79's REPAIR_01, I81's probe, RV106's review, SI1b's draft package, this record, and the rulings after `030020aca3`. They reach main with the next records PR.
