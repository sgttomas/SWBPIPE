# Retained return — C2-REVIEW (independent code review of canvas slice C2), 2026-09-19

Retained by ROOT. What follows is the agent's final message as the host stored it, with the machine's repository path replaced by `{REPO_ROOT}`. One further kind of substitution is made and nothing else is changed: the agent quoted path patterns it searched for, and the repository's path-anchor validator reads such a literal as a machine path wherever it appears, so ROOT writes the pattern it quoted as the user home pattern; the pattern it quoted as the home pattern. The message exactly as the host stored it has SHA-256 `f4dc86027b300cb5e0ff19205e7e5b332c6ce8ef8700e37504ff198c64d9e313` (116 lines), by this script.

---

# C2-REVIEW return

**Verdict: PASS.** No actionable finding. I found no defect, no picking change, no renderer-construction change, no weakened test and no unnamed behaviour change. Six residual risks and unchecked areas are listed at the end; none blocks merge.

I am **Claude Opus 5 (1M context)** (`claude-opus-5[1m]`), running as TASK (Type 2), read-only, in the worktree I was started in. I delegated nothing.

## Provenance

- Sealed brief hash read: `a8bcb95d09678071dacd57c1a7594798a8982db277d08a70b3b2db1e2d3a14da` — matches the seal. Read whole.
- Skill body read: `.agents/skills/software-code-review/SKILL.md`, SHA-256 `06c27b1be5cfbd9e638570918a8f837d8439c8073c40d3ef708e53874f95570a` — matches the seal.
- `git rev-parse HEAD` = `badd2469d22ce7ab2013984605e98582114199e3`, branch `codex/swbpipe-c2-20260919`. Worktree clean; I ran no state-changing git command and modified no file.
- **Product tree equals the candidate.** `git diff 0758e91e3 HEAD -- projects/chirality-piping/apps/desktop` is empty; the only difference between HEAD and the candidate is ROOT's records-only commit (four files: the lane `GRAPH.json`, `INDEX.md`, my brief, the retained `C2_RETURN.md`).
- `git merge-base origin/main HEAD` = `d20eb12945a…` = `origin/main`, so `origin/main...HEAD` is the candidate's own range.
- Range under `apps/desktop`: six files, +1260 −36, all under `src/features/viewport/`. `PipeViewport.tsx` +6 −0; `viewportFigureEdge.test.ts` +4 −2; `viewportResource.ts` +68 −22; `viewportResource.test.ts` +16 −12 (the brief's "+28" is the diffstat's combined column); `viewportHalo.ts` +577; `viewportHalo.test.ts` +589.

## Checks I ran

| Command (from the desktop app) | Result |
|---|---|
| `npx vitest run src/features/viewport/viewportSelection.test.ts` (first, as ordered) | 69 passed |
| `npx vitest run src/features/viewport` | 12 files, 239 passed |
| `npx tsc -p tsconfig.json --noEmit` | exit 0, no output |
| `node --check` on the lane's probe tool | OK |

No Playwright, sweep, benchmark, probe or dev server.

## 1. Picking is untouched — confirmed

- `viewportSelection.ts` (`fdf3eaa49b96…`) and `viewportSelectionPresentation.ts` (`00384d283179…`) are **byte-identical** to `origin/main` by SHA-256 of `git show` on both sides.
- 69 picking tests pass, unchanged in count.
- Every halo mesh sets `raycast = ignoreRaycast`, carries no `selectionEntityKey`, no `instanceEntityKeys` and no `entityRef`; `viewportHalo.test.ts:519` raycasts the halo group on the same matrix as a hit figure and gets `[]`, and asserts `setPickables` / `setPointPrimitives` are never called and both internals stay empty.
- **Bounds and fit are untouched.** `fitViewportCamera` takes `renderTransform.localBounds` (model-derived), not a scene box; there is no `Box3.setFromObject` anywhere in `features/`. The halo group is never passed to `applyPalettePresentation`, `applyVisibilityPresentation`, `applySelectionPresentation` or `setPickables`. `applyThemePresentation` only sets `scene.background`; the only `scene.traverse` in the file is `applyGizmoThemePresentation`, which is given the gizmo scene.

## 2. The instrument — confirmed

