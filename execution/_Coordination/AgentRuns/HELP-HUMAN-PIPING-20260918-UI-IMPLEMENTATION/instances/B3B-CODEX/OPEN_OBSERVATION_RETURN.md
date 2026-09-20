# Open observation correction return

TASK /root/b3_manager/b3b_implementation, same Astra/low allocation, no delegation. Verified sealed repair brief SHA256 `7f774a40c126c4c213647915215ee5ff088068c1b577588a13217425dc011128`. Base `52f1baf01bf90f6af959042cc94e18366f48f1ae`. Original WORKER_RETURN, SOURCE_FREEZE and evidence remain immutable.

Source frozen in `_run_records/worker/OPEN_OBSERVATION_FREEZE.json`, SHA256 `1846e69325983d63a22952e21adc189d83bce308a963dbcaea5d43ffaa6baa63`. Five changed maintained paths: workspaceSession.ts, projectPersistenceIntegrity.ts and its test, workspaceSession.shell.test.tsx, App.projectHandlers.test.tsx. Browser scenario, viewport, all precommit and model/history adoption gates unchanged.

## Cause and repair

Both real browser cases reached reopened persisted claims but no observation. Source traced Open commit/captured requestEpoch followed by awaited hash; PipeViewport model and selection effects call invalidateDraftReview → onInvalidateDraft → session.invalidateDirectDraftContext and advance UI epoch. That invalidates the old stillCurrent guard even though the snapshot is already adopted. This exact browser scheduling is source-inferred, not instrumented. Navigation alone uses plain chrome state and does not advance that epoch.

New deterministic fail-before calls the actual session invalidateDirectDraftContext callback while committed Open hashing is held: model integrity remains null. The narrow repair replaces only the two postcommit observation guards with production-used ownsOpenPersistenceObservation. It checks captured request, generation, project identity and openedBasisSequence. Draft invalidation and subsequent same-session local edits no longer drop persisted-snapshot observations; replacement, newer requests, different identities and newer saved-basis ownership reject old observations. Existing precommit/catch gates, canonical saved-basis publication and model/history adoption are untouched.

## Checks

Assigned workdir explicitly used throughout. Vitest/TypeScript cwd projects/chirality-piping/apps/desktop; one Vitest worker. Same Darwin arm64/Node v24.18.0/npm11.16.0/Vitest4.1.7 environment as prior return. Canonical hashes in `_run_records/worker/OPEN_OBSERVATION_EVIDENCE_HASHES.json`.

- `npx vitest run src/features/workspace/workspaceSession.shell.test.tsx -t 'publishes committed Open observations' --maxWorkers=1`: expected fail, one failed/nine skipped, 1.04s. `open-observation-fail-before.log`. Expected verified open record, received null after actual callback.
- `npx vitest run src/features/workspace/workspaceSession.shell.test.tsx src/features/workspace/projectPersistenceIntegrity.test.ts src/App.projectHandlers.test.tsx -t 'publishes committed Open observations|keeps Open snapshot ownership|retains the opened baseline|blocks Save through delayed Open|blocks same-ID replacement' --maxWorkers=1`: five passed,52 skipped,3.64s. `open-observation-pass-after.log`. Covers actual invalidation callback, exact owner rejection, newer local edit with open snapshot evidence and canonical Undo, blocked delayed-Open Save, and same-ID replacement sequencing.
- `npx tsc --noEmit -p tsconfig.json`: passed, empty `open-observation-typescript.log`; duration not separately measured.
- `git diff --check`: passed; empty output, duration not separately measured.

No tests/UI/browser/native/build beyond the expressly granted narrow unit/TypeScript checks. The identical browser cases remain manager-owned for rerun. No qualification/acceptance claim. Source released to manager; no further edits planned.
