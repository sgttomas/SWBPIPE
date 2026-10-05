# I61 RETURN: U3 grant 1 (facade capture installation)

**Status: grant 1 is implemented in WT/f2a-facade and is uncommitted (no Git writes). Controls 1–5 hold. I am returning because three rulings are needed before grant 2 or a merge.** The brief says "Return early only if a ruling is needed", and none of these is mine to decide:
- **R-1 (checkpoint item 1):** the successor carrier's public surface.
- **R-2 (checkpoint item 2):** the bytes of the public `RETAINED_PRECISION_UNAVAILABLE` notice.
- **R-3 (new, finding F-1):** decision 5's runtime dependency needs minimal edges in six maintained `Cargo.lock` files that are outside the write fence. Without them the branch cannot pass CI's `--locked` cargo suite.

ROOT's two mid-grant additions are done and reported separately: **A-1 is RV82 N9** (single-parse custody) and **A-2 is RV82 N1′** (R08 pinned at its call site).

**Run facts.**
- **Role and basis:** TASK Type 2 under ROOT's grant (brief `BRIEFS/I61_U3_FACADE_CAPTURE.md`, NUM `92750f1b6e`), with no descendants.
- **Time:** 2026-10-04, from about 05:16Z (worktree creation) to 06:07Z. CHECKPOINT.md was written at about 05:27Z.
- **Host:** the memory guard (PID 5387) was running throughout. Default toolchain; `--locked --offline`; `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`; one Cargo job at a time, each under a 1200 s alarm. Nothing native, at scale or DEC-025 was run.
- **Git:** no Git writes. Reads used `GIT_OPTIONAL_LOCKS=0`.
- **Other agents' files:**
  - I read RV82's `REVIEW_RV82/u1_serializer_02/REVIEW.md` and its mutant script, only to get N1′/N9 and R08b's exact text. I did not modify them.
  - I did not touch WT/rv82/ or I65's files.
- **Writes:**
  - WT/f2a-facade, within the fence plus ROOT's additions (`retained_wire.rs` conservation split; `retained_wire_tests.rs`);
  - WT/scratch/i61_u3_facade_01/, holding the disposable archives `base`, `cand`, `stub`, `mut` and `mutstub`;
  - WT/targets/i61-u3/;
  - this folder.

## Rulings needed

**R-1, the carrier (CHECKPOINT §1).**
- **Grant 1:** the carrier is the crate-private `RetainedPreviewOutput.retained: Option<Result<RetainedSuccessor, W1Fallback>>`. No public type changes.
- **Proposed (A), additive:**
  - `successor() -> Option<&Value>`;
  - `into_publication() -> RetainedPublication { Ordinary | Successor }`;
  - `envelope()` is unchanged.

**R-2, the notice bytes (CHECKPOINT §2).**
- **Grant 1 implements N0:** exactly the ordinary bytes on every fallback. The cause is kept privately in `W1Fallback`.
- **Proposed N1:** one info `RETAINED_PRECISION_UNAVAILABLE` diagnostic, appended only when W1 work actually ran. That covers Preparation, Native, Candidate, Serializer and Precommit. Its bytes are given in CHECKPOINT §2 (ROUTING:96/98; C1:64/68; D1:573; COMP:66).
- **Open fact:** whether the base `preview-physics-1` readers accept that diagnostic.

**R-3, the downstream lock files.** See F-1 and the CHECKPOINT addendum.
- **Proposed:** extend the fence to exactly the six deltas in `_run_records/downstream_lock_deltas.diff`.

## What grant 1 implements

Paths are under PP/src. Hashes are in `_run_records/changed_files_sha256.txt` and the diff is in `_run_records/candidate.diff`.

**The frozen-candidate split (D-b), `retained_product.rs`.**
- `freeze_candidate` (:3650) runs every private gate, the proof and the move-plan checks, and returns `FrozenCandidate` (:3780) with the ordinary envelope **untouched**.
- `staged_envelope` (:3788) applies the shared `apply_prepared_overlay` (:3766) to a clone. `into_ordinary` returns the untouched owner.
- `project_candidate` is now `freeze_candidate` followed by `commit_private` (:3797). This is the private driver's unchanged bytes: U1's pins pass.

**G-B, at :3242.** It runs immediately before the late old-source capture and is bound to the observer by `permitted_probe(permit)`. A refusal skips the capture and is recorded.

**The serializer over a frozen candidate, `retained_wire.rs`.** `serialize_frozen` (:1474) reads the staged copy through the `SelectedCandidate` trait (:1480). `serialize_selected` is unchanged in behaviour.

