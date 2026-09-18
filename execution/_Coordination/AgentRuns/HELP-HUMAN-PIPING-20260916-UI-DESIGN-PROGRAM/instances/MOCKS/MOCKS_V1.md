# MOCKS V1 — the fifteen frames of the nine workflow states

Phase 3 of the SWB Piping Designer interface program. Fifteen static HTML frames under `frames/`, one sample model (`sample_model.md`) running through all of them, a contact page (`frames/index.html`). Every colour, type, spacing and layout value in a frame comes from `../DESIGN-SYSTEM/tokens.json` through `tokens.css`, generated with the design system's own `gen.mjs` mapping and inlined in each frame; the component rules follow `specimen.html`. The canvas in every frame is a schematic figure in inline SVG in the design system's §6 figure language, labelled in the frame as a mock rendering. Nothing in a frame works except the caption bar's theme switch and the scaling. Each frame is one self-contained file — inline CSS, inline SVG, inline script; no `<link>` and no external reference — because the owner's review surface renders a local HTML file from its bytes alone (§6).

Reading order: §1 what is common to all frames; §2 the frames one by one (what it shows, the §8 item it tests, decisions the design system did not make); §3 departures from the design system with reasons; §4 questions for the owner that only a screen can raise; §5 tokens and gaps; §6 how the frames were composed.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). The frames propose; nothing here is accepted design, and the sample values are placeholders, not results.

## 1. Common to every frame

**The page around the window.** A 32 px caption bar above the window names the frame, its state, stage, view and theme, and carries the theme switch (Light / Dark / System) and a link to the contact page. It is not product chrome; it is drawn in the tokens so that the page reads as one piece. The window below it is 1440 × 900 and scales down as a whole (`transform: scale`) to fit a narrower or shorter pane; it never reflows.

**The shell** at the design system §0 geometry: toolbar 48, rail 56, status bar 24, agent strip 44 or column 340, Model-view inspector 340, table drawer 280, issues drawer 200, Both view 55/45 with the inspector as a 300 px slide-over. Toolbar left to right: the SWBPIPE wordmark, project name with save state, the view switch, centred Run / Issues / Agent, then the units combobox and the ⌘K search. Status bar: the M-08 chips at the left, the issues count, the selection, the units and the information control at the right.

**One story.** Loop 4 header (pump P-401 discharge to vessel V-402 with a branch to the tie-in N3): 16 node rows, three sections, two materials, two load sets, seven restraints, three loads, two node-data rows, six cases from a sample rule pack (one edited, one authored), Run 02 failed on the nonlinear support at 20, Run 03 solved with the governing ratio 0.72 · EXP-A1 · pack 1.2 at node 70 and hanger H1 designed against the user's Vendor-A library, proposal P-12 landing and being accepted row by row, Run 04 and the Review page. The model changes only where the story says it does (the paste in state 3 adds the branch; state 4 adds restraints and loads; state 8 accepts P-12 row 1).

**Decisions the design system did not make, common to all frames** (each a proposal for the design system):

