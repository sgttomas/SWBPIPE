# Run-sheet entry in the grid (B1)

**Basis:** main `187b67bb6b` (2026-10-27). Design only; no product code.
Survey item B1 (`execution/_Coordination/PROPOSAL_2026-10-26_FEATURE_SURVEY.md`
§Bundle B), adopted design-first under the owner's 2026-10-27 delegation
(`loop/LOOP_INIT.md`). It serves FR-GUI-004 (`docs/PRD.md:503`, add and connect
model entities while preserving schema validation) and the model tree of PRD
§14.1. Paths are relative to `projects/chirality-piping/`; desktop paths are
relative to `apps/desktop/src/` where marked "src/". Standard claim fence
applies (F-PIP-2; claims taxonomy per DEC-081).

**What it is.** A "Run sheet" tab in the Model tree's Grid mode. Each row
creates the next end node and the element to it from three increments. The
sheet is a draft surface. It emits ordinary operation batches through the
existing vocabulary and the existing review queue. It adds no operation, no
model-document member and no numerical rule. The single-key codes below are
our own choice, mnemonic for SWBPIPE's own vocabulary; they are not taken from
any other tool.

**Risk class:** persistence (it creates model entities through operations).
Slices 1 and 4 take targeted regression checks and independent scrutiny
(AGENTS.md, Verification and authority).

## 1. Rows and columns

### 1.1 Row kinds

The Kind cell holds one code. A blank Kind is a run.

| Code | Row kind | Creates |
|---|---|---|
| blank or `R` | **Run**: a straight pipe from the current node to a new end node at the current node plus (dX, dY, dZ), or to an existing node named in the Node cell | end node (unless Node names one), pipe; support if a support code is set |
| `B` | **Bend run**: a run whose end node is a corner, realized as a bend of radius R between this row's pipe and the next row's | as Run, then the bend (§3.3) once the next run row exists |
| `S` | **Support only**: a support at the current node; no element, no increments | support |
| `F` | **Start from node**: the next rows continue from the existing or earlier-drafted node named in the Node cell (a branch) | nothing; a support if a support code is set |
| `P` | **Start at point**: a new node at the absolute X, Y, Z typed in the increment cells | node; a support if a support code is set |

"Current node" is the end node of the previous row in sheet order, or the node
an `F` or `P` row names or creates. The first row of a sheet must be `F` or `P`;
a run with no current node is refused (§4).

### 1.2 Columns

Every column maps to a member the model already has (`src/types.ts:376-548`;
`core/model_operations/operation_applier/src/lib.rs:3926-4201`,
`rich_authoring.rs:85-137`). No column adds one.

| Column | Meaning | Model member written |
|---|---|---|
| # | Row number and status (§2.5, §4) | none |
| dX, dY, dZ | Increments in the sheet's length unit; for `P`, absolute coordinates | `nodes[].position` (§6.2) |
| Kind | Row kind code (§1.1) | none |
| Support | Support code (§5.2): family of a support at the row's end node | `supports[].family` |
| Restraints | Restraint tokens of that support (`UX`…`RZ`, `lib.rs:60`) | `supports[].restraints` |
| Node | `F`: the node to start from. Run: optional existing end node ("To") | `pipe_segments[].from/to` |
| Section | A shared section id | `pipe_segments[].section_ref` (by `assign_section`) |
| OD, Wall | Inline outside diameter and wall, used when Section is blank; read-only copies of the section's values when it is set | `section.outside_diameter`, `section.wall_thickness` |
| Material | An existing material id | `pipe_segments[].material` |
| Orientation | The pipe's `y_reference`, three numbers | `pipe_segments[].y_reference` |
| Ins. thk, Ins. dens. | Insulation thickness and density | `section.insulation_thickness`, `section.insulation_density` |
| Contents dens. | Contents density | `section.contents_density` |
| Section extras (collapsible group) | Material density, mill tolerance, corrosion allowance | `section.material_density`, `section.mill_tolerance`, `section.corrosion_allowance` |
| Bend R, k, k source | Bend rows only: radius, flexibility factor and its source | the bend component (§3.3) |
| Label | The end node's label | `nodes[].label` |
| Ids | End node id and pipe id (editable before queueing); support and bend ids in the row's detail | entity ids |
| Position, Length | Read-only readouts of the resulting end point and pipe length | none |

**Operating condition.** The model has no per-element operating condition:
temperatures and pressures belong to load cases (thermal primitive loads,
pressure regions). The sheet has no operating-condition column and emits no
load operations. Survey item C1 (operating-condition sets, with its own
model-document version step) adds a "Condition set" column that inherits like
Material (§2.1). New pipes join no load case and no pressure region; the
existing missing-data findings show that.

