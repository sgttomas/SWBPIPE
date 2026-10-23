# B8: the desktop precision check

This design activates the retained-precision method on the desktop (B8). Paths are relative to `projects/chirality-piping/`. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

**Owner ruling, 2026-10-23:** "R1: change the semantics so users are confused, the concept is worth doing now. Don't study it that won't tell me anything new that your theory doesn't already make clear. It's precisely the rare cases that really matter and you might not find one or take forever to find one, so just implement it." Corrected the same day: "I should have said \"so users aren't confused\"".

The owner accepted Agent 0's framing:

- In a well-conditioned model a verified bound is extremely tight. Its value is a near-always-green numerical health check: "these results are numerically sound to at least N significant figures", backed by a guarantee.
- The rare failure is a wide or unestablished bound. It strongly signals that the ordinary numbers should not be relied on, and the cause is often a modelling problem.
- The check is shown as a per-result indicator with a clear warning on failure, not as ± ranges on every number. "Verified" must never read as engineering, code or professional verification (PRD §12.8, §21.2).

Agent 0, 2026-10-23:

- "Proceed with the caller-selected trigger as recommended".
- The indicator, its states (including the grey "not available for this model (reason)") and the surfaces must not depend on which verification method produces the bound. A later scalable method plugs into the same states and wording.
- Domain widening is Decision 22 and is not pursued.

No measurement study is part of this unit.

Revision 2 answers the first dual review of `bd58241f14`. It makes three changes:

- The check is attached to the ordinary result instead of replacing it (§2).
- States are classified from typed causes, per case (§3.2).
- The release build keeps the proof crates' debug assertions and overflow checks (§6).

## 1. Premises

1. **Trigger.** The retained route attempts only cases whose ordinary verdict is not `checks_passed`.
   - In code: `core/product_physics/src/lib.rs` `case_triggers`, and CLM-001 "Trigger".
   - The reader enforces it: a `selected` case must have ordinary quality `sensitive`, `unresolved` or `failed` (`core/reporting/result_export/src/retained_precision.rs` G5, "D6b").
   - Unchanged, a well-conditioned result would never receive a bound (§3.1).
2. **Reach.** The D1 domain allows at most:
   - 32 nodes, 32 members and 32 supports;
   - 4 materials;
   - 3 load cases with 128 loads each;
   - 2 combinations.

   Route E also needs nodal loads only, explicitly empty pressure regions and no combinations. Route L needs a pressure-free model. Most production models are outside the domain. For them the indicator is grey "not available for this model", with the reason. Widening the domain is Decision 22.
3. **Product build.** The desktop release build reports `Stale`.
   - It builds at opt-level 3, with no debug assertions and no overflow checks.
   - Its proof arithmetic carries soundness-bearing `debug_assert!`s, which release removes. Examples: `frame_kernel` `structural/retained/wide.rs`, `wide/multi.rs`, `exact_sum.rs` and `directed.rs`.

   §6 qualifies a release identity that keeps both safeguards.
4. **Existing pieces.**
   - The desktop already reads successors through the wasm Rust reader and registers them against a live capture (`previewService.ts` → `registerRetainedPrecision`).
   - `physics-retained-1` is missing from both fresh-identity sets: TS `knownSemanticLimitations.ts` `FRESH_SEMANTIC_CONTRACT_IDS`, and Rust `semantic_contract.rs` `FRESH_IDENTITIES`.
   - Whole-successor rules:
     - The reader marks a successor ineligible unless every case is `selected` or `not_required` (`retained_precision.rs:6621`).
     - Rule binding refuses a successor's `absolute_verified` and `not_covered` rows (`semantic_contract.rs:684-713`). Intensified bending stresses and section extremes are `not_covered`.

## 2. The flow: the check is attached to the ordinary result

**The ordinary result stays the current result.**

- Every output surface, rule binding and export reads it exactly as today.
- The precision check adds a statement about the values the user sees. It does not replace them.
- Consequently no output, rule binding or eligibility is lost by checking.
- REQ-006 (c)'s successor admission matters only when the user explicitly adopts confirmed values (§2.4).

### 2.1 Running the check

