# WORKING_ITEMS Run — R8 packaged GUI edited-load smoke

| Field | Value |
|---|---|
| RunID | `HELP-HUMAN-PIPING-20260820-R8-PACKAGED-GUI` |
| InstanceID | `WI-PKG07-DEL0706` |
| Package / deliverable | `PKG-07` / `DEL-07-06` |
| Basis | branch `codex/piping-product-20260820`; source-tree base `91b92f949b8d808b61268baca9a54f4f716c99ff`; Receipt-118/DAG-009/R5 selection basis plus owner-authorized retry |
| Objective | Close only the exact packaged-Tauri saved edited-load GUI residual |
| Outcome | `PASS` |

## Actual packaged GUI journey

WORKING_ITEMS used Computer Use (`@oai/sky`) against the actual macOS Tauri
`.app`. No browser or headless substitute was accepted. All model data was the
invented repository fixture.

1. Expanded the model tree, selected `load:L-100`, opened Load Cases, selected
   primitive `load:L-100-Y`, and observed `350 N`.
2. Entered `425`, queued
   `op:load-manager-load:L-100-load:L-100-Y-magnitude`, and applied the
   structured operation. The GUI reported `tauri_backend_apply`, one applied
   operation, zero pending operations, cleared solve state, and
   `session_state_only_not_yet_saved` before save.
3. Used `Create local` and `Save local`, then quit with `super+q` and relaunched.
   The fresh launch reported no local store open and `not started; result rows=0`.
4. Used `List local`, observed one saved snapshot, and explicitly reopened
   `project:invented-loop-01`. The reopened load editor showed `425 N` and
   `no changed magnitude queued`. Project summary showed zero pending
   operations, zero applied operations, zero editor intents, and zero agent
   proposals; the viewport still showed `not started; result rows=0`.
5. Ran mechanics. The initial package exposed `MECHANICS_SOLVED`, 830 result
   rows, and an available deformation view, but the explicit backend route and
   model/result identity were buried in the long solve-audit surface and were
   not reliably observable.

The observability failure was amended in place under `AMENDMENT_2.md`. Existing
state already carried `solveJob.backend_job_seam`, `model.project.id`,
`result.model_ref`, and result-row count. The repair renders these unchanged
values in a global `Solve proof` status row; no schema, backend, persistence,
solver, operation, or result-envelope contract changed.

After the repair, a fresh `.app` was built. The existing saved invented
snapshot was reopened again. Before Run it showed zero pending/applied/editor/
proposal state and zero result rows. Packaged Run then exposed:

```text
Mechanics = MECHANICS_SOLVED
Viewport = available; nodes=5; max=4.569777 mm
Solve proof = seam=tauri_backend_job; project=project:invented-loop-01; result_model=project:invented-loop-01; identity=match; rows=830; generation=6; job=backend-solve-job-1
Model SHA-256 = sha256:0f92378afb16d87c6b2e29c1c7dcdcf30ef555eda6c6c814e64c70bb5a231ea5
Input-manifest SHA-256 = 0b49bea1cbf6126a0d5ff896297b30cf2b84d4a212d7c538bcbfa137041d22c3
```

This final post-reopen repetition was the bounded proof-loop rerun explicitly
authorized after the observability repair; the original edit/save/relaunch/
reopen observations remain part of the same frozen N1 node.

## Package and evidence identity

- Bundle:
  `apps/desktop/src-tauri/target/release/bundle/macos/OpenPipeStress Technical Preview.app`
- Final executable SHA-256:
  `3a95da3d2269b124734bc22a0e46d820e3fdb6870630159757ff8bab30507a8c`
- Source-tree base: local commit
  `91b92f949b8d808b61268baca9a54f4f716c99ff`; the R8 node diff was
  uncommitted during execution and is assigned its integration commit by
  CHANGE after validated fan-in.
- Reopened-unsolved screenshot:
  `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-R8-PACKAGED-GUI/WI-PKG07-DEL0706/PACKAGED_REOPENED_UNSOLVED_V6.jpeg`
- Final solved-proof screenshot:
  `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-R8-PACKAGED-GUI/WI-PKG07-DEL0706/PACKAGED_SOLVED_PROOF_V6.jpeg`
- Earlier screenshot pairs and reviewer returns preserve every preceding
  proof/review attempt and actionable FAIL.

