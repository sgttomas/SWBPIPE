# Direction decision — the owner's synthesis, 2026-09-17

Status: owner direction given in session on 2026-09-17 in two messages, transcribed verbatim in §1 from bytes extracted from the ROOT session transcript. §2 onward is ROOT's transcription of that direction into a design basis, presented to the owner for confirmation; where the transcription and the owner's words differ, the owner's words govern. This record supersedes the recommendation in [`../CONCEPTS/CONCEPT_DIRECTIONS_V1.md`](../CONCEPTS/CONCEPT_DIRECTIONS_V1.md) §5: none of the three directions is chosen as returned. The form follows [`DESIGN_BRIEF_V1.md`](DESIGN_BRIEF_V1.md) §1, the owner's words first and the consequences after, and the verbatim-extraction practice of the D-70 ruling record.

## 1. The owner's words

### 1.1 Opening message

Transcript timestamp 2026-09-18T04:49:39Z. SHA-256 of the extracted bytes: `792893d61f65fef59ece6b85f4e85c90451adb2c7dfd8d4195555b1f4b4aefc0`.

> Let's discuss this some more, because this is it's own new thing.  A surface on which humans and agents can collaborate, but the human should not feel demoted in any way or part of the process.  So the app is built around the human's needs, with the agent being able to take equivalent action at the direction of the human.  Having the model be generated from the table data allows the agent to take quick and accurate actions in changing the model without having to do 3D modelling.  Also, if the human delegates actions to the agent, needs to have an effective means of checking the work and tables with rows for nodes and branching is my design choice as the optimal format.  The run book concept is valuable if I understand it to mean the workflows across the pages of tables, not the "model that never leaves the screen".  I don't think that's useful.  Both views are useful (model and table) and having them side by side can be useful too.  But all three configurations are normal ways to work for a stress engineer.  Let's discuss this to ensure we have it nailed down.

### 1.2 Answers to ROOT's six questions

Transcript timestamp 2026-09-18T05:08:31Z. SHA-256 of the extracted bytes: `0cdb0c3d3fe5c647a723cc1436955d8e677fb4656c1e4bcd1a2af2e018bd285e`. The owner returned ROOT's questions with the answers written inline; the whole message is reproduced so the answers keep their questions.

> **Questions that decide the design:**
>
> 1. **Row grammar.** "Rows for nodes and branching" reads to me as the CAEPIPE grammar: each row is a node, carrying the element that arrives at it, with From shown explicitly and defaulting to the previous row, so a branch is simply a row whose From points back. I'd keep From always visible so a row is self-describing and order-independent, which matters for agent diffs and for paste. Is that the grammar, and is element type (pipe, bend, valve, reducer, tee) a column on that row?
> ANSWER: I am well served to adopt the CAEPIPE grammar as I also want to export files that are compatible with that program.  Yes, element type is a column.
> 2. **Attachments.** Restraints, loads and components on a node: columns on the node row, or their own tables joined on node with a mark in the layout table? A's answer was separate tables with marks. Separate tables keep the layout row narrow and give the agent clean targets. Your call.
> ANSWER: To begin with, it will be separate tables with marks that we should adopt.
> 3. **Canvas authoring.** With the tables authoritative, does the canvas keep a few gestures that are literally row shortcuts, click a node to add a restraint row and route by direct distance entry, or is it a viewer, selector and probe only in the first version? I'd keep the few gestures, but it changes the mock states.
> ANSWER: keep the few gestures.
> 4. **Layouts.** Should Grid mean table only, full width, no canvas, given how many columns a node row carries? And is the layout remembered per stage, so Results can stay Split while Model stays Grid?
> ANSWER: Yes and Yes.
> 5. **Delegation and checking.** Governance keeps agent proposals as drafts until accepted. So delegation means the agent produces a batch, the human checks it in the tables and accepts the batch. Two things would make checking real: every row and cell shows who entered it and when, and the human can mark a row as checked, a plain human act rather than a status claim. Do you want the checked mark, and is a batch the right unit of acceptance?
> ANSWER: row by row approvals, but multi-row and whole batch are also possible.
> 6. **Symmetry.** Is the agent checking the human's work, leaving the same marks, in scope for the first version?
> ANSWER: agent should be able to check the human's work and provide some means of feedback without altering the tables.  This may be yet another full-screen page in the app where the agent and human work collaboratively on reviews and report writing, with the information there on-screen and edits visible (and can be hidden), showing previous iterations and current.
> One consequence to flag. The permanent canvas was C's answer to keeping the model visible during a Loads stage. Without it, the Loads page needs its own way to show where a load acts. Node-rows solve that if loads are marked on the layout row too, which is question 2.
> RESPONSE: Yes, node rows will be marked with such.  And views should be able to toggle between table view, model view, and mixed, for each stage, with memory switching between stages to the view state it was previously left in.

