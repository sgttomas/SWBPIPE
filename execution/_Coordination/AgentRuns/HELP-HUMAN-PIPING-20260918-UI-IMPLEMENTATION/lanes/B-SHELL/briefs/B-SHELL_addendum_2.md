# Sealed addendum 2 to the B-SHELL lane brief — slice B2F: project-handler repairs

Sealed by ROOT (HELP_HUMAN, Agent 0) on 2026-09-19T04:11Z. It adds one slice to the shell lane, to run after slice B2 has merged and before slice B3. Everything in the lane brief and addendum 1 still binds.

## Why

Slice B2 moved the session's handlers without changing them. Its implementer reported asymmetries it was told to leave alone. ROOT read the base (`{WORKING_ROOT}/apps/desktop/src/App.tsx` at `4dcab7505`; after B2 the same statements are in `src/features/workspace/workspaceSession.ts`, byte for byte) and confirms each reads as described. Whether each is visible to the user is for a failing test to show. They are behaviour changes, so they get a slice of their own, named, tested and reviewed, and they are not folded into a move.

1. **Opening a project keeps the previous project's operation diagnostics.** `handleOpenProject` resets the batch cells (`setBatchOutcomes({})`, `setBatchMessage(null)`) and not `operationOutcomes` or `operationMessage`. The blank-project handler and the fixture reset clear all four (base lines 1755 to 1756 and 1898 to 1899). Reported effect: the earlier project's operation diagnostics stay in the issue count and the Issues drawer.
2. **Listing projects can release the busy state early.** `handleListProjects` takes no request number and clears `projectBusy` unconditionally in `finally`. Every other project handler clears it only when its own request is still the latest. A list that finishes during a pending save re-enables the project controls. The native menu can start one, because `runMenuCommand` does not look at `projectBusy`.
3. **An open that finds nothing, or fails, degrades the project that is still open.** `handleOpenProject` and `handleCreateBlankProject` invalidate `ruleRevisionGate` and set `modelHashIntegrity` and `projectEnvelopeHashIntegrity` to null before their first `await`. When the open returns nothing (`open_missing`) or throws, the project that is still open keeps the nulled cells. `ProjectValidationPanel` reads a null cell as `open_verification_not_run_this_session`, so a recorded `mismatch_review_required` disappears from a project that is still open. An in-flight rule-check revision is also dropped, with `ruleCheckAggregate` already set and `analysisRun` not revised. This one touches result integrity, so it is first in priority.
4. **The apply path resets by hand.** `handleApplyIntent` clears the computed cells itself and does not call `clearComputedModelState`, so it is the one model-changing path that never invalidates `ruleRevisionGate`. The implementer judges it inert, because `commitModel` advances the request epoch and the model revision that the rule-check guard also reads. This item is optional: make the path use the shared reset only if reading and a test show that nothing observable changes; otherwise leave it and record why.

## The slice

- **Test first.** For each item write a test that fails on the merged B2 tree and shows the user-visible effect (the issue count and drawer content for item 1; the enabled state of the project controls during a pending save for item 2; the validation panel's integrity status after an open that finds nothing, for a project opened with a recorded mismatch, for item 3). If a test cannot show an effect, stop on that item, change nothing for it, and report what you found.
- **Smallest repair.** Item 3: a failed or empty open leaves the open project's integrity cells, its rule revision gate and its in-flight revision as they were; the invalidation and the nulling belong where the open commits (beside `advanceProjectSession()`), and the verified-match path for a successful open must read exactly as it does today. Item 1: open resets the same cells the blank-project handler resets. Item 2: the list must not release a busy state it does not own. Do **not** repair it by advancing `projectRequest`: that would make a pending save's `stillCurrent` false and drop the save's result, which is worse than the defect. Choose between an ownership count for the busy state and a guard in `runMenuCommand` that matches what the buttons already do, or both; say which and why.
- **Named semantic changes.** List each in the return: what changed for the user, and the test that holds it.
- **Untouched:** the one operation route, the generation gate classes and every guard's condition (item 3 moves where one invalidation is called, and that move is a named change), the solve-input basis, `commitModelAfterSolveInvalidation`, `clearComputedModelState`, the Current and Historical designation, undo and redo, persistence formats. No test weakened or deleted. No copy change.
- **Checks:** build, the unit suite, both Playwright lanes through the lock on the final candidate.
- **Review:** ROOT dispatches the independent Opus code review over the whole diff. No fidelity review: the slice draws no surface.

## From the independent review of B2

The review returned PASS with no actionable finding: `{RUN}/lanes/B-SHELL/reviews/B2-REVIEW_RETURN.md` (SHA-256 `97f4adeba1454a37655f0a46fd8be697f6b724c9a4e7169d9e178e03c2f69089`). Read it; it is the best account there is of what the extraction did and did not change. Three of its residual items are yours, and none of them belongs in B2F's diff unless it says so here.

- **No component calls a state hook.** The six state hooks are exported with their full setter sets, and only a comment says "Called only by the session". When B3 adds `WorkspaceSessionContext` and hands slices to components, make that mechanical: a lint rule, a module boundary or a test that fails when a file other than `workspaceSession.ts` imports a `*SessionState.ts` hook. Put it in B3's first commit.
- **The session's key set.** Add a small test that pins the session object's six slices and the eleven setters it exposes, and fails if a model, results, history, undo, redo, operation-outcome or project setter ever leaves the session. It belongs with the same B3 commit.
- **Your render-order finding, point 4.** The range "#32 to #46" sweeps in #35 and #39, which stayed on the same side. The conclusion stands. Correct the file by striking and restating, not by deleting.

The fourth item, the benchmark's executed-file list, is the canvas lane's and is already with it.

## Two changes by others in `styles.css`, which stays your file

- **Canvas furniture.** ROOT granted the canvas lane a scoped exception (its sealed addendum 2, §3). In its slice C1b it edits `src/styles.css` for the colours of the `.viewport-shell`, `.viewport-scale-bar` and `.viewport-fallback` rules, for the two mirrors `--ui-canvas` and `--ui-viewport-selection-geometry`, and for the one comment sentence that describes those two. Leave those rules and those two variables alone in B2F and B3. If B3's layout needs a change to a `.viewport-*` rule, return it to ROOT.
- **Token adoption.** The design system's V1.4 (`tokens.json` 1.3: a control-boundary token, `border.control`, and a lifted `text.disabled`) merges on its own branch. ROOT then dispatches a small slice of its own, outside your lane, that adopts 1.3 in the product: `src/design/**`, the generated `src/tokens.css`, and in `styles.css` only `--ui-border`, `--ui-disabled-text` and the block's comment. That resolves the review's observation F3 for you. ROOT tells you when it has merged; merge `origin/main` at your next clean point. B3 builds its controls from `border.control`, not from `border.strong`.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
