# Sealed brief — DESIGN-SYSTEM-01: design system for SWB Piping Designer

Sealed by ROOT (HELP_HUMAN) on 2026-09-18 before launch, on the owner's instruction of the same day ("seal the design-system brief"). Role: HELPS_HUMANS design manager, phase 3 of the SWB Piping Designer interface program. Model requested: Claude Fable 5.1 (quality and judgement matter). Mechanism: Claude Code `Agent` tool, general-purpose type, background. Type 2 does not delegate; this child works alone.

Path placeholders: `{REPO_ROOT}` is the ROOT worktree; `{RUN}` is `{REPO_ROOT}/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM`.

## Purpose

Define the design system from which the mock drafts of the nine workflow states will be drawn, complete enough that the mocks child invents nothing foundational: the foundations, colour in light and dark as peers, iconography, the marks vocabulary, the components including the table, the shell, the agent panel and the Review page primitives, the 3D presentation language, and the copy rules that keep every label inside the claims boundary.

## Accepted basis, read in this order

1. `{RUN}/instances/ROOT/DESIGN_BRIEF_V1.md` — V1.4, confirmed by the owner. Every decision must satisfy it. §4 is the product shape; §7 the mock states the system must serve; §5 the vocabulary; §6 the constraints and the decision-packet items.
2. `{RUN}/instances/ROOT/DIRECTION_DECISION_2026-09-17.md` — §2 the direction, §3 what carries from the three concept directions, §5 the governance touchpoints, §8 to §10 the owner's word and the eleven agreed recommendations.
3. `{RUN}/instances/CONCEPTS/CONCEPT_DIRECTIONS_V1.md` — §0 fixed points; §1.3, §2.3 and §3.3 for region sizes at 1440 × 900; §1.6, §2.6 and §3.6, the three presentation languages to choose between; §1.5, §2.5 and §3.5, the disclosure homes. The wireframes `{RUN}/instances/CONCEPTS/wireframes/*.svg` show the results state; the chosen direction is not any one of them, see the direction record §3.
4. `{RUN}/instances/RESEARCH/E_caepipe_format.md` — §3.2 to §3.4 and §6.1 to §6.2, so that the layout table's columns, the element types, the branch-connection kinds and the attachment fields are named as the target file names them.
5. `{RUN}/instances/RESEARCH/C_ui_constraints.md` — §1.1, §3 and §4 (the constraints and the mandatory disclosures M-01 to M-17).
6. `{RUN}/instances/RESEARCH/A_ux_audit.md` — the "Preserve list" only; `{RUN}/instances/RESEARCH/D_domain_ux_research.md` — §2 and §6.

Nothing from the current product's presentation carries over: do not read the product's stylesheets or components for style. Anything under `{REPO_ROOT}/projects/chirality-piping/apps/` is read-only and needed only if a fact about the engine's operations is required.

## What to deliver

Write to `{RUN}/instances/DESIGN-SYSTEM/`:

### `DESIGN_SYSTEM_V1.md`, in this order

1. **Foundations.** Type: the families for macOS with declared fallbacks (a system interface face for chrome; a face with tabular numerals for every number; if a web font is proposed, name it and its licence, and the specimen must not depend on it); the size scale with the smallest size read repeatedly stated; line heights; tabular numerals wherever a number appears. Spacing scale, radii, borders, elevation, focus rings, motion (minimal: drawers, disclosures, a proposal arriving). One density, tuned for tables of numbers, calm and professional, a CAE tool and not a web form.
2. **Colour.** Semantic tokens, each with a light and a dark value: surfaces, text, borders, selection, hover, focus, disabled; the six automatic statuses (M-08) as short-label chips; the issue classes (M-10) and the required-field mark; the marks of §4; proposal old and new; Checked and stale; historical; display-only. The result colour scale for ratio and for stress as a sequential scale with its legend semantics: a scale, never a verdict, and nothing coloured that has not been solved. Load-vector and case colours. The canvas palette in both themes: background, ground grid, pipe neutral, edge line, glyphs, vectors, proposal ghost, deformation ghost, selection and hover. Contrast: compute and report the ratio of every text and mark pairing in both themes as findings; make no conformance claim and name no target, since the target is the owner's to set and is TBD (M-16). Load the `dataviz` skill for the method of building and validating the sequential scale and the categorical set.
3. **Iconography.** The stage rail (Model, Loads, Results, Review, and at its foot Libraries, Rules, Issues), the views (Table, Model, Both), the canvas HUD (fit, view presets, section, isolate, hide, labels, deform, probe, route, restrain), table actions, and the marks. Style rules: grid, stroke, corners, optical sizes. Every icon has a text label or a tooltip.
4. **The marks vocabulary.** For each mark: glyph, token, placement (row gutter, cell corner, marks column, row band), tooltip content, keyboard reveal, and the filter it belongs to. Marks: the origins (entered, accepted from a proposal, propagated, generated, imported); proposed (a row band with old and new values, and the cell-level old and new); Checked and stale Checked; the attachment marks on the node row (restraint, load, node data); the issue marks by class with the required-field mark; historical run (M-13); display-only expression (M-15). The rule from the direction record §10: origins quiet, proposals loud.
5. **Components.**
   - The table: header with units (M-11); sort and filter; the marks column; the row gutter; cell states (entered, propagated, read-through from a load set, proposed, selected, editing, invalid, stale); the paste band with column mapping; the keyboard model (Tab, Enter, arrows, Escape, the key that opens a joined row, multi-row selection); the footer; row expansion for a joined attachment row in Table view; the case selector and Envelope toggle on results tables. Specify the layout table's columns in the CAEPIPE grammar: Node, From (shown, quiet when implied), Type, DX DY DZ or X Y Z, Section, Material, Load set with its read-through T and P values, marks.
   - The stage rail with counts; the view switch; the toolbar band (wordmark, project name and save state, view switch, Run with progress and its disabled reason, Issues count, Agent toggle, display units, palette search); the status bar (the M-08 chip with the authority domain in its popover, issues, selection, units).
   - The inspector (Model and Both views); the issues drawer; the drawer that holds the stage's current table in Model view.
   - The agent panel: a right column collapsible to a strip, present in every stage, never covering the tables. Conversation; the proposal queue as diff cards; Accept and Reject per row, multi-row and whole batch; rationale, constraints considered and TBD items on each proposal; the accepted record; the agent's checks.
   - The Review page primitives, three columns: the report outline in the required order; the section content with tables pulled live from Results and text the engineer writes; the comment stream holding the agent's checks and open issues attached by reference to rows, results and report text, never altering the tables. Iterations as named snapshots with a diff toggle; edits shown or hidden; the acceptance sentence once, at the top.
   - The probe card; the legend; the routing compass and direct distance entry; dialogs (Libraries import carrying M-05; export and handoff carrying M-04 and M-06; report preview carrying M-03 and M-07); About carrying M-01 and M-17; Preferences with the M-16 findings.
