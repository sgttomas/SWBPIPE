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
- **Inventory items ride B7.** B1, B3, B4/D1, B5, B6 and D8 ride B7a, or B7b where they depend on v2's deletion or the join: they move tables or the corpus, and the owner's principle favours removal, so re-registration and regeneration are paid once. B4/D1 carries a design question and is not planned yet (see "Open").
- **0.5.0 is accepted** as the current model format ("Current model format").
- **R-b′ for all members** is HELP_HUMAN's ruling under the owner's delegation, conditional on the reviewer's finding (B7a).
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
- **Python, a test-only verifier.** `core/analysis_runs/retained_precision.py` becomes an independent verifier that only tests use, never a product path.

After R1 and R2, B7's table, schema and reader changes are made in two places, not three: the Rust reader and the Python verifier.

**Bytes and registration.** R moves no reviewed input, priced layout, table, successor byte or corpus base, so it needs no re-registration. If it needs a Rust-reader change that moves any of them, it joins the batch and re-registers under D3. It does move desktop IPC and export bytes (the exact-bytes fix below); no producer byte moves.

**Exact bytes to the desktop (HELP_HUMAN, 2026-10-10).**
- **The defect.** The desktop's solve commands re-encode the producer's envelope. `solve_preview_mechanics_with_mode` in `apps/desktop/src-tauri/src/lib.rs` returns `serde_json::to_value(envelope)`, and the desktop's `serde_json` has no `preserve_order`, so the value carries sorted keys. It serves `run_preview_mechanics`, `run_preview_mechanics_with_solver_mode`, and the job path (`start_preview_mechanics_job`, `start_preview_mechanics_job_with_solver_mode` → `publish_solve_outcome` → `poll_preview_mechanics_job`). The webview then re-serializes: `ResultExportPanel.tsx`'s `jsonDataHref` writes `JSON.stringify` of the parsed object, which `save_local_result_json` saves. So IPC and exported bytes differ from the pinned producer bytes, and the hash identity of anything exported from the desktop is at risk.
- **The fix.** The native side hands over the producer's exact serialized bytes, and the Rust reader in the desktop reads those bytes. Exports write the received bytes, never a re-serialization of a parsed object.
- **What moves.** Desktop IPC payloads and exported result bytes, which become the producer's bytes. The IPC shape tests move with them (`services/previewService.test.ts` replays, `features/result-export/nativeResultSave.test.ts`, `App.test.tsx`, and the native tests in `src-tauri/src/lib.rs`). No producer byte moves.

**Wasm.**
- **Where the bindings go.** They are added to the desktop's existing wasm engine crate, `core/model_operations/operation_applier` (`npm run build:wasm`). That crate already resolves `open_pipe_stress_result_export` through product_physics.
  - result_export gains no dependency, so PP's `Cargo.lock`, a reviewed input, does not move.
- **What they expose.** `retained_precision::validate` and `validate_transport_metadata`, JSON in. Out comes the `Validation`, or the error's gate, code and detail.
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

**Vitest tests that change.**
- **Deleted:** `features/results/retainedPrecision.test.ts`, the reader-internal tests (`phi512`, `eHat`, `stopFeasible`, `nativeSchedule`, `accountingRules`, …). The Rust tests and the verifier cover these.
- **Re-pointed at the wasm reader:**
  - `retainedPrecisionIntegration.test.tsx`;
  - `retainedPrecisionOutputRefusal.test.tsx`;
  - `outputPolicy.test.ts`;
  - `result-export/retainedPrecisionResultExport.test.tsx`;
  - `stress-neutral/retainedPrecisionStressNeutral.test.tsx`;
  - `stress-neutral/StressNeutralExportPanel.test.tsx`;
  - `services/retainedPrecisionAnalysisRun.test.ts`.

**The Python verifier.**
- **Location.** It moves under `tests/`. A test asserts that no module outside `tests/` imports it.
- **Product callers.** `core/analysis_runs/compatibility.py`'s `_retained_validation` and `_retained_transport` stop importing it.
  - Python product code that meets a retained successor takes the Rust reader's verdict. It gets it through an `openpipestress-runner` subcommand that R adds over the same two functions.
  - It never re-derives a verdict.
- **Independence.** It shares no code with the producer.
  - Today it hashes through `core/serialization/canonical_json/adapter.py` and converts units through `core/units/adapter.py`. Both are subprocess bridges to crates the producer links.
  - R replaces both with Python implementations under `tests/`. These are checked against those crates' committed vectors, not by calling them.

