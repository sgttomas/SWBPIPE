# I61: U7, the eligibility switch-on (scoping plan)

**Read-only plan.** No code changed, nothing was built, and no Git writes were made.
- **Basis:** the memory branch head `7f07a2f7b4`, which carries U6 and NUM through `f8ce1eb32b`, grant 2 and D-U6-5, plus I65's T17_V4 line. NUM is at `071eec5c04`. Read with ROOT_RULINGS_V1 through "G7 Pass B on the final basis…".
- **Placeholders:**
  - P = `projects/chirality-piping`;
  - PY = P/core/analysis_runs;
  - RS = P/core/reporting/result_export/src;
  - TS = P/apps/desktop/src;
  - PP = P/core/product_physics/src.
- **Line numbers** are at `7f07a2f7b4`.

## 0. Summary

**What U7 does.** It flips one compile-time constant per reader:
- PY `_IMPLEMENTATION_COMPLETE`;
- RS `IMPLEMENTATION_COMPLETE`;
- TS `SUMMARY_COVERAGE_COMPLETE`.

**What follows from the flip:**
- An accepted-reader validation of a successor, **bound to the actual invocation**, of a `MECHANICS_SOLVED` statement whose cases are all `selected` or `not_required`, reports `numerical_eligible: true`.
- The carriers' standing token then becomes `numerically_eligible` where D2 §4.9.4's other conjuncts also hold.
- Nothing in the producer changes. No published byte changes.

**What U7 does not do.** It does not add a product caller. Successors still reach only library consumers (F-1), the T6 output panels still refuse them, and M stays a W1 admission threshold with no machine claim.

**What it needs first:**
- **Four open preconditions:**
  - RV91 N-2: TS standing bound to the live registration and model;
  - RV91 N-5: explicit panel gates and the T6 notice;
  - RV92 N-6: PP's R-1 docs and exposure;
  - RV92 N-8's live-output rerun and token comparison.
- **The memory branch's merge into NUM,** after RV93 and RV89.
- **A G7 Pass B on U7's basis.** The RS flag sits in the precommit reader, which is on the D1 call graph.

**The estimate:** about 11–16 agent-hours of authoring and 4–7 hours of review, or about 6–9 hours elapsed with the pre-flip work run in parallel (§5).

## 1. What the switch changes, by path:line

### 1.1 The flags and the readers' own output

| Reader | The flag | What it gates | Also update (stale text) |
|---|---|---|---|
| Python | `PY/retained_precision.py:30` `_IMPLEMENTATION_COMPLETE = False` | `:1709–1710`: `eligible = flag and invocation is not None and status.mechanics == "MECHANICS_SOLVED" and every case selected or not_required`; the reader's own `standing` is `"eligible"` or `"needs_recompute"` | The comment at `:27–29` ("stays false until U7"); `_validate_draft`'s docstring at `:1598` ("eligibility remains disabled with the API"); `validate_retained_precision`'s docstring at `:1591–1593` |
| Rust | `RS/retained_precision.rs:4269` `const IMPLEMENTATION_COMPLETE: bool = false;` | `:4309–4314`: the same conjunction into `Validation.numerical_eligible`. There is no `standing` field. `validate_transport_metadata` (`:4339`) stays `false` by construction | The comment at `:4267–4268` ("the reader is unaccepted until the snapshot-07 repair wave…") |
| TypeScript | `TS/features/results/retainedPrecision.ts:97` `const SUMMARY_COVERAGE_COMPLETE = false;` | `:1327–1328`: the same conjunction. The reader's `standing` is `'eligible'` or `'needs_recompute'`. The transport validation (`:1332ff`) stays not eligible | The comment at `:95–96` ("snapshot 04 only… held until snapshot 05") |

**All three readers already run every gate** (D-U6-1). The flag only adds a borrowed-read conjunct: Rust `list()` (`RS:224`) returns a slice, so evaluating it allocates nothing.

### 1.2 The standing a live successor then carries

The carrier token comes from D2 §4.9.4, unchanged since U6. Each language's implementation:
- PY `compatibility.py:318–328` (`_retained_standing_from`), reached from `numerical_use_standing` at `:482–494`;
- RS `semantic_contract.rs:584–603` (`retained_standing_from`), reached from `retained_standing` at `:647` and `numerical_use_standing_with_context` at `:731`;
- TS `retainedPrecisionStanding.ts:134–155`.

