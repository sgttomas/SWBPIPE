# A1-APP — retained return

Retained by ROOT. What follows is the child's final message as the host stored it, with the machine's repository path replaced by `{REPO_ROOT}`; nothing else is changed.

---

Return for brief A1-APP (desktop visible rename). The brief's SHA-256 matched `19fe7ca2…6480`. I am Claude Fable 5.1 (`claude-fable-5-1`), TASK Type 2, working alone. I delegated nothing and ran no state-changing git command.

All five work items are done inside `apps/desktop/**`, and every check the brief names is green. You need to act on a Python test outside my scope that the `productName` change will break, and on the local-store location that the new bundle identifier moves. Details are under "Outside my scope".

## Checks
- **Baseline before edits:** `npm run test:desktop` gave 72 files, 1171 tests passed.
- **Unit tests after edits:** `npm run test:desktop` gave 73 files, 1184 tests passed.
- **Build:** `npm run build:desktop` is green; `npx tsc -b` is clean.
- **Playwright first run:** from `apps/desktop`, `PLAYWRIGHT_WORKERS=1 ../../node_modules/.bin/playwright test --project=chromium-desktop` gave 186 passed, 10 skipped, 1 failed.
  - The failure was `ui-foundation.spec.ts:1924` (decorative viewport overlays).
  - My footer removal caused it. The canvas grew, and the frozen Box16 endpoint at 0.81 of canvas height missed the gizmo by 2.4 px. I did not run the e2e lane on the base commit; that the test passed there is inferred from the geometry, not observed.
  - I left the endpoints, tolerances and assertions unchanged. I set that one test's window to 1440×899 instead of 920, so the canvas geometry matches what the endpoints were characterized against, and put a comment there.
- **Playwright final run:** 187 passed, 10 skipped, 0 failed.
- **Rust (not named in the brief, run because I edited `lib.rs`):** `cargo test --lib` in `src-tauri` gave 91 passed.
- **Dist lane not run:** I moved one pinned string in `e2e/ui-foundation-dist.spec.ts`, and that lane was not exercised.

## Files changed (62: 60 modified, 2 new)
- **Shell and packaging:**
  - `index.html`, `src/App.tsx`, `src/styles.css`
  - `src-tauri/tauri.conf.json`, `src-tauri/Cargo.toml`, `src-tauri/src/lib.rs`
- **New:** `src/features/workspace/statusLabels.ts` and `statusLabels.test.ts`.
- **Labels wired in:** `src/features/solve/SolvePanel.tsx` and `src/features/viewport/PipeViewport.tsx`.
- **Features (acceptance sentence, name and export copy)**, all under `src/features/`:
  - `adapter-framework`, `build-readiness`, `caepipe-external`, `caepipe-mbf`, `comparison`, `design-workspace`, `diff-preview`, `editor-contract`
  - `export-adapter-sdk`, `export-review`, `external-prover`, `handoff`, `headless-runner`, `library`, `local-fea-handoff`, `missing-data`
  - `model-tree` (`ModelTree` and `PropertyInspector`), `native-package`, `operations` (`OperationApplyPanel` and `OperationLedgerPanel`), `pcf-export`
  - `project-storage`, `project-validation`, `report-lint`, `report` (`ReportPanel.tsx` and `renderableReportInput.ts`)
  - `result-export` (`ResultExportPanel.tsx` and `resultExportAdapter.ts`), `results` (`ResultsPanel.tsx` and `resultInterpretation.ts`)
  - `review-geometry`, `rule-check` (`RuleCheckPanel` and `RuleCheckRunPanel`), `rule-packs`, `run-audit`, `security-threat-model`, `stress-neutral`, `telemetry`, `validation-evidence`
  - the single file `exportUnitDisclosure.ts`
- **Services:** `libraryImportService.ts`, `ruleCheckService.ts`, `rulePackService.ts`.
- **Unit tests:** `src/App.test.tsx`, `src/features/rule-check/RuleCheckRunPanel.test.tsx`, `src/features/library/LibraryManagerPanel.test.tsx`, `src/features/rule-packs/RulePackManagerPanel.test.tsx`.
- **e2e:** `gui-workflow-validation.spec.ts`, `linear-authoring.spec.ts`, `r2-smoke.spec.ts`, `ui-foundation-dist.spec.ts`, `ui-foundation.spec.ts`.

