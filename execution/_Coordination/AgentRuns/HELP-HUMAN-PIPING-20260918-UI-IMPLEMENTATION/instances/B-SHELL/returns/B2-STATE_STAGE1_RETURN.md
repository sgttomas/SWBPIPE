# B2-STATE stage 1 return: pure modules

Stage 1 is done and all three checks are green: build, unit suite and the move audit. I have stopped here and changed no file after the last check.

**Model:** Claude Fable 5.1 (`claude-fable-5-1`), TASK (Type 2), working alone. I launched no agent and ran no git command that changes state. The brief's SHA-256 was confirmed as `a938b135e34a7e9e28531274a20bc3f24ad1d0a52de657ab1a57405871442d75` before anything else.

**Stage:** 1 of 3. I moved 39 top-level declarations out of `App.tsx` into seven React-free modules, by parser character offsets, so no statement was retyped.

## Files (paths relative to `{DESKTOP}`)

| File | State | SHA-256 |
|---|---|---|
| `src/App.tsx` | changed | `44db8deda564c08f73296f604bdaf6037a36d7d3b205eeb0c4148d3f81aa0f5f` |
| `src/features/workspace/solveGates.ts` | new | `1684fb9c8576c6c1a6b6b66b04e1f4a1cf2068d4d77ccb8b00c146c44ff63be4` |
| `src/features/workspace/solveProof.ts` | new | `86f33e407d6e40a0c470acd1288c0c4a04d849b41bb095e0b9feff1455630080` |
| `src/features/workspace/solveJobAudit.ts` | new | `df1e5ae7c5110bedf98a49dfee36bf1f98e07479e7dc96b5c4ac778ec0f6c755` |
| `src/features/workspace/projectPersistenceIntegrity.ts` | new | `42f46efd5bacd6bd4d50e77ed78bc157073cb4cadbca0f92cd4736c6ce3baf9a` |
| `src/features/workspace/workspaceSections.ts` | new | `5df8e486c58280bedd01f5e7ebf2902ebdb215cfbba73c87c397374b754c83f4` |
| `src/features/workspace/menuCommands.ts` | new | `a27964686b60800bb4958e26ebeec94586796a117a2a4937334f9c22ac6e23d0` |
| `src/features/workspace/sessionModel.ts` | new | `1ab4e80459a30261ce928bd83b23c331e9b75a4e6cb160f6728c8ec35e8d1d07` |

`git status --short` lists exactly these eight paths. `git diff --stat -- src/App.test.tsx src/App.deadControls.test.tsx src/design e2e` is empty. No authored file carries an absolute machine path.

The `App.tsx` diff is 56 insertions and 816 deletions:
- **Deletions:** I matched every deleted non-blank line to a line in a new module. The only exceptions are three lines from the trimmed `previewService` imports.
- **Additions:** seven import statements, the one-line `PreviewSolverMode` type import, four re-export statements, and one two-line comment above them.

## Checks

| Check | Command | Result |
|---|---|---|
| Build | `npm run build:desktop` from `{WORKING_ROOT}` | exit 0; `tsc -b` clean; 1,725 modules transformed |
| Unit suite | `npm run test:desktop` from `{WORKING_ROOT}` | exit 0; 75 files passed of 75; 1,190 tests passed of 1,190; same counts as the lane's base |
| Move audit | `node {RUN}/instances/B-SHELL/tools/b2_move_audit.mjs 7c6784d95ab83bd958d96938c978153b36734064` | exit 0; `Result: PASS.` |

The audit matches the brief's expectation exactly:
- **Section 1:** all 62 `identical`, each moved statement in the module the brief names.
- **Section 2:** all 177 `identical` in `AppSession`.
- **Section 3:** PASS, with 13 effects in `AppSession`, in base order, byte for byte.
- **Section 4:** 4 top-level statements and 0 inside function bodies. The four are `App.tsx`'s re-exports. The tool prints them without the leading `export`, because it strips that from every statement.

I ran two further checks of my own:
- **Unused symbols:** a one-off `tsc --noUnusedLocals`, with no config edit, reports the same seven unused items as the base and nothing new.
- **Exports:** every export of the new modules is used by another non-test module.

## Rule 11 edits

None. No moved statement was changed. The only difference is a leading `export` on declarations another module uses, which the audit tool ignores by design. Not exported: `NO_BACKEND_JOB_TOKEN`, `safeJobToken`, `restoredSolveJob`, `SUPPORTED_MODEL_NORMALIZATION_ID` and `NATIVE_MENU_COMMAND_IDS`.

## Choices to confirm or correct

1. **`sessionModel.ts` order.**
   - The table's column says "in base order", but its row lists `clonePreviewModel` second.
   - Base order is `SessionModelCheckpoint` @193, `uiModelIdentityHash` @3301, `selectionForOperationOutcome` @3900, `clonePreviewModel` @3915.
   - I followed base order. These are function declarations, so order has no effect. Say so if you want the row's order instead.
2. **Module headers.**
   - The audit tool treats everything before a file's first statement as that statement's leading comment.
   - A header in a module with no imports (`solveGates.ts`, `workspaceSections.ts`) would therefore make its first declaration CHANGED.
   - Those two have no header, and neither does `menuCommands.ts`. Each already opens with the base comment that introduces it.
   - The other four have a three- or four-line header above their imports.
   - This is a limit of the tool worth knowing: a comment placed above a module's imports is outside everything it compares.
3. **Import form.** In the new modules every type comes in through an `import type` statement. In `App.tsx` the added imports use the file's existing inline `type` modifiers.

## For someone else to handle

- **Unused import.** `HardDrive` (lucide-react) is imported by `App.tsx` and never used, at the base and now. I left it, because the move did not make it unused.
- **Uncalled handler.** `handleApplyNextQueuedIntent` (#142) is declared in `AppSession` and never called. It is not in the brief's dead-code list. It stays in place, and by rule 5 moves with the handlers in stage 3.
- **`restoredSolveJob`** is now in `solveJobAudit.ts`, unexported and uncalled, as the brief foresaw.
- **Benchmark source binding.** `e2e/ui-foundation/resource-lifecycle-source.benchmark.ts` lists `src/App.tsx` among its `candidateExecutedFiles`. From this stage on, code the app executes also lives in the new workspace modules. Whoever next runs that benchmark decides whether the list should grow. It is outside my write scope.
- **Comments naming `App.tsx`.** None refers to anything stage 1 moved. Two refer to things stage 3 will move: `src/styles.css` line 3562 (the `data-theme` layout effect) and the comment in `workspaceCanvasBudget.ts` that says App owns the subscription. I will raise them again at stage 3.
- **Possible defects in the handlers.** I changed none of them.
  - `handleOpenProject` leaves `operationOutcomes` and `operationMessage` set. The previous project's operation diagnostics therefore stay in the issue count and in the Issues drawer after another project is opened.
  - `handleListProjects` has no `projectRequest` guard and clears `projectBusy` unconditionally. A list that finishes during a pending save re-enables the project buttons early. The native menu can start one, because `runMenuCommand` does not check `projectBusy`.

I claim no usability, conformance or performance acceptance. PDU-045 and PDU-046 remain holds.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).