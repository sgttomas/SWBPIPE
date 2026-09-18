# Sealed brief — MOCKS-03: regenerate the frames from design system V1.2 and UX specification V1.1

Sealed by ROOT (HELP_HUMAN, successor) on 2026-09-18 before launch, after design system V1.2 and UX specification V1.1 were accepted (hashes in `briefs/_INDEX.md`). Role: HELPS_HUMANS design manager, working alone; Type 2 does not delegate. Model requested: Claude Fable 5.1 (the result is judged by eye). Mechanism: Claude Code `Agent` tool, general-purpose type, background.

Path placeholders: resolve `{REPO_ROOT}` with `git rev-parse --show-toplevel`; `{WORKING_ROOT}` is `{REPO_ROOT}/projects/chirality-piping`; `{RUN}` is `{WORKING_ROOT}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM`. Never write an absolute machine path in anything you produce.

## Purpose

Regenerate the mock frames so that they draw design system V1.2 exactly and show the behaviour of UX specification V1.1, with the owner's rulings of 2026-09-18 applied. After this pass no frame contradicts the design system, the specification or a ruling. The frames become the comparison basis an implementation is later read against. They are not advance acceptance of any implementation, and nothing here is implemented.

## Read, in this order

1. `{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md` at V1.2 with `tokens.json` 1.2 and `specimen.html`: the system to draw. §9 rows 50 onward list every change from V1.1 with its source; §8 lists what is open.
2. `{RUN}/instances/UX-SPEC/UX_SPEC_V1.md` at V1.1 with its change log, and its semantic-changes section. Where the two differ, the design system governs appearance and the specification governs behaviour and strings the design system's §7 does not have. The specification's `RETURN.md` lists sixteen contradictions its author found with V1.2 and the reading taken for each; ROOT accepted those readings, so for those sixteen the specification governs and the frames draw its reading (they are recorded as candidates for a later design system correction). Report any further contradiction you find; do not resolve it silently.
3. `{WORKING_ROOT}/execution/_Coordination/_DECISIONS/D-71_RULING_2026-09-18.md` and its two addenda beside it: the owner's words. A ruling governs over both documents.
4. `{RUN}/instances/MOCKS/`: the second pass as it stands, `frames/`, `tools/`, `sample_model.md`, `MOCKS_V2.md`, `shots/`. Reuse the generator and the sample model. `MOCKS_V2.md`'s questions Q-15 to Q-22 and gaps G-7 to G-12 are answered in V1.2; do not ask them again.
5. `{RUN}/instances/ROOT/REVISION_PASS_PREPARATION_2026-09-18.md` §1 to §3.

Read-only: everything outside `{RUN}/instances/MOCKS/`.

## What to deliver, in place under `{RUN}/instances/MOCKS/`

- `frames/`: the sixteen design frames (the eighteen of the second pass less the two decision aids) regenerated from `tokens.json` 1.2 and V1.2's components, each one self-contained HTML (styles inlined, no network request, scale-to-fit as before), both themes where the frame has them. The two decision-aid frames `d71_item1_status_bar_light` and `d71_item2_results_caption_light` are retired: D-71 is ruled and both sentences they illustrated are removed from the product. Delete the two frames and their shots, and record the retirement with its reason in `MOCKS_V3.md`; git history keeps them.
- Every frame applies the rulings: the name is SWBPIPE alone; no maturity sentence and no acceptance sentence anywhere; the status bar's information control opens About directly; status and evidence labels drawn only from V1.2's one table with their authority domain; the historical-run wording as V1.2 gives it; agent cards carry one of the five class words; the export names the model batch file and never another vendor's product; the Checked mark with the addendum's exact words.
- Every frame applies the pointer-control rule: each action a frame shows has its visible primary control as V1.2 §5 lists them (the toolbar's Undo, Redo, Inspector and Agent toggles; Fit first in the HUD; the routing compass's Reverse, Place and Cancel; the footer's selection group; close controls on the pinned probe, toasts, drawers and the inspector).
- The states the answers changed are drawn: Q-15 to Q-22 and G-7 to G-12 as V1.2 states them, including the run log's placement, the narrow-canvas HUD, the toast, the Review header's three buttons and Kind menu, `canvas.edgeAlt` on result-coloured elements, and the narrow-window order with the canvas never below 220 px.
- Result standing is drawn as V1.2's run-standing table gives it: the state-8 frame (model changed after a run) shows the stale band in ROOT's settled wording with no "stays the solve basis" phrase, values hatched in the tables, the rail caption Stale, no status chip, and no result colour, legend scale or probe on the canvas. If a Historical frame exists or is cheap to add from state 7, add one (`s7_both_light_historical`) showing the historical band and the same absence of a current-model overlay; say in `MOCKS_V3.md` whether you did.
- `index.html` regenerated to list every frame with its state, view, theme and what it tests.
- `frames/tokens.css` regenerated from `tokens.json` 1.2; `frames/mocks.css` revised; `tools/` updated as needed, taking the Playwright location at run time and carrying no machine path; `sample_model.md` unchanged unless a state needs a value, in which case list the change.
- `shots/`: 1:1 stage screenshots of every frame, `index.png`, and `report.json` with the measured facts (fonts, sizes, no network request, no overflow, the docked inspector's and canvas's measured widths, the HUD's size, the scale steps' colours).
- `MOCKS_V3.md`: per frame, what changed from the second pass and its source (ruling item, V1.2 §9 row, specification section); every departure with its reason; contradictions found between the two documents; questions only a screen can raise, numbered from Q-23; token or component gaps, numbered from G-13. `MOCKS_V1.md` and `MOCKS_V2.md` stay unchanged as history.
- `RETURN.md` replaced for MOCKS-03: what was read, what was produced with SHA-256 per frame, the lint and its result, the rendering verification, model, uncertainties.

## Constraints that bind

Canadian English ("colour", "Analyze"). "Accept", never "Approve". Never certify, seal, approve, authenticate, comply, compliant or sign-off as a control; "Review/signoff block" remains the registered section name. Units on every value. One sample model through every frame. Both themes as peers. V1.2 §7 binds every string; the specification adds the strings V1.2 does not have. Run no product build, test or dev server; rendering the frames with the existing Playwright install is permitted. No state-changing git command. No delegation.

## Acceptance

ROOT accepts when: every design frame is regenerated and renders offline in its theme with the whole shell; the two decision-aid frames are gone and recorded; a case-insensitive search of every frame, `index.html`, `MOCKS_V3.md` and `RETURN.md` finds no "SWB Piping Designer", no former product name, no "Technical preview", no "not a released product", no acceptance sentence or variant, no "CAEPIPE", no "stays the solve basis", and neither unhyphenated short label (the search strings may be built by concatenation in the lint so the tools do not carry them); every value traces to `tokens.json` 1.2 or is a listed gap; every label chip matches V1.2's table; the state-8 frame shows the Stale standing as specified; `MOCKS_V3.md` records every change, departure, contradiction and question; shots and report exist; no absolute path; nothing outside the instance directory changed. ROOT inspects the frames by eye before accepting.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
