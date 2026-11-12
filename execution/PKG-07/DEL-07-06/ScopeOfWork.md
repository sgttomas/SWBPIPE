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

This deliverable is the baseline every desktop surface is held to so that an engineer can review a model and its results without a pointer and without relying on colour. It sets the measurable accessibility target, the presentation conventions that make the surfaces read as one workspace (type, spacing, control sizes, states, themes and density), and the use cases, typical of an engineer's work, that the desktop completes from start to finish with no step outside it. It owns no panel's content. The result it contributes is a desktop in which model creation, missing data, results, assumptions, provenance and findings can all be reached, read and acted on by keyboard and assistive technology, at a stated standard, and in which an engineer can carry typical work through to its result without leaving the application.

It serves SOW-036 under OBJ-006 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-07, row DEL-07-06), and PRD §14.1 and §14.4. `docs/RELEASE_TARGET_v1.0.md` names this deliverable an owner of exit criterion 8 (usability) and, with DEL-07-02, of exit criterion 7 (authoring time).

- **OUT-001** — The accessibility target and the presentation conventions, with the design tokens, themes and density that carry them.
- **OUT-002** — The use cases an engineer typically goes through, each completing in the desktop with no step outside it.

## Deliverable Definition — Ontology

**Terms the baseline implements.**

- *Workspace control.* Any control, status indicator or drawn mark of the desktop workspace that a user reads or operates: buttons, fields, tabs, tree and table rows, menus, badges, legends and the viewport's selectable items.
- *Accessibility target.* The WCAG 2.2 level AA criteria selected for the controls of the desktop's modelling workspace: the command band and its search, the model tree, the properties and active-task panels with their side-rail drawers, the viewport's camera, selection, visibility and measurement controls, the review and results dock, and the theme and density settings. They include Keyboard (2.1.1), Focus Visible (2.4.7), Contrast (Minimum) (1.4.3) at 4.5:1 for text, Non-text Contrast (1.4.11) at 3:1 for graphical objects and control boundaries, and Target Size (Minimum) (2.5.8) at 24 by 24 CSS pixels, applied as that criterion states it.
- *Design token.* A named value for a colour, a size, a space or a type step. One token source generates the stylesheet every surface uses.
- *Theme.* Light or dark, following the system setting unless the user chooses one. *Density.* Comfortable or compact.
- *Presentation conventions.* What every surface shares: type, spacing and control size taken from one scale; tabular figures in tables; hover, pressed, selected, focus and disabled states that are distinct, each with a signal other than colour; and a comfortable and a compact density.
- *Use case.* A piece of work an engineer typically does with the product, from a stated start to a stated result. A use case *completes in the desktop* when a person reaches its result using only the desktop application and the system dialogs it opens to choose a file or a printer. A terminal, another application, a file edited by hand and a step known only from the code are each a *step outside* it.
- *Owner's verdict.* The owner's yes or no on whether the desktop is ready, given after using it.
- *User preference.* The chosen theme and density. It belongs to the user and the machine and is never written to the model document.

**Boundary.** The baseline sets the standard and the shared means. The following belong elsewhere:

- each panel's content, labels and keyboard commands: the deliverable that owns the panel. Each meets this baseline for its own surface;
- the shell that mounts the panels and carries focus between them: DEL-07-11;
- how a finding's class and fields are shown: DEL-07-04;
- the interaction-performance limits of the workspace: DEL-07-01/REQ-012;
- the definition of the authoring time, its threshold and its measurement: DEL-07-02/REQ-019. The timed entry is one of the use cases here only as work that completes in the desktop;
- the capability each use case exercises, and whether its numbers are correct: the deliverable that owns the capability. A use case that cannot complete because a capability is absent or refuses is that deliverable's shortfall, shown here;
- the wording of statuses and notices: DEL-01-04.

**Not decided.** The accessibility target for the report preview and export surfaces and for generated report files.

**Windows.** Windows is not a release platform for v1.0. Where the desktop is built for Windows, its native user experience is examined in the launched application with its webview: native dialogs and menus, Ctrl accelerators, focus, display scaling across monitors, the 3D view's graphics, printing, and save and reopen. A run in a browser alone does not show it, and a Windows check of another native boundary does not meet it.

## Completion and Reliance Basis — Epistemology

The project's own examination, VER-006's checks among it, shows whether the use cases complete in the desktop. It is not an independent usability validation, and the owner's verdict is the owner's act, not that validation. The desktop's usability is not presented or relied on as independently validated until user testing, which the owner arranges, has examined it. Only the owner releases this restriction.

### Requirements

