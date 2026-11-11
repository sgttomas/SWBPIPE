---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-07-06
package_id: PKG-07
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-036]
package_objective_refs: [OBJ-006]
---

# Scope of Work — DEL-07-06 Accessibility and usability baseline

## Purpose and Objective Traceability

This deliverable is the baseline every desktop surface is held to so that an engineer can review a model and its results without a pointer and without relying on colour. It sets the measurable accessibility target, the presentation conventions that make the surfaces read as one workspace (type, spacing, control sizes, states, themes and density), and the usability walkthrough by which the workspace is judged fit for engineering review. It owns no panel's content. The result it contributes is a desktop in which model creation, missing data, results, assumptions, provenance and findings can all be reached, read and acted on by keyboard and assistive technology, at a stated standard.

It serves SOW-036 under OBJ-006 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-07, row DEL-07-06), and PRD §14.1 and §14.4. `docs/RELEASE_TARGET_v1.0.md` names this deliverable an owner of exit criterion 8 (usability) and, with DEL-07-02, of exit criterion 7 (authoring time).

- **OUT-001** — The accessibility target and the presentation conventions, with the design tokens, themes and density that carry them.
- **OUT-002** — The usability walkthrough: its use cases, findings and dispositions.

## Deliverable Definition — Ontology

**Terms the baseline implements.**

- *Workspace control.* Any control, status indicator or drawn mark of the desktop workspace that a user reads or operates: buttons, fields, tabs, tree and table rows, menus, badges, legends and the viewport's selectable items.
- *Accessibility target.* The WCAG 2.2 level AA criteria the owner selected for workspace controls. They include Keyboard (2.1.1), Focus Visible (2.4.7), Contrast (Minimum) (1.4.3) at 4.5:1 for text, Non-text Contrast (1.4.11) at 3:1 for graphical objects and control boundaries, and Target Size (Minimum) (2.5.8) at 24 by 24 CSS pixels, applied as that criterion states it.
- *Design token.* A named value for a colour, a size, a space or a type step. One token source generates the stylesheet every surface uses.
- *Theme.* Light or dark, following the system setting unless the user chooses one. *Density.* Comfortable or compact.
- *Presentation conventions.* What every surface shares: type, spacing and control size taken from one scale; tabular figures in tables; hover, pressed, selected, focus and disabled states that are distinct, each with a signal other than colour; and a comfortable and a compact density.
- *Use-case walkthrough.* A recorded pass through the desktop by named use cases, each with what was attempted, what was found and how each finding was dispositioned. A *must-fix finding* is one the walkthrough's dispositions mark as blocking release.
- *User preference.* The chosen theme and density. It belongs to the user and the machine and is never written to the model document.

**Boundary.** The baseline sets the standard and the shared means. The following belong elsewhere:

- each panel's content, labels and keyboard commands: the deliverable that owns the panel. Each meets this baseline for its own surface;
- the shell that mounts the panels and carries focus between them: DEL-07-11;
- how a finding's class and fields are shown: DEL-07-04;
- the interaction-performance limits of the workspace: DEL-07-01/REQ-012;
- the definition of the authoring time: DEL-07-02/REQ-019. The walkthrough includes the timed entry and dispositions the usability findings it raises, under REQ-016;
- the wording of statuses and notices: DEL-01-04.

**Not decided.** The accessibility target for the report preview and export surfaces and for generated report files.

## Completion and Reliance Basis — Epistemology

### Requirements

- **REQ-001** — Model creation, missing data, results, assumptions, provenance and findings are each visible for review on a surface a user can reach without a pointer.
- **REQ-002** — Every panel and every review workflow of the workspace can be reached and operated by keyboard alone, with visible focus and a focus order that follows the reading order.
- **REQ-003** — Every icon action, status indicator and compact control has a text label or a tooltip and an accessible name that says what it does or states.
- **REQ-004** — Workspace controls meet the accessibility target in every theme and density.
- **REQ-005** — Each large tree and results table offers search or filtering, and a way to copy what it shows, operable by keyboard.
- **REQ-006** — Inline validation shows the class of each finding as DEL-07-04 presents it, with the six classes distinguishable without colour.
- **REQ-007** — A displayed dimensional value shows its unit.
- **REQ-008** — Undo and redo are offered only where a reversible model edit exists, and say what they will undo or redo. Findings remain visible through undo and redo.
- **REQ-010** — No text the baseline supplies, such as a shared label or a preference control, states or implies certification, sealing, approval, authentication or code compliance (PRD §21.2).
- **REQ-011** — Colour is never the only signal of a state, a class, a status or a selection.
- **REQ-012** — The workspace offers a light and a dark theme that follow the system setting unless the user chooses one, and a comfortable and a compact density. The choice is a user preference.
- **REQ-013** — One token source generates the stylesheet. No workspace surface sets a colour, a space, a control size or a type step outside the tokens.
- **REQ-014** — Every workspace surface follows the presentation conventions, and in either density every target meets Target Size (Minimum) (2.5.8).
- **REQ-016** — *Usability.* A use-case walkthrough of the desktop is recorded with its use cases, findings and dispositions, and its must-fix findings are closed. The walkthrough includes, as one of its use cases, the timed entry DEL-07-02/REQ-019 defines, and records the time and the usability findings it raises. Owner-arranged user testing follows, and its findings are the owner's to disposition (`docs/RELEASE_TARGET_v1.0.md`, exit criteria 7 and 8).

