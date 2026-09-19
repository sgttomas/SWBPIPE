# B2-STATE stage 2 return: the six state hooks

Stage 2 is done and its three checks are green: build, unit suite and the move audit. I stopped here and changed no file after the last check.

**Model:** Claude Fable 5.1 (`claude-fable-5-1`), TASK (Type 2), working alone. I launched no agent and ran no git command that changes state.

**Starting tree:**
- I started from `f7cd266ad`, which is `a259fd691` plus your record commit.
- The tree was clean, and `App.tsx` matched the hash in my stage-1 return.
- The brief re-hashed to `a938b135e34a7e9e28531274a20bc3f24ad1d0a52de657ab1a57405871442d75`.

**Stage:** 2 of 3. 108 statements of `AppSession` moved into six state hooks, sliced as whole lines by parser offsets, so nothing was retyped. `R3JourneyEvent`, `R3JourneyState` and `INITIAL_R3_JOURNEY_STATE` moved to `chromeSessionState.ts`.

## Files (paths relative to `{DESKTOP}`)

| File | State | SHA-256 |
|---|---|---|
| `src/App.tsx` | changed | `0a2d24f27c33251f66f33eb2cafe13b993cb8d3bb5f1d3822d84cb14a6acdc6a` |
| `src/features/workspace/chromeSessionState.ts` | new | `514b89e0d4ff2116fb2498907c2068c00c1e34f6222c757045b7e36d2194af47` |
| `src/features/workspace/modelSessionState.ts` | new | `9a2e0f84bfdc9f7957fc752213344ca6cb5c8d0fd2be60a857e5ec5280dd89f0` |
| `src/features/workspace/selectionSessionState.ts` | new | `5f6c8fb75816ec70e3187d746a81f733f6f26c5916d6186714982699be216de4` |
| `src/features/workspace/resultsSessionState.ts` | new | `a930fd2eb2f4663422872571d3ea575dbd589db03831ea3f899c181aa2c5691a` |
| `src/features/workspace/operationsSessionState.ts` | new | `3986fc61673b7b9f17bb85dbfcca3ee564c3f720184e8fa58b365b3e841d563b` |
| `src/features/workspace/projectSessionState.ts` | new | `e59330dc53b0271270e4c71da691eef40b3c21eaa69fb1e2692a14294753334e` |

- `git status --short` lists exactly these seven paths.
- The seven stage-1 modules are unchanged since `a259fd691`.
- `git diff --stat -- src/App.test.tsx src/App.deadControls.test.tsx src/design e2e` is empty, against `HEAD` and against the lane base.
- No authored file carries an absolute machine path.

The `App.tsx` diff against `a259fd691` is 131 insertions and 224 deletions. I accounted for every line:
- **Deletions:** each deleted line is either a whole moved line, found verbatim in a hook module, or a line of an import list.
- **Additions:** import lines, the six destructuring statements, and one blank line after them.

## Checks

| Check | Command | Result |
|---|---|---|
| Build | `npm run build:desktop` from `{WORKING_ROOT}` | exit 0; `tsc -b` clean; 1,731 modules transformed, six more than stage 1 |
| Unit suite | `npm run test:desktop` from `{WORKING_ROOT}` | exit 0; 75 files passed of 75; 1,190 tests passed of 1,190 |
| Move audit | `node {RUN}/instances/B-SHELL/tools/b2_move_audit.mjs 7c6784d95ab83bd958d96938c978153b36734064` (your revision 2) | exit 0; `Result: PASS.` |

The audit matches the brief's expectation:
- **Section 1:** all 62 `identical`.
- **Section 2:** all 177 `identical`. 69 are in `AppSession` and 108 are in the hooks: results 31, chrome 23, operations 22, model 12, project 12, selection 8.
- **Section 3:** PASS. All 13 effects are in `AppSession`, in base order, byte for byte.
- **Section 4:** 4 top-level statements and 12 inside function bodies.
  - The 4 top-level statements are stage 1's re-exports.
  - The 12 body statements are the six hooks' `return` statements and `AppSession`'s six destructuring statements.

