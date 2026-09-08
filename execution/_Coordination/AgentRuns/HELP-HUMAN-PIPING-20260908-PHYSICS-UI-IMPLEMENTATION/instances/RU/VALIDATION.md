# RU validation record

## Frozen identity and containment

- Sealed launch brief SHA-256: `415600c5f60e722a980a32ffb06d08f811c41d64918be8a8ab4b8cac9eb97f8d` — match.
- Frozen manifest SHA-256: `9f71f16fe91112a42ed01fc69aee040665c4c1663d3e9d5a02d8df00747f4446` — match.
- Frozen diff SHA-256: `df93c21cb945f6ed5d3e42ecbfd8f4f25ef462ba5c453d76757b36d2b2fa2056` — match; 2,280 lines; nine path headers.
- All 19 manifest-declared members independently hashed `MATCH`; exact output is `_run_records/hash-verification.txt`.
- `git diff --check` over the nine-path fence exited 0. The six tracked changed paths and three declared untracked files equal the expected fence; no source was written by RU.

## Focused commands

All paths below are repository-relative in narrative form; exact host paths remain in the captured command output where applicable.

- `npx vitest run src/features/viewport/routeDraft.test.ts` — PASS, 12/12, 360 ms total.
- `npx vitest run src/features/model-tree/typedInspector.test.tsx` — PASS, 9/9, 971 ms total.
- `npx vitest run src/App.test.tsx -t 'native straight-route Add and Apply|retains a malformed restored member|handles offline null'` — PASS, 8/8 selected, 124 skipped, 14.03 s.
- `PLAYWRIGHT_WORKERS=1 npx playwright test e2e/linear-authoring.spec.ts --project=chromium-desktop` — PASS, 1/1, 12.6 s; the test itself forces and asserts 1024×768.
- Task-private `review-findings.test.tsx` — final PASS, 6/6, 3.41 s. Four tests intentionally assert confirmed defective outcomes; one asserts the intra-route ID defect; one independently confirms structural equality ignores object-key order and rejects array reversal/value mutation.

The focused production checks passing alongside the private reproductions establishes missing coverage rather than a flaky production suite.

## Failed-command preservation

Three review-command failures are preserved losslessly under `_run_records/FAILED_COMMAND_001.md`, `_run_records/FAILED_COMMAND_002.md`, and `_run_records/FAILED_COMMAND_003.md`, with redirected raw outputs retained in `_run_records/repros/vitest-output-attempt2.txt` and `_run_records/hash-verification-attempt1.txt`. They were review-harness issues: a zsh reserved variable, an omitted React import/private mock field, and zsh `path` shadowing. Corrected reruns passed.

## Exclusions and cleanup

The scoped browser checks used the existing prebuilt WASM service. No Rust, WASM, or native build, native-app run, full harness, evidence sweep, Git mutation, source edit, dependency-row action, lifecycle action, commit, or push occurred. The Playwright-created `apps/desktop/test-results/.last-run.json` and now-empty directory were removed after its result had been captured in RU records. RU retained only its allowed review documents and `_run_records` evidence.

Actual model identity is not exposed for mechanical self-verification. The parent dispatch requested `gpt-5.6-sol` with high reasoning; this record states that requested configuration rather than claiming an independently verified runtime identity. Parent: `/root` HELP_HUMAN. Role: fresh bounded ephemeral Agent 2 generalist. No delegation occurred.
