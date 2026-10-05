# I61 RETURN: U3 grant 2 follow-on (D-U6-5 and the reruns on the merged base)

**Status: complete, with no stop.** The work is uncommitted in WT/f2a-memory, on top of `f8ce1eb32b`: NUM, carrying U6, merged into the memory branch.
- **The milestone still publishes its successor on the merged base,** in both modes, with U6's 07h reader at precommit. It is not a Precommit fallback, and its bytes are U1's pinned successors (`ac6986b0…` sparse, `6cd1d249…` dense).
- **D-U6-5 is done.** One new PP test reads U6's two carrier fixtures with `include_str!` and compares them byte for byte with the successor the actual Direct entry publishes.
- **Every rerun is identical to grant 2's run.** This covers PP in the registered and Stale builds, runner/headless (registered), the 324-output sweeps (registered and Stale), and U5.

**Run facts.**
- **Role and basis:** TASK Type 2 under ROOT, with no descendants. The basis is ROOT's follow-on message (grant 2 committed as `664f8df7b7`; the merge `f8ce1eb32b`) and grant 2's brief and records (`R/I61/u3_grant2_01/`).
- **Time:** 2026-10-04, 15:50Z to about 16:00Z.
- **Host:**
  - The memory guard (PID 5387) ran throughout.
  - Default toolchain; `--locked --offline`; `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`; one cargo job at a time, each under a perl alarm.
  - Targets were WT/targets/i61-u3g2/ only (`reg`, `stale`, `reg-runner`, `mut`). The default target was never used.
- **Git:** no Git writes. Reads used `GIT_OPTIONAL_LOCKS=0`, and `git archive f8ce1eb32b` made the scratch trees.
- **Writes:**
  - one test file in WT/f2a-memory;
  - WT/scratch/i61_u3_grant2_02/;
  - WT/targets/i61-u3g2/;
  - this folder.
- **Fence:** `retained_memory.rs` is untouched, and ROOT's pending T17_V4 change is not anticipated. Production text is untouched: `lib.rs`, `retained_product.rs` and the seams are byte-identical to `f8ce1eb32b` (`changed_files_sha256.txt`).

## 1. D-U6-5

**The test** is `retained_facade_tests::u3g2_d_u6_5_carrier_fixtures_are_the_live_successors` (+33 lines, test file only).
- It reads `fixtures/results/retained_precision_milestone_successor_{sparse_interactive,dense_scrutiny}.json` with `include_str!`.
- **In the registered build:**
  - it takes the successor that `run_linear_static_preview_value_with_retained_direct` publishes (`into_publication()` is `Successor`), from one ordinary run that consulted G-C once;
  - it builds U1's document form;
  - it requires that document to equal the fixture **byte for byte**.
- **In any other build** it makes the same comparison against the private driver's successor document, which U1 pinned.
- **Either way,** the fixture's sha256 and its `receipt_sha256` are U1's pinned values.

**Results:**
- The test passes in the registered build and in the Stale build.
- **Mutants: 2 of 2 killed, none by a compile error** (`mutants.json`):
  - one byte of the sparse carrier changed: killed only by the new test;
  - the published successor gaining a key at the transfer: killed by the new test and the two existing successor tests.

## 2. Reruns on the merged base (all identical to grant 2)

| Run | Result |
|---|---|
| **The five `u3g2_*` tests plus D-U6-5, registered** | 6 of 6 pass. The milestone publishes the successor in both modes (`Successor`, not Precommit) under the 07h reader. Its live bytes are `ac6986b0…` and `6cd1d249…` (`outputs_sha256.txt`) |
| **PP, registered,** all targets, `--no-fail-fast` | **705 passed, 1 failed (Mac t13), 10 ignored.** Per test this equals grant 2's run plus the new test |
| **PP, Stale** (`RUSTFLAGS=--cfg=i61_u3g2_stale`) | The same 705/1/10, outcome-identical to registered |
| **runner/headless, registered** | 85 passed, 2 failed (base's own `load_reference` failures), identical to grant 2's run |
| **The 324-output sweep, registered and Stale** | **Both byte-identical to grant 2's run** (`sweep/vs_grant2.txt`). So controls 1 and 2 hold as before: Stale publishes base `b54caba7ab`'s bytes on all 324 outputs. Registered differs only at Direct (2 successors, 4 notices, 64 exact, the 2 errors equal to base's). **No change from grant 2** |
| **U5,** on the merged base's live successor bytes, both modes | `u5_report.json` and `u5_run.log` are **byte-identical to U5's**, with no stops and 97 of 97 class claims per mode. This used RV86's extract and the pinned `u5_compare.py` (`df4684d3…`), with WT/f2a-memory's Python reader, which now carries the 07h reader's F5 changes |

## Records (`_run_records/`)

All paths are placeholders, with no machine paths.
- **Candidate:** `candidate.diff` (against `f8ce1eb32b`: the one test), `candidate_status.txt`, `changed_files_sha256.txt`.
- **Suites:** `suites/{reg_pp,stale_pp,reg_runner}.outcomes`.
- **Sweeps:** `sweep/sweep_cand_{reg,stale}.tsv`, `sweep/compare.json`, `sweep/vs_grant2.txt`.
- **U5:** `u5/run_u5.sh`, `u5/u5_report.json`, `u5/u5_run.log`.
- **Mutants:** `mutants.py`, `mutants.json`.
- **Scripts and outputs:** `run_suites.sh`, `run_sweeps.sh` (grant 2's harness and comparer, reused unchanged from `R/I61/u3_grant2_01/_run_records/`), `outputs_sha256.txt`.
