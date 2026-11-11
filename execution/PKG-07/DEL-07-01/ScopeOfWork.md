---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-07-01
package_id: PKG-07
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-020,SOW-077]
package_objective_refs: [OBJ-006]
---

# Scope of Work — DEL-07-01 3D viewport and centerline editor

## Purpose and Objective Traceability

This deliverable is where the engineer sees the piping model in three dimensions and builds it by hand. The viewport draws the centreline model as it is stored: nodes, pipe runs, bend arcs, branches, component symbols, supports, labels and load arrows. The editor turns pointer and keyboard work in that view into model operations: placing nodes, routing and continuing pipe runs, splitting, reconnecting and transforming runs, and placing components. The tool palette, which gives every model-building command one place, is drawn and routed here. Nothing the viewport does changes the model except through the operation route, and nothing it draws is an engineering value the user did not enter.

It serves SOW-020 and SOW-077 under OBJ-006 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-07, row DEL-07-01), and PRD §14.1, §14.2, §5.3, FR-GUI-001 and FR-GUI-004. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names this deliverable. It supports criterion 1, where the reference system is built and solved in the desktop, and criterion 7, where that system is entered from blank within the authoring time.

- **OUT-001** — The 3D viewport and centreline editor: the drawn model, selection and visibility in the view, and the editing gestures that produce model operations.
- **OUT-002** — The tool palette as drawn and routed: every command of the DEL-07-09 vocabulary reachable from one palette, each leading to its operation route.

## Deliverable Definition — Ontology

**Terms the viewport and editor implement.**

- *Centreline model.* The piping model as a 3D frame of nodes joined by line elements. The viewport draws that model and no other geometry; shell and solid detail is not part of it.
- *Component symbol.* A simple glyph that marks a bend, tee, reducer, valve, flange or expansion joint at its place on the centreline. A symbol shows that a component exists and where; it carries no dimension the user did not enter.
- *Actual OD display.* A drawing of each straight span at its section's entered outside diameter. A span whose section does not give a valid diameter is drawn as a centreline.
- *Draft.* A node, pipe or component the user has placed or typed but not yet applied. A draft is visible and editable, is transient, and cancelling it changes nothing in the model.
- *Intent and batch.* What an editing gesture produces: one structured operation or an ordered set of them, queued for review and applied only on the user's Apply.
- *Selection.* A set of typed entity identifiers, shared with the model tree and the property inspector. Point selection picks one entity, modified clicks add or remove, and box selection picks every entity the box encloses.
- *Hide and isolate.* View-only filters on what is drawn. Hidden entities stay in the model and in the tree.
- *Measurement.* The distance between two authored nodes, read from their stored positions and shown with its unit.
- *Split into equal parts.* One pipe becomes N pipes of equal length with N − 1 new nodes. Node k is stored at A + (B − A)·(k/N), computed per component in binary64 from the stored end positions A and B, so every node is placed from the ends and none from its neighbour.
- *End point by length and direction.* A new end point computed from a start node and one of: a length along a direction vector, a length at an angle from the horizontal, or a horizontal run at a slope of 1 in n. For angle and slope the user states which way is up.
- *Tool palette.* The searchable list of model-building commands, in the groups DEL-07-09 fixes. A palette command opens the form or tool that builds the operation; it changes nothing itself.
- *Settled frame.* The state of the viewport when nothing is moving: no animation frame is scheduled.

**Boundary.** This deliverable draws and edits the model in the 3D view. The following belong elsewhere:

- the operation vocabulary, its coverage and the palette's groups: DEL-07-09;
- validation, preview, acceptance and application of every operation: DEL-16-02, DEL-16-03 and DEL-16-06;
- the model tree, tables, property inspector, run sheet and support forms: DEL-07-02;
- component, material and section forms, which the viewport hosts when a component is placed: DEL-07-03;
- deformed shapes, rotation glyphs, reaction arrows and result colour drawn in this viewport: DEL-07-05;
- comparison overlays: DEL-07-08;
- the shell that mounts the viewport, shared selection and session history: DEL-07-11 under the DEL-00-05 rule;
- theme, density, focus and contrast: DEL-07-06.