| The live milestone successor (`MECHANICS_SOLVED`; its one case `selected`) | Before U7 | After U7 |
|---|---|---|
| With the actual invocation; requested refs equal to the receipt's case order (PY and RS); a registered validation for the model's cases (TS) | `needs_recompute` | **`numerically_eligible`** |
| With no invocation (`numerical_use_standing` without context; RS `for_source`-only callers) | `needs_recompute` | `needs_recompute` (`invocation_bound` is false) |
| With another invocation (mode, request or model) | Refused at G8: `unsupported` | Unchanged |
| A successor with an `unavailable` case | `needs_recompute` | `needs_recompute` (that case fails the conjunction) |
| A statement the reader refuses | `unsupported` | Unchanged |

**What follows from an eligible token:**
- **TS:** `numericalResultQuality.ts:154–158` maps eligible to status `integrity_checked`, the same mapping as every other TS route (`:199`).
- **The classification summary** stops withholding relative-verified and input-derived rows. It still withholds absolute-verified and not-covered rows, because S-I has not landed: PY `compatibility.py:348–370`, RS `semantic_contract.rs:664–700`, TS `retainedPrecisionStanding.ts:175–187`.
- **Row binding stays class-based** (`absolute_verified` and `not_covered` are refused): RS `semantic_contract.rs:537/553`, PY `compatibility.py:336/576`. U7 does not change it.

### 1.3 Consumers that newly rely on it

| Consumer | Where | What becomes reachable | Product reachability today |
|---|---|---|---|
| **Rule-check binding** (TS) | `TS/services/ruleCheckService.ts:131–135` (`numericalResultStanding(...).eligible`, then `retainedPrecisionInvocation`) | A successor may be rule-checked, with relative and input-derived rows bindable | Only with a registered native capture; the native path delivers no successor (F-1) |
| **Rule-check binding** (runner) | `P/core/runner/headless/src/result_envelope_binding.rs:258` (requires `numerically_eligible` with the actual invocation) | An attested headless successor could bind | Headless is refused at D1.0 (D-2), so no headless successor exists |
| **Classification summary and notices** | TS `knownSemanticLimitations.ts:84–168`, `ResultsPanel.tsx:107` (standing text); PY and RS `classification_summary` | The withheld counts change (§1.2); the standing text reads eligible | Library or TS-mock only |
| **The derivative** (`RS/derivative.rs:107` `derive_document`) | Class-based disclosures (`retained_precision_absolute_verified` and `_not_covered`, D-U6-2) and the receipt copy (`:174`) | No flag dependency found. **To be confirmed by the inventory (§3, slice A)** | Library only |
| **The AnalysisRun carriers** (PY `compatibility.py:86/172`; TS `analysisRunCompatibility.ts`) | The receipt copy, compared canonically (RV92 S-1 is fixed) | The standing recorded with a run follows §1.2 | Library or TS-mock only |
| **The T6 output panels** (TS `ResultExportPanel.tsx:12–18` `liveResultBinding`; `StressNeutralExportPanel.tsx:83–88` `liveStressBinding`) | Each gates on `hasCurrentSourceContract`, `!isLoadReferenceRoute` and `numericalResultStanding(...).eligible` | **After U7 the eligibility conjunct would pass for a successor.** Only the builders' shared-refusal throw would remain (RV91 N-5), so an explicit gate is needed | No successor reaches the desktop today |
| **The stress-neutral packager** (PY `P/core/handoff/stress_neutral/package_v0_3.py:32`) | `SUPPORTED_METHODS` excludes `preview-physics-retained-1` | Nothing: it keeps refusing (T6, D-U6-8) | — |
| **The legacy 0.1.0 AnalysisRun wrapper** | D-U6-9 refusal | Nothing | — |

### 1.4 What stays closed

- **Public activation.** No product caller is added:
  - the desktop calls only the ordinary wrapper;
  - the Direct facade has no product caller;
  - Headless is refused (D-2).
  
  Native W1 activation, the desktop native successor witness, N-7's memoization (required "before native activation") and T6's successor outputs are later work. **RR:9250 says "Public activation stays with U7".** §6 decision D-U7-1 asks ROOT to restate this as "reader eligibility is U7; public activation is separately ruled".
