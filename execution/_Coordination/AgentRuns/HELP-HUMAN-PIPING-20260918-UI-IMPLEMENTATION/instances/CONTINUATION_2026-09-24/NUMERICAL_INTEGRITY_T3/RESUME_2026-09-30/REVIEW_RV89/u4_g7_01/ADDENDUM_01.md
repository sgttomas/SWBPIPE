# RV89 G7, addendum 01: the hardened Pass B and its run on the final basis

**Reviewer:** RV89, TASK (Type 2), dispatched directly by ROOT. No descendants. This follows this folder's REVIEW.md (Pass A: PASS, 0/1/3).

**Candidate:**
- I65's Pass B repair and its final run, in `R/I65/u4_g7_02/`: RETURN.md (`637331ec…`), `_run_records/` (the `g7_pass.sh`, `pass_checks.py`, `delta_inventory2.py`, `premise_pins.json` and `delta_reviewed.json` it uses), and `runs/`.
- **The final basis:** memory branch `7f07a2f7b413b37ecaa879f81b3d759a9cde7f13`.
  - Against `ba1faa1c`, it differs in exactly six PP files: `lib.rs`, `retained_product.rs`, `retained_wire_tests.rs`, `retained_tests_hooks/grant2.rs`, `retained_facade_tests.rs` and `retained_memory.rs` (the T17_V4 line).
  - My copy, WT/rv89_g7/base, was brought to it with `git archive` of those six files. Its projects/chirality-piping equals the tree: 2,950 of 2,950 blobs, with no extra file.

**Oracles:** RV89's own.
- My registered build of the final basis: law tests, all nine witnesses, the challenge, PP, runner/headless and my sweep.
- A counting-allocator probe on grant 2's hooks.
- `nm` on the built binaries.
- Pass B's gates, run by me on my own logs, copies and mutations (`evidence/addendum_01/`).

## Verdict: **PASS**. The final basis is re-qualified, and the six added tests are the expected delta

| | Count |
|---|---|
| BLOCKING | 0 |
| SHOULD-FIX | 0 |
| NOTE | 2 (N-4, N-5: Pass B hygiene for later reruns; neither affects this run) |

**1. S-1(a)–(e), RV87 SF-1 and RV87 N-1 are implemented as ruled, and the stops fire.**
- **(a)** `delta_inventory2.py` classifies every hunk. It stops with **5** on a `live`, `item` or `cfg-test-stmt` hunk that has no fingerprinted reviewed entry. A deletion inside a function falls to `item`, which is conservative.
- **(b)** There is one `VERDICT` line. A stop gives 2–5, and any other non-zero gate gives 6, including a gate that crashes.
- **(c)** The `REGISTERED_PROFILES` block is compared byte for byte with `0c7827b6ad`'s, threshold included. The law gate needs equal identity, inputs and layouts, `0 failed`, and all 7 registered tests run.
- **(d)** D is read from the run (`TB_D=$D`, `TB_D_RUN`).
- **(e)** The three premise lines (retained_precision.rs:4252, :4253, :4305) are rule keys for the line map (exit 4) and are checked by the premise gate (exit 4).
- **RV87 SF-1:** the §11 set must be exactly the reviewed 410, and an incomplete TEXT run (including `scc`) is gate 6.
- **RV87 N-1:** a self-recursive text ancestor outside `loop_bounds.recursion_reviewed` (the reviewed 11) makes the run incomplete.

**My spot-checks**, run with the gates' own code on my inputs (`gate_spot_checks.txt`):

| Gate | Passing case | Stops |
|---|---|---|
| entry | my final copy → 0 | threshold doubled, identity edited, layout edited → 3 each |
| law | my final registered log → 0 | my real RUSTFLAGS Stale log, an empty log, a registered test FAILED, a registered test absent → 3 each |
| premise | final → 0 | projection keeps the successor id, `validate` dispatches the unprojected source → 4 each |
| forms | final → 0 | Pass A's block without the line → 6 |
| statics | my copy → 0 | |
| outcomes | `ba1faa1c` PP → 0 | final PP → 6 (the 6 added tests) |
| text | I65's final outputs → 0 | |
| delta | final with I65's reviewed table → 0 | final with an empty table → 5 (exactly the 3 statements); the U6 delta `0c7827b6ad..ba1faa1c` with an empty table → 5 (F5 and 4 other live hunks, plus 3 items) |

