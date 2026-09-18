# MOCKS V2 — the second pass: eighteen frames drawn from design system V1.1

Child: HELPS_HUMANS design manager, phase 3, working alone (no delegation). Brief: `{RUN}/briefs/MOCKS-02_regeneration.md` (2026-09-18). This document records, per frame, what changed from the first pass and its source (the direction record §11 decision number, the design system V1.1 §9 change-log row, the UX specification section); every departure from V1.1 or the specification with its reason; the decisions the frames made that neither document states; the questions only a screen can raise, numbered from Q-15; and the token or component gaps found, numbered from G-7. `MOCKS_V1.md` is unchanged and remains the history of the first pass; its fourteen questions are answered by the direction record §11 and are not asked again.

Frames: [`frames/index.html`](frames/index.html). Screenshots and the measured facts: `shots/` (`report.json`). The generator: `tools/`. The sample model: [`sample_model.md`](sample_model.md), unchanged (no new state needed a value).

## 1. Common to every frame

**Tokens.** `frames/tokens.css` is regenerated from `tokens.json` 1.1 (98 colour tokens; V1 had 88) with the design system's `gen.mjs` mapping and inlined into every frame. The ten tokens V1.1 added are all used: `stale.band` / `stale.ink` (§9 row 38, s8_table), `rail.captionFailed` / `rail.captionStale` / `rail.captionHistorical` (row 39, s6 and s8_table; Historical is the rail caption's default colour in `mocks.css` and no frame shows it), `draft.bar` (row 40, s2 and s3), `canvas.hint` (row 41, s2), `pressed.fill` / `pressed.ink` (row 42, the Agent toggle in s8_table and every latched HUD tool), `bar.track` (row 43, every ratio data bar and the probe's), and the re-anchored dark `result.scale.1–7` (row 11, decision 11, s7_both_dark). `layout.canvas.min` 220, `layout.inspector.label` 88, `layout.cellPadding` 8 and `layout.headerPadding` 5 (rows 1, 34, 35) are emitted and the frames draw those values.

**Stylesheet.** `frames/mocks.css` is revised to the V1.1 specimen's rules: the rail captions in their own tokens (row 16, 39); the data bar's track on `bar.track` (row 43); the latched Agent toggle as `.btn.latched` on `pressed.fill` / `pressed.ink` and the HUD's latched tools on the same pair, distinct from the segmented view switch (row 42); the hint strip as a 22 px strip on `canvas.hint` with 12 px `text.secondary` (row 41, §5.6); the draft row's dashed 3 px `draft.bar` at the gutter's left edge and the typed value in `accent.text` (row 40, §5.1); the stale band (row 38, §5.1); the disabled combobox (row 8); the failure banner and the empty state at the specimen's anatomy (row 6); the run log's rows and footer (row 5); the results header disclosure at the specimen's 520 px; the Both-view inspector docked beside the canvas at `layout.inspector.both` (rows 1, 48), the slide-over rule kept only for the fallback no frame draws; the HUD's two-row form for a canvas under 400 px (row 1, §0); the Review outline's note row (row 33, the specimen's Review outline).

**The shell.** Geometry as V1 (§0): toolbar 48, rail 56, status bar 24, agent strip 44 or column 340, Model-view inspector 340, table drawer 280, issues drawer 200, Both view 737 / 603 with the strip; measured in `shots/report.json` (`regions`, `docked`). New in this pass: in Both view the inspector docks on the canvas's right edge at 300 px and the canvas draws in the remaining 303 px while the table keeps 737 px (decision 1; rows 1, 48). The Run button's tooltip names the blockers of the solve in one line (row 23; s1). The status chips follow the policy of V1.1 §2.3 and the specification §5.4: one chip while the model cannot be solved (s1, s2, s3), none when it is complete but unsolved (s4, s5; decision 2), two after a solve, one per authority domain (s7, s8_model; decision 3), none after the model changed (s8_table; decision 4), User rules checked and Human review required on the Review page only (s9; decision 3), and none after the failed run (s6; §2 below).

**The canvas.** The figure generator (`tools/canvas.mjs`) gained three things this pass. A camera option that draws the figure at the scale and centre of the canvas the engineer set it in and pans only as far as needed to keep the selected node in view, which is decision 1's rule for the docked inspector (§0: "keeps its camera scale and centre"); plates and labels of nodes outside the panned view are not drawn, so nothing is clamped to the edges. The draft ghost of decision 12: a faint tube outline at the draft's diameter in `canvas.draft` at 30% and 1 px, a thin dashed centreline of 1.5 px with a 6/4 dash, and the draft node's plate with a `canvas.draft` edge (row 12, §5.6). The label budget of decision 13 (§6.5, row 13): a plate that cannot be placed without overlapping yields, except the current row's node, which is always labelled; the triad and the scale reference are registered as occupied before the labels are placed, which the first pass did not do and which let a node label land on the "1000 N" text in s6.

## 2. The frames

The fifteen first-pass frames are regenerated in place; the three new frames are `s4_both_light_column`, `d71_item1_status_bar_light` and `d71_item2_results_caption_light`. A frame not named below changed only by the common items of §1: `s4_table_light`, whose two row expansions V1.1 adopted as row 27, and `s8_model_light`, whose proposal ghost, drawer columns and inspector line are row 32.

### s1_table_light — state 1

- The Run button's tooltip is one line, "Run is unavailable — Section missing at node 20, Material missing at node 20" (V1.1 §5.2, §7.1; row 23). The first pass broke it over two lines.
- The start row's dashes, the required marks on row 20 and the empty marks column are as the first pass drew them, now the design system's rule (decision 7; row 7).
- One chip, Model incomplete (decision 2; V1.1 §2.3). The information popover with the maturity sentence as the M-01 home until packet item 1 is ruled (V1.1 §7.4).

### s2_model_light, s2_model_dark — state 2

- The draft ghost is decision 12's three parts (row 12; V1.1 §5.6, §6.9); P-9 of the first pass is superseded. The compass keeps a 3 px `canvas.draft` dot at its origin, which is the compass's and not the ghost's (frame decision F-12).
- The hint strip under the HUD is on `canvas.hint`, 22 px, 12 px `text.secondary`, "Route · ↩ places 50 · ⇥ next axis · − reverses · ⎋ cancels" (rows 24, 41, G-4 closed; §5.6). The first pass drew it at 11 px on `canvas.labelBg`.
- The draft row in the drawer carries the dashed `draft.bar` and its typed value in `accent.text` (rows 24, 40, G-3 closed; §5.1 Draft).
- The dark frame tests the ghost's outline and centreline against the dark ground (V1.1 §8, first item).

### s3_table_light — state 3

- The paste band's three preview rows are draft rows on `draft.bar` (rows 25, 40; §5.1 The paste band). Otherwise as the first pass, whose editing cell's unit at caption size, header padding and propagated tick are now rows 25, 34 and 15.

### s4_both_light — state 4, the inspector docked

- The inspector docks on the canvas's right edge at 300 px and closes with ⌘I or ⎋ (decision 1; rows 1, 48; V1.1 §0, §5.3; specification §5.1). The table pane stays 737 px (measured: tables 737, canvas 303, inspector 300, strip 44). P-5 of the first pass (fit to the visible part) is superseded by the camera rule: the canvas keeps the camera it had at 603 px and would pan only to keep node 20 in view; node 20 is in view at pan 0, so the figure is cropped at both edges exactly as the rule says (Q-20).
- The HUD wraps to two rows because the canvas is under 400 px (§0, §5.6); measured `hudRows` 2. The mock line moves under the HUD (departure D2-9).
- The glyph plates are no longer forced to the node's left side (row 26 was written for the slide-over that covered the canvas; the docked inspector covers nothing) — departure D2-8.
- The hover card on the open restraint mark, the checked and stale marks in the gutter and the read-through group off with Type at 96 are as the first pass (row 26).

### s4_both_light_column — state 4, the agent column was open (new)

- Draws the specification §10.9 rule as V1.1 §0 and row 48 adopt it: with the agent column open the surfaces were 1044 wide, the split 574 / 470, and docking the inspector would have left the canvas at 170 px, under `layout.canvas.min` 220; so the column collapsed to its strip first, the surfaces are 1340 again and the split is as with the strip (737 / 303 / 300 / 44). The canvas is never collapsed.
- What the static frame carries as evidence of the transition: the canvas keeps the camera it had in the 470 px canvas (the model is drawn smaller than in `s4_both_light`; frame decision F-2), the Agent toggle is not latched because the column is closed, the strip is shown, and a toast names the collapse and the shortcut that reopens the column: "Agent column collapsed to its strip: the docked inspector would leave the canvas under its 220 px minimum. ⌘⇧G reopens it" (frame decision F-3; gap G-8). The label "80" yields to the Guide plate under the budget rule (F-9).

### s5_table_light — state 5

- The combination editor is a row expansion under OCC1 with the joined-row anatomy (decision 14; row 14; V1.1 §5.1; the specimen's combination editor): the caption "Combination editor · OCC1 · row expansion · ⎋ closes" with the expanded chevron, the case's terms as chips (W + P1 + SE1 0.3 g X), the available terms dimmed after them, the Stress type and Rule comboboxes with the pack named ("Rule: OCC-A1 · sample-rules 1.2"), Cancel ⎋ and Done ↩ on a second line. The first pass's cell popover is overruled. The Expression cell keeps the focus and selection rings.
- The footer carries "1 expansion open" beside Origins (row 27, as the specimen); the status bar's selection reads "Case OCC1 · Expression · expansion open" (F-11). The Loads header band, the M-15 caption and the Origin column's wordings are as the first pass (row 28).
- The "+ term" control of the first pass's composer is not in V1.1's anatomy and is dropped (D2-6).

### s6_both_light — state 6

- The run log is the shorter popover hanging from the Run button (decision 5; row 5; V1.1 §5.2): 360 px, the title "Run 02 · 15:02:14–15:02:41 · settings S-02", four rows (Assembled 0.3 s; W, SUS solved 0.8 s; OPE1 stopped at iteration 50 · node 20 with Show node 20; OPE2, EXP1, EXP2, OCC1, hangers, rules not run) and the footer Run settings…, Run record, Run again. The first pass's ten-row log is replaced. It still covers the tab strip's right end and the HUD's first buttons (D2-4, Q-16).
- One failure banner across the top of the page, "Run 02 failed: nonlinear support at node 20 did not converge." with Show node 20, at the specimen's anatomy; the drawer opens filtered to the Nonlinear class with the clearing chip "Nonlinear 1 ×" and "All classes 2", and its selected row carries the same link (decision 6; rows 6, 47; §5.3). The empty state is §5.3's sentence verbatim, left-aligned as the specimen draws it (D2-5).
- **No status chip.** The first pass showed "Model incomplete" with its popover after the non-convergence. The specification §5.4 item 5 says the chips after a failed run are whatever statuses the run record carries and that a stopped run which carries none shows no chip; what a run that stops at the iteration limit emits is the open engine question §11 Q11; so the frame draws no chip and no chip popover. The rail caption Failed in `rail.captionFailed` and the banner carry the state (D-3 adopted, rows 16, 39; Review disabled with "No solved run").

### s7_both_light, s7_both_dark — state 7

- The ratio data bars sit on `bar.track` in the table and in the probe (row 43, G-6 closed). The results header disclosure is open with the acceptance sentence once and the run identity line under it (decision 9; row 9), at the specimen's 520 px width (D2-11). Two chips, one per authority domain (decision 3). The legend's quantity line, the pinned probe 18 px from node 70 and clear of the legend, and "Sorted by ratio" are as the first pass (row 29).
- The dark frame draws the re-anchored scale (decision 11; row 11): measured steps `#045c59 … #74d1cb`, the brightest under `canvas.edge`, on the tubes, the legend bar and the data bars (V1.1 §8, fifth item).

### s7_table_light — state 7, Envelope on

- The case selector stays visible and disabled, reading "Case: all" with the tooltip "Envelope is on: the governing case per row", drawn with the specimen's `.combo.disabled` (decision 8; row 8; §5.1). The Ratio column menu and the Envelope footer chip are as the first pass (row 30).

### s8_table_light — state 8, the model changed since the run

- The stale band in `stale.band` / `stale.ink` with the Stale glyph (§3.2, `m-stalerun`), "Model changed since Run 03:" in the ink, the sentence in `text.primary` and Run again at its right (decision 4; rows 4, 31, 38; G-1 closed; §5.1). The rail's Results caption reads Stale in `rail.captionStale` (row 39; G-2 closed). No chip (decision 4). The hatched cells, the not-yet-designed row and the library caption are as the first pass (row 31).
- The Agent toggle is latched on `pressed.fill` / `pressed.ink` while the column is open (row 42; G-5 closed).
- The proposal card's consequence line is the specification's: "Accepting a row changes the model. Run 03 stops being the solve basis and is kept as a historical run; its results stay readable as stale until the next run." (specification §6.3) — departure D2-1 from V1.1 §5.4's wording. The collapsed accepted row truncates its new value (Q-18).

### s9_table_light, s9_table_dark — state 9

- The header carries the iteration combobox, the compared-with line, Show edits, Snapshot…, Report preview and Export…, as the specimen's Review header draws them (text buttons, Export… on the accent) (decision 10; rows 10, 33; §5.5; specification §7.1). "Compare with…" is not drawn (D2-2, Q-17). Export… as a second accent button on the page is the specimen's choice (Q-19).
- The outline's last row is "Review/signoff block" with the lock and the state word "empty", and the note "fixed sections come from the record" is the specimen's note row (row 37; P-10 adopted; §5.5).
- The comment stream's filter row is the specification's eight chips, All, Open, Resolved, Checks, Open issues, Notes, Drafts, Mine, with counts (specification §7.5; D2-3, Q-22).
- The acceptance sentence once at the top; the chips User rules checked and Human review required (decision 3; §7.4).

### d71_item1_status_bar_light — decision aid, D-71 item 1, option A (new)

- State 1 with the maturity sentence "Technical preview — not a released product." permanently at the right end of the status bar in the secondary text style (12 px regular `text.secondary`, between the selection and the units combobox so the information control stays last; F-6), beside the frame's current placement in the information popover, which stays open. The sentence therefore appears twice in this frame and nowhere else twice; the lint expects it.
- The label "Decision aid · D-71 item 1 · option A" sits in the frame's caption bar and in the toolbar band between the centre group and the units combobox (F-5), so the frame cannot be mistaken for the design's rule. Nothing else differs from `s1_table_light`.

### d71_item2_results_caption_light — decision aid, D-71 item 2, option A (new)

- State 7 Both with the listed short variant "Acceptance and professional judgment remain with the responsible engineer." as a one-line caption under the run identity in the results header (a 12 px `text.secondary` band under the header row, the band's rule moved under it; F-7), visible without a click, in place of the disclosure placement. The disclosure stays open holding only the run identity line so the delta from `s7_both_light` is visible (F-7). The canonical sentence appears nowhere in this frame; the lint expects the short variant once.
- The same label as item 1, with "item 2". Nothing else differs from `s7_both_light`.

### index.html

Regenerated for eighteen frames with state, stage, view, theme, what each tests (the V1.1 §8 item or the packet item) and what to look at; the two decision-aid rows carry the label under the frame name; links to the sample model, this document, `MOCKS_V1.md` and `RETURN.md`.

## 3. Departures from V1.1 or the specification, with reasons

- D2-1 **The consequence line.** The card reads the specification §6.3's sentence, not V1.1 §5.4's and the specimen's "Accepting a row changes the model: current results are cleared and the run is kept as historical." Reason: V1.1's own §5.1 stale band and decision 4 keep the values readable ("The values below are from the model as solved"), the specification restates the line in §2.6's words, the specification governs behaviour, and the sentence is not among the strings V1.1 §7 binds.
- D2-2 **"Compare with…" not drawn** (specification §7.1). The Review header at 1440 holds the title, the iteration combobox, the compared-with line, Show edits, Snapshot…, Report preview and Export… with no room for another control; V1.1 §5.5 does not list it. Q-17 asks where it lives.
- D2-3 **Eight filter chips** on the Review page (specification §7.5) where V1.1 §5.5 lists six: the specification's strings are adopted because V1.1 lacks "Open issues" and "Drafts"; they take two rows in the 320 px column (Q-22).
- D2-4 **The run log popover covers the tab strip's right end and the HUD's first buttons** although V1.1 §5.2 says it covers only the toolbar's neighbourhood: at 164 px tall (title, four rows, footer) hanging from a button that sits over the 737 px table pane, it cannot avoid the header rows under a 48 px toolbar (Q-16).
- D2-5 **The empty state is left-aligned** under the tab strip as the specimen draws it (`padding 16 12`, `text.secondary`, 62 ch); the first pass centred it with a bold lead.
- D2-6 **The "+ term" control** of the first pass's composer is dropped: V1.1 §5.1's anatomy lists the terms, the available terms, the two comboboxes and the two buttons; the available chips are the way to add a term.
- D2-7 **The stale band's markup** puts the bold lead-in inside the body span so the sentence flows as one paragraph with Run again on its first line; the specimen has the lead-in as a sibling, which at 1044 px wraps to three rows. The appearance V1.1 specifies (band, ink, primary text, the button at the right) is kept.
- D2-8 **Glyph plates in the docked Both view** are placed on the node's default side, not forced left (row 26 says "placed away from the docked inspector"): the docked inspector covers no part of the canvas, so there is nothing to place away from; the plates stay inside the 303 px canvas by the placement's clamp.
- D2-9 **The mock line in a canvas under 400 px** sits under the HUD instead of centred on the bottom edge (D-9): in 303 px the bottom edge holds the triad and the scale reference. The mock line is a requirement of the mocks, not a product element (row 20).
- D2-10 **The results header disclosure** is 520 px wide as the specimen's `.disc` rule sets it; the first pass used 640 in the 737 px pane. The identity line wraps to two lines.
- D2-11 **The hover card and the docked inspector** are drawn together in both state-4 Both frames, as the first pass composed them (MOCKS_V1 §6); a hover is not strictly the state the storyboard describes.

## 4. Decisions the frames made that neither document states

- F-1 The docked camera's pan threshold: the selected node is kept at least 72 px inside the canvas's side edges; the two frames needed no pan (measured `pan` 0).
- F-2 The column-open frame's camera is the one the engineer had in the 470 px canvas (fit at 470, centre kept), so its model is smaller than in `s4_both_light`; this is what the rule produces and what makes the two frames distinguishable.
- F-3 The toast: `surface.raised`, elevation 2, `radius.card`, 8 / 12 padding, 13 / 18, the agent icon, 284 px wide over the docked inspector's empty foot, with the shortcut on its own line in `text.secondary`; its text is the frame's. V1.1 §7.1 says what toasts say and has no toast anatomy (G-8).
- F-4 The two-row HUD is 156 px wide (five tools per row); the mock line then sits under it (G-10).
- F-5 The decision-aid label: a dashed `border.strong` outline, 11 px medium `text.secondary`, "Decision aid · D-71 item n · option A", in the caption bar and in the toolbar band between the centre group and the units combobox, where the band has room at 1440. A mocks-only element.
- F-6 In `d71_item1` the sentence sits left of the units combobox and the information control, in 12 px regular `text.secondary`; "right end" is read as the bar's right group, keeping the information control last.
- F-7 In `d71_item2` the caption is a 12 px `text.secondary` band under the header row; the disclosure stays open with the identity line only, which shows the delta from `s7_both_light`.
- F-8 The drawer's selected row: class, message, the link, the entity; the message truncates with an ellipsis and carries its full text in the tooltip (G-11).
- F-9 Budget: a label that cannot be placed yields except the current row's node; the triad and the scale reference are occupied first.
- F-10 The combination editor's available terms: P2, T1, T2, SUS, OPE1, OPE2 (the primitives and cases not in OCC1's expression).
- F-11 The status bar's selection in s5 reads "Case OCC1 · Expression · expansion open".
- F-12 The compass keeps a 3 px `canvas.draft` dot at the current node.
- F-13 The run log's Run again carries the run icon and is the accent button; Run settings… and Run record are plain compact buttons, as the specimen.
- F-14 The rail's disabled tooltips "No run yet" (s1–s5) and "No solved run" (s6) are attributes and not visible in a static frame.

## 5. Questions only a screen can raise

- Q-15 `s4_both_light_column`: ⌘⇧G while the inspector is docked at 303 px would take the canvas to 170 px. Does the toggle close the inspector, turn it into the slide-over for that width, or refuse to open the column until the inspector closes? V1.1 §0 and the specification §10.9 say what happens when the inspector opens, not what happens when the column reopens.
- Q-16 `s6_both_light`: the shorter run log still covers the tab strip's right end and the HUD's first buttons at 1440. Accept as drawn, hang it right-aligned to the Run button so it sits over the canvas's top edge instead of the table's header rows, or close it on the first click elsewhere?
- Q-17 `s9`: where does "Compare with…" live? The compared-with line itself as a combobox, or an entry in the iteration combobox's menu; the header has no room for a fourth button.
- Q-18 `s8_table_light`: the collapsed accepted row cannot hold "Type Rigid (Y) → Variable spring", the row count and "accepted 16:31 · Undo" in 316 px; the new value truncates. Drop the old value in the collapsed line (the diff table above keeps it), or let the line wrap?
- Q-19 `s9`: Export… on the accent, as the specimen draws it, makes two accent buttons on the Review page (Run in the toolbar). Intended, or should Export… be a plain button like Report preview?
- Q-20 `s4_both_light`: under decision 1's camera rule the docked canvas shows the model cropped at both edges until the engineer presses F. Acceptable, or should docking offer Fit (F) in the toast or refit when the whole model was in view before?
- Q-21 `s6_both_light`: with no chip after the failed run the status bar's left end is empty while the rail caption and the banner carry the state. Acceptable until the engine answers §11 Q11, or should "Model incomplete" persist from before the run until a solve?
- Q-22 `s9`: eight filter chips take two rows in the 320 px comment column. Keep all eight, or fold "Open issues" and "Drafts" (0 here) into a kind menu?

## 6. Token and component gaps

- G-7 The three Review icons of V1.1 §3.2 (Report preview, Export, Snapshot) are specified and not in the specimen's sprite; the specimen's Review header draws text-only buttons and the frames follow it. The Results Stale glyph is in the sprite (`m-stalerun`) and is used.
- G-8 No toast component: V1.1 §7.1 gives toasts' wording and §5 no anatomy; `s4_both_light_column` draws one (F-3).
- G-9 No rule for a caption band under the results header; only the decision aid needs one (F-7), and only if the owner rules for option A.
- G-10 The narrow-canvas HUD's width when it wraps (156 px, five per row here) and where the mock line goes are not specified (F-4, D2-9).
- G-11 The issues drawer row has no overflow rule for a message beside a link and an entity; the frame truncates the message with an ellipsis and a tooltip (F-8).
- G-12 The run log's title line is "13 px medium" in §5.2 with the time span in the numeric face; the row glyphs (checked, warning, none) are the frame's reading of "a glyph".

## 7. How the frames were composed

Static frames draw several moments together where the storyboard describes them in sequence; each is named so the owner can discount it: s3, the edit state on 130's DY and the paste band (typing, then ⌘V); s4_table, both joined rows open at once; s4_both and s4_both_column, the restraint mark's hover card and the docked inspector together, and in the column frame a toast that names a transition already completed; s6, the run log popover open while the drawer is open and filtered; s7_both, the probe pinned and the selection the same element; s9, both frames with edits shown (the hidden state is the same text without underlines and strikes). The two decision-aid frames are `s1_table_light` and `s7_both_light` with one placement changed each and the label added; they are not the design's rule and are not counted among the sixteen frames that draw it.

The generator (`tools/`: `model.mjs`, `canvas.mjs`, `ui.mjs`, `frames.mjs`, `build.mjs`, `render.mjs`) is the working record of every value and position; every colour, size and spacing in a frame resolves to `tokens.css` or is listed above. The frames are self-contained: one `<style>` block per file holding the generated `tokens.css` and `mocks.css`, the figure and the glyph sprite inline, and the only script the theme switch and the scaling; no frame attempts a network request (verified, `shots/report.json`).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). The frames are proposals for the owner's judgement; the sample values are placeholders and not engine output; nothing here establishes acceptance, qualification or release.