**The dispatch, `lib.rs`.**
- **G-A** calls `retained_memory::admit` (:2243). `Some(Ok(permit))` goes to `permitted_dispatch` (:2887). The ordinary route is `ordinary_dispatch` (:2266) and is unchanged.
- **The permitted path** runs everything after G-A on one scoped thread with the permit's reserved stack (`on_reserved_stack` :2907, STACK_PLAN §1). A spawn failure means StackReservation, after which the ordinary route runs on the caller's thread. A panic is re-raised with its payload.
- **`permitted_run` (:2921):**
  - the D1.3 Domain guard;
  - the single observed ordinary run;
  - the finalization check;
  - G-C;
  - `retained_w1`.
- **`retained_w1` (:2950)** runs, in order:
  - coexistence;
  - G-B's outcome;
  - preparation and native;
  - freeze;
  - staging and serialization;
  - **precommit validation** with `result_export::retained_precision::validate(&successor, Some(&{request, solver_mode}))` (decision 5);
  - the **transfer**, which only moves values.

  Every fallback returns the untouched ordinary owner.

**The U3 consumer shim, `retained_memory.rs` :305.** It holds the API.md §2 names: `admit`, `CapturePermit::{reserved_stack_bytes, check_late, check_complete}`, `LateFacts`, `CompleteFacts` and `PhaseRefusal`.
- The method bodies are `match *self._profile {}`, so `RegisteredProfile` stays uninhabited.
- `assess` wraps `admit` with unchanged results.
- **Merge point with I65's G5:** I65 replaces these bodies and may extend the fact records.

**The dependency, `Cargo.toml`.** `open_pipe_stress_result_export` moved from a dev-dependency to a runtime dependency. PP's `Cargo.lock` is byte-unchanged.

## Controls

### 1. Ordinary bytes: identical to base

**The fixture sweep** (`_run_records/fixture_sweep*.{tsv,txt}`, `zz_i61_u3_fixture_sweep.rs`).
- **Inputs:** every request-shaped fixture under `P/fixtures`: 31 requests, plus 5 models wrapped as `{model}`, for 36 inputs.
- **Routes:** `run_linear_static_preview` (typed default), the typed mode route, the value route, retained Direct and retained Headless, each in both modes. That gives 324 outputs.
- **Hashed:** each envelope's bytes, or its error, plus the admission report's Debug.
- **Result:** base (b54caba7ab) and the final candidate are **byte-identical**.

**The PP suite** (lib plus integration targets).
- All 650 base outcomes are unchanged. The candidate adds 6 tests, all ok, for 656 outcomes.
- The only failure is the known Mac `t13_committed_fallback_uz_is_byte_identical`, as at base.

**runner/headless.** 87 outcomes, identical to base: 85 passed and the 2 known Mac load_reference failures.
- It was run in the candidate archive with R-3's one-line lock edge, because `--locked` refuses the worktree as it stands (F-1).

**Production build warnings.**
- PP has 8 warnings, the same set as base.
- The new lines are `result_export`'s own pre-existing `derived` warning (F-3).

### 2. U1's pinned successor bytes

| Mode | File | Receipt |
|---|---|---|
| sparse | `ac6986b0…` | `efc1a39b…` |
| dense | `6cd1d249…` | `3e26499f…` |

- `u3_permitted_path_publishes_the_pinned_successor` (committed) publishes these from the facade's single observed run, through freeze, staging, serialization and precommit. The returned ordinary envelope is byte-identical to the plain route.
- **Behind the archive stub,** the actual `run_linear_static_preview_value_with_retained_direct` publishes the same bytes in both modes (`stub_e2e.txt`, `outputs_sha256.txt`).

### 3. Fault controls

Each stage below falls back to the preserved ordinary bytes (N0, pending R-2).

**Committed** (`u3_each_stage_fault_falls_back_to_the_ordinary_bytes`):

| Stage | Trigger | Result |
|---|---|---|
| Preparation | annulus refusal | `W1Fallback::Preparation` |
| Native | the prepared source is withdrawn | `Native` |
| Proof or facade | Maxima and ValuesCompletion faults | `Candidate` |
| Serializer | a foreign invocation (S2) | `Serializer(Association "invocation")` |
| Precommit | corrupted receipt hash | `Precommit{G1, RETAINED_PRECISION_RECEIPT_MISMATCH}` |
| Precommit binding | validated against the other mode | `Precommit{G8, RETAINED_PRECISION_INVOCATION_MISMATCH}` |
| Coexistence | — | exact publication returned as is |

**The transfer has no fault point by construction.**
- All fallible work happens before it: the staging clone, serialization, building the invocation and validation.
- The transfer then moves `frozen.into_ordinary()` and the validated `Value`.
- Mutant X10 shows the tests catch a transfer that publishes the wrong owner.