**Quantity units.** Section quantities are stored as entered `{value, unit}`
pairs; the sheet never converts them. Each quantity column has one column
unit, chosen in its header from the unit catalog
(`src/services/unitCatalogService.ts`). It defaults to the project's unit for
that dimension where `project.units` declares one; otherwise the header reads
"Choose unit" and entry in that column is refused until one is chosen. A value
inherited from an existing pipe keeps its stored unit and shows it beside the
value when it differs from the column unit.

**Sheet header.** Three sheet-level fields, always visible:
- **Length unit** of the increments (§6.2), default `project.units.length`.
- **Source**: required non-blank text, used as the `provenance` of every
  entity the sheet creates and as the bend's `geometry_source_reference`.
- **Numbering**: id prefixes and step (§3.2).

The sheet is transient view state, like the grid's cell drafts
(`src/features/model-tree/ModelTree.tsx:87-90`). It is not saved and is
cleared when the project session changes (DEL-07-02 CLM-033).

## 2. Inheritance

### 2.1 Which cells inherit

| Inherits | Never inherits |
|---|---|
| Section, OD, Wall, Material, Orientation, Ins. thk, Ins. dens., Contents dens., the section extras, Bend R | dX, dY, dZ, Kind, Support, Restraints, Node, k, k source, Label, Ids |

- A blank increment means zero on that axis. It is shown as a muted `0`. It
  is not inheritance.
- Support, Restraints and Node never inherit, so no support or connection is
  repeated by default.
- k and its source never inherit: k is user data that the product does not
  suggest or default (`bend_authoring.rs:5-7`). The explicit **Fill down**
  command (§5.1) copies an entered value into chosen rows as entered values.
- Bend R inherits only from earlier bend rows of the same chain.

### 2.2 From which row

Each row has one **predecessor**:
- for a Run, Bend run or `S` row, the previous row in sheet order;
- for an `F` or `P` row, none: it starts a new chain (§2.3).

A blank inheriting cell takes the predecessor's **effective** value (entered or
inherited), recursively, back to the chain's start. An `S` row passes its own
predecessor's values through unchanged.

### 2.3 The chain start

An `F` or `P` row's own inheriting cells, when entered, start the chain's
values. Its blank inheriting cells resolve as follows:
1. `F` naming a node drafted earlier in this sheet: the effective values of the
   row that drafted it.
2. `F` naming a stored node: from the stored pipes that meet it. If every such
   pipe has the same stored value for a field, that value is inherited, shown
   as "from ⟨pipe id⟩". If they differ, or none meets the node, the field is
   blank and the next run row must enter it. A shared section inherits as the
   Section id when all meeting pipes have the same `section_ref`.
3. `P`: nothing; every required field must be entered.

### 2.4 Ending or continuing inheritance

- An entered value applies to its row and continues to every later row of the
  chain until another entered value.
- An entered value equal to the inherited one is still an entered value.
- Clearing an entered cell (Delete or Backspace) returns it to inheritance.
- An optional member (insulation, contents density, the section extras) is
  ended by entering `-`, which means "absent". Absence then inherits.
- Entering a Section blanks the inline OD and Wall, which show the section's
  values read-only. Entering OD or Wall on a row with an inherited Section
  clears the Section for that row and onward; the cell says so.

### 2.5 How it shows

Each cell shows its value and its origin. The origin is shown by a glyph and
text, never by colour alone, and is part of the cell's accessible description.

| Origin | Shown as | Accessible description |
|---|---|---|
| Entered | plain | "entered" |
| Inherited | muted, `↑` | "inherited from row ⟨n⟩" |
| From a stored pipe | muted, `⤷` | "inherited from pipe ⟨id⟩" |
| Derived (ids, labels, readouts) | italic | "derived: ⟨rule⟩" |
| Absent | `—` | "absent" |

An edit that changes the effective value of later rows marks those cells
"changed by row ⟨n⟩" until the user next queues or moves focus into them. The
queue step (§3.5) only emits what the cells show: no value enters a batch that
is not displayed in a cell or in the row's detail.

## 3. The row-to-operation mapping

### 3.1 The builder

A pure builder (`src/features/model-tree/runSheet/runSheetBatch.ts`) maps the
resolved rows to one `OperationBatch` (`src/services/operationBatchService.ts`).
Intents use the shapes the existing builders already emit:
- `create_node` and `connect_pipe_run` as `src/features/viewport/routeDraft.ts:416-479`;
- `assign_section` as `src/features/toolkit/SectionAssignment.tsx:51-62`;
- `create_support` as the support form's create path, with `family` and
  `restraints` only;
- `insert_bend_at_corner` as `src/features/bend-authoring/BendAuthoringForms.tsx:95-125`.

Common fields:
- `author_type` `"user"`, `operation_status` `"proposed"`.
- `source`: `source_ref` the builder's path, `source_channel`
  `"local_desktop_preview"`, `source_role` `"gui_editor"`.
