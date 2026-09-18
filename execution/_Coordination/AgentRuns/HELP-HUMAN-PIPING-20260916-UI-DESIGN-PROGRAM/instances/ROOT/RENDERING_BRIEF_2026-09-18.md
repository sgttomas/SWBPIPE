# Rendering brief — what the redesigned interface needs from the canvas

Status: brief from the design program (HELP_HUMAN, ROOT of this run) to the piping session, owed under [D-70](../../../../_DECISIONS/D-70_RULING_2026-09-17.md) effect 4 for the piping session's separate overlay and deformation observation run, which D-70 places after the baseline characterization tranche. Written 2026-09-18 by ROOT from the accepted design system V1 §6 (the figure language) and §2.6 to §2.8 (the canvas tokens), the design brief V1.4 §4 and the direction record §2. It specifies what the canvas must look like and do, and what to observe; it does not specify how anything is drawn, and it asks for no implementation of the design. The engine stays: Three.js, instanced meshes, the invalidation scheduler, the ownership ledger, the typed model index and picking, as the rendering boundary answer of 2026-09-17 stated. The form follows the D-70 ruling record's practice of stating the ask, the basis and the boundary in that order. Not an acceptance; nothing here is a product claim.

## 1. What the canvas is in the redesign

- **Derived, never authoritative.** The tables are the model. The canvas is generated from them and holds nothing they do not; every authoring gesture in the canvas (direct distance entry with a routing compass, click a node to add a restraint row) writes a table row through the same structured operation as typing it.
- **Present in two of three views.** Each stage (Model, Loads, Results, Review) has Table, Model and Both views, remembered per stage. The canvas is absent in Table view and returns in Model or Both with its camera and display state intact. Switching stage or view must not lose the camera, the selection, the label mode, the legend or the deformation scale.
- **Sizes at 1440 × 900.** Model view: 1000 × 828 with the table drawer closed, 1000 × 548 with it open. Both view: 603 × 828 beside a 737-wide table, resizable, with the agent strip; when the agent column opens the surfaces narrow to 1044 wide and the canvas shrinks with its share. Minimum window 1280 × 800, surfaces 1180 × 728. Device pixel ratio 1 and 2 both matter on macOS.
- **Two themes, one set of tokens.** Every colour on the canvas is a token with a light and a dark value ([`../DESIGN-SYSTEM/tokens.json`](../DESIGN-SYSTEM/tokens.json), the `canvas.*`, `result.scale.*` and categorical groups). A theme switch repaints the canvas live; nothing is designed for one theme only.

## 2. The figure language, as behaviours to observe

The design system calls the language "the figure": the canvas draws the figure the report prints, with explicit glyphs. Each item below is an observable behaviour; the specification text is in [`../DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md`](../DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md) §6, and the schematic in the specimen's canvas section shows every element once.

1. **Pipe.** Real outside diameter; neutral matte shading (a soft darkening toward the silhouette, never a highlight); a one-pixel edge line at every zoom level, at the silhouette and at every change of section, in the theme's edge token. Diameter changes read as steps in the edge. Insulation, when modelled, as a translucent sleeve. An element without a section draws as a 1.5 px centreline so a model can be routed before sections exist.
2. **Fittings.** Recognisable geometry, not symbols: elbows swept at radius with tangent-point ticks, tees as joined tubes with the branch saddle, reducers as cones, valves as the standard silhouette extruded, flanges as disc pairs, rigid elements in the shade token, expansion joints convoluted, beams as their section. Every fitting takes the edge line and, when results are shown, the result colour.
3. **Restraint glyphs.** The standard symbols the report legend explains, at a fixed screen size at any zoom: anchor as a hatched block, +Y as a triangle under the pipe, guide as a bracket pair, limit stop as a plate on the stopped side, two-way as a bar through the pipe, skewed along its vector, spring as a can with a rod, connecting node as a thin tie. Each glyph carries a small direction arrow. A gap draws as visible clearance with its value as a label; friction as a short hatch on the contact face with μ as a label.
4. **Load vectors.** Arrows from the node scaled by magnitude with a scale reference in the HUD; the arrow's form carries the kind (single head for force, double head along the axis for moment, bar at the base for displacement, a row of short arrows along the element for uniform load, tail streaks for wind, zigzag base for seismic); colour by kind or by the selected case, neutral when result colour is on; labelled with value and unit on hover and selection, and permanently when Labels is on.
5. **Node labels.** The node number on a small plate offset one pixel from the node, 12 px tabular. Budget: one label per 3600 square pixels of canvas, priority selected, hovered, current row, restrained, loaded, node data, branch points, then the rest by spacing; the current row's node is always labelled; labels never overlap, the lower priority yields. The control cycles All, Budget, Off.
6. **Selection and hover.** Selection is a 2 px halo around the element's silhouette and its glyphs, hover a 1 px halo, both in their tokens; multiple selection halos every selected element; both sync with the tables in both directions and with the inspector. Isolate dims everything else to 20 % opacity; Hide removes it and the HUD shows the hidden count.
7. **Result colour.** Applied to tube and fittings as a flat colour per element, or interpolated per node along the element for stress, from a seven-anchor sequential scale that flips its anchor in dark mode; unsolved elements stay in the unsolved token at 60 % opacity; edge lines remain over colour; halos draw over colour unchanged; the legend sits in the canvas with the scale, units, rule ID and pack version; a probe at the cursor reads the values.
8. **Deformation.** The deformed shape solid, in result colour or pipe neutral; the undeformed shape as a 1 px dashed ghost; the scale factor stated in the legend, adjustable by stepper and field, defaulting to the factor that makes the maximum displacement one tenth of the model's extent rounded to a round number; animation as a play control that oscillates the factor from zero to the stated value, one case at a time.
9. **Ghosts.** A pending agent proposal draws its geometry or glyph dashed at 60 % opacity in the proposal-ghost token beside the current one, a removed element as a dashed outline, and the affected node's label plate takes the ghost edge; accepting turns the ghost into the drawing. The engineer's own routing draft is a second ghost in the draft token, never the same colour.
10. **Camera.** Fit (all or selection), Iso, Top, Front, Right, and a named "Report figure" camera saved with the project; the report prints the figure exactly as the canvas draws it at that preset, in light by default, with the legend and the scale reference. The triad states the up axis.

