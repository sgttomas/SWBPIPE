# Run-sheet entry in the grid (B1)

**Basis:** main `187b67bb6b` (2026-10-27). Design only; no product code.
Revision 5, after Codex reviews r1 (`ebd9660fa1`), r2 (`3bfc23653a`), r3
(`7cd24fb5b1`) and r4 (`28860397e2`). Codex r5 passed it at `6a26ef95b8`, and
Agent 0 decided O1–O10 and the two S2 session changes (§3.6, §8).
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
| Node id, Pipe id | End node id and pipe id (editable before queueing) | entity ids |
| Ids and sources (collapsible group) | Support id and Support label, on rows with a support code; Bend start id, Bend end id, Bend arc id, Bend component id and Bend geometry source, on `B` rows only. Each is editable before queueing; blank shows the derived value (§3.2) or, for the geometry source, the sheet's Source | `supports[].id`, `supports[].label`; the bend's `new_nodes[].id`, `new_pipe.id`, `component.id` and `component.geometry_source_reference` (§3.3) |
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
"Choose unit" and entry in that column is refused until one is chosen. Where
the catalog is unavailable (the browser preview), or offers no unit of that
dimension, the unit is typed in the header; the engine's conversion checks it
(§6.2). A value
inherited from an existing pipe keeps its stored unit and shows it beside the
value when it differs from the column unit.

**Sheet header.** Three sheet-level fields, always visible:
- **Length unit** of the increments (§6.2), default `project.units.length`.
- **Source**: required non-blank text, used as the `provenance` of every
  entity the sheet creates and as a bend's `geometry_source_reference` where
  its row's Bend geometry source is blank.
- **Numbering**: id prefixes, start and step, **Pad numbers to N digits** and
  **Pipe number from** (§3.2).

The sheet is transient view state, like the grid's cell drafts
(`src/features/model-tree/ModelTree.tsx:87-90`). It is not saved. It is held
beside the workspace session (`src/features/model-tree/runSheet/runSheetStore.ts`),
so it follows every session checkpoint whichever surface is shown. It is
cleared when the model's project changes. A history reset of the same project,
such as Open State of one of its saved states, folds the ledger instead
(§3.6), and the rows reconcile.

## 2. Inheritance

### 2.1 Which cells inherit

