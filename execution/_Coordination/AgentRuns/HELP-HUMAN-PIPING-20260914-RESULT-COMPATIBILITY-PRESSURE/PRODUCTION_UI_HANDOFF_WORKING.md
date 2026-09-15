# Production UI foundation handoff — working

Status: prepared for final readiness assessment; implementation of this UI tranche is not activated by this document. HELP_HUMAN will freeze the readiness decision only after the accepted compatibility/native source and evidence are available.

The user wants a professional engineering modelling workspace with a proper 3D viewport and an industry-familiar tool palette. The accepted predecessor is the 20260913 RESULTS-ENGINEERING-3D run: `instances/ROOT/FINAL_ACCEPTANCE_V1.md`, its `UI_ACCEPTANCE_V1.json`, `instances/UI/INTERACTION_CONTRACT_V1.md`, and `instances/UI/successor-v3/prototype/index.html`. These are derivative design and simulated interaction evidence, not a production implementation or a claim of personal visual approval. The original session handoff remains unchanged.

## Interface handoff

| Surface | Production seam to retain | First production slice |
|---|---|---|
| Viewport | `features/viewport/PipeViewport.tsx`, `routeDraft.ts`, `viewportRouting.ts` | Camera/orbit/pan/zoom and standard views, readable axes/units, construction plane, unambiguous picking, explicit route draft and matching tree selection. Camera and selection never dirty the model. |
| Model tree and selection | `features/model-tree/ModelTree.tsx` and typed entity identity | One selection identity across tree, canvas, task rail and properties; clear supported entity groups and deterministic focus. |
| Property inspector | `features/model-tree/PropertyInspector.tsx` plus existing typed setters | Show entered values, units and provenance; commit supported edits through the shared operation route, with inline validation and reference guards. |
| Tool palette | `features/toolkit/capabilityCatalog.ts` | Build, Supports, Properties, Loads, Edit, Select and View, Review; searchable named commands and keyboard access; explicit disabled reasons for unsupported actions. |
| Model mutation and history | `services/operationService.ts`, `operationBatchService.ts`, Rust operation applier, App history guards | Prepare/validate/review/apply on an exact source model; atomic route and edit batches; one checkpoint per successful batch; busy/stale/cancel protections. Human and synthetic agent-authored intents must reach the same applier and exact model hash. |
| Result inspection | Accepted result semantics, analysis record and Current/Historical designation from this foundation tranche | Quantity-aware tables and overlays with declared units/frame/location/basis; Historical stays readable without Current eligibility; diagnostic work and discrete evidence do not become ratios or moments. |

Paths in this table are relative to `projects/chirality-piping/apps/desktop/src`, except the explicitly named Rust applier. The final source binding will name exact candidate hashes rather than treating this working table as a frozen API manifest.

Production development may restructure internal component props and layout while preserving the accepted typed operation and result contracts. The current viewport already separates selection, draft preparation, reviewed application and invalidation callbacks; the tree and inspector share `EntityRef` and `EditorOperationIntent`. Preserve those distinctions through the redesign, including one history checkpoint per successful atomic batch and result revision identity separate from mechanics-run identity.

## Readiness decision to complete at tranche closure

1. Stable typed operation and result interfaces: pending final compatibility acceptance and unchanged public operation/model/input-manifest contract check.
2. Truthful Current/Historical behavior: pending new and legacy record verification plus lifecycle checks.
3. Native modelling lifecycle: pending actual author/solve/edit/undo/redo/save/normal quit/reopen/re-solve witness on the supplied candidate.
4. Accepted viewport/selection/tree/inspector/palette interface handoff: predecessor design accepted; current interface binding and final root handoff pending.

If these gates pass, the first production UI foundation can begin while private pressure runtime, objective connectors and numerical/sparse work proceed in separately owned scopes. It need not wait for the entire physics backlog, vendor exports, the agent harness, formal whole-project closure or unrelated interoperability readiness. Pressure and connector controls remain explicitly unavailable until their owning runtime contracts exist.

## Suggested execution boundary

Use one WORKING_ITEMS manager for the coupled desktop workspace slice and one Sol/high product writer at a time. Agent 0 retains interaction priorities and integration decisions. Use a direct Astra/high independent review and Sol/high native/keyboard/pointer workflow witness. Add a HELPS_HUMANS design manager only if personal visual alignment or interaction decisions materially change the accepted design basis; do not create a duplicate design layer automatically. Stay within six active instances, serialize native/browser builds and final Git integration, and isolate concurrent physics writes.

Acceptance should exercise real production actions, not simulated prototype results: node and pipe creation, support/load editing, shared selection and properties, atomic application, undo/redo identity, Current invalidation, native persistence and source-aware result inspection. Validate 1024×768 minimum and the larger accepted viewport sizes, keyboard focus and task accessibility, and clearly identified unavailable capabilities. Prepare the exact bounded file scope after this foundation closes; no production UI file is changed by this working handoff.
