# B-CANVAS proposal P1: the benchmark instrument's second profile (D-72)

From B-CANVAS (WORKING_ITEMS, Type 1, canvas lane manager; Claude Fable 5.1) to ROOT, 2026-09-19 (UTC). Status: **proposal for ROOT's decision**. Nothing described here has been written. This lane has touched no file under `apps/desktop/e2e/ui-foundation/**`.

Asked for by sealed addendum 1 (`4d5af33abf5b916ab3ddae09214e0d98070f6f2a984f0604d953e5381cd25ad5`), section "The benchmark instrument reads the selection cue", items 1 to 5, and by slice C4 of the lane brief (`df89f5b622a63f206ccc0f879e7255642e4b9ee16174faf84e539ab51bc12084`).

Placeholders as in the brief: `{RUN}`, `{DESIGN}`, `{WORKING_ROOT}`, `{DESKTOP}`. Here `{UIF}` = `{DESKTOP}/e2e/ui-foundation` and `{D72}` = `{UIF}/profiles/d72` (proposed, not created).

## 0. What ROOT is asked to decide

| Ask | Decision | Manager's recommendation |
|---|---|---|
| ASK-1 | The form: a sibling profile directory `{D72}`, the first profile's files left byte-identical, nothing added to `fixture-manifest.json` (§2). | Accept. |
| ASK-2 | The bound values of §3, all copied from the D-72 ruling and its addendum; canvas sizes checked by exact equality. | Accept. |
| ASK-3 | The visual tokens and the halo rule's form (§4): the same region, tolerance, "newly qualifying" test, floor of 4 and 3:1 pair check as the first profile; the pixels move from a centred diamond to a predicted band outside the element's outline. | Accept the form now. Its constants are frozen with the profile (§7). |
| ASK-4 | Whether the product draws a 1 px casing in `canvas.bg` outside the 2 px halo (§4.5). This is a design matter, not the lane's to decide. | Decide after C2's probe reports how many of the 200 samples per fixture have ground beside the halo. The rule is the same either way. |
| ASK-5 | Whether the halo is drawn over nearer geometry (§4.6). Also a design matter; it needs a stencil buffer on the persistent renderer, which is a foundation change to justify with C2. | Drawn over, for the user first and for the oracle second. |
| ASK-6 | What happens to `viewportSelectionPresentation.ts` when C2 removes the diamond (§5.2): (A) the file stays on disk, byte-identical and unreferenced, or (B) a frozen preimage under `{UIF}/fixtures/` as commit `683cd5ab2` did for the geometry source, which edits first-profile files. | (A) for this lane. (B) belongs with ASK-9 as separately scoped instrument maintenance. |
| ASK-7 | Reference host and browser (§3.3): the second profile states its own reference-profile record with D-72 item 2's host and display, and reuses the first profile's Chromium binding by import while recording the version in its own descriptor. | Accept, unless the provisioned Chromium on the reference host has moved. |
| ASK-8 | Who owns the cross-lane bindings (§6): the redesigned shell's pane selectors, the "filter to presented rows" control, the view and stage switches, the per-point reset, and one additive optional field `labels.mode` in the diagnostics type, which lives in the shell lane's `src/features/workspace/uiDiagnostics.ts`. | ROOT assigns. The canvas lane writes the profile; the shell lane supplies the test ids and the field. |
| ASK-9 | Four defects already present in the first profile (§8). | Name them now, fix none in this lane. The second profile depends on none of them. |
| ASK-10 | That the brief's write-scope sentence "`apps/desktop/e2e/ui-foundation/**` only as C4 states" covers creating `{D72}` with the halo witness in it, and when (§7). | Yes, written in C4's slot, frozen after the lane's last slice that touches a bound file. |
| ASK-11 | On acceptance of ASK-1 to ASK-3: slice C1b moves the two held constants (scene background, selected colour and the cue's rim) to `canvas.bg` and `canvas.selection`, and the lane returns the two-line `styles.css` change. From that commit the first profile's freeze no longer describes the lane's product, by design (§7). | Accept. |

## 1. Basis

| Record | SHA-256 or revision |
|---|---|
| Lane base | `4dcab750501d039af5b2973991cbe0f3f423b80a` |
| `D-72_RULING_2026-09-18.md` | `76f2eff773d75b2ee1d9facc1f0dcf8196635e5f520d0dbf7abc66b09063e62b` |
| `D-72_RULING_ADDENDUM_2026-09-18.md` | `e44fb50e244c614689c8ed98f0db87ac65f8fa899f0aee90d2a499d22e49e2cd` |
| D-72 packet, as it stands with its appended ruling sections | `609d044ece07d98d52d1032142a0511a87e78cecdf3ad3d3830596ab7d9bfe3c` |
| `{UIF}/fixture-manifest.json` (first profile) | `6e7fba8fdba11853ad7aa558c7e7c29ffb1a82e79c11a1b7633827335f458739` |
| `{DESKTOP}/src/design/tokens.json` (tokens 1.2) | `00c1afe97365973e6c05a2a5d938a25c46022a55c3cbc8d04ff3a9d2d68cf1e9` |
| `src/features/viewport/viewportSelectionPresentation.ts` at lane base, equal to the first profile's `CUE_SOURCE_SHA256` | `00384d2831797e5cba8acff21e82e36f21a853c398e8c2a801ccfed0f3a55931` |
| `src/features/viewport/viewportSelection.ts` at lane base | `fdf3eaa49b9685b932bce404421086c45a82fa22e5c8abc3f8146fc09cb846d5` |
| `{UIF}/fixtures/frozen-oracle-geometry.ts.txt`, equal to the first profile's `CUE_GEOMETRY_SOURCE_SHA256` | `c0c09ebdb05a49565bf61e576ff0b391037916c614f5add8f05f4270539f5f8e` |
| Read-only inventory `../returns/INV1-INSTRUMENT_RETURN.md` (Claude Sonnet 5) | `c2279008809c16f032ff51b337cd70f45d833f3404c9f564e59c4aef8e25cc1b` |

The inventory states its own limits: about 30 of roughly 60 hash literals untraced; three large spec files and one verifier sampled and not read whole. Every line cited below as "checked" was re-read by the manager at the lane base. Lines cited as "INV1" were not. One inventory error was found this way and is corrected in §8, item 3.

## 2. Form: a sibling profile beside the first

### 2.1 What the first profile pins, and where (checked)

- Window 1440 × 920 and device pixel ratio 2 are literals, not parameters: `playwright.performance.config.ts:29-30` (the base of most configs), `playwright.resource-lifecycle-source.config.ts:54-55`, `characterization-commands.ts` `validateBoundaryMetadata` (`p.windowWidth !== 1440 || p.windowHeight !== 920`, `p.browserDpr !== 2`), `characterization-mode.ts` `validateReferenceProfile` (`JSON.stringify(record.viewport) !== "[1440,920]"`, host `Apple M5 Max`, `128 * 1024 ** 3` bytes, 60 or 120 Hz). INV1 lists five further sites (`benchmark-harness.ts` near 2027, `full-cohort-controller.ts` near 526, two baseline driver literals, `load-availability-diagnostic.mjs:54`). D-72's 1440 × 900, 603 × 828, 1000 × 828, 138 and 230 appear nowhere under `{UIF}`.
- The same validator requires exactly two panes with the selectors `.workspace-pane-tree` and `.workspace-pane-inspector`, both visible, and pins the label policy by phase-id prefix: `point-selection-*` requires labels off, `orbit-*` requires labels on. The cap 80 is `selected_label_cap_candidate` in `fixture-manifest.json`.
- `fixture-manifest.json` is bound by its own hash: drivers take it from `UI_FOUNDATION_MANIFEST_SHA256`, the README states `6e7fba8f…` three times, and the frozen evidence of the recorded demonstration cites that value. A key added to the manifest changes the hash and cuts the file on disk loose from every record that cites it.
- The limits are `PERFORMANCE_TARGETS_MS` at `performance-targets.ts:64-65` (assignment 2000, point 100, box 200, filter 200, centreline 16.7, real outside diameter 33.3). The browser is `REQUIRED_CHROMIUM_RUNTIME_BINDING` at `candidate-server-response.ts:27-31` (153.0.8010.36).

So the addendum's phrase "beside the first in `fixture-manifest.json`" cannot be met inside that file without breaking item 1 of the same addendum. The second profile is therefore new bound values in new files.

### 2.2 The proposed directory

`{D72}/` holds everything the second profile adds. Planned contents:

| File | Purpose |
|---|---|
| `profile.json` | The descriptor: every bound value of §3, the first manifest's hash, the shared fixtures and samples by path and hash, the basis hashes of §1, a `status` of `DRAFT_UNFROZEN` until the freeze. The timed runner refuses a draft. |
| `visual-tokens.json` | §4.1. In the repository, unlike the first profile's external `VISUAL_TOKENS_V4.json`; its hash is in the descriptor. |
| `display-primitives.json` | §4.3: the declared display primitive for each entity kind and geometry mode, stated independently of the product's code, as the point-hit policy is today. |
| `boundary.ts` | The second profile's boundary validator (§3.2). A new function. `validateBoundaryMetadata` is not edited and not called. |
| `halo-witness.ts` | §4: band prediction and the paired witness. Imports `pngPixelDigest` from `../../benchmark-harness.ts`, which INV1 reports as profile-agnostic; nothing else of the first cue. |
| `commands.ts`, `control-binding.json` | Phase ids and control bindings for the redesigned shell (§6). `characterization-commands.ts` and `candidate-control-binding-v1.json` are not edited. |
| `run-plan.ts` | S-1 (§3.4). Imports `PERFORMANCE_TARGETS_MS`; defines no limit of its own. |
| `freeze-d72-point-oracle.mjs` | §5.1. Imports `point-hit-oracle.mjs` and `box-selection-oracle.mjs` unchanged. |
| `playwright.d72-*.config.ts`, `d72-*.benchmark.ts` | Drivers and configs at 1440 × 900. Named `*.benchmark.ts`, which neither ordinary Playwright lane matches. |
| `d72-profile.spec.ts` | Hermetic checks (§5.3), discovered by the source lane. |
| `first-profile-seal.json` | §2.3. |
| `README.md` | How to run, in the form of the first profile's README. |

Reused unchanged, by import or by hash: the two fixtures and their 200 + 20 + 20 samples, the V3 point-hit oracle, the box-selection oracle and their policy hashes (`bc0cb199…`, `8195cd97…`), `pngPixelDigest`, the limits, the Chromium binding, the compositor-trace and causal-presentation method. Schema ids stay in the existing `openpipestress.ui-foundation.*` and `ui-foundation.*` namespaces; the identity layer is Tranche A2's.

### 2.3 A seal on the first profile

`first-profile-seal.json` lists every file under `{UIF}` outside `profiles/`, with its SHA-256 at the lane base, and `d72-profile.spec.ts` recomputes the list. An accidental edit to the first profile then fails the ordinary source lane and names the file. A deliberate edit (ASK-6 option B, or a repair under ASK-9) regenerates the seal in the same change, so it is visible in review. If ROOT would rather not couple the two, the seal can be a script that ROOT's reviewer runs instead of a spec.

## 3. Bound values

### 3.1 Table

| Quantity | Value | Source |
|---|---|---|
| Window | 1440 × 900 CSS px | D-72 item 3 |
| Device pixel ratio, theme, density (gate) | 2, Light, Comfortable | item 2 |
| Observations that do not gate, one run per size each | external display in its 60 Hz mode; device pixel ratio 1; Dark | item 2; addendum effect 3 |
| Canvas, Both view | 603 × 828 CSS px, beside the 737-wide table with the agent strip | item 3 |
| Canvas, Model view | 1000 × 828 CSS px, drawer closed | item 3 |
| Label cap, Both / Model | 138 / 230, and the rule `floor(width × height / 3600)`; the descriptor holds both and the spec checks that they agree (499,284 / 3600 = 138.69; 828,000 / 3600 = 230) | item 3 |
| Label population by phase | Off, 0 rendered: point selection, box selection, filter. Budget: assignment, orbit | item 3 |
| Boundary record at every segment | canvas CSS size, drawing-buffer size, device pixel ratio, label mode, rendered label count beside the cap | item 3; D-70 effect 1 |
| Limits | the six values of `PERFORMANCE_TARGETS_MS`, imported; the spec checks that they equal the first manifest's `acceptance_targets_ms` | item 1 |
| Core workload per run | one assignment, 200 point selections, 20 box selections, 20 filters, two orbit windows (centreline and real outside diameter) of 2 s warm-up and 10 s measurement | item 4 |
| Filter segment | named "filter to presented rows"; the substitution for the tree filter is declared in the method before the freeze | item 4 |
| Redesigned-only workloads | row selection to canvas highlight and canvas pick to row selection, p95 ≤ 100 ms; view switch and stage switch, p95 ≤ 200 ms to presented; split drag and drawer resize, p95 ≤ 33.3 ms; orbit with result colour, edge lines and labels at Budget, the item 1 orbit limits | item 4 |
| Run plan | S-1 (§3.4) | addendum effect 1 |
| Baseline | none collected; the recorded demonstration is the before-picture | addendum effect 2 (S-2) |

Two of the redesigned-only workloads cannot run on this tranche's product. "Orbit with result colour, edge lines and labels at Budget" needs result colouring, which the brief lists as absent (a gap outside the tranche), and an edge line, which no slice of the brief draws (proposal P2). The descriptor declares them and marks each `NOT_RUNNABLE` with the reason; it does not drop them.

### 3.2 Boundary validation

Exact equality, no tolerance: window 1440 × 900; canvas 603 × 828 in the `both` configuration and 1000 × 828 in the `model` configuration; `browserDpr` 2 on gated runs and 1 on the declared observation; drawing buffer within one device pixel of `floor(css × dpr)` as today. If the implemented shell yields another canvas size at 1440 × 900 (a border, a scrollbar), the validator rejects the run and the numbers come back to ROOT. The lane adjusts neither the profile nor the canvas to fit.

Labels: in Off phases `labels.mode === "off"` and `renderedCount === 0`; in Budget phases `labels.mode === "budget"`, `labels.budget` equals the configuration's cap, and `renderedCount <= budget`, with the count published. `labels.mode` is the additive field of ASK-8. The first profile's validator reads only `enabled`, `renderedCount` and `budget` (checked), so the added field does not disturb it.

Geometry mode, camera, model identity and generation, theme and density, drift between boundaries: as the first validator checks them. Panes: the redesigned shell's selectors, fixed jointly with the shell lane (§6).

### 3.3 Reference host and browser

D-72 item 2 rules the host (Apple M5 Max, 128 GiB), the internal display in its 120 Hz mode as the gated reference, a pinned Chromium bound per run and a production build. The second profile states its own reference-profile record, because the first's validator rejects any viewport but `[1440,920]`; host model, memory and the internal-display conditions are the same values. For the browser it imports `REQUIRED_CHROMIUM_RUNTIME_BINDING` and also records `153.0.8010.36` in `profile.json`, so that the two profiles use one browser and a later change to the imported constant cannot move the second profile unnoticed. If the provisioned Chromium on the reference host has moved, the second profile states its own binding before any run and the first stays as it is.

### 3.4 Run plan (S-1)

One run per fixture size (1,000 and 10,000 pipes) per canvas configuration on the internal display: four gated runs. A run whose every gated quantity is at or under 80 % of its limit settles its configuration. If any lies above 80 % and at or under 100 %, that configuration runs twice more and all three must meet every limit. Any gated quantity over its limit is a failure. An invalid run is repeated and retained. Six observation runs (external display, device pixel ratio 1, Dark; one per size each) are published and do not gate. The first profile's ten-run plan, `plan.length === 10` and the ten-run scorer are neither edited nor reused. The picking regression tests pass first.

## 4. The visual cue for a halo

### 4.1 Tokens (both themes)

| | Light | Dark | Token |
|---|---|---|---|
| Selection | `#106dce` = [16, 109, 206] | `#6dadff` = [109, 173, 255] | `color.canvas.selection` |
| Ground | `#ebedef` = [235, 237, 239] | `#191c1f` = [25, 28, 31] | `color.canvas.bg` |
| Hover, used only as a negative control | `#5d94da` = [93, 148, 218] | `#5788c7` = [87, 136, 199] | `color.canvas.hover` |

These are today's values of the product token file at the hash in §1. The freeze binds the token file's hash as it then is and asserts that `visual-tokens.json` equals it. Equality is asserted at the freeze, not in the ordinary lanes, so that a token slice in the shell lane is never blocked by a file it cannot edit.

Read-back: `--canvas-selection` and `--canvas-bg` from the computed style of `.app-shell`, with `data-theme` as today. The generated properties are defined on `:root[data-theme]` (`src/tokens.css`) and inherit. The first profile's read-back of `--ui-viewport-selection-geometry` and `--ui-canvas` (`candidate-camera-preflight.benchmark.ts:103-114`) is left alone.

The two-line stylesheet change that goes to ROOT with C1b: `--ui-canvas: var(--canvas-bg);` and `--ui-viewport-selection-geometry: var(--canvas-selection);`, each once, replacing the per-theme literals.

### 4.2 What stays the same as the first profile

The region is the same 48 × 48 CSS px clip, `clip.x = floor(centerCss.x) − 24` and likewise for y, centred on the independently projected authored centre of the expected winner; the PNG is `48 × dpr` square and opaque. The tolerance is the same ±48 per channel. A pixel counts only when it qualifies after the pick and did not before, and the image hash must have changed. The floor is the same 4. The local check is the same 3:1 between the two members of a pair. Before and after are captured with the camera unchanged.

### 4.3 Which pixels

The first cue is a diamond at the winner's centre, so its pixels sit at fixed offsets. A halo sits outside the element's outline, so its pixels depend on the element's drawn size. The oracle predicts them, from the authored model and the read-back camera only:

1. `display-primitives.json` declares, for each entity kind and geometry mode, the primitive the product draws (a pipe at real outside diameter: a cylinder between its end nodes of radius OD / 2; and so on for the other kinds and for the centreline mode). It is a declared policy with its own hash, written from the design and the product's documented behaviour and never generated from product code.
2. The outline `O(W)` is the boundary of the convex hull of the primitive's projected sample points (32 points on each end circle of a cylinder; the tangent-cone circle of a sphere; the vertices of a cone or box).
3. The halo band `B(W)` is the set of device pixels whose centre lies between 1 and 3 CSS px outside `O(W)`. A 2 px halo spans 0 to 2 px. Starting the band at 1 px keeps a recoloured body from qualifying under a 1 px prediction error; ending it at 3 px allows the same error outward.
4. The exclusive band `B*(W)` is `B(W)` less the bands of every other entity that is not hidden. A wrong winner lights its own band, and none of `B*(W)`. This matters here as it did not for the diamond: at the fitted camera a median pipe of these fixtures is 6 to 10 px long, so the 48 px region holds several neighbours, among them the shared-endpoint neighbours of PR #794.

A wrong `display-primitives.json` predicts a band where no halo is, and the witness fails. It cannot produce a pass.

### 4.4 Which colour, what count, what local check

- `Q` is the set of pixels of `B*(W)` that newly qualify as Selection (§4.1, ±48).
- The casing band `C(W)` is the set of pixels between 2 and 4 CSS px outside `O(W)`. A pair is a pixel of `Q` and a pixel of `C(W)` within 2 CSS px of it, the second within ±48 of Ground after the pick, the two at a contrast of at least 3:1, each casing pixel used once and chosen in a fixed order as `winnerCuePairs` does.
- Pass: the image changed and at least 4 pairs newly qualify.
- Published and never gated: `|B*(W)|`, `|Q|`, the pair count, the number of newly qualifying pixels more than 3 px inside `O(W)` (a halo leaves the element's own colour alone, so this should be 0), and the net growth of Selection-coloured pixels, as `netColorGrowthDiagnosticOnly` is today.
- Negative controls in `d72-profile.spec.ts`, on synthetic images: a halo on `W` passes; a halo on a shared-endpoint neighbour fails; a recoloured body with no halo fails; a halo in the hover colour fails; an unchanged image fails; a non-opaque PNG fails.

Colours within ±48 of Selection that could enter the region, computed from the token file: `canvas.draft` (the same value; the profile's state precondition is that no routing draft is open), `canvas.axisZ` (the gizmo; unchanged between before and after), `cat.4` in both themes and `cat.6` in light (no canvas object takes them today: load arrows take `cat.1` and `cat.2` only), and `focus.ring`. The last is a real hazard: a pick gives the canvas keyboard focus, and a focus ring that appears between before and after is newly qualifying. The capture protocol therefore takes "before" with the canvas already focused, and the band rule keeps a ring at the canvas border out of `B*(W)` except for an element at the border. In dark, hover differs from Selection by 56 on blue alone; C2's rule that a selected element shows no hover halo keeps that margin from mattering.

### 4.5 The local check and the casing (ASK-4)

The design's own finding is that the halo reads against the ground: 4.36:1 light, 7.39:1 dark. Against a tube it is 2.21:1 light and 1.85:1 dark; against `canvas.pipeShade` 1.41:1 and 3.06:1; against the edge line 1.81:1 and 1.42:1 (computed from the token file). In these fixtures most halos will lie over other tubes. The pair rule of §4.4 holds wherever ground is beside the halo and fails where it is not. A 1 px casing in `canvas.bg` outside the halo makes it hold everywhere, and is invisible on open ground.

That is a change to the adopted presentation, so it is ROOT's and the design's. The lane offers two facts. Where a halo lies over a tube, the state indicator is under 3:1 against what it is drawn on, which bears on WCAG 2.2 SC 1.4.11 for the canvas. And C2's probe can report, before the decision, how many of the 200 samples per fixture pass the pair rule with no casing. If ROOT declines the casing and that number is under 200, the lane brings the numbers back. It does not lower 3:1 and does not drop the pair check.

### 4.6 Drawn over nearer geometry, or depth-tested (ASK-5)

The first cue is drawn with `depthTest: false`. In the centreline mode of today's product the node spheres (radius 0.095 m) swallow the pipes between them (radius 0.052 m, median length 0.1 m), so a depth-tested halo on a pipe would be hidden from the user as well as from the oracle. The lane recommends a halo drawn over nearer geometry and masked by the element's own outline, which needs a stencil buffer: three 0.181.2 creates none by default and the renderer is built with `{ antialias: true }` only. That is a change to the persistent renderer and comes back to ROOT, justified, with C2. The rule of §4.3 to §4.4 is the same either way: an occluded band pixel does not qualify.

### 4.7 What the rule needs of C2

The halo is unlit and not tone-mapped, in the token's exact sRGB value, 2 CSS px wide at any device pixel ratio. It is display-only and is never an input to picking, which is analytic over the typed model index. A selected element shows no hover halo. Between two captures nothing else in the region changes.

## 5. What the oracle script binds, and the ordinary lanes

### 5.1 Bindings of `freeze-d72-point-oracle.mjs`

The first manifest's hash, and the fixture and sample bytes by hash; the point-hit and box policies and their oracle modules by hash, unchanged; a camera preflight per canvas configuration; `visual-tokens.json`, `display-primitives.json` and the product token file by hash; the cue source, which is the halo's source files at their hashes at the freeze; the geometry source, which is `viewportSelection.ts` at its hash at the freeze.

Addendum item 4 asks why the geometry source moves. There are two reasons, and neither is a change to picking. The lane base already differs from the first profile's bound preimage (`fdf3eaa4…` against `c0c09ebd…`), because PR #794 repaired picking after that preimage was frozen. And `viewportSelection.ts` also holds code that C3, C4 and C5 must change: `isolateSelectionVisibility`, `prioritizedLabelKeys` with its `Math.min(80, …)`, and `fittedViewportDistance`. The lane does not change `pointPickPrimitives`, `pickPointPrimitive`, `boxSelectEntityKeys`, their helpers, their tolerances or `viewportSelection.test.ts`. With the freeze the lane returns a declaration-level diff of the file between the lane base and the freeze commit, so the reviewer can see which top-level declarations changed.

The executed-source record, per file with SHA-256, is taken at the freeze from the production build's module graph by an instrument-side build, with a named floor that must appear in it. Canvas lane: `PipeViewport.tsx`, `viewportResource.ts`, `viewportSelection.ts`, the C1 palette and figure-material modules, the C2 halo files, the C4 label files, the C5 files, and `src/features/workspace/modelIndex.ts`. Shell lane, listed at ROOT's word: `src/App.tsx`, `src/styles.css`, `src/tokens.css`, `src/design/tokens.json`, and under `src/features/workspace/` the files `solveGates.ts`, `solveProof.ts`, `solveJobAudit.ts`, `projectPersistenceIntegrity.ts`, `workspaceSections.ts`, `menuCommands.ts`, `sessionModel.ts`, the six `*SessionState.ts` hooks, `workspaceSession.ts` when it exists, `uiDiagnostics.ts` and `statusLabels.ts`.

### 5.2 A live binding in the ordinary source lane (ASK-6)

`full-cohort-controller.spec.ts:585` and `:588` read `src/features/viewport/viewportSelectionPresentation.ts` from disk and expect `CUE_SOURCE_SHA256`. The geometry source beside it is read from `fixtures/frozen-oracle-geometry.ts.txt` (commit `683cd5ab2`), which is why PR #794 did not break this test. So an edit to the cue file, or its removal, fails the ordinary source lane and CI. The consequence for C2: the halo is built in new files and `viewportSelectionPresentation.ts` stays byte-identical. When the diamond leaves the scene (the design shows none; a named semantic change of C2), the file either stays unreferenced as the first profile's preimage (A) or moves to a frozen preimage as the geometry did (B). (B) edits a first-profile spec, adds a fixture and changes the method inventory's length, which touches §8 item 3; it is not this lane's to do. Until ROOT decides, C2 does (A), which needs no decision: it edits nothing of the first profile.

### 5.3 Both lanes stay green

The source lane (`testDir: "./e2e"`, default match, `**/*-dist.spec.ts` ignored) discovers the four specs under `{UIF}`: `causal-method-contract`, `full-cohort-controller`, `fresh-demo-policy`, `performance-targets`. The dist lane matches `**/*-dist.spec.ts` only and runs none of them. INV1 reports that the DEC-025 sweep drives only the npm lanes. The second profile edits none of the four. Its own spec is hermetic: no environment input, no page, no network, no timing, no external frozen file, no silent skip. No other new file ends in `.spec.ts` or `.test.ts`. Both lanes run through the lock with `PLAYWRIGHT_WORKERS=1` before the slice that adds `{D72}` is returned.

## 6. Bindings that belong to both lanes (ASK-8)

The first profile drives `toggle-viewport-labels`, `viewport-box-select`, `viewport-selection-filter`, `viewport-fit-model`, `viewport-view-isometric`, `viewport-geometry-schematic`, `viewport-geometry-actual-od`, `open-local-project`, reads `command-selection-readout` and `property-inspector`, and resets each point through a tree row (INV1). In the redesigned product the label control becomes All / Budget / Off (C4), the HUD becomes one group of ten (C5), the tree filter becomes a table filter, and the panes, view switch and stage switch are the shell lane's. `{D72}/control-binding.json` names the redesigned controls. The canvas lane supplies the canvas and HUD test ids; the shell lane supplies the rest. It is written last, against the merged product. The first binding file is not edited, and the root-level specs `e2e/ui-foundation.spec.ts` and `e2e/ui-foundation-dist.spec.ts`, which drive `toggle-viewport-labels` with `aria-pressed`, are outside this lane's write scope: C4 will return what it needs of them.

## 7. Order, and why the cue changes

The cue changes because the owner's adopted design changes how selection is presented (G-31: a halo over the element's own colour in place of a recolour). It does not change to obtain a result. The form of the rule is put to ROOT before the halo exists and before any timing of it exists. Its every number is either the first profile's (48 px region, ±48, floor 4, 3:1) or the design's (2 px halo, the token values). It is reviewed as instrument code by ROOT's independent reviewer. The first profile, its tokens, its oracle script and its recorded results are not edited and remain the before-picture.

1. ROOT decides ASK-1 to ASK-3 and ASK-11.
2. C1b moves the two held constants to tokens. From that commit the first profile's freeze would stop at `freeze-candidate-point-oracle.mjs:73-74`, where the read-back colour must equal the external tokens' `#a34400` or `#f08c22`, and no profile can freeze a visual oracle for the lane's product until step 5. The ordinary lanes are unaffected.
3. C2 builds the halo in new files behind a probe and reports the sample counts that ASK-4 and ASK-5 turn on.
4. `{D72}` is written in C4's slot with `status: DRAFT_UNFROZEN`, and returned to ROOT as a slice.
5. After the last slice that touches a bound file (C5, as planned), the descriptor, tokens, primitives, witness and plans are frozen by hash, with builds kept clear of timed runs. The timed runs are ROOT's, by a separate runner.

## 8. Defects already present in the first profile (named, not fixed)

1. The first profile's geometry binding no longer matches the product: `CUE_GEOMETRY_SOURCE_SHA256` (`benchmark-harness.ts:264`) is `c0c09ebd…`, and `viewportSelection.ts` at the lane base is `fdf3eaa4…`, a consequence of PR #794. The ordinary spec passes because it reads the frozen preimage. A timed run against a real source root would stop with "winner product geometry/cue source drift". Checked.
2. `verify-winner-cue-plan.mjs:12` holds `97b18c96…`, a geometry hash older than the one the harness binds, and asserts it at line 31. `full-cohort-controller.spec.ts:567` treats that same value as one the harness must reject. Checked.
3. The method inventory is required to have exactly 34 files at `characterization-observations.mjs:196` and `:278`, while `requiredMethodFiles` at `full-cohort-controller.ts:331` has 38 entries, counted by script. **Correction to the inventory:** INV1 placed the two checks in `resource-accounting.ts`; the line numbers are right and the file is `characterization-observations.mjs`. The retained return is verbatim and is not edited. Checked.
4. `generate-fixtures.mjs:663` reads from a `protocol-history/` directory that does not exist under `{UIF}`. Checked.

The second profile calls none of these paths. It binds the existing fixture bytes by hash and never regenerates them, carries its own plan verifier, and derives its method inventory's length from the list itself.

## 9. What this proposal does not settle

- How C2 draws the halo; that is C2's probe. §4.7 is what the rule needs of it.
- The casing (ASK-4) and drawing over nearer geometry (ASK-5); both are the design's.
- The redesigned shell's selectors and controls (ASK-8).
- Whether 200 of 200 samples remain actionable at the two new canvas sizes under the fitted camera. The freeze reports it per sample. A shortfall comes back to ROOT with the numbers; samples are not edited.
- Item 6 of D-72 leaves large-selection halos unfrozen; this proposal adds no number for them.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
