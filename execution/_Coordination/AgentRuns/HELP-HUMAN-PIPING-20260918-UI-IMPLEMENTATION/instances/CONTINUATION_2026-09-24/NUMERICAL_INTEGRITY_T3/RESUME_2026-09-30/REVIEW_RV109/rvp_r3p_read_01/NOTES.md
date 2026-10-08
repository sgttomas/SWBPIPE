# RV109 (RV-P, B1): early read of SP's first part at checkpoint R3′ (interim notes)

TASK (Type 2), RV109, role RV-P for B1, for ROOT (HELP_HUMAN, Agent 0), who is the return path. I made no delegation. 2026-10-07 UTC.

**These are interim notes, not a round-2 verdict.** PLAN_v2 §2.2 lets RV-P start reading at R3′; RV-P's round 2 on SP comes later, by its own brief.

**The request:** ROOT's message to RV109, with RR "R3′: SP's first part verified; c = 1 through the n-case path; no serializer split; RV109 reads early" (NUM `a4c2cfa01b`; RR read at `e5789aeb…`). It asks for a read against DESIGN_v2 T-2, T-5, T-6 and T-7 and RV-P's oracles (PLAN_v2 §5), on six points (a)–(f).

**The candidate:** `codex/piping-t3-b1-20261007` at **`56c5579f07b1cf3203c178e10add51bba0b24a58`**, one commit over I1 `262bd687f0`. It changes six files in SP's fence: `PP/lib.rs`, `retained_product.rs`, `retained_wire.rs`, `retained_facade_tests.rs`, `retained_product_tests.rs` and `retained_tests_hooks/grant2.rs`. I read `retained_product.rs` with `git diff -w`.

**Read:**
- DESIGN_v2 §1.2 (T-2 to T-12), PLAN_v2 §2.2 and §5;
- the commit's diff;
- the code around it: `bind_observations`, `solver_observations`, `capture_case_source`, `finish`, the hooks, `retained_w1` and the envelope's row-id qualification in `lib.rs`;
- **after forming my own view,** I85's `R/I85/b1_sp_01/CHECKPOINT_R3P.md` (sha256 `3e349d7a…`, equal to the dispatch's).

**Placeholders:** as in `R/REVIEW_RV109/rvp_round1_01/REVIEW.md`. `SP` and `I1` = my archive copies `WT/rv109/sp` (`56c5579f07`) and `WT/rv109/i1` (`262bd687f0`), now deleted. `E` = this folder's `evidence/`.

**Limits kept.**
- **Cargo:** 25 jobs (`E/cargo_jobs_rv109_r3p.log`), each through `WT/tools/t3_cargo.sh`, `--locked --offline`, with REVIEW.md's settings, in fresh targets `WT/targets/rv109-*`.
  - I ran no test binary outside cargo, so ROOT's restated host rule (every heavy job under the lock) held throughout.
  - The lock was shared with I85, I89, I90, I92, RV111 and RV113; I killed no job.
- **Waits:** one wait per job, each ending on the job's result or when its process was gone. No wait of mine is running.
- **No Git writes.** Every write used an absolute path. No record folder is named `build`.
- **Reviewer-only edits in my copies:** the probe (`#[cfg(test)]`), the mutants (one at a time, restored and checked by sha256), and, in one labelled run, `caps::LOAD_CASES = 3` to simulate I2 (§3.6). All were deleted afterwards.

## 0. In brief

| BLOCKING | SHOULD-FIX | NOTE |
|---|---|---|
| 0 | 2 | 7 |

**What holds:**
- **(a) c = 1 is untouched by these structures.**
  - **Bytes:** the plain, Direct-published and driver bytes of all 112 input-and-mode rows from round 1 are identical at I1 and at the checkpoint, and so is every successor and receipt.
  - **Adapter counts:** **identical at every W1 stage boundary for every c = 1 input** (104 rows: after the ordinary run, after preparation, after native, and after the candidate). The only rows that differ are the 8 multi-case fixture rows (c = 2), where the capture now accepts more than one case, as designed.
  - No parking event occurs at c = 1.
- **(b) Custody runs once,** before any attempt, and refuses on each planted fault.
- **(c) Decision 19's binding keeps `bind_observations`' envelope checks.** The case-qualified row ids work in every case order I tried (5 orders × 2 modes).
- **(d) T-7 isolates a failed preparation to its case,** with its snapshot at preparation.
- **(e) The domain re-check is right.**

**The two SHOULD-FIXes:**
- **R3P-1:** `into_single` sends a multi-case invocation with one case in A down the one-case path, whatever case sits in the capture's own fields.
- **R3P-2:** `prepare_cases` can refuse after attempts have started.

