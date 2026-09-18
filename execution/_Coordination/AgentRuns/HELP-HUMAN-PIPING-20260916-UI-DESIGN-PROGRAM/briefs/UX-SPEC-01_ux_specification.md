# Sealed brief — UX-SPEC-01: the UX specification

Sealed by ROOT (HELP_HUMAN) on 2026-09-18 before launch, after the owner approved the fourteen mock-review recommendations. Role: HELPS_HUMANS design manager, phase 3 of the SWB Piping Designer interface program. Model requested: Claude Fable 5.1 (extensive analysis and judgement). Mechanism: Claude Code `Agent` tool, general-purpose type, background. Type 2 does not delegate; this child works alone.

Path placeholders: `{REPO_ROOT}` is the ROOT worktree; `{RUN}` is `{REPO_ROOT}/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM`.

## Purpose

Write the UX specification of SWB Piping Designer: what every surface does, in what states, in response to what, and which operation of the existing product each control invokes. It is the document the implementers read after the baseline characterization tranche (D-70) when the redesign is built, and the document the acceptance-criteria proposal (D-70 effect 6) will be written against. It specifies behaviour; the design system specifies appearance; the mock frames show the two together. It is not an implementation plan and chooses no technology.

## Accepted basis, read in this order

1. `{RUN}/instances/ROOT/DESIGN_BRIEF_V1.md` — V1.4: §2 to §7. Every behaviour must satisfy it.
2. `{RUN}/instances/ROOT/DIRECTION_DECISION_2026-09-17.md` — §2 the direction, §5 the governance touchpoints, §10 the agreed recommendations, §11 the fourteen mock-review decisions. Where the design system V1 and §11 differ, §11 wins (a V1.1 revision applying §11 is in progress in parallel; do not wait for it).
3. `{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md` — §0 (shell), §4 (marks), §5 (components, including the table's keyboard model and cell states), §7 (copy rules and disclosure homes), §8 (open items).
4. `{RUN}/instances/MOCKS/frames/index.html` and the fifteen frames, `{RUN}/instances/MOCKS/MOCKS_V1.md` and `sample_model.md` — the states as drawn; the frames are the worked examples the specification must explain.
5. `{RUN}/instances/RESEARCH/E_caepipe_format.md` — §3 and §6: the layout grammar, the attachment fields, the mapping and its sixteen gaps, for the table columns and the export.
6. `{RUN}/instances/RESEARCH/C_ui_constraints.md` — §1, §3 and §4 in full: the constraints, the operation model the product has (§3), and the mandatory disclosures.
7. `{RUN}/instances/RESEARCH/B_ui_inventory.md` — the inventory of the current product's commands, surfaces and operations, to learn what the engine and the typed interfaces already provide.
8. Product source, read-only, for the real names of operations and types: `{REPO_ROOT}/projects/chirality-piping/apps/desktop/src/features/toolkit/capabilityCatalog.ts`; `{REPO_ROOT}/projects/chirality-piping/apps/desktop/src/App.tsx` (the `WORKSPACE_SECTIONS` table and the command surfaces); the structured-operation layer and its author type (find it from C §3 item 11 and B's inventory); `{REPO_ROOT}/projects/chirality-piping/schemas/` for the model, run and report schemas; `{REPO_ROOT}/projects/chirality-piping/docs/TYPES.md` §4 for the status vocabulary. Read what you need to name things correctly; do not read the current UI's components for behaviour to copy, since nothing in the current presentation carries over.

## What to deliver

Write to `{RUN}/instances/UX-SPEC/`:

### `UX_SPEC_V1.md`

In this order:

1. **Scope and reading guide.** What the document specifies, what it leaves to the design system and the engine, how to read a state table.
2. **The model of the interface.** One spine: model, selection, issues, run, revision. The tables as the model and the canvas as derived. Stages and views, with the per-stage memory rule and the first-open defaults. The one structured-operation route for every mutation (typed row, gesture, paste, accepted proposal), the undo model (one checkpoint per operation, a paste and an accepted proposal each one operation), and what happens to results when the model changes (stale, historical run kept).
3. **The tables.** The layout table in the CAEPIPE grammar: every column, its type, unit, precision, validation, propagation rule and origin behaviour; From shown and implied; branches; element types and their type-specific fields; the Load set column and its read-through T and P values with the edit-or-fork rule; numeric node IDs with insert, split and renumber behaviour; the marks column. The attachment tables (restraints, loads, node data including branch-connection kinds) with their columns, and the joined-row expansion. The load cases table (generated, edited, authored; expressions display-only; the combination row expansion). The results tables in reviewer order with the case selector, Envelope, sort and filter. The hanger table. For each table: selection, keyboard model, paste with mapping, filters (unchecked, agent-origin, changed since), empty state.
4. **The canvas.** What it shows per stage and view; selection and cursor sync; the probe; the authoring gestures (direct distance entry with the compass, click a node to add a restraint) and the rows they write; labels, legend, deformation, ghosts; camera presets and the report figure; what persists across view and stage switches.
5. **The inspector, issues, run and status.** The inspector's sections per selection kind; docked in Model and Both (decision 1). Issues: classes, the drawer, filters, links to entities, counts on the rail and status bar. Run: the button, its disabled reasons, progress, failure with the banner once and the drawer row, the run record, immutability and naming. The status chip policy (decisions 2 to 4) with the authority domain popover.
6. **The agent collaboration protocol.** The panel (column and strip); conversation; the proposal lifecycle from draft to accepted or rejected, per row, multi-row and batch; how a proposal lands in the tables and the canvas; rationale, constraints considered and TBD items; the accepted record; what a proposal may and may not do; the agent's checks and open issues by reference; the Checked mark (per row, multi-row, stale on change, the unchecked filter, who may set and clear it); origins per row and cell and the changes-since control.
7. **The Review page.** The three columns; the report outline in the required order with the fixed sections; section content with live tables and authored text; iterations as named snapshots with a diff toggle; edits shown or hidden; the comment stream (kinds, references, resolve, reopen); Report preview and Export; where the acceptance sentence and the required notices appear; what is and is not editable.
8. **Libraries, rules, export and handoff.** Libraries import (materials, sections, hanger tables, rule packs) with provenance; the rules editor with display-only expressions; the export to the CAEPIPE model batch file as a projection, with the sixteen gaps from E §6 stated as behaviours (what is written, what is dropped, what is transformed) and the export metadata; handoff.
9. **Disclosures and vocabulary.** Every mandatory disclosure M-01 to M-17 with its surface, trigger and wording source; the status vocabulary; the copy rules that bind every string.
10. **Empty, error and edge states.** New project; a model with no sections; a failed run; a reopened historical run; a proposal whose row was edited before acceptance; a paste with unmapped columns; a stale Checked mark; a library missing; window at the minimum size.
11. **Open questions for engineering**, each stated as a behaviour the design needs and a question about the engine or the typed interfaces.

### `OPERATIONS_MAP.md`

A table: every control and gesture in the specification → the operation it invokes → the existing operation id, capability entry or typed interface that provides it (with the source file and line), or **GAP** when the product has no such operation, with one line on what the gap is. Group by surface. At the end, the list of gaps, since that list is what the implementers estimate from.

### `RETURN.md`

What was read with paths and sections; what was produced; model and effort actually used; every uncertainty; any place the brief could not be satisfied.

## Constraints that bind

- Every string in the specification obeys the design system's copy rules and the constraints sheet: Accept and Reject, never Approve; the six statuses with their authority domains; no certify, seal, approve, authenticate, comply or compliant; ratios as number, rule ID and pack version; no boundary sentence repeated across product surfaces.
- The product is SWB Piping Designer, SWBPIPE for short; the old name appears only when quoting a source or naming an existing identifier.
- The tables are the model; every mutation is one structured operation through the one route; the canvas authors nothing that is not a row.
- Stages Model, Loads, Results, Review; views Table, Model, Both; numeric node IDs; tees as node data; load sets read through on the row; acceptance row by row with multi-row and batch; the Checked mark kept.
- Mouse driven and keyboard fluent; the whole table and every form operable from the keyboard.
- Hanger design under Results; stress isometrics and run comparison excluded, and the data model must not preclude them.
- The engine stays; the specification names what the interface asks of it, never how it is built.
- Nothing here is a product claim or an acceptance; the specification describes intended behaviour of a redesign that is not built.

## Exclusions

- Write only under `{RUN}/instances/UX-SPEC/`. No product source, test, harness or governance file is modified.
- No implementation plan, no technology choice, no code, no estimates.
- No new claims vocabulary and no proposal to change governed text; name any need as an item for the owner's decision packet.
- No delegation.

## Acceptance

ROOT accepts when: every surface in the brief §4 and every frame has its behaviour specified with states and transitions; every table has its columns, keyboard model and validation; the proposal lifecycle, the Checked mark and the agent's checks are specified end to end; every mandatory disclosure has its trigger and wording source; the operations map covers every control and names every gap with a source line for every non-gap; the open questions are behaviours, not technology choices; RETURN is complete.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
