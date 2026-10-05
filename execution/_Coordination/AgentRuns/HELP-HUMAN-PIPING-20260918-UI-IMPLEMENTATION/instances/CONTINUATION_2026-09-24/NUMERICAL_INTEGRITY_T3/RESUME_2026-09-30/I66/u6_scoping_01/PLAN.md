# I66: U6 carriers and standing, and the reader round U7 needs (scoping plan)

**Headline.** U6 is bounded but larger than I61's figure: about **19–26 agent-hours** for the carriers in four units, plus **6–9 hours** for the reader round, plus **12.5–16.5 hours** of review. I61's plan had 6–10 hours plus 2–3 hours of review.
- **What drives the size:** the TypeScript registration and standing work, and the derivative's class disclosures. At the milestone, 69 of the 98 or 99 rows are `absolute_verified`.
- **The critical path:** with the units run in parallel, U6 takes about **15–20 hours elapsed**. That appears shorter than the rest of the chain to U7: U4 G4–G6 (the U4 plan estimated 33–49 authoring hours across G2–G6, RR "U4 plan"), then U3 grant 2, which waits for G5, then the live milestone. So U6 should stay off the critical path if it is dispatched now. The TypeScript unit is the one to watch.
- **The first unit is an end-to-end slice:**
  - the input is the producer's pinned milestone successor, both modes, byte-identical to PP's committed pins;
  - it is carried through the real Rust canonical derivative (`derive_document`, then `validate_document`);
  - it comes back out with its receipt byte-identical and revalidated;
  - standing is `needs_recompute` with eligibility off.
- **Nine decisions are proposed for ROOT. None is owner-reserved.**

This plan is records only: no code, no Cargo, npm or pytest runs, and no Git writes.
- **When:** about 06:35Z to 07:20Z on 2026-10-04, with the memory guard (PID 5387) running.
- **Basis:** NUM `fc5d92c56c`. That is the brief's `4c0b9c735a` plus ROOT's U3 grant 1 merge (`b1f80234dc`) and records. HEAD moved to `7e4f5a51dd` (RV86, records only) while this was written. No reader or carrier file changed across either step; `_run_records/BASIS.txt` records this, and READ_ORIGINS.json hashes the files read at `7e4f5a51dd`.
- **U3 grant 1b:** read only as committed `4b31bbf23a`, via `git show`. I61's working tree was not read.
- **Main:** `origin/main` is `09106477e3`. It has changed no maintained piping file since the merge base `381be775ae`.

**Abbreviations:**
- P = projects/chirality-piping
- PP = P/core/product_physics/src
- RS = P/core/reporting/result_export/src
- PY = P/core/analysis_runs
- TS = P/apps/desktop/src
- R = the RESUME_2026-09-30 record root
- RR = T3/ROOT_RULINGS_V1.md at `fc5d92c56c`
- D1 = DESIGN_NUMERICS/DESIGN.md
- D2 = DESIGN_STANDING/DESIGN.md
- C1 = R/I32/f2a_wire_c1/WIRE_CONTRACT.md

## 0. Facts that shape the plan

| # | Fact | Evidence |
|---|---|---|
| F-1 | **No production caller can deliver a successor in the milestone domain.** The Direct entry has no product caller. Tauri calls only the ordinary wrapper, and a source test forbids it from calling the retained entries. Headless is refused under D-2. So in the first package the successor reaches **library consumers** (Rust, Python). The desktop needs correct but fail-closed handling, and a native desktop witness of a successor is impossible until native W1 is activated. | src-tauri/src/lib.rs:1558–1563; P/core/product_physics/tests/retained_precision_admission.rs:217–222; RR "U4 plan" D-1, D-2; RR "U3 grant 1 verified" F-5 |
| F-2 | **Every carrier fails closed today.** Rust and Python standing say `unsupported`. TS says `needs_recompute` with finding `SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED`. The AnalysisRun, derivative and desktop exports refuse the successor. The Python stress-neutral packager refuses it through its own `SUPPORTED_METHODS`. | RS/semantic_contract.rs:211–224, 462–464; PY/compatibility.py:240–241, 327–330; TS/features/results/numericalResultQuality.ts:70, 92; P/core/handoff/stress_neutral/package_v0_3.py:32, 405–407 |
| F-3 | **The Python reader's public entry refuses every input while incomplete.** Rust and TS run all gates and hold only eligibility. A Python carrier therefore cannot obtain `needs_recompute`, which breaks D2 §4.7's three-language standing parity. | PY/retained_precision.py:29, 1582–1585 versus RS/retained_precision.rs:4260–4312 and TS/features/results/retainedPrecision.ts:97, 1299–1319 |
| F-4 | **69 of 98 (sparse) and 69 of 99 (dense) milestone rows are `absolute_verified`** (25 relative, 3 input-derived). The derivative's 1:1 row accounting forbids "a disclosure beside the value". | R/I61/receipt_experiment_02/_run_records/checks_and_class_parity.txt; RR "RV85 … U5" (97 class claims); RS/derivative.rs:443–451 |
| F-5 | **D2's G4 premise holds only in the schema.** D2 says base readers "already refuse a `retained_precision` member through their closed field lists". In fact the base preview-physics-1 readers in all three languages refuse `source_block_recovery` and `carrier_evidence` but **not** `retained_precision`. The results schema does forbid it (RV78). U6 therefore needs a dispatch-level downgrade guard. No base-reader edit is needed. | D2 §4.9.3 G4; RS/preview_physics_evidence.rs:248–251; PY/preview_physics_evidence.py:142; TS/features/results/previewPhysicsEvidence.ts:127; R/REVIEW_RV78/carried_artefacts_01/REVIEW.md §2 |
| F-6 | **The typed envelope cannot carry a successor.** `MechanicsEnvelope` and `ResultItem` have no `retained_precision` or row `recovery_method`. The headless runner holds the typed envelope and calls `into_parts()`. Only R-1's `into_publication()` preserves a successor. | PP/lib.rs:817–836, 1981–1994; P/core/runner/headless/src/lib.rs:691, 772–776; `4b31bbf23a`:PP/lib.rs:2187–2246 |
| F-7 | **Reader eligibility omits one conjunct.** It does not apply D2 §4.9.4's ordinary-eligibility conjunct to `not_required` cases (checks_passed, passive basis, retained fidelity, evidence refs). This is unreachable in D1, where one case means that case is selected. | RS/retained_precision.rs:4300–4305; PY/retained_precision.py:1700; TS retainedPrecision.ts:1317; C1:160; D2 §4.9.4 |
| F-8 | **The pinned successor bytes come from committed code.** `u3_permitted_path_publishes_the_pinned_successor` writes them with `I61_U3_OUT`. Their sha256 values are pinned twice. | PP/retained_facade_tests.rs:13–16, 59–80; PP/retained_wire_tests.rs:26–29 |