- D-1 The start node's row (10) carries no Section, Material or Load set: those columns are entered on the first element's row (20) and propagate from there. The design system fixes the grammar but not what the start row shows; "—" in `text.muted` is proposed.
- D-2 A propagated cell carries the corner tick only; a cell typed over loses it. The Section tick on node 110 (P2 entered) is the worked example.
- D-3 The rail's stage items carry a caption in `type.size.caption` when a stage has a state to name: "Failed" (state 6, `status.failedInk`) and "Stale" (state 8, `issue.warning`) alongside the design system's "Historical". The Results and Review items are disabled with the tooltip "No run yet" before the first run.
- D-4 The status chip. "Model incomplete" (MODEL_INCOMPLETE · Solver) is shown before any run while the model cannot be solved (states 1–3: missing section and material, then no restraint). When the model is complete but unsolved (states 4 and 5) no chip is shown, because none of the six statuses applies and "Ready" is forbidden. After a solved run two chips are shown side by side, one per authority domain: "Mechanics solved" (Solver) and "User rules checked" (Rule pack). On the Review page "User rules checked" and "Human review required" (Human). See Q-2 and Q-3.
- D-5 The table footer is the counts line: rows, elements, bends, restraints, loads, node data, checked and stale, issues; on the right the read-through and Origins switches as outline chips. The design system names the footer counts without listing them.
- D-6 Tab strips carry counts in `text.muted` after the name (Layout 16 · Restraints 7 · Node data 2).
- D-7 Node label plates in the canvas are placed by a search that keeps them off the tubes, off the glyph plates and inside the canvas; the selected node's plate takes a `canvas.selection` edge, a proposal-ghost node a `canvas.proposalGhost` edge, and a node with an open issue an `issue.warning` edge.
- D-8 Two azimuths are used for the isometric figure: 40° for the wide Model-view canvas (1000 × 548) and 200° for the tall Both-view canvas (603 × 828); elevation 28° in both. At the design system's implied 45°-class view the branch riser 150–160 projects onto the main riser 30–50 of this sample, so the mocks cannot use it. The HUD's view preset is drawn as "Iso" in both cases; the product would let the engineer rotate.
- D-9 The canvas carries, at the bottom, the triad with "Y up" at the left, the 1 m bar (and 1000 N arrow where loads are drawn) at the right, and the line "Mock rendering, not the engine's canvas · schematic isometric, Y up" centred in `text.muted`. The mock line is a requirement of the brief, not a product element.
- D-10 Restraint glyph plates: "+Y · gap 3 mm · μ 0.30", "Rigid Y · RS-02", "VS · H1", "Guide X Z · gap 2 mm · μ 0.30" at 11 px in `canvas.labelBg`; load plates "−2000 N · W", "+4.0 mm · T1" at 12 px. Load vectors take `cat.1` (force) and `cat.3` (displacement) on the Model stage and the neutral `canvas.vector` on the Results stage where the tube carries the result colour (state 8's Model canvas also uses the neutral vector so the proposal ghost is the only loud thing).
- D-11 Result colour on tubes is the design system's seven-step scale interpolated in OKLCH with `color-mix`, so it follows the theme's tokens without a new colour.

## 2. The frames

### s1_table_light — state 1, Model · Table · light

**Shows.** A project not yet saved ("Loop 4 header · not saved"); the Layout tab with two rows: node 10 (start, all "—") and node 20 with DX in the edit state ("3000" with the caret and the unit "mm" in the cell) and the required asterisks (`mark.required`) on Section, Material and Load. The empty-state hint under the rows (↩ adds row 30, ⇥ moves right, ⌘V pastes rows, propagation, what the asterisk means). Run disabled on `disabled.fill` with its tooltip drawn: "Run is unavailable — Section missing at node 20, Material missing at node 20". Issues 3 (octagon, blocking) in the toolbar, the rail and the status bar; Results and Review disabled in the rail; the status chip "Model incomplete"; the status bar's information popover open with the About link and the maturity line "Technical preview — not a released product." as its only other line (M-01 placement per §7.4). Marks column present and empty; gutter empty.

**Tests §8.** State 1: an empty model's first row with the required marks and the Run button's reason; the gutter and marks column empty.

**Decisions.** The Run tooltip names only the two blockers of the solve; the missing load set is a "Blocks rule check" issue and is left to the Issues count (the design system's example tooltip names exactly those two). The empty-state hint's wording. The third issue class shown in the count but not the tooltip. The tooltip is drawn hanging from the button and covering the T, P and Marks headers, as a hover would.

### s2_model_light, s2_model_dark — state 2, Model · Model · light and dark

**Shows.** The canvas at 1000 × 548 with nodes 10–40 drawn and the engineer routing from node 40: the compass at 40 with +Y active (56 px, `canvas.axisY`) and X, Z at 40 px hairlines; the draft ghost from 40 upward as a dashed tube in `canvas.draft` at 0.7; the length field above-right of the axis tip with "1500" and the caret and "mm" muted, the focus ring on the field; the draft node plate "50" with a `canvas.draft` edge; the routing hint strip under the HUD ("Route · ↩ places 50 · ⇥ next axis · − reverses · ⎋ cancels"); the HUD with Route pressed and Deformation and Probe disabled (no results). The table drawer (280) on the Layout tab with rows 10–40, node 40 selected, and the draft row 50 (dashed `canvas.draft` bar in the gutter, muted text, the typed 1500 in `accent.text` under DY, propagated P1 · CS-A · OP1 with ticks). The inspector for node 40 with a "Routing from node 40" block on top (next node, axis, length in accent, what the row carries, "Bend at 40… B" and "Cancel ⎋"). Run disabled (no restraint yet), Issues 1 blocking, "Model incomplete". The dark frame is the same screen under the dark tokens.

**Tests §8.** State 2: the compass and direct distance entry writing rows into the drawer; the draft ghost against the dark ground; label budget on a small model.

**Decisions.** The routing block at the top of the inspector. The hint strip under the HUD (the design system places keyboard hints nowhere in particular). The draft row's anatomy (dashed bar, muted text, accent length). The draft row's Type shown as "Pipe" until the engineer turns the run. The length field's position above-right of the axis tip so the node's own plate stays readable. "No restraint in the model" as one "Blocks solve" issue, which keeps Run disabled and the chip at "Model incomplete".

### s3_table_light — state 3, Model · Table · light

**Shows.** Rows 10–130 with read-through on (T1 P1 T2 P2 in `surface.sunken` with ticks), propagation ticks on Section, Material and Load from row 30 down and none on 110's Section (P2 entered), node 130 selected with DY in the edit state ("−1500" and the caret). Under the rows the paste band on `surface.raised` at elevation 1: "Paste · 3 rows from the clipboard · after node 130 · 2 columns ignored · node IDs taken from the source", the mapping row with each source column naming its target (Sect → Section, Matl → Material) and Sched and Note set to Ignore in the dashed style, a three-row preview in the layout grammar drawn as draft rows (140 a branch from 70 with P3 · CS-B · OP2 entered, 150 and 160 propagating them with ticks), Paste 3 rows ↩ (primary) and Cancel ⎋. Footer: 13 rows · 12 elements · 3 bends · 1 valve · 1 reducer · 32 propagated cells · 0 issues; chips Read-through on, Origins. Marks column empty (no restraints yet).

**Tests §8.** State 3: propagation marks after keyboard entry; the paste band with two unmapped columns; the footer counts.

**Decisions.** The preview rows reuse the draft-row style. Where the source left Section, Material or Load blank the preview shows the propagated value with its tick, so the engineer sees what the paste will produce. Ignored columns keep their source name beside "Ignore". The edit cell's unit is set at caption size at the cell's right edge so a 72 px column holds "−1500 mm" with the caret.

### s4_both_light — state 4, Model · Both · light

**Shows.** Tables at 55 % (737 px): Layout with read-through off (Type at 96 so the table fits the pane without a scroll), all 16 rows, the marks column filled: restraint marks on 10, 20, 60, 80, 120, 130, 160; load marks on 40 and 130; node-data marks on 70 and 90; the gutter with the checked mark on 10 and 20, the stale checked mark on 30 (amber, dashed), the accepted-origin diamond on 60 (H1 came from proposal P-09); the provenance warning tick on the CS-B cell at 140; node 20 selected; the hover card on node 20's restraint mark ("Restraint +Y · gap 3 mm · μ 0.30 · RS-01 — ⌘↩ opens the row here"). Canvas at 45 % with the inspector slide-over (300) open for node 20 over its right half: geometry, section and material, the load set read through, the restraint row, Origin and Checked (with Clear check ⌘⇧K), the provenance disclosure. The figure shows every glyph: anchors hatched at 10, 130, 160; the +Y support with its 4 px clearance, base, friction ticks and up-arrow at 20 with the plate to its left; the rigid rod at 80 (partly under the slide-over); the spring can at 60; the guide brackets with inward arrows at 120; the force arrow at 40 and the displacement arrow with its base bar at 130.

**Tests §8.** State 4: restraint and load marks on node rows, the glyphs with their direction arrows, gap and friction labels; the inspector slide-over open (open item 3).

**Decisions.** The figure is fitted to the visible part of the canvas with the selected node in view (the canvas is not re-fitted when the slide-over opens in the product; here it is drawn as the engineer would have left it). The +Y plate placed on the node's left when the node is near the covered side. The hover card wording. The mock label shortened and left-aligned so the slide-over does not cut it. The footer's "Read-through off" chip with the T, P values available in the Load cell's tooltip (open item 4: off by default in Both, as the design system says).

### s4_table_light — state 4, Model · Table · light

**Shows.** The same table with read-through on (964 px) and two joined rows open in place: under node 20 the Restraints joined row ("Restraints · node 20 · joined row · ⎋ closes") with the +Y row (Y, gap 3, μ 0.30, rigid, RS-01) and "Add restraint…"; under node 40 the Loads joined row from the Loads stage (Force, Y, −2000, N, W, note) and "Add load…". Node 40 selected with its load mark in the open state. Footer counts including "2 joined rows open" and "1 issue" in `issue.warning`.

**Tests §8.** State 4, Table: restraint and load marks on node rows, and the mark opening the row in place (direction record §10).

**Decisions.** The joined row's anatomy: a 2 px `border.strong` rule at the left, the caption with the expanded chevron, the child table at its own column widths, an add-row. Two joined rows drawn open at once to show both kinds; in use one opens per ⌘↩ and stays until ⎋.

### s5_table_light — state 5, Loads · Table · light

**Shows.** The Loads stage on the Cases tab (Cases 6 · Load sets 2 · Loads 3 · Wind 0 · Seismic 1). A header band: "Cases · generated from rule pack sample-rules 1.2 · sha256:9b1c4e02… on 2026-09-17 10:05", the button "Generate from rule pack…", and at the right the M-15 caption once: "Rule expression: Display only, not accepted as input" with the display-only glyph. The table: Case, Expression, Stress type, Rule, Origin, Rule expression. Generated rows carry the generated glyph in the gutter and "Generated · sample-rules 1.2"; EXP1 carries "Edited by R. Tufts · was T1 (generated)" and no glyph; OCC1 "Authored by R. Tufts". Every rule expression sits in the dashed `displayOnly.border` frame on `surface.sunken` in `type.family.mono`. OCC1 selected with its Expression cell focused and the combination editor open under it: the terms W + P1 + SE1 (0.3 g X) as chips, the available terms dimmed, Stress type and Rule comboboxes with the pack named, Cancel ⎋ and Done ↩. Footer: 6 cases · 4 generated · 1 edited · 1 authored · 4 with a rule · 2 load sets.

**Tests §8.** State 5: generated, edited and authored case rows with their origin glyphs; the display-only expression frame.

**Decisions.** The header band above the tab strip on the Loads stage, carrying the pack identity, the generate action and the single M-15 caption (the column header says only "Rule expression"; the frame is the form). The combination editor as a cell popover with term chips. The Origin column's wording for the three origins.

### s6_both_light — state 6, Results · Both · light

**Shows.** Results just after Run 02 stopped. Across the top of the tables side the failure banner on `issue.warningTint`: "Run 02 failed: nonlinear support at node 20 did not converge." with "Show node 20" (the solver's diagnostic in one sentence, C-58/C-59). The results header "Stresses · Run 02 · 15:02 · stopped in OPE1" with the Information control closed. The tab strip, then the empty state ("No results — Run 02 stopped in OPE1 (case 3 of 6) at the iteration limit. A run's results are one immutable set, so a stopped run has none. Change the support at node 20 or the iteration limit in Run settings, then Run again."). The Issues drawer (200) open and filtered to the Nonlinear class (filter chip "Nonlinear 1 ×", "All classes 2"), the row selected. Under the Run button its popover, the run log: assemble, W, SUS solved with times; OPE1 stopped at iteration 50 · node 20; OPE2, EXP1, EXP2, OCC1, hangers and rules not run; Run settings…, Run record, Run again. The status chip "Model incomplete" with its popover open: the raw token `MODEL_INCOMPLETE` in mono, Authority Solver, the run and time, the reason. The rail's Results item current with the caption "Failed"; Review disabled ("No solved run"). The canvas with node 20 selected and its plate carrying the warning edge, loads in the neutral vector colour, Deformation and Probe disabled.

**Tests §8.** State 6: the running Run button, then the failure banner and the issues drawer; the status chip and its popover.

**Decisions.** The running moment is evidenced by the run log in the Run button's popover (concept direction 3's progress popover), since a static frame cannot show the running button and the failure at once; the log names the stage and case as the running tooltip would. The empty state's wording. The drawer opened filtered to the failing class (the concept storyboard's "opens to the Nonlinear class") with a filter chip that clears. The rail caption "Failed".

### s7_both_light, s7_both_dark — state 7, Results · Both · light and dark

**Shows.** The results header: "Stresses", "Run 03 · solved 15:21 · immutable", the case selector "Case: EXP1", the Envelope switch off, the evidence chip "Internally verified" (raw token INTERNALLY_VERIFIED in its tooltip), the Information control open, and under the band the disclosure on `surface.raised`: the acceptance sentence (M-02) verbatim and, in mono, the run identity (Run 03 · 2026-09-17 15:21 · sha256:4df0f798… · solver 0.2.0 · rule pack sample-rules 1.2 · sha256:9b1c4e02… · settings S-03). The tab strip (Summary · Stresses · Displacements · Restraint loads · Restraint summary · Forces and moments · Hangers). The stress table for EXP1 sorted by ratio: Node, Element, Case, Stress [MPa], Allowable [MPa], Ratio with its data bar and the 1.0 tick, Rule, Pack; 15 rows; node 70 selected. Footer: 15 elements · 6 cases solved · 0.72 governing · EXP1 · node 70 · EXP-A1 · pack 1.2; "Sorted by ratio". The canvas: tubes coloured by the EXP1 ratio, the selected element 60–70 with its halo, the legend at the top right (Ratio · expansion · EXP1, the scale bar with the marker at 0.72 · node 70, Rule EXP-A1 · pack 1.2, Range 0 – 1.00 with "set", Unsolved none), the probe pinned beside node 70 (Node, Element, Case, Stress, Allowable, Ratio with bar, Rule, Pack; footer "Run 03 · pinned" and the evidence chip), the HUD with Probe pressed and Deformation available. Status bar: "Mechanics solved", "User rules checked", Issues 1, "Element 60–70 · node 70". The dark frame is the same screen under the dark tokens.

**Tests §8.** State 7: the stress table with data bars, the case selector and Envelope, the evidence chip, the results header disclosure open; the coloured model with the legend and probe in both themes.

**Decisions.** The disclosure carries the run identity under the sentence (the design system puts hashes in the run record and popovers; this is the results surface's own record line — see Q-9). The legend's quantity line "Ratio · expansion" with the case beside it. The probe pinned at 18 px from the node, kept clear of the legend. "Sorted by ratio" as a footer chip. Element column at 80 and Allowable at 116 so three-digit elements and the unit fit.

### s7_table_light — state 7, Results · Table · light

**Shows.** The same results surface in the Table view with Envelope on: the table shows each element's governing case (EXP1 or OCC1 per row) and the case selector reads "Case: all" in `text.disabled`; the Information control closed (the disclosure is one click away). The Ratio column header's menu open: Sort descending (on, ⌘↓), Sort ascending, Data bar · 1.0 tick (on), Colour by scale, Filter ratio ≥ 0.50 (3 of 15 rows), Unchecked rows only, "Rule and pack are their own columns · width 120 · reset". Footer: 15 elements · 6 cases in the envelope · the governing line; chips "Envelope: governing case per element", "Sorted by ratio".

**Tests §8.** State 7, Table: the stress table with data bars, the case selector and Envelope, the evidence chip.

**Decisions.** With Envelope on the case selector is disabled rather than hidden, so the engineer sees why it does not answer. The column menu's contents (sort, the bar, the filter, the width). The Envelope footer chip.

### s8_table_light — state 8, Results · Table · light, agent column open

**Shows.** The Hangers tab with the agent column open (surfaces at 1044). Results header "Hangers · Run 03 · solved 15:21 · immutable" with the evidence chip. Under it the stale band on `issue.warningTint` with the stale glyph: "Model changed since Run 03: proposal P-12 row 1 accepted at 16:31 (variable spring at node 80). The values below are from the model as solved; Run 03 stays the solve basis until the next run." and "Run again". A caption line naming the library: Vendor-A springs · user import 2026-09-15 · 24 sizes · source recorded in Libraries · the design rule · max variation 25 %. The table: Node, Tag, Type, Design load [N], Travel [mm], Library, Size, Rate [N/mm], Cold load [N], Hot load [N], Variation [%]; H1 at 60 designed in Run 03 (5980 N, +5.9 mm, A-3, 120 N/mm, 6688 / 5980 N, 11.8 %) with its numbers hatched in `stale.stripe`; a second row for node 80 (RS-02, now a variable spring by acceptance, the accepted diamond in the gutter) with "—" in every design column: not yet designed. Footer: 2 hanger locations · 1 designed in Run 03 · 1 not yet designed · 1 proposal row pending. The rail's Results caption "Stale"; the toolbar's Agent toggle pressed. The agent column on the Proposals tab (Conversation · Proposals 1 · Checks · Accepted 2), state "Idle · P-12 open": the card "Replace the rigid support at node 80 with a variable spring", the engineer's question quoted, "P-12 · draft until accepted · 2 rows · 16:22"; row 1 (Restraints · node 80) collapsed to one line with the done check, "Type Rigid (Y) → Variable spring +4", "accepted 16:31 · Undo"; row 2 (Restraints · node 20, pending) as the diff table Field / Old / New with Gap 3 mm → 0 mm and the Accept and Reject icon buttons; Rationale; Constraints considered (four); TBD (two, with the warning glyph); "Schema and constraints: passed"; the consequence line ("Accepting a row changes the model: current results are cleared and the run is kept as historical."); "Reject remaining" and "Accept remaining (1 row)". Under the card the Accepted record line (P-09 at 11:40, P-12 row 1 at 16:31) and the "Ask the agent…" line. Status bar without a chip (see Q-4), Issues 1, "Node 80 · 1 row".

**Tests §8.** State 8: the hanger table against a user-supplied library; Accept row by row; the Accepted record.

**Decisions.** The stale band on the results header after a model change and the hatched numbers (the design system defines the stale stripe for cells and the historical band for reopened runs, not this case). An accepted spring location appears in Hangers as "not yet designed" before the next run. The library caption line. An accepted row collapses on the card to a one-line summary with Undo; the pending row keeps its diff. "Accept remaining (n rows)" and "Reject remaining" once a row has been decided. The Accepted record surfaced as a line under the card as well as the Accepted tab. The rail caption "Stale".

### s8_model_light — state 8, Model · Model · light

**Shows.** The moment P-12 lands (16:22, before any acceptance). The canvas with node 80 selected: the rigid rod stays solid and beside it the proposal ghost, a spring can and rod in dashed `canvas.proposalGhost` at 0.7 with its plate "Variable spring · P-12" (ghost edge) under the rod's own plate; loads in the neutral vector colour. The drawer on the Restraints tab: all seven restraints; rows 20 and 80 banded in `proposal.band` with the `proposal.bar` and the proposed diamond in the gutter; on 80 Type "Rigid (Y)" struck in `proposal.old` beside "Variable spring" in `proposal.new`, Library "— Vendor-A springs", Max variation "— 25"; on 20 Gap "3 0"; proposal ticks in `proposal.new` on the changed cells. Footer: 7 restraints · 2 proposed rows · P-12 · draft until accepted; chips Accept row ⌘⇧A, Reject row ⌘⇧R, Origins. The inspector for node 80 with the restraint row and, under it in `proposal.new`, "◆ Proposed · P-12 · Variable spring · Vendor-A · 25 %". The agent strip with the badge 1. Status bar: "Mechanics solved", "User rules checked" (Run 03 still current), "Node 80 · 1 row · 2 proposed rows".

**Tests §8.** State 8, Model: a proposal landing as banded rows with old and new values and as a ghost in the canvas.

**Decisions.** The ghost is drawn beside the current glyph, not over it, 24 px along the run, so both read. The drawer's Restraints table drops Connecting node and Note (all "—" or secondary) to fit 1000 px with the wider Type needed for old and new side by side. The inspector's proposed line under the current restraint.

### s9_table_light, s9_table_dark — state 9, Review · Table · light and dark

**Shows.** The Review page as the whole surface: the header "Review · Loop 4 header · Run 04", the iteration combobox "Iteration 2 · 09:40 · Run 04", "compared with Iteration 1 · 2026-09-17 15:40 · Run 03", the Show edits switch on, Report preview and Export… at the right; under the header the acceptance sentence once (M-02, Review surface class) and nowhere else. Three columns at 280 / flexible / 320: the outline (1 Notice, 2 Identity, 3 Libraries and rule packs, 4 Model — current, 5 Load cases, 6 Assumptions and warnings, 7 Results · Stresses, 8 Results · Restraint loads, 9 Hanger selection, 10 Review/signoff block) with a lock glyph on the fixed sections and a state word on the others (edited, drafted, live, empty). The content at §4 Model: the engineer's text with the inserted text underlined in `accent.text` and the removed text struck in `proposal.old` (the rigid rod becoming the spring at 80; the nozzle load 7.4 → 3.9 kN), a live block "Live from the model record · Restraints · nodes 20, 60, 80 · not editable here" with the lock glyph, the inserted/removed legend, and the three rows (20 carrying the comment glyph in its gutter, 60 and 80 with the accepted diamond); then §5 Load cases (drafted) with its paragraph and the live Cases table. The comment stream: Comments 4, "+ Comment", filter chips (All 4, Open 3, Resolved 1, Checks 2, Notes 1, Mine 1), four cards each with its kind chip, its reference link, author and time, text, and Resolve / Reopen. The agent strip with the badge 1 (P-12 row 2 still pending). Status bar: "User rules checked", "Human review required", Issues 1, "Section 4 · Model". The dark frame is the same screen under the dark tokens. The outline's row 10 carries the registered name of the required report content item, "review/signoff block" (RESEARCH-C `C_ui_constraints.md` §4 row M-07, from `docs/PRD.md` §19.2), capitalised as a heading; the design system's §5.5 outline writes "Review and sign-off block", which is its own wording and not registered (P-10).

**Tests §8.** State 9: the three-column Review page with live tables, edits shown and hidden, two iterations, the comment stream, and the acceptance sentence once at the top.

**Decisions.** "Report preview" and "Export…" in the page header. The "compared with" line beside the iteration combobox. State words in the outline and the lock glyph on fixed sections, with the note "fixed sections come from the record". The live-block caption's wording and the inserted/removed legend on the first live block. The comment card anatomy (kind chip, reference link, author · time, text, one action). The comment glyph in the gutter's state slot of a referenced live row. The Notice section (which carries M-03 with the old product name) is not the open section, so no frame renders it.

## 3. Departures from the design system, with reasons

- P-1 Table header padding 5 px instead of 8 and header units at `type.size.caption`: the 56 px read-through columns cannot hold "P1 [bar]" at body size with 8 px padding; the DS names the widths as defaults and resizable, so the mocks keep the widths and tighten the header.
- P-2 The inspector's label column is 88 px (the specimen's rows read as 120): in the 300 px slide-over a 120 px label leaves 156 px for values like "R. Tufts · 2026-09-17 14:02".
- P-3 The failure banner appears once, across the top of the Results page, not also across the drawer (§5.2 says both): in Both view both are visible at once, and the drawer's selected Nonlinear row carries the same link.
- P-4 Column widths for tables the design system does not size (restraints, hangers, stresses' Element and Allowable, cases' Origin) are set here to fit their headers and values; listed in `tools/ui.mjs`.
- P-5 The Both-view canvas of state 4 is drawn as the engineer would have panned it (fitted to the part not under the slide-over), not re-fitted by the product.
- P-6 The empty marks column in states 1–3 is drawn with its header "Marks" and no glyphs, as §8 asks; the read-through cells of the start node are empty rather than sunken.
- P-7 The Model-stage tab strip in the Table view is the same strip as the drawer's (Layout · Restraints · Node data), which §5.3 gives for the drawer; the Loads and Results strips likewise.
- P-8 Two view azimuths (D-8) rather than one preset, for this sample's geometry.
- P-9 The draft ghost tube is drawn at the tube's diameter with a dashed stroke; the DS's "dashed accent" leaves the stroke width open (see Q-12).
- P-10 The Review outline's row 10 is "Review/signoff block", the registered wording of the required report content item (RESEARCH-C `C_ui_constraints.md` §4 row M-07; `docs/PRD.md` §19.2), where §5.5 of the design system writes "Review and sign-off block" and "the sign-off block". The design system's is its own rendering — no registered source, the report notice template and the claims registry included, carries "sign-off" — so the frames use the registered name; §7.1's ban on "Sign off" concerns proposal controls, and the report item is a section heading, not a control.

## 4. Questions for the owner that only a screen can raise

- Q-1 State 4 (Both): the 300 px slide-over hides half the 603 px canvas. Is a slide-over right, or should the inspector dock and the canvas shrink to 303 px while it is open (open item 3)?
- Q-2 States 4 and 5: no status chip when the model is complete but unsolved. Show nothing, keep "Model incomplete" until the first solve, or is a seventh short label needed (decision-packet item 4)?
- Q-3 States 7 and 9: two chips side by side (one per authority domain). One chip (the latest), or two, or all three including the always-true "Human review required"?
- Q-4 State 8 (Table): after a model change the chips are dropped and the header band plus the rail caption "Stale" carry the fact. Should the status bar say something instead?
- Q-5 State 6: the Run button's popover as the run log hides the tab strip and part of the HUD. Keep it on the button, or move the log into the Issues drawer or the Run record page?
- Q-6 State 6: one failure banner (page) plus the drawer row, or the banner in both places as §5.2 says?
- Q-7 State 1: the required marks sit on row 20 (the first element) and row 10 shows "—". A CAEPIPE veteran may expect the first row to carry Section, Material and Load. Which does the owner want?
- Q-8 State 7 (Table): with Envelope on the case selector is shown disabled ("Case: all"). Hide it instead?
- Q-9 State 7: the run identity line (hashes, solver, pack, settings) inside the results header disclosure. Keep it there, or in the Run record only?
- Q-10 State 9: "Report preview" and "Export…" in the page header, and the always-true "Human review required" chip on the Review page. Right place, right chip?
- Q-11 State 7 dark: the light end of the result scale on the dark canvas reads bright next to the tube edge lines. Acceptable under the contrast findings (M-16), or should the dark scale be re-anchored?
- Q-12 State 2: the draft ghost at tube diameter reads as a run of dashes. Keep, or a thin dashed centreline with a ghost outline?
- Q-13 State 8 (Model): with sixteen node plates and seven glyph plates the header run is dense. Should node labels hide by default above a density and return with L?
- Q-14 State 5: the combination editor as a cell popover. Is a popover the right home, or a drawer under the row?

## 5. Tokens used, and gaps

All colours, type sizes, spacings and layout values in the frames resolve to `tokens.css`, generated from the design system's `tokens.json` with its `gen.mjs` mapping (88 colour tokens, version 1.0) and inlined in every frame (kept in `frames/` only as a build source). No new colour appears. Derived values: result colours between the seven scale steps by `color-mix(in oklch, …)`; the running button's progress fill as `text.inverse` at 22 % over `accent.fill` (drawn in the CSS, not in a frame).

Gaps (nearest existing token used; a token proposed):

- G-1 A band colour for "results stale after a model change" (state 8): `issue.warningTint` is used. Proposed `stale.band` / `stale.ink`.
- G-2 The rail caption "Failed": `status.failedInk` is used; "Stale": `issue.warning`. Proposed to name rail caption colours by state.
- G-3 The draft row's gutter bar: `canvas.draft` dashed is used. Proposed `draft.bar` for tables.
- G-4 The routing hint strip and the keys hint use `canvas.labelBg`; a `canvas.hint` token would separate them from node plates.
- G-5 The pressed Agent toggle in the toolbar uses `selection.band` / `accent.text` (the segmented control's pressed style). Proposed a `pressed` state for toolbar buttons distinct from the segmented control.
- G-6 Ratio data bar track: `surface.sunken` is used. Proposed `bar.track`.

## 6. How the frames were composed

Static frames draw several moments together where the storyboard describes them in sequence; each is named so the owner can discount it:

- s3: the edit state on 130's DY and the paste band are two moments (typing, then ⌘V on the next empty row).
- s4_table: both joined rows are open at once.
- s4_both: the hover card on the restraint mark and the slide-over inspector are shown together.
- s6: the Run popover (the log) and the status chip's popover are both open; the drawer is open and filtered.
- s7_both: the probe is pinned and the selection is the same element.
- s9: both frames show edits on; the "edits hidden" state is the same text without the underlines and strikes, not drawn separately because the dark frame must be the same screen as the light one.

The generator (`tools/`: `model.mjs`, `canvas.mjs`, `ui.mjs`, `frames.mjs`, `build.mjs`, `render.mjs`) is the working record of every value and position. The frames it emits are self-contained: `build.mjs` inlines the generated `tokens.css` and the shared `mocks.css` into one `<style>` block in every frame and in `index.html`, so each file renders from its own bytes — the owner's review surface renders a local HTML file without resolving a relative `<link>`, as ROOT reported on the first delivery. The two `.css` files stay in `frames/` only as the build's sources; the only script in a frame is the theme switch and the scaling.
