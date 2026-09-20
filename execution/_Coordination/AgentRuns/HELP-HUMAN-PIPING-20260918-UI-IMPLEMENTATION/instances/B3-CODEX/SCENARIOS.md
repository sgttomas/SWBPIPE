# B3 connected scenario set

This is a reusable navigation index to maintained tests, not product acceptance.
The candidate results and environment are in the manager return and raw run logs.
All test paths below are relative to `projects/chirality-piping/apps/desktop`.

| Journey | Maintained scenario | State checked beyond pixels |
|---|---|---|
| New blank → node → pipe → Add → review → Apply | `e2e/linear-authoring.spec.ts`, compact blank-to-straight authoring | Explicit reviewed draft, exact Add/Apply route and model entities |
| Open/create → sections/stages/views → select → inspector → routing | `e2e/workspace-layout.spec.ts`, workspace preserves visible model; routing uses inspector | Pane/drawing geometry, retained view and inspector state, focus and mounted drafts |
| Invalid authoring input → recovery → apply → undo/redo | `e2e/r2-smoke.spec.ts`, R2 from-blank A12 and solve/result/overlay journeys | Operation outcome, selection, model cells and history |
| Solve → Current → results/report → edit → result withdrawal | `e2e/gui-workflow-validation.spec.ts`, invented fixture transitions; `src/App.shell.test.tsx` chip/stage policy | Current standing and no stale reuse, result/engine diagnostics |
| Save/reopen → Historical | `src/App.shell.test.tsx` reopened Historical; existing persistence/compatibility source and dist cases | Historical never lights Current chips or enables Review |
| Keyboard menu and pointer equivalence | `src/App.shell.test.tsx`, native shell command equivalence; Tauri native witness | Same command sink and menu state; native owns Cmd-1/2/3/I, webview duplicate disabled |
| User split → inspector open/close; narrow drawer 180/600; active measurement | `e2e/workspace-layout.spec.ts`; `e2e/ui-foundation-dist.spec.ts` content-aware narrow canvas budget | Stored preferences restore, drawn floor and rendered/readout clearance, pointer hit ownership |
| Routing cancel/disarm | `src/App.shell.test.tsx`, routing borrows inspector; viewport portal tests | Prior inspector collapse state restored, focus leaves closing subtree, one live authoring panel |
| Popup cancel/commit → preserved source → queue | `CompactSelect.test.tsx`, `richForms.test.tsx`, `e2e/b3-accessibility.spec.ts`, native final value backcheck | No inferred family on passive dismissal; explicit value/basis identity; real first/second Escape; queue JSON omits missing family |
| Loads/Table or Historical Results → New Blank | `App.shell.test.tsx`, independent fidelity and actual native witness | Model/Both reset at success only; failed/superseded create preserves prior shell and preferences |
| Text REDO pending → neutral model undo/redo → text redo | actual9d native witness + ROOT backcheck | One model checkpoint each, text REDO remains; custom native Edit model pointer commands also verified |

Actual native actions and unavailable states are separately recorded by the native executor.
Browser-native-class layout cases remain browser evidence. B3A unsaved-title and B3B
busy-project gating/save-integrity work are deferred. Full B5 user-split slide-over and
agent-column transitions remain deferred; this slice retains the supported browser
fallback below the native minimum. Practitioner usability holds remain.
