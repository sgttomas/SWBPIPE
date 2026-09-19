# Return: B2F-HANDLERS

Items 1 to 3 are repaired with failing tests first, item 4 is left as it is, and all four checks pass on the final candidate. For item 2 I departed from B-SHELL's recommendation (a), because an existing safety-net test forbids it; the reasons are under "Repairs".

**Model.** By my own statement I am Claude Fable 5.1 (`claude-fable-5-1`), working as TASK (Type 2). I delegated nothing and ran no git command that changes state. The head is unchanged at `5e5af35dc29a1584845878c398c3f96a681e5ba1`.

**Tooling note.** My file tools refused paths outside their own session worktree. I therefore wrote into the assigned worktree with shell commands, using exact-match replacements that each asserted a single occurrence.

## Files

| File | State | SHA-256 |
|---|---|---|
| `{DESKTOP}/src/features/workspace/workspaceSession.ts` | changed, +20 −7 | `984ee2d5e198d15b80fea445f878ad09e597fa23c6c05efe054f95337a8a586a` |
| `{DESKTOP}/src/App.projectHandlers.test.tsx` | new, 445 lines, 7 tests | `22f28e6f315be3ba036cf1c0fe49830be99ad31410d0300343944dba7b5ed7d5` |

The fallback of appending to `src/App.test.tsx` was not needed. Copied from `src/App.test.tsx`, none of it imported:
- the hoisted `invokeMock` and the `vi.mock` of `@tauri-apps/api/core`;
- the `afterEach`, `deferred` and `nativeMenuCommand`;
- an `inventedOpenEnvelope` equivalent;
- `openWorkspaceSection`, reduced to three sections and keeping its native-menu fallback;
- `runMechanicsButton`;
- the pattern for decoding the validation export link.

## Tests

The failures were recorded by running the final test file against the HEAD source. I wrote `git show HEAD:…` into the file, ran the tests, then restored my copy and checked the restore by hash. All 7 fail on that tree and all 7 pass after the repair.

**Item 1.** "drops the earlier project's operation diagnostics from the issue count and the drawer when a project is opened".
- Unrepaired: `expect(element).not.toBeInTheDocument()` fails with "expected document not to contain element, found <section aria-label="Operation diagnostics" …> INVENTED_OPEN_CARRYOVER".
- The issue count also stayed at baseline + 1 after the open.
- Repaired: passes.

**Item 2, save then list.** "keeps the project controls disabled through a native list command while a save is pending, then lands the save".
- Unrepaired: `expect(element).toBeDisabled()` fails with "Received element is not disabled", at the check after the list settles.
- Repaired: passes. The controls stay disabled and `list_local_projects` is never invoked.
- The save's message then lands, and the validation packet's `summary.last_operation` is `save`.

**Item 2, list then native save.** "leaves a save's busy state alone when a listing that started first settles during the save".
- Unrepaired: the same `toBeDisabled()` failure, after the list settles.
- It also failed with guard (b) alone, which is why the `finally` check exists.
- Repaired: passes.

**Item 3.** Three tests, each "keeps a recorded model-hash mismatch when …": "a later open finds nothing", "a later open fails", and "a blank create fails".
- Unrepaired, each expects `integrity=mismatch_review_required` and receives `…; integrity=open_verification_not_run_this_session`.
- Repaired: all three pass. The `tree-row-project-…` row for the project id is unchanged.
- The messages are the `open_missing`, `Open failed: …` and `Blank create failed: …` ones.

**Item 3, in-flight revision.** "lets an in-flight rule-check revision land when an open finds nothing".
- The test holds `buildAnalysisRunPreview` open with a spy while an open returns `null`.
- Unrepaired: `run-audit-status` was expected to contain `USER_RULE_FAILED` and showed `Run statusHUMAN_REVIEW_REQUIRED, MECHANICS_SOLVED, RULE_INPUTS_INCOMPLETE`.
- Repaired: passes.

**Item 4.** I wrote no test and made no change.

## Repairs

**Item 3, lines 1439–1467 and 1505–1528.**
- The three statements moved from before the first `await` to directly before `advanceProjectSession();`, in the same relative order.
- In the blank handler they now sit after `stillCurrent`. In the open handler they sit after `stillCurrent` and after the `!opened` return.
- From the commit point on, the open reads as it did. `stillCurrent` is untouched.

**Item 1, lines 1539–1540.** `setOperationOutcomes({}); setOperationMessage(null);` follow `setBatchMessage(null);`, which is the blank handler's position for them.

