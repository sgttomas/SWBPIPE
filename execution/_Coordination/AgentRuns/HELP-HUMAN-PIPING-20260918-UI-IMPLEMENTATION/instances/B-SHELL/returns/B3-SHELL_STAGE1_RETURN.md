# B3-SHELL return: stage 1, the shell's control layer

Stage 1 is done and I have stopped. Both checks pass, the guarded diff is empty, and nothing visible changed. No recorded first-profile value in `e2e/ui-foundation/**` is contradicted by the new shell, but three conditions of that instrument constrain stage 2; they are under the benchmark-hooks table.

**Model:** I am Claude Fable 5.1 (`claude-fable-5-1`), by my own statement. TASK, Type 2; I launched no agent.

**Base:** head `c809b5b5a5397a712ac785c50eed7e2a3418bb40`, branch `codex/swbpipe-b-shell-20260918`. The brief's hash verified as `079876c3…9cec`. I ran no git command that changes state.

**How I wrote:** every file went through the shell, as the launch message predicted. New files by heredoc; edits by Python exact-match replacements that assert a single occurrence.

**Reading:** the handoff whole; specification §2 whole, §5.4, §5.5 and §9.3; `B3_CUT_PROPOSAL.md`; the B2 finding; addendum 1; the named sections of addenda 2 and 4. From `MOCKS_V4.md`, the `s2_model_light` frame and `report.json` I read only what the drawer geometry needed. I did not read specification §10.9 or §12, the operations map, the design-system sections, addendum 3 or the frame bodies: they bear on stage 2 and the owner's usage limit applies. I will read them before stage 2.

## Files

Paths are relative to `apps/desktop/`.

