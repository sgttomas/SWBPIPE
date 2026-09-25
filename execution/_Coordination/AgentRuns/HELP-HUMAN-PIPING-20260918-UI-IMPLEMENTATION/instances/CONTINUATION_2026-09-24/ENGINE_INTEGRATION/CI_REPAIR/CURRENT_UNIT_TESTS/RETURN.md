# CURRENT_UNIT_TESTS — RETURN (Type 2 TASK, dispatched by HELP_HUMAN ROOT)

All paths are repository-relative. `_run_records/` means `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/ENGINE_INTEGRATION/CI_REPAIR/CURRENT_UNIT_TESTS/_run_records/`; machine paths and raw outputs are kept there.

## Result

Complete. Review finding S3 is closed at unit level. A new vitest file drives a qualified Current result through the session's own
solve path and qualification gate. It then renders `SolvePanel` and `DesignWorkspacePanel` directly with the props `App.tsx`
passes them. It asserts that the recorded units, including `N*m/rad` and `N/m`, appear unconverted in `solve-job-unit-policy`
and `design-workspace-units`. A negative control (no Current result) asserts `results=none` in both lines. No production
code, e2e spec, fixture or style was changed. No commit was made.

## Basis and git

- Governing files read: `AGENTS.md` (sha256 `c8ce87ef…1dffd`), `projects/chirality-piping/AGENTS.md` (`d9481951…bc792`) and
  `agents/AGENT_TASK.md` (`1a13a5b0…c8fb7`, matching the brief).
- Only git write: `git merge --ff-only 347e214fe`, a fast-forward from 8e216efbf to 347e214fe. Before any edit,
  `git status --porcelain` showed only the untracked `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/ENGINE_INTEGRATION/CI_REPAIR/INDEPENDENT_REVIEW/`.
- Candidate: HEAD 347e214fe plus the single untracked test file below. `_run_records/full-suite-candidate-state.txt` records the
  worktree state, including ROOT's concurrent edits, none of which are under `projects/chirality-piping/apps/desktop/src`.

## File delivered

| Path | sha256 |
|---|---|
| `projects/chirality-piping/apps/desktop/src/features/workspace/currentResultUnitPolicy.test.tsx` (new) | `f6721af8bf98710af970e6ed990b723a7497e32f546388780060a2799bbe6a2e` |

Sources the tests rely on, all unchanged:

