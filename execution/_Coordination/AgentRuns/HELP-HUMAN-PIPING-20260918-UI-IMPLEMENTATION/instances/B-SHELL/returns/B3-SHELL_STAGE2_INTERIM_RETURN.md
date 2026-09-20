# B3-SHELL return: stage 2, stopped early on a frozen-floor failure

**Model:** Claude Fable 5.1 (`claude-fable-5-1`), by my own statement. TASK, Type 2. I launched no agent.

I have stopped before the final candidate. The shell is built and the unit suite and build are green. A frozen regression floor in `e2e/ui-foundation.spec.ts` fails under the new shell because of what the layout now is. The brief says to report that with the numbers and not adjust it myself. The full Playwright lanes were **not** run.

- **Base:** head `def97eb4865febc2cdb1fdf83a369e939e06dad9`. I re-read `workspaceSession.ts` before editing it.
- **How I wrote:** everything went through the shell (heredocs, and Python replacements that assert their occurrence counts). I ran no git command that changes state.
- **Reading:** design system §0, §5.2, §5.3 and addendum 3 whole. Of the frames, only the structure of `s2_model_light` and its measured facts.
- **Not read:** specification §10.9 and §12, operations map rows 1 to 40, and the bodies of the `s1_both_light` and `s2_model_dark` frames. None of my decisions rests on them. They are owed before this slice closes.

## 1. The stopper: the frozen 200 × 200 canvas floor

**The test.** `e2e/ui-foundation.spec.ts`, "task and analysis dock preserve usable canvas", 12 variants. Its header reads "Frozen regression floor … must retain a 200 × 200 CSS-pixel canvas".

**The state it measures.** A property task is open in the inspector, and the viewport's own authoring panel is active (`command-pipe`).

**Why it fails.** In Both view the docked inspector takes its 300 px from the canvas, as the specification says. `PipeViewport`'s authoring panel then takes another 265 px from inside the canvas pane.

Measured through the lock, browser runtime:

| Window | Both view, inspector closed, panel active | Both view, inspector docked, panel active | Model view, panel active |
|---|---|---|---|
| 1440 × 920 | pane 603, drawn canvas 338 × 679 | pane 303, **drawn canvas 38 × 679** | 735 × 412 |
| 1280 × 800 | pane 531, 266 × 559 | pane 231, **drawn canvas 1 × 559** | 575 × 292 |
| 1024 × 768 (narrow fallback) | 576 × 540 | 576 × 540 (the inspector lies over the canvas) | not measured |

- At 1440 and 1280 the floor fails, with 38 px and 1 px against 200 px.
- The shell's own rule still holds: the canvas pane is 303 and 231 px, above the 220 px minimum. The starvation happens inside `features/viewport/**` and its `.viewport-shell:has(.viewport-intents.active)` rule. Neither is mine to touch.
- **`r2-smoke.spec.ts:528` is hit by the same cause.** The canvas gesture there was covered by the authoring panel. I inserted a step that closes the inspector before the gesture. **B-SHELL should decide that step together with the floor.**

I did edit this test's structure, which is mine to move:

- the dock-under-canvas relation became "the page covers the surfaces";
- the solve proof is read on the Evidence tab;
- the page is opened or closed around the canvas-side checks.

I did **not** change the 200 px value, and I did not rearrange the scenario to avoid the failing state.

Choices I can see, none of which I took:

- (a) the canvas lane moves the authoring panel out of the canvas pane, which is the design's routing block in the inspector;
- (b) an interim shell rule: arming a viewport authoring tool closes the Both view's docked inspector;
- (c) ROOT re-scopes the floor's scenario.

## 2. ROOT's condition 3: the collapsed drawer strip covers canvas furniture

Native-runtime class, 1440 × 900, Model view, drawer collapsed. The strip is at x 56, y 848, 1000 × 28.

- **Scale bar** at x 952, y 829.2, 90 × 32.8: its lower 14 px are covered (y 848 to 862).
- **Axis triad** at x 68, y 768, 96 × 96: its lower 16 px are covered (y 848 to 864).
- The deformation status (y 104) and the fallback message (absent) are not covered.

I moved no furniture and faked nothing. With the drawer open nothing is covered. The same overlay occurs in the narrow fallback's Both view.

