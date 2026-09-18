# Design system V1 — SWBPIPE

Status: V1.2, revision under sealed brief DESIGN-SYSTEM-03, 2026-09-18, for ROOT's acceptance. What changed from V1.1: the owner's rulings on decision packet D-71 are applied as design consequences (all nine items; two of them, the maturity sentence's removal and the Checked mark's classification, were ruled after the brief was sealed and reached this revision as a message from ROOT, recorded in §9); the frames' questions Q-15 to Q-22 are answered, as defaults that use of the product may change; the gaps G-7 to G-12 are closed; R-4, R-5 and R-6 of ROOT's revision record are applied; and one general rule is added and the whole system checked against it: every action has a visible control operated by a mouse click, and a keyboard shortcut is an accelerator for a control that exists, never the only way (§5, §7.6). Every V1.2 change is listed in §9 from row 50 with its source. V1.1 was the revision under sealed brief DESIGN-SYSTEM-02 (phase 3 of the interface program), 2026-09-18. What changed then: the fourteen mock-review decisions of the direction record §11 are applied; the frames' decisions and departures (MOCKS_V1 §1–§3) are adopted or overruled with reasons; the six token gaps (MOCKS_V1 §5) are closed with ten new colour tokens; the dark result scale is re-anchored and re-validated; the Both-view inspector docks; the copy rules carry the frames' strings; after ROOT's review of V1.1 the docked inspector's narrow case follows UX_SPEC_V1 §10.9 and a spelling rule is added (§9 rows 48 and 49). Every change is listed in §9 with its source. V1 was written by the HELPS_HUMANS design manager under sealed brief DESIGN-SYSTEM-01, 2026-09-18, on the basis read in that brief's order: the design brief V1.4, the direction record, the concept directions V1 (§0, the shells, the three presentation languages and the disclosure homes), RESEARCH-E §3.2–3.4 and §6.1–6.2, RESEARCH-C §1.1, §3 and §4, RESEARCH-A's preserve list and RESEARCH-D §2 and §6. V1.1 additionally read the direction record §10–§11, MOCKS_V1 with its fifteen frames and `tools/`, and RESEARCH-C §4 row M-07. V1.2 additionally read ROOT's revision record of 2026-09-18, MOCKS_V2 §3 to §6 with its frames and generator, and decision packet D-71; what it consulted beyond the brief's list is recorded in `RETURN.md`. No product stylesheet was read; nothing outside `instances/DESIGN-SYSTEM/` was written.

Companions: [`tokens.json`](tokens.json) holds every token in this document (colour as light and dark values; type, spacing, radii, borders, elevation, focus, motion, layout and icon values as plain values) and, from V1.2, the one table of status and evidence labels (`labels`, §2.3) and the five class words of agent cards (`agentCardClasses`, §5.4). [`specimen.html`](specimen.html) renders them offline in both themes. `tools/` is the chain that keeps the three in agreement: `palette.mjs` builds `tokens.json` in OKLCH and runs the validator, `contrast.mjs` computes the findings, `gen.mjs` generates the CSS blocks, the embedded JSON and the tables of §2.2, §2.3 and §2.9, `splice.mjs` writes them between the generated-block markers here and in the specimen, `agree.mjs` checks that the three files agree, and `render.mjs` renders the specimen offline in both themes. [`RETURN.md`](RETURN.md) records what was read, the checks' results, and what is uncertain.

How to read. A token is written `group.name`; its CSS custom property is `--group-name`. A colour token always has a light and a dark value; the two themes are designed together in OKLCH so that each pairing keeps its role in both, and the values are converted to sRGB hex. "Sample" values in figures and the specimen are placeholders from one small fictional model and are labelled as such. Node IDs are numeric throughout. The claims boundary applies to every example string in this document exactly as it applies to the product.

## 0. The shell this system dresses

The product shape is fixed by the brief (§4) and the direction record (§2): stages Model, Loads, Results, Review on a rail; views Table, Model, Both per stage, remembered per stage, first opening Model in Both, Loads in Table, Results in Both, Review in Table; the agent in a right column that collapses to a strip and never covers the tables. The numbers below are the design system's, given so that the mocks draw the same shell.

| Region | Size at 1440 × 900 | Notes |
|---|---|---|
| Toolbar band | 48 px | One unified title-and-toolbar band; macOS traffic lights inset 78 px on the left. |
| Status bar | 24 px | |
| Stage rail | 56 px | Model, Loads, Results, Review; at its foot Libraries, Rules, Issues. |
| Agent column | 340 px open, 44 px as a strip | Present in every stage. Opening it reflows the surfaces; it never overlays them. |
| Surfaces | 1340 × 828 with the strip; 1044 × 828 with the column open | 900 − 48 − 24 high; 1440 − 56 − 44 wide. |
| Table view | tables at the full surface width | Issues drawer 200 px at the bottom when open. |
| Model view | canvas 1000 wide beside a docked inspector of 340; the stage's current table as a bottom drawer of 280 px | Canvas 1000 × 548 = 42% of the window with the drawer open, 1000 × 828 = 64% closed. |
| Both view | table 55% / canvas 45% of the surface width (737 / 603 at 1440 with the strip), resizable, remembered | The inspector docks on the canvas's right edge at 300 px (opened by the toolbar's Inspector toggle, closed by the same toggle or the inspector's own close control; ⌘I, a double-click on a row or a node and Escape are the accelerators, §5) and the canvas shrinks to the remainder (canvas 303 at 1440 with the strip) while it is open; the table never reflows (decision 1). The rules for the narrow cases follow the table. |
| Minimum window | 1280 × 800 | Surfaces 1180 × 728 with the strip. |

**The docked inspector in Both view (decision 1).** Opening the inspector takes 300 px (`layout.inspector.both`) from the canvas pane and nothing from the table; the split between table and canvas is the engineer's and is not moved. The canvas has a minimum drawing width of 220 px (`layout.canvas.min`): while the remainder is at or above it, the canvas keeps its camera scale and centre and pans only as far as needed to keep the selected node in view, and pans back when the inspector closes. When opening the inspector would take the canvas below its minimum, the agent column collapses to its strip first (it is collapsible by design and re-opens from the toolbar's Agent toggle or a click on the strip; ⌘⇧G is the accelerator), which widens the surfaces and restores the split; if the canvas would still fall below its minimum, the inspector opens as a slide-over over the canvas for that window size only, with the tooltip "Docked inspector needs a wider window" (UX_SPEC_V1 §10.9, adopted by ROOT). The canvas is never collapsed. At and above the minimum window the slide-over case does not arise unless the engineer's own split leaves the canvas under 520 px. In a canvas narrower than 400 px the HUD wraps to two rows, the legend collapses to its title row (hover or click expands it) and the probe pins to the canvas's bottom edge; the narrow canvas's furniture is laid out in §5.6 (G-10). The table keeps its width in every case.

**The fitted camera (Q-20; a default).** Decision 1's rule, that the canvas keeps its camera when its width changes, is for a camera the engineer placed. A camera is *fitted* from a Fit (the HUD's Fit control, its accelerator F, a view preset, or the first display of a model) until the engineer orbits, pans or zooms. While the camera is fitted, docking and undocking the inspector refit by themselves, as does any other change of the canvas's width (the agent column opening or closing, the end of a drag of the split); once the engineer has moved the camera it is kept as decision 1 says. No toast announces either case. The owner's amendment to Q-20 asks for a primary control operated by a mouse click, which ROOT reads as the general pointer rule of §5: Fit is a visible control in the HUD at every canvas width, the narrow case included (§5.6), and F only accelerates it. The fitted state is a proposed semantic addition, one bit of interface state beside the camera, and is flagged as such for implementation (§8 item 12).

**The agent column reopened while the inspector is docked (Q-15; a default).** The rule above says what happens when the inspector opens; this is the other order. With the inspector docked, opening the agent column would take the canvas under its minimum (1440: 470 − 300 = 170). The column opens, because the engineer's last explicit act wins, and the inspector becomes the slide-over over the canvas for as long as the column is open at that width; it re-docks when the column closes. Nothing is refused and nothing closes, and the selection's context is never lost. It mirrors the first rule, where opening the inspector sends the column to its strip.

| Window and agent | Surface width | Table / canvas | Canvas with the inspector docked |
|---|---|---|---|
| 1440, strip | 1340 | 737 / 603 | 303 |
| 1440, column open | 1044, then 1340 | 574 / 470, then 737 / 603 | 170 would be below the minimum, so the agent column collapses to its strip first: canvas 303, as with the strip |
| 1280, strip | 1180 | 649 / 531 | 231 |
| 1280, column open | 884, then 1180 | 486 / 398, then 649 / 531 | 98 would be below the minimum, so the column collapses first: canvas 231 |
| 1440, inspector docked, then the agent column opened | 1044 | 574 / 470 | 170 docked would be below the minimum, so the inspector becomes the slide-over over the 470 px canvas while the column is open, and re-docks (canvas 303) when it closes (Q-15) |
| 1280, inspector docked, then the agent column opened | 884 | 486 / 398 | 98 docked would be below the minimum: the same rule, the slide-over over the 398 px canvas (Q-15) |

## 1. Foundations

### 1.1 Type

| Token | Value | Use |
|---|---|---|
| `type.family.ui` | `-apple-system, BlinkMacSystemFont, system-ui, "SF Pro Text", "Helvetica Neue", Helvetica, Arial, sans-serif` | Chrome, labels, cells, conversation, report text. On macOS this resolves to SF Pro Text; the fallbacks are declared for any other host. |
| `type.family.numeric` | the same family with `font-variant-numeric: tabular-nums` | Every number, everywhere: cells, inspector values, probe, legend, chips, counts, footers. SF Pro Text, Helvetica Neue and Helvetica carry tabular figures; Arial's figures are tabular by construction. |
| `type.family.mono` | `ui-monospace, "SF Mono", Menlo, Monaco, Consolas, monospace` | Raw status tokens, hashes and run identities in popovers and the run record, display-only rule expressions (M-15), file names, node codes when copied. Never for a value in a table. |

No web font is proposed. The system faces above are sufficient on macOS and the specimen depends on nothing else. Should a non-Apple host ever be targeted, the candidate is Inter (SIL Open Font License 1.1), which carries tabular figures; that is a later decision and not part of this system.

Size scale, in px with its line height:

| Token | Size / line | Weight | Use |
|---|---|---|---|
| `type.size.caption` | 11 / 14 | 400 | Captions only: canvas HUD tick labels, the "sample" label, axis names on the compass, the units in column headers (P-1), the unit at the right edge of an editing cell (s3), the rail's state captions Failed, Stale and Historical (D-3), the restraint glyph plates in the canvas (D-10). Never a value, never a control label. |
| `type.size.chip` | 12 / 16 | 500 | Chip labels, keyboard hints and the canvas hint strip, footer counts, the outline's state words, load plates in the canvas. |
| `type.size.body` | 13 / 18 | 400 | Everything read repeatedly: cells, column headers (500), controls, inspector rows, issues, conversation, proposal diffs, report text. **13 px is the smallest size for anything read repeatedly.** |
| `type.size.title` | 15 / 20 | 600 | Panel and section titles, dialog titles, the proposal card title. |
| `type.size.heading` | 18 / 24 | 600 | The Review page title, About. |
| `type.size.figure` | 22 / 28 | 500, letter-spacing −0.2 | One hero figure per surface at most: the Summary's governing ratio with its rule and pack beside it. |

Rules. No uppercase grey headings; titles are sentence case at 15/600. Numbers are right-aligned in cells with a fixed number of decimals per column (lengths 0 or 1 decimal in mm, stresses 1 decimal in MPa, ratios 2 decimals, temperatures 0 decimals, pressures 2 decimals; the column's unit and precision are one setting), a true minus sign (U+2212), and no thousands separators unless the display preference asks for thin-space groups. Units live in column headers in square brackets (`Stress [MPa]`) at caption size in `text.secondary` after the name, so that a 56 px read-through column holds `P1 [bar]` (P-1); after a value with a thin space in the inspector, probe and legend (`124.6 MPa`); and never inside a committed cell. An editing cell shows its unit at caption size in `text.muted` at the cell's right edge, so a 72 px column holds `−1500 mm` with the caret (s3). Text never wears a series or scale colour: values and labels stay in the text tokens and a mark beside them carries the colour.

### 1.2 Spacing, radii, borders, elevation

Spacing is a 4 px system: `space.0` 0, `.1` 2, `.2` 4, `.3` 6, `.4` 8, `.5` 12, `.6` 16, `.7` 20, `.8` 24, `.9` 32, `.10` 40. Cell padding 0 8; header cell padding 0 5, tighter than the cell so the default widths hold their headers (P-1); control padding 0 8; panel padding 12; dialog padding 20; 16 px side gutter for any page-like surface (Review, About, Preferences). Radii: `radius.control` 3 (inputs, buttons, chips, menu items), `radius.card` 6 (proposal cards, probe card, legend, popovers), `radius.sheet` 10 (dialogs and sheets), `radius.pill` reserved for the count badge on the agent strip and the rail. Borders are 1 px hairlines (`border.hairline`) between regions and rows; `border.strong` frames inputs, the table and the drawers; nothing decorative is thicker than 1 px. Elevation: level 0 flat for panels and tables, level 1 for a drawer edge and a hovered card, level 2 for popovers, the probe card and the legend, level 3 for dialogs; in dark the shadows are deeper and each raised surface also carries a 1 px inner light ring (`elevation.*.dark`), because a dark surface cannot be raised by shadow alone.

### 1.3 Focus, selection, hover, disabled

Focus is a 2 px ring in `focus.ring`, offset 1 px outside controls and inset 2 px inside table cells so it is never clipped; a focused cell also rises above its neighbours. Selection is a fill (`selection.band`) with a 3 px bar (`selection.bar`) on the left edge of the row; in the canvas it is a halo. Focus and selection share the platform's accent hue and differ in form; they coexist (a focused cell inside a selected row shows both). Hover is a wash (`hover.wash`) over the row or control, pressed a stronger wash (`pressed.wash`) for the moment of the press. A toggle that stays latched (the toolbar's Agent toggle while the column is open and its Inspector toggle while the Both-view inspector is open; a HUD tool such as Route or Probe while it is the active tool) sits on `pressed.fill` with its glyph and label in `pressed.ink`, a state of its own and not the segmented view switch's, whose active segment is `surface.panel` with `text.primary` (G-5). Disabled controls sit on `disabled.fill` with `text.disabled`, keep their label, and carry the reason in a tooltip (the Run button says why it is disabled: "Run is unavailable — Section missing at node 20, Material missing at node 20"). The accent hue is spent on interaction only: selection, focus, links, the one primary action per surface, latched toggles, and the engineer's own draft: the ghost in the canvas and the draft row's bar in the tables (`draft.bar`, G-3). Nothing else is blue.

### 1.4 Motion

Hover 80 ms; disclosures and popovers 120 ms; drawers, the agent column and the docked inspector 180 ms (the canvas resizes with the inspector in the same motion); a proposal arriving 240 ms (the row band fades in and the diff card rises 4 px into place); a toast enters and leaves in 120 ms and stays 6 s, or 10 s when it carries an action (`motion.toast.ms`, `motion.toastAction.ms`; §5.6). Easing `cubic-bezier(0.2, 0, 0, 1)` in, `cubic-bezier(0.4, 0, 1, 1)` out. Nothing bounces, pulses or shimmers. Under `prefers-reduced-motion` every duration is 0. The canvas's own camera easing belongs to the engine and is not specified here.

### 1.5 Density

One density, tuned for tables of numbers: row 26 px, header row 28 px, control 26 px (22 px compact in the HUD, chips and the status bar), rail item 44 px, toolbar 48 px, status bar 24 px, gutter column 32 px, marks column 72 px. Icons draw at 16 px inside 26 px controls, 20 px in the rail, 12 px in the gutter, marks column and chips. Column widths by default (resizable): gutter 32, Node 56, From 56, Type 100 (96 in Both view with read-through off, so the table fits its 737 px pane without a scroll), DX DY DZ 72 each, Section 72, Material 72, Load 64, each T and P read-through 56, marks 72; the other tables' widths are in §5.1 (P-4). Header cells pad 0 5 where cells pad 0 8 (P-1). The inspector's label column is 88 px (P-2). It is the density of a CAE tool: no stacked label-over-input forms, no card padding around a number, no 30 px web controls.

## 2. Colour

### 2.1 The hue budget

Each hue has one job and is spent nowhere else. This is the rule that keeps "colour as approval" out of the product (RESEARCH-D §6) and keeps the loud thing loud.

| Hue | Job | Tokens |
|---|---|---|
| Blue (accent) | Interaction: selection, focus, links, the primary action, latched toggles, the engineer's draft ghost and draft row, the Z axis of the triad | `accent.*`, `selection.*`, `focus.ring`, `border.focus`, `text.link`, `pressed.fill`, `pressed.ink`, `canvas.selection`, `canvas.hover`, `canvas.draft`, `draft.bar` |
| Violet | Proposals, and nothing else: the proposed row band and bar, new values, the ghost in the canvas | `proposal.*`, `canvas.proposalGhost` |
| Red | Blocking issues and the required mark (model validity, solve, rule check), the failed status and the rail's Failed caption, the X axis of the triad | `issue.blocking*`, `mark.required`, `status.failed*`, `rail.captionFailed` |
| Amber | Warnings and staleness: provenance, assumption and nonlinear warnings, the stale Checked mark, the results-stale band and the rail's Stale caption, the incomplete statuses | `issue.warning*`, `mark.checkedStale`, `stale.band`, `stale.ink`, `rail.captionStale`, `status.incomplete*` |
| Blue-grey | Informational issues and the review-required status | `issue.info*`, `status.review*` |
| Teal | The result scale, only, on the tube and in the legend and ratio bars | `result.scale.1–7` |
| Warm neutral | Historical (M-13), on the band and as the rail's caption | `historical.*`, `rail.captionHistorical` |
| Neutral ink | Origins, Checked, attachment marks, the comment glyph, glyphs, vectors when result colour is on or a proposal ghost is drawn, the statuses that are neither failed nor incomplete; and the neutral plates, the hint strip, the data bar's track, and the edge line with its alternate | `mark.origin`, `mark.checked`, `mark.attachment`, `status.solved*`, `canvas.glyph`, `canvas.vector`, `canvas.hint`, `bar.track`, `canvas.edge`, `canvas.edgeAlt` |
| Categorical 1–8 | Load kinds and cases, only | `cat.1–8` |
| Green | The Y axis of the triad, only. Green marks nothing as good, ready, solved or checked. | `canvas.axisY` |

### 2.2 Tokens

Every colour token, light and dark. The tables below are generated from `tokens.json` by `tools/gen.mjs` and spliced in by `tools/splice.mjs`; the hex values are the file's, and `tools/agree.mjs` re-checks them against it.

<!-- GENERATED:COLOUR_TABLES:BEGIN -->
**surface**

| Token | CSS variable | Light | Dark |
|---|---|---|---|
| `surface.base` | `--surface-base` | `#eef0f3` | `#1b1e22` |
| `surface.panel` | `--surface-panel` | `#ffffff` | `#25282c` |
| `surface.raised` | `--surface-raised` | `#ffffff` | `#2e3236` |
| `surface.sunken` | `--surface-sunken` | `#f5f7f9` | `#202327` |
| `surface.header` | `--surface-header` | `#f0f3f5` | `#292d31` |
| `surface.rowAlt` | `--surface-rowAlt` | `#f9fafb` | `#272a2e` |
| `surface.canvas` | `--surface-canvas` | `#ebedef` | `#191c1f` |

**text**

| Token | CSS variable | Light | Dark |
|---|---|---|---|
| `text.primary` | `--text-primary` | `#22272c` | `#e5e8ec` |
| `text.secondary` | `--text-secondary` | `#5b6168` | `#acb2b7` |
| `text.muted` | `--text-muted` | `#7d848b` | `#878d93` |
| `text.disabled` | `--text-disabled` | `#a6abb1` | `#5f6469` |
| `text.inverse` | `--text-inverse` | `#ffffff` | `#13161a` |
| `text.link` | `--text-link` | `#1762b6` | `#87bafd` |

**border**

| Token | CSS variable | Light | Dark |
|---|---|---|---|
| `border.hairline` | `--border-hairline` | `#d5d8db` | `#393e42` |
| `border.strong` | `--border-strong` | `#b3b8be` | `#53595f` |
| `border.focus` | `--border-focus` | `#1a73d5` | `#70adfb` |

**accent**

| Token | CSS variable | Light | Dark |
|---|---|---|---|
| `accent.fill` | `--accent-fill` | `#106dce` | `#4c94ec` |
| `accent.fillHover` | `--accent-fillHover` | `#0161bd` | `#59a0f9` |
| `accent.text` | `--accent-text` | `#005bb3` | `#91c1ff` |

**selection**

| Token | CSS variable | Light | Dark |
|---|---|---|---|
| `selection.band` | `--selection-band` | `#e2eefe` | `#223e61` |
| `selection.bar` | `--selection-bar` | `#106dce` | `#65a7fa` |

**hover**

| Token | CSS variable | Light | Dark |
|---|---|---|---|
| `hover.wash` | `--hover-wash` | `rgba(30,36,48,0.06)` | `rgba(255,255,255,0.06)` |

**pressed**

| Token | CSS variable | Light | Dark |
|---|---|---|---|
| `pressed.wash` | `--pressed-wash` | `rgba(30,36,48,0.10)` | `rgba(255,255,255,0.10)` |
| `pressed.fill` | `--pressed-fill` | `#d4e6ff` | `#254974` |
| `pressed.ink` | `--pressed-ink` | `#0053a4` | `#a8ceff` |

**disabled**

| Token | CSS variable | Light | Dark |
|---|---|---|---|
| `disabled.fill` | `--disabled-fill` | `#e9ebee` | `#2b2e32` |

**focus**

| Token | CSS variable | Light | Dark |
|---|---|---|---|
| `focus.ring` | `--focus-ring` | `#1a73d5` | `#70adfb` |

**scrim**

| Token | CSS variable | Light | Dark |
|---|---|---|---|
| `scrim` | `--scrim` | `rgba(20,22,28,0.32)` | `rgba(0,0,0,0.55)` |

**proposal**

| Token | CSS variable | Light | Dark |
|---|---|---|---|
| `proposal.new` | `--proposal-new` | `#6b3baa` | `#c4a4fe` |
| `proposal.band` | `--proposal-band` | `#f2edfe` | `#3b2f51` |
| `proposal.bar` | `--proposal-bar` | `#854ece` | `#ba93fb` |
| `proposal.old` | `--proposal-old` | `#7d848b` | `#878d93` |

**mark**

