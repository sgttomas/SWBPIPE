# B3B implementation return

Frozen, uncommitted implementation on base `88c6c7463199d1d69caebde04678fa74facca78f` in assigned wt3. TASK `/root/b3_manager/b3b_implementation`, fresh native collaboration child of WORKING_ITEMS `/root/b3_manager`; supplied attribution Astra/low. No descendants. Host unrestricted; brief/resource/write boundaries instruction-enforced. Verified WORKER_BRIEF SHA256 `7f65af39f4f71318f79e503263e73c08645191400a4df4ae50eb63369a41d8f9` and LAUNCH_INPUTS SHA256 `3dab806393f018f2bcec13ed129db96f73873335772b0e0dad51a36248bfa996` before work. Applied software-defect-diagnosis. Additional actual inputs and origins/hashes: `_run_records/worker/CONSULTED_INPUTS.json`.

Source freeze: `_run_records/worker/SOURCE_FREEZE.json`, SHA256 `ded873f772afe235d2b29ba20e855d7653364c1e8ba2a30baca943300af4fa51`. It inventories all 13 product/test paths, including untracked files. Manager notified before return; no product/test edits after freeze. No staging, commits, push, PR, build command, browser/UI/native runtime, C3 or CI writes.

## Implemented behavior

- Ref-backed synchronous owner covers New Local, New Blank, Open (including explicit ID through the same handler), Save and List before request counters or awaited work. Only the owner releases busy in finally; preparation that can throw is inside that try. Both native and pointer routes reach these handlers. Rejected commands do not advance projectRequest. Native shell state includes projectBusy; Rust disables all five File entries, including while the initial model is absent. Report-package/edit ownership stays independent.
- The bounded dispatch guarantee is rejection of native commands delivered while the owner is busy. No knowledge or rejection guarantee is claimed for an event first delivered after busy ends. Native presentation is an asynchronous projection, not the authority.
- Save/Create retire the old observation while returned-byte verification is pending, then record source, local observation time, snapshot scope, exact persisted/recomputed values and canonical comparison. Service failure before fulfillment retains prior evidence. Verification exceptions after fulfillment produce unavailable evidence; missing legacy claims remain not_persisted. Response identity mismatch is diagnosed and cannot adopt metadata/model or claim verified_match. Same-session edits do not prevent snapshot observation, but retain the existing stricter model/history adoption gate.
- Existing canonical saved-basis and Historical behavior remain. Production-used verifiedWriteBasis/isLocalModelEdited retain generation, fulfillment-order and hash-owned comparison decisions. A verified exact retained Historical carrier has explicit retained_historical_carrier standing while canonical integrity remains mismatch_review_required. Missing standing displays not_recorded. No persisted model/envelope field was introduced.
- Validation copy uses actual open/save/create source. Storage Audit now downloads both existing observations and receives envelope evidence from App. Both displays distinguish persisted snapshot evidence from later unsaved content. Actual data-URI JSON equality is tested.

## Diagnosis and oracle dispositions

Fail-before busy test observed List, Open, duplicate List, Create and Save calls in one busy interval instead of List alone. Fail-before integrity tests exposed missing source/time and a wrong-project claim accepted solely by equal value. Raw output retained; diagnosis/approach sent to manager before repair.

Formerly reachable setups amended by explicit owner busy policy (original approximate lines):

1. 308 List→Save overlap now asserts blocked Save, then a separately owned Save after List release.
2. 528 Save→missing Open overlap now asserts blocked Open and prior mismatch preservation after failed Save.
3. 553/586/613 old clear-on-landed-write assertions become snapshot evidence after Save/Create, including a newer UI edit. Create synthetic fixture now maps native flat camelCase modelHash/projectEnvelopeHash into actual persisted snake_case response fields. Missing claims still have dedicated not_persisted checks.
4. 740 delayed Open hash→missing Open now proves the second Open is blocked while retaining edit/Undo canonical baseline behavior.
5. 790 delayed Open hash→Save now proves Save is blocked until verification, then saves the edited model and retains Undo comparison.
6. 821 Save→same-ID replacement now proves replacement blocked until Save release, then verifies reopened baseline/history reset. Production helper additionally rejects old-generation publication with the same ID.
7. Historical delayed Open→Save setup now explicitly releases Open verification after asserting the blocked Save. The subsequent save preserves exact carrier, Historical context, no Current solve chip, no undo, and canonical mismatch/retained-carrier labels.
8. 991 P2 later missing/failed Open is blocked during Save, then executed after fulfillment; landed canonical B, preserved observations, Undo A/Redo B remain asserted.
9. 1024 P2 concurrent writes become blocked second dispatch during both backend wait and hash verification, then sequential valid/invalid writes with the same canonical Undo/Redo checks. Separate production-used helper tests supply actual canonical A/B hashes with reverse verification completion; later valid A wins, later invalid A does not retire earlier valid B, old generation is rejected, unowned live hash cannot clear Edited.
10. App.test original ~16601 preserves stale batch/publication assertions: Open is blocked during Save; Save fails/relinquishes ownership; Open then replaces; the unreadable stale batch is discarded. Saved-basis generation coverage is in production helper tests above.
11. App.test ~3739/3753/3946/3961/8536 initial null observation copy changes from open-only to persistence-verification wording. All other assertions in the initial packet and persistence roundtrip cases remain. Existing verified_on_open labels remain.

