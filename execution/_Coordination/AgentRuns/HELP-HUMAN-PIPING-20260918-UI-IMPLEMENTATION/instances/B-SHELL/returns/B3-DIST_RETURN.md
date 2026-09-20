# B3-DIST return: the dist lane's browser tests moved to the new shell; compact project checked

I am Claude Fable 5.1 (`claude-fable-5-1`), by my own statement. Role: TASK (Type 2), working alone; I launched no agent. The brief's hash verified as `7851610832b6572ecb8bb44e7c3d5e3d1c38fc827dfe09261f2e3a12b048aa4f`. I worked in the assigned worktree, on `codex/swbpipe-b-shell-20260918` at `ba8228d2a`.

**Result.** Of the 29 unmoved dist tests, 10 now pass and 19 stop on frozen geometry.
- 15 of the 19 have their structure moved and then stop on a frozen check: preflight, visual-clarification and the 12 appearance variants; rail-clearance needed no move.
- The 4 canvas-budget variants are left untouched, stopped as the brief directs.
- `toolkit-batch-dist.spec.ts` and the settled "decorative viewport overlays…" test both pass.
- The source lane's compact project is clean except one `r2-smoke` test, reported below and not changed.

I never tried my file tools on the worktree: every edit went through the shell with short Python scripts, exact-match replacements, each asserting its occurrence count. I ran no state-changing git command. I touched no product code, nothing under `e2e/ui-foundation/**`, no driver and no configuration.

## 1. Files changed

Under `projects/chirality-piping/apps/desktop/`:

| File | SHA-256 |
|---|---|
| `e2e/toolkit-batch-dist.spec.ts` | `0faca6572f26354ba0ce2b907556a604d8726d483fd297bd0a5d2ef4b54fe082` |
| `e2e/ui-foundation-dist.spec.ts` | `ff6a39ecde704c62c68178da5b0e6fd73fa89290e8c493ccd81c2bc6adddd448` |

## 2. Move inventory

No `test.skip`, no `test.fixme`, no widened timeout, and no floor, tolerance or expected value changed.

1. **`toolkit-batch-dist.spec.ts`, "dist self-weight plan previews…"**
   - Old step: click `undo-session-model-edit` directly.
   - New step: `openWorkspaceSection(page, "operations")` first, then the click.
   - Reason: Undo lives on the Review changes tab, and the preceding tree check had left that tab. This is the source lane's move.

2. **"[visual-clarification]…"**, three moves; the 200 px floor and every unobscured check are untouched.
   - Old: both toggles expected open at first load (1440). New: the tree toggle is open, the inspector toggle is closed (named change 13), then `ensureRail(inspector, true)` and it is asserted open.
   - Old: `readSavedPanelSizes` read `leftRailPx`, `rightRailPx` and `dockPx`. New: it reads `bothSplitPct` and `tableDrawerPx`, the sizes the shell uses. The guarantee is still that opening and closing changes no stored size.
   - Old: the dock loop tested `workspace-dock` for the `collapsed` class and closed it by `workspace-dock-close`. New: the narrow table drawer is the dock's successor. Open is `toggle-tree` with `aria-expanded="true"` after the section is summoned; closed is the chevron clicked and `aria-expanded="false"`.

3. **"outer overlays own actual workspace intersections and restore controls 1440"**
   - Old obstacle: `resize-property-inspector`.
   - New obstacle: the inspector's "Properties" tab, measured at x 1107, y 201.6, 137.5 × 32.
   - Reason: it is an inspector-layer control that the drawers actually cross. Both drawers measure x 662, y 142, 760 × 720. `inspector-close` (y 83 to 115) and `toggle-inspector` (y 42 to 70) lie above the drawers at 1440.
   - The rest is unchanged: focus, the `ArrowLeft` press and the unobscured checks. The 1024 variant needed nothing.

4. **"production appearance …", 12 variants**
   - First load at 1280 and wider: the inspector toggle is now expected closed (change 13).
   - Hover loop, tree rail: the chevron in wide Both view is disabled. The test now asserts `aria-disabled="true"` and the title "Both view keeps the tables open", then reaches both chevron states in Model view, then returns to Both. The inspector rail is exercised in Both view as before. State, focus and minimum-target checks stay on the successor controls.
   - Hover contrast: the `.workspace-pane-toggle-label` and `.workspace-pane-toggle-icon` witnesses are removed (section 4 gives the reason). The `svg` glyph witnesses are kept at 3:1, and I added a structural `expect(glyphs.length).toBeGreaterThan(0)` so the witness cannot be empty.
   - Overlay list: the "Property inspector splitter" entry became "Property inspector close control" (`inspector-close`).

5. **"populated Results title and entered values retain resolved contrast", 2 variants**
   - The title witness moves from the dock header's "Results" heading to the latched tab `stage-tab-results`. The test asserts its text "Results" and `aria-pressed="true"`; the 4.5 floor is unchanged and passes.
   - `ensureRail(inspector, true)` is added before the required-flag read, because the inspector is closed at first open.

