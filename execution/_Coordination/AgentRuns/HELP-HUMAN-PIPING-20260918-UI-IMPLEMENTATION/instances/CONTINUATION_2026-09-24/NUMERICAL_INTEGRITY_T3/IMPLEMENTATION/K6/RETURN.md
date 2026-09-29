# I15 return: slice K6 (harness observations: kernel sparse and dense memory and runtime on RF-LARGE, and the runner)

- **Brief:** `T3/TASK_BRIEFS/I15_K6_IMPLEMENTATION.md`.
  - As dispatched it had sha256 `7a4338f1…`, at commit `195f15ab5`.
  - At `3e90176c6` it has sha256 `1d754551…`. RV16's records review added one line (the RV16-N4 ruling) at `adf43e1c5`.
  - ROOT ruled on Q1–Q13 and on stale-design items 1–12 in the brief itself.
- **Branch:** `codex/piping-k6-20260928`. The checked head is `3e90176c6`: K6 at `014b2ae04`, with main `59cb20073` merged in, which includes F1b (#1052).
- **Platform:** `aarch64-apple-darwin` (Apple M5 Max, 18 cores, 128 GiB, macOS 26.6.2), with rustc and cargo 1.97.1.
  - **Every observation is Mac-only.** No Linux or Windows observation was run.
- **Observations only.** No time or memory bound is asserted in any test or record. The only claims (§7) are:
  - the observed growth fits;
  - the actual-to-estimate ratios for F1b's two estimates.
- **Paths:** records use the placeholders `<wt>`, `<scratch>`, `<VENV>` and `<home>`.
  - `P/` = `projects/chirality-piping/`.
  - `H` = `P/core/solver/performance_harness/`.
  - `T3/` = `P/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/`.
  - `FK`, `SD`, `NI`, `SA` and `PP` are as in the brief.

## 1. Brief, basis, delegation and rulings

- **Delegation.**
  - ROOT (HELP_HUMAN, the chirality-piping session) dispatched I15 directly, as a Type 2 TASK. It used the host's native background-subagent mechanism (D-GOV-35), with the brief as the assignment.
  - ROOT sent each grant, ruling and pause as an in-session message.
  - I15 delegated nothing (Type 2 does not delegate) and made no Git write.
  - **Scopes and enforcement:**
    - The scope was the brief's write set and host rules, together with ROOT's slot grants.
    - The host enforced its session permission system only. It did not restrict writes to the write set, block Git, or cap memory. I15 kept to those rules itself, and the memory guard and the binary's heap cap bounded the observation processes.
    - Returns went to ROOT at each checkpoint, through the subagent return path.
  - ROOT made every commit:
    - `9ababe4f2`: A1 and A2;
    - `1f354c20b`: main `1cdeae2c1`, with K5, merged in;
    - `962dd4e3b`: B1;
    - `3799e3764`: B2;
    - `ada18de70`: B3;
    - `014b2ae04`: C;
    - `3e90176c6`: main `59cb20073`, with F1b, merged in.
  - One disclosed index slip at A1 (`git add -N` and `git reset -q` on `H/`, with nil net effect) is recorded in ROOT's A1-stop ruling.
- **Basis:**
  - D1 `DESIGN.md` revision 5a.2 (`fb62ef4a…`): §4.8 (`:823-834`), §6's K6 row (`:1021`), and §7.2 (`:1080`);
  - `_COMMON.md`;
  - `I8R_K1_RESUME.md:24-50` (the Mac host);
  - `PAUSE_2026-09-28.md`;
  - K1's RETURN §9, §11 and §12;
  - P1's `DETECTION/RETURN.md` §5 and its scripts;
  - F1b's constants, quoted from `PP` at `130445db2` (`:2871-2878` for 96·n², `:2950-2958` for 24 bytes per identity-order profile entry). They are never imported.
- **Rulings** (`ROOT_RULINGS_V1.md`, at the numerics head `9a93e8516`):
  - "K6: spawn and rulings";
  - "K6: rulings on I15's checkpoint-0 plan";
  - "K6: rulings at I15's A1 stop";
  - "K6: A2 accepted; schedule approved; B1 granted";
  - "K6: B1 accepted; B2 granted";
  - "K6: rulings at I15's B2 stop (Q3's premise measure)";
  - "K6: B2 accepted; the dense-path finding (N10); B3 granted";
  - "K6: B3 accepted".
  - ROOT's C-stop ruling (2026-09-29) added the two runner tests (§10). It was delivered in session, for ROOT to record.
- **The plan:** `T3/IMPLEMENTATION/K6/PLAN_CHECKPOINT0.md` (sha256 `4b4d9b27…`, 633 lines), approved with rulings N1–N20.

## 2. Files and line counts

K6 changes only H, one pytest wrapper and its own records (§9.3). The line counts below are at `3e90176c6`, against main `59cb20073`.

| File | Lines | Change |
|---|---|---|
| `H/Cargo.toml` | 32 | +18: the `[[bin]] k6_observe` (`test = false`), the `[[test]] k6_alloc` (`harness = false`) and the NI path dependency (Q2(b)) |
| `H/Cargo.lock` | 75 | +27: three in-repo path packages (`curved_bend`, `nonlinear_integration`, `nonlinear_supports`) and no registry package (`_run_records/a1/lock/`) |
| `H/src/lib.rs` | 2,287 | +1: `pub mod k6;`, and nothing else (§9.3) |
| `H/src/k6/mod.rs` | 123 | new: `Mode`, `N2_REFUSAL_MEMBERS`, FNV-1a and `debug_digest` |
| `H/src/k6/models.rs` | 547 | new: the R1 generator, in exact integer arithmetic |
| `H/src/k6/canonical.rs` | 192 | new: `k6-model v1`, serialized and parsed |
| `H/src/k6/counts.rs` | 408 | new: the counts line, the skyline count and the estimates |
| `H/src/k6/lanes.rs` | 191 | new: the identity-order lane's entries (N19's documented copy) |
| `H/src/k6/staged.rs` | 668 | new: the staged sequence, the entries and the two lanes |
| `H/src/k6/parity.rs` | 85 | new: bitwise K and the DEC-053 delta |
| `H/src/bin/k6_observe/main.rs` | 784 | new: the CLI, JSONL, refusals and repeats |
| `H/src/bin/k6_observe/alloc.rs` | 232 | new: the counting, capped allocator |
| `H/tests/k6_{alloc,bin,counts,models,parity,staged}.rs` | 111, 280, 346, 162, 164, 71 | new: tests B–G |
| `H/runner/k6_runner.py` | 1,349 | new: the runner (standard library) |
| `H/runner/test_k6_runner.py` | 622 | new: test H, 35 tests |
| `H/observations/k6/` | 21 models (2,717 lines), `models_sha256.txt` (43), `counts.jsonl` (39); at D, `k6_packet.json` and `SHA256SUMS` | new |
| `H/README.md` | 115 | +57 (+1 at D): the K6 section |
| `P/tests/test_performance_harness_runner.py` | 33 | new: the Q9(b) wrapper |
| `T3/IMPLEMENTATION/K6/**` | — | records: the plan, this RETURN, CHANGE_RECORD, `_run_records/` and SHA256SUMS |

The legacy harness is unchanged: `H/examples/sparse_default_promotion_observation.rs`, every DEC-023/050/053 function, constant and test, and `P/validation/benchmarks/*.dec05{0,3}.json`. The Scope 8 path scan shows this (§9.3).

## 3. Scope, item by item

1. **The split (Q1(a)).**
   - Binary64 kernel observations ship now. No W1 observation was made.
   - K6b adds W1 after K4 merges. §13 gives the interface.
   - D1 5a.3 is now selected, and K4 is unblocked (ROOT, "D1 revision 5a.3 SELECTED").
2. **What the limits and ceilings need.**
   - **D-8's binary64 baseline:** the per-stage medians and minima (§6, `_run_records/d/tables/stages_table.txt`) and the peak memory, for the sparse path at 10–10,000 members and the dense path at 10–1,000 members plus the ceiling, on every RF-LARGE family and the DEC-053 nine.
   - **The counts:** §6.1.
   - **The fits:** §7.1.
   - **The runner:** §13.
   - **The dense ceiling:** §7.2 and §8.1, at 66, 606, 6,006 and 8,190 DOFs and on the nine.
   - **The lane guard:** §7.2 and §8.1, at every size the ascent admitted.
   - **A sparse profile ceiling:** the grid ladder, §8.1.
   - **What K6 cannot give:** §8.7.
3. **Models and sizes (Q10, Q11).**
   - All 24 RF-LARGE cases and the DEC-053 nine (the 33 sealed models), plus the ceiling chain and five grids (disclosed as outside the sealed set).
   - R1's node and member order is kept. The section uses explicit products and `PI`.
   - The cross-check against `references.py --model` equals R1's models on all 24 (`_run_records/crosscheck/`).
   - The 33 kernel-model sha256s are in `H/observations/k6/models_sha256.txt`, and the runner's independent Python generator reproduces every one of them (test B).
4. **The observation binary (Q2(b), Q6(a)).**
   - One process per (model, mode), with five in-process repeats.
   - It runs the staged sequence at public-API boundaries, with N1's `reduce`, `densify` and `witness` stages and `stage_begin` lines. It then runs SA's two entries.
   - It has the counting, capped allocator, the refusals by name, and the counts and parity lines (§13).
5. **The runner (Q8(a), Q9(b)).**
   - It uses `/usr/bin/time -l` on macOS and `-v` on Linux, and puts the binary in a new session.
   - On macOS, the watchdog polls the binary (found with `pgrep -P`) every 100 ms and SIGKILLs the process group. On Linux, it sets `RLIMIT_AS` in the child.
   - It records the classification, units, load and `memorystatus_level`, and has `--plan`, `--smoke`, `--run`, `--packet` and `--project`.
   - Its tests run on the DEC-025 pytest surface through the wrapper (N15, confirmed at A2).
6. **Where each run happens (Q3, Q4, Q7).**
   - Everything ran on the Mac, in ROOT's slots B1, B2 and B3, one process at a time, with the memory guard running. The guard logged no event in any slot.
   - Q3's admission rule was amended at the B2 stop, as ruled (§5).
7. **Parity (§4.8 items 1–3).** Bitwise K, outcome-class parity and the DEC-053 basis are covered in §6.3. The A1-stop ruling reads item 3 as applying to Passed publications.
8. **Published bytes: none change** (§9.3).

## 4. Checkpoint-0 positions, as ruled

- **Q1–Q13:** as the brief records, and Q4 (the ceiling run) was granted at B3. Stale-design items 1–12 are recorded as rulings in the brief.
- **N1:** stage map refinements (`reduce`, `densify`, `witness`, `stage_begin`, and the counts before the repeats). Approved.
  - `stage_begin` is what located N10's time (§8.4).
- **N2:** the `[[bin]]` and the `harness = false` allocator test. Approved.
- **N3:** entry stages on every repeat, except dense at 1,000 members or more and the ceiling (`--entry-repeats 1`). Approved.
- **N4:** RF-LARGE's `y_reference` follows P1's rule. Confirmed.
- **N5, N19:** the copies of SA's basis string and `FormationSource`, and of PP's lane builder. Approved.
  - They are pinned by test E, by the lane-profile parity line and by K6-M18.
- **N6:** after each main merge, E and H's suite are re-run.
  - This was done at `1f354c20b` (K5) and at `3e90176c6` (F1b), and E held both times (§11).
- **N7:** `--counts-only` at 1,000 and 10,000 members, at A1–A2, under a 512 MiB cap. Approved.
- **N8:** two peak models. The cap is on the in-place model, and ρ is recorded on both. Approved.
- **N9:** dense admission uses 96·n² + E_base, and the claim ratio stays against the bare 96·n². Approved.
- **N10:** record, finish the tier, then stop. Confirmed as a finding at B2 (§8.4).
- **N11:** B split into B1, B2 and B3, with `--repeats 1` for the two timeouts and for lane-lu at 1,000 members.
- **N12:** K1's table is a partial cross-check, as scoped.
- **N13:** the Python generator lives in the runner.
- **N14:** the grid ladder, with 128×128 conditional. It was admitted into B2 and ran.
- **N15:** DEC-025's sandbox allows `ps`, `pgrep` and process-group kills (A2).
- **N16:** the Linux live test uses `RLIMIT_AS` with no `time` wrapper.
- **N17:** the debug test set was fixed at A1.
- **N18:** `memorystatus_level` ≥ 80 before each run.
- **N20:** the disclosures, carried to §15.
- **Added by ROOT:**
  - `--counts-file` (A1 stop);
  - ρ net of a no-op baseline, measured at 100 members or more for runs at 1,000 or more (A1 stop);
  - the load-wait rule for timed runs (B1 grant);
  - the footprint basis with projected RSS ≤ 0.8·C (B2 stop).

## 5. The schedule as run

The approved schedule has 138 rows (`_run_records/a2/plan.txt`, sha256 `93cd36d5…`). Their final states:

