# B3-SHELL return: stage 2, final candidate

**This is not a final candidate.** I stopped on a frozen-geometry test, and the dist lane's test moves are not done. The heading is the one the relay asked for. Neither full Playwright lane was run.

- **Model:** Claude Fable 5.1 (`claude-fable-5-1`), by my own statement. TASK, Type 2; I launched no agent.
- **Base:** head `f48168b679c153bdabeb3585942199b081940248`, which contains my interim stage-2 work as `587aba1f9`. This return covers only what changed on top of it. For everything unchanged since the interim return, see that return.
- **Method:** I wrote through the shell only and ran no state-changing git. After the cut, all six files were whole.

## 1. Stop: the frozen Box16 endpoints, with the numbers

The same test fails in both lanes: `e2e/ui-foundation.spec.ts:1960` and `e2e/ui-foundation-dist.spec.ts:1467`, "decorative viewport overlays pass real canvas gestures while view controls stay interactive". I did not touch it.

**What it measures.** Two "frozen Box16 endpoints", given as fractions of the drawn canvas: start (0.29, 0.58) and end (0.08, 0.81). The test requires the end point to land inside the 96 × 96 axis triad. It sets the window to 1440 × 899 so that the canvas matches the geometry the endpoints were characterized against.

**Numbers under the new shell** (browser runtime, Both view):

| Quantity | Value |
|---|---|
| Drawn canvas top | y 217 |
| Drawn canvas height | 658 (899 − 24 − 217) |
| End point's y | 217 + 0.81 × 658 = **749.98** |
| Axis triad | y **767** to 863 (bottom offset 12) |
| Result | the end point is 17 px above the triad |
| End point's x | 793 + 0.08 × 603 = 841.2, inside the triad's 805 to 901 |

- The condition `0.81 h > h − 108` needs a canvas height under 568 px. It was 560 before; it is 658 now.
- With the docked inspector, or in the native runtime at 692 high, it misses as well.
- The taller canvas is the design's. Whoever owns the frozen characterization (ROOT and the canvas lane) decides what becomes of this test.

## 2. Decision 1: the interim armed-tool rule is built

I built the rule ROOT described, not the exclusive fallback. It came out cleanly.

**What "the panel is active" means.** `PipeViewport`'s `viewportIntentPanelActive` is true when:
- a **node, pipe or component** tool is armed, **or**
- an intent queued from the canvas is pending, that is, `source_role === "viewport_editor"` or an id starting `op:viewport-intent-`.

Support and Load do not show the panel. The rule is therefore keyed on what the session knows: `canvasAuthoringPanelActive(armedCreationTool, editorIntents)` in `shellLayout.ts`.

**One risk.** That predicate restates the component's, because `features/viewport/**` is mounted and not modified. If the component's condition changes, the two can drift. The e2e test holds them together by asserting that the `data-canvas-authoring` attribute agrees with the `.viewport-intents.active` class.

**Table pane minimum.** I chose **320 px** (`SHELL_REGIONS.tablePaneLendingMinPx`).
- The table lends the inspector width down to 320 px and no further; past that the canvas gives the rest.
- A table already under 320 px by the engineer's split lends nothing.
- The canvas's 220 px minimum still wins over everything.
- Specification §10.9 says the table has no floor. This minimum is part of the interim departure.

**Geometry** (pinned by unit tests and by `workspace-layout.spec.ts` in the native-runtime class; screenshot measured):

| Window | No tool, inspector docked | Tool armed, inspector docked | Armed, split at 29 % |
|---|---|---|---|
| 1440 × 900 | table 737, canvas pane 303 | table **437**, canvas pane **603**, inspector 300; drawn canvas 338 × 659 | table 320, canvas 720 |
| 1280 × 800 | 649 and 231 | **349 and 531** | table 320, canvas 560 |

- Nothing closes and no tool is disarmed.
- The splitter follows the table's edge and keeps the stored split at 55.
- Disarming the tool restores the specification's rule.

