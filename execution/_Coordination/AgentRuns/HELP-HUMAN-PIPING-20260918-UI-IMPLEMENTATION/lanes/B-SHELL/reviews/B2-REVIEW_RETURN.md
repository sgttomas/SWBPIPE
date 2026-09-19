# B2-REVIEW: independent read-only code review of slice B2 (state extraction; candidate 5d2e261e0) — retained return

Retained by ROOT. What follows is the child's final message as the host stored it, with two substitutions and nothing else changed. The machine's repository path is replaced by `{REPO_ROOT}`. In check 9, the reviewer lists the four path patterns it searched for; the fourth was a literal temporary-folder path, which the repository's path-anchor validator reads as a machine path wherever it appears, so ROOT writes it here as `{the macOS temporary-folder root}`. The message exactly as the host stored it has SHA-256 `225e133c97136fb402d09c4c047121018f72b3b9610bab8c145771f2ce559e09` (137 lines), by ROOT's extraction script.

---

# B2-REVIEW return

I am **Claude Opus 5 (1M context)** (`claude-opus-5[1m]`), running as TASK (Type 2), read-only, alone, in a fresh context.

**Brief** `{RUN}/lanes/B-SHELL/briefs/B2-REVIEW_code_review.md` — SHA-256 read `69bb0ce07fb472d9017c42988765d525016314d62e0b23805e8891576648ba9a`, **matches** the seal.
**Skill** `{REPO_ROOT}/.agents/skills/software-code-review/SKILL.md` — SHA-256 read `06c27b1be5cfbd9e638570918a8f837d8439c8073c40d3ef708e53874f95570a`, **matches** the seal.

Paths below are relative to `{WORKING_ROOT}/apps/desktop` unless marked. The worktree is unchanged; I ran no state-changing git command. Scratch went to a temporary directory outside the repository.

---

## Verdict: **PASS** — no actionable finding

The slice is a pure move. I established that independently of the lane's tools, then ran the lane's tools and they agree.

---

## How I established that moves are moves

I did not sample. I compared whole texts four ways.

1. **Byte-exact line multiset, whole file.** Base `src/App.tsx` (4,356 lines) against the candidate's `src/App.tsx` + all 14 new modules. Exactly **82** non-blank base lines are absent from the candidate: **53 import-related** (18 whole `import` lines, 9 `} from "…"` closers, 26 specifier fragments) and **29 declaration headers that gained the `export` keyword** — each verified by testing that `export <line>` is present verbatim on the candidate side. Nothing else. No executable line anywhere was altered.
2. **Component body, byte-exact including indentation.** Every one of the 2,763 lines of the base `AppSession` body is present byte-for-byte (leading whitespace included) in the candidate. Ordered `diff` of the base body against `useWorkspaceSession` + the new `AppSession` yields **6 hunks**: removal of the state-cell declarations (which moved into the six hooks), the six destructurings, the session `return` object, the `AppSession` destructurings, and one trailing comment. No hunk touches a handler, an effect, a guard or the JSX.
3. **Declaration-level ordered comparison.** Extracting every top-level declaration with a balanced-body parser: **62 of 63 are byte-identical in full** once a leading `export` is stripped. The one that differs is `AppSession`, which is what the slice splits.
4. **Uniform translation.** Base `src/App.tsx` lines **737–2410 are byte-identical to `features/workspace/workspaceSession.ts` lines 315–1988 — 1,674 consecutive lines at a single unbroken offset of 422.** That slab is the whole handler-and-effect block; it moved as one unedited unit. The remaining moved regions are likewise uniform-offset slabs, broken only where a header gained `export`.