- `operation_id` `op:run-sheet:⟨rowKey⟩:⟨n⟩` and `change_id`
  `change:run-sheet:⟨rowKey⟩:⟨n⟩`. `rowKey` is the row's internal key (§3.2)
  and `n` the operation's index within the row. Both stay unique within the
  batch, as the preflight requires (`atomic_batch.rs:272-282`).
- `batch_id` `batch:run-sheet-⟨sheetKey⟩-⟨seq⟩`.
- Text values are trimmed before emission, as the applier trims them.

### 3.2 Ids

Ids are entity identity, not order (FR-MOD-003).
- **Assignment.** A row's ids are assigned once, when the row first gets a
  value. The end node is `⟨nodePrefix⟩⟨n⟩` and the pipe `⟨pipePrefix⟩⟨n⟩`,
  with `n` the next number of the sheet's counter. Defaults: `node:N`,
  `pipe:P`, start 10, step 10, following the model's `node:`/`pipe:` style.
  - A support takes `support:⟨token⟩`, where `token` is the node id without
    its `type:` prefix.
  - A bend takes `node:⟨token⟩-T1`, `node:⟨token⟩-T2`, `pipe:⟨token⟩-bend` and
    `component:⟨token⟩-bend`, as `InsertBendAtCornerForm` does
    (`BendAuthoringForms.tsx:100-101`).
- **Reservation.** A number is skipped when any id it yields is reserved:
  - ids in the model;
  - create, connect and insert targets in pending editor intents and queued
    batches (the rule of `routeDraft.ts:539-575`, widened to queued batches);
  - ids held by other rows of this sheet.
- **Stability.**
  - The counter only moves forward, so ids are never renumbered.
  - Inserting a row takes the next number. A deleted draft row's numbers are
    not reused in the session.
  - The user may overwrite a draft row's id cell; the entered id is checked
    against the same reservations.
- **Labels.** A blank Label is the node id's token (derived). Pipe labels are
  the pipe id's token. Bend records use the form's labels "Bend start", "Bend
  end", "Bend arc" and "Bend".

### 3.3 Operations per row

Rows are emitted in sheet order. Coordinates are in the stored unit (§6.2).

| Row | Operations, in order |
|---|---|
| Run to a new node | `create_node`(end); `connect_pipe_run`(current → end); `assign_section` if Section is set; `create_support`(end) if a support code is set |
| Run to an existing node (Node cell) | `connect_pipe_run`(current → Node); `assign_section`; `create_support` |
| Bend run | as Run. Its `insert_bend_at_corner` is appended after the **next** run row's operations |
| `S` | `create_support`(current node) |
| `F` | `create_support`(Node) if a support code is set; otherwise nothing |
| `P` | `create_node`(point); `create_support` if set |

Payloads:
- **`create_node`**: `after` `{id, label, position: {x, y, z}, provenance}`.
  `unit` is the project's stored length unit, `dimension` `length`, `before`
  `not_present` (`lib.rs:3622-3818`).