**The frozen 200 × 200 floor test now passes.** Its value and its scenario are untouched. It passed at 1024, 1280 and 1440 in light comfortable, run alone.
- Under the rule the before and after states already agree, so I removed my earlier "close the inspector to compare like with like" line from that test.
- I also removed the `ensureInspectorCollapsed` step in `r2-smoke.spec.ts`; that gesture passes without it.

## 3. Decision 2: the strip and the canvas furniture

This is the position-only edit ROOT granted.
- `.viewport-axis-triad` now has `bottom: calc(12px + var(--shell-canvas-foot-inset, 0px))`, and `.viewport-scale-bar` has the same with 14 px. No other property of those rules changed.
- The shell sets the variable on the canvas pane in three cases:
  - Model view with the drawer collapsed: 28 px.
  - Below 1280 px with the drawer collapsed: 28 px.
  - Below 1280 px with the drawer open: the drawer's height.

**Measured**, native class, 1440 × 900, drawer collapsed. The strip starts at y 848.

| Furniture | Extent | Clear of the strip |
|---|---|---|
| Scale bar | y 801.2 to 834 | yes |
| Axis triad | y 740 to 836 | yes |

An e2e test asserts that neither rectangle intersects the table pane in four states: Model view with the drawer open, Model view collapsed, narrow open, narrow collapsed. I found no furniture positioned from inside `features/viewport/**` that the strip reaches; the deformation status sits at y 104.

## 4. Checks

| Command | Result |
|---|---|
| `npm run build:desktop` (from `projects/chirality-piping`) | exit 0 |
| `npm run test:desktop` | exit 0; **80 files, 1,285 tests passed** |
| `npm run test:e2e`, `npm run test:e2e:dist` (full lanes) | **not run**: the candidate is not final |

**Triage through the lock.** I queued behind ROOT's sweep and the other lane each time.

Source lane, `chromium-desktop` project only:
- Passing: all 10 tests in `workspace-layout.spec.ts`, `gui-workflow-validation`, `r2-smoke`, `linear-authoring`, `result-compatibility`, and `ui-foundation.spec.ts` except the frozen test in section 1.
- The `chromium-compact` project was triaged only for `workspace-layout`, `linear-authoring` and `gui-workflow-validation`, and only before my later edits. It is unverified now.

Dist lane, `--config playwright.dist.config.ts`, run once as triage: **23 passed, 30 failed, none moved yet.** By group:
- `toolkit-batch-dist:54`: `undo-session-model-edit` is not visible.
- `ui-foundation-dist:45` preflight: a centre is covered.
- `ui-foundation-dist:127`: expects `toggle-inspector` open at first load.
- `ui-foundation-dist:239`: a rail handle's centre is covered by `workspace-review`.
- `ui-foundation-dist:302` (4 variants): expects the dock not collapsed.
- `ui-foundation-dist:497` (2 failures logged; their titles were not captured): uses `resize-property-inspector` as an obstacle.
- `ui-foundation-dist:617` (12 variants):
  - the collapsed-rail hover label `.workspace-pane-toggle-label` no longer exists;
  - the inspector is expected open.
- `ui-foundation-dist:1027` (2 variants): the dock header's "Results" heading.
- `ui-foundation-dist:1198` (4 variants): the old rail-preferences test. It should become the split and drawer test already written in `workspace-layout.spec.ts`.
- `ui-foundation-dist:1279`: `resize-property-inspector`.
- `ui-foundation-dist:1334`: `workspace-dock-close` on a stage surface.
- `ui-foundation-dist:1467`: the frozen test in section 1.

Most of these duplicate tests I have already moved in the source lane. It is a large mechanical job, and I left it rather than start it near the usage limit.

After the last unit run no source file changed; the screenshots were written before it. Ports 5174 and 5175 are closed.

## 5. Files changed on top of `f48168b67`

All hashes are SHA-256. Every other stage-2 file is as committed in `587aba1f9`.

Under `apps/desktop/`:

