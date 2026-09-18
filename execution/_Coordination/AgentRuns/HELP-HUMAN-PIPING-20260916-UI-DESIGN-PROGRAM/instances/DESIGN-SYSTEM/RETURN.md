# RETURN — DESIGN-SYSTEM-01 · Design system V1

Run `HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM`, instance `DESIGN-SYSTEM`, 2026-09-18.
Brief: `{RUN}/briefs/DESIGN-SYSTEM-01_design_system.md` (sealed). `{RUN}` is
`projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM`.

## 1 What was read, in the brief's order

| # | Path | Sections read |
|---|---|---|
| 1 | `{RUN}/instances/ROOT/DESIGN_BRIEF_V1.md` (V1.4) | whole: purpose, fixed points, mandatory disclosures M-01 to M-17, constraints C-01 to C-110, decision-packet items 1 to 9 |
| 2 | `{RUN}/instances/ROOT/DIRECTION_DECISION_2026-09-17.md` | whole; §10 (origins quiet, proposals loud) applied directly in §4 of the document |
| 3 | `{RUN}/instances/CONCEPTS/CONCEPT_DIRECTIONS_V1.md` | whole; §0 fixed points, §1.3/2.3/3.3 shells, §1.5/2.5/3.5 disclosure homes, §1.6/2.6/3.6 presentation languages used directly |
| 4 | `{RUN}/instances/CONCEPTS/wireframes/run_book_split.svg` | text labels only (the words on the wireframe, not its drawing) |
| 5 | `{RUN}/instances/RESEARCH/E_caepipe_format.md` | whole: the MBF layout grammar (one row per node carrying the arriving element, From explicit, branch by From, node data and joints as codes, T/P records) |
| 6 | `{RUN}/instances/RESEARCH/C_ui_constraints.md` | whole, including §4 disclosures |
| 7 | `{RUN}/instances/RESEARCH/A_ux_audit.md` | whole, including the Preserve list |
| 8 | `{RUN}/instances/RESEARCH/D_domain_ux_research.md` | whole |

Also read: the sealed brief itself, and the bundled `dataviz` skill (`SKILL.md`,
`references/color-formula.md`, `references/palette.md`, `scripts/validate_palette.js`)
for the colour-scale method and its validator, as the brief suggests.

