# RETURN — CONCEPTS-01: north-star concept directions

Role: HELPS_HUMANS design manager, phase 2 of the SWB Piping Designer interface program. Sealed brief: `briefs/CONCEPTS-01_north_star_directions.md`. Worked alone; no delegation, no subagents. Date: 2026-09-17.

## What was read, in the brief's order

All paths are relative to `{REPO_ROOT}` (the ROOT worktree; absolute path substituted by ROOT on retention, as declared in `briefs/_INDEX.md`). Run record root: `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM/`.

1. `briefs/CONCEPTS-01_north_star_directions.md` — the sealed brief, in full.
2. `instances/ROOT/DESIGN_BRIEF_V1.md` — the accepted V1.1 brief, in full.
3. `instances/RESEARCH/D_domain_ux_research.md` — in full.
4. `instances/RESEARCH/A_ux_audit.md` — sections "Preserve list" and "What the owner is probably reacting to (ranked)" in full; the "Findings table" in full for reference. The numbered sections 1 to 10 were not read.
5. `instances/RESEARCH/C_ui_constraints.md` — sections 1.1, 1.5, 1.7, 3 (3.1 to 3.4) and 4 (including the negative disclosure rules), in full. Sections 1.2 to 1.4, 1.6, 1.8 to 1.10, 2 and 5 were not read.
6. `instances/ROOT/OPENING_REPORT_2026-09-16.md` and `instances/ROOT/RENDERING_BOUNDARY_ANSWER_2026-09-17.md` — in full.
7. Optional grounding, read only: `projects/chirality-piping/apps/desktop/src/App.tsx` (the `WORKSPACE_SECTIONS` table, lines 407 to 460, and the four references to it) and `projects/chirality-piping/apps/desktop/src/features/toolkit/capabilityCatalog.ts` (the entry ids, labels and groups). Nothing else in the product source was read.

Not read: `instances/RESEARCH/B_ui_inventory.md` (not in the brief), the governance documents cited by C (`docs/CONTRACT.md`, `docs/PRD.md`, `docs/claims_registry.md` and others); constraints were taken from C's extraction as the brief directs.

## What was produced

All under `instances/CONCEPTS/`:

- `CONCEPT_DIRECTIONS_V1.md` — the main document. Section 0 fixes what all directions share; sections 1 to 3 give the three directions (A Layout Sheet, B Workbench, C Run Book), each with thesis, borrowings and departures, shell in three layouts with canvas share estimates, storyboard of the eight states with the agent panel's place and how a proposal appears, a disclosure table for M-01 to M-17, the 3D presentation language in outline, and risks; section 4 compares; section 5 recommends C with reasons and states what would change the advice; section 6 names decision-packet items without proposing governed-text changes; section 7 indexes the wireframes.
- `wireframes/` — nine SVGs, one per direction per layout, for state 7 (results on the model and in the table): `layout_sheet_{model,grid,split}.svg`, `workbench_{model,grid,split}.svg`, `run_book_{model,grid,split}.svg`. Each is a 1440 × 900 viewBox, greys only, boxes and labels, drawn from the same region geometry the document's shell tables state. Each carries an SVG `<title>` and a status-bar caption naming the direction, layout and canvas share.
- `RETURN.md` — this file.

The wireframes were generated from a small Python script kept in the session scratchpad (not in the repository), so the nine files share one geometry and one label set. Verification done: all nine parse as XML; each was rendered and inspected in a browser at about 570 px wide (smaller than half size), where the region titles, tab strips, table headers and values, probe card, legend and status bar are all readable. One defect found and fixed during that check: the results tab strip overflowed its region in the narrower layouts; it now truncates behind an overflow marker. All nine renders were inspected after the fix.

## Model and effort actually used

Model: Claude Fable 5.1 (`claude-fable-5-1`), as the brief requested. Reasoning effort: the session's configured setting (25), not raised. One agent, one pass, no delegation. Tools used: file reading, one scratchpad Python script for the SVGs, qlmanage and the browser pane for rendering checks, a local HTTP server on 127.0.0.1 for the browser check (stopped after use). No git command that changes state was run. No file outside `instances/CONCEPTS/` was created or modified; the script, renders and contact sheet live only in the session scratchpad.

## Uncertain, or judgement calls the owner may want to overturn

1. **Canvas share figures are estimates** built on an assumed 52 px unified title-and-toolbar band and a 24 px status bar. Real macOS chrome and the eventual toolbar height will move the numbers by a few points. The ratios between directions are more reliable than the absolute values.
2. **Direction B's Grid layout** puts the live canvas above the inspector in the right column at 26% of the window (38% with the inspector collapsed). The brief's layout table has the canvas as the right surface but does not say where the inspector goes; this is recorded as B's one departure. It is the weakest layout of the nine.
3. **M-01, the maturity line.** The constraints as extracted still bind it to the app shell; the accepted brief says the language need not exist and sends it to the decision packet. All three directions place it on demand (About) and name a status-bar popover as the fallback until the registry entry is revised. If the owner wants the shell placement to stand unchanged, the fallback is the home.
4. **Short status labels** on the wireframes ("Mechanics solved", "User-rule checked", "Internally verified") stand in for the raw tokens, with the raw token intended to be one click away. Whether these count as listed short variants under the registry is a registry question, named as decision-packet item 4; no new vocabulary is proposed.
5. **Hanger design placement.** A puts it on its own sheet, B in the bench drawer, C on the Results page (after the design pass has run). C's placement follows the Aspect Pipe Stress habit of hanger design as part of the analysis; an owner who thinks of hangers as supports may prefer it under Model.
6. **Loads as a stage in C** separates a load from the element row it acts on. The document names this as C's second risk and the mitigation (marked cells on the Model page open the same row); it is unproven until mocks.
7. **Sample values** in the wireframes (stresses, allowables, rule IDs such as EXP-A1, pack 1.2, run 03, project "Loop 4 header") are layout placeholders, not engineering data, and must not be carried into mocks as if real.
8. **Secondary captions at 12 px** (rail and tool-strip captions) are small at half size; everything read repeatedly is 15 px or larger in the SVG units.
9. **The recommendation** is advice, and it leans on the reading that the owner's user runs and reviews whole analyses rather than living in the canvas; section 5 states what would change the advice to B or A.

## Where the brief could not be satisfied

No requirement was left unmet. Two points are near the line and are stated rather than hidden: B's Grid layout canvas share (item 2 above), and the M-01 home, which depends on the owner's decision packet (item 3 above).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
