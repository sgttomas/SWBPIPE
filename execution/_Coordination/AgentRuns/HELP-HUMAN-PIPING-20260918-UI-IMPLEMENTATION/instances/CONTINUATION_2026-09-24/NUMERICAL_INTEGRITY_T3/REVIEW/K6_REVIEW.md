# RV18: independent full-diff review of slice K6

- **Reviewer:** RV18 (Type 2 TASK, independent reviewer).
- **PR:** #1053, `codex/piping-k6-20260928`.
- **Head reviewed:** `ae3320b5a` (`ae3320b5a93aa0e65a71483b4db6f40eeb9d8f1c`), base main `59cb20073`.
- **Date:** 2026-09-29.
- **Verdict: PASS.** No BLOCKING finding. 4 SHOULD-FIX and 8 NOTEs.

**In short.**
- **No product byte changes (Scope 8).** I re-ran both scans. The PR adds 935 files and modifies 4, all in `H/`, apart from the pytest wrapper and K6's records. `H/src/lib.rs` changes by exactly `+pub mod k6;`. The lock gains three in-repo packages and no registry package, and no crate outside `H` depends on the harness. The two DEC-050/053 pins pass.
- **The binary is correct where it counts.**
  - Test E compares the complete `Debug` of SA's result, and I re-derived the binary's 300 + 300 parity lines from their recorded digests.
  - My reading of the allocator finds its accounting right.
  - The refusals by name are in the binary, and K6-M12 is killed on re-run.
  - My own ports of SD's RCM and skyline agree with `counts.jsonl` on 21 models. The product's own counts agree at every measured size.
  - My independent cross-check against `references.py` finds 24 of 24 models equal, `y_reference` included.
- **The runner is correct as ruled.** The watchdog targets the process group and the PID under `time`, `RLIMIT_AS` applies on Linux only, the parser units are right, and admission follows ROOT's rulings. The records pass my ascent and admission checks.
- **The records hold.** Both `SHA256SUMS` verify (891 and 24 files). The packet and D's 11 tables regenerate byte for byte. Every RETURN number I traced matches my own recomputation from the raw JSONL.
- **The merges are clean.** `1f354c20b` and `3e90176c6` each add exactly main's delta, with an empty remerge diff.
- **The four SHOULD-FIX findings:**
  - RV18-1: the disclosed gap. The time-budget and first-repeat stops have no test, and two mutants survive. **A test is needed before merge.**
  - RV18-2: two further mutants survive. One puts the wrong number in the claims' headline quantity (`repeats_heap_peak`); the other flattens the per-stage peaks.
  - RV18-3: a SIGTERM or Ctrl-C to the runner leaves the observation process running with no watchdog and no timeout. This happened at B2's pause.
  - RV18-4: one committed log carries a machine-specific temporary path and a shortened home-directory path.

## Findings

