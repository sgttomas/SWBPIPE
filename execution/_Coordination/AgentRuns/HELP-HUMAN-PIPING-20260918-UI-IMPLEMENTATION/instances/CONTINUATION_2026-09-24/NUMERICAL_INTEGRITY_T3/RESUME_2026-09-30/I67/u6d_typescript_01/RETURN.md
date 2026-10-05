# I67 return: U6d, the TypeScript carriers and standing

I67 is a TASK (Type 2) under ROOT, working to `BRIEFS/U6_FANOUT_COMMON.md` and `BRIEFS/I67_U6D_TYPESCRIPT.md`, on the plan `R/I66/u6_scoping_01/PLAN.md` (sha256 `8742d105…`) §1d, §3, §4 and §6 U6d, as ruled in RR "U6 plan accepted" (D-U6-1 to D-U6-9) and "U6a verified and committed; fan-out". It did not delegate.

**Verdict: §1d is delivered inside the fence, and every in-fence control passes. One stop: a single existing test outside the fence pins the fresh set exactly, so D-U6-6 makes it fail.**
- **The successor** (`preview-physics-retained-1`) now dispatches, registers, stands, binds, records and reopens in TypeScript, always through the accepted reader, which is unchanged.
- **Standing** comes only from a registered reader validation of the exact bytes, never from `numerical_quality`. With the reader's flag held it is at best `needs_recompute`, in both modes, with and without the invocation.
- **The 14 shared parity scenarios** give the standing and dispatch codes that the shared file expects, which are the values Rust U6a asserts.
- **Existing identities are unchanged:** a 63-envelope sweep is identical to base on 17 desktop carrier outcomes, and every existing test keeps its outcome, apart from the stopped pin.
- **Mutants:** 103 of 103 are killed in the final run, every one by an assertion.
- **The stop (S-1):** `features/results/knownSemanticLimitations.test.ts:46–50` is outside U6d's fence. ROOT must grant or apply a 3-line patch, which is still an exact equality and gains only the ruled member. With the patch, the whole suite passes: 3417 of 3417 tests pass, and tsc reports 0 errors.

## Basis, host and fence

- **Worktree:** `WT/f2a-carriers-ts`, branch `codex/piping-f2a-carriers-ts-20261004`, at `844448112f` (U6a). No Git writes; Git reads used `GIT_OPTIONAL_LOCKS=0`.
  - NUM moved from `4e4b7eb5ad` to `e5c64ef062` during the run (RV87, records only). No reader, carrier or fixture file changed. The sibling U6 branches have no commits past `844448112f`.
- **When:** 2026-10-04, about 08:33Z to 09:37Z. The memory guard (PID 5387) ran throughout.
- **Runtime, disclosed** (`_run_records/runtime.txt`):
  - `WT/f2a-carriers-ts/P/node_modules` is a symlink to `REPO_ROOT/P/node_modules`. REPO_ROOT's desktop app has no `node_modules` of its own, so none was linked. No install was run.
  - `apps/desktop/public/{wasm-engine,self-weight-engine}` were copied from `WT/f2a-readers`, with hashes recorded. Nothing was built.
  - Node v24.18.0 and Vitest 4.1.10. `TMPDIR` was set to `WT/scratch/i67_u6d/tmp`.
  - Vitest wrote its cache under the ignored `apps/desktop/node_modules/.vite`.
  - **ROOT, note:** the symlink shows as untracked (`?? P/node_modules`), because `.gitignore`'s `node_modules/` matches directories, not symlinks. Do not stage it.
- **Not run:** nothing native, solver-at-scale or DEC-025, and no Cargo. Lanes are scratch copies: `base` is a `git archive` of `844448112f`; `cand` and `mut` are the candidate plus the proposed pin patch.
- **The fence held: 14 files** (10 changed, 4 new), all in the PLAN §6 U6d list. The reader `retainedPrecision.ts` is untouched. No T6 panel and no file outside §1d's list was edited.

## Changed files (`_run_records/changed_files_sha256.txt`; diff: `_run_records/candidate.diff`)

