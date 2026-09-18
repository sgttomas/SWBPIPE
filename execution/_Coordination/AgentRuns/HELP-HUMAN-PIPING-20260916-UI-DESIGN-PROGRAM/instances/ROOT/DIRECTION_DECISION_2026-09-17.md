# Direction decision — the owner's synthesis, 2026-09-17

Status: CONFIRMED by the owner on 2026-09-18 with corrections (§9): readings (a), (b) and (e) confirmed; (c) corrected to the view names Table, Model and Both; (f) corrected to numeric node IDs throughout; (d), the checked mark, settled on 2026-09-18 after discussion: kept (§10). The owner's direction was given in session on 2026-09-17 in two messages, transcribed verbatim in §1 from bytes extracted from the ROOT session transcript. §2 onward is ROOT's transcription of that direction into a design basis, updated for the corrections; where the transcription and the owner's words differ, the owner's words govern. This record supersedes the recommendation in [`../CONCEPTS/CONCEPT_DIRECTIONS_V1.md`](../CONCEPTS/CONCEPT_DIRECTIONS_V1.md) §5: none of the three directions is chosen as returned. The form follows [`DESIGN_BRIEF_V1.md`](DESIGN_BRIEF_V1.md) §1, the owner's words first and the consequences after, and the verbatim-extraction practice of the D-70 ruling record.

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

1. **The tables are the model.** The layout table uses the CAEPIPE grammar: one row per node, carrying the element that arrives at that node; From shown on every row and defaulting to the previous row's node; a branch is a row whose From names an earlier node; element type is a column for what the target file knows as elements (pipe, bend, valve, reducer, rigid, expansion joint and the rest); a tee is a branch-connection kind recorded in the node-data table, not an element type (§9, item e); node IDs are numeric throughout (§9, item f). Restraints, loads and node data are separate tables joined on the node, and the layout row carries a mark for each attachment it has, which opens the joined row. The grammar and the field names are chosen so that export to a CAEPIPE-compatible file is a projection of the tables, not a translation. Export compatibility with CAEPIPE is a product requirement of this direction (owner answer 1); its format is characterised by RESEARCH-E (§7).
2. **Stages are the workflow across pages of tables.** Model, Loads, Results and Review are the stages, each a set of tabbed tables; the rail is the order the work is done and read, not a set of modes: one selection, one undo, one issues list, one Run button. Report writing lives on the Review page and the report is produced from it (§8, item a).
3. **Three views per stage, all normal.** Table: the stage's tables at full width, no canvas. Model: the canvas, with the stage's current table as a drawer and the inspector. Both: table and canvas side by side. The view is remembered per stage; switching stages restores the view that stage was last left in. None of the three is the posture of the app.
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
- §4: layouts replaced by views (Table, Model, Both) remembered per stage; stages added; the layout table, attachment tables, origin marks, checked tag, Review page and export added to the surfaces.
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

ROOT proceeded on these readings. The owner's word on each, given on 2026-09-18 (§9), is noted after it.

- (a) The Review page is the fourth stage and replaces Report on the rail; the report is produced from the Review page. Owner: "approved."
- (b) Export comes first; import of the same format is not assumed. The format is whatever RESEARCH-E finds CAEPIPE publicly documents. Owner: "correct."
- (c) The views are called Table, Model and Mixed, the owner's words. Owner corrected the names to `Table`, `Model`, `Both`; applied throughout.
- (d) The checked tag is a tag bound to row content, not a status, pending packet item 7. Owner: "I don't understand this one, we need to discuss more." Settled after discussion the same day, §10: "keep the checked mark."

Added after RESEARCH-E returned ([`../RESEARCH/E_caepipe_format.md`](../RESEARCH/E_caepipe_format.md) §6.1, gaps 1 and 3), the same day:

- (e) **Tees.** In CAEPIPE a tee is not an element type; it is a branch-connection kind recorded at the node (welding tee, sweepolet, weldolet, fabricated, extruded, radiused, thickened-pipe branch) and written to the file as a node code. The owner's answer 1 lists "tee" among element types. ROOT's reading, pending the owner's word: the element-type column carries the elements the file knows as elements (pipe, bend, valve, reducer, rigid, expansion joint and the rest), and the branch-connection kind lives in the node-data table with a mark on the node row, so that export stays a projection. The alternative is to keep tee kinds in the element-type column and translate them on export. Owner: "Your reading is correct."
- (f) **Node IDs.** The documented file uses numeric node numbers; the brief specifies alphanumeric IDs to beat the numeric-only complaint. ROOT's reading, pending the owner's word: IDs stay alphanumeric in the product, and export assigns numeric node numbers through a recorded map carried in the export metadata. The alternative is numeric IDs throughout, which removes one of the brief's departures from the target tools. Owner: "change to numeric IDs throughout." Applied: node IDs are numeric throughout.

## 9. The owner's word, 2026-09-18

Transcript timestamp 2026-09-18T06:08:18.062Z. SHA-256 of the extracted bytes: `cd18a70104884e6915a010f18b0ad50ea15c27f943e57cc306d6a3e911e41f10`.

> a) approved.
> b) correct.
> c) `Table` , `Model`, `Both`
> d) I don't understand this one, we need to discuss more.
> e) Your reading is correct.
> f) change to numeric IDs throughout.

