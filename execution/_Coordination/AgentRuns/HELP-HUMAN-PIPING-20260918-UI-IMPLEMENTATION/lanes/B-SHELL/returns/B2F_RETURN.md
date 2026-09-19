# Retained return — slice B2F (B-SHELL manager), 2026-09-19

Retained by ROOT. What follows is the agent's final message as the host stored it, with the machine's repository path replaced by `{REPO_ROOT}`; nothing else is changed. The message exactly as the host stored it has SHA-256 `42647a312a20eeb9197a8980c1c9f025158a9e4aba84d4f263d08dc3a15a657b` (137 lines), by this script.

---

# Slice B2F return: project-handler repairs (addendum 2)

Slice B2F is frozen for review at `36e79602eac5751e518898d88869a71703b85c03`. Items 1 to 3 are repaired, each with a failing test first. Item 4 is left as it is. I start nothing further until ROOT resumes me.

This is B-SHELL (WORKING_ITEMS, Type 1), lane `codex/swbpipe-b-shell-20260918`, returning to ROOT. By my own statement I ran as Claude Fable 5.1 (`claude-fable-5-1`). Placeholders are as in the lane brief; `{DESKTOP}` is `{WORKING_ROOT}/apps/desktop`.

One implementer child did the work: B2F-HANDLERS, a TASK (Type 2) agent that reports itself as Claude Fable 5.1. I launched it in the foreground. It worked for about 31 minutes, and its return came back as the result of my own `Agent` call. The call was not cut off, and nothing went through ROOT. I dispatched no reviewer.

## Candidate and range

- **Frozen candidate:** `36e79602e`. That commit is records only; the product tree is frozen at `572c4a34b23fbbe09f31e2e41ae03910c657d2ed`.
- **Base:** `origin/main` `64f86e17f`, which is also the merge base, because `origin/main` is merged into the lane.
- **Diff range:** `64f86e17f..36e79602e`.
  - Product: two files under `{DESKTOP}/src`.
    - `features/workspace/workspaceSession.ts` changes by +20 −7, inside `handleCreateBlankProject`, `handleOpenProject` and `handleListProjects` only.
    - `App.projectHandlers.test.tsx` is new, with 445 lines and 7 tests.
  - Records: files under `{RUN}/instances/B-SHELL/` only.
  - The range also shows five files under `{RUN}/lanes/B-SHELL/`. They come from ROOT's three commits that were already on the lane (`93046b051`, `9651fd421`, `5f70ad178`); none of my commits touch that folder.

**Commits, in order:**

| Commit | What |
|---|---|
| `66a0bf103` | Merge of `origin/main`, my first git step; no conflict |
| `5e5af35dc` | Records: retained launch brief, sealed child brief `briefs/B2F-HANDLERS.md` (`73952ad0…`), index row, lane log |
| `ef81fc32e` | The seven tests, failing on that tree by design, so a reviewer can see red then green |
| `572c4a34b` | The repairs |
| `36e79602e` | Records: the child's return retained byte for byte (`returns/B2F-HANDLERS_RETURN.md`, `5dee58e7…`), returns index, lane log |

## Checks

| Check | Command | Tree | Result |
|---|---|---|---|
| Launch brief and four instruments | `shasum -a 256` | records branch and lane | all five hashes match |
| Baseline after the merge | `npx tsc -b`, `npx vitest run` and `npm run build`, from `{DESKTOP}` | `66a0bf103` | type check exit 0; 75 files, 1,190 tests passed; build exit 0 |
| Fail first, my own re-run | `npx vitest run src/App.projectHandlers.test.tsx`, with `workspaceSession.ts` set back to the lane-head blob `d9956a17…` | tests and unrepaired source | 7 failed of 7, each with the failure text the child quotes; the repaired file was then restored and its hash checked |
| Build | `npm run build:desktop`, from `{WORKING_ROOT}` | candidate | exit 0 |
| Unit suite | `npm run test:desktop`, from `{WORKING_ROOT}` | candidate | 76 files, 1,197 tests passed (1,190 plus the 7) |
| Playwright, source lane | `PLAYWRIGHT_WORKERS=1 sh {RUN}/tools/with_e2e_lock.sh npm run test:e2e`, from `{DESKTOP}` | run once by the child, on the tree whose two file hashes equal the committed blobs | 374 passed, 20 skipped, exit 0 |
| Playwright, dist lane | the same with `npm run test:e2e:dist` | same | 53 passed, exit 0 |
| Registered checks | harness `self-check`; `python3 -m pytest -q tools/practitioner_harness`; `validate_claims_language.py`; `validate_path_anchors.py`, from `{REPO_ROOT}` | final records tree | exit 0 with no finding naming this lane; 379 passed; VALID, 335 files; PASS |

- I did not run the Playwright lanes again, under the launch brief's usage-limit clause. ROOT's DEC-025 sweep runs them on the frozen candidate.
- No existing test was changed: `git diff --stat origin/main -- src/App.test.tsx src/App.deadControls.test.tsx src/design e2e`, from `{DESKTOP}`, is empty.
- No tolerance, oracle, benchmark limit or frozen value was touched.
- No listener is left on 5174 or 5175, no browser-test lock is held, and the worktree is clean.

## Tests that carry the control layer (addendum 3, B2F)

Each repair's test was written first and fails on the unrepaired tree. All seven are in `src/App.projectHandlers.test.tsx`.

- **Item 1:** "drops the earlier project's operation diagnostics from the issue count and the drawer when a project is opened".
- **Item 2:**
  - "keeps the project controls disabled through a native list command while a save is pending, then lands the save";
  - "leaves a save's busy state alone when a listing that started first settles during the save".
