# T3 D2 — result standing, source envelopes and transport

Design TASK D2 (Type 2, HELPS_HUMANS-style design posture) for the T3 WORKING_ITEMS manager, 2026-09-26. **This is a proposal.** It becomes a basis only if ROOT selects it after the independent design review V1.

- **Brief:** `T3/TASK_BRIEFS/D2_STANDING_DESIGN.md` with `_COMMON.md`. `P/` is `projects/chirality-piping/`, `T3/` is this tranche's records folder, `PP` is `P/core/product_physics/src/lib.rs`, `LSI/` is T1's `LOAD_STATE_IMPLEMENTATION/` records folder.
- **Bases.** Main `c61a540ea` (the T3 branch head `e14f7fd13` has the same product source). T1 candidate `f3270ea79`, read only through `git show` and `git diff c61a540ea...f3270ea79`. Line numbers are at those commits and will drift.
- **Design against T1 as merged.** Every place where T1's final merge could change this design is marked **[T1-n]**; the list is in §10.
- **Nothing was built or run against product code** (host hold). One read-only, standard-library corpus scan was run (§12). The probe sources for the release are in `_run_records/` with their predicted outcomes.

## 1. Recommendation in brief

| # | Item | Recommendation |
|---|---|---|
| 1 | Composite finalization | **Port T1's SF-1 republication to both pre-0.4 source identities (option F1).** When a selected join cannot finalize, the invocation is republished on its ordinary route in the same ledger, every successful attempt is declined, and every case publishes its ordinary response with its ordinary standing. One rule then governs all three source identities. The `Err` becomes an unreachable invariant. Two named residuals remain (§4.1.5); they close with D1's general method, or with option F2 if ROOT wants them closed sooner. |
| 2 | Joined eligibility | **A reader-side re-derivation of the resolved 0.4.0 case** in Rust, Python and TypeScript, written from T1's wire records, bound to the actual invocation. It replaces `physics_source::actual_materials` for 0.4.0. Joined results become `numerically_eligible` only with an invocation whose re-derivation matches every published operand. |
| 3 | Binding route | **No new route.** `result_envelope_binding` already mints `QualifiedPreviewEvidence` and a canonical document for any eligible result. Eligibility makes joined results flow through it. T3 replaces T1's "no document" characterization with a positive route test and adds route-generated joined carriers. Desktop export stays T6. |
| 4 | Selected-UNAVAILABLE | **Tighten physics-source-1 and source-blocks-1 to the joined S13 rule.** No producer can emit the diagnostic on a selected case, and no committed envelope carries it. Tightening refuses no qualified byte; relaxing would weaken a qualified reader. |
| 5 | source-blocks-1 re-homing | **Retire source-blocks-1 for fresh solves once D1's general method covers its domain on the preview route (option R-B).** Historical source-blocks-1 stays readable and byte-unchanged, becomes historical-only like precision-1, and the `N_SB` / `N_SB_MIXED` texts stop promising T3. If D1 does not cover that domain within T3, ROOT chooses between a successor identity (R-A) and extending T0R's containment. |
| 6 | Transport and display | **Display:** refuse a display conversion whose nonzero value would appear as zero or as a subnormal in the target unit; show the stored unit instead. **Transport:** the 2^53 − 1 limit is wider than T1 noted: every finite binary64 above 2^53 − 1 in magnitude is refused by the checked canonical carriers. T3 owns the requirement, the reference cases and the fallback when a receipt cannot be hashed; **T6 adopts `openpipestress_jcs_binary64_v1`** for canonical result-value positions. **Comparison policy:** no protected predicate changes. |
| 7 | Standing across languages | Four standing changes, each identical in Rust, Python and TypeScript and proved by one shared case file per slice (§4.7). |

**No owner decision is needed.** Nine technical decisions go to ROOT (§9).

## 2. Distinctions this design depends on

- **Publication versus standing.** A result can be published (readable, verifiable) without being Current. Standing is whole-envelope on every route today: `numerical_use_standing*` needs every requested case to qualify (`semantic_contract.rs:367-453`; `compatibility.py:284-335`; `numericalResultQuality.ts:68-112`). An envelope that contains a sensitive or failed case is never Current, on any route.
- **The finding standard, read against that rule.** The brief's standard is "no invocation loses a correct result because another case failed, and no failed or Sensitive case gains standing". Under whole-envelope standing, the first half can mean "correct values stay published". It can mean "a correct case stays usable" only with case-scoped standing, which no accepted instrument defines. This design meets the standard in the first sense and names what the second sense would need (option F3, §4.1).
- **Unreachable versus refused.** A reader rule that refuses a fact no producer can emit changes no honest output. A producer path that becomes unreachable is kept as an invariant, not deleted.
- **Re-derivation versus replay.** A reader re-derives the operands that enter the solve (materials, eigenstrain, motions, factors) from the actual invocation. It does not assemble, solve or replay the retained source. This is the existing physics-source-1 trust basis (`physics_source.rs:896-974`).
- **Correctly rounded versus host-rounded arithmetic.** `+ − × ÷ √` and fused multiply-add are reproducible bit for bit in every language. Host `exp` and `exp_m1`, which T1's logarithmic law uses (`case_state/thermal.rs:9-11, 484, 666-674` at T1), are only faithful (within one unit in the last place).

## 3. Current state, with citations

### 3.1 Composite finalization

**Main.**
- The value route returns `Err("SOURCE_BLOCKS_FINALIZATION_FAILED")` when the producer claims `source-blocks-1` or `physics-source-1` but has no receipt (`PP:1249-1253`). That happens when the invocation receipt fails: `PP:1668-1680` pushes the blocking `SOURCE_BLOCK_RECOVERY_FINALIZATION_FAILED` at `diagnostic:source-recovery:publication` and leaves `source_block_recovery` empty.
- A selected case whose *own* finalization fails pushes a blocking diagnostic at `diagnostic:source-recovery:<case>:finalization` (`PP:2914-2926`). The case loop then returns a blocked `MODEL_INCOMPLETE` envelope with no results (`PP:1430-1432`, `:1497-1499`, `blocked_envelope` at `PP:10718`). That also loses every case's results; it is not an `Err`, but it is the same finding.
- The composite receipt requires every case to qualify: `composite::validate_publication` refuses unless some case is exact and all cases are qualified (`source_receipt/composite.rs:938-947`). The physics-source-1 receipt schema is closed to that shape: `status` is `const "qualified"`, and every case has `outcome: const "qualified"` and `failure: null` (`P/schemas/physics_source_recovery.schema.json`).
- The non-composite source-blocks-1 receipt admits failed cases (`partial`, `source_receipt.rs:859, 904`). Its remaining invocation-level triggers are resource and internal failures: the publication reservations (`source_receipt.rs:677-684, 715-722`), the case-budget policy check, and the numerical-summary check. The invocation ledger is 64M, and a committed N05 case charges 3.2M (`fixtures/product_preview/source_blocks/n05-dense_scrutiny.raw.json`), so about twenty such cases exhaust it.
- The receipt hashes with the checked canonical profile (`source_receipt.rs:5, 30-32`), which refuses any value above 2^53 − 1 in magnitude (§3.6). A large but finite published action therefore also fails finalization.

**Trigger reproduced by T1's review.** P12: the physics-source-1 `mixed` witness with its second case's modulus basis removed, which makes that case sensitive while its pressure region keeps retained source unavailable, gives `P12 Err SOURCE_BLOCKS_FINALIZATION_FAILED`. P3, the 0.4.0 analogue, gave the same at checkpoint 3 (`LSI/REVIEW_CHECKPOINT_3/_run_records/review3_probes.rs.txt:124-134, 533-543`; `probes_final.log:32, 41`).

**T1 at `f3270ea79`.**
- SF-1 (`LSI/CP4_WIRE_ADDENDUM.md` §1.2). The wrapper `run_linear_static_preview_captured` (`lib.rs:1433-1466`) clones the request when `case_state::is_load_state` holds. It reruns `run_linear_static_preview_captured_once` with `withholding_load_state_join(cause)` when `load_state_join_failure` was recorded.
  - Failures are recorded at the per-case arm (`lib.rs:3494-3503`) and the receipt arm (`lib.rs:1988-2000`), both gated to 0.4.0.
  - In the rerun, successful attempts are declined by `decline_withheld` (`source_recovery.rs:217-229` at T1) with an `UNAVAILABLE` suffix naming the cause (`lib.rs:2403-2440`).
