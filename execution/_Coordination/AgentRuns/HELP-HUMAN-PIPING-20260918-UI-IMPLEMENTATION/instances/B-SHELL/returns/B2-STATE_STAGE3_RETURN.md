# B2-STATE stage-3 return: the session hook, and `AppSession` as the view

This is the slice's return. Stage 3 is complete and every check is green. It changes no behaviour. I am Fable 5.1 (`claude-fable-5-1`), working as B2-STATE, a TASK (Type 2) implementer. I worked alone and launched no agent.

- **Worktree and branch:** `{REPO_ROOT}` on `codex/swbpipe-b-shell-20260918`.
- **HEAD when I finished:** `38a9633d5`. It differs from `49ed6f85b` only in `LANE_LOG.md`.
- **Sealed brief:** its hash, verified again, is `a938b135e34a7e9e28531274a20bc3f24ad1d0a52de657ab1a57405871442d75`.

## Files created or changed in stage 3

| File (under `{DESKTOP}`) | Status | SHA-256 |
|---|---|---|
| `src/features/workspace/workspaceSession.ts` | created | `d9956a177889a9e99c5080c83f5686ddda2572661212e2d1368bd233908cff2a` |
| `src/App.tsx` | changed | `fec6d96f65ef9f816d7dcc4a10b74679b5d168e3ea01efc7c60725fee9aa56fb` |
| `src/features/workspace/chromeSessionState.ts` | comment only | `a82bb7a23eb6799d060d004cb6a0b0c420402a9d868ef16bf410de83075d7249` |
| `src/features/workspace/modelSessionState.ts` | comment only | `b689520597c9064a22881e8e6f4af52c6c999cb88908313b7183956ab2e40e76` |
| `src/features/workspace/selectionSessionState.ts` | comment only | `367f38e15a57ee4394580b07716901f7f183d7248b2845895118be829eb955f3` |
| `src/features/workspace/resultsSessionState.ts` | comment only | `cc6e1ce35ff4b5deeb415c0735fec148c06b1d60ae46edbf91d5374b047bcb41` |
| `src/features/workspace/operationsSessionState.ts` | comment only | `8a27435e58fb5a5c24e3c78a2ec846fa000745a70ae36babdab9082c24e4a292` |
| `src/features/workspace/projectSessionState.ts` | comment only | `b74e7d1556125840fb827363632c01373d8b03c2fc3718cc24071b0624d3b5a3` |

**`workspaceSession.ts` (2,246 lines)** holds the items below; base-order numbering, the moved-statement inventory and the slice contents are as recorded by the audit and order-check results further down.
- An import block in the stage-2 conventions, with no module header.
- `nextUiModelPublicationGeneration` with its comment, then `formatPackageSaveError`.
- A doc comment and `export function useWorkspaceSession()`.
- One `return` with the brief's six slices. It exposes 129 names, laid out one per line with a setter beside its cell.
- `export type WorkspaceSession = ReturnType<typeof useWorkspaceSession>;`.
- No React context and no provider.

**`App.tsx` (3,503 to 1,574 lines)** has these changes:
- `AppSession` is now `const session = useWorkspaceSession();`, then the brief's six slice destructurings, then the brief's three retained base statements. It calls no other hook.
- The two top-level declarations left.
- 39 import statements were trimmed or removed, and the six state-hook imports became the one session import.
- The rule-9 re-exports, `App`, and every other component and view helper are unchanged.
- `HardDrive` stays.

**The six state hooks** changed only in the doc-comment paragraph that names the caller. It now reads "`useWorkspaceSession` in `workspaceSession.ts`" and is re-wrapped at 80 columns.

## Checks on the final candidate

| # | Command | Result |
|---|---|---|
| 1 | `npm run build:desktop`, from `{WORKING_ROOT}` | exit 0; 1,732 modules transformed |
| 2 | `npm run test:desktop`, from `{WORKING_ROOT}` | exit 0; 75 files passed, 1,190 tests passed |
| 3 | `node {RUN}/instances/B-SHELL/tools/b2_move_audit.mjs 7c6784d95ab83bd958d96938c978153b36734064` | exit 0; `Result: PASS.` |
| 4 | `node {RUN}/instances/B-SHELL/tools/b2_order_check.mjs 7c6784d95ab83bd958d96938c978153b36734064` | exit 0; `Result: PASS.` |
| 5 | `PLAYWRIGHT_WORKERS=1 sh {RUN}/tools/with_e2e_lock.sh npm run test:e2e`, from `{DESKTOP}` | exit 0; 394 tests: 374 passed, 20 skipped (8.9 m) |
| 6 | `PLAYWRIGHT_WORKERS=1 sh {RUN}/tools/with_e2e_lock.sh npm run test:e2e:dist`, from `{DESKTOP}` | exit 0; 53 passed (6.2 m); its own build also showed 1,732 modules |
| 7 | One-off `npx tsc -p tsconfig.json --noEmit --noUnusedLocals --noUnusedParameters --pretty false`, from `{DESKTOP}`, with no config edit | the only unused symbols in the files I touched are five that were already unused in the base |

