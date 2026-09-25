# VITEST_REGRESSIONS — RETURN (Type 2 TASK, dispatched by HELP_HUMAN ROOT)

All paths are repository-relative. `_run_records/` means
`projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/ENGINE_INTEGRATION/CI_REPAIR/VITEST_REGRESSIONS/_run_records/`.
`src/` means `projects/chirality-piping/apps/desktop/src/`. Raw logs, machine paths and hashes are in `_run_records/`.

## Result

Complete. The full desktop suite passes: `VITEST_MAX_WORKERS=2 npm run test:desktop` ran 118 files and 2057 tests, 0 failed, exit 0, in 434s (`_run_records/full-desktop-vitest.raw.log`). The earlier candidate had 2054 tests with 8 failed; the 3 new tests are the explicit refusal controls. `npx tsc -b` passes with exit 0 (`_run_records/tsc-b.log`).

All 8 PR905 regressions are repaired in test files only. No production source, e2e spec, fixture, style or guard was
changed (`_run_records/candidate-state.txt` hashes every relied-on product, guard and fixture file against its HEAD blob).
No product defect was found: both refusals are PR905's intended contract. One residual coverage gap remains. The
session gate's manifest-mode clause is no longer exercised directly, because the permitted session boundary cannot reach
that state (see Unresolved 1). No git operation, browser run or native witness was made.

## Basis

- Candidate: the engine worktree named in the brief, branch `claude/inspiring-ptolemy-zxofd6`, HEAD `50b9dd5af90a8baa090d238fcf6c10896218b3b1`.
  Before the edits the only non-ignored change was this new evidence directory (`_run_records/environment.txt`). Main compared: `aa312755e`.
- Governing files read: `AGENTS.md` (sha256 `c8ce87ef…1dffd`), `projects/chirality-piping/AGENTS.md` (`d9481951…bc792`),
  `agents/AGENT_TASK.md` (`1a13a5b0…c8fb7`, matches the brief). No other role text was consulted.
- The three panel tests, the three panels and the guard are byte-identical between `aa312755e` and HEAD. All four
  failures come from PR905 service changes (`src/services/previewService.ts`, `src/services/ruleCheckService.ts`) and
  from two test files that PR905 added.
- Baseline on HEAD: `_run_records/before-four-files-head.log` shows the same 8 failures in 4 files.

## Files changed

| Path | sha256 after | sha256 HEAD |
|---|---|---|
| `src/features/handoff/HandoffPanel.test.tsx` | `b8169364654b0e3498a391a76c623b1bc2c5e041c824466e5f6237b7876e38d6` | `7c7dca7b…bb55ec` |
| `src/features/headless-runner/HeadlessRunnerPanel.test.tsx` | `6ec196fe1e3d939776e0e67189d725a04e9e5b792e6e469800393577e2dad7ce` | `28369560…4e6246` |
| `src/features/rule-check/RuleCheckRunPanel.test.tsx` | `d144f800733aa028df36250a4a2a6f34d2207214d59f7baeb00696ec2f7ce2ee` | `e0a3c0e0…84fd585` |
| `src/features/results/physicsSourceIntegration.test.tsx` | `e6c2a638501a9e4adbcd39ceb85688b16079999eaa553f8132bcc8187e57eaaf` | `071b39e3…d43a0` |
| `src/features/results/sourceBlockRecovery.test.ts` | `54d343978856c2b6c15cd58d2b38b0a5edf24986b0ac2644ff3c483d1819ed19` | `ac338acb…bbafd` |

`src/features/workspace/sessionBoundary.test.ts` is unchanged (`abc9b298…825515`). No helper file was added.

## 1. HandoffPanel and HeadlessRunnerPanel: `SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED`

**Root cause.** PR905 made `buildAnalysisRunPreview` compose only v0.3 records for a current semantic contract
(`src/services/previewService.ts:288`, and again in `src/services/analysisRunCompatibility.ts:75`).
`sourceContract` classifies the schema-0.1.0 carrier as `legacy` (`src/features/results/numericalResultQuality.ts:38`),
and `sourceSemanticBinding` refuses legacy (`numericalResultQuality.ts:18`). Both tests built their analysis record from
the legacy `fixtures/product_preview/invented_mechanics_result.json`, at `HandoffPanel.test.tsx:10` and
`HeadlessRunnerPanel.test.tsx:10` on HEAD. The panels' builders (`buildHandoffPackage`, `buildHeadlessRunnerPacket`) are
unchanged and refuse nothing.

