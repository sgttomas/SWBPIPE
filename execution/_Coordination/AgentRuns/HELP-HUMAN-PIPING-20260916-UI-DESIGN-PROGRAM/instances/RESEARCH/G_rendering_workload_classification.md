# G — Rendering-brief workloads classified against the product as it exists

Bounded read-only research for ROOT of run `HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM`,
under the bounded brief `briefs/RESEARCH-G_rendering_workload_classification.md`. Source
reading only: no build, test, benchmark, dev server or product run was executed. Nothing
outside this file was written.

## 0. Basis

**Revision read.** `HEAD` = `451c5f595e0488a0d6064d9b0f972c5e8fc1d09d` (branch
`codex/swb-ui-design-handoff-reconciliation`). The only working-tree changes at the time of
reading were this run's own brief index and brief file; no product or instrument file was
modified or dirty.

**Citation convention.** Every path below is relative to `{WORKING_ROOT}` =
`projects/chirality-piping`. Line numbers are at the `HEAD` above.

**What was read.**

- The rendering brief `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM/instances/ROOT/RENDERING_BRIEF_2026-09-18.md`
  and its notice `execution/_Coordination/NOTICE_2026-09-18_RENDERING_BRIEF.md`.
- Product rendering source: `apps/desktop/src/features/viewport/PipeViewport.tsx` (4,532 lines,
  read in full or by targeted extraction of every `THREE.*` construction site),
  `viewportResource.ts`, `viewportSelection.ts`, `viewportSelectionPresentation.ts`,
  `viewportRouting.ts`, `selectionPresentationBinding.ts`;
  `apps/desktop/src/features/workspace/uiDiagnostics.ts`, `workspaceCanvasBudget.ts`,
  `uiPreferences.ts`; `apps/desktop/src/features/results/ResultsPanel.tsx`;
  `apps/desktop/src/features/agent-proposals/AgentProposalPanel.tsx`;
  `apps/desktop/src/features/diff-preview/DiffPreviewPanel.tsx`;
  `apps/desktop/src/features/design-workspace/DesignWorkspacePanel.tsx`;
  `apps/desktop/src/App.tsx`; `apps/desktop/src/styles.css`.
- Instrument: `apps/desktop/e2e/ui-foundation/README.md`, `characterization-commands.ts`,
  `benchmark-harness.ts`, `performance-targets.ts`, `full-cohort-controller.ts`,
  `fixture-manifest.json`, `generate-fixtures.mjs`, and the fixture/sample inventory;
  `apps/desktop/e2e/ui-foundation-workflows.ts`, `apps/desktop/e2e/workspace-driver.ts`,
  `apps/desktop/e2e/ui-foundation.spec.ts`, `apps/desktop/e2e/r2-smoke.spec.ts`.
- Evidence reports: `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-PICKING-STABILITY/RUNTIME_REPORT.md`
  and `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260915-PRODUCTION-UI/instances/ROOT/D70_BASELINE/BASELINE_CHARACTERIZATION_REPORT.md`.

**What a source reading could not determine.**

- Any runtime cost. No frame time, repaint time, memory figure or draw-call count for a
  behaviour that is not already in the recorded evidence above is derivable from source.
- Whether the in-product mechanics preview can produce a displacement result at 1,000 or
  10,000 pipes within a benchmark session. The existing functional exercise of the
  deformation overlay uses a five-node canned preview
  (`apps/desktop/e2e/r2-smoke.spec.ts:570-580`).
- Whether the frozen boundary profile's host, display and Chromium bindings can be
  re-established for a later observation run; that is an environment fact, not a source fact.

**Two standing cautions carried into every row below.** A type, a schema field, a CSS rule or
a test fixture is not a rendered behaviour. Where a capability exists only in one of those
forms, the row says so. Trace spans are not hardware GPU execution time, Chromium
presentation feedback is not physical scanout, and boundary snapshots are not continuous
monitoring.

**Dead-code finding that affects several rows.** `PipeViewport.tsx` defines per-element
renderers that nothing calls: `pipeMesh` (`:3670`), `supportMesh` (`:4019`), `componentMesh`
(`:4034`, which contains the elbow arc, valve/flange ring pairs and expansion-joint bellows),
`deformedPipeMesh` (`:4182`), `deformationMarker` (`:4202`), `referenceGround` (`:4221`),
`buildOrientationGizmo` (`:4364`) and `axisLabelSprite` (`:4373`). Search:
`grep -rn "pipeMesh|supportMesh|componentMesh|deformedPipeMesh|deformationMarker|buildOrientationGizmo" apps/desktop/src apps/desktop/e2e`
returns only these definitions plus unrelated `UiOwnedResourceCounts` field names
(`viewportResource.ts:111-114`, `:1002-1050`). The drawn scene is built only from the
instanced paths listed at `PipeViewport.tsx:1232-1244` and `:1252-1264`. Recognisable fitting
geometry therefore **is not drawn** at this revision, notwithstanding the unreferenced
function.

---

## 1. Item-by-item classification

Class 1 = implemented and observable now with the instrument as it stands or with
configuration only. Class 2 = the product does this (fully, or in the stated partial form)
but the instrument has no named setup or query action, stimulus or probe for it. Class 3 =
the product does not do this.

### 1.1 Rendering brief §2 — the figure language

