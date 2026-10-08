# T3-SI1c merged: #1112 at `13d02273f4`

**The merge.** [#1112](https://github.com/sgttomas/chirality/pull/1112) merged on 2026-10-07 at 23:27:14Z, as merge commit `0b6c5d7362`.
- Parents: main `54f1ba1f6d` and the PR head `13d02273f4` (`_run_records/PARENTS.txt`).
- Command: `gh pr merge 1112 --merge --match-head-commit 13d02273f48f22b97d7e4bace87b4189d7f03594` (`_run_records/MERGE_COMMAND.txt`).
- Main's diff is the slice's 5 maintained files plus the 4 package files under `IMPLEMENTATION/SI1C/`: 9 files, +1,788 / −161.
- The PR was cut from main `e33f3e2f1b`. Main then moved to `54f1ba1f6d` (#1111, records only, sharing no path with the PR). Main does not require an up-to-date branch, and the PR was CLEAN at the merge.

**The PR's heads.** It was cut at `b8bc059e35`: one commit on main `e33f3e2f1b`, with the 5 slice files from SI1c's branch at `f5665f8862` and the 4 package files from NUM (`_run_records/PR_CUT.txt`). Two package-only commits followed, each correcting the package's wording at RV111's request; no slice file changed.
- **`2881cb1969`** (RV111 ADDENDUM_02, SF-1): N-4 changes the runner's bytes for a non-finite caller value or limit, in point and bounded runs. With its N-1 and N-2.
- **`13d02273f4`** (RV111 ADDENDUM_03, R-1): in a bounded run, a check that binds no bound follows the point path, so D applies to it (I88 counted 1,513 such lines).
- **RV111 ADDENDUM_04, R-2 (NOTE, no re-cut):** PR_BODY's "a check that binds a solver bound reads in interval mode" holds for a nonzero bound. A bound of exactly zero binds the exact point, so that check reads on the point path. CHANGE_RECORD §3's "with b > 0" is exact. **This record corrects PR_BODY accordingly.**

## Gates

| Gate | Result | Evidence |
|---|---|---|
| Independent complete-diff review | RV111: PASS (0/1/3). The repair round was CONFIRMED with no residual (ADDENDUM_01). The PR head was CONFIRMED with one package SHOULD-FIX (ADDENDUM_02), and both re-cuts were CONFIRMED (ADDENDUM_03 and ADDENDUM_04) | `R/REVIEW_RV111/si1c_01/` |
| `source_equality.py` | 5/5 PASS at each head, with \|S\| = 5, against main `e33f3e2f1b`. At `13d02273f4` it ran against NUM `38e01032c7`. RV111 reproduced it at each head | `_run_records/se*.txt`, `se*.json` |
| `check_citations.py` with the package index | PASS: 0 parsed citations. The package states "I87 §2.4" (the comparison truth table) by hand | `_run_records/citations*.txt` |
| `validate_run_record_leaks.py` (main's) | PASS at each head: 4 files, 0 credentials, 0 machine-local symlinks | `_run_records/leaks*.txt` |
| GEN-8 | 1 passed at each head | `_run_records/gen8*.txt` |
| Hosted CI on `13d02273f4` | All automatic runs succeeded. The full-SHA dispatch 37697773464 (`target_base` = main `e33f3e2f1b`) succeeded. The first dispatch, 37696852261 on `b8bc059e35`, succeeded. The second, 37697481281 on `2881cb1969`, was cancelled when the third superseded it | `_run_records/CI_RUNS_13d02273f4.txt` |
| The full 40-manifest suite before the freeze | Satisfied by the exact-head DEC-025's 40 manifests, as for S-I1, T6S, SI1b and B6 | `dec025/` |
| Exact-head Mac DEC-025 against a fresh main baseline | **`ALL-DONE`** on `b8bc059e35`, carried over to `13d02273f4`. Baseline: `WT/main-baseline` at main `e33f3e2f1b`. Both use fresh targets | `dec025/` |
| Pass B | Not applicable: the rules crates are outside PP's closure | CHANGE_RECORD §3 |

**DEC-025's carry-over.** Both later commits change only `IMPLEMENTATION/SI1C/`, which no DEC-025 suite reads (RR "RV99 confirms #1100's amended head and the DEC-025 carry-over"). Main moved to `54f1ba1f6d` (#1111, records only) after the cut. That changed only `P/execution/`, which the same rule covers.

## DEC-025 in detail (`dec025/`)

- **The sweep** stopped at PP's known Mac `t13` (`s11g_tests::t13_committed_fallback_uz_is_byte_identical`).
- **Suites against main:** 38 of 40 manifests are identical.
  - `expression_evaluator`: 58 → 66. 13 tests were added; 5 were removed by rename or split (I88 RETURN §5, REPAIR_01).
  - `rule_check_runner`: 35 → 44. 10 were added; 1 was removed by rename.
  - Every test passes on both sides.
- **pytest:** 3,799 passed, 32 skipped, 130 subtests. This is unchanged from B6's run.
- **vitest:** 141 files and 3,620 tests, unchanged.
- **Builds:** `build:wasm:desktop` and `build:desktop` exit 0.

## After the merge

- NUM absorbs main.
- **S-I2's planning** accounts for SI1c, with RV111's N-3 (the desktop's library magnitude parser).
- The next records-only PR carries this record to main.