- **`connect_pipe_run`**: `after` `{id, label, from, to, section, material,
  y_reference, provenance}` (`lib.rs:3926-4201`).
  - `section` holds OD and wall (inline, or copied verbatim from the shared
    section's `properties`), plus any insulation, contents and extras members.
  - It never carries `section_ref`, which the applier refuses
    (`lib.rs:4029-4038`).
  - `unit` is the OD column's unit, `dimension` `length`.
- **`assign_section`**: `target` (`Element`, pipe), `field_path`
  `section_ref`.
  - `before` is the canonical JSON of the pipe record as `connect_pipe_run`
    stored it.
  - `after` is `{"section_ref": id}`. It rewrites OD and wall from the section
    (`section_bindings.rs:142-184`), with values identical to those copied.
- **`create_support`**: `after` `{id, label, node, restraints, family,
  provenance}`, `unit` `none`, `dimension` `dimensionless`
  (`lib.rs:4971-5262`).
- **`insert_bend_at_corner`**: `target` (`Node`, corner), `field_path`
  `bend_at_corner`.
  - `before` is the canonical corner context (`bend_authoring.rs:475-479`;
    TS `cornerContext`, `BendAuthoringForms.tsx:33-39`).
  - `after` is `{radius: {value, unit}, flexibility_factor: {value: k, unit:
    "none"}, flexibility_source_reference, new_nodes: [T1, T2], new_pipe: arc,
    component: {id, label, provenance, geometry_source_reference}}`
    (`bend_authoring.rs:486-754`).

**Corner context inside a batch.** Each step of an atomic batch runs on the
state the earlier steps left (`atomic_batch.rs:108-162`). The builder therefore
computes each bend's `before` on a **projected model**. That is the basis model
with the builder's own records applied as the applier stores them:
- nodes and pipes appended in emission order;
- `assign_section` adds `section_ref`;
- an earlier bend removes its corner, appends T1 and T2 and the arc, and
  retargets the two straights' ends to T1 and T2 (`bend_authoring.rs:641-731`).

A wrong projection cannot change the model wrongly. The engine compares
`before` with its own context and refuses a mismatch
(`OP-BEFORE-VALUE-MISMATCH`, `bend_authoring.rs:503-505`), and the regression of
§7 S1 pins equality.

**What a bend row needs** (otherwise the row is refused, §4):
- a next row that is a run in the same chain;
- the same Section, OD, Wall and Material on both rows. The applier requires
  one section and material (`bend_authoring.rs:523-526`);
- no support code on the bend row, and no `F` row naming its corner. Nothing
  may refer to the corner or either pipe (`bend_authoring.rs:605-639`), and the
  corner must join exactly two pipes;
- entered k and k source, a positive Bend R, and a turn the engine admits.
  The turn angle and tangent length are checked by the engine and mapped to
  the row (§4.2).

After apply, the corner node no longer exists: the bend replaces it by T1 and
T2. The sheet shows the corner as "realized as bend".

### 3.4 Editing, inserting and deleting rows

**The rule: the sheet never changes the stored position of a node the user
did not edit.**
- A draft row has no stored geometry. Its increments are the entered data,
  and its position is derived from them.
- An applied row is a view of the model. Its stored positions are the data,
  and its increments are derived from them.

Why this rule: every node that moves is one the user edited, and every move
appears as its own diff row. The alternative, shifting all downstream applied
nodes, would move geometry the user did not touch. It is also ambiguous where
a downstream node starts a branch or carries a support. The shift remains
available as an explicit, previewed command (open choice O1).

**Draft rows** (nothing in the model yet):

| Edit | Effect |
|---|---|
| Edit a row | The batch is rebuilt. Later draft rows keep their increments; their derived positions and inherited values are recomputed and marked "changed by row ⟨n⟩". No model geometry moves. |
| Insert a row | It takes new ids (§3.2); the rows after it keep theirs. Positions and inheritance after it are recomputed and marked. |
| Delete a row | Its ids are released from the sheet but not reused. Later rows' positions and inheritance are recomputed and marked. An `F` row naming the deleted row's node is refused until it is changed. |

**Queued rows** are read-only. Their status names the batch. Clearing the batch
from Pending changes returns them to draft. A draft row whose predecessor is
queued waits for that batch to be applied.

**Applied rows** (slice 4):
- **Status.** A row is applied when the ids it creates exist in the model:
  for a bend row, its pipe, T1, T2, the arc and the component, with the corner
  absent. It then shows stored values. Its increments read
  `stored(end) − stored(predecessor)`, converted to the sheet unit for
  display only.
- **Edit an increment of applied row k.** The end node takes the new absolute
  position `stored(predecessor) + c(Δ′)` (§6.2). The batch holds one `set_field`
  per changed axis on `Node` `position.x|y|z`, in the project length unit, with
  the current stored value as `before` (field rule `lib.rs:408-437`). The
  editor is the grid's existing builder (`buildGridOperationIntent`,
  `src/features/workspace/table/modelTableAdapter.ts`). Downstream nodes stay
  where they are; row k+1's displayed increment changes and is marked.
- **Insert between applied rows k and k+1.**
  - The batch is `create_node`(new), `connect_pipe_run`(N_k → new),
    `reconnect_pipe_run`(pipe_{k+1}, end `from`, target new)
    (`execution/PKG-16/DEL-16-06/Design/pipe-reconnect.md` §1).
  - N_{k+1} keeps its position. Reconnect is preview-gated
    (`src/features/toolkit/BatchReviewPanel.tsx:17-19`), so the pipe's new
    chord is reviewed before apply.
- **Delete applied row k.**
  - Not the last row: `delete_support` for supports this row created,
    `delete_pipe_run`(pipe_k), `reconnect_pipe_run`(pipe_{k+1}, `from` →
    N_{k−1}), then `delete_node`(N_k).
  - The last row: `delete_support`, `delete_pipe_run`, `delete_node`.
  - Anything else referring to N_k or pipe_k (another support, a load, a
    branch) refuses, as `delete_node` and `delete_pipe_run` already do
    (`lib.rs:3903-3915`, `OP-NODE-DELETE-REFERENCED`; `pipe-reconnect.md` §7).
- **Refused in v1.** Any edit, insert or delete touching a realized bend's
  corner, tangent points or adjacent straights. Moving a straight's far node
  would break the bend's tangency, which the generic node edit does not check
  (`pipe-reconnect.md` §2). These rows say "edit through the bend tools".

### 3.5 Queueing, diff preview and undo

