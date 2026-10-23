# B8: the desktop precision check

This design activates the retained-precision method on the desktop (B8). Paths are relative to `projects/chirality-piping/`. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

**Owner ruling, 2026-10-23:** "R1: change the semantics so users are confused, the concept is worth doing now. Don't study it that won't tell me anything new that your theory doesn't already make clear. It's precisely the rare cases that really matter and you might not find one or take forever to find one, so just implement it." Corrected the same day: "I should have said \"so users aren't confused\"".

The owner accepted Agent 0's framing:

- **The usual case.** In a well-conditioned model a verified bound is extremely tight. It acts as a near-always-green numerical health check: "these results are numerically sound to at least N significant figures", backed by a guarantee.
- **The rare case.** A failed or unestablished bound is a strong signal that the ordinary numbers should not be relied on. The cause is often a modelling problem.
- **Presentation.** The outcome is a per-result indicator with a clear warning on failure. It is not ± ranges on every number, and it never reads as engineering, code or professional verification (PRD §12.8, §21.2).

Agent 0, 2026-10-23, on this design:

- **The trigger.** "Proceed with the caller-selected trigger as recommended".
- **Method-agnostic presentation.** The indicator, its states (including the grey "not available for this model (reason)") and the output surfaces must not depend on which verification method produces the bound. A later scalable method plugs into the same states and wording.
- **No domain widening.** It is Decision 22.

No measurement study is part of this unit.

## 1. Premises this design rests on

1. **The current trigger skips well-conditioned cases.** The retained route attempts only cases whose ordinary verdict is not `checks_passed` (`core/product_physics/src/lib.rs` `case_triggers`; CLM-001 "Trigger"). The reader enforces this too: a `selected` case must have ordinary quality `sensitive`, `unresolved` or `failed` (`core/reporting/result_export/src/retained_precision.rs` G5, "D6b").

   Without a change, green could only mean that the ordinary residual and conditioning screens passed (`accuracy_evidence: not_claimed`), which carries no guarantee. §3.1 adds a caller-selected trigger.
2. **The reach is small.** The D1 domain allows at most:
   - 32 nodes, 32 members and 32 supports;
   - 4 materials;
   - 3 load cases, with at most 128 loads each;
   - 2 combinations.

   On route E it also needs nodal loads only and explicitly empty pressure regions, with no combinations. Route L needs a pressure-free model. Most production models are therefore outside the method's domain. Their indicator reads "not available for this model", with the reason, and is grey rather than alarming. Widening the domain is Decision 22 and is not pursued.
3. **The product build is unregistered.** The desktop release build is unregistered and therefore `Stale`. Its identity is opt-level 3, no debug assertions, `panic=unwind`, no RUSTFLAGS (`apps/desktop/src-tauri/Cargo.toml` has no profile overrides). The single registered profile is the debug build (`retained_memory.rs` `REGISTERED_PROFILES`). §6 qualifies the product build.
4. **The pipeline exists but is unused.** The desktop already reads successors through the wasm Rust reader. It registers them against a live capture (`previewService.ts` → `registerRetainedPrecision`). Result export and stress-neutral export already carry the receipt and class disclosure.

   None of it is reached:
   - no job calls the Direct entry;
   - `physics-retained-1` is missing from both fresh-identity sets (TS `knownSemanticLimitations.ts` `FRESH_SEMANTIC_CONTRACT_IDS`; Rust `semantic_contract.rs` `FRESH_IDENTITIES`), so `currentSolvedResult` is always null for it (DEL-04-07 Design `desktop-exact-outputs.md` §9).

## 2. The flow on the desktop

1. **The ordinary solve is unchanged.** It runs as today, in the capped worker. Its result becomes current and is displayed at once. The precision check never delays it.
2. **The check starts automatically.** When an ordinary solve completes with a result, the desktop starts one precision-check job for the same raw request bytes, mode and budget.

   Lifecycle:
   - Starting a new solve, editing the model or resetting the session cancels a running check (the worker is killed and its lease is released).
   - At most one check runs at a time.
   - A check never counts against the ordinary queue's limit of 4. It is a separate job kind with its own limit of 1 queued or running check.
