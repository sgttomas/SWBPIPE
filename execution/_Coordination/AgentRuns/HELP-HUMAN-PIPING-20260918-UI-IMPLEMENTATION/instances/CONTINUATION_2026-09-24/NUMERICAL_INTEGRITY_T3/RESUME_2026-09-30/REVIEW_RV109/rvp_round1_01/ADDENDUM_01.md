# RV109 (RV-P, B1) round 1, ADDENDUM_01: confirmation of ST's repair round 1

TASK (Type 2), RV109, role RV-P for B1, for ROOT (HELP_HUMAN, Agent 0), who is the return path. I made no delegation. 2026-10-07 UTC. This addendum adds to `REVIEW.md` in this folder; REVIEW.md and its `SHA256SUMS` are unchanged.

**The request:** ROOT's message to RV109, carrying RR "Owner decision: SI1c is option D, a repair within grammar 1.0.0; RV108 passes B6; RV109 passes ST with SF-1" (RR read at `0c41c577…`). That ruling repairs SF-1 before I1, with N-1 and N-4 in the same round. N-2 goes to SP, N-3 to SA and N-5 to SQ.

**The candidate:** `codex/piping-t3-b1-20261007` at **`98a77c716ef0bf1126c855d1f99ccb0778a5457a`** in `WT/b1`, one commit over `a8e719f5b4`.

**Read:**
- I85's brief `R/BRIEFS/B1_ST_REPAIR_01.md` (`be7d4c3f…`);
- the commit's diff;
- **after forming my own view,** I85's `R/I85/b1_st_01/REPAIR_01.md`: sha256 `8d2d714ee0ab71566b69f5bdbc691ceaa5e8bcd4fa04f026b4dc14327ef0dd81`, equal to the dispatch's `8d2d714e…`. `SHA256SUMS.repair_01` checks 34 of 34 OK.

**Placeholders:** as in REVIEW.md. `HEAD` = my fresh archive `WT/rv109/head` of `98a77c716e`, now deleted. `E1` = `evidence/addendum_01/`.

**Limits kept.**
- **24 cargo jobs** (`E1/cargo_jobs_rv109_addendum_01.log`), each through `WT/tools/t3_cargo.sh` with REVIEW.md's settings, in fresh targets `WT/targets/rv109-a1-*`. They interleaved with I88's jobs; I killed none.
- **Waits:** one wait per job, each ending on the job's result file or when its process was gone. No wait of mine is running.
- **No Git writes;** reads used `GIT_OPTIONAL_LOCKS=0`. Nothing in the system temp directory.
- **Cleanup:** `HEAD` and the four `rv109-a1-*` targets were deleted after the last job ended (13:53 UTC). My scratch is kept.
- **Reuse:** the `a8e719f5b4` side of every comparison is my round-1 run of that same revision (REVIEW.md §2 item 8; full logs kept in scratch). The two revisions' archives differ only in the two test files.

## 0. Result

**CONFIRMED.** SF-1 is fixed, and so are N-1 and N-4. Nothing is weakened, and no product `src` changed.

| BLOCKING | SHOULD-FIX | NOTE |
|---|---|---|
| 0 | 0 | 1 (A1-N-1, record wording only) |

## 1. SF-1 is fixed

**The new pin** is `assert_no_triggered_case_pin(label, raw, input_sha256)`, `pub(super)` inside the `#[cfg(test)]` facade module. It holds two-body B's former body, plus two new checks. For each mode, on the private driver:
- **A precondition:** diagnostics capacity = length before `retained_w1`. `observed()` shrinks the diagnostics.
- **The capacity check:** after `NoTriggeredCase`, `(capacity, len)` = `(capacity before, capacity before)`, so no slot was reserved.
- **The collision variant:** a base already carrying `diagnostic:retained-precision:<case>:unavailable`, shrunk, must give `NoTriggeredCase` with the colliding base's exact bytes.