## What was done, by item

**1. `DEC-105`, the maturity sentence.**
- The sentence is gone from the footer, `BuildReadinessPanel`, the `ExportReviewPanel` review note, and a lowercase variant in a `ReportLintPanel` target text.
- The footer held only the two removed sentences, so I removed the `<footer>`, its three CSS rules, the header-comment mention and one `auto` track from the two legacy `grid-template-rows`. The effective layout is flex.
- The footer test now asserts that no footer, maturity sentence or acceptance sentence is rendered.

**2. `DEC-100`, the acceptance sentence.**
- All variants are removed from every product surface, with nothing put in their place.
- In `ReportLintPanel` the 20 target texts are scan-only descriptions, stripped before the packet is emitted. I dropped the clause from each; the lint logic is untouched.
- Four boundary summaries had the sentence as their entire "OK" value: `ReportPanel`, `RunAuditPanel`, `ResultExportPanel` and the static Boundary line in `ReportPanel`.
  - The static line is removed.
  - The three functions now return the data rendering `human_review_required=true; professional_claim=false`. The "requires attention" branches are unchanged.
  - Please confirm this form.
- I stopped rendering `professional_boundary_notice` in three places. The data is unchanged.
  - `RuleCheckRunPanel` (testid `rule-check-professional-boundary`)
  - `LibraryManagerPanel` import validation
  - `RulePackManagerPanel` validation
- `SolvePanel` also lost "no professional acceptance record", which was part of the same string.

**3. `DEC-102`, registered labels.**
- `statusLabels.ts` holds exactly the eight rows. The domains come from the design system's `tokens.json` labels table: Solver, Rule pack, Human and Evidence.
- `readableWorkspaceStatus` and both copies of `formatStatus` are replaced by the module. "Review required" and "Inputs needed" are gone.
- The status pill shows "Domain · Label" with the raw token in its body. Text runs show "Domain · Label (TOKEN)".
- These values keep readable text and get no registered form: "Not run" (`not_run`, `not_computed`), "Demo computed", and any other enum, lowercased with spaces (for example `ready_for_preview_diagnostics`).
- The two legacy fixture enums map to the token the product has always shown for them:
  - `not_performed_user_rule_inputs_missing` to `RULE_INPUTS_INCOMPLETE`
  - `not_provided` to `HUMAN_REVIEW_REQUIRED`
  - The mapping was in `App.tsx`; it is now in the module and `SolvePanel` uses it too.
- The pinned assertions are moved, and a unit test pins the eight forms (13 tests).
- Lowercase prose "human review required" remains, not as a status label:
  - `SolvePanel.tsx:363`
  - `ResultsPanel.tsx:188`
  - `reportPackageRequest.ts:268`
  - `renderableReportInput.ts:109`

**4. `DEC-101` (iii), the name.**
- `productName`, the window title, the `index.html` title, the `<h1>`, the macOS app menu, "Untitled SWBPIPE Project", the file-dialog filter name and the Cargo description are all SWBPIPE.
- The bundle identifier is `com.swbpipe.desktop`.
- Report titles drop "(Technical Preview)".
- The export-panel `source_name` and `contributor` strings are renamed, along with the review-geometry node names and the `BuildReadinessPanel` literals.
- I removed "technical preview" where prose called the product that, including the report limitation statement, which now begins "Output over invented or user-local data; …". Revert that one if you disagree.

**5. `DEC-103` item 9, the export copy.**
- The panel heading is "Model batch file (.mbf)" and its aria-label is "Model batch file (.mbf) export".
- The harness panel is "External run evidence" with "external run" wording. This is for the owner to replace.
- The vendor name is removed from notes, boundary notes, loss and diagnostic messages, the `$` comment header of the emitted `.mbf` text, and the download file names, which now use `…-mbf-package-…` and `…-external-run-…`.
- No compatibility flag is rendered in either panel. The `const false` data fields stay in the data.
- The export-review list omits the `document_kind` text for the two vendor-named kinds.
- The harness state-binding line displays `mbf:` in place of the `caepipe-mbf:` prefix.
- I removed the sentences "does not assert CAEPIPE compatibility" from the emitted boundary notes.

