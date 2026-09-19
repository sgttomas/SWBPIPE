C1A-PALETTE return: canvas colours now come from tokens, the figure is matte and unlit, and both themes repaint live. Every check in the brief ran and passed.

## 1. Who I am and how I worked

- I am C1A-PALETTE, a TASK (Type 2) implementer under B-CANVAS, running as model `claude-fable-5-1`.
- Before anything else I ran `shasum -a 256` on `{RUN}/instances/B-CANVAS/briefs/C1A-PALETTE.md`. It gave `e2c8fffe4957d8ce112a66d0a2449755cf77a499182ed51b1e59482d7d2db77b`, matching the launch message.
- I worked alone and launched no agent. Harness notices named three background agents that belong to the session that launched me; I sent them nothing.
- I ran no state-changing git command. The git commands I used were `rev-parse`, `status`, `diff`, `show`, `ls-files`, `check-ignore` and `archive` (to a scratch directory outside the repository, deleted afterwards).
- I worked only in the assigned worktree, on branch `codex/swbpipe-b-canvas-20260918`, with HEAD at `de348f2cd683165afcf6c399268b285f2ccb369a` throughout.
- **One thing for you to judge:** this session's file-writing tool refused paths in the assigned worktree, because the session itself was opened in a different worktree of the same repository.
  - The launch message assigns this worktree.
  - `ORCHESTRATION_PLAN.md` and `lanes/B-CANVAS/INDEX.md` establish lane worktrees.
  - The session told me to make file changes through the shell.
  - I therefore wrote every file with shell heredocs and small exact-once Python replacement scripts, inside the assigned worktree only. Throwaway scripts, logs and the look script live in the session scratchpad outside the repository.

## 2. What I read

**Whole:**
- the brief (twice, the second time after a context compaction);
- `{RUN}/instances/ROOT/ACTIVATION_2026-09-18.md` and `{RUN}/tools/with_e2e_lock.sh`;
- `{DESIGN}/instances/ROOT/IMPLEMENTATION_HANDOFF_2026-09-18.md`;
- `{DESKTOP}/src/features/viewport/viewportResource.ts`, `viewportResource.test.ts`, `viewportSelectionPresentation.ts` and `selectionPresentationBinding.ts`;
- `{DESKTOP}/src/design/TOKENS_SOURCE.md` and `src/design/tokens.test.ts`;
- `playwright.config.ts`, `vite.config.ts`, `package.json` and `e2e/workspace-driver.ts`;
- three r181's eight chunks named in the brief, and `helpers/GridHelper.js`.

**In part:**
- `DESIGN_SYSTEM_V1.md`: head, §0, §2.1, §2.6, §2.7, §2.8, the §2.9 method with its `canvas.*` and `cat.*` rows, and all of §6.1 to §6.11.
- `UX_SPEC_V1.md`: §4.1 and §12 with its closing paragraph. I did not read §4.5.
- The two frames `s2_model_light.png` and `s2_model_dark.png`, viewed and pixel-sampled.
- `tokens.json`: structure, note, and all `surface.*`, `canvas.*` and `cat.*` values.
- `PipeViewport.tsx`: imports, the model, result and routing effects, and all builders.
- `e2e/r2-smoke.spec.ts`, `e2e/linear-authoring.spec.ts`, `e2e/ui-foundation.spec.ts`, `e2e/ui-foundation-workflows.ts`, `e2e/ui-foundation/ui-foundation-performance.benchmark.ts` and `playwright.dist.config.ts`.
- `scripts/build-wasm-engine.mjs`, the theme lines of `App.tsx`, and `src/features/workspace/uiDiagnostics.ts`.
- three's `WebGLProgram.js` define lines and the DFG lookup lines of `WebGLRenderer.js`.
- `ORCHESTRATION_PLAN.md`, `HANDOFF_STATE.md` and `lanes/B-CANVAS/INDEX.md`, by search, for how worktrees are used.

## 3. Files changed or added, with SHA-256

All under `{DESKTOP}/src/features/viewport/`:

| File | State | SHA-256 |
|---|---|---|
| `PipeViewport.tsx` | changed (+60 −322) | `9f43616c06359481abef1d5747e78ba72835f1c47eeb3ee53494b67652bdcd47` |
| `viewportResource.ts` | changed (+167 −14) | `40a0430e7f3103c1c274a735781c4d04076be7e5cc9f1eb3cc924ffca2439a8c` |
| `viewportResource.test.ts` | changed (+225 −0; no existing line touched) | `91bed80c3b2111d5629e6a81a270c336ef5de7a3ffc34e31db4c397f6726b0af` |
| `viewportPalette.ts` | added | `afd1867d453d5eedaa19fbc5a593f344bf42557c93f8d418cadb5fe41f6be3f1` |
| `viewportPalette.test.ts` | added | `98db4beda3a04a02bd3603e0086978c56cdc81cf03b5252751e599ba143a9de9` |
| `viewportFigureMaterial.ts` | added | `7fb742f07c18703158299b533ac9a369a6d2015854284cd9132ded2f9c90f17f` |
| `viewportFigureMaterial.test.ts` | added | `e21db16ee9b911cecdd572cd68ab61c9db42bf51b1fbd914f5037f674feca5c1` |
| `viewportColourLiterals.test.ts` | added | `23b0363d896c94ed92d33533265afcb0043f15a98674786565931c36d5013856` |