- **Item 3:**
  - "keeps a recorded model-hash mismatch when a later open finds nothing";
  - the same "… when a later open fails";
  - the same "… when a blank create fails";
  - "lets an in-flight rule-check revision land when an open finds nothing".
- **Item 4:** no test and no change.
  - The child's reading matches mine: the missing `ruleRevisionGate` invalidation is inert.
  - `commitModel` advances `uiModelRevisionRef` and, through the layout effect, the model revision. `currentSolvedResultRef` changes too.
  - An in-flight rule-check revision therefore fails three other conditions of its guard.
  - The shared reset would add a second `invalidateReportPackageComputedState()` call, and showing that to be unobservable needs more than a short reading.

## The repairs, and one departure from my own brief

- **Item 3:**
  - In `handleOpenProject` and `handleCreateBlankProject`, three statements moved from before the first `await` to directly before `advanceProjectSession()`: `ruleRevisionGate.current.invalidate()`, `setModelHashIntegrity(null)` and `setProjectEnvelopeHashIntegrity(null)`. Their relative order is unchanged.
  - From the commit point on, a successful open reads as before.
  - No `stillCurrent` condition changed, and neither gate class changed.
- **Item 1:** `setOperationOutcomes({})` and `setOperationMessage(null)` follow `setBatchMessage(null)` in the open handler, at the blank handler's position.
- **Item 2:** `handleListProjects` alone changed.
  - It returns at once while `projectBusy` is true.
  - It records `projectRequest.current` without advancing it, and releases in `finally` only if that number is unchanged.
  - A pending save's `stillCurrent` is therefore unaffected.
  - I read the orderings and found no path that leaves the busy state stuck. They were a list followed by a request-numbered handler in either settling order, two lists, and a superseded handler. The latest request-numbered handler always releases in its own `finally`.
- **Departure:**
  - My brief recommended a `runMenuCommand` guard on the five `file.*` commands. The child built it and an existing safety-net test failed.
  - That test is "discards a pending batch when another model opens, and ignores delayed save metadata after a model replacement" (`src/App.test.tsx`, near 16599 to 16627). It fires a native `file.save-local`, then a native `file.open-local`, and requires the open to supersede the pending save.
  - I confirmed this by reading the test. No existing test may change, so the guard was dropped and `runMenuCommand` is byte for byte the base's.

## Controls

- **Controls added:** none. The slice adds, removes and restyles no control, and `src/App.tsx` is untouched.
- **Contrast not yet checked:** none, because no control was touched.
- **Appearance left for the closing pass:** none, because the slice draws no surface.

## Semantic changes, named

1. **Open and operation diagnostics.**
   - Today an open keeps the previous project's operation diagnostics in the issue count and the Issues drawer, and keeps the operation message.
   - Now it clears them, as "New blank" does.
   - Held by the item 1 test.
2. **Failed or empty open.**
   - Today an open that finds nothing or fails, or a blank create that fails, nulls the open project's integrity cells. A recorded `mismatch_review_required` then reads `open_verification_not_run_this_session`.
   - Now the cells stay as they were.
   - Held by the three mismatch tests.
3. **Where the rule-revision invalidation is called.**
   - Today such an open or create drops an in-flight rule-check revision: `ruleCheckAggregate` stays set while `analysisRun` is not revised.
   - Now the revision lands.
   - Held by the in-flight test.
4. **Consequence of the same move.**
   - Today the open project's integrity status disappears while an open is pending, and a superseded open nulls the cells.
   - Now the status stays until the new project commits.
   - No dedicated test covers the pending window.
5. **List while busy.**
   - Today a list can start during another project request. On settling it re-enables the controls and publishes its message over the pending request.
   - Now it does not start.
   - Held by the first item 2 test.
6. **List settling after a later request starts.**
   - Today it releases that request's busy state.
   - Now it releases only when no request-numbered handler has started since the list began.
   - Held by the second item 2 test.

## What I bring back, not decided here

1. **The native menu is not disabled while a project request is busy, and the safety net pins that reachability.**
   - A native open during a pending save supersedes the save and drops its result.
   - Addendum 2 called dropping a save's result "worse than the defect", yet that test requires it for the menu path.
   - If the owner wants the native menu to match the buttons, that test must change in a slice that names the change. Disabling native items is `src-tauri/**`, which is outside this lane.
   - Proposal: leave it as it is for this tranche and list it for the owner.
2. **`handleCreateProject` and `handleSaveProject` also null the two integrity cells before their first `await`, and do not restore them when the request fails or is superseded.**
   - This is the same class of defect as item 3, on the project that stays open. It is outside addendum 2's scope.
   - Proposal: a small follow-up of the same shape, test first, if ROOT wants it. It would be the same move, to where the save or create lands.
3. **A list that settles during a later request still publishes its message, operation and index over that request until the request lands.** Addendum 2 asked only about the busy state.
4. **A same-tick edge:** the list's guard reads the render's `projectBusy`, as the buttons do. Two commands dispatched in one tick would both see the stale value. Closing that needs a ref outside the handler bodies.
5. **Host note:** my file-writing tool refuses paths outside the session's own worktree, and the child's did the same. We both wrote into `swbpipe-wt3` through the shell. My records and the child's product edits were checked by hash afterwards.

## Next slice

B3, the shell, comes next. Its first commit carries the guard that no file other than `workspaceSession.ts` imports a `*SessionState.ts` hook, and the session key-set test (addendum 2). In the same commit I correct `B2_RENDER_ORDER_FINDING.md` point 4 by striking and restating. B3 also takes the `tokens.json` versus `statusLabels.ts` agreement test, and the two comment corrections in `styles.css`. Before starting I may return a proposal for a different cut of B3 to B7.

I did not push, open a pull request, rewrite history or touch another worktree's files. I claim no usability, conformance or performance acceptance. PDU-045 and PDU-046 remain holds.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