- **122 ok and 2 timed out, all measured.**
  - B1 measured 110 (T1, T2, T3a, T4 and T5 up to 96×96).
  - B2 measured 13 (T3b's 12 and grid 128×128).
  - B3 measured 1 (T6).
- **14 were refused by name and never run:**
  - `dense` and `lane-lu` at 10,000 members, on all six models (12 rows), for `n2_mode_at_or_above_10000_members`;
  - CONT n10000 `lane-id`, AX and ROT (2 rows), for `cont_n10000_identity_lane`.
- **Grid 128×128** was deferred by name at B1 (`conditional_run_needs_rulings_on_projection (N14)`). It was admitted in B2 at E_adm × ρ_fp 1.225, with a projected RSS of 1.62 GiB, and it ran.
- **The admission rule as run:**
  - B1, and B2's first run (097): Q3 as written, with ρ = max(RSS net of the no-op baseline, move-model heap) / E_adm.
  - From 098 onward: the B2-stop rule. ρ is read on the macOS footprint net of the baseline, and a run also needs a projected RSS (the footprint estimate × the largest measured RSS/footprint in the family and mode, floored at 1.45 for dense) of at most 0.8·C.
  - Dense 1,000 was admitted on P1's Linux peak ≤ C/2. The ceiling run was admitted at a 7.78 GiB footprint estimate (≤ 8 GiB) and a projected RSS of 11.29 GiB (≤ 12.8 GiB).
  - Every row's admission and reason is in `_run_records/d/tables/schedule_as_run.txt`.
- **No admitted run was killed by the watchdog or aborted at the heap cap.**
  - B2's RSS was 0.61–0.97 of the projection. B3's was 0.585 of it.
- **B2 was paused once**, for F1b's DEC-025 slot.
  - Run 100 was killed in its witness stage and voided (its files are kept as `VOIDED_pause_100_*`), and then re-run from the start.
  - The runner's resume skip kept 097–099.

## 6. Results

The full per-run table (stage medians and minima, peaks, counts, load and runner commit) is in `_run_records/d/tables/results_table.txt` and `stages_table.txt`, and in the packet `H/observations/k6/k6_packet.json`. §16 says how every number here is traced.

Conventions:
- **Heap:** peak requested bytes, in place.
- **Move:** the move model, where a growing `realloc` holds both blocks.
- **Footprint:** `time -l`'s peak memory footprint.
- **RSS:** `time -l`'s maximum resident set, or "(ps)" for the watchdog's peak where the process group was killed.
- **Staged total:** the sum of the stage medians of one repeat, without the entries.
- **"a / b":** the median and the minimum over the completed repeats.
- **Load:** timings carry the recorded load and are context only.

### 6.1 Counts (`H/observations/k6/counts.jsonl`; closed forms in §9.1)

| Model | Nodes | Members | DOFs | Pattern | Free lower (nonzero) | RCM profile (hbw) | Identity profile | Contributions |
|---|---|---|---|---|---|---|---|---|
| CHAIN-n00010-AX | 11 | 10 | 66 | 1116 | 534 (152) | 168 (4) | 444 | 1440 |
| CHAIN-n00010-ROT | 11 | 10 | 66 | 1116 | 534 (396) | 529 (13) | 524 | 1440 |
| CHAIN-n00100-AX | 101 | 100 | 606 | 10836 | 5664 (1592) | 1788 (4) | 4764 | 14400 |
| CHAIN-n00100-ROT | 101 | 100 | 606 | 10836 | 5664 (4176) | 5569 (13) | 5564 | 14400 |
| CHAIN-n01000-AX | 1001 | 1000 | 6006 | 108036 | 56964 (15992) | 17988 (4) | 47964 | 144000 |
| CHAIN-n01000-ROT | 1001 | 1000 | 6006 | 108036 | 56964 (41976) | 55969 (13) | 55964 | 144000 |
| CEIL-CHAIN-n01364-AX | 1365 | 1364 | 8190 | 147348 | 77712 (21816) | 24540 (4) | 65436 | 196416 |
| CHAIN-n10000-AX | 10001 | 10000 | 60006 | 1080036 | 569964 (159992) | 179988 (4) | 479964 | 1440000 |
| CHAIN-n10000-ROT | 10001 | 10000 | 60006 | 1080036 | 569964 (419976) | 559969 (13) | 559964 | 1440000 |
| CONT-n00010-AX | 11 | 10 | 66 | 1116 | 297 (90) | 98 (3) | 675 | 1440 |
| CONT-n00010-ROT | 11 | 10 | 66 | 1116 | 297 (225) | 279 (10) | 832 | 1440 |
| CONT-n00100-AX | 101 | 100 | 606 | 10836 | 3132 (945) | 1043 (3) | 57510 | 14400 |
| CONT-n00100-ROT | 101 | 100 | 606 | 10836 | 3132 (2385) | 3024 (10) | 69232 | 14400 |
| CONT-n01000-AX | 1001 | 1000 | 6006 | 108036 | 31482 (9495) | 10493 (3) | 5637735 | 144000 |
| CONT-n01000-ROT | 1001 | 1000 | 6006 | 108036 | 31482 (23985) | 30474 (10) | 6767482 | 144000 |
| CONT-n10000-AX | 10001 | 10000 | 60006 | 1080036 | 314982 (94995) | 104993 (3) | 562627485 | 1440000 |
| CONT-n10000-ROT | 10001 | 10000 | 60006 | 1080036 | 314982 (239985) | 304974 (10) | 675174982 | 1440000 |
| DEC053 cantilever-chain-8 | 9 | 8 | 54 | 900 | 420 (120) | 132 (4) | 348 | 1152 |
| DEC053 grid-frame-4x3 | 12 | 17 | 72 | 1656 | 528 (164) | 308 (9) | 782 | 2448 |
| DEC053 cantilever-chain-24 | 25 | 24 | 150 | 2628 | 1332 (376) | 420 (4) | 1116 | 3456 |
| DEC053 cantilever-chain-32 | 33 | 32 | 198 | 3492 | 1788 (504) | 564 (4) | 1500 | 4608 |
| DEC053 grid-frame-5x5 | 25 | 40 | 150 | 3780 | 1536 (456) | 1398 (18) | 3086 | 5760 |
| DEC053 cantilever-chain-48 | 49 | 48 | 294 | 5220 | 2700 (760) | 852 (4) | 2268 | 6912 |
| DEC053 grid-frame-5x6 | 30 | 49 | 180 | 4608 | 1965 (580) | 1938 (19) | 4046 | 7056 |
| DEC053 grid-frame-6x8 | 48 | 82 | 288 | 7632 | 3438 (1002) | 3980 (22) | 8462 | 11808 |
| DEC053 grid-frame-7x8 | 56 | 97 | 336 | 9000 | 4053 (1176) | 5036 (25) | 11390 | 13968 |
| GRID-16x16 | 256 | 480 | 1536 | 43776 | 21204 (6022) | 49314 (50) | 132446 | 69120 |
| GRID-32x32 | 1024 | 1984 | 6144 | 179712 | 89988 (25350) | 394194 (98) | 1118942 | 285696 |
| GRID-64x64 | 4096 | 8064 | 24576 | 728064 | 370404 (103942) | 3149874 (194) | 9192926 | 1161216 |
| GRID-96x96 | 9216 | 18240 | 55296 | 1645056 | 841284 (235782) | 10626194 (290) | 31299806 | 2626560 |
| GRID-128x128 | 16384 | 32512 | 98304 | 2930688 | 1502628 (420870) | 25182450 (386) | 74517470 | 4681728 |
| TREE-n00010-AX | 11 | 10 | 66 | 1116 | 534 (172) | 389 (17) | 590 | 1440 |
| TREE-n00010-ROT | 11 | 10 | 66 | 1116 | 534 (442) | 595 (17) | 674 | 1440 |
| TREE-n00100-AX | 101 | 100 | 606 | 10836 | 5664 (1792) | 4933 (19) | 6530 | 14400 |
| TREE-n00100-ROT | 101 | 100 | 606 | 10836 | 5664 (4672) | 6715 (17) | 7379 | 14400 |
| TREE-n01000-AX | 1001 | 1000 | 6006 | 108036 | 56964 (17992) | 49933 (19) | 65930 | 144000 |
| TREE-n01000-ROT | 1001 | 1000 | 6006 | 108036 | 56964 (46972) | 67915 (17) | 74429 | 144000 |
| TREE-n10000-AX | 10001 | 10000 | 60006 | 1080036 | 569964 (179992) | 499933 (19) | 659930 | 1440000 |
| TREE-n10000-ROT | 10001 | 10000 | 60006 | 1080036 | 569964 (469972) | 679915 (17) | 744929 | 1440000 |

### 6.2 Per run

**Sparse** (SA `SparseInteractive`).

| # | Model | Outcome | Reps | Wall s | Heap MiB | Move MiB | Footprint MiB | RSS MiB | Staged total s | factor med / min s | entry_checked s |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | CHAIN-n00010-AX | Passed | 5 | 0.1 | 0.3 | 0.3 | 2.1 | 3.2 | 0.001 | 2.85e-05 / 2.79e-05 | 0.000866 / 0.000853 |
| 8 | CHAIN-n00010-ROT | Passed | 5 | 0.1 | 0.3 | 0.3 | 2.3 | 3.3 | 0.002 | 7.14e-05 / 5.01e-05 | 0.00136 / 0.00127 |
| 61 | CHAIN-n00100-AX | Sensitive | 5 | 0.2 | 2.6 | 2.6 | 5.0 | 6.0 | 0.011 | 0.000317 / 0.000294 | 0.00865 / 0.00847 |
| 68 | CHAIN-n00100-ROT | Sensitive | 5 | 0.2 | 2.8 | 2.8 | 5.5 | 6.5 | 0.013 | 0.00047 / 0.00045 | 0.011 / 0.011 |
| 85 | CHAIN-n01000-AX | Sensitive | 5 | 1.5 | 27.0 | 27.0 | 34.4 | 35.4 | 0.098 | 0.00258 / 0.00251 | 0.0778 / 0.077 |
| 88 | CHAIN-n01000-ROT | Sensitive | 5 | 2.1 | 29.3 | 29.3 | 40.4 | 41.4 | 0.132 | 0.00493 / 0.00455 | 0.109 / 0.109 |
| 109 | CHAIN-n10000-AX | NumericallyUnresolved | 5 | 6.5 | 254.2 | 254.2 | 687.9 | 711.5 | 0.530 | 0.0299 / 0.0277 | 0.371 / 0.365 |
| 116 | CHAIN-n10000-ROT | NumericallyUnresolved | 5 | 7.6 | 277.1 | 277.1 | 284.9 | 335.3 | 0.619 | 0.0505 / 0.05 | 0.444 / 0.441 |
| 17 | CONT-n00010-AX | Passed | 5 | 0.0 | 0.2 | 0.2 | 2.1 | 3.2 | 0.001 | 2.49e-05 / 2.04e-05 | 0.000805 / 0.00079 |
| 24 | CONT-n00010-ROT | Passed | 5 | 0.1 | 0.3 | 0.3 | 2.1 | 3.2 | 0.001 | 4.08e-05 / 3.06e-05 | 0.00118 / 0.00111 |
| 77 | CONT-n00100-AX | Passed | 5 | 0.2 | 2.2 | 2.2 | 7.9 | 8.9 | 0.010 | 0.000189 / 0.000171 | 0.00827 / 0.00821 |
| 84 | CONT-n00100-ROT | Passed | 5 | 0.2 | 2.5 | 2.5 | 5.2 | 6.3 | 0.014 | 0.000298 / 0.000294 | 0.0117 / 0.0116 |
| 93 | CONT-n01000-AX | Passed | 5 | 1.5 | 25.3 | 25.3 | 35.4 | 36.5 | 0.103 | 0.00197 / 0.00187 | 0.0825 / 0.0823 |
| 96 | CONT-n01000-ROT | Passed | 5 | 2.0 | 27.7 | 27.7 | 40.7 | 41.7 | 0.137 | 0.00273 / 0.00265 | 0.116 / 0.116 |
| 125 | CONT-n10000-AX | Sensitive | 5 | 15.6 | 243.4 | 243.4 | 346.1 | 370.6 | 1.039 | 0.0314 / 0.031 | 0.829 / 0.815 |
| 132 | CONT-n10000-ROT | Sensitive | 5 | 18.9 | 267.4 | 267.4 | 302.3 | 388.7 | 1.291 | 0.0302 / 0.029 | 1.08 / 1.07 |
| 27 | DEC053 cantilever-chain-8 | Passed | 5 | 0.0 | 0.2 | 0.2 | 2.1 | 3.2 | 0.001 | 2.54e-05 / 2.47e-05 | 0.000648 / 0.000637 |
| 40 | DEC053 grid-frame-4x3 | Passed | 5 | 0.1 | 0.4 | 0.4 | 2.3 | 3.4 | 0.002 | 3.97e-05 / 3.75e-05 | 0.00138 / 0.00132 |
| 30 | DEC053 cantilever-chain-24 | Passed | 5 | 0.1 | 0.7 | 0.7 | 2.8 | 3.8 | 0.002 | 7.4e-05 / 6.45e-05 | 0.00198 / 0.00191 |
| 56 | DEC053 cantilever-chain-32 | Passed | 5 | 0.1 | 0.9 | 0.9 | 2.9 | 4.0 | 0.003 | 8.98e-05 / 8.53e-05 | 0.00267 / 0.00266 |
| 49 | DEC053 grid-frame-5x5 | Passed | 5 | 0.1 | 0.9 | 0.9 | 3.2 | 4.3 | 0.004 | 0.000144 / 9.75e-05 | 0.00335 / 0.00319 |
| 33 | DEC053 cantilever-chain-48 | Passed | 5 | 0.1 | 1.4 | 1.4 | 3.8 | 4.9 | 0.005 | 0.000127 / 0.000125 | 0.00401 / 0.00388 |
| 59 | DEC053 grid-frame-5x6 | Passed | 5 | 0.1 | 1.0 | 1.0 | 3.4 | 4.5 | 0.005 | 0.000173 / 0.000128 | 0.00401 / 0.00393 |
| 43 | DEC053 grid-frame-6x8 | Passed | 5 | 0.2 | 1.7 | 1.7 | 6.7 | 7.7 | 0.008 | 0.000279 / 0.00024 | 0.00678 / 0.0062 |
| 46 | DEC053 grid-frame-7x8 | Passed | 5 | 0.2 | 2.0 | 2.0 | 4.8 | 5.9 | 0.010 | 0.000322 / 0.000289 | 0.00802 / 0.00776 |
| 133 | GRID-16x16 | Passed | 5 | 0.8 | 11.4 | 11.4 | 18.5 | 19.5 | 0.052 | 0.00297 / 0.00271 | 0.0422 / 0.0421 |
| 134 | GRID-32x32 | Passed | 5 | 3.4 | 46.2 | 46.3 | 82.4 | 83.4 | 0.234 | 0.0314 / 0.0311 | 0.198 / 0.195 |
| 135 | GRID-64x64 | Sensitive | 5 | 18.1 | 197.1 | 198.1 | 282.8 | 334.6 | 1.246 | 0.45 / 0.445 | 1.1 / 1.09 |
| 136 | GRID-96x96 | Sensitive | 5 | 60.0 | 449.8 | 505.1 | 728.2 | 887.8 | 4.088 | 2.16 / 2.15 | 3.72 / 3.7 |
| 137 | GRID-128x128 | Sensitive | 5 | 150.5 | 886.3 | 981.0 | 1,235.9 | 1,570.3 | 10.124 | 6.48 / 6.46 | 9.53 / 9.52 |
| 11 | TREE-n00010-AX | Passed | 5 | 0.1 | 0.3 | 0.3 | 2.2 | 3.2 | 0.001 | 3.62e-05 / 3.17e-05 | 0.000965 / 0.000916 |
| 14 | TREE-n00010-ROT | Passed | 5 | 0.1 | 0.3 | 0.3 | 2.4 | 3.4 | 0.002 | 6.52e-05 / 6.04e-05 | 0.00156 / 0.00152 |
| 71 | TREE-n00100-AX | Sensitive | 5 | 0.2 | 2.6 | 2.7 | 5.3 | 6.4 | 0.010 | 0.000362 / 0.000322 | 0.00843 / 0.00831 |
| 74 | TREE-n00100-ROT | Sensitive | 5 | 0.2 | 3.0 | 3.0 | 5.9 | 6.9 | 0.014 | 0.000557 / 0.000548 | 0.0121 / 0.0119 |
| 89 | TREE-n01000-AX | Sensitive | 5 | 1.7 | 27.8 | 28.0 | 37.7 | 38.7 | 0.105 | 0.00304 / 0.00297 | 0.0847 / 0.0836 |
| 92 | TREE-n01000-ROT | Sensitive | 5 | 2.3 | 31.1 | 31.4 | 48.7 | 49.7 | 0.143 | 0.00557 / 0.00546 | 0.123 / 0.119 |
| 119 | TREE-n10000-AX | NumericallyUnresolved | 5 | 6.6 | 258.9 | 260.2 | 390.3 | 443.7 | 0.546 | 0.0348 / 0.033 | 0.377 / 0.372 |
| 122 | TREE-n10000-ROT | NumericallyUnresolved | 5 | 8.0 | 286.2 | 288.7 | 575.8 | 673.0 | 0.647 | 0.0581 / 0.0574 | 0.47 / 0.467 |

**Dense** (SA `DenseScrutiny`).

| # | Model | Outcome | Reps | Wall s | Heap MiB | Move MiB | Footprint MiB | RSS MiB | Staged total s | factor med / min s | entry_checked s |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 2 | CHAIN-n00010-AX | Passed | 5 | 0.1 | 0.6 | 0.6 | 2.5 | 3.6 | 0.001 | 8.94e-05 / 8.78e-05 | 0.00109 / 0.00103 |
| 5 | CHAIN-n00010-ROT | Passed | 5 | 0.1 | 0.6 | 0.6 | 2.6 | 3.6 | 0.002 | 9.93e-05 / 9.71e-05 | 0.00164 / 0.00155 |
| 62 | CHAIN-n00100-AX | Sensitive | 5 | 1.7 | 35.1 | 35.1 | 45.9 | 46.9 | 0.105 | 0.0758 / 0.0659 | 0.0955 / 0.0905 |
| 65 | CHAIN-n00100-ROT | Sensitive | 5 | 1.5 | 35.3 | 35.3 | 44.8 | 45.8 | 0.096 | 0.0666 / 0.0642 | 0.0941 / 0.0917 |
| 97 | CHAIN-n01000-AX | Sensitive | 5 | 622.2 | 3,319.2 | 3,319.2 | 3,411.5 | 4,782.7 | 87.761 | 79.6 / 69.4 | 94.5 / 94.5 |
| 100 | CHAIN-n01000-ROT | FactorRefused (timed_out) | - | 1800.1 | 3,319.5 | 3,319.5 | - | 3,413.4 (ps) | 87.324 | 86.1 / 86.1 | - |
| 138 | CEIL-CHAIN-n01364-AX | Sensitive | 5 | 1268.0 | 6,160.5 | 6,160.5 | 6,188.8 | 6,761.5 | 179.860 | 174 / 166 | 181 / 181 |
| 18 | CONT-n00010-AX | Passed | 5 | 0.1 | 0.5 | 0.5 | 2.5 | 3.5 | 0.001 | 5.03e-05 / 4.77e-05 | 0.00094 / 0.000917 |
| 21 | CONT-n00010-ROT | Passed | 5 | 0.1 | 0.6 | 0.6 | 2.6 | 3.6 | 0.002 | 4.89e-05 / 4.42e-05 | 0.00128 / 0.0012 |
| 78 | CONT-n00100-AX | Passed | 5 | 0.9 | 33.8 | 33.8 | 42.7 | 43.8 | 0.053 | 0.0293 / 0.0289 | 0.0513 / 0.0507 |
| 81 | CONT-n00100-ROT | Passed | 5 | 0.9 | 34.0 | 34.0 | 42.6 | 43.7 | 0.058 | 0.03 / 0.0287 | 0.0555 / 0.0542 |
| 105 | CONT-n01000-AX | Passed | 5 | 232.9 | 3,199.1 | 3,199.1 | 3,340.3 | 3,479.0 | 33.249 | 27.8 / 27.7 | 33.4 / 33.4 |
| 108 | CONT-n01000-ROT | Passed | 5 | 232.4 | 3,201.5 | 3,201.5 | 3,343.1 | 3,906.9 | 33.474 | 28.2 / 27.5 | 32.9 / 32.9 |
| 28 | DEC053 cantilever-chain-8 | Passed | 5 | 0.0 | 0.4 | 0.4 | 2.3 | 3.3 | 0.001 | 4.94e-05 / 4.81e-05 | 0.000736 / 0.000734 |
| 37 | DEC053 grid-frame-4x3 | Passed | 5 | 0.1 | 0.7 | 0.7 | 2.8 | 3.8 | 0.002 | 7.25e-05 / 5.6e-05 | 0.00149 / 0.00143 |
| 31 | DEC053 cantilever-chain-24 | Passed | 5 | 0.1 | 2.4 | 2.4 | 5.0 | 6.1 | 0.004 | 0.00102 / 0.00101 | 0.0038 / 0.00364 |
| 53 | DEC053 cantilever-chain-32 | Passed | 5 | 0.1 | 4.1 | 4.1 | 7.8 | 8.8 | 0.007 | 0.00245 / 0.00234 | 0.00648 / 0.0061 |
| 50 | DEC053 grid-frame-5x5 | Passed | 5 | 0.1 | 2.5 | 2.5 | 5.1 | 6.1 | 0.005 | 0.000667 / 0.000634 | 0.00442 / 0.00431 |
| 34 | DEC053 cantilever-chain-48 | Passed | 5 | 0.3 | 8.6 | 8.6 | 13.2 | 14.2 | 0.016 | 0.00763 / 0.00748 | 0.0154 / 0.0152 |
| 60 | DEC053 grid-frame-5x6 | Passed | 5 | 0.1 | 3.5 | 3.5 | 6.4 | 7.4 | 0.007 | 0.00121 / 0.00119 | 0.00617 / 0.00614 |
| 44 | DEC053 grid-frame-6x8 | Passed | 5 | 0.3 | 8.5 | 8.5 | 14.1 | 15.1 | 0.017 | 0.0053 / 0.00516 | 0.0151 / 0.015 |
| 47 | DEC053 grid-frame-7x8 | Passed | 5 | 0.3 | 11.3 | 11.3 | 17.3 | 18.4 | 0.023 | 0.00827 / 0.00823 | 0.0206 / 0.0202 |
| 12 | TREE-n00010-AX | Passed | 5 | 0.1 | 0.6 | 0.6 | 2.5 | 3.6 | 0.001 | 9.05e-05 / 8.96e-05 | 0.0011 / 0.00108 |
| 15 | TREE-n00010-ROT | Passed | 5 | 0.1 | 0.6 | 0.6 | 2.6 | 3.7 | 0.002 | 8.97e-05 / 8.58e-05 | 0.00164 / 0.00148 |
| 72 | TREE-n00100-AX | Sensitive | 5 | 1.5 | 35.1 | 35.1 | 44.9 | 45.9 | 0.095 | 0.0664 / 0.0649 | 0.0925 / 0.0905 |
| 75 | TREE-n00100-ROT | Sensitive | 5 | 1.5 | 35.5 | 35.5 | 45.1 | 46.1 | 0.095 | 0.0651 / 0.0646 | 0.0929 / 0.0913 |
| 101 | TREE-n01000-AX | FactorRefused (timed_out) | - | 1800.1 | 3,317.7 | 3,317.9 | - | 3,412.2 (ps) | 66.337 | 65.2 / 65.2 | - |
| 104 | TREE-n01000-ROT | Sensitive | 5 | 507.5 | 3,323.3 | 3,323.6 | 3,418.5 | 3,714.0 | 72.429 | 64.8 / 64.6 | 72.4 / 72.4 |

**The identity-order lane** (`lane-id`).

| # | Model | Outcome | Reps | Wall s | Heap MiB | Move MiB | Footprint MiB | RSS MiB | Staged total s | lane_solve med / min s | entry_checked s |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 3 | CHAIN-n00010-AX | LaneSolved | 5 | 2.0 | 0.1 | 0.1 | 1.6 | 2.5 | 0.000 | 2.51e-05 / 2.28e-05 | - |
| 6 | CHAIN-n00010-ROT | LaneSolved | 5 | 0.0 | 0.1 | 0.1 | 1.7 | 2.6 | 0.000 | 5.8e-05 / 5.23e-05 | - |
| 63 | CHAIN-n00100-AX | LaneSolved | 5 | 0.0 | 0.6 | 0.6 | 2.4 | 3.3 | 0.001 | 0.000198 / 0.000197 | - |
| 66 | CHAIN-n00100-ROT | LaneSolved | 5 | 0.0 | 1.0 | 1.0 | 2.9 | 3.9 | 0.001 | 0.000465 / 0.000461 | - |
| 86 | CHAIN-n01000-AX | LaneSolved | 5 | 0.1 | 6.4 | 6.4 | 11.0 | 11.9 | 0.006 | 0.00194 / 0.00192 | - |
| 87 | CHAIN-n01000-ROT | LaneSolved | 5 | 0.1 | 11.5 | 11.6 | 16.4 | 17.3 | 0.009 | 0.00504 / 0.00491 | - |
| 111 | CHAIN-n10000-AX | LaneSolved | 5 | 0.5 | 58.6 | 58.6 | 124.8 | 142.7 | 0.065 | 0.0216 / 0.0199 | - |
| 114 | CHAIN-n10000-ROT | LaneSolved | 5 | 0.6 | 107.1 | 107.3 | 209.3 | 263.9 | 0.092 | 0.0514 / 0.0501 | - |
| 19 | CONT-n00010-AX | LaneSolved | 5 | 0.0 | 0.1 | 0.1 | 1.6 | 2.5 | 0.000 | 1.87e-05 / 1.52e-05 | - |
| 22 | CONT-n00010-ROT | LaneSolved | 5 | 0.0 | 0.1 | 0.1 | 1.6 | 2.6 | 0.000 | 3.47e-05 / 3.14e-05 | - |
| 79 | CONT-n00100-AX | LaneSolved | 5 | 0.0 | 1.0 | 1.2 | 2.9 | 3.8 | 0.001 | 0.000142 / 0.00014 | - |
| 82 | CONT-n00100-ROT | LaneSolved | 5 | 0.0 | 1.7 | 2.1 | 3.2 | 4.1 | 0.001 | 0.000309 / 0.000301 | - |
| 94 | CONT-n01000-AX | LaneSolved | 5 | 0.1 | 69.1 | 100.6 | 229.1 | 229.9 | 0.010 | 0.00571 / 0.00559 | - |
| 95 | CONT-n01000-ROT | LaneSolved | 5 | 0.1 | 71.1 | 102.3 | 272.9 | 273.6 | 0.011 | 0.00707 / 0.00668 | - |
| 25 | DEC053 cantilever-chain-8 | LaneSolved | 5 | 0.0 | 0.1 | 0.1 | 1.6 | 2.5 | 0.000 | 1.96e-05 / 1.78e-05 | - |
| 38 | DEC053 grid-frame-4x3 | LaneSolved | 5 | 0.0 | 0.1 | 0.1 | 1.7 | 2.6 | 0.000 | 3.07e-05 / 2.57e-05 | - |
| 32 | DEC053 cantilever-chain-24 | LaneSolved | 5 | 0.0 | 0.2 | 0.2 | 1.7 | 2.7 | 0.000 | 5e-05 / 4.78e-05 | - |
| 54 | DEC053 cantilever-chain-32 | LaneSolved | 5 | 0.0 | 0.2 | 0.2 | 1.9 | 2.8 | 0.000 | 6.72e-05 / 6.56e-05 | - |
| 51 | DEC053 grid-frame-5x5 | LaneSolved | 5 | 0.0 | 0.2 | 0.2 | 2.0 | 2.9 | 0.000 | 6.31e-05 / 5.95e-05 | - |
| 35 | DEC053 cantilever-chain-48 | LaneSolved | 5 | 0.0 | 0.3 | 0.3 | 2.0 | 2.9 | 0.000 | 9.62e-05 / 9.39e-05 | - |
| 57 | DEC053 grid-frame-5x6 | LaneSolved | 5 | 0.0 | 0.3 | 0.3 | 2.1 | 3.0 | 0.000 | 8.14e-05 / 7.39e-05 | - |
| 41 | DEC053 grid-frame-6x8 | LaneSolved | 5 | 0.0 | 0.5 | 0.5 | 2.3 | 3.3 | 0.000 | 0.000156 / 0.000138 | - |
| 48 | DEC053 grid-frame-7x8 | LaneSolved | 5 | 0.0 | 0.6 | 0.6 | 2.6 | 3.5 | 0.001 | 0.000175 / 0.000167 | - |
| 9 | TREE-n00010-AX | LaneSolved | 5 | 0.0 | 0.1 | 0.1 | 1.6 | 2.6 | 0.000 | 2.9e-05 / 2.31e-05 | - |
| 16 | TREE-n00010-ROT | LaneSolved | 5 | 0.0 | 0.1 | 0.1 | 1.7 | 2.6 | 0.000 | 5.92e-05 / 5.24e-05 | - |
| 69 | TREE-n00100-AX | LaneSolved | 5 | 0.0 | 0.7 | 0.7 | 2.6 | 3.5 | 0.001 | 0.000207 / 0.000203 | - |
| 76 | TREE-n00100-ROT | LaneSolved | 5 | 0.0 | 1.0 | 1.0 | 3.0 | 3.9 | 0.001 | 0.000501 / 0.000488 | - |
| 90 | TREE-n01000-AX | LaneSolved | 5 | 0.1 | 7.5 | 7.5 | 12.6 | 13.5 | 0.006 | 0.00206 / 0.00196 | - |
| 91 | TREE-n01000-ROT | LaneSolved | 5 | 0.1 | 11.4 | 11.5 | 16.1 | 17.0 | 0.009 | 0.00531 / 0.00502 | - |
| 117 | TREE-n10000-AX | LaneSolved | 5 | 0.5 | 68.5 | 68.5 | 105.7 | 130.8 | 0.062 | 0.0224 / 0.022 | - |
| 124 | TREE-n10000-ROT | LaneSolved | 5 | 0.6 | 105.8 | 106.1 | 210.1 | 265.2 | 0.096 | 0.0541 / 0.0534 | - |

**The dense LU lane** (`lane-lu`).

| # | Model | Outcome | Reps | Wall s | Heap MiB | Move MiB | Footprint MiB | RSS MiB | Staged total s | lane_solve med / min s | entry_checked s |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 4 | CHAIN-n00010-AX | LaneSolved | 5 | 0.0 | 0.1 | 0.1 | 1.7 | 2.6 | 0.000 | 8.94e-05 / 8.48e-05 | - |
| 7 | CHAIN-n00010-ROT | LaneSolved | 5 | 0.0 | 0.1 | 0.1 | 1.7 | 2.6 | 0.000 | 7.57e-05 / 6.73e-05 | - |
| 64 | CHAIN-n00100-AX | LaneSolved | 5 | 0.3 | 5.9 | 5.9 | 8.5 | 9.4 | 0.061 | 0.0602 / 0.0593 | - |
| 67 | CHAIN-n00100-ROT | LaneSolved | 5 | 0.4 | 5.9 | 5.9 | 9.0 | 9.8 | 0.061 | 0.0602 / 0.0598 | - |
| 98 | CHAIN-n01000-AX | LaneSolved | 1 | 66.5 | 553.9 | 553.9 | 574.8 | 575.3 | 66.483 | 66.4 / 66.4 | - |
| 99 | CHAIN-n01000-ROT | LaneSolved | 1 | 72.0 | 553.9 | 553.9 | 574.8 | 575.4 | 71.946 | 71.9 / 71.9 | - |
| 20 | CONT-n00010-AX | LaneSolved | 5 | 0.0 | 0.1 | 0.1 | 1.6 | 2.5 | 0.000 | 3.49e-05 / 3.23e-05 | - |
| 23 | CONT-n00010-ROT | LaneSolved | 5 | 0.0 | 0.1 | 0.1 | 1.6 | 2.5 | 0.000 | 3.5e-05 / 3.2e-05 | - |
| 80 | CONT-n00100-AX | LaneSolved | 5 | 0.2 | 4.7 | 4.7 | 9.1 | 10.0 | 0.027 | 0.0262 / 0.0261 | - |
| 83 | CONT-n00100-ROT | LaneSolved | 5 | 0.2 | 4.7 | 4.7 | 9.2 | 10.1 | 0.027 | 0.0255 / 0.0254 | - |
| 106 | CONT-n01000-AX | LaneSolved | 1 | 27.1 | 433.9 | 433.9 | 502.9 | 503.5 | 27.051 | 27 / 27 | - |
| 107 | CONT-n01000-ROT | LaneSolved | 1 | 27.1 | 434.0 | 434.0 | 502.9 | 503.6 | 27.007 | 26.9 / 26.9 | - |
| 26 | DEC053 cantilever-chain-8 | LaneSolved | 5 | 0.0 | 0.1 | 0.1 | 1.6 | 2.5 | 0.000 | 4.12e-05 / 3.8e-05 | - |
| 39 | DEC053 grid-frame-4x3 | LaneSolved | 5 | 0.0 | 0.1 | 0.1 | 1.7 | 2.6 | 0.000 | 4.2e-05 / 3.8e-05 | - |
| 29 | DEC053 cantilever-chain-24 | LaneSolved | 5 | 0.0 | 0.4 | 0.4 | 2.2 | 3.1 | 0.001 | 0.000941 / 0.000917 | - |
| 55 | DEC053 cantilever-chain-32 | LaneSolved | 5 | 0.0 | 0.7 | 0.7 | 2.9 | 3.8 | 0.002 | 0.00206 / 0.00205 | - |
| 52 | DEC053 grid-frame-5x5 | LaneSolved | 5 | 0.0 | 0.4 | 0.4 | 2.3 | 3.2 | 0.001 | 0.000476 / 0.000461 | - |
| 36 | DEC053 cantilever-chain-48 | LaneSolved | 5 | 0.1 | 1.5 | 1.5 | 3.5 | 4.4 | 0.008 | 0.00715 / 0.00702 | - |
| 58 | DEC053 grid-frame-5x6 | LaneSolved | 5 | 0.0 | 0.6 | 0.6 | 2.6 | 3.5 | 0.001 | 0.000958 / 0.000942 | - |
| 42 | DEC053 grid-frame-6x8 | LaneSolved | 5 | 0.2 | 1.4 | 1.4 | 4.2 | 5.0 | 0.005 | 0.00459 / 0.00447 | - |
| 45 | DEC053 grid-frame-7x8 | LaneSolved | 5 | 0.2 | 1.8 | 1.8 | 5.1 | 6.0 | 0.008 | 0.00761 / 0.00713 | - |
| 10 | TREE-n00010-AX | LaneSolved | 5 | 0.0 | 0.1 | 0.1 | 1.7 | 2.6 | 0.000 | 7.21e-05 / 6.83e-05 | - |
| 13 | TREE-n00010-ROT | LaneSolved | 5 | 0.0 | 0.1 | 0.1 | 1.7 | 2.6 | 0.000 | 8.28e-05 / 7.72e-05 | - |
| 70 | TREE-n00100-AX | LaneSolved | 5 | 0.3 | 5.9 | 5.9 | 8.6 | 9.5 | 0.061 | 0.0604 / 0.0602 | - |
| 73 | TREE-n00100-ROT | LaneSolved | 5 | 0.3 | 5.9 | 5.9 | 8.6 | 9.5 | 0.061 | 0.0602 / 0.0599 | - |
| 102 | TREE-n01000-AX | LaneSolved | 1 | 63.8 | 554.0 | 554.0 | 574.7 | 575.3 | 63.721 | 63.6 / 63.6 | - |
| 103 | TREE-n01000-ROT | LaneSolved | 1 | 63.5 | 554.2 | 554.2 | 591.9 | 592.5 | 63.445 | 63.4 / 63.4 | - |

### 6.3 Parity and outcome classes

The binary's own parity lines, over all 124 measured runs, were all equal:

| Item | Lines | Runs | Where |
|---|---|---|---|
| `staged_vs_entry_checked` | 300 | 64 | every completed sparse (38) and dense (26) run, at every size including the ceiling |
| `entry_plain_vs_entry_checked` | 300 | 64 | the same runs |
| `bitwise_k` | 25 | 25 | every completed dense run up to 1,000 members |
| `rcm_count_equals_ordering` | 190 | 38 | every sparse run |
| `lane_identity_profile_equals_count` | 155 | 31 | every `lane-id` run |
| `repeat_determinism` | 116 | 116 | every run with more than one repeat |

**Cross-mode rows.** Classes agree on every model except the two N10 cases. The DEC-053 basis is asserted only where both modes are Passed (A1-stop ruling).

| Model | Sparse | Dense | Classes equal | Relative delta | DEC-053 asserted | Within 1e-9 |
|---|---|---|---|---|---|---|
| DEC053 cantilever-chain-24 | Passed | Passed | True | 1.009e-11 | True | True |
| DEC053 cantilever-chain-32 | Passed | Passed | True | 3.804e-11 | True | True |
| DEC053 cantilever-chain-48 | Passed | Passed | True | 5.857e-11 | True | True |
| DEC053 cantilever-chain-8 | Passed | Passed | True | 1.030e-13 | True | True |
| DEC053 grid-frame-4x3 | Passed | Passed | True | 1.006e-13 | True | True |
| DEC053 grid-frame-5x5 | Passed | Passed | True | 8.253e-14 | True | True |
| DEC053 grid-frame-5x6 | Passed | Passed | True | 1.794e-13 | True | True |
| DEC053 grid-frame-6x8 | Passed | Passed | True | 4.594e-13 | True | True |
| DEC053 grid-frame-7x8 | Passed | Passed | True | 5.515e-13 | True | True |
| CHAIN-n00010-AX | Passed | Passed | True | 5.241e-13 | True | True |
| CHAIN-n00010-ROT | Passed | Passed | True | 3.063e-11 | True | True |
| CHAIN-n00100-AX | Sensitive | Sensitive | True | 4.068e-10 | False | - |
| CHAIN-n00100-ROT | Sensitive | Sensitive | True | 7.149e-08 | False | - |
| CHAIN-n01000-AX | Sensitive | Sensitive | True | 3.994e-05 | False | - |
| CHAIN-n01000-ROT | Sensitive | FactorRefused | False | - | None | - |
| CONT-n00010-AX | Passed | Passed | True | 9.508e-17 | True | True |
| CONT-n00010-ROT | Passed | Passed | True | 2.100e-14 | True | True |
| CONT-n00100-AX | Passed | Passed | True | 4.758e-16 | True | True |
| CONT-n00100-ROT | Passed | Passed | True | 2.623e-14 | True | True |
| CONT-n01000-AX | Passed | Passed | True | 4.758e-16 | True | True |
| CONT-n01000-ROT | Passed | Passed | True | 8.764e-13 | True | True |
| TREE-n00010-AX | Passed | Passed | True | 6.059e-12 | True | True |
| TREE-n00010-ROT | Passed | Passed | True | 1.936e-11 | True | True |
| TREE-n00100-AX | Sensitive | Sensitive | True | 1.447e-08 | False | - |
| TREE-n00100-ROT | Sensitive | Sensitive | True | 4.783e-08 | False | - |
| TREE-n01000-AX | Sensitive | FactorRefused | False | - | None | - |
| TREE-n01000-ROT | Sensitive | Sensitive | True | 2.337e-04 | False | - |

- **Outcome classes, by mode.**
  - **Sparse:** 21 Passed, 13 Sensitive and 4 NumericallyUnresolved.
    - The 4 unresolved are CHAIN and TREE at 10,000 members. The ROT pair failed at `factor` and ran the sparse witness (0.03 s). The AX pair failed at `finish`.
    - F1b's product-level gate records named M03 refusals on 8 of its 12 sparse 10,000-member runs.
  - **Dense:** 19 Passed, 7 Sensitive and 2 FactorRefused (the N10 pair).
  - **The lanes:** every run solved.

## 7. The two claims

### 7.1 Growth fits (log-log least squares against members; points and residuals in `_run_records/d/tables/fits.txt`)

The "all" columns use every measured size. The "≥100" columns drop the 10-member point, where the process baseline dominates. "Net" means net of the run's own tier baseline, a no-op run of the same binary in the same slot: RSS 1,818,624 B and footprint 1,147,192 B in every tier. CHAIN/dense includes the ceiling chain (1,364 members, the RF-LARGE-CHAIN geometry). Without it, CHAIN/dense's heap slope over 100–1,000 members is 1.975.

| Family / mode | Members | Heap, all | Heap, ≥100 | Move, ≥100 | Footprint net, ≥100 | RSS net, all | RSS net, ≥100 |
|---|---|---|---|---|---|---|---|
| CHAIN/dense | 10, 100, 1000, 1364 | 1.889 | 1.976 | 1.976 | 1.889 | 1.689 | 1.966 |
| CHAIN/lane-id | 10, 100, 1000, 10000 | 1.002 | 1.004 | 1.004 | 1.006 | 0.792 | 1.008 |
| CHAIN/lane-lu | 10, 100, 1000 | 1.866 | 1.972 | 1.972 | 1.875 | 1.418 | 1.861 |
| CHAIN/sparse | 10, 100, 1000, 10000 | 1.000 | 0.999 | 0.998 | 1.015 | 0.841 | 1.017 |
| CONT/dense | 10, 100, 1000 | 1.881 | 1.975 | 1.975 | 1.905 | 1.651 | 1.943 |
| CONT/lane-id | 10, 100, 1000 | 1.506 | 1.737 | 1.809 | 2.114 | 1.241 | 2.053 |
| CONT/lane-lu | 10, 100, 1000 | 1.839 | 1.964 | 1.964 | 1.794 | 1.397 | 1.781 |
| CONT/sparse | 10, 100, 1000, 10000 | 1.002 | 1.018 | 1.018 | 0.893 | 0.804 | 0.910 |
| DEC053/dense | 8, 17, 24, 32, 40, 48, 49, 82, 97 | 1.374 | - | - | - | 0.979 | - |
| DEC053/lane-id | 8, 17, 24, 32, 40, 48, 49, 82, 97 | 0.998 | - | - | - | 0.323 | - |
| DEC053/lane-lu | 8, 17, 24, 32, 40, 48, 49, 82, 97 | 1.318 | - | - | - | 0.704 | - |
| DEC053/sparse | 8, 17, 24, 32, 40, 48, 49, 82, 97 | 0.872 | - | - | - | 0.517 | - |
| GRID/sparse | 480, 1984, 8064, 18240, 32512 | 1.028 | 1.028 | 1.055 | 1.006 | 1.064 | 1.064 |
| TREE/dense | 10, 100, 1000 | 1.868 | 1.974 | 1.973 | 1.891 | 1.606 | 1.924 |
| TREE/lane-id | 10, 100, 1000, 10000 | 1.005 | 1.006 | 1.006 | 0.970 | 0.780 | 0.983 |
| TREE/lane-lu | 10, 100, 1000 | 1.866 | 1.971 | 1.971 | 1.890 | 1.419 | 1.875 |
| TREE/sparse | 10, 100, 1000, 10000 | 0.998 | 0.993 | 0.991 | 1.011 | 0.853 | 1.023 |

- **Sparse (the §4.8 single claim, at kernel level): heap is linear in members.**
  - The slope is 0.998–1.002 over 10–10,000 members for CHAIN, TREE and CONT, and 0.991–1.018 at 100 members or more.
  - RSS net of the baseline gives 0.910–1.023 at 100 or more.
- **Dense: heap grows as n².** The slope is 1.974–1.976 over 100–1,000 members, approaching 2 as the n² buffers dominate.
- **lane-lu** grows as n² as well: 1.964–1.972.
- **lane-id:**
  - CHAIN and TREE are linear (1.004–1.006), because their identity profile is banded.
  - CONT is 1.737 in heap and 1.809 in move over 100–1,000 members. Its identity profile grows as 27s² (§9.1).
- **The DEC-053 nine** mix two shapes (chains and grids), so their fits are context only.

### 7.2 F1b's estimates against the measured peaks

F1b's estimates, quoted from PP at `130445db2` and never imported: dense 96·n² (`PP:2878`), and the lane at 24 bytes per identity-order profile entry P_id (`PP:2958`). F1b's provisional ceiling is 6 GiB = 6,442,450,944 B.

**Dense, at every size including the ceiling.**

| Model | Members | DOFs | 96·n² (B) | Heap | Move | Footprint | RSS | Heap / 6 GiB |
|---|---|---|---|---|---|---|---|---|
| CHAIN-n00010-AX | 10 | 66 | 418,176 | 1.3883 | 1.3883 | 6.3087 | 8.9330 | 0.0001 |
| CHAIN-n00010-ROT | 10 | 66 | 418,176 | 1.4491 | 1.4491 | 6.4263 | 9.0505 | 0.0001 |
| CONT-n00010-AX | 10 | 66 | 418,176 | 1.3562 | 1.3562 | 6.2304 | 8.8546 | 0.0001 |
| CONT-n00010-ROT | 10 | 66 | 418,176 | 1.4201 | 1.4201 | 6.4263 | 9.0505 | 0.0001 |
| TREE-n00010-AX | 10 | 66 | 418,176 | 1.4023 | 1.4056 | 6.3479 | 8.9721 | 0.0001 |
| TREE-n00010-ROT | 10 | 66 | 418,176 | 1.4820 | 1.4881 | 6.6222 | 9.2464 | 0.0001 |
| CHAIN-n00100-AX | 100 | 606 | 35,254,656 | 1.0429 | 1.0429 | 1.3649 | 1.3937 | 0.0057 |
| CHAIN-n00100-ROT | 100 | 606 | 35,254,656 | 1.0497 | 1.0497 | 1.3329 | 1.3617 | 0.0057 |
| CONT-n00100-AX | 100 | 606 | 35,254,656 | 1.0043 | 1.0043 | 1.2711 | 1.3017 | 0.0055 |
| CONT-n00100-ROT | 100 | 606 | 35,254,656 | 1.0112 | 1.0112 | 1.2683 | 1.2985 | 0.0055 |
| TREE-n00100-AX | 100 | 606 | 35,254,656 | 1.0449 | 1.0451 | 1.3366 | 1.3659 | 0.0057 |
| TREE-n00100-ROT | 100 | 606 | 35,254,656 | 1.0557 | 1.0571 | 1.3417 | 1.3705 | 0.0058 |
| CHAIN-n01000-AX | 1000 | 6006 | 3,462,915,456 | 1.0051 | 1.0051 | 1.0330 | 1.4482 | 0.5402 |
| CHAIN-n01000-ROT (timed_out) | 1000 | 6006 | 3,462,915,456 | 1.0051 | 1.0051 | - | 1.0336 | 0.5403 |
| CONT-n01000-AX | 1000 | 6006 | 3,462,915,456 | 0.9687 | 0.9687 | 1.0115 | 1.0534 | 0.5207 |
| CONT-n01000-ROT | 1000 | 6006 | 3,462,915,456 | 0.9694 | 0.9694 | 1.0123 | 1.1830 | 0.5211 |
| TREE-n01000-AX (timed_out) | 1000 | 6006 | 3,462,915,456 | 1.0046 | 1.0047 | - | 1.0332 | 0.5400 |
| TREE-n01000-ROT | 1000 | 6006 | 3,462,915,456 | 1.0063 | 1.0064 | 1.0351 | 1.1246 | 0.5409 |
| CEIL-CHAIN-n01364-AX | 1364 | 8190 | 6,439,305,600 | 1.0032 | 1.0032 | 1.0078 | 1.1010 | 1.0027 |
| DEC053 cantilever-chain-8 | 8 | 54 | 279,936 | 1.5184 | 1.5184 | 8.4877 | 12.4079 | 0.0001 |
| DEC053 grid-frame-4x3 | 17 | 72 | 497,664 | 1.4807 | 1.4807 | 5.7949 | 8.0000 | 0.0001 |
| DEC053 cantilever-chain-24 | 24 | 150 | 2,160,000 | 1.1737 | 1.1737 | 2.4350 | 2.9431 | 0.0004 |
| DEC053 cantilever-chain-32 | 32 | 198 | 3,763,584 | 1.1491 | 1.1491 | 2.1767 | 2.4640 | 0.0007 |
| DEC053 grid-frame-5x5 | 40 | 150 | 2,160,000 | 1.2257 | 1.2257 | 2.4653 | 2.9734 | 0.0004 |
| DEC053 cantilever-chain-48 | 48 | 294 | 8,297,856 | 1.0897 | 1.0897 | 1.6685 | 1.8007 | 0.0014 |
| DEC053 grid-frame-5x6 | 49 | 180 | 3,110,400 | 1.1803 | 1.1803 | 2.1492 | 2.5021 | 0.0006 |
| DEC053 grid-frame-6x8 | 82 | 288 | 7,962,624 | 1.1190 | 1.1190 | 1.8519 | 1.9877 | 0.0014 |
| DEC053 grid-frame-7x8 | 97 | 336 | 10,838,016 | 1.0964 | 1.0964 | 1.6780 | 1.7778 | 0.0018 |

- **At 1,000 members and at the ceiling:**
  - heap is 0.969–1.006 × 96·n²;
  - the footprint is 1.008–1.035;
  - RSS is 1.033–1.448.
- **At the ceiling (8,190 DOFs):**
  - heap is 1.0032 × 96·n², and 1.0027 × the 6 GiB ceiling;
  - the footprint is 1.0078 × 96·n²;
  - RSS is 1.1010 × 96·n².
- **At 10 members and on the nine,** the O(nnz) state is large beside n², so the heap ratio is 1.09–1.52. The footprint and RSS ratios there are dominated by the process baseline.

**The identity-order lane, at every size the ascent admitted.** The last three columns also give RV17-N2's form 16P + 24P′, with P = P_id and P′ = the lane's entry count; that reading of RV17-N2 is I15's.

| Model | Members | 24·P_id (B) | Heap | Move | Footprint | RSS | 16P + 24P′ (B) | Heap / that | Move / that |
|---|---|---|---|---|---|---|---|---|---|
| CHAIN-n00010-AX | 10 | 10,656 | 6.0549 | 6.0549 | 153.7830 | 246.0060 | 12,912 | 4.997 | 4.997 |
| CHAIN-n00010-ROT | 10 | 12,576 | 8.8361 | 8.8368 | 140.7271 | 220.1730 | 24,848 | 4.472 | 4.472 |
| CONT-n00010-AX | 10 | 16,200 | 3.9344 | 3.9344 | 103.1778 | 163.8400 | 14,448 | 4.412 | 4.412 |
| CONT-n00010-ROT | 10 | 19,968 | 4.0077 | 4.0077 | 85.3490 | 135.3846 | 23,080 | 3.467 | 3.467 |
| TREE-n00010-AX | 10 | 14,160 | 5.0833 | 5.0833 | 121.5136 | 190.9153 | 15,248 | 4.721 | 4.721 |
| TREE-n00010-ROT | 10 | 16,176 | 6.8090 | 6.8090 | 109.4080 | 171.1731 | 27,248 | 4.042 | 4.042 |
| CHAIN-n00100-AX | 100 | 114,336 | 5.6108 | 5.6108 | 22.2140 | 30.6655 | 138,192 | 4.642 | 4.642 |
| CHAIN-n00100-ROT | 100 | 133,536 | 7.7834 | 7.8123 | 23.0689 | 30.4280 | 265,328 | 3.917 | 3.932 |
| CONT-n00100-AX | 100 | 1,380,240 | 0.7412 | 0.8923 | 2.1844 | 2.8845 | 958,368 | 1.067 | 1.285 |
| CONT-n00100-ROT | 100 | 1,661,568 | 1.0679 | 1.3315 | 1.9920 | 2.5835 | 1,210,360 | 1.466 | 1.828 |
| TREE-n00100-AX | 100 | 156,720 | 4.6373 | 4.6373 | 17.5654 | 23.7313 | 166,448 | 4.366 | 4.366 |
| TREE-n00100-ROT | 100 | 177,096 | 5.8583 | 5.8794 | 17.6722 | 23.2212 | 294,368 | 3.524 | 3.537 |
| CHAIN-n01000-AX | 1000 | 1,151,136 | 5.8024 | 5.8024 | 9.9776 | 10.8028 | 1,390,992 | 4.802 | 4.802 |
| CHAIN-n01000-ROT | 1000 | 1,343,136 | 8.9982 | 9.0224 | 12.8329 | 13.5401 | 2,670,128 | 4.526 | 4.538 |
| CONT-n01000-AX | 1000 | 135,305,640 | 0.5356 | 0.7799 | 1.7757 | 1.7815 | 90,587,568 | 0.800 | 1.165 |
| CONT-n01000-ROT | 1000 | 162,419,568 | 0.4589 | 0.6606 | 1.7616 | 1.7666 | 109,311,160 | 0.682 | 0.982 |
| TREE-n01000-AX | 1000 | 1,582,320 | 4.9490 | 4.9490 | 8.3770 | 8.9773 | 1,678,448 | 4.666 | 4.666 |
| TREE-n01000-ROT | 1000 | 1,786,296 | 6.7188 | 6.7369 | 9.4291 | 9.9700 | 2,965,568 | 4.047 | 4.058 |
| CHAIN-n10000-AX | 10000 | 11,519,136 | 5.3370 | 5.3370 | 11.3644 | 12.9901 | 13,918,992 | 4.417 | 4.417 |
| CHAIN-n10000-ROT | 10000 | 13,439,136 | 8.3562 | 8.3757 | 16.3302 | 20.5910 | 26,718,128 | 4.203 | 4.213 |
| TREE-n10000-AX | 10000 | 15,838,320 | 4.5339 | 4.5339 | 7.0002 | 8.6625 | 16,798,448 | 4.275 | 4.275 |
| TREE-n10000-ROT | 10000 | 17,878,296 | 6.2067 | 6.2214 | 12.3222 | 15.5535 | 29,677,568 | 3.739 | 3.748 |
| DEC053 cantilever-chain-8 | 8 | 8,352 | 6.7322 | 6.7322 | 198.1676 | 315.8314 | 10,128 | 5.552 | 5.552 |
| DEC053 grid-frame-4x3 | 17 | 18,768 | 4.9986 | 4.9986 | 94.2980 | 146.6598 | 19,520 | 4.806 | 4.806 |
| DEC053 cantilever-chain-24 | 24 | 26,784 | 5.9482 | 5.9482 | 67.2996 | 103.9904 | 32,400 | 4.917 | 4.917 |
| DEC053 cantilever-chain-32 | 32 | 36,000 | 5.9934 | 5.9934 | 54.1676 | 81.0098 | 43,536 | 4.956 | 4.956 |
| DEC053 grid-frame-5x5 | 40 | 74,064 | 3.1040 | 3.1040 | 27.8775 | 41.1458 | 69,680 | 3.299 | 3.299 |
| DEC053 cantilever-chain-48 | 48 | 54,432 | 5.7981 | 5.7981 | 37.6311 | 55.3839 | 65,808 | 4.796 | 4.796 |
| DEC053 grid-frame-5x6 | 49 | 97,104 | 2.8951 | 2.8951 | 22.4441 | 32.5642 | 90,656 | 3.101 | 3.101 |
| DEC053 grid-frame-6x8 | 82 | 203,088 | 2.6399 | 2.6399 | 12.0221 | 16.8609 | 180,848 | 2.965 | 2.965 |
| DEC053 grid-frame-7x8 | 97 | 273,360 | 2.4233 | 2.4233 | 9.8307 | 13.4256 | 236,000 | 2.807 | 2.807 |

- **CONT (the case the guard exists for):**
  - heap is 0.459–1.068 × 24·P_id at 100–1,000 members, and the move model is 0.661–1.332;
  - against 16P + 24P′, the move model is 0.98–1.17 at 1,000 members.
- **CHAIN and TREE:** P_id is small (banded), so the entries and the adjacency dominate. That gives 4.5–9.0 × 24·P_id, with small absolute sizes: at most 107 MiB at 10,000 members.
- **CONT at 10,000 members is never run.**
  - K6's counts give P_id = 675,174,982 (ROT) and 562,627,485 (AX); F1b's full-block form is 675,179,982.
  - 24·P_id is 16.2 GB (ROT) and 13.5 GB (AX), which the A1-stop ruling records as context.
- **lane-lu:** F1b states no constant. The measured heap is 1.002–1.003 × the derived E_adm at 1,000 members, which is about 16 bytes per n² entry (the dense view plus the reduced matrix, plan §7.4).

## 8. Observations for the ceiling and budget rulings

### 8.1 The dense and lane ceilings' evidence

- **Dense.**
  - F1b's coefficient of 96 is accurate at kernel level to within 3.1% in heap at 1,000 members and above (0.969–1.006). At the ceiling it is within 0.32%.
  - The O(nnz) state adds 20,449,798 B (19.5 MiB) at the ceiling.
  - **The peak falls in the n² stage:** `prepare` holds 6,158.4 MiB and SA's `entry_checked` 6,160.5 MiB. The later stages are lower (`factor` 2,574 MiB, `finish` 4,625 MiB), as plan §7.1 derived.
  - **The ceiling bounds estimated heap, not resident memory.** On macOS, RSS was 1.10 × 96·n² at the ceiling and up to 1.45 × at 1,000 members (§8.3). ROOT recorded this at the B2 stop and at B3.
- **The observation lane (lane-id).**
  - F1b's 24·P_id is the right order for CONT, the only family where the identity profile is large: 0.46–1.33 across heap and move at 100–1,000 members.
  - RV17-N2's 16P + 24P′ is closer on the move model at 1,000 members (0.98–1.17).
  - For banded families the constant under-counts by 4.5–9×, but those lanes stay under about 110 MiB at 10,000 members.
- **The dense LU lane (lane-lu):** about 16 bytes per n² entry, 1.002 × E_adm. That is about one sixth of 96·n² at the same size, so F1b's dense guard covers it wherever dense scrutiny runs it beside the gate.
- **A sparse profile ceiling (the grid ladder).**
  - The largest RCM profile measured is 25,182,450 entries (grid 128×128), at 981 MiB move-model heap: 40.8 bytes per profile entry and 351 bytes per pattern entry.
  - RF-LARGE's sparse path needs 236–305 bytes per pattern entry at 1,000 and 10,000 members (`_run_records/d/sparse_and_timing.txt`).
  - At 1,000 members and above, and on the grids, sparse heap is 0.64–0.90 of the derived E_adm, which uses upper bounds R ≤ nnz and Z ≤ nnz, as plan §7.2 expected.

### 8.2 The product-level gap

P1's product-level Linux peaks (`DETECTION/RETURN.md:224-289`, on `c61a540ea`) against K6's kernel figures on the Mac, per model and mode. The OS, the level and the allocator all differ, so this is context, not a comparison of builds.

| Mode | Members | P1 Linux RSS MiB (product) | K6 heap MiB | K6 footprint MiB | K6 RSS MiB | Heap / P1 | Footprint / P1 |
|---|---|---|---|---|---|---|---|
| dense | 10 | 34.4–34.4 | 0.5–0.6 | 2.5–2.6 | 3.5–3.7 | 0.016–0.017 | 0.072–0.077 |
| dense | 100 | 49.9–52.6 | 33.8–35.5 | 42.6–45.9 | 43.7–46.9 | 0.654–0.704 | 0.825–0.920 |
| dense | 1000 | 3373.0–3617.2 | 3199.1–3323.3 | 3340.3–3418.5 | 3412.2–4782.7 | 0.919–0.948 | 0.945–0.990 |
| sparse | 10 | 34.4–34.6 | 0.2–0.3 | 2.1–2.4 | 3.2–3.4 | 0.007–0.008 | 0.061–0.068 |
| sparse | 100 | 49.6–52.7 | 2.2–3.0 | 5.0–7.9 | 6.0–8.9 | 0.043–0.057 | 0.099–0.150 |
| sparse | 1000 | 3372.8–3617.2 | 25.3–31.1 | 34.4–48.7 | 35.4–49.7 | 0.007–0.009 | 0.010–0.013 |

- **Dense at 1,000 members:**
  - the kernel's heap is 0.92–0.95 of P1's product peak, and its footprint 0.95–0.99;
  - Q3's premise (kernel below P1) holds on both.
  - It does not hold on macOS RSS for CHAIN-AX (1.33 × P1). That was the B2 stop; the ruling moved the premise to the footprint.
- **Sparse at 1,000 members:**
  - the kernel sparse path needs 25–31 MiB of heap and 34–49 MiB of footprint;
  - P1's product run in sparse mode peaked at 3,373–3,617 MiB (3.3–3.5 GiB), the same as its dense mode.
  - The difference lies outside the kernel sparse path. K6 does not attribute it.
- **CONT at 10,000 members, sparse:**
  - the kernel heap is 243–267 MiB (move model) and RSS 371–389 MiB;
  - F1b's product-level gate records 4.82–5.17 GiB RSS;
  - F1b's attribution puts most of that in result-row publication (ROOT, B1 accepted).
- **So the dense-scrutiny and lane ceilings cannot be set from kernel figures alone.** V-P's product-level measurements are needed (§8.7).

### 8.3 macOS RSS against the footprint: not a function of size

| Mode | Size (members) | Runs | RSS / footprint, min–max |
|---|---|---|---|
| dense | 10 | 6 | 1.396–1.421 |
| dense | 100 | 6 | 1.021–1.024 |
| dense | 1000 | 4 | 1.041–1.402 |
| dense | 1364 | 1 | 1.093–1.093 |
| dense | DEC053 nine | 9 | 1.059–1.462 |
| lane-id | 10 | 6 | 1.565–1.600 |
| lane-id | 100 | 6 | 1.297–1.380 |
| lane-id | 1000 | 6 | 1.003–1.083 |
| lane-id | 10000 | 4 | 1.143–1.262 |
| lane-id | DEC053 nine | 9 | 1.366–1.594 |
| lane-lu | 10 | 6 | 1.542–1.557 |
| lane-lu | 100 | 6 | 1.097–1.104 |
| lane-lu | 1000 | 6 | 1.001–1.001 |
| lane-lu | DEC053 nine | 9 | 1.173–1.568 |
| sparse | 10 | 6 | 1.457–1.511 |
| sparse | 100 | 6 | 1.135–1.208 |
| sparse | 1000 | 6 | 1.020–1.030 |
| sparse | 10000 | 6 | 1.034–1.286 |
| sparse | DEC053 nine | 9 | 1.159–1.503 |
| sparse | grid 18240 | 1 | 1.219–1.219 |
| sparse | grid 1984 | 1 | 1.012–1.012 |
| sparse | grid 32512 | 1 | 1.271–1.271 |
| sparse | grid 480 | 1 | 1.057–1.057 |
| sparse | grid 8064 | 1 | 1.183–1.183 |

- **The small sizes:** at 10 members every mode sits near the no-op process's own RSS/footprint of 1.585 (1,818,624 / 1,147,192 B), because the baseline dominates.
- **Above that, the ratio varies from run to run and does not follow size:**
  - dense at 1,000 members is 1.041–1.402 across four models of the same size;
  - the ceiling is 1.093, below CHAIN-AX at 1,000's 1.402;
  - lane-lu at 1,000 is 1.001;
  - sparse at 10,000 is 1.034–1.286.
- **Load:** 097 (1.402) ended at a 1-minute load of 12.4 while I13 was building. The ceiling ran at a load of 5.2–6.4.
- **Hypothesis only, untested** (recorded at the B2 stop): macOS keeps freed large blocks resident, so RSS accumulates across the stages' rising and falling heap. The footprint is the measure macOS uses for memory pressure, and the one comparable with P1's Linux RSS.
- **As ruled:** the premise and ρ are read on the footprint, and the watchdog stays on RSS.

### 8.4 N10: the dense-path finding (confirmed; routed by ROOT)

Both N10 models took the same path: the dense factor refused a pivot the sparse path passes, and the dense pair witness then ran until the kill.

| | CHAIN-n01000-ROT (run 100) | TREE-n01000-AX (run 101) |
|---|---|---|
| assembly → geometry | 0.019 s | 0.018 s |
| densify | 0.059 s, 839 MiB | 0.055 s, 839 MiB |
| prepare | 1.13 s, 3,319 MiB (1.005 × 96·n²) | 1.10 s, 3,318 MiB |
| factor | refused after 86.1 s: `NumericallyUnresolved(nonpositive or cancellation-unresolved structural pivot; global_dof=Some(6001))` | refused after 65.2 s, at `global_dof=Some(6002)` |
| witness | about 1,712.7 s, in flight at the kill | about 1,733.7 s, in flight at the kill |
| wall | 1,800.1 s (timeout 1,800 s) | 1,800.1 s |
| RSS (ps) | 3,413 MiB | 3,412 MiB |

- **How the witness time is measured:** wall minus the completed stages. Start-up is about 0.1 s, the difference between 097's wall and its stage total. Both runs recorded the counts line and every stage up to the kill (N1).
- **The binary's 1,740 s budget is not checked inside the witness.** The runner's timeout killed both.
- **Class disagreement:** the sparse path publishes both models as Sensitive, so the classes differ (the only two cross-mode disagreements).
- **The voided earlier attempt of run 100** refused at the same DOF after 64.7 s.
- **The refusals fall at the last pivots** (6001 and 6002 of 6006), so almost the whole factor ran first.
  - That is consistent with, but does not prove, I15's hypothesis: the dense screen's operation count 2j + 2 exceeds the skyline's 2(i − first_i) + 2 late in a banded matrix.
- **F1b's gate part 2** shows the same timeouts at product level, on base and candidate.
- **ROOT recorded this as a T3 finding on main's dense path and routed it to a kernel follow-up slice.** That slice bounds the witness and examines the screen.

### 8.5 Time (observations with their load; no claim)

- **Dense at 1,000 members:**
  - the factor takes 27.8 s (CONT), 64.8 s (TREE-ROT) and 79.6 s (CHAIN-AX) as the median;
  - `entry_checked` takes 33–95 s;
  - a whole process (5 repeats, 1 entry repeat) takes 232–622 s.
- **The ceiling (8,190 DOFs):** the factor takes 166–188 s per repeat, `entry_checked` 181 s, and the process 1,268 s.
- **The dense factor's slope** is 2.99–3.03 over 100–1,000 members (the n³ expected).
- **Sparse at 10,000 members:**
  - the staged total per repeat is 0.53–1.29 s, and `entry_checked` 0.37–1.08 s;
  - the geometry stage takes 0.06–0.09 s, so SA's per-node edge scan is small at this size.
- **The lanes at 1,000 members:** lane-lu takes 27–72 s per solve, and lane-id about 0.01 s.

### 8.6 D-8's binary64 baseline

- W1's budget is added to the ordinary binary64 attempt, whose cost per stage and peak memory are in §6.2 and `stages_table.txt`.
- The deterministic counts W1's work scales with are in §6.1: pattern entries, free lower entries (all and nonzero), RCM profile and half-bandwidth, and contributions.
- With K4's golden work counts, those give W1's work at 10,000 members without running W1.

### 8.7 What K6 cannot give

- **Product overhead:** PP's own memory beyond SA's path. That covers the legacy lanes where they overlap the gate, the envelope, the receipts, the n ≤ 256 source-recovery view and result-row publication. This is V-P's product-level measurement (§8.2 shows it is large).
- **A target-machine policy:** how much memory the product may assume on a user's machine is the owner's decision. K6 measured one 128 GiB Mac with no swap.
- **Other platforms:** RSS and allocator behaviour on Linux and Windows. The Linux `RLIMIT_AS` path is unit-tested and live-tested only where a Linux sweep runs it.
- **W1 (K6b):** limbs per entry, work units by stage and precision, peak memory per precision, and seconds per work unit by interleaved runs. That comes after K4 merges.
- **Whether a ceiling should also bound time.** §8.4 and §8.5 show that the dense path's time can dominate before its memory does.

## 9. Derivations

### 9.1 The closed-form counts

The premises hold for RF-LARGE, the nine and the grids by construction: every node carries a frame, no two frames join the same node pair, and there are no springs, user elements or blocks.

- **Pattern entries = 36·(N + 2m).** `SparsePattern::from_connectivity` (`FK/structural/sparse.rs:82-130`) gives each node the 6×6 blocks of itself and its element neighbours. That is one diagonal block per node, plus two off-diagonal blocks per member.
  - With N = n + 1 for RF-LARGE: 36(3n + 1). This is K1's figure (`IMPLEMENTATION/K1/RETURN.md:416`).
- **Lower entries** = 18(N + 2m) + 3N.
- **Contributions** = 144·m: one per entry of each frame's 12×12 matrix, both triangles.
- **Dense entries** = (6N)².
- **Free lower entries:**
  - CHAIN and TREE: 57n − 36;
  - CONT: 63s − 18, with s = n/2.
- **The identity-order profile's full-block bound:**
  - CHAIN and TREE: 57n − 36;
  - CONT: 27s² + 36s − 18, which is 675,179,982 at s = 5,000.
- **Checked:**
  - the Rust closed forms up to 100 members and on the nine (test C1);
  - the runner's Python closed forms at every size (test H);
  - K1's table at every size and orientation (C2, within N12's scope);
  - an independent standard-library port of RCM and the skyline rule, run once for all 33 models and the extras (`_run_records/oracle/`) and pinned as constants in `k6_counts.rs`.