## Checks

| Check | Result |
|---|---|
| Focused App Vitest for proof binding and real deferred lifecycle callbacks | PASS, 7 selected / 60 skipped |
| Focused Playwright `gui-workflow-validation.spec.ts` | PASS, 2/2 (`chromium-desktop`, `chromium-compact`) |
| Registered `desktop-test` (`npm run test:desktop`) | PASS, 29 files / 533 tests |
| Registered `desktop-build` (`npm run build:desktop`) | PASS; existing Vite chunk-size warning only |
| `npm run tauri -- build --bundles app` | PASS; fresh macOS `.app` bundled |
| Actual packaged Computer Use journey | PASS; predicates above |
| Registered harness self-check | PASS execution (exit 0); existing repository-wide generated-view findings reported |
| Registered harness pytest | PASS, 350 tests |
| Fresh frozen integrated-diff software-code-review 1 | FAIL with one blocking stale-proof finding plus one review-brief control defect; preserved and remediated |
| Fresh frozen integrated-diff software-code-review 2 | FAIL after 23/23 hashes: overlapping native-run callbacks not generation-bound; evidence overclaim |
| Fresh frozen integrated-diff software-code-review 3 | FAIL after 31/31 hashes: post-commit invalidation, lost pre-start cancellation, callback coverage overclaim |
| Fresh frozen integrated-diff software-code-review 4 | FAIL after 40/40 hashes: model transition dropped a requested cancellation before the backend start receipt |
| Fresh frozen integrated-diff software-code-review 5 | PASS; 47/47 hashes, exact 48-path containment, exact profile/check evidence, 100% review, no actionable finding |

## Scope and claim boundary

- Product behavior writes are limited to `apps/desktop/src/App.tsx`, its
  Vitest test, and the focused Playwright workflow assertion. Supporting
  evidence/status writes are within the authorized DEL-07-06 and exact R8
  AgentRuns surfaces.
- The exact packaged edited-load residual is removed only because every named
  GUI predicate was observed on the actual packaged runtime.
- PDU-045 and PDU-046 remain `VERIFIED_NOT_VALIDATED`; the measurable target
  remains `TBD_by_human_project_authority`.
- No lifecycle, target-stage, release, issuance, professional-reliance,
  accessibility-conformance, certification, sealing, authentication, or
  code-compliance effect is claimed.

## Review remediation

Reviewer 1 verified 16/16 frozen hashes and containment, then rejected the
first implementation because retained old results could be combined with a
new running/cancelled/failed solve job. `AMENDMENT_3.md` authorized the
correction. The row now exists only when the current solve is completed and
its job ID/backend seam, exact current model SHA-256, input-manifest SHA-256,
result run/model identity, and row count all match. A model revision during an
in-flight solve rejects late publication. Focused tests cover completed,
running, cancelling, cancelled, failed, different-job, changed-hash, and
late-result cases. The corrected package was rebuilt and the bounded
post-reopen host observation above repeated.

Reviewer 2 then found an overlapping native-run callback race; Amendment 4
added synchronous generation acquisition and bound every terminal callback.
Reviewer 3 found post-commit model invalidation, lost ordinary pre-start
cancellation, and non-integrated callback tests; Amendment 5 moved invalidation
before model commit, retained immediate cancellation until the start receipt,
and added rendered deferred native-event coverage. Reviewer 4 found the
remaining Run -> Cancel -> Open -> delayed receipt sequence: UI state was safe,
but invalidation discarded the cancellation before it could reach the newly
created backend job. Amendment 6 retains a detached generation tombstone until
that receipt, dispatches cooperative cancellation exactly once, and keeps all
stale callbacks UI-inert. The exact rendered sequence now passes, as do the
seven-test focused lifecycle set, registered 533-test desktop suite, build,
package, focused Playwright, harness checks, and final V6 packaged reopen/solve
observation.

Fresh reviewer 5 closed all four prior findings and returned
`PASS / VALID_FOR_TERMINAL_FAN_IN`. Its non-blocking residuals are limited to
sequential screenshot evidence versus mocked timing adversity, the narratively
observed `425 N` not appearing in the final screenshot pair, and an
already-started invalidated backend poll continuing to terminal while every
stale publication/finalization callback remains gated.
