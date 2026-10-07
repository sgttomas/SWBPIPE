# I85 B1-ST repair round 1: RV109's SF-1, N-1 and N-4 (tests only)

TASK (Type 2), I85, role I-P, for ROOT (HELP_HUMAN, Agent 0), who is the return path. I made no delegation. 2026-10-07 UTC. This record adds to `RETURN.md` in this folder; it does not replace it. RETURN.md and its `SHA256SUMS` are unchanged.

**Brief (verified before work):** `R/BRIEFS/B1_ST_REPAIR_01.md`, sha256 `be7d4c3f1b8f9d0015c4e88a78ff122b741133b8731840051cbc9e9a7298c112`. B1_COMMON and B1_ST still apply.

**Read:**
- `R/REVIEW_RV109/rvp_round1_01/REVIEW.md`, sha256 `707045b99bf777443163ad42a4c846e92234b1d502465662c8162d167ab1569d` (verified);
- from RV109's evidence: R10's and R16's exact edits (`evidence/tools/mutants.py`), and R10's discriminator, `zz_rv109_r10_discriminator` in `evidence/probe/zz_rv109_probe.rs`, with its two runs. I used the discriminator as a reference, not as code to copy;
- RR "Owner decision: SI1c is option D, a repair within grammar 1.0.0; RV108 passes B6; RV109 passes ST with SF-1", at NUM `81d9d0fa0b`.

**Placeholders:** as in RETURN.md. `S` = `WT/scratch/i85_b1_st`. `MUT` = `S/mut`, a `git archive` of the new head (`P` without `P/execution`), now deleted.

**Limits kept.**
- **Tests only, inside ST's fence.** No product `src` changed:
  - `PP/lib.rs` is still `84fba5ca57905d69ea979ce815e584f26eb95ac45d32a54e30918d7580edd586`, as at `a8e719f5b4`;
  - `retained_product.rs`, `retained_memory.rs` and the law tests are unchanged.

  Only `retained_facade_tests.rs` and `retained_memory_witness_tests.rs` changed.
- **Cargo:** 13 jobs, each through `WT/tools/t3_cargo.sh`, with RETURN.md's settings (`--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, `TMPDIR` in `S`, `RUSTFLAGS` unset except Stale's `--cfg=i85_b1_st_stale`). They waited behind ROOT's DEC-025 `B6_1199726f69` (12:26–13:15 UTC) and interleaved with I88's jobs (`_run_records/repair_01/cargo_jobs_i85_repair_01.log`). I killed no job.
- **Waits.** One wait per job, with one slip:
  - Twice I started a second wait on the evidence job by mistake. Each was a shell loop that only read the process list, and I stopped each at once with TaskStop.
  - I then stopped the first wait too, and waited instead with bounded loops, one at a time, each ending by itself. The last ended when the job's process was gone.
  - **No wait of mine is running,** and no process of mine remains.
- **Git:** one commit on `codex/piping-t3-b1-20261007`, not pushed; reads used `GIT_OPTIONAL_LOCKS=0`. Nothing was written to the system temp directory. The host accepted every write into this folder.
- **Cleanup:** `MUT` and its target `WT/targets/i85-b1-st/mut` are deleted, as the brief asks.

## 0. Outcomes in brief

| Item | Outcome |
|---|---|
| **SF-1:** T-4 pinned ahead of R-2's reservation | **Done.** Two assertions were added to the shared `NoTriggeredCase` pin: no diagnostics slot after `NoTriggeredCase`, and a collision variant. **R10 is killed by each of them independently** (§3) |
| **N-1:** "Passed with no seed is `NotRequired`" | **Done. R16 is killed** |
| **N-4:** default-suite `NoTriggeredCase` pins for W2b's input and W6's PHYS-R4 input | **Done:** `b1_t4_w2b_and_w6_phys_r4_inputs_are_no_triggered_case_pins`, not `#[ignore]`, in both modes, on the private driver and Direct. The ignored witnesses keep their inputs and assertions |
| PLAN_v2's five mutants | **Still killed, each by an assertion** |
| PP registered suite against `a8e719f5b4` | 711 → **712 ok**; 1 failed (`t13`, known) and 11 ignored, both unchanged. **The only difference is the 1 added test** (§4) |
| PP Stale | The same single difference. Stale = registered, outcome for outcome |
| c = 1 successor pins | Pass |
| s11f, PP-tests' admission guard, RE's carrier test and the in-fence guards | Pass, with their expected text unchanged |

