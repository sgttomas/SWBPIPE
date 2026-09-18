# Concept directions V1 — SWB Piping Designer human interface

Status: proposal for the owner's choice. Written by the HELPS_HUMANS design manager under sealed brief CONCEPTS-01 (phase 2 of the interface program), 2026-09-17. Basis: the accepted design brief V1.1, research returns D, A and C, the opening report and the rendering boundary answer. No product file was read for any purpose other than grounding the operation list; none was modified.

How to read this. Section 0 fixes what every direction shares, so the directions themselves only say what is distinctive. Sections 1 to 3 are the three directions, each in the order the brief asks for: thesis, borrowings and departures, shell, storyboard, disclosure homes, 3D presentation language, risks. Section 4 compares them, section 5 recommends, section 6 names the items that belong in the owner's decision packet, section 7 indexes the wireframes.

The three directions are different organising ideas, not three skins:

| | Direction | Organising idea in one line |
|---|---|---|
| A | Layout Sheet | The model is a sheet of elements addressed by node; the 3D view is the sheet's picture. |
| B | Workbench | The model is an object on a bench; one selection, one inspector, tables in drawers underneath. |
| C | Run Book | An analysis is a bound sequence, model to loads to results to report; each stage is a page beside the model. |

## 0. Fixed for all three directions

These come from the accepted brief and the constraints sheet and are not up for choice between directions. They are stated once so that the directions can be compared on what actually differs.

**Window and chrome.** Default 1440 × 900, minimum 1280 × 800, macOS. A single unified title-and-toolbar band of about 52 px carries the product wordmark, project name and save state, the layout switcher (Model, Grid, Split), the Run button with its progress, the Issues count, the Agent toggle, the display-units selector and the palette search. One status bar of 24 px at the bottom carries the analysis status chip (M-08), the issues count, the current selection and the display units. That leaves 824 px of height for the surfaces. Canvas share figures below use the whole 1440 × 900 window as the denominator.

**Three layouts.** Model (canvas hero, inspector right, grid as a bottom drawer), Grid (grid hero left, canvas live right), Split (grid and canvas equal). Every direction supports all three; the directions differ in what else is on screen and in how the surfaces behave.

**One spine.** One model, one selection, one issues list, one run. Selecting a row selects the element in the canvas and vice versa; the inspector, the issues list, the results tables and the agent panel all follow that selection. Every edit, from the grid, the canvas, the inspector or an accepted agent proposal, is the same structured operation with one undo checkpoint.

**The grid.** One row per element: From, To, DX, DY, DZ (or X, Y, Z absolute, switchable per column group), section, material, T1..Tn, P1..Pn, and marked cells for restraints, loads and components on the element's nodes. Values propagate down by connectivity, not row order, and propagated cells are visibly marked; typing over a propagated cell makes it entered. Alphanumeric node IDs with default increment of 10; insert and split renumber nothing. Paste from a spreadsheet opens a mapping preview before it commits as one operation. Tab, Enter and arrows move as in a spreadsheet; every cell and every form field is reachable from the keyboard.

**Load cases.** One table in the shape of the Aspect Pipe Stress load case editor: rows W, T1, P1, OPE, SUS, EXP, OCC and combinations as expressions; an origin column per row (generated from rule pack name and version, edited, authored); generation fills rows, hand authoring adds rows, both are the same rows.

**Run and results.** Run is a button, never a screen. Progress and any failure reason appear where the engineer is looking. Each run is immutable and named. Results are tables first (displacements, restraint loads, restraint summary, element forces and moments, stresses with ratio, rule ID and pack version, per case and enveloped, hanger table), then colour on the model with a probe and a legend, then animated deflection. A reopened saved run is labelled as historical (M-13).

**Agent.** A visible collaborator in its own panel: conversation, proposals as diffs with rationale, accept or reject per proposal, and a record of what was accepted. Proposals are drafts until accepted (M-14), never applied on their own, and unknowns show as TBD rather than invented values.

**Governance off the surface.** No boundary sentence is repeated across product surfaces. The acceptance sentence (M-02) has one placement per surface class, in a header disclosure on the results surface and in the report; the maturity line (M-01) and the product name are decision-packet items (brief §6). Provenance is one disclosure away on any entity, value or run. No label reads as certification, approval or code compliance; ratios show number, rule ID and pack version, and the colour scale is a scale, not a verdict.

**Deferred and absent.** Stress isometrics and run comparison do not appear in any state. Hanger design is present in all three.

**Rendering.** The engine stays as it is. The 3D presentation language sections describe what the model looks like, not how it is drawn.

---

## 1. Direction A — Layout Sheet

### 1.1 Thesis

The model is a sheet. Every element is a row, every node is an address, and every other thing an engineer attaches to the system, restraints, loads, load cases, results and hangers, is another sheet joined on the same addresses. The 3D view is the picture of the sheet: the current row is the highlighted element, the highlighted element is the current row, and the cursor moves in both at once. This is CAEPIPE's Layout window, which a veteran will recognise in the first second, generalised from one text window to a small set of tabbed sheets that all share one model and one undo. Results are not a separate processor; the stress table is just another sheet whose rows carry the same node and element numbers the engineer typed in the first place.

### 1.2 Borrowed and departed

