---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-00-05
package_id: PKG-00
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-060]
package_objective_refs: [OBJ-013]
---

# Scope of Work — DEL-00-05 GUI state and interaction architecture

## Purpose and Objective Traceability

This deliverable is the rule for what the GUI may hold and how it may change the model. The project (the model, its saved states and its analysis runs) is durable. The camera, hover, selection, a drag in progress, the panel layout and a job's progress are transient and never become model truth. Every action in the GUI that changes the model becomes a model operation sent through an application-service command, and undo and redo step through those edits. The result it contributes is that the viewport, the tree, the inspector, the editors and the palette behave as one editor on one model: one selection, one history, one way to change the model, and warnings that keep their class.

It serves SOW-060 under OBJ-013 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-00, row DEL-00-05, and architecture-basis row AB-00-05 in §8.1), with `docs/PRD.md` §5.3, FR-GUI-004 and FR-GUI-005 and `docs/SPEC.md` §8. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names it. It reaches criterion 7 (authoring time) and criterion 8 (usability) through DEL-07-02 and DEL-07-06, and criterion 17 through the one operation route.

- **OUT-001** — The GUI state rule: the split between durable and transient state, the mutation route, the scope of undo and redo, the shared selection, and the preservation of warning classes.

## Deliverable Definition — Ontology

**Terms the architecture implements.**

- *Durable project state.* The model document, the saved model states and the analysis runs with their records: everything a save persists and a hash covers.
- *Transient session state.* Camera and viewport settings, hover, selection, drag and snap, input not yet applied, panel layout, job progress, and any other view state, such as tree expansion, filters, focus and display preferences. It is not written into the model document or a run record and is not hashed, and losing it loses no engineering data.
- *Mutation route.* A GUI action that changes the model builds a structured model operation; an application-service command validates and applies it; the GUI shows the applied result. The GUI never writes the model document itself.
- *Undo and redo.* Steps over the applied, reversible model edits in order. Undo restores exactly the model before the edit and redo the model after it. After each step the diagnostics are those of the model at that step, so a change in solve readiness shows, and results computed for the model it replaced are no longer current. A change of transient state is not an entry in the history.
- *Selection.* One shared selection of model entities, expressed as stable entity identifiers and not as the state of a widget. The viewport, the tree and the inspector read and set the same selection.
- *Warning class.* One of the six classes defined by DEL-00-06, which a diagnostic keeps when the GUI shows it.

**Boundary.** This deliverable owns the cross-cutting rule; it implements no screen, component or viewport. Its criteria are observed on the desktop the deliverables below build, and a shortfall is repaired there.

- DEL-07-02 owns the behaviour of the model tree and the property inspector within this rule.
- DEL-07-01 owns the viewport and DEL-07-09 the tool palette.
- DEL-07-11 owns the workspace shell, which holds session state and coordinates the shared selection and history across panels.
- DEL-07-04 owns how warnings and blockers are shown, including the five distinctions of `docs/PRD.md` §14.4 and FR-GUI-006; DEL-07-07 owns how job progress is shown.
- The deliverables of PKG-16 own model operations; DEL-00-03 owns commands; DEL-00-06 owns the classes.
- The component and state-management mechanism is a choice of the implementing deliverable within the baseline DEL-00-01 records.

## Completion and Reliance Basis — Epistemology

### Requirements

- **REQ-001** — Durable project state is separate from transient session, viewport, selection and job-progress state (decomposition AB-00-05).
- **REQ-002** — GUI actions that change the model go through application-service commands and never mutate the domain directly. GUI state does not bypass application services, diagnostics, unit validation or data-boundary checks (`docs/PRD.md` §5.3).
- **REQ-003** — Undo and redo are scoped to reversible model edits, recover state deterministically, and preserve diagnostics when an edit changes solve readiness (`docs/PRD.md` FR-GUI-005).
- **REQ-004** — Selection and interaction are defined independently of the GUI framework. This deliverable owns the cross-cutting rule, and DEL-07-02 owns model-tree and property-inspector behaviour within it.
- **REQ-005** — GUI warnings preserve the solve-blocking, rule-check-blocking, provenance, assumption, nonlinear and IP-boundary classes (decomposition AB-00-06; `docs/SPEC.md` §8; the classes are DEL-00-06's).

### Acceptance criteria

- **AC-001** — Changing any transient state (camera, hover, selection, panel layout, a filter, a display preference, a job's progress) leaves the model document, its canonical hash and the stored run records unchanged, and no saved model document or run record holds a transient field.
- **AC-002** — Every GUI action that edits the model results in an applied model operation in the operation history. The only other changes to the working model are undo and redo stepping through that history and opening or replacing the project by its own command. No GUI module writes the model document except by taking the applied result of a command.
- **AC-003** — After any reversible model edit, undo yields a model whose canonical hash equals the hash before the edit and redo yields the hash after it. The diagnostics shown after each step equal those computed fresh for that model, and results computed for the replaced model are not shown as current. A change of transient state adds no history entry.
- **AC-004** — Selecting an entity in the viewport, the tree or the inspector selects the same entity identifier in the other two.
- **AC-005** — A diagnostic shown in the GUI carries the class it had in its result envelope, and the user can tell a solve blocker, a rule-check blocker and each warning class apart.

## Production and Verification Method — Praxeology

**Method choices that define the architecture.**

- A person's edit and an agent's proposal use the same operation route.
- Selection is held as entity identifiers, so no surface depends on another's widgets.

**How each criterion is examined.**

- **VER-001** — AC-001 and AC-003, executable check: drives transient changes and sequences of edit, undo and redo in the desktop and observes the model hash, the diagnostics and the length of the history at each step.
- **VER-002** — AC-002, executable check: for each edit path observes that a model change is accompanied by an applied operation, and searches the GUI modules for a write to the model document outside the path of a command's result.
- **VER-003** — AC-004 and AC-005, executable check: a browser journey that selects in each of the three surfaces and observes the other two, and that compares the class shown for each diagnostic with the class in its envelope.

## Governing Values and Decisions — Axiology

- **AX-001** — *One model, one route.* Better is fewer GUI code paths that can change the model without an operation; the target is none. A smoother interaction is never bought by editing state directly.
- **AX-002** — *Undo is exact.* Better is more kinds of edit whose undo restores the identical model. An edit that cannot be undone exactly is declared irreversible and says so; it is never undone approximately.
- **AX-003** — *Transient stays transient.* Better is more interaction state kept out of the model document and the history. Engineering data never lives only in transient state.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-060 OBJ-013 | REQ-001 | AC-001 | VER-001 | Owner ruling: decomposition AB-00-05; the criterion states the expected result, an unchanged hash and unchanged run records |
| OUT-001 | SOW-060 OBJ-013 | REQ-002 | AC-002 | VER-002 | Owner ruling: `docs/PRD.md` §5.3 and decomposition AB-00-05; the comparison is the model against the operation history |
| OUT-001 | SOW-060 OBJ-013 | REQ-003 | AC-003 | VER-001 | Owner ruling: `docs/PRD.md` FR-GUI-005; the criterion states the expected result, hash equality with the model before and after the edit |
| OUT-001 | SOW-060 OBJ-013 | REQ-004 | AC-004 | VER-003 | Owner ruling: decomposition AB-00-05 and the allocation of tree and inspector behaviour to DEL-07-02; the journey states what must be selected |
| OUT-001 | SOW-060 OBJ-013 | REQ-005 | AC-005 | VER-003 | Owner ruling: decomposition AB-00-06 and `docs/SPEC.md` §8 name the six classes; the comparison is the class in the envelope |
