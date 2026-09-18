# MOCKS V4 — the fourth pass: the frames after the owner's direction on the contradictions

Child: HELPS_HUMANS design manager, working alone (no delegation), the MOCKS-03 child resumed. Brief: `{RUN}/briefs/MOCKS-04_contradictions.md` (2026-09-18). Sources: the owner's direction as ROOT recorded it (`{RUN}/instances/ROOT/OWNER_DIRECTION_2026-09-18_CONTRADICTIONS.md`, all three sections), design system V1.3 (§9 rows 91 to 111; `tokens.json` stays 1.2) and UX specification V1.2 (§13 rows 35 to 47). This document records per frame what changed and its source, the outcome of C-17 to C-25, the state of Q-23 to Q-30 and of the frames' gaps G-13 to G-20, and what is new. `MOCKS_V1.md`, `MOCKS_V2.md` and `MOCKS_V3.md` are unchanged and remain the history.

Short names: "DS" is the design system at V1.3; "spec" is the specification at V1.2; "row *n*" is DS §9's row; "spec row *n*" is the specification's §13 row; an item number alone (5, 15) is one of the first sixteen contradictions.

## 1. The rename and the count

Eighteen frames, as in the third pass. **`s1_table_light` is retired and `s1_both_light` takes its place** (C-22; spec §10.1 and spec row 42; DS §8): a new project opens the Model stage in Both view, so state 1 is drawn there. The old frame and its screenshot are deleted by `tools/build.mjs`, which removes any frame file and shot not in the frame list; git history keeps them. No Table-view frame of state 1 remains.

## 2. Common to every frame