Borrowed from CAEPIPE: the layout window as the primary authoring surface, one row per element with node, type, DX/DY/DZ and data columns; the cursor sync between text and graphics; the no-hourglass stance, every row edit redraws immediately; sorted result tables.

Borrowed from Aspect Pipe Stress: the element field set and its names (From, To, DX, DY, DZ, diameter, wall, material, temperatures, pressures, and the auxiliary data reached from the row); the red-marked propagated values, rendered here as a mark rather than a colour; the load case editor as an editable table with stress type per case; the standard output table set.

Departures, and why:

- Sheets are tabs over one model, not windows or modes. The reason is the brief's one-spine principle and the audit finding that mode hops are where CAESAR-lineage tools lose undo and context.
- Absolute coordinates are a column group the engineer can show beside or instead of DX/DY/DZ, and paste from a spreadsheet is a first-class entry path. The reason is the length-only complaint in D §2.
- Alphanumeric node IDs, propagation by connectivity with a visible mark. The reason is the order-dependent propagation hazard documented for imported models (D §2).
- No error-check screen. Issues are a live drawer with counts in the status bar; each issue links to its row. The gate becomes a Run button that says why it is disabled.
- Results are a sheet joined on node and element, not an output processor with a case-by-report picker. The picker survives as a case selector and a table-tab strip on the sheet.

Departure from the brief: none. A remembers Split as the layout for new projects because the sheet-and-picture pairing is the direction's identity; the brief leaves the default open.

### 1.3 Shell at 1440 × 900

Regions: the toolbar band (52 px) and status bar (24 px) as in §0; the sheet region with a tab strip (Layout, Restraints, Loads, Cases, Results, Hangers), a column header with units, rows, and a footer with the row count and the propagated-value count; the canvas with a corner HUD (view presets, fit, section view, isolate and hide, labels, and, when a run exists, the legend and case selector); the inspector, which in this direction is a slide-over, opened by double-clicking a marked cell, from the row gutter, or from the canvas selection, and closed by Escape; the agent panel, a right-hand drawer of 380 px that overlays the canvas edge and does not reflow the sheet; the issues drawer, a bottom drawer of 200 px under the sheet.

| Layout | Always visible | Drawer | Inspector | Canvas size and share |
|---|---|---|---|---|
| Model | Canvas 1100 wide, inspector docked right at 340 | Sheet as bottom drawer, 240 px default, resizable, tab strip on its top edge | Docked right, showing the current row's attachments and provenance | 1100 × 584 = 50% with the sheet open; 1100 × 824 = 70% closed |
| Grid | Sheet 900 wide with its tab strip; canvas 540 wide | Issues under the sheet; agent over the canvas edge | Slide-over from the right edge of the sheet, 380 px, over the canvas | 540 × 824 = 34% |
| Split | Sheet 720, canvas 720 | Issues under the sheet; agent over the canvas edge | Slide-over as in Grid | 720 × 824 = 46% |

The sheet keeps its tab strip in every layout, so the engineer's mental map (which sheet am I on) never changes when the layout does.

### 1.4 Storyboard

1. **New project, first element.** Split layout. The Layout sheet shows one blank row with From prefilled as 10 and the To cell focused; the canvas shows a ground grid and the triad, nothing else. The engineer types 20, Tab, 3000 in DX, Enter. The row completes, a straight pipe from 10 to 20 appears in the canvas at real outside diameter as soon as a section exists; until then the pipe draws as a thin centreline and the section and material cells carry a required mark. The status bar shows two solve-blocking issues; the Run button's hover text says which. Agent panel closed, its tab visible on the right edge.
2. **Routing in the canvas.** Model layout. The engineer clicks Route in the canvas HUD. A compass appears at node 20 with the three global axes; typing 2500 shows a ghost pipe along the current axis, Tab or an arrow key changes the axis, Enter commits. In the sheet drawer a new row 20 to 30 appears with DZ −2500 and the section, material and temperatures marked as propagated. A bend is inserted at 20 with the default radius shown in the row's component cell. Agent panel closed.
3. **Grid editing and paste.** Grid layout. The engineer copies forty rows from a spreadsheet and pastes at the row after 30. A paste band appears above the affected rows with a column mapping header (From, To, DX, DY, DZ recognised; two unmapped columns offered as ignore or map) and a count; Enter commits as one operation. Propagated cells are marked, the gutter shows the propagation chain, and the Issues drawer updates live with the two nodes whose IDs collided (a model-validity blocker each, linked to the rows). Undo removes the whole paste. Agent panel closed.
4. **Restraints and loads.** Split layout, Restraints sheet. Rows are per node: Type, Direction, Gap [mm], Friction, Stiffness [N/mm], Connecting node. Typing +Y on node 30's row draws the glyph under the pipe in the canvas at once; entering a gap draws it as a visible clearance; friction as a hatch mark. On the Loads sheet, a force of −2000 N in Y at node 40 draws a scaled arrow with its value. The Layout sheet's marked cells for those nodes now show the attachment; double-clicking one opens the inspector slide-over with the same fields. Agent panel closed.
5. **Load cases.** Cases sheet. The engineer clicks Generate from rule pack; rows W, T1, P1, OPE, SUS, EXP fill with their expressions and the origin column reads generated with the pack name and version. The engineer changes EXP's stress type; its origin becomes edited. A new row typed as OCC1 = W + P1 + U1 is authored. Expressions are text cells; the rendered expression beside each is labelled display-only (M-15). Agent panel closed.
6. **Running and failure.** Split layout. Run in the toolbar; a thin progress strip under the toolbar names the stage (assembling, solving case 3 of 6). The run stops; a banner across the top of the sheet says the nonlinear support at node 60 did not converge after the iteration limit, with Show node 60; the Issues drawer lists it under Nonlinear with the same link; the status chip reads Model incomplete for mechanics, attributed to the solver. Nothing modal. Agent panel closed.
7. **Results.** Any layout; the wireframes show all three. Results sheet, table tabs across the top in reviewer order: Stresses, Displacements, Restraint loads, Restraint summary, Forces and moments, Hangers. The sheet header carries the run name, the case selector with an Envelope toggle, a sort-by-ratio control, and an information disclosure that holds the acceptance sentence, the evidence-status label and the run identity. Columns: Node, Element, Case, Stress [MPa], Allowable [MPa], Ratio, Rule, Pack. Clicking a row highlights the element; the canvas colours the pipe by ratio with the legend and case selector in the HUD, and a probe card follows the cursor with the same eight values. In the Model layout the results sheet is the bottom drawer and the inspector shows the selected element's rows across all cases. Agent panel closed.
8. **Hanger design, report, agent proposal.** Hangers sheet: rows per hanger location, Design load [N], Travel [mm], Library, Size, Cold load [N], Hot load [N], with a Design pass button and a Select from library button that needs a vendor table imported through Libraries (that import dialog carries M-05). Report from the File menu opens a section checklist and a preview with the required notice and content block. The engineer opens the Agent panel from the toolbar; it slides over the canvas edge. The agent proposes replacing the rigid support at node 80 with a variable spring, with rationale and one TBD (the vendor table to use). The proposal is a diff card in the panel and, in the sheets, the affected rows are marked proposed with old and new values side by side; the canvas shows the proposed glyph as a ghost. Accept applies the same operation as a hand edit, records the rationale, and clears the current results while keeping the run as historical.

