# RV85: independent review of U3 grants 1b (`4b31bbf23a`) and 1c (`886bef131a`)

**Verdict: PASS for both grants.** 0 BLOCKING, 1 SHOULD-FIX, 7 NOTE.

**As the originator, I confirm all six of my grant-1 findings are closed:**
- **S1:** the permit is linear.
- **S2:** carried hooks fire on the reserved-stack thread, so my hazard demonstration no longer reproduces.
- **S3:** every permitted output keeps its report.
- **N1:** G-C runs only after exact selection and G-B.
- **N6:** the staging overlay falls back typed, with the notice.
- **N7:** the single-parse guard is stronger.

Each fix leaves a small residual: the NOTEs below.

**No published byte changes without a permit:**
- **My 550-row sweep** is byte-identical on `4b31bbf23a`, on `886bef131a`, on grant 1 (`bee3dc07ca`) and on `b54caba7ab`. All four give sha256 `577764c7…e00d`.
- **R-1's carrier** names `Ordinary` with the plain bytes on all 196 retained calls, and never a successor.
- **The suites' failure sets equal grant 1's.**

**The one SHOULD-FIX (T1):** the notice's message reservation is correct, but no test pins it. The publish-time check on the message can never fail, and two mutants that remove or shrink the reservation survive the whole PP `--lib`.

**Run facts.**
- **Role:** TASK Type 2, dispatched directly by ROOT (HELP_HUMAN), with no descendants. ROOT added grant 1c to this review mid-task.
- **Basis:**
  - NUM `fc5d92c56c` (RR "RV85 on U3 grant 1: PASS, merged; …");
  - `R/I61/u3_facade_02/RETURN.md` and `R/I61/u3_facade_03/RETURN.md`;
  - my grant-1 report, `R/REVIEW_RV85/u3_facade_01/REVIEW.md`.
- **Time:** 2026-10-04, 06:50Z to 07:37Z.
- **Host:**
  - The memory guard (PID 5387) was checked before every Cargo job.
  - Default toolchain; `--locked --offline`; `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`; one Cargo job at a time.
  - No install, no new tooling, and no solver-at-scale, native or DEC-025 job. Python ran the repo's own stdlib-only reader.
- **Git:** reads only, with `GIT_OPTIONAL_LOCKS=0`.
- **Copies:**
  - `git archive` of P (excluding `execution/`) for each tree: `bee3dc07ca` (base), `4b31bbf23a` (cand, mut, stub) and `886bef131a` (c1c, mut1c, stub1c), all in WT/rv85/. I deleted them afterwards.
  - Targets went to WT/targets/rv85/{g1b,g1c}, and scratch to WT/scratch/rv85_u3_facade_02.
- **Changed-file hashes:** all eight hashes (four files per grant) match I61's `changed_files_sha256.txt` for 1b and 1c.
- **Two disclosures:**
  1. **My disposable sweep and carrier tests sat in the 1c tree during the 1c PP suite run.** They failed there for lack of their environment variables. I excluded them from the outcomes (`evidence/suite_1c_disposable_rv85_tests_excluded.txt`) and ran them separately, where they pass.
  2. **One hash-matching command wrote a 321-byte scratch file to the system temp directory** by mistake. It contained only hash fragments from I61's record. I deleted it at once.

**Placeholders:** WT, NUM, P, PP and R as in the brief. Line numbers are at `886bef131a` unless marked. `evidence/` is this folder's evidence subfolder.

## Findings

