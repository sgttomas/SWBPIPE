# Design brief V1.2 — SWB Piping Designer human interface

Status: V1.2 recorded by ROOT (HELP_HUMAN) on 2026-09-17 from the owner's direction answers in §1.3; the verbatim exchange and the consequences are at [`DIRECTION_DECISION_2026-09-17.md`](DIRECTION_DECISION_2026-09-17.md). V1.1 was ACCEPTED by the owner in session on 2026-09-17 with one correction and remains the accepted basis until the owner confirms the V1.2 transcription; the V1.2 changes are listed in §10. Written from the owner's answers below and the research returns A to D. It authorizes design work in phases 2 to 5 and no implementation.

## 1. Owner answers

### 1.1 The seven opening questions

In-session transcription of the owner's answers to the seven opening questions, 2026-09-17:

> 1) What was the session that made you say "I don't like it"? What were you trying to do, and where did it fall apart?
> ANSWER: I have said from the very beginning that it was terrible.  I have never once liked it.  It's a terrible interface for humans, but it was working for building the app's physics engine and connecting it to an interface so I never bothered to do anything about it, until recently.
> 2) Who is the one user whose delight matters most: you running a real analysis, a stress engineer at a firm deciding whether to trust it, or a layout designer routing pipe?
> ANSWER: a stress engineer running a real analysis because they feel comfortable with the interface.
> 3) Model-first or table-first? Should that person live in the 3D canvas, or in a grid with 3D as verification?
> ANSWER: Both should be available depending on what the user want to use as the primary means.
> 4) What is the bar? Two or three products in any domain that feel like what this should feel like, and which piping tools you respect versus want to beat.
> ANSWER: CAEPIPE, Aspect Pipe Stress (formerly CAESAR II).
> 5) Where does the agent live in the UI? A visible collaborator with its own panel, or an invisible actor whose proposals arrive as ordinary reviewed changes?
> ANSWER: a visible collaborator with its own panel, in fact, maybe
> 6) Governance on the surface: how much of the boundary, provenance and hash apparatus must stay visible at all times versus on demand? Is "Technical preview" a permanent footer?
> ANSWER: This is far far far too prominent.  It was useful during development, particularly when I was doing this months ago with weaker agents, because it kept the focus on the manner of product I wanted to build and conduct in building it.  But now it's a distraction, the user won't normally every want to see that information, and the "Technical Preview" language does not need to exist.
> 7) Platform: macOS native window first? Minimum window size you care about? Dark mode? Keyboard-first?
> ANSWER: macOS first yes, and pick a reasonable window size.  Yes darkmode, no to keyboard-first.

### 1.2 Acceptance of V1 and the seven follow-up questions

The owner's acceptance of the V1 draft, with its correction, and the answers to the V1 open questions, 2026-09-17:

> when you say "nothing requires the keyboard" that's not strictly true, because the user could be doing plenty of data entry with the keyboard.  Otherwise this is good.

> 1. Finish answer 5: "a visible collaborator with its own panel, in fact, maybe" what?
>    ANSWER:  my mistake I forgot to delete the "in fact, maybe".  I had nothing further to add.
> 2. Product name: `openpipestress.com` serves a separate open-source project's documentation. Keep the name and accept the collision, or rename before wordmark and design-system work?
>    ANSWER: The name change is "SWBPIPE" for short, or "SWB Piping Designer" in long form.  swbpipe.com and swbpipe.org are both available and affordable.
> 3. Which output tables are contractual deliverables on the projects you have in mind, and in what order do reviewers read them?
>    ANSWER: all the standard ones you would expect.  Outputs should look familiar to our target use: stress engineer who knows the target apps.
> 4. Load cases: generated from the rule pack and shown as editable rows, as proposed, or authored by hand with generation as a helper?
>    ANSWER: both should be possible, if the rows are editable, then one can just "edit" all the rows from scratch so it seems both forms of behaviour should be able to be expressed through the same primitives with just different paths selected.
> 5. Is a generated stress isometric in scope for the first design, and if so which markup is mandatory?
>    ANSWER: no, not necessary at this stage.  That should be something that is still in the future.
> 6. Hanger design: in scope with user-supplied vendor tables, or out of scope for this design?
>    ANSWER: in scope
> 7. Run comparison: part of the first design, or a later evidence feature?
>    ANSWER: later evidence feature.