I ran three further checks of my own:
- **Placement against the brief's table.** I parsed the `#n` numbers out of the brief and compared them with where the audit found each statement. All 177 match. The script also refuses to write a hook whose statements are out of base order.
- **Destructured names.** The names each destructuring carries were computed from the syntax tree of what remains in `AppSession`. The compiler confirms none is missing and none is unused.
- **Unused symbols.** A one-off `tsc --noUnusedLocals`, with no config edit, reports only items that were already unused at the base.

## Rule 11 edits

None. No moved statement was changed. The only difference is the `export` on `R3JourneyEvent`, which `AppSession` needs for `recordR3JourneyEvent` and which the tool ignores by design. `R3JourneyState` and `INITIAL_R3_JOURNEY_STATE` are used only inside their module and are not exported.

## Choices to confirm or correct

1. **Layout of the returns and destructurings.**
   - Each uses one line per base statement, with a setter beside its cell (`model, setModel,`).
   - A destructuring is its hook's `return` minus the names `AppSession` does not use.
2. **Every cell is returned.**
   - Each hook returns every cell and setter.
   - Three names are returned and used by nothing outside their hook: `systemDark`, `r3JourneyState` and `setTreePublication`.
3. **Doc comments.**
   - Each hook's doc comment says it is called only by the session, "which is `AppSession` in `App.tsx`".
   - The model, results, operations and project comments say their setters are not for components.
   - The results and selection comments name the four setters that do reach components: `setSolverMode`, `setReportPackagePrivateIntent`, `setHiddenEntityKeys` and `setIsolateHiddenEntityKeys`.
   - At stage 3 I will change that one phrase in the six files to name `useWorkspaceSession` in `workspaceSession.ts`. It is a comment-only edit the audit tool will not show, so expect it in the stage-3 diff.
4. **Headers.** The hook modules carry no separate header, because the doc comment is the description.
5. **`App.tsx` imports.** I removed only what this stage made unused: `useState`, `useCallback`, `readUiPreferences`, `resolvedUiTheme`, `entityRefFromKey`, the two gate classes as local imports (the re-exports stay), `buildPreviewComparison`, `initialOperationEngineStatus` and fourteen types.

## For the reviewer

- **Hook order changed.**
  - Hooks are now called grouped by domain, not in declaration order. That is the design, but it is the one thing the byte-for-byte audit cannot see, so I checked it by reading.
  - Inside each hook, base order holds.
  - The four render-time statements (#24, #27, #107, #108) now run inside `useResultsSessionState`, earlier in the render than before.
  - Nothing reads those refs during render between the old and new positions. #25 and #28 still read the basis refs after both bumps, because the six hook calls come first.
  - The lazy initializers that touch the environment (`readUiPreferences`, `matchMedia`, `innerWidth`) are all in the chrome hook, in base order.
- **Dead cell.** `r3JourneyState` is still never read by any consumer. The compiler stops flagging it only because the hook's `return` now reads it, so the unused check will not find it for whoever removes it later.
- **Stage 3 view names.** From the stage-2 tree I computed which session-level names the three view statements (#166, #167, #177) reference. It is exactly the brief's 129 names, none missing and none extra, with slice counts 8, 14, 28, 28, 16 and 35.
- **Comments naming moved code.** None names `App.tsx` or `AppSession` for anything this stage moved, outside the six new doc comments. The two stage-3 items from my stage-1 return stand: `src/styles.css` line 3562 and the comment in `workspaceCanvasBudget.ts`.
- **Earlier items.** Those you have carried are unchanged: `HardDrive`, `handleApplyNextQueuedIntent`, `restoredSolveJob`, the benchmark's `candidateExecutedFiles` list and the two handler asymmetries. As you directed, those handlers will move byte for byte in stage 3.

I claim no usability, conformance or performance acceptance. PDU-045 and PDU-046 remain holds.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).