6. **"rail preferences resize actual panes…", 4 variants, and "1024 drawers retain overlay geometry…"**
   - Both are replaced by their successors copied verbatim from the source lane's `workspace-layout.spec.ts`, together with that file's local helpers `readShellGeometry` and `chooseView`.
   - New titles: "the split and the drawer resize actual panes, persist, and keep the inspector's controls contained ${theme} ${width}" and "below 1280 px the table drawer and the inspector lie over the canvas, keep its geometry, and return focus to their openers".
   - The guarantees are the same: pointer and keyboard resizing, bounds (15 to 85 %, canvas at least 220 px, drawer 120 to 600 px), persistence over a reload, contained inspector controls, fit projections, and Escape returning focus to the opener.
   - The old pixel values fell with the rails: 280/340, 420/520, 220/280, 30, 40 and 390.

7. **"empty ordered selection publishes independently…"**
   - `workspace-dock-close` from Results becomes `showModelTree(page)`, with its import added. This is the source lane's move.

Preflight and rail-clearance needed no structure move.

## 3. Tests stopped on frozen geometry, with numbers

All measurements are from the dist build in the browser runtime. For stops A, B and D I ran a throwaway copy of the spec with only the frozen helpers made soft, and the copy is deleted. The three 1024-state tests plus the appearance tests (light comfortable, all three widths) passed everything else there, so their structure moves are verified past the stop.

**A. The collapsed table strip covers the measurement readout at 1024 × 768.**
- The readout `viewport-measurement-readout` is at x 56, y 714, 924 × 30. The strip `.workspace-pane-tree` is at x 56, y 716, 924 × 28.
- The readout's centre hits `.shell-tab-strip-fill`, and its title text hits `workspace-review`.
- With the drawer open (x 56, y 464, 924 × 280) the readout is fully under it.
- The second return's foot inset moved only the triad marker and the scale bar; this item is not named there.
- Stops:
  - preflight, line 124;
  - visual-clarification, line 197, where expected true is received false;
  - rail-clearance, line 284, "center is covered by workspace-review";
  - inside the canvas-budget test.
- Context: the target button is at x 489, y 443, 58 × 32, and with the 280 px drawer open it stays clear only because its centre is at y 459. The canvas is x 56, y 204, 924 × 510.

**B. The orientation frame, appearance test line 898.**
- The helper samples a 96 px square inset 8 px from the canvas's bottom-left, where the gizmo is painted in the main canvas.
- 1024 × 768 with no readout: the canvas is x 56, y 204, 924 × 540 and runs under the strip at y 716. The clip is x 64, y 640. The samples at y 735.5 are owned by `stage-tab-model-tree` and `workspace-review`. The foot inset moved the DOM marker to y 608 but not the painted frame.
- 1280 × 800: the splitter `resize-model-tree` is at x 693, y 109, 24 × 667 and overlaps the canvas pane, which starts at x 705. The clip is at x 713, and the three samples at x 713.5 are owned by the splitter.
- 1440 × 920: the splitter is x 781 to 805, the canvas starts at x 793, the clip is at x 801, and the three samples at x 801.5 are owned by the splitter. This happens with the inspector open or closed.
- With the narrow drawer open, all the bottom samples are owned by the drawer ("Batch review"). This is the visual-clarification dock loop, which the soft copy reached.

**C. `expectWorkspaceGeometry`, canvas pane area greater than inspector area, at 1280 × 800 with the inspector docked; appearance test line 824.**
- Comfortable: pane 231 × 695 = 160,545 received, against 208,500 expected (300 × 695).
- Compact: 161,469 received, against 209,700 expected.
- This helper was already reshaped by the earlier child: the 35 % canvas-width check was made conditional, and the area comparison moved from the canvas to the canvas pane. It fails even in that form. I did not touch it.

**D. "content-aware narrow canvas budget", 4 variants: left untouched.**
- It still fails at its first line, where the dock is expected not collapsed.
- A move cannot keep its scenario, for three reasons:
  - the drawer splitter `resize-task-dock` is not visible below 1280 px, so the 180 and 600 px states cannot be reached by the splitter, and its unobscured check has nothing to hold;
  - the bounds are now 120 to 600 with a default of 280 and 16 px steps, so 180 is not reachable by key (184 or 168);
  - the narrow drawer overlays the canvas instead of shrinking it, so the canvas box stays 924 × 510 while it is covered.
- Derived from the measured rectangles, with a surface of y 81 to 744: a 600 px drawer starts at y 144 and covers the whole canvas and the target; a 180 px drawer starts at y 564 and leaves y 204 to 564 visible. In every state the readout is covered, as in stop A.
- ROOT must decide what the budget means under the overlay fallback.

## 4. Guarantees with no successor

