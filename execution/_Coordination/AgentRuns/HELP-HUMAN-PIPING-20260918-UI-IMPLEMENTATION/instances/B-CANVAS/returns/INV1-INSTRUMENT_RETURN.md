# INV1-INSTRUMENT — Read-only inventory of pinned first-profile values in the UI-foundation benchmark instrument

**Who I am:** Claude Sonnet 5 (model id `claude-sonnet-5`), running as a read-only Claude Code sub-agent.

**Brief hash verification:** I ran `shasum -a 256` on the sealed brief at `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/B-CANVAS/briefs/INV1-INSTRUMENT.md` (path relative to `{WORKING_ROOT}`) both earlier in this task and again just now as a final check. Both times it produced `4322fc25107c1c289be6670e1a880a610e1ef04b646fb38c0b580f3e269885fe`, an exact match to the value given at launch. I read the brief whole and followed it.

**What I wrote/ran:** Nothing state-changing. Every action was `Read`, `git show HEAD:./<path>` (used for everything under `src/features/viewport/**` and for `src/features/workspace/uiDiagnostics.ts`, per the brief), `ls`, `find`, `grep`, `shasum`, and one `python3 -c` invocation that only parsed JSON already on disk and printed it (no file written; `python3 -m json.tool`-style pretty-printing used earlier for the same reason). I launched no agent and worked alone throughout.

**What I read whole:** `README.md`; both D-72 ruling records; `fixture-manifest.json`; `candidate-control-binding-v1.json`; `freeze-candidate-point-oracle.mjs`; `verify-winner-cue-plan.mjs`; `verify-fixtures.mjs`; `benchmark-harness.ts` (2878 lines, sequential full read); `point-hit-oracle.mjs`; `box-selection-oracle.mjs`; `generate-fixtures.mjs`; `performance-targets.ts`; `characterization-mode.ts`; `characterization-commands.ts`; `full-cohort-controller.ts` (911 lines); `causal-method-contract.ts`; `causal-phase-journal.ts`; `chromium-compositor-trace.ts`; `candidate-server-response.ts`; `characterization-observations.mjs` (455 lines) and its `.d.mts`; `resource-accounting.ts`; `causal-presentation-extractor.mjs` (663 lines) and its `.d.mts`; `lossless-trace-json.mjs`/`.d.mts`; `fresh-demo-policy.mjs`/`.d.mts`; `tsconfig.causal-method.json`; `observer-contract-binding-v1.md`; `functional-qualification-contract-v1.json`; `capture-bundle-manifest.mjs`; `capture-external-candidate-bundle-manifest.mjs`; `serve-bound-candidate-output.mjs`; `load-availability-diagnostic.mjs`; `aggregate-results.mjs`; `verify-baseline-point-actionability.mjs`; `verify-box-selection-oracle.mjs`; `verify-point-hit-oracle.mjs`; `run-baseline-session-supervised.mjs`; `resource-lifecycle-page.tsx`/`.html`; `candidate-resource-qualification.benchmark.ts`; all 12 `playwright.*.config.ts` files; `candidate-camera-preflight.benchmark.ts`; `candidate-visual-point-preflight.benchmark.ts`; `compositor-trace-preflight.benchmark.ts`; `baseline-canvas-point.benchmark.ts`; `ui-foundation-performance.benchmark.ts`; `resource-lifecycle-source.benchmark.ts`; `characterization-observations.d.mts`; `verify-characterization-observations.mjs`; product files `src/features/workspace/uiDiagnostics.ts`, `src/features/viewport/viewportSelection.ts`, `src/features/viewport/viewportSelectionPresentation.ts` (read via `git show HEAD:./<path>`); `apps/desktop/playwright.config.ts` and `playwright.dist.config.ts` (via `git show HEAD:./<path>`); `apps/desktop/package.json` and root `package.json` scripts (via `git show`).

**What I read in part (targeted `grep`, not full read):** `causal-presentation-preflight.benchmark.ts` (837 lines / 51 KB — grepped for constants/env/testids/assertions); `causal-method-contract.spec.ts` (1133 lines); `full-cohort-controller.spec.ts` (1397 lines, plus an earlier full read of lines 540–629 for the stale-hash negative test and lines ~1060–1340 for `D70_*` usage); `performance-targets.spec.ts` (379 lines); `verify-causal-presentation-extractor.mjs` (632 lines, not opened — only referenced by name/import elsewhere); `viewportResource.ts` (grepped for exports, not read whole); `tools/release/run_evidence_sweep.py` (1325 lines, grepped for surface/script names). Bulk data under `fixtures/` and `samples/` was not read except for file kind/size/hash accounting already present in `fixture-manifest.json` and `verify-fixtures.mjs`.

All paths below are relative to `{DESKTOP}` (`apps/desktop`) unless marked otherwise.

---

## 1. Window / viewport / canvas / DPR / ROI sizes