- ROOT's CP4 N-2 ruling: the republication **continues the same ledger** (`lib.rs:832-845`; `LSI/REVIEW_CHECKPOINT_4/BACKCHECK.md` N-2). Note that `CP4_WIRE_ADDENDUM.md` §1.2 still says "a fresh ledger"; the backcheck's B-2 records that this statement was superseded.
- The 0.4.0 replay reservation `c ≤ L − c` is a screen only (CP4 review SF-1R).
- The pre-0.4 `Err` is unchanged. It now also names `LOAD_REFERENCE_SOURCE_SEMANTIC_CONTRACT_ID` (`lib.rs:1424-1428`), and is pinned by `the_pre04_composite_finalization_failure_is_unchanged_by_this_fallback` (`source_receipt/load_state_fallback_tests.rs:293-312`).
- The CP4 review's differential shows pre-0.4 output byte-identical on 106 runs (`LSI/REVIEW_CHECKPOINT_4/RETURN.md:63`).

### 3.2 Joined eligibility

- The T1 joined reader (`result_export/src/load_reference_source.rs`, 218 lines) has three steps: J0 identity, J1 the joined pre-pass S1–S13, J2 the receipt, and J3 `physics_source::validate(&projected, None)`. It never passes an invocation, and it returns `Ok(false)` for every admitted envelope (`load_reference_source.rs:29-31, 76-78`). Python mirrors it (`analysis_runs/load_reference_source.py`), and TS has an invocation-free port (`loadReferenceSourceEvidence.ts`, T1 ruling §12).
- Standing:
  - Rust has T1's declared early return, `needs_recompute` for `LOAD_REFERENCE_SOURCE_ID` after validation and `standing_reason` (`semantic_contract.rs:470-476` at T1).
  - Python has the same (`compatibility.py:335-339` at T1).
  - ROOT decision §7 (`LSI/T1_WAVE1_RULINGS.md`) routes invocation-based eligibility to T3.
- The physics-source-1 analogue:
  - `numerical_use_standing_with_context` passes the actual invocation to `physics_source::validate` (`semantic_contract.rs:385-405`).
  - That validator binds the invocation hash, model identity and case coverage (`source_blocks.rs:743-903`).
  - It then re-derives each member's E, ν and α from the authored material and the case's modulus basis, comparing by binary64 equality (`physics_source.rs:809-974, 1128-1141`; Python `physics_source.py:371-425`).
  - This selection logic is pre-0.4: it reads the case-level `modulus_basis_ref`. A 0.4.0 case selects per member in `analysis_state.element_states[].material_selection`, and uses fused `mul_add` interpolation (`case_state/material.rs:320` at T1). So `actual_materials` cannot be reused for 0.4.0.
- The joined record already carries the resolved intermediates (`load_reference.rs:36-150` at T1; `LSI/CP2_WIRE_ADDENDUM_1.md` §3): selected pair, consumed points, interpolation fraction, law segments, datum stretches, thermal and fit strains, total eigenstrain, support motions and the contribution ledger. The receipt binds each record into `physical_evidence_sha256` (`load_reference_source.rs:166-180`). A selected case's source payload also commits `resolved_evidence_sha256`, member bits, eigen loads and prescribed motions (`CP2_WIRE_ADDENDUM_2.md` §5.4).
- The readers do not check that law segments lie inside `[T_lower, T_upper]` (`CP4_WIRE_ADDENDUM.md` §2, last line).

### 3.3 Binding route

- `build_result_export_document_with_evidence` (`P/core/runner/headless/src/result_envelope_binding.rs:236-270`) builds the canonical document through `derive_document` only when `numerical_use_standing_with_context(..., Some(&evidence.actual_invocation))` is `numerically_eligible` (`:258-260`).
- The actual-solve route (`headless/src/lib.rs:731-756`) mints `QualifiedPreviewEvidence` and calls that function for every solved Value invocation.
- `derive_document` and `validate_document` already accept the joined identity and copy its receipt (T1 diff in `derivative.rs:97-116, 326-334`).
- So the only thing that blocks a joined document is standing. T1 characterizes the absence in `load_reference_route_tests.rs:473-598` (`joined_actual_solve_retains_invocation_bound_receipt_without_canonical_export_both_modes`, with `CURRENT_NUMERICAL_INTEGRITY_NEEDS_RECOMPUTE`). The committed `fixtures/results/load_reference_source_*.document.json` carriers come from the test-side `derive_document` with a test origin (`LSI/T1_WP4_HEADLESS/RETURN.md:11-17`; T1 ruling §11).

### 3.4 Selected-UNAVAILABLE

- **Joined reader: refuses.** It has S13 `JOIN_SELECTED_UNAVAILABLE_DIAGNOSTIC`: a `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` whose `affected_refs` names a selected case is refused. The check is in `load_reference.rs:596-618` (inside `if joined`), `load_reference_evidence.py:416` and `loadReferenceEvidence.ts:397`, with four shared cases and killed mutants (`LSI/T1_WP1_JOINED_READERS/RETURN.md:292-357`).
- **physics-source-1 and source-blocks-1: accept.** Neither reader checks that diagnostic code in any language:
  - Rust `source_blocks.rs`, `physics_source.rs`;
  - Python `source_blocks.py`, `physics_source.py`;
  - TS `sourceBlockRecovery.ts`, `physicsSourceRecovery.ts`.

  (T1 `git grep`: the code occurs only in the load-reference readers and at the emitter.) A resealed envelope carrying the contradiction is therefore accepted. With an actual invocation, by reading, it keeps its eligibility.
- **Emitters.** There is one, on both branches: the `Err` arm of the attempt (`PP:1888-1897`; T1 `lib.rs:2425-2438`). In that arm `selected_source` stays `None`, so the case is never selected. T1's SF-1 declines emit the diagnostic only for cases that are then published ordinarily. No producer path emits it on a selected case.
- **Corpora.** The scan (§12) found that no committed envelope-shaped JSON carries the diagnostic at all:
  - main: 0 of 43 receipt-bearing envelopes;
  - T1: 0 of 83.

  The joined mutation cases are stored as edits, not as envelopes.

### 3.5 source-blocks-1 carries (T0R R-1, R-2; confirmed T3 scope by ROOT)

- **Selected plus ordinary.** The envelope reads `needs_recompute` with `SOURCE_BLOCKS_ORDINARY_CASE_LEGACY_SEMANTICS`, derived from the receipt, in all three languages:
  - Rust: `semantic_contract.rs:329-338`, `source_blocks.rs:657-667`;
  - Python: `compatibility.py:338-343`, `source_blocks.py:450`;
  - TS: `knownSemanticLimitations.ts:53-61`, `sourceBlockRecovery.ts:285-300`.
- **All-selected.** The envelope stays Current, but its abs-sum summary and the headline's row are refused for rule binding with `RULE_SOURCE_BLOCKS_SUMMARY_NOT_RELIABLE`:
  - Rust: `semantic_contract.rs:340-352`;
  - Python: `compatibility.py:364-380`;
  - TS: `knownSemanticLimitations.ts:63-68`, used by `ruleCheckService.ts:327`;
  - native: `src-tauri/src/lib.rs:3206-3216`;
  - `rule_check_runner`: `lib.rs:93`.
- **What the selected rows publish.** Selected source-blocks-1 cases already publish six signed `support_reaction_component_v2` rows (checked in `sourceBlockRecovery.ts` `SUPPORT_COMPONENT`). Their stress summary is still the abs-sum `open_formula_stress_summary`. Ordinary cases keep norm-only reactions and the abs-sum summary.
- **User texts.** `N_SB` and `N_SB_MIXED` (`knownSemanticLimitations.ts:20-21`) promise T3.
- **Correction to `STAGE0_MAP.md` §3.** T1 edits `knownSemanticLimitations.ts` (the fresh set, six lines), so the texts are a T1-overlap file.

### 3.6 Transport and display

- **Display chain.** `QuantityReadout` (`features/display-units/index.tsx:95`) → `convertDisplayQuantities` (`services/displayQuantityService.ts`) → Tauri or WASM `convert_display_quantities` (`operation_applier/src/display_units.rs:7-33`) → `units::convert_for_dimension` (`units/src/lib.rs:917-950`).
  - The conversion is `(v·f_from + off_from − off_to)/f_to` (`lib.rs:714-720`). Identity returns the input bits.
  - Only a non-finite result is refused (`display_units.rs:27-28`). A nonzero value that becomes zero or subnormal is reported as `converted`.
  - The companion-unit path `convertForDisplay` / `formatConverted` (`services/unitConversion.ts:68-85`) has the same gap, and prints "0" for an underflowed value.
  - None of these files is changed by T1.
