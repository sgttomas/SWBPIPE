# I61 RETURN: U3 grant 1b (R-1, R-2 and the two flags)

**Status: complete, with no stop.** Everything is uncommitted in WT/f2a-facade, on top of `bee3dc07ca`.
- **Item 1, R-1:** implemented. Without a permit the carrier yields `Ordinary` on every path.
- **Item 2, R-2:**
  - **The condition holds.** Every base `preview-physics-1` reader and carrier I tested accepts the notice. That covers result_export's base readers, the desktop result admission and runner/headless, and also the Python base reader. Classification, standing and export availability are unchanged.
  - **N1 is implemented** through the private driver and the existing fault hooks.
- **Item 3, the flags:** both are fixed within the fence.
  - `CapturePermit` is now linear.
  - The armed test hooks are now carried onto the reserved-stack thread.
- **Controls 1–5 hold.** No published byte changes while no permit exists.

**Run facts.**
- **Role and basis:** TASK Type 2 under ROOT, with no descendants. The grant is ROOT's message and RR "U3 grant 1 verified; R-1, R-2 and R-3 ruled" (NUM `efde9ca2d1`).
- **Time:** 2026-10-04, 06:13Z to 06:50Z.
- **Host:** the memory guard (PID 5387) was running throughout. Default toolchain; `--locked --offline`; `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`; one Cargo job at a time, each under a 1200 s alarm. vitest and Python ran under 600 s and 300 s alarms. Nothing native, at scale or DEC-025 was run.
- **Git:** no Git writes. Reads used `GIT_OPTIONAL_LOCKS=0`. `git archive bee3dc07ca` made the lane.
- **Other agents' files:** I did not touch R/REVIEW_RV85/, WT/rv85/, R/REVIEW_RV82/, WT/rv82/ or I65's files.
- **Writes:**
  - WT/f2a-facade, within the fence: `lib.rs`, the permit consumer in `retained_memory.rs`, `retained_product.rs` and `retained_facade_tests.rs`;
  - WT/scratch/i61_u3_facade_02/: the `lane` archive of `bee3dc07ca` with P's `node_modules` linked and the prebuilt `public/`; `cand`; `stub`; `mut`; `mutstub`; `out`;
  - WT/targets/i61-u3/;
  - this folder.

## 1. R-1, the carrier (Proposal A)

All references are to `lib.rs`.
- **`pub enum RetainedPublication { Ordinary(MechanicsEnvelope), Successor(serde_json::Value) }`** (:2187).
- **`RetainedPreviewOutput::successor() -> Option<&Value>`** (:2233) returns `Some` only when the permitted transfer completed.
- **`into_publication(self) -> RetainedPublication`** (:2241) gives `Successor` on a completed transfer, otherwise `Ordinary(envelope)`, notice included where R-2 applies.
- **`envelope()` and `into_parts()` are unchanged.** `envelope()` is now documented as "the ordinary base: the publication unless `successor()` is present".
- **No other public type changed.**

