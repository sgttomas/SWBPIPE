# I85 B1-SP return: the n-case transaction (DESIGN_v2 T-1 to T-13), before I3

TASK (Type 2), I85, role I-P, for ROOT (HELP_HUMAN, Agent 0), who is the return path. I made no delegation. 2026-10-07 UTC.

**This return is SP's end except one step.** ROOT asked me to return now, before I3, and to be resumed at I3 for the W-C2 pin (§7). RV-P round 2 can read SP from the head below.

**Brief:** `R/BRIEFS/B1_SP.md` (sha256 `85a56027…`, verified at SP's start), with B1_COMMON's rules. **Continued by:**
- RR "R3′: SP's first part verified; c = 1 through the n-case path; no serializer split; RV109 reads early" (NUM `a4c2cfa01b`);
- RR "I89's SA verified and ruled…", ruling 2: the seam saturates;
- RR "RV109's early read of SP: R3P-1 must land before I2; the notes folded into SP" (RV109's `R/REVIEW_RV109/rvp_r3p_read_01/NOTES.md`, sha256 `7d312cf0…`, which I read);
- RR "RV112 passes SA; G-B's byte bound corrected at SQ; I2 opens" (RV112 N-3 and N-4), and RR "I2: SA merged into b1 at eca6c00a72";
- ROOT's messages: the host rule (every heavy job under the lock); I2's timing; this return.

**Placeholders:** as in CHECKPOINT_R3P.md. `S` = `WT/scratch/i85_b1_st`. `I1` = `262bd687f0`; `I2` = `eca6c00a72`. `F` = this folder's `_run_records/final/`.

**Limits kept.**
- **Cargo:** every job through `WT/tools/t3_cargo.sh`, `--locked --offline`. No test binary ran outside cargo: the mutant loops call `t3_cargo.sh test`. My job lines since R3′ are in `F/cargo/cargo_jobs_i85_sp_final.log` (it also holds ROOT's I2 build check, which ran in `WT/b1`). The lock was shared with I88 to I93 and RV109 to RV114; I killed no job.
- **Waits:** one at a time, each bounded and ending by itself. No wait of mine is running.
- **Git:** commits on `codex/piping-t3-b1-20261007` only. No push and no merge; ROOT merged I2.
- **Fence:** every change is in SP's fence:
  - PP `lib.rs` (the retained section and its test hooks), `retained_product.rs`, `retained_receipt.rs`, `retained_wire.rs`;
  - the facade, product and wire tests, and `grant2.rs`.

  The two W-C2 fixtures in `P/fixtures/results/` come at I3. **Untouched:** FK, the schema, `Cargo.lock`, the reviewed statics, the readers, SA's `retained_memory*.rs`, the ordinary route, and the visibility of every D1 item.
- **Records:** absolute paths; no folder named `build`.

## 0. In brief

| Item | State |
|---|---|
| T-8: one `CaseBatchCall` | **Done.** One `RecordedInvocation` shared by the cases, and each slot keeps its own Run. The batch is the prepared attempts in request order (R3P-4). A call failure before any Run makes every submitted case `Native` |
| T-9: one freeze per selected Run | **Done.** Each runs on its case's own slot and scope, through `freeze_case`, which the one-case private driver shares |
| T-10, T-11 | **Done.** <br>• No case selected gives the furthest stage's cause.<br>• Otherwise one staging copy with each frozen overlay, then the n-case serializer and precommit.<br>• The serializer: `cases[]` with `not_required`; sources in registration order; ordinals mapped to request indices; `execution_order`; `charged` = the call's `invocation_after` |
| T-12, T-13 | **Done.** C1:68's detail goes only on cases selected when the successor was abandoned. One ordinary run; G-C once |
| c = 1 | **Runs through the n-case code; the one-case facade branch is retired.** Byte-identical: every pin passes, and the written bytes equal the committed fixtures |
| W-C2 (before I3) | **Holds.** Both modes: A `selected`, B `not_required`, C `unavailable` (`kernel_unresolved`, at Ceiling), read from the successor that precommit received. Precommit refuses at G5 (`ATTEMPT_MISMATCH`), as planned before SR-RS |
| Ordinal mapping | **{0, 2}** in Runs, `owner_refs`, `execution_order`, sources and product attempts |
| N-16 | **No difference** between the batch and the one-case runs (§3.3) |
| Coexistence, A1-N-7 | n05 with a second case **selects exact blocks in both modes**. Pinned: `Coexistence`, exact bytes, no notice, no reservation, `{runs: 1, complete_gates: 0}` |
| RV109's notes | **R3P-1 to R3P-9 are all addressed** (§4) |
| RV112's notes | **N-3 and N-4 are done** (§4) |
| The seam's saturation | **Pinned:** `CaseLoadsTotal` gives (u64::MAX, 384), and the seam records no capture error |
| Suites against I1 | Changes are only **+15 `b1_sp_*`** (mine) and **+8 `b1_sa_*`** (SA's). Stale = registered. The runner is identical. RE carriers 17/17; witnesses 10/10 as at R3′ |
| Guards | s11f 11/11; admission guard 5/5. **One expected-text change,** with its reason (§5). No new rule-8 site |
| Mutants | **33 runs of 29 mutants;** every SP mutant, R17 and RV109's six are killed. One mutant is equivalent and was replaced (§6) |
| I3 | **Front-run** against SR-RS at `b5cb7faaeb`: W-C2 passes precommit. The patch, the pins and the fixtures are ready (§7). One expected change |
| **For ROOT** | **The summary headlines at c ≥ 2 (§8).** I conclude the accepted base readers determine the rule. Your ruling with RV-P |

## 1. Head and commits

**Head: `603e238517`**, `WT/b1`, clean. The commits since R3′ (`56c5579f07`), in order:

| Commit | Content |
|---|---|
| `7458527ff7` | T-8 to T-12; c = 1 through them; the one-case facade branch retired; the seam saturates |
| `89222942ee` | T-13 and the coexistence tests. `before_native` fires only when a case was prepared |
| `59393e2d43` | Corrects the seam's comment |
| `0ec3651a36` | RV109's R3P-1, R3P-2, R3P-3, R3P-6 and R3P-7 |
| `8db7906ee7` | R3P-9: one custody prelude |
| `c17340d50b` | Pins S20's cumulative capacity. **I2 merged here** |
| `eca6c00a72` | ROOT's I2 merge (SA, `b1-a` at `9812c83ded`) |
| `603e238517` | Post-I2 pins; RV112 N-3 and N-4; the seam's saturation pin |

`F/commits.txt` lists every commit from I1. **Diffs:**
- `F/diffs/sp_rest_56c5579f07__603e238517.diff`: SP's fence files only, so I2's SA files are left out;
- its `--stat` (+1,602 / −356) and `-w --stat` (+1,596 / −350);
- SP's whole change from I1 (+2,336 / −450 in 8 files);
- `i2_merge_brought.stat`.

In `retained_product.rs`, most of the moved lines are the freeze body: `freeze_candidate`'s closure became `ProductCapture::freeze_case`, with `self.capture`/`self.trace` rewritten to `self`/`trace`. Read it with `-w`.

## 2. The transaction (T-8 to T-13)

### 2.1 Owners

- **The capture holds the invocation's one `RecordedInvocation`** (`native_invocation`), and each case's slot holds its own `RecordedCase` (`native`). Both are read through `native_pair()`, and the one-case API is unchanged in use.
- **Each slot also keeps its row block** (`scope_rows`), which custody binds for c ≥ 2.
- **`CaptureError` derives `Clone`:** a call failure's cause is copied onto each submitted case's trace.
- **`CaseAttempt` gains:**
  - its proof flag and overlay work;
  - its end: `Preparation`, `Prepared`, `Native`, `Selected`, `Frozen(FrozenCase)` or `Candidate(RefusedCase)`.

### 2.2 T-6 custody additions (c ≥ 2)

**`bind_case_rows`:** each requested case's rows form one block of the envelope, in request order. It makes one pass, with at most two comparisons per row, and stores each block in its slot.

**Also, from RV109's notes:**
- A is validated before any attempt (R3P-2);
- prior errors are taken in request order (R3P-3).

### 2.3 T-8

`PreparedCases::native` enters each prepared attempt's native stage. It then calls `native_call` over the requests of the attempts with `end = Prepared`, in request order (R3P-4: never by a slot's `source`).

**Inside `native_call`:**
- duplicate and source-presence checks;
- `OriginCapacity::for_calls(&[n], &[])`;
- `RecordedInvocation::new(60e9, …)`;
- `solve_cases` with the case limit 20e9.

**One submitted source is borrowed in place,** with no event, as before. **Several** move into one adapter-reserved vector for the call (one `AllocationRequest` and its `RustCapacityBytes`, plus 2n `MapWrite`), and back afterwards.

**Each Run:**
- one ending `selected` completes native;
- any other fails native with `NativeUnavailable` and takes the attempt's terminal snapshot then;
- a failure of the call itself fails every submitted attempt with that cause.

**The one-case `solve_native`** runs the same `native_call` on its one case.

### 2.4 T-9

`PreparedCases::freeze` runs, for each `Selected` attempt in request order, inside `with_case(request)`. It calls `ProductCapture::freeze_case(trace, overlay_work, proof_attempted, ordinary, &scope, native)`, which is the former `freeze_candidate` body.

**`CaseScope`** gives a case's rows, its `preview_cases[]` evidence index and count, and whether its ids are case-qualified. `WHOLE` is the one-case scope.

**The scope reaches every envelope read in the freeze:**
- `bind_rows_view`;
- observation binding (`bind_observations_in`);
- `validate_final_metadata`: the case-qualified id after the first case, except `_v2` kinds, as `lib.rs` qualifies them;
- `prepared_maxima`, the aliases and `observables_view`.

At c = 1 the scope is `WHOLE`, so the reads and events are unchanged.

### 2.5 T-10 to T-12 (`lib.rs`)

**`retained_w1` keeps the gates and T-5.** It then calls `w1_transaction`, which runs T-6 to T-11 and returns either the successor or (cause, selected attempts). `ReservedNotices::publish(ordinary, cause, selected)` puts C1:68's detail only on notices whose bit is set: the cases in A that were frozen when the successor was abandoned (decision 6; R3P-5).

**T-10's cause with no case selected** is the furthest stage reached: `Candidate`, then `Native` (including a call failure), then `Preparation`. At c = 1 these are the one-case causes.

### 2.6 T-11 staging and the serializer

**Staging** (`PreparedCases::staged_envelope`):
- one copy of the ordinary owner;
- each frozen case's overlay on its own block and evidence case, in request order;
- for c ≥ 2, the summary headlines over the staged rows (`stage_headlines`; §8).

**The serializer** (`retained_wire::serialize_cases`):
- **Envelope:** the identity once; then each selected case's method token, omitted legacy disclosure and selected diagnostic, in request order; then each unavailable case's diagnostic.
- **`cases[]`**, in request order: `selected`, `unavailable` or `not_required`. `not_required` is checked against the published verdict `checks_passed`.
- `ordinary_attempts[]`, one per requested case. D6a's references are collected in one pass over the diagnostics (I82's c² guidance).
- `product_attempts[]`, in start order; `sources[]`, by the kernel's source id.
- One material basis, with `case_indices` = every requested case.
- **Runs and calls name request indices,** through the ordinal map.
- **`execution_order`** follows `inv.runs()`.
- **`charged`** is checked against the last Run's and the call's `invocation_after`.

The one-case `serialize_selected` and `serialize_unavailable` share the helpers, unchanged in bytes. `serialize_frozen` is removed.

### 2.7 Hooks

- **`before_native` and `before_staging`** now take `PreparedCases`. The withdraw fault fires only when a case was prepared, as before T-8.
- **A1-N-1:** `before_precommit` hands a copy of the successor to `grant2::counted_with_successor`, carried across the hop with the tally.

## 3. Evidence for each acceptance item

### 3.1 c = 1 byte identity

- PP registered, all targets, at `603e238517`: every pre-existing test passes (§3.10).
- **The pins at `603e238517`** (`F/pins/pins_603e238517.sha256`) equal the committed fixtures, under `u3_permitted_path_…`, `u3g2_direct_entry_…` and `u8_l0_…`:
  - milestone `ac6986b0…` and `6cd1d249…`;
  - L = 0 `93c6c865…` and `dbb3d477…`.
- **Receipts include each attempt's adapter counts,** so the c = 1 event sequence through the new code is the old one.

### 3.2 W-C2 before I3, and the ordinal mapping

`b1_sp_w_c2_transaction_outcomes_and_ordinal_mapping`, both modes, through `w1_transaction`:
- **The outcome:** `Precommit { G5, ATTEMPT_MISMATCH }`, with A selected (bit 0) and the ordinary owner untouched.
- **The captured successor:**
  - `cases` = selected, not_required (four keys, `product_attempt_ref` null), unavailable;
  - C: `{kernel_unresolved, kernel, prepared_product_failure, attempt 1}`, its Run terminal `{unresolved, {space: unresolved, tag: ceiling}}`;
  - sources (0, case 0, case-a, attempt 0) and (1, case 2, case-c, attempt 1);
  - product attempts (0, case 0) and (1, case 2);
  - **`owner_refs` and `execution_order` = [case 0, case 2]**;
  - one material basis with `case_indices` [0, 1, 2]; one group with sources [0, 1]; `charged` = the call's `invocation_after`;
  - **the record point:** C's snapshot (at its Run) is at or below A's (at A's last proof stage) in every count, and strictly below in `RowVisit`;
  - every snapshot's observation capacity sums the three case ids (S20);
  - **the staging order:** A's selected diagnostic, then C's unavailable diagnostic, last; the method token on A's rows only;
  - the headlines (§8).

**Through `retained_w1` and Direct since I2** (`b1_sp_w_c2_through_retained_w1_publishes_t12`, `b1_sp_w_c2_direct_entry_counts_one_run_through_g_c`): the ordinary bytes, then case-a's and case-c's plain N1 notices.

### 3.3 N-16

The batch Runs equal the one-case runs, in both modes:
- **A:** (selected, 2 physical records, precision 128);
- **C:** (`unresolved Ceiling`, 4 records).

There is no finding.

### 3.4 Multi-case coexistence (A1-N-7)

`b1_sp_multi_case_coexistence_pin` uses n05 plus `case-2` (load id `torque:2`). Exact blocks are selected in both modes, **not** T-3 (b). Both paths are pinned:
- **the private driver:** `Coexistence`, the plain bytes, and the diagnostics' capacity unchanged (no reservation);
- **Direct (registered, admitted since I2):** `Coexistence`, `{runs: 1, complete_gates: 0}`, the plain bytes and no notice.

### 3.5 Fault tests and T-12's notice counts

**`b1_sp_w_c2_transaction_faults_and_abandonment`** (cause, selected bits):

| Fault | Result |
|---|---|
| T-6 custody | (`Preparation`, 0) |
| T-7 on C | A's successor reaches precommit with C unavailable at preparation, no `CaseSource`, Run or references, and one call over A |
| T-7 on A | (`Native`, 0) |
| T-8 call failure | (`Native`, 0) |
| Staging | (`Staging`, 0b01) |
| Each serializer check, including every C1:68 detail | (`Serializer`, 0b01) |
| Precommit | (G1, 0b01) |
| **R-b′'s limit:** B, not attempted, demoted | `Serializer(Untranslated, …recovery_finding)`, 0b01 |

**Through `retained_w1` since I2,** with both notices and `with_notice`'s exact bytes:
- a C1:68 serializer refusal puts the detail on case-a's notice only;
- one without a detail, or a call failure, leaves both notices plain.

**R3P-1 (A, B):** one notice.

**T-5:** `b1_sp_t5_…` covers counts 1 to 3 since I2.

### 3.6 T-12's base readers with several notices

`u3_r2_base_readers_accept_the_unavailable_notice` adds W-C2: case-a's and case-c's N1 notices, plain or with the detail on case-a's only. The readers accept them, with the same contract and standing (`needs_recompute`).

**The bytes for SR-PY and SR-TS** are in `F/t12_bytes/` (base, noticed plain, noticed receipt, both modes, and the request). They are reproduced at `603e238517`, identical (`F/pins/`).

### 3.7 T-13, empty hooks, Stale

`b1_sp_w_c2_direct_entry_…`:
- **registered:** `ONE_RUN_THROUGH_G_C`, with no hook armed before or after;
- **Stale:** the plain bytes and `ONE_RUN`.

Every fault test also asserts that its armed faults fired.

### 3.8 `retained_product_tests`' prepared-scope cases

- At R3′: "multiple cases".
- **R3P-9:** `prepare_case` now runs `prepare_cases(…, 1, &[0])`, so every one-case custody fault test exercises production's custody.

### 3.9 The seam (RR ruling 2 on SA) and RV112 N-4

- **`b1_sp_seam_overflow_is_a_typed_g_b_refusal`** (registered): a permitted probe preset to `usize::MAX` gives G-B's (`CaseLoadsTotal`, u64::MAX, 384), with the run untouched. The capture's only error is `finish`'s report of the skipped late capture, as after any G-B refusal.
- **`b1_sp_first_g_b_refusal_stands_and_stops_later_late_hooks`:** with G-B's fault at case A:
  - the running total is 3, A's loads only;
  - only A's late hook ran, and no case has a late capture;
  - Direct gives `LateGate`, exact bytes and `ONE_RUN`.

### 3.10 Suites at `603e238517` against I1 (`F/suites_i2/`)

| Suite | I1 | `603e238517` |
|---|---|---|
| PP registered, all targets | 712 ok, 1 failed (t13), 11 ignored | **735 ok, 1 failed (t13), 11 ignored.** Only +15 `b1_sp_*` and +8 `b1_sa_*` (`diff_reg_I1__603e238517.txt`) |
| PP Stale | | **Identical to registered**, outcome for outcome |
| runner/headless | 85 ok, 2 failed (`load_reference`) | **Identical** |
| RE `retained_precision_carriers` | | 17/17 |
| Witnesses | 10/10 | **10/10, the same outcomes** as R3′ (`F/witness_i2/`) |

**Compiler warnings** are the same as at I1.

## 4. RV109's and RV112's notes

| Note | Disposition |
|---|---|
| R3P-1 | `into_single` and the one-case facade branch are retired (`7458527ff7`). **Pinned:** (A, B) and (B, A) with A = one case run on that case's own slot and source; (C, B) ends `Native` on C's own Ceiling Run. Through `retained_w1`: one notice |
| R3P-2 | Custody refuses A out of range or out of order before any attempt: [0, 3], [3], [2, 0], [0, 0] and [2, 2] |
| R3P-3 | Prior errors are taken in request order (parked first); pinned with errors in cases 0 and 2 |
| R3P-4 | T-8's batch is the attempts with `end = Prepared` |
| R3P-5 | The detail is placed by the selected bits |
| R3P-6 | `fail_next_preparation`'s and `fail_preparation_of_case`'s docs name the case |
| R3P-7 | Three custody tests: a missing or doubled final mode row; a parked case's incomplete late capture (`finish`); native in a parked slot. Each refuses before any attempt |
| R3P-8 | S13, S14 and S20 were re-run after I2 and T-11: **killed** (§6) |
| R3P-9 | One custody prelude: `prepare_case` → `prepare_cases(…, 1, &[0])` |
| RV112 N-3 | `permitted_run`'s comment is current |
| RV112 N-4 | After a G-B refusal, a later late hook returns at once: the first refusal stands, and no later G-B check, running total or late capture runs. Pinned (§3.9) |

## 5. Guards

**Outside `src`:** s11f 11/11 and PP-tests' admission guard 5/5. Rule 8's table is unchanged: no new site in `lib.rs` uses a rule-8 shape (counters use `checked_*`).

**Inside `src`:**
- **`u3_n9_single_parse_custody`** has one expected text changed: the required call `"retained_wire::serialize_frozen(&frozen, &staged, capture)"` is now `"retained_wire::serialize_cases(&mut prepared, &staged, capture)"`. **The reason,** stated in the test: T-11's n-case serializer replaces the one-case one, and the call still takes the parse's borrowed half. The test's other assertions are unchanged, including `borrowed_raw()` = 2, the call counts and the forbidden list.
- `u3_permitted_outputs_keep_the_report_and_gate_order`, `u3_capture_permit_is_linear`, `u1_serializer_reads_no_legacy_work_field` and `u3g2_no_permit_path_runs_once_without_a_copy` pass unchanged.

## 6. Mutants (`F/mutants/`)

Three batches. Each mutant is one or more exact edits of an archive copy, then PP's whole `--lib` suite, registered, through `t3_cargo.sh`. The sources were restored and checked after every run.

| Batch | Revision | Mutants | Result |
|---|---|---|---|
| `mutants_pre_i2` | `59393e2d43` | M1–M5, R10, R16 (ST's, moved to SP's code); R17; S1–S8b (22) | 19 killed. **Before I2, as expected:** R17 and S4 survive (C = 1). **S8a is equivalent:** `CaseSet::push` also refuses past C, as RV109 found for its S16 twin |
| `mutants_i2` | `603e238517` | R17, S4, S5, S6a, and RV109's S3, S4, S7, S13, S14, S20 | **10 of 10 killed** |
| `mutants_i2b` | `603e238517` | S8a2 (c = C + 1 admitted by attempting only the first C cases) | **Killed** |

**The SP list** (PLAN_v2 §2.2), each killed:

| Item | Mutant |
|---|---|
| Staging order | S1 |
| `attempt_ref` | S2 |
| The snapshot record point | S3 |
| Detail placement | S4 |
| Reservation count | S5 and RV S13 |
| Decision 5's set | S6a custody; S6b call failure; S6c no case selected; S6d staging; S6e serializer; S6f precommit |
| The ordinal mapping | S7 |
| Domain c = C + 1 | S8a2 |
| Domain, one combination | S8b |
| RV109 N-2 | R17 (`.all` for `.any`) |

Each mutant's failing tests are in its `mutants.json`.

## 7. I3: the front-run and the remaining step

**The run** (`F/preview_i3/`): a scratch copy of `603e238517` with SR-RS's reader files at `b1-r`'s `b5cb7faaeb` (`retained_precision.rs`, `source_blocks.rs`, and the contract test).
- **W-C2 passes precommit** in both modes, and so do (A, B) and (B, A).
- **With the post-I3 test patch** (`F/scripts/post_i3.py`) and the two fixtures, PP `--lib` gives 573 ok and 1 failed (t13).

**The expected pins** (`F/preview_i3/wc2_pins.json`):

| Mode | Document sha256 | Receipt sha256 | Published bytes sha256 |
|---|---|---|---|
| sparse | `7922e3e5278d0d87dc5faf79dfbc1f2a384899e97df306cc742355cdacdb6269` | `cccb9664e1c58f0582348df3348d8b6e0b0941bcb0294a5b351a4d092ed18886` | `c7a1859330e9e36e18f5572838dbad72ea251a817241acc6968f8f83b70170fa` |
| dense | `f2800bd4f2b4c90217918a6e1287f98305a1b6b07893295b5790f387c075d3a3` | `612e23ca4b90604b3d2351fd7465d2e3cefb0f3fbb36bdea39efaa82a68bc07a` | `a77c010b4ae7ffa9c535c31305b8a91fcc3c05e8a512075fa5b4694dbae3062c` |

**The one expected change: the T-7-on-C fault test ends at `Precommit { G8, PREPARATION_MISMATCH }`, not G5.**
- **Why:** the hook zeroes C's diameter fact. C's product attempt truthfully records that tampered fact in `preparation.members[].old_facts`. SR-RS's G8 recomputes the fact from the request and refuses: `facts[0] == bits(d)`, `RS:3983` at `b5cb7faaeb` (3987 in the traced copy, which carries 4 lines of my tracing), traced in `F/preview_i3/g8_trace.txt`.
- **This is a correct refusal of an injected fault, not a producer defect.** C's per-case outcomes are still read from the captured successor. A T-7 test that publishes a successor would need a real input whose preparation fails beside a selected case; I know of none.

**The patch also:**
- turns W-C2, (A, B), (B, A), the `retained_w1` arm and Direct into successor pins;
- adds `b1_sp_w_c2_fixtures_are_the_live_successors` (D-U6-5);
- asserts that the successor's one `RETAINED_PRECISION_UNAVAILABLE` diagnostic is C's receipt-backed one, not an N1 notice.

**At I3 I will:**
1. apply `post_i3.py` with `wc2_pins.json`;
2. add `P/fixtures/results/retained_precision_w_c2_successor_{sparse_interactive,dense_scrutiny}.json`, written by the Direct test (`I85_WC2_OUT`);
3. run the suites, the guards, a mutant re-run on the pin and the pins;
4. append the result to this return.

## 8. For ROOT: the summary headlines at c ≥ 2

**The question.** At c ≥ 2 the successor's `summary.max_displacement` and `summary.max_open_formula_stress` are across cases. Which row do they name after the selected cases' overlays?

**What the texts say:**
- **DESIGN_v2 T-11's staging** names "values, the `recovery_method` token, the maxima patches" for each selected case, then the unavailable cases' diagnostics (DESIGN_v2 §1.2, T-11). It does not mention the summary. Nor do T-1 to T-13 elsewhere, nor PLAN_v2 §2.2.
- **C1 says nothing about how a headline is chosen.** It says only how a headline binds: "absolute and not_covered rows/headlines refuse binding" (C1:160).
- **D2 §4.9.10** (`T/DESIGN_STANDING/DESIGN.md:711`): "A summary or headline whose `result_ref` names a withheld row is refused as that row is."
- **I31's B2 return** recorded the ordinary rule: the producer "selects across cases using actual values, then case id, then location id" (`R/I31/f2a_certificate_b2/RETURN.md:244–246`). It added: "Preserve those exact finite-value/tie rules".
- **At c = 1** the frozen overlay sets the summary to the case's own aliases: `apply_prepared_overlay`, I51's "two new summary aliases" (`R/I51/prepared_producer_completion_02/API.md:234`). That is the same rule within one case.

**What the accepted readers require.** This is what settles the choice.
- **The base preview-physics reader** checks that each headline is exactly the governing row of its kind over all load-case rows: the greatest value, ties to the smaller case id, then the smaller location. A mismatch is `HEADLINE_BINDING`.
  - RS: `RE/src/preview_physics_evidence.rs`, `headline` at 775–802 and `governs` at 174–180, called at 639–651.
  - PY: `preview_physics_evidence.py:304–316`, `min(…, key=(-value, case id, entity_ref))`.
  - TS: `previewPhysicsEvidence.ts:308–314`.
- **The retained readers run that base check on every successor.** G7 validates the successor's projection with the unchanged base branch: `semantic_contract.rs:437–443`, and `retained_precision.rs:4305`, `for_source(&projected)`.
- **The ordinary producer** applies the same order: `lib.rs` `maximum_across_cases`, 3805–3842 at `603e238517`.

**My rule (`stage_headlines`).** After the overlays, for c ≥ 2, each headline is recomputed over the staged rows:
- the row of its kind with the greatest value;
- a tie goes to the smaller case id, then to the smaller location;
- only where the ordinary envelope has the headline (a headline covers the whole requested domain).

The value, unit, location and `result_ref` are the row's own. This is the readers' governing row, so G7's headline check holds. Each frozen case's aliases are its own rows' maximum under the same order, so the rule equals "the maximum of the per-case aliases".

**The alternatives:**
1. **Keep the ordinary headlines.** Whenever the ordinary winner is a selected case's row whose value the overlay changed, the headline names a row with another value, or no longer the governing row. G7 then refuses with `HEADLINE_BINDING`, and every such successor is abandoned. **W-C2 is such a case.**
2. **Set the headlines to null at c ≥ 2.** The base reader then refuses with `HEADLINE_PRESENCE`, because rows of the kind exist.
3. **Per-case aliases, then the maximum across cases.** This is the same as my rule.

So the readers leave no other choice that publishes. The open question is only whether DESIGN's T-11 text should say so.

**What changes in W-C2** (the ordinary bytes against the successor, `F/t12_bytes/` and `F/preview_i3/`):
- **Displacement:** unchanged. It stays case-b's row: `result:loadcase:case-b:disp:node-section-b`, 282.942… mm. B is `not_required`, so the row is unchanged.
- **Stress:** the row changes; the value does not.
  - **Ordinary:** `result:elastic-maximum:6:case-a:2:M1`, 2.5172202259989904e-08 Pa sparse (4.619475839070268e-08 dense).
  - **Successor:** `result:elastic-maximum:6:case-c:2:M1`, with the same value.
  - **Why it moves:** in the ordinary run, A's and C's M1 rows tie at that value (body 1's floating-point noise; C carries A's body-0 loads plus B's), and case-a wins on case id. A's overlay makes its M1 row 8.170865639376687e-92. C's row, unchanged because C is unavailable, then governs.
- **Readers and carriers that read a headline:**
  - the three base readers (above), reached by the retained readers through G7;
  - rule binding: `semantic_contract.rs` `rule_binding_refusal`, and `retained_binding_refusal`, which binds a headline as its `result_ref` row (D2 §4.9.10);
  - RE's `retained_precision_carriers.rs:579–580`, which reads each headline's `result_ref`;
  - the desktop's evidence views (`previewPhysicsEvidence.ts`).

  No reader checks a headline against a receipt field.

**The ruling I propose:** adopt the base readers' governing-row rule as T-11's staging of the summary headlines. Do it by an erratum to DESIGN_v2 T-11, or a line in B1's change record. RV-P round 2 can check `stage_headlines` against `governs`.

## 9. Other notes for ROOT

1. **No stop fired.** No FK, schema, reader, reviewed-input or D1-visibility change; no c = 1 byte change. N-16 shows no difference.
2. **I82's third c² item** (the integrity diagnostic loop) is `assessed_numerical_quality` in the ordinary route. That is outside SP's fence, so it is unchanged. Its c² term stays as G5 measures it.
3. **New adapter events at c ≥ 2, for SQ's pricing:**
   - custody's row-block pass;
   - T-8's source vector;
   - the qualified-id formatting.

   None of them occurs at c = 1.
4. **Memory (DESIGN_v2 N-10):** T-9 keeps each frozen case's payload and certificate live until staging. The snapshot record point follows S-3 (C before A in W-C2).
5. **The T-7-on-C G8 refusal** after I3 (§7) is expected. 07n (SC) should not use a tampered-preparation successor as a must-pass base.
6. **The custody failure's private trace** no longer records the prior-cause `Option<CaptureError>` cost that the old one-case prelude added. The trace costs are not receipt bytes; only `I51_PRIOR_CAUSE`'s print shows them.

## 10. Records

- **This file, and `SHA256SUMS.return`,** which covers it and every file under `_run_records/final/`. CHECKPOINT_R3P.md and its SHA256SUMS are unchanged.
- **`_run_records/final/`:**
  - `commits.txt`, `diffs/` and `cargo/`;
  - `suites_i2/`: outcomes, warnings, filtered logs and diffs at `603e238517`;
  - `witness_i2/`, `pins/` and `t12_bytes/`;
  - `mutants/`: three batches, each with `mutants.json`, `run.out` and the per-mutant filtered logs;
  - `preview_i3/`: the SR-RS preview logs, the G8 trace, the W-C2 documents' hashes and `wc2_pins.json`;
  - `scripts/`: `mutants_sp.py` (and the version run on `59393e2d43`), `sp_suites.sh`, `post_i3.py`, `cargo_cand.sh` and `sanitize.py`.

Machine paths are replaced by `WT`, `VENV` and `~`. A line over 4,000 bytes is cut to 1,000 bytes, followed by its length and sha256.

**Kept in `S`:** `sp02/` (the logs, archives `mut`, `mut2` and `preview_i3`, and the W-C2 documents in `wc2_out`) and the targets `WT/targets/i85-b1-st{,-mut,-mut2,-preview}`. The I3 step uses them.