- **The canonical number limit.** It is not specific to integer literals.
  - `derivative::guard_json` refuses any finite value with `fract() == 0 && |v| > 9 007 199 254 740 991` (`result_export/src/derivative.rs:13-19`). The checked canonical profile does the same (`canonical_json/src/lib.rs:95-108`, `CHECKED-JSON-UNSAFE-INTEGER` / `CHECKED-JSON-UNSAFE-INTEGRAL-FLOAT`), and so do Python (`canonical_json/adapter.py:42`) and TS (`resultExportAdapter.ts:25`).
  - Every binary64 above 2^53 in magnitude is integral, so **every** finite value above 2^53 − 1 in magnitude (1e16, 1e21, 1e300) is refused when a canonical document, an AnalysisRun or a source receipt is built.
  - Raw envelopes (serde with `float_roundtrip`) carry such values unchanged.
  - T1's CP4 note records the same limit for readers (`CHECKPOINT_4.md:70-74`).
- **The scientific profile.** `openpipestress_jcs_binary64_v1` (`canonical_json/src/binary64.rs`, PR901) is standalone. No product, runner or desktop path uses it (`SCIENTIFIC_TRANSPORT_FOUNDATION/RETURN.md`).
- **Protected comparison.** The mechanics benchmark predicate is absolute 1e-9 (`P/validation/benchmarks/mechanics/src/lib.rs:1437-1441`). ROOT confirmed that protected predicates stay unchanged (`STAGE1_PLAN.md` §8 item 4).

## 4. Options and the recommended design, per item

### 4.1 Composite finalization

#### 4.1.1 Options

| | Option | What the failing invocation publishes | Contract change | Meets the standard? |
|---|---|---|---|---|
| **F1** | **Port SF-1:** republish every case on its ordinary route, same ledger | physics-1 (exact models) or preview-physics-1 (other pre-0.4 models). Every case has its ordinary rows, `UNAVAILABLE` naming the cause, and ordinary standing | None. Existing identities, tables, schemas and readers | Standing: yes. Publication: yes, except residuals R-1a and R-1b (§4.1.5) |
| F2 | Mixed envelope: a `partial` receipt under the source identity | Selected cases keep their recovered rows. Failed cases publish ordinary rows under a failed receipt entry. The envelope is `needs_recompute` | physics-source-1 needs a new receipt policy, because its schema is closed to `qualified`: a new schema file, producer `failed_physics` cases, readers in 3 languages, AnalysisRun and results schema branches. source-blocks-1 already has `partial` | Standing: yes. Publication: yes, fully |
| F3 | F2 plus case-scoped standing | As F2. A requested subset of qualified cases can be eligible | F2 plus subset standing in every standing function, a non-bindable headline in partial envelopes, and consumers that request subsets | Both senses. Needs a product-semantics choice ("Current for a case") that no instrument covers |

#### 4.1.2 Why F1

- **Standing is identical under F1 and F2.** In both, an envelope containing a failed or sensitive case is `needs_recompute`, exactly as on every other route. F2's only gain is that the recovered values of the selected case remain visible for inspection, and that the N06 sub-case (R-1b) publishes instead of blocking.
- **F2's price.** It needs a second receipt contract for a PR905-qualified family, and a partial-composite reader branch in three languages, which V-level review then has to qualify.
- **D1 makes F2's gain short-lived.** D1's general method (interface I-1, I-3) makes the recovered values the ordinary values, and makes N06-class ordinary attempts pass. F2 would be built to be retired.
- **F1 is never worse than today.** On every input where today's route returns `Err` or a blocked envelope, F1 returns either published ordinary results or a blocked envelope that names the cause. Every other input is byte-identical (§6.3).
- **F1 keeps one rule for all three identities.** "A selected join publishes only when its receipt finalizes; otherwise the invocation publishes ordinarily." T1's 0.4.0 path already follows it.

#### 4.1.3 Recommended mechanism (slice S-D; waits for T1's merge)

All changes are in `PP` and generalize T1's code. They add nothing new to `source_recovery.rs`.

1. **Wrapper.** In `run_linear_static_preview_captured`, the clone condition becomes: `capture.is_some() && load_state_join_withheld.is_none() && model.combinations.is_empty() && no support has a nonlinear record`. That is, any captured invocation in which a retained-source attempt is possible (`PP:1885`). The rest of the wrapper is unchanged: at most two runs, and the rerun continues the same ledger (ROOT N-2). **[T1-1]**
2. **Decline.** In the attempt, the pre-0.4 arm currently returns `Ok(recovery)` (T1 `lib.rs:2404`). It becomes: if `load_state_join_withheld` is set, `Err(recovery.decline_withheld())`. The replay-reservation screen stays 0.4.0-only. Porting it would decline every committed pre-0.4 witness, which charge about 3.2M of their 4M case limit, and so change pre-0.4 selection.
3. **Failure recording.** Drop the 0.4.0 gates on `record_load_state_join_failure` in the per-case arm (T1 `lib.rs:3494-3500`) and the receipt arm (T1 `lib.rs:1988-1994`). The first run keeps its blocking diagnostics; they are never published, because the rerun replaces the envelope.
4. **Diagnostic text.** The `UNAVAILABLE` suffix naming the withheld join (T1 `lib.rs:2428-2431`) applies to pre-0.4 as well. Readers never validate message text.
5. **The `Err` line** (`PP:1249-1253`; T1 `lib.rs:1424-1428`) stays as an invariant guard with its text unchanged. After S-D it is unreachable: a rerun selects nothing, so its identity is ordinary. §6.1 adds a test that no fallback input reaches it. No other consumer matches the string (T1 `git grep`: only the guard and the characterization test).
6. **What the republication publishes.**
   - A 0.3.0 exact model publishes `physics-1`, with `contract_evidence.exact_cases`.
   - A 0.1.0, 0.2.0 or non-exact 0.3.0 model publishes `preview-physics-1`, rendered, because `source_selected` is false (`PP:1504`).
   - Each case carries its ordinary quality. A case whose attempt was declined or failed carries one `SOURCE_BLOCK_RECOVERY_UNAVAILABLE`.
   - Rows are bit-identical to the uncaptured typed route. Diagnostics differ only by those `UNAVAILABLE` entries.
   - Both identities' readers already admit an `UNAVAILABLE` info diagnostic whose `affected_refs` is a load case id (ROOT_SELECTION F-1 admits model-entity refs; main already emits it when a lone sensitive case's recovery fails). §6.2 still pins this with positive controls in three languages.

#### 4.1.4 The T1 characterization test

`source_receipt/load_state_fallback_tests.rs::the_pre04_composite_finalization_failure_is_unchanged_by_this_fallback` is **replaced in place, not deleted**. **[T1-8]**

- It is renamed `the_pre04_composite_finalization_failure_republishes_every_case_ordinarily`.
- It keeps its P12 input and asserts, in both modes:
  - `Ok`;
  - identity `physics-1`;
  - no receipt;
  - `MECHANICS_SOLVED`;
  - no blocking diagnostic;
  - rows bit-identical to the typed route;
  - `case` carries `UNAVAILABLE` with "invocation join withheld", and `case:ordinary-pressure` carries its own `UNAVAILABLE`;
  - quality `sensitive`;
  - standing `needs_recompute` in Rust and Python, and TS `numericalResultStanding` not eligible.