### 1.5 Disclosure homes

| ID | Home in Direction A |
|---|---|
| M-01 | About SWBPIPE window, on demand. Until the decision packet resolves the registry entry, the same single sentence sits in the status bar's right-end popover. |
| M-02 | Results sheet header information disclosure (one placement for the results surface class); the report. Not on the Layout sheet, the canvas or the inspector. |
| M-03 | Every generated report and the report preview, from the renderer. |
| M-04 | Reports that include hanger selection records or handoff metadata; the report preview shows it when those sections are included. |
| M-05 | The Libraries import dialog (materials, sections, components, hanger tables, rule packs) and the export and redaction dialogs. |
| M-06 | The export and handoff dialog. |
| M-07 | The report; the report preview outline shows each required item as a section. |
| M-08 | Status bar chip with a short label; the popover shows the raw status and its authority domain (solver, rule pack, human). |
| M-09 | Results sheet header, beside the run name, as a label chip with the raw token in its tooltip; on each report case page. |
| M-10 | Issues drawer, grouped by class, each row linked to its sheet row; inline marks on the affected cells. |
| M-11 | Every column header on every sheet, every inspector field, every probe value, every legend. |
| M-12 | Inspector slide-over, Provenance disclosure for the selected entity or value; Libraries dialog for imported records. |
| M-13 | Results sheet header banner when a saved run is reopened, with the stated limit. |
| M-14 | Agent panel: proposal cards marked Proposal, diff view, Accept and Reject, rationale, assumptions and TBD items; the Accepted record tab. |
| M-15 | Cases sheet, rendered-expression column caption; the rule pack editor in Libraries. |
| M-16 | Preferences, Appearance section: contrast findings as warnings, no conformance statement. |
| M-17 | About SWBPIPE, Scope and limitations tab. |

### 1.6 3D presentation language

The canvas is a drawing that happens to be three-dimensional. Pipes are drawn at real outside diameter with flat shading and a dark edge line, so that a screenshot reads like a rendered isometric. Fittings are geometric: elbows at their bend radius, tees as joined tubes, reducers as cones, valves and flanges as the standard drawing silhouettes extruded. Restraint glyphs are the standard two-dimensional symbols given thickness: an anchor as a hatched block, +Y as a triangle under the pipe, a guide as a bracket pair, a limit stop as a plate on the stopped side; a gap draws as a visible clearance between glyph and pipe, and friction as a short hatch on the contact face. Loads are arrows from the node, scaled by magnitude with a scale reference in the HUD, labelled with value and unit. Node labels are the primary annotation because they are the address: on for all nodes with restraints, components or selection, thinned by a budget elsewhere, always on for the current row. Result colour is applied to the tube with the legend in the HUD, its scale stated in ratio or stress with units; nothing is coloured that has not been solved. Deformation shows the deformed shape solid with the undeformed shape as an outline ghost, the scale factor stated in the HUD and adjustable; animation is a play control on the same HUD.

### 1.7 Risks