The two hash-bound files are byte-identical at the end:
- `viewportSelectionPresentation.ts`: `00384d2831797e5cba8acff21e82e36f21a853c398e8c2a801ccfed0f3a55931`
- `viewportSelection.ts`: `fdf3eaa49b9685b932bce404421086c45a82fa22e5c8abc3f8146fc09cb846d5`

Nothing changed under `e2e/**`, `App.tsx`, `styles.css`, `tokens.json` or `tokens.css`.

**Screenshots** are in `{RUN}/instances/B-CANVAS/screenshots/C1A/`: 34 PNG files of the viewport region, 1:1 at device scale factor 1.
- The window was 1440×920 and the canvas 794×580 (529×580 while routing).
- The "before" images were taken from the unchanged tree.

Before set:

| File | SHA-256 |
|---|---|
| `before_dark_deformation.png` | `9b20c3241634af2626bee1c174f334527d202505bdffc66df056a09365ed3532` |
| `before_dark_deformation_selected.png` | `d6b9b1188aa581aa4939b710f8ec7e7ed9081f713642400060f17daa3cc1db51` |
| `before_dark_model.png` | `41875204453d21939a2834af38adfc361c4734ac4c922279a3f4ec18b98d9a13` |
| `before_dark_model_nolabels.png` | `55258833ad57ec010ffafa6b2c28fd598ab48863d5cd024c4da37fd4ba235877` |
| `before_dark_route.png` | `e470f2754f4d416b4457526ec5d388d0135f53ecd7ec1d82920f2e0813f85741` |
| `before_dark_selected_actual_od.png` | `5dd4e7089c98b7a3acd14c5f38b351e484e27ea01d63e3e48b826dcb3d7c1236` |
| `before_dark_selected_schematic.png` | `cfb49245b7efee1c76e82d6940ab7fe9253d88b22b0debbc7685962445aaf1cd` |
| `before_light_deformation.png` | `49fe7a791bf70937026cbb3080cb4f416832243059a97db89cbccb564a5b9988` |
| `before_light_deformation_selected.png` | `4781393283fb31e98f0e661f88bee8cc7a3890429700b5d4bc84fdc65272df6e` |
| `before_light_model.png` | `6a3f267c7cc03cd0fd8cb828d691567011d1e08bc2b50544358f71ebabe762f9` |
| `before_light_model_nolabels.png` | `70121eb65c85c89ae0d8874a3fd0226c8bc7d24546c69c3744b1f25036eebe5f` |
| `before_light_route.png` | `fcfb85914bb4b81545bc9c5ee647713ddfd466e883c141cd6af54407b7fa3f0e` |
| `before_light_selected_actual_od.png` | `745496a631c3cb59a30abfce6dcad88c74530e63354be26aae1ae415fd3c7cfa` |
| `before_light_selected_schematic.png` | `d972b8622021bf24cde50dacf01c6c9df98e298ae245abf21e156b6c25dd876f` |
| `before_live_1_light.png` | `0bd442f464b5c2b1dd65b409df14767dd6eaf608d31ca0c39ffa3f9b1732c648` |
| `before_live_2_dark.png` | `178a864112d2bdfb20937423ea707f32e217020ea2201d9cfbc8475f181f27db` |
| `before_live_3_light_again.png` | `0bd442f464b5c2b1dd65b409df14767dd6eaf608d31ca0c39ffa3f9b1732c648` |

After set:

| File | SHA-256 |
|---|---|
| `dark_deformation.png` | `4e71e3b562f5112551f21e05d84b6c0eb854881d42ab9e606c0817ea9b955d4a` |
| `dark_deformation_selected.png` | `d45ab647e894851bb7e8e048efa824145f28ce7730879b7c7f8837539cea4569` |
| `dark_model.png` | `b1971c4dbe18362ea2c2873d2d1d95cca01905d099f1674d6c548ce2830bf1dd` |
| `dark_model_nolabels.png` | `e52008931c44b43a7131ddfa0bed25c3ab7487bd5bd2944fc4bfc803a34419a2` |
| `dark_route.png` | `9ae4e17999b574633eaf190700b7ae2de50f9163cbc2e87f2ff2490415568cbd` |
| `dark_selected_actual_od.png` | `f9c2b6ebc7c6aa193685af2401ee71a442a9482c1293a26f8b4eda944adff73e` |
| `dark_selected_schematic.png` | `3cccf23e02ea24c280e4f42baaaaa597134007aab6b01551bf866702b4aea39e` |
| `light_deformation.png` | `a969e0337bcdacfdc8b6a7d04b50ee1cd86f258866d50a3f6743acba71b8ac25` |
| `light_deformation_selected.png` | `c830f2e0f311365e5fa6222fe2089ebfdd9c6542e5349cae364aa08fb5a40b1b` |
| `light_model.png` | `e011c2453e7f102395a33bcf3e3d1c444eb3587286a7cf0f8b3795d9696dfe38` |
| `light_model_nolabels.png` | `dc198c2ddbbe6af76823e3b70dd219e7ace3b492a1849c3e3f6753cc84e66e40` |
| `light_route.png` | `46e42d68d763e3266f4170585a99cf071e9a219bba2bc95faede98cf66848bee` |
| `light_selected_actual_od.png` | `05882d291e5a665480d5f982b833a1b95b622da8a29c0e94463eb072331cb1e2` |
| `light_selected_schematic.png` | `3eaa868bdc7da7192fa56f3315025e359bc4fe53baf722f7121994fe219ba432` |
| `live_1_light.png` | `45807f5e944ddfdf44a3e36dd7c9c9a0a13f082e3416a0e71856114dc58b5de0` |
| `live_2_dark.png` | `6282f28e49f4077324d7a77a680ad0cb3e4ab776a8e4647b74159b84c49ba7e8` |
| `live_3_light_again.png` | `45807f5e944ddfdf44a3e36dd7c9c9a0a13f082e3416a0e71856114dc58b5de0` |

What each state shows:
- `model`: grid, loads and labels on.
- `model_nolabels`: the same with labels off.
- `selected_actual_od`: `pipe:P-110` selected with Actual OD on.
- `selected_schematic`: the same selection in schematic radius.
- `deformation_selected` and `deformation`: after the sample solve completed ("available; nodes=5; max=4.927112 mm"). The second has `support:S-100` selected instead of the pipe, which is why one cone is orange there and in `route`.
- `route`: a pipe route started from `node:N-140` with the hover ghost shown.
- `live_1` to `live_3`: one page with `pipe:P-120` selected, switched light to dark to light with no reload.

## 4. How the work was done, where it is not obvious