**Import statements** — the audit's declared blind spot — I closed separately. Resolving every relative specifier against `src/`, the set of (imported name → module) pairs is **192 on the base side and 192 on the candidate side, with zero difference in either direction**. No import dropped, added or re-pointed. There are no bare side-effect imports, no `.css` imports, no dynamic `import()`, on either side. The candidate adds three `export … from` re-exports in `App.tsx`, which preserve its export surface exactly (base exported `SolveProofEvidence`, `SolveRunGenerationGate`, `RuleRevisionGenerationGate`, `commitModelAfterSolveInvalidation`, `isSupportedChangedModelPersistenceResponse`, `App`, `solveProofStatus`; the candidate exports the same seven), which is what keeps `src/App.test.tsx` importing from `"./App"` unchanged.

---

## Check by check

**1. Safety net untouched — PASS.** `src/App.test.tsx` blob at the candidate is `86d1366aaaeadf5d7420b6fd0f2108a87c1f3fae`, **identical to `origin/main`'s blob**; it differs from the audit base only by PR #802's line, as the brief states. No test file anywhere in the range is added, modified, deleted, skipped or loosened — the range touches 15 code files and records, nothing else. I ran `npx vitest run src/App.test.tsx`: **1 file passed, 218 tests passed** (the jsdom `HTMLCanvasElement.getContext` noise is the pre-existing WebGL-in-jsdom warning, not a failure). **No new tests were added**, so nothing new is pinned; that is consistent with a move-only slice but it means the extraction's new boundary (the six hooks, the session object) has no test of its own.

**2. Result integrity, line by line — PASS.** Before → after, all byte-identical:

| Mechanism | Base | Candidate |
|---|---|---|
| `SolveRunGenerationGate` | `src/App.tsx:217` | `features/workspace/solveGates.ts:5` |
| `RuleRevisionGenerationGate` | `src/App.tsx:261` | `features/workspace/solveGates.ts:49` |
| `commitModelAfterSolveInvalidation` | `src/App.tsx:277` | `features/workspace/solveGates.ts:65` |
| `clearComputedModelState` | `src/App.tsx:1708` | `workspaceSession.ts:1286` |
| `invalidateReportPackageComputedState` | `src/App.tsx:1721` | `workspaceSession.ts:1299` |
| `stillCurrent`, all 8 sites | `1175, 1291, 1371, 1432, 1547, 1569, 1772, 1866` | `753, 869, 949, 1010, 1125, 1147, 1350, 1444` |
| Current / Historical designation | `setHistoricalRun` at `861, 1105, 1711, 1761`; `historicalRun` at `572, 1774`; `HistoricalRunPanel` in the JSX | `workspaceSession.ts:439, 683, 1289, 1339, 1352`; cell at `resultsSessionState.ts:39`; `src/App.tsx:816` |
| Solve-input basis | `ruleCheckRunBasis` `src/App.tsx:577`; `currentInputManifestRef` `:707,:709`; `resultBasisRef` `:575,:576` | `workspaceSession.ts:281`; `resultsSessionState.ts:66,:68`; `resultsSessionState.ts:42,:43` |
| Reviewed application | `handleProposal :1229`, `handleAddDraftReview :1360`, `handleApplyDraftReview :1413`, `handleValidateIntent :1542`, `handleApplyIntent :1564`, `handleRunOperationBatch :1286` | `workspaceSession.ts:807, 938, 991, 1120, 1142, 864` |
| Undo / redo | `handleUndoSessionModelEdit :1662`, `handleRedoSessionModelEdit :1685` | `workspaceSession.ts:1240, 1263` |

Every pair sits at the uniform −422 offset, inside the 1,674-line identical slab. The guards that "read a value from a closure" still read it from the same closure: the handler bodies are unchanged, and the refs they read are the *same objects*, passed by reference through a shorthand return and a destructuring — nothing is spread, copied or rebuilt. No guard can now read a newer or older value (see the render-order analysis below).

**3. Order and timing — PASS.** All 13 effects live in one function (`useWorkspaceSession`), in base order, byte for byte, with byte-identical dependency arrays. Positional kinds line up exactly: `E, LE, LE, E, E, E, LE, E, E, LE, E` then `useRef` then `E, E`. **No `useEffect` became a `useLayoutEffect` or the reverse.** Hook call-site counts are equal on both sides: `useState` 67, `useRef` 31, `useMemo` 4, `useCallback` 2, `useEffect` 9, `useLayoutEffect` 4. No state update moved across an `await` (handler bodies are unchanged). `AppSession` remains a single component — `useWorkspaceSession` is a hook, not a provider component — so all 68 cells stay in one fiber and batching is unchanged; no previously atomic pair can now be observed apart.

