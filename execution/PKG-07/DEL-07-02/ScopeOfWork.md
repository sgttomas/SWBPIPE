---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-07-02
package_id: PKG-07
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-020,SOW-021,SOW-077]
package_objective_refs: [OBJ-006]
---

# Scope of Work — DEL-07-02 Model tree and property inspector

## Purpose and Objective Traceability

This deliverable is where the engineer reads and edits the model as records. The model tree and its tables list every entity by kind; the property inspector shows the selected entity's typed properties with their units, sources and findings, and edits them. The run sheet enters a piping run row by row from the keyboard; bulk change edits many entities at once; context menus put each entity's commands on the entity; and supports are created and edited here. Every edit is a structured operation that the user reviews before it is applied, and no field is ever filled with a value the user did not give. The result this deliverable contributes is that a complete model can be entered, inspected and corrected quickly and by keyboard, with what is missing always visible.

It serves SOW-020, SOW-021 and SOW-077 under OBJ-006 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-07, row DEL-07-02), and PRD §14.1, §14.3, §5.3, §5.5, FR-GUI-002, FR-GUI-003 and FR-GUI-004. Exit criterion 7 of `docs/RELEASE_TARGET_v1.0.md` names this deliverable with DEL-07-06: from blank, the reference system is entered by run sheet and solved in 30 minutes or less. The 30 minutes is Agent 0's provisional target and becomes final only on the owner's acceptance after the walkthrough.

- **OUT-001** — The model tree, tables and property inspector: navigation, selection, typed property display and edit, and the model-data sections.
- **OUT-002** — Generic and support editing: creation, change and guarded removal of entities from the tree, tables and inspector, including supports and restraints, with the run sheet, bulk change, context menus and the review of queued changes.

## Deliverable Definition — Ontology

**Terms the tree and inspector implement.**

- *Model tree.* The model's entities grouped by kind, in a tree view and a grid view of engineering tables. A row is an entity, addressed by its stable identifier. Lists are virtualized, so their cost does not grow with what is off screen.
- *Property inspector.* The typed properties of the selected entity: type, geometry, units, material, section, component properties, loads, support settings, flexibility and stress-intensification inputs, source and provenance, validation status, dependent rule-pack inputs, and linked assumptions and notes (PRD §14.3).
- *Not stated.* How an absent member is shown. An empty field means the model holds no value; it is never read as zero and never shown as a default.
- *Intent, pending change and batch.* An edit produces one structured operation (an intent) or an ordered set applied whole (a batch). Pending changes wait in one review queue until the user applies them.
- *Guarded removal.* Removal of an entity that something else still names is refused, and the refusal lists what names it.
- *Support form.* The form that creates or edits a support: its family (anchor, guide, line stop, vertical support, spring, variable spring hanger, spring hanger, constant-effort support, nonlinear), its restraints, hanger data, gap, lift-off and friction settings, and an optional direction. Restraints are typed by the user and never derived from geometry.
- *Run sheet.* A draft table in the grid view in which each row creates the next end node and the element to it from three increments. Row kinds are: run; bend run, whose end node is a corner realized as a bend; support only; start from an existing node; start at a point; valve run; and flange run. Blank cells inherit section, material and condition set from the row above and show the inherited value and its origin. Rows are transient and are not saved with the model.
- *Sheet numbering.* Identifiers a sheet assigns: a prefix and a number from a counter with a start and a step, 10 and 10 unless changed, optionally zero-padded. A number is assigned once and is never reused or renumbered. Start-from-node and support-only rows take no number.
- *Bulk change.* One change applied to every matching entity of the selection, previewed entity by entity before it is queued.
- *Context menu.* A menu opened on an entity, a tree row, a grid row or a table cell that lists commands which already exist elsewhere and runs them by their existing paths.
- *Line.* A named group of pipes, held in the model's lines section.
- *Operating-condition set.* A named set of keyed operating conditions that a pipe may reference; DEL-02-01 defines the record.

