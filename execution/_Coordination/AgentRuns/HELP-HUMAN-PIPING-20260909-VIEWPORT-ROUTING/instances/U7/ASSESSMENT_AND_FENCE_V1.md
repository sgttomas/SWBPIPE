# U7 viewport-routing assessment and implementation fence V1

Status: `FROZEN_CANDIDATE_FOR_INDEPENDENT_REFUTATION`
Basis: repository HEAD `533332349a4607eee561d4ef90fb05a62d86519e`.

## Current source findings

1. **Mixed-unit pointer capture is incorrect.** `PipeViewport.tsx:328-349, 771-799, 1717-1731, 1754-1784` produces a point in the scene/model coordinate basis, then writes the raw numbers into a draft labelled with the selected `coordinateUnit`. `routeDraft.ts:416-442` emits those numbers and that selected unit. The accepted operation producer converts the intent unit into `model.project.units.length` (`operation_applier/src/lib.rs:2948-3145`; regression `:9809-9841`). Thus a model-space point `3.2` in a metre model captured into a millimetre draft is interpreted as `3.2 mm` and stored as `0.0032 m`, rather than the intended `3200 mm` / `3.2 m`. Validation correctly validates the relabelled value; it cannot recover the lost basis. This is a consumer defect; no producer change is indicated.
2. **Authoring and navigation share pointerdown without arbitration.** `PipeViewport.tsx:351-375` attaches `OrbitControls` to the renderer canvas, while `:855-862` attaches authoring to the containing viewport and `:773-803` commits selection/draft capture on primary pointerdown. There is no pointerup, movement, or controls-change gate. An armed blank-space orbit gesture can therefore capture its initial point. This is a source-level interaction collision requiring browser/native confirmation after repair.
3. **No-WebGL pointer authoring synthesizes unseen geometry.** `PipeViewport.tsx:328-338` replaces the canvas with a WebGL-unavailable message but installs `fallbackDraftPointFromHostEvent`; `:762-799, 1770-1784` then maps host pixels and model bounds to coordinates. Current jsdom coverage at `App.test.tsx:14724` exercises this synthetic path. The selected behavior is fail-closed pointer authoring with manual numeric entry retained.
4. **The useful 3D route preview is absent.** The Three scene renders committed pipes/nodes and result overlays (`PipeViewport.tsx:382-448`), but no draft route or construction-plane object. The UI only names fixed `XZ @ Y=0` at `:1249`. Existing continuation correctly waits for an accepted own commit (`:304-319, 638-669`) and is the state transition to preserve.

## Selected first tranche

Keep Three.js, the current palette, exact existing/new endpoint distinction, explicit draft fields, and the accepted `Add route -> frozen validation/diff -> Apply` service path.

For a **new endpoint**:

- Show an explicit construction-plane selector with only `XY`, `XZ`, and `YZ`. The selected plane passes through the exact chosen `From` node. The readout names the fixed axis, its elevation, the model length unit, and the From node ID, for example `XZ · Y=2.4 m · through node:N-120`.
- Offer `Free`, `X`, `Y`, and `Z` axis constraints. Only the two axes lying in the selected plane are enabled. Projection intersects the chosen plane first; an axis constraint then holds the other two coordinates at the From-node anchor. `Free` is the transient initial interaction state. Changing plane or constraint affects later pointer placement and invalidates a frozen review; it does not silently rewrite manually typed X/Y/Z.
- Render a transient Three.js construction grid, dashed ghost segment, and endpoint marker. Pointer hover drives the ghost in model units. A click captures the constrained point into editable endpoint fields. Pointer capture also assigns the existing next-available node ID/label logic, while provenance remains blank and Add remains disabled until explicitly supplied.
- Convert model-space coordinates to the selected draft unit through the existing `convertDisplayQuantities` producer in one three-axis batch. Accept only three finite, ID-matched results in the requested unit; otherwise publish no coordinate change and show a local draft error. Separate generation/invalidation prevents delayed conversion from overwriting a manual edit, cancel, mode change, selection change, or model replacement.
- Use pointerdown only to start a candidate gesture. Commit picking/capture on a same-position primary pointerup; any intervening movement, pointer cancellation, or OrbitControls navigation change cancels authoring. Hover may still update the ghost. No engineering tolerance is introduced.

