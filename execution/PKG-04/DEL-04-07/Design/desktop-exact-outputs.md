# Desktop outputs for the current exact identity: implementation design

This is the implementation design for shared epoch 8 (DEL-04-08 Design [b7-plan.md](../../DEL-04-08/Design/b7-plan.md), "After B7b": "Desktop outputs for the current exact identity are epoch 8", owner, 2026-10-20). It discharges DEL-04-08 REQ-007 (c) when AC-007 (c) holds: "the output surfaces it opens take the Rust reader's verdict and standing, as every reader surface does (CLM-003), and AC-005 and the desktop output-policy tests pass on its head." The design is written alongside epoch 7 (R-A); its implementation follows epoch 7. Under the owner's "Merge with the gap" (2026-10-15), every desktop output surface refuses `pressure-2` and `pressure-source-1` today ([pressure-contract.md](pressure-contract.md), "Reading pressure-2").

Line numbers cite main at `44fcec3c17`. Paths are relative to `projects/chirality-piping/`.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## 1. Summary

The unit needs no change to `core/reporting/result_export/**` or to any crate in `core/product_physics`'s dependency closure. It needs no new reader capability, no wasm export, no Tauri change, no schema or semantic-table change and no D3 re-registration (§3). The Rust reader already reads both identities on the desktop. The Rust derivative and the Python stress-neutral package already emit them. What is missing is desktop TypeScript:

1. **Current.** The desktop's static fresh-identity set omits both identities, so neither can be Current. Every output panel receives only the Current result, so none of them ever sees an exact result. The policy's per-surface refusals are a second, inner layer (§2.1).
2. **The opened surfaces' projections and gates.**
   - The result JSON export and the stress-neutral export lack the exact routes' members: the evidence object, and the receipt that pressure-source-1 carries. Rust's derivative and Python's package already have these.
   - The stress-neutral validator needs the exact routes' transport readings: through the source for pressure-2, which has no transport reader, as Python reads it; and through the existing transport reader for pressure-source-1.
   - The rule check needs a per-surface gate, and the registered invocation it already has a function for.
3. **The policy and its texts.** The admitted surfaces get an entry that adds no refusal. The remaining surfaces keep refusing under the existing code, with a reworded text.

The owner's choices are in §8. They concern which surfaces open (recommended: result export, stress-neutral and rule check) and whether load/reference-state results open with them (recommended: not in this unit).

## 2. Current state, from the code

### 2.1 Current, and the panels that receive it

- **The fresh-identity sets differ.**
  - The desktop's set is `FRESH_SEMANTIC_CONTRACT_IDS = [preview-physics-2, preview-physics-retained-1]` (`apps/desktop/src/features/results/knownSemanticLimitations.ts:17-20`). Its comment says the set "still names neither pressure-2 nor pressure-source-1, which Rust's `FRESH_IDENTITIES` holds" (`:14-16`).
  - Rust's set holds `PREVIEW_PHYSICS_ID`, `PRESSURE_SOURCE_ID`, `PRESSURE_ID` and `PREVIEW_PHYSICS_RETAINED_ID` under "One static set in every language" (`core/reporting/result_export/src/semantic_contract.rs:658-668`).
- **Current requires fresh membership.** `currentSolvedResult` requires `isFreshSemanticResult(result)`, a matching 0.3.0 record and manifest, numerically eligible standing and the native invocation (`apps/desktop/src/features/workspace/resultsSessionState.ts:56-76`).
- **The output panels receive only the Current result.**
  - These receive `currentSolvedResult` (`apps/desktop/src/App.tsx`):
    - the rendered report, report and report-lint panels (`:681, :695, :705`);
    - both rule-check panels (`:917-929`);
    - result export, stress-neutral and the interop and handoff panels (`:977-987`);
    - native package, handoff and export review (`:991, :1002, :1012`).
  - Only the results, solve and missing-data panels receive the raw result.
  - An exact result therefore reaches no output panel today, whatever the policy says.
- **Four functions consult `isFreshSemanticResult`:**
  - the Current qualification (`resultsSessionState.ts:58`);
  - the Current result-export builder (`resultExportAdapter.ts:196`, `CURRENT_SOURCE_IDENTITY_NOT_FRESH`);
  - the rule-check service (`services/ruleCheckService.ts:140`, `RULE_SOURCE_IDENTITY_NOT_FRESH`);
  - the report-package gate (`features/report/reportPackageRequest.ts:32`).

### 2.2 The policy

- **Entries.** `OUTPUT_POLICY` names 21 surfaces (`outputPolicy.ts:62-67`). `pressure` and `pressure_source` refuse on every one, with `PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE` and their own texts (`:36-44, :85, :91`).
- **The two read paths.**
  - The result-export and stress-neutral panels read the per-surface decision (`surfaceOutputRefusal`, `:158-167`).
  - The eighteen other surfaces read the route's shared reason through `LoadReferenceOutputGate` → `routeOutputRefusal` (`loadReferenceOutputAvailability.ts:36-38`; `outputPolicy.ts:131-136`), which ignores the per-surface table.