**Boundary.** DEL-00-05 owns the rule for GUI state, selection and the mutation route; this deliverable owns the behaviour of the tree, tables and inspector within it. The following belong elsewhere:

- material, section, component, library and rule-pack editors, and load-case and combination editing in the load manager: DEL-07-03. This deliverable shows the selected load in the inspector and does not edit load cases;
- the 3D viewport and its tools: DEL-07-01;
- the missing-data panel and what blocks a solve or a rule check: DEL-07-04;
- the meaning, validation and application of every operation: DEL-16-01, DEL-16-02 and DEL-16-06;
- the review queue's preview and Apply interaction and the operation ledger: DEL-07-08. Moving queued single changes into one batch is this deliverable's;
- the mathematics of supports and their refusals: DEL-04-03 and DEL-04-04;
- hanger sizing and its surfaces: DEL-04-11;
- the shell that mounts these surfaces and holds the session: DEL-07-11;
- coverage of the operation vocabulary: DEL-07-09.

**Not decided.** Replacing the kind of an existing element in bulk waits for an operation that changes a component's kind; none exists and none is committed here. Whether the support form creates snubbers, a family the model document defines, is not decided (DEL-07-09 roadmap item R2).

## Completion and Reliance Basis — Epistemology

The tree and inspector show what the model document holds and collect what the user enters. A field that is filled establishes only that a value was entered, with whatever source the user recorded.

### Requirements

- **REQ-001** — The tree and tables let the user navigate lines, nodes, elements, components, supports, load cases, states and analysis runs. They hold no copy of the model: what they show is read from the model document (SOW-020; PRD FR-GUI-002).
- **REQ-002** — Selecting an entity shows its editable typed properties, units, provenance, warnings and dependencies, for every property family of PRD §14.3 (SOW-021; PRD FR-GUI-003).
- **REQ-003** — A unit-bearing value is shown and edited with its unit, and a value of the wrong dimension is refused with a finding that names the field.
- **REQ-004** — A value that a solve or a rule check needs and the model lacks is shown as a finding on the entity that lacks it. Nothing in the tree, tables, inspector, run sheet or bulk change supplies it (PRD §5.5).
- **REQ-005** — The source, provenance and redistribution or private status of materials, sections, components and rule-pack references are shown wherever those records are shown.
- **REQ-006** — Every edit is a structured operation that is validated, queued for review and applied on the user's Apply. Selection, expansion, filters, drafts and focus are transient (PRD §5.3; decomposition AB-00-05).
- **REQ-007** — A diagnostic shown on an entity keeps the class, severity, source and remediation it had in its envelope, and no text states or implies certification, approval or code compliance (PRD §21.2).
- **REQ-008** — Fixtures, examples and screenshots for these surfaces use invented data only.
- **REQ-009** — The tree and tables stay responsive on a 10,000-pipe model: on the reference profile DEL-07-01/REQ-012 defines, filtering to presented rows, row selection to canvas highlight and table readiness after a model is assigned meet the limits stated there.
- **REQ-010** — The user creates, changes and deletes entities from the tree, tables and inspector, including supports and restraints of every family in the support form, equipment and nozzle boundary conditions as explicit free, rigid or spring degrees of freedom, and a tee's modifier source. Removal is guarded. A nonlinear or non-convergence finding that names a support is shown on that support. A stored value is never rebuilt by an edit that did not touch it (SOW-077; DEC-111; DEL-07-09 vocabulary rows 2, 12, 13, 16, 18 and 21).
- **REQ-011** — Every unit-bearing field of the tree, tables and inspector lets the user choose its unit from the catalog's units of that dimension.
- **REQ-012** — The model-level modulus basis records are entered and edited in these surfaces.
- **REQ-013** — *Model data.* Lines, tags and notes on the project and on each entity, provenance records on materials, sections, components and supports, assumptions and open issues, and constraints and equipment interfaces as records that nothing evaluates are entered and edited here through structured operations (PRD FR-MOD-002, FR-MOD-004, FR-MOD-006, FR-GUI-004).
- **REQ-014** — *Context menus.* Entities in the viewport, tree rows, grid rows and table cells and row headers each have a context menu of existing commands, with shortcuts shown. An unavailable command is listed, focusable, disabled and described. Shift+F10 and the ContextMenu key open the menu; on the canvas a secondary press opens it only if the pointer moved no more than 4 CSS px, so a right-drag still pans. The menu uses the menu, menu-item and separator roles and supports the arrow keys, Home, End, type-ahead, Enter, Space and Escape. Tab and Shift+Tab close the menu and move focus to the next or previous tab stop from the invoker. Editing fields keep their native menu. In the Table view, the tree, grid and table menus offer Hide selected, Isolate selected and Fit selection. Opening a menu runs nothing. Right-clicking an unselected entity selects it; a selected one keeps the selection. After a command or Escape, focus returns to the invoker or, if it is gone, to the nearest place that still exists.
- **REQ-015** — *Run sheet.* The grid view has a run sheet whose rows, of the kinds defined above, create nodes, pipes, bends, valves, flanges and supports through existing operations in one reviewed batch that is applied whole. The requirement further fixes:
  - *Entered values only.* A flexibility factor, a stress-intensification pair and every component value are typed on their own row and never inherit; a bend's two directional factors are entered both or neither. Restraints are typed, with one token for all six.
  - *Preview.* A batch that realizes a bend or reconnects a pipe cannot be applied before its preview has been shown for that exact batch on the current model.
  - *All or nothing.* One refused row blocks the queue, and the reason names the row.
  - *Applied rows.* Editing an applied row's increment moves only that row's end node; deleting a row removes only what that row created; a row whose stored record was changed elsewhere says so.
  - *Units.* Each quantity column has one unit, starting as the project's; a column with no unit accepts no entry.
  - *Identity.* Identifiers follow sheet numbering and may be typed before queueing; consecutive bends are allowed; the sheet adds no member to the model document.
  - *History.* A row's status follows queue, apply, Undo and Redo, and a batch prepared on an earlier model is withdrawn when the model changes.
