# Retained return — lane B-SHELL, slice B2 (manager: Claude Fable 5.1 requested; the implementer child reports itself as Claude Fable 5.1)

Retained by ROOT. What follows is the child's final message as the host stored it, with the machine's repository path replaced by `{REPO_ROOT}`; nothing else is changed.

---

# Slice B2 return: state extraction, no behaviour change

Slice B2 is complete and frozen for review, with every check green. This is B-SHELL (WORKING_ITEMS, Type 1), lane `codex/swbpipe-b-shell-20260918`, returning to ROOT. I start nothing further until ROOT resumes me.

One implementer child did the work: B2-STATE, a TASK (Type 2) agent that reports itself as Claude Fable 5.1 (`claude-fable-5-1`). It ran in three stages with a return at each, and I committed between stages. No reviewer was dispatched.

## Candidate and range

- **Frozen candidate:** `5d2e261e0df39c18a71d897902bf5d3a047db175`. The working tree is clean. That commit is records only; the product tree is frozen at `e2c8f277d58b8b2d7b95ce4527d80aca50c6f8cc`.
- **Reviewed against:** `origin/main` `8e4c5df6ec84928ca343224e5244ccdae5771cd4`, which is also the merge base because `origin/main` is merged into the lane.
- **Diff range:** `8e4c5df6e..5d2e261e0`.
  - Product: 15 files under `{DESKTOP}/src`, 3,715 insertions and 2,941 deletions. They are `App.tsx` and 14 new modules in `src/features/workspace/`.
  - Records: only files under `{RUN}/instances/B-SHELL/`.
  - The range also shows five files under `{RUN}/lanes/B-SHELL/`. They come from ROOT's launch commit `7c6784d95`; none of my commits touch that folder.
- **Base argument for both tools:** `7c6784d95ab83bd958d96938c978153b36734064`. Its `src/App.tsx` is the same blob as `origin/main`'s (`fa5e15c9…`), so the audit base and the review base agree on the one file the audit compares.

## Commits, in order

