# UX specification V1 — SWB Piping Designer

Status: V1, proposal for ROOT's acceptance. Written by the HELPS_HUMANS design manager under brief UX-SPEC-01 (phase 3 of the SWB Piping Designer interface program), 2026-09-18. Basis, read in the brief's order: the design brief V1.4 (§2–§7), the direction record (§2, §5, §10, §11), design system V1 (§0, §4, §5, §7, §8, with `tokens.json` 1.0 and `specimen.html` as they stood at the start of this work), the fifteen mock frames with `MOCKS_V1.md` and `sample_model.md`, RESEARCH-E §3 and §6, RESEARCH-C §1, §3 and §4, RESEARCH-B, and the product source named in the brief, read only for the names of operations, types and schemas. Where design system V1 and the direction record §11 differ, §11 governs here; a V1.1 revision of the design system is being written in parallel and was not read.

Companions: [`OPERATIONS_MAP.md`](OPERATIONS_MAP.md) maps every control and gesture in this document to the operation it invokes and to the existing operation, capability entry or typed interface that provides it, or names the gap. [`RETURN.md`](RETURN.md) records what was read, what was produced and what is uncertain.

Nothing in this document is a product claim or an acceptance. It describes the intended behaviour of a redesign that is not built, over an engine that stays as it is.

---

## 1. Scope and reading guide

**What this document specifies.** Behaviour: what every surface does, in which states, in response to which events, and which operation of the existing product each control invokes. It covers the shell (rail, views, toolbar, status bar, palette, menus), the tables of the four stages, the canvas, the inspector, issues, the run, the agent panel and its protocol, the Review page, libraries, rules, export and handoff, the mandatory disclosures, and the empty, error and edge states. Every string it specifies is bound by the copy rules in §9.3.

**What it leaves to the design system.** Appearance: tokens, type, spacing, colour in light and dark, iconography, glyph forms, motion, densities and the shell geometry (design system §0–§6). This document names components by the design system's names and states only what they do.

**What it leaves to the engine.** Numerics and semantics: how a mechanics solve, a rule evaluation, a hash, a unit conversion, a file grammar or a diagnostic is computed. The specification names what the interface asks of the engine and never how the engine does it. Where the engine has no operation for a behaviour, the operations map says **GAP** and §11 states the question.

**Precedence.** Design brief V1.4 §2–§7; then the direction record §2, §5, §10 and §11; then design system V1; then the mock frames as worked examples. Where a frame drew a moment the design system had not specified, the frame's decision is adopted here unless §11 or this document says otherwise; each such departure is listed in `RETURN.md`.

**How to read a state table.** Each surface has a state table with four columns: *State*, the name used elsewhere in this document; *Shown*, what is on screen in that state; *Enabled*, which controls act; *Leaves on*, the events that move it to another state, written `event → next state`. Events are user actions (a key, a click, a gesture), engine returns (a solve completes, the engine rejects an operation) or agent actions (a proposal lands). A state table lists only what differs between states; the surface's constant behaviour is in the prose above it.

**Notation.** Keys are written with the macOS symbols the design system uses: ↩ Return, ⇥ Tab, ⎋ Escape, ⌫ Delete, ⌘ ⌥ ⇧ ⌃ modifiers, ← → ↑ ↓ arrows, Space. "Row operation" means one structured operation through the one route of §2.4. "Table" means the one table component of design system §5.1 unless a named instance is meant. "The run" means the current solve basis (§2.6). Units are SI in every example; the display-units toggle changes presentation only (§2.7). Node numbers in examples are the sample model's.

**Source citations.** Paths are relative to the repository root and carry a line number, for example `projects/chirality-piping/apps/desktop/src/types.ts:645`. They name existing operations, capability entries, typed interfaces and schemas; nothing of the current presentation is carried over.

---

## 2. The model of the interface

### 2.1 One spine

Every surface reflects five things, and only these, so that no two surfaces can disagree:

| Spine element | What it is | Who owns it | Where it shows |
|---|---|---|---|
| **Model** | The set of tables (§3): layout, restraints, loads, node data, load sets, load cases and combinations, plus the project's library and rule-pack references. The canvas, the inspector, the issues list and every export are derived from it. | The engine's model document, mutated only through the one route (§2.4). | Every table; the canvas; the inspector; the footer counts. |
| **Selection** | One ordered set of selected entities (rows) with one primary entity, and one focused cell. | The interface, product-wide. | Table selection bands, canvas halos, the inspector, the status bar ("Node 40 · 1 row"), the palette's context. |
| **Issues** | One live list of findings against the model and the run, each classed and pointing at its entity (§5.2). | Derived: model validity from the operation layer, missing data from the readiness check, run findings from the run record, provenance and content findings from libraries. | The issues drawer, the rail's Issues count, the toolbar count, the status bar count, gutter and cell marks, canvas halos. |
| **Run** | The current solve basis: the most recent run whose model hash equals the current model's, or none. A run is an immutable, named record (§5.3). | The engine's run record. | The Run button, the status chips, the results tables, the legend and probe, the Review page's live tables, the rail caption. |
| **Revision** | The model's content hash after the last applied operation, and the sequence number of that operation. | The engine (hash), the ledger (sequence). | The project save state, the changes-since control, the stale band, the Checked mark's binding. |

Stages are not modes: switching stages changes which tables are on screen and nothing in the spine.

### 2.2 The tables are the model; the canvas is derived

Every entity the product knows is a row in exactly one table, and a row has no hidden fields: what the inspector shows for a row is the row's cells plus the row expansion's fields (§3.2). The canvas draws the rows and holds nothing they do not; it can select, probe, and start row operations through its few gestures (§4.4), and nothing else. The agent acts only through row operations (§6). A value the engineer sees in the canvas, the inspector, a results table, a report or an export can always be found in a table cell or in a run record.

### 2.3 Stages and views

Four stages on the rail, in the order the work is done and read; three views per stage.

| Stage | Tables (tab strip order) | Canvas shows | First-open view |
|---|---|---|---|
| Model | Layout · Restraints · Node data | Geometry, restraint glyphs, load vectors, labels, the routing draft, proposal ghosts | Both |
| Loads | Cases · Load sets · Loads · Wind · Seismic | Geometry with the selected case's loads in the case's colour and every other load at 40 % | Table |
| Results | Summary · Stresses · Displacements · Restraint loads · Restraint summary · Forces and moments · Hangers | Result colour, legend, probe, deformation | Both |
| Review | The Review page (§7); no tab strip | No canvas; the report figure is a block in the page | Table |

At the rail's foot: Libraries, Rules, Issues (§8, §5.2). They are pages that open over the surfaces of the current stage and return to it on ⎋ or on the rail item; they do not change the stage's remembered view.

**Views.** Table: the stage's tables at full surface width, no canvas; the inspector is not shown and the row expansion (§3.1) serves its purpose. Model: the canvas with a docked inspector on its right and the stage's current table in a bottom drawer. Both: table and canvas side by side at the remembered split; the inspector docks on the canvas's right and the canvas shrinks while it is open; the table never reflows (direction record §11 decision 1). The view switch is the three-segment control in the toolbar (⌘1 ⌘2 ⌘3).

**The per-stage memory rule.** Each stage remembers the view it was last left in, for the project, and restores it when the stage is re-entered. Before a stage has ever been left, its first-open default applies. The memory is saved with the project's interface state, never with the model.

**What persists across a stage or view switch.** The selection, the undo stack, the issues list and the run (the spine); each table's scroll position, sort, filters and column widths; the canvas camera, label mode, hidden and isolated sets, deformation state and legend range; the inspector's open state per view; the drawer heights; the agent column's open state (product-wide). The probe's pinned card persists while the Results stage is current and closes when the run changes.

**The rail's states.** Results and Review are disabled until the first run of the project completes or a saved run is reopened, with the tooltip "No run yet"; Review is disabled after a failed run when no solved run exists ("No solved run"). Captions under a rail label carry a state when there is one to name: Results carries "Failed" after a failed run, "Stale" when the model changed since the run (§2.6), "Historical" when a reopened saved run is shown (M-13). The Issues item carries the issue count.

### 2.4 The one route

Every mutation of the model is one structured operation, or one batch of structured operations, submitted to the engine's operation layer, checked by it against the schema, the constraints and the units, and applied only by it. The routes that produce operations are:

| Path | Produces | Shown before apply as | Applies on |
|---|---|---|---|
| A typed cell, a row insert, a row delete, a row-expansion field, an inspector field | one operation | the cell in its editing state; the value as typed | ↩, ⇥, arrow commit, or focus leaving the cell |
| A canvas gesture (route, add restraint) | one operation or one batch | the draft ghost and the draft row (§4.4) | ↩ in the gesture |
| A paste | one batch | the paste band with mapping and preview (§3.1) | Paste ↩ |
| A generation (load cases from the rule pack, self-weight case) | one batch | the generation band, same anatomy as the paste band (§3.5) | Generate ↩ |
| A library application (a section or material chosen in a cell, a hanger size chosen) | one operation | the cell or the candidate row | the choice |
| An accepted proposal row, rows or batch | one batch per accepted unit | the banded rows, the ghost and the card diff (§6.3) | Accept |
| Undo, redo | the inverse of one checkpoint | — | ⌘Z, ⇧⌘Z |

The engine checks every operation against the schema, the constraints and the units before it is applied; the interface never mutates the model itself, never keeps a second copy that can diverge, and never applies an operation the engine rejected. A rejected operation shows where it came from: an invalid cell (§3.1 cell states), a blocked paste band, a proposal card that "cannot be accepted", a gesture hint. Diagnostics the engine returns with an applied operation join the issues list.

Author attribution: an operation carries its author type, the engineer or the agent. An accepted proposal is applied as the agent's operation accepted by the engineer, never rewritten as the engineer's own (§6.6).

### 2.5 The undo model

One undo stack for the whole product, across stages and views. One checkpoint per applied operation or batch:

- A typed cell is one checkpoint. A propagation that follows a typed cell (§3.2) is part of the same checkpoint.
- A paste is one checkpoint. A generation is one checkpoint.
- A route commit (node, element and any bend the direction change inserted) is one checkpoint.
- An accepted proposal row is one checkpoint; an accepted selection of rows is one; Accept all is one.
- A row delete with its attachments is one checkpoint.

⌘Z restores the model and the selection as they were before the checkpoint and shows the toast "Undid: Gap at node 20 · 3 mm ← 0 mm" with Redo; ⇧⌘Z reapplies. Undo never re-establishes a run: a run made against the earlier revision stays historical (§2.6), and results are not restored by undo. Undo is never offered for a run, a snapshot, an export, a library import or a Checked mark, which are not model edits; the Checked mark's currency follows the content (§6.7), so undoing a change may make a stale mark current again. Diagnostics are recomputed after every undo and redo.

