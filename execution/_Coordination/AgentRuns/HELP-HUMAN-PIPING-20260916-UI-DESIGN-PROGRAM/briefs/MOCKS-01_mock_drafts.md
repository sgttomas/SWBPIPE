# Sealed brief — MOCKS-01: mock drafts of the nine workflow states

Sealed by ROOT (HELP_HUMAN) on 2026-09-18 before launch, under the phase 3 sequence the owner agreed on 2026-09-18 (design system first, then the mocks from it). Role: HELPS_HUMANS design manager, phase 3 of the SWB Piping Designer interface program. Model requested: Claude Fable 5.1 (quality and judgement matter). Mechanism: Claude Code `Agent` tool, general-purpose type, background. Type 2 does not delegate; this child works alone.

Path placeholders: `{REPO_ROOT}` is the ROOT worktree; `{RUN}` is `{REPO_ROOT}/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM`.

## Purpose

Draft the fifteen frames of the nine workflow states as static, self-contained HTML pages drawn from the design system and one sample model, so that the owner can open each frame in a browser, in light and dark, and judge the design of SWB Piping Designer as a stress engineer would judge a screen: is this where I would do this work, and is everything I need here.

## Accepted basis, read in this order

1. `{RUN}/instances/ROOT/DESIGN_BRIEF_V1.md` — V1.4, confirmed. §4 the product shape, §7 the mock states and the frame formula, §5 the vocabulary, §6 the constraints.
2. `{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md` — the whole document; `tokens.json` is the single source of every colour, type and spacing value; `specimen.html` is the reference rendering of every component and mark, and its inline CSS is the starting point for the frames. §8 lists what each state must test.
3. `{RUN}/instances/ROOT/DIRECTION_DECISION_2026-09-17.md` — §2 the direction, §10 the agreed recommendations (load sets read through on the row, the load mark opening the load row in place, first-open views, origins quiet and proposals loud, the agent column never covering the tables, hanger design under Results, the Review page in three columns, the checked mark kept).
4. `{RUN}/instances/CONCEPTS/CONCEPT_DIRECTIONS_V1.md` — §1.4, §2.4 and §3.4, the storyboards, for what the engineer sees and does in each state; the chosen direction is the synthesis in the direction record §3, not any one of the three.
5. `{RUN}/instances/RESEARCH/E_caepipe_format.md` — §3.2 to §3.4 for the layout grammar, so the sample rows are the rows a CAEPIPE veteran expects.
6. `{RUN}/instances/RESEARCH/C_ui_constraints.md` — §4, the mandatory disclosures, and the design system §7 for where each one lives.

Nothing from the current product's presentation carries over. Anything under `{REPO_ROOT}/projects/chirality-piping/apps/` is read-only and only for a fact about an operation the product actually has.

## What to deliver

Write to `{RUN}/instances/MOCKS/`:

### `sample_model.md`

One small fictional piping system used in every frame, so the states read as one project: a title, the units, the node rows in the layout grammar (numeric nodes, From, Type, DX DY DZ, Section, Material, Load set), the sections and materials, the load sets, the restraints and loads with their nodes, the load cases as generated from a sample rule pack with one edited and one authored row, one hanger location, and the result values that state 7 shows (stresses, allowables, ratios with rule IDs and a pack version, a governing case). Values must be plausible to a stress engineer and labelled as sample; they are placeholders, not engine output, and must not be presented as real results.

### `frames/`

Fifteen HTML files, named `s<state>_<view>_<theme>.html`:

| State | Frames |
|---|---|
| 1. New project, the first node row | `s1_table_light` |
| 2. Routing in the canvas with direct distance entry, rows appearing in the drawer | `s2_model_light`, `s2_model_dark` |
| 3. Editing the layout table: propagation, keyboard entry, the paste band | `s3_table_light` |
| 4. Restraint and load tables, marks on the node rows, glyphs with direction arrows; the inspector slide-over open in Both | `s4_both_light`, `s4_table_light` |
| 5. Load cases: generated rows edited, an authored case beside them; the display-only expression | `s5_table_light` |
| 6. Running, then the failure banner, the issues drawer and the status chip | `s6_both_light` |
| 7. Results: the stress table with ratios, the case selector and Envelope, the evidence chip, the results header disclosure; the coloured model with legend and probe | `s7_both_light`, `s7_table_light`, `s7_both_dark` |
| 8. Hanger design against a user-supplied library; an agent proposal landing as proposed rows, accepted row by row; the ghost in the canvas | `s8_table_light`, `s8_model_light` |
| 9. The Review page: three columns, live tables, edits shown and hidden, two iterations, the comment stream, the acceptance sentence once at the top | `s9_table_light`, `s9_table_dark` |