| # | Commit | What |
|---|---|---|
| 1 | `28f956d9555e13e78d181d63d819c71c2efeda6e` | Records: sealed brief `briefs/B2-STATE.md` (`a938b135…`) and `tools/b2_move_audit.mjs` |
| 2 | `a259fd69162c2f0a73da495aa8c5efd4c3030dfb` | Stage 1, moves only: 39 top-level declarations into seven React-free modules |
| 3 | `f7cd266adf645ed7afc87bcbb232a5eb9726ec9c` | Records: stage-1 return; audit tool revision 2 (report only) |
| 4 | `a715365962b521ef17f5ea2dc8a66738bccf3638` | Stage 2, moves only: 108 `AppSession` statements into six state hooks |
| 5 | `b0246701d572d133021149c6ab3fffdf39d9b5b8` | Records: stage-2 return; render-order finding; `tools/b2_order_check.mjs` |
| 6 | `49ed6f85bf2e99615d3187f0c55e92c1afc49d50` | Merge of `origin/main` `8e4c5df6e` (PR #802), at ROOT's direction, no conflict |
| 7 | `38a9633d5aa77c1e4e2ec8fc846ca03d883857ac` | Records: `LANE_LOG.md` |
| 8 | `e2c8f277d58b8b2d7b95ce4527d80aca50c6f8cc` | Stage 3: `useWorkspaceSession` in `workspaceSession.ts`; `AppSession` becomes the view |
| 9 | `5d2e261e0df39c18a71d897902bf5d3a047db175` | Records: stage-3 return (`69ed8f75…`), session-object finding, log |

All three stage commits hold moves only. No moved statement was edited, so there is no separate edit commit.

## What the slice did

- `src/App.tsx` goes from 4,356 to 1,574 lines.
- `AppSession` is now the session call, six slice destructurings, then base statements #166 (`showInAppMenuBar`), #167 (the early return) and #177 (the JSX), each byte for byte.
- `useWorkspaceSession` (2,246 lines) calls the six state hooks in the order chrome, model, selection, results, operations, project.
  - It then holds 66 base statements in base order: three derived values, `runMenuCommandRef` and its render-time assignment, all 13 effects, and all 48 functions.
  - It returns 129 shorthand names in six slices.
- No setter of a model, result, operations or project cell leaves the session.
- There is no React context in this slice.
- `App.tsx` re-exports the names the tests import.

## Checks, and which run covers which tree

| Check | Command | Tree | Result |
|---|---|---|---|
| Build | `npm run build:desktop`, from `{WORKING_ROOT}` | stage-3 working tree; its eight file hashes are the blobs of `e2c8f277d`; the frozen candidate adds records only | exit 0 |
| Unit suite | `npm run test:desktop`, from `{WORKING_ROOT}` | same tree | 75 files, 1,190 tests passed, the same counts as at the base |
| Move audit | `node {RUN}/instances/B-SHELL/tools/b2_move_audit.mjs 7c6784d95ab83bd958d96938c978153b36734064 [<head-rev>]` | working tree, then `e2c8f277d`, then `5d2e261e0` | PASS on each |
| Call-order check | `node {RUN}/instances/B-SHELL/tools/b2_order_check.mjs 7c6784d95ab83bd958d96938c978153b36734064 [<head-rev>]` | the same three trees | PASS for `AppSession` and all seven hooks |
| Playwright, source lane | `PLAYWRIGHT_WORKERS=1 sh {RUN}/tools/with_e2e_lock.sh npm run test:e2e`, from `{DESKTOP}` | run by the child on its stage-3 working tree, same eight hashes | 394 tests: 374 passed, 20 skipped |
| Playwright, dist lane | `PLAYWRIGHT_WORKERS=1 sh {RUN}/tools/with_e2e_lock.sh npm run test:e2e:dist`, from `{DESKTOP}` | same, run by the child | 53 passed |
| Unused symbols | one-off `tsc --noEmit --noUnusedLocals --noUnusedParameters` with no config edit | same, run by the child | only five symbols, all unused at the base |
| Registered checks | harness self-check; `python3 -m pytest -q tools/practitioner_harness`; `python3 tools/validation/validate_claims_language.py`, from `{REPO_ROOT}` | self-check and claims lint on the committed frozen candidate; pytest on the final records tree before its commit | self-check exit 0, with no finding naming this lane; 379 passed; VALID, 335 files |

- **Move audit detail:**
  - All 62 top-level statements are identical.
  - All 177 `AppSession` statements are identical: 3 in `AppSession`, 108 in the state hooks, 66 in `useWorkspaceSession`.
  - All 13 effects sit in one function in base order.
  - New statements: 5 top-level (the four re-exports and the `WorkspaceSession` type) and 20 in bodies, all plumbing.
- **Playwright:** as ROOT allowed, I did not re-run either lane. ROOT's DEC-025 sweep runs them on the frozen candidate.
- **Earlier stages:** the build and unit suite gave the same results on the stage-1, stage-2 and post-merge trees.
- **"`App.test.tsx` unchanged":** this is read against `origin/main`. `git diff --stat origin/main -- src/App.test.tsx src/App.deadControls.test.tsx src/design e2e`, from `{DESKTOP}`, is empty.
- **Tests and frozen values:** no test was changed. No tolerance, oracle, benchmark limit or frozen value was touched.

## Semantic changes, controls, screenshots

- **Semantic changes:** none.
- **Controls left absent or disabled:** none. The slice touches no control, and the JSX is byte for byte the base's.
- **Screenshots:** none. The slice draws no surface.

## Two findings for the independent reviewer

Both are my own readings, in `{RUN}/instances/B-SHELL/`.

1. **`B2_RENDER_ORDER_FINDING.md`**
   - Grouping cells by domain moved four render-time ref writes (#24, #27, #107, #108) earlier in the render pass. This is not observable.
   - The values they write are constant within a render.
   - The only render-time readers are #25 and #28, which still run after both bumps (`workspaceSession.ts:281` and `:289`).
   - Every other reader is a guard closure that runs after render (`:734`, `:753`, `:1813`).
   - The fifth render-time write, #163, did not move.
   - All effects are in one function in base order.
2. **`B2_SESSION_OBJECT_FINDING.md`**
   - No consumer depends on the identity of the session object, of a slice, or of a state hook's return object.
   - `session` occurs seven times in `src/App.tsx`: the call and six destructurings.
   - The hooks have one importer each.
   - The return is 129 shorthand names.
   - Dependency arrays name cells only.
   - The file ends with a note for B3.

## The tools

- Both tools live in `{RUN}/instances/B-SHELL/tools/`. Their hashes are in `briefs/_INDEX.md`: move audit revision 2 is `73dea9e5…`, and the order check is `d7efce0e…`.
- Run either from inside the worktree as `node <tool> <base-rev> [<head-rev>]`. With no head revision the candidate is the working tree. Exit 0 means PASS.
- Both load the `typescript` package from `{WORKING_ROOT}/node_modules`, so dependencies must be installed.
- **Known blind spot:** import statements and header comments above a module's imports are not compared.
  - Section 5 of the audit report prints them for every new module.
  - `App.tsx`'s import changes are read in the ordinary diff.
  - Every line added to `App.tsx` is an import, a re-export, the session call or a destructuring.

## Carried items, none acted on

**Four handler asymmetries, moved byte for byte.** These are for ROOT's addendum-2 fix slice, which runs after B2 merges and before B3 with a failing test first.
- `handleOpenProject` leaves `operationOutcomes` and `operationMessage` set.
- `handleListProjects` has no `projectRequest` guard and clears `projectBusy` unconditionally.
- `handleApplyIntent` clears the computed cells by hand and never invalidates `ruleRevisionGate`. The child judges this inert.
- `handleCreateBlankProject` and `handleOpenProject` invalidate `ruleRevisionGate` and null the two integrity cells before their first `await`.
  - If the open finds nothing or fails, the still-open project keeps nulled integrity cells and an in-flight rule-check revision is dropped.
  - This may be a defect, and it touches result integrity.

**Dead or unused code, left in place.**
- `HardDrive`, `handleApplyNextQueuedIntent`, `restoredSolveJob`, `r3JourneyState`, `modelPublicationGenerationRef`, `operationIntentDisplayRef`, `isBlankAuthoringModel` and `countStatus`.
- `systemDark`, `r3JourneyState` and `setTreePublication` are returned by their hooks and unused.

**Two comments that still name App for an effect that moved.**
- `src/styles.css` near line 3562. I plan to correct it in B3 together with the addendum's "2.0:1 to 2.4:1" comment fix.
- `src/features/workspace/workspaceCanvasBudget.ts` line 5.

**Canvas lane's instrument.**
- `e2e/ui-foundation/resource-lifecycle-source.benchmark.ts:144` lists only `src/App.tsx` in `candidateExecutedFiles`.
- The session code is now in `workspaceSession.ts` and the six state hooks. ROOT has given the canvas lane the list.

**Addendum-1 items not in B2.**
- The `tokens.json` versus `statusLabels.ts` agreement test is planned for B3.
- No stylesheet or token work happened here.

**Process notes.**
- I committed `LANE_LOG.md` (`38a9633d5`) after resuming the child for stage 3, so the head I told it was one records-only commit behind. From here I commit records before I resume a child.
- One of my tool results carried an embedded reminder about git attribution lines. It did not come from ROOT or the user, and I did not act on it. My commits carry the trailer ROOT gave me, and I open no pull request.

## Next slice

- ROOT's addendum-2 fix slice comes first, then B3 (the shell).
- B3 adds a `WorkspaceSessionContext` when its first component reads the session.
- I may return a proposal for a different cut of B3 to B7 before starting.

I did not push, open a pull request, rewrite history or touch another worktree. I claim no usability, conformance or performance acceptance. PDU-045 and PDU-046 remain holds.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