- The canvas-primary engineer may feel the sheet is the master and the model is a viewer. Mitigation: the Model layout, the route compass and canvas placement of restraints all write rows as their side effect; the wireframes show the Model layout as a full working state. If the owner's own habit is the canvas, this direction will feel like a step back.
- Six sheet tabs can become six modes if each grows its own toolbar and selection. The rule is one selection, one undo, one tab strip; any sheet that needs its own command surface is a design failure.
- Results as a joined sheet may not look like an output processor to an Aspect Pipe Stress veteran who expects to pick case by report. The case selector and table tabs are meant to satisfy this; if the owner's reviewers want the processor picker, the direction should add it as a dialog that produces the same sheet.
- A slide-over inspector in Grid and Split is one more thing that opens over the canvas. If engineers keep it open, the canvas in those layouts drops below a third of the window.

---

## 2. Direction B — Workbench

### 2.1 Thesis

The model is a thing on a bench. You point at it, and everything about what you are pointing at, its geometry, its section and material, its temperatures and pressures, its restraints and loads, and after a run its results, is in one column on the right. There are only six verbs, Select, Route, Restrain, Load, Probe and Measure, and they live in a narrow strip on the left; the canvas is the form. The grid, the load cases, the issues and the result tables are drawers under the bench: pulled out when the engineer wants rows, pushed back when they want the picture. An AutoPIPE or Onshape user will feel at home; a CAEPIPE user will recognise the graphics window with the text window in a drawer; an Aspect Pipe Stress user will find the element form, field for field, in the inspector.

### 2.2 Borrowed and departed

Borrowed from CAEPIPE: the graphics window as the working surface; click in graphics, see in text; immediate redraw. Borrowed from Aspect Pipe Stress: the element form's field set and names, rendered as the inspector's sections, with the auxiliary panes (restraints, displacements, hangers, bends, SIFs) as expandable sections of the same column; the output report set, presented as the tabs of the Results drawer; the load case editor as the Load cases drawer.

Departures, and why:

- The spreadsheet is a drawer, not the master. The reason is the direction's organising idea: one selection, one inspector; the grid is a way to see many selections at once. The brief's equality of grid and model is kept by the Grid layout, where the drawer becomes the hero.
- Restraints and loads are placed with a tool on the model, then edited in the inspector, rather than typed as auxiliary data first. The reason is the restraint-semantics complaint in D §2: seeing the glyph as you place it prevents the gap-sign and direction mistakes.
- The probe is a first-class tool, not a results-drawer feature. Hover any element after a run and read its values; click to pin the probe.
- No error-check screen; issues are a drawer with a status-bar count.

Departure from the brief: the brief's Model layout has the grid as a bottom drawer; in B that is the case in every layout except Grid, where the drawer becomes the hero and the canvas moves to the right column above the inspector. The canvas share in that layout is the lowest of any direction (see 2.3); B accepts this because Grid is the layout an engineer chooses when they want rows.

### 2.3 Shell at 1440 × 900

Regions: the toolbar band and status bar as in §0; the tool strip, 48 px on the left, with the six verbs and their accelerators; the canvas with a corner HUD (view presets, fit, section, isolate and hide, labels, legend and case selector after a run, deformation scale); the inspector, a 360 px column on the right, always present in Model and Grid, a slide-over in Split; the bench drawer, a bottom drawer with tabs Grid, Load cases, Issues, Results, Hangers, resizable, remembered; the agent panel, a third column of 340 px that opens to the right of the inspector and pushes the canvas, so that the agent and the inspector are read together when a proposal is open.

| Layout | Always visible | Drawer | Inspector | Canvas size and share |
|---|---|---|---|---|
| Model | Tool strip, canvas 1032 wide, inspector 360 | Bench drawer under the canvas, 280 px default, any tab | Docked right, sectioned | 1032 × 544 = 43% with the drawer open; 1032 × 824 = 66% closed |
| Grid | Tool strip, grid 800 wide as hero, canvas 592 wide in the right column above the inspector | Issues and Load cases as tabs under the grid | Docked under the canvas, 260 px tall, collapsible | 592 × 564 = 26%; 592 × 824 = 38% with the inspector collapsed |
| Split | Tool strip, grid 696, canvas 696 | Bench drawer under both, closed by default | Slide-over from the right, 360 px | 696 × 824 = 44% |

With the agent panel open in the Model layout the canvas is 692 × 824 = 44%, which is still more than the inspector and agent together.

### 2.4 Storyboard

