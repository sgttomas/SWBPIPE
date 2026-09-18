# Design system V1 — SWB Piping Designer

Status: V1, proposal for ROOT's acceptance. Written by the HELPS_HUMANS design manager under sealed brief DESIGN-SYSTEM-01 (phase 3 of the interface program), 2026-09-18. Basis, read in the brief's order: the design brief V1.4, the direction record, the concept directions V1 (§0, the shells, the three presentation languages and the disclosure homes), RESEARCH-E §3.2–3.4 and §6.1–6.2, RESEARCH-C §1.1, §3 and §4, RESEARCH-A's preserve list and RESEARCH-D §2 and §6. No product stylesheet or component was read; nothing outside `instances/DESIGN-SYSTEM/` was written.

Companions: [`tokens.json`](tokens.json) holds every token in this document (colour as light and dark values; type, spacing, radii, borders, elevation, focus, motion, layout and icon values as plain values). [`specimen.html`](specimen.html) renders them offline in both themes. [`RETURN.md`](RETURN.md) records what was read, how agreement between the three files was checked, and what is uncertain.

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
| Both view | table 55% / canvas 45% of the surface width (737 / 603 at 1440 with the strip), resizable, remembered | The inspector is a 300 px slide-over on the canvas's right edge (⌘I, double-click, Escape); it never reflows the table. |
| Minimum window | 1280 × 800 | Surfaces 1180 × 728 with the strip. |

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
| `type.size.caption` | 11 / 14 | 400 | Captions only: canvas HUD tick labels, the "sample" label, axis names on the compass. Never a value, never a control label. |
| `type.size.chip` | 12 / 16 | 500 | Chip labels, keyboard hints, footer counts, column-header units when the header wraps. |
| `type.size.body` | 13 / 18 | 400 | Everything read repeatedly: cells, column headers (500), controls, inspector rows, issues, conversation, proposal diffs, report text. **13 px is the smallest size for anything read repeatedly.** |
| `type.size.title` | 15 / 20 | 600 | Panel and section titles, dialog titles, the proposal card title. |
| `type.size.heading` | 18 / 24 | 600 | The Review page title, About. |
| `type.size.figure` | 22 / 28 | 500, letter-spacing −0.2 | One hero figure per surface at most: the Summary's governing ratio with its rule and pack beside it. |