- **M.** M stays D-7's W1 admission threshold on requested and moving heap bytes in the one registered dev/test build. It is not an RSS, stack, concurrency or machine figure. **Any supported-machine statement of M stays owner-held** (decision 9; RR:8828, :8887). U7 says nothing about machines.
- **The registered profile.** It is unchanged in identity and scope: one dev/test build. The release identity stays unregistered, and hosted Linux CI stays Stale.
- **Producer-origin authentication.** The readers bind the supplied statement and invocation; "they do not establish producer origin" (`RS:4270–4271`). After U7, a byte-valid successor statement presented with its invocation reads eligible in any build of a reader, whether or not a registered producer made it. This is the design (C-2: carriers revalidate, and no carrier trusts a token). **It is stated here so that ROOT rules on it explicitly** (D-U7-6).

## 2. The recorded U7 preconditions

| # | Precondition | Source | Status | Evidence, or the work needed |
|---|---|---|---|---|
| 1 | U1–U6 accepted | step-4 PLAN §2 U7; RR:8814 | **Partly open** | U1/U2: accepted on review (RR "U1 grant 1 accepted…"). U3 grants 1–1d: RV85 PASS. **U3 grant 2 and the D-U6-5 follow-on: RV93 reviewing.** **U4 G7 Pass B: RV89 confirming.** U5: RV86 PASS with limits; rerun on live bytes byte-identical (grant 2 and follow-on). U6: RV92 PASS and confirmations, merged into NUM `f172f86abe`. **The memory branch merges into NUM after RV93 and RV89** |
| 2 | The milestone published through the facade in both modes, with reference agreement | step-4 PLAN; RR:9420 | **Closed** | `R/I61/u3_grant2_01/`, `u3_grant2_02/`. The Direct entry publishes `ac6986b0…` / `6cd1d249…` under the 07h reader. U5's report and log are byte-identical, and RV86's limit 4 is discharged |
| 3 | RV79-N1: the independent D37 table | RR:9751 | **Closed** | U6e 07g (`5e1e2625ac`) plus the 07h N4 premise (`cc4dd61d67`): the corpus `d37` drives every reader's D37 test. RV90 derived the table independently and it matches. Merged with U6 |
| 4 | RV86 S-1: ROOT has `u5_reference_01/ADDENDUM_01.md` | RR:9430 | **Closed** | Committed in NUM `6e796235c8` ("U5 addendum"). S-2 (the extract pin) also landed in grant 2 (NUM `0198176dc9`) |
| 5 | RV91 N-2 = RV88 U6d S-1: TS successor standing tied to the live native registration and the model | RR:10027, :10094 | **Open** | **Work (I67):** `retainedPrecisionStanding` (`TS/…/retainedPrecisionStanding.ts:150`) reads only the byte registration (`:96–100`) and the model's case ids. It must also require the live native capture that `previewService.ts:138–155` (`hasNativeMechanicsInvocation`) holds for these bytes and this model: an unchanged capture, an unchanged source, and a caller model equal to the given model. Otherwise it gives `needs_recompute`. Tests: another model with the same case ids; a cancel or invalidation; a mutated invocation, each giving `needs_recompute`. Mocked IPC, as in U6d. **A declared difference follows** (D-U7-4) |
| 6 | RV91 N-5: the result-export and stress-neutral panels refuse a successor by an explicit gate, plus the T6 notice | RR:10028 | **Open** | **Work (I67):** gate `liveResultBinding` and `liveStressBinding` explicitly on the exported `loadReferenceOutputRefusal(result) === null` (`TS/…/loadReferenceOutputAvailability.ts:41–43`, which covers both routes; `isRetainedPrecisionRoute` at `:36` is module-private), in place of the load/reference-only `isLoadReferenceRoute`. The panels then show `RETAINED_PRECISION_OUTPUT_REFUSAL` (`:24–26`), not the generic empty text. Tests with the flag forced on through the seams. **ROOT** posts the U7 line on T6's work-graph row |
| 7 | RV92 N-8: carrier tests and the survival chain rerun on **live** output, in all three languages; standing compared by **token**, not by TS's status string | RR:10374; RR:9927 | **Open** (by construction U7 work) | **Work (I61):** take the successor bytes from the registered Direct entry (`I61_U3G2_OUT`; they equal the D-U6-5 fixtures) and the actual invocation. Rerun PY `tests/test_retained_precision_carriers.py`, RS `result_export/tests/retained_precision_carriers.rs`, TS `retainedPrecisionIntegration.test.tsx` and `retainedPrecisionAnalysisRun.test.ts`, and RV92's survival chain (18 cases). Compare the carrier token across the three languages. Pin TS's `integrity_checked` mapping separately (`numericalResultQuality.ts:158`). The other N-8 items (S-1, N-1, N-2 to N-5, and N-9 by scope sentence) are **closed** (I66 `6383e8e70e`, I67 `b10ee5cf08`, RV92 u6f_02, RR:10501) |
| 8 | RV92 N-6: R-1's docs and exposure. **C-1:** `into_parts()` drops a successor. **C-3:** `successor()` is a second public path | RR:10370 (routed to U3 grant 2) | **Open** | **Not done in grant 2** (it was not in grant 2's brief, and I did not carry it). `PP/lib.rs:2254` has no doc comment; `:2235` is `pub fn successor`. **Work (I61), line-neutral:** an inline `#[doc = "…ordinary base and report only; a successor is dropped (R-1)."]` on line 2254; `pub(crate) fn successor` with its doc reworded on lines 2233–2235. Its only callers are in-crate tests (`retained_facade_tests.rs`). Then **take the G7 Pass B rerun anyway** (D-U7-3) |
| 9 | RV78's scoped review of the semantic-table fixture and the YAML successor branch | RR:8795 (A4) | **Closed** | RV78 PASS (0 BLOCKING); accepted with the readers |
| 10 | The capture (a) Run representation (decision 4) | RR:8823 (D38) | **Closed for U7** | Ruled as D38. The reader relaxation and its pin moved to wider F2a under D36 (RR:9117, :9174). It does not affect the one-case milestone |
| 11 | A fresh review of the switch | step-4 PLAN | **U7's own** | RV94 (§3) |
| 12 | U6f, the complete-diff review | RR:9469 | **Closed** | RV92 PASS, plus u6f_02 confirmations |
| 13 | **Found here:** a G7 Pass B on U7's basis | QUALIFICATION §11; RR:10459 ("any code change on the D1 call graph re-runs TEXT…") | **Open** (U7 work) | The RS flag is in the precommit reader that PP calls (`PP/lib.rs`, `retained_w1`), so it is on the D1 call graph. So is N-6's `lib.rs` edit, by file. **Work:** ROOT or I65 run the hardened Pass B on the U7 head; RV89 confirms. Expected: the registered entry holds, with no new text site and no allocation (§4) |
| 14 | **Found here:** eligibility expectations on the shared corpus | — | **Open** (decision D-U7-2) | Today each reader pins `numerical_eligible == False` on passing entries. Examples: PY `test_retained_precision_contract.py:149–155, :200, :211, :245, :280`; RS `retained_precision_contract.rs` (6 sites); TS `retainedPrecision.test.ts` (6) and `retainedPrecisionIntegration.test.tsx` (36). After the flip, the expected value is the conjunction in §1.1 |