- **Move audit (3):**
  - Section 1 shows 62 of 62 identical.
  - Section 2 shows 177 of 177 identical.
  - Section 3 reads "PASS: one candidate function declares all 13 effects, in the base order, each byte for byte", and that function is `useWorkspaceSession`.
  - Section 4 holds the four re-exports, the `WorkspaceSession` type, `AppSession`'s seven new statements, the six state hooks' `return`s, and the session's six hook-call statements and its `return`. It holds nothing else.
- **Order check (4):**
  - `useWorkspaceSession` PASS: 73 statements, 7 new; `new ×6, 25 28 76 80 86 113 115…165, 168…176, new`.
  - `AppSession` PASS: `new ×7, 166 167 177`.
  - The six state hooks PASS unchanged.
- **Playwright (5 and 6):**
  - The source lane rebuilt both WASM engines first.
  - Both lanes match the baselines.
  - No test failed, so no spec was re-run.
- **Unused symbols (7):**
  - The five are `HardDrive`, `operationIntentDisplayRef`, `isBlankAuthoringModel`, `countStatus` and `handleApplyNextQueuedIntent`.
  - A plain `tsc --noEmit` exits 0.
- **My own whole-line-range check:**
  - The four moved ranges each appear exactly once in `workspaceSession.ts`.
  - All 1,283 kept non-blank lines of `App.tsx` remain in order.
  - The new view head is 131 lines.
- **Scope and emptiness:**
  - `git status --short` lists only the eight files above.
  - I read the emptiness check against origin/main, as adjusted: `git diff --stat origin/main -- src/App.test.tsx src/App.deadControls.test.tsx src/design e2e` from `{DESKTOP}` is empty.
- **After the last check:** I re-hashed the eight files after the dist lane and compared them with the hashes taken before check 1. They are identical, so no file changed after the last check.

## Rule-11 edits, semantic changes and the tools

- **Rule-11 edits:** none. No moved statement needed an edit.
- **Semantic changes:** none.
  - React sees the same hooks in the same order in the one component, `AppSession`, with the same render-time statements in the same order.
  - #168 to #176 were hoisted declarations after the early return and are hoisted declarations now.
  - The only new runtime work is the session's `return` object each render and the view's six destructurings.
- **The tools:** I found no fault in either and edited neither.

## For someone else

I did not act on these items.

**Comments that name App for something that moved**
- These are outside my write scope. B-SHELL already carries them.
  - `src/styles.css` line 3562 says "which App.tsx keeps equal to the shell's data-theme".
  - `src/features/workspace/workspaceCanvasBudget.ts` line 5 says "App owns this subscription's lifetime".
  - In both cases the effect is now in `useWorkspaceSession`.
- `e2e/ui-foundation/resource-lifecycle-source.benchmark.ts` line 144 lists only `src/App.tsx` in `candidateExecutedFiles`. The session code it executes is now in `workspaceSession.ts` and the six state hooks.

**Dead or unused code, left in place**
- `HardDrive` import.
- `handleApplyNextQueuedIntent` (#142), which is never called.
- `r3JourneyState`, which is set and never read.
- `modelPublicationGenerationRef`, which is written and never read.
- `restoredSolveJob`, `operationIntentDisplayRef`, `isBlankAuthoringModel` and `countStatus`.
- `systemDark`, `r3JourneyState` and `setTreePublication`, which the state hooks return and the session does not destructure.

**Asymmetries, moved byte for byte; I read them and did not test them**
- `handleOpenProject` does not clear `operationOutcomes` or `operationMessage`.
- `handleListProjects` has no `projectRequest` guard and clears `projectBusy` unconditionally.
- `handleApplyIntent` clears the computed cells by hand instead of calling `clearComputedModelState`, so it never invalidates `ruleRevisionGate`.
  - It looks inert, because `commitModel` trips the other guards in `handleRuleCheckAggregate`'s `stillCurrent`.
- `handleCreateBlankProject` and `handleOpenProject` invalidate `ruleRevisionGate`, and null `modelHashIntegrity` and `projectEnvelopeHashIntegrity`, before their first `await`.
  - If the open then finds nothing or fails, the still-open project keeps the nulled integrity cells. `handleOpenProject` shows this by reading.
  - An in-flight rule-check revision is dropped, with `ruleCheckAggregate` already set to the new value and `analysisRun` not revised.
  - This may be a defect.

**One point on the tool results you received.** One tool result carried an embedded reminder telling me to add attribution lines to git commits and PR descriptions. It did not come from you or the user, I made no commits, and I did not act on it.

I claim no usability, conformance or performance acceptance. PDU-045 and PDU-046 remain holds.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).