- **REQ-001** — Model creation, missing data, results, assumptions, provenance and findings are each visible for review on a surface a user can reach without a pointer.
- **REQ-002** — Every panel and every review workflow of the workspace can be reached and operated by keyboard alone, with visible focus and a focus order that follows the reading order.
- **REQ-003** — Every icon action, status indicator and compact control has a text label or a tooltip and an accessible name that says what it does or states.
- **REQ-004** — The controls the accessibility target covers meet it in every theme and density.
- **REQ-005** — Each large tree and results table offers search or filtering, and a way to copy what it shows, operable by keyboard.
- **REQ-006** — Inline validation shows the class of each finding as DEL-07-04 presents it, with the six classes distinguishable without colour.
- **REQ-007** — A displayed dimensional value shows its unit.
- **REQ-008** — Undo and redo are offered only where a reversible model edit exists, and say what they will undo or redo. Findings remain visible through undo and redo.
- **REQ-010** — No text the baseline supplies, such as a shared label or a preference control, states or implies certification, sealing, approval, authentication or code compliance (PRD §21.2).
- **REQ-011** — Colour is never the only signal of a state, a class, a status or a selection.
- **REQ-012** — The workspace offers a light and a dark theme that follow the system setting unless the user chooses one, and a comfortable and a compact density. The choice is a user preference.
- **REQ-013** — One token source generates the stylesheet. No workspace surface sets a colour, a space, a control size or a type step outside the tokens.
- **REQ-014** — Every workspace surface follows the presentation conventions, and in either density every target of a control the accessibility target covers meets Target Size (Minimum) (2.5.8).
- **REQ-016** — *Usability.* Each use case AC-007 lists completes in the desktop with no step outside it, and each is tested thoroughly before the owner is asked. Before the owner is asked, the embedded agent completes each use case in the desktop from its stated start to its stated result. The owner then uses the desktop and gives the owner's verdict (`docs/RELEASE_TARGET_v1.0.md`, exit criterion 8).

### Acceptance criteria

- **AC-001** — Starting from the first focusable control, every panel of the workspace and every step of creating a model and reviewing missing data, results, assumptions, provenance and findings can be completed by keyboard alone, with focus visible and moving in reading order.
- **AC-002** — In each theme and each density, for the controls the accessibility target covers: every control's text contrast is at least 4.5:1, every graphical object's and control boundary's contrast is at least 3:1, and every target meets Target Size (Minimum) (2.5.8).
- **AC-003** — Every icon action, status indicator and compact control has an accessible name; every state, class, status and selection has a signal other than colour; every displayed dimensional value has a unit; and no text the baseline supplies states or implies a claim PRD §21.2 prohibits.
- **AC-004** — Undo and redo are enabled only when a reversible edit exists and name the edit they will reverse or reapply, and after each undo and redo the findings shown are those of the model as it then stands.
- **AC-005** — The stylesheet equals the one generated from the token source; the theme follows the system setting and the user's choice; switching density changes control size; and the model document is unchanged by any preference.
- **AC-006** — Each large tree and results table can be searched or filtered, and its content copied, by keyboard.
- **AC-007** — Each use case below completes in the desktop with no step outside it, on invented models, and the owner's verdict is yes.
  - *Blank to released report.* From a blank project the user chooses units, enters a piping system with its components and supports, defines load cases and combinations, solves, reads the results, and releases, saves and prints a report that carries the PRD §19.3 notice.
  - *Timed entry.* The entry DEL-07-02/REQ-019 defines is made from a blank project to a completed solve without leaving the desktop.
  - *Missing data.* The user opens a model that lacks values a solve requires. Every missing value is shown as a finding on its entity; from each finding the user reaches the entity and enters the value; and the solve, refused before, then completes.
  - *Failed solve.* The user solves a model that the solver refuses or cannot solve. The desktop says why, in findings that name the entities concerned; the user reaches each entity from its finding and corrects the model; and the solve then completes.
  - *Change a support and compare.* On a solved model the user saves the model state, changes a support, saves the new state and solves again, then selects the two states and their runs, reads the differences in inputs and results in a table and in the viewport, and exports the comparison report.
  - *Review and export results.* On the solved reference system the user finds, for a chosen case or combination, the largest displacement, the largest support load and the terminal loads by sorting and filtering; sees each located in the viewport; reads its unit and the assumptions and provenance behind it; and exports the table to CSV.
  - *Every required analysis.* On the reference system the user sets up and solves each analysis exit criterion 1 names and reads each result, among them the hydrotest marked as a test case, the modal frequencies and mode shapes, the spectrum response and the hanger table.
  - *User rule pack.* The user brings a private rule pack into the local store through the rule-pack editor, selects it, applies it to an analysis run and reads each outcome as within or exceeding the user's limit, with each rule input the model lacks shown as a finding.
  - *Save, close and resume.* The user saves a project, quits, reopens it and finds the model and its saved states, runs and reports as they were left. After the desktop is ended without saving, reopening it recovers the work with at most one autosave interval lost.

## Production and Verification Method — Praxeology

