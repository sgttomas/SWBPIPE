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

## 2026-06-11 — A3 fifth sub-slice: explicit straight-pipe creation (`TP-APP-R2-CONNECTPIPE-001`)

The viewport editor now has an explicit straight-pipe form for user-entered
pipe id, label, endpoint nodes, material, outside diameter, wall thickness,
non-zero local `y_reference`, and provenance. The form queues a structured
`connect_pipe_run` intent with `field_path=pipe_segments` and applies it
through the same operation review/acceptance path as other local-session
edits. Legacy one-click pipe-run viewport gestures remain blocked because
they still carry only underspecified `viewport.connect_pipe_run` data.

The Rust `core/model_operations/operation_applier` crate and browser local
operation mirror both validate the explicit pipe payload: matching id,
`before=not_present`, project length unit, `dimension=length`, non-duplicate
pipe id, existing endpoint nodes/material, positive OD/wall quantities, and
non-zero `y_reference`. Applying the intent appends a new `pipe_segments`
record to the returned session model without mutating the input model in
place. The app test confirms `pipe:P-150` is created, selected in the model
tree and viewport selection layer, visible in the property inspector, and
recorded with local-session acceptance only.

Residuals remain in A3: true canvas raycast/gesture geometry capture,
rigid/component authoring, and broader editor coverage as new authoring
surfaces land.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-92;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor/_run_records/WORKING_ITEMS_RUN_2026-06-11_explicit_straight_pipe_connectivity.md`,
the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and same-named
records under `DEL-16-02_Operation validation and diff preview/_run_records/`
and `DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-11 — A4 first sub-slice: load-case primitive magnitude manager (`TP-APP-R2-LOADMGR-001`)

The desktop app now has a right-rail Load Cases manager over the invented
preview model. It surfaces load-case counts, primitive-load rows, combination
terms, and a focused primitive-load magnitude editor. Selecting
`load:L-100-P` exposes `primitive_loads.2.magnitude.value`; changing the
magnitude queues `op:load-manager-load:L-100-load:L-100-P-magnitude` as a
structured `update_load` intent. Applying the queued operation uses the
existing OperationApplyPanel, records local-session acceptance, clears stale
solve results, and leaves persistence to the Save local path.

Residuals remain in A4: load-case creation, load status/kind editing,
arbitrary primitive-load creation, imposed-displacement authoring breadth,
full combination editing/algebra authoring, unit picker/display retirement
after Phase B, and packaged-Tauri saved-project smoke over edited load data.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-91;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/_run_records/WORKING_ITEMS_RUN_2026-06-11_load_case_manager_primitive_magnitude.md`,
the same-named record under
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-01_Primitive load case engine/_run_records/`,
and the same-named record under
`DEL-05-02_Load-case algebra engine/_run_records/`.

## 2026-06-11 — A8 first sub-slice: Playwright R2 smoke harness (`TP-APP-R2-PLAYWRIGHT-001`)

The desktop workspace now has a Playwright harness and root script
`npm run test:e2e:desktop`. The first smoke test runs the technical-preview
fixture through initial shell checks, local-only boundary checks, a nonblank
and animated Three.js viewport assertion, mechanics preview solve,
`result_rows=647`, viewport displacement-overlay availability, result
filtering/detail inspection for `pipe:P-120`, and deterministic report-packet
export checks.

Vitest is scoped to `src/**/*.test.{ts,tsx}` so the unit suite and Playwright
suite remain separate. Playwright local output directories are ignored.

Residuals remain in A8: authored create/edit -> solve -> report automation,
full manual SMOKE checklist parity, packaged Tauri saved-project solve smoke,
and CI browser provisioning policy.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-90;
`execution/PKG-00_Software Architecture Runway/1_Working/DEL-00-08_Layered software test and acceptance strategy/_run_records/WORKING_ITEMS_RUN_2026-06-11_playwright_r2_smoke_harness.md`.

## 2026-06-11 — A6 first sub-slice: viewport displacement overlay (`TP-APP-R2-DEFORMEDVIEW-001`)

The Three.js viewport now consumes the current mechanics result and renders a
review-only shape overlay after a solved preview run. The first slice uses
available `displacement_magnitude` rows by node, draws a teal overlaid
centerline/marker set, and exposes a toolbar status with node count, maximum
reported displacement magnitude, and an explicit boundary:
`scale=normalized_display_offset_not_physical_length`,
`vector_direction=TBD`, and `professional_claim=false`.

When no result is present the overlay status is `not started`; when the
current mechanics result is incomplete, such as the browser edited-model guard,
the overlay status is `blocked` and no deformed overlay is rendered.

Residuals remain in A6: true directional deformed shape once displacement
vectors exist, support-reaction visualization, stress/governing-ratio views,
and richer result-selection coupling.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-89;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor/_run_records/WORKING_ITEMS_RUN_2026-06-11_viewport_deformation_overlay.md`
and the same-named record under
`DEL-07-05_Results viewer/_run_records/`.

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
