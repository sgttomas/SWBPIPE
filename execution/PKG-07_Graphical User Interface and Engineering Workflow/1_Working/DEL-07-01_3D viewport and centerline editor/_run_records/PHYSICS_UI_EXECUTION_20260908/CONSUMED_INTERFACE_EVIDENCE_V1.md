# U7 consumed-interface evidence — native straight authoring

Date: 2026-09-08
Lane HEAD: `779dedb8670625b36af07b89fc5557470e47c50e`
Status: implementation-ready evidence; no dependency row is adjudicated here

## Adopted preparation

This execution adopts these four preparation records together, with V3's edit guards controlling earlier text:

- `PHYSICS_UI_PREPARATION_20260907/CANDIDATE_NATIVE_AUTHORING_SLICE_V1.md`
- `PHYSICS_UI_PREPARATION_20260907/DEPENDENCY_AND_CASE_ADDENDUM_V2.md`
- `PHYSICS_UI_PREPARATION_20260907/EDIT_READINESS_CLARIFICATION_V3.md`
- `PHYSICS_UI_PREPARATION_20260907/SCENARIO_AND_ACCEPTANCE_V1.md`

This record adds current source evidence, an exact implementation fence, and the selected inspector behavior. It does not expand the adopted product slice.

## DEL-07-01 dependency disposition requested from PROJECT_SETUP

The source below proves which contracts the selected straight-authoring slice consumes. Formal satisfaction remains a PROJECT_SETUP/Owner act.

| Row | Upstream | Exact current producer contract | Exact current consumer | U7 disposition |
|---|---|---|---|---|
| `DAG-002-E0478` | DEL-02-01 canonical domain model | `apps/desktop/src/types.ts:105-169` defines project, analysis status, materials, sections, nodes, and pipe segments; `:612-699` defines structured editor intents and the human-acceptance boundary. Rust `operation_applier/src/lib.rs:2948-3146` and `:3252-3495` validate and construct canonical node/pipe records. | `PipeViewport.tsx:1712-1776` emits `create_node`; `:1779-1854` emits `connect_pipe_run`; `App.tsx:902-965` applies an atomic batch and commits the returned model. | Directly consumed; no producer change identified. Formal row remains for PS. |
| `DAG-002-E0479` | DEL-02-02 units | Node resolver `lib.rs:2969-3052` requires project length metadata and accepted length units; pipe resolver `:3273-3356` does the same. Desktop forms load catalog choices and carry explicit unit/dimension metadata. | `PipeViewport.tsx:224-240`, `:895`, `:1028-1042`, `:1750-1758`, and `:1827-1835`; App passes the intent unchanged to the service. | Directly consumed; no conversion or producer change identified. Formal row remains for PS. |
| `DAG-002-E0480` | DEL-02-05 persistence | `projectService.ts:450-502` builds the blank document; `:504-611` provides create/open/save. Tauri `src-tauri/src/lib.rs:1935-2205` prepares and persists the model document. | `App.tsx:1154-1395` invokes create/open/save and uses revision/epoch guards. `projectService.test.ts:108` covers create/save/open; `:128` covers a blank, explicitly incomplete document. | Directly consumed; incomplete node-only documents are persistable. No producer change identified. Formal row remains for PS. |
| `DAG-002-E0481` | DEL-03-02 section/component schema | `types.ts:136-146` carries named section records; pipe segments carry inline quantities at `:153-168`. `section_bindings.rs:121-162` applies an existing section reference and materializes its pipe dimensions. | The pipe form selects an existing material and emits inline OD/wall at `PipeViewport.tsx:989-1042` and `:1788-1806`. Existing `SectionAssignment.tsx:13-119` remains the separate named-section assignment path. | Directly consumed. The straight pipe can exist with inline section dimensions; named `assign_section` remains a separate existing operation. No producer change identified. Formal row remains for PS. |
| `DAG-002-E0482` | DEL-03-03 bend/elbow fields | No symbol needed by this slice. | No bend/elbow authoring is added or altered. | Untouched and unconsumed. The active formal TBD row is not silently satisfied or deleted. |
| `DAG-002-E0483` | DEL-03-04 branch fields | No symbol needed by this slice. | No branch authoring is added or altered. | Untouched and unconsumed. The active formal TBD row remains. |
| `DAG-002-E0484` | DEL-03-05 rigid components | No symbol needed by this slice. | No valve/flange/reducer/specialty authoring is added or altered. | Untouched and unconsumed. The active formal TBD row remains. |
| `DAG-002-E0485` | DEL-03-06 expansion joint | No symbol needed by this slice. | No expansion-joint authoring is added or altered. | Untouched and unconsumed. The active formal TBD row remains. |