### 9.2 The staged sequence equals SA's entry

- **Test E** runs in both modes, on every model up to 100 members and on the nine. It asserts that the staged sequence's displacements and report `Debug` are bit-identical to `solve_assembled_with_formation_check(…, true)`, and the plain variant to `solve_assembled`.
- **The observation runs record the same equality at every size:** 300 of 300 `staged_vs_entry_checked` lines, including 10,000 members sparse and the 8,190-DOF ceiling.
- **So the stage map is the product's kernel path** on this base. This includes K5's `constrained_geometry` in the selected entry, which for frame-only models gives the same result as the public `geometry` (N6).
- **E was re-run after each main merge:**
  - at `1f354c20b` (K5), before B;
  - at `3e90176c6` (F1b), passing (§11).

### 9.3 Scope 8: no published byte changes

Both scans ran on `3e90176c6` (`_run_records/d/scope8_scans.txt`).

- **The path scan** (`git diff 59cb20073 3e90176c6`): at `3e90176c6`, before D's records, K6 changes 916 files.
  - H/ (44 files) and `P/tests/test_performance_harness_runner.py` (the Q9(b) wrapper) are the only non-record files.
  - The other 871 are under `T3/IMPLEMENTATION/K6/`.
  - Nothing in FK, SD, NI, PP, `P/core/runner/headless`, `P/validation`, `P/provenance`, `.github`, `tools/` or `P/tools` changes.
  - `H/src/lib.rs`'s diff is the one `pub mod k6;` line.