- **Queue rows** builds one batch from all draft rows and runs
  `validateOperationBatch` (`src/services/operationBatchService.ts`). It then
  hands the batch to the workspace's existing queue,
  `handleQueueOperationBatch` (`src/features/workspace/workspaceSession.ts:1277-1306`).
- **All or nothing.** Queue is disabled while any draft row is refused; the
  disabled reason lists the rows (open choice O5).
- **Preview.** The Pending changes review shows the batch's per-step diff rows
  (`BatchReviewPanel.tsx`). The sheet adds nothing to that path. Bend batches
  should also require preview (open choice O6).
- **Apply and undo.** Apply is the user's explicit act in the review queue. One
  applied batch pushes one whole-model undo checkpoint
  (`workspaceSession.ts:1367-1372`). Undo removes the batch's entities. Row
  status is derived from the model (§3.4), so the rows return to draft with
  their ids and values intact; Redo makes them applied again.

## 4. Validation and refusal

### 4.1 When and where

Each row is checked when it is committed (Enter, Tab or focus leaving the row)
and again when any row it depends on changes. Refusals are shown on the row
before anything is queued:
- the row header shows a refusal glyph and a short reason;
- the offending cell is marked, and the full text is the cell's accessible
  description;
- a polite live region announces "Row ⟨n⟩ refused: ⟨reason⟩".

### 4.2 Sheet rules and engine rules

The sheet's own rules run in TypeScript on the resolved rows. The engine's
rules run by validating the built batch on every committed change, debounced.
Each step's diagnostics map to their row through the `rowKey` in the
`operation_id`. The batch stops at its first blocked step (`atomic_batch.rs:150-160`),
so later rows show "not checked: an earlier row is refused".

| Condition | Rule | Refusal text (draft) |
|---|---|---|
| No current node | first row not `F`/`P`, or `F` names no node | "Start the sheet with F (from a node) or P (at a point)." |
| Zero length | all increments blank or 0 and no Node | "This run has no length." |
| Below resolution | `x_{k−1} + c(Δ)` equals `x_{k−1}` on every axis | "The increment is below the coordinate resolution at this position." |
| Both forms | increments entered and a Node named on a run | "Enter increments or an end node, not both." |
| Coincident | end position equals a stored or drafted node's position exactly | "Ends on node ⟨id⟩. Name it in Node to connect to it." |
| Near-coincident | within the model's node-proximity tolerance (`model_settings.node_proximity_tolerance`, or `DEFAULT_NODE_PROXIMITY_TOLERANCE_M` = 5 mm, `core/product_physics/src/model_settings.rs:32`) and not joined | warning only, the same meaning as `NODE_NEAR_DUPLICATE` (`core/product_physics/src/topology.rs:145-292`) |
| Missing material | effective Material blank or not in the model | "Enter a material id of this model." |
| Missing section | no Section and no OD/Wall; Section not a shared pipe section of the binding rules (`section_bindings.rs:45-85`); OD or wall not positive, or wall ≥ OD/2 | "Enter a section, or OD and wall." with the specific failure |
| Missing orientation | effective Orientation blank or zero | "Enter an orientation (y reference)." |
| Parallel orientation | `y − (y·x̂)x̂` is exactly zero for the run direction x̂ | "The orientation is parallel to this run; enter another." Near-parallel is left to the engine and the solve (open choice O4). |
| Invalid code | a pasted Kind or Support value outside §1.1 or §5.2 | "⟨text⟩ is not a ⟨kind/support⟩ code." Typed invalid keys never enter (§5.2). |
| Unit text in an increment | a cell with anything but a number | "Increments are numbers in ⟨unit⟩, the sheet's unit." |
| Quantity without unit | a quantity column with no chosen unit | "Choose the ⟨column⟩ unit." |
| Support | code set with no restraints, or invalid or duplicate tokens | "Enter the restraints of this support." |
| Bend | §3.3 requirements | specific text, e.g. "A bend needs the next run." |
| Ids | reserved, duplicate or blank | "Id ⟨id⟩ is already used." |
| Source | header Source blank | Queue disabled: "Enter the sheet's Source." |

**Mixed units.** There is one increment unit per sheet, so increments in mixed
units cannot be entered. The Length unit selector is disabled while any draft
row has entered increments, with the reason "Draft rows hold increments in
⟨unit⟩. Queue or clear them to change the unit." Changing the unit never
reinterprets an entered number.

## 5. Keyboard flow and accessibility

### 5.1 Movement

The sheet reuses the engineering table's cell model and handler conventions
(`src/features/workspace/table/EngineeringTable.tsx:368-401`,
`tableState.ts:33-49`):
- one roving tab stop;
- arrows move between cells when not editing;
- a printable key starts editing a text or number cell;
- Escape cancels an edit and is consumed;
- modified keys (Ctrl, Meta, Alt) are ignored, so shell accelerators pass
  through.