| ID | Class | Site | Evidence | Fix |
|---|---|---|---|---|
| RV18-1 | SHOULD-FIX | `H/src/bin/k6_observe/main.rs:715-725`: the `first_repeat_over_limit` and `time_budget` stops. This is the gap ROOT asked me to judge. | **No test reaches either stop.** No test passes `--first-repeat-limit-s` or `--time-budget-s` (`H/tests/k6_bin.rs`), and no observation reached them (RETURN.md:750).<br>**Two mutants survive H's whole suite:**<br>– RV18-M1: the first-repeat stop never fires;<br>– RV18-M2: the budget multiplied by 10^6.<br>(`mutations/mutation_table.txt`, `logs/rs-RV18-M1.log`, `rs-RV18-M2.log`)<br>**Neither is equivalent.** On the unmutated binary, CHAIN-n00010-AX sparse `--repeats 3` gives:<br>– `--first-repeat-limit-s 0`: 1 repeat, `first_repeat_over_limit`;<br>– `--time-budget-s 0`: 1 repeat, `time_budget`;<br>– `--time-budget-s 3600`: 3 repeats, null.<br>(`probes/probe_stops.out`)<br>**Why before merge:**<br>– It is Q7's ruled behaviour.<br>– B3's repeat 0 took 548.6 s of stage time against the 600 s limit (91%, `checks/lane_gap_ceiling.txt`).<br>– K6b reuses the binary for W1, whose first repeats will be longer.<br>– If a stop broke, the runner's timeout kill would take its place. That loses the `time -l` footprint, the summary and the heap peaks, as the two N10 records show. | Add one `k6_bin` test covering the three probe runs above. It runs in under 1 s in debug. Show RV18-M1 and RV18-M2 killed. |
| RV18-2 | SHOULD-FIX | `main.rs:261-262`, the summary's `repeats_heap_peak(_move)` as the maximum of the stage peaks; and `main.rs:244`, `alloc::stage_reset()` at each stage's start. | **Two mutants survive H's whole suite:**<br>– RV18-M3: the summary takes the last stage's peak;<br>– RV18-M4: the stage peaks never restart.<br>F3 (`k6_bin.rs:42-67`) checks only that two runs agree and that the peak is positive.<br>**What they change.** On dense CHAIN-n00010-AX, `--repeats 2 --entry-repeats 1` (the configuration of every dense 1,000-member run and of the ceiling):<br>– the true peak is 579,691 B, and M3 reports 145,351 B;<br>– under M4, assembly, prepare and recovery all report 579,691 B.<br>(`probes/probe_m3_m4.out`)<br>**Why it matters.** `repeats_heap_peak` is the quantity behind both of RETURN's claims (the fits and F1b's ratios) and behind admission's ρ. The stage peaks carry RETURN §8.1 and N10.<br>**The committed records are unaffected.** Every measured run's summary equals the maximum of its stage peaks (`checks/digest_check.out`). | In `k6_bin`, on that run, assert:<br>– the summary's peak equals the maximum of the stage lines' `heap_peak`, and the same for the move model;<br>– `heap_peak` is at least that;<br>– repeat 1's `assembly` peak is below repeat 0's `prepare` peak.<br>Show RV18-M3 and RV18-M4 killed. |
| RV18-3 | SHOULD-FIX | `H/runner/k6_runner.py:876-917`. The child starts in a new session (`start_new_session=True`), and the poll loop has no `try/finally`. `main` (`:1302-1345`) installs no SIGTERM or SIGINT handling. | **Probe.** A process running `launch()` on a sleeping child under `/usr/bin/time` gets SIGTERM after 2 s. It exits with −15, and the wrapper and the child keep running, reparented to PID 1 (`probes/probe_term.out`; RV18 then killed them).<br>**It happened in K6's slot.** At ROOT's B2 pause, the driver exited with 143 while run 100 was in its witness stage. The run was killed separately and voided (`T3/IMPLEMENTATION/K6/_run_records/b2/b2.log`).<br>**What is lost.** Until someone kills it, the orphan has only the heap cap. It has no RSS watchdog, the design's backstop for non-heap growth on a Mac without swap, and no timeout.<br>**The brief's words** (§5, "so no orphan survives") hold for the watchdog and timeout kills, which RV18-M9 and K6-M4 confirm. They do not hold for an interrupted runner. | In `launch()`, wrap the loop in `try/finally`: if the wrapper has not been reaped, `killpg(pgid, SIGKILL)` and `wait4`. In `main()`, map SIGTERM to `SystemExit` so the `finally` runs (SIGINT already raises).<br>Add the probe as a `LiveLimit` test that asserts no survivor.<br>This matters before K6b and V-K reuse the runner under paused slots. |
| RV18-4 | SHOULD-FIX | `T3/IMPLEMENTATION/K6/_run_records/c/logs/py-K6-M5.log:33-34` and `:233-234`. | **The log keeps two kinds of machine path.**<br>– All four lines carry the per-user macOS temporary-directory path (the var-folders form) of the watchdog's `.time.txt`.<br>– Line 34 also carries a pytest-shortened home-directory path to the outer worktree's venv. Line 234's copy was replaced with `<VENV>`, so the scrub missed only these.<br>GEN-8's own `MACHINE_ABS_PATH_RE` flags the four lines (`records_checks.txt`).<br>**GEN-8 itself passes** (`gen8.txt`), so the lint treats the file as historical.<br>**The rule does not.** `_COMMON.md`'s rule ("Never write a machine-specific absolute path … in a committed record") and RETURN.md:13 do not hold for this file.<br>**The rest is clean.** The other 938 files have no machine path, user name, host name or model identifier. The one other regex hit is the synthetic string at `H/runner/test_k6_runner.py:318` (RV18-N2). | Replace the paths with `<tmp>` and `<VENV>`, regenerate `T3/IMPLEMENTATION/K6/SHA256SUMS`, and re-run the scan over the PR's files. |
| RV18-N1 | NOTE | `k6_runner.py:291` (`POLL_S = 0.1`); `test_k6_runner.py:561-576`. | **RV18-M6 (`POLL_S = 1.0`) survives.** The live test's tolerance, "within one poll's growth", grows with the poll.<br>The measured interval is 0.114 s in my `--smoke` run (the 100 ms sleep plus `ps`; `checks/smoke_summary.txt`). The design sets 100 ms (DESIGN.md:826). | Pin `POLL_S` in a test, or assert that the median sample interval is below 0.2 s. |
| RV18-N2 | NOTE | `k6_runner.py:589-594` (`sanitize`), `:864` and `:929-931`; `test_k6_runner.py:317-318`. | **RV18-M7 survives.** `sanitize` without the home-directory prefix passes, because the test covers only the Volumes and var forms.<br>**Linux.** GNU `time -v` writes "Command being timed" with the binary's absolute path into `<run>.time.txt`, which is never sanitized, so a Linux run's records would carry a machine path.<br>**The test string.** The synthetic path at `:318` is a `MACHINE_ABS_PATH_RE` hit in the product tree. GEN-8 passes. | Test a home-directory path and a private/tmp path. Sanitize or drop that line on Linux. Consider building the test string at run time. |
| RV18-N3 | NOTE | `k6_runner.py:736-744` (`host_busy`); `test_k6_runner.py:524-539`. | **RV18-M8 survives:** the quiet-host wait ignores a running DEC-025 sweep. `QuietHost` tests only the cargo check. | Add a case where only the sweep pattern is busy. |
| RV18-N4 | NOTE | `main.rs:632-652`; `k6_runner.py:682-685`. | **The runner trusts the binary's `equal` flags.**<br>– The binary compares the `Debug` length and FNV-1a 64 digest, not the bytes.<br>– `parity_failures` reads only `equal`.<br>– An always-equal mutant cannot be killed without a fault.<br>**The records allow a check anyway.** They carry both digests, and I re-derived staged against checked (300 of 300), plain against checked (300 of 300) and repeat determinism (116 of 116) from them (`checks/digest_check.out`).<br>**Test E itself is complete and strict.** It compares the full `Debug` strings (`k6_staged.rs:30-34`), and `StructuralSolution`'s `Debug` is derived (`FK/structural.rs:315`), so it is bitwise for non-NaN values. | Have the packet recompute each parity from the digests, not from `equal`. |
| RV18-N5 | NOTE | `k6_runner.py:509-541`, the B2-stop rule as implemented. | **Two readings differ from the ruling's text, with no effect on the records.**<br>– The RSS-to-footprint ratio is taken over runs smaller than the candidate, with 1.45 as the dense floor and the default. ROOT's text says "measured in the same family and mode". No admission changes on the records: the dense floor dominated (CHAIN n1000's 1.402 < 1.45), and lane-lu's same-size ratios (1.001) were below its 100-member ones (1.104).<br>– The P1 branch projects from max(P1, E_adm). For run 100 that gave 5.11 GiB (the recorded 5,483,753,635 B), where ROOT's ruling said "about 4.8 GiB". Both are within 6.4 GiB, and the runner's figure is the more conservative. | Record the implementation as ruled, or include same-size measured runs. |
| RV18-N6 | NOTE | `H/src/k6/models.rs:117-119`; `canonical.rs:112`; `main.rs:447`. | **The lane refusal is keyed on the id.** `is_cont_n10000` tests the id prefix, and `--model-file` takes the id from the file, so a renamed CONT n10000 file escapes the by-name lane-id refusal.<br>The estimate refusal (`main.rs:521-524`; 24·P_id is 13.5 GB for AX and 16.2 GB for ROT) and the heap cap still stop it, and the runner never passes `--model-file`. | Optional: decide by family and member count. |
| RV18-N7 | NOTE | RETURN.md:739; CHANGE_RECORD.md:5; the packet's `slot` field. | **Three small record inaccuracies:**<br>(a) The provenance lock copy already differed from `H/Cargo.lock` at main `59cb20073`: it lacks `sparse_direct` (`revisions.txt`). So "now differs … (K6 added three path packages)", and ROOT's D ruling's "no longer matches", overstate K6's part. It is still read by nothing.<br>(b) "From main `d1cc97ce4`": the first K6 commit's parent is `56dd72334` (RV16-S1's correction; the piping tree is the same).<br>(c) Run 137 (grid 128×128) carries `slot` B1, its planned slot, though it ran in B2. Its record carries B2's `runner_commit`, and RETURN §5 counts it correctly. | Correct the wording when the records are next touched. |
| RV18-N8 | NOTE | `test_k6_runner.py:577-582`, `:586-593`; `k6_runner.py:864`. | **The Linux live path has never run.**<br>– On Linux, both live tests run a Python child under `RLIMIT_AS` = 128 MiB with no wrapper. The negative control grows to 64 MiB on top of the interpreter's address space.<br>– `run_tier` on Linux needs GNU `/usr/bin/time`, which many images lack.<br>K6 discloses this (RETURN §8.7, §14). The first Linux sweep that collects the wrapper will be the first live run. | Run the wrapper once on Linux before relying on a Linux sweep. Check for `/usr/bin/time` in `launch`. |