| # | Item | Class | What exists today (file:line) | What is missing | Class 2: smallest harness adaptation / Class 3: nearest existing behaviour |
|---|---|---|---|---|---|
| 2.1a | Pipe drawn at real outside diameter | 1 | `Actual OD` mode converts each pipe's OD and wall through the unit service and feeds a per-pipe radius into the instanced tube (`apps/desktop/src/features/viewport/PipeViewport.tsx:728-800`, `:1232-1237`, `:3715-3758`); toolbar control `viewport-geometry-actual-od` (`:2235`). Instrument drives it by name: `geometry("actual-od")` (`apps/desktop/e2e/ui-foundation/characterization-commands.ts:36`), with `odStatus`/`odGeneration` validated at every boundary (`:55-56`) and a cold/warm conversion contract (`apps/desktop/e2e/ui-foundation/fixture-manifest.json`, `od_conversion_states`). | — | — |
| 2.1b | Neutral matte pipe shading, darkening toward the silhouette, never a highlight | 1 | Tubes are `MeshStandardMaterial` at `metalness 0.2`, `roughness 0.58` (`PipeViewport.tsx:3732`) under one ambient light at 0.72 and one directional key at 1.2 (`apps/desktop/src/features/viewport/viewportResource.ts:415-418`). A shaded tube is drawn on every measured frame. | The current material and key light produce a specular highlight; the brief asks for none. This is a material/token change on an existing draw, not a new draw. | — |
| 2.1c | One-pixel edge line at every zoom, at the silhouette and at every change of section | 3 | Nothing. Searches: `grep -rniE "EdgesGeometry\|LineSegments\|LineBasicMaterial\|outline\|silhouette\|halo" apps/desktop/src` returns only CSS `outline` rules for DOM focus/selection and the one `LineSegments` in `viewportSelectionPresentation.ts:8,16`. No model-geometry edge pass exists. | The whole behaviour. | Nearest: the selection cue's `LineSegments` centreline spans (`viewportSelectionPresentation.ts:16-18`, `:60-79`), drawn only for selected entities, with `depthTest: false`, along the authored centreline rather than the silhouette. Measuring it would characterise a selection overlay of at most the selected set, not a per-element silhouette-and-section edge pass over every element at every zoom. |
| 2.1d | Insulation as a translucent sleeve | 3 | Nothing drawn. `insulation_thickness`/`insulation_density` exist only as inspector schema fields and validation (`apps/desktop/src/features/model-tree/PropertyInspector.tsx:419`, `:476`, `:1721-1722`). Search: `grep -rniE "insulation" apps/desktop/src` returns only inspector and toolkit-test hits, none in `features/viewport`. | The whole behaviour. | No nearest behaviour: no second concentric or translucent sleeve is drawn around a pipe anywhere. |
| 2.1e | An element without a section draws as a 1.5 px centreline | 3 | `instancedPipeMeshes` always draws a tube; in schematic mode every pipe gets the same fixed world radius 0.052, independent of section (`PipeViewport.tsx:3715-3758`). Sectionless elements are not distinguished; only invalid geometry is excluded (`:3722-3730`). | Screen-width line rendering, and any section-presence branch. | Nearest: schematic mode, which is a fixed-world-radius tube, not a fixed-screen-width line. Its cost scales with tube tessellation (10 radial segments, `:3731`) and would not characterise a line-primitive path. |
| 2.2 | Fittings as recognisable geometry (swept elbows with tangent ticks, tee saddles, reducer cones, valve silhouettes, flange disc pairs, convoluted expansion joints, beams as their section), each taking the edge line and the result colour | 3 | Components are drawn as four generic instanced primitives chosen by kind: a partial torus for bends/elbows, a thin cylinder for branches, a short cylinder for expansion joints, a box for everything else (`PipeViewport.tsx:3819-3864`, geometry selection at `:3836-3841`). The richer shapes exist only in the unreferenced `componentMesh` (`:4034-4165`) — see the dead-code finding in §0. | Every named fitting form; tangent ticks; branch saddles; the edge line (2.1c); the result colour (2.7a). | Nearest: the four generic instanced primitives above. They carry no section, no sweep radius and no branch geometry, so their instance counts and triangle counts would not characterise swept/saddled per-fitting geometry. |
| 2.3a | Restraint glyphs as the standard symbols (anchor hatched block, +Y triangle, guide bracket pair, limit-stop plate, two-way bar, spring can and rod, connecting tie) | 3 | Every support, of every kind, is drawn as one four-sided cone of fixed world size 0.18 × 0.34, rotated 45°, offset 0.26 below the node, in one colour (`PipeViewport.tsx:3788-3817`). No branch on restraint kind exists in any drawn path. | Per-kind symbol geometry. | Nearest: the single generic cone. One shared geometry and material for all supports; measuring it characterises one instanced cone mesh per spatial chunk, not seven distinct symbol families. |
| 2.3b | Glyphs at a fixed screen size at any zoom | 3 | Support cones are world-sized (`PipeViewport.tsx:3800`), so their screen size changes with zoom. | Screen-space sizing for glyphs. | Nearest: the selection marker point sprite, which *is* fixed screen size — `gl_PointSize = 11.0 * pixelRatio` in a `ShaderMaterial` on `THREE.Points` (`viewportSelectionPresentation.ts:19-44`), with the pixel ratio refreshed from the renderer (`viewportResource.ts:609-615`). That path exists but is used only for the selected set and draws a diamond, not a restraint symbol. |
| 2.3c | Direction arrow on each glyph; gap as visible clearance with its value as a label; friction as a hatch with μ as a label | 3 | Nothing. No arrow, clearance or hatch geometry is constructed for supports; searches for drawn gap/friction geometry in `features/viewport` return nothing. | The whole behaviour. | No nearest behaviour. Load arrows (2.4b) are the only arrow geometry, and they are attached to load primitives, not to restraints. |
| 2.4a | Load vectors scaled by magnitude, with the kind carried by the arrow's form, a scale reference in the HUD, and value/unit labels on hover, selection and Labels-on | 3 | `buildLoadArrows` gives every arrow the same fixed shaft and head scale regardless of magnitude (`PipeViewport.tsx:4288-4312`); form never varies; direction is taken only from a global X/Y/Z axis flag (`globalDirectionVector`, `:4349-4362`); colour is a two-way split on whether the dimension string contains `moment` (`:4277-4279`). No label, no HUD scale reference. | Magnitude scaling, per-kind arrow form, HUD scale reference, value/unit labels. | Nearest: the fixed-size two-colour arrow pair. Its instance count is the arrow count and its geometry is constant, so it cannot characterise magnitude-dependent transforms, six arrow forms or per-arrow text. |
| 2.4b | Load vectors drawn from the node and toggleable | 1 | Instanced shaft + head pairs per spatial chunk, anchored to node, pipe midpoint or support node (`PipeViewport.tsx:4258-4322`, `loadAnchor` `:4325-4347`), rebuilt on model/index change (`:1266-1270`) and shown/hidden through `setAuxiliaryVisibility` (`viewportResource.ts:566-571`, `:618-622`) behind the `toggle-viewport-loads` control (`PipeViewport.tsx:2186-2195`). Loads default to on (`:427`), and the frozen fixtures carry 100 and 1,000 nodal force arrows (`apps/desktop/e2e/ui-foundation/fixture-manifest.json`, `counts.nodal_force_arrows`), so arrows are in every measured orbit frame of the existing cohort. | — | — |
| 2.5a | Node labels on a plate, budget-limited, priority-ordered, never overlapping, with a control | 1 | Labels are absolutely positioned DOM buttons in a layer over the canvas (`PipeViewport.tsx:2279-2316`), projected and repositioned after every main render (`:1303-1366`, invoked at `viewportResource.ts:783`), with a screen-space 62 × 30 px rectangle overlap test that hides the later, lower-priority plate (`PipeViewport.tsx:1349-1354`) and a rendered count published to diagnostics (`:1360`, `:1180-1183`). Priority order is primary selection, hover, selected set, diagnostic keys, then spatial context (`apps/desktop/src/features/viewport/viewportSelection.ts:609-643`). Control: `toggle-viewport-labels` (`PipeViewport.tsx:2176-2185`). Instrument: `labels(enabled)` (`apps/desktop/e2e/ui-foundation/characterization-commands.ts:35`), with `enabled`, `renderedCount` and `budget` validated at every boundary and a per-phase label policy (`:53-54`, `:74-75`). | — | — |
| 2.5b | Budget of one label per 3,600 square pixels of canvas; the control cycles All, Budget, Off | 3 | The budget is a fixed count of 80 keys, independent of canvas area (`viewportSelection.ts:610`; published as `budget: 80` at `PipeViewport.tsx:1182`; frozen as `selected_label_cap_candidate: 80` in `apps/desktop/e2e/ui-foundation/fixture-manifest.json`). The control is a two-state boolean (`PipeViewport.tsx:426`, `:2176-2185`). The current row's entity is always included first (`viewportSelection.ts:623`), which the brief also asks for. | Area-proportional budget; the three-state All/Budget/Off control; an "All" mode at all. | Nearest: the fixed cap of 80 with overlap suppression. At 1,000 and 10,000 pipes an area-proportional budget at 1000 × 828 would admit roughly 230 plates against today's 80 cap, and the recorded evidence has only 0–3 plates rendered (`execution/.../D70_BASELINE/BASELINE_CHARACTERIZATION_REPORT.md:13`; `.../HELP-HUMAN-PIPING-20260918-PICKING-STABILITY/RUNTIME_REPORT.md:24`), so measuring today's budget characterises neither the plate count nor the overlap-resolution work of the proposed one. |
| 2.6a | Selection as a 2 px halo around the element's silhouette and its glyphs | 3 | Selection is shown by (i) rewriting the per-instance colour to the theme's selection colour (`viewportResource.ts:262-290`, `:975-986`; light `#a34400`, dark `#f08c22`, `:218-219`) and (ii) a display-only overlay group of centreline `LineSegments` plus 11 px diamond `Points`, drawn with `depthTest: false` at render order 10,000/10,001 (`viewportSelectionPresentation.ts:11-58`, `:60-85`). | A silhouette outline pass; any halo around glyphs; a 2 px screen-space stroke. | Nearest: the recolour-plus-centreline-and-diamond cue above. It replaces the element's colour rather than surrounding it, so it cannot be used to characterise an additive outline pass, and it would conflict with result colour (2.7a) rather than drawing over it. |
| 2.6b | Multiple selection; two-way sync with the tables and the inspector | 1 | Ordered multi-selection flows through one shared state (`PipeViewport.tsx:470-530`, `apps/desktop/src/App.tsx:2712-2741`); canvas picking and box selection publish it (`PipeViewport.tsx:1940-1959`, `:2100-2113`), tree rows select into it, and the inspector follows. Diagnostics expose ordered typed refs for selection, box and inspector (`apps/desktop/src/features/workspace/uiDiagnostics.ts:93-110`). Instrument: 200 frozen point probes, 20 directional box samples, typed tree-row selection by `tree-row-<type>-<id>`, and set sizes 0/1/100/all (`apps/desktop/e2e/ui-foundation/fixture-manifest.json`, `selected_set_sizes`, `box_selection_protocol`; `apps/desktop/e2e/ui-foundation/characterization-commands.ts:29-32`). | — | — |
| 2.6c | Hover as a 1 px halo | 3 | Hover exists only on the DOM label plate: `onPointerEnter`/`onPointerLeave` set `hoveredEntityKey` (`PipeViewport.tsx:2297-2298`), which feeds label priority only (`:531-539`, `viewportSelection.ts:624`). There is no canvas hover hit-test and no 3D hover cue: `handleViewportPointerMove` (`PipeViewport.tsx:1726-1775`) drives routing ghost, box rectangle and pointer capture, never a hover pick. Search: `grep -rniE "hover" apps/desktop/src/features/viewport` returns only these label-plate and CSS hits. | Canvas hover picking and any hover cue on geometry. | Nearest: label-plate hover, which changes label priority only and does not exercise a per-move pick over 1,000–10,000 primitives. |
| 2.6d | Isolate dims everything else to 20 % opacity | 3 | Isolate computes the complement of the selection plus its authored context and hands it to the same hidden-key mask as Hide (`viewportSelection.ts:246-276`, composed at `:219-225`; dispatched at `PipeViewport.tsx:2141-2146`, wired at `App.tsx:2718-2729`). The mask removes geometry: zero-scale instance matrices for instanced meshes, `object.visible = false` otherwise (`viewportResource.ts:322-341`). No opacity path exists for isolate. | Partial-opacity presentation; retaining the dimmed set in the draw. | Nearest: isolate-as-hide, which *removes* the non-context set from the frame. Its frame cost is a reduction, whereas dimming keeps every element drawn and adds a transparent pass; measuring today's isolate would characterise the opposite direction of work. |
| 2.6e | Hide removes the element and the HUD shows the hidden count | 2 | Hide exists and removes geometry (`viewportSelection.ts:235-240`; mask applied at `viewportResource.ts:573-581`, `:322-341`); picking honours the same mask (`viewportResource.ts:624-636`); the status strip shows a hidden count (`PipeViewport.tsx:2244`). The count shown is the number of *selected* items that are hidden (`:489-492`), not the total hidden count. Functional coverage exists (`apps/desktop/e2e/ui-foundation.spec.ts:1302-1355`), but the benchmark command surface has no visibility action: `benchmarkCommands` exposes only `select`, `home`, `camera`, `labels`, `geometry`, `query` (`apps/desktop/e2e/ui-foundation/characterization-commands.ts:27-39`). | A total hidden count in the HUD; any timed or frame-sampled observation of hide. | Add `visibility(action: "hide" \| "isolate" \| "show-all")` to `benchmarkCommands`, driving the existing toolbar buttons (`PipeViewport.tsx:2226-2228`) by real keyboard activation after a typed tree-row selection of known refs, and add a `viewport.visibility { hiddenCount, isolateActive }` field to `UiDiagnosticsSnapshot` (`uiDiagnostics.ts:126-129` is the pattern) so `query()` can witness the mask. Where responsiveness is the subject, time from the real activation event to the next generation-bound render submission. |
| 2.7a | Result colour applied flat per element to tube and fittings | 3 | No result-driven colour exists anywhere. Every drawn colour is a literal: pipes `0x4f6f73` (`PipeViewport.tsx:3747`), nodes `0x2f6f73` (`:3779`), supports `0x6b7d49` (`:3810`), components `0x24705a`/`0x6f5a92`/`0x1f6f73` (`:3842`), deformed geometry `0x0f8f85` (`:3901`, `:3928`), load arrows `0x7b4ea3`/`0xd9822b` (`:4278`). Searches: `grep -rniE "resultColou?r\|stressColou?r\|colou?rByResult\|heatmap" apps/desktop/src` → no matches; `grep -rniE "colou?r ?scale\|colorScale\|sequentialScale\|legend" apps/desktop/src` → only three `<legend>` CSS rules in `styles.css:3432,3448,3680`. The `result` prop reaches the viewport (`PipeViewport.tsx:170`, `App.tsx:2736`) and is consumed only by `buildDeformationOverlay` (`PipeViewport.tsx:578`, `:4400-4498`). | The whole behaviour, including the token set, the seven-anchor scale and the dark-mode anchor flip. | Nearest: the per-instance colour attribute already used for selection (`viewportResource.ts:975-986`). That attribute is the mechanism a flat per-element result colour would use, so it is architecturally available, but today it carries exactly two values (base, selected) and measuring it characterises a two-value rewrite, not a per-element scale evaluation and upload. |
| 2.7b | Result colour interpolated per node along the element for stress | 3 | Each pipe is one instance of a shared unit cylinder with **one height segment** and 10 radial segments (`PipeViewport.tsx:3731`), and instance colour is a single value per instance (`:3747`, `viewportResource.ts:983`). There is no per-vertex colour attribute and no per-element geometry. | Longitudinal tessellation and per-vertex or shader-side interpolation. | Nearest: per-instance flat colour, which by construction cannot vary along an element. Measuring it answers nothing about the instanced path's support for the per-node case; §4 question 1 asks exactly this, and the source answer is "not as instanced today" (see §4). |
| 2.7c | Unsolved elements in the unsolved token at 60 % opacity; edge lines over colour; halos over colour unchanged | 3 | No unsolved/solved distinction is drawn. The shared pipe material is opaque (`PipeViewport.tsx:3732`); the only transparent product materials are the deformed overlay (`:3887`, `:3920`), the route ghost and grids (`:3954`, `:3970`, `:3996`) and the selection cue (`viewportSelectionPresentation.ts:17,20`). | The whole behaviour; also depends on 2.1c and 2.6a. | Nearest: the deformed-overlay transparency at 0.82/0.86, applied to a different layer for a different purpose. |
| 2.7d | A legend in the canvas carrying the scale, units, rule ID and pack version | 3 | The only in-canvas readout is `viewport-scale-bar`, which renders a single length quantity of value 1 in the project length unit (`PipeViewport.tsx:2343-2345`). Search for a canvas legend: see 2.7a. | The whole behaviour. | Nearest: the one-quantity scale bar. It has no scale ramp, no rule identity and no pack version, and its layout cost is one DOM span. |
| 2.7e | A probe at the cursor reads the values | 3 | The cursor reads nothing after a run. The Measure tool reads authored geometry only: it takes two authored nodes or one authored pipe as targets and reports a length (`PipeViewport.tsx:433-440`, `:605-627`, `:2119-2133`, readout at `:2432-2440`), never a result row. Search: `grep -rniE "probe\|hoverValue\|cursorValue" apps/desktop/src/features/viewport apps/desktop/src/features/results` → no matches. | The whole behaviour. | Nearest: the Measure tool. It is target-based, not cursor-based, requires an explicit two-target selection, and reads authored coordinates rather than result values. |
| 2.8a | The deformed shape drawn solid | 2 | Drawn when a result carries displacement rows: instanced deformed tubes plus instanced deformed node markers, replaced into `resultLayer` (`PipeViewport.tsx:1252-1264`, `:3866-3906`, `:3908-3933`), built by `buildDeformationOverlay` from `displacement_magnitude` rows with signed global `ux/uy/uz` components (`:4400-4498`), with state and boundary reported in the toolbar (`:2166-2174`). Functional coverage exists at five nodes (`apps/desktop/e2e/r2-smoke.spec.ts:570-580`). The instrument cannot reach it: nothing in `e2e/ui-foundation/` runs a solve or supplies a result, and the frozen `*.deformation-overlay.json` fixtures are written by `generate-fixtures.mjs:231-245`, listed in `fixture-manifest.json` with hashes, and **loaded by nothing** — `grep -rn "deformation-overlay\|deformationOverlay" apps/desktop/e2e apps/desktop/src` returns only the generator and the manifest. They are a fixture, not a rendered behaviour. | Any instrument route that puts the result layer on screen. | Add a named `deformation()` setup that drives the real product path — open the Solve section and activate `run-mechanics-preview`, then wait on the existing `viewport-deformation-status` / `viewport-deformation-summary` test IDs (`PipeViewport.tsx:2169-2173`) for `available` — and add `viewport.deformation { state, nodeCount }` to `UiDiagnosticsSnapshot` so `query()` and `validateBoundaryMetadata` can bind it. Then run the existing 2 s warm-up / 10 s orbit window with the result layer populated. Uncertainty: whether the in-product preview solves the 1,000/10,000-pipe fixtures at all (see §5). |
| 2.8b | The undeformed shape as a 1 px dashed ghost | 3 | The undeformed model stays in `modelLayer` as solid opaque tubes while the deformed overlay is added to `resultLayer` (`PipeViewport.tsx:1226-1264`); no dashed or ghosted variant of the authored geometry is constructed. The only dashed material in the product is the single route-ghost line (`:3991-4000`). | A dashed, thin, ghosted rendering of the authored geometry. | Nearest: the solid undeformed geometry, which is simply the ordinary model draw; measuring it measures the baseline model, not a second ghost pass. |
| 2.8c | Scale factor stated in the legend, adjustable by stepper and field, defaulting to max displacement = one tenth of model extent, rounded | 3 | The offset is a hard-coded normalized display offset of 0.65 local units applied along the unit displacement direction (`PipeViewport.tsx:4469-4485`), and the product states in its own boundary string that this is `scale=normalized_display_offset_not_physical_length` (`:4495`). There is no stepper, no field, no legend entry and no extent-derived default. | A physical scale factor, its default rule, and its controls. | Nearest: the fixed normalized offset. Changing it is not possible through any control, so "cost of changing the scale factor" has no existing analogue; rebuilding the overlay happens only when `model` or `result` changes (`:1264`). |
| 2.8d | Animation: a play control oscillating the factor from zero to the stated value, one case at a time | 3 | Nothing. No animation loop, play control or time-varying factor exists; the invalidation scheduler renders on demand and stops (`viewportResource.ts:36-46`, `:672-675`). Search: `grep -rniE "animat\|oscillat\|playback" apps/desktop/src/features/viewport` returns no animation driver. | The whole behaviour. | Nearest: continuous orbit, which is the only sustained multi-frame workload the product produces today. Orbit re-renders with a static scene graph, whereas animation would rewrite instance matrices every frame; orbit frame intervals therefore cannot characterise animation cost. |
| 2.9a | A pending agent proposal drawn as dashed geometry at 60 % opacity beside the current one; removed elements as dashed outlines; ghost edge on the affected node's label plate; accepting turns the ghost into the drawing | 3 | Proposals are DOM only. `AgentProposalPanel` renders text facts and lists (`apps/desktop/src/features/agent-proposals/AgentProposalPanel.tsx:1-156`); `DiffPreviewPanel` renders a textual diff (`apps/desktop/src/features/diff-preview/DiffPreviewPanel.tsx`); `DesignWorkspacePanel` is unrelated to the canvas. Search: `grep -rn "viewport\|THREE\|ghost" apps/desktop/src/features/agent-proposals apps/desktop/src/features/diff-preview apps/desktop/src/features/design-workspace` → no matches. Queued intents reach the viewport only as a DOM list (`PipeViewport.tsx:587-588`, `:2424-2426`). | The whole behaviour, at any count. | Nearest: the single route-draft ghost (2.9b), which is one dashed centreline for one draft. Two hundred ghosted elements is a different order and a different construction; measuring one line says nothing about it. |
| 2.9b | The engineer's own routing draft as a second ghost, in the draft token, never the same colour | 2 | A routing draft ghost exists: a dashed `THREE.Line` with `LineDashedMaterial` plus an orange sphere marker and a construction grid, replaced into `routingLayer` and updated on pointer move (`PipeViewport.tsx:1278-1301`, `:3991-4017`, `:3945-3959`), gated by the armed Pipe tool and a resolved From node (`:629-661`). No instrument command arms the Pipe tool or moves a draft pointer (`characterization-commands.ts:27-39`). | Distinct colour: the ghost uses `0xf08c22` (`PipeViewport.tsx:3996`), which is exactly the dark-theme selection colour `SELECTED_COLOR_DARK` (`viewportResource.ts:219`) — the brief requires a draft token that is never the same colour. Also: one line, not ghosted geometry at 60 % opacity. | Add a `routeDraft(fromRef: TypedIdentity, plane, axis)` setup that activates the real `command-pipe` control (`PipeViewport.tsx:2361-2372`), chooses a typed From node through the existing picker, and moves a real pointer across the canvas; expose `viewport.routing { ghostPresent, plane, axis }` in diagnostics so `query()` witnesses it. Stimulus must be real pointer movement, since per-move ghost update is the subject. |
| 2.10a | Camera presets Iso, Top, Front | 1 | `ViewPreset = "iso" \| "front" \| "top"` (`PipeViewport.tsx:180`), buttons at `:2332-2342`, applied by `fitViewportCamera` (`:3022-3050`). Instrument: `camera()` clicks `viewport-view-isometric` then `viewport-fit-model` (`characterization-commands.ts:34`), with camera position/target/up/fov/near/far/aspect validated at every boundary (`:57-58`) and read back before timing (`benchmark-harness.ts`, `validateCandidateMeasuredCameraBinding`). | — | — |
| 2.10b | A Right preset | 3 | Absent from the preset union (`PipeViewport.tsx:180`) and from the button group (`:2332-2342`); `fitViewportCamera` has branches for front, top and iso only (`:3037-3044`). | The preset. | Nearest: Front, which is `+Z` at the fitted distance (`:3038`). A `+X` view would exercise the same code path with different constants, so cost is not the issue; the item is a missing control, not a missing capability. |
| 2.10c | Fit, of all or of the selection | 1 | Three commands: Fit Model, Fit Visible, Fit Selection, each computing displayed bounds over the chosen key set with the visibility mask and Actual-OD radii (`PipeViewport.tsx:2152-2158`, `:2076-2089`, `displayedBoundsForEntityKeys` in `viewportSelection.ts`); buttons at `:2229-2231`. Instrument uses Fit Model as part of the frozen camera recipe (`characterization-commands.ts:34`; `fixture-manifest.json`, `cameras.candidate_standard_command_preflight`). | — | — |
| 2.10d | A named "Report figure" camera saved with the project, and a report that prints the figure exactly as the canvas draws it at that preset | 3 | No camera is persisted. `cameraStateRef` is written on every camera change (`PipeViewport.tsx:885-890`) and **never read** — `grep -n "cameraStateRef" apps/desktop/src/features/viewport/PipeViewport.tsx` returns only the declaration at `:375` and that write. No camera field appears in the project document or in any report path. The instrument's own frozen record states the baseline product exposes no camera readback (`apps/desktop/e2e/ui-foundation/fixture-manifest.json`, `cameras.baseline_reachable_recipe.exact_state_observable: false`). | Persistence, naming, and any report binding. | Nearest: the live `OrbitControls` pose, which survives only while the `ViewportResource` lives (`viewportResource.ts:411-413`) and is re-fitted whenever the project session generation changes (`PipeViewport.tsx:1415-1422`). It cannot be saved, named or reproduced in a report. |
| 2.10e | The triad states the up axis | 1 | A separate gizmo scene with an `AxesHelper` and three canvas-texture sprite labels is rendered into a scissored corner after every main render (`viewportResource.ts:419-427`, `:789-800`, `renderGizmo`), repainted on theme change (`:296-320`), hosted at `PipeViewport.tsx:2329-2331`. Instrument: the camera `up` vector is recorded and validated at every boundary (`characterization-commands.ts:57`). | — | — |

