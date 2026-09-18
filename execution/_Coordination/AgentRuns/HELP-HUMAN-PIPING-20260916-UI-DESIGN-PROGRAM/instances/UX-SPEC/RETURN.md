# RETURN — UX-SPEC-02: the UX specification revised to V1.1

Executor: HELPS_HUMANS design manager (Type 2, working alone; no delegation, no agent spawned). Brief: `briefs/UX-SPEC-02_revision.md`, sealed SHA-256 `43019b92ee8e7118db2413eaa3c7c72f8840a119226379eb406b871f5085ccfd`, verified before anything else was read. Date: 2026-09-18. Model: Claude Fable 5.1 (`claude-fable-5-1`), run through the Claude Code `Agent` tool. Paths below are relative to the run folder `{RUN}` for run documents and to the repository root for product source. This return replaces the UX-SPEC-01 return; that return's history is in the repository's history and in the brief index.

## 1. What was read

In the brief's order.

| # | Document | Read |
|---|---|---|
| 1 | `instances/UX-SPEC/UX_SPEC_V1.md`, `OPERATIONS_MAP.md`, `RETURN.md` as they stood | in full |
| 2 | `instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md` at V1.2 | the status paragraph; §0; §2.3 to §2.5; §4; §5 in full (the pointer rule and its audit, §5.1 to §5.6); §6; §7; §8; §9 from row 48 to row 89. Not read: §1, §2.1, §2.2, §2.6 to §2.9 and §3, which are appearance this document does not cite by number |
| 2 | `instances/DESIGN-SYSTEM/RETURN.md` | §4 (the key-only actions), §5 and §6 (uncertainties and correction 1) |
| 3 | `_DECISIONS/D-71_RULING_2026-09-18.md` and its two addenda; `DEC-099` to `DEC-105` in `_Decomposition/SOFTWARE_DECOMP.md` §12 | in full |
| 4 | `instances/ROOT/REVISION_PASS_PREPARATION_2026-09-18.md` | §1, §2, §6 |
| 5 | `instances/RESEARCH/G_rendering_workload_classification.md` | §2 in full; §1 was consulted through §2's citations and not read item by item (see §7 uncertainty 12) |
| 6 | `instances/ROOT/IMPLEMENTATION_HANDOFF_PREPARATION_2026-09-18.md` | §2 and §4 |
| 7 | Product source at `HEAD` (read-only) | `apps/desktop/src/App.tsx` around `commitModelAfterSolveInvalidation`, its call, `clearComputedModelState` and its callers, the menus and the status label function; `features/viewport/viewportSelection.ts`, `viewportResource.ts` and `PipeViewport.tsx` at the lines §12 cites; `features/results/HistoricalRunContext.tsx`; `features/agent-proposals/AgentProposalPanel.tsx`; `features/caepipe-mbf/CaepipeMbfExportPanel.tsx` at the cited lines; `types.ts`; the schemas and documents the map cites, at the cited lines; `docs/claims_registry.md` §1 |

## 2. What was produced, and what changed

Under `instances/UX-SPEC/` only. Nothing else was written anywhere in the repository; no build, test or dev server was run; no state-changing git command was run (`git rev-parse`, `git log`, `git diff --stat` and `git status` were used read-only). Edits were made as anchored replacements by a small script that refuses to save unless every anchor matches exactly once; each batch reported its count and none failed silently. Working copies and scripts were kept in the session's scratch directory, outside the repository.