**Not U7 preconditions** (ruled or noted):
- N-7's memoization (before native activation);
- RV78-N1 and F-7 (wider F2a);
- RV85 U1, the permit's binding to its invocation (RV93 is assessing it; it becomes a U7 item only if RV93 or ROOT says so);
- the D38 pin (wider F2a).

## 3. Slices, owners, order and review

| Slice | Owner | Worktree | Content | Can start |
|---|---|---|---|---|
| **A. Inventory and oracle** (records only) | I61 | NUM records; read-only on NUM after the memory merge | (1) every test or pin asserting held eligibility, in all three languages, with the line it will carry after the flip; (2) every consumer in §1.3, confirming or refuting the derivative's and AnalysisRun's flag independence; (3) a stdlib-only oracle script that computes the expected `numerical_eligible` and carrier token for every corpus base, must-pass entry, U6 case and live milestone input, from the conjunction alone; (4) the expected delta table | **Now** (read-only) |
| **P. PP's R-1 docs** (N-6) | I61 | WT/f2a-memory, or NUM after the merge | The line-neutral `lib.rs` edits (§2 row 8), plus a test that `into_parts()` drops the successor on the registered milestone | After RV93's verdict on grant 2, so as not to move its review basis |
| **T. TS pre-flip** (N-2, N-5, the token pin) | I67 | WT/f2a-carriers-ts, rebased on NUM | §2 rows 5–6, plus the RV92 N-8 item-5 pin (the token, and the `integrity_checked` mapping), each tested through the seams with eligibility forced, before the flip | **Now** (it depends only on U6 in NUM) |
| **F. The flip, atomic** | I66 (PY and RS readers and carriers), then I67 (TS), on one branch | A new `codex/piping-f2a-u7-…` from NUM after the memory merge and slices P and T | The three constants and their stale text (§1.1). Each pin from slice A moves from `False` to the oracle's value. **If D-U7-2 is adopted:** snapshot 07i adds an `eligible` expectation to the bases and must-pass entries, consumed by all three readers. Committed as one change | After the merge, P and T |
| **L. Live reruns** | I61 | The F branch (read-only) plus scratch | §2 row 7; the PP 324-output sweep, registered and Stale (it must be byte-identical to `u3_grant2_02`); the U5 rerun; the three-language token comparison on the live milestone | After F |
| **Q. Pass B on the U7 head** | ROOT, with I65's script | The F branch | §2 row 13 | After F (parallel with L) |
| **R. Review** | **RV94** (fresh) | Its own copy | The whole U7 diff: flags, pins, oracle, slices P/T/F and L's evidence. RV89 confirms Pass B. RV91 and RV92 confirm their own findings (N-2, N-5; N-6, N-8) briefly, or RV94 covers them, as ROOT prefers | After L and Q |

