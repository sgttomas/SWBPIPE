# C2-HALO return, retained (model that ran: Claude Fable 5.1, claude-fable-5-1, by its own statement)

Retained by B-CANVAS (ROOT's script). What follows is the agent's final message as the host stored it, with the machine's repository path replaced by `{REPO_ROOT}`; nothing else is changed. The message exactly as the host stored it has SHA-256 `c31b3481c8c736a9ad67c4bffefa637748711f485f8c57d2b3948c5ddda06c1d` (247 lines), by this script.

---

# C2-HALO return: selection and hover are halos over the element's own colour

Form (b) is the working form in the tree: the halo is drawn over nearer geometry and masked by depth. Every check in the brief passes. The stencil form (c) exists only as a patch and a build copy outside the repository. I ran no timed series: whether D-72's limits hold is unmeasured and is ROOT's to decide.

**Who I am.** Claude Fable 5.1 (`claude-fable-5-1`), TASK (Type 2), child of B-CANVAS.
- I worked alone, launched no agent and ran no state-changing git command.
- I wrote files through the shell only, and wrote nothing under `e2e/**`.
- The brief's SHA-256 (`0e6ae041…9b947`) and the base build's `index.html` hash (`fc785c1b…8180c`) both verified before I started.
- I received one message from my parent (the session-limit resume note with three review facts) and acted on it within the brief.

**What I read.**
- Whole: the brief; P1 §4; `viewportFigureMaterial.ts`; `viewportResource.ts`; `viewportSelectionPresentation.ts`; `selectionPresentationBinding.ts`; the C1E-EDGE return; the lock script; the builders and the selection and hover wiring in `PipeViewport.tsx`.
- In part: addendum 2 §1 to §2 (ASK-4, ASK-5, ASK-6); design system §1.3, §2.8, the two rows of §2.9, §6.2 to §6.7; the handoff's G-31 rows; `viewportResource.test.ts`.
- Not opened: specification §4.1; the two frames. I can report no difference between a frame and the specification.

## Mechanism as built (`viewportHalo.ts`, new)

`ViewportHaloPresentation` owns one `Group`, added to the scene after the five layers. For each base geometry ("shape") that has a haloed element, and for each kind of halo (selection, hover), it keeps one pair of `InstancedMesh`: a mask and a shell.

**Instances.**
- A pair carries only the haloed instances' matrices.
- The matrices are read from `userData.instanceBaseMatrices`, the base matrix, never the zeroed matrix of a hidden instance.
- A non-identity `matrixWorld` is premultiplied.
- One pair serves every spatial chunk of a shape, so there are two draws per shape and kind however many elements are haloed.
- Halo meshes are not frustum-culled, refuse raycasts, and have `matrixAutoUpdate` off.
- Capacity grows in powers of two (1, 2, 4, …) and never shrinks while the shape lives.

**Draw order.** Everything is in the transparent list with `NoBlending`, after the figure and any transparent overlay, and before the first profile's cue at 10 000 and 10 001.
- Masks draw at 9 000, the hover shell at 9 001, the selection shell at 9 002.

**Mask.**
- It uses the element's own geometry and three's `begin_vertex` and `project_vertex` chunks, so its coverage equals the figure's.
- `colorWrite` is false, `depthFunc` is `AlwaysDepth`, `depthWrite` is true, and `gl_Position.z = -w × 0.9999`, a window depth of 5e-5.
- The depth test stays enabled because WebGL writes no depth with the test off.

**Shell.**
- `depthTest` is on with `LessEqual`, `depthWrite` is off, and `z = -w × 0.9996`, a window depth of 2e-4.
- It therefore draws everywhere except inside the outline of any haloed element, and it shows over nearer geometry.
- The separation between mask and shell depth is thousands of steps at 24 bits and about ten at 16 bits.

**Two shell forms.**
- A `SphereGeometry` is grown along its normals. The vertex is projected as it stands and again moved 1 % of its eye distance along the view-space normal. The screen difference gives the direction, and the vertex moves `haloWidth` device px along it.
- Every other shape uses an edge-quad geometry built from `EdgesGeometry(source, 1°)`, which holds crease and border edges only.
  - Each edge is four vertices with `position` (own end), `haloOther` (the other end) and `haloSide` (side ±1, end ±1).
  - Both ends go to view space, and an end behind the near plane is brought to it along the edge.
  - The vertex moves `haloWidth` away from the other end and `haloWidth` to its side.
  - The position is written already divided, with `w = 1`, so the varying is affine in the screen.
  - The fragment shader discards what lies beyond a round cap of the width.
  - The mask hides the inner half of every edge and every edge that is not on the silhouette. What remains is the silhouette grown by a disc of the width, with no rule per shape and no crack.

**Colour and width.**
- The `haloColour` uniform is set with `Color.setHex(token)`.
- Output goes through `colorspace_fragment`; materials are unlit, `toneMapped: false`, and carry no texture.
- Width is 2 × pixel ratio for selection and 1 × pixel ratio for hover.
- One width rule, as the review asked: the ratio and the drawing-buffer size are read from the renderer in `resize()`, where it is sized, and never per frame.
- Pictures measure the exact token value: `#106dce`, `#6dadff`, `#5d94da`, `#5788c7`.

**Resource wiring.**
- `setSelectionPresentation`, `setVisibilityPresentation` and `replaceLayer` call `halo.sync([modelLayer], selected, hidden, hovered)`.
- The new `setHoverPresentation(key)` returns at once for the same key. Otherwise it costs one scan, one instance write and one invalidation.
- `setThemePresentation` repaints two colour uniforms in place.
- `dispose()` disposes the halo first.
- `PipeViewport.tsx` gains one effect that passes `hoveredEntityKey`.
- `SelectionPresentationBinding` and its contract are untouched. The renderer's construction is unchanged.

**Removed.**
- The selected branch of `applyInstancedSelection`, which is now `paintInstancedBase`.
- The material recolour of non-instanced objects. Base-colour and base-emissive repaint remain.

**What gets a halo.**
- Halos follow the model layer only: pipes in both modes, nodes, supports, and the four placeholder kinds.
- Load arrows are not selectable today. They carry their owner's key and were recoloured with it; now they are neither recoloured nor haloed. The same holds for the deformed overlay.

**What draws after the halo.**
- The cue has `depthTest: false`.
- The gizmo calls `clearDepth()` in its scissor before it draws (verified in `renderGizmo`).
- Nothing else with a depth test draws after the halo.

## Options and recommendation

| | (a) depth-tested shell | (b) built: masked by depth | (c) stencil (patch only) |
|---|---|---|---|
| Draws per shape and kind of halo | 1 | 2 | 2 |
| Vertices per haloed pipe | needs a closed grown shell with per-shape corner rules; the edge shell cannot be used unmasked (~64) | 64 mask + 120 shell | the same |
| Memory | 1 instance buffer | 2 (2.1 MB with all 10,000 pipes) | the same, plus 8 stencil bits per sample |
| Ledger per first shape | +1 instance matrix, +1 material | +2 instance matrices, +1 geometry, +2 materials | the same |
| When the element is occluded | halo hidden; a centreline-mode pipe inside its node spheres shows almost none | halo shows over the nearer geometry | the same picture |
| Fragility | per-shape miter heuristics; z-fighting behind the element | relies on the standard depth convention (no reversed or logarithmic depth); leaves near depth in masked pixels for anything drawn later with a depth test | changes renderer construction; leaves depth alone |
| MSAA edges | not built; no claim | per-sample; element edge antialiased against the halo | the same |

All eighteen fixture pictures are pixel-equal between (b) and (c).

I recommend **(b)**. It gives the same picture with no construction change. Its one liability is the near depth left in masked pixels, which matters only to something drawn after render order 9 002 with a depth test. Nothing does that today.

Choose (c) if a later slice must draw depth-tested geometry after the halo. C3's dimming does not need it: it draws before the halo.

Under both forms the halos are the outline of the union of haloed elements. Where two selected elements overlap on screen, no halo line crosses the other's body.

## Files and SHA-256

All under `{DESKTOP}/src/features/viewport/`:

| File | State | SHA-256 |
|---|---|---|
| `viewportHalo.ts` | new | `2b34fda11eef060f17c876b94c1f20a59320ccc9a13f541528439030ae2863c2` |
| `viewportHalo.test.ts` | new, 24 tests | `035c19eb5b94c16f328635a0e9d7ebeb5baaa539f3d17fb4eba2d54921891279` |
| `viewportResource.ts` | modified | `18bd799d1f93138fb2bc2456a1300e68e2febfc288dffc85b5ae47ae4e0b5a4f` |
| `PipeViewport.tsx` | modified | `d70bc5be78074a8f2de7c81372dd9dcf756d03e155e270f6e93c52f2a6174a12` |
| `viewportResource.test.ts` | modified | `5fb9649923fe95bccf82be194ca5dc47af983e994964501fc48263030d438498` |
| `viewportFigureEdge.test.ts` | modified | `a442ff1060b39e41c8adf2165eb40a3e5a67ecb74050471e1bc6c1ee008c3055` |

Evidence:
- `{LANE}/probes/C2/SUMMARY.md`: `7d1e3c1f90febc3751133cd9258d0cb78b878cf21987f41ffe99095a6f7d82d3`.
- `{LANE}/probes/C2/stencil_candidate.patch`: `cfa85823bbf26fd47cc556fc7add973e7b3ddea24f8425b0273d618d8094ca42`. It is a diff against the working form, because `viewportHalo.ts` is untracked and `git diff` alone could not express it. `git apply --check` passes.
- `{LANE}/screenshots/C2/README.md`: `d7724e1e6596741359bd654aafe6946a1852722c73cb0a3f41bfeb2ddfc49159`, with 71 PNGs each hashed.

`git status --short` also shows `LANE_LOG.md` modified; that change is not mine.

## Tests moved

Each assertion of the display recolour moved to the new presentation. No test of behaviour was deleted or loosened.

1. `viewportResource.test.ts`, "updates selection and theme presentation…", renamed "keeps a selected mesh at its own colour…".
   - Old: selected material colour `toBe(SELECTED_DARK)`.
   - New: `toBe(0x24705a)` and `not.toBe(SELECTED_DARK)`.
2. "updates instanced selection colors…", renamed "keeps a selected instance at its base colour…".
   - Old: instance 1 `toBe(SELECTED_DARK)`.
   - New: `toBe(0x2f6f73)`.
3. "reapplies hidden, selected, theme…".
   - Old: `toBe(SELECTED_LIGHT)`.
   - New: `toBe(0x4f6f73)`.
4. "keeps the selected colour on a selected instance across a repaint…", renamed "repaints a selected instance with the new base…".
   - Old: three assertions of `SELECTED_LIGHT`, `SELECTED_DARK`, `SELECTED_LIGHT`.
   - New: the pipe role colour of each theme, plus an assertion that the role colour is not the selected colour.
5. "repaints every role colour in all five layers…".
   - Old: `instanceHex(pipes, 0)` equals `SELECTED_DARK`, then `SELECTED_LIGHT`.
   - New: the pipe role colour in dark, then in light.
6. `viewportFigureEdge.test.ts`, "keeps the line in canvas.edge over a selected element's held colour", renamed "…on a selected element, which keeps its own colour".
   - Old: `not.toBe(role colour)`.
   - New: `toBe(role colour)`.

## Checks (all through the lock)

| Check | Exit | Result |
|---|---|---|
| Picking test before any edit | 0 | 69 passed |
| `npx tsc -b` | 0 | clean |
| `npx vitest run src/features/viewport` | 0 | 12 files, 239 tests |
| `npm run test:desktop` | 0 | 82 files, 1297 passed |
| `npm run build:desktop` | 0 | hash below |
| `test:e2e` (`PLAYWRIGHT_WORKERS=1`) | 0 | 374 passed, 20 skipped, 0 failed, 8.7 min |
| `test:e2e:dist` | 0 | 53 passed, 2.8 min; its build equals the working copy's hash |
| Picking test last | 0 | 69 passed |
| `validate_claims_language.py` | 0 | 338 files, valid |
| The two byte-identical files | n/a | `00384d28…55931` and `fdf3eaa4…846d5`, equal to the brief's |

No e2e test that reads pixels, selection feedback or frozen geometry failed.

Tree state:
- Nothing under `e2e/**` is touched.
- No switch is present: a scan of the new file for storage, URL or global reads found none.
- No stencil form is in the tree; the two patched files were restored and hash-checked.
- No absolute path is in any file I authored.
- Port 5186 is free and the lock is released.
- The console held zero entries in every look, in both builds.

**Build copies.**
- `dist_c2_working/index.html`: `e9aca49e81b340ff09d61d8b1628d544d966cfb6db305b3b357b1d6b7e028db3`.
- `dist_c2_stencil/index.html`: `3c53bf6412961bdaf8fbeefc8fab643677f7218fa08d16ed72e6ff74b82f2922`.

**Ledger and update cost.**
- The exact ledger counts are in `SUMMARY.md` and are unit-tested.
- A scene with no halo owns nothing of the halo: its ledger equals a resource with no halo object.
- Updating 10,000 selected keys out of 20,001 elements took a median of 1.02 ms and a worst of 1.11 ms. This was observed in jsdom, not a browser.

## Semantic changes, named as changes

1. **A selected element keeps its own colour and edge line and gains a 2 px outline outside its silhouette.** Before, its instance or material colour was overwritten with `canvas.selection`.
2. **Hover gains a canvas cue.** Before, hover showed only as a label. Now an unselected, visible hovered element has a 1 px `canvas.hover` outline. This includes hover arriving from a label button. A selected or hidden element has none.
3. **The halo shows through nearer geometry.** A selected pipe inside its node spheres is outlined over them. Before, the recoloured tube was hidden there.
4. **Overlapping selected elements are outlined as their union.** Where two selected or hovered elements overlap on screen, no halo line crosses the other's body.
5. **Load arrows and the deformed overlay no longer change colour when their owner is selected.** They get no halo.
6. **A hidden selected element has no halo.** The first profile's cue already behaved so.

**Contrast not yet checked:** none; no control was touched.

**Stylesheet request (not made).** In `styles.css`, add to the active state of the label plate, keeping whatever border width the rule already has:

```css
.viewport-select-target.active { border-color: var(--canvas-selection); }
```

## Appearance left for the closing pass (nothing tuned)

**Contrast.**
- Halo on a tube is 2.21:1 in light and 1.85:1 in dark.
- On the edge line it is 1.81:1 and 1.42:1.
- On the ground it is 4.36:1 and 7.39:1.
- Hover on the ground is 2.67:1 in light and 4.69:1 in dark.
- Hover against selection is 1.64:1 and 1.58:1, so the two read apart mainly by width.

**Coverage at ratio 1.** A 2 px diagonal halo has one to two pixels within ±48 of the token (median 1.41 CSS px); the rest is antialiased. At ratio 2 the median is 2.0 CSS px.

**Beside the diamond cue.**
- The first profile's cue also draws a thin selection-coloured span line along a selected pipe's axis. The cue is stronger than the halo.
- With everything selected, the 20,000 diamonds bury the figure and the halo reads only as the mass's outer outline. See `*_all_selected_*`.
- At the fitted camera on the sample, the diamonds crowd the short halos.

**Rounded parts.** The rounded caps come from `discard`, so their curved parts are not multisampled. The radius is 2 device px at ratio 1.

**Hover beside a selection.** Hovering an element beside a selected one hides the selected halo where it crossed the hovered element's body.

## What the second profile should know

- The halo spans 0 to 2 CSS px outside the drawn outline at both ratios, exact in width. Only its outer CSS pixel lies in P1 §4.3's 1 to 3 px band.
  - At ratio 1 that is about one device pixel, part of it antialiased. The ±48 qualifier may see few pixels on thin or diagonal outlines.
  - At ratio 2 it is about two device pixels.
- The outline is the drawn polygon's, a 10-gon prism and a 12 × 8 sphere, not the ideal circle's.
- Halo pixels appear over nearer geometry, so an occluded band pixel can qualify under (b) and under (c).
- A neighbour that is also selected removes band pixels inside its own outline.
- The cue still draws, so Selection-coloured pixels also appear at the winner's centre and along a selected pipe's axis.

## What C3's dimming will meet

- The halo draws at 9 000 to 9 002 in the transparent list.
- A dimmed figure needs `transparent` together with opacity, and should sort before 9 000. The default render order of 0 does that.
- The mask takes the element's full outline whatever its opacity.
- A dimmed element can be hovered, and then gets a full-strength hover halo.

## Outside my scope, and small limits

- A rebuilt `ViewportResource` starts with no hovered key until the hovered element next changes. This affects hover only.
- The mask and shell override depth, so geometry between the eye plane and the near plane (0.1) would still be haloed.
- There is no chunk culling of halos. With everything selected, a close view still submits every haloed instance. Measure that case.
- `figureEdgeWidth` remains captured once at construction while the halo reads the ratio in `resize()`. Both read the same renderer ratio, which is set once.

## What I did not do

- Any timed series or `resources` run.
- Any casing.
- Option (a) as a build.
- A read of the specification or the frames.
- Any edit to tokens, the stylesheet, `e2e/**`, `uiDiagnostics.ts`, `modelIndex.ts` or `{LANE}/tools/**`.

I claim no usability, conformance or performance acceptance.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