| File | What changed |
|---|---|
| `UX_SPEC_V1.md` (V1.1; filename kept; no section renumbered) | The thirty-two changes of its §13, each with its source. In summary: the rulings applied (name, both sentences removed with M-01 and M-02 marked retired in the seventeen-row table, registered labels, hanger selection's short variant, rendered historical wording, five class words, the export named by what it writes, the Checked mark as ruled); Q-15 to Q-22 as behaviour with eight state tables or tables; the pointer rule (§2.8) with a control column in the keyboard model and a control for every key in §3 to §6; R-1 to R-5; run standing (§2.6) with a per-surface table and a transition table, and §4.3, §5.4, §6.3, §10.4 brought into agreement; the one mutation route (§2.2, §2.4); the new §12 (twenty semantic changes, both sides cited) and §13 (change log); §11 questions 8, 11, 12, 15 updated and 25 to 27 added |
| `OPERATIONS_MAP.md` | Re-verified against `HEAD`; 9 line numbers corrected and 19 ambiguous shorthand citations given their full path; rows 138, 236 retired and row 310 entered as retired; rows 164 and 166 re-classed; 36 rows added (278 to 313) in a new section S; gap entries G-30 to G-34 added and G-06, G-08, G-11, G-16 to G-21, G-26, G-28 revised; the note on which gaps the identity-layer act touches; counts restated |
| `RETURN.md` | This record |

Counts stated in the map: 313 numbered rows, 3 retired, 14 cross-references, 296 with a provider or a gap in their own cell; 34 gap entries (engine 10, typed interface 14, host 1, rendering 3, interface 6).

## 3. Citation re-verification: method and result

**Method.** (1) Every backticked `path:line` and every shorthand `:line` in both documents was extracted by script and resolved to a file: a shorthand takes the bare path in parentheses that follows it in its segment, else the last full path before it in the cell, else, inside a "host" parenthesis, the desktop host's `lib.rs`. (2) For each citation the script checked that the file exists at `HEAD`, that the line is inside it, and whether an identifier named beside the citation appears on that line or within five lines of it. (3) Every citation the script could not match by identifier was read by eye against the printed source line. (4) Independently, the 67 distinct product files the two documents cite were compared between the parent of PR #793's merge and `HEAD` with `git diff --stat`, read-only.

**Result for the map as V1 left it.** 534 citations checked. Moved because of PRs #793 to #795: **0**. The diff over the 67 cited files is empty: the three PRs changed `features/viewport/viewportSelection.ts` and the `e2e/ui-foundation` instrument, neither of which V1 cited. Corrected for an imprecision already present in V1: **9** (`types.ts` `changes` 611 → 610, the receipt's `acceptance` 797 → 798, `applied_model_hash` 796 → 797; `hanger.schema.yaml` `cold_load` 287 → 286, `hot_load` 290 → 289, `travel_range` 296 → 295; `model.schema.yaml` `Support.directions` 1877 → 1906 and the `rigid` enumeration value 295 → 314 in rows 49 and 50). Ambiguous shorthands made explicit with their full path: **19** citations in 18 rows (28, 47, 52, 59, 68, 72, 86, 109, 110, 113, 127, 129 with two, 154, 253, 254, 256, 259, 262), three of which a reader resolving "the last path" would have taken to the wrong file. Re-classed because a provider changed: **0**. Re-classed because this revision changed the control's meaning: **2** (row 164, Isolate, whose provider hides where the design dims; row 166, withdrawn unsolved drawing). Three more rows keep their class with a corrected statement of today's behaviour (8, 137, 199).

**Result for the map as revised.** 589 citations; every one names a file that exists at `HEAD` and a line inside it; 507 matched by identifier on the line, and the other 82 were read by eye (section headings cited by their heading line, string literals, lines inside a cited function, schema keys one line above their `$ref`).

**Result for the specification.** 31 citations in V1, 80 in V1.1 (§12 adds most). Moved: **1** (`instances/DESIGN-SYSTEM/specimen.html` 1258 → 1568, because the specimen was regenerated at V1.2). Re-pointed because the wording source changed: M-13 now cites the rendered lines `HistoricalRunContext.tsx:321` to `:323` and no longer the code comment at `:9`. Added: `docs/DIRECTIVE.md:117` beside `:105` for M-17, since `:105` is the section heading and the list is at `:117`. The four citations of run documents (`instances/…`) resolve against the run folder, not the repository root, as in V1.

## 4. The key audit

Every key named in the specification's §3 keyboard models, §4, §5 and §6, against the pointer rule (§2.8). "Found" means V1 of the specification already named a visible control; "design system" means V1.2 names the control and the specification now adopts it; "this document" means the specification names a control the design system does not.

| Key | Action | Where | In V1 | Primary control in V1.1 | Source of the control |
|---|---|---|---|---|---|
| a character; ↩ (not editing) | start editing a cell | §3.1 | key only | a click on the focused cell; a double-click on any cell | design system |
| ↩, ⇥, arrows (editing) | commit an edit | §3.1, §2.4 | key, or focus leaving | a click on another cell; the footer's edit chip, Commit | design system |
| ⎋ (editing) | cancel an edit | §3.1 | key only | the edit chip's Cancel | design system |
| ⇥ ⇧⇥ ← → ↑ ↓ | move the focus | §3.1 | key only | a click on the target cell, glyph or slot | design system |
| ⎋ (not editing) | clear the selection; close an expansion, drawer, popover | §3.1 | key only | the clear control on the selected count; the close controls; a click outside | design system |
| ⌘↩ | open a row expansion or a joined row | §3.1, §3.2, §3.5, §3.6, §3.7 | key, or a click on a marks slot or the Expression cell | the marks slot; the expansion chevron in the owning cell (Type, Expression, Size, and Stress) | design system; Stress is this document's |
| ⎋, ⌘↩ | close a row expansion | §3.1, §3.3 | key, or "its chevron" | the caption's chevron; its close control | design system |
| ⌥↩ | insert a row; add a case | §3.1, §3.2, §3.5 | key, or an "Add row" toolbar control | the footer's Insert row below; the "Add row" and "Add case" lines | design system |
| ↩ on the last cell | add a row | §3.2 | key | the "Add row" line | design system |
| ⌘A | select all rows | §3.1 | key only | the gutter's header cell | design system |
| ⇧↑ ⇧↓, ⇧-click, ⌘-click | extend or toggle the selection | §3.1, §4.2 | key and modifier-click | the gutter (and the canvas entity), with the modifier held | design system |
| ⌘C, ⌘V | copy rows; paste | §3.1 | key only | the footer's Copy rows and Paste | design system |
| ⌫ | delete rows; clear a cell | §3.1, §3.2 | key only | the footer's Delete rows; for a cell, editing it and clearing the text | design system; the cell case is this document's |
| Space | open a mark's popover; toggle a box | §3.1, §3.2 | key only | a click on the glyph or the box | design system |
| ⌥I | list every mark on a row | §3.1, §6.8 | key only | "All marks on this row" in any mark's popover | design system |
| ⌘⇧A, ⌘⇧R | Accept, Reject a proposed row | §3.1, §6.3 | found (card buttons, footer chips) | the card row's check and cross buttons; the footer's chips | found |
| ⌘⇧K | Check, Check again, Clear check | §3.1, §6.7, §10.7 | key, the inspector's control, a "row menu" | the faint check in an empty state slot; the footer's Check rows; the Checked popover's buttons; the inspector's control; Edit menu | design system |
| ⌘Z, ⇧⌘Z | undo, redo | §2.5, §3.1 | key, and Undo in toasts | the toolbar's Undo and Redo | design system |
| ⌘K | the palette | §3.1, §5.5 | found | the toolbar's palette field | found |
| ⌘F | filter the table | §3.1 | key only | a click on the footer's Filter control | design system (§4 names the control) |
| ⌘↓, ⌥↓ | sort by a column; open the column menu | §3.1 | key, or hover | the header's sort indicator; the header's column menu button | design system |
| ⌘⇧S | Split element… | §3.2 | key, palette, a "row menu" | Insert › Split element…; the palette; the sheet's Split and Cancel | this document (the "row menu" had no visible opener and is removed) |
| — | Renumber nodes… | §3.2 | palette, a "row menu" | Analyze › Renumber nodes…; the palette; the sheet's buttons | this document |
| ↩ in the edit-or-fork popover | choose Edit or Fork | §3.2 | key only | the two choices are buttons; a click outside cancels | this document |
| ↩, ⎋ in the paste and generation bands | Paste, Generate, Cancel | §3.1, §3.5 | found | the bands' two buttons | found |
| ↩, ⎋ in the combination editor | Done, Cancel | §3.5 | found | the editor's two buttons | found |
| ⌘1 ⌘2 ⌘3 | the view switch | §2.3 | found | the three-segment control | found |
| ⎋ on a Libraries or Rules page | return to the stage | §2.3 | key, or the rail item | the rail item; the page's close control | this document |
| F I H L D P R S | the HUD tools | §4.3 to §4.6 | found (tools), but headings named the key | the HUD's ten tools, Fit first at every width | design system |
| — | show what is hidden | §4.6 | a HUD text, not stated as a button | the count as a text button, "3 hidden · Show all" | design system |
| ⇥, arrows (routing) | choose the axis | §4.4 | key only | a click on an axis handle | design system |
| − (routing) | reverse the axis | §4.4 | key only | Reverse beside the length field; the opposite stub | design system |
| ↩ (routing) | place the node | §4.4 | key only | Place beside the length field; "Place node 50" in the routing block | design system |
| ⎋ (routing) | cancel | §4.4 | key, and the inspector's Cancel | Cancel beside the length field; the routing block's Cancel | design system |
| B (routing) | bend at the node | §4.4 | key, and the block's line | the routing block's "Bend at 40…" | design system |
| ↩, ⎋ (add restraint) | add or discard the draft restraint row | §4.4 | key only | the draft row's Add and Cancel buttons | this document |
| ⎋, P, second click | unpin the probe | §4.3 | key only | the pinned card's close control | design system |
| ⌘I, double-click, ⎋ | open and close the inspector | §4.2, §5.1 | key and double-click only | the toolbar's Inspector toggle; the inspector's close control in Both view | design system |
| ⌘⇧I | open the issues drawer | §5.2 | found (status bar, rail, toolbar, Run reason) | the same four entries | found |
| ⎋ | close the issues drawer; collapse the table drawer | §5.2, §2.3 | key only | the drawer's close control; the collapse chevron on the table drawer's tab strip | design system |
| click on Run | open the run log | §5.3 | "a popover on the Run button", opener unstated | a click on the Run button while it runs or after the run has ended; the first click outside closes it | design system, with this document's opening rule |
| ⌘⇧G | open and collapse the agent column | §5.5, §6.1 | key and the toolbar toggle | the Agent toggle; a click on the strip; the header's collapse control | design system |
| ⌘↩ (conversation) | send a message | §6.2 | key only | the Send control | design system |
| ⌘↑ ⌘↓ | reorder an editable report section | §7.2 | key, and a drag handle | the drag handle | found (a drag is the design system's own control for it) |
| ⎋ | close a dialog, a sheet, a popover, a toast | throughout | key only, or unstated | Cancel or Close on every dialog and sheet; a click outside a popover; the toast's close control | design system |

Actions found to need nothing: Run and Stop, the Issues entries, Accept and Reject at every level, the paste and generation bands, the combination editor, the view switch, the palette.

## 5. The semantic changes named (specification §12)

1. Isolate dims where today it hides (G-30). 2. Selection is a halo where today it replaces the element's colour (G-31). 3. Camera and interface state persist where today they do not (G-17). 4. A run's results after a model change: today the run stops being the solve basis, which is kept, and the results are removed from the product, which changes to a Stale run kept readable on a Historical record's terms (G-11); the product was read at `HEAD` for this and the brief's statement was confirmed and extended (the results are not merely un-based today, they are cleared, with the proposal and the report package state). 5. Label Budget replaces the cap of 80 (G-16). 6. Per-row proposal decisions where today there is one proposal slot presented whole (G-18). 7. A pending proposal survives a model change to other rows, where today any applied operation clears it (G-18). 8. The fitted-camera state of Q-20 (G-32). 9. The hidden count counts everything hidden, where today it counts selected hidden items (G-30). 10. Deformation as a stated scale factor, where today it is a fixed normalized display offset with its own published boundary string (G-16). 11. The undeformed shape as a dashed ghost (G-16). 12. The routing draft in its own colour, where today it uses the selection colour (G-31). 13. Label plates as part of the printed figure, where today they are an HTML overlay (G-16). 14. Real outside diameter as the resting state and every canvas colour a token (G-16). 15. The Checked mark, which does not exist today (G-08). 16. Runs kept as a list, where today there is one slot (G-10). 17. Numeric, renumberable node identity, where today ids are free strings (G-01). 18. Registered status label forms in place of two curated ones (G-34). 19. The two removed sentences, the name and the export's name (G-34). 20. Keys as accelerators only (G-33).

## 6. Contradictions found with design system V1.2

Where the specification could follow V1.2 it does; these are the places where following it needed a reading, or where V1.2 disagrees with itself or with a governing source. None was resolved by changing anything outside this directory.

1. **The Review rail item.** V1.2's run-standing table says the Review item "is enabled by a Current solved run" and that a Historical run "enables nothing", yet gives Stale and Historical live blocks a band, which supposes the page is open. Reading taken: enabled by a Current solved run, stays reachable while that run is Stale, never enabled by a Historical run alone (tooltip "Needs a current run", V1.2's phrase used in a place V1.2 does not put it).
2. **Report readiness is not defined in V1.2.** The brief asks for it per standing. Reading taken (§2.6, §7.6): Report preview opens in every standing with the run's band printed on each live block; Print, Save and the report package need a Current run. Today's product invalidates the report package state on any model change, which supports the reading.
3. **The probe.** V1.2 makes the Probe tool Current-run-only on every stage; V1 of the specification gave it Model- and Loads-stage readings that need no run. Resolved toward V1.2: those readings are withdrawn.
4. **The Run button and the run log.** V1.2 opens the log by "clicking the Run button while it runs, or after it stops", which collides with the same click starting a run. Reading taken: the click opens the log while running and after a run has ended until the model changes; "Run again" is in the log; after a model change the click starts a run.
5. **"Commit".** V1.2 §7.1 forbids "Commit" under Tables and the canvas, and V1.2 §5.1 names the edit chip's button "Commit". The specification uses §5.1's name; one of the two lines in V1.2 needs to change.
6. **The Checked mark's words.** V1.2 §7.3 says the words are exactly the ruled ones and no others, while V1.2 §4 and §5.1 name buttons "Check rows", "Check again", "Clear check" and a "Stale" filter. The specification keeps the buttons (they carry the ruled stale text's two acts), drops a separately named stale filter, and lets the footer's counts filter.
7. **M-04's trigger.** V1.2 §5.5 and §7.4 say export metadata, hanger selection records or handoff data; the specification keeps RESEARCH-C §4's wording (design-authoring records, handoff packages, export metadata, external-prover references), as V1 did and said.
8. **Restraint kinds.** V1.2 §5.1 sizes a "Connecting node" column and §6.4 draws a skewed restraint and a connecting-node tie; the specification classes both as engine gap G-22 and does not offer them, by the logic R-4 applied to element kinds. V1.2 was not asked to apply R-4 to restraints.
9. **The units selector.** V1.2's toolbar names "SI" / "US"; the specification keeps a third choice, the entered units, because the product's preference type has it today and dropping it would be a semantic change nobody asked for.
10. **HUD contents.** V1.2's HUD has exactly ten tools; the specification's Follow selection, box-selection filter and Colour by had no home in it. They are placed in the View tool's menu.
11. **Expansion owners.** V1.2 names three cells that own an expansion (Type, Expression, Size); the specification's Stresses expansion needs a fourth, the Stress cell.
12. **The Kind menu** has no way to clear a chosen kind in V1.2; the specification adds "All kinds" to the menu.
13. **The outline's state word "current"** is listed in V1.2 and defined nowhere; the specification reads it as "unchanged since the compared iteration".
14. **V1.2 §8 item 17** says its band "agrees with UX_SPEC_V1 §6.3"; V1's §6.3 line said the run "is kept as a historical run". V1.1 adopts V1.2's own consequence sentence (§5.4) instead, and explains it beside the quotation.
15. **Tooltip form.** V1.2 §7.6 puts the accelerator in parentheses after the control's name, while V1.2 §4's tooltips read "Accept ⌘⇧A · Reject ⌘⇧R" and "⌘↩ opens the row". The specification quotes §4's strings where it quotes them and follows §7.6 for its own.
16. **The Both-view split's limit.** V1.2 gives the canvas a 220 px minimum and the table no floor. The specification stops a drag of the split where the canvas would fall under 220 px and states no table floor; V1.2 says neither.

