# Slice B2: the manager's finding on render order

Lane B-SHELL's manager (WORKING_ITEMS, Type 1), 2026-09-19. ROOT asked the manager to read this itself before committing stage 2, because it is the one thing the byte-for-byte move audit cannot see and it sits on the solve-input basis and the generation gates. It is a named item for the independent reviewer of slice B2. Candidate read: stage 2, commit `a71536596`; base `7c6784d95`. Paths are relative to `{WORKING_ROOT}/apps/desktop`.

## What changed in the order of execution

At the base every statement of `AppSession` runs top to bottom, numbered #1 to #177 by `tools/b2_move_audit.mjs`. From stage 2, `AppSession` first calls six state hooks, unconditionally and in this order: chrome, model, selection, results, operations, project. Inside each hook the base order holds. So a render now runs the cells grouped by domain and then the rest of `AppSession` in base order. `tools/b2_order_check.mjs` prints the exact sequence and checks the three mechanical facts: base numbers increase inside every function; no hook call sits under a condition, a loop or a nested function; no hook is called after the early return.

Four of the moved statements are not declarations but render-time writes to refs. They now run earlier in the render pass, inside `useResultsSessionState`:

| # | Statement | Base line |
|---|---|---|
| 24 | bump `resultBasisRef` when `result` changed | `src/App.tsx:576` |
| 27 | bump `analysisBasisRef` when `analysisRun` changed | `:583` |
| 107 | `currentSolvedResultRef.current = currentSolvedResult` | `:708` |
| 108 | `currentInputManifestRef.current = inputManifest` | `:709` |

## Finding: not observable

1. **What they write does not depend on where they run.** Each compares or assigns a state value (`result`, `analysisRun`, `inputManifest`) or a value derived from one in the same render (`currentSolvedResult`). State is constant within a render, so each statement writes the same value wherever it sits in the pass.
2. **Who reads those refs during render.** Only two statements: #25 `ruleCheckRunBasis` and #28 `dormantOutputBasis`. Both stay in `AppSession` directly after the six hook calls (candidate `src/App.tsx:364` and `:372`), so both still read after both bumps, as at the base (`:580` and `:587` after `:576` and `:583`). The manager listed every occurrence of the four refs at the base and in the candidate; there is no other render-time reader.
3. **Every other reader runs after render.** They are the guard closures `renderedBasisIsCurrent` and `stillCurrent` in `handleRuleCheckAggregate` and `isCurrent` in `handleSaveReportPackage` (candidate `src/App.tsx:820`, `:840`, `:1902`), evaluated in an event handler or after an `await`. A render pass is synchronous, so where inside it the ref was written cannot change what they see.
4. **Statements that changed sides.** Those that ran after #24 and #27 at the base and run before them now are chrome, model and selection declarations (#39, #74 to #96 less the effects, #98, #99, #112, #114). Those that ran before #107 and #108 and run after them now are operations and project declarations (~~#32 to #46~~ #32 to #34, #36 to #38 and #40 to #46, #51, #58 to #73, #97, #100 to #102). Corrected 2026-09-19 after the independent review of B2: the struck range swept in #35 `selectedReviewTarget` and #39 `modelHash`, which are results and model cells and stayed on the same side. The conclusion is unchanged. All are `useState`, `useRef`, `useCallback` or `useMemo` declarations; none reads one of the four refs.
5. **The memos and the lazy initializers.** The order of four memos changed (`activeModelIndex` and `selectedPipeRefs` now run before `comparison`; `effectiveHiddenKeys` still runs after all three). `modelIndexFor` writes only its own `WeakMap` cache keyed by the model, and `buildPreviewComparison` is pure, so their order is not observable. The initializers that read the environment (`readUiPreferences`, `matchMedia`, `window.innerWidth`) are all in the chrome hook in base order; the others (`emptySelection`, `new Set`, `initialSolveJob`, `initialOperationEngineStatus`) are pure.
6. **Effects.** No effect moved in stage 2. All 13 are in `AppSession` in base order, byte for byte; React runs effects in the order they are declared in the component, and a custom hook that declares none adds none.

The manager therefore kept the four statements in `useResultsSessionState`, where the cells they write live, and did not take the alternative ROOT offered (leaving them in `AppSession` at their base position).

## What stage 3 changes about this

Stage 3 moves the six hook calls and every remaining statement up to #165, with #168 to #176, into `useWorkspaceSession`, which `AppSession` calls first. A custom hook is a function call inside the same component, so React sees the same component with the same hooks in the same order as after stage 2, and the 13 effects stay one sequence in base order. The manager re-runs both tools on the stage-3 candidate.

## Stage 3, as built

Both tools pass on the stage-3 tree. `tools/b2_order_check.mjs` prints for `useWorkspaceSession`: six new statements (the six hook calls, in the same order as at stage 2), then `25 28 76 80 86 113 115 … 165 168 … 176`, then the new `return`; and for `AppSession`: seven new statements (the session call and six destructurings), then `166 167 177`. `tools/b2_move_audit.mjs` finds all 13 effects in `useWorkspaceSession`, in base order, byte for byte, and no effect anywhere else. So the render order of stage 2 is the render order of stage 3, with one function boundary added around it; #168 to #176 are function declarations, hoisted at the base and hoisted now. The companion finding on the identity of the session object is `B2_SESSION_OBJECT_FINDING.md`.

Where the statements of this finding are in the stage-3 tree (`e2c8f277d`): the four render-time writes are in `src/features/workspace/resultsSessionState.ts` at `:43`, `:45`, `:67` and `:68`; the six hook calls end at `src/features/workspace/workspaceSession.ts:188`, `:202`, `:212`, `:241`, `:265` and `:279`; the two render-time readers follow them directly, #25 `ruleCheckRunBasis` at `:281` and #28 `dormantOutputBasis` at `:289`; the guard closures that read the refs after render are `renderedBasisIsCurrent` at `:734` and `stillCurrent` at `:753` in `handleRuleCheckAggregate`, and `isCurrent` at `:1813` in `handleSaveReportPackage`.

The base `AppSession` body has five render-time writes to refs, and the four above are the only ones whose place in the render pass changed. The fifth is #163, `runMenuCommandRef.current = runMenuCommand;` (base `src/App.tsx:2375`): it stays directly after its `useRef` (#162) and before the two effects that follow it (#164, #165), in base order inside `useWorkspaceSession` (`src/features/workspace/workspaceSession.ts:1952` to `:1953`), and its only reader is the native-menu listener of #164, which runs after render. The manager listed the five by searching the base body for statements at body level that assign to or test a ref's `.current`.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