| Key | In a cell | Result |
|---|---|---|
| Enter | any | Commit. Then the **fast path**: dX → dY → dZ → Kind → Support → the next row's dX. Enter on the last row's Support appends a row. |
| Tab / Shift+Tab | any | Commit and move to the next or previous editable cell in column order, wrapping rows. Past the last cell, focus leaves to the sheet footer, as in the table. |
| Arrow keys | not editing | Move; read-only readouts are reachable here only |
| Delete / Backspace | not editing | Clear the cell, returning it to inheritance (§2.4) |
| Ctrl/⌘+D | not editing | **Fill down**: copy this cell's effective value into the selected following rows as entered values |
| Ctrl/⌘+Enter | anywhere in the sheet | Queue rows (same as the button) |
| Shift+F10, ContextMenu | row header or cell | The row menu (§5.3) |

Ctrl/⌘+D and Ctrl/⌘+Enter are unused in the shell. The shell owns ⌘1–3, ⌘I,
⌘Z and ⇧⌘Z (`workspaceSession.ts:3415-3456`; native menu accelerators
`apps/desktop/src-tauri/src/lib.rs:4496-4525`) and the palette owns ⌘K
(`src/features/toolkit/ToolkitPalette.tsx:46-52`). The sheet handles its two
keys only while focus is inside it, and stops their propagation.

### 5.2 Single-key codes

Kind codes `R B S F P` (§1.1). Support codes:

| Code | `family` (`rich_authoring.rs:329-339`) |
|---|---|
| `A` | `anchor` |
| `G` | `guide` |
| `L` | `line_stop` |
| `V` | `vertical_support` |
| blank | no support |

The spring, hanger, constant-effort and nonlinear families need records the
sheet does not hold, so they have no code. They are added afterwards in the
support form.

**Restraints are entered, not inferred.** The support path's coverage states
"No inferred restraints or stiffness"
(`execution/PKG-07/DEL-07-09/Vocabulary_Coverage.csv` row 2). The Restraints
cell takes tokens separated by spaces or commas, case-insensitive, and the
shorthand `*` for all six. Both are typed by the user. Deriving guide or
line-stop restraints from the run direction is open choice O3.

**Entering a code.** In a focused, non-editing code cell, a letter key sets the
code at once, case-insensitive, with no editor opened. A key that is not a code
changes nothing, and the live region says "⟨key⟩ is not a ⟨kind/support⟩ code.
Codes: …". The cell's accessible description lists its codes.

**Clashes.** Codes act only inside a focused code cell, and the sheet consumes
them there. The viewport's unmodified H and I act only while focus is in the
viewport (`src/features/viewport/PipeViewport.tsx:2442-2448`). A context menu,
while open, takes every key (`Design/context-menus.md`, Keyboard). No code
uses a modifier. No clash exists.

### 5.3 Menus and accessibility