### 1.3 The direction, after the phase 2 concept return

The owner's opening statement, 2026-09-17, verbatim:

> Let's discuss this some more, because this is it's own new thing.  A surface on which humans and agents can collaborate, but the human should not feel demoted in any way or part of the process.  So the app is built around the human's needs, with the agent being able to take equivalent action at the direction of the human.  Having the model be generated from the table data allows the agent to take quick and accurate actions in changing the model without having to do 3D modelling.  Also, if the human delegates actions to the agent, needs to have an effective means of checking the work and tables with rows for nodes and branching is my design choice as the optimal format.  The run book concept is valuable if I understand it to mean the workflows across the pages of tables, not the "model that never leaves the screen".  I don't think that's useful.  Both views are useful (model and table) and having them side by side can be useful too.  But all three configurations are normal ways to work for a stress engineer.  Let's discuss this to ensure we have it nailed down.

The owner's answers to ROOT's six questions and the flagged consequence, 2026-09-17, verbatim; the questions are abbreviated to their titles here and reproduced in full in the direction record:

> 1. Row grammar. ANSWER: I am well served to adopt the CAEPIPE grammar as I also want to export files that are compatible with that program.  Yes, element type is a column.
> 2. Attachments. ANSWER: To begin with, it will be separate tables with marks that we should adopt.
> 3. Canvas authoring. ANSWER: keep the few gestures.
> 4. Layouts. ANSWER: Yes and Yes.
> 5. Delegation and checking. ANSWER: row by row approvals, but multi-row and whole batch are also possible.
> 6. Symmetry. ANSWER: agent should be able to check the human's work and provide some means of feedback without altering the tables.  This may be yet another full-screen page in the app where the agent and human work collaboratively on reviews and report writing, with the information there on-screen and edits visible (and can be hidden), showing previous iterations and current.
> Flagged consequence (loads shown on node rows). RESPONSE: Yes, node rows will be marked with such.  And views should be able to toggle between table view, model view, and mixed, for each stage, with memory switching between stages to the view state it was previously left in.

## 2. What the answers change