| # | Sev | Where | Evidence | Remedy |
|---|---|---|---|---|
| T1 | SHOULD-FIX | lib.rs:3014, :3041, :3058–3060, :3064; retained_facade_tests.rs:194–199 | **R-2's message reservation is not pinned.** By reading, the reservation is correct: `reserve` reserves 196 bytes (135 + 35 + 25 + 1, the longest variant), so `publish` cannot allocate. But the publish-time check `notice.message.len() <= notice.message.capacity()` (:3064) is always true for a `String`. `u3_r2_notice_bytes_are_pinned` compares the longest token to the literal 25, not to `RECEIPT_ENCODING_DETAIL_MAX`. **Mutants W01 (`RECEIPT_ENCODING_DETAIL_MAX = 0`) and W02 (message reservation removed) survive the whole PP `--lib`,** on 1b and unchanged in 1c. A regression would allocate after W1 work ran, and no test would notice. (The diagnostics-slot half is pinned: B06 is killed.) | Before a permit exists (grant 2, or a 1d), in the `#[cfg(test)]` check, assert that the message capacity is unchanged across the pushes, or that the required length fits before pushing. Pin `RECEIPT_ENCODING_DETAIL_MAX` to the maximum of `receipt_encoding_detail`'s tokens in a test that names the constant. |
| U1 | NOTE | retained_memory.rs:302; retained_facade_tests.rs:398–419 | **S1's residual.** The permit is linear, but it is not bound to its invocation. It travels with the same parse's halves through one call site, which N7's counts now pin, and that is sufficient while there is one site. Linearity itself is a text check over three files. | Optional for G5 (I65): a lifetime or digest binding. A dependency-free compile-time "not `Copy`/`Clone`" assertion could replace the text check. |
| U2 | NOTE | lib.rs:2961–2968, :3179–3188 | **S2's residual: unfired faults vanish with the worker.** `carry_test_hooks` takes the caller's armed faults and never returns the unconsumed ones. Stub row G shows this: a precommit fault armed before a G-B refusal never fires, yet the caller then shows nothing armed. So across the hop, "none left armed" holds whether or not a fault fired. The other test seams (`DISPATCH_COUNT`, `WORK_TRACE`, `ACTIVE`) are not carried; ROOT already routed them to grant 2. | **Grant 2:** hand the worker's leftover `Armed` back to the caller after the join, or assert consumption inside the worker. Keep asserting the cause and `into_publication()`. |
| U3 | NOTE | I61 `_run_records/r2_*`; evidence/py_check_1{b,c}.txt | **The R-2 negative controls are real but narrow.** The only refusal the base readers apply to a diagnostic is a dangling `result:` reference. On the product's own notice bytes, my Python run also accepts a notice naming a non-existent case, and a duplicated notice id. So the readers accept the notice; they do not validate it. **R-2's condition (accepted, standing unchanged) is established.** The notice's own shape is pinned only by PP's byte tests. | None. A recorded limit. |
| U4 | NOTE | lib.rs:3095–3098 vs :2995–2997 | **N2 is re-targeted by N1's fix.** The permitted path now checks G-B's outcome in `permitted_run` (:2995), so `retained_w1`'s own check (V07) is unreachable from it. My stub mutant SV07 survives as equivalent, and my SV18, which removes the `permitted_run` check, is killed. | Grant 2's N2 test must pin the `permitted_run` check, and that G-C is not consulted. V07 no longer needs to be pinned. |
| U5 | NOTE | retained_product.rs:3769–3783, :3448 | **N6's residual.** Only one of the five typed overlay sites (`pipe_stress_extrema[]`) is exercised. Q01 (the `values` site) and Q02 (the key site) survive. `PreparedCase::into_ordinary`'s `expect` stays as an invariant, and I agree: `ordinary` is `Some` until `freeze_candidate` consumes `self`. | Optional: one fault per site. |
| U6 | NOTE | retained_facade_tests.rs:316–345 | **N7's residual.** Q03 survives: a request re-derived by a helper defined *outside* the guarded section, with the raw value read there. That is inherent to a text guard. The structural argument holds by reading: one parse, `CapturedInvocation` is not `Clone`, and the permitted path only moves or borrows. | None, or optional: type the custody so the typed request cannot be built except by `parse`. |
| U7 | NOTE | lib.rs:3020–3025 | **F-1 is faithful to C1:68's text** (C1:68: unencodable or saturation-ambiguous runs carry the three work-counter details; publication-hash failures "similarly" carry `publication_hash_range`). The successor schema's `receipt_failure.check` enum is wider: it also has `encoding` and `association` (schema :6507–6514). So the base notice's `receipt_encoding` reason covers fewer cases than the successor's own `receipt_failure` vocabulary. ROOT accepted F-1, and no byte changes until a permit exists. | None. |