### 1.2 Rendering brief §3 — what the tables need from the canvas

| # | Item | Class | What exists today (file:line) | What is missing | Class 2 adaptation / Class 3 nearest |
|---|---|---|---|---|---|
| 3.1a | Element selection selects the row, within one frame | 1 | Canvas pick → ordered selection publication → tree and inspector (`PipeViewport.tsx:1812-1915`, `:1940-1959`; `App.tsx:2726-2738`). Measured directly: 200 real-pointer point actions per session against a frozen independent oracle, with the end condition requiring the expected ordered selection, viewport feedback and property identity plus a browser-observed capture (`apps/desktop/e2e/ui-foundation/fixture-manifest.json`, `publication_timing.point_end`; targets at `apps/desktop/e2e/ui-foundation/performance-targets.ts:63-64`). Recorded p95 42.036 ms at N=10,000 and 46.699 ms at N=1,000 against a 100 ms target (`.../PICKING-STABILITY/RUNTIME_REPORT.md:12`; `.../D70_BASELINE/BASELINE_CHARACTERIZATION_REPORT.md:27`). | — | — |
| 3.1b | Row selection highlights the element, within one frame | 2 | The path exists and is exercised: `commands.select(typed identity)` clicks `tree-row-<type>-<id>` (`characterization-commands.ts:29-32`, `benchmark-harness.ts` `treeRowTestId`), and the product publishes an `external` selection that re-applies the instanced cue (`PipeViewport.tsx:515-530`, `:1368-1371`). But the tree-row activation is deliberately **outside** the timer: it is the per-point reset, "recorded separately and excluded from point timing" (`apps/desktop/e2e/ui-foundation/README.md`; `fixture-manifest.json`, `point_hit_protocol.per_point_reset`). No timed metric exists for this direction. | A timed row → element metric. | Promote the existing untimed reset into a timed action class using the same causal witness the point lane uses: real keyboard or pointer activation of `tree-row-<type>-<id>` for a frozen sample of typed refs, start at the capture-listener observation of the input event, end when `viewport.selection` carries the expected ordered refs on a new action- and generation-associated render submission (`uiDiagnostics.ts:93-110`), with the browser-observed capture as the conservative upper bound. No product change is required. |
| 3.2a | Picking resolves to the node or element the row grammar names, including glyphs | 1 | Analytic point picking over frozen primitives — capsule for pipes, sphere for nodes, cone for supports, sphere for components — with a node-before-pipe tie rule and the visibility mask applied (`viewportSelection.ts:459-534`, `pickPointPrimitive` `:536+`; entry at `viewportResource.ts:624-636`). Independently checked by an offline V3 oracle over 200 probes per fixture (`apps/desktop/e2e/ui-foundation/point-hit-oracle.mjs`, `fixture-manifest.json` `point_hit_protocol`). Supports are the only "glyph" drawn, and they are picked. | — | — |
| 3.2b | Picking resolves to label plates | 2 | Label plates are real `<button>` controls carrying `data-entity-key` and a click handler that selects the entity (`PipeViewport.tsx:2286-2313`, `chooseViewportTarget` `:1637-1658`), publishing an input kind of `label` (`:500`). They are not part of the 3D pick (`pointPickPrimitives` covers pipes, nodes, supports, components only, `viewportSelection.ts:459-534`). No timed sample exercises them, because labels are forced OFF for every timed point action (`fixture-manifest.json`, `point_hit_protocol.labels_state_during_point_actions`; enforced at `characterization-commands.ts:74`). | A labels-on selection sample class. | Add a small labels-on point sample set whose expectations come from the plates' own `data-entity-key` after a projection query, activated by a real pointer click on the plate, and relax the phase label policy at `characterization-commands.ts:74` for that phase only. Keep the existing labels-off point population untouched so the frozen oracle and p95 are unaffected. |
| 3.3 | The probe reads per-element and per-node values at the cursor after a run | 3 | See 2.7e: no result probe exists; the Measure tool reads authored length for explicitly chosen targets (`PipeViewport.tsx:2119-2133`, `:2432-2456`). | The whole behaviour. | Nearest: the Measure tool, which is target-driven and authored-geometry-only. Measuring it characterises a two-node distance conversion, not a per-frame cursor query against a result set. |
| 3.4a | A pointer-placed draft along a constrained axis in a construction plane | 2 | Exists: plane select (XY/XZ/YZ) and axis radios (Free/X/Y/Z) with applicability rules (`PipeViewport.tsx:2615-2660`, `viewportRouting.ts` `applicableRoutingAxes`, `constrainRoutingPoint`), a raycast onto the plane (`PipeViewport.tsx:1989-2033`, `raycastDraftPoint` `:3278-3297`), a live ghost and grid (`:1278-1301`), a frozen draft review and apply gate (`routeDraft.ts`, `App.tsx:1347-1520`). | No instrument driver; see 2.9b. | Same adaptation as 2.9b, extended with a `commitDraft()` action that activates the real review/apply controls, so authoring latency can be timed from the real pointer event to the committed model generation. |
| 3.4b | A routing compass showing a ghost along the current axis **as a length is typed**, committing a row on **Enter**, with **Tab** cycling the axis | 3 | Typed authoring is coordinate-based, not distance-based: X/Y/Z fields for the new endpoint (`PipeViewport.tsx:2664-2668`), and the product states "Typed coordinates are authoritative. Pointer placement is only a draft aid." (`:2617`). Searches: `grep -rn '"Tab"\|key === .Tab' apps/desktop/src/features/viewport` → no matches; `grep -rn '"Enter"' apps/desktop/src/features/viewport` → no matches. No compass widget exists. | Distance entry, the Enter commit, the Tab axis cycle, and the compass itself. | Nearest: the plane/axis radio group plus typed coordinates (3.4a). It is a different grammar with different keystrokes, so its interaction latency would not characterise typed-distance authoring. |
| 3.5 | The table drawer in Model view resizes without a canvas relayout stall; the split in Both view drags live | 3 | Neither surface exists. There is no stage (Model/Loads/Results/Review) and no Table/Model/Both view: the workspace is a fixed three-pane grid, tree \| viewport \| inspector (`styles.css:497-535`; `App.tsx:2659-2790`), and the viewport is always mounted. There is no table drawer. Note also that `--pane-agent-col` and `.workspace-pane-agent` exist in CSS only (`styles.css:501,504,533`); `grep -rn "workspace-pane-agent" apps/desktop/src` returns no TSX match, so the agent column the brief's §1 assumes is not rendered. | Stages, the three views, per-stage view memory, the table drawer, the Both split. | Nearest: the tree and inspector splitters, which drag live (`App.tsx:2743-2756`, `beginWorkspaceResize`), plus the canvas resize path — a `ResizeObserver` that calls `renderer.setSize` and re-derives aspect (`viewportResource.ts:376`, `:443`, `:821-831`) and a reserve calculator that observes only chrome rows (`workspace/workspaceCanvasBudget.ts:1-60`). Measuring a tree-rail drag would characterise a horizontal pane resize beside an unchanged table, not a drawer opening beneath the canvas or a live split against a 737-wide table. |

