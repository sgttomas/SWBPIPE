# RETURN — T1_WAVE2_REVIEW (independent, non-author review)

- **Role:** TASK (Type 2), fresh-context reviewer. I wrote none of the reviewed bytes and delegated nothing. I report to the T1 load-state WORKING_ITEMS manager.
- **Brief:** `TASK_BRIEFS/T1_WAVE2_REVIEW.md`, with `_T1_COMMON.md` (Wave 2), root `AGENTS.md` and `agents/AGENT_TASK.md`.
- **Candidate:** branch `codex/piping-load-states-20260925` at `b0324db9d`. Reviewed: `a9528b2e1` (T0R merge obligations), `c5ec4dcba` (WP2 native), `61fb7b219` (WP4 headless), `64711fd94` (WP2 desktop readers).
- **Method:** per ROOT's coordination instruction, every check ran on a scratch `git archive` of `b0324db9d` (`projects/chirality-piping`, with `execution/` excluded). Code was read from that archive or with `git show b0324db9d:<path>`, never from the live worktree, which a WP3 TASK is editing. The wasm engine output (gitignored) was copied into the archive.
- **Paths:** all relative to `projects/chirality-piping/`. Logs are in `_run_records/`, with machine paths replaced by `<SCRATCH>`.
- **Git:** none. I wrote nothing outside this folder.

## Verdict: FINDINGS

Nothing blocks. Explicit items 1–4 are all CLEAR, and §13 is confirmed. There is one SHOULD-FIX, which is test strength only (F1). F2–F4 are NOTEs.

## Findings

