# I61 RETURN: U1 grant 2, stopped at the D38 reader check (S-1)

**Status: STOPPED, as grant 2 instructs: "Any other reader failure is a stop: report it, don't decide it."**

**Why it stopped.** The D38 receipt, and every other one-case unavailable receipt, fails first at **G3 `RETAINED_PRECISION_COVERAGE_MISMATCH` ("no selected case")** in all three accepted readers. That is T3's ruled rule: a one-case invocation whose case is unavailable has no successor publication. ROOT expected the first failure at the stricter "native entered ⇒ Run" check, which sits later, at G5. I did not decide the conflict (S-1 below).

**Done before the stop; uncommitted in WT/f2a-serializer on top of `59a5de2032`:**
- Item 1, G-i: the closed translations.
- Item 3: U2 on the failure path, with the seam's `I61_FOREIGN_OWNER` assertion deliberately flipped.
- Item 4: the C1–C3 unavailable representation, including D38.
- Item 5: the mutant set for all of it.

**Protected controls 1–5 hold.** The milestone's ordinary bytes and the pinned successor bytes are unchanged. No reader, schema, fixture, dispatch, retained_memory.rs or production caller was touched.

**RV82's grant-1 findings** (routed in RR "RV82 on U1 grant 1: PASS; routing") were repaired in the same stream: S2, S1, N1, N6, N7 and N3, each reported separately below. N4 is in item 3.

TASK Type 2 under ROOT's grant (RR "U1 grant 1 with U2 verified and committed; … grant 2 and RV82 dispatched", NUM `af8bd1141b`), with no descendants.
- **Time:** 2026-10-04T04:16Z to about 05:15Z.
- **Host:** the memory guard (PID 5387) was running throughout. Default toolchain; `--locked --offline`; `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`; one Cargo job at a time, each under a 1200 s alarm.
- **Git:** no Git writes; reads used `GIT_OPTIONAL_LOCKS=0`. I did not touch R/REVIEW_RV82/ or WT/rv82/.
- **Writes:** WT/f2a-serializer (within the fence), WT/scratch/i61_u1_serializer_01/lane/ (the TS and Rust reader lane, reused) and WT/scratch/i61_u1_serializer_02/, WT/targets/i61-u1/, and this folder.

## Stop items (reported, not decided)

**S-1. The one-case D38 and unavailable receipts fail first at G3, not at "native entered ⇒ Run".** `_run_records/reader_results_unavailable.txt`

All five emitted representations are schema-valid (0 violations; G0–G2 pass). Each reader's first failure on each one is G3:

| Reader | First failure on all five | Location of "any selected" | Location of "native entered ⇒ Run" |
|---|---|---|---|
| Python | G3 COVERAGE_MISMATCH | P/core/analysis_runs/retained_precision.py:1634 | :836 (`_g5_stages`, G5) |
| Rust | G3 COVERAGE_MISMATCH | P/core/reporting/result_export/src/retained_precision.rs:664 | :2245–2246 |
| TypeScript | G3 COVERAGE_MISMATCH | P/apps/desktop/src/features/results/retainedPrecision.ts:227 | :678 |

The five representations are:
- the preparation refusal (sparse);
- D38 (sparse and dense);
- the candidate refusals `abandoned` and `values` after a selected Run (sparse).

**A probe, not a reading.** I called the Python reader's own `_g5_stages` check list directly on each product attempt:
- the D38 receipts fail **:836**, the expected native⇒Run check, and also **:837**, "preparation completed ⇔ source_ref present";
- the other three receipts pass `_g5_stages`.

The :837 failure comes from my injected trigger. To obtain D38 from PP, I withdraw the prepared source, which leaves `source_ref` null after a completed preparation. A real D38 failure, such as an origin capacity or allocation error after preparation, keeps a prepared source; that is S-2.

**ROOT's decision:** T3 already rules that no one-case unavailable successor is published, so the readers are faithful. The next reader round's pin for D38 therefore needs a frame. Two candidates:
- a multi-case corpus (wider F2a);
- a G5-scoped fixture.

Only ROOT can choose it.

**S-2. D38 with a prepared source; fails closed.** C2 §4 says a case "uses a source_ref whenever a source was prepared", and the reader rule at PY:837 agrees. Its CaseSource needs `kernel_source_sha256` and `stiffness_sha256`.
- Both are available from the source itself: `PrimitiveSource::encoding()` (K4SRC) and `stiffness_encoding()` (K4STF) are `pub`.
- The kernel normally registers those bytes at the native call, which never happened in D38.
- Building the CaseSource before registration is a reading of C2 §4 together with D38. The serializer refuses instead (`Untranslated`, `sources[].kernel_source_sha256`).
- This branch cannot be reached from PP without a kernel fault seam, so mutant V06 survives as untestable.