## Identity left deliberately
- **Provenance strings pinned by code or fixtures outside my scope:**
  - `source_name` "OpenPipeStress desktop session (…)" at `reportPackageRequest.ts:35`, `renderableReportInput.ts:75` and `stateComparisonHandoffSections.ts:96`. It is compared with `toEqual` against `fixtures/reports/invented/component_provenance_cross_layer_projection.json`, and `core/reporting/report_package/src/wire.rs` also loads that fixture.
  - "OpenPipeStress DEL-08-06 report-section assembler" and "OpenPipeStress Type 2 worker" at `stateComparisonHandoffSections.ts:15` and `:18`, mirroring `core/reporting/state_comparison_handoff_sections/engine.py:31`.
  - "OpenPipeStress analysis record 0.2" at `analysisRunCompatibility.ts:9`, mirroring `core/analysis_runs/compatibility.py:125` and a fixture.
- **Cargo metadata:** `src-tauri/Cargo.toml:5` `authors` (the brief limited that file to the description), plus the crate and lib names.
- **Document kinds, package name and events:**
  - every `openpipestress.technical_preview.*` document kind
  - `@openpipestress/desktop`
  - the `openpipestress-native-menu-command` event
- **Download prefix:** `openpipestress-preview-`, validated by `src-tauri/src/native_result_download.rs:12-13`.
- **Storage:** `.opsproj` and the store filename.
- **Vendor-named identifiers:** file, directory and component names, test ids, CSS classes, `profile_id`, `target_family`, `export_id`, witness and ref ids, the `CAEPIPE-*` SourceID refs, `DOTH-CAEPIPE-LOCAL-006`, code `CAEPIPE-EXTERNAL-PARSER-NO-ROWS`, and the fields `caepipe_compatibility_claim` and `software_makes_caepipe_compatibility_claim`.
- **Enum-like values containing technical_preview:**
  - `technical_preview_requires_human_engineering_review` (`lib.rs:3854`, compared in the App test)
  - `csp_status`, `profile_status` and `profile_id` values
  - `workflow_validation_status`
  - the `:technical-preview` suffix of `limitation_id`
- **Emitted-contract test and mocks:** `lib.rs:7003-7004` asserts the emitted `professional_boundary_notice` of the core crate, which is a data contract. The test mocks of that notice stay truthful to it in `RuleCheckRunPanel.test.tsx` and `App.test.tsx:17136`.

## Could not fully do
- The collapsed redaction "Details" list prints raw JSON paths. For the `.mbf` package it still shows the two vendor-named field names. The fix belongs with the identity tranche's field rename. The App test excludes only that list and asserts the rest of both panels has no "caepipe".
- `ReportLintPanel.tsx:933` remediation text, "Rewrite the text as decision-support evidence…", is left. It is not a registered acceptance text, but it is now stale advice.

## Outside my scope, for ROOT or a sibling
- **Will break:** `tests/test_release_packaging_script.py:515` expects `OpenPipeStress-Technical-Preview_0.1.0_aarch64-apple-darwin.app.zip`, and that name is derived from the real `tauri.conf.json` `productName`. It becomes `SWBPIPE_0.1.0_…`. Lines 211, 431 and 433 carry the old name and identifier as literals. `docs/BUILD_AND_RELEASE.md` and `docs/security/redaction_export_controls.md` also name the old strings.
- **Local store location:** the store sits under `app_local_data_dir` (`lib.rs:551`), which is keyed by the bundle identifier, so the new identifier starts from an empty store location. This is the same orphaning concern as the store filename ruling. It needs a fallback or an owner note, and the new Apple App ID is an owner act.
- **Core notice text:** `core/rules/rule_check_runner/src/lib.rs:81` and `core/rules/rule_pack_document/src/lib.rs:1210` still emit the acceptance sentence inside the notice const. If that changes, `lib.rs:7003` moves with it.
- **Fixtures still show the old name in provenance:** `fixtures/product_preview/*` and the examples say "OpenPipeStress project".
- **Core titles still carry the old name:** `core/reporting/pdf_emitter/src/lib.rs:68` `PDF_INFO_TITLE` and the schema titles.
- **SMOKE.md:** `apps/desktop/SMOKE.md` is frozen and still carries the old strings.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