**4. One mutation route — PASS.** `applyModelOperation` at 2 call sites (base `1491, 1575` → `workspaceSession.ts:1069, 1153`), `applyOperationBatch` / `validateOperationBatch` at 2 (base `1299, 1442` → `877, 1020`). `setModel` is written exactly once, in `commitModel`, on both sides. Write-site counts are equal for `setResult` (7), `setAnalysisRun` (8), `setUndoStack` (9), `setRedoStack` (9), `setAppliedOperations` (7), `setInputManifest` (7), `setHistoricalRun` (7), `setUiModelRevision` (1). The session object exposes exactly **11 setters** — `setUiPreferences`, `setActiveSection`, `setOpenMenu`, `setOperationTab`, `setReviewDetailsOpen`, `setAuditDrawerOpen`, `setIssuesDrawerOpen`, `setHiddenEntityKeys`, `setIsolateHiddenEntityKeys`, `setSolverMode`, `setReportPackagePrivateIntent` — none of which touches the model, results, history, undo/redo, operation outcomes or project state. The JSX is byte-identical, so what components can reach is unchanged; tables and the canvas remain projections.

**5. State identity — PASS.** 68 state cells, identical names, no duplicates across the six hooks, identical initializers (all byte-identical declarations). Nothing moved from derived to stored or the reverse: `currentSolvedResult`, `resolvedTheme` and `effectiveHiddenKeys` are still derived, in the same relative positions. Persistence is untouched — `features/workspace/uiPreferences.ts` is not in the diff, `readUiPreferences` is still the first statement of the session's first hook, and the project open/save handlers are byte-identical, so the saved key and shape are unchanged.

**6. Moves are moves — PASS.** Established four ways above. **The only textual change to moved code anywhere is the `export` keyword added to 29 top-level declarations**, which the move requires and which is semantically inert; every one of those new exports has a consumer (no gratuitously widened surface). The manager's return does not enumerate those 29 individually, but its framing covers the class ("moves only; no moved statement was edited"; the audit strips `export` by design). I found **no other edit inside moved code**, so I have nothing to add to its list.