## 3. Checks

| Command | From | Result |
|---|---|---|
| `npm run build:desktop` | `projects/chirality-piping` | exit 0 |
| `npm run test:desktop` | same | exit 0; **80 files, 1,283 tests passed** |
| `npm run test:e2e`, `npm run test:e2e:dist` | `apps/desktop` | **not run** |

- `App.deadControls.test.tsx` and `src/design/themeAttribute.test.tsx` are green and unedited.
- Every Playwright run and throwaway script went through `with_e2e_lock.sh` with `PLAYWRIGHT_WORKERS=1`. Another worktree's runs held the lock repeatedly, so I queued behind them. Port 5174 is closed now.

**Triage runs, `chromium-desktop` only:**

| Spec | Result |
|---|---|
| `workspace-layout.spec.ts` | 7 of 7 passed, including the D-72 geometry test |
| `linear-authoring`, `result-compatibility` | passed |
| `gui-workflow-validation` | 2 of 3 passed; my moves to the third were not re-run |
| `r2-smoke` + `ui-foundation.spec.ts` (run together) | 38 passed, 21 failed |

The 21 failures break down as:

- 12 are the floor above.
- 2 are r2-smoke: `layout-mode-tree` clicked while the tree is not shown; and a `viewport-deformation-status` step made with the Solve page open.
- 7 are ordinary structure moves in `ui-foundation.spec.ts`, not yet made:
  - the removed Units disclosure `details.display-preference-control` (:63);
  - a tree filter filled while another stage is shown (:385);
  - `workspace-dock-header` expected to say "Review changes" (:1084);
  - `#geometry-tools` hidden in a collapsed drawer (:1239);
  - the "Resize model tree" separator name (:1412);
  - `workspace-dock-close` on a stage surface (:1820);
  - a triad gesture ordinate (:1969), where the expected value is greater than 767 and 749.98 was received because the canvas is taller; I have not read it further.

The dist lane has not been tried at all.