## Canonical checks and evidence

All logs below are under `_run_records/worker/`; SHA256 inventory is `EVIDENCE_HASHES.json`. Commands use assigned wt3 explicitly; Vitest/TypeScript cwd is `projects/chirality-piping/apps/desktop`; Rust cwd adds `src-tauri`. Darwin arm64, Node v24.18.0, npm 11.16.0, rustc 1.97.1 (8bab26f4f 2026-07-14), cargo 1.97.1 (c980f4866 2026-06-30), Vitest 4.1.7. jsdom logs its existing unavailable WebGL context diagnostics; these are not native evidence.

- `npx vitest run src/App.projectHandlers.test.tsx -t 'B3B synchronous' --maxWorkers=1`: expected fail, 1 failed/33 skipped, 2.04 s. `fail-before-busy.log`.
- `npx vitest run src/features/workspace/projectPersistenceIntegrity.test.ts --maxWorkers=1`: expected failures before source repair. `fail-before-integrity.log`.
- `npx vitest run src/App.projectHandlers.test.tsx src/features/workspace/projectPersistenceIntegrity.test.ts --maxWorkers=1`: initial oracle inventory failures in `handlers-1.log`; 39/40 in `handlers-2.log` (Create fixture mapping above); 41/41 in `handlers-3.log` (13.01 s).
- `npx vitest run src/features/workspace/projectPersistenceIntegrity.test.ts src/App.projectHandlers.test.tsx --maxWorkers=1`: 42/42, 12.79 s, `handlers-final.log`.
- `npx vitest run src/App.test.tsx -t 'renders the engineering workspace from invented local fixtures|round trips local create, save, and open project controls|blocks replacement during Save' --maxWorkers=1`: 3/3 selected, 215 skipped, 9.67 s, `app-focused-1.log`.
- `npx vitest run src/App.projectHandlers.test.tsx -t 'B3B projects native busy state' --maxWorkers=1`: first added test failed because invoke includes an explicit undefined second argument, preserved `no-model-native-state.log`; corrected assertion, no production relaxation.
- `npx vitest run src/App.projectHandlers.test.tsx -t 'B3B projects native busy state|B3A verifies unchanged Historical' --maxWorkers=1`: 2/2, 34 skipped, 1.94 s, `no-model-historical-final.log`.
- Final affected backcheck after moving synchronous preparation inside owner try and making absent standing explicit: `npx vitest run src/App.projectHandlers.test.tsx -t 'B3B synchronous|B3B projects native|re-derives verified-at-save|retaining the newer edit|B3A verifies unchanged Historical|failed save or create|blank create' --maxWorkers=1`: 11/11, 25 skipped, 4.23 s, `final-edge-backcheck.log`.
- `cargo test --lib native_shell_state`: 3/3, 99 filtered out; test-profile compilation 13.83 s, test execution 0.00 s, `rust-native-state.log`.
- `npx tsc --noEmit -p tsconfig.json`: passed; `typescript-freeze.log` (empty successful output). Initial missing type import failure retained in `typescript-1.log`; subsequent passed logs retained. Duration not separately measured.
- `git diff --check`: passed, empty output; duration not separately measured.

## Remaining verification and limits

The new `e2e/b3b-project-persistence.spec.ts` is one connected test using the existing two configured viewport projects and attaches browser identity. It covers synthetic blank/create/save/list/open, source transitions and actual JSON equality at 1440×920 and 1280×800 through existing public controls. It is prepared, not run. Browser resources/native app/build/full suites were outside this TASK's grant. No native menu visual/interactive witness, performance, release, engineering qualification, acceptance or C3 claim follows. Manager/ROOT retain native/browser witnessing, independent frozen-diff review, batch integration and full required checks. No permission request is pending from this TASK.

## Manager backcheck correction before final freeze

Intermediate freeze retained at `SOURCE_FREEZE_INTERMEDIATE.json` (SHA256 214e6b1cb949fe673200973e62e97532144a07007e0de5d990025dd9e97da637). Manager identified null/undefined fulfilled responses causing a second dereference inside catch. The narrow regression reproduced 2 failures and 2 unhandled errors (`malformed-fail-before.log`, 4.16 s). Catch now publishes unavailable evidence only when returned model/project summary identity is inspectable and bound to the request; uninspectable responses retain the truthful null observation, report failure and release the owner. Expanded null/undefined Save and Create checks plus adjacent failure/edit/delayed-verification checks pass 11/11 (29 skipped), 4.18 s: `npx vitest run src/App.projectHandlers.test.tsx -t 'B3B handles malformed|failed save or create|retaining the newer edit|B3B observes landed' --maxWorkers=1`, `malformed-pass-after.log`. TypeScript passes again (`typescript-refreeze.log`) and diff check passes. Browser spec now relies on configured viewports to avoid four duplicated runs; remains unrun. Final freeze supersedes the intermediate only for candidate identity, preserving all prior evidence.