| # | Severity | Location | Concrete failure scenario | Suggested repair |
|---|---|---|---|---|
| F1 | SHOULD-FIX | `core/runner/headless/src/load_reference_route_tests.rs:556-563` (WP4, `joined_actual_solve_…`) | Ruling §11 asks the test to show that reparsed receipt bytes cannot produce a document. The test calls `build_result_export_document(…, &reparsed)` and asserts `is_err()`. But that function always returns `Err` (`result_envelope_binding.rs:230-233`): it is the typed legacy path. So the assertion would still pass if a reparsed joined receipt could mint a document through `build_result_export_document_with_evidence`. The pattern is inherited from the source-block test (`result_envelope_binding.rs:674-677`). The property itself does hold: standing is `needs_recompute` (asserted at :527-544), `result_envelope_document` is `None`, and the unavailability text is exact (:546-551). Only this particular assertion is empty. | This is an in-crate test, so it can do what the manager's probe does. Build a digest-consistent `QualifiedPreviewEvidence` for the joined raw (actual invocation, payload, mechanics, runner and request digests), call `build_result_export_document_with_evidence` with the reparsed mechanics, and assert `CURRENT_NUMERICAL_INTEGRITY_NEEDS_RECOMPUTE`. Or reword the comment so it claims only "no public bytes-to-document path". |
| F2 | NOTE | `apps/desktop/src/features/results/loadReferenceOutputRefusal.test.tsx:131-165` (manager's §14 invariance tests) | `perturbed` changes only numbers and strings. It leaves booleans, `null`, array lengths and key sets as they are. So a future leak of a boolean result flag or a count, such as `result.results.length` or `diagnostics.length`, into the geometry, diff or ledger packets would pass the test. Two further gaps: the ledger and diff invariance runs use `proposal: null`, and the review-geometry case does not assert that the download link is present. By code reading, nothing leaks today: `ReviewGeometryPanel.tsx:143-169` reads only `result.run_id`, `run.run_id` and `run.model_state_ref`; `DiffPreviewPanel.tsx:165-166`, and `OperationLedgerPanel.tsx:162-163`, `:236` and `:306`, read only `run_id` and `model_state_ref`. The proposal path is the native `sample_agent_proposal`, built from the result (`src-tauri/src/lib.rs:1851+`). It carries only a result-row or diagnostic id, which §14 treats as an identifier. | Make `perturbed` flip booleans and change array lengths, for example by appending an invented element. Add one invariance case with a proposal built from each result. Assert `review-geometry-export-link` is present. |
| F3 | NOTE | `apps/desktop/src/features/results/loadReferenceOutputRefusal.test.tsx:182-183`; `resultExportAdapter.ts:58` | The test expects `deriveResultDocument(…)` to reject, but gives no message. The call fails first on `SOURCE_CARRIER_HASH_MISMATCH`, so the refusal in the derive step is never reached by a test. It is present in the code, and the build and validate refusal points are proven with the exact text. | Build a derive input whose carrier checksum matches the result digest, then assert `LOAD_REFERENCE_OUTPUT_REFUSAL`. |
| F4 | NOTE | host | Two `rev` processes (not mine, and not started by any reviewed code) have run at about 96% CPU each for about 4 h. That takes about 2 of the 4 cores for the whole session, and the load average was 6–7. This is a plausible main contributor to the 30 s vitest timeouts that ROOT wants root-caused (§14). | ROOT or the manager checks those processes before WP7's quiet-host sweep. I did not touch them. |

## Explicit item 1 — the 32 surviving mutants: CLEAR

The operators come from `run_mutants.py`: `throw X` becomes `void X`, and `blocks(c, N)` becomes `blocks(true, N)`. The survivor list is from `mutation_results.json` at `b0324db9d`: R 7, J 1, L 24.

**All 8 reader survivors: each is unreachable or equivalent. Verified line by line against `loadReferenceEvidence.ts` and `loadReferenceSourceEvidence.ts` at `b0324db9d`.**

| Site | Verdict and reason |
|---|---|
| `loadReferenceEvidence.ts:505` `RECORD_CASE_UNRESOLVED` (R12) | Unreachable. `validateRecord` runs only in the S10 loop (`:337`). **Solved case:** S9 (`:333`) requires `setEq(recordIds, qualityIds)` and `setEq(caseIds, qualityIds)`. `recordIds` comes from `text(record.load_case_id)` (S8), and S7 `CASE_DUPLICATE` makes case ids unique, so every record id is some case id. **Unsolved case:** S9 requires `records` to be empty, so the loop never runs. **Transport route** (`raw=false`): it is always treated as solved. |
| `:343` `RECORD_CASE_UNRESOLVED` (S10b, joined) | Unreachable. The same condition, and the S10 loop (which includes `:505`) has already run on every record. |
| `:513` `MEMBER_MATERIAL_COVERAGE` (find fails) | Unreachable. `memberIds` are non-empty strings (`text()`), and `:510` maps every non-string `pipe_id` to `""`. So set equality guarantees a material whose string `pipe_id` equals each member's. `same()` on two strings is strict equality. |
| `:531` `MEMBER_CONTRIBUTION_COVERAGE` and `:538` `SUPPORT_CONTRIBUTION_COVERAGE` (find fails) | Unreachable. `ofKind` collects string `source_id`s of that owner kind. Set equality at `:527` and `:535` against `member_state:<pipe>` and `support_state:<support>:<dof>` (built from `text()` values) guarantees a contribution with that exact `source_id`, so `find` cannot return `undefined`. |
| `:368` `REGION_MATERIAL_BINDING` (member not found) | Reachable, but equivalent. With the mutant, `without(undefined, …)` returns `undefined`, `same(object, undefined)` is false, and `:369` raises the same code. The recorded case `REGION_MATERIAL_UNBOUND_PIPE` reaches it, with the same outcome either way. |
| `:226` `SOURCE_LOAD_REFERENCE_TABLE_IDENTITY` and `loadReferenceSourceEvidence.ts:67` `…SOURCE_TABLE_IDENTITY` | Unreachable. Each runs only after the byte sha256 equals the pinned sha (`44bc41c0…`, `d1628194…`). The pinned bytes carry the right identity, profile and policy, and are asserted by the parity test's table cases. Only a sha256 collision could reach it. |

**Ledger survivors: I sampled at least one per stated reason (10 of the 24), and all hold.** The reference is the pinned `schemas/physics_source_recovery.schema.json`. The closed schema is applied twice before the ledger runs:

- `receipt()` (J2, `JOIN_RECEIPT_SHAPE`, on the received receipt with only the policy substituted);
- `validateSourceBlocksComposite` (`RECEIPT_SHAPE`, on the projection). `project()` changes only the policy and the hashes.

`case.rows`, `projections` and `supports` all `$ref` the closed `row`, `projection` and `support` definitions, and every key the checks read is `required`.

| Reason | Sampled | Why no input reaches it |
|---|---|---|
| Criterion bound | `PROJECTION_CRITERION` `:337` | Schema `projection.relative_limit` is `{"const": 1e-09}`, and `relative_error_bound` has `maximum 1e-09` (both required). The check is the same predicate. |
| Recipe enum | `DERIVED_RECIPE` `:367`, `SUPPORT_NORM_INPUTS` `:359`, `STRESS_ACTION_BINDING` `:390`, `SUMMARY_SUBTOTAL_RANGE` `:422` | `row.recipe_id` is `anyOf[enum{translation_norm_scaled_v1, retained_source_straight_stress_v1, retained_source_endpoint_normal_max_v1, support_force_norm_scaled_checked_v1, support_moment_norm_scaled_checked_v1}, null]`. `DERIVED_ROW` requires a non-null recipe. The four composite recipes are sent to `validatePhysicsSourceDerived` (`:629`), so `derived()` only ever sees `translation_norm_scaled_v1`. The branches for `support_force_norm_scaled_v1`, `straight_open_stress_v1`, `reviewed_stress_summary_v1`, `section_property_from_source_v1` and the else-branch are dead. So is all of `stressObservation` and `stressSummaryObservation`, since those are reached only through two of the dead recipes. |
| `outcome` const | `FAILED_OUTCOME` `:590`, `FAILURE_CATEGORY` `:593` | `case.outcome` is `{"const": "qualified"}` and `case.failure` is `{"type": "null"}`, so the `else` branch at `:588` never runs. |
| Closed receipt schema (component cap) | `SPRING_ATTRIBUTION_DUPLICATE` `:457` | `support.components` has min and max 6, and `SIX_SUPPORT_COMPONENTS` gives set equality with 6 names, so each slot appears once. `SUPPORT_ACTION_OWNER` requires `dof mod 6 == slot` and `source_id == support_id`. Within one component, `SUPPORT_TERM_DUPLICATE` (unique on kind, source and dof) catches a repeated `ground_spring`. Across supports, the `source_id`s differ (`SUPPORT_IDENTITY` and `SUPPORT_COVERAGE`). So `(source_id, dof)` can never repeat. |
| Pre-emption by an earlier check | `DERIVED_NORM_RANGE` `:375`, `UNSELECTED_SUPPORT_CERTIFICATE` `:660` | `:375`: every value is already finite, through pre-pass S2 `finiteTree` (`NUMBER_INVALID`, over the whole source, including in-memory NaN or Infinity) and the ledger `VALUE_NONFINITE` (`bitsHex` per row). `:660`: `success` is always true (outcome const), so the branch needs `method !== EXACT`. `QUALIFIED_OUTCOME` rules out `null`, and `ORDINARY_SELECTION` (`:584`) has already required `supports` to be empty. The recorded case gives `SOURCE_BLOCKS_ORDINARY_SELECTION`. |

- **Reachable survivors found:** none. So no killing input is owed.
- **The 11 pre-empted ledger cases** in `loadReferenceSourceLedger.cases.json` also hit exactly the earlier codes listed above, and they pass on the candidate (109/109).

## Explicit item 2 — §13 mode custody: CONFIRMED (no blocking asymmetry)

- **The binding treats every identity the same.** `build_result_export_document_with_evidence` (`core/runner/headless/src/result_envelope_binding.rs:236-269`) checks, for every identity, only:
  - the payload, invocation, mechanics, runner and request digests;
  - `actual_invocation.request == solve_payload`;
  - that `solver_mode ∈ {sparse_interactive, dense_scrutiny}` and the invocation has exactly 2 keys;
  - the run id and the model identity;
  - then `numerical_use_standing_with_context(source, requested, Some(&actual_invocation))`, then `derive_document`.
- **Only source-block routes read the invocation.** In `numerical_use_standing_with_context` (`core/reporting/result_export/src/semantic_contract.rs:457-548`), only the source-blocks-1 and physics-source-1 branches (`:481-499`) consume `actual_invocation`: `source_blocks.rs:748-750` checks invocation shape and mode, and `:916` checks `requested_mode == solver_mode`. physics-1, preview-physics-1 and load-reference-1 all fall through to the same generic `numerical_quality` checks (`:500-547`), which never read the invocation. load-reference-source-1 returns `needs_recompute` before any of this (`:474`).
- **`derive_document` has no per-identity mode check.** It treats `PHYSICS_ID | PHYSICS_SOURCE_ID | LOAD_REFERENCE_ID | LOAD_REFERENCE_SOURCE_ID | PREVIEW_PHYSICS_ID` alike: it copies `contract_evidence` (`derivative.rs:96-105`). `solver_mode` appears there only as a row category (`:252`).
- **No binding exists for physics-1 or preview-physics-1 that load-reference-1 lacks.** The manager's probe result (all six cases accepted) is what this code predicts. So the property predates T1, and the T6 routing stands.
- **The WP4 pin** (`load_reference_route_tests.rs:398-411`) documents the behaviour as observed and not yet flipped.
- **I did not re-execute the probe in Rust** (see Limits). The confirmation is from source reading of the candidate.
- **One nuance for T6.** load-reference-1 also publishes `load_reference_states[].solve.requested_mode`, and the reader checks that every record agrees (`SOLVE_CONSISTENCY`), but nothing binds it to the invocation. A T6 binding could use it. physics-1 has no such field. This is extra evidence that load-reference-1 carries, not a binding it lacks.

## Explicit item 3 — desktop output classification (§12, §14): CLEAR

- **How I enumerated surfaces.** I listed every non-test desktop file with an output primitive: `ControlledExportLink`, `href=`, `download`, `Blob(`, clipboard, `window.open`, `print`, `invoke(…)`, `saveReportPackage` and `renderCalculationReport` (52 files). I then followed every native command that carries a result.
- **Refused surfaces: the gate encloses every download link.** A per-file count of links and buttons inside `<LoadReferenceOutputGate>` against the file total gives, for all 17 plain-gated panels (PCF, CAEPIPE .mbf, CAEPIPE external harness, export adapter SDK, adapter framework, external prover, missing-data, design workspace, rule-check completeness, report lint, solve job, headless runner, local FEA handoff, native package, handoff package, export review, report packet), links inside = links total.
  - SolvePanel's 4 buttons outside the gate are the solver-mode and run/cancel controls, which produce no output.
  - In RenderedReportPanel, the render button is gated and `onRender` returns early (`:69`). Its links exist only after a render.
  - "Save Report Package…" stays enabled, but `buildReportPackageRequest` throws T0R's `REPORT-PACKAGE-FRESH-RESULT-UNAVAILABLE: N_REPORT` (`reportPackageRequest.ts:223-226`). `FRESH_SEMANTIC_CONTRACT_IDS` holds both load/reference identities (`knownSemanticLimitations.ts:10-17`).
- **Stress-neutral export** is refused in the live binding (`:86`), the panel (`:207`), the builder (`:489`), the validator (`:620`) and the table path (`:1059`).
- **Result export** is refused in the live binding (`ResultExportPanel.tsx:14`), in derive, validate and build (`resultExportAdapter.ts:58`, `:118`, `:172`), and through `save_local_result_json` (only reachable through `ControlledExportLink` on a built packet). Result data truly flows into each refused output, as the RETURN §5 table cites; I spot-checked the PCF, rendered report, stress-neutral and result-export entries.
- **Left unchanged, and correctly so.**
  - Each of these is mounted with `model` only (`App.tsx:658-663`, `:1448-1453`, `:562`, `:1011`), or additionally with project summary, intent or proposal props that carry no result (`:929-950`, `:486`): Build readiness, Validation evidence, Telemetry, Secret library, Threat model, Accessibility, Editor contract, Redaction, Project storage audit (a count only), Project validation, and Offline proposal intake.
  - Review geometry, operation diff preview and operation review ledger are identifier-only by code reading (see F2).
  - `run_rule_checks` (RuleCheckRun) and `sample_agent_proposal` are local analyses. They write no file and send no external request.
  - Project save is persistence, which brief item 4 requires.
- **No other surface lets load/reference-state result data out.**
- **Wording.** The one reason (`loadReferenceOutputAvailability.ts:17-19`) says the output "is not yet available on the desktop; it is routed to T6. The result remains readable here; this is not a finding about the result." No refusal path calls the result invalid or unsupported.
- **Tests:** `loadReferenceOutputRefusal.test.tsx` 64/64 on the candidate.

## Explicit item 4 — validation path unaffected: CLEAR

- **The desktop commit touches nothing outside the desktop source and the run records.**
  - `git diff --stat 61fb7b219 64711fd94 -- core/runner/headless tools/validation/qualification_load_reference.py core/handoff ':(glob)**/*.py'` lists only 5 run-record scripts under `execution/…/T1_WP2_DESKTOP_READERS/_run_records/`. None is product, harness or packager code.
  - `git diff --name-only 61fb7b219 64711fd94`, filtered to paths outside `apps/desktop/src/` and `execution/`, is empty.
  - `64711fd94..b0324db9d` adds only this review's brief.
- **The validation path is untouched after the merge.** `git log a9528b2e1..b0324db9d -- core/handoff/stress_neutral tools/validation/qualification_load_reference.py core/runner/headless/src/main.rs` is empty. `stress_neutral` and `qualification_load_reference.py` are unchanged across the whole post-merge T1 range. WP4's headless changes are test-only: a `#[cfg(test)]` module, the CLI test and a Python consumer test.
- **Python still behaves the same on the candidate.** Readers, joined readers, headless-artifact consumer, qualification harness and stress-neutral package suites: 524 passed and 17 skipped. The skips are the artifact gates, which need the Rust lanes' env vars.
- **One failure is my setup error, not a finding.** `test_qualification_load_reference.py::PinTests::test_pinned_identities_match_recorded_bytes` shells out to `git show`, and the archive has no `.git`.

## General items (proportionate)

5. **Readers.**
   - **Parity re-run on the candidate.** I ran the parity test with `LOAD_REFERENCE_PARITY_OUT`, and `LOAD_REFERENCE_PARITY_COMPARE` pointed at the committed Rust and Python logs. It passed, and the regenerated `ts_outcomes.json` and `ts_source_outcomes.json` are **byte-identical** to the committed ones. Re-running `summarize_parity.py` gives lr 438/438 and lrs 282/282 (accept/refuse classes).
   - **Codes are compared too.** The vitest cases additionally compare codes against Python, with a documented `TS_CODES` exception set for binary64 representation.
   - **Dispatch.** Each new identity is tied to its one profile and requires an object `contract_evidence`. load-reference-1 with `source_block_recovery`, or load-reference-source-1 without the closed receipt shape, dispatches `unsupported` (`numericalResultQuality.ts:60-86`). A relabelled physics-1 header that is also re-profiled dispatches to the load-reference-1 reader and is refused there (`EVIDENCE_SHAPE`).
   - **Standing, TS.** For load-reference-1: reader, then T0R's generic standing. For load-reference-source-1: registered validation, then the declared early `needs_recompute`. T0R's `standingReason` is null for that route.
   - **Standing, Rust and Python** follow the checkpoint order: `semantic_contract.rs:462-475`; `compatibility.py:327-339`.
6. **Lossless types.** `loadReferenceRoundTrip.test.ts` 8/8 on the candidate. I did not re-derive the canonical wasm comparison.
7. **Saved results.** `loadReference.resultsSessionState.test.ts` shows Current is lost after a `reference_configurations` fit edit or an `analysis_state` boundary-motion edit, and a byte copy has no registration. 7/7 pass.
8. **Native persistence (read only; not built).** In `c5ec4dcba`, 0.4.0 is `current` with `migrated_document: None`, and 0.4.1, 0.5.0, 0.3.1 and 1.0.0 stay `newer_than_supported`. Pre-0.4 documents carrying 0.4.0 keys are retained as authored, and 0.1.0 walks only its no-op. The browser mirror (`projectService.ts`) matches for 0.4.0. The browser's 0.3.0 divergence predates T1 and is routed (§14).
9. **WP4.**
   - The artifact lanes write the actual `payload`, `invocation`, `raw` and `document` values of the solve under test (`:457-466`, `:566-575`).
   - Blocked 0.4.0 envelopes keep load-reference-1 through `blocked_envelope`, `mechanics_producer_for_model` and `formulation_basis_for_model` (`product_physics/src/lib.rs:954-975`, `:11330-11380`). A non-exact 0.4.0 document is blocked by `PRESSURE_CONTRACT_UNSUPPORTED` (`pressure_runtime.rs:115-123`).
   - See F1.
10. **Merge obligations.**
    - The standing order is correct in all three languages (above).
    - `preview: None` is in both T1 early returns (`lib.rs:2224`, member section missing; `:2327`, prescribed DOF).
    - The fresh sets hold 6 identities in TS and Rust (`FRESH_IDENTITIES`, `semantic_contract.rs:408-409`).
    - Schema branch pins are identity-keyed. `test_load_reference_schema.py`, `test_load_reference_source_schema.py` and `test_source_block_schema_contract.py`, plus the preview-physics consumer suites, give 927 passed and 11 skipped (the source-block artifact gates) on the candidate.
    - I found no path by which a 0.4.0 envelope carries a non-T1 identity.

## Checks run (all on the `b0324db9d` archive)

| Check | Result |
|---|---|
| vitest: `loadReferenceReaders`, `loadReferenceReaderCases`, `loadReferenceSourceLedger`, `loadReferenceOutputRefusal` | 4 files, 231/231 (`_run_records/vitest_readers_ledger_refusal.log`) |
| vitest: parity (OUT and COMPARE), session, round trip, AnalysisRun | 4 files, 403/403; TS outcome logs byte-identical (`_run_records/vitest_parity_session_roundtrip_analysisrun.log`) |
| pytest: LR/LRS readers, headless artifacts, qualification harness, stress-neutral package | 524 passed, 17 skipped, 1 setup error (no `.git`) |
| pytest: LR/LRS schema pins, source-block schema contract, preview-physics consumer suites | 927 passed, 11 skipped (`_run_records/pytest_schema_preview.log`) |
| Survivor analysis | 8/8 reader, and 10/24 ledger sampled across all 5 reasons: none reachable |

## Timing comparison (5 timed-out cases): not possible

- **Why.** The host was not quiet: load average 6–7 on 4 cores throughout, with another TASK's vitest workers and the two `rev` processes (F4). So I did not compare base against candidate timings.
- **What I measured instead.** On the candidate I measured the per-call cost of the only code T1 adds on those paths, the shared `loadReferenceOutputRefusal` / `sourceContract`:
  - about 4–14 ms per call on the largest physics-source raws (`mixed-dense_scrutiny`, `fields-dense_scrutiny`), measured under that load (unmemoized receipt-shape check);
  - effectively zero on non-receipt routes, and on a null result.
- **physicsSourceIntegration.** T1 adds at most 4 such calls per case (stress-neutral build, validate and table path), which is well under 0.1 s against cases of about 19 s. T1 is therefore implausible as the cause there.
- **App.test.** Each gated panel adds one call per render. The App render test does not use a receipt-carrying result, as far as I can see, so the added cost is negligible, but I did not measure render counts.
- **Base history.** The base had the same timeout class in the same files (WP2 desktop RETURN §3).

## Limits

- **No Rust or cargo run.** A build from the scratch path would rebuild every workspace crate into the shared target (the path-dependent metadata changes). With free disk at about 9.2 GB against the 8 GB floor, I judged that unsafe. So I did not re-execute the §13 probe, the headless tests, `result_export` or `src-tauri`. Item 2 and general items 8–10 rest on source reading plus the committed logs.
- **Not run:** the full desktop vitest suite, the build, e2e, and VP-STATIC.
- **Scope.** I sampled 10 of the 24 ledger survivors. Of the 14 not sampled, 11 fall under the recipe-enum reason already verified, and 3 are pre-empted cases recorded in the ledger case file.
- **Symlinks.** I created one: `<SCRATCH>/cand/projects/chirality-piping/node_modules`, pointing at the sibling engine worktree's `node_modules`. I removed it before returning. I created no link in the live worktree. The `apps/desktop/node_modules` currently present in the worktree belongs to the WP3 TASK, not to me.
- **Scratch.** The scratch archive is deleted.