- **Nothing in the current interface is a starting point.** The owner never liked it; it was scaffolding for building the engine. The redesign is a new product interface over the existing engine, typed interfaces and operation model, not a cleanup of the existing shell. The audit's preserve list (A §Preserve list) names the mechanisms that carry over; nothing in its presentation does.
- **The product is a surface on which the engineer and the agent collaborate, built around the engineer's needs.** The engineer must never feel demoted in any part of the process. The agent can take equivalent action at the engineer's direction, through the same means the engineer uses, and everything the agent does is legible where the engineer works. There is no agent-private surface.
- **The tables are the model.** The 3D model is generated from the table data and holds nothing the tables do not. This is what lets the agent change the model quickly and accurately without 3D modelling, and it makes the tables the format in which delegated work is checked, which the owner names as the optimal format: rows for nodes and branching.
- **The user is a practising stress engineer, and the measure is comfort.** Comfort means the engineer recognises the working vocabulary and screen grammar of the tools they already trust: node numbers as the shared address space, one row per node in the CAEPIPE layout grammar with From, To and DX/DY/DZ, the restraint vocabulary, load cases named the way they name them, and the standard output tables. Novelty is spent only where those tools are known to hurt (D §2).
- **The bar is CAEPIPE and Aspect Pipe Stress, and export must be compatible with CAEPIPE.** CAEPIPE sets the standard for the two-window pairing of a text layout beside a live 3D view and for responsiveness, and its layout grammar is adopted so that an exported file is a projection of the tables rather than a translation (RESEARCH-E characterises the format). Aspect Pipe Stress sets the standard for element input fluency, the explicit load case editor and the output processor. The redesign must feel native to a veteran of either on day one and must beat both where their users complain: length-only geometry entry, numeric-only node IDs with order-dependent propagation, lost undo, cryptic error gates, per-machine configuration, and grids that hide.
- **Two working surfaces of equal rank, one source of truth.** The tables and the 3D model are both first-class working surfaces and both are normal ways to work. Table view, Model view and Mixed view are available for every stage and remembered per stage; the canvas is not a fixture that never leaves the screen. Every edit in either surface is the same row operation; the canvas keeps a few gestures that are row shortcuts.
- **The workflow is stages of tables.** Model, Loads, Results and Review are pages of tables in the order the work is done and read. Stages are not modes: one selection, one undo, one issues list, one Run button. The report is written on the Review page and produced from it.
- **The agent is a visible collaborator with its own panel, and its work is checkable row by row.** Its proposals stay drafts shown as diffs until the engineer accepts them, as the accepted framework requires (constraints M-14, C §1.7). Acceptance is row by row, with multi-row and whole-batch acceptance available. Every row and cell shows its origin and when it was set; the engineer can mark a row as checked. The agent can check the engineer's work and give feedback without altering the tables, on the Review page. The panel is where the collaboration is legible; the tables, the model and the run stay the hero (D §6).
- **Governance leaves the surface.** The boundary, provenance and hash apparatus moves to on-demand places: a provenance inspector for any entity or run, a run record view, and reports. Product surfaces stop repeating the boundary sentence. The "Technical preview" footer goes. This last item is a governance change, not only a design one; see §6.
- **The product is SWB Piping Designer, SWBPIPE for short.** The wordmark, window title, report headers and registered boundary sentences take the new name; see §6 for the governance consequence.
- **macOS first, mouse driven, keyboard fluent for data entry, dark mode a peer of light.** "Not keyboard-first" means no command-line or modal-key interaction model is required to operate the product. It does not mean the keyboard is secondary: element entry, coordinate and value editing, spreadsheet paste and load-case editing are keyboard work, and the tables and every form must be fully operable from the keyboard with Tab, Enter and arrow-key movement in the way a spreadsheet user expects. Accelerators and the palette remain for fluency.

## 3. Design principles

1. **One spine.** One model, one selection, one issues list, one run. Tables, model, inspector, issues, results and the agent panel all reflect the same selection and the same revision.
2. **Tables and model are the page.** Everything else is a drawer or an inspector. In Table view the tables have the whole width; in Model and Mixed views the canvas is never a postage stamp.
3. **Familiar first, better second.** Use the vocabulary and screen grammar of CAEPIPE and Aspect Pipe Stress wherever they are good. Depart only where their users are known to suffer, and make the departure obvious and optional.
4. **Explicit over implicit.** Restraints drawn as what they do. Propagated values marked. Gaps, friction and directions visible in the model, not encoded in codes.
5. **Honest, located feedback, no modal gates.** Issues are always live, each one points at the entity, and running is a button, not a screen.
6. **Tables are first-class results.** Sortable, filterable, conditionally formatted, units in headers, with the model as the picture beside them, never the other way round.
7. **Provenance on demand, never as wallpaper.** Any value, entity or run can show where it came from and what it is bound to, in one place, when asked.
8. **Calm professional density.** Type, spacing and colour of a CAD or CAE tool, not a web form. Tabular numerals. A colour system with semantic tokens, in light and dark.
9. **Ratios and statuses are information, not approval.** A stress ratio shows the number, the rule and the pack version. Nothing in the interface reads as certification or code compliance (C §1.1).
10. **One set of primitives, several paths.** Generated and hand-authored load cases, pointer-placed and typed geometry, agent-proposed and user-made edits are the same rows and the same operations reached by different paths, never parallel systems.
11. **The tables are the model.** The 3D view is generated from the tables and holds nothing they do not. Nothing can be authored in the canvas that is not a row, and nothing the agent does is invisible in the tables.
12. **Equivalent action, visible and checkable.** The engineer and the agent act through the same row operations. Agent work lands as marked proposals the engineer accepts row by row; every row and cell carries its origin; the engineer's checks and the agent's checks are visible without either party touching the other's standing.