- **The admitted decision is retained-specific.** `admitted_when_eligible` returns a retained-precision text when standing is not eligible (`N_OUTPUT_NOT_NUMERICALLY_ELIGIBLE`, `:55`, `:163-166`). It is the wrong text for the exact routes.

### 2.3 The reader the desktop already has

The operation applier's wasm module exports the base readers (`core/model_operations/operation_applier/src/lib.rs:1193-1225`):

- `reader_dispatch_json`
- `reader_validate_json`
- `reader_transport_json`
- `reader_row_semantics_json`

`resultReader.ts` is the desktop's only path to them.

`reader_verdict.rs` already provides:

- **`pressure` validation:** `validate_pressure_evidence`, then G2's model checks, then `exact_model_checks` (`:241-256`).
- **`pressure-source` validation** with the captured invocation, and its standing findings (`:257-267`).
- **A `pressure-source` transport reader** (`:156-170`), and the refusal of `transport("pressure")` by name (`SOURCE_PRESSURE_TRANSPORT_UNSUPPORTED`, `:52`, `:162`).
- **The standing of both routes** (`:252-345`). pressure-source-1 is never eligible without the registered invocation (`SOURCE_BLOCKS_VALIDATED_INVOCATION_REQUIRED`). It is never eligible with the load/reference-state group either (`LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE_IN_T1`, `:316-318`).

The desktop records both identities, and holding a record grants no output:

- It builds and validates the 0.3 AnalysisRun for both (`services/analysisRunCompatibility.ts:118-134, :204-222`).
- It imports both semantic tables (`:4-5`).

A committed pressure-2 producer fixture reads as numerically eligible against its request model (`features/result-export/pressureResultConsumers.test.ts:31-37`).

### 2.4 The formats already defined for both identities

- **Result JSON.** Rust's `derive_document` copies `contract_evidence` for `PRESSURE_ID` and `PRESSURE_SOURCE_ID`. It copies `source_block_recovery` whole for pressure-source-1 only (b7-plan MJ8), and refuses the receipt on any other identity (`core/reporting/result_export/src/derivative.rs:156-176`). `validate_document` binds both (`:390-426`). The pressure-2 results branch is `schemas/results_pressure_2.v0.3.schema.json`.
- **Stress-neutral.**
  - Python's package admits both identities.
  - It copies `contract_evidence` for both, and `source_block_recovery` for pressure-source-1 (`core/handoff/stress_neutral/package.py:99-107, :431-551`).
  - It reads a pressure-2 package through its source, because there is no transport reader: `_transport_contract` dispatches to `_pressure_source_reading` (`:110-147`). Without the source it takes Rust's refusal by name.
  - `schemas/stress_neutral_export.v0.3.schema.json` has the exact branches.
- **Rule check.** The Tauri backend's qualification already admits both identities. It requires Rust's fresh set and `numerically_eligible` standing with the supplied invocation (`apps/desktop/src-tauri/src/lib.rs:2856-2900`). The rule-binding helper refuses only successor rows by accuracy class, so it binds exact rows as it binds preview-physics-2 rows (`semantic_contract.rs:683-688`). The desktop already has the accessor that supplies pressure-source-1's registered invocation as `sourceBlockInvocation`, and nothing calls it for that purpose today (`features/results/resultStanding.ts:87-94`).

## 3. Question 2: the priced source

**Answer: the unit can be built without changing `core/reporting/result_export/**` or any crate `core/product_physics` depends on.** Every needed change is desktop TypeScript under `apps/desktop/src/`. There is no D3 consequence and no collision with epoch 7.

The evidence, surface by surface:

- **No new reader is needed.** Each opened surface reads only what §2.3 lists. It reaches those through `resultReader.ts`, and through the Tauri rule-check backend that §2.4 describes.
  - pressure-2 needs `readerValidate("pressure", source, model)`.
  - pressure-source-1 needs `readerValidate("pressure-source", source, undefined, invocation)` through the existing registration (`resultStanding.ts:62-72`).
  - The pure projections use the source-recovery check that the record builder already uses (`analysisRunCompatibility.ts:255-260`).
  - pressure-source-1 transport uses `readerTransport("pressure-source", header)`.
  - Standing and row semantics come from dispatch and row semantics.
  - The wasm module's exports do not change.
- **pressure-2's missing transport reader is not a blocker.** The desktop never validates a stress-neutral package without its source:
  - The builder validates the packet with `args.result` (`StressNeutralExportPanel.tsx:536`).
  - The export link validates it with `binding[1]`, and the link exists only while `binding` is set (`:94-130`).
  - There is no desktop import or reopen path for stress-neutral packages: these are the only two callers of `validateStressNeutralExportPacket`.

  The desktop therefore reads a pressure-2 package through its source, exactly as Python does (§2.4). A pressure-2 header presented without its source keeps Rust's refusal by name, `readerTransport("pressure", …)` → `SOURCE_PRESSURE_TRANSPORT_UNSUPPORTED`. The result-export validator always takes its source (`resultExportAdapter.ts:117-118`), so it has no transport question.
