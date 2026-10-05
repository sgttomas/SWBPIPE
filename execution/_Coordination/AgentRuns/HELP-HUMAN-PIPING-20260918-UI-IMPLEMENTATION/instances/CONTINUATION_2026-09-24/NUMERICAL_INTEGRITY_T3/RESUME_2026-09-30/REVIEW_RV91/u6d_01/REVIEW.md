# RV91: independent review of U6d (TypeScript carriers and standing)

RV91 is a TASK (Type 2) dispatched directly by ROOT (HELP_HUMAN, Agent 0). ROOT is the return path, and nothing was delegated.

- **Brief:** `R/BRIEFS/RV91_U6D_TS_REVIEW.md`, at NUM `5b191a99a1`. Basis read: `BRIEFS/U6_FANOUT_COMMON.md`; PLAN `R/I66/u6_scoping_01/PLAN.md` §1d, §3, §4 and §6 U6d; RR from "U6 plan accepted…" through "U6b (Python carriers) verified…"; D2 §4.7 and §4.9; I67's `RETURN.md`.
- **Candidate:** `9555b6ffc2` on `codex/piping-f2a-carriers-ts-20261004` (15 files, +1407/−31).
- **Base:** `844448112f` (U6a).
- **Independence:** I did not write this code. My oracles are my own sweep, probes, parity harnesses (TS and Python), schema check, merge preview and mutants. The author's tests and records were used only as objects under review (their mutant list is re-run, not trusted).
- **Host:** lanes are `git archive` copies under `WT/rv91/` (cand, base, an APFS-clone mutant lane, and a merge preview). `P/node_modules` is linked to REPO_ROOT's and the prebuilt WASM is copied from `WT/f2a-readers` (hashes equal I67's); nothing was installed or built. Python checks used the existing REPO_ROOT venv with the prebuilt I52 checked-JSON and units CLIs. No Git writes, no Cargo, nothing native, solver or DEC-025. The memory guard (PID 5387) ran throughout. `TMPDIR` was `WT/scratch/rv91_u6d/tmp`. Details: `_run_records/runtime.txt`, `_run_records/basis.txt`.
- **When:** 2026-10-04, about 10:52Z to 11:25Z.

## Verdict

**PASS, with 1 SHOULD-FIX and 5 NOTEs. Nothing is BLOCKING.** The SHOULD-FIX is I66's F-U6b-3 on the TypeScript side, which ROOT has already routed to I67's repair round. ROOT rules on F1 (N-1).

- **Existing behaviour is unchanged.** Base 3,258/3,258 and candidate 3,417/3,417 Vitest, `tsc` clean in both; the only base test missing from the candidate is the S-1 pin, renamed. My own sweep of **69 existing-identity envelopes** (86 envelopes in 72 files, embedded objects included) is identical to base on **25 carrier outcomes** each, including registration through mocked IPC, AnalysisRun v0.3/v0.2 bytes, a ResultsPanel render and reopen.
- **Registration is bound to the exact bytes it validated.** Eight kinds of edit (row value, zero sign, new member, `numerical_quality`, receipt body, method token, diagnostic, contract evidence) each void it; a copy never registers; an edit made while the reader awaits does not register the edited bytes. Standing never reads `numerical_quality` and stays `needs_recompute`.
- **The 14 shared cases agree in all three languages.** TS 14/14 (my harness, mocked IPC) and Python U6b 14/14 (my harness at `c89a7a986c`); Rust U6a asserts the same file in `u6a_shared_carrier_cases_rust` (read; ROOT ran it at U6a). Registered TS binding equals Python row for row (98/98, 99/99) and the summaries are equal (25/69/0/0/3/1–2, withheld 97).
- **F1 (I67):** a declared, fail-closed difference consistent with the accepted plan §3 and TS's existing registration routes. **TS need not match;** I recommend pinning it in the shared case file (N-1).
- **Guards and codes hold:** AnalysisRun equality (drop, null, hash, body, extra member → `ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH`; key order → passes; receipt on a base record → `…DOWNGRADE_FORBIDDEN`), reopen (a re-hashed altered copy is caught by the copy check alone), the rule-check gate (refuses before any backend call; post-U7 it sends the captured invocation), and every T6 surface (none edited) refuses. The TS-built successor records validate under U6c's AnalysisRun schemas.
- **The S-1 pin patch** keeps exact equality and gains only the ruled id; its hunk equals I67's proposed patch line for line.
- **New text claims nothing beyond the receipt.** `N_RP_ABSOLUTE` and `N_RP_NOT_COVERED` are D2's texts verbatim. The upward bound is never below b over 25,017 binary64 samples (exact rational check), and all 69+69 milestone labels are sound and tight in the reader's SI unit.
- **F-U6b-3:** yes, the TS v0.2 builder silently drops `retained_precision` (SF-1).
- **Mutants:** I67's sample of 28 (of 103) all reproduce killed. 11 of my own: 8 killed (7 by I67's suite, 1 only by my probe), 3 survive (two product-equivalent hardening clones, one real test gap) (N-3).
- **Merge preview:** the carriers tip `924c6284cb` (U6a+U6c+U6b+U6e 07h) plus U6d's 15 files (no overlap) gives 3,429/3,429 and `tsc` clean.

