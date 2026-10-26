# Desktop outputs for the current exact identity: implementation design

This is the implementation design for shared epoch 8. The owner placed the unit as epoch 8, after epoch 7, on 2026-10-20: "I approve all your recommended options, provisionally where appropriate" (DEL-04-08 Design [b7-plan.md](../../DEL-04-08/Design/b7-plan.md), "After B7b", through #1489). It discharges DEL-04-08 REQ-007 (c) when AC-007 (c) holds: "the output surfaces it opens take the Rust reader's verdict and standing, as every reader surface does (CLM-003), and AC-005 and the desktop output-policy tests pass on its head." The design was written alongside epoch 7 (R-A), which is on main through #1497; its implementation follows. Under the owner's "Merge with the gap" (2026-10-15), every desktop output surface refuses `pressure-2` and `pressure-source-1` until that implementation lands ([pressure-contract.md](pressure-contract.md), "Reading pressure-2").

The owner ruled on 2026-10-21, accepting HELP_HUMAN's recommended set: "A accept your recommendations for the entire set." The set has three rulings:
- **10A** opens the result JSON export, the stress-neutral export and the rule check (this design's choice 1 (A)).
- **11B** has load/reference-state results under the exact identities follow the Rust reader. That is this design's choice 2 (A), not the (B) it first recommended.
- **12** approves the DEL-04-08 commitment text as revised to match; the owner confirms the revised text before the implementation PR applies it (§8).

This revision replaces the source-aware refusal with 11B, in §5.2, §5.4, §6 and §8.

Line numbers cite main at `44fcec3c17`. Every cited desktop, reader and Tauri source file is unchanged on `fa0028a7d7`; the hosted workflow `piping-desktop-e2e.yml` is not (§6.4). Citations added in the 11B revision are to `fa0028a7d7`. Paths are relative to `projects/chirality-piping/`, except `.github/`, root `AGENTS.md`, `docs/PRODUCT_BOUNDARIES.md` and repository-root `tools/validation/`.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## 1. Summary

The unit needs no change to `core/reporting/result_export/**` or to any crate in `core/product_physics`'s dependency closure. It needs no new reader capability, no wasm export, no Tauri product change, no schema or semantic-table change and no D3 re-registration (§3). The Rust reader already reads both identities on the desktop. The Rust derivative and the Python stress-neutral package already emit them. What is missing is desktop TypeScript:

1. **Current.** The desktop's static fresh-identity set omits both identities, so neither can be Current. Every output panel except the solve job and missing-data receives only the Current result, so none of them ever sees an exact result. The policy's per-surface refusals are a second, inner layer (§2.1).
2. **The opened surfaces' projections and gates.**
   - The result JSON export and the stress-neutral export lack the exact routes' members: the evidence object, and the receipt that pressure-source-1 carries. Rust's derivative and Python's package already have these.
   - The stress-neutral validator needs the exact routes' transport readings: through the source for pressure-2, which has no transport reader, as Python reads it; and through the existing transport reader for pressure-source-1.
   - The rule check needs a per-surface gate, and the registered invocation it already has a function for.
3. **The policy and its texts.** The admitted surfaces get an entry that adds no refusal. The remaining surfaces keep refusing under the existing code, with a reworded text.

4. **Load/reference-state results follow the reader.** No source-level step is added. The committed pressure-2 load/reference-state results read numerically eligible, and the committed pressure-source-1 ones never do, in Rust's own pinned outcomes (§5.4). The opened panels disclose the resolved reference states as data, and the files carry them as Rust's derivative and Python's package already do.

The owner's rulings are in §8: result export, stress-neutral and rule check open, and load/reference-state results follow the reader.

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
    - the design workspace panel (`:635`).
  - The results, solve and missing-data panels receive the raw result (`:625`, `:906`, `:1519`). Of these, the solve job and missing-data are policy output surfaces, and for them the policy gate is the only guard. The audit, viewport and diagnostics views also take the raw result, and carry no download.
  - So an exact result reaches no Current-receiving output panel today, whatever the policy says. Only the two raw-result output surfaces show its refusal.
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
- **A `pressure-source` transport reader** (`:295-306`), and the refusal of `transport("pressure")` by name (`SOURCE_PRESSURE_TRANSPORT_UNSUPPORTED`, `:52`, `:301`).
- **The shallow source-recovery check** (`:283-293`): the header, the transport metadata and the publication commitment, without raw-row validation.
- **The standing of both routes** (`:391-462`). pressure-source-1 is never eligible without the registered invocation (`SOURCE_BLOCKS_VALIDATED_INVOCATION_REQUIRED`). It is never eligible with the load/reference-state group either (`LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE_IN_T1`, `:428`).

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
  - The pure projections and validators read a supplied pressure-source-1 source through the full raw reader, `readerValidate("pressure-source", source)` with no invocation (`reader_verdict.rs:258-267`, `:275-283`). Rust's derivative reaches the same full reader through `for_source` (`semantic_contract.rs:639-641`), and Python calls it too (`core/analysis_runs/compatibility.py:335`). The shallow source-recovery check is not enough here: it omits raw-row validation (`reader_verdict.rs:283-293`), so a coherently rehashed invalid source would pass it.
  - pressure-source-1 transport uses `readerTransport("pressure-source", header)`.
  - Standing and row semantics come from dispatch and row semantics.
  - The wasm module's exports do not change.
- **pressure-2's missing transport reader is not a blocker.** The desktop never validates a stress-neutral package without its source:
  - The builder validates the packet with `args.result` (`StressNeutralExportPanel.tsx:536`).
  - The export link validates it with `binding[1]`, and the link exists only while `binding` is set (`:94-130`).
  - There is no desktop import or reopen path for stress-neutral packages: these are the only two callers of `validateStressNeutralExportPacket`.

  The desktop therefore reads a pressure-2 package through its source, exactly as Python does (§2.4). A pressure-2 header presented without its source keeps Rust's refusal by name, `readerTransport("pressure", …)` → `SOURCE_PRESSURE_TRANSPORT_UNSUPPORTED`. The result-export validator always takes its source (`resultExportAdapter.ts:117-118`), so it has no transport question.
- **The tables and schemas are untouched.** The tables are embedded in the reader with pinned sha256 (`semantic_contract.rs:279, :324`; `reader_verdict.rs:56-62`), and the desktop's record builder already imports them. The schemas already carry both identities.
- **The Tauri crate and the wasm crate are outside PP's closure.** `apps/desktop/src-tauri` and `operation_applier` depend on PP, and PP depends on neither (`core/product_physics/Cargo.toml`, `[dependencies]`). Neither changes here anyway.

**What would force a change in the priced closure:** a requirement to validate a pressure-2 stress-neutral package without its source on the desktop, for example a future package import. That would need a pressure-2 transport reader in `reader_verdict.rs`, which changes `core/reporting/result_export/src/**`. The consequences:

- `result_export` is a PP dependency (`core/product_physics/Cargo.toml`, "U3 (decision 5)"), so the change sits in the priced closure.
- It would need a D3 re-registration: `tools/retained_pricing` `run`, `compare`, and `generate --check` if any coefficient moves.
- It would collide with epoch 7's own D3 registration (`source-work-nondyadic-inputs.md`, D3 section).

This design does not need that capability and does not add it. If the owner wants package import on the desktop, it is a separate unit, placed after epoch 7.

## 4. The surfaces

### 4.1 Current (the qualification every surface depends on)

- **Change.** Add `PRESSURE_CONTRACT_ID` and `PRESSURE_SOURCE_CONTRACT_ID` to `FRESH_SEMANTIC_CONTRACT_IDS` (`knownSemanticLimitations.ts:17-20`). The desktop's set then equals Rust's `FRESH_IDENTITIES` (`semantic_contract.rs:660-668`), which says it is one static set in every language. Membership is not standing. `resultsSessionState.ts` is unchanged: an exact result becomes Current only with Rust's eligible standing, the matching 0.3.0 record and manifest, and the live native invocation. For pressure-source-1, the eligible standing is the registered one with the captured invocation.
- **Load/reference-state results.** The same qualification applies, with no source-level step (§5.4). An eligible pressure-2 load/reference-state result becomes Current. A pressure-source-1 one never does, because Rust's standing always carries `LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE_IN_T1` (`reader_verdict.rs:427-428`).
- **Reader verdict and standing.** The Rust dispatch and standing (`reader_verdict.rs:391-462, :548-556`).
- **Claim text.** None added. Current is a qualification, not a notice.
- **Surfaces that start receiving an exact result.** Once an exact result can be Current, these receive it too. The implementation inspects and tests each:
  - **The fifteen refused output panels** (§4.5) show the route's refusal instead of an empty state.
  - **`ComparisonPanel`** shows the preview comparison built from the Current result (`resultsSessionState.ts:104-106`).
  - **`KnowledgePanel`** lists computed records from the result's diagnostics (`features/knowledge/KnowledgePanel.tsx:43-54`).
  - **`ReviewGeometryPanel`** writes a glTF review download. That download names the result only by `run_id` and embeds no result value (`features/review-geometry/ReviewGeometryPanel.tsx:61-78, :169`).
  - **The status bar and stage rail** count a qualified Current result (`features/workspace/shell/ShellStatusBar.tsx:32-39`).
  - **The native menu state** reads the Current result (`features/workspace/workspaceSession.ts:2618`).

  None of these carries result values to a file or a handoff, so none is an output surface in the policy's sense (`outputPolicy.ts:60-61`).

### 4.2 Result JSON export (`result-export`)

- **Policy.** Both exact routes are admitted on this surface (§5.1).
- **What it emits for an eligible Current result.** The 0.3 result document that `buildCurrentResultExport` assembles, with the identity's semantic contract reference. To match Rust's derivative (§2.4), the implementation adds these to `deriveResultDocument` (`resultExportAdapter.ts:41-72`) and to `validateResultDocument` (`:117-151`):
  - `contract_evidence`, copied whole, for both identities;
  - `source_block_recovery`, copied whole, for pressure-source-1 only;
  - the downgrade refusals: a receipt is refused on pressure-2, and a retained receipt on either identity.

  The new arms use Rust's codes for the same conditions (`derivative.rs:390-426`): `SOURCE_BLOCK_RECEIPT_BINDING_MISMATCH`, `SOURCE_BLOCKS_DOWNGRADE_FORBIDDEN` and `RETAINED_PRECISION_DOWNGRADE_FORBIDDEN`. The desktop's existing arms keep theirs: `SOURCE_PHYSICAL_METADATA_MISMATCH` and `SOURCE_RECOVERY_METADATA_CONTRADICTION` (`resultExportAdapter.ts:59-62, :120-128`).

  The pure projection and the validator read `readerValidate("pressure", source, model)` for pressure-2 and `readerValidate("pressure-source", source)` for pressure-source-1. This mirrors the preview-physics arm at `:57` and the full reader that Rust's `derive_document` reaches (§3).
- **Reader verdict and standing.** The Current builder keeps its chain: the per-surface refusal (`:194`), eligible standing (`:195`), fresh membership (`:196`) and the `CURRENT_*` bindings (`:197-229`). Row semantics come from `readerRowSemantics`, which is Rust's table.
- **Claim text.** None added. The export's existing data fields are unchanged, including its `professional_boundary` booleans, which are a data contract under the registry's §4. BS-VALID is not placed here; the result JSON export is not a handoff surface in the registry's list.
- **Load/reference states.** The document carries them as data, as Rust's does, and the panel shows the §5.4 disclosure line.

### 4.3 Stress-neutral export (`stress-neutral`)

- **Policy.** Both exact routes are admitted (§5.1).
- **What it emits for an eligible Current result.** The 0.3 package, with these members (`StressNeutralExportPanel.tsx:410-537`, matching Python's `_method_namespaces`):
  - **Semantic table.** The identity's table file in `SEMANTIC_TABLE_FILES`: `semantic_contract_v0_3_pressure_2.json` and `semantic_contract_v0_3_pressure_source_1.json` (`:852-863`). These are the refs Python writes (`package.py:435`).
  - **`contract_evidence`**, copied whole, for both identities.
  - **`source_block_recovery`**, copied whole, for pressure-source-1 only.

  Rows, witnesses, annotations and checksums are built exactly as now. Witness dispositions read the identity's table through `analysisRowSemantics`.
- **Validator changes** (`:540-670`):
  - The `SN-PHYSICAL-EVIDENCE-MISMATCH` and `SN-SOURCE-RECOVERY-CONTRADICTION` checks (`:556-561`) gain the exact arms. The receipt must be present and equal on pressure-source-1, and absent elsewhere. An absent receipt, or one on another identity, is `SN-SOURCE-RECOVERY-CONTRADICTION`. An unequal receipt takes a new code, `SN-SOURCE-BLOCK-RECEIPT-MISMATCH`, after Rust's `SOURCE_BLOCK_RECEIPT_BINDING_MISMATCH`. A copied `contract_evidence` that differs is `SN-PHYSICAL-EVIDENCE-MISMATCH`.
  - `validateNeutralTransportEvidence` (`:865-872`, called at `:615`) gains two arms:
    - `pressure_source`: `readerTransport("pressure-source", header)`.
    - `pressure`: with a supplied source, `readerValidate("pressure", source)` and the existing source bindings. Without a source, `readerTransport("pressure", header)`, which keeps Rust's refusal by name. To give this arm its source, the validator passes the `source` it already holds to the transport step.
- **Reader verdict and standing.**
  - The panel's live binding requires eligible standing and the native invocation (`:59-67`).
  - The builder refuses non-empty standing findings (`:424`, `SN-NUMERICAL-INTEGRITY-NEEDS-RECOMPUTE`). It also requires the bound 0.3 record (`:425-435`).
- **Claim text.** None new. The panel keeps its existing BS-VALID placement unchanged (`StressNeutralExportPanel.tsx:196-197, :281`). The package's boundary notes are the existing desktop notes. The retained boundary note stays off the exact identities, because `retained` is false for them (`:418, :445`).
- **Load/reference states.** The package carries them as data, as Python's does, and the panel shows the §5.4 disclosure line beside its unchanged BS-VALID placement.

### 4.4 Rule check (`rule-check`)

Two panels make up this surface. Both receive the Current result. The run also has a non-Current path: with no supplied envelope, `runRuleChecks` solves the model itself, and that transient result is never Current (`RuleCheckRunPanel.tsx:669`; `ruleCheckService.ts:123-127`). preview-physics-2 already takes this path, and the exact identities take it too. It has the same policy and standing checks as the Current path, but no record or manifest binding, which the backend does not need.

- **The completeness JSON (`RuleCheckPanel.tsx:54-64`).**
  - Today its download sits behind `LoadReferenceOutputGate`, which reads the route's shared reason.
  - Change: the gate takes its surface id, and calls `surfaceRouteRefusal(result, surface)` (§5.2).
- **The rule-check run (`RuleCheckRunPanel.tsx`, `services/ruleCheckService.ts:132-159`).**
  - Today the run has no policy gate; only fresh membership guards it (`:140`). A retained successor is admitted at eligible standing (`:143-148`), and `retainedPrecisionIntegration.test.tsx:491-509, :539-548` assert that it reaches the backend. That retained path stays exactly as it is (§10), although the retained policy entries mark `rule-check` refused (`outputPolicy.ts:100`). Reconciling that cell with the run belongs to B8.
  - Change, for the exact routes only (`pressure`, `pressure_source`): `checkSolvedSource` calls `refuseSurfaceOutput(source, model, "rule-check")` before invoking, so the exact entries govern the run as they govern the download (§5.2). The retained and ungated routes keep today's checks, unchanged.
  - For both exact routes it also requires the desktop's standing, `numericalResultStanding(source, model).eligible`, before invoking, as it already does for the retained routes (`:143-146`). A non-eligible exact result is refused with `${finding}: ${N_RULE_CHECK_NOT_NUMERICALLY_ELIGIBLE}`, where the finding is Rust's first standing finding and the new text is "The result is not numerically eligible for rule checks. It remains readable here." The retained text at `:145` is unchanged. That standing is `reader_verdict::standing` with the model's checks. The backend's own check is `numerical_use_standing_with_context`, which for pressure-2 does not run the model checks (`lib.rs:2884`). So the desktop step makes the Rust reader's full standing govern both the Current path and the transient path.
  - For `pressure_source` it attaches `sourceBlockInvocation: retainedSourceRecoveryInvocation(source, model)` (`resultStanding.ts:87-94`). Without it, the backend's standing for pressure-source-1 cannot be eligible.
  - The run's aggregate rebuilds the AnalysisRun with its rule status as it does today (`features/workspace/workspaceSession.ts:1011-1060`). The record builder already admits both identities.
- **Reader verdict and standing.** First the desktop's `reader_verdict::standing` (above). Then the backend's unchanged qualification: Rust's fresh set and `numerical_use_standing_with_context` with the invocation (`lib.rs:2856-2900`). Rows bind through the unchanged `rule_binding_refusal` (`lib.rs:3273`).
- **Persistence.** A rule-check aggregate on a Current exact result is written into its AnalysisRun (`workspaceSession.ts:1034-1052`). A project save stores the result and that record (`workspaceSession.ts:2182-2183`). Saved exact runs can therefore carry `USER_RULE_CHECKED` or `USER_RULE_FAILED`. The record format and its builder are unchanged, and a reopened saved run is historical, never Current (DEL-07-05 ScopeOfWork, "Current reading and output boundary").
- **What it emits.** The completeness JSON and the run result, unchanged in shape. The rule status in the record is one of the registered tokens, `RULE_INPUTS_INCOMPLETE`, `USER_RULE_CHECKED` or `USER_RULE_FAILED`. Each is shown in its registered display form with its authority domain (`docs/claims_registry.md` §2.1).
- **Claim text.** None new.
- **Load/reference states.** Both panels show the §5.4 disclosure line. The run panel takes it from the service, because the transient path's result is disposed after the run (`ruleCheckService.ts:123-127`) and is never Current: `checkSolvedSource` computes the line from the qualified source, after the standing check and before the invoke, and returns it on the `tauri_backend` route beside the unchanged `RuleCheckRunResult` (`loadReferenceDisclosure: string | null`). The panel stores it in the same `runPresentation` as the result (`RuleCheckRunPanel.tsx:681`), so it is cleared and invalidated exactly when the result is. It is never written into the run result, the record or a file. The backend needs no change for them: for an eligible pressure-2 load/reference-state result, `numerical_use_standing_with_context` is `numerically_eligible`, and rows bind by load case as for any other pressure-2 result. For pressure-source-1 the desktop's standing stops the run before invoke, and the backend's own standing would refuse it too.
- **Numerical meaning to review (§6.4).**
  - The exact identity publishes one stress maximum per D-5 section basis and chooses no governing basis (DEL-07-05 ScopeOfWork, "Current reading and output boundary"). A user rule binds the row the user selects.
  - Every table row has `governing_ratio_eligible: false` in both exact tables. Nothing reads that field today.

### 4.5 Reports, the report package and the other surfaces

**These stayed refused in this unit** (ruling 10A, §8). The rendered report and the report package were opened later under DEL-08-01 Design `exact-route-report-sections.md` (owner, 2026-10-26: "My two decisions are to approve your recommendations in both cases. Proceed accordingly."); the report JSON packet and the other fifteen surfaces still refuse.

- **Report JSON** (`report`) stays refused with `PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE`; its builder still selects one headline and fixed demo references.
- **Rendered report** (`rendered-report`) admits an exact-route result only when Export Result JSON admits it, with the sections of DEL-08-01 Design `exact-route-report-sections.md`. Its native render admits the pressure-2 and pressure-source-1 identities and binds each exact input to the result and model its solve job delivered and received. The PRD §19.3 notice keeps its existing place.
- **Report package** (`report-package`) saves a Current preview-physics-2 result or an eligible pressure-2 or pressure-source-1 result. Its TypeScript and Rust identity checks admit those three identities; its exact sections are checked against the source as DEL-08-01 Design `exact-route-report-sections.md` specifies.
- **The other fifteen surfaces** keep refusing: PCF, CAEPIPE MBF and external, export-adapter SDK, adapter framework, external prover, missing-data, design workspace, report lint, solve job, headless runner, local FEA, native package, handoff and export review. Each is an interop or handoff format with its own owner and its own design. This is the same scope the retained routes keep.

## 5. Policy and text changes

### 5.1 A decision that adds no refusal

- **The new value.** `SurfaceDecision` gains `"admitted"` (`outputPolicy.ts:70`). It means: this policy adds no refusal on this surface, and the surface's own source and standing checks apply, as for an ungated route.
- **How the functions read it.**
  - `routeSurfaceDecision`, `surfaceRouteRefusal` and `surfaceOutputRefusal` return null for it (`:139-167`).
  - `admitted_when_eligible` keeps its retained meaning and text unchanged.
- **The exact entries.**
  - `"admitted"` on the surfaces the owner opened (ruling 10A): `result-export`, `stress-neutral`, `rule-check`.
  - `"refused"` on every other surface, `report-package` included.

On an admitted surface the exact identities then behave as preview-physics-2 behaves today. Eligibility is the surfaces' own existing checks (§4.2–4.4), and no retained-precision text can appear on an exact result.

A non-eligible exact result is therefore never refused by the policy functions on an admitted surface. It is refused where each surface already checks standing:
- the result export's Current builder, `CURRENT_NUMERICAL_INTEGRITY_NEEDS_RECOMPUTE` (`resultExportAdapter.ts:195`);
- the stress-neutral builder, `SN-NUMERICAL-INTEGRITY-NEEDS-RECOMPUTE` (`StressNeutralExportPanel.tsx:424`);
- the rule-check service, with Rust's finding and the §4.4 text.

A non-eligible exact result is never Current, so the Current-receiving panels show their empty state for it.

### 5.2 The gate reads the surface

- **The decision stays a function of the route.** The owner ruled that load/reference-state results follow the reader (ruling 11B, §8), so the policy adds no source-level condition. Every level reads the route's per-surface entry:
  - `surfaceRouteRefusal`, the route level, for pure projections and validators;
  - `surfaceOutputRefusal`, the Current level, for panels and builders;
  - the gate;
  - the rule-check service.

  `routeOutputRefusal` and `loadReferenceOutputRefusal` stay for their remaining direct callers (`RenderedReportPanel.tsx:70`, `outputPolicy.test.ts:149`). They return the route's shared reason, which for the exact routes is the reworded text (§5.3). Their only remaining product caller is the rendered report, which stays refused.
- **The change.** `LoadReferenceOutputGate` takes the surface id its panel already passes as `testIdPrefix`; the surfaces are named by these prefixes (`outputPolicy.ts:60-61`). It calls `surfaceRouteRefusal(result, surface)` in place of `routeOutputRefusal`.
- **What stays the same.** For every route and every gated surface the decision is unchanged, except the exact routes on an admitted gated surface; today that is `rule-check` only. The reason: the retained entries refuse on all eighteen gated surfaces, and the ungated routes add no refusal.
- **The check.** A maintained test enumerates every route and every gated surface. It asserts that the refuse or admit decision is equal, except in the opened cells. It compares decisions, not texts, because the exact routes' refusal texts are reworded (§5.3). It runs on a plain and a load/reference-state source of each exact identity, and asserts that the two decide alike on every surface.

### 5.3 Texts

- **The code is kept.** `PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE` stays the code of every remaining exact refusal. AC-006 keeps reading as written: "The desktop output policy refuses each named identity with its stated code".
- **The two texts are reworded.** They no longer say every output is unavailable. This follows Decision 12's rewording of the retained text (`outputPolicy.ts:46-47`). Under the opened surfaces:
  - `N_PRESSURE_2_OUTPUT`: "This output of pressure-2 results (3.0.0/exact_pressure_v3) is not available on the desktop; only the result JSON and stress-neutral exports and the rule check admit a numerically eligible result. The result remains readable here; this is not a finding about the result."
  - `N_PRESSURE_SOURCE_OUTPUT`: the same sentence with "pressure-source-1".
- **No third text.** A load/reference-state result takes its identity's text on every refused surface. The text names the opened surfaces, and those surfaces do admit an eligible load/reference-state result (§5.4), so the text is true for it.
- **No new notice on the opened panels, other than the §5.4 disclosure line.**
  - `knownSemanticNotices` emits nothing for the exact routes (`knownSemanticLimitations.ts:117-134`), and this unit adds nothing in its place.
  - The exported document and package bind the identity's semantic table by id and sha256 (§4.2, §4.3). That table carries the identity's `supported_profile_limitations`, so the limitations travel with the output by reference.
  - Neither available limitations text may simply be shown on more panels:
    - The producer's `formulation_basis.limitations` are not bound to the table for the exact routes. The reader checks only that they are non-empty strings (`semantic_contract.rs:605-611`; `source_blocks.rs:610-616`); equality with the table is enforced for preview-physics and the retained routes only (`preview_physics_evidence.rs:265`; `retained_precision.rs:6681`).
    - Both the producer texts and the tables' `supported_profile_limitations` contain acceptance-boundary sentences that are not registered statements. Examples: "No code compliance or professional acceptance is produced" in the producer fixtures; "do not authenticate a producer or grant engineering acceptance" in `semantic_contract_v0_3_pressure_2.json`. Placing either on new surfaces conflicts with `docs/claims_registry.md` §4.
  - The results panel already shows the producer texts in its standing line (`ResultsPanel.tsx:110`), a placement older than this unit (§9).
- **This is also why reports stay closed (§4.5).** A report body must state its limitations in prose. An export binds them by reference.
- **The texts are checked twice.** They are reviewed against `docs/claims_registry.md` and pass GEN-13 (repository-root `tools/validation/validate_claims_language.py`, which scans the desktop TS sources). They carry no BS-ACCEPT text and no prohibition litany. They are UI reasons, never written into a document (`knownSemanticLimitations.ts:1-3`).
- **Comments.** The stale comments that state the old refusal are updated with the code: `outputPolicy.ts:17-26, :39-44, :83-91`, `loadReferenceOutputAvailability.ts`, `analysisRunCompatibility.ts:122-125`, `resultExportAdapter.ts:56-57`, `StressNeutralExportPanel.tsx:413, :421-423, :849-851`, `knownSemanticLimitations.ts:10-16`, `ruleCheckService.ts:137-139`, `reportPackageRequest.ts:23-27`, `resultExportAdapter.ts:191` and `LoadReferenceOutputGate.tsx:5-7`. `loadReferenceOutputAvailability.ts:3-5` states T1's deferral of every load/reference-state output; its comment records that the owner's ruling of 2026-10-21 ends that deferral for the exact identities. The same holds for the header comments of the test files that state it, for example `loadReference.resultsSessionState.test.ts:1-7`.

### 5.4 Load/reference-state results follow the reader

Ruling 11B (§8) leaves two questions open in the design: whether the committed pressure-2 load/reference-state results are numerically eligible, and what their outputs disclose about the reference states.

**Eligibility, from the readers' existing outcomes.**

- **The pinned native record.** R2 (`fixtures/results/base_reader_outcomes.json`) records Rust's dispatch and standing for every committed load/reference-state raw with its request model. `base_reader_outcomes.rs` pins it natively, and `baseReaderWasmEquivalence.test.ts` matches the wasm build to it input for input (AC-005). On main at `fa0028a7d7`:

  | Input | Mode | Standing |
  |---|---|---|
  | `pressure/load_reference/connected` | sparse, dense | `numerically_eligible`, no findings |
  | `pressure/load_reference/pressure` | sparse, dense | `numerically_eligible`, no findings |
  | `pressure_source/load_reference_source/{n05,n06,fields,mixed,eigen_motion}` | sparse, dense | `needs_recompute`: `LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE_IN_T1` |
  | `load_reference_fallback_uz` (pressure-2, result-export test fixture) | sparse, dense | `needs_recompute`: `NUMERICAL_INTEGRITY_NOT_QUALIFIED`, `NUMERICAL_CASE_EVIDENCE_INCOMPLETE` |

- **The desktop already agrees.** `loadReference.resultsSessionState.test.ts:54` asserts the connected raw eligible in both modes, through the same reader. It also asserts the fallback raw not eligible (`:94`) and the joined raws not eligible with the T1 finding (`:117`).
- **The twins come from these raws.** Rust's `pressure_load_state_{connected,pressure}_{sparse,dense}.document.json` are derived from exactly these four raws and their request models, and are pinned byte for byte (`load_reference_contract.rs:439-490`). Python's `.stress_neutral.json` twins are built from the same raws (`tests/test_stress_neutral_physics_source.py:484-512`), and its `.analysis_run.json` twins are written from them by `tests/test_load_reference_readers.py:291-306`. One member of the Rust twins is not source-derived: their `load_basis_refs` come from the writer's invented document template (`load_reference_contract.rs:243-246`, `LC-TP-PHYS-014`), not from the request model. §6.2 therefore checks that member against the Python records. Those are built from the raw sources (`test_load_reference_readers.py:301-302`), and their load cases equal the request models'.
- **Conclusion.** The committed pressure-2 load/reference-state results are numerically eligible in both modes, on Rust's own standing. No reader change, regeneration or new eligibility rule is needed. pressure-source-1 load/reference-state results stay never eligible by Rust's own finding. The pressure-2 fallback is a not-eligible pressure-2 load/reference-state case, so the refusal by standing has a committed fixture too.

**The policy.** Nothing is added. A load/reference-state result is decided by its route's entry (§5.2) and then by each surface's own standing checks (§4.2–4.4), as every other result is. So:

- An eligible pressure-2 load/reference-state result is Current, and the three opened surfaces admit it.
- pressure-source-1 load/reference-state results are refused at the opened surfaces' standing checks (§5.1): the builders' codes, and at the rule-check service Rust's finding, `LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE_IN_T1` (or `SOURCE_BLOCKS_VALIDATED_INVOCATION_REQUIRED` without a registration). They never become Current, so a Current-receiving panel never sees one; the policy functions return null for them on the opened surfaces, as for any exact result.
- Every other surface refuses both, with the identity's code and reworded text.

**What the outputs disclose about the reference states.**

- **In the files: the reference states travel as data, unchanged.** The result JSON document and the stress-neutral package carry exactly the members Rust's derivative and Python's package carry. The desktop adds none and drops none. Three members disclose the reference states:
  - `contract_evidence.load_reference_states`, copied whole (§4.2, §4.3). For each load case it carries:
    - the reference configuration;
    - the resolved element states (material point, temperatures, expansion law, fit and eigenstrain);
    - each contribution with its classification (`eigenstrain`, `prescribed_boundary`, `ordinary_applied`);
    - the excluded stored sources with their reasons;
    - the history kind.
  - `formulation_basis.limitations`, copied as the producer published them. Rust's derivative and Python's package copy them already (`load_reference_contract.rs:444-456`; `core/handoff/stress_neutral/package.py:476`), and so do the desktop builders (`resultExportAdapter.ts:59`; `StressNeutralExportPanel.tsx:516`). They include the load/reference-state limitations, for example "History is independent equilibrium only; no installation, contact, friction or predecessor history is represented."
  - `load_basis_refs` in the document and the record.

  The §6.2 oracle checks this parity on the committed twins. No sentence is written into a file, consistent with §5.3.
- **On the panels: one disclosure line, from the admitted bytes.** Each opened panel shows one line when the result carries `contract_evidence.load_reference_states`:
  - the result export panel;
  - the stress-neutral panel;
  - the rule-check completeness panel;
  - the rule-check run panel.

  A new `features/results/loadReferenceDisclosure.ts` builds the line, and a small component renders it with the test id `{surface}-load-reference-disclosure`. The line is shown only when the surface admits the result, which on these surfaces means Rust's eligible standing. The export, stress-neutral and completeness panels compute it from their Current result. The run panel takes it from the service's route, computed from the qualified source on both the Current and the transient path (§4.4). It reads only the published records' `reference_configuration_id`, `load_case_id` and `history.kind`, in published order, without duplicates. It computes and converts nothing, and it changes no standing, row or byte.

  The texts are below. `{configurations}`, `{cases}` and `{histories}` are the published identifiers, comma-separated.
  - **Result export:** `N_LOAD_REFERENCE_EXPORT`: "This result includes resolved load/reference states (reference configuration {configurations}; load cases {cases}; history {histories}). The export carries them in contract_evidence.load_reference_states, with the producer's limitations."
  - **Stress-neutral:** `N_LOAD_REFERENCE_PACKAGE`: the same first sentence, then "The package carries them in contract_evidence.load_reference_states, with the producer's limitations."
  - **Both rule-check panels:** `N_LOAD_REFERENCE_RULE_CHECK`: the same first sentence, then "Rules bind the published rows of these load cases, which are solved with these states' resolved contributions."
- **Registry classification.** Under `docs/claims_registry.md` §4, each line is a deliverable-specific scope statement in ordinary prose. It is not a boundary statement.
  - It carries no BS-ACCEPT text or short variant.
  - It carries no "not authoritative"-family phrasing and no prohibition litany.
  - It carries no status label.
  - The rule-check panels keep showing each rule status in its §2.1 display form with its authority domain, unchanged.
  - The stress-neutral panel's BS-VALID placement is unchanged, and no second boundary statement is added there ("one surface, one boundary").
  - The producer's own limitation sentences are not placed on the panels (§5.3).
  - The lines are UI text, never written into a document, and they pass GEN-13.
- **Unchanged.** The solve panel's `LoadReferenceStatesBlock` keeps its full read-only view and its labels (`features/solve/LoadReferenceStatesBlock.tsx`). Its "needs recompute — not Current in T1" label for the joined route remains true, by Rust's finding.

## 6. Verification

### 6.1 The desktop output-policy tests (VER-006, AC-007 (c))

- **`outputPolicy.test.ts`.** The exact entries are asserted cell by cell: admitted on the opened surfaces and refused, with the code, on every other surface, the report package included. Every other assertion still holds: fail-closed routes and surfaces, the retained entries unchanged, and one entry per route.
- **The §5.2 equality test.**
- **Load/reference-state results (§5.4).** `outputPolicy.test.ts:138-149` becomes a cell-by-cell test for both identities' load/reference-state raws:
  - At both policy levels (`surfaceRouteRefusal`, `surfaceOutputRefusal`), the opened surfaces return null for every exact load/reference-state raw, and every other surface refuses with the §5.3 text. The policy functions do not read standing on an `"admitted"` cell (§5.1).
  - The standing declines are asserted where they occur (§5.1), in `loadReferenceOutputRefusal.test.tsx` for the panels and builders and in `ruleCheckService.test.ts` for the run:
    - the pressure-2 raws with their request models pass every surface's standing check;
    - the pressure-source-1 raws are refused by every builder's standing code, and by the rule-check service with Rust's finding (`SOURCE_BLOCKS_VALIDATED_INVOCATION_REQUIRED` unregistered; `LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE_IN_T1` registered);
    - the pressure-2 fallback raw (`load_reference_fallback_uz`) is refused likewise (`NUMERICAL_INTEGRITY_NOT_QUALIFIED`).
- **Refusal tests that become admission tests, with the refusal kept on every unopened surface.** The test files are those found asserting the exact refusals:

| Test file | Lines |
|---|---|
| `features/results/loadReferenceOutputRefusal.test.tsx` | 78, 117, 202, 220 |
| `features/results/pressureSourceIntegration.test.tsx` | 133-156 |
| `features/results/ResultsPanel.test.tsx` | 93 |
| `features/results/KnownSemanticNotices.test.tsx` | 63-67 |
| `features/results/knownSemanticLimitations.test.ts` | 36-39 |
| `features/workspace/exactSessionSolve.session.test.tsx` | 177-202 |
| `features/workspace/loadReference.resultsSessionState.test.ts` | 51 and 65 (the eligible raw becomes Current), 103-110 |
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
| `features/results/retainedPrecisionIntegration.test.tsx` | 491-509, 539-548: unchanged, and must still pass (the retained run path, §4.4) |

- **Admission.** For each opened surface and each identity, in both modes, an eligible Current result yields the output. For pressure-2 this covers a plain result and both committed load/reference-state results (`connected`, `pressure`).
  - The panel shows its link only while the live binding holds, and a model edit voids it. For a load/reference-state result, that includes an edit of a reference configuration or an analysis state (`loadReference.resultsSessionState.test.ts:67-76`).
  - Each refusal is kept and asserted:
    - a non-eligible standing, including the pressure-2 fallback and every pressure-source-1 load/reference-state result;
    - a missing or mismatched record;
    - a pressure-source-1 result without its registered invocation;
    - a saved or copied result, which is not Current.
- **The disclosure line (§5.4).**
  - For an admitted pressure-2 load/reference-state result, each opened panel shows its line. The line names the published reference configurations, load cases and history kinds.
  - No line appears for a plain result, nor on a refused surface.
  - The line's text is asserted against its constant, and the constants pass GEN-13.
  - On the rule-check run panel the line appears on both the Current and the transient path, from the service's route. A basis change or a new run clears it with the result.
- **Contract mutations.** Each is refused by name, with the codes of §4.2 and §4.3:
  - an exact document or package whose `contract_evidence` differs from its source's;
  - a receipt on pressure-2;
  - a missing or changed receipt on pressure-source-1;
  - a retained receipt on either identity;
  - a pressure-2 package validated without its source (`SOURCE_PRESSURE_TRANSPORT_UNSUPPORTED`);
  - a pressure-source-1 source with a row mutated and its hashes recomputed coherently. The full reader refuses it (§3), where the shallow check would not.
- **Rule check.**
  - A pressure-2 and a pressure-source-1 run, on both the Current and the transient path, reach `run_rule_checks` with their invocation arguments only at the desktop's eligible standing. An eligible pressure-2 load/reference-state result reaches it too. This replaces `loadReference.resultsSessionState.test.ts:103-110`, which asserts today that such a result never reaches the backend. A pressure-source-1 load/reference-state result never reaches it (`:112-126`, kept).
  - A refused policy cell, or a non-eligible standing, stops the run before invoke.
  - The jsdom tests mock the backend. So a Tauri test runs `qualify_rule_mechanics_with_context` and the row binding on committed fixtures. It covers pressure-2 without an invocation, plain and load/reference-state (`pressure/load_reference/connected`), and pressure-source-1 with its registered invocation. For pressure-source-1 the plain raw is admitted, and a load/reference-state raw is refused (`RULE_NUMERICAL_NEEDS_RECOMPUTE`; its R2 token is `needs_recompute`). `qualify_rule_mechanics_with_context` is private, so these tests go in `lib.rs`'s inline test module, beside the existing exact-route tests (`lib.rs:5254-5300`). `lib.rs` therefore changes in its test module only. That crate is outside `core/product_physics`'s closure. Neither `npm run test:desktop` nor the release sweep runs these tests: the sweep discovers Cargo manifests only under `core` and `validation/benchmarks` (`tools/release/check_release_readiness.py:28`). So the implementation runs them explicitly on its head, `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` with the tests' filter, and the PR names the head.
- **Persistence.** An exact run with a rule status saves, reopens as historical evidence, and validates against its saved source. A pressure-2 load/reference-state run is among them. It is never Current after reopening.

### 6.2 Cross-language agreement

The external bases for the two projections are Rust's derivative and Python's package. Two kinds of check use them, for each identity in both modes.

- **Validator acceptance (the committed twins).**
  - The desktop's `validateResultDocument` accepts Rust's committed `*.document.json` twin with its source.
  - The desktop's `validateStressNeutralExportPacket` accepts Python's committed `*.stress_neutral.json` twin with its source and record.
  - These twins exist only for load/reference-state cases (`fixtures/results/pressure_load_state_*`, `pressure_source_load_state_*`). Under ruling 11B, all of them are acceptance fixtures for the validators, which check contract parity and not standing. That covers 4 pressure-2 and 10 pressure-source-1 inputs, each with its document and its package.
- **The admitted-case oracle for pressure-2 load/reference-state results: the committed twins.** The four pressure-2 twins are derived from raws that read numerically eligible (§5.4). They serve as the builders' equal-input oracle:
  - **The inputs.** `fixtures/product_preview/pressure/load_reference/{connected,pressure}-{sparse_interactive,dense_scrutiny}.raw.json`, each with its `*.request.json` model.
  - **The check.** The desktop session solves each input through the native replay (as `loadReference.resultsSessionState.test.ts` does). It builds the Current result export and the stress-neutral package, and requires their source-derived members to equal the committed twins'. For the document, those are rows and accounting, `contract_evidence` (with `load_reference_states`), `formulation_basis`, `numerical_quality`, `producer` and the semantic contract reference. For the package, they are rows, CSV, witnesses, stable-ID map, annotations, `contract_evidence`, `formulation_basis`, `producer`, `numerical_quality` and the semantic contract.
  - **`load_basis_refs`, checked against its own basis.** The Rust twins' `load_basis_refs` are template members, not source-derived (§5.4), so they are not compared. The desktop document's `load_basis_refs` must instead equal the load cases of the desktop's 0.3 record. That record's `load_basis_refs` must equal the committed Python `.analysis_run.json` twin's (`case:cold`, `case:hot`, or their `-pressure` counterparts), which Python builds from the raw source. The record's refs are shaped `{object_type, ref}` and the document's `{ref_type, ref_id}`; the test maps `LoadCase`/`ref` to `load_case`/`ref_id` explicitly. The same rule applies to the plain oracle below. No Rust writer or existing twin changes.
  - **The excluded members.** Run-specific members, such as identifiers, hashes over the desktop's own record and manifest, and solver build references, are named and excluded explicitly in the test.
- **The pressure-source-1 load/reference-state twins stay refusal fixtures at the builders.** They are never eligible (§5.4). The desktop builders refuse them by standing, and the test asserts each builder's standing code (§5.1) for each of the ten inputs, with Rust's finding in the standing.
- **An admitted-case oracle for plain results (equal inputs).** No committed twin covers a plain exact result, and no committed pressure-source-1 twin is eligible. So the implementation adds an equal-input oracle for eligible plain results:
  - **The inputs.** pressure-2: `fixtures/results/pressure_connected_mechanics_{sparse,dense}.json` with `core/product_physics/tests/fixtures/exact_pressure_connected_request.json`. pressure-source-1: at least one plain raw control from `fixtures/product_preview/pressure_source/physics_source/`, both modes, with its request.
  - **Stress-neutral.** The existing Python writer (`tests/test_stress_neutral_physics_source.py`, `STRESS_NEUTRAL_WRITE_FIXTURES=1`) is extended to write Python's package and record for these inputs as new committed twins. The desktop test builds its package from the same inputs and requires the source-derived members to equal Python's: rows, CSV, witnesses, stable-ID map, annotations, `contract_evidence`, `source_block_recovery`, `formulation_basis`, `producer`, `numerical_quality` and the semantic contract. The desktop validator also accepts Python's package, and the Python validator test reads the desktop-shaped members through its own checks.
  - **Result JSON.** The source-derived members of the desktop document are compared with a Rust `derive_document` output for the same source. That twin comes from a writer outside `core/reporting/result_export/**`. The headless runner's existing export test already writes plain pressure-2 raw and document pairs under `HEADLESS_PRESSURE_OUTPUT_DIR` (`core/runner/headless/tests/pressure_export.rs:13-15`), and its crate is outside `core/product_physics`'s closure. The comparison is required for both identities in both modes. For pressure-source-1, the existing headless writer `core/runner/headless/tests/physics_source_connected.rs:178-200` already writes raw and document pairs under `HEADLESS_COMPOSITE_OUTPUT_DIR`. Validator acceptance and the desktop round trip do not replace it, because no committed Rust twin is a plain result, and no pressure-source-1 twin is eligible.
  - **Fixture rules.** These are new files written by existing writers, or temporary outputs of those writers read in the same check. No existing committed fixture changes (§10). Epoch 7 (R-A) is on main through #1497, and its regenerated pressure-source-1 bytes are the basis. Tests derive digests from the fixtures and pin no received digest.

### 6.3 AC-005 and the readers

No reader source changes, so the native outcomes and the wasm build's inputs are unchanged. These pass unchanged on the head:

- `baseReaderWasmEquivalence.test.ts` (R2, `fixtures/results/base_reader_outcomes.json`);
- `retainedPrecisionWasmEquivalence.test.ts` (R1);
- `test-support/retainedPrecisionCorpus.test.ts`;
- `pressureSourceCorpusParity.test.ts`;
- the verifier-independence test, the CLI binding and Python `rust_reader`.

A change in any committed outcome record is a defect signal here, not something to regenerate.

VER-007 requires VER-001 to VER-006 on the head of each duty's own change (DEL-04-08 ScopeOfWork VER-007). A change under `apps/desktop/**` selects only `desktop-test` and `desktop-build` (`software-workflow.json:64-73`). So the implementation runs the rest on its head, and the PR names the head:
- the AC-005 Rust and Python checks, which are `base_reader_outcomes.rs`, the R1 outcome test, `reader_cli.rs`, `piping-pytest`'s `rust_reader` and the verifier-independence test;
- the VER-006 release-sweep tests, through `tools/release/run_evidence_sweep.py --execute`, or the named tests directly;
- the §6.1 Tauri tests, with `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`, which no other command runs.

These are not reused by an invariance argument.

### 6.4 Where checks run, and independent review

- **Where the vitest suite runs.** `npm run test:desktop` runs in hosted CI on dispatch and on a pull request whose diff selects `piping-desktop` (`docs/CI_STRATEGY.md`); when epoch 8 was planned it ran on dispatch only. Beyond it, the required pull-request lane runs only `r2-smoke.spec.ts`, `b3b-project-persistence.spec.ts` and, since #1496, `rotation-glyph.spec.ts` (`:110` on `fa0028a7d7`). All three are pressure-free; the last is a viewport harness with no model result. The broad browser matrix runs on dispatch only (`:236-258`). So:
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
  7. **Persistence.** Saved exact runs with a rule status (§4.4), and the transient rule-check path.
  8. **Load/reference-state results.** That the eligibility evidence of §5.4 is Rust's own, and that no source-level step is added. That the disclosure lines read only admitted bytes and pass the registry. That the builders' members equal the committed pressure-2 twins.

## 7. Owners, implementation and reconciliation

**Owner of the implementation.** One implementation manager (WORKING_ITEMS) owns epoch 8's implementation and its reconciliation, dispatched by HELP_HUMAN after epoch 7 merges. Under the owner rule of 2026-10-17, the reconciliation of the affected ScopeOfWork, Design and dependency conditions comes in the same implementation PR, before integration. Commitment and acceptance-criterion text stays the owner's (§8). The deliverables it affects:

| Deliverable | Owns | Implementation | Reconciliation |
|---|---|---|---|
| DEL-07-05 Results viewer | `outputPolicy.ts`, `loadReferenceOutputAvailability.ts`, `LoadReferenceOutputGate.tsx`, `knownSemanticLimitations.ts`, notices | §5; fresh set (§4.1); `loadReferenceDisclosure.ts` (§5.4) | ScopeOfWork "Current reading and output boundary": the refusal sentence is replaced (proposed text below) |
| DEL-08-04 Result export format | `apps/desktop/src/features/result-export/**` (and the Rust derivative, unchanged) | §4.2 | none: its ScopeOfWork names neither exact identity |
| DEL-17-06 Stress-neutral CSV/JSON package | `apps/desktop/src/features/stress-neutral/**`, Python package (unchanged) | §4.3 | ScopeOfWork "Package member path references" row (proposed text below) |
| DEL-07-12 Interoperability and evidence-review workspace | `stress-neutral/**` (shared), the handoff panels | none beyond §4.3; its panels keep refusing | none expected |
| DEL-07-04 Missing-data warning and blocking UX | `rule-check/**`, `services/ruleCheckService.ts` | §4.4 | none: its ScopeOfWork names neither exact identity |
| DEL-08-06 State, comparison, and handoff report sections | `reportPackageRequest.ts` | §4.5: the exact arm at `:31` goes | ScopeOfWork "Desktop report-package request" (proposed text below) |
| DEL-08-01 Calculation report generator | `ReportPanel.tsx`, `RenderedReportPanel.tsx` | none (they keep refusing) | none |
| DEL-07-11 Desktop workspace shell | `App.tsx`, `features/workspace/**` (Current) | none expected; Current changes through the fresh set | none expected |
| DEL-04-08 Retained-precision result verification | co-lists `outputPolicy.ts`; holds REQ-006 (c), REQ-007 (c), CLM-003, AC-006, AC-007, VER-006, VER-007 | none of its code | REQ-006 (c), CLM-003, VER-006, VER-007 and the REQ-007 (c) discharge, in the owner-confirmed text (§8). Design `b7-plan.md`: D6 (`:198`) and the epoch 8 status paragraph (`:24` after #1498) |
| DEL-04-07 (this deliverable) | this design; [pressure-contract.md](pressure-contract.md) | none | `pressure-contract.md`: "Reading pressure-2" and the D7 paragraph (`:24`), whose clauses state the refusal as current, become pointers to this design as built; this file's as-built note |
| DEL-05-03 Section bases and stress definitions | none | none | Design `section-bases-and-timoshenko.md:210`: "Desktop output surfaces for the exact identities stay refused" becomes the as-built pointer below |

- **TASK contributions.** TASKs, if used, report their slices' effects. The manager reconciles the combined result.
- **Scope of the design PRs.** The design PRs edit only DEL-04-07. DEL-04-08's edits come with the implementation PR, in the text the owner confirms (§8).

**Proposed reconciliation text.** These describe the as-built boundary under the owner's rulings, and change no commitment. Each replaces only the quoted sentence or clause. Clearly historical descriptions (for example "until T4-U2's export branch landed") keep their wording.

- **DEL-07-05**, "Current reading and output boundary". The sentence beginning "Under the owner's 2026-10-15 ruling "Merge with the gap"" becomes:

  > Since epoch 8 (owner, 2026-10-21: "A accept your recommendations for the entire set."; DEL-04-07 Design `desktop-exact-outputs.md`), the result JSON export, the stress-neutral export and the rule check admit pressure-2 and pressure-source-1 results at the Rust reader's numerically eligible standing. An eligible pressure-2 load/reference-state result is admitted with them, and each of those panels discloses its resolved load/reference states; a pressure-source-1 load/reference-state result is never eligible. Every other desktop output surface refuses both identities by name (`PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE`, `apps/desktop/src/features/results/outputPolicy.ts`).
- **DEL-08-06**, "Desktop report-package request". The sentence beginning "The gate refuses preview-physics-2" becomes:

  > The gate refuses preview-physics-2, its retained successor, pressure-2 and pressure-source-1 with T0R's `REPORT-PACKAGE-FRESH-RESULT-UNAVAILABLE` (the exact identities since epoch 8; DEL-04-07 Design `desktop-exact-outputs.md` §4.5).
- **DEL-17-06**, "Package member path references". The sentence beginning "The desktop panel refuses pressure-2 and pressure-source-1 by name" becomes:

  > Since epoch 8 the desktop panel admits pressure-2 and pressure-source-1 at the Rust reader's numerically eligible standing (an eligible pressure-2 load/reference-state result included; a pressure-source-1 one is never eligible), copies `contract_evidence` (and pressure-source-1's receipt) as the Python builder does, and reads a pressure-2 package through its source (DEL-04-07 Design `desktop-exact-outputs.md` §4.3); it admits the retained successors only at numerically eligible standing with the live native capture, which no product caller delivers before B8.
- **DEL-04-08 Design `b7-plan.md`, D6 (`:198`).** The clause "and, as for the ordinary exact identity, the desktop refuses its output on every surface (`PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE` since T4-U4b/U5b; owner, 2026-10-15: "Merge with the gap")" becomes "and, since epoch 8, the desktop's result JSON export, stress-neutral export and rule check admit it at the Rust reader's numerically eligible standing, while every other desktop surface refuses it (`PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE`; DEL-04-07 Design `desktop-exact-outputs.md`)". The epoch 8 status paragraph ("After B7b") gains "As built: on main through #{PR}."
- **DEL-04-07 Design `pressure-contract.md`.** In "Reading pressure-2", the sentences from "and every desktop output surface (export, stress-neutral, rule check, Current) still refuses" to "these refusals stand until that implementation lands" become: "Since epoch 8 the desktop's result JSON export, stress-neutral export and rule check admit pressure-2 and pressure-source-1 at the Rust reader's numerically eligible standing, and every other desktop output surface refuses them by name (`PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE`); the owner's "Merge with the gap" (2026-10-15) and the epoch 8 placement (2026-10-20) are recorded in [desktop-exact-outputs.md](desktop-exact-outputs.md)." The D7 paragraph's closing clause "the desktop refuses its output on" (`:24`) is replaced in the same form.
- **DEL-05-03 Design `section-bases-and-timoshenko.md:210`.** "Desktop output surfaces for the exact identities stay refused (owner, "Merge with the gap")." becomes "Desktop outputs for the exact identities are as built in epoch 8 (DEL-04-07 Design `desktop-exact-outputs.md`)."

## 8. The owner's rulings and the commitment text

The owner ruled on 2026-10-21, accepting HELP_HUMAN's recommended set: "A accept your recommendations for the entire set." Its rulings 10A, 11B and 12 are this section's items 1, 2 and 3.

**1. Which surfaces open (ruling 10A; this design's choice 1 (A)).** Result export, stress-neutral and rule check open (§4.2–4.4).
- These are the surfaces DEL-04-07 named as refused ("export, stress-neutral, rule check, Current"), and each already has its exact-route format and verdict in Rust or Python (§2.4).
- Reports, the report package and the other fifteen surfaces keep refusing (§4.5).
- Reports of exact results remain unavailable until DEL-08-01 designs their limitations sections. The report package is unchanged, closed for every identity.

**2. Load/reference-state results under the exact identities follow the reader (ruling 11B; this design's choice 2 (A)).** The ruling ends T1's deferral (`loadReferenceOutputAvailability.ts:3-5`) for the exact identities. This design discharges what the ruling left open (§5.4):
- **Eligibility.** The committed pressure-2 load/reference-state results are numerically eligible in both modes, on Rust's pinned standing. pressure-source-1 ones stay never eligible, by Rust's own finding (`reader_verdict.rs:427-428`).
- **Policy.** No source-aware refusal is added (§5.2).
- **Disclosure.** The outputs disclose the reference states as data, in the files, and with one disclosure line on each opened panel.

**3. The commitment text (DEL-04-08), revised to match items 1 and 2 (ruling 12).** The owner approved this text as revised to match the rulings, and confirms it before the implementation PR applies it. REQ-006 (c), CLM-003 and the REQ-007 (c) discharge are commitment text. The 2026-10-19 rule lets HELP_HUMAN edit acceptance-criterion wording only. The current text of REQ-006 (c)'s first bullet:

> **(c) No desktop outputs for the current exact identity.**
> - Exact-route results (`pressure-2`, `pressure-source-1`) solve and can be viewed on the desktop with the Rust reader's standing, but every desktop output surface refuses them (`PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE`). The Python AnalysisRun record and the stress-neutral export accept them.

The revised text:

- **REQ-006 (c)**: its heading and first bullet are replaced; the retained-precision bullet is unchanged.

  > **(c) Desktop outputs for the current exact identity.**
  > - An exact-route result (`pressure-2`, `pressure-source-1`) is Current on the desktop only at the Rust reader's numerically eligible standing, with the live native invocation and its matching record. Its result JSON export, stress-neutral export and rule check admit it, and each takes the reader's verdict and standing. Every other desktop output surface refuses it (`PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE`), and the report package refuses it as it refuses every fresh identity (`REPORT-PACKAGE-FRESH-RESULT-UNAVAILABLE`). A `pressure-2` result carrying load/reference states takes the same standing: when eligible it is admitted, the two exports carry its resolved load/reference states, and the admitting panels disclose them. A `pressure-source-1` result carrying them is never numerically eligible (`LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE_IN_T1`), so no desktop output admits it. The Python AnalysisRun record and the Python stress-neutral export accept them all.

- **CLM-003**, "Retained-source recovery". Its third bullet becomes:

  > - The Python AnalysisRun record and the stress-neutral export accept its results. On the desktop an eligible result can be Current, and its result JSON and stress-neutral exports and rule check admit it; a `pressure-source-1` result carrying load/reference states is never eligible (REQ-006 (c)).

- **VER-006**: its `desktop-test` bullet gains the tests that assert the two declines REQ-006 (c) now names beyond the policy's code. It becomes:

  > - `desktop-test`: `apps/desktop/src/features/results/outputPolicy.test.ts`, `retainedPrecisionOutputRefusal.test.tsx`, `loadReferenceOutputRefusal.test.tsx`, `apps/desktop/src/features/report/reportPackageRequest.test.ts` and `apps/desktop/src/services/ruleCheckService.test.ts`;

- **VER-007**: a bullet is added after the O4 bullet, because no registered check runs the Tauri crate's tests (§6.1):

  > - for the desktop outputs, the desktop rule-check backend's exact-route tests in `apps/desktop/src-tauri/src/lib.rs`'s test module, run with `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` and named with their head in the pull request;

- **REQ-007 (c)**: unchanged until AC-007 (c) holds on the implementation head. Then this sentence is appended, in the form of (a) and (b), with the date and pull request filled in:

  > **Discharged on {date}.** The desktop outputs are on main through #{PR}: the result JSON export, the stress-neutral export and the rule check admit `pressure-2` and `pressure-source-1` results at the Rust reader's numerically eligible standing, an eligible `pressure-2` load/reference-state result included, each taking the reader's verdict and standing; every other desktop output surface refuses them (DEL-04-07 Design `desktop-exact-outputs.md`). AC-005 and the desktop output-policy tests pass on that head. Recorded under the owner's decision of 2026-10-21 ("A accept your recommendations for the entire set.").

- **AC-006 and AC-007 (c)**: unchanged. Both read correctly after the change: AC-006's "refuses each named identity with its stated code" holds on every refused surface, VER-006 above names the tests for the report package's, the builders' and the rule check's declines, and VER-007 names the backend's rule-check tests.

Not reserved: `apps/desktop/native-smoke/expected.json` is the procedure for the owner-held G10 check (DEL-07-05 Design `owner-mac-check.md`), so this unit leaves it untouched. The automated smoke therefore runs no exact case on the native host, and the jsdom session tests carry that path.

## 9. Interactions

- **Epoch 7 (R-A), on main through #1497.**
  - **What R-A changed.** R-A regenerated the pressure-source-1 received bytes and their derived records: the raw source controls, `fixtures/results/pressure_source_reader_corpus.json`, and the `pressure_source_load_state_*.{document,stress_neutral,analysis_run}.json` twins (`24e9ce299a`). It did not touch the pressure-2 load/reference-state raws or twins, last written by their writers in O4 (`df857b3b6c`). Desktop tests read these files (`pressureSourceCorpusParity.test.ts:11, :22-27`; the raw controls and twins in the files of §6.1 and §6.2).
  - **What this unit therefore does.**
    1. It implements on main after #1497, on the regenerated bytes.
    2. It pins no received digest; tests derive digests from the fixtures.
    3. It changes no fixture that a writer owns. Its only fixture additions are §6.2's new plain twins, written by existing writers.
  - `fixtures/**` selects no registered check (`software-workflow.json:63-82`), so a fixture-only change does not select `desktop-test`.
  - **R-A's D3 registration** is unaffected, because this unit changes no priced source.
- **The frozen UI benchmark** (pending the owner when this unit was planned; ruled 2026-10-22, `b7-plan.md`, "Owner decision: the frozen UI benchmark").
  - No file under `apps/desktop/e2e/ui-foundation/**` changes.
  - Its models were then 0.2.0, refused at save and edit as `retired_schema`, and could not produce an exact identity, so the benchmark never met an exact result. The owner's ruling regenerated them as pressure-free 0.6.0 models.
  - No e2e spec exercises an exact result.
- **B8 and G10** (held by the owner). Neither bears on this unit.
  - **B8** activates retained successors on the desktop. The retained policy entries, texts and standing are unchanged, and the exact identities are not successors.
  - **G10's** milestone model is pressure-free (model 0.6.0 without `pressure_contract`), and the native smoke's other model has nonlinear supports, which the exact route excludes. So no G10 observation moves. This holds while `expected.json` is left alone (§8).
- **Observations, not changed here.**
  - physics-retained-1 is in neither fresh set, so its `admitted_when_eligible` cells cannot be reached from the product. This is consistent with Rust and belongs to B8.
  - Two inherited labels, both older than this unit:
    - stress-neutral packages built from user models carry `privacy_classification: "public_metadata_and_invented_results"` (`StressNeutralExportPanel.tsx:283`);
    - the Tauri file-name check fixes downloads to `openpipestress-preview-*` (`apps/desktop/src-tauri/src/native_result_download.rs:103-108`).

    The exact identities inherit both. Changing either belongs to its owner (DEL-17-06, DEL-12-01).
  - The exact producers' `formulation_basis.limitations` and the exact tables' `supported_profile_limitations` contain unregistered acceptance-boundary sentences (§5.3). The results panel already shows the producer texts. Reviewing those texts against the registry is changing producer or reviewed-input bytes, which is outside this unit (§10). It belongs to DEL-07-05 and the table owners.

## 10. Blast radius: what stays byte-identical

- `core/reporting/result_export/**`, including its tests and test fixtures.
- Every crate in `core/product_physics`'s dependency closure, and `core/product_physics/**`, including the GENERATED PROFILE block in `retained_memory.rs`. No D3 registration input moves.
- `core/model_operations/operation_applier/**` (the wasm exports), and the product code of `apps/desktop/src-tauri/**`. Only its tests gain the §6.1 rule-check cases, in `lib.rs`'s inline test module (`#[cfg(test)]`), so `lib.rs` changes in test code only.
- The semantic tables `fixtures/results/semantic_contract_v0_3_*.json`, and the schemas `results.v0.3`, `results_pressure_2.v0.3`, `analysis_run.v0.3` and `stress_neutral_export.v0.3`.
- Python product code (`core/analysis_runs/**`, `core/handoff/stress_neutral/**`, `core/reporting/**`) and `tests/verifiers/**`.
- Every existing committed corpus, outcome record, raw control and twin under `fixtures/results/**` and `fixtures/product_preview/**`, and `validation/references/**`. The only fixture additions are §6.2's new plain twins.
- The published bytes of every identity, and the AnalysisRun record format and builder. Reachable records change in one way: a Current exact run can now record a rule status from its rule check (§4.4).
- The retained policy entries, the retained texts and codes, the retained rule-check run path (§4.4), and preview-physics-2's behaviour on every surface.
- `apps/desktop/e2e/ui-foundation/**`, `apps/desktop/native-smoke/**` and `apps/desktop/src/services/projectService.ts` (the owner's deferred decision 7). No change here needs any of them.
- `REVIEWED_INPUTS` and every reviewed-input digest.
- The report package's behaviour for every identity other than the exact ones, which move to the common fresh-result refusal.

Changes are confined to:
- `apps/desktop/src/**` TypeScript and its tests;
- the §6.1 Tauri tests;
- the §6.2 twins and the writer extensions that produce them;
- the comments listed in §5.3;
- the reconciliation text of §7 and the owner-confirmed text of §8.

## 11. As built

On main through #1505. The owner confirmed the §8 text on 2026-10-21 ("14 confirm, 15A, 16 start now."), and the implementation applies it verbatim. Departures from the text above, all within it:

- **The disclosure module.** `features/results/loadReferenceDisclosure.tsx` holds both the line builder and its component. The rule-check run panel's line uses the test id prefix `rule-check-run`, so it is distinct from the completeness panel's. The completeness panel shows its line only when the gate admits the result and Rust's standing is eligible; it does not rely on receiving only the Current result.
- **The stress-neutral validator.** For `pressure_source`, the validator runs the transport reader on the header and, when a source is supplied, the full raw reader on that source, as §3 requires; §4.3 named only the transport arm. A coherently resealed source that the full reader refuses is therefore refused at the validator too.
- **`load_basis_refs`.** An exact-route document names its load bases `{ref_type: "load_case", ref_id}`, as Rust's derivative does; other identities keep the record's `object_type`, so their documents are unchanged (§10).
- **The gate.** `LoadReferenceOutputGate` opens only where the policy adds no refusal at all; any other decision, `admitted_when_eligible` included, refuses there, because the gate reads no standing.
- **The run route.** `RuleCheckRunRoute`'s `loadReferenceDisclosure` member is optional in the type, so that existing mocks of the route still type-check. The service always sets it.
- **The plain twins (§6.2).**
  - Python packages and records: `fixtures/results/pressure_plain_connected_{sparse,dense}.{stress_neutral,analysis_run}.json` and `pressure_source_plain_n05_{sparse,dense}.{stress_neutral,analysis_run}.json`, written by `tests/test_stress_neutral_physics_source.py`.
  - Rust documents: `pressure_plain_connected_{sparse,dense}.document.json` from `core/runner/headless/tests/pressure_export.rs`, and `pressure_source_plain_n05_{sparse,dense}.document.json` from `core/runner/headless/tests/physics_source_connected.rs`.
  - Each headless writer solves the request itself. Its raw is value-equal to the committed raw, with the same canonical digest; only the JSON encoding differs. Each writer asserts that equality and validates its document against the committed raw, so the desktop comparison reads the committed raw.

- **The CSV comparison (§6.2).** The plain twins' CSV is compared as binary64 values, and byte for byte only where the two writers' number texts coincide (`CSV_BYTE_EQUAL`, `exactOutputTwins.test.tsx`); see "CSV number text" below.

- **The desktop-shaped fixtures.** Beyond the plain twins, eight `apps/desktop/src/test-support/desktop-shaped/desktop_shaped_{pressure_plain_connected,pressure_source_plain_n05}_{sparse,dense}.{stress_neutral,analysis_run}.json` files are the desktop builder's own package and record for the plain inputs. They are kept outside `fixtures/`, whose publication corpus (`core/reporting/result_export/tests/hash_corpus.rs`) requires a transported receipt to equal its original as a JSON value; the desktop writes a float zero as `0`, so its transported pressure-source-1 receipt is equal in value and canonical hash, not in JSON number form. They are written by a new writer, `apps/desktop/src/features/result-export/exactOutputTwins.test.tsx` under `DESKTOP_STRESS_NEUTRAL_WRITE_FIXTURES=1`, and byte-checked otherwise. That test also rebuilds them to show the builder deterministic. They exist so that Python's validators read desktop-shaped packages (§6.2): `tests/test_stress_neutral_physics_source.py::test_desktop_shaped_plain_packages_pass_the_python_validators`. Their byte check runs with `desktop-test` (in hosted CI when `piping-desktop` is selected, §6.4); their Python check runs in `piping-pytest`.

**Observations, not changed here.** Assessed after epoch 8 (2026-10-22), each against its commitments as stated below: the record reading of pressure-source-1 was a defect and is repaired; JSON number form and CSV number text conform on the stated reading; document diagnostics were a defect and are repaired.

- **Document diagnostics (repaired).** The desktop document now maps each source diagnostic as Rust's derivative does (`documentDiagnostic`, `resultExportAdapter.ts`; Rust `result_envelope_binding.rs` `export_mechanics_diagnostic`): `{solver_component, source}`, `affected_object: {preview_diagnostic, <id>}`, Rust's class rules and remediation, with the desktop derivative's own provenance. It mapped each to `{preview_entity, affected_refs[0]}` with a fixed class and remediation and no `id`, which lost every affected ref after the first (the n05 raw's `['torque', 'case']` lost `case`), against DEL-08-04 ("Diagnostics must not be dropped by exporters or adapters"; V-6: diagnostics "are preserved") and DEL-04-07 REQ-004. The affected object now names the diagnostic, which traces to the bound raw source and all its refs. The §6.2 oracle compares the diagnostics, less provenance, with the Rust plain twins. The retained-successor derivative goldens, whose fixed desktop-shaped base carries this mapping (`core/reporting/result_export/tests/retained_precision_derivative_golden.rs`), were regenerated: only their diagnostics and derivative hash moved (`test_o4_text_only_envelope_diff.py` binds this), and two input digests in `retained_precision_reader_outcomes.json` moved with them, verdicts unchanged. Every identity's desktop document changes in its diagnostics, preview-physics-2's included; §10 is epoch 8's blast radius and does not bind this later repair.
- **Desktop record reading of pressure-source-1 (repaired).** The desktop's 0.3 record builder and validator (`validateRetainedRecoverySource`, `analysisRunCompatibility.ts`) now read a pressure-source-1 source through the full raw reader, `readerValidate("pressure-source", source)` with no invocation, as Python's record builder does. They used the shallow source-recovery check, which a coherently resealed invalid source passes, against DEL-14-02's condition that a refused source is never recorded. `exactOutputContractMutations.test.ts` shows the builder and the validator refusing such a source. Record bytes are unchanged. It belongs to DEL-14-02.
- **JSON number form.** The desktop's JSON writes every number in JavaScript's form, so a float zero published as `0.0` travels as `0` in its downloads, including a transported pressure-source-1 receipt. Values and canonical hashes are unchanged, and Python's validators and the desktop's accept it. It is inherent in the desktop's JSON writer and older than epoch 8. It belongs to DEL-17-06 and DEL-08-04. For the result document this conforms: DEL-08-04 SCA011-HASH-001 binds copied members by canonical value and keeps exact bytes on the native save (owner, 2026-10-11: "Keep envelope, hash by value."), and the readers compare numbers by value (DEL-04-08 `b7-plan.md` A2). For the stress-neutral package it conforms on the reading that DEL-17-06 CLM-042's "byte/hash parity" means emitted bytes match their recorded checksums and that source preservation is by value (each source value's bits are kept in `source_value_bits`). Byte equality with the producer's number form would be a different reading, the owner's to make.
- **CSV number text.** The desktop stress-neutral CSV writes some small numbers in JavaScript's number text where Python writes `repr` (for example `1e-8` against `1e-08`). The binary64 values are equal. The writer is older than epoch 8. It belongs to DEL-17-06. This conforms on the same reading of CLM-042: DEL-17-06 REQ-006 leaves the exact CSV text `TBD`, both validators read the value as binary64, and each package's CSV checksum is over its own text. Byte parity between the two writers would be the owner's to require.
- **The `eigen_motion` receipt.** The committed pressure-source-1 `eigen_motion` receipt binds an invocation whose request carries no `materials` member, so a desktop solve of it never registers. It is never eligible either way.
- **An unreachable arm.** The rule-check run's policy refusal cannot be reached for the exact routes, because their `rule-check` cell is `admitted`. It stays as a fail-closed guard.