## 1. Notes

| ID | Class | Path | Evidence | Remedy |
|---|---|---|---|---|
| **R3P-1** | SHOULD-FIX | PP `retained_product.rs` `PreparedCases::into_single`; PP `lib.rs` `retained_w1` | `into_single` checks only `attempts.len() == 1`. With c ≥ 2 and \|A\| = 1, the one-case continuation (native, candidate, staging, the one-case serializer) runs on **the case in the capture's own fields, which is the last requested case**, not necessarily the attempted one. **Shown at the API on this head** (§3.6): for orders (A, B) and (C, B) with A = {0}, the `PreparedCase` holds case B (its unprepared captured source, 2 loads). Native then refuses (`NativeUnavailable`), and so does the candidate. For (B, A), the attempted case is last; native runs, and the one-case candidate refuses "observation fixed fields", because its binding expects an unqualified row id for a case at index 1. **With a reviewer-only `LOAD_CASES` = 3 (simulating I2; §3.6),** `retained_w1` on the private driver ends `Native` for (A, B) and (C, B), after running native on B's unprepared source, and `Candidate` for (B, A). Each publishes the ordinary bytes plus one notice, and no successor. These outcomes are fallbacks only by accident of later checks. The path is reachable after I2 and before T-8. CHECKPOINT_R3P §2.4 and §7.3, and R3′ ruling 4, describe the interim only for several attempts | Route every c ≥ 2 invocation to the interim fail-safe until T-8: `into_single` returns `Err(self)` unless `capture.cases_seen() == 1` (or `requested == 1`). Or bring the attempted case into the capture's own fields first. Pin it with a two-case test where A is the first case |
| **R3P-2** | SHOULD-FIX | PP `retained_product.rs` `prepare_cases` | **One refusal sits inside the attempt loop:** `request >= requested` returns `CustodyFailure` after earlier attempts have run, and drops their parts and traces. Nothing checks that `attempted` is strictly increasing and unique; a repeated index would run a second attempt on the same case, starting from the source the first attempt left in its slot. These cases are unreachable from `retained_w1`, whose A is a subset of the request in order. But `prepare_cases` is the interface T-8 to T-11 and the tests build on, and T-6 requires every refusal before any attempt | Validate `attempted` in custody, before any attempt: each index less than `requested`, strictly increasing. Add a test with an out-of-range index and one with a duplicate, both refused before any attempt (preparation capacity 0) |
| R3P-3 | NOTE | `prepared_custody` (code and doc), CHECKPOINT_R3P §2.2 item 1 | The doc says "a prior capture error of any case, the first in request order, is the cause". The code takes the capture's own `error` first; that belongs to the **last** case seen. Then it takes the parked slots, in order. With errors in case 0 and case 2, the cause reported is case 2's. The tests plant one error at a time, so they cannot see this | Check the parked slots first, then the own fields; or reword the doc. The cause matters only to internal evidence (T-12 publishes notices only) |
| R3P-4 | NOTE (for T-8) | `CaseSlot::source` after T-7 | After T-7, a slot's `source` is `Some` for **every** case: the prepared source for a successful attempt, but **the old captured source** for a `not_required` case (B in W-C2, which I85's test asserts) and for a case whose preparation failed. `prepare_active_case` sets `source` only at its end | T-8 must select its batch by `CaseAttempt::prepared` (with `request`), never by `slot.source.is_some()`. A typed "prepared" marker, or keeping the prepared source in the attempt, would make the mistake impossible |
| R3P-5 | NOTE (for T-12) | PP `lib.rs` `ReservedNotices::publish` | The receipt-encoding detail is applied to **every** notice when the cause is a C1:68 serializer failure. T-12 (decision 6) places it only on the cases that were selected when the successor was abandoned; a case unavailable for its own cause keeps the plain notice. This is exact at c = 1, and CHECKPOINT_R3P §2.4 says the placement is still one-case | Give `publish` the selected set, or a per-notice flag, when T-11 and T-12 land; pin it with a two-attempt fault test |
| R3P-6 | NOTE | PP `retained_tests_hooks/grant2.rs` `fail_preparation_of_case` doc | The doc says "`fail_next_preparation` would fail the first attempted case instead". `fail_next_preparation` is consumed at G-C (`at_complete_gate`) and zeroes `facts[0]` of **the case in the capture's own fields, which at c ≥ 2 is the last requested case** | Reword: "… would fail the last requested case, the one in the capture's own fields at G-C" |
| R3P-7 | NOTE | `prepared_custody`, `finish`, `bind_observations_by_case` (tests) | **Three of T-6's c ≥ 2 checks are unpinned,** because my mutants survive:<br>• **S3:** the per-case final presence check dropped (one mode row and the parity rows per case);<br>• **S4:** `finish` no longer checks the parked slots' late captures. Then a parked case without its late capture is no longer a custody refusal; it degrades to a per-case preparation failure. This is part of (b);<br>• **S7:** native work in a parked slot not refused | Add these to SP's T-6 custody fault tests and mutant list, as refusals before any attempt (preparation capacity 0):<br>• a case missing its final mode row, or carrying two;<br>• a parked slot whose late capture did not complete;<br>• a parked slot with `native` set |
| R3P-8 | NOTE | `ReservedNotices` (T-5); `observation_capacity_bytes` | **Unpinned until later, as expected:**<br>• S13 (one diagnostics slot whatever \|A\|) and S14 (no cross-notice id check) survive at `LOAD_CASES` = 1. The T-5 test's multi-count arms run only after I2;<br>• S20 (the observation capacity record assigned, not summed) survives until T-11 serializes a multi-case snapshot | Re-run S13 and S14 after I2, and S20 after T-11 (the n-case serializer), and keep them in SP's mutant list |
| R3P-9 | NOTE | `prepare_owned_case` (kept) beside `prepared_custody` | **Two custody preludes now exist.** The production path uses `prepared_custody` (via `prepare_cases`). The one-case `prepare_owned_case` keeps its own prelude. It sits behind `prepare_case`, which `retained_product_tests` call, and behind `PreparedCase::prepare_observed`. So the existing custody fault tests exercise the old prelude, not production's. They are event-identical at c = 1 (§3.3), but they can drift. `CustodyFailure` also drops the trace and leaves the error out of the capture; harmless for T-12 | When the one-case path is retired (R3′ ruling 1), route `prepare_case` through `prepare_cases(…, 1, &[0])` and move the custody fault tests onto it |