Not read: anything under `{REPO_ROOT}/projects/chirality-piping/apps/` (no fact about
the engine's operations was needed), and no stylesheet or component of the current
product. One product dependency was used read-only as a tool, not as a source:
Playwright from `projects/chirality-piping/node_modules`, to drive a headless Chromium
for the rendering checks in §5. The current product's name was observed only in the
title of a preview tab already open in the App and is used here only as a string the
specimen must not contain.

## 2 What was produced

All under `{RUN}/instances/DESIGN-SYSTEM/`:

| File | Content | SHA-256 |
|---|---|---|
| `DESIGN_SYSTEM_V1.md` | §0 shell geometry; §1 Foundations; §2 Colour (hue budget, every token, status and issue mappings, result scale, categorical set, contrast findings with the full 128-row table); §3 Iconography; §4 Marks vocabulary; §5 Components; §6 3D presentation language; §7 Copy rules and disclosure homes; §8 Open items and what the mocks must test. 86,722 bytes. | `39a9eb0d51c7f530c7fafb57968e558b354ccfeab1f127b7d81f9aff83a88103` |
| `tokens.json` | 88 colour tokens as `{"name": {"light", "dark"}}` (hue named on the categorical slots) and 85 plain tokens: type (families, sizes, line heights, weights, minimum repeatedly-read size, figure letter-spacing), space, radius, border widths, elevation (light/dark), focus, motion, layout, icon. Version 1.0. | `4c1252d75d4b455cb7e16cc0c612bcd6b64e391d07b92dc76831b867f59f2b0a` |
| `specimen.html` | one self-contained page, inline CSS and script, no external reference, system fonts with declared fallbacks; twelve sections: foundations, every colour token in both themes with the value in force, result scale and categorical set, icons, marks placements, the layout table with every mark and cell state on eleven sample rows in the CAEPIPE grammar with the paste band and footer, the results header (case selector, Envelope, evidence label, the M-02 sentence verbatim) with data bars and a stale row, load-case rows with a display-only expression, the six status chips with the popover, the proposal diff card, the probe card and legends, the shell (rail with counts, view switch, toolbar band, agent strip, status bar), Review page primitives, the canvas palette drawn as a schematic figure, and Preferences with the contrast findings computed live from the embedded tokens. Theme follows the system preference with a manual System / Light / Dark switch. | `dfb8eb4daab979766e9c35523d67076f19ced5ec8383325f300c1ee72fefae8b` |
| `RETURN.md` | this record | — |
| `tools/` | the generator and check scripts used (not deliverables): `palette.mjs` (OKLCH design, categorical ordering search, writes `tokens.json`), `contrast.mjs` (the 128 pairings), `gen.mjs` (CSS variable blocks and embedded JSON from `tokens.json`), `splice.mjs`, `render.mjs` (headless renders), `agree.mjs` (the agreement check in §4). They were run from the session's scratch directory with Node 24. | — |

Nothing else in the repository was modified; no git command that changes state was run.

## 3 Model and effort actually used

Claude Fable 5.1 (`claude-fable-5-1`) at maximum reasoning effort, one agent, no
delegation. Tools: file read and write, Bash (Node and Python for generation and
checks), the App's browser pane, and a headless Chromium through Playwright. One
context compaction occurred mid-run; work resumed from the files on disk and a
transcript summary, and every file was re-checked afterwards by the scripts in §4.

## 4 How agreement between the document, the token file and the specimen was checked

`tokens.json` is the single source. The §2.2 colour tables and the §2.9 contrast table
in the document were generated from it and spliced in; the specimen's three CSS
variable blocks (light; dark by `prefers-color-scheme`; dark by the manual switch) and
its embedded JSON were generated from it. `tools/agree.mjs` then re-read the three
files independently and checked:

- the 88 token names and light/dark values in the document's tables against the JSON
  (88 rows, no difference), and the document's CSS variable names;
- each token's value in each of the specimen's three CSS blocks (264 checks) and the
  embedded JSON byte-equal to `tokens.json`;
- the document's 128-row contrast table against a fresh computation from the JSON;
- nine plain-token phrases in the prose (row 26 / header row 28, gutter 32, marks
  column 72, toolbar 48 / status bar 24, the type sizes, `space.9`, `radius.card`)
  against the JSON values;
- the specimen contains neither the fence token, `DEC-081`, the current product's
  name, nor any external `src`, `href`, `@import` or `url(http…)`; the document carries
  the fence line.

Final result against the hashes above: no differences. Layout-table column widths in
§1 are prose defaults, not tokens; the specimen uses them literally.

## 5 Rendering verification

- Browser pane: the file loaded from `file://` at a 1440 × 900 viewport, light;
  the first sections inspected by scrolling; the manual Dark switch applied
  (`data-theme="dark"`, body background `#1b1e22`, `color-scheme: dark`) and the
  System setting restored; a script probe confirmed 88 token rows and 128 contrast
  rows rendered from the embedded JSON, no horizontal page scroll, and zero resource
  requests. The pane renders a local file as a static snapshot with the light system
  preference, and deeper scroll positions did not repaint in that mode, so the full
  per-section inspection and the system-dark path were verified headlessly.
- Headless Chromium (Playwright) with every non-`file://` request aborted: 0 requests
  attempted, 0 console errors or warnings, at 1440 and 720 wide, each with
  `prefers-color-scheme` light and dark (the System setting) and then the manual switch
  to the other theme (attribute, background and `color-scheme` verified each time).
  Per-section images were inspected in all four combinations; page `scrollWidth`
  equals the viewport at both widths; no panel overflows its box; the smallest font is
  11 px and only in captions.
- Found and fixed during verification: a class collision between the Review page grid
  and the review status chip; token tables clipped at 1440 (regrouped into two columns,
  single-row groups merged into "interaction" and "state overlays" panels); the
  categorical grid overflowing at 720; the layout and results tables stretched to the
  page width (now at their default column widths, the frame shrinks to the table);
  a proposed cell that cannot show old and new values in a 72 px column (rule in §5 of
  the document amended: the new value with the corner tick, the old value struck only
  when there is room, otherwise in the tooltip and on the card); the edit cell's unit
  overlapping the value; the proposal card's per-row Accept and Reject not fitting a
  316 px card (now 22 px icon buttons with the accelerators in their tooltips; §5
  amended); the rail item height with a caption; and in the canvas figure, label
  collisions, a hover halo too faint to see, the undeformed ghost hidden under the pipe
  (the first span now sags, the ghost is drawn on top) and the resting support moved to
  node 20 in both the figure and the table so they agree.

## 6 Colour findings, in brief

- 128 pairings (WCAG 2.x relative luminance; translucent tokens composited over their
  named surface), both themes, in §2.9 of the document and live in the specimen's
  Preferences section. Text pairings sit between 3.3:1 and 15.1:1 (`text.primary`
  8.9 to 15.1, `text.secondary` 5.1 to 7.8, `text.muted` 3.3 to 5.0) except
  `text.disabled` at 2.0 to 2.8:1, used only for disabled controls. Hairlines are
  recessive by intent (1.1 to 1.6:1). `canvas.pipe` on the light canvas is 1.97:1 and
  is carried by the edge line (7.9:1). `result.scale.1` beside `canvas.unsolved` is
  1.02:1 in light, so unsolved elements are dimmed to 60 % and never share the scale's
  hue.
- Categorical set: eight slots in a fixed order (orange, magenta, green, purple, rose,
  azure, olive, cyan) validated with the dataviz validator against the canvas surface
  in both themes: lightness band, chroma floor, adjacent-pair CVD separation and the
  normal-vision floor pass; all-pairs separation passes only for the first three slots
  (force, moment, displacement), so the arrow's form and label carry the load kind and
  cases beyond three must not rely on colour alone. All eight are ≥ 3:1 on the canvas
  and on `surface.panel` in both themes.
- Result scale: one hue (teal), seven anchors, validated as ordinal in both themes
  (ΔL ≥ 0.06 between steps; light end 2.02:1 light and 3.00:1 dark against the canvas),
  dark-mode anchor flip so near-zero is never darker than the pipe neutral,
  interpolation in OKLCH.
- No conformance is claimed and no target is named (M-16).

## 7 Uncertainties

1. The eight open items in §8 of the document: (1) the short labels for the six
   statuses and the two evidence labels, and the Checked tooltip wording (packet item
   4); (2) the export's name before compatibility evidence exists (packet item 9);
   (3) the inspector in Both view as a 300 px slide-over rather than a docked column;
   (4) whether the read-through T and P columns are on by default in Both; (5) the
   categorical set's all-pairs standing beyond three slots; (6) the contrast target
   (M-16), with `text.disabled`, the hover halo and the hairlines as the owner's
   input; (7) the wording of the rail's "Historical" caption and the results header
   band; (8) the wordmark, typeset SWBPIPE proposed, no drawn mark. The eight mock
   states that test them are listed in the same section.
2. M-01's placement follows packet item 1 (About and the status bar's information
   glyph in this system); the wordmark and product name in the specimen are placeholders
   pending that decision.
3. Specimen choices that changed the prose during verification and should be judged in
   the mocks: Type column default 100 px (76 does not fit "Expansion joint"); the
   proposed-cell rule; the icon action buttons on the diff card.
4. The figure language is drawn schematically in 2D SVG; depth, occlusion, glyph
   scale at real zoom levels and the deformation animation are untested until a
   prototype renders them.
5. Fonts: SF Pro Text and SF Mono were what rendered on this Mac; the declared
   fallbacks were not visually checked on another platform.
6. Contrast is measured as WCAG 2.x ratio only; no other perceptual measure was
   computed. The categorical CVD validation was run against the canvas surfaces; on
   `surface.panel` only the ratio was checked.
7. The sample values are placeholders from one fictional model and are labelled so;
   no engine output was used.

## 8 Where the brief could not be satisfied

Nothing knowingly. Two partial points: the categorical set cannot meet the validator's
all-pairs target for eight slots under any ordering, so the document makes form the
carrier beyond three slots instead of changing the palette; and at half width (720)
the wide tables scroll inside their frames and the shell toolbar wraps to two rows,
which is legible but not the 1280 minimum window the system specifies.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