### 2.6 What happens to results when the model changes

A run binds to the model hash it solved. The moment any operation is applied after a run:

1. The run stops being the current solve basis. The status chips drop (§5.4); the rail's Results caption reads "Stale"; the results header carries the stale band: "Model changed since Run 03: gap at node 20 set to 0 mm at 16:31. The values below are from the model as solved; Run 03 stays the solve basis for these values until the next run." with "Run again".
2. The results tables keep the run's values, hatched (design system §5.1 cell state *Stale*), readable and copyable; the legend reads "Run 03 · model changed since"; elements added or changed since the run draw unsolved in the canvas.
3. The run record is kept, unchanged, as a historical run (§5.3); the run list shows it with the hash it solved.
4. The Run button is enabled if nothing blocks it; a new run replaces the stale set.

Results are never shown against a model they were not computed from without the band; a run's results are never silently reused.

### 2.7 Display units and precision

The units toggle in the toolbar (SI, US, or the entered units) changes presentation only: no stored value changes, every value is converted deterministically for display, and a value whose conversion is unavailable shows in its entered unit with a tooltip that says so. Column headers, inspector rows, the probe, the legend and reports carry the displayed unit. Precision is one setting per column (design system §1.1) and applies to display, copy and report tables; the stored value keeps its entered precision.

---

## 3. The tables

### 3.1 What every table does

One component serves every table (design system §5.1). This section states the behaviour common to all instances; §3.2–§3.8 state each instance's columns, validation and specifics.

**Header.** The column name with its unit in square brackets. Hover or ⌥↓ on a header opens the column menu: Sort ascending, Sort descending (⌘↓ on a focused cell of the column sorts by it), Data bar (numeric columns with a scale; ratio columns also "1.0 tick"), Colour by scale (ratio columns), Filter… (a comparison and a value for numeric columns; a value list for enumerated ones), Unchecked rows only, Precision, Unit (display), Hide column, Pin column, Width, Reset column. Sort is a view: it never reorders the model's rows; the footer shows a "Sorted by ratio" chip with a clear control, and ↩-down movement follows the file order even while sorted. Filtering hides rows; the footer says "3 of 15 rows".

**Selection.** Clicking the gutter selects the row; ⇧-click extends; ⌘-click toggles; ⇧↑ ⇧↓ extend from the keyboard; ⌘A selects all. Clicking a cell focuses it and selects its row. The selected rows are the product selection (§2.1): a layout row selects its node and the element arriving at it; a restraint row selects that restraint; a load-case row selects the case; a results row selects the element or node it reports. A joined row's selection is the attachment row, and its node row shows the selection band lightly to say where it belongs.

**Keyboard model.** The spreadsheet idiom of design system §5.1, fully operable without the mouse; the table below restates it with the additions of this document.