- **The dependency-closure scan:** no crate outside H depends on the harness.
  - No `Cargo.toml` outside H names `performance_harness`.
  - Outside H, the package name occurs only in `P/provenance/build-artifacts/core__solver__performance_harness__Cargo.lock:34`, which K6 does not change and nothing in `P/tests`, `P/tools`, `tools/` or `.github/` reads. It also appears in recorded build logs (68 files in execution records and `validation/evidence`), which are not build inputs.
  - The only `P/tests` readers of H are the wrapper and the two DEC-050/053 pins, which read `H/src/lib.rs` as text and pass (§11).
- **So no binary that T9 or the both-entry gate builds can change.** Neither was run (brief, Scope 8), and ROOT may ask for one.
- **The provenance lock copy** now differs from `H/Cargo.lock` (K6 added three path packages). The copy is outside K6's write set and is read by nothing.

### 9.4 Paths no observation reached

Each of these paths was reached by a test instead, or by no run at all, as stated:

- **Classifications `killed_by_rss_watchdog`, `heap_cap_abort`, `rlimit_abort`, `refused_by_binary` and `error`:** no observation run.
  - The watchdog kill is covered by the live test H1, the group-kill test (§10), `--smoke` and K6-M1 to M4.
  - The heap-cap abort is covered by F4 (`k6_bin::heap_cap_abort_is_marked`), `--smoke` and K6-M11.
  - `rlimit_abort` runs only on Linux: it has a mocked unit test, and the live path runs on a Linux sweep.
  - The binary's refusals by name are covered by test G and K6-M12. The runner refuses first, so no admitted run reached them.