- `git diff --name-status origin/main...HEAD -- apps/desktop/e2e` is empty. Nothing under `e2e/**` changed.
- No tolerance, oracle, limit or frozen value anywhere in the range (no `styles.css`, no `uiDiagnostics.ts`, no file outside `src/features/viewport/` in the product).
- `viewportSelectionPresentation.ts` is byte-identical, so the first profile's diamond cue is drawn exactly as before, at `renderOrder` 10 000 / 10 001, `transparent: true, depthTest: false, depthWrite: false`.

## 3. Rendering foundation retained — confirmed

- **Renderer construction is untouched.** `new THREE.WebGLRenderer({ antialias: true })` and `setPixelRatio(Math.min(window.devicePixelRatio, 2))` are outside every diff hunk. No `stencil`, no context attribute, no render target, no post-processing. This meets the owner's rule for form (b) exactly.
- Persistent renderer, instancing and chunking, the invalidation scheduler, the typed model index and `render()` itself are all outside the diff.
- **Ledger accounting balances by construction**, and I traced every path: `ensurePair` reports `+{instanceMatrices: 2}`, `disposePair` reports `−{instanceMatrices: 2}` on every exit (capacity growth, retire, dispose); `edgeGeometry` reports `+1` and both `retire` and `dispose` report the matching `−`; mask and shell materials report `+1` each and `dispose` reports `shellMaterials.size + (maskMaterial ? 1 : 0)`. The halo shares the figure's geometry for the mask (and for a sphere's shell) and never disposes it — `InstancedMesh.dispose()` frees only the instance buffers. `ViewportResource.dispose()` calls `halo.dispose()` **before** `ownership.disposeObjects(this.scene.children)`, and `halo.dispose()` does `group.removeFromParent()`, so the shared figure geometry is counted and disposed exactly once by the layer walk. Tests pin all of this (`viewportHalo.test.ts:377, 414, 455`, including `created − disposed === live` over every ledger key).
- **With nothing selected or hovered the scene draws what it drew before.** Pairs are created lazily in `ensurePair`, which is reached only when `counts` is non-empty; with an empty selection and no hover the halo group has no children at all (pinned at `:330`, where a resource with the halo and one with `halo = null` produce equal ledger snapshots across layer replacement, selection, hover, theme and visibility). After a selection is cleared the pairs remain but `fill` sets `count = 0` and `visible = false`, and three's `projectObject` returns early on an invisible object, so no draw call is issued. This is consistent with the manager's 359-call base.

## 4. The depth-mask mechanism (b) — read as three.js and as GLSL

