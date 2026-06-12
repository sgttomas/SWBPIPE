# MEMORY: DEL-00-08

## 2026-06-12 - TP-INTEGRATED-VERIFY-002

- Independent verification session re-ran the R2 exit-chain surfaces (Tauri
  32/32, product_physics 25/25, units 13/13, Playwright 2/2) and produced the
  Phase A exit-evidence snapshot
  `plans/VERIFICATION_2026-06-12_r2_exit_chain.md`.
- Packaged-binary boot smoke recorded as SMOKE `TP-MAC-140` with the H4
  exception note (tauri-driver lacks macOS support; packaged GUI journey gap
  stays in A5/A8 as finding F-4).
- Findings F-1 (load-category coercion) and F-2 (sweep git-state false-clean)
  were repaired same-session (`TP-R2VERIFY-FIX-001`/`-002`); F-3
  (export-schema compat stance) routed to the next PKG-17 tranche.
- Evidence: `_run_records/WORKING_ITEMS_RUN_2026-06-12_r2_exit_chain_verification.md`;
  stage advancement awaits the human `D-14` ruling.

## 2026-06-12 - TP-APP-R2-SAVEDPROJECT-SMOKE-001

- WORKING_ITEMS landed the A8 saved-project backend smoke sub-slice.
- `apps/desktop/src-tauri/src/lib.rs` now has
  `r2_from_blank_saved_project_opens_solves_and_renders_report`, which applies
  the A12 from-blank rehearsal steps, saves the authored model into the local
  SQLite project store, reopens it by project id, solves the loaded model
  through `run_preview_mechanics`, and renders through
  `render_calculation_report`.
- Validation passed: focused Tauri regression 1/1 and full
  `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` 31/31.
- Residual A8 scope remains: broader SMOKE checklist parity.

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

## 2026-06-12 - TP-APP-R2-WASMPKG-001

- Regression repair: the packaged `.app` (`tauri build`) failed the first
  authoring step ("New blank") with `WASM-ENGINE-ASSET-ABSENT: generated glue
  module import failed: TypeError: 'text/html' is not a valid JavaScript MIME
  type`.
- Root cause: all frontend hashing/operations route through the wasm engine in
  every mode by design (H1/F-5a, DEC-020/ADR-0001, no fallback), but the
  generated glue lived under `src/services/wasmEngine/__generated__/`, which
  the Vite dev server serves and `vite build` never emits into `dist`. The
  packaged tauri:// asset protocol 404'd onto `index.html` and the dynamic
  import died on the text/html MIME. `tauri.conf.json` `beforeBuildCommand`
  was plain `npm run build`, so a fresh `tauri build` was not self-sufficient
  either.
- Fix shape: `scripts/build-wasm-engine.mjs` now emits the glue + wasm into
  `public/wasm-engine/` (Vite publicDir: served at the site root in dev AND
  copied verbatim into `dist`), keeping the DEC-025 F-4 atomic rename swap and
  cleaning the legacy `__generated__` location. `loadWasmEngine.ts` browser
  lane imports a fully-qualified `new URL("/wasm-engine/...", location.href)`
  URL (Vite's dev `injectQuery` helper skips protocol-carrying URLs; the
  root-absolute path form gets `?import` appended, which the dev server
  refuses for publicDir files). Node/Vitest lane probes the new on-disk
  candidates and imports the glue via `pathToFileURL`. The
  `WASM-ENGINE-ASSET-ABSENT` + remediation failure text is unchanged; no
  second engine, no silent fallback.
- Loud build-time guard: `vite.config.ts` plugin `wasm-engine-dist-guard`
  fails `vite build` at buildStart when `public/wasm-engine/` is missing the
  glue/wasm, and at closeBundle when `dist/wasm-engine/` is — named
  `WASM-ENGINE-ASSET-ABSENT` error with the exact `npm run build:wasm
  --workspace apps/desktop` remediation. Demonstrated: removing the assets
  fails the build with that message; restoring them passes.
- `tauri.conf.json` `beforeBuildCommand` is now `npm run build:wasm && npm run
  build`, so a fresh `tauri build` is self-sufficient.
- New production-dist Playwright lane: `npm run test:e2e:dist --workspace
  apps/desktop` (root alias `test:e2e:dist:desktop`) builds wasm + dist and
  runs `playwright.dist.config.ts` against `vite preview` of `dist/`;
  `e2e/wasm-engine-dist.spec.ts` asserts shell load, engine_state=ready, and
  the "Created blank local model document..." success message — the exact
  human-found regression, replayed. Dev-server lane unchanged (`*-dist.spec.ts`
  is testIgnored there). DEC-025 sweep keeps five surfaces; the dist lane runs
  as a second command inside `desktop_playwright_e2e`.
- Lanes covered: dev server (e2e 2/2), Vitest/node (241/241; count includes
  concurrent sibling-task additions), production dist (dist e2e 1/1; `vite
  build` green with `dist/wasm-engine/` present), python 358/358.
- Honest residual gap: the tauri:// asset protocol itself cannot be exercised
  by a browser harness; the packaged `.app` pass remains the human TP-MAC-141
  manual verification (dispatcher rebuilds the bundle at fan-in).
