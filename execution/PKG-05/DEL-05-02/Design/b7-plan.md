# B7: one reader, the v3 retained-source join and the legacy removals

This is the governing plan for B7. Two of its legacy items were done ahead of it, in the pre-B7 batch: E13 and the Rust reader's namespace rename (see B7a). The rest is not implemented yet. It lands in this order:

1. **Unit R**, the one-reader consolidation, ahead of B7: R1 (the retained reader), then R2 (the base-route readers).
2. **The join → B7a → B7b**, as stacked PRs. Each is reviewed; the three merge to main as one batch. Until that batch merges, main keeps the named "retained-source recovery not joined" diagnostic on v3.

This file lives with DEL-05-02 because DEL-05-02 owns the result readers and their conformance corpora (`retained-combinations.md`; `deliverable.yaml` points to every reader source in all three languages; `compatibility.py`'s record builder stays with DEL-14-02). The retained-combinations contract stays in its own file.

## Decisions applied

The owner, 2026-10-10:

- **Direction:** "nothing needs to be preserved for backwards compatibility. I want the best solver engine and I want clean code … if the right solution is to rewrite something, then do it, don't hold onto legacy code that requires workarounds or isn't consistent with the newer code and methods."
- **One current version per format:** "as you recommended, no-compatibility principle carries."
  - Model documents, analysis-run records and stress-neutral exports each keep only their current version.
  - Older versions are refused by name ("re-author" or "re-export"). No migration code is kept.
  - This reverses the 2026-10-08 decision that kept model formats 0.1.0/0.2.0 as the pressure-free namespace. The end state is one model format that expresses models with or without pressure.
- **One product reader:** "move to one product reader in Rust." Asked whether this covers the base-route readers, the owner answered "All readers.": for every result reader (retained, preview-physics, physics, physics-source, load-reference, source-blocks and standing), Rust is the only product reader, native and wasm on the desktop; the TypeScript copies are deleted; Python is kept only as an independent test verifier that shares no code with the producer.
- **DEC-028:** "Close: one version." The `.opsproj` package reader accepts only the current package version and refuses older versions by name, with no migration. B7 carries it.

HELP_HUMAN, 2026-10-10, on this plan:
- **The principle carries to result formats.** Under the owner's "no-compatibility principle carries", result exports (0.1/0.2 beside 0.3.0) and the mechanics-result 0.1.0 carrier keep only their current version. B7a removes them and their reviewed `semantic_contract_v0_2.json` table.
- **Inventory items ride B7.** B1, B3, B4/D1, B5, B6 and D8 ride B7a, or B7b where they depend on v2's deletion or the join: they move tables or the corpus, and the owner's principle favours removal, so re-registration and regeneration are paid once. B4/D1 is ruled separately (next item).
- **B4/D1: option (b), gated on evidence.** Retire the preview-route source blocks in favour of W1, after a census. The fallback order is HELP_HUMAN's (B7a's first rows).
- **0.5.0 is accepted** as the current model format ("Current model format").
- **R-b′ for all members** leaves B7. T4's review found straights masked the same way: a 10 × 1 m skew cantilever publishes Passed with an exact statics error of 6e5–7e5 × RD(1e-9)·q, silenced by `q ≤ 2^10·B`. HELP_HUMAN ruled on 2026-10-10 that this is a correctness defect fixed now, not in B7. One T4 unit, "K-D5 member-relative bound", fixes both parts with one re-registration: a member-relative forward-error bound on Δ for every member, and R-b′ as one predicate for all members. T3 reviews the K-D5 interface. B7 rebases on that unit.
- **Unit R carries the exact-bytes fix** (below).

HELP_HUMAN, 2026-10-10, on the v3 retained-source join: D1, D2 (inside B7, no constant-substitution shim), D3, D4 (a), D5, D6 and D7, stated below where they apply.

**Terms.**
- *Reviewed inputs* are `REVIEWED_INPUTS` in `core/product_physics/src/build_identity.rs` (17 files) and their digests in the registered profile (`retained_memory.rs`).
- *Priced* means a layout or TEXT entry of the registered memory profile.
- *The corpus* is:
  - `fixtures/results/retained_precision_cases.json` (07m/07n: 26 bases, 534 mutations, 78 must-pass);
  - `retained_precision_cases_07o.json` (19 bases, 69 mutations, 19 must-pass).
- *Re-registration* follows **D3**. It is approved on the join head and on the B7 heads if M stays 10.5 GiB (11,274,289,152 bytes) and the 5 % text budget holds. Otherwise it returns to HELP_HUMAN for a ruling. Any change of M or of the budget model is decision 22, which the owner holds.

## Unit R: one product reader

Unit R has two stages, both ahead of the join: **R1**, the retained reader (this section down to R2), and **R2**, the base-route readers. Each gates the TypeScript deletion on its own equivalence test.

**The arrangement (R1).** One product reader replaces the three retained readers held at parity:
- **Rust, the only product reader.** `core/reporting/result_export/src/retained_precision.rs` reads retained-precision successors natively, and compiled to wasm for the desktop.
- **TypeScript, deleted.** `apps/desktop/src/features/results/retainedPrecision.ts` goes.
- **Python, a test-only verifier.** `core/analysis_runs/retained_precision.py` becomes `tests/verifiers/retained_precision.py`, an independent verifier that only tests use, never a product path.

After R1 and R2, B7's table, schema and reader changes are made in two places, not three: the Rust reader and the Python verifier.

**Bytes and registration.** R moves no reviewed input, priced layout, table, successor byte or corpus base, so it needs no re-registration. If it needs a Rust-reader change that moves any of them, it joins the batch and re-registers under D3. It does move desktop IPC and export bytes (the exact-bytes fix below); no producer byte moves.

**Exact bytes to the desktop (HELP_HUMAN, 2026-10-10).**
- **The defect.** The desktop's solve commands re-encode the producer's envelope. `solve_preview_mechanics_with_mode` in `apps/desktop/src-tauri/src/lib.rs` returns `serde_json::to_value(envelope)`, and the desktop's `serde_json` has no `preserve_order`, so the value carries sorted keys. It serves `run_preview_mechanics`, `run_preview_mechanics_with_solver_mode`, and the job path (`start_preview_mechanics_job`, `start_preview_mechanics_job_with_solver_mode` → `publish_solve_outcome` → `poll_preview_mechanics_job`). The webview then re-serializes: `ResultExportPanel.tsx`'s `jsonDataHref` writes `JSON.stringify` of the parsed object, which `save_local_result_json` saves. So IPC and exported bytes differ from the pinned producer bytes, and the hash identity of anything exported from the desktop is at risk.
- **The fix.** The native side hands over the producer's exact serialized bytes, and the Rust reader in the desktop reads those bytes. Exports write the received bytes, never a re-serialization of a parsed object.
- **What moves.** Desktop IPC payloads and exported result bytes, which become the producer's bytes. The IPC shape tests move with them (`services/previewService.test.ts` replays, `features/result-export/nativeResultSave.test.ts`, `App.test.tsx`, and the native tests in `src-tauri/src/lib.rs`). No producer byte moves.
- **As built (R1).** `run_preview_mechanics`, `run_preview_mechanics_with_solver_mode` and a completed job's `result` return the producer's bytes (`serde_json::to_string` of the envelope) as a string. `previewService` reads the result from them once and keeps them beside it (`receivedResultBytes`, while the result is unchanged); a retained successor's registration reads those bytes. For the G10 milestone model, `solves_hand_the_desktop_the_producers_exact_bytes` (`src-tauri`) and `services/exactResultBytes.test.ts` check them against PP's pins, through the committed copies `fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.desktop_result_{mode}.json`; the native save writes received bytes unchanged.
- **The export panel (owner, 2026-10-11: "Keep envelope, hash by value.").** The result-export panel's download stays DEL-08-04's schema-first result envelope, which the desktop derives from the parsed result (`deriveResultDocument`); its bytes are not the producer's. The envelope identifies each member it copies (the raw source, its rows, the model, the request) by the sha256 of the member's canonical JSON value (`openpipestress_jcs_ijson_v1`), never by received bytes, in the desktop (`resultExportAdapter.ts`, `resultDigest`) and in Rust (`derivative::digest`). The producer's exact bytes stay on the native save path. DEL-08-04's commitment is unchanged.