- **Pass order and depth arithmetic are correct.** Mask at `renderOrder` 9 000, hover shell 9 001, selection shell 9 002; all three `transparent: true` with `NoBlending`, which is what puts them after every opaque draw (three renders the whole opaque list before the transparent one, and `renderOrder` only orders within a list). The mask writes `gl_Position.z = −w·0.9999` → window depth 5e-5; the shell writes `−0.9996` → 2e-4. The shell's `LessEqualDepth` therefore fails inside the mask and passes everywhere else, including over nearer geometry. The mask is `colorWrite: false`, `depthFunc: AlwaysDepth`, `depthWrite: true` — the comment is right that turning the test off would also stop the write.
- **Nothing the product draws later with a depth test can be affected.** I enumerated every later draw. Within `features/`, the only `renderOrder` values above 3 are the cue's 10 000 / 10 001, and the cue has `depthTest: false`. The gizmo is a second `renderer.render` into its own scene, preceded by `setScissor(...)` + `clearDepth()` (which respects the scissor box) with `autoClear = false`, so it never meets the mask's depth. Labels and overlays are DOM, updated by `labelUpdater` after the frame. The depth buffer is cleared at the head of the next frame (`autoClear` is restored to its previous value). No opaque object can be affected at all, because opaque always precedes transparent.
- **Mask coverage equals the figure's, sample for sample.** `MASK_VERTEX` is `begin_vertex` + `project_vertex`, the same two chunks the figure's vertex shader uses, and `FigureMaterial` displaces no vertex and uses no `discard` or `alphaTest`. Both draw `FrontSide`. `USE_INSTANCING` is injected by three for an `InstancedMesh`, and `project_vertex` in r181 applies `instanceMatrix`.
- **Instanced and chunked meshes under every define.** The mask inherits instancing from three; `EDGE_SHELL_VERTEX` does its own `modelViewMatrix * instanceMatrix` under `#ifdef USE_INSTANCING`; `NORMAL_SHELL_VERTEX` uses `beginnormal_vertex` + `defaultnormal_vertex`, which in r181 divides out the instance matrix's scale correctly. I confirmed all five chunks (`begin_vertex`, `project_vertex`, `beginnormal_vertex`, `defaultnormal_vertex`, `colorspace_fragment`) and `BufferAttribute.clearUpdateRanges` / `addUpdateRange` exist in the installed three 0.181.2. One mask/shell pair holds every chunk's haloed instances of a shape (test `:112` covers two chunks sharing one geometry), which is why `frustumCulled = false` is right.
- **Edge-shell maths check out.** The near-plane value `−1.0001·P[3][2]/(P[2][2]−1)` reduces to `−1.0001·n` in view space for a standard perspective matrix, so the "behind" test and the two `mix` clamps land the end exactly on the near plane; both ends behind emits `vec4(0,0,2,1)`, which clips. `gl_Position` is written pre-divided with `w = 1`, so `vHaloCap` interpolates in the screen plane; the NDC offset `2·haloWidth/haloViewport` is the correct device-pixel-to-NDC conversion, and `haloAlong` is converted to pixels by `·0.5·haloViewport`. The across coordinate `haloSide.x·haloSide.y·haloWidth` correctly re-expresses the second end in the first end's frame (the second end's `haloAlong` is the negation of the first's). `haloLength` is identical at both ends because both clamp the same segment. The degenerate end-on case falls back to `vec2(haloSide.y, 0)`, giving a disc. The geometry builder's vertex order and `haloSide` signs are pinned per edge in `viewportHalo.test.ts:560`, and the edge counts match the shapes (30 for the 10-sided prism, 8 for the cone, 12 for the box, 296 for the torus arc).
- **Widths at DPR 1 and 2.** `SELECTION_HALO_CSS_PX = 2`, `HOVER_HALO_CSS_PX = 1`, multiplied by `renderer.getPixelRatio()`. `setViewportFromRenderer` is called from `resize()`, and the constructor calls `this.resize()` after building the halo, so the uniforms are never left at their `(1,1)` defaults. `render()` sets the GL viewport from the host's CSS size, which three multiplies by the same pixel ratio, so `haloViewport` always equals the drawing buffer. Pinned at `:273` for both ratios and at `:285` for the renderer read.
- **`discard` is confined to the shell's round cap** and never appears in the mask, so it cannot perforate mask coverage. The shell writes no depth, so losing early-Z there costs nothing.
- **Hidden instances.** `write` uses `instanceBaseMatrices`, never the drawn matrix, and `wanted` rejects a hidden key before that point, so a zero matrix can never reach a halo. Pinned at `:136`.
- **No per-frame work and no loop.** `render()` is untouched; nothing in `viewportHalo.ts` calls `invalidate`. Each of `setHoverPresentation`, `setSelectionPresentation`, `setThemePresentation`, `setVisibilityPresentation` and `replaceLayer` invalidates exactly once, and `setHoverPresentation` returns before doing anything when given the same key. Pinned at `:197` (two invalidations for four hover calls, `sync` never called on the hover path, ledger unchanged) and `:228` (one invalidation per theme switch with no create or dispose). `setTheme` and `setViewport` mutate two uniforms in place.

## 5. Behaviour as named — confirmed, and I found no unnamed change

Each of the manager's six named changes has a test that holds it:

1. Selection leaves the element's own colour and its edge line: `applySelectionPresentation` now always paints the base, `viewportFigureEdge.test.ts:386` holds the line at `canvas.edge` over a selected element that keeps its role colour, and `viewportHalo.test.ts:91` holds all three instances at the role colour while one gains a halo.
2. Hover outlines an unselected element only, including hover from a label — `PipeViewport.tsx:2307` sets `hoveredEntityKey` from the label's `onPointerEnter`, the same state the new effect watches. `:169` covers select-the-hovered, deselect, hide, unhover.
3. A hidden element shows none: `syncHoverFrom`'s guard and `wanted`'s `!hidden.has(key)`; `:136` and `:169`.
4. Overlapping halos read as a union: a consequence of both masks sharing `renderOrder` 9 000 and both shells testing against the combined mask depth. Not directly testable off a GPU.
5. Load arrows and the deformed overlay take no halo and no longer recolour: `syncHalo` passes `[this.modelLayer]` only; `:154` pins an arrow in the authored-load layer keeping its `loadForce` role colour with no pair.
6. A hidden selected element has no halo: `:136`.