The four unconsumed component rows belong to the broader DEL-07-01 decomposition, not to this accepted first straight-authoring release. U7 makes no change to their recorded status.

## DEL-07-02 semantic activation

The selected interaction adds an inline `Apply` action for the currently selected editable inspector field. That changes inspector behavior even though it reuses the existing operation contract. Therefore all seven DEL-07-02 legacy dependency rows are activated for formal disposition:

| Row | Upstream contract used by the current inspector | Evidence and effect |
|---|---|---|
| `DAG-002-E0486` | DEL-02-01 canonical model | `PropertyInspector.tsx:1-151` reads selected canonical entities and builds an `EditorOperationIntent`. New Apply sends that same exact intent; schema interpretation does not change. |
| `DAG-002-E0487` | DEL-02-05 persistence | Applied edits enter the session model through `App.tsx:998-1085`; later save persists that model through `:1346-1395`. Selection, expansion, and validation UI state remain transient. |
| `DAG-002-E0488` | DEL-03-02 section schema | Existing selected pipe/section fields and the separate `SectionAssignment` integration remain. The Apply addition does not alter the section producer schema. |
| `DEP-007-02-004` | DEL-02-02 units | Inspector quantity parsing and validation use the existing unit catalog and emit exact `unit`/`dimension`; Apply does not add defaults or conversions. |
| `DEP-007-02-005` | DEL-03-01 material provenance | Existing material references and provenance-visible fields remain as supplied by the model. Apply changes only the selected intent's execution route. |
| `DEP-007-02-006` | DEL-06-01 rule-pack schema | Existing rule-pack reference display remains read-only unless the current intent builder already supports the selected field. No new rule semantics are introduced. |
| `DEP-007-02-007` | DEL-06-04 private rule-pack lifecycle/checksum | Existing checksum/source/private status display remains unchanged and is never inferred or rewritten by Apply. |

Current inspector behavior is Queue/Validate only: props at `PropertyInspector.tsx:39-51`, Queue at `:358-367`, Validate at `:368-377`, validation preview at `:934-977`. The implementation adds an `onApplyIntent` callback and an inline Apply control for the same current intent. `App.tsx` supplies the already-existing `handleApplyIntent`; this is a reuse of the service contract and a semantic extension of DEL-07-02.

### Exact old/new selected-revision flow

Current flow:

1. The inspector builds one selected-field intent from the current model and draft.
2. `Queue change` copies it into the global review queue, or `Validate` calls `handleValidateIntent`.
3. A user later applies from `OperationApplyPanel` using `handleApplyIntent`.

Selected new flow:

1. The inspector builds the same selected-field intent with the same model ID, field path, unit, dimension, before, after, and source metadata.
2. `Validate` remains available and renders the existing validation diff.
3. Inline `Apply` passes that exact current intent to `App.handleApplyIntent`; no inspector-side mutation occurs.
4. App freezes the current revision, hashes the current model, calls the existing single-operation service, rejects stale/mismatched completion, creates one undo checkpoint, commits the returned model, clears stale solve results, and retains the receipt.
5. The selected revision witness edits `load:UI-A-FY` magnitude from `350 N` to `500 N`, then uses existing undo and redo. No load-case slicing or selective solve is introduced.

## Existing service guarantees consumed unchanged

- `operationBatchService.ts:55-95` routes validation/application to Tauri when native and to the same WASM exports in browser tests. It preserves claimed model hash and the complete batch receipt.
- `atomic_batch.rs:96-158` applies ordered operations to a temporary current model. A later operation therefore sees an earlier created node. `:160-198` publishes `committed_as_one_batch` only when all steps complete; failure reports `rolled_back_no_model_published`.
- `App.tsx:879-965` freezes model, hash, revision, and submitted batch; checks stale responses and hash identity; requires a complete receipt; records one undo checkpoint; then commits once.
- `App.tsx:548-562`, `:862-877`, `:976-1085`, and `:1098-1142` already invalidate stale work across commit, cancellation, apply, undo, and redo.
- `resolve_create_node` accepts finite explicit coordinates including zero. It has no section, support, load, or solve-readiness gate.
- `resolve_connect_pipe_run` requires distinct existing endpoint IDs, an existing material ID, positive inline OD/wall in accepted length units, wall below radius, a nonzero y-reference, and provenance. It does not prove endpoints have different coordinates, establish a perpendicular frame, require a named section reference, or impose global model completeness.
- Save uses document/storage validity. Node-only incomplete create/save/reopen is an intended regression witness; it is not promoted to a solvable model.

## Current evidence and gaps the implementation must close

Existing tests already prove the underlying seams:

- `App.test.tsx:13550` queued inspector edit, stale-result clearing, re-solve, and save.
- `App.test.tsx:14526` explicit viewport node apply plus undo/redo.
- `App.test.tsx:14674` pointer-to-node draft capture.
- `App.test.tsx:14745` explicit straight-pipe connectivity; `:14895` endpoint picking.
- `App.test.tsx:15011` underspecified node blocking.
- `App.test.tsx:15630` delayed apply discarded after open; `:15660` continuation/cancel.
- `App.test.tsx:15681` shared section behavior.
- `App.test.tsx:15835` delayed batch withdrawal; `:15879` atomic rollback and successful two-member/one-undo batch; `:15944` open/save stale completion handling.
- `operationBatchService.test.ts:16-40` exact native args, WASM route, rollback envelope, and numeric pass-through.
- `projectService.test.ts:108-158` create/save/open and blank incomplete persistence.
- `ExistingToolkitEngine.test.tsx:21-78` section assignment and existing support/material/toolkit behavior.
- `e2e/workspace-layout.spec.ts` covers the persistent workspace, routing, property edit, and analysis surfaces.

The implementation must add evidence for new-endpoint routing as the exact ordered `[create_node, connect_pipe_run]` batch, Add/Apply review state, inline inspector Apply, invalidation when a draft/model/cancel changes, no duplicate on repeated activation, node-only save/reopen, the complete invented scenario, and separate native Tauri execution evidence.

Browser Playwright is useful interaction evidence and may use the WASM route. It is not evidence of a native backend job/run/hash. The native walkthrough must separately record the packaged Tauri route, job/run identifiers, submitted and applied hashes, save/reopen result, solve result, revision, and undo/redo.

## N7 accepted history boundary

The accepted N7 history is the 112-member PASS at `HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY/instances/N7_FINAL_REVIEW/V4_BACKCHECK/RETURN.md`, its `MENU_REPAIR_PARENT_FINAL_HANDOFF_V1.md`, and PR 715. U7 does not restart that review. That acceptance is evidence for its reviewed product/contract members; it does not by itself change any of the fifteen formal dependency-row statuses above.

## Source snapshot hashes

All hashes are SHA-256 at the stated HEAD:

| Path | SHA-256 |
|---|---|
| `apps/desktop/src/types.ts` | `b23991700988d1fc80811089dd9daf54ddddc9690610f21493c71cd7b7a71caf` |
| `apps/desktop/src/App.tsx` | `72f1d67875de11108c696a0923463a4d4c6f8216d391c84b3da1c94ab4ca9a66` |
| `apps/desktop/src/App.test.tsx` | `679981e56cea23cb84924a243a0757680e79e5efb81c5711fd66caa8aae6f942` |
| `apps/desktop/src/features/viewport/PipeViewport.tsx` | `286823e90c3594d8fa893261698036a545674975ed12cb3b19c57dd6817d1c9b` |
| `apps/desktop/src/features/model-tree/PropertyInspector.tsx` | `697df27e88d87e7004c0a331c4905857573b90c428f62310aaf15f3a1fb42022` |
| `apps/desktop/src/features/model-tree/typedInspector.test.tsx` | `9b2c28c7fd4c2757949416d0b061fd5d7a43b147b4395ab4eabb56237b435d95` |
| `apps/desktop/src/features/workspace/WorkspaceToolbar.tsx` | `5c3d907734f75b7f98d0016d0a5c7859cb463253e087544c67b5a464e4ed6420` |
| `apps/desktop/src/styles.css` | `5e239ae690ac918d5aa4577916c816e6accd21eb89023a29f5c7b9e2c9de5b5b` |
| `apps/desktop/src/services/operationBatchService.ts` | `0cf84f7b8ced2786fa03729087fd1197bf8773afb6d15a9653cb86e8a0ecc52f` |
| `apps/desktop/src/services/operationService.ts` | `8f213ac1f3a0afb1f98072b7fa476e6d89ce13bde9b44e6fa56f60a13b5670e2` |
| `apps/desktop/src/services/projectService.ts` | `da38305ab02c69be95cdae4e79c9546568038716d597cc72385883938cc1363d` |
| `core/model_operations/operation_applier/src/lib.rs` | `83f7e3063fc483a15aecd4fc2a513abba1ee001c2cc8f568037281194640cbc7` |
| `core/model_operations/operation_applier/src/atomic_batch.rs` | `f73a763cd9d273e8f802304f1150c316ff5470739692fb41c95dc19ff2450aa6` |
| `core/model_operations/operation_applier/src/section_bindings.rs` | `424cba3f1e6292d75233bd403b60c922e2e56365c7c7b15b463c6efe58d9fd72` |