- **The binary's `first_repeat_over_limit` and `time_budget` stops:** reached by no run. B3's repeat 0 took about 555 s, under 600 s. There is no dedicated test (§15).
- **The dense witness's completion:** never reached. Both N10 witnesses were killed.
- **The sparse witness:** reached on CHAIN and TREE n10000-ROT.

## 10. The mutation table (C; `_run_records/c/`)

- **The runs:**
  - each from a fresh `git archive ada18de70` copy;
  - the NONE controls first;
  - Rust with H's full `cargo test`, a fresh target per run and at most two cargo jobs;
  - the runner through the N15 pytest invocation plus `--noconftest`, since the tests' conftest only builds unrelated tool binaries.
- **The C stop:**
  - K6-M4 and K6-M26 survived the committed suite.
  - ROOT ruled two tests into `runner/test_k6_runner.py`, in existing classes: `LiveLimit.test_kills_take_the_whole_group` and `PlanAdmission.test_measured_runs_are_kept_and_not_run_records_are_run`.
  - The re-run, on fresh copies with the amended file overlaid, gave NONE 35 passed, with each survivor killed by its new test.
- **Plan §12, corrected.** M4's intended kill "the wrapper survives" was wrong.
  - Under `/usr/bin/time`, the killed binary is the wrapper's only child, so the wrapper exits by itself.
  - The new test gives the child a sleeping helper in its process group.