## 2. The chosen direction

The product is a surface on which a stress engineer and an agent work on the same analysis with the same means, built around the engineer's needs, where the agent can take equivalent action at the engineer's direction and the engineer never feels demoted. The mechanism that makes this possible is that the tables are the model: the 3D model is generated from the table data and holds nothing the tables do not, so every action either party can take is a row action, and the tables are also the format in which the engineer checks work that was delegated.

1. **The tables are the model.** The layout table uses the CAEPIPE grammar: one row per node, carrying the element that arrives at that node; From shown on every row and defaulting to the previous row's node; a branch is a row whose From names an earlier node; element type is a column. Restraints, loads and node data are separate tables joined on the node, and the layout row carries a mark for each attachment it has, which opens the joined row. The grammar and the field names are chosen so that export to a CAEPIPE-compatible file is a projection of the tables, not a translation. Export compatibility with CAEPIPE is a product requirement of this direction (owner answer 1); its format is characterised by RESEARCH-E (§7).
2. **Stages are the workflow across pages of tables.** Model, Loads, Results and Review are the stages, each a set of tabbed tables; the rail is the order the work is done and read, not a set of modes: one selection, one undo, one issues list, one Run button. Report writing lives on the Review page and the report is produced from it (§8, item a).
3. **Three views per stage, all normal.** Table: the stage's tables at full width, no canvas. Model: the canvas, with the stage's current table as a drawer and the inspector. Mixed: table and canvas side by side. The view is remembered per stage; switching stages restores the view that stage was last left in. None of the three is the posture of the app.
4. **The canvas is derived and live, with a few gestures that are row shortcuts.** Selection, cursor sync and the probe; route by direct distance entry, which writes a layout row; click a node to add a restraint row. Every gesture is the same structured operation as typing the row. Nothing can be authored in the canvas that is not a row.
5. **Equivalent action, visible and checkable.** The agent acts only through row operations at the engineer's direction. Its proposals are drafts until accepted and land in the tables as marked proposed rows and cells with old and new values, and as ghosts in the canvas. Acceptance is row by row; multi-row and whole-batch acceptance are also available. Every row and cell carries its origin (entered by the engineer, agent proposal accepted by the engineer, propagated, generated by a rule pack, imported) and when. The engineer can mark a row as checked; that mark is a human act recorded as a tag on the row's content, never a software status, and it lapses visibly when the row changes.
6. **The agent checks the engineer's work without touching the tables.** On the Review page the agent's feedback is attached to rows, results and report text by reference: checks and open issues, the classes of output the agent is permitted to produce. The engineer and the agent work on reviews and on the report there together, with edits visible or hidden, and previous iterations retained beside the current one.
7. **The engineer is never demoted.** There is no agent-private surface. The agent panel holds the conversation, the queue of proposals and the checks; all of the agent's work is legible in the same tables and canvas the engineer uses. The engineer directs, accepts, tags and writes. The tables and the model are the hero.

## 3. What carries over from the concept return

| From | Carried | Not carried |
|---|---|---|
| A, Layout Sheet | The sheet as the model; joined tables with marks; cursor sync in both directions; one tab strip and one selection across sheets; the results sheet joined on node and element. | The slide-over inspector as the only inspector (open for the design system). |
| B, Workbench | The probe as a first-class canvas tool with its card at the cursor; the few authoring gestures on the canvas; the physical-glyph vocabulary as one option for the presentation language. | The inspector as master; the six-verb tool strip; the grid as a drawer in every layout. |
| C, Run Book | The stage rail in work order; live issues on every page; Run as a button that says why it is disabled; the report assembled from the pages the engineer has been reading; the agent's permanent seat. | The canvas that never leaves the screen; the stage-contextual right pane as the only right pane. |

## 4. Consequences for the brief

Applied as V1.2 of [`DESIGN_BRIEF_V1.md`](DESIGN_BRIEF_V1.md), each change tagged with the answer that caused it:

- §2: data authority stated (the tables are the model; the canvas is derived); the layout grammar changed from one row per element to one row per node in the CAEPIPE grammar; CAEPIPE-compatible export added as a requirement; the collaboration surface stated as the product's organising idea.
- §3: principle 2 rewritten for the Table view; principles 11 (the tables are the model) and 12 (equivalent action, visible and checkable) added.
- §4: layouts replaced by views (Table, Model, Mixed) remembered per stage; stages added; the layout table, attachment tables, origin marks, checked tag, Review page and export added to the surfaces.
- §5: node rows, views, stages, origins, "accept" for proposals and the checked tag added to the vocabulary.
- §6: three items added to the decision packet (§6 below).
- §7: mock states rewritten for views and stages; state 9 (Review page) added.
- §9: phase 3 restated.

## 5. Governance touchpoints

The chosen direction stays inside the constraints sheet ([`../RESEARCH/C_ui_constraints.md`](../RESEARCH/C_ui_constraints.md)) as follows; the items that need an owner ruling are in §6.

- Every mutation, by gesture, by typed row, by paste or by accepted proposal, is one structured operation through the one applier route (C-74, C-75). The canvas gestures are shortcuts to row operations, not a second route.
- Agent proposals are drafts shown as diffs until the engineer accepts them (M-14, C-77). Row-by-row acceptance of a proposal is the human gate M-14 requires; it is acceptance of a proposed edit, not engineering acceptance, and the interface says "accept", never "approve" (C-06).
- The agent's feedback on the Review page is limited to the output classes the boundary permits for agent output: drafts, proposals, evidence summaries, checks and open issues (C-77). It never states that work is accepted, verified or approved.
- The checked tag is a human tag on a row, in the sense of the state-labelling affordances the PRD intends (C-22), not a status from the automatic vocabulary (C-09, C-10) and not an acceptance record (C-06, C-65). It binds to the row's content so that it lapses visibly on change, in the spirit of C-65.
- The Review page shows computed outcomes beside review text, so it is a surface in the class that carries the acceptance sentence (M-02); the placement rule of one disclosure per surface class applies to it as to the results tables.
- Per-cell origin is already representable: the operation layer's author type includes the agent (C §3 item 11).
- Exported files carry export metadata, which the report supplement covers when reports include it (M-04). "Compatible with CAEPIPE" is a product claim that needs evidence before it is made on any surface (C-05); until then the export is described by what it produces.

## 6. Added to the decision packet

Continuing the brief's three items and the concept return's three:

7. **The checked tag.** Whether a human-authored, content-bound "checked" tag on a row is within the PRD's intended state-labelling affordances (C-22) and outside the acceptance-record prohibition (C-06, C-65), and what it may be called.
8. **Agent feedback on the Review page.** The wording envelope for agent checks and open issues, confirmed against the agent-output row of the professional boundary (C-77), and confirmation that the Review page is one surface class for the acceptance sentence (M-02).
9. **The export and its claim.** What the export may be called before compatibility evidence exists (C-05), and the export-metadata notice it triggers in reports (M-04).

## 7. New work

- **RESEARCH-E, CAEPIPE exchange format.** A bounded, objective characterisation from public documentation of the file formats CAEPIPE reads and writes, the layout row fields and element types, attachment data, load cases and units, and a mapping onto the grammar in §2 item 1 with the gaps named. Opus, per the owner's model rule. Brief at [`../../briefs/RESEARCH-E_caepipe_format.md`](../../briefs/RESEARCH-E_caepipe_format.md); return to `../RESEARCH/E_caepipe_format.md`.
- **Mock state 9.** The Review page: the agent's checks on the engineer's work, the report draft with a previous and a current iteration, edits shown and hidden.
- **Rendering brief scope.** The canvas being derived does not reduce the rendering brief owed to the piping session under D-70: overlays, result colour, deformation and ghosted proposals are still canvas work. The brief is cut after the owner confirms this record.

## 8. Open for the owner's word

ROOT proceeded on these readings; each is reversible.

- (a) The Review page is the fourth stage and replaces Report on the rail; the report is produced from the Review page.
- (b) Export comes first; import of the same format is not assumed. The format is whatever RESEARCH-E finds CAEPIPE publicly documents.
- (c) The views are called Table, Model and Mixed, the owner's words.
- (d) The checked tag is a tag bound to row content, not a status, pending packet item 7.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