## 4. The product shape

**Window.** Default 1440 × 900 on macOS, minimum 1280 × 800. The existing 1024 × 768 layout target survives only as a regression guard for the current shell and does not constrain the redesign.

**Stages.** Four pages of tables on a rail, in the order the work is done and read; the rail is navigation, not modes:

| Stage | Tables | Also here |
|---|---|---|
| Model | Layout table (node rows), Restraints, Node data (components, SIFs, flanges) | The canvas in Model and Mixed views; inspector; issues |
| Loads | Load cases, Loads (forces, moments, displacements per node), wind and seismic when in scope | Loads of the selected case highlighted in the canvas |
| Results | Summary, Stresses, Displacements, Restraint loads, Restraint summary, Forces and moments, Hangers | Result colour, probe and legend in the canvas; the run identity |
| Review | Review of the engineer's work by the agent; report outline and draft with iterations | Report produced from here; export and handoff |

**Views.** Three views of every stage, switchable at any time and remembered per stage, so that switching stages restores the view that stage was last left in. All three are normal ways to work; none is the posture of the app:

| View | What is on screen | For |
|---|---|---|
| Table | The stage's tables at full width, no canvas | Volume input, checking rows, reading results |
| Model | The canvas, with the stage's current table as a drawer and the inspector | Routing, reviewing on the model, results on the model |
| Mixed | Tables and canvas side by side | The CAEPIPE pairing |

**Surfaces.**

- *Layout table.* The CAEPIPE grammar: one row per node, carrying the element that arrives at it. From is shown on every row and defaults to the previous row's node; a branch is a row whose From names an earlier node; element type (pipe, bend, valve, reducer, tee and the other types the engine supports) is a column; then DX, DY, DZ or absolute coordinates as a switchable column group, section, material, temperatures, pressures. Each attachment class the node has (restraint, load, node data) shows as a mark on the row that opens the joined row. Values propagate down by connectivity and are visibly marked as propagated. Paste from a spreadsheet with a mapping preview, alphanumeric node IDs with auto-increment, fully keyboard operable in the spreadsheet idiom. Field names follow CAEPIPE's where the engine has the same concept, so that export is a projection.
- *Attachment tables.* Restraints, loads and node data are separate tables joined on the node, in the shape the target user expects (per node: type, direction, gap, friction, stiffness, connecting node; per load: kind, direction, value with unit). Adding a row in an attachment table marks the node row; the mark opens the row. Separate tables to begin with, by the owner's decision.
- *3D model.* Generated from the tables. Real outside diameter by default, fittings and components with recognisable geometry, restraints as glyphs that show direction, gap and friction, loads as scaled vectors, node labels with a budget that prioritises selection and hover. Selection and cursor sync with the tables in both directions; a probe that reads values at the cursor. A few authoring gestures that are row shortcuts: direct distance entry with a routing compass, which writes a layout row; click a node to add a restraint row. Fit, presets, section views, isolate and hide. Proposed changes appear as ghosts.
- *Inspector.* Contextual properties of the selection with units on every field, required fields marked, provenance behind a disclosure.
- *Issues.* One live list, counts in the status bar, each issue pointing at its entity, classed with the accepted warning taxonomy (M-10) in plain words.
- *Load cases.* One editable, attributable table of cases and combinations in the Aspect Pipe Stress editor's shape. Generation from the project's rule pack fills rows the engineer can edit or delete; an empty table authored by hand uses the same rows. Each row records whether it was generated, edited or authored.
- *Hanger design.* In scope. The two-pass ritual made explicit: a design pass that finds the load at each hanger location, then selection against the user's own vendor tables supplied as a library with provenance, output as a hanger table. No vendor data ships with the product (M-05).
- *Run.* One button; progress and failure reasons on screen; each run immutable and named, with a run record on demand.
- *Results.* Tables first, in the standard set the target user expects: summary, displacements, restraint loads and restraint summary, element forces and moments, stresses with ratio, rule ID and pack version, per case and enveloped, and the hanger table. Then colour on the model with probes and a legend, then animated deflection.
- *Origins and the checked mark.* Every row and cell carries its origin (entered by the engineer, agent proposal accepted by the engineer, propagated, generated by a rule pack, imported) and when. The engineer can mark a row as checked: a human act recorded as a tag bound to the row's content, never a software status, lapsing visibly when the row changes. Filters show unchecked rows, agent-origin rows and rows changed since a chosen point.
- *Agent panel.* A visible collaborator: conversation, the queue of proposals as diffs with rationale, accept or reject per row with multi-row and whole-batch acceptance, and a record of what was accepted. Proposals land in the tables as marked proposed rows and cells with old and new values, and in the canvas as ghosts.
- *Review page.* A full page where the engineer and the agent work on reviews and on the report together. The agent's feedback on the engineer's work is attached by reference to rows, results and report text and never alters the tables; it is limited to checks and open issues. Report outline and draft, with edits visible or hidden, and previous iterations retained beside the current one.
- *Reports, export and handoff.* Generated documents carry the required notices (M-03 to M-07); the interface does not. Export to a CAEPIPE-compatible file is a projection of the tables (format per RESEARCH-E); what the export is called before compatibility evidence exists is a decision-packet item.