**The halo carries no result meaning.** `sync`/`syncHover` take only selected keys, hidden keys and the hovered key. No run, deformation, diagnostic or result state reaches `viewportHalo.ts`, and the file imports only three, the entity-key type and the palette.

I looked for a behaviour change that is *not* named and found none. Two things I checked and cleared: `registerInstancedSelectionPresentation`'s switch from `applyInstancedSelection(mesh, new Set(), …)` to `paintInstancedBase(mesh)` is behaviourally identical (an empty set painted every instance at base already); and the new `invalidate()` on a hover change is redundant with the pointer-move handler, which already calls `viewportResourceRef.current?.invalidate()` on every move over the canvas.

## 6. Tests are real, and none weakened

`viewportHalo.test.ts` is 24 substantive tests over behaviour, resources, picking and geometry — no `skip`, no `only`, no `todo` anywhere in the three test files. Every changed assertion:

| Where | What it pinned | What pins it now | Verdict |
|---|---|---|---|
| `viewportFigureEdge.test.ts:389` | a selected instance's colour is **not** the role colour (i.e. it was recoloured) | the same line inverted to `toBe(role)`; the two neighbouring assertions (edge line at `canvas.edge`, the unselected instance) are untouched, and the absence of recolour is now pinned by `viewportResource.test.ts` and the halo suite | **move** (the named semantic change, inverted) |
| `viewportResource.test.ts:369` | selected material colour `=== SELECTED_DARK` | `toBe(0x24705a)` **plus** a new `not.toBe(SELECTED_DARK)`; `0x24705a` is the colour the test's own `MeshStandardMaterial` was constructed with, not a token | **strengthened** (two assertions where there was one) |
| `:417` | selected instance colour `=== SELECTED_DARK` | `toBe(0x2f6f73)`, the base the test passed to `registerInstancedSelectionPresentation`, alongside the unchanged assertion for instance 0 | **move** |
| `:523` | selected instance colour `=== SELECTED_LIGHT` after hiding | `toBe(0x4f6f73)`, again the fixture's own base; `SELECTED_LIGHT` ≠ `0x4f6f73`, so the pin still discriminates | **move** |
| `:774–791` | the selected colour survives a repaint | the repainted role colour, **plus** two new `expect(viewportRoleHex(theme,"pipe")).not.toBe(SELECTED_*)` guards that keep the assertion from passing vacuously | **strengthened** |
| `:911, :927` | selected instance at `SELECTED_DARK` / `SELECTED_LIGHT` after a theme change | `viewportRoleHex(theme, "pipe")`; the arrow, deformed and ghost assertions in the same test are untouched | **move** |

Nothing was deleted outright, no tolerance moved, and the two renamed tests describe the behaviour they now hold. **No appearance is pinned beyond token identity**: the three raw hexes are fixture inputs defined a few lines above their use, not design values; `SELECTED_LIGHT` / `SELECTED_DARK` remain derived from `viewportTokenColour(…, "canvas.selection")` and survive as negative pins.

## 7. Tokens — confirmed

`paintShell` reads `canvas.selection` / `canvas.hover` through `viewportTokenColour(this.theme, token)`; both tokens exist in `design/tokens.json` and in both themes of `tokens.css`. No literal: `viewportColourLiterals.test.ts` scans **every** non-test file under the viewport folder (it does not work from a fixed list for the literal check) and reports zero offences, so `viewportHalo.ts` is covered. Live repaint on theme change is pinned at `viewportHalo.test.ts:228`, including that the meshes, geometries, materials and instance buffers are the same objects afterwards and that nothing was disposed.

## 8. The probe tool change — confirmed

`afc84e916` touches nine files, all under the lane's own instance directory (`tools/`, `probes/C2-PROBE/`, `returns/`). It alters no product file, sits outside `apps/desktop`, and so ships in no bundle. `node --check` passes. The tool holds `RESERVED_PORTS = {5174, 5175, 5176, 5177, 5183, 5184, 5186}` and its README states that every browser and server goes through the run's lock script, one series per hold, on port 5185 by default — consistent with the manager's report that 5185/5186 were free and 5183/5184 never bound. Its only writes outside its `--out` file are `mkdtemp` scratch directories in the self-test, removed by `rmSync`. **The probe-only build copy left no trace**: the full file list of `origin/main...HEAD` contains no build artefact, no `dist`, and nothing outside the six product files, the lane records, the probe JSON and the screenshots. The retained records claim no performance acceptance; `slices/C2.md` mentions D-72 nowhere and carries the F-PIP-2 fence, and the manager's return states the numbers "guide the lane. Only your qualification runs decide D-72."