**Palette (`viewportPalette.ts`)**
- It imports `../../design/tokens.json` and parses it once at module load into two frozen tables, with no DOM read.
- It exposes `parseTokenColour`, `VIEWPORT_PALETTE`, `VIEWPORT_ROLE_TOKENS` (the brief's table, unchanged), `viewportTokenColour`, `viewportRoleColour`, `viewportRoleHex`, `isViewportPaletteRole` and `viewportShadeRatio`.
- The parser's documentation avoids the literal forms, so the source-scan test needs no exception for it.
- The measured shade ratios are light [0.5757, 0.5949, 0.6058] and dark [0.4863, 0.5044, 0.5213].
- This is the first product module to import `tokens.json`, so the token file (13,671 bytes raw) is now in the main bundle.

**Figure material (`viewportFigureMaterial.ts`)**
- It is a `THREE.ShaderMaterial` with `lights: false` and no texture.
- It uses exactly the eight chunks the brief names, and the uniforms `tint`, `opacity` and `shadeRatio`.
- The fragment is the brief's formula. `isOrthographic` is respected for the view direction.
- The material exposes a read-only `color` property that is the `tint` uniform's `THREE.Color`. The foundation's selection code and the repaint therefore treat it like any coloured material without a branch.
- Its default tint is white, so an instanced figure draws its instance colour alone.

**Roles**
- I used companion functions, not a fourth argument: `registerInstancedRolePresentation(mesh, keys, role, theme?)`, `registerPaletteRole(object, role, theme?)` and `registerGridPaletteRoles(grid, centreLineRole, lineRole, theme?)`.
  - The first stores `userData.viewportPaletteRole` and calls the unchanged `registerInstancedSelectionPresentation`.
  - The third stores a frozen `userData.viewportPaletteGridRoles`.
- `applyPalettePresentation(roots, theme)` paints in place:
  - instance base colours, and rewrites `userData.viewportBaseColor`;
  - material colours and tints, and rewrites `material.userData.viewportBaseColor` where a selection registration put a number there;
  - the figure materials' `shadeRatio`;
  - line-material colours;
  - each `GridHelper`'s colour attribute, in three's layout: four vertices per line index, the centre index in the first role and every other index in the second.
- A mesh registered without a role keeps the number it was given, and a test covers this.
- Builders no longer call `setColorAt` with a new `THREE.Color` per instance. Registration paints the base colours.
- Builders paint for the default theme (light) and `replaceLayer` repaints the incoming objects for the resource's theme. In dark, a layer replacement therefore paints role objects twice. This happens on replacement only, never per frame.

**`setThemePresentation` order**
- Background (held values, reusing the existing `THREE.Color`).
- Gizmo.
- Role colours in all five layers.
- Selection in the four layers that carry one.
- Cue.
- One `invalidate()`.

**`replaceLayer`**
- It paints only the incoming objects before selection is applied.
- Painting only those objects also keeps the file's existing fake resources (which have no routing layer) working unedited.

**Scratch colours**
- `applyInstancedSelection` and the gizmo repaint reuse two module-level scratch `THREE.Color` objects.
- What they paint is unchanged.

**Held block**
- `viewportResource.ts` has one block between `// HELD-COLOURS:BEGIN` and `// HELD-COLOURS:END`, with the comment the brief asks for.
- It holds six named constants: `HELD_SCENE_BACKGROUND_LIGHT/DARK`, `SELECTED_COLOR_LIGHT/DARK` and `HELD_SELECTION_CUE_RIM_LIGHT/DARK`.
- `GIZMO_THEME_PALETTES` keeps its shape. `canvas` is the held background, and `badge` and `axes` come from the palette.
- The gizmo contrast test passes unedited. Axes on the held ground measure 4.14/3.59/3.84 light and 7.24/8.62/8.19 dark. Axes on `surface.raised` measure 5.26/4.57/4.89 light and 4.93/5.86/5.57 dark.

**Lights removed**
- No lit material remains in the scene. What is left is the figure material, `MeshBasicMaterial` arrows, line materials, the selection cue's own shader and gizmo sprites.
- I removed the `AmbientLight` and the `DirectionalLight` from the `ViewportResource` constructor.

**Load arrows**
- They stay `MeshBasicMaterial`, now white, so the instance colour is the drawn colour (see change 5 in section 6).
- Moments are still built first, so the order of output objects is unchanged.

**Grids**
- Opacity is 1.
- The materials keep `transparent = true`, and the routing grid keeps `depthWrite = false` and `renderOrder = 1`. Pass and draw order are what they were.

**Dead code removed from `PipeViewport.tsx`**
- Each of these had zero word-boundary references across `{DESKTOP}/src` and `{DESKTOP}/e2e` after removal:
  - `pipeMesh`
  - `supportMesh`
  - `componentMesh`
  - `deformedPipeMesh`
  - `deformationMarker`
  - `referenceGround`, the `model` overload
  - `routeConstructionGrid`, the `model` overload
  - `buildOrientationGizmo`
  - `axisLabelSprite`
  - `toVector`
  - the file-local `isRigidComponent`
- The local variable `referenceGround` in the model-layer effect remains.
- The `isRigidComponent` functions in three other files are their own file-local copies and are untouched.
- None of the listed builders turned out to be referenced.
- Kept because they are live: `isBendComponent`, `isBranchComponent`, `isExpansionJointComponent`, `marker`, `referenceGroundFromBounds` and `routeConstructionGridFromBounds`.

**Source scan**
- The scan pattern found 70 colour literals in `PipeViewport.tsx` and 19 in `viewportResource.ts` at HEAD.
- It now finds the six held ones and nothing else in any non-test file under the folder.

**No existing test was edited.**
- `viewportResource.test.ts` gained import names and one appended `describe` block of six tests.

**Tests added: 26**
- `viewportPalette.test.ts`, 10 tests:
  - the token census against the token file, read from disk by a separate route;
  - every token in both themes;
  - the two translucent tokens' alpha;
  - frozen tables;
  - the accepted forms and 25 rejected values, each reporting the offending value;
  - the role table;
  - every role in both themes;
  - two literal spot checks;
  - the shade ratio checked against three's own colour conversion.
- `viewportColourLiterals.test.ts`, 4 tests:
  - the pattern itself;
  - which files are found;
  - the held block as exactly `0xdfe5e8, 0x0c1114, 0xa34400, 0xf08c22, 0xffffff, 0x0c1114`, and nothing but constants;
  - no literal elsewhere.
- `viewportFigureMaterial.test.ts`, 6 tests.
- Repaint tests, 6:
  - dark then light token on a role mesh, with `shadeRatio` and `viewportBaseColor`;
  - a selected instance keeps the held colour while an unselected one takes the new base, including deselect after a repaint;
  - a `GridHelper` carries the two tokens, with the same attribute and geometry objects;
  - `replaceLayer` paints for the current theme, including the routing layer;
  - the five-layer repaint:
    - the ledger snapshot and the published owned-resource snapshot are unchanged;
    - no dispose call is made;
    - every geometry, material, colour attribute and instance-colour attribute is the same object;
    - `invalidate` is called once per repaint;
    - the 0.82 opacity is kept;
  - a mesh without a role keeps its number.

## 5. Checks

| # | Command (from) | Exit | Counts |
|---|---|---|---|
| 1, first | `npx vitest run src/features/viewport/viewportSelection.test.ts` (`{DESKTOP}`), before any change | 0 | 1 file, 69 tests passed |
| 2a | `npm run test:desktop` (`{WORKING_ROOT}`) | 0 | 78 files, 1216 tests passed, 0 failed, 0 skipped |
| 2b | `npm run build:desktop` (`{WORKING_ROOT}`) | 0 | `tsc -b` clean; 1721 modules transformed; only the existing chunk-size warning |
| 3a | `PLAYWRIGHT_WORKERS=1 sh {RUN}/tools/with_e2e_lock.sh npm run test:e2e` (`{DESKTOP}`) | 0 | 394 tests: 374 passed, 20 skipped, 0 failed, 0 flaky; 9.3 min |
| 3b | `PLAYWRIGHT_WORKERS=1 sh {RUN}/tools/with_e2e_lock.sh npm run test:e2e:dist` (`{DESKTOP}`) | 0 | 53 tests: 53 passed, 0 skipped, 0 failed; 2.8 min |
| 4 | `python3 tools/validation/validate_claims_language.py` (`{REPO_ROOT}`) | 0 | "VALID claims-language surfaces: 323 files scanned; DEC-081 registry taxonomy satisfied" |
| 1, last | `npx vitest run src/features/viewport/viewportSelection.test.ts` (`{DESKTOP}`), after everything else | 0 | 1 file, 69 tests passed |

- **Unit suite log:** the log carries jsdom's "Not implemented: HTMLCanvasElement.prototype.getContext" traces from `new THREE.WebGLRenderer` in the `ViewportResource` constructor. I did not touch that line. I did not count those traces before the change.
- **Source-lane skips:** all 20 skips are in `e2e/ui-foundation/full-cohort-controller.spec.ts`, ten per project, which is the expected `D70_WRITER_*` skip.
- **After each Playwright lane:** the lock directory was absent and nothing listened on 5174 or 5175.
- **Additional runs, all exit 0:** `npx tsc -b`, and `npx vitest run src/features/viewport` (9 files, 172 tests).

**Check 5, the look**
- I used a throwaway Playwright script outside the repository, run as `sh {RUN}/tools/with_e2e_lock.sh sh <scratch>/look.sh ...`.
- The script started `npx vite --host 127.0.0.1 --port 5184 --strictPort` and drove the installed Google Chrome 153, headless. WebGL 2 ran on hardware (ANGLE Metal on the host's Apple GPU).
- Every run ended with "port 5184 free" and the lock directory absent.
- There were four look sessions:
  - the before set, on the unchanged tree;
  - a work-in-progress set, to scratch;
  - a read-only `git archive` copy of HEAD, to scratch, to get ledger numbers from the unchanged code (its 17 images are pixel-identical to the `before_` set);
  - the after set (its 17 images are pixel-identical to the work-in-progress set I inspected).
- I also made one lock-held browser launch on `about:blank` to read the WebGL renderer string.

**Console**
- The only error or warning in any state is `THREE.Object3D.add: object not an instance of THREE.Object3D. undefined`, twice per page load.
- It appears identically in the before run, the HEAD-copy run and the after run.
- The change introduces no error and no warning. There was no shader or WebGL error in either theme with the model drawn, a selection, Actual OD on, the deformation overlay and a started route.
- The OD status, the solve summary and the route ghost coordinates are identical before and after.

**Pixels**
- The drawn colours are the tokens exactly. In `light_model_nolabels.png` and `dark_model_nolabels.png`:

| Token | Exact pixels, light | Exact pixels, dark |
|---|---|---|
| `canvas.pipe` | 1447 | 1471 |
| `canvas.pipeShade` | 282 | 283 |
| `canvas.glyph` | 581 | 553 |
| `cat.1` | 988 | 988 |
| `canvas.gridMajor` | 174 | 189 |
| `canvas.gridMinor` | 1658 | 1660 |

- In the route images, `canvas.gridMajor` is 2650 / 2664 pixels.
- In the route images, `canvas.draft` is 239 light (82 of those are the shell's accent on a DOM button) / 157 dark.
- With a selection, the held selected colour is exact: 405 px `#a34400` light and 384 px `#f08c22` dark.
- The held background is exact.
- Every image has between 1040 and 1388 unique colours.

**Live switch**
- `live_2_dark.png` holds the dark tokens: 696 px `canvas.pipe`, 553 px `canvas.glyph`, 553 px `cat.1`, 189 px `canvas.gridMajor`, 1660 px `canvas.gridMinor` and 1191 px held selected colour.
- It holds 0 pixels of any light token or light held colour.
- `live_3_light_again.png` is byte-identical to `live_1_light.png`.

**Check 6, `git status --short` at the end**
```
 M projects/chirality-piping/apps/desktop/src/features/viewport/PipeViewport.tsx
 M projects/chirality-piping/apps/desktop/src/features/viewport/viewportResource.test.ts
 M projects/chirality-piping/apps/desktop/src/features/viewport/viewportResource.ts
 M projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/B-CANVAS/briefs/_INDEX.md
?? projects/chirality-piping/apps/desktop/src/features/viewport/viewportColourLiterals.test.ts
?? projects/chirality-piping/apps/desktop/src/features/viewport/viewportFigureMaterial.test.ts
?? projects/chirality-piping/apps/desktop/src/features/viewport/viewportFigureMaterial.ts
?? projects/chirality-piping/apps/desktop/src/features/viewport/viewportPalette.test.ts
?? projects/chirality-piping/apps/desktop/src/features/viewport/viewportPalette.ts
?? projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/B-CANVAS/returns/
?? projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/B-CANVAS/screenshots/
```
- The `briefs/_INDEX.md` change and the `returns/` folder are not mine. I did not touch them.
- The only ignored content is what the brief's own checks produce: `dist/`, `test-results/`, `tsconfig.tsbuildinfo`, the WASM `public/` output and the cargo targets.

## 6. Semantic changes, named as changes

1. **The routing draft leaves the selection's colour.**
   - Today: the route ghost line and its end marker draw `0xf08c22`, which is the dark selected colour, in both themes.
   - Now: both draw `canvas.draft` (`#106dce` light, `#6dadff` dark) and repaint with the theme.

2. **Element kinds are no longer told apart by colour.**
   - Today: pipes are `0x4f6f73`, nodes `0x2f6f73`, supports `0x6b7d49`, bend and rigid `0x1f6f73`, branch `0x24705a` and expansion joint `0x6f5a92`.
   - Now:
     - pipes, bends, branches and expansion joints are `canvas.pipe`;
     - nodes and rigid elements are `canvas.pipeShade`;
     - supports are `canvas.glyph`.
   - Kind is carried by shape alone, and the shapes are still today's placeholder shapes.

3. **Shading is matte and unlit.**
   - Today: `MeshStandardMaterial` sits under an ambient (0.72) and a directional (1.2) light. A drawn colour sat well below its literal and varied with orientation.
     - Pipes drew about `#34494c` to `#3b5255`.
     - That measured about 6.5 to 7.5:1 on the light ground and 2.0 to 2.3:1 on the dark one.
   - Now: an unlit figure material draws exactly the token on a facing surface and the token times `shadeRatio` at the silhouette. For a tube the silhouette is exactly `canvas.pipeShade`. The two lights are removed.

4. **The deformation overlay loses its colour and glow.**
   - Today: teal `0x0f8f85` with an emissive `0x03433f`, at opacity 0.82 (tubes) and 0.86 (markers).
   - Now: `canvas.deformGhost` with no glow, at the same opacities and transparent flag.

5. **Load arrows draw their token, and a selected arrow draws the selected colour.**
   - Today: the arrow material colour and the instance colour were the same literal, so they multiplied.
     - A force arrow drew `#b83f02` in both themes (measured, 988 px), not `0xd9822b`.
     - A selected arrow drew the selected colour times the arrow colour.
   - Now: a force arrow is exactly `cat.1` (`#cf630d` / `#d97230`), a moment arrow is `cat.2`, and a selected arrow is exactly the held selected colour.
   - Only moment against everything else is told apart, as today.

6. **Grids.**
   - Today: the reference ground draws `0xb6bfb9` centre lines and `0xdce1db` lines at opacity 0.55. The routing grid draws `0x2f6f73` and `0x9bb7b4` at 0.44.
   - Now: the ground draws `canvas.gridMajor` centre lines and `canvas.gridMinor` lines. The routing grid draws `canvas.draft` centre lines and `canvas.gridMajor` lines. Both draw at opacity 1.

7. **A theme change repaints the model.**
   - Today: a theme change repainted only the background, the gizmo and the selection colour. The model kept one set of literals in both themes, and the routing layer was in no repaint list.
   - Now: every role colour in all five layers and the gizmo repaints in place, selection is re-applied, and there is one invalidation.

8. **Gizmo colours.**
   - Today: the axes are `0x8b1e1e/0x146b32/0x1e4f9a` light and `0xff8a80/0x6ee7a1/0x8ab4ff` dark, with the badge `0xffffff` / `0x0c1114`.
   - Now: the axes are `canvas.axisX/Y/Z` and the badge is `surface.raised`.
   - In dark the badge becomes a visible `#2e3236` disc. Before, it was the colour of the ground.

Selection's behaviour, the held background, picking and every control are unchanged.

## 7. Controls

None was added, removed, enabled, disabled or renamed. No copy changed.

## 8. Where the design could not be followed, and why

- **No two-weight ground grid.**
  - `GridHelper` has two colour slots, and the role table maps today's two literals to the two grid tokens.
  - `canvas.gridMajor` therefore appears only on the two centre lines.
  - A cadence of major lines could be painted by rewriting values alone, because every line index has its own four vertices. That needs a design decision on the interval. I did not invent one.
- **No edge line (§6.2).** It is not in this slice. In light it is what the design relies on to carry the geometry.
- **Background and selection are held at today's values.**
  - Specification §12 row 14 ("every canvas colour is a token") is therefore not complete.
  - Selection is still a recolour plus the cue, not a `canvas.selection` halo (§6.6).
- **Load kinds.** Everything other than a moment takes `cat.1`, so §2.7's slots 3 to 6 are not used, as the brief directs.
- **Restraints.** They are solid cones in `canvas.glyph`, not a stroke over `canvas.glyphFill` with an arrow and a plate (§6.4).
- **Other §6 items out of scope:** real outside diameter as the resting state, fitting geometry and label plates are not in this task.
- **The role table is unchanged. One row I think is weak: `deformedShape` to `canvas.deformGhost`.**
  - §6.8 gives that token to the dashed undeformed outline while the deformed shape draws solid.
  - The product draws the opposite arrangement, with a solid model and a thin deformed overlay.
  - With this token at 0.82 opacity, the overlay the user asked to see becomes the quietest object on the canvas (numbers in section 10).
  - A token that would read without inventing a value is `canvas.edge` (7.27:1 light, 11.65:1 dark on the held ground) or `canvas.vector`. That is your decision; I did not change the row.
- **Two rows look right but read heavy or close:** `support` as a solid fill in dark, and `loadForce` beside the held selected colour (section 10).

## 9. Differences found between a frame and the specification

- **Tube shading.**
  - Both frames draw the tube as flat `canvas.pipe` with one darker band along one flank.
  - The band values are not tokens: `#959aa0` light and `#656b71` dark.
  - The light frame holds 0 pixels of `canvas.pipeShade`.
  - §6.2 asks for darkening toward the silhouette in `canvas.pipeShade`, never reading as lit.
  - The implementation follows the specification: both silhouettes darken and reach exactly `canvas.pipeShade`.
- **Ground grid.**
  - The frames draw it in both grid tokens, a two-weight grid. Light has 281 px of `canvas.gridMajor` and 767 px of `canvas.gridMinor`; dark has 78 and 258.
  - §2.8 names both tokens for the ground grid and, in what I read, states no cadence.
- **Routing.**
  - The frames draw no routing construction grid. They draw a compass at the node and a dashed `canvas.draft` centreline with a faint tube outline.
  - The product draws a construction grid, which the brief's table colours.
  - I did not read specification §4.5, so I do not say which governs.
- **Not a frame difference, but found here:**
  - In `tokens.json`, `canvas.draft` and `canvas.selection` hold the same value in both themes (`#106dce` / `#6dadff`), and the frames draw them so.
  - Row 12 asks that the draft is "never the selection's".
  - Once selection moves to `canvas.selection`, the draft and the selection will be the same colour and told apart by form only.

## 10. What reads wrong in the running product

Contrast is WCAG relative luminance.
- The held ground is `#dfe5e8` light and `#0c1114` dark.
- The held ground differs from `canvas.bg` by 1.08:1 in light and 1.11:1 in dark.

| Token | Light, on held ground (design, on `canvas.bg`) | Dark, on held ground (design, on `canvas.bg`) |
|---|---|---|
| `canvas.pipe` | 1.82:1 (1.97) | 4.44:1 (4.00) |
| `canvas.gridMajor` | 1.20:1 (1.31) | 1.45:1 (1.31) |
| `canvas.gridMinor` | 1.03:1 (1.12) | 1.25:1 (1.12) |
| `canvas.pipeShade` and `canvas.deformGhost` | 2.85:1 | 2.68:1 |
| `canvas.glyph` | 9.61:1 | 14.07:1 |
| `canvas.draft` | 4.03:1 | 8.20:1 |
| `cat.1` | 3.03:1 | 5.79:1 |
| `cat.2` | 3.71:1 | 5.19:1 |
| held selected colour | 4.88:1 | 7.67:1 |

- **Light: the figure is pale.**
  - The tubes went from a lit dark teal at about 6.5 to 7.5:1 to a grey at 1.82:1.
  - With no edge line yet, the geometry is carried by a fill below 3:1.
  - The supports (9.61:1), the arrows and a selection are now the strong things on the canvas.
- **Light: the grid is effectively invisible on the held ground.**
  - `canvas.gridMinor` is 1.03:1, and most antialiased pixels are 1.01 to 1.02.
  - It was just as faint before (1.01 to 1.02:1 measured).
  - Only the two centre lines read.
- **Dark: the figure reads better than before.**
  - The tube is 4.44:1, where it measured about 2:1 before.
  - The darker held ground lifts every token slightly above its designed contrast.
- **Dark: the grid is quieter than before.** It is 1.25:1 at full coverage, where the old grid measured about 1.38:1.
- **Dark: the supports are the loudest objects.** Solid near-white cones at 14.07:1 sit beside a 4.44:1 tube. The design intends that ink as a stroke at fixed screen size.
- **Both: the deformation overlay is the quietest object.**
  - After blending at 0.82 it is about 2.29:1 light and 2.18:1 dark.
  - It is thinner than the tube it sits beside.
  - It was a saturated teal with a glow before.
- **Both: the held selected colour and `cat.1` are neighbours.**
  - They measure 1.61:1 apart in light and 1.32:1 in dark.
  - A selected load arrow is hard to tell from an unselected one in dark, and a selected pipe shares a colour family with the arrows.
  - This ends when selection moves to `canvas.selection`. The draft then meets the token-value finding in section 9.
- **Both: fittings are hard to tell apart.** They share the tube's grey and keep the old placeholder shapes. Node spheres read as darker beads on the tube.
- **Both: a selected element's silhouette darkens by the same ratio,** so the selected colour varies toward its rim. The lit material varied it before as well.
- **Not seen in the product: `cat.2`.** The sample has no moment load, so that row is covered by unit tests only.

## 11. Ledger and per-frame work

**Ledger**
- I measured against a read-only copy of HEAD whose screenshots are pixel-identical to the before set.
- The ownership ledger's `live`, `created` and `disposed` counts are identical in all 17 states. No count changed.
- For example, the loaded model holds 17 geometries, 18 materials, 3 textures, 11 instance matrices and 11 instance colours.
- With the overlay it holds 19, 20, 3, 13 and 13.

**Renderer info**
- three's own `renderer.info.memory.textures` reads 3 where it read 4.
- r181 uploads a lookup texture for physically based materials, and no such material remains.
- That texture was never an owned resource.

**Per frame**
- Draw calls, triangles, lines and points per frame are identical in all 17 states. For example, the model alone is 12 calls and 1888 triangles, and with a route it is 19 calls and 3368 triangles.
- I changed no code in the render loop or in any pointer handler. Repaint runs only from `setThemePresentation` and `replaceLayer`.
- I did not measure frame time and ran no benchmark.

## 12. Changes outside my scope, as exact proposals (none made)

- **Pre-existing console error.**
  - In `ViewportResource.replaceLayer`, replace `layer.add(...objects);` with `if (objects.length > 0) layer.add(...objects);`.
  - The file is in my folder, but `replaceLayer` is the retained rendering foundation and this is not this task.
- **`{DESKTOP}/src/design/tokens.json`, the design lane's file.** Confirm or re-step `canvas.draft` against `canvas.selection` before the selection slice, because the two tokens hold the same value in both themes.
- **Nothing outside the write scope was needed for this task.** I did not touch `styles.css`, including whatever shell value mirrors the held background.

## 13. What I did not do

- I built none of these: the edge line, a real-OD resting state, fitting geometry, glyph strokes or plates, label styling, a `canvas.selection` halo, a `canvas.bg` background, or a major-line cadence.
- `canvas.edgeAlt` and the other exposed tokens without a row have no consumer.
- I did not run the benchmark instrument or any `*.benchmark.ts` file, a timed benchmark, the Tauri shell or a headed window.
- I did not fix the pre-existing console error.
- I did not remove other functions in `PipeViewport.tsx` that were already unreferenced at HEAD and are not in the brief's list: `isNodeDraftValid`, `isPipeDraftValid`, `raycastDraftPoint`, `unscale`, `buildExplicitNodeIntent`, `buildExplicitPipeIntent`, and the unused `registerSelectionPresentation` import.
- I did not edit any existing test, tolerance, oracle or frozen value.
- I did not commit.
- I attribute nothing to the owner beyond the records named above.

## 14. Claims

- I claim no usability, conformance or performance acceptance.
- The statements above are what I ran and measured on this host today: unit and browser test results, pixel samples, contrast arithmetic and ledger counts.
- The standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).