I65's own 33 controls (`runs/pass_b_controls.out.txt`) agree.

**Can any delta still exit 0?**
- **By design, yes.** A production hunk that the lexical graph classes `unreachable` (not on the D1 graph, or reached only through `edge_zero`), a `not-d1` file, or a `no-code` hunk exits 0 when every output and outcome is unchanged. That is Pass A's and G4's standard, and each hunk is listed in `delta_inventory.json`.
- **Two paths should be closed** before Pass B is relied on again: N-4 (stale outputs on a reused tag) and N-5 (qualification tests edited with unchanged outcomes). **Neither occurred in this run.**
  - The final run's outputs were all written in the run itself (10:25–10:30 MDT), and its TEXT chain exited 0.
  - The law, witness and challenge test files are unchanged at `7f07a2f7b4`.

**2. The three `delta_reviewed.json` entries are confirmed.**
- **The statements:**
  - lib.rs:2379, `ordinary_run_entered()`;
  - lib.rs:3005, `at_complete_gate(&mut observer)`;
  - retained_product.rs:3244, `before_late_gate(&*self)`.
- **They are absent from every non-test build, at compile time.** Each statement carries `#[cfg(test)]`, and so does the module it calls: `retained_tests_hooks` (lib.rs:3174), with `grant2` declared only inside it. A non-test build could not even name the function.
  - `nm` shows **no** hook symbol in the challenge binary (`retained_memory_challenge-*`, an integration test linking the non-test lib) or in the non-test rlib.
  - The lib test binary carries them: 316 `retained_tests_hooks` symbols.
- **They allocate nothing in the lib test binary,** whether unarmed or armed, by reading and by measurement.
  - **By reading** (grant2.rs):
    - `TALLY` is a const-initialized `RefCell<Option<Arc<Tally>>>`. Its clone is `None`, or a reference-count increment.
    - `ARMED` is a `Cell<Armed>` holding a `Copy` struct, so `consume` is a get and a set.
    - Armed, the hooks only assign in place: `adapter.counts`, which is a `Cell` of an array; `facts[0].diameter = 0.0`; and the test-only field `trace_fault = Some(Copy enum)`.
    - There is no `format!`, string or collection, so no text either.
  - **By measurement** (`rv89_g7f_alloc_probe.rs`): a counting global allocator counted this thread's allocations around each call, on a fresh thread, so each thread-local was first touched inside the count. It recorded **0 allocations** for:
    - `ordinary_run_entered`, on first touch of `TALLY` and again;
    - `before_late_gate`, unarmed (first touch of `ARMED`) and armed;
    - `at_complete_gate`, unarmed (twice), with the complete-gate fault armed, and with the candidate fault armed;
    - both hooks under a live `counted` tally.
  - **The positive control** (`Box::new(u64)`) counts 1 allocation of 8 B.
  - The preparation-armed case (`facts[0].diameter = 0.0`) was read, not run, because a probe capture has no facts.
- **No production type changes.** `Armed` and `Carried` are test-only, and my printed record (identity, inputs, layouts, 244 atoms, all phases) equals `ba1faa1c`'s. The witnesses run in that lib test binary with their outcomes unchanged.

**3. The final run's exit-6 delta is exactly the six added tests, all passing, and the registered entry is unchanged byte for byte.**
- **My own run of the final basis** (`final_basis_run.txt`):
  - PP gives 705 passed, 1 failed (t13), 10 ignored. Against my `ba1faa1c` registered run, the outcome lists differ by **exactly six added `ok` lines**: `u3g2_direct_entry_publishes_the_pinned_successor`, `…_w1_fallbacks_append_one_notice`, `…_no_w1_refusals_keep_exact_bytes`, `u3g2_late_gate_refusal_is_final_and_g_c_is_not_consulted`, `u3g2_no_permit_path_runs_once_without_a_copy` and `u3g2_d_u6_5_carrier_fixtures_are_the_live_successors`.
  - runner/headless (85 passed, 2 failed), the law outcomes (42 passed, 0 failed) and the printed record are identical to `ba1faa1c`'s. The maxima are 0.8881 / 0.8929 M.
  - All nine witnesses' and the challenge's output lines are identical, with peaks of 3,541,898 / 2,252,863 B.
  - **My sweep of the final basis is byte-identical to the G6R registered sweep** (sha256 `25cce1e14090048263ef285cb1a571c41ccc34955e57de0fb388df598940ccbb`), so grant 2 changes no published byte on my inputs.