| Key | Not editing | Editing |
|---|---|---|
| a character | starts editing, replacing the value | inserts |
| ↩ | starts editing the focused cell; on the last row's last editable cell of the layout table, adds a row (§3.2) | commits and moves down one row (⇧↩ up) |
| ⇥ ⇧⇥ | moves right / left; wraps to the next row's first editable cell | commits and moves right / left |
| ← → ↑ ↓ | move the focus one cell; ← from the Node cell focuses the gutter; → from the last column focuses the marks column | ← → move the caret; ↑ ↓ commit and move |
| ⎋ | clears the multi-row selection; then closes the open row expansion, drawer or popover: one level per press | cancels the edit and restores the value |
| ⌘↩ | opens the row expansion (on a marks slot, that mark's joined row; on the Type cell, the element fields; on a load-case row, the combination editor) | commits, then opens |
| ⌥↩ | inserts a row below with the next node number | commits, then inserts |
| ⌘A | selects all rows | selects the text |
| ⌘C | copies the selected rows as tab-separated text with a header row, in display units and column precision | copies text |
| ⌘V | pastes; a single value pastes into the focused cell; more than one row, or a header row, opens the paste band | pastes text |
| ⌫ | on a row selection: deletes the rows with an undoable toast; on a cell: clears the value (an empty required cell shows the required mark) | deletes text |
| Space | on a gutter or marks slot: opens the mark's popover; on a switch cell: toggles | inserts |
| ⌥I | opens the row's marks popover listing every mark the row carries | — |
| ⌘⇧A ⌘⇧R | Accept, Reject the focused proposed row (or every selected proposed row) | — |
| ⌘⇧K | marks the selected rows Checked; again clears | — |
| ⌘Z ⇧⌘Z | undo, redo | text undo, redo |
| ⌘K | the command palette | — |
| ⌘F | focuses the table's filter field in the footer | — |

**Cell states** (design system §5.1): Entered, Propagated, Read-through, Proposed, Selected, Editing, Invalid, Required-empty, Stale, Historical, Disabled. Cell rules apply on commit: a cell that fails the interface's own rule (type, range, enumeration, uniqueness, reference) becomes *Invalid* with the message under it and keeps the typed text until corrected or escaped; a cell the engine rejects after commit reverts to its previous value and shows the engine's diagnostic in the same popover and in the issues list. A committed cell whose value the engine accepted with a warning (out of range, provenance) keeps the value and gains the warning corner.

**Validation vocabulary.** Numbers accept a leading sign, a decimal point and, when the display unit is US and the column is a length, the feet-inch-fraction forms RESEARCH-E §3.5 documents (`10'8`, `1'6-3/8`); a typed unit suffix that matches the column's dimension is accepted and converted for display, and one that does not is *Invalid* ("Expected a length"). Enumerations are comboboxes that also accept typed text with completion. References (a section name, a material name, a load set, a node) must resolve; a reference that does not resolve is *Invalid* with "No section named P4 · Libraries" as a link. Nothing is ever pre-filled to satisfy a rule (C-26 to C-29).

**Gutter and marks column.** As design system §4: the origin slot and the state slot on the left; the three attachment slots on the right (layout table only). An empty marks slot shows a plus on hover with "Add restraint…", "Add load…" or "Add node data…"; the plus opens the joined row empty in Table view and the inspector's attachments section in Model and Both.

**Row expansion.** ⌘↩ or a click on a marks slot opens a block under the row, indented by the gutter, with the attachment table's own header and the node's rows in that table, editable in place with the same keyboard model, and an "Add restraint…" row at its end. One expansion per table is open at a time in the frames; this document allows several, each closed by ⎋ from inside it or by its chevron. In Model and Both views ⌘↩ focuses the same row in the inspector instead of expanding. On a load-case row the expansion is the combination editor (§3.5).

**The paste band.** ⌘V with more than one row or a header row opens a band above the target rows on `surface.raised`: a mapping row in which each source column names its target in a combobox, a count ("42 rows from the clipboard · after node 130 · 2 columns ignored · node IDs taken from the source"), a preview of the first three rows in the target grammar drawn as draft rows, and Paste (↩) and Cancel (⎋). Recognised source headers are pre-mapped, case-insensitively, including the CAEPIPE Layout window's names: Node, To, From, Type, DX, DY, DZ, X, Y, Z, Section, Sect, Material, Matl, Load, T1…T10, P1…P10, Note, Comment. An unrecognised column is offered "Ignore" or any target. The first row is treated as a header when at least half its cells match a header name; otherwise the mapping row shows column letters. Node IDs are taken from the source when the mapped Node column has values, and auto-incremented when it does not. A source row that would be invalid (a section name that does not resolve, a From that does not exist) is shown invalid in the preview; Paste stays enabled, and the offending cells land empty with the required mark and an issue, because a paste never fabricates a value. The paste applies as one batch and one checkpoint; the rows carry the Entered origin with the paste time.

**Footer.** The counts line: rows and selected count, then counts that act as filters when clicked (propagated cells, proposed rows, unchecked rows, checked and stale, issues on this table), then the active filter chips with a clear control, then the Changed since… control (§6.8), and on the right the table's switches (Read-through, Origins) as outline chips. The footer also holds the filter field (⌘F).

**Filters common to every table.** Unchecked rows; Agent-origin rows (origin Accepted or Proposed); Changed since…; Issues (by class); Origins (Entered, Accepted, Propagated, Generated, Imported); and the table's own (§3.2–§3.8). Filters combine; the footer says how many rows remain.

**Empty state.** A table with no rows shows one line of hint text under the header row in `text.secondary` naming the way to add the first row (each instance's wording is in its section) and, in the layout table, the first row already present (§10.1).

**Copy and export.** Every table can be copied (⌘C) and exported from its column menu's "Export table…" as CSV or JSON with the display units and the table's provenance line (run identity for results tables); results exports carry the run's binding set (C-98, §5.3).

### 3.2 The layout table

One row per node, carrying the element that arrives at it, in the CAEPIPE grammar (RESEARCH-E §3.2, §6.1). Rows are in file order: the order they were created or inserted; the order is the export order and never changes by sorting.

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

**Offsets and absolute coordinates.** The DX DY DZ group and the X Y Z group are one switchable column group; the header shows which. In offsets mode a cell is the To node's offset from the From node; committing an offset moves the To node and every node downstream of it along the From chain by the same vector, so that the rest of the run keeps its shape, as a CAEPIPE veteran expects; the toast says "Moved node 40 and 12 downstream nodes". In absolute mode a cell is the node's coordinate; committing it moves only that node, and the offsets of its neighbours change. Both are one operation each (a batch of node moves in offsets mode, one checkpoint).

**Branches.** A branch is a row whose From names an earlier node. It carries offsets from that node. The branch-connection kind at the From node (welding tee, sweepolet, weldolet, fabricated tee, extruded tee, radiused branch, branch on thickened pipe) is node data on the From node's row (§3.3), and a branch whose From node has no branch-connection kind raises the issue "Blocks rule check · Branch connection kind missing · node 70" (a SIF is rule data, never inserted silently, C-28); the solve is not blocked.

**Element types and their type-specific fields.** The Type cell's expansion (⌘↩ on Type) and the inspector's element section carry the fields of the element's type, every one user-entered, none pre-filled:

| Type | Fields in the expansion |
|---|---|
| Pipe | none beyond the row |
| Bend | Bend radius [mm] (the section's bend radius when the section record carries one, shown as propagated; else required); bend wall thickness; bend material; flexibility factor (user value); SIF in-plane and out-of-plane (user values); intermediate nodes with angle (up to two) |
| Valve | Weight [N]; thickness factor; insulation factor; additional weight; offset (x, y, z) |
| Reducer | OD at end 1 and end 2 [mm]; wall at end 1 and end 2 [mm]; cone angle [deg] (each end's values propagate from the sections on either side and are shown propagated) |
| Rigid | Weight [N] |
| Expansion joint | Axial, lateral and torsional stiffness [N/mm, N/mm, N·m/deg]; pressure thrust area [mm²]; bending stiffness; weight |

The Type enumeration offers exactly the kinds the engine represents; the kinds the target file knows and the engine does not (slip joint, ball joint, hinge, tie rod, elastic element, beam, cut pipe, jacketed pipe, jacketed bend, miter bend) are not offered, and the enumeration's footer line says "Other element kinds are recorded scope decisions · About › Scope and limitations" (C-23). They are gaps or non-goals in the operations map.

**Propagation rule.** Section, Material and Load propagate by connectivity: a row without an entered value takes the value of the row of its From node (its previous row for a continuation, the tee node's row for a branch). Propagation is recomputed on every change; a propagated cell shows the corner tick, its tooltip names the source node, and typing over it makes the cell entered and starts a new propagation from that row. Clearing an entered cell (⌫) returns it to propagation when a source exists, and to required-empty when none does. A propagated value is written to the export like an entered one (RESEARCH-E §6.1, section and material rows; see §8.3).

**The Load set column and read-through.** The Load cell names a load set (§3.4); the T and P columns show the set's values on `surface.sunken` with the corner tick; the tooltip reads "Read through from load set OP1 · typing over edits the set or forks a new one". Committing a change in a read-through cell opens a two-choice popover under the cell:

- **Edit OP1** — "changes T1 on 12 rows that use OP1": applies the edit to the set (one operation).
- **Fork a new set** — a name field prefilled with the next free name ("OP3"): creates the set with the edited value, assigns it to this row, and re-propagates the Load column from this row down to the rows that were propagating from it (one batch, one checkpoint). Rows that had entered their own set are untouched.

↩ chooses the highlighted choice (Edit when the set is used by one row, otherwise Fork, so that the common case is one key). The read-through group is on by default in Table view and off in Both view and in the Model-view drawer, where the Load cell's tooltip lists the values; the footer chip toggles it. When the sets carry different numbers of pairs, the columns show the largest count and unused pairs read "—".

**Numeric node IDs: insert, split, renumber.**

- *New row.* ↩ on the last row's last editable cell, or ⌥↩ on any row, or the "Add row" toolbar control, adds a row whose node number is the next multiple of the node increment above the highest number in the model (increment 10 by default, set in Preferences). The new row's From is implied from the row above it; its Type is Pipe; Section, Material and Load propagate.
- *Insert between.* ⌥↩ on a row that is not the last inserts a row below it. Its number is the integer midpoint between the neighbours when the gap allows a whole number that is not taken (20 and 30 → 25); otherwise the next free multiple of the increment above the model's highest number. The row below keeps its number and its typed From; if its From was implied, it now implies the inserted node. Neighbours are never renumbered.
- *Split element.* "Split element…" on a row (row menu, palette, or ⌘⇧S) asks for a distance from the From node (the field is prefilled with half the element's length, shown as a proposal the engineer confirms or types over) and inserts a node inside the element: the new row takes the row's From and the first part of the offsets; the existing row's From becomes the new node and its offsets the remainder; attachments stay on their nodes. One operation, one checkpoint.
- *Renumber.* "Renumber nodes…" (palette, row menu) offers "From 10 by 10 in file order" with editable start and increment and a preview of old → new for every row; it rewrites every reference in one operation. Node numbers are otherwise never changed by the interface. Import keeps source numbers.
- *Editing a Node cell* renumbers that node alone, with the same reference rewrite, and is *Invalid* when the number is taken.

**Delete rows.** ⌫ on a row selection deletes the selected rows' elements and nodes, and the attachments of the deleted nodes, in one batch; a row whose From was the deleted node heals to the deleted row's From with its offsets re-derived from positions so that nothing else moves. The toast lists what went: "Deleted 2 rows · nodes 30, 40 · 1 restraint · Undo".

**Marks column.** Each attachment class the node has shows as a mark in its slot (restraint, load, node data); the tooltip summarises the rows ("Restraint · +Y, gap 3 mm, μ 0.30 · ⌘↩ opens the row"); ↩ or a click on the slot opens the joined row in place (Table view) or in the inspector (Model, Both); the mark's popover (Space) lists every row of that class on the node with a link each. A node with two rows of one class shows the same mark with a count badge.

**Filters specific to the layout table.** Has restraint; Has load; Has node data; Branch rows; Start rows; Propagated cells (rows with any); Type (by kind).

**Empty state.** A new project's layout table holds the start row 10 and a first element row 20 with DX in the editing state (§10.1); the hint line reads: "↩ commits and adds row 30 · ⇥ moves right · ⌘V pastes rows · Section, Material and Load propagate from the row above once entered · the asterisk marks what the solve needs".

**State table — a layout row.**

| State | Shown | Enabled | Leaves on |
|---|---|---|---|
| Normal | values, origin glyph, marks | every cell editable | typing → Editing; ⌘↩ → Expanded; proposal lands → Proposed |
| Editing | the input with the unit after the caret; the text selected on entry | the input; ⎋ | commit → Normal (or Invalid); ⎋ → Normal |
| Invalid | the ring, the message popover, the typed text | the input; ⎋ | corrected commit → Normal; ⎋ → Normal with the old value |
| Expanded | the row plus its expansion block | the block's cells and add-row | ⎋ or the chevron → Normal |
| Proposed | the band, bar and diamond; old and new values | Accept, Reject; cells are read-only while pending | Accept → Normal with origin Accepted; Reject → Normal unchanged; engineer edits another cell of the row → the pending row becomes Stale-proposed (§6.3) |
| Draft | dashed gutter bar, muted values (a routing or paste preview) | none (the gesture or band owns it) | commit → Normal; cancel → removed |
| Checked / Checked stale | the check or the dashed check in the state slot | as Normal | content change → stale; ⌘⇧K → cleared |

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

**The joined-row expansion.** Opening a node row's mark shows, under the node row, that class's table for that node: the class's own header row, the node's rows, editable, and an add-row. Two classes can be open under one node at once (Table view). The expansion's caption reads "Restraints · node 20 · joined row · ⎋ closes". In Model and Both views the inspector's attachments section shows the same rows and opens the joined row in the drawer on request.

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

**Generated.** "Generate from rule pack…" opens a generation band (same anatomy as the paste band): the pack's name, version and checksum; the load sets found; a preview of the rows the generation will create; Generate (↩) and Cancel. Generation creates the primitive cases the load sets imply (W; T1…Tn; P1…Pn; D1… when displacements exist; SE1, WIN1 when seismic or wind inputs exist) and the combinations the pack declares for its checks (the sustained, operating, expansion-range and occasional forms), as one batch with the Generated origin and the pack identity; rows that already exist with the same name are skipped and the band says so. What the pack declares for generation is an engineering question (§11).

**Edited.** Any generated cell typed over makes the row "Edited by name · was …" and the row is never touched by a later generation. **Authored.** A row added with ⌥↩ or "Add case" is authored. **Regenerate** (in the band) replaces only rows still marked Generated and leaves edited and authored rows.

**The combination row expansion** (direction record §11 decision 14). ⌘↩ on a combination row, or a click on its Expression cell, opens a block under the row: the terms as chips with a factor field each ("W + 1.0 P1 + 1.0 T1"), the available cases and combinations dimmed as chips to add, a basis control (Sum; Difference of two states; Range envelope of several), Stress type and Rule comboboxes, Cancel (⎋) and Done (↩). Done applies the change as one operation per changed term (one batch, one checkpoint). A term that references a case not yet solvable (missing inputs) shows the triangle and the issue.

**Expressions are display-only.** The Expression column and the Rule expression column are one-way renderings; neither accepts typed text; no parser exists (M-15, DEC-037). The M-15 caption appears once in the Loads stage's header band: "Rule expression: Display only, not accepted as input".

**Filters.** Generated, Edited, Authored, With a rule, Without a rule, Stress type.

**Empty state.** "No load cases · Generate from the rule pack, or add a case with ⌥↩; both make the same rows".

**Wind and Seismic tabs.** Each is a small table of the case-level inputs the engine takes as user-entered values (wind: pressure, shape factor, direction, exposed spans by element; seismic: gravity acceleration or an axis factor per axis); each row is a case of that kind. Nothing is prefilled.

### 3.6 The results tables

Under the Results stage, in reviewer order: Summary · Stresses · Displacements · Restraint loads · Restraint summary · Forces and moments · Hangers. Every results table is read-only, copyable and exportable, shows the run's values only, and carries the results header above its tab strip.

**The results header.** The table name; "Run 03 · solved 15:21 · immutable"; the case selector (a combobox of the run's cases and combinations in load-case-table order); the Envelope switch; the evidence label chip beside the run name (M-09); the historical band when the run is a reopened saved run (M-13); the stale band when the model changed (§2.6); and the Information control whose disclosure carries the acceptance sentence (M-02) once for the results surface class and, under it in monospace, the run identity line (run, time, model hash, solver version, rule pack name, version and hash, settings identity), one click from any results table (direction record §11 decision 9).

**Case selector and Envelope.** The selector filters every results table to one case. With Envelope on, the selector stays visible and disabled reading "Case: all" (decision 8), and each table shows per row its governing case: for Stresses the case with the highest ratio (or the highest stress when no rule applies to the row); for Displacements, Restraint loads and Forces and moments, per component the extreme absolute value with the governing case in the cell's tooltip and "Envelope" in the Case column. The footer chip reads "Envelope: governing case per element". The envelope is computed by the interface from the run's rows; it invents nothing.

**Summary.** The one hero figure: the governing ratio with its rule ID and pack version ("0.72 · EXP-A1 · pack 1.2 · node 70 · EXP1"); then a table per stress type (Sustained, Expansion, Occasional …) of the governing element, case, stress, allowable and ratio; the maximum displacement per case with its node; the maximum restraint load per case; the run's counts (elements, cases solved, cases stopped, hangers designed); the status chips (§5.4) with their authority domains; and the diagnostics count with a link to Issues. When the run has no rule pack, the hero figure is the maximum stress with "no rule pack".

**Stresses.** Node · Element (From–To) · Case · Stress [MPa] · Allowable [MPa] · Ratio (with its data bar and the 1.0 tick) · Rule · Pack. Sorted by ratio descending by default. A row whose rule inputs are incomplete shows "—" in Allowable and Ratio and the rule's completeness state in the tooltip; the row's Rule cell shows the rule and "inputs incomplete". Expansion (⌘↩) shows the stress components the run reports for the row (axial, bending, torsional, hoop, longitudinal) with their locations. Filters: Ratio ≥ x; Node; Element; Case; Rule; Unchecked rows only.

**Displacements.** Node · Case · DX DY DZ [mm] · RX RY RZ [deg]. **Restraint loads.** Node · Tag · Type · Case · FX FY FZ [N] · MX MY MZ [N·m]; a one-way support's row also shows its state in the case (active, lifted off) from the run's diagnostics when the run reports it (C-58), never inferred. **Restraint summary.** Node · Tag · Type · then per component the maximum and minimum over cases, each with its case in the tooltip. **Forces and moments.** Element · End (node) · Case · FX FY FZ · MX MY MZ, with an "Axes: Global / Local" control in the header when the run reports both.

**Selection sync.** Selecting a results row selects its element or node in the tables and the canvas; the probe reads the same row (§4.3).

**Empty states.** Before any run the Results stage is disabled (§2.3). After a failed run: "No results — Run 02 stopped in OPE1 (case 3 of 6) at the iteration limit. A run's results are one immutable set, so a stopped run has none. Change the support at node 20 or the iteration limit in Run settings, then Run again." After a run with no rule pack, the Stresses table shows stresses without Allowable, Ratio, Rule and Pack, and the header line says "No rule pack · set one in Rules to see ratios".

### 3.7 The hanger table

Under Results, because the design pass needs a solve (direction record §10 recommendation 6). One row per variable-spring or constant-support location (a Restraints row of those types).

| Column | Content |
|---|---|
| Node · Tag · Type | from the Restraints row |
| Design load [N] | from the run's design pass: the vertical load at the location in the weight case with the location held |
| Travel [mm] | from the design pass: the vertical movement at the location between the weight case and the operating case named in Run settings |
| Library | the user-imported hanger table named on the Restraints row, with its provenance line above the table ("Vendor-A springs · user import 2026-09-15 · 24 sizes · source recorded in Libraries") |
| Size · Rate [N/mm] · Cold load [N] · Hot load [N] · Variation [%] | the selected record's values and the resulting loads; Variation = |cold − hot| / hot |
| State | Not yet designed (no run since the row was added); Designed in Run 03; Selected (a size chosen); Stale (model changed) |

**The two-pass ritual made explicit.** Pass one is the run: with a hanger location in the model, the run reports the design load and travel per location and the table fills. Pass two is selection: ⌘↩ on a row lists the library's candidate sizes whose working range contains the hot load and whose variation is within the row's Max variation, sorted by variation, with the ones outside the limit dimmed and their reason; choosing one writes Size, Rate, Cold load and Hot load onto the Restraints row (one operation, origin Entered with the library record as provenance) and the canvas glyph takes the can-and-rod form with its size label. A location with no library named shows "Name a library on the restraint row" and a Blocks-rule-check issue is not raised: hanger selection is a design act, not a rule. No vendor data ships with the product; the table is empty of sizes until a library is imported (M-05, C-51). A selected size becomes stale like any result when the model changes.

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
| Results · Model, Both | Result colour on tubes and fittings for the selected case or the envelope (§4.5); the legend; the probe; deformation on request; vectors neutral; unsolved elements neutral at 60 % with the legend's Unsolved swatch |
| Any stage · Table | no canvas |
| Review | no canvas; the "Report figure" block in the content column is a static rendering at the saved report-figure preset (§4.6) |

### 4.2 Selection and cursor sync

Both directions, always: clicking an element, node, glyph or vector selects its row and scrolls the table to it; selecting a row halos its entity and, when "Follow selection" is on in the HUD (on by default), pans the camera to keep the selected entity in view without changing zoom. Hover is a thinner halo and a light wash on the row. The focused table row's node is the "current row" and is always labelled. Double-click on an entity opens the inspector for it (⌘I toggles the inspector; ⎋ closes a slide-over). ⇧-click extends the selection; a drag draws a box selection with a filter (nodes, elements, restraints, loads, all) in the HUD; ⌘-click toggles.

### 4.3 The probe

P toggles the probe. While on, a card follows the cursor over the nearest element or node and reads what the results table shows for that row for the selected case: Node, Element, Case, Stress, Allowable, Ratio with its bar, Rule, Pack, and a footer with the run name and the evidence chip. A click pins the card at 18 px from the entity; a second click on another entity re-pins; ⎋ or P unpins. The probe is disabled before a run and while the run is stale it shows the stale hatch on its values and "model changed since" in the footer. It repeats nothing the results header carries: no acceptance sentence. On the Loads stage the probe reads the load rows at the node; on the Model stage it reads the layout row (node, from, type, length, section, material, load set and the read-through values).

### 4.4 The authoring gestures, and the rows they write

Two gestures, both row shortcuts through the one route; the canvas authors nothing else.

**Route by direct distance entry (R).** With a node selected, R (or the Route tool) opens the compass at that node: three axis handles X, Y, Z; the active axis solid; a length field with its unit at the handle. ⇥ or an arrow key cycles the axis; − reverses the direction; typing sets the length; B toggles a bend at the current node (the direction change inserts a bend on commit); ↩ commits; ⎋ cancels. While typing, the draft ghost (a thin dashed centreline with a faint tube outline, direction record §11 decision 12) draws from the node, the draft node's label plate shows the next number, and the layout table (drawer or side table) shows a draft row: dashed gutter bar, muted values, the typed length in the axis column in accent, and the propagated Section, Material and Load with their ticks. The inspector shows a "Routing from node 40" block on top: next node, axis, length, what the row will carry, "Bend at 40 · B", "Cancel ⎋". The hint strip under the HUD reads "Route · ↩ places 50 · ⇥ next axis · − reverses · ⎋ cancels".

On ↩ the gesture writes one batch: a new node at the computed position, the element from the current node to it (type Pipe, or the row's type if the engineer set it in the draft row), and, when B is on or the direction changed from the previous element, the bend at the current node with the section's bend radius propagated or required. The draft row becomes a normal row with the Entered origin; the compass moves to the new node so the next segment can be typed without a click. The batch is one checkpoint. Every value that would be required on a typed row is required here in the same way; nothing is invented.

**Add a restraint (S).** With the Add restraint tool, hovering a node shows the +Y glyph ghosted; a click on the node opens a new Restraints row for it with Type focused (the joined row in the Table view's layout table is not visible in Model view, so the row opens in the inspector's attachments section, or in the drawer's Restraints tab in Model view); ↩ on a valid row commits one operation. ⎋ before commit discards the row. The glyph appears on commit.

**What the gestures may not do.** No dragging of nodes, no free-hand placement, no fillet by pointer, no direct editing of a glyph. Selecting and probing are not authoring.

### 4.5 Labels, legend, deformation and ghosts

**Labels (L).** Cycles All, Budget, Off; Budget is the default (direction record §11 decision 13) with the priority order of design system §6.5. Restraint plates ("+Y · gap 3 mm · μ 0.30", "VS · H1") and load plates ("−2000 N · W") follow the label mode; the selected and hovered entities' plates always show. Labels never overlap.

**Legend (Results stage).** The quantity and unit; the scale bar with five ticks; the case name or "Envelope"; for a ratio the rule ID and pack version; the range with "set" when the engineer changed it; the Unsolved swatch; for deformation the factor, the stepper and the play control. The legend is a scale, never a verdict: nothing changes colour at 1.0; 1.0 is a labelled tick.

**Result colour.** On the Results stage the tube and fittings take the scale colour of the selected case's value per element (ratio when a rule applies, else stress; the legend says which; the HUD's "Colour by" offers Ratio, Stress, Displacement, and Off). Edge lines remain; halos draw over colour. With Envelope on, colour follows the governing case per element.

**Deformation (D).** Toggles the deformed shape drawn solid with the undeformed shape as a dashed ghost, for the selected case only, never for an envelope (the control is disabled with "Select a case" when Envelope is on). The factor defaults to the value that makes the maximum displacement one tenth of the model's extent, rounded to a round number, and the legend says "Deformed ×50 · default"; the stepper and field change it; play oscillates the factor from 0 to the stated value. Disabled before a run.

**Ghosts.** The engineer's routing draft (accent, dashed centreline, faint outline) and the agent's proposal ghost (violet, dashed, beside the current glyph or geometry, 24 px along the run so both read) are never the same colour and can coexist. A pending proposal that removes an element draws it as a dashed outline; accepting turns the ghost into the drawing; rejecting removes it.

### 4.6 Camera presets and the report figure

Fit (F; all, or the selection when one exists), Iso, Top, Front, Right, and "Report figure": the named camera saved with the project's interface state and used by the report's figure block and the report preview. "Set report figure from this view" (HUD view menu, palette) saves the current camera; the report figure block on the Review page re-renders. Section (a cut plane along an axis at a slider position) hides what lies beyond the plane for reading congested models; Isolate (I) dims everything but the selection to 20 %; Hide (H) removes the selection with "3 hidden · Show all" in the HUD. The report prints the figure as the canvas draws it at the preset, in the report's theme (light by default), with the legend and scale reference.

### 4.7 What persists

Across view and stage switches: the camera (one camera for all stages), label mode, colour-by choice, legend range, deformation state and factor, hidden and isolated sets, the section plane, Follow selection, the HUD's box-selection filter. Saved with the project's interface state: the camera, the report figure, label mode, hidden sets. Never saved: the probe pin, the routing draft, the compass.

---

## 5. The inspector, issues, run and status

### 5.1 The inspector

A docked column of 340 px in Model view and, by direction record §11 decision 1, a docked column in Both view too: the canvas shrinks while it is open and the table never reflows. In Table view there is no inspector; the row expansion carries the same fields. ⌘I toggles it; double-click on an entity opens it; ⎋ closes it when it has focus. Every edit in the inspector is the same row operation as the cell would make (§2.4); required fields carry the asterisk; every value carries its unit; provenance sits behind a disclosure.

Sections, by the kind of the primary selection:

| Selection | Sections in order |
|---|---|
| Nothing | "Select a row, a node or an element" and the counts of the model |
| A layout row (node with its arriving element) | Identity (node, element From–To, type); Geometry (DX DY DZ and X Y Z both, editable; the element length read-only); Section and material (names with the library record and a link to Libraries); Load set (name; every T and P read through; "Edit set", "Fork set"); Element fields (the type's fields of §3.2); Attachments (restraints, loads, node data as compact sub-lists with "Add…" each; a row opens the joined row); Issues on this entity (class in words, message, Show); Origin and Checked (who, when; the Check control); Provenance (the seven fields of C-46 read from the library records the row references) |
| A restraint row | Identity (node, tag); the Restraints columns as fields; a pending proposal's line in `proposal.new` when one targets the row; Issues; Origin and Checked; Provenance (hanger library) |
| A load row | node, kind, direction, value, case; Issues; Origin and Checked |
| A node-data row | node, kind, the kind's fields; Issues; Origin and Checked; Provenance (SIF source) |
| A load set | name, the pairs, used-by rows |
| A load case or combination | name, expression (display-only), stress type, rule (with the rule's display-only expression), origin; the terms as fields |
| A results row | read-only: the row's values with units, the run identity line, the evidence label, Show in table; no Checked control |
| Several rows | the count and the kinds; Checked (multi-row); the fields the rows share, editable together (one batch, one checkpoint; the field label says "5 rows") |
| During routing | the "Routing from node 40" block above the node's sections (§4.4) |

**State table — the inspector.**

| State | Shown | Enabled | Leaves on |
|---|---|---|---|
| Closed | — | ⌘I, double-click | ⌘I → Open |
| Open, docked | the sections for the selection | fields, links, Check | ⌘I or ⎋ (with focus) → Closed; view → Table → Closed (remembered as open for that view) |
| Open, routing | the routing block on top | the routing fields | commit or cancel → Open, docked |
| Open, multi-row | the shared fields | the fields, Check | selection change → Open, docked |

### 5.2 Issues

One live list. Classes, in words, with the severity glyph (design system §2.4): Invalid model (schema or topology; blocking); Blocks solve; Blocks rule check; Provenance; Assumption; Nonlinear; Content boundary; Note. Sources: the operation layer's diagnostics when an operation is checked or applied (invalid model); the readiness check (blocks solve; blocks rule check); library records (provenance; content boundary); the run record's diagnostics (nonlinear; notes). Every issue points at its entity and is a button that selects it, switching stage if the entity's table lives on another stage.

**The drawer.** A bottom drawer of 200 px under the tables (resizable, remembered), opened from the status bar count, the rail's Issues item, the toolbar count, the Run button's reason link or ⌘⇧I. Grouped by class in the order above with counts; each group is a filter chip; a class chip filters ("Nonlinear 1 ×", "All classes 2"); the row is the glyph, the class in words, the message and the entity. After a failed run the failure banner appears once on the Results page (direction record §11 decision 6) and the drawer's row for the failing class carries the same "Show node 20" link. The drawer never blocks anything; it is not a modal gate.

**Counts.** The rail's Issues badge; the toolbar's Issues button with the worst class's glyph; the status bar's count; each table's footer count of issues on that table; the gutter's state slot per row; the cell corner or ring per cell; the canvas halo on an entity with a blocking issue.

**Filters in the drawer.** Class; Stage (Model, Loads, Results); Table; Unchecked rows only; "Since last run".

### 5.3 The run

**The button.** One Run button, centred in the toolbar, the surface's primary action in every stage.

| State | Shown | Enabled | Leaves on |
|---|---|---|---|
| Enabled | "Run" | click, palette "Run" | click → Running |
| Disabled | "Run" on `disabled.fill`; tooltip names the reason: "Run is unavailable — Section missing at node 20, Material missing at node 20" (the blocking issues, up to three, then "and 2 more · Issues"); "No load cases"; "A run is in progress"; "Engine unavailable" | tooltip; the reason's Issues link | the reason clears → Enabled |
| Running | a progress bar inside the button with the current stage in its tooltip ("Solving OPE1 · case 3 of 6"); a Stop control | Stop | completes → Completed; stops or fails → Failed; Stop → Failed with "Stopped by the engineer" |
| Completed | "Run" again; the run log popover offers "Run record" | click | model change → Enabled (stale) |
| Failed | "Run" again; the rail caption "Failed"; the banner on the Results page | click | click → Running |

**Progress.** Stages in the tooltip and the log: assemble; each case by name in load-case order; hangers (the design pass); rules. The interface never synthesises a percentage; the bar fills by stages completed of stages planned.

**Failure.** The banner across the top of the Results page, once: the solver's diagnostic in one sentence, "Run 02 failed: nonlinear support at node 20 did not converge." with "Show node 20"; the issues drawer's Nonlinear row carries the same link; the results empty state (§3.6); the run record keeps the log and the diagnostics. The reason is always the engine's diagnostic, never inferred (C-58, C-59).

**The run log.** A popover on the Run button (direction record §11 decision 5): the run name, time span and settings identity; one line per stage with its time or its stop reason; "Run settings…", "Run record", "Run again". The full log lives on the run record.

**Run settings…** A sheet: solver mode (interactive, scrutiny); iteration limit for nonlinear supports; the cases to run (all by default); the operating case for hanger travel; hanger design pass on or off; the rule pack (the project's); the settings identity ("S-03") that the run record and the results header cite. Settings are not model edits and are not on the undo stack; they are saved with the project.

**The run record.** A page opened from the log, the results header disclosure, the chip popover or the palette: run name and number, time, the model hash it solved, the solver name and version, the settings identity and values, the cases and combinations, the rule pack name, version and checksum, the library references, the evidence label, the diagnostics, the full log, the immutability statement ("This record is read-only; a change to the model requires a new run"), the record's own hash, and Export (results JSON, the report package). The historical caption (M-13) when the run is a reopened saved run.

**Immutability and naming.** Runs are numbered per project ("Run 03") in order of completion; a run may be given a name ("Run 03 · with spring at 80") that lives in the project's run index, never in the record. Every run of the project is kept in the run list (results header's run menu) with its model hash; selecting an earlier run shows it under the historical band with "reopening cannot establish a current solve basis" in the band's tooltip and never as the current basis.

### 5.4 The status chip policy

The status bar's left end carries the M-08 chips, each a short label with the raw token, its authority domain, the run and time, and the reason in its popover (design system §2.3):

| Raw token | Short label | Authority domain |
|---|---|---|
| `MODEL_INCOMPLETE` | Model incomplete | Solver |
| `MECHANICS_SOLVED` | Mechanics solved | Solver |
| `RULE_INPUTS_INCOMPLETE` | Rule inputs incomplete | Rule pack |
| `USER_RULE_CHECKED` | User rules checked | Rule pack |
| `USER_RULE_FAILED` | User rule failed | Rule pack |
| `HUMAN_REVIEW_REQUIRED` | Human review required | Human |

The policy (direction record §11 decisions 2–4):

1. Before any run, one chip when the readiness check reports the model cannot be solved: Model incomplete (Solver), with the blockers in the popover. When the model is complete but unsolved, no chip: an absent status is honest and no seventh label exists.
2. After a solved run, two chips, one per authority domain: Mechanics solved (Solver) and, when a rule pack is set, the rule-pack status (Rule inputs incomplete, User rules checked or User rule failed); with no rule pack, the Solver chip alone.
3. Human review required (Human) is shown on the Review page only, beside the rule-pack chip, unless the registry requires it elsewhere (decision-packet item 4).
4. After a model change the chips drop; the rail caption "Stale" and the results header band carry the fact; the status bar shows no chip until the next run.
5. After a failed run the chips are whatever statuses the run record carries; a stopped run that carries none shows no chip, and the rail caption "Failed" with the banner carry the fact (§11 asks the engine what a stopped run emits).

The popover: the raw token in monospace, "Authority", the run and time, "Reason", and the historical caption when the run is reopened. Short labels are decision-packet item 4; the raw token is always one click away. The two evidence labels (M-09) follow the same pattern in the results header: "Internally verified" and "Prover correlated" with the raw token in the tooltip; the third label is never emitted (C-19).

### 5.5 The toolbar, status bar, palette and menus

**Toolbar band.** Left to right: the SWBPIPE wordmark; the project name with its save state ("Loop 4 header · saved", "· edited", "· not saved"); the view switch; centred, Run, the Issues count, the Agent toggle (⌘⇧G); the units toggle; the palette field (⌘K, "Search or command…"). One toolbar per surface: the tables' controls live in their footers and column menus; the canvas's in its HUD.

**Status bar.** The chips; the issues count (opens the drawer); the selection ("Node 40 · 1 row · 2 proposed rows"); the display units; the information control whose popover carries "About SWB Piping Designer…" and, until decision-packet item 1 is ruled, the registered maturity sentence as its only other line (M-01). Nothing else lives there.

**Command palette (⌘K).** Searches commands, nodes, elements, restraints, cases, runs, sections, materials, rule IDs and issues in one list; a command runs; an entity selects and navigates; the list shows the accelerator and, for a disabled command, its reason. Commands are the union of the menu items, the HUD, the table row menu and the agent panel's actions.

**Menu bar.** File (New project, Open…, Open recent, Save, Save as…, Import library…, Export…, Report preview, Close); Edit (Undo, Redo, Cut, Copy, Paste, Paste with mapping…, Select all, Find, Check rows, Clear check, Accept row, Reject row, Preferences…); View (Table, Model, Both; Model, Loads, Results, Review; Inspector, Issues, Agent; Labels, Deformation, Probe; Fit, Iso, Top, Front, Right, Report figure; Light, Dark, System); Insert (Row, Row below, Split element…, Restraint, Load, Node data, Load case, Combination, Snapshot…); Analyse (Run, Stop, Run settings…, Run record, Generate load cases…, Renumber nodes…); Window and Help (About, Scope and limitations). The menu bar, the toolbar and the palette are the three command surfaces; there is no fourth.

---

## 6. The agent collaboration protocol

### 6.1 The panel

A right column of 340 px, present in every stage, collapsible to a 44 px strip (⌘⇧G, the toolbar's Agent toggle). It reflows the surfaces and never overlays the tables or the canvas. The strip shows the agent glyph, the count of open proposals as a badge, and a dot while the agent is working; the column's header shows "Agent", the working state ("Idle", "Working…" with a stop control, "Idle · P-12 open") and the tab row: Conversation, Proposals (count), Checks, Accepted. The column has no control that acts on the model except Accept and Reject; everything the agent proposes is read in the tables and the canvas.

| State | Shown | Enabled | Leaves on |
|---|---|---|---|
| Strip | glyph, badge, working dot | open | ⌘⇧G or click → Column |
| Column · Idle | the active tab | tabs, input, Accept/Reject | ⌘⇧G → Strip |
| Column · Working | "Working…" with Stop; the conversation shows "Preparing proposal…" | Stop; the tables stay editable | proposal lands → Idle with a card; Stop → Idle |
| Unavailable | "No agent configured · Preferences" | Preferences link | configuration → Strip |

### 6.2 Conversation

Messages with author and time; the engineer's input at the bottom, ⌘↩ sends. The agent's messages may reference rows, results, report text and runs as links that select the referent (and navigate). Unknowns are listed under the message as TBD items with the triangle, and conflicts or gaps as a list with the octagon of their class; the agent never fills a value it does not have (C-78). A message may carry an "evidence summary": a short read of the run's values by reference, never a verdict. The engineer may ask for anything; the agent can answer, check, and propose; it cannot run, export, snapshot, check rows, accept its own proposals, or touch libraries, rules or preferences.

### 6.3 The proposal lifecycle

A proposal is one card in the Proposals tab and a set of banded rows in the tables (and ghosts in the canvas). Its unit of decision is the row.

**States of a proposal.**

| State | Meaning | Shown |
|---|---|---|
| Draft | the agent is composing; nothing has landed | "Preparing proposal…" in the conversation; no rows, no ghosts |
| Proposed | landed; the engine found no schema, constraint or unit error; every row pending | the card; banded rows with old and new values; ghosts; the strip badge; the tab counts ("Restraints 7 · 2 proposed") |
| Blocked | the engine rejected one or more rows | the card's rejection line lists the engine's diagnostics; the blocked rows' Accept is disabled with the reason; the other rows can be decided |
| Partly decided | some rows accepted or rejected | decided rows collapse to one line with Undo or "rejected"; "Accept remaining (n rows)" and "Reject remaining" appear |
| Closed | every row decided | the card moves to Accepted (with its outcome: accepted, partly accepted, rejected); bands and ghosts gone |
| Withdrawn | the agent withdrew it before every row was decided | "Withdrawn by the agent · reason"; undecided rows unband; decided rows keep their outcome |

**States of a proposal row.**

| State | Meaning | Shown |
|---|---|---|
| Pending | awaiting the engineer | the diff table on the card; the band in the table; Accept, Reject |
| Accepted | applied through the one route | the one-line summary with "accepted 16:31 · Undo"; the row's origin becomes Accepted with the diamond; the ghost becomes the drawing |
| Rejected | not applied | "rejected 16:33"; the band and ghost removed; nothing changed |
| Stale | the engineer changed the target row after the proposal landed | "The row changed since this proposal · the agent can revise"; Accept disabled; Reject enabled; the band stays with a hatch |
| Blocked | the engine rejected the row | the diagnostics; Accept disabled |

**Per row, multi-row and batch.** Accept row (⌘⇧A on the focused proposed row; the check button on the card's row) applies that row's operation set as one batch and one checkpoint. Multi-row: select several proposed rows in a table and ⌘⇧A, or ⇧-click several rows on the card, then "Accept selected (3 rows)": one batch, one checkpoint. Whole batch: "Accept all rows" on the card, or "Accept all (n rows)" above the queue for every open proposal, each confirming with the count: one batch per proposal, one checkpoint each. Reject mirrors each level and confirms with the count when more than one row. A row is an indivisible unit: its field changes are accepted or rejected together.

**What an acceptance records.** On the accepted operation: the proposal identity, the row, the rationale, the constraints considered and the TBD items (M-14); the engineer's name and the time; the author type stays the agent's, and the acceptance is a review decision, never an engineering acceptance (C-06). The Accepted tab lists one line per accepted unit with time, proposal, rows, the rationale disclosure and Undo while the checkpoint is still reversible; records reopened from an earlier session show "Acceptance not recorded in this session" (C-68).

**Consequence line on the card.** "Accepting a row changes the model. Run 03 stops being the solve basis and is kept as a historical run; its results stay readable as stale until the next run." (This restates the frames' consequence line in the words of §2.6.)

### 6.4 How a proposal lands in the tables and the canvas

At landing: the affected rows take the band, the bar and the diamond in the origin slot; changed cells show the new value in `proposal.new` with the struck old value before it when the column has room, otherwise in the tooltip and on the card; a new row is banded whole with "new" in its origin tooltip; a removed row is banded with every value struck; the tab strip counts proposed rows; the canvas draws the ghost beside the current geometry or glyph and gives the node's plate the ghost edge; the inspector shows the proposed line under the current values; the footer reads "7 restraints · 2 proposed rows · P-12 · draft until accepted" with the Accept row and Reject row chips. Nothing changes stage or view; the card's "Show rows" link navigates. The selection band paints over the proposal band and the bar and diamond remain.

### 6.5 What a proposal may and may not do

May: any row operation on the model tables (layout, restraints, loads, node data, load sets, load cases, combinations), including new rows, deleted rows, renumbering, and multi-table changes in one proposal; propose report text as a Draft comment on the Review page (§7.4); withdraw itself. May not: run, stop or configure a run; export; snapshot; set or clear Checked; accept or reject anything; import or change libraries or rule packs; change preferences, the camera or any interface state; write a value it has not been given (TBD instead); use the words the copy rules forbid (§9.3). A proposal that would need something it may not do says so as a TBD item ("Vendor table to use: TBD").

### 6.6 The agent's checks and open issues by reference

The Checks tab and the Review page's comment stream hold the agent's feedback on the engineer's work: cards of class "Check" or "Open issue", each with the referent as a link ("Restraints row, node 30"; "Stresses · OPE1 · node 40"; "Report §4, paragraph 2"), the text, the author and time, and a state the engineer sets (Open, Resolved; Reopen). Checks never alter a table, never set a mark, and never use the forbidden words; their output classes are the permitted ones: drafts, proposals, evidence summaries, checks, open issues (C-77). A referenced row shows a small comment glyph in its state slot on the Review page only.

### 6.7 The Checked mark

A human act: the engineer marks a row Checked (⌘⇧K, the Check control in the inspector, the row menu), per row or on a multi-row selection. It is a tag bound to the row's content, recorded with the engineer's name and the time, shown as the check in the state slot, and never a software status (its governance standing and wording are decision-packet item 7).

- **Binding.** The mark binds to a content hash of the row's own cells (its table's columns and its expansion's fields). A row whose content changes, by any route, shows the mark stale: the dashed check in amber with "Checked by R. Tufts · 14:02 · the row changed since · Check again or Clear". If the content returns to the checked state (undo, or a reverting edit), the mark is current again.
- **Who may set and clear.** The engineer only. The agent cannot set, clear or refresh it; software only reports staleness. "Check again" (⌘⇧K on a stale row) re-binds to the current content; "Clear check" removes it.
- **Multi-row.** ⌘⇧K on a selection marks every selected row; when the selection mixes checked and unchecked rows the control reads "Check 3 unchecked rows"; a second ⌘⇧K on a fully checked selection clears them.
- **Filters and counts.** "Unchecked rows" (the inverse of the mark), "Stale checks"; the footer counts "12 checked · 1 stale"; the Review page's outline shows checked counts per live table.
- **Not exported.** The mark is not a model fact: it is not written to the export, not carried in the run record, and appears in the report only if the engineer inserts the counts in review text.

### 6.8 Origins per row and cell, and the changes-since control

Every row and cell carries its origin: Entered by name at time; Accepted from proposal P-n by name at time; Propagated from node n; Generated by rule pack name and version at time; Imported from file at time. A cell's origin is the origin of the last operation that set it; a row's origin is the operation that created it; the gutter shows the row origin (quiet, and nothing at all for Entered unless Origins is on); a propagated cell shows its corner tick; Space on a slot or ⌥I lists everything. The Origins footer switch shows a dot for entered rows so that the column can be scanned.

**Changes since…** A footer control offering: last run, last Checked (this row's), a named snapshot (a Review iteration), a time. It applies a filter chip "Changed since Run 03 · 4 rows" and gives every changed cell the corner tick with "Changed since Run 03: 3 mm → 0 mm · by R. Tufts 16:31" in its tooltip. It reads the operation ledger; it never guesses.

---

## 7. The Review page

### 7.1 The three columns

A full page in Table view: the outline (280 px), the content (flexible), the comment stream (320 px). The header: "Review · Loop 4 header · Run 04"; the iteration combobox ("Iteration 2 · 09:40 · Run 04"); "compared with Iteration 1 · 2026-09-17 15:40 · Run 03" with "Compare with…"; the Show edits switch; "Snapshot…"; and, at the right, "Report preview" and "Export…" (direction record §11 decision 10). Under the header, once, in `text.secondary`, the acceptance sentence (M-02) for the Review surface class; it does not repeat in any section. The status bar on this page shows the rule-pack chip and Human review required (§5.4).

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

Comparison is absent in this design and the outline shows no entry for it. State words: empty, drafted, edited (since the compared iteration), live. Clicking an outline row scrolls the content; ⌘↑ ⌘↓ move an editable section among the editable ones.

### 7.3 Section content

**Live table blocks.** The table component in read mode with a caption "Live from Run 04 · Stresses · Envelope" or "Live from the model record · Restraints · nodes 20, 60, 80 · not editable here" and the lock glyph; the block's case selector and sort are its own and are saved with the report; a block from a stale run shows the stale band and hatch; nothing in a live block is editable here, and clicking a row selects it in the spine (and navigates on request). "Insert live table…" in the text toolbar offers any model or results table with a row filter (by node range, by selection).

**Text blocks.** The engineer's writing with a simple toolbar: paragraph, list, reference (a row, a result, a run, a rule ID, a section), insert live table, insert the report figure. References render as links. Numbers typed in text carry no automatic unit; the toolbar's "Insert value" places a referenced cell's value with its unit as a live token that updates with the run and shows the stale hatch when the run is stale.

**Edits shown or hidden.** With Show edits on, text inserted since the compared iteration is underlined in `accent.text` and removed text is struck in `proposal.old`, with the legend "inserted · removed since Iteration 1" once, on the first block that has edits; with it off the current text reads clean. Live blocks are not diffed (run comparison is excluded); a live block whose run differs from the compared iteration's says "Iteration 1 used Run 03" in its caption.

**The report figure block.** The canvas at the Report figure preset, rendered for the report's theme, with the legend and scale reference; "Set from the canvas" re-captures.

### 7.4 Iterations as named snapshots

"Snapshot…" asks for a name (prefilled "Iteration 3") and freezes: every text block, the live blocks' captions and settings with the run identity they showed, the model hash, the comment states, and the report figure. Iterations are immutable and listed in the combobox newest first with time and run; "Current" is the working state. "Compare with…" chooses the iteration Show edits diffs against (the previous iteration by default). Reopening an iteration shows it read-only with the historical band ("Iteration 1 · read-only · Run 03") and "Restore as current" which copies its text into the working state as one edit.

### 7.5 The comment stream

One stream, ordered by referent then time, with the filter row: All, Open, Resolved, Checks, Open issues, Notes, Drafts, Mine. Kinds: Check (agent), Open issue (agent), Note (engineer or agent), Draft (agent-proposed text for a section). Each card: the kind chip, the referent link, the author and time, the text, and one action: Resolve or Reopen (Note, Check, Open issue) or Insert / Discard (Draft). "+ Comment" adds a Note attached to the current selection or the current paragraph. A comment references a row in any table, a result row, a run, a report section or paragraph; comments never alter the tables; a referenced live row shows the comment glyph on this page only. Insert on a Draft places the agent's text into the section as an edit by the engineer (shown as inserted text), and the Draft becomes Resolved; the engineer may edit the inserted text freely.

### 7.6 Report preview and Export

"Report preview" opens the preview dialog: the outline rendered as it prints, with the notice and the required content block as fixed sections, the review/signoff block, the theme choice, and Print or Save (PDF, HTML). "Export…" opens the export and handoff dialog (§8.3, §8.4). The report is produced from this page and nowhere else.

### 7.7 What is and is not editable

Editable: text blocks; the order of editable sections; live blocks' case selector, sort and row filter; the review/signoff block's fields; comments and their states; iteration names. Not editable: the fixed sections' content, live values, the acceptance sentence, the notices, the run identity, an iteration once taken. The agent edits nothing on this page; it comments and drafts.

---

## 8. Libraries, rules, export and handoff

### 8.1 Libraries

The Libraries page (rail foot) lists the project's imported libraries by kind: materials, sections, components, hanger tables. Columns: Name, Kind, Records, Source, Redistribution status, Review status, Imported (time), Used by (count). Actions: Import…, Open (the record table with every field and its provenance), Delete (refused when referenced: "Used by 14 rows"), Quarantine. No library of any kind ships with the product; a new project's Libraries page is empty (C-51, M-05).

**Import…** A sheet: the file; the kind; the seven provenance fields by their record identifiers (`source_name`, `source_location`, `source_license`, `contributor`, `contributor_certification`, `redistribution_status` with its enumeration, `review_status` with its enumeration); the findings list of the six import flag classes (required fields missing; units missing or inconsistent; provenance missing; redistribution status unclear; values appear to be protected standards data; values outside user-defined reasonableness ranges) with the triangle; the quarantine action for suspected protected content, which stops the import, sets the record's `review_status` to `quarantined` and its `redistribution_status` to `protected_suspected`, records an issue and asks for review; Import and Cancel. The M-05 sentence, verbatim from the registry, once at the top. An imported library's records become available in the Section, Material and Library comboboxes; the layout row's provenance disclosure reads from them.

### 8.2 Rules

The Rules page lists rule packs: Name, Version, Checksum, Public or private, Source note, Status, and "Project rule pack" (one is set for the project; the results header, the load-case generator and the hanger design cite it). Actions: Import…, New draft, Open, Set as project rule pack, Compute checksum, Delete. The editor (Open) has three parts: declarations (required inputs with their source kinds), value slots (allowables and limits with their units and provenance: every value user-supplied, never prefilled), and checks (each with its ID, the quantity, the relation and the limit). Expressions are authored with the structured composer, never as text; the rendered expression appears beside each check in the dashed display-only frame with the caption "Display only, not accepted as input · notation not frozen" (M-15). A pack whose checksum does not match its content shows the mismatch as a warning and cannot be set as the project rule pack until recomputed. Public example packs are labelled "Invented values · not engineering data" (C-50).

### 8.3 Export to the model batch file

"Export…" (Review page header, File menu, palette) opens the export and handoff sheet. Its first target is the model batch file: what the dialog calls "Model batch file in the documented CAEPIPE MBF grammar" until decision-packet item 9 rules a name; the dialog makes no compatibility claim and carries the M-06 sentence once. Fields: target; units system for the file (SI or English, the file's one switch); what is included (layout, sections, materials, load sets, restraints, loads, node data, analysis options); the export metadata summary; Write file… and Cancel.

**The export is a projection.** Each table projects to the file's records with the correspondence RESEARCH-E §6 tabulates; the interface writes, drops or transforms as below and records every drop and transformation in the export's loss report, which the dialog previews as a list and the file's manifest carries.

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
| 13 Load cases | Dropped: the file has no section for them; the loss report lists every case with "selected in the target application". |
| 14 Combinations | Dropped likewise. |
| 15 Per-item units | Transformed to the one file-level switch, as gap 8. |
| 16 Export metadata | Written beside the file, never inside it: the manifest, the stable id map (every row to its line), the loss report, the validation report, the unit disclosure, the source model hash, the run identity when a run exists. The report's supplement (M-04) is named as what the report will carry when export metadata or a handoff package is included. |

Further projection rules: Section, Material and Load are written on every row where the value is entered and on the first row where a propagated value applies, then only on change (RESEARCH-E §7 item 4 leaves whether repeating is accepted unstated; the loss report records the choice); a second and later attachment on one node goes on a location line; tags are truncated to 14 characters with a loss entry; a start row writes absolute coordinates; a branch row writes the re-anchor line then its offsets; row order is file order. What the export is called before compatibility evidence exists is decision-packet item 9; the dialog says what it produces and nothing more.

### 8.4 Handoff

The second target of the same sheet: the handoff package (model, run record, results export, report, export metadata, provenance summary), with the M-06 sentence once and the private-data controls (excluded by default; inclusion is the engineer's explicit act, C-88). The package is written with its manifest and hashes; the run record cited is the current or the chosen historical run.

---

## 9. Disclosures and vocabulary

### 9.1 The mandatory disclosures

Every M-item of RESEARCH-C §4, with its surface, its trigger and its wording source. Wording is never paraphrased: it is the registered text or the source's text, verbatim; this document does not restate the sentences.

| ID | Surface | Trigger | Wording source |
|---|---|---|---|
| M-01 | About; until decision-packet item 1 is ruled, the status bar's information popover as its only other line | always | claims registry BS-MATURITY, one sentence: "Technical preview — not a released product."; source lines `projects/chirality-piping/docs/claims_registry.md:67`; the sentence as it exists in the product `projects/chirality-piping/apps/desktop/src/App.tsx:3282` |
| M-02 | The results header's information disclosure (results surface class); the top of the Review page (Review surface class, packet item 8); the report | a results table is shown; the Review page is shown; a report is produced | claims registry BS-ACCEPT canonical text, or a listed short variant where the registry allows; source lines `projects/chirality-piping/docs/claims_registry.md:31` |
| M-03 | The report and the report preview; the Notice section of the outline | a report is produced or previewed | the required report notice (PRD §19.3; `docs/report_notice_template.md` §"Required Notice", first block), verbatim; the product name inside it is decision-packet item 3; source lines `projects/chirality-piping/docs/report_notice_template.md:35`; `projects/chirality-piping/docs/PRD.md:1244`; emitted by the renderer `projects/chirality-piping/core/reporting/report_renderer/src/lib.rs:716` |
| M-04 | The report and its preview; named in the export dialog | design-authoring records, handoff packages, export metadata or external-prover references are included (comparison outputs are excluded from this design) | `docs/report_notice_template.md` §"Required Notice", second block, as RESEARCH-C §4 records it; source lines `projects/chirality-piping/docs/report_notice_template.md:35`; recorded at `instances/RESEARCH/C_ui_constraints.md:285` |
| M-05 | The Libraries import sheet; the export and redaction surfaces | the sheet opens | claims registry BS-IP, canonical or a listed short variant; source lines `projects/chirality-piping/docs/claims_registry.md:17` |
| M-06 | The export and handoff sheet | the sheet opens | claims registry BS-VALID, canonical or a listed short variant; source lines `projects/chirality-piping/docs/claims_registry.md:52` |
| M-07 | The report outline on the Review page and the preview | always on the Review page | PRD §19.2 (the required content list, §7.2); source lines `projects/chirality-piping/docs/PRD.md:1213`; the renderer's section order `projects/chirality-piping/core/reporting/report_renderer/src/lib.rs:457` |
| M-08 | The status bar chip with the authority domain in its popover; the Summary table | a solve or rule-check state exists | `docs/TYPES.md` §4; the short labels are decision-packet item 4; source lines `projects/chirality-piping/docs/TYPES.md:52`; `projects/chirality-piping/schemas/analysis_status.schema.yaml:107` |
| M-09 | The results header chip beside the run name; the probe footer; the report's case pages | a run is shown | claims registry §2 evidence labels; only the two emitted labels; source lines `projects/chirality-piping/docs/claims_registry.md:87` |
| M-10 | The issues drawer; the gutter's state slot; cell marks; the rail's Issues count; the canvas halo | a finding exists | the class names of SPEC §8 and PRD §14.4, spelled in words per design system §2.4; source lines `projects/chirality-piping/docs/SPEC.md:684`; `projects/chirality-piping/docs/PRD.md:872`; the class enumeration `projects/chirality-piping/schemas/model.schema.yaml:376` |
| M-11 | Every column header, inspector row, probe row, legend, report table and export preview | always | the unit catalogue's symbols; source lines `get_unit_catalog` `projects/chirality-piping/apps/desktop/src-tauri/src/lib.rs:1381` |
| M-12 | The inspector's provenance disclosure; the Libraries record view; the import sheet | a governed value is shown | the seven fields and enumerations of `docs/IP_AND_DATA_BOUNDARY.md` §4; source lines `projects/chirality-piping/docs/IP_AND_DATA_BOUNDARY.md:58`; `projects/chirality-piping/schemas/hanger.schema.yaml:64` |
| M-13 | The results header band; the rail's Historical caption; the chip popover | a saved run is reopened | the existing component's wording: "Historical saved run" and "reopening cannot establish a current solve basis"; source lines `projects/chirality-piping/apps/desktop/src/features/results/HistoricalRunContext.tsx:321` and `:9` |
| M-14 | The proposal card ("draft until accepted"), the diff, Accept and Reject, rationale, constraints considered, TBD items, the Accepted record | a proposal exists | PRD §11.8 FR-AGENT-001 to 005 as reflected in RESEARCH-C §4; source lines `projects/chirality-piping/docs/PRD.md:533` (FR-AGENT-001 at `:537`) |
| M-15 | The rule-expression cell's dashed frame and caption on the Loads page and in the Rules editor | an expression is rendered | DEC-037: "Display only, not accepted as input", with the notation declared not frozen; source lines recorded at `instances/RESEARCH/C_ui_constraints.md:296`; the rendering `projects/chirality-piping/apps/desktop/src/features/rule-packs/ExpressionComposer.tsx:403` |
| M-16 | Preferences › Appearance › Contrast findings | the panel opens | the specimen's line: "Findings only. No conformance is claimed; the target is set by the project authority."; source lines `instances/DESIGN-SYSTEM/specimen.html:1258`; recorded at `instances/RESEARCH/C_ui_constraints.md:297` |
| M-17 | About › Scope and limitations | the tab opens | `docs/DIRECTIVE.md` §6: scope, validation status, known limitations, data-boundary constraints, professional-responsibility limitations; source lines `projects/chirality-piping/docs/DIRECTIVE.md:105` |

Negative rules: the fence token never appears on a product surface; the retired phrase family that design system §7.1 lists under Anywhere never appears; no surface stacks three or more boundary terms outside a registered sentence; the retired status tokens named in `docs/TYPES.md` §4 are never emitted. One placement per surface class: the acceptance sentence appears in the three homes above and nowhere else, not in the inspector, the probe, the canvas, the agent panel or the Model and Loads pages.

### 9.2 The status vocabulary

The six automatic statuses with their authority domains (§5.4), the two evidence labels, and the historical designation are the whole status vocabulary of the interface. Rows, runs, proposals, iterations and libraries have *states* (pending, accepted, rejected, stale, open, resolved, designed, imported, quarantined), which are named in words and never rendered as chips of the status family. "Checked" is a tag. "Accepted" is the review decision on a proposal row and the review status of a library record; it is never an engineering acceptance and never a status chip.

### 9.3 The copy rules that bind every string

Design system §7 binds every string in this document and in the product; restated here with the additions this document makes.

| Where | Allowed | Never |
|---|---|---|
| Proposal controls | Accept, Reject, Accept row, Reject row, Accept selected (n rows), Accept remaining (n rows), Reject remaining, Accept all rows, Reject proposal, Accept all (n rows), Reject all | the forbidden verbs of design system §7.1; Apply; Confirm; Sign off |
| Proposal standing | Proposal, draft until accepted, rationale, constraints considered, TBD, withdrawn, stale | verified, validated, recommended by |
| The Checked mark | Check, Checked, Checked by name, stale, Check again, Clear check, Unchecked rows, Stale checks | verified, reviewed, signed |
| Statuses | the six short labels with the raw token one click away | any seventh label; Ready, OK, Pass, Fail |
| Results | number, rule ID, pack version; Ratio, Allowable, Governing case, Envelope, Unsolved, Stale | Pass, Fail, OK, Safe, Acceptable, Exceeds |
| Evidence | Internally verified, Prover correlated | the third label; Validated |
| Agent output | check, open issue, proposal, draft, evidence summary, TBD, withdrawn | accepted (of work), verified, correct |
| Runs | immutable, named, numbered, Historical saved run, current solve basis, stale, stopped, failed | Final, Released |
| Export | Model batch file in the documented CAEPIPE MBF grammar; projection; loss report | compatible with (until evidence exists) |
| Anywhere | | the words design system §7.1 lists under Anywhere, including the retired phrase family; the fence token |

Ratios are always number, rule ID and pack version together, in three columns in a table and one line elsewhere ("0.72 · EXP-A1 · pack 1.2"). Toasts state the outcome and offer Undo. Empty states name the way forward. Tooltips on icon controls carry the same words as the menu item. The product is SWB Piping Designer; SWBPIPE is the wordmark, window-title short form and file badge; the old name appears only inside a registered sentence quoted by ID, which decision-packet item 3 will replace.

---

## 10. Empty, error and edge states

### 10.1 New project

File › New project asks for a name and creates a project with an empty model, no libraries, no rule pack, no runs. The Model stage opens in Both view (first-open default): the layout table holds the start row 10 (all "—") and row 20 with DX in the editing state and the required asterisks on Section, Material and Load; the hint line under the rows; the marks column present and empty; the gutter empty; the canvas shows the ground grid, the triad and node 10 as a point. Run is disabled with "Run is unavailable — Section missing at node 20, Material missing at node 20"; the missing load set is a Blocks-rule-check issue; Issues reads 3; Results and Review are disabled with "No run yet"; the chip reads Model incomplete; the project name shows "· not saved". Libraries is empty with "Import a library to name sections and materials". The Section and Material comboboxes on row 20 offer "Import library…" as their only item until a library exists.

### 10.2 A model with no sections

Elements route and draw as centrelines; every element row's Section cell carries the required asterisk and the octagon; the issues list holds one Blocks-solve issue per element ("Section missing · node 20"); Run is disabled naming the first three; the canvas legend is absent; a paste that maps no Section column lands rows in this state. Entering a section on the first element's row propagates down the chain and clears the issues of every row that propagates.

### 10.3 A failed run

§5.3 Failed: the banner once on the Results page with the engine's sentence and "Show node 20"; the drawer's Nonlinear row; the rail caption "Failed"; the results empty state naming the stopped case, the cause and the two remedies (§3.6); the run log ending at the stop with the cases not run listed; Review disabled with "No solved run" unless an earlier solved run exists, in which case Review shows that run and the results header offers it under the historical band; the chips as the run record emits them (§5.4 item 5). The failed run is kept, numbered, in the run list.

### 10.4 A reopened historical run

Opening a project whose saved run's model hash no longer matches, or choosing an earlier run in the run list: the results tables show the run under the historical band "Historical saved run · reopening cannot establish a current solve basis" (M-13); the rail's Results caption reads "Historical"; the chip popover carries the historical caption; the probe and legend footer read "Run 01 · historical"; Run is enabled and a new run replaces the view with the current basis; the Review page's live blocks from a historical run carry the band; export of results from a historical run is allowed and the export names it historical.

### 10.5 A proposal whose row was edited before acceptance

The row becomes Stale-proposed (§6.3): the band gains the hatch; the card's row reads "The row changed since this proposal · the agent can revise"; Accept is disabled with that reason; Reject stays enabled; the conversation shows the agent a system line "Row Restraints · node 20 changed after P-12 landed"; the agent may withdraw the row or issue a revised proposal that supersedes the stale row (the old row is then shown "superseded by P-13"). The other rows of the proposal are unaffected.

### 10.6 A paste with unmapped columns

The band shows the unmapped source columns as "Sched → Ignore", "Note → Ignore" with their source names kept; the count line says "2 columns ignored"; Paste is enabled; the preview shows what will land; the toast after Paste repeats "Pasted 3 rows · 2 columns ignored · Undo". A source column that could map to a target the table has (for example "Mat") is offered the nearest target in the combobox but not pre-mapped; nothing lands from an ignored column.

### 10.7 A stale Checked mark

The dashed amber check with its tooltip; the footer count "1 stale"; the Stale checks filter; the inspector's Origin and Checked section reads "Checked by R. Tufts · 14:02 · stale · Check again ⌘⇧K · Clear check"; the Review page's outline shows "1 stale check" on the live table that holds the row. Undo of the change that made it stale makes it current again without any act.

### 10.8 A library missing

Opening a project that references a library file that cannot be found: every row that references a record from it shows the reference in `text.primary` with the provenance triangle and "Library not found · Libraries"; the issues list holds one Provenance issue per library ("Library Vendor-A springs not found at its recorded path"); Run is not blocked by the missing provenance but is blocked by any value the solve needs that the library provided (a section's dimensions), which the readiness check reports as Blocks solve; the Libraries page shows the library greyed with "Locate…" and "Remove reference" (refused while rows use it). Hanger rows whose library is missing read "Library not found" in the Library column.

### 10.9 Window at the minimum size

At 1280 × 800 the surfaces are 1180 × 728 with the strip. Table view: the tables scroll horizontally; the read-through group stays on. Model view: the canvas keeps its width and the drawer its height floor of 200 px. Both view: the split holds 55/45; the canvas has a floor of 320 px and the table pane a floor of 480 px; opening the docked inspector when both floors cannot be met collapses the agent column to the strip first, and if the floors still cannot be met the inspector opens as a slide-over over the canvas for that window size only, with the tooltip "Docked inspector needs a wider window". Below 1280 × 800 the window does not resize.

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
8. **The Checked mark.** The design needs a human tag bound to a row's content hash, with name and time, persisted with the project and never exported. Question: is there a project-level place for row tags that is not the model payload, so that the model hash does not change when a row is checked?
9. **Changes since.** The design needs "changed since the last run, the last Checked, a snapshot, a time" at row and cell grain. Question: does the ledger record the model hash and time per applied operation so the interface can answer without diffing payloads?
10. **Runs as a list.** The design needs every run of a project kept, numbered, named, immutable, with its model hash, and the ability to show an earlier run as historical. Question: can the project store hold many run records rather than one, and does the record carry the settings identity and the hanger design pass?
11. **Statuses of a stopped run.** The design shows whatever statuses the run record carries. Question: what statuses and diagnostics does a run that stops at the iteration limit emit, and does it carry the case it stopped in?
12. **Evidence labels.** The design shows the evidence label per run in the results header. Question: which component emits the two labels and where does the run record carry them?
13. **Envelope and governing case.** The design computes the governing case per row in the interface from the run's rows. Question: does the run report every case's stresses per element with the rule's allowable per row so the envelope is a selection and not a computation?
14. **Stress ratio rows.** The design needs, per element and case, stress, allowable, ratio, rule ID and pack version. Question: does the rule-check result bind each check outcome to an element and case so a row can be formed without inference?
15. **Result colour, probe and presets in the canvas.** The design needs per-element result values for colour, a probe that reads the row under the cursor, a Right preset, a saved report-figure camera and a section plane. Question: which of these are the rendering brief's overlays and which need data the run record does not carry today?
16. **Snapshots of the Review page.** The design needs named, immutable iterations of report text bound to a run and a model hash. Question: is a model state record the right carrier for an iteration, and can it hold report text and comment states beside the model hash?
17. **Comments by reference.** The design needs comments attached to rows, results, runs and report paragraphs, with open and resolved states, persisted with the project. Question: is there a project-level record for notes with typed references, and can it reference a report paragraph?
18. **Agent conversation and proposal records.** The design needs a conversation, proposals with per-row rationale, constraints considered and TBD items, per-row decisions, withdrawal, and an accepted record that survives reopen. Question: what proposal record does the agent route accept today, does it carry constraints considered and per-row status, and what is the mechanism by which a live agent binding would arrive given the standing hold?
19. **Paste with mapping.** The design needs a header-mapped multi-row paste that lands as one batch with source node IDs kept. Question: can the layer take a batch of create-node, connect, assign and set-field operations of that size as one checked unit with one undo checkpoint?
20. **Export projection choices.** The design writes section, material and load on change only, truncates tags, and writes a location line per extra attachment. Question: does the export foundation take these as options and record them in the loss report, and where does the load-reference key of gap 5 sit in its grammar?
21. **Element kinds not represented.** The design offers only the kinds the engine represents and names the rest as recorded scope decisions. Question: which of slip joint, ball joint, hinge, tie rod, elastic element and beam are gaps and which are non-goals, so About › Scope and limitations can say so?
22. **Skewed restraints and connecting nodes.** The design's Restraints table needs a restraint along a direction vector and a restraint between two pipe nodes. Question: can the support record carry a direction vector and a second node reference?
23. **Concentrated weight as node data.** The design records a weight at a node as node data that participates in every case that needs mass. Question: is a node mass representable, or only a force in the weight case?
24. **Zero displacement.** The design writes a zero imposed displacement as a value. Question: does the engine treat a zero imposed displacement as a specified value, and what should the export write given the target file ignores zero?

---

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