**The shared conformance corpus.** These become the conformance corpus shared by the Rust reader and the Python verifier:
- the 07m/07n/07o corpora;
- `retained_precision_b2_parity_probes.json` and `retained_precision_carrier_cases.json`;
- the successor fixtures `retained_precision_*_successor_*.json`;
- `retained_precision_rv120_b3_inputs.json`.

Both run every entry unmodified. `expected_by_reader` and `expected_unbound_by_reader` lose their `typescript` member.

**What is gated, and how equivalence is shown.** Deleting the TS reader is gated on an equivalence test. It runs on R's head while both readers are still present.
1. **Wasm matches native Rust.** This covers every corpus entry, including its mutation, must-pass, transport (`expected_transport`) and unbound (`expected_unbound`) forms. Through wasm, each gives the same outcome as the native Rust reader: a pass with an identical `Validation`, or the same gate, code and detail.
2. **Wasm matches the TS reader, except where declared.** The exceptions are 47 entries (46 in 07m/07n, 1 in 07o) with 92 expectations, where the TypeScript expectation differs from Rust's: Rust reports finer base codes at G7. There the desktop takes Rust's outcome, and the change is declared.
3. **Every desktop gate code gives the same outcome through wasm.** These are the codes the standing, disclosure, quality, analysis-run, historical-run, result-export and stress-neutral consumers receive, run in the vitest suites above. The only changes allowed are item 2's.

A difference outside item 2 stops the deletion.

### R2: the base-route readers (owner, 2026-10-10: "All readers.")

**Rust, the only product reader.** In `core/reporting/result_export/src/`: `preview_physics_evidence.rs`, `physics_evidence.rs`, `physics_source.rs`, `load_reference.rs`, `load_reference_source.rs`, `source_blocks.rs`, and the dispatch and standing in `semantic_contract.rs` (`for_source`, `for_source_metadata`, `standing_reason`, `rule_binding_refusal`, `class_binding_refusal`, `numerical_use_standing*`, `classification_summary*`, `signature`, `canonical_metadata`). The desktop calls them through the same wasm engine bindings as R1, so PP's `Cargo.lock` does not move.

**TypeScript readers deleted** (under `apps/desktop/src/features/results/`): `previewPhysicsEvidence.ts`, `physicsResultEvidence.ts`, `physicsSourceRecovery.ts`, `loadReferenceEvidence.ts`, `loadReferenceSourceEvidence.ts` and `sourceBlockRecovery.ts`; the route dispatch, standing and downgrade guards in `numericalResultQuality.ts`; and the table lookups in `resultSemantics.ts`. What stays in TypeScript is presentation over the Rust verdict: notice texts (`knownSemanticLimitations.ts`), panels and labels.

**Call sites that change** (non-test importers, under `apps/desktop/src/`):
- `features/result-export/resultExportAdapter.ts`, `ResultExportPanel.tsx`;
- `features/stress-neutral/StressNeutralExportPanel.tsx`;
- `services/analysisRunCompatibility.ts`, `previewService.ts`, `ruleCheckService.ts`;
- `features/workspace/resultsSessionState.ts`, `workspaceSession.ts`;
- `features/solve/LoadReferenceStatesBlock.tsx`;
- `features/comparison/ComparisonPanel.tsx`, `features/report/reportPackageRequest.ts`, `features/handoff/HandoffPanel.tsx`, `features/headless-runner/HeadlessRunnerPanel.tsx`;
- `features/results/ResultsPanel.tsx`, `HistoricalRunContext.tsx`, `knownSemanticLimitations.ts`, `loadReferenceOutputAvailability.ts`, `outputPolicy.ts`, `resultInterpretation.ts`.

**Vitest tests.**
- **Deleted** (they test the TypeScript readers themselves): `previewPhysicsEvidence.test.ts`, `physicsResultEvidence.parity.test.ts`, `physicsSourceRecovery.test.ts`, `loadReferenceEvidence.parity.test.ts`, `loadReferenceReaders.test.ts`, `loadReferenceReaderCases.test.ts`, `loadReferenceSourceLedger.test.ts`, `sourceBlockRecovery.test.ts`, `numericalResultQuality.test.ts`, `resultSemantics.test.ts`. Their cases move into the shared corpus below.
- **Re-pointed at the wasm reader:** `physicsSourceIntegration.test.tsx`, `knownSemanticLimitations.test.ts`, `outputPolicy.test.ts`, `features/solve/LoadReferenceStatesBlock.test.tsx`, `features/workspace/{resultsSessionState,loadReference.resultsSessionState}.test.ts`, `features/result-export/{physicsResultExport,resultExportAdapter}.test.ts`, `services/{analysisRunCompatibility,previewService}.test.ts`, `features/stress-neutral/StressNeutralExportPanel.test.tsx`, `features/rule-check/RuleCheckRunPanel.test.tsx`, `features/report/reportPackageRequest.test.ts`, `features/handoff/HandoffPanel.test.tsx`, `features/headless-runner/HeadlessRunnerPanel.test.tsx`, `App.test.tsx`.

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

