# Sealed brief — C1A-PALETTE: canvas colours from tokens, first part

Sealed by B-CANVAS (WORKING_ITEMS, Type 1, the canvas lane manager of Tranche B) on 2026-09-19 before launch. Role of the reader: TASK (Type 2). Model requested: Claude Fable 5.1. Mechanism: Claude Code `Agent` tool, general-purpose type, background. **You work alone and never delegate: launch no agent.** Your parent is B-CANVAS; corrections reach you by message with your agent id.

Path placeholders: resolve `{REPO_ROOT}` with `git rev-parse --show-toplevel` in the git worktree your launch message names; `{WORKING_ROOT}` is `{REPO_ROOT}/projects/chirality-piping`; `{DESKTOP}` is `{WORKING_ROOT}/apps/desktop`; `{RUN}` is `{WORKING_ROOT}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION`; `{DESIGN}` is `{WORKING_ROOT}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM`. Write no absolute machine path in any file you author or in your return. Work only in the worktree you are given. **Run no git command that changes state** (no add, commit, stash, checkout, reset, merge, rebase, push); your parent commits.

## Authority and design basis

The owner authorized this program to implement on 2026-09-18 (`{RUN}/instances/ROOT/ACTIVATION_2026-09-18.md`). The design basis is read and never edited: the handoff `{DESIGN}/instances/ROOT/IMPLEMENTATION_HANDOFF_2026-09-18.md` (§3's constraints bind you); design system V1.3 `{DESIGN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md` (read §2.7, §2.8, the `canvas.*` and `cat.*` rows of §2.9, and §6.1, §6.2, §6.3, §6.4, §6.11); specification V1.2 `{DESIGN}/instances/UX-SPEC/UX_SPEC_V1.md` (§4.1 and §12 rows 12 and 14); the frames `{DESIGN}/instances/MOCKS/shots/s2_model_light.png` and `s2_model_dark.png` as drawings of intent, not pixel contracts. Where a frame and the specification differ the specification governs and you report the difference. You never attribute to the owner anything those records do not say. The product's own token file is `{DESKTOP}/src/design/tokens.json` with the generated `{DESKTOP}/src/tokens.css`; both belong to another lane and you never edit them. `tokens.json` may be imported from TypeScript (`resolveJsonModule` is on).

## Limits that bind this work (the lane's limits, carried in full)

- **One mutation route.** Every engineering action is a typed operation through `applyModelOperation` / `applyOperationBatch` into the Rust applier. No component mutates the model any other way. Tables and canvas are projections; neither holds state the model does not.
- **Result integrity.** Preserve the Current and Historical designation, the exact solve-input basis, the generation gates (`SolveRunGenerationGate`, `RuleRevisionGenerationGate`, the `stillCurrent` idiom), `commitModelAfterSolveInvalidation`, `clearComputedModelState`, reviewed application, undo and redo, and persistence compatibility. A Historical record never gains a current-model overlay, a chip or a readiness cue through a presentation change. The Stale standing is not drawn in this tranche: after a model change there are no results to show, as today.
- **Picking repair preserved.** PR #794's shared closest-point computation and its tests stay. Never alter a tolerance, an oracle expectation, a benchmark limit or a frozen characterization value to make something pass. If a test of frozen geometry breaks because layout changed, stop and report it to your parent with the numbers; do not adjust it yourself.
- **Rendering foundation retained**: persistent renderer, instancing and chunking, invalidation scheduler, resource ownership ledger, typed model index, selection and picking. You change none of these. If the task seems to need such a change, stop and return the need to your parent.
- **Every control maps to an operation or a classed gap.** A control whose gap is outside this tranche is absent, or disabled with its reason shown. It is never faked, and never wired to placeholder data. This task adds, removes, enables and disables no control.
- **Semantic changes are named** in your return as changes, with today's behaviour and the new behaviour. Nothing semantic is delivered as restyling.
- **Separately scoped work stays separate**: pressure runtime, connector mechanics, sparse execution, export implementation, schemas, `core/**`, `src-tauri/**`. The identity layer (document kinds, schema ids, crate names, `caepipe` identifiers, the four kept identifiers) is not touched.
- **Copy.** The product is SWBPIPE. Status labels come only from `src/features/workspace/statusLabels.ts` with their domains. "Accept", never "Approve", as a control; none of certify, seal, approve, authenticate, comply, compliant or sign-off as a control; no maturity sentence, no acceptance sentence, no other vendor's product named; Canadian spelling in anything a user reads. Every action has a pointer control; a key is an accelerator only. This task changes no copy.
- **Accessibility.** WCAG 2.2 AA for every control you touch: name, role, state, focus order, visible focus, contrast from tokens, target size. PDU-045 and PDU-046 remain holds; claim no usability acceptance.
- **Browser tests share the host.** Another lane works at the same time in another worktree of this repository on the same machine. The Playwright configurations use fixed ports and reuse a listening server. Every Playwright run and every dev or preview server you start goes through `sh {RUN}/tools/with_e2e_lock.sh <command>`, with `PLAYWRIGHT_WORKERS=1`. Waiting for the lock is normal, sometimes for many minutes. Run **both** lanes: the source lane (`npm run test:e2e` from `{DESKTOP}`) and the dist lane (`npm run test:e2e:dist`). Stop every server you start and confirm the port is free and the lock released. For looking at the product use a port other than 5174 and 5175 (use 5184). If your parent tells you ROOT has announced a timed benchmark, run no build or test until told it is over.
- **Never weaken a test.** A test that pinned display text or structure that the design changes moves to the new text or structure, and you say so; a test of behaviour is never deleted or loosened. `src/App.test.tsx` is the safety net for behaviour.
- **Frozen history** is never edited: `{WORKING_ROOT}/execution/**` (you write only the screenshots folder named below), `validation/evidence/**`, `plans/**`, `docs/_history/**`, `docs/_ScopeChange/**`.
- **Picking first.** Before anything else is looked at, and again at the end, run `npx vitest run src/features/viewport/viewportSelection.test.ts` from `{DESKTOP}` (it holds the shared-endpoint fixture tests). A change that alters a picking result is rejected, not explained.
- **Per-frame work is measured, not assumed.** This task must add no work per frame and none per pointer move. Repaint happens on a theme change and when a layer is replaced, never in the render loop.
- **The benchmark instrument.** Nothing under `{DESKTOP}/e2e/ui-foundation/**` is edited. Two product files are hash-bound by that instrument and stay **byte-identical** in this task: `src/features/viewport/viewportSelectionPresentation.ts` and `src/features/viewport/viewportSelection.ts`.

## Write scope

`{DESKTOP}/src/features/viewport/**` and its tests, except the two byte-identical files above. `{RUN}/instances/B-CANVAS/screenshots/C1A/**` for screenshots. Nothing else: not `App.tsx`, not `styles.css`, not `tokens.json`, not `tokens.css`, not `e2e/**`. If you need a change outside the scope, do not make it; put the exact change in your return.

## Two colours are held, and why

The benchmark instrument recognises a selection by the selected colour and reads the scene background back from the shell. Until its second profile is accepted, **two colour families keep today's values exactly**:

1. the scene background: `0xdfe5e8` light and `0x0c1114` dark (`viewportResource.ts`: the constructor, `applyThemePresentation`, and `GIZMO_THEME_PALETTES.*.canvas`, which mirrors it);
2. the selected colour and its cue rim: `SELECTED_COLOR_LIGHT` `0xa34400`, `SELECTED_COLOR_DARK` `0xf08c22`, and the rim pair `0xffffff` / `0x0c1114` passed in `updateSelectionCue`.

Selection's behaviour is unchanged in this task: `applyInstancedSelection` still recolours, the cue still draws. Gather the held literals into one clearly marked block of named constants in `viewportResource.ts`, with a comment saying they are held for the benchmark instrument's second profile and move to `canvas.bg` and `canvas.selection` in a later slice. No other colour literal remains anywhere in `src/features/viewport/**` outside test files and the new palette module's own parser tests.

## What to build

### 1. A palette module: `src/features/viewport/viewportPalette.ts`

The one place where the canvas reads tokens.

- Import `../../design/tokens.json`. Parse at module load, once, into frozen per-theme tables. No DOM read, no `getComputedStyle`: the numbers come from the token file, so a canvas and a test get the same values.
- `parseTokenColour(value)` accepts `#rgb`, `#rrggbb`, `rgb(r,g,b)` and `rgba(r,g,b,a)` (spaces tolerated) and returns `{ hex, alpha }`; anything else throws with the offending value. `canvas.labelBg` and `canvas.hint` are the two `rgba` tokens.
- Expose all 22 `canvas.*` tokens for both themes (`bg`, `gridMajor`, `gridMinor`, `pipe`, `pipeShade`, `edge`, `edgeAlt`, `glyph`, `glyphFill`, `label`, `labelBg`, `hint`, `vector`, `selection`, `hover`, `draft`, `proposalGhost`, `deformGhost`, `unsolved`, `axisX`, `axisY`, `axisZ`), the eight `cat.1` to `cat.8` slots, and `surface.raised`. Several have no consumer until later slices; they are parsed and tested now so later slices only add consumers. `canvas.edgeAlt` is exposed and **not consumed**: its use is a later feasibility probe.
- A **role** table, the single binding from what the canvas draws to a token. Roles and tokens:

| Role | Drawn by (today's literal) | Token |
|---|---|---|
| `pipe` | pipe instances (`0x4f6f73`) | `canvas.pipe` |
| `node` | node spheres (`0x2f6f73`) | `canvas.pipeShade` |
| `support` | support cones (`0x6b7d49`) | `canvas.glyph` |
| `componentBend` | bend torus (`0x1f6f73`) | `canvas.pipe` |
| `componentBranch` | branch stub (`0x24705a`) | `canvas.pipe` |
| `componentExpansion` | expansion joint (`0x6f5a92`) | `canvas.pipe` |
| `componentRigid` | rigid box (`0x1f6f73`, shared with the bend today) | `canvas.pipeShade` |
| `deformedShape` | deformed pipes and their node markers (`0x0f8f85`, emissive `0x03433f`) | `canvas.deformGhost` |
| `groundGridMajor`, `groundGridMinor` | reference ground (`0xb6bfb9`, `0xdce1db`, opacity 0.55) | `canvas.gridMajor`, `canvas.gridMinor`, opacity 1 |
| `routeGridAxis`, `routeGridLine` | routing construction grid (`0x2f6f73`, `0x9bb7b4`, opacity 0.44) | `canvas.draft`, `canvas.gridMajor`, opacity 1 |
| `routeDraft` | route ghost line and its end marker (`0xf08c22`) | `canvas.draft` |
| `loadForce` | load arrows that are not moments (`0xd9822b`) | `cat.1` |
| `loadMoment` | moment arrows (`0x7b4ea3`) | `cat.2` |
| `gizmoAxisX`, `gizmoAxisY`, `gizmoAxisZ` | gizmo axes and letters | `canvas.axisX`, `canvas.axisY`, `canvas.axisZ` |
| `gizmoBadge` | the disc behind each gizmo letter (`0xffffff` / `0x0c1114`) | `surface.raised` |

  Reasons you may need: the design draws a neutral figure, so fittings take the tube's colour and are told apart by shape, and a rigid element is `canvas.pipeShade` (design system §6.3); a restraint glyph is `canvas.glyph` (§6.4), and a solid cone in `canvas.glyphFill` would vanish against the ground; load kinds take `cat.1` force and `cat.2` moment (§2.7), and today's code only tells moments from everything else, which you keep; the routing draft has its own token and is never the selection's colour (specification §12 row 12, a named semantic change); the grid tokens are already stepped for the ground, so they draw at opacity 1; `surface.raised` for the gizmo badge keeps the existing contrast test true (axis letter on badge at 4.5:1 or better) where `canvas.labelBg` over the ground would not. Node spheres and today's deformed overlay have no drawing in the design; the tokens above are the manager's choice and are reported as such. Do not change the table; if you think a row is wrong, say so in your return.
- `GIZMO_THEME_PALETTES` keeps its exported shape (`canvas`, `badge`, `axes`) because a test reads it; `axes` and `badge` now come from the palette, `canvas` stays the held background.

### 2. Neutral matte shading, so that a drawn colour is its token

Today every model mesh is a `MeshStandardMaterial` under an ambient and a directional light. In three r181 a lit surface renders well below its albedo (roughly 0.4 of it in linear terms here), so a tube given `canvas.pipe` would render darker than `canvas.pipeShade` in light and at about 2:1 on the ground in dark, where the design states 4.0:1. The design asks for shading that is "neutral and matte: the tube's colour is `canvas.pipe` with a soft darkening toward the silhouette in `canvas.pipeShade`, enough to read as a cylinder and never enough to read as lit" (§6.2).

- Add `src/features/viewport/viewportFigureMaterial.ts`: a small `THREE.ShaderMaterial` factory, **no texture and no light**, that supports instancing and per-instance colour through three's own chunks (`color_pars_vertex`, `color_vertex`, `beginnormal_vertex`, `defaultnormal_vertex`, `begin_vertex`, `project_vertex`, `color_pars_fragment`, `colorspace_fragment`), a `tint` uniform for a non-instanced mesh, an `opacity` uniform, and a `shadeRatio` colour uniform. Fragment colour: `mix(base * shadeRatio, base, smoothstep(0.0, 0.75, facing))`, where `base` is the instance colour times the tint and `facing` is `abs(dot(normalize(viewNormal), normalize(viewDirection)))`. A surface facing the camera therefore renders exactly its token; the silhouette renders the token times the ratio.
- `shadeRatio` per theme is `linear(canvas.pipeShade) / linear(canvas.pipe)` per channel, from the palette; it is set when a material is painted for a theme. Do not invent another shade value.
- Every model, result and routing mesh that is a `MeshStandardMaterial` today uses the figure material (the deformed overlay keeps its `transparent` flag and its opacity values 0.82 and 0.86; the emissive glow goes, because a matte figure has none). Load arrows are already unlit `MeshBasicMaterial` and stay so. When no lit material remains in the scene, remove the two lights from `ViewportResource` and say so in your return; if one must remain, say which and why.
- A texture-free material is deliberate: a shared texture would have to enter the resource ownership ledger and `disposeObjectChildren`, which is foundation you do not change. The ledger's counts for materials and geometries must come out as they do today; say in your return whether any count changed.
- A shader cannot be compiled in the jsdom unit tests. It is proven in the browser: after your change the running product must show no shader or WebGL error in the console in either theme, with the model drawn, a selection made, Actual OD on, and the deformation overlay shown if the sample project offers a solve.

### 3. Live repaint in both themes

- Builders in `PipeViewport.tsx` register a **role** with what they build (on `userData`), not only a number. `registerInstancedSelectionPresentation(mesh, keys, baseColor)` keeps its signature working, because tests use it with arbitrary colours; add the role as an optional fourth argument or a companion function. A mesh with a role has its `viewportBaseColor` rewritten from the palette when it is painted for a theme; a mesh without one keeps the number it was given.
- `ViewportResource.setThemePresentation(theme)` repaints, in place, every role-coloured object in all **five** layers (model, authored loads, result, diagnostic, routing) and the gizmo: instance base colours, material tints, `shadeRatio`, line-material colours, and the vertex-colour attribute of each `GridHelper` (rewrite the attribute's values; do not rebuild the helper). It then re-applies selection over the new base colours and invalidates once. `replaceLayer` paints the incoming objects for the current theme the same way, so a rebuilt layer is right in dark without a theme change. The routing layer is not in today's repaint lists; it must be in the new one.
- Repaint creates and disposes **no** GPU resource: no new material, geometry, texture or mesh. It allocates no per-instance object in a loop where a scratch `THREE.Color` will do (today's `applyInstancedSelection` allocates one `THREE.Color` per instance per call; reuse one scratch colour there too, without changing what it paints).
- Order on a theme change stays: background (held values), gizmo, role colours, selection, cue, invalidate.

### 4. Dead code

`PipeViewport.tsx` holds builders that nothing calls: `pipeMesh`, `supportMesh`, `componentMesh`, `deformedPipeMesh`, `deformationMarker`, `referenceGround` (the `model` overload; `referenceGroundFromBounds` is live), `routeConstructionGrid` (the `model` overload; `routeConstructionGridFromBounds` is live), `buildOrientationGizmo`, `axisLabelSprite`, and helpers only they use (`toVector`, the file-local `isRigidComponent`). They hold most of the file's colour literals, including fourteen uses of the selected colour. Confirm each is unreferenced across `{DESKTOP}/src` and `{DESKTOP}/e2e` (word-boundary search; a local variable named `referenceGround` exists and is not a call), remove the ones that are, list them in your return, and keep any that turns out to be referenced, saying where.

### 5. Tests (vitest, beside the code)

- `viewportPalette.test.ts`: all 22 `canvas.*` tokens, `cat.1` to `cat.8` and `surface.raised` resolve in both themes; the two `rgba` tokens parse to the alpha the token file states; the parser rejects a malformed value; every role resolves in both themes; values are read through the token file rather than re-typed, with at most two literal spot checks.
- A source-scan test: no colour literal (`0x` followed by six hex digits, a quoted `#` hex colour, `rgb(` or `rgba(`) in any non-test file under `src/features/viewport/` except the held block in `viewportResource.ts`, whose exact allowed set the test states, and the parser's own documentation comments if any.
- Repaint tests in `viewportResource.test.ts` (no WebGL needed; follow the file's existing style): a role-registered instanced mesh takes the dark token after a dark repaint and the light token after a light one; a selected instance keeps the held selected colour across a repaint and an unselected one takes the new base; a `GridHelper` painted for a theme carries the two grid tokens in its colour attribute; repaint leaves the ownership ledger's snapshot unchanged.
- Existing tests are not edited unless one pins something this task changes by design; if so, move it, never loosen it, and list it. The gizmo contrast test must pass unedited.

## How to check your work

All from `{WORKING_ROOT}` unless stated. Report each command, its exit status and its counts.

1. `npx vitest run src/features/viewport/viewportSelection.test.ts` from `{DESKTOP}`: first, before you change anything, and last.
2. `npm run test:desktop` and `npm run build:desktop`.
3. From `{DESKTOP}`: `PLAYWRIGHT_WORKERS=1 sh {RUN}/tools/with_e2e_lock.sh npm run test:e2e`, then `PLAYWRIGHT_WORKERS=1 sh {RUN}/tools/with_e2e_lock.sh npm run test:e2e:dist`. Twenty tests in `e2e/ui-foundation/full-cohort-controller.spec.ts` skip on missing `D70_WRITER_*` inputs; that is expected. Any other skip or any failure is reported with its output.
4. From `{REPO_ROOT}`: `python3 tools/validation/validate_claims_language.py`.
5. Look at the product in a real browser through the lock (dev server on port 5184, stopped afterwards): both themes; the sample project's model; one pipe selected; Actual OD on; the grid on; loads shown; a route started if the sample allows it, so the draft ghost and construction grid are seen; the deformation overlay if a solve is offered. Save 1:1 PNG screenshots of the viewport region under `{RUN}/instances/B-CANVAS/screenshots/C1A/` with names that say theme and state (for example `light_model.png`, `dark_selected_actual_od.png`), and take the same states **before** your change first (`before_` prefix) so the pair can be compared. Read the browser console in each state and report any error or warning that your change introduces. A throwaway Playwright script for this lives outside the repository or is deleted before you return; it is not part of the change.
6. `git status --short` at the end (read-only): list every file you changed or added, with its SHA-256.

## What to return

Your final message is your return; your parent retains it verbatim. It states: who you are (model id), that you verified this brief's hash before starting (your launch message gives it), that you worked alone and ran no state-changing git command; what you read, whole and in part; every file changed or added with its SHA-256; how the work was done where it is not obvious; each check with command, exit status and counts; **semantic changes named as changes**, with today's behaviour and the new behaviour (at least: the routing draft leaves the selection's colour for its own token; element kinds are no longer told apart by colour; shading is matte and unlit); controls left absent or disabled (expected: none touched); where the design could not be followed and why; differences found between a frame and the specification; what reads wrong in the running product in either theme, including what the held background does to the new colours (measure the contrast of `canvas.pipe`, `canvas.gridMajor` and `canvas.gridMinor` against the held ground in both themes); the screenshots by path; any ledger count that changed; anything that needs a change outside your write scope, as an exact proposed change; and what you did not do. Claim no usability, conformance or performance acceptance.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