- **REQ-016** — *Bulk change.* Over the selection the user can assign a section, reassign a material, replace or remove a support kind, set the radius of unrealized bends, add a tag and assign or clear an operating-condition set. The preview counts affected, unchanged, not-matched and refused entities and shows before and after for each affected one. Any refusal queues nothing. The change is one batch, applied whole and reverted by one Undo. Restraints are kept or entered, never inferred.
- **REQ-017** — *Review as one batch.* The user's queued single changes can be moved into one batch, in queue order, that is validated and applied all or none as one undo step. A change made stale by another in the batch is refused by name and never rebased; changes sharing an operation identifier, and another author's changes, are not moved.
- **REQ-018** — *Operating-condition sets and support direction.* Condition sets and their keyed conditions are created, edited and deleted in the tree; a pipe's set is shown and assigned in the inspector and the pipes table; a member the solve does not read is marked as not used by the solve. A support's optional direction is entered as three numbers, written exactly as entered and never normalized; a partial vector, a non-number and negative zero are refused before queueing; a stored direction is shown exactly as stored, and no other edit of the support changes or drops it.
- **REQ-019** — *Authoring time.* From a blank project, a person following DEL-09-01's run sheet enters the reference system in the desktop, with geometry, components and supports entered through the run sheet and these surfaces and load cases and combinations through DEL-07-03's load manager, and solves it in 30 minutes or less (`docs/RELEASE_TARGET_v1.0.md` criterion 7; the threshold is provisional). Which solve ends the clock is not decided: the criterion says only that the system is solved.

### Acceptance criteria