## 1. Carrier inventory

"Today" is the behaviour at `fc5d92c56c`. "Fails closed" means the successor is refused or reads as not Current; it never gains reliance.

### 1a. Rust (`RS`, T3; Tauri and headless only call into it)

| Carrier | File:line | Schema/shape | Accepts the successor today? | U6 change |
|---|---|---|---|---|
| Header dispatch | semantic_contract.rs:183–341 (allow-list :211–222; profile :309–316; table :327–336) | results 0.3 metadata | **No, fails closed:** `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` | Successor branch: `retained_precision::validate_transport_metadata` → (retained table, "0.3.0"). Add a `RETAINED_PRECISION_DOWNGRADE_FORBIDDEN` guard: no other identity may carry `retained_precision` or a row with the W1 token (F-5). Mirrors :234–240. |
| Raw dispatch | semantic_contract.rs:344–377 | raw envelope | **No, fails closed** | Successor branch: `retained_precision::validate(source, None)` (G0–G7). Base branches unchanged; the reader's G7 calls them on its projection, so there is no recursion. |
| Current-admission set | semantic_contract.rs:402–413 | — | **No** | Add the successor id (D-U6-6). |
| Standing | semantic_contract.rs:453–557 | — | **No:** `unsupported` (:462–464) | Successor branch; see §3. Never reaches the `numerical_quality` branch (:504–556). |
| Rule binding refusal | semantic_contract.rs:433–441 (called at src-tauri/src/lib.rs:3216; the reason reaches core/rules/rule_check_runner's `RefusedSolverResult`, :92–99) | — | n/a | Classes from a **validated** receipt only. `absolute_verified` → `RULE_QUANTITY_BELOW_VERIFIED_FLOOR`; `not_covered` → `RULE_QUANTITY_NOT_COVERED`. A headline naming such a row is refused the same way (D2 §4.9.9). |
| Classification summary | (new fn in semantic_contract.rs) | D2 §4.9.9 shape | n/a | `classification_summary(source, invocation)` over validated classes. |
| Canonical derivative | derivative.rs:58–321 (for_source :64; evidence copy list :95–106; receipt copy list :107–118; disposition :176–184) | results.v0.3 `ResultEnvelope` successor branch (schema exists) | **No, fails closed** at :64 | Copy `retained_precision` and `contract_evidence` (D2 §4.9.7). Route `absolute_verified` and `not_covered` rows to `row_disclosures` with class and bound (D-U6-2). Refuse a receipt on any other identity. |
| Derivative validator | derivative.rs:323–… (receipt check :327–343; evidence :350–354; disposition recompute :473–489) | same | **No, fails closed** | Exact receipt equality → `RETAINED_PRECISION_RECEIPT_BINDING_MISMATCH`. Recompute the class dispositions from the validated receipt. |
| Headless canonical export | P/core/runner/headless/src/result_envelope_binding.rs:236–269 (standing gate :258) | typed `MechanicsEnvelope` in | **Unreachable** (typed envelope; D-2) | **None in U6.** This is wider F2a with Headless admission: the runner output carries a Value and uses `into_publication()` (F-6). |
| Tauri rule standing and binding | src-tauri/src/lib.rs:2794–2835, 3208–3224 | Value | Fails closed (standing `unsupported`) | **No edit.** It inherits the RS helpers. TS passes the registered invocation as the existing `sourceBlockInvocation` argument (§1d). |
| Tauri persistence | src-tauri/src/lib.rs:729, 1016 (`mechanics_result_json`, `analysis_run_json`) | JSON text | **Preserves bytes** | No edit. Pinned by a TS reopen test (§4). |
| R-1 interface | `4b31bbf23a`:PP/lib.rs:2187–2246 | `RetainedPublication {Ordinary, Successor(Value)}` | Always `Ordinary` without a permit | U6 reviews it as the carrier interface (§6, U6f, items C-1 to C-3). No PP edit by U6. |

### 1b. Python (`PY`, T3; the T6 packager is read-only)

| Carrier | File:line | Schema/shape | Accepts the successor today? | U6 change |
|---|---|---|---|---|
| Dispatch | compatibility.py:226–318 (allow-list :240; downgrade :251; preview branch :304–306) | raw/transport | **No, fails closed** | Successor branch: the reader with `check_receipt`, else the transport validator. Return (id, `c74742ce…`, retained table path). Add the downgrade guard. Depends on D-U6-1. |
| Fresh set | compatibility.py:217–219 | — | No | Add the id (D-U6-6). |
| Standing | compatibility.py:321–379 | — | **No:** `unsupported` (:327–330) | Successor branch with invocation via `source_block_context`; parity with Rust (§3). |
| Rule binding refusal and summary | compatibility.py:408–423 (new summary fn) | — | n/a | As in Rust. |
| AnalysisRun build | compatibility.py:74–171 (allow-list :94; receipt copy :166–169); build_analysis_run :508–510 | analysis_run.v0.3 | **No, fails closed** | Add the id to both allow-lists. Copy the complete receipt into `analysis_run.retained_precision` (C1:162; D2 §4.9.6). |
| AnalysisRun validate | compatibility.py:513–557 (receipt mirror :523–531) | analysis_run.v0.3 | **No** | Exact equality → `ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH`. A receipt on another identity → `ANALYSIS_RETAINED_PRECISION_DOWNGRADE_FORBIDDEN`. |
| Legacy 0.1.0 wrapper | records.py:73–185 | analysis_run 0.1.0 | **Accepts any source.** It hashes the receipt but does not carry it. | Refuse a source carrying `retained_precision` (D-U6-9). |
| Stress-neutral packager (T6) | P/core/handoff/stress_neutral/package_v0_3.py:32, 43–61, 403–407 | stress_neutral 0.3 | **Refuses:** `SN-SOURCE-METHOD-UNSUPPORTED` | **No edit.** U6b pins the refusal by test (the T6 boundary holds). |

### 1c. Schemas (T3 appends branches after T1's; D2 §4.9.6)

| Schema | File:line | Today | U6 change |
|---|---|---|---|
| results.v0.3 | schemas/results.v0.3.schema.yaml:1146–1148, 1523–1590 (successor branch); RowDisclosure :1939–2060 (reason_code :2022–2033) | Successor branch present and accepted (RV78) | Add the two reason codes (D-U6-2), admitted only in the successor branch. RV78-N2's probes become tests. |
| AnalysisRun | schemas/analysis_run.v0.3.schema.json: SemanticContract :240–330; receipt members :680–702; per-identity oneOf :704–~1050 (preview-physics-1 :893–939) | **No successor branch:** fails closed (RV78-S1) | Add the id and `c74742ce…` (enum plus oneOf), `analysis_run.retained_precision` (`$ref` to retained_precision_mp_v2), and a successor branch that requires it and forbids `source_block_recovery` and `contract_evidence`. Every existing branch forbids it. |
| Stress-neutral | schemas/stress_neutral_export.v0.3.schema.json: ids :226–236, 384–394, 396–420; namespaces :423–428; oneOf :3045 (preview-physics-1 :3430) | **No branch:** fails closed (RV78-S1) | Add the id enums and an eighth branch, the preview-physics-1 branch plus a required closed `retained_precision`. The other branches forbid it. **Schema support does not lift the T6 refusal** (C1:162). |

### 1d. TypeScript desktop (`TS`, T3 for admission and standing; T6 for outputs)

| Carrier | File:line | Accepts the successor today? | U6 change |
|---|---|---|---|
| Dispatch and binding | features/results/numericalResultQuality.ts:25–35, 53–87 | **No:** `"unsupported"` | New route `retained_preview_physics`. The binding is {id, `c74742ce…`}. Downgrade guard as in Rust. |
| Standing (sync) | numericalResultQuality.ts:88–145 | **No:** needs_recompute, finding `SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED` | Successor branch reads **only** the registration (§3). It never uses the `numerical_quality` branch :103–142. |
| Registration (async) | services/previewService.ts:111–132 (direct :49–64; job :209–231) | Registers an unsupported route without a validator | Run `validateRetainedPrecision(source, capture.invocation)` before registering, and record the outcome against the exact bytes. The new module follows T1's pattern (features/results/loadReferenceSourceEvidence.ts:88–122). |
| Semantic table | features/results/resultSemantics.ts:18–31 | throws | Return the retained table (its rows equal preview-physics-1's). |
| Fresh set, reasons, binding, notices, labels | features/results/knownSemanticLimitations.ts:10–17, 45–70, 89–110, 121–127 | n/a | Add the id. `ruleBindingRefusal` and `classificationSummary` over registered classes. `N_RP_ABSOLUTE` and `N_RP_NOT_COVERED` text (D2 §4.9.9). The successor reuses the preview labels. |
| AnalysisRun | services/analysisRunCompatibility.ts:77–146 (copy :141–142), 159–203 (mirror :171–180) | **No:** builder throws | Route the new id, copy the receipt and validate equality, with the same codes as Python. |
| Types | types.ts:504–551, 651–699 | Members absent; values survive at runtime | Add optional `retained_precision` (envelope and `analysis_run`) and row `recovery_method`. |
| Reopen | features/results/HistoricalRunContext.tsx:267–310 | Shows "unsupported" | Revalidate the receipt without an invocation, and compare the AnalysisRun copy. Standing stays needs_recompute: saved records never mint registration (previewService.ts:133). |
| Rule-check gate | services/ruleCheckService.ts:121–127, 315–333 | Refused as not fresh | Require standing eligible, as for the receipt routes (:124). Pass the retained invocation as `sourceBlockInvocation`. Precheck uses the class codes. |
| Standing text | features/results/ResultsPanel.tsx:106 | Generic text showing the **ordinary** `numerical_quality.status` | Successor text from the registered receipt, never from `numerical_quality` (D2 §4.9.2). |
| Output refusal (T6) | features/results/loadReferenceOutputAvailability.ts:16–36 | n/a | Add `RETAINED-PRECISION-OUTPUT-NOT-YET-AVAILABLE` to `loadReferenceOutputRefusal`/`refuseLoadReferenceOutput`. `isLoadReferenceRoute` is unchanged, so the load-reference text is never shown for a successor. |
| T6 surfaces | resultExportAdapter.ts:58, 118, 172; ResultExportPanel.tsx:14–16; StressNeutralExportPanel.tsx:86, 489, 620, 1059; RenderedReportPanel.tsx:70; LoadReferenceOutputGate.tsx:9; reportPackageRequest.ts:25–33 | Fail closed | **No edit.** They refuse through the shared function above, or through the fresh-result report refusal once the id is fresh. Pinned by test. |
| Other consumers | resultsSessionState.ts:53–75; workspaceSession.ts:847–873; ComparisonPanel.tsx:158–160; KnownSemanticNotices.tsx | — | **No edit.** Current requires eligible standing plus `hasNativeMechanicsInvocation` with the mode (:75). Pinned by test. |

## 2. Ownership and collisions

**Tracks in the work graph** (WORK_GRAPH.md tranche table):
- **T3 owns:** the readers, standing, admission sets, binding refusal, AnalysisRun receipt carriage, the derivative receipt-copy list and the schema branches (D2 §4.7, §4.9.6–4.9.7).
- **T6 (PLANNED, not active) owns:** desktop stress-neutral export, result export, the report package and stress-neutral packaging (T6 row; D2 §4.9.6).
- **T1 (COMPLETE) created:** the schema branches and `loadReferenceOutputAvailability.ts`.

| File (U6 writes) | Owner | Collision and coordination |
|---|---|---|
| RS/semantic_contract.rs, RS/derivative.rs | T3 (S-G1) | **No other writer.** U3 grant 1b's base-reader acceptance tests touched only PP. U6e writes RS/retained_precision.rs, a disjoint file. |
| PY/compatibility.py, PY/records.py | T3 | No other writer. U6e writes PY/retained_precision.py, disjoint. |
| schemas/{results,analysis_run}.v0.3 | T3, appending after T1 | T1 is complete. **No live writer.** |
| schemas/stress_neutral_export.v0.3.schema.json | **T6** (packaging contract) | U6 writes inside T6's file under C1:162 and RV78-S1. T6 is inactive. **Reserve it for U6, and post a notice on T6's work-graph row** (D-U6-8). |
| TS/features/results/loadReferenceOutputAvailability.ts | T1-created; the shared T6 output fence (I30 SOURCE_MAP) | Same: reservation plus T6 notice. All T6 panels stay untouched. |
| TS results, services and types files (§1d) | T3 (S-G1 standing honesty, which the work graph's UI rule keeps on the tranche) | No other writer. ResultsPanel.tsx is a one-line standing-text branch. |
| New files (§6) | T3 | Fresh check below. |

**Active TASKs:**
- **I61** (`WT/f2a-facade`) writes PP only, and its U3 grant 2 stays in PP plus runner/headless locks. The one cross-lane item is D-U6-5: an optional PP test that compares the U6 fixture with live output. ROOT routes it into U3 grant 2.
- **I65** (U4 G4) writes records only.
- **RV85 and RV86** write review records only.
- **U6 never writes PP, FK, runner/headless or src-tauri.**

**Fresh collision check** (FIRST_PUBLICATION_PATH §3; `_run_records/collision_check.py`, output `COLLISIONS.json`):
- **The search:** `git grep -F` over P/{core,apps,schemas,fixtures,tests,validation,tools} at three trees: NUM `fc5d92c56c`, `origin/main` `09106477e3`, and the facade head `4b31bbf23a`.
- **The result: all 25 proposed names and all 9 proposed paths are absent** at all three trees.
- **One first choice was dropped:** the route token `preview_physics_retained` matched as a substring of the existing table file name, so it was replaced by `retained_preview_physics`.
- **Reservation status:** none of the names is in RV69's reservation list (R/verification/rv69_c3_selection_02/CHECKS.json), so each needs ROOT's reservation (D-U6-4).
- **The limit:** no conclusion is drawn about untracked work on other branches. Recheck at grant.

## 3. Standing

**Where standing is computed today:**
- **Rust:** RS/semantic_contract.rs:453–557, consumed by src-tauri/src/lib.rs:2822 and runner result_envelope_binding.rs:258.
- **Python:** PY/compatibility.py:321–379.
- **TS:** TS/features/results/numericalResultQuality.ts:88–145, consumed by resultsSessionState.ts:55, ResultExportPanel.tsx:16, StressNeutralExportPanel.tsx:86/500, HistoricalRunContext.tsx:277, LoadReferenceStatesBlock.tsx:66 and ruleCheckService.ts:124.
- **The readers' own eligibility:** RS :4300–4305, PY :1700, TS :1317. They are held by the flags RS :4260, PY :29 and TS :97.

**The U6 rule, identical in the three languages** (D1 §5 item 3; D2 §4.9.4; C1:160):
1. **Dispatch the successor id to the accepted reader.**
   - **Any reader error** gives `unsupported`. TS records the reader's first code as a finding.
   - **The reader passes, but the invocation is absent** (Rust and Python), or nothing is registered (TS): `needs_recompute`. TS records `RETAINED_PRECISION_VALIDATION_REQUIRED`.
2. **Otherwise the standing is `numerically_eligible` only if all of these hold:**
   - the reader's `numerical_eligible`;
   - the requested refs equal `body.cases[].basis_ref` in order;
   - every `not_required` case meets the base ordinary-eligibility predicate (F-7; RS :539–553, PY :371–378, TS :134–136 reused);
   - `MECHANICS_SOLVED`.
   
   If any fails, the standing is `needs_recompute`, and TS records `RETAINED_PRECISION_NOT_NUMERICALLY_ELIGIBLE`.
3. **`numerical_quality` never contributes** to a successor's standing. The generic branches are unreachable for the successor id. A mutant that routes the successor to them must fail a test, using an invalid receipt with the quality rewritten to `checks_passed`.

**Why it stays `needs_recompute` until U7:** all three flags are false, so the reader's `numerical_eligible` is false, and rule 2 cannot pass. U6 does not touch the flags. U7 flips them. After that, eligibility needs the actual invocation:
- in Rust and Python, through the existing `actual_invocation` / `source_block_context`;
- in TS, through a validation registered at direct or job registration against the captured `{request, solver_mode}` and the exact source bytes.

**A later edit to the source** voids the TS registration (fingerprint check, previewService.ts:140–150), and G1 refuses it in every language.

**Binding is separate from standing** (D2 §4.9.9). Absolute and `not_covered` rows refuse binding, even when the envelope is Current.

## 4. The receipt's survival

The governing rule is FIRST_PUBLICATION_PATH §3: "Later receipt operations that alter covered rows or metadata invalidate the earlier certificate."
- **The mechanism:** carriers never recompute `receipt_sha256` or `publication_sha256`, and never repair a document. They copy it whole or refuse it.
- **Invalidation is G1** (`publication_sha256` over the envelope minus the receipt), together with TS's byte-bound registration.

| Operation | Rule | Where | U6 test |
|---|---|---|---|
| Raw publication → AnalysisRun | **Copy** the complete receipt and **validate equality**; the record's `received_result` hash binds the whole raw document | PY compatibility.py:166–169, 523–531; TS analysisRunCompatibility.ts:141–142, 171–180 | Build and validate both modes. A dropped or altered copy fails with its code. Reader revalidation from record plus source. |
| Raw → canonical derivative | **Copy** the receipt and `contract_evidence`. **Disclose** absolute and `not_covered` rows. Validate equality and the class dispositions. | RS derivative.rs:95–118, 176–184, 327–354, 473–489 | U6a E2E (§6). |
| Derivative or raw → desktop result export | **Refuse** (T6) | TS resultExportAdapter.ts:58, 118, 172 via the shared refusal | Output-refusal test. |
| → stress-neutral package | **Refuse** (T6) | PY package_v0_3.py:32; TS StressNeutralExportPanel.tsx:489, 620, 1059 | Python and TS refusal tests. |
| → report package | **Refuse** (the existing fresh refusal) | TS reportPackageRequest.ts:30; RenderedReportPanel.tsx:70 | TS test. |
| Save and reopen | **Preserve the bytes, then revalidate.** Reopen never mints standing. | src-tauri lib.rs:729, 1016; TS HistoricalRunContext.tsx:267–310 | Reopen test. A mutated saved row reads `unsupported`. |
| Native registration | **Validate before registering.** Any later byte change **invalidates** it. | TS previewService.ts:111–152 | Registration tests, with mocked IPC. |
| Rule-check status composition | **Preserve:** it changes only the AnalysisRun's `analysis_status`, never the raw document | TS previewService.ts:280–300; headless lib.rs:832 | The `received_result` hash is unchanged after the rule-check aggregate. |
| Legacy dimension binding | **No-op:** legacy route only | TS previewService.ts:302–306 | Identity test. |
| Legacy 0.1.0 AnalysisRun wrapper | **Refuse** (D-U6-9) | PY records.py:73 | Python test. |
| Relabel to a base identity that keeps the receipt or tokens | **Refuse:** `RETAINED_PRECISION_DOWNGRADE_FORBIDDEN` (F-5) | RS, PY and TS dispatch | Test in each language. |
| Headless runner | **Not carried:** typed envelope plus `into_parts()` (F-6) | runner lib.rs:691, 774 | None in U6. It is a wider-F2a precondition of Headless admission. |
| Any edit to a covered row, diagnostic, `numerical_quality`, `contract_evidence`, header or status | **Invalidate:** G1 gives `unsupported`, and TS registration is void | readers G1 | One-row mutation in each language. |

## 5. The reader-round items U7 needs

All are D36-tracked unless marked.

| Item | What | Owner reader | Needed before U7? | Cost (author + review) | Bundle |
|---|---|---|---|---|---|
| **D-U6-1** (I66; new) | The Python public entry runs every gate. `_IMPLEMENTATION_COMPLETE` gates eligibility only, as in Rust and TS. | Python (I62 lineage) | **Yes, before U6b** | 0.5–1 h + 0.5 h (RV79 scoped) | First, in U6e |
| **F5** (U1) | Readers enforce the exact D6a/A2 ordinary list: the diagnostics whose `affected_refs` name the case, once each, in envelope order, excluding `RETAINED_PRECISION_*` and the T1-omitted disclosure. They also pin the row token against M20. Checkpoint A's D6a (RR:8117) ruled that no reader requires the refs to name the case, and made the exact list a producer obligation. A2 (RR:8821) fixed that list, and the readers still apply the relaxed rule (PY:907; RS:4130; TS:1255). **Enforcing A2 amends checkpoint A's D6a (D-U6-7). It can change eligibility, so it gates U7.** | All three, plus the corpus (I62, I63, I64 lineages) | **Yes** | 4–6 h + 1.5–2 h (RV78 parity; RV79–RV81 scoped) | U6e, in parallel with U6a–U6d (disjoint files) |
| **RV79-N1** | The D37 test writes its expected table literally, independent of `rp`, over the full 25-record class-2 space, killing M35 and M37 | Python test | **Yes** (I61 plan U7; RR:8763) | 1–1.5 h + 0.5 h | With F5's Python part |
| **RV80-N2** | Pin `integral_receipt`'s receipt-only scope (M50) | Rust test | No (optional) | 0.5 h + 0.25 h | With F5's Rust part |
| **RV78-N2** | RV78's 26 YAML probes become schema tests | Python schema test | No | ~1 h + in U6c review | **Into U6c** |
| **RV78-N1** | The table binds the projection and work policies, the 20B/60B limits, the token and the canonicalization | Table fixture; all readers' pinned hash; **PP's bound table hash and pinned successor bytes; U5 evidence** | No (NOTE) | 3–5 h with the re-pin cascade | **Defer** to the first post-milestone table revision (D-U6-7) |
| **F-7** (I66; new) | Reader eligibility applies D2 §4.9.4's `not_required` conjunct | All three | No: unreachable in D1. U6's carriers apply it. | ~2 h | Wider F2a |
| D38 pin, `capture` (a), RV82 N2/N3/N8/N9 | Multi-case and refusal forms | — | No | — | Wider F2a (RR "U1 grant 2: I61's stop resolved") |
| Deferred-base survivors (PY M05, M12, M13, M17, M24, M25, M28, and others) | Need producer-solved bases | All | No | — | U8 |
| RV80-N1, RV81 R35 | — | — | Done | — | — |

## 6. Units, write fences, order, costs and review

Every unit gets a fresh independent review of its complete frozen diff, with mutants on each new branch, and ROOT verification before fan-in. TASKs make no Git writes.

**U6.0. ROOT's rulings** on D-U6-1 to D-U6-9, the reservation, and the T6 notice. About 0.5–1 h of ROOT time.

**U6a. The end-to-end slice: Rust carriers** (first unit; about 5–7 h; review 2–3 h by a fresh Rust reviewer)
- **Checkpoint A (about 1–1.5 h):**
  - add `fixtures/results/retained_precision_milestone_successor_{sparse_interactive,dense_scrutiny}.json`, byte-identical to PP's pinned files (sha256 `ac6986b0…` and `6cd1d249…`; receipts `efc1a39b…` and `3e26499f…`);
  - source them either from ROOT's rerun of `u3_permitted_path_publishes_the_pinned_successor` with `I61_U3_OUT`, or from I61's hash-matched U5 inputs;
  - add `fixtures/results/retained_precision_carrier_cases.json`: the shared standing-parity scenarios (fixture id, invocation yes/no, requested refs, one rehash-free edit, expected standing and code), consumed by U6a, U6b and U6d.
  
  ROOT verifies the sha256 values and releases the fixtures to U6b and U6d.
- **Fence:** RS/semantic_contract.rs, RS/derivative.rs, the three fixtures above, and the new `core/reporting/result_export/tests/retained_precision_carriers.rs`. RS/retained_precision.rs is **read-only**.
- **The deliverable:** the §1a rows, namely dispatch, downgrade guard, fresh set, standing, binding refusal, summary, and derivative copy and disclosures.
- **The end-to-end test, in both modes:**
  1. the fixture source passes `for_source` and `for_source_metadata`;
  2. standing is `needs_recompute`, with and without the fixture invocation;
  3. `derive_document` then `validate_document` passes (the pattern of result_export tests/preview_physics_contract.rs:58);
  4. the derivative's `retained_precision` is byte-equal to the source's, and passes `validate_transport_metadata`, never eligible;
  5. every row's disposition follows the reader's validated class: the 69 `absolute_verified` rows are disclosed with their bound; relative and input-derived rows keep their table disposition;
  6. the negatives each fail with their code: a dropped or altered derivative receipt; a mutated covered row (gives `unsupported`); a receipt on a preview-physics-1 source; and the successor routed through the generic standing.
  
  The R-2 noticed ordinary envelope keeps its base behaviour.
- **Regression:** the result_export suite and runner/headless are unchanged. ROOT reruns PP's U1 and U3 pin tests against the new result_export, because PP's precommit reader calls `for_source` on its projection.

**U6b. Python carriers** (about 4–5 h; review 1.5–2 h)
- **Fence:** PY/compatibility.py, PY/records.py, and the new `tests/test_retained_precision_carriers.py`.
- **Prerequisites:** D-U6-1 landed (U6e) and checkpoint A.
- **The deliverable:** §1b.
- **Tests:**
  - AnalysisRun build and validate in both modes, including drop, alter and downgrade cases;
  - the parity scenarios;
  - binding codes;
  - the stress-neutral refusal pin;
  - the records.py refusal;
  - the R-2 noticed envelope.
- **Runtime:** ROOT-qualified checked-JSON and units CLIs (I52 RETURN:190–194).

**U6c. Schemas** (about 2–3 h; review 1–1.5 h, RV78 lineage)
- **Fence:** schemas/analysis_run.v0.3.schema.json, schemas/stress_neutral_export.v0.3.schema.json, the results.v0.3 RowDisclosure enum, and tests/test_retained_precision_schema.py.
- **The deliverable:** §1c, plus RV78-N2's probes.
- **Order:** it runs in parallel once D-U6-2 is ruled.

**U6d. TypeScript carriers and standing** (about 8–11 h; review 3 h, RV81 lineage or fresh)
- **Fence:** TS/features/results/{numericalResultQuality.ts, resultSemantics.ts, knownSemanticLimitations.ts, loadReferenceOutputAvailability.ts, ResultsPanel.tsx, HistoricalRunContext.tsx}; the new `TS/features/results/retainedPrecisionStanding.ts`; TS/{types.ts, services/analysisRunCompatibility.ts, services/previewService.ts, services/ruleCheckService.ts}; and the new tests `features/results/retainedPrecisionIntegration.test.tsx`, `services/retainedPrecisionAnalysisRun.test.ts` and `features/results/retainedPrecisionOutputRefusal.test.tsx`.
- **The deliverable:** §1d. Everything else in §1d is read-only and pinned by test.
- **Tests:**
  - registration through mocked direct and job IPC with the fixture and its invocation;
  - synchronous standing;
  - a mutation that voids the registration;
  - AnalysisRun copy and validate;
  - reopen;
  - every T6 surface refusing;
  - the rule-check gate;
  - the parity scenarios.
- **Runtime:** a ROOT-bound node_modules, WASM assets and cwd (I52 RETURN:196–202).
- **Not possible in U6:** a native witness. Tauri never delivers a successor (F-1). That witness belongs to native activation.

**U6e. The reader round** (about 6–9 h; review 2–3 h)
- **Fences:**
  - PY/retained_precision.py and tests/test_retained_precision_contract.py;
  - RS/retained_precision.rs and `core/reporting/result_export/tests/retained_precision_contract.rs`;
  - TS/features/results/retainedPrecision.ts and retainedPrecision.test.ts;
  - fixtures/results/retained_precision_cases.json (snapshot 07g).
- **Contents, in order:** D-U6-1, then F5, RV79-N1 and RV80-N2.
- **Disjoint from U6a–U6d.**
- **Not touched:** the flags. They stay false until U7.

**U6f. Integration and the complete review** (about 3–4 h of review and 1 h of ROOT)
- **The review:** a fresh reviewer of the whole U6 plus U6e diff, with the three-language parity table on `retained_precision_carrier_cases.json`.
- **ROOT's suite runs:** result_export, analysis_runs, desktop Vitest, runner/headless, and PP's pins.
- **The R-1 carrier-interface review items:**
  - **C-1:** `envelope()` and `into_parts()` stay ordinary-only and silently omit a successor. Document "ordinary base only". The headless switch to `into_publication()` is a wider-F2a precondition (F-6).
  - **C-2:** `Successor(Value)` is precommit-validated, and carriers revalidate it, never trusting it.
  - **C-3:** the only successor-preserving consumer path is `into_publication()`, and it has no product caller until native activation (F-1).
- **After U4 G6 grants a permit:** U7 or U9 replaces the fixture input with a live `into_publication()` output and reruns the U6a and U6b tests.

**The order:**
1. U6.0.
2. U6a checkpoint A, together with U6e's D-U6-1.
3. In parallel: the rest of U6a, U6b, U6c, U6d and U6e's F5 round.
4. U6f.
5. U7, which also needs U3 grant 2, the U4 G6 permit, the live milestone in both modes with U5 rerun (RR "RV85 …"), and a fresh review of the switch.

**The totals:**

| | Hours |
|---|---|
| Authoring, carriers (U6a–U6d) | 19–26 |
| Reader round (U6e) | 6–9 |
| Reviews | 12.5–16.5 |
| Elapsed, longest chain (checkpoint A → U6d → its review → U6f) | about 15–20 |

## 7. Decisions needed

All are for ROOT. **None is owner-reserved.** Each realizes an accepted design (D1, D2, C1 or the RR rulings) without a new numerical meaning, criterion or availability.

| # | Decision | Proposed answer | Basis |
|---|---|---|---|
| **D-U6-1** | Python reader public-entry semantics (F-3) | Run `_validate_draft`'s gates in `validate_retained_precision`, and let `_IMPLEMENTATION_COMPLETE` gate eligibility only, as in Rust and TS. This is a reader change, scoped and reviewed by RV79. | D2 §4.9.4 (`unsupported` only on a G failure) and §4.7 parity; RR:8761 (only eligibility and activation are held); RS:4300, TS:1317 |
| **D-U6-2** | How the derivative discloses classes (RV78-N3; F-4) | **(A) Withhold:** `absolute_verified` and `not_covered` rows take disposition `disclosed`, with reason codes `retained_precision_absolute_verified` and `retained_precision_not_covered`. The message names the class and the bound, and the value stays as `source_value`. The machine-readable bounds are in the copied receipt. The alternative, (B), keeps the values and adds a closed side list. S-I (interval binding, later) revisits the absolute rows. | D2 §4.9.9 ("withheld… never binds… never appears unlabelled"; "one `row_disclosures` entry per such row"); derivative.rs:443–451 cardinality; C1:162 |
| **D-U6-3** | The TS scope while native W1 is held (F-1) | **Full §1d in U6**, tested with mocked IPC. The native witness is deferred to activation and stated as a qualification limit. The alternative (deferring registration and UI) would break D2 §4.7's standing parity and leave U7's TS flag inert. | C1:162; D2 §4.7 (S-1, S-6 parity); RR "U4 plan" (Tauri ordinary only) |
| **D-U6-4** | Reserve the 25 names and 9 paths (§2) | Reserve them; recheck at each grant. | FIRST_PUBLICATION_PATH §3; C1 §3, §6; COLLISIONS.json |
| **D-U6-5** | The fixture's provenance | Commit byte-identical copies of PP's pinned files, checked by sha256 in every language test. Route to I61's U3 grant 2 a one-assertion PP test that `include_str!`s the fixtures and compares them with the live serializer output. | PP/retained_facade_tests.rs:13–16, 59–80; I30 SOURCE_MAP ("freeze… names and source/hash manifest") |
| **D-U6-6** | The successor joins the Current-admission sets in U6 | Yes. Standing, not freshness, gates every reliance. TS's rule-check gate adds the standing check (§1d), and the report refusal holds. | D2 §4.7 table row S-1 ("S-G, atomic"); D2 §4.9.6 |
| **D-U6-7** | Bundling the reader round, and F5's rule | F5, RV79-N1, RV80-N2 and D-U6-1 now (U6e). **F5 amends checkpoint A's D6a:** readers enforce A2's exact list (the diagnostics whose `affected_refs` name the case, once each, in envelope order, excluding `RETAINED_PRECISION_*` and the T1-omitted disclosure). RV78-N2 into U6c. **Defer RV78-N1** (its table re-pin cascades into PP's bound table hash, the pinned successor bytes, U5 and the fixtures). F-7 reader-side goes to wider F2a. | RR:8117 (checkpoint A, D6a); RR:8821 (A2); RR:9015 (F5); RR D36 (gating = could change eligibility); RR "The F2a readers accepted" routing |
| **D-U6-8** | T6 coordination | Reserve the stress-neutral schema and `loadReferenceOutputAvailability.ts` for U6, and post a notice on T6's work-graph row: successor outputs are refused on the desktop until T6; the schema branch admits transport only; the derivative discloses classes. No T6 panel is edited. | D2 §4.9.6; C1:162; RV78-S1; T6 row (T1 §12 precedent) |
| **D-U6-9** | The legacy 0.1.0 AnalysisRun wrapper | Refuse sources carrying `retained_precision`, one guard, so a receipt is never silently dropped. | C1:162 (AnalysisRun copies the complete receipt); §4 |

## 8. Limits of this plan

- **What the estimates rest on:** the costs are judgements from the cited code and the reader rounds' history, not measurements.
- **What was not run:** no code, test or schema validator ran. The inventory is from source reading at the stated lines.
- **The collision check's scope:** it covers committed trees only.
- **What the live-facade replacement depends on:** the replacement in U6f waits on U4 G6 and U3 grant 2.
- **What U6 cannot provide:** a native desktop witness of a successor (F-1).