**7. Nothing visible — PASS.** `src/styles.css` is not in the diff; no `.css` import exists on either side. The JSX (base statement #177) matched *identical*; no DOM structure, role, name or `data-testid` changed — any such change would have appeared as a changed line in the whole-file byte comparison, and none did. `e2e/**` is untouched, so the Playwright specs are unchanged. `features/workspace/statusLabels.ts` is untouched and `src/App.tsx:55` still imports `professionalStatusToken`, `ruleCheckStatusToken`, `statusDisplay` from it as the only source of status labels.

**8. Scope — PASS.** The lane's own eight commits write only `src/App.tsx`, the 14 new `src/features/workspace/*.ts` modules, and `{RUN}/instances/B-SHELL/**`. `features/viewport/**`, `features/workspace/modelIndex.ts`, `e2e/ui-foundation/**`, `src-tauri/**`, `core/**` and schemas are untouched; no tolerance, oracle or benchmark limit changed; the picking repair and its tests are untouched. The five files under `{RUN}/lanes/B-SHELL/` that the three-dot diff shows come from ROOT's launch commit `7c6784d95`, not from the lane — I confirmed this per file with `--diff-filter=A`.

**9. Records — PASS.** No authored file added or modified anywhere in the range contains an absolute machine path (`/Users/`, `/home/`, `C:\`, `{the macOS temporary-folder root}`). `{RUN}/instances/B-SHELL/briefs/_INDEX.md` lists the one child brief `B2-STATE.md` with its SHA-256 `a938b135…`, seal time, requested model (`fable` / Claude Fable 5.1) and role; `returns/_INDEX.md` lists all three stage returns with their SHA-256 and the model that ran.

**Design handoff §3 (the ten constraints)** — 1 and 2 hold (above); 3 holds (check 2); 4, 5 and 10 hold by scope; 6 and 7 hold because no control and no copy changed and no semantic change is claimed or found; 8 has one confirmed consequence, below; 9 is not claimed.

---

## The four things the byte-for-byte audit cannot see

**Hook call order — settled, not observable.** Reconstructing the full candidate execution order in base statement numbers: chrome `1,2,3,74,75,77-79,81-85,87-96`; model `4,5,13-18,39,98,99,112`; selection `6-12,114`; results `19-24,26,27,29,30,31,35,47-57,103-111`; operations `32,33,58-73,97,100-102`; project `34,36-38,40-46,51`; then `useWorkspaceSession`'s own `25,28,76,80,86,113,115-165,168-176`; then `AppSession`'s `166,167,177`.

From the move audit's own statement kinds I enumerated the base body exhaustively: the render-time statements that are *not* declarations are exactly `#24`, `#27` (the two `IfStatement` basis bumps), `#107`, `#108`, `#163` (the three `ExpressionStatement` ref writes), `#167` (the early return) and `#177` (the JSX). That is an exhaustive confirmation of the manager's "five render-time ref writes", arrived at by kind rather than by search.

**Every reader of the four refs:**

- `resultBasisRef` — `#25 ruleCheckRunBasis` (`workspaceSession.ts:284`) and `#28 dormantOutputBasis` (`:289`). Both render-time, both still strictly after the bump.
- `analysisBasisRef` — `#28` (`:289`), and one closure read at `:1821`.
- `currentSolvedResultRef` — three closure reads only: `:737`, `:757`, `:1819`.
- `currentInputManifestRef` — three closure reads only: `:738`, `:758`, `:1820`.

All six closure reads are inside `renderedBasisIsCurrent` / `stillCurrent` in `handleRuleCheckAggregate` and `isCurrent` in `handleSaveReportPackage` — evaluated in an event handler or after an `await`, never during render. A render pass is synchronous and invokes none of them, so where inside the pass the ref was written cannot change what they see.

The two render-time readers did change sides relative to `#107`/`#108` (they now evaluate after those assignments, where before they evaluated before) — **but neither reads those refs**; `ruleCheckRunBasis` reads only `projectSessionGeneration`, `uiModelRevision` and `resultBasisRef.current.sequence`, and `dormantOutputBasis` only `uiModelRevision` and the two sequences. `#25` also now evaluates after the `analysisBasisRef` bump where before it was before it — again, it does not read that ref. The statements that changed sides in the other direction are all `useState` / `useRef` / `useMemo` / `useCallback` declarations with pure or literal initializers; I checked every one, and none reads any of the four refs (the only initializer that reads a ref is `getPreparationEpoch = useCallback(() => requestEpochRef.current, [])`, and it reads it inside a closure). The four statements' order *relative to each other* is unchanged. The fifth write, `#163`, did not move relative to its `useRef` (`#162`) or to the two effects that follow it.

I also checked the memo reordering the finding rests on: `activeModelIndex` and `selectedPipeRefs` now evaluate before `comparison`. `modelIndexFor`'s only module state is a `WeakMap` keyed by the model, `buildPreviewComparison` builds from its arguments and never calls `modelIndexFor`, and `entityRefFromKey` is a pure parse — so the swap is unobservable.

**Every hook call is unconditional.** All 22 call sites in `useWorkspaceSession` are at the function body's top level, none under a condition, loop or nested function. `useWorkspaceSession` has exactly one `return`, the final one — **no early return before a hook**. `AppSession` calls exactly one hook, `useWorkspaceSession()`, as its first statement; the early return (`#167`) follows every hook in both versions. Base `#168`–`#176`, the statements that in the base followed the early return, are all `function` declarations — hoisted then, hoisted now — so moving them ahead of the early return is inert; I verified by inspection that the base had no executable statement after `#167` other than the JSX. `npx tsc -p tsconfig.json --noEmit` exits **0**, which mechanically rules out any use-before-declaration the reordering could have introduced.

**The session object — settled, identity not depended on.** `session` occurs seven times in `src/App.tsx`: the call at `:133` and six destructurings at `:143, :157, :185, :215, :233, :263`. It appears in no dependency array, no memo key, no JSX attribute, no spread, no ref and no context — and `AppSession` declares no hook other than `useWorkspaceSession`, so it contains no dependency array at all. The six slice objects are likewise destructured in place and never held. The six state hooks' return objects are destructured in the same statement that calls them. **All properties of all seven return objects are shorthand** — I checked mechanically that no return object contains a single `name: value` property other than the six slice keys — so every value the view receives is literally the base's own binding. Callback identity is unchanged: the whole component contains exactly **two** `useCallback`s (`handleTreePublication` and `getPreparationEpoch`), both with `[]` deps, both moved byte-identical; every other handler was a plain `function` declaration recreated per render and still is.

**Module state — settled.** `nextUiModelPublicationGeneration` has exactly one declaration and one use on each side: base `src/App.tsx:291` / `:842` → candidate `workspaceSession.ts:125` / `:420` (`++nextUiModelPublicationGeneration` inside `commitModel`). Still one module-level `let`, still not exported, still colocated with its only writer. A repository-wide grep finds no other reader anywhere, in tests or elsewhere — the only other hits are the run's own records. Its stated purpose (generations must not repeat across same-page `App` remounts) is preserved: an ESM module singleton is instantiated once per program in both arrangements, and a remount of `App` resets neither.

**Closures across the new boundary — settled.** The asynchronous handlers are byte-identical and now declared in `useWorkspaceSession`'s body, where the destructured hook results of *that same invocation* are in scope; they therefore capture the same render's values they captured before. The refs they read after an `await` are the same objects, not copies: `useRef` returns a stable object, the hook returns it under a shorthand key, and the destructuring copies the reference. No ref is spread or rebuilt anywhere in the new plumbing. Because the hooks form an acyclic import graph and no new module imports `App.tsx`, and because none of the modules the session imports has a top-level side-effect statement, the import regrouping cannot shift observable module-evaluation behaviour either.

---

## The lane's two tools: what their PASS does and does not mean

I ran each once, on the frozen candidate.

- `b2_move_audit.mjs 7c6784d95… 5d2e261e0…` → **exit 0, PASS**. 62 top-level and 177 `AppSession` statements matched, and **all 239 matched as `identical`** — not one needed the `reindented` or `CHANGED` fallback. 13 effects in one function in base order. 25 statements with no base counterpart, which I read in full: 4 re-export declarations, the `WorkspaceSession` type alias, `const session = useWorkspaceSession()`, six `AppSession` destructurings, six hook calls with their destructurings, six hook `return`s and the session `return`. Nothing else.
- `b2_order_check.mjs 7c6784d95… 5d2e261e0…` → **exit 0, PASS** for `AppSession` and all seven hooks.

**Is what they count as identical sound?** Yes, with two limits worth stating.

- Ignoring a leading `export` is applied only to *top-level* statements, never to body statements. That is correct: a moved top-level helper must gain one, and a body statement cannot.
- The `reindented` status dedents each line, which would mask a whitespace change inside a multi-line template literal. Moot here — nothing matched as `reindented`.
- **The move audit does not check order** except for effects, and section 4 (new statements) does not increment its failure count. So a PASS means "every base statement arrived unedited somewhere, and the effects are in order" — not "the candidate added nothing" and not "render order is preserved". A human must read section 4; I did.
- **The order check verifies monotonicity *within* each function, not *across* them.** Every one of its seven hooks can pass while statements have been reordered wholesale across hook boundaries — which is exactly what happened. Its PASS is therefore necessary, not sufficient, and the cross-function question is the one I settled above from the code.

Both limits are honestly disclosed in the manager's return and in `B2_RENDER_ORDER_FINDING.md`, which is why the manager wrote the two findings by hand. Having read both findings after doing my own work, I confirm them. One imprecision, non-actionable: the render-order finding's point 4 enumerates the statements that changed sides as "#32 to #46", which sweeps in `#35` and `#39`, both of which in fact stayed on the same side; every statement in that range is a `useState`/`useRef` declaration regardless, so the conclusion is unaffected.

---

## Residual risk and consequences — none actionable before merge

1. **The six state hooks are exported and each returns its full setter set**, including `setModel`, `setResult`, `setUndoStack`. The session object exposes none of these, which is what check 4 asks; and calling e.g. `useModelSessionState()` elsewhere would yield an independent, disconnected cell rather than a route to mutate the session's model — so no mutation can bypass the applier today. The only thing standing between B3 and a component calling a state hook directly is the doc-comment convention "Called only by the session". Worth a lint rule or a non-exported module boundary when B3 hands slices to components. Severity **trivial**; **not actionable before merge**.
2. **Benchmark provenance is now incomplete.** `e2e/ui-foundation/resource-lifecycle-source.benchmark.ts:144` lists `src/App.tsx` in `candidateExecutedFiles`; the session code it used to execute now lives in `workspaceSession.ts` and the six state hooks. The file is in the **canvas lane's** write scope, so B2 correctly did not touch it, and the manager has already routed it to the canvas lane through ROOT. Confirming it here so it is not lost: it should land before the benchmark's next provenance record is relied on. Severity **minor**, owned by another lane, **not actionable against B2**.
3. **No new test pins the new boundary.** The slice adds no test, which is legitimate for a move, but nothing mechanically defends the hook call order, the shorthand-only return, or "no model setter leaves the session" against a future edit. A small test asserting the session object's key set would be cheap insurance for B3. **Not actionable before merge.**
4. The carried items in the manager's return — four handler asymmetries, the dead cells (`r3JourneyState`, `modelPublicationGenerationRef`, `restoredSolveJob`, `handleApplyNextQueuedIntent`, `HardDrive`, `operationIntentDisplayRef`, `isBlankAuthoringModel`, `countStatus`), `systemDark` and `setTreePublication` returned unused, the two stale comments — I verified the slice **changed none of them**, so per the brief I report none. Specifically: `r3JourneyState` is read nowhere at the base and nowhere now; `systemDark` is read only by `resolvedTheme`, which moved into the same hook with it; `setTreePublication` is read only by `handleTreePublication`, which moved with it; `modelPublicationGenerationRef` is written once and read never, before and after.

---

## What I did not check

- I ran **only** `npx vitest run src/App.test.tsx` (218 passed) and `npx tsc -p tsconfig.json --noEmit` (exit 0), plus the two lane tools. I did **not** run the full unit suite, `npm run build:desktop`, either Playwright lane, the DEC-025 evidence sweep, the registered checks or a dev server. The manager's claims of 1,190 tests, exit-0 build, 374 passed + 20 skipped (source lane) and 53 passed (dist lane) are therefore **unverified by me**; ROOT's sweep covers them on the frozen candidate.
- 74 of the 75 unit test files were not run by me.
- I verified equivalence by reading code, not by executing the application; I took no screenshot and made no observation of rendered output.
- I did not audit the run's coordination records beyond check 9, and did not verify the merge commit `49ed6f85b`'s content beyond confirming that the three-dot diff excludes main's own changes and that the `App.tsx` and `App.test.tsx` blobs are what the brief states.
- I did not assess the four carried handler asymmetries as defects on their merits; I confirmed only that this slice moved them unchanged.
- I claim no usability, conformance, performance or lifecycle acceptance; this is a code review, not acceptance.

**The slice is suitable for manager fan-in.** Remaining risk is the unverified-by-me check set above and the four items under residual risk, none of which blocks the merge gate.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