## 2. The six points

### (a) c = 1 byte identity and the adapter event sequence

- **Read.** At c = 1 the new code is event-neutral by construction:
  - `prepared_case_seen` parks only when `prepared_one_case_seen && parked.len() + 1 < c`;
  - `with_case(i)` is `f(self)` for the last case;
  - `finish` and custody call the unchanged `bind_observations` when nothing is parked;
  - `observation_fields_of(…, qualified = false)` adds no event;
  - `ReservedNotices` performs R-2's one id check, one `try_reserve_exact(1)` and one message reservation;
  - `prepare_active_case` is the old closure body. The trace is now created after custody instead of before, which the custody checks do not observe.
  - The one difference is the summed `observation_capacity_bytes`, written once per slot at c = 1.
- **Bytes** (`E/probe/compare_i1_sp.txt`): my round-1 probe on I1 and on the checkpoint gives 112 of 112 rows identical: plain, Direct-published, the driver's owner, and every successor and receipt. I1's rows equal round 1's head rows (`a8e719f5b4`) too, 112 of 112.
- **Adapter counts at every stage** (`E/probe/stage_*.jsonl`, `stage_compare.txt`). For each input and mode, I took the counts after the observed ordinary run, after the revision's production preparation (I1's `prepare_case`; the checkpoint's `prepare_cases(…, 1, &[0])` then `into_single`), after native and after the candidate.
  - **All 104 c = 1 rows are identical at every boundary,** including the outcome text: 22 frozen candidates, 8 proof refusals, 6 native refusals, and the custody and preparation failures.
  - **The 8 differing rows** are the four `multicase` source-block fixtures (c = 2) in both modes. I1 refused them at the first early hook's one-case scope check. The checkpoint accepts the two-case request, which is T-2 as designed, and captures case 0 until that case's own refusal ("missing produced mode", as for the exact-selected one-case fixtures). The later hooks then stop on the error, so `cases_seen` is 1. These fixtures are `Domain` on the Direct entry and `Coexistence` on the driver at both revisions, so no byte differs.
- **No parking event at c = 1:** none of the 104 rows changes its `MapWrite` or `AllocationRequest` counts.

### (b) Custody: refusals before any attempt, and one late capture per case

- `prepared_custody` runs before the first attempt and checks:
  - the prior errors (own fields and every parked slot);
  - the adapter fault;
  - the preconditions (one final hook, the preview contract, `MECHANICS_SOLVED`, no exact-block selection, no native work in any slot);
  - `cases_seen() == requested`;
  - the per-case binding.

  Mutants S5 (parked errors ignored) and S6 (no count check) are killed. A refusal returns `CustodyFailure` with no attempt, and I85's test checks preparation capacity 0.