| # | Value | File : line | Enclosing function/constant | Exported | Reused by / entry points | Profile-agnostic? |
|---|---|---|---|---|---|---|
| 1 | `viewport:{width:1440,height:920}`, `deviceScaleFactor:2` | `e2e/ui-foundation/playwright.performance.config.ts:29-30` | module-level `use` block (base config) | default export | Extended by `playwright.baseline-points.config.ts`, `playwright.candidate-*.config.ts` (via `candidateBase` which itself extends this), `playwright.causal-presentation-preflight.config.ts`, `playwright.compositor-preflight.config.ts` | No — literal in config body |
| 2 | `viewport:{width:1440,height:920}`, `deviceScaleFactor:2` | `e2e/ui-foundation/playwright.resource-lifecycle-source.config.ts:54-55` | module-level `use` block (standalone config, does not extend #1) | default export | `resource-lifecycle-source.benchmark.ts` | No |
| 3 | Baseline-only 96×96 ROI clamp hard-coding `1440`/`920` in clip math | `e2e/ui-foundation/benchmark-harness.ts:~2027-2032` | `measureCanvasPointSelection` (exported) | Yes (function), but literal is inline in body | `baseline-canvas-point.benchmark.ts`, `ui-foundation-performance.benchmark.ts` (baseline branch), `causal-presentation-preflight.benchmark.ts` (baseline branch) | **No** — the only branch of this exported function that a second profile could not reuse unchanged |
| 4 | `actualViewport.width!==1440 \|\| height!==920 \|\| deviceScaleFactor!==2` → throws | `e2e/ui-foundation/full-cohort-controller.ts:~526` | `runCandidateCausalPerformance` (exported async) | Yes | `ui-foundation-performance.benchmark.ts` (candidate branch, via `executeCollectionRun`) | No — hard gate in function body |
| 5 | `viewport==="[1440,920]"` string compare | `e2e/ui-foundation/characterization-mode.ts:~45-54` | `validateReferenceProfile` (exported) | Yes | `ui-foundation-performance.benchmark.ts` (imported, not directly called there but part of the reference-profile chain used by `characterization-observations.mjs`) | No |
| 6 | `windowWidth===1440`, `windowHeight===920`, `dpr<=2` | `e2e/ui-foundation/characterization-commands.ts:47-77` | `validateBoundaryMetadata` (exported) | Yes | `full-cohort-controller.ts` boundary capture path | No |
| 7 | `viewportCssPixels:[1440,920]`, `deviceScaleFactor:2` (data literal, environment record) | `e2e/ui-foundation/baseline-canvas-point.benchmark.ts:45-46` | test body (module scope, not a function) | N/A (script) | `playwright.baseline-points.config.ts` | No |
| 8 | `viewportCssPixels:[1440,920]`, `deviceScaleFactor:2` | `e2e/ui-foundation/ui-foundation-performance.benchmark.ts:150-151` | test body (baseline branch) | N/A | `playwright.performance.config.ts` | No |
| 9 | `page.goto`/`newPage({viewport:{width:1440,height:920},deviceScaleFactor:2})` | `e2e/ui-foundation/load-availability-diagnostic.mjs:54` | top-level script | N/A (standalone CLI) | invoked directly, not via Playwright config | No |
| 10 | `benchmark.viewport.css_pixels:[1440,920]`, `device_pixel_ratio_cap:2` | `e2e/ui-foundation/fixture-manifest.json` (data) | — | — | read by `ui-foundation-performance.benchmark.ts:358` (`manifest.benchmark.acceptance_targets_ms`), `verify-fixtures.mjs` | data, not code |
| 11 | `BASELINE_CANVAS_CSS=[919,628]` | `e2e/ui-foundation/generate-fixtures.mjs` (top const) | module const | No | source of `fixture-manifest.json`'s `baseline_canvas_css` | No |
| 12 | `baseline_canvas_css:[919,628]` | `e2e/ui-foundation/fixture-manifest.json` (data) | — | — | mirrors #11 | data |
| 13 | Candidate ROI clip fixed at `48×48` px (CSS), computed from live camera projection, not a hardcoded viewport size | `e2e/ui-foundation/freeze-candidate-point-oracle.mjs:138` (`clip.width/height=48`) | top-level `for` loop in `main` script | N/A | consumed by `benchmark-harness.ts` `measureCanvasPointSelection` (candidate branch, `probe.visual_plan?.clip`) and `causal-presentation-preflight.benchmark.ts:274` | **Yes** — this path is camera/canvas-driven, not a hardcoded window size |
| 14 | `coordinateToleranceCssPx = 0.01` (point) | `e2e/ui-foundation/benchmark-harness.ts:~2106` | `measureCanvasPointSelection` | Yes | all point-selection entry points | Yes (constant, not size-derived) |
| 15 | `coordinateToleranceCssPx = 1` (box) | `e2e/ui-foundation/benchmark-harness.ts` (box-selection block, ~2286-2647) | `measureCanvasBoxSelection` | Yes | all box-selection entry points | Yes |
| 16 | Second-profile target sizes (window 1440×900; canvases 603×828 "Both view" / 1000×828 "Model view") | D-72 ruling + addendum (`../../execution/_Coordination/_DECISIONS/D-72_RULING_2026-09-18.md` item 3; addendum) | — | — | **Confirmed absent from instrument code** — `grep -R "900\|603\|1000×828\|1000,828"` for these as size literals returned zero hits anywhere under `e2e/ui-foundation` (excluding bulk fixture data) | informational only |
| 17 | Functional-qualification appearance matrix, a **different**, unrelated 3-viewport list `[[1024,768],[1280,800],[1440,920]]` | `e2e/ui-foundation/functional-qualification-contract-v1.json:17` (`appearance_matrix.viewports_css`) | data | — | this is the *ordinary* accessibility/appearance qualification contract, not the performance instrument; do not conflate with #1-#9 | data |

## 2. Label state / cap / budget / phase policy

| # | Value | File : line | Function/constant | Exported | Entry points | Notes |
|---|---|---|---|---|---|---|
| 1 | `selected_label_cap_candidate: 80` | `e2e/ui-foundation/fixture-manifest.json` (data) | — | — | read wherever the manifest is loaded | data |
| 2 | Product-published shape `labels:{enabled,renderedCount,budget}` | `src/features/workspace/uiDiagnostics.ts` (`UiDiagnosticsSnapshot.viewport.labels`) | product type | exported type | validated by instrument (see Q.C) | product source, read via `git show HEAD:` |
| 3 | Phase label policy: `id.startsWith("point-selection-")⇒labels.enabled===false`; `id.startsWith("orbit-")⇒labels.enabled===true` | `e2e/ui-foundation/characterization-commands.ts:74-75` | `validateBoundaryMetadata` | Yes | `full-cohort-controller.ts` | No — hard-codes phase-name prefixes |
| 4 | Labels forced OFF before point sampling: `ensureViewportToggle(page,"toggle-viewport-labels",false,…)` | `candidate-camera-preflight.benchmark.ts:101`; `candidate-visual-point-preflight.benchmark.ts:48,74` (toggled off then back on); `baseline-canvas-point.benchmark.ts:98,188`; `ui-foundation-performance.benchmark.ts:232,270` | test bodies, call into `ensureViewportToggle` (`benchmark-harness.ts`, exported, profile-agnostic) | function: Yes | listed files | `ensureViewportToggle` itself is size/profile-agnostic; the *call sites* choose true/false |
| 5 | Labels forced ON for orbit-adjacent preflight: `ensureViewportToggle(…,"toggle-viewport-labels",true,…)` | `compositor-trace-preflight.benchmark.ts:56` | test body | — | — | matches policy row 3 |
| 6 | Second-profile label caps Off=138 / Budget=230 | D-72 ruling item 3 | — | — | **Confirmed absent from code** — `grep -R "138\|230"` (excluding fixtures/samples) returns zero hits as size/cap literals | informational only |
| 7 | Control binding for the Labels toggle: role `button`, name `"Labels"`, testid `toggle-viewport-labels`, state `aria-pressed` | `e2e/ui-foundation/candidate-control-binding-v1.json` (`controls.labels`) | data | — | drives every `ensureViewportToggle` call | data |

## 3. Shell layout selectors, panes, and product controls

### 3a. Shell/pane/panel selectors

| # | Selector | File : line | Function | Exported |
|---|---|---|---|---|
| 1 | `.app-shell`, `data-theme`, `data-density` | `characterization-commands.ts` `captureBoundary` (90-109); also read at `candidate-camera-preflight.benchmark.ts:60,104,109` | `captureBoundary` | Yes |
| 2 | `.workspace-pane-tree`, `.workspace-pane-inspector` — must equal exactly this 2-element array, in order | `characterization-commands.ts:47-77` | `validateBoundaryMetadata` | Yes |
| 3 | `.panel.model-tree`, `.panel.inspector` | `characterization-commands.ts` `captureBoundary` | same | Yes |
| 4 | `.workspace-pane-inspector .panel.inspector > h2`, `.panel.model-tree` | `benchmark-harness.ts:~lines within installInstrumentation, 492-1437` | `installInstrumentation` | Yes (whole function) |
| 5 | `panels.length===2`, `theme∈["light","dark"]`, `density∈["comfortable","compact"]` | `characterization-commands.ts:47-77` | `validateBoundaryMetadata` | Yes |
| 6 | `[data-testid="model-tree-filter-input/-virtual/-summary/-empty"]` | `benchmark-harness.ts` (`installInstrumentation` region) | same | Yes |

### 3b. Every literal test id, one row each, with every file that references it (from an exhaustive `grep` census; excludes bulk fixtures/samples)

| Test id | Occurrences | Files |
|---|---|---|
| `command-selection-readout` | 20 | `benchmark-harness.ts`, `candidate-visual-point-preflight.benchmark.ts`, `causal-method-contract.spec.ts`, `causal-presentation-preflight.benchmark.ts`, `generate-fixtures.mjs` |
| `viewport-canvas` | 10 | `benchmark-harness.ts`, `candidate-camera-preflight.benchmark.ts`, `causal-method-contract.spec.ts`, `causal-method-contract.ts`, `causal-presentation-preflight.benchmark.ts`, `characterization-commands.ts`, `compositor-trace-preflight.benchmark.ts`, `full-cohort-controller.spec.ts`, `full-cohort-controller.ts`, `load-availability-diagnostic.mjs`, `resource-lifecycle-source.benchmark.ts`, `verify-causal-presentation-extractor.mjs` |
| `model-tree-filter-input` | 10 | `benchmark-harness.ts`, `causal-method-contract.spec.ts`, `characterization-commands.ts`, `full-cohort-controller.ts` |
| `aggregate-property-inspector` | 10 | `benchmark-harness.ts`, `causal-method-contract.spec.ts` |
| `model-tree-virtual` | 8 | `benchmark-harness.ts`, `causal-method-contract.spec.ts`, `causal-presentation-preflight.benchmark.ts`, `characterization-commands.ts`, `full-cohort-controller.ts` |
| `model-tree-filter-summary` | 7 | `benchmark-harness.ts`, `candidate-camera-preflight.benchmark.ts`, `load-availability-diagnostic.mjs` |
| `viewport-view-isometric` | 6 | `candidate-camera-preflight.benchmark.ts`, `candidate-visual-point-preflight.benchmark.ts`, `causal-presentation-preflight.benchmark.ts`, `characterization-commands.ts`, `compositor-trace-preflight.benchmark.ts`, `generate-fixtures.mjs` |
| `viewport-fit-model` | 6 | same file set as above |
| `viewport-view-cube` | 4 | `baseline-canvas-point.benchmark.ts`, `causal-presentation-preflight.benchmark.ts`, `compositor-trace-preflight.benchmark.ts`, `ui-foundation-performance.benchmark.ts` (baseline-only camera control — nested `getByRole("button",{name:"Iso"})`) |
| `viewport-selection-filter` | 4 | `benchmark-harness.ts`, `causal-presentation-preflight.benchmark.ts`, `characterization-commands.ts`, `full-cohort-controller.ts`, `generate-fixtures.mjs` |
| `property-inspector` | 2 | `benchmark-harness.ts`, `causal-method-contract.spec.ts`, `generate-fixtures.mjs` |
| `local-project-message` | 2 | `resource-accounting.ts` |
| `command-bar` | 2 | `candidate-visual-point-preflight.benchmark.ts` (`.locator(".command-context > summary")`) |
| `open-local-project` | 1 | `generate-fixtures.mjs`, `resource-accounting.ts` |
| `model-tree-filter-empty` | 1 | `benchmark-harness.ts` |
| `clear-model-tree-filter` | 1 | `benchmark-harness.ts` |
| `viewport-box-select` | (via role/testid pair, not counted in literal-string grep) | `benchmark-harness.ts`, `characterization-commands.ts`, `full-cohort-controller.ts`, `generate-fixtures.mjs` |
| `viewport-geometry-schematic` | — | `characterization-commands.ts`, `full-cohort-controller.ts`, `generate-fixtures.mjs` |
| `viewport-geometry-actual-od` | — | `characterization-commands.ts`, `full-cohort-controller.ts`, `generate-fixtures.mjs` |
| `` tree-row-${encodeURIComponent(type)}-${encodeURIComponent(id)} `` (candidate, dynamic) | — | `benchmark-harness.ts` exported `treeRowTestId(ref,phase)`; used by `causal-presentation-preflight.benchmark.ts`, `full-cohort-controller.ts` |
| `` tree-row-${id} `` (baseline, dynamic, different pattern) | — | same `treeRowTestId`, phase-branched |
| `` tree-row-${fixture.project.id} `` (plain, 3rd distinct variant) | — | `load-availability-diagnostic.mjs:97` only |
| `` viewport-select-${entityId} `` (dynamic, accessible-label control) | — | `benchmark-harness.ts` `measureAccessibleLabelSelection` |

### 3c. `candidate-control-binding-v1.json` — full contract (read in full)

| Control key | role | name | testid | state |
|---|---|---|---|---|
| `labels` | button | Labels | `toggle-viewport-labels` | `aria-pressed` |
| `boxSelect` | button | Box Select | `viewport-box-select` | `aria-pressed` |
| `selectionFilter` | combobox | Selection filter | `viewport-selection-filter` | options `["All","Pipes","Nodes","Supports","Components"]` |
| `fitModel` | button | Fit Model | `viewport-fit-model` | — |
| `isometric` | button | Isometric | `viewport-view-isometric` | — |
| `schematic` | button | Schematic | `viewport-geometry-schematic` | `aria-pressed` |
| `actualOd` | button | Actual OD | `viewport-geometry-actual-od` | `aria-pressed` |
| `openLocal` | button | Open local | `open-local-project` | — |
| `feedback.selectionSummary` | status | Selection summary | `command-selection-readout` | — |
| `feedback.propertyInspector` | region | Property inspector | `property-inspector` | — |
| `feedback.typedTreeRow` | — | — | `` tree-row-${encodeURIComponent(type)}-${encodeURIComponent(id)} `` | — |
| `sourceOnlyResourceReader` | module `src/features/viewport/viewportResource.ts`, export `currentOwnedViewportResourceSnapshot` | — | — | `readOnly:true`, `currentOnly:true` |

File-level fields: `schema:"openpipestress.ui-foundation.candidate-control-binding/v1"`, `status:"FROZEN_PROSPECTIVE_PENDING_PRODUCT_SOURCE_INTEGRATION"`, `governedBindingSha256:"12ea9947ff0251713b733e980f8ea7e5c536069f35a86791d82cd6300bebfbaa"`. I confirmed by `git show HEAD:` that `src/features/viewport/viewportResource.ts:146` really does `export function currentOwnedViewportResourceSnapshot(): OwnedViewportResourceSnapshot | null` — the binding is real, not aspirational, and it is actually called (not just referenced) at `e2e/ui-foundation/resource-lifecycle-page.tsx:14-17`.

## 4. Selection visual cue

| # | Value | File : line | Function/constant | Exported | Phase |
|---|---|---|---|---|---|
| 1 | `cssSize:11, interior:.34, outer:.5, erosionDevice:1` | `benchmark-harness.ts:~265-276` (`validateWinnerCuePlan`) and `freeze-candidate-point-oracle.mjs:145` | both | Yes / N/A(script) | candidate |
| 2 | `clip:{width:48,height:48}` | same two sites | same | — | candidate |
| 3 | `rimSrgb`: light `[255,255,255]`, dark `[0,0,0]` | `freeze-candidate-point-oracle.mjs:145` | plan literal | — | candidate |
| 4 | Theme input srgb: light `[163,68,0]`, dark `[240,140,34]` | `freeze-candidate-point-oracle.mjs:74` (asserted against `visualTokens`) and `benchmark-harness.ts:~277-295 winnerCuePairs` | `winnerCuePairs`/freeze script | Yes/N/A | candidate |
| 5 | Scene srgb: light `[223,229,232]`, dark `[12,17,20]` | `freeze-candidate-point-oracle.mjs:170` | freeze script | N/A | candidate |
| 6 | `span=11*dpr`, `size=48*dpr` (throws if `>768`) | `benchmark-harness.ts:277-295` | `winnerCuePairs` | Yes | candidate |
| 7 | Erosion boundaries `.34*span±1` / `.5*span±1`, `≥4 pairs` required | `benchmark-harness.ts:277-295` | `winnerCuePairs` | Yes | candidate |
| 8 | `minimumQualifyingInteriorPixels=4`, `requiredLocalContrastRatio=3`, `tolerancePerChannel=48` | `benchmark-harness.ts:296-319` | `pairedWinnerCueWitness` | Yes | candidate — these three numbers are **hard-coded independently here**; the *same three numbers* are also carried, as data, inside the external `VISUAL_TOKENS_V4.json` (`viewportSelectionCue.renderedPixelOracle.*`), from which `freeze-candidate-point-oracle.mjs:171-173` copies them into each frozen probe's `selection_visual_oracle` for provenance — the runtime witness does not read that copy back, it re-checks its own literals |
| 9 | `--ui-viewport-selection-geometry` CSS var read | `candidate-camera-preflight.benchmark.ts:111` | inline `page.evaluate` in test body | N/A | candidate — **the only place in the instrument this variable is read** (confirmed by exhaustive grep) |
| 10 | `--ui-canvas` CSS var read | `candidate-camera-preflight.benchmark.ts:112` | same | N/A | candidate — also the only occurrence |
| 11 | `preflight.visualStyleReadback.viewportSelectionGeometry.toLowerCase()===selectedThemeTokens.viewportSelectionGeometry.toLowerCase()` | `freeze-candidate-point-oracle.mjs:73` | freeze script | N/A | consumes row 9's output (see Q.D) |
| 12 | `actualCssSelectionToken: preflight.visualStyleReadback.viewportSelectionGeometry` re-embedded, provenance only | `freeze-candidate-point-oracle.mjs:174` | freeze script | N/A | candidate |
| 13 | Baseline cue: `source:"unchanged baseline PipeViewport selected material 0xf08c22"`, `theme:"baseline light"`, `srgb:[240,140,34]`, `tolerancePerChannel:80`, `minimumQualifyingInteriorPixels:1` | `baseline-canvas-point.benchmark.ts:131-136` | inline object literal in test body | N/A | baseline |
| 14 | Same baseline literal, independently re-inlined | `ui-foundation-performance.benchmark.ts:244-250` | inline in test body | N/A | baseline — confirms this cue block is duplicated, not shared, across the two baseline driver files |
| 15 | `0xf08c22` / `[240,140,34]` product selection colour | referenced as prose/data throughout; canonical source is the baseline literal above | — | — | baseline |
| 16 | Schema `winner-cue-pair/v1` | `freeze-candidate-point-oracle.mjs:142` | plan object | — | candidate |
| 17 | Result status `PASS_PAIRED_WINNER_CUE_TRANSITION` | `benchmark-harness.ts:296-319` `pairedWinnerCueWitness` | — | Yes | candidate |
| 18 | External schema `piping-ui-visual-tokens/v4` | `freeze-candidate-point-oracle.mjs:46` (asserted) | — | — | the tokens file itself, `VISUAL_TOKENS_V4.json`, is **outside this repository** — only referenced by an absolute CLI-arg path (`--visual-tokens`) and pinned by hash |
| 19 | `pngPixelDigest`: `centralRadius=min(w,h)*0.25` | `benchmark-harness.ts:118-260` | `pngPixelDigest` | Yes | both phases |
| 20 | Contrast formula `(max+0.05)/(min+0.05)`, sRGB gamma constants `0.04045/12.92/0.055/1.055/2.4`, Rec.709 weights `0.2126/0.7152/0.0722` | `benchmark-harness.ts:118-260` | `pngPixelDigest` | Yes | both |
| 21 | PNG signature `[137,80,78,71,13,10,26,10]`, CRC32 poly `0xedb88320` | `benchmark-harness.ts:118-260` | `pngPixelDigest` | Yes | both — profile-agnostic decode machinery |
| 22 | `winnerCuePairs`/`pairedWinnerCueWitness` require both a "before" and "after" PNG clipped to the candidate's frozen 48×48 ROI | `causal-presentation-preflight.benchmark.ts:295-347` (`captureWinnerCue`, `pairedWinnerCueWitness` call) | test body | — | candidate |
| 23 | Baseline minimum qualifying pixels differs from candidate's (1 vs 4) — this is the concrete origin of the brief's "minimum counts (1, 4)" | rows 8 and 13 above | — | — | — |
| 24 | `48` per-channel tolerance (candidate) vs `80` (baseline) — origin of the brief's "tolerances (48 per channel)" plus the baseline's distinct, looser `80` | rows 8 and 13 | — | — | — |

## 5. Hashes bound in code or prose

| # | Named constant | Value (first 12 hex shown) | Binds | File:line (defines) | Bytes location |
|---|---|---|---|---|---|
| 1 | `POLICY_SHA256` / `point_hit_policy_sha256` | `bc0cb1994...` | point-hit selection policy | `freeze-candidate-point-oracle.mjs:10`, `ui-foundation-performance.benchmark.ts:10`, `fixture-manifest.json` | in-repo (policy is the oracle logic itself, not a separate file — the hash is a fixed constant, not computed from a byte source in what I read) |
| 2 | `BOX_POLICY_SHA256` / `box_selection_protocol.policy_sha256` | `8195cd971...` | box-selection policy | same files | as above |
| 3 | `VISUAL_TOKENS_SHA256` | `009b27db8...` | `VISUAL_TOKENS_V4.json` bytes | `freeze-candidate-point-oracle.mjs:12`, `ui-foundation-performance.benchmark.ts:12`, `candidate-visual-point-preflight.benchmark.ts:53` | **outside this repository** — path only given via `--visual-tokens` CLI arg |
| 4 | `CUE_SOURCE_SHA256` / `cueSourceSha256` | `00384d283...` | `src/features/viewport/viewportSelectionPresentation.ts` | `benchmark-harness.ts:262` (exported const, authoritative), `freeze-candidate-point-oracle.mjs:36` | in-repo. **I re-hashed the current HEAD copy of that product file myself and it matches exactly** (`00384d283...`) |
| 5 | `CUE_GEOMETRY_SOURCE_SHA256` / `geometrySourceSha256` | `c0c09ebdb...` | `src/features/viewport/viewportSelection.ts` | `benchmark-harness.ts:264` (exported const, authoritative — confirmed at `validateWinnerCuePlan`, line ~271), `freeze-candidate-point-oracle.mjs:39` | in-repo. **I re-hashed the current HEAD copy myself and it does NOT match** — HEAD's `viewportSelection.ts` currently hashes to `fdf3eaa49b9685b932bce404421086c45a82fa22e5c8abc3f8146fc09cb846d5`, not `c0c09ebdb...`. This is a live, freshly-observed fact as of this read; another agent is concurrently editing this exact file, so it may change again before this report is used. |
| 6 | Stale/superseded geometry hash used as a negative-test fixture elsewhere, but checked as if authoritative by `verify-winner-cue-plan.mjs` | `97b18c967...` | (deliberately wrong/old declaration) | `verify-winner-cue-plan.mjs:12,31`; the same literal is used intentionally as the "old-declaration" value in `full-cohort-controller.spec.ts:540-629`'s negative test | **Inconsistency**: `verify-winner-cue-plan.mjs` is a real, standalone offline CLI comparator whose own `geometryHash` constant is the *known-superseded* value, not the current `c0c09ebdb...` that `benchmark-harness.ts`/`freeze-candidate-point-oracle.mjs` actually use |
| 7 | `UI_FOUNDATION_MANIFEST_SHA256` / self-hash of `fixture-manifest.json` | `6e7fba8fd...` | `fixture-manifest.json` bytes | stated in `README.md`; enforced by `candidate-server-response.ts` `bindCandidateDriverEntry` via env | in-repo — **I hashed the file myself and it matches exactly** |
| 8 | `governed_attachment_sha256` | `ef009d111...` | `OBSERVABILITY_ATTACHMENT_V1.md` (the product's read-only attachment doc) | `fixture-manifest.json`; also stated as the pinned value in `observer-contract-binding-v1.md:6-7` | governed UI run evidence — described as held outside this instrument tree |
| 9 | `verifier_binding_sha256` | `8ab87d0d0...` | verifier's own binding record | `fixture-manifest.json` | in-repo pin, target file not identified with certainty in what I read |
| 10 | `governed_binding_sha256` (control binding) | `12ea9947f...` | `candidate-control-binding-v1.json` itself (self-referential integrity field inside the file) | `fixture-manifest.json`, `candidate-control-binding-v1.json` | in-repo |
| 11 | `INTERNAL120_AUTHORITY_SHA256` | `6df272b7a...` | `OWNER_DIRECTION_INTERNAL120_20260917.md` | `characterization-observations.mjs:131` (exported const) | described as living at `../../OWNER_DIRECTION_INTERNAL120_20260917.md` relative to the seed file — outside `e2e/ui-foundation` |
| 12 | `ONE_SUCCESS_AUTHORITY_SHA256` | `ba5e8bcee...` | `OWNER_DIRECTION_ONE_SUCCESS_20260917.md` | `characterization-observations.mjs:130` (module-scope, not exported) | outside `e2e/ui-foundation`, same pattern as #11 |
| 13 | `ORIGINAL_RETURN_SHA256` | `b9cd6955f...` | the original seed `RETURN.json` for the continuation/attempt-budget ledger | `characterization-observations.mjs:120` (exported const) | outside this directory (evidence-root relative) |
| 14 | Focused-mode box-sample expected hash | `1b1eced5f...` | one specific box-selection sample's expected result, used only in "focused" 8-segment mode | `full-cohort-controller.ts` (`focusedBoxSamples` region, ~box samples [1,7]) | in-repo, embedded literal |
| 15 | `WORKSPACE_TASK_ACCEPTANCE_V1.md` governed source hash | `43bcf3a72...` | ordinary functional-qualification governing doc | `functional-qualification-contract-v1.json:6-7` | outside `e2e/ui-foundation` (`instances/ROOT/...`) |
| 16 | `REQUIRED_CHROMIUM_RUNTIME_BINDING` (`browserVersion:"153.0.8010.36"`, `product`, `revision:"@507c6ee3e2f3b2ca0e660547e5b9ea4820c67f4c"`) | not a hash but a pinned identity triple | exact Chromium build | `candidate-server-response.ts:27-31` (exported const) | consumed by nearly every candidate `.benchmark.ts`/config |
| 17 | `SAME_TRACE_EXPORT_PROFILE.exporterSha256` | `dc3a3b53c...` | the trace exporter binary/script | `performance-targets.ts:19-25` | not located as an in-repo file in what I read |
| 18 | `SAME_TRACE_EXPORT_PROFILE.traceTimeSha256` | `6a5fa3cf6...` | trace-time conversion reference | `performance-targets.ts:19-25` | not located |
| 19 | `PAGE_CLOCK_SOURCE.timeClamperHeaderSha256` | `f8065edb3...` | Chromium `TimeClamper` header source | `causal-presentation-extractor.mjs:13-21` (exported const) | external Chromium source, not in this repo |
| 20 | `PAGE_CLOCK_SOURCE.timeClamperImplementationSha256` | `2491cb64f...` | same | same | external |
| 21 | `PAGE_CLOCK_SOURCE.performanceImplementationSha256` | `ff28d2604...` | `performance.now()` implementation source | same | external |
| 22 | `REQUIRED_CHROMIUM_BINDING.primarySourceManifestSha256` | `f87fd6532...` | Chromium source manifest | `causal-presentation-extractor.mjs:2-9` (module const, not exported directly — only via `REQUIRED_CHROMIUM_BINDING` re-export at line 663) | external |
| 23 | `REQUIRED_CHROMIUM_BINDING.diagnosisSha256` / `.diagnosisOutputManifestSha256` / `.methodBriefSha256` | `151020569...` / `dce9bc5a1...` / `bc21adf30...` | supporting diagnosis records for the Chromium binding | same | not located in this repo tree |
| 24 | `sha256` of `causal-presentation-extractor.mjs`+`point-hit-oracle.mjs`+`box-selection-oracle.mjs` bundled as `dependencies[]` | computed at run time, not a literal | those three files' current bytes | `freeze-candidate-point-oracle.mjs:41` | in-repo, self-hashing |
| 25 | `verifierDriverFiles` hash-bound trio | computed at run time | `playwright.resource-lifecycle-source.config.ts`, `resource-accounting.ts`, `resource-lifecycle-source.benchmark.ts` | `resource-lifecycle-source.benchmark.ts:151-155` | in-repo, self-hashing (see Q.B) |
| 26 | `candidateExecutedFiles` (product side) hash-bound quintuple | computed at run time | `src/App.tsx`, `src/features/viewport/PipeViewport.tsx`, `src/features/viewport/viewportResource.ts`, `resource-lifecycle-page.html`, `.tsx` | `resource-lifecycle-source.benchmark.ts:143-149` | product source, read from the *candidate* source root, not this repo's HEAD |
| 27 | `governedBindingSha256` inside `candidate-control-binding-v1.json` | `12ea9947f...` | same file (self-integrity), duplicate of row 10 | `candidate-control-binding-v1.json` | in-repo |
| 28 | `oldMethod.files.length!==34 \|\| newMethod.files.length!==34` (a **second**, code-level "34" requirement, independent of the README's prose claim) | `resource-accounting.ts:196` and again at `resource-accounting.ts:278` (`method.files?.length!==34`) | `oneSuccessHistory`, `verifyContinuationFiles` | both exported functions | This directly conflicts with `full-cohort-controller.ts`'s `requiredMethodFiles` array, which I counted programmatically (Node `eval`) at **38** entries, not 34 — so there are now *two* independent places asserting "34" (README prose + these two `resource-accounting.ts` checks) that disagree with the actual 38-entry array in `full-cohort-controller.ts` |
| — | ~60 distinct 64-hex-character strings total found by exhaustive grep census (excluding fixtures/samples) | — | — | — | Only the ~28 rows above were traced to a named constant and a stated meaning within the time available; the remainder (roughly 30 literals, mostly appearing once each) are cited in `README.md` prose or embedded in various `*.mjs` scripts (`capture-*-manifest.mjs`, `verify-characterization-observations.mjs`, etc.) without my having traced each one to its exact binding target — see "could not determine" below |

## 6. Run plan / cohort shape

| # | Value | File : line | Function/constant | Exported |
|---|---|---|---|---|
| 1 | `FULL_COHORT_RECIPE`: `pointCount:200, boxCount:20, filterCount:20, warmupMs:2000, measuredMs:10000, trailingMs:250, maximumFeedbackMarkers:4096, activeObserverSampleCap:16000, globalObserverSampleCap:24000, domMutationCap:4096` | `full-cohort-controller.ts:24-29` | `FULL_COHORT_RECIPE` | Yes |
| 2 | `UI_FOUNDATION_RUNS` default `"1,2,3,4,5"` | `baseline-canvas-point.benchmark.ts:23`, `ui-foundation-performance.benchmark.ts:40` | module scope | N/A |
| 3 | `UI_FOUNDATION_PIPE_COUNTS` default `"1000,10000"` | `ui-foundation-performance.benchmark.ts:39` | module scope | N/A |
| 4 | `validateFixedSelection`: `JSON.stringify(counts)==="[1000,10000]"`, `JSON.stringify(runs)==="[1,2,3,4,5]"` | `characterization-mode.ts:17-20` | `validateFixedSelection` | Yes |
| 5 | `assignmentOnly` requires the same two exact JSON strings a 3rd time | `ui-foundation-performance.benchmark.ts:61` | module scope | N/A |
| 6 | `runCollectionDisposition`: `segmentCount===243` | `characterization-mode.ts:21-26` | `runCollectionDisposition` | Yes |
| 7 | `attemptDisposition`: `plan.length===10 && records.length===10` | `characterization-mode.ts` | `attemptDisposition` | Yes |
| 8 | `scorePerformanceCohort` hard-requires exactly 10 runs = `[1000,10000]×[1,2,3,4,5]` | `performance-targets.ts:203-234` | `scorePerformanceCohort` | Yes |
| 9 | `CONTINUATION_SLOTS` = 9 named slots `["1000.2".."1000.5","10000.1".."10000.5"]` | `resource-accounting.ts:119` | exported const | Yes |
| 10 | `seed.counts.attempted===1, invalidFailed===1, unattempted===9`, `collection.runs.length===10` | `resource-accounting.ts:221-228` | `validateContinuationPolicy` | Yes |
| 11 | Reference profile: `hostModel:"Apple M5 Max"`, `memoryBytes===137438953472`, `refreshHz:60` (or `120` for internal120), `viewport:"[1440,920]"`, `browserDpr:2`, `effectiveDprCap:2` | `characterization-mode.ts:45-54` `validateReferenceProfile` | Yes |
| 12 | internal120 successor profile: `refreshHz:120`, display `name:"Color LCD"`, `connection:"spdisplays_internal"`, `mirror:"spdisplays_off"`, resolution regex `/@ 120\.00Hz$/` | `resource-accounting.ts:132-144` | `validateDisplayTransition` | Yes |
| 13 | Collection modes: `"characterization"` vs default/qualification | `characterization-mode.ts` `collectionMode()` | Yes | consumed by `ui-foundation-performance.benchmark.ts:46` |
| 14 | `fresh-demo-policy`: `fixtureSize===10000, runNumber===1, refreshHz===120, purpose∈["timed","smoke"]` | `fresh-demo-policy.mjs:16-22` | `validateFreshDemoPolicy` | Yes |
| 15 | "Focused" causal proof: exactly **8 segments**, point sample fixed at `2`, box samples `[1,7]` | `full-cohort-controller.ts` (focused-mode region); `causal-presentation-preflight.benchmark.ts:66-67` (`UI_FOUNDATION_CAUSAL_POINT_SAMPLE` default `"2"`) | — | — |
| 16 | Orbit gesture window for the focused proof: `12250ms` total (`2000+10000+250`) | `full-cohort-controller.ts` | same region | — |
| 17 | Reduced preflight orbit durations: `orbitWarmupMs=500, orbitMeasuredMs=2000, orbitTrailingMs=250` (distinct from the full 2000/10000/250) | `causal-presentation-preflight.benchmark.ts:69-72` | module const | N/A |
| 18 | Compositor preflight uses even smaller windows: `measureOrbit(page,0,500)` warmup, `measureOrbit(page,0,2000)` measured | `compositor-trace-preflight.benchmark.ts:103,105` | test body | — |
| 19 | `resource_replacement_cycles: 20` | `fixture-manifest.json` (data); enforced at `resource-accounting.ts:114` (`if(cycles!==20)throw`) in `exerciseRealOpenReplacementCycles` | Yes |
| 20 | 5 mount/unmount cycles for the source actual-App lifecycle witness | `resource-lifecycle-source.benchmark.ts:46` (`for cycle=1;cycle<=5`) | test body | — |
| 21 | Second-profile run plan S-1 (1 run per fixture×canvas config, escalate to 3 only if any gated quantity is `>80%` and `≤100%` of its limit) | D-72 addendum item 1 | — | — | **Confirmed absent from code** |
| 22 | `historicalSlots=["1000.2","1000.3","1000.4"]`, exactly 3 historical claims required (`t.history.length!==3`) | `resource-accounting.ts:157,198` | `oneSuccessHistory` | not exported (module scope) |

## 7. The six numeric limits (2000, 100, 200, 200, 16.7, 33.3 ms)

| # | Field | Value | Defined at | Consumed at |
|---|---|---|---|---|
| 1 | `assignment` | 2000 ms | `performance-targets.ts:64-65` `PERFORMANCE_TARGETS_MS` (exported const) | `performance-targets.ts:195` `scorePerformanceRun`; mirrored in `fixture-manifest.json.acceptance_targets_ms.assignment`; asserted in `performance-targets.spec.ts:43` |
| 2 | `pointP95` | 100 ms | same | same three sites (`.pointP95`) |
| 3 | `boxP95` | 200 ms | same | same (`.boxP95`) |
| 4 | `filterP95` | 200 ms | same | same (`.filterP95`) |
| 5 | `centerlineP95` | 16.7 ms | same | same (`.centerlineP95`) |
| 6 | `actualOdP95` | 33.3 ms | same | same (`.actualOdP95`) |
| 7 | `RAW_OBSERVER_REFERENCES` (`p95Ms:0.5, maximumMs:2, totalPercent:3`) — non-gating companion values | `performance-targets.ts:66` (exported) | independently re-declared (not imported) at `causal-method-contract.ts:75` (type) and `:319` (value); asserted in `performance-targets.spec.ts:44` |
| 8 | Warmup/measured orbit window `2000ms`/`10000ms` | `performance-targets.ts:172` (hard check) and `full-cohort-controller.ts:24-29` `FULL_COHORT_RECIPE` | `performance-targets.spec.ts:201-202` (`toBe(2000)`, `toBe(10000)`) |
| 9 | `nearestRankP95 = sorted[Math.ceil(.95*n)-1]` | independently implemented 3×: `benchmark-harness.ts:2844-2848` (exported), `performance-targets.ts:77` (explicit comment: "deliberately not imported from controller arithmetic"), `causal-method-contract.ts:118` (non-exported `percentile95`) | consumed at every run/cohort-scoring call site in each of those three files |
| 10 | `boxFilterLabel` all/pipes/nodes/supports/components → All/Pipes/Nodes/Supports/Components map | independently inlined ≥3×: `benchmark-harness.ts` (non-exported helper), `full-cohort-controller.ts:713`, `characterization-commands.ts` `runCharacterizationSmoke` | consumed wherever a filter sample's label must be displayed |

## 8. Environment variables read (exhaustive grep census, excluding fixtures/samples)

| Env var | Files that read it | Purpose (brief) |
|---|---|---|
| `UI_FOUNDATION_EVIDENCE_DIR` | `benchmark-harness.ts` (`requireEvidenceRoot`), most `.benchmark.ts` files, `resource-accounting.ts`, `characterization-observations.mjs`, `run-baseline-session-supervised.mjs`, `fresh-demo-policy.mjs`, playwright configs | absolute output root for a run; must contain `/instances/VERIFY/` for `requireEvidenceRoot` |
| `UI_FOUNDATION_CANDIDATE_ORACLE_DIR` | `benchmark-harness.ts` (`validateCandidateOracleBinding`), `resource-accounting.ts`, `candidate-camera-preflight.benchmark.ts` (must be *unset*) | absolute dir of the frozen candidate point-oracle output |
| `UI_FOUNDATION_REFERENCE_PROFILE` / `_SHA256` | `characterization-mode.ts`, `resource-accounting.ts`, `characterization-observations.mjs` | hash-bound reference host profile JSON |
| `UI_FOUNDATION_COHORT_ID` | `ui-foundation-performance.benchmark.ts:44-45`, `resource-accounting.ts` | freeform cohort identity, regex `/^[A-Za-z0-9._:-]{1,48}$/` |
| `UI_FOUNDATION_DIAGNOSTIC_MODE` | `ui-foundation-performance.benchmark.ts`, `fresh-demo-policy.mjs` | selects `candidateDiagnosticMode` (e.g. `assignment-collection`) |
| `UI_FOUNDATION_CANDIDATE_SOURCE_STAGE` | `candidate-server-response.ts`, `capture-external-candidate-bundle-manifest.mjs`, `serve-bound-candidate-output.mjs`, `fresh-demo-policy.mjs` | `"preliminary"` or `"final"` candidate provenance stage |
| `UI_FOUNDATION_CANDIDATE_SOURCE_ROOT` | `candidate-server-response.ts`, `resource-lifecycle-source.benchmark.ts`/`.config.ts`, `resource-accounting.ts` | absolute root of the candidate product checkout |
| `UI_FOUNDATION_CANDIDATE_ORACLE_MANIFEST_SHA256` | `benchmark-harness.ts`, `resource-accounting.ts`, `causal-presentation-preflight.benchmark.ts` | expected hash of `CANDIDATE_POINT_ORACLE_MANIFEST.json` |
| `UI_FOUNDATION_RUNS` | `baseline-canvas-point.benchmark.ts`, `ui-foundation-performance.benchmark.ts`, `run-baseline-session-supervised.mjs` | comma list of run numbers, default `1,2,3,4,5` |
| `UI_FOUNDATION_CONTINUATION_CLAIM` / `_SHA256` | `resource-accounting.ts`, `ui-foundation-performance.benchmark.ts` | hash-bound claim file for a continuation slot launch |
| `UI_FOUNDATION_COLLECTION_MODE` | `ui-foundation-performance.benchmark.ts`, `fresh-demo-policy.mjs` | `"characterization"` vs ordinary qualification |
| `UI_FOUNDATION_SMOKE` | `ui-foundation-performance.benchmark.ts`, `fresh-demo-policy.mjs` | must be `"controls"` or unset — untimed smoke path |
| `UI_FOUNDATION_MANIFEST_SHA256` | `candidate-server-response.ts`, `baseline-canvas-point.benchmark.ts`, `ui-foundation-performance.benchmark.ts`, `resource-accounting.ts` | expected hash of `fixture-manifest.json` |
| `UI_FOUNDATION_CONTINUATION_CLAIM_SHA256` | as `_CLAIM` above | — |
| `UI_FOUNDATION_CANDIDATE_BUNDLE_MANIFEST_SHA256` | `candidate-server-response.ts`, `serve-bound-candidate-output.mjs`, `resource-accounting.ts` | expected hash of the external production-bundle manifest |
| `UI_FOUNDATION_PHASE` | `benchmark-harness.ts` (`benchmarkPhase`), `compositor-trace-preflight.benchmark.ts`, `causal-presentation-preflight.benchmark.ts`, `fresh-demo-policy.mjs` | `"baseline"` or `"candidate"` |
| `UI_FOUNDATION_CONTINUATION_TOKEN` | `resource-accounting.ts`, `ui-foundation-performance.benchmark.ts` | one-shot exclusivity token for a claimed slot |
| `UI_FOUNDATION_CANDIDATE_BUNDLE_MANIFEST` | `candidate-server-response.ts`, `serve-bound-candidate-output.mjs`, `resource-accounting.ts` | absolute path to that manifest |
| `UI_FOUNDATION_PREFLIGHT_STAGE_TIMEOUT_MS` | `candidate-camera-preflight.benchmark.ts` (5000-120000, default 60000), `compositor-trace-preflight.benchmark.ts` (5000-60000, default 60000), `causal-presentation-preflight.benchmark.ts` (default 60000) | per-stage timeout, **range differs by file** |
| `UI_FOUNDATION_PIPE_COUNTS` | `ui-foundation-performance.benchmark.ts`, `fresh-demo-policy.mjs` | comma list of fixture sizes, default `1000,10000` |
| `UI_FOUNDATION_METHOD_MANIFEST_SHA256` | `resource-accounting.ts`, `fresh-demo-policy.mjs` | hash of the bound method-file manifest |
| `UI_FOUNDATION_FRESH_DEMO_POLICY_SHA256` | `fresh-demo-policy.mjs` | hash of the fresh-demo policy JSON |
| `UI_FOUNDATION_CONTINUATION_POLICY` / `_SHA256` | `resource-accounting.ts`, `ui-foundation-performance.benchmark.ts` | hash-bound continuation-policy JSON |
| `UI_FOUNDATION_CANDIDATE_OUTPUT_ROOT` | `candidate-server-response.ts`, `serve-bound-candidate-output.mjs` | must equal `${sourceRoot}/apps/desktop/dist` |
| `UI_FOUNDATION_METHOD_MANIFEST_PATH` | `resource-accounting.ts`, `fresh-demo-policy.mjs` | path to the method manifest |
| `UI_FOUNDATION_FEASIBILITY_SAMPLE_COUNT` | `baseline-canvas-point.benchmark.ts`, `run-baseline-session-supervised.mjs` | bounds a feasibility-only subset (1-200) of point samples |
| `UI_FOUNDATION_ACTION_TIMEOUT_MS` | `baseline-canvas-point.benchmark.ts` (default 30000), `ui-foundation-performance.benchmark.ts` (default 180000), `fresh-demo-policy.mjs` | per-action timeout — **default differs by file** |
| `D70_WRITER_ORIGINAL_RETURN` | `full-cohort-controller.spec.ts:1133,1247,1334` | hash-bound original `RETURN.json` used by continuation/historical tests; skipped when absent |
| `UI_FOUNDATION_FRESH_DEMO_POLICY` | `fresh-demo-policy.mjs` | path to the fresh-demo policy JSON |
| `UI_FOUNDATION_CONTINUATION_` (prefix check) | `fresh-demo-policy.mjs:26` | asserts no `UI_FOUNDATION_CONTINUATION_*` var is set during a fresh demo |
| `UI_FOUNDATION_CAUSAL_METHOD_OUTPUT_DIR` | `playwright.causal-method-contract.config.ts` | absolute output dir override |
| `UI_FOUNDATION_CAMERA_RECIPE` | `candidate-camera-preflight.benchmark.ts` | must equal `"isometric_then_fit_model"` |
| `UI_FOUNDATION_BUNDLE_MANIFEST_SHA256` | `baseline-canvas-point.benchmark.ts`, `ui-foundation-performance.benchmark.ts` | baseline-side production-bundle hash record (distinct name from `_CANDIDATE_BUNDLE_MANIFEST_SHA256`) |
| `D70_WRITER_PREPARED_INPUTS` | `full-cohort-controller.spec.ts:1064-1065` | runner-input JSON file for a D-70 writer test; skipped when absent |
| `UI_FOUNDATION_TEARDOWN_GRACE_MS` | `run-baseline-session-supervised.mjs` | 1000-60000, default 10000 — grace period before forced teardown |
| `UI_FOUNDATION_CAUSAL_POINT_SAMPLE` | `causal-presentation-preflight.benchmark.ts` | which of the 200 point samples the focused causal proof exercises, default `2` |
| `UI_FOUNDATION_CAUSAL_PIPE_COUNT` | `causal-presentation-preflight.benchmark.ts` | fixture size for the focused proof, default `1000` (or `10000` candidate-only) |
| `UI_FOUNDATION_CANDIDATE_VALIDATE_ONLY` | `serve-bound-candidate-output.mjs` | `"1"` = validate the bundle and exit without serving |
| `D70_CONTENT_WIDTH_REJECTED` | `full-cohort-controller.spec.ts:1216` | file path used by one specific D-70 negative test |
| `PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH` / `_SHA256` | `candidate-server-response.ts` (`bindRequiredChromiumExecutable`), `load-availability-diagnostic.mjs`, `playwright.compositor-preflight...`/`playwright.performance.config.ts` (existence-only) | pinned Chromium executable, hash-verified where `bindRequiredChromiumExecutable` is used |
| `UI_FOUNDATION_CANDIDATE_BUNDLE_MANIFEST` (dup. name) | listed above | — |
| `PLAYWRIGHT_WORKERS` | `../../apps/desktop/playwright.config.ts`, `playwright.dist.config.ts` | worker-count override for the DEC-025 evidence sweep (product/repo-level, not instrument-owned) |
| `CI` | same two ordinary configs | serial-worker fallback |

## 9. Playwright configuration files (all 12, read in full)

| # | Config file | testMatch / testDir | Viewport | Web server / port | Extends |
|---|---|---|---|---|---|
| 1 | `playwright.performance.config.ts` | `testDir:"."`, `ui-foundation-performance.benchmark.ts` | 1440×920, dpr 2 (base) | `npm run preview -- --port 5176` | — (base config; standalone) |
| 2 | `playwright.baseline-points.config.ts` | `baseline-canvas-point.benchmark.ts` | inherited (1440×920/2) | inherited (5176, `npm run preview`) | `playwright.performance.config.ts`; `timeout:30min` |
| 3 | `playwright.candidate-performance.config.ts` | `ui-foundation-performance.benchmark.ts` | inherited | `serve-bound-candidate-output.mjs` on 5176 | `playwright.performance.config.ts`; adds `bindRequiredChromiumExecutable` launch options, `retries:0`, `repeatEach:1` |
| 4 | `playwright.candidate-preflight.config.ts` | `candidate-camera-preflight.benchmark.ts` | inherited | `serve-bound-candidate-output.mjs` on 5176 | `playwright.performance.config.ts`; `workers:1`, `timeout:10min` |
| 5 | `playwright.candidate-visual-preflight.config.ts` | `candidate-visual-point-preflight.benchmark.ts` | inherited | inherited from candidate base (5176) | `playwright.candidate-performance.config.ts`; `workers:1`, `timeout:10min` |
| 6 | `playwright.candidate-compositor-preflight.config.ts` | `compositor-trace-preflight.benchmark.ts` | inherited | inherited (5176) | `playwright.candidate-performance.config.ts`; `workers:1`, `timeout:10min` |
| 7 | `playwright.candidate-causal-presentation-preflight.config.ts` | `causal-presentation-preflight.benchmark.ts` | inherited | inherited (5176) | `playwright.candidate-performance.config.ts`; `workers:1`, `timeout:5min`, `trace/video/screenshot:off` |
| 8 | `playwright.causal-presentation-preflight.config.ts` | `causal-presentation-preflight.benchmark.ts` (baseline variant, note: same test file as #7 but **not** the candidate base) | inherited | inherited (5176, `npm run preview` via base) | `playwright.performance.config.ts` directly (not candidate); `workers:1`, `timeout:5min` |
| 9 | `playwright.candidate-resource.config.ts` | `candidate-resource-qualification.benchmark.ts` | inherited | inherited (5176) | `playwright.candidate-performance.config.ts`; `timeout:20min` |
| 10 | `playwright.compositor-preflight.config.ts` | `compositor-trace-preflight.benchmark.ts` (baseline variant) | inherited | inherited (5176, preview) | `playwright.performance.config.ts` directly; `timeout:10min` |
| 11 | `playwright.causal-method-contract.config.ts` | `testDir:"."`, `causal-method-contract.spec.ts` | not set (no `use.viewport`) | none (no `webServer` block) | standalone; `workers:1`, `timeout:30s`; binds `bindRequiredChromiumExecutable` |
| 12 | `playwright.resource-lifecycle-source.config.ts` | `testDir:"."`, `resource-lifecycle-source.benchmark.ts` | **1440×920, dpr 2 (independently declared, not inherited)** | `npm run dev -- --port 5177` (**different port**) | standalone; hash-verifies `resource-lifecycle-page.html/.tsx` are byte-identical between this repo and the candidate source root |

---

## Question A — `fixture-manifest.json` production/verification; ordinary lanes touching instrument files

`fixture-manifest.json` is produced by `generate-fixtures.mjs` (`main()`, lines ~651-665 write the manifest after computing per-fixture counts, camera recipes, and all the policy/binding hashes listed in category 5). **Before it writes anything**, `generate-fixtures.mjs:651-665` reads and hash-checks 9 "preservedHistory" files from a `protocol-history/` subdirectory. I confirmed via `ls`, `git log --oneline -- protocol-history`, and `git ls-files` that **`protocol-history/` does not exist anywhere in the repository** (untracked, not gitignored, simply absent) — so `node generate-fixtures.mjs` as currently written would fail with `ENOENT` before producing a manifest. It is verified by `verify-fixtures.mjs` (read in full): re-hashes every `files[]` entry, checks per-fixture node/pipe/support/component/nodal-arrow counts, sample counts (200/20/20), and the precision-regression translation pair.

Adding a key to `fixture-manifest.json` would change its own SHA-256, which is pinned as `UI_FOUNDATION_MANIFEST_SHA256` in `README.md` and enforced by `candidate-server-response.ts`'s `bindCandidateDriverEntry` (reads the env var and compares) — so yes, any edit to the manifest breaks that binding until the pinned env value is updated to match.

Ordinary lanes: `../../package.json` defines `test:desktop`→`npm test --workspace apps/desktop` (= `vitest run`, product unit tests — I found no evidence it touches this Playwright-based instrument); `test:e2e:desktop`→`apps/desktop`'s `test:e2e` = `build:wasm && playwright test`, which uses `playwright.config.ts` (`testDir:"./e2e"`, `testIgnore:["**/*-dist.spec.ts"]`, **no other `testMatch` restriction**); `test:e2e:dist:desktop`→`test:e2e:dist` uses `playwright.dist.config.ts` (`testMatch:["**/*-dist.spec.ts"]`). Because Playwright's default file-discovery pattern matches any `*.spec.ts`/`*.test.ts` under `testDir`, and neither `causal-method-contract.spec.ts`, `full-cohort-controller.spec.ts`, `fresh-demo-policy.spec.ts`, nor `performance-targets.spec.ts` is named `*-dist.spec.ts`, **the ordinary `test:e2e:desktop` lane's file-discovery rules include all four of those `.spec.ts` files**, running them under the ordinary config's own project matrix (`chromium-desktop` 1440×920, `chromium-compact` 1280×800, `baseURL:5174`) rather than each file's own dedicated `playwright.*.config.ts`. None of the `*.benchmark.ts` files match Playwright's default discovery pattern, so they never run under either ordinary config — only via their own named configs, invoked directly. `../../tools/release/run_evidence_sweep.py` (grepped, not read whole) contains zero literal references to "ui-foundation"; it drives exactly `npm run test:desktop`, `test:e2e:desktop`, and `test:e2e:dist:desktop` as its `desktop_playwright_e2e` and vitest surfaces (lines 202, 226-227), so its only path to this instrument is the file-discovery overlap just described.

## Question B — instrument files hash-bound by another instrument file or recorded result

- `verifierDriverFiles` (`resource-lifecycle-source.benchmark.ts:151-155`): `playwright.resource-lifecycle-source.config.ts`, `resource-accounting.ts`, `resource-lifecycle-source.benchmark.ts` itself — hashed via `bindFiles()` and written into `actual-app-lifecycle.json`.
- `dependencies[]` in `freeze-candidate-point-oracle.mjs:41`: `freeze-candidate-point-oracle.mjs`, `point-hit-oracle.mjs`, `box-selection-oracle.mjs` — self-hashed and embedded in `CANDIDATE_POINT_ORACLE_MANIFEST.json`.
- `requiredMethodFiles` (`full-cohort-controller.ts`, exported const array): 38 entries (programmatically counted), covering nearly every `.ts`/`.mjs`/`.json`/`.md` instrument file — this is the broadest hash-binding list in the instrument.
- `resource-accounting.ts:196,278` independently requires exactly **34** method files (`oldMethod.files?.length!==34`, `method.files?.length!==34`) — a second, code-level 34-count that disagrees with the 38-entry array above.
- `candidate-control-binding-v1.json` is itself one of the 12 entries in `fixture-manifest.json`'s `files[]` array (hash-bound there).
- `fixture-manifest.json` is hash-bound by `README.md`'s `UI_FOUNDATION_MANIFEST_SHA256` line and by `candidate-server-response.ts`'s runtime check.
- Recorded-result binding: `continuationClaims`/`successfulEntry`/`validCompleteResult` in `resource-accounting.ts` hash-chain each claim to the previous claim's SHA-256 (`claim.previousClaimSha256`), forming a linked ledger where editing an earlier claim file breaks every later one.

## Question C — where the product publishes what the instrument reads

Read via `git show HEAD:./src/features/workspace/uiDiagnostics.ts` in full. The global is `globalThis.__openPipeStressUiDiagnosticsV1` (schema `"openpipestress.ui-diagnostics/v1"`), installed once via `Object.defineProperty` (non-writable, non-configurable) at the bottom of that file, exposing exactly `{schema, readCurrent(), projectAuthoredPoint(request)}`, matching `observer-contract-binding-v1.md`'s documented shape exactly.

| Published field (product) | Instrument validation | Validating file:line |
|---|---|---|
| `schema` | `=== "openpipestress.ui-diagnostics/v1"` | `benchmark-harness.ts` `readCandidateDiagnostics` |
| `model.generation`, `.identityHash`, `.projectSessionGeneration`, `.indexGeneration` | monotonic-advance / identity-stability checks | `resource-accounting.ts:113-227` (`readSettledResourceBoundary`, `exerciseRealOpenReplacementCycles`); `benchmark-harness.ts` waits at `waitForFirstUsable`/camera-move `page.waitForFunction` blocks |
| `viewport.camera.{position,target,up,fovDegrees,near,far,aspect,localRenderOrigin}` | tolerances `1e-9`(pos/target/up), `1e-12`(fov/near/far/aspect) | `benchmark-harness.ts:1837-1891` `validateCandidateMeasuredCameraBinding` |
| `viewport.canvas.{cssLeft,cssTop,cssWidth,cssHeight,dpr,bufferWidth,bufferHeight}` | tolerance `1e-6`(css), exact (dpr/buffer) | same function |
| `viewport.labels.{enabled,renderedCount,budget}` | phase-policy check (enabled true/false by feedback kind) | `characterization-commands.ts:74-75` |
| `viewport.geometry.{mode,odGeneration,odStatus}` | driven by `schematic`/`actualOd` controls, read back after toggling | `full-cohort-controller.ts` (geometry-mode region) |
| `viewport.selection.{orderedRefs,primaryRef,renderSubmissionSequence,...}` | equality against oracle `orderedRefs`/`primaryRef` | `causal-presentation-preflight.benchmark.ts:761-763,779-781` |
| `viewport.box.{orderedRefs,primaryRef,direction,filter,...}` / `"unavailable"` | `toEqual({status:"unavailable"})` baseline check, then populated after a real drag | `causal-presentation-preflight.benchmark.ts:738-739,774` |
| `viewport.resources.owned.{live,created,disposed}`, `.context.{generation,canvasConnected,lostCount,restoredCount}`, `.ownedPendingRafCount`, `.rendererInfo.{geometries,textures}` | balance invariant `created-disposed===live`, nonnegative-safe-integer checks | `resource-accounting.ts:20-45` `validateOwnedResourceSnapshot` |
| `viewport.mainRender.{submissionSequence,generation,submittedAt,selectionPresentation,nextPaintOpportunity}` | generation/sequence must advance and match model generation | `resource-accounting.ts`, `candidate-camera-preflight.benchmark.ts:88-100` |
| `viewport.inspector.{ref,generation,publicationSequence}` | equality against expected ref | `causal-presentation-preflight.benchmark.ts:763` |
| `tree.{generation,query,visibleCount,publicationSequence}` | count equality against `expectedTreeFilterCount` oracle | `benchmark-harness.ts` `measureTreeFilter` |
| `ProjectionResult` (`status`, `ndc`, `canvasCssPoint`, `insideClosedNdc`, `insideCanvasCss`, `clip`) | cross-checked against an independent `projectPointToNdc` computation to `1e-9`/`1e-6` | `freeze-candidate-point-oracle.mjs:94-110`; `candidate-camera-preflight.benchmark.ts` (`projectCandidateAuthoredPoint` calls) |

`viewportSelection.ts` and `viewportSelectionPresentation.ts` (both read via `git show HEAD:./src/features/viewport/<file>`) are not diagnostics-publication files — they are the hash-pinned *geometry*/*presentation* sources described in category 5, rows 4-5.

## Question D — what the instrument does with `--ui-viewport-selection-geometry` / `--ui-canvas`, step by step

1. **Camera preflight** (`candidate-camera-preflight.benchmark.ts:34-160`, entry point `playwright.candidate-preflight.config.ts`): after standard-command Isometric+Fit Model clicks and labels-off, at **lines 103-114** the test does `getComputedStyle(document.querySelector(".app-shell"))` and reads back `--ui-viewport-selection-geometry` (line 111) and `--ui-canvas` (line 112) into a `visualStyleReadback` object (`{status,resolvedTheme,themePreference,viewportSelectionGeometry,viewportScene}`). This is the **only site in the instrument that ever reads these two CSS custom properties** (confirmed by exhaustive grep). The object is written, unvalidated at this point, into `candidate-camera-preflight-${pipeCount}.json` (line 150).
2. **Offline freeze** (`freeze-candidate-point-oracle.mjs`, invoked manually/offline, not by any Playwright config): reads that same preflight JSON (line 59), and at **lines 68-73** asserts `preflight.visualStyleReadback.status==="AVAILABLE"`, that `resolvedTheme∈["light","dark"]`, and — case-insensitively — that `preflight.visualStyleReadback.viewportSelectionGeometry.toLowerCase() === visualTokens[resolvedTheme].viewportSelectionGeometry.toLowerCase()` where `visualTokens` is the externally-supplied, hash-pinned `VISUAL_TOKENS_V4.json`. Line 74 separately checks the theme's input sRGB triple. The `--ui-canvas` reading (`viewportScene`) is captured in the same object but I found no line anywhere that asserts against it directly by name — it travels only as unvalidated provenance data alongside `viewportSelectionGeometry`.
3. Still in `freeze-candidate-point-oracle.mjs`, **line 174**, the now-validated `viewportSelectionGeometry` string is copied — unchanged — into each frozen probe's `candidate_preflight.selection_visual_oracle.actualCssSelectionToken` field, purely as a provenance record; it is not compared again after this point.
4. The frozen output (`ui-foundation-${pipeCount}.candidate-runtime-point-oracle-v3.json`, one per fixture size) also carries, per probe, a `visual_plan` object (built at lines 142-148) whose `clip:{x,y,width:48,height:48}` is derived purely from the independently-projected camera/canvas geometry (**not** from the CSS-variable strings) — this is the 48×48 ROI cited in category 1/4.
5. **Timed point run**: `benchmark-harness.ts`'s `measureCanvasPointSelection` (candidate branch) takes its screenshot clip from `probe.visual_plan?.clip` (the value computed in step 4), i.e., by the time of the actual timed/causal measurement, the CSS-variable strings read in step 1 have already been reduced (via the freeze step) to a pinned numeric ROI and an unvalidated provenance string; they are not re-read from the live page during the timed run itself. `causal-presentation-preflight.benchmark.ts:274` (`const clip = phase==="candidate" ? runtimeOracle.visual_plan?.clip : {…}`) shows the same consumption pattern for the one-sample causal proof.
6. Net effect: `--ui-viewport-selection-geometry` is read once (untimed, preflight), validated once (offline, against an externally-supplied token file), then frozen into a per-probe hash-bound artifact; `--ui-canvas` is read at the same moment but, in what I read, is never asserted against a pinned expectation by name — only retained as an unused field in the same readback object.

---

## What I could not determine, and why

- **Roughly 30 of the ~60 distinct SHA-256 literals** found by exhaustive grep (category 5) were not traced to a named constant and a stated binding target within the time available — most appear once, in `README.md` prose or inside CLI scripts I read only in part (`capture-bundle-manifest.mjs`'s own dependency chain, `verify-characterization-observations.mjs`'s embedded fixtures). I would need a second, hash-by-hash pass to close this out.
- **`verify-causal-presentation-extractor.mjs`** (632 lines) was never opened — I relied on its behavior being implied by the extractor it verifies (`causal-presentation-extractor.mjs`, read in full) and by its name. Any pinned value that exists only inside that verifier file is not captured above.
- **`causal-method-contract.spec.ts`** (1133 lines) and **`full-cohort-controller.spec.ts`** (1397 lines) were only grep-sampled/partially read; some of their inline fixture literals (test-only synthetic values, not necessarily "pinned first-profile values" in the sense the brief means) may not be reflected here.
- **`performance-targets.spec.ts`** was grep-sampled, not read end-to-end; I am confident in the constants it exercises (matched category 7) but cannot rule out additional edge-case literals in its untouched middle section.
- I could not locate, within this repository, the byte sources that rows 17-23 of category 5 (`SAME_TRACE_EXPORT_PROFILE.exporterSha256`/`traceTimeSha256`, `PAGE_CLOCK_SOURCE`'s three Chromium-source hashes, `REQUIRED_CHROMIUM_BINDING`'s remaining three hashes) actually bind — they read as references to Chromium's own source tree and to out-of-repo diagnosis artifacts; I did not have a path to verify them the way I verified the two `viewportSelection*.ts` hashes.
- **`viewportResource.ts`** was grepped for its export list only, not read in full; I confirmed `currentOwnedViewportResourceSnapshot` and `OWNED_RESOURCE_KEYS`-shaped `resources` machinery exist and are exported, but did not verify every field name in its snapshot type against `resource-accounting.ts`'s expectations line by line.
- The **live hash mismatch I found** on `src/features/viewport/viewportSelection.ts` (category 5, row 5) is a snapshot of one moment during a concurrent edit by another agent; it may already be different again. I report it as what I observed, not as a stable fact.
- I did not open `../../apps/desktop/src-tauri/tauri.conf.json` even though a comment in `playwright.config.ts` cites it ("the packaged Tauri window default (1440×920 per src-tauri/tauri.conf.json)") — I have not independently verified that file's contents, only the comment referencing it.
- Per the brief's "one mutation route" / "Result integrity" / "never alter a tolerance..." lane limits, no design proposal is offered here — this is strictly a report of what exists.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).