| Token | CSS variable | Light | Dark |
|---|---|---|---|
| `mark.checked` | `--mark-checked` | `#5b6168` | `#acb2b7` |
| `mark.checkedStale` | `--mark-checkedStale` | `#a46e01` | `#e9b452` |
| `mark.origin` | `--mark-origin` | `#7d848b` | `#878d93` |
| `mark.attachment` | `--mark-attachment` | `#5b6168` | `#acb2b7` |
| `mark.required` | `--mark-required` | `#ba2b2e` | `#fb817a` |

**issue**

| Token | CSS variable | Light | Dark |
|---|---|---|---|
| `issue.blocking` | `--issue-blocking` | `#ba2b2e` | `#fb817a` |
| `issue.blockingTint` | `--issue-blockingTint` | `#feebe9` | `#4b2724` |
| `issue.warning` | `--issue-warning` | `#a46e01` | `#e9b452` |
| `issue.warningTint` | `--issue-warningTint` | `#fff1d5` | `#433215` |
| `issue.info` | `--issue-info` | `#426882` | `#8fb7d4` |
| `issue.infoTint` | `--issue-infoTint` | `#e9f1f8` | `#27353e` |

**status**

| Token | CSS variable | Light | Dark |
|---|---|---|---|
| `status.incompleteFill` | `--status-incompleteFill` | `#fff1d5` | `#433215` |
| `status.incompleteInk` | `--status-incompleteInk` | `#774f02` | `#efc876` |
| `status.solvedFill` | `--status-solvedFill` | `#eef0f3` | `#32363a` |
| `status.solvedInk` | `--status-solvedInk` | `#353b42` | `#d4d8dd` |
| `status.failedFill` | `--status-failedFill` | `#feebe9` | `#4b2724` |
| `status.failedInk` | `--status-failedInk` | `#a21921` | `#ffaba3` |
| `status.reviewFill` | `--status-reviewFill` | `#e9f1f8` | `#27353e` |
| `status.reviewInk` | `--status-reviewInk` | `#25526f` | `#a8d1ee` |

**historical**

| Token | CSS variable | Light | Dark |
|---|---|---|---|
| `historical.tint` | `--historical-tint` | `#f4efe7` | `#322d25` |
| `historical.ink` | `--historical-ink` | `#5e5443` | `#cec2af` |

**displayOnly**

| Token | CSS variable | Light | Dark |
|---|---|---|---|
| `displayOnly.border` | `--displayOnly-border` | `#b3b8be` | `#53595f` |

**stale**

| Token | CSS variable | Light | Dark |
|---|---|---|---|
| `stale.stripe` | `--stale-stripe` | `#ced1d5` | `#44484d` |
| `stale.band` | `--stale-band` | `#fef6e7` | `#362c1c` |
| `stale.ink` | `--stale-ink` | `#7d5002` | `#e9ca89` |

**rail**

| Token | CSS variable | Light | Dark |
|---|---|---|---|
| `rail.captionFailed` | `--rail-captionFailed` | `#a21921` | `#ffaba3` |
| `rail.captionStale` | `--rail-captionStale` | `#7d5002` | `#e9ca89` |
| `rail.captionHistorical` | `--rail-captionHistorical` | `#5e5443` | `#cec2af` |

**canvas**

| Token | CSS variable | Light | Dark |
|---|---|---|---|
| `canvas.bg` | `--canvas-bg` | `#ebedef` | `#191c1f` |
| `canvas.gridMajor` | `--canvas-gridMajor` | `#ced1d5` | `#2d3135` |
| `canvas.gridMinor` | `--canvas-gridMinor` | `#dfe1e4` | `#222629` |
| `canvas.pipe` | `--canvas-pipe` | `#a6abb1` | `#757b81` |
| `canvas.pipeShade` | `--canvas-pipeShade` | `#81878d` | `#53595f` |
| `canvas.edge` | `--canvas-edge` | `#42484f` | `#c7cbd0` |
| `canvas.edgeAlt` | `--canvas-edgeAlt` | `#e5e8ec` | `#2a2e33` |
| `canvas.glyph` | `--canvas-glyph` | `#30363c` | `#dbdee2` |
| `canvas.glyphFill` | `--canvas-glyphFill` | `#dbdee2` | `#34383d` |
| `canvas.label` | `--canvas-label` | `#22272c` | `#e5e8ec` |
| `canvas.labelBg` | `--canvas-labelBg` | `rgba(255,255,255,0.82)` | `rgba(28,31,37,0.82)` |
| `canvas.hint` | `--canvas-hint` | `rgba(255,255,255,0.90)` | `rgba(46,50,54,0.92)` |
| `canvas.vector` | `--canvas-vector` | `#42484f` | `#c7cbd0` |
| `canvas.selection` | `--canvas-selection` | `#106dce` | `#6dadff` |
| `canvas.hover` | `--canvas-hover` | `#5d94da` | `#5788c7` |
| `canvas.draft` | `--canvas-draft` | `#106dce` | `#6dadff` |
| `canvas.proposalGhost` | `--canvas-proposalGhost` | `#854ece` | `#ba93fb` |
| `canvas.deformGhost` | `--canvas-deformGhost` | `#81878d` | `#53595f` |
| `canvas.unsolved` | `--canvas-unsolved` | `#a6abb1` | `#757b81` |
| `canvas.axisX` | `--canvas-axisX` | `#bd413f` | `#f07f77` |
| `canvas.axisY` | `--canvas-axisY` | `#308639` | `#73c076` |
| `canvas.axisZ` | `--canvas-axisZ` | `#2171cc` | `#70adfb` |

**draft**

| Token | CSS variable | Light | Dark |
|---|---|---|---|
| `draft.bar` | `--draft-bar` | `#106dce` | `#6dadff` |

**result**

| Token | CSS variable | Light | Dark |
|---|---|---|---|
| `result.scale.1` | `--result-scale-1` | `#4ab9b2` | `#045c59` |
| `result.scale.2` | `--result-scale-2` | `#15a6a0` | `#04706b` |
| `result.scale.3` | `--result-scale-3` | `#05908b` | `#09847f` |
| `result.scale.4` | `--result-scale-4` | `#087a76` | `#099993` |
| `result.scale.5` | `--result-scale-5` | `#056662` | `#24ada7` |
| `result.scale.6` | `--result-scale-6` | `#01524e` | `#52bfb9` |
| `result.scale.7` | `--result-scale-7` | `#003e3c` | `#74d1cb` |

**bar**

| Token | CSS variable | Light | Dark |
|---|---|---|---|
| `bar.track` | `--bar-track` | `#e0e3e6` | `#3c4045` |

**cat**

| Token | CSS variable | Light | Dark |
|---|---|---|---|
| `cat.1` | `--cat-1` | `#cf630d` | `#d97230` (orange) |
| `cat.2` | `--cat-2` | `#b84999` | `#c960aa` (magenta) |
| `cat.3` | `--cat-3` | `#3b9555` | `#429c5a` (green) |
| `cat.4` | `--cat-4` | `#3f5bb8` | `#6280da` (purple) |
| `cat.5` | `--cat-5` | `#cb4f73` | `#db6686` (rose) |
| `cat.6` | `--cat-6` | `#098db2` | `#089ac3` (azure) |
| `cat.7` | `--cat-7` | `#8c8305` | `#97902a` (olive) |
| `cat.8` | `--cat-8` | `#0b959b` | `#009298` (cyan) |
<!-- GENERATED:COLOUR_TABLES:END -->

Notes on the sets. Surfaces step by lightness, not by colour: `surface.base` behind everything, `surface.panel` for tables and panels, `surface.sunken` for inputs, read-through cells and wells, `surface.header` for column headers, `surface.rowAlt` for the optional zebra (on by default on tables wider than eight columns), `surface.raised` for popovers and cards; `surface.canvas` equals `canvas.bg`. The four `status.*` pairs serve the six automatic statuses (M-08) as described in §2.3. `mark.required` equals `issue.blocking` by design (a missing required value is a blocking finding, C-28) and is kept as its own token so the mark can be retuned without touching the issue colour. `proposal.old` equals `text.muted` by design: an old value is muted and struck, never coloured. `canvas.unsolved` equals `canvas.pipe`: an unsolved element is the neutral pipe, and in a coloured view it is also drawn at 60% opacity so that form, not only hue, separates it from a low-ratio element (§6.7).

The ten tokens added in V1.1 close the frames' gaps (MOCKS_V1 §5). `pressed.fill` / `pressed.ink` are the latched toggle (G-5): the accent hue at a fill light enough to be a state and an ink dark enough to be read on it, distinct from `selection.band` / `accent.text`, which the frames had borrowed from the segmented control. `stale.band` / `stale.ink` are the results-stale band (G-1): amber like the stale Checked mark, paler than `issue.warningTint` and with its own ink, so that a band that states a fact about the run is not read as a warning. `rail.captionFailed`, `rail.captionStale` and `rail.captionHistorical` equal `status.failedInk`, `stale.ink` and `historical.ink` by design (G-2) and are kept as their own tokens so a caption can be retuned without touching a chip or a band. `canvas.hint` (G-4) is the plate of the hint strip and the keyboard hints on the canvas: translucent over the ground like `canvas.labelBg` but whiter in light and lighter in dark, so a hint is not mistaken for a node plate. `draft.bar` equals `canvas.draft` by design (G-3): the draft row's bar is the ghost's colour because the row and the ghost are one thing. `bar.track` (G-6) is the data bar's track, a neutral one step from `surface.panel` that stays visible on the selection band.

The one colour token added in V1.2 is `canvas.edgeAlt` (R-6): the alternate edge line, the opposite polarity of `canvas.edge` in each theme (a light line in the light theme, a dark line in the dark theme). It exists for one case, the line on a result-coloured fill whose lightness is near the edge's own (§2.6, §6.7); the neutral drawing never uses it. V1.2 also adds plain tokens: `layout.hud.inset`, `layout.hud.width`, `layout.hud.widthWrapped` and `layout.hud.wrapBelow` for the HUD (G-10), `layout.toast.width`, `layout.toast.inset`, `motion.toast.ms` and `motion.toastAction.ms` for the toast (G-8), and `layout.runlog.width` for the run log popover, whose 360 px was prose until now.

### 2.3 Status and evidence labels: the one table (M-08, M-09)

Ruled (D-71 item 4): the six automatic statuses and the two evidence labels take their display form from one table and from nowhere else. The table lives in `tokens.json` (`labels`); the copy below is generated from it between markers, the specimen renders the same data, and `tools/agree.mjs` checks that every label drawn anywhere is a row of it.

<!-- GENERATED:LABEL_TABLE:BEGIN -->
| Raw token | Label | Authority domain, shown with the label | Chip tokens | Kind |
|---|---|---|---|---|
| `MODEL_INCOMPLETE` | Model incomplete | Solver | `status.incompleteFill` / `status.incompleteInk` | status |
| `MECHANICS_SOLVED` | Mechanics solved | Solver | `status.solvedFill` / `status.solvedInk` | status |
| `RULE_INPUTS_INCOMPLETE` | Rule inputs incomplete | Rule pack | `status.incompleteFill` / `status.incompleteInk` | status |
| `USER_RULE_CHECKED` | User-rule checked | Rule pack | `status.solvedFill` / `status.solvedInk` | status |
| `USER_RULE_FAILED` | User-rule failed | Rule pack | `status.failedFill` / `status.failedInk` | status |
| `HUMAN_REVIEW_REQUIRED` | Human review required | Human | `status.reviewFill` / `status.reviewInk` | status |
| `INTERNALLY_VERIFIED` | Internally verified | Evidence | `status.solvedFill` / `status.solvedInk` | evidence |
| `PROVER_CORRELATED` | Prover correlated | Evidence | `status.solvedFill` / `status.solvedInk` | evidence |
<!-- GENERATED:LABEL_TABLE:END -->

Three rules bind every use of a label, on every surface and in the report.

1. **The raw token is reachable in place.** A label is never drawn without its token one act away on the same element: the chip's tooltip reads the token, and a click on the chip opens its popover with the token in `type.family.mono`.
2. **The authority domain is shown with the label.** The chip reads the domain, a middle dot, then the label ("Solver · Mechanics solved", "Rule pack · User-rule checked", "Human · Human review required"). The domain is part of the chip and not only of its popover, because after ruling 3 the chips and their domains are what carries, on the results surfaces, the distinction between what the software computed and what a person decides. The three status domains are the governed ones (Solver, Rule pack, Human). For the two evidence labels the domain word is "Evidence"; that word is this system's and is an open item (§8 item 14).
3. **No label exists outside the table.** No seventh status, no third evidence label (`ENGINEER_ACCEPTED` is never emitted), no abbreviation and no curated synonym; the product's earlier short forms are not kept. The two rule-pack labels take the hyphenated forms of the table everywhere: chips, popovers, tooltips, the report, generated text and the agent's text.

Chip anatomy: 20 px tall, `radius.control`, 12 px; the domain in the chip's ink at weight 400, a middle dot, the label at weight 500; no icon, no border; the fill and ink are the row's `status.*` pair. "Mechanics solved" and "User-rule checked" are neutral on purpose: a solved model is information, not a green light. The popover: the raw token in `type.family.mono`, the authority domain, the run name and time, and the reason when the status has one (the blockers of the solve; the rule inputs that are missing).

**When chips show (decisions 2, 3 and 4; D-4; rulings 4 and 6; Q-21).** The status bar speaks for the current model and its Current run, and for nothing else.

- Before any run and whenever the model cannot be solved, the bar shows one chip, Solver · Model incomplete, and the Issues count names why.
- When the model is complete but no run has solved it, the bar shows no chip (ruled): none of the six statuses applies, an absent status is honest, and no seventh label is added (decision 2).
- After a solved run the bar shows two chips side by side, one per authority domain: Solver · Mechanics solved and Rule pack · User-rule checked, the second replaced by Rule inputs incomplete or User-rule failed when that is the pack's state (decision 3); with no rule pack set, the Solver chip alone.
- Human · Human review required shows on the Review page, beside the rule-pack chip, and in the report (ruled); it shows on no other page.
- After a failed run the bar shows whatever statuses the run record carries. A stopped run that carries none shows no chip and the bar's left end stays empty (Q-21, a default until the engine answers the specification's §11 question 11): the rail caption Failed and the banner carry the state, and Model incomplete is not carried over from before the run, because a chip states a status the run record carries and the interface asserts none of its own.
- After a model change the chips drop: the rail's Results caption reads Stale (`rail.captionStale`), the results header carries the stale band (§5.1) and the status bar says nothing about the run (decision 4).
- A Historical run lights no chip in the status bar (ruling 6; R-9): its recorded statuses are read in the historical band's popover, as labels with their domains and tokens, and on the run record. A chip in the bar would read as the current model's standing, which a Historical record never supplies (§5.1, run standing).

Chips are read, never clicked to change anything; the click opens the popover.

**The evidence chip (M-09).** It follows the same anatomy and the same three rules beside the run name in a results header and in the probe's footer, on `status.solvedFill` / `status.solvedInk`. It appears only when the run record carries evidence (ruled): a run whose record has no evidence entry shows no chip and no placeholder, and the header closes up. It is drawn for a Current run only (§5.1). `ENGINEER_ACCEPTED` is never emitted.

### 2.4 Issue classes (M-10) and the required mark

Severity carries the colour and the glyph; the class is always spelled in words, so the six classes never collapse into one another (C-56).

| Class token | Words on the surface | Severity | Glyph | Colour |
|---|---|---|---|---|
| model-validity blocking (PRD §14.4) | Invalid model | blocking | octagon | `issue.blocking` on `issue.blockingTint` |
| `SOLVE_BLOCKING` | Blocks solve | blocking | octagon | `issue.blocking` |
| `RULE_CHECK_BLOCKING` | Blocks rule check | blocking | octagon | `issue.blocking` |
| `PROVENANCE_WARNING` | Provenance | warning | triangle | `issue.warning` on `issue.warningTint` |
| `ASSUMPTION_WARNING` | Assumption | warning | triangle | `issue.warning` |
| `NONLINEAR_WARNING` | Nonlinear | warning | triangle | `issue.warning` |
| `IP_BOUNDARY_WARNING` | Content boundary | warning | triangle with a lock | `issue.warning` |
| informational | Note | info | circle with i | `issue.info` on `issue.infoTint` |

An issue row reads "Blocks solve · Section missing · node 20". The required mark is an asterisk in `mark.required` at the left of an empty required cell, and the same row carries the octagon in its gutter; a value that exists but is weak (provenance missing, out of range) carries the triangle in the cell's corner instead. Nothing is ever pre-filled to make a mark go away (C-26 to C-29).

### 2.5 The marks of §4, proposals, Checked, historical, display-only

Origins and attachment marks are neutral ink (`mark.origin`, `mark.attachment`): quiet. Proposals are violet (`proposal.band`, `proposal.bar`, `proposal.new`; old values `proposal.old` struck): loud. Checked is neutral ink (`mark.checked`), a human tag and not a colour; stale Checked is amber (`mark.checkedStale`) with a dashed glyph. Historical is a warm neutral band (`historical.tint` / `historical.ink`) with a hatch, never a colour that reads as status; what a Historical run may and may not drive is the run-standing table of §5.1. Display-only (M-15) is form, not hue: monospace on `surface.sunken` inside a dashed `displayOnly.border` with the caption "Display only, not accepted as input" in `text.muted`. Stale results after a model change are an amber band (`stale.band` / `stale.ink`) across the results header, with the result cells hatched in `stale.stripe` (G-1, decision 4): a fact about the run, never a verdict. The engineer's own draft row is a dashed `draft.bar` in the gutter with muted text and the typed value in `accent.text` (G-3): the same blue as the draft ghost, because the row and the ghost are one thing.

### 2.6 The result colour scale

One hue, teal, light to dark, seven anchors `result.scale.1` to `.7` with continuous interpolation in OKLCH between them. In light the near-zero end is the lightest step (L 0.72) and the maximum the darkest (L 0.33), and "more" is further from the pale ground. In dark the anchor flips: the near-zero end is the darkest teal and the maximum the brightest, and in V1.1 the whole dark ramp is re-anchored one step down (decision 11; Q-11) so that the brightest step sits below the edge line: `result.scale.7` is `#74d1cb` (L 0.80) under `canvas.edge` `#c7cbd0` (L 0.84), where V1 had it at L 0.92, brighter than the line that carries the geometry. Shifting the V1 ramp (L 0.50 to 0.92 in steps of 0.07) down by exactly one of its steps would have left the top at L 0.85, still above the edge, so the bottom moved one step down to `result.scale.1` `#045c59` (L 0.43) and the step was narrowed to 0.062, which puts the top at L 0.80 and keeps every adjacent pair at least 0.06 apart in the 8-bit values. The dark pipe neutral sits at L 0.58, between steps 3 and 4: a near-zero element is darker than an unsolved one and a fully loaded element brighter, and in the middle of the range hue (teal against grey) and the 60% dimming of unsolved elements (§6.7) do the separating. The ramp was built and checked with the dataviz method's ordinal validator in both themes against `canvas.bg`: monotone lightness, adjacent steps at least 0.06 apart in OKLCH L, the near-zero step at 2.02:1 (light) and 2.18:1 (dark, was 3.00:1) against the canvas, single hue. The cost of the decision is at the far end: the brightest dark step now reads at 9.55:1 against the ground (was 13.8:1) and the edge line on it at 1.10:1 (§2.9), so on a fully loaded element in dark the geometry is carried by the silhouette against the ground rather than by the line. V1.2 answers this, and the same weakness in light, where the edge line reads at 1.02:1 on step 6, with the alternate edge `canvas.edgeAlt` (R-6; §6.7): on a result-coloured element the line takes whichever of `canvas.edge` and `canvas.edgeAlt` reads better against the element's fill, which keeps the line on a fill at or above 2.75:1 (light) and 2.90:1 (dark) across the whole scale (§2.9).

Legend semantics, the same for ratio and for stress:

- The legend carries the scale for a Current run only. For a Stale or a Historical run it is a note card with no scale, because no result colour is drawn on the current model then (§5.1, run standing; §5.6).
- The legend is a scale, never a verdict. It is a vertical bar with five ticks, the quantity and unit (`Ratio` or `Stress [MPa]`), the rule ID and pack version when the quantity is a ratio (`Rule EXP-A1 · pack 1.2`), and the case name or `Envelope`.
- For ratio the default range is 0 to the greater of 1.0 and the maximum in the run; 1.0 is a labelled tick and nothing changes colour at it. For stress the range is 0 to the maximum. The engineer can set the range; the legend then says `range set`.
- Nothing is coloured that has not been solved: unsolved elements draw in `canvas.unsolved` at 60% opacity and the legend carries an `Unsolved` swatch.
- The probed or selected element's value is a marker on the bar.
- The legend and the ratio cells are the only places the scale appears. In tables, a ratio cell carries a small inline bar (data bar) in the scale colour beside the number; the number stays in `text.primary`.

### 2.7 Load-vector and case colours

Eight categorical slots in a fixed order, assigned in sequence and never cycled, validated with the dataviz validator (`scripts/validate_palette.js`) against `canvas.bg` in both themes. Order: 1 orange, 2 magenta, 3 green, 4 purple, 5 rose, 6 azure, 7 olive, 8 cyan. The order was chosen by enumerating every ordering with orange first and keeping the ones that clear both hard gates in both themes, then taking the best minimum CVD separation.

Validator findings (adjacent pairlist, the default for vectors drawn beside one another): light, all eight inside the band, chroma above the floor, worst adjacent CVD ΔE 10.4 (deutan), worst adjacent normal-vision ΔE 16.9, all eight at or above 3:1 on the light canvas; dark, worst adjacent CVD ΔE 10.2 (protan), normal-vision ΔE 16.9, all eight at or above 3:1 on the dark canvas. All-pairs (any two vectors can be neighbours in a canvas): the first three slots pass in both themes (worst 8.1 light, 6.1 dark in the floor band); the first six do not (rose against green and magenta collapse under deutan). The consequence is a rule, not a colour: colour is never the only carrier of a load's kind. Kind is carried by the arrow's form (§6.4), by the label on hover and selection, and by the legend row that pairs glyph, colour and name.

Assignment: load kinds take slots 1 to 6 in fixed order: force → `cat.1`, moment → `cat.2`, specified displacement → `cat.3`, uniform and weight → `cat.4`, wind → `cat.5`, seismic → `cat.6`. Cases take slots by row order in the load-case table; a ninth and later case takes `canvas.vector` neutral with its label. When a case is selected on the Loads page, its loads draw in the case's slot and every other load draws in `canvas.vector` at 40% opacity. When result colour is on the tube, and while a proposal ghost is drawn in the canvas, vectors draw in `canvas.vector` only: one colour job per view, and the ghost is the one loud thing when it is there (D-10).

### 2.8 The canvas palette