| File | State | SHA-256 |
|---|---|---|
| `src/App.tsx` | changed (+3: one import, the provider's open and close tags around `<main>`) | `3b0412d83a689baae9eafec7ede90136da3db1d4eac4c406f22786620045e418` |
| `src/features/workspace/chromeSessionState.ts` | changed (two cells, their import, the doc comment) | `af2599081176126bc34a25114dd685e64fefcaa1c37626da214c8c405cf1ba4c` |
| `src/features/workspace/workspaceSession.ts` | changed (destructure, one import, one effect, three names in the chrome slice) | `415c30a8d07d007a79fa0f0b8c10be2e067da80cfbc03e95655e159d0f929fa0` |
| `src/features/workspace/WorkspaceSessionContext.tsx` | new | `20b6f7d36e111a8b0cd6026fc1c6e76568829d46322195e03a33791295018158` |
| `src/features/workspace/shellLayout.ts` | new, React-free | `c054910c41de3097a81bc4be12193e090ca0bd66a43a16cea44363465a59c6f7` |
| `src/features/workspace/shellLayout.test.ts` | new, 37 tests | `25c7e823ad2e16f758374bd6cb08361880aeb0efb7831801036bb725b86bf081` |
| `src/features/workspace/sessionBoundary.test.ts` | new, 3 tests (the import guard) | `abc9b2983f3d587694ef0ff944d8b790e1e8af7158994d20ebd2f68bf9825515` |
| `src/features/workspace/statusLabels.tokens.test.ts` | new, 10 tests (label agreement) | `857e8d7ecd1b97f0fed3fa9164138ce9bd7c15096ff952212d3a55350c379a48` |
| `src/features/workspace/workspaceSession.shell.test.tsx` | new, 8 tests (key set, switch invariance, context) | `4b21d4d93a3871533880a52696fa123ad13d8615eb9a2a4b3c753711a635de8a` |

Nothing was deleted, and no existing test, e2e file or stylesheet was touched. No authored file contains an absolute machine path. The `styles.css` comment corrections are due in its first edit, which is stage 2.

## Checks

| Command | From | Result |
|---|---|---|
| `npm run build:desktop` | `projects/chirality-piping` | Exit 0. The first attempt failed on one type error in my new test (`modelAssignment` possibly null); I fixed it and rebuilt. |
| `npm run test:desktop` | `projects/chirality-piping` | Exit 0: 80 files and 1,259 tests passed. That is the base's 76 files and 1,201 tests plus my 4 files and 58 tests. |
| `git diff --stat -- src/App.test.tsx src/App.projectHandlers.test.tsx src/App.deadControls.test.tsx e2e src/styles.css` | `apps/desktop` | Empty. |

`git status --short` lists only the nine files above. No file changed after the last check. I ran no Playwright lane.

I ran one throwaway measuring script, kept outside the repository. It ran twice through `sh {RUN}/tools/with_e2e_lock.sh` with `PLAYWRIGHT_WORKERS=1`, against a dev server on port 5174 that the script started and stopped. The lock was free and no other run was waiting. Port 5174 is confirmed not listening afterwards.

## What stage 1 built

1. **Import guard** (`sessionBoundary.test.ts`).
   - It scans `src/**` and fails if any file other than `workspaceSession.ts` names a `*SessionState` module specifier.
   - Exemption: `xSessionState.test.ts(x)` may import only its own module.
   - It also pins that exactly six state hooks exist and that the session imports six.
2. **Session key-set test.**
   - It pins the six slices in order and the exact setters per slice.
   - It fails if a model, results, history, undo, redo, operation-outcome, project or selection setter is exposed.
   - The base's eleven setters are unchanged. The one setter this slice adds, which the test names, is `setStageViewMemory` on chrome. `setStageSurface` stays inside the session.
3. **Label agreement test.** The `labels` table in `tokens.json` agrees with `statusLabels.ts` on tokens, order, label, domain and kind. The only extra key allowed in the copy is `chip`.
4. **`WorkspaceSessionContext`.**
   - `WorkspaceSessionProvider` takes the session; `AppSession` wraps its `<main>` in it.
   - Hooks: `useWorkspaceSessionContext`, `useSessionModel`, `useSessionSelection`, `useSessionResults`, `useSessionOperations`, `useSessionProject`, `useSessionChrome`.
   - A component outside the provider throws `WORKSPACE-SESSION-CONTEXT-ABSENT`.
   - `AppSession` still calls `useWorkspaceSession()` once. No effect or memo lists the session or a slice as a dependency.
5. **Two new chrome cells.**
   - `stageSurface` is a `WorkspaceSectionId` or null, and is kept inside the session.
   - `stageViewMemory` is exposed with its raw setter.
   - The comment where `stageViewMemory` is declared says it is session state and that saving it with the project is G-17.
6. **One new effect in the session**, on `[activeSection]`: `setStageSurface(current => stageSurfaceAfter(current, activeSection))`.
   - It sits beside the existing effect on the same dependency, so React batches the two.
   - On mount it sets null over null and causes no re-render.
   - B-SHELL's records cite the session's effect count as 13. It is now 14.
7. **Switch invariance test**, run through the real session.
   - Scenario one is a Current solved run: `solveProof`, `inputManifest` and `analysisRun` are held.
   - Scenario two is one applied operation followed by a run made after it, so the undo stack holds a checkpoint.
   - In each, every stage and every view of it is visited: 13 switches.
   - After each switch, 35 cells are compared by `Object.is`: model, hash, revision, generation, result, run records, solve proof, job, undo and redo stacks, applied operations, outcomes, batches, project summary, selection.
   - A separate test opens pages over a stage and makes the same comparison.
   - `ruleCheckRunBasis` and `dormantOutputBasis` are rebuilt on every render from cells already in the list, so they are held through those cells.

## The design stage 1 fixes for stage 2

**Section-to-home map, as coded (`SECTION_HOMES`).**

| Section | Home | Rebuilt by |
|---|---|---|
| none active (the model tree) | Model stage, tab `model-tree` | B4 |
| `operations` | Model stage, tab `review-changes` | B4, B5 |
| `loads` | Loads stage, tab `load-cases` | B4 |
| `results` | Results stage, tab `results` | B6 |
| `evidence` | Results stage, tab `evidence`, labelled "Evidence" | B6 |
| `report` | Review stage, tab `report` | B7 |
| `libraries`, `rule-packs` | pages of kind `rail` | not in this tranche |
| `solve`, `project`, `exports` | pages of kind `over-stage` | B6 for `solve`; the others not in this tranche |

**Navigation.**
- `shellLocation(activeSection, stageSurface)` returns `{ stage, tab, surface, page, pageKind }`.
- The stage is never stored; it is derived. `activeSection === null` always means the Model stage's tree.
- Every other stage is present exactly while one of its sections is active. This keeps today's `activeSection === X` semantics intact: `DormantSection`, `reviewActive` and expensive-section activation.
- `stageSurface` remembers only the surface a page was opened over.
- The rail calls `setActiveSection(sectionForStage(stage, stageSurface))`.
- A page's close control and `workspace-dock-close` call `setActiveSection(sectionAfterPageClose(stageSurface))`. From Evidence, open Project, close: you return to Evidence.

**Views.**
- `viewForStage(memory, stage)` and `rememberStageView(memory, stage, view)`; the latter returns the same object when nothing changes.
- `stageViews('review')` is `['table']`, because the Review page has no canvas. The view switch reports Model and Both as not enabled on Review.
- `viewSwitchItems` and `inspectorToggleState` carry the ruled tooltips and reasons.
- `AGENT_UNAVAILABLE_REASON` is "Agent: not available yet".

**Rail.** `railStageState(stage, run)` returns `{ stage, label, enabled, reason, caption, tooltip }`. `RunPresence` is `{ hasResult, hasCurrentSolvedResult, historicalRunShown, solveJobState }`, built by `runPresenceFromCells({ result, historicalRun, solveJob })`.

| Session state | Results | Review |
|---|---|---|
| Before a run, while a run is going, or after a model change | disabled, "No run yet" | disabled, "No run yet" |
| A held result, solved | enabled | enabled |
| A held result, not solved | enabled | disabled, "No solved run" |
| `solveJob.state` failed, no result | enabled, caption "Failed", tooltip "Failed · Results" | disabled, "No solved run" |
| `solveJob.state` cancelled, no result | enabled, no caption | disabled, "No solved run" |
| Historical run shown | enabled, caption "Historical" | disabled, "Needs a current run" |
| Historical run shown and a later run failed | enabled, caption "Historical" (it is what is on screen) | disabled, "Needs a current run" |

"Stale" is never produced, and a test holds that. `viewMenuStageItems(run)` is the same function projected to the ids `view.stage.*`, so the menu and the rail cannot disagree; a test holds that too.

**Chip function signature.**

```ts
statusChips(inputs: StatusChipInputs, stage: ShellStage, pageOpen = false): StatusChip[]
// StatusChip = { token, label, domain }
// StatusChipInputs = RunPresence & { modelMechanicsStatus, resultMechanicsStatus,
//                                    resultProfessionalStatus, rulePackStatus }
```

- The inputs are built by `statusChipInputsFromCells({ model, result, historicalRun, solveJob, ruleCheckAggregate })`.
- `statusChipText(chip)` gives the face "Domain · Label".
- Every chip is a row of `statusLabels.ts`. An evidence label can never be a status-bar chip.

| Rule | What the function does |
|---|---|
| 1 | One Solver chip only while `model.analysis_status.mechanics` records `MODEL_INCOMPLETE`. |
| 2 | Otherwise no chip. |
| 3 | For a solved result off Review: the Solver chip, plus the rule-pack status when a pack is set. |
| 4 | On Review: three chips with a pack; Solver and Human with none. Human comes from the result's own `professional_acceptance` through `professionalStatusToken`. It does not appear when a page covers Review. |
| 5 | After a model change there is no run, so rule 1 or rule 2 applies. |
| 6 | With a Historical run shown the list is empty on every stage. |
| 7 | A failed or stopped run with no result gives an empty list, and Model incomplete is not carried over. A run that completed unsolved shows only its Solver status. |

**Context shape.** The value is the `WorkspaceSession` itself, six slices. Components read a slice through its hook.

**Geometry** (`SHELL_REGIONS`, `shellGeometry`, `clampBothSplit`). Unit tests hold the 48, 56, 44 and 24 px regions.
- Both view at 1440 × 900: table pane 737 × 828 at x 56; canvas 603 × 828 at x 793.
- With the inspector open, the canvas is 303 wide and the inspector 300 wide at x 1096. The table does not move.
- Model view: canvas 1000 × 828 with the drawer collapsed; inspector 340 wide at x 1056. With the drawer open the canvas is 1000 × 548.
- The split stops where the canvas would fall under 220 px.

## Judgment calls for B-SHELL and ROOT

1. **"A rule pack is set" is read as `ruleCheckAggregate !== null`.**
   - The aggregate is the session's cell for the rule check run against a loaded pack.
   - Today's pill shows the result's own `rule_check` value, and the preview fixture records `RULE_INPUTS_INCOMPLETE` with no pack loaded. Under §5.4 that value is not drawn without an aggregate.
   - This is a named semantic change for stage 2.
2. **A Historical run empties the chip list entirely.**
   - That includes rule 1's Model incomplete chip for the current model. I followed the brief's words ("lights no chip", "Historical lights none").
   - The other reading is that the run contributes nothing and rules 1 and 2 still speak for the current model. Reversing it is one line.
3. **Rule 1 reads the status the model document records.**
   - The interface asserts none of its own.
   - The preview fixture records `MODEL_INCOMPLETE` on a model that solves, so that chip shows at start-up, as today's pill does.
   - Stage 2's "rule 2, no chip" product test needs a model whose recorded status is something else.
4. **While a run is in flight, rules 1 and 2 apply.** This is today's behaviour; the specification is silent.
5. **After a model change, the Results rail item reads "No run yet".** That is the only permitted copy for the state. It is slightly loose after an edit, and it is G-11's consequence.
6. **The browser fixture's run of an edited model completes unsolved.**
   - It records `MODEL_INCOMPLETE` with 0 rows, so rule 7's first row is a live path.
   - Stage 2's product tests that need a solved run must solve before editing.
7. **`view.section.*` commands can reach a stage the rail disables.** For example `view.section.results` before a run, which today's tests and `openWorkspaceSection` rely on. My plan is that the new `view.stage.*` items follow the rail's disabled states, and the old section items stay enabled so that no behaviour test is weakened. B-SHELL may rule otherwise before stage 2.
8. **Review has Table view only.** No copy is ruled for the disabled Model and Both segments there. I need wording, or they are disabled without a reason, which the brief's own rule forbids.
9. **Frame against D-72.**
   - In `s2_model_light` the drawer is in flow under the canvas: canvas 1001 × 548 with the drawer open. A collapsed 28 px strip in flow would give 800 px, not 828.
   - To give D-72's 1000 × 828 "with the drawer collapsed", the collapsed strip must lie over the foot of the canvas's box. It is coded that way and I am reporting it as a difference between the frames and the decision.

## Benchmark-hooks table

- Today's numbers are measured in Chromium, light theme, comfortable density, at 1440 × 900. Where a value differs at 1440 × 920 it follows a slash.
- They were taken on the stage-1 tree, which renders as the head does.
- Stage-2 numbers are designed values from `shellGeometry`.
- The first-open state is the Model stage in Both view; "inspector open" means the Inspector toggle is latched.
- The instrument itself runs at 1440 × 920, where every surface height is 848, not 828.
- The "Used by" column says whether `e2e/ui-foundation/**` (the instrument), `ui-foundation-workflows.ts` (workflows, this lane's file) or both use the hook.

| Hook | Used by | Today: element and geometry | Stage 2: element | Stage 2: designed geometry at 1440 × 900 |
|---|---|---|---|---|
| `.app-shell` (reads `data-theme`, `data-density`) | both | `<main>`, 0,0 1440 × 900 / 920 | same `<main>`, same attributes | same |
| `desktop-preview-shell` | workflows | same element | same | same |
| `.workspace-pane-tree` | both | left rail pane, x 6 y 161.2, 280 × 685.2 / 705.2 | the table pane that holds the model tree | Both: x 56 y 48, 737 × 828. Model: the drawer, x 56 y 596, 1000 × 280; collapsed 1000 × 28 at y 848. Table: 1340 × 828 |
| `.panel.model-tree` | instrument | x 7 y 194.2, 278 × 570.2 | the `ModelTree` panel inside that pane, unchanged | fills the pane under the 28 px tab strip |
| `model-tree-filter-input` | both | x 107 y 271, 128 × 32 | same input in `ModelTree` | inside the table pane |
| `model-tree-virtual` | instrument | x 17 y 334.4, 258 × 420 | same | about 717 wide in Both view |
| `model-tree-filter-summary`, `clear-model-tree-filter`, `model-tree-filter-empty`, `tree-row-*`, `[role=treeitem]`, `.tree-item-label` | both | inside `ModelTree` | unchanged; `ModelTree` is rehoused, not rebuilt | — |
| `toggle-tree` (`aria-expanded`) | workflows | full-width header button of the tree pane, x 7 y 162.2, 278 × 32, expanded | Model view: the drawer's chevron, carrying `aria-expanded`. Both and Table views: no honest successor, since the pane is always shown | 28 × 28 at the strip's right end. `ensureRail('tree')` in my workflows file moves to "ensure the table pane is shown" |
| `.workspace-pane-inspector` | both | right rail pane, x 1094 y 161.2, 340 × 685.2 / 705.2 | the docked inspector | Both, inspector open: x 1096 y 48, 300 × 828. Model: x 1056 y 48, 340 × 828. Table: not shown |
| `.workspace-pane-inspector .panel.inspector` and its `> h2` | instrument (8 places), also `property-inspector` and `aggregate-property-inspector` | panel x 1095 y 194.2, 338 × 300; h2 x 1105 y 212.2, 318 × 36.4 | same `PropertyInspector`, same markup | at the docked inspector's top-left |
| `toggle-inspector` (`aria-expanded`) | workflows | full-width header button of the inspector pane, x 1095 y 162.2, 338 × 32, expanded | the toolbar's Inspector toggle | in the 48 px band, right of centre. It must keep `aria-expanded`, not only `aria-pressed`, because `ensureRail` reads that attribute |
| `command-bar`, `.command-context > summary`, `command-selection-readout`, `.viewport-toolbar > span`, `viewport-*`, `canvas` | instrument | inside `PipeViewport`: bar x 293 y 162.2, 794 × 43; canvas x 293 y 285.2, 794 × 560 / 580; readout 0 × 0 inside a closed `<details>` | unchanged; `PipeViewport` is mounted, not modified | canvas region x 793 y 48, 603 × 828 (303 wide with the inspector open); Model view 1000 × 828. The drawn canvas is smaller by the viewport's own bars |
| `workspace-dock-close` | workflows | absent until a section opens; it is the dock header's close button | the page's close control (⎋ accelerates it) | top-right of the page header |
| `menu-view`, `menu-item-view.section.*` | workflows | in-app menu bar, x 95.9 y 77.2, 48.6 × 32 | same menu, same ids; View gains Table, Model, Both and the four stages | see "Menu bar" below |
| `workspace-section-*` | workflows | the dock's section containers | same containers, rehoused in the table pane or a page | — |
| `workspace-review` | workflows | toolbar button, x 1318.3 y 116.2, 79 × 32 | kept on the control that opens Review changes | toolbar band, or the Model stage's second tab (see question 2 below) |
| `open-local-project` | instrument (`resource-accounting.ts`, fixtures, control binding) | title bar, x 1151.4 y 10.9, 92.9 × 32 | same button in the Project page's header | findable by role and name while the page is closed |
| button "Save local" | instrument (`resource-accounting.ts`) | title bar, x 1338.1 y 10.9, 89.9 × 32 | same button in the Project page's header, same accessible name | same condition |
| `local-project-message` | instrument (`resource-accounting.ts`) | project strip summary, x 12 y 57.8, 163.9 × 15.4 | the Project page's message, a `role=status` | same condition |
| labels "Appearance theme", "Workspace density" | workflows | title-bar selects, x 729.4 and 850.2, y 10.9 | labelled selects in the in-app View menu | labelled controls kept, as the brief requires |

**Menu bar.** Whether an in-app menu-bar row fits between the 48 px toolbar and the surfaces is not in the 48 / 56 / 44 / 24 arithmetic. Every surface height I give, and D-72's 828, assumes no such row. In the browser runtime a menu-bar row would take about 32 px from the 828. That is a number for B-SHELL before stage 2.

**Does any recorded first-profile value become untrue?** No.
- The folder records no measured geometry of the App's shell.
- The only shell numbers in it are the synthetic doubles in `full-cohort-controller.spec.ts:1025`. Both panes there are 300 wide, one at x 0 and one at x 1000. Those are inputs to a validator and do not pass through the App.

**Three conditions of the instrument constrain stage 2**, and the canvas lane should know them:
1. **Both panes visible at every boundary.**
   - `captureBoundary` requires exactly one `.workspace-pane-tree` and one `.workspace-pane-inspector`, each visible with positive size, and both panels visible.
   - Today that holds at first open: both rails are expanded at 1440 px.
   - Under the new shell it holds only if the inspector is open in Both view at first open, or the Model view's drawer is open.
   - So the Both view's inspector must default to open (canvas 303 × 828), or the instrument must open it. The instrument has no call that opens it today: it does not import my workflows file.
   - D-72's 603 × 828 is the canvas with the inspector closed. That conflicts with this condition, and ROOT and the canvas lane must settle it before stage 2 lands.
2. **The instrument pins the window at 1440 × 920.** D-72's canvases are stated at 1440 × 900. At 1440 × 920 they are 603 × 848 and 1000 × 848.
3. **Pane rectangles must be identical across the boundaries of one run.** Opening a page or switching view in mid-run would trip "boundary profile/model/binding drift". Today only panel width and height may vary.

## For someone else to handle

- **ROOT and the canvas lane:** the three conditions above, and the drawer-overlay difference in judgment call 9.
- **B-SHELL before stage 2:** judgment calls 1, 2, 7 and 8, and three stage-2 questions.
  1. The in-app menu-bar row's height against the 828.
  2. Where `workspace-review` lives.
  3. Whether the toolbar's Inspector toggle may carry `aria-expanded`.
- **B-SHELL's records:** the session's effect count is now 14, not 13.

I claim no usability, conformance or performance acceptance. PDU-045 and PDU-046 remain holds.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).