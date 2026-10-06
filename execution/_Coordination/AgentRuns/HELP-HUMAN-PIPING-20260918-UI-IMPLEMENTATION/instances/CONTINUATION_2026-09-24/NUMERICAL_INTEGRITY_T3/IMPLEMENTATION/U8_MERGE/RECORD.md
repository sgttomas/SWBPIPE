# U8 merged: #1102 at `f7a7572e35`

**The merge.** [#1102](https://github.com/sgttomas/chirality/pull/1102) merged on 2026-10-06 at 16:41:07Z, as merge commit `f8ed4f0551`.
- Parents: main `d069e7130c` and the PR head `f7a7572e35` (`_run_records/PARENTS.txt`).
- Command: `gh pr merge --merge --match-head-commit` (`_run_records/MERGE_COMMAND.txt`).
- Main's diff is U8's 7 maintained files plus the 4 package files under `IMPLEMENTATION/U8/`: 11 files, +29,864 / −14.

## Gates

| Gate | Result | Evidence |
|---|---|---|
| Independent complete-diff review | RV97: PASS, with repairs CONFIRMED. PR heads `61c35f56a8` and `b18dd4f369` CONFIRMED. `f7a7572e35` and the DEC-025 carry-over CONFIRMED (ADDENDUM_02) | `R/REVIEW_RV97/` |
| `source_equality.py` | 5/5 PASS at `61c35f56a8`, `b18dd4f369` and `f7a7572e35` (the last against NUM `55f5b7ecf1` and main `d069e7130c`) | `_run_records/se*.txt`, `se*.json` |
| `check_citations.py` with U8's index | PASS at each head: 10 resolved, 0 ambiguous, 0 unresolved | `_run_records/citations*.txt` |
| GEN-8 | 1 passed at each head, the last at `f7a7572e35` | `_run_records/gen8*.txt` |
| Hosted CI on `f7a7572e35` | All automatic runs succeeded (Piping Desktop E2E, pec-tests, Harness Pre-merge Validation, governance-harness). The full-SHA dispatch 37489501933 (`target_base` = main `d069e7130c`) succeeded | `_run_records/CI_RUNS_f7a7572e35.txt` |
| Pass B | I72 on `bd6b4be2c3`: `DELTAS TO READ`, the same gate vector as F's; U8's only new outcomes are its three tests, all ok | `R/I72/u8_passb_01/` |
| The full 40-manifest suite before the freeze | PASS on `bd6b4be2c3`: 38 of 40 identical to F′'s | `IMPLEMENTATION/U8_GATES/full_suite/` |
| Exact-head Mac DEC-025 against a fresh main baseline | **`ALL-DONE`** on `61c35f56a8`. Baseline: `run_suites_nff.sh` on a clean worktree of main `75a8c3291f`, in a fresh target. Carried to `f7a7572e35` by ruling (package text and #1101's records only), with RV97 confirming the premise. The suites were rerun in a fresh target (below) | `dec025/` |

## DEC-025 in detail (`dec025/`)

- **The sweep** stopped at PP's known `t13`, as in every Mac DEC-025.
- **The suites, first pass** (`compare_shared_target.txt`): 37 of 40 identical. `operation_applier`'s tests did not compile: 279 errors (244 E0308, 26 E0277, 9 E0631). rustc's notes name two versions of `serde_json` (1.0.150 and 1.0.151) and of `serde_core` in one build (`operation_applier_shared_target_excerpt.txt`).
  - **The cause was the host, not U8.** `dec025_mac.sh` built every candidate's suites in one persistent target directory. There, `operation_applier` is built under two lockfiles: its own (`serde_json` 1.0.150) and `self_weight_wasm`'s (1.0.151; manifest 004, which depends on it). The persistent target mixed the two resolutions. The baseline uses a fresh target per run and passed 194/0.
  - U8 changes no manifest, lockfile or `operation_applier` source. The hosted Numerical cargo suite passed on `f7a7572e35`.
- **The suites, rerun** on the same tree `61c35f56a8` in a fresh target, under the T3 lock (`compare.txt`, `suites_candidate.log`, `suites_fresh_meta.txt`): **38 of 40 identical** to the baseline. PP 705 → 708 and result_export 172 → 173 are U8's added tests, all ok. The known `t13` fails on both sides. `operation_applier` passes 194 / 0, as on main.
- **The repair:** `dec025_mac.sh` now builds each candidate's suites in a fresh per-label target, as the baseline does (`SESSION_2026-10-06/host_tools/dec025_mac.sh.txt`).
- **pytest:** 3,750 passed, 32 skipped. That is S-I1's 3,733 plus U8's 17 (the retained suites, 463 → 480).
- **vitest:** 138 files and 3,574 tests passed. That is 3,552 plus U8's 22, all in `retainedPrecision.test.ts`.
- **Builds:** `build:wasm:desktop` and `build:desktop` exit 0.

## After the merge

- NUM absorbs main.
- B0 (F2a breadth: contract and identities) and T3-SI1b (the point-path panic repair) become ready.
- The T6 slice is the next product PR, after I75's SF-1 repair and RV101's confirmation.
- The records-only PR carries this record to main.