### 1.3 Rendering brief §4 — the eight proposed observations

| # | Observation | Class | What exists today (file:line) | What is missing | Class 2 adaptation / Class 3 nearest |
|---|---|---|---|---|---|
| 4.1 | Result colour on every element with edge lines and labels at budget — frame time steady-state; time from run completion to first coloured paint; memory delta | 3 | None of the three subjects exists: no result colour (2.7a), no edge lines (2.1c). Labels at budget do exist and are measured (2.5a). Memory is not recorded in the qualified lane: `processMemory: "UNAVAILABLE_NOT_MEASURED", heap: "UNAVAILABLE_NOT_MEASURED"` (`apps/desktop/e2e/ui-foundation/full-cohort-controller.ts:905-906`), confirmed in evidence (`.../PICKING-STABILITY/RUNTIME_REPORT.md:34`; `.../D70_BASELINE/BASELINE_CHARACTERIZATION_REPORT.md:41`). | Result colour, edge lines, a run-completion event to time from, and a memory series in the qualified lane. | Nearest: the existing labels-on orbit window, which measures frame intervals over an uncoloured, edgeless scene. A heap/RSS delta does exist in the separately labelled trace-off feasibility mode (`apps/desktop/e2e/ui-foundation/baseline-canvas-point.benchmark.ts:176-179`; `benchmark-harness.ts:1571-1598`), but feasibility rows are never aggregated as qualification samples (`apps/desktop/e2e/ui-foundation/README.md`). Measuring today's orbit would characterise the *absence* of the proposed work. |
| 4.2 | Per-node interpolated stress colour along elements — whether the instanced path supports it; frame time | 3 | See 2.7b: one instance per pipe, one height segment (`PipeViewport.tsx:3731`), one colour per instance (`viewportResource.ts:983`). | Longitudinal tessellation, a per-vertex colour attribute or a shader that can vary along the instance. | Nearest: per-instance flat colour. The source answer to "whether the instanced path supports it" is in §4 below; no measurement is required to establish it, and a frame time for flat colour would not bound the interpolated case. |
| 4.3 | Deformation animation at the default scale — frame rate over ten seconds; cost of changing the scale factor | 3 | No animation and no adjustable factor (2.8c, 2.8d). The static deformed overlay does exist but is unreachable by the instrument (2.8a). | The animation, the factor control, and a deformation-bearing benchmark route. | Nearest: the existing 2 s warm-up / 10 s measured orbit window (`performance-targets.ts:39-53`; `full-cohort-controller.ts:795-831`), which has exactly the right shape for a ten-second frame-rate observation but drives camera motion over a static scene graph. Animation rewrites instance matrices each frame; orbit does not, so orbit p95 cannot stand in. If 2.8a's adaptation lands, an orbit window *over the static deformed overlay* becomes available and is worth recording as its own row — it is not the animation. |
| 4.4 | Two hundred elements ghosted as a pending proposal — frame time with ghosts; time to accept | 3 | No proposal ghosts at any count (2.9a). | The whole behaviour. | Nearest: the single route-draft ghost (2.9b) and the existing frozen-draft apply path (`App.tsx:1400-1520`), which commits one operation and a new model generation. Timing one draft apply would characterise a single-row commit, not 200 ghosts becoming the drawing. |
| 4.5 | Label budget recompute on continuous camera motion — frame time during orbit; confirmation that no two labels overlap at rest | 2 | The frame-time half is already measured exactly as written: labels are ON for both orbit windows (`characterization-commands.ts:74-75`; `performance-targets.ts:44`), the label updater runs after **every** main render (`viewportResource.ts:783`; registered at `PipeViewport.tsx:1362`), and the orbit gesture is a real canvas pointer path (`full-cohort-controller.ts:809-827`). Recorded: centreline and Actual-OD reported-presentation interval p95 8.335 ms at N=10,000 (`.../PICKING-STABILITY/RUNTIME_REPORT.md:15-16`) and 16.668 ms at N=1,000 (`.../D70_BASELINE/BASELINE_CHARACTERIZATION_REPORT.md:30-31`). Boundary validation checks `renderedCount ≤ budget` and the labels-on policy (`characterization-commands.ts:53-54`, `:74-75`). | A non-overlap confirmation. The product *enforces* non-overlap by hiding the lower-priority plate (`PipeViewport.tsx:1349-1354`), and diagnostics publish only the count, not the placements. | Add a `labelPlacements` query: read every `[data-entity-key]` plate's `getBoundingClientRect()` inside the selection layer at rest and assert pairwise non-intersection, alongside the existing `renderedCount`. This is a read-only DOM query in the harness, needing no product change; optionally publish the placement rectangles in `uiDiagnostics` for a frozen witness. Note the recorded rendered counts are 0–3 plates, so a meaningful confirmation also needs a camera pose that brings many plates into frustum. |
| 4.6 | Selection halo on five hundred elements — frame time; pick latency to the table | 2 | Pick latency to the table is the existing point metric (3.1a). Large selections are supported and partly frozen: set sizes 0/1/100/all are required on the 10,000 fixture (`fixture-manifest.json`, `selected_set_sizes` and `selection_set_sizes_required_on_10000_fixture`), the selection cue compacts dynamic buffers over the selected set each update (`viewportSelectionPresentation.ts:60-85`), and the instanced recolour walks every instance on every selection change (`viewportResource.ts:975-986`). | No 500-element sample, and no frame-time observation taken *while* a large selection is held. The halo itself does not exist (2.6a); this row measures the existing cue. | Add a box-selection sample whose frozen oracle yields 500 typed refs, then run the existing orbit window with that selection retained, and record `rendererInfo.calls/triangles` (`uiDiagnostics.ts:130-135`) at the window boundaries. Stimulus stays a real primary-pointer canvas drag under the existing box protocol (`fixture-manifest.json`, `box_selection_protocol`). Word the result as the cost of today's recolour-plus-overlay cue, not of a silhouette halo. |
| 4.7 | Theme switch with results shown — time to full repaint | 3 | Results are never shown (2.7a), so the observation as written has no subject. The theme switch itself exists but repaints very little of the canvas: `setThemePresentation` changes the scene background, repaints the gizmo axes and labels, and re-applies the selection colour (`viewportResource.ts:553-564`, `:292-320`). `applySelectionPresentation` rewrites non-selected instances back to their *unchanged* base colour (`:280`, `:983`), so pipes, nodes, supports, components, load arrows, grids and the DOM label plates do not change with the theme — the plate's colours are literals in CSS with no dark override (`styles.css:1506-1549`; the dark block at `:3576-3585` does not touch `.viewport-select-target`). | A token-driven canvas palette; results; any repaint timing. | Nearest: the existing theme switch. A driver exists outside the benchmark lane — `setAppearance(page, theme, density)` (`apps/desktop/e2e/ui-foundation-workflows.ts:160-171`) — and the benchmark records `theme` at every boundary but pins it (`characterization-commands.ts:60`, `:95`). Timing today's switch would characterise a background-plus-gizmo repaint of a scene whose element colours are unaffected; it sets no bound on a full token repaint. |
| 4.8 | Canvas at 603 × 828 and 1000 × 828, DPR 1 and 2 — whether the edge line stays one pixel and glyphs stay fixed-size at both | 3 | Neither subject exists: no edge line (2.1c), no fixed-screen-size glyph (2.3b). Separately, the two proposed canvas sizes and DPR 1 are outside the instrument's frozen profile: `validateBoundaryMetadata` requires `p.browserDpr !== 2` to fail, `windowWidth === 1440`, `windowHeight === 920` and `c.dpr <= 2` (`characterization-commands.ts:50-53`), and every recorded boundary was 794 × 557 CSS / 1588 × 1114 buffer at DPR 2 (`.../PICKING-STABILITY/RUNTIME_REPORT.md:24`; `.../D70_BASELINE/BASELINE_CHARACTERIZATION_REPORT.md:13`). The renderer sets its pixel ratio once at construction, capped at 2 (`viewportResource.ts:407`), and resize changes size and aspect only (`:821-831`). | Edge lines, fixed-size glyphs, and a profile that admits a second DPR and the two canvas sizes. | Nearest: the canvas/DPR recording itself, which is complete and validated but frozen at one profile. Even after the two behaviours exist, this observation needs a profile change: a second frozen boundary profile at DPR 1 and the two canvas geometries, which by construction cannot share the current profile's bound `referenceProfileSha256` (`characterization-commands.ts:63`, `:105-107`). |

