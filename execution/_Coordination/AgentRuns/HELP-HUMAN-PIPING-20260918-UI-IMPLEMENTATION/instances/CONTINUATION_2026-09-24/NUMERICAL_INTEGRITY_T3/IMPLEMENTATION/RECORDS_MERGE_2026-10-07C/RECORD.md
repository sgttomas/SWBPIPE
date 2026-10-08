# Records-only PR #1111: squash-merged

**The merge.** [#1111](https://github.com/sgttomas/chirality/pull/1111) was squash-merged on 2026-10-07 at 23:00:19Z as `54f1ba1f6d`.
- It has a single parent, main `e33f3e2f1b`, which had not moved since the cut (`_run_records/PARENTS.txt`).
- **Command:** `gh pr merge 1111 --squash --match-head-commit 18a20d329f`, with an explicit subject and body (`_run_records/SQUASH_BODY.txt`).
- The squash commit's tree equals the PR head's.

**The content.** NUM's `projects/chirality-piping/execution/` at `0b8299e496`: 1,043 added, 2 modified (the T3 rulings and the work graph), 0 deleted, nothing outside `execution/`.
- **One commit,** H `18a20d329f`. An earlier staging at NUM `00658c76c1` was amended before the push, to carry RV111's ADDENDUM_01.
- **The cut followed NUM's absorb of main** with #1109's RV58 fixture repair (RR "NUM absorbs main with #1109's RV58 fixture repair; …"). So the PR adds no symlink, and it leaves `R/REVIEW_RV58/` as main has it.
- **#1112 (SI1c) was open at the cut.** This PR shares no path with it and leaves out its package, `IMPLEMENTATION/SI1C/`.
- **The squash body corrects the PR description** (RV117 N-3): I88's SI1c return and RV109's confirmation of ST's repair were already on main.

**Gates** (the records-only set):

| Gate | Result | Evidence |
|---|---|---|
| GEN-8 on the head | 1 passed (ROOT), and 1 passed (RV117, independently) | `_run_records/gen8.txt`; `R/REVIEW_RV117/records_01/` |
| `validate_run_record_leaks.py` (main's, since #1109) | PASS: 1,044 files, 0 credentials, 0 machine-local symlinks (ROOT and RV117) | `_run_records/leaks.txt` |
| The automatic CI on H `18a20d329f` | 4/4 succeeded | `_run_records/CI_RUNS.txt` |
| Independent review | RV117: PASS (0/0/5) | `R/REVIEW_RV117/records_01/` |

**Not in the PR:**
- RV109's round 2 on SP;
- RV111's ADDENDUM_02 to ADDENDUM_04;
- RV117's review;
- this record;
- the rulings after `0b8299e496`;
- the records of the agents still running (RV113 on SR-TS, SR-RS's repair and SR-PY; I97's B2-C).

They reach main with the next records PR.