**Wasm.**
- **Where the bindings go.** They are added to the desktop's existing wasm engine crate, `core/model_operations/operation_applier` (`npm run build:wasm`). That crate already resolves `open_pipe_stress_result_export` through product_physics, which re-exports it as `result_export`.
  - result_export gains no dependency, so PP's `Cargo.lock`, a reviewed input, does not move.
- **What they expose.** `retained_precision::validate` and `validate_transport_metadata`, JSON in. Out comes the `Validation`, or the error's gate, code and detail.
  - As built: `validate_retained_precision_json(source, invocation?)` and `validate_retained_precision_transport_json(source)` return `retained_precision::outcome_json`: `{"ok":true,"validation":{…}}` with each row's binary64 words as 16 hex digits, or `{"ok":false,"error":{"gate","code","detail"}}`. Text that is not JSON throws `WASM-ENGINE-INPUT-JSON-INVALID`. Their native twins are `retained_precision_outcome_json` and `retained_precision_transport_outcome_json`.
- **How the desktop loads them.** Through `services/wasmEngine/loadWasmEngine`.

**TypeScript call sites that change** (under `apps/desktop/src/`):
- These move to the wasm binding:
  - `features/results/retainedPrecisionStanding.ts`;
  - `retainedPrecisionDisclosure.ts`;
  - `numericalResultQuality.ts` (`validateRetainedPrecisionTransport`, `RETAINED_METHOD`);
  - `HistoricalRunContext.tsx`;
  - `services/analysisRunCompatibility.ts`;
  - `features/stress-neutral/StressNeutralExportPanel.tsx`.
- `knownSemanticLimitations.ts` and the modules above import `decodeBinary64`, `binary64Bits` and the reader's types. These move to a small module that holds no validation.
- As built: the binding is `features/results/retainedPrecisionReader.ts`, and the module without validation is `retainedPrecisionTypes.ts`. A parsed value is written to the reader as JSON text that keeps its negative zeros.