**Behind the archive stub** (evidence, not code):
- G-B refusal gives `LateGate`, G-C refusal gives `CompleteGate`, and a spawn failure at 2^62 gives `StackReservation`. All three return exact ordinary bytes in both modes.
- **T20's C1 construction** stays fail-closed (`SOURCE_BLOCKS_FINALIZATION_FAILED`), exactly as the ordinary route.
- **Under a stub that admits everything,** all 72 fixture invocations keep their exact ordinary bytes. There were no panics. The causes were:

  | Cause | Count |
  |---|---|
  | Domain | 32 |
  | Coexistence | 26 |
  | Preparation | 8 |
  | Candidate | 2 |
  | Successor (the milestone) | 2 |
  | Parse error (same as plain) | 2 |

### 4. Nothing is weakened

- No reader, schema, fixture or existing check changed.
- There is no permit constructor: `RegisteredProfile {}` is still uninhabited, and the stub exists only in the disposable archive (decision 7).
- `retained_precision_admission.rs::legacy_native_and_typed_call_graphs_cannot_acquire_retained_admission` is unchanged and passes. See F-2: my first layout tripped it, and I corrected the layout, not the check.

### 5. Mutants

The mutant set is in `mutants.py` and the results are in `mutants_summary.txt`.
- **33 mutants, all killed, none by a compile error.**
- **25 run in the candidate tree** against the committed tests: X01–X18, R08b, R08, N1a, N1b and N9a–c.
  - X11 (stack size ignored) is killed by a stack-overflow abort of the test binary. The others fail named tests.
- **8 run in the stub tree** against the archive-only tests: Y01–Y08 (G-B ignored, G-B outcome unchecked, G-C ignored, stack failure not ordinary, permit dispatch bypassed, Domain guard removed, finalization check removed, observer unbound). These branches are unreachable without a permit, so grant 2 commits their tests.
  - **Y07 survived run 1.** I added the C1 stub test, re-cloned the stub tree, and it is now killed by `u3_archive_stub_finalization_failure_stays_fail_closed`.

## ROOT's additions (reported separately)

### A-1. RV82 N9: single-parse custody (test plus structural argument)

**The argument.**
- **One constructor.** `CapturedInvocation` derives only `Debug` (source_receipt.rs:101–102). Its only constructor is `parse` (:109), which returns the typed request and the capture from the same raw `Value` in one call. No `impl … for CapturedInvocation` and no `Clone` exists.
- **One production parse site.** It is the dispatch (lib.rs:2251), and that parse runs once per invocation.
- **The permitted path only moves or borrows the two halves:**
  - `request` goes into `slot`, then into `permitted_run`, and on into the observed run;
  - `&capture` goes to the observed run (`Some(capture)`, where the observer records S2's digest), then to `retained_w1`, `serialize_frozen` (S2's check) and the precommit invocation;
  - the spawn-failure fallback uses the same `request` and `&capture`.
- **So the typed request that runs and the invocation S2 binds are the two halves of one parse.**

**One existing re-derivation, unchanged.** source_receipt's private `requested()` re-derives a typed request for replay checks only. It never runs a solve or creates a capture.