| File (TS = P/apps/desktop/src) | sha256 | Change |
|---|---|---|
| TS/features/results/numericalResultQuality.ts | `7baf4155…` | The successor's header route and binding; the downgrade guard; the standing branch, which reads only the registration; `ordinaryCaseEligible`, factored out unchanged |
| TS/features/results/retainedPrecisionStanding.ts (new) | `4e116971…` | Registration; standing; the seams `retainedStandingFrom` and `classificationSummaryFrom`; classes; the registered invocation; the standing text |
| TS/features/results/knownSemanticLimitations.ts | `4bccb682…` | The fresh set gains the id (D-U6-6). The two binding codes, `ruleBindingRefusal` over registered classes, `classificationSummary` use. `N_RP_*` texts, labels and notices; the preview labels are reused |
| TS/features/results/resultSemantics.ts | `d2a3251d…` | The successor's own pinned table |
| TS/features/results/loadReferenceOutputAvailability.ts | `910713da…` | `RETAINED-PRECISION-OUTPUT-NOT-YET-AVAILABLE` in the shared refusal (D-U6-8); `isLoadReferenceRoute` unchanged |
| TS/features/results/ResultsPanel.tsx | `daa96524…` | The successor's standing text from the receipt; its row labels shown (F8) |
| TS/features/results/HistoricalRunContext.tsx | `20e4c1cc…` | Reopen: reader revalidation without an invocation, and the AnalysisRun copy compared |
| TS/services/analysisRunCompatibility.ts | `27b75a9c…` | The route; the reader before recording; the receipt copied whole; equality and downgrade codes |
| TS/services/previewService.ts | `ed2cdb27…` | Registration at direct and job capture, reader first |
| TS/services/ruleCheckService.ts | `e92d5b55…` | The gate needs eligible standing; the registered invocation passed as `sourceBlockInvocation`; precheck class notices |
| TS/types.ts | `d31fed4e…` | Optional `retained_precision` (envelope and `analysis_run`) and row `recovery_method` |
| TS/features/results/retainedPrecisionIntegration.test.tsx (new) | `f33dbde1…` | 76 tests |
| TS/services/retainedPrecisionAnalysisRun.test.ts (new) | `7ae73ed1…` | 19 tests |
| TS/features/results/retainedPrecisionOutputRefusal.test.tsx (new) | `2470462c…` | 64 tests |

### How the change works

- **Dispatch** (`numericalResultQuality.ts`).
  - `sourceContract` gains the route `retained_preview_physics`. It needs the successor id, its profile `product_preview_retained_w1a_v2`, the inherited preview header and contract evidence, and a `retained_precision` object.
  - The route only selects the reader. The statement itself is checked by `validateRetainedPrecision` (G0–G8) at registration, at AnalysisRun build and validation, and at reopen.
  - The binding is {id, `c74742ce…`}. The successor reads its own pinned table, whose rows are preview-physics-1's.
- **The downgrade guard** (`retainedPrecisionDowngrade`, mirroring Rust `forbid_retained_member` and `forbid_retained_rows`).
  - Any other identity, legacy 0.1.0 included, is `unsupported` if it carries a `retained_precision` member (null included) or any raw row with the W1 token. The token is read from the reader's own `RETAINED_METHOD`.
  - Standing reports `RETAINED_PRECISION_DOWNGRADE_FORBIDDEN`.
  - Builders refuse such a source as unsupported. A record of another identity that carries a receipt is refused `ANALYSIS_RETAINED_PRECISION_DOWNGRADE_FORBIDDEN`.