- **The hovered rail label's 4.5:1 contrast** (`.workspace-pane-toggle-label`), collapsed and expanded, on both rails at all three widths.
  - The drawer chevron has no text label, only `aria-label` and `title`.
  - The Inspector toggle's `.shell-toolbar-label` is rendered only at 1440 (57.2 × 18.2) and is hidden at 1280 and 1024.
  - Nearest held guarantee: the hovered glyph contrast at 3:1 on both successor toggles, and the toggles' names. I did not add a witness for the 1440-only label. B-SHELL may want one.
- **The separate disclosure glyph** (`.workspace-pane-toggle-icon`). The drawer chevron's `svg` is now the disclosure glyph, and it is witnessed.
- **The collapsed rail widths (30 and 40 px), the 390 px narrow drawer, and the inspector splitter's keyboard and pointer resize with persistence.** The rails and that splitter no longer exist. The nearest guarantees are the split and drawer tests and the fixed 300 and 340 px inspector widths they assert.
- **A drawer-height resize below 1280 px.** No control exists; see stop D.

## 5. Last command on each spec file

Every run was from `apps/desktop` through `sh {RUN}/tools/with_e2e_lock.sh` with `PLAYWRIGHT_WORKERS=1`. I queued behind other lanes' runs several times.

| Spec | Command | Result |
|---|---|---|
| `ui-foundation-dist.spec.ts` | `npx playwright test --config playwright.dist.config.ts e2e/ui-foundation-dist.spec.ts` | 30 passed, 19 failed, 0 skipped |
| `toolkit-batch-dist.spec.ts` | the same config with this file, run together with a probe that is now deleted | 1 passed |
| `workspace-layout.spec.ts` | `npx playwright test e2e/workspace-layout.spec.ts --project=chromium-compact` | 11 passed |
| `linear-authoring.spec.ts` | same form | 1 passed |
| `gui-workflow-validation.spec.ts` | same form | 3 passed |
| `result-compatibility.spec.ts` | same form | 1 passed |
| `ui-foundation.spec.ts` | same form | 49 passed |
| `r2-smoke.spec.ts` | same form | 9 passed, 1 failed |

- The 19 dist failures are exactly the tests in section 3. They are 1 preflight, 1 visual-clarification, 1 rail-clearance, 4 budget and 12 appearance; among the appearance variants, 1024 and 1440 fail on B and 1280 on C.
- The baseline run of both dist files before my edits gave 21 passed and 29 failed. The full dist lane should therefore show 19 failed, all in this file, and the other dist files as before.
- `report-package-dist`, `result-compatibility-dist` and `wasm-engine-dist` were not run by me.
- The note `--project <name>` placed before the file is read as a project list; use `--project=chromium-compact`.

## 6. Compact project finding, not changed

`r2-smoke.spec.ts:250`, "R2 desktop preview smoke covers solve, results, report, and viewport overlay", fails at line 559: `viewport-select-node:N-140` is hidden.
- State at 1280 × 800: Both view, pipe tool armed, inspector docked.
  - table pane: 349 px wide;
  - canvas pane: x 405, 531 px wide;
  - authoring panel: 265 px wide at x 671;
  - drawn canvas: 266 × 559.
- The camera was framed for a wider canvas. N-140 projects to x 402 in a layer 266 px wide, so `PipeViewport` sets `display: none` on it as outside the frustum.
- At 1440 the drawn canvas is 338 px wide, N-140 is clamped at x 309 and shown, and the test passes.
- The cause is the one in the interim return's section 1: the authoring panel starves the canvas. The symptom is new and not named in either return.
- In a throwaway copy, now deleted, one "Fit Visible" click before the endpoint pick made the whole test pass in compact. I did not apply it: it adds a scenario step and hides that the camera is not re-framed when the canvas narrows. B-SHELL decides.

## 7. For others

- **Product, shell or canvas lane:**
  - the strip and the narrow drawer cover the in-flow measurement readout (A);
  - the painted orientation frame runs under the strip (B at 1024);
  - the split splitter's 24 px hit box overlaps 12 px of the canvas, 4 px into the orientation frame (B at 1280 and 1440).
- **ROOT:**
  - what the budget floor means with an overlay drawer that has no narrow splitter (D);
  - the area rule at 1280 with the inspector docked (C), including the earlier child's conditional 35 % width check in `expectWorkspaceGeometry`;
  - the `r2-smoke` compact finding (section 6).
- The legacy `leftRailPx`, `rightRailPx` and `dockPx` are still written to storage, at 280, 340 and 260.

## 8. Closing checks

- `npm run build:desktop` from `{WORKING_ROOT}`: exit 0.
- `npx tsc -p tsconfig.json --noEmit` from `{DESKTOP}`: exit 0. It does not cover the e2e files, because `tsconfig.json` includes only `src` and `vite.config.ts`. They are type-loaded only by Playwright's transform at run time.
- `git status --short` lists only the two files in section 1. My probe specs (`e2e/zz-scratch-dist.spec.ts` and `e2e/zz-scratch.spec.ts`) are deleted.
- Ports 5174 and 5175 are closed.
- No file changed after the last check.
- No absolute machine path is written in any file.

I claim no usability, conformance or performance acceptance. PDU-045 and PDU-046 remain holds.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).