For an **existing endpoint**, keep explicit node identity and show the transient ghost only when both exact endpoint IDs resolve. Plane/axis controls are inactive because no projected endpoint is being authored.

On successful own-commit continuation, retain the selected plane, applicable axis, endpoint unit, material, dimensions, y-reference and pipe provenance; move From to the accepted end; clear the consumed pipe/new-node identity and endpoint provenance; and keep new-end mode ready for the next pointer capture. Cancel, tool exit, external replacement/open/create, undo, redo, or stale response clears the ghost and continuation through the existing invalidation rules. A preview remains transient and never becomes an operation, checkpoint, persistence record, or topology inference.

Standalone node pointer capture remains on the explicitly stated global `XZ @ Y=0` plane in this tranche, but receives the same unit conversion, click-versus-drag protection, blank provenance, and no-WebGL fail-closed behavior. Typed node and route coordinates remain authoritative and may intentionally depart from the transient plane/axis aid.

## Alternatives rejected for this tranche

- Keeping only global `XZ @ Y=0` is too restrictive for elevated route continuation.
- Arbitrary rotated or user-defined planes require a larger coordinate-system contract and are deferred.
- Parsing catalog `factor_representation` strings in React would duplicate the accepted conversion producer.
- Rebuilding the Three renderer on every pointer move would disturb camera/navigation state; draft visuals should update through dedicated transient scene-object refs.
- A bounds-based no-WebGL coordinate fallback invents spatial input without a visible projection basis.

## Exact proposed source/test fence

Only these paths may change after root release:

1. `projects/chirality-piping/apps/desktop/src/features/viewport/PipeViewport.tsx`
2. `projects/chirality-piping/apps/desktop/src/features/viewport/viewportRouting.ts` (new private helper)
3. `projects/chirality-piping/apps/desktop/src/features/viewport/viewportRouting.test.ts` (new focused tests)
4. `projects/chirality-piping/apps/desktop/src/App.test.tsx`
5. `projects/chirality-piping/apps/desktop/src/styles.css`
6. `projects/chirality-piping/apps/desktop/e2e/linear-authoring.spec.ts`

Read-only consumed files: `routeDraft.ts`, `displayQuantityService.ts`, `unitCatalogService.ts`, `types.ts`, `App.tsx`, operation producers, and registered test/build configuration. No service, public type/schema, operation contract, persistence, Rust, Tauri configuration, package manifest, component-family implementation, or deliverable-state edit is proposed.

## Acceptance and validation plan

Focused helper tests must cover all three plane equations and anchors, every applicable axis constraint, model-unit to draft-unit conversion including `3.2 m -> 3200 mm`, malformed/unavailable conversion rejection, and gesture completion versus movement/cancel.

App-level tests must prove: no-WebGL pointer actions leave coordinates untouched while typed entry remains usable; delayed conversion cannot overwrite invalidated/manual state; Add/review/Apply still emits the exact single operation or atomic `[create_node, connect_pipe_run]` batch; canceled/stale paths publish no model, receipt, or checkpoint; and own-commit continuation retains the route aids while external model events clear them. Existing provenance, ID reservation, receipt/hash/revision, warning rejection, and zero-coordinate cases remain unchanged and passing.

Playwright must exercise a visible Three ghost and plane/grid/readout at 1024x768, exact new-versus-existing endpoint behavior, an elevated From node on a non-global plane, an applicable axis constraint, an orbit drag that does not capture, pointer capture followed by editable values, Add/review/Apply, and continued routing from the accepted end. Run all configured projects. The later isolated native witness must repeat the mixed-unit capture using a catalog unit different from the model unit and confirm save/reopen only after explicit Apply. Browser/native visual evidence supplements, rather than substitutes for, focused and App-level assertions.

Registered affected checks after the final frozen source cut are desktop tests and desktop build. Root/CHANGE owns broader clean verification and Git. A required path outside this fence, a need to change the conversion producer, or inability to distinguish navigation without changing OrbitControls configuration returns to root before expansion.