| Path | sha256 |
|---|---|
| `projects/chirality-piping/fixtures/product_preview/invented_mechanics_result_precision_1_sparse.json` | `264361d5…6e820e6` (matches the sparse output recorded in `projects/chirality-piping/fixtures/product_preview/precision_fixture_generation.json`) |
| `projects/chirality-piping/fixtures/product_preview/invented_preview_model.json` | `986c0559…4f871c` (matches that record's `input_model`) |
| `projects/chirality-piping/apps/desktop/src/features/solve/SolvePanel.tsx` | `e5bd20ba…68f0f6a1` |
| `projects/chirality-piping/apps/desktop/src/features/design-workspace/DesignWorkspacePanel.tsx` | `17ede84d…d053aaae` |
| `projects/chirality-piping/apps/desktop/src/features/workspace/resultsSessionState.ts` | `455a5b29…a6813f6f2380` |

## How the Current result is produced (no bypass)

- `App.tsx` passes `SolvePanel` the session `result` and `DesignWorkspacePanel` the qualified `currentSolvedResult`. The tests
  pass the same session cells: `result`, `currentSolvedResult`, `analysisRun`, `comparison`, `solveJob`, `solverMode`, `knowledge`,
  `editorIntents`, `proposal` and `selectedReviewTarget`.
- The Current result comes from the real `useWorkspaceSession().results.handleRun()`, with mocked Tauri IPC. That path runs
  backend job start/poll, native invocation registration, `hasNativeMechanicsInvocation`, `buildCurrentSessionInputManifest`,
  `buildAnalysisRunPreview` (v0.3.0) and the unchanged `currentSolvedResult` gate in `resultsSessionState.ts`.
- The IPC mock replays the recorded producer bytes for the exact recorded input model and mode only; any other request is
  rejected. This is a unit transport replay, not a native UI qualification witness, and the file header says so.
- The positive test also asserts:
  - `result === currentSolvedResult`;
  - native invocation binding is true;
  - the result has 830 rows;
  - the unit set equals the fixture's own set;
  - the received bytes are unchanged (`toEqual(sourceFixture)`).
- Placement: the file lives in `projects/chirality-piping/apps/desktop/src/features/workspace/` rather than beside each panel. It uses the session hook and renders both
  panels from one Current result, which matches the combined coverage that `App.test.tsx` had on main.

Exact assertions:
- `solve-job-unit-policy` (Current): `model=angle=rad,force=N,length=m,pressure=Pa,stress=MPa,temperature=degC; results=MPa,N,N*m,N*m/rad,N/m,boolean,count,m,mm,mode_code,rad,state_code; rows=830; conversion=false`
- `design-workspace-units` (Current): `…; results=<same 12 units>; comparison=MPa,N,N*m,mm,rad; conversion=false`
- Negative control (session before any run, `result` and `currentSolvedResult` both null): the solve line ends `results=none; rows=0; conversion=false`,
  and the design line ends `results=none; comparison=none; conversion=false`.

## Commands and results

All commands were run from `projects/chirality-piping` with `PATH=/opt/node24/bin:$PATH`: node v24.21.0, vitest 4.1.10, tsc 5.9.3, 4 CPUs
(`_run_records/environment.txt`).

| Command | Result | Record |
|---|---|---|
| `npm test --workspace apps/desktop -- src/features/workspace/currentResultUnitPolicy.test.tsx` | 2/2 passed, exit 0 | `_run_records/new-test-pass.log` |
| Mutation of `SolvePanel.tsx` (below) | positive test fails, exit 1; restored | `_run_records/mutation-solvepanel.log` |
| Mutation of `DesignWorkspacePanel.tsx` (below) | positive test fails, exit 1; restored | `_run_records/mutation-designworkspace.log` |
| Mutation of the qualification gate in `resultsSessionState.ts` (below) | positive test fails, exit 1; restored | `_run_records/mutation-qualification-gate.log` |
| `npx tsc -b` (in `projects/chirality-piping/apps/desktop`) | exit 0 | `_run_records/tsc-b.log` |
| `VITEST_MAX_WORKERS=2 npm run test:desktop` (final candidate) | 118 files: 114 passed, 4 failed; 2054 tests: 2046 passed, 8 failed; exit 1. The new file passed. All 8 failures predate this work (see Baseline). | `_run_records/full-desktop-vitest.log` |
| Baseline: the 4 failing files run with the new file removed from the worktree, then restored (sha verified) | the same 8 failures, exit 1 | `_run_records/baseline-without-new-test.log` |

## Mutation evidence

Every mutation was a one-line temporary edit. The original was restored by byte copy, and each record shows the sha256 before,
after and against the HEAD blob, plus an empty `git diff`.

1. `SolvePanel.tsx` `result_units` gained `.filter((unit) => unit !== "N*m/rad")` (sha `e5bd20ba…` → `e20f44c2…` → `e5bd20ba…`).
   The test received `results=MPa,N,N*m,N/m,…` where it expected `results=MPa,N,N*m,N*m/rad,N/m,…`.
2. `DesignWorkspacePanel.tsx` `result_units` gained `.filter((unit) => unit !== "N/m")` (`17ede84d…` → `db94d5de…` → `17ede84d…`).
   The design-line assertion failed the same way.
3. `resultsSessionState.ts` changed `&& numericallyEligible` to `&& numericallyEligible && false` (`455a5b29…` → `e49c3a8f…` → `455a5b29…`).
   The test failed with `expected null not to be null`. This shows the Current result must pass through the real gate.

## Superseded first draft (disclosed)

The first draft imported `useResultsSessionState` directly. The full suite then flagged it under
`sessionBoundary.test.ts > lets no file but workspaceSession.ts import a *SessionState module`. I rewrote the test to use
`useWorkspaceSession`, which is permitted and is the App's actual solve path, and repeated every check on the final bytes.
The draft and its logs are in `_run_records/superseded_first_draft/` and are not current evidence. The first full-suite
run also had two `App.test.tsx` timeouts at 30s under load. They did not recur in the final run.

## Unresolved / for the caller

- The following 8 failures occur on 347e214fe without this file, and I did not touch them:
  - `projects/chirality-piping/apps/desktop/src/features/handoff/HandoffPanel.test.tsx` and `projects/chirality-piping/apps/desktop/src/features/headless-runner/HeadlessRunnerPanel.test.tsx` both fail with
    `SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED`. Both build an analysis run from the legacy `invented_mechanics_result.json`.
  - `projects/chirality-piping/apps/desktop/src/features/rule-check/RuleCheckRunPanel.test.tsx` has 5 failures, with `RULE_NATIVE_INVOCATION_REQUIRED` and a missing
    `rule-check-run-result`.
  - `projects/chirality-piping/apps/desktop/src/features/workspace/sessionBoundary.test.ts` fails because of the existing guard offenders
    `projects/chirality-piping/apps/desktop/src/features/results/physicsSourceIntegration.test.tsx` and `projects/chirality-piping/apps/desktop/src/features/results/sourceBlockRecovery.test.ts`.

  The worktree shows ROOT editing `projects/chirality-piping/tests/test_headless_runner_contract.py` and a `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/ENGINE_INTEGRATION/CI_REPAIR/HEADLESS_RUNNER_TESTS/` directory,
  which may be related.
- Observation only, not claimed as a defect: `App.tsx` feeds `SolvePanel` the session `result`, not `currentSolvedResult`. The two
  are identical in the qualified case, and the test asserts this. A retained result that is not qualified would still have its
  units listed by the solve line. Reference inspection clears `result`, so it is unaffected.
- The design-line assertion includes `comparison=MPa,N,N*m,mm,rad`, which comes from `buildPreviewComparison`'s default bases
  (load:L-100 vs combination:C-OPER-ALT). A deliberate change to comparison logic would need this expectation updated.
- No commit, push, browser run or native witness was made. The test file and this evidence directory are untracked, and
  integration and independent review are the caller's.
