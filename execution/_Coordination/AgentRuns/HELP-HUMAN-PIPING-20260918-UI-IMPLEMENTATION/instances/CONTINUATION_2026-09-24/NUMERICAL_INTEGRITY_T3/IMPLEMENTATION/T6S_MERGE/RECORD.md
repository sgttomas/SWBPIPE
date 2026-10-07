# T6S merged: #1104 at `d953e12187`

**The merge.** [#1104](https://github.com/sgttomas/chirality/pull/1104) merged on 2026-10-07 at 00:17:29Z, as merge commit `bfb26596bf`.
- Parents: main `d8c88774d0` and the PR head `d953e12187` (`_run_records/PARENTS.txt`).
- Command: `gh pr merge --merge --match-head-commit` (`_run_records/MERGE_COMMAND.txt`).
- Main's diff is the slice's 19 maintained files plus the 4 package files under `IMPLEMENTATION/T6S/`: 23 files, +3,177 / −1,903.

## Gates

| Gate | Result | Evidence |
|---|---|---|
| Independent complete-diff review | RV101: PASS (0/1/10). SF-1 was repaired by I75 for 16- and 17-digit ties, and CONFIRMED (ADDENDUM_01). The PR head `d953e12187`'s scope, package truth and Pass B premise were CONFIRMED, 0/0/4 (ADDENDUM_02) | `R/REVIEW_RV101/t6s_01/` |
| `source_equality.py` | 5/5 PASS at `d953e12187` against NUM `0416b3b2ce` and main `d8c88774d0`, with \|S\| = 19. RV101 reproduced it, and it also passes against NUM `53f626a5c8` | `_run_records/se.txt`, `se.json` |
| `check_citations.py` with the package index | PASS: 2 resolved, 0 ambiguous, 0 unresolved | `_run_records/citations.txt` |
| GEN-8 | 1 passed at `d953e12187` | `_run_records/gen8.txt` |
| Hosted CI on `d953e12187` | All automatic runs succeeded (Piping Desktop E2E, pec-tests, Harness Pre-merge Validation, governance-harness). The full-SHA dispatch 37546714187 (`target_base` = main `d8c88774d0`) succeeded | `_run_records/CI_RUNS_d953e12187.txt` |
| The full 40-manifest suite before the freeze | **Ruled:** satisfied by the exact-head DEC-025's 40 manifests, because the slice moves no registered identity (RR "I80's package accepted; …") | `dec025/` |
| Exact-head Mac DEC-025 against a fresh main baseline | **`ALL-DONE`** on `d953e12187`. Baseline: a clean worktree of main `d8c88774d0`. Both use fresh targets | `dec025/` |
| Pass B | **Not applicable (ruled):** no D1 crate source, embedded static, reviewed input, reader source, manifest, lock or build script changed. RV101 confirmed this at the head | RR "I80's package accepted; …"; CHANGE_RECORD §5.1 |

The package's §7 still lists items 1–2 "for ROOT's ruling". Both were ruled before the cut, as its gate rows record (RV101 A2-N1).

## DEC-025 in detail (`dec025/`)

- **The sweep** stopped at PP's known `t13`, as in every Mac DEC-025.
- **Suites against main:** 39 of 40 manifests are identical. `result_export` goes 173 → 177 (+4 added, all ok): the 3 golden tests and RV95 N-5's masking test.
- **pytest:** 3,773 passed, 32 skipped. That is main's 3,750 plus the slice's 23 (the dispatcher test).
- **vitest:** 141 files and 3,612 tests passed. That is main's 3,574 plus the slice's 38 (44 added, 6 renamed).
- **Builds:** `build:wasm:desktop` and `build:desktop` exit 0.
- **RV101 A2-N4:** the slice's tests had run only on the pre-U8 tree. On the combined tree every T6S test file passes, and every count equals the expected delta exactly (RR "I80's package accepted; …"), so U8's corpus and fixtures do not interact with the slice.

## After the merge

- NUM absorbs main.
- **T3-SI1b** becomes the next product slice into NUM, after RV104's review.
- **B6** may start, as the corpus's single writer before B1's snapshot.
- **The T6 node** is done for this slot. NT-7, NT-10 and I75's item d go to T6's later slot. Activation is B8's.
- The records-only PR carries this record to main.