- **The entry:** the entry gate on my final copy against `git show 0c7827b6ad` gives equal, with threshold `4_026_531_840`.
- **I65's run:** `runs/final/VERDICT.txt` is `DELTAS TO READ exit=6`, and its only non-zero gate is `pp_outcomes:6`. My re-evaluation of its TEXT outputs against Pass A gives 0, with D 14,734.
- **`price_delta.py` double count.** On the final tree, `price_delta.py` adds F5 to a T17_V4 form that already includes it. It is informational, not a gate, and overstates V4 by 205,960 B, which is still 1.13 GB below V2_hash.

## Findings (addendum)

| ID | Sev | Where | Evidence | Remedy |
|---|---|---|---|---|
| N-4 | NOTE | `u4_g7_02/_run_records/g7_pass.sh:20`, :67, :77, :80 | **A reused tag can pass on stale outputs.**<br>– `pass_<tag>/` is not cleared; only `work/` and `verdict.tsv` are.<br>– The TEXT chain's exit code (:67), the §11 run (:77) and the controls run (:80) are not gates.<br>– So on a rerun under the same tag, a TEXT chain that fails before `sens.py` leaves the previous `sens_pb.summary.json`, `sens_pb/` and `edges_pb.json` in place, and the `text`, `delta`, `noncand` and `controls` gates would read them.<br>– A fresh tag is safe: a missing summary is gate 6. The final run used fresh outputs | Clear `pass_<tag>/`, except the new `logs/`, at the start. Record the exit codes of `run_text_part2.sh`, the §11 `text_budget.py` run and `audit_controls_g7.py` as gates, with non-zero → 6 |
| N-5 | NOTE | `delta_inventory2.py:129–132`, :153–154 (class `test`) | **Test-file hunks never stop, including the qualification's own evidence.** `retained_memory_law_tests.rs` (`PINNED_RECORD`, the registered tests), `retained_memory_witness_tests.rs` (stack sizes, asserted outcomes) and `tests/retained_memory_challenge.rs` can be weakened while their outcomes stay `ok`, and Pass B would exit 0. None of them changes at `7f07a2f7b4` | Treat hunks in those three files as needing a reviewed entry (exit 5), or as deltas to read (6) |

## Execution record

- **Who.** RV89, TASK (Type 2) under ROOT. No descendants.
- **When.** 2026-10-04, about 10:33–10:45 MDT, within the 1.5-hour box.
- **Memory guard.** `memguard.sh` PID 5387 was running, and every cargo job checked it.
- **Cargo.** The default toolchain, `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2` (1 for the witnesses, the challenge and the probe), and `TMPDIR` in scratch. One cargo job at a time, with targets in WT/targets/rv89_g7/ (base, and `stale` for the RUSTFLAGS build).
- **The allocation probe.** It was mounted in my copy's lib.rs only for its run. lib.rs was then restored from `7f07a2f7b4` with `git show`, compared equal, and the probe file removed.
- **Not run.** I did not run the whole Pass B script, because it writes to I65's scratch and targets. I ran its gates on my own inputs instead. No Git writes or index operations (Git reads, `archive` and `show` used `GIT_OPTIONAL_LOCKS=0`), no installs, no native, solver-at-scale or DEC-025 jobs, and nothing in the system temp directory. WT/f2a-memory and I65's files were only read.
- **Writes.** Only this folder (ADDENDUM_01.md, `evidence/addendum_01/`, SHA256SUMS), WT/rv89_g7/, WT/targets/rv89_g7/ and WT/scratch/rv89_u4_g7_01/. Machine paths in the evidence are replaced by `WT` and `R`.
- **Copies.** No finding needs them. WT/rv89_g7 and WT/targets/rv89_g7 are deleted after this addendum.