## Reviewer, brief and delegation

- **Reviewer.** RV18 is a Type 2 TASK, dispatched directly by ROOT (HELP_HUMAN) as a background subagent of ROOT's session. ROOT is the only return path.
  - I did not design, implement or test K6. I delegated nothing.
  - I made no Git write and no index operation, and no GitHub write. My Git use in `<wt>/k6` was read-only: `log`, `show`, `diff`, `grep`, `archive`, `merge-base`, `rev-parse` and `status`. `status` (in `<wt>/k6`, and once in `<wt>/numerics`) may refresh the index stat cache, as RV17 disclosed. I ran no `git fetch`.
  - My writes are this file and `T3/REVIEW/_run_records/k6_review/**`, left uncommitted.
- **Read:**
  - Root `AGENTS.md`, `agents/AGENT_TASK.md`, `T3/TASK_BRIEFS/_COMMON.md`, and `I8R_K1_RESUME.md:24-50`;
  - `TASK_BRIEFS/I15_K6_IMPLEMENTATION.md`, with Q1–Q13 and the RV16-N4 addition;
  - `ROOT_RULINGS_V1.md` at the numerics head `d98fb6f58`, every K6 section from "K6: spawn and rulings" to "K6: rulings at C and D";
  - K6's `PLAN_CHECKPOINT0.md`, `RETURN.md` (987 lines), `CHANGE_RECORD.md` and the `_run_records/` I cite;
  - `REVIEW/F1B_REVIEW.md` and `REVIEW/K5_REVIEW.md`, for method;
  - the complete diff of `H` and the wrapper;
  - the code it relies on: SA's sparse entries, `check_pattern`, `symmetry_basis` and `formation_source`; SD's RCM, `from_entries_with_order` and `solve_sparse_prepared`; PP's `assemble_reduced_sparse_entry_system`, `append_reduced_element_entries` and `observation_lane_profile`; FK's `StructuralSolution`;
  - R1's `references.py` (`80d473a7…`) and P1's `gen.py.txt` `y_reference` rule.
- **Abbreviations.**
  - `P/` = `projects/chirality-piping/`; `T3/` = the T3 folder; `H` = `P/core/solver/performance_harness/`.
  - `FK`, `SD`, `SA` and `PP` are as in the brief.
  - Record paths in the table are under `T3/REVIEW/_run_records/k6_review/`.
  - Line numbers are at `ae3320b5a`.
- **Host.**
  - Every build came from `git archive ae3320b5a` copies under `<wt>/rv18`, with the target `<wt>/rv18-target`. Each mutant had its own fresh archive and target, deleted after its run.
  - Builds used rustc 1.97.1, `--offline --locked`, `-j 4` and `RUST_TEST_THREADS=2`, with at most two cargo jobs of my own. I12 and other agents were building, and the 1-minute load ran from 4.4 to 9.5.
  - The memory guard log was unchanged throughout: 2 start lines, sha256 `79e2ce8e…`.
  - **Runs made:**
    - no observation run above 100 members and no dense run at 1,000 or more;
    - the binary ran at 10 and 100 members and on the nine, through `--smoke`;
    - `--counts-only` at 100 members or fewer;
    - `--emit-model` (O(n)) and `--list-models`.

