---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-07-08
package_id: PKG-07
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-076]
package_objective_refs: [OBJ-015, OBJ-016]
---

# Scope of Work — DEL-07-08 Design-authoring state and comparison workspace

## Purpose and Objective Traceability

This deliverable is where the engineer works with the history of a design and not only its current model. It shows the design knowledge and constraints a model is built against, the saved states of the model and the runs solved on them, and what changed between any two of them, as tables and as an overlay on the model. It is also where every proposed change, whoever or whatever wrote it, is reviewed, previewed and explicitly applied, and where the record of what was applied can be read. The result it contributes is that a change to a piping model is always a reviewed, attributable step, and that two versions of a design or two runs of it can be compared without leaving the desktop.

It serves SOW-076 under OBJ-015 and OBJ-016 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-07, row DEL-07-08), and PRD §14.5, §13.1, §13.2, §15.1 to §15.3, FR-GUI-007 and FR-CMP-001 to FR-CMP-010. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names this deliverable.

- **OUT-001** — The design-knowledge and constraint panels, the state and run browser, and the comparison views with their overlay.
- **OUT-002** — The review and apply interaction for operations and proposals, and the presentation of the operation ledger.

## Deliverable Definition — Ontology

**Terms the workspace implements.**

- *Design knowledge.* The recorded basis a model is designed against: endpoints, line data, corridors, zones, equipment and their sources and unresolved assumptions. The panel shows these records in the form DEL-13-01 defines.
- *Constraint finding.* A message the constraint engine returns about the model against a constraint. The panel shows it; it evaluates no constraint.
- *State.* A named, immutable snapshot of the model. *Run.* A recorded solve of one state. A state's snapshot and a run's record are immutable; their name, tags, notes and state-level or run-level assumptions are labels the user may edit.
- *Comparison.* A deterministic result that classifies the entities of two states as added, removed, changed, unchanged or unresolved (DEL-14-03), and gives the result deltas of two runs with the rows left unmatched on each side (DEL-14-04, in DEL-14-05's form). A *mapping* pairs entities whose identifiers differ. A *tolerance profile* is the user's absolute and relative limits by result category.
- *Comparison overlay.* The model drawn with the entities a comparison classifies marked by class, with a legend in words.
- *Review queue.* The pending operations and batches of the session. Each item has an author, a validation finding and a preview.
- *Review stages.* An item is *proposed* when queued, *validated* when the engine's validate-only pass admits it, *accepted* when the user applies it, and *audited* when its decision record and outcome record are in the operation history. The stages are shown as distinct and none is inferred from another.
- *Apply.* The user's explicit act that sends a validated item to the operation engine.
- *Operation ledger.* The operation history as DEL-16-03 keeps it: each batch's decision record (accepted, rejected, held or withdrawn) and outcome record (applied, refused or unknown), with its operations and each operation's author. Session undo and redo are not the ledger and are not acceptance history.
- *Proposal.* A batch written outside the desktop's forms: by an agent, or offline and brought in as a file. A proposal carries one of five class words: "Check", "Open issue", "Draft", "Proposal" or "Evidence summary".

**Boundary.** The workspace presents and routes. The following belong elsewhere:

- the design-knowledge and constraint records and the constraint engine: DEL-13-01, DEL-13-02 and DEL-13-03. The constraint panels show constraint records and findings; editing a constraint record is DEL-07-02's;
- state and run records: DEL-14-01 and DEL-14-02. The comparison engines: DEL-14-03 and DEL-14-04. Mapping, tolerance and comparison export forms: DEL-14-05. CSV export of a comparison view: DEL-08-04;
- the operation form, validation and preview, the acceptance record and receipt, agent controls, and application: DEL-16-01, DEL-16-02, DEL-16-03, DEL-16-04 and DEL-16-06;
- moving queued single changes into one batch: DEL-07-02/REQ-017;
- the forms that author an operation: the deliverables that own each editor;
- the session, its history and its guard on replacing the working model: DEL-07-11;
- how a finding's class and fields are shown: DEL-07-04;
- reading a single result: DEL-07-05.

**Not decided.** The display words for the four review stages. They are the claims registry's to give (DEL-01-04); until it does, the stage names above are shown. Which surface enters design knowledge is not decided (DEL-13-01). How a user reviews and confirms a mapping is not decided (decomposition OI-014).

## Completion and Reliance Basis — Epistemology

### Requirements

- **REQ-001** — The workspace provides design-knowledge panels, constraint and warning panels, a state and run browser, comparison tables and a graphical comparison overlay.
- **REQ-002** — Every panel shows a record in the form its owning deliverable defines, with the identities and fields the record carries. No panel gives a record a meaning, a status or a placeholder value of its own.
- **REQ-003** — A change to the model or to design knowledge is made only as an operation through the review queue and an explicit Apply. No panel writes a stored payload.
- **REQ-004** — Proposed, validated, accepted and audited are shown as distinct stages of an item. Nothing is shown as accepted before the user's Apply, or as audited before its record exists.
- **REQ-005** — The browser saves the working model as a named state with tags, notes and state assumptions, records a run against a saved state, edits a state's or a run's labels, lists saved states, runs and comparisons with their names, tags, notes, identities and hashes, opens each for review, and reopens a state as the working model through the shell's guard. A removal the store refuses is shown by the refusal's name. The browser never changes a snapshot, a recorded run or a hash (DEL-14-01, DEL-14-02; PRD FR-CMP-001).
- **REQ-006** — Constraint findings and warnings keep their governed fields and classes and are shown under DEL-07-04's presentation rules.
- **REQ-007** — A comparison is shown as a review aid. No comparison view, overlay or export option presents a comparison as external validation, approval or compliance.
- **REQ-008** — Missing information is visible: an unmatched entity, an unmapped pair, a value without a unit and an absent record are each stated, and nothing is defaulted to make a comparison complete.
- **REQ-011** — *Comparison content.* A comparison view shows the two states or runs compared with their identities; the mapping table; unmatched entities; added, removed and changed entities; changed solver settings; changed rule-pack and library references; result deltas with units; items outside the user's tolerance profile; the overlay; and the export options (PRD §14.5). Each view renders from a view model that gives the rows its filters match, with each side's identity, stored values and units.
- **REQ-012** — *Review and apply.* The queue shows each pending item with its operations, its author, the model revision it was written against, its declared assumptions, its validation finding and its deterministic preview. Apply is offered only for an item that is validated and previewed, and the only decision passed to the engine is the user's own Apply or Reject. A refusal is shown by its name and leaves the model unchanged. An item made stale by another change is refused and must be queued and validated again; it is never rebased.
- **REQ-013** — *Ledger.* The ledger panel shows the operation history as recorded: each batch's decision record and outcome record, including rejected, held, withdrawn, refused and unknown, with its operations and each operation's author. Undo and redo do not alter it and are not shown as part of it.
- **REQ-014** — *Proposals.* An agent's or an offline proposal enters the same queue and passes the same validation, preview and Apply as a change from a form. It is shown with its author and its class word. A proposal's retained context that is reloaded is shown as "acceptance unknown" and must be queued and validated afresh. The offline intake offers the machine-readable capability list and operation schema for download as its authoring reference.
- **REQ-015** — *Mapping and tolerance.* The mapping editor and tolerance profiles are Should capabilities (PRD FR-CMP-005, FR-CMP-007, FR-CMP-008), and no exit criterion of the release target requires them. Where the workspace offers them, the user maps, unmaps and marks entities intentionally unmatched, and defines a tolerance profile of absolute and relative rules by result category, each written as a record in DEL-14-05's form, with no number supplied by the product.

### Acceptance criteria

- **AC-001** — For an invented project with design-knowledge records, constraints with findings, two saved states and two runs: each panel shows every record with the identities and fields the stored record carries and nothing more; and after browsing, reopening, comparing, saving a state, editing labels, recording a run and a refused removal, each snapshot, run record and hash is byte for byte what it was, and the edited labels read back.
- **AC-002** — For a queued batch: the item shows its operations, author, model basis and declared assumptions equal to the batch as registered; the preview shown equals the validate-only preview the engine returns; Apply is not offered before validation or before the preview is shown; after Apply the ledger shows one decision record and one outcome record with every operation and its author as recorded; a rejected batch is shown in the ledger as rejected; a refused batch leaves the model document unchanged and shows the refusal's name; and a stale item is refused.
- **AC-003** — For two states built by hand to differ by one added, one removed and one modified entity, one changed unit-bearing value and one entity whose identifier changed: the view shows exactly those classes with both sides' values and units; shows the renamed entity as the comparison classifies it, removed on one side and added on the other without a mapping and as one pair once a mapping names both; states a unit-bearing value without a unit and an absent record without defaulting either; shows under each filter the rows the view model gives, with each side's identity, stored values and units; and the overlay marks the same entities with a legend in words.
- **AC-004** — For two runs whose results differ by amounts set by hand, under a tolerance profile set by hand: the view shows each delta with its unit, flags exactly the items outside the tolerance, lists the changed solver settings and rule-pack references, and shows under each filter the rows the view model gives, with each side's identity, stored values and units.
- **AC-005** — An agent's proposal and an offline proposal each appear in the queue with author and class word, cannot be applied without validation and the user's Apply, and a reloaded proposal context reads as "acceptance unknown".
- **AC-006** — No text of the workspace states or implies a claim PRD §21.2 prohibits, or presents a comparison or a review stage as validation, approval or compliance; any boundary language is a registered statement used word for word; and every status label is the registry's display form.
- **AC-007** — Where the mapping editor and tolerance profiles are offered: a mapping made in the editor pairs the renamed entity of AC-003's fixture; an entity marked intentionally unmatched shows that classification; unmapping restores the earlier classification; and a tolerance rule left without a value leaves its deltas unclassified with the reason.

## Production and Verification Method — Praxeology

**Method choices.** Each panel is a view over records and results other deliverables produce, and holds no copy that could diverge. The review queue is one component for every author of a change, so a proposal cannot reach the model by a shorter path than a form. Comparison views are pure functions from a comparison result to a view model, which the table, the overlay and the export read alike.

**How each criterion is examined.**

- **VER-001** — Executable check (observes the panels against the stored records of an invented project): reads every displayed identity and field and compares them with the stored records, then compares each saved record's bytes before and after the session.
- **VER-002** — Executable check (observes the review queue, the operation engine and the ledger in a session): queues a batch, compares the displayed preview with the engine's validate-only preview, reads the state of Apply at each stage, applies, and compares the ledger panel with the stored decision and outcome records; repeats with a rejected, a refused and a stale batch and compares the model document's bytes.
- **VER-003** — Executable check (observes the state-comparison view and overlay against hand-built states): compares the classes, values, units, filtered rows and marked entities with the differences built into the fixture, before and after a mapping made in the editor, an entity marked intentionally unmatched and an unmapping.
- **VER-004** — Executable check (observes the run-comparison view against hand-set deltas and tolerances): compares each displayed delta, flag, setting, reference and filtered row with the fixture's, and leaves one tolerance rule without a value and reads how its deltas are classified.
- **VER-005** — Executable check (observes the queue with an agent's and an offline proposal): reads author, class word and the state of Apply, reloads a proposal's context and reads its marking.
- **VER-006** — Comparison with a named reference (the prohibited claims of PRD §21.2 and the claims registry's registered statements and display forms): reads every text the workspace can show and compares it with them.

## Governing Values and Decisions — Axiology

- **AX-001** — Every change is a reviewed step. Better is one queue with one Apply for every author, measured by no path to the model that skips validation, never a convenience that applies on import or on an agent's say.
- **AX-002** — A record of the past is not rewritten. Better is a saved state, a run and a ledger entry read back as stored, measured by their bytes, never a view that tidies, completes or relabels them.
- **AX-003** — A difference is shown, not judged. Better is a delta with both values, their units and the tolerance it was tested against, never a comparison that reads as agreement with an authority.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-076 OBJ-015 OBJ-016 | REQ-001 REQ-002 REQ-005 REQ-006 | AC-001 | VER-001 | The stored records are the reference and are written by their owning deliverables (DEL-13-01, DEL-13-03, DEL-14-01, DEL-14-02), not this one. This shows the panels alter nothing. |
| OUT-002 | SOW-076 OBJ-015 | REQ-003 REQ-004 REQ-012 REQ-013 | AC-002 | VER-002 | The registered batch, the engine's validate-only preview (DEL-16-02) and the stored decision and outcome records (DEL-16-03) are the references; the model document's bytes before the refused batch are the reference for no change. |
| OUT-001 | SOW-076 OBJ-016 | REQ-008 REQ-011 | AC-003 | VER-003 | The differences are built into the fixture by hand, so the expected classes do not come from the comparison engine. The classes' names are DEL-14-03's. |
| OUT-001 | SOW-076 OBJ-016 | REQ-011 | AC-004 | VER-004 | The deltas and the tolerance profile are set by hand in the fixture; which items fall outside is arithmetic on them. |
| OUT-002 | SOW-076 OBJ-015 | REQ-014 | AC-005 | VER-005 | PRD §5.3 makes an agent's action a model operation under the same review; the five class words are those stated above. The scenario states the expected markings. |
| OUT-001 | SOW-076 OBJ-016 | REQ-007 | AC-006 | VER-006 | Outside source: PRD §21.2. The registered statements and display forms are DEL-01-04's, which this deliverable does not write. |
| OUT-001 | SOW-076 OBJ-016 | REQ-015 | AC-007 | VER-003 VER-004 | PRD FR-CMP-005, FR-CMP-007 and FR-CMP-008. The renamed entity and the valueless rule are built into the fixture by hand; the classification of an unclassified delta is DEL-14-04's. |