Each frame is the whole window at 1440 × 900: the toolbar band, the stage rail with the right stage current, the surfaces in the named view, the agent column or strip, the status bar, with the shell geometry of the design system §0 and every value from `tokens.json`. Each frame fits a smaller viewport by scaling its 1440 × 900 stage down as a whole, never by reflowing, so that the owner can open it in a narrower pane and see the frame entire. Static: nothing needs to work, but hover cards, open popovers, the open disclosure and the open drawer are drawn in the state the storyboard describes. The canvas in a frame is drawn as a schematic figure in inline SVG in the design system's figure language (§6), labelled as a mock rendering and not the engine's. The frames may share `tokens.css` and `mocks.css` in the same directory by relative link; no frame makes a network request.

### `index.html`

A contact page in the same directory linking every frame with its state, view and theme and one line on what to look at, and a link to the sample model.

### `MOCKS_V1.md`

For each frame: what it shows, which design system §8 item it tests, and every decision the frame makes that the design system did not, recorded as a proposal for the design system. A list of the places the frame departs from the design system, with the reason. A list of questions for the owner that only a screen can raise. Tokens proposed, if any component needed a value the token file does not have; in the frames, use the nearest existing token and list the gap here.

### `RETURN.md`

What was read with paths and sections; what was produced; model and effort actually used; how the frames were verified; every uncertainty; any place the brief could not be satisfied.

## Design constraints that bind

- Every colour, type and spacing value comes from `tokens.json`; no new colour appears in a frame. Numbers are tabular, right-aligned, with units in headers.
- The product is SWB Piping Designer, SWBPIPE for short. The old name appears nowhere.
- The copy rules of the design system §7: Accept and Reject, never Approve; the six statuses as short labels with the raw token one click away; no certify, seal, approve, authenticate, comply or compliant; ratios as number, rule ID and pack version; Checked as a human tag.
- Every mandatory disclosure that belongs on a shown surface appears exactly once, where the design system §7 puts it, in its registered wording; the acceptance sentence appears once on the results surface (in its header disclosure), once at the top of the Review page, and nowhere else. The maturity line follows decision-packet item 1 as the design system placed it.
- The tables are the model; the canvas is derived. Stages Model, Loads, Results, Review; views Table, Model, Both with the first-open defaults. Numeric node IDs. Tees as node data. Load sets read through on the row. Acceptance row by row with multi-row and batch. The checked mark kept.
- The agent column never covers the tables; proposals are read in the tables.
- Dark and light are peers: the dark frames are the same screens under the dark tokens, not a restyling.
- Hanger design under Results; stress isometrics and run comparison appear nowhere.
- Accessibility: no conformance claim, no target named.
- Verify the frames before returning: open every frame headlessly with network requests blocked (Playwright is available read-only in `{REPO_ROOT}/projects/chirality-piping/node_modules`, as the design-system child used it; its script `{RUN}/instances/DESIGN-SYSTEM/tools/render.mjs` shows the pattern), at 1440 and at a narrower width, in the frame's theme; fix overflow, collisions and illegible text; state what was checked in RETURN.

## Exclusions

- Write only under `{RUN}/instances/MOCKS/`. No product source, test, harness or governance file is modified; nothing under `instances/DESIGN-SYSTEM/` is modified (propose changes in `MOCKS_V1.md` instead).
- No implementation, no component library, no framework code; plain HTML, CSS and inline SVG, and only the script needed for the theme and the scaling.
- No new claims vocabulary and no proposal to change governed text; name any need as an item for the owner's decision packet.
- No delegation.

## Acceptance

ROOT accepts when: all fifteen frames exist, open offline and scale to fit; each shows the state its storyboard describes in the named view and theme with the whole shell; one sample model runs through all of them; every value traces to the token file; every mandatory disclosure that belongs on a shown surface appears once in the right place and no forbidden word appears; `MOCKS_V1.md` records each frame's decisions, departures and questions; RETURN states what was verified and what is uncertain.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