## Revisions reviewed (`revisions.txt`)

- **PR #1053** is open and mergeable at `ae3320b5a`. Its hosted checks show 12 SUCCESS and 4 SKIPPED.
  - The Linux "Numerical cargo suite" job ran `H`'s `cargo test` in 75 s and passed.
  - That includes `k6_counts`' pinned oracle constants and `k6_models`' canonical bytes, so they hold on Linux too.
- **The history.**
  - First parent: `9ababe4f2` (A1, A2), `1f354c20b` (merge of main with K5), `962dd4e3b` (B1), `3799e3764` (B2), `ada18de70` (B3), `014b2ae04` (C), `3e90176c6` (merge of main with F1b), and `ae3320b5a` (D).
  - The branch starts at `56dd72334`, whose piping tree equals `d1cc97ce4`'s.
- **The D commit** changes 24 files: K6's records, `H/README.md` (+1 line), `observations/k6/k6_packet.json` and `observations/k6/SHA256SUMS`.
- **B's binary.** Its Rust source (`H/src/**`, FK, SD and NI's library) is unchanged since `1f354c20b`, from which it was built. Only NI's `#[cfg(test)]` `s11k_tests.rs` changed in the solver tree since then.
- **The runner.** The B2 and B3 records came from the head's `k6_runner.py` (sha256 `737eff78…` at `3799e3764`, `ada18de70`, `014b2ae04` and `ae3320b5a`). B1 used `962dd4e3b`'s runner.

## 1. Scope 8: no product byte changes

- **The path scan** (`git diff --name-status 59cb20073 ae3320b5a`) gives 935 added and 4 modified files.
  - The modified files are `H/Cargo.lock`, `H/Cargo.toml`, `H/README.md` and `H/src/lib.rs`.
  - Outside `H/` and `T3/IMPLEMENTATION/K6/`, only `P/tests/test_performance_harness_runner.py` is added.
  - No file under FK, SD, NI, `curved_bend`, `straight_pipe`, PP, `P/core/runner`, `P/validation`, `P/provenance`, `.github`, `tools/` or `P/tools` changes. Nor does `H/examples/sparse_default_promotion_observation.rs`.
- **`H/src/lib.rs`**: the diff is exactly one added line, `pub mod k6;`, after the `use` block. The legacy functions, constants and tests in that file are byte-identical.
- **The lock** gains `open_pipe_stress_curved_bend`, `open_pipe_stress_nonlinear_integration` and `open_pipe_stress_nonlinear_supports`, and the harness's own dependency on NI. It has no `source` or `checksum` line.
- **The dependency closure.**
  - No `Cargo.toml` outside `H` names the harness.
  - Outside `H` and the execution records, the harness is named only in docs, plans, the tranche manifest's path list, the DEC-050 JSON, recorded coverage evidence, the provenance lock copy and the three `P/tests` files.
  - No code under `.py`, `.rs`, `.toml`, `.yml`, `.js` or `.sh` outside execution records reads `provenance/build-artifacts`. The copy is inert (see RV18-N7(a) on its staleness).
- **The pins.** The two DEC-050/053 pytest pins pass from my archive, beside the wrapper: 37 passed in 6.6 s (`suites/pytest_head.log`).
- **So no binary that T9 or the both-entry gate builds can change.** I agree with RETURN §9.3. I ran neither.

## 2. The observation binary

### 2.1 Staged against SA's entry (test E)

- **The staged sequence** (`H/src/k6/staged.rs:195-475`) replicates SA's `solve_assembled_with_formation_check`, and its `!selected` sibling, at public-API boundaries, as SA runs them:
  - the pattern check, `geometry` and the frame-family basis;
  - `formation_source` for frames only, and `prepare_formation_checked_*` or `prepare_assembled_*`;
  - `order_sparse_structural` with `factor_sparse_structural_profile` (sparse), or `factor_structural_cholesky` (dense);
  - the pair witness on a refusal, and `finish_*`.
- **Two differences** can matter only for other models:
  - the staged checked path uses the public `geometry`, not K5's `constrained_geometry`;
  - it omits `unscaled_evidence`, which never refuses `new`'s evidence.
  - For frame-only models both are the same, and N6's re-runs of E after each merge confirm it.
- **E compares the whole result.** `k6_staged.rs:30-34` compares the full `Debug` of `Result<StructuralSolution, StructuralError>`. The `Debug` is derived, so it covers the displacements, the report, `load_fidelity` and `formation_check`. f64 `Debug` round-trips, so equality is bitwise for non-NaN values. E is complete and strict.
- **E is shared-input by design.** `entry()` (`staged.rs:511-548`) reuses the staged `k`, evidence, force and `free`. So E pins geometry through finish, not assembly, evidence or reduction, which are the product's own calls and inputs to both paths.
- **The binary's lines** compare digests (RV18-N4). I re-derived them from the records: 300 of 300 staged against checked, over 64 runs, and 300 of 300 plain against checked.

### 2.2 The allocator (`H/src/bin/k6_observe/alloc.rs`)

- **`reserve` (`:89-102`)** adds the request to `CURRENT` only if the result is at most `CAP` (`next > cap` refuses). So exactly at the cap succeeds and one byte over fails.
- **`alloc` and `alloc_zeroed` (`:169-199`)** reserve before calling `System`. They release if `System` returns null, and note both peaks only on success.
- **`dealloc`** releases `layout.size()`.
- **A growing `realloc` (`:208-222`)** reserves the difference (the in-place model, which carries the cap). The move model notes `level + old`, which is exactly the old and new blocks alive together. On failure it releases the difference and returns null, with the old block still valid.
- **A shrinking `realloc`** releases the difference. By N8's definition it adds no move peak, though a shrink that moves also holds both blocks briefly.
- **The abort marker** (`:142-163`) is formatted into a stack buffer and written to fd 2 through a forgotten `File`, without allocating. The null return then makes Rust's allocation-error path abort with SIGABRT.
- **Coverage.** F1, F2 and F4 cover this, and the smoke run's heap-cap abort was classified `heap_cap_abort` with exit −6.
- **I found no accounting defect.** RV18-2 concerns the binary's use of the stage peaks, not the allocator.

