# S-I1 merged: #1100 at `ef266247de`

**The merge.** [#1100](https://github.com/sgttomas/chirality/pull/1100) merged on 2026-10-06 at 14:59:35Z, as merge commit `75a8c3291f`.
- Parents: main `c1571f7feb` and the PR head `ef266247de` (`_run_records/PARENTS.txt`).
- Command: `gh pr merge --merge --match-head-commit` (`_run_records/MERGE_COMMAND.txt`).
- Main's maintained diff is S-I1's 8 files, plus the 4 package files under `IMPLEMENTATION/S_I1/`.

## Gates

| Gate | Result | Evidence |
|---|---|---|
| Independent complete-diff review | RV99: PASS (0/3/6). Repairs CONFIRMED (ADDENDUM_01). PR head `20e7e3e5a2` CONFIRMED (ADDENDUM_02). Amended head `ef266247de` and the DEC-025 carry-over CONFIRMED (ADDENDUM_03) | `R/REVIEW_RV99/s_i1_01/` |
| `source_equality.py` | 5/5 PASS at `20e7e3e5a2` against NUM `b9030f501c`, and at `ef266247de` against NUM `f9657c51c5` | `_run_records/se.txt`, `se2.txt` |
| `check_citations.py` | PASS: 34 D2 §4.11 citations, resolved at NUM `377d1d5cfb` (D2 5b.3) | `_run_records/citations.txt`, `citations2.txt` |
| GEN-8 | 1 passed at `20e7e3e5a2` and at `ef266247de` | `_run_records/gen8.txt`, `gen8_2.txt` |
| Hosted CI on `ef266247de` | All automatic runs succeeded (pec-tests, Harness Pre-merge Validation, governance-harness, Piping Desktop E2E). The full-SHA dispatch 37476322784 (`target_base` = main `c1571f7feb`) succeeded | `_run_records/CI_RUNS_ef266247de.txt` |
| Exact-head Mac DEC-025 against a fresh main baseline | **`ALL-DONE`**, run on `20e7e3e5a2`. Baseline: `run_suites_nff.sh` on a clean worktree of main `c1571f7feb`. Carried to `ef266247de` by ruling (package text only), with RV99 confirming the premise | `dec025/` |
| Pass B | Not applicable: the rules crates are outside PP's closure (I61 PLAN §3) | — |

**DEC-025 in detail** (`dec025/`):
- The evidence sweep stopped at PP's known `t13`, as in every Mac DEC-025.
- **Suites against main:** 38 of 40 manifests identical. `expression_evaluator` 32 → 50 (+18 added, all ok); `rule_check_runner` 21 → 33 (+12 added, all ok).
- **pytest:** 3,733 passed, 32 skipped. That is F′'s 3,540 plus `test_rule_interval.py`'s 193.
- **vitest:** 138 files and 3,552 tests passed.
- **Builds:** `build:wasm:desktop` and `build:desktop` exit 0.

## After the merge

- NUM absorbs main.
- T3-SI1b, the point-path panic repair, becomes ready.
- U8 is the next product PR.
- The records-only PR carries this record to main.