**Vitest tests that change.**
- **Deleted:** `features/results/retainedPrecision.test.ts`, the reader-internal tests (`phi512`, `eHat`, `stopFeasible`, `nativeSchedule`, `accountingRules`, …). The Rust tests and the verifier cover these. Its one carrier test (the exact route's dispatch, binding, standing, table and notices) moves to `retainedPhysicsRoute.test.ts`, on lane P's m3x successor.
- **Re-pointed at the wasm reader:**
  - `retainedPrecisionIntegration.test.tsx`;
  - `retainedPrecisionOutputRefusal.test.tsx`;
  - `outputPolicy.test.ts`;
  - `result-export/retainedPrecisionResultExport.test.tsx`;
  - `stress-neutral/retainedPrecisionStressNeutral.test.tsx`;
  - `stress-neutral/StressNeutralExportPanel.test.tsx`;
  - `services/retainedPrecisionAnalysisRun.test.ts`.

**The Python verifier.**
- **Location.** `tests/verifiers/retained_precision.py`, in the test-only package `tests/verifiers/`. `tests/test_verifier_independence.py` asserts that no module outside `tests/` imports it, by package or by path.
- **Product callers.** `core/analysis_runs/compatibility.py`'s `_retained_validation` and `_retained_transport` no longer import it.
  - Python product code that meets a retained successor takes the Rust reader's verdict through `core/analysis_runs/rust_reader.py`. It runs `openpipestress-runner read retained-precision` (`validate`; `{"source", "invocation"?}` in) or `read retained-precision-transport` (`validate_transport_metadata`; `{"source"}` in), with the runner at `OPENPIPESTRESS_RUNNER_BIN`. Out comes the reader's own `retained_precision::outcome_json`, the wasm bindings' shape, as `outcome` in the envelope `{"artifact", "schema_version", "reader", "outcome"}`: `{"ok": true, "validation"}` with exit 0, or `{"ok": false, "error": {gate, code, detail}}` with exit 1; an unusable input exits 2. R2 adds its readers to the same `read` table.
  - It never re-derives a verdict. Every other product path reaches the retained reader only through those two functions: `compatibility.py`'s dispatch, standing, classification summary and rule binding, its analysis-run builder and validator, and the stress-neutral 0.3 packager. The product preview service (whose 0.1.0 wrapper refuses a receipt-carrying source by name, unread) and the qualification tools do not reach it.
  - The product takes Rust's text. A product refusal reads differently only in the declared G7 class, where Rust reports finer base codes: Rust's code on raw reads, and on transport reads the same code with Rust's detail. On the corpus that is its 47 declared entries (23 of them on transport); two test-built statements of the same class (in `test_retained_precision_b3.py` and `test_retained_precision_carriers.py`) read the same way. Every pass or refusal, every refusal outside that class, every passing validation, standing and classification summary, and every analysis-run record built is unchanged.
- **Independence.** It shares no code with the producer and spawns no process.
  - It hashes through `tests/verifiers/canonical_json.py` and converts units through `tests/verifiers/units.py`, in place of the subprocess bridges `core/serialization/canonical_json/adapter.py` and `core/units/adapter.py`.
  - These are checked against the crates' committed vectors, read from their files: `fixtures/canonical_hash/cases.json` and `fixtures/results/precision_transport_v0_3.json` for canonical JSON, and, for units, `core/units/src/lib.rs`'s catalog (every entry's symbol, dimension, factor and offset bits, and each dimension's canonical unit) and its witness tables. The verifier's replay of the producer-solved successors and of the mixed-unit physics-source statements matches the producer's hashes and converted bits.
  - Its base step (G7) and the exact route's G8 materials still run the Python base readers in `core/analysis_runs/`, which R2 moves under `tests/`.

**The shared conformance corpus.** These become the conformance corpus shared by the Rust reader and the Python verifier:
- the 07m/07n/07o corpora;
- `retained_precision_b2_parity_probes.json` and `retained_precision_carrier_cases.json`;
- the successor fixtures `retained_precision_*_successor_*.json`;
- `retained_precision_rv120_b3_inputs.json`.

Both run every entry unmodified. `expected_by_reader` and `expected_unbound_by_reader` lose their `typescript` member.

**What is gated, and how equivalence is shown.** Deleting the TS reader is gated on an equivalence test. It runs on R's head while both readers are still present.
1. **Wasm matches native Rust.** This covers every corpus entry, including its mutation, must-pass, transport (`expected_transport`) and unbound (`expected_unbound`) forms. Through wasm, each gives the same outcome as the native Rust reader: a pass with an identical `Validation`, or the same gate, code and detail.
2. **Wasm matches the TS reader, except where declared.** The exceptions are 47 entries (46 in 07m/07n, 1 in 07o) with 92 expectations, where the TypeScript expectation differs from Rust's: Rust reports finer base codes at G7. As run, 14 more readings differ in the same way (below); they are declared in this class, 106 readings in all. There the desktop takes Rust's outcome, and the change is declared.
3. **Every desktop gate code gives the same outcome through wasm.** These are the codes the standing, disclosure, quality, analysis-run, historical-run, result-export and stress-neutral consumers receive, run in the vitest suites above. The only changes allowed are item 2's.

As run (R1). Item 1: 835 reader inputs (the corpus's 45 bases, 603 mutations and 97 must-pass entries, the 20 B2 probes, RV120's 12 inputs, the 12 successor fixtures and the carrier file's 46 forms), 2,483 readings, all equal to the native outcomes committed in `fixtures/results/retained_precision_reader_outcomes.json` (result_export's `r1_native_reader_outcomes`; `retainedPrecisionWasmEquivalence.test.ts` keeps reading them). Item 2: the 92 corpus expectations (47 entries) differ as stated, and 14 more readings differ in the same way, G7 in both readers with Rust's own code: the unbound readings of two of those entries, which the corpus does not state, and three readings of each of four RV120 probes, whose TypeScript codes the TypeScript reader's own tests pinned. No other outcome differs; G7 details differ on 38 readings. Item 3: the suites above pass with only their imports re-pointed; the exact-bytes change then moves their mocked IPC to bytes.

A difference outside item 2 stops the deletion.

### R2: the base-route readers (owner, 2026-10-10: "All readers.")

**Rust, the only product reader.** In `core/reporting/result_export/src/`: `preview_physics_evidence.rs`, `physics_evidence.rs`, `physics_source.rs`, `load_reference.rs`, `load_reference_source.rs`, `source_blocks.rs`, and the dispatch and standing in `semantic_contract.rs` (`for_source`, `for_source_metadata`, `standing_reason`, `rule_binding_refusal`, `class_binding_refusal`, `numerical_use_standing*`, `classification_summary*`, `signature`, `canonical_metadata`). The desktop calls them through the same wasm engine bindings as R1, so PP's `Cargo.lock` does not move.

**TypeScript readers deleted (as built, stage D)** (under `apps/desktop/src/features/results/`):
- `previewPhysicsEvidence.ts`, `physicsResultEvidence.ts`, `physicsSourceRecovery.ts`, `loadReferenceEvidence.ts`, `loadReferenceSourceEvidence.ts`, `sourceBlockRecovery.ts`, and the test-only pressure-1 reader `pressureContractEvidence.ts`;
- the route dispatch, standing, downgrade guards and table bindings of `numericalResultQuality.ts`, which keeps the route vocabulary (identities, route names, `isRetained*`);
- the table lookups of `resultSemantics.ts`, which keeps the row labels (family, dimension, category) over Rust's signature.

What stays in TypeScript is presentation over the Rust verdict, and the desktop's session state:
- notice texts, panels and labels. `knownSemanticLimitations.ts` keeps the source-blocks-1 notice choice, which goes with B4/D1 (b);
- R1's retained carrier (`retainedPrecisionStanding.ts`) and G4's source-route registrations (`resultStanding.ts`). They hold the captured invocation and read nothing;
- G3's caller-model check in `previewService.ts`;
- the analysis-run record builder's row interpretation (`analysisRunCompatibility.ts`, DEL-14-02, as Python's record builder). It reads the pinned table the Rust dispatch binds.

**Call sites (as built).** Every importer listed below reads through two modules:
- `features/results/resultReader.ts` wraps the four bindings (`readerDispatch`, `readerValidate`, `readerTransport`, `readerRowSemantics`). `sourceContract`, `sourceSemanticBinding`, `currentSemanticContract`, `hasCurrentSourceContract` and `retainedPrecisionDowngrade` present the dispatch;
- `resultStanding.ts` holds `numericalResultStanding`, the registrations, `retainedSourceRecoveryInvocation` and `sourceRecoveryModeMatches`.

The verdicts are synchronous once the engine is loaded, and `main.tsx` loads it before the first render. Without it, every result reads unsupported (`RESULT_READER_UNAVAILABLE`).

The readers read received text:
- previewService keeps a native solve's bytes;
- the bundled browser reference is read from its bytes;
- a value without received text (a saved record, a packet header, an edited copy, a test value) is written with `jsonTextKeepingNegativeZero`.

The importers:
- `features/result-export/resultExportAdapter.ts`, `ResultExportPanel.tsx`;
- `features/stress-neutral/StressNeutralExportPanel.tsx`;
- `services/analysisRunCompatibility.ts`, `previewService.ts`, `ruleCheckService.ts`;
- `features/workspace/resultsSessionState.ts`, `workspaceSession.ts`;
- `features/solve/LoadReferenceStatesBlock.tsx`;
- `features/comparison/ComparisonPanel.tsx`, `features/report/reportPackageRequest.ts`, `features/handoff/HandoffPanel.tsx`, `features/headless-runner/HeadlessRunnerPanel.tsx`;
- `features/results/ResultsPanel.tsx`, `HistoricalRunContext.tsx`, `knownSemanticLimitations.ts`, `loadReferenceOutputAvailability.ts`, `outputPolicy.ts`, `resultInterpretation.ts`, `retainedPrecisionDisclosure.ts`.