**Deferred beyond this design.** Generated stress isometrics. Run comparison as an evidence feature. Import of CAEPIPE files is not assumed by this brief. All are kept out of the mock states and the specification; the data model must not preclude them.

**Command surfaces.** A menu bar, a single toolbar per surface, a command palette. The five parallel command systems of the current shell collapse to these three.

## 5. Vocabulary and identity

- Product: SWB Piping Designer; SWBPIPE in short form, window titles and file badges.
- Node IDs: stable, human-editable, alphanumeric, default numeric increment of 10, insert and split preserve neighbours, import keeps source IDs.
- Rows: one row per node with the element that arrives at it, From explicit; the engineer's sentence "from 10 to 20, 3 m in X" is the mental model; a branch is a row whose From points back.
- Stages: Model, Loads, Results, Review. Views: Table, Model, Mixed.
- Restraints: anchor, +Y, guide, limit stop, gap, friction, connecting node; rendered graphically.
- Load cases: the engineer's own naming (W, T1, P1, OPE, SUS, EXP, OCC and combinations), with the generating rule attributed.
- Origins: entered, accepted from a proposal, propagated, generated, imported. Proposals are accepted or rejected, never approved. A row the engineer has checked carries a checked mark; the word is a tag, not a status.
- Statuses: the accepted authority-attributed vocabulary (M-08), presented as short labels with the raw status one click away.

## 6. Constraints the design honours, and the items it must send back

Honoured by design: the claims boundary (C-01 to C-13, on every label); no accept or approve control on results, runs or reports (C-06); one structured-operation route for every mutation, gesture, typed row, paste or accepted proposal alike (C-74, C-75); units on every value (M-11); provenance on governed data, on demand (M-12); missing-data findings and warning classes (M-10); the historical run designation (M-13); agent output as draft and diff until the engineer accepts it (M-14, C-77), and agent feedback limited to checks and open issues; display-only rule expressions (M-15); accessibility findings as warnings, no conformance claim (M-16); the required content of reports (M-03 to M-07, M-17); no fence token or retired phrases on product surfaces (C-18, C-19); no shipped vendor or standards content (M-05).

Sent back to the owner as one decision packet, prepared with the confirmed direction so that design and governance move together:

1. **The maturity line.** "Technical preview — not a released product." is a registered boundary statement bound to the app shell by the claims registry and anchored by the claims-language lint (C-15, C-20, M-01). The owner's direction that the language need not exist requires revising the registry entry and the lint anchor.
2. **Boundary sentence placement.** The registry binds the acceptance sentence (C-14, M-02) to results, rule-check, comparison and solve surfaces. The design proposes one placement per surface class, in the report, in a results header disclosure and on the Review page, rather than the current fifty.
3. **The product name.** Registered sentences and the required report notice carry "OpenPipeStress" (M-03, M-05, C-17), as do the repository, package and document titles. The rename to SWB Piping Designer changes those registered texts and their lint anchors, and the packet must state the exact replacement sentences.
4. to 6. The concept return's items: short status labels as listed variants, vendor hanger tables as a library class, reuse of the historical-run wording ([`../CONCEPTS/CONCEPT_DIRECTIONS_V1.md`](../CONCEPTS/CONCEPT_DIRECTIONS_V1.md) §6).
7. to 9. The direction record's items: the checked mark's standing, the wording envelope for agent feedback on the Review page and that page as a surface class, and what the export is called before compatibility evidence exists ([`DIRECTION_DECISION_2026-09-17.md`](DIRECTION_DECISION_2026-09-17.md) §6).

## 7. Mock states

The mock drafts cover these workflow states. Each is drafted in the view it is most naturally worked in and in one other view, in light and dark; states 4 and 7 are also drafted in Mixed view.

1. New project, empty model, the first node row (Table).
2. Routing in the canvas with direct distance entry, the rows appearing as the pipe is drawn (Model).
3. Editing the layout table with propagation, keyboard entry and a paste from a spreadsheet (Table).
4. Restraint and load tables, marks on the node rows, glyphs on the model (Mixed).
5. Load cases: generated rows edited, and a hand-authored case beside them (Table).
6. Running, with live issues and a failure reason.
7. Results: stress table with ratios, model colour, probe and legend (Mixed).
8. Hanger design against a user-supplied vendor table, and an agent proposal landing as proposed rows with old and new values, accepted row by row.
9. Review page: the agent's checks on the engineer's work attached by reference, the report draft with a previous and a current iteration, edits shown and hidden.

## 8. Questions resolved on 2026-09-17

All seven V1 open questions are answered in §1.2 and applied above: answer 5 stands as "a visible collaborator with its own panel"; the product is SWB Piping Designer; outputs are the standard set in familiar form; load cases share primitives across generated and authored paths; stress isometrics and run comparison are deferred; hanger design is in scope. The direction questions of §1.3 are applied as V1.2; four readings ROOT took in applying them are listed for the owner's word in the direction record §8.

## 9. What happens next

Once the owner confirms the V1.2 transcription, ROOT seals phase 3 briefs: the design system (tokens, type, colour in light and dark, iconography, density, the marks vocabulary for origins, proposals and checks, and the 3D presentation language) and the mock drafts of states 1 to 9, using Fable children for the qualitative work. RESEARCH-E's return on CAEPIPE's exchange format feeds the layout table's columns and names before the mocks are drafted. The rendering brief owed to the piping session under D-70 is cut from §4's model surface. The decision packet in §6 is prepared alongside.

## 10. Change log

V1.1 to V1.2, 2026-09-17, from the owner's direction answers (§1.3): §1.3 added; §2 rewritten for the collaboration surface, the tables as the model, the CAEPIPE grammar and export, views and stages, and row-by-row acceptance; §3 principle 2 rewritten, principles 11 and 12 added; §4 layouts replaced by stages and views, layout table rewritten to node rows, attachment tables, origins and the checked mark, the Review page and export added; §5 rows, stages, views and origins added; §6 items 4 to 9 referenced; §7 rewritten with views and state 9; §8 and §9 restated. V1 to V1.1: the keyboard correction and the seven follow-up answers of §1.2.