- **The exception is R3P-2's in-loop check.**
- **One late capture per case:**
  - `finish` checks each slot's late hook, its single late capture and its source. Each late hook checks its case against `model.load_cases[parked.len()]` (S19 killed);
  - custody checks the count.
  - The parked-slot half of `finish`'s check is unpinned (S4; R3P-7).
- **My multi-case read** (`E/probe/multi_sp.jsonl`): W-C2-shaped requests in orders (A, B, C), (C, B, A), (B, A), (A, C) and (C, A, B), both modes. Every case has its own slot in request order, with its own captured source (A 3 loads, B 2, C 5) and counters (1, 1, 1, 1). Custody passes, and the ordinary owner is untouched.

### (c) Decision 19's binding

- **`bind_observations_by_case` keeps every check of `bind_observations`, per case:**
  - no prior error; completion and custody counters; the captured case id; the invocation mode; the parity presence;
  - one pass over the envelope rows. Each mode or parity row is bound to the case whose captured id its basis names; a row naming no requested case is refused;
  - per-case counts of exactly one final mode row and the parity rows its prefix made;
  - each row's fixed fields, value and text against the capture.
- **The qualified ids** (`index > 0` → `qualified_load_case_result_id`) match the envelope's rule (`lib.rs`: the first solved case unqualified; later cases qualified, except `_v2` rows under preview or exact pressure, which these kinds are not).
- **My orders put B, C and A each at index 0 in turn.** All pass custody, which shows the qualification is keyed to request position, not to a particular case.
- **Mutants killed:** S2 (no qualification), S17 (every row bound to case 0) and S18 (the one-case binder at c ≥ 2).
- **Unpinned:** the per-case presence check (S3; R3P-7).

### (d) T-7: failure isolation and snapshot points

- **Each attempt runs `prepare_active_case` on its own slot.** On failure, the stage fails, the error stays in the case's slot, and `trace.freeze` takes the terminal snapshot at preparation, with the case in the own fields. The later cases continue.
- **Attempt ids are the start order.**
- **Killed:** S8 (stop after a failure), S9 (no failure snapshot), S10 (error not kept), S11 (id = request index) and S12 (attempt on the wrong slot). A successful attempt takes no snapshot yet; T-8/T-9 must take it at their terminal stage, with `with_case(request)`.
- **For T-8,** a failed or `not_required` case still has a `source` (R3P-4).
- **`fail_preparation_of_case(i)`** is keyed on the request index and carried across the hop like the other faults.

### (e) The domain re-check

`w1_case_ids` gives `Domain` for:
- `load_cases` missing or not an array;
- c = 0;
- c > C;
- any combination;
- a non-string id.

`CaseSet::push` also refuses past C, so S16's twin (dropping `c > C`) is equivalent. The test covers 1, C, C + 1, none and a combination (S15 and S16 killed). `u3_each_stage_fault_falls_back_to_the_ordinary_bytes` keeps its C + 1 and combination arms.

### (f) What would make T-8 to T-11 harder or wrong

- **R3P-1:** the one-case continuation at c ≥ 2.
- **R3P-4:** select T-8's batch by `CaseAttempt::prepared`.
- **R3P-5:** T-12's detail placement.
- **`native` per slot.** `CaseSlot` holds `native: Option<(RecordedInvocation, RecordedCase)>` per case. T-8 needs one `RecordedInvocation` (60 G) for the call, with a `RecordedCase` per case. CHECKPOINT_R3P §5 already plans that split ("the invocation becomes shared"); I agree, and the custody check `parked…native.is_some()` should follow it.
- **Snapshots.** The invocation-level records (`observation_capacity_bytes`, `support_capacity_bytes`, `prepared_capacity_bytes`, the adapter counts) are now cumulative over cases, which is what DESIGN_v2 S-3's cumulative snapshots need. The per-case `capacities` and `work` strings live in the slots, so the serializer must read them per case (`with_case`).
- **For SA (already ruled, R3′ ruling 5):** `retained_error_text` reads only the own fields. `capture_bytes` reads the adapter's cumulative `RustCapacityBytes`, so it already includes the parked slots.

## 3. Evidence (`E/`)

### 3.1 The checkpoint's suite (control and guards)