---

## 2. Semantic differences an implementer must treat as a change, not a restyle

1. **Isolate dims versus isolate hides.** The brief: "Isolate dims everything else to 20 %
   opacity" (`RENDERING_BRIEF_2026-09-18.md:21`). The product composes the isolate complement
   into the same hidden-key mask as Hide (`viewportSelection.ts:246-276`, `:219-225`), and the
   mask zeroes instance matrices or clears `visible` (`viewportResource.ts:322-341`). It also
   changes what can be picked, because picking honours the mask (`viewportResource.ts:624-636`,
   `hiddenKeys` passed to `pickPointPrimitive`). Dimmed elements would remain drawn **and**
   would have to be either pickable or explicitly excluded — a behavioural decision, not a
   token change, and one that inverts the direction of the frame-cost effect.

2. **Selection replaces colour versus selection adds a halo.** The brief: "Selection is a 2 px
   halo around the element's silhouette … halos draw over colour unchanged"
   (`RENDERING_BRIEF_2026-09-18.md:21`, `:22`). The product overwrites the element's own
   instance colour with the selection colour (`viewportResource.ts:262-290`, `:975-986`). Once
   result colour exists, the current mechanism destroys the value it is drawn over; an additive
   outline pass is required, not a different selection token.

3. **Hidden count means two different things.** The brief: "Hide removes it and the HUD shows
   the hidden count" (`:21`). The product's status strip shows the number of *selected* items
   that are hidden (`PipeViewport.tsx:489-492`, `:2244`), which is zero whenever the selection
   is cleared after hiding. A total hidden count is a new published quantity.