- **AC-001** — For an invented model holding every entity kind of REQ-001, each entity has one row; selecting a row selects the same identifier in the viewport and the inspector; the inspector shows every stored member of the selected entity with its unit and source; an absent member reads "not stated"; a value of the wrong dimension is refused by name; and each missing solve-required or rule-required value appears as a finding on its entity. Changing selection, filters or expansion leaves the model's canonical hash unchanged.
- **AC-002** — Each create, change and delete of REQ-010 and REQ-013 queues an operation, leaves the model unchanged until Apply, and after Apply gives a document that passes the document validator. Removing an entity that is still referenced is refused and lists the referencing entities. After an edit of one member, every other member of the record is bit for bit what it was. An explicit family that the model document does not define is refused.
- **AC-003** — On the DEL-07-01/REQ-012 reference profile, the filter, row-selection and table-readiness segments of the frozen workload meet their limits at 1,000 and 10,000 pipes.
- **AC-004** — Every unit-bearing field offers the catalog's units of its dimension and no other. A modulus basis record entered here is stored as entered.
- **AC-005** — Each context menu lists only commands available elsewhere and runs the same path; every disabled item states its reason; the menu opens from the keyboard on every surface of REQ-014 and answers each key REQ-014 names, Tab and Shift+Tab included; an editing field shows its native menu; a right-drag on the canvas pans and opens no menu; opening a menu leaves the model, the queue and the selection as REQ-014 states; and after each command and after Escape, focus is on the invoker when it is still shown, and otherwise on the nearest place of the same surface that still exists.
- **AC-006** — A run sheet holding each row kind queues one batch whose applied model equals a model built by hand from the same rows, record for record. A refused row queues nothing and is named. A bend batch cannot be applied before its preview. Entered factors appear only on the row that entered them. Applied-row edits and deletions change only what REQ-015 allows. Undo and Redo restore the canonical hash and the row statuses. Numbers are assigned once and never reused.
- **AC-007** — For each bulk change, the preview's four counts and before-and-after values equal those computed from the selection; a selection containing one entity the engine would refuse queues nothing; and one Undo restores the model's canonical hash.
- **AC-008** — Moving queued single changes into one batch preserves queue order; Apply applies all or none; one Undo reverts all; a change made stale by an earlier one in the batch is refused by name; and changes that share an operation identifier or have another author stay where they were.
- **AC-009** — A condition set can be created, edited and deleted, and an edit changes only the members the user touched. Deleting a set a pipe still uses is refused and names every pipe. A support created with a direction stores the three entered numbers exactly; each of the three refusals of REQ-018 queues nothing; and after any other edit of that support the stored direction is unchanged.
- **AC-010** — Starting from a blank project and following DEL-09-01's run sheet, a person enters the reference system in the desktop as REQ-019 states. The entered model equals the reference system's fixture record for record, apart from the differences that run sheet lists, and the time from blank to a completed solve is 30 minutes or less.

## Production and Verification Method — Praxeology

**Method choices that define these surfaces.**

- Surfaces build operations; one engine validates and applies them. A check the engine makes is made by the engine, before queueing, and its refusal is shown by its own code.
- Nothing inherits that the user would have to justify. Section, material and condition set inherit down a run sheet because they are properties of a run; factors, restraints and component values do not.
- Applied geometry is never moved implicitly. An edit to one row changes that row's node.
- Identifiers are identity, not order: they are assigned once and never renumbered.
- A sheet is a draft surface, not a document: it is held beside the session and is not saved.
- Every pointer route has a keyboard route.

**How each criterion is examined.**

