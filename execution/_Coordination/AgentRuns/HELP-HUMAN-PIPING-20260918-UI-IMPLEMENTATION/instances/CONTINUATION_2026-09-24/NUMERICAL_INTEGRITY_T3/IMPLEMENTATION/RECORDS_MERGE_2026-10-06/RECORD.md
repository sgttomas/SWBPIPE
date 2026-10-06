# Records-only PR #1101: squash-merged

**The merge.** [#1101](https://github.com/sgttomas/chirality/pull/1101) was squash-merged on 2026-10-06 at 15:38:38Z as `d069e7130c`, a single parent on main `75a8c3291f` (`_run_records/PARENTS.txt`). Command: `gh pr merge 1101 --squash --match-head-commit 93d15f1bd3`, with an explicit subject and body, after confirming main was unmoved at `75a8c3291f`.

**The content.** NUM's `projects/chirality-piping/execution/` at `1720a5c06b`, without `IMPLEMENTATION/U8/`, which U8's PR #1102 carries (A1-S-1): 758 added, 3 modified, 0 deleted, nothing outside `execution/`.

**The heads:**
- **H `11b2d04f13`:** NUM `4e6c2fcbfe`;
- **H2 `e41566921d`:** H minus `IMPLEMENTATION/U8/`;
- **H3 `93d15f1bd3`:** NUM `1720a5c06b` minus `IMPLEMENTATION/U8/`, which adds RV102's S-1 and S-2 fixes. This is the merged head.

**Gates** (the records-only set):

| Gate | Result | Evidence |
|---|---|---|
| GEN-8 on the exact head | 1 passed, on H, H2 and H3 | `_run_records/gen8.txt`, `gen8_2.txt`, `gen8_3.txt` |
| The automatic CI on H3 | 4/4 succeeded | `_run_records/CI_RUNS_H3.txt` |
| Independent review | RV102: PASS at H2 (0/2/6), with S-1 (four RV98 evidence files ignored by `build/`) and S-2 (the stale work-graph T3 section) fixed in H3. ADDENDUM_01: PASS at H3 (0/0/4) | `R/REVIEW_RV102/records_01/` |

**Not in the PR:** RV102's ADDENDUM_01, this record and later rulings. They reach main with the next records PR.