**Mutants** (`E1/mutants/summary.md`; whole lib suite with the ignored witnesses):
- **R10** (reserve, then T-4) is killed by **the capacity check:** "T-4 precedes R-2: no diagnostics slot was reserved". It fails both in two-body B's pin and in N-4's default pin (W2b's input).
- **R10b** is new: R10 that also gives the slot back with `shrink_to_fit`, so the capacity check cannot see it. It is killed **by the collision variant alone** ("T-4 precedes R-2's collision check"; the cause is `NoticeReservation`), in both pins. The collision assertion therefore kills a reservation ahead of T-4 on its own.

**The pin asserts what its name says:** `NoTriggeredCase`, with:
- the exact plain bytes;
- no notice;
- no reserved slot;
- no W1 stage (the native sentinel stays armed);
- the collision ordering;
- on Direct: admitted (registered), one ordinary run through G-C once, no successor, and the plain bytes; Stale: `ONE_RUN` and the plain bytes.

The input's PROBE sha and its `checks_passed` verdict are asserted first.

## 2. N-1 and N-4 are fixed

- **N-1.** The classifier test gains `assert_eq!(one(Some(V::ChecksPassed), None), NotRequired, "Passed with no seed")`. **R16 is killed by exactly that assertion.**
- **N-4.** `retained_memory::witness_tests::b1_t4_w2b_and_w6_phys_r4_inputs_are_no_triggered_case_pins` is a default-suite test (no `#[ignore]`). It runs the same pin on:
  - W2b's input (sha `d74d01ce…`);
  - W6's PHYS-R4 input (sha `19a424c5…`).

  So each input is pinned in both modes, on the private driver and on Direct: `NoTriggeredCase`, exact plain bytes, no notice, no reservation, no W1 work. It passes registered and Stale.
- **Each half is live.** P1 (keyed on `initial`) passes W2b's half (a report-Passed seed) and is killed at **W6's half** ("W6's PHYS-R4 input SparseInteractive"). R10 is killed at W2b's half.

## 3. Scope, and nothing weakened

**Only the two test files changed:**

| File | Blob `a8e719f5b4` → `98a77c716e` |
|---|---|
| `retained_facade_tests.rs` | `148d5e4673` → `8ac979fa78` (+71 / −37) |
| `retained_memory_witness_tests.rs` | `aeabe8e48a` → `3381a6bd13` (+33 / −14) |

- `PP/lib.rs` is still `84fba5ca…` (blob `f26924d239`).
- `retained_product.rs`, `retained_memory.rs` and the law tests have the same blobs.
- Nothing else changed in the tree.

**All 51 removed lines were checked mechanically against the added lines** (normalized for the `{label}` message prefix and the `raw` reference). 41 are code and 10 are comments or doc:
- **39 code lines reappear verbatim.** These are two-body B's assertions and Direct block, now in the pin, and W2b's input builder, now in `fn w2b_input()`.
- **The other 2 code lines** are the two witness calls, which now pass `w2b_input()` and the constants `W2B_INPUT_SHA256` and `W6_PHYS_R4_INPUT_SHA256`. Those constants equal the former literals byte for byte.
- **4 comment lines reappear verbatim.** The other 6 are doc lines rewritten to the new shape: the classifier test's list, and two-body B's doc, which now points to the pin.
- Two-body B's test keeps its own seed-shape assertion (structural failure, W2 published).

**The ignored witnesses are unchanged in substance.** They keep the same inputs (sha asserted), the same assertions and the same labels. Their `I65_G5_WITNESS` lines are identical to `a8e719f5b4`'s, sorted, line for line, registered and Stale (10 passed each).

## 4. Suites and pins (`E1/suites/`, `E1/witness/`)

| Run | `a8e719f5b4` | `98a77c716e` | Per-test difference |
|---|---|---|---|
| PP registered, all targets | 711 ok / 1 failed (`t13`) / 11 ignored | **712** / 1 (`t13`) / 11 | **only** `+ retained_memory::witness_tests::b1_t4_w2b_and_w6_phys_r4_inputs_are_no_triggered_case_pins … ok` |
| PP Stale `--lib` | 548 / 1 / 11 | 549 / 1 / 11 | the same single line; Stale = registered over all 561 lib tests |
| Witnesses (`--lib witness_ -- --ignored`), registered and Stale | 10 passed | 10 passed | lines identical |
| RE `retained_precision_carriers` | 16 passed | 16 passed | – |

- **The c = 1 successor pins pass,** registered and Stale:
  - `u1_milestone_successor_both_modes`;
  - `u3_permitted_path_publishes_the_pinned_successor`;
  - `u3g2_direct_entry_publishes_the_pinned_successor`;
  - `u3g2_d_u6_5_carrier_fixtures_are_the_live_successors`;
  - `u3_r1_carrier_names_exactly_one_publication`;
  - `u8_l0_isolated_node_publishes_pinned_successor`;
  - `u8_d_u6_5_l0_fixtures_are_the_live_successors`;
  - W2-deep at R/16 and R/64.
- **The guards pass:**
  - `s11f_site_test` 11/11 and PP-tests' `retained_precision_admission` 5/5;
  - the in-fence guards: `u3_n9_single_parse_custody`, `u3_permitted_outputs_keep_the_report_and_gate_order`, `u3_capture_permit_is_linear`, `u3g2_no_permit_path_runs_once_without_a_copy`, `u1_serializer_reads_no_legacy_work_field`, `the_registered_profile_is_the_only_permit_source`, `admit_grants_a_permit_for_the_milestone_in_the_registered_build` and `challenge_bounds_are_the_profile`.

  Their inputs (`lib.rs` and the other product files) are byte-identical to round 1.
- **The runner was not rerun.** It does not compile PP's `cfg(test)` modules, and PP's non-test source is unchanged.

**The full mutant set at the head** (`E1/mutants/`):
- the pristine control: 560 passed, 1 failed (`t13`);
- all of PLAN_v2's five, and my R6–R16 with R10b, are killed by assertions;
- only R17 survives: equivalent at c = 1, and SP's by ruling (N-2);
- every mutant compiled, and `PP/lib.rs` was restored and checked afterwards (`84fba5ca…`).

## 5. Finding

| ID | Severity | Path | Evidence | Remedy |
|---|---|---|---|---|
| A1-N-1 | NOTE | `R/I85/b1_st_01/REPAIR_01.md` §5 item 3 | It says the N-4 test's "name does not match `witness_`, so the explicit witness runs do not pick it up". libtest filters on the full path, `retained_memory::witness_tests::b1_t4_…`, which does contain `witness_`. The witness runs skip it only because `--ignored` selects ignored tests: 10 run and 551 are filtered out of 561. The outcome holds, but the stated reason does not. It would matter only for a `--include-ignored` witness run, which would also run this cheap pin | None to the code. A records-only correction, if ROOT wants one |

## 6. The PR-head ledger, extended

**Head `98a77c716e`** = `4a51783e65` (ST) + `a8e719f5b4` (test-only) + `98a77c716e` (ST repair 1, test-only), over main `47a3bdfcf5`. Rows 1–30 are REVIEW.md §3's, unchanged in content. In the repair's two files their line numbers shift, but not their content or commit, except as rows 31–35 state.

| # | File | Hunk (`a8e719f5b4` → `98a77c716e`) | Content | Commit | Reviewed |
|---|---|---|---|---|---|
| 31 | PP `retained_facade_tests.rs` | `-1140,8 +1140,9` | the classifier test's doc ("no seed (in A, unless the verdict is Passed)") | `98a77c716e` | RV109 r1 A1 |
| 32 | | `-1195,6 +1196,8` | N-1's row "Passed with no seed" | `98a77c716e` | RV109 r1 A1 |
| 33 | | `-1215,50 +1218,81` | `assert_no_triggered_case_pin` (SF-1's capacity check and collision variant); two-body B's test now calls it. This rewrites row 16's two-body B pin | `98a77c716e` | RV109 r1 A1 |
| 34 | PP `retained_memory_witness_tests.rs` | `-157,21 +157,28` | W2b's witness calls `w2b_input()`; `fn w2b_input()`; the two sha constants. This touches row 26's W2b pin | `98a77c716e` | RV109 r1 A1 |
| 35 | | `-254,10 +261,22` | W6-PHYS-R4's witness uses its constant; N-4's default-suite test. This touches row 28's W6-PHYS-R4 pin | `98a77c716e` | RV109 r1 A1 |

**Every hunk of `47a3bdfcf5..98a77c716e` is now reviewed.** Nothing is carried open from round 1 for ST. N-2, N-3 and N-5 are with SP, SA and SQ by ruling. Later rounds extend this ledger with I1's merge of main and SP's commits.

## 7. Records

- `ADDENDUM_01.md` (this file) and `SHA256SUMS.addendum_01`, which covers this file and every file under `evidence/addendum_01/`.
- **`evidence/addendum_01/`:**
  - `suites/`: the filtered logs and the test-by-test differences;
  - `witness/`: the witness lines, registered and Stale;
  - `mutants/`: `summary.md`, the filtered logs, each apply record and `lib.rs` sha, the pristine control and the restore check;
  - `diffs/`: the repair's diff;
  - `tools/`: `a1_confirm.sh`, `records_a1.sh`, `runjob.sh`, `mutants.py` (with R10b added), `mutant_summary.py`, `suite_diff.py` and `sanitize.py`;
  - `cargo_jobs_rv109_addendum_01.log`.
- Machine paths are replaced by placeholders, as in REVIEW.md.