## 1. Head and commit

**New head: `98a77c716ef0bf1126c855d1f99ccb0778a5457a`** on `codex/piping-t3-b1-20261007`, one commit over `a8e719f5b4`:

`98a77c716e piping(T3 B1 ST): repair 1, pin T-4 ahead of R-2's reservation; RV109 N-1 and N-4`

| File | Blob at `a8e719f5b4` | Blob at the head | sha256 at the head | Lines |
|---|---|---|---|---|
| `PP/retained_facade_tests.rs` | `148d5e4673` | `8ac979fa78` | `7b3d229c48f03e0f9f86e99e07e7ba41f5e4054370c3c6f8d162ca7610732bba` | +71 / −37 |
| `PP/retained_memory_witness_tests.rs` | `aeabe8e48a` | `3381a6bd13` | `dc7161103bbe59bb23b1362047cba55c351036a21c243e737ebe1b5daa582715` | +33 / −14 |

The diff is `_run_records/repair_01/repair_01.diff`.

## 2. The changes

### 2.1 SF-1 (`PP/retained_facade_tests.rs`)

**The shared pin.** `b1_t4_two_body_case_b_is_a_no_triggered_case_pin`'s body is now `pub(super) fn assert_no_triggered_case_pin(label, raw, input_sha256)`, so that N-4's test runs the same pin.
- Its visibility is test-only, inside the `#[cfg(test)]` facade module, as R3 ruling 3 accepted for `w_c2_case_c`.
- Two-body B's test keeps its own seed-shape check (structural failure, W2 published) and then calls the pin.

**Per mode, the pin asserts:**
- **The input:** its PROBE sha256, and that the published verdict is `[(case, checks_passed)]`.
- **The private driver** (`observed()` shrinks the diagnostics, as the existing tests rely on):
  - the observed run equals the plain run;
  - **precondition:** diagnostics capacity = length;
  - with a native-stage sentinel armed, `retained_w1` returns `NoTriggeredCase`, and the bytes are the plain bytes;
  - **SF-1 (new):** `(capacity, len)` equals `(capacity before, capacity before)`, so no diagnostics slot was reserved. The message reads "T-4 precedes R-2: no diagnostics slot was reserved";
  - the sentinel is still armed, so no W1 stage ran.
- **SF-1's collision variant (new).** On a fresh observed run, the base gets a diagnostic whose id is `diagnostic:retained-precision:<case>:unavailable`, as `u3_each_stage_fault_falls_back_to_the_ordinary_bytes` does for the milestone, and is shrunk again. `retained_w1` must return **`NoTriggeredCase`** ("T-4 precedes R-2's collision check"), with the colliding base's exact bytes. A reservation made ahead of T-4 ends at `NoticeReservation` here.
- **The actual Direct entry,** as before:
  - registered: admitted, `NoTriggeredCase`, no successor, `ONE_RUN_THROUGH_G_C`;
  - Stale: no W1, `ONE_RUN`;
  - either way: the sentinel is handed back unfired, there are 0 notices, and the bytes are exactly the plain bytes.

### 2.2 N-1 (the classifier test)

**New row:** `assert_eq!(one(Some(V::ChecksPassed), None), NotRequired, "Passed with no seed")`, with a comment saying the Passed verdict decides alone. The test's doc now reads "no seed (in A, unless the verdict is Passed)".

### 2.3 N-4 (`PP/retained_memory_witness_tests.rs`)

