# I85 B1-SP checkpoint R3′: T-2, T-5, T-6 and T-7 of the n-case transaction

TASK (Type 2), I85, role I-P, for ROOT (HELP_HUMAN, Agent 0), who is the return path. I made no delegation. 2026-10-07 UTC. **This is checkpoint R3′,** taken after T-2, T-6 and T-7. SP is not finished, and RV-P (RV109) may start reading.

**Brief (verified before work):** `R/BRIEFS/B1_SP.md`, sha256 `85a5602792ea5fce7ede4efd8f5cdb26fadc07b6a3d0b83b8881a824a5fcb6e8`. B1_COMMON's rules hold.

**Read:**
- PLAN_v2 §1 (the fence and guards), §2.2 (SP), §4 and §8;
- DESIGN_v2 T-1 to T-13 and the outcome table;
- RV107's ADDENDUM_01 (`f3310d04…`): A1-S-2 and A1-N-1 to N-11;
- RR "RV109 confirms ST's repair; I1 (= I1′) at `262bd687f0`; phase 2 opens with SP and SA" (NUM `25c745f905`);
- I82's STUDY §3.3, for decision 19's c² sites.

**Placeholders:** as in RETURN.md (ST). `S` = `WT/scratch/i85_b1_st`. `I1` = `262bd687f0`, archived to `S/base_i1` (`P` without `P/execution`).