**Ordering.** A and T start now. P starts after RV93's grant-2 verdict. F waits for the memory merge plus P and T. L and Q run in parallel after F. R follows, and U9 after acceptance. **I66 and I67 take turns on the F branch,** as in the post-U6f round (RR:10383).

## 4. Risks and controls

**These must stay byte-identical:**
1. **Every published byte,** in every build: PP's 324-output sweep, registered and Stale, against `u3_grant2_02`'s TSVs. The flags are reader-side; PP's precommit uses only `validate`'s `Ok`/`Err`, never `numerical_eligible`.
2. **The successor bytes:** the D-U6-5 test; `ac6986b0…` / `6cd1d249…`.
3. **Every gate outcome:** the 07h corpus (277 + 23 + 15 entries) and U6's case file (format v3, five declared entries). The same gate, code and classifications in all three languages.
4. **Every non-retained identity's standing:** precision-1, physics, preview-physics-1, source-blocks, physics-source, load-reference and load-reference-source. The flag is read only in the retained branch (PY `compatibility.py:488`; RS `semantic_contract.rs:736`; TS `numericalResultQuality.ts:154`).
5. **Every transport and metadata route** (it stays not eligible), the stress-neutral packager's refusal and the D-U6-9 refusal.

**Controls that only the eligible path changed:**
- **The oracle diff.** Before and after the flip, each language's full outcome dump changes only in `numerical_eligible`, the standing token and the summary's withheld counts. It changes on exactly the oracle's set (invocation-bound, `MECHANICS_SOLVED`, every case selected or not_required, D2 §4.9.4's other conjuncts), and stays the same everywhere else. RV92's 546-input three-language harness is reused.
- **No assertion is removed.** Each changed pin moves from a constant `False` to the oracle's value. RV94 checks the diff line by line.
- **Mutants:**
  - the flag false again;
  - each conjunct removed (the invocation, `MECHANICS_SOLVED`, the case status, requested refs, the not_required conjunct);
  - TS N-2's live-capture binding removed;
  - N-5's panel gate removed;
  - N-6's `pub(crate)` reverted.
  
  None may be killed only by a compile error.

**The Stale build.** PP publishes no successor there (it has no permit). The sweep stays at base `b54caba7ab`'s bytes. The readers' eligibility does not depend on the build: a Stale-built reader judges a supplied successor exactly as the registered one does. CI on hosted Linux (Stale) still exercises eligibility through the committed D-U6-5 fixtures, which are byte-identical to the live output.