1. **New project, first element.** Model layout. Empty canvas with a ground grid and the triad; the inspector shows Project defaults (units, default section and material, default temperatures) so they are set once and inherited. The engineer picks Route, clicks the origin, and a compass appears; typing 3000 and Enter makes element 10 to 20. The inspector switches to that element and shows every field with its unit; section and material carry a required mark until the defaults are set. Status bar: two solve-blocking issues. Agent panel closed, its toggle in the toolbar.
2. **Routing in the canvas.** Model layout. Route continues from node 20; the compass snaps to the global axes, Tab cycles the axis, typing a length shows the ghost pipe, Enter commits; a direction change inserts a bend with the radius shown in the inspector's Bend section. The bench drawer is closed; the engineer is looking at the model. Agent panel closed.
3. **Grid editing and paste.** Grid layout. The grid is the same one as in §0 with the same paste band and propagation marks; the canvas on the right follows the current row, and the inspector under it shows that row's attachments. Keyboard entry moves down the rows as in a spreadsheet, and the canvas cursor follows. Agent panel closed.
4. **Restraints and loads.** Model layout. The engineer picks Restrain and clicks node 30; a small popover lists Anchor, +Y, Guide, Limit stop, Connecting node; choosing +Y draws the glyph and opens the inspector's Restraints section with Gap and Friction fields, so the gap is typed while its clearance is visible. Load at node 40: a popover for Force, Moment, Displacement; the arrow draws as the value is typed. Both are the same rows the Grid drawer shows as marked cells. Agent panel closed.
5. **Load cases.** Model layout, bench drawer on the Load cases tab. The same editor as §0: Generate from rule pack fills rows with their origin; edits change the origin to edited; a typed row is authored. The canvas stays above the drawer at 43%. Agent panel closed.
6. **Running and failure.** Model layout. Run; progress in the toolbar button itself and in the status bar. On failure a banner across the top of the canvas states the reason and offers Show node 60; the node is haloed in the canvas; the Issues drawer opens on the Nonlinear class with the same item. Agent panel closed.
7. **Results.** Any layout. The canvas colours the pipe by ratio with the legend and case selector in the HUD; the Probe tool shows a card at the cursor with node, element, case, stress, allowable, ratio, rule and pack; the bench drawer's Results tab shows the stress table with its tabs in reviewer order, the run name, the case selector and Envelope toggle, and the information disclosure holding the acceptance sentence and evidence label; the inspector's Results section shows the selected element's rows across all cases. In Grid layout the results table is the hero and the coloured model sits above the inspector. Agent panel closed.
8. **Hanger design, report, agent proposal.** Bench drawer, Hangers tab: Design pass, then Select from library, which asks for a vendor table through Libraries (M-05 on that dialog); the hanger table fills. Report from File, with the section checklist and preview. The engineer opens the agent; the third column appears to the right of the inspector. The proposal to replace the rigid support at node 80 with a variable spring is a diff card in the agent column; the inspector shows a Proposed column beside the current values for node 80; the canvas ghosts the proposed glyph. Accept writes the operation, records the rationale and TBD items, and the current run becomes historical.

### 2.5 Disclosure homes

| ID | Home in Direction B |
|---|---|
| M-01 | About SWBPIPE, on demand; until the decision packet resolves the registry entry, the status bar's right-end popover carries the single sentence. |
| M-02 | Results drawer header information disclosure (the one placement for the results surface class); probe card footer link to the same disclosure, not a repeat; the report. |
| M-03 | Every generated report and the report preview. |
| M-04 | Reports including hanger selection or handoff metadata. |
| M-05 | Libraries import dialog; export and redaction dialogs. |
| M-06 | Export and handoff dialog. |
| M-07 | The report and its preview outline. |
| M-08 | Status bar chip with authority in the popover. |
| M-09 | Results drawer header chip, raw token in tooltip; report case pages. |
| M-10 | Issues drawer tab, grouped by class, linked to entity; halo on the entity in the canvas; marks on inspector fields. |
| M-11 | Every inspector field, grid column header, probe value and legend. |
| M-12 | Inspector Provenance section, collapsed by default, for the selected entity; Libraries dialog for imported records. |
| M-13 | Results drawer header banner for a reopened saved run. |
| M-14 | Agent column: Proposal cards, diff, Accept and Reject, rationale, assumptions, TBD; Accepted record. |
| M-15 | Load cases drawer, rendered-expression caption; rule pack editor in Libraries. |
| M-16 | Preferences, Appearance. |
| M-17 | About SWBPIPE, Scope and limitations. |

### 2.6 3D presentation language

The canvas is a physical model under studio light. Pipes are drawn at real outside diameter with material shading and a soft key light, so that diameter changes and insulation, when modelled, read as mass. Fittings are true geometry: elbows swept at radius, tees with the branch saddle, reducers, valves with a body and a bonnet, flanges as pairs of discs. Restraint glyphs are simplified hardware: a shoe on a base for +Y, a spring can with a hanger rod for a spring, a guide frame around the pipe, a stop plate on the stopped side, an anchor as a block bolted to the ground plane; each carries a small direction arrow so the semantics are visible without reading the glyph, and a gap shows as a visible clearance. Loads are arrows scaled by magnitude with a scale reference in the HUD, coloured by kind, labelled on hover and selection. Node labels are shown only for selection, hover, restraints and components, with a budget for the rest; labels are the exception in this direction, the geometry does the talking. Result colour is a heat on the tube with the legend in the HUD and a probe card at the cursor; unsolved elements stay in the material colour. Deformation is animated with a scale slider and a play control; the undeformed shape stays as a faint ghost.

### 2.7 Risks

- To an Aspect Pipe Stress or CAEPIPE veteran the grid-in-a-drawer reads as a demotion, and the audit is clear that a demoted grid reads as a toy. The Grid layout exists to answer this, but at 26% canvas it is the weakest layout in any direction.
- Six verbs and a tool strip is a CAD idiom; the pointer-and-popover way of placing restraints is slower than typing a column for an engineer placing forty of them. The grid drawer must be as fast as A's sheet, or the direction fails on volume input.
- The inspector as a permanent column costs 360 px in every layout but Split; engineers who never look at it will resent the space.
- The physical rendering language is the most expensive to get right and the easiest to make look like a game. If the shading is wrong the whole direction loses credibility.

---

## 3. Direction C — Run Book

### 3.1 Thesis