- The replacement is recorded in the S-D implementation record:
  - the old test name and assertion, and the file sha256 before and after;
  - the reason (ROOT's selection of this design);
  - the mutant that restores the pre-0.4 gate, which must fail the new test.
- Companion tests for source-blocks-1:
  - the invocation-limit trigger, 21 N05 cases (probe PR-3);
  - a per-case finalization failure under a private case limit (T1's pattern);
  - the large-action trigger (probe PR-5a).

#### 4.1.5 Residuals of F1 (named, not hidden)

- **R-1a.** A selected case's recovered values are not published in the fallback. Its ordinary sensitive values are published instead, not Current. This is the price of option F1. It closes when D1's general method gives the ordinary route those values, or with F2.
- **R-1b.** A selected case whose own ordinary attempt was *rejected* has no ordinary response. N06 is the example (`physics_source/n06-*.raw.json`: `ordinary_attempt = rejected`, `NUMERICAL_INTEGRITY_ASSEMBLY_UNRESOLVED`). The republication therefore publishes a blocked envelope, with the integrity diagnostic and the named cause, as T1 does for 0.4.0 (`CP4_WIRE_ADDENDUM.md` §1.2, last paragraph).
  - Today the same input returns `Err` (probe PR-2), so F1 is not worse.
  - The user can solve that case alone and get its selected, eligible result.
  - It closes with D1 or F2.

### 4.2 Joined eligibility (slice S-E; waits for T1's merge, and see I-4)

#### 4.2.1 Design

A new re-derivation, `RESOLVED-CASE-REDERIVATION-v1`, lives in new files:
- `P/core/reporting/result_export/src/load_reference_rederive.rs`;
- `P/core/analysis_runs/load_reference_rederive.py`;
- `apps/desktop/src/features/results/loadReferenceRederive.ts`.

Its source discipline, rounding rules, checks and standing are as follows.

**Source discipline.**
- It is written from T1's wire records (`CP2_WIRE.md`, `CP2_WIRE_ADDENDUM_1.md` §3–5, `CP3_WIRE_ADDENDUM.md` §1–2, `CP2_WIRE_ADDENDUM_2.md` §5, and any later accepted addendum **[T1-3]**), not from `case_state/*`.
- Step RD-0 comes before coding. The implementer lists every re-derived field and the arithmetic the records specify for it. Where the records are silent (for example the operation order of `derived_G_pa`), a records addendum is requested through the manager. Nothing is taken from product code.
- The independent reviewer checks this discipline.

**Arithmetic and rounding.**
- Correctly rounded operations and fused multiply-add are reproduced bit for bit:
  - Rust uses `f64::mul_add`;
  - Python 3.11 has no `math.fma`, so it computes the exact rational with `fractions.Fraction` and rounds with `float()`, which is correctly rounded in CPython;
  - TS computes an exact BigInt rational with round-half-even.
- Temperature identity uses exact rationals built from the shortest round-trip decimal and the unit's affine definition (`CP2_WIRE_ADDENDUM_1.md` §5), with checked i128 arithmetic as the producer states (`case_state/temperature.rs:1-11` at T1). An overflow is a scope refusal (below), never a guess.
- **Host-rounded fields.** These are the logarithmic law's stretches and strain (`exp`, `exp_m1`). A recorded value is accepted when it is within one unit in the last place of the reader's own evaluation of a bit-identical argument (the faithful-rounding bound). The argument itself must match exactly. Every downstream value is re-derived from the *recorded* value. Rust, Python (C libm) and TS (V8's own `exp`) therefore reach the same outcome on every host (decision DD-4).

**Checks.** They form step J4, which runs only when an actual invocation is supplied, after J0–J3.

| Check | What it compares |
|---|---|
| J4.1 invocation | Uses the projected physics-source-1 validation with the invocation: invocation hash (`source_blocks_invocation_v1`), invocation shape, model project id, case coverage and order, requested mode, source coverage, combinations. It skips `actual_materials` through a new `MaterialCheck::External` parameter (slice S-C) |
| J4.2 document | 0.4.0 with the load-state keys; normalization to SI by the wire rules |
| J4.3 members | Per member, in `pipe_segments` order: `material_selection_kind`, `consumed_material_points`, `interpolation_fraction`, `selected_E_pa`, `selected_nu`, `derived_G_pa`, `retained_G_ignored`, the temperatures, `thermal_definition`, `expansion_law_id`, `coefficient_datum_k`, the consumed law indices and segments, both datum stretches, `thermal_strain`, `thermal_stretch`, `fit_kind`, `fit_input`, `fit_strain`, `fit_stretch`, `total_eigenstrain` (`λ_fit·λ_thermal − 1` by the records' formula), `reference_length_m` |
| J4.4 admissibility | Coverage and positivity over each definition's interval (`CP3_WIRE_ADDENDUM.md` §2), evaluated by the reader. The *consulted* lists are not matched item by item; they keep T1's structural checks. The new check that every consumed and consulted segment's `start_k` and `end_k` lie in `[T_lower, T_upper]` closes the gap `CP4_WIRE_ADDENDUM.md` §2 records |
| J4.5 supports | `support_components` are exactly the rigidly restrained DOFs that carry `boundary_motion`, with normalized value and unit. Every other restrained DOF is zero |
| J4.6 contributions | Stored primitives in `load_sources`, with factor, normalized and applied magnitude; `excluded_sources`; member states (`value` is `total_eigenstrain`); support states; pressure regions |
| J4.7 physical evidence | Each `exact_cases[i].pipe_materials[j]`: `E_pa`, `nu`, `G_pa`, `alpha_per_kelvin`, `thermal_consumed`, `material_selection_kind`, `resolved_eigenstrain` equal the J4.3 values. This replaces `actual_materials` for 0.4.0 |
| J4.8 geometry | `reference_geometry.projection_sha256` recomputed from the authored geometry, provided RD-0 finds a specification. If none exists, it is an addendum request |

Numbers compare by binary64 value (T1's `same`, `load_reference.rs:278-290` at T1), so `1` and `1.0` agree across languages.

**Scope.** The re-derivation covers the whole `resolved_straight_load_state_v1` profile, because a joined envelope's other cases are ordinary load-reference-1 cases (`CP2_WIRE_ADDENDUM_2.md` §5.4, last paragraph). An input outside what the reader can re-derive gives the reason `SOURCE_LOAD_REFERENCE_REDERIVATION_SCOPE` and `needs_recompute`, never eligibility. An i128 overflow is one example.

**Standing** (identical in the three languages):
- **`numerically_eligible`** needs all of the following:
  - J0–J3 pass;
  - an actual invocation is supplied, and J4 passes;
  - the receipt aggregate is `qualified`;
  - no combinations;
  - `MECHANICS_SOLVED`;
  - the requested basis refs equal the receipt's case order.
- **`needs_recompute`** when:
  - there is no invocation (canonical carriers, transport metadata, stored raws without capture);
  - the requested refs differ;
  - the input is outside scope;
  - the aggregate is not `qualified` (impossible for the producer, but checked).
- **`unsupported`** when a J4 comparison fails, because the envelope was not produced from this invocation. This is the physics-source-1 behaviour: an `actual_materials` error becomes `unsupported`, `semantic_contract.rs:400-404`.

**Wiring.**
- T1's early returns are removed, and the joined identity joins the source-identity branch (§4.7 S-1). **[T1-2]**
- TS gains `validateLoadReferenceSourceRecovery(source, invocation, callerModel)`, which registers a validated token like `validateKnownSourceRecovery` (`sourceBlockRecovery.ts:108-278`). The desktop captures the 0.4.0 Value invocation at dispatch, as it already does for source methods.

**Consequence.** An eligible joined result is Current and rule-eligible, because the native rule gate uses Rust standing with the invocation (`src-tauri/src/lib.rs:2800-2837`). Desktop export and the report package keep refusing both load-reference identities until T6 (T1 ruling §12; T0R SF-7). DD-9 asks ROOT to confirm this.

#### 4.2.2 The TypeScript position

The desktop Current gate is TS, and the native rule gate is Rust. If TS stayed invocation-free, a joined result could be rule-eligible natively while the UI showed it as not Current. That breaks outcome parity.

**Recommended (TS-a): a check-for-check TS port in S-E**, with shared cases. This is T1's pattern for the joined ledger (ruling §12).

The alternative (TS-b) is a native-delegated verdict: a Tauri command returns Rust's J4 outcome, and TS registers it. It needs one implementation fewer. But it leaves a browser session with no invocation-capable verdict, which is `needs_recompute`, and it adds an IPC contract. That is decision DD-3.

#### 4.2.3 Dependency on D1

If D1 retires the retained-source join for fresh 0.4.0 solves within T3's horizon (interface I-4), S-E would qualify an identity that fresh solves no longer publish. ROOT should therefore sequence S-E after D1's decision on the joined identity. The design is otherwise independent of D1.

### 4.3 Binding route (in S-E)

1. **No code change in `result_envelope_binding.rs`.** Its gate (`:258`) and `derive_document` already handle the joined identity once standing is eligible.
2. **The T1 test is replaced in place.** `load_reference_route_tests.rs::joined_actual_solve_retains_invocation_bound_receipt_without_canonical_export_both_modes` becomes `joined_actual_solve_mints_bound_evidence_and_canonical_document_both_modes`. **[T1-4]** Over the five committed joined witnesses in both modes it asserts:
   - a document and a `QualifiedPreviewEvidence`, with `canonical_export_unavailability` absent;
   - `validate_document` accepts the document;
   - the receipt is copied, and `qualification_ref == run_id:invocation_digest`;
   - standing is `numerically_eligible` with the invocation, and `needs_recompute` without it.

   These controls must still refuse:
   - a mode relabel (the receipt binds the mode);
   - a forged invocation (hash);
   - a digest-consistent forged proof (T1 wave-2 F1's check keeps its expectation, `CURRENT_NUMERICAL_INTEGRITY_NEEDS_RECOMPUTE`);
   - an invocation whose model differs in one material point or one boundary motion (J4 → `unsupported`).
3. **Carriers.** The committed test-origin carriers stay byte-unchanged as historical fixtures. New route-generated carriers, `fixtures/results/load_reference_source_route_*.document.json`, are generated only by the actual headless route and owned by S-E, with a regeneration script and hashes in the S-E record. A test asserts that route output equals them byte for byte. From then on, the route is the source of the joined carriers.
4. **Headless `lib.rs`** is unchanged. The T6 vacuous-test pattern at `result_envelope_binding.rs:674` stays T6's (T1 ruling §16).

### 4.4 Selected-UNAVAILABLE alignment (slice S-A; can land before T1's merge)

**What the diagnostic means on a selected case.** It says "the bounded retained-source method did not produce a selected response" (`PP:1893-1895`) for a case whose receipt says `selected_method = retained_source_blocks_exact_v1`, and which carries `SOURCE_BLOCK_RECOVERY_SELECTED`. The two facts contradict each other. No producer emits them together (§3.4), so the pair arises only from a producer defect, or an edit resealed with the unkeyed receipt and publication hashes.

**The two directions.**

| Direction | Effect on qualified readers | Effect on committed corpora |
|---|---|---|
| **Tighten** (recommended): physics-source-1 and source-blocks-1 refuse like joined S13 | One added refusal, `SOURCE_BLOCKS_SELECTED_UNAVAILABLE_DIAGNOSTIC`, in `source_blocks.rs::validate_in` (shared by the composite path), `source_blocks.py`, `physics_source.py` if its path is separate, `sourceBlockRecovery.ts` and `physicsSourceRecovery.ts` | None: 0 of 43 and 0 of 83 envelopes carry the diagnostic (§12) |
| Relax: joined S13 removed | Weakens the T1-qualified joined readers in three languages. Four shared cases (`DIAG-unavailable-on-selected*`) flip to accept, and two killed mutants become equivalent | None, but the joined reader admits a contradiction |

**Recommended: tighten**, with the joined rule copied exactly: refuse an `UNAVAILABLE` whose `affected_refs` contains the id of a case with `selected_method == retained_source_blocks_exact_v1`.

- **Placement.** The check goes directly after the `EXACT_SOURCE_REQUIRED` check in the qualified branch, at the same position in each language, so the first failure is deterministic.
- **What it does not do.** A stricter bijection (UNAVAILABLE exactly on failed cases) is not proposed. It is not needed for alignment and it adds surface.
- **Standing effect.** A resealed contradictory envelope goes from `numerically_eligible` (with an invocation) to `unsupported`. The joined reader is unchanged.

### 4.5 source-blocks-1 re-homing

#### 4.5.1 Options

| | Option | Fresh preview-route solves | Carries (1) mixed, (2) abs-sum | Cost |
|---|---|---|---|---|
| **R-B** | **Retire source-blocks-1 for fresh solves, gated on D1** | preview-physics-1, with D1's method supplying accuracy and standing (I-3) | Moot for fresh results. Historical envelopes keep derived reasons | One producer gate, standing reasons, fresh sets, texts, test migrations |
| R-A | Successor identity (proposed name `preview-physics-source-1`, which ROOT would reserve): source-blocks under preview-physics-1 semantics, as physics-source-1 is to physics-1 | Selected cases publish the circular maximum through the retained endpoint-maximum recipe (`source_receipt/composite.rs` `retained_source_endpoint_normal_max_v1`) and their six components. Ordinary cases are rendered by `preview_physics` | Fixed for fresh results | New table, receipt policy, schema branches, readers in 3 languages, producer post-loop changes, fixtures. Retired again when D1 lands |
| R-0 | Keep T0R's containment | source-blocks-1 as today | Stay; texts reworded without "until T3" | Texts only |

#### 4.5.2 Why R-B

- **The admitted set is small.** The bounded method's preview-route domain (order ≤ 2 free blocks, axis-aligned straight members, nodal loads only, at most 256 DOFs; `STAGE0_MAP.md` §2.5) is what D1's general method must cover in any case (I-3).
- **No standing is lost at retirement, provided I-3 holds.** An all-selected envelope, Current today, is replaced on a fresh solve by a preview-physics-1 result that D1 qualifies.
- **Building R-A would build an identity to retire it.**
- **Retiring before D1 would demote correct results.** Today's all-selected envelopes would fall back to sensitive preview-physics-1. So retirement is gated on D1, not scheduled on its own.

#### 4.5.3 Recommended mechanism (slice S-F; after T1's merge and after D1's preview-route method lands)

1. **Producer.** A pre-0.4 model attempts retained source only when `pressure_runtime::is_exact(model)` holds (`PP:1885`). The 0.4.0 path is unchanged. A fresh non-exact solve therefore never publishes source-blocks-1. **[T1-1]**
2. **Standing** (§4.7 S-3). All non-composite source-blocks-1 envelopes become historical:
   - `standing_reason` keeps `SOURCE_BLOCKS_ORDINARY_CASE_LEGACY_SEMANTICS` for mixed envelopes;
   - it adds `SOURCE_BLOCKS_1_HISTORICAL_SEMANTICS` for all others;
   - source-blocks-1 leaves the three fresh sets;
   - `rule_binding_refusal` stays as defence in depth.

   The alternative, keeping historical all-selected envelopes Current, is decision DD-7.
3. **Bytes.** Historical source-blocks-1 bytes, schemas, tables and fixtures stay unchanged and readable. Readers still validate them fully. A tampered one stays `unsupported`, because validation comes first, as in T0R A2 item 10.
4. **Texts** (`knownSemanticLimitations.ts:20-21`, a T1-overlap file) **[T1-5]**:
   - **`N_SB`:** "Historical retained-source result (source-blocks-1). Its summary stress is the sum of absolute axial and bending components, not the circular-section maximum; with nodal loads only it is conservative and at most √2 (about 1.414) times the maximum. It stays readable but is not Current, rule- or export-eligible. Solve again to obtain preview-physics-1 results."
   - **`N_SB_MIXED`:** "Historical retained-source result (source-blocks-1) with an ordinary load case whose rows keep the retired precision-1 semantics (norm-only reactions and an absolute-sum stress summary). It stays readable but is not Current, rule- or export-eligible. Solve again to obtain preview-physics-1 results."
   - If ROOT keeps historical all-selected envelopes Current (DD-7 (i)), `N_SB`'s last two sentences become: "Rule checks cannot bind to it. Solve again to obtain the circular-section maximum."
   - Either way, no text names T3, and the `RULE_SOURCE_BLOCKS_SUMMARY_NOT_RELIABLE` reason stays.
5. **Test migrations.** These are listed in §6.4.

**If I-3 does not hold within T3,** R-B cannot land without a standing loss. T3 then records the carries as open, and ROOT chooses R-A or R-0 (DD-6). Under R-0 the texts are reworded to "until the general accuracy method is available", so that they never promise a tranche that will not deliver.

### 4.6 Transport and display

#### 4.6.1 Cross-unit display representability (slice S-B; can land before T1's merge)

**The rule.** A display conversion is reported `converted` only when one of these holds:
- the input is zero (an exact zero, displayed as "0"; a negative zero displays as "0" too);
- the conversion is the identity (the input bits are returned, `units/src/lib.rs:943-945`);
- the input, the canonical intermediate and the output are all normal binary64.

Otherwise it is `unavailable`, with `DISPLAY_UNIT_RANGE: nonzero value is not representable as a normal binary64 in <unit>; shown in <stored unit>`. Non-finite results are already refused.

For normal operands, a multiplicative conversion has at most two roundings, so the relative error is below 2.3e-16, far inside the 4-significant-digit display. Affine temperature conversions are absolute-accurate; they are not refused for relative loss.

**Where it goes.**
- `display_units.rs` applies the rule around `convert_for_dimension`. The units API is unchanged, so every other caller keeps its behaviour.
- `unitConversion.ts::convertForDisplay` returns `null` (entered-only) under the same rule.
- `QuantityReadout` already falls back to the stored value with a notice for `unavailable` (`display-units/index.tsx:90-93`).
- No WASM artifact is committed (`loadWasmEngine.ts:1-13`); it is built from `operation_applier`.

**Tests.**
- Rust unit tests on `display_units`, covering the probe's eight items (PR-6).
- TS tests on `convertForDisplay` / `dualUnitDisplay`: a nonzero value that would display "0" displays entered-only.
- No existing expectation changes. If one is found asserting a subnormal or zero conversion as `converted`, it is changed with its justification recorded.

#### 4.6.2 Scientific transport and the 2^53 − 1 limit

- **What belongs to T3** (the numerical requirement): every finite binary64 result value either survives raw publication, readers, display and canonical transport bit for bit, or is refused with a named, specific reason. It is never silently altered. T3 supplies:
  1. **Reference cases.** R1's RF-RANGE family extends to published values in `[2^53, 1e21)`, `1e21` and up, and subnormals, through the public entry and the headless route. P1 records where each is carried or refused.
  2. **Reader acceptance.** Every T3-touched reader accepts every finite JSON number as binary64, as T1 did for the load-reference readers (`CP4_WIRE_ADDENDUM.md` §2). A reader that refuses a finite value is a T3 reader defect.
  3. **The fallback.** A source receipt that cannot be hashed because a published value is outside the checked profile is a finalization failure, and S-D republishes it ordinarily. That is designed behaviour (probe PR-5).
  4. **Display representability** (§4.6.1).
- **What belongs to T6** (export contracts, structured outputs, persistence): adopting `openpipestress_jcs_binary64_v1` for result-value positions when canonical results documents, AnalysisRuns and stress-neutral packages are materialized and hashed (the Rust, Python and TS guards cited in §3.6), and in persistence and desktop export. Two conditions apply:
  - **Exact-integer positions** (counts, indices, work units, schema-owned integers) keep the checked integer rule, from an inventory per schema, as the scientific return requires.
  - **Existing hashes stay valid.** Both profiles format with ECMAScript, so every document the checked profile admits must canonicalize to identical text under binary64. T6 proves this by a differential over every committed carrier. PR-7 runs a 200,000-value pre-check.
- **Source receipts** keep declaring `openpipestress_jcs_ijson_v1`. Changing a receipt's canonicalization would change its bytes, and the bounded method is retiring (R-B) or bounded anyway.
- **In short, for T1's note:** the limit is a carrier-profile property. The reader is not changed, the carriers are adopted by T6, and T3 owns the range requirement and its evidence (DD-8).

#### 4.6.3 Comparison policy for new T3 cases

- **Numerical references** (R1's families and the D2 standing cases that compare values) use the existing relative form `|obs − exp| ≤ 1e-9 · max(|exp|, scale)`, with each zero-valued expectation's scale derived and stated. This is ROOT's answer, `STAGE1_PLAN.md` §8.4.
- **Transport and display cases** compare bits, or the representability class. They use no tolerance.
- **Reader and standing parity cases** compare outcomes and error codes exactly.
- **Protected predicates are not touched:** the absolute 1e-9 benchmark, DEC-026 and DEC-050/053. D2 found none to be wrong, so no options go to ROOT and no approving instrument is needed.

### 4.7 Standing across languages

The baseline is T0R's standing edits (`standing_reason`, `rule_binding_refusal`, the static fresh sets) plus T1's: the joined early return, and the fresh sets gaining the two load-reference identities (`LSI/CHECKPOINT_6.md`, Resolutions).

| Change | Rust (`result_export/src/semantic_contract.rs`) | Python (`analysis_runs/compatibility.py`) | TS | Slice |
|---|---|---|---|---|
| S-1 joined eligibility | Remove T1's early return (`:470-476` at T1). Add `LOAD_REFERENCE_SOURCE_ID` to the source branch (`:385-405` pattern) with `load_reference_source::validate_with_invocation` | Remove the early return (`:335-339` at T1). Add the joined contract to the source branch with `source_block_context` | `numericalResultStanding` → `loadReferenceSourceStanding` reads the validated token | S-E |
| S-2 selected-UNAVAILABLE | No standing-function edit. The validators refuse, giving `unsupported` | same | same | S-A |
| S-3 source-blocks retirement | `standing_reason` adds `SOURCE_BLOCKS_1_HISTORICAL_SEMANTICS` after the mixed reason. `FRESH_IDENTITIES` drops source-blocks-1 | `_standing_reason`, `FRESH_CONTRACT_IDS` | `standingReason`, `FRESH_SEMANTIC_CONTRACT_IDS`, `sourceBlockStanding` finding | S-F |
| S-4 composite fallback | none (existing identities) | none | none | S-D |

**How parity is tested.** Each slice adds one shared JSON case file with these columns:
- the envelope reference;
- the invocation reference, or null;
- the requested refs;
- the expected standing;
- the expected reason or error code.

The Rust test, the Python test and the TS test each consume it, as T1's `load_reference_source_mutations.json` and `loadReferenceSourceLedger.cases.json` are consumed. A parity summary records exact agreements, declared language-specific strings (with reasons), zero undeclared differences, and zero cases accepted by one language and refused by another. Each new branch has a mutant in each language, killed by a named shared case.

## 5. Interface assumptions on D1

D2 depends on D1 only here. D1's design is not read or designed here.

- **I-1 (row semantics).** Where D1's general method runs on the ordinary route, its results publish with the ordinary identities' row semantics: preview-physics-1, physics-1, load-reference-1. Or they publish under a D1-proposed identity whose rows have those semantics.
- **I-2 (standing evidence).** D1's results earn standing either through `numerical_quality` `checks_passed` (then no standing-function change), or through a receipt or certificate the readers verify. In the receipt case, §4.7 gains a D1 branch built like S-1: invocation-bound, re-deriving the operands that enter the solve, one shared case file, mutants in each language. The fresh sets gain the D1 identity once ROOT reserves it.
- **I-3 (domain).** On the preview route and in both modes, D1's method covers at least the bounded retained-source domain, with N05 and N06 as positive controls. **R-B (S-F) waits for this.** If I-3 fails, DD-6 applies.
- **I-4 (existing source identities).** D1 says whether physics-source-1 and load-reference-source-1 remain fresh identities.
  - S-D stays correct either way; only its reach shrinks.
  - S-E is worth building only if the joined identity remains fresh through T3's horizon.
- **I-5 (range).** D1's range scaling (PHYS-R4) is independent of §4.6. The display and transport requirements apply to whatever values D1 publishes.
- **I-6 (receipt contents).** If a D1 receipt publishes values beyond ±(2^53 − 1) or hashes with the checked profile, the §4.6.2 fallback rule applies to it as well.

**Validity if D1 chooses differently.** Items 1, 3 (given 2), 4, 6 and 7 hold as written. Item 2 depends only on I-4. Item 5 depends on I-3 and has a stated fallback.

## 6. Verification plan

### 6.1 Probe reproductions

Sources are in `_run_records/d2_probe/`, the run plan and predictions in `_run_records/PROBE_PLAN.txt`. They build from exported copies of `c61a540ea` and `f3270ea79`, never from T1's worktree.

| Probe | Input | Predicted main | Predicted T1 | Used for |
|---|---|---|---|---|
| PR-1 | P12 | `Err` both modes | `Err` (pinned) | S-D trigger; replaced test's input |
| PR-2 | P12 with N06 as the selected case | `Err` | `Err` | Residual R-1b |
| PR-3 / 3b | 21 / 2 copies of the source-blocks N05 case | `Err` / `Ok` source-blocks-1 | same | source-blocks trigger and control |
| PR-4 | P3 | fixture absent | `Ok` load-reference-1 (SF-1) | 0.4.0 baseline |
| PR-5a/b/c | N05 with a 1e16 / 1e16 / 1e15 N·m torque | `Err` / `Err` / `Ok` | same | Receipt-hash range trigger |
| PR-6 | 8 display conversions | subnormal or zero outputs reported `converted` | same | S-B baseline |
| PR-7 | guards, both profiles, 200,000-value differential | checked profile refuses every \|x\| > 2^53 − 1; binary64 admits; `differ=0` | same | §4.6.2 |

After implementation, the same probes run on the candidate. PR-1, 2, 3 and 5 must give `Ok` or a blocked envelope, never `Err`. PR-6 must give `unavailable` for the underflow items.

### 6.2 Per-slice controls

| Slice | Positive controls | Negative, tamper and mutation controls |
|---|---|---|
| S-A | Every committed physics-source-1 and source-blocks-1 raw and carrier still accepted; standing unchanged (digest of all outcomes before and after) | Four shared cases per family, mirroring T1's (`unavailable-on-selected`, resealed, among refs, and an ordinary-case control that is accepted). The mutant disabling the check is killed in each language |
| S-B | Identity subnormal, exact zero, negative zero and normal round trips unchanged | Subnormal-in, subnormal-out and zero-out are `unavailable`; mutants removing each clause are killed |
| S-C | physics-source-1 corpus outcomes identical (outcome-neutral refactor) | A mutant where `External` still runs `actual_materials` is killed by a 0.4.0 fixture in S-E |
| S-D | PR-1/3/5 give ordinary publications, rows bit-equal to the typed route, readers accept them in 3 languages, standing `needs_recompute`. The committed witnesses still select | The mutant restoring the pre-0.4 gate fails the replaced test. PR-2 gives a blocked envelope with the cause. The invariant guard is never reached over all S-D tests (a counting hook in the test build). Ledger: `charged ≤ invocation_limit` and `attempts` counts both runs (T1 backcheck B-1's lesson) |
| S-E | All five joined witnesses, both modes, eligible with their invocation, in 3 languages; route documents minted | One mutation per J4 row (for example E off by 1 ulp, a swapped consumed point, a wrong interpolation fraction, a segment outside `[T_lower, T_upper]`, a moved boundary motion, a dropped excluded source, `resolved_eigenstrain` changed) gives `unsupported`. No invocation gives `needs_recompute`. A logarithmic-law field moved by 1 ulp is accepted and by 2 ulp refused, the same in all three languages. A scope refusal gives `needs_recompute` |
| S-F | A fresh solve of each `source_blocks/*.request.json` gives preview-physics-1 with D1 standing. Historical raws readable, with the new reasons | Tampered historical raw is `unsupported`. The mutant keeping source-blocks-1 in a fresh set is killed |

### 6.3 Pre-0.4 differential (S-D, S-F)

Following the CP4 review, every committed pre-0.4 request in the producer fixture folders, plus the probe inputs, runs through the base and the candidate in both modes. Outputs must be byte-identical except on the inputs the slice is meant to change: the fallback triggers for S-D, and the non-exact source-selected requests for S-F. Each changed input is listed with its reason.

### 6.4 Existing tests that change

None is deleted, and none is weakened.

| Test | Slice | Change and justification |
|---|---|---|
| `product_physics/src/source_receipt/load_state_fallback_tests.rs::the_pre04_composite_finalization_failure_is_unchanged_by_this_fallback` | S-D | Renamed and turned into a behavioural test (§4.1.4). It pinned the defect as a characterization only; T1 labelled it "not an endorsement" |
| `headless/src/load_reference_route_tests.rs::joined_actual_solve_retains_invocation_bound_receipt_without_canonical_export_both_modes` | S-E | Replaced by the positive route test (§4.3). It pinned T1's "not eligible in T1" ruling, which S-E supersedes |
| Joined standing assertions (`needs_recompute` for an admitted joined envelope) in the Rust, Python and TS reader tests | S-E | With an invocation → `numerically_eligible`; without → `needs_recompute` (kept). Comments stating "never eligible" are updated |
| Doc comments in `load_reference_source.rs:29-31` and the Python and TS peers | S-E | Updated to the new standing |
| `product_physics/tests/source_block_recovery.rs` (independent N05/N06 × sign × rotation × mode) | S-F | Identity assertions change from source-blocks-1 to preview-physics-1. The physical value assertions against the independent annulus reference are kept unchanged and must pass under D1 |
| Headless `actual_source_blocks_bind_exact_invocation_and_canonical_source_both_modes`, `source_block_capture_retains_material_overrides_order_and_extra_request_fields`, `actual_value_stress_publication_range_refusal_cannot_qualify_export` | S-F | Split in two. (a) The historical raw with its committed invocation reads with the new standing. (b) A fresh solve publishes preview-physics-1. Capture-custody assertions move to a surviving source identity (physics-source-1) where they are identity-independent |
| Python `tests/test_source_blocks_validation.py` and TS `sourceBlockRecovery.test.ts` / `resultsSessionState.test.ts` / `ruleCheckService.test.ts` standing expectations on source-blocks-1 fixtures | S-F | Eligible → `needs_recompute` with the new reason, under DD-7 (ii) only |
| `knownSemanticLimitations.test.ts` / `KnownSemanticNotices.test.tsx` text pins | S-F | New texts |
| T0R's three fresh-set pins (Rust, Python, TS) | S-F | Now name five identities, and they stay exact pins |

An audit in S-F also checks the packaged self-test and the native tests in `src-tauri` for producer-driven source-blocks-1 use.

## 7. Slices, order and T1 serialization

| Slice | Content | Write set (main paths) | T1 overlap | When |
|---|---|---|---|---|
| **S-A** | Selected-UNAVAILABLE tightening | `result_export/src/{source_blocks.rs, physics_source.rs}`; `analysis_runs/{source_blocks.py, physics_source.py}`; `apps/desktop/src/features/results/{sourceBlockRecovery.ts, physicsSourceRecovery.ts}`; shared case files; `P/tests/test_source_blocks_validation.py`, `test_physics_consumer_contract.py`; Rust tests in `result_export/tests/` | disjoint | **Before T1 merges** |
| **S-B** | Display representability | `operation_applier/src/display_units.rs` (not in T1's diff); `apps/desktop/src/services/unitConversion.ts`; tests | disjoint | **Before T1 merges** |
| **S-C** | `physics_source` material-check parameter (outcome-neutral) | `result_export/src/physics_source.rs`; `analysis_runs/physics_source.py`; `physicsSourceRecovery.ts` if its structure needs it | disjoint | **Before T1 merges** |
| **S-D** | Composite fallback for pre-0.4 | `PP`; `source_receipt/load_state_fallback_tests.rs`; a new `source_receipt/pre04_fallback_tests.rs` | **T1** | After T1 merges and main is merged into the T3 branch. ROOT serializes `PP` |
| **S-E** | Joined eligibility, binding route, TS port | New `load_reference_rederive.{rs,py}`, `loadReferenceRederive.ts`; `load_reference_source.rs`, `load_reference_source.py`, `loadReferenceSourceEvidence.ts`; `semantic_contract.rs`; `compatibility.py`; `numericalResultQuality.ts`; the desktop invocation capture; `headless/src/load_reference_route_tests.rs`; new route carriers | **T1** | After T1 merges; after S-C; after D1's I-4 answer |
| **S-F** | source-blocks-1 retirement | `PP`; `semantic_contract.rs`, `compatibility.py`, `knownSemanticLimitations.ts`, `sourceBlockRecovery.ts`; the tests in §6.4 | **T1** | After T1 merges and D1's preview-route method (I-3) |
| T6 handoff | binary64 adoption in the canonical carriers | T6's own | n/a | Routed by ROOT (DD-8) |

- **Order.** S-A, S-B and S-C run in parallel and independently. After the T1 merge: S-D (small, first) → S-E → S-F.
- **Merge gate** (per `STAGE1_PLAN.md` §5):
  - complete-diff independent review;
  - hosted CI, including the dual-viewport dispatch whenever TS changes;
  - a clean DEC-025 sweep;
  - native witnesses on the owner's Mac, recorded as outstanding if not yet available (S-E: a joined result shown Current in the native app with its invocation; S-D: a P12-class invocation publishes).
- **Each slice is one atomic PR** across its three languages, so parity never lands half-way (T0R SF-3's lesson).

## 8. What T3 completes and what remains

| Finding item | After this design, with D1 | What remains |
|---|---|---|
| Composite `SOURCE_BLOCKS_FINALIZATION_FAILED` | Closed as a finding: no invocation returns `Err`, and no failed or sensitive case gains standing (S-D) | Residuals R-1a and R-1b until D1's general method lands, or F2 |
| Joined eligibility (T1 §7) | Closed if the joined identity stays fresh (S-E) | TS parity under DD-3's choice |
| Binding route (T1 §11) | Closed with S-E | Desktop export of both load-reference identities stays T6 (T1 §12). The vacuous binding test pattern stays T6 (T1 §16) |
| Selected-UNAVAILABLE (T1 WP1 N-2) | Closed (S-A) | None |
| T0R carries R-1, R-2 | Closed for fresh results with S-F. Historical envelopes carry derived reasons permanently | If I-3 fails: DD-6 |
| M34 display range | Closed for the display chain (S-B) | None in T3 |
| M34 transport range | T3 range requirement, references and fallback delivered | Canonical carrier adoption: T6 (DD-8). Persistence: T6 (already routed) |
| M34 comparison policy | Stated; no protected predicate changes | None |
| Case-scoped Current (F3) | Not proposed | Open only if ROOT or the owner wants it |

Containment alone closes no finding group. Group closure follows the graph's rule and needs the paired VP-ORACLES and VP-ROBUST evidence on the merged candidate.

## 9. Decisions for ROOT

Each decision is technical and within ROOT's delegated correctness authority. The recommendation is listed first.

| ID | Decision | Options (recommended first) |
|---|---|---|
| DD-1 | Composite finalization policy | **F1** port SF-1 / F2 partial receipt (new physics-source receipt policy) / F3 F2 plus case-scoped standing |
| DD-2 | Accept residuals R-1a and R-1b as open until D1 | **Accept, and record them in the graph's T3 row** / close them now with F2 |
| DD-3 | TS position for joined eligibility | **TS-a** check-for-check port / TS-b native-delegated verdict |
| DD-4 | Host-rounded (`exp`) fields in the re-derivation | **Faithful bound (one ulp) on the recorded value, exact argument** / exact equality (host-dependent: a macOS-produced result may fail on Linux or in V8) / exclude logarithmic-law cases from eligibility (`needs_recompute`) |
| DD-5 | Selected-UNAVAILABLE direction | **Tighten** physics-source-1 and source-blocks-1 / relax joined S13 |
| DD-6 | source-blocks-1 re-homing | **R-B gated on D1's I-3** / R-A successor identity / R-0 keep containment with reworded texts |
| DD-7 | Historical all-selected source-blocks-1 at retirement | **(ii) historical-only, like precision-1** (re-solving is always possible) / (i) keep it Current, with source-blocks-1 left in the fresh sets and the summary refusal kept |
| DD-8 | Transport split | **T3 owns the range requirement, references, reader acceptance, fallback and display; T6 adopts binary64 in the canonical carriers and persistence** / T3 adopts it in the carriers now |
| DD-9 | Confirm that eligible joined results are Current and rule-eligible, with desktop export and the report package still refused until T6 | **Confirm** / keep joined results off rule checks until T6 |

**Owner-level question.** None. If ROOT prefers F3, whether "Current" may apply to one load case of an envelope is a product-semantics choice that no accepted instrument covers. It would go to the owner with three options:
- (a) whole-envelope Current only (recommended);
- (b) case-scoped Current for source identities only;
- (c) case-scoped Current on every route.

## 10. Places where T1's final merge could change this design

- **[T1-1]** SF-1's final shape: the budget field names, the wrapper, `decline_withheld`, the ledger rule. S-D generalizes whatever T1 merges; if T1 changes SF-1 again, S-D follows it.
- **[T1-2]** The joined reader's step structure (J0–J3, `prepass`, the projection), the position of T1's standing early return, and the TS joined port. S-E wires into the merged versions.
- **[T1-3]** The joined record and wire records. The re-derivation is specified from them. Any addendum after `f3270ea79` is re-read in RD-0.
- **[T1-4]** `load_reference_route_tests.rs` test names and assertions.
- **[T1-5]** `knownSemanticLimitations.ts` and the fresh-set constants.
- **[T1-6]** Any change to T1's S13 rule before merge. S-A copies the merged rule.
- **[T1-7]** T1's desktop export refusal gate and its T6 routing (DD-9 assumes ruling §12).
- **[T1-8]** The name and location of the characterization test.
- **[T1-9]** The committed joined witnesses and carriers, which S-E's positive controls use.

## 11. Records consulted

- **Instructions.** Root `AGENTS.md`, `agents/AGENT_TASK.md`, and `agents/AGENT_HELPS_HUMANS.md`. The last was read **deliberately**, for the design posture the brief names, and is recorded here as the wider consultation the Root doctrine requires.
- **Brief and common terms** (sha256 prefixes at `e14f7fd13`):
  - `T3/TASK_BRIEFS/D2_STANDING_DESIGN.md` `ce8477bc8009af2e`;
  - `_COMMON.md` `892a2e4e6f8b2e68`;
  - `T3/STAGE0_MAP.md` `cfdb9dab66b0aa58`;
  - `T3/STAGE1_PLAN.md` `f5c75dcf5bb1bb4a`;
  - `T3/OWNER_DIRECTION.md` `7e4068e584148997`.
- **T0R** (at `e14f7fd13`):
  - `DEFAULT_ROUTE_DESIGN/DESIGN.md` `5fdadc0d11754294`, §1–7;
  - `ROOT_SELECTION.md` `923ec424ea322a20`;
  - `ROOT_RULINGS.md` `81af712fe10472e6`.
- **Other records:**
  - `CORRECTNESS_DESIGN/COMPOSITE_ENGINE/SELECTION.md`, by search;
  - `ENGINE_INTEGRATION/RETURN.md`, the conversion-scope paragraph;
  - `SCIENTIFIC_TRANSPORT_FOUNDATION/RETURN.md`.
- **T1 at `f3270ea79`** (sha256 prefixes), all under `LSI/`:
  - `CP4_WIRE_ADDENDUM.md` `93a2c6786b5e213f`;
  - `CP2_WIRE_ADDENDUM_1.md` `c389f5e3878c6a32` (§3–5);
  - `CP2_WIRE_ADDENDUM_2.md` `ec66628ef8db1ac7` (§5–6);
  - `CP3_WIRE_ADDENDUM.md` `f69043b682d027cc` (§2);
  - `T1_WAVE1_RULINGS.md` `9469952871067a73`;
  - `CHECKPOINT_4.md` `6997b6a959708347`;
  - `CHECKPOINT_5.md` `2b246465c7907048`;
  - `CHECKPOINT_6.md` `af1fec41e6346a21`;
  - `T1_WP1_JOINED_READERS/RETURN.md` `94acf9e43c6fc92c`;
  - `T1_WP4_HEADLESS/RETURN.md` `21c6fb46efde8f2e`;
  - `REVIEW_CHECKPOINT_3/RETURN.md` `0834b1a5281cf718`;
  - `REVIEW_CHECKPOINT_3/_run_records/review3_probes.rs.txt` `b119196432e51300`;
  - `REVIEW_CHECKPOINT_3/_run_records/probes_final.log` `480ad49f6cdedad5`;
  - `REVIEW_CHECKPOINT_4/RETURN.md` `3e713809d33c63f0`;
  - `REVIEW_CHECKPOINT_4/BACKCHECK.md` `703beaa36235e2de`.
- **Product source at `c61a540ea`:**
  - `PP` (entry, case loop, attempt, per-case and invocation finalization, `blocked_envelope`);
  - `source_receipt.rs`, `source_receipt/composite.rs`;
  - `result_export/src/{semantic_contract.rs, source_blocks.rs, physics_source.rs, derivative.rs}`;
  - `analysis_runs/{compatibility.py, source_blocks.py, physics_source.py}`;
  - `headless/src/{result_envelope_binding.rs, lib.rs}`;
  - `units/src/lib.rs`;
  - `operation_applier/src/display_units.rs`;
  - `canonical_json/src/{lib.rs, binary64.rs}`;
  - `apps/desktop/src/{features/results/{numericalResultQuality.ts, knownSemanticLimitations.ts, sourceBlockRecovery.ts, physicsSourceRecovery.ts}, services/{displayQuantityService.ts, unitConversion.ts}, features/display-units/index.tsx}`;
  - `src-tauri/src/lib.rs` (the rule gate);
  - `schemas/physics_source_recovery.schema.json`;
  - the committed `fixtures/product_preview/{physics_source, source_blocks}` raws.
- **Product source at `f3270ea79`,** through `git show` and the three-dot diff:
  - `PP` (wrapper, attempt, failure recording, entry);
  - `source_recovery.rs` (reserve, decline);
  - `source_receipt/load_state_fallback_tests.rs`;
  - `case_state/{thermal,temperature,material}.rs` (headers and operation markers only, to state the arithmetic facts; not used as a specification);
  - `result_export/src/{load_reference.rs, load_reference_source.rs, semantic_contract.rs, derivative.rs}`, with the diff of the last two;
  - `analysis_runs/compatibility.py` (diff);
  - the TS `numericalResultQuality.ts` and `knownSemanticLimitations.ts` diffs;
  - `headless/src/load_reference_route_tests.rs` (outline).
- **Not read:** D1's and R1's outputs (none existed); T1's worktree (never touched).

## 12. Sources, probes and limits

**Run.**
- `_run_records/scan_unavailable.py.txt` is a standard-library Python scan of every committed JSON blob under `P/` at `c61a540ea` and `f3270ea79`, through `git show`. It is read-only and needed no build. Output (`_run_records/scan_unavailable.log`):
  - main: 10,314 JSON files, 43 receipt-bearing envelope objects, 0 carrying `SOURCE_BLOCK_RECOVERY_UNAVAILABLE`;
  - T1: 10,534 JSON files, 83 receipt-bearing envelope objects, 0 carrying it.
- Git reads (`git show`, `git diff`, `git grep`, `git ls-tree`) and one read-only survey of committed fixtures (receipt statuses, work charges and diagnostic codes, quoted in §3.1 and §4.1.5).

**Prepared, not run (host hold).**
- The probe crate `_run_records/d2_probe/{Cargo.toml.txt, src/main.rs.txt}`.
- The run plan and predictions in `_run_records/PROBE_PLAN.txt`.

Every behavioural prediction marked "predicted" in §6.1 is unverified until those probes run. Nothing in §3 depends on them: §3 cites source and T1's own recorded probe logs.

**Not done.**
- No build, test, probe or suite.
- No Git write, and no product, test, fixture or schema edit.
- No read of D1's work.
- No material, component or code-rule data.

**Limits.**
- The claim that readers accept a resealed contradictory envelope with eligibility (§3.4) is by reading. The S-A tests establish it.
- The re-derivation's arithmetic specification may be incomplete in T1's records. RD-0 finds out before any code is written.
- R-B's gate (I-3) depends on D1's design, which this design does not evaluate.

`_run_records/SHA256SUMS` lists this folder's files.
