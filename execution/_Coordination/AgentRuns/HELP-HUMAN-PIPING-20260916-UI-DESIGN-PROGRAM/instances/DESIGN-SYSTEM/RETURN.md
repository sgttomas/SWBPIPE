# RETURN — DESIGN-SYSTEM-02 · Design system V1.1

Run `HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM`, instance `DESIGN-SYSTEM`, 2026-09-18.
Brief: `{RUN}/briefs/DESIGN-SYSTEM-02_revision.md` (sealed). `{RUN}` is
`projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM`
under `{REPO_ROOT}`. This record replaces the V1 return (brief DESIGN-SYSTEM-01) for
this instance; the V1 return's facts that still hold are restated where they matter.

## 1 What was read, in the brief's order

| # | Path | What was read |
|---|---|---|
| 1 | `{RUN}/briefs/DESIGN-SYSTEM-02_revision.md` | whole: the assignment, the fourteen decisions to apply, the deliverables and the acceptance checks |
| 2 | `{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md` (V1) | whole, as the text to revise in place |
| 3 | `{RUN}/instances/DESIGN-SYSTEM/tokens.json` (1.0) | whole, as the source to revise; a copy kept in the session scratch directory as the regression baseline |
| 4 | `{RUN}/instances/DESIGN-SYSTEM/specimen.html` (V1) | whole |
| 5 | `{RUN}/instances/DESIGN-SYSTEM/RETURN.md` (V1) | whole |
| 6 | `{RUN}/instances/DESIGN-SYSTEM/tools/*.mjs` (V1) | whole; rewritten or extended in this revision |
| 7 | `{RUN}/instances/ROOT/DIRECTION_DECISION_2026-09-17.md` | §10 (the eleven agreed recommendations) and §11 (the fourteen mock-review decisions, verbatim, with the owner's word and its hash) |
| 8 | `{RUN}/instances/MOCKS/MOCKS_V1.md` | whole: §1 D-1 to D-11, §2 the twelve frames' decisions, §3 P-1 to P-10, §4 Q-1 to Q-14, §5 G-1 to G-6, §6 |
| 9 | `{RUN}/instances/MOCKS/tools/ui.mjs`, `frames.mjs`, `canvas.mjs`, `build.mjs` | as the working record of the frames' values: column widths (P-4), the inspector label width, header padding and unit size (P-1), the pressed style (G-5), the hint and track colours (G-4, G-6), the draft bar (G-3), the token mapping `build.mjs` shares with `gen.mjs` (kept compatible). The rendered frames themselves were not re-opened. |
| 10 | `{RUN}/instances/RESEARCH/C_ui_constraints.md` | §4 "Mandatory disclosures", row M-07 (the registered "review/signoff block", from `docs/PRD.md` §19.2; `docs/SPEC.md` §9; `OPS-K-REPORT-1`) |

Also used as a tool, not as a source: the bundled `dataviz` skill's
`scripts/validate_palette.js` (skill version 2.1.275, file SHA-256
`6ac628a48e67d58918f4cb2bde55076a2344f77a986cee767eb7bf3306496914`), supplied to
`tools/palette.mjs` at run time through `--validator`; no path to it is stored in any
file. Playwright from `{REPO_ROOT}/projects/chirality-piping/node_modules` was used
read-only to drive a headless Chromium for §5, with Node v24.18.0.

Not read and not written: anything under `instances/MOCKS/` other than the files above
(nothing there was modified), product source, tests, governance files. No git command
that changes state was run; `git status` and `git diff --stat` were run read-only at the
end to confirm that only files under `instances/DESIGN-SYSTEM/` changed.

## 2 What changed

All under `{RUN}/instances/DESIGN-SYSTEM/`; every file revised in place.

| File | Change | Bytes | SHA-256 |
|---|---|---|---|
| `DESIGN_SYSTEM_V1.md` | Revised to **V1.1**. Status line rewritten; §0 docked Both-view geometry with the narrow-case table; §1 header padding, caption-size header units, the latched toggle, the draft bar, the 88 px inspector label; §2 hue budget rows and notes for ten new tokens, the chip policy (§2.3), the stale band and draft row (§2.5), §2.6 rewritten for the re-anchored dark scale, §2.7 and §2.8 notes, §2.9 method and findings re-read; §3.2 icon rows; §4 marks rows (Stale results, Draft row, Comment) and the required mark's placement; §5.1 header, columns, cell states (Draft), paste band, footer as the counts line, row expansion generalised (joined row, element fields, combination editor), the Loads header band, results tables (disabled "Case: all", the identity line in the disclosure, the stale band, the Ratio menu, the hangers caption) and the width table; §5.2 rail captions and tooltips, the Run tooltip, the latched Agent toggle, the run log popover, the status bar policy; §5.3 docked inspector, routing block, proposed line, one banner and the filtered drawer, the empty state, the drawer's strip; §5.4 the collapsed accepted row, the consequence line, Accept remaining / Reject remaining, the record line; §5.5 header, outline with "Review/signoff block", live-block captions, comment stream; §5.6 probe, legend, narrow-canvas rules, the three-part draft ghost, the hint strip, the preview dialog's home; §6.4 plates, §6.5 Budget default and placement search, §6.9 ghost placement, §6.10 Iso azimuth and the bottom-edge furniture; §7.1 rows for the frames' strings and the sign-off exception, §7.4 M-02, M-07, M-08, M-13 homes; §8 open items 3, 4, 6, 7 revised and 9 to 11 added, the second-pass mock list; **new §9 change log** with 49 rows, each with its source (rows 48 and 49 record ROOT's review of V1.1; §9 below). The generated §2.2 and §2.9 tables now sit between markers. | 130,384 | `34ba3f1fadacbd1c3854e7bb654098b5525e25b9734b4106dfdd39fc94f0cf1f` |
| `tokens.json` | Version **1.1**. Ten colour tokens added: `pressed.fill`, `pressed.ink` (G-5), `stale.band`, `stale.ink` (G-1), `rail.captionFailed`, `rail.captionStale`, `rail.captionHistorical` (G-2), `canvas.hint` (G-4), `draft.bar` (G-3), `bar.track` (G-6): 98 colour tokens. `result.scale.1` to `.7` dark re-anchored (decision 11): `#045c59 #04706b #09847f #099993 #24ada7 #52bfb9 #74d1cb`. Layout gains `inspector.label` 88, `canvas.min` 220, `cellPadding` 8, `headerPadding` 5: 89 plain tokens. Nothing else differs from 1.0 (checked by a diff against the kept 1.0 copy); the file is reproduced byte for byte by `tools/palette.mjs --write`. | 11,744 | `b6a382727728845c7cafe3d9e66b4bf87df888c9a6bebaae8c1f9f59dbbafe59` |
| `specimen.html` | **V1.1**. Heading and footer; the CSS block, the embedded token JSON and a new embedded pair list (`<script id="pairs">`) generated between markers; header cells at 0 5 with caption-size units; the layout table's start row with "—" and empty read-through cells, the editing cell's unit at its right edge, a draft row with the dashed `draft.bar`, tab counts, the paste band's wording, the footer as the counts line with the switches; inspector rows at an 88 px label with the routing block and the proposed line; the results header with "Case: all" disabled and Envelope on, the run identity line in the disclosure, the stale band with Run again, stale cells with the stale glyph, Element 80 / Allowable 116, the results footer chips; the Loads header band and the combination editor as a row expansion under OCC1; the chip policy's six moments; the proposal card with the collapsed accepted row, the consequence line and Accept remaining (1 row) / Reject remaining, the record line; the legend's quantity line and the pinned probe; the shell after a model change (rail caption Stale in `rail.captionStale`, no chip) with the docked-inspector geometry as three proportional strips; toolbar states with the latched Agent toggle and a latched HUD tool on `pressed.fill` / `pressed.ink`; rail captions Failed, Stale, Historical; the failure banner with the empty state; the run log popover; the two-chip status and the Model incomplete popover; the Review header with Report preview and Export…, the outline's state words and "Review/signoff block", the inserted/removed legend, the comment stream's header and filters; the canvas figure with the hint strip on `canvas.hint`, the draft ghost as a 1.5 px dashed centreline with a faint outline and the draft node's plate, the length field above-right of the axis tip, the +Y glyph's plate, neutral vectors; data bars on `bar.track`; the script reads the pair list from the page, groups `draft` under interaction, `rail` under status and `bar` under result, and colours the scale step labels by OKLab lightness. Self-contained, no network. | 155,817 | `5be4f482069225de0f35c2bbce57e4046d4ebae6095a9ed8970f7bb6611ef6b7` |
| `RETURN.md` | this record | — | — |
| `tools/palette.mjs` | Rewritten: ordered token list; the ten new tokens; the re-anchored dark ramp (L 0.43 to 0.80 in steps of 0.062, hue 190); the validator located through `--validator <path>` or `DATAVIZ_VALIDATOR`, with a built-in twin of its ordinal check as the fallback; `--write <dest>`; `--search` for the V1 categorical ordering search. The old absolute import of the validator is gone. | — | — |
| `tools/contrast.mjs` | The pair list exported (146 pairs: the 128 of V1 plus 18 for the new tokens, the edge line on the near-zero and brightest steps, and the data bar on its track); `computeRows`, `toMarkdown`. | — | — |
| `tools/gen.mjs` | Generates the CSS blocks, the embedded token JSON, the embedded pair list, the §2.2 colour tables (one per group) and the §2.9 table from `tokens.json`. | — | — |
| `tools/splice.mjs` | Idempotent replacement between markers in the specimen (`GENERATED:TOKENS_CSS`, the two `<script>` blocks) and the document (`GENERATED:COLOUR_TABLES`, `GENERATED:CONTRAST_TABLE`). | — | — |
| `tools/agree.mjs` | The agreement check of §4, extended for V1.1. | — | — |
| `tools/render.mjs` | Output directory as an argument; V1.1 element and colour checks; the manual-switch check also records the step label colours. Still locates Playwright relative to its own location. | — | — |

Dispositions of the frames' items are in the document's §9: every item of MOCKS_V1
§1, §2 and §3 is adopted except P-5 and P-9 (superseded by decisions 1 and 12), P-8
(adopted for the frames, overruled as a product preset), the s5 frame's cell popover
(overruled by decision 14) and D-8's second azimuth (a frame convention, not a preset).
The six gaps G-1 to G-6 are closed by the ten tokens above.

## 3 Model and effort actually used

Claude Fable 5.1 (`claude-fable-5-1`) at maximum reasoning effort, one agent, no
delegation. Tools: file read and write, Bash (Node 24 for generation and checks), and a
headless Chromium through Playwright. One context compaction occurred mid-run, after the
tool chain and `tokens.json` 1.1 were written and before the document and specimen were
edited; work resumed from the files on disk and a transcript summary, and every
deliverable was re-checked afterwards by the scripts in §4 and §5. The document and
specimen edits were applied as anchored replacements (each anchor required to occur
exactly once), so no edit landed in an unintended place.

## 4 Agreement check

Command, from `tools/`:
`node gen.mjs ../tokens.json <scratch>/gen && node splice.mjs <scratch>/gen ../specimen.html ../DESIGN_SYSTEM_V1.md && node agree.mjs ../DESIGN_SYSTEM_V1.md ../tokens.json ../specimen.html`

Result against the hashes in §2: `problems: []` — **zero differences**. Counts reported:
98 colour tokens, 89 plain tokens, 98 colour rows in the document, 146 contrast rows in
the document, 146 pairs in `contrast.mjs`. What the check covers:

- the 98 token names, light and dark values and CSS variable names in the document's
  §2.2 tables against `tokens.json`, and the tables byte-equal to a fresh generation;
- each token's value in each of the specimen's three CSS blocks (294 checks), the CSS
  block byte-equal to a fresh generation, the embedded token JSON equal to
  `tokens.json`, the embedded pair list equal to `contrast.mjs`, every pair naming known
  tokens;
- the document's §2.9 table equal row by row to a fresh computation (146 rows);
- fifteen plain-token phrases in the prose against the JSON (row 26 / header row 28,
  gutter 32, marks column 72, toolbar 48 / status bar 24, the three type sizes,
  `space.9`, `radius.card`, the 88 px inspector label, the 220 px canvas minimum,
  header cell padding 0 5, cell padding 0 8, canvas 303 with the 300 px docked
  inspector, the 340 px Model-view inspector);
- the status line says V1.1, the token file says 1.1, the specimen's heading and CSS
  comment say V1.1; §2.6 quotes the re-anchored top step `#74d1cb` and the edge value
  `#c7cbd0`;
- §9 cites decision 1 to 14, D-1 to D-11, P-1 to P-10, G-1 to G-6 and the twelve
  frames of MOCKS_V1 §2;
- the specimen (with the M-02 sentence removed first) contains none of the forbidden
  words or tokens (approve, certify, seal, authenticate, comply, sign-off, the fence
  token, DEC-081, the old product name) and no external `src`, `href`, `@import` or
  `url(http…)`; the document carries the fence line; neither file contains an absolute
  user path.

A separate check confirmed that `tools/palette.mjs --write` reproduces `tokens.json`
byte for byte with the validator supplied, and a grep confirmed that no file under
`instances/DESIGN-SYSTEM/` contains an absolute filesystem path.

## 5 Rendering verification, offline, both themes

`node tools/render.mjs <path to specimen.html> <scratch>/shots`, headless Chromium
through Playwright with every non-`file://` request aborted, four contexts: 1440 × 900
and 720 × 900, each with `prefers-color-scheme` light and dark (the specimen's System
setting), and in each the manual switch to the other theme.

Recorded in all four contexts: **0 requests attempted, 0 console errors or warnings**;
page `scrollWidth` equals the viewport (no horizontal scroll at 1440 or 720); no panel,
card, probe, legend or shell overflows its box; the smallest font is 11 px and only in
captions; 98 token rows and 146 contrast rows rendered from the embedded JSON and the
embedded pair list. The V1.1 elements are present and take their tokens in both themes:
the stale band's background is `rgb(254, 246, 231)` light and `rgb(54, 44, 28)` dark
(`stale.band` `#fef6e7` / `#362c1c`); the latched toggle's fill `rgb(212, 230, 255)` /
`rgb(37, 73, 116)` (`pressed.fill`); the rail caption `rgb(125, 80, 2)` /
`rgb(233, 202, 137)` (`rail.captionStale`); the data bar's track `rgb(224, 227, 230)` /
`rgb(60, 64, 69)` (`bar.track`); the disabled "Case: all" selector, the draft row with a
dashed gutter bar, the 1.5 px dashed draft centreline, the hint strip, the three
geometry strips, the run log, the banner, the "Review/signoff block" row, the Report
preview button and the combination editor are all found. The manual switch sets the
attribute, the background and `color-scheme` each way, and the scale step labels change
with the theme by lightness (light: steps 1 and 2 dark text; dark: steps 1 to 4 white).

Per-section images inspected: sections 2, 4, 5, 6, 7, 8, 9, 10 and 11 at 1440 in both
themes, and sections 5 and 9 at 720. Found and fixed during the inspection: the card's
two action buttons wrapped in the 316 px card (now compact); the figure's draft-ghost
caption ran past the canvas edge; the inspector's proposed line and its origin and
Checked values were truncated at the 88 px label (the rows are now labelled "Entered
by" and "Checked by" with the value "R. Tufts · 2026-09-17 14:02", which is P-2's own
example of what 88 px leaves room for). After the fixes the checks above were re-run
with the same results. The App's browser pane was not used for this revision.

## 6 Colour findings, in brief

- Dark result scale (decision 11): V1's dark ramp ran L 0.50 to 0.92 in steps of 0.07,
  its top above the dark edge line (`canvas.edge` `#c7cbd0`, L 0.84). Shifting it down
  by exactly one step would leave the top at L 0.85, still above the edge, so the bottom
  moved one step down to L 0.43 (`#045c59`) and the step narrowed to 0.062, putting the
  top at L 0.80 (`#74d1cb`), 0.04 under the edge, with every adjacent pair still at
  least 0.06 apart after 8-bit rounding. Validator output, both themes, with the real
  `validate_palette.js`: light `Lightness monotone true · Adjacent ΔL true (all gaps
  >= 0.06) · Light-end contrast true (#4ab9b2 at 2.02:1 vs surface) · Single hue true
  (hue spread 2°) · ok true`; dark the same with `#045c59 at 2.18:1 vs surface`. The
  light ramp is unchanged.
- Consequences in the findings (§2.9 of the document): the dark near-zero step on the
  canvas 2.18:1 (was 3.00:1); the brightest dark step 9.55:1 (was 13.8:1); the edge
  line on the brightest step 1.10:1 dark and 1.29:1 light; the near-zero step beside an
  unsolved element 1.83:1 dark (was 1.33:1); legend swatches 1.65:1 to 7.21:1 dark; the
  data bar on its track 1.84:1 to 9.29:1 light and 1.33:1 to 5.83:1 dark.
- New tokens: `stale.ink` on `stale.band` 6.47:1 / 8.65:1; `text.primary` on the stale
  band 14.02:1 / 11.14:1; the three rail captions on the base 6.09:1 to 10.57:1;
  `pressed.ink` on `pressed.fill` 5.97:1 / 5.66:1, the fill itself 1.11:1 / 1.82:1 on
  the base; `draft.bar` on the panel 5.12:1 / 6.40:1 and on the selection band 4.36:1 /
  4.70:1; `text.secondary` on `canvas.hint` over the canvas 6.15:1 / 6.21:1;
  `bar.track` on the panel 1.29:1 / 1.42:1 (recessive by intent).
- Categorical set unchanged and re-validated with the same validator: adjacent
  pairlist pass in both themes (worst adjacent CVD ΔE 10.4 deutan light, 10.2 protan
  dark; normal-vision 16.9 both), as in V1.
- No conformance is claimed and no target is named (M-16).

## 7 Uncertainties

1. Decision 11 was applied as "the bottom one step down and the top under the edge",
   not as a literal shift of the whole ramp by its own step, because the literal shift
   leaves the top at L 0.85, above the edge at 0.84; §2.6 says so. If ROOT reads the
   decision as the literal shift, the top step is the one value to revisit.
2. The cost of decision 11 is the edge line on the brightest dark step (1.10:1): on a
   fully loaded element in dark the line nearly merges with the tube and the silhouette
   carries the geometry. The same pairing is 1.29:1 in light, unchanged from V1. Both
   are now open item 6's input for the contrast target.
3. The docked inspector's narrow case (§0): after ROOT's review the rule is
   UX_SPEC_V1 §10.9's (the agent column collapses to its strip first; failing that the
   inspector opens as a slide-over for that window size only, with the tooltip "Docked
   inspector needs a wider window"; the canvas is never collapsed), kept with this
   system's 220 px canvas minimum, the camera rule and the 400 px rules for the HUD,
   the legend and the probe. The UX specification's own floors are 320 px for the
   canvas and 480 px for the table pane, larger than the 220 px kept here as directed;
   which number governs is for ROOT to reconcile (document §8 item 3). The geometry is
   drawn only as proportional strips in the specimen; no frame draws the column
   collapsing or the slide-over fallback. Open item 3 asks the second mock pass for
   state 4 with the column open.
4. The chip policy assumes the engine reports `MODEL_INCOMPLETE` before any run
   (open item 9); "Human review required" on the Review page only awaits packet item 4
   (decision 3); the run log's four-row grouping assumes the log can be grouped by case
   (open item 10).
5. "Review/signoff block" is used as registered text on the strength of RESEARCH-C §4
   row M-07 (from `docs/PRD.md` §19.2); the copy rules ban "sign-off" as a control and
   name the heading as the one exception (P-10). The PRD itself was not re-read in this
   revision; ROOT should confirm the reading.
6. The rail captions "Failed" and "Stale", the stale band's sentence, the Run tooltip's
   "Run is unavailable — …", the run log's wording and the empty state are this system's
   wording taken from the frames, state names and not registered text (open item 7,
   §7.1).
7. The frames' column widths (P-4) were transcribed into §5.1 from `tools/ui.mjs`; the
   rendered frames were not re-opened, so a width that the generator overrides per
   frame would not have been seen.
8. The specimen composes moments: the shell shows the after-model-change state with
   the geometry strips where a canvas would be; the results header shows Envelope on,
   the stale band and, as a labelled variant, the historical band at once; the Loads
   table shows a selected case with its expansion open. Each is labelled as a sample.
9. The comment glyph on a referenced live row (§4, §5.5) is specified but not drawn in
   the specimen's Review live block, which has no gutter.
10. Fonts as in V1: SF Pro Text and SF Mono rendered on this Mac; the declared
    fallbacks were not visually checked elsewhere.
11. Contrast is measured as the WCAG 2.x ratio only; the categorical CVD validation is
    the validator's and unchanged from V1.

## 8 Where the brief could not be satisfied

Nothing knowingly. Two partial points: the dark scale is one step down at its bottom but
its top moved by 0.12 rather than 0.07 (item 1 above, explained in §2.6); and the
specimen shows the docked-inspector geometry as proportional strips rather than a drawn
canvas, which the second mock pass is asked to draw.

## 9 Revision after ROOT's review of V1.1, same day

ROOT's review found one conflict and asked for one addition; both are applied in place
and recorded as §9 rows 48 and 49 of the document. Only files under
`instances/DESIGN-SYSTEM/` changed; `tokens.json` is untouched (hash as in §2).

1. **The docked inspector's narrow case** now follows `{RUN}/instances/UX-SPEC/UX_SPEC_V1.md`
   §10.9, which ROOT adopts: when opening the docked inspector would take the canvas
   below its minimum, the agent column collapses to its strip first (⌘⇧G re-opens it);
   if the canvas would still fall below its minimum, the inspector opens as a
   slide-over over the canvas for that window size only, with the tooltip "Docked
   inspector needs a wider window". The canvas is never collapsed to an edge tab. The
   220 px minimum, the camera rule and the 400 px HUD, legend and probe rules stay.
   Revised: the §0 paragraph and the narrow-case table's last column (1440 with the
   column open: the column collapses, canvas 303 as with the strip; 1280 with the column
   open: canvas 231), §5.3, §8 item 3 and the second-pass mock list, the status line,
   §9 row 1 and the new row 48; in the specimen the third geometry strip now draws
   table 737 / canvas 303 / inspector 300 / strip 44 after the column has collapsed, its
   caption states the rule, and the edge-tab element and its style are removed. The
   phrase "edge tab" survives in the document only in row 48's record of what it
   replaced.
2. **Spelling rule** added to §7.1: product copy uses Canadian English ("Analyze",
   "colour", "centre"); the registered sentences keep their own spelling. Row 49.

Confirmed by ROOT without change: the reading of decision 11 (top under the edge, bottom
one step down) and "Review/signoff block" as registered text.

Checks re-run after the revision: `agree.mjs` reports `problems: []` (zero differences)
against the hashes now in §2; `render.mjs` in the same four contexts reports 0 requests,
0 console errors or warnings, no horizontal scroll at 1440 or 720, no panel overflow,
smallest font 11 px, 98 token rows and 146 contrast rows, three geometry strips, every
V1.1 element present and on its token in both themes, and the manual switch working
each way; section 9's image was inspected in light at 1440. No file under the instance
contains an absolute filesystem path; a read-only `git status` shows only files under
`instances/DESIGN-SYSTEM/` modified by this task.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