Both themes carry the same roles: `canvas.bg`; `canvas.gridMajor` and `canvas.gridMinor` for the ground grid; `canvas.pipe` and `canvas.pipeShade` for the tube and its shaded side; `canvas.edge` for the edge line and `canvas.edgeAlt` for its alternate on result-coloured fills (R-6); `canvas.glyph` and `canvas.glyphFill` for restraint glyphs; `canvas.label` on `canvas.labelBg` for node labels and glyph plates; `canvas.hint` for the hint strip and keyboard hints (G-4); `canvas.vector`; `canvas.selection` and `canvas.hover` halos; `canvas.draft` for the engineer's routing ghost; `canvas.proposalGhost`; `canvas.deformGhost`; `canvas.unsolved`; `canvas.axisX/Y/Z` for the triad and the compass. In light the drawing is a mid grey tube with a dark edge on a pale ground; in dark a lighter grey tube with a light edge on a near-black ground. Neither theme is a flip of the other: each is stepped so the edge reads against the tube and the tube against the ground (findings in §2.9).

### 2.9 Contrast findings

Method. For every text and mark pairing in the system the WCAG 2.x contrast ratio (relative luminance, sRGB) was computed from `tokens.json` for both themes; translucent tokens are composited over the surface named after the slash before the ratio is taken. The table is generated by `tools/contrast.mjs` from the token file and spliced in between markers; the specimen carries the same pair list and computes the same ratios live from the same embedded tokens, and `tools/agree.mjs` checks that the table, the pair list and the tokens agree. V1.1 adds eighteen pairs for the new tokens and for the edge line and the data bar on the re-anchored scale. V1.2 adds six pairs: the edge line on the step nearest its own lightness in light, and the edge line and its alternate either side of the step at which the line switches (R-6). These are findings for the owner's target, which is TBD (M-16, C-91 to C-94): no conformance is claimed and no target is named here.

<!-- GENERATED:CONTRAST_TABLE:BEGIN -->
| Foreground | Background | Role | Light | Dark |
|---|---|---|---|---|
| `text.primary` | `surface.panel` | text | 15.06:1 | 12.04:1 |
| `text.primary` | `surface.base` | text | 13.19:1 | 13.61:1 |
| `text.primary` | `surface.sunken` | text | 14.02:1 | 12.84:1 |
| `text.primary` | `surface.header` | text | 13.51:1 | 11.29:1 |
| `text.primary` | `surface.rowAlt` | text | 14.41:1 | 11.73:1 |
| `text.primary` | `surface.raised` | text | 15.06:1 | 10.51:1 |
| `text.primary` | `selection.band` | text | 12.83:1 | 8.85:1 |
| `text.primary` | `proposal.band` | text | 13.14:1 | 9.98:1 |
| `text.primary` | `hover.wash/surface.panel` | text | 13.46:1 | 10.03:1 |
| `text.primary` | `issue.blockingTint` | text | 13.10:1 | 10.58:1 |
| `text.primary` | `issue.warningTint` | text | 13.48:1 | 10.02:1 |
| `text.primary` | `issue.infoTint` | text | 13.20:1 | 10.26:1 |
| `text.primary` | `historical.tint` | text | 13.16:1 | 11.11:1 |
| `text.primary` | `stale.band` | text | 14.02:1 | 11.14:1 |
| `text.primary` | `disabled.fill` | text | 12.61:1 | 11.10:1 |
| `text.secondary` | `surface.panel` | text | 6.26:1 | 6.91:1 |
| `text.secondary` | `surface.base` | text | 5.48:1 | 7.81:1 |
| `text.secondary` | `surface.header` | text | 5.62:1 | 6.48:1 |
| `text.secondary` | `surface.sunken` | text | 5.83:1 | 7.37:1 |
| `text.secondary` | `selection.band` | text | 5.33:1 | 5.08:1 |
| `text.secondary` | `proposal.band` | text | 5.46:1 | 5.73:1 |
| `text.secondary` | `canvas.hint/canvas.bg` | canvas hint text | 6.15:1 | 6.21:1 |
| `text.muted` | `surface.panel` | text | 3.79:1 | 4.41:1 |
| `text.muted` | `surface.base` | text | 3.32:1 | 4.99:1 |
| `text.muted` | `surface.header` | text | 3.40:1 | 4.14:1 |
| `text.muted` | `surface.sunken` | text | 3.53:1 | 4.70:1 |
| `text.disabled` | `surface.panel` | text (disabled) | 2.31:1 | 2.48:1 |
| `text.disabled` | `surface.base` | text (disabled) | 2.03:1 | 2.80:1 |
| `text.disabled` | `surface.sunken` | text (disabled) | 2.15:1 | 2.64:1 |
| `text.link` | `surface.panel` | text | 6.07:1 | 7.38:1 |
| `text.link` | `surface.base` | text | 5.32:1 | 8.34:1 |
| `accent.text` | `surface.panel` | text | 6.67:1 | 7.95:1 |
| `accent.text` | `selection.band` | text | 5.69:1 | 5.84:1 |
| `text.inverse` | `accent.fill` | text on button | 5.12:1 | 5.84:1 |
| `text.inverse` | `accent.fillHover` | text on button | 6.09:1 | 6.76:1 |
| `pressed.ink` | `pressed.fill` | latched toggle | 5.97:1 | 5.66:1 |
| `pressed.fill` | `surface.base` | latched toggle fill | 1.11:1 | 1.82:1 |
| `proposal.new` | `surface.panel` | text | 7.42:1 | 7.11:1 |
| `proposal.new` | `proposal.band` | text | 6.47:1 | 5.89:1 |
| `proposal.new` | `surface.base` | text | 6.50:1 | 8.04:1 |
| `proposal.old` | `surface.panel` | text (struck) | 3.79:1 | 4.41:1 |
| `proposal.old` | `proposal.band` | text (struck) | 3.30:1 | 3.66:1 |
| `mark.checked` | `surface.panel` | mark | 6.26:1 | 6.91:1 |
| `mark.checkedStale` | `surface.panel` | mark | 4.37:1 | 7.84:1 |
| `mark.checkedStale` | `surface.base` | mark | 3.82:1 | 8.85:1 |
| `mark.origin` | `surface.panel` | mark | 3.79:1 | 4.41:1 |
| `mark.origin` | `surface.rowAlt` | mark | 3.62:1 | 4.30:1 |
| `mark.origin` | `surface.header` | mark | 3.40:1 | 4.14:1 |
| `mark.attachment` | `surface.panel` | mark | 6.26:1 | 6.91:1 |
| `mark.required` | `surface.panel` | mark | 6.05:1 | 6.01:1 |
| `mark.required` | `surface.sunken` | mark | 5.63:1 | 6.40:1 |
| `draft.bar` | `surface.panel` | draft row bar | 5.12:1 | 6.40:1 |
| `draft.bar` | `selection.band` | draft row bar | 4.36:1 | 4.70:1 |
| `issue.blocking` | `surface.panel` | text | 6.05:1 | 6.01:1 |
| `issue.blocking` | `issue.blockingTint` | text | 5.26:1 | 5.28:1 |
| `issue.blocking` | `surface.base` | text | 5.30:1 | 6.79:1 |
| `issue.warning` | `surface.panel` | text | 4.37:1 | 7.84:1 |
| `issue.warning` | `issue.warningTint` | text | 3.91:1 | 6.52:1 |
| `issue.warning` | `surface.base` | text | 3.82:1 | 8.85:1 |
| `issue.info` | `surface.panel` | text | 5.94:1 | 6.97:1 |
| `issue.info` | `issue.infoTint` | text | 5.21:1 | 5.94:1 |
| `status.incompleteInk` | `status.incompleteFill` | chip text | 6.47:1 | 7.73:1 |
| `status.solvedInk` | `status.solvedFill` | chip text | 9.91:1 | 8.50:1 |
| `status.failedInk` | `status.failedFill` | chip text | 6.77:1 | 7.19:1 |
| `status.reviewInk` | `status.reviewFill` | chip text | 7.32:1 | 7.82:1 |
| `status.incompleteInk` | `surface.panel` | chip text | 7.23:1 | 9.30:1 |
| `status.failedInk` | `surface.panel` | chip text | 7.79:1 | 8.18:1 |
| `status.reviewInk` | `surface.panel` | chip text | 8.35:1 | 9.18:1 |
| `rail.captionFailed` | `surface.base` | rail caption | 6.82:1 | 9.24:1 |
| `rail.captionStale` | `surface.base` | rail caption | 6.09:1 | 10.57:1 |
| `rail.captionHistorical` | `surface.base` | rail caption | 6.51:1 | 9.53:1 |
| `historical.ink` | `historical.tint` | text | 6.49:1 | 7.78:1 |
| `historical.ink` | `surface.panel` | text | 7.43:1 | 8.43:1 |
| `stale.ink` | `stale.band` | band text | 6.47:1 | 8.65:1 |
| `stale.ink` | `surface.panel` | band text | 6.95:1 | 9.36:1 |
| `border.hairline` | `surface.panel` | hairline | 1.43:1 | 1.37:1 |
| `border.hairline` | `surface.base` | hairline | 1.25:1 | 1.55:1 |
| `border.strong` | `surface.panel` | border | 2.00:1 | 2.09:1 |
| `border.strong` | `surface.base` | border | 1.75:1 | 2.36:1 |
| `focus.ring` | `surface.panel` | focus ring | 4.71:1 | 6.39:1 |
| `focus.ring` | `surface.base` | focus ring | 4.13:1 | 7.22:1 |
| `focus.ring` | `selection.band` | focus ring | 4.01:1 | 4.69:1 |
| `selection.bar` | `selection.band` | mark | 4.36:1 | 4.38:1 |
| `proposal.bar` | `proposal.band` | mark | 4.61:1 | 5.06:1 |
| `displayOnly.border` | `surface.sunken` | border | 1.86:1 | 2.23:1 |
| `stale.stripe` | `surface.panel` | mark | 1.53:1 | 1.61:1 |
| `bar.track` | `surface.panel` | bar track | 1.29:1 | 1.42:1 |
| `bar.track` | `selection.band` | bar track | 1.10:1 | 1.04:1 |
| `canvas.label` | `canvas.labelBg/canvas.bg` | canvas text | 14.65:1 | 13.58:1 |
| `canvas.label` | `canvas.bg` | canvas text | 12.83:1 | 13.92:1 |
| `canvas.gridMajor` | `canvas.bg` | hairline | 1.31:1 | 1.31:1 |
| `canvas.gridMinor` | `canvas.bg` | hairline | 1.12:1 | 1.12:1 |
| `canvas.pipe` | `canvas.bg` | canvas mark | 1.97:1 | 4.00:1 |
| `canvas.pipeShade` | `canvas.bg` | canvas mark | 3.09:1 | 2.41:1 |
| `canvas.edge` | `canvas.pipe` | canvas mark | 4.00:1 | 2.63:1 |
| `canvas.edge` | `canvas.bg` | canvas mark | 7.88:1 | 10.50:1 |
| `canvas.glyph` | `canvas.bg` | canvas mark | 10.41:1 | 12.68:1 |
| `canvas.glyph` | `canvas.glyphFill` | canvas mark | 9.05:1 | 8.75:1 |
| `canvas.glyphFill` | `canvas.bg` | canvas mark | 1.15:1 | 1.45:1 |
| `canvas.vector` | `canvas.bg` | canvas mark | 7.88:1 | 10.50:1 |
| `canvas.selection` | `canvas.bg` | canvas mark | 4.36:1 | 7.39:1 |
| `canvas.selection` | `canvas.pipe` | canvas mark | 2.21:1 | 1.85:1 |
| `canvas.hover` | `canvas.bg` | canvas mark | 2.67:1 | 4.69:1 |
| `canvas.draft` | `canvas.bg` | canvas mark | 4.36:1 | 7.39:1 |
| `canvas.proposalGhost` | `canvas.bg` | canvas mark | 4.50:1 | 7.05:1 |
| `canvas.deformGhost` | `canvas.bg` | canvas mark | 3.09:1 | 2.41:1 |
| `canvas.unsolved` | `canvas.bg` | canvas mark | 1.97:1 | 4.00:1 |
| `canvas.axisX` | `canvas.bg` | canvas mark | 4.48:1 | 6.52:1 |
| `canvas.axisY` | `canvas.bg` | canvas mark | 3.89:1 | 7.77:1 |
| `canvas.axisZ` | `canvas.bg` | canvas mark | 4.16:1 | 7.38:1 |
| `result.scale.1` | `canvas.bg` | result scale on canvas | 2.02:1 | 2.18:1 |
| `result.scale.2` | `canvas.bg` | result scale on canvas | 2.56:1 | 2.88:1 |
| `result.scale.3` | `canvas.bg` | result scale on canvas | 3.33:1 | 3.76:1 |
| `result.scale.4` | `canvas.bg` | result scale on canvas | 4.41:1 | 4.88:1 |
| `result.scale.5` | `canvas.bg` | result scale on canvas | 5.80:1 | 6.20:1 |
| `result.scale.6` | `canvas.bg` | result scale on canvas | 7.72:1 | 7.75:1 |
| `result.scale.7` | `canvas.bg` | result scale on canvas | 10.20:1 | 9.55:1 |
| `result.scale.1` | `canvas.unsolved` | result scale beside unsolved | 1.02:1 | 1.83:1 |
| `result.scale.7` | `canvas.unsolved` | result scale beside unsolved | 5.18:1 | 2.39:1 |
| `canvas.edge` | `result.scale.1` | edge line on the near-zero step | 3.91:1 | 4.81:1 |
| `canvas.edge` | `result.scale.7` | edge line on the brightest step | 1.29:1 | 1.10:1 |
| `canvas.edge` | `result.scale.6` | edge line on the step nearest its own lightness in light | 1.02:1 | 1.35:1 |
| `canvas.edge` | `result.scale.2` | edge line on the last step it keeps | 3.08:1 | 3.64:1 |
| `canvas.edgeAlt` | `result.scale.2` | alternate edge on the last step it leaves | 2.44:1 | 2.30:1 |
| `canvas.edgeAlt` | `result.scale.3` | alternate edge on the first step it takes | 3.18:1 | 3.00:1 |
| `canvas.edgeAlt` | `result.scale.7` | alternate edge on the far step | 9.74:1 | 7.63:1 |
| `canvas.edgeAlt` | `canvas.bg` | alternate edge against the ground | 1.05:1 | 1.25:1 |
| `result.scale.1` | `surface.raised` | legend swatch | 2.37:1 | 1.65:1 |
| `result.scale.2` | `surface.raised` | legend swatch | 3.00:1 | 2.17:1 |
| `result.scale.3` | `surface.raised` | legend swatch | 3.91:1 | 2.84:1 |
| `result.scale.4` | `surface.raised` | legend swatch | 5.18:1 | 3.68:1 |
| `result.scale.5` | `surface.raised` | legend swatch | 6.81:1 | 4.68:1 |
| `result.scale.6` | `surface.raised` | legend swatch | 9.06:1 | 5.85:1 |
| `result.scale.7` | `surface.raised` | legend swatch | 11.97:1 | 7.21:1 |
| `result.scale.1` | `bar.track` | data bar on its track | 1.84:1 | 1.33:1 |
| `result.scale.7` | `bar.track` | data bar on its track | 9.29:1 | 5.83:1 |
| `cat.1` | `canvas.bg` | load vector on canvas | 3.29:1 | 5.21:1 |
| `cat.2` | `canvas.bg` | load vector on canvas | 4.02:1 | 4.67:1 |
| `cat.3` | `canvas.bg` | load vector on canvas | 3.19:1 | 5.00:1 |
| `cat.4` | `canvas.bg` | load vector on canvas | 5.26:1 | 4.58:1 |
| `cat.5` | `canvas.bg` | load vector on canvas | 3.65:1 | 5.10:1 |
| `cat.6` | `canvas.bg` | load vector on canvas | 3.27:1 | 5.23:1 |
| `cat.7` | `canvas.bg` | load vector on canvas | 3.34:1 | 5.16:1 |
| `cat.8` | `canvas.bg` | load vector on canvas | 3.09:1 | 4.53:1 |
| `cat.1` | `surface.panel` | case dot in table | 3.86:1 | 4.51:1 |
| `cat.2` | `surface.panel` | case dot in table | 4.72:1 | 4.04:1 |
| `cat.3` | `surface.panel` | case dot in table | 3.74:1 | 4.32:1 |
| `cat.4` | `surface.panel` | case dot in table | 6.18:1 | 3.96:1 |
| `cat.5` | `surface.panel` | case dot in table | 4.28:1 | 4.41:1 |
| `cat.6` | `surface.panel` | case dot in table | 3.84:1 | 4.53:1 |
| `cat.7` | `surface.panel` | case dot in table | 3.92:1 | 4.47:1 |
| `cat.8` | `surface.panel` | case dot in table | 3.63:1 | 3.92:1 |
<!-- GENERATED:CONTRAST_TABLE:END -->

Reading the findings.

- Body text: `text.primary` on every surface sits between 8.9:1 and 15.1:1 across both themes, and on the stale band at 14.0:1 (light) and 11.1:1 (dark); `text.secondary` between 5.1:1 and 7.8:1, and on the hint strip over the canvas at 6.2:1 in both themes; `text.muted` between 3.3:1 and 5.0:1. Muted ink is used for origins, captions, state words and the struck old value, never for a value the engineer reads.
- `text.disabled` measures 2.0:1 to 2.8:1, including the disabled "Case: all" selector on a sunken field (2.15:1 light, 2.64:1 dark). That is the disabled state; the label stays legible enough to be found and the reason is in a tooltip. If the owner's target requires more, `text.disabled` is the one token to lift.
- Chips and captions: every status ink on its fill is above 6.4:1 in both themes; the warning ink on its tint is the lowest coloured text at 3.91:1 (light); the rail's three captions sit between 6.1:1 and 10.6:1 on the base; the stale band's ink on its band at 6.5:1 (light) and 8.7:1 (dark).
- Latched toggle: `pressed.ink` on `pressed.fill` at 6.0:1 (light) and 5.7:1 (dark); the fill itself against the base at 1.1:1 (light) and 1.8:1 (dark), which is what a state fill should be: a wash, read through its ink and its glyph.
- Marks: origins at 3.4:1 to 4.4:1, attachments and Checked at 6.3:1 to 6.9:1, stale Checked at 3.8:1 (light, on base) to 8.9:1, required and blocking above 5.2:1, the focus ring at 4.0:1 to 7.2:1, the draft row's bar at 5.1:1 and 6.4:1 on the panel and 4.4:1 and 4.7:1 on the selection band.
- Hairlines, grid lines, the stale hatch and the data bar's track are deliberately recessive (1.0:1 to 1.6:1); they separate, they do not inform. `border.strong`, which frames inputs and the table, measures 1.8:1 to 2.4:1.
- Canvas: the light pipe neutral is 1.97:1 against the light ground, by design a pale drawing; its edge line reads at 7.9:1 against the ground and 4.0:1 against the tube, so the geometry is carried by the edge, and the dark tube reads at 4.0:1. The selection halo is drawn against the ground (4.4:1 light, 7.4:1 dark), not against the tube. The hover halo is 2.7:1 in light. Glyph ink is above 10:1 against the ground in both themes.
- Result scale: on the canvas the steps run from 2.0:1 to 10.2:1 (light) and, after the re-anchoring, from 2.2:1 to 9.6:1 (dark, was 3.0:1 to 13.8:1); in the legend card from 2.4:1 to 12.0:1 (light) and 1.7:1 to 7.2:1 (dark). The edge line reads at 3.9:1 (light) and 4.8:1 (dark) on the near-zero step and at 1.3:1 (light) and 1.1:1 (dark) on the brightest step: on a fully loaded element the line nearly merges with the tube and the silhouette against the ground carries the geometry, in both themes; the lowest reading of all is 1.02:1, the edge line on step 6 in light. With the alternate edge of V1.2 the line that is drawn on a fill is the better of the two: `canvas.edge` keeps steps 1 and 2 (3.1:1 and 3.6:1 on step 2) and `canvas.edgeAlt` takes over from step 3 (3.2:1 and 3.0:1) to step 7 (9.7:1 and 7.6:1); along the interpolated scale the switch falls at about a quarter of the range in light and a third in dark, and the lowest reading of the drawn line anywhere on the scale is 2.75:1 (light) and 2.90:1 (dark). Against the ground the alternate edge is recessive by construction (1.05:1 light, 1.25:1 dark): at the silhouette of a strongly coloured element the fill itself carries the outline (10.2:1 light, 9.6:1 dark). The near-zero step beside an unsolved element is 1.0:1 in light and 1.8:1 in dark, which is why unsolved elements are also dimmed (§2.6). The data bar on its track runs from 1.8:1 to 9.3:1 (light) and 1.3:1 to 5.8:1 (dark); the number beside the bar stays in `text.primary`.
- Categorical: every slot is at or above 3.1:1 on the canvas and 3.6:1 as a dot on a panel in both themes.

## 3. Iconography

### 3.1 Style rules

A 16 px grid with a 1 px safe margin; 1.5 px strokes with round caps and joins; corners at 1.5 px radius; `currentColor` so the icon takes the text token of its context; no fills except the selected rail item, whose glyph fills at the same stroke; optical sizes 12 (gutter, marks, chips; 1.25 px stroke), 16 (controls, menus, HUD) and 20 (rail). No shields, no ticks in circles, no traffic-light discs: nothing that reads as approval. Every icon has a visible text label or a tooltip with the same words as its menu item (C-97); the rail shows labels under its icons, the HUD shows tooltips with the accelerator. A tooltip names the control first and its accelerator after it in parentheses; the accelerator is never the whole tooltip (§7.6).

### 3.2 The set