**Not decided.** PRD §14.2 lists equipment interfaces, route corridors, no-go volumes and supportable zones among what the viewport displays. No requirement of this scope or of any other commits to drawing them.

## Completion and Reliance Basis — Epistemology

The viewport is a view and an entry surface. What it draws is evidence of what the model document holds, and nothing more: a drawn model is not a checked model.

### Requirements

- **REQ-001** — The user creates and edits nodes, pipe runs, bends and component symbols in a 3D centreline viewport (SOW-020; PRD §14.1, FR-GUI-004).
- **REQ-002** — The viewport draws pipe centrelines, bend arcs, branch symbols, valves, flanges, reducers, expansion joints, supports and restraints, node and element labels and load vectors, each selectable. Deformed shapes, reaction arrows and stress and result overlays are drawn in the same viewport by DEL-07-05 (PRD §14.2, FR-GUI-001).
- **REQ-003** — The viewport is a Three.js view inside the React interface of the Tauri desktop (DEC-009).
- **REQ-004** — Every change to the model made from the viewport is a structured operation that passes validation, preview and the user's Apply. Camera, hover, selection, drafts, visibility and measurement are transient and are not written to the model document (PRD §5.3; decomposition AB-00-05).
- **REQ-005** — Coordinates and every quantity entered in the viewport carry a unit and are checked by the unit system. A new entry starts in the project's unit, and where the project has none for that quantity the field has no default and says so.
- **REQ-006** — The viewport and its tools supply no component dimension, stress intensification or flexibility factor, allowable, code value or catalogue value. A value the model lacks stays missing and is shown as missing (PRD §5.5).
- **REQ-007** — A diagnostic that names a model entity is reachable from that entity in the view, with the class it had in its result envelope.
- **REQ-008** — No colour, label, status or state in the viewport states or implies certification, approval or code compliance (PRD §21.2).
- **REQ-011** — The tool palette offers every NORMATIVE-NOW item of the DEL-07-09 vocabulary in its group, each command opening the form or tool that builds its operation. A command that cannot run in the current context is shown disabled with its reason. A roadmap item is listed as not available and cannot be invoked. No palette command changes the model by any route other than the one operation route (SOW-077; DEC-111).
- **REQ-012** — *Interaction performance.* On the reference profile the workspace meets every limit below:
  - a model assigned to the session is presented on the canvas, with responsive tables, within 2,000 ms;
  - point selection to presented feedback, row selection to canvas highlight and canvas pick to row selection: 95th percentile at or under 100 ms;
  - box selection or filtering to presented feedback, and a view or stage switch with camera, selection, label mode and legend kept: 95th percentile at or under 200 ms;
  - the interval between presented frames while orbiting: 95th percentile at or under 16.7 ms in centreline display and 33.3 ms in Actual OD display, with result colour, edge lines and labels at their budget; while dragging the split or resizing the drawer, at or under 33.3 ms;
  - a settled viewport schedules no animation frame;
  - the resources the viewport owns return to a stable baseline after 20 model replacements and after repeated unmount and remount.

  The reference profile is an Apple M5 Max with 128 GiB, a pinned Chromium, a production build, the laptop's own display, device pixel ratio 2, Light appearance and Comfortable density, in a 1440 × 900 window, with the canvas at 603 × 828 (model beside table) and at 1000 × 828 (model alone), labels off and at the budget of 138 and 230, on invented models of 1,000 and 10,000 pipes. One run per size and canvas settles a configuration when every gated quantity is at or under 80 % of its limit; a quantity between 80 % and 100 % requires two more runs, all three within every limit; any quantity over its limit fails. An external monitor at 60 Hz, device pixel ratio 1 and the Dark theme are each measured once per size and published without gating. No limit is set for deformation animation, ghosted proposals, large-selection halos, theme switch with results, first coloured paint, per-node colour, memory or cold start.