4. **Deformation offset is normalized display, not a scale factor.** The brief asks for a
   stated, adjustable scale factor with an extent-derived default (`:23`). The product applies
   a fixed 0.65 local-unit normalized offset and publishes its own boundary string
   `scale=normalized_display_offset_not_physical_length` (`PipeViewport.tsx:4469-4485`,
   `:4495`). Introducing a real factor changes what the overlay *means*, not how it looks, and
   the existing boundary wording would have to change with it.

5. **Undeformed geometry is the model, not a ghost.** The brief asks for the undeformed shape
   as a 1 px dashed ghost beside the deformed solid (`:23`). Today both are drawn solid in
   separate layers (`PipeViewport.tsx:1226-1264`). Ghosting the authored geometry while a
   result is shown is a change to what the model layer draws in a given mode, with consequences
   for picking (the authored geometry is what `pointPickPrimitives` targets,
   `viewportSelection.ts:459-534`).

6. **The draft ghost currently uses the selection colour.** The brief requires the draft ghost
   to be "in the draft token, never the same colour" as the proposal ghost (`:24`). The route
   ghost is `0xf08c22` (`PipeViewport.tsx:3996`), identical to `SELECTED_COLOR_DARK`
   (`viewportResource.ts:219`). Any token assignment must also separate draft from selection.

7. **Camera state is not persisted, and there is no view to persist it across.** The brief
   requires camera, selection, label mode, legend and deformation scale to survive stage and
   view switches, and a named report camera saved with the project (`:8`, `:25`). The product
   has no stage or view model at all (`styles.css:497-535`; `App.tsx:2659-2790`), and its
   camera record is written and never read (`PipeViewport.tsx:375`, `:885-890`). The camera is
   additionally re-fitted whenever the project session generation changes (`:1415-1422`).
   Persisting and naming a camera is new state, new storage in the project document, and a new
   report binding.

8. **Label mode is binary and its budget is a count.** The brief specifies a three-state
   All/Budget/Off control and an area-proportional budget (`:20`). The product has a boolean
   toggle (`PipeViewport.tsx:426`, `:2176-2185`) and a fixed cap of 80 (`viewportSelection.ts:610`),
   which is frozen into the instrument's boundary validation and fixture manifest
   (`characterization-commands.ts:53-54`; `fixture-manifest.json` `selected_label_cap_candidate`).
   Changing the budget rule changes an evidence binding, not only the UI.