- **The tables and schemas are untouched.** The tables are embedded in the reader with pinned sha256 (`semantic_contract.rs:279, :324`; `reader_verdict.rs:60-66`), and the desktop's record builder already imports them. The schemas already carry both identities.
- **The Tauri crate and the wasm crate are outside PP's closure.** `apps/desktop/src-tauri` and `operation_applier` depend on PP, and PP depends on neither (`core/product_physics/Cargo.toml`, `[dependencies]`). Neither changes here anyway.

**What would force a change in the priced closure:** a requirement to validate a pressure-2 stress-neutral package without its source on the desktop, for example a future package import. That would need a pressure-2 transport reader in `reader_verdict.rs`, which changes `core/reporting/result_export/src/**`. The consequences:

- `result_export` is a PP dependency (`core/product_physics/Cargo.toml`, "U3 (decision 5)"), so the change sits in the priced closure.
- It would need a D3 re-registration: `tools/retained_pricing` `run`, `compare`, and `generate --check` if any coefficient moves.
- It would collide with epoch 7's own D3 registration (`source-work-nondyadic-inputs.md`, D3 section).

This design does not need that capability and does not add it. If the owner wants package import on the desktop, it is a separate unit, placed after epoch 7.

## 4. The surfaces

### 4.1 Current (the qualification every surface depends on)

- **Change.** Add `PRESSURE_CONTRACT_ID` and `PRESSURE_SOURCE_CONTRACT_ID` to `FRESH_SEMANTIC_CONTRACT_IDS` (`knownSemanticLimitations.ts:17-20`). The desktop's set then equals Rust's `FRESH_IDENTITIES` (`semantic_contract.rs:660-668`), which says it is one static set in every language. Membership is not standing. `resultsSessionState.ts` is unchanged: an exact result becomes Current only with Rust's eligible standing, the matching 0.3.0 record and manifest, and the live native invocation. For pressure-source-1, the eligible standing is the registered one with the captured invocation.
- **Reader verdict and standing.** The Rust dispatch and standing (`reader_verdict.rs:252-345, :409-417`).
- **Claim text.** None added. Current is a qualification, not a notice.
- **Surfaces that start receiving an exact result.** Once an exact result can be Current, these receive it too. The implementation inspects and tests each:
  - **The fifteen refused output panels** (§4.5) show the route's refusal instead of an empty state.
  - **`ComparisonPanel`** shows the preview comparison built from the Current result (`resultsSessionState.ts:104-106`).
  - **`KnowledgePanel`** lists computed records from the result's diagnostics (`features/knowledge/KnowledgePanel.tsx:43-54`).
  - **`ReviewGeometryPanel`** writes a glTF review download. That download names the result only by `run_id` and embeds no result value (`features/review-geometry/ReviewGeometryPanel.tsx:61-78, :169`).
  - **The status bar and stage rail** count a qualified Current result (`features/workspace/shell/ShellStatusBar.tsx:32-39`).

  None of these carries result values to a file or a handoff, so none is an output surface in the policy's sense (`outputPolicy.ts:60-61`).

### 4.2 Result JSON export (`result-export`)

- **Policy.** Both exact routes are admitted on this surface (§5.1).
- **What it emits for an eligible Current result.** The 0.3 result document that `buildCurrentResultExport` assembles, with the identity's semantic contract reference. To match Rust's derivative (§2.4), the implementation adds these to `deriveResultDocument` (`resultExportAdapter.ts:41-72`) and to `validateResultDocument` (`:117-151`):
  - `contract_evidence`, copied whole, for both identities;
  - `source_block_recovery`, copied whole, for pressure-source-1 only;
  - the downgrade refusals: a receipt is refused on pressure-2, and a retained receipt on either identity.

  The pure projection reads `readerValidate("pressure", source, model)` for pressure-2 and the source-recovery check for pressure-source-1. This mirrors the preview-physics arm at `:57`.
- **Reader verdict and standing.** The Current builder keeps its chain: the per-surface refusal (`:194`), eligible standing (`:195`), fresh membership (`:196`) and the `CURRENT_*` bindings (`:197-229`). Row semantics come from `readerRowSemantics`, which is Rust's table.
- **Claim text.** None added. The export's existing data fields are unchanged, including its `professional_boundary` booleans, which are a data contract under the registry's §4. BS-VALID is not placed here; the result JSON export is not a handoff surface in the registry's list.

### 4.3 Stress-neutral export (`stress-neutral`)