**Instrument specs in the lanes (ROOT's answer 1).** Four `e2e/ui-foundation/**` specs run in the source lane: `causal-method-contract`, `fresh-demo-policy`, `full-cohort-controller` and `performance-targets`. None of them loads the App; they use `data:` pages and pure functions. None needs anything from the shell, and none was edited.

**Files after the checks.** No source file changed after the last unit run. The 16 PNGs were written after it.

## 4. Files

All hashes are SHA-256.

**Under `apps/desktop/`:**

| File | State | SHA-256 |
|---|---|---|
| `e2e/gui-workflow-validation.spec.ts` | changed | `6caf90c6bd2c3a4d9a2db2ebc374fe707ded9db14a6c709332453c53013a2bb9` |
| `e2e/linear-authoring.spec.ts` | changed | `80c2532edad2fcaac15a03da333c48455a052f541761ec306135881dd464d020` |
| `e2e/r2-smoke.spec.ts` | changed | `68bd210605fa721c4f5b5e2f1176246736fdddbfd89f69cd73cbd630de77b0d1` |
| `e2e/ui-foundation-dist.spec.ts` | changed | `e6fd6b7f3f54df1f0721eeeaf3f4775965fa2a067ef3d8ca3050d68f3d6f341c` |
| `e2e/ui-foundation-workflows.ts` | changed | `e338de17732dd74b2da8f6d17d8d4cfb77991b083018472ea972865d2ce9942e` |
| `e2e/ui-foundation.spec.ts` | changed | `192d6bfe7716e5996e4d7d1e76d5655d9852f7f96ab962b26a6b4affecd64839` |
| `e2e/wasm-engine-dist.spec.ts` | changed | `ed29de65b7121e1ef6a78348b10144473163d33a0a4010978a986a1fcc5a01aa` |
| `e2e/workspace-driver.ts` | changed | `8a24ff8662d336b259b1b443fd1a73dc2ea39df43c68e17f0a9eeedefa3b0c9c` |
| `e2e/workspace-layout.spec.ts` | changed | `15398caf5569e6568baaf21cbb2f79288a7bd09bac9a80c4b1d19609f4061547` |
| `src/App.projectHandlers.test.tsx` | changed | `466a3f99b8d5e704c5b7e07af05e52e28898a1a2aa594ec1f57131ef545f5502` |
| `src/App.test.tsx` | changed | `0c65d7bc4b3975f46dd07d9babfb707a62204a341ad13b22e4419f018d2fbd72` |
| `src/App.tsx` | changed | `ac333b1586ae129bf03f82d0783ff2ffb5b4463a2377d102a9b4ebecd44524b5` |
| `src/features/display-units/index.tsx` | changed | `4bbdc032a715c1744da13ef7ed2c10afaae0bc989cd3299554af02239b560aea` |
| `src/features/toolkit/ToolkitPalette.test.tsx` | changed | `c84690b02392af87a54702453317cf9f828e358b1ec8ebb5bdbea02c5a7f6592` |
| `src/features/toolkit/ToolkitPalette.tsx` | changed | `793fcba9dfd4eadc925b62a73fa36796e59cef64daa6626b0adde41e576d3490` |
| `src/features/workspace/chromeSessionState.ts` | changed | `1822ee1ba7ac4271c4ad89faf30fc83f6fc3cc8d1e56a0e98d59507d8e6669bb` |
| `src/features/workspace/menuCommands.ts` | changed | `6fc00caffd3795e9a4e5ac3e7078e1dba6f0f009c0180d83de5f7b438e348334` |
| `src/features/workspace/shellLayout.test.ts` | changed | `ff0bb285d16e682d2661dcddf6e47ffe433827ed4617c3ae66c05df915648e2f` |
| `src/features/workspace/shellLayout.ts` | changed | `cc4f1d108ba4e7fead32f7a977b8434886aa364aa0a240fef2fe92947b26054e` |
| `src/features/workspace/uiPreferences.test.ts` | changed | `1ff3c527b820b4b644f64b03f4005f914f55bcb141855c1bed54ac141e7eb2d8` |
| `src/features/workspace/uiPreferences.ts` | changed | `a784c2bd580362697f1013a57c9560366deb94a7f0dc61727a9b6101a7c5cecc` |
| `src/features/workspace/workspaceSession.ts` | changed | `16b3198e708808dc3e654cd140d80111a1f233175855f7ad53fe115e7b0b7b29` |
| `src/styles.css` | changed | `719b8b01c23fa7b8f1f5bd8b3d4fa444bb991df82e0b07ec6d630feb6d213ff9` |
| `src/App.shell.test.tsx` | new | `8bd51546789d7c6514d48b4e1b1dbef98d82766211c38b72ef031faeeac76699` |
| `src/features/workspace/shell/AgentStrip.tsx` | new | `f443b2dcc20cdf3b1a2f1e29d37f586f36b94a67d91ad01eb95f2745053b693f` |
| `src/features/workspace/shell/DisabledReason.tsx` | new | `9fd86feefc50904243920067186a7d4986cb7204ac5f442e7d9890c5b65c4068` |
| `src/features/workspace/shell/ShellStatusBar.tsx` | new | `c8684af8ace795e43f6667ee682c9d1650fed653fd04ac4bef4b6c1ecb5f7074` |
| `src/features/workspace/shell/ShellToolbar.tsx` | new | `8fedcc189ef35e0f8326115b6fdad1373f2b1ba80ab93415665cc922ddd9072d` |
| `src/features/workspace/shell/StageRail.tsx` | new | `1b2f2c6d937642cb7facc39573eded7829fa23b880983c89aa54aa48ccf222fd` |
| `src/features/workspace/shell/StageTabStrip.tsx` | new | `e5ecc7601c02107f54bf7f99262e7c1ae5dfff9109f6df3f87ed849113f79c88` |

Deleted: `src/features/workspace/WorkspaceToolbar.tsx` and `WorkspaceToolbar.test.tsx`. What the test held is re-homed in `App.shell.test.tsx` and `ToolkitPalette.test.tsx`.

**Screenshots** are under `{RUN}/instances/B-SHELL/shots/B3/`. They are interim: this candidate is not final.

| PNG | SHA-256 |
|---|---|
| `loads_both_light` | `f18bd23fb3883a3c4648496015c9d5573d463c4f69a9e61ab7008c5c3bc5aa5d` |
| `loads_model_light` | `7f16ed2cd35e43a511d6145cafc56fdc6795146d10aafe0f6ad8b2df20bf9e00` |
| `loads_table_light` | `0d7e40a1e0b8cb773146b672e1f783df65fd4fec90e98135c4db8ebed6be177c` |
| `model_both_dark` | `bb7bef138bc30aa50c5cc8153e3c51d32068b6061b258b08a11279746646e274` |
| `model_both_inspector_light` | `22a22351610fed284e238af18b5093bd1ee4de2cc5d8ebb15a3d89165d3f4154` |
| `model_both_light` | `7480bf01ba715e7da4b62717b0f3eba12396af733a25b7d3d11717c6a6378701` |
| `model_model_drawer_collapsed_light` | `e0bc419d6548270fe84ce21a5d3f8089f3adaf33fb7518ec233a6ded21f027bd` |
| `model_model_light` | `b930487e8df7e71bf78ad6351f92d6092331ea5c079f2edd5b207053c37ef4e2` |
| `model_table_light` | `01351da76b86e5692469be426fcf9915d27b03ffab21ce8e6763fee251a12aac` |
| `native_model_both_light` | `f045fa5638de98ae9f25102afbb92475496314835db72e426eba9e0a21a1e812` |
| `native_model_model_drawer_collapsed_light` | `2a7027b76231c044c9e2da3f6d3528635954242259f72495f131c9984658490e` |
| `project_page_light` | `541dc99c8affdcfce1444bed53ecbc820516f8dc20af24d9c762cf974aa9f182` |
| `results_both_light` | `0eac964ad4d83fbb39a0507ccb492996ea789bc0792a02e71971f8fc9a0671fa` |
| `results_model_light` | `ffc97775787220c2d59afef749dd016631bc06a88b085a1771f2d04902477492` |
| `results_table_light` | `01a6a040e6c70bc0686168e4bdfcdc0f04e6dcf0048b26d9b4255867c10798f3` |
| `review_table_light` | `177b9435433399484a2ed6cef429b18b82858a3ab6aa993a640a875d18a57a12` |

## 5. What is built

- **Order of regions:** in-app menu bar (browser runtime only), toolbar band, a body of rail, surfaces and agent strip, then the status bar.
- **One DOM, three layouts.** A CSS grid keyed on `data-view` and `data-stage` lays the same panes out three ways. Nothing is re-parented, so `PipeViewport`, the inspector and all ten sections stay mounted. A product test holds element identity and kept state across every stage and view.
- **`setActiveSection` is still the only navigation primitive.**
  - New chrome handlers: `chooseStageView`, `enterStage`, `closeShellPage`.
  - One new cell, `narrowWindow`.
  - No new setter is exposed; the key-set test is unchanged.
- **Table view and `PipeViewport` (finding).**
  - Hidden with `display:none`, the renderer's `ResizeObserver` reads 0 × 0, so the buffer is resized to 1 × 1 and the camera aspect is set to 1.
  - Table view therefore keeps the canvas pane at its Both-view box, with `visibility:hidden` and no pointer events. No resize happens and the camera is untouched.
  - `workspaceCanvasBudget.ts` now only feeds a variable that nothing uses. Neither viewport file was edited.
- **`uiPreferences`:** adds `bothSplitPct` (15 to 85, default 55) and `tableDrawerPx` (120 to 600, default 280). The storage key and version stay. Old records read without error. `leftRailPx`, `rightRailPx` and `dockPx` remain in the record, unused; nothing is migrated.
- **Narrow fallback, below 1280 px.** Today's behaviour is kept, because `App.deadControls.test.tsx` pins it at 1024 and jsdom itself is 1024 wide.
  - The canvas takes the surface.
  - The table pane is a bottom drawer over it, and the inspector slides over its right edge.
  - Both start tucked away and close when the canvas is used. ⎋ inside one returns focus to its opener.
- **`styles.css`:**
  - both comment corrections are made, and `@import "./tokens.css";` is still the first rule;
  - retired, with their hard-coded colours: the old `.titlebar*`, `.display-preference-control`, `.workspace-toolbar`, `.modeling-workspace*`, `.workspace-pane-toggle*`, `.workspace-pane-agent` and `.workspace-splitter*` rules;
  - the new block writes no colour literal;
  - no `.viewport-shell`, `.viewport-scale-bar`, `.viewport-fallback` or `.viewport-deformation-status` rule was changed.

## 6. Geometry at 1440 × 900, measured

| Region | Native-runtime class | Browser runtime (menu bar row 33 px) |
|---|---|---|
| Toolbar, rail, agent strip, status bar | 48, 56, 44, 24 | the same |
| Surfaces | x 56, y 48, 1340 × 828 | x 56, y 81, 1340 × 795 |
| Both: table pane and canvas pane | 737 × 828 and **603 × 828** | 737 × 795 and 603 × 795 |
| Both, inspector docked | canvas 303 × 828, inspector 300 × 828 at x 1096; the table does not move | canvas 303 × 795 |
| Model, drawer open | canvas 1000 × 548, drawer 1000 × 280 at y 596, inspector 340 × 828 | 1000 × 515 |
| Model, drawer collapsed | canvas **1000 × 828**, strip 1000 × 28 at y 848 | 1000 × 795 |
| Table | pane 1340 × 828; the canvas box is kept, hidden | 1340 × 795 |

- `e2e/workspace-layout.spec.ts` pins the native column. It sets the product's own `__TAURI_INTERNALS__` marker after the model has loaded, and only view switches follow.
- The drawn canvas is smaller than its pane by `PipeViewport`'s own bars: 603 × 692 in native Both view.
- At the instrument's 1440 × 920, the native heights are 848.

## 7. Cross-lane hooks as built

All measurements are from the 1440 × 900 browser runtime, Model stage, Both view, at first open.

| Hook | Element now | Measured |
|---|---|---|
| `.app-shell`, `desktop-preview-shell` | same `<main>` | x 0, y 0, 1440 × 900 |
| `.workspace-pane-tree` (exactly one) | the table pane holding the model tree | x 56, y 81, 737 × 795; Model view drawer 1000 × 280 |
| `.panel.model-tree`, `model-tree-filter-input`, `model-tree-virtual` | unchanged `ModelTree` | x 56, y 109, 736 × 767; filter 582 × 32; list 712 × 420 |
| `toggle-tree` (`aria-expanded`) | the table pane's chevron. Always present; disabled with its reason in Table view and in wide Both view, where `aria-expanded` is true | x 764, y 81, 28 × 27 |
| `.workspace-pane-inspector` (exactly one), `.panel.inspector > h2` | the docked inspector. **Hidden at first open in Both view** (ROOT's answer 1), so the instrument's two-pane boundary capture needs the toggle opened first | open: x 1096, y 48, 300 × 828, h2 279 × 36.4 (native-runtime class; the browser-runtime row was not measured with the inspector open). Model view: x 1056, 340 wide |
| `toggle-inspector` | the toolbar's Inspector toggle | x 923.1, y 42.5, 91.2 × 28 |
| `command-bar`, `.command-context > summary`, `command-selection-readout`, `.viewport-toolbar > span`, `canvas` | unchanged, inside `PipeViewport` | bar x 793, y 81, 603 × 56; canvas x 793, y 217, 603 × 659 |
| `workspace-dock-close` | a page's close control; absent when no page is open | x 1316.1, y 82.5, 71.9 × 32 (measured on the Project page over Review, Table view) |
| `menu-view`, `menu-item-view.section.*`, `workspace-section-*` | unchanged; the View menu gains views, stages, theme and density | x 95.9, y 0 |
| `workspace-review` | the Model stage's second tab. Absent on other stages, where the drivers fall back to the section command | x 110.4, y 81, 114.8 × 27 |
| `open-local-project`, button "Save local", `local-project-message` | the Project page's header and strip. Found by role and name in jsdom at all times; **visible only while the Project page is open**, so `resource-accounting.ts` must open it or use File › Save | x 1039.6, y 82.5; message x 68, y 128 |
| "Appearance theme", "Workspace density" | always-mounted selects in the toolbar's Appearance disclosure. This is a control the specification's toolbar does not list. It exists because `src/design/themeAttribute.test.tsx` needs the label at first render, and the native runtime needs a pointer home | x 1178, y 42.5, 28 × 28 |

**`toggle-inspector` attributes.** I kept `aria-expanded` with `aria-controls="shell-inspector"` and did not add `aria-pressed`. `ensureRail` reads `aria-expanded`, and no driver reads a pressed state.

## 8. Controls table

| Control | Maps to | Held by |
|---|---|---|
| Undo, Redo | `handleUndoSessionModelEdit`, `handleRedoSessionModelEdit`. Disabled with "Nothing to undo" and "Nothing to redo". No key is bound: the product and the native menu bind none, so the tooltips name none | App.shell "Nothing to undo" |
| Select (⎋) | `handleArmCreationTool(null)`. A departure from the specification's toolbar list, kept until the canvas HUD carries it | App.test, unchanged |
| View switch (⌘1 ⌘2 ⌘3) | `chooseStageView`. On Review, the Model and Both segments are disabled with "Review has no canvas" | App.shell pointer rule and mounted panels |
| Run, Stop | `handleRun`, `handleCancelRun`. Run is never disabled: today nothing blocks a solve except a running one | App.shell "runs and stops" |
| Issues (toolbar, rail, status bar) | `setIssuesDrawerOpen` | deadControls; App.test |
| Inspector toggle (⌘I) | `toggleWorkspaceRail("inspector")`. Disabled in Table and Model view with the ruled reasons | App.shell |
| Agent toggle, agent strip | none. Gap G-19 (host). `aria-disabled`, focusable, reason "Agent: not available yet" | App.shell |
| Display units | `DisplayUnitSelector compact`; same behaviour | App.test "view.units" |
| Appearance selects; View menu and palette theme and density items | `setUiPreferences` | App.shell; `themeAttribute.test.tsx` |
| Palette field (⌘K) | `ToolkitPalette`. The seven group shortcuts moved inside the dialog. A View group lists the shell's commands | ToolkitPalette.test; App.shell |
| Rail stages; View › stages | `enterStage` → `setActiveSection`. Same states and reasons as the rail | App.shell rail states |
| Rail Libraries, Rules | `setActiveSection` and `closeShellPage` | App.shell view memory |
| Stage tabs; drawer chevron | `setActiveSection`; `toggleWorkspaceRail("tree")` | App.shell; layout spec |
| Both splitter, drawer splitter | `uiPreferences` only | App.test splitter test; layout spec |
| Page close (⎋), inspector close | `closeShellPage`, `toggleWorkspaceRail` | App.shell |
| Status chips | none; read-only popover | App.shell chip tests |
| About | `setAuditDrawerOpen` (today's audit drawer) | App.test |

**The native runtime reaches these only by the toolbar, the rail and the palette:** the views, the stages, theme and density. `NATIVE_MENU_COMMAND_IDS` is unchanged.

## 9. Semantic changes, named

1. **The dock becomes three views and four stages.** Pages open over the surfaces and return to the stage underneath. View memory is session state. (App.shell)
2. **⎋ closes a page onto its stage and never leaves a stage.** Before, it closed any section. Re-choosing a page in the View menu returns to the stage underneath. Re-choosing a stage surface returns to the model tree, as before.
3. **Summoning a section into a collapsed table drawer opens the drawer.**
4. **Chip rule 1:** one chip, only while the model document records `MODEL_INCOMPLETE`.
5. **Chip rule 2:** no chip. The preview fixture records `ready_for_preview_diagnostics`, so the shell opens with none.
6. **Chip rules 3 and 4:** the Solver chip, plus the Rule pack chip only when `ruleCheckAggregate !== null`. Human appears on the Review page only.
7. **The fixture's recorded `RULE_INPUTS_INCOMPLETE` stays readable** on the Analyze page's `readiness-rule` row ("Rule pack · Rule inputs incomplete (RULE_INPUTS_INCOMPLETE)") and in `RuleCheckPanel`.
8. **Chip rule 6:** a Historical run empties the chip list entirely, as ROOT accepted.
9. **Chip rule 7:** a failed or stopped run leaves the bar empty. An unsolved result shows only its registered Solver status.
   - **Finding:** an unregistered recorded mechanics value, for example `MECHANICS_BLOCKED`, no longer shows its raw token in the bar. It is read in readable form on `readiness-mechanics` and in the deformation status.
10. **The solve proof left the status bar.** It is read on the Evidence tab (`status-pill-solve-proof` kept).
    - The storage badge and the "Local · no network · no telemetry" text are read in the About and audit drawer.
    - The project strip moved to the Project page.
11. **Save state is not shown.** No cell says whether the model was edited since the last save.
12. **The pending-changes count is on the Review changes tab,** visible on the Model stage only.
13. **The Both view's inspector is closed at first open,** where before it was open at 1280 px and wider. The inspector is 300 or 340 px wide and is no longer resizable.
14. **A toolkit command that needs the canvas or the inspector** moves a Model stage left in Table view to Both view.

## 10. Test moves so far

**`App.test.tsx`**

- 18 `status-pill-mechanics` assertions move to a new helper, `expectStatusChip`: the chip's tooltip must equal the token and its face must read "Solver · Mechanics solved" or "Solver · Model incomplete".
  - Old text: `MECHANICS_SOLVED` and `MODEL_INCOMPLETE` in the pill's body.
  - Rules 1, 3 and 7.
- `status-pill-professional` (two sites) and `status-pill-rule-check` (two sites) are now expected absent. The recorded values are read on the Analyze page's readiness rows. Rules 1, 2 and 4.
- `MECHANICS_BLOCKED` and `MECHANICS_NONCONVERGED` are read on `readiness-mechanics`, and no chip is expected. Rule 7.
- "CAD-shell menu": Review changes latches its tab and the page container stays collapsed.
- Two `workspace-review` clicks made from the Results stage become `openWorkspaceSection("operations")`.
- Two `toolkit-group-*` clicks open the palette first.
- The splitter test moves to the split (15 to 85 %, steps of 2) and the drawer (120 to 600 px).
- The ⎋ test pins that ⎋ first tucks the drawer away, and only then reaches the workspace.

**Other unit tests**

- `App.projectHandlers.test.tsx`: one chip assertion.
- `ToolkitPalette.test.tsx`: four tests move to the group band inside the dialog, and focus returns to the palette field.
- `uiPreferences.test.ts`: the two new fields, and a new test that reads an old record.

**e2e**

- The drivers gain `showModelTree`, `showCanvas`, `projectButton`, `projectCommand` (File menu, optionally by keyboard), `expectStatusChip`, `expectNoStatusChip` and `expectRecordedStatusOnAnalyzePage`. `closeWorkspacePanels` and `setAppearance` are moved.
- `workspace-layout.spec.ts` is rewritten: the D-72 geometry test, split and drawer resize with persistence and the 220 px stop, and the narrow fallback.
- `gui-workflow-validation`, `linear-authoring`, `r2-smoke` and `ui-foundation-dist` are partly moved.
- `ui-foundation.spec.ts` is unfinished.

## 11. For others, and lists for the closing pass

**Canvas lane**

- The authoring panel and the docked inspector (section 1).
- The strip overlay (section 2).
- The three instrument conditions still stand, and the inspector is now closed at first open.
- The native canvases are 848 high at 1440 × 920.

**Frame against D-72.** `s2_model_light` keeps the collapsed strip in flow. The build lays the strip over the canvas's foot, per ROOT.

**Contrast not yet checked**

- every `ShellToolbar` control;
- the view switch;
- the rail items, with their disabled and caption states;
- the agent strip;
- the stage tabs and the chevron, including the disabled chevron, which uses `--ui-disabled-text`;
- the status chips and their popover;
- the About and Issues buttons;
- the page header and its project buttons;
- the inspector close control;
- the menu's disabled reasons;
- the palette's View commands and group chips.

**Appearance left for the closing pass**

| Place | What it uses now | Note |
|---|---|---|
| Rail selection | a 3 px `--ui-accent` bar | no filled glyph |
| Rail captions | `--rail-caption*` | — |
| Run button | `--accent-fill` with `--text-inverse` | — |
| Latched toggle | `--pressed-fill` with `--pressed-ink` | — |
| Splitters | invisible until hover or focus | — |
| Popovers | `--surface-raised` with `--elevation-2` | — |
| Chips | plain bordered pills | no per-status tint |
| Toolbar icons | lucide | not the design's icon set |

I claim no usability, conformance or performance acceptance. PDU-045 and PDU-046 remain holds.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).