- **Observation: a descendant holding the runner's pipes would stall its pipe close.**
  - The first draft of that test let the helper inherit the pipes, and it passed on M4 after 31 s.
  - The group kill prevents this.

| Mutant | Side | Mutation | Result | Killed by |
|---|---|---|---|---|
| NONE (py) | py | control: unmutated archive copy | passes | 35 passed in 6.49s |
| NONE (rs) | rs | control: unmutated archive copy | passes |  |
| K6-M1 | py | watchdog never kills (comparison skipped) | killed | LiveLimit::test_live_memory_limit |
| K6-M2 | py | watchdog compares KiB with bytes | killed | LiveLimit::test_live_memory_limit; Watchdog::test_kib_against_bytes |
| K6-M3 | py | watchdog polls the time wrapper's PID | killed | LiveLimit::test_live_memory_limit |
| K6-M4 | py | SIGKILL to the child only, not the group (watchdog kill) | killed | LiveLimit::test_kills_take_the_whole_group |
| K6-M5 | py | RLIMIT_AS applied on Darwin too (not only Linux) | killed | LiveLimit::test_live_memory_limit; LiveLimit::test_negative_control; RlimitOnlyOnLinux::test_darwin_does_not |
| K6-M6 | py | time -l bytes read as KiB | killed | Parsers::test_time_l_bytes |
| K6-M7 | py | mean for median | killed | Aggregation::test_median_and_minimum |
| K6-M13 | py | admission admits an estimate > C/2 | killed | PlanAdmission::test_lane_id_deferred_by_name_when_the_estimate_fails; PlanAdmission::test_rho_is_the_largest_footprint_ratio_at_smaller_sizes_net_of_baseline |
| K6-M19 | py | rho taken as the smallest ratio, not the largest | killed | PlanAdmission::test_rho_is_the_largest_footprint_ratio_at_smaller_sizes_net_of_baseline |
| K6-M21 | py | the runner's by-name n2 refusal covers dense only | killed | PlanAdmission::test_refusals_by_name |
| K6-M22 | py | (own, B2-stop rule) projected RSS <= 0.8C check removed | killed | PlanAdmission::test_projected_rss_above_0_8_c_is_deferred_by_name |
| K6-M23 | py | (own, B2-stop rule) rho read on RSS, not on the footprint | killed | PlanAdmission::test_footprint_not_rss_decides_rho; PlanAdmission::test_projected_rss_above_0_8_c_is_deferred_by_name; PlanAdmission::test_rho_is_the_largest_footprint_ratio_at_smaller_sizes_net_of_baseline |
| K6-M24 | py | (own, B2-stop rule) the dense RSS/footprint floor 1.45 dropped | killed | PlanAdmission::test_dense_uses_the_ruled_rss_ratio_floor |
| K6-M25 | py | (own, B1 grant) load-wait hysteresis removed (resume at load <= 8) | killed | QuietHost::test_load_wait_hysteresis |
| K6-M26 | py | (own, B1 fix) the resume skip counts a not_run record as measured | killed | PlanAdmission::test_measured_runs_are_kept_and_not_run_records_are_run |
| K6-M8 | rs | pattern entries from one triangle | killed | k1_table_cross_check; closed_forms; failed targets: --test k6_counts |
| K6-M9 | rs | identity profile reported as RCM | killed | oracle_constants; k1_table_cross_check; rcm_count_equals_ordering; failed targets: --test k6_counts |
| K6-M10 | rs | realloc accounting wrong (a shrink releases nothing) | killed | k6_alloc (harness=false: assertion `left == right` failed: realloc down releases the difference); failed targets: --test k6_alloc |
| K6-M11 | rs | cap off by one (>= for >) | killed | k6_alloc (harness=false: an allocation exactly at the cap succeeds); failed targets: --test k6_alloc |
| K6-M12 | rs | the binary's >= 10,000 n2 refusal narrowed to dense only (lane-lu passes) | killed | n2_modes_at_10000_refused_before_n2; failed targets: --test k6_bin |
| K6-M14 | rs | Q3 transposed in the generator | killed | oracle_constants; loads_match_r1_at_ten_members; canonical_bytes_equal_committed_models; failed targets: --test k6_counts, --test k6_models |
| K6-M15 | rs | n - 1 members (CHAIN) | killed | peak_requested_bytes_are_deterministic; estimate_over_half_cap_refused; heap_cap_abort_is_marked; n2_modes_at_10000_refused_before_n2; closed_forms; k1_table_cross_check; rcm_count_equals_ordering; oracle_constants; model_shapes_follow_r1; canonical_bytes_equal_committed_models; outcome_parity_and_d |
| K6-M16 | rs | staged sequence skips the contribution audit (contributions None, sparse) | killed | staged_equals_sa_entries_small_models_both_modes; staged_equals_sa_entries_100_member_models_sparse; failed targets: --test k6_staged |
| K6-M17 | rs | alloc_zeroed forwarded without accounting | killed | counts_file_replaces_the_counts_phase; peak_requested_bytes_are_deterministic; estimate_over_half_cap_refused; k6_alloc (harness=false: assertion `left == right` failed: alloc_zeroed is counted); failed targets: --test k6_alloc, --test k6_bin |
| K6-M18 | rs | the basis string's family text changed (the mixed branch) | killed | staged_equals_sa_entries_small_models_both_modes; staged_equals_sa_entries_100_member_models_sparse; staged_equals_sa_entries_100_member_models_dense; failed targets: --test k6_staged |
| K6-M20 | rs | TREE branch directions swapped (+z at odd k) | killed | oracle_constants; canonical_bytes_equal_committed_models; outcome_parity_and_dec053_basis_100_member_models; failed targets: --test k6_counts, --test k6_models, --test k6_parity |