- **The display units selector** is the 80 px control that reads its choice, "SI" (row 91; contradiction 9). No frame opens its menu, so the third choice, "As entered", is not drawn; the brief asked for it only where a menu is open.
- **The inlined stylesheet** changed (`frames/mocks.css`: the run text button, the 80 px units control, the edit chip's buttons, the tighter filter row), and every frame inlines it.
- For those two reasons **no frame is byte-identical to the third pass**. `frames/tokens.css` is byte-identical, because `tokens.json` did not change, and so is `sample_model.md`. Frames that changed by these two common items and nothing else: `s2_model_light`, `s2_model_dark`.
- **Tooltips (15, variant A; row 106; spec row 45).** Every tooltip a frame carries names the control and then its accelerator in parentheses. Changed strings: the marks' "… · Open the row (⌘↩)" (all state-4 frames, and the visible tooltip in `s4_both_light` and `s4_both_light_column`); the proposed row's "Proposed by the agent · P-12 · Accept (⌘⇧A) · Reject (⌘⇧R)"; "Accept row (⌘⇧A)" and "Reject row (⌘⇧R)" on the footer's chips and the card's icon buttons (both state-8 frames); the drawer's close control is "Close (⎋)". The joined rows' and the combination editor's captions drop "⎋ closes"; the key is in the close control's tooltip, "Close (⎋)" (`s4_table_light`, `s5_table_light`). The hint strip of state 2 and the hint line of state 1 are not tooltips and keep their accelerators (DS §7.6).

## 3. The frames

### s1_both_light — state 1 (new name; C-22)

- Both view: the tables pane at 737 px with the layout table (read-through off, so that it fits the pane) holding the start row and row 20 with DX in the editing state and the three required asterisks, the "Add row" line and the hint line, which wraps to two lines at this width; the canvas at 603 px with the ground grid, the triad, the scale reference and node 10 as a point (spec §10.1).
- The Inspector toggle is available and unlatched, because this is Both view; the third pass drew it disabled in Table view.
- The edit chip reads "Editing DX · node 20" with the text buttons "Apply" and "Cancel" (5; row 102; spec row 40).
- The Run button's tooltip is unchanged in words and now hangs under the button, right-aligned to it, so that it lies over the table's tab strip and not over the HUD (departure D4-1).
- One chip, Solver · Model incomplete; Deformation and Probe disabled with "Needs a current run".

### s3_table_light — state 3

- The paste band's buttons read "Cancel" and "Paste 3 rows", names only, keys in the tooltips (C-20, variant A; row 107; spec row 46). They were already so in the third pass; confirmed and the frame's index text now cites the ruling.
- The edit chip's buttons are "Apply" and "Cancel" (5).

### s4_both_light, s4_both_light_column, s4_both_light_slideover, s4_table_light — state 4

- The marks' tooltips in the ruled form (§2). `s4_table_light`: the joined rows' captions without the key.

### s5_table_light — state 5

- The combination editor's caption without the key (row 106).

### s6_both_light — state 6

- The drawer's header is, left to right, the title with the count, the class chips, the "Filter" menu button and the close control (C-24; row 96). No filter of the menu is set in this moment, so the button reads "Filter" and is not latched; the class filter the failed run sets is a chip, not an entry of the menu.
- The failed run's header carries the run menu too (C-17; row 95; spec §10.3: the run menu is how an earlier solved run is reached after a failure).

### s7_both_light, s7_both_dark, s7_table_light, s7_both_light_historical, s8_table_light — the results header

- The run text is the run menu in DS's anatomy: a text button in the secondary text colour with the 12 px expanded chevron after it and the tooltip "Runs" (row 95). The third pass drew a hand-made chevron and a longer tooltip.
- In the 737 px pane the header wraps exactly as row 95 now rules: the Run identity control on a second line, right-aligned, nothing dropped (`s7_both_light`, `s7_both_dark`).
- C-25 confirmed: the Historical frame's note card reads "Historical saved run · Run 03" and its popover "Recorded on Run 03"; no frame names Run 02 as a Historical run.

### s8_table_light — state 8, Stale

- No State column (C-21; spec row 37; it never had one). The state is now said in both places the specification names: the footer reads "1 stale · 1 not yet designed" and keeps those counts while a row is selected, and the hanger selection's caption ends with the row's own state, "Hanger selection · node 60 · Vendor-A springs · user import 2026-09-15 · Stale". The third pass's footer said "1 designed in Run 03", which was wrong under a Stale run.

### s8_model_light — state 8 (Model)

- The ruled tooltips only (§2).

### s9_table_light, s9_table_dark — state 9

- Three status chips: Solver · Mechanics solved, Rule pack · User-rule checked, Human · Human review required (C-23, variant B; row 108; spec row 47). Every other frame has at most two (`report.json` `lint.chips`; the lint's expectation per frame is 3 for the two Review frames and 0, 1 or 2 elsewhere).
- The Kind button reads "Kind: all" (C-18; row 94; spec row 36). In the dark frame the open menu has "All kinds" first, carrying the check because nothing is chosen, a separator, then the four kinds with their counts; the third pass had it last.
- "Kind: all" is wider than "Kind", and the filter row no longer held in the 320 px column: this pass's first screenshot showed the combobox's chevron and the menu's counts cut off. The row's chips are tightened (departure D4-2; Q-31).

### index.html

Regenerated: the fourth pass's introduction, `s1_both_light`, and the rows whose text names a changed item.

## 4. Departures, with reasons

| # | Departure | Reason |
|---|---|---|
| D4-1 | The Run tooltip in `s1_both_light` hangs right-aligned under the Run button. | Left-aligned it covered the canvas's HUD, which state 1 now has. |
| D4-2 | In the Review page's filter row the chips have 5 px side padding and 4 px gaps; DS's chip has more. | With DS's chip the four chips and "Kind: all" need about 338 px and the column gives 296. Q-31. |
| D4-3 | The empty model's camera in `s1_both_light` is fitted to the whole sample model's extent, so node 10 sits where the later states draw it. | Neither document says what an empty canvas is fitted to; one node has no extent. G-21. |
| D4-4 | `s8_table_light` keeps the state counts in the footer while a row is selected. | C-21 puts the state in the footer; the footer rule lets counts give way to the selection group (Q-30). The state was kept, because it has no other home in the table. |

The third pass's departures D3-2 to D3-8 stand. D3-1 is no longer a departure: row 95 makes the wrapped header the rule.

## 5. C-17 to C-25: outcomes

| # | Outcome | Drawn |
|---|---|---|
| C-17 | Kept function; DS gains the run menu (row 95). | The run text button with chevron, tooltip "Runs", on every results header, the failed run's included. |
| C-18 | The specification follows DS: "Kind: all" (spec row 36). | Both `s9` frames. |
| C-19 | No product effect (ROOT). | Eighteen frames. |
| C-20 | Ruled A: names only (row 107). | `s3_table_light`, as before. |
| C-21 | The specification follows DS: no State column (spec row 37). | `s8_table_light`: footer and caption. |
| C-22 | A new project opens in Both view. | `s1_both_light`. |
| C-23 | Ruled B: three chips on Review (row 108). | Both `s9` frames. |
| C-24 | Kept function; DS gains the Filter menu button (row 96). | `s6_both_light`. |
| C-25 | Run 03 (row 103; spec row 44). | Confirmed in `s7_both_light_historical`. |

Of the first sixteen, the frames are touched by 5 (Apply) and 15 (tooltips), both drawn; 9 by the units control's width; 4, 1, 2, 13 and 16 change no frame; 10 and 11 (the View tool's menu, the stress components) are drawn by no frame, because no frame opens them.

## 6. Q-23 to Q-30 and G-13 to G-20, re-stated

| # | State |
|---|---|
| Q-23 (selection group on read-only results tables; "Unchecked rows only" there) | Open. V1.3 does not speak to it. |
| Q-24 (the slide-over hides the selected node and most of the HUD) | Open. |
| Q-25 (the Review header needs more than 1340 px) | Open. The compared-with combobox still truncates. |
| Q-26 (the results header in the 737 px pane) | **Closed by V1.3** (row 95): the header wraps as drawn. |
| Q-27 (the toast over the footer's right-hand chips) | Open. |
| Q-28 (the run log covers the HUD) | Open. |
| Q-29 (data bars in the result colour under a Historical run) | Open. Row 93 says what the stress components do under a Stale and a Historical run, not what the bars do. |
| Q-30 (the footer's counts give way whenever a row is selected) | Open, and sharper: C-21 now puts the hanger's state in those counts (D4-4). |
| G-13 (the select-all control's glyph) | Open. |
| G-14 (hanger selection's marker for the size in use and its Select control) | Open. |
| G-15 (the Conversation tab's message anatomy) | Open. |
| G-16 (the mechanism and threshold for `canvas.edgeAlt`) | Open. |
| G-17 (label and plate density in a small fitted figure) | Open. |
| G-18 (the drawer's further filters) | **Closed by V1.3** (row 96). |
| G-19 (the iteration menu's anatomy; where "Current" appears) | Open. |
| G-20 (the run menu's component) | **Closed by V1.3** (row 95). |

The frames' gap numbers are the frames' own series. The specification has a gap series of its own with the same letter (its G-22 is the engine gap that reserves the Connecting node column), and the two should not be read as one list.

## 7. New in this pass

**A difference between the two documents, reported and not resolved.**

- **C-26.** Spec §13 rows 36 and 37 cite the direction's items the wrong way round: row 36 (the Kind button) cites "direction (C-21)" and row 37 (no State column) cites "direction (C-18)". In `MOCKS_V3.md` §6, in ROOT's direction record and in DS row 94, C-18 is the Kind button and C-21 is the State column. The rules themselves agree in both documents; only the citations are crossed. No frame is affected.

**Questions.**

- **Q-31.** The Review page's filter row with "Kind: all" does not hold in 320 px with DS's chip (D4-2). With a kind chosen the button reads longer still ("Kind: Evidence summaries 1" is about 170 px), and then no tightening will hold the row. Does the Kind button truncate, or does the row wrap?
- **Q-32.** The Connecting node column. Contradiction 8's outcome is that DS keeps the column as drawn appearance and the specification reserves it until the engine supports it (row 104; spec row 41). `s4_table_light`'s joined Restraints row still draws the column, holding "—"; `s8_model_light` does not (it never did, for width). Is a reserved column drawn empty, or absent until it is offered? The frames were left as they were.
- **Q-33.** State 1 in Both view shows the canvas with a HUD of ten tools over an empty model, of which two are disabled. Are Section, Isolate, Hide and Labels live with nothing to act on?

**Gaps.**

- **G-21.** The camera of an empty model: what the canvas is fitted to when the model has one node and no extent (D4-3), and the scale reference it then shows.

## 8. The lint

`tools/render.mjs` gained three checks, modelled on the design system's `tools/agree.mjs`: a tooltip (any `title` attribute, any SVG `<title>`, any drawn tooltip) that writes a key outside parentheses; a control whose face carries a key (the hint strip and the palette field excepted); and "Commit" as any control's text, tooltip or label. "Commit" is back in the list of barred labels and "Apply" is out of it (row 102). The bar-overflow check now covers the Review page's filter row and the drawer's header. `MOCKS_V3.md` joins the history files that the retired-string search does not read.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