- **Row menu.** Through the shared context menu (`Design/context-menus.md`):
  Insert row above, Insert row below, Delete row, Fill down, Start branch here
  (appends an `F` row naming this row's end node) and Fit row in viewport.
  Unavailable items stay focusable and give their reason.
- **Roles and names.** The sheet is a `table` region named "Run sheet". Each
  cell's accessible name is "Row ⟨n⟩ ⟨column⟩: ⟨value⟩ ⟨unit⟩". Its description
  carries the origin (§2.5) and any refusal.
- **Row header.** It names the row's kind and status ("Row 4, bend run, draft,
  refused: a bend needs the next run").
- **Announcements.** A polite live region reports added rows, refusals, queue
  results and the follow state (§5.4).

### 5.4 The viewport follows

- **Overlay.** While the sheet has draft or queued rows, the viewport draws
  them as a transient overlay in stored coordinates: a polyline from each
  row's start to its end. It uses the draft-route palette role
  (`PipeViewport.tsx:4400`) and the route-ghost precedent
  (`src/features/viewport/viewportRouting.ts:25-29`).
  - The current row is emphasized.
  - Refused rows are dashed.
  - Bend corners are marked; their tangent points are the engine's and appear
    only after apply.
  - The overlay is view state; it is not model data and never selectable as
    an entity.
- **Follow.** On by default, toggled in the sheet toolbar. When a row is
  committed:
  - if its end point lies outside the visible frame's inner 80 %, the camera
    pans to centre it at the same zoom;
  - if the draft's extent no longer fits, the camera zooms out to fit the
    draft and its start node. This is how "the viewport grows as rows are
    typed".
  - A manual camera move turns Follow off until the user turns it on.
  - In Table view the viewport is not displayed and no camera action runs.
- **Commands.** Overlay and follow travel through the viewport's existing
  view-command dispatcher (`viewportViewCommandRef`; command union
  `src/features/viewport/viewportSelection.ts:63-74`). Two transient commands
  are added: `set-run-sheet-overlay` and `follow-point`.

## 6. Boundaries

### 6.1 No model-document member

Every value the sheet writes lands in an existing member through an existing
operation (§1.2, §3.3). The sheet's rows, codes, inheritance, numbering and
follow state are transient and never written. The model document stays 0.8.0
(`execution/PKG-02/DEL-02-01/Design/model-document-0.8.0.md` §1), with no
version step. Two later features would need a member, and are not part of B1:
- an operating-condition column (C1, which brings its own version step);
- reopening a saved sheet as a sheet (open choice O8).

### 6.2 The arithmetic

The model stores node positions in `project.units.length` (U_s)
(`lib.rs:3647-3663`). Increments are typed in the sheet unit U_e. For each
axis:

```
c(v)  = v                                        if U_e and U_s are the same unit
      = ((v · f_e + o_e) − o_s) / f_s            otherwise   (length: o = 0)
x_k   = x_{k−1} + c(dx_k)                        binary64, round to nearest
x_P   = c(X)                                     for a P row
```

- `c` is `open_pipe_stress_units::convert_for_dimension`: the identity branch
  at `core/units/src/lib.rs:943`, and the transforms at `lib.rs:714-720`. The
  sheet reaches it through the engine's display-conversion facade
  (`core/model_operations/operation_applier/src/display_units.rs`, via
  `src/services/displayQuantityService.ts`). It never reimplements a factor.
- `x_{k−1}` is the predecessor's exact binary64 position: stored for an
  applied predecessor, computed for a draft one. Chained rows therefore round
  once per conversion and once per addition, as sequential absolute entry
  would.
- No value on the write path is rounded to a display precision. Display
  formatting affects readouts only.
- `create_node` is emitted with `unit` = U_s, so the applier's conversion takes
  the identity branch (`lib.rs:3781-3810` → `core/units/src/lib.rs:943`) and
  stores `x_k` unchanged.
- The JSON text of `x_k` is `JSON.stringify`'s shortest round-trip form. The
  engine parses it with serde_json's `float_roundtrip`, which reaches the
  applier through `core/product_physics/Cargo.toml:30` by feature
  unification. The stored bits therefore equal the computed bits.
- Canonical JSON (JCS) prints numbers the same way, so hashes and `before`
  texts agree.
- Default U_e = U_s, so the default sum is exactly the stored coordinates plus
  the typed numbers.

No new tolerance or default enters. The near-coincidence warning reuses the
model's existing tolerance (§4.2). Bend geometry (tangent points, admissible
turn and radius) is computed by the engine's `insert_bend_at_corner`, not by
the sheet.

### 6.3 No protected content and no engineering defaults

- The sheet suggests no section, schedule, radius class, flexibility factor,
  SIF, allowable or material.
- Sections and materials are the model's own records. k and Bend R are typed
  by the user.
- Orientation is never defaulted. The solve refuses a pipe without one
  (`PIPE_ORIENTATION_INPUT_MISSING`, `core/product_physics/src/lib.rs:8397-8405`),
  and the sheet does not change that.
- Fixtures for the slices are invented, synthetic data.

### 6.4 Vocabulary

The sheet is another surface over existing operations, as the context menus
are. DEL-07-09's vocabulary is unchanged:
- slices 1–3 use `create_node`, `connect_pipe_run`, `assign_section`,
  `create_support` and `insert_bend_at_corner`;
- slice 4 adds `set_field` on node position, `reconnect_pipe_run`,
  `delete_pipe_run`, `delete_node` and `delete_support`.

A route entry in DEL-07-09's coverage record is that deliverable's own update.

## 7. Slices

Each slice is one PR. Every slice runs its focused Vitest files and the
affected-check selection (`tools/software_workflow/select_affected_checks.py`).

**S1. Rules and builder, no UI** (persistence; independent scrutiny).
- **Code.** `runSheetModel.ts` (row types, the inheritance resolution of §2,
  id assignment of §3.2, sheet rules of §4.2) and `runSheetBatch.ts` (§3.3,
  projected-model corner contexts), under `src/features/model-tree/runSheet/`.
- **Fixture.** `fixtures/model_operations/run_sheet_authoring.json`, invented
  data: a basis model, sheets, the hand-built expected batches and the
  expected applied models. Cases:
  - a straight chain;
  - a chain with two consecutive bends;
  - a shared-section run;
  - insulation ended by `-`;
  - an `F` branch from a stored node with agreeing and disagreeing pipes;
  - a `P` start;
  - a run to an existing node;
  - a non-identity unit (sheet `in`, model `m`).