A stress analysis is a bound sequence: model, loads, run, results, report. The interface is the run book. A narrow rail on the left names the stages in the order an engineer does the work and a reviewer reads it; each stage has one page of tables beside the model, which never leaves the screen; the run is the event that turns the pages into an immutable record; the report is the book itself, assembled from the pages the engineer has been looking at. This is the Aspect Pipe Stress sequence, Input, Load Cases, Error Check, Output, made explicit and made non-modal: any stage is one click away, the issues are live in every stage, the run is a button, and what the engineer sees on the Results page is what the report prints.

### 3.2 Borrowed and departed

Borrowed from Aspect Pipe Stress: the stage sequence itself; the load case editor as the Loads page; the output report set as the Results page tabs; the hanger design as a pass and a selection. Borrowed from CAEPIPE: the graphics beside the text in every stage; the cursor sync; sorted stresses first.

Departures, and why:

- No error-check stage. Issues are a drawer at the bottom of every page with a count on the rail; the Run button states why it is disabled. The reason is D §2's cryptic-gate complaint and the brief's principle 5.
- No output processor. Results are pages in reviewer order with a case selector and an Envelope toggle; the report is generated from the same pages. The reason is the brief's principle 6 and the report as the place credibility is won.
- The model page is the layout grid of §0, with the canvas always beside it. The canvas is not a stage; it is present in every stage, which is the CAEPIPE half of the pairing.
- The agent is a persistent column, collapsible to a strip, present in every stage, because a proposal can concern any stage.

Departure from the brief: none in substance. The brief's inspector exists on the Model page; on the other pages the right pane is stage-contextual (load detail, probe and legend, report options) rather than an entity inspector. The brief's Model layout is available in every stage; in the Results stage it puts the result tables in the bottom drawer.

### 3.3 Shell at 1440 × 900

Regions: the toolbar band and status bar as in §0; the stage rail, 56 px on the left, with Model, Loads, Results, Report, and at its foot Libraries and Rules (project data) and Issues; the stage page, a table region with its own tab strip (Model: Layout grid, Restraints, Components; Loads: Cases, Loads, Wind, Seismic; Results: Summary, Stresses, Displacements, Restraint loads, Restraint summary, Forces and moments, Hangers; Report: Outline, Preview); the canvas, in every stage, with the corner HUD; the right pane, stage-contextual, 344 px (Model: inspector; Loads: load detail; Results: probe, legend, run identity; Report: options); the agent column at the far right, 340 px open, 44 px collapsed as a strip that shows the count of open proposals; the issues drawer, 200 px at the bottom of the page.

| Layout | Always visible | Drawer | Inspector or right pane | Canvas size and share |
|---|---|---|---|---|
| Model | Rail, canvas 996 wide, right pane 344, agent strip 44 | Stage page as a bottom drawer, 280 px default | Docked right, stage-contextual | 996 × 544 = 42% with the page drawer open; 996 × 824 = 63% closed; 700 × 824 = 44% with the agent column open and the drawer closed |
| Grid | Rail, stage page 860 wide as hero, canvas 480, agent strip | Issues under the page | Slide-over from the page's right edge over the canvas | 480 × 824 = 31% |
| Split | Rail, page 670, canvas 670, agent strip | Issues under the page | Slide-over as in Grid | 670 × 824 = 43% |

The rail is the same in every layout and every stage, so the engineer always knows where they are in the book.

### 3.4 Storyboard

1. **New project, first element.** Model stage, Split layout. The page shows the Layout grid with one blank row, From prefilled as 10; the canvas is empty with a ground grid and triad; the rail shows Loads, Results and Report dimmed with captions (no cases, no run, no run). The engineer types 20, Tab, 3000, Enter; the element appears. The Run button is disabled with the hover reason (section and material missing); the Issues item on the rail shows 2. Agent column collapsed to its strip.
2. **Routing in the canvas.** Model stage, Model layout. Route from the canvas HUD, compass, direct distance entry, Enter; the page drawer shows the new row with propagated marks; the inspector shows the new element. Agent strip.
3. **Grid editing and paste.** Model stage, Grid layout. The Layout grid is the hero; paste band, mapping, one operation; the canvas at the right follows the current row; the issues drawer under the page updates live. Agent strip.
4. **Restraints and loads.** Restraints on the Model stage's Restraints tab (rows per node, same columns as A) or by clicking a node in the canvas with the Restrain command from the HUD; glyphs draw at once. Loads on the Loads stage's Loads tab (rows per node with kind, direction and value in units), vectors drawn in the canvas beside the page; the right pane shows the selected load's detail. Agent strip.
5. **Load cases.** Loads stage, Cases tab, Split layout. Generate from rule pack fills rows with origin; edit and author as in §0; the canvas shows the loads of the selected case highlighted. Agent strip.
6. **Running and failure.** Any stage. Run; a progress popover from the button names the stage and case. On failure the rail's Results item shows a failure mark, and the Results page opens with a banner stating the reason and Show node 60; the Issues drawer opens to the Nonlinear class. The status chip reads Model incomplete for mechanics, attributed. Agent strip.
7. **Results.** Results stage, any layout. The page tabs in reviewer order, Summary first: max ratio per case with the node and element where it occurs, then Stresses with Node, Element, Case, Stress [MPa], Allowable [MPa], Ratio, Rule, Pack, sortable, with the case selector and Envelope toggle in the page header beside the run name, the evidence label and the information disclosure holding the acceptance sentence. The canvas colours the model by ratio; the right pane holds the legend, the case selector and the pinned probe; hovering an element shows the probe card. In the Model layout the Results page is the bottom drawer. Agent strip.
8. **Hanger design, report, agent proposal.** Results stage, Hangers tab: Design pass, then Select from library (vendor table through Libraries, M-05 on that dialog); the hanger table fills. Report stage: the Outline lists the sections in the required order with the notices and content block shown as fixed items, the Preview renders them. The engineer expands the agent column; the agent proposes replacing the rigid support at node 80 with a variable spring, as a diff card with rationale, constraints considered and one TBD. The affected rows on the Model and Results pages are marked proposed; the canvas ghosts the glyph. Accept writes the operation; the rail marks Results as superseded by a model change, the run stays as historical.

