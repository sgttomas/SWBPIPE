# MEMORY: DEL-00-08

## 2026-06-04 - TP-AUTHORITY-REFRESH-0_7-DAG006

- WORKING_ITEMS/TASK workforce current-authority refresh applied to active deliverable-local surfaces for `DEL-00-08`.
- Current authority basis is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.7` plus approved `execution/_DAG/DAG-006/` active graph authority.
- Historical run records, historical DAG row IDs, review dispositions, lifecycle `_STATUS.md`, aggregate DAG artifacts, candidate edges, repo-level governance files, schemas, code, and tools were intentionally not changed by this refresh.
- Preserved historical references remain evidence of earlier work, not current authority claims.

## 2026-06-11 - TP-APP-R2-PLAYWRIGHT-001

- WORKING_ITEMS landed the first A8 Playwright GUI smoke harness slice.
- Root `npm run test:e2e:desktop` now delegates to the desktop workspace
  Playwright run; `apps/desktop/playwright.config.ts` starts/reuses the Vite
  dev server on `127.0.0.1:5174` and uses system Chrome when available.
- `apps/desktop/e2e/r2-smoke.spec.ts` covers the technical-preview shell,
  local-only boundary signals, nonblank/changing viewport canvas, preview
  solve, `result_rows=647`, displacement-overlay status, result
  filtering/detail metadata, and deterministic report-packet export flags.
- Validation passed: `npm run test:e2e:desktop` 1/1 Playwright tests,
  `npm test --workspace apps/desktop` 31/31 Vitest tests,
  `npm run build --workspace apps/desktop`, and `git diff --check -- .
  ':!init/init-prompt.md'`.
- Residual A8 scope remains: authored create/edit -> solve -> report journey,
  full manual SMOKE checklist parity, packaged Tauri saved-project solve
  smoke, and CI browser provisioning policy.
