# Plan Completion Log

Archive ledger for landed items from the active completion plan (currently
[PLAN_2026-06-10_prd_completion.md](PLAN_2026-06-10_prd_completion.md)). When
a plan item lands, its row in the plan is compressed to one line and the full
narrative moves here, newest entry first. This file is history, not
authority: lifecycle state lives in deliverable `_STATUS.md` files, evidence
lives in `_run_records/**`, and rulings live in the decision register and
`SOFTWARE_DECOMP.md` decision log. Nothing here is a release, professional,
certification, or code-compliance claim.

---

## 2026-06-11 — A5 first sub-slice: model-bound solve guard (`TP-APP-R2-SOLVEBOUND-001`)

The preview mechanics service now refuses to reuse bundled solved-result rows
for an edited model in browser fixture mode. Edited browser-session models
return a `MODEL_INCOMPLETE` mechanics result with zero result rows and an
explicit `BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL` blocking
diagnostic. The unchanged fixture model still returns the bundled solved
fixture for browser preview workflows.

The Tauri backend path was verified as model-bound: direct
`run_preview_mechanics(Some(model))` and the solve-job registry both solve the
supplied edited model payload and publish result envelopes bound to the edited
`project.id`, not the bundled fixture id.

Residuals remain in the A5 row: full packaged-Tauri GUI smoke over a saved
edited project snapshot, richer incomplete-model UI copy, and broader
persisted non-fixture solve coverage as authoring surfaces grow.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-88;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-07_Solve execution UX- progress, cancellation, and diagnostics/_run_records/WORKING_ITEMS_RUN_2026-06-11_edited_model_solve_binding.md`,
the same-named record under
`execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/DEL-14-02_Analysis run records/_run_records/`,
and the same-named record under
`execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-06_Solver diagnostics and singularity detection/_run_records/`.

## 2026-06-10 — A3 fourth sub-slice: session undo/redo checkpoints (`TP-APP-R2-UNDOREDO-001`)

The Apply Operations panel now exposes local-session Undo/Redo controls for
applied structured operations. Applying an operation records a checkpoint for
the previous session model and selection; undo restores that checkpoint,
moves the current model to redo, and clears stale solve results; redo restores
the undone model and clears stale solve results again. The history summary is
explicitly labeled local-session-only and saved-project-mutated=false.

Residuals remain in the A3 row: true canvas raycast/gesture geometry capture,
straight-pipe connectivity creation, and broader editor coverage as new
authoring surfaces land.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-87;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/_run_records/WORKING_ITEMS_RUN_2026-06-10_session_undo_redo_checkpoints.md`
and the same-named record under
`DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-10 — A3 third sub-slice: explicit node create operation (`TP-APP-R2-CREATENODE-001`)

The viewport editor now has an explicit node-geometry form for user-entered
node id, label, and finite x/y/z coordinates in the project length unit. The
form queues a structured `create_node` intent and applies it through the
existing operation seam; the browser local engine and Rust applier accept only
explicit node payloads, reject duplicate ids, and preserve the no-silent
conversion/default posture. The applied target becomes the active model-tree
and inspector selection. During browser smoke this tranche also fixed
viewport/operation-panel hit-test layout issues so the new form and apply
buttons are actionable in the live app.

Residuals remain in the A3 row: true canvas raycast/gesture geometry capture,
straight-pipe connectivity creation, undo/redo, and broader editor coverage
as new authoring surfaces land.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-86;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor/_run_records/WORKING_ITEMS_RUN_2026-06-10_explicit_node_create_operation.md`,
the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and same-named
records under `DEL-16-02_Operation validation and diff preview/_run_records/`
and `DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-10 — A3 second sub-slice: property-inspector inline validation (`TP-APP-R2-INLINEVALID-001`)

The property inspector now exposes validate-only feedback for draft editor
intents before queue/apply. The UI calls the existing structured-operation
validation seam and displays application status, schema/unit/before-state
states, diff rows, diagnostics, and a no-mutation/professional-boundary note.
This landed as an A3 editor UX sub-slice; broader editor coverage, undo/redo,
and true geometry-capture workflows remain in the A3 row.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-85;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/_run_records/WORKING_ITEMS_RUN_2026-06-10_property_inspector_inline_validation.md`
and
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-02_Operation validation and diff preview/_run_records/WORKING_ITEMS_RUN_2026-06-10_property_inspector_inline_validation.md`.

## 2026-06-10 — A3 first sub-slice: viewport selection binding (`TP-APP-R2-VIEWSELECT-001`)

Viewport entity selection controls for loaded nodes, straight pipes,
supports, and component markers now drive shared selection, model-tree
active state, property-inspector binding, and viewport active highlight.
Residuals (remain in the A3 row): true canvas raycast/gesture geometry
capture, node/straight-pipe creation tools with explicit
coordinates/connectivity, undo/redo, and inline validation messages.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-84;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor/_run_records/WORKING_ITEMS_RUN_2026-06-10_viewport_selection_binding.md`
and the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`.

## 2026-06-10 — A2 landed: model-document persistence (`TP-APP-R2-PERSIST-001`)

Implemented under `DEC-019` (D-08 ruling): in-document semver authority,
application-service transform chain (migrate-in-memory-on-open /
persist-on-save), refusal semantics for newer/unsupported documents, store
v9 evidence-only migration ledger with pre/post hashes, and
validation-preflight evidence replacing the prior TBD marker. Open residuals
(remain in the A2 row): compatibility-window size (human ruling), explicit
"Migrate project" operation, sibling JSON-slot coverage.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-83;
`execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-05_Project persistence and round-trip serialization/_run_records/WORKING_ITEMS_RUN_2026-06-10_model_document_schema_migration.md`
(with same-day `TASK_RUN_2026-06-10_*.md` records in the same folder).

## 2026-06-10 — A1 landed: apply-operation command path (`TP-APP-R2-EDITLOOP-001`)

New `core/model_operations/operation_applier` crate plus the
`apply_model_operation` and `validate_model_operation` Tauri commands
(desktop bridge now 14 commands, two on the mutating path) and an Apply
Operations panel. Inspector modify intents apply to the session model;
viewport gesture intents block pending A3 geometry capture; unit conversion
blocks pending D-01/Phase B.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-82;
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-02_Operation validation and diff preview/_run_records/WORKING_ITEMS_RUN_2026-06-10_apply_operation_command_path.md`.

## 2026-06-10 — Decision packets D-01 and D-08 prepared and ruled

Both packets drafted and ruled same day: `D-01` → `DEC-018` (SI-canonical
with dual display catalog, as the packet proposed; Phase B unblocked);
`D-08` → `DEC-019` (per-document semver transform chain,
migrate-in-memory-on-open / persist-on-save, as the packet proposed; A2
unblocked). `D-10` packet drafted the same day, `AWAITING_RULING`. Packets
and state: `execution/_Coordination/_DECISIONS/_REGISTER.md`; rulings:
`SOFTWARE_DECOMP.md` §12. This completed items 1–2 of the plan's original
"first three tranches" sequence; item 3 (A2) landed the same day, above.