**S-3. `source_decline` for a source-constructor refusal; fails closed.** C2/C3 require `source_decline {input_owner, constructor_counts, error}`.
- The constructor counts are not captured typed, so the serializer refuses with `Untranslated`, `cases[].source_decline`.
- The test is `u1g2_source_constructor_refusal_fails_closed`, driven by an actual `TraceFault::SourceConstruction`; mutant V05 is killed.

## What was done

### Item 1, G-i: the closed translations (`retained_wire.rs`)

**Tables:**
- **Reason**, in all four spaces:
  - attempt: stop, stop_rule, verification_estimate, charge, verification_failed, publication_enclosure, uc, theta, g_validity;
  - **Stop**: all 14 AttemptStop variants, including count_range and work_accounting;
  - **Unresolved**: all 11;
  - **Refusal**: all 5.
- **WideError, SumError and LedgerError.**
- **BlockRefusal**, with C1's `block_step` sentinel for a `usize::MAX` row.
- **NumericError, ViewIssue and BridgeError**, through the kernel's typed `BridgeFailure`.
- **ProductError**, through the typed `ProductFailureView`, including HelperError.
- **SourceError and MemberProperty; OriginError; CaptureError; G5aError; SectionError**, including arithmetic.
- **PublicError**, over every `FailureRef` and `PreparedCandidateError` variant: preparation; native `{run_ref}` when a Run exists; D38's `capture` before any Run; proof, values, abandoned, numeric (with the actual optional cause), observable and g5a.
- **Outcome reasons.**
- **A failed verification's reason** (C1 §1 item 2). A Rejected verification record is refused as association, because the reader requires `failed`.

**Sites replaced:**
- Outcome reasons, check failures, lane errors, group refusals, build reasons, BlockRefusal and section arithmetic. None of these is untranslated any more.
- **Moved off `Untranslated`:** DirectionalSpringAction is now association (C2); an out-of-table trigger, station or property is now `encoding` (C1 §4: "unknown variants fail encoding").

**Wire-less variants.** Six variants have no wire form, so each records a typed failure:

| Variant | Typed failure | Basis |
|---|---|---|
| `WideError::CountRange` | encoding | C1 §4 |
| `WideError::WorkAccounting` | encoding | C1 §4 |
| `SourceError::ZeroDirection` | association | C2 §3 ("impossible … typed internal association failure") |
| `SourceError::NonFiniteDirection` | association | C2 §3 |
| G5a `quantity_kind ∉ {0,1}` | encoding | — |
| a section property index ≥ 5 | encoding | — |

**One lossy mapping, from the closed schema:** `UnresolvedReason::WorkAccounting{fault, prior}` emits only `{fault}`, because the accepted schema has no `prior` member. I flag it for ROOT.

**The corpus** is `u1g2_translation_corpus`, with 266 entries across 20 `$def`s. Its sha256 `4489a9df…c6f6` is pinned. In the Python lane, the 260 encodable values validate against their schema `$def` (jsonschema Draft 2020-12): **0 invalid** (`_run_records/corpus_validation.txt`).

**Still `Untranslated`; not reached by the prepared driver, or needing a reading:**
- initial `formation_failure` (basis_index not captured);
- `not_attempted` (cause not captured);
- W2 `failed` (ForceFailure; outside the named G-i list);
- F3 (R-b′; ruled fail-closed);
- a named or temperature material basis, and derived E/ν;
- `Unpublishable`;
- `source_decline` (S-3);
- D38 with a prepared source (S-2);
- two impossible out-of-table index fallbacks: a conversion property and an adapter event.

### Item 3, U2 on the failure path

- **FK `final_case.rs`, U2 only:**
  - `ProductCertificateSpent` now keeps the proof's anchor from the moment it exists, so it travels into both `CertifiedProductProof.work` and `ProductProofFailure.work`;
  - new `ProductCertificateSpent::owner_matches` and `ProductProofFailure::owner_matches`. Both are false before the anchor exists (a refused start).
  - 13 added lines and 1 changed line; no change to any accounting array or capacity.