**Decision: the refusal is intended, so the tests were stale.**
- PR905's `src/services/previewService.test.ts:563` asserts "analysis composition refuses legacy and unknown raw without a historical fallback".
- `fixtures/product_preview/PRECISION_FIXTURES.md` keeps the legacy fixture "for legacy read/hash/rounding tests".
- `bindSourceResultDimensions` now leaves non-legacy carriers untouched (`previewService.ts:296`).
- PR905's own transition for legacy rows that carry received dimension declarations is the historical-format protocol
  control `precision()` plus `referenceSession()` in `src/features/result-export/resultExportAdapter.test.ts:26,37`
  (the "reference-enriched" and "reference-absent" cases).

**Change.**
- Success path: legacy rows with their `bindSourceResultDimensions` declarations, under a precision-1 header, through
  the same pattern (`precisionCarrier`). The file comment says it is never native output and never Current.
- The manifest's solver name and version are now the producer that the header declares. The supported record requires
  this (`analysisRunCompatibility.ts:166`, `ANALYSIS_SOURCE_PRODUCER_MISMATCH`).
- Absent control: the same rows without declarations, under the same header.
- Added a non-vacuity check: the witness count equals the number of declared rows and is greater than 0 (830).
- Added a separate refusal test in each file: the legacy carrier is rejected with `SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED`
  and left unchanged.

**Preserved intent.**
- Every witness preserves the received value, unit and declared dimension from source to target, with no conversion.
- Rows without declarations produce no witnesses, and the runner adds `RUNNER_RECEIVED_DIMENSION_WITNESS_UNAVAILABLE`.

**Mutations** (each restored; sha256 verified against the HEAD blob):

| Id | Mutation | Log | Result |
|---|---|---|---|
| MH1 | `HandoffPanel.tsx` target dimension no longer preserved | `mutation-MH1-handoff-target-dimension.log` | success test fails |
| MH2 | `HandoffPanel.tsx` dimension filter removed | `mutation-MH2-handoff-dimension-filter.log` | absent control fails ("got 830") |
| MR1 | `HeadlessRunnerPanel.tsx` target dimension no longer preserved | `mutation-MR1-runner-target-dimension.log` | success test fails |
| MR2 | `HeadlessRunnerPanel.tsx` unavailable-witness diagnostic suppressed | `mutation-MR2-runner-unavailable-diagnostic.log` | absent control fails |
| MP1 | `previewService.ts:288` historical v0.2 fallback restored for legacy | `mutation-MP1-analysis-legacy-fallback.log` | both refusal tests fail |

## 2. RuleCheckRunPanel: `RULE_NATIVE_INVOCATION_REQUIRED` (5 cases)

**Root cause.** PR905's `runRuleChecks` requires the current model and a live native invocation registration before it
invokes `run_rule_checks` (`src/services/ruleCheckService.ts:118,120`). The desktop-run tests passed `modelStub` and
`resultStub`. These are the five tests at HEAD `RuleCheckRunPanel.test.tsx:312,367,428,460,502`, so the service threw
first and the panel showed `RULE-CHECK-BACKEND-ERROR (run): … RULE_NATIVE_INVOCATION_REQUIRED`.

**Decision: the requirement is intended.**
- PR905's `src/services/ruleCheckService.test.ts:252,259` rejects bundled references, unregistered pairs, clones,
  mutated sources and changed models.
- `…/ENGINE_INTEGRATION/PHYSICS_READER_JOIN/RULE_SOURCE_IMMUTABILITY/AUDIT.md` says reopened, cloned or imported history
  must receive no live registration.
- The App passes this panel the qualified `currentSolvedResult` (`src/App.tsx:906-909`).

**Change.**
- New `nativeSolvedBasis()`: the captured precision pair (the pair App.test.tsx uses for Current) is replayed through
  mocked IPC with `createNativeMechanicsReplay` and registered only by the production `runPreviewMechanics` route. It
  asserts registration and numerical eligibility, then resets the mock.
- The desktop-run tests render with it. Selections of `result:stress:demo` now use a real stress row id.
- The new-basis test uses two separately invoked solves (sparse, then dense) instead of `{...resultStub, run_id: "…-b"}`.
- Strengthened checks:
  - the rule backend receives the natively invoked source and its model;
  - the selected binding is forwarded;
  - the first call in the authored-reference test is `run_rule_checks`.
