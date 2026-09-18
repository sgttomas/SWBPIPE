# Operations map — SWB Piping Designer UX specification V1

Companion to [`UX_SPEC_V1.md`](UX_SPEC_V1.md), written under brief UX-SPEC-01 on 2026-09-18. Every control and gesture the specification names is one row below: the operation it invokes, and the existing operation id, capability entry or typed interface that provides it, with the source file and line, or **GAP** with one line on what the gap is. Rows are grouped by surface and numbered through. The consolidated gap list at the end is the list the implementers estimate from.

Paths are relative to the repository root. Line numbers are as read on 2026-09-18 in the worktree named in `RETURN.md`. Nothing here is a product claim: a provider row says that a named operation, entry or type exists in the product source, not that the current presentation carries over.

Reading the provider column:

- **change kind** — a `change_kind` the operation applier accepts (`projects/chirality-piping/core/model_operations/operation_applier/src/lib.rs:2137`, `check_kinds`), reached through `applyModelOperation` (`projects/chirality-piping/apps/desktop/src/services/operationService.ts:72`, checked first by `validateModelOperation` at `:57`) or, for several operations at once, `applyOperationBatch` (`projects/chirality-piping/apps/desktop/src/services/operationBatchService.ts:89`, `OperationBatch` at `:4`, `validateOperationBatch` at `:81`). The host commands are `apply_model_operation` and `apply_model_operation_batch` (`projects/chirality-piping/apps/desktop/src-tauri/src/lib.rs:1837`, `:1857`). Every operation carries the intent shape `EditorOperationIntent` (`projects/chirality-piping/apps/desktop/src/types.ts:645`) with `author_type` `"user"` or `"agent"` (`:650`; the schema's `OperationAuthorType` at `projects/chirality-piping/schemas/model_operation.schema.json:306` also names `import_adapter` and `project_template`).
- **composed** — the design's one operation is a batch of existing change kinds assembled by the interface and applied as one unit with one undo checkpoint. It is a provider row, not a gap; the composition is named.
- **derived** — the interface computes a view from data the product already returns; no engine operation is invoked. It is a provider row when the data exists.
- **GAP (engine)**, **GAP (typed interface)**, **GAP (rendering)**, **GAP (interface)** — the class of work the gap implies: an engine operation or result the run does not produce; a field or record the typed interfaces and schemas do not carry; a canvas capability the viewport does not draw; interface behaviour with no engine dependency. Each cites its entry in the gap list (G-nn).

Abbreviations are not used for paths. Key letters follow the specification's notation.

---

## A. Shell — rail, views, toolbar, status bar, palette, menus, preferences, window

| # | Control or gesture | Operation it invokes | Provider (file:line) or GAP |
|---|---|---|---|
| 1 | Rail: Model, Loads, Results, Review | Navigate to a stage (interface navigation; nothing in the spine changes) | `WORKSPACE_SECTIONS` `projects/chirality-piping/apps/desktop/src/App.tsx:407` (section navigation exists; the four stages regroup the sections `operations` `:409`, `loads` `:414`, `solve` `:431`, `results` `:436`, `report` `:441`) |
| 2 | Rail foot: Libraries | Open the Libraries page over the stage | `libraries` section `projects/chirality-piping/apps/desktop/src/App.tsx:419`; list route `listLocalLibraries` `projects/chirality-piping/apps/desktop/src/services/libraryImportService.ts:200` |
| 3 | Rail foot: Rules | Open the Rules page | `rule-packs` section `projects/chirality-piping/apps/desktop/src/App.tsx:425`; `listLocalRulePacks` `projects/chirality-piping/apps/desktop/src/services/rulePackService.ts:184` |
| 4 | Rail foot: Issues (with count) | Open the issues drawer | `view.issues` command `projects/chirality-piping/apps/desktop/src/App.tsx:3383`; `IssuesHome` `:3703` |
| 5 | Rail: Results and Review disabled ("No run yet", "No solved run") | Derived from the presence of a run | `LocalProjectEnvelope.analysis_run` `projects/chirality-piping/apps/desktop/src/types.ts:971`; `SolveJobAuditState.state` `:511` (derived) |
| 6 | Rail caption "Failed" | Derived from the job state | `SolveJobEvent.state` `projects/chirality-piping/apps/desktop/src/types.ts:501` (derived) |
| 7 | Rail caption "Historical" | Derived from the reopened-run designation | `designation: "historical_saved_run"` `projects/chirality-piping/apps/desktop/src/features/results/HistoricalRunContext.tsx:11` (derived) |
| 8 | Rail caption "Stale" | Derived from a run whose model hash differs from the current model's, while the run is kept | **GAP (engine/typed interface)** G-11: results are cleared on any model change (`clearComputedModelState` `projects/chirality-piping/apps/desktop/src/App.tsx:1695`); no stale state exists |
| 9 | View switch Table, Model, Both (⌘1 ⌘2 ⌘3) | Interface layout state | **GAP (interface)** G-28: the product has panels and a viewport (`view.close-panels` `projects/chirality-piping/apps/desktop/src/App.tsx:3386`), not three views |
| 10 | Per-stage view memory and first-open defaults | Interface state saved with the project | **GAP (typed interface)** G-17: `LocalProjectEnvelope` `projects/chirality-piping/apps/desktop/src/types.ts:964` carries no interface state |
| 11 | Toolbar: project name and save state ("· saved", "· edited", "· not saved") | Derived from the model hash against the last saved hash | `computeModelHash` `projects/chirality-piping/apps/desktop/src/services/hashService.ts:117`; `LocalProjectEnvelope.model_hash` `projects/chirality-piping/apps/desktop/src/types.ts:972` (derived) |
| 12 | Toolbar: Run | See K |  |
| 13 | Toolbar: Issues count | Open the drawer | as row 4 |
| 14 | Toolbar: Agent toggle (⌘⇧G) column or strip | Interface layout state | `AgentProposalPanel` `projects/chirality-piping/apps/desktop/src/features/agent-proposals/AgentProposalPanel.tsx:4` exists as a section panel; column/strip layout **GAP (interface)** G-28 |
| 15 | Toolbar: units toggle (SI, US, entered) | Display conversion only | `DisplayUnitPreference` `projects/chirality-piping/apps/desktop/src/features/display-units/targets.ts:1`; `DisplayUnitSelector` `projects/chirality-piping/apps/desktop/src/features/display-units/index.tsx:57`; `convertDisplayQuantities` `projects/chirality-piping/apps/desktop/src/services/displayQuantityService.ts:20` (host `convert_display_quantities` `projects/chirality-piping/apps/desktop/src-tauri/src/lib.rs:1862`; unit catalogue `get_unit_catalog` `:1381`); capability `view.units` `projects/chirality-piping/apps/desktop/src/features/toolkit/capabilityCatalog.ts:345` |
| 16 | Palette ⌘K: commands | Run a command | `ToolkitPalette` `projects/chirality-piping/apps/desktop/src/features/toolkit/ToolkitPalette.tsx:17` (dialog `:101`); `capabilityAvailability` `projects/chirality-piping/apps/desktop/src/features/toolkit/capabilityCatalog.ts:409` |
| 17 | Palette ⌘K: entities (nodes, elements, restraints, cases, runs, sections, materials, rule IDs, issues) | Select and navigate | **GAP (interface)** G-28: the palette searches capabilities only (`placeholder` `projects/chirality-piping/apps/desktop/src/features/toolkit/ToolkitPalette.tsx:118`) |
| 18 | Status bar: status chips | See L |  |
| 19 | Status bar: issues count | Open the drawer | as row 4 |
| 20 | Status bar: selection ("Node 40 · 1 row · 2 proposed rows") | Derived from the selection | primary selection `EntityRef` `projects/chirality-piping/apps/desktop/src/types.ts:803`, state `projects/chirality-piping/apps/desktop/src/App.tsx:540`; ordered multi-row selection with a primary entity **GAP (interface)** G-28 |
| 21 | Status bar: information control (About; M-01 line) | Open About | maturity sentence exists as a string `projects/chirality-piping/apps/desktop/src/App.tsx:3282`; About dialog **GAP (interface)** G-28 |
| 22 | File › New project | Create a project with an empty model | `createLocalProject` `projects/chirality-piping/apps/desktop/src/services/projectService.ts:504` (host `create_local_project` `projects/chirality-piping/apps/desktop/src-tauri/src/lib.rs:2151`); `buildBlankLocalModelDocument` `:450`; menu `file.new-blank` `projects/chirality-piping/apps/desktop/src/App.tsx:3350` |
| 23 | File › Open…, Open recent | Open a project | `openLocalProject` `projects/chirality-piping/apps/desktop/src/services/projectService.ts:541` (host `:2232`); `listLocalProjects` `:555` (host `:2286`); menu `file.open-local` `projects/chirality-piping/apps/desktop/src/App.tsx:3352` |
| 24 | File › Save, Save as… | Save the project envelope | `saveLocalProject` `projects/chirality-piping/apps/desktop/src/services/projectService.ts:571` (host `save_local_project` `projects/chirality-piping/apps/desktop/src-tauri/src/lib.rs:2293`); Save as… composed from `createLocalProject` `:504` then save; menu `file.save-local` `projects/chirality-piping/apps/desktop/src/App.tsx:3355` |
| 25 | File › Import library… | See O row 175 |  |
| 26 | File › Export…, Report preview | See Q and N |  |
| 27 | Edit › Undo (⌘Z) | Restore the checkpoint before the last operation or batch | `handleUndoSessionModelEdit` `projects/chirality-piping/apps/desktop/src/App.tsx:1649`; `SessionModelCheckpoint` `:192`; stack `:622`; capability `edit.undo` `projects/chirality-piping/apps/desktop/src/features/toolkit/capabilityCatalog.ts:315`; one checkpoint per batch `projects/chirality-piping/apps/desktop/src/App.tsx:1336`; menu `edit.undo` `:3363` |
| 28 | Edit › Redo (⇧⌘Z) | Reapply | `handleRedoSessionModelEdit` `projects/chirality-piping/apps/desktop/src/App.tsx:1672`; capability `edit.redo` `projects/chirality-piping/apps/desktop/src/features/toolkit/capabilityCatalog.ts:324`; menu `:3364` |
| 29 | Edit › Undo toast text ("Undid: Gap at node 20 · 3 mm ← 0 mm") | Derived from the receipt's before/after | `AppliedOperationReceipt.before/after` `projects/chirality-piping/apps/desktop/src/types.ts:794` (derived) |
| 30 | Edit › Preferences… › Appearance (Light, Dark, System) | Interface preference | `resolvedUiTheme` `projects/chirality-piping/apps/desktop/src/features/workspace/uiPreferences.ts:79`; `resolvedTheme` `projects/chirality-piping/apps/desktop/src/App.tsx:537` |
| 31 | Edit › Preferences… › Contrast findings (M-16) | Show findings | `AccessibilityBaselinePanel` `projects/chirality-piping/apps/desktop/src/features/accessibility-baseline/AccessibilityBaselinePanel.tsx:4` |
| 32 | Edit › Preferences… › node increment, number formatting | Interface preference | **GAP (interface)** G-28 |
| 33 | View › Inspector (⌘I) | Toggle the inspector | `view.inspector` `projects/chirality-piping/apps/desktop/src/App.tsx:3372` |
| 34 | View › Issues, Agent, Labels, Deformation, Probe, camera presets | See J, M, H |  |
| 35 | Insert › Row, Row below, Split element…, Restraint, Load, Node data, Load case, Combination, Snapshot… | See B, C, E, N | native menu bridge `listenToNativeMenu` `projects/chirality-piping/apps/desktop/src/services/nativeMenu.ts:21`; `NATIVE_MENU_COMMAND_IDS` `projects/chirality-piping/apps/desktop/src/App.tsx:462`; `menus` `:3344` |
| 36 | Analyse › Run, Stop, Run settings…, Run record, Rule checks | See K | menu `analyze.run` `projects/chirality-piping/apps/desktop/src/App.tsx:3405`, `analyze.cancel` `:3406`, `analyze.rule-checks` `:3408` |
| 37 | Analyse › Generate load cases… | See E row 100 |  |
| 38 | Analyse › Renumber nodes… | See B row 66 |  |
| 39 | Help › About, Scope and limitations (M-17) | Open the About page | boundary content exists in `AuditBoundaryDrawer` `projects/chirality-piping/apps/desktop/src/App.tsx:3642`; a scope-and-limitations page per `docs/DIRECTIVE.md` §6 (`projects/chirality-piping/docs/DIRECTIVE.md:105`) **GAP (interface)** G-28 |
| 40 | Window: minimum 1280 × 800, pane floors, slide-over fallback | Interface layout | **GAP (interface)** G-28 |

## B. The layout table

| # | Control or gesture | Operation it invokes | Provider (file:line) or GAP |
|---|---|---|---|
| 41 | Header menu: sort (⌘↓), filter, data bar, precision, unit, hide, pin, width, reset | Table view state; never reorders the model | results filtering exists (`ResultsPanel` filter `projects/chirality-piping/apps/desktop/src/features/results/ResultsPanel.tsx:59`); the one table component with sort-as-view for model tables **GAP (interface)** G-28 |
| 42 | Row selection (gutter click, ⇧, ⌘, ⇧↑ ⇧↓, ⌘A) | Set the product selection | primary selection `EntityRef` `projects/chirality-piping/apps/desktop/src/types.ts:803` / `projects/chirality-piping/apps/desktop/src/App.tsx:540`; multi-row ordered selection **GAP (interface)** G-28 |
| 43 | Keyboard model (↩ ⇥ arrows ⎋ ⌘↩ ⌥↩ ⌘C ⌘V ⌫ Space ⌥I ⌘F) | Cell focus and editing (interface); commits invoke the rows below | **GAP (interface)** G-28: spreadsheet keyboard model of the table component |
| 44 | Edit X, Y, Z (absolute mode) | One `set_field` on the node's coordinates | change kind `set_field` `projects/chirality-piping/core/model_operations/operation_applier/src/lib.rs:2142`; `Node` `projects/chirality-piping/schemas/model.schema.yaml:1215`, `coordinates` `:1220`; `PreviewModel.nodes` `projects/chirality-piping/apps/desktop/src/types.ts:147` |
| 45 | Edit DX, DY, DZ (offsets mode): move the To node and its downstream chain | One batch of `set_field` coordinate changes, one checkpoint | **GAP (engine)** G-02: no translate-chain change kind; nearest `transform_pipe_run` `projects/chirality-piping/core/model_operations/operation_applier/src/lib.rs:2141` (explicit pipe-run transform, `buildGeometryBatch` `projects/chirality-piping/apps/desktop/src/features/geometry-tools/geometryDraft.ts:79`); would be composed from `set_field` per node through the batch route |
| 46 | Edit Node (renumber one node; every reference rewritten) | One renumber operation | **GAP (engine)** G-01: roadmap "Node renumbering" `projects/chirality-piping/apps/desktop/src/features/toolkit/capabilityCatalog.ts:375`; node ids are string `Id`s (`projects/chirality-piping/schemas/model.schema.yaml:1215`) |
| 47 | Edit From (re-anchor the arriving element to an earlier node) | One `set_field` on the element's `start_node_ref`, or composed delete and reconnect | `set_field` `:2142` on `start_node_ref` `projects/chirality-piping/schemas/model.schema.yaml:478`; if reference fields are not settable, composed `delete_pipe_run` `:2153` + `connect_pipe_run` `:2147` (`projects/chirality-piping/core/model_operations/operation_applier/src/lib.rs`) |
| 48 | From shown implied (muted) vs entered (branch) | Derived from the previous row's node | `start_node_ref` `projects/chirality-piping/schemas/model.schema.yaml:478` (derived) |
| 49 | Edit Type: Pipe → Bend, Valve, Reducer, Rigid, Expansion joint | One `insert_component_symbol` at the node (or `delete_component` when returning to Pipe) | `insert_component_symbol` `projects/chirality-piping/core/model_operations/operation_applier/src/lib.rs:2166`; `delete_component` `:2152`; `buildCreateComponentIntent` `projects/chirality-piping/apps/desktop/src/features/component-creation/componentIntent.ts:246`; kinds `creatableComponentKinds` `:3` (bend, tee, reducer, valve, flange, expansion_joint); `component_type` enum incl. `rigid` `projects/chirality-piping/schemas/model.schema.yaml:295`; capability `build.component` `projects/chirality-piping/apps/desktop/src/features/toolkit/capabilityCatalog.ts:51` |
| 50 | Type: Rigid | As row 49 | schema kind exists (`rigid`, `:295`); the creation intent does not offer it: **GAP (interface)** G-29 |
| 51 | Type: kinds not offered (slip joint, ball joint, hinge, tie rod, elastic element, beam, cut pipe, jacketed pipe, jacketed bend, miter bend) with the footer line to Scope and limitations | None | **GAP (engine)** G-24: no component or element kind (`element_type` `projects/chirality-piping/schemas/model.schema.yaml:468`: straight_pipe, frame, rigid, component_link) |
| 52 | Bend fields: radius, wall, material, flexibility factor, SIF in/out, intermediate nodes | `set_field` on the component's geometry and modifiers | `PreviewComponent.geometry.bend_radius` `projects/chirality-piping/apps/desktop/src/types.ts:12`, `bend_angle` `:13`; `modifiers.flexibility_factor_user_value` `:50`, `sif_user_value` `:47`; intermediate nodes composed from `split_pipe_run` `:2140` |
| 53 | Valve fields: weight, offset, factors | `set_field` | `geometry.weight` `projects/chirality-piping/apps/desktop/src/types.ts:32`, `center_of_gravity` `:33`; thickness and insulation factors **GAP (typed interface)** G-23 (no field) |
| 54 | Reducer fields: OD and wall at each end, cone angle | `set_field` | `end_a_size` `:30`, `end_b_size` `:31` (`projects/chirality-piping/apps/desktop/src/types.ts`); wall per end and cone angle **GAP (typed interface)** G-23 |
| 55 | Rigid fields: weight | `set_field` | `geometry.weight` `projects/chirality-piping/apps/desktop/src/types.ts:32`; `rigid_body_length` `:29` |
| 56 | Expansion joint fields: axial, lateral, torsional stiffness, thrust area, bending stiffness, weight | `set_field` | `modifiers.axial_stiffness_user_value` `:54`, `lateral_stiffness_user_value` `:55`, `angular_stiffness_user_value` `:56`, `torsional_stiffness_user_value` `:57`; `geometry.effective_area` `:39`; `weight` `:32` (`projects/chirality-piping/apps/desktop/src/types.ts`) |
| 57 | Section cell: choose or type a section | One `assign_section` (or `detach_section` on clear) | `assign_section` `projects/chirality-piping/core/model_operations/operation_applier/src/lib.rs:2148`; `detach_section` `:2149`; capability `properties.assign-section` `projects/chirality-piping/apps/desktop/src/features/toolkit/capabilityCatalog.ts:168`; `section_ref` `projects/chirality-piping/schemas/model.schema.yaml:487` |
| 58 | Section combobox items from the libraries; "Import library…" when none | List records; materialise a chosen record into the model | `listLocalLibraries` `projects/chirality-piping/apps/desktop/src/services/libraryImportService.ts:200`; `create_section` `projects/chirality-piping/core/model_operations/operation_applier/src/lib.rs:2154`; capability `properties.section` `projects/chirality-piping/apps/desktop/src/features/toolkit/capabilityCatalog.ts:155` |
| 59 | Material cell | One `set_field` on the element's `material_ref`; a library record materialised with `create_material` | `set_field` `:2142`; `material_ref` `projects/chirality-piping/schemas/model.schema.yaml:484`; `create_material` `projects/chirality-piping/core/model_operations/operation_applier/src/lib.rs:2155`; capability `properties.material` `projects/chirality-piping/apps/desktop/src/features/toolkit/capabilityCatalog.ts:127` |
| 60 | Propagation of Section, Material, Load by connectivity (corner tick, source tooltip, clear returns to propagation) | Materialised as `assign_section` / `set_field` on each propagating row, attributed as Propagated | change kinds as rows 57 and 59 (composed per row); the Propagated origin class **GAP (typed interface)** G-07 |
| 61 | Load set cell; T1…Tn, P1…Pn read-through cells; edit-or-fork popover | One operation on the set, or one batch (create set, assign, re-propagate) | **GAP (engine/typed interface)** G-03: no load-set record; nearest `LoadCase.load_type` temperature and pressure `projects/chirality-piping/schemas/model.schema.yaml:522`; capability `properties.temperature` `projects/chirality-piping/apps/desktop/src/features/toolkit/capabilityCatalog.ts:140` |
| 62 | New row (↩ on the last cell, ⌥↩, Add row) | One batch: `create_node` + `connect_pipe_run` | `buildRouteSubmission` `projects/chirality-piping/apps/desktop/src/features/viewport/routeDraft.ts:109`; `buildNodeCreationSubmission` `:91`; `create_node` `projects/chirality-piping/core/model_operations/operation_applier/src/lib.rs:2145`; `connect_pipe_run` `:2147`; capabilities `build.node` `:25`, `build.pipe` `:38` (`projects/chirality-piping/apps/desktop/src/features/toolkit/capabilityCatalog.ts`) |
| 63 | New row's number (next multiple of the increment) | Interface numbering rule | **GAP (engine)** G-01 (numeric identity; today ids are free strings) |
| 64 | Insert between (⌥↩ on a middle row; midpoint number) | One batch: `create_node`, `delete_pipe_run`, two `connect_pipe_run` | composed from `:2145`, `:2153`, `:2147` (`projects/chirality-piping/core/model_operations/operation_applier/src/lib.rs`); the numbering rule as row 63 |
| 65 | Split element… (⌘⇧S; distance from From) | One `split_pipe_run` | `split_pipe_run` `projects/chirality-piping/core/model_operations/operation_applier/src/lib.rs:2140`; capability `build.split` `projects/chirality-piping/apps/desktop/src/features/toolkit/capabilityCatalog.ts:64` |
| 66 | Renumber nodes… (start, increment, preview old → new) | One renumber-all operation with reference rewrite | **GAP (engine)** G-01 (roadmap R1 `projects/chirality-piping/apps/desktop/src/features/toolkit/capabilityCatalog.ts:375`) |
| 67 | Delete rows (⌫; elements, nodes, attachments; downstream heal) | One batch: `delete_pipe_run`, `delete_node`, `delete_support`, `delete_primitive_load`, `delete_component`, then `connect_pipe_run` for the heal | composed from `:2153`, `:2146`, `:2164`, `:2160`, `:2152`, `:2147` (`projects/chirality-piping/core/model_operations/operation_applier/src/lib.rs`); capability `edit.remove` `projects/chirality-piping/apps/desktop/src/features/toolkit/capabilityCatalog.ts:292` |
| 68 | Start row (absolute coordinates; no element; no required marks) | `create_node` with coordinates | `create_node` `:2145`; `coordinates` `projects/chirality-piping/schemas/model.schema.yaml:1220`; required-mark placement rule interface |
| 69 | Branch row (From names an earlier node) | `connect_pipe_run` from that node | `connect_pipe_run` `projects/chirality-piping/core/model_operations/operation_applier/src/lib.rs:2147` |
| 70 | Required asterisks (Section, Material, Load on the first element's row) | Derived from the readiness check | `MissingDataBlockingPanel` `projects/chirality-piping/apps/desktop/src/features/missing-data/MissingDataBlockingPanel.tsx:31`; `blocks_mechanics_solve` `:104` (derived) |
| 71 | Cell rules on commit (type, range, enumeration, uniqueness, reference resolution; unit suffix; feet-inch forms) | Interface checks before the engine's | interface; engine rejection surfaced from `OperationOutcome.validation` `projects/chirality-piping/apps/desktop/src/types.ts:774` and `diagnostics` `:776` |
| 72 | Engine-rejected cell (reverts, shows the diagnostic) | Read the outcome of `validateModelOperation` | `validateModelOperation` `projects/chirality-piping/apps/desktop/src/services/operationService.ts:57`; `OperationOutcomeDiagnostic` via `:776` |
| 73 | Marks column slots (open joined row; plus → Add restraint / load / node data) | Open the expansion; create rows per C | interface open; creation as C rows 78, 86, 89 |
| 74 | Row expansion (⌘↩; Type fields; joined rows; combination editor) | Interface | **GAP (interface)** G-28 |
| 75 | Paste with mapping band (⌘V; header mapping; preview; source node IDs kept; ignored columns) | One batch of `create_node`, `connect_pipe_run`, `assign_section`, `set_field`, `create_support` … | **GAP (interface)** G-27: no mapping band; landing route `applyOperationBatch` `projects/chirality-piping/apps/desktop/src/services/operationBatchService.ts:89` |
| 76 | Copy (⌘C) as tab-separated text with header, display units, precision | Interface clipboard | **GAP (interface)** G-28 |
| 77 | Filters: has restraint / load / node data, branch rows, start rows, type | Derived | `PreviewModel.supports` `projects/chirality-piping/apps/desktop/src/types.ts:170`, `components` `:222` (derived) |
| 78 | Filters: propagated cells, unchecked rows, agent-origin rows, changed since… | Derived from origins, the Checked mark and the ledger | **GAP (typed interface)** G-07, G-08, G-09 |
| 79 | Footer: counts, filter chips, Read-through and Origins switches, ⌘F filter field | Interface | **GAP (interface)** G-28 (Origins depends on G-07; Read-through on G-03) |
| 80 | Export table… (CSV, JSON with units and provenance line) | Interface export | **GAP (interface)** G-28 |
| 81 | Empty state (start row 10 and row 20 editing; hint line) | Derived from a blank model | `buildBlankLocalModelDocument` `projects/chirality-piping/apps/desktop/src/services/projectService.ts:450`; pre-created rows interface |

## C. The attachment tables — restraints, loads, node data

| # | Control or gesture | Operation it invokes | Provider (file:line) or GAP |
|---|---|---|---|
| 82 | Restraints: add row (mark slot, table add-row, canvas S) | One `create_support` | `create_support` `projects/chirality-piping/core/model_operations/operation_applier/src/lib.rs:2156`; capability `supports.restraint` `projects/chirality-piping/apps/desktop/src/features/toolkit/capabilityCatalog.ts:73`; `Support` `projects/chirality-piping/schemas/model.schema.yaml:1872`, `support_type` `:1877` |
| 83 | Type: Anchor, Two-way, Guide, Limit stop | `support_type` anchor, restraint, guide, line_stop | `support_type` enum `projects/chirality-piping/schemas/model.schema.yaml:1877`; capability `supports.boundary` `projects/chirality-piping/apps/desktop/src/features/toolkit/capabilityCatalog.ts:118` |
| 84 | Type: +Y, −Y one-way with Gap and μ | `create_support` with nonlinear gap and friction | `PreviewModel.supports[].nonlinear` `projects/chirality-piping/apps/desktop/src/types.ts:178` (`gap` `:185`, `friction_coefficient` `:186`); capability `supports.nonlinear` `projects/chirality-piping/apps/desktop/src/features/toolkit/capabilityCatalog.ts:103` |
| 85 | Type: Variable spring, Constant support, User spring | `support_type` variable_spring_hanger, constant_effort_support, spring | `support_type` `projects/chirality-piping/schemas/model.schema.yaml:1877`; `SpringHanger` `:1808`; `supports[].hanger` `projects/chirality-piping/apps/desktop/src/types.ts:199` (`cold_load` `:206`, `hot_load` `:207`, `travel_range` `:209`); capability `supports.hanger` `projects/chirality-piping/apps/desktop/src/features/toolkit/capabilityCatalog.ts:88` |
| 86 | Type: Rod hanger (rigid Y restraint with a tag) | `support_type` restraint, direction Y | `support_type` `:1877` (by convention; export gap 11 in Q) |
| 87 | Type: skewed restraint along a direction vector; connecting node between two pipe nodes | None | **GAP (engine)** G-22: `Support.directions` `projects/chirality-piping/schemas/model.schema.yaml:1877` carries axes, no vector or second node |
| 88 | Edit a restraint field (direction, gap, μ, stiffness, library, max variation, tag, note) | One `update_support` | `update_support` `projects/chirality-piping/core/model_operations/operation_applier/src/lib.rs:2144`; Max variation **GAP (typed interface)** G-06 (no field on the hanger record `projects/chirality-piping/apps/desktop/src/types.ts:199`) |
| 89 | Delete a restraint | One `delete_support` | `delete_support` `projects/chirality-piping/core/model_operations/operation_applier/src/lib.rs:2164` |
| 90 | Tag ≤ 14 characters | Interface rule from the export limit (RESEARCH-E §6.2) | **GAP (interface)** G-28 |
| 91 | Design load, Travel columns (read-only from the run) | See G | **GAP (engine)** G-06 |
| 92 | Loads: add row (force, moment, displacement, rotation, uniform) | One `create_primitive_load` | `create_primitive_load` `projects/chirality-piping/core/model_operations/operation_applier/src/lib.rs:2159`; `LoadCase.load_type` `projects/chirality-piping/schemas/model.schema.yaml:522`; `ElementUniformDistributedForceLoadRecord` `:792`; capability `loads.primitive` `projects/chirality-piping/apps/desktop/src/features/toolkit/capabilityCatalog.ts:216` |
| 93 | Loads: Kind Concentrated weight | Node mass in every case that needs mass | **GAP (engine)** G-23; today only a force in the weight case (row 92) |
| 94 | Loads: Case column (must name a primitive case of matching kind) | Reference check | `LoadCase` `projects/chirality-piping/schemas/model.schema.yaml:516`; `loads` `:559` |
| 95 | Loads: edit, delete | `update_load`; `delete_primitive_load` | `update_load` `projects/chirality-piping/core/model_operations/operation_applier/src/lib.rs:2143`; `delete_primitive_load` `:2160` |
| 96 | Wind and Seismic tabs (case-level inputs; exposed spans) | `create_load_case` of kind wind or seismic with its inputs | `PreviewModel` seismic `projects/chirality-piping/apps/desktop/src/types.ts:76`, wind `:88`, equivalent static `:99`; capabilities `loads.wind` `:228`, `loads.wind-exposure` `:242`, `loads.seismic` `:257` (`projects/chirality-piping/apps/desktop/src/features/toolkit/capabilityCatalog.ts`) |
| 97 | Node data: Branch connection kind and fields (welding tee, sweepolet, weldolet, fabricated, extruded, radiused, thickened) | `insert_component_symbol` kind tee at the node; `set_field` on `branch_connection_type` and reinforcement | `creatableComponentKinds` "tee" `projects/chirality-piping/apps/desktop/src/features/component-creation/componentIntent.ts:3`; `geometry.branch_connection_type` `projects/chirality-piping/apps/desktop/src/types.ts:24`, `branch_reinforcement_area` `:25`; `branch_header_sif_user_value` `:48`, `branch_branch_sif_user_value` `:49` |
| 98 | Node data: SIF user values (in, out, axial, torsional) | `set_field` on modifiers | `modifiers.sif_user_value` `projects/chirality-piping/apps/desktop/src/types.ts:47` |
| 99 | Node data: Flange (type, weight, gasket and bolt-circle diameters) | `insert_component_symbol` kind flange | `creatableComponentKinds` "flange" `projects/chirality-piping/apps/desktop/src/features/component-creation/componentIntent.ts:3`; `weight` `projects/chirality-piping/apps/desktop/src/types.ts:32` |
| 100 | Node data: Concentrated weight with offset; Weld type and mismatch; Threaded joint | None | **GAP (engine)** G-23 |
| 101 | Joined-row expansion under the node row; two classes open at once | Interface | **GAP (interface)** G-28 |
| 102 | Warning "Branch connection on a node with no branch" | Derived | `Assumption` `projects/chirality-piping/schemas/model.schema.yaml:34` / `Diagnostic` `:371` (derived) |
| 103 | Add row asks for the node (combobox) | Interface | interface; node list `PreviewModel.nodes` `projects/chirality-piping/apps/desktop/src/types.ts:147` |

## D. Load sets

| # | Control or gesture | Operation it invokes | Provider (file:line) or GAP |
|---|---|---|---|
| 104 | Load sets table (Set, T1 P1 … Tn Pn, specific gravity, Used by, Note); add pair; add set | Create or edit a load-set record | **GAP (engine/typed interface)** G-03 |
| 105 | Delete a set (refused when used) | Delete with reference check | **GAP** G-03 |
| 106 | Fork from a layout row (§3.2) | One batch: create set, assign, re-propagate | **GAP** G-03 |

## E. Load cases and combinations, wind, seismic

| # | Control or gesture | Operation it invokes | Provider (file:line) or GAP |
|---|---|---|---|
| 107 | Add case (⌥↩, Add case) | One `create_load_case` | `create_load_case` `projects/chirality-piping/core/model_operations/operation_applier/src/lib.rs:2157`; capability `loads.cases` `projects/chirality-piping/apps/desktop/src/features/toolkit/capabilityCatalog.ts:204` |
| 108 | Delete case | One `delete_load_case` | `delete_load_case` `projects/chirality-piping/core/model_operations/operation_applier/src/lib.rs:2158` |
| 109 | Edit Case name | `set_field` | `set_field` `:2142`; `LoadCase.name` `projects/chirality-piping/schemas/model.schema.yaml:540` |
| 110 | Stress type column (Sustained, Operating, Expansion, Occasional, Hydrotest, Other) | `set_field` on a stress category | **GAP (typed interface)** G-05: no stress-category field found on `LoadCase` `:516` or `Combination` `:125` (`projects/chirality-piping/schemas/model.schema.yaml`); `load_type` `:522` names hydrotest and user_occasional kinds only |
| 111 | Rule column (a check ID from the project pack) | Bind a rule to the row | binding plan derives from the pack (`deriveRuleCheckBindingPlan` `projects/chirality-piping/apps/desktop/src/services/ruleCheckService.ts:238`); a per-row binding stored in the model **GAP (typed interface)** G-05 |
| 112 | Combination expansion: add or remove a term, factor | `create_combination_term`, `delete_combination_term` | `:2163`, `:2165` (`projects/chirality-piping/core/model_operations/operation_applier/src/lib.rs`); `LoadCaseManagerPanel` `projects/chirality-piping/apps/desktop/src/features/load-cases/LoadCaseManagerPanel.tsx:1704`; capability `loads.combinations` `projects/chirality-piping/apps/desktop/src/features/toolkit/capabilityCatalog.ts:271` |
| 113 | Combination expansion: basis Sum, Difference of two states, Range envelope | `create_combination` with basis | `create_combination` `:2161`, `delete_combination` `:2162`; `COMBINATION_BASIS_OPTIONS` (mechanics, result_state_subtraction, range_envelope) `projects/chirality-piping/apps/desktop/src/features/load-cases/LoadCaseManagerPanel.tsx:68`, create at `:1562`; `Combination.basis` `projects/chirality-piping/schemas/model.schema.yaml:132` |
| 114 | Expression column (one-way rendering of terms) | Derived | `Combination` terms `projects/chirality-piping/schemas/model.schema.yaml:125` (derived); rule expression rendering `renderExpressionText` `projects/chirality-piping/apps/desktop/src/features/rule-packs/ExpressionComposer.tsx:403` |
| 115 | Generate from rule pack… (band, preview, Generate ↩); Regenerate | One batch of `create_load_case` and `create_combination` with the Generated origin | **GAP (engine)** G-04: rule packs declare no generation; nearest generation route `generateSelfWeightPlan` `projects/chirality-piping/apps/desktop/src/services/selfWeightPlanService.ts:51` (host `generate_self_weight_operation_plan` `projects/chirality-piping/apps/desktop/src-tauri/src/lib.rs:1847`), `selfWeightPlanBatch` `projects/chirality-piping/apps/desktop/src/features/self-weight-authoring/SelfWeightPlanPanel.tsx:8`, capability `loads.self-weight` `projects/chirality-piping/apps/desktop/src/features/toolkit/capabilityCatalog.ts:283` |
| 116 | Origin column (Generated · pack; Edited by · was; Authored by) | Derived from the ledger | **GAP (typed interface)** G-07 |
| 117 | Rule expression column in the dashed frame (M-15) | Derived, display-only | `renderExpressionText` `projects/chirality-piping/apps/desktop/src/features/rule-packs/ExpressionComposer.tsx:403` |
| 118 | Filters: generated / edited / authored, with rule, stress type | Derived | depends on G-07, G-05 |
| 119 | Wind, Seismic tabs | As row 96 |  |

## F. The results tables and the results header

| # | Control or gesture | Operation it invokes | Provider (file:line) or GAP |
|---|---|---|---|
| 120 | Tab strip: Summary, Stresses, Displacements, Restraint loads, Restraint summary, Forces and moments, Hangers | Show a results table | `MechanicsResult` `projects/chirality-piping/apps/desktop/src/types.ts:287`; `ResultFamily` (displacement, rotation, force, moment, reaction, stress, section_property, ratio, rule_check) `projects/chirality-piping/schemas/results.v0.2.schema.yaml:912`; `ResultsPanel` `projects/chirality-piping/apps/desktop/src/features/results/ResultsPanel.tsx:38`; `buildResultInterpretation` `projects/chirality-piping/apps/desktop/src/features/results/resultInterpretation.ts:16` |
| 121 | Case selector | Filter every table to one case | derived over result rows and their case references (`ResultMetadata` `projects/chirality-piping/schemas/results.v0.2.schema.yaml:614`); filter pattern `projects/chirality-piping/apps/desktop/src/features/results/ResultsPanel.tsx:59` |
| 122 | Envelope switch (selector visible and disabled; governing case per row) | Derived: governing case per element from per-case rows | **GAP (interface derivation)** G-14; data question Q13 |
| 123 | Stresses: Stress, Allowable, Ratio (bar, 1.0 tick), Rule, Pack | Derived from check outcomes bound to element and case | `CheckOutcome` (computed_value, limit_value, acceptability_relation) `projects/chirality-piping/schemas/rule_check_run_result.schema.json:98`; `runRuleChecks` `projects/chirality-piping/apps/desktop/src/services/ruleCheckService.ts:101`; `RulePackRef` `projects/chirality-piping/schemas/results.v0.2.schema.yaml:967`; per-element per-case binding **GAP (typed interface)** G-15 |
| 124 | Stresses expansion (stress components) | Derived | `ResultFamily` stress rows `projects/chirality-piping/schemas/results.v0.2.schema.yaml:912` (as the run reports them) |
| 125 | Displacements, Restraint loads, Forces and moments tables | Derived | `ResultFamily` displacement, rotation, reaction, force, moment `:912` (`projects/chirality-piping/schemas/results.v0.2.schema.yaml`) |
| 126 | Restraint loads: one-way support state (active, lifted off) | Derived from run diagnostics only | `Diagnostic` `projects/chirality-piping/apps/desktop/src/types.ts:255`; `AnalysisRunEnvelope.analysis_run.diagnostics` `:445` (derived; never inferred) |
| 127 | Restraint summary (max and min per component over cases) | Derived | `ResultFamily` reaction `:912` (derived) G-14 |
| 128 | Forces and moments: Axes Global / Local | Derived when both frames are reported | **GAP (typed interface)** G-15: frames per force row not established |
| 129 | Summary: hero ratio, per stress-type table, maxima, counts, chips, diagnostics count | Derived | `CheckOutcome` `:98`, `MechanicsResult` `:287`, statuses per L (derived) G-14 |
| 130 | Sort, filter, data bar, colour by scale | Table view state | `projects/chirality-piping/apps/desktop/src/features/results/ResultsPanel.tsx:59` (filter); the rest **GAP (interface)** G-28 |
| 131 | Selection sync (results row ↔ entity) | Select the entity | `resolveEntitySelection` `projects/chirality-piping/apps/desktop/src/features/results/resultInterpretation.ts:124` |
| 132 | Copy; Export table… (CSV, JSON with the run's binding set) | Export | `buildCurrentResultExport` `projects/chirality-piping/apps/desktop/src/features/result-export/resultExportAdapter.ts:138`; `saveNativeResultJson` `projects/chirality-piping/apps/desktop/src/features/result-export/nativeResultSave.ts:96` (host `save_local_result_json` `projects/chirality-piping/apps/desktop/src-tauri/src/lib.rs:4197`); CSV **GAP (interface)** G-28 |
| 133 | Results header: run name and number, time, "immutable" | Derived from the run record | `analysis_run.run_id` `:436`, `run_name` `:437`, `created_at` `:439`, `immutability_policy` `:488` (`projects/chirality-piping/apps/desktop/src/types.ts`) |
| 134 | Results header: run menu (every run of the project) | List the project's runs | **GAP (typed interface)** G-10: one `analysis_run` slot `projects/chirality-piping/apps/desktop/src/types.ts:971` |
| 135 | Results header: evidence label chip (M-09) | Show the run's label | **GAP (engine/typed interface)** G-13: no emission found in the desktop source; vocabulary `projects/chirality-piping/docs/claims_registry.md:87` |
| 136 | Results header: historical band (M-13) | Derived | `buildHistoricalRunContext` `projects/chirality-piping/apps/desktop/src/features/results/HistoricalRunContext.tsx:263`; `HistoricalRunPanel` `:318` |
| 137 | Results header: stale band with "Run again"; hatched values; unsolved elements | Keep the run after a model change | **GAP (engine/typed interface)** G-11 |
| 138 | Results header: information disclosure (M-02 sentence; run identity line) | Derived | wording `projects/chirality-piping/docs/claims_registry.md:31`; identity from `solver_version` `:441`, `model_state_ref` `:440`, `rule_pack_refs` `:446`, `settings_ref` `:442` (`projects/chirality-piping/apps/desktop/src/types.ts`); `buildAnalysisRunV02` `projects/chirality-piping/apps/desktop/src/services/analysisRunCompatibility.ts:39` |
| 139 | Empty state after a failed run (stopped case, cause, remedies) | Derived from the job's diagnostics | `SolveJobAuditState.error_message` `projects/chirality-piping/apps/desktop/src/types.ts:523`; `events` `:522`; the stopped case **GAP (engine)** G-12 |
| 140 | "No rule pack" header line | Derived | `appliedRuleCheckStatus` `projects/chirality-piping/apps/desktop/src/services/previewService.ts:136` (derived) |

## G. The hanger table

| # | Control or gesture | Operation it invokes | Provider (file:line) or GAP |
|---|---|---|---|
| 141 | Rows per hanger location; Design load; Travel; State | The run's design pass | **GAP (engine)** G-06: no design pass; locations exist (`supports[].hanger` `projects/chirality-piping/apps/desktop/src/types.ts:199`) |
| 142 | ⌘↩ candidate sizes from the library (hot-load range, variation limit; dimmed with reason) | Derived over the library's records | `HangerRecord` `projects/chirality-piping/schemas/hanger.schema.yaml:303`; `Hanger` `:253` (`cold_load` `:287`, `hot_load` `:290`, `travel_range` `:296`); `HangerDocument` `projects/chirality-piping/apps/desktop/src/features/hanger-selection/hangerSelection.ts:18` (derived); variation limit depends on G-06 |
| 143 | Choose a size (writes Size, Rate, Cold load, Hot load onto the restraint row) | One batch: `update_support` with the record's values and provenance | `buildHangerSelectionBatch` `projects/chirality-piping/apps/desktop/src/features/hanger-selection/hangerSelection.ts:25`; `HangerSelectionPanel` `projects/chirality-piping/apps/desktop/src/features/hanger-selection/HangerSelectionPanel.tsx:8`; `update_support` `projects/chirality-piping/core/model_operations/operation_applier/src/lib.rs:2144`; capability `properties.hanger-library` `projects/chirality-piping/apps/desktop/src/features/toolkit/capabilityCatalog.ts:195` |
| 144 | Library provenance line above the table | Derived | `LibraryMetadata` `projects/chirality-piping/schemas/hanger.schema.yaml:100`; `Provenance` `:64` |
| 145 | "Name a library on the restraint row"; "Library not found" | Derived | `classifyLibraryReference` `projects/chirality-piping/apps/desktop/src/services/libraryImportService.ts:384` |
| 146 | Stale after model change | As row 137 | G-11 |

## H. The canvas

| # | Control or gesture | Operation it invokes | Provider (file:line) or GAP |
|---|---|---|---|
| 147 | Draw geometry, glyphs, load vectors, grid, labels | Render | `PipeViewport` state `viewPreset` `projects/chirality-piping/apps/desktop/src/features/viewport/PipeViewport.tsx:425`, `showLabels` `:426`, `showLoads` `:427`, `showGrid` `:428`; `setAuxiliaryVisibility` `:1275` |
| 148 | Click, ⇧-click, ⌘-click, hover | Select entities | `ViewportTool` `projects/chirality-piping/schemas/viewport_editor.schema.yaml:360`; capability `view.select` `projects/chirality-piping/apps/desktop/src/features/toolkit/capabilityCatalog.ts:333`; multi-select ordering **GAP (interface)** G-28 |
| 149 | Box selection with a kind filter | Select | `ViewportExposureInteraction` "box-selection" `projects/chirality-piping/apps/desktop/src/features/viewport/PipeViewport.tsx:178`; the kind filter **GAP (rendering)** G-16 |
| 150 | Follow selection (camera pans to keep the selection in view) | Camera | **GAP (rendering)** G-16 |
| 151 | Double-click → inspector | Toggle inspector | `view.inspector` `projects/chirality-piping/apps/desktop/src/App.tsx:3372` |
| 152 | Probe (P): card, pin, unpin; per-stage readings | Read the row under the cursor | **GAP (rendering)** G-16: no probe in the viewport |
| 153 | Route (R): compass, axis cycle ⇥, − reverse, length, B bend, ↩ commit, ⎋ cancel; draft ghost and draft row | One batch: `create_node` + `connect_pipe_run` (+ `insert_component_symbol` bend) | `RouteDraft` `projects/chirality-piping/apps/desktop/src/features/viewport/routeDraft.ts:11`; `buildRouteSubmission` `:109`; `validateRouteDraft` `:152`; bend `insert_component_symbol` `projects/chirality-piping/core/model_operations/operation_applier/src/lib.rs:2166` composed; capability `build.pipe` `projects/chirality-piping/apps/desktop/src/features/toolkit/capabilityCatalog.ts:38`; menu `insert.pipe` `projects/chirality-piping/apps/desktop/src/App.tsx:3394` |
| 154 | Add restraint (S): ghost glyph on hover, click opens the row, ↩ commits | One `create_support` | `create_support` `:2156`; menu `insert.support` `projects/chirality-piping/apps/desktop/src/App.tsx:3395`; `ViewportTool` `projects/chirality-piping/schemas/viewport_editor.schema.yaml:360` |
| 155 | Labels (L): All, Budget, Off | Render | `setShowLabels` `projects/chirality-piping/apps/desktop/src/features/viewport/PipeViewport.tsx:2181` (on/off); Budget mode **GAP (rendering)** G-16 |
| 156 | Colour by (Ratio, Stress, Displacement, Off); result colour on the tube; legend | Render | **GAP (rendering)** G-16: no result colouring or legend in the viewport |
| 157 | Deformation (D) | Render the deformed shape | `buildDeformationOverlay` `projects/chirality-piping/apps/desktop/src/features/viewport/PipeViewport.tsx:4400`; `deformation` `:578` |
| 158 | Deformation factor stepper, field, play | Render | **GAP (rendering)** G-16 |
| 159 | Routing draft ghost | Render the draft | `RouteDraft` `projects/chirality-piping/apps/desktop/src/features/viewport/routeDraft.ts:11`; `ViewStyle` ghosted `projects/chirality-piping/schemas/viewport_editor.schema.yaml:522` |
| 160 | Proposal ghost (violet, beside the current glyph) | Render a pending proposal | **GAP (rendering)** G-16: no viewport rendering of a pending proposal found |
| 161 | Camera: Fit (F), Iso, Top, Front | Camera | `fitViewportCamera` `projects/chirality-piping/apps/desktop/src/features/viewport/PipeViewport.tsx:3022`; `ViewPreset` (iso, front, top) `:180`; `fit-model` `:2154`; `CameraState` `projects/chirality-piping/schemas/viewport_editor.schema.yaml:279` |
| 162 | Camera: Right; Report figure preset; "Set report figure from this view" | Camera; saved with the project | Right **GAP (rendering)** G-16; saved preset **GAP (typed interface)** G-17 |
| 163 | Section plane | Render | **GAP (rendering)** G-16 |
| 164 | Isolate (I), Hide (H), Show all | Render | `isolate-selection` `:2141`, `hide-selection` `:2135`, `show-all` `:2147` (`projects/chirality-piping/apps/desktop/src/features/viewport/PipeViewport.tsx`) |
| 165 | Persist camera, label mode, hidden sets, report figure with the project | Interface state saved | **GAP (typed interface)** G-17 |
| 166 | Unsolved elements after a model change (neutral at 60 %) | Derived | depends on G-11 |
| 167 | Required-mark halo on entities with a blocking issue | Derived | `resolveDiagnosticEntitySelection` `projects/chirality-piping/apps/desktop/src/features/results/resultInterpretation.ts:133` (derived) |

## I. The inspector

| # | Control or gesture | Operation it invokes | Provider (file:line) or GAP |
|---|---|---|---|
| 168 | Toggle (⌘I, double-click, ⎋) | Interface | `view.inspector` `projects/chirality-piping/apps/desktop/src/App.tsx:3372` |
| 169 | Docked in Both (canvas shrinks; table never reflows) | Interface layout | **GAP (interface)** G-28 (direction record §11 decision 1) |
| 170 | Edit a field (any selection kind) | The same change kind the cell would make | rows 44–59, 88, 95, 98, 109–113; inspector route `inspectorView: "properties"` `projects/chirality-piping/apps/desktop/src/features/toolkit/capabilityCatalog.ts:301` |
| 171 | Multi-row shared-field edit ("5 rows") | One batch of `set_field` / `update_support` | composed through `applyOperationBatch` `projects/chirality-piping/apps/desktop/src/services/operationBatchService.ts:89` |
| 172 | Provenance disclosure (seven fields) | Derived from the referenced library records | `Provenance` `projects/chirality-piping/schemas/hanger.schema.yaml:64`; `classifyLibraryReference` `projects/chirality-piping/apps/desktop/src/services/libraryImportService.ts:384`; fields `projects/chirality-piping/docs/IP_AND_DATA_BOUNDARY.md:58` |
| 173 | Issues on this entity (Show) | Derived | `resolveDiagnosticEntitySelection` `projects/chirality-piping/apps/desktop/src/features/results/resultInterpretation.ts:133` |
| 174 | Origin and Checked section; Check control | See M rows 213–216 | G-07, G-08 |
| 175 | "Routing from node 40" block | Derived from the draft | `RouteDraft` `projects/chirality-piping/apps/desktop/src/features/viewport/routeDraft.ts:11` |
| 176 | Results-row inspector (read-only values, run identity, evidence label, Show in table) | Derived | rows 120, 133, 135 |

## J. Issues

| # | Control or gesture | Operation it invokes | Provider (file:line) or GAP |
|---|---|---|---|
| 177 | Open and close the drawer (⌘⇧I, status bar, rail, toolbar, Run reason link) | Interface | `view.issues` `projects/chirality-piping/apps/desktop/src/App.tsx:3383`; `IssuesHome` `:3703` |
| 178 | Classes in words (Invalid model, Blocks solve, Blocks rule check, Provenance, Assumption, Nonlinear, Content boundary, Note) | Derived from the diagnostic class and the readiness flags | `Diagnostic.class` `projects/chirality-piping/schemas/model.schema.yaml:376`; `Diagnostic` `projects/chirality-piping/apps/desktop/src/types.ts:255`; `blocks_mechanics_solve` and rule-blocking `projects/chirality-piping/apps/desktop/src/features/missing-data/MissingDataBlockingPanel.tsx:104`; `Assumption` `projects/chirality-piping/schemas/model.schema.yaml:34`; content boundary findings `validateLibraryImport` `projects/chirality-piping/apps/desktop/src/services/libraryImportService.ts:69`; the eight-word mapping is the interface's |
| 179 | Issue row → select its entity (switch stage if needed) | Select | `onSelectDiagnostic` `projects/chirality-piping/apps/desktop/src/App.tsx:3710`; `resolveDiagnosticEntitySelection` `projects/chirality-piping/apps/desktop/src/features/results/resultInterpretation.ts:133` |
| 180 | Group by class with counts; class chips as filters | Derived | `IssuesHome` `projects/chirality-piping/apps/desktop/src/App.tsx:3703` (gathers model, result and operation diagnostics) |
| 181 | Counts on the rail, toolbar, status bar, table footers; gutter state slot; cell corners; canvas halo | Derived | as row 180; per-row placement derived from the diagnostic's entity reference (`Diagnostic` `projects/chirality-piping/apps/desktop/src/types.ts:255`) |
| 182 | Filters: class, stage, table | Derived | `IssuesHome` `projects/chirality-piping/apps/desktop/src/App.tsx:3703` (derived) |
| 183 | Filters: unchecked rows only; since last run | Derived | **GAP (typed interface)** G-08, G-09 |
| 184 | Failure banner once on the Results page; drawer row carries "Show node 20" | Derived from the job's diagnostics | `SolveJobAuditState` `projects/chirality-piping/apps/desktop/src/types.ts:511`; `error_message` `:523`; placement interface (direction record §11 decision 6) |
| 185 | Operation diagnostics from a rejected or warned cell | Derived | `OperationOutcome.diagnostics` `projects/chirality-piping/apps/desktop/src/types.ts:776`; `IssuesHome` `operationOutcomes` `projects/chirality-piping/apps/desktop/src/App.tsx:3707` |

## K. The run

| # | Control or gesture | Operation it invokes | Provider (file:line) or GAP |
|---|---|---|---|
| 186 | Run (button, palette, menu) | Start a solve job | `startPreviewMechanicsJob` `projects/chirality-piping/apps/desktop/src/services/previewService.ts:87` (host `start_preview_mechanics_job` `projects/chirality-piping/apps/desktop/src-tauri/src/lib.rs:1672`, with solver mode `:1691`); `projects/chirality-piping/apps/desktop/src/App.tsx:988`; menu `analyze.run` `:3405` |
| 187 | Disabled reasons: blocking issues (up to three, "and n more"), "No load cases", "A run is in progress", "Engine unavailable" | Derived | `MissingDataBlockingPanel` `projects/chirality-piping/apps/desktop/src/features/missing-data/MissingDataBlockingPanel.tsx:31`; `blocks_mechanics_solve` `:104`; running flag `projects/chirality-piping/apps/desktop/src/App.tsx:3405`; `BuildReadinessPanel` `projects/chirality-piping/apps/desktop/src/features/build-readiness/BuildReadinessPanel.tsx:15` |
| 188 | Progress bar by stages (assemble, each case, hangers, rules); no synthesised percentage | Poll the job | `pollPreviewMechanicsJob` `projects/chirality-piping/apps/desktop/src/services/previewService.ts:100` (host `:1707`); `SolveJobEvent` `projects/chirality-piping/apps/desktop/src/types.ts:501`; `percentages_synthesized: false` `:515`; per-case granularity as the job's events report it (**GAP (engine)** G-12 if they do not) |
| 189 | Stop | Cancel the job | `cancelPreviewMechanicsJob` `projects/chirality-piping/apps/desktop/src/services/previewService.ts:106` (host `:1715`); `projects/chirality-piping/apps/desktop/src/App.tsx:925`; menu `analyze.cancel` `:3406` |
| 190 | Failure (banner sentence from the engine; "Show node 20") | Derived | `SolveJobAuditState.error_message` `projects/chirality-piping/apps/desktop/src/types.ts:523`; diagnostics `:445` |
| 191 | Run log popover (stages, times, stop reason; Run settings…, Run record, Run again) | Derived from the job events | `SolveJobAuditState.events` `projects/chirality-piping/apps/desktop/src/types.ts:522`; the full log persisted in the run record **GAP (typed interface)** G-12 |
| 192 | Run settings…: solver mode | Set the mode for the next run | `PreviewSolverMode` `projects/chirality-piping/apps/desktop/src/services/previewService.ts:56`; `run_preview_mechanics_with_solver_mode` `projects/chirality-piping/apps/desktop/src-tauri/src/lib.rs:1451` |
| 193 | Run settings…: iteration limit, cases to run, operating case for hanger travel, hanger pass on/off, settings identity | Set and cite settings | `settings_ref` `projects/chirality-piping/apps/desktop/src/types.ts:442` exists as a reference; the settings themselves **GAP (engine)** G-12 |
| 194 | Rule checks after the solve | Run rule checks | `runRuleChecks` `projects/chirality-piping/apps/desktop/src/services/ruleCheckService.ts:101` (host `run_rule_checks` `projects/chirality-piping/apps/desktop/src-tauri/src/lib.rs:2611`); `RuleCheckStatus` `:23`; `appliedRuleCheckStatus` `projects/chirality-piping/apps/desktop/src/services/previewService.ts:136`; menu `analyze.rule-checks` `projects/chirality-piping/apps/desktop/src/App.tsx:3408` |
| 195 | Run record page (identity, hash, solver, settings, cases, pack, libraries, label, diagnostics, log, immutability, record hash, Export) | Read the run record | `AnalysisRunEnvelope` `projects/chirality-piping/apps/desktop/src/types.ts:428`; `buildAnalysisRunV02` `projects/chirality-piping/apps/desktop/src/services/analysisRunCompatibility.ts:39`; `verifyAnalysisRunRecord` `:86`; `projects/chirality-piping/schemas/analysis_run.v0.2.schema.json:14`; `immutability_policy` `projects/chirality-piping/apps/desktop/src/types.ts:488`; input manifest `buildCurrentSessionInputManifest` `projects/chirality-piping/apps/desktop/src/services/inputManifestService.ts:74`; model hash `projects/chirality-piping/apps/desktop/src/services/hashService.ts:117` |
| 196 | Runs numbered per project; named; every run kept; select an earlier run as historical | Keep a run list | `run_name` `projects/chirality-piping/apps/desktop/src/types.ts:437` exists; list and index **GAP (typed interface)** G-10 |
| 197 | Historical run on reopen | Build the historical context | `buildHistoricalRunContext` `projects/chirality-piping/apps/desktop/src/features/results/HistoricalRunContext.tsx:263`; `projects/chirality-piping/apps/desktop/src/App.tsx:1930` |
| 198 | Export from the run record (results JSON, report package) | Export | rows 132, 235 |
| 199 | Model change after a run → run stops being the basis | Clear the current basis, keep the record | `clearComputedModelState` `projects/chirality-piping/apps/desktop/src/App.tsx:1695` clears; keeping the record as stale **GAP** G-11 |

## L. The status chips

| # | Control or gesture | Operation it invokes | Provider (file:line) or GAP |
|---|---|---|---|
| 200 | Model incomplete (Solver) before a run; no chip when complete but unsolved | Derived from the readiness check | `MissingDataBlockingPanel` `projects/chirality-piping/apps/desktop/src/features/missing-data/MissingDataBlockingPanel.tsx:31`; `AutomaticAnalysisStatus` `projects/chirality-piping/schemas/analysis_status.schema.yaml:107`; `projects/chirality-piping/docs/TYPES.md:52`; `StatusPill` `projects/chirality-piping/apps/desktop/src/App.tsx:3755` |
| 201 | Mechanics solved (Solver) after a run | Derived from the run record | `analysis_status` `projects/chirality-piping/schemas/analysis_run.v0.2.schema.json:14`; `SolveJobEvent.analysis_status` `projects/chirality-piping/apps/desktop/src/types.ts:508` |
| 202 | Rule inputs incomplete, User rules checked, User rule failed (Rule pack) | Derived | `RuleCheckStatus` `projects/chirality-piping/apps/desktop/src/services/ruleCheckService.ts:23`; `appliedRuleCheckStatus` `projects/chirality-piping/apps/desktop/src/services/previewService.ts:136` |
| 203 | Human review required (Human), Review page only | Derived | `AutomaticAnalysisStatus` `projects/chirality-piping/schemas/analysis_status.schema.yaml:107`; `Actor` `:67`; placement interface (direction record §11 decision 3) |
| 204 | Popover: raw token, Authority, run and time, Reason, historical caption | Derived | `SoftwareStatusRecord` (basis) `projects/chirality-piping/schemas/analysis_status.schema.yaml:325`; `Actor` `:67`; `projects/chirality-piping/docs/TYPES.md:52` |
| 205 | Chips drop after a model change | Derived | `clearComputedModelState` `projects/chirality-piping/apps/desktop/src/App.tsx:1695` |
| 206 | Chips after a failed run (as the record emits; none when none) | Derived | `SolveJobEvent.analysis_status` `projects/chirality-piping/apps/desktop/src/types.ts:508`; what a stopped run emits **GAP (engine)** G-12 |
| 207 | Evidence labels (M-09) | As row 135 | G-13 |

## M. The agent panel and protocol

| # | Control or gesture | Operation it invokes | Provider (file:line) or GAP |
|---|---|---|---|
| 208 | Column / strip; tabs Conversation, Proposals, Checks, Accepted; working state; Stop | Interface | panel `projects/chirality-piping/apps/desktop/src/features/agent-proposals/AgentProposalPanel.tsx:4`; layout **GAP (interface)** G-28; live working state **GAP** G-19 |
| 209 | Conversation (⌘↩ send; agent messages with links, TBD items, evidence summaries) | Talk to the agent | **GAP (host)** G-19: no live binding; today a proposal arrives as a document (`OfflineProposalIntakePanel` textarea `projects/chirality-piping/apps/desktop/src/features/offline-proposal-intake/OfflineProposalIntakePanel.tsx:34`; `loadSampleProposal` `projects/chirality-piping/apps/desktop/src/services/previewService.ts:473`; host `sample_agent_proposal` `projects/chirality-piping/apps/desktop/src-tauri/src/lib.rs:1815`) |
| 210 | Proposal lands (card; banded rows; tab counts; strip badge; footer line) | Present a proposal | `AgentProposal` `projects/chirality-piping/apps/desktop/src/types.ts:600` (`changes` before/after `:611`, `rationale` `:618`, `assumptions` `:619`); capabilities `review.pending` `:354`, `review.agent` `:366` (`projects/chirality-piping/apps/desktop/src/features/toolkit/capabilityCatalog.ts`); rows and ghosts in the tables and canvas **GAP (interface, rendering)** G-18, G-16 |
| 211 | Accept row (⌘⇧A; card row check) | Apply the row's changes as one batch with `author_type` "agent" | `applyOperationBatch` `projects/chirality-piping/apps/desktop/src/services/operationBatchService.ts:89`; `author_type` `projects/chirality-piping/apps/desktop/src/types.ts:650`; `requires_user_acceptance` `:622`, `acceptance_recorded_as_review_only` `:624`; `OperationAcceptanceRecord` on the outcome `:781` and receipt `:797`; ledger actor `projects/chirality-piping/apps/desktop/src/features/operations/OperationLedgerPanel.tsx:270`; per-row decision state **GAP (typed interface)** G-18 |
| 212 | Accept selected (n rows); Accept remaining; Accept all rows; Accept all (n rows) across proposals | One batch per accepted unit | composed through `:89` (`projects/chirality-piping/apps/desktop/src/services/operationBatchService.ts`); one checkpoint per batch `projects/chirality-piping/apps/desktop/src/App.tsx:1336` |
| 213 | Reject row / selected / remaining / proposal | Record a rejection; nothing applied | **GAP (typed interface)** G-18: no rejection record |
| 214 | Stale-proposed row (target changed since landing) | Compare the proposal's model basis with the current hash | `OperationModelBasisEvidence` `projects/chirality-piping/apps/desktop/src/types.ts:777`; `claimed_model_hash` `projects/chirality-piping/apps/desktop/src-tauri/src/lib.rs:1852` (detection); per-row stale state and "superseded by" **GAP (typed interface)** G-18 |
| 215 | Blocked row (engine rejected) | Validate before landing | `validateModelOperation` `projects/chirality-piping/apps/desktop/src/services/operationService.ts:57`; `OperationOutcome.validation` `projects/chirality-piping/apps/desktop/src/types.ts:774` |
| 216 | Withdrawn by the agent | Withdraw | **GAP (host)** G-19 |
| 217 | Accepted record (proposal, row, rationale, constraints considered, TBD, engineer, time; Undo while reversible; survives reopen) | Persist the acceptance | `OperationAcceptanceRecord` `projects/chirality-piping/apps/desktop/src/types.ts:781`; `LocalProjectEnvelope.editor_intents` `:967`, `proposal` `:968` (one); constraints considered and per-row TBD fields, and receipts persisted with the project **GAP (typed interface)** G-18 |
| 218 | Consequence line on the card | Derived string | depends on G-11 |
| 219 | What a proposal may not do (run, export, snapshot, Checked, libraries, rules, preferences) | Enforced by the route: proposals reach only the operation layer | `applyModelOperation` `projects/chirality-piping/apps/desktop/src/services/operationService.ts:72` (by construction) |
| 220 | Checks tab and Review-page comment stream: Check, Open issue cards by reference; Open / Resolved / Reopen | Agent feedback records | **GAP (host, typed interface)** G-19, G-21 |
| 221 | Check (⌘⇧K): set, clear, Check again; multi-row; stale on content change; counts | A human row tag bound to the row's content hash | **GAP (typed interface)** G-08: nearest `StateTag` `projects/chirality-piping/schemas/model_state.schema.json:659` (model-state level, not row level) |
| 222 | Filters Unchecked rows, Stale checks | Derived | G-08 |
| 223 | Origins per row and cell (Entered, Accepted, Propagated, Generated, Imported); Origins switch; ⌥I list | Derived from a persisted ledger with author classes | session ledger `AppliedOperationReceipt` `projects/chirality-piping/apps/desktop/src/types.ts:786`; `author_type` `:650`; `OperationAuthorType` (user, agent, import_adapter, project_template) `projects/chirality-piping/schemas/model_operation.schema.json:306`; `OperationLedgerPanel` `projects/chirality-piping/apps/desktop/src/features/operations/OperationLedgerPanel.tsx:211`; persistence and Propagated/Generated classes **GAP (typed interface)** G-07 |
| 224 | Changes since… (last run, last Checked, a snapshot, a time) | Read the ledger | `AppliedOperationReceipt.sequence` `:788`, `applied_model_hash` `:796` (`projects/chirality-piping/apps/desktop/src/types.ts`) in session; persisted with time **GAP (typed interface)** G-09 |

## N. The Review page

| # | Control or gesture | Operation it invokes | Provider (file:line) or GAP |
|---|---|---|---|
| 225 | Outline in the required order with fixed sections | Derived from the renderer's sections | `projects/chirality-piping/core/reporting/report_renderer/src/lib.rs:457` (Model Input Summary), `:475` (Load Cases), `:512` (Results), `:576` (Warnings, Assumptions, And Provenance), `:617` (Audit Manifest), `:645` (Rule Pack References), `:696` (Limitations), `:705` (signoff rows), `:716` (Professional Boundary And Human Review); `ReportSectionEnvelope` `projects/chirality-piping/schemas/report_sections.schema.yaml:476`, `report_section_id` `:480`; `projects/chirality-piping/docs/PRD.md:1213` |
| 226 | Live table blocks (caption, own case selector and sort, row filter; Insert live table…) | Select blocks for the report input | report input built from model and run `buildRenderableReportInput` `projects/chirality-piping/apps/desktop/src/features/report/renderableReportInput.ts:276`; block selection and filters persisted **GAP (typed interface)** G-20 |
| 227 | Text blocks (paragraph, list, reference, Insert value token, Insert the report figure) | Authored text in the report | **GAP (typed interface)** G-20: no authored text in the report input |
| 228 | Reorder editable sections (⌘↑ ⌘↓, drag) | Section order | **GAP (typed interface)** G-20 |
| 229 | Show edits switch (inserted, removed since the compared iteration) | Diff of text | **GAP (interface)** G-20 |
| 230 | Snapshot… (named iteration; immutable; bound to run and hash); iteration combobox; Compare with…; Restore as current | Take and read snapshots | nearest carrier `ModelStateRecord` `projects/chirality-piping/schemas/model_state.schema.json:293`, `state_kind` design_snapshot `:325`; no app operation **GAP (typed interface)** G-20 |
| 231 | Comment stream (+ Comment; Resolve, Reopen; Draft Insert / Discard; filters) | Notes by reference | nearest `StateNote` `projects/chirality-piping/schemas/model_state.schema.json:616`; **GAP (typed interface)** G-21 |
| 232 | Report figure block; "Set from the canvas" | Render at the saved preset | G-16, G-17 |
| 233 | Report preview (outline as printed; notice; signoff block; theme; Print; Save PDF, HTML) | Render the report | `renderCalculationReport` `projects/chirality-piping/apps/desktop/src/services/reportRenderService.ts:47` (host `render_calculation_report` `projects/chirality-piping/apps/desktop/src-tauri/src/lib.rs:1386`); Print and PDF **GAP (interface)** G-28 |
| 234 | Review/signoff block fields (name, role, date, notes) | Values into the signoff rows | renderer rows `projects/chirality-piping/core/reporting/report_renderer/src/lib.rs:705`; entering values **GAP (typed interface)** G-20 |
| 235 | Export… (report package) | Save the package | `buildReportPackageRequest` `projects/chirality-piping/apps/desktop/src/features/report/reportPackageRequest.ts:77` (profile `:292`); `saveReportPackage` `projects/chirality-piping/apps/desktop/src/services/reportPackageSaveService.ts:74` (host `save_report_package` `projects/chirality-piping/apps/desktop/src-tauri/src/lib.rs:171`); menu `file.save-report-package` `projects/chirality-piping/apps/desktop/src/App.tsx:3356` |
| 236 | Acceptance sentence at the page top (M-02) | Wording | `projects/chirality-piping/docs/claims_registry.md:31` |
| 237 | Notice section (M-03; M-04 when applicable) | Wording | `projects/chirality-piping/docs/report_notice_template.md:35`; `projects/chirality-piping/docs/PRD.md:1244` |
| 238 | Header: "Review · project · Run 04"; iteration line | Derived | rows 133, 230 |

## O. Libraries

| # | Control or gesture | Operation it invokes | Provider (file:line) or GAP |
|---|---|---|---|
| 239 | Libraries page list (Name, Kind, Records, Source, Redistribution status, Review status, Imported, Used by) | List | `listLocalLibraries` `projects/chirality-piping/apps/desktop/src/services/libraryImportService.ts:200` (host `list_local_libraries` `projects/chirality-piping/apps/desktop/src-tauri/src/lib.rs:3661`); `LibraryKind` `:18`; Used by derived from references `classifyLibraryReference` `:384`; capability `properties.libraries` `projects/chirality-piping/apps/desktop/src/features/toolkit/capabilityCatalog.ts:183` |
| 240 | Import… (file, kind, seven provenance fields, findings of the six flag classes, Import) | Validate then save | `validateLibraryImport` `projects/chirality-piping/apps/desktop/src/services/libraryImportService.ts:69` (host `validate_library_import` `projects/chirality-piping/apps/desktop/src-tauri/src/lib.rs:3304`); `saveLocalLibrary` `:172` (host `save_local_library` `:3571`); `Provenance` `projects/chirality-piping/schemas/hanger.schema.yaml:64`; `LibraryMetadata` `:100`; fields `projects/chirality-piping/docs/IP_AND_DATA_BOUNDARY.md:58` |
| 241 | Quarantine (stops the import; `review_status` quarantined; `redistribution_status` protected_suspected; issue; review) | Set the record's status | `review_status` and `redistribution_status` enumerations `projects/chirality-piping/docs/IP_AND_DATA_BOUNDARY.md:58` (quarantine rule at §5); `Provenance` `projects/chirality-piping/schemas/hanger.schema.yaml:64` |
| 242 | Open (record table with provenance) | Open | `openLocalLibrary` `projects/chirality-piping/apps/desktop/src/services/libraryImportService.ts:186` (host `:3628`) |
| 243 | Delete (refused when referenced) | Delete with reference check | `deleteLocalLibrary` `projects/chirality-piping/apps/desktop/src/services/libraryImportService.ts:208` (host `:3671`); references `classifyLibraryReference` `:384` |
| 244 | Locate… / Remove reference for a missing library | Re-point or drop a reference | `classifyLibraryReference` `projects/chirality-piping/apps/desktop/src/services/libraryImportService.ts:384` (detection); re-pointing **GAP (interface)** G-28 |
| 245 | M-05 sentence on the import sheet | Wording | `projects/chirality-piping/docs/claims_registry.md:17` |

## P. Rules

| # | Control or gesture | Operation it invokes | Provider (file:line) or GAP |
|---|---|---|---|
| 246 | Rules page list; Import…; Open; Delete | List, validate, save, open, delete a pack | `listLocalRulePacks` `projects/chirality-piping/apps/desktop/src/services/rulePackService.ts:184` (host `:3220`); `validateRulePack` `:138` (host `validate_rule_pack` `projects/chirality-piping/apps/desktop/src-tauri/src/lib.rs:2590`); `saveLocalRulePack` `:160` (host `:3165`); `openLocalRulePack` `:172` (host `:3195`); `deleteLocalRulePack` `:192` (host `:3230`) |
| 247 | New draft | Create a draft document | `buildDraftRulePackDocument` `projects/chirality-piping/apps/desktop/src/services/rulePackService.ts:227` |
| 248 | Compute checksum; mismatch warning | Compute and stamp | `computeRulePackChecksum` `projects/chirality-piping/apps/desktop/src/services/rulePackService.ts:150` (host `compute_rule_pack_document_checksum` `projects/chirality-piping/apps/desktop/src-tauri/src/lib.rs:3158`); `stampChecksumIntoDocument` `:412` |
| 249 | Set as project rule pack | Persist the project's pack binding | **GAP (typed interface)** G-25: no project-level binding in `LocalProjectEnvelope` `projects/chirality-piping/apps/desktop/src/types.ts:964`; the run cites `rule_pack_refs` `:446` |
| 250 | Editor: declarations, value slots, checks; structured composer | Edit the pack document | `setFormulaExpression` `projects/chirality-piping/apps/desktop/src/features/rule-packs/ExpressionComposer.tsx:218`; `ExpressionComposer` usage `projects/chirality-piping/apps/desktop/src/features/rule-packs/ExpressionComposer.tsx:1220` |
| 251 | Rendered expression in the dashed frame with the M-15 caption | Derived, display-only | `renderExpressionText` `projects/chirality-piping/apps/desktop/src/features/rule-packs/ExpressionComposer.tsx:403` |
| 252 | "Invented values · not engineering data" label on public example packs | Derived from the pack's public flag (C-50) | **GAP (interface)** G-28 |

## Q. Export and handoff

| # | Control or gesture | Operation it invokes | Provider (file:line) or GAP |
|---|---|---|---|
| 253 | Export sheet: target, units system, included tables, metadata preview, Write file… | Build the export packet | `buildCaePipeMbfExportPacket` `projects/chirality-piping/apps/desktop/src/features/caepipe-mbf/CaepipeMbfExportPanel.tsx:151`; `renderCaePipeMbfText` `:450`; packet schema `projects/chirality-piping/schemas/caepipe_mbf_export.schema.json` (`unit_system_disclosure` `:17`, `manifest` `:18`, `stable_id_map` `:21`, `loss_report` `:22`, `validation_report` `:23`); Write file… today a browser download `:85`, native save **GAP (interface)** G-26 |
| 254 | Gap 1 alphanumeric IDs: written as they are | Projection | numeric ids by G-01; renderer `:450` |
| 255 | Gap 2 element type → code (Pipe blank; Bend; Valve, Reducer, Rigid, Expansion joint) | Projection | `renderCaePipeMbfText` `projects/chirality-piping/apps/desktop/src/features/caepipe-mbf/CaepipeMbfExportPanel.tsx:450`; the correspondence and its loss entry **GAP (typed interface)** G-26 (RESEARCH-E §6 gap 2) |
| 256 | Gap 3 tee: node-data kind → node code on the tee node's line | Projection | `branch_connection_type` `projects/chirality-piping/apps/desktop/src/types.ts:24`; renderer `:450`; G-26 |
| 257 | Gap 4 temperatures and pressures through load sets; gap 5 the load-reference key with its loss note | Projection | **GAP** G-03 (sets) and G-26 (key) |
| 258 | Gaps 6, 7 origins and Checked dropped with loss entries | Projection | `loss_report` `projects/chirality-piping/schemas/caepipe_mbf_export.schema.json:22`; `LossReportEntry` `:359` (category `:379`) |
| 259 | Gaps 8, 15 per-value and per-item units → one file switch with the unit disclosure | Projection | `unit_system_disclosure` `:17`; `convertDisplayQuantities` `projects/chirality-piping/apps/desktop/src/services/displayQuantityService.ts:20` |
| 260 | Gap 9 concentrated mass written as weight | Projection | not representable today: G-23; loss entry G-26 |
| 261 | Gaps 10, 12 snubber and time-varying load: not offered, nothing written | None | recorded non-goal "Snubbers" `projects/chirality-piping/apps/desktop/src/features/toolkit/capabilityCatalog.ts:375`; G-24 |
| 262 | Gap 11 rod hanger written as a rigid Y restraint with a loss note | Projection | row 86; `loss_report` `:22` |
| 263 | Gaps 13, 14 load cases and combinations dropped with one loss entry per case | Projection | `loss_report` `projects/chirality-piping/schemas/caepipe_mbf_export.schema.json:22` |
| 264 | Gap 16 export metadata beside the file (manifest, stable id map, loss, validation, unit disclosure, hash, run identity) | Projection | `manifest` `:18`, `stable_id_map` `:21`, `validation_report` `:23` (`projects/chirality-piping/schemas/caepipe_mbf_export.schema.json`); model hash `projects/chirality-piping/apps/desktop/src/services/hashService.ts:117` |
| 265 | Write-on-change for Section, Material, Load; tag truncation; location lines; row order = file order | Projection options recorded in the loss report | **GAP (typed interface)** G-26 |
| 266 | M-06 sentence on the sheet | Wording | `projects/chirality-piping/docs/claims_registry.md:52` |
| 267 | Handoff package (model, run record, results export, report, export metadata, provenance summary; private data excluded by default) | Build the package | `buildHandoffPackage` `projects/chirality-piping/apps/desktop/src/features/handoff/HandoffPanel.tsx:159`; `HandoffPanel` `:43`; `private_payload_included: false` `:271` |
| 268 | Results export from a historical run (named historical) | Export | `buildCurrentResultExport` `projects/chirality-piping/apps/desktop/src/features/result-export/resultExportAdapter.ts:138`; historical designation `projects/chirality-piping/apps/desktop/src/features/results/HistoricalRunContext.tsx:11` |

## R. Edge states (§10)

| # | Control or gesture | Operation it invokes | Provider (file:line) or GAP |
|---|---|---|---|
| 269 | New project first-open state | Blank model; interface pre-creates rows 10 and 20 | `buildBlankLocalModelDocument` `projects/chirality-piping/apps/desktop/src/services/projectService.ts:450`; row pre-creation interface (row 62) |
| 270 | Model with no sections (centreline drawing; per-element Blocks-solve issues) | Derived | `MissingDataBlockingPanel` `projects/chirality-piping/apps/desktop/src/features/missing-data/MissingDataBlockingPanel.tsx:31`; centreline rendering **GAP (rendering)** G-16 |
| 271 | Failed run | Rows 184, 190, 139 |  |
| 272 | Reopened historical run | Rows 136, 197 |  |
| 273 | Proposal row edited before acceptance | Row 214 |  |
| 274 | Paste with unmapped columns | Row 75 |  |
| 275 | Stale Checked mark; undo makes it current | Row 221 | G-08 |
| 276 | Library missing on open | Rows 145, 244 |  |
| 277 | Window at minimum size | Row 40 |  |

---

## The gap list

Grouped by class. Each entry names the rows that depend on it and the open question in `UX_SPEC_V1.md` §11 where one exists. Interface-only work with no engine or typed-interface dependency is collected under G-28 so the engine-facing gaps read as a list of their own.

### Engine gaps (an operation or a result the engine does not produce)

| ID | Gap | Rows | §11 |
|---|---|---|---|
| G-01 | Numeric node identity with renumbering: one node, or all by start and increment, with every reference rewritten in one operation. Roadmap "Node renumbering" (`projects/chirality-piping/apps/desktop/src/features/toolkit/capabilityCatalog.ts:375`); node ids are free string `Id`s. | 46, 63, 66, 254 | 1 |
| G-02 | Offsets that translate the To node and its downstream chain as one checkpoint: no translate-chain change kind; composed from `set_field` per node unless one is added. | 45 | 2 |
| G-03 | Named load sets: a record of T and P pairs referenced by the layout row, read through, edited for every reader or forked; used-by counts and delete refusal. | 61, 79, 104–106, 257 | 3 |
| G-04 | Generation of primitive cases and combinations from the load sets and the rule pack as a previewable batch with the Generated origin; Regenerate that leaves edited and authored rows. | 115 | 4 |
| G-06 | The hanger design pass: design load and travel per hanger location from the run, the operating case named in settings, the Max variation field on the hanger record, and the hanger state. | 88, 91, 141, 142 | 5, 6 |
| G-12 | Run settings beyond solver mode (iteration limit, cases to run, operating case for travel, hanger pass on or off) recorded under the existing `settings_ref`; per-case progress events; the full log in the run record; what a stopped run emits (statuses, the case it stopped in). | 139, 188, 191, 193, 206 | 11 |
| G-13 | Evidence label emission (`INTERNALLY_VERIFIED`, `PROVER_CORRELATED`) and its carrier in the run record; not emitted in the desktop source today. | 135, 207 | 12 |
| G-22 | Restraint kinds the support record cannot carry: a restraint along a direction vector; a restraint between two pipe nodes. | 87 | 22 |
| G-23 | Node data and element fields not represented: concentrated weight as node mass with offset; weld type and mismatch; threaded joint; valve thickness and insulation factors; reducer wall per end and cone angle. | 53, 54, 93, 100, 260 | 23 |
| G-24 | Element kinds not represented (slip joint, ball joint, hinge, tie rod, elastic element, beam, cut pipe, jacketed pipe and bend, miter bend) and the recorded non-goals (snubber, cold spring): to be sorted into gaps and non-goals for About › Scope and limitations. | 51, 261 | 21 |

### Typed-interface gaps (a field or record the schemas and types do not carry)

| ID | Gap | Rows | §11 |
|---|---|---|---|
| G-05 | Stress type per case and combination; a rule ID bound per row and stored with the model. | 110, 111, 118 | 14 |
| G-07 | Origins per row and cell persisted with the project, with author classes Entered, Accepted, Propagated, Generated, Imported (the applier's `author_type` and the session receipts exist; propagation and generation classes and persistence do not). | 60, 78, 79, 116, 118, 174, 223 | 7 |
| G-08 | The Checked mark: a human row tag bound to the row's content hash with name and time, set and cleared by the engineer only, stale on content change, persisted with the project outside the model payload, never exported; its filters and counts. | 78, 174, 183, 221, 222, 275 | 8 |
| G-09 | Changes since (last run, last Checked, a snapshot, a time) at row and cell grain from a persisted ledger that records hash and time per applied operation. | 78, 183, 224 | 9 |
| G-10 | Runs as a kept list: every run numbered per project, named (`run_name` exists), immutable, with its model hash; an earlier run shown as historical. One `analysis_run` slot today. | 134, 196 | 10 |
| G-11 | Results retained as stale after a model change: the run kept as the historical basis, values hatched and readable, the stale band, chips dropped, unsolved elements drawn unsolved. Today the run is cleared on any change. | 8, 137, 146, 166, 199, 218 | — |
| G-14 | Envelope (governing case per row), the Restraint summary and the Summary page as interface derivations over per-case per-element rows; no engine operation, but the rows must exist. | 122, 127, 129 | 13 |
| G-15 | Per-element per-case binding of stress, allowable, ratio, rule ID and pack version in the check outcomes; force and moment frames (global, local). | 123, 128 | 14 |
| G-17 | Interface state persisted with the project: per-stage view memory, camera and the report-figure preset, label mode, hidden sets, table views, drawer heights, agent column. | 10, 162, 165, 232 | — |
| G-18 | Proposal record extensions: per-row decision state (pending, accepted, rejected, stale, blocked), constraints considered and TBD per row, rejection record, "superseded by", accepted receipts persisted with the project (one `proposal` slot and session receipts today). | 210, 211, 213, 214, 217 | 18 |
| G-20 | The Review page's records: authored text blocks with references and value tokens, live block selection and filters, editable section order, review/signoff values, iterations as named immutable snapshots bound to run and hash (a `ModelStateRecord` is the nearest carrier), the diff, restore. | 226–230, 234 | 16 |
| G-21 | Comments by reference (rows, results, runs, report paragraphs) with open and resolved states, persisted with the project (`StateNote` is the nearest carrier). | 220, 231 | 17 |
| G-25 | A project-level rule-pack binding ("project rule pack") persisted with the project and cited by the run, the generator and the hanger pass. | 249 | — |
| G-26 | Export projection options and records: the element-code correspondence and its loss entry, the load-reference key's loss note, write-on-change for Section, Material and Load, tag truncation, location lines, mass-as-weight entries, and a native file save (today a browser download). | 253, 255–257, 260, 265 | 20 |

### Host gap

| ID | Gap | Rows | §11 |
|---|---|---|---|
| G-19 | A live agent binding: conversation, proposals arriving in session, withdrawal, checks and open issues by reference, drafts for the Review page. The standing hold (DEC-042) governs when; today a proposal arrives as a document through the offline intake. | 208, 209, 216, 220 | 18 |

### Rendering gaps (the canvas)

| ID | Gap | Rows | §11 |
|---|---|---|---|
| G-16 | Canvas capabilities the viewport does not draw: the probe; result colour on the tube and the legend; deformation factor stepper and animation; label Budget mode; the Right preset; the report-figure camera; the section plane; the proposal ghost; box-selection kind filter; follow-selection panning; centreline drawing for sectionless elements. | 149, 150, 152, 155, 156, 158, 160, 162, 163, 210, 232, 270 | 15 |

### Interface gaps (no engine or typed-interface dependency)

| ID | Gap | Rows | §11 |
|---|---|---|---|
| G-27 | Paste with mapping: the band (header mapping, preview, source IDs kept, ignored columns) landing as one batch through the existing batch route. | 75, 274 | 19 |
| G-28 | The interface components with no engine dependency: the three views and per-stage layout; the table component (sort as view, filters, row expansion, footer, keyboard model, copy and CSV export); palette entity search; About and Scope and limitations pages; window floors and the slide-over fallback; docked inspector in Both; agent column and strip; multi-row ordered selection; print and PDF from the preview; re-pointing a missing library; preferences for node increment and number formatting. | 9, 14, 17, 20, 21, 32, 39–43, 74, 76, 79, 80, 90, 101, 130, 132, 148, 169, 208, 233, 244, 252 | — |
| G-29 | The creation intent does not offer the schema's `rigid` component kind. | 50 | — |

Counts: 277 numbered rows, of which 263 carry a provider or a gap in their own cell and 14 are cross-references to rows listed under another surface; 29 gap entries (G-01 to G-29). Every row that is not a gap and not a cross-reference cites at least one source file and line, or a gap entry it depends on.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