### Acceptance criteria

- **AC-001** — Starting from the first focusable control, every panel of the workspace and every step of creating a model and reviewing missing data, results, assumptions, provenance and findings can be completed by keyboard alone, with focus visible and moving in reading order.
- **AC-002** — In each theme and each density, every workspace control's text contrast is at least 4.5:1, every graphical object's and control boundary's contrast is at least 3:1, and every target meets Target Size (Minimum) (2.5.8).
- **AC-003** — Every icon action, status indicator and compact control has an accessible name; every state, class, status and selection has a signal other than colour; every displayed dimensional value has a unit; and no text the baseline supplies states or implies a claim PRD §21.2 prohibits.
- **AC-004** — Undo and redo are enabled only when a reversible edit exists and name the edit they will reverse or reapply, and after each undo and redo the findings shown are those of the model as it then stands.
- **AC-005** — The stylesheet equals the one generated from the token source; the theme follows the system setting and the user's choice; switching density changes control size; and the model document is unchanged by any preference.
- **AC-006** — Each large tree and results table can be searched or filtered, and its content copied, by keyboard.
- **AC-007** — The walkthrough record names its use cases, findings and dispositions, includes the timed entry with its time and the findings it raised, and no must-fix finding is open.

## Production and Verification Method — Praxeology

**Method choices.** The tokens are held in one file and the stylesheet is generated from it, so a check can compare the two. Contrast and target size are read from the rendered desktop in each theme and density, not from the token values alone, so that a surface that departs from the tokens is caught.

**How each criterion is examined.**

- **VER-001** — Executable check (observes the running desktop by keyboard): walks the workspace and the creation and review workflows by keyboard alone and reads the focused element and its focus indicator at each step.
- **VER-002** — Comparison with a named reference (the WCAG 2.2 level AA thresholds of the accessibility target): computes text and non-text contrast and target sizes from the rendered desktop in each theme and density and compares them with the thresholds.
- **VER-003** — Executable check (observes the accessibility tree of every workspace surface): reads each control's accessible name, each state's non-colour signal, each value's unit, and the state and name of undo and redo with and without a reversible edit, with the findings shown after each step.
- **VER-004** — Executable check (observes the token source, the stylesheet and the preference): regenerates the stylesheet and compares it with the one in use, switches theme and density and reads the applied values, and compares the model document's bytes before and after.
- **VER-005** — Executable check (observes each large tree and table by keyboard): searches, filters and copies, and reads the result.
- **VER-006** — Judgment by a named role (Agent 0 conducts the use-case walkthrough and records it; the owner arranges user testing and dispositions its findings): reads the record for its use cases, findings and dispositions and for any open must-fix finding.

## Governing Values and Decisions — Axiology

- **AX-001** — The standard is measured, not asserted. Better is a contrast ratio, a target size and a keyboard path read from the rendered desktop, never a statement that a surface is accessible.
- **AX-002** — One convention serves every surface. Better is a surface that takes its values from the tokens, measured by the count of values set outside them being zero, never a panel with a scale of its own.
- **AX-003** — A control that cannot act says why. Better is a stated reason at the control, never a silent no-op or a hidden command.
- **AX-004** — Usability is judged by use. Better is a finding from a recorded use case with its disposition, never a checklist completed without the workflow being attempted.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-036 OBJ-006 | REQ-001 REQ-002 | AC-001 | VER-001 | Outside source: WCAG 2.2 success criteria 2.1.1 and 2.4.7, selected by owner ruling as the target. |
| OUT-001 | SOW-036 OBJ-006 | REQ-004 REQ-014 | AC-002 | VER-002 | Outside source: WCAG 2.2 success criteria 1.4.3, 1.4.11 and 2.5.8, with their published thresholds. |
| OUT-001 | SOW-036 OBJ-006 | REQ-003 REQ-006 REQ-007 REQ-010 REQ-011 | AC-003 | VER-003 | Owner ruling: the selected target, PRD §14.4 and PRD §21.2. The expected names and signals are those each surface's own scope states. |
| OUT-001 | SOW-036 OBJ-006 | REQ-008 | AC-004 | VER-003 | Owner ruling: SOW-036 in the decomposition and PRD FR-GUI-005 for undo and redo. The expected state of undo and redo follows from whether the session holds a reversible edit, and the expected findings are those computed fresh for the model at each step. |
| OUT-001 | SOW-036 OBJ-006 | REQ-012 REQ-013 | AC-005 | VER-004 | The token source is the reference for the stylesheet and for the applied values. This guards against a surface departing from the tokens; whether the tokens meet the target is AC-002's to show. |
| OUT-001 | SOW-036 OBJ-006 | REQ-005 | AC-006 | VER-005 | Owner ruling: SOW-036 in the decomposition, whose cited source asks for search and filter in large model trees and copy from result tables. The check exercises each tree and table the workspace has. |
| OUT-002 | SOW-036 OBJ-006 | REQ-016 | AC-007 | VER-006 | Owner ruling: exit criteria 7 and 8 of the release target. The judgment is Agent 0's walkthrough and the owner's disposition of user-testing findings; the timed entry is the one DEL-07-02/REQ-019 defines. |
