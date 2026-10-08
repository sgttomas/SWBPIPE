# T3-B6 merged: #1107 at `1199726f69`

**The merge.** [#1107](https://github.com/sgttomas/chirality/pull/1107) merged on 2026-10-07 at 13:15:15Z, as merge commit `2007709549`.
- Parents: main `025c1cf326` and the PR head `1199726f69` (`_run_records/PARENTS.txt`).
- Command: `gh pr merge --merge --match-head-commit` (`_run_records/MERGE_COMMAND.txt`).
- Main's diff is the slice's 11 maintained files plus the 4 package files under `IMPLEMENTATION/B6/`: 15 files, +900 / −71.

## Gates

| Gate | Result | Evidence |
|---|---|---|
| Independent complete-diff review | RV108: PASS (0/0/7). PR head `1199726f69`'s scope, package and equality CONFIRMED, 0/0/3 (ADDENDUM_01) | `R/REVIEW_RV108/b6_01/` |
| `source_equality.py` | 5/5 PASS at `1199726f69` against NUM `c698a0b9a5` and main `025c1cf326`, with \|S\| = 11. RV108 reproduced it twice | `_run_records/se.txt`, `se.json` |
| `check_citations.py` with the package index | PASS: 7 resolved, all on the carrier case file's `scope` line, all present before B6 | `_run_records/citations.txt` |
| GEN-8 | 1 passed at `1199726f69` | `_run_records/gen8.txt` |
| Hosted CI on `1199726f69` | All automatic runs succeeded. The full-SHA dispatch 37621653258 (`target_base` = main `025c1cf326f1b9ef827626f27d0e2b9cf5b415fe`) succeeded. The first dispatch, 37620732340, was refused because ROOT passed a short SHA; it was not a defect in the PR | `_run_records/CI_RUNS_1199726f69.txt` |
| The full 40-manifest suite before the freeze | Satisfied by the exact-head DEC-025's 40 manifests, as for S-I1, T6S and SI1b | `dec025/` |
| Exact-head Mac DEC-025 against a fresh main baseline | **`ALL-DONE`** on `1199726f69`. Baseline: `WT/main-baseline` at main `025c1cf326`. Both use fresh targets | `dec025/` |
| Pass B | Not applicable: no file in the D1 milestone's call graph (RV108 item 5) | `R/REVIEW_RV108/b6_01/REVIEW.md` |

## DEC-025 in detail (`dec025/`)

- **The sweep** stopped at PP's known `t13`.
- **Suites against main:** 39 of 40 manifests are identical. `result_export` goes 177 → 180 (+3 added, all ok).
- **pytest:** 3,773 → 3,799 passed (+26, B6's Python additions), with 32 skipped both times.
- **vitest:** 141 files; 3,612 → 3,620 (+13 added, 5 renamed or dropped with F-U6b-2's forms).
- **Builds:** `build:wasm:desktop` and `build:desktop` exit 0.

## Corrections to the package (RV108 ADDENDUM_01)

The package is unchanged on main; read it with these:
- **A-N1:** CHANGE_RECORD §2 item 3 and the PR body call Python's transport validator "the twin of" Rust's and TypeScript's. It is exact on G0–G2 only; at the base step Python runs both Rust's check and TS's (REVIEW N6(a)).
- **A-N2:** CHANGE_RECORD §6 carries only part of the routing. The full routing is in RR "Owner decision: SI1c is option D, …" and "B6's PR #1107 cut; RV108 confirms its head; …". It adds:
  - SC's corpus entries for RV108 N1, N2 and N4;
  - SC's transport scope sentence (N6);
  - SR-PY's docstring wording (N6(a)).

  §4's "pre-existing reader differences and wording" also leaves out N5 (test strength, to SC) and N7 (no action).
- **A-N3:** two added comment lines cite `R/REVIEW_RV92/u6f_01`, a form `check_citations.py` does not parse. It resolves to the same record as the indexed `RV92 u6f_01`.

## After the merge

- NUM absorbs main.
- **B1's I1 and I1′ coincide.** `b1` absorbs main at I1, after ST's repair round and RV109's confirmation.
- **T3-SI1c** (I88) has its PR next.
- The records-only PR carries this record to main.