**Limits kept.**
- **Cargo:** 17 jobs, each through `WT/tools/t3_cargo.sh`, `--locked --offline`, with B1_COMMON's settings. They interleaved with I89's and I88's jobs; I killed no job (`_run_records/cargo_jobs_i85_sp.log`).
- **Waits:** one wait per job, each a bounded loop that ended by itself. No wait of mine is running.
- **Git:** one commit on `codex/piping-t3-b1-20261007`, not pushed; no merge.
- **Fence:** every change is inside SP's fence. That means `PP/lib.rs` (the retained section), `retained_product.rs`, `retained_wire.rs` (one line's argument), the facade and product tests, and `grant2.rs`.
- **Untouched:** FK, the schema, `Cargo.lock`, the reviewed statics, the readers, `retained_memory.rs` (SA's), and the visibility of any D1 item.

## 0. In brief

| Item | State |
|---|---|
| **T-2:** the capture holds each requested case | **Done.** One owner. Per-case slots hold each case's scope and custody counters, observations, and late capture (source, facts, operational records). The one-case scope checks are now per case |
| **T-5:** one notice per case in A | **Done** (`ReservedNotices`), with no heap allocation for the case set |
| **T-6:** invocation custody, once | **Done** (`prepared_custody`). For c ≥ 2, the observations of every case are bound in one pass over the envelope (`bind_observations_by_case`, decision 19) |
| **T-7:** one product attempt per case in A | **Done** (`prepare_cases`): request order, ids in start order, failure isolated to its case. Also done: `fail_preparation_of_case(index)` (decision 23, A1-N-6) and `bind_preparation`'s attempt index |
| Domain re-check (SF-2) | **Done** (`w1_case_ids`): c = 0, c > C and a combination give `Domain` |
| c = 1 byte identity | **Holds.** c = 1 now runs through the new T-4 to T-7 code. Every successor pin passes, and the pin tests' successor bytes equal the committed fixtures (§4) |
| PP suite against I1 | **+5 new tests, all ok;** nothing else differs, registered or Stale. The runner is identical, RE's carriers pass 17/17, and the witnesses are identical (§4) |
| Guards | s11f 11/11 and PP-tests' admission guard 5/5 pass with no expected-text change, and so do the in-fence guards. No new rule-8 site |
| Multi-case evidence before I2 | Below `retained_w1`, on W-C2: T-2, T-6 and T-7 in both modes, with per-case failure. Through `retained_w1`, c ≥ 2 is still `Domain` until I2 (A1-S-2) |
| Not yet | T-8 to T-13, W-C2 end to end, the fault tests, the coexistence pin, the base readers with several notices, A1-N-1's successor capture, and the mutants (§5) |
| Re-estimate | About **18–28 h** to finish SP. That brings SP to **about 20–30 h** in all, against the plan's 16–25 h (§6) |
| Split the serializer? | **No:** the three-implementer cap, and the coupling (§6) |

## 1. Head and commit

**Head: `56c5579f07b1cf3203c178e10add51bba0b24a58`**, one commit over I1 `262bd687f0`:

`piping(T3 B1 SP): T-2, T-5, T-6 and T-7 of the n-case transaction`

| File | Blob at I1 | Blob at head | sha256 at head | Lines |
|---|---|---|---|---|
| `PP/lib.rs` | `f26924d239` | `07d2f51192` | `a1c1da21487d8fb263441962e1b8019cf90a87382bf119d4270709e308fef508` | +114 / −35 |
| `PP/retained_product.rs` | `d43529ff2f` | `95d200e430` | `6272d21130c7f00c5708a7b84243cbd02bc2f9408fc16f9bc952610cfcd706c2` | +433 / −112 |
| `PP/retained_wire.rs` | `5bfc26473f` | `1b4bd42205` | `62cb287471581a885079197949eb508be96d00fb2c65719aee938ee99dd85b36` | +6 / −5 |
| `PP/retained_facade_tests.rs` | `8ac979fa78` | `707195ef89` | `61d22c124ce3eb6c3d498224ede0eaa96523e9d7bc83d3a086da625033048f9e` | +213 / −0 |
| `PP/retained_product_tests.rs` | `82591de9b7` | `5c8d1731a1` | `b79239154975f79c6623c1b1deae51bc6aeccae874aaec7e659af81fe29a88da` | +14 / −2 |
| `PP/retained_tests_hooks/grant2.rs` | `99c8240a3f` | `d59e111213` | `bc60fc01c0c5514194af2e20b71ab55d9c614cef3d50bc611d3cd2f6422a7422` | +16 / −2 |

**Reading the diff.** In `retained_product.rs`, most of the −112 lines are the preparation body moved, one indent level shallower, out of `prepare_owned_case`'s closure into `prepare_active_case`. `git diff -w` shows 375 / 69 lines for that file (`_run_records/sp_r3p.diff_w.stat`), and the diff itself is in `_run_records/sp_r3p.diff`.

## 2. What is done

### 2.1 T-2: one owner, per-case capture (`PP/retained_product.rs`)

**The slot.** The macro `per_case_capture!` declares `CaseSlot`, which holds every per-case field of `ProductCapture`, and a `swap_case` that exchanges them with the capture's own fields by moves only.
- **Per-case fields:** the scope and custody counters (`prepared_one_case_seen`, `prepared_late_calls`, `prepared_source_permit`, `source_capture_entries`, `case_calls`, `observation_calls`); the observations; the late capture (members, terms, supports, spring and fixed maps, facts, source, `case_id`, operational records); and the attempt's later state (error, verdicts, native, G5a, coverage and so on).
- **The invocation's fields stay in the one owner:** the adapter and its capacity arrays, the invocation, normalization, selections and basis record, the seeds, the permit and G-B's refusal, and `late_loads_total`.

**Parking.**
- A later requested case's early hook parks the earlier case's slot.
- The first park reserves room for c − 1 slots through the adapter: one `AllocationRequest`, with its `RustCapacityBytes`. Each park records one `MapWrite`.
- **At c = 1 nothing is parked, and no event is added.**

**Access to a case.** `with_case(i, f)` runs `f` with request case `i` in the capture's own fields, then puts every case back. For the last case seen it is just `f(self)`. Also added: `cases_seen()` and `parked_cases()`.

**Hooks.**
- The early hook (`prepared_case_seen`) and the late hook (`prepared_case_source`) check their case against `model.load_cases[parked.len()]`, not `[0]`, and accept c ≥ 1.
- The prepared probe's `capture_case_source` accepts several cases. The non-prepared legacy route still requires one case.
- `finish` checks every slot's late hook.
- At c = 1 the adapter events are identical: `i51_c0_isolated_guard_accounting_boundaries`, which counts every event offset of the early and late hooks, passes unchanged.

**Cumulative capacity.** `observation_capacity_bytes` is now summed over cases (`checked_add`). It is a "cumulative prefix" field of the T-11 snapshots, and at c = 1 it is the one value it was.

### 2.2 T-6: custody, once (`prepared_custody`)

**The checks, in order:**
1. A prior capture error of any case, the first in request order.
2. The adapter's fault.
3. The ordinary preconditions: one final hook, the preview contract, `MECHANICS_SOLVED`, no exact-block selection, and no native work in any slot.
4. **One complete late capture per requested case** (`cases_seen() == requested`).
5. Each case's observations bound to the envelope.

At c = 1 this is `prepare_owned_case`'s old prelude, with the same adapter events.

**For c ≥ 2,** `bind_observations_by_case` makes one pass over the envelope rows (decision 19), not one pass per case.
- **Each case keeps `bind_observations`' checks:** its completion and custody counters, exactly one final mode row, the parity rows its producing prefix made, and each row's value and text equal to what was captured.
- **A row is bound to the case** whose captured id its basis names; a row naming no requested case is refused.
- **Ids of the cases after the first.** The envelope qualifies their row ids with the case (`result:loadcase:<case>:…`; `lib.rs` `qualified_load_case_result_id`). `observation_fields_of(…, qualified)` expects that id, and charges the formatting as one `LibraryBoundary`, as `validate_modulus_record` does.
- **The c = 1 path** still calls the unchanged `bind_observations`.

### 2.3 T-7: per-case preparation (`prepare_cases`)

**The body.** `prepare_owned_case`'s preparation body is now `prepare_active_case(&mut trace, &mut AttemptParts)`. The one-case `prepare_case`, which tests use, and the n-case `prepare_cases` share it.

**`prepare_cases(ordinary, requested, attempted)`:**
- custody first (T-6);
- then, for each case in A in request order, one `CaseAttempt { request, attempt, prepared, parts, trace }` on its own slot (`with_case`);
- **attempt ids are the actual start order.**

**A failing preparation makes only its case unavailable.** Its trace's preparation stage fails, its terminal snapshot is taken then (`trace.freeze`), its error stays in its slot, and it has no prepared source (`source_ready` is false). The other cases continue.

**`PreparedCases::into_single`** gives today's `PreparedCase` or `PreparedCaseFailure` when there is exactly one attempt. That is how c = 1 continues on the one-case T-8 to T-13 for now.

**Test hook (decision 23, A1-N-6).** `grant2::fail_preparation_of_case(index)` takes the request index. It sits in `Armed`, `merge` and `names`, and is carried across the hop like the others. It is consumed at that case's attempt, so it works on the private driver and the actual entry alike.

**`bind_preparation(source, attempt, attempt_ref)`** now takes the attempt's index (T-7). The c = 1 serializer passes 0, which gives the same bytes.

### 2.4 `retained_w1` (`PP/lib.rs`)

**The order:**
1. Coexistence.
2. G-B's outcome.
3. **`w1_case_ids`,** which replaces `w1_case_id`: the request's cases, in request order, with `Domain` for c = 0, c > `caps::LOAD_CASES` or a combination. It keeps the line `let model = &capture.borrowed_raw()["model"];` that `u3_n9` pins.
4. **T-4:** ST's `.any(…Attempted)` check, kept verbatim so that R17 stays meaningful, then A computed by `CaseSet::attempted`.
5. **T-5:** `ReservedNotices::reserve`.
6. **T-6 and T-7:** `prepare_cases`.
7. **One attempt:** the one-case path from native on, unchanged.

**New types.** `CaseSet` and `ReservedNotices` are fixed arrays of `caps::LOAD_CASES`, with no heap allocation for the case set. They contain no rule-8 shape: counters use `checked_add` into a plain assignment.

**T-5 in detail:** every notice's id is checked against the base and against the other notices; one `try_reserve_exact(|A|)`; then each message. At c = 1 the order and the allocations are R-2's. Publishing appends the notices in request order, within capacity. The detail placement stays one-case for now (T-12 is still to do).

**Interim, until T-8.** Several attempts take a fail-safe fallback: the ordinary bytes, then every case's notice (`Native` if any attempt prepared, otherwise `Preparation`).
- This is reachable only after I2, since c ≥ 2 is `Domain` until then.
- It is fail-safe, and no test pins it.

## 3. The tests so far

**New, in `PP/retained_facade_tests.rs`.** The section "B1 SP (I85)" adds the builder `w_c2()`, three cases on U8's two-body model:
- `case-a`: A's three moments on body 0;
- `case-b`: B's tip force and torque on body 1;
- `case-c`: A's loads then B's. Their ids are suffixed `:c`, because primitive-load ids are unique across the model (validation's `DUPLICATE_ID`).

| Test | Pins |
|---|---|
| `b1_sp_t2_the_capture_holds_each_requested_case` | Both modes.<br>• The observed run equals the plain run and solves.<br>• No slot has an error; three slots, two of them parked.<br>• **Each slot is its request case:** its id; counters (1, 1, 1, 1); its own observations (id and mode); its own captured source's load count (3, 2, 5); both members' facts and operational records.<br>• One seed per case.<br>• The published verdicts are A Sensitive, B Passed and C Sensitive, as PROBE's one-case runs found, so **T-4's A is {A, C}** |
| `b1_sp_t6_t7_custody_once_then_one_attempt_per_case_in_a` | Both modes, with no fault, then with case 0's and then case 2's preparation failing.<br>• Attempts `[(request 0, id 0), (request 2, id 1)]`.<br>• **Per attempt:** `prepared`; the preparation stage Completed or Failed; `source_ready`; the terminal snapshot present only on failure; the helper's refusal; native not entered; the error in its own slot only on failure.<br>• **B has no attempt,** and keeps its captured source (2 loads).<br>• The ordinary owner is untouched, and the hook fires |
| `b1_sp_t6_custody_refuses_the_whole_invocation` | Dense mode.<br>• A requested case never seen ("prepared case count").<br>• A planted prior error in a parked slot or in the capture's own fields.<br>• Case B's captured observation tampered ("observation captured value/text").<br>• A mode row naming no requested case ("observation final case").<br>Each refuses **before any attempt** (no preparation capacity recorded) |
| `b1_sp_t5_one_reserved_notice_per_case_in_a` | For every count 1..=C: every slot reserved before W1, then published in request order with N1's exact bytes, and no allocation. Refusals: C + 1 cases, none, duplicate ids (once C ≥ 2), and a base already carrying the id (nothing changed). **Before I2 only count 1 runs;** after I2 the same test covers 1–3 |
| `b1_sp_domain_recheck_names_the_requested_cases` | `w1_case_ids`: one case; C cases (`case-0…`); `None` for C + 1, for no case, and for a combination |

**Restated, in `PP/retained_product_tests.rs`.** `i51_c0_isolated_late_hook_custody_and_prefixes`, case "multiple cases":
- the late capture of case 0 now succeeds, because each case's capture is its own;
- `finish` passes, because it does not know the count;
- `prepare_cases(e, 2, …)` then refuses at custody with "prepared case count", before any attempt.

This is PLAN_v2's "restated for n-case custody". No other existing test changed.

## 4. Evidence at the head (`_run_records/`)

| Run | I1 `262bd687f0` | Head `56c5579f07` | Difference |
|---|---|---|---|
| PP registered, all targets | 712 ok, 1 failed (`t13`, Mac), 11 ignored | 717 ok, 1 failed (`t13`), 11 ignored | **+5 new tests (ok):** the five `b1_sp_*`. Nothing else (`suites/diff_reg_I1__head.txt`) |
| PP Stale (`--cfg=i85_b1_st_stale`) | 712 / 1 / 11 | 717 / 1 / 11 | The same 5. Stale = registered, outcome for outcome, on both sides |
| runner/headless | 85 ok, 2 failed (the known `load_reference` pair) | the same | none |
| Witnesses, `--lib witness_ -- --ignored` | 10 passed | 10 passed | `I65_G5_WITNESS` lines identical: W2 Preparation, W6 Native on case C, the `NoTriggeredCase` pins, the rest as QUAL §4 |
| RE `retained_precision_carriers` | – | 17 passed | – |
| s11f and the admission guard | – | 11/11 and 5/5 (`guards/sp_guards01_pp.log`, and the full suite) | no expected-text change |

**Compiler warnings** are the same as at I1, apart from cargo's summary-line wording.

**c = 1 byte identity.**
- **The pin tests pass:** `u1_milestone_successor_both_modes`, `u3_permitted_path_…`, `u3g2_direct_entry_publishes_the_pinned_successor`, `u3g2_d_u6_5_…`, `u3_r1_…`, `u8_l0_…`, `u8_d_u6_5_l0_…`, and W2-deep on the witness stack. They now run through `w1_case_ids`, `ReservedNotices`, `prepared_custody` and `prepare_cases`.
- **The bytes the pin tests write** (`pins/pins.sha256`) equal the committed fixtures:
  - milestone: `ac6986b0…` and `6cd1d249…`;
  - L = 0: `93c6c865…` and `dbb3d477…`.
- **The receipts include the adapter counts,** so the c = 1 event sequence of the new code equals the old.

## 5. What remains in SP

| Step | Content |
|---|---|
| **T-8** | **One `CaseBatchCall`:** `solve_cases` over the prepared sources of A in request order, with one `RecordedInvocation` and the case limit per case.<br>• The invocation becomes shared, and each slot keeps its `RecordedCase`. About 19 uses of `native` change type: product, receipt projection and wire.<br>• Map the owner ordinal to the request index.<br>• A Run that is not selected gives `kernel_*` unavailable; a call failure before any Run goes to T-12 |
| **T-9** | Freeze per selected case: `freeze_candidate`'s proof on each case's slot, with that case's trace, against the one untouched ordinary owner. The snapshot is taken at each attempt's terminal stage |
| **T-10 and T-11** | **Staging:** one copy; the selected overlays in request order, then the unavailable diagnostics.<br>**The n-case serializer:**<br>• `cases[]` with `not_required`;<br>• `ordinary_attempts[]` for every requested case;<br>• `product_attempts[]` in start order;<br>• `sources[]` in registration order;<br>• one call, with its groups and builds;<br>• `execution_order`, `charged` and the ordinal mapping.<br>**Then:** abandonment on any serializer failure; precommit |
| **T-12 and T-13** | The detail only on the cases selected when the successor was abandoned; counting |
| **Hooks** | A1-N-1: a test-only successor capture, carried across the hop |
| **Tests** | **Before SR-RS:** W-C2 on the private driver (`Precommit G5 … ATTEMPT_MISMATCH`, with per-case outcomes read from the captured successor); the ordinal mapping; the multi-case fault tests (custody, preparation, call failure, staging, each serializer detail, precommit) with T-12's notice counts; the base readers with several notices, and the bytes written for SR-PY and SR-TS; the `retained_product_tests` restatements.<br>**After I2:** the n05 two-case coexistence pin (A1-N-7).<br>**After I3:** W-C2's successor pinned, with the fixtures |
| **Mutants** | Staging order, `attempt_ref`, the snapshot point, detail placement, reservation count, decision 5's set, the ordinal mapping, the domain re-check (c = C + 1; a combination) and R17 |

**c = 1 for T-8 to T-11.** I propose running c = 1 through the n-case T-8 to T-11 too, as T-4 to T-7 already do, and retiring the one-case path at the end.
- **Why:** c = 1 byte identity is then the strongest check of the n-case serializer, and there is one transaction to review.
- **The alternative** (keeping the one-case path for c = 1) holds byte identity by construction, but leaves the n-case serializer with no byte oracle until W-C2 is pinned after I3. I will take the single-path route unless ROOT rules otherwise.

## 6. Re-estimate, and the serializer

**Done so far:** about 2 h of agent time for T-2, T-5, T-6, T-7 and the domain re-check, plus about 1.5 h of waiting for the lock. The plan put R3′ at about half of SP; this part came in under that.

**Remaining:**

| Step | Estimate |
|---|---|
| T-8 | 3–5 h |
| T-9 | 3–5 h |
| T-10 and T-11, with c = 1 identity through them | 6–9 h |
| T-12, T-13 and A1-N-1 | 1–2 h |
| Tests (W-C2, faults, readers, restatements; after I2 coexistence; after I3 pins) | 4–6 h |
| Mutants, suites and records | 2–3 h |
| **Total** | **About 18–28 h,** plus repair rounds |

**SP in all: about 20–30 h,** against PLAN_v2's 16–25 h. The extra is mostly T-9 and the serializer: the one-case candidate and serializer own the capture, and have to be refactored to per-case owners within one capture.

**Should the serializer be split out (PLAN_v2 R3′, risk 12)? My recommendation is no.**
- **The cap.** Three implementers are working now (I85, I88, I89). A second serializer implementer would be a fourth.
- **The coupling.** T-11 depends on the shared-invocation and per-case frozen structures that T-8 and T-9 define, and c = 1 byte identity runs through all of them. Splitting them would cost more coordination than it saves.
- **When to revisit:** after T-9, if a slot is free.

## 7. For ROOT (and for SA, I89)

1. **No stop fired.** No FK, schema, reader or D1-visibility change; no c = 1 byte change.
2. **Accessors SA can use for multi-case G-B and G-C** (my files, read-only to SA):
   - `cases_seen()`, `parked_cases()` (each `CaseSlot` has `error`, `observable_error`, `observations`, `source` and the rest), and `with_case(i, f)`.
   - `retained_error_text` today reads only the capture's own `error` and `observable_error`. **For RetainedErrorTextBytes at c ≥ 2, SA needs the sum over `parked_cases()` too.**
   - The parked-slot reservation is adapter-accounted, so `capture_bytes` (the adapter's `RustCapacityBytes`) sees it.
   - The seam fields are unchanged.
3. **Interim behaviour after I2.** Until T-8, a c ≥ 2 Direct input that SA's D1.4 admits takes the interim fallback (§2.4): the exact ordinary bytes, then one N1 notice per case in A. It is fail-safe, and no test pins it.
4. **The design question in §5** (c = 1 through the n-case T-8 to T-11): my default is yes.
5. **For RV-P's ledger:**
   - one commit, `56c5579f07`, over I1;
   - read `retained_product.rs` with `git diff -w`;
   - the new tests are listed in §3.

## 8. Records

**`_run_records/`:**
- `sp_r3p.diff` (I1..head), its `--stat` and `-w --stat`, and `commits.txt`;
- `scripts/`: `sp_suites.sh`, `run_r3p.sh`, `cargo_cand.sh` and `sanitize.py`;
- `suites/`: the PP logs at I1 and head, registered and Stale; their outcomes, warnings and diffs; the runner logs filtered to cargo's Running, test, failure and result lines;
- `witness/`, `guards/` and `pins/`;
- `build/`: the build and lib runs while working;
- `run_r3p.out` and `cargo_jobs_i85_sp.log`.

Machine paths are replaced by `WT`, `VENV` and `~`; a line over 4,000 bytes is cut to 1,000 bytes, followed by its length and sha256. **SHA256SUMS** covers this file and every file under `_run_records/`.

**Kept in `S`** for the rest of SP: `base_i1`, the full logs, and the targets `WT/targets/i85-b1-st{,/base,/stale,/base-stale}`.