**The test.** `retained_facade_tests::u3_n9_single_parse_custody` pins every fact above in the source text:
- lib.rs has exactly one `CapturedInvocation::parse(`, inside the dispatch;
- the permitted path contains no `parse(`, `from_value(`, capture or request clone, and no struct literal;
- it contains the required move and borrow sites;
- there is no parse in retained_product, retained_wire, retained_receipt or production retained_memory;
- in source_receipt, there is one `impl CapturedInvocation` with one `Self {` literal (parse's), no `-> Self`, and no trait impl.

**Behaviourally,** the stub E2E succeeds through S2's digest check on the facade's single capture, and the committed serializer fault shows a foreign capture is refused.

**Mutants:** N9a (a second parse in `permitted_run`), N9b (the request re-derived from the capture) and N9c (the dispatch parses twice) are all killed.

### A-2. RV82 N1′: R08 pinned at its call site in `run_conservation`

**The change.** `run_conservation` (retained_wire.rs:1093) now reads the run's four exact amounts in the same order as before, then calls `run_conservation_amounts` (:1105). That function holds the three checks unchanged, including R08's `(after_conserved(e, before, increment, after), "cases[].run.invocation_after")`. Encoder-failure order is unchanged.

**The test.** `retained_wire_tests::u1g2_run_conservation_pins_each_check_at_its_call_site` calls the checks with explicit amounts:
- the conserved amounts pass;
- after ± 1 gives exactly `work_counter_inconsistent` at `cases[].run.invocation_after`;
- a wrong case charge is refused at `case_charge`;
- a wrong increment is refused at `invocation_increment`, then at the `invocation_after` it implies.

**Mutants:** RV82's **R08b**, applied verbatim at the call site, is now **killed**. Also killed: the R08 helper mutant, N1a (the case_charge call site) and N1b (the increment call site).

## Findings

**F-1. Downstream lock files (R-3).** Making result_export a runtime dependency of PP adds an edge in every maintained `Cargo.lock` that builds PP.

| Lock file | Change |
|---|---|
| `P/core/runner/headless` | +1 line |
| `P/apps/desktop/src-tauri` | +1 line |
| `P/core/loads/self_weight_wasm` | +1 line, +10-line package block |
| `P/core/model_operations/operation_applier` | +1 line, +10-line package block |
| `P/validation/benchmarks/numerical_integrity` | +1 line, +10-line package block |
| `P/validation/benchmarks/physics_audit_regression` | +1 line, +10-line package block |

- **The deltas are in `downstream_lock_deltas.diff`.** Cargo computed them offline in the disposable archive.
- **No registry package or version changes.**
- **The `float_roundtrip` feature is unchanged.** `cargo tree -e features -i serde_json` shows it was already on in all seven graphs, because PP enables it.
- **These files are outside the fence** (runner/headless explicitly), so I did not write them. The local runner/headless `--locked` run refuses the worktree as it stands.
- **CI's "Numerical cargo suite"** (`tools/ci/numerical_ci.py`) runs `cargo fetch --locked` and `cargo test --locked` per discovered manifest.
- **Unmeasured:** whether the wasm crate's output grows. I ran no wasm build.

**F-2. The structural admission test tripped on my first layout.**
- `legacy_native_and_typed_call_graphs_cannot_acquire_retained_admission` scans lib.rs between `fn run_linear_static_preview_value_dispatch(` and `fn run_linear_static_preview_captured(` for `ProductCapture::`.
- My new `permitted_run` sat in that span and failed it.
- **The fix:** I moved the permitted-path functions, unchanged, after `run_linear_static_preview_observed`. The check is unchanged.
- **Its property still holds.** The legacy and typed entries pass `None` and never reach `permitted_dispatch`, which only `Some(Ok(permit))` from `admit` reaches.
- **For grant 2,** RV or the check's owner may want the check to name `permitted_dispatch`'s single caller explicitly. I did not change it.

**F-3. Warnings.**
- Shim items that U4 G5 reads (the `LateFacts` and `CompleteFacts` fields and the `PhaseGate` variants) carry `#[allow(dead_code)]`. `assess` is now used only by the census tests, so it carries `#[cfg_attr(not(test), allow(dead_code))]`.
- PP's production warning set equals base.
- `result_export`'s pre-existing `function derived is never used` now appears in PP's production graph. The readers are outside the fence.

**F-4. Changed behaviour on an unreachable path.** `freeze_candidate` returns a typed refusal ("frozen candidate without certificate") where the old commit called `certificate.unwrap()`. A certified Ok always carries the certificate, so I could not construct this case and there is no mutant for it.

**F-5. Note for I65 and U4 (D-2).** The stub admitted every entry. The dispatch takes the permitted path for Headless as for Direct whenever `admit` returns Ok. Under D-2, U4's `admit` must refuse Headless (DOMAIN §3, caller).

**F-6. Note for U4's D1 predicate.** Under a permit-everything stub, W1 on non-D1 models fell back cleanly every time:
- Preparation for invented_preview, dec092, and rejected_stress_range;
- Candidate for precision_connected_ui;
- Domain or Coexistence elsewhere.

There were no panics and no byte change. This is defence in depth only; the D1 predicate remains the gate.

**F-7. Precommit cost for G4.** The invocation `Value` deep-copies the raw request (CHECKPOINT §3).

**F-8. ROOT's correction is noted.** The `prior` omission cites RR:7784 and C3:261–263. There is no code change.

## Evidence index (`_run_records/`)

All paths are placeholders (WT, NUM, P, PP), with no machine paths.

**Code and hashes**
- `changed_files_sha256.txt`: base and candidate hashes for each changed file.
- `candidate.diff` and `candidate_status.txt`.

**Suites and warnings**
- `suite_{base,final}_{pp,runner}.outcomes`.
- `build_{base,final}.warnings`.

**Fixture sweep**
- `fixture_sweep.tsv`, `fixture_sweep_compare.txt` and `zz_i61_u3_fixture_sweep.rs`. The sweep test was archive-only and identical in both archives.

**Stub archive**
- `stub_patch.py` and `stub_test_tail.rs`: the disposable stub.
- `stub_e2e.txt`.
- `outputs_sha256.txt`: the committed-test and stub-dispatch successor files, which match U1's pins.

**Mutants**
- `mutants.py` and `mutants_summary.txt` (runs 1 to 3).

**Other records**
- `downstream_lock_deltas.diff` (F-1).
- `run_suites.sh`, `run_cand_runner.sh`, `run_sweep.sh` and `run_final.sh`.