**Vitest tests (as built).**
- **Deleted** (they test the TypeScript readers themselves): `previewPhysicsEvidence.test.ts`, `physicsResultEvidence.parity.test.ts`, `physicsSourceRecovery.test.ts`, `loadReferenceEvidence.parity.test.ts`, `loadReferenceReaders.test.ts`, `loadReferenceReaderCases.test.ts`, `loadReferenceSourceLedger.test.ts`, `sourceBlockRecovery.test.ts`, `numericalResultQuality.test.ts`, `resultSemantics.test.ts`, and the pressure-1 reader's `pressureContractEvidence.test.ts` and `pressureConnectorEvidence.test.ts`. Their cases are the shared corpus below.
- **Re-pointed at the wasm reader:** the consumer suites above, and R1's retained suites where they called the deleted TypeScript dispatch. Native replays and fixture readings of physics-source-1 and load-reference-source-1 hand over the fixture bytes.
- **Added:** `baseReaderWasmEquivalence.test.ts` (item 1 below, kept as the desktop reader's conformance test). It runs over `test-support/baseReaderInputs.ts`, which builds the corpus statements from the corpus files as raw JSON.

**The shared conformance corpus.** The Rust reader and the Python verifier both run, unmodified:
- the result fixtures `fixtures/results/preview_physics_*.json` (including `preview_physics_tamper_vector.json`) and `physics_*_mechanics_*.json`;
- the raw source fixtures under `fixtures/product_preview/{physics_source,load_reference,load_reference_source,source_blocks}/`;
- `core/reporting/result_export/tests/fixtures/` (`load_reference_mutations.json`, `load_reference_source_mutations.json`, `source_blocks_statement_control.json`, the `load_reference_fallback_uz*` fixtures);
- the two TypeScript-only case files, `loadReferenceReaderCases.cases.json` and `loadReferenceSourceLedger.cases.json`, moved to `fixtures/results/` and read by both.

**What is gated, and how equivalence is shown.** Deleting the TypeScript base readers is gated on an equivalence test on R2's head, while both readers are present:
1. Every base-reader fixture and corpus entry gives the same outcome through wasm as through the native Rust reader: a pass with identical output (contract, standing, classification, row metadata), or the same code and detail.
2. It also gives the TypeScript reader's outcome, except for per-language differences that the R2 head counts and declares before deletion. There the desktop takes Rust's outcome.
3. Every desktop gate code (route classification, standing reason, rule-binding refusal, transport refusal, downgrade refusal) gives the same outcome through wasm in the vitest suites above, with only item 2's changes.

A difference outside item 2 stops the deletion.

**Gate results (stage D).**
1. All 560 statements give the native outcome through wasm, and through `resultReader.ts` reading their received text:
   - 549 dispatches, 548 reader validations, 73 transports and 9,997 rows (by digest);
   - the 11 texts no serde Value holds are refused at the text boundary.
2. Against the TypeScript readers, before deletion, no difference falls outside the declared classes:
   - route: 549 same;
   - binding: 493 same, and 1 accepted (the legacy 0.1 result, null where TypeScript throws);
   - standing: 499 same, 24 declared (code) and 10 declared (detail), and 16 accepted (transported metadata: needs_recompute where TypeScript throws a TypeError);
   - validations:
     - preview-physics: 14 same and 17 declared;
     - physics: 6 same;
     - physics-source: 94 same, 40 declared, and 1 accepted (the test seam, `lrs/RELABEL-joined-as-physics-source-resealed`);
     - source-blocks: 12 same and 2 declared;
     - load-reference: 230 same, 10 declared (detail) and 6 declared (parse);
     - load-reference-source: 121 same, 1 declared (detail) and 4 declared (parse);
   - transports: 73 same; rows: 9,997 same.
3. The consumer suites, base against head, give the same decisions with only item 2's changes:
   - every consumer test passes at both;
   - re-pointed expectations take Rust's code where both refuse: preview-physics `ROW_SIGNATURE` and `DANGLING_RESULT_REF`, and physics `INCOMPLETE_STRESS_HEADLINE`;
   - under G5, a successor whose receipt fails the transport checks reads unsupported. Its standing finding is `SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED`, and R1's registration still records the reader's code;
   - the bundled browser reference keeps its file's signed zeros.

**Rulings on what the TypeScript readers compute and Rust did not (T3 under the owner's decision, 2026-10-11).**
- G1: Rust's standing returns the TypeScript standing's findings, its codes in its order, beside `numerical_use_standing_with_context`'s token, which is unchanged.
- G2: the physics, preview-physics and load-reference readers take an optional model. With one, Rust runs TypeScript's model checks after the unchanged reader: every solved case is a model load case, each case's members are the model's pipes but those an objective connector replaces, and a support row names a model support (`SOURCE_PHYSICS_MODEL_CASE_COVERAGE`, `_MODEL_MEMBER_COVERAGE`, `_SUPPORT_BINDING`; on load-reference-1 under `SOURCE_LOAD_REFERENCE_PHYSICS_EVIDENCE`). Without one, the readers are today's. Preview-physics-1's TypeScript reader ignores its model, so it has no model check.
- G3: the check that the caller's model is the dispatched one stays in `previewService`.
- G4: TypeScript keeps each source route's registration (source-blocks, physics-source and the joined route) and passes the registered invocation to Rust's standing.
- G5: the header dispatch is the superset of both languages'. Rust's adds TypeScript's checks after `for_source_metadata`, whose retained transport reading stays: each source route's receipt shape (physics-source-1, source-blocks-1, and the joined policy plus the physics-source-1 shape), the evidence object the load-reference routes need, no evidence on precision-1, no carrier namespace, and no identity outside the desktop's routes (pressure-1). Order differences in the refusal code are item 2's declared class; a tampered retained statement takes Rust's table selection.
- `pressureContractEvidence.ts` is deleted in stage D.

**As built (R2 stage A).** `result_export::reader_verdict` holds the verdicts: `dispatch` (route, table binding `{id, sha256}`, header refusal, standing `{route, token, status, eligible, findings}`), `validate` and `transport` per reader, and `row_semantics` (signature, complete metadata, canonical metadata). The rulings live there, outside every existing reader and table: no priced function changes, and the retained pricing tool reaches none of them from the priced route. `operation_applier` exports them to wasm (`reader_dispatch_json`, `reader_validate_json`, `reader_transport_json`, `reader_row_semantics_json`); `openpipestress-runner read` adds `dispatch`, the six readers, five `<reader>-transport` entries and `row-semantics`. `fixtures/results/base_reader_outcomes.json` pins the native outcomes of 560 corpus statements (result_export's `base_reader_outcomes`, which also checks every Rust expectation the corpus files state), rows by digest. The desktop hands the readers the texts it received, as R1's exact-bytes path does.

**A2: numbers by value (R2, 2026-10-11).** Stage A found physics-source-1's section-parameters check comparing `pa_per_mpa` against the float `1e6` by JSON form, so a source re-serialized in JCS or `JSON.stringify` form (`1000000`) was refused. The readers now compare numbers by value (`json_value_eq`: binary64 equality, an integer equal to a float only when the float is that exact integer), at that check and at the result document's copied-member bindings in `derivative::validate_document` (the copied receipt, retained receipt, producer, numerical quality, formulation basis and contract evidence, and each row's copied value and witness quantities), which refused a document whose copies were written in JCS form. Every other numeric comparison already read by value (`as_f64`, the schema checkers' `const`/`enum`, load-reference's `same`, the retained receipt's integral normalization), read integers that every form keeps as integers, or compared two places of one statement, which any value-preserving re-serialization writes alike. `base_reader_outcomes` and the retained corpus run over every input in JCS form, in `JSON.stringify` form, and with only the source or only its model and invocation re-serialized: all outcomes are unchanged, except where the form changes a value. `JSON.stringify` rounds the unsafe integer of `NUM-unsafe-integer-in-publication`, and both forms write `-0` as `0`, which erases the tamper of four retained mutations and changes the reported bits, not the verdict, of statements with `-0` rows. The committed outcome files do not change.

**Item 2 as measured (R2 stage A).** Over the 560 statements, through wasm (equal to native on every outcome): route 549 same; standing 499 same, 34 in the declared class (both ineligible, Rust's finer code or detail, or TypeScript's missing registration where its own validation refused); reader validations 477 same, 80 in the declared class (including 10 text literals that Rust refuses at the text boundary and TypeScript's reader refuses after parsing); transport 73 and rows 9,997 all the same. Three more differences are declared (T3 under the owner's decision, 2026-10-11):
- the legacy 0.1 result's table binding is null in Rust where TypeScript's `sourceSemanticBinding` throws; both mean no binding;
- TypeScript's standing throws a TypeError on transported metadata (no results or status), where Rust answers `needs_recompute`; Rust's answer stands;
- one joined mutation (`RELABEL-joined-as-physics-source-resealed`) passes TypeScript's invocation-free ledger port and is refused by Rust's physics-source-1 reader without an invocation (`PHYSICS_SOURCE_PHYSICAL_SHAPE`, the corpus's pinned `ps` outcome); the two are different checks, so this is not a reader disagreement.

**What moves.** Desktop outcomes only where item 2 declares a difference, and the desktop bundle. No producer byte, reviewed input or table moves. Rust reader layouts and statics are unchanged unless R2 changes a Rust entry point; if it does, R2 joins the batch and re-registers under D3.

**The Python verifier's dependencies to cut (R1 and R2).**
- **Subprocess bridges to producer crates.** `core/serialization/canonical_json/adapter.py` (used by `compatibility.py` and `source_blocks.py`) and `core/units/adapter.py` (used by `physics_source.py`). R1 cut the retained verifier's use of both. They are replaced by Python implementations under `tests/`, checked against the crates' committed vectors rather than by calling them.
- **Product-path calls.** `compatibility.py`'s reader half (`_source_contract` and the dispatch into the evidence modules; `_retained_validation` and `_retained_transport` take the Rust verdict since R1) is called on product paths by its own analysis-run builder (`build_analysis_run_v0_3`, `validate_analysis_run_v0_3`), by `core/handoff/stress_neutral/package_v0_3.py` and by `core/product_preview/service.py`; `tools/validation/qualification_*` call it too. After R2 those callers take the Rust reader's verdict through the `openpipestress-runner` reader subcommand (R1 adds the retained entry, R2 the base dispatch); the qualification tools may call the verifier only from tests.
- **Moved under `tests/`:** `preview_physics_evidence.py`, `physics_evidence.py`, `physics_source.py`, `load_reference_evidence.py`, `load_reference_source.py`, `source_blocks.py`, and `compatibility.py`'s reader half (`retained_precision.py` moved in R1). `core/analysis_runs/__init__.py` stops re-exporting them. The record builder stays in `compatibility.py` (DEL-14-02).

## The join: `pressure-source-1`

**Preconditions:**
- T4's #1233 and #1234 are merged. v3 and `PressureContract` exist only there.
- T3's SQ2, TEXT-chain and statics tools are runnable on current heads.

**Identity (D1).** The reserved identity is `openpipestress.result_semantics/0.3.0/pressure-source-1`:
- receipt policy `PRESSURE-SOURCE-1`;
- case-evidence domain `pressure_source_case_evidence_v1`;
- one identity for model 0.3.0 and 0.4.0;
- profile `exact_pressure_v3`, shared with `pressure-1`.

It is published whenever any case of a v3 invocation publishes a selected retained-source response.
- **Unchanged:** the method's hash domains, the recovery token `retained_source_blocks_exact_v1` and the work limits.
- **Source family:** exactly v2's:
  - straight members;
  - linear restraints and springs;
  - nodal force and moment loads;
  - explicitly empty `pressure_regions`;
  - 0.4.0 eigen element loads and prescribed motions;
  - at most 256 DOFs.
- **Declined by name**, each with its existing decline:
  - pressure regions of any sign, zero included;
  - components (connectors, realized bends);
  - nonlinear supports;
  - non-nodal loads.

**D2: no shim.** The join goes inside B7 as the first stacked PR, with no constant substitution.
- `physics_source_recovery.schema.json` becomes contract-neutral: the policy and `profile_mode` are per contract.
- A reviewed `semantic_contract_v0_3_pressure_source_1.json` is added.
- Both are reviewed inputs, so the join head re-registers (D3).
- Rows still resolve through `physics_source_contract()`. A resident table for them would add a reader static, which goes to HELP_HUMAN before any code (O-10).

**D5.** On the join, ruling 3(b) (straight-only documents authored as v2) and ruling 3(c) (the not-joined diagnostic) lapse by their own condition. Ruling 3(a)'s SP-1 twin clause widens to retained-source publications, with a closed exclusion list:
- identity, profile and limitations;
- `profile_mode` and `profile_version`;
- the receipt policy;
- the invocation-bound hashes: invocation, publication, receipt, case evidence, `normalized_source_sha256` and `functional_plan_sha256`;
- the three work counters, asserted exactly as Δ = 24 × (len(v3 contract) − len(v2 contract)) per reservation.

**D6.** Until T4-U2's export branch lands, export of `pressure-source-1` is refused by name with `PRESSURE_1_EXPORT_NOT_AVAILABLE`.

**D7** is recorded in DEL-04-07 Design (`pressure-contract.md`).

| Unit | Content |
|---|---|
| J1 producer | Delete `exact_admission::joins_retained_source`, both gate sites, and T4-I31's `RETAINED_SOURCE_NOT_JOINED` diagnostic, flag and emission. Choose the identity, policy and case domain from the captured contract, never the envelope label. Give `pressure_v3_formulation_basis` joined and unjoined static texts. Add v3 twins `fixtures/product_preview/pressure_source/*`. Regenerate T4's two v3 reader corpora and the `pressure_1` table's limitations. |
| J2 corpus | `fixtures/results/pressure_source_reader_corpus.json`: v3 twins of the 12 committed source fixtures × 2 modes, plus mutations. A PP test generates it and checks that it is current. |
| J3 Rust reader | `SourceContract`; `Method::PressureJoined` for 0.4.0 (no projection onto v2); header dispatch (allowlist, downgrade exceptions, profile, row table); cross-read and downgrade refusals by name; the D6 export refusal. |
| J4 Python verifier | The same gates and codes, reading J2 unmodified. |
| (J5 dropped) | After R2 the desktop reads through wasm, so J3's Rust change covers it; there is no TypeScript reader to change (owner, 2026-10-10: "All readers."). |
| J6 schema and text | `results.v0.3` gains the `PRESSURE-SOURCE-1` receipt and the v3 physical-evidence definitions. DEL-04-07 Design records ruling 3's lapse. |
| J7 consequences | The D5 twin test over 12 fixtures × 2 modes. The frozen RF references in `validation/references/t3_r1` that lie in the source family, run through v3 recovery at the references' own criterion, with no reference or tolerance touched. T4's oracles (`t4_i6`, `t4_i9`, `t4_i12`) decline by name, with ordinary bytes unchanged. Mutants M1 and MJ1–MJ9, each killed by a named test. Byte identity, statics, the TEXT chain, SQ2 and an independent review. |

**Blast radius.**
- **Moves:**
  - two reviewed inputs: the neutral receipt schema and the new table. The join head re-registers (D3);
  - priced TEXT, slightly: the joined limitation sentence and the new constants are added, and the not-joined diagnostic is removed;
  - T4's two v3 reader corpora are regenerated, and T4's "v3 not joined" pins flip. Both are declared.
- **Expected unchanged:** priced memory.
- **Unchanged:** every v2 byte, every successor byte, the 07m/07n/07o outcomes and the source-block corpora.

## B7a and B7b

**The split** is the one T3's legacy inventory proposed; item labels such as A1, E8 or D6 are that inventory's, kept for traceability.
- **B7a** holds everything that does not depend on deleting v2: the H-4 wave, the pressure-free half of the model-format change, the other old format versions and the legacy removals.
- **B7b** consolidates the exact route onto v3. That needs the join.

**Registration.** Each head re-registers under D3 and regenerates what it moves. B7b re-derives the exact-route bytes that B7a has already moved (E15, D6). The release identity is the build identity registered on B7b's head, which the batch lands with. Release remains the owner's act.

### B7a

| Item | Change | Blast radius |
|---|---|---|
| B4/D1 census (first in B7a; HELP_HUMAN 2026-10-10) | Run every committed source-block case through W1, in both modes, and at ordinary and UTM coordinates where the corpus has them. Criterion: every case that today publishes a retained/exact source-block result must still publish a retained result through W1, at the same or better standing. A case that would drop to ordinary-only is a capability loss. **If some cases fail**, do not fall back to (a) at once: the owner's direction prefers one mechanism, rewritten where needed, over keeping two. First assess whether W1 can be extended to certify the uncovered cases. If it can at a proportionate cost, extend W1 in B7a, then take (b). Return to HELP_HUMAN with the failing cases, the extension's size, and (a) as the alternative only if the extension is disproportionate or needs a design T3 cannot settle. | The census itself moves nothing. A W1 extension, if needed, moves priced W1 types and TEXT inside B7a's re-registration. |
| B4/D1 (b): retire the preview-route source blocks | When the census passes (or after the W1 extension): the pressure-free captured route no longer selects source blocks, and the non-composite `source-blocks-1` identity, its precision-1 ordinary-case rows, `SOURCE_BLOCKS_ORDINARY_CASE_LEGACY_SEMANTICS` and `RULE_SOURCE_BLOCKS_SUMMARY_NOT_RELIABLE` go. W1's coexistence rule (D-15) and `retained_wire.rs`'s `legacy_source` path go, and `legacy_source` and `legacy_source_work` are deleted from the SCHEMA. The exact-route composites (`physics-source-1`, `load-reference-source-1`, `pressure-source-1`) keep their source blocks under either outcome. | The SCHEMA member removal in every successor and corpus base (reviewed SCHEMA). `source_blocks_1` and `source_block_recovery.schema.json` (reviewed) are deleted, or reduced to what the composites still use. Priced `source_recovery`/`source_receipt` types and W1's legacy-source types. The preview source-block corpora (`fixtures/product_preview/source_blocks/**`). The readers (Rust `source_blocks.rs` and standing, the verifier) and the desktop notices `N_SB`, `N_SB_MIXED`. All inside B7a's single re-registration; the SCHEMA and bases move once. |
| H-4: C1–C3 | Rename `component_user_stiffness_macro_element_count` to the curved macro-element count. Remove the historical joint row kind (`component_user_stiffness_macro_element_review`); do not rename it. Delete the retired thrust count and `expansion_joint_pressure_thrust_load_review`. | Priced `MechanicsEnvelope`/`Summary`; all ten semantic tables (reviewed); all 45 corpus bases; every result fixture; `results.v0.3`; the desktop's summary display. |
| H-4: C4–C6 | Reword `preview_physics.rs` LIMITATIONS[1] without "legacy". Apply T4-U4's arc-frame unification on preview-physics-1 if it is ready. Reword any other LIMITATIONS that name retired inputs. | Preview-physics-1 and preview-physics-retained-1 tables (reviewed); `results.v0.3`; 19 result fixtures; all 45 corpus bases; the 6 mutations whose edit paths index `limitations`; 12 source-block raw fixtures and 2 precision fixtures. |
| Model format, pressure-free half (A1, E8, the namespace predicates) | Define 0.5.0 (below) for documents without a pressure contract, and refuse 0.1.0/0.2.0 by name with a re-author diagnostic. Delete the 0.1.0 → 0.2.0 chain (`apps/desktop/src-tauri/src/model_document_migration.rs`), its TypeScript mirror (`apps/desktop/src/services/projectService.ts`, `apps/desktop/src/features/workspace/projectPersistenceIntegrity.ts`) and PP's version arms (`pressure_runtime.rs`, `source_recovery.rs`). Key the W1 namespace on the contract's absence instead of the version: `retained_memory.rs` `NamespaceBranch::Preview`, the Rust reader's `preview_namespace` and the verifier's `_preview_namespace` (the names were aligned in pre-B7; only the predicate changes here). Geometry operations take the current version, not only 0.1.0. | About 112 model fixtures. All 45 corpus bases (30 at 0.1.0, 15 at 0.2.0), and with them every invocation digest, receipt, publication hash and must-pass entry. The milestone and G10 fixtures (below) and the bundled demo models. The migration fields of `schemas/project_persistence.schema.yaml` and `docs/architecture/persistence_contract.md`'s until-B7 notes. The desktop open path's refusal statuses and save normalization (DEL-07-11 Design). The pricing chain's rule for the Rust reader's `preview_namespace` (re-pointed in pre-B7) if the predicate's TEXT moves. |
| Analysis-run records (E1) | Keep 0.3.0 only. Delete the 0.1, legacy-desktop 0.1 and 0.2 schemas and their dispatcher arms; `core/analysis_runs/legacy.py`; the non-JCS `records.canonical_json` hash; `build_analysis_run_v0_2`; and the desktop's legacy digests (`analysisRunCompatibility.ts`, `HistoricalRunContext.tsx`). | `fixtures/analysis_runs/invented/{legacy_desktop_v0_1,legacy_python_v0_1,analysis_run_v0_2}.json`; the `test_analysis_run_*` suites and `HistoricalRunContext.test.tsx`. Saved runs from older records are refused by name. No reviewed input, priced layout or corpus moves. |
| Stress-neutral exports (E3) | Native 0.3 builders in Python and in the desktop. Today both project from the 0.1 packet (`package.py`, `buildStressNeutralExportPacketV01`). Delete the 0.1 and 0.2 schemas and their dispatcher arms, `package.py`, `package_v0_2.py` and the panel's `semantic_contract_v0_2.json` reference. | 0.3 package bytes must not change. A byte test over the 14 committed `*.stress_neutral.json` fixtures checks this, and any change stops the item. Also `test_stress_neutral_*` and `StressNeutralExportPanel.test.tsx`. |
| Result exports (E2) and the mechanics-result 0.1.0 carrier (B2's format part; confirmed by HELP_HUMAN 2026-10-10) | Keep one export version, 0.3.0. Delete `results.v0.1`, `results.v0.2` and the dispatcher's inline copies; `derivative.rs`'s `legacy_checksum` and legacy-enriched branches; and the runner's relabeling of 0.3.0 as 0.2.0. Readers refuse a producer-less 0.1.0 mechanics result, and `semantic_contract_v0_2.json` leaves the reviewed inputs. | One reviewed input fewer. The v0_2 SHA pins (10 files). `invented_demo_result_legacy_0_1.json` and the `result_export_v0_2*` fixtures. Runner and `derivative_contract` tests. The Tauri and `product_preview/service.py` consumers. |
| E11 | Delete `#[allow(dead_code)] mod pressure_exact`'s dead items, the `formation_guard::RecoveryRecord` fields that only tests read, and the unread `unit`/`label` fields in PP `lib.rs`. | `RecoveryRecord` is priced, so its layout is re-priced. No published byte moves. |
| E13 (done in pre-B7) | PP's second combination engine, `append_combination_results`, was deleted in the pre-B7 batch; its test was ported to `append_combined_vector_magnitude`. | None left for B7. |
| E13 leftover | `append_combined_vector_magnitude` (PP `lib.rs`) keeps a `support_vectors` parameter that every caller passes empty, and its reaction arm. Delete both, together with the pricing decision ID_AUDIT[39] that covers them. | No published byte moves; TEXT and the pricing decision list change, so it rides B7a's re-registration. |
| `springs` | `SelectedSourceRecovery.springs` (`source_recovery.rs`) is never read; delete it. | `source_recovery` types are priced, so the layout is re-priced. No published byte moves. |
| E14 | Delete the DEC-050/053 unscaled-LU observation lane and its rows (`legacy_unscaled_DEC050_DEC053`, `sparse_live_path_dense_parity_relative_delta`), or re-base the parity observation on the current solve. | Every envelope's observation rows (122 and 47 occurrences in the corpus); tables; successor bytes; the memory guard's byte estimate; the milestone model's displayed rows (G10, held). |
| E15 | Use one correctly rounded norm (`norm3`) for `source_receipt`'s `scaled_norm` and for `displacement_magnitude`. | Successor and corpus bytes where the last bits differ; every source receipt (all source-block corpora). |
| E16 | Replace `stress_recovery`'s `#[path]` include of `correct_norm.rs` with a small shared crate. | PP `Cargo.lock` (reviewed). No published byte moves. |
| E17 | The G7 magnitude guard compares exactly against a correctly rounded norm. This replaces nested libm `hypot` with a 64-eps tolerance in the Rust reader and the verifier; the TypeScript copy is gone after R2. | Outcomes of near-tolerance tampering may move: the 10 `g7*` mutations. Reader statics and TEXT. |
| D6 | One checked `WorkTotal`. Delete the frame kernel's saturating counter (`legacy_saturated`), its compatibility wrappers (`gap_sign` beside `gap_sign_with_work`, `multi.rs`'s legacy getter) and `SaturationNotExcluded`. | Source-receipt bytes, and `legacy_source_work[].rejected` in every successor and corpus base. Priced `source_recovery` and `source_receipt` types. |
| D3 strings | Desktop notices that narrate history ("until T3", "until T6" in `knownSemanticLimitations.ts` and `numericalResultQuality.ts`) state current behaviour instead. | Desktop text and its tests only. |
| B1: precision-1 | On main no route publishes precision-1: every branch overwrites the default `mechanics_producer()` identity (`lib.rs`). Make the identity a function of the route and delete `PRECISION_SEMANTIC_CONTRACT_ID`'s default. Delete the readable historical route and its notices: Rust `PRECISION_1_HISTORICAL_SEMANTICS` (`semantic_contract.rs`), the verifier's and `compatibility.py`'s historical branches, TS `N_P1`, `REPORT_PACKAGE_PRECISION_1_HISTORICAL`, the `"precision"` route in `numericalResultQuality.ts`, and the Tauri precision-1 branches. Delete `semantic_contract_v0_3_precision_1.json` and re-root inheritance by inlining the rows each table needs: preview-physics-1 and physics-1 inherit it directly, and preview-physics-retained-1, physics-retained-1 and load-reference-1 through them. Under B4/D1 (b) the non-composite source-blocks-1 table goes, so nothing is inlined there; only if HELP_HUMAN later chooses (a) are its rows inlined into source-blocks-1. | One reviewed input fewer; every inheriting table's bytes (`inherited_semantic_contract_sha256`), so five reviewed inputs move. Reader statics shrink (re-priced). The precision-1 branches of `results.v0.3`, `stress_neutral_export.v0.3` and `analysis_run.v0.3`; `package_v0_3.py`'s default precision-1 reference. Fixtures `precision_connected_ui_mechanics_{dense,sparse}.json` and `precision_transport_v0_3.json` go. Stored precision-1 results are refused by name. The desktop's standing text changes (B8 stays held). |
| B3: retired row kinds | The preview and exact routes stop generating retired kinds: `render` emits the current rows directly (support components v2, the certified maximum) instead of generating `reaction_resultant` and `open_formula_stress_summary` (`lib.rs`) and stripping them (`preview_physics.rs` `RETIRED_KINDS`, `RETIRED_CODES`). Delete the strip step, the readers' retired-kind checks (`preview_physics_evidence` `RETIRED_CODES`, the tables' `retired_source_kinds` key), TS `N_RULE_RETIRED`, `RULE_BINDS_RETIRED_RESULT`, `RETIRED_ID_TAILS`, `isRetiredResultId`, and the dead retired rows in every table including source-blocks-1's ordinary-case rows, which go with B4/D1 (b); the generator goes entirely. | Eight tables (reviewed). Priced `MemberRecord`/`ResultItem` paths. No successor row may change: the census must show 0 row changes, else the item stops. Rule packs bound to retired ids get the generic missing-result refusal. |
| B5: stale reservations | Delete from `reserved_inactive_successors` the identities that are published (`source-blocks-1` in physics-1, physics-retained-1 and load-reference-1; `load-reference-source-1` in load-reference-1) and the unused `reactions-1`, `stress-1` and `precision-2`. No reader consumes the key. | Table bytes only (reviewed inputs); `tests/test_load_reference_readers.py`'s reservation assertion. |
| B6: downgrade refusals | Verified on main: only `ANALYSIS_LEGACY_SOURCE_DOWNGRADE_FORBIDDEN` (`compatibility.py`, `analysisRunCompatibility.ts`) guards a legacy route; it goes with E1. `SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN` (a source-block member on a non-source identity) and the retained-precision downgrade guard are closed-shape checks between current identities, and the join uses the first (MJ7), so they stay. The first is renamed without "LEGACY" (`SOURCE_BLOCKS_DOWNGRADE_FORBIDDEN`). | The 2 corpus entries expecting the old code; Rust `semantic_contract.rs` and `derivative.rs`, `compatibility.py`, the verifier and the join's MJ7 test. |
| D8: snapshot pin tests | Replace the per-snapshot replays (Rust `retained_precision_contract.rs` `snapshot_07*_mutation_outcomes`, their Python twins) with one corpus-driven harness per language over the regenerated corpus (ids, `expected`, must-pass). The TypeScript twins go with R1 and R2. | Tests only. |
| `.opsproj` one version (DEC-028; owner, 2026-10-10: "Close: one version") | Contract only: no `.opsproj` reader exists yet, and the report package writes `schema_version 1.0.0` only. When a reader lands, it accepts only the current package version and refuses older versions by name, with no migration. | None today. The reader's tests and DEL-02-05/DEL-08-01's commitments carry the rule. |
| Corpus | Regenerate the 45 bases once, under B7a's producer, as one snapshot that replaces 07m, 07n and 07o. Keep mutation ids where the meaning is unchanged, and re-derive the limitations-path mutations. Regenerate the successor, preview, precision, source-block and demo fixtures. | Census: first failures and must-pass outcomes unchanged, except those declared. |

### B7b

| Item | Change | Blast radius |
|---|---|---|
| D4 (a): W1 route E | Rebase `physics-retained-1` from `physics-1` with `{2.0.0, exact_straight_pressure_v2}` onto `pressure-1`. | DEF-E (`retained_precision_prepared_exact_v1.json`) is replaced, and the `physics-retained-1` table changes (both reviewed). The exact successor fixtures, `retained_precision_rv120_b3_inputs.json` and G5's route-E pricing move. No shared-corpus base moves, because none is exact. |
| v2 deletion (A4, A5) | Delete `ExactContract::StraightV2` and the v2-only producer, reader and desktop arms, including the cross-read refusals (`V3_READ_AS_V2`, `V2_READ_AS_V3`). Delete the `physics-1`, `physics-source-1`, `load-reference-1` and `load-reference-source-1` tables. `physics_source_recovery.schema.json` takes v3 constants. `pressure-1` gets its own registered table, priced under O-10. | Reviewed inputs: four tables out, one in, plus the receipt schema. The v2 raw fixtures and derived documents are replaced by their v3 twins. The v2 branches of `results.v0.3` and `stress_neutral_export.v0.3`, and the desktop's pressure-authoring panels, change. |
| B5, last entry | Delete `pressure-1` from `reserved_inactive_successors` in the tables that survive, since it is active by then. | Table bytes (reviewed), re-registered with B7b. |
| Model format, exact half (A6) | 0.5.0 gains the contract-present form and the load/reference-state group, and 0.3.0/0.4.0 are refused by name. Every version-keyed selector keys instead on the contract and the group: `case_state::is_load_state` and `LOAD_STATE_MODEL_VERSION`, `preview_physics.rs`'s 0.4.0 test, `pressure_runtime::is_exact`, and the join's record key, which today is present exactly on 0.4.0. | Every exact and load-reference fixture is re-authored once, to 0.5.0 with v3. The source-block and load-reference corpora, T4's v3 reader corpora and J2 move with them. |

## Current model format

**Evidence.** It comes from `apps/desktop/src-tauri/src/model_document_migration.rs`, `core/product_physics/src/pressure_runtime.rs`, `core/product_physics/src/case_state/` and the schemas. `schemas/model.schema.yaml` has one shape for every version; the meaning of each version lives in code.

| Version | What it expresses |
|---|---|
| 0.1.0 / 0.2.0 | No pressure contract (a declared one is refused) and no `pressure_regions`. It solves on the preview route, with combinations (0.2.0 adds their shape fields), nonlinear supports, components and `equivalent_static`. 0.1.0 reaches 0.2.0 by a no-op migration. |
| 0.3.0 | The exact pressure contract is required (`PRESSURE_CONTRACT_REQUIRED`), and every case carries `pressure_regions`, with `[]` when it is unpressurized. It solves on the exact route only. That route refuses combinations, nonlinear supports, `equivalent_static` and, until T4 extends it, components. |
| 0.4.0 | 0.3.0 plus the load/reference-state group: `reference_configurations`, material `expansion_laws`, and an explicit `analysis_state` on every case. All of these are required, and so is the exact contract. The version alone selects the load-reference identities. |

**Choice: neither 0.3.0 nor 0.4.0; B7 defines 0.5.0.**

Neither existing version expresses models with and without pressure as one format.
- **Both require the exact contract.** Every document therefore goes to the exact route, and a pressure-free model that needs the preview route cannot be written in either.
- **The milestone model is such a model.** It is pressure-free on the preview route (W1 route L). Writing it as 0.3.0 would move it onto route E.
- **Widening either in place is ruled out.** It would change what an existing version can contain, against DEC-033's rule that such a change bumps the minor version. Behaviour would also stay keyed on a version that means two things.

**Model document 0.5.0** is the only current version:
HELP_HUMAN accepted this design on 2026-10-10. DEL-02-05 CLM-003 and DEL-04-07 Design point to it.

- **`pressure_contract` is optional.**
  - Absent: the document is pressure-free and solves on the preview route. `pressure_regions` and every pressure primitive are refused by name.
  - Present: it names the current exact contract (v3 after B7b), and every case carries `pressure_regions`, as 0.3.0 requires today.
- **The load/reference-state group is optional as a whole.** When present, it carries 0.4.0's rules (an explicit `analysis_state` on every case) and requires the exact contract, as today.
- **Behaviour keys on presence.** It keys on whether the contract and the group are present, never on the version string.
- **Every other version is refused by name** with a re-author diagnostic. Nothing is migrated.

**Re-authoring the fixtures.**
- **B7a** re-authors every committed 0.1.0/0.2.0 model to 0.5.0 and re-checks its pins. This includes:
  - `fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json`;
  - the G10 desktop-shaped milestone fixture `fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.desktop_model.json`;
  - the demo models `invented_demo_model.json` and `invented_preview_model.json`.

  The pins re-checked include `core/product_physics/tests/g10_desktop_milestone_model.rs`, `apps/desktop/src/services/g10DesktopMilestoneModel.test.ts`, the milestone successor fixtures and corpus base, and both `retained_precision_admission.rs` suites.
- **B7b** re-authors the 0.3.0/0.4.0 fixtures.

## Held by the owner

These are not decided here.
- **G10 interpretation.** B7a changes the milestone model's document version, its ordinary-route bytes and its displayed rows (C4, E14, the model format). Whether G10 is witnessed before B7a, or re-issued against the B7 head, is the owner's choice.
- **B8 (activation).** R and B7a change the desktop's retained standing text and notices. Nothing here activates anything.
- **Decision 22.** Any change of M or of the budget model.
