# RETURN — T1_WP2_DESKTOP_READERS

- **Role:** TASK (Type 2). I report to the T1 load-state WORKING_ITEMS manager.
- **Brief:** `TASK_BRIEFS/T1_WP2_DESKTOP_READERS.md`, read with `_T1_COMMON.md` (Wave 2). Binding manager rulings (a) to (e) also apply; they are in `T1_WAVE1_RULINGS.md` §12 and its ROOT notes.
- **Checkout:** the load-state worktree on branch `codex/piping-load-states-20260925`.
  - My base was `fe3e2dbbc`. HEAD is now `61fb7b219`.
  - The commits in between (WP2 native persistence, WP4 headless, and the ruling documents) change nothing under `apps/desktop/src/`. So "before" below means the same bytes at the base and at HEAD.
  - I made no Git writes.
- **Paths:** all paths are relative to `projects/chirality-piping/`. Run records are in `_run_records/`, with machine paths replaced by placeholders such as `<SCRATCH>`, `<SHARED_TARGET>` and `<REPO>`.
- **Status:** execution is complete. This is not acceptance.
  - Two checks do not fully pass. 32 of the 260 mutants survive; each is shown below to be equivalent or unreachable. The full desktop suite has timeout-only failures (see §3).
  - Items for the manager are in §6 and §7.

## 1. Files changed

The panels the brief excludes (Load Case Manager, Materials) and `src-tauri` are not touched.

These two files stay byte-identical, as ruling (a) requires:

| File | sha256 before | sha256 after |
|---|---|---|
| `apps/desktop/src/features/results/sourceBlockRecovery.ts` | `9c53c8cdbd34ba1d7154d305097f4cd0147d464aac9f7c5cd020e59d5b774d57` | identical |
| `apps/desktop/src/features/results/physicsSourceRecovery.ts` | `ef7ab4c9505825dff5085ee8f836d4e506da458fe67036dfb7ed6036fcd10e42` | identical |

Changed and new files:

| File | sha256 before | sha256 after |
|---|---|---|
| `apps/desktop/src/App.tsx` | `0f6f778f7d4d64620c3bea166111e2750dca472e3687c9eb35ba129ff59c5125` | `7273cd03adb5dc1d4863fa3999d96ad6dea18e646b5119a67c83933564952441` |
| `apps/desktop/src/features/adapter-framework/AdapterFrameworkPanel.tsx` | `e12808badcf75d523c603548805494824407f8a16b7d56fd412855d433796478` | `6d0b0f33b682250adf3a1eb28f6e19c7363466fb7bb65924c0ec9fe769632cfa` |
| `apps/desktop/src/features/caepipe-external/CaepipeExternalHarnessPanel.tsx` | `0dc6d8d5ae6410f23739ce9e8f25e480909378008f5d27e6a81b923be867c9c1` | `ebfab6c2b90c70df7703dcbe8e34bbd86453c2b3c34bb800e0f66259b3ae0575` |
| `apps/desktop/src/features/caepipe-mbf/CaepipeMbfExportPanel.tsx` | `4a40efc8f28eb16ac71d4dc77d2639db31add3dbc0bab179e967278aeb3a7149` | `08e673b020df8ad5ed4cecd886c7b9164d363da0b17ed988e0a28c85fd0c1f42` |
| `apps/desktop/src/features/design-workspace/DesignWorkspacePanel.tsx` | `17ede84dfff962cb216c59be50f478ab4913713bbc70830bbd31ba52d053aaae` | `e7a3deaf07b76c2da39265a5ddf9749196c408862454cdc18ecf0926ee31447c` |
| `apps/desktop/src/features/diff-preview/DiffPreviewPanel.tsx` | `1377370e5d700544324d054016a67056fe882510c69c4f8d327e79cc38a1a28b` | `d1ca89c3e8bb021255623066b4c7266e758475beeb4d9b5975d89fa9c7956b6a` |
| `apps/desktop/src/features/export-adapter-sdk/ExportAdapterSdkPanel.tsx` | `808ff52f1fca4ef8ef88e7e5ca4dceda5d14d15864963039ac80d45715e192f2` | `057aaa4c20337c08a31080250de15d5886bcb66ddfdbd21b44b11d3b16d71cb5` |
| `apps/desktop/src/features/export-review/ExportReviewPanel.tsx` | `b8ddf121628e11f8ba5d43177b85c1e8f33da4ae34cc36c57c3bbd4818a416c3` | `c42d459fcc7dcbd3922d92fdc9e8a20677eb1df22fcb9270d6abb0e32b75c489` |
| `apps/desktop/src/features/external-prover/ExternalProverBoundaryPanel.tsx` | `6c6e51fc6d98b6ea9ebb9dc86d0d3abdd952e77b7f315f8c92182c3b78a8bd17` | `b1bf52febaaee324291c4d76b77494d1581cbd4ee1eb8fdf550b733f4d47f28f` |
| `apps/desktop/src/features/handoff/HandoffPanel.tsx` | `7f29f982980e051f5acd19a712155716232243eff29db4d521a9bbb9941cccb9` | `b5c4a37ff15bee80fc9a3437a05d88f35e2c006aee0bbe9462c22d479972c1b9` |
| `apps/desktop/src/features/headless-runner/HeadlessRunnerPanel.tsx` | `35f6e719aade8625af744e7aa7dd9bdd7d5b132b8d59c17828481b7d6f40ce35` | `bb9358219ccf629b4027a67b596f28089b75a02ff59bdd90ba07c34766a05f55` |
| `apps/desktop/src/features/local-fea-handoff/LocalFeaHandoffPanel.tsx` | `08a9ad96a5f8b231b9305eb79c89678bbd7357639cb9c0e7ba4af25e487ead17` | `814e5116c4c409b375847c980df8d4296d6506c2fbb294a4b4180d6c78f08bf3` |
| `apps/desktop/src/features/missing-data/MissingDataBlockingPanel.tsx` | `38733e9ebc4247c8bd3f46139feda99d83f032dd0659aeae89cdc0b88cb4b245` | `aad93d322ca0fb1d3fa269613b30bf8bc1f18060574253780c6444657d829c03` |
| `apps/desktop/src/features/native-package/NativePackagePanel.tsx` | `e8319df2592f1f325bc7ba4b27bf40c1a139f3a3608ccff5602f9627f8d4dc72` | `d8894dabc0db97fe33a0b32c9fd0e335624c1cafa2f77fc31575f83f1b96d5c8` |
| `apps/desktop/src/features/operations/OperationLedgerPanel.tsx` | `717069282056ce4a8f420eab3776af8eb2d7ef50918e2fc02ad766ada850d6f0` | `a3620fb7757951821e81932ded2a48a4ecd42717cb17a5f9fd1aff8baefe0c72` |
| `apps/desktop/src/features/pcf-export/PcfExportPanel.tsx` | `e2e1e326fc5dfdc40d8a6e33d3b3d97be62d400a0d42d7546bb03905d577c34c` | `1ac7f24d4985a79394d1506642c5eaa380fafa593d2d75b51c603010dda4035d` |
| `apps/desktop/src/features/report-lint/ReportLintPanel.tsx` | `8f948277289f2fdfd1e0834835f1c6dc96790b6cb91628140ec3b0e1c1a0448b` | `3027f3db82db0c7640663ed174e2439017bbc788cd61dc5eb0e3e2c496f13009` |
| `apps/desktop/src/features/report/RenderedReportPanel.tsx` | `846accdb86db0045e34d64a57233c74ca4cde2eabb63f7062a9fba9d356d931f` | `6f7561db055ddc2668655b1e6eda56c2ee1830c57a8e51ed906ca31efb8a4b92` |
| `apps/desktop/src/features/report/ReportPanel.tsx` | `9ea6809c0ba4e704b6679df16a905036b1536ea9fee1fd3d08be37b4e33a6c46` | `03f12d7041d7fa6ad70e5945a52c5b690ee3ea5875c057fe7c71136b506ac1ce` |
| `apps/desktop/src/features/result-export/ResultExportPanel.tsx` | `df3c45d15328b40682ed24270c2cb3e2f2f967fb902b9cc461b27b2a1c8f0a12` | `6131bd1792bbe572b3a1e51b8c419904f59bdfb5fc26e29beb3fb66137094bf5` |
| `apps/desktop/src/features/result-export/resultExportAdapter.ts` | `748be07de27c7673c170f7acf9ff9934984a1b6e80eba8c6060d19a9ba3d062b` | `2473cc21cc85e1fe2fa4b7e2d863e1aca08dc390268404a94dc899e7a7d2cdc1` |
| `apps/desktop/src/features/results/numericalResultQuality.ts` | `f4784fb694355bd0aa7f50f5253691c49667134c3315e87679113616cb9af24d` | `19682a56a72c433f52cc584e3b66fb78f1c9356fcb1fcc3abfdcc46cc994254a` |
| `apps/desktop/src/features/results/resultSemantics.ts` | `8abd3154a987c894f235f80242d08d1c18e08102448a57ff8d7ce6a9cba13afd` | `e0696f374c33e8e918f02f749f9e8f76b1eef1ad11befaefde2a25f0f74ea2ab` |
| `apps/desktop/src/features/review-geometry/ReviewGeometryPanel.tsx` | `37eb1ae08e8edf73495ec3c4106e2358f82a038f4b46e722fd0449695c08e67b` | `73d75e90b3d8da6bbe732e5aa8197f5ccf4331f91f81599cb3145d5f2de748ce` |
| `apps/desktop/src/features/rule-check/RuleCheckPanel.tsx` | `a3d946807f80fa082e97a8db5f35092088561ccda57d0f94d89b46600d28a557` | `035b289d18011deb030b7b6ce28f7c1421c584e2dd25802d07f56172afad423e` |
| `apps/desktop/src/features/solve/SolvePanel.tsx` | `e5bd20ba1ba873c7add676bca9d9be189abbe31b338c658e8f89217968f0f6a1` | `73bba1ca4a29763655b2dd2e4f31a72f3f1e2450822f3bb29d7c686b397947f9` |
| `apps/desktop/src/features/stress-neutral/StressNeutralExportPanel.tsx` | `c76e778166a22a8d046d27e05558deee5060db6413f05cf8520bc2704a25c214` | `a40b088a8e6828f27797dada594c763f1cade439fa5a581a287cac4353964a94` |
| `apps/desktop/src/services/analysisRunCompatibility.ts` | `eb268074dc48c3d230877c7c936d089c270d2175746111c31c51c26d35512b15` | `0f1d22df9adcee33680ba7895a3316fde3d5dc390e451ef3d1b5d55f3b4c042e` |
| `apps/desktop/src/services/previewService.ts` | `93c8bebc85a4e551782abc815555f346d73f4c61cc2de335c7fc4e96786267f4` | `5d8a7443ca67166d63c7ac6cef8836dfaf8d8b504adb19a91e87622c84662ce2` |
| `apps/desktop/src/services/projectService.ts` | `d0305330b8c4d428347896ba99332800815a98cbc084aa19faa492bfe74d6c1b` | `7d9b644c78dedeec954a14ba934a46505a26e83ef8a4426a13beb425584a5293` |
| `apps/desktop/src/types.ts` | `f2d9580a7a711f037826895941cd0120ef27e486e006a0d4c79901024f32400c` | `9173faf0c4100b963f6e72790f18a25ad69d4a593523b25ca66067ccd59eb58d` |
| `apps/desktop/src/features/results/LoadReferenceOutputGate.tsx` | (new) | `bd78b8065d51c08267006d300e50d3bf53806f054a6a2d8a14a70b97f646c70d` |
| `apps/desktop/src/features/results/loadReferenceEvidence.parity.test.ts` | (new) | `8bd7f3df6fff1f23fac3331b700fe25f83150b0e5ef2937b1c2405d78b0a10f7` |
| `apps/desktop/src/features/results/loadReferenceEvidence.ts` | (new) | `8b04eb1a512ce60e8940e91b274f6c9dd77e9d2fd7d895f52a4972da5b93811f` |
| `apps/desktop/src/features/results/loadReferenceOutputAvailability.ts` | (new) | `4dcc9ad4e16b3171a1d3bb54d2667c713c97acf0799fc2921d0b37f009ff62b9` |
| `apps/desktop/src/features/results/loadReferenceOutputRefusal.test.tsx` | (new) | `6158acf8e0fba27f4c895cab475472423c2b83ae7920ebf299ec1f795cc2efea` |
| `apps/desktop/src/features/results/loadReferenceReaderCases.cases.json` | (new) | `5fa80299c64c59bab444def1a7ec2dea43fc07d52ae3d90c5c6f0a342a9f6811` |
| `apps/desktop/src/features/results/loadReferenceReaderCases.test.ts` | (new) | `40dca013c33df82a09f92a6406570a1ae96a532e7189fdafa05bf70d36058550` |
| `apps/desktop/src/features/results/loadReferenceReaders.test.ts` | (new) | `81b4f5a9c32ce321cf996b69907d1bbb55f1162c62089d3d653f9de1a5a01d4b` |
| `apps/desktop/src/features/results/loadReferenceSourceEvidence.ts` | (new) | `295da88658cd88b724196d3144384a480c484768c1e20318a0698457b4c5ee62` |
| `apps/desktop/src/features/results/loadReferenceSourceLedger.cases.json` | (new) | `04cc26ba25585393a80c5b6b328ee3809fd27c95c5e05988afb77e9a81bbb031` |
| `apps/desktop/src/features/results/loadReferenceSourceLedger.test.ts` | (new) | `d125bb7e56378cf7b24a7b8e9f5d800c2b86cdfa365f905ce8067d8fd379822e` |
| `apps/desktop/src/features/workspace/loadReference.resultsSessionState.test.ts` | (new) | `d5b2826d0ec95dd18940e56cef872ea88aea694e0b761cb4a1d47bc677212527` |
| `apps/desktop/src/services/loadReferenceAnalysisRun.test.ts` | (new) | `ede98cc8c43ad3762924731fcaffd68553d810f2f11603e7fcf7ac465bdfdcd0` |
| `apps/desktop/src/services/loadReferenceRoundTrip.test.ts` | (new) | `d767cf2066e7e7636ee89ac111e60107a64b87e19a4dc8f38484eecb32c409fd` |

