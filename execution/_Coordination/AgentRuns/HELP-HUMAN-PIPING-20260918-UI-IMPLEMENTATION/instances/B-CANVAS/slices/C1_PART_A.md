# Slice C1, first part (C1a): return to ROOT

From B-CANVAS (WORKING_ITEMS, Type 1, canvas lane manager; Claude Fable 5.1), 2026-09-19 (UTC). Lane branch `codex/swbpipe-b-canvas-20260918`. Placeholders as in the lane brief. This file is the durable copy of the manager's return message for the slice; it is committed together with the slice's screenshots and records.

## 1. Slice

C1, first part, as addendum 1 orders it: every canvas colour other than the scene background and the selected colour (with its cue rim) reads a design token, with live repaint in both themes. The two held families stay at today's values, in one marked block, until ROOT has decided proposal P1. `canvas.edgeAlt` is C6 and has no consumer.

## 2. Commits and diff range

| Commit | What |
|---|---|
| `cbc538777` | Returns of INV1-INSTRUMENT and C1A-PALETTE retained; proposals P1 and P2 |
| `2dcf97757d9f320502d1f140ab2d7a4d6521753c` | The slice's code: eight files under `{DESKTOP}/src/features/viewport/` |
| `9e2fe2826dfe9506d7adc558e8edad658944e9ed` | Merge of `origin/main` at `8e4c5df6ec84928ca343224e5244ccdae5771cd4` (PR #802), a merge commit, taken with no child running, as ROOT directed. No conflict. Under `apps/desktop` it brought `e2e/r2-smoke.spec.ts`, `src-tauri`, `src/App.test.tsx`, two panels and one panel test; nothing under `features/viewport`. |
| the commit that holds this file | Screenshots with their index, proposal P3, P1 with one line citation corrected, the lane's brief index, this record |

Review range for the lane's own product changes: `git diff 8e4c5df6ec84928ca343224e5244ccdae5771cd4...HEAD -- projects/chirality-piping/apps/desktop` (three dots). It shows eight files, +1144 −336, all under `src/features/viewport/`. The same diff is `git diff cbc538777 2dcf97757`.

| File under `{DESKTOP}/src/features/viewport/` | State | SHA-256 |
|---|---|---|
| `PipeViewport.tsx` | changed | `9f43616c06359481abef1d5747e78ba72835f1c47eeb3ee53494b67652bdcd47` |
| `viewportResource.ts` | changed | `40a0430e7f3103c1c274a735781c4d04076be7e5cc9f1eb3cc924ffca2439a8c` |
| `viewportResource.test.ts` | changed, 225 lines added, no existing line touched | `91bed80c3b2111d5629e6a81a270c336ef5de7a3ffc34e31db4c397f6726b0af` |
| `viewportPalette.ts` | added | `e3d1fa5bf1d48ea466293931579d0d60aded438eb437cf6f8d4889c1d89e7de8` |
| `viewportPalette.test.ts` | added | `08af140839dacea6a702d819771b6d32d2c558d910541b7824614cbbd4dc51ab` |
| `viewportFigureMaterial.ts` | added | `7fb742f07c18703158299b533ac9a369a6d2015854284cd9132ded2f9c90f17f` |
| `viewportFigureMaterial.test.ts` | added | `e21db16ee9b911cecdd572cd68ab61c9db42bf51b1fbd914f5037f674feca5c1` |
| `viewportColourLiterals.test.ts` | added | `23b0363d896c94ed92d33533265afcb0043f15a98674786565931c36d5013856` |

Six of the eight hashes equal those in the child's return. The two palette files differ by the manager's one-row change (§6, change 4).

Who did what. The TASK child C1A-PALETTE (Claude Fable 5.1, `claude-fable-5-1`) implemented the slice under the sealed brief `../briefs/C1A-PALETTE.md` (`e2c8fffe4957d8ce112a66d0a2449755cf77a499182ed51b1e59482d7d2db77b`); its return is `../returns/C1A-PALETTE_RETURN.md` (`01f6199ccb97a9e94f33fcf2324c775cbf7a0ccd0ddb47ac7f5d878e6c3108cc`). The manager read the whole diff of `viewportResource.ts`, every added line of `PipeViewport.tsx`, both new modules whole, the literal-scan test whole and the repaint tests; looked at the screenshots; changed one row of its own role table; merged `origin/main`; and ran every check below itself on the merged head.

## 3. What the slice does

- `viewportPalette.ts` imports the product's `src/design/tokens.json`, parses the 22 `canvas.*` tokens, `cat.1` to `cat.8` and `surface.raised` once into frozen per-theme tables (hash colours and both functional notations, so the two translucent tokens parse with their alpha), and holds the one role table that binds what the canvas draws to a token.
- `viewportFigureMaterial.ts` is a texture-free, light-free `ShaderMaterial` built from three's own chunks for instancing, per-instance colour and output colour space. A facing surface draws exactly its token; the silhouette darkens by linear(`canvas.pipeShade`) / linear(`canvas.pipe`) for the theme, so a tube's silhouette is exactly `canvas.pipeShade`. Every figure mesh uses it. No lit material remains, so the scene's two lights are removed.
- Live repaint: roles are kept on `userData`; `setThemePresentation` repaints all five layers and the gizmo in place and re-applies the selection; `replaceLayer` paints incoming objects for the current theme. The routing layer was in no repaint list before. Nothing is created or disposed by a repaint, and no colour object is allocated per instance.
- `PipeViewport.tsx` loses eleven unreferenced builders and helpers (the manager had confirmed zero references at the lane base before sealing the brief; the child confirmed it again).
- A source-scan test refuses any colour literal in a non-test file of the folder outside the marked held block, and pins that block to exactly the six held values.

## 4. Checks, all run by the manager at `9e2fe2826dfe9506d7adc558e8edad658944e9ed`

| # | Command | From | Exit | Result |
|---|---|---|---|---|
| 1 | `npx vitest run src/features/viewport/viewportSelection.test.ts` (picking first) | `{DESKTOP}` | 0 | 1 file, 69 tests passed |
| 2 | `npm run test:desktop` | `{WORKING_ROOT}` | 0 | 78 files, 1216 tests passed, 0 failed, 0 skipped |
| 3 | `npm run build:desktop` | `{WORKING_ROOT}` | 0 | `tsc -b` clean; built; only the existing chunk-size warning |
| 4 | `PLAYWRIGHT_WORKERS=1 sh {RUN}/tools/with_e2e_lock.sh npm run test:e2e` | `{DESKTOP}` | 0 | 394 tests: 374 passed, 20 skipped, 0 failed, 0 flaky; 9.1 min (01:31:07Z to 01:42:57Z, including a wait for the lock). All 20 skips are in `e2e/ui-foundation/full-cohort-controller.spec.ts`, the expected skip on absent `D70_WRITER_*` inputs. |
| 5 | `PLAYWRIGHT_WORKERS=1 sh {RUN}/tools/with_e2e_lock.sh npm run test:e2e:dist` | `{DESKTOP}` | 0 | 53 passed, 0 skipped, 0 failed; 2.8 min |
| 6 | `python3 tools/validation/validate_claims_language.py` | `{REPO_ROOT}` | 0 | VALID, 323 files scanned |
| 7 | `python3 tools/practitioner_harness/harness.py self-check` (registered, always) | `{REPO_ROOT}` | 0 | No finding names a file of this lane |
| 8 | `python3 -m pytest -q tools/practitioner_harness` (registered for `execution/**`) | `{REPO_ROOT}` | 0 | 379 passed |
| 9 | The look: the child's throwaway script, outside the repository, through the lock, dev server on 5184 | `{DESKTOP}` | 0 | 17 states in both themes; port free and lock absent afterwards |
| 10 | `npx vitest run src/features/viewport/viewportSelection.test.ts` (picking last) | `{DESKTOP}` | 0 | 69 passed |

After each browser run the lock directory was absent and nothing listened on 5174, 5175 or 5184. No timed benchmark was announced. The child ran the same checks on its own tree before the manager's change and the merge; its results are in its return, §5, and agree.

Per-frame work. The slice adds none: repaint runs only on a theme change and a layer replacement, and the render loop and pointer handlers are untouched. The child measured draw calls, triangles, lines and points per frame and the ownership ledger's `live`, `created` and `disposed` counts in all 17 states against a read-only copy of the lane base: identical. The physically based fragment shader is replaced by a shorter unlit one. Frame time was not measured and no benchmark file was run.

## 5. What ROOT asked the manager to check

- **Scene background and selected colour.** Unchanged in either theme. The six values sit in one block between `// HELD-COLOURS:BEGIN` and `// HELD-COLOURS:END` in `viewportResource.ts`: `0xdfe5e8`, `0x0c1114` (background; also `GIZMO_THEME_PALETTES.*.canvas`), `0xa34400`, `0xf08c22` (selected), `0xffffff`, `0x0c1114` (cue rim). Pixel samples: the held background is exact; with a selection, 405 px of exactly `#a34400` in light and 384 px of exactly `#f08c22` in dark. `styles.css` is untouched, so `--ui-canvas` and `--ui-viewport-selection-geometry` still mirror them.
- **Bound files.** `viewportSelectionPresentation.ts` is `00384d2831797e5cba8acff21e82e36f21a853c398e8c2a801ccfed0f3a55931` and `viewportSelection.ts` is `fdf3eaa49b9685b932bce404421086c45a82fa22e5c8abc3f8146fc09cb846d5`, both as at the lane base. Nothing under `e2e/` is changed by the lane.
- **One effect on what the first profile sees, stated so that the reviewer need not find it.** The selected element's body was recoloured through a lit material and drew darker than the selected colour; it now draws exactly the selected colour on facing surfaces. The instrument's paired witness reads the 11 px diamond, which is unchanged, so its result is unaffected; only `netColorGrowthDiagnosticOnly`, a diagnostic, would grow. In dark, a force arrow (`cat.1`, `#d97230`) now lies within ±48 per channel of the held selected colour, where the old arrow (`#b83f02` drawn) did not; in light both old and new do. Arrows do not change between a before and an after capture, so they cannot newly qualify.

## 6. Semantic changes, named

1. **Element kinds are no longer told apart by colour.** Pipes, bends, branches and expansion joints take `canvas.pipe`; nodes and rigid elements `canvas.pipeShade`; supports `canvas.glyph`. Kind is carried by shape, and the shapes are still today's placeholders.
2. **The routing draft leaves the selection's colour.** The route ghost line and its marker drew `0xf08c22`, the dark selected colour, in both themes; they draw `canvas.draft` and repaint with the theme (specification §12 row 12, G-31).
3. **Shading is matte and unlit.** A drawn colour used to sit well below its literal and vary with orientation; it is now the token on a facing surface and the token times the shade ratio at the silhouette.
4. **The deformed overlay draws in neutral ink with no glow.** It was teal `0x0f8f85` with an emissive term, about 3.1:1 light and 4.8:1 dark on the ground. The brief's role table sent it to `canvas.deformGhost`; the child measured that at the overlay's 0.82 opacity as 2.29:1 light and 2.18:1 dark and said so. The manager changed the row to `canvas.vector`: 4.70:1 light and 8.10:1 dark on the held ground, 4.95:1 and 7.43:1 on `canvas.bg`. The design gives `canvas.deformGhost` to a dashed undeformed outline that this product does not draw (design system §6.8); the deformation presentation itself is outside the tranche. Opacities and the transparent pass are as they were.
5. **Load arrows draw their token, and a selected arrow draws the selected colour.** Material colour and instance colour used to multiply, so a force arrow drew `#b83f02`, not its literal. A force arrow is now exactly `cat.1` and a moment arrow `cat.2`. Only moment against everything else is told apart, as before.
6. **Grids draw the grid tokens at full opacity.** Ground: `canvas.gridMajor` centre lines, `canvas.gridMinor` lines. Routing grid: `canvas.draft` centre lines, `canvas.gridMajor` lines. The materials stay in the transparent pass, so draw order is unchanged.
7. **A theme change repaints the model.** Before, only the background, the gizmo and the selection colour changed with the theme.
8. **The gizmo takes `canvas.axisX/Y/Z` on a `surface.raised` badge.** In dark the badge becomes a visible `#2e3236` disc. The existing gizmo contrast test passes unedited (axes on the badge 4.57:1 to 5.86:1; on the held ground 3.59:1 to 8.62:1).

Selection's behaviour, picking, the held colours and every control are unchanged.

## 7. Controls

None added, removed, enabled, disabled or renamed. No copy changed. No gap is opened or closed by this slice.

## 8. Screenshots

`../screenshots/C1A/`: 34 PNG files and `README.md` (`9520be667fa2a2a78a76e58656c05a1d91ddf2f24cae90695750ce4c1d4ad278`), which lists every file with its SHA-256 and says which six the manager re-took on the merged head after the change of §6.4. Before and after in both themes for: model with labels, model without labels, a selected pipe at actual outside diameter and at schematic radius, the deformed overlay with a pipe and with a support selected, a started route, and one page switched light to dark to light without a reload (the third image is byte-identical to the first). The only console message in any state, before and after, is the one in §9.4.

## 9. Brought back to ROOT

1. **Proposal P1**, the instrument's second profile: `../proposals/P1_SECOND_PROFILE.md` (`cd4650cb4a8c5d65b80bed8555b2349c1aa222938bb987aa09d1e696d7fd4626`). Eleven asks. C1b and C2 wait on ASK-1 to ASK-3 and ASK-11.
2. **Proposal P2**, a slice for the edge line: `../proposals/P2_EDGE_LINE_SLICE.md` (`abb5d7dbc681684bc27cc713ddf977ca05e2f041e1cbc20bc85811bc45ef0a61`). Five asks.
3. **Change request P3** for the shell lane's `styles.css`: `../proposals/P3_STYLESHEET_REQUEST_C1.md` (`0ecab4d2a6a04c9199ffd96fbbe76ba9e1b3991a67b90ebfe09fea5a19a14318`): `.viewport-shell`, `.viewport-scale-bar`, `.viewport-fallback`. The two mirror variables follow with C1b.
4. **A console error that predates the lane**, twice per page load: `THREE.Object3D.add: object not an instance of THREE.Object3D. undefined`. `ViewportResource.replaceLayer` calls `layer.add(...objects)` with an empty list. The repair is one guard, `if (objects.length > 0) layer.add(...objects);`. `replaceLayer` is the retained foundation, so the manager proposes it and has not made it.
5. **`canvas.draft` and `canvas.selection` hold the same value in both themes** (`#106dce`, `#6dadff`) in the product token file, and the frames draw them so. Once C1b moves selection to its token, the routing draft and a selection are the same colour, told apart by form only. Specification §12 row 12 says the draft has "its own token and form, never the selection's". The token file is not this lane's; the design owner or ROOT confirms or re-steps it before C2. Proposal P1 §4.4 already treats it as a precondition of the cue.
6. **No two-weight ground grid.** Both frames draw the ground grid in both grid tokens. `GridHelper` has two colour slots, used as today (centre lines, other lines), so `canvas.gridMajor` appears on two lines only. A cadence of major lines can be painted by values alone, but the design states no interval that the child or the manager could find. It needs a number from the design.
7. **Frame against specification.** Both frames draw the tube flat with one darker band along one flank, in values that are not tokens (`#959aa0`, `#656b71`); design system §6.2 asks for darkening toward the silhouette in `canvas.pipeShade`, "never enough to read as lit". The slice follows the specification.
8. **Host routing.** C1A-PALETTE's completion notice reached the manager directly, because the manager was mid-turn when the child finished; INV1's had gone to ROOT while the manager was stopped. The manager therefore did not wait for a relay. It extracted the child's final message byte-for-byte from the host's task record by script (printing only size and hash; the transcript was not read) and retained it. If ROOT extracts it too and the hashes differ, ROOT's file governs.

## 10. What reads wrong in the running product after this slice

- **Light: the figure is pale, and will stay so until the edge line exists.** Tubes went from a lit teal at about 6.5 to 7.5:1 on the ground to `canvas.pipe` at 1.82:1 on the held ground (1.97:1 on `canvas.bg`, by design, with the edge line carrying the geometry at 7.88:1). This is the gate theme. Proposal P2 ASK-5 asks ROOT whether the lane may reach `main` in this state.
- **Light: the ground grid is nearly invisible on the held ground** (`canvas.gridMinor` 1.03:1, `canvas.gridMajor` 1.20:1; designed 1.12:1 and 1.31:1 on `canvas.bg`). It was as faint before (1.01 to 1.02:1 measured). It improves a little with C1b.
- **Dark: supports are the loudest objects** (solid cones in `canvas.glyph`, 14.07:1, beside a 4.44:1 tube). The design intends that ink as a stroke at fixed screen size; fixed-size glyphs are outside the tranche.
- **Both: the held selected colour and `cat.1` are neighbours** (1.61:1 apart in light, 1.32:1 in dark), so a selected load arrow is hard to tell from an unselected one. This ends with C1b.
- **Both: fittings share the tube's grey and keep their placeholder shapes;** node spheres read as darker beads.
- `cat.2` was not seen in the product: the sample has no moment load. It is covered by unit tests only.

## 11. Next

C1b, on ROOT's decision of P1 (ASK-1 to ASK-3 and ASK-11): the scene background to `canvas.bg`, the selected colour and the cue's rim to `canvas.selection` and its pair, the held block and its test removed, and the two-line `styles.css` change returned. Then C1E or C2 in the order ROOT chooses under P2 ASK-2. If ROOT would rather not decide P1 yet, the lane can take C3 (isolate as dimming, G-30) first: it changes neither held colour and leaves the selection presentation as it is.

The manager claims no usability, conformance or performance acceptance. The statements above are what was run and measured on this host today.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