| Group | Name | Glyph | Label or tooltip |
|---|---|---|---|
| Rail | Model | two nodes joined by a line with one bend | Model |
| Rail | Loads | an arrow landing on a line | Loads |
| Rail | Results | three horizontal bars of unequal length | Results |
| Rail | Review | a page with a pen line | Review |
| Rail foot | Libraries | three stacked plates | Libraries |
| Rail foot | Rules | a diagonal ruler with ticks | Rules |
| Rail foot | Issues | a circle with an exclamation and the count | Issues |
| Views | Table | a 3 × 3 grid | Table |
| Views | Model | an isometric cube outline | Model |
| Views | Both | a rectangle split vertically, grid lines left, a cube dot right | Both |
| HUD | Fit | four corner brackets | Fit (F); first in the HUD and present at every canvas width (§5.6) |
| HUD | View presets | a cube with one face marked; menu Iso, Top, Front, Right, Report figure | View |
| HUD | Section | a plane cutting a cube | Section |
| HUD | Isolate | one solid square among dashed ones | Isolate selection (I) |
| HUD | Hide | an eye with a slash | Hide selection (H); while anything is hidden the HUD carries the count as a text button, "3 hidden · Show all" |
| HUD | Labels | a tag reading 10 | Node labels (L): Budget, All, Off |
| HUD | Deform | a bent line beside a straight ghost | Deformation (D) |
| HUD | Probe | a crosshair with a centre dot | Probe (P) |
| HUD | Route | a polyline ending in an arrowhead | Route (R) |
| HUD | Restrain | the +Y triangle under a line | Add restraint (S) |
| Table | Add row | a plus in a row outline | Add row (⌥↩) |
| Table | Delete rows | a minus in a row outline | Delete rows (⌫) |
| Table | Sort | two small arrows | Sort |
| Table | Filter | a funnel | Filter |
| Table | Columns | three vertical bars | Columns |
| Table | Copy | two overlapping sheets | Copy rows (⌘C) |
| Table | Paste | a clipboard over a grid | Paste with mapping (⌘V) |
| Table | Check | a check | Check rows (⌘⇧K); Clear check when every selected row is Checked |
| Table | Expand | a chevron, right closed, down open | Open joined row (⌘↩) |
| Table | Changes since | a clock with a returning arrow | Changed since… |
| Table | Unchecked | an empty check box | Unchecked rows |
| Table | Origins | a small dot with rays | Show origins |
| Table | Read-through | two overlapping cells | Read-through on / off |
| Toolbar | Run | a right-pointing triangle in a rounded frame; a square while running | Run; while running, Stop |
| Toolbar | Undo | an arrow turning back to the left | Undo (⌘Z), with the name of the operation it would undo |
| Toolbar | Redo | an arrow turning forward to the right | Redo (⇧⌘Z) |
| Toolbar | Inspector | a window with its right part ruled off | Inspector (⌘I); latched on `pressed.fill` while the Both-view inspector is open |
| Toolbar | Agent | a speech bubble with a spark | Agent (⌘⇧G); latched on `pressed.fill` while the column is open |
| Panels | Close | a cross | Close (⎋): the inspector in Both view, the issues drawer, a row expansion, a pinned probe, a toast, a dialog |
| Panels | Collapse | a chevron, down open, up closed | Collapse or open the table drawer; collapse the agent column to its strip |
| Agent | Send | a paper dart | Send (⌘↩) |
| Compass | Reverse | two opposed arrows | Reverse the axis (−) |
| Compass | Place | a check | Place node *50* (↩) |
| Compass | Cancel | a cross | Cancel the route (⎋) |
| Review | Report preview | a page with a magnifier at its corner | Report preview |
| Review | Export | a tray with an up arrow | Export… |
| Review | Snapshot | a camera outline | Snapshot… |
| Review | Comment | a speech bubble | Comment |
| Results | Stale | a dashed circle with a centre dot | Model changed since the run |
| Marks | see §4 | | |

V1.2 (G-7): the three Review icons, Report preview, Export and Snapshot, are in the specimen's sprite and the Review header is drawn with them; the icons added for the pointer rule (Undo, Redo, Inspector, Copy, Send, Reverse) are in the sprite too, and Check, Place, Cancel, Close and Collapse reuse the check, the cross and the chevrons already there.

## 4. The marks vocabulary

The rule from the direction record §10: origins quiet, proposals loud. Origins are a 12 px neutral glyph in the gutter; proposals are a band, a bar, a diamond and a coloured new value. Everything else sits between.

Placements. The **row gutter** is 32 px wide with two 12 px slots: the origin slot on the left, the state slot on the right. The **marks column** is the last column, 72 px, with three fixed 20 px slots: restraint, load, node data. A **cell corner** mark is a 5 px triangle in the top-right corner of a cell. A **row band** is a fill across the row with a 3 px bar on its left edge. Tooltips appear on hover after 300 ms and on focus with Space. Pointer reveal, the same for every mark (the pointer rule, §5): a click on a glyph opens that mark's popover with its tooltip text and, as its last line, the link "All marks on this row", which opens the row's marks popover; a click on the gutter beside the glyphs selects the row; a click on a marks-column slot opens that attachment's joined row. An empty state slot shows a faint check on hover ("Check"), as the marks column's empty slots show a faint plus; a click sets the mark, and a Checked glyph's popover carries Check again and Clear check as buttons. Keyboard reveal, the accelerators of the same, the same for every mark: `←` from the Node cell focuses the gutter, `→` from the last column focuses the marks column, Space on a focused gutter or marks slot opens that mark's popover with its tooltip text, and `⌥I` on any row opens the row's marks popover listing every mark the row carries with its tooltip. Filters live in the footer's Filter control and in the rail's Issues item.

| Mark | Glyph (12 px) | Token | Placement | Tooltip | Keyboard reveal | Filter |
|---|---|---|---|---|---|---|
| Entered | nothing; a dot when origins are shown | `mark.origin` | origin slot | Entered by *name* · *date time* | Space on the slot; ⌥I | Origins: Entered |
| Accepted from a proposal | diamond outline with a centre dot | `mark.origin` | origin slot | Accepted from proposal *P-12* by *name* · *date time* · rationale on the record | Space; ⌥I | Origins: Accepted |
| Propagated | an arrow onto a bar | `mark.origin` | origin slot; and cell corner on each propagated cell: the tick only, and a cell typed over loses it (D-2) | Propagated from node *10* (*Section*, *Material*, *Load*) · typing over makes it entered | Space; ⌥I; Space on the cell | Origins: Propagated |
| Generated | a four-point spark | `mark.origin` | origin slot | Generated by rule pack *name* *version* · *date time* | Space; ⌥I | Origins: Generated |
| Imported | a tray with a down arrow | `mark.origin` | origin slot | Imported from *file* · *date time* · provenance in Libraries | Space; ⌥I | Origins: Imported |
| Proposed row | filled diamond | `proposal.new`; band `proposal.band`; bar `proposal.bar` | origin slot while pending; row band | Proposed by the agent · *P-12* · Accept ⌘⇧A · Reject ⌘⇧R · old and new values in the cells | Space; ⌥I | Proposed |
| Proposed cell | new value in `proposal.new` with the corner tick; the struck old value in `proposal.old` before it only when the column has room for both, otherwise in the tooltip and on the card | as above | inside the cell; cell corner | *field*: *old* → *new* *unit* · Accept this row | Space on the cell | Proposed |
| Checked | a check | `mark.checked` | state slot | Checked by *name* · *date time* · bound to this row's content · not a software status | Space; ⌥I | Unchecked (inverse) |
| Checked, stale | a dashed check with a small dot | `mark.checkedStale` | state slot | Checked by *name* · *date time* · the row changed since · Check again or Clear | Space; ⌥I | Unchecked; Stale |
| Restraint | the +Y triangle under a line | `mark.attachment` | marks column, slot 1 | Restraint · *+Y, gap 3 mm, μ 0.3* · ⌘↩ opens the row | ↩ on the slot; ⌘↩ | Has restraint |
| Load | an arrow onto a dot | `mark.attachment` | marks column, slot 2 | Load · *Force −2000 N in Y (W)* · ⌘↩ opens the row | ↩ on the slot; ⌘↩ | Has load |
| Node data | a line with a short branch stub | `mark.attachment` | marks column, slot 3 | Node data · *Welding tee; flange WN* · ⌘↩ opens the row | ↩ on the slot; ⌘↩ | Has node data |
| Issue, blocking | octagon | `issue.blocking` | state slot; cell ring on the offending cell | *Blocks solve* · *Section missing* · Show in Issues | Space; ⌥I; ⌘⇧I opens Issues | Issues: class |
| Issue, warning | triangle | `issue.warning` | state slot; cell corner | *Provenance* · *Material source not recorded* | Space; ⌥I | Issues: class |
| Issue, note | circle with i | `issue.info` | state slot | *Note* · *Assumption recorded for review* | Space; ⌥I | Issues: class |
| Required | asterisk | `mark.required` | inside the empty cell, left, on the first element's row; the start node's row shows "—" in `text.muted` under Section, Material and Load and carries no mark (decision 7, D-1) | Required to solve · *Section* | Space on the cell | Issues: Blocks solve |
| Historical run (M-13) | a clock | `historical.ink` on `historical.tint`; caption `rail.captionHistorical` | band across the results header; caption on the rail | the band's own text, "Historical saved run · Run a fresh solve to establish current results."; its information control opens the popover, which carries "Historical results cannot drive current overlays, rule checks, comparisons or report readiness." and the run's recorded statuses (ruling 6) | a click on the band's information control; Space on the band | Results: Historical |
| Stale results (after a model change) | a dashed circle with a centre dot | `stale.ink` on `stale.band`; cells hatched `stale.stripe`; caption `rail.captionStale` | band across the results header; caption on the rail | Model changed since Run *03* · *proposal P-12 row 1 accepted at 16:31* · Run again | Space on the band | Results: Stale |
| Draft row | a dashed 3 px bar at the gutter's left edge, no glyph | `draft.bar`; text `text.muted`; the typed value `accent.text` | the row being routed or a paste-preview row | Draft · ↩ places node *50* · ⎋ cancels | — | none |
| Comment (Review page only) | a speech bubble | `mark.origin` | state slot of a referenced live row | Comment by *name* · *date time* · Show in comments | Space; ⌥I | Comments: referenced |
| Display-only expression (M-15) | lines in a dashed frame | `displayOnly.border` | the rendered-expression cell or caption | Display only, not accepted as input · notation not frozen | Space on the cell | none |

Priority when a slot is contested. Origin slot: proposed wins while a proposal is pending; otherwise the origin. State slot: blocking, then warning, then stale Checked, then Checked, then comment, then note; the ⌥I popover always lists everything. Row band: the selection band paints over the proposal band, and the proposal bar and diamond remain, so a selected proposed row is unmistakably both.

## 5. Components

**The pointer rule (general; ROOT's reading of the owner's amendment to Q-20).** The owner's words were about Q-20, the refit on dock and undock: the engineer should not have to press buttons there; keyboard inputs are acceptable, but the action needs a primary control operated by a mouse click. Applying that to the whole system is ROOT's reading, recorded as a reading in `../../../../_DECISIONS/D-71_RULING_2026-09-18.md` so that the owner can correct it; this system states the general rule on that footing, and if the reading is corrected the rule narrows to Q-20 and the controls it produced stay as ordinary design choices. The rule: every action has a visible primary control operated by a mouse click. A keyboard shortcut is an accelerator for a control that exists, never the only way. The control is on the surface where the action applies (a button, a HUD tool, a toggle, a chevron, a row or footer control, a link in a popover) for anything in the working loop; the menu bar and the command palette also list every command, and are the home of the commands outside the working loop. A double-click, a modifier-click, a hover and a drag may accelerate too, but none of them is the primary control, because none of them is visible. Typing a value is input, not an action: a number has to be typed, and the control for it is the cell or the field, which a click focuses. The engineer is never asked to press a key in order to see, fit, open, close, place, send, check or undo anything.

V1.1 was checked against the rule action by action. The table lists every action that had only a key in V1.1, or a key and an invisible gesture, with the control V1.2 gives it or the control that was already there and is now named; the last rows are the actions found to be already conformant.

| Action | In V1.1 | Primary control in V1.2 | Where specified |
|---|---|---|---|
| Open the Both-view inspector | ⌘I; a double-click | The toolbar's Inspector toggle, latched while open | §5.2, §5.3 |
| Close the Both-view inspector | Escape | The same toggle; the close control in the inspector's header | §5.3 |
| Open and collapse the agent column | ⌘⇧G, beside the toolbar's Agent toggle | The Agent toggle (found); a click on the strip opens the column; the collapse control in the column's header | §5.2, §5.4 |
| Fit | F, beside the HUD's Fit tool | The Fit tool (found), now first in the HUD and present at every canvas width; a fitted camera also refits by itself | §0, §5.6 |
| Isolate, Hide, Labels, Deformation, Probe, Route, Add restraint | I, H, L, D, P, R, S, beside the HUD tools | The HUD tools (found) | §3.2, §5.6 |
| Show what is hidden | none stated | The HUD's count, a text button: "3 hidden · Show all" | §6.6 |
| Routing: choose the axis | ⇥ or an arrow key | A click on an axis handle of the compass | §5.6 |
| Routing: reverse the axis | − | The Reverse control beside the length field; a click on the active handle's opposite stub | §5.6 |
| Routing: place the node | ↩ | The Place control beside the length field; "Place node 50" in the inspector's routing block | §5.3, §5.6 |
| Routing: cancel | ⎋, beside the routing block's Cancel in the inspector | The Cancel control beside the length field, present when the inspector is closed too; the routing block's Cancel (found) | §5.3, §5.6 |
| Routing: bend at the node | B, beside the routing block's control | The routing block's "Bend at 40…" (found) | §5.3 |
| Unpin the probe | Escape | The close control on the pinned card; a second click on the node | §5.6 |
| Start editing a cell | ↩, or typing | A click on the focused cell (a second click, or a double-click on any cell) | §5.1 |
| Commit an edit | ↩, ⇥ | A click on any other cell; the footer's edit chip, Commit | §5.1 |
| Cancel an edit | ⎋ | The footer's edit chip, Cancel | §5.1 |
| Open a row's expansion (element fields, the combination editor, hanger selection) | ⌘↩; Space on the Expression cell | The expansion chevron in the cell that owns the expansion | §5.1 |
| Open a joined attachment row | ⌘↩, beside a click on the marks slot | The marks slot (found); the faint plus on an empty slot (found) | §4, §5.1 |
| Close a row expansion | ⎋ or ⌘↩ | The chevron on the expansion's caption line; the close control at its right end | §5.1 |
| Insert a row | ⌥↩ | The footer's selection group, Insert row below; the "Add row" line that ends every editable table | §5.1 |
| Delete rows | ⌫ | The footer's selection group, Delete rows | §5.1 |
| Copy rows; paste | ⌘C; ⌘V | The footer's selection group, Copy rows and Paste | §5.1 |
| Select all rows | ⌘A | The gutter's header cell | §5.1 |
| Clear the row selection | ⎋ | The clear control on the footer's selected count; a click on any cell | §5.1 |
| Check rows; clear the check (Table view, where there is no inspector) | ⌘⇧K | The faint check in an empty state slot; the footer's selection group, Check rows; the Checked popover's buttons; the inspector's control (found) | §4, §5.1, §5.3 |
| Reveal a mark | Space on the slot; ⌥I | A click on the glyph; "All marks on this row" in the mark's popover | §4 |
| Undo, redo | ⌘Z, ⇧⌘Z; Undo in a toast and in the Accepted record | The toolbar's Undo and Redo | §5.2 |
| Send a message to the agent | ⌘↩ | The Send control at the input's right end | §5.4 |
| Close the issues drawer; collapse the table drawer | ⎋ | The drawer's close control; the collapse chevron at the right end of the table drawer's tab strip, which also reopens it | §5.3 |
| Close a dialog | Escape | Every dialog's Cancel or Close button | §5.6 |
| Close a popover, the run log, a column menu | ⎋ | A click outside it (found) | §5.2 |
| Dismiss a toast | none stated | The toast's close control | §5.6 |
| View switch; palette; Issues; sort; Accept and Reject; paste band; combination editor; Run and Stop | ⌘1 ⌘2 ⌘3; ⌘K; ⌘⇧I; ⌘↓; ⌘⇧A ⌘⇧R; ↩ ⎋; ↩ ⎋ | The segmented switch, the palette field, the three Issues entries, the header's sort indicator and menu, the card's row and card buttons, the band's and the editor's two buttons, the Run button: all found, unchanged | §5.1, §5.2, §5.4 |

### 5.1 The table

One table component serves every table in the product: the layout table, the attachment tables, load cases and load sets, every results table, hangers, and the live tables on the Review page. What follows is its full specification; the layout table is the reference instance.

**Header.** The gutter's header cell is the select-all control (⌘A accelerates it). 28 px, `surface.header`, 13 px medium, header cell padding 0 5 (P-1), units in square brackets after the name at caption size in `text.secondary` (M-11; P-1): `DX [mm]`, `T1 [°C]`, `P1 [bar]`, `Stress [MPa]`. Above the header the stage's tab strip, whose tabs carry counts in `text.muted` after the name (`Layout 16 · Restraints 7 · Node data 2`, D-6); the strip is the stage's in every view (P-7). A header cell carries the sort indicator (two small arrows, the active direction filled) and, on hover, the column menu (sort, filter, precision, unit, hide, pin). Sorting never reorders the layout table's rows on disk: the table is the model and the row order is the file order; sort is a view, indicated by a `sorted` chip in the footer, and Enter-down still follows the file order. Filtering hides rows and the footer says how many.

**Columns of the layout table, in the model batch file grammar** (RESEARCH-E §3.2, §6.1), left to right:

| Column | Content | Notes |
|---|---|---|
| gutter | origin and state slots | §4 |
| Node | the node number | numeric, auto-increment 10 on a new row, editable; insert and split preserve neighbours. The start node's row (10) is the model's origin: it shows "—" in `text.muted` under Section, Material and Load, its read-through cells are empty rather than sunken, and it carries no required mark; those columns are entered on the first element's row (20) and propagate from there (decision 7; D-1, P-6) |
| From | the node the element comes from | shown on every row; drawn in `text.muted` when it is the previous row's node (implied), in `text.primary` when it names an earlier node (a branch) or was typed |
| Type | the element arriving at the node | the engine's element kinds and no others (R-4): Pipe (blank in the file, shown as "Pipe"), Bend, Valve, Reducer, Rigid, Expansion joint. The kinds the file's grammar knows and the engine does not represent are not offered, and the enumeration's last line says "Other element kinds are recorded scope decisions · About › Scope and limitations"; a tee is never here, and flange-bearing kinds are node data. The cell carries the expansion chevron for the element's fields |
| DX DY DZ, or X Y Z | offsets from From, or absolute coordinates | one switchable column group; the group's header shows which; a row whose coordinates are absolute in the file (the `*` form) shows a corner tick |
| Section | the section name | propagates down until changed; a propagated cell carries the corner tick only, and a cell typed over loses it (D-2) |
| Material | the material name | propagates; marked |
| Load | the load set name | propagates; marked |
| T1 … Tn, P1 … Pn | the set's temperatures and pressures read through | `surface.sunken` cells with a corner tick, empty on the start node's row (P-6); typing over one edits the set or forks a new one, offered as a choice in a small popover |
| marks | restraint, load, node data | §4 |

Element fields beyond these (bend radius, valve weight, reducer ends, expansion joint stiffnesses) live in the row's expansion (the Type cell's expansion chevron; ⌘↩ accelerates it) and in the inspector, not as columns. The read-through group is on by default in Table view and off in Both view and in the Model-view drawer, where the Load cell's tooltip shows the values; the footer's Read-through chip shows and switches the state (D-5). In Both view with the group off, Type narrows to 96 so the table fits its 737 px pane without a scroll (s4_both).

**Cell states.**