9. **Labels are DOM, not canvas.** Plates are HTML buttons in an overlay layer
   (`PipeViewport.tsx:2279-2316`), positioned after each render (`:1323-1361`). They are
   therefore outside the 3D pick, outside the theme repaint (`styles.css:1506-1549` has no dark
   override), and outside anything a report would print from the canvas. "The report prints the
   figure exactly as the canvas draws it" (`:25`) cannot hold for DOM plates as currently built.

10. **Actual OD is a mode, not the default.** The brief's pipe draws at real outside diameter
    (`:16`). The product defaults to schematic fixed-radius tubes and resets to schematic on
    every new project session (`PipeViewport.tsx:441`, `:1395-1401`), with Actual OD requiring
    an asynchronous unit-service conversion that can report `partial-unavailable` or
    `unavailable` (`:746-800`). Making real OD the resting state changes the startup path and
    the failure surface, not the styling.

---

## 3. What the existing instrument records per run, and what it cannot record

### 3.1 Recorded

- **Timed action metrics**, all as caller-qualified duration intervals with a nearest-rank p95:
  model assignment to usable presentation; 200 point selections; 20 directional box selections;
  20 tree filters; two orbit windows (centreline and Actual OD), each 2 s warm-up plus 10 s
  measured (`apps/desktop/e2e/ui-foundation/performance-targets.ts:55-67`;
  `full-cohort-controller.ts:654-831`, `:891-895`). Targets:
  `{assignment: 2000, pointP95: 100, boxP95: 200, filterP95: 200, centerlineP95: 16.7, actualOdP95: 33.3}` ms
  (`performance-targets.ts:66-67`).
- **Orbit presentation evidence**: every qualified consecutive Chromium-reported presentation
  gap in the conservative bracketed envelope, with source occurrence indexes, endpoint
  intervals and the same-trace integer-microsecond duration basis bound to one finalized,
  hash-checked capture (`performance-targets.ts:19-53`; `full-cohort-controller.ts:203-259`).
  The evidence reports are explicit that this is a Chromium-reported metric and not physical
  scanout (`.../PICKING-STABILITY/RUNTIME_REPORT.md:32`).
- **Boundary metadata at every stopped boundary**: canvas CSS and drawing-buffer size, DPR,
  canvas origin; camera kind, sequence, position, target, up, fov, near, far, aspect and local
  render origin; model generation and identity hash; label enabled/rendered/budget; geometry
  mode, OD generation and OD status; theme, density, window size, browser DPR; the two named
  panes' and panels' geometry; a reference-profile hash; and drift detection against a previous
  boundary (`characterization-commands.ts:47-77`, `:90-109`). Each record carries its own
  limitation string: "boundary snapshot only … no continuous foreground/display monitoring"
  (`:107`).
- **Renderer and ownership counters** as boundary diagnostics: geometries, textures, calls,
  triangles, points, lines, app-owned pending RAF count, and the owned-resource ledger
  (`apps/desktop/src/features/workspace/uiDiagnostics.ts:9-30`, `:130-140`;
  `viewportResource.ts:988-996`). The controller labels these
  `BOUNDARY_DIAGNOSTIC_COUNTS_ONLY_NOT_FULL_RESOURCE_PROOF`
  (`full-cohort-controller.ts:905-906`).
- **Trace extraction**: an explicitly bounded CDP trace requiring exact ordered markers and a
  `tracingComplete` payload with `dataLossOccurred === false`, plus offline extraction of
  attributable `ProxyMain::BeginMainFrame` occurrences with known and unknown-phase counts
  (`apps/desktop/e2e/ui-foundation/chromium-compositor-trace.ts`;
  `apps/desktop/e2e/ui-foundation/README.md`; results at `.../PICKING-STABILITY/RUNTIME_REPORT.md:30`).
  These trace spans are not complete frame cost and are not hardware GPU execution time
  (`RUNTIME_REPORT.md:30-32`).
- **Display and environment custody**: a macOS display profile with exact reported refresh mode
  and mirroring state, captured pre and post, with the limitation "pre/post host boundary only;
  no window-to-display or continuous-monitoring proof" (`characterization-commands.ts:194-214`);
  pinned Chromium, source, build, fixture, oracle, method and sample hashes
  (`performance-targets.ts:10-17`).
- **Untimed control-path smoke** covering assignment, a point pick, a box drag, two filters,
  both geometry modes and an orbit in each, with the explicit claim "control-path smoke only;
  no timing, trace or performance score" (`characterization-commands.ts:129-192`).

### 3.2 Not recordable by the instrument as it stands

Listed explicitly, as the brief requires.

- **Memory delta of any kind in the qualified lane.** `processMemory: "UNAVAILABLE_NOT_MEASURED"`,
  `heap: "UNAVAILABLE_NOT_MEASURED"` (`full-cohort-controller.ts:905-906`), confirmed in both
  evidence reports (`.../PICKING-STABILITY/RUNTIME_REPORT.md:34`;
  `.../D70_BASELINE/BASELINE_CHARACTERIZATION_REPORT.md:41`). JavaScript used-heap and
  Chromium-reported process RSS deltas exist only in the separately labelled trace-off
  feasibility mode (`baseline-canvas-point.benchmark.ts:176-179`; `benchmark-harness.ts:1571-1598`),
  whose rows "are never aggregated as qualification samples"
  (`apps/desktop/e2e/ui-foundation/README.md`), and RSS carries its own attribution limitation
  (`benchmark-harness.ts:1580`).
- **Time to first coloured paint, or time from run completion to any paint.** No result run is
  performed in the instrument, and no metric is defined from a solve completion; the defined
  starts are assignment entry, pointerdown for points and boxes, and the tree input event for
  filters (`fixture-manifest.json`, `publication_timing`).
- **Physical scanout, or any display-side presentation fact.** Chromium presentation feedback
  is not physical scanout and frame callbacks are paint opportunities only
  (`apps/desktop/e2e/ui-foundation/README.md`; `RUNTIME_REPORT.md:32`).
- **Hardware GPU execution time.** GPU-thread trace context is explicitly unbound same-window
  context, not a causal per-frame join (`RUNTIME_REPORT.md:32`).
- **Continuous monitoring of anything.** Every profile, resource and geometry record is a
  boundary snapshot (`characterization-commands.ts:107`, `:213`;
  `BASELINE_CHARACTERIZATION_REPORT.md:13`).
- **Native startup, and cold process-to-window time.** `startup: "UNAVAILABLE_NATIVE_STARTUP_NOT_MEASURED"`
  (`full-cohort-controller.ts:904`); `BASELINE_CHARACTERIZATION_REPORT.md:41`.
- **Uninstrumented cost.** `trueCost: "UNPROVED_NO_UNINSTRUMENTED_HEADROOM_CLAIM"`
  (`full-cohort-controller.ts:906`); wrapper, marker, diagnostics, DOM and trace costs remain
  in the workload and nothing is subtracted (`apps/desktop/e2e/ui-foundation/README.md`).
- **Any behaviour with no named command.** The benchmark command surface is exactly
  `select`, `home`, `camera`, `labels`, `geometry`, `query`
  (`characterization-commands.ts:27-39`). Hide, Isolate, Show All, Fit Visible, Fit Selection,
  Front, Top, Measure, Box-Select filter beyond the frozen samples, the creation tools, theme
  and density are not in it. Theme and density are read and pinned, never switched, in this
  lane (`:50-53`, `:60`, `:95`).
- **A second display profile, a second DPR, or a different window geometry.** The boundary
  validator rejects anything but `browserDpr === 2`, `1440 × 920`, and a matching
  `referenceProfileSha256` (`characterization-commands.ts:50-53`, `:63`), and the display
  profile validator requires one exact main online display identity
  (`:194-206`).
- **Comparison between the 1,000-pipe and 10,000-pipe records.** Different display, product and
  method; both reports refuse the comparison (`BASELINE_CHARACTERIZATION_REPORT.md:11`;
  `RUNTIME_REPORT.md:26`). No speedup or scaling statement is made here either.

---

## 4. Answers to the rendering brief §4's two questions, from source only

### 4.1 "Which of the behaviours in §2 the current architecture supports as overlays on instanced meshes without per-element meshes, and for each that it does not, the nearest supportable alternative and its cost."

Answered from source; no runtime probe is needed for the support question itself, and every
cost figure is marked TBD.