**What moves.** Desktop outcomes only where item 2 declares a difference, and the desktop bundle. No producer byte, reviewed input or table moves. Rust reader layouts and statics are unchanged unless R2 changes a Rust entry point; if it does, R2 joins the batch and re-registers under D3.

**The Python verifier's dependencies to cut (R1 and R2).**
- **Subprocess bridges to producer crates.** `core/serialization/canonical_json/adapter.py` (used by `compatibility.py`, `source_blocks.py` and `retained_precision.py`) and `core/units/adapter.py` (used by `physics_source.py` and `retained_precision.py`). They are replaced by Python implementations under `tests/`, checked against the crates' committed vectors rather than by calling them.
- **Product-path calls.** `compatibility.py`'s reader half (`_source_contract`, the dispatch into the evidence modules, `_retained_validation`, `_retained_transport`) is called on product paths by its own analysis-run builder (`build_analysis_run_v0_3`, `validate_analysis_run_v0_3`), by `core/handoff/stress_neutral/package_v0_3.py` and by `core/product_preview/service.py`; `tools/validation/qualification_*` call it too. After R2 those callers take the Rust reader's verdict through the `openpipestress-runner` reader subcommand (R1 adds the retained entry, R2 the base dispatch); the qualification tools may call the verifier only from tests.
- **Moved under `tests/`:** `preview_physics_evidence.py`, `physics_evidence.py`, `physics_source.py`, `load_reference_evidence.py`, `load_reference_source.py`, `source_blocks.py`, `retained_precision.py`, and `compatibility.py`'s reader half. `core/analysis_runs/__init__.py` stops re-exporting them. The record builder stays in `compatibility.py` (DEL-14-02).

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
| B1: precision-1 | On main no route publishes precision-1: every branch overwrites the default `mechanics_producer()` identity (`lib.rs`). Make the identity a function of the route and delete `PRECISION_SEMANTIC_CONTRACT_ID`'s default. Delete the readable historical route and its notices: Rust `PRECISION_1_HISTORICAL_SEMANTICS` (`semantic_contract.rs`), the verifier's and `compatibility.py`'s historical branches, TS `N_P1`, `REPORT_PACKAGE_PRECISION_1_HISTORICAL`, the `"precision"` route in `numericalResultQuality.ts`, and the Tauri precision-1 branches. Delete `semantic_contract_v0_3_precision_1.json` and re-root inheritance by inlining the rows each table needs: preview-physics-1 and physics-1 inherit it directly, and preview-physics-retained-1, physics-retained-1 and load-reference-1 through them. Inline its rows into source-blocks-1 until B4/D1 is decided. | One reviewed input fewer; every inheriting table's bytes (`inherited_semantic_contract_sha256`), so five reviewed inputs move. Reader statics shrink (re-priced). The precision-1 branches of `results.v0.3`, `stress_neutral_export.v0.3` and `analysis_run.v0.3`; `package_v0_3.py`'s default precision-1 reference. Fixtures `precision_connected_ui_mechanics_{dense,sparse}.json` and `precision_transport_v0_3.json` go. Stored precision-1 results are refused by name. The desktop's standing text changes (B8 stays held). |
| B3: retired row kinds | The preview and exact routes stop generating retired kinds: `render` emits the current rows directly (support components v2, the certified maximum) instead of generating `reaction_resultant` and `open_formula_stress_summary` (`lib.rs`) and stripping them (`preview_physics.rs` `RETIRED_KINDS`, `RETIRED_CODES`). Delete the strip step, the readers' retired-kind checks (`preview_physics_evidence` `RETIRED_CODES`, the tables' `retired_source_kinds` key), TS `N_RULE_RETIRED`, `RULE_BINDS_RETIRED_RESULT`, `RETIRED_ID_TAILS`, `isRetiredResultId`, and the dead retired rows in every table except source-blocks-1, whose ordinary-case rows follow B4/D1. | Eight tables (reviewed). Priced `MemberRecord`/`ResultItem` paths. No successor row may change: the census must show 0 row changes, else the item stops. Rule packs bound to retired ids get the generic missing-result refusal. |
| B5: stale reservations | Delete from `reserved_inactive_successors` the identities that are published (`source-blocks-1` in physics-1, physics-retained-1 and load-reference-1; `load-reference-source-1` in load-reference-1) and the unused `reactions-1`, `stress-1` and `precision-2`. No reader consumes the key. | Table bytes only (reviewed inputs); `tests/test_load_reference_readers.py`'s reservation assertion. |
| B6: downgrade refusals | Verified on main: only `ANALYSIS_LEGACY_SOURCE_DOWNGRADE_FORBIDDEN` (`compatibility.py`, `analysisRunCompatibility.ts`) guards a legacy route; it goes with E1. `SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN` (a source-block member on a non-source identity) and the retained-precision downgrade guard are closed-shape checks between current identities, and the join uses the first (MJ7), so they stay. The first is renamed without "LEGACY" (`SOURCE_BLOCKS_DOWNGRADE_FORBIDDEN`). | The 2 corpus entries expecting the old code; Rust `semantic_contract.rs` and `derivative.rs`, `compatibility.py`, the verifier and the join's MJ7 test. |
| D8: snapshot pin tests | Replace the per-snapshot replays (Rust `retained_precision_contract.rs` `snapshot_07*_mutation_outcomes`, their Python twins) with one corpus-driven harness per language over the regenerated corpus (ids, `expected`, must-pass). The TypeScript twins go with R1 and R2. | Tests only. |
| R-b′ for all members (conditional; HELP_HUMAN 2026-10-10, under the owner's delegation) | T4's O-13 work found that R-b′'s resolution conjunct (`q > 2^10·B`) masks gross arc errors at k ≥ 1e8 and adopted an extra arc firing condition `B_f > RD(1e-9)·q` (DEL-04-01 Design, `arc-recovery-guard.md`). If the reviewer finds straight members masked the same way, `formation_guard.rs`'s `rb_prime_fires` becomes one predicate for all members. If not, this item is dropped. | `formation_guard.rs`; the S11-G pins (`s11g_tests.rs`, straight_pipe's `t17`). Straight ends that newly fire move cases from `NUMERICAL_INTEGRITY_CHECKS_PASSED` to `NUMERICAL_INTEGRITY_SENSITIVE`; any corpus base, successor or table outcome that moves is declared. TEXT; priced `RecoveryRecord` only if a field changes. |
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

## Open: needs a ruling

- **B4/D1: the source-blocks route on the preview route, and W1's `legacy_source` member.** On main a captured pressure-free solve whose selected case uses exact source-block recovery publishes `source-blocks-1` (`lib.rs`, the non-composite branch), and its ordinary cases keep precision-1 row semantics (Rust `SOURCE_BLOCKS_ORDINARY_CASE_LEGACY_SEMANTICS`, `RULE_SOURCE_BLOCKS_SUMMARY_NOT_RELIABLE`). W1 records the source-block attempt in `legacy_source` and `legacy_source_work` (`retained_wire.rs`; the frozen reviewed SCHEMA). This is a choice between mechanisms, not plain removal, so it is not planned yet:
  - **(a) Keep source blocks, re-expressed.** The non-composite route publishes ordinary cases with preview-physics-1 rows, and `legacy_source` is renamed for what it records. Keeps exact rational recovery on the preview route. Moves `source_blocks_1` and `source_block_recovery.schema.json` (reviewed), the priced `source_recovery`/`source_receipt` types, the source-block corpora and the SCHEMA name in every successor and corpus base.
  - **(b) Retire the non-composite source route for W1.** W1 retained precision becomes the preview route's only retained mechanism, and `legacy_source` collapses and is deleted from the SCHEMA. Needs evidence that W1 covers source blocks' admitted family (straight, nodal, ≤ 256 DOF, no nonlinear supports or combinations); a case W1 cannot certify publishes ordinary-only, which is a possible capability loss. Moves the SCHEMA, every successor and corpus base, the `source_blocks_1` table, the source-block corpora and the readers.
  - Either way the exact-route composite identities (`physics-source-1`, `load-reference-source-1`, `pressure-source-1`) keep source blocks; only the preview-route use is in question.
