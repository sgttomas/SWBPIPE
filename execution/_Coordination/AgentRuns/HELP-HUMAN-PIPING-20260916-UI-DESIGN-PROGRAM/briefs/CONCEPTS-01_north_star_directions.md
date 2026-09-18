# Sealed brief — CONCEPTS-01: north-star concept directions

Sealed by ROOT (HELP_HUMAN) on 2026-09-17 before launch. Role: HELPS_HUMANS design manager, phase 2 of the SWB Piping Designer interface program. Model requested: Claude Fable 5.1 (quality and judgement matter). Mechanism: Claude Code `Agent` tool, general-purpose type, background. Type 2 does not delegate; this child works alone.

## Purpose

Produce two or three north-star concept directions for the SWB Piping Designer human interface, each strong enough that the owner can choose one and the design system and mock drafts can follow from it without reopening the brief.

## Accepted basis, read in this order

1. `{REPO_ROOT}/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM/instances/ROOT/DESIGN_BRIEF_V1.md` — the accepted V1.1 brief. Every direction must satisfy it. Where a direction departs from it, say so and why.
2. `instances/RESEARCH/D_domain_ux_research.md` — what stress engineers expect; CAEPIPE and Aspect Pipe Stress interaction models; borrowable patterns.
3. `instances/RESEARCH/A_ux_audit.md` — sections "Preserve list" and "What the owner is probably reacting to" only; the findings table for reference.
4. `instances/RESEARCH/C_ui_constraints.md` — sections 1.1, 1.5, 1.7, 3 and 4. The constraints bind; the mandatory disclosures in section 4 must each have a home in every direction, on demand where the brief says so.
5. `instances/ROOT/OPENING_REPORT_2026-09-16.md` and `instances/ROOT/RENDERING_BOUNDARY_ANSWER_2026-09-17.md` — context on the current shell and the rendering engine that stays.
6. Optional, for grounding the surfaces only: `{REPO_ROOT}/projects/chirality-piping/apps/desktop/src/App.tsx` (the `WORKSPACE_SECTIONS` table) and `features/toolkit/capabilityCatalog.ts` for the operations the product actually has. Read, never modify.

## What to deliver

Write to `{REPO_ROOT}/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM/instances/CONCEPTS/`:

- `CONCEPT_DIRECTIONS_V1.md` — the main document. For each direction:
  - A name and a one-paragraph thesis: the single organising idea, stated so a stress engineer would recognise it.
  - What it borrows from CAEPIPE and from Aspect Pipe Stress, and where and why it departs.
  - The shell: window regions at 1440 × 900, what is always visible, what is a drawer, what is an inspector, in each of the three layouts (Model, Grid, Split). Give the canvas share of the window in each layout as an estimate.
  - The storyboard: the eight mock states from the brief §7, one short paragraph each describing what the engineer sees and does. Include where the agent panel is and how a proposal appears.
  - Where each mandatory disclosure (C §4, M-01 to M-17) lives, in a short table.
  - The 3D presentation language in outline: real outside diameter, fittings, restraint glyphs, load vectors, labels, result colour, deformation.
  - Risks and what would make the direction fail with this user.
  - A comparison table across directions at the end, and a recommendation with reasons. The owner chooses; the recommendation is advice.
- One low-fidelity wireframe SVG per direction per layout for state 7 (results on the model and in the table): `wireframes/<direction>_<layout>.svg`, 1440 × 900 viewBox, grey boxes and labels only, no colour system, legible at half size. Nine files at most.
- `RETURN.md` — what was read (with paths), what was produced, model and effort actually used, anything uncertain, and any place the brief could not be satisfied.

## Design constraints that bind

- The product is SWB Piping Designer, SWBPIPE for short. Do not use the old name in any new text except when quoting a source.
- No label, chip, badge or generated text may assert certification, sealing, approval, authentication or code compliance. Statuses use the accepted authority-attributed vocabulary. Ratios show number, rule ID and pack version.
- Units on every value. Provenance on demand. No boundary sentence repeated across product surfaces; one placement per surface class, or in the report.
- The agent is a visible collaborator with its own panel; proposals are diffs the engineer accepts; the model and the run remain the hero.
- Grid and 3D model are equal primary surfaces; the engineer chooses; both are live and linked; every edit in either writes the same element.
- Mouse driven and keyboard fluent: the grid and every form fully operable from the keyboard in the spreadsheet idiom; no command-line or modal-key model required.
- Dark and light are peers; design nothing that only works in one.
- Hanger design is in scope; stress isometrics and run comparison are deferred and must not appear.
- The rendering engine stays: Three.js, instanced meshes, invalidation scheduling, typed model index and picking. Directions describe what the model looks like, not how it is drawn.

## Exclusions

- No product source, test, harness or governance file is modified. Write only under `instances/CONCEPTS/`.
- No implementation plan, no component library choice, no code.
- No new claims vocabulary and no proposal to change governed text; where a direction needs one, name it as an item for the owner's decision packet.
- No delegation.

## Acceptance

ROOT accepts when: two or three directions are genuinely different organising ideas rather than styling variants; each satisfies the brief or states its departure; every mandatory disclosure has a home in every direction; the eight states are storyboarded for each; the wireframes exist and match the storyboards; and the recommendation gives reasons an engineer would recognise.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