| File | SHA-256 |
|---|---|
| `e2e/r2-smoke.spec.ts` | `e645e608476163bd4b35ecc1caa1d7f3fc794cc37f1dc39775639849b8aadeca` |
| `e2e/ui-foundation-workflows.ts` | `c805f3e52492b52cf731f2e4368577ed80c39bbafa74b41fc804e95724052872` |
| `e2e/ui-foundation.spec.ts` | `6c3a7f301163ebf15021e0f4283a54c929d73688ca87d354854183954ab7acf3` |
| `e2e/workspace-driver.ts` | `ca4d1772ecc47b478c2d29917ffc5f9c756451ec9471fb434894e738f80ee0a7` |
| `e2e/workspace-layout.spec.ts` | `71d2e072b0e3e5ccfb2325de07eebdd8126ae5250a8fc72eff216bfbfcade2b8` |
| `src/App.tsx` | `82bd2aaed7758e9693daeab7e2fb40ad1e64ebb03668863fd0c63d817c353bc8` |
| `src/features/workspace/shell/AgentStrip.tsx` | `fbd9b3ed429f180afa364a098d0f51ae02239e2b37d1ae1552cb26e18e9f5e3b` |
| `src/features/workspace/shell/ShellToolbar.tsx` | `f8a170d662843d22f5327a73dec032c296aae34648a91d3bdbfcdb6dbc6dc95c` |
| `src/features/workspace/shellLayout.test.ts` | `8c698a469c0f744493c5ea909b618214a423be31e82d39a7f72272ef6b1db2d1` |
| `src/features/workspace/shellLayout.ts` | `dd4297efd7dab42d3b5962ed4e63ddafb61d50433bba8645ad669308ec9844da` |
| `src/styles.css` | `3ab97e605bec2470ec730edf079a8143efdf01ef1fd555491865e40516e812db` |

Screenshots retaken, under `{RUN}/instances/B-SHELL/shots/B3/`:

| PNG | SHA-256 |
|---|---|
| `loads_both_light` | `f95ee5fa5e11904dcfa4edd7c6189c96199af29f144fa9896e2805a0a966bf43` |
| `loads_model_light` | `0b142d374669a5170152233e53df9e5566775bdaf01b34691fd2ae39a1784423` |
| `loads_table_light` | `71d11449ac2f46fd44580294fe70b0dd815a654d4b14115f940a995e18ab6631` |
| `model_both_dark` | `be72c1954240158379e4579bef9257a252f4214d722ca5712e141925830d7ba0` |
| `model_both_inspector_light` | `f14f5e7f8d4cf75a98d982e098572e6d64c76a02e5827d41a4a1a0a5d7ca47c1` |
| `model_both_inspector_tool_armed_light` (new) | `43ac26cbcaf225c4b8fb3de388f7864df77372454546686d75e6c6edb644cbb5` |
| `model_both_light` | `b59aa56abf89085c55bd7ffa81417afdcf1c6dd83ea494fc78610352f2e2843f` |
| `model_model_drawer_collapsed_light` | `dc283bbcb0c7db4e05f804bf541d7dba219bd6a51a10f352bfcb2841ae4bfb82` |
| `model_model_light` | `5b19b02e5582b9da9f74a545dbe8aca6c4cb9d1d0a28415fe0bb5c7582094220` |
| `model_table_light` | `41b0404a30a3ebda5f529953436edaf9a859e7782ea5796117a645c0bfe44f85` |
| `native_model_both_light` | `d889fc19967b30c4e3dea84003691650bb77023bfe34c551f7c19f03b72539c9` |
| `native_model_model_drawer_collapsed_light` | `2064da61735c4a3e3fbc38bc0b9373f3dc6f15afb5e99c3bbfcdc5fefd4400f1` |
| `project_page_light` | `695a9f59e0c46a6c43f12b18a6ee62874114094cbb40a7747b9bbfb3db2c8a8a` |
| `results_both_light` | `77f05ae37134fbd79b9d9a1f990785f1a24e82009989d7591fff082b8083fb18` |
| `results_model_light` | `58a96bbdd9dad0ccb14a83ba6f92e11e423a3ddf3bf1a893dd2e1f2e44b696b1` |
| `results_table_light` | `c50c2819723b41dd7884dff4086d1fe88935786151abba8abbfbf8d09b1b178c` |
| `review_table_light` | `7f5e78f4f39d464b69c6017c6dccc203ff990f00698a0ffdc3df6e12c25a8dc3` |