## 9. Scope and records — confirmed

Only `features/viewport/**` changed in product (verified by name filter). The two shell-lane requests are returned as requests and **not** made: no `styles.css` and no `uiDiagnostics.ts` appear anywhere in the range. No authored file in the range carries an absolute machine path (I grepped every non-PNG changed file for the user home pattern, the home pattern and a Windows drive prefix — none). The lane index lists each child brief with its hash, timestamp, model and return: `C2-HALO.md` `0e6ae041…` (`fable`, Claude Fable 5.1) and `C2-PROBE.md` `f32d9618…` (`fable`), with the retained returns' own hashes and the resume records.

## Residual risk, and what I did not check

Confirmed defects: none. The following are unverified risks and notes, none actionable before merge:

1. **Open-ended geometry and the front-face mask (needs a GPU).** The bend is a `TorusGeometry(…, Math.PI·0.75)` — an arc with no end caps. The mask material, like the figure, draws `FrontSide`, so where an open end shows the arc's interior the mask writes no depth and the edge shell's interior-edge quads can pass the depth test and paint halo colour inside the element's apparent outline. The figure already shows through there identically, and in the product a bend's ends abut pipes, so this may never be visible. The lane's 71 screenshots are the place to settle it; I could not.
2. **`matrixWorld` is read at sync time, before three has updated it.** `write()` premultiplies by `source.matrixWorld`, and `replaceLayer` calls `syncHalo()` immediately after `layer.add(...)`, when a freshly added object's `matrixWorld` is still identity. Today this is correct and unreachable: every selectable model object is an `InstancedMesh` registered by `registerInstancedRolePresentation` with its transform baked into the instance matrices, the layer groups carry no transform, and `setOrigin` moves the camera rather than the layer — so `isIdentity` is always true and the premultiply is dead defensive code. It becomes a one-frame misplacement if a later slice puts a transform on a chunk container or halos a plain `Mesh` with a local transform. Worth a line in C3's notes.
3. **Pixel ratio captured once (C1's residual risk, not widened).** `setViewportFromRenderer` runs only where the renderer is sized. The renderer's own pixel ratio is likewise set once in the constructor, so the halo's width is always exactly `CSS px × renderer.getPixelRatio()` and stays consistent with the edge line, which uses the same ratio. A DPR change without a size change leaves both stale together; the halo introduces no new inconsistency.
4. **A hover transition scans every instance key twice** (count pass, then write pass) and allocates one array and one `Map`. At the 10 000-pipe fixture that is roughly 40 000 key comparisons per hover *change* — not per pointer move, because `setHoverPresentation` returns early on the same key. The product already picks and calls `invalidate()` on every pointer move, and the manager measured 4.26 ms per pointer event with and without the halo. Not actionable.
5. **Pair capacity never shrinks** until the layer is replaced (the test pins 40 → 12 keeping capacity 64), and an `edgeGeometries` entry persists for a shape that is still in the layer but no longer haloed. Both are documented and bounded by the shape count and the largest selection.
6. `dispose()` calls `ledger.disposed({ geometries: 0 })` when no edge geometry was ever built. The ledger is a plain counter, so this is a no-op; cosmetic only.

**Not checked — everything that needs a browser or a GPU**: actual rasterization and the resulting picture; the 2 px / 1 px widths as drawn; MSAA behaviour at silhouette edges and the un-multisampled `discard` caps; blending and transparency sorting as the driver performs it; the union reading of overlapping halos; contrast ratios; the 71 screenshots and the pixel-for-pixel equality of forms (b) and (c); real draw-call, triangle and frame-time figures (I verified the 359-call claim only by reading the code path). Also not run, because the brief did not allow them: Playwright, the e2e and dist suites, `npm run test:desktop`, `npm run build:desktop`, the claims-language validator, the practitioner harness, any sweep, benchmark, probe or dev server. I verified the manager's claims about those only where the code or the commits could corroborate them.

**Suitable for manager fan-in.** I claim no usability, conformance or performance acceptance, and this review is not lifecycle acceptance.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
