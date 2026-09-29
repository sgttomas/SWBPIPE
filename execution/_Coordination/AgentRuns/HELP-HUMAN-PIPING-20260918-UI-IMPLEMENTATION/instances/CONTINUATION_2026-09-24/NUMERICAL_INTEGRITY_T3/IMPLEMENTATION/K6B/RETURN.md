# I16 return: slice K6b (W1 observations on the K6 harness: limbs, work, memory per precision and seconds per work unit, and FK's `retained_api` export)

> **W1-T4 (10,000 members) is pre-KF3 throughout this return.** Its figures come from slot K6B-S4 (b3) on `4eeb206c0`, before KF3. KF3 changes how W1a treats a `Uc` it cannot form, and records partial stage work on every error path ("K6b: slot K6B-S3 stopped at W1-T4's first row", "K6b: slot K6B-S4 (b3) accepted", `ROOT_RULINGS_V1.md`). **After KF3 merges, K6b merges main and re-runs W1-T4 as RETURN addendum 1 (records addendum), which may follow K6b's merge. ROOT's W1 limits use the post-KF3 figures.** W1-T1 to W1-T3 (10 to 1,000 members) are not expected to change under KF3: none of their attempts stopped.

- **Status:** checkpoint D. The slice is observation only: no test or record asserts a time or memory bound.
- **Branch:** `codex/piping-k6b-20260929` in `<wt>/k6b`, from main `ab02ee3a6` (K4 merged, PR #1054), with main `0f5d8c7b4` (KF1, PR #1056) merged in by ROOT as `b86081221`.
- **Platform:** `aarch64-apple-darwin` (Apple M5 Max, 18 cores, 128 GiB, macOS 26.6.2), rustc 1.97.1, Python 3.13.14. **Every observation is Mac-only.**
- **Placeholders:** `<wt>` is the T3 worktree root, `<scratch>` is `<wt>/scratch`. Records carry no machine paths or host names (`_run_records/d/assemble_records.py.txt` checks it).
- **Paths:** `P/`, `T3/`, `H`, `FK`, `SD`, `NI`, `SA` and `PP` as in `I15_K6_IMPLEMENTATION.md`. `K4R` is `FK/structural/retained/`; `K4T` is `P/core/solver/frame_kernel/tests/retained_k4/`.

## 1. Brief, basis, delegation and rulings

- **Brief:** `TASK_BRIEFS/I16_K6B_IMPLEMENTATION.md`, which builds on `I15_K6_IMPLEMENTATION.md`. The plan is `IMPLEMENTATION/K6B/PLAN_CHECKPOINT0.md` (sha256 `7d6b7af3…`, committed by ROOT at `c0436769f`).
- **Delegation:** ROOT (HELP_HUMAN, the chirality-piping session) dispatched I16 directly as a Type 2 TASK, through the host's native background-subagent mechanism (D-GOV-35), with the brief as the assignment. Each checkpoint returned to ROOT through the subagent return path; ROOT committed every change. I made no Git writes and no index operations. When KF1 merged, before ROOT's correction, I ran `git status`, `git log` and one `git fetch` (remote-tracking refs only); ROOT recorded the fetch as disclosed.
- **Rulings** (`ROOT_RULINGS_V1.md`, read at the numerics head):
  - "K6b and V-K: spawn"; "V-K: rulings on I17's checkpoint-0 plan" Q8 (the export's fields and methods);
  - "K6b: rulings on I16's checkpoint-0 plan, and A0's export" (C-1 the facade, C-2 `PrecisionState` stays crate-private, Q1–Q7);
  - "K6b A0 accepted"; "K6b: A1 accepted; K4's stop-rule memory finding"; "K6b: A2 accepted; W1-T1 to W1-T3 approved";
  - "K6b: B's W1-T1 to W1-T3 accepted; W1-T4 approved before KF1"; "K6b: W1-T4 stopped by the binary's backstop; deferred until KF1";
  - "K6b: C's mutation table; M10's test approved"; "KF1 merged"; "K6b: main merged; E_max from KF1; slot K6B-S3 approved";
  - "K6b: slot K6B-S3 stopped at W1-T4's first row; W1 at 10,000 members ends in Span" ((a) the parity fix in H, (b) the Span routed to KF3, (c) W1-T4 re-run in b3);
  - "K6b: slot K6B-S4 (b3) accepted; D now, with W1-T4 pre-KF3".
- **Commits on the branch (made by ROOT):**

| Commit | Checkpoint | What |
|---|---|---|
| `c0436769f` | 0 | the plan |
| `bb89e4f8f` | A0 | FK's `retained_api` export (visibility only); cherry-picked onto V-K as `3018343c2`, same patch-id |
| `fdbf132d4` | A0 | H's test that the export suffices from outside FK |
| `73031b134` | A1 | the `w1a` mode, the adapter, W1 counts, attempt and prefix records, the W1 estimate |
| `f4d40dd17` | A2 | the runner's W1 tiers (K6's four modes frozen), `k6b_analysis.py`, counts for the 33 sealed models |
| `8bbcdfc8b` | C | the runner defers by name any row the binary's backstop would refuse |
| `1f5c1a6d0` | C | each prefix limit pinned to its segment's exact end (kills M10) |
| `b86081221` | — | main `0f5d8c7b4` (KF1) merged in |
| `082990c8d` | — | E_max follows KF1's bounded trackers at every site; counts regenerated |
| `4eeb206c0` | b3 fix | the stage-identity check on stopped builds; Span outcomes recorded, not stops |

- **D (this checkpoint, proposed next commit):** `H/runner/k6b_analysis.py` gains `--packet` (the plan's §4 packet, not built at A2), with one runner test; `H/observations/k6b/k6b_packet.json` and `SHA256SUMS`; two README sentences; and `T3/IMPLEMENTATION/K6B/` (this RETURN, `CHANGE_RECORD.md`, `_run_records/`, `SHA256SUMS`).

## 2. Files and line counts (against main `0f5d8c7b4`, D included)

| File | Lines | Note |
|---|---|---|
| `FK/structural.rs` | +29 | the `retained_api` facade (A0) |
| `FK/structural/retained/*.rs` (10 files) | +240 −260 | `pub(crate)` → `pub` and 20 dead-code markers removed (A0); nothing else |
| `H/src/k6/w1/{mod,adapter,counts,rows,staged}.rs` | 15, 92, 477, 199, 333 | new: the adapter, W1 counts and estimate, row keys and R1's quantities, the staged sequence and the work accounting |
| `H/src/k6/{mod,counts,staged}.rs` | +6, +54 −1, +22 | `Mode::W1a`; W1 counts in `K6Counts`; W1 stages |
| `H/src/bin/k6_observe/main.rs`, `w1.rs` | +220 −10, 283 | the `w1a` branch, its CLI and lines |
| `H/tests/k6b_{adapter,bin,export,w1}.rs`, `k6b_support/mod.rs` | 64, 230, 134, 487, 113 | new tests |
| `H/runner/k6_runner.py` | +43 −8 | W1 tiers, `W1_PAIR`, W1-T4's by-name deferral, the backstop check |
| `H/runner/k6b_analysis.py`, `k6b_sources.py` | 334, 129 | new: W1 figures, smoke, projection, packet; the independent K4SRC writer |
| `H/runner/test_k6_runner.py` | +231 −2 | runner tests (44 at A2 → 47 at D) |
| `H/observations/k6b/counts.jsonl`, `sources.txt` | 33, 35 | counts (post-KF1 estimate) and K4SRC digests |
| `H/observations/k6b/k6b_packet.json`, `SHA256SUMS` | 26,516, 3 | D: b3's packet (W1-T4 pre-KF3) and the folder's hashes |
| `H/README.md` | +10 | the K6b section |

Totals in the product tree: Rust +2,729 lines (tests 1,028), Python +737, data 26,587, README +10. D itself adds 88 lines to `k6b_analysis.py`, 60 to `test_k6_runner.py`, the packet and `SHA256SUMS`, and two README sentences. `P/tests`, PP, SA, NI, SD, the fixtures and `.github` are untouched. No dependency or lockfile change.

## 3. Scope, item by item

1. **The export (A0).** "Raise to `pub`, and re-export from `FK/structural.rs` … exactly the items in K4 `RETURN.md` §16's export list … and nothing else."
   - As ruled (C-1): a facade `pub mod retained_api { pub use super::retained::…; }` right after the private `mod retained;`, so K4's names (`adaptive::POLICY` included) enter no other namespace.
   - **72 items, 38 methods and 130 fields** (§13.1 lists them). 240 `pub(crate)` → `pub` edits, 20 `#[allow(dead_code)]` markers removed. Visibility only.
   - Still crate-private (C-2 and Q8): `PrecisionState` and `RetainedSolve::state`; the fields of `PrimitiveSource`, `RetainedSolve`, `CaseLimit`, `InvocationMeter`, `AttemptWork`, `WidthWork` and `SumWork`; `Kind::{ALL, index}`; everything on §16's "Not exported".
   - **Scan** (`_run_records/a0/scan.txt` at A0, `_run_records/d/scan_d.txt` at D): `retained_api` is named only by `FK/structural.rs`, H's sources and tests, and H's README. The only `Cargo.toml` depending on H is H's own. FK's diff against main has the same patch-id as A0's commit `bb89e4f8f`.
   - FK's full suite: 394 passed, 0 warnings, rustfmt clean (`_run_records/a0/fk_full.log`). I17 confirmed the export covers V-K.
2. **The W1 mode.** `Mode::W1a` (`"w1a"`, `materializes_n2() = false`); the adapter `K6Model` → `SourceParts` (§3.2 of the plan); the staged sequence `w1_source`, `w1_solve` over `solve_case`; the counts line's W1 keys; the W1 estimate E_max and E_sel128 (§9.1); the runner's `MODES`, W1 tiers and `estimate_key` (unchanged, `estimate_adm_bytes_w1a`). K6's allocator, stage observer, watchdog, `launch` and record schema are reused unchanged; the new JSONL kinds are `attempt` and `prefix` (Q2, as ruled).
3. **What each run records.** The outcome and its attempts; for a selected case the publication digest, the row classes and **R1's unchanged predicate** (in CI at 10 and 100 members from `K4T/r1_large.txt`; in the records at every size where R1 publishes values, §6.4); work by stage and precision per attempt; storage counts; the heap peaks of the call and of each budget-truncated prefix; stage times; RSS and footprint through the runner.
4. **Models, sizes and the ascent.** RF-LARGE (three families, both orientations) at 10, 100, 1,000 and 10,000 members, and the DEC-053 nine (all covered by W1a: Q4). The ascent: each size after the previous one recorded, admission re-evaluated. 10,000 members only as ROOT approved (b2, then b3).
5. **Seconds per work unit.** Interleaved per model in one slot: w1a and K6's sparse alternate ABAB (even model index) or BABA (odd). s/LME is the median `w1_solve` time over `meter_charged`, with the load recorded (§8.1).

## 4. Checkpoint-0 positions, as ruled

- **Q1 (memory per precision): (a) with (c).** The whole-call peak, plus budget-truncated prefixes through the public API (`CaseLimit` at each segment's end b_j), in pass 1. The 512 and 1024 terms stay derived; no escalating model.
- **Q2:** the mode is `w1a`; new kinds `attempt` and `prefix`; K6's kinds keep their schema.
- **Q3:** four tiers; `CaseLimit` and `InvocationMeter` both `u64::MAX`, recorded in `start`; a budget outcome is a stop; W1 counts-only runs at 1,000 and 10,000 members during A2 (512 MiB cap).
- **Q4:** all nine DEC-053 models are covered by W1a (straight frames, rigid restraints, nodal forces).
- **Q5:** ABAB/BABA pairs, 5 repeats, prefixes in pass 1 only, s/LME = median `w1_solve` / `meter_charged`.
- **Q6:** (a) the adapter's K4SRC against an independent Python build from R1's `references.py --model` (24 of 24 equal, `_run_records/a1/adapter_check.out`) and (c) the CI chain `runner/k6b_sources.py` → `observations/k6b/sources.txt` → the Rust FNV-1a check.
- **Q7:** as planned; K6b's R1 check is not the honesty check (K4's and V-K's are), but a failure is still a stop.
- **Conflicts recorded:** K6's value-based profile cannot stand for W1's structural storage (C-5); a wide value is 8L + 16 bytes (C-4); RF-LARGE's W1 memory is dominated by the per-member operators; Iy ↔ Iz is equivalent here (M3e); `solve_cases` has an equality test only (C-9).

## 5. The schedule as run

`plan.txt` (A2, sha256 `091ee187…`) lists 270 rows: K6's 138, unchanged, and W1's 132 (W1-T1: RF-LARGE at 10 and the nine, 60 rows; W1-T2 at 100, 24; W1-T3 at 1,000, 24; W1-T4 at 10,000, 24). Each W1 row is one process; each model has four (w1a, sparse, w1a, sparse, or the reverse).

| Run | Source → binary sha256 | Slot, local time | Tiers | Processes | Result |
|---|---|---|---|---|---|
| **b** (pre-KF1) | `f4d40dd17` → `592c35a7…` | K6B-S1 and S2, 11:55:57–11:58:07 | W1-T1..T3 | 108, all ok, all admitted | accepted; W1-T4 then approved before KF1 |
| b, W1-T4 attempt | same | K6B-S3 | W1-T4 row 247 | 1 | **refused by the binary's backstop** (E_max 9.23 GB > heap cap / 2), no solve; voided (`VOIDED_w1t4_backstop_*`); W1-T4 deferred until KF1 |
| **b2** (post-KF1) | `082990c8d` → `4f55137b…` | K6B-S3, 16:05:55–16:08:32 | W1-T1..T3, then W1-T4 | 108 ok, then row 247 ok | **stopped:** row 247's parity item `w1_stages_equal_totals` was false (§7.3); record kept as the stop's evidence |
| **b3** (the result) | `4eeb206c0` → `20b67776…` | K6B-S4, 16:30:21–16:42:50 | W1-T1..T4 | **132, all ok, all admitted, 0 parity failures** | accepted; W1-T4 pre-KF3 |

- **Admission in b3:** every row admitted by K6's rule (ρ on the footprint, projected RSS ≤ 0.8 C, the ascent) and by the binary's backstop (E_adm ≤ heap cap / 2). No row was deferred. W1-T4's by-name deferral was lifted by the one recorded override in `run_w1t4.py`, as ROOT approved.
- **Caps:** C = 8 GiB (RSS watchdog), heap cap 7.5 GiB, `CaseLimit` = `InvocationMeter` = `u64::MAX`. No watchdog kill, heap-cap abort or budget outcome in any run. The memory guard ran throughout and logged no kill (`_run_records/d/guard_status.txt`).
- **The slot label in the records** is each tier's planned slot (W1-T1 and T2 "K6B-S1", W1-T3 "K6B-S2", W1-T4 "K6B-S3"). b2 ran in K6B-S3 and b3 in K6B-S4.
- **Load:** an unrelated long-running external process (one core) kept the 1-minute load at 3.4–6.0 throughout, as ROOT recorded; it is recorded per run, not waited out.

## 6. Results (b3; W1-T4 pre-KF3)

All tables are generated from the records by `_run_records/d/d_tables.py.txt`. `_run_records/d/tables.md` holds each table in full, and the packet (`H/observations/k6b/k6b_packet.json`) holds every run of both passes.

### 6.1 W1 per model and size (pass 1; the 10,000-member rows are pre-KF3)

RF-LARGE:

| model | members | outcome | attempts | charged LME | stop rule LME (% of call) | shift fact. | profile | pattern | rows | heap MiB | footprint MiB | RSS MiB | call s (median) | ns/LME | load | E_adm MiB | heap/E | rho_fp |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| CHAIN-n00010-AX | 10 | Selected 128 | 128 Accepted, 256 Verified | 9129471 | 1126845 (12.3%) | 1 | 534 | 1116 | 263 | 1.0 | 3.6 | 5.5 | 0.007 | 0.806 | 4.01 | 21.7 | 0.0515 | 0.1147 |
| CHAIN-n00010-ROT | 10 | Selected 128 | 128 Accepted, 256 Verified | 11244502 | 1109238 (9.9%) | 1 | 534 | 1116 | 263 | 1.0 | 3.5 | 5.4 | 0.015 | 1.291 | 4.01 | 21.7 | 0.0513 | 0.1118 |
| TREE-n00010-AX | 10 | Selected 128 | 128 Accepted, 256 Verified | 9088175 | 1298010 (14.3%) | 1 | 534 | 1116 | 263 | 1.0 | 3.4 | 5.3 | 0.008 | 0.892 | 4.01 | 21.7 | 0.0454 | 0.1075 |
| TREE-n00010-ROT | 10 | Selected 128 | 128 Accepted, 256 Verified | 10957856 | 1075268 (9.8%) | 1 | 534 | 1116 | 263 | 0.9 | 3.4 | 5.3 | 0.015 | 1.327 | 4.01 | 21.7 | 0.0425 | 0.1053 |
| CONT-n00010-AX | 10 | Selected 128 | 128 Accepted, 256 Verified | 7103333 | 1473268 (20.7%) | 0 | 297 | 1116 | 278 | 1.2 | 5.4 | 7.3 | 0.007 | 0.932 | 4.08 | 21.6 | 0.0623 | 0.2004 |
| CONT-n00010-ROT | 10 | Selected 128 | 128 Accepted, 256 Verified | 9806965 | 1545960 (15.8%) | 1 | 297 | 1116 | 278 | 1.2 | 4.0 | 5.9 | 0.012 | 1.226 | 4.08 | 21.6 | 0.0622 | 0.1360 |
| CHAIN-n00100-AX | 100 | Selected 128 | 128 Accepted, 256 Verified | 89784810 | 8505628 (9.5%) | 1 | 5664 | 10836 | 2513 | 10.9 | 16.5 | 18.4 | 0.072 | 0.798 | 4.38 | 46.0 | 0.2605 | 0.3353 |
| CHAIN-n00100-ROT | 100 | Selected 128 | 128 Accepted, 256 Verified | 111923940 | 8486914 (7.6%) | 1 | 5664 | 10836 | 2513 | 10.9 | 16.5 | 18.3 | 0.145 | 1.297 | 4.38 | 46.0 | 0.2604 | 0.3343 |
| TREE-n00100-AX | 100 | Selected 128 | 128 Accepted, 256 Verified | 90125790 | 10647843 (11.8%) | 1 | 5664 | 10836 | 2513 | 9.4 | 13.5 | 15.3 | 0.077 | 0.857 | 4.19 | 46.0 | 0.2093 | 0.2687 |
| TREE-n00100-ROT | 100 | Selected 128 | 128 Accepted, 256 Verified | 108591655 | 8460174 (7.8%) | 1 | 5664 | 10836 | 2513 | 8.8 | 13.3 | 15.1 | 0.138 | 1.269 | 4.19 | 46.0 | 0.1976 | 0.2638 |
| CONT-n00100-AX | 100 | Selected 128 | 128 Accepted, 256 Verified | 70051193 | 13849213 (19.8%) | 0 | 3132 | 10836 | 2663 | 10.7 | 18.9 | 20.7 | 0.059 | 0.847 | 4.17 | 45.3 | 0.2587 | 0.3932 |
| CONT-n00100-ROT | 100 | Selected 128 | 128 Accepted, 256 Verified | 93367469 | 14709661 (15.8%) | 1 | 3132 | 10836 | 2663 | 10.7 | 15.9 | 17.7 | 0.114 | 1.224 | 4.17 | 45.3 | 0.2587 | 0.3261 |
| CHAIN-n01000-AX | 1000 | Selected 128 | 128 Accepted, 256 Verified | 1013371056 | 198796321 (19.6%) | 1 | 56964 | 108036 | 25013 | 85.8 | 112.5 | 121.2 | 0.785 | 0.775 | 4.17 | 289.3 | 0.2967 | 0.3850 |
| CHAIN-n01000-ROT | 1000 | Selected 128 | 128 Accepted, 256 Verified | 1236879507 | 198702925 (16.1%) | 1 | 56964 | 108036 | 25013 | 85.8 | 112.5 | 128.2 | 1.523 | 1.232 | 4.34 | 289.3 | 0.2967 | 0.3850 |
| TREE-n01000-AX | 1000 | Selected 128 | 128 Accepted, 256 Verified | 922653051 | 121940308 (13.2%) | 1 | 56964 | 108036 | 25013 | 85.9 | 124.9 | 126.7 | 0.800 | 0.867 | 4.59 | 289.4 | 0.2968 | 0.4279 |
| TREE-n01000-ROT | 1000 | Selected 128 | 128 Accepted, 256 Verified | 1114735751 | 109153161 (9.8%) | 1 | 56964 | 108036 | 25013 | 86.0 | 126.0 | 127.8 | 1.393 | 1.250 | 5.13 | 289.5 | 0.2971 | 0.4315 |
| CONT-n01000-AX | 1000 | Selected 128 | 128 Accepted, 256 Verified | 806819446 | 245487770 (30.4%) | 0 | 31482 | 108036 | 26513 | 81.2 | 114.0 | 115.8 | 0.608 | 0.753 | 6.01 | 282.0 | 0.2879 | 0.4003 |
| CONT-n01000-ROT | 1000 | Selected 128 | 128 Accepted, 256 Verified | 1042064338 | 254278921 (24.4%) | 1 | 31482 | 108036 | 26513 | 81.3 | 117.5 | 119.3 | 1.198 | 1.150 | 5.41 | 282.1 | 0.2881 | 0.4127 |
| CHAIN-n10000-AX | 10000 | Unresolved(ExactSumSpan) | 128 Rejected, 256 Failed | 6835124231 | 0 (0.0%) | - | 569964 | 1080036 | - | 831.7 | 889.1 | 986.1 | 4.968 | 0.727 | 5.46 | 2724.2 | 0.3053 | 0.3260 |
| CHAIN-n10000-ROT | 10000 | Unresolved(ExactSumSpan) | 128 Rejected, 256 Failed | 8386149922 | 0 (0.0%) | - | 569964 | 1080036 | - | 831.7 | 886.4 | 983.5 | 10.092 | 1.203 | 4.79 | 2724.2 | 0.3053 | 0.3250 |
| TREE-n10000-AX | 10000 | Unresolved(ExactSumSpan) | 128 Rejected, 256 Failed | 6674734550 | 0 (0.0%) | - | 569964 | 1080036 | - | 832.5 | 874.6 | 978.2 | 5.177 | 0.776 | 3.68 | 2725.1 | 0.3055 | 0.3205 |
| TREE-n10000-ROT | 10000 | Unresolved(ExactSumSpan) | 128 Rejected, 256 Failed | 8194818095 | 0 (0.0%) | - | 569964 | 1080036 | - | 833.9 | 863.7 | 963.9 | 9.555 | 1.166 | 3.66 | 2726.8 | 0.3058 | 0.3163 |
| CONT-n10000-AX | 10000 | Selected 128 | 128 Accepted, 256 Verified | 8234772320 | 2596783796 (31.5%) | 0 | 314982 | 1080036 | 265013 | 814.0 | 995.4 | 1061.4 | 6.090 | 0.740 | 4.54 | 2649.1 | 0.3073 | 0.3753 |
| CONT-n10000-ROT | 10000 | Unresolved(ExactSumSpan) | 128 Rejected, 256 Failed | 6884609317 | 0 (0.0%) | - | 314982 | 1080036 | - | 814.9 | 903.8 | 965.9 | 8.134 | 1.181 | 4.94 | 2650.3 | 0.3075 | 0.3406 |

The DEC-053 nine:

| model | members | outcome | attempts | charged LME | stop rule LME (% of call) | shift fact. | profile | pattern | rows | heap MiB | footprint MiB | RSS MiB | call s (median) | ns/LME | load | E_adm MiB | heap/E | rho_fp |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| DEC053 cantilever-chain-8 | 8 | Selected 128 | 128 Accepted, 256 Verified | 7295927 | 1353844 (18.6%) | 1 | 420 | 900 | 213 | 0.9 | 3.3 | 5.2 | 0.005 | 0.668 | 4.08 | 21.1 | 0.0455 | 0.1043 |
| DEC053 cantilever-chain-24 | 24 | Selected 128 | 128 Accepted, 256 Verified | 21942177 | 3806363 (17.3%) | 1 | 1332 | 2628 | 613 | 2.7 | 9.2 | 11.0 | 0.015 | 0.703 | 4.08 | 25.4 | 0.1171 | 0.3175 |
| DEC053 cantilever-chain-48 | 48 | Selected 128 | 128 Accepted, 256 Verified | 43931984 | 7542846 (17.2%) | 1 | 2700 | 5220 | 1213 | 5.4 | 8.7 | 10.6 | 0.030 | 0.672 | 4.08 | 31.9 | 0.1845 | 0.2373 |
| DEC053 grid-frame-4x3 | 17 | Selected 128 | 128 Accepted, 256 Verified | 11544479 | 1586088 (13.7%) | 1 | 636 | 1656 | 414 | 1.4 | 4.1 | 6.0 | 0.010 | 0.863 | 4.08 | 23.1 | 0.0604 | 0.1276 |
| DEC053 grid-frame-6x8 | 82 | Selected 128 | 128 Accepted, 256 Verified | 85805999 | 6397333 (7.5%) | 1 | 7758 | 7632 | 1848 | 6.9 | 12.6 | 14.5 | 0.078 | 0.904 | 4.08 | 40.2 | 0.1703 | 0.2860 |
| DEC053 grid-frame-7x8 | 97 | Selected 128 | 128 Accepted, 256 Verified | 106575518 | 7441411 (7.0%) | 1 | 9849 | 9000 | 2180 | 8.2 | 14.3 | 16.2 | 0.097 | 0.906 | 4.24 | 44.3 | 0.1860 | 0.2989 |
| DEC053 grid-frame-5x5 | 40 | Selected 128 | 128 Accepted, 256 Verified | 34879241 | 3317605 (9.5%) | 1 | 2760 | 3780 | 925 | 3.2 | 6.4 | 8.3 | 0.031 | 0.884 | 4.24 | 29.1 | 0.1096 | 0.1816 |
| DEC053 cantilever-chain-32 | 32 | Selected 128 | 128 Accepted, 256 Verified | 29473702 | 5245362 (17.8%) | 1 | 1788 | 3492 | 813 | 3.4 | 6.6 | 8.4 | 0.020 | 0.672 | 4.24 | 27.6 | 0.1322 | 0.1975 |
| DEC053 grid-frame-5x6 | 49 | Selected 128 | 128 Accepted, 256 Verified | 45338910 | 4048499 (8.9%) | 1 | 3765 | 4608 | 1122 | 4.0 | 7.6 | 9.5 | 0.041 | 0.897 | 4.24 | 31.4 | 0.1278 | 0.2057 |

- **Outcomes:** at 10 to 1,000 members and on the nine, every w1a process selects 128 and verifies at 256 (56 of 66 processes). **At 10,000 members (pre-KF3), only CONT-AX selects**; CHAIN-AX and -ROT, TREE-AX and -ROT, and CONT-ROT end `Unresolved(ExactSumSpan)` in the 256 verification (§7.3). Pass 2 repeats every outcome, charged work and heap peak exactly.
- **Storage:** every attempt's `StorageCounts` equals W1's counts (the parity items `w1_profile_equals_storage`, `w1_pattern_equals_storage`, `w1_limbs_equal_table`), with 4, 4, 8 and 16 limbs per entry at 128, 256, 512 and 1024.
- **5a.3's shift:** one shifted factorization in each verification, except CONT-AX at every size (none needed).

### 6.2 Work by precision and part (pass 1, repeat 0; LME) at 1,000 and 10,000 members (10,000 pre-KF3)

| model | p | role | outcome | solve | stop rule | pass | build | v-build (bounded / wide / uc) | unstaged | charged by |
|---|---|---|---|---|---|---|---|---|---|---|
| CHAIN-n01000-AX | 128 | Candidate | Accepted | 26108889 | 198796321 | 0 | 214708433 | 0 / 0 / 0 | 0 | 439613643 |
| CHAIN-n01000-AX | 256 | Verification | Verified | 29805515 | 0 | 115522606 | 281559094 | 36006828 / 93938498 / 16924872 | 0 | 573757413 |
| CHAIN-n01000-ROT | 128 | Candidate | Accepted | 37327954 | 198702925 | 0 | 245137500 | 0 / 0 / 0 | 0 | 481168379 |
| CHAIN-n01000-ROT | 256 | Verification | Verified | 49688186 | 0 | 156136850 | 335491939 | 36006828 / 131921494 / 46465831 | 0 | 755711128 |
| TREE-n01000-AX | 128 | Candidate | Accepted | 29755752 | 121940308 | 0 | 201668945 | 0 / 0 / 0 | 0 | 353365005 |
| TREE-n01000-AX | 256 | Verification | Verified | 34514082 | 0 | 120255600 | 268539435 | 36006828 / 93940248 / 16031853 | 0 | 569288046 |
| TREE-n01000-ROT | 128 | Candidate | Accepted | 38303000 | 109153161 | 0 | 227821836 | 0 / 0 / 0 | 0 | 375277997 |
| TREE-n01000-ROT | 256 | Verification | Verified | 52395910 | 0 | 156156558 | 318302103 | 36006828 / 131877608 / 44718747 | 0 | 739457754 |
| CONT-n01000-AX | 128 | Candidate | Accepted | 18029964 | 245487770 | 0 | 142941410 | 0 / 0 / 0 | 0 | 406459144 |
| CONT-n01000-AX | 256 | Verification | Verified | 20759423 | 0 | 28837176 | 209663750 | 36006828 / 93938498 / 11154627 | 0 | 400360302 |
| CONT-n01000-ROT | 128 | Candidate | Accepted | 27678112 | 254278921 | 0 | 173095299 | 0 / 0 / 0 | 0 | 455052332 |
| CONT-n01000-ROT | 256 | Verification | Verified | 34941394 | 0 | 94782413 | 263295499 | 36006828 / 131921494 / 26064378 | 0 | 587012006 |
| CHAIN-n10000-AX | 128 | Candidate | Rejected(VerificationFailed) | 259517786 | 0 | 0 | 2147746443 | 0 / 0 / 0 | 0 | 2407264229 |
| CHAIN-n10000-AX | 256 | Verification | Failed(Stop(Span)) | 297507618 | 0 | 0 | 2816190317 | 360060828 / 939384998 / 0 | 14716241 | 4427860002 |
| CHAIN-n10000-ROT | 128 | Candidate | Rejected(VerificationFailed) | 373220067 | 0 | 0 | 2452036614 | 0 / 0 / 0 | 0 | 2825256681 |
| CHAIN-n10000-ROT | 256 | Verification | Failed(Stop(Span)) | 496996537 | 0 | 0 | 3355514267 | 360060828 / 1319214994 / 0 | 29106615 | 5560893241 |
| TREE-n10000-AX | 128 | Candidate | Rejected(VerificationFailed) | 296112910 | 0 | 0 | 2017353381 | 0 / 0 / 0 | 0 | 2313466291 |
| TREE-n10000-AX | 256 | Verification | Failed(Stop(Span)) | 343729014 | 0 | 0 | 2686092122 | 360060828 / 939402498 / 0 | 31983797 | 4361268259 |
| TREE-n10000-ROT | 128 | Candidate | Rejected(VerificationFailed) | 382991767 | 0 | 0 | 2314419160 | 0 / 0 / 0 | 0 | 2697410927 |
| TREE-n10000-ROT | 256 | Verification | Failed(Stop(Span)) | 524096964 | 0 | 0 | 3219154324 | 360060828 / 1318776233 / 0 | 75318819 | 5497407168 |
| CONT-n10000-AX | 128 | Candidate | Accepted | 178178964 | 2596783796 | 0 | 1429481990 | 0 / 0 / 0 | 0 | 4204444750 |
| CONT-n10000-AX | 256 | Verification | Verified | 233076239 | 0 | 289109150 | 2096643784 | 360060828 / 939384998 / 112052571 | 0 | 4030327570 |
| CONT-n10000-ROT | 128 | Candidate | Rejected(VerificationFailed) | 276625079 | 0 | 0 | 1811687868 | 0 / 0 / 0 | 0 | 2088312947 |
| CONT-n10000-ROT | 256 | Verification | Failed(Stop(Span)) | 349350963 | 0 | 0 | 2713626046 | 360060828 / 1319214994 / 0 | 54043539 | 4796296370 |

Parts: solve = rhs + solve + refinement + recovery; build = formation + assembly + residual formation + factor + condition; pass = the verification pass (scale, estimate, charge, bound, shift); v-build = the verification's shared data; unstaged = charged work no stage records (§7.3). The 10- and 100-member rows and every stage are in `tables.md` and the packet.

### 6.3 Memory per precision: the prefixes (pass 1; 10,000 pre-KF3)

Heap above the call's start (MiB), with the increment over the previous prefix, and the prefix call's time:

| model | solve_128 | solve_256 | verify_256 | call | last segment | stop-rule increment | kept-entry equivalent |
|---|---|---|---|---|---|---|---|
| CHAIN-n01000-AX | 35.7 (+35.7), 0.17 s | 61.8 (+26.1), 0.37 s | 85.3 (+23.5), 0.65 s | 85.3, 0.79 s | 0.0 | 0.0 | 0 |
| CHAIN-n01000-ROT | 35.7 (+35.7), 0.41 s | 61.8 (+26.1), 0.82 s | 85.3 (+23.5), 1.37 s | 85.3, 1.52 s | 0.0 | 0.0 | 0 |
| TREE-n01000-AX | 35.7 (+35.7), 0.20 s | 61.8 (+26.1), 0.38 s | 85.4 (+23.5), 0.68 s | 85.4, 0.80 s | 0.0 | 0.0 | 0 |
| TREE-n01000-ROT | 35.8 (+35.8), 0.35 s | 61.9 (+26.1), 0.74 s | 85.5 (+23.5), 1.26 s | 85.5, 1.39 s | 0.0 | 0.0 | 0 |
| CONT-n01000-AX | 34.5 (+34.5), 0.11 s | 60.6 (+26.1), 0.24 s | 80.6 (+20.0), 0.45 s | 80.6, 0.61 s | 0.0 | 0.0 | 0 |
| CONT-n01000-ROT | 34.6 (+34.6), 0.27 s | 60.7 (+26.1), 0.60 s | 80.7 (+20.0), 0.97 s | 80.7, 1.20 s | 0.0 | 0.0 | 0 |
| CHAIN-n10000-AX | 351.6 (+351.6), 1.69 s | 612.6 (+261.0), 3.74 s | - | 826.0, 4.97 s | 213.4 | - | - |
| CHAIN-n10000-ROT | 351.6 (+351.6), 3.82 s | 612.6 (+261.0), 8.06 s | - | 826.0, 10.09 s | 213.4 | - | - |
| TREE-n10000-AX | 352.0 (+352.0), 2.00 s | 613.0 (+261.0), 3.86 s | - | 826.4, 5.18 s | 213.4 | - | - |
| TREE-n10000-ROT | 352.8 (+352.8), 3.49 s | 613.8 (+261.0), 7.51 s | - | 827.2, 9.56 s | 213.4 | - | - |
| CONT-n10000-AX | 346.1 (+346.1), 1.10 s | 607.1 (+261.0), 2.47 s | 807.4 (+200.2), 4.52 s | 807.4, 6.09 s | 0.0 | 0.0 | 0 |
| CONT-n10000-ROT | 346.7 (+346.7), 2.78 s | 607.7 (+261.0), 6.08 s | - | 807.9, 8.13 s | 200.2 | - | - |

- A prefix j stops at segment j + 1's first budget check (`Unresolved(Budget(Case))`); the parity item `w1_prefix_segments` checks that its completed segments equal the full call's. Each prefix limit equals its segment's exact end (the M10 test).
- At 1,000 members the three segments take about 35, 26 and 20–24 MiB. At 10,000 members they take about 346–353, 261 and 200 MiB. **Heap per precision is linear in members.**
- **After KF1 the stop rule adds no heap above the verification's peak** at 1,000 members or on CONT-AX at 10,000 (call peak = verify_256 prefix peak). For the five Span rows the call's last segment is the failed verification build (213.4 MiB, 200.2 MiB on CONT-ROT), not a stop rule, and the analysis reports no stop-rule figure for them (`k6b_analysis.w1_figures`, fixed at `4eeb206c0`).
- 512 and 1024 were never reached, so their terms are derived only (E_max's `shared`, `state` and `verify` terms, in each counts line).

### 6.4 R1's predicate on the published rows

`_run_records/b3/r1_compare.out` (`k6b_r1_compare.py`, exact `Fraction` arithmetic, R1's `references.json` `7b176dbb…`): **10,293 comparisons on the 19 pass-1 selected RF-LARGE dumps, 0 failures or missing**, worst 1.75e-6 of the allowance. It covers every R1 value at 10 and 100 members, and R1's published subset at 1,000 and 10,000 (CONT-AX, 215 values). b (pre-KF1) gave 10,078 comparisons, 0 failures. In CI, `tests/k6b_w1.rs` checks every R1 row at 10 and 100 members from `K4T/r1_large.txt` (sha256 asserted).

### 6.5 Parity and determinism

- b3: 1,668 parity lines, 0 false. The W1 items are `w1_profile_equals_storage`, `w1_pattern_equals_storage`, `w1_limbs_equal_table`, `w1_stages_equal_totals`, `w1_work_closes`, `w1_source_encoding_equals_evidence` (selected cases), `w1_budget_not_reached`, `repeat_determinism` and `w1_prefix_segments`.
- `w1_work_closes`: on every run the attempts' charges add up to the meter's, including the stopped builds (row 247's 6,835,124,231 LME).
- **N-4 (a cross-check, not a stop):** each of b3's 66 sparse processes has K6's recorded heap peaks minus a constant 141 B, at every model and size (`_run_records/d/n4_sparse_crosscheck.out`). The constant does not depend on the model; it is consistent with the path strings the two runners pass (the records scrub them).

## 7. The claims, and the findings

### 7.1 Claim 1: growth fits (log-log least squares against members; the packet's `fits`, both passes, 10 to 10,000 members; the 10,000-member points pre-KF3)

| family/mode | heap-move slope | points | net-RSS slope |
|---|---|---|---|
| CHAIN/sparse | 0.999 | 16 | 0.816 |
| CHAIN/w1a | 0.948 | 16 | 0.812 |
| CONT/sparse | 1.002 | 16 | 0.778 |
| CONT/w1a | 0.919 | 16 | 0.793 |
| DEC053/sparse | 0.873 | 18 | 0.365 |
| DEC053/w1a | 0.881 | 18 | 0.572 |
| TREE/sparse | 0.998 | 16 | 0.822 |
| TREE/w1a | 0.979 | 16 | 0.824 |

- **W1's heap grows with slope 0.92–0.98 per family, and K6's sparse with 1.00.** The w1a points at 10,000 members are to the Span failure for five models: their heap peaks are 831.7–833.9 MiB (CHAIN, TREE) and 814.9 MiB (CONT-ROT), against 814.0 MiB for CONT-AX's full selected call. RSS slopes are lower because the process baseline dominates at 10 and 100 members (K6's constraint). The DEC-053 rows mix chains and grids and are not a family fit.

### 7.2 Claim 2: the measured heap against W1's estimate (10,000 members pre-KF3)

E_adm is E_max from `082990c8d` (KF1's bounded trackers). ρ_fp is max(footprint net of the no-op baseline, heap move) / E_adm, as K6's admission uses it:

| mode | size | n | rho_fp min–max | heap/E min–max | largest rho_fp |
|---|---|---|---|---|---|
| sparse | 10 | 6 | 2.729–3.058 | 0.639–0.725 | TREE-n00010-ROT |
| sparse | 100 | 6 | 0.995–1.594 | 0.586–0.768 | CONT-n00100-AX |
| sparse | 1000 | 6 | 0.846–1.271 | 0.667–0.796 | TREE-n01000-AX |
| sparse | 10000 | 6 | 0.721–1.753 | 0.642–0.731 | CHAIN-n10000-AX |
| sparse | DEC053 | 9 | 1.117–3.224 | 0.594–0.724 | DEC053 cantilever-chain-8 |
| w1a | 10 | 6 | 0.105–0.200 | 0.042–0.062 | CONT-n00010-AX |
| w1a | 100 | 6 | 0.264–0.393 | 0.198–0.260 | CONT-n00100-AX |
| w1a | 1000 | 6 | 0.385–0.432 | 0.288–0.297 | TREE-n01000-ROT |
| w1a | 10000 | 6 | 0.316–0.375 | 0.305–0.307 | CONT-n10000-AX |
| w1a | DEC053 | 9 | 0.104–0.317 | 0.045–0.186 | DEC053 cantilever-chain-24 |

- **W1's measured heap is 4–31% of E_max, and ρ_fp at most 0.43**, at every size from 10 to 10,000 members. At 10,000 members E_adm is 2,649–2,727 MiB (2.59–2.66 GiB, 2.78–2.86 GB) and the heap 814–834 MiB.
- **Correction:** my b3 report to ROOT gave E_adm at 10,000 members as "2.65–2.73 GiB", and ROOT's b3 ruling repeats it. The values are 2.59–2.66 GiB (2,649–2,727 MiB); I had divided MiB by 1,000.
- Before KF1, E_max at 10,000 members was 9.2–9.5 GB (the unbounded tracker term), and ρ at 100 and 1,000 members was 0.17–0.27 (b).
- The rows for sparse are K6's estimate, for the record (§7.5).

### 7.3 Findings

**F1. K4's stop-rule tracker memory (A1; routed to KF1, merged as PR #1056).**
- K4's `ExtremeTracker` kept every row within `WINDOW_ULPS` of the running extreme (two `ExactWideSum`s, 4,304 B each) until `decide` returned, so the stop rule's memory depended on the data. The worst case at 10,000 members was about 3.2 GB (6.4 GB with `Vec` slack). No published value was affected.
- b measured it at 1,000 members: 0–8,773 kept entries, at most 5.5% of the worst-case term, with the kept fraction not growing with size (CONT 0.39, 0.43 and 0.33 of rows at 10, 100 and 1,000).
- KF1 bounds the tracker (T = 512, G = 4,096). E_max now follows KF1's bounds at every site (`082990c8d`): the stop rule's G + T rows, the pivot margin's 1.5 T, a solve attempt's 2,816 rows and the fallback's per-state rows.
- **Before and after KF1 at 1,000 members:**

| model | stop rule LME b | b3 | call heap MiB b | b3 | stop-rule increment MiB b | b3 | kept equivalent b | b3 | call s b | b3 |
|---|---|---|---|---|---|---|---|---|---|---|
| CHAIN-n01000-AX | 82276385 | 198796321 | 102.1 | 85.3 | 16.8 | 0.0 | 4088 | 0 | 0.84 | 0.79 |
| CHAIN-n01000-ROT | 82182989 | 198702925 | 102.1 | 85.3 | 16.7 | 0.0 | 4075 | 0 | 1.58 | 1.52 |
| TREE-n01000-AX | 104014164 | 121940308 | 89.5 | 85.4 | 4.1 | 0.0 | 1005 | 0 | 0.79 | 0.80 |
| TREE-n01000-ROT | 82263945 | 109153161 | 85.5 | 85.5 | 0.0 | 0.0 | 0 | 0 | 1.38 | 1.39 |
| CONT-n01000-AX | 137930906 | 245487770 | 116.6 | 80.6 | 36.0 | 0.0 | 8773 | 0 | 0.56 | 0.61 |
| CONT-n01000-ROT | 146722057 | 254278921 | 116.7 | 80.7 | 36.0 | 0.0 | 8774 | 0 | 1.15 | 1.20 |

KF1 removes the stop rule's heap increment (up to 36 MiB here) and raises its work 1.2–2.4 ×; the call time is unchanged within the load.

**F2. The binary's backstop (b, W1-T4; fixed at C, `8bbcdfc8b`).**
- The runner admitted row 247 on E_max × measured ρ (≈ 1.7 GB), and the binary refused it (`estimate_exceeds_half_cap`: raw E_max 9.23 GB > 3.75 GiB). The backstop worked; A2 had not tested the runner's admission against it for a mode whose ρ is far below 1 (my miss).
- Ruled option (c): the backstop stays independent of the runner; `admission()` now defers by name any row the backstop would refuse, and a test asserts it for every admitted row (mutant M14). After KF1 the backstop admits all six W1-T4 models.

**F3. W1 at 10,000 members ends in `Span` on five of six models (b2, b3; pre-KF3; routed to KF3).**
- CHAIN-AX and -ROT, TREE-AX and -ROT and CONT-ROT: the 128 candidate completes its builds and is `Rejected(VerificationFailed)`; the 256 verification's shared build stops with `Failed(Stop(Span))` in its `uc` stage, after `bounded_formation` (360,060,828 LME) and `wide_formation` (939.4 M on AX, 1,319.2 M on ROT). The outcome is `Unresolved(ExactSumSpan)`, deterministic over 5 repeats and 2 passes. **Only CONT-AX selects (at 128).** This agrees model by model with V-K's B.
- ROOT's reading (`K4R/bound.rs:405-420`): `uc_bounds`' directed recurrences on a long chain grow the comparison-matrix bound geometrically past `ExactWideSum`'s span. Since B = min(Uc, S) per block and S is independently certified, KF3 will treat an unformable Uc as unavailable rather than stop the attempt. No published value changes; only whether W1 can publish.
- **K4's accounting on that error path** (`K4R/verify.rs:471-485`): `gamma_m`/`uc_bounds` return `Err` before `stages.uc` is recorded, while the build's total (`:500-504`) counts all its work, so the stage breakdown falls short of the charged total by the partial `uc` work: 14.7 M (CHAIN-AX), 29.1 M (CHAIN-ROT), 32.0 M (TREE-AX), 75.3 M (TREE-ROT) and 54.0 M (CONT-ROT) LME. The charge is right (`verify_precision` charges all of it). Routed to KF3.
- **K6b's parity check assumed the identity K4 guarantees only for completed builds** and stopped b2 (row 247). Fixed in H (`4eeb206c0`, ruling (a)): the stage identity is checked on completed builds; on a build K4 marked `Failed(Stop(_))` each stage sum is at most its charged total, and the remainder is reported on the attempt line as `own_unstaged` / `shared_unstaged`; the outcome's `work_p_own` and `work_p_shared` are the charged totals. A small-size test reaches the same path with a case limit inside the 256 verification's `uc` stage (unstaged 102,396 LME at 10 members), and six mutants are killed (§10).

### 7.4 CONT-AX at 10,000 members (the one selected case; pre-KF3)

- The call charges 8,234,772,320 LME, with a heap peak of 814.0 MiB (footprint 995 MiB, RSS 1,061 MiB) and a median call of 6.09 s (0.74 ns/LME), 7.6 × the binary64 sparse `entry_checked` (7.5 × at 1,000 members).
- **The 128 candidate (Accepted):** own 2,774,962,760 LME: rhs 538,712; solve 71,009,136; refinement 72,457,960; recovery 34,173,156; **stop rule 2,596,783,796 (31.5% of the call)**. Its shared build is 1,429,481,990 LME (formation 279.3 M, assembly 7.0 M, residual formation 291.1 M, factor 376.2 M, condition 475.8 M). Gate coalesced; pivot margin minimum 1.48e31; rcond 4.0e-9; worst residual 0.00273.
- **The 256 verification (Verified):** own 522,185,389 LME (the pass: scale 26.4 M, estimate 126.8 M, charge 135.9 M, bound 942; no shift); shared 3,508,142,181 LME, with `uc` complete at 112,052,571.
- **The stop rule's memory and time:** no heap above the verify_256 prefix (807.4 MiB = the call's peak), so its kept-entry equivalent is 0; about 1.57 s (the full call's 6.09 s less the verify_256 prefix's 4.52 s).
- 265,013 rows published: 148,718 relative-verified, 101,289 absolute-verified, 15,006 input-derived, 0 unpublishable. R1's 215 values at 10,000 members pass (§6.4).

### 7.5 A K6 observation (not a stop): the sparse footprint at 10,000 members

| model | pass | class | heap MiB | footprint MiB | RSS MiB | E_adm MiB | heap/E | rho_fp |
|---|---|---|---|---|---|---|---|---|
| CHAIN-n10000-AX | 1 | NumericallyUnresolved | 254.2 | 693.2 | 717.0 | 394.8 | 0.644 | 1.753 |
| CHAIN-n10000-AX | 2 | NumericallyUnresolved | 254.2 | 687.9 | 711.7 | 394.8 | 0.644 | 1.740 |
| CHAIN-n10000-ROT | 1 | NumericallyUnresolved | 277.1 | 285.8 | 340.7 | 394.8 | 0.702 | 0.721 |
| CHAIN-n10000-ROT | 2 | NumericallyUnresolved | 277.1 | 284.9 | 334.7 | 394.8 | 0.702 | 0.719 |
| TREE-n10000-AX | 1 | NumericallyUnresolved | 258.9 | 391.0 | 416.2 | 394.8 | 0.659 | 0.988 |
| TREE-n10000-AX | 2 | NumericallyUnresolved | 258.9 | 391.0 | 416.2 | 394.8 | 0.659 | 0.988 |
| TREE-n10000-ROT | 1 | NumericallyUnresolved | 286.2 | 570.4 | 667.9 | 394.8 | 0.731 | 1.442 |
| TREE-n10000-ROT | 2 | NumericallyUnresolved | 286.2 | 570.3 | 666.3 | 394.8 | 0.731 | 1.442 |
| CONT-n10000-AX | 1 | Sensitive | 243.4 | 341.0 | 365.7 | 379.4 | 0.642 | 0.896 |
| CONT-n10000-AX | 2 | Sensitive | 243.4 | 341.5 | 366.2 | 379.4 | 0.642 | 0.897 |
| CONT-n10000-ROT | 1 | Sensitive | 267.4 | 295.3 | 380.2 | 379.4 | 0.705 | 0.776 |
| CONT-n10000-ROT | 2 | Sensitive | 267.4 | 332.9 | 389.8 | 379.4 | 0.705 | 0.875 |

K6's sparse estimate gives ρ_fp up to 1.75 (CHAIN-AX) while heap/E is 0.64–0.73, so the excess is outside the heap: CHAIN-AX's footprint is 693 MiB against a 254 MiB heap. Every run stayed far below the caps. It is K6's mode and estimate, recorded for ROOT.

## 8. Observations for ROOT's W1 limits (with their load; no claim)

### 8.1 Time

- **s/LME:** 0.73–0.93 ns on the RF-LARGE AX models and 1.15–1.33 ns on the ROT models, at every size, and 0.67–0.91 ns on the nine (§6.1). The charged work, not the size, predicts the call time.
- **W1/binary64 multiple** (median `w1_solve` over K6's sparse `entry_checked` in the adjacent process; and over the sparse staged total; 10,000 members pre-KF3; the nine are in `tables.md`):

| model | pass 1 entry | pass 1 staged | pass 2 entry | pass 2 staged | W1 outcome |
|---|---|---|---|---|---|
| CHAIN-n00010-AX | 8.7 | 6.9 | 8.7 | 6.8 | Selected 128 |
| CHAIN-n00010-ROT | 11.4 | 9.5 | 11.8 | 9.6 | Selected 128 |
| CHAIN-n00100-AX | 9.4 | 7.4 | 9.4 | 7.4 | Selected 128 |
| CHAIN-n00100-ROT | 13.3 | 11.3 | 13.3 | 11.3 | Selected 128 |
| CHAIN-n01000-AX | 10.2 | 8.0 | 10.3 | 8.1 | Selected 128 |
| CHAIN-n01000-ROT | 14.1 | 11.8 | 13.9 | 11.7 | Selected 128 |
| CHAIN-n10000-AX | 13.7 | 9.2 | 13.8 | 9.4 | Unresolved(ExactSumSpan) |
| CHAIN-n10000-ROT | 22.6 | 16.3 | 22.8 | 16.3 | Unresolved(ExactSumSpan) |
| CONT-n00010-AX | 8.1 | 6.5 | 7.5 | 5.9 | Selected 128 |
| CONT-n00010-ROT | 10.7 | 8.9 | 10.4 | 8.7 | Selected 128 |
| CONT-n00100-AX | 7.3 | 5.8 | 8.1 | 6.5 | Selected 128 |
| CONT-n00100-ROT | 9.9 | 8.5 | 10.0 | 8.4 | Selected 128 |
| CONT-n01000-AX | 7.5 | 5.9 | 7.5 | 6.0 | Selected 128 |
| CONT-n01000-ROT | 10.4 | 8.8 | 10.5 | 8.9 | Selected 128 |
| CONT-n10000-AX | 7.6 | 6.0 | 7.6 | 6.1 | Selected 128 |
| CONT-n10000-ROT | 7.9 | 6.5 | 7.8 | 6.5 | Unresolved(ExactSumSpan) |
| TREE-n00010-AX | 9.0 | 7.1 | 9.0 | 7.2 | Selected 128 |
| TREE-n00010-ROT | 10.6 | 9.2 | 10.7 | 9.2 | Selected 128 |
| TREE-n00100-AX | 9.5 | 7.6 | 8.9 | 7.2 | Selected 128 |
| TREE-n00100-ROT | 11.3 | 9.8 | 11.4 | 9.0 | Selected 128 |
| TREE-n01000-AX | 9.5 | 7.7 | 9.6 | 7.7 | Selected 128 |
| TREE-n01000-ROT | 11.8 | 10.0 | 11.8 | 9.9 | Selected 128 |
| TREE-n10000-AX | 14.1 | 9.8 | 14.2 | 9.7 | Unresolved(ExactSumSpan) |
| TREE-n10000-ROT | 20.7 | 15.1 | 20.8 | 14.9 | Unresolved(ExactSumSpan) |

- At 10,000 members the Span rows' times are to the failure, so their multiples are not a selected-case figure.

### 8.2 Work

- At 1,000 members a selected call charges 0.81–1.24 G LME; at 10,000 members CONT-AX charges 8.23 G, and the Span rows 6.67–8.39 G before stopping.
- The stop rule is 7–32% of a selected call after KF1: 10–21% at 10 members, 8–20% at 100, 10–30% at 1,000, 7–19% on the nine, and 31.5% on CONT-AX at 10,000.

### 8.3 Memory

- W1's heap: about 0.085–0.09 MB per member (81–86 MiB at 1,000, 814–834 MiB at 10,000), 2.8–3.3 × K6's sparse heap at the same size.
- The shared builds dominate (§6.3), as the estimate's per-member operator terms predict (C-4/C-5).

## 9. Derivations

### 9.1 The W1 estimate (`H/src/k6/w1/counts.rs`, `estimate`)

- A wide value `Wide<L>` takes w_L = 8L + 16 bytes (48, 80, 144). Attempts run at (L, R) = (4, 4), (4, 8), (8, 16), (16, 16) for 128–1024; verifications at (L_P, W) = (4, 8), (8, 16), (16, 16).
- With m members, n DOFs, n_f free DOFs, nnz pattern entries, P profile entries, B blocks and `rows` published rows:
  - S(p) = m(164 w_L + 16) + nnz(w_L + w_R) + m(5 w_R + 8) + P w_L + n_f(2 w_L + 56) + B w_L, kept in the group cache;
  - T(p) = m(164 w_R + 16) when R ≠ L, a build transient;
  - U(p) = (n + 6m + rows) w_L, the state;
  - V(P) = nnz w_{L_P} + 144 m w_W + B(4 w_{L_P} + 8), kept;
  - the verification pass's transient: P w_{L_P} + 5 rows (w_{L_P} + 8) + 2 n_f w_{L_P};
  - **KF1's tracker terms** (row 4,304 B, table entry 40 B): the stop rule 4,608 × 4,304 + 3 × 2 × rows × 40; the pivot margin 768 × 4,304 + 2 n_f × 40 on each shared build; a solve attempt 2,816 × 4,304 + 2 × 5 n_f × 40 + 2 n_f × 4,304;
  - E_fix: the harness's model, the source twice, the case and the group.
- **E_max** is E_fix plus the maximum over the schedule's time order (128, 256, v256, 512, v512, 1024, v1024) of everything kept plus the current transient; **E_sel128** stops at v256. Transients are added on top of kept bytes, so the estimate bounds the peak rather than tracks it.
- `tests/k6b_w1.rs::the_estimate_equals_a_hand_derivation` re-derives every term for CHAIN-n00010-AX; mutants M6a and M6b are killed by it.

### 9.2 The staged sequence equals K4's own entries

`w1_solve` is exactly `solve_case(source, CaseLimit::new(Lc), &mut InvocationMeter::new(Li))` under the stage observer. `tests/k6b_w1.rs::the_staged_solve_equals_k4s_own_entries` checks that the publication, the evidence and the charged work equal K4's `solve_case` and `solve_cases` on the same source. The binary's `repeat_determinism` item checks the publication, evidence and charge across repeats.

### 9.3 No published byte changes

A0 is visibility only (§3 item 1; FK's diff against main has A0's patch-id). No crate other than H names `retained_api`, and no crate depends on H. So no product path can change, and T9 and the both-entry gate are not run (the brief's Gates).

### 9.4 Paths no observation reached

- 512 and 1024 attempts and verifications: RF-LARGE and the nine select at 128 (or stop in the 256 verification). Their memory and work terms are derived only.
- A budget outcome on a full call: never (limits `u64::MAX`); budget stops occur only in the deliberate prefixes.
- `Refused` and `SourceRefused` outcomes: none; every model is a valid source.
- `solve_cases` with several cases: no model has several (C-9); equality test only.
- A completed-build stage identity failure: none in any run.

## 10. The mutation table

Each mutant ran on a clean `git archive` of its base with the candidate's files copied over it, its own target under `<wt>/k6b-mut/`, deleted afterwards; NONE first. Rust mutants run H's K6b test binaries in debug; Python mutants the runner suite.

| Mutant | Change | Result | Killing test |
|---|---|---|---|
| NONE (C, base `f4d40dd17`) | the candidate | passes | — |
| M1 | the adapter drops the last member | killed | `the_adapters_bytes_equal_the_independent_python_bytes`, `the_encoding_is_deterministic_and_independent_of_list_order` |
| M2 | the adapter swaps the first two nodes | killed | `the_adapters_bytes_equal_the_independent_python_bytes` |
| M3 | the adapter passes Iy as J | killed | same |
| M3e | the adapter swaps Iy and Iz | **equivalent** (derived: Iy = Iz in every section used) | — |
| M4 | the per-precision work drops the shift stage | killed | `work_by_precision_files_each_attempt_under_its_own_precision` |
| M5a | an attempt's work filed under the next precision | killed | same |
| M5b | the limbs table's 256 reads 8 | killed | `w1a_prints_its_lines_and_every_parity_holds` |
| M6a | the estimate forms k_q at L instead of R | killed | `the_estimate_equals_a_hand_derivation` |
| M6b | the estimate omits V(P) | killed | same |
| M7a | the runner treats w1a as an n² mode | killed | `test_refusals_by_name`, `test_w1_admission_and_argv`, … |
| M7b | the runner ignores the measured ρ for w1a | killed | `test_every_admitted_row_passes_the_binarys_backstop` |
| M8 | the facade omits `StageWork` | killed (does not compile) | — |
| M9 | the W1 tiers lose their alternation | killed | `test_w1_tiers` |
| M10 | the prefix limit is b_j − 1 | survived at C; **killed** from `8bbcdfc8b` by the approved assertion (`1f5c1a6d0`) | `prefixes_stop_on_the_case_budget_after_each_segment` |
| M11 | s/LME divides by the own-stage work | killed | `test_w1_figures` |
| M12 | the row keys swap end i and end j | killed | the R1 predicate tests |
| M13 | W1's profile from a reduced adjacency | killed | `w1a_prints_its_lines_and_every_parity_holds` |
| M14 | the runner's admission omits the binary's backstop | killed | `test_every_admitted_row_passes_the_binarys_backstop` |
| NONE (fix, base `082990c8d`) | the parity-fix candidate | passes | — |
| M15 | the fix reverted: a stopped build held to the equality | killed | `a_build_that_stops_partway_is_checked_against_its_charged_total` |
| M16 | the inequality applied to completed builds too | killed | same |
| M17 | a stopped build passes unchecked | killed | same |
| M18 | the per-precision shared total is the stage sum | killed | same |
| M20 | the unstaged remainder swaps own and shared | killed | same |
| M19 | the analysis reports a stop-rule increment for an unresolved case | killed | `test_w1_figures` |

**24 mutants: 23 killed, 1 equivalent (M3e, derived), no survivor.** C's NONE and M10 were re-run from clean copies after the assertion (`_run_records/c/results_c2.jsonl`). D's packet builder (`--packet`) is covered by `test_k6b_packet` and had no mutant campaign.

## 11. Suites and H's debug suite time

| Point | H's Rust suite (`cargo test`, debug, `-j 4`, `RUST_TEST_THREADS=2`) | Runner suite | Pytest wrapper (with the DEC-050/053 pins) |
|---|---|---|---|
| before K6b (A1's base) | 55 tests, 65.3 s | — | — |
| A1 | 70, 78.8 s | 44 (A2) | 46 (A2) |
| after KF1 (`082990c8d`) | 70, 83.5 s | 45 | 47 |
| parity fix (`4eeb206c0`, `--all-targets`) | 71, 80.6 s | 46 | not re-run |
| D | no Rust change | **47 of 47** | not re-run (it builds with cargo; ROOT's gate runs it) |

H's debug suite grew by about 15 s, under the brief's two-minute threshold. Every build was warning-free and rustfmt-clean.

## 12. Toolchain and host

`RUSTUP_TOOLCHAIN=1.97.1` (rustc 1.97.1 `8bab26f4f` 2026-07-14, LLVM 22.1.6), `RUSTUP_AUTO_INSTALL=0`, `CARGO_INCREMENTAL=0`, `--offline --locked`, one cargo job at `-j 4`, `RUST_TEST_THREADS=2`, target `<wt>/k6b-target`, mutants under `<wt>/k6b-mut/`. Observation binaries are `--release` builds from a `git archive` of the exact commit (b: `<wt>/k6b-target/b`, b2 and b3 in their own targets). The memory guard (`<wt>/guard/memguard.sh`, floor 35%) ran throughout. Host: Apple M5 Max, 18 cores, 128 GiB, macOS 26.6.2 (25G83), Python 3.13.14.

## 13. Interface for ROOT's W1 limits, V-K and F2a

### 13.1 The export (`FK/structural.rs`, `pub mod retained_api`)

- **Items (72):**
  - `adaptive`: `absolute_bound`, `body_extent`, `classify`, `classify_rows`, `classify_rows_floored`, `coupled_scales`, `intensified_k`, `solve_case`, `solve_cases`, `stress_scale`, `threshold`; `AttemptOutcome`, `AttemptReason`, `AttemptRecord`, `AttemptRole`, `AttemptStop`, `BudgetScope`, `CaseLimit`, `CaseOutcome`, `GateTest`, `InvocationMeter`, `Publication`, `PublishedRow`, `Refusal`, `RetainedEvidence`, `RetainedSolve`, `RowClass`, `StageWork`, `StorageCounts`, `UnresolvedReason`, `VerificationSummary`; `FLOOR_RATIO_BITS`, `K_SQRT2_BITS`, `K_TWO_SQRT2_BITS`, `METHOD_TOKEN`, `POLICY`, `PRECISIONS`, `RCOND_LABEL`;
  - `combine`: `CombinationOutcome`, `CombinationReason`, `RetainedCombination`;
  - `factor`: `reverse_cuthill_mckee`, `BodyGeometry`; `ledger`: `LedgerRefusal`;
  - `recover`: `layout`, `End`, `Kind`, `QuantityId`, `QuantityMeta`;
  - `source`: `Component`, `Constraint`, `DirectionalSpring`, `Dof`, `MemberProperty`, `NodalLoad`, `PrimitiveSource`, `SourceError`, `SourceParts`, `Spring`, `SpringKind`, `Station`, `StraightMember`, `SupportGroup`;
  - `verify`: `e_hat`, `phi_512`, `resolution_hats`, `PHI_SCALE_BITS`; `wide::multi`: `AttemptWork`, `Binary64Outcome`, `WidthWork`; `wide`: `WideError`; `wide_sum`: `SumWork`.
- **Methods (38):** `CaseLimit::{new, get}`; `InvocationMeter::{new, charged, limit, exhausted}`; `RetainedSolve::{publish, evidence, source, selected_precision}`; `Component::{ALL, index, from_index}`; `Dof::{global, from_global}`; `PrimitiveSource::{new, nodes, members, springs, directional_springs, constraints, loads, stations, supports, node_count, dof_count, constraint, free_dofs, body_of_node, body_count, body_nodes, member_index, encoding, stiffness_encoding}`; `RetainedCombination::solve`; `Binary64Outcome::value`; `AttemptWork::limb_multiply_equivalents`; `SumWork::limb_multiply_equivalents`.
- **Fields (130):** 85 in `adaptive` (the output and evidence records, `StageWork`'s 19, `StorageCounts`, `AttemptRecord`, `VerificationSummary`, `Publication`, `PublishedRow`, `RetainedEvidence`), 41 in `source` (the input types) and 4 in `recover`. The list, item by item, is `_run_records/a0/a0_edit.log`.

### 13.2 The observation binary's `w1a` mode (`H/src/bin/k6_observe/`)

```
k6_observe --model <id> --mode w1a --heap-cap-bytes <n> [--repeats 5] [--time-budget-s <s>]
           [--first-repeat-limit-s 600] [--counts-file <f>] [--case-limit <u64>] [--invocation-limit <u64>]
           [--w1-prefixes] [--dump-published <f>]
k6_observe --emit-source --model <id>
```

- **Refusals:** the backstop refuses a run whose E_adm exceeds half the heap cap (exit 3, `estimate_exceeds_half_cap`); a counts file without W1 counts is refused (exit 2). w1a is never an n² mode.
- **JSONL** (schema `k6-observe-v1`; K6's kinds unchanged):
  - `start` gains `case_limit`, `invocation_limit`, `w1_prefixes`, `dump_published`;
  - `counts` gains the `w1_*` keys (source ok/error, nodes, members, stations, constraints, loads, DOFs, free DOFs, bodies, pattern and profile entries, half bandwidth, blocks, rows, limbs per entry and storage bytes by precision, wide bytes, K4SRC length and FNV-1a) and `estimate_adm_bytes_w1a`, `estimate_w1_sel128_bytes` and every term (`estimate_w1_{fixed,decide,shared_p,state_p,verify_P}`);
  - `stage`: `w1_source`, `w1_solve`, `w1_prefix_<j>`;
  - `outcome`: class (`Selected`, `Refused`, `Unresolved`, `SourceRefused`), selected and verification precision, reason, rows by class and digests (selected), attempts, `meter_charged`, `budget_reached`, and per precision `work_<p>_attempts`, `work_<p>_own`, `work_<p>_shared` (**charged totals**);
  - `attempt` (new): repeat, index, precision, role, outcome, residual basis, corrections, gate, pivot margin, rcond, worst residual, storage, `own_<19 stages>`, `shared_<19 stages>`, **`stages_complete`, `own_unstaged`, `shared_unstaged`**, `own_total`, `charged_by`, `shared_work`, `shared_built_here`, `stop_rule_work`, `verification_work`, `verification_shared_work`, `verification_shared_built_here`, and the verification summary (data blocks, shift factorizations, uc missing, g max, g violation);
  - `parity`: §6.5's items; `prefix` (new): prefix, segment, case limit, class, reason, attempts, `meter_charged`.
- **Rows dump** (`k6b-rows v1`): one line per published row: key, kind, body, class with bound bits, value bits or `underflow±`/`overflow±`.

### 13.3 H's library API (`H/src/k6/w1/`)

- `adapter::{source(&K6Model) -> Result<PrimitiveSource, SourceError>, source_parts, member_id, load_source_id, STATION_FRACTION}`.
- `counts::{compute(&K6Model) -> (W1Counts, Option<String>), estimate(&W1Counts, &W1SizeFacts) -> W1Estimate, W1SizeFacts::of_this_build, structural_profile, storage_bytes, limbs_per_entry, LIMBS_PER_ENTRY}` and the size constants (§9.1).
- `staged::{w1_source, w1_solve(PrimitiveSource, W1Limits, Stage, &mut dyn StageObserver) -> W1Solve, attempts_of, own_total, shared_total, charged_by, builds_completed, unstaged, stages_equal_totals, work_closes, work_by_precision -> Vec<PrecisionWork>, segments, prefix_limits, stage_fields, stage_sum}`. `PrecisionWork` carries the stage sums (`own`, `shared`) and the charged totals (`own_total`, `shared_total`).
- `rows::{r1_values, r1_passes, write_rows, class_counts, key, magnitude}`.

### 13.4 The runner and the analysis (`H/runner/`)

- `k6_runner.py`: `MODES = K6_MODES + ('w1a',)`, `W1_PAIR = ('w1a', 'sparse', 'w1a', 'sparse')`, tiers W1-T1 to W1-T4 (orders 139–270), `CONDITIONAL_TIERS` (W1-T4 deferred by name unless overridden), pass-1 `--dump-published`/`--w1-prefixes`, and the backstop check in `admission()`. Stops are unchanged: parity failures, watchdog or heap-cap kills on admitted runs, errors, binary refusals. **The outcome class is never a stop**, so a `Span`-unresolved w1a case is recorded and the tier continues.
- `k6b_analysis.py`: `--smoke`, `--project`, `--figures <jsonl>`, and `--packet --records <dir> [--note …]` (schema `k6b-packet-v1`: K6's packet of the records, plus `w1` per w1a process with figures, work by precision, repeat 0's attempts, ρ and heap/E, and `w1_pairs` with the W1/binary64 multiples). `observations/k6b/k6b_packet.json` is b3's packet, with its three notes (W1-T4 pre-KF3).
- `k6b_sources.py`: `--write`/`--check` of `observations/k6b/sources.txt`.

### 13.5 What ROOT's W1 limits can use now, and what they still need

- **Per family and size** (§6.1–§6.3, §8): the selected precision, work by stage and precision, storage, heap, footprint and RSS, s/LME with its load, the W1/binary64 multiple, and the estimate's ratios. Covered: RF-LARGE at 10, 100, 1,000 and (pre-KF3) 10,000 members, and the DEC-053 nine. Not covered: any model that escalates to 512 or 1024 (derived terms only), and any other family.
- **Still needed:**
  - **the post-KF3 W1-T4** (RETURN addendum 1): which of the five Span models then select, at what precision, and their work, heap and time (ROOT's b3 ruling names the post-KF3 figures for the limits);
  - measured 512 and 1024 memory and work (an escalating model; V-K's RF-RANGE and THIN cases are the nearest evidence);
  - product-level memory and time (V-P) and Linux;
  - whether the per-case limit also bounds time; K6b gives s/LME, so a work limit converts to a time at the recorded load.
- **For V-K:** the export is shared (A0 on both branches; whichever merges first carries it). K6b's six 10,000-member outcomes agree with V-K's B. The runner, the binary and the packet are reusable by path.
- **For F2a:** until KF3, the evidence's stage breakdown falls short of the charged total on a build stopped partway; K6b's attempt line reports the remainder. The retained-state digest (§4.1.8) is not reachable through the export (C-2); F2a asks for an accessor if it needs one.

## 14. What was not done

- No limit, threshold or ceiling is proposed (the brief).
- **W1-T4 post-KF3:** named as RETURN addendum 1.
- No 512 or 1024 measurement (Q1, as ruled); no product-level run (V-P); nothing on Linux; no V-K reference family, floor check or seeded fault; K6's ceiling and grid runs are out of scope.
- The plan's N-5 (W1 displacements against binary64 as context) was not run. K6's crosscheck was not re-run (Q6): the adapter check builds from `references.py --model` directly.
- The pytest wrapper was not re-run after the parity fix or D (it builds with cargo).

## 15. Disclosures

- **My misses:** A2 tested the runner's admission but not the binary's backstop for w1a (F2); M10 survived C until the approved assertion; the parity item assumed an identity K4 holds only for completed builds (F3); the plan's packet was not built until D.
- **Records choices:** b2 keeps row 247 and the logs; its W1-T1 to W1-T3 per-run files (superseded by b3) are listed with their sha256 in `b2/DROPPED.txt`. Files over 512 KiB (the 1,000- and 10,000-member rows dumps) are kept as their first 40 lines, with the full size, lines and sha256 in each folder's `TRIMMED.txt`. The archived sources (`src/`) are not records; `source.txt` names each commit.
- **The slot labels** in the records are the tiers' planned slots (§5).
- **N-4's constant 141 B** is explained only as consistent with the runners' path strings; it was not traced further.
- **Git:** one early `git fetch` (remote-tracking refs only), disclosed and noted by ROOT; `git ls-remote` (read-only) at D to confirm main.

## 16. Records (`T3/IMPLEMENTATION/K6B/`)

- `PLAN_CHECKPOINT0.md`, `RETURN.md`, `CHANGE_RECORD.md`, `SHA256SUMS` (every file below).
- `_run_records/`:
  - `a0/`: A0's edit script and log (the export, item by item), FK's full suite, H's export test, the scan;
  - `a1/`: the adapter check against R1 (24 of 24), H's suite before and after, the non-test build;
  - `a2/`: the plan (`plan.txt`), the projection, the counts-only runs, `--smoke`, the suites;
  - `b/`: the pre-KF1 run of W1-T1..T3 (108 processes), its report and R1 comparison, and the voided W1-T4 row 247;
  - `c/`: the mutation drivers, results and logs (C, and the M10 re-run);
  - `kf1/`: the post-KF1 plan and backstop check, H's suite, the regenerated counts;
  - `b2/`: slot K6B-S3: row 247 (the stop's evidence), the index, the logs and the report;
  - `b3/`: slot K6B-S4, the result: 132 processes, the report, the R1 comparison, the slot scripts (W1-T4 pre-KF3);
  - `fix/`: the parity fix's suite, runner suite and mutants;
  - `d/`: the tables (`d_tables.py.txt`, `tables.md`), the packet's reproduction (from `_run_records/b3/records` too), the D scan, N-4's cross-check, the runner suite at D, the guard status and the assembly script.