**Method choices.** The tokens are held in one file and the stylesheet is generated from it, so a check can compare the two. Contrast and target size are read from the rendered desktop in each theme and density, not from the token values alone, so that a surface that departs from the tokens is caught. When the embedded agent carries out a use case, it acts through its capability catalog (DEL-16-06): it reads, proposes operations, starts jobs and requests acts. It does not use the desktop's controls. The catalog is compared with what each use case needs from its stated start to its stated result, to show whether it reaches it; the comparison is of reach, so one tool call may cover several steps a person takes in the desktop. The stated start of a use case is set by the test arrangement: it creates the blank project or opens the model or the stored project, and where a use case ends and restarts the application, or resumes after the desktop ended without saving, it relaunches the application. It does these through the product's own new, open and recovery, and it never supplies the state a use case is meant to find: what is found after a relaunch is what the product itself saved or recovered. The agent is started on the workspace that results; it does not create, open or restart one. Four things stay a person's act in every use case: Apply on a proposal; each person-entered value the use case states, a private rule pack among them, since the pack is the user's own; the act each request asks for; and the selection and statement of a user release. In these runs only, the test harness does each of them, through the desktop's own controls, so that the runs need no one attending. In the rule pack use case that means the harness, standing for the person, brings the pack into the local store through the rule-pack editor; the agent then selects it, applies it to the run and reads the outcomes, and writes no part of the pack. The product's rule is unchanged: a person applies, performs each requested act and releases (DEL-16-06; DEL-08-01 REQ-019). A run in which a scripted provider stands in for the model examines the route only; it does not show that the embedded agent completes a use case.

**How each criterion is examined.**

- **VER-001** — Executable check (observes the running desktop by keyboard): walks the workspace and the creation and review workflows by keyboard alone and reads the focused element and its focus indicator at each step.
- **VER-002** — Comparison with a named reference (the WCAG 2.2 level AA thresholds of the accessibility target): computes text and non-text contrast and target sizes from the rendered desktop in each theme and density and compares them with the thresholds.
- **VER-003** — Executable check (observes the accessibility tree of every workspace surface): reads each control's accessible name, each state's non-colour signal, each value's unit, and the state and name of undo and redo with and without a reversible edit, with the findings shown after each step.
- **VER-004** — Executable check (observes the token source, the stylesheet and the preference): regenerates the stylesheet and compares it with the one in use, switches theme and density and reads the applied values, and compares the model document's bytes before and after.
- **VER-005** — Executable check (observes each large tree and table by keyboard): searches, filters and copies, and reads the result.
- **VER-006** — Executable check (drives the running desktop through each use case of AC-007, one check per use case, from its stated start to its stated result using only the desktop's own controls, and observes the result the use case names): a use case fails when any step needs something outside the desktop. For printing the check observes that the desktop hands the report to the system print dialog, which is a proxy for the printed page. For the timed entry it observes that the entry completes; the time is examined under DEL-07-02/VER-007. Each use case is also carried out by the embedded agent, and its result is compared with the result the use case names. And judgment by a named role (the owner uses the desktop and answers yes or no on whether it is ready), asked only when every check passes.

## Governing Values and Decisions — Axiology

- **AX-001** — The standard is measured, not asserted. Better is a contrast ratio, a target size and a keyboard path read from the rendered desktop, never a statement that a surface is accessible.
- **AX-002** — One convention serves every surface. Better is a surface that takes its values from the tokens, measured by the count of values set outside them being zero, never a panel with a scale of its own.
- **AX-003** — A control that cannot act says why. Better is a stated reason at the control, never a silent no-op or a hidden command.
- **AX-004** — Usability is judged by use. Better is a use case carried from its start to its result in the running desktop, measured by the count of use cases that need a step outside it, which is to be zero. It is never a checklist of surfaces each examined alone, and a use case is never made to pass by narrowing what it asks of the desktop.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-036 OBJ-006 | REQ-001 REQ-002 | AC-001 | VER-001 | Outside source: WCAG 2.2 success criteria 2.1.1 and 2.4.7, which the accessibility target includes. |
| OUT-001 | SOW-036 OBJ-006 | REQ-004 REQ-014 | AC-002 | VER-002 | Outside source: WCAG 2.2 success criteria 1.4.3, 1.4.11 and 2.5.8, with their published thresholds. |
| OUT-001 | SOW-036 OBJ-006 | REQ-003 REQ-006 REQ-007 REQ-010 REQ-011 | AC-003 | VER-003 | The accessibility target, PRD §14.4 and PRD §21.2. The expected names and signals are those each surface's own scope states. |
| OUT-001 | SOW-036 OBJ-006 | REQ-008 | AC-004 | VER-003 | SOW-036 in the decomposition and PRD FR-GUI-005 for undo and redo. The expected state of undo and redo follows from whether the session holds a reversible edit, and the expected findings are those computed fresh for the model at each step. |
| OUT-001 | SOW-036 OBJ-006 | REQ-012 REQ-013 | AC-005 | VER-004 | The token source is the reference for the stylesheet and for the applied values. This guards against a surface departing from the tokens; whether the tokens meet the target is AC-002's to show. |
| OUT-001 | SOW-036 OBJ-006 | REQ-005 | AC-006 | VER-005 | SOW-036 in the decomposition, whose cited source asks for search and filter in large model trees and copy from result tables. The check exercises each tree and table the workspace has. |
| OUT-002 | SOW-036 OBJ-006 | REQ-016 | AC-007 | VER-006 | Exit criterion 8 of the release target, and the owner's own yes or no. The use cases are drawn from PRD §10 and the release target's exit criteria. The expected result of each step is the one the owning deliverable's scope states; this criterion shows that the work completes in the desktop, not that its numbers are correct, and it is not an independent usability validation. |