| Inherits | Never inherits |
|---|---|
| Section, OD, Wall, Material, Orientation, Ins. thk, Ins. dens., Contents dens., the section extras, Bend R | dX, dY, dZ, Kind, Support, Restraints, Node, k, k source, Label, Node id, Pipe id, the Ids and sources cells |

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
- **Assignment.** A row's number is assigned once, when the row first gets a
  value, from the sheet's counter. Every row takes one, `F` and `S` rows
  included; their numbers' ids stay reserved and unused. The end node is
  `⟨nodePrefix⟩⟨n⟩` and the pipe `⟨pipePrefix⟩⟨n⟩`. Defaults: `node:N`,
  `pipe:P`, start 10, step 10, following the model's `node:`/`pipe:` style.
  - **Padding.** `⟨n⟩` is the number's digits, zero-padded to **Pad numbers to
    N digits** (1 to 16) when it is set, with the sign kept. Blank is no
    padding. A wider number keeps its digits: with prefix `node:RS-` and pad
    3, 10 gives `node:RS-010` and 100 `node:RS-100`.
  - **Pipe number from** "its end node" (the default, as before): the pipe
    takes its row's own number.
  - **Pipe number from** "its start node": the pipe takes the number of the row
    whose end node is its start, when that start node is sheet-numbered (its
    id is its row's derived id) and the pipe id is not already taken, by an
    applied row's pipe or by an earlier pipe from the same node in sheet
    order. The origin reads "by its start node ⟨id⟩ (row ⟨n⟩)".
  - **Fallback.** A start node that is not sheet-numbered (a model node, an
    entered id, a row with no number), or a second pipe from one node: the
    pipe takes its row's own number, with the origin "by its end node" and the
    reason. The pipe that starts at that row's own end node has first claim
    to that id under the start-node rule. When it claims it, the falling-back
    pipe gets no id and is refused "Enter the pipe id." with the reason, and
    its id is entered. The row's own number alone would collide in the
    commonest case: a chain from a model node names its first pipe and its
    second pipe alike.
  - A support takes `support:⟨token⟩`, where `token` is the node id without
    its `type:` prefix, unless a Support id is entered.
  - A bend takes `node:⟨token⟩-T1`, `node:⟨token⟩-T2`, `pipe:⟨token⟩-bend` and
    `component:⟨token⟩-bend` from its corner's token, as
    `InsertBendAtCornerForm` does (`BendAuthoringForms.tsx:100-101`), unless
    its Bend start, end, arc or component id is entered. Its geometry source
    is the sheet's Source unless a Bend geometry source is entered.
- **Reservation.** A number is skipped when any id it yields is reserved. A
  number yields the same ids, padded as set, under either pipe option, so
  reservation covers whichever pipe takes the pipe id. Reserved are:
  - ids in the model;
  - create, connect and insert targets in pending editor intents and queued
    batches (the rule of `routeDraft.ts:539-575`, widened to queued batches);
  - ids held by other rows of this sheet.
- **Stability.**
  - The counter only moves forward, so ids are never renumbered.
  - Inserting a row takes the next number. A deleted draft row's numbers are
    not reused in the session.
  - The user may overwrite a draft row's id cells: Node id, Pipe id, Support
    id and the four bend ids. An entered id is checked against the same
    reservations and against every other id the sheet creates.
  - An applied row keeps the pipe id its batch created; it is not derived
    again, so a later row cannot rename it.
  - The Numbering fields, padding and Pipe number from included, are fixed
    once any row has a number: a change would otherwise rename the ids
    already assigned.
- **Labels.** A blank Label is the node id's token (derived). Pipe labels are
  the pipe id's token. A blank Support label is the support id's token. Bend
  records use the form's labels "Bend start", "Bend end", "Bend arc" and
  "Bend".

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
    (`bend_authoring.rs:486-754`). The T1, T2, arc and component ids and the
    geometry source are the row's, entered or derived (§3.2); the projection
    retargets the straights to those T1 and T2 ids.

**Corner context inside a batch.** Each step of an atomic batch runs on the
state the earlier steps left (`atomic_batch.rs:108-162`). A bend's `before` is
therefore the corner context of that intermediate state. Batch validation
returns no model (`atomic_batch.rs:1-16`, 166-192), so the builder cannot read
it from the engine.

**What the context contains.** The canonical corner context holds no
coordinate the engine computes (`bend_authoring.rs:475-479`). It is:
- the corner node's record, which no earlier step changes;
- the full records of the pipes that meet the corner, in model order;
- the bend markers at the corner.

An earlier bend changes a later corner's context only through ids. It
retargets the shared straight's `from` or `to` to T1 or T2, ids the sheet's
own row names (`retarget`, `bend_authoring.rs:649-657`). It writes the tangent
positions only into the new node records (`bend_authoring.rs:644-647`), and
those records are never in a later corner's context:
- a corner's pipes are the two straights;
- the earlier bend's component sits at its own T1 (`bend_authoring.rs:671,
  689`), not at the later corner.

So consecutive bends need no bend geometry in the builder.

**The projection (recommended).** The builder keeps a **record projection**:
the basis model plus the records its own earlier steps write, with ids and
verbatim payloads only. Its effects, and nothing else, are:
- `create_node` and `connect_pipe_run` append their `after` records
  normalized as the applier stores them (§3.3);
- `assign_section` adds `section_ref` and copies the section's OD and wall
  objects verbatim;
- `insert_bend_at_corner` removes the corner from `nodes` and replaces the
  corner id with the T1 or T2 id in the two straights' ends.

The projection computes no number. It does not place T1, T2 or the arc; they
are absent from it, and no later context reads them. It is used only to form
`before` texts and for the sheet's own reference checks.

A wrong projection cannot change the model. The engine compares `before` with
its own context and refuses a mismatch (`OP-BEFORE-VALUE-MISMATCH`,
`bend_authoring.rs:503-505`). The sheet validates every batch before queueing
(§4.2), and S1 pins equality for chains of two and three consecutive bends
(§7).

**Routes considered and not recommended** (open choice O10):
- *Refuse consecutive bends in v1.* This needs no projection of bends, but it
  refuses common isometric geometry (an offset, a rolled pair) for no
  numerical reason.
- *Engine-backed projection.* For example, apply each prefix of the batch to
  a scratch copy through `applyOperationBatch` and read `applied_model`. That
  returns an acceptance-shaped outcome (`atomic_batch.rs:192-198`) for a
  change no one accepted, and costs one engine apply per bend. The
  alternative, a validate call that returns its projected model, would reverse
  validate's deliberate "returns no model" (`atomic_batch.rs:1-10`) in
  DEL-16-06's engine.

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

**Applied rows** (shown from S2; edited in slice 4). Status comes from the
history ledger and the reconciliation (§3.6), not from entity existence. An
applied row shows stored values. Its increments read
`stored(end) − stored(predecessor)`, converted to the sheet unit for display
only. The chain resolves from the stored end positions of its applied rows, so
a row after an applied one continues from where the model holds that row's
end, including after an outside move that Adopt current values accepts.
- An applied bend row's corner no longer exists. Its own pipe ends at the
  stored incoming tangent point T1, which its readouts show. The chain
  continues from the stored outgoing tangent point T2, so the next row's
  increments read `stored(end) − stored(T2)`. The removed corner is never
  resolved.
- The bend's T1, T2, arc and component stay resolvable as stored records, so
  an `F` row may name either tangent node.
- Each tangent point is read on its own. If one was deleted outside the
  sheet, the row is `detached`, and what that tangent gave is shown as missing
  ("The bend's outgoing point ⟨id⟩ no longer exists"), never replaced by the
  corner.
  - Without T1, the row has no position or increment readout.
  - Without T2, Start branch here is unavailable, the following row has no
    increment readout, and a draft row after the bend is refused with that
    reason.
  - On any applied row, Start branch here is unavailable once the row's end
    node no longer exists.

**Ownership.** Each applied row records the ids its applied batches created,
in its history (§3.6). A row **owns** exactly those entities:

| Row | Owns |
|---|---|
| Run to a new node | its end node, its pipe, and its support if it created one |
| Run to an existing node | its pipe and its support. Never the node. |
| Bend run | its pipe, T1, T2, the arc and the component, and its support (none is allowed, §3.3) |
| `S` | its support |
| `F` | its support, if any. Never the node. |
| `P` | its node and its support |

A node is never deleted unless a row owns it. A node that existed before the
sheet created anything, a node an `F` row names and a node a run connects to
are never deleted by the sheet.

- **Edit an increment of applied row k** (a run to a new node, or a `P` row).
  - The end node takes the new absolute position `stored(predecessor) + c(Δ′)`
    (§6.2); for `P`, `c(X′)`.
  - The batch holds one `set_field` per changed axis on `Node`
    `position.x|y|z`, in the project length unit, with the current stored value
    as `before` (field rule `lib.rs:408-437`). The intents come from the grid's
    existing builder (`buildGridOperationIntent`,
    `src/features/workspace/table/modelTableAdapter.ts`).
  - Downstream nodes stay where they are. Row k+1's displayed increment
    changes and is marked.
  - Runs to an existing node and `F` and `S` rows have no increment to edit.
- **Insert a run between applied rows k and k+1** (k+1 a run of the same
  chain starting at N_k):
  - The batch is:
    - `create_node`(new);
    - `connect_pipe_run`(N_k → new);
    - `assign_section`(new pipe), when the inserted row's effective Section
      is set;
    - `create_support`(new), if a support code is set;
    - `reconnect_pipe_run`(pipe_{k+1}, end `from`, target new)
      (`execution/PKG-16/DEL-16-06/Work/pipe-reconnect.md` §1).
  - `assign_section` is formed as in §3.3. Its `before` is the canonical JSON
    of the new pipe's record in the record projection. Without it, the new
    pipe would carry local quantities, and later edits to the shared section
    would not reach it (`section_bindings.rs:187-233`).
  - N_{k+1} keeps its position. Reconnect is preview-gated
    (`src/features/toolkit/BatchReviewPanel.tsx:17-19`), so the pipe's new
    chord is reviewed before apply.
- **Delete an applied row.** The batch deletes only what the row owns, in
  this order: its supports, then its pipe, then any reconnect, then its node.

  | Row | Batch | Refused when |
  |---|---|---|
  | Run to a new node, a later run in its chain starts at its end node | `delete_support`(owned); `delete_pipe_run`(own pipe); `reconnect_pipe_run`(next pipe, `from` → own start node); `delete_node`(end) | anything else refers to the end node or the pipe: another pipe (a branch, or an `F` row naming it), a support it does not own, a load (`lib.rs:3903-3915`, `OP-NODE-DELETE-REFERENCED`; `pipe-reconnect.md` §7) |
  | Run to a new node, last of its chain | `delete_support`(owned); `delete_pipe_run`; `delete_node`(end) | as above |
  | Run to an existing node | `delete_support`(owned); `delete_pipe_run` | a later run of the chain starts there. Deleting this row would leave that run with no predecessor in the sheet; delete or move that run first. |
  | `S` | `delete_support` | — |
  | `F` | `delete_support`(owned), or nothing if it created none | its chain has later rows. They would lose their start; delete them first. |
  | `P`, including the sheet's first row | `delete_support`(owned); `delete_node` | its chain has later rows, or anything else refers to its node |

  - The chain boundaries are explicit. A deletion never makes a row continue
    from a different node than the stored one, and never re-parents a branch.
  - A deleted row's ids leave the row but are not reused (§3.2).
- **Refused in v1.** Any edit, insert or delete touching a realized bend's
  corner, tangent points or adjacent straights. Moving a straight's far node
  would break the bend's tangency, which the generic node edit does not check
  (`pipe-reconnect.md` §2). These rows say "edit through the bend tools".

### 3.5 Queueing, diff preview and undo

- **Queue rows** builds one batch from all draft rows and runs
  `validateOperationBatch` (`src/services/operationBatchService.ts`). It then
  hands the batch to the workspace's existing queue,
  `handleQueueOperationBatch` (`src/features/workspace/workspaceSession.ts:1277-1306`).
  - Queue reads no render: the batch and the values the ledger records are
    built from one snapshot of the sheet's committed state, so an edit
    committed by Ctrl/⌘+Enter is the one queued. If the sheet changes while
    the batch is built, nothing is queued.
  - While the batch is checked, the acting rows are queued and read-only. An
    engine refusal returns them to draft by the `refused` transition (§3.6),
    with the refusal on the failing row, and the batch is not handed over.
  - The sheet registers its batch without moving the section or focus
    (`registerOperationBatch(batch, { reveal: false })`), so focus stays on
    the same cell (§5.1). Pending changes shows the batch.
- **All or nothing.** Queue is disabled while any draft row is refused; the
  disabled reason lists the rows (open choice O5).
- **Preview.** The Pending changes review shows the batch's per-step diff rows
  (`BatchReviewPanel.tsx`). Every batch containing `insert_bend_at_corner` or
  `reconnect_pipe_run` requires a preview of that exact batch on the current
  model revision before Apply. One predicate, `operationRequiresPreview`
  (`BatchReviewPanel.tsx`), names both kinds. The batch gate uses it, and so
  does the single-intent apply path, which places such an intent in batch
  review as a one-operation batch. The bend form's intents are therefore gated
  too (O6).
- **Apply and undo.** Apply is the user's explicit act in the review queue. One
  applied batch pushes one whole-model undo checkpoint
  (`workspaceSession.ts:1367-1372`), and the checkpoint carries the batch id
  as its `operation_id`. Undo and Redo move that checkpoint between the stacks
  with the same `operation_id` (`workspaceSession.ts:1720-1765`). The sheet
  follows them through its row history (§3.6).

### 3.6 Row lifecycle

Entity existence cannot give a row's status. After an increment edit, the
row's ids exist before the edit, after it, and after its Undo. A row's status
therefore combines two things:
- a **history ledger** that mirrors the session's undo and redo stacks, which
  says which of the row's actions are in effect;
- a **reconciliation** against the model on every model revision, which says
  whether the model still holds what those actions wrote.

**Row entries.** A row's entry for a sheet checkpoint is
`{batchId, action, ownedIds, removedIds, valuesAfter, expected}`:
- `action` is one of:
  - `create`, `edit`, `insert` or `delete`, for the row the batch acts for;
  - `refresh`, for any other sheet row whose owned entities the batch
    changed (below).
- `ownedIds` are the ids the row owns after the checkpoint (§3.4).
- `removedIds` are the row's **outstanding tombstones**: every id the row once
  owned and a sheet checkpoint removed, and that is not owned again. Examples
  are a deleted row's node, pipe and supports, or a support an edit removed.
  - They carry forward: `removedIds(new) = (removedIds(previous) ∪
    removed by this checkpoint) \ ownedIds(new)`.
  - Every later entry of the row, including a `refresh`, inherits them, so a
    removal stays checked after any number of later checkpoints.
  - A tombstone ends only when the id is owned again: by **Adopt current
    values** (below), or by a sheet operation that creates the id. Ids are
    never reused (§3.2), so in practice that is Adopt.
  - **Remove from sheet** discards the row and its tombstones.
- `valuesAfter` are the row's applied entered values after the checkpoint.
  A `refresh` copies them from the row's previous entry.
- `expected` is the row's complete expectation after the checkpoint:
  - each id in `ownedIds` maps to the JSON record the batch outcome's
    `applied_model` holds for it;
  - each id in `removedIds` maps to "absent".

  It is captured at the `checkpoint` event (§3.6 events).

**Rows a batch affects.** The checkpoint record holds an entry for every sheet
row the batch affects, not only for the row it acts for. The affected rows
are found from the batch outcome's diff rows: every `entity_ref` in
`operation_outcomes[].diff_preview` (`atomic_batch.rs:142`) that is in
some row's `ownedIds` adds that row. For example:
- an insertion reconnects the next row's pipe, so that row gets a `refresh`
  entry with the new pipe record;
- a deletion with a reconnect does the same;
- a bend batch shortens the straight a later row owns, and that row gets the
  shortened record.

Undo and Redo move the whole record, so every affected row's expectation
falls back or returns with it. A change to a sheet-owned entity by **another**
producer gets no `refresh`: the row correctly reads "Changed outside the
sheet".

**The edit buffer.** Each row also keeps a `buffer`: entered values that no
in-effect entry holds. It has two layers:
- `edits`: values the user typed;
- `restored`: values an Undo put back into a draft row (below). They are
  synthetic; the user did not type them in this state.

The row shows `restored` overlaid by `edits`. Only `edits` count as "edited,
not applied".
- For a draft row that was never applied, `edits` holds all its values and
  `restored` is empty.
- For an applied row, both layers are empty until the user edits a cell.
  The edited values are then shown as "edited, not applied" over the stored
  values, which stay in the row's detail.
- Queue builds the batch from the buffer over the current entry's
  `valuesAfter`. The pending marker refers to the buffer and never replaces
  it.
- The buffer is cleared only:
  - when a checkpoint puts its values in effect;
  - by **Discard edit**, which returns the row to its current entry's values;
  - for the `restored` layer, by Redo (transitions below).
- `refused`, `invalidated`, `withdrawn` and `abandoned` leave the buffer
  intact. Requeue reads it.

**The ledger.** The sheet keeps one ledger: a list of checkpoint records
`{operationId, rowEntries}`, plus a cursor.
- Records `[0, cursor)` are in effect, in apply order. Records
  `[cursor, end)` are undone, and Redo takes them in list order from the
  cursor.
- So `ledger[cursor−1]` mirrors the top of the session's undo stack, and
  `ledger[cursor]` mirrors the top of its redo stack.
- **Every** session checkpoint has a record. A checkpoint from another surface
  gets a **foreign** record with no row entries, so the ledger stays in step
  with the session.

A row's entries in effect are its entries in `ledger[0, cursor)`, in order.
The last of them is its **current entry**.
- A row with none is `draft` and shows its buffer.
- A row with one shows its buffer over the current entry's `valuesAfter`.

**Session events.** The workspace session reports these through one transient
callback added in S2:

| Event | When | Payload |
|---|---|---|
| `queued` | `handleQueueOperationBatch` registers a batch (`workspaceSession.ts:1277-1306`) | `batch_id`, registration key, basis revision and hash |
| `validated` / `refused` | a validate or apply run returns | `batch_id`, outcome diagnostics |
| `checkpoint` | any apply pushes an undo checkpoint (`workspaceSession.ts:1367`, 1506, 1569, 1664), before the model is committed (a change at the batch site, see below) | `operation_id`, and for a batch the outcome's `applied_model` and `operation_outcomes` |
| `undone` / `redone` | `handleUndoSessionModelEdit` / `handleRedoSessionModelEdit` (`workspaceSession.ts:1720-1765`) | the checkpoint's `operation_id` |
| `history-reset` | the session empties its stacks (`setUndoStack([])`: `workspaceSession.ts:1878`, 2190, 2277, 2405, 3475) | none |
| `invalidated` | a registration is retired because the model changed (below) | `batch_id`, key, reason |
| `withdrawn` | Pending changes cleared, or the project replaced (`handleClearReviewQueue`, `withdrawBatchRegistrations`, `workspaceSession.ts:1245-1275`) | `batch_id`, key |
| `abandoned` | a run of a registered batch ends with neither an outcome nor a retirement | `batch_id`, key, reason |

**Transitions.**

| Event | Ledger | Rows |
|---|---|---|
| Queue (the sheet) | — | Each acting row gets a **pending** marker `{batchId, action}`, which is not in the ledger. Its values stay in the buffer. Status `queued`. |
| `refused`, `invalidated`, `withdrawn`, `abandoned` for a pending batch | — | The marker is dropped; the buffer is kept. The status is re-derived (reconciliation below). For `invalidated` and `abandoned`, the row is marked "Requeue needed". |
| `checkpoint` with a pending batch's id | Dispose of records `[cursor, end)`, since the session cleared its redo stack. Append a record with an entry for every affected row (acting rows and `refresh` rows), each with its complete `expected` from `applied_model`. Cursor + 1. | The acting rows' buffers move into their entries' `valuesAfter` and are cleared. |
| `checkpoint` with any other id | Dispose of `[cursor, end)`; append a foreign record; cursor + 1 | — |
| `undone` with `ledger[cursor−1].operationId` | Cursor − 1 | Each of the record's rows falls back to its previous entry in effect. An acting row with none left is `draft`. Its `restored` layer becomes its create entry's `valuesAfter`, and its `edits` are kept. |
| `redone` with `ledger[cursor].operationId` | Cursor + 1 | The record's entries are in effect again. Each row's `restored` layer is cleared. Each `edits` value equal to the in-effect `valuesAfter` is dropped, and only real differences stay "edited, not applied". So create A, Undo, Redo reads `applied` with no edit marker. |
| `undone` or `redone` with any other id, or `history-reset` | Fold the in-effect records into a permanent base. Discard the undone ones. Cursor = base length. | Each row keeps its current entry, which can no longer be undone from the sheet. |

- **Disposal.** An entry in a disposed record is gone. A row whose entries
  are all disposed is already `draft`, since disposal follows an Undo. It
  keeps its ids and buffer. A row that still has an earlier entry in effect
  keeps it.
- **Worked example.** Apply create A, then edit B, then edit C; the cursor
  is 3.
  - Undo matches C at `ledger[2]`: cursor 2, and the row shows B's values.
  - Undo matches B at `ledger[1]`: cursor 1, and the row shows A's values.
  - Redo matches `ledger[1]` = B: cursor 2.
  - Instead, after the second Undo, a new edit D disposes B and C, appends
    D, and sets the cursor to 2. The row's entries are A, D.
- **Cap.** The session keeps 25 checkpoints (`slice(0, 25)`). A checkpoint
  dropped from the bottom can no longer be undone, so its record never moves
  again. It needs no event.

**Reconciliation.** The sheet re-derives the status of every row with a
current entry on every model revision. This covers revisions from the sheet,
other surfaces, Undo, Redo and opened states.
- Look up every key of the current entry's `expected`, owned and removed
  ids alike, in the current model.
  - If every owned id's record equals its expected record, and every removed
    id is absent, the status is `applied`, or `deleted` for a deletion.
  - Otherwise the status is `detached`. The row is read-only, shows stored
    values, and lists the differing ids under "Changed outside the sheet".
- A deleted row's removed ids are checked like any others, so another
  surface restoring one of them detaches the row ("Restored outside the
  sheet"). The check is never vacuous.
- A draft row whose reserved id now exists in the model gets the refusal
  "Id ⟨id⟩ now exists in the model". It stays `draft`, and Queue is blocked.
- A `detached` row offers:
  - **Adopt current values**, only when every owned id exists: replace
    `expected` with the current records, with no operation. Tombstoned ids
    that exist again become owned again and leave `removedIds`. Tombstones
    still absent stay. For a deleted row, this applies once all its removed
    ids exist again;
  - **Remove from sheet**, with no operation.

**Cost.**
- **Indexing is already paid per revision.** The session builds one index per
  model revision: `activeModelIndex`, memoized on model, project session
  generation and UI model revision (`modelSessionState.ts:36-39`). The tree
  and viewport already consume it. `modelIndexFor` caches it per model object
  and generation key (`modelIndex.ts:66-77`). A cold call with any other key
  rebuilds it by scanning the model (`buildModelIndex`, `modelIndex.ts:79`).
- The sheet therefore takes `activeModelIndex` as a prop, as `ModelTree` takes
  `modelIndex`. It never calls `modelIndexFor` with a generation key of its
  own, so it adds no index build.
- Reconciliation looks up each expected id in that index's `entities` map
  and never scans a collection.
- Each revision therefore costs one map lookup and one JSON-value comparison
  per expected id, over the rows with a current entry. It is independent of
  the model's size. Rows without a current entry cost one reservation check
  per id.
- Measurement (S2): a 500-row sheet (about 1,500 expected ids, including 50
  bend rows) on a model of at least 5,000 entities, over 100 revisions. S2
  reports:
  - rows, expected ids and model entities;
  - median and 95th-percentile reconciliation time, with the host;
  - the lookup count, asserted equal to the number of expected ids;
  - **separately**, median and 95th-percentile cold `buildModelIndex` time
    for the same model, as context, since every revision already pays it;
  - the number of `buildModelIndex` calls per revision with the sheet
    mounted, asserted equal to the number without it.

  A median above one 60 Hz frame (16.7 ms) is reported as a finding for
  Agent 0, not passed silently.
- S2 measured it (`runSheet/runSheetReconcile.measure.test.tsx`; indicative,
  an ordinary test run on a shared host). On 500 rows with 50 bend rows,
  1,600 expected ids and 6,703 model entities, over 100 revisions, the
  reconciliation took a median of 2.7 ms (95th percentile 4.4 ms), with one
  lookup per expected id. The cold `buildModelIndex` of the same model took a
  median of 7.7 ms (95th percentile 10.9 ms). The session built one index per
  revision with the sheet mounted and one without it.

**Why reconciliation is exact.**
- Undo and Redo commit a clone of the whole checkpoint model
  (`commitModel(clonePreviewModel(checkpoint.model))`,
  `workspaceSession.ts:1735`, 1758). Returning to the state some apply
  produced therefore restores each owned record as the same JSON value that
  was captured from that apply's `applied_model`.
- The comparison is equality of JSON values: deep, independent of key order,
  numbers by value. This is the same relation as equality of their RFC 8785
  canonical texts. JCS is a function of the JSON value, and it prints +0 and
  −0 alike, as the value comparison treats them.
- So a row returns to `applied` exactly when its records are restored. That
  holds whichever surface or checkpoint caused the change: for example,
  Undo of another surface's deletion of a sheet-owned support. It needs no
  tolerance and no matching of checkpoint ids. Any other difference keeps the
  row `detached`.

**Retiring stale registrations** (a session change, in S2's scope).

**Adopted by Agent 0.** The two cross-producer session changes below are
adopted (owner's 2026-10-27 delegation): retiring stale queued batches from
every producer (step 1), and Prepare/Publish/Notify publication (step 2).
- Today a queued batch whose basis revision is no longer current shows
  disabled Validate and Apply buttons (`BatchReviewPanel.tsx:68-77`).
  `handleRunOperationBatch` returns silently on a revision mismatch
  (`workspaceSession.ts:1318`). So the hash refusal at
  `workspaceSession.ts:1345` is never reached, and the queued rows would
  never be released. S2 changes the session so that:
  1. In the layout effect that advances `modelRevision`
     (`workspaceSession.ts:668-672`), after the increment, every batch
     registration whose `basisRevision` differs from the new revision is
     revoked (removed from `batchRegistrations`). Its Pending changes entry
     becomes a retired notice: "Prepared on an earlier model and withdrawn.
     Prepare it again." Its only action is Dismiss. `invalidated` is emitted
     once per registration. This applies to every producer's batches, since
     none of them could run anyway. BatchReviewPanel's tests for the stale
     state change with it.
  2. **Transactional publication: a change to the apply paths.**
     - **Today.**
       - `handleRunOperationBatch` pushes the undo checkpoint
         (`workspaceSession.ts:1367-1372`), then calls `commitModel`, then
         deletes the registration and filters the queue (1392-1395).
       - `commitModel` itself runs external callbacks, such as the
         viewport's `retire-box-gesture` (`workspaceSession.ts:720`) and
         report-package invalidation, interleaved with ref writes and state
         setters (716-762).
       - So a throw part-way leaves a partial state. For example, an undo
         checkpoint is pushed with no model change, and the catch (1400-1401)
         restores nothing.
       - A separate standalone PR fixes this gap now, ahead of the B1 slices:
         history is queued before a `commitModel` that can fail
         (`workspaceSession.ts:1367-1372`). S2 generalizes that fix to every
         publish path.
     - **The change.** S2 splits every model-publishing path into three
       phases. The paths are batch apply, single-intent apply, draft-review
       apply, Undo and Redo.
       1. **Prepare.** Fallible and side-effect free. It computes the next
          model's commit plan: identity hash, revision numbers and index
          generation. It computes the checkpoint object, and the sheet's
          staged transition, a pure function of the event and the sheet
          state that writes nothing. A throw here changes nothing:
          - the model, stacks, registration, queue entry, ledger, cursor,
            buffers and row statuses are all as before;
          - the batch stays queued and can be applied again;
          - the message is "Apply failed before publication; nothing
            changed."
       2. **Publish.** One synchronous block with no `await` and no external
          callback: only ref assignments and React state setters, which do
          not throw for prepared values. It publishes the model and
          revision refs, the undo push and redo clear, the registration
          delete and queue filter, and the sheet's staged transition (one
          setter). React applies them in one batch, so no render or layout
          effect sees some without the others. The revision step then
          never sees the applied batch as stale.
       3. **Notify.** After publication, the external callbacks run
          (`retire-box-gesture`, selection pruning). Each runs in its own
          `try`. A failure is reported as an error and does not unpublish.
          These are view notifications, not state of record. Report-package
          invalidation and the solve gates become state writes in Publish,
          so no state of record depends on a callback.
     - **Implementation constraints** (Codex r5):
       - **What Publish covers:**
         - model, assignment and revision state;
         - the commit, request and publication refs;
         - persistence and hash ownership;
         - the operation-busy state and draft registrations;
         - history, receipts and review context;
         - batch registration and the queue;
         - solve and report invalidation;
         - the staged sheet state.
       - **The no-throw claim is narrow.** It holds only for setters given
         prepared values, and for plain refs the session owns. Existing
         updater bodies that clone, hash, parse or do other fallible work
         move to Prepare. Updaters in Publish only place prepared values.
       - **Caller-supplied outcomes.** `ownedOutcome.current`
         (`handleApplyIntent`'s optional argument) may be an accessor
         supplied by its caller. Assigning it is outside Publish: it runs in
         Notify, with its external code caught there.
       - **`commitModel` is split.** The existing function cannot stay
         intact inside Publish. Its fallible work and external callbacks go
         to Prepare and Notify, and only its prepared writes stay in Publish.
       - **Batching.** The desktop uses React 19 (`apps/desktop/package.json`)
         with `createRoot` (`src/main.tsx:12`), which
         batches state updates automatically, including in async
         continuations after an `await`. So Publish's setters land in one
         render without an explicit batching wrapper, provided Publish
         itself contains no `await`.
  3. In `handleRunOperationBatch`, every return after the registration lookup
     that produces no outcome, and is not covered by step 1, emits
     `abandoned` with its reason. These are the returns where
     `stillCurrent()` fails. The plain "busy" return (`workspaceSession.ts:1318`)
     is not an abandonment: the batch stays queued and its buttons stay
     disabled until the request ends. Today every model change that can occur
     while a batch runs withdraws or retires its registration first, so
     `abandoned` covers a revision that moves before the retirement sees it.
  4. `handleClearReviewQueue` and `withdrawBatchRegistrations` emit
     `withdrawn` for each registration they revoke.
- **Requeue** is an ordinary Queue on the current model:
  - it rebuilds the batch from the rows' entered values and the current
    record projection;
  - it uses a new `batch_id`, the current basis hash and `before` texts read
    from the current model;
  - it re-checks ids against the current reservations (the retired
    registration no longer reserves anything) and reruns the sheet rules and
    validation.

  The retired batch's content is never resubmitted. If an applied-row edit's
  target changed meanwhile, the requeued `set_field` takes the current stored
  value as `before` and the user's entered value as `after`, and the diff
  shows it.

**Other rules.**
- `deleted` rows are hidden, with a count and an "Undo restores" note. Queue
  never emits operations for `deleted` or `detached` rows. A deleted row can
  return only through Undo of its deletion.

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
| Ids | reserved, duplicate or blank | "Id ⟨id⟩ is already used." for a node or pipe id; "⟨cell⟩ ⟨id⟩ is already used." in the Support id and bend id cells; "Enter the ⟨node/pipe⟩ id." for a blank id, including a start-node pipe whose fallback number is taken (§3.2), with the reason |
| Not applicable | Support id or Support label on a row with no support code; a bend id or Bend geometry source on a row that is not `B` | "⟨cell⟩ applies only to a row with a support code; clear it." or "⟨cell⟩ applies only to B rows; clear it." |
| Source | header Source blank | Queue disabled: "Enter the sheet's Source." |
| Numbering | start or step not whole, step below 1, or padding not a whole number from 1 to 16 | Queue disabled: "Numbering needs a whole start and a whole step of at least 1, within the exact-integer range." or "Pad numbers to a whole number of digits from 1 to 16, or leave it blank." |

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
- modified keys (Ctrl, Meta, Alt) are ignored, except the two chords below,
  so shell accelerators pass through;
- no key acts during IME composition (`isComposing`), so Enter that confirms a
  composition never commits or moves (`EngineeringTable.tsx:369`).

| Key | In a cell | Result |
|---|---|---|
| Enter | dX, dY, dZ, Kind, Support | Commit. Then the **fast path**: dX → dY → dZ → Kind → Support → the next row's dX. Enter on the last row's Support appends a row. |
| Enter | any other editable cell (Node, Material, Section, Restraints, k, k source, Label and the rest) | Commit and move to the same column in the next row, as the table's Enter does (`tableState.ts:43`). On the last row it commits and stays; it does not append. |
| Tab / Shift+Tab | any | Commit and move to the next or previous editable cell in column order, wrapping rows. Past the last cell, focus leaves to the sheet footer, as in the table. |
| Arrow keys | not editing | Move; read-only readouts are reachable here only |
| Delete / Backspace | not editing | Clear the cell, returning it to inheritance (§2.4) |
| Ctrl/⌘+D | not editing | **Fill down**: copy this cell's effective value into the selected following rows as entered values. Shift+ArrowDown and Shift+ArrowUp extend the row selection from the focused row; with no selection, the next row is filled |
| Ctrl/⌘+Enter | anywhere in the sheet | Queue rows (same as the button) |
| Shift+F10, ContextMenu | row header or cell | The row menu (§5.3) |

Ctrl/⌘+D and Ctrl/⌘+Enter are unused in the shell. The shell owns ⌘1–3, ⌘I,
⌘Z and ⇧⌘Z (`workspaceSession.ts:3415-3456`; native menu accelerators
`apps/desktop/src-tauri/src/lib.rs:4496-4525`) and the palette owns ⌘K
(`src/features/toolkit/ToolkitPalette.tsx:46-52`). The sheet handles its two
keys only while focus is inside it, and stops their propagation.

**Focus recovery.** Tested in S2:
- after Queue, focus stays on the same cell, now read-only;
- after Delete row, focus goes to the same column of the next shown row, else
  the previous row, else the sheet footer;
- after Insert row, focus goes to the new row's dX;
- after Undo or Redo, focus stays on the same row's cell when the row is still
  shown, else follows the Delete rule;
- after a row menu command, the context-menu recovery order applies
  (`Work/context-menus.md`, Keyboard).

Native assistive-technology verification (VoiceOver on macOS) remains open.

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
while open, takes every key (`Work/context-menus.md`, Keyboard). No code
uses a modifier. No clash exists.

### 5.3 Menus and accessibility

- **Row menu.** Through the shared context menu (`Work/context-menus.md`):
  Insert row above, Insert row below, Delete row, Fill down, Start branch here
  (appends an `F` row naming this row's end node) and Fit row in viewport.
  On an applied bend row, Start branch here names its outgoing tangent node
  T2. On a bend row that is not applied it is unavailable, because the corner
  is replaced when the bend applies.
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
(`execution/PKG-02/DEL-02-01/Work/model-document-0.8.0.md` §1), with no
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
  applied predecessor, computed for a draft one. Each row rounds once in the
  conversion of its own increment and once in the addition.
- **Conversion before addition is the rule.** For a non-identity unit, this is
  not the same number as converting an absolute coordinate typed in U_e. With
  a predecessor at 0.1 m and an increment of 0.3 in, `0.1 + c(0.3)` is
  `0.10762000000000001` m. Converting the predecessor to inches, adding 0.3
  and converting back gives `0.10762000000000002` m. The sheet uses the first,
  and S1 pins this case.
- No value on the write path is rounded to a display precision. Display
  formatting affects readouts only.
- `create_node` is emitted with `unit` = U_s, so the applier's conversion takes
  the identity branch (`lib.rs:3781-3810` → `core/units/src/lib.rs:943`) and
  stores `x_k` unchanged.
- The JSON text of `x_k` is `JSON.stringify`'s shortest round-trip form. The
  engine parses it with serde_json's `float_roundtrip`, which reaches the
  applier through `core/product_physics/Cargo.toml:30` by feature
  unification. The stored bits therefore equal the computed bits, with one
  exception: negative zero.
- **Signed zero.** `JSON.stringify(-0)` is `"0"`, so −0 cannot cross the wire.
  It arises only from a typed `-0`, or from a sum of two negative zeros. The
  builder normalizes every computed coordinate with `x === 0 ? 0 : x` before
  using it as a position. That normalized value is the one that is displayed,
  projected, emitted and stored. The bit-identity claim holds for the
  normalized value, and S1 tests a `-0` entry.
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
Fixtures are invented data.

**S1. Rules, builder and record projection, no UI** (persistence; independent
scrutiny).
- **Code.** Under `src/features/model-tree/runSheet/`:
  - `runSheetModel.ts`: row types, the inheritance resolution of §2, id
    assignment of §3.2 and the sheet rules of §4.2;
  - `runSheetBatch.ts`: §3.3, the record projection of §3.3 and −0
    normalization (§6.2). It takes the resolver's output as its only input.
- **Fixture.** `fixtures/model_operations/run_sheet_authoring.json`: a basis
  model, sheets, hand-built expected batches and expected applied models.
  Cases:
  - a straight chain;
  - chains of two and of three consecutive bends;
  - a shared-section run;
  - insulation ended by `-`;
  - an `F` branch from a stored node, with agreeing and disagreeing pipes;
  - a `P` start;
  - a run to an existing node;
  - a non-identity unit (sheet `in`, model `m`), including the 0.1 m + 0.3 in
    case of §6.2;
  - a `-0` entry;
  - a batch whose last step is refused (a bend radius too large at the
    final corner).
- **Tests.**
  - The builder's batch deep-equals each hand-built batch.
  - The wasm engine applies each batch to its expected model.
  - A Rust test (`operation_applier/tests/run_sheet_authoring.rs`) applies the
    same batches natively to the same models: native/wasm equivalence,
    DEL-16-06 CLM-003.
  - Every created coordinate is bit-identical (`Object.is`) to the builder's
    normalized value. The non-identity case equals `convert_for_dimension`
    plus one addition, and differs from the absolute-entry value as stated.
  - **Displayed equals emitted.** For every fixture row, each value the
    resolver reports for display equals the corresponding field of the
    emitted payloads; no emitted field lacks a displayed source.
  - Projected corner contexts equal the engine's for the two- and three-bend
    chains. A corrupted projection makes the engine refuse with
    `OP-BEFORE-VALUE-MISMATCH`.
  - **Late-step rollback.** The batch with a refused last step leaves the model
    unchanged (`rolled_back_no_model_published`). The refusal maps to that
    row.
  - **Existing-node ownership.** A run to an existing node and an `F` row emit
    no `create_node` and record no owned node.
  - Inheritance tests cover every row of §2.1–2.4; refusal tests cover every
    row of §4.2.

**S2. Preview gate, then the sheet in Grid mode.**
- **Prerequisite, first in the PR: the preview gate** (§3.5, O6).
  - One predicate, `operationRequiresPreview(change_kind)`, is true for
    `reconnect_pipe_run` and `insert_bend_at_corner`.
  - `batchRequiresPreview` (`BatchReviewPanel.tsx:17-19`) uses it, and so does
    the single-intent apply path (`handleApplyIntent` in
    `workspaceSession.ts`), which the bend form's intents take.
  - `PREVIEW_REQUIRED_REASON` (`BatchReviewPanel.tsx:34`) names the operation
    kind instead of "a reconnect".
  - **Behaviour change, accepted by Agent 0 (O6).** The existing bend form
    (`features/bend-authoring/`) now needs a preview before Apply. The S2
    PR states this in its body and in the deliverable text that describes
    the bend form. That text belongs to DEL-07-03 and DEL-03-03, which list
    the form among their code paths.
  - Tests:
    - a sheet batch with a bend, a bend-form intent, and a requeued
      single-intent bend batch (`App.tsx:499-500`);
    - each, applied directly with no preview, is refused with the reason and
      leaves the model unchanged;
    - each applies after a clean preview of that exact content;
    - an edited batch needs a new preview.
- **Code.** The "Run sheet" tab in `EntityGrid` (`ModelTree.tsx:821`):
  - columns, origin display and the sheet header;
  - code cells, the keyboard flow of §5.1–5.3 and row menus;
  - per-row refusals from both rule sets;
  - Queue to `handleQueueOperationBatch`;
  - the session changes of §3.6: the event callback, the retirement of stale
    registrations, `abandoned` and `withdrawn`;
  - the ledger, the reconciliation, and Requeue (§3.6), for create batches.
- **Tests.**
  - `RunSheet.test.tsx`:
    - the Enter fast path and row append;
    - Enter in Material, Restraints and k moving down the column, and staying
      on the last row;
    - Tab order;
    - Enter during IME composition and with modifiers doing nothing;
    - valid and invalid code keys;
    - H, I and ⌘ keys reaching their owners;
    - Escape and Fill down;
    - the focus-recovery cases of §5.1;
    - accessible names and descriptions, and live-region text;
    - the unit selector lock;
    - deleting a draft `P`, `F` or `S` row and the recomputation it causes.
  - `App.runSheet.test.tsx`, through the shell:
    - typing rows, Queue, the batch in Pending changes with its diff, Apply;
    - Undo returning the rows to `draft`, and Redo returning them to
      `applied`;
    - **multiple Undo and Redo**: two sheet batches with another surface's
      checkpoint between them, undone three times and redone three times.
      The ledger cursor and every row's status follow at each step;
    - **restoration**: a sheet-owned support deleted in the inspector makes
      its row `detached`. Undo of that deletion returns the row to `applied`.
      Adopt current values and Remove from sheet also work;
    - **history reset**: opening a saved state folds the ledger, and the rows
      reconcile;
    - **Redo after Undo of a create**: create A, Undo, Redo. The row reads
      `applied` with no "edited, not applied" marker. A value typed while it
      was draft survives Redo as an edit only if it differs;
    - **injected failures** (test doubles that throw):
      - in Prepare, via the sheet's staged transition, the identity hash or
        the checkpoint clone: the model hash, both stacks, the
        registration, the queue entry, the ledger cursor, every buffer and
        every row status are unchanged, and a second Apply succeeds;
      - in Notify, via the viewport's `retire-box-gesture`: everything is
        published once, the error is reported once, and nothing is rolled
        back;
      - the same for an Undo and a Redo;
    - **apply order and retirement**:
      - the sheet's own apply emits `checkpoint` and no `invalidated` for
        its batch;
      - a geometry-tools batch queued before it on the same basis is retired
        with `invalidated` and shown as a retired notice;
      - Clear pending changes emits `withdrawn`, and the sheet's rows return
        to `draft` with their buffers;
    - the reconciliation cost measurement above, reported in the PR;
    - **stale queue basis and Requeue**:
      - a model edit on another surface after Queue retires the registration
        and emits `invalidated`;
      - the rows return to `draft`, marked "Requeue needed";
      - Requeue queues a new batch on the current model and applies;
      - a running batch abandoned by a revision change releases its rows
        too;
    - **late-step rollback** in the UI: rows return to `draft` with the
      refusal on the failing row;
    - withdrawal by clearing Pending changes.
  - The S1 fixture batches queued from the UI equal the hand-built batches.

**S3. Viewport overlay and follow.**
- **Code.** The two view commands of §5.4, the overlay layer and the follow
  rule in `PipeViewport.tsx`, shared with DEL-07-01's viewport code paths.
- **Tests.** `PipeViewport.runSheet.test.tsx` covers:
  - the published overlay geometry and the dashed refused rows;
  - follow on commit;
  - a manual camera move turning Follow off;
  - no camera action in Table view.

  jsdom has no WebGL, so these tests check the published state and camera
  requests. A Playwright step in `e2e/` checks the drawn overlay.

**S4. Applied-row edits** (persistence; independent scrutiny).
- **Code.** §3.4 for applied rows: edit increment, insert and delete, by
  ownership, with the bend-adjacent refusals. The §3.6 lifecycle for `edit`,
  `insert` and `delete` entries.
- **Tests.** Every batch case below is hand-built; the builder's batch
  deep-equals it, and native and wasm apply it to the same expected model.
  - **Increment edit.** Downstream stored positions are unchanged, bit for
    bit.
  - **Undo and Redo of applied edits.**
    - Create A, edit B, edit C. Undo, Undo, Redo, Redo: the row shows C,
      then B, A, B and C, and stays `applied`. Ids exist throughout.
    - **Divergent edit.** After two Undos, a new edit D disposes B and C.
      The row's entries are A, D, and a further Redo is unavailable in both
      the session and the sheet.
  - **Invalidated edit, then Requeue.**
    - Apply A, enter and queue edit B, then change an unrelated entity on
      another surface.
    - The batch is retired, the row returns to `applied` showing B as
      "edited, not applied", and Requeue emits B's values on the current
      model.
  - **Neighbours stay applied.**
    - After an insertion between applied rows, and after a deletion with a
      reconnect, the next row has a `refresh` entry and stays `applied`.
    - After Undo it is `applied` with its earlier record; after Redo, with
      the new one.
  - **Outside restoration of a deletion.** Delete an applied row. Restore
    one of its entities on another surface (re-create the same id): the
    row is `detached`, "Restored outside the sheet". Undo of that outside
    change returns it to `deleted`.
  - **Restoration after later checkpoints.**
    - Clear an applied row's support code, so an edit deletes support S.
    - Then apply an increment edit of the same row, and an insertion that
      gives it a `refresh` entry.
    - Re-create S on another surface: the row is `detached`. S stays in
      every later entry's `removedIds` until Adopt.
  - **Shared-section insertion.** The inserted pipe carries `section_ref`, and
    a later shared-section edit propagates to it.
  - **Deleting the first row** (a `P` with no later rows) deletes its node and
    support. With later rows, it is refused.
  - **Deleting `F` and `S` rows** deletes only their supports. An `F` with
    later rows is refused.
  - **Existing-node ownership.** Deleting a run to an existing node, and
    deleting an `F` row, never emits `delete_node`.
  - **Deleted rows are not recreated.** After a deletion applies, Queue emits
    nothing for the row. Undo of the deletion restores it as `applied`.
  - Refusals cover a support or branch at the deleted node, and rows next to
    a bend.
  - Insert and delete batches containing a reconnect are preview-gated.

S1 lands first. S2 depends on S1, S3 on S2, and S4 on S2.

**Ids and sources, padding and pipe numbering** (RT-G2, exit criterion 7;
gaps G1–G3 of DEL-09-01 `Work/reference-system-run-sheet.md` §6). After S2.
- **Code.** The "Ids and sources" column group (§1.2), shown and hidden like
  Section extras; the header's Pad numbers to N digits and Pipe number from
  (§3.2); the rows' entered support and bend ids, support label and bend
  geometry source in the resolver, the builder and the record projection;
  the not-applicable and id refusals of §4.2. No operation kind, engine change
  or model-document member.
- **Tests.**
  - `runSheet.test.ts`: padding and its reservation and refusal; start-node
    pipe numbering, its fallback and the fallback's refusal; an applied row
    keeping its pipe id; entered support id and label; two supports at one
    node, applied by the wasm engine; entered bend ids and geometry source in
    consecutive bends, with projected corner contexts equal to the engine's
    and the batch applied by the wasm engine; not-applicable cells refused by
    name; entered support and bend ids refused when reserved, stored or
    duplicated.
  - `RunSheet.test.tsx`: the group's toggle, labelled cells, derived and
    entered origins, refusals that name the cell, Tab into the group; the
    header fields, their lock once a row has a number, and an invalid padding
    disabling Queue.
  - `runSheet.session.test.tsx`: through the workspace session and the wasm
    engine, a sheet with two supports at one node and two consecutive bends
    with entered ids, labels and geometry source, padded start-node numbering,
    validated and applied with ids exactly as entered.

## 8. Decisions (Agent 0)

Each choice below was decided by Agent 0 under the owner's 2026-10-27
delegation and adopted as recommended. The reasons are kept.

| # | Choice | Decision | Consequence |
|---|---|---|---|
| O1 | Applied-row edit: keep downstream stored positions, or shift downstream nodes with the edited one | Decided by Agent 0 (owner's 2026-10-27 delegation): adopted as recommended. Keep (§3.4). Later, an explicit "Shift following rows" command on the selected downstream pipes through the existing `transform_pipe_run` translate, previewed. | No implicit geometry moves. A mistyped length in an applied run needs one extra explicit command. |
| O2 | Should k and k source inherit between bend rows? | Decided by Agent 0 (owner's 2026-10-27 delegation): adopted as recommended. No. Use explicit Fill down. | Holds the "k is user data, never suggested or defaulted" rule (`bend_authoring.rs:5-7`). One keystroke per bend run. |
| O3 | Derive guide and line-stop restraints from an axis-aligned run, or fill all six for an anchor | Decided by Agent 0 (owner's 2026-10-27 delegation): adopted as recommended. No in v1. Restraints are typed, with `*` for all six. | Holds the coverage record's "No inferred restraints". Allowing it would revise that record and fill shown, code-derived values; V would still need typed restraints, since the model has no vertical axis. |
| O4 | Add the kernel orientation check (as reconnect's R7) to `connect_pipe_run`, so near-parallel orientation refuses at authoring and not at solve | Decided by Agent 0 (owner's 2026-10-27 delegation): adopted as recommended. Yes, as its own applier PR with regression tests. Agent 0's conditions for that PR: (a) it lists exactly which previously accepted connect batch shapes are now refused, and why each is wrong; (b) it includes a regression test that fails at base; (c) it shows that every stored fixture and model that replays `connect_pipe_run` still passes, or lists each one that fails and why; (d) it gets a separate Codex review of the refusal change. If an `apps/desktop/native-smoke/expected.json` change would be needed, the PR stops and reports it. | Changes refusals for existing connect batches (persistence risk class). Without it, the sheet catches only exactly parallel orientations. |
| O5 | Queue all draft rows or only the valid ones | Decided by Agent 0 (owner's 2026-10-27 delegation): adopted as recommended. All or nothing. | Matches the atomic batch. One refused row blocks the queue, and the reason names it. |
| O6 | Require preview for every batch or intent containing `insert_bend_at_corner` | Decided by Agent 0 (owner's 2026-10-27 delegation): adopted as recommended. Yes, as S2's prerequisite (§7). Gating the existing bend form behind preview is accepted; S2 states the behaviour change in its PR body and in the deliverable text that describes the bend form (§7 S2). | The bend moves the corner to tangent points the user did not type, so the review shows them before apply. The bend form's intents are gated too, which adds one preview step to that existing path. |
| O7 | Operating-condition column | Decided by Agent 0 (owner's 2026-10-27 delegation): adopted as recommended. Defer to C1. C1's brief must carry the operating-condition column for the run sheet (§1.2). | B1 needs no version step. Rows inherit conditions once C1's member exists. |
| O8 | Persist sheets, so a sheet reopens as a sheet | Decided by Agent 0 (owner's 2026-10-27 delegation): adopted as recommended. No. Rows are transient; a later "load run from model" view (with B4) can derive rows from stored geometry. | Persisting would need workspace persistence, kept beside the project and not necessarily in the model document. It needs a model-document member, and a version step, only if sheets are to travel with the model. |
| O9 | Default numbering `node:N`/`pipe:P`, start 10, step 10, counter-based | Decided by Agent 0 (owner's 2026-10-27 delegation): adopted as recommended. As designed. | Ids never renumber; insertion order is not encoded in ids. Labels remain free for the user's scheme. |
| O10 | Consecutive bends (§3.3) | Decided by Agent 0 (owner's 2026-10-27 delegation): adopted as recommended. Allow them, with the id-only record projection. | The builder mirrors three structural effects (append, `section_ref`, corner replacement) and computes no number. A drift is refused by the engine and pinned by S1. The alternatives are refusing consecutive bends, which loses common geometry, or an engine-backed projection, which needs an acceptance-shaped scratch apply or a change to DEL-16-06's validate contract. |

Codex review r1 agreed with O1–O9 and r2 with O10; r5 passed the design.