### 3.5 Disclosure homes

| ID | Home in Direction C |
|---|---|
| M-01 | About SWBPIPE, on demand; until the decision packet resolves the registry entry, the status bar's right-end popover carries the single sentence. |
| M-02 | Results page header information disclosure (one placement for the results surface class); Report stage preview as printed. Not on the Model or Loads pages. |
| M-03 | Report stage Outline as a fixed section and in the Preview; every generated report. |
| M-04 | Report Outline when hanger selection or handoff sections are included. |
| M-05 | Libraries and Rules dialogs from the rail's foot; export and redaction dialogs. |
| M-06 | Export and handoff dialog (File menu and Report stage). |
| M-07 | Report Outline shows every required item as a section; the Preview renders them. |
| M-08 | Status bar chip; the rail's Results item repeats the short label only as a caption, with the authority in the popover. |
| M-09 | Results page header chip; report case pages. |
| M-10 | Issues drawer on every page, grouped by class, linked to the row and the entity; count on the rail. |
| M-11 | Every column header, field, probe value, legend and report table. |
| M-12 | Model page inspector, Provenance disclosure; Libraries and Rules dialogs for imported records. |
| M-13 | Results page header banner for a reopened saved run; the rail's Results item captioned Historical. |
| M-14 | Agent column: Proposal cards, diff, Accept and Reject, rationale, assumptions, TBD; Accepted tab. |
| M-15 | Loads page Cases tab expression caption; Rules dialog editor. |
| M-16 | Preferences, Appearance. |
| M-17 | About SWBPIPE, Scope and limitations. |

### 3.6 3D presentation language

The canvas draws the model as it will appear in the report figure, so the engineer never has to wonder whether the reviewer will see something different. Pipes at real outside diameter with neutral shading and a light edge line; fittings as geometry (elbows at radius, tees, reducers, valves and flanges as standard silhouettes). Restraint glyphs are the standard symbols, the same ones the report legend explains: anchor block, +Y triangle, guide brackets, stop plate, spring can, with gap as clearance and friction as a hatch. Loads are arrows scaled by magnitude with the value label, coloured by the case they belong to when a case is selected on the Loads page. Node labels are on for every visible node up to a budget that prefers restrained, loaded and selected nodes, because the reviewer reads the figure by node number. Camera presets are shared between the canvas and the report figures, so Fit and the four presets produce the figure. Result colour on the tube with a legend whose scale and units are the report legend; deformation as a scaled shape with the factor stated in the legend, animation as a play control, and the same scaled shape is what the report figure prints.

### 3.7 Risks

- A stage rail can turn back into modes. If any stage acquires its own selection, undo or toolbar, the direction has failed in exactly the way Aspect Pipe Stress users complain about. The rule is that stages are filters over one spine.
- Loads on their own stage separates a load from the node it acts on. An engineer used to typing the load on the element row will look for it on the Model page. Mitigation: the Model page's marked cells show loads and open the same row; the wireframes and mocks must prove this.
- The report-figure rendering language is the plainest of the three and may read as dated next to B.
- The persistent agent strip is 44 px of every layout; at 1280 × 800 it competes with the rail for width.

---

## 4. Comparison

| | A Layout Sheet | B Workbench | C Run Book |
|---|---|---|---|
| Organising idea | Sheets joined on node addresses; canvas as the picture | One selection, one inspector; tables in drawers | Stages as pages beside a permanent model; report as the book |
| Feels most like | CAEPIPE | AutoPIPE, Onshape; Aspect Pipe Stress form in the inspector | Aspect Pipe Stress sequence with CAEPIPE's pairing |
| Grid and model equality | Grid is the spine; the model is co-equal by layout | Model is the spine; the grid is co-equal by layout | Both are co-equal; stages change the page, not the pairing |
| Canvas share, Model layout (drawer open, closed) | 50%, 70% | 43%, 66% | 42%, 63% |
| Canvas share, Grid layout | 34% | 26% (38% inspector collapsed) | 31% |
| Canvas share, Split layout | 46% | 44% | 43% |
| Agent panel | Right drawer over the canvas edge, 380 px | Third column beside the inspector, 340 px | Persistent right column, 340 px, collapsible to 44 px strip |
| How a proposal appears | Proposed rows in the sheet with old and new; ghost glyph | Diff card plus Proposed column in the inspector; ghost glyph | Diff card; proposed rows on the affected pages; ghost glyph |
| Results presentation | Results sheet with table tabs, case selector, probe in HUD | Results drawer plus probe tool and inspector Results section | Results page in reviewer order, Summary first; probe pane |
| Load case editor | Cases sheet | Load cases drawer | Loads page |
| Hanger design | Hangers sheet | Hangers drawer | Results page Hangers tab |
| Report | File dialog with preview | File dialog with preview | Report stage, outline and preview from the same pages |
| Where governance lives | Results sheet disclosure, About, Libraries, report | Results drawer disclosure, About, Libraries, report | Results page disclosure, Report stage, About, Libraries |
| Departure from the brief | None | Grid layout puts the canvas above the inspector at 26% | None in substance; right pane is stage-contextual |
| Strongest for | Volume input, CAEPIPE veterans, engineers who check models by reading rows | Routing and reviewing on the model, restraint placement, probing results | Running and reviewing whole analyses, report-driven projects, reviewers |
| Would fail if | The canvas-primary path feels like a viewer | The grid drawer feels like a demotion | Stages become modes; loads feel far from the model |
| Rendering language | Drawing with node labels as the address | Physical model under light, glyphs as hardware | Report figure; canvas and figure are the same picture |

