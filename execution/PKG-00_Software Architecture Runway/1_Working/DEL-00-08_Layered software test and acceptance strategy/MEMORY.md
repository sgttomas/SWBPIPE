# MEMORY: DEL-00-08

## 2026-06-12 - TP-APP-R2-FROMBLANK-E2E-001

- WORKING_ITEMS landed the next A8 Playwright harness sub-slice.
- `apps/desktop/e2e/r2-smoke.spec.ts` now includes a second browser e2e test
  that clicks `New blank`, uses the A12 fixture values, authors two nodes,
  material, section, pipe, support, load case, primitive load, and combination
  through visible GUI controls, and applies all nine operations through the
  structured-operation seam.
- The test asserts local wasm operation application and
  `professional_approval=false` for every applied intent.
- Browser mode then honestly reports the current desktop/backend boundary:
  edited-model mechanics returns zero rows with
  `BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL`, and rendered report
  remains `REPORT-RENDERER-DESKTOP-ONLY`.
- Validation passed: `npm run test:e2e --workspace apps/desktop` 2/2
  Playwright tests, `npm test --workspace apps/desktop` 213/213 Vitest tests,
  and `npm run build --workspace apps/desktop`.
- Residual A8 scope remains: packaged Tauri/backend saved-project solve +
  render smoke and broader SMOKE checklist parity.

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