- **PP registered, all targets, at `56c5579f07`:** 717 ok, 1 failed (`t13`), 11 ignored.
- **Against `98a77c716e`** (PP identical to I1's): only the 5 `b1_sp_*` tests are added (`diff_98a77c716e__56c5579f07.txt`).
- `s11f_site_test` 11/11 and the admission guard 5/5.

### 3.2 Bytes

`compare_i1_sp.txt` shows 112 identical rows. `compare_r1head_i1.txt` shows I1 equal to round 1's head.

### 3.3 Stage counts

`stage_i1.jsonl`, `stage_sp.jsonl` and `stage_compare.txt`: 104 c = 1 rows identical; the 8 multi-case fixture rows differ by design (§2 (a)).

### 3.4 Multi-case read

`multi_sp.jsonl`, 10 rows (§2 (b), (c)). In each, T-4 gives B `NotRequired` wherever B sits, A = the other cases in request order, and attempt ids are 0, 1, … in start order.

### 3.5 Mutants (`E/mutants/summary.md`)

Twenty, each one exact edit of `SP`'s `PP/lib.rs` or `retained_product.rs`, then PP's whole `--lib` suite, registered. The pristine control is §3.1's run.
- **Killed by assertions (14):** S1, S2, S5, S6, S8, S9, S10, S11, S12, S15, S16, S17, S18, S19.
- **Survived (6):** S3, S4, S7 (R3P-7); S13, S14, S20 (R3P-8).
- Every mutant compiled. `SP`'s sources were restored and checked against their pre-mutant sha256s (`sp_pristine_src.sha256` = `sp_restored_src.sha256`).

### 3.6 The interim one-case continuation (R3P-1)

`zz_rv109_interim_single_attempt`, on orders (A, B), (C, B), (B, A) and (B, C), both modes:

| Order (A) | Case in the capture's own fields after `into_single` | Native | Candidate | `retained_w1` on the driver, `LOAD_CASES` = 1 | `retained_w1`, reviewer-only `LOAD_CASES` = 3 (simulated I2) |
|---|---|---|---|---|---|
| (A, B), A = {0} | B: its unprepared captured source, 2 loads | `NativeUnavailable` | refused (`NativeUnavailable`) | `Domain`, exact bytes | **`Native`**, ordinary bytes + 1 notice, no successor |
| (C, B), A = {0} | B, 2 loads | `NativeUnavailable` | refused (`NativeUnavailable`) | `Domain`, exact bytes | **`Native`**, + 1 notice |
| (B, A), A = {1} | A (the attempted case; prepared, 3 loads) | ok | refused: "observation fixed fields" (the one-case binder at index 1) | `Domain`, exact bytes | **`Candidate`**, + 1 notice |
| (B, C), A = {1} | C (attempted; prepared, 5 loads) | `NativeUnavailable` (C's own outcome; alone it is Native at Ceiling) | refused | `Domain`, exact bytes | `Native`, + 1 notice |

The results are the same in both modes (`interim_c1.jsonl`, `interim_c3.jsonl`).
- The rows for (A, B) and (C, B) show the one-case continuation working on a case that was never attempted.
- The fallbacks hold only because native and the one-case binder refuse later.
- The bytes match what the interim fail-safe would have published, but the work and the stated cause are wrong.
- `LOAD_CASES` = 3 was set only in my copy, for this one run. The copy was then restored (`sp_restored2_src.sha256` equals the pre-edit hashes).

## 4. For ROOT

1. **R3P-1 and R3P-2 are for I85's continuing SP work,** before or with T-8. Both are small.
2. **R3P-3 to R3P-9** fold into SP's remaining tests, mutants and docs. R3P-4, R3P-5 and the `native` split are guidance for T-8 and T-12.
3. **No stop is indicated:** no c = 1 byte or event change, and nothing outside SP's fence.

## 5. Host and cleanup

- **Copies and targets:** `WT/rv109/{sp,i1}` and the targets `WT/targets/rv109-{sp,sp-probe,sp-mut,i1-probe}` were deleted at 17:34 UTC, after the last job ended. My scratch `WT/scratch/rv109_rvp_01/r3p/` is kept.
- **Commands** are in `E/tools/`:
  - `r3p_jobs.sh`: the suite, both probes, then the mutants;
  - `r3p_interim.sh`: the interim probe, unmodified and simulated-I2;
  - `sp_mutants.py`, the probe sources and shims, and the comparison tools.

## 6. Records

- `NOTES.md` (this file) and `SHA256SUMS`.
- **`evidence/`:**
  - `suites/`;
  - `probe/`: the probe source and shims, the outputs at I1 and the checkpoint, the stage counts, the multi-case read, the interim runs and the comparisons;
  - `mutants/`;
  - `diffs/`: the checkpoint's diff and its `-w` diff of `retained_product.rs`;
  - `tools/`;
  - `cargo_jobs_rv109_r3p.log`.
- Machine paths are replaced by placeholders.
