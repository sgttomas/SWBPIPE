# RV85: independent review of U3 grant 1 (facade capture, permit branch unreachable)

**Verdict: PASS.** 0 BLOCKING, 3 SHOULD-FIX, 7 NOTE.

No published byte changes while no permit exists, and nothing is weakened. The three SHOULD-FIX findings are not defects in this candidate's reachable behaviour. Each must be settled before U4 G5 or U3 grant 2 makes a permit constructible:
- **S1:** the permit is `Copy`.
- **S2:** the thread-local test seams do not cross the reserved-stack hop.
- **S3:** a permitted invocation loses its admission report.

**What I verified:**
- **The permit branch is statically unreachable.** `RegisteredProfile` is still an empty enum. No `CapturePermit` literal, permit constructor or `unsafe` code exists in PP.
- **`admit` is the old `assess` body.** The only change is that it returns `admission(report)`, which is always `Err`, and `assess` wraps it.
- **My own sweep is byte-identical to base.** It ran 55 inputs × 5 routes × 2 modes, giving 550 rows, with the admission report's Debug hashed.
- **The suites match base.** PP, runner/headless and result_export have identical failure sets, and PP adds exactly the 6 new tests.
- **The lock deltas are exactly R-3.**
- **Mutants:** all 25 of I61's committed-tree mutants are killed. RV82's R08b is killed on the whole PP `--lib`. 9 of my 12 are killed, and the three survivors are explained.
- **The permitted path, exercised in my own disposable stub** (decision 7: archive only; written independently of I61's):
  - the actual Direct entry publishes U1's pinned successor bytes in both modes;
  - all 49 fixture inputs keep their value-route bytes through the permitted Direct and Headless entries, with no panic;
  - the G-B, G-C and stack fallbacks behave as designed.

**Run facts.**
- **Role:** TASK Type 2, dispatched directly by ROOT (HELP_HUMAN), with no descendants.
- **Candidate:** `bee3dc07ca`, the branch head, against base `b54caba7ab`.
- **Records:** NUM `efde9ca2d1`.
- **Time:** 2026-10-04, 06:13Z to 06:48Z.
- **Host:**
  - The memory guard (PID 5387) was checked before every Cargo job.
  - Default toolchain; `--locked --offline`; `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`; one Cargo job at a time.
  - No install, no new tooling, and no solver-at-scale, native or DEC-025 job.
- **Git:** reads only, with `GIT_OPTIONAL_LOCKS=0`.
- **Copies:** `git archive` of P (excluding `execution/`) into WT/rv85/{cand,base,mut,stub}. I deleted them afterwards. Targets went to WT/targets/rv85 and scratch to WT/scratch/rv85_u3_facade_01.
- **Changed-file hashes:** all seven PP files match I61's `changed_files_sha256.txt` (`evidence/candidate_files_sha256.txt`).

**Placeholders:** WT, NUM, P, PP and R as in the brief. Line numbers are at `bee3dc07ca`. `evidence/` is this folder's evidence subfolder.

## Findings

| # | Sev | Where | Evidence | Remedy |
|---|---|---|---|---|
| S1 | SHOULD-FIX | PP/src/retained_memory.rs:300; lib.rs:2894, 2933, 2938; retained_product.rs:145, 3244 | **`CapturePermit` derives `Clone, Copy`.** A permit is G-A's admission of one invocation's census. COMPOSITION §2 calls the gates "successive reservations of the same actual invocation"; §4 says "one start capability exists per actual case/attempt". With `Copy` and a `&'static` field, any crate code can duplicate a permit, keep it after the invocation and hand it to `permitted_dispatch` or `permitted_probe` with another request. Nothing in the types prevents reuse. `Copy` is only a convenience: `permitted_run` uses `permit` after moving it into the observer (:2933 → :2938), and the late hook copies it out (:3244). | Before G5 constructs any permit. Under D-5, I65 owns the derive and I61 the call sites. Drop `Clone`/`Copy`, and have `permitted_dispatch` consume the permit. The observer owns it (`permitted_probe(permit)`), G-C is called through the observer, and the late hook borrows `&self.permit`. Bind the permit to its invocation as well: either a lifetime on the `CapturedInvocation`, or the capture digest, checked once in `permitted_dispatch`. |
| S2 | SHOULD-FIX | lib.rs:3003–3030 (hooks), :2967, :2985, :2991; retained_memory.rs:407; source_receipt/composite.rs:1169; lib.rs:3204 | **Test seams are `thread_local!` and do not cross the reserved-stack hop.** Grant 1's committed tests are **not** vacuous: they call `retained_w1` on the test thread and assert the exact `W1Fallback`, and X03 and X07–X09 are killed. Through the actual permitted entry, though, my stub shows the following (`evidence/stub_control.out`, rows F). With each of the three hooks set on the caller, the invocation published the **SUCCESSOR**. `envelope()` still equalled the plain bytes, and the unconsumed flag fired on the **next** same-thread `retained_w1` call (Native, G1, G8). The same applies to `DISPATCH_COUNT`, composite's `WORK_TRACE` and the dense-ceiling override. Also, `permitted_run` never calls `ordinary_dispatch_entered()`. | **For grant 2:** (a) Carry fault injection across the hop explicitly: snapshot the caller's pending seams in `permitted_dispatch` and install them at the top of the worker closure, or pass a `#[cfg(test)]` fault plan into `permitted_run`. Count the single observed run there too. (b) Every fault test asserts the hook was consumed, the exact `W1Fallback`, and the publication variant (R-1 `into_publication()`). Never assert `envelope()` bytes alone: on success it is the same untouched ordinary base. (c) Add one canary test, in which a caller-set hook fires through the actual entry, and a positive control in the same harness. |
| S3 | SHOULD-FIX | lib.rs:2899, 2930, 2942 (`admission: None`); doc lib.rs:2221; runner/headless lib.rs:755 | **A permitted invocation loses its G-A report.** `admit` returns only the permit (API.md §2), so every permitted output has `admission() == None`. That includes the StackReservation, Domain, G-B and G-C fallbacks; my stub shows it in rows A and E. The public doc says "None means the existing pre-parse refusal returned before census entry", and runner/headless says "did not enter admission". DOMAIN §3 expects the private report to keep every fact. | ROOT routes this. Either API.md is amended so that `admit` yields the report with the permit, and every permitted output carries `Some(report)` (I65 G5 and I61 grant 1b/2), or the documented meaning changes under U6's carrier review (R-1). |
| N1 | NOTE | lib.rs:2938–2941 vs :2955–2961 | **G-C is consulted after G-B has refused, and after exact selection.** Stub row C shows `complete_calls_after_late_refusal=1`. There is no byte effect: all three cases are "no W1 work", which means exact ordinary bytes under R-2. But COMPOSITION §2 makes G-C the successor of G-B's reservation ("Only here may R_complete replace R_late"), and a G-C refusal then masks the true cause (LateGate or Coexistence). | Check coexistence and the G-B outcome before `check_complete`, or document the order. |
| N2 | NOTE | lib.rs:2958–2961 | **Mutant V07 (G-B outcome check removed) survives the committed tests,** because it is permit-only. My stub kills it (SV07): the cause becomes `Preparation`. That matters once grant 1b lands: under R-2 N1, a Preparation fallback appends the notice, so dropping this check would change a permitted invocation's published bytes. | Grant 2 commits the G-B, G-C and stack tests, as planned. Include this one explicitly. |
| N3 | NOTE | lib.rs:2289–2292 | **Mutant V02 survives the whole PP suite.** It drops `LOAD_REFERENCE_SOURCE_SEMANTIC_CONTRACT_ID` from `source_finalization_failed`. The predicate is base's inline check moved verbatim, and base's tests are a subset of the candidate's, so the gap pre-exists: it is not a regression. | Optional test. |
| N4 | NOTE | retained_product.rs:3750–3757 | **Mutant V12 is equivalent.** It replaces F-4's `None` arm with `unreachable!`. Every `Ok(payload)` (:3745) follows `certificate=Some(certified)` (:3732), so the arm is unreachable, and the F-4 unwrap removal is correct. | None. |
| N5 | NOTE | retained_product.rs:3788–3792; retained_wire.rs:1524; lib.rs:2976–2992 | **The staging profile, for G4.** During serialization, three envelope-sized owners coexist: the untouched ordinary owner, the staged clone and the successor `Value` built by `to_value(overlaid)`. The receipt is added on top. `staged` drops at :2978, before precommit, and precommit adds the invocation deep copy (F-7) and the reader's temporaries. COMPOSITION §3 counts any base-projection clone as another token. Nothing yet meters staging, serialization or precommit: there is no PublicationWork owner, and API.md's `budgets()` is not in the shim. | Price it in G4. Grant 2 meets `budgets()`. Optionally, overlay onto the `Value` to save one full tree. |
| N6 | NOTE | retained_product.rs:3766–3777 (`apply_prepared_overlay`); :3446 (`into_ordinary`) | **The staging overlay has moved onto the production facade path,** and it carries unwrap, expect and index sites; `into_ordinary` carries an `expect`. These are safe by construction: the patch indices and keys come from the same envelope via `prepared_maxima`, and `ordinary` stays `Some` until `freeze_candidate` consumes `self`. But a broken invariant would panic on the reserved thread and be re-raised, losing the ordinary publication instead of falling back. | Optional: make staging fallible and map a failure to a W1 fallback. |
| N7 | NOTE | retained_facade_tests.rs:170–188; tests/retained_precision_admission.rs:199–216 | **The single-parse custody guard is textual.** It forbids `parse(` and `from_value(`, but not other re-derivations from `borrowed_raw()` (`from_str(`, `deserialize(`), and it does not count the `permitted_dispatch(` call sites. The structural argument holds by reading (§4). F-2's relocation leaves the legacy call-graph check true but blind to `permitted_dispatch`. | Optional: widen the forbidden list, and assert exactly one `permitted_dispatch(` call, in the `Some(Ok(permit))` arm. |

## 1. No published byte change while no permit exists

**Unreachability, by reading.**
- `enum RegisteredProfile {}` is unchanged (retained_memory.rs:299). `CapturePermit` holds `&'static RegisteredProfile`, so no safe code can build one.
- There is no `CapturePermit {` literal and no `unsafe` or `transmute` in PP.
- `admission()` still returns `Err(report)` (:350–354).
- `admit` (:374–402) is byte-for-byte the old `assess` body (base :312–343), with the tail `match admission(report) {Err(r)=>r, Ok(p)=>match *p._profile {}}` replaced by `admission(report)`. The new `assess` re-wraps exactly that tail.
- In the dispatch, `Some(Err(report)) => Some(report)` reproduces base's `retained_entry.map(assess)`.
- `ordinary_dispatch` (lib.rs:2266–2287) is base's code moved verbatim: the budget, `ordinary_dispatch_entered`, one captured run, and the finalization check (now named `source_finalization_failed`, with the same three ids).

**My sweep** (`evidence/zz_rv85_sweep.rs`, `list.txt`, `sweep_{cand,base}.tsv`). It is an integration test over the public API, identical in both archives.
- **Inputs:** 55 in total.
  - 44 request fixtures, which is every request-shaped JSON under P/{fixtures,validation,examples,tests}. That includes the 14 `validation/qualification` load-reference requests, which I61's sweep did not cover.
  - 5 models wrapped as `{model}`.
  - 6 synthetic inputs: an imposed-displacement refusal, a 70-deep unknown field (census `DepthLimit`), an unparseable request, a relabelled request, a two-case milestone, and one with no request materials.
- **Routes:** typed default, typed mode, value, retained Direct and retained Headless (with runner-shaped roots), each in both modes.
- **Hashed:** envelope bytes or the error text, plus the sha256 of the admission report's Debug and its `census_complete`/`unknown` flags.
- **Result:** 550 rows, **byte-identical** to base (sha256 `577764c7…e00d` for both).
- **Coverage of the 220 retained rows:**
  - 212 admission reports: 208 census-complete and 4 `DepthLimit`;
  - 4 pre-parse `None` (the imposed-displacement refusal);
  - 4 parse errors.
  
  Across all routes there are 14 typed-parse or value errors, and no panics.

**The suites** (`evidence/suite_*.outcomes`):

| Suite | Base `b54caba7ab` | Candidate `bee3dc07ca` |
|---|---|---|
| PP lib + 21 integration targets | 648 ok, 1 F, 1 ign | 654 ok, 1 F, 1 ign |
| runner/headless | 85 ok, 2 F | 85 ok, 2 F |
| result_export | 149 ok | 149 ok |

- **PP's difference is exactly the 6 added tests,** all ok: the five `retained_facade_tests` and `u1g2_run_conservation_pins_each_check_at_its_call_site`.
- **The failures are exactly t13 and the two load_reference tests,** the same as base. runner/headless and result_export are identical outcome by outcome.
- **Production build warnings:** PP lib has 8 warnings, the same set as base. `result_export`'s pre-existing `derived` warning now also appears in PP's runtime graph (F-3).

## 2. The permitted path's structure (by reading, then my stub)

**`on_reserved_stack`** (lib.rs:2907–2918).
- **Spawn failure:** the scoped spawn returns `Err`, and the closure, which holds only `&mut slot`, is dropped unrun. `slot` is still `Some(request)`, so `permitted_dispatch` runs `ordinary_dispatch` on the caller's thread with the same request and `&capture`. **The request is not lost.**
- **Panic:** `join` gives `Err(payload)`, and `resume_unwind(payload)` runs inside `scope`. The panic is joined explicitly, so `a_thread_panicked` stays clear, and `scope` re-raises the original payload. The committed test checks this, and X12 is killed.
- **The `(None, None)` arm** is unreachable: `slot` is `Some` until the closure runs.
- **The stack size is honoured:** X11 is killed by a stack-overflow abort.

**`permitted_run`** (lib.rs:2921–2943), in order:
1. the D1.3 domain guard, to `ordinary_dispatch`;
2. **exactly one** `run_linear_static_preview_observed` with the permit-bound observer;
3. the finalization check (same as the ordinary route);
4. G-C;
5. `retained_w1`.

Equivalence with the ordinary route for the admitted family:
- Only load-state models take the SF-1 rerun in `run_linear_static_preview_captured`, and only exact pressure raises the budget. The guard routes both to `ordinary_dispatch`.
- Every other model gets `SourceRecoveryBudget::default()` and the same single observed run as `captured_once`.
- `prepare_owned_case` itself refuses unless `final_calls == 1`.

**`retained_w1`** (lib.rs:2950–2998).
- **The ordinary envelope is never mutated.** Every fallback returns the original owner:
  - `ordinary`;
  - `failure.ordinary`, which `prepare_owned_case` only borrows;
  - `prepared.into_ordinary()`, where `solve_native` never touches it;
  - `refusal.ordinary`, which `freeze_candidate` only borrows;
  - `frozen.into_ordinary()`, for Serializer and Precommit.
- **No fallible step follows the transfer's first move.** After `validate` returns `Ok`, the function only drops the invocation and moves the frozen owner and the successor `Value`. Back in `permitted_run` and the join, everything is a move.
- **Precommit validation uses the actual invocation:** `{request: capture.borrowed_raw(), solver_mode: capture.mode()}`. That is the same raw `Value` and mode that `parse` digested.
- **A reader refusal becomes the ordinary fallback** (`Precommit{gate, code}`), as do the G1 and G8 committed faults. X07, X08, X09 and my V06 (the mode left out of the invocation) are all killed.
- **`validate`'s `Ok(Validation)` is discarded.** That is correct while eligibility is off (result_export retained_precision.rs:4260, `IMPLEMENTATION_COMPLETE = false`).

**Gate order against COMPOSITION §2.**
- **G-A** runs before any observer exists: `admit` is in the dispatch, and the observer is created in `permitted_run`.
- **G-B** (retained_product.rs:3243–3247) runs after the scope checks, after observation custody, and after `source_selected` returns early on exact selection (:3237). It is immediately before `capture_case_source`. A refusal skips the capture and never touches the ordinary solve.
- **G-C** runs after the complete ordinary owner and before any W1 owner.
- **Coexistence** returns before any W1 work.
- **The one wrinkle is N1:** G-C is evaluated before the coexistence and G-B outcomes.

**My stub** (`evidence/rv85_stub_patch.py`, `rv85_stub.diff`, `rv85_stub_tests.rs`, `stub_*.out`). It uses process-global switches, so they reach the worker thread. One sequential test drives the actual public entry.

| Row | Scenario | Result |
|---|---|---|
| A | The milestone through `run_linear_static_preview_value_with_retained_direct` | Both modes: **U1's pinned successor files** `ac6986b0…59dc` and `6cd1d249…c9b5`, receipts `efc1a39b…` and `3e26499f…`. `envelope()` equals the plain value-route bytes. G-B and G-C are each called once. `admission()` is None (S3). |
| B | 49 fixture inputs × 2 modes × {Direct, Headless}, everything admitted | **0 byte mismatches, 0 panics.** Causes per route: Domain 60, Coexistence 26, Preparation 8, Candidate 2, Successor 2. Headless also published the successor, which confirms F-5: G5 must refuse Headless under D-2. |
| C | G-B refusal | `LateGate`, plain bytes (N1) |
| D | G-C refusal | `CompleteGate`, plain bytes |
| E | Stack 2^62 | `StackReservation`, plain bytes, ordinary run on the caller |
| F | The three hooks set on the caller | See S2 |

My stub mutants, run against that test (`evidence/stub_summary.txt`), are all killed:
- SV07, the G-B outcome check removed;
- SV13, G-C ignored;
- SV14, a stack failure that runs nothing.

## 3. The frozen-candidate split (`retained_product.rs`)

**`freeze_candidate`** (:3650–3760) is the old `project_candidate` up to the point of mutation.
- Every private gate, the proof, the observables and G5a checks, and the precharged move plan (:3739–3744) run first. Only then does the adapter snapshot freeze.
- That matches I51 §4: "freeze the adapter before any receipt construction". The ordinary envelope is only borrowed.

**Staging works on a copy.** `staged_envelope` (:3788–3792) clones and then overlays. X15, X18 and my V11 (maxima patches skipped in staging) are killed.

**`into_ordinary`** (:3794) returns the untouched owner. X06, X09 and X10 are killed.

**`commit_private`** (:3797–3802) gives the private driver's unchanged bytes. U1's pinned successor test passes and X16 is killed.
- `trace.freeze` is idempotent (retained_receipt.rs:57–58), and `TraceCosts` is an order-free sum. So moving the freeze before `record::<bool>()` changes no trace value.
- My V08 (the freeze skipped) and V10 (`private_committed` not set) are killed.

**`FrozenCandidate::typed_trace`** passes the certificate's work, which U2's owner binding needs. My V09 (proof work dropped) is killed.

**The F-4 unwrap removal is correct** (N4).

**One trade-off:** `apply_prepared_overlay` now clones its `LocatedQuantity` strings. In `commit_private` that means allocation after the first mutation. It is harmless because `project_candidate` has only test callers (:3815, :3843 and the test files). The facade's staging allocates before the transfer.

## 4. Single-parse custody (A-1, RV82-N9) and R08 at its call site (A-2, RV82-N1′)

**Custody holds by reading.**
- `CapturedInvocation` derives only `Debug`. `parse` is its sole constructor (source_receipt.rs:100–128).
- lib.rs has one parse, in the dispatch.
- `permitted_dispatch` → `permitted_run` → `retained_w1` only moves `request` (through `slot`) and borrows `&capture`. The spawn-failure fallback reuses the same two halves.
- `requested()` re-derivations exist only in the pre-existing source-blocks receipt code, never on the W1 path.
- N9a–c are killed. The guard's reach is limited (N7).

**R08.**
- **RV82's R08b, applied verbatim** at retained_wire.rs:1113, is **killed on the whole PP `--lib`.** 491 pass, and only `u1g2_run_conservation_pins_each_check_at_its_call_site` fails.
- R08, N1a and N1b are also killed.
- The split leaves the encoder-failure order unchanged: the four `e.exact` reads are first, then the three checks.

## 5. ROOT's two design points

**`Copy` on `CapturePermit`** (S1).
- **Today:** no permit exists, so nothing can be reused.
- **Once G5 registers a profile:** `Copy` lets a permit outlive its invocation and be reused across invocations, and the `&'static` field gives no lifetime tie. The type system then does not stop a second `permitted_dispatch` or `permitted_probe` with a different request.
- **Yes, it should be linear,** consumed by `permitted_dispatch` and bound to its invocation. The change is small (S1's remedy). The accepted API.md signatures (`&self` gate methods) do not need `Copy`.

**The thread-local hooks** (S2).
- **Grant 1's committed fault tests cannot pass vacuously.** They run `retained_w1` on the test thread and assert the exact cause.
- **Once grant 2 drives the actual entry:** a caller-set hook does not fire. My stub shows the invocation then succeeds while `envelope()` still equals the plain bytes. So:
  - a test that asserts bytes alone passes vacuously;
  - a test that asserts the cause fails, but for the wrong reason;
  - the stale flag fires later on the caller's thread.
- **Grant 2 must do what S2's remedy lists.**

## 6. The lock deltas (R-3)

**Semantic diff** of each lock, parsed with `tomllib` (`evidence/lock_semantic_diff.txt`):
- **runner/headless and the Tauri app:** the only change is `open_pipe_stress_result_export` added to `open_pipe_stress_product_physics`'s dependencies. Both graphs already held the package, because both depend on result_export directly.
- **self_weight_wasm, operation_applier, numerical_integrity and physics_audit_regression:** that edge, plus one added path package, `open_pipe_stress_result_export 0.1.0` (no source, no checksum). Its dependencies (canonical_json, units, serde_json, sha2) match result_export's Cargo.toml.
- **All seven locks:** no registry package added, removed or re-versioned. Lock version 4 is unchanged. PP's own lock is unchanged.

**The set is complete.** Outside `execution/` and `provenance/` snapshots, the only maintained locks containing PP are these six plus PP's own, and every dependent manifest has a committed lock.

**`cargo metadata --locked --offline`** (`evidence/cargo_metadata_locked_offline.txt`):
- **It passes for PP and five of the six,** for the candidate and the base alike. The candidate graphs gain result_export where the base lacked it.
- **The Tauri app fails offline in both trees** with `failed to download android_system_properties v0.1.5` (an uncached registry crate). It needs network, so I verified its one-line delta by the semantic diff only.

## 7. Mutants (`evidence/mutants_rv85.py`, `mutant_defs.py`, `mutants_i61_set.py`, `mutants_summary.txt`)

**The setup.**
- The mutants ran on a clean `git archive` of `bee3dc07ca` (WT/rv85/mut), with target WT/targets/rv85/mut.
- t13 is excluded with `--skip`.
- Every file is restored and checked by sha256 after each mutant.
- The controls: whole `--lib` 492 ok; the whole PP suite 654 ok; I61's filter 37 ok.

**Results:**
- **I61's 25 committed-tree mutants,** extracted verbatim from its `mutants.py` and run with its own filter: **25/25 killed**, with no compile-error kill. X11 kills by a stack-overflow abort.
- **RV82's R08b, verbatim, on the whole PP `--lib`:** killed.
- **My own 12:** 9 killed and 3 survived.

| Mutant | Area | Scope | Result |
|---|---|---|---|
| V01: ordinary finalization check removed | dispatch | all | killed (t20, load_state fallback) |
| V02: finalization predicate loses LOAD_REFERENCE_SOURCE | dispatch | all | **survived** (N3, pre-existing) |
| V03: the no-permit route reports a W1 result | dispatch | all | killed |
| V04: the exact-pressure budget is dropped in `ordinary_dispatch` | dispatch | all | killed (14 tests) |
| V05: coexistence predicate inverted | fallback | lib | killed |
| V06: precommit invocation without `solver_mode` | fallback | lib | killed |
| V07: G-B outcome check removed | fallback | lib | **survived** (permit-only; killed in my stub as SV07; N2) |
| V08: the frozen Ok path skips the trace snapshot | frozen split | lib | killed |
| V09: frozen `typed_trace` drops the proof work | frozen split | lib | killed |
| V10: `commit_private` does not mark `private_committed` | frozen split | lib | killed |
| V11: staging skips the maxima patches | frozen split | lib | killed |
| V12: F-4's `None` arm becomes `unreachable!` | frozen split | lib | **survived**, equivalent (N4) |

**My stub mutants** SV07, SV13 and SV14 are 3/3 killed (§2).

## 8. Nothing is weakened

**The diff touches 13 files,** all in the brief's list: PP Cargo.toml, the six PP source and test files, and the six locks. There is no reader, schema, fixture or result_export change.

**The test diffs only add lines.** retained_wire_tests.rs gains 15 lines, and retained_facade_tests.rs is new.

**No permit constructor or test permit** exists in maintained code. `legacy_native_and_typed_call_graphs_cannot_acquire_retained_admission` is unchanged and passes. F-2's relocation leaves its property true (N7).

## For ROOT

**Nothing needs a ruling before `bee3dc07ca` merges into NUM.**

**Rulings and routing before G5 or grant 2:**
- **S1:** make the permit linear and invocation-bound. This crosses owners under D-5: I65 owns the derive and the type, and I61 the call sites.
- **S2:** fold the hop-proof fault-seam requirements into the grant-2 brief.
- **S3:** choose between an API.md amendment (`admit` carries the report) and changing the documented meaning of `admission()` under U6/R-1.

**Notes for routing:**
- **N2:** for the grant-2 tests; it becomes byte-relevant once 1b's notice lands.
- **N1 and N5:** for G4 and grant 2.
- **N3, N6 and N7:** optional.
