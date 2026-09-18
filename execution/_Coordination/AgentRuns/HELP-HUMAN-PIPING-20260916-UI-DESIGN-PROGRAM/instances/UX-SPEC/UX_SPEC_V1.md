# UX specification V1.1 — SWBPIPE

Status: V1.1, revision under sealed brief UX-SPEC-02, 2026-09-18, for ROOT's acceptance. What changed from V1: the owner's rulings of 2026-09-18 on decision packet D-71 are applied (the ruling record and its two addenda, codified as `DEC-099` to `DEC-105`; where they differ the addenda govern, and the rulings govern over the packet's recommendations and over anything V1 said); the document agrees with design system V1.2 by name, section and number; the answered questions Q-15 to Q-22 are specified as behaviour with their state tables, as defaults the owner expects to revisit in use; the pointer rule is applied to every key (§2.8); a run's three standings, Current, Stale and Historical, are specified separately wherever results are shown (§2.6); the one mutation route is stated in the handoff's words (§2.2, §2.4); every behaviour that differs in meaning from today's product is named as a change, with both sides cited (§12); and the operations map is re-checked against the current `HEAD`. Every change is listed in §13 with its source. The filename is kept so that other records' citations of `UX_SPEC_V1.md` still resolve, and no section of V1 is renumbered. V1 was written by the HELPS_HUMANS design manager under brief UX-SPEC-01 (phase 3 of the interface program), 2026-09-18, on the basis read in that brief's order: the design brief V1.4 (§2–§7), the direction record (§2, §5, §10, §11), design system V1, the fifteen mock frames with `MOCKS_V1.md` and `sample_model.md`, RESEARCH-E §3 and §6, RESEARCH-C §1, §3 and §4, RESEARCH-B, and the product source named in the brief, read only for the names of operations, types and schemas. V1.1 additionally read design system V1.2 with its change log from row 50 and its return, the D-71 ruling record and its two addenda, `DEC-099` to `DEC-105`, ROOT's revision record §1, §2 and §6, RESEARCH-G §1 and §2, the implementation-handoff preparation §2 and §4, and the product source at `HEAD` for what the product does today.

Companions: [`OPERATIONS_MAP.md`](OPERATIONS_MAP.md) maps every control and gesture in this document to the operation it invokes and to the existing operation, capability entry or typed interface that provides it, or names the gap. [`RETURN.md`](RETURN.md) records what was read, what was produced and what is uncertain.

Nothing in this document is a product claim. It describes the intended behaviour of a redesign that is not built, over an engine that stays as it is.

---

## 1. Scope and reading guide

**What this document specifies.** Behaviour: what every surface does, in which states, in response to which events, and which operation of the existing product each control invokes. It covers the shell (rail, views, toolbar, status bar, palette, menus), the tables of the four stages, the canvas, the inspector, issues, the run, the agent panel and its protocol, the Review page, libraries, rules, export and handoff, the mandatory disclosures, the empty, error and edge states, and the behaviours that differ in meaning from today's product (§12). Every string it specifies is bound by the copy rules in §9.3.

**What it leaves to the design system.** Appearance: tokens, type, spacing, colour in light and dark, iconography, glyph forms, motion, densities and the shell geometry (design system V1.2 §0–§6). This document names components, tokens and controls by design system V1.2's names and numbers and states only what they do; "design system" below always means V1.2.

**What it leaves to the engine.** Numerics and semantics: how a mechanics solve, a rule evaluation, a hash, a unit conversion, a file grammar or a diagnostic is computed. The specification names what the interface asks of the engine and never how the engine does it. Where the engine has no operation for a behaviour, the operations map says **GAP** and §11 states the question.

**Precedence.** The owner's rulings of 2026-09-18 (the D-71 ruling record and its two addenda, `DEC-099` to `DEC-105`); then design brief V1.4 §2–§7; then the direction record §2, §5, §10 and §11; then design system V1.2; then the mock frames as worked examples. Where a frame drew a moment the design system had not specified, the frame's decision is adopted here unless §11 of the direction record, design system V1.2 or this document says otherwise. The answers to Q-15 to Q-22 are defaults: the owner expects to revisit details of this kind once the product is in use.

**How to read a state table.** Each surface has a state table with four columns: *State*, the name used elsewhere in this document; *Shown*, what is on screen in that state; *Enabled*, which controls act; *Leaves on*, the events that move it to another state, written `event → next state`. Events are user actions (a click on a control, its accelerator key, a gesture), engine returns (a solve completes, the engine rejects an operation) or agent actions (a proposal lands). A state table lists only what differs between states; the surface's constant behaviour is in the prose above it.

**Notation.** Keys are written with the macOS symbols the design system uses: ↩ Return, ⇥ Tab, ⎋ Escape, ⌫ Delete, ⌘ ⌥ ⇧ ⌃ modifiers, ← → ↑ ↓ arrows, Space. A key is always an accelerator for a control that exists (§2.8); where a sentence names a key alone, the control is the one §2.8's rule and the section's own table name. "Row operation" means one structured operation through the one route of §2.4. "Table" means the one table component of design system §5.1 unless a named instance is meant. "The run" means the Current run, the current solve basis (§2.6); a Stale or a Historical run is always called so. "The model batch file grammar" is this document's name for the layout grammar RESEARCH-E documents; the product never names another vendor's product (§8.3). Units are SI in every example; the display-units toggle changes presentation only (§2.7). Node numbers in examples are the sample model's.

**Source citations.** Paths are relative to the repository root and carry a line number, for example `projects/chirality-piping/apps/desktop/src/types.ts:645`. A `:line` written alone refers to the file cited immediately before it in the same sentence or cell. They name existing operations, capability entries, typed interfaces and schemas; nothing of the current presentation is carried over. Every citation was re-verified at the current `HEAD` for V1.1 (`RETURN.md` §3). A path that carries a name the rulings remove from product copy is a citation of the source as it is, not product copy.

---

## 2. The model of the interface

### 2.1 One spine

Every surface reflects five things, and only these, so that no two surfaces can disagree:

| Spine element | What it is | Who owns it | Where it shows |
|---|---|---|---|
| **Model** | The set of tables (§3): layout, restraints, loads, node data, load sets, load cases and combinations, plus the project's library and rule-pack references. The canvas, the inspector, the issues list and every export are derived from it. | The engine's model document, mutated only through the one route (§2.4). | Every table; the canvas; the inspector; the footer counts. |
| **Selection** | One ordered set of selected entities (rows) with one primary entity, and one focused cell. | The interface, product-wide. | Table selection bands, canvas halos, the inspector, the status bar ("Node 40 · 1 row"), the palette's context. |
| **Issues** | One live list of findings against the model and the run, each classed and pointing at its entity (§5.2). | Derived: model validity from the operation layer, missing data from the readiness check, run findings from the run record, provenance and content findings from libraries. | The issues drawer, the rail's Issues count, the toolbar count, the status bar count, gutter and cell marks, canvas halos. |
| **Run** | The current solve basis: the run made on the model as it is now, with no operation applied since, or none. A run is an immutable, named record (§5.3) and is shown in one of three standings, Current, Stale or Historical (§2.6); only a Current run is the solve basis. | The engine's run record. | The Run button, the status chips, the results tables, the legend and probe, the Review page's live tables, the rail caption. |
| **Revision** | The model's content hash after the last applied operation, and the sequence number of that operation. | The engine (hash), the ledger (sequence). | The project save state, the changes-since control, the stale band, the Checked mark's binding. |

Stages are not modes: switching stages changes which tables are on screen and nothing in the spine.

### 2.2 The tables and the canvas are projections of one model

"The tables are the model" describes the table-first experience: every entity the product knows is a row in exactly one table, and a row has no hidden fields: what the inspector shows for a row is the row's cells plus the row expansion's fields (§3.2). Stated exactly, there is one canonical model, the engine's model document, and the tables and the canvas are both projections of it and editors of it. Neither holds state the model does not; neither is independently mutable; neither is a rival copy that can diverge from the other. The canvas draws the rows and holds nothing they do not; it can select, probe, and start row operations through its few gestures (§4.4), and nothing else. A value the engineer sees in the canvas, the inspector, a results table, a report or an export can always be found in a table cell or in a run record.

Every engineering action, by a table cell, a canvas gesture, a paste, an accepted proposal or a future harness, is a typed operation through the existing applier (§2.4). There is no second route for the interface. A human's action and an agent's action are semantically the same operation: the same change kinds, the same checks, the same checkpoint and the same receipt, differing only in the author type the operation carries and in the acceptance record of an accepted proposal (§6.3). The agent acts only through row operations (§6).

### 2.3 Stages and views

Four stages on the rail, in the order the work is done and read; three views per stage.

| Stage | Tables (tab strip order) | Canvas shows | First-open view |
|---|---|---|---|
| Model | Layout · Restraints · Node data | Geometry, restraint glyphs, load vectors, labels, the routing draft, proposal ghosts | Both |
| Loads | Cases · Load sets · Loads · Wind · Seismic | Geometry with the selected case's loads in the case's colour and every other load at 40 % | Table |
| Results | Summary · Stresses · Displacements · Restraint loads · Restraint summary · Forces and moments · Hangers | Result colour, legend, probe, deformation | Both |
| Review | The Review page (§7); no tab strip | No canvas; the report figure is a block in the page | Table |

At the rail's foot: Libraries, Rules, Issues (§8, §5.2). Libraries and Rules are pages that open over the surfaces of the current stage and return to it by the rail item or the page's close control (⎋ accelerates it); Issues opens the drawer (§5.2). They do not change the stage's remembered view.

**Views.** Table: the stage's tables at full surface width, no canvas; the inspector is not shown and the row expansion (§3.1) serves its purpose. Model: the canvas with the inspector always docked on its right (340 px) and the stage's current table in a bottom drawer (280 px, collapsible to its 28 px tab strip by the chevron at the strip's right end). Both: table and canvas side by side at the remembered split (55 % / 45 % by default); the inspector docks on the canvas's right at 300 px (`layout.inspector.both`), opened and closed by the toolbar's Inspector toggle, and the canvas shrinks while it is open; the table never reflows (direction record §11 decision 1; the narrow cases are §10.9). The view switch is the three-segment control in the toolbar (⌘1 ⌘2 ⌘3 accelerate it).

**The per-stage memory rule.** Each stage remembers the view it was last left in, for the project, and restores it when the stage is re-entered. Before a stage has ever been left, its first-open default applies. The memory is saved with the project's interface state, never with the model.

**What persists across a stage or view switch.** The selection, the undo stack, the issues list and the run with its standing (the spine); each table's scroll position, sort, filters and column widths; the canvas camera with its fitted state (§4.6), label mode, hidden and isolated sets, deformation state and legend range; the inspector's open state per view; the drawer heights; the agent column's open state (product-wide). The probe's pinned card persists while the Results stage is current and closes when the run changes or stops being Current. A stage or view switch is a presentation change: it never changes a run's standing (§2.6).

**The rail's states.** Results is disabled until there is a run to show (the first run of the project completes, stops or fails, or a saved run is reopened), with the tooltip "No run yet". Review is enabled by a Current solved run and stays reachable while that run is Stale, so that writing is not interrupted; before the first run its tooltip is "No run yet", after a failed run with no solved run "No solved run", and when the only run is a Historical one "Needs a current run": a Historical run enables nothing (§2.6). Captions under a rail label carry a state when there is one to name: Results carries "Failed" after a failed run, "Stale" when the model changed since the run (§2.6), "Historical" when a reopened saved run or an earlier run from the run list is shown (M-13). A caption is a state name, not a status. The Issues item carries the issue count.

### 2.4 The one route

Every mutation of the model is one structured operation, or one batch of structured operations, submitted to the engine's operation layer, checked by it against the schema, the constraints and the units, and applied only by it. The route is the one that exists: the typed operation intent applied by the existing applier, one operation through `applyModelOperation` and several through `applyOperationBatch` (the operations map's reading guide cites both). Every engineering action takes it, whatever surface or actor starts it: a table cell, a canvas gesture, a paste, an accepted proposal, or a future harness driving the product without the interface. The routes that produce operations are:

| Path | Produces | Shown before apply as | Applies on |
|---|---|---|---|
| A typed cell, a row insert, a row delete, a row-expansion field, an inspector field | one operation | the cell in its editing state; the value as typed | a click on another cell, the footer's edit chip (Commit), or focus leaving the cell; ↩, ⇥ and an arrow commit accelerate |
| A canvas gesture (route, add restraint) | one operation or one batch | the draft ghost and the draft row (§4.4) | the gesture's Place or Add control (↩ accelerates) |
| A paste | one batch | the paste band with mapping and preview (§3.1) | the band's Paste button (↩ accelerates) |
| A generation (load cases from the rule pack, self-weight case) | one batch | the generation band, same anatomy as the paste band (§3.5) | the band's Generate button (↩ accelerates) |
| A library application (a section or material chosen in a cell, a hanger size chosen) | one operation | the cell or the candidate row | the choice |
| An accepted proposal row, rows or batch | one batch per accepted unit | the banded rows, the ghost and the card diff (§6.3) | Accept |
| Undo, redo | the inverse of one checkpoint | — | the toolbar's Undo and Redo (⌘Z, ⇧⌘Z accelerate them) |
| A future harness (a scripted or headless driver) | the same operations and batches | whatever the harness shows | the harness's submission: the same applier, the same checks |

The engine checks every operation against the schema, the constraints and the units before it is applied; the interface never mutates the model itself, never keeps a second copy that can diverge, and never applies an operation the engine rejected. A rejected operation shows where it came from: an invalid cell (§3.1 cell states), a blocked paste band, a proposal card that "cannot be accepted", a gesture hint. Diagnostics the engine returns with an applied operation join the issues list.

Author attribution and semantic equivalence: an operation carries its author type, the engineer or the agent, and nothing else about it depends on who made it. A gap typed by the engineer and the same gap proposed by the agent and accepted are the same change kind on the same target with the same checks, the same checkpoint and the same receipt; that equivalence is a property the implementation tests, not a convention. An accepted proposal is applied as the agent's operation accepted by the engineer, never rewritten as the engineer's own (§6.3).

### 2.5 The undo model

One undo stack for the whole product, across stages and views. One checkpoint per applied operation or batch:

- A typed cell is one checkpoint. A propagation that follows a typed cell (§3.2) is part of the same checkpoint.
- A paste is one checkpoint. A generation is one checkpoint.
- A route commit (node, element and any bend the direction change inserted) is one checkpoint.
- An accepted proposal row is one checkpoint; an accepted selection of rows is one; Accept all is one.
- A row delete with its attachments is one checkpoint.

Undo and Redo are two icon buttons in the toolbar (§5.5; ⌘Z and ⇧⌘Z accelerate them; each is disabled with "Nothing to undo" or "Nothing to redo", and its tooltip names the operation). Undo restores the model and the selection as they were before the checkpoint and shows the toast "Undid: Gap at node 20 · 3 mm ← 0 mm" with Redo; Redo reapplies. Undo never re-establishes a run: an undo is itself a model change, so a run made before it is Stale (§2.6) even when the undo returns the model to the content that run solved, and results are not restored by undo; the product behaves this way today, clearing computed state on undo and redo as on any other edit (§12). Undo is never offered for a run, a snapshot, an export, a library import or a Checked mark, which are not model edits; the Checked mark's currency follows the content (§6.7), so undoing a change may make a stale mark current again. Diagnostics are recomputed after every undo and redo.

### 2.6 Run standing: what happens to results when the model changes

A run binds to the model hash it solved. A results surface shows a run in one of three standings, and what the run may drive depends on its standing (design system §5.1, run standing; ruling 6; R-9).

- **Current.** The run was made on the model as it is now and no operation has been applied since. It is the current solve basis.
- **Stale.** The model changed since the run. The run stops being the current solve basis at the moment the operation is applied; this is how the product already behaves, clearing the solve proof and the model hash when a model commit lands (§12). The run record is kept unchanged, and the run is held on a Historical record's terms, with its own band.
- **Historical.** A reopened saved run, or an earlier run chosen from the run list (§5.3).

The rule is one: result colour, deformation and probed values are an overlay on the current model, and only a Current run may drive an overlay, a status chip, an evidence chip or any other cue that the current model has results. A Stale run and a Historical run drive none of them; their values are read in the tables, under their band.

| Surface | Current | Stale | Historical |
|---|---|---|---|
| Results header line (§3.6) | table, run, time, "immutable"; the case selector and Envelope; the evidence chip when the run record carries evidence; the Run identity control | the same line, without the evidence chip | the same line, without the evidence chip |
| Band under the header | none | the stale band: "Model changed since Run 03: gap at node 20 set to 0 mm at 16:31. The values below are from the model as solved." with "Run again" at its right; where the band has one line, "Model changed since Run 03 · gap at node 20 set to 0 mm at 16:31 · Run again" | the historical band, in the product's rendered text: "Historical saved run · Run a fresh solve to establish current results."; its information control opens a popover that carries "Historical results cannot drive current overlays, rule checks, comparisons or report readiness." and, under it, the run's recorded statuses as labels with their domains and raw tokens (§5.4) |
| Result cells | plain | hatched (cell state *Stale*), readable and copyable | plain under the band; readable and copyable |
| Status chips (§5.4) | the chips of the policy | no chip | no chip from the run; its recorded statuses are read in the band's popover and on the run record |
| Evidence chip (M-09) | beside the run name and in the probe's footer, when and only when the run record carries evidence | none | none |
| Rail's Results caption | none | Stale | Historical |
| Legend (§4.5) | the scale | a note card with no scale, no range and no marker: "Model changed since Run 03" and "No result colour on the current model" | a note card: "Historical saved run · Run 02" and "No result colour on the current model" |
| Canvas colour | result colour on tubes and fittings; deformation on request | the neutral figure: no result colour, no deformed shape; the HUD's Deformation tool disabled with "Needs a current run" in its tooltip | the same neutral figure; the same tool disabled with the same reason |
| Probe (§4.3) | reads the results row under the cursor | the Probe tool disabled with "Needs a current run"; a pinned card closes when the run stops being Current | the same |
| Hanger table (§3.7) | design values and selections plain | hatched; State reads Stale | plain under the band |
| Review page live blocks (§7.3) | live from the run, plain | the block carries the stale band and its cells the hatch | the block carries the historical band |
| Report readiness (§7.6) | the Review rail item is enabled; Report preview, its Print and Save, and the report package are available | no readiness cue is lit by the run; the Review page stays reachable so that writing continues; Report preview opens and prints the stale band on every live block; Print, Save and the report package are disabled with "Needs a current run", and the band's "Run again" is the way forward | no readiness cue and nothing enabled by the run: the Review rail item is not enabled by it, Report preview shows the historical band on every live block, and Print, Save and the report package are disabled with "Needs a current run"; a results export of the Historical record itself stays allowed and names it Historical (§10.4) |
| Run button | enabled, or disabled by a blocker of the solve | enabled if nothing blocks it; a new run becomes Current and the Stale run stays in the run list | enabled if nothing blocks it; a new run becomes Current |

The stale band's sentence names the change with its time: for an engineer's edit "gap at node 20 set to 0 mm at 16:31", for an accepted row "proposal P-12 row 1 accepted at 16:31 (variable spring at node 80)"; after several changes it names the first change since the run and the count ("and 3 more changes"). No sentence on any surface says that a run remains the basis of anything after the model has changed.

**Transitions.**

| From | Event | To |
|---|---|---|
| no run | a run completes | Current |
| Current | any operation is applied (a cell, a gesture, a paste, a generation, an accepted proposal row, an undo, a redo) | Stale |
| Stale | a run completes | the new run is Current; the earlier run stays in the run list and is Historical when chosen from it |
| Stale | undo, redo, or an edit that returns the content to what the run solved | Stale: a run's basis is given up when the model changes and is never re-established by a later change (§2.5) |
| Current or Stale | an earlier run is chosen from the run list | that run is shown Historical; the latest run keeps its own standing and returns when chosen again |
| project opened with a saved run | — | Historical, always: reopening never establishes a current solve basis, whether or not the saved hash matches the model |
| Historical | any presentation change: a stage or view switch, a theme or units change, a camera or label change, a sort or filter, Envelope, the run list, reopening the project, docking a panel | Historical: a presentation change never makes a run Current, and never gives it an overlay or a readiness cue |
| Historical or Stale | a run completes | the new run is Current |

Results are never shown against a model they were not computed from without their band; a run's results are never silently reused; only a completed run on the current model produces a Current standing. What today's product does at each of these points, and what this design changes, is §12 item 4.

### 2.7 Display units and precision

The units toggle in the toolbar (SI, US, or the entered units) changes presentation only: no stored value changes, every value is converted deterministically for display, and a value whose conversion is unavailable shows in its entered unit with a tooltip that says so. Column headers, inspector rows, the probe, the legend and reports carry the displayed unit. Precision is one setting per column (design system §1.1) and applies to display, copy and report tables; the stored value keeps its entered precision.

### 2.8 The pointer rule

Every action has a visible primary control operated by a mouse click; a key is an accelerator for a control that exists, never the only way (design system §5). Its source has two parts, kept apart here. The owner's words were about Q-20 only: "for `Q-20` I don't want the engineer to have to press buttons. Keyboard inputs are acceptable but it needs a primary control via mouse click." Applying them as a general rule of the design, to every action and not only to Fit, is ROOT's reading, stated in the D-71 ruling record so that the owner can correct it; design system V1.2 and this document apply that reading. The control is on the surface where the action applies (a button, a HUD tool, a toggle, a chevron, a footer control, a link in a popover) for anything in the working loop; the menu bar and the command palette also list every command and are the home of the commands outside the working loop. A double-click, a modifier-click, a hover and a drag may accelerate too, but none of them is the primary control, because none of them is visible. Typing a value is input, not an action: the control for it is the cell or the field, which a click focuses. The engineer is never asked to press a key in order to see, fit, open, close, place, send, check or undo anything.

Consequences for this document. Every keyboard table below has a column naming the control each key accelerates. A state table's "Leaves on" events name the control; its key is the accelerator. In copy, a tooltip reads the control's name and then its accelerator in parentheses ("Fit (F)", "Inspector (⌘I)", "Place node 50 (↩)"); a toast, a band and an empty state name the control to click, never a key to press; the hint strip lists a tool's accelerators, and every key on it has its control on the canvas or in the inspector; "Press" is not a word the product uses (design system §7.6). Closing is an action too: every dialog and sheet has a visible Cancel or Close button, every drawer and expansion a close or collapse control, every pinned card and toast a close control, and every popover, menu and the run log closes by a click outside it; ⎋ accelerates each of them. Every key in §3, §4, §5 and §6 was audited against the rule; the audit, with the control each key-only action was given, is in `RETURN.md` §4.

---

## 3. The tables

### 3.1 What every table does

One component serves every table (design system §5.1). This section states the behaviour common to all instances; §3.2–§3.8 state each instance's columns, validation and specifics.

**Header.** The column name with its unit in square brackets. The gutter's header cell is the select-all control. A header cell carries the sort indicator (two small arrows, the active direction filled), which a click cycles, and the column menu button, drawn on hover and on focus and opened by a click (⌥↓ accelerates it; ⌘↓ on a focused cell of the column sorts by it). The column menu: Sort ascending, Sort descending, Data bar (numeric columns with a scale; ratio columns also "1.0 tick"), Colour by scale (ratio columns), Filter… (a comparison and a value for numeric columns; a value list for enumerated ones), Unchecked rows only, Precision, Unit (display), Hide column, Pin column, Width, Reset column, Export table…. Sort is a view: it never reorders the model's rows; the footer shows a "Sorted by ratio" chip with a clear control, and ↩-down movement follows the file order even while sorted. Filtering hides rows; the footer says "3 of 15 rows".

**Selection.** Clicking the gutter beside the glyphs selects the row; the gutter's header cell selects all rows (⌘A accelerates it); the clear control on the footer's selected count, or a click on any cell, clears a multi-row selection (⎋ accelerates). ⇧-click extends and ⌘-click toggles, with the gutter as their control; ⇧↑ ⇧↓ extend from the keyboard. Clicking a cell focuses it and selects its row; a click on the focused cell, or a double-click on any cell, starts editing. The selected rows are the product selection (§2.1): a layout row selects its node and the element arriving at it; a restraint row selects that restraint; a load-case row selects the case; a results row selects the element or node it reports. A joined row's selection is the attachment row, and its node row shows the selection band lightly to say where it belongs.

**Keyboard model.** The spreadsheet idiom of design system §5.1, fully operable without the mouse and, by the pointer rule (§2.8), fully operable without the keyboard except for typing a value. Every key accelerates a control; the last column names it. The table restates the design system's and adds this document's rows.

| Key | Not editing | Editing | The control it accelerates |
|---|---|---|---|
| a character | starts editing, replacing the value | inserts | the cell: a click on the focused cell, or a double-click on any cell, starts editing |
| ↩ | starts editing the focused cell; on the last row's last editable cell of the layout table, adds a row (§3.2) | commits and moves down one row (⇧↩ up) | the cell; the footer's edit chip, Commit; a click on another cell commits; the "Add row" line |
| ⇥ ⇧⇥ | moves right / left; wraps to the next row's first editable cell | commits and moves right / left | a click on the target cell |
| ← → ↑ ↓ | move the focus one cell; ← from the Node cell focuses the gutter; → from the last column focuses the marks column | ← → move the caret; ↑ ↓ commit and move | a click on the target cell, glyph or slot |
| ⎋ | clears the multi-row selection; then closes the open row expansion, drawer or popover: one level per press | cancels the edit and restores the value | the footer's edit chip, Cancel; the clear control on the selected count; the expansion's or the drawer's close control; a click outside a popover |
| ⌘↩ | opens the row expansion (on a marks slot, that mark's joined row; on the Type cell, the element fields; on a load-case row, the combination editor; on a hanger row, hanger selection) | commits, then opens | the marks slot; the expansion chevron in the cell that owns the expansion |
| ⌥↩ | inserts a row below with the next node number | commits, then inserts | the footer's selection group, Insert row below; the "Add row" line that ends every editable table |
| ⇧↑ ⇧↓, ⇧-click | extend the row selection | — | the gutter, with ⇧ held |
| ⌘A | selects all rows | selects the text | the gutter's header cell |
| ⌘C | copies the selected rows as tab-separated text with a header row, in display units and column precision | copies text | the footer's selection group, Copy rows |
| ⌘V | pastes; a single value pastes into the focused cell; more than one row, or a header row, opens the paste band | pastes text | the footer's selection group, Paste |
| ⌫ | on a row selection: deletes the rows with an undoable toast; on a cell: clears the value (an empty required cell shows the required mark) | deletes text | the footer's selection group, Delete rows; for a cell, editing it and clearing the text |
| Space | on a gutter or marks slot: opens the mark's popover; on a check-box cell: toggles | inserts | a click on the glyph or the box |
| ⌥I | opens the row's marks popover listing every mark the row carries | — | "All marks on this row", the last line of any mark's popover |
| ⌘⇧A ⌘⇧R | Accept, Reject the focused proposed row (or every selected proposed row) | — | the card's row buttons; the footer's Accept row and Reject row chips while proposed rows are selected (§6.3, §6.4) |
| ⌘⇧K | marks the selected rows Checked; again clears | — | the faint check in an empty state slot; the footer's selection group, Check rows; the Checked popover's Check again and Clear check buttons; the inspector's Check control |
| ⌘Z ⇧⌘Z | undo, redo | text undo, redo | the toolbar's Undo and Redo |
| ⌘K | the command palette | — | the toolbar's palette field |
| ⌘F | focuses the footer's Filter control | — | a click on the footer's Filter control |
| ⌘↓ ⌥↓ | sorts by the focused cell's column; opens its column menu | — | the header's sort indicator; the header's column menu button |
| ⌘⇧S | Split element… on the focused row (§3.2) | — | Insert › Split element… in the menu bar; the palette |


**Cell states** (design system §5.1): Entered, Propagated, Read-through, Proposed, Selected, Editing, Invalid, Required-empty, Stale, Historical, Disabled, Draft. While a cell is being edited the footer holds the edit chip ("Editing DX · node 70" with Commit and Cancel), the pointer controls for committing and cancelling. Cell rules apply on commit: a cell that fails the interface's own rule (type, range, enumeration, uniqueness, reference) becomes *Invalid* with the message under it and keeps the typed text until corrected or escaped; a cell the engine rejects after commit reverts to its previous value and shows the engine's diagnostic in the same popover and in the issues list. A committed cell whose value the engine accepted with a warning (out of range, provenance) keeps the value and gains the warning corner.

**Validation vocabulary.** Numbers accept a leading sign, a decimal point and, when the display unit is US and the column is a length, the feet-inch-fraction forms RESEARCH-E §3.5 documents (`10'8`, `1'6-3/8`); a typed unit suffix that matches the column's dimension is accepted and converted for display, and one that does not is *Invalid* ("Expected a length"). Enumerations are comboboxes that also accept typed text with completion. References (a section name, a material name, a load set, a node) must resolve; a reference that does not resolve is *Invalid* with "No section named P4 · Libraries" as a link. Nothing is ever pre-filled to satisfy a rule (C-26 to C-29).

**Gutter and marks column.** As design system §4: the origin slot and the state slot on the left; the three attachment slots on the right (layout table only). Pointer reveal is the same for every mark: a click on a gutter glyph opens that mark's popover with its tooltip text and, as its last line, the link "All marks on this row", which opens the row's marks popover; a click on the gutter beside the glyphs selects the row; a click on a marks-column slot opens that attachment's joined row. An empty state slot shows a faint check on hover ("Check"); a click sets the Checked mark, and a Checked glyph's popover carries Check again and Clear check as buttons (§6.7). An empty marks slot shows a faint plus on hover with "Add restraint…", "Add load…" or "Add node data…"; the plus opens the joined row empty in Table view and the inspector's attachments section in Model and Both. Space on a focused slot and ⌥I accelerate the reveals.

**Row expansion.** A click on a marks slot, or on the expansion chevron of a cell that owns an expansion (⌘↩ accelerates both), opens a block under the row, indented by the gutter, with a caption line. The chevron sits at the right edge of the owning cell (Type, Expression, a hanger row's Size, a Stresses row's Stress) and is drawn on row hover, on selection and whenever the block is open. Four things open this way with the same anatomy: a joined attachment row (the attachment table's own header and the node's rows in that table, editable in place with the same keyboard model, and an "Add restraint…" row at its end); the element fields of the Type cell (§3.2); the combination editor under a load case (§3.5); and hanger selection under a hanger row (§3.7). A block closes by the chevron on its caption line or the close control at the caption's right end (⎋ from inside it, or ⌘↩, accelerates). One expansion opens per click and stays until it is closed, so several may be open at once; the footer counts the open ones ("2 joined rows open"). In Model and Both views the same controls open the row in the docked inspector instead of expanding.

**The paste band.** A paste of more than one row or of a header row (the footer's Paste; ⌘V accelerates it) opens a band above the target rows on `surface.raised`: a mapping row in which each source column names its target in a combobox, a count ("42 rows from the clipboard · after node 130 · 2 columns ignored · node IDs taken from the source"), a preview of the first three rows in the target grammar drawn as draft rows, and two buttons, Paste *3 rows* and Cancel (↩ and ⎋ accelerate them). Recognised source headers are pre-mapped, case-insensitively, including the column names of the model batch file grammar's layout: Node, To, From, Type, DX, DY, DZ, X, Y, Z, Section, Sect, Material, Matl, Load, T1…T10, P1…P10, Note, Comment. An unrecognised column is offered "Ignore" or any target. The first row is treated as a header when at least half its cells match a header name; otherwise the mapping row shows column letters. Node IDs are taken from the source when the mapped Node column has values, and auto-incremented when it does not. A source row that would be invalid (a section name that does not resolve, a From that does not exist) is shown invalid in the preview; Paste stays enabled, and the offending cells land empty with the required mark and an issue, because a paste never fabricates a value. The paste applies as one batch and one checkpoint; the rows carry the Entered origin with the paste time.

**Footer.** One line (design system §5.1). The counts: rows and the selected count ("16 rows · 1 selected") with a clear control on the count. While rows are selected the selection group follows it, five compact icon buttons with tooltips: Insert row below, Delete rows, Copy rows, Paste, Check rows; while proposed rows are selected the Accept row and Reject row chips join it (§6.4). While a cell is being edited the same place holds the edit chip instead ("Editing DX · node 70" with Commit and Cancel). The selection group and the edit chip take the place of the counts that follow, which return when the selection clears. Then the counts of what the table holds, each a filter when clicked (propagated cells, proposed rows, unchecked rows, checked and stale, issues on this table); then the Filter control (⌘F focuses it) and the active filter chips with a clear control; then the Changed since… control (§6.8); and on the right the table's switches (Read-through, Origins) as outline chips. Sorted, filtered, envelope and expansion states show as chips here. The row actions live in the footer because the strip above the table belongs to the stage's tabs and never grows a toolbar.

**Filters common to every table.** Unchecked rows; Agent-origin rows (origin Accepted or Proposed); Changed since…; Issues (by class); Origins (Entered, Accepted, Propagated, Generated, Imported); and the table's own (§3.2–§3.8). Filters combine; the footer says how many rows remain.

**Empty state.** A table with no rows shows one line of hint text under the header row in `text.secondary` naming the way to add the first row (each instance's wording is in its section) and, in the layout table, the first row already present (§10.1).

**Copy and export.** Every table can be copied (the footer's Copy rows; ⌘C) and exported from its column menu's "Export table…" as CSV or JSON with the display units and the table's provenance line (run identity for results tables); results exports carry the run's binding set (C-98, §5.3).

### 3.2 The layout table

One row per node, carrying the element that arrives at it, in the model batch file grammar (RESEARCH-E §3.2, §6.1). Rows are in file order: the order they were created or inserted; the order is the export order and never changes by sorting.

| Column | Type · unit · precision | Validation | Propagation | Origin behaviour |
|---|---|---|---|---|
| gutter | origin and state slots | — | — | row origin in the origin slot; state slot priority per design system §4 |
| Node | positive integer; no unit | unique in the model; > 0; editing renumbers every reference to it (From cells, attachment rows, loads, node data, load-case terms that name it) in the same operation | — | Entered; Imported when pasted with source IDs |
| From | integer | must name a node whose row is above this row; empty only on a start row | implied value is the previous row's node, shown in `text.muted`; a typed value that differs is a branch, shown in `text.primary` | typing over an implied From makes it entered; an implied From has no origin |
| Type | enumeration: Pipe, Bend, Valve, Reducer, Rigid, Expansion joint, Flange-bearing kinds are node data, not types | one of the offered kinds; Pipe is the value of a new row (it is the file's blank and is an entered value, not a default that hides a decision) | — | Entered |
| DX, DY, DZ — or X, Y, Z | number · length · column precision | a number; at least one non-zero on an element row (a zero-length element is *Invalid*: "Element has no length") | — | Entered |
| Section | reference to a section record | must resolve to a section in the project's libraries | propagates by connectivity (below) | Propagated until typed over |
| Material | reference to a material record | must resolve | propagates | Propagated until typed over |
| Load | reference to a load set (§3.4) | must resolve | propagates | Propagated until typed over |
| T1 … Tn, P1 … Pn | read-through: the named set's values · °C, bar · 0 and 2 decimals | numbers; editing applies the edit-or-fork rule (below) | read through, not propagated | the set's origin |
| marks | restraint, load, node data slots | — | — | — |

**From shown and implied.** Every row shows From. A row whose From was not typed shows the previous row's node in `text.muted` and exports as a continuation (`T` row); a row whose From was typed and names an earlier node shows it in `text.primary` and exports as a re-anchor (`F` row followed by the `T` row). A typed From that equals the previous row's node is treated as implied.

**Start rows.** The first row of the model, and any row whose From is cleared, is a start row: it carries no element, shows "—" in Type, Section, Material, Load and the read-through cells, and its DX DY DZ cells are the node's absolute coordinates, marked with the absolute corner tick and the tooltip "Absolute coordinates: this row starts a run". The required marks for Section, Material and Load sit on the first element's row, never on the start row (direction record §11 decision 7). A second start row creates a second disconnected run; the model is valid with several runs.

**Offsets and absolute coordinates.** The DX DY DZ group and the X Y Z group are one switchable column group; the header shows which. In offsets mode a cell is the To node's offset from the From node; committing an offset moves the To node and every node downstream of it along the From chain by the same vector, so that the rest of the run keeps its shape, as an engineer used to offset-entry layout tables expects; the toast says "Moved node 40 and 12 downstream nodes". In absolute mode a cell is the node's coordinate; committing it moves only that node, and the offsets of its neighbours change. Both are one operation each (a batch of node moves in offsets mode, one checkpoint).

**Branches.** A branch is a row whose From names an earlier node. It carries offsets from that node. The branch-connection kind at the From node (welding tee, sweepolet, weldolet, fabricated tee, extruded tee, radiused branch, branch on thickened pipe) is node data on the From node's row (§3.3), and a branch whose From node has no branch-connection kind raises the issue "Blocks rule check · Branch connection kind missing · node 70" (a SIF is rule data, never inserted silently, C-28); the solve is not blocked.

**Element types and their type-specific fields.** The Type cell's expansion (the cell's expansion chevron; ⌘↩ accelerates it) and the inspector's element section carry the fields of the element's type, every one user-entered, none pre-filled:

| Type | Fields in the expansion |
|---|---|
| Pipe | none beyond the row |
| Bend | Bend radius [mm] (the section's bend radius when the section record carries one, shown as propagated; else required); bend wall thickness; bend material; flexibility factor (user value); SIF in-plane and out-of-plane (user values); intermediate nodes with angle (up to two) |
| Valve | Weight [N]; thickness factor; insulation factor; additional weight; offset (x, y, z) |
| Reducer | OD at end 1 and end 2 [mm]; wall at end 1 and end 2 [mm]; cone angle [deg] (each end's values propagate from the sections on either side and are shown propagated) |
| Rigid | Weight [N] |
| Expansion joint | Axial, lateral and torsional stiffness [N/mm, N/mm, N·m/deg]; pressure thrust area [mm²]; bending stiffness; weight |

The Type enumeration offers exactly the kinds the engine represents, and no others (R-4; design system §5.1); the kinds the target file knows and the engine does not (slip joint, ball joint, hinge, tie rod, elastic element, beam, cut pipe, jacketed pipe, jacketed bend, miter bend) are not offered, and the enumeration's footer line says "Other element kinds are recorded scope decisions · About › Scope and limitations" (C-23). They are gaps or non-goals in the operations map.

**Propagation rule.** Section, Material and Load propagate by connectivity: a row without an entered value takes the value of the row of its From node (its previous row for a continuation, the tee node's row for a branch). Propagation is recomputed on every change; a propagated cell shows the corner tick, its tooltip names the source node, and typing over it makes the cell entered and starts a new propagation from that row. Clearing an entered cell (⌫) returns it to propagation when a source exists, and to required-empty when none does. A propagated value is written to the export like an entered one (RESEARCH-E §6.1, section and material rows; see §8.3).

**The Load set column and read-through.** The Load cell names a load set (§3.4); the T and P columns show the set's values on `surface.sunken` with the corner tick; the tooltip reads "Read through from load set OP1 · typing over edits the set or forks a new one". Committing a change in a read-through cell opens a two-choice popover under the cell:

- **Edit OP1** — "changes T1 on 12 rows that use OP1": applies the edit to the set (one operation).
- **Fork a new set** — a name field prefilled with the next free name ("OP3"): creates the set with the edited value, assigns it to this row, and re-propagates the Load column from this row down to the rows that were propagating from it (one batch, one checkpoint). Rows that had entered their own set are untouched.

Each choice is a button; the highlighted one is Edit when the set is used by one row and otherwise Fork, and ↩ accelerates the highlighted choice, so that the common case is one click or one key; a click outside the popover, or ⎋, cancels the change. The read-through group is on by default in Table view and off in Both view and in the Model-view drawer, where the Load cell's tooltip lists the values; the footer chip toggles it. When the sets carry different numbers of pairs, the columns show the largest count and unused pairs read "—".

**Numeric node IDs: insert, split, renumber.**

- *New row.* The "Add row" line that ends the table, or the footer's Insert row below on the last row (↩ on the last row's last editable cell and ⌥↩ accelerate them), adds a row whose node number is the next multiple of the node increment above the highest number in the model (increment 10 by default, set in Preferences). The new row's From is implied from the row above it; its Type is Pipe; Section, Material and Load propagate.
- *Insert between.* The footer's Insert row below on a row that is not the last (⌥↩ accelerates it) inserts a row below it. Its number is the integer midpoint between the neighbours when the gap allows a whole number that is not taken (20 and 30 → 25); otherwise the next free multiple of the increment above the model's highest number. The row below keeps its number and its typed From; if its From was implied, it now implies the inserted node. Neighbours are never renumbered.
- *Split element.* "Split element…" on a row (Insert › Split element… in the menu bar, or the palette; ⌘⇧S accelerates it) opens a small sheet with Split and Cancel buttons that asks for a distance from the From node (the field is prefilled with half the element's length, shown as a proposal the engineer confirms or types over) and inserts a node inside the element: the new row takes the row's From and the first part of the offsets; the existing row's From becomes the new node and its offsets the remainder; attachments stay on their nodes. One operation, one checkpoint.
- *Renumber.* "Renumber nodes…" (Analyze › Renumber nodes… in the menu bar, or the palette) opens a sheet with Renumber and Cancel buttons that offers "From 10 by 10 in file order" with editable start and increment and a preview of old → new for every row; it rewrites every reference in one operation. Node numbers are otherwise never changed by the interface. Import keeps source numbers.
- *Editing a Node cell* renumbers that node alone, with the same reference rewrite, and is *Invalid* when the number is taken.

**Delete rows.** The footer's Delete rows (⌫ on a row selection accelerates it) deletes the selected rows' elements and nodes, and the attachments of the deleted nodes, in one batch; a row whose From was the deleted node heals to the deleted row's From with its offsets re-derived from positions so that nothing else moves. The toast lists what went: "Deleted 2 rows · nodes 30, 40 · 1 restraint · Undo".

**Marks column.** Each attachment class the node has shows as a mark in its slot (restraint, load, node data); the tooltip summarises the rows ("Restraint · +Y, gap 3 mm, μ 0.30 · ⌘↩ opens the row"); a click on the slot opens the joined row in place (Table view) or in the inspector (Model, Both), and the joined row lists every row of that class on the node (↩ or ⌘↩ on the focused slot accelerates the click; Space on it opens the mark's popover, the accelerator of "All marks on this row"). A node with two rows of one class shows the same mark with a count badge.

**Filters specific to the layout table.** Has restraint; Has load; Has node data; Branch rows; Start rows; Propagated cells (rows with any); Type (by kind).

**Empty state.** A new project's layout table holds the start row 10 and a first element row 20 with DX in the editing state (§10.1); the hint line names controls first and keys after them (§2.8): "Click a cell to type · Add row adds row 30 (↩ on the last cell) · Paste in the footer pastes rows (⌘V) · Section, Material and Load propagate from the row above once entered · the asterisk marks what the solve needs".

**State table — a layout row.**

| State | Shown | Enabled | Leaves on |
|---|---|---|---|
| Normal | values, origin glyph, marks | every cell editable | a click on the focused cell, a double-click, or typing → Editing; the expansion chevron or a marks slot (⌘↩) → Expanded; proposal lands → Proposed |
| Editing | the input with the unit after the caret; the text selected on entry; the footer's edit chip | the input; the edit chip's Commit and Cancel | Commit, a click on another cell, ↩ or ⇥ → Normal (or Invalid); Cancel or ⎋ → Normal |
| Invalid | the ring, the message popover, the typed text | the input; the edit chip's Cancel | corrected commit → Normal; Cancel or ⎋ → Normal with the old value |
| Expanded | the row plus its expansion block | the block's cells and add-row | the caption's chevron or close control (⎋) → Normal |
| Proposed | the band, bar and diamond; old and new values | Accept, Reject; cells are read-only while pending | Accept → Normal with origin Accepted; Reject → Normal unchanged; engineer edits another cell of the row → the pending row becomes Stale-proposed (§6.3) |
| Draft | dashed gutter bar, muted values (a routing or paste preview) | none (the gesture or band owns it) | commit → Normal; cancel → removed |
| Checked / Checked stale | the check or the dashed check in the state slot | as Normal; the mark's popover offers Check again and Clear check | content change → stale; Check again → Checked; Clear check (⌘⇧K on a checked row) → cleared |

### 3.3 The attachment tables

Separate tables joined on the node; the layout row carries a mark per class; a row added here marks the node row. Every table below has the common behaviour of §3.1, is sorted by node in file order by default, and can open from the node row's mark. Adding a row from the table's own "Add…" row asks for the node first (a combobox of node numbers with completion); adding from a node row's mark fills it.

**Restraints.**

| Column | Type · unit | Validation | Note |
|---|---|---|---|
| Node | node reference | must exist | |
| Tag | text, up to 14 characters | length (the export limit, RESEARCH-E §6.2) | free |
| Type | enumeration: Anchor; +Y (one-way vertical); −Y; Two-way (X, Y, Z or any combination); Guide; Limit stop; Variable spring; Constant support; User spring (rate and hot load entered); Rod hanger (a rigid Y restraint with a tag) | one kind | the kinds the engine's support families and hanger records represent; a skewed restraint along a vector and a connecting node between two pipe nodes are gaps named in the operations map |
| Direction | axes: X, Y, Z, and for rotations RX, RY, RZ; multi-select | at least one for Two-way and Guide; fixed for Anchor (all), +Y/−Y (Y) | shown as "X, Z" |
| Gap [mm] | number ≥ 0 | | one-way, guide and limit stop; "—" otherwise |
| μ | number 0–1, 2 decimals | | friction on the contact face |
| Stiffness [N/mm] | number > 0 or "rigid" | "rigid" is the entered word, not a hidden default | per direction when several are restrained: the expansion holds a value per direction |
| Library | hanger library reference | must resolve to an imported hanger table | variable spring and constant support |
| Max variation [%] | number, 0–100 | | variable spring; default absent (required for design: asterisk) |
| Design load [N] · Travel [mm] | read-only from the last run's design pass (§3.7) | — | hatched when stale |
| Note | text | | |

A +Y row's canvas glyph, gap and friction follow the row; a row whose Type changes from a linear to a one-way kind keeps its node and tag and clears the fields that no longer apply, listing them in the toast.

**Loads.** One row per load on a node or element.

| Column | Type · unit | Validation |
|---|---|---|
| Node (or Element for a uniform load) | reference | must exist |
| Kind | Force; Moment; Displacement; Rotation; Uniform load; Concentrated weight; Wind (case-level); Seismic (case-level) | one kind; wind and seismic rows are read-only summaries of the case-level inputs on the Wind and Seismic tabs |
| Direction | X, Y, Z (RX, RY, RZ for a moment or rotation) | one |
| Value | number · unit per kind (N, N·m, mm, deg, N/m) | a number; zero is a value, never ignored (the engine's zero-displacement rule, RESEARCH-E §4.2, is stated in the tooltip when the kind is Displacement: "A zero displacement is written as zero; the target file treats zero as absent") |
| Unit | display | follows the units toggle |
| Case | reference to a load case (§3.5): W, T1, P1, D1, OCC… | must name a primitive case of a matching kind |
| Note | text | |

**Node data.** One row per item on a node.

| Kind | Fields | Validation |
|---|---|---|
| Branch connection | Kind: Welding tee, Sweepolet, Weldolet, Fabricated tee, Extruded tee, Radiused branch, Branch on thickened pipe; per kind: pad thickness, crotch radius, fillet radius, thickness, length [mm]; SIF in-plane and out-of-plane user values | the node must be a From node of at least one branch row, else the warning "Assumption · Branch connection on a node with no branch" |
| SIF | in-plane, out-of-plane, axial, torsional user values | numbers ≥ 1 |
| Flange | type: WN, SO, DW, SW, FW, LJ, TH; weight [N]; gasket diameter, bolt circle diameter [mm] | |
| Concentrated weight | weight [N]; offset (x, y, z) [mm] | |
| Weld | type: butt, fillet, concave fillet, tapered transition; mismatch [mm] | |
| Threaded joint | none | |

**The joined-row expansion.** Opening a node row's mark shows, under the node row, that class's table for that node: the class's own header row, the node's rows, editable, and an add-row. Two classes can be open under one node at once (Table view); each closes by its caption's chevron or close control. The expansion's caption reads "Restraints · node 20 · joined row · ⎋ closes". In Model and Both views the inspector's attachments section shows the same rows and opens the joined row in the drawer on request.

**Empty states.** Restraints: "No restraints · click a node in the canvas with Add restraint (S), or use a node row's restraint slot"; Loads: "No loads · add a load from a node row's load slot or here; loads belong to a case"; Node data: "No node data · branch connections, SIFs, flanges, weights and welds live here".

### 3.4 Load sets

On the Loads stage's Load sets tab. A load set is a named record of temperature and pressure pairs referenced by the layout row's Load column and read through on it (direction record §10 recommendation 1).

| Column | Type · unit | Validation |
|---|---|---|
| Set | name, up to 5 characters | unique; the export limit (RESEARCH-E §3.7) |
| T1 [°C], P1 [bar] … Tn, Pn | numbers · 0 and 2 decimals | numbers; n up to 10; "Add pair" grows the columns for every set |
| Specific gravity | number | |
| Used by | count of rows, read-only | a link that filters the layout table to the rows |
| Note | text | |

Deleting a set that rows use is refused with "Used by 12 rows · reassign them first"; editing a set's value applies to every row that reads it. Forking from a layout row (§3.2) creates rows here. Empty state: "No load sets · a set names the temperatures and pressures a row reads through; type one in a row's T1 cell to create the first".

### 3.5 Load cases and combinations

On the Loads stage's Cases tab, in the shape of the Aspect Pipe Stress editor: one table for primitive cases and combinations, generated, edited or authored through the same rows.

| Column | Content | Editable |
|---|---|---|
| Case | name (W, T1, P1, D1, SE1, WIN1, SUS, OPE1, EXP1, OCC1 …) | yes; unique |
| Expression | for a primitive case, its kind and source ("Weight", "T1 · from load sets OP1, OP2"); for a combination, the terms rendered one way from the structured terms ("W + P1 + T1", "OPE1 − SUS") | display-only text; edited through the expansion |
| Stress type | Sustained, Operating, Expansion, Occasional, Hydrotest, Other | yes |
| Rule | a rule ID from the project's rule pack, or "—" | yes; a combobox of the pack's checks |
| Origin | Generated · sample-rules 1.2; Edited by R. Tufts · was T1 (generated); Authored by R. Tufts | read-only |
| Rule expression | the rule's declared formula, rendered one way, in the dashed display-only frame (M-15) | never |

**Generated.** "Generate from rule pack…" opens a generation band (same anatomy as the paste band): the pack's name, version and checksum; the load sets found; a preview of the rows the generation will create; and two buttons, Generate and Cancel (↩ and ⎋ accelerate them). Generation creates the primitive cases the load sets imply (W; T1…Tn; P1…Pn; D1… when displacements exist; SE1, WIN1 when seismic or wind inputs exist) and the combinations the pack declares for its checks (the sustained, operating, expansion-range and occasional forms), as one batch with the Generated origin and the pack identity; rows that already exist with the same name are skipped and the band says so. What the pack declares for generation is an engineering question (§11).

**Edited.** Any generated cell typed over makes the row "Edited by name · was …" and the row is never touched by a later generation. **Authored.** A row added with "Add case", the table's add-row line (⌥↩ accelerates it), is authored. **Regenerate** (in the band) replaces only rows still marked Generated and leaves edited and authored rows.

**The combination row expansion** (direction record §11 decision 14). The expansion chevron in a combination row's Expression cell (⌘↩, or Space on the cell, accelerates it) opens a block under the row: the terms as chips with a factor field each ("W + 1.0 P1 + 1.0 T1"), the available cases and combinations dimmed as chips to add, a basis control (Sum; Difference of two states; Range envelope of several), Stress type and Rule comboboxes with the pack named, and two buttons, Cancel and Done (⎋ and ↩ accelerate them). Done applies the change as one operation per changed term (one batch, one checkpoint). A term that references a case not yet solvable (missing inputs) shows the triangle and the issue.

**Expressions are display-only.** The Expression column and the Rule expression column are one-way renderings; neither accepts typed text; no parser exists (M-15, DEC-037). The M-15 caption appears once in the Loads stage's header band: "Rule expression: Display only, not accepted as input".

**Filters.** Generated, Edited, Authored, With a rule, Without a rule, Stress type.

**Empty state.** "No load cases · Generate from rule pack…, or Add case; both make the same rows".

**Wind and Seismic tabs.** Each is a small table of the case-level inputs the engine takes as user-entered values (wind: pressure, shape factor, direction, exposed spans by element; seismic: gravity acceleration or an axis factor per axis); each row is a case of that kind. Nothing is prefilled.

### 3.6 The results tables

Under the Results stage, in reviewer order: Summary · Stresses · Displacements · Restraint loads · Restraint summary · Forces and moments · Hangers. Every results table is read-only, copyable and exportable, shows the run's values only, and carries the results header above its tab strip.

**The results header.** The table name; "Run 03 · solved 15:21 · immutable"; the run menu (§5.3); the case selector (a combobox of the run's cases and combinations in load-case-table order); the Envelope switch; the evidence chip beside the run name (M-09), drawn when, and only when, the run record carries evidence and the run is Current, with no placeholder and no space held for it otherwise; and the Run identity control. Its disclosure opens under the band and holds one thing, in monospace: the run identity line (run, date and time, model state hash, solver version, rule pack name, version and hash, settings identity: "Run 03 · 2026-09-17 15:21 · sha256:4df0f798… · solver 0.2.0 · rule pack sample-rules 1.2 · sha256:9b1c4e02… · settings S-03"), provenance one click from any results table, with the full record on the run record (direction record §11 decision 9). The disclosure holds no sentence, no heading and no space for one: the acceptance sentence V1 placed above that line is removed from the product (`DEC-100`), and the control is named for what is left, "Run identity". Under the header sits at most one band, by the run's standing (§2.6): none for a Current run, the stale band for a Stale run, the historical band for a Historical run; never both. What every other part of the results surface does in each standing is §2.6's table.

**Case selector and Envelope.** The selector filters every results table to one case. With Envelope on, the selector stays visible and disabled reading "Case: all" (decision 8), and each table shows per row its governing case: for Stresses the case with the highest ratio (or the highest stress when no rule applies to the row); for Displacements, Restraint loads and Forces and moments, per component the extreme absolute value with the governing case in the cell's tooltip and "Envelope" in the Case column. The footer chip reads "Envelope: governing case per element". The envelope is computed by the interface from the run's rows; it invents nothing.

**Summary.** The one hero figure: the governing ratio with its rule ID and pack version ("0.72 · EXP-A1 · pack 1.2 · node 70 · EXP1"); then a table per stress type (Sustained, Expansion, Occasional …) of the governing element, case, stress, allowable and ratio; the maximum displacement per case with its node; the maximum restraint load per case; the run's counts (elements, cases solved, cases stopped, hangers designed); the status chips (§5.4) with their authority domains, for a Current run only (for a Stale or a Historical run the Summary shows the run's recorded statuses as plain labels with their domains and tokens under its band, never as chips); and the diagnostics count with a link to Issues. When the run has no rule pack, the hero figure is the maximum stress with "no rule pack".

**Stresses.** Node · Element (From–To) · Case · Stress [MPa] · Allowable [MPa] · Ratio (with its data bar and the 1.0 tick) · Rule · Pack. Sorted by ratio descending by default. A row whose rule inputs are incomplete shows "—" in Allowable and Ratio and the rule's completeness state in the tooltip; the row's Rule cell shows the rule and "inputs incomplete". Expansion (the Stress cell's expansion chevron; ⌘↩ accelerates it) shows the stress components the run reports for the row (axial, bending, torsional, hoop, longitudinal) with their locations. Filters: Ratio ≥ x; Node; Element; Case; Rule; Unchecked rows only.

**Displacements.** Node · Case · DX DY DZ [mm] · RX RY RZ [deg]. **Restraint loads.** Node · Tag · Type · Case · FX FY FZ [N] · MX MY MZ [N·m]; a one-way support's row also shows its state in the case (active, lifted off) from the run's diagnostics when the run reports it (C-58), never inferred. **Restraint summary.** Node · Tag · Type · then per component the maximum and minimum over cases, each with its case in the tooltip. **Forces and moments.** Element · End (node) · Case · FX FY FZ · MX MY MZ, with an "Axes: Global / Local" control in the header when the run reports both.

**Selection sync.** Selecting a results row selects its element or node in the tables and the canvas, in every standing; for a Current run the probe reads the same row (§4.3).

**Empty states.** Before any run the Results stage is disabled (§2.3). After a failed run: "No results — Run 02 stopped in OPE1 (case 3 of 6) at the iteration limit. A run's results are one immutable set, so a stopped run has none. Change the support at node 20 or the iteration limit in Run settings, then Run again." After a run with no rule pack, the Stresses table shows stresses without Allowable, Ratio, Rule and Pack, and the header line says "No rule pack · set one in Rules to see ratios".

### 3.7 The hanger table

Under Results, because the design pass needs a solve (direction record §10 recommendation 6). One row per variable-spring or constant-support location (a Restraints row of those types).

| Column | Content |
|---|---|
| Node · Tag · Type | from the Restraints row |
| Design load [N] | from the run's design pass: the vertical load at the location in the weight case with the location held |
| Travel [mm] | from the design pass: the vertical movement at the location between the weight case and the operating case named in Run settings |
| Library | the user-imported hanger table named on the Restraints row, with its provenance line above the table ("Vendor-A springs · user import 2026-09-15 · 24 sizes · source recorded in Libraries") |
| Size · Rate [N/mm] · Cold load [N] · Hot load [N] · Variation [%] | the selected record's values and the resulting loads; Variation = the absolute difference of cold load and hot load, divided by hot load |
| State | Not yet designed (no run since the row was added; every design column reads "—" and the footer counts "1 not yet designed"); Designed in Run 03; Selected (a size chosen); Stale (the model changed since the run, §2.6) |

**The two-pass ritual made explicit.** Pass one is the run: with a hanger location in the model, the run reports the design load and travel per location and the table fills. Pass two is selection, the row's expansion (the Size cell's expansion chevron; ⌘↩ accelerates it): a caption line ("Hanger selection · node 80 · Vendor-A springs · user import 2026-09-15"); under it, once, the content boundary's short variant, verbatim from the registry: "no protected standards content; code-specific data is user-supplied" (`DEC-103` item 5; M-05); then the library's candidate sizes as rows at the hanger table's widths (Size, Rate, Cold load, Hot load, Variation, working range), sorted by variation: the candidates are the sizes whose working range contains the hot load and whose variation is within the row's Max variation, and the ones outside either limit are shown disabled with their reason; each eligible row ends in a "Select" text button. Selecting writes Size, Rate, Cold load and Hot load onto the Restraints row (one operation, origin Entered with the library record as provenance), closes the block, and the canvas glyph takes the can-and-rod form with its size label. Hanger tables are a library class the user imports and the product never bundles (`DEC-103` item 5; M-05, C-51): until a library is imported the table has no sizes to offer and says so in the caption's place ("No hanger library imported · Import through Libraries"); a location whose row names no library shows "Name a library on the restraint row". A Blocks-rule-check issue is not raised for either: hanger selection is a design act, not a rule. A selected size becomes stale like any result when the model changes. Selecting a size is itself a model change, so the run is Stale after the first selection (§2.6): the remaining locations keep their design values, hatched, and their expansions still open against them, so that the ritual can be finished before the next run. Under a Historical run the expansion opens read-only.

**Empty state.** "No hanger locations · a Restraints row of type Variable spring or Constant support is a hanger location; the next run designs it".

### 3.8 Per-table summary

| Table | Selection selects | Paste with mapping | Filters beyond the common set | Empty state |
|---|---|---|---|---|
| Layout | node and arriving element | yes, full grammar | has restraint / load / node data; branch; start; propagated; type | start row 10 and row 20 editing; hint line |
| Restraints | the restraint | yes (Node, Tag, Type, Direction, Gap, Friction, Stiffness…) | type; with gap; with friction; hanger locations | hint line |
| Loads | the load | yes | kind; case | hint line |
| Node data | the item | yes | kind | hint line |
| Load sets | the set | yes (Set, T1, P1 …) | used / unused | hint line |
| Cases | the case or combination | yes (Case, Stress type, Rule; terms through the expansion) | generated / edited / authored; stress type; with rule | hint line with Generate |
| Results (each) | element or node | copy and export only | case; ratio ≥; node; element; unchecked | run-state text |
| Hangers | the restraint row | copy and export only | state; library | hint line |

---

## 4. The canvas

### 4.1 What it shows, per stage and view

The canvas is the figure the report prints (design system §6.1): geometry at real outside diameter with the edge line, fittings and components with recognisable geometry, restraint glyphs with their direction arrows, gap clearance and friction hatch, loads as scaled vectors with the kind carried by the arrow's form, node labels under the budget rule, the triad with the up axis, and the scale reference. Elements without a section draw as a centreline so a model can be routed before sections exist.

| Stage · view | Canvas content |
|---|---|
| Model · Model, Both | Geometry, glyphs, load vectors in kind colour, labels; the routing draft (§4.4); proposal ghosts (§6.3); the required-mark halo on entities with a blocking issue |
| Loads · Model, Both | Geometry and glyphs; the selected case's loads in the case's slot colour, every other load in the neutral vector colour at 40 %; selecting a load-case row selects the case; selecting a load row selects the vector |
| Results · Model, Both | For a Current run: result colour on tubes and fittings for the selected case or the envelope (§4.5); the legend; the probe; deformation on request; vectors neutral; elements for which the run reports no value neutral at 60 % with the legend's Unsolved swatch. For a Stale or a Historical run: the neutral figure, the legend's note card, and the Deformation and Probe tools disabled with their reason (§2.6) |
| Any stage · Table | no canvas |
| Review | no canvas; the "Report figure" block in the content column is a static rendering at the saved report-figure preset (§4.6) |

### 4.2 Selection and cursor sync

Both directions, always: clicking an element, node, glyph or vector selects its row and scrolls the table to it; selecting a row halos its entity and, when "Follow selection" is on in the HUD's View menu (on by default), pans the camera to keep the selected entity in view without changing zoom. Selection is a halo drawn over the element's own colour, never a replacement of it (design system §6.6; §12 item 2). Hover is a thinner halo and a light wash on the row. The focused table row's node is the "current row" and is always labelled. In Both view the inspector is opened by the toolbar's Inspector toggle and closed by the same toggle or the close control in its header; a double-click on an entity or on a row's gutter, ⌘I and ⎋ accelerate them (§5.1). ⇧-click extends the selection and ⌘-click toggles; a drag draws a box selection, with its filter (nodes, elements, restraints, loads, all) in the HUD's View menu.

### 4.3 The probe

The HUD's Probe tool toggles the probe (P accelerates it). While on, a card follows the cursor over the nearest element or node and reads what the results table shows for that row for the selected case: Node, Element, Case, Stress, Allowable, Ratio with its bar, Rule, Pack, and a footer with the run name and, when the run record carries evidence, the evidence chip. A click pins the card at 18 px from the entity, kept clear of the legend (in a canvas narrower than 400 px it pins to the canvas's bottom edge); a click on another entity re-pins; the close control at the pinned card's top right unpins (⎋, and a second click on the same node, accelerate it). The card shows what the table shows for that row and nothing more.

The probe reads results and works for a Current run only (design system §5.6; §2.6): before a run, and for a Stale or a Historical run, the tool is disabled with "Needs a current run" in its tooltip, and a pinned card closes when the run stops being Current. V1 of this document also gave the probe readings on the Model and Loads stages (the layout row, the load rows); those are withdrawn, because the design system defines the tool by the run's standing and the hover, the selection and the inspector already carry those rows.

| State | Shown | Enabled | Leaves on |
|---|---|---|---|
| Unavailable | the tool disabled with "Needs a current run" | — | a run becomes Current → Off |
| Off | — | the Probe tool | the tool (P) → Following |
| Following | the card at the cursor | a click on an entity | click → Pinned; the tool (P) → Off; the run stops being Current → Unavailable |
| Pinned | the card 18 px from the entity with its close control | the close control; a click on another entity | close control (⎋, second click on the node) → Following; click elsewhere → Pinned there; the run stops being Current → Unavailable |

### 4.4 The authoring gestures, and the rows they write

Two gestures, both row shortcuts through the one route; the canvas authors nothing else.

**Route by direct distance entry.** With a node selected, the HUD's Route tool (R accelerates it) opens the compass at that node: three axis handles X, Y, Z; the active axis solid; a length field with its unit at the handle. The pointer controls come first (§2.8): a click on an axis handle makes that axis active; to the right of the length field sit three icon buttons, Reverse, Place and Cancel, and a click on the short stub the active axis draws on its opposite side reverses it too; the length is typed, because a number is input. The keys accelerate the same controls: ⇥ or an arrow key cycles the axis, − reverses it, ↩ places, ⎋ cancels, and B toggles the bend that the inspector's routing block offers as "Bend at 40…" (the direction change inserts a bend on placing). While typing, the draft ghost (a thin dashed centreline with a faint tube outline, direction record §11 decision 12) draws from the node, the draft node's label plate shows the next number, and the layout table (drawer or side table) shows a draft row: dashed gutter bar, muted values, the typed length in the axis column in accent, and the propagated Section, Material and Load with their ticks. The inspector shows a "Routing from node 40" block on top: next node, axis, length, what the row will carry, and three compact buttons, "Place node 50" (↩), "Bend at 40…" (B) and "Cancel" (⎋), each with its accelerator in its tooltip. The Cancel control beside the length field is present when the inspector is closed too. The hint strip under the HUD lists the tool's accelerators, "Route · ↩ places 50 · ⇥ next axis · − reverses · ⎋ cancels", and every key on it has its control on the canvas or in the inspector.

On Place (↩) the gesture writes one batch: a new node at the computed position, the element from the current node to it (type Pipe, or the row's type if the engineer set it in the draft row), and, when B is on or the direction changed from the previous element, the bend at the current node with the section's bend radius propagated or required. The draft row becomes a normal row with the Entered origin; the compass moves to the new node so the next segment can be typed without a click. The batch is one checkpoint. Every value that would be required on a typed row is required here in the same way; nothing is invented.

**Add a restraint.** With the HUD's Add restraint tool (S accelerates it), hovering a node shows the +Y glyph ghosted; a click on the node opens a new Restraints row for it, as a draft row with Type focused (in Table view, the joined row under the node's layout row; in Model and Both views, the inspector's attachments section, or the drawer's Restraints tab in Model view). The draft row ends in two buttons, Add and Cancel: Add on a valid row applies one operation (↩ accelerates it) and Cancel discards the row (⎋ accelerates it). The glyph appears when the row is added.

**What the gestures may not do.** No dragging of nodes, no free-hand placement, no fillet by pointer, no direct editing of a glyph. Selecting and probing are not authoring.

### 4.5 Labels, legend, deformation and ghosts

**Labels.** The HUD's Labels tool (L accelerates it) cycles Budget, All, Off from Budget; Budget is the default (direction record §11 decision 13) with the budget of one label per 3600 square px of canvas and the priority order of design system §6.5; All is on demand for the dense figure and is not the remembered state. Restraint plates ("+Y · gap 3 mm · μ 0.30", "VS · H1") and load plates ("−2000 N · W") follow the label mode; the selected and hovered entities' plates always show, and the current row's node is always labelled. Labels never overlap.

**Legend (Results stage).** For a Current run: the quantity and unit; the scale bar with five ticks and the marker of the selected or probed value; the case name or "Envelope"; for a ratio the rule ID and pack version; the range with "set" when the engineer changed it; the Unsolved swatch; for deformation the factor, the stepper and the play control. The legend is a scale, never a verdict: nothing changes colour at 1.0; 1.0 is a labelled tick. For a Stale or a Historical run the legend is a note card of the same width with no scale, no range, no marker and no control: a title line ("Model changed since Run 03", or "Historical saved run · Run 02") and one line, "No result colour on the current model" (§2.6). In a canvas narrower than 400 px the legend collapses to its title row and expands on hover or click.

**Result colour.** Only a Current run colours the model (§2.6). On the Results stage, for a Current run, the tube and fittings take the scale colour of the selected case's value per element (ratio when a rule applies, else stress; the legend says which; "Colour by" in the HUD's View menu offers Ratio, Stress, Displacement, and Off). Edge lines remain; halos draw over colour unchanged. With Envelope on, colour follows the governing case per element. For a Stale or a Historical run the canvas draws the neutral figure, whatever "Colour by" says; a Historical record is never painted onto the current model.

**Deformation.** The HUD's Deformation tool (D accelerates it) toggles the deformed shape drawn solid with the undeformed shape as a dashed ghost, for the selected case only, never for an envelope (the tool is disabled with "Select a case" when Envelope is on). The factor is a stated scale factor: it defaults to the value that makes the maximum displacement one tenth of the model's extent, rounded to a round number, and the legend says "Deformed ×50 · default"; the stepper and field change it; play oscillates the factor from 0 to the stated value. The tool needs a Current run: before a run, and for a Stale or a Historical run, it is disabled with "Needs a current run" (§2.6).

**Ghosts.** The engineer's routing draft (accent, dashed centreline, faint outline) and the agent's proposal ghost (violet, dashed, beside the current glyph or geometry, 24 px along the run so both read) are never the same colour nor the same form and can coexist; while a proposal ghost is drawn the load vectors are neutral. A pending proposal that removes an element draws it as a dashed outline; accepting turns the ghost into the drawing; rejecting removes it.

### 4.6 The HUD, camera presets, the fitted camera and the report figure

**The HUD.** One group of ten tools at the canvas's top-left, in this order: Fit, View, Section, Isolate, Hide, Labels, Deformation, Probe, Route, Add restraint (design system §5.6). In a canvas of 400 px or more it is one row; from 399 px down to the 220 px minimum it wraps to two rows of five, Fit to Hide and Labels to Add restraint, so that Fit is the first tool at every canvas width; no tool is dropped and none moves into a menu. A tool that cannot act is disabled with its reason in its tooltip. The View tool's menu holds the presets and the canvas's switches: Iso, Top, Front, Right, Report figure, "Set report figure from this view", Follow selection, the box-selection filter, and Colour by. Each tool's letter (F, I, H, L, D, P, R, S) accelerates the tool and is shown in its tooltip after its name.

**Presets.** Fit (all, or the selection when one exists), Iso, Top, Front, Right, and "Report figure": the named camera saved with the project's interface state and used by the report's figure block and the report preview. "Set report figure from this view" (the HUD's View menu, the palette) saves the current camera; the report figure block on the Review page re-renders. The report prints the figure as the canvas draws it at the preset, in the report's theme (light by default), with the legend and scale reference.

**The fitted camera (Q-20; a default).** Direction record §11 decision 1's rule, that the canvas keeps its camera when its width changes, is for a camera the engineer placed. A camera is *fitted* from a Fit (the HUD's Fit tool, its accelerator F, a view preset, or the first display of a model) until the engineer orbits, pans or zooms. While the camera is fitted, any change of the canvas's width refits by itself: docking and undocking the inspector, the agent column opening or closing, the end of a drag of the split. Once the engineer has moved the camera it is kept: the canvas keeps its camera scale and centre and pans only as far as needed to keep the selected node in view, and pans back when the inspector closes. No toast announces either case. Fit is a visible control at every canvas width and F only accelerates it. The fitted state is one bit of interface state beside the camera and is a proposed semantic addition (§12 item 8).

| State | Meaning | On a change of the canvas's width | Leaves on |
|---|---|---|---|
| Fitted | the camera is as the last Fit, preset or first display left it | refits to the new width, silently | orbit, pan or zoom → Placed |
| Placed | the engineer moved the camera | keeps scale and centre; pans only to keep the selected node in view, and pans back when the width returns | the Fit tool (F), a preset from the View menu → Fitted |

**Section, Isolate, Hide.** Section (a cut plane along an axis at a slider position) hides what lies beyond the plane for reading congested models. Isolate (I) dims everything but the selection to 20 % opacity; it removes nothing (§12 item 1). Hide (H) removes the selection from the drawing, and while anything is hidden the count follows the last HUD tool as a text button, "3 hidden · Show all", which shows everything again; the count is of everything hidden, not of hidden items in the selection (§12 item 9).

### 4.7 What persists

Across view and stage switches: the camera (one camera for all stages) with its fitted state, label mode, colour-by choice, legend range, deformation state and factor, hidden and isolated sets, the section plane, Follow selection, the box-selection filter. Saved with the project's interface state: the camera and its fitted state, the report figure, label mode (Budget or Off; All is on demand and is not remembered), hidden sets. Never saved: the probe pin, the routing draft, the compass. None of this is model state, and none of it touches a run's standing (§2.6).

---

## 5. The inspector, issues, run and status

### 5.1 The inspector

A docked column on the canvas's right: 340 px in Model view, where it is always docked and has no close control, and 300 px (`layout.inspector.both`) in Both view, where by direction record §11 decision 1 the canvas shrinks while it is open and the table never reflows. In Both view it is opened by the toolbar's Inspector toggle, which stays latched while it is open, and closed by the same toggle or by the close control at the right end of its header line; ⌘I, a double-click on a row's gutter or on a node, and ⎋ (with focus in the inspector) accelerate them. The toggle is disabled in Table view ("Table view opens rows in place"), where the row expansion carries the same fields, and in Model view ("The inspector is always docked in Model view"). Every edit in the inspector is the same row operation as the cell would make (§2.4): nothing in the inspector is a second way to mutate the model; required fields carry the asterisk; every value carries its unit; provenance sits behind a disclosure. The narrow cases, in both orders, are §10.9.

Sections, by the kind of the primary selection:

| Selection | Sections in order |
|---|---|
| Nothing | "Select a row, a node or an element" and the counts of the model |
| A layout row (node with its arriving element) | Identity (node, element From–To, type); Geometry (DX DY DZ and X Y Z both, editable; the element length read-only); Section and material (names with the library record and a link to Libraries); Load set (name; every T and P read through; "Edit set", "Fork set"); Element fields (the type's fields of §3.2); Attachments (restraints, loads, node data as compact sub-lists with "Add…" each; a row opens the joined row); Issues on this entity (class in words, message, Show); Origin and Checked (who, when; the Check control, or Check again and Clear check when the row is checked); Provenance (the seven fields of C-46 read from the library records the row references) |
| A restraint row | Identity (node, tag); the Restraints columns as fields; a pending proposal's line in `proposal.new` when one targets the row; Issues; Origin and Checked; Provenance (hanger library) |
| A load row | node, kind, direction, value, case; Issues; Origin and Checked |
| A node-data row | node, kind, the kind's fields; Issues; Origin and Checked; Provenance (SIF source) |
| A load set | name, the pairs, used-by rows |
| A load case or combination | name, expression (display-only), stress type, rule (with the rule's display-only expression), origin; the terms as fields |
| A results row | read-only: the row's values with units, the run identity line, the evidence label, Show in table; no Checked control |
| Several rows | the count and the kinds; Checked (multi-row); the fields the rows share, editable together (one batch, one checkpoint; the field label says "5 rows") |
| During routing | the "Routing from node 40" block above the node's sections, with its three buttons, "Place node 50", "Bend at 40…" and "Cancel" (§4.4) |

**State table — the inspector (Both view; in Model view it is always Open, docked, and in Table view it is absent).**

| State | Shown | Enabled | Leaves on |
|---|---|---|---|
| Closed | the toolbar's Inspector toggle unlatched | the toggle | the toggle (⌘I, a double-click on a gutter or a node) → Open, docked, or Open, slide-over when §10.9's rule requires it |
| Open, docked | the sections for the selection; the toggle latched; the close control in the header | fields, links, Check | the toggle or the close control (⌘I; ⎋ with focus) → Closed; view → Table → Closed (remembered as open for that view); the agent column opened while the canvas would fall under 220 px → Open, slide-over (Q-15) |
| Open, slide-over | the same sections over the canvas, with the tooltip "Docked inspector needs a wider window" | the same | the agent column closes, or the window widens → Open, docked; the toggle or the close control → Closed |
| Open, routing | the routing block on top | the routing fields and the block's three buttons | Place or Cancel → Open, docked |
| Open, multi-row | the shared fields | the fields, Check | selection change → Open, docked |


### 5.2 Issues

One live list. Classes, in words, with the severity glyph (design system §2.4): Invalid model (schema or topology; blocking); Blocks solve; Blocks rule check; Provenance; Assumption; Nonlinear; Content boundary; Note. Sources: the operation layer's diagnostics when an operation is checked or applied (invalid model); the readiness check (blocks solve; blocks rule check); library records (provenance; content boundary); the run record's diagnostics (nonlinear; notes). Every issue points at its entity and is a button that selects it, switching stage if the entity's table lives on another stage.

**The drawer.** A bottom drawer of 200 px under the tables (resizable, remembered), opened from the status bar count, the rail's Issues item, the toolbar count or the Run button's reason link (⌘⇧I accelerates them), and closed by the close control at the right end of its header (⎋ accelerates it). Grouped by class in the order above with counts; each group is a filter chip; a class chip filters ("Nonlinear 1 ×", "All classes 2"); the row is the glyph, the class in words, the message and the entity, one line that never wraps: the message truncates with its full text in the tooltip, and the glyph, the class, the entity and the selected row's link never truncate. After a failed run the failure banner appears once on the Results page (direction record §11 decision 6); the drawer carries no second banner, opens filtered to the failing class, and its selected row carries the same "Show node 20" link. The drawer never blocks anything; it is not a modal gate.

**Counts.** The rail's Issues badge; the toolbar's Issues button with the worst class's glyph; the status bar's count; each table's footer count of issues on that table; the gutter's state slot per row; the cell corner or ring per cell; the canvas halo on an entity with a blocking issue.

**Filters in the drawer.** Class; Stage (Model, Loads, Results); Table; Unchecked rows only; "Since last run".

### 5.3 The run

**The button.** One Run button, centred in the toolbar, the surface's primary action in every stage.

| State | Shown | Enabled | Leaves on |
|---|---|---|---|
| Enabled (no run, or the latest run is Stale or Historical) | "Run" | click, palette "Run", Analyze › Run | click → Running |
| Disabled | "Run" on `disabled.fill`; tooltip names the reason: "Run is unavailable — Section missing at node 20, Material missing at node 20" (the blocking issues, up to three, then "and 2 more · Issues"); "No load cases"; "A run is in progress"; "Engine unavailable" | tooltip; the reason's Issues link | the reason clears → Enabled |
| Running | a progress bar inside the button with the current stage in its tooltip ("Solving OPE1 · case 3 of 6"); a Stop control | Stop; a click on the button opens the run log | completes → Completed; stops or fails → Failed; Stop → Failed with "Stopped by the engineer" |
| Completed (the run is Current) | "Run" again | a click opens the run log, whose footer offers Run settings…, Run record and Run again | Run again → Running; model change → Enabled, and the run is Stale (§2.6) |
| Failed | "Run" again; the rail caption "Failed"; the banner on the Results page | a click opens the run log, which ends at the stop | Run again → Running; model change → Enabled |

**Progress.** Stages in the tooltip and the log: assemble; each case by name in load-case order; hangers (the design pass); rules. The interface never synthesises a percentage; the bar fills by stages completed of stages planned.

**Failure.** The banner across the top of the Results page, once: the solver's diagnostic in one sentence, "Run 02 failed: nonlinear support at node 20 did not converge." with "Show node 20"; the issues drawer's Nonlinear row carries the same link; the results empty state (§3.6); the run record keeps the log and the diagnostics. The reason is always the engine's diagnostic, never inferred (C-58, C-59).

**The run log (direction record §11 decision 5; Q-16, a default).** A popover on the Run button, opened by a click on the button while it runs or after the run has ended and before the model changes; once the model has changed, the button starts a run and the ended run's log is reached from its run record. Title line: the run name, the time span and the settings identity ("Run 02 · 15:02:14–15:02:41 · settings S-02"); while the run is going the span's end is the running clock. At most four rows: assembly; the solved cases as one row ("W, SUS solved · 0.8 s"); the stopped or running case ("OPE1 stopped at iteration 50 · node 20", with "Show node 20"); and the items not run as one row; then a footer with "Run settings…", "Run record" and "Run again". A completed step carries the neutral Entered dot, never a check, because the check is the Checked mark's and belongs to a person; the step that stopped the run carries the warning triangle; a step not run, or running now, carries no glyph. The full log lives on the run record. Placement: the popover hangs from the Run button with its left edge at the button's left edge or at the canvas pane's left edge, whichever is further right, so that in Both view it lies over the canvas's top edge and the table's tab strip and header rows, where the engineer is working, stay uncovered; in Table view it hangs from the button's left edge. It covers the HUD's first tools while it is open.

| State | Shown | Enabled | Leaves on |
|---|---|---|---|
| Closed | — | the Run button | a click on the Run button while running, Completed or Failed → Open |
| Open, running | the title line with the running clock; the rows so far | Stop (on the button); "Show node…" links | the first click outside it, or ⎋ → Closed; the run ends → Open, ended |
| Open, ended | the title line with the span; the rows; the footer | "Run settings…", "Run record", "Run again", "Show node 20" | a footer control, the first click outside it, or ⎋ → Closed |

**Run settings…** A sheet: solver mode (interactive, scrutiny); iteration limit for nonlinear supports; the cases to run (all by default); the operating case for hanger travel; hanger design pass on or off; the rule pack (the project's); the settings identity ("S-03") that the run record and the results header cite. Settings are not model edits and are not on the undo stack; they are saved with the project.

**The run record.** A page opened from the run log, the results header's Run identity disclosure, a chip's popover, the historical band's popover or the palette: run name and number, time, the model hash it solved, the solver name and version, the settings identity and values, the cases and combinations, the rule pack name, version and checksum, the library references, the evidence label, the diagnostics, the full log, the immutability statement ("This record is read-only; a change to the model requires a new run"), the record's own hash, and Export (results JSON; the report package for a Current run only, §2.6). The record states the run's standing in words, Current, Stale or Historical, and a Historical run's recorded statuses are read here and in the band's popover, never as chips (§5.4).

**Immutability and naming.** Runs are numbered per project ("Run 03") in order of completion; a run may be given a name ("Run 03 · with spring at 80") that lives in the project's run index, never in the record. Every run of the project is kept in the run list (the results header's run menu) with its model hash. Choosing an earlier run shows it as a Historical run, under the historical band in the product's rendered text (§2.6), and never as the current basis; choosing the latest run again returns it with its own standing.

### 5.4 The status chip policy

Ruled (D-71 item 4, option A; `DEC-102`): the six automatic statuses and the two evidence labels take their display form from one registered table and from nowhere else (design system §2.3; `tokens.json` `labels`). The status bar's left end carries the M-08 chips.

| Raw token | Label | Authority domain, shown with the label | Kind |
|---|---|---|---|
| `MODEL_INCOMPLETE` | Model incomplete | Solver | status |
| `MECHANICS_SOLVED` | Mechanics solved | Solver | status |
| `RULE_INPUTS_INCOMPLETE` | Rule inputs incomplete | Rule pack | status |
| `USER_RULE_CHECKED` | User-rule checked | Rule pack | status |
| `USER_RULE_FAILED` | User-rule failed | Rule pack | status |
| `HUMAN_REVIEW_REQUIRED` | Human review required | Human | status |
| `INTERNALLY_VERIFIED` | Internally verified | Evidence | evidence |
| `PROVER_CORRELATED` | Prover correlated | Evidence | evidence |

Three rules bind every use of a label, on every surface, in the report, in generated text and in the agent's text. **The raw token is reachable in place:** the chip's tooltip reads the token, and a click on the chip opens its popover with the token in monospace. **The authority domain is shown with the label,** inside the chip: "Solver · Mechanics solved", "Rule pack · User-rule checked", "Human · Human review required"; for the two evidence labels the domain word is "Evidence" (the design system's word, its §8 item 14). **No label exists outside the table:** no seventh status, no third evidence label (the reserved third is never emitted, C-19), no abbreviation and no curated synonym; the two rule-pack labels take the hyphenated forms everywhere.

The policy (direction record §11 decisions 2–4; `DEC-102`; R-3; Q-21). The status bar speaks for the current model and its Current run, and for nothing else.

1. Before any run, and whenever the model cannot be solved, one chip: Solver · Model incomplete, with the blockers in the popover.
2. When the model is complete but no run has solved it, no chip (ruled): an absent status is consistent with the vocabulary, and no seventh label exists.
3. After a solved run, while it is Current, two chips, one per authority domain: Solver · Mechanics solved and, when a rule pack is set, the rule-pack status (Rule inputs incomplete, User-rule checked or User-rule failed); with no rule pack, the Solver chip alone.
4. Human · Human review required is shown on the Review page, beside the rule-pack chip, and in the report; on no other page (ruled).
5. After a model change the chips drop: the run is Stale, the rail caption "Stale" and the results header band carry the fact, and the status bar says nothing about the run until the next run.
6. A Historical run lights no chip in the status bar: its recorded statuses are read in the historical band's popover, as labels with their domains and tokens, and on the run record. A chip in the bar would read as the current model's standing, which a Historical record never supplies (§2.6).
7. After a failed run the chips are whatever statuses the run record carries (Q-21, a default until the engine answers §11 question 11):

| What the stopped or failed run's record carries | Status bar's left end | What carries the state |
|---|---|---|
| one or more of the six statuses | those chips, each with its domain | the chips; the rail caption "Failed"; the banner |
| no status | empty: no chip, and Model incomplete is not carried over from before the run | the rail caption "Failed", the banner and the run log |

A chip states a status the run record carries; the interface asserts none of its own. Chips are read, never clicked to change anything; the click opens the popover: the raw token in monospace, the authority domain, the run and time, and the reason when the status has one (the blockers of the solve; the rule inputs that are missing).

**The evidence chip (M-09).** It follows the same anatomy and the same three rules beside the run name in the results header and in the probe's footer. It appears only when the run record carries evidence, and only for a Current run: a run whose record has no evidence entry shows no chip and no placeholder, and the header closes up.


### 5.5 The toolbar, status bar, palette and menus

**Toolbar band.** Left to right: the SWBPIPE wordmark; the project name with its save state ("Loop 4 header · saved", "· edited", "· not saved"); Undo and Redo as two icon buttons (the one undo stack of the product; disabled with "Nothing to undo"; the tooltip names the operation, "Undo: accept P-12 row 1 (⌘Z)"); the view switch; centred, Run; the Issues count as a button; then the two toggles of the right-hand panels in the order the panels sit on the screen, the Inspector toggle (⌘I; §5.1) and the Agent toggle (⌘⇧G; §6.1), each latched while its panel is open, both dropping their labels and keeping their tooltips below 1360 px of window width; the display units selector; the palette field (⌘K, "Search or command…"). One toolbar per surface: the tables' controls live in their footers and column menus; the canvas's in its HUD.

**Status bar.** The chips (none, one or two by §5.4); the issues count (opens the drawer); the selection ("Node 40 · 1 row · 2 proposed rows"); the display units; and the About control, an information glyph with the tooltip "About SWBPIPE…" that opens About directly. V1 hung a popover from this control whose lines were the About link and the maturity sentence (M-01); the owner removed that sentence from the product entirely (the second addendum; `DEC-105`), and with the acceptance sentence gone too (`DEC-100`) nothing is left for a popover to hold, so the control opens About. Nothing else lives in the status bar: no hashes, and nothing about a Stale run, which the rail caption and the results header carry.

**Command palette (⌘K).** Searches commands, nodes, elements, restraints, cases, runs, sections, materials, rule IDs and issues in one list; a command runs; an entity selects and navigates; the list shows the accelerator and, for a disabled command, its reason. Commands are the union of the menu items, the HUD, the tables' footer and header controls and the agent panel's actions. The palette and the menu bar list every command, so a command outside the working loop always has a visible home (§2.8).

**Menu bar.** File (New project, Open…, Open recent, Save, Save as…, Import library…, Export model batch file (.mbf)…, Report preview, Close); Edit (Undo, Redo, Cut, Copy, Paste, Paste with mapping…, Select all, Find, Check rows, Clear check, Accept row, Reject row, Preferences…); View (Table, Model, Both; Model, Loads, Results, Review; Inspector, Issues, Agent; Labels, Deformation, Probe; Fit, Iso, Top, Front, Right, Report figure; Isolate, Hide, Show all; Light, Dark, System); Insert (Row, Row below, Split element…, Restraint, Load, Node data, Load case, Combination, Snapshot…); Analyze (Run, Stop, Run settings…, Run record, Generate load cases…, Renumber nodes…); Window and Help (About SWBPIPE, Scope and limitations). "Analyze" is the product's existing spelling and Canadian English (R-5). "Export model batch file (.mbf)…" opens the export and handoff sheet with that target chosen (§8.3). The menu bar, the toolbar and the palette are the three command surfaces; there is no fourth. About shows SWBPIPE, version and build, the name and nothing appended to it, followed directly by its tabs (M-17).

---

## 6. The agent collaboration protocol

### 6.1 The panel

A right column of 340 px, present in every stage, collapsible to a 44 px strip by the toolbar's Agent toggle or the collapse control at the right end of the column's header; the whole strip is a button that opens the column (⌘⇧G accelerates all three). It reflows the surfaces and never overlays the tables or the canvas. The strip shows the agent glyph, the count of open proposals as a badge, and a dot while the agent is working; the column's header shows "Agent", the working state ("Idle", "Working…" with a stop control, "Idle · P-12 open") and the tab row: Conversation, Proposals (count), Checks, Accepted (count). The column has no control that acts on the model except Accept and Reject; everything the agent proposes is read in the tables and the canvas. When the docked inspector needs the width, the column collapses to its strip by itself, with the toast "Agent column collapsed to its strip: the docked inspector needs the width." and its action "Reopen"; when the column is opened while the inspector is docked, the inspector becomes the slide-over (§10.9; Q-15).

| State | Shown | Enabled | Leaves on |
|---|---|---|---|
| Strip | glyph, badge, working dot | the strip; the Agent toggle | a click on the strip or the toggle (⌘⇧G) → Column |
| Column · Idle | the active tab | tabs, input and Send, Accept/Reject | the toggle or the header's collapse control (⌘⇧G) → Strip; the docked inspector needs the width (§10.9) → Strip, with the toast |
| Column · Working | "Working…" with Stop; the conversation shows "Preparing proposal…" | Stop; the tables stay editable | proposal lands → Idle with a card; Stop → Idle |
| Unavailable | "No agent configured · Preferences" | Preferences link | configuration → Strip |

### 6.2 Conversation

Messages with author and time; the engineer's input at the bottom as a growing field with the Send control at its right end (disabled while the field is empty; ⌘↩ accelerates it). The agent's messages may reference rows, results, report text and runs as links that select the referent (and navigate). Unknowns are listed under the message as TBD items with the triangle, and conflicts or gaps as a list with the octagon of their class; the agent never fills a value it does not have (C-78). A message may carry an evidence summary: a short read of what the record shows about a referent, by reference, never a verdict; as a card it carries the class word "Evidence summary" (§6.6). The status and evidence labels in the agent's text are the registered forms of §5.4, hyphenated forms included, and the words design system §7.1 forbids in agent text never appear in it: titles, rationales, constraints, TBD items, checks, summaries and drafts alike; the runtime guard applies to comment and draft text as it does to rationale. The engineer may ask for anything; the agent can answer, check, and propose; it cannot run, export, snapshot, check rows, accept its own proposals, or touch libraries, rules or preferences.

### 6.3 The proposal lifecycle

A proposal is one card in the Proposals tab and a set of banded rows in the tables (and ghosts in the canvas). Its unit of decision is the row.

**States of a proposal.**

| State | Meaning | Shown |
|---|---|---|
| Draft | the agent is composing; nothing has landed | "Preparing proposal…" in the conversation; no rows, no ghosts |
| Proposed | landed; the engine found no schema, constraint or unit error; every row pending | the card; banded rows with old and new values; ghosts; the strip badge; the tab counts ("Restraints 7 · 2 proposed") |
| Blocked | the engine rejected one or more rows | the card's rejection line lists the engine's diagnostics; the blocked rows' Accept is disabled with the reason; the other rows can be decided |
| Partly decided | some rows accepted or rejected | decided rows collapse to their receipt line (below); "Accept remaining (n rows)" and "Reject remaining" appear |
| Closed | every row decided | the card moves to Accepted (with its outcome: accepted, partly accepted, rejected); bands and ghosts gone |
| Withdrawn | the agent withdrew it before every row was decided | "Withdrawn by the agent · reason"; undecided rows unband; decided rows keep their outcome |

**States of a proposal row.**

| State | Meaning | Shown |
|---|---|---|
| Pending | awaiting the engineer | the diff table on the card; the band in the table; Accept, Reject |
| Accepted | applied through the one route | the receipt line ending "accepted 16:31 · Undo"; the row's origin becomes Accepted with the diamond; the ghost becomes the drawing; the run becomes Stale (§2.6) |
| Rejected | not applied | the receipt line ending "rejected 16:33"; the band and ghost removed; nothing changed |
| Stale | the engineer changed the target row after the proposal landed | "The row changed since this proposal · the agent can revise"; Accept disabled; Reject enabled; the band stays with a hatch |
| Blocked | the engine rejected the row | the diagnostics; Accept disabled |

**The decided row's receipt (Q-18; a default).** A row that has been decided collapses to one line: the done check; the first changed field with its new value ("Type Variable spring") and, when the row changes more fields, their count ("+4"); and at the right "accepted 16:31 · Undo" or "rejected 16:31". The line never wraps; the new value is the first thing to truncate. The old value is not on the line: it is in the line's tooltip ("Type: Rigid (Y) → Variable spring") and in the diff, which a click on the line's chevron opens again. The row's place in the proposal ("row 1 of 2") stays on the row's title line above the receipt.

| Card row state | Shown | Enabled | Leaves on |
|---|---|---|---|
| Pending | the diff table (field, old, new); the Accept and Reject icon buttons at the row's right end | Accept, Reject | Accept → Receipt, accepted; Reject → Receipt, rejected; the target row is edited → Stale; the engine rejects → Blocked |
| Receipt, accepted | the one-line receipt with "accepted 16:31 · Undo" | the chevron; Undo while the checkpoint is reversible | chevron → Receipt with its diff open; Undo → Pending |
| Receipt, rejected | the one-line receipt with "rejected 16:31" | the chevron | chevron → Receipt with its diff open |
| Receipt with its diff open | the receipt line and the diff under it, read-only | the chevron; Undo as above | chevron → Receipt |

**Per row, multi-row and batch.** Accept row (the check button at the right end of the card's row, tooltip "Accept row ⌘⇧A"; the footer's Accept row chip on a focused proposed row; ⌘⇧A accelerates them) applies that row's operation set as one batch and one checkpoint. Multi-row: select several proposed rows in a table and use the footer's Accept row chip (⌘⇧A), or ⇧-click several rows on the card, then "Accept selected (3 rows)": one batch, one checkpoint. Whole batch: "Accept all rows" on the card, or "Accept all (n rows)" above the queue for every open proposal, each confirming with the count: one batch per proposal, one checkpoint each. Reject mirrors each level (the cross button, tooltip "Reject row ⌘⇧R") and confirms with the count when more than one row. A row is an indivisible unit: its field changes are accepted or rejected together.

**What an acceptance records.** On the accepted operation: the proposal identity, the row, the rationale, the constraints considered and the TBD items (M-14); the engineer's name and the time; the author type stays the agent's, and the acceptance is a review decision, never an engineering acceptance (C-06). The Accepted tab lists one line per accepted unit with time, proposal, rows, the rationale disclosure and Undo while the checkpoint is still reversible; records reopened from an earlier session show "Acceptance not recorded in this session" (C-68).

**Consequence line on the card.** "Accepting a row changes the model. Results from Run 03 stay readable, marked stale, until the next run." It is the design system's sentence (§5.4) and says what the engineer will see: on acceptance Run 03 stops being the current solve basis and becomes Stale (§2.6); its values stay readable under the stale band, and it drives no overlay and no chip until the next run. No sentence on the card says the run remains the basis of anything.

### 6.4 How a proposal lands in the tables and the canvas

At landing: the affected rows take the band, the bar and the diamond in the origin slot; changed cells show the new value in `proposal.new` with the struck old value before it when the column has room, otherwise in the tooltip and on the card; a new row is banded whole with "new" in its origin tooltip; a removed row is banded with every value struck; the tab strip counts proposed rows; the canvas draws the ghost beside the current geometry or glyph and gives the node's plate the ghost edge; the inspector shows the proposed line under the current values; the footer reads "7 restraints · 2 proposed rows · P-12 · draft until accepted" with the Accept row and Reject row chips. Nothing changes stage or view; the card's "Show rows" link navigates. The selection band paints over the proposal band and the bar and diamond remain.

### 6.5 What a proposal may and may not do

May: any row operation on the model tables (layout, restraints, loads, node data, load sets, load cases, combinations), including new rows, deleted rows, renumbering, and multi-table changes in one proposal; propose report text as a Draft comment on the Review page (§7.4); withdraw itself. May not: run, stop or configure a run; export; snapshot; set or clear Checked; accept or reject anything; import or change libraries or rule packs; change preferences, the camera or any interface state; write a value it has not been given (TBD instead); use the words the copy rules forbid (§9.3). A proposal that would need something it may not do says so as a TBD item ("Vendor table to use: TBD").

### 6.6 The agent's checks and open issues by reference

The Checks tab and the Review page's comment stream hold the agent's feedback on the engineer's work. Every card the agent authors carries exactly one of five class words as its label, and no other word serves as one (D-71 item 8; `DEC-103`; design system §5.4; `tokens.json` `agentCardClasses`): **Proposal** (a proposed edit as a diff; the Proposals tab; Accept and Reject), **Check** (a check of the engineer's work against a referent; Resolve, Reopen), **Open issue** (something the agent could not settle, by reference; Resolve, Reopen), **Evidence summary** (the agent's summary of what the record shows about a referent: a run, result rows, a library record; Resolve, Reopen) and **Draft** (text the agent drafts for a report section; Insert, Discard). V1 called the agent-authored card of the fourth kind a "Note"; an agent card never reads Note now, and "Note" remains the word for the engineer's own comment. Each card carries its class chip, the referent as a link ("Restraints row, node 30"; "Stresses · OPE1 · node 40"; "Report §4, paragraph 2"), the text, the author and time, and a state the engineer sets (Open, Resolved; Reopen). Checks never alter a table, never set a mark, and never use the forbidden words (C-77). A referenced row shows a small comment glyph in its state slot on the Review page only.

### 6.7 The Checked mark

Ruled (D-71 item 7, option A, by the first addendum; `DEC-104`). The open item is closed.

**Classification.** The Checked mark is a PRD §16.3 tag: a plain human tag, human-authored, set on one row or on many rows at once, recording who and when and a hash of the row's content; shown stale when that content changes; set and cleared by the engineer only, never by the agent and never by the product. It is not the PRD §21.3 human acceptance record: the reserved third evidence label stays reserved, and a formal acceptance of a whole analysis on the Review page remains a possible later owner act that this design neither creates nor forecloses.

**Storage boundary.** The mark is carried in the project's interface state and not in the model payload, so checking a row never changes the model hash. It is never emitted in any status, never written to an export or a run record, and never rendered in a report: not in a fixed section, not in the review/signoff block, and not as a count the product inserts. It is not one of the labels of §5.4, it lights no chip, and the footer's count of Checked rows is a filter; no surface turns it into a status or a cue that anything is ready.

**Words, exactly these.** The control is "Check"; the mark is "Checked". Its tooltip: "Checked by *name* · *date time* · bound to this row's content · not a software status". When stale, after the same name and date: "the row changed since · Check again or Clear". The filter is "Unchecked rows". Those five are the whole of `DEC-104`'s list. The control names beyond it are not in that list and are derived from the addendum, not ruled by it: "Check rows" is the control "Check" on a selection, from the addendum's "set on one row or many"; "Check again" and "Clear check" are buttons for the two acts the ruled stale text names, "Check again or Clear". They are this design's and the design system's names, and are the owner's to change.

- **Controls.** The faint check in an empty state slot, drawn on hover ("Check"); the footer's selection group, Check rows; the Checked glyph's popover, with Check again and Clear check as buttons; the inspector's Check control; Edit › Check rows and Clear check. ⌘⇧K accelerates them.
- **Binding.** The mark binds to a content hash of the row's own cells (its table's columns and its expansion's fields). A row whose content changes, by any route, shows the mark stale: the dashed check with the stale tooltip. If the content returns to the checked state (undo, or a reverting edit), the mark is current again.
- **Who may set and clear.** The engineer only. The agent cannot set, clear or refresh it; software only reports staleness. "Check again" re-binds to the current content; "Clear check" removes it.
- **Multi-row.** Check rows on a selection marks every selected row; when the selection mixes checked and unchecked rows the control reads "Check 3 unchecked rows"; on a fully checked selection it reads "Clear check" and clears them.
- **Filters and counts.** "Unchecked rows" is the filter; it includes rows whose mark is stale. The footer counts "12 checked · 1 stale", and each count filters when clicked. The counts live in the tables' footers only; V1's checked counts on the Review page's outline are withdrawn, because beside a report they would read as a cue that something is ready.


### 6.8 Origins per row and cell, and the changes-since control

Every row and cell carries its origin: Entered by name at time; Accepted from proposal P-n by name at time; Propagated from node n; Generated by rule pack name and version at time; Imported from file at time. A cell's origin is the origin of the last operation that set it; a row's origin is the operation that created it; the gutter shows the row origin (quiet, and nothing at all for Entered unless Origins is on); a propagated cell shows its corner tick; a click on the origin glyph opens its popover, whose last line, "All marks on this row", lists everything (Space on a slot and ⌥I accelerate them). The Origins footer switch shows a dot for entered rows so that the column can be scanned.

**Changes since…** A footer control offering: last run, last Checked (this row's), a named snapshot (a Review iteration), a time. It applies a filter chip "Changed since Run 03 · 4 rows" and gives every changed cell the corner tick with "Changed since Run 03: 3 mm → 0 mm · by R. Tufts 16:31" in its tooltip. It reads the operation ledger; it never guesses.

---

## 7. The Review page

### 7.1 The three columns

A full page in Table view: the outline (280 px), the content (flexible), the comment stream (320 px). The header: "Review · Loop 4 header · Run 04"; the iteration control, a combobox of named snapshots ("Iteration 2 · 09:40 · Run 04") whose menu lists the iterations and, after a separator, "Compare with…" (Q-17, a default); and, at the right, three buttons of one weight, each with its icon: "Snapshot…", "Report preview" and "Export…" (direction record §11 decision 10). Export… is a plain button like Report preview (Q-19, a default): a window has one accent, and it is Run; producing the report is not this page's primary act, writing it is. Under the header's hairline the three columns begin at once: the acceptance sentence that V1 placed there is removed from the product (`DEC-100`), and the space it held is closed. The status bar on this page shows the rule-pack chip and Human · Human review required, for a Current run (§5.4).

**Comparison (Q-17).** Choosing "Compare with…" starts a comparison, and only then does the compared-with control appear beside the iteration control, itself a combobox ("compared with Iteration 1 · 2026-09-17 15:40 · Run 03") whose menu changes the compared iteration or clears the comparison ("Clear comparison"), with the "Show edits" switch beside it. With nothing compared neither is there, so the header carries no fourth button.

| State | Header shows | Enabled | Leaves on |
|---|---|---|---|
| Nothing compared | the iteration control; the three buttons | "Compare with…" in the iteration menu | "Compare with…" and an iteration chosen → Comparing |
| Comparing | the iteration control; the compared-with combobox; the Show edits switch; the three buttons | the combobox's menu (another iteration; Clear comparison); Show edits | another iteration → Comparing (against it); Clear comparison → Nothing compared |

### 7.2 The report outline

Sections in the required order (M-07; PRD §19.2 as reflected in RESEARCH-C §4), each row with a state word and either a lock glyph (fixed: the content comes from the record) or a drag handle (editable text sections may be reordered among themselves only):

| # | Section | Fixed or editable | Content source |
|---|---|---|---|
| 1 | Notice | fixed | the required report notice (M-03) and, when the report includes design-authoring records, handoff packages, export metadata or external-prover references, the supplement (M-04) |
| 2 | Identity | fixed | software version, solver version, build identifier, date and time, units, coordinate system, model state name and hash, run name and number |
| 3 | Libraries and rule packs | fixed | library references; rule-pack name, version, checksum, source note; provenance summary |
| 4 | Model | editable text plus live blocks | the engineer's description; live blocks from the model record (Layout, Restraints, Node data, Load sets) inserted on request; the report figure block |
| 5 | Load cases | editable text plus a live block | the Cases table, live |
| 6 | Assumptions and warnings | fixed | the run's diagnostics by class and the unresolved assumptions |
| 7… | Results · one entry per results table selected | live | Summary, Stresses, Displacements, Restraint loads, Restraint summary, Forces and moments |
| … | Hanger selection | live | the hanger table with its library provenance |
| … | Review/signoff block | form | the block the engineer fills (name, role, date, notes); the product neither fills it nor labels anything as accepted |

Comparison is absent in this design and the outline shows no entry for it. State words: empty, drafted, edited (since the compared iteration), current (unchanged since it), live; under the outline the note "fixed sections come from the record". Clicking an outline row scrolls the content; the drag handle moves an editable section among the editable ones (⌘↑ ⌘↓ accelerate it).

### 7.3 Section content

**Live table blocks.** The table component in read mode with a caption "Live from Run 04 · Stresses · Envelope" or "Live from the model record · Restraints · nodes 20, 60, 80 · not editable here" and the lock glyph; the block's case selector and sort are its own and are saved with the report. A results block follows its run's standing (§2.6): from a Current run it reads plain; from a Stale run it carries the stale band and its cells the hatch; from a Historical run it carries the historical band in the product's rendered text; in neither of the last two does the block, the outline or the header light any cue that a report is ready. Nothing in a live block is editable here, and clicking a row selects it in the spine (and navigates on request). "Insert live table…" in the text toolbar offers any model or results table with a row filter (by node range, by selection).

**Text blocks.** The engineer's writing with a simple toolbar: paragraph, list, reference (a row, a result, a run, a rule ID, a section), insert live table, insert the report figure. References render as links. Numbers typed in text carry no automatic unit; the toolbar's "Insert value" places a referenced cell's value with its unit as a live token that updates with the run and shows the stale hatch when the run is Stale.

**Edits shown or hidden.** With Show edits on, text inserted since the compared iteration is underlined in `accent.text` and removed text is struck in `proposal.old`, with the legend "inserted · removed since Iteration 1" once, on the first block that has edits; with it off the current text reads clean. Live blocks are not diffed (run comparison is excluded); a live block whose run differs from the compared iteration's says "Iteration 1 used Run 03" in its caption.

**The report figure block.** The canvas at the Report figure preset, rendered for the report's theme, with the legend and scale reference; "Set from the canvas" re-captures.

### 7.4 Iterations as named snapshots

"Snapshot…" asks for a name (prefilled "Iteration 3") and freezes: every text block, the live blocks' captions and settings with the run identity they showed, the model hash, the comment states, and the report figure. Iterations are immutable and listed in the combobox newest first with time and run; "Current" is the working state. "Compare with…", in the iteration menu (§7.1), chooses the iteration Show edits diffs against (the previous iteration is offered first). Reopening an iteration shows it read-only under a band of its own ("Iteration 1 · read-only · Run 03") with "Restore as current", which copies its text into the working state as one edit. An iteration's band is about the report text; the run it cites is Historical (§2.6), and reopening an iteration never makes that run Current.

### 7.5 The comment stream

One stream, ordered by referent then time; a header "Comments 4" with "+ Comment". Kinds: Check, Open issue, Evidence summary and Draft, the agent's four comment kinds by their class words (§6.6); and Note, the engineer's own comment. An agent card never reads Note. Each card: the kind chip, the referent link, the author and time, the text, and one action pair: Resolve or Reopen (Note, Check, Open issue, Evidence summary) or Insert and Discard (Draft). "+ Comment" adds a Note attached to the current selection or the current paragraph. A comment references a row in any table, a result row, a run, a report section or paragraph; comments never alter the tables; a referenced live row shows the comment glyph on this page only. Insert on a Draft places the agent's text into the section as an edit by the engineer (shown as inserted text), and the Draft becomes Resolved; the engineer may edit the inserted text freely.

**The filter row (Q-22; a default).** One row that holds in the 320 px column: four chips with counts for state and ownership, All, Open, Resolved, Mine, and one Kind menu for the agent's kinds, Checks, Open issues, Drafts, Evidence summaries, each with its count in the menu. When a kind is chosen the menu's button reads the kind and its count ("Kind: Checks 1"); a kind with no cards takes no space anywhere. State and kind are two filters and combine. The engineer's own notes are reached with Mine.

| State chip | Kind menu | The stream shows | The Kind button reads |
|---|---|---|---|
| All | none chosen | every card | "Kind" |
| Open, Resolved or Mine | none chosen | the cards in that state, or the engineer's own | "Kind" |
| All | a kind chosen | every card of that kind | "Kind: Checks 1" |
| Open, Resolved or Mine | a kind chosen | the cards that are both | the kind and its count within the state |
| any | "All kinds" chosen in the menu | the kind filter clears | "Kind" |

### 7.6 Report preview and Export

"Report preview" opens the preview dialog: the outline rendered as it prints, with the notice and the required content block as fixed sections, the review/signoff block, the theme choice, a Close button, and Print or Save (PDF, HTML). Print, Save and the report package need a Current run (§2.6): for a Stale or a Historical run the preview still opens, with the run's band printed on every live block, and Print and Save are disabled with "Needs a current run". "Export…" opens the export and handoff sheet (§8.3, §8.4). The report is produced from this page and nowhere else.

### 7.7 What is and is not editable

Editable: text blocks; the order of editable sections; live blocks' case selector, sort and row filter; the review/signoff block's fields; comments and their states; iteration names. Not editable: the fixed sections' content, live values, the notices, the run identity, an iteration once taken. The agent edits nothing on this page; it comments and drafts.

---

## 8. Libraries, rules, export and handoff

### 8.1 Libraries

The Libraries page (rail foot) lists the project's imported libraries by kind: materials, sections, components, hanger tables. Hanger tables are a library class like the others, imported by the user and never bundled (`DEC-103` item 5). Columns: Name, Kind, Records, Source, Redistribution status, Review status, Imported (time), Used by (count). Actions: Import…, Open (the record table with every field and its provenance), Delete (refused when referenced: "Used by 14 rows"), Quarantine. No library of any kind ships with the product; a new project's Libraries page is empty (C-51, M-05).

**Import…** A sheet: the file; the kind; the seven provenance fields by their record identifiers (`source_name`, `source_location`, `source_license`, `contributor`, `contributor_certification`, `redistribution_status` with its enumeration, `review_status` with its enumeration); the findings list of the six import flag classes (required fields missing; units missing or inconsistent; provenance missing; redistribution status unclear; values appear to be protected standards data; values outside user-defined reasonableness ranges) with the triangle; the quarantine action for suspected protected content, which stops the import, sets the record's `review_status` to `quarantined` and its `redistribution_status` to `protected_suspected`, records an issue and asks for review; and two buttons, Import and Cancel. The M-05 sentence, verbatim from the registry, once at the top; the registry act of `DEC-101` gives that sentence the name SWBPIPE. An imported library's records become available in the Section, Material and Library comboboxes; the layout row's provenance disclosure reads from them.

### 8.2 Rules

The Rules page lists rule packs: Name, Version, Checksum, Public or private, Source note, Status, and "Project rule pack" (one is set for the project; the results header, the load-case generator and the hanger design cite it). Actions: Import…, New draft, Open, Set as project rule pack, Compute checksum, Delete. The editor (Open) has three parts: declarations (required inputs with their source kinds), value slots (allowables and limits with their units and provenance: every value user-supplied, never prefilled), and checks (each with its ID, the quantity, the relation and the limit). Expressions are authored with the structured composer, never as text; the rendered expression appears beside each check in the dashed display-only frame with the caption "Display only, not accepted as input · notation not frozen" (M-15). A pack whose checksum does not match its content shows the mismatch as a warning and cannot be set as the project rule pack until recomputed. Public example packs are labelled "Invented values · not engineering data" (C-50).

### 8.3 Export to the model batch file

Ruled (D-71 item 9, by the owner's amendment; `DEC-103`): the export is named by what it writes and never names another vendor's product. The command, in the File menu and the palette, is "Export model batch file (.mbf)…"; the Review header's "Export…" opens the same sheet; the sheet's title, with that target chosen, is "Model batch file (.mbf)". There is no line attributing the grammar to anyone and no compatibility flag, in the sheet, its tooltips, the loss report's headings, its examples or anywhere else in the product or the user guide; this document's own explanatory prose says "the model batch file grammar". The sheet carries the M-06 sentence once, which names no vendor and whose registered placement on export surfaces is unchanged. Fields: target; units system for the file (SI or English, the file's one switch); what is included (layout, sections, materials, load sets, restraints, loads, node data, analysis options); the loss report as a list; the export metadata summary; Write file… and Cancel.

**The export is a projection.** Each table projects to the file's records with the correspondence RESEARCH-E §6 tabulates; the interface writes, drops or transforms as below and records every drop and transformation in the export's loss report, which the sheet previews as a list under the heading "Loss report" and the file's manifest carries. No heading, entry or example in the loss report names another vendor's product: an entry says what was dropped or transformed and why, in the words of the table below.

| Gap (RESEARCH-E §6) | Behaviour on export |
|---|---|
| 1 Alphanumeric IDs | Nothing to do: node IDs are numeric throughout. Written as they are. |
| 2 Element type to code | Written: Pipe as a blank; Bend as the tangent-intersection code with its radius; Valve, Reducer, Rigid, Expansion joint as their joint codes with their fields. Types the file has no code for are not offered in the product. The correspondence is the interface's table and is listed in the loss report as "assembled from documented lists". |
| 3 Tee | Written: the branch-connection kind on the node-data row becomes the node code on the tee node's line, with its kind-specific fields. Never an element. |
| 4 Temperatures and pressures | Written through the load-set route: each load set becomes a named load record; each row references its set by name. Row-level temperature comments are not written. |
| 5 The load reference key | Written as the reference key the published examples use, and the loss report names it "undocumented in the specification; used in published examples" so the reader knows. |
| 6 Origins | Dropped. Recorded in the loss report as "provenance not carried by the format". |
| 7 Checked | Dropped. Recorded likewise. |
| 8 Per-value units | Transformed: every value converted to the file's one unit system, deterministically, with the unit disclosure written beside the file; the loss report lists the conversions performed. |
| 9 Concentrated mass | Transformed: written as weight; the loss report says so and names the acceleration used. |
| 10 Snubber | Not offered in the product (a recorded non-goal). Nothing written. |
| 11 Rod hanger | Written as the rigid Y restraint it is modelled as, with its tag; the loss report notes the kind. |
| 12 Time-varying load | Not offered. Nothing written. |
| 13 Load cases | Dropped: the file has no section for them; the loss report lists every case with "not carried by the file; selected in the application that reads it". |
| 14 Combinations | Dropped likewise. |
| 15 Per-item units | Transformed to the one file-level switch, as gap 8. |
| 16 Export metadata | Written beside the file, never inside it: the manifest, the stable id map (every row to its line), the loss report, the validation report, the unit disclosure, the source model hash, the run identity when a run exists. The report's supplement (M-04) is named as what the report will carry when export metadata or a handoff package is included. |

Further projection rules: Section, Material and Load are written on every row where the value is entered and on the first row where a propagated value applies, then only on change (RESEARCH-E §7 item 4 leaves whether repeating is accepted unstated; the loss report records the choice as "written on change"); a second and later attachment on one node goes on a location line; tags are truncated to 14 characters with a loss entry; a start row writes absolute coordinates; a branch row writes the re-anchor line then its offsets; row order is file order. The sheet says what it writes and nothing more.

### 8.4 Handoff

The second target of the same sheet: the handoff package (model, run record, results export, report, export metadata, provenance summary), with the M-06 sentence once and the private-data controls (excluded by default; inclusion is the engineer's explicit act, C-88). The package is written with its manifest and hashes; the run record cited is the Current run or the chosen Historical run, and the package names the run's standing; a Stale or a Historical run is never described in it as current results (§2.6).

---

## 9. Disclosures and vocabulary

### 9.1 The mandatory disclosures

Every M-item of RESEARCH-C §4, with its surface, its trigger and its wording source. Wording is never paraphrased: it is the registered text or the source's text, verbatim; this document does not restate the sentences. Two disclosures are retired by the owner's rulings and have no home on any surface, in any form or short variant; their rows stay, marked retired, so that the seventeen-row table stays traceable and nothing is renumbered. What carries the distinction between what the software computed and what a person decides is now the status chips with their authority domains (M-08) and the report's required notice (M-03).

| ID | Surface | Trigger | Wording source |
|---|---|---|---|
| M-01 | none: retired by the owner's ruling, `DEC-105` | never | The maturity sentence is removed from every live surface: no banner, footer, status bar, information popover or About line, and the space each held is closed (§5.5). Record: the second addendum to the D-71 ruling; `DEC-105`, superseding `DEC-099`. The registry entry it came from is `projects/chirality-piping/docs/claims_registry.md:67`, retired by the same act; the sentence still exists in the product source at `projects/chirality-piping/apps/desktop/src/App.tsx:3282` until the implementation tranche removes it together with the lint anchor and the registry entry. |
| M-02 | none: retired by the owner's ruling, `DEC-100` | never | The acceptance sentence and every listed short variant are removed from the product: not the results header or its disclosure, not the Review page, not the report's body, the inspector, the probe, the canvas or the agent panel, and the space each held is closed (§3.6, §7.1). Record: the D-71 ruling, item 2; `DEC-100`. The registry entry is `projects/chirality-piping/docs/claims_registry.md:31`, whose placement clause is retired for product surfaces by the same act. The control word "Accept" for a proposed edit, the Checked mark and the report's required notice are untouched. |
| M-03 | The report and the report preview; the Notice section of the outline | a report is produced or previewed | the required report notice (PRD §19.3; `docs/report_notice_template.md` §"Required Notice", first block), verbatim; the product name inside it becomes SWBPIPE by the PRD-level act of `DEC-101` (scope-change bundle SCA-010); source lines `projects/chirality-piping/docs/report_notice_template.md:35`; `projects/chirality-piping/docs/PRD.md:1244`; emitted by the renderer `projects/chirality-piping/core/reporting/report_renderer/src/lib.rs:716` |
| M-04 | The report and its preview; named in the export dialog | design-authoring records, handoff packages, export metadata or external-prover references are included (comparison outputs are excluded from this design) | `docs/report_notice_template.md` §"Required Notice", second block, as RESEARCH-C §4 records it; source lines `projects/chirality-piping/docs/report_notice_template.md:35`; recorded at `instances/RESEARCH/C_ui_constraints.md:285` |
| M-05 | The Libraries import sheet; the export and redaction surfaces; the hanger-selection expansion, which carries the content boundary's short variant once (§3.7; `DEC-103` item 5) | the sheet opens; a hanger selection expansion opens | claims registry BS-IP, canonical or a listed short variant; source lines `projects/chirality-piping/docs/claims_registry.md:17` |
| M-06 | The export and handoff sheet | the sheet opens | claims registry BS-VALID, canonical or a listed short variant; source lines `projects/chirality-piping/docs/claims_registry.md:52` |
| M-07 | The report outline on the Review page and the preview | always on the Review page | PRD §19.2 (the required content list, §7.2); source lines `projects/chirality-piping/docs/PRD.md:1213`; the renderer's section order `projects/chirality-piping/core/reporting/report_renderer/src/lib.rs:457` |
| M-08 | The status bar chips, none, one or two by §5.4, each drawn with its authority domain and with its raw token reachable in place; Human review required on the Review page and in the report; the Summary table | a solve or rule-check state exists for the current model and its Current run | `docs/TYPES.md` §4 for the tokens; the labels are the registered display forms of `DEC-102` (design system §2.3); source lines `projects/chirality-piping/docs/TYPES.md:52`; `projects/chirality-piping/schemas/analysis_status.schema.yaml:107` |
| M-09 | The results header chip beside the run name; the probe footer; the report's case pages | a Current run is shown and its run record carries evidence; never a placeholder | claims registry §2 evidence labels; only the two emitted labels, in the registered display forms of `DEC-102`; source lines `projects/chirality-piping/docs/claims_registry.md:87` |
| M-10 | The issues drawer; the gutter's state slot; cell marks; the rail's Issues count; the canvas halo | a finding exists | the class names of SPEC §8 and PRD §14.4, spelled in words per design system §2.4; source lines `projects/chirality-piping/docs/SPEC.md:684`; `projects/chirality-piping/docs/PRD.md:872`; the class enumeration `projects/chirality-piping/schemas/model.schema.yaml:376` |
| M-11 | Every column header, inspector row, probe row, legend, report table and export preview | always | the unit catalogue's symbols; source lines `get_unit_catalog` `projects/chirality-piping/apps/desktop/src-tauri/src/lib.rs:1381` |
| M-12 | The inspector's provenance disclosure; the Libraries record view; the import sheet | a governed value is shown | the seven fields and enumerations of `docs/IP_AND_DATA_BOUNDARY.md` §4; source lines `projects/chirality-piping/docs/IP_AND_DATA_BOUNDARY.md:58`; `projects/chirality-piping/schemas/hanger.schema.yaml:64` |
| M-13 | The results header band with the product's rendered text; its popover with the second sentence and the run's recorded statuses; the rail's Historical caption; the legend's note card | a saved run is reopened, or an earlier run is chosen from the run list | the text the product renders (D-71 item 6; `DEC-103`): "Historical saved run", "Run a fresh solve to establish current results." and "Historical results cannot drive current overlays, rule checks, comparisons or report readiness."; source lines `projects/chirality-piping/apps/desktop/src/features/results/HistoricalRunContext.tsx:321`, `:322` and `:323` |
| M-14 | The proposal card ("draft until accepted"), the diff, Accept and Reject, rationale, constraints considered, TBD items, the Accepted record | a proposal exists | PRD §11.8 FR-AGENT-001 to 005 as reflected in RESEARCH-C §4; source lines `projects/chirality-piping/docs/PRD.md:533` (FR-AGENT-001 at `:537`) |
| M-15 | The rule-expression cell's dashed frame and caption on the Loads page and in the Rules editor | an expression is rendered | DEC-037: "Display only, not accepted as input", with the notation declared not frozen; source lines recorded at `instances/RESEARCH/C_ui_constraints.md:296`; the rendering `projects/chirality-piping/apps/desktop/src/features/rule-packs/ExpressionComposer.tsx:403` |
| M-16 | Preferences › Appearance › Contrast findings | the panel opens | the specimen's line: "Findings only. No conformance is claimed; the target is set by the project authority."; source lines `instances/DESIGN-SYSTEM/specimen.html:1568`; recorded at `instances/RESEARCH/C_ui_constraints.md:297` |
| M-17 | About › Scope and limitations | the tab opens | `docs/DIRECTIVE.md` §6: scope, validation status, known limitations, data-boundary constraints, professional-responsibility limitations; source lines `projects/chirality-piping/docs/DIRECTIVE.md:105` (§6) and `:117` (the list) |

Negative rules: the fence token never appears on a product surface; the retired phrase family that design system §7.1 lists under Anywhere never appears; no surface stacks three or more boundary terms outside a registered sentence; the retired status tokens named in `docs/TYPES.md` §4 are never emitted; the maturity sentence and the acceptance sentence appear nowhere, in any form (M-01 and M-02 above); the product is never given a longer or earlier name, or a maturity word appended to its name; and no product copy names another vendor's product (§8.3).

### 9.2 The status vocabulary

The six automatic statuses with their authority domains (§5.4), the two evidence labels, and the historical designation are the whole status vocabulary of the interface. Rows, runs, proposals, iterations and libraries have *states* (pending, accepted, rejected, stale, open, resolved, designed, imported, quarantined), which are named in words and never rendered as chips of the status family. "Checked" is a tag (§6.7; `DEC-104`). A run's standing (Current, Stale, Historical) is a state named in words, on bands and captions, never a chip. "Accepted" is the review decision on a proposal row and the review status of a library record; it is never an engineering acceptance and never a status chip.

### 9.3 The copy rules that bind every string

Design system V1.2 §7 binds every string in this document and in the product; restated here with the additions this document makes. Product copy uses Canadian English, as the product's existing menu "Analyze" and the design system's "colour" both do ("Analyze", "colour", "centre", "centreline", "modelled", "labelled"; R-5); the registered sentences keep their own spelling verbatim.

| Where | Allowed | Never |
|---|---|---|
| Proposal controls | Accept, Reject, Accept row, Reject row, Accept selected (n rows), Accept remaining (n rows), Reject remaining, Accept all rows, Reject proposal, Accept all (n rows), Reject all | the forbidden verbs of design system §7.1; Apply; Confirm; Sign off |
| Proposal standing | Proposal, draft until accepted, rationale, constraints considered, TBD, withdrawn, stale | verified, validated, recommended by |
| The Checked mark | The ruled words (`DEC-104`): Check, Checked, Checked by *name*, bound to this row's content, not a software status, the row changed since, Check again or Clear, Unchecked rows. Derived control names, not in `DEC-104`'s list (§6.7): Check rows, from the addendum's "set on one row or many"; Check again and Clear check, from its stale text "Check again or Clear" | verified, reviewed, signed; any word that makes the mark a status |
| Statuses | the six labels of the one registered table (§5.4), each with its authority domain and its raw token reachable in place | any label not in the table; the product's earlier short forms; an unhyphenated rule-pack label; Ready, OK, Pass, Fail |
| Results | number, rule ID, pack version; Ratio, Allowable, Governing case, Envelope, Unsolved, Stale | Pass, Fail, OK, Safe, Acceptable, Exceeds |
| Evidence | the two evidence labels of the same table, Internally verified and Prover correlated, drawn only when the run record carries evidence and only for a Current run | the third label; Validated; an evidence chip on a run that carries none |
| Agent output | the five class words as card labels and no others: Check, Open issue, Draft, Proposal, Evidence summary; TBD; withdrawn | the words design system §7.1 forbids anywhere in agent text; Note as the label of an agent card |
| Runs | immutable, named, numbered, the historical band's rendered text "Historical saved run · Run a fresh solve to establish current results." and its popover's "Historical results cannot drive current overlays, rule checks, comparisons or report readiness.", No result colour on the current model, Needs a current run, Model changed since Run *03*, The values below are from the model as solved, Run again, Run settings…, Run record, solved, stopped in *OPE1*, Run *02* failed: *reason*, not yet designed, No run yet, No solved run, Failed, Stale and Historical as rail captions | Final, Released; any sentence that says a run remains the basis after the model has changed |
| Export | Export model batch file (.mbf)…; Model batch file (.mbf); the model batch file; projection; Loss report; Write file… | the name of another vendor's product, in a command, a dialog, a loss report heading, a tooltip or an example; a line attributing the grammar to anyone; a compatibility flag; compatible with |
| Anywhere | SWBPIPE as the product's only name | any longer or earlier product name; a maturity word appended to the name; the maturity sentence and the acceptance sentence, in any form; the words design system §7.1 lists under Anywhere, including the retired phrase family; the fence token; "Press" as an instruction |

Ratios are always number, rule ID and pack version together, in three columns in a table and one line elsewhere ("0.72 · EXP-A1 · pack 1.2"). Toasts state the outcome of an act that no permanent surface records and offer its one action ("Pasted 3 rows · 2 columns ignored" with Undo; "Agent column collapsed to its strip: the docked inspector needs the width." with Reopen); a state a permanent surface already carries raises no toast: a failed run has the banner, the rail caption and the run log, a model change has the stale band and the caption. Empty states name the way forward and the control to click. Tooltips on icon controls carry the same words as the menu item, the control's name first and its accelerator after it in parentheses (§2.8). The product is SWBPIPE, and only SWBPIPE (D-71 item 3, amended; `DEC-101`): the wordmark, the window title, About, the report's identity block, the file badge, every example and every tooltip read SWBPIPE; nothing is appended to the name, and the name the product had before appears nowhere.

---

## 10. Empty, error and edge states

### 10.1 New project

File › New project asks for a name and creates a project with an empty model, no libraries, no rule pack, no runs. The Model stage opens in Both view (first-open default): the layout table holds the start row 10 (all "—") and row 20 with DX in the editing state and the required asterisks on Section, Material and Load; the hint line under the rows; the marks column present and empty; the gutter empty; the canvas shows the ground grid, the triad and node 10 as a point. Run is disabled with "Run is unavailable — Section missing at node 20, Material missing at node 20"; the missing load set is a Blocks-rule-check issue; Issues reads 3; Results and Review are disabled with "No run yet"; the one chip reads Solver · Model incomplete; the project name shows "· not saved". Libraries is empty with "Import a library to name sections and materials". The Section and Material comboboxes on row 20 offer "Import library…" as their only item until a library exists.

### 10.2 A model with no sections

Elements route and draw as centrelines; every element row's Section cell carries the required asterisk and the octagon; the issues list holds one Blocks-solve issue per element ("Section missing · node 20"); Run is disabled naming the first three; the canvas legend is absent; a paste that maps no Section column lands rows in this state. Entering a section on the first element's row propagates down the chain and clears the issues of every row that propagates.

### 10.3 A failed run

§5.3 Failed: the banner once on the Results page with the engine's sentence and "Show node 20"; the drawer's Nonlinear row; the rail caption "Failed"; the results empty state naming the stopped case, the cause and the two remedies (§3.6); the run log ending at the stop with the cases not run listed; Review disabled with "No solved run" unless an earlier solved run exists, in which case the results header's run menu offers that run, shown as a Historical run under the historical band, and a Historical run enables nothing (§2.6); the chips as the failed run's record carries them, and none when it carries none (§5.4 item 7; Q-21). The failed run is kept, numbered, in the run list.

### 10.4 A reopened historical run

Opening a project with a saved run, or choosing an earlier run in the run list, shows a Historical run (§2.6): the results tables show the run under the historical band in the product's rendered text, "Historical saved run · Run a fresh solve to establish current results." (M-13); the band's information control opens the popover with "Historical results cannot drive current overlays, rule checks, comparisons or report readiness." and, under it, the run's recorded statuses as labels with their domains and raw tokens; the rail's Results caption reads "Historical"; the status bar shows no chip from the run; the header shows no evidence chip; the legend is the note card "Historical saved run · Run 02" with "No result colour on the current model"; the canvas draws the neutral figure, and the Deformation and Probe tools are disabled with "Needs a current run"; the result cells read plain under the band and are readable and copyable; the Review page's live blocks from the run carry the band, the run enables nothing there, and Print, Save and the report package are disabled with "Needs a current run". Run is enabled if nothing blocks it, and a new run becomes Current and replaces the view. No presentation change, reopening included, ever gives the record a current-model overlay or a readiness cue, or makes it Current. Export of results from a Historical run is allowed, and the export names it Historical.


### 10.5 A proposal whose row was edited before acceptance

The row becomes Stale-proposed (§6.3): the band gains the hatch; the card's row reads "The row changed since this proposal · the agent can revise"; Accept is disabled with that reason; Reject stays enabled; the conversation shows the agent a system line "Row Restraints · node 20 changed after P-12 landed"; the agent may withdraw the row or issue a revised proposal that supersedes the stale row (the old row is then shown "superseded by P-13"). The other rows of the proposal are unaffected.

### 10.6 A paste with unmapped columns

The band shows the unmapped source columns as "Sched → Ignore", "Note → Ignore" with their source names kept; the count line says "2 columns ignored"; Paste is enabled; the preview shows what will land; the toast after Paste repeats "Pasted 3 rows · 2 columns ignored · Undo". A source column that could map to a target the table has (for example "Mat") is offered the nearest target in the combobox but not pre-mapped; nothing lands from an ignored column.

### 10.7 A stale Checked mark

The dashed amber check with its tooltip ("Checked by R. Tufts · 2026-09-18 14:02 · the row changed since · Check again or Clear"); the footer count "1 stale", which filters when clicked; the row is included by the Unchecked rows filter; the inspector's Origin and Checked section reads "Checked by R. Tufts · 2026-09-18 14:02 · the row changed since · Check again or Clear" with the two buttons Check again and Clear check under it. Undo of the change that made it stale makes it current again without any act.

### 10.8 A library missing

Opening a project that references a library file that cannot be found: every row that references a record from it shows the reference in `text.primary` with the provenance triangle and "Library not found · Libraries"; the issues list holds one Provenance issue per library ("Library Vendor-A springs not found at its recorded path"); Run is not blocked by the missing provenance but is blocked by any value the solve needs that the library provided (a section's dimensions), which the readiness check reports as Blocks solve; the Libraries page shows the library greyed with "Locate…" and "Remove reference" (refused while rows use it). Hanger rows whose library is missing read "Library not found" in the Library column.

### 10.9 Window at the minimum size, and the docked inspector's narrow cases

Reconciled to design system V1.2 §0 (R-1): V1's floors of 320 px for the Both-view canvas and 480 px for the table pane are withdrawn. The one floor is the canvas's minimum drawing width of 220 px (`layout.canvas.min`); the table has no floor of its own, because it keeps its width in every case and the split is the engineer's.

At 1280 × 800 the surfaces are 1180 × 728 with the strip. Table view: the tables scroll horizontally; the read-through group stays on. Model view: the canvas keeps its width beside the 340 px inspector, and the table drawer keeps its remembered height (280 px by default) or collapses to its 28 px tab strip by its chevron. Both view: the split holds 55 % / 45 % (649 / 531 at 1280 with the strip; 737 / 603 at 1440) unless the engineer moved it, and a drag of the split stops where the canvas would fall under 220 px. Below 1360 px of window width the toolbar's Inspector and Agent toggles drop their labels and keep their tooltips. Below 1280 × 800 the window does not resize.

**The narrow-case order.** Opening the docked inspector takes 300 px (`layout.inspector.both`) from the canvas pane and nothing from the table. While the remainder is at or above 220 px the canvas keeps its camera, or refits when the camera is fitted (§4.6). When opening the inspector would take the canvas below 220 px: first the agent column collapses to its strip, with the toast "Agent column collapsed to its strip: the docked inspector needs the width." and its action "Reopen"; if the canvas would still fall below 220 px, the inspector opens as a slide-over over the canvas for that window size only, with the tooltip "Docked inspector needs a wider window". The canvas is never collapsed. At and above the minimum window the slide-over case does not arise unless the engineer's own split leaves the canvas under 520 px.

**The other order (Q-15; a default).** With the inspector docked, opening the agent column would take the canvas under its minimum. The column opens, because the engineer's last explicit act wins, and the inspector becomes the slide-over over the canvas for as long as the column is open at that width; it re-docks when the column closes. Nothing is refused and nothing closes, and the selection's context is never lost.

| Window and agent | Surface width | Table / canvas | Canvas with the inspector docked |
|---|---|---|---|
| 1440, strip | 1340 | 737 / 603 | 303 |
| 1440, column open, then the inspector opened | 1044, then 1340 | 574 / 470, then 737 / 603 | 170 would be below the minimum: the column collapses to its strip first; canvas 303 |
| 1280, strip | 1180 | 649 / 531 | 231 |
| 1280, column open, then the inspector opened | 884, then 1180 | 486 / 398, then 649 / 531 | 98 would be below the minimum: the column collapses first; canvas 231 |
| 1440, inspector docked, then the agent column opened | 1044 | 574 / 470 | 170 docked would be below the minimum: the inspector becomes the slide-over over the 470 px canvas while the column is open, and re-docks (canvas 303) when it closes |
| 1280, inspector docked, then the agent column opened | 884 | 486 / 398 | 98 docked would be below the minimum: the same rule, the slide-over over the 398 px canvas |

| State | Agent | Inspector | Leaves on |
|---|---|---|---|
| A | strip | closed | Inspector toggle → B; Agent toggle or strip → C |
| B | strip | docked | Inspector toggle or close control → A; Agent toggle or strip → D when the canvas would fall under 220 px, otherwise docked with the column open |
| C | column | closed | Inspector toggle → B when the canvas would fall under 220 px (the column collapses, with the toast), otherwise docked with the column open; Agent toggle or collapse control → A |
| D | column | slide-over over the canvas | the column closes → B (the inspector re-docks); Inspector toggle or close control → C |
| E | strip | slide-over (the window or the engineer's split is too narrow even with the strip) | the window or the split widens → B; Inspector toggle or close control → A |

**The narrow canvas.** In a canvas narrower than 400 px the HUD wraps to two rows of five with Fit first (§4.6), the legend collapses to its title row and expands on hover or click, and the probe pins to the canvas's bottom edge (§4.3). No tool is dropped at any width.

---


## 11. Open questions for engineering

Each is a behaviour the design needs and a question about the engine or the typed interfaces; none chooses a technology. The operations map's gap list is the estimate basis; these are the questions behind it.

1. **Node numbers as entity identity.** The design needs numeric node IDs that the engineer can renumber (one node, or all by increment) with every reference rewritten in one operation. Question: can the operation layer take a node renumber as one change kind that rewrites references atomically, or must the interface compose it from create, reconnect and delete kinds with a stable internal identity behind the visible number?
2. **Offsets that move the downstream chain.** Committing DX on a row moves the To node and every downstream node by the same vector as one checkpoint. Question: is a batch of node coordinate changes the intended route, or should the layer offer a translate-chain change kind so the batch is not composed by the interface?
3. **Named load sets.** The design needs a load set (T1…Tn, P1…Pn, specific gravity) as a record referenced from the layout row, read through and forkable. Question: does the load-case model represent temperatures and pressures per element in a way a named set can bind to, and can editing a set update every element that reads it as one operation?
4. **Generation of load cases from the rule pack.** The design needs generation of primitive cases and combinations from the load sets and the pack. Question: what does a rule pack declare that generation can read (the case kinds its checks require), and does the layer emit the cases and combinations as a previewable batch?
5. **The hanger design pass.** The design needs the run to report, per hanger location, the design load and the travel between the weight case and a named operating case, with the location held in the first and free in the second. Question: can the solve take hanger locations as design points and return those two values per location in the run record?
6. **Selection against a hanger library.** The design needs candidate sizes from a user-imported table filtered by hot load range and variation limit. Question: is that filter the interface's to compute over the library records, and does the support record carry rate, cold load and hot load as the selection writes them?
7. **Per-row and per-cell origins that persist.** The design needs the origin of every row and cell (entered, accepted, propagated, generated, imported) to survive save and reopen. Question: is the applied-operation ledger persisted with the project with author type per operation, and can propagation and generation be attributed as operation classes?
8. **The Checked mark.** Its classification is closed (`DEC-104`, §6.7); the engineering question stands. The design needs a human tag bound to a row's content hash, with name and time, persisted with the project's interface state and never in the model payload, a status, an export, a run record or a report. Question: is there a project-level place for row tags that is not the model payload, so that the model hash does not change when a row is checked?
9. **Changes since.** The design needs "changed since the last run, the last Checked, a snapshot, a time" at row and cell grain. Question: does the ledger record the model hash and time per applied operation so the interface can answer without diffing payloads?
10. **Runs as a list.** The design needs every run of a project kept, numbered, named, immutable, with its model hash, and the ability to show an earlier run as historical. Question: can the project store hold many run records rather than one, and does the record carry the settings identity and the hanger design pass?
11. **Statuses of a stopped run.** The design shows whatever statuses the run record carries, and no chip when it carries none (Q-21, a default until this is answered). Question: what statuses and diagnostics does a run that stops at the iteration limit emit, and does it carry the case it stopped in?
12. **Evidence labels.** The design shows the evidence label per run in the results header, only when the run record carries evidence and only for a Current run. Question: which component emits the two labels and where does the run record carry them?
13. **Envelope and governing case.** The design computes the governing case per row in the interface from the run's rows. Question: does the run report every case's stresses per element with the rule's allowable per row so the envelope is a selection and not a computation?
14. **Stress ratio rows.** The design needs, per element and case, stress, allowable, ratio, rule ID and pack version. Question: does the rule-check result bind each check outcome to an element and case so a row can be formed without inference?
15. **Result colour, probe and presets in the canvas.** The design needs per-element result values for colour, a probe that reads the row under the cursor, a Right preset, a saved report-figure camera, a section plane, and an edge-line colour chosen per element from its fill (design system §6.7, `canvas.edgeAlt`). Question: which of these are the rendering brief's overlays and which need data the run record does not carry today?
16. **Snapshots of the Review page.** The design needs named, immutable iterations of report text bound to a run and a model hash. Question: is a model state record the right carrier for an iteration, and can it hold report text and comment states beside the model hash?
17. **Comments by reference.** The design needs comments attached to rows, results, runs and report paragraphs, with open and resolved states, persisted with the project. Question: is there a project-level record for notes with typed references, and can it reference a report paragraph?
18. **Agent conversation and proposal records.** The design needs a conversation, proposals with per-row rationale, constraints considered and TBD items, per-row decisions, withdrawal, and an accepted record that survives reopen. Question: what proposal record does the agent route accept today, does it carry constraints considered and per-row status, and what is the mechanism by which a live agent binding would arrive given the standing hold?
19. **Paste with mapping.** The design needs a header-mapped multi-row paste that lands as one batch with source node IDs kept. Question: can the layer take a batch of create-node, connect, assign and set-field operations of that size as one checked unit with one undo checkpoint?
20. **Export projection choices.** The design writes section, material and load on change only, truncates tags, and writes a location line per extra attachment. Question: does the export foundation take these as options and record them in the loss report, and where does the load-reference key of gap 5 sit in its grammar?
21. **Element kinds not represented.** The design offers only the kinds the engine represents and names the rest as recorded scope decisions. Question: which of slip joint, ball joint, hinge, tie rod, elastic element and beam are gaps and which are non-goals, so About › Scope and limitations can say so?
22. **Skewed restraints and connecting nodes.** The design's Restraints table needs a restraint along a direction vector and a restraint between two pipe nodes. Question: can the support record carry a direction vector and a second node reference?
23. **Concentrated weight as node data.** The design records a weight at a node as node data that participates in every case that needs mass. Question: is a node mass representable, or only a force in the weight case?
24. **Zero displacement.** The design writes a zero imposed displacement as a value. Question: does the engine treat a zero imposed displacement as a specified value, and what should the export write given the target file ignores zero?
25. **A Stale run kept on a Historical record's terms.** The design keeps the run record after a model change, readable under the stale band, while the run stops being the solve basis exactly as it does today (§2.6, §12 item 4). Question: can the project hold the run record and its results after a model commit without restoring the solve proof or the model hash, so that the stale-response guards and the Current and Historical designation are untouched?
26. **Interface state beside the model.** The design needs one carrier, saved with the project and outside the model payload, for per-stage view memory, the camera with its fitted bit, the report-figure preset, label mode, hidden sets, table views and the Checked marks. Question: is that one record or several, and can it be versioned without touching the model hash?
27. **Dimming and halos.** The design dims the complement of an isolated selection to 20 % and draws selection as a halo over unchanged colour. Question: can the instanced renderer carry a per-instance opacity and an outline pass, and are dimmed elements pickable or excluded from picking?

---

## 12. Semantic changes named as changes

Every behaviour below differs in meaning from what the product does today. None of them is a restyling, and none may be delivered as one (the implementation-handoff preparation §2 constraint 7). Each row cites both sides: what the product does at the current `HEAD`, by file and line, read for this revision and not taken from any brief; and what this design specifies, by section here and in design system V1.2. Each names the operations map's gap entry it depends on. Items 1, 2, 5 and 9 to 14 follow RESEARCH-G §2, whose citations were re-read at `HEAD`.

| # | Behaviour | Today (file:line at `HEAD`) | This design | Gap |
|---|---|---|---|---|
| 1 | Isolate | Isolate hides. `isolateSelectionVisibility` returns a hidden-key mask for the complement (`projects/chirality-piping/apps/desktop/src/features/viewport/viewportSelection.ts:246`), `composedVisibilityHiddenKeys` joins it to the Hide mask (`:219`), and `applyVisibilityPresentation` zeroes the instance matrices or clears `visible` for every key in it (`projects/chirality-piping/apps/desktop/src/features/viewport/viewportResource.ts:322`); picking skips hidden keys (`projects/chirality-piping/apps/desktop/src/features/viewport/viewportSelection.ts:578`), so an isolated model's complement can be neither seen nor picked. | Isolate dims everything but the selection to 20 % opacity and removes nothing; Hide removes (§4.6; design system §6.6). Whether a dimmed element can be picked is a decision the implementation must make and state (§11 question 27). | G-30 |
| 2 | Selection | Selection replaces the element's colour: `applyInstancedSelection` sets a selected instance's colour to the selection colour and every other instance's to the base colour (`projects/chirality-piping/apps/desktop/src/features/viewport/viewportResource.ts:975`). Over result colour that would destroy the value it is drawn on. | Selection is a 2 px halo around the silhouette, and hover a thinner one, drawn over the element's own colour, unchanged (§4.2, §4.5; design system §6.6, §6.7). | G-31 |
| 3 | Camera and interface state | Not persisted. The camera record is written on every camera change (`projects/chirality-piping/apps/desktop/src/features/viewport/PipeViewport.tsx:375`, `:885`) and never read; the camera is refitted whenever the project session generation changes (`:1415`); the project envelope carries no interface state (`projects/chirality-piping/apps/desktop/src/types.ts:964`). | Per-stage view memory, the camera with its fitted state, the report-figure preset, label mode, hidden sets, table views, drawer heights and the agent column persist across stage and view switches and are saved with the project's interface state, never with the model (§2.3, §4.7). | G-17 |
| 4 | A run's results after a model change | Two things happen, and this design changes only the second. First, the run stops being the solve basis: a model commit goes through `commitModelAfterSolveInvalidation` (`projects/chirality-piping/apps/desktop/src/App.tsx:276`, called at `:853`), which invalidates the solve-run gate and clears the solve proof (`:857`) and the model hash (`:858`). Second, the results are removed from the product: every applied operation, batch, undo and redo calls `clearComputedModelState` (`projects/chirality-piping/apps/desktop/src/App.tsx:1695`; callers at `:1333`, `:1472`, `:1515`, `:1666`, `:1689`), which sets the result, the analysis run, the input manifest and the rule-check aggregate to null, clears the historical-run context, the proposal and the review target, and invalidates the report package (`:1696` to `:1704`); the product says so: "Previous solve results were cleared" (`:1336`). No stale state exists. | The first is kept exactly: after a model change the run is not the solve basis, and nothing restores the proof or the hash. The second changes: the run record and its values are kept, readable, hatched and copyable under the stale band, as a Stale run held on a Historical record's terms; it drives no canvas overlay, no status chip, no evidence chip and no readiness cue until the next run (§2.6; design system §5.1 run standing, §8 items 16 and 17). | G-11 |
| 5 | Label mode | A boolean toggle (`projects/chirality-piping/apps/desktop/src/features/viewport/PipeViewport.tsx:426`, button at `:2176`) and a fixed cap of 80 labels (`projects/chirality-piping/apps/desktop/src/features/viewport/viewportSelection.ts:610`), which the performance instrument's boundary validation and fixture manifest also bind (RESEARCH-G §2 item 8). | Three states, Budget, All, Off, with Budget the default and remembered state: one label per 3600 square px of canvas, by the priority order (§4.5; design system §6.5). Replacing the cap changes an evidence binding, not only the control. | G-16 |
| 6 | Proposal decisions | One proposal slot (`projects/chirality-piping/apps/desktop/src/types.ts:968`), presented whole with no per-row decision (`projects/chirality-piping/apps/desktop/src/features/agent-proposals/AgentProposalPanel.tsx:4`); any model change clears it (`setProposal(null)`, `projects/chirality-piping/apps/desktop/src/App.tsx:1703`); acceptance receipts live in the session (`projects/chirality-piping/apps/desktop/src/types.ts:786`). | The unit of decision is the row: per-row, multi-row and batch Accept and Reject, each one batch and one checkpoint; rows go Stale or Blocked singly; a pending proposal survives a model change to other rows; decided rows keep a receipt; the accepted record persists (§6.3, §6.4). | G-18 |
| 7 | A pending proposal and a model change | Any applied operation clears the proposal (`projects/chirality-piping/apps/desktop/src/App.tsx:1703`). | Only the rows whose target changed become Stale-proposed; the other rows stay pending (§6.3, §10.5). | G-18 |
| 8 | The fitted camera (Q-20) | No such state: the camera is fitted when the session generation changes (`projects/chirality-piping/apps/desktop/src/features/viewport/PipeViewport.tsx:1415`) or on an explicit fit (`fitViewportCamera`, `:3022`), and a change of the canvas's width neither refits nor is tracked. | One bit of interface state beside the camera, fitted from a Fit until the engineer orbits, pans or zooms; while fitted, any change of the canvas's width refits silently; once placed, the camera is kept (§4.6; design system §0, §8 item 12). | G-32 |
| 9 | The hidden count | The viewport's status shows the number of selected items that are hidden (`projects/chirality-piping/apps/desktop/src/features/viewport/PipeViewport.tsx:489`), which is zero whenever the selection is cleared after hiding. | The HUD's count is of everything hidden, a text button, "3 hidden · Show all" (§4.6; design system §6.6). A total hidden count is a new published quantity. | G-30 |
| 10 | Deformation scale | A fixed normalized display offset of 0.65 local units (`projects/chirality-piping/apps/desktop/src/features/viewport/PipeViewport.tsx:4469`), published with the boundary string `scale=normalized_display_offset_not_physical_length` (`:4495`). | A stated, adjustable scale factor with an extent-derived default, shown in the legend ("Deformed ×50 · default"), with play (§4.5; design system §6.8). A real factor changes what the overlay means, and the published boundary wording changes with it. | G-16 |
| 11 | The undeformed shape | Deformed and authored geometry are both drawn solid, in separate layers (RESEARCH-G §2 item 5). | While deformation is shown the undeformed shape is a dashed ghost and the deformed shape is solid (§4.5). What the model layer draws in that mode changes, with consequences for picking. | G-16 |
| 12 | The routing draft's colour | The route ghost is drawn in the selection colour: `0xf08c22` (`projects/chirality-piping/apps/desktop/src/features/viewport/PipeViewport.tsx:3996`) is `SELECTED_COLOR_DARK` (`projects/chirality-piping/apps/desktop/src/features/viewport/viewportResource.ts:219`). | The draft ghost has its own token and form, never the selection's and never the proposal ghost's (§4.5; design system §6.9). | G-31 |
| 13 | Labels as part of the figure | Label plates are HTML elements in an overlay layer (`projects/chirality-piping/apps/desktop/src/features/viewport/PipeViewport.tsx:2279`), outside the 3D pick, the theme repaint and anything a report would print from the canvas (RESEARCH-G §2 item 9). | The report prints the figure as the canvas draws it, plates included, in the report's theme (§4.6; design system §6.10). | G-16 |
| 14 | Real outside diameter; canvas colours as tokens | The canvas defaults to schematic fixed-radius tubes and resets to schematic on every new project session (`projects/chirality-piping/apps/desktop/src/features/viewport/PipeViewport.tsx:441`, `:1395`); model colours are literals, and the theme switch repaints only the background, the gizmo and the selection colour (RESEARCH-G §2 item 10; the implementation-handoff preparation §4). | Pipes draw at real outside diameter as the resting state, and every canvas colour is a token with light and dark values, repainted live (§4.1; design system §6.2, §6.11). | G-16 |
| 15 | The Checked mark | None exists. | A human row tag outside the model payload, never in a status, an export, a run record or a report (§6.7; `DEC-104`). New state, new storage. | G-08 |
| 16 | Runs as a list | One `analysis_run` slot in the project envelope (`projects/chirality-piping/apps/desktop/src/types.ts:971`); a reopened saved run is Historical (`projects/chirality-piping/apps/desktop/src/features/results/HistoricalRunContext.tsx:11`). | Every run of the project is kept, numbered and named; an earlier run chosen from the list is Historical (§5.3). The Historical designation and what it may not drive are kept exactly. | G-10 |
| 17 | Node identity | Node ids are free strings (`projects/chirality-piping/schemas/model.schema.yaml:1215`). | Numeric node numbers the engineer can renumber, with every reference rewritten in one operation (§3.2). | G-01 |
| 18 | Status label forms | Two curated labels, "Review required" and "Inputs needed" (`projects/chirality-piping/apps/desktop/src/App.tsx:3768`, `:3769`). | Only the registered display forms, each with its authority domain and its raw token reachable in place (§5.4; `DEC-102`). | G-33 |
| 19 | Text the product no longer displays, and its name | The shell carries the maturity sentence and an acceptance short variant (`projects/chirality-piping/apps/desktop/src/App.tsx:3282`); the export panel and its packet name another vendor's product in identifiers and copy (`projects/chirality-piping/apps/desktop/src/features/caepipe-mbf/CaepipeMbfExportPanel.tsx:151`). | Neither sentence on any surface (`DEC-100`, `DEC-105`); the product is SWBPIPE only (`DEC-101`); the export names no other vendor's product (`DEC-103`); §5.5, §8.3, §9.1. These are display decisions with registry, lint and identity-layer acts behind them, executed together in an owner-authorized tranche. | G-33 |
| 20 | Keys as the only way | Not audited against the product; the design's own V1 had key-only actions. | Every action has a visible primary control and a key only accelerates it (§2.8; `RETURN.md` §4). New controls, not new operations. | G-33 |

What is not a semantic change: the one mutation route, the applier, the undo stack's one checkpoint per batch, the Current and Historical designation, the stale-response guards and the persistence compatibility are kept as they are (§2.2, §2.4; the implementation-handoff preparation §2 constraints 1 to 3).

---

## 13. Change log V1 to V1.1

Every change of this revision with its source. "Ruling" is the D-71 ruling record of 2026-09-18, "addendum 1" and "addendum 2" its two addenda; "DS" is design system V1.2; "prep" is ROOT's revision record; "brief" is sealed brief UX-SPEC-02.

| # | Change | Where | Source |
|---|---|---|---|
| 1 | The product is SWBPIPE and only SWBPIPE: the title, the toolbar, the status bar's About control, About, Help, and the copy rules; the longer name and the wordmark distinction are dropped; nothing is appended to the name. | title, §5.5, §9.1, §9.3 | ruling item 3; `DEC-101`; DS §7.5 |
| 2 | The maturity sentence is removed from the product: M-01 retired and its homes removed; the status bar's information control opens About directly. | §5.5, §9.1 M-01, §9.3 | addendum 2; `DEC-105`; DS §5.2, §7.4 |
| 3 | The acceptance sentence and its variants are removed from every surface: M-02 retired; the results header's disclosure holds the run identity line alone and is named "Run identity"; the Review page's columns begin under the header; the probe and §7.7 no longer mention it. | §3.6, §4.3, §7.1, §7.7, §9.1 M-02 | ruling item 2; `DEC-100`; DS §5.1, §5.5, §7.4 |
| 4 | Status and evidence labels only from the registered table, hyphenated forms included, each with its domain in the chip and its token reachable in place; the evidence chip only when the run record carries evidence and only for a Current run. | §5.4, §3.6, §9.1 M-08 and M-09, §9.3, §10.1 | ruling item 4; `DEC-102`; DS §2.3 |
| 5 | Hanger tables are a user-imported library class; hanger selection is the row's expansion and carries the content boundary's short variant once, verbatim; the no-library caption. | §3.7, §8.1, §9.1 M-05 | ruling item 5; `DEC-103`; DS §5.1 |
| 6 | The historical-run wording is the text the product renders; the unrendered phrase V1 quoted is dropped; M-13's source lines are the rendered ones. | §2.6, §5.3, §9.1 M-13, §9.3, §10.4 | ruling item 6; `DEC-103`; R-7; DS §5.1 |
| 7 | The Checked mark is ruled option A: its classification, its storage boundary and its exact words are stated; the open item closes; the Review outline's checked counts, the "Stale checks" filter name and the report-text exception are withdrawn. | §6.7, §9.2, §9.3, §10.7, §11 question 8 | addendum 1; `DEC-104`; DS §7.3 |
| 8 | Agent cards carry one of the five class words; the agent-authored "Note" becomes "Evidence summary" and Note stays the engineer's own; the forbidden words bind all agent text. | §6.2, §6.6, §7.5, §9.3 | ruling item 8; `DEC-103`; DS §5.4 |
| 9 | The export never names another vendor's product: the command, the sheet's title, the loss report's heading and entries, the paste band's header list, §3.2's prose and the copy rules; no attribution line and no compatibility flag; this document's prose says "the model batch file grammar". | §1, §3.1, §3.2, §8.3, §9.3 | ruling item 9; `DEC-103`; DS §5.6 |
| 10 | Q-15: the agent column reopened while the inspector is docked; the slide-over state; the state table of the two panels. | §5.1, §6.1, §10.9 | prep §2; ruling; DS §0 |
| 11 | Q-16: the run log's placement, its opening rule, its rows and glyphs, and its state table. | §5.3 | prep §2; DS §5.2 |
| 12 | Q-17: "Compare with…" in the iteration menu; the compared-with combobox and Show edits only while comparing; state table. | §7.1, §7.4 | prep §2; DS §5.5 |
| 13 | Q-18: the decided row's receipt line and the card row's state table. | §6.3 | prep §2; DS §5.4 |
| 14 | Q-19: Export… on the Review header is a plain button; the window's one accent is Run. | §7.1 | prep §2; DS §5.5 |
| 15 | Q-20: the fitted camera, with its state table; Fit first in the HUD at every width. | §4.6, §4.7 | prep §2; the owner's words on Q-20 (D-71 ruling record); DS §0, §5.6 |
| 16 | Q-21: after a failed run the bar shows what the record carries, and nothing when it carries nothing; table. | §5.4, §10.3, §11 question 11 | prep §2; R-3; DS §2.3 |
| 17 | Q-22: the comment stream's filter row, four chips and the Kind menu; table. | §7.5 | prep §2; DS §5.5 |
| 18 | The pointer rule: stated once; every keyboard table gains the control each key accelerates; every key-only action in §3, §4, §5 and §6 is given its control (the audit is `RETURN.md` §4); copy names the control first and the key after it; empty-state hints rewritten. | §1, §2.8, §3.1, §3.2, §3.3, §3.5, §3.6, §3.7, §4.2 to §4.6, §5.1 to §5.3, §5.5, §6.1 to §6.3, §6.7, §6.8, §7.2, §9.3 | the owner's words on Q-20 (D-71 ruling record), generalized by ROOT's reading, stated there so it can be corrected; DS §5, §7.6; brief item 3 |
| 19 | R-1: §10.9's floors reconciled to `layout.canvas.min` 220 px, the 300 px Both-view inspector and the narrow-case order; V1's 320 px and 480 px floors are withdrawn; the geometry table. | §2.3, §5.1, §10.9 | prep §1 R-1; DS §0 |
| 20 | R-2: every citation of the design system reconciled to V1.2 by name, section and token (inspector widths, the HUD's ten tools and its View menu, the footer's selection group and edit chip, the expansion chevron, the Run identity control, the toolbar's Undo, Redo and Inspector toggle, toast rules, label cycle order, the specimen's line for M-16). | throughout; §9.1 M-16 | prep §1 R-2; DS |
| 21 | R-3: the chip after a failed run follows the run record. | §5.4, §10.3 | prep §1 R-3 |
| 22 | R-4: the Type column offers the engine's kinds only, now in agreement with DS §5.1; V1's departure is no longer one. | §3.2 | prep §1 R-4; DS §5.1 |
| 23 | R-5: Canadian English and the product's spelling "Analyze" for the menu. | §5.5, §9.3 | prep §1 R-5; DS §7.1 |
| 24 | R-9 and result integrity: run standing specified separately for Current, Stale and Historical on every results surface, with transitions; a Stale run is held on a Historical record's terms and drives no overlay, chip or readiness cue; the stale band's sentence as settled by ROOT; the clause that said a run remained the basis after a model change is removed everywhere, the consequence line included; V1's colour on unchanged elements, stale probe and historical probe and legend footer are withdrawn. | §2.1, §2.3, §2.5, §2.6, §3.6, §3.7, §4.1, §4.3, §4.5, §5.3, §5.4, §6.3, §7.3, §7.6, §8.4, §10.3, §10.4, §11 question 25 | brief item 5; prep §1 R-9; DS §5.1 run standing, §8 items 16 and 17; the implementation-handoff preparation §2 constraint 3 |
| 25 | One mutation route: tables and canvas are projections and editors of the canonical model; every engineering action, a future harness included, is a typed operation through the existing applier; a human's and an agent's action are the same operation. | §2.2, §2.4 | brief item 6; the implementation-handoff preparation §2 constraints 1 and 2 |
| 26 | Semantic changes named as changes, with both sides cited and the gap each depends on. | §12 | brief item 7; RESEARCH-G §2; the implementation-handoff preparation §4 |
| 27 | The probe's Model- and Loads-stage readings are withdrawn: the tool reads results and needs a Current run. | §4.3 | DS §5.1, §5.6 |
| 28 | The Review rail item is enabled by a Current solved run and stays reachable while that run is Stale; a Historical run enables nothing; report readiness per standing. | §2.3, §2.6, §7.6, §10.4 | DS §5.1 run standing; brief item 5 |
| 29 | The Run button opens the run log while running and after the run has ended, until the model changes; "Run again" is in the log's footer. | §5.3 | DS §5.2 |
| 30 | Open questions: 8, 11, 12 and 15 updated; 25 to 27 added. | §11 | this revision |
| 31 | Citations re-verified at `HEAD`: the specimen's line for M-16 moved with V1.2; M-13's lines are the rendered ones; M-17 cites the list's line as well as its section. | §9.1 | brief item 8 |
| 32 | Editorial: the hanger table's Variation formula is written in words, because its bars broke the table's columns; the two frame-era remarks that V1 allowed several expansions "while the frames show one" are replaced by the design system's rule. No behaviour changes. | §3.1, §3.7 | this revision |
| 33 | Correction 1 (ROOT, from REVIEW-03): the pointer rule's source is stated in two parts, the owner's words, which were about Q-20 only, and ROOT's reading that generalizes them; rows 15 and 18 of this log cite it so. | §2.8, §13 rows 15 and 18 | ROOT correction 1; D-71 ruling record, "The frames' questions Q-15 to Q-22" |
| 34 | Correction 1: the control names "Check rows", "Check again" and "Clear check" are attributed to their derivation from the addendum's "set on one row or many" and its stale text, not to `DEC-104`'s list of words. | §6.7, §9.3 | ROOT correction 1; the first addendum |

---

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