3. **Decline before work.** The worker first calls a census-only entry, `retained_direct_admission` (§3.2).

   This entry:
   - parses the captured invocation;
   - applies the D1 law and the build status;
   - applies the bound at L = min(per-solve budget, the registered profile's M);
   - does not solve.

   If it declines, the job returns the decline reason and no publication. For a large model this costs one parse and returns in milliseconds. Nothing is re-solved.
4. **The check itself.** If admitted, the worker calls `run_linear_static_preview_value_with_retained_direct` with the same L and `RetainedTrigger::EveryCase` (§3.1).

   The job returns two things:
   - the method's single publication: the successor, or the ordinary bytes with at most one notice per attempted case;
   - an outcome frame outside the wire (§3.2).

   The worker's allocator cap, growth steps and pool admission are the ordinary job's. A worker resource refusal ends the check with outcome `incomplete` (§4).
5. **Adoption.** When the check returns a publication and the session's current result still comes from the same request digest, that publication becomes the current result. It is registered against the live capture as today, and it is what every output surface reads.

   - **Successor.** Its covered values replace the ordinary values; this is the publication PRD §12.8 defines.
   - **Ordinary bytes with the notice.** The values are byte-identical to the ordinary run's (same invocation, deterministic route). The publication adds the one information notice per attempted case.
   - **Decline.** No publication is adopted, and the ordinary result stays as it was.
6. **Leaving the session.** A result saved and reopened, or one whose invocation is no longer live, keeps today's standing (`needs_recompute`). Its indicator reads "Precision check: not current". Nothing is revalidated from saved bytes as if it were live.

Cost:

- **Large models.** Declined at the census, so they cost one parse.
- **In-domain models.** Run in the background and can be cancelled.
- **Memory.** The check runs under the same per-solve budget and pool as the ordinary solve. Its admission uses the registered profile's priced requirement, which is accounting, not measured memory. Its real footprint is bounded by the worker cap. On a loaded machine the pool can refuse growth, and the check then reads "did not complete".
- **Timing.** No timing is measured here (heavy lane). The design does not depend on it, because the check never blocks the ordinary result.

## 3. Product changes in `core/`

### 3.1 The caller-selected trigger

- **The entry.** `run_linear_static_preview_value_with_retained_direct` gains a `RetainedTrigger` argument with two values:
  - `OrdinaryNotPassed`: today's rule, used by every existing test and by the corpus generators;
  - `EveryCase`: every requested case that DN §4.3 does not exclude is attempted, `checks_passed` and `not_assessed` included.

  Excluded cases are an ordinary Mechanism, Asymmetric or InvalidInput failure that W2 did not publish. They stay excluded under both values.
- **The receipt.** It records the trigger in a new required field, `retained_precision.body.trigger`, with the value `"ordinary_not_checks_passed"` or `"every_case"` (schema `retained_precision_mp_v2.schema.json`).
- **Reader G5 (D6b).** Under `every_case` a `selected` case may have any ordinary quality, and no case is `not_required`. Under `ordinary_not_checks_passed` the rule is unchanged. The Python verifier applies the same rule.
- **Committed successors.** All of them gain `"trigger": "ordinary_not_checks_passed"` and are regenerated by their existing writers.
- **The corpus.** `fixtures/results/retained_precision_cases.json` gains `every_case` entries:
  - a selected `checks_passed` case on route L and on route E;
  - a refused mutation of each of the following:
    - the trigger value;
    - a `not_required` case under `every_case`;
    - a `checks_passed` selected case under the other trigger.
- **What does not change.** No gate, tolerance, precision ladder or limit changes. The D1 domain and its caps, the formation definitions and the 10^-9 rule are unchanged.
- **Pricing.** Pricing already prices every phase at C = 3 attempted cases, so attempting `checks_passed` cases adds no new resource phase. D3 confirms this (§6).

### 3.2 Public decline and outcome facts

Neither fact is serialized into any published document.

**`retained_direct_admission(&Value, PreviewSolverMode, RetainedByteLimit) -> Result<(), RetainedDecline>`** runs the same parse and `admit` as the Direct entry, without the solve. `RetainedDecline` is a coarse, method-agnostic reason, with the D1 clause id kept for diagnostics:

- `Build`: the build is not qualified (D1.1, `Stale` or `Missing`).
- `Size { fact, observed, cap }`: a D1.9 cap.
- `Feature(token)`: a D1.3–D1.8 family fact, mapped to a stable token, for example `pressure_regions`, `curved_members`, `nonlinear_supports`, `directional_springs`, `imposed_motion`, `member_loads_on_pressure_route`, `combinations_on_pressure_route`, `components`, `equivalent_static`, `analysis_state`, `modulus_temperature` or `load_reference_states`.
- `Budget { required }`: the bound, or a limit above M refused as `Unqualified`.
- `Other`: census, provenance or control bytes.

The mapping is total over `AdmissionRefusal`. A test enumerates every `D1Clause` and `FamilyFact`.

**`RetainedPreviewOutput::outcome() -> RetainedOutcome`** is public and classifies the existing private `retained` result:

| `RetainedOutcome` | Source | Indicator (§4) |
|---|---|---|
| `Published` | a successor | per-case from the receipt |
| `NotEstablished` | `W1Fallback::Native`, `Candidate`, `Preparation` | not confirmed |
| `MethodLimit` | a single-case facade or sharper-allowance refusal (REQ-006 (e), (f)) | not available: method limit |
| `Incomplete` | `Staging`, `Serializer`, `Precommit`, `CombinationCustody`, `LateGate`, `CompleteGate`, `StackReservation`, `NoticeReservation`, `PermitUnbound` | did not complete |
| `NotApplicable` | `Domain`, `NoTriggeredCase` | not available |
| `OtherMethod` | `Coexistence` (a pressure-source-1 publication) | per-case from the source receipt |

Before implementation, the implementer checks each `W1Fallback` path that can carry a REQ-006 (e) or (f) refusal. Where one shares a variant with a kernel failure, the variant is split, so that `MethodLimit` is never shown as `NotEstablished`. A facade test pins each row.

Inside a published successor, a case or combination that is not selected takes its state from the receipt:

| Receipt | State |
|---|---|
| `unavailable` with phase `kernel` | not confirmed |
| `unavailable` with code `facade_certificate` | not available: method limit |
| `unavailable` with phase `receipt`, `routing` or `preparation` | did not complete |
| combination `retained_unavailable` | not confirmed |
| combination `ordinary` | not checked |
| combination `base_withheld` | the existing withheld text |

### 3.3 TBD-001 and TBD-002 (reviewed inputs; D3 applies)

**TBD-001's direction.** User-facing product text no longer uses "certified", "certificate" or "certify". The formation definitions and the tables' `formation_warrant` keep "certificate" as the technical name of the kernel's error proof (REQ-008, AX-007). Identifiers keep their names.

| Text | Old | New |
|---|---|---|
| `retained_wire.rs` `SELECTED_MESSAGE` | "…binds their certified classes, the native attempts and work, and the ordinary-attempt evidence." | "…binds their verified classes, the native attempts and work, and the ordinary-attempt evidence." |
| `SELECTED_MEMBER_LOAD_MESSAGE` | "…relative to that value; the certificate certifies that declared-load problem." and "…binds their certified classes…" | "…relative to that value; the verified bounds apply to that declared-load problem." and "…binds their verified classes…" |
| `COMBINATION_SELECTED_MESSAGE` | "…from its own solve and certificate. …binds their certified classes…" | "…from its own solve and error proof. …binds their verified classes…" |
| `preview-physics-retained-1` table, member-load limitation | "Retained-precision certificates certify that declared-load problem." | "Retained-precision bounds apply to that declared-load problem." |
| `pressure-source-1` table, maxima limitation | "…and do not certify primitive geometry or material accuracy." | "…and do not bound primitive geometry or material accuracy." |

The three copies of the selected member-load text change together:

- the reader's G4 copy (`result_export/src/retained_precision.rs`);
- `tests/verifiers/retained_precision.py`;
- the pricing tool's regex model (`tools/retained_pricing/base/w1_member_load_model.py`).

The following are regenerated by their writers:

- the committed successors;
- the pressure-source reader corpus;
- the J7 census envelope and diagnostics digests, under AC-002's stated product-text clause.

The table digests in `semantic_contract.rs` and the twelve reviewed-input digests move. The text lengths are priced, so D3 re-prices them.

**TBD-002's repair.** In the `physics-retained-1` table's `accuracy_classification.scope`, "model 0.3.0 with the 2.0.0/exact_straight_pressure_v2 contract" becomes "model 0.6.0 with the 3.0.0/exact_pressure_v3 contract". The scope then matches DEF-E and the code. It is the same reviewed input and goes through the same D3.

### 3.4 Fresh identities

`physics-retained-1` joins `FRESH_IDENTITIES` (Rust `semantic_contract.rs`) and `FRESH_SEMANTIC_CONTRACT_IDS` (TS). The two sets stay equal, as their comment requires. The report package refuses it as it refuses every fresh identity.

## 4. What the user sees

### 4.1 One indicator, method-agnostic

The desktop forms one view model per result set (load case or combination): `PrecisionCheck { state, method, relativeLimit, counts, reason, largestChange }`.

Adapters fill it from:

- a validated retained successor (MP-v2 receipt);
- a validated `pressure-source-1` receipt (its covered quantities' intervals with `relative_limit` 10^-9);
- the job's outcome or decline (§3.2);
- a non-live or saved result.

The UI, the exports and the surface policy read only `PrecisionCheck`. A later scalable method supplies another adapter and reuses every state and text below. The `method` field is shown only in the details.

### 4.2 States and exact wording

The indicator's label is **"Precision check"**. Each result set shows one state:

| State | Badge | Headline |
|---|---|---|
| checking | neutral, spinner | "Precision check: running" |
| confirmed | green | "Precision check: confirmed" |
| not confirmed | red | "Precision check: not confirmed" |
| not available | grey | "Precision check: not available for this model" |
| did not complete | grey | "Precision check: did not complete" |
| not current | grey | "Precision check: not current" |

The result header shows the worst state over its result sets, in the order not confirmed, did not complete, not available, confirmed. The order of the two grey states does not change the badge, but it keeps the headline deterministic. The shell status chip carries the same badge. Expanding the indicator shows each result set's state and details.

**Detail texts.** Values in braces are filled in.

- **checking:** "The results shown are from the ordinary solve. A second, extended-precision solve is checking them in the background."
- **confirmed:** "Every checked value agrees with the exact solution of the model as entered to within 1 part in 10⁹ (about 9 significant figures). Values near zero are instead within the small absolute bound shown beside them. This checks the solver's arithmetic only. It says nothing about whether the model, its inputs or its design are right."
  - When some quantities are not covered: "{n} quantities are outside what this check covers and keep their ordinary values; they are marked "not checked"."
  - When the confirmed values differ from the ordinary solve's by more than 10⁻⁶ relative on any covered row: "The confirmed values replace the ordinary solve's values. The largest change was {x} % in {row label}."
- **not confirmed:** "The extended-precision solve could not establish that these results are accurate to 1 part in 10⁹. Do not rely on these values until the cause is understood. This usually points to the model: a mechanism or weakly restrained direction, a very short or very stiff element next to flexible ones, or stiffnesses that differ by many orders of magnitude. Review the model and the solver diagnostics, then solve again. The values shown are the ordinary solve's."
- **not available:** "This check runs only on small linear models. {reason}" The reasons are:
  - `Size`: "This model has {observed} {fact}; the check supports at most {cap}."
  - `Feature`: "This model uses {feature}, which the check does not support."
  - `Build`: "This build of the application is not qualified to run the check."
  - `Budget`: "The per-solve memory budget is below the {required} the check needs."
  - `MethodLimit`: "Some values of this result are too close to zero or too sensitive to section rounding for the check to state a bound in its fixed form. This is a limit of the check, not a finding about the model."
  - `Other`: "The model is outside the check's supported range."
  - The solve failed before any check: "The ordinary solve did not produce a result for this case."
- **did not complete:** "The check stopped before reaching a conclusion ({cause}). The ordinary results are unchanged. Try again with Check precision, or solve again." The causes are:
  - "not enough free memory at the moment";
  - "it was cancelled";
  - "an internal check failed".
- **not current:** "This result was not checked in this session, or the model has changed since. Solve again to check it."

**Actions.** "Check precision" retries after a did-not-complete state. "Cancel check" stops a running check.

**Per row.**

- A not-covered row of a successor keeps today's label, "not checked", replacing the "Uncovered quantity" prefix in `N_RP_NOT_COVERED`.
- An absolute-bound row keeps `N_RP_ABSOLUTE`'s bound.
- No ± range is shown on relative rows.

**Words the indicator never uses:** "certified", "verified" in a headline, "valid", "approved", "compliant" or "correct". The detail texts use "confirmed", "established" and "accurate to" (the K-D5 R5 vocabulary). Every literal lives in one TS module, `precisionCheckText.ts`. GEN-13 scans that module, and `tests/test_claims_language_surface.py` pins its texts.

### 4.3 Claims registry

The indicator is a numerical-quality indicator, not one of the six analysis statuses or the evidence-status labels of registry §2.1. The registry gains one entry, `NQ-PRECISION`. It holds:

- the six display forms above, as the only forms;
- the boundary sentence "This checks the solver's arithmetic only. It says nothing about whether the model, its inputs or its design are right.", which appears with every confirmed state.

This is owner text (§8).

## 5. Output surfaces

**The rule.** A retained result is admitted on exactly the surfaces that admit its base identity:

| Retained identity | Base identity |
|---|---|
| `preview-physics-retained-1` | `preview-physics-2` |
| `physics-retained-1` | `pressure-2` |

It is admitted only at the Rust reader's numerically eligible standing with the live capture. Every document that carries result values also carries the precision-check line (§5.1). The report package keeps refusing every fresh identity.

| Surface | `preview-physics-retained-1` | `physics-retained-1` | Change |
|---|---|---|---|
| result-export, stress-neutral | admitted when eligible | admitted when eligible | none (receipt and class disclosure exist) |
| rule-check | admitted when eligible | admitted when eligible | policy cell; Tauri `is_fresh_identity` admits `physics-retained-1` (the run path already reads row classes) |
| pcf-export, caepipe-mbf, caepipe-external, export-adapter-sdk, adapter-framework, external-prover, missing-data, design-workspace, solve-job, report-lint, export-review | admitted when eligible | refused, as `pressure-2` | `LoadReferenceOutputGate` reads standing (it treats `admitted_when_eligible` as refused today) |
| handoff, local-fea, native-package, report, rendered-report, headless-runner | admitted when eligible, with the precision-check line | refused, as `pressure-2` | the line in each document; `RenderedReportPanel.tsx:70` uses `surfaceOutputRefusal` |
| report-package | refused (`REPORT-PACKAGE-FRESH-RESULT-UNAVAILABLE`) | refused | none |

**What this changes.** It retires REQ-006 (c)'s rule that "every output surface refuses it (`RETAINED-PRECISION-OUTPUT-NOT-YET-AVAILABLE`)". A refused cell now uses the base identity's refusal text, so a user never loses an output by having a result checked. The 11 surfaces that admit on route L carry no result values (ids, counts and diagnostics only). They are gated only because the gate is per route.

### 5.1 Exports and the result JSON

- **Result JSON.** The adopted publication itself: either the successor with its `retained_precision` receipt, or the ordinary bytes with each attempted case's `RETAINED_PRECISION_UNAVAILABLE` notice. Wire formats are unchanged apart from §3.1's `trigger` field.
- **Stress-neutral export.** Today's class disclosure, unchanged.
- **Value-carrying documents** (handoff, local-fea, native-package, report, rendered-report, headless-runner). One line: "Precision check, per case: {id}: {state}{, n values not checked}{, a values to an absolute bound}." It is formed from `PrecisionCheck` and placed with the document's existing notices.
- **Exports of a non-confirmed result.** These are not blocked. The line states the result's state, so an exported report of a result that was not confirmed says so.

## 6. Qualifying the product build (D3 on the product build)

The registration for the desktop's release identity is a second `REGISTERED_PROFILES` entry. It has the same M (12 GiB), margins, TEXT reserve and R. Its identity is:

`profile=release; opt_level=3; debug_assertions=false; panic=unwind; rustflags=` (empty), with rustc 1.97.1 on aarch64-apple-darwin and `pkg=open_pipe_stress_product_physics@0.2.0`

Steps, on the final priced source, through `t3_slot.sh` (allocation counts and stack witnesses, no RSS or timing):

1. **Read the product's identity from the actual build.** Build the desktop release binary (`tauri build` through the wrapper). A test-only Tauri command, under the existing `native-smoke` feature, reads `retained_profile()` and the compiled identity. The registered identity is that value. No value is derived from Cargo defaults.
2. **Collect the in-build atoms and reader layouts** in the release identity: `cargo test --release --lib profile_` in `core/product_physics`, with the same identity checked in the run's output. Compare them with the debug atoms. A differing atom is evaluated, never assumed.
3. **Run D3** on the final source (`tools/retained_pricing run` and `generate --check`), with all 80 budget evaluations at M = 12 GiB.
4. **Run the in-build checks** in the release identity:
   - the phase record and the law tests;
   - the memory challenge (`retained_direct_peak_is_within_the_profiles_bounds`, which counts allocations);
   - the six stack witnesses at R/64.

   The release record is pinned beside the debug one (`PINNED_RECORD` per identity).
5. **Bind the desktop lock.** A maintained test checks that every package in `core/product_physics/Cargo.lock`'s resolved closure has the same version and checksum in `apps/desktop/src-tauri/Cargo.lock`. Pricing reasons about the PP lock's dependency sources. The desktop resolves from its own lock, which the reviewed inputs do not bind.

**Tests to generalize.** The tests that assume one profile are generalized to "each registered profile":

- `retained_memory_law_tests.rs:836-841`;
- the hard-coded index-0 assertions;
- `registered()`'s literal identity in `retained_facade_tests.rs` and `tests/retained_precision_admission.rs`.

Each registration-dependent test asserts the registered branch in either registered identity and `Stale` elsewhere, as today.

**Stop condition.** The design expects the release build to fit:

- the priced atoms are type layouts, which opt-level does not change;
- release stack frames are smaller than opt-level 0's.

If any phase in either mode exceeds ⌊0.9 M⌋ with R and the TEXT reserve, or any stack witness fails at R/64, registration stops. It goes to the owner as Decision 22. No margin, R, M or cap changes to make it fit.

**Upkeep.** A rustc upgrade, a RUSTFLAGS change or a priced-source change makes the product build `Stale` until it is re-registered. The indicator then reads "not available … not qualified", and the ordinary route stands.

## 7. Verification

- **Core.** The affected crates' full suites (`product_physics`, `result_export`, `operation_applier`, `headless` runner) through the wrappers, in the debug registered build, the release registered build and an emulated unregistered build, with main comparisons. Also:
  - registered `piping-pytest` in full, with the five binary presets rebuilt for the head;
  - VER-001 to VER-006 on the final head;
  - D3 (`run`, `generate --check`).
- **Facade tests:**
  - `EveryCase` selects a `checks_passed` case on route L and route E;
  - `OrdinaryNotPassed` bytes are unchanged against the committed successors (apart from §3.1's `trigger` field and §3.3's texts);
  - each `RetainedOutcome` row;
  - the decline mapping's totality;
  - `retained_direct_admission` agrees with the Direct entry's admission on the corpus inputs.
- **Desktop.** Full `desktop-test` with the wasm engine rebuilt, plus:
  - `precisionCheck` adapter tests over the retained and pressure-source corpora;
  - state and wording tests for every state;
  - output-policy tests for every cell in §5;
  - job lifecycle tests (cancel on edit, adoption only on a matching digest, decline without a publication);
  - the GEN-13 lint.
- **The open path changes,** because a new job kind is added to the native solve. So the native smoke and `npm run test:e2e:dist` run on the final head:
  - the native smoke observes a confirmed check on a small invented model (route L);
  - the native smoke observes "not available" on the larger milestone model;
  - the dist e2e asserts the grey "not qualified" state of a hosted, unregistered build.
- **Review.** Dual review of this design. On the implementation's final head, dual review with independent numerical scrutiny: an independent Claude reviewer and Codex. Codex also independently reads the registration and D3 evidence.

## 8. Commitment text for the owner

The exact old→new texts go to the owner in one bundle with this design's reviewed head:

- **DEL-04-08 ScopeOfWork:**
  - CLM-001 "Trigger" and "Entry";
  - CLM-003's successor reach;
  - REQ-003 (a);
  - REQ-004's registration bullets and AC-004 ("in each registered build identity");
  - REQ-006 (c)'s successor bullet (§5's rule);
  - a new REQ-009, AC-009 and VER-009 for the desktop precision check (§2, §4, §5);
  - REQ-008's texts;
  - TBD-001 and TBD-002 resolved;
  - AX-004 without B8.
- **PRD §12.8**'s last bullet.
- **Claims registry:** `NQ-PRECISION` (§4.3).

Design-only consequences are reconciled in the implementation PR without further approval:

- DEL-04-07's `deliverable.yaml` condition, "retained-precision results reach no desktop solve until B8";
- DEL-07-11's scope-fence note;
- DEL-04-07 `desktop-exact-outputs.md` §9's B8 observation;
- the `outputPolicy.ts` B8 comment.

## 9. What stays unchanged

- Every gate, tolerance, precision ladder and limit; the D1 domain and caps; M, the 0.9 M margin, the TEXT reserve and R.
- The ordinary route's bytes and the ordinary solve's timing.
- Retained-source recovery's method and limits.
- Frozen references under `validation/references/**`.
- The report package's refusal.
- G10 and the frozen UI benchmark.