## 1. S1 and S2 on `4b31bbf23a`

### S1: is the permit linear?

**Yes.**
- **No derive and no impl:** `CapturePermit` (retained_memory.rs:300 at 1b, :302 at 1c) has no derive. No `impl Clone` or `impl Copy` exists in PP.
- **The facade's path:**
  1. It reads `reserved_stack_bytes()`.
  2. It moves the permit into a `move` closure, with the report as well in 1c (`report` is `Copy`).
  3. The closure moves it onto the worker and into `permitted_run`, then into the observer through `permitted_probe(permit)`.
- **Uses after that are borrows only:** G-B borrows `self.permit.as_ref()` (retained_product.rs:3244), and G-C borrows `observer.permit()`.
- **On a spawn failure** the permit drops, unrun, with the closure.
- **Nothing reads a moved permit,** and the type system now rejects any reuse. B20 (`Copy` added back) is killed, though only by the text test (U1).
- **Residual (U1):** the permit is not bound to its invocation.

### S2: do the carried hooks make the hazard impossible?

**Yes, for the W1 faults. I reran my grant-1 hazard demonstration** through my own disposable stub; it is process-global, written independently of I61's, and archive-only (decision 7). See `evidence/stub_control_1b.out` and `stub_control_1c.out`, rows F.

| Fault armed on the caller | Grant 1 (`bee3dc07ca`) | 1b and 1c |
|---|---|---|
| withdraw native | published SUCCESSOR; the flag leaked to the next caller-thread call | `Native` |
| corrupt precommit | same | `Precommit G1` |
| rebind precommit | same | `Precommit G8` |
| serializer encoding (new hook) | — | `Serializer` |

In 1b and 1c, every row also shows:
- the publication is `Ordinary` with plain bytes plus the notice;
- the caller is left with nothing armed;
- the next same-thread `retained_w1` gives SUCCESSOR, so nothing leaked.

**Without the carry it fails again.** My SV15 (dispatch without `carry_test_hooks`) brings the grant-1 behaviour back, and the stub kills it in both grants.

**The dense-ceiling override** is copied, not taken. W10 (not installed on the worker) is killed.

**Residual (U2):** an unfired fault is silently discarded with the worker.

## 2. R-1, the public carrier

**Added, additive only:**
- `pub enum RetainedPublication { Ordinary(MechanicsEnvelope), Successor(Value) }` (lib.rs:2187);
- `successor()` (:2235);
- `into_publication()` (:2243).

**No existing public signature changed.** `envelope()`, `admission()` and `into_parts()` are as before. The diff adds items and removes nothing public.

**Without a permit:**
- **By reading,** `retained` is `None` on every path: the imposed-displacement refusal, every refused or shared `ordinary_dispatch(…, None)`, and a parse error, which returns `Err`.
- **My carrier test** (`evidence/zz_rv85_carrier.rs`) covers 49 inputs × 2 modes × Direct and Headless, 196 calls on each grant (`carrier_1{b,c}.tsv`). On every call:
  - `successor()` is `None`;
  - `into_publication()` is `Ordinary`;
  - its bytes equal `envelope()` and the plain value-route bytes.

**Under my stub (rows A and B):**
- success gives `Successor`, carrying U1's pinned file bytes (`ac6986b0…`, `6cd1d249…`) in both modes;
- `successor()` equals the publication, and `envelope()` stays plain;
- every fallback gives `Ordinary`.

My W08 (the `Successor` variant carries the ordinary bytes) and W09 (`successor()` on a fallback) are killed.

## 3. R-2, the notice