Result: 2 controls pass, and 26 of 26 mutants are killed, each by its intended test.

## 11. Suites, the merged head, and H's debug suite time

- **On `3e90176c6`** (`_run_records/d/`):
  - H's full `cargo test --offline --locked -j 8` passes: lib 25, `k6_alloc` F1 and F2, `k6_bin` 8, `k6_counts` 4, `k6_models` 7, `k6_parity` 4, and `k6_staged` 3 (test E).
  - The N15 pytest invocation passes 37 of 37: the wrapper's 35 and the two DEC-050/053 pins.
- **The merge brought one change into H's dependency closure:** NI's `s11k_tests.rs`, a `#[cfg(test)]` module (`NI/src/lib.rs:9-10`).
  - NI's library, which H links, is unchanged, and cargo rebuilt nothing.
  - The observation binary's Rust source is also unchanged since `1f354c20b`, from which B's binary was built. Since then, only the runner's Python and the test-only module have changed in the solver tree.
  - So every figure in this RETURN measures the code at `3e90176c6`.
- **H's debug suite, cold, on the Mac:**
  - base 11 s;
  - candidate 59 s at A1, and 68 s in C's NONE control from a fresh target;
  - test E alone takes 40 s.
  - The increase of 48–57 s is under the brief's two-minute line.
- **The non-test build has no warnings (A1).** The DEC-050/053 pins pass.

## 12. Toolchain and host

- **Host:** Apple M5 Max, 18 cores (18 logical), 128 GiB (`hw.memsize` 137,438,953,472), macOS 26.6.2 (25G83), Darwin 25.6.0, no swap.
- **Toolchain:** rustc 1.97.1 (`8bab26f4f`, LLVM 22.1.6) and cargo 1.97.1 (`c980f4866`). Python 3.13.14 ran the runner; the `<VENV>` is used for pytest.
- **Observation binary:** `k6_observe`, release, built from `git archive 1f354c20b` (tree `be7c0dddf`), sha256 `1bbdfb2a…d72e2` (`_run_records/b1/binary.txt`).
- **Builds and tests:** `RUSTUP_TOOLCHAIN=1.97.1`, `RUSTUP_AUTO_INSTALL=0`, `CARGO_INCREMENTAL=0`, `--offline --locked`, `-j 8` at most, `RUST_TEST_THREADS=4`, and at most two cargo jobs of I15's own.
- **Timed runs:** one observation process at a time, with the load-wait rule (wait above a 1-minute load of 8, resume below 6), `memorystatus_level` ≥ 80, and the memory guard running (floor 35%).
- **Other processes on the host:**
  - I13's builds ran during parts of B1 and B2, and each run's recorded load shows them.
  - During B3, a Codex app-server job in another project folder (nice 0, one core) and DS1's niced emulator ran alongside. ROOT accepted this as disclosed.

## 13. Interface for F1b's ceilings, D-8, K6b, V-K and V-P

### 13.1 The observation binary (`H/src/bin/k6_observe/`)

```
k6_observe --model <id> | --model-file <canonical file>
           --mode sparse|dense|lane-id|lane-lu --heap-cap-bytes <u64>
           [--repeats 5] [--entry-repeats <k>] [--time-budget-s <u64>] [--first-repeat-limit-s 600]
           [--counts-file <counts.jsonl>] [--dump-solution <file>] [--allow-over-estimate]
k6_observe --noop --heap-cap-bytes <u64>              (process baseline: start and summary lines only)
k6_observe --counts-only --model <id> --heap-cap-bytes <u64> [--dump-pattern <file>]
k6_observe --emit-model --model <id>                  (canonical k6-model v1 bytes)
k6_observe --list-models
```

- **Model ids:**
  - `RF-LARGE-{CHAIN,TREE,CONT}-n{00010,00100,01000,10000}-{AX,ROT}`;
  - `DEC053:<fixture_id>`;
  - outside the sealed set, `K6-CEIL-CHAIN-n01364-AX` and `K6-GRID-<s>x<s>`.
- **Exit codes:** 0 done (an M03 refusal is an outcome); 2 usage; 3 refused; 4 internal error. A heap-cap refusal aborts after writing `k6_observe: heap cap refused …` to fd 2.
- **`--counts-file`** takes the counts line from a counts-only record, checked by the model's canonical FNV-1a digest, and recomputes the estimates.
- **JSONL (`k6-observe-v1`), one object per line, flushed:**
  - `start`: schema, pid, model, model_from_file, mode, counts_only, repeats, entry_repeats, heap_cap_bytes, time_budget_s, first_repeat_limit_s, allow_over_estimate.
  - `counts`: counts_source, model, family, mode, §6.1's fields, `estimate_f1b_bytes_<mode>` and `estimate_adm_bytes_<mode>` for all four modes, the build's `size_*` facts, model_canonical_len and model_canonical_fnv64, and phase_elapsed_ns, phase_peak_heap and phase_peak_heap_move.
  - `refusal`: reason, estimate_adm_bytes, heap_cap_bytes, heap_peak_so_far.
  - `stage_begin`: repeat, stage, heap_current.
  - `stage`: repeat, stage, ok, elapsed_ns, heap_current_begin/end, heap_peak, heap_peak_move, alloc_calls, error.
  - `outcome`: repeat, class, failed_stage, factor_error, error, formation_demoted, load_fidelity_flagged, contribution_rounding_rows, recovery_ok, recovery_digest, solution_debug_len/_fnv64.
  - `parity`: item, repeat, equal, and fields specific to the item.
  - `summary`: repeats_completed, stop_reason, heap_peak and heap_peak_move, the same pair for the counts, repeats and parity phases, and elapsed_ns.
- **Stages:** `assembly`, `evidence`, `ledger`, `reduce`, `geometry`, `densify`, `prepare`, `factor`, `witness`, `finish`, `recovery`, `entry_checked`, `entry_plain`. For the lanes: `lane_entries`, `densify_lu`, `lane_reduce`, `lane_solve`.
- **How K6b adds a W1 mode:**
  - a `Mode` variant with its own `materializes_n2` answer;
  - a staged sequence over K4's W1 entry, whose public path is FK's `retained` export, added by its first consumer (`ROOT_RULINGS_V1.md:1149`);
  - counts-line fields for limbs per entry and work units by stage and precision;
  - an admission estimate in `counts::admission_estimate_bytes`;
  - the runner's `MODES`, `TIERS` and `estimate_key` extended.
  - The allocator, the JSONL kinds, the runner, the watchdog and the admission rule are reused unchanged. Seconds per work unit need the interleaved A, B, A, B method (`_COMMON.md:32`).

### 13.2 The counts API (`H/src/k6/counts.rs`, for K4's work formula)

```rust
pub struct K6Counts { pub nodes, members, dofs, free_dofs, restrained_dofs, pattern_entries, lower_entries,
    free_entries, free_lower_entries, free_lower_nonzero: usize, pub rcm_profile_entries: u128,
    pub rcm_half_bandwidth: usize, pub identity_profile_entries: u128, pub identity_half_bandwidth: usize,
    pub lane_entries, lane_off_diagonal, contributions: usize, pub dense_entries: u128 }
pub fn compute(model: &K6Model, frames: &[FrameElement], sink: PatternSink<'_>) -> Result<K6Counts, String>;
pub fn rcm_profile(dimension: usize, entries: &[SymmetricMatrixEntry]) -> Result<(u128, usize), String>;
pub fn f1b_estimate_bytes(mode: Mode, c: &K6Counts) -> Option<u128>;          // 96·n² or 24·P_id, quoted
pub fn base_bytes(c: &K6Counts, s: &SizeFacts) -> u128;                       // E_base, plan §7.1
pub fn admission_estimate_bytes(mode: Mode, c: &K6Counts, s: &SizeFacts) -> u128;
pub fn parse_counts_line(line: &str) -> Option<(String, K6Counts, u64)>;
pub const F1B_DENSE_BYTES_PER_ENTRY: u128 = 96; pub const F1B_LANE_BYTES_PER_PROFILE_ENTRY: u128 = 24;
```

- `k6::models::model(id)`, `sealed_model_ids()` and `extra_model_ids()` give the models.
- `k6::staged::{setup, staged_solve, entry, lane_id, lane_lu}` take a `&mut dyn StageObserver`.

### 13.3 The runner (`H/runner/k6_runner.py`)

- **CLI:**
  - `--plan --counts <counts.jsonl>`: the schedule, with each run's estimate, cap, admission and reason; it starts no child.
  - `--smoke --binary <b> --records <d> --counts <c>`.
  - `--run --tier <T1..T6> --binary --counts --records --source-commit --source-tree`: one tier, one process at a time, after the quiet-host wait.
  - `--packet --records <d>`.
  - `--project --records <d> --counts <c>`.
  - `--emit-model <id>` and `--model-hashes`: the independent Python generator.
- **The record (`k6-runner-record-v1`, one JSON object per process, appended to `records.jsonl`):**
  - run_id, system, argv (basenames only), wrapper, rss_cap_bytes, timeout_s, poll_s, rlimit_as_applied;
  - load_before/after, memorystatus_before/after;
  - classification (one of ok, timed_out, killed_by_rss_watchdog, heap_cap_abort, rlimit_abort, refused_by_binary, error), with classification_detail;
  - exit_code, wall_s, binary_pid_found, killed_by_rss_watchdog, timed_out, kill_after_s;
  - watchdog_last_kib, watchdog_polls, watchdog_samples, survivors;
  - rss: wait4_maxrss_bytes, ps_max_kib, ps_samples, and `time`'s time_max_rss_bytes and time_peak_footprint_bytes, all in bytes;
  - peak_rss_bytes and peak_rss_source;
  - summary, stdout_lines, stdout_sha256, stderr_tail (sanitized), and repeats_heap_peak/_move.
  - `run_tier` adds the schedule fields, `admission`, baseline_rss_bytes, baseline_footprint_bytes, source_commit, source_tree, quiet_host, peak_footprint_bytes, estimate_adm_bytes, estimate_f1b_bytes, `stages` (per stage: samples, median_ns, min_ns, heap peaks, ok) and parity_failures.
  - `cross_mode` rows compare sparse and dense per model.
- **The packet (`k6-packet-v1`):** schema, observation_only, claims, no_thresholds, metadata (no host identifiers), runs, not_run, cross_mode, fits (heap_move and rss_net by family and mode) and f1b_ratios.
- **The admission rule** (`admission()`, as ruled at the B2 stop):
  - never, by name, for an n² mode at 10,000 members or more, and for CONT n10000 lane-id;
  - the previous size of the same family, orientation and mode must be recorded (the ascent);
  - ρ_fp = the largest max(footprint − baseline footprint, move heap)/E_adm at smaller sizes of the same family and mode, measured at 100 members or more for a run at 1,000 or more; 2 where none has been measured;
  - admitted if P1's Linux peak ≤ C/2 (the footprint estimate is then max(P1, E_adm)), or E_adm·ρ_fp ≤ C/2;
  - then, projected RSS = the footprint estimate × the largest RSS/footprint in the family and mode (1.45 as the default, and as a floor for dense) must be ≤ 0.8·C; otherwise deferred by name;
  - C = 8 GiB, with the heap cap at C − 512 MiB, and 16 GiB for T6. ρ_rss is recorded beside ρ_fp.

### 13.4 Measured ratios for F1b's constants: what they cover

- **96·n² (dense):**
  - covered: 66, 606, 6,006 and 8,190 DOFs on RF-LARGE and the ceiling chain, and 54–336 DOFs on the nine;
  - not covered: other topologies at 1,000 members or more, and anything above 8,190 DOFs.
- **24·P_id (lane):**
  - covered: CHAIN and TREE at 10–10,000 members, CONT at 10–1,000, and the nine;
  - not covered: CONT at 10,000 members (never run), and other lane-heavy topologies.
- **Sparse and lane-lu:** F1b states no constant. K6's derived E_adm and the measured ratios are in §7.2 and §8.1.

### 13.5 What ROOT still needs for each ruling

- **The dense-scrutiny and lane ceilings:** PP's product-level peak and its attribution (V-P), the target-machine policy (owner), other platforms, and whether time is also bounded (§8.4, §8.5).
- **D-8's W1 limits:** K4's work counts, K6b's seconds per work unit and memory per precision, and V-K's kernel-lane W1 runs.
- **N10:** the kernel follow-up slice ROOT has routed.

## 14. What was not done