## 3. What the tables need from the canvas

- Row selection highlights the element, and element selection selects the row, within one frame at the sizes in §1.
- Picking resolves to the node or element the row grammar names, including glyphs and label plates.
- The probe reads per-element and per-node values at the cursor after a run.
- The routing compass shows a ghost along the current axis as a length is typed and commits a row on Enter; Tab cycles the axis.
- The table drawer in Model view resizes without a canvas relayout stall; the split in Both view drags live.

## 4. What to observe in the overlay and deformation run

At the two sizes D-70 fixed for the baseline (1,000 and 10,000 pipes), on the same hardware and with the per-run canvas dimensions, device pixel ratio and rendered label count reported as D-70 requires:

| Observation | What to record |
|---|---|
| Result colour on every element with edge lines and labels at budget | Frame time steady-state; time from run completion to first coloured paint; memory delta |
| Per-node interpolated stress colour along elements | Whether the instanced path supports it; frame time |
| Deformation animation at the default scale | Frame rate over ten seconds; cost of changing the scale factor |
| Two hundred elements ghosted as a pending proposal | Frame time with ghosts; time to accept (ghost becomes drawing) |
| Label budget recompute on continuous camera motion | Frame time during orbit; confirmation that no two labels overlap at rest |
| Selection halo on five hundred elements | Frame time; pick latency to the table |
| Theme switch with results shown | Time to full repaint |
| Canvas at 603 × 828 and 1000 × 828, DPR 1 and 2 | Whether the edge line stays one pixel and glyphs stay fixed-size at both |

And two questions for the piping session, answered in the observation report:

1. Which of the behaviours in §2 the current architecture supports as overlays on instanced meshes without per-element meshes (edge lines, per-node colour interpolation, dashed ghosts, fixed-screen-size glyphs, non-overlapping label plates), and for each that it does not, the nearest supportable alternative and its cost.
2. Which observations in §4 the baseline tranche's harness can already make, and which need a new probe.

## 5. What this brief does not ask

No engine replacement or change to instancing, invalidation, the ledger, the index or picking. No product UI change in the baseline tranche (D-70 effect 3). No implementation of the design; the mocks that follow the design system are drafts for the owner's review, not a specification to build. No claim about the product's performance or fitness; the observation report states what was measured, under the standard claim fence.

## 6. Where the details live

- Design system V1: [`../DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md`](../DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md) §6 (the language), §2.6 (result scale), §2.7 (categorical set), §2.8 (canvas tokens), §2.9 (contrast findings); [`../DESIGN-SYSTEM/tokens.json`](../DESIGN-SYSTEM/tokens.json); [`../DESIGN-SYSTEM/specimen.html`](../DESIGN-SYSTEM/specimen.html) section 11, the schematic figure.
- The direction: [`DIRECTION_DECISION_2026-09-17.md`](DIRECTION_DECISION_2026-09-17.md) §2; the brief: [`DESIGN_BRIEF_V1.md`](DESIGN_BRIEF_V1.md) §4.
- The rendering boundary answer: [`RENDERING_BOUNDARY_ANSWER_2026-09-17.md`](RENDERING_BOUNDARY_ANSWER_2026-09-17.md).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