### 2.3 Refusals

- **In the binary** (`main.rs:440-451`), before frames or counts:
  - `Mode::materializes_n2()` (`H/src/k6/mod.rs:55-57`, Dense and LaneLu) at `members >= 10_000` gives `n2_mode_at_or_above_10000_members`;
  - lane-id on CONT n10000 gives `cont_n10000_identity_lane`;
  - the estimate refusal follows at `:521-524`.
- **Coverage.** G1 covers both n² modes at 10,000 members, with a heap before refusal below 64 MiB, far under 8·n². G3 covers both CONT orientations.
- **My re-run of K6-M12** (dense-only refusal) is killed at `k6_bin.rs:130`.
- **In the runner, independently** (`k6_runner.py:415-421`): 14 rows are "never" in the plan and in the records.

### 2.4 Counts (`checks/counts_oracle.out`, `records_check.out`, `cont_identity_quadratic.txt`)

- **My own port.** I wrote my own port of SD's `reverse_cuthill_mckee` and of `from_entries_with_order`'s first-column rule, reading SD, not K6's oracle. I ran it on the prepared free block's nonzero positions from `--counts-only --dump-pattern`, and on the lane's positions.
  - It matches `counts.jsonl` on all 21 models at 100 members or fewer and the nine: RCM profile and half-bandwidth, identity profile and half-bandwidth, nonzero and lane counts.
  - It also matches my closed forms, computed from the canonical model alone: pattern, lower, free, free-lower, contributions and dense.
- **Above 100 members,** the binary's parity lines carry the product's own numbers. `ordering_profile_entries` and `ordering_half_bandwidth` equal `counts.jsonl` on all 190 lines (38 sparse runs, 10 to 10,000 members and the grids). SD's `original_profile_entry_count` equals it on all 31 lane-id runs.
- **CONT n10000's never-run identity profile** follows exactly from the runtime-verified sizes. The quadratic through s = 5, 50 and 500 predicts K6's 562,627,485 (AX) and 675,174,982 (ROT). F1b's full-block form, 675,179,982, is the ROT value plus s.

### 2.5 Generators (`checks/crosscheck.out`)