- **The new test.** `b1_t4_w2b_and_w6_phys_r4_inputs_are_no_triggered_case_pins` is in the default suite, not `#[ignore]`. It runs `assert_no_triggered_case_pin` on:
  - W2b's input (sha `d74d01ce…`), an ordinary Passed report at the cap-maximal counts;
  - W6's PHYS-R4 input (sha `19a424c5…`), one body, W2-published Passed.

  **Why it lives in the witness module:** `w6_input()` and `law_tests::cap_maximal()` are visible only inside `retained_memory`. Making `retained_memory`'s test modules visible would be an edit to `retained_memory.rs` outside ST's `CompleteFacts`-only allowance. Its doc says it is not a witness.
- **The input builder.** W2b's input is now built by `fn w2b_input()`, the same statements moved out of the witness. The two PROBE input shas are now constants, `W2B_INPUT_SHA256` and `W6_PHYS_R4_INPUT_SHA256`.
- **The ignored witnesses are kept as they were:**
  - `witness_w2b_cap_maximal_passed_report_no_triggered_case` and `witness_w6_phys_r4_input_no_triggered_case` run the same inputs, whose shas are still asserted, with the same assertions;
  - their `I65_G5_WITNESS` lines are identical to before (§4).

## 3. Mutants (`_run_records/repair_01/mutants/`)

**Method.** RETURN §7's: one textual edit of `MUT`'s `PP/lib.rs`, then PP's whole `--lib --no-fail-fast`, registered. `PP/lib.rs` was restored and checked (`84fba5ca…`) after each. `t13` fails in every run, as on base, and is excluded. R10's and R16's edits are RV109's exactly; R10 includes its `drop(notice)`.

| Mutant | Compiled | Killed by (first failing assertion of each failing test) |
|---|---|---|
| **R10** T-4 after the reservation (RV109) | yes | **KILLED.** Two-body B pin: "T-4 precedes R-2: no diagnostics slot was reserved". The W2b/W6 default pins: the same assertion, on W2b's input |
| R10, with the capacity assertion removed from the test copy (`r10_collision_only.py`) | yes | **KILLED by the collision variant alone.** Two-body B pin and W2b's input: "T-4 precedes R-2's collision check" (the cause is `NoticeReservation`). Both files were restored and checked |
| **R16** `not_required` needs a seed (RV109) | yes | **KILLED.** Classifier test: "Passed with no seed" |
| M1 keying on `initial` | yes | KILLED. Classifier test "W2-published Passed"; two-body B pin (cause); W6's PHYS-R4 default pin (cause) |
| M2 verdict by position | yes | KILLED. Classifier test "request order, looked up by id" |
| M3 dropping decision 21 | yes | KILLED. Classifier test "Mechanism … Some(Failed), no W2"; decision-21 test "excluded: A is empty" |
| M4 a seedless case excluded | yes | KILLED. Classifier test "no seed"; decision-21 test "no seed: in A, so W1 runs" |
| M5 `NoTriggeredCase` appends a notice | yes | KILLED. Decision-21 test "excluded: the exact ordinary bytes"; two-body B pin and W2b's default pin "the exact ordinary bytes, no notice" |

**7 of 7 killed by assertions, none by compilation.** R10 is killed by each of SF-1's two assertions independently. R17 (`.all` for `.any`) stays equivalent at c = 1 and is SP's, by ruling.

## 4. Suites and guards at the head (`_run_records/repair_01/suites/`, `witness/`, `guards/`)

| Run | At `a8e719f5b4` | At `98a77c716e` | Per-test difference |
|---|---|---|---|
| PP registered, all targets | 711 ok, 1 failed (`t13`), 11 ignored | **712 ok**, 1 failed (`t13`), 11 ignored | **+1:** `retained_memory::witness_tests::b1_t4_w2b_and_w6_phys_r4_inputs_are_no_triggered_case_pins … ok`. Nothing else (`diff_reg_a8e719f5b4__98a77c716e.txt`) |
| PP Stale (`--cfg=i85_b1_st_stale`), all targets | 711 / 1 / 11 | 712 / 1 / 11 | the same single line. Stale = registered, outcome for outcome |
| Witnesses, `--lib witness_ -- --ignored` (registered) | 10 passed | 10 passed | `I65_G5_WITNESS` lines identical. The earlier lines are from `4a51783e65`, whose witness module equals `a8e719f5b4`'s |
| RE `retained_precision_carriers` | 16 passed | 16 passed | – |