6. **The 3D presentation language.** One specification, choosing between or combining the three languages in the concept return with the reasons stated: pipe at real outside diameter with its shading and edge treatment; fittings; restraint glyphs that show direction, gap and friction; load vectors with a scale reference and labels; node labels with the budget rule; selection and hover; result colour on the tube with the legend; deformation with the undeformed ghost and a stated, adjustable scale; proposed changes as ghosts; camera presets shared with the report figures; both themes. Describe what the model looks like, not how it is drawn; the engine stays.
7. **Copy rules.** The words allowed and forbidden on buttons, chips, labels and generated text: Accept and Reject for proposals, never Approve; the six statuses as short labels with the raw token one click away; never certify, seal, approve, authenticate, comply or compliant; ratios as number, rule ID and pack version; Checked as a human tag and its tooltip wording; the disclosure homes for the chosen direction, one placement of the acceptance sentence per surface class (results header disclosure, Review page top, report); the product name SWB Piping Designer and SWBPIPE.
8. **Open items** and what the mocks must test.

### `tokens.json`

Every token in the document, machine-readable: colour tokens as `{"name": {"light": …, "dark": …}}`, type and spacing tokens as plain values. The document and the file must agree; say in RETURN how that was checked.

### `specimen.html`

One self-contained page: inline CSS and script, no network requests, system fonts with the declared fallbacks. It renders the foundations; every colour token in both themes, following the system preference with a manual switch; the table component with every mark and cell state on sample rows in the layout grammar (numeric nodes, From, Type, DX DY DZ, Section, Material, load set read-through, the marks column); the status chip; a proposal diff card; the probe card; the legend; the stage rail and the view switch. Legible at 1440 wide and at half size. It is a specimen, not an implementation: no product code, no component library, no framework. Sample values are placeholders and are labelled as sample.

### `RETURN.md`

What was read, with paths and sections; what was produced; model and effort actually used; every uncertainty; any place the brief could not be satisfied.

## Design constraints that bind

- The product is SWB Piping Designer, SWBPIPE for short. The old name appears only when quoting a source.
- No label, chip, badge or generated text asserts certification, sealing, approval, authentication or code compliance. Statuses use the accepted authority-attributed vocabulary. Ratios show number, rule ID and pack version.
- Units on every value. Provenance on demand. One placement of the acceptance sentence per surface class.
- The tables are the model; the canvas is derived. Stages Model, Loads, Results, Review. Views Table, Model, Both, with the first-open defaults in the brief. Numeric node IDs. Tees as node data. Temperatures and pressures as named load sets read through on the row. Acceptance row by row with multi-row and batch; Accept, never Approve. The checked mark is settled: kept, called Checked, stale on change, with an unchecked filter.
- The agent is a visible collaborator in a right column that collapses to a strip; the tables and the model are the hero; the engineer is never demoted.
- Mouse driven and keyboard fluent: the table and every form fully operable from the keyboard in the spreadsheet idiom.
- Dark and light are peers; design nothing that works in only one.
- Hanger design in scope under Results; stress isometrics and run comparison excluded.
- The rendering engine stays; describe appearance, not drawing.
- Accessibility findings as warnings; no conformance claim; no numeric target named (M-16).

## Exclusions

- Write only under `{RUN}/instances/DESIGN-SYSTEM/`. No product source, test, harness or governance file is modified.
- No implementation plan, no component library choice, no framework code. The specimen is plain HTML, CSS and a little script for the theme switch.
- No new claims vocabulary and no proposal to change governed text; name any need as an item for the owner's decision packet.
- No delegation.

## Acceptance

ROOT accepts when: every colour, type and spacing value used by a component is a token with light and dark values; every mark has glyph, token, placement, tooltip and keyboard reveal; the table is specified to the cell state with the layout columns named; the agent panel and the Review page primitives are specified; the presentation language is one specification with the alternatives stated; the specimen renders offline and agrees with the document and the token file; the copy rules are stated; contrast ratios are reported as findings only; RETURN lists what was read and what is uncertain.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