**Tests:**
- `u3_no_permit_entries_are_the_ordinary_route`: both modes; `successor()` is `None` and `into_publication()` is `Ordinary` with the plain bytes.
- `u3_r1_carrier_names_exactly_one_publication`: a completed transfer gives `Successor` with the validated value, while `envelope()` stays the plain bytes; a precommit fallback gives `Ordinary` with the plain bytes plus the notice.
- **Behind the archive stub,** the actual Direct entry gives `Successor` (U1's pinned bytes) on success, and `Ordinary` on every fallback across all 72 fixture invocations.

## 2. R-2, the notice

### 2a. The condition holds: the base readers and carriers accept it (established before any notice code was written)

**The bytes tested** are the ruled notice appended after the ordinary prefix:
- `id` `diagnostic:retained-precision:{case}:unavailable`, `code` `RETAINED_PRECISION_UNAVAILABLE`, `severity` `info`;
- `source` `core/product_physics`, `affected_refs` `[case]`;
- fixed text with no receipt reference;
- also the receipt-encoding variant of the text.

**The publications** are two: the Sensitive milestone, and an exportable checks-passed solve (the zero-pressure invented model the maintained runner test uses), each in both modes. **Each reader also refused a malformed notice** (`affected_refs: ["result:not-a-row"]`), so none of these checks passes vacuously.

| Reader or carrier | How tested | Result |
|---|---|---|
| result_export's base readers (Rust) | committed PP test `u3_r2_base_readers_accept_the_unavailable_notice` | `semantic_contract::for_source` admits, with the same contract as base. `standing_reason` and `numerical_use_standing` (with and without the invocation) are unchanged: `needs_recompute` for the milestone, `numerically_eligible` for the exportable solve |
| Desktop result admission (TS) | lane archive of `bee3dc07ca`, vitest (`r2_ts_i61R2Probe.test.ts`) | `sourceContract` is `preview_physics`, `validatePreviewPhysicsEvidence` passes, and `numericalResultStanding` equals base (milestone `needs_recompute`; exportable `integrity_checked`, eligible). `canonicalSha256HexCheckedV1` hashes |
| runner/headless | lane archive, probe module (`r2_runner_i61_r2_probe.rs`) repeating the dispatch's Value route with the notice appended. The probe's copy equals the maintained route on the unchanged envelope | Runner status, diagnostics and job are unchanged, and nothing blocks. Export availability equals base: the milestone has none at base either (Sensitive); the exportable solve mints its export document, which `validate_document` accepts and which carries the notice (38 diagnostics against base's 37) |
| Python base reader (not required; checked anyway) | `r2_py_probe.py` | `validate_preview_physics_evidence` passes, and `numerical_use_standing` equals base |

### 2b. N1, implemented

All references are to `lib.rs`.

**The constants.**
- `RETAINED_UNAVAILABLE_NOTICE` (:2998) is the fixed product text: "Retained-precision recovery is unavailable for this load case. Its published rows keep their ordinary values, standing and diagnostics."
- The code is `retained_wire::UNAVAILABLE_CODE`, and the id is U1's id form.

**When it applies.** A notice is appended only where W1 work ran: a Preparation, Native, Candidate, Serializer or Precommit fallback. It goes after the ordinary diagnostic prefix, and only after a single-case W1 attempt.

Exact ordinary bytes, with no notice, are kept wherever no W1 work ran:
- G-A, G-B and G-C refusals;
- a stack spawn failure;
- the Domain guard;
- coexistence;
- an invocation outside D1.4 (two cases, or a combination);
- a failed notice reservation.

**The receipt-encoding text.** A serializer refusal whose `check.wire()` is one of C1:68's named details appends " Reason: receipt_encoding; detail: {token}." The named details are `work_counter_range`, `work_counter_inconsistent` and `saturation_not_excluded`, plus `publication_hash_range`, which C1:68 says is used "similarly". The function is `receipt_encoding_detail`, :3008. An `association` or `encoding` refusal is not a receipt-encoding fallback and gets the plain text (finding F-1).

**Space reserved before W1 (COMP:66).** `ReservedNotice::reserve` (:3017) does this before preparation starts:
- `try_reserve_exact(1)` on the ordinary owner's diagnostics;
- the message buffer reserved for the longest variant;
- the notice built in advance.

If the reservation fails, or the base already carries the notice's id, W1 does not start and the cause is `W1Fallback::NoticeReservation`, with exact bytes. `publish` only pushes into the reserved capacity. In test builds it asserts that the spare capacity exists, and the tests shrink the base's capacity first, so only the reservation can supply it.

**`w1_case_id` (:3061)** names the one case W1 attempts (D1.4). Outside D1.4 the cause is `Domain`, so a notice never names an invented case.

**Tests** (`retained_facade_tests.rs`):
- `u3_each_stage_fault_falls_back_to_the_ordinary_bytes` pins exact bytes for every case:
  - preparation, native, candidate (Maxima and ValuesCompletion), serializer association and precommit (G1 and G8) each give plain + notice;
  - nine serializer checks, through a new `fail_next_serializer` hook, give plain + notice with or without the reason, as the table above says;
  - coexistence, an existing notice id, two cases, and a combination each give exact bytes.
- `u3_r2_notice_bytes_are_pinned` pins the text against a literal, with no "receipt", and checks that the message capacity covers the longest detail.

The expected bytes are built in the test, independently of the product constants.

## 3. The two flags (RV85 is also assessing them)

**The flag: `CapturePermit` derived `Copy`.**
- **Proposal, implemented:** a linear permit (`retained_memory.rs` :300) with no derive and no impl.
- The facade reads `reserved_stack_bytes()`, then moves the permit onto the reserved-stack thread. From there it moves into the observer through `permitted_probe(permit)`.
- G-B borrows it (`self.permit.as_ref()`, `retained_product.rs` :3244). G-C borrows it through `observer.permit()` (:3308).
- On a spawn failure it drops unrun with the closure. Nothing re-reads a moved permit.
- A fail-closed `W1Fallback::PermitUnbound` arm covers an observer without a permit at G-C. It is unreachable by construction (F-2).
- **Test:** `u3_capture_permit_is_linear` is structural: no derive or `Clone`/`Copy` impl, the permit moves into the observer, and G-B and G-C borrow the observer's permit. Mutant B20, which adds the derive back, is killed by it.
- **API.md's `&self` signatures are kept.** A permit consumed by value at G-C would change U4's interface, which is I65's. I propose it to I65 as optional for G5. It is not needed for linearity.

**The flag: `thread_local!` test hooks will not fire on the reserved-stack thread.**
- **Proposal, implemented:** the armed faults move with the work. They are `retained_tests_hooks::Armed`: corrupt, withdraw, rebind, the serializer check, and lib.rs's dense-scrutiny ceiling override.
- `permitted_dispatch` wraps its work in `carry_test_hooks` (:2958). In test builds this takes the caller thread's armed faults and installs them on the worker. The ceiling override is copied rather than taken, because it is a setting. In production builds it is the identity.
- **Test:** `u3_test_hooks_follow_the_reserved_stack_thread`:
  - uncarried, an armed fault does not fire on another thread and stays armed, which shows the hazard is real;
  - carried, each kind (precommit, native, serializer, rebind) fires there once and none is left armed;
  - the ceiling override is visible there and kept here.
- **Behind the archive stub,** a fault armed on the test thread fires on the actual Direct entry's reserved-stack thread: Precommit G1 and Native, each publishing plain + notice.
- **For grant 2, not changed:** other modules' thread-local test seams are outside the fence or not mine. These are U4's `retained_memory::tests::DISPATCH_COUNT`, `source_receipt/composite.rs`'s `WORK_TRACE`, and `historical_pressure_reference.rs`'s `ACTIVE`. Committed permit-path tests that rely on them must carry them the same way.

## Controls

1. **No published byte changes without a permit.**
   - **The fixture sweep:** 36 request-shaped fixtures × 5 routes (typed default, typed mode, value, retained Direct, retained Headless) × 2 modes = 324 outputs, including the admission report. They are **byte-identical** to base (`b54caba7ab`, the same file as grant 1's).
   - **PP:** grant 1's 656 outcomes are unchanged, plus 5 new tests, all ok. Against base, all 650 outcomes are unchanged. t13 (Mac) is the only failure, as at base.
   - **runner/headless,** run directly in WT now that `bee3dc07ca` carries the lock edges: 87 outcomes, identical to base (85 passed and the 2 known Mac load_reference failures).
   - **Production warnings:** PP's set equals base. The one new line is result_export's own existing warning.
2. **The pinned successor bytes** are published unchanged in both modes, through the committed private-driver test and through the stub's actual entry (`outputs_sha256.txt`).
3. **Fault controls:** see §2b. Each W1-ran fault gives plain + the ruled notice, and each no-W1 fault gives exact bytes.
   - **Behind the stub, under a permit-everything profile:** every fixture invocation's bytes are plain, or plain + notice exactly where W1 ran. The causes are 36 Domain, 26 Coexistence, 4 Preparation (+notice), 2 Candidate (+notice), 2 successor and 2 parse errors equal to the plain route. There were no panics.
   - The stub's G-B, G-C and NoStack refusals give exact bytes.
   - T20 C1 stays fail-closed.
4. **Nothing is weakened.**
   - No reader, schema, fixture or existing check changed.
   - There is still no permit constructor in maintained code. The stub, with its inhabited profile, exists only in the disposable archive. There, `u3_capture_permit_is_linear` fails by construction, because the stub replaces the maintained struct.
5. **Mutants: 39.**
   - **38 are killed, none by a compile error.**
   - **B08** (combinations not checked) survived run 1. I added the one-case-with-combination check, and it is now killed.
   - **The one survivor is Y09.** The fail-closed `PermitUnbound` arm proceeding to W1 is equivalent in every reachable state: `permitted_probe` always holds the permit and nothing takes it.

   | Group | Mutants |
   |---|---|
   | The candidate tree, against committed tests | B01–B20 (R-1, R-2, hooks, linear permit) and 10 of grant 1's mutants, re-expressed on the new text |
   | The stub tree, the permit-only branches | Y01–Y09 |

## Findings

**F-1. A reading, implemented and flagged for ROOT.** I read ROOT's "a receipt-encoding fallback" as C1:68's named details only:
- the three work-counter tokens, plus `publication_hash_range` ("similarly");
- **not** `association` or `encoding`, which get the plain text.

The alternative is that every serializer refusal states "reason receipt_encoding; detail check.wire()". That is a one-line change in `receipt_encoding_detail`, plus the test's two expectations. **No byte is published either way until a permit exists.**

**F-2. Y09 is equivalent.** `PermitUnbound` is unreachable by construction and fails closed with exact bytes.

**F-3. A new no-W1 cause, `NoticeReservation`.** I added it for R-2's reservation and for a pre-existing notice id. It gives exact bytes and never starts W1. G4 prices the reservation: one `Diagnostic` slot, plus a message buffer of 135 + 35 + 25 + 1 bytes, plus the id and case strings.

**F-4. The defensive Domain guard is extended to D1.4 for W1** (one load case, no combinations), so the notice always names the one attempted case. Permit-everything stub sweeps now show multi-case fixtures as Domain instead of Preparation.

**F-5. For grant 2:** see §3 for the other modules' thread-local seams.

## Records (`_run_records/`)

All paths are placeholders, with no machine paths.

**Code and hashes**
- `changed_files_sha256.txt`: against `bee3dc07ca`.
- `candidate.diff`, `candidate_status.txt`.

**Suites, warnings and the sweep**
- `suite_{base,final}_{pp,runner}.outcomes`, `build_{base,final}.warnings`.
- `fixture_sweep.tsv`, `fixture_sweep_compare.txt`.

**R-2 acceptance**
- `r2_*`: the TS, runner and Python probes and their results, and `r2_envelopes_sha256.txt`.

**Stub and mutants**
- `stub_patch.py`, `stub_test_tail.rs`, `stub_e2e.txt`.
- `mutants.py`, `mutants_summary.txt`.

**Other records**
- `outputs_sha256.txt`, `run_final.sh`.
