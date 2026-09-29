# K6 merge record

- **PR:** https://github.com/sgttomas/chirality/pull/1053.
  - ROOT (HELP_HUMAN) merged it on 2026-09-29 at 09:52:09Z as `7ac7b1c377b614e2276d0b828205ac288ced5829`: a merge commit with `--match-head-commit cd325c1fe`, under the owner's standing Git authorization.
  - Main was `59cb20073` (F1b), an ancestor of the head.
- **Candidate head:** `cd325c1fe8e56e536ef2a7503a3ab3f97b1ed01a`, on branch `codex/piping-k6-20260928`.
- **Where it ran:** the owner's Mac (`aarch64-apple-darwin`, rustc 1.97.1). ROOT dispatched the implementer (I15) and the reviewer (RV18) directly.
- **Scope:** T3 slice K6, harness observations. It adds kernel-level sparse and dense memory and runtime observations on RF-LARGE and the DEC-053 models, the observation binary and runner, and the observation packet, all in `P/core/solver/performance_harness/**`, plus the pytest wrapper `P/tests/test_performance_harness_runner.py`.
  - **No product byte changes** (Scope 8; RV18 re-verified this independently).
  - The account is `IMPLEMENTATION/K6/CHANGE_RECORD.md` and `RETURN.md` (with addendum 1), on the merged branch.

## The chain (branch point: main `56dd72334`)

| Commit | Content |
|---|---|
| `9ababe4f2` | Checkpoints A1 and A2: the binary, generator, counts, allocator, runner and dry run |
| `1f354c20b` | A merge of main `1cdeae2c1` (K5). Test E was re-run (N6) |
| `962dd4e3b` | B1: T1 to T5, with the runner fixes made during the slot |
| `3799e3764` | B2: dense and lane-lu at 1,000 members, grid 128×128, and the B2-stop rule (the footprint measure) |
| `ada18de70` | B3: the Q4 ceiling run |
| `014b2ae04` | C: the mutation table, 26 of 26 killed after two survivor tests were added |
| `3e90176c6` | A merge of main `59cb20073` (F1b). H's suite and test E were re-run |
| `ae3320b5a` | D: RETURN, CHANGE_RECORD, records and the observation packet |
| `cd325c1fe` | Fixes for RV18's review, and addendum 1 |

## Gates

- **Independent review, RV18** (`REVIEW/K6_REVIEW.md`, with `REVIEW/_run_records/k6_review/`):
  - **PASS** at `ae3320b5a`, with 0 BLOCKING, 4 SHOULD-FIX and 8 NOTEs:
    - RV18-1: the time-budget and first-repeat stops were untested;
    - RV18-2: the headline heap figure and stage peaks were unpinned;
    - RV18-3: stopping the runner left the observation process running;
    - RV18-4: a log still contained machine paths.
  - **PASS** on the delta at `cd325c1fe`: all fixed, and 13 of 13 mutants killed from clean archives, including RV18's own. The SIGTERM handling is sound and restores the previous handler, and no route admits a CONT-like n10000 lane-id run.
  - ROOT's rulings: `ROOT_RULINGS_V1.md`, every K6 section.
- **Mutants:** 26 of 26 at C, plus 13 of 13 on RV18's fixes.
- **Hosted CI on `cd325c1fe`:**
  - pull_request runs: Piping Desktop E2E **36546533793**, Harness Pre-merge **36546533839**, governance-harness **36546533809** and pec-tests **36546533788**, all successful. 12 checks passed and 4 were skipped as selected, including the Linux numerical cargo suite;
  - the full-SHA dispatch **36548351414** (target_base `59cb200730eef01a805a5b9f4b840a845ff4204b`): success.
- **DEC-025,** under the owner's Mac decision (`OWNER_DIRECTION.md`, 2026-09-28), on the final head `cd325c1fe`. The evidence is in `dec025/`, and the driver is `M03_SKEW_PIN_MERGE/dec025/dec025_mac.sh.txt`, unchanged.
  1. **The sweep invocation** failed at the cargo surface on product_physics's platform test `t13` (fail-fast), as expected on the Mac. The summary JSON is sanitized; its original sha256 is in `sweep_json_original_sha256.txt`.
  2. **All 39 manifests** were run with `--no-fail-fast`, against F1b's Mac run at `6fa422979`, whose piping source equals main `59cb20073`'s (`suites_vs_baseline.txt`).
     - performance_harness grows 25 → 54 tests, K6's own.
     - The failing tests are exactly the baseline's three Mac platform tests.
     - **operation_applier reported 0 tests, against 194: a build failure in the shared target, not a K6 effect.**
       - Its test targets failed to compile with E0308. Two `serde_json` versions (1.0.150, pinned by its lock, and 1.0.151, pinned by `core/loads/self_weight_wasm`, which the sweep builds earlier) met in one graph. This is consistent with a collision through the target folder that all 39 manifests share; the mechanism is not proven.
       - K6 changes no file in operation_applier or its dependency closure.
       - **Re-run alone on `cd325c1fe` with a fresh target folder: 194 passed, 0 failed** (`operation_applier_rerun/`).
       - ROOT then removed the shared sweep target folder (3.1 GB), so the next sweep starts clean.
       - The DEC-025 driver's shared target is recorded as a procedure weakness: a fresh target per sweep, or per manifest lock, would avoid it.
  3. **Surfaces 2, 3 and 5:**
     - pytest: 3062 passed, 32 skipped (K6 adds the runner tests);
     - the wasm build: exit 0;
     - vitest: 134 files, 2822 of 2822;
     - the production build: exit 0.
  - Machine paths are sanitized to `<WORKTREE>`, `<VENV>`, `<wt>`, `<home>` and `<tmp>`, and ANSI colour codes are stripped.
- **GEN-8** on `cd325c1fe`: 1 passed (ROOT, with `set -o pipefail`, and RV18).
- **T9 and the both-entry gate:** not run. K6 changes no product byte (Scope 8), as ruled in the brief.

## What K6 established (observations, not thresholds; RETURN §7–§8)

- **Kernel sparse heap is linear in members:** slope 0.998–1.002 from 10 to 10,000.
- **F1b's dense estimate (96·n²) matches kernel heap:**
  - within 0.6% at 1,000 members;
  - 1.0032 × at the ceiling run (8,190 DOFs, 1.0027 × F1b's 6 GiB provisional ceiling);
  - footprint 1.008–1.035 ×;
  - macOS RSS 1.04–1.45 × above it, varying between runs rather than with size.
- **The product-level gap:** CONT n10000 sparse takes about 0.25 GiB of heap in the kernel, against 4.8–5.2 GiB of RSS at product level.
- **N10, a T3 finding on main's dense path:** the dense factor refuses a late pivot on two 1,000-member models, and the O(n⁴) dense witness then runs until the kill. It is routed to a kernel follow-up.
- **What K6 cannot give:** product overhead, a target-machine policy, other platforms, W1 (K6b, after K4) and a time bound.
  - The provisional dense-scrutiny and lane ceilings remain the owner's decision, with V-P's product-level measurements.