- **W1 (Q1(a)):** that is K6b's, after K4 merges.
- **Linux or Windows observations:** none. The `RLIMIT_AS` path is unit-tested, and it runs live only where a Linux sweep runs the wrapper.
- **Product-level runs:** those are V-P's (`DESIGN.md:834`).
- **Dense at 10,000 members:** never, by ruling. The estimate is 96 × 3,600,720,036 = 345,669,123,456 B, about 322 GiB.
- **CONT n10000 lane-id:** never, by ruling.
- **T9 and the both-entry gate:** not run, since byte identity holds by construction (§9.3).
- **No native witness:** a join item.

## 15. Disclosures

1. **At A1**, the DEC-053 basis was breached on three 100-member Sensitive models (at up to 7.15e-8). ROOT ruled option (a): the basis is asserted where both modes are Passed.
2. **At A1**, an index slip (`git add -N` and `git reset -q` on `H/`) had nil net effect. ROOT recorded it.
3. **Runner changes made during B1, each tested** and committed at `962dd4e3b`:
   - `run_entries()` so that `cross_mode` rows are not read as runs;
   - the resume skip;
   - `exited()` via `waitid(WNOWAIT)` in `find_child`;
   - the load-wait hysteresis;
   - `cross_mode` for the sparse and dense modes only;
   - the recorded macOS `time -l` fixture in place of a provisional one;
   - closing the pipes after the pumps join.
4. **The B2 stop** (Q3's premise refuted on RSS) led to the ruled footprint rule in the runner, with four tests, committed at `3799e3764`.
   - B2 and B3 ran through two small drivers (`_run_records/b2/b2_driver.py.txt` and `b3/b3_driver.py.txt`). Each calls the committed runner unchanged and adds only `runner_commit` to each record; B2's driver also lifted the grid's conditional flag, which ROOT had admitted.
   - A one-shot guard (`t5_guard.sh.txt`) would have stopped B2 before the grid on any non-N10 divergence. It allowed T5.
5. **B2 was paused** for F1b's DEC-025 slot. Run 100 was voided in its witness stage and re-run.
   - The per-tier no-op baselines are re-run on each resume, which overwrote the scratch copies only.
6. **During B3** a process outside this work ran on the host. ROOT accepted it as disclosed.
7. **At C**, two mutants survived and ROOT ruled two tests in (§10). The plan's §12 claim about M4 is corrected. The runner runs used `--noconftest`.
8. **The binary's time-budget and first-repeat stops** have no dedicated test and were reached by no run (§9.4).
9. **The packet** (`H/observations/k6/k6_packet.json`) and `H/observations/k6/SHA256SUMS` were written at D, not at B as plan §1 said. The packet is the runner's own `--packet` output over the combined B records, and it regenerates byte for byte from the committed records (§16).
10. **N20:**
    - the section differs from the exact value by 9 ulp against 0.0019π (plan §3.2);
    - `/bin/ps` is setuid (plan §9);
    - there is no Linux observation, and the `-v` parser fixture is constructed;
    - the only Linux peaks are P1's, at product level.

## 16. Records (`T3/IMPLEMENTATION/K6/`)

- **`PLAN_CHECKPOINT0.md`, this `RETURN.md`, `CHANGE_RECORD.md`, and `SHA256SUMS`** over every other file in the folder.
- **`_run_records/`:**
  - `a1/`: the lock diff, the `lib.rs` diff, the suites and the DEC-053 breach;
  - `a2/`: the plan (`plan.txt`), `--smoke`, the N15 pytest and the runner's tests;
  - `crosscheck/`: the 24-model check against `references.py` and the literal-model check;
  - `models/`: the two generators' hashes;
  - `oracle/`: the independent RCM and skyline port, and its outputs;
  - `b1/`, `b2/`, `b3/`: raw JSONL, runner records, `.time.txt`, `.stderr.txt`, `.u` solution dumps, the per-tier baselines, `records*.jsonl`, the metadata, logs and drivers;
  - `c/`: the mutation table, the 32 logs, the scripts and the proposed-test log;
  - `d/`: the merged-head suites, the Scope 8 scans, and the tables this RETURN embeds, with their generators and the packet-reproduction procedure.
- **Tracing.**
  - Every number in §5–§8 comes from `_run_records/d/tables/`, which `d_tables.py.txt` computes from the committed B records, combined as `_run_records/d/packet_reproduction.txt` describes.
  - The combined `records.jsonl` (166 lines, sha256 `fcc89445…`) equals the runner's working records. The packet's sha256 (`b5419aee…`) is the same from either.
  - Each raw JSONL's sha256 is in its runner record (`stdout_sha256`).

## RETURN addendum 1 (RV18's review; fixes on `ae3320b5a`)

- **Verdict:** RV18's independent review PASSED at `ae3320b5a`, with 0 BLOCKING, 4 SHOULD-FIX and 8 NOTEs (`REVIEW/K6_REVIEW.md` on the numerics branch).
- **Rulings:** `ROOT_RULINGS_V1.md`, "K6: rulings on RV18's review" (numerics `21776d312`):
  - fix all four SHOULD-FIX findings before merge;
  - fix NOTEs N1–N3, N6 and N7 where cheap;
  - record N4, N5 and N8.
- The fixes are uncommitted in `<wt>/k6`, for ROOT's commit. The records are in `_run_records/rv18/`.

### A1.1 Files

| File | Lines | Change against `ae3320b5a` |
|---|---|---|
| `H/runner/k6_runner.py` | 1,412 | +106 −43: the poll loop is moved verbatim into `_watch`, with the same indentation, so every earlier mutant's text still applies; `stop_group`, `sigterm_raises_exit` and `read_time_file` are added, and `launch` calls `_watch` in `try`/`finally` (RV18-3, N2) |
| `H/runner/test_k6_runner.py` | 717 | +96 −1: four tests, and `test_sanitize` extended (RV18-3, N1–N3); still in existing classes, so the wrapper is unchanged |
| `H/src/k6/models.rs` | 566 | +22 −3: `is_cont_n10000` decided from the model's content (N6) |
| `H/tests/k6_bin.rs` | 455 | +178 −3: three tests and three small line helpers (RV18-1, RV18-2, N6) |
| `T3/IMPLEMENTATION/K6/_run_records/c/logs/py-K6-M5.log` | — | RV18-4: the four machine-path lines scrubbed |
| `CHANGE_RECORD.md`, this addendum, `_run_records/rv18/`, `SHA256SUMS` | — | records |

The fixes touch only H's runner, one generator predicate and two test files, plus K6's records, so **Scope 8 is unchanged** (§9.3). `H/observations/k6/` is unchanged, and its `SHA256SUMS` still verifies.

### A1.2 The four SHOULD-FIX findings

- **RV18-1, the time stops.** New `k6_bin::first_repeat_and_time_budget_stops`, on RF-LARGE-CHAIN-n00010-AX sparse.
  - **The three runs RV18 probed,** with `--repeats 3`:
    - a first-repeat limit of 0 s gives 1 repeat and `first_repeat_over_limit`;
    - a budget of 0 s gives 1 repeat and `time_budget`;
    - a budget of 3,600 s gives 3 repeats and a null reason.
  - **A fourth run, which the ruling did not list.** RV18-M2 multiplies the budget by 10^6, and 0 × 10^6 = 0, so at a zero budget M2 behaves exactly like the unmutated binary.
    - A budget in whole seconds is exceeded only by a run of at least 1 s. The fourth run makes one: 1,000 repeats under a 1 s budget stop by time. That was about 50 repeats in debug here, and the stop would still come long before the last repeat on a machine 20× faster.
    - The test takes about 1.1 s (`k6_bin`'s total is 1.7 s), not the "under 1 s" of the ruling.
  - **Result:** RV18-M1 fails at the first run (3 repeats, null). **RV18-M2 passes the first three runs and fails only at the fourth** (all 1,000 repeats ran, with a null reason), which shows the fourth run is needed.
- **RV18-2, the headline peak.** New `k6_bin::summary_peak_is_the_largest_stage_peak_and_stage_peaks_restart`, on dense CHAIN-n00010-AX with `--repeats 2 --entry-repeats 1`.
  - It asserts:
    - `repeats_heap_peak` equals the largest stage `heap_peak` (579,691 B), and the move model likewise;
    - the process `heap_peak` is at least that;
    - repeat 1's `assembly` peak is below repeat 0's `prepare` peak (562,391 B).
  - **Result:** RV18-M3 is killed (the summary reports 145,351 B, the last stage's peak), and RV18-M4 is killed (repeat 1's `assembly` reports 579,691 B).
- **RV18-3, an interrupted runner.**
  - **The change in `launch`:**
    - `sigterm_raises_exit()` installs a SIGTERM handler that raises `SystemExit(143)`, for the life of the observation process, in the main thread only, where no other handler is installed. The previous handler is restored afterwards.
    - `launch` starts the process and runs `_watch` inside `try`. Its `finally` SIGKILLs the process group and reaps the wrapper if the loop did not end by itself (SIGTERM, SIGINT or an error).
  - **Why in `launch`, not `main()`:** installing the mapping there also covers drivers that call `run_tier` directly, as B2's and B3's did. RV18's B2-pause case was exactly such a driver.
  - **Test:** `LiveLimit.test_a_terminated_runner_leaves_no_survivor`.
    - A driver process runs `launch` on a sleeping child, which starts the sleeping helper and records its process group.
    - SIGTERM goes to the driver. The driver must exit with 143 and leave no process of the group.
    - The cap is 1 GiB, so Linux's `RLIMIT_AS` does not bind on the interpreters.
  - **Result:** own mutants K6-M27 (the `finally` kill removed) and K6-M28 (SIGTERM not mapped) are both killed.
- **RV18-4.** The four lines of `_run_records/c/logs/py-K6-M5.log` now read `<tmp>` for the per-user temporary directory and `<VENV>` for the shortened home path.
  - GEN-8's own `MACHINE_ABS_PATH_RE` (`tools/practitioner_harness/surface_roles.py:22`), applied to every file this PR adds or changes (1,145 files with this addendum's records), finds 0 hits (`_run_records/rv18/scan_gen8.py.txt`).
  - That includes RV18-N2's synthetic test string, now built at run time.

### A1.3 The NOTEs

- **N1:** `Watchdog.test_poll_interval_is_the_designs_100_ms` pins `POLL_S = 0.1` and `launch`'s default (DESIGN.md:826). It kills RV18-M6.
- **N2:**
  - `test_sanitize` now also covers a home-directory (`Users`) path, a `private/tmp` path and a `home` path, all built at run time. It kills RV18-M7.
  - `read_time_file` replaces GNU `time -v`'s "Command being timed" line with `<omitted>` and sanitizes the rest of the output file in place before the file becomes a record. macOS `time -l` names no path.
  - `Parsers.test_time_v_output_file_is_scrubbed` checks this, and kills own mutant K6-M30.
- **N3:** `QuietHost.test_a_running_sweep_alone_makes_the_host_busy`: `host_busy` reports `['sweep']`, and the wait sleeps once. It kills RV18-M8.
- **N6:** `K6Model::is_cont_n10000` is now true when there are at least 10,000 members and either the family is CONT or the model carries CONT's restraint signature. The signature is m + 1 nodes, the first fully fixed, the next m/2 pinned in UX, UY and UZ, and nothing else restrained.
  - `k6_bin::renamed_cont_n10000_lane_id_refused` emits R1's CONT-n10000-AX, renames its id and passes it through `--model-file`. It is refused by name, before any count.
  - It kills own mutant K6-M29 (the id-prefix test restored), under which the renamed model is refused only by the estimate (`estimate_exceeds_half_cap`).
  - **No record changes.** On all 39 committed models the new predicate gives the old value:
    - CONT n10000 AX and ROT are CONT with 10,000 members, so both predicates are true;
    - every other model has fewer than 10,000 members, or lacks the signature: CHAIN and TREE at 10,000 members have one restrained node, and the grids have no m + 1 nodes;
    - B's binary (`1bbdfb2a…`) predates the change, and no admitted run reached the refusal, since the runner refused both CONT n10000 lane-id rows by name first.
- **N7:**
  - **(a) The provenance lock copy** (`P/provenance/build-artifacts/core__solver__performance_harness__Cargo.lock`) already differed from `H/Cargo.lock` at main `59cb20073`: it lacks `sparse_direct`.
    - §9.3's "now differs from `H/Cargo.lock` (K6 added three path packages)" overstates K6's part. The copy was already stale on main, and K6's three packages widen the difference.
    - It is still read by nothing.
  - **(b)** CHANGE_RECORD's base is corrected to `56dd72334`, the first K6 commit's parent. The brief cited `d1cc97ce4`, whose piping tree is the same.
  - **(c) Run 137 (grid 128×128)** carries `slot: B1`, its planned slot, in its raw record and in the packet, though it ran in B2. Its `runner_commit` is B2's, and §5 counts it in B2. Raw records are not edited, so the field stays as written.
- **Recorded, not changed (as ruled):**
  - **N4:** the runner trusts the binary's `equal` flags. The records carry both digests, and RV18 re-derived all 300 + 300 + 116 parity lines from them. Test E itself compares the full `Debug` strings.
  - **N5:** the B2-stop rule as implemented takes the RSS-to-footprint ratio over smaller measured runs of the family and mode, with 1.45 as the dense floor and default. The P1 branch projects from max(P1, E_adm). Neither changes an admission on the records.
  - **N8:** the Linux live path has never run. The first Linux sweep that collects the wrapper will be its first live run, and `run_tier` on Linux needs GNU `/usr/bin/time`.

### A1.4 Runs (`_run_records/rv18/`)

- **H's full suite** (`cargo test --offline --locked -j 4`, `RUST_TEST_THREADS=2`) passes: lib 25, `k6_alloc` F1 and F2, `k6_bin` 11 (was 8), `k6_counts` 4, `k6_models` 7, `k6_parity` 4, `k6_staged` 3. That took 69 s, warm.
- **The non-test build** has no warnings, and rustfmt was applied to the two Rust files.
- **The N15 pytest invocation** passes 41 of 41: the wrapper's 39 (was 35) and the two DEC-050/053 pins.
- **Mutation re-run** (`mutation_table_rv18.txt`): each run is a fresh `git archive ae3320b5a` copy with the four changed files overlaid, and the mutants applied with RV18's own script, verbatim, or with I15's.
  - Both NONE controls pass: Rust (a fresh target, 76 s) and runner (39 passed).
  - RV18-M1, M2, M3, M4, M6, M7 and M8 are killed, each by its intended new test.
  - K6-M4 and RV18-M9 are re-run after the loop moved into `_watch`. Both are still killed by `test_kills_take_the_whole_group`.
  - Own mutants K6-M27, M28, M29 and M30 are killed.
  - **Result: 13 of 13 killed.**
- **`--smoke` rerun**, with the amended runner and a release binary built from the archive with the overlay (sha256 `8164db33…`, `smoke_binary.txt`):
  - 42 of 42 model runs ok, with 0 parity failures;
  - `staged_vs_entry_checked` equal on 84 of 84 lines (as at A2);
  - the watchdog killed its child at 136,432 KiB against the 128 MiB cap, after 16 polls at 0.1 s, with no survivor;
  - the heap-cap abort was classified `heap_cap_abort`;
  - 9 s in all.
- **Unchanged:** B's observation binary and every B record, the packet and D's tables.
