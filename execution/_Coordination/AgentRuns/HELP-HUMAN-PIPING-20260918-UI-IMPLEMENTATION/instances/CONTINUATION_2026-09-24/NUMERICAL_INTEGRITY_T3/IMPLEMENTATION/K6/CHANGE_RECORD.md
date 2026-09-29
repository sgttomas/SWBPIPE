# K6 change record: kernel memory and runtime observations, and the runner

This is the draft PR record for slice K6 of T3 (numerical integrity), following `.agents/skills/chirality-change/SKILL.md`. It was implemented by I15 (TASK). The details are in `RETURN.md`.

- **Branch:** `codex/piping-k6-20260928`, from main `d1cc97ce4`, whose piping tree equals `24dea2dae`'s.
- **Commits (made by ROOT):**
  - `9ababe4f2`: A1 and A2, the Rust side, the runner and the dry run;
  - `1f354c20b`: main `1cdeae2c1` (K5) merged in, with E and H's suite re-run;
  - `962dd4e3b`: B1, the observation records T1–T5, and the runner fixes made in the slot;
  - `3799e3764`: B2, dense and lane-lu at 1,000 members and grid 128×128, and the B2-stop rule in the runner;
  - `ada18de70`: B3, the Q4 ceiling run;
  - `014b2ae04`: C, the mutation table, with two runner tests added at the C stop;
  - `3e90176c6`: main `59cb20073` (F1b, #1052) merged in. It overlaps no K6 file.
- **Proposed next commit:**
  - checkpoint D: this record, `RETURN.md`, `_run_records/d/` and `SHA256SUMS`;
  - in H, `observations/k6/k6_packet.json`, `observations/k6/SHA256SUMS` and one README line.
- **Size (the product tree, against main `59cb20073`):**
  - at `3e90176c6`, 45 files, all under H except the one pytest wrapper;
  - +6,414 lines of code (Rust 4,365, Python 2,004, Cargo 45), plus 2,856 lines of models, counts, hashes and README;
  - `H/src/lib.rs` is +1 (`pub mod k6;`).
  - Records are under `T3/IMPLEMENTATION/K6/`.
- **Basis:**
  - D1 `DESIGN.md` revision 5a.2 (`fb62ef4a…`): §4.8 and the K6 row of §6;
  - the I15 brief (`TASK_BRIEFS/I15_K6_IMPLEMENTATION.md`), with ROOT's rulings on Q1–Q13 and stale-design items 1–12;
  - the checkpoint-0 plan (`4b4d9b27…`), with rulings N1–N20;
  - `ROOT_RULINGS_V1.md`'s K6 sections, from the A1 stop to B3 accepted, and ROOT's C-stop ruling.
- **Platform:** Mac (`aarch64-apple-darwin`, Apple M5 Max, 128 GiB), rustc 1.97.1. **Every observation is Mac-only.**

## What changes

- **New in H: `src/k6/`.**
  - The R1 model generator, keeping R1's node order, in exact integer arithmetic, with the section from explicit products and `PI`.
  - The canonical `k6-model v1` bytes.
  - The deterministic counts.
  - A staged kernel sequence at public-API boundaries, pinned bit for bit to SA's two entries (test E).
  - The two legacy lanes.
  - Bitwise K and the DEC-053 delta.
- **New in H: the `k6_observe` binary.** One model in one mode per process, five repeats, JSONL stage lines, and a counting, capped allocator that lives only in the binary. It refuses by name every n² mode at 10,000 members or more, CONT n10000 lane-id, and any estimate above half the cap.
- **New in H: `runner/k6_runner.py`** (standard library only).
  - It runs the binary under `/usr/bin/time`. On macOS it has a 100 ms RSS watchdog on the binary's PID, with a process-group SIGKILL; on Linux it sets `RLIMIT_AS`.
  - It classifies each process, aggregates the medians and minima, and applies the ruled admission rule.
  - It has `--plan`, `--smoke`, `--run`, `--packet` and `--project`, and an independent Python generator that reproduces all 33 model hashes.
- **New in H: `observations/k6/`.** The 21 small models, `models_sha256.txt`, `counts.jsonl`, and, at D, the packet and `SHA256SUMS`.
- **Tests:**
  - H: `tests/k6_{alloc,bin,counts,models,parity,staged}.rs` (B–G) and `runner/test_k6_runner.py` (H, 35 tests);
  - `P/tests/test_performance_harness_runner.py` (Q9(b)), which puts the runner's tests on the DEC-025 pytest surface.
- **Unchanged:** FK, SD, NI, PP, the headless runner, the legacy harness (functions, constants, tests and example), `P/validation/benchmarks/**`, `P/provenance/**`, `.github/**`, `tools/**` and `P/tools/**`.

## What differs from the design's letter (ROOT's rulings)

- **Q1(a):** binary64 now, with W1 in K6b after K4.
- **Q2(b):** H depends on NI, so the observed path is SA's.
- **Q3:** C = 8 GiB, with the heap cap at C − 512 MiB, and the ascent rule. Q3 was amended twice:
  - at the A1 stop, ρ is taken net of a no-op baseline, and at 100 members or more for a run at 1,000 or more;
  - at the B2 stop, ρ and the premise are read on the macOS footprint, the watchdog stays on RSS, and a run also needs a projected RSS ≤ 0.8·C.
- **Q4:** the 1,364-member ceiling chain at 16 GiB. **Q5:** the grid ladder. Both are disclosed as outside the sealed set.
- **Q6(a):** public-API stages, with the bundling disclosed.
- **Q8(a):** the packet is in H.
- **Q12 and RV16-N4:** the lanes are observed; no n² mode runs at 10,000 members or more; CONT n10000 lane-id is never run.
- **The A1-stop ruling:** the DEC-053 basis is asserted only where both modes are Passed. §4.8 item 3 is read as applying to Passed publications.

## Results

- **The two claims (observations, not thresholds).**
  - **Kernel sparse heap is linear in members:** the log-log slope is 0.998–1.002 from 10 to 10,000 members on CHAIN, TREE and CONT. Dense is n²: 1.974–1.976 over 100–1,000 members.
  - **F1b's dense 96·n² is accurate at kernel level:** heap is 0.969–1.006 × at 1,000 members and 1.0032 × at the 8,190-DOF ceiling (1.0027 × the 6 GiB provisional ceiling).
  - **F1b's lane 24·P_id** is the right order for CONT (0.46–1.33 in heap and move) and under-counts the banded families by 4.5–9×, at small absolute sizes.
- **For the ceiling ruling (recorded by ROOT at B2 and B3):**
  - macOS RSS runs 1.03–1.45 × 96·n² at 1,000 members and at the ceiling;
  - the RSS excess over the footprint is not a function of size;
  - product-level memory is far above the kernel's: sparse at 1,000 members is about 1% of P1's product peak, and CONT n10000 is 0.24–0.26 GiB of heap in the kernel against 4.8–5.2 GiB RSS at product level.
- **N10, confirmed, on main's dense path, not K6's code.** The dense Cholesky refuses a pivot at DOF 6001 or 6002 of 6006 on CHAIN-n01000-ROT and TREE-n01000-AX, and the O(n⁴) dense witness then runs until the 1,800 s kill. The sparse path publishes both models as Sensitive. ROOT routed this to a kernel follow-up slice.
- **The schedule:** 124 of 138 rows were measured (122 ok, and 2 timed out, the N10 pair); 14 were refused by name. No admitted run was killed by the watchdog or aborted at the heap cap. The memory guard logged no event.
- **Parity:**
  - staged equals SA's entry on all 300 lines, at every size;
  - bitwise K holds on 25 of 25 dense runs;
  - the cross-mode classes agree on every model except the N10 pair;
  - the DEC-053 basis holds on all 19 both-Passed models (≤ 5.9e-11).
- **Mutations:** 26 of 26 killed, with the NONE controls first. Two survivors of the committed suite (K6-M4 and K6-M26) are killed by the two tests ROOT ruled in at the C stop.
- **On `3e90176c6`:**
  - H's full suite passes, including E;
  - the N15 pytest passes 37 of 37, including the two DEC-050/053 pins;
  - the Scope 8 scans show that no crate outside H depends on the harness and that no published byte changes.
  - H's cold debug suite went from 11 s to 59–68 s.

## Limits

- **Mac only:** no Linux or Windows observation. The Linux `RLIMIT_AS` path is unit-tested, and it runs live only on a Linux sweep.
- **Not measured by K6:**
  - product-level overhead (V-P);
  - a target-machine memory policy (the owner);
  - W1 (K6b);
  - whether a ceiling should bound time.
- **Timings carry their recorded load and are context only.** No comparison of builds is made.
- **The binary's time-budget and first-repeat stops** were reached by no run, and have no dedicated test.

## Gates (ROOT runs the PR)

- An independent complete-diff review on the exact head.
- Hosted CI, with the full-SHA dispatch of `piping-desktop-e2e.yml`.
- DEC-025 under the owner's Mac decision.
- GEN-8 before the records commit that goes to main.
- T9 and the both-entry gate are not run (Scope 8: byte identity holds by construction; RETURN §9.3).

## Downstream notices

- **F1b's ceilings (owner decision):** §7.2 and §8 of RETURN give the measured ratios, the product-level gap and the macOS RSS observation.
- **D-8 and F2a:** the binary64 baseline and the counts (RETURN §6 and §8.6).
- **K6b, V-K and V-P:** they reuse the runner and the binary by path (RETURN §13).
- **N10:** routed by ROOT to a kernel follow-up slice on the T3-close list.
