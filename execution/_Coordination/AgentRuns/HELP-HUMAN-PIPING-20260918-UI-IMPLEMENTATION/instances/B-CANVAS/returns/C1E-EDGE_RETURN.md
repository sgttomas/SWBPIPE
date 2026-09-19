# C1E-EDGE return, retained (model that ran: Claude Fable 5.1, claude-fable-5-1, by its own statement)

Retained by B-CANVAS (ROOT's script). What follows is the agent's final message as the host stored it, with the machine's repository path replaced by `{REPO_ROOT}`; nothing else is changed. The message exactly as the host stored it has SHA-256 `ae51f1895355d9fa0bff1ba0c7150c023e9357f612774521e5260382b40a6fc1` (230 lines), by this script.

---

# C1E-EDGE return: the edge line is built and drawn; no probe series was run

**The line's per-frame cost is not measured, and the rule for keeping it has not been applied.** The probe tool's README never appeared and its `STATUS.md` never went away, so the brief did not let me run a series. I was ready to measure at 01:00 local on 2026-09-19. I checked every five minutes for ninety minutes, to 02:30, and neither condition held at any check. T1-ORBIT-PROBE-2's last file under `{LANE}/probes/T1/` is from 00:20:52. Everything else in the brief is done. The manager can run the series or resume me by message.

**Who I am.** Claude Fable 5.1, `claude-fable-5-1`, as TASK (Type 2). I worked alone, launched no agent, and ran no state-changing git command. Before starting I verified this brief's SHA-256 (`2bbcf72d…de74dcb`), the probe's (`ac5290bc…598368`) and the base build's `index.html` (`e79fafbe…bd7dfb`). The probe was patched by its own child at 00:17:17 and ended my wait at `59b1807659e6b82f0554c0af659076ec4e2f2b569ae35dc846ec0f9e835c7aac`. It still shows as modified in the worktree; that change is not mine.

**What I read.**
- Whole: the brief; P2; addendum 2; the C1A screenshot README; T1's `STATUS.md`; the lock script; `viewportFigureMaterial.ts` and its test; `viewportPalette.ts`; `viewportColourLiterals.test.ts`.
- In part:
  - design system §2.7, §2.8, the `canvas.*` rows of §2.9, §6.1 to §6.3, §6.6, §6.8 and §6.11;
  - specification §4.1;
  - handoff §3;
  - addendum 3's ledger rule;
  - the C1A return (the look method, §10 and §11);
  - review checks 1 to 9;
  - `viewportResource.ts` and its repaint tests;
  - the builders in `PipeViewport.tsx`;
  - three 0.181.2's chunks, geometries, `ShaderMaterial.copy` and the `WebGLProgram` prefixes;
  - the probe's header and its launch, theme and fixture code;
  - `e2e/workspace-driver.ts`.
- Not opened: the frames `s2_model_light.png` and `s2_model_dark.png`, addenda 4 and 5, the specification beyond §4.1. I can report no frame-versus-specification difference.

## Mechanism as built

The line is mechanism A: an option of `FigureMaterial`, `edge?: FigureEdgeOutline | null`. A material built without it assembles today's shader byte for byte, with no new uniform and empty `defines`; a test pins this against a literal copy. With it, the material gains two uniforms:
- `edge`, a `THREE.Color` set by `setHex`, so it sits in the same working space as `tint`.
- `edgeWidth`, the line's width in device pixels.

Each shape's constants are material `defines`.

One rule measures everything: for a value `g` that is zero on a line of the picture, `g / length(dFdx(g), dFdy(g))` is the distance to that line in device pixels. The band is `1 − smoothstep(w − 0.5, w + 0.5, distance)`, mixed over the unchanged shaded colour. The fragment shaders contain no `if`, loop or `discard`, so every derivative is outside control flow; a test pins that.

**Camera in the element's frame.** It comes from `modelViewMatrix * instanceMatrix` alone, as the brief's point 3 describes, with a guarded divisor. A hidden instance's zero matrix gives a camera at the origin, so no plane is built and nothing is drawn. No layer above the meshes carries a scale or a shear: the five layers are plain `Group`s under the scene, and the origin shift only moves the camera, the orbit target and the selection-cue group. Under an orthographic camera the part of the outline that depends on the eye is left out; the product has only a `PerspectiveCamera`.

**Prism (pipes, branch, expansion joint).** I built the manager's points 2 to 4 as written:
- The vertex shader finds the two tangent vertices from `α = atan(c.x, c.z)`, `β = acos(cos(π/N)/ρ)` and the floor/ceil rule.
- It forms `g± = dot(p.xz − v, n)`, with `n` turned toward the axis.
- It carries `g+, g−` and the cap pair in one `vec4` varying.
- When `ρ ≤ 1` (the camera inside the tube's radius), no band is drawn.

I repeated the manager's numerical check with my sign rule: 120,000 lines over N of 8, 10 and 12, from just outside the tube to thousands of radii away, with no failure and no line through two vertices.

The cap rim uses a value of 1 at the cap's centre vertex and 0 at its rim vertices. That value is affine on each fan triangle and zero exactly on the rim edge. A cap is told from the side by its flat normal (`step(0.5, abs(normal.y))`).

**Box (rigid).** This differs from the brief's `uv` suggestion. For each of the face's two in-plane axes it takes `(halfExtent − |p|)` over that coordinate's screen rate. A border whose neighbouring face is also seen draws at half width, so a crease drawn from both sides totals one line width. With `uv` alone it would have been two.

**Torus (bend).** This is where my analysis differs from the manager's:
- A torus normal lies in its section plane. Whether a face is seen therefore depends only on the eye projected into that plane, and the prism rule applies exactly, section by section.
- The tangent vertex changes along the sweep, so the planes are found per fragment.
- `dFdx(g)` across that change would be a jump and would draw a false band across the tube. The distance instead combines the derivatives of the continuous section coordinates with the plane normal.
- It costs one `atan`, one `acos`, two `sin`, two `cos` and three `normalize` per bend fragment.
- The arc's open ends have no line.

**Outlines, width and colour.**
- `figureEdgeOutlineFor(geometry)` reads the outline from three's `geometry.parameters`, so each constant is stated once, where the geometry is made.
- It returns null for shapes it has no rule for. My test caught a real defect here before any browser run. `ConeGeometry` is a `CylinderGeometry` by class whose parameters lack `radiusTop`, so `undefined === undefined` passed it as a prism with an undefined radius. Cones are now refused by class.
- The width is `ViewportResource.figureEdgeWidth`, read once where the renderer's pixel ratio is set. It is passed down `applyPalettePresentation(roots, theme, edgeWidth = 1)`. `paintFigureShade` writes it, next to the shade ratio and the `canvas.edge` colour.
- The width is not a constant and is not read per frame. The renderer's construction is unchanged, and nothing was added to the render loop.
- Review finding F4's unused import is dropped from `PipeViewport.tsx`; the export stays.

**No line** on node spheres, supports, the deformed overlay, its markers and the routing marker: their materials ask for none. Load arrows are a different material; grids and the gizmo are not figure materials.

## Files changed or added

| File under `{DESKTOP}/src/features/viewport/` | SHA-256 |
|---|---|
| `PipeViewport.tsx` (M) | `7478684ddbf633c2d1b72dd8b5229e0d315c5cd8fdc5d4c5829ded927365d045` |
| `viewportFigureMaterial.ts` (M) | `127117c24d4ef9b358a7cbfe542eee207bccfa224220c5244dbedce5bb669790` |
| `viewportResource.ts` (M) | `910008e3d197094d89cfffb91bc68dcb017c338a5b15550b33ad41d5dbf4c211` |
| `viewportFigureEdge.test.ts` (new, 24 tests) | `c54330eeba4df613886c37140a8a18bae025fb405d551f567f301f7b5c7f72e9` |

Evidence folders:
- `{LANE}/probes/C1E/SUMMARY.md`, SHA-256 `81d0a21c699ff3265979355e70d0d2e88331bb22fdb4edd71cb9b4c647fcff69`.
- `{LANE}/screenshots/C1E/README.md`, SHA-256 `4f77bdaf904af0dcad56c17fcddfbbc47e64ca6fc6abaf19ccd55243aada304a`, plus 40 PNGs; the README gives each file's state and SHA-256.

Read-only `git status --short` lists the four viewport files, the C1E screenshot folder, the lane's `probes/` folder and T1's `orbit_probe.mjs`. Nothing else is touched:
- no file under `e2e/**` is touched;
- none of my authored files carries an absolute path;
- a scan for URL, storage or global switches in `viewportFigureMaterial.ts` and `viewportResource.ts` found none;
- no probe code and no scratch file is in the tree.

## Evidence by stage

**Stage 0.** Hashes verified; picking test 69/69 before any edit.

**Stage 1.**
- Tests were written first, then the shader.
- The first typecheck failed on literal template types; I fixed it with a declared source type.
- I looked in headless Chrome with hardware WebGL, from the source on port 5186: both themes, both geometry modes, both ratios, fitted and close. There were zero console entries, so every shader compiled and linked.
- Stage-1 build copy: `dist_stage1`, `index.html` `c43b10550124d44a9dbbf3b39ed5debb3ac3d936058f51c6e85748afb91af16e`.
- The decision point was not reached, for want of the tool.

**Stage 2.**
- All four placeholder kinds drew their line with zero console entries.
- Stage-2 build copy: `dist_stage2`, `0933b9d42a30898a44b3c582ebd56359a1f8b2be4b0299598ad06f01259044d1`. `npm run build:desktop` on the final tree reproduced that hash exactly, so it is the final build.
- The pipe material's shader is identical in the stage-1 and stage-2 builds.
- The `resources` command and the orbit series were not run.

**Stage 3.** Not built. The design and the reason are under "Stage 3" below.

**Stage 4: closing checks.** All heavy commands went through the lock.

| Check | Exit | Result |
|---|---|---|
| Picking test, first and last | 0 | 69 passed both times |
| `npx tsc -b` | 0 | clean |
| `npx vitest run src/features/viewport` | 0 | 10 files, 210 tests (186 before, 24 new) |
| `npm run test:desktop` | 0 | 79 files, 1254 passed |
| `npm run build:desktop` | 0 | the hash above |
| `test:e2e` | 0 | 374 passed, 20 skipped (the expected `D70_WRITER_*`), 0 failed, 9.0 min |
| `test:e2e:dist` | 0 | 53 passed, 0 skipped, 0 failed, 2.8 min |
| `validate_claims_language.py` | 0 | 337 files, valid |
| The two byte-identical files | n/a | hashes equal to the brief's |

- No test that reads canvas pixels or frozen geometry broke.
- Ports 5185 and 5186 were free at the end.
- The listener on 5174 at 01:14:29 belonged to another lane (worktree `swbpipe-wt3`), which took the lock after my lanes ended.

## Measurement

There are no tables, no load averages, no canvas sizes and no rule outcomes, because no series and no `resources` run was made. `{LANE}/probes/C1E/SUMMARY.md` says this first. It then lists the three builds by hash and the series to run, and states what the line costs by construction:
- No draw, vertex, geometry, material, texture, buffer or scene object is added.
- Per pipe fragment: six derivatives, three lengths, three divisions, one `smoothstep`, two `mix` and one `step`.
- Per pipe vertex (64 per instance): one 4×4 matrix product and a handful of trigonometric calls.
- T1's status note matters to whoever measures. On this host the uncapped orbit is bound by the main thread, and the tool does not see GPU cost under that bound. The line adds no main-thread work per frame. "No difference" here would not speak for a GPU-bound host.

**Ledger counts.** None changed. A unit test gives equal `live` counts for an edged and a plain pipe mesh: 1 mesh, 1 geometry, 1 material, 0 textures, 1 instance matrix, 1 instance colour. The ledger stays equal across four theme switches with no `dispose` call and no change to `material.version`. The running ledger was not read, for want of `resources`.

## Stage 3: not built

- **Model index.** No field was added.
- **Reason.**
  - The node sphere's radius is 0.095.
  - The sample's one outside radius is 0.084 (0.052 in schematic).
  - Both fixtures hold radii of 0.03 to 0.09.
  - So in every model the product opens today, every pipe end is inside a node sphere. A ring on the side surface at an end could not be seen. The fixture pictures show this.
- **Design, if the node geometry changes later.**
  - In `modelIndex.ts`: `pipeEndFacts: ReadonlyMap<EntityKey, Readonly<{ from: PipeEndFact; to: PipeEndFact }>>`, with `PipeEndFact = Readonly<{ free: boolean; meetsDifferentSection: boolean }>`, built where the index is built and frozen.
  - In the viewport: one light geometry per chunk that shares the base attributes and index, with an `InstancedBufferAttribute` of two flags. The side band is `0.5 − |p.y|` by the same rule.
  - That would add one geometry to the ledger per pipe chunk.
- **Fixture counts that bear on it.**
  - 1,000 pipes: 984 of 1,001 nodes join differing sections; 17 free ends.
  - 10,000 pipes: 9,843 of 10,001 nodes join differing sections; 158 free ends.
- The cap rims of brief point 4 are in stage 1, under the same spheres.

## Semantic changes, named as changes

1. **The figure gains a line it never had.** Before, pipes and placeholders were a fill shaded toward the silhouette. Now a 1 CSS px band in `canvas.edge` lies inside the silhouette of pipes in both modes, bends, branches and expansion joints. Every seen face of the rigid box is outlined. A visible cap has a line at its rim.
2. **Element outlines darken in light and lighten in dark.** The rim was `canvas.pipeShade`; it is now `canvas.edge`.
3. **Distant tubes draw as the line's colour alone.** A tube under two line widths wide is all `canvas.edge`. Large models at the fitted camera read darker in light and lighter in dark than today.
4. **A selected element shows `canvas.edge` at its outline over the held selected colour.** Before, a selected element was the selected colour right to its silhouette.

**Controls left absent or disabled.** None; no control was touched and no copy changed.

**Where the design could not be followed.**
- The line is inside the silhouette, not on it.
- Changes of section read only where a cap is open to view, which node spheres prevent today.
- The 1.5 px centreline for an element with no section is slice C5's; this slice does not draw it.

## What reads wrong in the running product

I looked at my own pictures at full size.
- **A thin pale line under a pipe lying on the ground plane** (`light_dpr2_angled_joint.png`). This is not mine. The same pixels are in the base build (`#b4b7bc #b3b7bb`) and vanish in both builds with the grid off. With the grid off the candidate's edge reads `#6d7379`, `#42484f`, `#696f75`. The dark line beside it makes it easier to notice.
- **At ratio 1 the line's darkest pixel is often short of the token.** I measured `#575d63` and `#666b71` against `#42484f`, because the ink splits between a multisampled outer pixel and a smoothstepped inner one. At ratio 2 a full `#42484f` pixel appears, so the line is a little heavier in tone at ratio 2.
- **Three orbit angles of one close view** show no break or pulsing that I could see. They are stills, not a recording.
- **No doubled line where tubes meet and no tick at an angled joint** in the sample: a node sphere sits over every joint.
- **The bend's open arc ends** have no line.
- **In the fixtures** the figure is mostly beads; no line reading is possible there.

## What follows from the mechanism

**The `canvas.edgeAlt` choice of slice C6.**
- Per material: one more `setHex` in the repaint, for free.
- Per element: the `edge` uniform cannot vary per instance, and `instanceColor` is taken. So it needs the same one-geometry-per-chunk route as stage 3, with a flag or colour attribute and one `mix`.
- Cheaper still once result colour exists: choose in the shader from the fill's own luminance, with no new data.

**Slice C2's halo** cannot stand on this band, which is inside the silhouette; a halo is outside it. The reusable parts are:
- the camera-local computation;
- the tangent-vertex rule, which is what an analytic halo quad (P2's mechanism C) would need;
- `figureEdgeOutlineFor`;
- the option, define and uniform pattern with its repaint path.

## Appearance left for the closing pass

None of this was tuned.
- **Derived silhouette shades** (role colour times shade ratio) against `canvas.bg`, with the held ground in brackets:
  - node and rigid: `#646a70`, 4.66:1 (4.30) in light; `#3a3f45`, 1.61:1 (1.79) in dark. The dark value is the worst.
  - support: `#23292e`, 12.53:1 in light; `#9fa3a9`, 6.75:1 in dark.
- `canvas.edge` on the held ground is 7.27:1 in light and 11.65:1 in dark.
- The rigid box's line on `canvas.pipeShade` is 2.55:1 in light and 4.35:1 in dark. §2.9 has no row for this pair.
- **Roles the design does not name:** the line on node spheres and supports; creases on a box; a line at a torus's open end; the line's weight at ratio 1 against ratio 2 (numbers above).
- The band's inner edge is a one-pixel smoothstep.

**Contrast not yet checked:** none; no control touched.

**The measured node-sphere pixel** (`dark_schematic_dpr1_node_sphere.png`, ratio 1, dark):
- The first fully covered rim pixel is `#444a50` on the left and `#42474d` on the top, each after one antialiased pixel (`#30363b`, `#32373d`).
- The centre is `#53595f`; the ground is `#0c1114`.
- The predicted `#3a3f45` is the value at facing exactly zero. No pixel centre reaches it, so the two agree.

**Screenshots:** `{LANE}/screenshots/C1E/`. They comprise:
- the sixteen base pictures (both themes, both geometry modes, ratios 1 and 2, fitted and close);
- per theme at ratio 2: `selected_pipe`, `hidden_beside_visible`, `kind_bend`, `kind_branch`, `kind_rigid`, `kind_expansion`, `free_end`, `angled_joint`, `diameter_change_view1`, `diameter_change_view2`;
- `light_dpr2_orbit_1` to `light_dpr2_orbit_3`;
- `dark_schematic_dpr1_node_sphere.png`.

The README states what the data does not show: a readable change of diameter, and an uncovered pipe end.

**Commands I ran through the lock while T1 was alive:**
- vitest, four times;
- `tsc -b`, three times;
- `build:desktop`, three times;
- the look script, fifteen times: four failed for my script's own reasons and one was a repeat to read a failure;
- `test:desktop`;
- `test:e2e`;
- `test:e2e:dist`.

**Outside my write scope:** nothing is needed.

**A probe observation, not a defect of my work:** the lock has no queue, so a waiter can be passed over. T1 wrote no evidence file after 00:20:52 while other lanes and I took the lock.

**What I did not do:**
- any probe series or `resources` run;
- stage 3;
- a read of the running ledger or renderer counts;
- a recorded orbit;
- opening the two frames;
- any change to a limit, tolerance, test of behaviour, token, stylesheet, `e2e/**`, `{LANE}/tools/**` or `uiDiagnostics.ts`.

I claim no usability, conformance or performance acceptance.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