## 2. What each change does

### Types — `types.ts`

- **Lossless 0.4.0 types.** The shapes follow `core/product_physics/src/case_state/input.rs` and `schemas/load_reference_state.schema.json`:
  - `ReferenceConfigurationInput` (with its member and support-motion records);
  - `ExpansionLawInput` (with its points);
  - `AnalysisStateInput`, with every closed variant of the fit and thermal definitions;
  - the result records `LoadReferenceStateRecord` and `LoadReferenceContractEvidence`.
- **Where they attach.**
  - `PreviewModel.reference_configurations?` is typed as nullable because the Rust `Option` serializes as `null`.
  - `materials[].expansion_laws?` and `load_cases[].analysis_state?`.
- **No projection.** All the new keys are optional pass-through fields, and nothing projects them away. `services/loadReferenceRoundTrip.test.ts` proves byte-exact round trips for the committed 0.4.0 request models through every path a model or result takes: browser persistence, the project envelope, `cloneModel`, the input manifest, the IPC capture, and the wasm operation route (compared canonically, see §6.3).

### Readers

- **`features/results/loadReferenceEvidence.ts` (new)** ports Python `load_reference_evidence.py`, check for check:
  - the pre-pass S1–S13 (shared with the joined reader through `method`);
  - the projection to physics-1;
  - the physics-1 validator, whose failures are wrapped as `SOURCE_LOAD_REFERENCE_PHYSICS_EVIDENCE: …`;
  - `_guarded`, which reports any foreign exception as `SOURCE_LOAD_REFERENCE_MALFORMED`;
  - the pinned table and transport-schema bytes (`verify…`);
  - `schemaShape`, a port of the Python `_shape` interpreter used for the transport metadata.
- **`features/results/loadReferenceSourceEvidence.ts` (new)** ports `load_reference_source.py`:
  - J0 is the identity and profile check.
  - J1 is the joined pre-pass.
  - J2 checks the receipt: the policy, then the closed physics-source-1 receipt shape with only the policy substituted, then the receipt and publication hashes, case order, method, mode, and the per-case physical hash over `load_reference_source_case_evidence_v1`.
  - J3 projects to physics-source-1 and runs the transport check, `validatePhysicsSourceEvidence`, and the invocation-free source-blocks ledger port `validateSourceBlocksComposite` (ruling (a); the check table is in §4.3).
  - `validateLoadReferenceSourceEvidence` resolves to `false` (admitted, never numerically eligible). It validates a snapshot taken at call time and registers its outcome against the source's checked-JSON fingerprint.
  - `loadReferenceSourceStanding` returns one of three findings: the registered error, `SOURCE_LOAD_REFERENCE_SOURCE_VALIDATION_REQUIRED` (not validated, or edited since), or `LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE_IN_T1`.
- **`features/results/numericalResultQuality.ts`** adds explicit `sourceContract` dispatch for the two identities:
  - `load_reference`: the table sha `44bc41c0…4f4d`, profile `resolved_straight_load_state_v1`, contract evidence must be an object, and no `source_block_recovery` or `carrier_evidence`.
  - `load_reference_source`: the table sha `d1628194…8337`, profile `resolved_straight_load_state_source_v1`, contract evidence must be an object, and the joined receipt shape is required.
  - Anything else, including relabelled or cross-profile headers, stays `unsupported`.
  - **Standing for `load_reference`:** the reader runs first (its error becomes a finding), then T0R's generic standing, unchanged.
  - **Standing for `load_reference_source`:** the declared T1 early return `needs_recompute`, placed after validation and after T0R's standing reason (none applies to this identity).