- New refusal control: a stub result and a byte-identical clone of the native source each give
  `RULE_NATIVE_INVOCATION_REQUIRED`. There is no `run_rule_checks` call, no result, the aggregate is lifted as null, and
  Run is re-enabled.
- A sixth, not-failing test passed vacuously on HEAD: "prevents an old-pack request from reviving findings…"
  (HEAD `:546`). Its run never reached the backend. It now uses the native basis and asserts that exactly one rule
  request was made.

**Preserved intent:**
- rendering of per-check outcomes, the aggregate and the relation, with no boundary notice;
- authored references supersede caller bindings;
- worst-of aggregate lift;
- same-ID basis retirement that keeps drafts;
- stale old-basis and old-pack runs cannot publish or clear the busy state.

**Mutations and probes:**

| Id | Mutation | Log | Result |
|---|---|---|---|
| MK1 | native gate removed (`ruleCheckService.ts:120`) | `mutation-MK1-rule-native-gate.log` | refusal control fails |
| MK2 | panel stale-run guard removed after the run | `mutation-MK2-rule-panel-stale-guard.log` | new-basis and old-pack tests fail |
| MK3 | panel forwards the authored reference as a caller binding | `mutation-MK3-rule-authored-ref-forwarded-as-caller-binding.log` | authored-ref test fails |
| probe | MK2 run against the HEAD test file | `probe-MK2-stale-guard-against-HEAD-test-file.log` | the old-pack test still passes, which shows it was vacuous |
| MK3a | only the authored-input filter removed | `mutation-MK3a-authored-filter-only-not-detectable.log` | not detected |

MK3a is a limitation that main already had, judged by inspection because the test logic is the same there. No selection
exists for an authored input, so a later `result_id.length > 0` filter drops it anyway.

## 3. sessionBoundary: guard offenders

**Root cause.** The guard `src/features/workspace/sessionBoundary.test.ts:57` flagged two PR905 test files. Both import
`../workspace/resultsSessionState` (HEAD `physicsSourceIntegration.test.tsx:11` and `sourceBlockRecovery.test.ts:24`).
They set the session cells directly through its setters (HEAD physics `:58-60,:82-83`; source blocks `:260-262,:278,:377-379`).

**Change.** Both files now use `useWorkspaceSession()`, as `currentResultUnitPolicy.test.tsx` does, with a unit
transport replay:
- `load_preview_model` returns the captured request model;
- the session's own `handleRun` backend job must dispatch exactly the captured request and mode;
- polling returns the captured producer bytes.

Current is then the session's `currentSolvedResult`, and the downstream export, stress-neutral, report and rule
assertions run on the session's own result, manifest and analysis-record triple. The guard is not modified. Both files'
`afterEach` also clears `localStorage`, which the session writes.

How each hook assertion was carried over:
- **Physics, Current identity.** Replaced by the session's Current being the session's own result object, plus the
  manifest mode equalling the requested mode. The direct-IPC registration premise is kept through the existing
  `received()`.
- **Physics, `badManifest` (HEAD `:82-83`).** The session never publishes a manifest for a mode it did not request
  (`workspaceSession.ts:858,888-892`). The test now does two things instead:
  1. It asserts the gate's own two predicates are false for the other mode: `hasNativeMechanicsInvocation(…, otherMode)`
     and `physicsSourceModeMatches(…, otherMode)`.
  2. It runs a session solve in the other mode that receives the bytes recorded under the first mode. Result and
     Current are null, and the job fails with `SOLVE_NATIVE_INVOCATION_BINDING_REQUIRED`.
- **Source blocks, result and manifest without an analysis record, gives null (HEAD `:261`).** This state cannot be
  built through the session, which publishes all three together (`workspaceSession.ts:888-892`). The shared gate clause
  is still covered by `src/features/workspace/resultsSessionState.test.ts:30`.
- **Source blocks, `setAnalysisRun(revision)` keeps Current (HEAD `:278`).** Replaced by the session's own
  `handleRuleCheckAggregate('USER_RULE_CHECKED')`. The published record equals the test-built revision, and Current
  and the manifest are retained.