1. **The ordinary solve is unchanged.** It runs in the capped worker as today. Its result becomes current and is displayed at once.
2. **Start.** When an ordinary solve completes with a result, the desktop starts one check job.
   - The job reuses that solve's captured invocation (the same raw request bytes, model and mode), not a fresh capture of the current model.
   - The job carries the solve's generation token: session, solve job id and capture.
3. **Cancellation.** A new solve, a model edit, a mode change or a session reset cancels a running check.
   - The worker is killed and its lease is released after reaping, as for ordinary jobs.
   - At most one check is queued or running. It does not count against the ordinary queue's limit of 4.
4. **Decline before work.** The worker first calls a census-only entry, `retained_direct_admission` (§3.2). The entry:
   - applies the same imposed-displacement pre-parse refusal as the dispatcher;
   - parses the captured invocation;
   - applies the D1 law and build status;
   - applies the bound at L = min(per-solve budget, the registered profile's M).

   It never solves. A declined check returns only its reason. For a large model this costs one parse.

   L ≤ M by construction, so `Unqualified` cannot arise here. That is consistent with DEL-04-07's "No larger retained budget is offered".
5. **The check.** If admitted, the worker calls `run_linear_static_preview_value_with_retained_direct` with the same L and `RetainedTrigger::EveryCase` (§3.1). It returns:
   - the method's single publication: the successor, or the ordinary bytes with each attempted case's notice;
   - per-case typed outcomes, outside the wire (§3.2).

   The worker's allocator cap, growth steps and pool admission are the ordinary job's.

### 2.2 Accepting the result

1. **Generation check.** The check's result is accepted only if its generation token is still current after every await.
2. **Successor validation.** A returned successor is validated by the wasm Rust reader against the check's captured invocation. The desktop accepts it only if every check passes:
   - the invocation is the current result's captured invocation;
   - the reader validates the successor at G0–G8;
   - its row ids match the current result's rows by id.

   Otherwise the check reads "did not complete".
3. **Custody.** The successor is held as check evidence, not as a current result. It counts once against the retained-result limits (`memory_pool.rs`), and is disposed of when the check is superseded or the session resets.

### 2.3 What the indicator states about the displayed values

For each displayed value, an adapter supplies an upper bound E on |displayed value − exact value of the recorded problem|, or "no bound". The recorded problem is the linear problem formed from this model's recorded inputs (PRD §12.8).

**From a validated retained successor** (rows paired by id with the current ordinary rows; o = ordinary value, s = successor value):

- A `relative_verified` row has |s − exact| ≤ 10⁻⁹·|s| (REQ-001). So E = |o − s| + 10⁻⁹·|s|.
- An `absolute_verified` row has its published bound b (SI). So E = |o − s| + b, with o and s normalized to SI exactly as the reader normalizes them.
- `input_derived` rows (prescribed zeros, Lamé and section-pressure stresses, user-input reviews) are inputs and carry no solve bound. They are excluded from the figure.
- `not_covered` and `non_quantity` rows have no bound.

**From a validated `pressure-source-1` publication**, which is the ordinary result itself:

- Each covered quantity's published `absolute_error_bound` is E.
- Its other quantities have no bound.

**Arithmetic rule.** Both rules compare values in SI, normalized as the reader normalizes them. E carries one further binary64 unit in the last place of |o| for o's own normalization. Every E and every ratio is computed in binary64 with an upward-rounding margin: each sum and product is multiplied by (1 + 2⁻⁵⁰), and the result is then rounded up to the printed precision. The printed figure is never better than the guarantee.

**The per-case figure.** The relative error of a relative-class row is r = E/|o|. The per-case figure is N = ⌊−log₁₀ max r⌋, capped at 9, over every `relative_verified` row; for source recovery, over its rows with a nonzero value.

- Rows near zero (`absolute_verified`) are not in N. They are shown as "within {E} {unit}".
- When o = s, r is about 10⁻⁹, so a well-conditioned case reads 9.

### 2.4 Adopting confirmed values (explicit, rare)

When a successor is valid and eligible, and the displayed ordinary values are confirmed to fewer than 6 significant figures (or the ordinary attempt failed, REQ-003 (d)), the check offers "Use the confirmed values".

Adopting makes the successor the current result, in the existing successor path:

- It is registered against the live capture.
- The analysis record is rebuilt for it.
- Derived rule-check, proposal and report state is invalidated, and the bundle is published together, as `workspaceSession.ts` publishes a solve.

From then on:

- The figure is computed on the successor's own values (E = 10⁻⁹·|s| and b), so it reads 9.
- REQ-006 (c)'s rule (§5) and the existing successor rule bindings apply. Rows without a verified class cannot bind rule checks, as today, and the rule-check panel says why.
- Outputs made before adoption carry the ordinary values. The adoption notice says so.

A successor that is not eligible (some case not selected) is never adopted. Its selected cases still state their figure on the ordinary values.

### 2.5 After the session

A saved or reopened result, or one whose invocation is no longer live, keeps today's standing. Its indicator reads "Precision check: not current". No figure is computed from saved bytes.

### 2.6 Cost

- **Out-of-domain models.** Declined at the census.
- **In-domain models.** Checked in the background, cancellable.
- **Memory.** Admission uses the registered profile's priced requirement, which is accounting. The worker cap bounds the real footprint, and a pool refusal reads "did not complete" with a retry.
- **Timing.** Not measured here. The check never blocks the ordinary result.
- **Frozen UI benchmark and G10.**
  - The benchmark's models carry no load cases, so no solve and no check runs.
  - Browser e2e and the benchmark run without the native host, where no check exists.
  - G10's milestone observations are on the ordinary result, which is unchanged.

## 3. Product changes in `core/`

### 3.1 The caller-selected trigger

- **Two triggers.** The Direct entry gains a `RetainedTrigger` argument.
  - `OrdinaryNotPassed` is today's rule. Every existing test and corpus generator uses it.
  - `EveryCase` attempts every requested case that DN §4.3 does not exclude, including `checks_passed` and `not_assessed` cases.
- **The receipt.** It records the trigger in a new required field, `retained_precision.body.trigger`, with the value `"ordinary_not_checks_passed"` or `"every_case"`. The field is bound by the receipt hash.
- **Reader G5 (D6b).** D6b is a consistency rule keyed on that field:
  - under `every_case`, a `selected` case may have any ordinary quality, and no case is `not_required`;
  - under `ordinary_not_checks_passed`, the rule is unchanged.

  The Python verifier applies the same rule.
- **What does not change.** Every other G5 check, DN exclusions, every numerical gate, tolerance, precision ladder and limit, the D1 domain, the formation definitions and the 10⁻⁹ rule are all unchanged.
- **Schema.** The field is added to the current v2 schema in place, and committed successors gain `"trigger": "ordinary_not_checks_passed"`. Under AX-002 there is one current version, no compatibility variant and no older reader.
- **The corpus.** `fixtures/results/retained_precision_cases.json` gains:
  - a selected `checks_passed` case on route L and on route E under `every_case`;
  - refused mutations of the trigger value;
  - a `not_required` case under `every_case`;
  - a `checks_passed` selected case under the other trigger.
- **Pricing.** It already prices C = 3 attempted cases. D3 confirms that no phase moves (§6).

### 3.2 Typed decline and per-case outcomes

Neither is serialized into any published document.

**`retained_direct_admission(&Value, PreviewSolverMode, RetainedByteLimit) -> Result<(), RetainedDecline>`** runs the dispatcher's imposed-displacement refusal, the same parse and the same `admit` as the Direct entry, without solving.

`RetainedDecline` has these reasons:

| Reason | Source |
|---|---|
| `Build` | D1.1, `Stale` or `Missing` |
| `Size { fact, observed, cap }` | a D1.9 cap |
| `Feature(token)` | a D1.3–D1.8 family fact, mapped to a stable token such as `pressure_regions`, `curved_members`, `nonlinear_supports`, `directional_springs`, `imposed_motion`, `member_loads_on_pressure_route`, `combinations_on_pressure_route`, `components`, `equivalent_static`, `analysis_state`, `modulus_temperature`, `load_reference_states` |
| `Budget { required }` | the bound |
| `Other` | census, provenance or control bytes |

Each reason keeps its D1 clause id for diagnostics. The mapping is total, and a test enumerates every `D1Clause` and `FamilyFact`.

**`RetainedPreviewOutput::case_outcomes() -> Vec<(case or combination id, CaseOutcome)>`** returns each attempted result set's typed end.

- These outcomes are preserved per case before `w1_transaction` reduces failures to the furthest stage (`lib.rs:3678-3686`). One cause→state table classifies them. The same table applies to the out-of-band outcomes and to the `unavailable` reasons of a published successor's cases and combinations.
- The state is decided by the typed cause, never by phase or by the code `facade_certificate`.

| Typed cause | Indicator state |
|---|---|
| A selected case | its figure (§2.3) |
| Kernel terminal with evidence of conditioning or a mechanism: `UnresolvedReason`/`Refusal` `Ceiling`, `MechanismWitnessed`, `NegativeEnergy`, `ZeroDiagonal` | not confirmed (red) |
| A proof gate's predicate failure after a selected native run (`PreparedCandidateError::Proof` predicates, including REQ-006 (e)'s sharper allowance and REQ-006 (f)'s loaded maximum) | not established (grey) |
| Kernel or proof limits that are the method's own: `Budget`, `WorkAccounting`, `ExponentRange`, `LedgerUnavailable` (member loads) | not established (grey) |
| Internal faults: `Capture` faults, staging, serializer, precommit, combination custody, stack and notice reservation, unbound permit, receipt encoding | did not complete (grey, no retry offered) |
| Resource: worker cap, growth or footprint refusal, `LateGate`/`CompleteGate`, pool refusal | did not complete (grey, retry offered) |
| Cancelled | did not complete (grey, retry offered) |
| A DN §4.3-excluded case, which withdraws the whole successor | not available: "the ordinary solve failed for {case}" (deterministic, no retry) |
| Domain, route or no case to attempt | not available, with its reason |

Before implementing, the implementer enumerates every variant of `W1Fallback`, `UnresolvedReason`, `Refusal`, `PreparedCandidateError` and `CaptureError` into this table. Any variant whose evidence is ambiguous goes to "not established", never to red. A facade test pins each row, including:

- the existing sharper-allowance control (`o4_m3_judging_tests.rs` `o4_w1e_sharper_allowance_refusal_keeps_the_ordinary_result`), which must read "not established", not red;
- the loaded-maximum control (`w1_domain_stage1_tests.rs` `w1_loaded_maximum_too_wide_for_the_gate_keeps_the_ordinary_result`);
- a mixed invocation (one case red, one not established);
- a numerical proof failure after native selection.

**Coexistence** is split. A `pressure-source-1` publication keeps its source bounds (§2.3). The source republication path (`lib.rs:3298-3312`) is `pressure-2` with no receipt, so it reads "did not complete" for the cases the check attempted. A later decline never erases a bound that the current publication already carries. Source-recovery cases keep their figure. A case the source receipt does not cover reads "not available: only cases flagged by the ordinary checks are recovered on this route".

### 3.3 TBD-001 and TBD-002 (proposed, pending the owner; reviewed inputs, D3)

**TBD-001, proposed direction.** User-facing product text drops "certified", "certificate" and "certify". The formation definitions and the tables' `formation_warrant` keep "certificate" as the technical name of the kernel's error proof (REQ-008, AX-007). Identifiers keep their names.

| Text | Old | New |
|---|---|---|
| `retained_wire.rs` `SELECTED_MESSAGE` | "…binds their certified classes, the native attempts and work, and the ordinary-attempt evidence." | "…binds their verified classes, the native attempts and work, and the ordinary-attempt evidence." |
| `SELECTED_MEMBER_LOAD_MESSAGE` | "…relative to that value; the certificate certifies that declared-load problem." and "…binds their certified classes…" | "…relative to that value; the verified bounds apply to that declared-load problem." and "…binds their verified classes…" |
| `COMBINATION_SELECTED_MESSAGE` | "…from its own solve and certificate. …binds their certified classes…" | "…from its own solve and error proof. …binds their verified classes…" |
| `preview-physics-retained-1` table, member-load limitation | "Retained-precision certificates certify that declared-load problem." | "Retained-precision bounds apply to that declared-load problem." |
| `pressure-source-1` table, maxima limitation | "…and do not certify primitive geometry or material accuracy." | "…and do not bound primitive geometry or material accuracy." |

The three copies of the member-load text change together:

- the reader's G4 copy;
- `tests/verifiers/retained_precision.py`;
- `tools/retained_pricing/base/w1_member_load_model.py`.

Their writers regenerate the committed successors, the pressure-source reader corpus and the J7 census envelope and diagnostics digests, under AC-002's stated product-text clause. Table digests and the reviewed-input digests move, and the text lengths are priced, so D3 re-prices.

**TBD-002 repair.** In the `physics-retained-1` table's `accuracy_classification.scope`, "model 0.3.0 with the 2.0.0/exact_straight_pressure_v2 contract" becomes "model 0.6.0 with the 3.0.0/exact_pressure_v3 contract". This matches DEF-E and the code. It goes through the same D3.

### 3.4 Fresh identities

`physics-retained-1` joins `FRESH_IDENTITIES` (Rust) and `FRESH_SEMANTIC_CONTRACT_IDS` (TS), which stay equal. This matters only for an adopted successor (§2.4).

## 4. What the user sees

### 4.1 One method-agnostic view model

Per result set (load case or combination), the view model is `PrecisionCheck { state, figure N?, nearZeroRows, rowsWithoutBound, reason, method }`.

The adapters (§2.3, §3.2) fill it from:

- a retained successor;
- a source receipt;
- a typed outcome or decline;
- a non-live result.

The UI, exports and policy read only `PrecisionCheck`. A later method adds an adapter that supplies E per row and typed outcomes. It reuses every state and text. The method's name appears only in the details.

### 4.2 States and exact wording

The label is **"Precision check"**.

**Per result set:**

| State | Badge | Headline |
|---|---|---|
| running | neutral | "Precision check: running" |
| confirmed (N ≥ 6) | green | "Precision check: {N} significant figures confirmed" |
| low precision (N < 6) | red | "Precision check: only {N} significant figures confirmed" |
| not confirmed | red | "Precision check: not confirmed" |
| not established | grey | "Precision check: no bound established" |
| not available | grey | "Precision check: not available" |
| did not complete | grey | "Precision check: did not complete" |
| not current | grey | "Precision check: not current" |

**The result header** shows one summary:

- red, if any set is red;
- otherwise "running", if a check is running;
- otherwise, if every set is confirmed: "Precision check: at least {min N} significant figures confirmed";
- otherwise, if some sets are confirmed: "Precision check: {k} of {n} results confirmed";
- otherwise the first applicable of did not complete, no bound established, not available and not current.

The shell status chip carries the header's badge. Expanding shows each set's line.

**Detail texts.** Values in braces are filled in.

- **running:** "The results shown are from the ordinary solve. A second, extended-precision solve is checking them in the background."
- **confirmed:** "The values shown agree with the exact solution of the linear problem formed from this model's recorded inputs to at least {N} significant figures. Values near zero are instead within the small bound shown beside them. This checks the solver's arithmetic only. It says nothing about whether the model, its inputs or its design are right." It adds:
  - with member loads: "Uniform member loads enter this problem as declared equivalent nodal loads; that small difference from the authored loads is disclosed, not bounded."
  - with rows without a bound: "{n} values have no bound from this check and are marked "no bound"."
- **low precision:** "The values shown agree with the exact solution of the linear problem formed from this model's recorded inputs to only {N} significant figures. Do not rely on them beyond that precision." It adds:
  - when confirmed values exist: "The check's own solution is confirmed to 9 significant figures; choose Use the confirmed values to work with it."
  - otherwise: "This often points to the model: a mechanism or weakly restrained direction, a very short or very stiff element next to flexible ones, or stiffnesses that differ by many orders of magnitude."
- **not confirmed:** "The extended-precision solve found evidence that this problem is close to singular, and could not establish a bound. Do not rely on these values until the cause is understood. This often points to the model: a mechanism or weakly restrained direction, a very short or very stiff element next to flexible ones, or stiffnesses that differ by many orders of magnitude. Review the model and the solver diagnostics, then solve again."
- **not established:** "The check ran but could not state a bound for these values within its fixed limits. That alone does not mean the values are wrong, and it does not confirm them."
- **not available:** "{reason}" (the reason sentence starts the text):
  - `Size`: "This model has {observed} {fact}; the precision check supports models with at most {cap}."
  - `Feature`: "This model uses {feature}, which the precision check does not support."
  - `Build`: "The precision check is not set up for this build of the application."
  - `Budget`: "The per-solve memory budget is below the {required} the precision check needs."
  - `Other`: "This model is outside the range the precision check supports."
  - excluded case: "The ordinary solve failed for {case}; the precision check runs only when every load case solves."
  - combination not attempted: "This combination is outside the range the precision check supports ({reason})."
  - uncovered source case: "Only cases flagged by the ordinary checks are recovered on this route."
- **did not complete:** "The check stopped before reaching a conclusion ({cause}). The ordinary results are unchanged." The causes are "not enough free memory at the moment", "it was cancelled" and "an internal check failed". The first two add "Try again with Check precision."
- **not current:** "This result was not checked in this session, or the model has changed since. Solve again to check it."

**Other rules:**

- **Actions:** "Check precision" (retry, transient causes only), "Cancel check" and "Use the confirmed values" (§2.4).
- **Rows:**
  - a row without a bound is marked "no bound";
  - a near-zero row shows "within {E} {unit}";
  - a relative row shows nothing extra, so there are no ± ranges.
- **Existing labels.** The precision check is shown first. The existing standing lines keep their meaning and wording: "Numerical integrity: {status}" and the reader standing. Under a red or grey state the standing line is not repeated as a reassurance.
- **Vocabulary.** The indicator never uses "certified", "verified", "valid", "approved", "compliant" or "correct". The detail texts use "confirmed", "established" and "agree with" (the K-D5 R5 vocabulary).
- **One module.** Every literal is in `apps/desktop/src/features/results/precisionCheckText.ts`. GEN-13 scans it, and `tests/test_claims_language_surface.py` pins it.

### 4.3 Claims registry

The registry gains `NQ-PRECISION`. It registers:

- the indicator's display forms above, as its only forms;
- the boundary sentence "This checks the solver's arithmetic only. It says nothing about whether the model, its inputs or its design are right.", which appears with every confirmed or low-precision state.

§2.1 rule 3 ("No label exists outside the table") is amended to say that the precision-check forms are registered in `NQ-PRECISION`, and that they are neither analysis statuses nor evidence-status labels. This is owner text (§8).

## 5. Output surfaces

**While the ordinary result is current** (the normal case), every surface admits exactly what it admits today. Documents that carry result values gain a precision-check section (§5.1). These are handoff, local-fea, native-package, report, rendered-report and headless-runner, plus the result JSON and stress-neutral exports.

**After "Use the confirmed values".** The adopted successor is admitted on exactly the surfaces that admit its base identity, at the reader's numerically eligible standing with the live capture:

| Retained identity | Base identity |
|---|---|
| `preview-physics-retained-1` | `preview-physics-2` |
| `physics-retained-1` | `pressure-2` |

| Surface | `preview-physics-retained-1` | `physics-retained-1` |
|---|---|---|
| result-export, stress-neutral | admitted when eligible (unchanged) | admitted when eligible (unchanged; reachable once fresh) |
| rule-check | admitted when eligible; rows without a verified class do not bind (existing rule) | the same; Tauri `is_fresh_identity` admits it |
| the other 11 non-value surfaces | admitted when eligible | refused, as `pressure-2` |
| the six value surfaces | admitted when eligible, with §5.1's section | refused, as `pressure-2` |
| report-package | refused (`REPORT-PACKAGE-FRESH-RESULT-UNAVAILABLE`) | refused |

What the implementation changes:

- `LoadReferenceOutputGate` reads standing; today it treats `admitted_when_eligible` as refused.
- `RenderedReportPanel.tsx:70` uses `surfaceOutputRefusal`.
- A refused cell uses its base identity's refusal text, which retires `RETAINED-PRECISION-OUTPUT-NOT-YET-AVAILABLE`.
- Each of the 17 policy cells has its own output-policy test, so "no result values" is tested, not assumed.

### 5.1 The precision-check section in exported documents

Each value-carrying document gains one section, formed from `PrecisionCheck`. It contains:

- the per-case headline;
- the boundary sentence, for confirmed and low-precision states;
- the reliance warning, for red states;
- the declared-load disclosure, where it applies;
- a keyed list of the emitted rows that have no bound (row id → "no bound");
- a keyed list of the emitted near-zero rows with their bound E.

Rows that are not emitted are not listed. Where the document already has per-row structure (report and rendered-report rows), the mark is placed on the row itself.

The result JSON export of the ordinary result is unchanged, and the section goes with its notices. The checked publication (the successor or ordinary-plus-notice bytes) can be exported separately as "Precision check evidence (JSON)", with the receipt. Exports of a result that is not confirmed are not blocked: the section states it.

Tests check the emitted artifacts, not only the policy cells.

## 6. Qualifying the product build (D3 on the product build)

### 6.1 Release safeguards

The desktop's `Cargo.toml` gains per-package release overrides. `core/product_physics/Cargo.toml` gains the same overrides, for its own test builds. The overrides set:

```toml
[profile.release.package.open_pipe_stress_product_physics]
debug-assertions = true
overflow-checks = true
# the same for every crate in the retained proof and reader closure:
# frame_kernel (structural/retained/**), correct_norm, result_export, units, canonical_json
```

The safeguards are enforced as follows:

- **Overflow checks.** `build_status` gains a fail-closed probe: an overflowing checked addition through `black_box` must panic under `catch_unwind`. If it does not, the build is `Stale`.
- **Debug assertions.** The registered release identity records `debug_assertions=true` through the existing key.
- **Settings the identity does not record.** A build-script check rejects any LTO or codegen-units value other than the registered release settings, so a differently configured build fails closed rather than silently matching.

### 6.2 The registration

The registration is a second `REGISTERED_PROFILES` entry. It keeps the same M (12 GiB), 0.9 M margin, 5 % TEXT reserve and R (64 MiB).

**Identity.** The value is read from the actual Tauri release build, never derived from Cargo defaults. A test-only Tauri command under the existing `native-smoke` feature returns the compiled identity and `retained_profile()`.

**Steps, on the final priced source:**

1. **Lock alignment first.** The resolved closure of `core/product_physics/Cargo.lock` must equal `apps/desktop/src-tauri/Cargo.lock`'s versions and checksums. Today `cfg-if`, `libc` and `typenum` differ. The desktop lock is updated to the PP lock's versions, and a maintained test keeps them equal.
2. **Build.** Build the desktop release binary with `tauri build`, through the wrapper, and read the identity.
3. **Atoms.** In that identity (`cargo test --release --lib profile_` in `core/product_physics`, with the run's identity checked), collect the in-build atoms and reader layouts. Compare them with the debug atoms; a differing atom is evaluated, not assumed.
4. **D3.** `tools/retained_pricing run` and `generate --check`, with all 80 budget evaluations at M = 12 GiB.
5. **In-build checks in the release identity:**
   - the phase record and law tests;
   - the allocation-count challenge, `retained_direct_peak_is_within_the_profiles_bounds`;
   - the six stack witnesses at R/64;
   - the full `frame_kernel` retained suites (`retained_k4`, `retained_wide*`);
   - a byte reproduction of the committed successors.

   The release record is pinned beside the debug record (`PINNED_RECORD` per identity), and the phase and challenge checks assert that they ran, not skipped.

**Tests to generalize.** Tests that assume one profile are generalized:

- `retained_memory_law_tests.rs:836-841`;
- the forged index `Some(1)`, which becomes `REGISTERED_PROFILES.len()`;
- the hard-coded index-0 assertions;
- `registered()`'s literal identity in `retained_facade_tests.rs` and `tests/retained_precision_admission.rs`.

Each registration-dependent test asserts the registered branch in either registered identity, and `Stale` elsewhere.

**Residual.** `cargo test --release` compiles test-only code (`carry_test_hooks`), so the allocation and stack evidence comes from the test artifact. The shipped binary is exercised by the native smoke, which runs a real check in it (§7).

**Stop condition.** If any phase in either mode exceeds ⌊0.9 M⌋ with R and the TEXT reserve, or any stack witness fails at R/64, registration stops. The same applies if the release safeguards cannot be kept. In each case the result goes to the owner as Decision 22. No margin, R, M, cap or safeguard is changed to fit.

**Upkeep.** Any of the following makes the product build `Stale` until it is re-registered:

- a rustc upgrade;
- a RUSTFLAGS or profile change;
- a lock change in the closure;
- a priced-source change.

The indicator then reads "not available: the precision check is not set up for this build", and the ordinary route stands.

## 7. Verification

- **Core.** The affected crates' full suites (`product_physics`, `frame_kernel`, `result_export`, `operation_applier`, the headless runner) run through the wrappers in three builds, with main comparisons:
  - the debug registered build;
  - the release registered build;
  - an emulated unregistered build.

  Also on the final head:
  - registered `piping-pytest` in full, with the five binary presets rebuilt for the head;
  - VER-001 to VER-006;
  - D3.
- **Facade tests:**
  - `EveryCase` on routes L and E;
  - the existing trigger's bytes unchanged, apart from §3.1's field and §3.3's texts;
  - every row of the cause→state table;
  - the decline mapping's totality;
  - `retained_direct_admission` agreeing with the Direct entry's admission on the corpus inputs.
- **Desktop.** Full `desktop-test` with the wasm engine rebuilt, plus:
  - the `PrecisionCheck` adapters over both corpora;
  - the E and N arithmetic, against an independent exact-rational oracle in the test;
  - every state, text and header aggregation;
  - job lifecycle: generation guard, cancellation, identical-request/different-mode, stale completion;
  - adoption;
  - all 17 policy cells;
  - the emitted documents' sections;
  - GEN-13.
- **Native.** The open path gains a job kind, so the native smoke and `npm run test:e2e:dist` run on the final head:
  - the native smoke observes a confirmed check on a small invented model, and "not available" on the larger milestone model;
  - the dist e2e observes the hosted build's "not set up for this build".
- **Review.** Dual review of this design. On the final head, dual review with independent numerical scrutiny: an independent Claude reviewer and Codex. The scrutiny covers §2.3's arithmetic, §3's producer and reader changes, and the registration and D3 evidence.

## 8. Commitment text for the owner

One bundle of exact old→new text, prepared as edits on this branch. It contains:

**DEL-04-08 ScopeOfWork:**

- CLM-001 "Trigger" and "Entry";
- CLM-003's successor reach and registered builds;
- REQ-003 (a);
- REQ-004 and AC-004 ("each registered build identity", with the release safeguards);
- REQ-006 (c)'s successor bullet (§5);
- AC-006, retiring the "refuses each named identity" clause and `retainedPrecisionOutputRefusal.test.tsx` from VER-006;
- a new REQ-009, AC-009 and VER-009 for the desktop precision check: §2's flow, §2.3's figure and its threshold of 6, the cause→state table and every indicator text;
- REQ-008, AC-008 and VER-008, covering `precisionCheckText.ts`;
- TBD-001 resolved as §3.3 proposes, and TBD-002;
- AX-004, without B8 and without "This deliverable activates nothing".

**PRD §12.8**'s last bullet, desktop-specific: other product surfaces stay held.

**Claims registry:** `NQ-PRECISION`, and the §2.1 rule 3 amendment.

**Reconciled in the implementation PR as design consequences:**

- DEL-04-07's `deliverable.yaml` condition;
- DEL-04-07 `memory-budget-decision.md`, the sentences saying the desktop does not use the retained route;
- DEL-04-07 `desktop-exact-outputs.md` §9;
- DEL-07-11's scope-fence note;
- `outputPolicy.ts`'s B8 comment.

## 9. What stays unchanged

- Every numerical gate, tolerance, precision ladder and limit; the D1 domain and caps; M, the 0.9 M margin, the TEXT reserve and R.
- The ordinary route's bytes and timing.
- The current result and its outputs, unless the user adopts confirmed values.
- Rule binding of successor rows (D2 4.9.9).
- Retained-source recovery's method and limits.
- Frozen references under `validation/references/**`.
- The report package's refusal.
- G10 and the frozen UI benchmark.