## 6. Other changes this round

**Product:**
- The toolbar's Issues button now carries the label "Issues" with its count, and the agent strip the label "Agent", as the frames draw them.
- The solve proof's recorded line opens in flow on the Evidence tab. The old upward popover rule had put it out of reach there.

**New e2e tests in `workspace-layout.spec.ts`:**
- The interim rule, at both sizes.
- Strip and furniture clearance.
- "A switch to Table view hides the canvas without resizing its renderer": the buffer size, the CSS size and the camera are equal before the switch, during it and after it.

**Driver fixes:** `showModelTree` now closes an open page first. A page lies over the tree without hiding it from layout.

**Test moves in `ui-foundation.spec.ts`:**

| Line | Old | New |
|---|---|---|
| :63 | Units disclosure | the toolbar's "Display units" combobox |
| :385 | `filter.fill("")` | `showModelTree` first, then the fill |
| :1084 | dock header "Review changes" | the latched `workspace-review` tab |
| :1239, :1259 | snapshot read after a tree selection | reopen Review changes and its Geometry or Self weight sub-tab, then the same visibility assertion |
| :1412 | "Resize model tree" and "Resize property inspector" | "Resize table and canvas", with the inspector separator asserted absent; "Resize task dock" becomes "Resize table drawer", in Model view, with a bound check added |
| :1820 | `workspace-dock-close` from Results | `showModelTree` |

**Test moves in `r2-smoke.spec.ts`:**
- `layout-mode-tree` is now clicked after `ensureTreeExpanded`.
- One `viewport-deformation-status` step comes after `showCanvas`.

**Status-pill moves:** nothing new; the inventory in the interim return stands.

## 7. Semantic changes to add to the interim list

15. **INTERIM: the armed-tool rule** of section 2.
    - Today's specification: the inspector always takes its width from the canvas, and the table never moves.
    - Now: while the canvas's authoring panel is on screen, the inspector takes its width from the table, down to 320 px.
    - Retires when the routing block moves into the inspector.
    - Held by `shellLayout.test.ts` and `workspace-layout.spec.ts`.
16. **Furniture lift.** While the table strip or drawer lies over the canvas's foot, the axis triad and the scale bar sit above it by the strip's or drawer's height.

**Still for the owner's list:**
- The save state is not shown. Operations-map row 11 derives it from the model hash against the last saved hash, and the session holds no last-saved-hash cell.
- An unregistered recorded mechanics value no longer shows its raw token in the status bar.

## 8. Differences and things for others

- **Specification §10.9, states C to E.** The slide-over inspector and the agent-column collapse are not built; that is B5. When the split leaves the canvas under 220 px with the inspector docked, the build holds the canvas at 220 px and the table gives way. The specification has the inspector slide over instead.
- **Frames.**
  - The Issues tooltip in the frames is "Issues (⌘⇧I)". No such key is bound, so mine reads "Issues".
  - `s2_model_dark` puts the control first in the Inspector tooltip; the specification puts the summary first. I show the reason alone.
- **Canvas lane, through ROOT:**
  - section 1;
  - the predicate duplication in section 2;
  - `resource-accounting.ts` needs the Project page open, or File › Save;
  - the instrument's two-pane boundary capture needs the inspector opened first.
- **Contrast not yet checked** and **appearance left for the closing pass:** as listed in the interim return. Two controls are added to the first list: the Issues label and the Agent strip label.

## 9. What remains before a true final candidate

- ROOT's decision on the frozen test in section 1.
- The 29 dist-lane moves in section 4 (30 failures, less the frozen test).
- A `chromium-compact` pass of the source lane.
- One run of each full lane, and the consolidated full return the brief asks for: the controls table, the hooks table and the status-pill inventory, merged from the interim return.

I claim no usability, conformance or performance acceptance. PDU-045 and PDU-046 remain holds.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).