## 7. Uncertainties

1. **Hanger selection under a Stale run.** Selecting a size is a model change, so the run is Stale after the first selection. The specification lets the remaining locations be selected against their hatched design values so that the two-pass ritual can finish before the next run. V1.2 does not say; the alternative (selection needs a Current run) would force a run per hanger.
2. **Undo of an accepted row** returns the card row to Pending. V1 and V1.2 say only that Undo is offered.
3. **The stale band after several changes** names the first change and a count ("and 3 more changes"). V1.2 gives the one-change form only.
4. **A reopened saved run is always Historical**, even when its saved hash matches the model. This follows the product's own code comment and rendered text; V1 had said "whose model hash no longer matches".
5. **The draft restraint row's Add and Cancel**, the Split and Renumber sheets' buttons, and the close control on the Libraries and Rules pages are this document's controls; V1.2 has the rule but not these instances.
6. **Identifier matching in the citation check is heuristic.** 507 of 589 map citations matched by script; 82 were read by eye. A citation that names the right file and a plausible nearby line could still be a few lines from the best anchor.
7. **§12's today sides** for items 11, 13 and part of 14 rest on RESEARCH-G §2's citations (the two solid layers, the overlay's stylesheet, the theme repaint); I re-read the lines this document cites itself and not those.
8. **The "Evidence" domain word** and **`canvas.edgeAlt`** are carried from V1.2 as it flags them (its §8 items 13 and 14).
9. **Carried from the first return and not re-examined:** evidence-label emission was searched in the desktop source only; whether `set_field` accepts reference fields; whether a stress category exists outside `LoadCase` and `Combination`; whether solve events are per case; the export renderer's code table was not read for behaviour; the frames were read through their generator.
10. **The word "Commit"** appears in the specification as a control name (contradiction 5). If ROOT rules for §7.1, the edit chip's button needs another word in both documents.
11. **Rows 278 to 313 of the map** are classed mostly as interface gaps with the operation they reuse. I did not search the product for an existing control that already matches one (for example a toolbar undo button); a found control would turn a gap into a provider.
12. **RESEARCH-G §1** was not read item by item; if it classifies a workload in a way that bears on a §12 row, that is not reflected.
13. **The class breakdown of the first 29 gaps** in the first return (13 typed-interface, 4 interface) did not match the map's own tables (14 and 3). The map now states the counts by its tables.

## 8. Where the brief could not be fully satisfied

1. "Retire rows for controls it removes (the acceptance disclosure, the compatibility flag)": V1's map had no row for a compatibility flag, because V1 never drew one. Row 310 enters it already retired, with today's rendered flag cited, so that the removal is traceable; rows 138 and 236 are the acceptance disclosure's two homes.
2. "Correct moved lines": no cited line moved because of PRs #793 to #795. The corrections made are of imprecisions V1 already had (§3).
3. The acceptance search: the three files contain neither the longer product name, nor the maturity sentence or either half of it, nor the removed clause about the solve basis, nor an acceptance sentence or variant, nor an absolute path. The other vendor's product name appears only inside source file paths and identifiers, in the map's provider column and once in the specification's §12 (row 19, a file path on the "today" side). The product's former name does not appear at all. The phrases "acceptance sentence" and "maturity sentence" appear as names of the removed things.
4. `contributor_certification` still appears once in the specification (§8.1), in code font, as the name of an existing provenance field; and "Review/signoff block" remains as the registered section name.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
