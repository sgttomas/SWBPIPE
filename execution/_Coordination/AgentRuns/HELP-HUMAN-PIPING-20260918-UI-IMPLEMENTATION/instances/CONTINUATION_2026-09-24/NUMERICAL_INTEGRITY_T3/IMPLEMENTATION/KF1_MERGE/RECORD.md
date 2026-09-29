# KF1 merge record

- **PR:** https://github.com/sgttomas/chirality/pull/1056.
  - ROOT (HELP_HUMAN) merged it on 2026-09-29 at 21:34:25Z as `0f5d8c7b46570dbb8e7efabe542f99d04d63aad9`: a merge commit with `--match-head-commit 66adfede4`, under the owner's standing Git authorization.
  - Main was `8cca91701`, an ancestor of the head. Its piping tree equals `ab02ee3a6`'s (K4 merged); PR #1055 changed only `projects/chirality-app-v4/**`.
- **Candidate head:** `66adfede42de817efb5e0090342c02e3e4382f21`, on branch `codex/piping-kf1-20260929`.
- **Where it ran:** the owner's Mac (`aarch64-apple-darwin`, rustc 1.97.1). ROOT dispatched the implementer (I18) and the reviewer (RV20) directly.
- **Scope:** T3 slice KF1. It bounds the memory of K4's extreme trackers with every result unchanged, in `FK/src/structural/retained/adaptive.rs`.
  - Tests: the new `FK/tests/retained_k4/kf1_tracker_tests.rs` and a helper in `adaptive_tests.rs`.
  - One declared, additive row in `FK/tests/s11_site_table.rs`, authorized by ROOT.
  - **Kernel only:** `retained` stays private in FK.
  - The account is `IMPLEMENTATION/KF1/RETURN.md` and `CHANGE_RECORD.md` (with addenda 1 and 2), on the merged branch.

## The chain (branch point: main `8cca91701`)

| Commit | Content |
|---|---|
| `d0566126e` | Checkpoint 0: the plan |
| `68db15d41` | A: `BoundedExtremeTracker` and `TrackerSet` at every tracker site (T = 64), the tests and the site-table row |
| `d267a755b` | D: RETURN, CHANGE_RECORD and run records |
| `1854911d1` | T = 512 (ROOT's ruling on the corrected work figures); addendum 1 |
| `66adfede4` | RV20's review closed: the shared cap's work asserted and the trackers' finish order pinned; addendum 2 (tests and records only) |

## Gates

- **Independent review, RV20** (`REVIEW/KF1_REVIEW.md`, with `REVIEW/_run_records/kf1_review/`):
  - **PASS** at `1854911d1`, with 0 BLOCKING, 1 SHOULD-FIX and 5 NOTEs.
    - **Result equality:** RV20 checked RETURN §3's proof step by step, and found no difference over 36,000 streams. These included probes whose keys are unrelated to their values, and arbitrary collapse schedules.
    - **The shared cap** held over 400 rounds.
    - **A key-only replay** reproduced the work pins independently.
    - **RV20-1 (SHOULD-FIX):** no test checked the shared-cap collapse's work. RV20-M4 survived.
  - **PASS** at the final head `66adfede4` (a confirmation): RV20-1, N1, N3 and N5 are closed, and RV20-M4 and RV20-M5 are killed from clean archives.
    - Its one NOTE, C-N1, is recorded as a bracketed correction in ROOT's rulings: "every reachable refusal is `Span`" is shown only for the refusals constructed so far.
  - ROOT's rulings: `ROOT_RULINGS_V1.md`, from "KF1: spawn" to "KF1: rulings on RV20's review".
- **Tests:**
  - FK's full suite passes 401 at `1854911d1`.
  - At the final head only `kf1_tracker_tests.rs` changed, and KF1's 8 tests pass. DEC-025's frame_kernel suite ran 402.
  - `gen_k4_vectors.py --check` gives 23 of 23; GEN is unchanged.
- **Mutants:** I18's 10 killed at A, plus KF1-M7 and KF1-M8 at T = 512, and RV20-M4 and RV20-M5 at the final head. RV20-M1 and RV20-M2 are equivalent on every input constructed.
- **Hosted CI on `66adfede4`:**
  - pull_request runs: Piping Desktop E2E **36628177877**, Harness Pre-merge **36628178388**, governance-harness **36628177857** and pec-tests **36628177872**, all successful;
  - the full-SHA dispatch **36628173972** (target_base `8cca91701d558553d97c321537e447cb92b57cf3`): success;
  - the numerical cargo job took 10.7 min on the pull_request run and 17.5 min on the dispatch;
  - the earlier head `1854911d1` was green as well (dispatch 36621651732).
- **DEC-025,** under the owner's Mac decision (`OWNER_DIRECTION.md`, 2026-09-28), on the final head `66adfede4`. The evidence is in `dec025/`, and the driver is `M03_SKEW_PIN_MERGE/dec025/dec025_mac.sh.txt`, unchanged.
  - **Clean start:** the shared sweep target was fresh (deleted before and after), and the leftover summary was removed from the sweep worktree (K4's procedure note).
  1. **The sweep invocation** failed at the cargo surface on product_physics's platform test `t13` (fail-fast), as expected on the Mac.
  2. **All 39 manifests** were run with `--no-fail-fast`, against K4's Mac run at `5a46a6278`, whose piping source equals main `8cca91701`'s (`suites_vs_baseline.txt`).
     - frame_kernel grows 394 → 402 (KF1's 8 tests).
     - The failing tests are exactly the baseline's three Mac platform tests.
  3. **Surfaces 2, 3 and 5:**
     - pytest: 3062 passed, 32 skipped;
     - the wasm build: exit 0;
     - vitest: 134 files, 2822 of 2822;
     - the production build: exit 0.
  - **An earlier DEC-025 at `1854911d1`** (`dec025/earlier_1854911d1/`) gave the same result, with frame_kernel at 401.
  - Machine paths are sanitized, and ANSI colour codes are stripped.
- **GEN-8** on `66adfede4`: 1 passed (ROOT, with `set -o pipefail`). It also passed at `1854911d1`.
- **T9 and the both-entry gate:** not run. KF1 is kernel only.

## What KF1 established

- **K4's extreme trackers now use memory that does not depend on the data,** at every site, with results bit-identical to K4's, refusals included.
  - The stop rule holds at most 4,096 unevaluated rows per call (17.6 MB), peaking at 4,608 (19.8 MB) while a buffer grows.
  - The fallback holds at most 2,048 rows, and a solve attempt at most 2,560.
  - Each table keeps at most one 40 B entry per row kept in a window. Before KF1, the worst case was about 3.2 GB at 10,000 members.
- **Work:**
  - At T = 512 no control, and no RF-LARGE frame at up to 100 members, gains any work.
  - The worst case is one extra exact evaluation per row (17,506 LME), whatever T is. K6b re-measures at W1's sizes.
- **The budget boundary** is recorded: a case whose limit lies between K4's work and KF1's now ends on budget. No W1 limits exist yet.

## Routed

- **K6b:** merge main, recompute E_max from KF1's code for every tracker (and the fallback's per-state row list, about n_f × 4.3 KB), re-run W1-T3, and run W1-T4.
- **V-K:** merge main, re-run the kill matrix, and run B.
- **KF2 (K6's N10, the dense witness):** still to be briefed.

## Scratch to prune

`<wt>/kf1-target`, and `<wt>/scratch/sweep_kf1` and `sweep_kf1_1854911d1`. The records above keep every hash.