- **`features/results/resultSemantics.ts`:** each identity reads its own pinned semantic table (`fixtures/results/semantic_contract_v0_3_load_reference{,_source}_1.json`).
- **`services/previewService.ts`:** a load-reference source registers as a native invocation only after its reader passes. The joined reader is awaited.
- **`services/analysisRunCompatibility.ts`:** the readers run before a v0.3 AnalysisRun record is built or validated, as Python `_source_contract` does.
  - `load-reference-1` is recorded like physics-1, with no retained namespace. A record that carries retained evidence is refused (`ANALYSIS_PHYSICS_SOURCE_DOWNGRADE_FORBIDDEN`).
  - `load-reference-source-1` is recorded like physics-source-1: it retains `contract_evidence` and `source_block_recovery`, and both are checked on validation.
- **`services/projectService.ts` (ruling (c)):** the browser mirror of native `c5ec4dcba`.
  - 0.4.0 is `current`, with `target_schema_version` "0.4.0". No migrated document is produced and nothing is injected.
  - 0.4.1 and later remain `newer_than_supported`. 0.3.0 is unchanged (§7).

### Output refusals (ruling (d))

- **`features/results/loadReferenceOutputAvailability.ts` (new)** holds the one shared function and one reason:
  - `LOAD-REFERENCE-OUTPUT-NOT-YET-AVAILABLE: Output of load/reference-state results (load-reference-1 and load-reference-source-1) is not yet available on the desktop; it is routed to T6. The result remains readable here; this is not a finding about the result.`
  - Helpers: `isLoadReferenceRoute`, `loadReferenceOutputRefusal`, `refuseLoadReferenceOutput`.
- **`features/results/LoadReferenceOutputGate.tsx` (new)** shows that reason in place of a surface's download or handoff controls. The display stays.
- **The 21 gated panels.** Each wraps its download or handoff in the gate. `RenderedReportPanel` also returns early in `onRender`. `DiffPreviewPanel` and `OperationLedgerPanel` gain an optional `result` prop, which `App.tsx` passes.
- **Stress-neutral and result export** keep their own refusal points, with the same reason, in the builder, the validator and the panel:
  - `StressNeutralExportPanel.tsx` (the builder, both validators, the live binding and the panel);
  - `resultExportAdapter.ts` (derive, validate and build) and `ResultExportPanel.tsx` (the live binding and the finding).
- **The report package** keeps T0R's `REPORT-PACKAGE-FRESH-RESULT-UNAVAILABLE` / `N_REPORT`.

### Tests (all new; all values invented)

| File | Tests | Proves |
|---|---|---|
| `features/results/loadReferenceEvidence.parity.test.ts` | 368 | Per-case three-language parity over the committed raws and both mutation corpora. It fails on any disagreement. |
| `features/results/loadReferenceReaders.test.ts` | 38 | Dispatch, profile binding, receipt shape, pinned tables, standing order, T1 early return, and lossless projection. |
| `features/results/loadReferenceReaderCases.test.ts` + `.cases.json` | 20 | 14 targeted load-reference-1 refusals, each equal to Python's code; guards reached only by non-JSON values; `at`/`guarded` semantics; transport-schema byte pin; the joined snapshot-at-call-time registration. |
| `features/results/loadReferenceSourceLedger.test.ts` + `.cases.json` | 109 | 108 ledger cases, each equal to Python `_validate_source_blocks(doc, None, context=physics_source)`, plus a coverage test. |
| `features/workspace/loadReference.resultsSessionState.test.ts` | 7 | Current, and loss of Current after a `reference_configurations` or `analysis_state` edit; sensitive results; a tampered source never registers; LR-1 rule binding identical to physics-1; a joined result refused through standing. |
| `services/loadReferenceRoundTrip.test.ts` | 8 | The browser 0.4.0 mirror; byte-exact persistence, clone, manifest, IPC and wasm round trips. |
| `services/loadReferenceAnalysisRun.test.ts` | 20 | The desktop AnalysisRun equals the Python `build_analysis_run_v0_3` carriers by canonical hash (normalization in §6.4); forged records are refused. |
| `features/results/loadReferenceOutputRefusal.test.tsx` | 67 | Each of the 21 gated panels refuses both identities with the shared reason and leaves physics-1 unchanged; stress-neutral and result-export refusal points; the report package T0R reason. |

## 3. Checks