- **Compiler warnings:** the same messages and counts as at `a8e719f5b4` (`*.warnings`).
- **The c = 1 successor pins pass,** registered and Stale:
  - `u1_milestone_successor_both_modes`;
  - `u3_permitted_path_publishes_the_pinned_successor`;
  - `u3g2_direct_entry_publishes_the_pinned_successor`;
  - `u3g2_d_u6_5_carrier_fixtures_are_the_live_successors`;
  - `u3_r1_carrier_names_exactly_one_publication`;
  - `u8_l0_isolated_node_publishes_pinned_successor`;
  - `u8_d_u6_5_l0_fixtures_are_the_live_successors`;
  - W2-deep's successors on the witness stack.
- **The guards pass with their expected text unchanged:**
  - `PP-tests/s11f_site_test.rs` 11/11 and `PP-tests/retained_precision_admission.rs` 5/5, both in the PP suite;
  - RE's carrier test 16/16;
  - the in-fence guards: `u3_n9_single_parse_custody`, `u3_permitted_outputs_keep_the_report_and_gate_order`, `u3_capture_permit_is_linear`, `u3g2_no_permit_path_runs_once_without_a_copy`, `u1_serializer_reads_no_legacy_work_field`, `the_registered_profile_is_the_only_permit_source`, `admit_grants_a_permit_for_the_milestone_in_the_registered_build` and `challenge_bounds_are_the_profile`.

  No product file changed, so no guard's input changed.

## 5. For ROOT and RV109

1. **No stop fired.** No product `src` change was needed.
2. **The pin's new shape.** Two-body B's private-driver and Direct assertions moved into `assert_no_triggered_case_pin`. Each existing assertion is kept, with the input's label added to its message; nothing was weakened. The pin now also runs, by default, on W2b's and W6's PHYS-R4 inputs.
3. **The N-4 test sits in `witness_tests`, not in the facade tests,** for the visibility reason in §2.3. It is a plain default-suite test there; its name does not match `witness_`, so the explicit witness runs do not pick it up.
4. **The W2b witness was refactored only to move its input builder** into `w2b_input()`. Its input sha and its assertions are unchanged, and its witness lines are identical.
5. **Ledger for RV-P:** one commit, `98a77c716e`, test-only, with two hunk groups:
   - `retained_facade_tests.rs`: the classifier row, the shared pin and two-body B's test;
   - `retained_memory_witness_tests.rs`: `w2b_input`, the sha constants and the N-4 test.
6. **Budget:** about 1.1 h of agent time (12:20–13:30 UTC), much of it waiting for DEC-025.

## 6. Records

- **`_run_records/repair_01/`:**
  - `repair_01.diff` and `commits.txt`;
  - `scripts/`: `run_all.sh`, `r10_collision_only.py`, `mutants.py` (with R10 and R16 added), `run_suites.sh`, `cargo_cand.sh` (now with a `LOGDIR` override) and `sanitize.py`;
  - `suites/`: the logs, the outcomes at both heads, the diffs and the warnings;
  - `witness/`, `guards/`, `mutants/` (`mutants.json` and the filtered logs, including `mutant_R10_collision_only`), `build/` and `run_all.out`;
  - `cargo_jobs_i85_repair_01.log`.
- **Path and length handling:** machine paths are replaced by `WT`, `VENV` and `~`. A line over 4,000 bytes is cut to 1,000 bytes, followed by its length and sha256.
- **`SHA256SUMS.repair_01`** covers REPAIR_01.md and every file under `_run_records/repair_01/`.
- **Kept in `S`:** the full logs, `BASE`, and the targets `WT/targets/i85-b1-st{,/base,/stale,/base-stale}`, for SP. `MUT` and its target are deleted.