Rules. No uppercase grey headings; titles are sentence case at 15/600. Numbers are right-aligned in cells with a fixed number of decimals per column (lengths 0 or 1 decimal in mm, stresses 1 decimal in MPa, ratios 2 decimals, temperatures 0 decimals, pressures 2 decimals; the column's unit and precision are one setting), a true minus sign (U+2212), and no thousands separators unless the display preference asks for thin-space groups. Units live in column headers in square brackets (`Stress [MPa]`), after a value with a thin space in the inspector, probe and legend (`124.6 MPa`), and never inside a cell. Text never wears a series or scale colour: values and labels stay in the text tokens and a mark beside them carries the colour.

### 1.2 Spacing, radii, borders, elevation

Spacing is a 4 px system: `space.0` 0, `.1` 2, `.2` 4, `.3` 6, `.4` 8, `.5` 12, `.6` 16, `.7` 20, `.8` 24, `.9` 32, `.10` 40. Cell padding 0 8; control padding 0 8; panel padding 12; dialog padding 20; 16 px side gutter for any page-like surface (Review, About, Preferences). Radii: `radius.control` 3 (inputs, buttons, chips, menu items), `radius.card` 6 (proposal cards, probe card, legend, popovers), `radius.sheet` 10 (dialogs and sheets), `radius.pill` reserved for the count badge on the agent strip and the rail. Borders are 1 px hairlines (`border.hairline`) between regions and rows; `border.strong` frames inputs, the table and the drawers; nothing decorative is thicker than 1 px. Elevation: level 0 flat for panels and tables, level 1 for a drawer edge and a hovered card, level 2 for popovers, the probe card and the legend, level 3 for dialogs; in dark the shadows are deeper and each raised surface also carries a 1 px inner light ring (`elevation.*.dark`), because a dark surface cannot be raised by shadow alone.

### 1.3 Focus, selection, hover, disabled

Focus is a 2 px ring in `focus.ring`, offset 1 px outside controls and inset 2 px inside table cells so it is never clipped; a focused cell also rises above its neighbours. Selection is a fill (`selection.band`) with a 3 px bar (`selection.bar`) on the left edge of the row; in the canvas it is a halo. Focus and selection share the platform's accent hue and differ in form; they coexist (a focused cell inside a selected row shows both). Hover is a wash (`hover.wash`) over the row or control, pressed a stronger wash (`pressed.wash`). Disabled controls sit on `disabled.fill` with `text.disabled`, keep their label, and carry the reason in a tooltip (the Run button says why it is disabled). The accent hue is spent on interaction only: selection, focus, links, the one primary action per surface, and the engineer's own draft ghost in the canvas. Nothing else is blue.

### 1.4 Motion

Hover 80 ms; disclosures and popovers 120 ms; drawers, the agent column and the inspector slide-over 180 ms; a proposal arriving 240 ms (the row band fades in and the diff card rises 4 px into place). Easing `cubic-bezier(0.2, 0, 0, 1)` in, `cubic-bezier(0.4, 0, 1, 1)` out. Nothing bounces, pulses or shimmers. Under `prefers-reduced-motion` every duration is 0. The canvas's own camera easing belongs to the engine and is not specified here.

### 1.5 Density

One density, tuned for tables of numbers: row 26 px, header row 28 px, control 26 px (22 px compact in the HUD, chips and the status bar), rail item 44 px, toolbar 48 px, status bar 24 px, gutter column 32 px, marks column 72 px. Icons draw at 16 px inside 26 px controls, 20 px in the rail, 12 px in the gutter, marks column and chips. Column widths by default (resizable): gutter 32, Node 56, From 56, Type 100, DX DY DZ 72 each, Section 72, Material 72, Load 64, each T and P read-through 56, marks 72. It is the density of a CAE tool: no stacked label-over-input forms, no card padding around a number, no 30 px web controls.

## 2. Colour

### 2.1 The hue budget

Each hue has one job and is spent nowhere else. This is the rule that keeps "colour as approval" out of the product (RESEARCH-D §6) and keeps the loud thing loud.

| Hue | Job | Tokens |
|---|---|---|
| Blue (accent) | Interaction: selection, focus, links, the primary action, the engineer's draft ghost, the Z axis of the triad | `accent.*`, `selection.*`, `focus.ring`, `border.focus`, `text.link`, `canvas.selection`, `canvas.hover`, `canvas.draft` |
| Violet | Proposals, and nothing else: the proposed row band and bar, new values, the ghost in the canvas | `proposal.*`, `canvas.proposalGhost` |
| Red | Blocking issues and the required mark (model validity, solve, rule check), the X axis of the triad | `issue.blocking*`, `mark.required`, `status.failed*` |
| Amber | Warnings and staleness: provenance, assumption and nonlinear warnings, the stale Checked mark, the incomplete statuses | `issue.warning*`, `mark.checkedStale`, `status.incomplete*` |
| Blue-grey | Informational issues and the review-required status | `issue.info*`, `status.review*` |
| Teal | The result scale, only, on the tube and in the legend and ratio bars | `result.scale.1–7` |
| Warm neutral | Historical (M-13) | `historical.*` |
| Neutral ink | Origins, Checked, attachment marks, glyphs, vectors when result colour is on, the six statuses that are neither failed nor incomplete | `mark.origin`, `mark.checked`, `mark.attachment`, `status.solved*`, `canvas.glyph`, `canvas.vector` |
| Categorical 1–8 | Load kinds and cases, only | `cat.1–8` |
| Green | The Y axis of the triad, only. Green marks nothing as good, ready, solved or checked. | `canvas.axisY` |

### 2.2 Tokens

Every colour token, light and dark. The tables below are generated from `tokens.json`; the hex values are the file's.

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

**canvas**

| Token | CSS variable | Light | Dark |
|---|---|---|---|
| `canvas.bg` | `--canvas-bg` | `#ebedef` | `#191c1f` |
| `canvas.gridMajor` | `--canvas-gridMajor` | `#ced1d5` | `#2d3135` |
| `canvas.gridMinor` | `--canvas-gridMinor` | `#dfe1e4` | `#222629` |
| `canvas.pipe` | `--canvas-pipe` | `#a6abb1` | `#757b81` |
| `canvas.pipeShade` | `--canvas-pipeShade` | `#81878d` | `#53595f` |
| `canvas.edge` | `--canvas-edge` | `#42484f` | `#c7cbd0` |
| `canvas.glyph` | `--canvas-glyph` | `#30363c` | `#dbdee2` |
| `canvas.glyphFill` | `--canvas-glyphFill` | `#dbdee2` | `#34383d` |
| `canvas.label` | `--canvas-label` | `#22272c` | `#e5e8ec` |
| `canvas.labelBg` | `--canvas-labelBg` | `rgba(255,255,255,0.82)` | `rgba(28,31,37,0.82)` |
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

**result**

| Token | CSS variable | Light | Dark |
|---|---|---|---|
| `result.scale.1` | `--result-scale-1` | `#4ab9b2` | `#02736e` |
| `result.scale.2` | `--result-scale-2` | `#15a6a0` | `#028984` |
| `result.scale.3` | `--result-scale-3` | `#05908b` | `#04a19b` |
| `result.scale.4` | `--result-scale-4` | `#087a76` | `#28b8b1` |
| `result.scale.5` | `--result-scale-5` | `#056662` | `#51cec7` |
| `result.scale.6` | `--result-scale-6` | `#01524e` | `#83e1db` |
| `result.scale.7` | `--result-scale-7` | `#003e3c` | `#b7f2ed` |

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

Notes on the sets. Surfaces step by lightness, not by colour: `surface.base` behind everything, `surface.panel` for tables and panels, `surface.sunken` for inputs, read-through cells and wells, `surface.header` for column headers, `surface.rowAlt` for the optional zebra (on by default on tables wider than eight columns), `surface.raised` for popovers and cards; `surface.canvas` equals `canvas.bg`. The four `status.*` pairs serve the six automatic statuses (M-08) as described in §2.3. `mark.required` equals `issue.blocking` by design (a missing required value is a blocking finding, C-28) and is kept as its own token so the mark can be retuned without touching the issue colour. `proposal.old` equals `text.muted` by design: an old value is muted and struck, never coloured. `canvas.unsolved` equals `canvas.pipe`: an unsolved element is the neutral pipe, and in a coloured view it is also drawn at 60% opacity so that form, not only hue, separates it from a low-ratio element (§6.7).

### 2.3 The six automatic statuses as chips (M-08)

The status bar chip shows a short label; the raw token, its authority domain and the run identity are one click away in the chip's popover. Short labels are decision-packet item 4 (whether they are listed variants of the registered texts); until ruled, the mapping below is the design's proposal and the raw token is always reachable.

| Raw token | Short label | Chip tokens | Authority domain in the popover |
|---|---|---|---|
| `MODEL_INCOMPLETE` | Model incomplete | `status.incompleteFill` / `status.incompleteInk` | Solver |
| `MECHANICS_SOLVED` | Mechanics solved | `status.solvedFill` / `status.solvedInk` | Solver |
| `RULE_INPUTS_INCOMPLETE` | Rule inputs incomplete | `status.incompleteFill` / `status.incompleteInk` | Rule pack |
| `USER_RULE_CHECKED` | User rules checked | `status.solvedFill` / `status.solvedInk` | Rule pack |
| `USER_RULE_FAILED` | User rule failed | `status.failedFill` / `status.failedInk` | Rule pack |
| `HUMAN_REVIEW_REQUIRED` | Human review required | `status.reviewFill` / `status.reviewInk` | Human |

Chip anatomy: 20 px tall, `radius.control`, 12 px medium label, no icon, no border. "Mechanics solved" and "User rules checked" are neutral on purpose: a solved model is information, not a green light. The popover: the raw token in `type.family.mono`, the authority domain, the run name and time, and the historical caption when the run is a reopened saved run (M-13).

The two evidence labels (M-09) follow the same pattern beside the run name in a results header: `INTERNALLY_VERIFIED` as "Internally verified" and `PROVER_CORRELATED` as "Prover correlated", on `status.solvedFill` / `status.solvedInk`, raw token in the tooltip; `ENGINEER_ACCEPTED` is never emitted.

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

Origins and attachment marks are neutral ink (`mark.origin`, `mark.attachment`): quiet. Proposals are violet (`proposal.band`, `proposal.bar`, `proposal.new`; old values `proposal.old` struck): loud. Checked is neutral ink (`mark.checked`), a human tag and not a colour; stale Checked is amber (`mark.checkedStale`) with a dashed glyph. Historical is a warm neutral band (`historical.tint` / `historical.ink`) with a hatch, never a colour that reads as status. Display-only (M-15) is form, not hue: monospace on `surface.sunken` inside a dashed `displayOnly.border` with the caption "Display only, not accepted as input" in `text.muted`.

### 2.6 The result colour scale

One hue, teal, light to dark, seven anchors `result.scale.1` to `.7` with continuous interpolation in OKLCH between them. In light the near-zero end is the lightest step and the maximum the darkest; in dark the anchor flips so that the near-zero end is the darkest teal, no darker than the pipe neutral, and the maximum the brightest. Either way "more" is further from the pipe neutral and the geometry stays readable. The ramp was built and checked with the dataviz method's ordinal validator in both themes against `canvas.bg`: monotone lightness, adjacent steps at least 0.06 apart in OKLCH L, the near-zero step at 2.02:1 (light) and 3.00:1 (dark) against the canvas, single hue.

Legend semantics, the same for ratio and for stress:

- The legend is a scale, never a verdict. It is a vertical bar with five ticks, the quantity and unit (`Ratio` or `Stress [MPa]`), the rule ID and pack version when the quantity is a ratio (`Rule EXP-A1 · pack 1.2`), and the case name or `Envelope`.
- For ratio the default range is 0 to the greater of 1.0 and the maximum in the run; 1.0 is a labelled tick and nothing changes colour at it. For stress the range is 0 to the maximum. The engineer can set the range; the legend then says `range set`.
- Nothing is coloured that has not been solved: unsolved elements draw in `canvas.unsolved` at 60% opacity and the legend carries an `Unsolved` swatch.
- The probed or selected element's value is a marker on the bar.
- The legend and the ratio cells are the only places the scale appears. In tables, a ratio cell carries a small inline bar (data bar) in the scale colour beside the number; the number stays in `text.primary`.

### 2.7 Load-vector and case colours

Eight categorical slots in a fixed order, assigned in sequence and never cycled, validated with the dataviz validator (`scripts/validate_palette.js`) against `canvas.bg` in both themes. Order: 1 orange, 2 magenta, 3 green, 4 purple, 5 rose, 6 azure, 7 olive, 8 cyan. The order was chosen by enumerating every ordering with orange first and keeping the ones that clear both hard gates in both themes, then taking the best minimum CVD separation.

Validator findings (adjacent pairlist, the default for vectors drawn beside one another): light, all eight inside the band, chroma above the floor, worst adjacent CVD ΔE 10.4 (deutan), worst adjacent normal-vision ΔE 16.9, all eight at or above 3:1 on the light canvas; dark, worst adjacent CVD ΔE 10.2 (protan), normal-vision ΔE 16.9, all eight at or above 3:1 on the dark canvas. All-pairs (any two vectors can be neighbours in a canvas): the first three slots pass in both themes (worst 8.1 light, 6.1 dark in the floor band); the first six do not (rose against green and magenta collapse under deutan). The consequence is a rule, not a colour: colour is never the only carrier of a load's kind. Kind is carried by the arrow's form (§6.4), by the label on hover and selection, and by the legend row that pairs glyph, colour and name.

Assignment: load kinds take slots 1 to 6 in fixed order: force → `cat.1`, moment → `cat.2`, specified displacement → `cat.3`, uniform and weight → `cat.4`, wind → `cat.5`, seismic → `cat.6`. Cases take slots by row order in the load-case table; a ninth and later case takes `canvas.vector` neutral with its label. When a case is selected on the Loads page, its loads draw in the case's slot and every other load draws in `canvas.vector` at 40% opacity. When result colour is on the tube, vectors draw in `canvas.vector` only: one colour job per view.

### 2.8 The canvas palette

Both themes carry the same roles: `canvas.bg`; `canvas.gridMajor` and `canvas.gridMinor` for the ground grid; `canvas.pipe` and `canvas.pipeShade` for the tube and its shaded side; `canvas.edge` for the edge line; `canvas.glyph` and `canvas.glyphFill` for restraint glyphs; `canvas.label` on `canvas.labelBg` for node labels; `canvas.vector`; `canvas.selection` and `canvas.hover` halos; `canvas.draft` for the engineer's routing ghost; `canvas.proposalGhost`; `canvas.deformGhost`; `canvas.unsolved`; `canvas.axisX/Y/Z` for the triad and the compass. In light the drawing is a mid grey tube with a dark edge on a pale ground; in dark a lighter grey tube with a light edge on a near-black ground. Neither theme is a flip of the other: each is stepped so the edge reads against the tube and the tube against the ground (findings in §2.9).

### 2.9 Contrast findings

Method. For every text and mark pairing in the system the WCAG 2.x contrast ratio (relative luminance, sRGB) was computed from `tokens.json` for both themes; translucent tokens are composited over the surface named after the slash before the ratio is taken. The table is generated by a script from the token file; the specimen's Preferences panel computes the same ratios live from the same tokens, so the two agree by construction. These are findings for the owner's target, which is TBD (M-16, C-91 to C-94): no conformance is claimed and no target is named here.

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
| `text.primary` | `disabled.fill` | text | 12.61:1 | 11.10:1 |
| `text.secondary` | `surface.panel` | text | 6.26:1 | 6.91:1 |
| `text.secondary` | `surface.base` | text | 5.48:1 | 7.81:1 |
| `text.secondary` | `surface.header` | text | 5.62:1 | 6.48:1 |
| `text.secondary` | `surface.sunken` | text | 5.83:1 | 7.37:1 |
| `text.secondary` | `selection.band` | text | 5.33:1 | 5.08:1 |
| `text.secondary` | `proposal.band` | text | 5.46:1 | 5.73:1 |
| `text.muted` | `surface.panel` | text | 3.79:1 | 4.41:1 |
| `text.muted` | `surface.base` | text | 3.32:1 | 4.99:1 |
| `text.muted` | `surface.header` | text | 3.40:1 | 4.14:1 |
| `text.muted` | `surface.sunken` | text | 3.53:1 | 4.70:1 |
| `text.disabled` | `surface.panel` | text (disabled) | 2.31:1 | 2.48:1 |
| `text.disabled` | `surface.base` | text (disabled) | 2.03:1 | 2.80:1 |
| `text.link` | `surface.panel` | text | 6.07:1 | 7.38:1 |
| `text.link` | `surface.base` | text | 5.32:1 | 8.34:1 |
| `accent.text` | `surface.panel` | text | 6.67:1 | 7.95:1 |
| `accent.text` | `selection.band` | text | 5.69:1 | 5.84:1 |
| `text.inverse` | `accent.fill` | text on button | 5.12:1 | 5.84:1 |
| `text.inverse` | `accent.fillHover` | text on button | 6.09:1 | 6.76:1 |
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
| `historical.ink` | `historical.tint` | text | 6.49:1 | 7.78:1 |
| `historical.ink` | `surface.panel` | text | 7.43:1 | 8.43:1 |
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
| `result.scale.1` | `canvas.bg` | result scale on canvas | 2.02:1 | 3.00:1 |
| `result.scale.2` | `canvas.bg` | result scale on canvas | 2.56:1 | 4.00:1 |
| `result.scale.3` | `canvas.bg` | result scale on canvas | 3.33:1 | 5.36:1 |
| `result.scale.4` | `canvas.bg` | result scale on canvas | 4.41:1 | 6.99:1 |
| `result.scale.5` | `canvas.bg` | result scale on canvas | 5.80:1 | 8.97:1 |
| `result.scale.6` | `canvas.bg` | result scale on canvas | 7.72:1 | 11.21:1 |
| `result.scale.7` | `canvas.bg` | result scale on canvas | 10.20:1 | 13.80:1 |
| `result.scale.1` | `canvas.unsolved` | result scale beside unsolved | 1.02:1 | 1.33:1 |
| `result.scale.7` | `canvas.unsolved` | result scale beside unsolved | 5.18:1 | 3.45:1 |
| `result.scale.1` | `surface.raised` | legend swatch | 2.37:1 | 2.26:1 |
| `result.scale.2` | `surface.raised` | legend swatch | 3.00:1 | 3.02:1 |
| `result.scale.3` | `surface.raised` | legend swatch | 3.91:1 | 4.05:1 |
| `result.scale.4` | `surface.raised` | legend swatch | 5.18:1 | 5.28:1 |
| `result.scale.5` | `surface.raised` | legend swatch | 6.81:1 | 6.77:1 |
| `result.scale.6` | `surface.raised` | legend swatch | 9.06:1 | 8.46:1 |
| `result.scale.7` | `surface.raised` | legend swatch | 11.97:1 | 10.42:1 |
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

Reading the findings.

- Body text: `text.primary` on every surface sits between 8.9:1 and 15.1:1 across both themes; `text.secondary` between 5.1:1 and 7.8:1; `text.muted` between 3.3:1 and 5.0:1. Muted ink is used for origins, captions and the struck old value, never for a value the engineer reads.
- `text.disabled` measures 2.0:1 to 2.8:1. That is the disabled state; the label stays legible enough to be found and the reason is in a tooltip. If the owner's target requires more, `text.disabled` is the one token to lift.
- Chips: every status ink on its fill is above 6.4:1 in both themes; the warning ink on its tint is the lowest coloured text at 3.91:1 (light).
- Marks: origins at 3.4:1 to 4.4:1, attachments and Checked at 6.3:1 to 6.9:1, stale Checked at 3.8:1 (light, on base) to 8.9:1, required and blocking above 5.2:1, the focus ring at 4.0:1 to 7.2:1.
- Hairlines, grid lines and the stale hatch are deliberately recessive (1.1:1 to 1.6:1); they separate, they do not inform. `border.strong`, which frames inputs and the table, measures 1.8:1 to 2.4:1.
- Canvas: the light pipe neutral is 1.97:1 against the light ground, by design a pale drawing; its edge line reads at 7.9:1 against the ground and 4.0:1 against the tube, so the geometry is carried by the edge, and the dark tube reads at 4.0:1. The selection halo is drawn against the ground (4.4:1 light, 7.4:1 dark), not against the tube. The hover halo is 2.7:1 in light. Glyph ink is above 10:1 against the ground in both themes.
- Result scale: on the canvas the steps run from 2.0:1 to 10.2:1 (light) and 3.0:1 to 13.8:1 (dark); in the legend card from 2.4:1 to 12:1. The near-zero step beside an unsolved element is 1.0:1 in light, which is why unsolved elements are also dimmed (§2.6).
- Categorical: every slot is at or above 3.1:1 on the canvas and 3.6:1 as a dot on a panel in both themes.

## 3. Iconography

### 3.1 Style rules

A 16 px grid with a 1 px safe margin; 1.5 px strokes with round caps and joins; corners at 1.5 px radius; `currentColor` so the icon takes the text token of its context; no fills except the selected rail item, whose glyph fills at the same stroke; optical sizes 12 (gutter, marks, chips; 1.25 px stroke), 16 (controls, menus, HUD) and 20 (rail). No shields, no ticks in circles, no traffic-light discs: nothing that reads as approval. Every icon has a visible text label or a tooltip with the same words as its menu item (C-97); the rail shows labels under its icons, the HUD shows tooltips with the accelerator.

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
| HUD | Fit | four corner brackets | Fit (F) |
| HUD | View presets | a cube with one face marked; menu Iso, Top, Front, Right, Report figure | View |
| HUD | Section | a plane cutting a cube | Section |
| HUD | Isolate | one solid square among dashed ones | Isolate selection (I) |
| HUD | Hide | an eye with a slash | Hide selection (H) |
| HUD | Labels | a tag reading 10 | Node labels (L) |
| HUD | Deform | a bent line beside a straight ghost | Deformation (D) |
| HUD | Probe | a crosshair with a centre dot | Probe (P) |
| HUD | Route | a polyline ending in an arrowhead | Route (R) |
| HUD | Restrain | the +Y triangle under a line | Add restraint (S) |
| Table | Add row | a plus in a row outline | Add row (⌥↩) |
| Table | Delete rows | a minus in a row outline | Delete rows (⌫) |
| Table | Sort | two small arrows | Sort |
| Table | Filter | a funnel | Filter |
| Table | Columns | three vertical bars | Columns |
| Table | Paste | a clipboard over a grid | Paste with mapping (⌘V) |
| Table | Expand | a chevron, right closed, down open | Open joined row (⌘↩) |
| Table | Changes since | a clock with a returning arrow | Changed since… |
| Table | Unchecked | an empty check box | Unchecked rows |
| Table | Origins | a small dot with rays | Show origins |
| Marks | see §4 | | |

## 4. The marks vocabulary

The rule from the direction record §10: origins quiet, proposals loud. Origins are a 12 px neutral glyph in the gutter; proposals are a band, a bar, a diamond and a coloured new value. Everything else sits between.

Placements. The **row gutter** is 32 px wide with two 12 px slots: the origin slot on the left, the state slot on the right. The **marks column** is the last column, 72 px, with three fixed 20 px slots: restraint, load, node data. A **cell corner** mark is a 5 px triangle in the top-right corner of a cell. A **row band** is a fill across the row with a 3 px bar on its left edge. Tooltips appear on hover after 300 ms and on focus with Space. Keyboard reveal, the same for every mark: `←` from the Node cell focuses the gutter, `→` from the last column focuses the marks column, Space on a focused gutter or marks slot opens that mark's popover with its tooltip text, and `⌥I` on any row opens the row's marks popover listing every mark the row carries with its tooltip. Filters live in the footer's Filter control and in the rail's Issues item.

| Mark | Glyph (12 px) | Token | Placement | Tooltip | Keyboard reveal | Filter |
|---|---|---|---|---|---|---|
| Entered | nothing; a dot when origins are shown | `mark.origin` | origin slot | Entered by *name* · *date time* | Space on the slot; ⌥I | Origins: Entered |
| Accepted from a proposal | diamond outline with a centre dot | `mark.origin` | origin slot | Accepted from proposal *P-12* by *name* · *date time* · rationale on the record | Space; ⌥I | Origins: Accepted |
| Propagated | an arrow onto a bar | `mark.origin` | origin slot; and cell corner on each propagated cell | Propagated from node *10* (*Section*, *Material*, *Load*) · typing over makes it entered | Space; ⌥I; Space on the cell | Origins: Propagated |
| Generated | a four-point spark | `mark.origin` | origin slot | Generated by rule pack *name* *version* · *date time* | Space; ⌥I | Origins: Generated |
| Imported | a tray with a down arrow | `mark.origin` | origin slot | Imported from *file* · *date time* · provenance in Libraries | Space; ⌥I | Origins: Imported |
| Proposed row | filled diamond | `proposal.new`; band `proposal.band`; bar `proposal.bar` | origin slot while pending; row band | Proposed by the agent · *P-12* · Accept ⌘⇧A · Reject ⌘⇧R · old and new values in the cells | Space; ⌥I | Proposed |
| Proposed cell | new value in `proposal.new` with the corner tick; the struck old value in `proposal.old` before it only when the column has room for both, otherwise in the tooltip and on the card | as above | inside the cell; cell corner | *field*: *old* → *new* *unit* · Accept this row | Space on the cell | Proposed |
| Checked | a check | `mark.checked` | state slot | Checked by *name* · *date time* · bound to this row's content · not a software status | Space; ⌥I | Unchecked (inverse) |
| Checked, stale | a dashed check with a small dot | `mark.checkedStale` | state slot | Checked by *name* · *date time* · the row changed since · check again or clear | Space; ⌥I | Unchecked; Stale |
| Restraint | the +Y triangle under a line | `mark.attachment` | marks column, slot 1 | Restraint · *+Y, gap 3 mm, μ 0.3* · ⌘↩ opens the row | ↩ on the slot; ⌘↩ | Has restraint |
| Load | an arrow onto a dot | `mark.attachment` | marks column, slot 2 | Load · *Force −2000 N in Y (W)* · ⌘↩ opens the row | ↩ on the slot; ⌘↩ | Has load |
| Node data | a line with a short branch stub | `mark.attachment` | marks column, slot 3 | Node data · *Welding tee; flange WN* · ⌘↩ opens the row | ↩ on the slot; ⌘↩ | Has node data |
| Issue, blocking | octagon | `issue.blocking` | state slot; cell ring on the offending cell | *Blocks solve* · *Section missing* · Show in Issues | Space; ⌥I; ⌘⇧I opens Issues | Issues: class |
| Issue, warning | triangle | `issue.warning` | state slot; cell corner | *Provenance* · *Material source not recorded* | Space; ⌥I | Issues: class |
| Issue, note | circle with i | `issue.info` | state slot | *Note* · *Assumption recorded for review* | Space; ⌥I | Issues: class |
| Required | asterisk | `mark.required` | inside the empty cell, left | Required to solve · *Section* | Space on the cell | Issues: Blocks solve |
| Historical run (M-13) | a clock | `historical.ink` on `historical.tint` | band across the results header; caption on the rail | Historical saved run · reopening cannot establish a current solve basis | Space on the band | Results: Historical |
| Display-only expression (M-15) | lines in a dashed frame | `displayOnly.border` | the rendered-expression cell or caption | Display only, not accepted as input · notation not frozen | Space on the cell | none |

Priority when a slot is contested. Origin slot: proposed wins while a proposal is pending; otherwise the origin. State slot: blocking, then warning, then stale Checked, then Checked, then note; the ⌥I popover always lists everything. Row band: the selection band paints over the proposal band, and the proposal bar and diamond remain, so a selected proposed row is unmistakably both.

## 5. Components

### 5.1 The table

One table component serves every table in the product: the layout table, the attachment tables, load cases and load sets, every results table, hangers, and the live tables on the Review page. What follows is its full specification; the layout table is the reference instance.

**Header.** 28 px, `surface.header`, 13 px medium, units in square brackets after the name (M-11): `DX [mm]`, `T1 [°C]`, `P1 [bar]`, `Stress [MPa]`. A header cell carries the sort indicator (two small arrows, the active direction filled) and, on hover, the column menu (sort, filter, precision, unit, hide, pin). Sorting never reorders the layout table's rows on disk: the table is the model and the row order is the file order; sort is a view, indicated by a `sorted` chip in the footer, and Enter-down still follows the file order. Filtering hides rows and the footer says how many.

**Columns of the layout table, in the CAEPIPE grammar** (RESEARCH-E §3.2, §6.1), left to right:

| Column | Content | Notes |
|---|---|---|
| gutter | origin and state slots | §4 |
| Node | the node number | numeric, auto-increment 10 on a new row, editable; insert and split preserve neighbours |
| From | the node the element comes from | shown on every row; drawn in `text.muted` when it is the previous row's node (implied), in `text.primary` when it names an earlier node (a branch) or was typed |
| Type | the element arriving at the node | Pipe (blank in the file, shown as "Pipe"), Bend, Valve, Reducer, Rigid, Expansion joint, Slip joint, Ball joint, Hinge, Tie rod, Elastic element, Beam, Cut pipe, Jacketed pipe, Miter bend: the kinds the target file treats as elements; a tee is never here |
| DX DY DZ, or X Y Z | offsets from From, or absolute coordinates | one switchable column group; the group's header shows which; a row whose coordinates are absolute in the file (the `*` form) shows a corner tick |
| Section | the section name | propagates down until changed; propagated cells marked |
| Material | the material name | propagates; marked |
| Load | the load set name | propagates; marked |
| T1 … Tn, P1 … Pn | the set's temperatures and pressures read through | `surface.sunken` cells with a corner tick; typing over one edits the set or forks a new one, offered as a choice in a small popover |
| marks | restraint, load, node data | §4 |

Element fields beyond these (bend radius, valve weight, reducer ends, expansion joint stiffnesses) live in the row's expansion (⌘↩ on the Type cell) and in the inspector, not as columns. The read-through group is on by default in Table view and off in Both view and in the Model-view drawer, where the Load cell's tooltip shows the values.

**Cell states.**

| State | Appearance |
|---|---|
| Entered | `text.primary` on the row surface; nothing else |
| Propagated | as entered, with the origin corner tick in `mark.origin`; tooltip names the source node |
| Read-through | on `surface.sunken` with the corner tick; the value belongs to the named load set |
| Proposed | the new value in `proposal.new` medium, preceded by the struck old value in `proposal.old` when the column has room for both (otherwise the old value is in the tooltip and on the card); the row carries the band and bar |
| Selected | `selection.band` across the row with the 3 px `selection.bar`; a selected cell inside a selected row adds a 1 px `selection.bar` outline |
| Editing | the cell becomes an input on `surface.panel` with the 2 px inset `focus.ring`; the text is selected on entry; the unit shows after the caret in `text.muted` |
| Invalid | a 2 px inset ring in `issue.blocking` and the message in a popover under the cell; the value stays as typed until corrected or escaped |
| Required, empty | the asterisk in `mark.required`; the gutter's state slot shows the octagon |
| Stale | a `stale.stripe` diagonal hatch over the cell (a result cell whose model has changed, a Checked row's content) |
| Historical | the results header band; cells unchanged |
| Disabled | `text.disabled`; never used in the layout table |

**Row gutter and marks column.** As in §4. The gutter also carries the row selector: clicking selects the row, ⇧-click extends, ⌘-click toggles. The marks column's empty slots show a faint plus on hover ("Add restraint…"), which opens the joined row empty.

**The paste band.** When a multi-cell paste arrives (⌘V with more than one row or a header row), a band appears above the affected rows on `surface.raised` at elevation 1: a mapping row in which each source column names its target column in a combobox (recognised names pre-mapped: From, To, Node, DX, DY, DZ, Section, Material, Load, T1…, P1…), unmapped columns offered "Ignore" or a target, a count ("42 rows, 2 columns ignored"), a preview of the first three rows in the target grammar, and two buttons: Paste (↩) and Cancel (⎋). The paste commits as one operation with one undo checkpoint; the rows it creates carry the entered origin, and node IDs are taken from the source when present and auto-incremented when not.

**Keyboard model.** The spreadsheet idiom, fully operable without the mouse (brief §2, C-96).

| Key | Not editing | Editing |
|---|---|---|
| Type a character | starts editing, replacing the value | inserts |
| ↩ | starts editing the focused cell | commits and moves down one row (⇧↩ up) |
| ⇥ / ⇧⇥ | moves right / left; wraps to the next row's first editable cell | commits and moves right / left |
| ← → ↑ ↓ | moves the focus one cell | ← → move the caret; ↑ ↓ commit and move |
| ⎋ | clears the multi-row selection, then closes the open drawer or popover: one level per press | cancels the edit and restores the value |
| ⌘↩ | opens the joined row for the focused row (on a marks slot, that mark's row; on the Type cell, the element fields) | commits, then opens |
| ⌥↩ | inserts a row below with the next node number | commits, then inserts |
| ⇧↑ ⇧↓, ⇧-click | extends the row selection | — |
| ⌘A | selects all rows | selects the text |
| ⌘C ⌘V | copies the selection as tab-separated text with headers; pastes with the band when multi-row | text |
| ⌫ | on a row selection: deletes the rows, with an undoable toast | deletes text |
| Space | on a gutter or marks slot: opens the mark's popover; on a check-box cell: toggles | inserts |
| ⌥I | opens the row's marks popover | — |
| ⌘K | the command palette | — |
| ⌘Z ⇧⌘Z | undo, redo: one stack for the whole product | text |
| ⌘⇧A ⌘⇧R | Accept, Reject the focused proposed row | — |
| ⌘⇧K | marks the selected rows Checked (again ⌘⇧K clears) | — |

**Footer.** 26 px on `surface.header`: the row count and selected count ("128 rows · 3 selected"), then counts that are filters when clicked: propagated, proposed, unchecked, issues on this table; then the active filter chips with a clear control; then the Changed since… control (last run, last Checked, a named snapshot, a time). Sorted and filtered states show as chips here.

**Row expansion for a joined attachment row (Table view).** ⌘↩ or a click on a marks slot expands a block under the node row, indented by the gutter width, with a 2 px `border.strong` left edge: the attachment table's own header row and the node's rows in that table, editable in place with the same keyboard model, and an "Add" row. Escape or ⌘↩ closes it. In Model and Both views the same key opens the row in the inspector instead.

**Results tables.** The header band above the columns carries the run name and its immutability ("Run 03 · immutable"), the case selector (a combobox of the run's cases) with the Envelope toggle (a switch labelled Envelope; when on, the Case column shows the governing case per row), the evidence label chip, the historical band when the run is reopened, and the information disclosure that holds the acceptance sentence once for the results surface class (§7.4). The stress table's columns are Node, Element (From–To), Case, Stress [MPa], Allowable [MPa], Ratio (number with its data bar), Rule, Pack; sorted by ratio descending by default. Every results table can be copied and exported (C-98).

### 5.2 The stage rail, the view switch, the toolbar band, the status bar

**Stage rail.** 56 px, `surface.base`, items 44 px tall with a 20 px icon over an 11 px label: Model, Loads, Results, Review; a hairline; Libraries, Rules, Issues at the foot. The selected item fills its glyph and draws a 3 px `selection.bar` on the rail's left edge; items that cannot yet be used (Results before a run) are `text.disabled` with a tooltip that says why. Counts sit as small badges at the item's top right: Issues carries the issue count, Results carries "Historical" as a caption under the label when a saved run is reopened (M-13), and a proposals count sits on the agent strip, not on the rail. The rail is navigation, never modes: switching stages keeps the selection, the undo stack and the issues list.

**View switch.** A three-segment control in the toolbar: Table, Model, Both, icons with labels, 26 px tall, the active segment on `surface.panel` with `text.primary`, the others `text.secondary`; ⌘1 ⌘2 ⌘3. The choice is remembered per stage.

**Toolbar band.** 48 px on `surface.base` with a hairline below. Left to right: the wordmark (SWBPIPE in 13 px medium, tracking 0.04 em, `text.secondary`; the long name SWB Piping Designer is in the window title and About), the project name with its save state ("Loop 4 header · saved", or "· edited" in `text.secondary`), the view switch, then centred the Run button (the surface's primary action, `accent.fill` with `text.inverse`; while running it shows a progress bar inside the button with the stage name in its tooltip and a stop control; when disabled it stays visible on `disabled.fill` and its tooltip names the reason: "Section missing at node 20, Material missing at node 20"), the Issues count as a button with the octagon or triangle of the worst class, the Agent toggle (⌘⇧G) that opens the column, the display units selector (a compact combobox "SI" / "US"; a display toggle only, C-40), and the palette search field (⌘K, 220 px, placeholder "Search or command…").

**Status bar.** 24 px on `surface.base` with a hairline above, 12 px medium: at the left the M-08 status chip with its popover (§2.3); then the issues count, which opens the drawer; then the selection ("Node 40 · 1 row"); at the right the display units and an information control whose popover carries the About link and, until decision-packet item 1 is ruled, the single registered maturity sentence (M-01) as its only line. Nothing else lives in the status bar: no hashes, no seams, no proofs.

### 5.3 The inspector, the issues drawer, the table drawer

**Inspector.** A docked 340 px column in Model view and a 300 px slide-over in Both view (§0), on `surface.panel`. Rows are inline label-value pairs at 26 px with the label in `text.secondary` at a fixed 120 px and the value right-aligned in `type.family.numeric` with its unit; editing in place. Sections in order: identity (node, element From–To, type); geometry (DX DY DZ and X Y Z both, the row's coordinates); section and material (with the library record name and a link to Libraries); load set (name and every T and P read through, with "Edit set" and "Fork set"); attachments (restraints, loads, node data as compact sub-lists, each row opening the joined row; "Add…" at the end of each); issues on this entity (class in words, message, Show); origin and Checked (who, when; the Checked control; multi-row when several rows are selected); provenance behind a disclosure (source, location, licence, contributor, redistribution status, review status: the seven fields of C-46, read from the library record). Required fields carry the asterisk. Nothing in the inspector is a second way to mutate the model: every edit is the same row operation.

**Issues drawer.** A bottom drawer of 200 px (resizable, remembered) under the tables, opened from the status bar, the rail, the Run button's reason or ⌘⇧I. Grouped by class in the order Invalid model, Blocks solve, Blocks rule check, Provenance, Assumption, Nonlinear, Content boundary, Note, each group with its count and a filter chip; a row is the severity glyph, the class in words, the message and the entity ("node 20"), and is a button that selects the entity in the tables and the canvas. A failed run adds a banner across the top of the drawer and of the Results page: the failure reason in one sentence with "Show node 60" (C-58, C-59: the reason is the solver's diagnostic, never inferred). The drawer never becomes a modal gate; Run is a button that says why it is disabled.

**Table drawer (Model view).** A bottom drawer under the canvas of 280 px (resizable, remembered) holding the stage's current table with the table's own tab strip on its top edge (Model: Layout, Restraints, Node data; Loads: Cases, Load sets, Loads, Wind, Seismic; Results: Summary, Stresses, Displacements, Restraint loads, Restraint summary, Forces and moments, Hangers). It is the same table component with the read-through group off. The canvas keeps its width; the drawer's tab strip never grows a toolbar of its own.

### 5.4 The agent panel

A right column of 340 px on `surface.panel` with a hairline on its left, present in every stage, collapsible to a 44 px strip (⌘⇧G). The strip shows the agent glyph, the count of open proposals as a pill badge, and a dot when the agent is working. Opening the column reflows the surfaces; it never overlays the tables or the canvas. The engineer is never demoted: the column has no controls that act on the model except Accept and Reject, and everything the agent proposes is read in the tables and the canvas.

Header: "Agent" (15/600), the working state ("Idle", "Working…" with a stop control), and a tab row: Conversation, Proposals (count), Checks, Accepted.

**Conversation.** Messages at 13/18 with the author in `text.secondary` and the time; the engineer's input at the bottom as a growing field with ⌘↩ to send. The agent's messages may contain references (rows, results, report text) rendered as links that select the referent. TBD items and conflicts appear as a list under the message in `text.secondary` with the octagon or triangle of their class, never as filled-in values (C-78).

**Proposal queue.** One diff card per proposal, `surface.raised` at elevation 1, `radius.card`, 12 px padding, a 3 px `proposal.bar` on its left edge:

- Title (15/600): what and where, in the engineer's grammar: "Replace the rigid support at node 80 with a variable spring".
- Meta line (12): "Proposal P-12 · draft until accepted · 2 rows in Restraints, 1 row in Layout". The words "draft until accepted" are the M-14 standing and always appear.
- Diff rows: one per affected row, each a compact table of field, old value and new value with units, old struck in `proposal.old`, new in `proposal.new`; a row whose values are unknown shows `TBD` in `text.secondary` where the value would be. Each row has Accept and Reject at its right end as 22 px icon buttons (check and cross; tooltips "Accept row ⌘⇧A" and "Reject row ⌘⇧R"), because a 316 px card has no room for text buttons beside the values; the card's only accent is its Accept-all button.
- Rationale: a paragraph in `text.primary`.
- Constraints considered: a bulleted list.
- TBD items: a list with the triangle glyph ("Vendor table to use: TBD").
- Validation: one line, "Schema and constraints: passed" or the failures, in `text.secondary`; a proposal that fails validation cannot be accepted and says so.
- Card actions: "Accept all rows" (accent), "Reject proposal" (text).

Queue actions above the cards: "Accept all (n rows)" and "Reject all", each confirming with the count. Accepting a row applies the same structured operation a hand edit would (C-74, C-75), records the rationale, constraints and TBD items on the accepted operation (M-14), marks the row's origin as accepted, and clears the current results while keeping the run as historical. The proposal's rows in the tables carry the band until every row is accepted or rejected.

**Checks.** The agent's checks and open issues on the engineer's work, by reference: each is a card with the class ("Check" or "Open issue"), the referent as a link ("Restraints row, node 30"; "Stresses OPE1, node 40"; "Report §3, paragraph 2"), the text, and a state the engineer sets (Open, Resolved). Checks never alter a table and never use the words the boundary forbids (§7.2). The same stream appears in the Review page's third column.

**Accepted.** The record: one row per accepted operation with time, the proposal, the rows, the rationale disclosure, and Undo where it is still reversible (C-99). Reopened records from an earlier session show acceptance as unknown (C-68) with the words "Acceptance not recorded in this session".

### 5.5 The Review page primitives

A full page in Table view, three columns on `surface.panel`, 16 px gutters, at 1440 with the strip: outline 280, content flexible, comments 320.

**Header.** The page title (18/600) "Review · Loop 4 header · Run 03", the iteration control: a combobox of named snapshots ("Iteration 2 (current)", "Iteration 1 · 2026-09-17"), "Compare with…" and a "Show edits" switch; "Snapshot…" creates a named iteration. Then, once, at the top of the content column, the acceptance sentence in `text.secondary` at 13/18 as the page's one placement (§7.4). It does not repeat in any section.

**Outline column.** The report sections in the required order (M-07), each a row with a drag handle for the sections the engineer may reorder and a lock glyph for the fixed items: Notice (M-03 and, when the report includes export metadata, hanger selection records or handoff data, the supplement M-04); Identity (software, solver, build, date, units, coordinate system, model state, run); Libraries and rule packs (name, version, checksum, source note); Model; Load cases; Assumptions and warnings; Results (one entry per results table selected); Comparison (when selected; absent in this design); Hanger selection; Review and sign-off block. Each row shows a small state: empty, drafted, edited since the last iteration. Clicking scrolls the content column.

**Content column.** Section blocks in outline order. A live table block is the table component in read mode with a caption "Live from Run 03 · Stresses · OPE1" and a lock glyph: it pulls from Results and cannot be edited here; its case selector and sort are the block's own. A text block is the engineer's writing at 13/18 with a simple toolbar (paragraph, list, reference); references to rows and results are links. With Show edits on, text inserted since the compared iteration is underlined in `accent.text` and text removed is struck in `proposal.old`; with it off, the current text reads clean. The sign-off block is a form the engineer fills (name, role, date, notes); it is the report's review/sign-off block (M-07) and the product neither fills nor labels it as accepted.

**Comment stream.** The agent's checks and open issues (§5.4 Checks), plus the engineer's own notes, in one stream ordered by referent then time; a filter row (All, Checks, Open issues, Resolved, Mine). Each card carries the referent link, the text, the author and time, and Resolve or Reopen. Comments attach by reference to rows, results and report text and never alter the tables; a comment on a table row shows a small comment glyph in that row's state slot on the Review page only.

**Report production.** "Preview report" opens the report preview dialog (§5.6); "Export…" opens export and handoff. The report is produced from this page and nowhere else.

### 5.6 The probe card, the legend, the routing compass, dialogs

**Probe card.** 300 px, `surface.raised`, elevation 2, `radius.card`, follows the cursor at 12 px offset; pinned with a click (P toggles the tool). Rows at 22 px: Node, Element (From–To), Case, Stress [MPa], Allowable [MPa], Ratio with its data bar, Rule, Pack; a footer at 12 px with the run name and the evidence label chip. It shows what the table shows for that row and nothing more; it does not repeat the acceptance sentence (the results header carries it).

**Legend.** 220 px, `surface.raised`, elevation 2, in the canvas's top-right under the HUD: the quantity and unit, the scale bar with five ticks, the case or Envelope, the rule and pack for a ratio, the range control, the Unsolved swatch, and for deformation the factor ("Deformed ×50", a stepper, the ghost swatch "Undeformed") with the play control. Everything in the legend is a label or a number in text tokens; the bar is the only coloured thing.

**Routing compass and direct distance entry.** At the current node, three axis handles in `canvas.axisX/Y/Z` at 12 px labels X, Y, Z; the active axis is drawn solid and the others hairline; a length field with its unit sits at the handle ("2500 mm"); ⇥ or an arrow key cycles the axis, typing sets the length, ↩ commits and writes a layout row (From the current node, To the next number, the offset in the axis), ⎋ cancels. The ghost pipe draws in `canvas.draft` dashed. A direction change inserts a bend with the default radius shown in the new row's expansion. Every commit is the same row operation as typing the row.

**Dialogs.** Sheets at `radius.sheet`, elevation 3, 20 px padding, a title at 15/600, actions at the bottom right (primary on `accent.fill`), Escape closes. Notices required by the registry appear once, at the top of the dialog, at 13/18 in `text.secondary`, verbatim from the registry:

| Dialog | Carries | Content |
|---|---|---|
| Libraries import | M-05 | The import of materials, sections, components, hanger tables and rule packs: a file field, the seven provenance fields (C-46) as a form with their enumerations, the six import flag classes (C-32) as a findings list with the triangle, and a quarantine action for suspected protected content (C-44). No built-in catalogue of any kind (C-51). |
| Export and handoff | M-06; and the report supplement M-04 is named as what the report will carry when export metadata is included | The export's name is decision-packet item 9; until ruled, the dialog says what it produces ("Model batch file in the documented CAEPIPE MBF grammar") and makes no compatibility claim. Fields: target, units system for the file, what is included, export metadata summary. |
| Report preview | M-03; M-07 | The outline of §5.5 rendered as it prints, with the required content block and the notice as fixed sections, the review/sign-off block, and Print or Save. |
| About | M-01; M-17 | SWB Piping Designer, version, build; the registered maturity sentence as its one line (until packet item 1 is ruled); a Scope and limitations tab with scope, validation status, known limitations, data-boundary constraints and professional-responsibility limitations. |
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

Recognisable geometry, not symbols and not hardware: elbows swept at their bend radius with the tangent points marked by short ticks; tees as joined tubes with the branch's saddle line (the branch-connection kind is node data; the canvas draws the kind's silhouette when the kind determines it, otherwise the plain saddle); reducers as cones between the two ends; valves as the standard drawing silhouette extruded, a body with a bonnet stub; flanges as a pair of discs; rigid elements as a tube in `canvas.pipeShade`; expansion joints as a convoluted section; beams as their section extruded; hinges, ball and slip joints as their standard drawing symbols given the tube's thickness. Every fitting takes the pipe's edge line and, when a result is shown, the result colour.

### 6.4 Restraints and loads

Restraint glyphs are the standard symbols the report legend explains, given thickness, in `canvas.glyph` stroke over `canvas.glyphFill`, always drawn at a fixed screen size so they read at any zoom: an anchor as a hatched block at the node; +Y as a triangle under the pipe pointing up; a guide as a bracket pair either side; a limit stop as a plate on the stopped side; a two-way restraint as a bar through the pipe in the restrained direction; a skewed restraint along its direction vector; a spring as a can with a rod; a connecting node as a thin tie to the other node. Each glyph carries a small arrow in `canvas.glyph` for the direction it acts (B's contribution). A gap draws as visible clearance between the glyph and the tube, with the gap value as a label; friction draws as a short hatch on the contact face with μ as a label. Selecting a restraint's row lights its glyph with the selection halo.

Loads are arrows from the node, scaled by magnitude with a scale reference in the HUD ("1000 N = 40 px" beside the triad), labelled with value and unit on hover and selection and, when Labels is on, permanently. The arrow's form carries the kind: a single-head arrow for a force; a double-head arrow along the axis for a moment; an arrow with a bar at its base for a specified displacement; a row of short arrows along the element for a uniform load or weight; an arrow with tail streaks for wind; an arrow on a zigzag base for seismic. Colour by kind (§2.7) or by case when a case is selected; neutral when result colour is on.

### 6.5 Node labels

Labels are the node number in `canvas.label` on a `canvas.labelBg` plate with a 1 px offset from the node, 12 px, tabular. The budget rule: on for every visible node up to a budget set by the canvas size (one label per 3600 square px of canvas), with priority selected, hovered, current row, restrained, loaded, node-data, then branch points, then the rest by spacing; the current row's node is always labelled. Labels never overlap; a label that would overlap yields to the higher priority. The Labels control cycles All, Budget, Off.

### 6.6 Selection and hover

Selection is a halo: a 2 px `canvas.selection` outline around the element's silhouette and its glyphs, drawn against the ground, with the node label plate taking a `canvas.selection` edge. Hover is a thinner 1 px `canvas.hover` halo. Both sync with the tables both ways, and with the inspector. Multiple selection halos every selected element. Isolate dims everything else to 20% opacity; Hide removes it and shows a count in the HUD ("3 hidden").

### 6.7 Result colour

Applied to the tube and fittings as a flat colour per element (or per node interpolated along the element for stress), from the scale of §2.6, with the legend in the canvas. Unsolved elements stay `canvas.unsolved` at 60% opacity. Edge lines remain, so coloured geometry still reads as geometry. Selection and hover halos draw over colour unchanged. The probe reads values at the cursor.

### 6.8 Deformation

The deformed shape draws solid with the result colour or the pipe neutral, and the undeformed shape as an outline ghost in `canvas.deformGhost` at 1 px, dashed. The scale factor is stated in the legend ("Deformed ×50") and adjustable with a stepper and a field; the default is the factor that makes the maximum displacement one tenth of the model's extent, rounded to a round number, and the legend says so. Animation is a play control in the legend that oscillates the factor from 0 to the stated value; per case, never a mix of cases.

### 6.9 Proposed changes

A pending proposal draws as a ghost: the proposed geometry or glyph in `canvas.proposalGhost` dashed at 60% opacity beside the current one, and a removed element as a dashed outline; the affected node label plate takes a `canvas.proposalGhost` edge. Accepting turns the ghost into the drawing; rejecting removes it. The engineer's own routing draft is the other ghost, in `canvas.draft`, and the two are never the same colour.

### 6.10 Camera presets and the report figure

Fit (all, or the selection), Iso, Top, Front, Right, and "Report figure": the named camera saved with the project that the report's figures use. The report prints the figure exactly as the canvas draws it at that preset, in the theme chosen for the report (light by default), with the legend and the scale reference. The triad in the bottom-left corner states the up axis in its label ("Y up").

### 6.11 Both themes

Every token in §2.8 has a light and a dark value and the figure was designed for both: pale ground, grey tube, dark edge, dark glyphs in light; near-black ground, mid-grey tube, light edge, light glyphs in dark. The result scale flips its anchor (§2.6); halos, ghosts and vectors keep their hues and are re-stepped for the dark ground. The engine stays as it is: this section describes what the model looks like, and the rendering brief owed to the piping session carries it as overlay specifications.

## 7. Copy rules

### 7.1 Allowed and forbidden words

| Where | Allowed | Forbidden |
|---|---|---|
| Proposal controls | Accept, Reject, Accept all rows, Reject proposal, Accept all (n rows), Reject all | Approve, Apply proposal, Confirm, Sign off |
| Proposal standing | Proposal, draft until accepted, rationale, constraints considered, TBD | Verified, validated, approved, recommended by |
| The checked mark | Checked, Checked by *name*, stale, Check again, Clear check, Unchecked rows | Verified, reviewed and approved, signed, accepted |
| Statuses | the six short labels of §2.3 with the raw token one click away | Any status not in the six; Ready, OK, Pass, Compliant, Certified, Sealed, Approved, Authenticated |
| Results | number, rule ID, pack version; Ratio, Allowable, Governing case, Envelope | Pass, Fail, OK, Compliant, Code compliant, Exceeds code, Safe, Acceptable |
| Evidence | Internally verified, Prover correlated (raw tokens in tooltips) | Engineer accepted (never emitted), Validated, Certified |
| Agent output | check, open issue, proposal, draft, evidence summary, TBD | accepted, approved, verified, correct, compliant |
| Runs | immutable, named, Historical saved run, current solve basis | Final, Released, Certified run |
| Anywhere | | certify, seal, approve, authenticate, comply, compliant, non-authoritative, not authoritative; the fence token; any stack of three or more boundary terms outside a registered sentence |

Generated text (toasts, empty states, tooltips, agent messages) follows the same table. Toasts say the outcome and offer Undo: "Applied. Results cleared." "Pasted 42 rows." "Run 03 failed: nonlinear support at node 60 did not converge. Show node 60."

### 7.2 Ratios

A ratio is always three things together: the number to two decimals, the rule ID and the pack version, in that order in a table (Ratio, Rule, Pack) and in one line elsewhere ("0.72 · EXP-A1 · pack 1.2"). A ratio never carries a verdict word or a verdict colour; the data bar is the scale, and 1.0 is a tick.

### 7.3 Checked

The control is "Check" (⌘⇧K) and the mark is "Checked". Its tooltip: "Checked by *name* · *date time* · bound to this row's content · not a software status". When stale: "Checked by *name* · *date time* · the row changed since · Check again or Clear". The filter is "Unchecked rows". The word is a tag on a row; its governance standing and final wording remain decision-packet item 7.

### 7.4 Disclosure homes for the chosen direction

One placement per surface class. The acceptance sentence (M-02) appears in the results header's information disclosure for the results surface class, once at the top of the Review page, and in the report; it does not appear on the Model or Loads pages, in the inspector, the probe, the canvas or the agent panel.

| ID | Home |
|---|---|
| M-01 | About; until packet item 1 is ruled, the status bar's information popover carries the single sentence. |
| M-02 | Results header disclosure (results surface class); Review page top (Review surface class, packet item 8); the report. |
| M-03 | The report and the report preview; the Notice section of the outline. |
| M-04 | The report and its preview when export metadata, hanger selection records or handoff data are included; named in the export dialog. |
| M-05 | The Libraries import dialog; the export and redaction surfaces. |
| M-06 | The export and handoff dialog. |
| M-07 | The report outline on the Review page and the preview. |
| M-08 | The status bar chip, with the authority domain in its popover. |
| M-09 | The results header chip beside the run name; the report's case pages. |
| M-10 | The issues drawer, the gutter's state slot, the cell marks, the rail's Issues count, the canvas halo on an entity with a blocking issue. |
| M-11 | Every column header, inspector row, probe row, legend and report table. |
| M-12 | The inspector's provenance disclosure; the Libraries dialogs. |
| M-13 | The results header band and the rail's Historical caption; the status chip's popover. |
| M-14 | The proposal card ("draft until accepted"), the diff, Accept and Reject, the rationale, constraints and TBD items, the Accepted record. |
| M-15 | The rule-expression cell's dashed frame and caption on the Loads page and in the Rules dialog. |
| M-16 | Preferences, Appearance, Contrast findings. |
| M-17 | About, Scope and limitations. |

### 7.5 The product name

SWB Piping Designer is the product; SWBPIPE is the wordmark, the window title's short form and the file badge. The old name appears only when a registered sentence is quoted by ID, and the replacement sentences are decision-packet item 3. The wordmark is set in the system face at 13 px medium with 0.04 em tracking; there is no logotype in this system.

## 8. Open items and what the mocks must test

Open items, none of which changes governed text and each named for the decision packet where it belongs:

1. The short labels for the six statuses and the two evidence labels (packet item 4) and the Checked tooltip wording (packet item 7).
2. The export's name in the export dialog before compatibility evidence exists (packet item 9).
3. The inspector in Both view as a slide-over rather than a docked column: chosen here to keep the tables unreflowed; the mocks should show state 4 with it open.
4. Whether the read-through T and P columns are on by default in Both view (this system says off, with the values in the Load cell's tooltip) or on with a horizontal scroll.
5. The categorical set's all-pairs standing beyond three slots: this system makes form the carrier of load kind; the rendering brief should confirm the arrow forms are drawable at the sizes stated.
6. The contrast target (M-16): the findings in §2.9 are the owner's input; `text.disabled`, the hover halo and the light pipe neutral are the pairs most sensitive to the choice.
7. The rail's Results item caption "Historical" and the results header band reuse the existing component's wording (M-13); no new wording is proposed.
8. The wordmark: a typeset SWBPIPE is proposed; a drawn mark is not part of this system.

What the mocks must test, state by state:

- State 1 (Table): an empty model's first row with the required marks and the Run button's reason; the gutter and marks column empty.
- State 2 (Model, also dark): the compass and direct distance entry writing rows into the drawer; the draft ghost against the dark ground; label budget on a small model.
- State 3 (Table): propagation marks after keyboard entry; the paste band with two unmapped columns; the footer counts.
- State 4 (Both, also Table): restraint and load marks on node rows, the glyphs with their direction arrows, gap and friction labels; the inspector slide-over open.
- State 5 (Table): generated, edited and authored case rows with their origin glyphs; the display-only expression frame.
- State 6 (Both): the running Run button, then the failure banner and the issues drawer; the status chip and its popover.
- State 7 (Both, also Table, also dark): the stress table with data bars, the case selector and Envelope, the evidence chip, the results header disclosure open; the coloured model with the legend and probe in both themes.
- State 8 (Table, also Model): the hanger table against a user-supplied library; a proposal landing as banded rows with old and new values and as a ghost in the canvas; Accept row by row; the Accepted record.
- State 9 (Table, also dark): the three-column Review page with live tables, edits shown and hidden, two iterations, the comment stream, and the acceptance sentence once at the top.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