- **Tests.**
  - The builder's batch deep-equals each hand-built batch.
  - The wasm engine applies it to the expected model.
  - A Rust test (`operation_applier/tests/run_sheet_authoring.rs`) applies the
    same batches natively to the same expected models: native/wasm
    equivalence, DEL-16-06 CLM-003.
  - Every created coordinate is bit-identical (`Object.is`) to the builder's
    computed value.
  - The non-identity case equals `convert_for_dimension` plus one addition.
  - Inheritance table tests cover every row of §2.1–2.4.
  - Refusal tests cover every row of §4.2.
  - Projected corner contexts equal the engine's.
  - A mutation check: corrupting a projected context makes the engine refuse.

**S2. The sheet in Grid mode.**
- **Code.** The "Run sheet" tab in `EntityGrid` (`ModelTree.tsx:821`):
  - columns, origin display and the sheet header;
  - code cells, the keyboard flow of §5.1–5.3 and row menus;
  - per-row refusals from both rule sets;
  - Queue to `handleQueueOperationBatch`;
  - status derivation (draft, queued, applied) and undo and redo.
- **Tests.**
  - `RunSheet.test.tsx`:
    - the Enter fast path and row append, and Tab order;
    - valid and invalid code keys;
    - H, I and ⌘ keys reaching their owners;
    - Escape;
    - Fill down;
    - accessible names and descriptions, and live-region text;
    - the unit selector lock.
  - `App.runSheet.test.tsx`, through the shell:
    - typing rows, Queue, the batch in Pending changes with its diff,
      Apply;
    - Undo returning the rows to draft, Redo returning them to applied.
  - The S1 fixture batches queued from the UI equal the hand-built batches.

**S3. Viewport overlay and follow.**
- **Code.** The two view commands of §5.4, the overlay layer and the follow
  rule in `PipeViewport.tsx`, shared with DEL-07-01's viewport code paths.
- **Tests.** `PipeViewport.runSheet.test.tsx` covers the published overlay
  geometry, the dashed refused rows, follow on commit, a manual camera move
  turning Follow off, and no camera action in Table view. jsdom has no
  WebGL, so these tests check the published state and camera requests. A
  Playwright step in `e2e/` checks the drawn overlay.

**S4. Applied-row edits** (persistence; independent scrutiny).
- **Code.** §3.4 for applied rows: edit increment, insert and delete, with
  the bend-adjacent refusals.
- **Tests.**
  - Hand-built batches for each case: the builder's batch deep-equals them,
    and native and wasm apply them to the same expected models.
  - Downstream stored positions are unchanged, bit for bit.
  - Reconnect batches are preview-gated.
  - Refusals cover supports or branches at the deleted node and rows next to
    a bend.

S1 lands first. S2 depends on S1, S3 on S2, and S4 on S2.

## 8. Open choices (Agent 0)

| # | Choice | Recommendation | Consequence |
|---|---|---|---|
| O1 | Applied-row edit: keep downstream stored positions, or shift downstream nodes with the edited one | Keep (§3.4). Later, an explicit "Shift following rows" command on the selected downstream pipes through the existing `transform_pipe_run` translate, previewed. | No implicit geometry moves. A mistyped length in an applied run needs one extra explicit command. |
| O2 | Should k and k source inherit between bend rows? | No. Use explicit Fill down. | Holds the "k is user data, never suggested or defaulted" rule (`bend_authoring.rs:5-7`). One keystroke per bend run. |
| O3 | Derive guide and line-stop restraints from an axis-aligned run, or fill all six for an anchor | No in v1. Restraints are typed, with `*` for all six. | Holds the coverage record's "No inferred restraints". Allowing it would revise that record and fill shown, code-derived values; V would still need typed restraints, since the model has no vertical axis. |
| O4 | Add the kernel orientation check (as reconnect's R7) to `connect_pipe_run`, so near-parallel orientation refuses at authoring and not at solve | Yes, as its own applier PR with regression tests. | Changes refusals for existing connect batches (persistence risk class). Without it, the sheet catches only exactly parallel orientations. |
| O5 | Queue all draft rows or only the valid ones | All or nothing. | Matches the atomic batch. One refused row blocks the queue, and the reason names it. |
| O6 | Require preview for every batch containing `insert_bend_at_corner` (extend `batchRequiresPreview`) | Yes. | The bend moves the corner to tangent points the user did not type, so the review shows them before apply. This also gates the bend form's batches. |
| O7 | Operating-condition column | Defer to C1. | B1 needs no version step. Rows inherit conditions once C1's member exists. |
| O8 | Persist sheets, so a sheet reopens as a sheet | No. Rows are transient; a later "load run from model" view (with B4) can derive rows from stored geometry. | Persisting would need a new model-document member and a version step. |
| O9 | Default numbering `node:N`/`pipe:P`, start 10, step 10, counter-based | As designed. | Ids never renumber; insertion order is not encoded in ids. Labels remain free for the user's scheme. |