## 5. Recommendation

Choose Direction C, Run Book, and carry two things into it from the others: A's sheet grammar for every page that holds rows (the Model page's Layout grid and Restraints tab, the Loads page, the Results tables and the Hangers table are all one sheet component with one keyboard model), and B's probe as a first-class canvas tool with its card at the cursor.

Reasons a stress engineer would recognise:

1. It is the order the work is done and the order it is checked. Model, loads, run, results, report is how an Aspect Pipe Stress user already thinks, and how a checker reads the calculation. The rail makes that order visible without making any step a mode, and the Run button with a stated reason for being disabled replaces the error-check gate with something honest.
2. The tables get the page they deserve while the model never leaves. The Results page in reviewer order, Summary first, is the deliverable set; the canvas beside it is the picture a reviewer uses to locate a number. That is the brief's principle 6 in its plainest form, and it avoids B's risk of a demoted grid and A's risk of results as one more sheet.
3. The report is a consequence, not a dialog. What the engineer sees on the Results page is what prints, with the notices and content block as fixed outline items. Credibility for this product is won in the report, and C is the only direction where the report is a place the engineer works rather than a thing they generate.
4. The agent has a permanent seat without being the hero. A proposal can concern geometry, a case or a hanger; a column present in every stage, collapsed to a strip when idle, is the right amount of visibility for a collaborator. A's drawer and B's third column both hide the agent until asked.
5. It satisfies all three layouts with the same pages, so the layout switch never changes the engineer's mental map, and its canvas shares are within a few points of A's in every layout.

What would change the advice. If the owner's own habit, running a real analysis, is to live in the canvas and route by pointing, B is the better fit and its Grid layout weakness can be accepted. If the owner's reviewers and the owner's own checking habit are row-by-row reading of the model, A is the better fit and the Results sheet should gain a case-by-report picker. C's own failure mode, stages hardening into modes, is a discipline the design system must enforce: one selection, one undo, one issues drawer, one Run button, and no stage-private toolbar.

## 6. Items for the owner's decision packet

Named here without proposing any change to governed text; the packet is prepared once the direction is chosen (brief §6).

1. The maturity line (M-01, brief §6 item 1). All three directions place it on demand in About and, until the registry entry changes, in the status bar popover. The packet needs the owner's ruling on whether an on-demand placement satisfies the shell banner requirement or whether the registry entry and lint anchor are revised.
2. Boundary sentence placement (M-02, brief §6 item 2). All three directions use one information disclosure on the results surface and the report. The packet needs the ruling that a disclosure counts as the surface carrying the sentence.
3. The product name (brief §6 item 3). Registered sentences and report notices carry the old name; the directions refer to them by ID only.
4. Short labels for the six automatic statuses and the two evidence labels. The directions show short labels with the raw token one click away (preserve list item 8). Whether those short labels are listed short variants of the registered texts is a registry question.
5. Vendor hanger tables as a library class. The directions treat them as imported libraries with provenance under M-05. The packet should confirm that framing against the hanger-selection item in the vocabulary (C §3, item 23).
6. The rail caption Historical and the page banner for a reopened run (M-13). The wording of the stated limit on the banner should be taken from the existing component, not written new.

## 7. Wireframe index

Nine low-fidelity wireframes for state 7 (results on the model and in the table), one per direction per layout, 1440 × 900 viewBox, grey boxes and labels only, drawn to be legible at half size. Each shows the toolbar band, the status bar, the canvas with a legend and probe, and the stress table with the columns named in the storyboards.

| File | Direction | Layout |
|---|---|---|
| `wireframes/layout_sheet_model.svg` | A Layout Sheet | Model |
| `wireframes/layout_sheet_grid.svg` | A Layout Sheet | Grid |
| `wireframes/layout_sheet_split.svg` | A Layout Sheet | Split |
| `wireframes/workbench_model.svg` | B Workbench | Model |
| `wireframes/workbench_grid.svg` | B Workbench | Grid |
| `wireframes/workbench_split.svg` | B Workbench | Split |
| `wireframes/run_book_model.svg` | C Run Book | Model |
| `wireframes/run_book_grid.svg` | C Run Book | Grid |
| `wireframes/run_book_split.svg` | C Run Book | Split |

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