- **Registration** (`retainedPrecisionStanding.ts`; `previewService.validateCapturedSource`, both direct and job).
  - After actual IPC, the reader runs with the captured `{request, solver_mode}`. Its outcome, the validation or the reader's first code, is recorded against the exact bytes it validated: checked JSON text plus negative-zero paths.
  - A refusal is recorded and rethrown, so the native invocation is not registered.
  - Copies, saved or reference bytes, a model-less solve and bytes outside the checked profile never register. Any later byte change voids the registration (T1's pattern).
- **Standing** (plan §3; D2 §4.9.4).
  - **Nothing registered:** `needs_recompute`, with `RETAINED_PRECISION_VALIDATION_REQUIRED`.
  - **The reader refused:** `unsupported`, with the reader's code as the finding.
  - **Otherwise:** `retainedStandingFrom`, the mirror of Rust `retained_standing_from`. It needs an invocation-bound validation with eligibility set, the model's requested load cases equal to the receipt's case order, `MECHANICS_SOLVED`, and every case `selected`, or `not_required` and ordinarily eligible (F-7). For the last, the base predicate is factored out of the generic branch as `ordinaryCaseEligible`, with identical behaviour.
  - **If that fails:** `needs_recompute`, with `RETAINED_PRECISION_NOT_NUMERICALLY_ELIGIBLE`.
  - `numerical_quality` is never read for the successor.
- **Binding and summary** (`knownSemanticLimitations.ts`).
  - `ruleBindingRefusal` selects the successor by producer id, as Rust does.
  - Over registered classes, `absolute_verified` gives `RULE_QUANTITY_BELOW_VERIFIED_FLOOR` and `not_covered` gives `RULE_QUANTITY_NOT_COVERED`. A headline is refused as the row its `result_ref` names.
  - With no valid registration every row is refused `RULE_QUANTITY_NOT_COVERED`, failing closed as in Rust F5.
  - `classificationSummary` (with `classificationSummaryFrom` as its seam) is Rust's shape and rule.
- **Text** (D2 §4.9.9).
  - `N_RP_ABSOLUTE` and `N_RP_NOT_COVERED` are D2's texts. In `N_RP_ABSOLUTE`, b is the receipt's published bound, printed upward to 3 significant digits (never below b), in the SI unit the reader classified in.
  - Each class row is labelled, and the successor reuses the preview labels and notices.
  - There are per-case summary notices, and an `N_RP_UNVALIDATED` notice when there are no validated classes.
  - The results-panel standing text comes from the registered receipt only.
  - Nothing names the stop-rule bound or extrema enclosure.
- **AnalysisRun** (`analysisRunCompatibility.ts`).
  - The record is built only after the reader passes. It copies `analysis_run.retained_precision` whole and carries no `source_block_recovery` or `contract_evidence`.
  - Validation reruns the reader and requires exact (key-order-insensitive) equality, else `ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH`.
- **The rule-check gate** (`ruleCheckService.ts`).
  - A successor needs eligible standing; otherwise it throws `<finding>: …` before any backend call.
  - The registered invocation goes in the existing `sourceBlockInvocation`.
  - The precheck shows class notices, never N-SB.
- **Reopen** (`HistoricalRunContext.tsx`).
  - The saved statement is revalidated without an invocation, and the reader's code is a finding.
  - The AnalysisRun copy is compared by checked hash.
  - Nothing registers, so standing stays `needs_recompute`, even with the post-U7 reader.
- **Output** (`loadReferenceOutputAvailability.ts`).
  - The shared function returns `RETAINED-PRECISION-OUTPUT-NOT-YET-AVAILABLE: …` for the successor. `isLoadReferenceRoute` is unchanged.
  - Every T6 surface refuses through it, or through the report package's fresh-result refusal.
- **Types:** optional `retained_precision` on the envelope and on `analysis_run`, and an optional row `recovery_method`.

## Tests (3 new files, 159 tests)

Inputs are PP's pinned milestone successors, checked by sha256 in every file, with their invocation, delivered through mocked IPC. This is a unit transport replay, not a native witness.

| Plan §6 U6d test | Where | What it shows |
|---|---|---|
| Registration through mocked direct and job IPC | Integration, both modes | Registered only after the reader passes with the captured invocation. `hasNativeMechanicsInvocation` holds; standing is `needs_recompute`, `NOT_NUMERICALLY_ELIGIBLE`; fresh by membership. |
| Synchronous standing | Integration | Copies, saved bytes, a model-less solve and unchecked bytes give `VALIDATION_REQUIRED`. Refused deliveries (edited row, quality claim, altered receipt, foreign mode) give `unsupported` with the reader's code, and are not registered natively. |
| A mutation voids the registration | Integration | Row value, zero sign, NaN or receipt edits void it; a revert restores it. |
| AnalysisRun copy and validate | AnalysisRun, both modes | Product path build; byte-equal copy; validation; reader revalidation from record plus source; drop, alter, body edit or null gives `ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH`; reordered keys pass. An edited or unchecked source gets the reader's code. A relabelled source is refused by all builders. A preview record with a receipt is `…DOWNGRADE_FORBIDDEN`. Rule-status composition leaves the raw hash and copy unchanged. Legacy dimension binding is identity. |
| Reopen | Integration, both modes | Saved: `[HISTORICAL_INPUT_MANIFEST_MISSING, VALIDATION_REQUIRED]`, not native, even with the post-U7 reader. A mutated row adds `RETAINED_PRECISION_RECEIPT_MISMATCH`. An altered or dropped copy gives `ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH`. No AnalysisRun means no compare. |
| Every T6 surface refusing | Output refusal, both modes | 18 gated panels show the successor reason, with no download and no load-reference text; preview-physics-1 is the control. Stress-neutral build, validate and panel; result export build, validate, derive and panel; report package (fresh refusal). |
| The rule-check gate | Integration, both modes | Refused before the backend: registered gives `NOT_NUMERICALLY_ELIGIBLE`; copy or refused gives `RULE_NATIVE_INVOCATION_REQUIRED`. Post-U7 simulation: eligible, with the registered invocation passed as `sourceBlockInvocation`. Precheck: headline and absolute rows refused with their labels; relative, input and non-quantity rows bind; unregistered rows are refused with `N_RP_UNVALIDATED`. |
| The 14 parity scenarios | Integration | All 14 equal the shared file's expected standing and dispatch (table below). |

**The post-U7 rules are tested without touching the flag:**
- through the pure seams, with every conjunct negated, F-7's fields, other statuses, duplicate or missing ids, and a two-case split summary;
- through a test-only wrapper of the reader (`u7.simulate`). It sets `numerical_eligible` on invocation-bound validations, and optionally fabricates one `not_covered` row, or no absolute rows, because no statement has them (U6a F3).

The wrapper is not product code; when off, it returns the reader's own frozen result.

### Parity with Rust U6a (`retained_precision_carrier_cases.json`, sha256 `952e39bf…`)

| Case | Expected standing | Expected dispatch | TypeScript (TS-specific finding) |
|---|---|---|---|
| `{m}:invocation` | `needs_recompute` | `ok` | equal (`…NOT_NUMERICALLY_ELIGIBLE`) |
| `{m}:no_invocation` | `needs_recompute` | `ok` | equal (`…VALIDATION_REQUIRED`) |
| `{m}:no_requested` | `needs_recompute` | `ok` | equal (`…NOT_NUMERICALLY_ELIGIBLE`) |
| `{m}:other_requested` | `needs_recompute` | `ok` | equal (`…NOT_NUMERICALLY_ELIGIBLE`) |
| `{m}:edited_row` | `unsupported` | `RETAINED_PRECISION_RECEIPT_MISMATCH` | equal (the reader's code) |
| `{m}:foreign_mode` | `unsupported` | `ok` | equal (`RETAINED_PRECISION_INVOCATION_MISMATCH`) |
| `{m}:relabelled_base_with_receipt` | `unsupported` | `RETAINED_PRECISION_DOWNGRADE_FORBIDDEN` | equal (`…DOWNGRADE_FORBIDDEN`) |

Each row holds for `{m}` = `sparse_interactive` and `dense_scrutiny`: 14 of 14 pass in the final run.

TS maps the file's `invocation` as follows:
- **`"fixture"`:** a capture of that invocation through mocked IPC;
- **`null`:** a delivery without a captured model;
- **requested refs:** the standing model's load cases;
- **dispatch:** the header route, then the reader without an invocation (an `unsupported` route reports its standing finding).

Rust asserts the same file in `u6a_shared_carrier_cases_rust`, verified by ROOT at `844448112f`. No case is accepted by one language and refused by the other. The only differences are the declared TS-specific strings and F1.

## Controls

**1. Suites and tsc** (`*_outcomes.tsv`; `compare_base_vs_*.txt`)

| Run | Tests | tsc |
|---|---|---|
| Base (`844448112f`, the worktree before any edit) | 3258/3258 passed, in 135 files | 0 errors |
| Candidate worktree, as delivered | 3416/3417. The only failure is the stopped pin, S-1. All 159 new tests pass | 0 errors |
| Candidate lane: worktree plus the proposed S-1 patch | **3417/3417** | 0 errors |

- **Existing outcomes:** every one of the 3258 base tests has the same outcome, except the S-1 pin. That pin fails in the worktree, and passes in the lane under its new title.
- **The reader:** its own tests, `retainedPrecision.test.ts` (436), are unchanged and pass.

**2. Existing identities: unchanged** (`zzI67Sweep.test.ts`; `sweep_*.tsv/.jsonl.gz`; `sweep_compare.txt`)
- **The sweep:** it walked every JSON file under P/{fixtures,core,tests} in the base and candidate lanes, and found 80 mechanics envelopes.
- **What it recorded, per envelope:**
  - route, binding, `hasCurrentSourceContract`, fresh;
  - standing with no model and with a model of its own cases;
  - standing reason, notices, output refusal, report reason, table;
  - each row's label, binding refusal and category;
  - `bindSourceResultDimensions` identity;
  - AnalysisRun v0.3 hash and validation; v0.2 build.
- **The result:**
  - **The 63 existing-identity envelopes are identical to base.** That is load-reference-1 ×6, load-reference-source-1 ×10, physics-1 ×6, physics-source-1 ×14, precision-1 ×4, preview-physics-1 ×7, source-blocks-1 ×15, and one legacy envelope with no producer.
  - **The 17 successor envelopes** are the 15 in corpus 07f plus the 2 milestones. They change as intended:
    - at base, `unsupported`, not fresh, AnalysisRun refused;
    - now, the retained route, fresh, an AnalysisRun that builds and validates, the report refused, `VALIDATION_REQUIRED` with no registration, every row binding-refused (unregistered), and the notices `headline-label` and `retained-precision-unvalidated`.

**3. Nothing weakened.**
- Every removed line is a rewrite (`candidate.diff`):
  - imports, union types and allow-lists that gain the new member;
  - doc lines;
  - `resultRowLabel`, rewritten with identical preview-physics-1 behaviour (pinned by the sweep's per-row labels);
  - `loadReferenceOutputRefusal`, gaining one branch;
  - the generic ordinary-case predicate, factored into `ordinaryCaseEligible` as its exact De Morgan form;
  - the precheck notice, still N-SB for every non-successor reason.
- No check, tolerance or test was removed or narrowed.
- The stopped pin is not edited. Its proposal keeps it exact.

**4. Mutants: 103 of 103 are killed in the final run, every one by an assertion** (`mutants.py`, `mutants_final.json/.log`; `mutants_round1.*` and `mutants_round2_survivors.*` are the earlier rounds)
- **The setup:** one mutant per new branch, across all ten changed product files: dispatch, guard, binding, standing, the shared predicate, registration, the seams, classes, summary, text, labels, notices, output refusal, table, AnalysisRun, capture, gate, precheck, panel and reopen.
- **Each run:** the 3 new test files plus 8 related existing ones, in the mutant lane.
- **Round 1:** 97/103 killed. The 6 survivors were each closed with a test before the final run:
  - **N23:** mixed evidence refs;
  - **S04:** a direct registration of unchecked bytes;
  - **S27:** a two-case summary split;
  - **K08:** a statement with no absolute rows;
  - **K15:** a mantissa carry (9.994e-5);
  - **K24:** an equivalent route guard, which was removed from the code. The replacement mutant is killed.
- **The final run, on the frozen candidate:** **103 of 103 killed**, all by assertion, none by a load or compile error; the control run passes 296/296.

## The stop, for ROOT to rule

**S-1 (a write outside the fence).** `apps/desktop/src/features/results/knownSemanticLimitations.test.ts:46–50` pins `FRESH_SEMANTIC_CONTRACT_IDS` by exact equality. D-U6-6 adds the successor, so the pin fails: it is the only failing test in the worktree.
- The file is not in PLAN §6 U6d's fence, so I did not edit it.
- **Proposed** (`_run_records/proposed_knownSemanticLimitations.test.ts.patch`): add the successor id to the sorted list, and the words "and U6's preview-physics-retained-1 (D-U6-6)" to the title. It is still an exact equality, widened only by the ruled member, as U6a did in Rust's `preview_physics_contract.rs`.
- **Verified in the candidate lane:** 3417 of 3417 tests pass, and tsc reports 0 errors.
- **ROOT:** grant it to U6d, or apply it at commit.

## Findings

- **F1 (a declared language difference).** TS standing is synchronous and registration-based (plan §3 rule 1), so an unregistered successor reads `needs_recompute` (`VALIDATION_REQUIRED`) even when its statement is invalid. Rust and Python read `unsupported` for an invalid statement without an invocation.
  - Neither is ever eligible.
  - Reopen adds the reader's own code as a finding.
  - The 14 shared cases agree.
- **F2 (for RV88).** The binding precheck refuses every row of an unregistered successor (`RULE_QUANTITY_NOT_COVERED` with `N_RP_UNVALIDATED`). Rust's helper classes a valid statement without an invocation, and would bind its relative rows, subject to standing.
  - The TS precheck is display only.
  - The rule check itself requires registration and eligible standing, and the backend enforces with Rust.
  - This fails closed. Ruling it the other way would need an async class registry for unregistered bytes, which plan §1d did not choose ("over registered classes").
- **F3 (the native witness and the milestone request).** PP's pinned request model is not a complete desktop model:
  - its load case has no `status`;
  - it has no `components`, `data_boundary` or `diagnostics`.
  
  The desktop input manifest therefore refuses it (`INPUT-MANIFEST-LOAD-BASIS-INCOMPLETE`), and a full workspace-session replay with the exact pinned invocation is impossible: adding fields changes the invocation, so the reader's G8 refuses. The tests use the exact invocation for delivery and registration, and add invented fields only to manifest and panel models.
  - **Consequence for D-U6-3:** a native or session witness needs a desktop-shaped successor request, and that belongs with native activation (F-1). This is the stated qualification limit: **no native witness in U6.**
- **F4 (a T6 header packet).** A header-only stress-neutral packet cannot carry `retained_precision`: its header reconstruction copies only `source_block_recovery` and `contract_evidence`. So a successor packet header reads unsupported (`SN-PRECISION-CONTRACT-MISMATCH`), not the shared refusal.
  - This fails closed, and no T6 file was edited. Noted for T6.
- **F5 (another consumer, not edited, as planned).** `ComparisonPanel`'s dimension list does not name the new route, so successor deltas show an `unknown` dimension. This is display only and conservative.
- **F6 (new product text, for RV88):**
  - `N_RP_UNVALIDATED`;
  - the per-case summary notices;
  - `N_RETAINED_PRECISION_OUTPUT`;
  - the results-panel standing text;
  - the upward 3-significant-digit printing of b, with SI units.
  
  `N_RP_ABSOLUTE` and `N_RP_NOT_COVERED` are D2's texts verbatim.
- **F7 (names for D-U6-4's recheck)** (`collision_check.py`, `COLLISIONS.json`). Beyond the 25 reserved names, this unit adds:
  - `PREVIEW_PHYSICS_RETAINED_CONTRACT_ID/_SHA256`, `retainedPrecisionDowngrade`, `ordinaryCaseEligible`;
  - `registerRetainedPrecision`, `retainedPrecisionRegistration`, `retainedStandingFrom`, `retainedPrecisionInvocation`, `retainedRowClasses`, `classificationSummaryFrom`, `retainedPrecisionStandingText`, `RetainedStandingToken`, `RetainedClassificationSummary`;
  - `N_RP_UNVALIDATED`, the notice id `retained-precision-unvalidated`, `upwardBoundText`, `retainedAbsoluteNotice`, `classBindingRefusal`, `retainedRowClassLabel`;
  - `RETAINED_PRECISION_OUTPUT_REFUSAL`, `N_RETAINED_PRECISION_OUTPUT`, `RETAINED_PRECISION_OUTPUT_NOT_YET_AVAILABLE`.
  
  All are absent at NUM `e5c64ef062`, `origin/main` `09106477e3`, the facade `8abb5274a9` and `844448112f`, except two:
  - `PREVIEW_PHYSICS_RETAINED_PROFILE`, which is U6a's own Rust constant (same name, same value; deliberate);
  - a first choice, `RETAINED_PRECISION_ROW_METHOD`, dropped because it is a substring of the reader's `…ROW_METHOD_MISMATCH`.
- **F8 (the fence, as worded).** `ResultsPanel.tsx` has two changed lines, not one:
  - the standing text;
  - `labelSource`, so that the successor's class labels show ("reuses the preview labels"; D2 "never unlabelled").
- **F9 (structure).** The new module closes an import cycle: `numericalResultQuality` → `retainedPrecisionStanding` → reader → `numericalResultQuality`, because the reader's G7 calls `sourceContract`.
  - No module reads a binding across it at top level. That is why the id, sha and profile are literals, pinned to the reader by test.
  - The integration test's reader wrapper depends on its import order, which is commented in the test.
- **F10 (cost).** The TS reader takes milliseconds on the milestone (76 integration tests run in about 3 s).
  - Registration validates once, AnalysisRun build plus validation twice, and reopen once.
  - Class labels refingerprint the source per call: about 2 per rendered row, on 50-row pages. That is acceptable now; a cache could come later (cf. U6a F7).

## Records and next

- **Records:** `_run_records/` holds the scripts, the proposed patch, sweeps, outcomes, mutants, collisions, runtime and the changed-file hashes, all with placeholder paths. SHA256SUMS covers this folder.
- **Next:**
  1. ROOT rules S-1.
  2. ROOT verifies and commits on `codex/piping-f2a-carriers-ts-20261004`, staging the 14 files only: not the `node_modules` symlink or `public/`.
  3. RV88 reviews.
  4. The branch merges into the carriers branch, then U6f.