Applied by ROOT the same day: the brief to V1.3 (views Table, Model, Both; numeric node IDs; tees as node data); §2 and §8 of this record; the phase 3 design-system brief sealed after the recommendations in §10 were agreed.

## 10. Recommendations agreed, 2026-09-18

The owner asked (transcribed by ROOT in session on 2026-09-18; the session transcript had not yet stored these two messages when the record was written, so the hashes are of the transcribed text, not of extracted bytes), SHA-256 `ea6aa1864f3d43c7f458cd90b31c15d601fb8e554704724be52665f4347b9b0a`:

> regarding section 4 and 7 of the direction decision, what makes sense to you and what would you recommend?

ROOT explained the checked mark as a second act distinct from accepting a proposal (a human tag on a row, stale on change, with an unchecked filter) and recommended keeping it, and made the recommendations below on the product shape, the mock states and the new work. The owner answered, SHA-256 `db86dfb87c86a61f1e6df3fe7e3107cb4a6efda729c272e287b175ed08b58bb3`:

> agreed on all, keep the checked mark, seal the design-system brief.

Adopted into the brief as V1.4:

1. Temperatures and pressures as named load sets: a Load column on the node row names the set, the set's values read through on the row, typing over them edits the set or forks a new one. Keeps export a projection and matches the CAEPIPE mental model; typing on the element remains possible.
2. Loads never force a stage hop: the load mark on a node row opens the load row in place on the Model page; the Loads stage is where cases are authored and all loads seen together.
3. First-open view per stage: Model opens in Both, Loads in Table, Results in Both, Review in Table; the per-stage memory rule governs after that.
4. Origins quiet, proposals loud: origin is a small gutter glyph with a filter and a changes-since control, not a colour on every cell; proposed rows are the one thing shown strongly.
5. The agent panel never covers the tables: a right column collapsible to a strip, present in every stage; proposals are read in the tables, the panel holds the conversation and the queue.
6. Hanger design under Results, since the design pass needs a solve; vendor tables through Libraries.
7. The Review page as three columns: report outline in the required order, section content with live tables from Results and authored text, and a comment stream with the agent's checks and open issues by reference; iterations as named snapshots with a diff toggle; the acceptance sentence once, at the top.
8. The checked mark kept: called Checked, per row with multi-row, stale on change, with an unchecked filter. The governance standing and wording remain packet item 7.
9. Fifteen mock frames rather than thirty-six: every state in its natural view in light; states 4 and 7 also in Table, state 8 also in Model; states 2, 7 and 9 also in dark. Light and dark as peers is proven by the tokens and the specimen.
10. HTML mocks: static, self-contained pages at 1440 × 900 with a light and dark switch, drawn from one small fictional sample model with values labelled as sample.
11. The rendering brief to the piping session is cut after the design system defines the presentation language; the decision packet is prepared after the mocks, with the frames as evidence.

## 11. Mock review decisions, 2026-09-18

The fifteen frames ([`../MOCKS/frames/index.html`](../MOCKS/frames/index.html)) raised fourteen questions that only a screen can raise ([`../MOCKS/MOCKS_V1.md`](../MOCKS/MOCKS_V1.md) §4). ROOT recommended an answer to each; the owner's word, extracted from the session transcript, timestamp 2026-09-18T10:06:37.626Z, SHA-256 `be8fdfdf9b0ee00ed7e17d034cc296e6d7bcadaf67a1b3c2d43d04812bee7786`:

> I approve your recommended approach for all fourteen, proceed accordingly.

Adopted, numbered as the questions are:

1. The inspector in Both view docks; the canvas shrinks while it is open; the table never reflows.
2. No status chip when the model is complete but unsolved; an absent status is honest and no seventh label is added.
3. Two chips after a solve, one per authority domain (mechanics, user rules). "Human review required" is shown on the Review page only, unless the registry requires it elsewhere, which decision-packet item 4 settles.
4. After a model change the chips drop, the rail's Results caption reads Stale and the results header carries the band; as drawn.
5. The run log stays a popover on the Run button, shorter; the full log lives on the run record.
6. The failure banner appears once, on the page; the drawer's row carries the link.
7. The required marks sit on the first element's row; the start node's row carries none; the CAEPIPE layout.
8. With Envelope on, the case selector stays visible and disabled so the control never moves.
9. The run identity stays inside the results header disclosure: provenance one click from the table.
10. Report preview and Export stay on the Review page header; the report is produced from that page.
11. The dark result scale is re-anchored one step down so its lightest step sits below the edge line: a token change.
12. The engineer's draft ghost is a thin dashed centreline with a faint tube outline.
13. Node labels default to Budget mode, with All on demand, as the design system says.
14. The combination editor is a row expansion under the case, matching the joined-row pattern.

Also adopted: the frames' decisions that the design system had not made (MOCKS_V1 §1 and §2) and their departures (§3) are folded into a design-system revision, and the six token gaps (§5) are closed there, so that specification, tokens, specimen and frames agree.

## 12. The decision packet

The nine items of the brief §6 are prepared as decision packet [D-71](../../../../_DECISIONS/D-71_interface_program_governed_text_decisions.md), a PROPOSAL awaiting the owner's ruling item by item, with RESEARCH-F as its inventory and the frames as its evidence.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