**Item 2, lines 1686–1707.** `handleListProjects` alone changed; `runMenuCommand` is byte for byte as at HEAD.
- **Form chosen:** guard (b), so the list returns at once while `projectBusy` is true. To it I added a read-only ownership check. The list records `projectRequest.current` when it starts and releases in `finally` only if the number is unchanged.
- The list reads the request number and never advances it, so a pending save's `stillCurrent` is unaffected.
- **Why (a) was dropped.** I implemented (a) first. With it, the existing test "Tier3 adversarial batch publication > discards a pending batch when another model opens, and ignores delayed save metadata after a model replacement" failed at `src/App.test.tsx:16620`. That test pins a native `file.open-local` superseding a pending save.
- Guard (a) makes that supersession unreachable, and no existing test may change, so (a) had to go. Of the five project handlers, only the list takes no request number, so the repair belongs to the list alone.
- **Why not (b) alone.** The reverse order still released a save's busy state.
- **Why not a count across all five handlers.** It is wider than the defect needs, which is the brief's own reason.

**Item 4 is left exactly as it is.**
- The missing invalidation is inert. `commitModel` advances `uiModelRevisionRef` synchronously and the model revision through the layout effect. `currentSolvedResultRef` also changes.
- An in-flight rule-check revision therefore fails three other conditions of its guard.
- The shared reset would add a second `invalidateReportPackageComputedState()` call. I judged that unobservable, but showing it needs a test longer than a short reading.

## Checks (final candidate; no file changed after)

1. From `{WORKING_ROOT}`, `npm run build:desktop`: exit 0.
2. From `{WORKING_ROOT}`, `npm run test:desktop`: 76 files and 1,197 tests passed (1,190 plus these 7), exit 0.
3. From `{DESKTOP}`, `PLAYWRIGHT_WORKERS=1 sh {RUN}/tools/with_e2e_lock.sh npm run test:e2e`: 374 passed and 20 skipped, exit 0. It ran once.
4. From `{DESKTOP}`, `PLAYWRIGHT_WORKERS=1 sh {RUN}/tools/with_e2e_lock.sh npm run test:e2e:dist`: 53 passed, exit 0. It ran once.

- `git status --short` lists only the two files above.
- The protected `git diff --stat` is empty.
- Nothing listens on ports 5174 or 5175.
- My lock was released. The lock present afterwards belongs to the other lane.

## Semantic changes, named

1. **Open and operation diagnostics.**
   - Today an open keeps the previous project's operation diagnostics in the issue count and the Issues drawer, and keeps the operation message.
   - Now it clears them, as "New blank" does.
   - Held by the item 1 test.
2. **Failed or empty open.**
   - Today an open that finds nothing or fails, or a blank create that fails, nulls the open project's integrity cells. A recorded `mismatch_review_required` then reads `open_verification_not_run_this_session`.
   - Now the cells are left as they were.
   - Held by the three item 3 tests.
3. **The moved `ruleRevisionGate` invalidation.**
   - Today such an open or create drops an in-flight rule-check revision. `ruleCheckAggregate` stays set while `analysisRun` is not revised.
   - Now the revision lands.
   - Held by the in-flight test.
4. **Consequence of the same move.**
   - Today the previous project's integrity status disappears while an open is pending.
   - Now it stays until the new project commits. A superseded open no longer nulls the cells either.
   - No dedicated test covers the pending window.
5. **List during a busy request.**
   - Today a list can start while another project request is busy. On settling it re-enables the controls and publishes its message over the pending request.
   - Now it does not start.
   - Held by the first item 2 test.
6. **List settling after a later request starts.**
   - Today a list that settles after a request-numbered handler has started releases that handler's busy state.
   - Now it releases only when no such request has started since the list began.
   - Held by the second item 2 test.

## For someone else

- `handleCreateProject` and `handleSaveProject` also null the integrity cells before their first `await`. They are out of this slice and I left them untouched.
- Guard (b) reads the render's `projectBusy`. A list dispatched in the same tick as another handler's start sees the stale value, as the buttons do. Closing that would need a ref outside the handler bodies.
- A list that settles during a later request still publishes its message and index until that request lands. The addendum asked only about the busy state.
- The native menu is not disabled while a project request is busy; only the in-app menu is. The safety net pins that reachability. If the owner wants the native menu to match, that test must change in a slice that names it.

After the last check I changed no file. I claim no usability, conformance or performance acceptance; PDU-045 and PDU-046 remain holds.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).