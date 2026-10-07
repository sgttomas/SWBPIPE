# Records-only PR #1108: squash-merged

**The merge.** [#1108](https://github.com/sgttomas/chirality/pull/1108) was squash-merged on 2026-10-07 at 14:42:26Z as `4f37590bfb`.
- It has a single parent, main `2007709549` (`_run_records/PARENTS.txt`).
- **Command:** `gh pr merge 1108 --squash --match-head-commit ea3b1443ea`, with an explicit subject and body (`_run_records/SQUASH_BODY.txt`), after confirming main was unmoved and the PR was CLEAN.
- The squash commit's tree equals the PR head's.

**The content.** NUM's `projects/chirality-piping/execution/` at `6f983f12f3`: 869 added, 2 modified (the T3 rulings and the work graph), 0 deleted, nothing outside `execution/`.
- **Two commits:**
  - H1 `145443e9e4` carried NUM `25c745f905`;
  - H2 `ea3b1443ea` added the delta to `6f983f12f3` after RV110's S-1.
- **S-1 (erratum E-7):** four of I85's sealed files, hidden by `build/`, were force-added.
- **No product PR was open at the cut.**
- **The squash body** corrects the PR description's content list (RV110 N-4 and ADDENDUM_01 A1-N-1).

**Gates** (the records-only set):

| Gate | Result | Evidence |
|---|---|---|
| GEN-8 on each head | H1: 1 passed; H2: 1 passed (ROOT), and 1 passed (RV110, independently) | `_run_records/gen8.txt`; `R/REVIEW_RV110/records_01/` |
| The automatic CI on H2 `ea3b1443ea` | 4/4 succeeded | `_run_records/CI_RUNS.txt` |
| Independent review | RV110: PASS (0/1/4); S-1 repaired; ADDENDUM_01 CONFIRMED the delta (0/0/3) | `R/REVIEW_RV110/records_01/` |

**Not in the PR:**
- RV110's ADDENDUM_01, this record, and the rulings after `6f983f12f3`;
- the records of the agents still running (I85 SP, I89 SA, I90 SR-RS, RV111 SI1c).

They reach main with the next records PR.

**Host note (RV110 A1-N-3):** an empty untracked `ee_code_nontest.diff` appeared at `WT/records-pr-b`'s root (08:27 local), in no commit. Nothing had it open, and ROOT removed it before the squash. Its name matches RV111's comments-only check, written by relative path while ROOT's working directory was that worktree.