| Named sub-question | Source answer | Cost |
|---|---|---|
| **Edge lines** | **Not supported today, and not as a property of the existing instanced draw.** There is no edge pass at all (2.1c). The instanced tube is a shared unit cylinder with 10 radial segments and no edge geometry (`PipeViewport.tsx:3731`). Nearest supportable alternative without per-element meshes: a second instanced pass over the same per-instance transforms using a line or shell technique, plus a separate mechanism for section changes, which are not represented in the instance data at all (schematic uses one radius for every pipe, `:3715-3758`). | TBD. Requires a runtime probe: a second full-scene instanced pass at 1,000 and 10,000 pipes, measured as an orbit window with the existing 2 s/10 s method. |
| **Per-node colour interpolation** | **Not supported.** One instance per pipe with one height segment (`PipeViewport.tsx:3731`) and one colour per instance (`viewportResource.ts:983`). Nearest supportable alternative without per-element meshes: (a) flat per-element colour, which the existing `instanceColor` attribute already carries and which is fully supported; or (b) subdividing each element into several instances so colour varies stepwise, which multiplies instance count by the subdivision factor. | TBD. (a) is an attribute rewrite over N instances; (b) multiplies instance count — both need a runtime probe, and (b) also changes picking primitives (`viewportSelection.ts:459-534`). |
| **Dashed ghosts** | **Supported as a line overlay, not as ghosted geometry.** A dashed line with `LineDashedMaterial` already exists and is drawn in its own layer (`PipeViewport.tsx:3991-4000`, `:1278-1301`); transparent instanced geometry also exists today in the deformed overlay (`:3887`, `:3920`). What is absent is any per-element dashed *outline* of geometry (2.8b, 2.9a), which depends on the same missing edge pass as the edge line. | TBD. A 200-element ghost layer and a whole-model dashed undeformed ghost are different magnitudes; both need a runtime probe. |
| **Fixed-screen-size glyphs** | **Supported as a technique, unused for restraints.** `THREE.Points` with a `ShaderMaterial` sizing by `gl_PointSize = 11.0 * pixelRatio` already draws fixed-screen-size marks over the scene, with `depthTest: false` and a high render order (`viewportSelectionPresentation.ts:19-48`), refreshed from the renderer's pixel ratio (`viewportResource.ts:609-615`). Restraint glyphs today are world-sized cones (`PipeViewport.tsx:3788-3817`). The point-sprite technique is limited to one shape per material and one size uniform, so seven restraint families plus direction arrows would need either several point materials with distinct fragment shaders, a texture atlas, or screen-space scaled instanced geometry. | TBD. The technique is proven to run in the existing scene; the cost of several such passes over 50 and 500 supports (`fixture-manifest.json`, `counts.supports`) needs a runtime probe. |
| **Non-overlapping label plates** | **Supported, as DOM, not as canvas overlays.** Plates are HTML buttons projected and laid out after every main render, with an O(n²) screen-rectangle overlap test against already-placed plates and `display: none` for losers (`PipeViewport.tsx:1323-1361`, invoked at `viewportResource.ts:783`). This already runs on every orbit frame and is inside the recorded orbit intervals. Being DOM, it is outside the 3D pick, outside the canvas theme repaint, and outside anything printed from the canvas (see §2 item 9). | TBD at the proposed plate count. The recorded evidence had 0–3 plates rendered under a cap of 80 (`BASELINE_CHARACTERIZATION_REPORT.md:13`; `RUNTIME_REPORT.md:24`); the proposed area budget at 1000 × 828 would admit roughly 230. Needs a runtime probe with a camera pose that brings many plates into frustum. |

**Behaviours from §2 that are not overlay questions at all**, because the product does not draw
the underlying thing: recognisable fittings (2.2), restraint symbol families (2.3a), load-vector
form and magnitude (2.4a), result colour of any kind (2.7a–c), the legend (2.7d), the result
probe (2.7e), the deformation scale and animation (2.8c, 2.8d), proposal ghosts (2.9a), the
Right preset (2.10b) and the report-figure camera (2.10d).

### 4.2 "Which observations in §4 the baseline tranche's harness can already make, and which need a new probe."

- **Already makes, as written:** none in full. The closest is 4.5, whose frame-time half is
  measured exactly as the brief describes (labels on, real orbit gesture, 10 s window, per-frame
  label recompute) and whose non-overlap half is not.
- **Needs a harness adaptation only, with no product change:** 4.5 (label-placement query),
  4.6 (a 500-ref box sample plus an orbit window holding that selection). Row 3.1b and row 3.2b
  in §1.2 are the same kind: existing product behaviour, missing instrument action.
- **Needs a harness adaptation *and* depends on a product route that exists but is unreachable:**
  the static deformed overlay (2.8a) — reachable only by driving the real mechanics preview, with
  an open question about whether it solves the benchmark fixtures at all.
- **Needs implementation before measurement is meaningful:** 4.1, 4.2, 4.3, 4.4, 4.7, 4.8. Each
  names a subject the product does not draw. Measuring today's nearest behaviour would not
  characterise the proposed one and would, in 4.1 and 4.7, chiefly characterise an absence.
- **Needs a profile change regardless of implementation:** 4.8, because the frozen boundary
  profile admits only DPR 2 at 1440 × 920 (`characterization-commands.ts:50-53`), and any
  memory-bearing observation (4.1), because the qualified lane records none
  (`full-cohort-controller.ts:905-906`).
- **TBD requiring a runtime probe, and which probe:** every cost figure in §4.1 above; the
  deformed-overlay orbit at 1,000 and 10,000 pipes (probe: the existing 2 s/10 s orbit window
  after a `deformation()` setup); the label-plate layout cost at a high plate count (probe: an
  orbit window with a frustum-filling camera pose and a raised cap); and any memory figure
  (probe: promoting the feasibility heap/RSS observation into a recorded series, with its
  existing attribution limitation preserved).

---

## 5. Uncertainties

1. **Whether the in-product mechanics preview can produce a result for the frozen 1,000- and
   10,000-pipe fixtures.** The deformed overlay is driven entirely by `displacement_magnitude`
   and signed `ux/uy/uz` rows in a `MechanicsResult` (`PipeViewport.tsx:4400-4463`). The only
   existing exercise solves five nodes (`apps/desktop/e2e/r2-smoke.spec.ts:570-580`). If the
   preview cannot solve the benchmark fixtures in session, row 2.8a's adaptation needs a
   different route, and the frozen `*.deformation-overlay.json` files — which nothing currently
   loads — would become a candidate input requiring a product ingestion path, which would make
   2.8a a class 3 item for measurement purposes. I could not settle this from source.

2. **Whether `directional` deformation holds at fixture scale.** `buildDeformationOverlay` falls
   back to a vertical display axis for every node if any node lacks matched component rows in
   the governing basis (`PipeViewport.tsx:4454-4463`, `:4487-4489`). Which branch a
   fixture-scale result takes affects what a deformation observation is actually looking at.

3. **The 200 outside-frustum point probes.** The frozen record notes 20/200 and 19/200 anchor
   projections lie outside the baseline Iso frustum (`apps/desktop/e2e/ui-foundation/README.md`;
   `fixture-manifest.json`, `point_hit_protocol.baseline_outside_frustum`). I did not determine
   whether the candidate camera recipe in use at this `HEAD` makes all 200 actionable, which the
   manifest requires before full qualification. This bears on any new sample class added under
   rows 3.1b and 3.2b.

4. **Whether the `DiffPreviewPanel` and queued-intent list have any canvas consequence I missed.**
   I searched for `viewport`, `THREE` and `ghost` across `features/agent-proposals`,
   `features/diff-preview` and `features/design-workspace` and found nothing, and I traced
   `queuedIntents` into `PipeViewport` where it feeds only validation and a DOM list
   (`PipeViewport.tsx:587-588`, `:2424-2426`). I did not exhaustively read all 384 lines of
   `DiffPreviewPanel.tsx` or all 3,169 lines of `PropertyInspector.tsx`.

5. **Reachability of the frozen environment.** Every class 1 and class 2 statement about the
   instrument assumes the bound host, display, Chromium revision and profile hash can be
   re-established (`characterization-commands.ts:194-214`; `performance-targets.ts:19-24`).
   That is an environment question for the piping session, not a source question.

6. **Whether the label overlap test is stable enough to serve as a non-overlap confirmation.**
   The test uses a fixed 62 × 30 px box regardless of the plate's actual rendered width
   (`PipeViewport.tsx:1349`), while CSS fixes the plate at 58 × 26 px plus a 1 px border
   (`styles.css:1506-1519`). The margin is small; whether a DOM-rectangle assertion would agree
   with the product's own test at every zoom is a runtime question.

7. **Whether anything reads `explicitHiddenKeys` versus `isolateHiddenKeys` differently in the
   report or export paths.** I traced both into the viewport and the composed mask
   (`App.tsx:2718-2729`; `viewportSelection.ts:219-225`) but did not audit export or report code
   for visibility awareness.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