**Only where W1 work ran.** `notice.publish` is called on exactly the Preparation, Native, Candidate, Serializer and Precommit fallbacks, plus 1c's Staging (W1 work ran there too).
- **Every no-W1 cause returns before the reservation, or never reaches it:** Coexistence, LateGate, D1.4 Domain, NoticeReservation, CompleteGate, PermitUnbound, StackReservation and the Domain guard. The success path drops the unused notice.
- **Under my stub (row B: 49 inputs × 2 modes × 2 entries):**
  - **0 mismatches against my own independent expectation:** plain bytes, plus my own notice JSON exactly where the cause ran W1 work;
  - **12 notices,** on Preparation and Candidate fallbacks;
  - **none on the 128 Domain, 52 Coexistence and 4 Successor calls.**
- **Rows C, D, E and H** (LateGate, CompleteGate, StackReservation, Coexistence) give exact plain bytes.
- My SV16 (a notice on CompleteGate) is killed.

**The reservation is infallible after W1 starts, by reading:**
- `reserve` runs before `prepare_case` (:3103). It reserves one diagnostics slot in the ordinary owner (:3039) and 196 message bytes (:3041), and builds every string up front.
- The same `Vec` reaches `publish` on every fallback through moves only: `failure.ordinary`, `into_ordinary()` or `refusal.ordinary`.
- `publish` only pushes within the reserved capacity, so it does not allocate.
- **The slot half is tested** (`shrink_to_fit` plus the capacity check; B06 is killed). **The message half is not (T1).**

**The text is fixed** (:3010). It is pinned against a literal, and W04, W05 and W06 are killed.

**F-1 is faithful to C1:68** (U7).

**The base-reader acceptance evidence is convincing for the condition:**
- **result_export (Rust):** I61's committed test, plus my stub's 12 real product-published notices. On each, `for_source`, `standing_reason` and `numerical_use_standing` (with and without the invocation) equal the plain envelope's: `numerically_eligible` on `precision_connected_ui`, `needs_recompute` on `rejected_stress_range`.
- **Python:** I61's probe, plus my own run on 6 real product notices (`evidence/py_check_1{b,c}.txt`). It accepts them with standing unchanged.
- **runner/headless and the TypeScript desktop admission:** I reviewed I61's probes by reading; I did not re-run them.
  - The runner probe is a copy of the dispatch. It is asserted equal to the maintained route on the unchanged envelope, and it mints and validates an export document that carries the notice.
  - Running the TS probe would need a link to REPO_ROOT's `node_modules`, and vitest's cache would write outside my fence.
- **The negative cases are real but narrow (U3).**

## 4. No published byte change without a permit

| Check | Grant 1 `bee3dc07ca` | 1b `4b31bbf23a` | 1c `886bef131a` |
|---|---|---|---|
| My sweep: 55 inputs × 5 routes × 2 modes, admission report hashed | `577764c7…` | `577764c7…`, identical | `577764c7…`, identical |
| PP lib + integration | 654 ok, 1 F, 1 ign | 659 ok, 1 F, 1 ign | 660 ok, 1 F, 1 ign |
| runner/headless | 85 ok, 2 F | identical | identical |
| result_export | 149 ok | identical | identical |
| PP production warnings | 8 | same set | same set |

- **PP's differences are exactly the added tests,** all ok: 1b adds 5, and 1c adds `u3_permitted_outputs_keep_the_report_and_gate_order`.
- **The only failures are t13 and the two load_reference tests.**
- **The sweep equals `b54caba7ab`** as well (`evidence/sweep_sha256.txt`).

## 5. Grant 1c: S3, N1, N6 and N7, confirmed as their originator