**The registered profile.**
- **The RS flag is a re-qualification trigger by rule** (QUALIFICATION §11; RR:10459). It is code reached by PP's precommit on D1, so Pass B on the U7 head is required and RV89 confirms it.
- **Expected outcome:** the identity, 14 reviewed inputs and 4 reader layouts are unchanged (`Validation` keeps its fields). There is no new text site. The added conjunct allocates nothing (`list()` borrows). The maxima stay 0.8881 / 0.8929 M.
- **If Pass B stops,** U7 waits for I65's delta: the same pattern as G7.
- **N-6's `lib.rs` edit** is in PP. Line-neutral, it keeps every TEXT line key; Pass B still runs, because its rule is by file.
- **TS and PY changes are not on D1,** and trigger nothing.

**Other risks:**
- **R1. Parity on the newly reachable path.** Post-U7 summaries and standings were tested only through seams (RR:9927). Live reruns may show differences such as TS's empty summary for an unregistered statement (I67-F2). Slice L may therefore add declared entries, which ROOT rules.
- **R2. TS N-2 makes TS stricter.** Without a live native capture, TS reads `needs_recompute` where PY and RS, given the invocation, read eligible. This is a declared difference (D-U7-4).
- **R3. Producer origin** (§1.4, D-U7-6).
- **R4. Schedule.** F waits for the memory merge, which waits for RV93 and RV89.

## 5. Estimate

| Slice | Authoring | Review |
|---|---|---|
| A (inventory and oracle) | 1.5–2 h | inside RV94 |
| P (N-6) | 0.5–1 h | inside RV94 |
| T (N-2, N-5, token pin) | 3–4.5 h | inside RV94, or RV91 confirms, 0.5–1 h |
| F (the flip; PY and RS, then TS; plus 07i if D-U7-2 is adopted, +1–1.5 h) | 3–4.5 h | — |
| L (live reruns, sweeps, U5, tokens) | 2–3 h, partly machine time | — |
| Q (Pass B) | 0.5–1 h machine time (ROOT or I65) | RV89 confirms, 0.5–1 h |
| R (RV94, the switch as a whole) | — | 3–4 h |
| **Total** | **about 11–16 agent-hours** | **about 4–7 h** |

**Elapsed:** about 6–9 hours from the memory merge, with A and T run in parallel before it. That is roughly 2 working sessions.

## 6. Decisions needed

**From ROOT:**
- **D-U7-1, the scope wording.** Proposed: "U7 switches reader eligibility for library consumers and TS's mocked path. Public activation (a product caller delivering a successor: native W1 on the desktop, T6 outputs, a Headless or another workspace's Direct caller) is separately ruled later." This restates RR:9250's "Public activation stays with U7". Another workspace's Direct caller also needs re-qualification (RR:10407, :10555).
- **D-U7-2, eligibility in the shared corpus.** Proposed: snapshot 07i adds one `eligible` expectation to each base and must-pass entry, consumed by all three readers, so that parity covers the path U7 opens. The alternative is reader-local expectations from the oracle.
- **D-U7-3, N-6 in `lib.rs`.** Proposed: line-neutral (an inline `#[doc]` on 2254; `pub(crate) fn successor` on 2235, since only in-crate tests call it), **and** take the Pass B rerun, combined with U7's own Pass B. The alternative, a doc-only C-3, keeps a second public successor path once permits exist.
- **D-U7-4, TS N-2's declared difference.** Proposed: TS additionally requires the live native capture of these bytes and this model; PY and RS require the actual invocation argument. Declared in the case file's `declared_differences` (it is stricter and fails closed).
- **D-U7-5, the order.** Proposed: F lands only after the memory merge into NUM and after slices P and T. All three flags flip in one commit.
- **D-U7-6, producer origin.** Proposed: confirmed as designed. Eligibility is a property of a supplied statement and its invocation. No reader authenticates producer origin (`RS:4270`; C-2), and U7 claims none.

**For the owner** (no new owner choice is needed for U7 itself):
- U7 makes **no supported-machine statement of M** (decision 9 stays owner-held).
- U7 adds **no public product surface**: the desktop has no successor caller, and T6 outputs stay refused.
- If ROOT prefers U7 to include any public activation (D-U7-1's alternative), that becomes an owner-facing scope question.