- **Policy.** Both exact routes are admitted (§5.1).
- **What it emits for an eligible Current result.** The 0.3 package, with these members (`StressNeutralExportPanel.tsx:410-537`, matching Python's `_method_namespaces`):
  - **Semantic table.** The identity's table file in `SEMANTIC_TABLE_FILES`: `semantic_contract_v0_3_pressure_2.json` and `semantic_contract_v0_3_pressure_source_1.json` (`:852-863`). These are the refs Python writes (`package.py:435`).
  - **`contract_evidence`**, copied whole, for both identities.
  - **`source_block_recovery`**, copied whole, for pressure-source-1 only.

  Rows, witnesses, annotations and checksums are built exactly as now. Witness dispositions read the identity's table through `analysisRowSemantics`.
- **Validator changes** (`:540-670`):
  - The `SN-PHYSICAL-EVIDENCE-MISMATCH` and `SN-SOURCE-RECOVERY-CONTRADICTION` checks (`:556-561`) gain the exact arms. The receipt must be present and equal on pressure-source-1, and absent elsewhere.
  - `validateNeutralTransportEvidence` (`:865-872`, called at `:615`) gains two arms:
    - `pressure_source`: `readerTransport("pressure-source", header)`.
    - `pressure`: with a supplied source, `readerValidate("pressure", source)` and the existing source bindings. Without a source, `readerTransport("pressure", header)`, which keeps Rust's refusal by name. To give this arm its source, the validator passes the `source` it already holds to the transport step.
- **Reader verdict and standing.**
  - The panel's live binding requires eligible standing and the native invocation (`:59-67`).
  - The builder refuses non-empty standing findings (`:424`, `SN-NUMERICAL-INTEGRITY-NEEDS-RECOMPUTE`). It also requires the bound 0.3 record (`:425-435`).
- **Claim text.** None new. The panel keeps its existing BS-VALID placement, if any, unchanged. The package's boundary notes are the existing desktop notes. The retained boundary note stays off the exact identities, because `retained` is false for them (`:418, :445`).

### 4.4 Rule check (`rule-check`)

Two panels make up this surface, and both receive the Current result.

- **The completeness JSON (`RuleCheckPanel.tsx:54-64`).**
  - Today its download sits behind `LoadReferenceOutputGate`, which reads the route's shared reason.
  - Change: the gate takes its surface id, and calls `surfaceSourceRefusal(result, surface)` (§5.2).
- **The rule-check run (`RuleCheckRunPanel.tsx`, `services/ruleCheckService.ts:132-159`).**
  - Today the run has no policy gate; only fresh membership guards it (`:140`).
  - Change: `checkSolvedSource` calls `refuseSurfaceOutput(source, model, "rule-check")` before invoking, so the policy governs the run as it governs the download (§5.2).
  - For `pressure_source` it attaches `sourceBlockInvocation: retainedSourceRecoveryInvocation(source, model)` (`resultStanding.ts:87-94`). Without it, the backend's standing for pressure-source-1 cannot be eligible.
  - The run's aggregate rebuilds the AnalysisRun with its rule status as it does today (`features/workspace/workspaceSession.ts:1011-1060`). The record builder already admits both identities.
- **Reader verdict and standing.** The backend's unchanged qualification decides: Rust's fresh set and Rust's standing with the invocation (`lib.rs:2856-2900`). Rows bind through the unchanged `rule_binding_refusal` (`lib.rs:3273`).
- **What it emits.** The completeness JSON and the run result, unchanged in shape. The rule status in the record is one of the registered tokens, `RULE_INPUTS_INCOMPLETE`, `USER_RULE_CHECKED` or `USER_RULE_FAILED`. Each is shown in its registered display form with its authority domain (`docs/claims_registry.md` §2.1).
- **Claim text.** None new.
- **Numerical meaning to review (§6.4).**
  - The exact identity publishes one stress maximum per D-5 section basis and chooses no governing basis (DEL-07-05 ScopeOfWork, "Current reading and output boundary"). A user rule binds the row the user selects.
  - Every table row has `governing_ratio_eligible: false` in both exact tables. Nothing reads that field today.

### 4.5 Reports, the report package and the other surfaces

**Recommended: these stay refused in this unit** (§8, choice 1).

- **Report JSON and rendered report** (`report`, `rendered-report`). Their builders are generic.
  - `selectedResultRefs` picks the single `max_open_formula_stress` headline (`features/report/ReportPanel.tsx:279-298`), but an exact result carries one headline per section basis.
  - The renderable report's only limitation is a fixed technical-preview statement (`features/report/renderableReportInput.ts:343-357`). It carries none of the exact identity's published limitations, such as the D-3 exclusions.

  Opening them needs DEL-08-01's own design of those sections and of the PRD §19.3 notice's place. They keep refusing with the reworded text (§5.3).
- **Report package.** It is closed for every fresh identity, preview-physics-2 included, "until T6" (`reportPackageRequest.ts:23-34`; DEL-08-06 ScopeOfWork, "Desktop report-package request"). Below its gate it accepts only preview-physics-2 (`:245-247`). With the exact identities fresh, they fall to the same fresh-result refusal as every fresh identity (`REPORT-PACKAGE-FRESH-RESULT-UNAVAILABLE`). The route-specific arm at `:31` is removed. The policy entry `report-package: refused` records that decision, as the retained entries already do (`outputPolicy.ts:15-16`).
- **The other fifteen surfaces** keep refusing: PCF, CAEPIPE MBF and external, export-adapter SDK, adapter framework, external prover, missing-data, design workspace, report lint, solve job, headless runner, local FEA, native package, handoff and export review. Each is an interop or handoff format with its own owner and its own design. This is the same scope the retained routes keep.

## 5. Policy and text changes

### 5.1 A decision that adds no refusal

- **The new value.** `SurfaceDecision` gains `"admitted"` (`outputPolicy.ts:70`). It means: this policy adds no refusal on this surface, and the surface's own source and standing checks apply, as for an ungated route.
- **How the functions read it.**
  - `routeSurfaceDecision`, `surfaceRouteRefusal` and `surfaceOutputRefusal` return null for it (`:139-167`).
  - `admitted_when_eligible` keeps its retained meaning and text unchanged.
- **The exact entries.**
  - `"admitted"` on the surfaces the owner opens (recommended: `result-export`, `stress-neutral`, `rule-check`).
  - `"refused"` on every other surface, `report-package` included.

On an admitted surface the exact identities then behave as preview-physics-2 behaves today. Eligibility is the surfaces' own existing checks (§4.2–4.4), and no retained-precision text can appear on an exact result.

### 5.2 The gate reads the surface

- **Two levels.** The policy keeps its two read levels.
  - **The route level**, `surfaceRouteRefusal`, is for pure projections and validators (`deriveResultDocument`, `validateResultDocument`, `validateStressNeutralExportPacket`). It reads only the route and surface.
  - **The source level** is a new function, `surfaceSourceRefusal(source, surface)`. It is the route level plus the source-only condition of §8, choice 2 (B): a source carrying `contract_evidence.load_reference_states` on an exact route is refused with its identity's code. `surfaceOutputRefusal` (the Current level) applies the source level before its eligibility step. The gates and the rule-check service read the source level.

  Under choice 2 (A), the source level equals the route level.
- **The change.** `LoadReferenceOutputGate` takes the surface id its panel already passes as `testIdPrefix`; the surfaces are named by these prefixes (`outputPolicy.ts:60-61`). It calls `surfaceSourceRefusal(result, surface)` in place of `routeOutputRefusal`.
- **What stays the same.** For every route and every gated surface the decision is unchanged, except the exact routes on an admitted gated surface; today that is `rule-check` only. The reason: the retained entries refuse on all eighteen gated surfaces, and the ungated routes add no refusal.
- **The check.** A maintained test enumerates every route and every gated surface, and asserts this equality except for the opened cells.

### 5.3 Texts

- **The code is kept.** `PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE` stays the code of every remaining exact refusal. AC-006 keeps reading as written: "The desktop output policy refuses each named identity with its stated code".
- **The two texts are reworded.** They no longer say every output is unavailable. This follows Decision 12's rewording of the retained text (`outputPolicy.ts:46-47`). Proposed, under the recommended surfaces:
  - `N_PRESSURE_2_OUTPUT`: "This output of pressure-2 results (3.0.0/exact_pressure_v3) is not available on the desktop; only the result JSON and stress-neutral exports and the rule check admit a numerically eligible result. The result remains readable here; this is not a finding about the result."
  - `N_PRESSURE_SOURCE_OUTPUT`: the same sentence with "pressure-source-1".
- **The texts are checked twice.** They are reviewed against `docs/claims_registry.md` and pass GEN-13 (`tools/validation/validate_claims_language.py`, which scans the desktop TS sources). They carry no BS-ACCEPT text and no prohibition litany. They are UI reasons, never written into a document (`knownSemanticLimitations.ts:1-3`).
- **Comments.** The stale comments that state the old refusal are updated with the code: `outputPolicy.ts:17-26, :39-44, :83-91`, `loadReferenceOutputAvailability.ts`, `analysisRunCompatibility.ts:122-125`, `resultExportAdapter.ts:56-57`, `StressNeutralExportPanel.tsx:421-423, :849-851` and `knownSemanticLimitations.ts:10-16`.

## 6. Verification

### 6.1 The desktop output-policy tests (VER-006, AC-007 (c))

- **`outputPolicy.test.ts`.** The exact entries are asserted cell by cell: admitted on the opened surfaces and refused, with the code, on every other surface, the report package included. Every other assertion still holds: fail-closed routes and surfaces, the retained entries unchanged, and one entry per route.
- **The §5.2 equality test.**
- **Refusal tests that become admission tests, with the refusal kept on every unopened surface.** The test files are those found asserting the exact refusals:

| Test file | Lines |
|---|---|
| `features/results/loadReferenceOutputRefusal.test.tsx` | 78, 117, 202, 220 |
| `features/results/pressureSourceIntegration.test.tsx` | 133-156 |
| `features/results/ResultsPanel.test.tsx` | 93 |
| `features/results/KnownSemanticNotices.test.tsx` | 63-67 |
| `features/results/knownSemanticLimitations.test.ts` | 36-39 |
| `features/workspace/exactSessionSolve.session.test.tsx` | 177-202 |
| `features/workspace/loadReference.resultsSessionState.test.ts` | 102-123 |
| `features/result-export/pressureResultConsumers.test.ts` | 120-135 |
| `features/result-export/retainedPrecisionResultExport.test.tsx` | 277-291 |
| `features/stress-neutral/nativeStressNeutralBuilder.test.tsx` | 100-117 |
| `features/stress-neutral/retainedPrecisionStressNeutral.test.tsx` | 130-133 |
| `features/stress-neutral/StressNeutralExportPanel.test.tsx` | 336-344 |
| `features/report/reportPackageRequest.test.ts` | 503-513, now `REPORT-PACKAGE-FRESH-RESULT-UNAVAILABLE` |
| `features/solve/LoadReferenceStatesBlock.test.tsx` | the solve job stays refused |
| `features/bend-authoring/lLineAuthoring.test.tsx` | 129-136 |
| `services/pressureSourceBridge.test.ts` | 142-158 |
| `services/ruleCheckService.test.ts` | 315-351 |
| `features/results/retainedPrecisionOutputRefusal.test.tsx` | its negative text checks |

- **Admission.** For each opened surface and each identity, in both modes, an eligible Current result yields the output.
  - The panel shows its link only while the live binding holds, and a model edit voids it.
  - Each refusal is kept and asserted: a non-eligible standing; a missing or mismatched record; a pressure-source-1 result without its registered invocation; a saved or copied result, which is not Current.
- **Contract mutations.** Each is refused by name:
  - an exact document or package whose `contract_evidence` differs from its source's;
  - a receipt on pressure-2;
  - a missing or changed receipt on pressure-source-1;
  - a retained receipt on either identity;
  - a pressure-2 package validated without its source (`SOURCE_PRESSURE_TRANSPORT_UNSUPPORTED`).
- **Rule check.** A pressure-2 and a pressure-source-1 run reach `run_rule_checks` with their invocation arguments. A refused policy cell stops the run before invoke.

### 6.2 Cross-language agreement

The committed Rust and Python twins are the external basis for the two projections. For each identity, in both modes:

- **Result JSON.** The desktop's `validateResultDocument` accepts Rust's committed `*.document.json` twin with its source.
- **Stress-neutral.** The desktop's `validateStressNeutralExportPacket` accepts Python's committed `*.stress_neutral.json` twin with its source and record.
- **Shared members.** Where the source is eligible, the members the desktop builds from the source equal Python's: rows, CSV, witnesses, stable-ID map, annotations, `contract_evidence`, `source_block_recovery` and the semantic contract.

These twins exist only for load/reference-state cases (`fixtures/results/pressure_load_state_*`, `pressure_source_load_state_*`). Under choice 2 (B), §8, the load/reference refusal sits at the source level only (§5.2), so the pure validators still read the twins. Plain-result twins, if wanted, come from the existing Python writer (`tests/test_stress_neutral_physics_source.py`, `STRESS_NEUTRAL_WRITE_FIXTURES=1`), not from a new writer under `core/reporting/result_export/**`.

### 6.3 AC-005 and the readers

No Rust source changes, so the native outcomes and the wasm build's inputs are unchanged. These pass unchanged on the head:

- `baseReaderWasmEquivalence.test.ts` (R2, `fixtures/results/base_reader_outcomes.json`);
- `retainedPrecisionWasmEquivalence.test.ts` (R1);
- `test-support/retainedPrecisionCorpus.test.ts`;
- `pressureSourceCorpusParity.test.ts`;
- the verifier-independence test, the CLI binding and Python `rust_reader`.

A change in any committed outcome record is a defect signal here, not something to regenerate.

### 6.4 Where checks run, and independent review

- **The vitest suite does not run on pull requests.** `npm run test:desktop` runs in hosted CI only on `workflow_dispatch` (`.github/workflows/piping-desktop-e2e.yml:100-103`). The required pull-request lane runs only `r2-smoke.spec.ts` and `b3b-project-persistence.spec.ts`, both pressure-free (`:108-110`). The broad browser matrix runs on dispatch only (`:236-258`). So:
  - The implementation PR runs `npm run build:wasm:desktop && npm run test:desktop` on its exact head, and dispatches `piping-desktop-e2e` for the full suite and matrix on that head.
  - The hosted `harness` result stays the merge check.
  - `r2-smoke`'s refusal of the bundled reference on stress-neutral and result export (`e2e/r2-smoke.spec.ts:805-808`) is not exact-route and stays as asserted.
- **No new browser spec.** None is added to the required lane. The jsdom session tests (§6.1) are the end-to-end path for exact results.
- **Also run:** GEN-13 (`harness-self-check`) for the texts.
- **Independent review is required.** It is warranted by the Root `AGENTS.md` classes for persistence and permissions, and by the numerical-use question in §4.4. A separate agent reviews the implementation against this design. It checks:
  1. **Inventory.** Nothing beyond the chosen surfaces opens. That covers all 21 surfaces and the Current-receiving panels of §4.1.
  2. **Fail-closed gates.** Every gate fails closed.
  3. **Contract parity.** Parity with Rust's derivative and Python's package (§6.2), including the read through the source.
  4. **Rule binding.** The binding exposure of the exact rows, the per-basis maxima included.
  5. **Claim texts.** The texts against the registry.
  6. **Reconciliation.** The reconciliation edits (§7).

## 7. Owners, implementation and reconciliation

**Owner of the implementation.** One implementation manager (WORKING_ITEMS) owns epoch 8's implementation and its reconciliation, dispatched by HELP_HUMAN after epoch 7 merges. Under the owner rule of 2026-10-17, the reconciliation of the affected ScopeOfWork, Design and dependency conditions comes in the same implementation PR, before integration. Commitment and acceptance-criterion text stays the owner's (§8). The deliverables it affects:

| Deliverable | Owns | Implementation | Reconciliation |
|---|---|---|---|
| DEL-07-05 Results viewer | `outputPolicy.ts`, `loadReferenceOutputAvailability.ts`, `LoadReferenceOutputGate.tsx`, `knownSemanticLimitations.ts`, notices | §5; fresh set (§4.1) | ScopeOfWork "Current reading and output boundary": the refusal sentence is replaced by the admitted surfaces and the remaining refusal |
| DEL-08-04 Result export format | `apps/desktop/src/features/result-export/**` (and the Rust derivative, unchanged) | §4.2 | ScopeOfWork statements of the desktop export's identities, where they name the refusal |
| DEL-17-06 Stress-neutral CSV/JSON package | `apps/desktop/src/features/stress-neutral/**`, Python package (unchanged) | §4.3 | ScopeOfWork "Package member path references" row: the desktop panel admits both identities and reads a pressure-2 package through its source |
| DEL-07-12 Interoperability and evidence-review workspace | `stress-neutral/**` (shared), the handoff panels | none beyond §4.3; its panels keep refusing | none expected |
| DEL-07-04 Missing-data warning and blocking UX | `rule-check/**`, `services/ruleCheckService.ts` | §4.4 | its ScopeOfWork where it states the rule check's admitted identities, if it does |
| DEL-08-06 State, comparison, and handoff report sections | `reportPackageRequest.ts` | §4.5: the exact arm at `:31` goes | ScopeOfWork "Desktop report-package request": pressure-2 and pressure-source-1 now take T0R's fresh-result refusal |
| DEL-08-01 Calculation report generator | `ReportPanel.tsx`, `RenderedReportPanel.tsx` | none (they keep refusing) | none, unless the owner opens reports (choice 1 (B)) |
| DEL-07-11 Desktop workspace shell | `App.tsx`, `features/workspace/**` (Current) | none expected; Current changes through the fresh set | none expected |
| DEL-04-08 Retained-precision result verification | co-lists `outputPolicy.ts`; holds REQ-006 (c), REQ-007 (c), CLM-003, AC-006, AC-007, VER-006 | none of its code | VER-006's test list; CLM-003 and REQ-006 (c) as the owner approves (§8); REQ-007 (c) discharge when AC-007 (c) holds |
| DEL-04-07 (this deliverable) | this design; [pressure-contract.md](pressure-contract.md) | none | "Reading pressure-2" and this file's as-built note |

- **TASK contributions.** TASKs, if used, report their slices' effects. The manager reconciles the combined result.
- **Scope of this PR.** This design PR edits only DEL-04-07. Epoch 7 is reconciling DEL-04-08, so DEL-04-08's edits wait for the implementation PR.

## 8. Choices reserved for the owner

**1. Which surfaces open.**

- **(A) Recommended: result export, stress-neutral and rule check.**
  - These are the surfaces DEL-04-07 named as refused ("export, stress-neutral, rule check, Current").
  - Each already has its exact-route format and verdict in Rust or Python (§2.4).
  - Reports, the report package and the other fifteen surfaces keep refusing (§4.5).
  - **Consequence:** reports of exact results remain unavailable until DEL-08-01 designs their limitations sections. The report package is unchanged, closed for every identity.
- **(B) As (A), and also report JSON and rendered report.**
  - **Consequence:** DEL-08-01 must design the per-basis headlines and the exact identity's limitations in the report input (§4.5) before implementation. That enlarges the unit and its review, and adds a report owner to the reconciliation.
- **(C) Only the two exports, as the retained routes have.**
  - **Consequence:** the rule check stays refused, and DEL-04-07's "rule check" stays an open gap.

**2. Load/reference-state results under the exact identities.** Since B7b, load/reference-state results publish under pressure-2 and pressure-source-1. Today they are refused with their identity's reason.

- **(B) Recommended: they stay refused in this unit.**
  - The refusal is the policy's source-level condition (§5.2): a source carrying `contract_evidence.load_reference_states` is refused on every surface, with the identity's code. It is applied by the gates, the Current level and the rule-check service, not by the pure projections and validators.
  - Reason: the desktop's basis references are the model's load cases and combinations only (`analysisRunCompatibility.ts:92-94`). Whether a reference-state row's basis passes the record's load-basis check (`:98-107`) has not been established.
  - **Consequence:** REQ-006 (c) keeps a residual limit for these results.
- **(A) They follow the reader.**
  - pressure-source-1 load/reference results stay never-eligible by Rust's own finding (`reader_verdict.rs:316-318`). Eligible pressure-2 load/reference results open with the chosen surfaces.
  - **Consequence:** the implementation must first establish and test their basis handling, which enlarges the unit.

**3. The commitment text (DEL-04-08), under (1A) and (2B).** This is proposed for the owner's approval at the implementation PR. HELP_HUMAN may apply AC wording that changes only to describe consequences of these decisions (owner, 2026-10-19). The current text:

> **(c) No desktop outputs for the current exact identity.**
> - Exact-route results (`pressure-2`, `pressure-source-1`) solve and can be viewed on the desktop with the Rust reader's standing, but every desktop output surface refuses them (`PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE`). The Python AnalysisRun record and the stress-neutral export accept them.

The proposed text:

- **REQ-006 (c)**, replacing its first bullet; the retained-precision bullet is unchanged:

  > **(c) Desktop outputs for the current exact identity.**
  > - An exact-route result (`pressure-2`, `pressure-source-1`) at the Rust reader's numerically eligible standing, with the live native invocation, is Current on the desktop. Its result JSON export, stress-neutral export and rule check admit it, and each takes the reader's verdict and standing. Every other desktop output surface refuses it (`PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE`). The report package refuses it as it refuses every fresh identity (`REPORT-PACKAGE-FRESH-RESULT-UNAVAILABLE`). A result carrying load/reference states is refused on every desktop output surface. The Python AnalysisRun record and the stress-neutral export accept them all.

- **CLM-003**, "Retained-source recovery", its third bullet: "The Python AnalysisRun record and the stress-neutral export accept its results. On the desktop an eligible result is Current, and its result JSON and stress-neutral exports and rule check admit it (REQ-006 (c))."
- **REQ-007 (c)**: unchanged until AC-007 (c) holds on the implementation head. Then a dated "**Discharged on …**" sentence, in the form of (a) and (b).
- **AC-006 and AC-007 (c)**: unchanged. Both read correctly after the change.

**4. Native smoke (G10's procedure).** Recommended: no exact case is added to `apps/desktop/native-smoke/expected.json`. That file is the procedure for the owner-held G10 check (DEL-07-05 Design `owner-mac-check.md`). The consequence is that no native-host run of an exact export is in the automated smoke. The jsdom session tests carry the path instead.

## 9. Interactions

- **Epoch 7 (R-A).**
  - **What R-A changes.** R-A's design changes no fixture now. Its implementation regenerates the pressure-source-1 received bytes and their derived records: the 24 raw source controls, `fixtures/results/pressure_source_reader_corpus.json`, the load/reference-source ledgers and mutations, and the derived documents (`source-work-nondyadic-inputs.md` §4, "Committed bytes that move"). Desktop tests read these files (`pressureSourceCorpusParity.test.ts:11, :22-27`; the raw controls in the files of §6.1).
  - **What this unit therefore does.**
    1. It implements after epoch 7 merges, rebased on the regenerated bytes.
    2. It pins no received digest; tests derive digests from the fixtures.
    3. It adds no fixture that an epoch 7 writer owns.
  - **What epoch 7's manager should know.**
    - The §6.2 parity tests will read the `pressure_source_load_state_*.{document,stress_neutral,analysis_run}.json` twins. Their writers (`load_reference_source_contract.rs`, `tests/test_stress_neutral_physics_source.py`, `tests/test_load_reference_source_readers.py`) must run with R-A's regeneration.
    - `fixtures/**` selects no registered check (`software-workflow.json:63-82`), so a fixture-only change does not select `desktop-test`.
  - **R-A's D3 registration** is unaffected, because this unit changes no priced source.
- **The frozen UI benchmark** (pending the owner).
  - No file under `apps/desktop/e2e/ui-foundation/**` changes.
  - Its 0.2.0 models are refused at open as `retired_schema`, so they never reach a solve or an output surface.
  - No e2e spec exercises an exact result.
- **B8 and G10** (held by the owner). Neither bears on this unit.
  - **B8** activates retained successors on the desktop. The retained policy entries, texts and standing are unchanged, and the exact identities are not successors.
  - **G10's** milestone model is pressure-free (model 0.6.0 without `pressure_contract`), and the native smoke's other model has nonlinear supports, which the exact route excludes. So no G10 observation moves. This holds if `expected.json` is left alone, as choice 4 recommends.
- **Observation, not changed here.** physics-retained-1 is in neither fresh set, so its `admitted_when_eligible` cells cannot be reached from the product. This is consistent with Rust and belongs to B8.

## 10. Blast radius: what stays byte-identical

- `core/reporting/result_export/**`, including its tests and test fixtures.
- Every crate in `core/product_physics`'s dependency closure, and `core/product_physics/**`, including the GENERATED PROFILE block in `retained_memory.rs`. No D3 registration input moves.
- `core/model_operations/operation_applier/**` (the wasm exports) and `apps/desktop/src-tauri/**`.
- The semantic tables `fixtures/results/semantic_contract_v0_3_*.json`, and the schemas `results.v0.3`, `results_pressure_2.v0.3`, `analysis_run.v0.3` and `stress_neutral_export.v0.3`.
- Python product code (`core/analysis_runs/**`, `core/handoff/stress_neutral/**`, `core/reporting/**`) and `tests/verifiers/**`.
- Every committed corpus, outcome record, raw control and twin under `fixtures/results/**` and `fixtures/product_preview/**`, and `validation/references/**`.
- The published bytes of every identity, and the desktop's AnalysisRun records for every identity.
- The retained policy entries, the retained texts and codes, and preview-physics-2's behaviour on every surface.
- `apps/desktop/e2e/ui-foundation/**` and `apps/desktop/native-smoke/**`.
- The report package's behaviour for every identity other than the exact ones, which move to the common fresh-result refusal.

Changes are confined to `apps/desktop/src/**` TypeScript and its tests, the comments listed in §5.3, and the reconciliation text of §7.
