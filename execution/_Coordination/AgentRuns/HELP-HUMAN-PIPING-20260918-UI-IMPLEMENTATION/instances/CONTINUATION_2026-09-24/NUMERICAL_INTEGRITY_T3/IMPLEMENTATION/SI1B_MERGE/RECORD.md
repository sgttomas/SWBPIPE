# T3-SI1b merged: #1106 at `b4f22e6ce7`

**The merge.** [#1106](https://github.com/sgttomas/chirality/pull/1106) merged on 2026-10-07 at 03:00:18Z, as merge commit `025c1cf326`.
- Parents: main `47a3bdfcf5` and the PR head `b4f22e6ce7` (`_run_records/PARENTS.txt`).
- Command: `gh pr merge --merge --match-head-commit` (`_run_records/MERGE_COMMAND.txt`).
- Main's diff is the slice's 3 maintained files plus the 4 package files under `IMPLEMENTATION/SI1B/`: 7 files, +974 / −6.

## Gates

| Gate | Result | Evidence |
|---|---|---|
| Independent complete-diff review | RV104: PASS (0/0/6). Repair round CONFIRMED (ADDENDUM_01). PR head `b4f22e6ce7`'s scope, package and equality CONFIRMED, 0/0/1 (ADDENDUM_02) | `R/REVIEW_RV104/si1b_01/` |
| `source_equality.py` | 5/5 PASS at `b4f22e6ce7` against NUM `9e09bc2a35` and main `47a3bdfcf5`, with \|S\| = 3. RV104 reproduced it | `_run_records/se.txt`, `se.json` |
| `check_citations.py` with the package index | PASS: 1 resolved (D2 §4.11.2) | `_run_records/citations.txt` |
| GEN-8 | 1 passed at `b4f22e6ce7` | `_run_records/gen8.txt` |
| Hosted CI on `b4f22e6ce7` | All automatic runs succeeded. The full-SHA dispatch 37555520168 (`target_base` = main `47a3bdfcf5`) succeeded | `_run_records/CI_RUNS_b4f22e6ce7.txt` |
| The full 40-manifest suite before the freeze | Satisfied by the exact-head DEC-025's 40 manifests, as for S-I1 and T6S | `dec025/` |
| Exact-head Mac DEC-025 against a fresh main baseline | **`ALL-DONE`** on `b4f22e6ce7`. Baseline: a clean worktree of main `47a3bdfcf5`. Both use fresh targets. The first attempt was stopped by ROOT before any work because of a quiet-check deadlock, and restarted after the fix (RR "A deadlock in DEC-025's quiet check, fixed; …") | `dec025/` |
| Pass B | Not applicable: the rules crates are outside PP's closure. RV104 confirmed this at the head (PP's lockfile lists 15 workspace crates, none of them a rules crate) | `R/REVIEW_RV104/si1b_01/ADDENDUM_02.md` |

**RV104 A2-N1:** CHANGE_RECORD §3's "I61 PLAN §3" means `R/I61/u8_plan_01/PLAN.md` §3 ("S-I1's readiness").

## DEC-025 in detail (`dec025/`)

- **The sweep** stopped at PP's known `t13`.
- **Suites against main:** 38 of 40 manifests are identical.
  - `expression_evaluator`: 50 → 58 (+8 added, all ok).
  - `rule_check_runner`: 33 → 35 (+2 added, all ok).
- **pytest:** 3,773 passed, 32 skipped, unchanged. The Python change is comments and one docstring.
- **vitest:** 141 files and 3,612 tests, unchanged.
- **Builds:** `build:wasm:desktop` and `build:desktop` exit 0.

## After the merge

- NUM absorbs main.
- **T3-SI1c** (point-path booleans over non-finite intermediates, with RV104's N-4 and N-5) is ready for a plan.
- B6's PR is next by the merge order, then PR-B1.
- The records-only PR carries this record to main.