- **The C3 seam** (`retained_receipt.rs::project`): any proof work, certified or refused, must belong to this capture's selected owner. Otherwise it returns `WorkAssociation`.
- **`PreparedCandidateRefusal.certificate` is private.** Callers use the accessors `certificate()` and `proof_failure()`. Three test sites were updated.
- **The serializer** refuses a refusal whose proof failure or surviving certificate is foreign, with `association` at `product_attempts[].proof.owner`. It checks this before any projection.
- **`I61_FOREIGN_OWNER` is deliberately flipped.** The seam now refuses the foreign proof failure with `Err(WorkAssociation)`; the same failure still projects against its own owner. Tests: `u2_failure_path_foreign_owner_refused` (swapped refusal causes) and `u2_failure_path_foreign_certificate_refused` (a commit refusal's surviving certificate, swapped).

### Item 4, the unavailable representation (`serialize_unavailable`; not a publication under T3)

- **Inputs:** `Refused::{Preparation, Native, Candidate}`, the three actual refusal owners.
- **The envelope:** successor identity and profile; no method token; the legacy disclosure kept and referenced (C2:160 with D39); one `RETAINED_PRECISION_UNAVAILABLE` info diagnostic with fixed product text (`diagnostic:retained-precision:{case}:unavailable`).
- **The case:** `reason {code, phase, cause: prepared_product_failure}`, following the readers' accepted D4d table (S06):
  - `source_unavailable/preparation` for a preparation refusal, or a capture failure without a Run;
  - `kernel_<terminal>/kernel` for a non-selected Run;
  - `facade_certificate/facade` after a selected Run.
- **D38:** `run:null`, `run_ref:null`, `source_ref:null`, no execution entry, `charged:0`, and `{kind:"capture", cause:<the actual CaptureError>}`.
- **The run, when one exists:** `kernel_terminal` from the actual ExecutionOutcome (selected, refused or unresolved, with its translated reason). The CaseSource is that of the selected owner, or the prepared source.
- **Tests:**
  - `u1g2_preparation_refusal_representation`;
  - `u1g2_d38_failure_before_any_run` (both modes);
  - `u1g2_candidate_refusal_after_selected_run` (`abandoned` and `values`);
  - `u1g2_kernel_terminals`;
  - `u1g2_failed_verification_keeps_its_reason`;
  - `u1g2_source_constructor_refusal_fails_closed`.
- **Not exercised:** a non-selected native Run. PP cannot reach one; the kernel terminal mapping itself is unit-tested.

### Item 5, mutants (`_run_records/mutants_summary*.txt`; disposable `mut/` tree)

**The final run on the frozen candidate: 46 of 48 killed, none by a compile error** (`mutants_summary.txt`).
- The 39 grant-2 mutants: 37 killed.
- RV82's survivors and S2: 9 of 9 killed (see "RV82's grant-1 findings").

`mutants_summary_run1.txt` is the first full run, before the U04 test and the RV82 repairs existed.
- **T01–T28,** one substitution per translation table: 27 killed. Most are killed by the pinned corpus; T06, T08, T09, T22, T25 and T26 are also killed by behavioural tests.
- **U01–U04,** the seam check, the FK anchor and the two serializer bindings: all killed. U04 needed `u2_failure_path_foreign_certificate_refused`.
- **V01–V07,** the unavailable and D38 paths: 6 killed.

**The two that survive cannot be constructed from PP:**
- **T27,** SectionError arithmetic. `PreparationArithmeticCause` has a private field, so the cause is FK-constructed only.
- **V06,** D38 with a prepared source (S-2). It needs a kernel origin fault seam.

## Controls

1. **Ordinary bytes.**
   - **PP suite** (lib plus 21 integration targets): 648 passed, 1 failed, 1 ignored. Against base (NUM `43a6368c21`, 618/1/1) no outcome is removed or changed; the 30 additions are grant 1's 16 and grant 2's 14 tests, all ok.
   - **runner/headless:** identical to base, 85 passed / 2 failed.
   - **The failures are the known Mac platform tests:** t13 in PP and the two load_reference tests in runner/headless.
   - **FK `--lib`:** 480 passed, 1 ignored, identical to grant 1. final_case.rs has not changed since that run.
   - `u1_ordinary_bytes_unchanged_under_capture`, `u1_load_row_case_capture` and `u1_already_sensitive_finding_is_captured_undisclosed` pass.
   - **Production build warnings:** base 9, candidate 8; the one removed is grant 1's `RecoveryFailure.stage`. No new warning.
2. **The pinned successor bytes are unchanged.** `u1_milestone_successor_both_modes` passes with grant 1's pinned values:
   - sparse: file `ac6986b0…`, receipt `efc1a39b…`;
   - dense: file `6cd1d249…`, receipt `3e26499f…`.
   
   This holds after the G-i rewiring and the refactor into shared helpers (`one_case`, `one_run`, `kernel_outcome`, `run_value`, `invocation_arrays`, `ordinary_value`, `finish`, `bind_preparation`). The readers' milestone results are therefore unchanged from grant 1.
3. **The readers on the unavailable representations:** see S-1.
4. **Nothing weakened.** The only test-assertion change is the deliberate `I61_FOREIGN_OWNER` flip ordered by the grant. No reader, schema, fixture, dispatch, retained_memory.rs or production caller changed, and lib.rs is unchanged in grant 2.
5. **Mutants:** as above.

## Changed files (sha256 of the final bytes; base `59a5de2032`)

| sha256 | File |
|---|---|
| `ebc6ed0c5a7a6a424f990a4b497b05b440c82d6cb6ad1de514b324ced001e67a` | P/core/product_physics/src/retained_wire.rs |
| `07701f7572289a065afa6f05955faa9304cbf87a4e18332e193f8670a2d4ef06` | P/core/product_physics/src/retained_wire_tests.rs |
| `1aab153ff0309fdf6f30658c1c98ba31d3d086fd289c44e0866c89207e42fa4d` | P/core/product_physics/src/retained_product.rs |
| `6ff9fd75df696dd3633555eb4d450c8cc985d90da63dd10cdd54112c53ad7141` | P/core/product_physics/src/retained_product_tests.rs |
| `2f48c0c5afbcc01e0e15e00a8d7ef2c5b5d1d8586ef20cc7a0883d52ce6f7459` | P/core/product_physics/src/retained_receipt.rs |
| `d4dbaf3cb455dd50c33162de0d2231a669aaa11d00ce6a81fa51227c6e6e3689` | FK/structural/retained/product_certificate/final_case.rs |

**Diff:** 6 files, +1335/−149 (`_run_records/candidate_tracked.diff`). lib.rs is unchanged in grant 2.

## RV82's grant-1 findings, repaired (each reported separately)

**S2, the invocation binding (SHOULD-FIX). Repaired.**
- `ProductCapture::invocation` now records `invocation_digest`, the observed invocation's `source_blocks_invocation_v1` digest, next to `invocation_mode`. No adapter event was added, so the receipt bytes are unchanged; the 64-byte copy is for U4's M, with F4.
- `one_case`, which both serializers use, refuses `association` at `invocation` unless the supplied invocation's digest equals the recorded one.
- **Test:** `u1g2_foreign_invocation_refused` uses same-mode invocations with a different case **label**, a different **project id** and a different **load magnitude**. All three are refused on `serialize_selected`; the label case is also refused on `serialize_unavailable`.
- **Mutants:** S2 (check removed) and S2b (digest not recorded) are both killed.
- **One grant-1 test changed.** `u1_refusals_are_typed` used a foreign two-case invocation to reach the Scope branch. Its expectation changes from Scope `cases` to Association `invocation`, because S2 now refuses the foreign invocation first. That is stricter, not weaker. The Scope branch remains for a candidate's own invocation, which the one-case prepared driver cannot produce.

**S1, F1 in both directions (SHOULD-FIX). Repaired.** `u1_load_row_case_capture` now asserts `seed.d5_diagnostic_ref == None`; its case has no K-D5 line. RV82's R06 (forcing `d5_line = true`) is killed by that test.

**N1, committed negatives for the six defensive checks. Done.** All six are killed:

| RV82 mutant | Check | Killed by |
|---|---|---|
| R08 | after = before + increment | `u1g2_run_and_body_charge_conservation_negatives` |
| R09 | body `charged` = final after | the same test, using a fresh invocation meter with the policy limit |
| R10 | the G4 guard (a doctored second legacy disclosure naming the case) | `u1g2_t1a_guard_negatives` |
| R14 | the omitted-code check (the captured id re-coded) | `u1g2_t1a_guard_negatives` |
| R13 | report outcome = assessed quality | `u1g2_initial_quality_crosscheck_negative` |
| R22 | the 2^53−1 boundary (exactly 2^53−1 is in range; 2^53 refuses) | `u1g2_safe_range_boundary` |

R08 needed one change. `RunWork` cannot be constructed from PP, and a real run is always self-consistent. So the after-check is re-expressed as the pure helper `after_conserved`, which the test calls directly. RV82's literal patch no longer applies; its equivalent, `R08_run_after_check_removed_reexpressed`, is killed.

**N6, the "last member" comment. Fixed.** It now says the member is key-sorted between `results` and `status`, and that every hash is canonical.

**N7, the encoder's precedence. Documented.** `Enc` returns the first failure it recorded. A structural early return from a later step takes precedence over failures recorded before it. Either way the cause is typed and in C1:68's vocabulary, and nothing is emitted.

**N3, the B′ comment. Corrected.** The control comment now names A and B only; B′ belongs to U3.

**Not repaired here, as routed:** N5 (to I65 G4); N8 and N9 (wider F2a); N2 (equivalent; none).
