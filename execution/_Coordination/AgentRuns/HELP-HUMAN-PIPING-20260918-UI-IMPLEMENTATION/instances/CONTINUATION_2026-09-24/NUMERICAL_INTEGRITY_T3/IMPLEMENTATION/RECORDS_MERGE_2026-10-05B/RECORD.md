# T3 records merged to main — PR #1088 (ROOT, 2026-10-05 UTC)

**PR [#1088](https://github.com/sgttomas/chirality/pull/1088)** was squash-merged at `2026-10-05T15:01:44Z` as `efca0cf6b63758ab8448581e388a8896606ec84b`, a single parent on main `a5ecca3b59`.
- **Equality:** main's `projects/chirality-piping/execution/` equals the gated head H2 `020d25a3d61c0a8f63ad92b5e0c602328b1a3ddb` byte for byte. Nothing else on main changed.
- **The command:** `gh pr merge --squash --match-head-commit` H2, with an explicit subject and body. It ran immediately after ROOT confirmed main was still `a5ecca3b59`. Auto-merge stayed off.
- **The facts:** in `_run_records/`.

## What reached main

- **60 paths under `projects/chirality-piping/execution/`** (50 added, 10 modified, 0 deleted), equal to the T3 integration branch at `f1b3a82531`. They are:
  - #1084's merge record;
  - RV96 addendum 02, and RV100's review;
  - the stray-scratch gathering, and the cleanup procedure's step 0;
  - the DEC-025 wrapper `run_dec025.sh`;
  - the handoff and steering prompt in final form;
  - errata E-3 and E-4;
  - the rulings and the work graph's T3 row.
- **No source, test, CI, tool or portability-policy change.**

## Gates

| Gate | Result | Evidence |
|---|---|---|
| **Independent review (RV100)** | **PASS at H** `e2b83da584` (0/1/7). S-1 was repaired by the re-cut. **PASS at H2** (0/0/5, notes only) | `R/REVIEW_RV100/records_01/` (REVIEW, ADDENDUM_01) |
| **GEN-8** | **H:** 1 passed. **H2:** 1 passed. **H2 with main `a5ecca3b59`, combined locally:** 1 passed | `_run_records/gen8_H.txt`, `gen8_H2.txt`, `gen8_H2_combined.txt` |
| **The PR's automatic CI on H2** | All four succeeded: Piping Desktop E2E 37328163446 (it selected no piping tests, since no piping inputs changed), pec-tests 37328163403, Harness Pre-merge 37328163411, governance-harness 37328163454 (1,156 passed) | `_run_records/CI_RUNS.jsonl` |
| **Main's move after the PR's base** | `f506f3e2de..a5ecca3b59` (#1089) touches 34 paths, all under `projects/chirality-app-v4/`. The PR's runs used GitHub's merge commit with that main, whose tree RV100 matched to its local combination (`5033cd22…`) | RV100 ADDENDUM_01 §3 |

**Informational only:** these ran before the owner's proportionate-CI direction, and are not gates for a records-only PR.
- **DEC-025 on H,** completed (`ALL-DONE` 14:48:51Z). All 40 manifests are identical to R3's (`dec025_informational/`).
- **The full-SHA dispatch 37322486897** on H, which succeeded.

**Void:** the first DEC-025 attempt on H ran no step, because the wrapper created no output folder. Its log is in `<WT>/scratch/u9_dec025/H1088_attempt1_no_outdir/`.

## The re-cut

The branch was cut from `f506f3e2de` as H. It then gained two commits:
- **`af7510c3a3`:** RV100's S-1 repair and its notes;
- **`020d25a3d6`:** the owner's direction on proportionate CI.

The squash put one commit on main. None of #1084's 13 redacted originals is in any of these trees (RV100).