- **My own check against R1.** I imported `references.py` (`80d473a7…`), built its RF-LARGE family, and compared `model_json(defn, full=True)` with the canonical bytes the release binary emits. The comparison covered:
  - node order, labels and coordinate bits;
  - member order, labels and ends;
  - `y_reference` (P1's rule, recomputed from the exact coordinates);
  - E and G bits, and A, I and J against the stated formula;
  - restraints;
  - every load, rounded once and exact, with no extra load.
  - Result: **24 of 24 equal**, in 50 s.
- **Hashes.** The emitted bytes of all 39 models, and the Python generator's, equal `models_sha256.txt`.
- **The section.** A is 1.2e-15 relative from the exact value, which is K6's disclosed 8–9 ulp (N20).
- **Precision.** No function of unspecified precision appears in `H/src/k6/` or the binary.
- **The lane copy.** The lane builder copy (`H/src/k6/lanes.rs`) matches PP's builder, its entry appender and its `observation_lane_profile` for frames and restraints.

## 3. The runner (`H/runner/k6_runner.py`)

- **The process tree.**
  - `Popen(..., start_new_session=True)` makes the wrapper the group leader, with `pgid = proc.pid` (`:876-878`).
  - `find_child` finds the binary with `pgrep -P` (`:821-832`), and each loop turn polls `ps -o rss=` on the binary, not on `time` (`:900-911`).
  - The loop sleeps 100 ms (`:917`), and `ps` adds about 14 ms.
  - The watchdog (`:907-911`) and the timeout (`:912-916`) both `killpg(pgid, SIGKILL)` and reap the wrapper. My RV18-M9 (timeout kills the wrapper only) and the re-run of K6-M4 are both killed by `test_kills_take_the_whole_group`.
  - My `--smoke`: killed at 138,640 KiB against 131,072 KiB, within one poll's growth (8,384 KiB), with no survivor.
- **Limits.** `RLIMIT_AS` applies only for `system == 'Linux'` (`:801-810`), tested with a mocked module. The RSS watchdog kills only on Darwin.
- **Units.**
  - `time -l`: maximum RSS and peak footprint in bytes.
  - `time -v`: KiB × 1024.
  - `ru_maxrss`: bytes on Darwin, KiB on Linux.
  - `ps`: KiB, compared as `kib * 1024 > cap`.
  - All are right, and all are tested.
- **Admission** (`:478-541`), against ROOT's rulings:
  - the by-name refusals come first;
  - the ascent needs the previous size recorded;
  - ρ is taken on the footprint net of the no-op baseline, with the move-model heap as a floor, from the same family and mode at smaller sizes, and at 100 members or more for runs at 1,000 or more; it is 2 where nothing is measured;
  - the RSS-based ρ is recorded beside it;
  - P1's branch is P1's Linux peak ≤ C/2;
  - projected RSS must be ≤ 0.8·C, with a dense floor of 1.45;
  - the load-wait is above 8, until below 6, and `memorystatus_level` must be ≥ 80.
  - This is as ruled, with RV18-N5's two readings.
- **Resume** (`:1076-1080`) skips a run already measured and re-runs a `not_run` record (K6-M26's test). A run killed with its runner leaves no record, and is re-run (run 100's void).
- **Scrubbing.** `stderr` and its tail are sanitized, and `argv` keeps basenames of absolute paths. See RV18-N2.

## 4. Records

- **Checksums.**
  - `T3/IMPLEMENTATION/K6/SHA256SUMS`: 891 of 891 OK, set-equal with the folder.
  - `H/observations/k6/SHA256SUMS`: 24 of 24 OK, set-equal.
- **Regeneration.** Following K6's `packet_reproduction.txt`, the combined `records.jsonl` has 166 lines and sha256 `fcc89445…`.
  - `--packet` gives `b5419aee…`, byte-identical to the committed packet.
  - `d_tables.py.txt`, `gen_return_tables.py.txt` and `sparse_and_timing.py.txt` reproduce all 9 tables, `return_tables.txt` and `sparse_and_timing.txt` byte for byte.
  - Every raw JSONL's sha256 equals its record's `stdout_sha256`.
- **RETURN against my own computation from the raw records** (`checks/records_check.out`; my own least squares):
  - **Fits:**
    - heap, all sizes: CHAIN, CONT and TREE sparse 1.000, 1.002 and 0.998;
    - heap at 100 members or more: 0.999, 1.018 and 0.993;
    - dense 1.976, 1.975 and 1.974; lane-id CONT 1.737; GRID 1.028;
    - RSS net at 100 members or more: 1.017, 0.910 and 1.023.
    - All equal RETURN §7.1.
  - **F1b's dense ratio:**
    - at 1,000 members, heap 0.9687–1.0063 × 96·n², footprint 1.0115–1.0351, RSS 1.0332–1.4482;
    - at the ceiling, heap 1.0032 (1.0027 × 6 GiB), footprint 1.0078, RSS 1.1010, and 0.585 of the projection.
    - Lane-id CONT: 0.459–1.068 (heap) and 0.661–1.332 (move); CHAIN and TREE 4.534–8.998, at most 107.1 MiB (`checks/lane_gap_ceiling.txt`).
    - All equal §7.2.
  - **The product gap:** dense at 1,000 members has heap/P1 0.919–0.948 and footprint/P1 0.945–0.990; sparse at 1,000 has heap/P1 0.007–0.009. Every row of §8.2 matches (`checks/lane_gap_ceiling.txt`).
  - **The RSS observation:** RSS/footprint at dense 1,000 is 1.041–1.402, and 1.093 at the ceiling; the loads are as §8.3 states.
  - **N10:** factor refused at `global_dof` 6001 after 86.114 s, and at 6002 after 65.167 s. The completed stages total 87.3 and 66.3 s, so the witness ran about 1,712.7 and 1,733.7 s. `ps` peaked at 3,495,344 and 3,494,048 KiB. All equal §8.4.
  - **The ceiling run:** 5 repeats, with repeat 0 at 548.6 s of stage time.
  - **The schedule as run:**
    - 124 measured (122 ok, 2 timed out) and 14 never;
    - grid 128×128 deferred in B1 and run in B2;
    - B2's RSS at 0.609–0.969 of its projection;
    - no watchdog or heap-cap event;
    - every measured row admitted, with the previous size recorded first, estimate × ρ ≤ C/2 or P1 ≤ C/2, and projected RSS ≤ 0.8·C.
  - **Parity:** bitwise K 25 of 25, `rcm_count_equals_ordering` 190 of 190, lane profile 155 of 155, repeat determinism 116 of 116.
- **The B2-stop ruling is applied as ruled.**
  - Run 097 was admitted under Q3 as written: P1 3,605.2 MiB ≤ C/2.
  - From 098 onward, the recorded admissions carry ρ on the footprint (098: 1.347, against 1.387 on RSS), the RSS-to-footprint ratio, and projected RSS ≤ 0.8·C (100: 5,483,753,635 B with 1.45; 137: 1.219; 138: 12,120,007,115 B ≤ 13,743,895,347 B).
- **Paths and identifiers.** No user name, host name, hardware model identifier or model identifier appears in the PR's 939 files (`records_checks.txt`). The only machine paths are RV18-4's.
- **GEN-8** passes at the head in `<wt>/k6`: 1 passed, and the working tree was clean before and after (`gen8.txt`).

## 5. The merges

- **`1f354c20b`** (main `1cdeae2c1`, with K5): the merge base is `56dd72334`. `git diff 56dd72334 1cdeae2c1` equals `git diff 9ababe4f2 1f354c20b` (702 files), and the remerge diff is empty.
- **`3e90176c6`** (main `59cb20073`, with F1b): the merge base is `1cdeae2c1`. The same equality holds over 485 files, and the remerge diff is empty.
- **Neither merge touches a file outside K6's write set.**

## 6. Suites and runs (`suites/`, `mutations/`)

- **H's suite.** `cargo test --offline --locked` from the archive, cold, passes in 70.7 s: lib 25, `k6_alloc` F1 and F2, `k6_bin` 8, `k6_counts` 4, `k6_models` 7, `k6_parity` 4 and `k6_staged` 3.
- **Builds.** The non-test build (`--lib --bins`, after a clean of the package) has no warnings. The release build has none either.
- **Pytest.** The wrapper and the two pins pass 37 of 37 in 6.6 s, under the 10 s condition.
- **`--smoke`** (release):
  - 44 processes: 42 ok, 1 `killed_by_rss_watchdog` and 1 `heap_cap_abort`;
  - staged against checked 84 of 84, plain 84 of 84, bitwise K 21 of 21, RCM 42 of 42.
- **Mutants.**
  - Controls: the NONE (Rust) and NONE-py controls pass.
  - Killed: K6-M4, K6-M12 and K6-M16 (re-runs) and RV18-M9, each at the site K6 names or the group-kill test.
  - Surviving: RV18-M1, M2, M3 and M4 (RV18-1, RV18-2) and RV18-M6, M7 and M8 (RV18-N1 to N3).

## 7. What I did not do

- No observation run above 100 members, no dense run at 1,000 members or more, and no ceiling run.
- No Linux or Windows run.
- No T9 and no both-entry gate (Scope 8 holds by construction), and no DEC-025 sweep.
- I re-ran 3 of K6's 26 mutants, not the whole table.
- I did not audit K6's own oracle (`k6_oracle.py`). I wrote my own instead.

## Delta check at cd325c1fe

- **Head:** `cd325c1fe` (`cd325c1fe8e56e536ef2a7503a3ab3f97b1ed01a`), one commit after `ae3320b5a`, with I15's fixes and RETURN addendum 1.
- **Date:** 2026-09-29.
- **Basis:** ROOT's rulings, "K6: rulings on RV18's review" (numerics `21776d312`).
- **Records:** `REVIEW/_run_records/k6_review/delta/`. `k6_review/SHA256SUMS` is rewritten over all 74 files, and the base review's 43 entries are unchanged.
- **Verdict: PASS.**
  - No BLOCKING or SHOULD-FIX finding remains.
  - All four SHOULD-FIX findings and NOTEs N1–N3, N6 and N7 are fixed. N4, N5 and N8 are recorded in RETURN addendum 1, as ruled.
  - The delta adds 3 NOTEs (D-N1 to D-N3), none of which needs a change before merge.

### D.1 What changed (`delta/revisions.txt`)

- **The delta is 214 files:** 206 added, all under `T3/IMPLEMENTATION/K6/_run_records/rv18/`, and 8 modified.
  - The modified code files are `H/runner/k6_runner.py`, `H/runner/test_k6_runner.py`, `H/src/k6/models.rs` and `H/tests/k6_bin.rs`.
  - The modified records are CHANGE_RECORD, RETURN, K6's `SHA256SUMS` and `_run_records/c/logs/py-K6-M5.log`.
- **Nothing else moves.** Nothing changes outside `H/` and `T3/IMPLEMENTATION/K6/`. `H/src/lib.rs`, `Cargo.toml`, `Cargo.lock`, `src/bin/` and `observations/` are unchanged, and so are B's records (`b1`, `b2`, `b3` and `d`). `git diff --check` is clean.
- **Scope 8 still holds.** The delta changes only H's runner, H's tests and one predicate in H's library, and no crate depends on H.
- **The runner's poll loop moved into `_watch` verbatim** (24 identical lines), so every earlier runner mutant applies unchanged.
- **Hosted checks at `cd325c1fe`:** 12 SUCCESS and 4 SKIPPED. The Linux "Numerical cargo suite" passes, so the new `k6_bin` tests hold on Linux.

### D.2 The re-run (`delta/mutations/`, `delta/suites/`)

- **Controls.** The Rust NONE control passes, from a fresh archive and target, in 83 s cold: lib 25, `k6_alloc` F1 and F2, `k6_bin` 11, `k6_counts` 4, `k6_models` 7, `k6_parity` 4 and `k6_staged` 3. The runner NONE control passes 39 tests.
- **13 of 13 mutants are killed, each at its intended new test.**
  - RV18-M1 and RV18-M2: `first_repeat_and_time_budget_stops`. M1 fails at the first run. M2 fails only at the fourth run: all 1,000 repeats ran, with a null reason. That confirms I15's point that the three runs I proposed cannot tell M2 apart, since 0 × 10^6 = 0.
  - RV18-M3 and RV18-M4: `summary_peak_is_the_largest_stage_peak_and_stage_peaks_restart`. M3 fails with 145,351 against 579,691; M4 with repeat 1's assembly at 579,691 against repeat 0's prepare at 562,391.
  - RV18-M6: `test_poll_interval_is_the_designs_100_ms`.
  - RV18-M7: the extended `test_sanitize`.
  - RV18-M8: `test_a_running_sweep_alone_makes_the_host_busy`.
  - K6-M4 and RV18-M9, re-applied inside `_watch`: `test_kills_take_the_whole_group`.
  - I15's K6-M27 (no kill in the `finally`) and K6-M28 (SIGTERM not mapped): `test_a_terminated_runner_leaves_no_survivor`.
  - I15's K6-M29 (the refusal keyed on the id again): `renamed_cont_n10000_lane_id_refused`.
  - I15's K6-M30 (the `time -v` file not scrubbed): `test_time_v_output_file_is_scrubbed`.
  - No helper process survived any run.
- **The wrapper and the two DEC-050/053 pins** pass 41 of 41, in 6.6 s.
- **`--smoke`** with the release binary:
  - 44 processes: 42 ok, 1 `killed_by_rss_watchdog` (134,368 KiB against 131,072 KiB, 15 polls, a 0.113 s median interval, no survivor) and 1 `heap_cap_abort`;
  - staged against checked 84 of 84, plain against checked 84 of 84, bitwise K 21 of 21, RCM 42 of 42.

### D.3 The SIGTERM handling (`delta/probes/probe_signals.out`)

- **The design.**
  - `sigterm_raises_exit()` maps SIGTERM to `SystemExit(143)` only on the main thread, and only where the previous handler is `SIG_DFL`. It returns a function that restores the previous handler.
  - `launch` installs the mapping before `Popen` and runs the loop in `try`. The `finally` calls `stop_group` (SIGKILL to the group, then reap the wrapper) when `_watch` did not return, and then restores the handler. SIGINT already raises `KeyboardInterrupt`, which takes the same `finally`.
- **Probes, on the head's runner:**
  - SIGTERM to a process running `launch()` on a tagged sleeper under `/usr/bin/time`: exit 143, with 2 tagged processes before and 0 after. At `ae3320b5a` the same probe left both running.
  - SIGINT: exit −2, after the `KeyboardInterrupt` traceback through `_watch`, with 0 tagged processes after.
  - On the main thread, the handler is `SIG_DFL` before, a Python handler during (read inside `_watch`), and `SIG_DFL` again after: it is restored.
  - A caller's own SIGTERM handler, installed first, is left in place during `launch` and is still in place after it.
  - `launch` on a worker thread completes (`ok`) and changes no handler.
- **The handling is sound for the runner's own `--run`, `--smoke` and the B drivers,** which all call `launch` from the main thread with the default handler. Its limits are D-N1.

### D.4 The N6 predicate (`delta/probes/probe_n6.out`, `probe_n6_method.txt`)

- **The new predicate.** `is_cont_n10000` (`H/src/k6/models.rs:126-138`) is true when the member count is at least 10,000, and either:
  - the family is CONT; the family comes from the id's prefix, as `canonical::family_of` reads it;
  - or the model carries CONT's restraint signature: m + 1 nodes, node 0 fully fixed, nodes 1 to m/2 pinned in UX, UY and UZ, and nothing else.
- **Its one call site** is the binary's refusal (`main.rs:447`).
- **Unchanged on the 39 committed models.** CONT n10000 AX and ROT are true, as before. CHAIN and TREE n10000 have one restrained node, and grids 96×96 and 128×128 do not have m + 1 nodes, so all four stay false.
- **Probes,** each through `--model-file`, lane-id, under a 512 MiB heap cap:
  - A, R1's CONT-n10000-AX renamed: refused by name, before any count;
  - B, the same content under a CHAIN id: refused by name, before any count (the signature holds);
  - C, renamed with node 0's RZ freed, and D, renamed with the last support pinned in four DOFs: **not refused by name**. Both are refused by `estimate_exceeds_half_cap` after the O(nnz) counts phase (157.7 MB of heap), before any lane entry.
- **So no route admits a CONT-like n10000 lane-id run.**
  - R1's model is refused by name under any id or family label.
  - A CONT-shaped variant outside the signature is refused by the estimate, whose 24·P_id alone is about 13.5–16.2 GB. The heap cap bounds anything beyond that.
  - The runner's own refusal still keys on its scheduled ids, which are R1's (`k6_runner.py:415-421`).

### D.5 Records (`delta/records_checks.txt`, `delta/gen8.txt`)

- **Checksums.** `T3/IMPLEMENTATION/K6/SHA256SUMS` lists 1,097 files: all OK and set-equal. Against `ae3320b5a`, it adds 206 entries and re-hashes 3: CHANGE_RECORD, RETURN and the M5 log. `H/observations/k6/SHA256SUMS`: 24 of 24 OK, and the folder is unchanged.
- **RV18-4 is fixed.** `py-K6-M5.log:33-34` and `:233-234` now carry `<tmp>` and `<VENV>`.
- **Paths and identifiers.** GEN-8's `MACHINE_ABS_PATH_RE` finds 0 hits in the PR's 1,145 files, which confirms I15's figure; the user name is also absent. There are no model, hardware or host identifiers.
- **GEN-8 passes** at the head in `<wt>/k6`: 1 passed, and the working tree was clean before and after.
- **RETURN addendum 1 and CHANGE_RECORD** match what I re-ran. N7's three points are corrected or noted as ruled: the base `56dd72334`, run 137's `slot`, and the provenance copy already stale on main.
- **The memory guard log** is unchanged: 2 start lines, sha256 `79e2ce8e…`.

### D.6 New NOTEs

| ID | Class | Site | Evidence | Fix |
|---|---|---|---|---|
| D-N1 | NOTE | `H/runner/k6_runner.py`, `sigterm_raises_exit` and `launch` | **The mapping has three narrow limits:**<br>(a) under a caller's own SIGTERM handler, protection depends on that handler raising; this is documented in the docstring, and the probe shows the handler kept;<br>(b) `launch` on a worker thread gets no mapping;<br>(c) a SIGTERM that lands inside `Popen` after the fork, before `proc` is assigned, or a SIGHUP (default action), can still orphan the group.<br>None applies to the runner's `--run`, `--smoke` or the B drivers, and (c)'s window is the exec of `/usr/bin/time`. | None needed. Optionally, map SIGHUP as SIGTERM is mapped. |
| D-N2 | NOTE | `H/src/k6/models.rs:126-138` | **CONT-shaped variants outside CONT's restraint signature escape the by-name refusal** (probes C and D). The estimate refusal stops them after the counts phase. That refusal relies on honest counts: a hand-edited `--counts-file` for the variant would pass it, and the heap cap would then abort the lane. | None needed. The by-name refusal is belt and braces, as ruled. |
| D-N3 | NOTE | `T3/IMPLEMENTATION/K6/RETURN.md`, addendum 1 | **Two small inaccuracies in the addendum:**<br>– it says the fixes are "uncommitted in `<wt>/k6`, for ROOT's commit"; they are committed at `cd325c1fe`;<br>– the stop test's fourth run makes it about 1.1 s, not "under 1 s" (`k6_bin` totals 1.85 s here). The addendum discloses the timing. | Update the wording when the records are next touched. |

### D.7 What I did not do in the delta

- No observation run above 100 members, no dense run at 1,000 members or more, and no Linux run. The Linux side of the new SIGTERM test has not run live, which extends N8.
- No T9, no both-entry gate and no DEC-025 sweep.
- My copies and targets under `<wt>/rv18` and `<wt>/rv18-target` are deleted. I made no Git write and no GitHub write.