**S3.**
- `admit` returns `Result<(CapturePermit, RetainedAdmissionReport), RetainedAdmissionReport>` (retained_memory.rs:380), with `admission(report).map(|permit| (permit, report))` (:403). The report is `Copy`.
- The report reaches every permitted output: StackReservation (on the caller), Domain, and every `permitted_run` output (:3005).
- **Stub:** `admission()` is `Some` on success, on LateGate, CompleteGate, StackReservation, exact selection and Staging, and on all 196 calls in row B.
- My SV20 (the StackReservation report dropped) and SV21 (the W1 output's report dropped) are killed, as are I61's C01–C03.

**N1.**
- `permitted_run` (:2993–3004) now checks, in order: exact selection (Coexistence), G-B's outcome (LateGate), then G-C. That is COMPOSITION §2's order.
- **Stub:**
  - a G-B refusal never consults G-C (row C, `complete_calls=0`);
  - a source-block fixture (`source_blocks/n05-sparse_interactive`) gives `Coexistence` with plain bytes and no G-C call (row H).
- My SV18 and SV19, which remove either check, are killed: G-C gets consulted. I61's C04 is killed.

**N6.**
- `apply_prepared_overlay` (retained_product.rs:3769) returns `Result<(), StagingFault>`, with five typed sites.
- `staged_envelope` returns `Result`. On a fault, the facade returns `W1Fallback::Staging` with the untouched owner and the notice (lib.rs:3127).
- `commit_private` keeps an `expect`, which is acceptable: it is only on the private driver, a test-only path.
- **Stub row I:** the carried staging fault on the actual entry gives `Staging("pipe_stress_extrema[]")` and plain bytes plus the notice.
- I61's C06–C09 are killed. **Residual: U5.**

**N7.**
- The guard now forbids `from_str(`, `from_slice(`, `from_reader(`, `deserialize(`, `Deserialize` and the struct literal.
- It counts exactly two `borrowed_raw()` reads, and pins the call counts of `permitted_dispatch(`, `permitted_run(`, `retained_w1(` and `carry_test_hooks(`, with the dispatch's call in the permit arm.
- I61's C10–C12 are killed. **Residual: U6.**

## 6. Mutants (`evidence/mutants_*`, `mutant_defs_1{b,c}.py`, `stub_summary_1{b,c}.txt`)

**The setup.**
- Fresh `git archive` trees; t13 is excluded with `--skip`; every file is restored and checked by sha256.
- **Controls:**
  - 1b: `--lib` 497 ok; I61's filter 42 ok.
  - 1c: `--lib` 498 ok; I61's filter 43 ok.

| Set | Run | Result |
|---|---|---|
| I61's 1b set (B01–B20 and 10 re-expressed X), committed tree, its own filter, verbatim | 30 | **30 killed**, no compile-error kill |
| I61's 1c set (C01–C04, C06–C12), committed tree, its own filter, verbatim | 11 | **11 killed** |
| My grant-1 survivors, re-run on 1b | 3 | V02 survived again (pre-existing). V12 survived again (equivalent). V07 survived (permit-only; in 1c it becomes equivalent on the permitted path: U4) |
| My own on the notice and carrier (1b, whole `--lib`): W01–W10 | 10 | 8 killed. **W01 and W02 survived (T1)** |
| My own on 1c (whole `--lib`): Q01–Q03 | 3 | **All survived** (U5, U6) |
| My stub mutants, 1b: SV07, SV13–SV17 | 6 | 5 killed. SV17 (`PermitUnbound` proceeds) survived, equivalent, as I61's Y09. SV07 is killed on 1b |
| My stub mutants, 1c: SV07, SV13, SV15, SV16, SV18–SV21 | 8 | 7 killed. SV07 survived, equivalent (U4) |

The I61 sets were extracted verbatim from I61's `mutants.py`, its "mut" tree only. I did not re-run its "mutstub" mutants. My own stub covers those branches.

## For ROOT

**Nothing needs a ruling before 1b and 1c merge.**

**T1 (SHOULD-FIX):** a test-only fix, to land before a permit exists (grant 2 or a 1d).

**For the grant-2 brief:**
- **U2:** hand back unconsumed faults after the hop.
- **U4:** N2's test now targets `permitted_run`'s G-B check.

**For I65 G5, optional:** U1's invocation binding.

**No action:** U3, U5, U6 and U7.