- **FE02 (HEAD `:377-379`).** The route-specific direct/job registration, export and rule assertions are unchanged.
  Current admission of the same negative-zero caller model now goes through the session's job route:
  - Current is the result and equals the raw bytes;
  - the session model and manifest keep `-0`;
  - the dispatched request has `+0`;
  - the result is registered.

**Mutations** (product files restored and sha256-verified against HEAD; the MS5 test file restored to its repaired sha256):

| Id | Mutation | Log | Result |
|---|---|---|---|
| MS1 | Current gate closed (`resultsSessionState.ts:70`) | `mutation-MS1-current-gate-closed.log` | 22 session-driven cases in both files fail |
| MS2 | physics-source gate clause (`:71`) made false | `mutation-MS2-gate-physics-source-mode-clause.log` | the 14 physics cases fail |
| MS3 | source-blocks gate clause (`:72`) made false | `mutation-MS3-gate-source-blocks-mode-clause.log` | the 8 source-block Current cases fail |
| MS4 | session rule revision not published (`workspaceSession.ts:971`) | `mutation-MS4-session-rule-revision-not-published.log` | the 6 lifecycle cases fail |
| MS5 | original offending import re-inserted into the repaired physics test | `mutation-MS5-guard-still-active.log` | guard fails, naming it |

## Commands

All commands ran from `projects/chirality-piping` with the brief's Node 24 `PATH` prefix, except `tsc`, which ran in
`apps/desktop`. Environment: node v24.21.0, vitest 4.1.10, tsc 5.9.3, 4 CPUs (`_run_records/environment.txt`).

| Command | Result | Record |
|---|---|---|
| `npm test --workspace apps/desktop -- --run <4 failing files>` (HEAD, before edits) | 8 failed, exit 1 | `before-four-files-head.log` |
| `npm test --workspace apps/desktop -- --run <4 files + the 2 results files>` (after) | 6 files, 97 tests passed, exit 0 | `focused-six-files-after.log` |
| mutations and probes: `python3 mutation_inputs/mutate.py <log> <target> <old> <new> <expect: fail or pass> -- <tests>` | as tabled above | `mutation-*.log`, `probe-*.log`, `mutation_inputs/` |
| `VITEST_MAX_WORKERS=2 npm run test:desktop` | 118 files, 2057 tests passed, 0 failed, exit 0, 434s | `full-desktop-vitest.raw.log` |
| `npx tsc -b` (in `apps/desktop`) | exit 0, no diagnostics | `tsc-b.log` |

Runtime: the physics file takes about 108s against about 105s on HEAD (measured verbose, same machine).

Mutation procedure (`_run_records/mutation_inputs/mutate.py`): apply one replacement whose old snippet occurs exactly once,
run the focused files, restore by byte copy, then verify the restored sha256 against the pre-mutation bytes and the HEAD blob,
with an empty `git diff`. Each log records these values. The MK3a snippets appear only in its own log, because MK3 reused the
snippet file names. In `mutation-MP1-analysis-legacy-fallback.log` the 25k-line dump of the resolved record is elided, and the
log marks each elision.

## Unresolved / for the caller

1. **Coverage gap: the gate's manifest-mode wiring for physics-source.** This is `resultsSessionState.ts:71` and the
   mode argument at `:73`.
   - The HEAD test caught its removal at `:83` (`probe-GAP-gate-ignores-manifest-mode-HEAD-test.log`: fails). The
     repaired test does not (`probe-GAP-gate-ignores-manifest-mode-repaired-test.log`: passes).
   - The state is unreachable through `useWorkspaceSession`. The mode predicates themselves stay covered here and in
     `physicsSourceRecovery.test.ts:87-88`.
   - Remedy, outside my write boundary: add a mismatched-manifest-mode control for physics-source (and ideally
     source-blocks, `:72`, which had no such control on HEAD either) to `src/features/workspace/resultsSessionState.test.ts`.
     The guard's own-module rule permits setters in that file. This needs caller authorization. I did not weaken or
     work around the guard.
2. Current producer fixtures carry no row-level `dimension`. On real precision/physics sources the handoff and runner
   therefore produce zero unit witnesses and disclose each row as unavailable. This is PR905 behaviour ("dimension
   absence is not producer attestation"). The repaired tests prove the preservation path only with the historical-format
   control carrier.
3. Evidence scope: every native transport in these tests is a unit replay of captured bytes. None of it is native UI
   qualification or rule-evaluation evidence. Independent review, integration, commit and the DEC-025 sweep belong to
   the caller.