- **REQ-013** — *Split into equal parts, and end points by length, angle or slope.* Split into N queues one batch of the existing split operation, with node placements where needed, so that each node is stored at the position defined above. Identifiers and labels follow patterns the user writes. Whatever the operation engine would refuse for one split is refused before queueing, with the engine's reason. The batch is applied whole and reverted by one Undo. A new route end, and a reconnection to a new point, accept the three forms of end-point entry. No slope, angle or vertical axis is defaulted or saved, and the coordinate displayed, the coordinate sent and the coordinate stored are the same binary64 value (PRD FR-GUI-004).
- **REQ-014** — The viewport supports multiple and box selection, hide, isolate and fit of the selection, measurement between authored nodes, and Actual OD display, and keeps its rendering resources across model changes instead of rebuilding them.
- **REQ-015** — A route is drawn with the pointer as successive nodes and pipes, and continues from a selected existing end point. A pipe is split and a component inserted within a run; runs are copied, rotated and mirrored; and a pipe is reconnected to another node or to a new point (DEL-07-09 vocabulary rows 1, 15, 19 and 20).

**Not decided.** What a copy, rotation or mirror does with the supports, loads and components attached to a run.

### Acceptance criteria

- **AC-001** — For an invented model that holds one of every drawn family in REQ-002, each entity appears in the viewport at the position its stored coordinates give, is selectable, and is selected under the same identifier in the tree and the inspector. Creating a node, a pipe run, a bend and a component symbol from the viewport each yields an applied operation and a model document that passes the document validator. Moving a node, and deleting a node and a pipe run, from the viewport each yields an applied operation and a document that passes the document validator. Changing camera, hover, selection, drafts, visibility or measurement leaves the model's canonical hash unchanged. No field offered by a viewport tool is prefilled with an engineering value, and no text in the viewport is a statement REQ-008 excludes.
- **AC-002** — Every NORMATIVE-NOW row of the DEL-07-09 vocabulary has a palette command in the group that contract assigns. Invoking each command and completing its form produces operations of the kinds the vocabulary binds that row to, and the model is unchanged until Apply. Each roadmap row is listed and cannot be invoked. With no selection and with each kind of selection, every disabled command states its reason.
- **AC-003** — On the reference profile, a run of the frozen workload (one model assignment, 200 point selections, 20 box selections, 20 filters and two ten-second orbit windows, with real pointer and keyboard input), together with the other segments REQ-012 limits (row selection to canvas highlight, canvas pick to row selection, view and stage switch, split drag and drawer resize, and orbit with result colour, edge lines and labels at their budget), meets every limit of REQ-012 under its pass rule, at both model sizes and both canvas sizes.
- **AC-004** — Splitting a pipe with end positions A and B into N parts stores node k at the binary64 value of A + (B − A)·(k/N) for every k and every N from 2 to 100, including end positions for which chained splitting differs by one unit in the last place. One Undo restores the model's canonical hash. A request the engine would refuse queues nothing and shows the engine's reason. For each of the three end-point entries, the stored coordinate equals the value the stated arithmetic gives, and equals the coordinate displayed before Apply.
- **AC-005** — On a 10,000-pipe invented model: box selection selects exactly the entities whose drawn geometry the box encloses; hide and isolate change what is drawn and leave the model's canonical hash unchanged; a measurement between two nodes equals the Euclidean distance of their stored positions; and a span is drawn at its entered outside diameter exactly when its section gives a valid one.
- **AC-006** — Each gesture of REQ-015 produces a batch that, applied, gives the geometry the gesture showed: a continued run shares the selected end node; a split or inserted component leaves the run's end nodes and total length unchanged; a copied, rotated or mirrored run has the transformed coordinates of the source; a reconnected pipe ends at the chosen node.
- **AC-007** — In each viewport tool, a coordinate or quantity entered with no unit, with an unknown unit or with a unit of another dimension is refused by name. A new entry starts in the project's unit; where the project has no unit for that quantity the entry has no default and its notice names the missing unit. For an invented model with one diagnostic naming a node, one naming a pipe and one naming a component, each diagnostic opens from that entity in the view and shows the class it had in its result envelope.

## Production and Verification Method — Praxeology

**Method choices that define the viewport.**

