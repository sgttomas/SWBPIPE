# U7 source release amendment V1

**Status:** `SEALED_PENDING_ROOT_RELEASE`
**Manager:** U7 `WORKING_ITEMS`
**Model:** `gpt-5.6-sol`, high reasoning
**Basis:** source `55df51ac3201456e0f181823e3aefefef47a73bb`; adopted Owner act `../../OWNER_ACT.md`; shared contract `../../SHARED_SOURCE_LAUNCH_CONTRACT_V1.md`; frozen U7 brief `../../../../../PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor/_run_records/PHYSICS_UI_EXECUTION_20260908/IMPLEMENTATION_BRIEF_V1.md` at successor SHA-256 `23a2ac4595c4b9ee2e18143b057a392f42c34c57051e5f328b45cbfc2ab1f709`.

Implement exactly the frozen straight-route and inline inspector Add/Apply slice. Preserve incomplete node-only create/save/reopen behavior, current atomic operation and checkpoint paths, stale/cancel/hash/revision protection, unit/provenance/rule-field truthfulness, persistent canvas, and actual native evidence. Do not change a producer, backend/service API, Rust/Tauri command, schema, component family, shared section-reference contract, unit policy, rule semantics, or persistence contract.

The exact nine source/test paths are:

- `{WORKING_ROOT}/apps/desktop/src/App.tsx`
- `{WORKING_ROOT}/apps/desktop/src/App.test.tsx`
- `{WORKING_ROOT}/apps/desktop/src/features/viewport/PipeViewport.tsx`
- `{WORKING_ROOT}/apps/desktop/src/features/viewport/routeDraft.ts` (new)
- `{WORKING_ROOT}/apps/desktop/src/features/viewport/routeDraft.test.ts` (new)
- `{WORKING_ROOT}/apps/desktop/src/features/model-tree/PropertyInspector.tsx`
- `{WORKING_ROOT}/apps/desktop/src/features/model-tree/typedInspector.test.tsx`
- `{WORKING_ROOT}/apps/desktop/src/styles.css`
- `{WORKING_ROOT}/apps/desktop/e2e/linear-authoring.spec.ts` (new)

Allowed evidence/control writes:

- `{WORKING_ROOT}/execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor/_run_records/PHYSICS_UI_IMPLEMENTATION_20260908/**`
- `{WORKING_ROOT}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/U7/**`

No DEL-07-02 or other source/deliverable path is writable. The manager may dispatch bounded `gpt-5.6-sol` high Agent 2 work inside these fences and must persist each sealed child brief first. During parallel F4 work, U7 may edit and run targeted JavaScript tests but may not run WASM, native, or Rust builds. Freeze the complete diff and focused checks, then return for fresh independent 100% product-diff review. Actual packaged Tauri build/walkthrough is root-serialized after both slices freeze and their reviews pass. Stop on any producer or fence change, failed consumer proof, stale-publication path, new default/contract, substantive engineering uncertainty, or review finding outside the brief.

Source writes remain held until CHANGE Step 0 passes and root explicitly releases this sealed amendment after reading its hash.
