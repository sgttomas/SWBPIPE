# WORKING_ITEMS Run Record — Playwright R2 Smoke Harness

- Date: 2026-06-11
- Agent: WORKING_ITEMS (Type 1 persona), app-integration tranche under
  `execution/_Coordination/_COORDINATION.md` Application Integration And
  Issuance Loop.
- Tranche: `TP-APP-R2-PLAYWRIGHT-001`, completion plan Phase A8 first
  sub-slice.
- Deliverable context: DEL-00-08 (layered software test and acceptance
  strategy).

## What Changed

- Added root `npm run test:e2e:desktop` and desktop workspace
  `npm run test:e2e` scripts.
- Added `@playwright/test` as a desktop dev dependency and
  `apps/desktop/playwright.config.ts`.
- Added `apps/desktop/e2e/r2-smoke.spec.ts`, which runs against a Vite dev
  server on `127.0.0.1:5174`.
- Scoped Vitest to `src/**/*.test.{ts,tsx}` so Playwright specs are not
  imported by the unit runner.
- Ignored local Playwright output directories under `apps/desktop/`.

## Automated Coverage

- Initial desktop technical-preview shell renders.
- Local project boundary text retains `network=false` and `telemetry=false`.
- Three.js viewport canvas is visible, nonblank, and changes across a 700 ms
  animation interval.
- Mechanics preview solve completes with `result_rows=647`.
- The viewport displacement overlay reports
  `available; nodes=5; max=33.211157 mm` and retains the normalized-display
  boundary.
- Results filtering exposes `result:force:pipe-P-120:axial`; the detail panel
  confirms `pipe:P-120` and `recovered_from_local_element_stiffness`.
- Report-packet export JSON confirms document kind, run audit ref, and false
  private/protected/release flags.

## Validation Evidence

- `npm run test:e2e:desktop`: 1 passed, 0 failed.
- `npm test --workspace apps/desktop`: 31 passed, 0 failed.
- `npm run build --workspace apps/desktop`: TypeScript check and Vite
  production build passed.
- `git diff --check -- . ':!init/init-prompt.md'`: passed for touched scope.

## Boundary Review

- This is a first automated smoke harness over the existing invented preview
  fixture. It does not yet automate the full authored create/edit -> solve ->
  report journey from a blank project.
- It does not add solver validation, protected standards text, private project
  data handling, network/cloud/telemetry paths, release readiness,
  professional approval, certification, sealing, authentication, or
  code-compliance claims.
- No lifecycle state change: DEL-00-08 `_STATUS.md` remains `CHECKING`.