- The editor builds operations and never applies them. One engine validates, previews and applies every change, for the viewport as for every other surface.
- Identifiers are explicit. A tool that creates entities asks for their identifiers or a pattern for them and never renumbers what exists.
- A position is computed once from stored values, in binary64, and shown, sent and stored without rounding to a display precision.
- The view is drawn from the model and the session's transient state on every change. No model fact lives only in the scene.
- The palette is one list read from the DEL-07-09 catalogue, so a command exists in one place and its availability has one rule.
- Performance limits are perception bounds on the presented frame, measured on a stated profile. Trace and GPU-thread records are kept and never gate.

**How each criterion is examined.**

- **VER-001** — Executable check (observes the drawn scene, the selection, the applied operations and the document hash): exercises the viewport on invented models through real pointer and keyboard input, for AC-001, AC-005, AC-006 and AC-007, comparing drawn positions and resulting geometry with values computed from the stored coordinates, and unit refusals and diagnostic classes with the project units and the supplied envelope.
- **VER-002** — Executable check (observes the palette's commands, their disabled reasons and the operations each produces): walks the DEL-07-09 vocabulary rows against the palette, for AC-002.
- **VER-003** — Executable check (observes interaction latency and Chromium-reported presentation intervals on the reference profile): runs the frozen workload and applies the pass rule, for AC-003.
- **VER-004** — Executable check (observes the stored node positions after the batch is applied by the operation engine): compares them with the stated interpolation and end-point arithmetic evaluated independently, for AC-004.
- **VER-005** — Judgment by a named role (an independent reviewer who did not write the code): examines the viewport's texts, colours and states against PRD §21.2 and `docs/claims_registry.md`, and its tools for prefilled engineering values, for AC-001.

## Governing Values and Decisions — Axiology

- **AX-001** — *Fewer steps to a correct model.* Better is the same geometry built with fewer gestures and less typing, measured by the gestures and keystrokes the reference system takes. Speed is never bought by supplying a value the user did not enter, by skipping validation or preview, or by applying a change without the user's Apply.
- **AX-002** — *What is drawn is what is stored.* Better is fewer ways the view can differ from the model: fewer families the viewport cannot draw and fewer states in which a drawn position is not the stored one. A richer picture never adds geometry, dimensions or detail the model does not hold.
- **AX-003** — *Stay responsive at size.* Better is lower interaction latency and a steadier frame interval on larger models, measured as REQ-012 states. Responsiveness is never gained by dropping entities from the view without saying so, by drawing a stale model, or by weakening a limit.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-020 OBJ-006 | REQ-001 REQ-002 REQ-003 REQ-004 REQ-006 REQ-008 | AC-001 | VER-001 VER-005 | The invented model says what must be drawn and where: drawn positions are compared with the stored coordinates, which the viewport did not write. The excluded statements are an owner ruling, PRD §21.2 and `docs/claims_registry.md` |
| OUT-002 | SOW-077 OBJ-006 | REQ-011 | AC-002 | VER-002 | Owner ruling: the DEL-07-09 vocabulary and its operation bindings say which commands must exist and what each must produce |
| OUT-001 | SOW-020 OBJ-006 | REQ-012 | AC-003 | VER-003 | Owner ruling: the limits, the reference profile, the workload and the pass rule are the owner's; the measured quantities are reported by the browser, not by the product |
| OUT-001 | SOW-020 OBJ-006 | REQ-013 | AC-004 | VER-004 | Outside source: the closed-form interpolation A + (B − A)·(k/N) and the trigonometry of the three end-point entries, evaluated in binary64 by a reference that imports no product code |
| OUT-001 | SOW-020 OBJ-006 | REQ-014 | AC-005 | VER-001 | Outside source: Euclidean distance and box enclosure computed from the stored coordinates; the 10,000-pipe workload is the owner's |
| OUT-001 | SOW-020 SOW-077 OBJ-006 | REQ-015 | AC-006 | VER-001 | Outside source: the rigid transforms and the length identities of a split are closed forms on the stored coordinates |
| OUT-001 | SOW-020 OBJ-006 | REQ-005 REQ-007 | AC-007 | VER-001 | The project units and the unit catalogue are DEL-02-02's, and the diagnostics come in an envelope the viewport did not write; the expected starting unit, refusals and classes are read from them |
