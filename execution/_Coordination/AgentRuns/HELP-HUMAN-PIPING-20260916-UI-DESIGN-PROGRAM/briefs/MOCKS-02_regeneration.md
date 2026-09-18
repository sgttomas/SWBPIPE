# Sealed brief — MOCKS-02: regenerate the frames from design system V1.1

Sealed by ROOT (HELP_HUMAN) on 2026-09-18 after accepting design system V1.1 (brief DESIGN-SYSTEM-02) and the UX specification V1 (brief UX-SPEC-01). Role: HELPS_HUMANS design manager, phase 3 of the SWB Piping Designer interface program. Model requested: Claude Fable 5.1 (the result is judged by eye). Mechanism: Claude Code `Agent` tool, general-purpose type, background. Type 2 does not delegate; this child works alone.

Path placeholders: `{REPO_ROOT}` is the ROOT worktree; `{RUN}` is `{REPO_ROOT}/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM`.

## Purpose

Regenerate the fifteen mock frames so that they draw design system V1.1 exactly, with the owner's fourteen decisions applied and the UX specification's behaviour where a frame shows a moment the design system leaves to behaviour; add the states V1.1 §8 asks the second pass to test; and add two decision-aid frames the owner's decision packet needs. After this pass no frame contradicts the design system or the specification, and the frames are the evidence the owner reviews.

## Accepted basis, read in this order

1. `{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md` (V1.1) with `tokens.json` 1.1 and `specimen.html` — the system to draw; §9 lists every change from V1 with its source; §8 lists what the second pass must test.
2. `{RUN}/instances/ROOT/DIRECTION_DECISION_2026-09-17.md` §11 — the fourteen decisions.
3. `{RUN}/instances/UX-SPEC/UX_SPEC_V1.md` — the behaviour; in particular §2.3 (stages, views, rail states), §3 (tables), §5.3 to §5.5 (run, chips, toolbar, status bar), §6 (agent protocol), §7 (Review page), §10 (edge states, including §10.9 the narrow-window rule). Where V1.1 and the specification differ, V1.1 governs appearance and the specification governs behaviour and strings not in V1.1 §7.
4. `{RUN}/instances/MOCKS/` — the first pass: `frames/`, `tools/`, `sample_model.md`, `MOCKS_V1.md`, `shots/`. Reuse the generator and the sample model. `MOCKS_V1.md` §4's fourteen questions are answered by §11 of the direction record; do not ask them again.
5. `{RUN}/instances/ROOT/DESIGN_BRIEF_V1.md` (V1.4) §4, §6 and §7 for the frames' formula and the constraints.

Read-only: everything outside `{RUN}/instances/MOCKS/`. Nothing under `instances/DESIGN-SYSTEM/` or `instances/UX-SPEC/` is modified.

## What to deliver

Under `{RUN}/instances/MOCKS/`, revising in place:

- `frames/` — the fifteen frames of the first pass regenerated from `tokens.json` 1.1 and V1.1's components (each frame self-contained, stylesheets inlined, no network request, scale-to-fit as before), plus:
  - the second-pass states V1.1 §8 lists: state 2 in dark with the draft ghost and hint strip (exists: `s2_model_dark`, regenerate); state 4 Both with the inspector docked and the canvas at 303 px (`s4_both_light`, regenerate) and a new `s4_both_light_column` with the agent column open, drawing the specification's §10.9 rule (the column collapses to its strip when the docked inspector would take the canvas below its minimum; the canvas is never collapsed); state 5 with the combination editor as a row expansion under OCC1 (`s5_table_light`, regenerate); state 6 with the shorter run log popover and the single failure banner with the drawer filtered (`s6_both_light`, regenerate); state 7 dark with the re-anchored scale, legend and probe (`s7_both_dark`, regenerate); state 8 with the stale band, rail caption, latched Agent toggle and data bars on their track (`s8_table_light`, regenerate); state 9 with the outline's "Review/signoff block" and the header's Report preview and Export… (`s9_table_light`, `s9_table_dark`, regenerate);
  - two decision-aid frames for the owner's decision packet D-71 items 1 and 2, drawn as labelled alternatives and not as the system's rule: `d71_item1_status_bar_light` — state 1 with the maturity sentence "Technical preview — not a released product." permanently at the right end of the status bar in the secondary text style (the packet's option A) beside the frame's current popover placement; `d71_item2_results_caption_light` — state 7 Both with the listed short variant "Acceptance and professional judgment remain with the responsible engineer." as a one-line caption under the run identity in the results header, visible without a click, in place of the disclosure placement. Each carries a small label "Decision aid · D-71 item n · option A" in its title bar area so it cannot be mistaken for the design.
  - `index.html` regenerated to list every frame with its state, view, theme and what it tests.
- The chip after a failed run: the frame that showed "Model incomplete" after a non-convergence now shows what the specification says (§5.4 item 5): the chips the run record carries; since that is an open engine question (§11 Q11), draw no chip and say so in `MOCKS_V2.md`.
- `frames/tokens.css` regenerated from `tokens.json` 1.1 (build source); `frames/mocks.css` revised to V1.1 (build source); `tools/` updated as needed; `sample_model.md` unchanged unless a new state needs a value, in which case the change is listed.
- `shots/` — 1:1 stage screenshots of every frame (light and dark as the frame is), `index.png`, `report.json` with the measured facts (fonts, sizes, no network, no overflow, the docked inspector's measured widths, the scale steps' colours). Do not retain page screenshots at other sizes.
- `MOCKS_V2.md` — per frame: what changed from the first pass and its source (decision number, V1.1 §9 row, specification section); every departure from V1.1 or the specification with its reason; the decisions the frames made that neither document states; questions only a screen can raise, numbered from Q-15; token or component gaps found, numbered from G-7. Keep `MOCKS_V1.md` unchanged as history.
- `RETURN.md` rewritten for MOCKS-02: what was read, what was produced, the lint (every mandatory disclosure once where V1.1 §7.4 places it, no forbidden word, no old product name, no absolute path), the rendering verification, model and effort, uncertainties.

## Constraints that bind

All of MOCKS-01's constraints continue: the product name; no approval, certification or compliance language; the six statuses with authority domains; Accept never Approve; the Checked mark as drawn in V1.1 §7.3; units on every value; one sample model through every frame; both themes as peers; nothing implemented, nothing claimed. V1.1 §7 binds every string; the specification §9.3 adds the strings V1.1 does not have. Short labels stay V1.1's forms ("User rules checked", "User rule failed") until the packet rules. M-01 and M-02 stay where V1.1 §7.4 places them in every frame except the two decision-aid frames.

## Exclusions

- Write only under `{RUN}/instances/MOCKS/`. No product source, test, harness or governance file is modified; nothing under other `instances/` folders.
- No implementation, no component library, no framework code; the generator is evidence tooling, not product code.
- No new claims vocabulary; no proposal to change governed text.
- No delegation.

## Acceptance

ROOT accepts when: every frame of the first pass is regenerated and renders offline in its theme with the whole shell; every second-pass state of V1.1 §8 is drawn; the two decision-aid frames exist and are labelled; the failed-run chip follows the specification; every value traces to `tokens.json` 1.1 or is a listed gap; the lint finds each disclosure once where placed and no forbidden word; `MOCKS_V2.md` records every change, departure and question; the shots and report exist; RETURN is complete.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