- **VER-001** — Executable check (observes rows, selection, inspector fields, findings, queued operations and the document hash): exercises the tree, tables and inspector on invented models with the operation engine, for AC-001, AC-002 and AC-004.
- **VER-002** — Executable check (observes interaction latency on the reference profile): the table segments of the DEL-07-01 workload under its pass rule, for AC-003.
- **VER-003** — Executable check (observes menu contents, roles, key handling, focus and the commands run): opens each menu by pointer and by keyboard on each surface, for AC-005.
- **VER-004** — Comparison with a named reference (hand-built expected batches and applied models for invented sheets): applies sheet batches through the operation engine and compares the applied model record for record; with an executable check of preview gating, refusals, history and numbering, for AC-006.
- **VER-005** — Executable check (observes preview counts, queued batches and the hash after Undo): runs each bulk change and the batch review on invented selections through the operation engine, for AC-007 and AC-008.
- **VER-006** — Executable check (observes stored condition-set and support records after each edit through the operation engine): for AC-009.
- **VER-007** — Comparison with a named reference (the reference system's fixture, which DEL-09-01 owns): a scripted entry following the run sheet compares the entered model with the fixture; and judgment by a named role (the person performing the walkthrough) times one unassisted entry and solve, for AC-010.

## Governing Values and Decisions — Axiology

- **AX-001** — *Faster entry of a correct model.* Better is less time and fewer keystrokes to enter the reference system, measured by the walkthrough time against 30 minutes. Speed is never bought by inferring, inheriting or defaulting an engineering value, by applying a change the user has not reviewed, or by weakening a refusal.
- **AX-002** — *Missing stays visible.* Better is fewer places where an absent or unsourced value can pass unnoticed, measured by the stored members the surfaces do not show and the findings not reachable from their entity. A tidier screen never hides a finding or shows an absent value as a number.
- **AX-003** — *Leave untouched data alone.* An edit changes what the user edited. Convenience in a form never justifies rewriting, normalizing or dropping a member the user did not touch.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-020 SOW-021 OBJ-006 | REQ-001 REQ-002 REQ-003 REQ-004 REQ-005 REQ-006 REQ-007 REQ-008 | AC-001 | VER-001 | The invented model says what must be shown: the surfaces are compared with the stored document, which they did not write. The property families are PRD §14.3 |
| OUT-002 | SOW-021 SOW-077 OBJ-006 | REQ-006 REQ-010 REQ-013 | AC-002 | VER-001 | The criterion states the expected record after each edit; validity is judged by the document validator and refusals by the operation engine, neither written by these surfaces. The families are those the model document defines (DEL-02-01) and DEL-04-03, DEL-04-04 and DEL-04-10 give meaning |
| OUT-001 | SOW-020 OBJ-006 | REQ-009 | AC-003 | VER-002 | Owner ruling: the limits, profile and pass rule of DEL-07-01/REQ-012; the measured quantities are reported by the browser |
| OUT-001 | SOW-021 OBJ-006 | REQ-011 REQ-012 | AC-004 | VER-001 | The unit catalog (DEL-02-02) says which units a dimension has; the stored record is compared with what was entered |
| OUT-002 | SOW-077 OBJ-006 | REQ-014 | AC-005 | VER-003 | Outside source: the WAI-ARIA menu pattern for roles and keys; the 4 CSS px threshold and the command list are an owner ruling under the scope delegation |
| OUT-002 | SOW-020 SOW-077 OBJ-006 | REQ-015 | AC-006 | VER-004 | A reference that imports no product code: expected batches and applied models built by hand for invented sheets. Row-status and numbering behaviour compare the product with the criterion's own statement |
| OUT-002 | SOW-021 SOW-077 OBJ-006 | REQ-016 | AC-007 | VER-005 | Expected counts and before-and-after values are worked by hand from the invented selection, not computed by the bulk-change code; refusals are the operation engine's |
| OUT-002 | SOW-077 OBJ-006 | REQ-017 | AC-008 | VER-005 | The criterion states the expected order and outcome; staleness is judged by the operation engine |
| OUT-002 | SOW-021 OBJ-006 | REQ-018 | AC-009 | VER-006 | The entered numbers are the expected stored numbers; the refusals and their codes are DEL-02-01's and DEL-04-03's |
| OUT-002 | SOW-020 OBJ-006 | REQ-019 | AC-010 | VER-007 | Release-target exit criterion 7. Its 30-minute threshold is Agent 0's provisional target until the owner accepts it. The entered model is compared with DEL-09-01's reference-system fixture, which these surfaces did not write, and the time is observed on a person's entry |