| Check | Command (from `projects/chirality-piping/`, Node 24) | Result |
|---|---|---|
| New desktop test files | `npx vitest run <file>` in `apps/desktop` | 368 / 38 / 20 / 109 / 7 / 8 / 20 / 67 passed |
| Full desktop suite at base `fe3e2dbbc` | `npx vitest run` (scratch `git archive` copy) | run 1: 121 files, 2138 tests; 7 failed / 2131 passed. Run 2: 9 failed / 2129 passed. All failures are timeouts under load (see below). |
| Full desktop suite, this change | `npx vitest run` (scratch copy synced to the worktree) | 129 files, 2775 tests (+8 files, +637 tests); 3 failed / 2772 passed. Two are 30 s timeouts in `App.test.tsx` (renders the workspace) and `physicsSourceIntegration.test.tsx` (mixed dense), the same class the base shows. Both files pass in isolation on mine and on base (225/225 and 17/17). The third was a real failure caused by my change: `sessionBoundary.test.ts` rejected my session test importing `./resultsSessionState`. I renamed the test to `features/workspace/loadReference.resultsSessionState.test.ts`, the form the guard admits, and the guard plus the renamed file then pass (2 files, 10 tests). `_run_records/desktop_checks.log` |
| Type check and production build | `npm run build --workspace apps/desktop` | passed (`tsc -b && vite build`, exit 0; the only warning is vite's existing chunk-size warning) |
| Three-language parity | the parity test with `LOAD_REFERENCE_PARITY_OUT` / `LOAD_REFERENCE_PARITY_COMPARE`, then `_run_records/parity_logs/summarize_parity.py` | lr: 228 entries, 438 compared outcomes agree, 0 disagree. lrs: 136 entries, 282 agree, 0 disagree. |
| Rust peers | `cargo test -p … --test load_reference_contract --test load_reference_source_contract` (shared target, `CARGO_INCREMENTAL=0`) | 5 + 5 passed (`_run_records/rust_tests_summary.log`) |
| Python peers | the `load_reference*` reader suites (venv, `-p no:cacheprovider`) | 392 passed |
| Ledger cases in Python | `PYTHONPATH=. python _run_records/make_ledger_cases.py <cases.json>` | 108 cases (`_run_records/ledger_python.log`). 11 are marked pre-empted, where both languages stop at an earlier check. |
| Reader cases in Python | `PYTHONPATH=. python _run_records/make_reader_cases.py <cases.json>` | 14 cases plus 2 non-JSON probes (`_run_records/reader_cases_python.log`) |

The baseline failures are the same class of timeout under shared-CPU load (4 cores, load average about 3–6) in files unrelated to this change. The base runs failed in `App.deadControls.test.tsx` (600 s), `App.test.tsx` and `physicsSourceIntegration.test.tsx` (30 s). No timeout was raised and no test was skipped.

### Out-of-scope paths (ruling (e))

`git diff --stat HEAD -- core/runner/headless tools/validation/qualification_load_reference.py core/handoff/stress_neutral ':(glob)**/*.py'` prints nothing, and `git status --porcelain` on those paths is empty. My change touches none of them. The WP4 changes under `core/runner/` are committed in `61fb7b219`; they are not mine and not in my diff. No Python, no packager and no committed joined package was touched.

I did not run the VP-STATIC scratch rerun. Nothing I changed feeds it: it is Rust, Python and headless only, and my change is `apps/desktop/src/**`. It should still give 507/507 in both modes. The manager runs it at integration, as ROOT's condition requires.

## 4. Mutation evidence

The runs used a scratch copy made with `git archive` and then synced with the worktree `src`, never in place. The script is `_run_records/run_mutants.py`. Results are in `_run_records/mutation_results.json`, and the four run logs are in `_run_records/mutation_logs/`.

A mutant is killed when any selected test fails. One mutant (`ROW_DEPENDENCY_CYCLE` removed) turns the dependency walk into an endless loop. Its test run never terminates, so I stopped it by hand after 1197 s and count it as detected. The script now stops such a run after 900 s; this is a mutant-run limit, not a test timeout.

| Group | What is mutated | Mutants | Killed | Survived |
|---|---|---|---|---|
| R | every load-reference-1 reader refusal (`require_` → true; each `throw` → no throw) | 112 | 105 | 7 |
| J | every joined reader refusal, including J2 | 13 | 12 | 1 |
| L | every ported ledger check (`blocks`/`composite` → true; each `unique(...)` removed) | 119 | 95 | 24 |
| D | dispatch and standing: the joined early return, the LR standing reader, both header profile/evidence guards, the receipt-shape guard and function, the joined registration binding, native registration | 8 | 8 | 0 |
| P | lossless-projection and retention guards: the browser 0.4.0 retention, the joined AnalysisRun evidence and receipt retention, the LR projection copy, the joined validation snapshot | 5 | 5 | 0 |
| O | the shared output refusal, the stress-neutral builder refusal, the result-export builder refusal | 3 | 3 | 0 |
| **Total** | | **260** | **228** | **32** |

The shared-refusal mutant, re-run without `--bail`, is caught by 45 tests (`mutation_logs/shared_refusal_without_bail.log`).

The D-group mutants ran against the session test under its former name, `features/workspace/loadReferenceSession.test.ts`. I renamed it later (§3) with no change to its content. The `tests` field in `mutation_results.json` keeps the former name, while `run_mutants.py` now names the new file.

### 4.1 Survivors: each is equivalent or unreachable

**R group, load-reference-1 reader (7).** All seven mirror Python line for line.

| Site | Why no input can kill it |
|---|---|
| `loadReferenceEvidence.ts:505` `RECORD_CASE_UNRESOLVED` (R12) | Unreachable. S9 `CASE_COVERAGE` already makes the record ids equal the case ids (solved), and an unsolved envelope must have no records. |
| `:343` `RECORD_CASE_UNRESOLVED` (S10b, joined) | Unreachable, for the same reason; S10 runs first. |
| `:513` `MEMBER_MATERIAL_COVERAGE` (material not found) | Unreachable. The set equality at `:510` guarantees a material with each member's pipe id. `text()` forbids the empty string that stands in for a non-string id. |
| `:531` `MEMBER_CONTRIBUTION_COVERAGE`, `:538` `SUPPORT_CONTRIBUTION_COVERAGE` (contribution not found) | Unreachable. The set equalities at `:527` and `:535` guarantee a contribution with that source id. |
| `:368` `REGION_MATERIAL_BINDING` (member not found) | Reachable: reader case `REGION_MATERIAL_UNBOUND_PIPE` reaches it, and Python agrees. But the mutant is behaviourally equivalent: the next check, `same(…, without(undefined))`, raises the same code. |
| `:226` `SOURCE_LOAD_REFERENCE_TABLE_IDENTITY` | Unreachable. It runs only after the sha256 of the bytes equals the pinned table sha, and those bytes carry the right identity. |

**J group (1).** `loadReferenceSourceEvidence.ts:67` `SOURCE_TABLE_IDENTITY` is the same case as `:226`.

**L group, ledger (24).** Each is unreachable on the physics-source-1 route in both languages, and the ledger case log shows it.

- **The closed receipt schema rejects the input first** (`PHYSICS_SOURCE_RECEIPT_SHAPE`):
  - `PROJECTION_CRITERION` (`:337`): `relative_limit` is const 1e-9 and the bound's maximum is 1e-9.
  - `FAILED_OUTCOME` (`:590`), `FAILURE_BLOCK_ORDER` (`:591`) and `FAILURE_CATEGORY` (`:593`): the case `outcome` is const "qualified".
  - `SPRING_ATTRIBUTION_DUPLICATE` (`:457`): support identities are unique, dof mod 6 fixes the slot, and the schema caps components.
  - `DERIVED_RECIPE` (`:367`), `SUPPORT_NORM_INPUTS` (`:359`), `STRESS_RECIPE_INPUTS` (`:361`), `SUMMARY_RECIPE_INPUTS` (`:363`) and `SECTION_RECIPE_KIND` (`:365`): the `recipe_id` enum admits only `translation_norm_scaled_v1` and the four composite recipes. The composite recipes go to `validatePhysicsSourceDerived`, which is unchanged.
  - The stress and summary observation checks are reachable only through `straight_open_stress_v1` or `reviewed_stress_summary_v1`, so these recipe cases cover them too: `STRESS_ACTION_BINDING` (`:390`), `STRESS_ACTION_RANGE` (`:394`), `STRESS_OUTPUT_RANGE` (`:396`), `STRESS_ACTION_SIGN` (`:398`), `STRESS_PA_OBSERVATION_RANGE` (`:399`), `SUMMARY_STRESS_COVERAGE` (`:402`, `:407`), `SUMMARY_INPUT_RANGE` (`:409`), `SUMMARY_PA_OBSERVATION_RANGE` (`:411`), `SUMMARY_SUBTOTAL_RANGE` (`:422`, `:424`) and `SUMMARY_OUTPUT_RANGE` (`:427`).
  - Pre-empted cases recorded: `FAILED_OUTCOME`, `FAILURE_BLOCK_ORDER`, `FAILURE_CATEGORY`, `PROJECTION_CRITERION`, `DERIVED_RECIPE`, `SPRING_ATTRIBUTION_DUPLICATE`, `SUPPORT_NORM_INPUTS`, `STRESS_RECIPE_INPUTS`, `SUMMARY_RECIPE_INPUTS`, `SECTION_RECIPE_KIND`.
- **An earlier check rejects the input first.**
  - `DERIVED_NORM_RANGE`, first site (`:375`, non-finite input values): every row value must already pass `VALUE_NONFINITE`, and JSON has no non-finite number. The second site (`:383`) is killed.
  - `UNSELECTED_SUPPORT_CERTIFICATE` (`:660`): the outcome is always "qualified", and for a non-exact method `ORDINARY_SELECTION` already requires no supports. The recorded case `UNSELECTED_SUPPORT_CERTIFICATE` gives `SOURCE_BLOCKS_ORDINARY_SELECTION` in both languages.

**Correction to my earlier plan.** I had recorded five further ledger checks as unreachable: `SUMMARY_UNSOURCED_HEADLINE`, `ROW_DEPENDENCY_CYCLE`, `NORM_TOTAL_RELATIVE_BOUND`, `AGGREGATE_STATUS` and `PHYSICAL_OBSERVATION_ESCAPE`. When I probed them in Python, a concrete input reached each one. They now have ledger cases and all five mutants are killed. `EVIDENCE_IDS` (the first site) and `SOURCE_PRESSURE_RHS` also gained cases and are killed.

### 4.2 Reader cases added for the mutation run

`make_reader_cases.py` → `loadReferenceReaderCases.cases.json`. Each case's code was produced by Python on the same bytes:

- `NUMERICAL_CASE_DUPLICATE`, `MATERIAL_DUPLICATE`, `SECTION_DUPLICATE`;
- `MEMBER_MATERIAL_COVERAGE`, `MEMBER_CONTRIBUTION_COVERAGE`, `SUPPORT_CONTRIBUTION_COVERAGE`;
- the point `TEMPERATURE_RANGE`, `MATERIAL_POINT_RANGE`, the direct `THERMAL_LAW_BINDING`, `REFERENCE_LENGTH`;
- `ARRAY_INVALID`, `REGION_CASE_UNRESOLVED`, `REGION_MATERIAL_BINDING`, `SUPPORT_COMPONENT_DOF` (name).

Guards that no JSON value reaches are mirrored by hand. Python's counterparts are the two probes in `reader_cases_python.log`:

- a non-JSON value raises `NUMBER_INVALID`;
- a hostile mapping raises `MALFORMED`, for both readers.

### 4.3 Ledger port, check for check (ruling (a))

`_run_records/ledger_check_table.md` is generated by `make_ledger_check_table.py` from the Python source, the TS source, the case file and the mutation results. It lists every check that Python `source_blocks._validate_source_blocks(doc, None, context=physics_source)` (with `physics_source.validate_source_case`) can raise, in source order. For each it gives:

- the Python site;
- the desktop line or lines raising the same code;
- the ledger cases whose Python outcome is that code;
- each desktop site's mutant status.

Summary:

- Every check that is evaluated without an invocation has a desktop counterpart. Every counterpart has at least one mutant, and each mutant is killed or listed in §4.1.
- 17 checks run only under `actual_invocation`, `model`, `actual` or `invocation` not None, so the invocation-free route never evaluates them. They are not ported, and the table marks them: `ACTUAL_INVOCATION_SHAPE`, `INVOCATION_HASH`, `CURRENT_MODEL*`, `REQUESTED_CASE_COVERAGE`, `REQUESTED_MODE`, `SUPPORT_NODE`, `ACTUAL_SUPPORT_LAW`, `NODAL_*_COVERAGE` and the `ACTUAL_*` physics-source checks.
- No check had to be approximated.
- The two derived-recipe families of the composite (`validatePhysicsSourceDerived`) and the transport and evidence checks come from the unchanged `physicsSourceRecovery.ts` and `physicsResultEvidence.ts`.

## 5. Output surfaces (ruling (d))

Groups: (a) result data enters an output, so the surface refuses; (b) model, project or geometry data only, left alone; (c) display or analysis only, left alone.

"Result data" is any datum taken from the result or its AnalysisRun record: values, units, statuses, diagnostics, row ids, run ids, or result and run references. Three panels carry only identifiers: Review geometry, Operation diff preview and Operation review ledger. I could not place them in (b) with confidence, so I refused them under the uncertainty rule and flag them here (§7, question 1).

Every refusal below is **open work routed to T6**, not an unresolved gap.

- **Python status:** these are desktop-only surfaces with no Python counterpart, except as noted. The Python stress-neutral packager (`core/handoff/stress_neutral`) accepts both identities. The Rust `result_export` peer reads both identities.
- **Tests:** each (a) panel has one refusal test per identity plus a physics-1 control in `loadReferenceOutputRefusal.test.tsx`.

### (a) Refused: one shared function, one reason

| Panel | Code evidence: where result data enters the output | Output (file:line) | Action |
|---|---|---|---|
| PCF export | `PcfExportPanel.tsx:187` passes `result` into `buildExportUnitSystemDisclosure`, which uses the result row units (`exportUnitDisclosure.ts:50`) | download `:94` | Gate `:90`, routed to T6 |
| CAEPIPE .mbf export | `CaepipeMbfExportPanel.tsx:200` `result_ref` (result `run_id`); `:166` AnalysisRun ref | download `:78` | Gate `:74`, routed to T6 |
| CAEPIPE external harness | `CaepipeExternalHarnessPanel.tsx:154` run id from `result.run_id`; `:187` `result_ref` | download `:56` | Gate `:52`, routed to T6 |
| Export adapter SDK | `ExportAdapterSdkPanel.tsx:151` result into the unit disclosure; `:247` `result_ref` | download `:70` | Gate `:66`, routed to T6 |
| Adapter framework | `AdapterFrameworkPanel.tsx:104` result diagnostics; `:164` result run id | download `:28` | Gate `:24`, routed to T6 |
| External prover boundary | `ExternalProverBoundaryPanel.tsx:109`, `:113`, `:118` result `run_id` | download `:32` | Gate `:28`, routed to T6 |
| Review geometry (identifiers only) | `ReviewGeometryPanel.tsx:172` `result_ref`; `:152` AnalysisRun ref | download `:75` | Gate `:71`, routed to T6 (uncertainty rule) |
| Missing-data blocking | `MissingDataBlockingPanel.tsx:141` result diagnostics; `:143` result status | download `:50` | Gate `:46`, routed to T6 |
| Design workspace | `DesignWorkspacePanel.tsx:161` result diagnostics; `:197`, `:201` result counts | download `:56` | Gate `:52`, routed to T6 |
| Rule-check completeness | `RuleCheckPanel.tsx:117–119` result statuses | download `:58` | Gate `:54`, routed to T6 |
| Report lint | `ReportLintPanel.tsx:122` result run id | download `:29` | Gate `:25`, routed to T6 |
| Solve job | `SolvePanel.tsx:28` result diagnostics; `:128–130` result statuses | download `:46` | Gate `:42`, routed to T6 |
| Headless runner (UI gate only) | `HeadlessRunnerPanel.tsx:120` result diagnostics; `:124` run `result_refs` | download `:35` | Gate `:31`, routed to T6. The CLI and runner binary are unaffected. |
| Local FEA handoff | `LocalFeaHandoffPanel.tsx:154–161` result regions, refs and envelope | handoff `:69` | Gate `:65`, routed to T6 |
| Native package | `NativePackagePanel.tsx:205` result row ids | download `:106` | Gate `:102`, routed to T6 |
| Handoff package | `HandoffPanel.tsx:184` result diagnostics; `:208–209` result rows | download `:89` | Gate `:85`, routed to T6 |
| Export safety review | `ExportReviewPanel.tsx:201` result diagnostics; `:474` result flags | download `:71` | Gate `:67`, routed to T6 |
| Report packet | `ReportPanel.tsx:47–50` result refs, diagnostics, units and provenance | download `:105` | Gate `:101`, routed to T6 |
| Rendered report | `RenderedReportPanel.tsx:75–78` `buildRenderableReportInput({ result, analysisRun })` | render/print `:69` | Gate `:70`, plus an early return in `onRender`; routed to T6 |
| Operation diff preview (identifiers only) | `DiffPreviewPanel.tsx:172` `analysis_run_ref` (run id) | download `:58` | Gate `:54`, routed to T6 (uncertainty rule) |
| Operation review ledger (identifiers only) | `OperationLedgerPanel.tsx:169`, `:243` `analysis_run_ref` and `model_state_ref` from the run | download `:52` | Gate `:48`, routed to T6 (uncertainty rule) |
| Stress-neutral export | the builder copies result evidence (`StressNeutralExportPanel.tsx:489` refusal first) | download (panel) | Refused in the builder `:489`, validators `:620` and `:1059`, live binding `:86`, panel `:207`; routed to T6. Python packager: accepts both identities. |
| Result export | `resultExportAdapter.ts:172` builds the result document | download (panel) | Refused in derive, validate and build (`:58`, `:118`, `:172`); live binding `ResultExportPanel.tsx:14`; routed to T6 |
| Report package request | `reportPackageRequest.ts` | request | Unchanged: T0R `REPORT-PACKAGE-FRESH-RESULT-UNAVAILABLE: N_REPORT` |

### (b) Left: model, project or geometry data only

| Panel | Evidence |
|---|---|
| Redaction export controls | Takes no result or run (`App.tsx:1011` mount). Its packet is built from the model only. |
| Project storage audit | `ProjectStorageAuditPanel.tsx:224` carries only `persisted_mechanics_result_count` from the project summary; no result is passed. |
| Project validation | `ProjectValidationPanel.tsx:296` carries the same count plus the model version check; no result is passed. |
| Offline proposal intake | `OfflineProposalIntakePanel.tsx:44` downloads the operation capability reference; no result. |
| Hanger selection, self-weight plan, geometry tools, boundary authoring, batch review, operation apply, editor contract, library manager, rule-pack manager, validation evidence, build readiness, telemetry, secret library, threat model, accessibility | Mounted without any result or run prop (`App.tsx:451–1011`). |

### (c) Left: display or analysis only (no download, handoff or request in the file)

| Panel | Evidence |
|---|---|
| Results, Comparison, Knowledge, Run audit, Diagnostics, Historical run | Receive the result for display (`App.tsx:620–918`, `:1447`, `:1499`). No download, `jsonDataHref`, `invoke`, `fetch` or clipboard write in the file. |
| RuleCheckRun | As agreed: LR-1 binds exactly as physics-1 (test `loadReference.resultsSessionState.test.ts`); a joined result is refused through standing. |
| Agent workbench, Agent proposal | Receive only `Boolean(result)` and status text (`App.tsx:470`, `:578`). |

**Persistence is not an output surface.** Project save and the browser snapshot keep the raw result and its resolved evidence, as brief item 4 requires. That is proved by the round-trip and session tests.

## 6. Findings and notes

1. **JavaScript number representation.** JS holds every JSON number as binary64 and has no JSON text for a non-finite value. A handful of direct-reader codes in the mutation corpora (listed in `TS_CODES` in the parity test, for example `NUM-unsafe-integer-in-publication` → `CHECKED-JSON-NUMBER-OUTSIDE-PROFILE`) therefore necessarily differ from Python's code. Their outcome (refuse) agrees in all three languages.
2. **Negative zero.** The JSON clone path loses `-0`. The joined registration therefore fingerprints the checked-JSON text *and* the negative-zero paths.
3. **The wasm operation route returns objects with sorted keys.** Content is equal, so that round trip is compared canonically, not byte for byte.
4. **AnalysisRun mirror (ruling (b)).** The desktop record equals the Python `build_analysis_run_v0_3` carrier (`fixtures/results/load_reference*.analysis_run.json`) by canonical hash for all 4 + 10 raws. Only fields in which the two builders already differ for every identity are normalized: `provenance`, `reproducibility.determinism_notes`, the `analysis_run_record` self-hash and `result_refs[].provenance`.
5. **Current consumers.** Once a `checks_passed` LR-1 result is Current, it reaches every panel given `currentSolvedResult` (`App.tsx`), and none of them dispatches on identity. That is why ruling (d) was requested.
6. **Browser 0.3.0 (ruling (c)).** The browser mirror still refuses 0.3.0 as `newer_than_supported`, and existing tests pin that. I checked for any other browser route that opens a 0.3.0 document; there is none:
   - Browser project create, save and open all go through `requireEditableModelDocument` (`projectService.ts:287`, `:577`).
   - The fixture model is 0.1.0.
   - Browser solve is unavailable.

   A 0.3.0 model can exist in browser memory only after the queued exact-pressure-profile edit (`PressureAuthoringPanel.tsx:5`, applied by the wasm engine). Saving it is then refused. Native treats 0.3.0 as current (WP2 persistence return), so the two surfaces differ; this is unchanged, as ruled.
7. **Python bytecode.** The Python reader modules were imported with the repo venv. Gitignored `__pycache__` files under `core/analysis_runs/` exist, and I cannot tell which run created them, so I left them in place. Later runs used `PYTHONDONTWRITEBYTECODE=1`.
8. **Wasm engine.** I built it from this tree (`npm run build:wasm --workspace apps/desktop`). Its output under `apps/desktop/public/` is gitignored and left in place.

## 7. Not done, and design questions

- **Not done:**
  - 32 mutants survive. All are shown in §4.1 to be equivalent or unreachable; I could not construct a killing input for any of them. The brief asks for every mutant to be killed. Keeping the checks follows ruling (a)'s check-for-check mirror.
  - I did not run the VP-STATIC rerun (reserved for the manager).
  - Every output refusal in §5 is open work routed to T6.
- **Design questions:**
  1. **Identifier-only packets.** Review geometry, Operation diff preview and Operation review ledger carry only a result or AnalysisRun reference, no result values. I refused them under the uncertainty rule. If a reference alone does not count as result data, the three gates can be removed (one line each, plus `result={result}` in `App.tsx` for the last two).
  2. **Unreachable ported checks.** Should the checks that the physics-source-1 schema makes unreachable stay in the desktop port (the current mirror), or be pruned in all languages together? Pruning only on the desktop would break check-for-check parity.
  3. **Browser vs native 0.3.0** (finding 6): whether the browser mirror should follow native's 0.3.0 `current` is outside this ruling; it is recorded for the owning loop.

## 8. Symlinks and scratch

I created six symlinks and have removed all of them. Each pointed at the sibling worktree's `node_modules`, which is left intact:

- in this worktree, `node_modules` and `apps/desktop/node_modules`;
- in each of my two scratch copies (base and mine), `node_modules` and `apps/desktop/node_modules`.

Both scratch copies are deleted. Free disk afterwards is 9.5 GB. The worktree has no untracked or modified file outside `apps/desktop/src/**` and this return folder.

## Manager integration note

- **§7 Q1, applying ROOT's guidance.** A result or AnalysisRun reference alone is not result data. The code shows that review geometry, operation diff preview and operation review ledger put only `result.run_id`, `analysis_run.run_id` and `analysis_run.model_state_ref` into their packets. The manager therefore reclassified them as group (c) and removed the three gates. `DiffPreviewPanel`, `OperationLedgerPanel` and `App.tsx` are back to their base bytes. The three packet builders are now exported for tests.
  - New tests in `loadReferenceOutputRefusal.test.tsx`: each packet is byte-identical when every result value is perturbed, and every AnalysisRun field other than those identifiers is perturbed, for both identities. The three downloads stay available.
  - Two leak mutants (a result summary added to the geometry packet, run hashes added to the ledger) are each caught by 2 tests.
  - The file now has 64 tests, all passing.
- **§7 Q2.** The ported checks stay: ruling (a) requires a check-for-check mirror. Pruning in every language together would be cross-language reader hardening, not T1 work.
- **§7 Q3.** Browser 0.3.0 is unchanged and routed to UI-SUCCESSOR (ROOT).
- **Survivors (§4.1).** They are not accepted until the combined wave-2 review verifies them (ROOT). Every reader survivor is checked, and the ledger survivors are sampled.
- **Manager checks on the integrated tree:**
  - desktop full vitest: 129 files, 2767 of 2772 passed. The 5 failures are all 30 s timeouts, in `App.test.tsx` (2) and `physicsSourceIntegration.test.tsx` (3), on a loaded host (load average 5–6 on 4 cores). Run in isolation, both files pass; the slowest cases are 21.3 s (App render) and 19.3 s (mixed dense). Per ROOT, these are not accepted as flakes: WP7's quiet-host sweep must pass with no timeout raised, or they are root-caused.
  - `npm run build`: passed.
  - VP-STATIC scratch rerun: 507/507 in both modes, with all runner I/O byte-identical to the recorded run (`LSI/_run_records/session5/wave2_vp_static_rerun.txt`).