## Counts

**0 BLOCKING · 1 SHOULD-FIX · 5 NOTE**

| # | Severity | One line |
|---|---|---|
| SF-1 | SHOULD-FIX | TS `buildAnalysisRunV02` builds from a legacy-shaped source carrying `retained_precision` (object or null) or W1 token rows and silently drops the receipt (I66 F-U6b-3, TS side; routed to I67's repair round). |
| N-1 | NOTE | F1 judged: TS `needs_recompute` vs Rust/Python `unsupported` for an unregistered invalid successor is an acceptable declared difference; TS need not match; pin it in the shared case file. |
| N-2 | NOTE | Post-U7 the TS successor standing is not bound to the live native registration or model (eligible for another model with the same case ids, and after an accepted cancel); every current consumer also checks `hasNativeMechanicsInvocation`. A U7 precondition. |
| N-3 | NOTE | Test gaps: route-selected binding (RV08) survives all tests; the pre-await fingerprint (RV09) is killed only by RV91's probe; the invocation clones (RV03, RV04) are untested but product-equivalent. |
| N-4 | NOTE | The standing text prints "Selected cases: n of m" from an unvalidated or refused receipt. |
| N-5 | NOTE | After U7, the result-export and stress-neutral panels would rely on the builders' shared-refusal throw (their gates use the unchanged `isLoadReferenceRoute`); the stress-neutral panel shows its generic empty text for a successor. For U7 and T6. |

## 1. Existing behaviour unchanged

**Suites** (`_run_records/suites/`).

| Lane | Files | Tests | `tsc` |
|---|---|---|---|
| Base `844448112f` | 135 | 3,258/3,258 | 0 errors |
| Candidate `9555b6ffc2` | 138 | 3,417/3,417 | 0 errors |
| Merge preview (`924c6284cb` + U6d's 15 files) | 138 | 3,429/3,429 | 0 errors |

Per-test comparison (`compare_base_cand.txt`): no base test changes outcome. One base title is absent from the candidate: the S-1 pin, present under its new title. 160 titles are new (159 tests in the three new files, plus the renamed pin).

**My sweep** (`_run_records/sweep/`). `zzRV91Sweep.test.tsx` walks every JSON document under one fixed input tree (the candidate's P/{fixtures, core, examples, validation, apps/desktop/e2e, tools}), finds every **embedded** mechanics envelope (not only top-level ones), and computes each outcome with the lane's own modules. It ran in the base and candidate lanes on identical inputs.
- **86 envelopes in 72 files.** Existing identities, 69: load-reference-1 ×6, load-reference-source-1 ×10, physics-1 ×6, physics-source-1 ×14, precision-1 ×4, preview-physics-1 ×7, source-blocks-1 ×15, no producer ×7. Successors, 17: the 15 corpus bases and the 2 milestones.
- **25 outcomes per envelope:** route; binding; current contract; `hasCurrentSourceContract`; fresh; standing reason; standing with no model, with a model of its own cases, and with a paired model where the file has one; notices; output refusal; `isLoadReferenceRoute`; semantic table; a per-row digest (label, binding refusal, `resultSemantics`, `analysisRowSemantics`); label count; refusal set; the binding precheck; `bindSourceResultDimensions`; AnalysisRun v0.3 (record hash or code) and v0.2; a ResultsPanel render (standing text and a digest of all panel text); registration through mocked direct IPC (native registration, with mode, and standing after it); and a reopen context digest.
- **Result: all 69 × 25 existing outcomes are identical to base.** In both lanes, 41 register natively, 61 build an identical AnalysisRun v0.3 record, 7 build v0.2, and all 69 render and reopen.
- **The 17 successors change on 19 fields, as intended:** route, binding, fresh, standing findings, notices, output refusal, table, labels, refusals, precheck, AnalysisRun v0.3 (now built), panel, registration and reopen.
- **The deliberate guard change, probed:** adding a `retained_precision` member to each of the 69 existing envelopes turns every one `unsupported` with `RETAINED_PRECISION_DOWNGRADE_FORBIDDEN` (base: its own route). No identity escapes the guard, legacy 0.1.0 included.

## 2. Registration and standing

**Bound to the exact bytes and the captured invocation** (`probes/review_cand.json`: `registration_voids`, `registration_edit_during_await`).
- After registration through mocked direct IPC (sparse), each of 8 edits voids it: standing becomes `[RETAINED_PRECISION_VALIDATION_REQUIRED]` and the native registration fails too. The edits are a row value, the sign of a zero, a new top-level member, `numerical_quality.status`, a receipt body member, a removed method token, a diagnostic and `contract_evidence`. Reverting a value edit restores the registration; reverting a delete-and-re-add does not (key order changes the checked text). Both are conservative.
- A `structuredClone` copy never registers.
- **An edit made while the reader awaits:** the reader validates the synchronously captured bytes ("validated"), the edited object reads unregistered, and the reverted object reads the validation. The validation is never recorded against bytes it did not check.
- Job IPC registers the same way (`probes/review2_cand.json`).
- The registered invocation is a `structuredClone` taken before the reader's first await. After registration the caller's objects cannot alter it, and any edit to the native capture voids `hasNativeMechanicsInvocation`, which the rule-check gate checks first.

**Standing reads only the registration and stays `needs_recompute`.**
- With every `numerical_quality` case rewritten to `failed` (structural status changed, evidence refs empty), `retainedStandingFrom` still gives `numerically_eligible` for a selected case under a forced validation. It never reads `numerical_quality` for a selected case. `not_required` cases use it only through the shared base predicate (F-7).
- With the real reader, standing is `needs_recompute` in every case: registered → `RETAINED_PRECISION_NOT_NUMERICALLY_ELIGIBLE`; unregistered → `RETAINED_PRECISION_VALIDATION_REQUIRED`.
- I67's N16 (the successor routed to the generic `numerical_quality` branch) reproduces killed.
- `ordinaryCaseEligible` is the exact De Morgan form of the removed condition (read, and the sweep's generic standings are identical).

**The 14 parity cases** (`parity/`, `probes/review_cand.json` `parity`).

| | TS (RV91 harness) | Python U6b (RV91 harness) | Rust U6a |
|---|---|---|---|
| 14 shared cases, standing | 14/14 equal expected | 14/14 equal expected | asserted from the same file (`u6a_shared_carrier_cases_rust`, which also asserts dispatch; ROOT ran it at U6a) |
| 14 shared cases, dispatch | 14/14 | 14/14 | as above |

My 7 extra cases (`parity/extra_cases.json`):
- **Agree:**
  - a null receipt on a relabelled base;
  - token rows on a relabelled base;
  - legacy 0.1.0 carrying the receipt.
  
  All three are `unsupported` with `RETAINED_PRECISION_DOWNGRADE_FORBIDDEN` in both languages.
- **Agree on the token, with language-specific codes:** a successor with its receipt dropped is `unsupported` in both. TS reports `SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED`, Python `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED`, as at base for unsupported headers.
- **Differ (F1):** an edited row with no invocation (both modes), and `numerical_quality` rewritten to `checks_passed` with no invocation. Python gives `unsupported`; TS gives `needs_recompute` with `VALIDATION_REQUIRED`. Both refuse dispatch with the same reader code (`RETAINED_PRECISION_RECEIPT_MISMATCH`).

**Binding and summary parity** (`parity/py_binding.json`, `probes/review2_cand.json`):
- A registered TS successor gives the same `ruleBindingRefusal` as Python's `rule_binding_refusal` on every row: 98/98 and 99/99, with 69 `RULE_QUANTITY_BELOW_VERIFIED_FLOOR`.
- The TS `classificationSummary` equals Python's `classification_summary(source, invocation)` exactly.
- An unregistered TS successor refuses all 98/99 rows and returns an empty summary. Python, validating without an invocation, refuses the 69 absolute rows and returns the counts. This is I67's F2, and it fails closed.

**F1 judged against D2 §4.7.**
- **What D2 §4.7 requires:** exact agreements, declared language-specific strings, zero undeclared differences, and "zero cases accepted by one language and refused by another".
- **F1 meets that:**
  - it is declared (I67 F1; RR "U6d verified");
  - nothing is accepted anywhere: all three are never eligible;
  - TS still refuses everything that relies on the statement: binding (all rows), the AnalysisRun (the reader runs at build), reopen (the reader's code is a finding), and rule checks (eligible standing required).
- **What it departs from:** the letter of D2 §4.9.4's table ("`unsupported`: any G-check fails"), for one input class only: an invalid statement that was never registered.
- **Why that is consistent with the accepted basis:**
  - the accepted plan §3 rule 1 says "nothing is registered (TS): `needs_recompute`";
  - TS's synchronous standing cannot run the asynchronous reader;
  - TS's existing receipt routes already behave this way. source-blocks-1 and physics-source-1 (`knownSourceStanding`) and load-reference-source-1 read `needs_recompute` with a validation-required finding for unregistered content-invalid receipts, where Rust and Python `for_source` refuse.
- **Judgment: TS need not match.** See N-1 for pinning it.

## 3. Downgrade guards and refusal codes

**AnalysisRun** (both modes; `probes/review_cand.json` `analysis_run_*`).

| Record or source | Result |
|---|---|
| As built | validates; the copy is byte-equal to the source receipt; no `contract_evidence` or `source_block_recovery` |
| Receipt dropped, null, `receipt_sha256` altered, body altered, extra member | `ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH` |
| Receipt keys reversed | validates (key-order-insensitive) |
| A preview-physics-1 record (the reader's projection) carrying the receipt | `ANALYSIS_RETAINED_PRECISION_DOWNGRADE_FORBIDDEN` |
| A valid record against an edited source | the reader's `RETAINED_PRECISION_RECEIPT_MISMATCH` |

The TS-built successor records validate under U6c's `analysis_run.schema.json` and `analysis_run.v0.3.schema.json` (at `c89a7a986c`, with the repo's `tests/schema_validation.py`). Dropping the receipt is refused: `probes/schema_check.json`.

**Reopen** (`reopen`).
- A saved successor reads `[HISTORICAL_INPUT_MANIFEST_MISSING, RETAINED_PRECISION_VALIDATION_REQUIRED]` and is not eligible, even with the post-U7 reader simulated.
- An altered AnalysisRun copy with the record hash recomputed (so no hash finding fires) still gives `ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH`: the new copy check works alone.
- A mutated saved row adds the reader's `RETAINED_PRECISION_RECEIPT_MISMATCH` and `HISTORICAL_RESULT_HASH_MISMATCH`.

**Rule-check gate** (both modes).
- A registered successor is refused `RETAINED_PRECISION_NOT_NUMERICALLY_ELIGIBLE` with **0** backend calls.
- A copy is refused `RULE_NATIVE_INVOCATION_REQUIRED`.
- Under the simulated post-U7 reader, exactly one `run_rule_checks` call is made, and its `sourceBlockInvocation` equals the captured invocation.

**T6 surfaces.**
- No T6 file is in the diff.
- Every call site routes through the shared function or a stricter gate: resultExportAdapter.ts:58, :118, :172; StressNeutralExportPanel.tsx:489, :620, :1059; RenderedReportPanel.tsx:70; LoadReferenceOutputGate.tsx:9. ResultExportPanel.tsx:14–16 and StressNeutralExportPanel.tsx:85–86 also require eligible standing and a native invocation.
- The shared function returns `RETAINED-PRECISION-OUTPUT-NOT-YET-AVAILABLE: …` for a registered and an unregistered successor, and never the load-reference text. A relabelled base carrying the receipt is `unsupported`, and the adapters refuse it as such.
- I67's 64 output-refusal tests pass in my run, as do L01 and the A-series mutants.

## 4. The S-1 pin patch

`knownSemanticLimitations.test.ts:46–51` still asserts `toEqual` on the sorted fresh set, an exact list. It gains only `openpipestress.result_semantics/0.3.0/preview-physics-retained-1` and a title change; no other assertion changes. The committed hunk's changed lines equal I67's `proposed_knownSemanticLimitations.test.ts.patch` line for line (`_run_records/s1_*_lines.txt`). The pin passes in the candidate and in the merge preview.

## 5. New product text

All the texts are captured in `probes/review_cand.json` `texts`.

- **`N_RP_ABSOLUTE` and `N_RP_NOT_COVERED`** equal D2 §4.9.9's texts verbatim, checked programmatically against DESIGN_STANDING/DESIGN.md.
- **b, upward to 3 significant digits** (`upwardBoundText`, knownSemanticLimitations.ts:61–68):
  - **Never below b:** checked by exact rational comparison (BigInt) on 25,017 binary64 values, from subnormal to `MAX_VALUE`, including 9.994e−5, 9.995e−5, 1, 1.25 and 0.0625. The format is always `d.dde±x`, and b = 0 prints `0`.
  - **Tight:** it is one step loose only where the 3-digit decimal equals b exactly (5 samples: 1.25 → 1.26e+0, 0.0625 → 6.26e−2, 1, 123). That step-up is required for soundness: I67's K13 (`>=`) prints a decimal below b, and my bound probe kills it.
  - **On the milestones:** all 69 + 69 absolute labels print a value at least the receipt's listed bound and at most one step above it, in the reader's SI unit (mm→m, MPa→Pa, N, N·m, Pa).
  - **Units:** b is in SI because the reader classifies on SI-normalized values (retainedPrecision.ts:786, :1036).
- **`N_RP_UNVALIDATED`:** "No quantity of this result is shown as verified, and rule checks cannot bind to any of them." This holds: the unregistered ResultsPanel shows no `checks_passed` and no "Numerical integrity", the precheck refuses every row, and the gate refuses.
- **Per-case notices:** "n quantities verified only to an absolute bound…" and "…uncovered…" are D2's UI-summary wording, counted from registered classes only.
- **Standing text** (retainedPrecisionStanding.ts:191–198): "validated … against the actual invocation" appears only for an invocation-bound registered validation; it says "refused … (code); unsupported" and "not validated … needs recompute" otherwise. See N-4 for the case count.
- **Output refusal text:** routed to T6; "not a finding about the result".
- **Not claimed anywhere:** the stop-rule bound and extrema enclosure. My probe asserts their absence from all new texts.

## 6. I66's F-U6b-3

**Yes.** `buildAnalysisRunV02` (analysisRunCompatibility.ts:78–80) checks only `producer`, `numerical_quality`, `formulation_basis`, `contract_evidence` and `source_block_recovery`. A legacy-shaped source that carries `retained_precision` (an object or null), or a row with the W1 method token, builds a v0.2 record without the receipt. The `received_result` hash binds the receipt, but the record does not carry it (`probes/review_cand.json` `f_u6b_3`). This is the behaviour Python's v0.2 builder had before U6b, which now refuses `ANALYSIS_LEGACY_SOURCE_DOWNGRADE_FORBIDDEN`. It is pre-existing at base, and U6d did not add the guard. There is no product caller: only `buildAnalysisRunV03` is called, from previewService.ts:303. See SF-1.

## 7. Mutants

`_run_records/mutants/`. Each mutant runs in the mutant lane against I67's 3 new and 8 related test files plus my probe file, with kills attributed per file set. The control passes 308/308.

- **I67's sample, 28 of 103** (N06, N09, N13, N16, N23, S03, S04, S05, S09, S14, S22, S25, K03, K13, K15, K17, L01, R01, A02, A04, A05, P01, P02, C01, C02, RP1, H03, H05; their exact text): **28/28 killed**, every one by I67's suite. My probes independently kill 16 of them.
- **My own 11:**

| Id | Mutation | I67 suite | RV91 probes |
|---|---|---|---|
| RV01 | guard: a null receipt member is admitted on another identity | killed | — |
| RV02 | guard: legacy 0.1.0 is exempt | killed | — |
| RV03 | registration keeps the caller's live invocation object | **survives** | survives |
| RV04 | the registered invocation is handed out by reference | **survives** | survives |
| RV05 | AnalysisRun equality on `receipt_sha256` only | killed | killed |
| RV06 | the label prints S\* instead of b | killed | killed |
| RV07 | no upward step when nearest is below b | killed | killed |
| RV08 | `ruleBindingRefusal` selects the successor by route, not by producer id | **survives** | survives |
| RV09 | the registration fingerprint is taken after the reader's await | **survives** | **killed** |
| RV10 | the precheck never shows the class label | killed | — |
| RV11 | reopen drops the reader's code | killed | killed |

**8 of 11 are killed.** RV03 and RV04 are product-equivalent: the capture is private to previewService, an edit to it voids the native registration, which the gate checks first, and Rust revalidates G8. RV08 and RV09 are real gaps (N-3).

## Findings

### SF-1 (SHOULD-FIX): the TS v0.2 builder drops a `retained_precision` member (I66 F-U6b-3, TS side)

- **Where:** `P/apps/desktop/src/services/analysisRunCompatibility.ts:79`.
- **Evidence:** `probes/review_cand.json` `f_u6b_3`:
  - a 0.1.0 source without producer, quality, formulation or evidence, but carrying the milestone receipt, routes `unsupported` under the guard, yet `buildAnalysisRunV02` builds a record with no `retained_precision`, whose `received_result` hash binds the receipt;
  - the same holds with `retained_precision: null`, and with only a W1 token row;
  - a successor itself is refused (`HISTORICAL_ANALYSIS_SOURCE_UNSUPPORTED`).
  
  Python's `_build_analysis_run` (record 0.2.0) refuses the member (U6b, compatibility.py:85–87).
- **Impact:** a receipt is silently dropped, the D-U6-9 hazard, on an explicit historical builder with no product caller. It fails open only on the record's carriage, never on standing.
- **Remedy (already routed by ROOT to I67's repair round):**
  - add `retained_precision` to line 79's forbidden-member list;
  - preferably refuse `retainedPrecisionDowngrade(result)` as well, so token rows are refused too, with the same code family as Python;
  - add a test for each of: member, null member, token row.

### N-1 (NOTE): F1 is acceptable as a declared difference; pin it

- **Where:** retainedPrecisionStanding.ts:148–151; numericalResultQuality.ts:139–144.
- **Evidence:** §2 above; `parity/py_parity.jsonl` and `probes/review_cand.json` `parity` (the 3 extra F1 cases).
- **Judgment:** TS need not match Rust and Python here. The difference is:
  - fail-closed in every reliance path;
  - declared;
  - prescribed by accepted plan §3 rule 1;
  - the same as TS's existing receipt routes.
  
  It departs only from the letter of D2 §4.9.4 ("`unsupported`: any G-check fails") for unregistered invalid statements. Its siblings, the unregistered binding (every row refused) and the empty unregistered summary, are I67's F2. Both fail closed.
- **Remedy (U6f or the repair round; ROOT's choice):**
  - add one case per mode to `retained_precision_carrier_cases.json`: an invalid edit with `invocation: null`, expecting `unsupported`, with a declared TS expectation of `needs_recompute`/`RETAINED_PRECISION_VALIDATION_REQUIRED`, so the difference is a tested fact rather than prose;
  - record it in U6f's parity summary as the one declared token difference.

### N-2 (NOTE): post-U7, successor standing is not bound to the live native registration or model

- **Where:** retainedPrecisionStanding.ts:140–154 (standing uses only the registration and `model.load_cases`); previewService.ts:131 (registration before the invalidation re-check at :133).
- **Evidence** (under the simulated post-U7 reader):
  - **Another model:** a registered successor is `eligible` for a different model with the same load-case ids (a moved node), while `hasNativeMechanicsInvocation` is false and the rule-check gate refuses `RULE_NATIVE_INVOCATION_REQUIRED` (`model_divergence_post_u7`).
  - **An accepted cancel:** a job cancelled while the reader awaited keeps the retained registration but loses the native one, and still reads `integrity_checked`/eligible (`probes/review2_cand.json` `cancel_during_validation`).
  
  TS source-blocks standing (`knownSourceStanding`) binds the model with `sameModelData`, but has the same registration order. Rust has the same split: requested refs come from the caller and the invocation is separately bound.
- **Impact now: none.**
  - The flag is false.
  - Every product consumer of `.eligible` also requires `hasNativeMechanicsInvocation`: resultsSessionState.ts:55–75; ResultExportPanel.tsx:14–16; StressNeutralExportPanel.tsx:85–86; resultExportAdapter.ts:218; ruleCheckService.ts:122.
- **Remedy before U7:** do one of the following:
  - bind `retainedPrecisionStanding` to the registered invocation's model, as `knownSourceStanding` does, or to the live native registration;
  - pin by test that every `.eligible` consumer also checks `hasNativeMechanicsInvocation`.
  
  Add this to U7's checklist.

### N-3 (NOTE): test gaps found by mutants

- **Where:** knownSemanticLimitations.ts:111 (RV08); retainedPrecisionStanding.ts:81–82 (RV09; RV03); :159 (RV04).
- **RV08:** a successor-id envelope whose TS dispatch fails (for example, a broken profile) is refused `RULE_QUANTITY_NOT_COVERED` on every row, matching Rust's producer-id selection. No test pins it. With route selection it would bind as an ordinary envelope in the (display-only) precheck. The gate still refuses, because such a source is not fresh.
- **RV09:** the "captured before the reader's first await" property, on which byte binding rests, is pinned only by my probe.
- **RV03 and RV04:** the clones are untested but product-equivalent.
- **Remedy (repair round):**
  - a test that a successor-id envelope with a failing header refuses every row;
  - adopt RV91's edit-during-await probe (`probes/zzRV91Review.test.tsx`, "binds the exact bytes…").

### N-4 (NOTE): "Selected cases: n of m" is read from an unvalidated or refused receipt

- **Where:** retainedPrecisionStanding.ts:193–198.
- **Evidence:** unregistered and refused texts both say "Selected cases: 1 of 1" (`texts`).
- **Impact:** the sentence sits after "receipt not validated" or "refused", so it implies no verification, but the count is the receipt's unverified claim.
- **Remedy (optional, text):** print "the receipt claims n of m selected cases" when the outcome is not a validation, or omit the count. Also, "historical values only" for a freshly received refused result echoes the generic unsupported text; consider "values shown for inspection only".

### N-5 (NOTE): after U7, two T6 panels would rely on the builders' throw

- **Where:** ResultExportPanel.tsx:14–16; StressNeutralExportPanel.tsx:85–86 and :207–212 (unchanged T6 files).
- **Evidence:** their live bindings exclude only `isLoadReferenceRoute`, which is unchanged by ruling. Today the successor's held standing keeps them inert, and I67's test pins the stress-neutral panel's generic "Run mechanics with the native backend…" text for a successor. Once U7 can make the standing eligible, these panels would proceed to build. The builders then refuse through `refuseLoadReferenceOutput` (resultExportAdapter.ts:58, :172; StressNeutralExportPanel.tsx:489), so the result still fails closed, but with a thrown finding rather than the gate's reason.
- **Remedy:** a U7 check, and a line in the T6 notice: either gate the live bindings on `loadReferenceOutputRefusal(result)`, or accept the throw and show the shared reason.

## For ROOT to rule

1. **F1 (N-1):**
   - accept as a declared parity difference (TS need not match);
   - decide whether to pin it now: add the "invalid, no invocation" case to the shared file in the repair round, or record it in U6f's parity summary.
2. **SF-1:** already routed to I67's repair round. Its scope: the member, the null member and token rows.
3. **N-2:** add it to U7's preconditions (bind TS successor standing to the native registration or model, or pin the consumers).
4. **N-3, N-4, N-5:** optional, for the repair round, U7 and the T6 notice respectively.

## Limits

- **No native witness.** As planned (F-1, D-U6-3), registration is exercised through mocked IPC with PP's pinned bytes.
- **Post-U7 behaviour** is exercised only through a test-only reader wrapper that sets `numerical_eligible`. The flag is untouched.
- **Rust parity** rests on reading `u6a_shared_carrier_cases_rust` and ROOT's U6a run. I ran no Cargo (brief).
- **The sweep's inputs** are the committed JSON under the six named directories. Envelopes built in test code are covered by the full suites, not by the sweep.
- **The merge preview** is a scratch overlay, not ROOT's merge.
- **NUM HEAD moved** during the review (to `ac5b318c5b`; records only). No basis file read here changed.

## Records

`_run_records/` (placeholder paths only; SHA256SUMS covers this folder):
- `runtime.txt`, `basis.txt`;
- `suites/`: run script and per-lane summaries;
- `sweep/`: sweep test, runner, comparer, outputs;
- `parity/`: Python harnesses, extra cases, outputs, CLI hashes;
- `probes/`: two probe files, runner, outputs and logs, schema check, import-cycle check, TS-built records;
- `mutants/`: runner, results, log;
- `s1_commit_lines.txt` and `s1_proposed_lines.txt`.

**The import cycle (I67 F9) is confirmed benign.** The new strongly connected component {numericalResultQuality, retainedPrecision, retainedPrecisionStanding} has no top-level reads across it. knownSemanticLimitations' top-level read of the successor id is outside the cycle, so ESM evaluates it after the cycle (`probes/import_cycles.json`).