| State | Appearance |
|---|---|
| Entered | `text.primary` on the row surface; nothing else |
| Propagated | as entered, with the origin corner tick in `mark.origin`; tooltip names the source node |
| Read-through | on `surface.sunken` with the corner tick; the value belongs to the named load set |
| Proposed | the new value in `proposal.new` medium, preceded by the struck old value in `proposal.old` when the column has room for both (otherwise the old value is in the tooltip and on the card); the row carries the band and bar |
| Selected | `selection.band` across the row with the 3 px `selection.bar`; a selected cell inside a selected row adds a 1 px `selection.bar` outline |
| Editing | the cell becomes an input on `surface.panel` with the 2 px inset `focus.ring`; the text is selected on entry; the unit shows at caption size in `text.muted` at the cell's right edge, so a 72 px column holds `−1500 mm` with the caret (s3) |
| Invalid | a 2 px inset ring in `issue.blocking` and the message in a popover under the cell; the value stays as typed until corrected or escaped |
| Required, empty | the asterisk in `mark.required`; the gutter's state slot shows the octagon |
| Stale | a `stale.stripe` diagonal hatch over the cell (a result cell whose model has changed, a Checked row's content) |
| Historical | the results header band; cells unchanged |
| Disabled | `text.disabled`; never used in the layout table |
| Draft | the row the engineer is routing (§5.6) or a paste-preview row (the paste band): a dashed 3 px `draft.bar` at the gutter's left edge, text in `text.muted`, the typed value in `accent.text`, propagated values with their ticks, Type shown as "Pipe" until committed (s2, s3; G-3) |

**Row gutter and marks column.** As in §4. The gutter also carries the row selector: a click beside the glyphs selects the row, ⇧-click extends, ⌘-click toggles; a click on a glyph opens that mark's popover, and an empty state slot offers the faint check on hover (§4). Every editable table ends with an "Add row" line in `text.muted`, one row high, which a click turns into a new row with the next node number (⌥↩ accelerates the insert below the focused row). The marks column's empty slots show a faint plus on hover ("Add restraint…"), which opens the joined row empty.

**The paste band.** When a multi-cell paste arrives (⌘V with more than one row or a header row), a band appears above the affected rows on `surface.raised` at elevation 1: a mapping row in which each source column names its target column in a combobox (recognised names pre-mapped: From, To, Node, DX, DY, DZ, Section, Material, Load, T1…, P1…), unmapped columns offered "Ignore" or a target, a count ("42 rows, 2 columns ignored"), a preview of the first three rows in the target grammar drawn as draft rows, showing the propagated value with its tick where the source left Section, Material or Load blank, an ignored column keeping its source name beside "Ignore" (s3), and two buttons: Paste *3 rows* (↩) and Cancel (⎋). The paste commits as one operation with one undo checkpoint; the rows it creates carry the entered origin, and node IDs are taken from the source when present and auto-incremented when not.

**Keyboard model.** The spreadsheet idiom, fully operable without the mouse (brief §2, C-96) and, by the pointer rule of §5, fully operable without the keyboard except for typing a value. Every key below accelerates a control; the last column names it.

| Key | Not editing | Editing | The control it accelerates |
|---|---|---|---|
| Type a character | starts editing, replacing the value | inserts | the cell: a click on the focused cell, or a double-click on any cell, starts editing |
| ↩ | starts editing the focused cell | commits and moves down one row (⇧↩ up) | the cell; the footer's edit chip, Commit; a click on another cell commits |
| ⇥ / ⇧⇥ | moves right / left; wraps to the next row's first editable cell | commits and moves right / left | a click on the target cell |
| ← → ↑ ↓ | moves the focus one cell | ← → move the caret; ↑ ↓ commit and move | a click on the target cell |
| ⎋ | clears the multi-row selection, then closes the open drawer or popover: one level per press | cancels the edit and restores the value | the footer's edit chip, Cancel; the clear control on the selected count; the drawer's or expansion's close control; a click outside a popover |
| ⌘↩ | opens the joined row for the focused row (on a marks slot, that mark's row; on the Type cell, the element fields) | commits, then opens | the marks slot; the cell's expansion chevron |
| ⌥↩ | inserts a row below with the next node number | commits, then inserts | the footer's selection group, Insert row below; the "Add row" line |
| ⇧↑ ⇧↓, ⇧-click | extends the row selection | — | the gutter, with ⇧ held |
| ⌘A | selects all rows | selects the text | the gutter's header cell |
| ⌘C ⌘V | copies the selection as tab-separated text with headers; pastes with the band when multi-row | text | the footer's selection group, Copy rows and Paste |
| ⌫ | on a row selection: deletes the rows, with an undoable toast | deletes text | the footer's selection group, Delete rows |
| Space | on a gutter or marks slot: opens the mark's popover; on a check-box cell: toggles | inserts | a click on the glyph or the box |
| ⌥I | opens the row's marks popover | — | "All marks on this row" in any mark's popover |
| ⌘K | the command palette | — | the toolbar's palette field |
| ⌘Z ⇧⌘Z | undo, redo: one stack for the whole product | text | the toolbar's Undo and Redo |
| ⌘⇧A ⌘⇧R | Accept, Reject the focused proposed row | — | the card's row buttons (§5.4) |
| ⌘⇧K | marks the selected rows Checked (again ⌘⇧K clears) | — | the state slot's faint check; the footer's selection group, Check rows; the inspector's Check control |

**Footer.** 26 px on `surface.header`, the counts line (D-5): rows and the selected count ("16 rows · 1 selected") with a clear control on the count; while rows are selected, the selection group follows it, five compact icon buttons with tooltips: Insert row below, Delete rows, Copy rows, Paste, Check rows (§3.2); while a cell is being edited the same place holds the edit chip instead ("Editing DX · node 70" with Commit and Cancel). The selection group and the edit chip take the place of the counts that follow, which return when the selection clears, so the footer stays one line in the 737 px pane. The row actions live in the footer because the strip above the table belongs to the stage's tabs and never grows a toolbar (§5.3). Then the counts of what the table holds, each a filter when clicked: elements, bends and other fittings, restraints, loads, node data, propagated cells, proposed rows, checked and stale, and the issues on this table (the count in the worst class's colour); then the active filter chips with a clear control; then the Changed since… control (last run, last Checked, a named snapshot, a time); at the right the Read-through and Origins switches as outline chips. Sorted, filtered, envelope and expansion states show as chips here ("Sorted by ratio", "Envelope: governing case per element", "2 joined rows open"); a results footer carries the governing line ("0.72 governing · EXP1 · node 70 · EXP-A1 · pack 1.2"), a proposal's standing ("2 proposed rows · P-12 · draft until accepted") and a hanger table's "1 not yet designed".

**Row expansion (Table view).** A click on a marks slot, or on the expansion chevron of a cell that owns an expansion (⌘↩ accelerates both), expands a block under the row, indented by the gutter width, with a 2 px `border.strong` left edge and a caption line with the expanded chevron, which closes the block on a click, and a close control at its right end ("Restraints · node 20 · joined row · ⎋ closes"). The expansion chevron is the 12 px chevron of §3.2 at the right edge of the owning cell (Type, Expression, a hanger row's Size), drawn on row hover, on selection and whenever the block is open. Four things open this way, all with the same anatomy (s4_table, decision 14; ruling 5): a **joined attachment row**, the attachment table's own header row and the node's rows in that table at that table's column widths, editable in place with the same keyboard model, and an "Add restraint…" row; the **element fields** of the Type cell (bend radius, valve weight, reducer ends, expansion joint stiffnesses); the **hanger selection** under a hanger row (below, under Results tables); and the **combination editor** under a load case (its Expression cell's chevron; ⌘↩ or Space accelerate it): the case's terms as chips (`W + P1 + SE1 (0.3 g X)`), the available terms dimmed after them, the Stress type and Rule comboboxes with the pack named, and Cancel ⎋ and Done ↩; it is a row expansion and not a popover so it never covers the rows below and takes the joined-row keys. One expansion opens per click and stays until its chevron or close control is clicked (⎋ or ⌘↩ accelerate); the footer counts the open ones. In Model and Both views the same controls open the row in the docked inspector instead.

**The Loads stage's header band.** Above the Cases tab strip: the pack identity ("Cases · generated from rule pack sample-rules 1.2 · sha256:9b1c4e02… on 2026-09-17 10:05"), the button "Generate from rule pack…", and at the right, once, the M-15 caption "Rule expression: Display only, not accepted as input" with the display-only glyph; the Rule expression column header says only "Rule expression", because the dashed frame is the form (s5). The Origin column reads "Generated · sample-rules 1.2", "Edited by *name* · was T1 (generated)" or "Authored by *name*", with the generated glyph in the gutter on generated rows only.

**Results tables.** The header band above the columns carries the table's name and the run with its immutability ("Stresses · Run 03 · solved 15:21 · immutable"), the case selector (a combobox of the run's cases, "Case: EXP1") with the Envelope toggle (a switch labelled Envelope; when on, the Case column shows the governing case per row and the selector stays visible, reading "Case: all" in `text.disabled`, so the control never moves and the engineer sees why it does not answer, decision 8), the evidence chip when, and only when, the run record carries evidence (§2.3), and the run identity control. The control's disclosure opens under the band on `surface.raised` and holds one thing, in `type.family.mono`: the run identity line, which is run, date and time, model state hash, solver version, rule pack name, version and hash, settings name ("Run 03 · 2026-09-17 15:21 · sha256:4df0f798… · solver 0.2.0 · rule pack sample-rules 1.2 · sha256:9b1c4e02… · settings S-03"): provenance one click from the table, and the full record on the run record (decision 9). The acceptance sentence that V1.1 placed above that line is removed from the product (ruling 3): the disclosure holds no sentence, no heading and no space for one, and the control is named for what is left, "Run identity".

**Run standing (ruling 6; R-9).** A results surface shows a run in one of three standings, and what the run may drive depends on it. The rule is one: result colour, deformation and probed values are an overlay on the current model, and only a Current run may drive an overlay, a status chip or any other cue that the current model has results. A Historical record never takes either; a run the model has moved on from is kept on the same terms, with its own band.

| | Current | Stale: the model changed since the run | Historical: a reopened saved run, or an earlier run chosen from the run list |
|---|---|---|---|
| Header line | table, run, time, "immutable"; the case selector and Envelope; the evidence chip when the record carries evidence; the run identity control | the same line, without the evidence chip | the same line, without the evidence chip |
| Band under the header | none | the stale band on `stale.band` / `stale.ink` with the stale glyph, the sentence below and "Run again" | the historical band on `historical.tint` / `historical.ink` with the hatch and the clock: "Historical saved run · Run a fresh solve to establish current results."; its information control opens a popover that carries "Historical results cannot drive current overlays, rule checks, comparisons or report readiness." and, under it, the run's recorded statuses as labels with their domains and tokens (§2.3) |
| Result cells | plain | hatched in `stale.stripe`, readable and copyable | plain under the band; readable and copyable |
| Status bar | the chips of §2.3 | no chip | no chip from the run |
| Rail's Results caption | none | Stale | Historical |
| Legend (§5.6) | the scale | a note card with no scale: "Model changed since Run 03" and "No result colour on the current model" | a note card with no scale: "Historical saved run · Run 02" and "No result colour on the current model" |
| Canvas | result colour on the tube, deformation, the probe | the neutral drawing: no result colour, no deformed shape; the HUD's Deformation and Probe tools disabled with the reason in their tooltips ("Needs a current run") | the same neutral drawing and the same two tools disabled |
| Review page and report | live blocks; the Review rail item is enabled by a Current solved run | live blocks carry the stale band | live blocks carry the historical band; a Historical run enables nothing and lights no cue that a report is ready |

Two bands can therefore sit under a header, never both at once: the historical band (M-13; ruling 6), and the stale band after a model change (decision 4; G-1): "Model changed since Run 03: proposal P-12 row 1 accepted at 16:31 (variable spring at node 80). The values below are from the model as solved." with "Run again" at its right; the compact form, where the band has one line, is "Model changed since Run 03 · proposal P-12 row 1 accepted at 16:31 · Run again". After a model change the run stops being the current solve basis (settled by ROOT, §8 item 17: the product clears the solve proof and the model hash when a model commit lands), and the band says nothing to the contrary and the result cells hatched in `stale.stripe`. The stress table's columns are Node, Element (From–To), Case, Stress [MPa], Allowable [MPa], Ratio (number with its data bar on `bar.track` and the 1.0 tick), Rule, Pack; sorted by ratio descending by default, with "Sorted by ratio" as a footer chip. The Ratio header's menu: Sort descending (⌘↓), Sort ascending, Data bar · 1.0 tick, Colour by scale, Filter ratio ≥ …, Unchecked rows only, and the note "Rule and pack are their own columns · width 120 · reset" (s7_table). The hanger table carries, under its header, a caption naming the library ("Vendor-A springs · user import 2026-09-15 · 24 sizes · source recorded in Libraries · max variation 25 %"), and a spring location accepted since the run shows "—" in every design column with the footer count "1 not yet designed" (s8_table). Hanger tables are a library class the user imports and the product never bundles (ruling 5): until a library is imported the table has no sizes to offer and says so in the caption's place ("No hanger library imported · Import through Libraries"). **Hanger selection** is the row's expansion (the Size cell's chevron; ⌘↩ accelerates it): a caption line ("Hanger selection · node 80 · Vendor-A springs · user import 2026-09-15"); under it, once, in `text.secondary`, the content boundary's short variant, verbatim: "no protected standards content; code-specific data is user-supplied"; then the library's candidate sizes as rows at the hanger table's widths (Size, Rate, Cold load, Hot load, Variation, working range), sorted by variation, the ones outside the row's Max variation or working range in `text.disabled` with the reason, each eligible row ending in a "Select" text button; selecting writes the size onto the Restraints row as one operation and closes the block. Every results table can be copied and exported (C-98).

**Column widths for the other tables (P-4).** Defaults, resizable, taken from the frames' `tools/ui.mjs`, where they were set to fit their headers and values:

| Table | Widths |
|---|---|
| Restraints | Node 56, Tag 56, Type 118 (176 while a proposal shows old and new side by side), Direction 76, Gap 80, Friction μ 80, Stiffness 124, Connecting node 126, Library 148, Max variation 128, Note 200 |
| Restraints, as a joined row | Type 120, Direction 80, Gap 80, Friction μ 80, Stiffness 124, Connecting node 126, Tag 72 |
| Loads, as a joined row | Kind 120, Direction 80, Value 88, Unit 56, Case 64, Note 200 |
| Cases | Case 72, Expression 140, Stress type 96, Rule 72, Origin 256, Rule expression 260 |
| Stresses | Node 56, Element 80, Case 64, Stress 96, Allowable 116, Ratio 120, Rule 72, Pack 48 |
| Hangers | Node 56, Tag 56, Type 108, Design load 116, Travel 90, Library 124, Size 64, Rate 98, Cold load 102, Hot load 94, Variation 100 |
| Review live block, Restraints | Node 56, Tag 56, Type 112, Gap 80, Friction μ 80, Library 124, Max variation 128 |

When a pane is narrower than a table, the secondary columns hide first (Restraints: Connecting node, then Note) so the columns that carry a proposal's old and new values keep their width (s8_model).

### 5.2 The stage rail, the view switch, the toolbar band, the status bar

**Stage rail.** 56 px, `surface.base`, items 44 px tall with a 20 px icon over an 11 px label: Model, Loads, Results, Review; a hairline; Libraries, Rules, Issues at the foot. The selected item fills its glyph and draws a 3 px `selection.bar` on the rail's left edge; items that cannot yet be used are `text.disabled` with a tooltip that says why: Results and Review "No run yet" before the first run, Review "No solved run" after a stopped one (D-3). Counts sit as small badges at the item's top right: Issues carries the issue count; a proposals count sits on the agent strip, not on the rail. A stage item carries a caption at `type.size.caption` under its label when the stage has a state to name (D-3; G-2): Results reads Failed in `rail.captionFailed` after a stopped run, Stale in `rail.captionStale` after a model change (decision 4), Historical in `rail.captionHistorical` when a saved run is reopened (M-13); the caption is a state name, not a status, and carries no glyph. The rail is navigation, never modes: switching stages keeps the selection, the undo stack and the issues list.

**View switch.** A three-segment control in the toolbar: Table, Model, Both, icons with labels, 26 px tall, the active segment on `surface.panel` with `text.primary`, the others `text.secondary`; ⌘1 ⌘2 ⌘3. The choice is remembered per stage.

**Toolbar band.** 48 px on `surface.base` with a hairline below. Left to right: the wordmark (SWBPIPE in 13 px medium, tracking 0.04 em, `text.secondary`; the window title and About carry the same name and no other, §7.5), the project name with its save state ("Loop 4 header · saved", or "· edited" in `text.secondary`), Undo and Redo as two icon buttons (the one undo stack of the product; disabled with "Nothing to undo"; the tooltip names the operation, "Undo: accept P-12 row 1 (⌘Z)"), the view switch, then centred the Run button (the surface's primary action, `accent.fill` with `text.inverse`; while running it shows a progress bar inside the button with the stage name in its tooltip and a stop control; when disabled it stays visible on `disabled.fill` and its tooltip names the blockers of the solve, and only those: "Run is unavailable — Section missing at node 20, Material missing at node 20"; a missing load set is a Blocks rule check issue and is left to the Issues count, s1), the Issues count as a button with the octagon or triangle of the worst class, then the two toggles of the right-hand panels in the order the panels sit on the screen: the Inspector toggle (⌘I), which opens and closes the docked inspector in Both view and stays latched while it is open (disabled in Table view, "Table view opens rows in place", and in Model view, "The inspector is always docked in Model view"), and the Agent toggle (⌘⇧G) that opens the column and stays latched on `pressed.fill` / `pressed.ink` while it is open (G-5); below 1360 px of window width the two toggles drop their labels and keep their tooltips, so the band holds at the 1280 px minimum; the display units selector (a compact combobox "SI" / "US"; a display toggle only, C-40), and the palette search field (⌘K, 220 px, placeholder "Search or command…").

**The run log popover (decision 5; Q-16; G-12).** Clicking the Run button while it runs, or after it stops, opens a popover under the toolbar band, `surface.raised` at elevation 2, 360 px wide (`layout.runlog.width`). Its title line is one line at `type.size.body` weight 500 in `text.primary`: the run's name, the time span in `type.family.numeric`, and the settings name, separated by middle dots ("Run 02 · 15:02:14–15:02:41 · settings S-02"); while the run is going the span's end is the running clock. Then at most four rows at 22 px, each a 12 px glyph slot, a name in `text.primary` at weight 500 and a time or a place in `text.secondary`. The three row glyphs are named from the marks vocabulary (§4), so that a frame never has to invent one: a step that completed carries the **Entered** glyph, the dot in `mark.origin`, a neutral mark that something happened and not a check, because the check is the Checked mark's and belongs to a person; the step that stopped the run carries the **Issue, warning** glyph, the triangle in `issue.warning`, the one loud thing in the popover; a step that was not run, or is running now, carries no glyph and its slot stays empty so the names align. The rows: assembly; the solved cases as one row ("W, SUS solved · 0.8 s"); the stopped or running case ("OPE1 stopped at iteration 50 · node 20", with "Show node 20"); and the items not run as one row ("OPE2, EXP1, EXP2, OCC1, hangers, rules not run"); then a footer with Run settings…, Run record and Run again. Placement (Q-16, a default): the popover hangs to the right of the Run button, its left edge at the button's left edge or at the canvas pane's left edge, whichever is further right, so that in Both view it lies over the canvas's top edge and the table's tab strip and header rows, where the engineer is working, stay uncovered; in Table view it hangs from the button's left edge. It covers the HUD's first tools while it is open, and it closes on Escape or on the first click outside it, which is ordinary popover behaviour. It is shorter than a log; the full log, every case and every iteration, lives on the run record (s6).

**Status bar.** 24 px on `surface.base` with a hairline above, 12 px medium: at the left the M-08 status chips with their popovers, none, one or two by the policy of §2.3; then the issues count, which opens the drawer; then the selection ("Node 40 · 1 row", "Element 60–70 · node 70", "Node 80 · 1 row · 2 proposed rows"); at the right the display units and the About control, an information glyph with the tooltip "About SWBPIPE…" that opens About directly. V1.1 hung a popover from this control whose two lines were the About link and the maturity sentence (M-01); the owner has since removed that sentence from the product entirely (ruling 2, as superseded), a popover holding one link is not worth a click, and so the popover goes and the control opens About. Nothing else lives in the status bar: no hashes, no seams, no proofs, and nothing about a stale run, which the rail caption and the results header carry (decision 4).

### 5.3 The inspector, the issues drawer, the table drawer

**Inspector.** A docked column on `surface.panel`: 340 px in Model view, and 300 px in Both view, where it docks on the canvas's right edge and the canvas shrinks while it is open (§0; decision 1); when the canvas would fall under its 220 px minimum, the agent column collapses to its strip first, and only if that is not enough does the inspector open as a slide-over for that window size, with the tooltip "Docked inspector needs a wider window" (UX_SPEC_V1 §10.9); and when the agent column is opened while the inspector is docked, the inspector becomes that slide-over for as long as the column is open at that width and re-docks when it closes (Q-15; §0). In Both view the inspector is opened by the toolbar's Inspector toggle and closed by the same toggle or by the close control at the right end of its header line; ⌘I, a double-click on a row's gutter or on a node, and Escape accelerate them (§5). In Model view it is always docked and has no close control. Rows are inline label-value pairs at 26 px with the label in `text.secondary` at a fixed 88 px (P-2: in 300 px a 120 px label left 156 px for a value like "R. Tufts · 2026-09-17 14:02") and the value right-aligned in `type.family.numeric` with its unit; editing in place. While the engineer is routing from the node, a routing block sits on top ("Routing from node 40": next node, axis, length in `accent.text`, what the row will carry, and three compact buttons, "Place node 50" (↩), "Bend at 40…" (B) and "Cancel" (⎋), each with its accelerator in its tooltip), and it goes when the route commits or cancels (s2). Sections in order: identity (node, element From–To, type); geometry (DX DY DZ and X Y Z both, the row's coordinates); section and material (with the library record name and a link to Libraries); load set (name and every T and P read through, with "Edit set" and "Fork set"); attachments (restraints, loads, node data as compact sub-lists, each row opening the joined row; "Add…" at the end of each; while a proposal touches an attachment, a proposed line under its row in `proposal.new`: "◆ Proposed · P-12 · Variable spring · Vendor-A · 25 %", s8_model); issues on this entity (class in words, message, Show); origin and Checked (who, when; the Check control, or Clear check ⌘⇧K when checked; multi-row when several rows are selected); provenance behind a disclosure (source, location, licence, contributor, redistribution status, review status: the seven fields of C-46, read from the library record). Required fields carry the asterisk. Nothing in the inspector is a second way to mutate the model: every edit is the same row operation.

**Issues drawer.** A bottom drawer of 200 px (resizable, remembered) under the tables, opened from the status bar, the rail, the Run button's reason or ⌘⇧I. Grouped by class in the order Invalid model, Blocks solve, Blocks rule check, Provenance, Assumption, Nonlinear, Content boundary, Note, each group with its count and a filter chip; a row is the severity glyph, the class in words, the message and the entity ("node 20"), and is a button that selects the entity in the tables and the canvas. The row's overflow (G-11): its height is fixed at the table's 26 px and it never wraps; the glyph, the class words, the entity and, on the selected row, the link ("Show node 20") never truncate and keep their natural widths, the entity and the link right-aligned; the message takes what is left and truncates with an ellipsis, and its full text is the row's tooltip, on hover after 300 ms and on focus. The drawer's header carries its close control at the right end (⎋ accelerates it). A failed run adds one banner, across the top of the Results page on `issue.warningTint` with the triangle: the failure reason in one sentence with "Show node 20" (C-58, C-59: the reason is the solver's diagnostic, never inferred); the drawer carries no second banner, because in Both view both would be visible at once; instead it opens filtered to the failing class, with a filter chip that clears ("Nonlinear 1 ×", "All classes 2"), and its selected row carries the same link (decision 6; P-3; s6). The Results page under the banner shows the empty state in `text.secondary`: "No results — Run 02 stopped in OPE1 (case 3 of 6) at the iteration limit. A run's results are one immutable set, so a stopped run has none. Change the support at node 20 or the iteration limit in Run settings, then Run again." The drawer never becomes a modal gate; Run is a button that says why it is disabled.

**Table drawer (Model view).** A bottom drawer under the canvas of 280 px (resizable, remembered) holding the stage's current table with the stage's tab strip on its top edge, the same strip as the Table view's (P-7; Model: Layout, Restraints, Node data; Loads: Cases, Load sets, Loads, Wind, Seismic; Results: Summary, Stresses, Displacements, Restraint loads, Restraint summary, Forces and moments, Hangers). It is the same table component with the read-through group off, and in its 1000 px it hides a table's secondary columns first (§5.1). The canvas keeps its width; the drawer's tab strip never grows a toolbar of its own. Its one control is the collapse chevron at the strip's right end, which collapses the drawer to its 28 px tab strip and opens it again (⎋ accelerates the collapse); the table's own controls are in its footer (§5.1).

### 5.4 The agent panel

A right column of 340 px on `surface.panel` with a hairline on its left, present in every stage, collapsible to a 44 px strip by the toolbar's Agent toggle or the collapse control at the right end of the column's header (⌘⇧G accelerates them). The strip shows the agent glyph, the count of open proposals as a pill badge, and a dot when the agent is working; the whole strip is a button that opens the column. Opening the column reflows the surfaces; it never overlays the tables or the canvas. The engineer is never demoted: the column has no controls that act on the model except Accept and Reject, and everything the agent proposes is read in the tables and the canvas.

Header: "Agent" (15/600), the working state ("Idle", "Idle · P-12 open", "Working…" with a stop control), and a tab row: Conversation, Proposals (count), Checks, Accepted (count).

**Conversation.** Messages at 13/18 with the author in `text.secondary` and the time; the engineer's input at the bottom as a growing field with the Send control, a 22 px icon button, at its right end (⌘↩ accelerates it; disabled while the field is empty). The agent's messages may contain references (rows, results, report text) rendered as links that select the referent. TBD items and conflicts appear as a list under the message in `text.secondary` with the octagon or triangle of their class, never as filled-in values (C-78).

**Proposal queue.** One diff card per proposal, `surface.raised` at elevation 1, `radius.card`, 12 px padding, a 3 px `proposal.bar` on its left edge:

- Title (15/600): what and where, in the engineer's grammar: "Replace the rigid support at node 80 with a variable spring".
- Meta line (12): "Proposal P-12 · draft until accepted · 2 rows in Restraints, 1 row in Layout · 16:22". The words "draft until accepted" are the M-14 standing and always appear. The engineer's question that led to the proposal is quoted above the meta line in `text.secondary` when there is one (s8_table).
- Diff rows: one per affected row, each a compact table of field, old value and new value with units, old struck in `proposal.old`, new in `proposal.new`; a row whose values are unknown shows `TBD` in `text.secondary` where the value would be. Each row has Accept and Reject at its right end as 22 px icon buttons (check and cross; tooltips "Accept row ⌘⇧A" and "Reject row ⌘⇧R"), because a 316 px card has no room for text buttons beside the values; the card's only accent is its Accept-all button. A row that has been decided collapses to one line, a receipt (Q-18, a default): the done check; the first changed field with its new value ("Type Variable spring") and, when the row changes more fields, their count ("+4"); and at the right "accepted 16:31 · Undo" or "rejected 16:31". The line never wraps; the new value is the first thing to truncate, with an ellipsis. The old value is not on the line: it is in the line's tooltip ("Type: Rigid (Y) → Variable spring") and in the diff, which a click on the line's chevron opens again. The row's place in the proposal ("row 1 of 2") stays on the row's title line above the receipt, as it does for a pending row. The pending rows keep their diff (s8_table).
- Rationale: a paragraph in `text.primary`.
- Constraints considered: a bulleted list.
- TBD items: a list with the triangle glyph ("Vendor table to use: TBD").
- Validation: one line, "Schema and constraints: passed" or the failures, in `text.secondary`; a proposal that fails validation cannot be accepted and says so.
- Consequence line, in `text.secondary`, above the actions: "Accepting a row changes the model. Results from Run 03 stay readable, marked stale, until the next run." V1.1's sentence said the results are cleared, which decision 4 and the stale band contradict (MOCKS_V2 D2-1); this one says what the engineer will see (§5.1, run standing).
- Card actions: "Accept all rows" (accent) and "Reject proposal" (text); once a row has been decided they read "Accept remaining (n rows)" and "Reject remaining" (s8_table).

Queue actions above the cards: "Accept all (n rows)" and "Reject all", each confirming with the count. Accepting a row applies the same structured operation a hand edit would (C-74, C-75), records the rationale, constraints and TBD items on the accepted operation (M-14), marks the row's origin as accepted, and makes the run Stale: its values stay readable under the stale band and it drives no overlay and no chip until the next run (§5.1). The proposal's rows in the tables carry the band until every row is accepted or rejected.

**Card classes (ruling 8).** Every card the agent authors carries exactly one of five class words as its label, and no other word serves as one. The words are in `tokens.json` (`agentCardClasses`).

| Class word | What the card is | Where it appears | Its actions |
|---|---|---|---|
| Proposal | a proposed edit as a diff (above) | the Proposals tab; the word opens the card's meta line | Accept, Reject, per row and per card |
| Check | a check of the engineer's work against a referent | the Checks tab; the Review page's comment stream | Resolve, Reopen |
| Open issue | something the agent could not settle, by reference | the Checks tab; the comment stream | Resolve, Reopen |
| Evidence summary | the agent's summary of what the record shows about a referent: a run, result rows, a library record. V1.1 called the agent-authored card of this kind a "Note"; an agent card never reads Note now, and "Note" remains the word for the engineer's own comment | the Checks tab; the comment stream | Resolve, Reopen |
| Draft | text the agent drafts for a report section | the comment stream, against the section | Insert, Discard |

The label is a kind chip (outline) at the head of the card, or for a proposal the first word of its meta line. The words accepted, approved, verified, correct and compliant never appear in agent text: titles, rationales, constraints, TBD items, checks, summaries and drafts (§7.1); the runtime guard applies to comment and draft text as it does to rationale. The product's own strings around the agent's text ("draft until accepted", "accepted 16:31 · Undo") are not agent text.

**Checks.** The agent's checks, open issues and evidence summaries on the engineer's work, by reference: each is a card with its class chip, the referent as a link ("Restraints row, node 30"; "Stresses OPE1, node 40"; "Report §3, paragraph 2"), the text, and a state the engineer sets (Open, Resolved). They never alter a table and never use the words the boundary forbids (§7.2). The same stream appears in the Review page's third column.

**Accepted.** The record: one row per accepted operation with time, the proposal, the rows, the rationale disclosure, and Undo where it is still reversible (C-99). The last accepted operations also appear as one line under the open card in `text.secondary` ("Accepted: P-09 · 11:40 · P-12 row 1 · 16:31"), so the queue and the record meet (s8_table). Reopened records from an earlier session show acceptance as unknown (C-68) with the words "Acceptance not recorded in this session".

### 5.5 The Review page primitives

A full page in Table view, three columns on `surface.panel`, 16 px gutters, at 1440 with the strip: outline 280, content flexible, comments 320.

**Header.** The page title (18/600) "Review · Loop 4 header · Run 04"; the iteration control, a combobox of named snapshots ("Iteration 2 · 09:40 · Run 04") whose menu lists the iterations and, after a separator, "Compare with…" (Q-17, a default): choosing it starts a comparison, and only then does the compared-with control appear beside the iteration, itself a combobox ("compared with Iteration 1 · 2026-09-17 15:40 · Run 03") whose menu changes the compared iteration or clears the comparison, with the "Show edits" switch beside it; with nothing compared neither is there, so the header carries no fourth button. At the right, three buttons of one weight, each with its icon (G-7): "Snapshot…", which creates a named iteration, "Report preview" and "Export…" (decision 10; s9): the report is produced from this page and nowhere else, so its two productions sit on its header. Export… is a plain button like Report preview (Q-19, a default): a window has one accent, and it is Run; producing the report is not this page's primary act, writing it is. Under the header's hairline the three columns begin at once: the acceptance sentence that V1.1 placed there is removed from the product (ruling 3), and the space it held is closed.

**Outline column.** The report sections in the required order (M-07), each a row with a drag handle for the sections the engineer may reorder and a lock glyph for the fixed items: Notice (M-03 and, when the report includes export metadata, hanger selection records or handoff data, the supplement M-04); Identity (software, solver, build, date, units, coordinate system, model state, run); Libraries and rule packs (name, version, checksum, source note); Model; Load cases; Assumptions and warnings; Results (one entry per results table selected); Comparison (when selected; absent in this design); Hanger selection; Review/signoff block, which is the registered name of the required report content item (RESEARCH-C `C_ui_constraints.md` §4 row M-07, from `docs/PRD.md` §19.2) capitalised as a heading (P-10): the section heading is the one place the word appears in the product. Each row shows a state word in `text.muted` (empty, drafted, edited, live, current) or, for the fixed sections, the lock glyph; under the outline the note "fixed sections come from the record" (s9). Clicking scrolls the content column.

**Content column.** Section blocks in outline order. A live table block is the table component in read mode with a caption and a lock glyph ("Live from Run 03 · Stresses · OPE1"; "Live from the model record · Restraints · nodes 20, 60, 80 · not editable here"): it pulls from Results or the model record and cannot be edited here; its case selector and sort are the block's own; a referenced row carries the comment glyph in its state slot (§4). A text block is the engineer's writing at 13/18 with a simple toolbar (paragraph, list, reference); references to rows and results are links. With Show edits on, text inserted since the compared iteration is underlined in `accent.text` and text removed is struck in `proposal.old`, and the first live block carries the inserted/removed legend (s9); with it off, the current text reads clean. The Review/signoff block is a form the engineer fills (name, role, date, notes); it is the report's registered review/signoff block (M-07) and the product neither fills nor labels it as accepted.

**Comment stream.** The agent's checks, open issues, evidence summaries and drafts (§5.4, card classes), plus the engineer's own notes, in one stream ordered by referent then time; a header "Comments 4" with "+ Comment"; and one filter row that holds in the 320 px column (Q-22, a default): four chips with counts for state and ownership, All, Open, Resolved, Mine, and one Kind menu for the agent's kinds, Checks, Open issues, Drafts, Evidence summaries, each with its count in the menu; when a kind is chosen the menu's button reads the kind and its count ("Kind: Checks 1") and a kind with no cards takes no space anywhere. State and kind are two filters and combine. Each card carries its kind chip (Check, Open issue, Evidence summary or Draft for the agent's cards, by ruling 8; Note for the engineer's own), the referent link, the author and time, the text, and one action pair: Resolve or Reopen, or for a Draft, Insert or Discard (s9). Comments attach by reference to rows, results and report text and never alter the tables; a comment on a table row shows a small comment glyph in that row's state slot on the Review page only.

**Report production.** "Report preview" on the page header opens the report preview dialog (§5.6); "Export…" beside it opens export and handoff (decision 10). The report is produced from this page and nowhere else.

### 5.6 The HUD, the probe card, the legend, the routing compass, the toast, dialogs

**The HUD (G-10).** One group of ten tools at the canvas's top-left, inset 8 px (`layout.hud.inset`), on `surface.raised` at elevation 1 with `radius.card`, 2 px padding and 2 px between 28 px buttons (`layout.hud.button`), in this order: Fit, View, Section, Isolate, Hide, Labels, Deformation, Probe, Route, Add restraint. In a canvas of 400 px or more (`layout.hud.wrapBelow`) it is one row, 304 px wide (`layout.hud.width`). In a narrower canvas, from 399 px down to the 220 px minimum, it wraps to two rows of five, 154 px wide (`layout.hud.widthWrapped`) and 64 px tall, the first row Fit to Hide and the second Labels to Add restraint, so that Fit is the first tool at every canvas width; no tool is dropped and none moves into a menu. While anything is hidden, the count follows the last tool as a text button ("3 hidden · Show all"), on a line of its own in the wrapped form. A tool that cannot act is disabled with its reason in the tooltip (Deformation and Probe without a Current run, §5.1). The narrow canvas stacks its furniture at the left edge, each 4 px under the last: the HUD; the hint strip while a tool is active, as wide as the canvas less the insets and wrapping to two lines at most; the legend's title row when there is a legend. Along the bottom edge the triad stays at the left and the scale reference at the right; the probe, when pinned, sits above them at the canvas's width less the insets. In the mocks only, the line that says a frame is a mock rendering sits under the last item of that stack, left-aligned at the inset, in a canvas under 400 px, and centred on the bottom edge in a wider one (D-9); it is not a product element.

**Probe card.** 300 px, `surface.raised`, elevation 2, `radius.card`, follows the cursor at 12 px offset; pinned with a click (P toggles the tool), and when pinned it sits 18 px from the node and is kept clear of the legend (s7_both); in a canvas narrower than 400 px it pins to the canvas's bottom edge (§0). Rows at 22 px: Node, Element (From–To), Case, Stress [MPa], Allowable [MPa], Ratio with its data bar, Rule, Pack; a footer at 12 px with the run name and the evidence label chip. A pinned card carries a close control at its top right (Escape and a second click on the node accelerate it). The card shows what the table shows for that row and nothing more, and the Probe tool works for a Current run only (§5.1, run standing).

**Legend.** 220 px, `surface.raised`, elevation 2, in the canvas's top-right under the HUD: the quantity and unit with the case beside it ("Ratio · expansion · EXP1", "Stress [MPa] · OPE1", or Envelope), the scale bar with five ticks and the marker of the selected or probed value ("0.72 · node 70"), the rule and pack for a ratio, the range control, the Unsolved swatch, and for deformation the factor ("Deformed ×50", a stepper, the ghost swatch "Undeformed") with the play control. Everything in the legend is a label or a number in text tokens; the bar is the only coloured thing. For a Stale or a Historical run the legend is a note card of the same width with no scale, no range and no marker (§5.1, run standing): a title line in `text.primary` at weight 500 ("Model changed since Run 03", or "Historical saved run · Run 02") and one line in `text.secondary`, "No result colour on the current model". It carries no control; Run is in the toolbar. In a canvas narrower than 400 px the legend collapses to its title row and expands on hover or click, and the HUD wraps to two rows (§0).

**Routing compass and direct distance entry.** At the current node, three axis handles in `canvas.axisX/Y/Z` at 12 px labels X, Y, Z; the active axis is drawn solid and the others hairline; a length field with its unit sits above-right of the active axis's tip ("1500" with the caret and "mm" in `text.muted`, the focus ring on the field), so the node's own plate stays readable (s2). The pointer controls come first (§5): a click on an axis handle makes that axis active; to the right of the length field sit three 22 px icon buttons, Reverse, Place and Cancel (§3.2), and a click on the short stub the active axis draws on its opposite side reverses it too; the length is typed, because a number is input. The keys accelerate the same controls: ⇥ or an arrow key cycles the axis, − reverses it, ↩ places, ⎋ cancels. Placing writes a layout row (From the current node, To the next number, the offset in the axis). The draft ghost has three parts (decision 12; Q-12): a thin dashed centreline in `canvas.draft`, 1.5 px with a 6/4 dash, from the node along the axis for the typed length; a faint tube outline at the draft's diameter, `canvas.draft` at 30% opacity and 1 px, so the size is read without a run of dashes; and the draft node's plate ("50") with a `canvas.draft` edge at the tip. Under the HUD a hint strip on `canvas.hint` (G-4), 22 px, `text.secondary` at 12 px, carries the tool's accelerators: "Route · ↩ places 50 · ⇥ next axis · − reverses · ⎋ cancels"; every HUD tool puts its keys there and nowhere else, and every key named there has its control on the canvas or in the inspector. While a route is open the drawer or table shows the draft row (§5.1) and the inspector its routing block (§5.3). A direction change inserts a bend with the default radius shown in the new row's expansion. Every commit is the same row operation as typing the row.

**Toast (G-8).** One component for the outcome of an act that no permanent surface records: a paste, a delete, an accepted row, a panel the product moved. Anatomy: `surface.raised`, elevation 2, `radius.card`, padding 8 12, 320 px wide (`layout.toast.width`); an optional 16 px icon in `text.secondary`; the message at 13/18 in `text.primary`, three lines at most (beside the action and the close control the message has about 190 px); at most one action as a text button in `text.link` ("Undo", "Reopen"); and a 22 px close control. Placement: the bottom-right corner of the surfaces, 8 px (`layout.toast.inset`) above the status bar and 8 px to the left of the agent strip or column, so it never covers the status bar, the strip or the column; with the inspector docked it lies over the inspector's foot. Duration: 6 s, or 10 s when it carries an action; the clock stops while the pointer is over it or the focus is in it; it enters and leaves in 120 ms. One at a time: a new toast replaces the one showing, and an Undo that was replaced is still the toolbar's Undo. Never for a state a permanent surface already carries: a failed run has the banner, the rail caption and the run log, a model change has the stale band and the caption, a docked or undocked inspector is its own evidence, so none of those raises a toast. The toast that names the agent column's collapse (§0) stays, because the reason for the collapse is on no surface: "Agent column collapsed to its strip: the docked inspector needs the width." with the action "Reopen".

**Dialogs.** Sheets at `radius.sheet`, elevation 3, 20 px padding, a title at 15/600, actions at the bottom right (primary on `accent.fill`); every dialog has a visible Cancel or Close button, and Escape accelerates it. Notices required by the registry appear once, at the top of the dialog, at 13/18 in `text.secondary`, verbatim from the registry:

| Dialog | Carries | Content |
|---|---|---|
| Libraries import | M-05 | The import of materials, sections, components, hanger tables and rule packs; hanger tables are a library class like the others, imported by the user and never bundled (ruling 5): a file field, the seven provenance fields (C-46) as a form with their enumerations, the six import flag classes (C-32) as a findings list with the triangle, and a quarantine action for suspected protected content (C-44). No built-in catalogue of any kind (C-51). |
| Export and handoff | M-06; and the report supplement M-04 is named as what the report will carry when export metadata is included | Ruled (ruling 9): the export is named by what it writes and never names another vendor's product. The command, in the File menu and the palette, is "Export model batch file (.mbf)…"; the Review header's "Export…" opens the same sheet; the sheet's title, with that target chosen, is "Model batch file (.mbf)". There is no line attributing the grammar to anyone and no compatibility flag, in the sheet, its tooltips, the loss report's headings or anywhere else in the product. Fields: target, units system for the file, what is included, the loss report as a list, export metadata summary; Write file… and Cancel. |
| Report preview | M-03; M-07 | Opened from the Review page header (decision 10): the outline of §5.5 rendered as it prints, with the required content block and the notice as fixed sections, the Review/signoff block, and Print or Save. |
| About | M-17 | SWBPIPE, version, build: the name and nothing appended to it (ruling 1). The maturity sentence that V1.1 gave About as its one line is removed from the product (ruling 2, as superseded) and the line's space is closed: the version and build line is followed directly by the tabs; a Scope and limitations tab with scope, validation status, known limitations, data-boundary constraints and professional-responsibility limitations. |
| Preferences | M-16 | Appearance: theme (System, Light, Dark); the contrast findings table of §2.9, computed from the tokens, headed "Contrast findings" with the line "Findings only. No conformance is claimed; the target is set by the project authority." Also display units, number formatting, the node increment. |

## 6. The 3D presentation language

### 6.1 The choice

The three languages in the concept return were: A, a drawing (flat shading, a dark edge line, the standard 2D symbols given thickness, node labels as the primary annotation); B, a physical model under studio light (material shading, hardware-like glyphs with a direction arrow, labels the exception); C, the report figure (neutral shading, a light edge line, the standard symbols the report legend explains, labels for every visible node up to a budget, camera presets shared with the report figures).

This system chooses **C, the report figure, as the base**, and takes two things from the others: **A's edge line as the carrier of the geometry**, made theme-aware, and **B's direction arrow on every restraint glyph**. It rejects B's studio lighting and hardware realism and A's labels-as-the-only-annotation stance. The reasons:

1. The tables are the model and the Review page produces the report. A canvas that is literally the figure the report prints, with the same camera presets and the same legend, means the engineer never wonders whether the reviewer will see something else. It is the only language in which the canvas is derived twice over: from the tables, and into the report.
2. Result colour must read as a scale. Studio highlights, specular falloff and material texture change the apparent lightness of the tube along its length, which corrupts a sequential scale; neutral matte shading does not. The concept return's own risk for B, that wrong shading makes the whole direction look like a game, is avoided by not attempting it.
3. Both themes must be peers. A drawing with an edge line survives inversion; a lit model does not, because the light has to be redesigned for a dark world. C with A's edge is the language that works in both without a second design.
4. Restraint semantics are where beginners fail (RESEARCH-D §2). B's small arrow on every glyph is explicit-over-implicit (principle 4) at almost no cost, so it is kept.
5. Node numbers are the shared address space (RESEARCH-D §3), so C's label rule, on for every visible node up to a budget, is the right default for a reviewer reading the figure; A's "primary annotation" stance is the same idea taken too far for a dense model.

The language is called the **figure**: the canvas draws the figure the report prints, with explicit glyphs.

### 6.2 The pipe

Pipes are drawn at real outside diameter, with insulation shown as a translucent sleeve when modelled. Shading is neutral and matte: the tube's colour is `canvas.pipe` with a soft darkening toward the silhouette in `canvas.pipeShade`, enough to read as a cylinder and never enough to read as lit. Every tube has an edge line in `canvas.edge`, one pixel at any zoom, drawn at the silhouette and at every change of section; in light it is a dark line on a mid grey tube on a pale ground, in dark a light line on a darker tube on a near-black ground, and in both the edge is what carries the geometry (contrast findings §2.9). Diameter changes read as steps in the edge. Elements without a section yet draw as a centreline of 1.5 px in `canvas.edge`, so a model can be routed before sections exist, and the section cell carries the required mark.

### 6.3 Fittings and components

Recognisable geometry, not symbols and not hardware: elbows swept at their bend radius with the tangent points marked by short ticks; tees as joined tubes with the branch's saddle line (the branch-connection kind is node data; the canvas draws the kind's silhouette when the kind determines it, otherwise the plain saddle); reducers as cones between the two ends; valves as the standard drawing silhouette extruded, a body with a bonnet stub; flanges as a pair of discs; rigid elements as a tube in `canvas.pipeShade`; expansion joints as a convoluted section. The list is the engine's element kinds, the ones the Type column offers, and the node data drawn on them (R-4); V1.1 also described beams, hinges and ball and slip joints, which the engine does not represent and the product does not offer, so they are not drawn. Every fitting takes the pipe's edge line and, when a result is shown, the result colour.

### 6.4 Restraints and loads

Restraint glyphs are the standard symbols the report legend explains, given thickness, in `canvas.glyph` stroke over `canvas.glyphFill`, always drawn at a fixed screen size so they read at any zoom: an anchor as a hatched block at the node; +Y as a triangle under the pipe pointing up; a guide as a bracket pair either side; a limit stop as a plate on the stopped side; a two-way restraint as a bar through the pipe in the restrained direction; a skewed restraint along its direction vector; a spring as a can with a rod; a connecting node as a thin tie to the other node. Each glyph carries a small arrow in `canvas.glyph` for the direction it acts (B's contribution), and a plate at 11 px in `canvas.label` on `canvas.labelBg` naming the restraint in the table's words ("+Y · gap 3 mm · μ 0.30", "Rigid Y · RS-02", "VS · H1", "Guide X Z · gap 2 mm · μ 0.30"), placed on the side of the node that is not covered by the docked inspector (D-10; s4_both). A gap draws as visible clearance between the glyph and the tube (4 px at the fixed glyph size), with the gap value on the plate; friction draws as a short hatch on the contact face with μ on the plate. Selecting a restraint's row lights its glyph with the selection halo.

Loads are arrows from the node, scaled by magnitude with a scale reference at the canvas's bottom right ("1000 N" beside the 1 m bar, D-9), labelled on a 12 px plate with value, unit and case ("−2000 N · W", "+4.0 mm · T1") on hover and selection and, when Labels is All, permanently (D-10). The arrow's form carries the kind: a single-head arrow for a force; a double-head arrow along the axis for a moment; an arrow with a bar at its base for a specified displacement; a row of short arrows along the element for a uniform load or weight; an arrow with tail streaks for wind; an arrow on a zigzag base for seismic. Colour by kind (§2.7) or by case when a case is selected; neutral when result colour is on and while a proposal ghost is drawn (D-10).

### 6.5 Node labels

Labels are the node number in `canvas.label` on a `canvas.labelBg` plate with a 1 px offset from the node, 12 px, tabular. The default is Budget (decision 13; Q-13): on for every visible node up to a budget set by the canvas size (one label per 3600 square px of canvas), with priority selected, hovered, current row, restrained, loaded, node-data, then branch points, then the rest by spacing; the current row's node is always labelled. Plates are placed by a search that keeps them off the tubes, off the glyph plates and inside the canvas, and a plate that cannot be placed yields to the higher priority; labels never overlap (D-7). The plate's edge names the node's state: `canvas.selection` when selected, `canvas.proposalGhost` on a proposal-ghost node, `issue.warning` on a node with an open issue, `canvas.draft` on the draft node (D-7). The Labels control (L) cycles Budget, All, Off from Budget; All is on demand for the dense figure and is not the remembered state.

### 6.6 Selection and hover

Selection is a halo: a 2 px `canvas.selection` outline around the element's silhouette and its glyphs, drawn against the ground, with the node label plate taking a `canvas.selection` edge. Hover is a thinner 1 px `canvas.hover` halo. Both sync with the tables both ways, and with the inspector. Multiple selection halos every selected element. Isolate dims everything else to 20% opacity; Hide removes it and shows a count in the HUD, which is a text button that shows everything again ("3 hidden · Show all"; §5.6).

### 6.7 Result colour

Applied to the tube and fittings as a flat colour per element (or per node interpolated along the element for stress), from the scale of §2.6, with the legend in the canvas. Unsolved elements stay `canvas.unsolved` at 60% opacity. Selection and hover halos draw over colour unchanged. The probe reads values at the cursor.

**Only a Current run colours the model (ruling 6; R-9).** Result colour, the deformed shape and the probe are an overlay on the current model, and the run-standing table of §5.1 decides whether there is one: for a Current run, all three; for a Stale run and for a Historical run, none of them. The canvas then draws the neutral figure of §6.2, the legend is its note card (§5.6), and the Deformation and Probe tools are disabled with their reason. The run's numbers are still read, in the tables, under their band. A Historical record is never painted onto the current model.

**The edge line on a coloured element (R-6; a default).** Edge lines remain over colour, so coloured geometry still reads as geometry, and on a result-coloured element the line is drawn in whichever of `canvas.edge` and `canvas.edgeAlt` has the higher contrast against the element's fill (its flat colour, or the mean of its interpolated colours). In both themes that keeps `canvas.edge` on the two steps nearest zero and gives `canvas.edgeAlt` the rest, the switch falling at about a quarter of the range in light and a third in dark, and the drawn line never reads below 2.75:1 on its fill (§2.9). The neutral pipe, unsolved elements, glyphs and the uncoloured figure always use `canvas.edge`. Choosing a line colour per element is a rendering requirement the engine has not been asked for before; it is an open item for the rendering observation (§8 item 13), and if it cannot be met the fallback is V1.1's: `canvas.edge` everywhere, the silhouette carrying a strongly coloured element.

### 6.8 Deformation

The deformed shape draws solid with the result colour or the pipe neutral, and the undeformed shape as an outline ghost in `canvas.deformGhost` at 1 px, dashed. The scale factor is stated in the legend ("Deformed ×50") and adjustable with a stepper and a field; the default is the factor that makes the maximum displacement one tenth of the model's extent, rounded to a round number, and the legend says so. Animation is a play control in the legend that oscillates the factor from 0 to the stated value; per case, never a mix of cases.

### 6.9 Proposed changes

A pending proposal draws as a ghost: the proposed geometry or glyph in `canvas.proposalGhost` dashed at 60% opacity beside the current one, 24 px along the run and never over it, so both read, with its own plate ("Variable spring · P-12", ghost edge) under the current glyph's plate (s8_model); a removed element draws as a dashed outline; the affected node label plate takes a `canvas.proposalGhost` edge. While a ghost is drawn the load vectors are neutral (D-10). Accepting turns the ghost into the drawing; rejecting removes it. The engineer's own routing draft is the other ghost, in `canvas.draft` as a thin dashed centreline with a faint tube outline (§5.6; decision 12), and the two are never the same colour nor the same form.

### 6.10 Camera presets and the report figure

Fit (all, or the selection; the HUD's first tool at every canvas width, and the start of the fitted state of §0), Iso, Top, Front, Right, and "Report figure": the named camera saved with the project that the report's figures use. Iso is azimuth 40°, elevation 28°, the wide-canvas view the frames drew (D-8); the frames' second azimuth of 200° for the tall Both-view canvas is a convention for one sample's geometry, whose branch riser projects onto the main riser at 40°, and not a preset: the product lets the engineer rotate (P-8). The report prints the figure exactly as the canvas draws it at that preset, in the theme chosen for the report (light by default), with the legend and the scale reference. Along the canvas's bottom edge: the triad at the left, stating the up axis in its label ("Y up"), and the 1 m bar with the load scale arrow at the right (D-9); the frames' centred mock line was a requirement of the mocks brief and is not a product element.

### 6.11 Both themes

Every token in §2.8 has a light and a dark value and the figure was designed for both: pale ground, grey tube, dark edge, dark glyphs in light; near-black ground, mid-grey tube, light edge, light glyphs in dark. The result scale flips its anchor (§2.6); halos, ghosts and vectors keep their hues and are re-stepped for the dark ground. The engine stays as it is: this section describes what the model looks like, and the rendering brief owed to the piping session carries it as overlay specifications.

## 7. Copy rules

### 7.1 Allowed and forbidden words

| Where | Allowed | Forbidden |
|---|---|---|
| Proposal controls | Accept, Reject, Accept all rows, Reject proposal, Accept all (n rows), Reject all | Approve, Apply proposal, Confirm, Sign off |
| Proposal standing | Proposal, draft until accepted, rationale, constraints considered, TBD | Verified, validated, approved, recommended by |
| The checked mark | Checked, Checked by *name*, stale, Check again, Clear check, Unchecked rows | Verified, reviewed and approved, signed, accepted |
| Statuses | the six status labels of the one table (§2.3), each with its authority domain and its raw token reachable in place | Any label not in the table; the product's earlier short forms; an unhyphenated rule-pack label; Ready, OK, Pass, Compliant, Certified, Sealed, Approved, Authenticated |
| Results | number, rule ID, pack version; Ratio, Allowable, Governing case, Envelope | Pass, Fail, OK, Compliant, Code compliant, Exceeds code, Safe, Acceptable |
| Evidence | the two evidence labels of the same table, Internally verified and Prover correlated, drawn only when the run record carries evidence | Engineer accepted (never emitted), Validated, Certified; an evidence chip on a run that carries none |
| Agent output | the five class words as card labels and no others: Check, Open issue, Draft, Proposal, Evidence summary (ruling 8); TBD | accepted, approved, verified, correct, compliant, anywhere in agent text; Note as the label of an agent card |
| Runs | immutable, named, the historical band's text "Historical saved run · Run a fresh solve to establish current results." and its popover's "Historical results cannot drive current overlays, rule checks, comparisons or report readiness." (the product's rendered text, ruling 6), No result colour on the current model, Needs a current run, solved, stopped in *OPE1*, Run *02* failed: *reason*, No results — …, Run again, Run settings, Run record, Model changed since Run *03*, The values below are from the model as solved, not yet designed, No run yet, No solved run, Failed and Stale as rail captions | Final, Released, Certified run, Aborted, Error as a status, Invalid results, Passed |
| The Run button | Run; Run is unavailable — *the blockers of the solve* | Ready, Validate, Check model |
| The Review page | Report preview, Export…, Snapshot…, compared with, Compare with…, Clear comparison, Show edits, Comment, Resolve, Reopen, Insert, Discard, Kind, the agent's kinds Checks, Open issues, Drafts and Evidence summaries, Note for the engineer's own comment, live, drafted, edited, empty, current, fixed sections come from the record; Review/signoff block as the registered section heading (RESEARCH-C §4 row M-07; P-10) | Sign off as a control, Approve section, Finalise, Release |
| Tables and the canvas | Draft, Paste *3 rows*, Ignore, joined row, ⎋ closes, Add restraint…, Read-through on / off, Origins, Sorted by ratio, Envelope: governing case per element, Route · ↩ places *50* | Confirm, Commit, Lock |
| Anywhere | SWBPIPE as the product's only name | any longer or earlier product name; a maturity word appended to the name; the maturity sentence and the acceptance sentence, in any form (rulings 1, 2 and 3); the name of another vendor's product (ruling 9); certify, seal, approve, authenticate, comply, compliant, sign-off as a control or a state, non-authoritative, not authoritative; the fence token; any stack of three or more boundary terms outside a registered sentence. The Review page's section heading "Review/signoff block" is registered text and the one exception (P-10). |

Generated text (toasts, empty states, tooltips, agent messages) follows the same table. Toasts say the outcome of an act and offer its one action (§5.6): "Pasted 3 rows · 2 columns ignored" with Undo; "Deleted 2 rows" with Undo; "Accepted P-12 row 1" with Undo; "Agent column collapsed to its strip: the docked inspector needs the width." with Reopen. A failed run raises no toast, because the banner, the rail caption and the run log carry it (G-8); V1.1's example of one is withdrawn. The frames' strings (MOCKS_V1 §2) are the reference wordings for the moments they draw; where this document and a frame differed, the frame's wording is adopted here unless §9 says otherwise.

Spelling. Product copy uses Canadian English, as the product's existing menu "Analyze" and this system's "colour" both do: "Analyze", "colour", "centre", "centreline", "modelled", "labelled". The registered sentences keep their own spelling verbatim.

### 7.2 Ratios

A ratio is always three things together: the number to two decimals, the rule ID and the pack version, in that order in a table (Ratio, Rule, Pack) and in one line elsewhere ("0.72 · EXP-A1 · pack 1.2"). A ratio never carries a verdict word or a verdict colour; the data bar is the scale, and 1.0 is a tick.

### 7.3 Checked

Ruled (D-71 item 7, option A; §9). The Checked mark is a plain human tag. It is set on one row or on many rows at once; it records who and when and a hash of the row's content; it is shown stale when that content changes; it is set and cleared by the engineer only, never by the agent and never by the product; it is kept in the project's interface state and not in the model; and it is never in a status, an export, a run record or a report. The ruled words are used exactly, wherever they apply. The control is "Check" (⌘⇧K accelerates it; the controls are in §4, §5.1 and §5.3) and the mark is "Checked". Its tooltip: "Checked by *name* · *date time* · bound to this row's content · not a software status". When stale: "the row changed since · Check again or Clear", after the same name and date. The filter is "Unchecked rows". The other control names in this system derive from the ruling and add no new word: "Check rows" in the footer's selection group is the control for many rows (the ruling's "set on one row or many"), and the "Check again" and "Clear check" buttons are the two halves of the stale text, "Check again or Clear". The word is a tag on a row and nothing more: it is not one of the labels of §2.3, it lights no chip, and the footer's count of Checked rows is a filter, and no surface turns it into a status or a cue that anything is ready. The anatomy is V1.1's, unchanged (§4): a check in `mark.checked` in the state slot, dashed with a dot in `mark.checkedStale` when stale.

### 7.4 Disclosure homes for the chosen direction

One placement per surface class. Two disclosures have no home, because the owner removed them from the product: the maturity sentence (M-01) and the acceptance sentence (M-02). Neither appears on any surface, in any form or short variant: not in the status bar or a popover of it, not in About, not in the results header or its disclosure, not on the Review page, not in the report's body, the inspector, the probe, the canvas or the agent panel; and the space each held is closed, not left as a gap (§5.1, §5.2, §5.5, §5.6). What carries the distinction between what the software computed and what a person decides is now the status chips with their authority domains (§2.3) and the report's required notice (M-03), which is governed text outside this system. Neither ruling touches the control word "Accept" for a proposed edit, or the Checked mark.

| ID | Home |
|---|---|
| M-03 | The report and the report preview; the Notice section of the outline. |
| M-04 | The report and its preview when export metadata, hanger selection records or handoff data are included; named in the export dialog. |
| M-05 | The Libraries import dialog; the export and redaction surfaces; and the hanger-selection surface, which carries the content boundary's short variant once, "no protected standards content; code-specific data is user-supplied" (ruling 5; §5.1). |
| M-06 | The export and handoff dialog. |
| M-07 | The report outline on the Review page, whose last row carries the registered name "Review/signoff block" (RESEARCH-C `C_ui_constraints.md` §4 row M-07; `docs/PRD.md` §19.2), and the preview. |
| M-08 | The status bar chips, none, one or two by §2.3, each drawn with its authority domain and with its raw token reachable in place; Human review required on the Review page and in the report. |
| M-09 | The results header chip beside the run name and the probe's footer, only when the run record carries evidence and only for a Current run; the report's case pages. |
| M-10 | The issues drawer, the gutter's state slot, the cell marks, the rail's Issues count, the canvas halo on an entity with a blocking issue. |
| M-11 | Every column header, inspector row, probe row, legend and report table. |
| M-12 | The inspector's provenance disclosure; the Libraries dialogs. |
| M-13 | The results header band with the product's rendered text, its popover with the second sentence and the run's recorded statuses, the rail's Historical caption (`rail.captionHistorical`), and the legend's note card (ruling 6; §5.1, run standing). |
| M-14 | The proposal card ("draft until accepted"), the diff, Accept and Reject, the rationale, constraints and TBD items, the Accepted record. |
| M-15 | The rule-expression cell's dashed frame and caption on the Loads page and in the Rules dialog. |
| M-16 | Preferences, Appearance, Contrast findings. |
| M-17 | About, Scope and limitations. |

### 7.5 The product name

Ruled (ruling 1): the product is SWBPIPE, and only SWBPIPE. The longer name that V1.1 used beside it is dropped everywhere: the window title, About, the report's identity block, the toolbar, the file badge, every example and tooltip, this document's title and the specimen all read SWBPIPE. Nothing is appended to the name: the product is never called a preview, or given any other maturity word, as part of its name. The name the product had before appears nowhere. SWBPIPE is set as a wordmark in the system face at 13 px medium with 0.04 em tracking; there is no logotype in this system.

### 7.6 Shortcuts in copy

The pointer rule of §5 has a consequence for words. A shortcut is never the name of an action and never the only instruction: a tooltip reads the control's name and then its accelerator in parentheses ("Fit (F)", "Inspector (⌘I)", "Place node 50 (↩)"); a toast, a band and an empty state name the control to click ("Reopen", "Run again"), never a key to press; the hint strip lists a tool's accelerators and every key on it has its control on the canvas or in the inspector (§5.6); the menu bar and the palette show the accelerator beside the command. "Press" is not a word the product uses to tell the engineer what to do.

## 8. Open items and what the mocks must test

The answers to Q-15 to Q-22, and the choices this revision made to apply them, are defaults: the owner expects to revisit details of this kind once the product is in use, and nothing marked "a default" in this document is doctrine. V1.1's item numbers are kept so that other records can still cite them; a closed item says what closed it.

1. Closed. The labels of the six statuses and the two evidence labels are ruled and are the one table of §2.3 (ruling 4). The Checked mark's classification and words are ruled too (D-71 item 7, option A): a plain human tag, with the words of §7.3.
2. Closed. The export is named by what it writes and never names another vendor's product (ruling 9; §5.6).
3. Closed. The docked inspector's narrow case follows UX_SPEC_V1 §10.9 with this system's 220 px canvas minimum, which ROOT confirmed as governing when it accepted V1.1; the specification's larger floors are reconciled to it in that document's revision (R-1). The second mock pass drew the column collapsing (`s4_both_light_column`). V1.2 adds the other order, the column reopened while the inspector is docked (Q-15; §0), which no frame draws yet.
4. The read-through T and P columns stay off by default in Both view, with the values in the Load cell's tooltip and the footer chip to switch them on; the frames confirm the table fits at 737 px that way (s4_both). Open only if the owner wants them on with a horizontal scroll.
5. The categorical set's all-pairs standing beyond three slots: this system makes form the carrier of load kind; the rendering observation should confirm the arrow forms are drawable at the sizes stated.
6. The contrast target (M-16): the findings in §2.9 are the owner's input; `text.disabled`, the hover halo, the light pipe neutral and the near-zero dark step on the canvas (2.18:1) are the pairs most sensitive to the choice. The edge line on a strongly coloured element (1.10:1 on the brightest dark step, 1.02:1 on step 6 in light) is carried here as ROOT directed (R-6) and is answered by `canvas.edgeAlt` if item 13 holds.
7. The captions "Failed" and "Stale" and the stale band's sentence (D-3, s8_table) are this system's wording, state names and not registered text; they are listed in §7.1. The historical band's wording is no longer open: it is the product's rendered text (ruling 6).
8. The wordmark: a typeset SWBPIPE; a drawn mark is not part of this system. The name itself is ruled (ruling 1).
9. Whether `MODEL_INCOMPLETE` is reported before any run: the chip policy (§2.3, D-4) assumes the engine reports it whenever the model cannot be solved, including before the first run. If the engine reports nothing before a run, the bar is empty until then and the Issues count and the Run tooltip carry the reason. What a stopped run's record carries is the same kind of question (Q-21): until the engine answers, a stopped run that carries no status shows no chip.
10. The run log popover's four rows (decision 5) assume the engine's log can be grouped by case as the frames drew it; the run record carries whatever the engine emits.
11. Hiding node labels above a density (Q-13) is not adopted: Budget is the density rule (decision 13); if the owner finds the dense figure of state 8 too busy, the budget constant (one label per 3600 square px) is the one number to change.
12. The fitted camera (Q-20; §0) is a proposed semantic addition, one bit of interface state beside the camera: fitted from a Fit until the engineer moves the camera. It is named as a change for implementation and is not a restyling.
13. `canvas.edgeAlt` (R-6; §6.7) asks the engine to choose an edge-line colour per element from the element's fill. Whether instanced geometry can do that is a question for the rendering observation; the fallback is V1.1's single edge.
14. The authority-domain word for the two evidence labels. Ruling 4 requires the domain to be shown with every label; the governed domains (Solver, Rule pack, Human) are the statuses'. "Evidence" is this system's word for the two evidence labels and is the owner's to confirm or replace; it is one field in `tokens.json`.
15. The Kind menu of the Review page's comment stream lists the agent's four kinds, as Q-22's answer is written. The engineer's own notes are reached with Mine; a note by another person has no kind entry. If that is missed in use, "Notes" is a fifth entry in the same menu.
16. A Stale run drives no canvas overlay in this system (§5.1, run standing), because after a model change the run is kept on a Historical record's terms and a Historical record never takes a current-model overlay (R-9). UX_SPEC_V1 §2.6 keeps the colour on unchanged elements and §10.4 gives a historical run a probe and a legend footer; ROOT confirmed the Stale column when it reviewed V1.2, and the specification's revision will follow the run-standing table.
17. Closed by ROOT (correction 1 to V1.2). After a model change the run stops being the current solve basis; the product already behaves this way, clearing the solve proof and the model hash when a model commit lands. The clause that V1.1 and V1.2 kept from decision 4's frame wording, which said the run remained the basis until the next run, was wrong and is removed from the band, its row in §4 and the vocabulary of §7.1. The band reads: "Model changed since Run 03: *the change, with its time*. The values below are from the model as solved." with "Run again" at its right; the compact form reads "Model changed since Run 03 · *change* · Run again". It agrees with the consequence line of §5.4 and with UX_SPEC_V1 §6.3.
18. The run log's placement (Q-16) is specified by its purpose, over the canvas's top edge with the table's header rows uncovered. The answer's words, "right-aligned under the Run button", cannot also hold at 1440, where the Run button sits over the table pane; §5.2 says which reading was taken.

What the third mock pass (MOCKS-03) must draw, beyond regenerating the eighteen frames from `tokens.json` 1.2:

- Every status and evidence chip from the one table, with its domain ("Solver · Mechanics solved", "Rule pack · User-rule checked"), in the status bar, the results header and the probe; no chip on a run that carries no evidence.
- The results header with the run identity disclosure alone, and the Review page with its columns directly under the header: no sentence in either, and no space where one was. The two decision-aid frames retire: what they asked is ruled.
- The toolbar with Undo, Redo and the Inspector toggle, latched in the docked Both-view frames; the status bar's About control with no popover.
- The table footer with the selection group in a frame with a selection, and the edit chip in the editing frame; the expansion chevron in the Type and Expression cells; the "Add row" line.
- State 2: the compass with Reverse, Place and Cancel beside the length field, and "Place node 50" in the routing block.
- State 4: the two-row HUD at 303 px with Fit first, the mock line under the stack (G-10); and a new frame, the agent column reopened with the inspector as the slide-over (Q-15).
- State 6: the run log over the canvas's top edge with the dot and triangle glyphs (Q-16, G-12); the empty left end of the status bar (Q-21); the drawer row's overflow (G-11).
- State 8: the receipt line (Q-18), the new consequence line, the neutral canvas and the legend's note card under a Stale run; the toast's anatomy where a frame shows one (G-8); the hanger selection expansion with the content boundary's short variant (ruling 5).
- State 9: the header with icons, Export… plain, "Compare with…" in the iteration menu (Q-17, Q-19, G-7); the four chips and the Kind menu (Q-22); an Evidence summary card and a Draft card (ruling 8); Human review required in the status bar.
- A Historical frame, which no pass has drawn: the historical band with the rendered text and its popover, the neutral canvas, the legend's note card, no status chip (ruling 6).
- About and the export sheet, if dialogs are drawn: the name alone, and "Model batch file (.mbf)".

## 9. Change log V1 to V1.1, and V1.1 to V1.2

Sources: the direction record §11 (decisions 1 to 14, which answer MOCKS_V1 §4's Q-1 to Q-14 in the same order), MOCKS_V1 §1 (D-1 to D-11), §2 (the twelve frames' decisions, cited as §2 s1 … §2 s9), §3 (P-1 to P-10) and §5 (G-1 to G-6), the brief DESIGN-SYSTEM-02, and ROOT's review of V1.1 (rows 48 and 49), which adopts `../UX-SPEC/UX_SPEC_V1.md` §10.9 for the docked inspector's narrow case. Every item of MOCKS_V1 §1, §2 and §3 is adopted unless the table says otherwise: P-5 and P-9 are superseded by decisions 1 and 12; P-8 is adopted for the frames and overruled as a product preset; the s5 frame's cell popover is overruled by decision 14; D-8's second azimuth is a frame convention, not a preset. The tokens and the specimen changed with the document; at V1.1 `tokens.json` was version 1.1 and the specimen was V1.1.

V1.1 to V1.2 is rows 50 onward; rows 1 to 49 stand as V1.1 wrote them, including where a later row supersedes what they say. Sources: the brief DESIGN-SYSTEM-03, which carries the owner's rulings on decision packet D-71 as design consequences (cited as ruling 1 to ruling 9), ROOT's recommended answers to Q-15 to Q-22 as the owner accepted them, with Q-20 as the owner amended it, the gaps G-7 to G-12 of MOCKS_V2 and R-3 to R-9 of ROOT's preparation record; and ROOT's message after the brief was sealed, which relays two later directions of the owner: the maturity sentence removed everywhere (superseding ruling 2) and the Checked mark ruled as option A (superseding ruling 7). The answers to the questions are defaults and not doctrine. `tokens.json` is version 1.2 and the specimen is V1.2. Rows 88 and 89 are ROOT's correction 1 to V1.2, made after ROOT reviewed the return; rows 1 to 87 stand as they were returned. Row 90 is ROOT's correction 2, from the independent review REVIEW-03; rows 1 to 89 stand unchanged. Where rows 50 to 89 give "Q-20 as amended by the owner" as the source of the general pointer rule, read it as row 90 states it: the owner's words concern Q-20, and the generalization is ROOT's reading.

| # | Change | Where | Source |
|---|---|---|---|
| 1 | The Both-view inspector docks on the canvas's right edge; the canvas shrinks to the remainder; the table never reflows. Geometry table, the 220 px canvas minimum, the camera rule, the narrow-canvas rules for HUD, legend and probe (the narrow case's rule is row 48). `layout.canvas.min` 220 added. | §0, §1.4, §5.3, §5.6, §8 item 3 | decision 1; Q-1; P-5 superseded (the frame's fit-to-the-visible-part is replaced by the camera rule) |
| 2 | No chip when the model is complete but unsolved; no seventh label. | §2.3 | decision 2; D-4; Q-2 |
| 3 | Two chips after a solve, one per authority domain; Human review required on the Review page only, pending packet item 4. | §2.3, §5.2, §7.4 M-08 | decision 3; D-4; Q-3 |
| 4 | After a model change the chips drop; the rail caption reads Stale; the results header carries the stale band; the status bar is silent about it. | §2.3, §2.5, §4, §5.1, §5.2, §7.4 | decision 4; Q-4; §2 s8_table |
| 5 | The run log is a shorter popover on the Run button (title, at most four rows, footer); the full log is on the run record. | §5.2 | decision 5; Q-5; §2 s6 |
| 6 | One failure banner, on the page; the drawer opens filtered to the failing class and its row carries the link; the empty-state wording. | §5.3 | decision 6; P-3; Q-6; §2 s6 |
| 7 | Required marks on the first element's row; the start node's row shows "—" and carries none; its read-through cells empty. | §4, §5.1 | decision 7; D-1; P-6; Q-7; §2 s1 |
| 8 | With Envelope on the case selector stays visible and disabled ("Case: all"). | §5.1 | decision 8; Q-8; §2 s7_table |
| 9 | The run identity line lives inside the results header disclosure under the acceptance sentence. | §5.1, §7.4 M-02 | decision 9; Q-9; §2 s7_both |
| 10 | Report preview and Export… on the Review page header; the preview dialog is opened from there; icons added. | §3.2, §5.5, §5.6 | decision 10; Q-10; §2 s9 |
| 11 | The dark result scale re-anchored one step down and re-validated: `result.scale.1–7` dark are `#045c59 … #74d1cb`; the top step under `canvas.edge`; the findings re-read; new pairs for the edge line and the data bar. | §2.6, §2.9, §8 item 6, `tokens.json` | decision 11; Q-11 |
| 12 | The draft ghost is a thin dashed centreline with a faint tube outline and the draft node's plate. | §5.6, §6.9 | decision 12; P-9 superseded; Q-12; §2 s2 |
| 13 | Node labels default to Budget with All on demand; the placement search; plate edges by state; the control cycles from Budget. | §3.2, §6.5, §8 item 11 | decision 13; D-7; Q-13 |
| 14 | The combination editor is a row expansion under the case, with the joined-row anatomy; the row expansion generalised to three cases. | §5.1 | decision 14; Q-14; §2 s5 (the frame's cell popover overruled) |
| 15 | A propagated cell carries the tick only; a cell typed over loses it. | §4, §5.1 | D-2; §2 s3 |
| 16 | Rail captions Failed, Stale, Historical at caption size in their own tokens; the disabled items' tooltips "No run yet" and "No solved run". | §1.1, §5.2 | D-3; G-2 |
| 17 | The footer is the counts line, with the Read-through and Origins switches as outline chips and the state chips. | §5.1 | D-5; §2 s3; §2 s4_table; §2 s7_both |
| 18 | Tab strips carry counts; the strip is the stage's in every view. | §5.1, §5.3 | D-6; P-7 |
| 19 | Iso is azimuth 40°, elevation 28°; the 200° azimuth is a frame convention and not a preset. | §6.10 | D-8; P-8 (adopted for the frames, overruled as a preset) |
| 20 | The triad at the left and the 1 m bar with the load scale at the right of the canvas's bottom edge; the mock line is not a product element. | §6.4, §6.10 | D-9 |
| 21 | Restraint glyph plates at 11 px and load plates at 12 px in the table's words; vectors neutral while a proposal ghost is drawn. | §1.1, §2.7, §6.4, §6.9 | D-10 |
| 22 | Result colour interpolated in OKLCH between the seven steps, as §2.6 already said; the specimen keeps the same interpolation. | §2.6 | D-11 (adopted, no wording change) |
| 23 | The Run tooltip names the blockers of the solve and only those: "Run is unavailable — …". | §1.3, §5.2, §7.1 | §2 s1 |
| 24 | The routing block on top of the inspector; the hint strip on `canvas.hint`; the draft row's anatomy; the length field above-right of the axis tip; − reverses the axis. | §5.1, §5.3, §5.6 | §2 s2; G-3; G-4 |
| 25 | Paste preview rows as draft rows with propagated values; ignored columns keep their source name; the editing cell's unit at caption size at the right edge. | §1.1, §5.1 | §2 s3 |
| 26 | Both view with read-through off narrows Type to 96; glyph plates placed away from the docked inspector; the restraint mark's hover card. | §1.5, §5.1, §6.4 | §2 s4_both; P-5 |
| 27 | The row expansion's anatomy: caption with the chevron, the child table at its own widths, the add row; one per ⌘↩ until ⎋; the footer counts them. | §5.1 | §2 s4_table |
| 28 | The Loads stage's header band: pack identity, Generate from rule pack…, the single M-15 caption; the Origin column's three wordings. | §5.1 | §2 s5 |
| 29 | The legend's quantity line carries the case; the probe pinned 18 px from the node and clear of the legend; "Sorted by ratio" as a footer chip; Element 80 and Allowable 116. | §5.1, §5.6 | §2 s7_both; P-4 |
| 30 | The Ratio column's header menu; the Envelope footer chip. | §5.1 | §2 s7_table |
| 31 | The stale band's sentence with Run again and the hatched values; the not-yet-designed hanger row; the library caption; the collapsed accepted row with Undo; Accept remaining (n rows) and Reject remaining; the consequence line; the Accepted record line under the card; the quoted question. | §4, §5.1, §5.4 | §2 s8_table; G-1 |
| 32 | The proposal ghost 24 px beside the current glyph with its own plate; the drawer hides Connecting node and Note first; the inspector's proposed line. | §5.1, §5.3, §6.9 | §2 s8_model |
| 33 | The Review header (iteration combobox, "compared with", Show edits, Snapshot…, Report preview, Export…); the outline's state words, lock glyph and note; live-block captions and the inserted/removed legend; the comment stream's header, filters and card anatomy; the comment glyph in the state slot. | §3.2, §4, §5.5 | §2 s9; decision 10 |
| 34 | Header cell padding 0 5 and header units at caption size in `text.secondary`; `layout.headerPadding` 5 and `layout.cellPadding` 8 added. | §1.1, §1.2, §1.5, §5.1, `tokens.json` | P-1 |
| 35 | The inspector's label column at 88 px; `layout.inspector.label` 88 added. | §1.5, §5.3, `tokens.json` | P-2 |
| 36 | The column widths of the other tables, in a table. | §1.5, §5.1 | P-4 |
| 37 | "Review/signoff block" as the outline's registered heading and in the preview; the exception written into the copy rules and the disclosure homes. | §5.5, §5.6, §7.1, §7.4 M-07 | P-10; RESEARCH-C `C_ui_constraints.md` §4 row M-07 |
| 38 | `stale.band` / `stale.ink` added; the stale band and its hatch specified; contrast rows. | §2.1, §2.2, §2.5, §2.9, §4, `tokens.json` | G-1 |
| 39 | `rail.captionFailed`, `rail.captionStale`, `rail.captionHistorical` added, equal by design to the failed ink, the stale ink and the historical ink; contrast rows. | §2.1, §2.2, §2.9, §5.2, `tokens.json` | G-2 |
| 40 | `draft.bar` added, equal by design to `canvas.draft`; the draft row's bar specified; contrast rows. | §1.3, §2.1, §2.2, §2.5, §4, §5.1, `tokens.json` | G-3 |
| 41 | `canvas.hint` added for the hint strip and keyboard hints; contrast row. | §2.1, §2.2, §2.8, §5.6, `tokens.json` | G-4 |
| 42 | `pressed.fill` / `pressed.ink` added for latched toggles, distinct from the segmented control; contrast rows. | §1.3, §2.1, §2.2, §2.9, §3.2, §5.2, `tokens.json` | G-5 |
| 43 | `bar.track` added for the data bar's track; contrast rows. | §2.1, §2.2, §2.9, §5.1, `tokens.json` | G-6 |
| 44 | The copy rules extended with the frames' strings, the Run button's, the Review page's and the tables' rows; toasts re-worded to the story. | §7.1 | MOCKS_V1 §2; decisions 4, 5, 6, 10 |
| 45 | The open items revised (3, 4, 6, 7) and added (9, 10, 11); the list of what the second mock pass must test. | §8 | decisions 1, 11, 13; D-3; D-4; Q-13 |
| 46 | The §2.2 and §2.9 tables and the specimen's CSS, token JSON and pair list are generated from `tokens.json` between markers by `tools/gen.mjs` and `tools/splice.mjs`; `tools/agree.mjs` checks the three files; the validator is located by path or environment, never by an absolute path. | status line, companions, §2.2, §2.9, `tools/` | brief DESIGN-SYSTEM-02 |
| 47 | The specimen is V1.1: the docked Both-view geometry, the chip policy's moments, the stale band and rail captions, the draft row and the draft ghost, the row expansion with the combination editor, the latched Agent toggle, the data bar's track, the shorter run log, the single banner, the collapsed accepted row, the Review header and the "Review/signoff block" row, the disabled "Case: all", the re-anchored dark scale, the ten new tokens. | `specimen.html` | decisions 1 to 14; G-1 to G-6 |
| 48 | The docked inspector's narrow case: when the canvas would fall under its minimum the agent column collapses to its strip first (⌘⇧G re-opens it); if that is not enough the inspector opens as a slide-over for that window size only, with the tooltip "Docked inspector needs a wider window"; the canvas is never collapsed. Replaces V1.1's edge tab in §0, §5.3, §8 item 3, the mock list and the specimen's geometry strips; the 220 px minimum, the camera rule and the 400 px rules stay. | §0, §5.3, §8, `specimen.html` | ROOT's review of V1.1, adopting `../UX-SPEC/UX_SPEC_V1.md` §10.9; decision 1 |
| 49 | Spelling rule: product copy uses Canadian English ("Analyze", "colour", "centre"); the registered sentences keep their own spelling. | §7.1 | ROOT's review of V1.1 |
| 50 | The product's name is SWBPIPE and nothing else: the longer name V1.1 used is dropped from the title, the wordmark's description, the window title, About, the token file's name, the specimen and the tools; nothing is appended to the name. | title, §5.2, §5.6 About, §7.5, `tokens.json` name, `specimen.html`, `tools/` | ruling 1 |
| 51 | The maturity sentence is removed from the product entirely: no status-bar popover line, no About line, no specimen sample and no example string; its §7.4 home (M-01) is removed and the space it held is closed. The status bar's information popover goes with it, and the control opens About directly. | §5.2 status bar, §5.6 About, §7.4, §7.5, `specimen.html` | ruling 2 as superseded by the owner's later direction, relayed by ROOT after the brief was sealed |
| 52 | The acceptance sentence is removed from the product: the results header's disclosure holds the run identity line alone and its control is named "Run identity"; the Review page's columns begin under the header's hairline; the M-02 home is removed from §7.4 and every space is closed. The Accept control, the Checked mark and the report's required notice are untouched. | §5.1 results tables, §5.5 header, §7.4, `specimen.html` | ruling 3 |
| 53 | The six statuses and the two evidence labels come from one table, held in `tokens.json` (`labels`) and generated into §2.3 between markers; the hyphenated forms "User-rule checked" and "User-rule failed" are used everywhere; the three rules (token reachable in place; authority domain shown with the label, inside the chip; no label outside the table). The chip's anatomy gains the domain. | §2.3, §7.4 M-08 and M-09, `tokens.json`, `tools/gen.mjs`, `tools/splice.mjs`, `specimen.html` | ruling 4 |
| 54 | Chip policy under the ruling: Human review required on the Review page and in the report; no chip when the model is complete but unsolved, now ruled; the evidence chip only when the run record carries evidence, with no placeholder. | §2.3, §5.1, §5.6 probe | ruling 4 |
| 55 | Hanger tables are a library class the user imports and the product never bundles; the hanger table says so when no library is imported; hanger selection is specified as the row's expansion and carries the content boundary's short variant, verbatim, once. | §5.1 row expansion and results tables, §5.6 Libraries, §7.4 M-05, `specimen.html` | ruling 5 |
| 56 | The historical band's wording is the rendered text, "Historical saved run · Run a fresh solve to establish current results.", and its popover carries "Historical results cannot drive current overlays, rule checks, comparisons or report readiness." | §4, §5.1, §7.1, §7.4 M-13, `specimen.html` | ruling 6; R-7 |
| 57 | Run standing: the header band, the legend, the canvas colour state, the status bar, the rail caption and the Review blocks are specified separately for a Current, a Stale and a Historical run; only a Current run drives an overlay, a chip, the evidence chip or a readiness cue. The legend's note card and the two disabled HUD tools follow. | §2.3, §5.1 run standing, §5.6 legend and probe, §6.7, `specimen.html` | ruling 6; R-9 |
| 58 | The Checked mark is ruled: a plain human tag, per row or for many rows, recording who, when and the hash of the row's content; stale when the content changes; set and cleared by the engineer only; kept in the project's interface state and never in a status, an export, a run record or a report. Words: control "Check", mark "Checked", the tooltip and the stale text as ruled, filter "Unchecked rows". The open item closes. | §4, §5.1, §5.3, §7.3, §8 item 1 | ruling 7 as superseded by D-71 item 7 option A, relayed by ROOT after the brief was sealed |
| 59 | Agent cards carry exactly one of five class words, Check, Open issue, Draft, Proposal, Evidence summary, held in `tokens.json` (`agentCardClasses`); the agent-authored "Note" becomes "Evidence summary" and Note stays the engineer's own; accepted, approved, verified, correct and compliant never appear in agent text. | §5.4 card classes and Checks, §5.5 comment stream, §7.1, `tokens.json`, `specimen.html` | ruling 8 |
| 60 | The export is named by what it writes: the command "Export model batch file (.mbf)…", the sheet's title "Model batch file (.mbf)", no attribution line and no compatibility flag; no other vendor's product is named in any product copy or in the specimen; the layout grammar is called the model batch file grammar. | §5.1, §5.6 Export and handoff, §7.1, §8 item 2, `specimen.html` | ruling 9 |
| 61 | The pointer rule: every action has a visible primary control operated by a mouse click; a keyboard shortcut is an accelerator for a control that exists, never the only way. Stated at the head of §5 with the audit of V1.1's key-only actions, and in §7.6 for copy: a tooltip names the control first and the key after it. | §5 intro, §7.6, §3.1 | Q-20 as amended by the owner |
| 62 | Pointer controls for the table: the gutter header selects all; the footer's selection group (Insert row below, Delete rows, Copy rows, Paste, Check rows, and the clear-selection control); the edit chip with Commit and Cancel; the "Add row" line; the keyboard table gains a column naming each key's control. | §5.1 | Q-20 |
| 63 | Pointer controls for expansions and marks: the expansion chevron in the owning cell; the block's closing chevron and close control; a click on a marks glyph opens its popover, which ends in "All marks on this row"; Check and Clear check as buttons. | §4, §5.1 row expansion, §5.3 | Q-20 |
| 64 | Pointer controls in the shell: Undo and Redo in the toolbar; the Inspector toggle beside the Agent toggle, labels dropping below 1360 px; the inspector's close control in Both view; the agent column's collapse control; the drawers' close and collapse controls; the agent's Send control. | §0, §5.2, §5.3, §5.4, §3.2 | Q-20 |
| 65 | Pointer controls on the canvas: the compass's axis handles take a click, with Reverse, Place and Cancel buttons beside the length field; the routing block's three buttons; the pinned probe's close control; the hidden count as the button "3 hidden · Show all"; the hint strip names only keys whose controls exist. | §5.3, §5.6, §6.6, `specimen.html` | Q-20 |
| 66 | Dialogs: every dialog has a visible Cancel or Close button; the combination editor's and the dialogs' buttons read their names, with the keys in tooltips. | §5.1, §5.6 | Q-20 |
| 67 | The fitted camera: the canvas refits on dock and undock when the camera has not moved since the last Fit, and otherwise keeps V1.1's camera rule; Fit is the first HUD tool at every canvas width. | §0, §5.6, §6.10, §8 item 12 | Q-20 |
| 68 | The agent column reopened while the inspector is docked: the column opens and the inspector becomes the slide-over for as long as the column is open at that width; two geometry rows added. | §0, §5.3, `specimen.html` | Q-15 |
| 69 | The run log's placement: its left edge at the Run button's left edge or the canvas pane's left edge, whichever is further right, so the table's header rows stay uncovered in Both view; 360 px (`layout.runlog.width`). | §5.2, §8 item 18, `tokens.json` | Q-16 |
| 70 | The Review header's comparison: "Compare with…" in the iteration menu; the compared-with combobox and "Show edits" appear only while something is compared; no fourth header button. | §5.5 | Q-17 |
| 71 | The decided row's receipt line: the done check, the first changed field with its new value, a count of further fields, the time and Undo; never wraps; the old value in the tooltip and the diff. | §5.4, `specimen.html` | Q-18 |
| 72 | Export… on the Review header is a plain button: the window's one accent is Run. | §5.5, `specimen.html` | Q-19 |
| 73 | After a failed run the bar shows the statuses the run record carries; a stopped run that carries none shows no chip and Model incomplete is not carried over. | §2.3, §8 item 9 | Q-21; R-3 |
| 74 | The comment stream's filter row: four chips for state and ownership, All, Open, Resolved, Mine, and one Kind menu for Checks, Open issues, Drafts, Evidence summaries; a kind with no cards takes no space. | §5.5, §8 item 15, `specimen.html` | Q-22 |
| 75 | Icons for the Review header's three buttons, Snapshot…, Report preview, Export…; the set also gains Undo, Redo, Inspector, Send, Copy rows, Reverse and Collapse. | §3.2, §5.5, `specimen.html` sprite | G-7; Q-20 |
| 76 | The toast is specified as one component: anatomy, placement, 6 s and 10 s durations (`motion.toast.ms`, `motion.toastAction.ms`), one at a time, and never for a state a permanent surface carries; V1.1's failed-run toast is withdrawn and the agent-column toast is kept. | §1.4, §5.6, §7.1, `tokens.json`, `specimen.html` | G-8 |
| 77 | G-9, the results caption that would have carried the acceptance sentence, closes unused: the sentence is removed, so no caption exists. | §5.1, §7.4 | G-9; ruling 3 |
| 78 | The HUD is one group of ten tools with Fit first; one row of 304 px in a canvas of 400 px or more, two rows of five, 154 px by 64 px, below that; the narrow canvas's stack and the mock line's place. | §0, §5.6, `tokens.json` layout.hud.*, `specimen.html` | G-10 |
| 79 | The issues drawer row's overflow: 26 px, never wraps; the message truncates with its full text in the tooltip; the glyph, class, entity and link never truncate. | §5.3, `specimen.html` | G-11 |
| 80 | The run log's glyphs are named from the marks vocabulary: Entered for a completed step, Issue warning for the stopped one, none for a step not run or running; the title line's type is stated. | §5.2, `specimen.html` | G-12 |
| 81 | The Type column offers the engine's element kinds only; the file's other kinds are listed as not offered. | §5.1 | R-4 |
| 82 | Canadian English re-checked across the new copy and the specimen; "Analyze" stays the product's spelling; registered sentences keep theirs. | §7.1, `specimen.html` | R-5 |
| 83 | `canvas.edgeAlt` added: a drawn line on a result-coloured element takes whichever of `canvas.edge` and `canvas.edgeAlt` has the higher contrast on the fill; the worst drawn line rises from 1.02:1 to 2.75:1 in light and from 1.10:1 to 2.90:1 in dark; six pairings added to the findings. | §2.2, §2.8, §2.9, §6.7, §8 items 6 and 13, `tools/palette.mjs`, `tools/contrast.mjs` | R-6 |
| 84 | The proposal card's consequence line no longer says results are cleared: "Accepting a row changes the model. Results from Run 03 stay readable, marked stale, until the next run." | §5.4, §8 item 17, `specimen.html` | MOCKS_V2 D2-1; decision 4 |
| 85 | The open items revised: 1, 2 and 3 closed, 6 to 9 updated, 12 to 18 added; the list of what the third mock pass must draw. | §8 | rulings 1 to 9; Q-15 to Q-22; R-6; R-9 |
| 86 | `tokens.json` is version 1.2: `canvas.edgeAlt`, the toast, HUD and run log values, `labels` and `agentCardClasses`; every colour value of 1.1 is unchanged. `tools/agree.mjs` checks the label table, the label chips, the agent card classes, the retired strings, the change log and the absence of machine paths; `tools/render.mjs` takes the Playwright location at run time. | `tokens.json`, `tools/` | brief DESIGN-SYSTEM-03 |
| 87 | The specimen is V1.2: label chips with their domains and the label table drawn from the tokens; the three run standings; hanger selection; the footer's selection group and edit chip; the toolbar's new controls; the toast, the issues row, the narrow canvas, the run log's glyphs, the slide-over geometry, the Review header and Kind menu, the five card classes, `canvas.edgeAlt` and the compass's buttons. | `specimen.html` | rulings 1 to 9; Q-15 to Q-22; G-7 to G-12; R-6 |
| 88 | Correction 1 to V1.2: after a model change the run stops being the current solve basis, so the stale band's closing clause, which said the run remained the basis until the next run, is removed everywhere. The band reads "Model changed since Run 03: *the change, with its time*. The values below are from the model as solved." with "Run again"; the compact form reads "Model changed since Run 03 · *change* · Run again". §8 item 17 is closed by ROOT and item 16 records that ROOT confirmed the Stale column. `tools/agree.mjs` fails if the removed clause returns. | §4, §5.1, §7.1, §8 items 16 and 17, `specimen.html`, `tools/agree.mjs` | ROOT's correction to V1.2 after its review; decision 4 as corrected |
| 89 | The owner records for rows 51 and 58, which those rows cite as relayed by ROOT: row 51, the maturity sentence's removal, is `../../../../_DECISIONS/D-71_RULING_ADDENDUM_2_2026-09-18.md`; row 58, the Checked mark as option A, is `../../../../_DECISIONS/D-71_RULING_ADDENDUM_2026-09-18.md`. They are cited here and not in those rows because rows 1 to 87 stand unchanged. | §9 rows 51 and 58 | ROOT's correction to V1.2 |
| 90 | Correction 2 to V1.2, two wording fixes. The pointer rule's heading and the sentence in §0 no longer present the general rule as the owner's amendment: the owner's words are attributed to Q-20, and the generalization to ROOT's reading as the D-71 ruling record states it, open to the owner's correction. The Checked mark's words lose the absolute "and no others": the ruled words are used exactly where they apply, and "Check rows", "Check again" and "Clear check" are shown to derive from the ruling's "set on one row or many" and its stale text. | §0, §5 intro, §7.3, §9 sources | ROOT's correction 2, from REVIEW-03; `../../../../_DECISIONS/D-71_RULING_2026-09-18.md`; `../../../../_DECISIONS/D-71_RULING_ADDENDUM_2026-09-18.md` |

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
